# Orchestration Protocol

Purpose: persistent instructions for how the leader agent should manage subagents and work packets during long-running V3 / library migration work, especially after context compaction or reset.

## 1. Leader vs subagent split

The leader agent owns:
- architecture
- sequencing
- integration
- verification interpretation
- commits / final reporting
- steering alignment

Subagents own bounded lanes only:
- tightly scoped implementation slices
- focused audits
- doc cleanup lanes
- recipe cleanup lanes
- targeted validation/research lanes

Do not outsource core architectural judgment or final verification.

## 2. Mandatory pre-dispatch context

Never dispatch a subagent without explicitly requiring it to read the relevant governing docs first.

Read order matters. Put the highest-value steering files first so the most
important principles remain near the top of the agent's context window.

Required first-pass steering read order for subagents:
1. `/usr/projects/tui-vfx/steering/INTENTIONS.md`
2. `/usr/projects/tui-vfx-recipes/steering/INTENTIONS.md`
3. `/usr/projects/mixed-signals/steering/INTENTIONS.md` when touching lower signal/math substrate
4. current shared briefing document for the active lane

Second-pass supporting docs:
- `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-recipe-vocabulary.md`
- `/usr/projects/global_prompts/standards/40_ofpf_standards.md`
- `/usr/projects/global_prompts/standards/50_tdd_protocol.md`
- `/usr/projects/global_prompts/standards/60_file_centric_execution.md`
- `/usr/projects/global_prompts/standards/65_subagent_orchestration.md`
- additional task-specific design/schema docs as needed

Leader note:
- this ORCHESTRATION file is primarily leader-facing
- subagents should not normally be told to read it directly
- instead, the leader should distill the relevant orchestration rules into the
  active briefing and the concrete task packet

Reflection step before action:
- after reading the first-pass steering docs, the subagent should briefly
  restate to itself (and later in the report if useful) the key repo-boundary
  rules and the definition of done for the assigned lane
- only then should it move on to orientation commands and deeper code reading

Every subagent prompt must explicitly instruct the agent to say it read those docs in the final report.

## 3. Orientation first

After the first-pass steering read and short reflection, subagents should take
orientation snapshots for the repos in scope before deeper code reads:
- `ofpf-orientation --root /usr/projects/tui-vfx`
- `ofpf-orientation --root /usr/projects/tui-vfx-recipes`
- `ofpf-orientation --root /usr/projects/mixed-signals`
- `ofpf-orientation --root /usr/projects/gt-design` when GTD is in scope

Prefer OFPF tools first for exploration and targeted reads. Fall back to narrow `rg` / `sed` / `jq` only when OFPF is unavailable or insufficient.

## 4. Work packet format

Each subagent packet should include, in this preferred order:
1. task first (one-sentence assignment)
2. why this matters
3. success condition
4. mode (`BLOCKER_MODE` / `FAMILY_MODE` when relevant)
5. exact task-scope paths used for grounding
6. exact write scope
7. explicit out-of-scope items
8. must-read docs in order
9. repo-boundary guardrails
10. first steps / grounding instructions
11. verification expected (exact shell-ready commands when possible)
12. reporting contract
13. closing task reminder

Default post-experiment stance:
- use `gpt-5.4-mini` with `reasoning_effort: medium` as the default bounded-work
  helper when the packet is concrete
- reserve `gpt-5.4` with `reasoning_effort: medium` for ambiguity, multi-system
  judgment, or higher-cost implementation mistakes
- use `gpt-5.3-codex-spark` with `reasoning_effort: low` only for low-context,
  doc-only helper lanes
- use unroled `gpt-5.5` with `reasoning_effort: low` when the owner explicitly
  wants that lane shape or when a packet is implementation-heavy but still
  concrete enough to avoid architectural freelancing
  - do not assign a role in the spawn request when the owner asks for this lane;
    role selection can silently force a different model/effort profile
  - keep the task packet narrower than a normal frontier packet: exact write
    scope, exact non-goals, exact tests, and exact edge-case probes are required
  - expect good in-scope execution and honest reporting, but leader review must
    still check semantic edge cases, file-size/OFPF pressure, and whether tests
    landed in the intended harness
  - prefer sending one mid-flight status check after the first compile/test
    failure report; do not repeatedly interrupt if the agent is making localized
    progress
  - after the agent returns, run the verification locally and do at least one
    leader-owned edge-case review before accepting the lane
  - include one canonical JSON/fixture shape when the packet touches parser,
    schema, or DTO propagation; otherwise the agent may pass the concept but
    miss a plan-required default or boundary assertion
- keep up to 5 useful lanes busy when independent work exists

