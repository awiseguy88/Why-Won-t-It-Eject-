//! Core domain model, state machine, retry policy, and evidence types
//! for "Why Won't It Eject?". Platform-agnostic and testable with fake backends.

pub mod device;
pub mod error;
pub mod evidence;
pub mod retry;
pub mod state;

pub use device::*;
pub use error::*;
pub use evidence::*;
pub use retry::*;
pub use state::*;