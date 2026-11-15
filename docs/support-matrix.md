# Support Matrix (proposed — confirm at release freeze)

> Proposed defaults from the plan. Confirm minimum OS versions and distro/package combinations against actual release-candidate tests before publishing the README.

## Desktop OS families (v1)

| Platform | Proposed supported systems | v1.0.0 artifacts |
| --- | --- | --- |
| Windows | Windows 11 (x64, ARM64). Windows 10 22H2 x64 = legacy smoke-test with compatibility notes. | Signed portable `.exe` + signed per-user setup `.exe`, each for x64/ARM64 |
| macOS | macOS 14+ on Intel + Apple silicon (where OS runs on that hardware) | One Developer ID–signed, notarized + stapled universal `.dmg` |
| Linux | Ubuntu 22.04/24.04, Debian 12/13, maintained Fedora (tested at freeze); x86_64 + ARM64; desktop session with UDisks2 + normal polkit | `.deb` + `.rpm` (both archs) + x86_64 `.AppImage` |

## In scope (v1)

- Physical USB mass storage: USB flash drives, external SSDs/HDDs (incl. USB-to-SATA/NVMe bridges, standard USB hubs).
- Single- and multi-partition devices **when the backend can determine complete eject scope**.
- macOS: APFS containers and physical-store mappings.
- Filesystems: FAT32/exFAT where supported; NTFS on Windows; APFS on macOS; ext4 on Linux.
- Hotplug detection, tray/menu-bar (where supported), full window with same actions, one-click safe eject, evidence-based blocker details, opt-in background retry, success notification.

## Out of scope / declined (v1)

- Network mounts, disk images, optical media, purely internal disks, mobile OSes.
- Ambiguous logical-device stacks; multi-store APFS or unclear ownership/eject scope.
- Always-elevated operation as a default requirement; drivers/services; force-eject/handle-close/process-kill.
- Launch at login (off by default; may follow v1 if packaging permits).

## Notes

- **Blocker details** depend on OS evidence and permissions; copy adapts (OS-attributed vs. verified open file vs. busy fallback).
- **Tray availability** varies (Linux desktops); full window remains usable; normal launch + visible window must work regardless of tray host.
- Hardware used for release testing must be able to exercise the native eject path (USB stick, UASP SSD, external HDD, dual-partition drive, multi-slot reader/enclosure).