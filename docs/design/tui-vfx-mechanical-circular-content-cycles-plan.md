<!-- <FILE>docs/design/tui-vfx-mechanical-circular-content-cycles-plan.md</FILE> - <DESC>Reviewed implementation plan for shared circular mechanical content cycles powering odometer drums, Solari flap stacks, slot reels, and explicit old/new Pair transitions</DESC> -->
<!-- <VERS>VERSION: 0.2.0</VERS> -->
<!-- <WCTX>Follow-on design after the mechanical display primitives landed: structured Odometer, private grid helpers, and SplitFlap multi-cell tiles exist; this plan adds ordered/circular face routes without regressing Pair old/new behavior.</WCTX> -->
<!-- <CLOG>Critical review update: align with existing code, make Pair the compatibility default, define face-grid route semantics, clarify route-vs-window direction, add concrete Rust/JSON sketches, validation, sequencing, tests, migration notes, file touch list, success criteria, and non-goals.</CLOG> -->

# Mechanical circular content cycles: drums, flap stacks, and reels

## Executive summary

The first mechanical-display tranche is already implemented in `/usr/projects/tui-vfx`:

- `ContentEffect::Odometer` is a structured tile-grid roll effect with `direction`, tagged `travel`, `tile_width`, `tile_height`, and optional `from_message`.
- `crates/tui-vfx-content/src/mechanical/*` contains private grid conversion, pair-roll, center-hinged SplitFlap tile, tile validation, and sizing helpers.
- `SplitFlap` preserves the legacy `1x1` character path and routes non-`1x1` valid even-height tiles through private mechanical tile helpers.
- `docs/CAPABILITIES_REFERENCE.md` already describes Odometer tile roll and SplitFlap `2/4/6/8` Solari tile support.

This plan is therefore the **next layer**, not the original primitive implementation: add shared ordered/circular **content cycles** so existing pair-based mechanisms can traverse physical intermediate faces.

```text
Pair today:      OLD ----------------> NEW
Circular cycle:  8 -> 9 -> 0 -> 1 -> 2
Flap stack:      A -> B -> C -> ... -> Z -> 0 -> ...
Slot reel:       BAR -> 7 -> star -> dollar -> BAR -> target
```

The key compatibility rule is: **`Pair` remains explicit and is the default for existing old/new behavior.** Ordered/circular cycles are opt-in unless a future recipe preset deliberately selects them. This avoids silently turning today's generic Odometer tile roll into a decimal-only odometer.

---

## Critical review findings from existing code

1. **Do not assume the earlier primitive plan is unimplemented.** `fnc_grid_text.rs`, `fnc_roll_grid_window.rs`, `fnc_split_flap_tile_frame.rs`, `types.rs`, `cls_odometer.rs`, and SplitFlap tile tests already exist. Extend these helpers instead of recreating them.
2. **`Odometer` is generic tile roll today, not decimal-only.** A default `decimal_digits` cycle would regress arbitrary text/glyph-grid recipes. The default must be `Pair`; decimal drums are opt-in.
3. **Faces must be grids, not only single chars.** A face may be `"7"`, `"BAR"`, or a multi-line glyph such as `"███\n  █\n███"`. Every face lowers through existing newline-aware grid helpers and is padded/clipped to tile size.
4. **Route direction and window motion are separate.** Existing `OdometerDirection::Up/Down/Left/...` controls visible window motion. New cycle direction (`forward`, `reverse`, `shortest`, `numeric_delta`) controls ordered face traversal. Do not merge them.
5. **SplitFlap legacy semantics are valuable.** The `1x1` path uses `SplitFlapCharset`, `cycles`, `jitter`, `dispersion`, `settle_hinge`, `rolling_flip`, `flip_preview`, and `flip_flicker`. New `mechanical` config must be opt-in and must not change output when absent.
6. **The current SplitFlap Alpha pool is exact.** It is `space + A-Z + 0-9 + '.', ',', '-', '!', '?'`; it does **not** include `/`. Recipes needing extra punctuation must use `ordered` faces or a new documented preset.
7. **Runtime no-op is not enough validation.** Current invalid SplitFlap tile sizes return target unchanged. Recipe validation must still reject invalid cycle configs clearly.
8. **No new dependency should be required.** Randomized/weighted reels can use deterministic hashing/LCG helpers derived from the existing FNV approach. Do not add `rand` without explicit approval.

---

## Goals

1. Add a shared schema-bearing `MechanicalCycleConfig` usable by Odometer, SplitFlap/Solari, and future slot-reel-like effects.
2. Preserve explicit old/new-only `Pair` mode and make it the compatibility default where behavior already exists.
3. Allow recipes to declare ordered faces, circular/bounded wrap behavior, route direction, tie-breaking, missing-face policy, extra rotations, cascade, and settle behavior.
4. Represent each face as a normalized tile grid so single-cell and multi-cell mechanisms share one route builder.
5. Migrate Odometer first because it already uses private mechanical pair-roll helpers and has lower legacy risk than SplitFlap.
6. Add SplitFlap/Solari adoption only after Odometer proves route/cascade mechanics.
7. Keep `TextTransformer` unchanged: public transformers still accept/return strings and may use `OwnedGrid` internally.
8. Make errors and tests concrete enough for a junior developer to implement safely.

