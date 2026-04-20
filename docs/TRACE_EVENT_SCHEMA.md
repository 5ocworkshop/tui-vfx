<!-- <FILE>docs/TRACE_EVENT_SCHEMA.md</FILE> - <DESC>Canonical schema reference for TraceEvent + TraceEnvelope shipped in tui-vfx-debug::inspection (since 0.9.0)</DESC> -->
<!-- <VERS>VERSION: 0.2.0</VERS> -->
<!-- <WCTX>Sub-plan B Phase B.1 — extend the schema reference with the shared TraceEmitter/TraceFrameContext stamping contract used by the compositor bridge and future recipe-side emitters, while preserving the canonical event/envelope tables.</WCTX> -->
<!-- <CLOG>0.2.0: document TraceFrameContext + TraceEmitter as the shared stamping authority for TraceEnvelope emission across bridge/composer emit sites.
0.1.0: initial AI-consumption schema reference covering all 18 TraceEvent variants across the four stages (lifecycle / resolution / composition / pipeline), envelope fields, stage mask semantics, NDJSON format, determinism.</CLOG> -->

# TraceEvent Schema — `tui-vfx-debug::inspection` (since 0.9.0)

This document is the canonical schema reference for the unified inspection surface the `tui-vfx` workspace exposes in `tui-vfx-debug::inspection`. It is intentionally AI-consumption-ready: a single place where an LLM (or human) can look up **every field of every variant** without running rustdoc.

- **Source of truth:** `crates/tui-vfx-debug/src/inspection/cls_trace_event.rs`.
- **Design spec:** gt-design `docs/superpowers/specs/2026-04-20-recipe-scene-composer-design.md` §9.
- **NDJSON CLI:** `tui-vfx-trace` — lands in Sub-plan B in the sibling `tui-vfx-recipes` repo (`tools/tui-vfx-trace/`).

## 1. Envelope — `TraceEnvelope`

Every emitted event is wrapped in a `TraceEnvelope` before it reaches the sink.

| Field | Type | Description |
|---|---|---|
| `event` | `TraceEvent` | The wrapped event (see §2–§5). |
| `frame_no` | `u64` | Monotonic frame counter since manager start. |
| `t_ms` | `u64` | Elapsed milliseconds since manager start. |
| `recipe_id` | `Option<RecipeId>` | Recipe identity if the event occurred inside a managed recipe run. `None` for workspace-level events (e.g. CLI-level resolution). Serialised as a flat string (opaque newtype; compare by content). |
| `seq_in_frame` | `u32` | Per-frame sequence counter (resets at each new frame). Provides a stable replay order when many events share the same `t_ms`. |

**NDJSON format:** one envelope per line, terminated with `\n`. `TraceReport::to_ndjson(writer)` / `TraceReport::from_ndjson(reader)` round-trip losslessly over the envelope list (the sink-side `dropped` counter is not transported; replays start at 0).

## 1.1 Shared emission authority — `TraceFrameContext` + `TraceEmitter`

The canonical producer-side contract is now: **one `TraceEmitter` per
frame-cycle, shared by reference across every emit site that belongs to
that frame**. The emitter owns the current `TraceFrameContext`
(`frame_no`, `t_ms`, optional `recipe_id`) plus the monotonic
`seq_in_frame` counter.

| Type | Fields / methods | Notes |
|---|---|---|
| `TraceFrameContext` | `frame_no: u64`, `t_ms: u64`, `recipe_id: Option<RecipeId>`, `new(frame_no, t_ms)`, `with_recipe_id(id)` | Cloneable frame metadata stamped onto every emitted envelope. |
| `TraceEmitter` | `new(sink, frame)`, `begin_frame(frame)`, `emit(event)` | `begin_frame` swaps the frame context and resets `seq_in_frame` to 0; `emit` atomically increments the per-frame sequence and forwards a stamped `TraceEnvelope` to the shared sink. |

`InspectionSinkBridge` now delegates its emission path to `TraceEmitter`;
future recipe-side lifecycle/composition emitters are expected to borrow the
same emitter instance for the active frame so sequence ordering stays global
within that frame.

## 2. Lifecycle stage — `StageMask::LIFECYCLE`

Source: `tui-vfx-recipes::manager` (emitters land in Sub-plan C).

### `LifecyclePhaseEntered { id, phase, t_ms }`

| Field | Type | Description |
|---|---|---|
| `id` | `RecipeId` | Recipe identity. |
| `phase` | `String` | Phase name entered (e.g. `"enter"`, `"dwell"`, `"exit"`). |
| `t_ms` | `u64` | Elapsed milliseconds since manager start (also duplicated in envelope for convenience). |

### `LifecyclePhaseTransition { id, from, to, t_ms, eased_progress }`

