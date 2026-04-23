# Subagent Briefing Experiment Protocol

Purpose: iteratively improve the real subagent briefing/task-packet system by editing an on-disk experimental packet, testing it with fresh medium helper agents, and scoring what those helpers actually understood from the docs and packet.

## Why this experiment exists

Previous prompt research drifted because the packet variation lived only in transient wrapper prompts. That tested prompt phrasing, not the real briefing system. This experiment fixes that by requiring every cycle to modify a real on-disk artifact before a fresh helper reads it.

## What is being tested

The experimental artifact is:
- `/usr/projects/tui-vfx/steering/experiments/model-compare/gpt54mini-medium-packet.md`

That file is the source-of-truth packet for each cycle. Each cycle must revise the file itself, then a fresh helper must read that revised file.

## Governing principles

- Test the real artifact, not just wrapper prompt wording.
- Use a NEW helper every cycle.
- Close the helper immediately after feedback is collected.
- Require the helper to do all grounding work before answering any evaluation questions.
- Keep the evaluation questions mostly stable so cycle-to-cycle results are comparable.
- Add adaptive questions based on previously observed failure modes.
- Record actual comprehension, not just opinion.
- Do not contaminate later cycles with helper memory; each helper should be a first-read reaction.

## Read order for the research agent

Before starting cycle work, read these files in order:
1. `/usr/projects/tui-vfx/steering/INTENTIONS.md`
2. `/usr/projects/tui-vfx-recipes/steering/INTENTIONS.md`
3. `/usr/projects/mixed-signals/steering/INTENTIONS.md`
4. `/usr/projects/gt-design/.omx/context/v3-managed-briefing-20260423T170500Z.md`
5. `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-recipe-vocabulary.md`
6. `/usr/projects/global_prompts/standards/40_ofpf_standards.md`
7. `/usr/projects/global_prompts/standards/50_tdd_protocol.md`
8. `/usr/projects/global_prompts/standards/60_file_centric_execution.md`
9. `/usr/projects/global_prompts/standards/65_subagent_orchestration.md`

Then run orientation snapshots:
- `ofpf-orientation --root /usr/projects/tui-vfx`
- `ofpf-orientation --root /usr/projects/tui-vfx-recipes`
- `ofpf-orientation --root /usr/projects/mixed-signals`
- `ofpf-orientation --root /usr/projects/gt-design`

## Write scope

Allowed during the experiment:
- `/usr/projects/tui-vfx/steering/experiments/model-compare/gpt54mini-medium-packet.md`
- `/usr/projects/tui-vfx/steering/experiments/model-compare/gpt54mini-medium-results.md`

Allowed only after all 10 cycles are complete and a recommendation has converged:
- `/usr/projects/tui-vfx/steering/ORCHESTRATION.md`
- `/usr/projects/gt-design/.omx/context/v3-managed-briefing-20260423T170500Z.md`

Out of scope:
- product/runtime/library code
- recipe content
- docs unrelated to briefing/orchestration
- any repo-wide cleanup

## Controlled task scenario

The experimental packet should keep testing the same representative task family:
- a bounded blocker-scoped V3 tooling/validator lane inside `/usr/projects/tui-vfx-recipes`
- not broad migration work
- not family-wide cleanup
- not mixed-signals extraction

Keep the scenario stable enough that changes in helper understanding mostly reflect packet quality, not task drift.

## Required cycle structure

A cycle only counts if all of the following happen:
1. Edit the on-disk experimental packet file.
2. Spawn a NEW medium helper agent.
3. Give the helper the packet file path and require it to read the packet plus the referenced docs.
4. Require the helper to perform all grounding work for the packet but not start any implementation/work task and not answer the comprehension questions yet.
5. Require the helper to stop after grounding and report that it is READY for questions.
6. Ask the fixed 7 comprehension questions and the adaptive 3 questions only after the helper has completed grounding and declared readiness.
7. Collect the helper response to the 10 questions.
8. Close that helper.
9. Score the response.
10. Update the results log.
11. Revise the packet file for the next cycle.

