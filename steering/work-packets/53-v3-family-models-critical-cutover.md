# Packet 53 — V3 family-models critical cutover

## Task first
Land the next bounded family-model cutover so V3 shader/filter/style/effect semantics rely less on legacy flat-model assumptions.

## Why this matters
The current direct/native V3 path is real, but the broader family-model migration is still incomplete. Chapter 100 still lists `tui-vfx-style/src/models/` restructuring per Decision 2 as release-blocking, which means large parts of the shader/style/effect surface are conceptually migrated but not yet fully reflected in the owning code/doc model.

## Success condition
- one clearly bounded family-model tranche is selected and completed
- the tranche moves a representative cluster toward the V3 primitive / Tier-1-factory model
- capability/docs surfaces for the touched families are updated
- no broad whole-catalog rewrite in one pass

## Mode
FAMILY_MODE

## Task-scope paths for grounding
- `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-upgrade-plan/100_tooling_ci_migration.md`
- `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-upgrade-debug-recipes-migration-log.md`
- `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-lowering-map.md`
- `/usr/projects/tui-vfx/docs/CAPABILITIES_REFERENCE.md`
- `/usr/projects/tui-vfx/crates/tui-vfx-style/src/models/`
- `/usr/projects/tui-vfx/docs/templates/capabilities.toml`
- `/usr/projects/tui-vfx/xtask/src/docs/`

## Exact write scope
- the smallest representative family cluster in `crates/tui-vfx-style/src/models/`
- the narrowest adjacent capability/doc generator surfaces needed for the tranche
- the smallest tests/docs proving the tranche

## Out of scope
- the full family catalog in one packet
- broad corpus rewriting
- unrelated recipe/runtime migrations outside the selected family cluster

## Must-read docs in order
1. `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-upgrade-plan/100_tooling_ci_migration.md`
2. `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-upgrade-debug-recipes-migration-log.md`
3. `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-lowering-map.md`
4. `/usr/projects/tui-vfx/steering/INTENTIONS.md`
5. `/usr/projects/gt-design/.omx/context/v3-managed-briefing-20260423T170500Z.md`
6. `/usr/projects/global_prompts/standards/40_ofpf_standards.md`
7. `/usr/projects/global_prompts/standards/50_tdd_protocol.md`
8. `/usr/projects/tui-vfx/steering/work-packets/COMMON_EXECUTION_RULES.md`

## Recommended first steps
1. Name the exact family cluster before editing.
2. State whether each touched family is becoming a primitive, a Tier-1 factory, or a wrapper/router.
3. Identify the smallest tests and generated-doc surfaces that must move with it.
4. Keep the tranche small enough that review can still verify semantic ownership precisely.

## Verification required
- exact tests for the chosen family cluster
- `cargo xtask docs generate` or the smallest equivalent doc-generator checks for touched surfaces
- one explicit statement of how the tranche reduces legacy flat-model dependence

## Reporting format
Report:
- exact family cluster chosen
- primitive / Tier-1 / wrapper decisions taken
- exact files changed
- exact commands run
- remaining family-model gaps left untouched

## Task reminder
Your task is still: land one bounded family-model tranche, not finish the entire shader/filter/style/effect migration in one pass.
