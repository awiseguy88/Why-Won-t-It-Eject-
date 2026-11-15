# Security Policy

## Supported versions

Security fixes target the latest released version and the default branch during active development toward v1.0.0.

## Reporting a vulnerability

Please **do not** report security vulnerabilities through public GitHub issues.

Instead, report privately to the maintainers via the repository's GitHub Security Advisories ("Report a vulnerability") if enabled, or by contacting the project owner through a private channel listed on the repository profile.

Please include:

- A description of the issue and potential impact
- Steps to reproduce or a proof of concept (use disposable/fictitious data only)
- Affected platform(s), OS version(s), and app version/commit
- Any suggested mitigation

We will acknowledge receipt as soon as practical, investigate, and coordinate a fix and disclosure timeline. Because this tool interacts with storage devices and OS eject APIs, we treat **wrong-device targeting, false “Safe to unplug” states, privilege escalation, and arbitrary command/path execution** as critical classes.

## Security posture (v1 design)

- Standard user process; no kernel driver, no background service, no force-eject/handle-close/process-kill.
- Native authorization only through normal OS mechanisms on direct user action; automatic retries stop if new authorization is required.
- Local-only operation: no accounts, telemetry, remote calls, or auto-update. Preferences stored locally; paths/process evidence in memory; redacted diagnostic export explicit.
- Frontend uses typed Tauri commands with bundled assets and local-only policy; no arbitrary shell execution.
- Release artifacts: signed where applicable, checksums, and build provenance per `docs/release-runbook.md`.