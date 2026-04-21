<!-- <FILE>docs/lifecycle/LIFECYCLE_POLICY.md</FILE> - <DESC>LifecyclePolicy semantics for recipe-side AnimationManager control</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Sub-plan B Phase B.4a — document reduce-motion semantics, dismiss defaults, loop clamping, hold behavior, and the from_reduce_motion_level ingress while explicitly deferring with_procedural_registry to B.4b.</WCTX> -->
<!-- <CLOG>0.1.0: initial LifecyclePolicy guide for B.4a.</CLOG> -->

# LifecyclePolicy

`AnimationManager` now accepts an opt-in `LifecyclePolicy` that lets hosts shape lifecycle behavior without changing the existing `LifecycleState<T>` public struct or the current `dismiss(id, now)` contract.

## Surface

- `LifecyclePolicy`
  - `reduce_motion: ReduceMotionMode`
  - `default_dismiss: DismissPolicy`
  - `max_loop_iterations: Option<u32>`
- `ReduceMotionMode`
  - `Off`
  - `Soft`
  - `Hard`
- `DismissPolicy`
  - `OnComplete`
  - `OnInput`
  - `Manual`
- `AnimationId`
  - `pub type AnimationId = u64`

## Reduce-motion semantics

### `Off`

- Leaves phase progress unchanged.
- Leaves loop timing unchanged.
- Procedural ticking remains enabled.

### `Soft`

- Leaves phase progress unchanged.
- Clamps loop timing to a minimum effective period of `800ms`.
- Keeps procedural ticking enabled.

### `Hard`

- Freezes phase progress at the first frame (`phase_t = 0.0`).
- Disables loop timing (`loop_t = None`).
- Treats procedural ticking as disabled.

## Dismiss semantics

`DismissPolicy` is stored on `LifecyclePolicy` in B.4a so hosts can declare intent now without breaking the explicit `dismiss(id, now)` API. Policy-driven lifecycle emission remains part of the later B.5 work.

## Hold semantics

- `hold(id, until)` pauses manager time for the target animation using manager-owned side tables.
- The pause does **not** mutate `LifecycleState<T>`.
- While held, render-plan progress and loop timing are computed against an adjusted clock.
- Once `until` passes, the next `tick()` releases the hold and progress resumes from the frozen point.

## Trace access

- `last_trace(id)` returns `Option<Arc<SceneTrace>>`.
- The manager owns the trace side table.
- Entries are removed on `dismiss`, `remove`, and `clear`.

## SSOT ingress

`LifecyclePolicy::from_reduce_motion_level(&str)` is the manager-side ingress point for Sub-plan C's SSOT motion adapter:

- `"hard"` → `ReduceMotionMode::Hard`
- `"soft"` → `ReduceMotionMode::Soft`
- `"off"` and any unrecognized value → `ReduceMotionMode::Off`

## Deferred to B.4b

- `with_procedural_registry`
- shared manifest reconciliation
- shared changelog / quickstart updates

<!-- <FILE>docs/lifecycle/LIFECYCLE_POLICY.md</FILE> - <DESC>LifecyclePolicy guide</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
