# Architecture — Why Won’t It Eject?

## Layers

| Layer | Crate/Dir | Responsibility | Does NOT |
| --- | --- | --- | --- |
| UI | `ui/` (TypeScript, Vite + React) | Window layout, status copy, accessibility, tray-friendly actions. Calls typed Tauri commands only. | Own device logic, native calls, retry timers, shell/path execution. |
| App shell | `src-tauri/` (Rust, Tauri 2) | Window + tray/menu, command bridge (`list_devices`, `request_eject`, `arm_retry`, `cancel_retry`, notifications), wiring core to backend. | Reimplement eject logic; contains thin orchestration only. |
| Core | `crates/eject-core/` | Device model, physical key + attach generation, removal scope, state machine (`ScopeStatus`), retry policy/state, evidence model, outcome normalization. Platform-agnostic, deterministic tests. | Any OS API, file I/O, timers tied to wall clock (injected clocks in tests). |
| Platform | `crates/eject-platform/` | `EjectBackend` trait + per-OS adapters: Windows (SetupAPI/CFGMGR32/IOCTLs/Restart Manager), macOS (Disk Arbitration/lsof), Linux (UDisks2/zbus, `/proc`). | Decide user consent, UI copy, or global retry policy (core owns policy; adapter reports native results + hints). |

## Core concepts

- **PhysicalDeviceKey**: `native_id` (platform stable ID) + `attach_generation` (bumped on reappearance). A label/mountpoint/drive letter/BSD node can be reused; the key + generation prevents acting on a replacement.
- **RemovalScope**: device key + approved volume IDs + display names for confirmation/copy. Every mutation revalidates identity, topology, eligibility, and consent for this scope.
- **EjectOutcome** (normalized): `ConfirmedEject`, `Busy`, `PermissionDenied`, `Unsupported`, `DeviceGone`, `Partial`, `UnknownOutcome`, `Failure`. Only `ConfirmedEject` permits “Safe to unplug”.
- **Evidence**: `BlockerEvidence` carries `source` (`OsAttributedVeto`, `VerifiedOpenFile`, `Observed`), process identity (PID + start ID where available), optional file path, observation time. UI copy depends on source.
- **Retry**: opt-in after retryable `Busy`; one timer/scope; monotonic scheduling; defaults 3s → 5s → 10s, 2-minute deadline; stops on cancel/quit/deadline/disconnect/topology change/unsupported/auth-needed/uncertain outcome; sleep/resume invalidates pending retry.

## Operation lifecycle

1. User selects physical device → core ensures scope (volumes under that drive) is known and shows scope if multi-volume/sibling.
2. User consents → app creates `operation_id`, marks scope `Attempting`, serializes per-scope (no overlapping native calls for same scope).
3. Backend **revalidates** scope internally, performs native eject (e.g. `CM_Request_Device_EjectW`, `DADiskUnmount`+`DADiskEject`, UDisks2 unmount+`PowerOff`), returns `EjectResponse` with normalized outcome.
4. Core maps outcome → `ScopeStatus`. On `Busy`, optional bounded diagnostics run (Event 225 / Restart Manager, lsof, `/proc` scan) → evidence attached.
5. If user armed retry and outcome retryable → schedule next attempt; on terminal/disconnect/etc. → stop scheduling.
6. On `ConfirmedEject` (original attachment/scope only) → `SafeToUnplug` + one notification; expire success association if a new attachment appears.

## Constraints enforced in code/docs

- No arbitrary shell/path execution from frontend; commands are typed and minimal.
- Local-only, no telemetry/accounts/remote calls/auto-update.
- Standard user process; OS-native authorization only on direct action; no force/kill/handle-close.
- Redacted export explicit; paths/process evidence in memory only otherwise.

See also: [support-matrix.md](support-matrix.md), [implementation-plan.md](implementation-plan.md), [hardware-qa.md](hardware-qa.md), [release-runbook.md](release-runbook.md), platform docs (`platform-windows.md`, `platform-macos.md`, `platform-linux.md`).