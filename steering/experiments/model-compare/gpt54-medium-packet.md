# Experimental Subagent Packet

## Packet revision
- Experiment lane: GPT-5.4 medium model-pure helper briefing
- Active cycle: 2
- Revision focus: force visible grounding and separate task scope from proposed write scope

## Task first
Find one blocker-scoped V3 tooling/validator lane in `/usr/projects/tui-vfx-recipes`.
Stay narrow. Do not widen into the whole V3 migration.
This is a read-only briefing-comprehension exercise, not implementation.
You must perform the full grounding work before you answer.

## Objective
Identify the next blocker-scoped V3 tooling/validator task inside `/usr/projects/tui-vfx-recipes` and report:
- the exact task-scope paths you grounded on
- the exact recommended write scope for one blocker lane
- exact verification commands
- notable risks
- the smallest supported source/test path set if the docs and repo evidence justify one

## Success condition
Return exactly one recommended blocker lane with:
- exact task-scope paths
- exact files to touch for the recommendation
- exact verification commands
- notable risks
- no scope widening into unrelated V3 migration work
- no alternative lane dump
- exact path strings, not just repo names or concepts
- if a concrete lane can be named, include the smallest likely source/test file set; if not, say why not

## Mode
BLOCKER_MODE
READ_ONLY_ANALYSIS

## In scope
- blocker-scoped V3 tooling/validator work in `/usr/projects/tui-vfx-recipes`
- read-only analysis only
- exact file/path identification
- exact verification-command identification
- repo-boundary judgment using the must-read docs and orientation snapshots
- one blocker recommendation only
- showing the grounding work you performed before answering

## Out of scope
- broad V3 migration planning
- family-wide normalization passes
- `mixed-signals` extraction
- recipe/debug recipe authoring work unless directly named
- runtime behavior changes
- edits to `ORCHESTRATION.md` or the shared briefing unless the 10-cycle evidence is already strong and the experiment is complete
- implementation work of any kind
- proposing multiple candidate lanes instead of choosing one

## Must-read docs in this exact order
1. `/usr/projects/tui-vfx/steering/INTENTIONS.md`
2. `/usr/projects/tui-vfx-recipes/steering/INTENTIONS.md`
3. `/usr/projects/mixed-signals/steering/INTENTIONS.md`
4. `/usr/projects/gt-design/.omx/context/v3-managed-briefing-20260423T170500Z.md`
5. `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-recipe-vocabulary.md`
6. `/usr/projects/global_prompts/standards/40_ofpf_standards.md`
7. `/usr/projects/global_prompts/standards/50_tdd_protocol.md`
8. `/usr/projects/global_prompts/standards/60_file_centric_execution.md`
9. `/usr/projects/global_prompts/standards/65_subagent_orchestration.md`

Then use the already-provided orientation snapshots for:
- `/usr/projects/tui-vfx`
- `/usr/projects/tui-vfx-recipes`
- `/usr/projects/mixed-signals`
- `/usr/projects/gt-design`

Then do only the narrow repo inspection needed to justify one blocker recommendation.

## Repo boundary guardrails
- `mixed-signals` owns reusable signal/math substrate only.
- `tui-vfx` owns renderer/effect semantics.
- `tui-vfx-recipes` owns recipe authoring truth, validator/tooling, compiled seams, preview/validator bridges, and generated V3 schema/docs surfaces.
- If a candidate task is still a validator/tooling boundary issue, keep it in `tui-vfx-recipes` unless the docs clearly demand lower signal/math substrate.

## Performance reminder
If the candidate blocker touches a hot path, call out the hot-path risk explicitly instead of giving only generic performance advice.

## Grounding proof block (required before the questions)
Start your response with these exact headings:
- `Docs read in order:`
- `Orientation snapshots consulted:`
- `Additional repo inspection performed:`
- `Why the candidate paths are justified:`

Rules for this block:
- List the docs in the order you actually read them.
- List the orientation snapshots you relied on.
- List only the additional repo reads/searches you actually performed.
- For every proposed write path, name the evidence that justified that path.
- Do not answer the questions until this grounding block is complete.
- After the grounding block, stop and say `READY FOR QUESTIONS`.

## Verification/reporting requirements
Your report must include:
- one-sentence assignment summary
- exact task-scope paths you grounded on
- exact recommended write-scope paths
- exact out-of-scope items
- exact verification commands
- the biggest likely mistake if rushed
- one blocker-lane recommendation only
- shell-ready command text for verification, not generic descriptions
- candidate source/test file paths if the docs support them
- if the docs support candidate tests, give the smallest runnable command you would actually use from the repo root

## Grounding-only stop point
Your first response should end after these headings only:
- `Docs read in order:`
- `Orientation snapshots consulted:`
- `Additional repo inspection performed:`
- `Why the candidate paths are justified:`
- `READY FOR QUESTIONS`

Do not answer the fixed or adaptive questions in the first response.

## Response discipline
- Use the docs and this packet as the source of truth.
- Quote or paraphrase the actual instruction that supports each answer.
- Prefer concrete file/path language over conceptual summaries.
- If the packet does not justify a concrete path, say so explicitly instead of substituting a repo name.
- Do not invent file paths; only name them when the docs or repo evidence support them.
- Do not omit runnable verification text if a candidate test set is supported.
- Keep the final recommendation narrow and blocker-scoped.
- Separate task-scope paths from recommended write-scope paths.

## Fixed questions (answer all 7)
For each answer, use this exact mini-format:
- Answer:
- Source file(s):
- Evidence phrase or rule:
- Implication for the task:

1. From the docs and packet, what is the assignment in one sentence?
2. What exact task-scope files/paths did you ground on for this task?
3. What exact things are out of scope?
4. Which repo owns the main concern, and why?
5. Is this blocker-scoped or family-scoped work, and why?
6. What exact verification is required before reporting completion?
7. What is the most likely mistake you would make if you rushed this task?

## Adaptive questions for cycle 2
Use the same 4-line mini-format for each answer.

A1. What is the difference between `task-scope paths` and `recommended write-scope paths` in this packet?
A2. Are you allowed to name a write path just because it sounds plausible from the briefing, or what evidence threshold must you meet first?
A3. If the briefing mentions files in `tui-vfx` as current critical files, should you automatically include them in the write scope for this task? Why or why not?

## Final recommendation block
After the grounding block and the 10 question answers, end with exactly these headings:
- `Recommended blocker lane:`
- `Task-scope paths grounded on:`
- `Smallest supported write scope:`
- `Smallest supported verification commands:`
- `Hot-path or scope risks:`

## Task reminder
Your task is still: identify one blocker-scoped V3 tooling/validator lane in `/usr/projects/tui-vfx-recipes` without broadening into the whole V3 migration.