## Known non-goals

- Do not modify production code as part of this document-review task.
- Do not remove or rename `OdometerDirection` / `OdometerTravel` in this slice.
- Do not make `{ "type": "odometer" }` valid again; structured Odometer remains required.
- Do not route all SplitFlap recipes through the new cycle path. Legacy `1x1` SplitFlap remains the default when `mechanical` is absent.
- Do not implement styled/SemanticScene face cycles yet. Strings lower to character-cell grids; style preservation is future work.
- Do not implement perspective-correct 3D Solari rendering. The current terminal-native hinge/grid model remains the visual substrate.
- Do not add broad public `MechanicalDisplay` effects until multiple public consumers prove the need.
- Do not add nondeterministic runtime randomness. Reels must be deterministic from recipe fields and input coordinates.

---

## Existing implementation anchors

### Current private mechanical module

```text
crates/tui-vfx-content/src/mechanical/
├── fnc_grid_text.rs              # grid_from_text, grid_to_text, paired_grids
├── fnc_roll_grid_window.rs       # old/new fixed-window pair roll
├── fnc_split_flap_tile_frame.rs  # center-hinged multi-cell tile frames
├── mod.rs
└── types.rs                      # MechanicalSource, MechanicalTile, sizing, validation
```

Cycle work should add new helpers beside these files. Avoid growing `cls_split_flap.rs` further.

### Current public schema facts

```rust
pub enum OdometerDirection {
    Up, Down, Left, Right, UpLeft, UpRight, DownLeft, DownRight,
}

#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum OdometerTravel {
    Axis,
    FullClear,
    Cells { cells: u16 },
}

ContentEffect::Odometer {
    direction: OdometerDirection,
    travel: OdometerTravel,
    tile_width: u16,
    tile_height: u16,
    from_message: Option<String>,
}

ContentEffect::SplitFlap {
    // legacy fields...
    dispersion: SplitFlapDispersion,
    tile_width: u16,
    tile_height: u16,
}
```

Any new `ContentEffect` field is public schema-bearing surface and needs serde, `ConfigSchema`, rustdoc, docs/schema metadata, and tooling awareness.

---

# Core model

## Vocabulary

| Term | Meaning |
| --- | --- |
| **Face** | One visible content value: digit, glyph, symbol, word, or multi-cell card. |
| **Face grid** | A face normalized into an `OwnedGrid` compatible with the mechanism tile. |
| **Cycle** | Ordered collection of faces plus wrap and lookup semantics. |
| **Route** | Concrete ordered list of face grids sampled between source and target, including endpoints. |
| **Pair** | Explicit old/new-only route `[from, to]`; no intermediate faces. |
| **Drum** | Odometer-like ordered cycle, commonly decimal and circular. |
| **Flap stack** | Solari/SplitFlap ordered cycle, commonly alphanumeric and forward-only. |
| **Reel** | Slot-machine-like cycle; may be weighted, shuffled, and include extra rotations. |
| **Window motion** | Existing visual sampling direction through a viewport. |
| **Route direction** | Cycle index traversal direction: forward, reverse, shortest, numeric-derived. |

## Public config sketches

Put schema-bearing types in `crates/tui-vfx-content/src/types/` or another public module re-exported by `types/mod.rs` if they appear inside `ContentEffect`. Internal resolved structs can stay in `mechanical`.

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct MechanicalCycleConfig {
    /// What faces exist between source and target. Default Pair preserves current behavior.
    #[serde(default)]
    pub source: MechanicalContentSource,

    /// How to choose a route through the source cycle.
    #[serde(default)]
    pub route: MechanicalRouteConfig,

    /// How multiple tiles/cells are scheduled relative to each other.
    #[serde(default)]
    pub cascade: MechanicalCascadePolicy,

    /// Optional post-route timing remap. First implementation may parse this before every visual mechanism uses it.
    #[serde(default)]
    pub settle: MechanicalSettleConfig,
}

impl Default for MechanicalCycleConfig {
    fn default() -> Self {
        Self {
            source: MechanicalContentSource::Pair,
            route: MechanicalRouteConfig::default(),
            cascade: MechanicalCascadePolicy::Simultaneous,
            settle: MechanicalSettleConfig::None,
        }
    }
}
```

### Content source

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ConfigSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum MechanicalContentSource {
    /// Direct old/new exchange. No intermediate ordered content exists.
    Pair,

    /// Ordered list of authored face strings. Strings are parsed with newline-aware grid rules.
    Ordered {
        faces: Vec<String>,
        #[serde(default)]
        wrap: CycleWrapMode,
    },

    /// Named preset expanded by tui-vfx. Presets must be documented and tested.
    Preset {
        preset: MechanicalCyclePreset,
        #[serde(default)]
        wrap: CycleWrapMode,
    },

    /// Same authored faces, shuffled deterministically once from seed.
    Randomized {
        faces: Vec<String>,
        seed: u64,
        #[serde(default)]
        wrap: CycleWrapMode,
    },

    /// Weighted reel source. Resolve by cumulative weights or capped expansion; do not allocate huge vectors.
    Weighted {
        faces: Vec<WeightedCycleFace>,
        seed: u64,
        #[serde(default)]
        wrap: CycleWrapMode,
    },
}

impl Default for MechanicalContentSource {
    fn default() -> Self { Self::Pair }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, ConfigSchema)]
#[serde(rename_all = "snake_case")]
pub enum CycleWrapMode {
    #[default]
    Circular,
    Bounded,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct WeightedCycleFace {
    pub value: String,
    pub weight: u16,
}
```

