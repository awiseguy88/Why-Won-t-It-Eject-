# Release Runbook — Why Won’t It Eject

## Before tagging

1. Default branch protected; CI checks green.
2. Third-party GitHub Actions pinned to full commit SHAs; dependency lockfiles committed.
3. Signing identities stored in release-only environment; Authenticode timestamps set; Windows signing via SignTool; Mac signing + notarization + staple; Linux built on validated baseline + dependency checks on supported distros.
4. Record exact versions and build inputs (Rust, Node/pnpm, Tauri, OS SDKs, signing cert serials/fingerprints — not private material).

## Release-candidate flow

1. Build RC as a **GitHub prerelease** from a tagged commit (e.g. `v1.0.0-rc.N`).
2. Run smoke tests on final binaries per platform/arch on clean systems.
3. Compute `SHA256SUMS.txt` after signing/stapling.
4. Create provenance attestations (GitHub artifact attestations) for every final public artifact.
5. Assemble a **draft** release. Publish job compares expected file manifest to actual uploaded filenames/hashes before publishing. Use immutable releases when available.
6. If candidate changes after release → new patch tag (`v1.0.1`); history remains verifiable.

## v1.0.0 publish steps

1. Tag `v1.0.0` after final source approval.
2. Build, sign, run smoke tests on final binaries.
3. `SHA256SUMS.txt` after signing/stapling.
4. Provenance attestations for all final artifacts.
5. Draft release → compare manifest → publish.
6. Release notes: exact support matrix, install/prerequisites/verification instructions, how to pick the right binary, Supported devices section, blocker-evidence caveat.
7. Link release at top of README with download table.

## Post-release

- Keep release history verifiable; do not overwrite tags.
- Add hardware QA results from RC matrix to compatibility notes.
- If new platform/arch support needed → new milestone tag, not rewriting v1.0.0 assets.