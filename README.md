# Why Won’t It Eject?

**Your drive won’t eject. Find out what’s using it.**

A free, open-source desktop utility for USB flash drives and external SSDs and hard drives. Select a drive, request safe eject, see what the computer can identify if it’s busy, and optionally let the app retry while you finish working.

> **Status:** v1.0.0 planning & scaffold (milestone-driven). This repository contains the project structure, core domain model, platform adapter stubs, Tauri app shell, UI scaffold, docs, and CI/release workflow templates described in the build plan. Native platform implementations, signing, and hardware validation are tracked as milestone work.

## Download (planned v1.0.0 release assets)

When `v1.0.0` is published, the [GitHub Releases](../../releases) page will include:

| Platform | Architecture | Asset |
| --- | --- | --- |
| Windows | x64 / ARM64 | `why-wont-it-eject-1.0.0-windows-{arch}-portable.exe`, `why-wont-it-eject-1.0.0-windows-{arch}-setup.exe` |
| macOS | Universal (Intel + Apple silicon) | `why-wont-it-eject-1.0.0-macos-universal.dmg` (Developer ID signed, notarized, stapled) |
| Linux | x86_64 / ARM64 | `.deb`, `.rpm` (both archs), `why-wont-it-eject-1.0.0-linux-x86_64.AppImage` |
| Verification | — | `SHA256SUMS.txt` and GitHub artifact provenance |

> **Proposed support matrix (confirm against release-candidate tests before publishing):** Windows 11 x64/ARM64 (Windows 10 22H2 x64 legacy smoke-test), macOS 14+ on Intel/Apple silicon, Ubuntu 22.04/24.04, Debian 12/13, maintained Fedora (x86_64/ARM64) with UDisks2 and desktop session. Results will be recorded in compatibility notes.

## What it does (v1 workflow)

1. **Select a physical device** — label/model, capacity, volumes and mount locations. Volumes are listed under their physical drive. No selection by default when multiple eligible devices are present.
2. **Request safe eject** — duplicate actions disabled per removal scope while in flight.
3. **Explain the result** — evidence-based blocker details. OS-attributed veto vs. verified open file are distinct. Plain-language busy fallback when no app can be confirmed.
4. **Wait with consent (optional)** — “Eject when ready” arms background retry (3s → 5s → 10s, 2-minute deadline, one timer/scope). Cancel anytime.
5. **Finish** — “Safe to unplug” only on verified whole-device native success; one notification (or in-app/tray fallback).

## Supported devices & limitations (v1)

- **In scope:** Physical USB mass storage, including drives behind USB-to-SATA/NVMe bridges and standard USB hubs. Simple single- and multi-partition devices when complete eject scope can be determined. APFS containers/physical-store mappings are a required macOS case.
- **Out of scope / declined:** Network mounts, disk images, optical media, purely internal disks, mobile OSes, ambiguous logical-device stacks, multi-store APFS or unclear ownership/eject scope, force-eject/handle-closing/process-kill actions.
- **Blocker details depend on OS evidence and permissions.** The app never claims an app “blocked eject” without OS attribution or validated evidence; it distinguishes “has this file open” from an OS veto.

## Project structure

```text
why-wont-it-eject/
  ui/                         # TypeScript UI (Vite + React), status/copy/accessibility
  crates/eject-core/          # Device model, state machine, retries, policies (platform-agnostic)
  crates/eject-platform/      # Windows, macOS, Linux backend adapters + trait
  src-tauri/                  # Tauri 2 app entry, tray, typed command bridge
  tests/fixtures/             # Synthetic topology, error and diagnostic fixtures
  docs/                       # Architecture, support, hardware QA, release runbook
  .github/workflows/          # CI, packaging, release
```

## Development

### Prerequisites

- **Rust** (stable, >= 1.77) and Cargo — [rustup.rs](https://rustup.rs)
- **Node.js** (LTS) and **pnpm**
- **Tauri 2 prerequisites** per platform: <https://v2.tauri.app/start/prerequisites/>
  - Linux: system dependencies for WebKitGTK and Tauri (see Tauri docs / `docs/platform-linux.md`)
  - macOS: Xcode Command Line Tools
  - Windows: Microsoft C++ Build Tools, WebView2 runtime

### Run locally

```bash
# install JS deps
pnpm install
pnpm --dir ui install

# run the desktop app in dev mode (requires Rust toolchain + platform deps)
pnpm tauri dev

# run core Rust tests (no UI/platform runtime required for eject-core unit tests)
cargo test -p eject-core
```

> Note: Platform adapters are scaffolded as stubs in this milestone; native enumeration/eject/diagnostics are implemented per `docs/implementation-plan.md` and platform docs.

## Architecture (summary)

- **UI (TypeScript/Tauri):** window, tray/menu where supported, status copy, accessibility. No arbitrary shell/path execution; typed commands only.
- **Core (Rust, `eject-core`):** physical-device key + attach generation + approved removal scope, state transitions, retry scheduling policy, evidence model, normalized outcomes. Deterministic/testable with fake backends.
- **Platform (`eject-platform`):** OS adapters (Windows PnP/SetupAPI/CM/Restart Manager, macOS Disk Arbitration/lsof, Linux UDisks2/zbus + `/proc`). Revalidate identity/scope before every mutation; serialize overlapping scopes.
- **Success rule:** only positive whole-device eject/power-off from native backend for the original attachment and approved scope. No false “Safe to unplug”.

See [docs/architecture.md](docs/architecture.md), [docs/support-matrix.md](docs/support-matrix.md), and [docs/release-runbook.md](docs/release-runbook.md).

## Security & privacy

- Standard user process. No driver, no service, no process-kill/close-handle/force-eject. Native OS authorization only on direct user actions (auto-retry stops if new auth needed).
- Local-only: preferences stored locally, paths/process evidence kept in memory, redacted diagnostic export is explicit. No accounts, telemetry, remote calls, or auto-update in v1.
- See [SECURITY.md](SECURITY.md).

## Contributing

Contributions welcome — especially platform adapter work, hardware compatibility reports, accessibility improvements, and tests. See [CONTRIBUTING.md](CONTRIBUTING.md) and hardware issue forms in `.github/ISSUE_TEMPLATE/`.

## License

[MIT](LICENSE)

## Acknowledgements

EjectLens covers Windows blocker detection and eject. This project’s distinguishing workflow is tray-first diagnosis with consent-based retry on Windows, macOS, and Linux.  
- [EjectLens](https://github.com/weinianxue/EjectLens) · [Tauri](https://v2.tauri.app/) · [UDisks2](https://storaged.org/doc/udisks2-api/latest/) · [Disk Arbitration](https://developer.apple.com/documentation/diskarbitration) · [Windows CM_Request_Device_EjectW](https://learn.microsoft.com/en-us/windows/win32/api/cfgmgr32/nf-cfgmgr32-cm_request_device_ejectw)