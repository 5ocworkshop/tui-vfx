# Packet 01 — V3 schema/docs freshness

## Task first
Fix the V3 code-derived docs freshness gate inside `/usr/projects/tui-vfx-recipes` with the smallest justified docs/tooling change.

## Objective
Restore and keep code-derived V3 schema/docs freshness inside `/usr/projects/tui-vfx-recipes` so the generated V3 API/docs gate passes and remains trustworthy.

## Why this matters
The V3 code-derived schema/docs surface is part of the source-of-truth story for the migration. If generated docs drift from the real `src/v3` surface, downstream readers and tools lose trust in the documentation and the freshness gate stops being useful.

## Mode
BLOCKER_MODE

## Success condition
By the end of this packet:
- the V3 schema export tests pass
- the generated V3 docs are refreshed from code
- the freshness gate passes
- the report clearly states whether any source files needed edits or whether this was generation-only

## Task-scope paths for grounding
- `/usr/projects/tui-vfx-recipes/src/v3/authoring/cls_v3_recipe_document.rs`
- `/usr/projects/tui-vfx-recipes/src/v3/compile/cls_compiled_runtime_overrides.rs`
- `/usr/projects/tui-vfx-recipes/src/v3/compile/cls_v3_motion_envelope.rs`
- `/usr/projects/tui-vfx-recipes/src/v3/compile/cls_v3_playback_timing.rs`
- `/usr/projects/tui-vfx-recipes/src/v3/compile/cls_v3_source_surface.rs`
- `/usr/projects/tui-vfx-recipes/src/v3/compile/fnc_apply_compiled_pipeline_replay_to_scene.rs`
- `/usr/projects/tui-vfx-recipes/src/v3/compile/fnc_execute_compiled_step_tree_to_scene.rs`
- `/usr/projects/tui-vfx-recipes/src/v3/fnc_export_v3_schema.rs`
- `/usr/projects/tui-vfx-recipes/docs/generated/V3_API.md`
- `/usr/projects/tui-vfx-recipes/docs/generated/v3_api.json`
- one generator entrypoint if needed, but only if that is truly the root cause

## Exact write scope
- `/usr/projects/tui-vfx-recipes/docs/generated/V3_API.md`
- `/usr/projects/tui-vfx-recipes/docs/generated/v3_api.json`
- only the smallest source-of-truth file(s) above if regeneration alone does not fix the gate
- one generator entrypoint only if it is clearly the root cause

## Out of scope
- runtime behavior changes unrelated to docs/schema visibility
- validator stage logic
- debug recipe content
- changes in `/usr/projects/tui-vfx`
- changes in `/usr/projects/mixed-signals`

## Must-read docs in order
1. `/usr/projects/tui-vfx/steering/INTENTIONS.md`
2. `/usr/projects/tui-vfx-recipes/steering/INTENTIONS.md`
3. `/usr/projects/gt-design/.omx/context/v3-managed-briefing-20260423T170500Z.md`
4. `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-recipe-vocabulary.md`
5. `/usr/projects/global_prompts/standards/40_ofpf_standards.md`
6. `/usr/projects/global_prompts/standards/50_tdd_protocol.md`
7. `/usr/projects/global_prompts/standards/60_file_centric_execution.md`
8. `/usr/projects/global_prompts/standards/65_subagent_orchestration.md`

## Repo-boundary guardrails
- Keep this lane in `/usr/projects/tui-vfx-recipes`; this is docs/tooling freshness, not render semantics or shared math substrate.
- Do not drift into `/usr/projects/tui-vfx` or `/usr/projects/mixed-signals`.

## Must-read emphasis
Pay special attention to:
- `tui-vfx-recipes/steering/INTENTIONS.md` for repo ownership
- the shared briefing for blocker-scoped behavior
- `src/v3/fnc_export_v3_schema.rs` as the intended code-derived export seam

## First steps
1. Run OFPF orientation on `/usr/projects/tui-vfx-recipes`.
2. Confirm the current failing freshness command and capture the exact error.
3. Determine whether the failure is:
   - stale generated docs only
   - missing source exposure/re-export
   - generator drift
4. Make the smallest fix that restores freshness.

## OFPF guidance
Use:
- `ofpf-orientation --root /usr/projects/tui-vfx-recipes`
- targeted focus/inspect around `src/v3/` and generated docs
Do not broad-read the whole repo.

## Verification required
Run exactly:
- `cargo test -p tui-vfx-recipes fnc_export_v3_schema`
- `python3 tools/fnc_generate_v3_docs.py --write` (or exact generator if the path differs)
- `just docs-v3-check`

## Performance note
This is offline docs/tooling work. No runtime performance optimization is expected, but if a proposed fix accidentally widens into runtime code, stop and report.

## Reporting format
Report:
- whether the root cause was stale generation, source visibility, or generator drift
- exact changed files
- exact command results
- whether the gate is now green
- any reason this may regress again

## Task reminder
Your task is still: fix V3 code-derived docs freshness in `/usr/projects/tui-vfx-recipes` with the smallest blocker-scoped change, and do not widen into runtime or recipe work.