### Presets

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ConfigSchema)]
#[serde(rename_all = "snake_case")]
pub enum MechanicalCyclePreset {
    /// "0" through "9".
    DecimalDigits,
    /// Current SplitFlapCharset::Alpha exactly: space, A-Z, 0-9, '.', ',', '-', '!', '?'.
    SplitFlapAlpha,
    /// Current SplitFlapCharset::Digits exactly: space, 0-9.
    SplitFlapDigits,
    /// Current SplitFlapCharset::Uppercase exactly: space, A-Z.
    SplitFlapUppercase,
}
```

If `solari_airport` is added later, define its exact face list in docs and tests. Do not silently change `split_flap_alpha`.

### Route config

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct MechanicalRouteConfig {
    #[serde(default)]
    pub direction: CycleDirectionPolicy,
    #[serde(default)]
    pub tie_breaker: CycleTieBreaker,
    /// Full additional wraps before landing. Slot recipes commonly use 2+.
    #[serde(default)]
    pub extra_rotations: u16,
    #[serde(default)]
    pub missing_face: CycleMissingFacePolicy,
}

impl Default for MechanicalRouteConfig {
    fn default() -> Self {
        Self {
            direction: CycleDirectionPolicy::Forward,
            tie_breaker: CycleTieBreaker::Forward,
            extra_rotations: 0,
            missing_face: CycleMissingFacePolicy::Error,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, ConfigSchema)]
#[serde(rename_all = "snake_case")]
pub enum CycleDirectionPolicy {
    #[default]
    Forward,
    Reverse,
    Shortest,
    NumericDelta,
    Authored,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, ConfigSchema)]
#[serde(rename_all = "snake_case")]
pub enum CycleTieBreaker {
    #[default]
    Forward,
    Reverse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, ConfigSchema)]
#[serde(rename_all = "snake_case")]
pub enum CycleMissingFacePolicy {
    #[default]
    Error,
    PairFallback,
    InsertAtEnd,
}
```

Do not add `blank` missing-face fallback in the first implementation; it is easy to confuse with blank source padding.

### Cascade and settle

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ConfigSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum MechanicalCascadePolicy {
    Simultaneous,
    Staggered { fraction: f32 },
    NumericCarry {
        #[serde(default = "default_stagger_fraction")]
        stagger_fraction: f32,
        #[serde(default)]
        unchanged: UnchangedCellPolicy,
    },
    Randomized { seed: u64, max_delay_fraction: f32 },
}

impl Default for MechanicalCascadePolicy {
    fn default() -> Self { Self::Simultaneous }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, ConfigSchema)]
#[serde(rename_all = "snake_case")]
pub enum UnchangedCellPolicy {
    #[default]
    Hold,
    SpinAndReturn,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, ConfigSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum MechanicalSettleConfig {
    None,
    Spring { overshoot: f32, settle_fraction: f32 },
    Ease { easing: EasingCurveName },
}

impl Default for MechanicalSettleConfig {
    fn default() -> Self { Self::None }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ConfigSchema)]
#[serde(rename_all = "snake_case")]
pub enum EasingCurveName {
    Linear,
    EaseOut,
    EaseOutBack,
}
```

If existing easing types are not schema-friendly in this crate, use a small local enum first. Do not pull a broad easing dependency into content.

---

# Internal resolved model

Resolved structs can remain `pub(crate)` in `crates/tui-vfx-content/src/mechanical`.

```rust
pub(crate) struct ResolvedMechanicalFace {
    pub(crate) value: String,
    pub(crate) grid: OwnedGrid,
}

pub(crate) struct ResolvedMechanicalCycle {
    pub(crate) faces: Vec<ResolvedMechanicalFace>,
    pub(crate) wrap: CycleWrapMode,
}

pub(crate) struct MechanicalCycleRoute {
    /// Always includes source and target endpoints.
    pub(crate) faces: Vec<ResolvedMechanicalFace>,
    pub(crate) selected_direction: CycleDirectionPolicy,
}

