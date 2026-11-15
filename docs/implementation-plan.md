# Implementation Plan & Milestones — Why Won’t It Eject?

Milestones map to the plan’s implementation order. Each item has an observable finish condition. Target solo effort: **six to ten focused weeks** assuming hardware access + signing enrollment early.

## Milestone: Native proof of concept

1. **Prove native eject on all three OSes** — minimal harness on disposable real USB drive per OS: idle eject, interrupted eject, busy veto, restricted permissions, one multi-volume drive. Test build/launch on both CPU families early.
   - *Finish:* Recorded raw native results on real hardware per OS (OS build, arch, fs, bridge, result).
2. **Prove device identity & removal scope** — standard USB maps to correct physical target; system disks, duplicate labels, drive-letter/node reuse fixtures behave correctly; APFS + sibling-reader topology validated.
   - *Finish:* Fixtures + notes pass for identity/scope cases; no wrong-device mutation.

## Milestone: Preview

3. **Shared core + fake backend** — deterministic tests: consent, partial ops, device generation, timer expiry, cancel, concurrent clicks, late responses.
   - *Finish:* `cargo test -p eject-core` green including fake-backend timing tests (mocked clocks).
4. **Windows vertical slice** — discovery → eject → explanation → retry → notification. Test on real hardware before tagging Windows preview.
   - *Finish:* Windows preview tag + portable files only after hardware slice passes.

## Milestone: Cross-platform beta

5. **Integrate macOS and Linux** — each adapter runs same usability flow, produces native evidence; Linux full-window path works without tray. Tag cross-platform beta after first physical pass per OS.
   - *Finish:* Beta tag with macOS + Linux evidence recorded.

## Milestone: v1.0.0

6. **Refine diagnosis & accessibility** — attributed vs. observed blockers distinct; known blockers resolved in controlled cases; missing evidence → plain busy text; keyboard/scaling/notifications pass per OS.
7. **Packaging & release automation** — installers/portables from tagged commit launch on clean systems; signing, hashes, provenance, draft asset validation succeed.
8. **RC hardware matrix & publish** — every supported OS/CPU has recorded hardware + clean-install pass; final release has exact support matrix, instructions, assets, verified integrity. Tag `v1.0.0`.

## Schedule risks

- Windows PnP topology / scope determination
- APFS synthesized → physical store mapping (macOS)
- Linux sibling drives (UDisks2 `PowerOff`) and polkit/auth behavior

**Mitigation:** limit obscure enclosures initially; preview Windows first while retaining all three platforms for stable tag.

## Definition of done (v1 gate)

- Zero false **Safe to unplug** in fixture + physical matrices.
- Zero mutations on wrong device.
- Zero blocker/critical regressions.
- Representative same-user blocker cases per platform; busy fallback verified when evidence inaccessible.
- Real-device signoff per supported OS/CPU.