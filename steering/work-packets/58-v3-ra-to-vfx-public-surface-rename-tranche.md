# Packet 58 — V3 Ra→Vfx public-surface rename tranche

## Task first
Execute the next bounded Rust-side `Ra*` → `Vfx*` rename tranche on the real public wire-format surface without pretending the whole cutover can happen in one step.

## Why this matters
The rename inventory says the real rename event lives in the `tui-vfx-recipes` public Rust type surface. Until bounded rename tranches land there, V3 still carries stale naming on its most important public schema-bearing APIs.

## Success condition
- one bounded public-surface rename tranche lands
- compatibility/migration notes are explicit where needed
- generated/doc surfaces for the touched types are updated
- no broad cross-repo rename wave in one pass

## Mode
BLOCKER_MODE

## Task-scope paths for grounding
- `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-ra-to-vfx-rename-inventory.md`
- `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-schema-overview.md`
- `/usr/projects/tui-vfx-recipes/src/recipe_schema/`
- `/usr/projects/tui-vfx-recipes/src/v3/`
- the exact public type/module cluster chosen for this rename tranche

## Exact write scope
- only the chosen public type/module rename tranche
- the smallest supporting doc/rustdoc/generator surfaces needed for the rename to be coherent
- the narrowest tests proving the renamed surface still behaves correctly

## Out of scope
- whole-repo mass rename
- archive/doc-history rewrites
- unrelated feature changes hiding inside the rename

## Must-read docs in order
1. `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-ra-to-vfx-rename-inventory.md`
2. `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-schema-overview.md`
3. `/usr/projects/tui-vfx/steering/INTENTIONS.md`
4. `/usr/projects/tui-vfx-recipes/steering/INTENTIONS.md`
5. `/usr/projects/gt-design/.omx/context/v3-managed-briefing-20260423T170500Z.md`
6. `/usr/projects/global_prompts/standards/40_ofpf_standards.md`
7. `/usr/projects/global_prompts/standards/50_tdd_protocol.md`

## Verification required
- the exact tests/build/doc-generator checks for the renamed public surface
- explicit note on compatibility/cutover behavior for downstream callers

## Task reminder
Your task is still: land one bounded public-surface rename tranche, not rename every `Ra*` surface in one packet.