pub(crate) struct TileCycleContext<'a> {
    pub(crate) from_face: &'a str,
    pub(crate) to_face: &'a str,
    pub(crate) tile_col: usize,
    pub(crate) tile_row: usize,
    pub(crate) tile_linear: usize,
    pub(crate) tile_width: u16,
    pub(crate) tile_height: u16,
}
```

## Face normalization rules

1. Parse each face string using the existing newline-aware grid helper (`split('\n')`; newline is structure, not visible content).
2. Reject empty face strings for ordered/randomized/weighted sources unless a named preset deliberately includes blank. The blank face should normally be a single space string.
3. Pad faces smaller than the tile rectangle with spaces.
4. Reject faces larger than `tile_width x tile_height` unless an explicit future `face_overflow` policy is added. Silent clipping hides authoring mistakes.
5. Preserve exact target output at `progress >= 1.0` by keeping the existing early return in transformers.
6. For partial edge tiles, pad source/target tile extraction to full tile size; blit only cells inside the output viewport.

---

# Route semantics

## Pair mode

`Pair` always builds exactly `[from, to]`. This is the default and must keep current Odometer pair-roll behavior and SplitFlap behavior when no `mechanical` field is present.

```json
"mechanical": {
  "source": { "type": "pair" }
}
```

## Ordered circular mode

Given cycle `[0,1,2,3,4,5,6,7,8,9]`:

| From | To | Forward route | Reverse route | Shortest route |
| --- | --- | --- | --- | --- |
| `8` | `2` | `8,9,0,1,2` | `8,7,6,5,4,3,2` | forward |
| `2` | `8` | `2,3,4,5,6,7,8` | `2,1,0,9,8` | reverse |
| `9` | `0` | `9,0` | `9,8,7,6,5,4,3,2,1,0` | forward |
| `0` | `9` | `0,1,2,3,4,5,6,7,8,9` | `0,9` | reverse |

Reference helper shape:

```rust
pub(crate) fn route_between(
    cycle: &ResolvedMechanicalCycle,
    from: &str,
    to: &str,
    route: MechanicalRouteConfig,
) -> Result<MechanicalCycleRoute, MechanicalCycleError>;
```

For `extra_rotations`, append complete wraps in the selected direction before the final endpoint. Example, decimal forward `8 -> 2` with `extra_rotations: 1`:

```text
8,9,0,1,2,3,4,5,6,7,8,9,0,1,2
```

Bounded cycles reject routes that would need to move past either end unless `PairFallback` or `InsertAtEnd` explicitly permits recovery.

## Numeric carry policy

`NumericDelta` is not a synonym for `shortest`. It uses structured numeric context when available:

- `099 -> 100`: changed tiles route forward.
- `100 -> 099`: changed tiles route reverse.
- `198 -> 199`: unchanged leading tiles hold; ones route forward.
- `0 -> 9` during decrement uses reverse.
- `9 -> 0` during increment uses forward.

First-slice constraints:

1. Only apply `NumericDelta`/`NumericCarry` to decimal digit faces from `decimal_digits` or an equivalent ordered source whose values are exactly `"0".."9"`.
2. Source and target numeric strings must have the same tile count after padding. If not, fail validation unless `missing_face: pair_fallback` is set.
3. For non-adjacent jumps such as `190 -> 200`, changed digits may route independently according to overall numeric sign; do not claim exact odometer carry physics for skipped intermediate values.
4. `NumericCarry` schedules changed suffix tiles; unchanged tiles follow `UnchangedCellPolicy`.

## Route direction vs window motion

Cycle route order does not automatically change `OdometerDirection`.

- `direction: "up"` still means old visible content exits upward and next route face enters from below.
- `mechanical.route.direction: "reverse"` means the next route face is the previous face in the ordered cycle.
- If an author wants decrement to visibly roll downward, they should set `direction: "down"` in this slice.

A future field such as `motion_binding: "follow_route_direction"` may be added after recipes prove the need. Do not implement hidden auto-flipping in the first pass.

---

# Sampling algorithm

## Tile-cycle renderer

The existing `roll_grid_window` samples one old/new grid pair. The cycle renderer should reuse it segment-by-segment rather than building an unrelated sampler.

```rust
pub(crate) fn roll_cycle_window(
    route: &MechanicalCycleRoute,
    progress: f64,
    direction: OdometerDirection,
    travel: OdometerTravel,
    tile: MechanicalTile,
) -> OwnedGrid {
    if progress <= 0.0 { return route.faces[0].grid.clone(); }
    if progress >= 1.0 { return route.faces.last().unwrap().grid.clone(); }

    let last_segment = route.faces.len().saturating_sub(1).max(1);
    let scaled = progress.clamp(0.0, 1.0) * last_segment as f64;
    let segment = scaled.floor().min((last_segment - 1) as f64) as usize;
    let local = scaled - segment as f64;

    let pair = MechanicalSource {
        from: route.faces[segment].grid.clone(),
        to: route.faces[segment + 1].grid.clone(),
    };
    roll_grid_window(&pair, local, direction, travel, tile)
}
```

This keeps motion semantics consistent with current Odometer and makes route-building independently testable.

## Odometer whole-grid compatibility path

To preserve current behavior:

- If `mechanical.source == Pair` and `mechanical.cascade == Simultaneous`, keep using the current whole-grid `roll_grid_window` path.
- If source is ordered/randomized/weighted/preset, segment the source/target into tile rectangles, build one route per tile, sample each route, and blit sampled tile grids into an output grid.

## SplitFlap cycle path

- If `mechanical` is absent: run current `SplitFlap` code unchanged.
- If `mechanical` is present and tile size is `1x1`: use route-selected intermediate faces as the character pool source, but preserve existing speed/cascade/jitter/dispersion/hinge visual phases where possible.
- If `mechanical` is present and tile height is `2/4/6/8`: build tile routes and feed adjacent route faces into `split_flap_tile_frame` for the local tile phase.
- Reject ambiguous configs where both legacy `cycles` and `mechanical.route.extra_rotations` are non-zero for SplitFlap. Avoid double-spin surprises.

---

# Recipe JSON examples

## 1. Explicit Pair mode (current old/new-only behavior)

```json
{
  "type": "odometer",
  "direction": "up",
  "travel": { "type": "axis" },
  "tile_width": 1,
  "tile_height": 3,
  "from_message": "AAA\nBBB\nCCC",
  "mechanical": {
    "source": { "type": "pair" }
  }
}
```

Expected: exactly today's pair roll frames such as `BBB\nCCC\n111` around one-third progress.

## 2. Decimal odometer increment with carry

```json
{
  "type": "odometer",
  "direction": "up",
  "travel": { "type": "cells", "cells": 1 },
  "tile_width": 1,
  "tile_height": 1,
  "from_message": "099",
  "mechanical": {
    "source": { "type": "preset", "preset": "decimal_digits" },
    "route": { "direction": "numeric_delta", "tie_breaker": "forward" },
    "cascade": {
      "type": "numeric_carry",
      "stagger_fraction": 0.35,
      "unchanged": "hold"
    },
    "settle": { "type": "spring", "overshoot": 0.12, "settle_fraction": 0.18 }
  }
}
```

Target message: `100`.

## 3. Multi-cell typography drum faces

```json
{
  "type": "odometer",
  "direction": "up",
  "travel": { "type": "axis" },
  "tile_width": 3,
  "tile_height": 3,
  "from_message": "███\n█ █\n███",
  "mechanical": {
    "source": {
      "type": "ordered",
      "wrap": "circular",
      "faces": [
        "███\n█ █\n███",
        "  █\n  █\n  █",
        "███\n  █\n███"
      ]
    },
    "route": { "direction": "forward" }
  }
}
```

Target message is the face for `2`. Validation should ensure each face fits `3x3`.

## 4. SplitFlap with current Alpha stack

```json
{
  "type": "split_flap",
  "from_message": "GATE 12",
  "speed": 1.0,
  "cascade": 0.18,
  "jitter": 0.1,
  "settle_hinge": true,
  "rolling_flip": true,
  "dispersion": "cascade",
  "tile_width": 1,
  "tile_height": 1,
  "mechanical": {
    "source": { "type": "preset", "preset": "split_flap_alpha" },
    "route": { "direction": "forward" },
    "cascade": { "type": "staggered", "fraction": 0.18 }
  }
}
```

If the recipe needs `/`, use `ordered` and include `/`; do not expect `split_flap_alpha` to contain it.

## 5. Multi-cell Solari cards

```json
{
  "type": "split_flap",
  "from_message": "OLD OLD\nOLD OLD\nOLD OLD\nOLD OLD",
  "speed": 1.0,
  "cascade": 0.2,
  "jitter": 0.05,
  "settle_hinge": true,
  "spring_settle": true,
  "dispersion": "center_out",
  "tile_width": 3,
  "tile_height": 4,
  "mechanical": {
    "source": {
      "type": "ordered",
      "faces": ["OLD\nOLD\nOLD\nOLD", "NEW\nNEW\nNEW\nNEW", "ETA\nETA\nETA\nETA"],
      "wrap": "circular"
    },
    "route": { "direction": "shortest", "tie_breaker": "forward" }
  }
}
```

## 6. Slot reel using weighted source

```json
{
  "type": "odometer",
  "direction": "up",
  "travel": { "type": "axis" },
  "tile_width": 3,
  "tile_height": 1,
  "from_message": "BAR",
  "mechanical": {
    "source": {
      "type": "weighted",
      "seed": 777,
      "wrap": "circular",
      "faces": [
        { "value": "7", "weight": 1 },
        { "value": "$", "weight": 2 },
        { "value": "★", "weight": 3 },
        { "value": "BAR", "weight": 1 }
      ]
    },
    "route": { "direction": "forward", "extra_rotations": 2 },
    "cascade": { "type": "staggered", "fraction": 0.33 },
    "settle": { "type": "spring", "overshoot": 0.2, "settle_fraction": 0.25 }
  }
}
```

---

# Validation rules

Add explicit runtime/recipe validation beyond generated `ConfigSchema`.

## Source validation

1. `ordered.faces`, `randomized.faces`, and `weighted.faces` must be non-empty.
2. `weighted.faces[*].weight` must be `> 0`.
3. Weighted total must fit in `u32`; reject overflow.
4. Duplicate face values are rejected for `ordered` and `randomized` by default.
5. Duplicate weighted values are rejected too; authors should combine weights into one entry.
6. `circular` cycles require at least two distinct faces.
7. All non-pair face grids must fit inside the mechanism tile size.
8. Preset expansion must be exact and tested.

## Route validation

1. Source and target endpoints must exist in the resolved cycle unless `missing_face` explicitly permits recovery.
2. `bounded` cycles reject routes that would move outside endpoints.
3. `shortest` requires `wrap: circular`; otherwise reject or behave as bounded direct path only if endpoints are ordered correctly.
4. `extra_rotations > 0` requires `wrap: circular`.
5. `numeric_delta` requires decimal digit faces and parseable source/target numeric tile values.
6. `authored` is reserved; reject in recipes until an override source exists.

## Cascade/settle validation

1. `staggered.fraction` and `numeric_carry.stagger_fraction` must be `0.0..=0.95`.
2. `randomized.max_delay_fraction` must be `0.0..=0.95`.
3. `settle.spring.overshoot` must be `0.0..=0.5`.
4. `settle.spring.settle_fraction` must be `0.0..=1.0` and should be non-zero when spring is used.
5. If a visual mechanism cannot honor `settle` yet, parsing is allowed but docs/tests must state it is currently accepted but inert for that mechanism.

## Existing mechanism validation remains

1. Odometer `tile_width` and `tile_height` must be non-zero.
2. `OdometerTravel::Cells { cells }` should reject `cells == 0` in validator paths.
3. SplitFlap `1x1` preserves legacy behavior.
4. SplitFlap multi-cell center hinge accepts even heights `2`, `4`, `6`, and `8` only.
5. SplitFlap `tile_width > 1 && tile_height == 1` remains invalid in this tranche unless a new recipe justifies it.

---

# Implementation sequencing

## Phase 0 — lock current behavior before cycle changes

Run and keep passing:

```bash
cargo test -p tui-vfx-content --test test_cls_odometer
cargo test -p tui-vfx-content --test test_cls_split_flap_tiles
cargo test -p tui-vfx-content cls_split_flap
```

Add a focused regression if needed proving `mechanical` absent keeps current outputs.

## Phase 1 — public config types and validation

Files:

- `crates/tui-vfx-content/src/types/cls_content_effect.rs`
- `crates/tui-vfx-content/src/types/mod.rs`
- new `crates/tui-vfx-content/src/types/cls_mechanical_cycle_config.rs` or equivalent
- `crates/tui-vfx-content/src/transformers/fnc_get_transformer.rs`

Tasks:

1. Add schema-bearing mechanical cycle types.
2. Add `#[serde(default, skip_serializing_if = "MechanicalCycleConfig::is_default")] mechanical: MechanicalCycleConfig` to Odometer.
3. Prefer `Option<MechanicalCycleConfig>` for SplitFlap so absent means legacy. If a non-optional default is used, ensure runtime can distinguish absence from explicit Pair only if behavior needs that distinction.
4. Add rustdoc for every new public field/type.
5. Add serde tests for Pair default, ordered config, unknown-field rejection, and invalid shapes.

