<!-- <FILE>docs/design/tui-vfx-v3-validator-canonicalization-checklist.md</FILE> - <DESC>Execution checklist for the V3 validator/canonicalization phase. Tracks the minimum checks and canonical outputs needed before broad family runtime implementation should proceed.</DESC> -->
<!-- <VERS>VERSION: 0.4.0</VERS> -->
<!-- <WCTX>VC-06 now has both normalized contract discovery and an opt-in strict validation gate after the binding-focused V3 debug corpus declared its runtime binding contracts.</WCTX> -->
<!-- <CLOG>0.4.0: mark VC-06 complete after pipeline-validator --rules --strict-contracts and V3 debug-corpus requires_bindings declarations landed in tui-vfx-recipes. 0.3.0: mark VC-06 complete-initial after normalized contract declaration checks and contract_usage reporting landed in tui-vfx-recipes; shift the next validator slice to corpus-compatible strict contract gates. 0.2.0: mark VC-08 complete after pipeline-validator --dump-normalized landed in tui-vfx-recipes; convert the tracker from all-open seed state to an as-built plan for the remaining validator/canonicalization work. 0.1.0: initial checklist. Seeds the concrete validation/canonicalization work items following the schema, catalog, lowering, and normalized-IR phases.</CLOG> -->

# tui-vfx V3 validator / canonicalization checklist

## Status tracker

| ID    | Check area                          | Status           | Notes                                                                                                                                                                                                                        |
| ----- | ----------------------------------- | ---------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| VC-01 | Authoring schema validation         | IN_PROGRESS      | V3 parse/load path exists in `tui-vfx-recipes::v3`; remaining work is stricter authoring-shape diagnostics and schema-report surfacing in tools                                                                              |
| VC-02 | Region-ref / compression resolution | IN_PROGRESS      | Initial normalized scope/region canonicalization exists; remaining work is coverage for unresolved refs and impossible resolved ranges                                                                                       |
| VC-03 | Style normalization validation      | IN_PROGRESS      | Initial `base_style` normalization exists; remaining work is validation that no dual style forms survive normalized IR                                                                                                       |
| VC-04 | Hint producer/consumer validation   | COMPLETE_INITIAL | First-class I/O plus legacy `emits_hint`/`binds` validation covers sequence visibility, parallel isolation, duplicate producers, missing producers, and value-kind mismatches                                                |
| VC-05 | Scene-layer placement validation    | IN_PROGRESS      | Initial scene placement/surface/default normalization exists; remaining work is stricter sibling-placement diagnostics and impossible surface checks                                                                         |
| VC-06 | Contract discovery validation       | COMPLETE         | Normalized declaration-shape checks and `contract_usage` reporting are in place; binding-focused V3 debug recipes now declare `requires_bindings`; `pipeline-validator --rules --strict-contracts` is the opt-in strict gate |
| VC-07 | Lowering invariant checks           | OPEN             | V2→V3 migration/lowering invariants still need a dedicated report path                                                                                                                                                       |
| VC-08 | Normalized IR dump / debug output   | COMPLETE         | `pipeline-validator --dump-normalized --format json` now emits the canonical normalized V3 IR through `RecipeLoadMode::Normalized`; `dump_normalized_recipe_pretty` remains the library helper                               |
| VC-09 | Migration-equivalence checks        | PARTIAL          | Critical V3 fixtures have render-hash/probe coverage; V2↔V3 equivalence reports remain follow-on work                                                                                                                        |
| VC-10 | Human-review-needed report          | OPEN             | Needed for unresolved lowering classes and migration review queues                                                                                                                                                           |

## Current go/no-go read

The original go/no-go rule was to avoid broad runtime family work until VC-01,
VC-02, VC-03, VC-04, VC-05, VC-07, and VC-08 had credible implementation
plans or initial code. That bar is now mostly met for the direct V3 path:

- VC-01/02/03/05 have first-pass code paths and focused tests.
- VC-04 has meaningful I/O visibility and kind-validation coverage.
- VC-08 has both library and CLI inspection surfaces.
- VC-07 still needs the clearest remaining validator-side plan before migration
  automation grows.

Because runtime-family work has already moved ahead through the compiled
execution plan, the validator follow-on should be incremental and non-breaking:
add diagnostics/reporting first, then promote specific contract mistakes to hard
errors after the V3 debug corpus is compatible.

## Completed initial VC-06 slice: contract discovery validation

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

## Next slices after VC-06

1. **VC-02/VC-05 strict diagnostics:** unresolved region references, impossible
   ranges, and invalid sibling placement should become hard load/validation
   errors.
2. **VC-07 lowering invariants:** add a report that says which lowerings were
   automatic, which were lossy, and which require human review.
3. **VC-09 migration-equivalence harness:** compare critical V2/V3 pairs through
   normalized intent and render/probe evidence.
4. **VC-10 human-review-needed report:** turn unresolved lowering classes into a
   machine-readable queue for migration work.

## Tooling evidence

Primary inspection command after VC-08:

```sh
cd /usr/projects/tui-vfx-recipes
cargo run -q -p pipeline-validator -- \
  --dump-normalized --format json \
  recipes/debug_recipes/scene/scene_braille_flag_runtime_wave.json
```

Expected verification lane for the next validator slice:

```sh
cargo fmt --all --check
cargo test -p tui-vfx-recipes v3::validate
cargo test -p pipeline-validator
cargo run -q -p pipeline-validator -- --rules --strict-contracts \
  recipes/debug_recipes/scene/scene_braille_flag_runtime_wave.json
python3 tools/fnc_generate_v3_docs.py --check
git diff --check
```

<!-- <FILE>docs/design/tui-vfx-v3-validator-canonicalization-checklist.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.4.0</VERS> -->
