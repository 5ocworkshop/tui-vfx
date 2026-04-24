# Packet 08 — Madeira next-slice plan

## Task first
Choose the single best next Madeira implementation slice from the audit results and define it concretely.

## Objective
Turn the Madeira parity audit into one narrow next implementation slice that can be executed safely.

## Why this matters
The audit should feed a concrete next step, not a vague promise of future parity.

## Current landed evidence snapshot
- Madeira is already operational on the supported V3 path:
  - `0370ce7` — `ballistic_fireworks` landed in the stock procedural lane
  - `8e11d71` — Madeira diagnostic/probe surfaces now describe support truthfully
  - `c1e8a0f` — Madeira preview-state and preview-area regression anchors are locked
  - `1025a75` — diagnostic example coverage for `PreviewItem` option handling landed
  - `3a235f7` — preview-area renders now skip no-op host replay setup
- The current `recipes/madeira_flag/madeira_flag.json` still documents the flag layer as the heart of the composition, but also records that the live recipe is a supported approximation (`sine_wave` + `gradient_overlay`) rather than the reference crate’s full physics-based waving flag.
- The reference repo at `/usr/projects/madeira-flag` still shows the flag as the main remaining fidelity seam: compound sine-wave motion, Braille sub-pixel rendering, and wave-correlated shading are the part of the original animation most worth tightening next.

## Mode
BLOCKER_MODE

## Success condition
- one next slice is chosen
- exact files to touch are named
- exact proof commands are named
- risks are explicit

## Task-scope paths for grounding
- Madeira audit results
- the exact seam identified by the audit

## Exact write scope
- none by default; this is a planning packet
- if you need to add one planning artifact or note, keep it narrowly scoped and state why

## Out of scope
- implementing the slice itself
- widening into multiple Madeira gaps at once

## Must-read docs in order
1. `/usr/projects/tui-vfx-recipes/steering/INTENTIONS.md`
2. relevant Madeira audit result(s)
3. `/usr/projects/gt-design/.omx/context/v3-managed-briefing-20260423T170500Z.md`
4. `/usr/projects/tui-vfx/steering/INTENTIONS.md`
5. `/usr/projects/global_prompts/standards/50_tdd_protocol.md`

## Repo-boundary guardrails
- This is slice planning, not execution.
- Choose one slice only; do not create a laundry list of loosely ranked tasks.

## Deliverable
Produce:
- slice name
- exact seam
- exact files
- why this slice is first
- what commands prove it when done

## Chosen next slice
**Slice name:** Madeira flag-layer fidelity refinement

**Exact seam:** the flag-layer approximation in `recipes/madeira_flag/madeira_flag.json`, specifically the `flag` layer pipeline and its current `sine_wave` + `gradient_overlay` live approximation.

**Exact files**
- `/usr/projects/tui-vfx-recipes/recipes/madeira_flag/madeira_flag.json`
- `/usr/projects/tui-vfx-recipes/src/v3/compile/test_render_compiled_plan_deterministically.rs`
- `/usr/projects/tui-vfx-recipes/src/preview/cls_direct_v3_preview_state.rs`

**Why this slice is first**
- The other Madeira subpieces now have basic operational coverage and baseline anchors.
- The flag layer is still the clearest semantic divergence from `/usr/projects/madeira-flag`.
- It is narrow enough to verify with existing Madeira regression surfaces before any broader V3 substrate work.

## Risks
- The fidelity gap may prove to depend on lower-level signal/time substrate work if the current sampler/shader approximation cannot be tightened enough inside the recipe layer alone.
- Any baseline hash movement needs to stay locked to the Madeira-specific regression tests above, not spread into unrelated preview fixtures.

## Exact proof commands
- `cd /usr/projects/tui-vfx-recipes && cargo test -p tui-vfx-recipes load_v3_document_reads_madeira_flag_recipe load_v3_compiled_loads_madeira_flag_recipe direct_v3_preview_state_supports_madeira_flag_recipe preview_from_recipe_path_with_cutover_fallback_prefers_direct_v3_for_madeira_flag madeira_flag_recipe_renders_for_preview_area -- --nocapture`
- `cd /usr/projects/madeira-flag && cargo test --lib`
- `cd /usr/projects/tui-vfx-recipes && cargo test -p tui-vfx-recipes --lib madeira_flag_recipe_renders_for_preview_area direct_v3_preview_state_supports_madeira_flag_recipe -- --nocapture`

## Reporting format
Be concrete. If more than one slice seems possible, explain why the chosen one is best first.

## Task reminder
Your task is still: pick the next slice, not perform Madeira implementation.