## Phase 2 — route resolution helpers

Add files under `crates/tui-vfx-content/src/mechanical/`:

```text
cls_resolved_cycle.rs
enum_cycle_error.rs
fnc_expand_cycle_preset.rs
fnc_normalize_cycle_face.rs
fnc_resolve_mechanical_cycle.rs
fnc_route_between.rs
fnc_weighted_cycle_order.rs
```

Tasks:

1. Resolve `Pair`, `Ordered`, `Preset`, `Randomized`, and `Weighted` into `ResolvedMechanicalCycle`.
2. Normalize face grids against `MechanicalTile`.
3. Implement forward/reverse/shortest route construction.
4. Implement missing-face policies.
5. Add route unit tests independent of Odometer/SplitFlap.

Required tests:

- decimal forward `8 -> 2` gives `8,9,0,1,2`.
- decimal reverse `2 -> 8` gives `2,1,0,9,8`.
- shortest chooses smaller path and tie-breaker is deterministic.
- pair mode returns `[from,to]`.
- missing face errors by default and pair-falls-back when requested.
- multi-line `3x3` face normalization rejects oversized faces and pads smaller faces.
- `split_flap_alpha` preset equals current code's pool exactly.

## Phase 3 — Odometer cycle rendering

Files:

- `crates/tui-vfx-content/src/transformers/cls_odometer.rs`
- `crates/tui-vfx-content/src/mechanical/fnc_roll_cycle_window.rs`
- possibly `crates/tui-vfx-content/src/mechanical/fnc_tile_rects.rs`

