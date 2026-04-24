# Packet 55 — V3 spatial leaves and field-hint threading

## Task first
Start the next bounded spatial-field tranche by building the first real typed field-hint runtime seam now that the minimal spatial leaves and basic cell-position threading are already landed.

## Why this matters
The spatial-field-hint plan now shows that Phase 1 (mixed-signals spatial leaves) and the basic Phase 2 cell-position threading are already materially landed. The real missing substrate is Phase 3: typed per-step field hints and one actual producer/consumer runtime seam.

## Success condition
- one bounded typed field-hint phase from the spatial-field-hint plan lands
- the chosen producer/consumer seam is test-proven
- one real V3/runtime consumer path benefits from the new data

## Mode
BLOCKER_MODE

## Task-scope paths for grounding
- `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-spatial-field-hint-plan.md`
- `/usr/projects/tui-vfx-recipes/src/v3/validate/col_collect_hints.rs`
- `/usr/projects/tui-vfx-recipes/src/v3/validate/fnc_validate_normalized_recipe.rs`
- `/usr/projects/tui-vfx-recipes/src/v3/compile/`
- `/usr/projects/tui-vfx-recipes/src/v3/compile/test_render_compiled_plan_deterministically.rs`
- `/usr/projects/tui-vfx-recipes/src/v3/compile/test_build_composition_spec_from_compiled_plan.rs`
- `/usr/projects/mixed-signals/steering/INTENTIONS.md`
- `/usr/projects/tui-vfx/steering/INTENTIONS.md`
- the exact producer/consumer payloads or runtime seam chosen for the tranche

## Exact write scope
- the exact V3 validator/compile/runtime files needed for the chosen typed field-hint seam
- the smallest `mixed-signals` files only if the chosen seam truly requires them
- the narrowest tests proving producer/consumer use

## Out of scope
- the full field/hint architecture in one packet
- generic Madeira parity work outside the chosen producer/consumer path
- unrelated naming cleanup
- redoing the already-landed basic spatial leaves/threading work

## Must-read docs in order
1. `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-spatial-field-hint-plan.md`
2. `/usr/projects/mixed-signals/steering/INTENTIONS.md`
3. `/usr/projects/tui-vfx/steering/INTENTIONS.md`
4. `/usr/projects/gt-design/.omx/context/v3-managed-briefing-20260423T170500Z.md`
5. `/usr/projects/global_prompts/standards/40_ofpf_standards.md`
6. `/usr/projects/global_prompts/standards/50_tdd_protocol.md`
7. `/usr/projects/tui-vfx/steering/work-packets/COMMON_EXECUTION_RULES.md`

## Verification required
- focused validator/compile/runtime tests for the chosen producer/consumer seam
- focused mixed-signals tests only if the tranche actually extends the shared signal substrate
- one explicit statement that this tranche advances **typed field hints / producer-consumer runtime**, not only basic coordinate threading

## Task reminder
Your task is still: land one bounded typed field-hint runtime tranche, not complete the full field-hint architecture.
