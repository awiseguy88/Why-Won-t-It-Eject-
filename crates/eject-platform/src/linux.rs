//! Linux adapter stub.
//! UDisks2 via zbus, /proc scanning, polkit handling.

use crate::{BackendError, BackendResult, EjectBackend, EjectRequest, EjectResponse};
use async_trait::async_trait;
use eject_core::device::{RemovalScope, TopologySnapshot};
use eject_core::error::EjectOutcome;
use eject_core::evidence::DiagnosticReport;

pub struct LinuxBackend;

#[async_trait]
impl EjectBackend for LinuxBackend {
    async fn refresh_topology(&self) -> BackendResult<TopologySnapshot> {
        // TODO(platform): Connect to UDisks2 ObjectManager, subscribe to changes,
        // map Drive/Block/Filesystem, protect root/system/boot chains.
        Err(BackendError::UnsupportedPlatform(
            "Linux adapter is a stub; implement UDisks2 topology".into(),
        ))
    }

    async fn request_eject(&self, _req: EjectRequest) -> BackendResult<EjectResponse> {
        // TODO(platform): Unmount volumes in scope, then Drive.PowerOff if CanPowerOff.
        Err(BackendError::UnsupportedPlatform(
            "Linux eject is a stub; implement UDisks2 unmount/poweroff".into(),
        ))
    }

    async fn collect_diagnostics(
        &self,
        _scope: &RemovalScope,
        _operation_id: &str,
    ) -> BackendResult<DiagnosticReport> {
        // TODO(platform): Scan /proc/*/fd, cwd, maps matching fs/device.
        Ok(DiagnosticReport {
            items: Vec::new(),
            incomplete: true,
            note: Some("Linux diagnostics not implemented in stub".into()),
        })
    }

    fn backend_name(&self) -> &'static str {
        "linux-udisks2"
    }
}