use crate::device::RemovalScope;
use crate::error::EjectOutcome;
use crate::evidence::DiagnosticReport;
use crate::retry::RetryState;
use serde::{Deserialize, Serialize};

/// High-level UI/core state for a device scope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScopeStatus {
    /// Eligible and idle.
    Eligible,
    /// A manual eject is in progress.
    Attempting { operation_id: String },
    /// Busy result; eligible for retry opt-in.
    Busy {
        diagnostics: Option<DiagnosticReport>,
    },
    /// Automatic retry is armed and waiting/running.
    RetryArmed {
        retry: RetryState,
        diagnostics: Option<DiagnosticReport>,
    },
    /// Retry deadline reached without success.
    RetryExpired {
        diagnostics: Option<DiagnosticReport>,
    },
    /// Backend confirmed safe to unplug.
    SafeToUnplug,
    /// Device disappeared.
    DeviceGone,
    /// Partial unmount/power-off.
    Partial { detail: String },
    /// Permission denied.
    PermissionDenied { detail: Option<String> },
    /// Unsupported.
    Unsupported { detail: String },
    /// Unknown/inconsistent outcome.
    UnknownOutcome { detail: String },
    /// Generic failure.
    Failure { detail: String },
}

/// An operation tracked by the core.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationRecord {
    pub operation_id: String,
    pub scope: RemovalScope,
    pub status: ScopeStatus,
    pub started_at_unix_ms: i64,
    pub completed_at_unix_ms: Option<i64>,
}

/// Transition intent from UI/user.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserIntent {
    RequestEject,
    ArmRetry,
    CancelRetry,
    Dismiss,
}

/// Derive a new status from a native outcome. Does not arm retry automatically.
pub fn status_from_outcome(
    outcome: EjectOutcome,
    diagnostics: Option<DiagnosticReport>,
) -> ScopeStatus {
    match outcome {
        EjectOutcome::ConfirmedEject => ScopeStatus::SafeToUnplug,
        EjectOutcome::Busy { .. } => ScopeStatus::Busy { diagnostics },
        EjectOutcome::PermissionDenied { detail } => ScopeStatus::PermissionDenied { detail },
        EjectOutcome::Unsupported { detail } => ScopeStatus::Unsupported { detail },
        EjectOutcome::DeviceGone => ScopeStatus::DeviceGone,
        EjectOutcome::Partial { detail } => ScopeStatus::Partial { detail },
        EjectOutcome::UnknownOutcome { detail } => ScopeStatus::UnknownOutcome { detail },
        EjectOutcome::Failure { detail } => ScopeStatus::Failure { detail },
    }
}

pub fn can_arm_retry(status: &ScopeStatus) -> bool {
    matches!(status, ScopeStatus::Busy { .. })
}

pub fn can_request_manual_eject(status: &ScopeStatus) -> bool {
    matches!(
        status,
        ScopeStatus::Eligible
            | ScopeStatus::Busy { .. }
            | ScopeStatus::RetryExpired { .. }
            | ScopeStatus::Failure { .. }
            | ScopeStatus::Partial { .. }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn busy_is_retryable_ui_state() {
        let st = status_from_outcome(EjectOutcome::Busy { native_hint: None }, None);
        assert!(can_arm_retry(&st));
        assert!(can_request_manual_eject(&st));
    }

    #[test]
    fn confirmed_is_terminal() {
        let st = status_from_outcome(EjectOutcome::ConfirmedEject, None);
        assert!(!can_arm_retry(&st));
        assert!(!can_request_manual_eject(&st));
    }
}
