//! Platform backend abstraction.
//!
//! Each OS implements `EjectBackend`. The core never calls native APIs directly.

use async_trait::async_trait;
use eject_core::device::{PhysicalDevice, RemovalScope, TopologySnapshot};
use eject_core::error::EjectOutcome;
use eject_core::evidence::DiagnosticReport;

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "windows")]
pub mod windows;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EjectRequest {
    pub scope: RemovalScope,
    /// Operation ID for correlating callbacks.
    pub operation_id: String,
    /// True if this is an automatic retry (backend should use no-user-interaction auth where applicable).
    pub is_auto_retry: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EjectResponse {
    pub outcome: EjectOutcome,
    /// Backend-provided diagnostic hints (may be empty; diagnostics pass is separate).
    pub native_detail: Option<String>,
}

#[async_trait]
pub trait EjectBackend: Send + Sync {
    /// Fresh topology snapshot. Callers should revalidate scope after this changes.
    async fn refresh_topology(&self) -> Result<TopologySnapshot, BackendError>;

    /// Request safe eject for the approved scope.
    /// Must revalidate identity/scope internally before any mutation.
    async fn request_eject(&self, req: EjectRequest) -> Result<EjectResponse, BackendError>;

    /// Bounded diagnostic pass after a busy result. Must not mutate device state.
    async fn collect_diagnostics(
        &self,
        scope: &RemovalScope,
        operation_id: &str,
    ) -> Result<DiagnosticReport, BackendError>;

    /// Human-readable backend name for support/debugging.
    fn backend_name(&self) -> &'static str;
}

#[derive(Debug, thiserror::Error)]
pub enum BackendError {
    #[error("topology refresh failed: {0}")]
    TopologyRefresh(String),
    #[error("eject call failed: {0}")]
    EjectCall(String),
    #[error("diagnostics failed: {0}")]
    Diagnostics(String),
    #[error("backend not supported on this platform: {0}")]
    UnsupportedPlatform(String),
}

/// Construct the platform backend for the current OS.
pub fn default_backend() -> Box<dyn EjectBackend> {
    #[cfg(target_os = "windows")]
    {
        Box::new(windows::WindowsBackend)
    }
    #[cfg(target_os = "macos")]
    {
        Box::new(macos::MacosBackend)
    }
    #[cfg(target_os = "linux")]
    {
        Box::new(linux::LinuxBackend)
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        compile_error!("unsupported target OS for why-wont-it-eject");
    }
}

pub type BackendResult<T> = Result<T, BackendError>;