Work packet quality rule:
- write packets for a junior-but-capable engineer who is not deeply familiar
  with the project
- packet quality is the first lever; improve packet specificity before jumping
  to a bigger model
- the experiment showed that exact path strings and exact verification wording
  are what keep bounded lanes stable, so name them whenever the docs support
  them
- include enough detail, map, guardrails, and verification direction that the
  assignee can stay accurate without guessing at repo boundaries or widening
  scope
- do not over-specify to the point that the packet is doing the work itself
- explicitly separate:
  - `task-scope paths` = the files/areas the agent grounded on to understand the
    problem
  - `write scope` = the smallest justified set of files they may actually edit
- require exact path strings rather than repo-name-only summaries whenever the
  packet can support that level of specificity
- require shell-ready verification commands rather than conceptual “run tests”
  summaries whenever the packet can support that level of specificity
- if the packet cannot justify an exact path or command yet, say that it is not
  concrete enough instead of inventing one
- if the packet names exact verification commands, the assignee should run or
  report those exact commands rather than swapping in broader substitutes
- when in doubt, add:
  - clearer in-scope / out-of-scope bullets
  - explicit repo-boundary reminders
  - explicit first steps
  - exact verification commands
  - a closing task reminder
- packets that authorize edits to metadata-bearing files must include the file
  metadata rule: `<CLOG>` / `// <CLOG>` is only 1-2 short lines about the latest
  file change, not a running history; git is the history
- for `gpt-5.5` low implementation packets, add explicit self-checks for:
  - non-obvious coordinate-frame or timing semantics named by the plan
  - OFPF soft/hard line pressure before final report
  - whether test filters actually ran the new tests
  - any pre-existing dirty files so the helper does not claim ownership of
    unrelated changes
  - whether parser/schema tests cover both acceptance and defaults, not just
    propagation through later phases

Post-experiment evidence trace:
- `/usr/projects/tui-vfx/steering/experiments/subagent-briefing-experiment-results.md`
  showed packet quality, exact pathing, and exact verification wording were the
  main drivers of stable bounded-lane performance
- `/usr/projects/tui-vfx/steering/experiments/model-compare/mini-medium-results.md`
  showed the smaller helper was unreliable on underspecified packets and became
  stable once the lane, scope, and command contract were made concrete
- `/usr/projects/tui-vfx/steering/experiments/model-compare/gpt54mini-medium-results.md`
  showed the corrected mini-medium setup is the best default for concrete
  bounded lanes, including blocker-scoped validator/tooling work
- `/usr/projects/tui-vfx/steering/experiments/model-compare/gpt54-medium-results.md`
  and `/usr/projects/tui-vfx/steering/experiments/model-compare/frontier-low-results.md`
  showed frontier helpers stay strong on harder boundary judgment but tend to
  need more prompting pressure to stay exact on paths, commands, and final
  recommendation shape
- codex-spark operational learnings from the doc-task experiment family remain
  stable: use spark for low-context doc surfaces only, not for runtime lanes or
  broad repo-context work
- first `gpt-5.5` low Packet 1 for V3 per-cell motion was strong enough to use
  again: it stayed in scope, read the required docs, produced passing tests, and
  reported honestly. Leader review still found a semantic coordinate-frame issue
  (`local_frame` origin vs returned scene-local coordinates) and OFPF/test
  placement cleanup. Lesson: this lane is effective when the packet spells out
  tricky semantics as direct tests, but final acceptance remains a leader-owned
  integration/review step.
- second `gpt-5.5` low Packet 2 reinforced the pattern: it cleanly carried typed
  cell-motion data through recipes DTO/normalization/compile and ran the right
  docs check, but leader review added the missing parser-default assertion. For
  schema packets, tasking should include a canonical JSON snippet plus a checklist
  for accept/reject/default/propagation tests.
- third `gpt-5.5` low Packet 3 showed that implementation lanes can find the
  real runtime seam when tasking permits narrow repo-boundary adjustment: root
  runtime integration lived in `tui-vfx-recipes`, not the compositor crate. For
  runtime packets, name likely preview/player/probe surfaces and explicitly ask
  for reduced-motion source discovery plus remaining tooling touchpoints, rather
  than assuming the engine repo owns the whole path.

Current model/task routing after the experiment family closed:
- `gpt-5.4-mini` with `reasoning_effort: medium` is the operational default
  bounded-work model when the packet is concrete
  - use it for:
    - blocker-scoped audits
    - targeted tests and validator/tooling seams
    - bounded fixture/doc cleanup
    - small-to-medium implementation slices with clear file ownership
    - packet-driven doc/process lanes that still need boundary discipline
  - avoid using it as the first choice when:
    - the lane is still ambiguous across repos or layers
    - the packet cannot yet name the smallest justified write scope
    - the task depends on non-trivial architectural tradeoff judgment
  - expected review level:
    - normal review always required
    - closer review for interaction semantics, keybindings, nuanced UX, and any
      lane where the packet still leaves real ambiguity
