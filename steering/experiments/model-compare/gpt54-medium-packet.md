# Experimental Subagent Packet

## Task first
Find one blocker-scoped V3 tooling/validator lane in `/usr/projects/tui-vfx-recipes`.
Stay narrow. Do not widen into the whole V3 migration.

## Objective
Identify the next blocker-scoped V3 tooling/validator task inside `/usr/projects/tui-vfx-recipes` and report the exact write scope, verification commands, and risks.

## Success condition
Return exactly one recommended blocker lane with:
- exact files to touch
- exact verification commands
- notable risks
- no scope widening into unrelated V3 migration work
- no alternative lane dump
- exact path strings, not just repo names or concepts
- if a concrete lane can be named, include the smallest likely source/test file set; if not, say why not

## Mode
BLOCKER_MODE

## In scope
- blocker-scoped V3 tooling/validator work in `/usr/projects/tui-vfx-recipes`
- read-only analysis unless the packet explicitly says otherwise
- exact file/path identification
- exact verification-command identification
- repo-boundary judgment using the must-read docs and orientation snapshots

## Out of scope
- broad V3 migration planning
- family-wide normalization passes
- `mixed-signals` extraction
- recipe/debug recipe authoring work unless directly named
- runtime behavior changes
- edits to `ORCHESTRATION.md` or the shared briefing unless the 10-cycle evidence is already strong and the experiment is complete
- implementation work of any kind

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
- If a candidate task is still a validator/tooling boundary issue, keep it in `tui-vfx-recipes` unless the docs clearly demand lower signal/math substrate.

## Performance reminder
If the candidate blocker touches a hot path, call out the hot-path risk explicitly instead of giving only generic performance advice.

## Verification/reporting
Your report must include:
- one-sentence assignment summary
- exact files/paths in scope
- exact out-of-scope items
- exact verification commands
- the biggest likely mistake if rushed
- one blocker-lane recommendation only
- shell-ready command text for verification, not generic descriptions
- candidate source/test file paths if the docs support them
- if the docs support candidate tests, give the smallest runnable command you would actually use from the repo root

## Response discipline
- Use the docs and this packet as the source of truth.
- Answer the fixed questions directly.
- Use the adaptive questions to show what you learned from the docs, not generic opinions.
- Quote or paraphrase the actual instruction that supports each answer.
- Prefer concrete file/path language over conceptual summaries.
- If the packet does not justify a concrete path, say so explicitly instead of substituting a repo name.
- Do not invent file paths; only name them when the docs or repo evidence support them.
- Do not omit runnable verification text if a candidate test set is supported.

## Task reminder
Your task is still: identify one blocker-scoped V3 tooling/validator lane in `/usr/projects/tui-vfx-recipes` without broadening into the whole V3 migration.
