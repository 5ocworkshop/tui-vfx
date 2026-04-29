<!-- <FILE>docs/new_kernel/K2_9_SIMPLE_MASK_DESCRIPTOR_DECISION_REPORT.md</FILE> - <DESC>K2.9 simple mask descriptor decision report</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>K2.9 descriptor discipline: accept only simple masks with clear v3.1 semantics.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — record simple mask descriptor decisions and deferrals.</CLOG> -->

# K2.9 Simple Mask Descriptor Decision Report

## Decision summary

Accepted four separate descriptors for K2.9:

```text
mask.blinds
mask.radial
mask.iris
mask.diamond
```

Rejected collapsing `mask.radial`, `mask.iris`, and `mask.diamond` into one broad geometry descriptor. The accepted vocabulary keeps origin/aperture concepts separated and avoids forcing unrelated inputs into one descriptor too early.

No v3.1 schema change was required for this packet. The pressure was descriptor, fixture, and player-adapter coverage.

## Accepted descriptors

| Descriptor | Decision | Inputs | Rationale |
|---|---|---|---|
| `mask.blinds` | Accepted | `orientation`, `count` | Legacy active recipe has direct band-orientation and count evidence; text-grid bands can render an honest smoke adapter. |
| `mask.radial` | Accepted | `origin`, `softEdge` | Legacy evidence is center-origin radial aperture; `origin` is intentionally center-only until non-center semantics exist. |
| `mask.iris` | Accepted | `shape`, `softEdge` | Legacy evidence uses circle/diamond aperture shape; the descriptor owns aperture shape, not radial origin. |
| `mask.diamond` | Accepted | `softEdge` | Legacy evidence has a centered diamond aperture; no authored radius/invert semantics were accepted. |

Descriptor pack path:

```text
descriptors/v3.1/packs/primitive.json
```

## Accepted adapter substrate

The K2.9 adapters are text-grid adapters only:

```text
crates/tui-vfx-player/src/fnc_apply_simple_mask_primitives.rs
```

| Function | Effect id | Evidence quality |
|---|---|---|
| `apply_mask_blinds` | `mask.blinds` | Honest binary band reveal. |
| `apply_mask_radial` | `mask.radial` | Honest binary circular reveal; coarse `softEdge`. |
| `apply_mask_iris` | `mask.iris` | Honest binary circle/diamond aperture reveal; coarse `softEdge`. |
| `apply_mask_diamond` | `mask.diamond` | Honest binary diamond aperture reveal; coarse `softEdge`. |

`softEdge` remains degraded evidence: it expands the reveal threshold slightly in text-grid space. It does not claim alpha feather parity.

## Deferrals and non-decisions

| Candidate field/concept | Decision | Reason |
|---|---|---|
| `radius` | Deferred | Not present in accepted legacy simple-mask payloads; current adapter derives radius from dimensions and `phase_t`. |
| `progress` | Rejected as descriptor input | Runtime progress is `PlayerSampleRequest.phase_t`, not authored descriptor data. |
| numeric `feather` | Deferred | Text-grid cannot prove real feather/alpha semantics. |
| `invert` | Deferred | No accepted simple-mask payload requires it; could fake enter/exit behavior without lifecycle discipline. |
| `center` field | Rejected | Use `origin` for radial center-only semantics; iris/diamond are implicitly centered. |
| `mask.wipe` corner/center expansion | Deferred | Corner and center wipe semantics require a broader descriptor/adapter decision. |
| square companion fixtures | Rejected as canonical targets | `mask_radial_square`, `mask_iris_square`, and `mask_diamond_square` are geometry-clarity variants and oracle evidence only. |
| style/shader/mask combination fixtures | Deferred | Useful future complex-composition evidence, but out of scope for simple primitive mask fixtures. |

## Sub-agent review synthesis

| Lane | Result | Adopted decision |
|---|---|---|
| B1 blinds/wipe map | PASS | Accept `mask.blinds`; keep deprecated fixtures as oracle-only; defer wipe corner/fade combinations. |
| B2 radial/iris/diamond map | PASS WITH FIX | Accept separate descriptors; classify square companions as `duplicateOrVariant`. |
| B3 adjacent style/shader scan | PASS | Keep K2.9 fixtures mask-only; defer complex style/shader combinations. |

## Remaining risks

- Text-grid masks prove deterministic cell hiding/revealing only; compositor visual parity remains future work.
- `mask.checkers` interactive playback was reported as visually incomplete in one recipe and should be investigated separately.
- Future complex recipes should not reuse these descriptors to hide unresolved lifecycle or composition semantics.

<!-- <FILE>docs/new_kernel/K2_9_SIMPLE_MASK_DESCRIPTOR_DECISION_REPORT.md</FILE> - <DESC>K2.9 simple mask descriptor decision report</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
