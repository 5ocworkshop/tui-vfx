<!-- <FILE>docs/arch/compositor-next-agent-workflow-handoff.md</FILE> - <DESC>Restartable agent workflow for compositor-next direct v3.1 vertical primitive slices</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Compositor-next execution handoff: warmed low-level coding agents implement vertical slices while the lead reviews, verifies, documents, and commits.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — document the repeatable two-agent vertical-slice workflow, scoreboard, active worktrees, review gates, and restart instructions.</CLOG> -->

# Compositor-Next Agent Workflow Handoff

## Purpose

This document is the restart point for the current compositor-next execution
workflow. If a session is interrupted, a fresh lead agent should read this file
after the normal project orientation and then continue from the state recorded
below.

The work is **v3.1 direct compositor-next migration**, not generic V3 work.
The target path is:

```text
canonical v3.1 RecipeDocument
  → LoadedV31Recipe::load(...)
  → load-time descriptor/catalog/direct-render validation
  → render_v31_recipe(...)
  → tui-vfx-compositor-next copied runtime behavior
```

No new bridge, shim, legacy-input, or alias-acceptance layer should be added.

## Current Scoreboard

Descriptor pack: `descriptors/v3.1/packs/primitive.json`

Current descriptor-effect count:

```text
120 v3.1 effect descriptors
```

Signed direct compositor-next v3.1 primitives:

```text
2 / 120
```

Signed primitives:

1. `shader.linearGradient`
2. `shader.highlighter`

Remaining:

```text
118 / 120
```

Human-facing progress banner:

```text
╔════════════════════════════════════════════════════╗
║ v3.1 DIRECT MIGRATION SCOREBOARD                  ║
╠════════════════════════════════════════════════════╣
║ Signed:  2 / 120  ██░░░░░░░░░░░░░░░░░░░░  1.7%   ║
║ Active:  2 slices in flight                       ║
║ Queue:   borderSweep, revealWipe, then onward     ║
╚════════════════════════════════════════════════════╝
```

## Current In-Flight Work

Two isolated worktrees were created from commit
`58973cd52baae4b6add2ba84fc7406fd07b6cdc4`
(`Add direct v3.1 highlighter and player-next path`):

| Agent | Primitive | Worktree | Branch |
| --- | --- | --- | --- |
| Zeno | `shader.focusField` | `/usr/projects/tui-vfx-slice-focus-field` | `slice/focus-field` |
| Tesla | `shader.glistenBand` | `/usr/projects/tui-vfx-slice-glisten-band` | `slice/glisten-band` |

If these Codex subagents are still available, keep them warm and feed follow-up
slices to the same agents after their current work is reviewed and integrated.
If they are gone, recreate fresh default/no-role `gpt-5.5` low agents using the
worker prompt template below.

If a warm agent shows confusion, merge drift, or repeated violations of the
direct v3.1 constraints, stop reusing it and return to fresh agents per slice.

## Operating Model

The user-approved execution model is:

```text
Lead agent
  - reads the docs fully
  - writes work packets
  - coordinates warmed low-level agents
  - reviews their diffs as the senior engineer
  - runs ai-de-slop, architect review, code review
  - updates docs/signoff
  - verifies and commits each phase

Low-level coding agents
  - implement one vertical primitive slice at a time
  - use TDD red/green/refactor
  - stay inside their assigned worktree and write scope
  - do not commit
  - report changed files, unsupported decisions, tests, and risks
```

The lead should not do most implementation work when a slice can be assigned
cleanly. The lead may still make narrow integration fixes, but the preferred
pattern is to delegate coding and reserve lead attention for review, design
coherence, de-slop, and verification.

## Per-Slice Contract

Every primitive slice must be vertical and complete before signoff:

1. Inspect the v3.1 descriptor entry.
2. Inspect existing copied compositor/style implementation.
3. Add or update a failing regression first.
4. Observe RED when practical and record if the clean RED step is impossible.
5. Implement the smallest direct v3.1 renderer/load-validation support.
6. Reject unsupported descriptor-valid semantics at `LoadedV31Recipe::load`.
7. Accept only descriptor-canonical v3.1 values.
8. Reject unresolved runtime-sourced inputs for the current direct path.
9. Run targeted tests.
10. Run ai-de-slop on touched files.
11. Run architect review and code review.
12. Update docs/signoff artifacts.
13. Run full phase verification.
14. Commit before starting the next phase on `master`.

Documentation is part of the phase. A slice is not ready for review until code,
tests, generated artifacts, hand-maintained docs, and signoff notes are updated.

## Direct v3.1 Rules Learned So Far

- `LoadedV31Recipe::load` is the single acceptance point for direct v3.1
  execution.
- `tui-vfx-player-next` must delegate to the same compositor-next v3.1 loader
  and renderer. It must not own a second recipe-loader logic set.
- Source inputs in the current direct renderer must all be literal, even when
  the first renderer ignores some styling inputs.
