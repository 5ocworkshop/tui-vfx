# Mini helper experiment packet — cycle 6/10

## Task
Identify exactly one blocker-scoped V3 tooling/validator lane in `/usr/projects/tui-vfx-recipes`.
Stay narrow. Do **not** widen into the whole V3 migration.

## Current blocker lane under test
The active blocker is the direct/native V3 timing seam:
- cadence-driven effects must use monotonic elapsed time instead of reset-on-loop normalized time
- KITT scanner must keep `bpm` support cleanly
- the direct preview/validator bridge must prove the behavior
- broader math/vocabulary/time normalization stays blocked until this timing baseline is green

## Source-of-truth reading order
Read these in order before answering:
1. `/usr/projects/tui-vfx/steering/INTENTIONS.md`
2. `/usr/projects/tui-vfx-recipes/steering/INTENTIONS.md`
3. `/usr/projects/mixed-signals/steering/INTENTIONS.md`
4. `/usr/projects/gt-design/.omx/context/v3-managed-briefing-20260423T170500Z.md`
5. `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-recipe-vocabulary.md`
6. `/usr/projects/global_prompts/standards/40_ofpf_standards.md`
7. `/usr/projects/global_prompts/standards/50_tdd_protocol.md`
8. `/usr/projects/global_prompts/standards/60_file_centric_execution.md`
9. `/usr/projects/global_prompts/standards/65_subagent_orchestration.md`

## Scope rules
In scope:
- one blocker-scoped V3 tooling/validator lane in `/usr/projects/tui-vfx-recipes`
- the timing seam, preview seam, loader seam, and validator bridge that prove it
- exact file/path identification from the current blocker lane
- exact verification-command identification from the current blocker lane
- read-only analysis unless the packet says otherwise

Out of scope:
- broad V3 migration planning
- family-wide normalization passes
- `mixed-signals` extraction unless a reusable substrate truly emerges
- recipe/debug-recipe authoring work unless directly named
- runtime behavior changes outside this seam
- edits to `ORCHESTRATION.md` or the shared briefing until after 10 valid cycles and only if the experiment recommendation supports it
- any file outside the packet's write scope
- invented paths or invented commands that are not named in this packet or the source docs

## Explicit files currently under test
- `/usr/projects/tui-vfx-recipes/src/v3/compile/cls_v3_playback_timing.rs`
- `/usr/projects/tui-vfx-recipes/src/preview/cls_direct_v3_preview_state.rs`
- `/usr/projects/tui-vfx-recipes/src/v3/fnc_load_v3_document.rs`
- `/usr/projects/tui-vfx-recipes/src/v3/compile/test_build_composition_spec_from_compiled_plan.rs`
- `/usr/projects/tui-vfx-recipes/src/v3/compile/test_render_compiled_plan_deterministically.rs`
- `/usr/projects/tui-vfx-recipes/tools/pipeline-validator/tests/test_v3_compiled_bridge.rs`
- `/usr/projects/tui-vfx-recipes/tools/pipeline-validator/tests/test_v3_probe_mode.rs`
- `/usr/projects/tui-vfx-recipes/recipes/debug_recipes/filters/filter_kitt_scanner.json`
- `/usr/projects/tui-vfx-recipes/recipes/debug_recipes/filters/filter_kitt_scanner_progress_binding.json`
- `/usr/projects/tui-vfx-recipes/recipes/debug_recipes/complex/complex_filter_kitt_native_only.json`
- `/usr/projects/tui-vfx-recipes/recipes/madeira_flag/madeira_flag.json`

## Fixture boundary
The debug recipes listed above are proof artifacts for this blocker lane, not recipe-authoring work.

## Exact verification commands for this lane
Use these exact commands before reporting completion:
- `cargo test -p tui-vfx-recipes sampled_timing_keeps_absolute_time_monotonic_across_loop_wraps`
- `cargo test -p tui-vfx-recipes load_v3_document_preserves_kitt_scanner_bpm_authoring`
- `cargo test -p tui-vfx-recipes direct_v3_preview_state_advances_with_elapsed_time`
- `cargo test -p tui-vfx-recipes direct_v3_preview_state_supports_madeira_flag_recipe`
- `cargo test -p tui-vfx-recipes compiled_pipeline_replay_keeps_absolute_time_separate_from_normalized_fields`
- `cargo test -p tui-vfx-recipes kitt_native_filter_uses_absolute_time_even_when_normalized_fields_match`
- `cargo test -p pipeline-validator test_v3_compiled_bridge`
- `cargo test -p pipeline-validator test_v3_probe_mode`
- `just check`
- `just docs-v3-check`

## Verification fallback rule
If one of the exact verification commands fails, report that exact failure rather than substituting a broader command.

## Repo boundary guardrails
- `mixed-signals` owns reusable signal/math substrate only.
- `tui-vfx` owns renderer/effect semantics.
- `tui-vfx-recipes` owns recipe authoring truth, validator/tooling, compiled seams, preview/validator bridges, and generated V3 schema/docs surfaces.

## What to return
Return exactly one recommended blocker lane with:
- exact files to touch
- exact verification commands
- notable risks
- no scope widening into unrelated V3 migration work

## Response format for each question
For each answer include:
- Answer
- Source file(s)
- Evidence phrase or rule
- Implication for the task

## Fixed questions
1. From the docs and packet, what is the assignment in one sentence?
2. What exact files/paths are in scope for this task?
3. What exact things are out of scope?
4. Which repo owns the main concern, and why?
5. Is this blocker-scoped or family-scoped work, and why?
6. What exact verification is required before reporting completion?
7. What is the most likely mistake you would make if you rushed this task?

## Adaptive questions for cycle 6
8. Are the debug recipes in this packet proof artifacts only, or do they make recipe authoring work part of this task?
9. If one exact verification command fails, should you replace it with a broader command or report that exact failure?
10. Should you read `ORCHESTRATION.md` directly, or stay with the packet, briefing, and named standards?

## Reminders
- Use the docs and this packet as the source of truth.
- Quote or paraphrase the actual instruction that supports each answer.
- Do not edit repo files.
- Keep the answer narrowly focused on one blocker-scoped lane.
