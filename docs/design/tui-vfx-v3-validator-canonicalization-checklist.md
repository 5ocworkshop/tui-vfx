<!-- <FILE>docs/design/tui-vfx-v3-validator-canonicalization-checklist.md</FILE> - <DESC>Execution checklist for the V3 validator/canonicalization phase. Tracks the minimum checks and canonical outputs needed before broad family runtime implementation should proceed.</DESC> -->
<!-- <VERS>VERSION: 0.9.1</VERS> -->
<!-- <WCTX>Track the current validator/canonicalization checklist state across tui-vfx planning docs and tui-vfx-recipes executable validator/tooling slices.</WCTX> -->
<!-- <CLOG>0.9.2: reconcile stale validator statuses with as-built schema, style, and dump evidence.</CLOG> -->

# tui-vfx V3 validator / canonicalization checklist

## Status tracker

| ID    | Check area                          | Status           | Notes                                                                                                                                                                                                                        |
| ----- | ----------------------------------- | ---------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| VC-01 | Authoring schema validation         | COMPLETE_INITIAL  | V3 parse/load path exists in `tui-vfx-recipes::v3`; `pipeline-validator` has a focused regression for malformed `metadata` shape in `--rules`; broader schema-report surfacing remains on `V3-VC01` |
| VC-02 | Region-ref / compression resolution | COMPLETE_INITIAL | Region refs/cycles already fail in normalization; normalized validation now rejects impossible literal row/column/cell-run ranges and non-positive rect dimensions while allowing dynamic binding-backed scope values        |
| VC-03 | Style normalization validation      | COMPLETE_INITIAL  | `normalize_base_style_into_base_style_override` and `validate_rejects_dual_style_form_survivors_in_leaf_payloads` cover the initial canonicalization/validation pair; broader equivalence remains on `V3-VC03` |
| VC-04 | Hint producer/consumer validation   | COMPLETE_INITIAL | First-class I/O plus legacy `emits_hint`/`binds` validation covers sequence visibility, parallel isolation, duplicate producers, missing producers, and value-kind mismatches                                                |
| VC-05 | Scene-layer placement validation    | COMPLETE         | Normalized validation rejects duplicate layer IDs, bad sibling refs, malformed placement/surface shapes, absolute sibling misuse, and non-positive absolute rect dimensions                                                  |
| VC-06 | Contract discovery validation       | COMPLETE         | Normalized declaration-shape checks and `contract_usage` reporting are in place; binding-focused V3 debug recipes now declare `requires_bindings`; `pipeline-validator --rules --strict-contracts` is the opt-in strict gate |
| VC-07 | Lowering invariant checks           | COMPLETE_INITIAL | `pipeline-validator --lowering-report --format json` reports automatic normalized/compiled lowering, lossless legacy-I/O lifting, scene-layer homes, and dynamic-scope human-review flags                                    |
| VC-08 | Normalized IR dump / debug output   | COMPLETE         | `pipeline-validator --dump-normalized --format json` now emits the canonical normalized V3 IR through `RecipeLoadMode::Normalized`; `dump_normalized_recipe_pretty` remains the library helper                               |
| VC-09 | Migration-equivalence checks        | PARTIAL          | Critical V3 fixtures have render-hash/probe coverage; V2↔V3 equivalence reports remain follow-on work                                                                                                                        |
| VC-10 | Human-review-needed report          | COMPLETE_INITIAL | `pipeline-validator --lowering-report --format json` now emits top-level and per-recipe `human_review_needed` queue entries keyed by lowering class/invariant so migration tooling can consume unresolved review work directly |

## Current go/no-go read

The original go/no-go rule was to avoid broad runtime family work until VC-01,
VC-02, VC-03, VC-04, VC-05, VC-07, and VC-08 had credible implementation
plans or initial code. That bar is now mostly met for the direct V3 path:

- VC-01/03 have first-pass code paths and focused tests.
- VC-02/05 now have initial hard validation for the highest-risk structural
  mistakes.
- VC-04 has meaningful I/O visibility and kind-validation coverage.
- VC-07/08 have both library/tooling evidence surfaces for migration review.

Because runtime-family work has already moved ahead through the compiled
execution plan, the validator follow-on should be incremental and non-breaking:
add diagnostics/reporting first, then promote specific contract mistakes to hard
errors after the V3 debug corpus is compatible.

## Completed initial VC-02 slice: literal scope/range validation

VC-02 now rejects impossible static scopes after region refs and selector sugar
normalize:

1. **Resolved-region baseline**
   - unresolved region refs and region-ref cycles already fail during
     normalization.
   - validator scope checks run after that normalized form is available.
2. **Literal range sanity**
   - `row_range` and `column_range` reject literal `start > end`.
   - `cell_run`/`cell_runs` reject literal `x_start > x_end`.
   - `rect`/`rect_exclude` reject literal non-positive `w` or `h`.
3. **Runtime-binding compatibility**
   - dynamic JSON leaves such as `{ "binding": "x0" }` remain validation-time
     permissive because host-provided runtime params are unavailable during
     normalization.
   - runtime range validation can be added later if a concrete runtime-param
     context is explicitly available.

## Completed VC-05 slice: scene geometry/surface validation

VC-05 now catches the static scene-layer mistakes that previously could leak to
the typed compile seam:

