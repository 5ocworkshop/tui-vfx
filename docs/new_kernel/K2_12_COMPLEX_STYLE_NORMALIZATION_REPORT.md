<!-- <FILE>docs/new_kernel/K2_12_COMPLEX_STYLE_NORMALIZATION_REPORT.md</FILE> - <DESC>K2.12 complex and style normalization report</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>K2.12 schema lock: remove generic complex owner-audit and unknown-style buckets.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — document exact complex and style blocker classifications.</CLOG> -->

# K2.12 Complex and Style Normalization Report

## Complex owner-audit result

All 73 complex owner-audit records are now normalized into explicit readiness kinds. The opt-in offender ledger emits the same mechanical categories via `schema-readiness --include-offenders`.

| Complex cluster | Records | Offender kind | Blocks schema-readiness declaration now? | Disposition |
|---|---:|---|---|---|
| Descriptor composition | 38 | `descriptorPack` | Yes, until queued/accepted/held | Descriptor expansion queue; not a schema-model rename. |
| Scene-local pipeline | 14 | `sceneSemantics` | Yes | Decide sequence/parallel/local ordering and scope semantics. |
| Runtime dynamism | 9 | `valueSourceSemantics` | Yes | Resolve `signal`, `source`, `binds`, and field hints. |
| Source/content pipeline | 8 | `sourceDescriptor` | Yes | Decide non-card source/content descriptors. |
| GUI human review | 2 | `guiHumanReview` | Yes until owner signs holdback | Hold back as visual conflict/human-review fixtures. |
| Backend renderer | 1 | `backendRenderer` | Yes until owner signs holdback | Hold back behind backend/compositor boundary. |
| Oracle-only | 1 | `oracleOnly` | No | Offline command-capture oracle only. |

## Exact complex holdbacks now encoded

- `sceneSemantics`: `complex_filter_to_mask_sourced_output`, `complex_nested_parallel_sequences`, all parallel/sequence local-pipeline fixtures, and shadow/style sequence variants.
- `guiHumanReview`: `complex_parallel_overlap_conflict_snapshot.json`, `v3_scheduler_overlap_conflict_mixed_family.json`.
- `backendRenderer`: `complex_shadow_mask_sampler_shader_filter_native_mix.json`.
- `oracleOnly`: `complex/command_capture_chain.json`.

## Unknown style result

The five K2.11 unknown style records are not left as `unknown`; they are classified as scope/content descriptor vocabulary work:

| Style record | Required vocabulary | Offender kind | Disposition |
|---|---|---|---|
| `styles/style_modulo_horizontal_every_third_row.json` | modulo row scope: axis, modulus, remainder | `contentDescriptor` | Scope vocabulary decision. |
| `styles/style_modulo_vertical_every_fourth_column_offset.json` | modulo column scope with offset remainder | `contentDescriptor` | Scope vocabulary decision. |
| `styles/style_non_empty_scope.json` | content/non-empty scope | `contentDescriptor` | Scope vocabulary decision. |
| `styles/style_outer_scope_band.json` | outer/perimeter band scope | `contentDescriptor` | Scope vocabulary decision. |
| `styles/style_predicate_interior.json` | predicate/ref scope registry | `contentDescriptor` | Highest risk; decide predicate registry semantics. |

## Evidence snapshot

```text
schema-readiness --include-offenders offender count: 386
complex offender kinds: descriptorPack=38, sceneSemantics=14, valueSourceSemantics=9, sourceDescriptor=8, guiHumanReview=2, backendRenderer=1, oracleOnly=1
unknown offender kinds: none
ownerAudit offender kinds: none
```

The raw summary fields still expose K2.11 migration statuses for continuity; this report is about offender-row normalization.

<!-- <FILE>docs/new_kernel/K2_12_COMPLEX_STYLE_NORMALIZATION_REPORT.md</FILE> - <DESC>K2.12 complex and style normalization report</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
