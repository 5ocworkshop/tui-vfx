# Experimental Subagent Packet

## Packet revision
- Experiment lane: GPT-5.4 medium model-pure helper briefing
- Active cycle: 10
- Revision focus: final freeze candidate for the converged output-stage/two-file briefing

## Task
Choose one blocker-scoped V3 tooling/validator lane in `/usr/projects/tui-vfx-recipes`.
Converged default lane:
- validator output-stage compiled-bridge scheduling/output-dump lane
Converged default smallest supported write scope:
- `/usr/projects/tui-vfx-recipes/tools/pipeline-validator/src/stages/functions/fnc_validate_output.rs`
- `/usr/projects/tui-vfx-recipes/tools/pipeline-validator/tests/test_fnc_validate_output.rs`
Read-only only. Stay narrow.

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

## Grounding block
Use exact headings, then stop with `READY FOR QUESTIONS`:
- `Docs read in order:`
- `Orientation snapshots consulted:`
- `Additional repo inspection performed:`
- `Why the candidate paths are justified:`

## Answer format
For every answer, use these four lines in order:
- `Answer: ...`
- `Source file(s): ...`
- `Evidence phrase or rule: ...`
- `Implication for the task: ...`

## Fixed questions
1. From the docs and packet, what is the assignment in one sentence?
2. What exact task-scope files/paths did you ground on for this task?
3. What exact things are out of scope?
4. Which repo owns the main concern, and why?
5. Is this blocker-scoped or family-scoped work, and why?
6. What exact verification is required before reporting completion?
7. What is the most likely mistake you would make if you rushed this task?

## Adaptive questions for cycle 10
A1. What exact evidence now supports recommending the converged output-stage/two-file briefing shape as the default for this task family?
A2. What exact evidence would still justify widening beyond the two-file default, if it appeared later?
A3. What exact final recommendation block should be emitted for the converged hypothesis?

## Final recommendation block
After the 10 answers, output exactly these headings with filled content and no bullets:
Recommended blocker lane:
Task-scope paths grounded on:
Smallest supported write scope:
Primary verification target and commands:
Hot-path or scope risks:
