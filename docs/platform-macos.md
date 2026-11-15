# Platform notes — macOS

See `docs/platform-windows.md` header for conventions. Content for macOS-specific native API details lives in the referenced stub implementation; fill in per implementation plan when implementing Disk Arbitration bindings.

Key acceptance items:
- Disk Arbitration registration (appear/disappear/change)
- Whole-disk resolution + APFS physical-store mapping
- `DADiskUnmount` → `DADiskEject`, `DADissenter` phases
- Bounded `lsof` diagnostics, Full Disk Access outside default flow
- Universal build, Developer ID, notarize, staple, Gatekeeper per CPU