Tasks:

1. Keep existing whole-grid Pair path unchanged.
2. For non-Pair source, segment source/target into tile rects.
3. Resolve a route per tile.
4. Apply cascade to produce tile-local progress.
5. Sample each route with `roll_cycle_window` and blit into output grid.
6. Keep `progress >= 1.0` returning `Cow::Borrowed(target)`.

Required Odometer tests:

- current pair-mode row/column/diagonal tests still pass unchanged.
- explicit Pair config matches absent/default Pair.
- decimal `099 -> 100` routes changed digits forward.
- decimal `100 -> 099` routes changed digits reverse.
- unchanged digits hold under `NumericCarry { unchanged: Hold }`.
- `SpinAndReturn` spins an unchanged tile and still lands on target.
- `extra_rotations` increases intermediate route length and still lands exactly.
- bounded cycle rejects impossible reverse/forward routes.

## Phase 4 — SplitFlap/Solari adoption

Files:

- `crates/tui-vfx-content/src/transformers/cls_split_flap.rs`
- `crates/tui-vfx-content/tests/transformers/test_cls_split_flap_tiles.rs`
- new focused cycle tests if large enough to avoid growing `cls_split_flap.rs` tests further

Tasks:

1. If `mechanical` absent, run current code unchanged.
2. If `mechanical` present with `1x1`, route through ordered face stacks while preserving existing visual phases.
3. If `mechanical` present with `2/4/6/8` tile height, build per-tile routes and feed adjacent route faces into the center-hinge helper.
4. Reject ambiguous double-spin configs (`cycles` and `mechanical.route.extra_rotations` both non-zero) until semantics are explicitly defined.
5. Preserve `from_message` grid parsing and newline handling.