- `gpt-5.3-codex-spark` with `reasoning_effort: low` is a fast doc-only helper
  for low-context surfaces
  - use it for:
    - compact operator docs
    - command references
    - handoff notes
    - small experiment/protocol drafts
    - narrow steering/process edits that do not require broad repo-memory
  - avoid using it by default for:
    - runtime code changes
    - architectural or ownership judgment
    - packets that need sustained multi-doc synthesis or broad repository
      context retention
  - expected review level:
    - moderate review for wording accuracy, path correctness, scope truth, and
      final doc completeness
- `gpt-5.4` with `reasoning_effort: medium` is the escalation model for harder
  bounded lanes
  - use it for:
    - trickier architectural audits
    - more ambiguous multi-system seams
    - packets that need the leader to resolve competing repo-boundary signals
    - harder implementation slices where the cost of a wrong move is higher
  - review implication:
    - expect a higher exactness-review burden on bounded packets; specifically
      check that the helper named exact paths, exact commands, and a filled
      final recommendation instead of abstract summaries
  - do not use it reflexively when `gpt-5.4-mini` plus a better packet is
    sufficient

Grounding-first rule for research/evaluation packets:
- if the packet is testing comprehension or prompt quality, require the helper
  to complete all grounding work first
- then require the helper to stop and declare `READY FOR QUESTIONS`
- only after that should the evaluator ask comprehension questions
- do not score answers from a helper that skipped the grounding-first stop point

## 5. Path discipline

The orchestration root is `/usr/projects/gt-design`, but active implementation may be primarily in:
- `/usr/projects/tui-vfx`
- `/usr/projects/tui-vfx-recipes`
- `/usr/projects/mixed-signals`

Use full paths in briefing docs, task packets, and reports whenever ambiguity is possible.

## 6. Performance rule

Always watch for non-performant patterns in touched scopes.
Target budget: 16.7 ms per frame for 60 FPS.

Flag or avoid:
- unnecessary nested loops on hot paths
- repeated allocations in per-cell/per-frame work
- recomputation that can be hoisted or cached
- avoidable string/Vec churn in render-time code
- unnecessary delays or synchronization on active playback paths

Subagents should call out hot-path risks even in audit lanes.

## 6A. Key-binding safety rule

When a task touches keyboard bindings:
- first inspect the current key map / handlers / help text before assigning a
  new binding
- do this even if the task instructions suggest a specific key
- if the requested key is already in use, do not silently override it
- choose an alternate key that minimizes collisions and report:
  - the collision you found
  - the alternate key you chose
  - why that alternate was safer
- update help text and any on-screen binding references together with the code
- treat keybinding collisions as a correctness issue, not a cosmetic one

## 7. Debug-recipe rules

When touching debug recipes:
- keep them polished and professional
- ensure the effect is clearly visible
- ensure message text fits the rect cleanly
- include description text explaining what the viewer should see
- use the standard body-text pattern:
  - line 1: category + human-readable effect name
  - line 2: concise behavioral cue
- do not create separate recipe categories just because a recipe is runtime-bound or procedural
- directory structure should follow what the recipe demonstrates, not merely how a value is driven

When updating a shader/filter/mask/sampler/style/content effect or other
primitive/effect family:
- update the corresponding debug recipe(s) in the same tranche whenever the
  visual semantics, naming, timing, authoring vocabulary, or parameter story
  are affected
- normalize the family in one pass when possible:
  - shared math substrate
  - timing semantics
  - vocabulary/comments/naming
  - rustdocs/public docs
  - generated-doc/schema metadata and drift checks when public or
    schema-bearing surfaces change
  - matching debug recipes/descriptions/layouts
- when repeated reusable math patterns are discovered, apply the boundary rule:
  - if the math is signal/math substrate
  - and it is reusable across 3+ real use cases
  - and it is not inherently tied to one renderer/effect semantic
  - then move it into `mixed-signals` instead of re-rolling it locally
- if the pattern is effect/render semantics, keep it in `tui-vfx` /
  `tui-vfx-recipes` and only extract the lower reusable math
- ensure those debug recipes remain high-quality references with:
  - clear descriptions
  - correct body text
  - adequate layout/contrast
  - representative timing/variation
- treat this as part of the definition of done for that family update, not as
  optional follow-up polish

