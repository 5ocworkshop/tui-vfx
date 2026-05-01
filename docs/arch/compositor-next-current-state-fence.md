<!-- <FILE>docs/arch/compositor-next-current-state-fence.md</FILE> - <DESC>Current-state fence for compositor-next vertical implementation before additive copy/workbench work continues</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Compositor-next Phase 0 — record dirty state, constraints, and additive boundaries before continuing vertical implementation.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — capture Phase 0 current-state fence for compositor-next vertical implementation.</CLOG> -->

# Compositor-Next Current-State Fence

## Purpose

This note records the starting state for the compositor-next vertical implementation lane. It exists so later phases can distinguish pre-existing exploratory work from compositor-next-owned work and avoid destructive cleanup.

## Governing direction

Read in full before this fence was written:

- `docs/arch/compositor-next-vertical-implementation-plan.md`
- `docs/arch/v31-schema-boundary-north-star.md`
- `docs/arch/primitive-workbench-schema-driven-workflow.md`
- `.omx/plans/prd-compositor-next-vertical.md`
- `.omx/plans/test-spec-compositor-next-vertical.md`
- `/usr/projects/tui-vfx-recipes/steering/INTENTIONS.md`

Execution constraints:

- Work vertically, not horizontally.
- Copy the hardened compositor first; do not rewrite it.
- Preserve old `tui-vfx-compositor` as fallback while compositor-next matures.
- Do not broadly revert `main`.
- Treat current `v3.1/debug_recipes` artifacts as reference/exploratory evidence unless a vertical primitive slice explicitly owns one.
- Run descriptor/schema hindsight audit before broad workbench generation.
- Use one existing shader primitive for the first vertical slice.
- Keep `source.indexedField` reserved for the first new from-scratch primitive after existing primitive migration workflow is proven.
- Follow the compositor-next plan-specific file-size heuristic: around 300 LOC is the reviewability target, and files above 500 LOC require a split or written cohesion justification. OFPF prefix-specific limits still apply where they are stricter.
- Follow TDD for behavior changes: RED, GREEN, then refactor/de-slop.

## Intake repository state

At intake for this Ralph run, branch/head was:

```text
master @ ec7ed70
```

`git status --short` reported pre-existing modified paths:

```text
 M Cargo.lock
 M Cargo.toml
 M docs/INDEX.md
 M docs/arch/INDEX.md
 M steering/ORCHESTRATION.md
 M steering/TASK_PACKET_TEMPLATE.md
 M steering/work-packets/COMMON_EXECUTION_RULES.md
 M steering/work-packets/README.md
```

and pre-existing untracked paths:

```text
?? crates/tui-vfx-compositor-next/
?? crates/tui-vfx-player/tests/test_compositor_next_primitive_tree.rs
?? docs/arch/v31-primitive-schema-hindsight-audit.md
?? primitives/
?? steering/SUBAGENT-GROUNDING.md
```

The untracked `crates/tui-vfx-compositor-next/` tree already contains a full copied crate shape, copied tests, and `tests/test_old_compositor_parity.rs`. The untracked `primitives/` tree currently contains a `shader/linear_gradient` primitive tree, not `shader/highlighter`.

This fence does not accept or reject those artifacts. It only records that they existed before this phase continued. Later phase review must classify them before claiming ownership.


## Sibling recipe repository state

Phase 0 also requires a starting-state baseline for `/usr/projects/tui-vfx-recipes` because the current v3.1 debug recipe corpus is reference evidence for later primitive slices. At intake, that repository reported:

```text
master @ c7e4fa6
working tree clean
```

The current recipe corpus contains 180 JSON files under `recipes/v3.1/debug_recipes/`, with top-level counts observed at intake:

```text
34 filters
33 content
25 styles
20 masks
18 scene
18 shaders
12 samplers
11 sources
4 complex
4 event_driven_dwell
1 root baseline fixture
```

These recipe artifacts are classified as `reference-current` / exploratory evidence for compositor-next planning. This fence does not mutate them and does not mark them as compositor-next validated. A later vertical primitive slice may own a specific recipe only when that slice explicitly names it.

## OFPF state

`ofpf-status --root /usr/projects/tui-vfx` initially reported the graph loaded but stale. The graph was regenerated with:

```bash
ofpf-load --root /usr/projects/tui-vfx
```

## Phase 0 boundary decisions

- No broad revert or cleanup is authorized by this fence.
- Additive compositor-next/workbench paths are the only allowed continuation path.
- Existing exploratory recipe artifacts remain available for human inspection.
- Current dirty state must be handled by path-scoped commits and reports rather than a blanket commit or blanket reset.
- Phase 2 copy/parity work may build on the existing untracked `crates/tui-vfx-compositor-next/` only after confirming it is a mechanical copy or documenting any deviations.
- Phase 3.5 audit work may build on `docs/arch/v31-primitive-schema-hindsight-audit.md` only after review against the plan acceptance criteria.

## Next phase entry criteria

Before Phase 1/2 claims completion:

1. Confirm `tui-vfx-compositor-next` metadata states copy-first/no-rewrite purpose.
2. Confirm workspace metadata includes the new crate intentionally.
3. Run RED/GREEN parity or compile tests in TDD order for any new behavior assertion.
4. Keep Phase 2 diff reviewable as copy/rename plus explicit parity proof, not behavior rewrite.

<!-- <FILE>docs/arch/compositor-next-current-state-fence.md</FILE> - <DESC>Current-state fence for compositor-next vertical implementation before additive copy/workbench work continues</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