Required SplitFlap tests:

- absent `mechanical` preserves existing `1x1` snapshots.
- explicit Pair in `mechanical` matches current old/new tile behavior.
- ordered alphabet cycle yields expected intermediate face sequence.
- unknown char with `missing_face: pair_fallback` preserves legacy-ish fallback.
- strict missing face errors in validator tests.
- multi-cell Solari route settles exactly on target.
- invalid tile sizes remain rejected/no-op at transformer layer and rejected by validator layer.

## Phase 5 — docs, schema, recipes, tooling

Files/surfaces:

- `docs/CAPABILITIES_REFERENCE.md`
- `docs/templates/capabilities.toml`
- `CAPABILITIES.md`
- `docs/generated/*`
- `xtask/src/docs/effect_metadata.rs`
- any schema tests / `docs/generated/effect_schemas.json`
- `/usr/projects/tui-vfx-recipes` DTO/schema/validator/player/debug recipes if recipe support lands in same tranche

Tasks:

1. Update rustdoc and hand docs with `mechanical` config examples.
2. Regenerate docs/schema where required.
3. Add debug recipes for Pair, decimal forward, decimal borrow, staggered spring carry, SplitFlap alpha stack, multi-cell Solari, and weighted slot reel.
4. Update validators so new fields are preserved and invalid configs fail clearly.

Commands to discover/run as applicable:

```bash
# /usr/projects/tui-vfx
cargo fmt
cargo test -p tui-vfx-content
cargo test
just docs-all
just docs-all-check
just docs-all-validate
just check-all

# /usr/projects/tui-vfx-recipes, if recipes/tooling are updated
just --list
just fmt-check
cargo test --test test_debug_recipes_qc
cargo test -p pipeline-validator --test test_debug_recipes_qc
just docs-v3-check
just v3-headless-smoke
just check
```

If a command name differs locally, record the actual command in the implementation report rather than claiming an unrun gate.

---

# Junior developer file touch list

| File | Expected change |
| --- | --- |
| `crates/tui-vfx-content/src/types/cls_content_effect.rs` | Add `mechanical` fields to Odometer/SplitFlap, rustdoc, key parameters, defaults. |
| `crates/tui-vfx-content/src/types/mod.rs` | Re-export public mechanical cycle config types. |
| `crates/tui-vfx-content/src/types/cls_mechanical_cycle_config.rs` | New schema-bearing config/source/route/cascade/settle types, if split out. |
| `crates/tui-vfx-content/src/mechanical/mod.rs` | Register new cycle helper modules. |
| `crates/tui-vfx-content/src/mechanical/types.rs` | Add internal route/tile context types only if cohesive; otherwise split. |
| `crates/tui-vfx-content/src/mechanical/fnc_resolve_mechanical_cycle.rs` | Resolve source config to face grids. |
| `crates/tui-vfx-content/src/mechanical/fnc_route_between.rs` | Build routes through ordered/circular cycles. |
| `crates/tui-vfx-content/src/mechanical/fnc_roll_cycle_window.rs` | Segment-by-segment cycle sampling via existing `roll_grid_window`. |
| `crates/tui-vfx-content/src/mechanical/fnc_tile_rects.rs` | Shared tile rectangle iteration/blitting if needed. |
| `crates/tui-vfx-content/src/transformers/cls_odometer.rs` | Add optional cycle rendering path; keep Pair path unchanged. |
| `crates/tui-vfx-content/src/transformers/cls_split_flap.rs` | Add opt-in mechanical cycle path without changing absent-mechanical path. |
| `crates/tui-vfx-content/src/transformers/fnc_get_transformer.rs` | Pass mechanical config to transformer constructors/builders. |
| `crates/tui-vfx-content/tests/transformers/test_cls_odometer.rs` | Add cycle route/carry rendering tests. |
| `crates/tui-vfx-content/tests/transformers/test_cls_split_flap_tiles.rs` | Add opt-in cycle/Solari stack tests. |
| `xtask/src/docs/effect_metadata.rs` | Add sample `mechanical` config for generated docs if schema examples need it. |
| `docs/CAPABILITIES_REFERENCE.md` | Document Pair vs ordered cycles, presets, examples, validation. |
| `docs/templates/capabilities.toml` | Update generated capabilities inputs. |
| `/usr/projects/tui-vfx-recipes/...` | Update schema/validator/player/debug recipes if implementation includes recipe support. |

