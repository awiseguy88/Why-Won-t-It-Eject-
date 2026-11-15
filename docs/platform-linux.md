# Platform notes — Linux

Key acceptance items:
- UDisks2 ObjectManager + change signals; Drive/Block/Filesystem mapping
- Protect root/system/boot chain, swap, internal/unknown devices
- `Filesystem.Unmount` per volume → `Drive.PowerOff`; `CanPowerOff`/`SiblingId` scope
- polkit: explicit user attempt vs `auth.no_user_interaction` auto-retry
- Bounded `/proc` scan (fd/cwd/mmap), fs/device matching
- Tray-free full-window path; GNOME/KDE Wayland/X11 smoke tests