| Field | Type | Description |
|---|---|---|
| `id` | `RecipeId` | Recipe identity. |
| `from` | `String` | Previous phase name. |
| `to` | `String` | Next phase name. |
| `t_ms` | `u64` | Elapsed milliseconds since manager start. |
| `eased_progress` | `f64` | Eased progress through the transition (0.0..1.0). |

### `LifecycleDismissed { id, reason, t_ms }`

| Field | Type | Description |
|---|---|---|
| `id` | `RecipeId` | Recipe identity. |
| `reason` | `String` | Dismissal reason (e.g. `"timeout"`, `"user"`). |
| `t_ms` | `u64` | Elapsed milliseconds. |

### `LifecycleHeld { id, until_ms }`

| Field | Type | Description |
|---|---|---|
| `id` | `RecipeId` | Recipe identity. |
| `until_ms` | `u64` | Milliseconds (absolute or elapsed per lifecycle-manager contract) at which the hold releases. |

## 3. Resolution stage — `StageMask::RESOLUTION`

Source: `tui-vfx-recipes::scene` (emitters land in Sub-plan B/C).

### `AssetResolved { name, found, fallback_reason }`

| Field | Type | Description |
|---|---|---|
| `name` | `String` | Recipe-declared asset name. |
| `found` | `bool` | Whether the asset resolved successfully. |
| `fallback_reason` | `Option<String>` | Populated when `found == false` to explain the fallback. |

### `ProceduralResolved { source_id, resolved, fallback_id }`

| Field | Type | Description |
|---|---|---|
| `source_id` | `String` | Recipe-declared procedural source identifier. |
| `resolved` | `bool` | Whether the primary source resolved. |
| `fallback_id` | `Option<String>` | Fallback id used when the primary was missing. |

### `TokenResolved { input, output, missing_keys }`

| Field | Type | Description |
|---|---|---|
| `input` | `String` | Input token reference (e.g. `"{theme}"`). |
| `output` | `String` | Resolved output value. |
| `missing_keys` | `Vec<String>` | Keys that were absent from the resolution context. |

### `RecipeBindingResolved { selector, recipe_id, theme }`

| Field | Type | Description |
|---|---|---|
| `selector` | `String` | Selector string that triggered the binding. |
| `recipe_id` | `RecipeId` | Recipe identity the binding resolved to. |
| `theme` | `String` | Theme in scope for this binding. |

## 4. Composition stage — `StageMask::COMPOSITION`

Source: `tui-vfx-recipes::scene` scene composer (emitters land in Sub-plan B).

### `LayerStarted { layer_id, z, source_kind, target_rect }`

| Field | Type | Description |
|---|---|---|
| `layer_id` | `LayerId` | Opaque layer identity (interned newtype from `tui-vfx-types`). |
| `z` | `i32` | Z-order (higher = on top). |
| `source_kind` | `String` | Content-source kind: `"scene-fragment"`, `"procedural"`, `"static-text"`, `"widget"`, ... |
| `target_rect` | `Rect` | Target rectangle in the destination surface. |

### `LayerCellPainted { layer_id, x, y, glyph, role }`

| Field | Type | Description |
|---|---|---|
| `layer_id` | `LayerId` | Owning layer. |
| `x` | `u16` | Destination x. |
| `y` | `u16` | Destination y. |
| `glyph` | `char` | Glyph written (Unicode scalar). |
| `role` | `RoleTag` | Per-cell semantic role tagged at paint time. |

### `LayerCompleted { layer_id, cells_painted, cells_skipped, fallback }`

| Field | Type | Description |
|---|---|---|
| `layer_id` | `LayerId` | Owning layer. |
| `cells_painted` | `u32` | Count of cells painted. |
| `cells_skipped` | `u32` | Count of cells skipped (out of bounds / masked / empty source). |
| `fallback` | `bool` | Whether a fallback source was used. |

### `LayerSkipped { layer_id, reason }`

| Field | Type | Description |
|---|---|---|
| `layer_id` | `LayerId` | Layer identity. |
| `reason` | `String` | Free-form skip reason. |

## 5. Pipeline stage — `StageMask::PIPELINE`

Source: `tui-vfx-compositor` per-cell pipeline. Reaches the inspection sink via the additive `InspectionSinkBridge` in `crates/tui-vfx-compositor/src/traits/cls_inspection_sink_bridge.rs`, which forwards every `CompositorInspector` callback into the registered `InspectionSink`.

### `SamplerApplied { dest_x, dest_y, src_x, src_y, sampler }`

| Field | Type | Description |
|---|---|---|
| `dest_x` | `u16` | Destination x. |
| `dest_y` | `u16` | Destination y. |
| `src_x` | `Option<u16>` | Source x after sampling (`None` = cell skipped / gap). |
| `src_y` | `Option<u16>` | Source y after sampling (`None` = cell skipped / gap). |
| `sampler` | `String` | Sampler name with unique suffix (e.g. `"ripple#1"`, `"None#1"`). |

