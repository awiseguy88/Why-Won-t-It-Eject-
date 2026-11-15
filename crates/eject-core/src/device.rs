use serde::{Deserialize, Serialize};
use std::fmt;

/// A stable identity for a physical device attachment.
/// Backends must populate native IDs; `attach_generation` is bumped by the core
/// when the same physical key is observed again after disappearance.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PhysicalDeviceKey {
    /// Platform-native stable identifier (e.g. PnP instance, disk UUID, UDisks2 drive path).
    pub native_id: String,
    /// Monotonic attachment generation assigned by the core.
    pub attach_generation: u64,
}

/// Identifies the approved removal scope for an operation.
/// All mutations must be revalidated against this scope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemovalScope {
    pub device_key: PhysicalDeviceKey,
    /// Volume/mount identifiers included in this scope (native IDs).
    pub volume_ids: Vec<String>,
    /// Human-readable names for confirmation/copy (e.g. "D:\ (Photos)", "Samsung T7").
    pub display_names: Vec<String>,
}

/// A mount point or volume exposed by a physical device.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VolumeInfo {
    pub id: String,
    pub label: Option<String>,
    pub mount_point: Option<String>,
    pub fs_type: Option<String>,
    pub size_bytes: Option<u64>,
}

/// A discovered physical removable device.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PhysicalDevice {
    pub key: PhysicalDeviceKey,
    pub model: Option<String>,
    pub vendor: Option<String>,
    pub label: Option<String>,
    pub size_bytes: Option<u64>,
    pub volumes: Vec<VolumeInfo>,
    /// True if the backend considers this device eligible for safe eject in v1.
    pub eligible: bool,
    /// If not eligible, a short reason for display.
    pub ineligible_reason: Option<String>,
}

impl PhysicalDevice {
    pub fn display_name(&self) -> String {
        if let Some(label) = &self.label {
            if !label.is_empty() {
                return label.clone();
            }
        }
        match (&self.vendor, &self.model) {
            (Some(v), Some(m)) => format!("{v} {m}"),
            (_, Some(m)) => m.clone(),
            (Some(v), _) => v.clone(),
            _ => "Removable drive".to_string(),
        }
    }
}

impl fmt::Display for RemovalScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.display_names.is_empty() {
            write!(f, "selected device")
        } else {
            write!(f, "{}", self.display_names.join(", "))
        }
    }
}

/// A snapshot of the current device topology.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TopologySnapshot {
    pub devices: Vec<PhysicalDevice>,
}