1. **Layer identity and sibling graph**
   - duplicate IDs fail.
   - self-sibling and unknown sibling placement references fail.
   - non-string `sibling_id` values fail before traversal.
2. **Placement shape**
   - placement must be the tagged V3 scene shape with `type` and `spec`.
   - supported placement types are currently `anchor` and `absolute`.
   - `absolute` placement cannot carry sibling references.
3. **Geometry sanity**
   - absolute placement `rect.width` and `rect.height` must be positive
     integers.
   - this is intentionally static geometry validation; clipping and runtime
     layout behavior remain scene composition responsibilities.
4. **Surface shape**
   - scene-layer `surface` must be an object.
   - only `base_style` and `shadow` are accepted at the normalized surface
     level, and each must be an object when present.
5. **Canonical defaults**
   - missing authored placement now normalizes to the tagged anchor shape:
     `{ "type": "anchor", "spec": { "anchor": "default" } }`.

## Completed VC-06 slice: contract discovery validation

VC-06 now connects the working I/O pathway, asset-backed Madeira flag work,
and normalized inspection surface:

1. **Declaration-shape validation**
   - `requires_bindings` entries require a declared string `type`.
   - `requires_assets` entries require a non-empty `canonical_path`.
   - malformed `requires_*` section shapes fail normalized validation with
     explicit `ValidateError` variants.
2. **Usage discovery report**
   - `collect_contract_usage_report` discovers `{{token_or_asset}}` template
     references and `{ "binding": "key" }` runtime binding references from the
     normalized recipe.
   - `pipeline-validator --dump-normalized --format json` includes a
     `contract_usage` block with discovered and undeclared usage sets.
3. **Debug recipe proof**
   - `recipes/debug_recipes/scene/scene_braille_flag_runtime_wave.json` proves a
     declared runtime binding (`wave_speed`) and declared file-backed asset token
     (`flag_art`) appear as declared contract usage in normalized dump output.
4. **Strict gate**
   - binding-focused V3 debug fixtures now declare `requires_bindings`.
   - `pipeline-validator --rules --strict-contracts` rejects undeclared runtime
     binding or template-placeholder usage while normal `--rules` stays
     backward-compatible.

## Completed initial VC-07 slice: lowering invariant report

VC-07 now exposes migration/lowering evidence without requiring a full automatic
V2→V3 migrator:

1. **Report surface**
   - `pipeline-validator --lowering-report --format json <recipe>` emits
     `kind: "v3_lowering_report"`.
   - each recipe includes metrics plus invariant rows with `automatic`,
     `lossless`, `human_review_required`, or `not_applicable` status.
2. **Automatic lowering evidence**
   - recipes load through the canonical normalized V3 IR.
   - normalized recipes compile to typed execution plans without late schema
     recovery.
   - scene layers are reported as preserved in their source/placement/surface/
     pipeline homes when present.
3. **Lossless compatibility evidence**
   - legacy `emits_hint`/`binds` payload edges are counted and reported as
     lossless compiled step-I/O preservation.
4. **Human-review queue seed**
   - dynamic runtime-bound scopes are counted and classified as
     `human_review_required`, giving migration work a concrete queue before
     broader V2/V3 equivalence automation lands.

## Completed initial VC-10 slice: machine-readable human-review-needed queue

VC-10 now turns lowering-report human-review flags into a direct queue surface
for migration work:

1. **Reuse the existing lowering-report mode**
   - `pipeline-validator --lowering-report --format json` remains the canonical
     entry point.
   - the queue rides alongside the existing lowering invariants and metrics
     instead of creating a parallel report format.
2. **Per-recipe queue entries**
   - each recipe report now includes `human_review_needed[]`.
   - queue entries carry a stable `class_id`, the source `invariant_id`, a
     machine-readable `count`, and the human-readable detail text.
3. **Top-level migration queue**
   - the report root now includes a flattened `human_review_needed[]` array with
     `path` and `recipe_id` so migration tooling can consume review work across
     a corpus without re-walking nested recipe objects.
4. **Initial unresolved class**
   - dynamic runtime-bound scopes are currently surfaced as
     `class_id = "dynamic_scope_runtime_binding"` sourced from
     `LM-scope.dynamic_values`.
   - future unresolved lowering classes should extend this queue surface rather
     than inventing a new report family.

## Next slices after VC-10

1. **VC-09 migration-equivalence harness:** compare critical V2/V3 pairs through
   normalized intent and render/probe evidence.
2. **Extend VC-10 coverage:** add additional unresolved lowering classes to the
   queue as validator/lowering work discovers new review-needed categories
   beyond dynamic runtime-bound scopes.

## Tooling evidence

Primary inspection command after VC-08:

```sh
cd /usr/projects/tui-vfx-recipes
cargo run -q -p pipeline-validator -- \
  --dump-normalized --format json \
  recipes/debug_recipes/scene/scene_braille_flag_runtime_wave.json
cargo run -q -p pipeline-validator -- \
  --lowering-report --format json \
  recipes/debug_recipes/styles/style_cell_position_binding.json
```

Expected verification lane for the next validator slice:

```sh
cargo fmt --all --check
cargo test -p tui-vfx-recipes v3::validate
cargo test -p pipeline-validator
cargo test -p tui-vfx-recipes
python3 tools/fnc_generate_v3_docs.py --check
git diff --check
```

<!-- <FILE>docs/design/tui-vfx-v3-validator-canonicalization-checklist.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.8.0</VERS> -->
