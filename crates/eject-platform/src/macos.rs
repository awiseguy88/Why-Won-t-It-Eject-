//! macOS adapter stub.
//! Disk Arbitration registration, APFS synthesized device handling,
//! bounded lsof diagnostics.

use crate::{BackendError, BackendResult, EjectBackend, EjectRequest, EjectResponse};
use async_trait::async_trait;
use eject_core::device::{RemovalScope, TopologySnapshot};
use eject_core::error::EjectOutcome;
use eject_core::evidence::DiagnosticReport;

pub struct MacosBackend;

#[async_trait]
impl EjectBackend for MacosBackend {
    async fn refresh_topology(&self) -> BackendResult<TopologySnapshot> {
        // TODO(platform): Register DADiskAppeared/Disappeared/Changed callbacks,
        // resolve whole disks and APFS physical stores.
        Err(BackendError::UnsupportedPlatform(
            "macOS adapter is a stub; implement Disk Arbitration topology".into(),
        ))
    }

    async fn request_eject(&self, _req: EjectRequest) -> BackendResult<EjectResponse> {
        // TODO(platform): DADiskUnmount(whole disk) then DADiskEject(physical).
        Err(BackendError::UnsupportedPlatform(
            "macOS eject is a stub; implement DADiskUnmount/DADiskEject".into(),
        ))
    }

    async fn collect_diagnostics(
        &self,
        _scope: &RemovalScope,
        _operation_id: &str,
    ) -> BackendResult<DiagnosticReport> {
        // TODO(platform): Bounded read-only lsof against selected filesystem.
        Ok(DiagnosticReport {
            items: Vec::new(),
            incomplete: true,
            note: Some("macOS diagnostics not implemented in stub".into()),
        })
    }

    fn backend_name(&self) -> &'static str {
        "macos-diskarbitration"
    }
}