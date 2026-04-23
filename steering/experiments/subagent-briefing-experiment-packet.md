# Experimental Subagent Packet

## Objective
Identify the next blocker-scoped V3 tooling/validator task inside `/usr/projects/tui-vfx-recipes` and report the exact write scope, verification commands, and risks without broadening into the whole V3 migration.

## Success condition
Return one narrow recommended blocker lane with:
- exact files to touch
- exact verification commands
- notable risks
- no scope widening into unrelated V3 migration work

## Mode
BLOCKER_MODE

## In scope
- blocker-scoped V3 tooling/validator work in `/usr/projects/tui-vfx-recipes`
- read-only analysis unless the packet explicitly says otherwise
- exact file/path identification
- exact verification-command identification

## Out of scope
- broad V3 migration planning
- family-wide normalization passes
- `mixed-signals` extraction
- recipe/debug recipe authoring work unless directly named
- runtime behavior changes

## Must-read docs
1. `/usr/projects/tui-vfx/steering/INTENTIONS.md`
2. `/usr/projects/tui-vfx-recipes/steering/INTENTIONS.md`
3. `/usr/projects/mixed-signals/steering/INTENTIONS.md` when relevant
4. `/usr/projects/gt-design/.omx/context/v3-managed-briefing-20260423T170500Z.md`
5. `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-recipe-vocabulary.md`
6. `/usr/projects/global_prompts/standards/40_ofpf_standards.md`
7. `/usr/projects/global_prompts/standards/50_tdd_protocol.md`
8. `/usr/projects/global_prompts/standards/60_file_centric_execution.md`
9. `/usr/projects/global_prompts/standards/65_subagent_orchestration.md`

## Repo boundary guardrails
- `mixed-signals` owns reusable signal/math substrate only.
- `tui-vfx` owns renderer/effect semantics.
- `tui-vfx-recipes` owns recipe authoring truth, validator/tooling, compiled seams, preview/validator bridges, and generated V3 schema/docs surfaces.

## Performance reminder
If the candidate blocker touches a hot path, call out the hot-path risk explicitly instead of giving only generic performance advice.

## Verification/reporting
Your report must include:
- one-sentence assignment summary
- exact files/paths in scope
- exact out-of-scope items
- exact verification commands
- the biggest likely mistake if rushed

## Task reminder
Your task is still: identify one blocker-scoped V3 tooling/validator lane in `/usr/projects/tui-vfx-recipes` without broadening into the whole V3 migration.
