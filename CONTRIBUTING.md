# Contributing to Why Won’t It Eject?

Thanks for your interest in contributing. This project aims to be a trustworthy, cross-platform eject utility: **correctness and safety over convenience** (never a false “Safe to unplug”, never mutate the wrong device).

## Ways to contribute

- **Platform adapters:** implement and harden Windows/macOS/Linux backends against the acceptance conditions in `docs/implementation-plan.md` and platform docs.
- **Core & tests:** state machine, retry/timer behavior, scope revalidation, fixture-driven tests (`tests/fixtures/`, `crates/eject-core`).
- **UI & accessibility:** keyboard navigation, focus, screen reader labels, dark/light themes, long-path layouts, tray/menu UX.
- **Hardware compatibility:** run the release/hardware scenarios and file reports using the hardware compatibility issue template.
- **Docs:** clarify support matrix, platform prerequisites, and troubleshooting with evidence.

## Development setup

See the [README](README.md#development) for prerequisites (Rust, Node/pnpm, Tauri platform deps).

```bash
pnpm install
pnpm --dir ui install
cargo test -p eject-core      # core logic tests
pnpm tauri dev                # desktop dev run (requires native toolchain)
```

## Principles (must hold for all changes)

1. **Safety:** Accept success only from a positive whole-device native eject/power-off for the original attachment and approved scope. Revalidate identity, topology, eligibility, and consent before every OS mutation. Serialize operations on overlapping scopes.
2. **Honest copy:** Only attribute a blocker to an app when OS/validated evidence supports it. Distinguish OS-attributed veto vs. verified open file vs. unknown/busy fallback.
3. **No dangerous actions:** No process-kill, handle-close, force-eject, drivers, or background services. No arbitrary shell/path execution from the frontend.
4. **Privacy:** Local-only. No telemetry/accounts/remote calls/auto-update in v1. Paths and process evidence stay in memory unless explicitly exported in redacted form.
5. **Deterministic tests:** Mock native responses and clocks for timing/state tests. Use versioned synthetic fixtures with fictitious identifiers.

## Pull requests

- Keep PRs focused; link to the relevant milestone/issue when applicable.
- Add or update tests/fixtures for behavior changes (core state/retry/copy decisions).
- Run `cargo fmt`, `cargo clippy`, `cargo test -p eject-core`, and UI lint/typecheck before submitting.
- Do not include secrets, signing material, or real user paths/process dumps.

## Reporting security issues

Please do **not** open public issues for security vulnerabilities. See [SECURITY.md](SECURITY.md).