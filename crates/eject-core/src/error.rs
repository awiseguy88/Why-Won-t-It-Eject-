use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CoreError {
    #[error("device not found")]
    DeviceNotFound,
    #[error("device no longer eligible: {0}")]
    NotEligible(String),
    #[error("removal scope changed; revalidation required")]
    ScopeChanged,
    #[error("operation already in flight for this scope")]
    OperationInFlight,
    #[error("retry not armed")]
    RetryNotArmed,
    #[error("backend error: {0}")]
    Backend(String),
}

/// Normalized outcome of a native eject attempt, as returned by a platform backend.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EjectOutcome {
    /// Backend confirmed whole-device eject/power-off for the approved scope.
    ConfirmedEject,
    /// OS reports the device is busy.
    Busy { native_hint: Option<String> },
    /// Permission/authorization denied.
    PermissionDenied { detail: Option<String> },
    /// Device or operation not supported by this backend/configuration.
    Unsupported { detail: String },
    /// Device disappeared before a conclusive result.
    DeviceGone,
    /// Some volumes unmounted but drive still connected / power-off failed.
    Partial { detail: String },
    /// Missing, inconsistent, or late native result; requires reconciliation.
    UnknownOutcome { detail: String },
    /// A failure not captured by other categories.
    Failure { detail: String },
}

impl EjectOutcome {
    pub fn is_retryable_busy(&self) -> bool {
        matches!(self, EjectOutcome::Busy { .. })
    }

    pub fn is_terminal_for_retry(&self) -> bool {
        matches!(
            self,
            EjectOutcome::ConfirmedEject
                | EjectOutcome::DeviceGone
                | EjectOutcome::Unsupported { .. }
                | EjectOutcome::PermissionDenied { .. }
                | EjectOutcome::UnknownOutcome { .. }
        )
    }
}