## 8. Timing / architecture rules currently in force

- cadence-driven motion should use monotonic elapsed time, not reset-on-loop normalized time
- keep normalized phase/loop progress available separately
- do not use recipe-period hacks as architectural fixes
- move reusable math/signal substrate lower into `mixed-signals`
- keep effect semantics in `tui-vfx` / `tui-vfx-recipes`
- when a truly different geometric model appears, add an explicit new basis rather than mutating old primitive semantics
- treat missing rustdocs, missing generated-doc inputs, stale debug recipes, or
  unvalidated schema drift as incomplete pipeline-family work

## 9. Repo-state expectations

During active library migration:
- GTD may be temporarily non-compiling
- do not treat GTD compile failures as blocking if they are downstream fallout from in-progress upstream library changes
- prioritize making the libraries and recipe pipeline correct first

## 10. Reporting expectations

Subagent final reports must include:
- explicit note that must-read docs were read
- changed files
- concrete evidence / commands run
- blockers or recommended next handoff
- any performance risks noticed

Leader summaries should stay short, concrete, and evidence-based.

## 10A. Merge/commit discipline

When agent work is accepted into the main working tree:

- commit that accepted merge promptly rather than letting many accepted merges
  accumulate uncommitted
- use one commit per accepted workset when practical so recovery/review stays
  clean
- include a brief user-facing summary of what was achieved/completed when the
  merge is accepted
- if work is not yet acceptable, send it back for refinement instead of mixing
  it into the main line prematurely
- once accepted work is committed, close the finished agent to free the slot

Work-packet lifecycle rule:
- do not treat a packet as complete just because an agent response looked good
- a packet becomes complete only after the related work is actually committed on
  `master`
- once the related commit is visible on `master`, remove or archive the packet
  from the active work-packet queue so the live backlog reflects real remaining
  work

## 11. Refresh rule after context reset

If context is compacted or partially lost, reread:
1. `/usr/projects/tui-vfx/steering/ORCHESTRATION.md`
2. current active shared briefing doc
3. relevant steering/design docs for the active lane

Then continue from the latest verified state rather than restarting discovery from scratch.


## 12. Queue discipline

The leader should keep useful agent lanes queued so agent slots stay productive.

Rules:
- always have the next bounded task packet ready before an agent finishes when possible
- prioritize reviewing and responding to agent completions over continuing long local work
- do direct work in the gaps between agent reviews, not at the expense of leaving completed agents idle
- if local work is more than a step or two from a natural pause, stop and switch focus when agents finish so the queue keeps moving
- review completed agent work promptly
- accept and integrate good work quickly
- send work back for refinement when it does not yet meet steering or verification standards
- once a workset is accepted and committed (or clearly no longer needed), close the agent to free the slot
- prefer keeping up to 5 useful bounded lanes busy rather than letting completed agents linger idle

## 13. Recursive self-improvement loop

The leader should periodically improve the orchestration approach itself rather
than treating prompts/briefings as fixed.

Cadence:
- after roughly every 5 completed work packets, pause for a short retrospective
- also run the retrospective sooner if there is obvious prompt drift, repeated
  cleanup, repeated scope creep, or repeated verification gaps

What to review:
- which task packets came back cleanly and why
- which packets needed correction and why
- whether the task scope was too broad or too vague
- whether architectural boundary rules were too buried
- whether performance guidance was too generic for the hot path
- whether the agent needed more explicit out-of-scope instructions
- whether the verification contract was specific enough
- whether the ordering of prompt sections helped or hurt

What to adjust:
- wording
- ordering
- level of detail
- definition-of-done bullets
- out-of-scope bullets
- verification checklist
- explicit “if you see X, do Y” rails
- prompt templates for recurring task families

Preferred task-packet ordering for bounded lanes:
1. task first
2. why this matters
3. success condition
4. mode
5. task-scope grounding paths
6. exact write scope
7. explicit out-of-scope items
8. must-read docs
9. repo-boundary reminders
10. first steps / grounding instructions
11. hot-path/performance reminders
12. required verification
13. reporting contract
14. closing task reminder

Institutionalization rule:
- when a retrospective yields a durable lesson, update this file and, if
  currently relevant, the active shared briefing document
- do not keep repeating the same orchestration mistake across cycles if the
  lesson can be captured once here

Quality rubric for reviewing medium-agent output:
- correctness of the main claim
- adherence to write scope
- respect for architecture boundaries
- verification quality
- performance awareness
- reporting clarity

Escalation rule:
- if the same type of issue appears in 2 or more recent packets, treat it as a
  briefing/prompt design failure and fix the orchestration materials before
  dispatching more similar work
