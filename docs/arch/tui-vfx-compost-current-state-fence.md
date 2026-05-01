<!-- <FILE>docs/arch/tui-vfx-compost-current-state-fence.md</FILE> - <DESC>Current-state fence for the tui-vfx-compost clean-sheet pure v3.1 compositor build</DESC> -->
<!-- <VERS>VERSION: 1.0.0</VERS> -->
<!-- <WCTX>tui-vfx-compost state fence: copied-crate artifacts are historical recovery material; active work is clean-sheet v3.1 compost substrate.</WCTX> -->
<!-- <CLOG>1.0.0: MAJOR — replace the copied-crate current-state fence with the active tui-vfx-compost clean-sheet v3.1 state boundary.</CLOG> -->

# tui-vfx-compost Current-State Fence

## Purpose

This fence records the active boundary for the clean-sheet `tui-vfx-compost`
work. It prevents future sessions from confusing historical copied-crate
artifacts with the current target.

## Active Target

The active compositor target is:

```text
crates/tui-vfx-compost/
```

The active data path is:

```text
v3.1 recipe
  → load-time validation / canonicalization
  → canonical loaded v3.1 structure
  → tui-vfx-compost source / scene / render substrate
  → tui-vfx-compost primitive runtime
  → rendered output
```

## Completed / Accepted State

The following parts are complete enough to rely on:

- v3.1 native transition model is documented and implemented.
- v3.1 descriptor/schema ambiguous-name audit is complete and guardrailed.
- Basic `tui-vfx-compost` crate/family directory structure exists.
- Root README anchors exist for the empty primitive families and validation
  family directories.
- `shader.linearGradient` is the first compost primitive proof seed.

## Historical / Recovery Material

Historical copied-crate work and preserved slice worktrees may contain useful
implementation ideas, but they are not the active target and must not be merged
blindly.

Rules:

- Do not delete preserved worktrees until the owner explicitly approves cleanup.
- Do not copy whole crates or whole directory trees from recovery material.
- Re-review any preserved primitive diff against current v3.1 schema names and
  the compost file layout before using it.
- Treat old copied-crate paths as abandoned recovery evidence, not as an
  implementation surface.

## Read-Only Reference

`crates/tui-vfx-compositor/` remains the read-only behavior reference for proven
runtime and primitive logic.

Allowed use:

- inspect file organization;
- inspect robust rendering behavior;
- inspect tests and helper patterns;
- port the minimum necessary behavior into OFPF-shaped compost modules.

Forbidden use:

- editing the reference crate for compost work;
- carrying old DTOs forward as a v3.1 execution layer;
- adding bridge/shim/lowering paths to preserve legacy field shapes.

## Current Next Phase

The next phase is **non-primitive substrate migration** into `tui-vfx-compost`:

```text
frame/cell/sample context
source materialization
scene/layer placement
render orchestration
signals / loopback / procedural value support
explicit diagnostics
```

This phase comes before broad primitive fan-out.

## Current Risks

- Old copied-crate files still exist in the working tree and can mislead agents.
- Preserved slice worktrees may use stale schema field names.
- External debug recipes may still contain older v3.1 names and should be
  canonicalized instead of supported through compatibility aliases.
- Broad primitive parallelism should remain paused until the compost substrate is
  stable enough to review slices against one target layout.

## Boundary Decisions

- The schema is stable enough to execute; reopen only for proven contract bugs.
- Validation happens at load time.
- Runtime execution reads canonical v3.1 directly.
- No `src/v31/`, `rendering/`, `bridge/`, `adapter/`, or `lowering/` runtime
  tree belongs in compost.
- Documentation, generated artifacts, tests, and signoff notes are part of every
  phase.
- Each phase gets de-slop, architect review, code review, iteration, and a
  commit before the next phase starts.

## Resume Checklist

Before resuming code work:

1. Read `steering/INTENTIONS.md`, `steering/OFPF-TOOLS.md`, and
   `steering/ORCHESTRATION.md` fully.
2. Read `docs/arch/tui-vfx-compost-agent-workflow-handoff.md` fully.
3. Read `docs/arch/tui-vfx-compost-vertical-implementation-plan.md` fully.
4. Confirm work is targeting `crates/tui-vfx-compost/`.
5. Confirm no packet asks an agent to create a crate copy, nested checkout, or
   translation layer.

<!-- <FILE>docs/arch/tui-vfx-compost-current-state-fence.md</FILE> - <DESC>Current-state fence for the tui-vfx-compost clean-sheet pure v3.1 compositor build</DESC> -->
<!-- <VERS>END OF VERSION: 1.0.0</VERS> -->
