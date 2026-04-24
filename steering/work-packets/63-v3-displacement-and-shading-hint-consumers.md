# Packet 63 — V3 displacement and shading hint consumers

## Task first
Build the next typed field-hint runtime slice by adding a real displacement or shading consumer that uses upstream hint output, not just the first dim-filter proof seam.

## Why this matters
Packet 55 established the first operational producer/consumer proof on the direct V3 path, but the original V3 motivation — especially Madeira flag fidelity — depends on richer consumers such as displacement and wave-correlated shading. The next tranche should extend the runtime beyond the minimal dim-filter demonstration.

## Success condition
- one richer consumer lane lands on the typed hint runtime seam
- the consumer reads upstream hint output operationally
- the slice is proven with focused render/validator/runtime tests

## Mode
BLOCKER_MODE

## Task-scope paths for grounding
- `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-spatial-field-hint-plan.md`
- `/usr/projects/tui-vfx-recipes/src/v3/compile/`
- `/usr/projects/tui-vfx-recipes/src/v3/validate/`
- `/usr/projects/tui-vfx-recipes/src/v3/compile/test_render_compiled_plan_deterministically.rs`
- `/usr/projects/tui-vfx-recipes/src/v3/compile/test_build_composition_spec_from_compiled_plan.rs`
- `/usr/projects/tui-vfx-recipes/recipes/madeira_flag/madeira_flag.json` if the chosen slice touches the Madeira motivation directly

## Exact write scope
- the exact runtime/validator files needed for the chosen displacement or shading consumer slice
- the narrowest supporting tests/proofs required

## Out of scope
- the full typed field architecture in one packet
- whole-Madeira parity completion
- unrelated naming/doc cleanup

## Must-read docs in order
1. `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-spatial-field-hint-plan.md`
2. `/usr/projects/tui-vfx/steering/work-packets/55-v3-spatial-leaves-and-field-hint-threading.md`
3. `/usr/projects/mixed-signals/steering/INTENTIONS.md`
4. `/usr/projects/tui-vfx/steering/INTENTIONS.md`
5. `/usr/projects/gt-design/.omx/context/v3-managed-briefing-20260423T170500Z.md`
6. `/usr/projects/global_prompts/standards/40_ofpf_standards.md`
7. `/usr/projects/global_prompts/standards/50_tdd_protocol.md`
8. `/usr/projects/tui-vfx/steering/work-packets/COMMON_EXECUTION_RULES.md`

## Verification required
- focused runtime tests proving the richer consumer reads upstream hint output
- one explicit statement of whether the slice advanced displacement, shading, or both
- if the slice touches Madeira, at least one Madeira-specific proof command

## Task reminder
Your task is still: land one richer typed-hint consumer slice, not finish the full field-hint architecture in one jump.