If any step is missing, do not count the cycle.

## Helper constraints

Each helper is a first-read interpretation checker only.

Helper should not:
- edit repo files
- implement code
- broaden scope
- perform unrelated research
- answer the 10 evaluation questions before grounding is complete

Helper should:
- read the packet and cited docs carefully
- perform all grounding work first
- stop and say it is ready before the researcher asks the 10 questions
- answer the comprehension questions concretely only after the researcher asks them
- identify likely misunderstandings if rushed
- quote or paraphrase the actual instructions it relied on

## The 7 fixed doc-grounded comprehension questions

These stay constant across cycles.

1. From the docs and packet, what is the assignment in one sentence?
2. What exact files/paths are in scope for this task?
3. What exact things are out of scope?
4. Which repo owns the main concern, and why?
5. Is this blocker-scoped or family-scoped work, and why?
6. What exact verification is required before reporting completion?
7. What is the most likely mistake you would make if you rushed this task?

Required answer format for each fixed question:
- Answer
- Source file(s)
- Evidence phrase or rule
- Implication for the task

## The adaptive 3 questions

These must change based on what previous helpers got wrong.

Examples:
- Should this move into `mixed-signals`, and why or why not?
- Are debug recipes part of this task?
- Should you read `ORCHESTRATION.md` directly?
- Is this a docs-only, code-only, or mixed lane?
- Are generated docs or handwritten docs the source of truth here?
- Should you widen into the whole V3 migration?

Rule:
- the adaptive 3 must target the actual misunderstandings from the previous cycle(s)
- document why each adaptive question was chosen in the results log

## Scoring rubric

Score each fixed and adaptive answer as:
- 2 = correct and well supported
- 1 = partially correct / incomplete / weakly supported
- 0 = incorrect / missing / contradicted by docs

Per cycle record:
- fixed score total
- adaptive score total
- major misunderstanding(s)
- strongest improvement(s)
- what changed in the packet this round

## What to vary across cycles

Allowed packet changes:
- ordering of sections
- task-first vs context-first emphasis
- repeated task reminder at end
- sharper in-scope / out-of-scope bullets
- explicit repo-boundary bullets
- explicit blocker/family mode labeling
- stronger verification checklist
- stronger reporting contract
- lane-local performance watchpoints
- stronger reflection/preflight requirements

Do not change too many variables at once if it prevents learning.

## Recommended packet design hypothesis to test

Current strongest hypothesis:
1. Task first
2. Bounds immediately after
3. Guardrail briefing next
4. Verification/reporting next
5. Repeated task summary at the end

The experiment should test whether that improves comprehension and scope discipline.

## Results log requirements

Write results to:
- `/usr/projects/tui-vfx/steering/experiments/model-compare/gpt54mini-medium-results.md`

For each cycle record:
- cycle number
- helper id
- packet revision summary
- whether the helper completed grounding before the questions
- the exact 7 fixed and 3 adaptive questions used
- fixed-question score
- adaptive-question score
- what the helper got wrong
- what improved vs previous cycle
- what you changed next
- a full feedback section capturing the helper's grounding response and the full answer set, or a faithful verbatim transcript/appendix reference

## Final convergence criteria

After 10 valid cycles, recommend changes only if the packet behavior has clearly improved.

Convergence signs:
- repo-boundary answers stabilize
- in-scope/out-of-scope recall stabilizes
- blocker-vs-family understanding stabilizes
- verification answers become precise
- repeated prior failure modes materially decline

## Final deliverable

At the end of the 10 cycles, provide:
- exact cycle count
- exact helper ids used
- per-cycle log summary
- enough full helper feedback in the results log that a human can audit what each helper actually said
- strongest successful packet changes
- recurring failure modes that remained
- recommended permanent changes to:
  - `/usr/projects/tui-vfx/steering/ORCHESTRATION.md`
  - `/usr/projects/gt-design/.omx/context/v3-managed-briefing-20260423T170500Z.md`

If the evidence is strong, then patch those two files after the experiment.