### `MaskChecked { x, y, visible, mask }`

| Field | Type | Description |
|---|---|---|
| `x` | `u16` | Cell x. |
| `y` | `u16` | Cell y. |
| `visible` | `bool` | Whether the mask passes the cell through. |
| `mask` | `String` | Mask name with unique suffix (e.g. `"wipe#1"`). |

### `ShaderApplied { x, y, before, after, shader, region }`

| Field | Type | Description |
|---|---|---|
| `x` | `u16` | Cell x. |
| `y` | `u16` | Cell y. |
| `before` | `Style` | Style before the shader ran. |
| `after` | `Style` | Style after the shader ran. |
| `shader` | `String` | Shader name with unique suffix (e.g. `"pulse#1"`). |
| `region` | `Option<String>` | Canonical region name (e.g. `"Border"`, `"Full"`). `None` when the emitter does not know. |

### `FilterApplied { x, y, before, after, filter }`

| Field | Type | Description |
|---|---|---|
| `x` | `u16` | Cell x. |
| `y` | `u16` | Cell y. |
| `before` | `Cell` | Cell before the filter ran. |
| `after` | `Cell` | Cell after the filter ran. |
| `filter` | `String` | Filter name with unique suffix. |

### `ShadowCellApplied { x, y, shadow_cell, source_role, source_empty }`

| Field | Type | Description |
|---|---|---|
| `x` | `u16` | Cell x in the extended shadow area. |
| `y` | `u16` | Cell y in the extended shadow area. |
| `shadow_cell` | `Cell` | Shadow cell as produced by the shadow stage (before final blend/preserve). |
| `source_role` | `Option<RoleTag>` | Role of the corresponding source cell (may be `None` when extrusion is from an empty source cell). |
| `source_empty` | `bool` | Whether the corresponding source cell is empty/missing. |

### `CellRendered { x, y, final_cell }`

| Field | Type | Description |
|---|---|---|
| `x` | `u16` | Cell x. |
| `y` | `u16` | Cell y. |
| `final_cell` | `Cell` | Final cell written to the destination surface. |

## 6. Selectors — `TraceSelector`

Declarative predicate combined by OR inside `TraceFilter`. Each selector matches against the envelope's wrapped event.

| Variant | Matches against | Behaviour |
|---|---|---|
| `Cell { x, y }` | Events carrying `(x, y)` | Exact coordinate match. |
| `Rect(Rect)` | Events carrying `(x, y)` | Half-open `[x, x+width) × [y, y+height)` containment. |
| `Role(RoleTag)` | `LayerCellPainted`, `ShadowCellApplied` | Equality on role tag. |
| `Layer(LayerId)` | `Layer*` events | Content equality on opaque id. |
| `Recipe(RecipeId)` | Envelope's `recipe_id` | Content equality on opaque id. |
| `All` | Every envelope | Always matches. |

Events that do not carry the selected facet simply do not match the selector (they are silently excluded, not errored).

## 7. Filter and stage mask

`TraceFilter` wraps four dimensions:

| Field | Type | Combinator |
|---|---|---|
| `selectors` | `Vec<TraceSelector>` | OR (empty list rejects everything). |
| `stages` | `StageMask` | AND on the event's stage bit. `NONE` rejects everything. |
| `frames` | `Range<u64>` | Half-open range over `frame_no`. |
| `time_ms` | `Range<u64>` | Half-open range over `t_ms`. |

Fast path: `TraceSink::accepts_any_stage()` reads the filter's `stages` and `selectors` without locking the sink. Emitters call this first and skip envelope construction when the filter is inert.

## 8. Determinism

Per spec §9.6:

1. Procedural sources are pure functions of their inputs — no RNG state, no `Instant::now`, no thread locals.
2. `PhaseSnapshot` is deterministically derived from `(t_ms, lifecycle_config, pipeline_config)`.
3. Asset bytes are hashed and recorded (digest only) so replays can detect "same recipe, different bytes."
4. `seq_in_frame` gives a stable replay order across events sharing the same `t_ms`.
5. Trace headers carry tool versions (`tui-vfx-debug` semver + schema version) so old traces remain meaningful or are explicitly flagged stale.

## 9. Versioning

- **0.9.0 (Sub-plan A Phase A.4):** initial schema — 18 variants across 4 stages; envelope with `seq_in_frame`.
- Future additive variants must preserve `#[non_exhaustive]` so client code using wildcard arms continues to compile.

<!-- <FILE>docs/TRACE_EVENT_SCHEMA.md</FILE> - <DESC>Canonical schema reference for TraceEvent + TraceEnvelope</DESC> -->
<!-- <VERS>END OF VERSION: 0.2.0</VERS> -->