OFPF guidance: keep helper files small and single-purpose. Do not add more large inline logic to `cls_split_flap.rs`; route to mechanical helpers.

---

# Migration notes

## Existing Odometer recipes

Current structured Odometer recipes continue to mean Pair old/new roll because `mechanical` defaults to Pair.

Before:

```json
{
  "type": "odometer",
  "direction": "up",
  "travel": { "type": "axis" },
  "tile_width": 1,
  "tile_height": 3,
  "from_message": "AAA\nBBB\nCCC"
}
```

After: same behavior. Authors only add `mechanical` when they want ordered intermediate faces.

## Existing SplitFlap recipes

No migration required when `mechanical` is absent. Existing fields keep existing meaning.

If adding `mechanical`, document that source/cycle config owns intermediate face order. Avoid setting both legacy `cycles` and `mechanical.route.extra_rotations` until implementation defines/rejects that combination.

## Recipe docs and generated schema

Generated docs must show:

- `mechanical.source.type: pair|ordered|preset|randomized|weighted`
- tagged `travel` remains unchanged for Odometer
- named presets and exact face lists
- validation notes for missing faces, tile dimensions, duplicate faces, and weighted values

---

# Success criteria

Implementation is complete only when all are true:

1. Current Odometer pair-roll tests pass unchanged.
2. Current SplitFlap legacy and tile tests pass unchanged when `mechanical` is absent.
3. Pair mode is explicit, documented, and tested.
4. Ordered decimal routes work forward, reverse, shortest, and with deterministic tie-breaking.
5. Numeric carry/borrow examples land correctly and unchanged tiles hold by default.
6. Multi-line/multi-cell face grids are normalized, validated, sampled, and blitted correctly.
7. SplitFlap ordered stacks are opt-in and do not alter legacy recipes.
8. Weighted/randomized reels are deterministic from seed and validated without new dependencies.
9. Docs/schema/tooling describe the new fields and reject invalid recipe configs.
10. Final verification includes content tests, docs/schema checks, and recipe validator/player checks if recipe files are changed.

---

# Remaining risks and open decisions

1. **Visual direction for decrement.** This plan keeps route direction separate from `OdometerDirection`. Some authors may expect decrement to auto-roll downward. Defer automatic motion binding until recipes prove the need.
2. **SplitFlap cycles overlap.** Existing `cycles` and new `extra_rotations` can double-count. First implementation should reject ambiguous combinations or document exact precedence before enabling both.
3. **Weighted route semantics.** Weighted reels can mean expanded ordered strip or weighted random sequence. This plan prefers deterministic cumulative/virtual expansion but needs tests to freeze exact order.
4. **Validation error channel.** Transformers currently often fall back to target/no-op. Recipe validators must be the user-facing strict surface; runtime transformers can stay defensive.
5. **Large face grids.** Multi-cell face routes can allocate many `OwnedGrid`s. Cache resolved preset faces per transform call and avoid cloning more than necessary, but do not introduce global mutable caches in the first slice.
6. **Graphemes and style.** Current helpers are `char`/cell based. Unicode grapheme clusters and styled cells remain future work.
7. **Tooling in sibling repo.** `/usr/projects/tui-vfx-recipes` may have additional DTO/player assumptions. Discover with tests rather than assuming pass-through preserves new fields.

---

# Recommended direction

Implement this as a shared mechanical content-cycle substrate layered on the existing private mechanical module. Keep `Pair` mode explicit and default so old/new-only animation remains available and existing recipes do not change. Treat carry as cascade/scheduling plus route-direction selection, not as window motion. Prove the route builder and Odometer rendering first, then carefully integrate SplitFlap/Solari behind an opt-in `mechanical` field.

This gives `tui-vfx` one reusable primitive family for ordered drums, flap stacks, and reels while keeping recipe JSON as the source of mechanical truth.

<!-- <FILE>docs/design/tui-vfx-mechanical-circular-content-cycles-plan.md</FILE> - <DESC>Reviewed implementation plan for shared circular mechanical content cycles powering odometer drums, Solari flap stacks, slot reels, and explicit old/new Pair transitions</DESC> -->
<!-- <VERS>END OF VERSION: 0.2.0</VERS> -->
