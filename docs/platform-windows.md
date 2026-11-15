# Platform docs

Per-platform native API notes. All adapters are scaffolds (stubs) in v1 milestone 1; implement against the acceptance conditions below and `crates/eject-platform/src/{windows,macos,linux}.rs`.

## Windows — `platform-windows.md` (TODO: complete)
- Enumerate volumes by GUID/mount path; resolve disk extents via `IOCTL_VOLUME_GET_VOLUME_DISK_EXTENTS`.
- Prototype `IOCTL_STORAGE_GET_DEVICE_NUMBER` as fallback for demonstrably single-disk removable volumes; corroborate via disk interface enumeration; decline if ambiguous.
- Map disk interface → PnP device instance (SetupAPI / Configuration Manager).
- Zero volumes, multiple extents, missing capabilities, compound enclosures: explicit handling. USB bus type alone insufficient for scope classification.
- Resolve ejectable device node; enumerate affected children; exclude boot/system disk, internal devices, unknown-ancestor removal.
- `CM_Request_Device_EjectW`: classify `CONFIGRET`, veto type, veto name as separate fields; buffer for veto details; do not require always-elevated app.
- Kernel-PnP Event 225 query across tight operation window; exact device-scope match; PID start identity where possible; PID reuse handling; clear stale evidence per attach; no full command-line export.
- Restart Manager: bounded `RmRegisterResources` candidate-file checks; `RmGetList`; individual follow-up or direct evidence before showing a path; folder fallback capped by count/time, skip reparse points; empty scan cannot clear busy.
- Publish signed portable `.exe` + per-user setup `.exe` (x64/ARM64); WebView2 present vs. absent paths; publish runtime prerequisite; test offline install before advertising.

## macOS — `platform-macos.md` (TODO: complete)
- Disk Arbitration: register appearance/disappearance/change via `DADiskAppeared`/`DADiskDisappeared`/`DADiskChanged` on serviced queue/run loop.
- Resolve `DADiskCopyWholeDisk`; APFS synthesized devices back to physical stores; include all affected volumes in scope; exclude system stores/disk images.
- `DADiskUnmount` (whole-disk option) then `DADiskEject` (physical whole disk); inspect `DADissenter` at each phase; partial unmount/eject → report partial, refresh topology, leave volumes observed state.
- Busy diagnostics: bounded read-only `lsof` against selected filesystem; machine-readable output + deadline; verify mount/path association; subprocess errors; privacy/permission limits; check Intel + Apple silicon; Full Disk Access outside default flow.
- Build universal app; verify both Mach-O slices; Developer ID + hardened runtime; notarize DMG; staple; Gatekeeper clean-account download test per Mac CPU.

## Linux — `platform-linux.md` (TODO: complete)
- UDisks2 `ObjectManager` + change signals; map `Drive`/`Block`/`Filesystem`; join device/mount info vs active namespace; protect root/system/boot storage chain, swap, internal/unknown devices; decline unmappable stacks; require UDisks2 + normal polkit.
- `Filesystem.Unmount` per mounted volume in scope; then `Drive.PowerOff` on eligible USB drive; use `CanPowerOff`/`SiblingId`/topology; show every affected drive before proceeding; decline unverifiable scope; revalidate before every unmount and final power-off.
- UDisks2 busy errors = retryable; explicit attempt = normal polkit; auto-retry = `auth.no_user_interaction`; missing service/auth/unsupported hardware result; power-off unsupported → show scope + explain system disk utility.
- Bounded `/proc` scan: fd, cwd, (if feasible) mmap evidence; match fs/device not loose path prefix; expect permission/namespace/vanished/inaccessible.
- Build against oldest declared WebKitGTK/glibc baseline; verify clean-system deps; test GNOME without indicator extension + KDE Wayland; X11 smoke test where advertised.
- Normal launch + visible window works regardless of tray host.