<!-- <FILE>docs/design/tui-vfx-v3-release-gate-policy.md</FILE> - <DESC>Accepted V3 release-gate tolerance, manifest, whitelist, and ownership policy.</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Record the project-owner-approved policy for Chapter 60 release-gate manifests, outcome states, approval ownership, and explicit fixture recapture.</WCTX> -->
<!-- <CLOG>0.1.0: initial accepted release-gate policy with structured outcome states, owner split, whitelist requirements, and no automatic stale-fixture recapture.</CLOG> -->

# V3 release-gate policy

This document records the accepted policy for Chapter 60 V3 release gates.

## Decision

V3 release gates use checked-in structured manifests. Gate outcomes are:

- `pass`
- `fail`
- `accepted_change`
- `stale_fixture`
- `not_applicable`

Implementation/tooling owners can classify and fix library-level failures, but
project-visible visual drift and GT-Design representative fixture selection
require project-owner approval. Fixture recapture is explicit only, never
automatic.

## Gate areas

Chapter 60 tracks six release-gate areas:

1. shadow fixtures
2. offscreen / slide fixtures
3. probe snapshots
4. trace expectations
5. GT-Design integration fixtures
6. role-aware lowering correctness

## Outcome states

| State | Meaning |
|---|---|
| `pass` | V3 output matches expected tolerance. |
| `fail` | Regression or unresolved mismatch. |
| `accepted_change` | V3 differs intentionally and the difference is documented and approved. |
| `stale_fixture` | V2 reference capture is outdated and needs explicit recapture approval. |
| `not_applicable` | Fixture does not apply to that recipe or surface. |

## Ownership

| Gate area | Owner |
|---|---|
| shadow / offscreen / role-aware lowering | tui-vfx implementation lead |
| probe / trace semantic expectations | tui-vfx tooling lead |
| GT-Design representative surfaces | project owner / GTD lead |
| whitelist approval for library semantics | tui-vfx implementation lead |
| whitelist approval for product-visible GTD behavior | project owner / GTD lead |
| stale fixture recapture | explicit approval required; never automatic |

The implementation agent may build and run the manifests, classify mechanical
failures, and fix implementation issues. Project-visible drift and GTD fixture
selection are owner decisions.

## Manifest entry shape

Accepted changes must be checked in as structured manifest entries. Commit
messages alone are not enough.

Example:

```json
{
  "id": "bsod_v3_fullscreen_jitter",
  "recipe": "recipes/debug_recipes/scene/bsod_crash_v3.json",
  "gate": "offscreen_slide",
  "state": "accepted_change",
  "reason": "V3 uses shorter jitter to match intended V2 feel rather than previous overlong V3 timing.",
  "approved_by": "project_owner",
  "approved_at": "2026-04-24",
  "evidence": {
    "v2_fixture": "fixtures/v2/bsod_crash.json",
    "v3_fixture": "fixtures/v3/bsod_crash_v3.json",
    "probe_report": "reports/bsod_v3_probe.json"
  }
}
```

## Recapture rule

Do not silently recapture V2 fixtures.

If a fixture is stale:

1. mark the gate entry `stale_fixture`
2. explain why the reference is stale
3. request explicit recapture approval from the responsible owner
4. preserve the old reference or its provenance until the new capture is
   accepted

## Plan impact

This resolves the release-gate ownership/tolerance decision for:

- Chapter 60 release gates
- VC-09 migration-equivalence reporting
- Chapter 100 tooling/CI cutover
- the V3 master punch list release-gate lanes

Implementation remains in the release-gate manifest/tooling lanes.

<!-- <FILE>docs/design/tui-vfx-v3-release-gate-policy.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
