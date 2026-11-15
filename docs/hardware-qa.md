# Hardware QA & Release Testing — Why Won’t It Eject?

> Run these scenarios per supported OS/CPU against release candidates before tagging `v1.0.0`. Record OS build, architecture, filesystem, bridge/enclosure, native result, expected copy, and actual outcome for every RC.

## Media & hardware coverage

- USB flash stick
- UASP external SSD
- External HDD
- Dual-partition USB drive
- Multi-slot reader or enclosure
- Filesystems: FAT32/exFAT (supported), NTFS (Windows), APFS (macOS), ext4 (Linux)
- Standard account + one restricted-visibility test per platform

## Required scenarios

| # | Scenario | Required result |
|---|---|---|
| 1 | Idle USB stick / external SSD | Correct device selected; normal OS completion; one success notification; clean session |
| 2 | Copy in progress, open file, CWD on drive, background indexer | No success while OS vetoes; supported attribution, verified open file, or busy fallback. After blocker clears, still-armed attempt can complete. |
| 3 | Closed copy, unplug after success, reinsert | Verify SHA-256 of test files after remount. Prefer ≥10 eject/reinsert cycles per OS/CPU × stick/SSD. |
| 4 | Multi-partition drive, hub, multi-slot reader, APFS | Show physical removal scope; protect other devices; report incomplete unmount/eject accurately. |
| 5 | Unplug + replace with identical label/reused path | Cancel old retry; discard stale callbacks; never act on replacement without new consent. |
| 6 | Unplug during OS call, timeout, daemon interruption, permission denied | Matching disconnect / unknown outcome / permission state; suppress background auth prompts + false success. |
| 7 | System disk, unresolved bridge, encrypted/RAID stack, app running off selected disk | Deny unsafe/ambiguous targeting; specific message when detectable. |
| 8 | Cancel just before call vs. after start; double-click; sleep/resume | Latest intent controls future work. In-flight native request may finish; result applies only to original attachment. Sleep/resume invalidates pending retry → fresh user action. |
| 9 | Standard accounts, non-ASCII paths, duplicate names, blocked notifications | Core flow remains usable; paths local unless exported. |
| 10 | Clean installs / normal launch / upgrade / uninstall | Signed Windows portable+setup; Gatekeeper both Mac CPUs; Linux dependency resolution + launch by package; upgrade/uninstall for installers. |

## Release gate (all must hold)

- Zero false **Safe to unplug** in fixture + physical matrices
- Zero mutations on wrong device
- Zero blocker/critical regressions
- Representative same-user blocker cases confirmed on each platform
- Busy fallback confirmed when evidence inaccessible
- Real-device signoff per supported OS/CPU

## Environment notes

- Use machines or controlled USB-passthrough setups that can exercise the native eject path (no VM-only without USB passthrough).
- Do not include real user paths/process dumps in reports; use fictitious identifiers per fixture conventions.