- Effect inputs for supported direct primitives must all be literal.
- Do not mirror aliases from older/copy runtime internals unless those aliases
  are descriptor-canonical v3.1 values.
- Descriptor-valid-but-unsupported values should fail loudly at load time with
  `V31LoadError::UnsupportedDirectInput`.
- Current `shader.highlighter` direct decisions:
  - `mode`: supports `band`; rejects descriptor-valid `row` and `centerOut`
    until direct compositor semantics exist.
  - `applyTo`: supports `foreground`, `background`, `both`.
  - `direction`: supports `leftToRight`, `rightToLeft`, `topToBottom`,
    `bottomToTop`.
  - `textContrast`: supports only `0.0`/`TextContrast::Preserve`.
  - `rowMask`: non-negative integer maps to a single-row compositor range.

## Worker Prompt Template

Use default/no-role `gpt-5.5` low agents for coding work.

```text
Coding task: implement one vertical slice in isolated worktree <WORKTREE>.
You are not alone in the repo; do not touch other worktrees and do not commit.
Primitive: <PRIMITIVE_ID>.

Follow repo workflow:
- TDD red/green/refactor.
- v3.1 only.
- No bridge/shim/legacy aliases.
- Validation happens at LoadedV31Recipe::load.
- Rendering passes the loaded v3.1 structure directly through
  tui-vfx-compositor-next::v31.
- Use OFPF tools to inspect descriptors and existing copied compositor/style
  implementation.

Suggested scope:
- crates/tui-vfx-compositor-next/src/v31/load.rs
- crates/tui-vfx-compositor-next/src/v31/render.rs
- compositor-next direct recipe tests
- docs/signoff if needed

Keep the change minimal and vertical:
- Add a canonical v3.1 fixture/test for <PRIMITIVE_ID>.
- Prove RED unsupported when practical.
- Implement supported descriptor-canonical subset using existing copied behavior.
- Reject unsupported descriptor-valid semantics at load with
  V31LoadError::UnsupportedDirectInput.
- Validate every authored source/effect input remains literal.

Run cargo fmt and targeted tests.
Return changed files, unsupported decisions, test commands/results, and
integration risks. Do not commit.
```

## Integration Procedure

When a worker finishes:

1. Inspect its worktree status and diff.
2. Review only the slice-owned files first.
3. Run targeted tests in that worktree.
4. If acceptable, merge or cherry-pick into `master` after ensuring `master` is
   clean.
5. Resolve conflicts manually as lead; do not let agents blindly merge each
   other.
6. Run ai-de-slop on the integrated changed files.
7. Run formal architect and code review.
8. Iterate on review blockers.
9. Run full phase verification.
10. Commit in the project’s current commit-message style:

```text
<subject>

Work Context:
  - <why this phase exists>

Changes:
* <path> (Version X.Y.Z):
  - <what changed>
```

Do not add co-authors.

## Required Verification Gates

At minimum, after integrating a slice:

```bash
cargo fmt --check
git diff --check
cargo test -p tui-vfx-compositor-next --test test_v31_direct_recipe -- --nocapture
cargo test -p tui-vfx-player-next --test player_next_v31 -- --nocapture
cargo test -p tui-vfx-compositor-next --test test_old_compositor_parity
cargo test -p tui-vfx-player --test test_compositor_next_primitive_tree
cargo test -p tui-vfx-compositor-next
cargo check -p tui-vfx-player-next
cargo clippy -p tui-vfx-compositor-next --all-targets -- -D warnings
cargo clippy -p tui-vfx-player-next --all-targets -- -D warnings
```

Run additional tests for any crate or tooling touched by the slice.

## Active Queue

Current active:

1. `shader.focusField`
2. `shader.glistenBand`

Recommended next queue, unless current slices reveal a better order:

3. `shader.borderSweep`
4. `shader.revealWipe`
5. Remaining shader primitives by complexity and migration demand
6. Filters
7. Masks
8. Samplers
9. Style effects
10. Complex/composition primitives last

Keep the scoreboard updated after each committed primitive.

## Recovery Checklist for a Fresh Lead Agent

1. Run project orientation and fully read steering files required by this repo.
2. Read:
   - `docs/arch/compositor-next-vertical-implementation-plan.md`
   - `docs/arch/v31-schema-boundary-north-star.md`
   - `docs/arch/primitive-workbench-schema-driven-workflow.md`
   - this file
3. Check `git status --short` in `/usr/projects/tui-vfx`.
4. Check worktrees:

```bash
git worktree list --porcelain
```

5. If warm agents are still active, wait for their reports.
6. If warm agents are not active, inspect the two slice worktrees manually and
   either continue the branches or recreate fresh agents from `master`.
7. Resume with lead review/integration, not broad implementation by the lead.

<!-- <FILE>docs/arch/compositor-next-agent-workflow-handoff.md</FILE> - <DESC>Restartable agent workflow for compositor-next direct v3.1 vertical primitive slices</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
