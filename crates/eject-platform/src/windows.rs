//! Windows adapter stub.
//! Implements volume enumeration, PnP eject, Event 225 parsing, Restart Manager.
//! See docs/platform-windows.md for native API notes.

use crate::{BackendError, BackendResult, EjectBackend, EjectRequest, EjectResponse};
use async_trait::async_trait;
use eject_core::device::{RemovalScope, TopologySnapshot};
use eject_core::evidence::DiagnosticReport;

pub struct WindowsBackend;

#[async_trait]
impl EjectBackend for WindowsBackend {
    async fn refresh_topology(&self) -> BackendResult<TopologySnapshot> {
        // TODO(platform): Enumerate volumes via FindFirstVolume/FindNextVolume,
        // resolve disk extents (IOCTL_VOLUME_GET_VOLUME_DISK_EXTENTS),
        // map to PnP device instances via SetupAPI/Configuration Manager.
        // Exclude boot/system disk, internal devices, ambiguous scopes.
        Err(BackendError::UnsupportedPlatform(
            "Windows adapter is a stub; implement native enumeration".into(),
        ))
    }

    async fn request_eject(&self, _req: EjectRequest) -> BackendResult<EjectResponse> {
        // TODO(platform): Revalidate scope, resolve device node, call
        // CM_Request_Device_EjectW, classify CONFIGRET + veto type/name.
        Err(BackendError::UnsupportedPlatform(
            "Windows eject is a stub; implement CM_Request_Device_EjectW".into(),
        ))
    }

    async fn collect_diagnostics(
        &self,
        _scope: &RemovalScope,
        _operation_id: &str,
    ) -> BackendResult<DiagnosticReport> {
        // TODO(platform): Query Kernel-PnP Event 225 in operation window;
        // optional Restart Manager file checks with bounded traversal.
        Ok(DiagnosticReport {
            items: Vec::new(),
            incomplete: true,
            note: Some("Windows diagnostics not implemented in stub".into()),
        })
    }

    fn backend_name(&self) -> &'static str {
        "windows-pnp"
    }
}
