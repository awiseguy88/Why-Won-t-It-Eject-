//! Tauri application entry point.
//!
//! Owns: typed command bridge, tray/menu integration, app state wiring.
//! Rust core (eject-core, eject-platform) owns device logic and retries.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eject_core::device::{PhysicalDeviceKey, RemovalScope, TopologySnapshot};
use eject_core::error::EjectOutcome;
use eject_core::retry::{RetryPolicy, RetryState};
use eject_core::state::{
    can_arm_retry, can_request_manual_eject, status_from_outcome, ScopeStatus,
};
use eject_platform::{default_backend, EjectBackend};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tauri::State;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiDevice {
    pub id: String,
    pub generation: u64,
    pub display_name: String,
    pub model: Option<String>,
    pub size_bytes: Option<u64>,
    pub eligible: bool,
    pub ineligible_reason: Option<String>,
    pub volumes: Vec<UiVolume>,
    pub status: ScopeStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiVolume {
    pub id: String,
    pub label: Option<String>,
    pub mount_point: Option<String>,
    pub fs_type: Option<String>,
}

struct AppState {
    backend: Box<dyn EjectBackend>,
    topology: TopologySnapshot,
    statuses: HashMap<String, ScopeStatus>, // keyed by native_id+generation
    retries: HashMap<String, RetryState>,
    policy: RetryPolicy,
    /// Scopes with an in-flight native call.
    in_flight: std::collections::HashSet<String>,
}

impl AppState {
    fn new() -> Self {
        Self {
            backend: default_backend(),
            topology: TopologySnapshot::default(),
            statuses: HashMap::new(),
            retries: HashMap::new(),
            policy: RetryPolicy::default(),
            in_flight: std::collections::HashSet::new(),
        }
    }

    fn scope_key(scope: &RemovalScope) -> String {
        format!(
            "{}::{}",
            scope.device_key.native_id, scope.device_key.attach_generation
        )
    }
}

type SharedState = Arc<Mutex<AppState>>;

#[derive(Serialize)]
struct CommandError {
    message: String,
}

impl<T: std::fmt::Display> From<T> for CommandError {
    fn from(value: T) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}

#[tauri::command]
async fn list_devices(state: State<'_, SharedState>) -> Result<Vec<UiDevice>, CommandError> {
    // TODO: refresh topology from backend on background task; for now return cached/synthetic.
    let st = state.lock();
    let devices = st
        .topology
        .devices
        .iter()
        .map(|d| {
            let key = format!("{}::{}", d.key.native_id, d.key.attach_generation);
            UiDevice {
                id: d.key.native_id.clone(),
                generation: d.key.attach_generation,
                display_name: d.display_name(),
                model: d.model.clone(),
                size_bytes: d.size_bytes,
                eligible: d.eligible,
                ineligible_reason: d.ineligible_reason.clone(),
                volumes: d
                    .volumes
                    .iter()
                    .map(|v| UiVolume {
                        id: v.id.clone(),
                        label: v.label.clone(),
                        mount_point: v.mount_point.clone(),
                        fs_type: v.fs_type.clone(),
                    })
                    .collect(),
                status: st
                    .statuses
                    .get(&key)
                    .cloned()
                    .unwrap_or(ScopeStatus::Eligible),
            }
        })
        .collect();
    Ok(devices)
}

#[tauri::command]
async fn request_eject(
    state: State<'_, SharedState>,
    device_id: String,
    generation: u64,
) -> Result<ScopeStatus, CommandError> {
    // Validate, serialize per-scope, call backend, update status, emit event.
    // Stub: demonstrates the command contract.
    let scope = RemovalScope {
        device_key: PhysicalDeviceKey {
            native_id: device_id.clone(),
            attach_generation: generation,
        },
        volume_ids: vec![],
        display_names: vec![device_id.clone()],
    };
    let key = AppState::scope_key(&scope);
    {
        let mut st = state.lock();
        if st.in_flight.contains(&key) {
            return Err("Operation already in flight for this scope".into());
        }
        let current = st
            .statuses
            .get(&key)
            .cloned()
            .unwrap_or(ScopeStatus::Eligible);
        if !can_request_manual_eject(&current) {
            return Err("Device is not in a state that allows eject".into());
        }
        st.in_flight.insert(key.clone());
        st.statuses.insert(
            key.clone(),
            ScopeStatus::Attempting {
                operation_id: "op-stub".into(),
            },
        );
    }
    // TODO: spawn native eject on background task, revalidate scope before/after,
    // run diagnostics on busy, reconcile unknown outcomes.
    let outcome = EjectOutcome::Unsupported {
        detail: "Backend adapter not implemented yet for this platform build".into(),
    };
    let status = status_from_outcome(outcome, None);
    {
        let mut st = state.lock();
        st.in_flight.remove(&key);
        st.statuses.insert(key, status.clone());
    }
    Ok(status)
}

#[tauri::command]
async fn arm_retry(
    state: State<'_, SharedState>,
    device_id: String,
    generation: u64,
) -> Result<ScopeStatus, CommandError> {
    let scope = RemovalScope {
        device_key: PhysicalDeviceKey {
            native_id: device_id,
            attach_generation: generation,
        },
        volume_ids: vec![],
        display_names: vec![],
    };
    let key = AppState::scope_key(&scope);
    let mut st = state.lock();
    let current = st
        .statuses
        .get(&key)
        .cloned()
        .unwrap_or(ScopeStatus::Eligible);
    if !can_arm_retry(&current) {
        return Err("Retry can only be armed after a busy result".into());
    }
    let now_ms = chrono_like_now_ms();
    let retry = RetryState::new(now_ms, st.policy);
    let status = ScopeStatus::RetryArmed {
        retry: retry.clone(),
        diagnostics: match current {
            ScopeStatus::Busy { diagnostics } => diagnostics,
            _ => None,
        },
    };
    st.retries.insert(key.clone(), retry);
    st.statuses.insert(key, status.clone());
    // TODO: schedule timer task honoring monotonic clock, deadline, cancellation.
    Ok(status)
}

#[tauri::command]
async fn cancel_retry(
    state: State<'_, SharedState>,
    device_id: String,
    generation: u64,
) -> Result<ScopeStatus, CommandError> {
    let key = format!("{device_id}::{generation}");
    let mut st = state.lock();
    st.retries.remove(&key);
    let status = ScopeStatus::RetryExpired { diagnostics: None };
    st.statuses.insert(key, status.clone());
    Ok(status)
}

fn chrono_like_now_ms() -> i64 {
    // Avoid depending on chrono in app stub; core/tests use injected clocks.
    // In production wire this to a proper time source.
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

fn main() {
    let state = Arc::new(Mutex::new(AppState::new()));

    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            list_devices,
            request_eject,
            arm_retry,
            cancel_retry,
        ])
        .setup(|app| {
            // TODO: build tray with "Open", per-drive actions, retry status, "Quit".
            // Tray must remain usable on Linux desktops without click events.
            let _ = app; // placeholder
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Why Won't It Eject");
}
