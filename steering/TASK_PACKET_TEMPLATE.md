# Task Packet Template

Purpose: reusable template for bounded subagent packets after the briefing experiments.

Use this template when dispatching a real subagent lane. Fill in the bracketed fields and delete any sections that truly do not apply.

---

# [Packet name]

## Task first
[State the assignment in one sentence. Keep it concrete and bounded.]

## Why this matters
[Explain why this specific lane is worth doing now and what larger goal it supports.]

## Success condition
By the end of this packet:
- [specific outcome 1]
- [specific outcome 2]
- [specific outcome 3]

## Mode
[BLOCKER_MODE or FAMILY_MODE]

## Task-scope paths for grounding
These are the files/areas you should use to understand the problem before you decide what the write scope is:
- `[full path]`
- `[full path]`
- `[full path]`

## Exact write scope
Only edit these paths unless the packet explicitly authorizes a nearby extension:
- `[full path]`
- `[full path]`
- `[full path]`

## Explicit out of scope
Do not widen into:
- `[out-of-scope item]`
- `[out-of-scope item]`
- `[out-of-scope item]`

## Must-read docs in order
1. `/usr/projects/tui-vfx/steering/INTENTIONS.md`
2. `/usr/projects/tui-vfx-recipes/steering/INTENTIONS.md`
3. `/usr/projects/mixed-signals/steering/INTENTIONS.md` when relevant
4. the active shared briefing in `/usr/projects/gt-design/.omx/context/`
5. `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-recipe-vocabulary.md`
6. `/usr/projects/global_prompts/standards/40_ofpf_standards.md`
7. `/usr/projects/global_prompts/standards/50_tdd_protocol.md`
8. `/usr/projects/global_prompts/standards/60_file_centric_execution.md`
9. `/usr/projects/global_prompts/standards/65_subagent_orchestration.md`
10. `[packet-specific extra doc if needed]`

## Repo-boundary guardrails
- `mixed-signals` owns reusable signal/math substrate only.
- `tui-vfx` owns renderer/effect semantics.
- `tui-vfx-recipes` owns recipe truth, tooling, validator/probe/preview, compiled seams, and generated V3 docs.
- [any lane-specific boundary note]

## First steps / grounding instructions
1. Run `ofpf-orientation` on the repos in scope.
2. Read the must-read docs in order.
3. Restate briefly:
   - what belongs in which repo
   - what counts as done
   - what the biggest scope risk is
4. Do the narrowest repo inspection needed before editing.

## Performance / hot-path reminders
- [lane-specific hot-path risk]
- [lane-specific hot-path risk]
- If this is not a hot-path-sensitive lane, say so explicitly.

## Verification required
Run exactly:
- `[shell-ready command]`
- `[shell-ready command]`
- `[shell-ready command]`

If a command fails, classify it as:
- in-scope failure
- expected downstream fallout
- or blocker

## Reporting contract
Your final report must include:
- docs read confirmation
- 3 reflection bullets
- exact task-scope paths used for grounding
- exact changed files (full paths)
- exact commands run
- pass/fail outcome per command
- blocker or handoff notes
- performance risks noticed

## Closing task reminder
Your task is still: [repeat the one-sentence assignment here].
Do not widen into [specific common drift areas].
