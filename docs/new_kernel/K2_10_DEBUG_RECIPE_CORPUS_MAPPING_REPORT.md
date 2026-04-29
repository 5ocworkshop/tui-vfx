<!-- <FILE>docs/new_kernel/K2_10_DEBUG_RECIPE_CORPUS_MAPPING_REPORT.md</FILE> - <DESC>K2.10 corpus-wide debug recipe migration mapping report</DESC> -->
<!-- <VERS>VERSION: 0.2.0</VERS> -->
<!-- <WCTX>K2.10 corpus-wide migration mapping and backlog board.</WCTX> -->
<!-- <CLOG>0.2.0: PATCH — downgrade value-source-shaped filter records after architecture review.
0.1.0: INIT — record corpus-wide migration mapping evidence and next-packet backlog.</CLOG> -->

# K2.10 Debug Recipe Corpus Mapping Report

## Scope

- Legacy evidence root: `../tui-vfx-recipes/recipes/debug_recipes`
- Canonical v3.1 root: `../tui-vfx-recipes/recipes/v3.1/debug_recipes`
- Descriptor pack: `descriptors/v3.1/packs/primitive.json`
- Report schema: `v3.1.player.migrationMappingBatch.1`
- Legacy root mutation policy: read-only.

## Recursive report summary

| Metric | Count |
|---|---:|
| Families | 18 |
| Records | 603 |
| canonicalExists | 21 |
| candidateReady | 0 |
| descriptorDecisionNeeded | 151 |
| schemaDecisionNeeded | 72 |
| ownerAuditNeeded | 280 |
| sourceDecisionNeeded | 67 |
| blockedByFieldCoverage | 4 |
| duplicateOrVariant | 3 |
| notYetClassified | 5 |

## Per-family status counts

| Family | Records | Main statuses |
|---|---:|---|
| other | 2 | canonicalExists 1; ownerAuditNeeded 1 |
| filters | 98 | canonicalExists 4; schemaDecisionNeeded 3; descriptorDecisionNeeded 46; ownerAuditNeeded 45 |
| masks | 41 | canonicalExists 8; descriptorDecisionNeeded 15; duplicateOrVariant 3; ownerAuditNeeded 15 |
| samplers | 13 | canonicalExists 2; descriptorDecisionNeeded 5; ownerAuditNeeded 6 |
| shaders | 133 | canonicalExists 3; descriptorDecisionNeeded 72; blockedByFieldCoverage 4; ownerAuditNeeded 54 |
| styles | 34 | canonicalExists 2; descriptorDecisionNeeded 13; notYetClassified 5; ownerAuditNeeded 14 |
| content | 111 | sourceDecisionNeeded 66; ownerAuditNeeded 45 |
| scene | 19 | schemaDecisionNeeded 19 |
| fixtures | 1 | sourceDecisionNeeded 1 |
| event_driven_dwell | 4 | canonicalExists 1; schemaDecisionNeeded 3 |
| bindable_rates | 8 | schemaDecisionNeeded 8 |
| easings | 29 | schemaDecisionNeeded 29 |
| motion_routes | 5 | schemaDecisionNeeded 5 |
| signals | 5 | schemaDecisionNeeded 5 |
| loopback | 3 | ownerAuditNeeded 3 |
| complex | 83 | ownerAuditNeeded 83 |
| shadows | 9 | ownerAuditNeeded 9 |
| subcell_shapes | 5 | ownerAuditNeeded 5 |

## Classification notes

- The report now extracts effect descriptors from legacy `kind`/`payload.type` evidence beyond masks.
- Unknown or schema-sensitive families do not default to `candidateReady`.
- Remaining mask records stay descriptor-decision gated to preserve the K2.9 masks report behavior.
- Source/content records use source descriptor decisions rather than forcing content effects into `source.card`.
- Timing, motion, binding, loopback, complex, shadow, and subcell records are planning evidence, not migration-ready fixtures.

## Value-source decision records

| Legacy path | Why blocked |
|---|---|
| `filters/filter_dim_sample_surface_angle_from.json` | `filter.dim.factor` is value-source/signal-shaped legacy evidence, not a plain descriptor-backed value. |
| `filters/filter_dim_sample_surface_radius.json` | `filter.dim.factor` is value-source-shaped legacy evidence, not a plain descriptor-backed value. |
| `filters/filter_dim_sample_surface_radius_from.json` | `filter.dim.factor` is value-source-shaped legacy evidence, not a plain descriptor-backed value. |

## Field-coverage blockers

| Legacy path | Descriptor | Unsupported fields |
|---|---|---|
| `shaders/primitives/shader_linear_gradient_diagonal.json` | `shader.linearGradient` | `gradient` |
| `shaders/primitives/shader_linear_gradient_background_channel.json` | `shader.linearGradient` | `gradient` |
| `shaders/primitives/shader_linear_gradient_apply_to_both.json` | `shader.linearGradient` | `applyTo`, `gradient` |
| `shaders/compositions/shader_border_sweep_position_binding.json` | `shader.borderSweep` | `position` |

<!-- <FILE>docs/new_kernel/K2_10_DEBUG_RECIPE_CORPUS_MAPPING_REPORT.md</FILE> - <DESC>K2.10 corpus-wide debug recipe migration mapping report</DESC> -->
<!-- <VERS>END OF VERSION: 0.2.0</VERS> -->
