# Task Packet Template

Purpose: reusable template for bounded subagent packets and follow-on tranches.

Use this template when dispatching a real subagent lane. Fill in the bracketed fields and delete any sections that truly do not apply. If the lane is not concrete enough to name exact paths or commands, say so instead of guessing.

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
[BLOCKER_MODE by default; FAMILY_MODE only when the packet is explicitly family-wide]

## Agent lane / model constraints
[If the owner requested an unroled model lane, state it here. Example:
`Use unroled gpt-5.5 low. Do not assign a specialist role because that may
change the model/effort profile.` Delete this section when not relevant.]

## Task-scope paths for grounding
These are the exact files/areas you should use to understand the problem before you decide what the write scope is:
- `[full path]`
- `[full path]`
- `[full path]`

## Exact write scope
Only edit these paths unless the packet explicitly authorizes a nearby extension.
If the packet cannot justify exact paths yet, say that the write scope is not concrete enough instead of inventing paths:
- `[full path]`
- `[full path]`
- `[full path]`

## Explicit out of scope
Do not widen into:
- `steering/ORCHESTRATION.md` or any leader-only orchestration policy; subagents should not read or use it
- `[out-of-scope item]`
- `[out-of-scope item]`
- `[out-of-scope item]`
- whole V3 migration
- unrelated orchestration policy or experiment-protocol redesign
- runtime/library code unless this packet explicitly owns it

## Must-read docs in order
1. `/usr/projects/tui-vfx/steering/INTENTIONS.md`
2. `/usr/projects/tui-vfx-recipes/steering/INTENTIONS.md`
3. `/usr/projects/mixed-signals/steering/INTENTIONS.md` when relevant
4. the active shared briefing file in `/usr/projects/gt-design/.omx/context/` (use the exact file path named by the packet)
5. `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-recipe-vocabulary.md`
6. `/usr/projects/global_prompts/standards/40_ofpf_standards.md`
7. `/usr/projects/global_prompts/standards/50_tdd_protocol.md`
8. `/usr/projects/global_prompts/standards/60_file_centric_execution.md`
9. `/usr/projects/global_prompts/standards/65_subagent_orchestration.md`
10. `/usr/projects/tui-vfx/steering/work-packets/COMMON_EXECUTION_RULES.md`
11. `[packet-specific extra doc if needed]`

## Repo-boundary guardrails
- `mixed-signals` owns reusable signal/math substrate only.
- `tui-vfx` owns renderer/effect semantics.
- `tui-vfx-recipes` owns recipe truth, tooling, validator/probe/preview, compiled seams, and generated V3 docs.
- Do not read or revise `ORCHESTRATION.md`; it is leader-only orchestration context and will confuse bounded worker lanes.
- Do not revise experiment protocols unless this packet explicitly owns that surface.
- [any lane-specific boundary note]

## Pipeline-touch definition of done
If this packet touches a shader, filter, mask, sampler, style, content effect,
motion route, shadow, scope, binding, or adjacent V3 pipeline file:
- move primitive reusable math/signal substrate to `mixed-signals` when it is
  renderer-agnostic and useful to 3+ real callers
- keep renderer/effect semantics in `tui-vfx` / `tui-vfx-recipes`
- normalize timing language and implementation (`elapsed`/absolute time for
  cadence, normalized phase/loop progress for phase)
- update rustdocs for public/schema-bearing items and generated-doc inputs when
  schema/API surfaces change
- align comments/docs/rustdocs/fixtures with canonical V3 vocabulary
- update primitive-first debug/reference recipes when visual semantics, timing,
  names, or parameters change
- prefer adding validator/probe/test coverage for any drift class discovered
- keep `<CLOG>` entries to the latest one- or two-line summary and update
  relevant `INDEX.md` files when docs move or become canonical

## Test-shape requirements
If this packet touches schema/parser behavior, include or point to one canonical
authored JSON fixture and require focused coverage for:
- accepted minimal form
- rejected unknown nested fields
- defaulted omitted fields
- validation boundary errors
- typed propagation into downstream IR/compiled structures

If this packet touches runtime behavior, name likely player/preview/probe
entrypoints and require focused coverage that those surfaces do not silently
drop the new feature. If reduced-motion, timing, or cache policy matters, require
the packet to identify the existing source of truth or report the missing seam
as a handoff.

## First steps / grounding instructions
1. Run `ofpf-orientation` on each repo in scope.
2. Read the must-read docs in order before broader file reads.
3. Restate briefly:
   - what belongs in which repo
   - what counts as done
   - what the biggest scope risk is
4. Do the narrowest repo inspection needed before editing.
5. If this is a `gpt-5.5` low lane, explicitly list the tricky semantics you
   will test before implementation (coordinate frame, timing, cache, schema
   defaults, or other packet-specific risks).
6. For schema/parser `gpt-5.5` low lanes, confirm the canonical JSON fixture
   shape before editing and name the default/boundary assertions you will add.

## Hot-path watchpoints
- [lane-specific hot-path risk]
- [lane-specific hot-path risk]
- If this is not a hot-path-sensitive lane, say so explicitly.

## Verification required
Run exactly. Only list commands the packet/docs actually justify:
- `[shell-ready command]`
- `[shell-ready command]`
- `[shell-ready command]`
Prefer documented validator/test commands over invented alternatives.

If a command fails, report that exact failure before considering any follow-up command, then classify it as:
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
- for `gpt-5.5` low lanes: OFPF line-pressure self-check, test-placement
  self-check, and any pre-existing dirty files noticed

## File metadata discipline
If you touch files with header metadata:
- keep `<CLOG>` / `// <CLOG>` entries to 1-2 short lines
- summarize only the latest change made to that file
- do not preserve or append a running history; git already provides history
- keep `<WCTX>` as stable work context unless the file's role changed

## Closing task reminder
Your task is still: [repeat the one-sentence assignment here].
Do not widen into [specific common drift areas].
