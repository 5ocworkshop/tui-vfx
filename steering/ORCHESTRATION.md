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

Minimum must-read set for this project family:
- `/usr/projects/tui-vfx/steering/ORCHESTRATION.md`
- current shared briefing document for the active lane
- `/usr/projects/tui-vfx/steering/INTENTIONS.md`
- `/usr/projects/tui-vfx-recipes/steering/INTENTIONS.md`
- `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-recipe-vocabulary.md`
- `/usr/projects/global_prompts/standards/40_ofpf_standards.md`
- `/usr/projects/global_prompts/standards/50_tdd_protocol.md`
- `/usr/projects/global_prompts/standards/60_file_centric_execution.md`
- `/usr/projects/global_prompts/standards/65_subagent_orchestration.md`
- `/usr/projects/mixed-signals/steering/INTENTIONS.md` when touching lower signal/math substrate
- additional task-specific design/schema docs as needed

Every subagent prompt must explicitly instruct the agent to say it read those docs in the final report.

## 3. Orientation first

Before deeper reads, subagents should start with orientation snapshots for the repos in scope:
- `ofpf-orientation --root /usr/projects/tui-vfx`
- `ofpf-orientation --root /usr/projects/tui-vfx-recipes`
- `ofpf-orientation --root /usr/projects/mixed-signals`
- `ofpf-orientation --root /usr/projects/gt-design` when GTD is in scope

Prefer OFPF tools first for exploration and targeted reads. Fall back to narrow `rg` / `sed` / `jq` only when OFPF is unavailable or insufficient.

## 4. Work packet format

Each subagent packet should include:
- objective / success condition
- exact write scope
- explicit out-of-scope items
- stop condition
- required docs to read first
- verification expected
- requirement to use full paths in report
- performance reminder when relevant

Use medium `gpt-5.4` subagents by default for bounded tasks.
Keep up to 5 useful lanes busy when independent work exists.

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
