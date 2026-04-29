# Holdback Register

Holdbacks below are not schema blockers for v3.1 schema readiness. They are explicit implementation/evidence backlog.

| Class | Legacy paths | Disposition | Reason | Future evidence required |
| --- | --- | --- | --- | --- |
| Backend renderer | `shadows/*`, `subcell_shapes/*`, shadow/subcell complex mixes | `backendHoldback` | Requires compositor/subcell/shadow backend evidence behind the player adapter seam. | Backend adapter, visual evidence, fixture-QC lane, no direct UI compositor dependency. |
| GUI human review | `complex/complex_parallel_overlap_conflict_snapshot.json`, `complex/v3_scheduler_overlap_conflict_mixed_family.json` | `guiHumanReviewHoldback` | Conflict policy needs reviewable visual/authoring decision. | Deterministic conflict diagnostics and owner signoff. |
| Oracle-only | command-capture artifacts, deprecated legacy records, offline loopback/demo artifacts | `oracleOnly` | Runtime command execution is explicitly out of scope. | Offline authoring/export packet only; no runtime execution. |
| Duplicate/variant | `masks/mask_diamond_square.json`, `masks/mask_iris_square.json`, `masks/mask_radial_square.json` | `duplicateVariant` | Covered by existing canonical masks or variant equivalence. | No release evidence unless owner wants variant fixtures. |

Backend policy: compositor-backed evidence belongs behind a player/backend adapter boundary. `tui-vfx-player-ui` must consume player evidence and must not directly construct compositor internals.
