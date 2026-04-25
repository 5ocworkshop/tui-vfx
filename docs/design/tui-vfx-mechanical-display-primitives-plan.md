<!-- <FILE>docs/design/tui-vfx-mechanical-display-primitives-plan.md</FILE> - <DESC>Design plan for shared mechanical display primitives powering Odometer grid roll and multi-cell SplitFlap/Solari effects</DESC> -->
<!-- <VERS>VERSION: 0.3.0</VERS> -->
<!-- <WCTX>Plan a shared grid-first mechanical-display substrate so the existing Odometer namespace can become useful without regressing the mature single-cell SplitFlap/Solari experience.</WCTX> -->
<!-- <CLOG>Accept odometer replacement direction: remove legacy/default compatibility requirements and make tile roll the primary odometer model.</CLOG> -->

# Mechanical display primitives: Odometer grid roll and multi-cell Solari

## Context and why this exists

`tui-vfx` already has two mechanical-display content effects:

- `SplitFlap`, which is mature: it models Solari-style character boards with
  charset selection, `from_message`, cycles, jitter, hinge frames, spring settle,
  rolling cards, flicker, and per-column dispersion.
- `Odometer`, which exists by name but is currently a shallow single-line digit
  counter. It does not deliver the mechanical wheel behavior implied by its name.

GT-Design's typography work exposed the gap. A 3-cell-tall glyph can roll like a
real odometer: old rows leave the fixed window while new rows enter from the
opposite edge. The same conversation also revealed a broader need: single-cell
SplitFlap is useful, but a real Solari board should support larger even-height
cards (`2`, `4`, `6`, `8` rows) hinged at the center.

This plan keeps the project aligned with the steering goals:

- **Grid-first, ecosystem-agnostic.** The primitive operates on `tui-vfx` cell
  grids, not ratatui buffers or GTD typography types.
- **Recipe-authoring truth lives here.** Downstream consumers should not
  reimplement odometer or Solari semantics in app code.
- **Schema regularity matters.** Odometer and SplitFlap should share mechanical
  vocabulary where they share behavior: source content, tile geometry,
  scheduling, dispersion, cycles, jitter, and blank handling.
- **Additive changes must earn their place.** We improve the mostly-unused
  Odometer namespace and extend the working SplitFlap capability without
  breaking existing single-cell SplitFlap recipes. The existing `Odometer` behavior is intentionally replaceable: it is not an important consumer-facing contract and should be rebuilt around the mechanical roll model rather than preserved.

## Non-goals

- Do not replace `SplitFlap` with a new public effect name.
- Do not break existing `split_flap` recipes or the current single-cell Solari
  output.
- Do not make the primitive GTD- or ratatui-specific.
- Do not expose a broad public `MechanicalDisplay` effect until at least three
  public effects need that wrapper. For this work, shared internal primitives
  are enough.

## Review verdict and implementation blockers

This feature is aligned with the project direction: it is grid-first, recipe
authoring belongs in `tui-vfx`, and the overlap between Odometer, SplitFlap, and
future Solari-style boards is enough to justify a shared primitive. The plan is
not yet implementable without the decisions below. Treat this section as a
blocking checklist before writing production code.

### Blocker 1 — decide the `TextTransformer` boundary

Current content effects implement:

```rust
pub trait TextTransformer {
    fn transform<'a>(
        &self,
        target: &'a str,
        progress: f64,
        signal_ctx: &SignalContext,
    ) -> Cow<'a, str>;
}
```

That means public content effects still return text, not `OwnedGrid` or
`SemanticScene`. The mechanical module may use `tui_vfx_types::OwnedGrid`
internally, but each public transformer must convert `target`/`from_message`
into a grid and convert the sampled grid back into a string before returning.

Required decision:

- Implement grid helpers inside `mechanical`, not by changing
  `TextTransformer`.
- Preserve exact `Cow::Borrowed(target)` behavior at `progress >= 1.0`.
- Keep the first implementation character-cell based. Do not promise styled
  grid preservation until a public API can accept a `SemanticScene`.

Suggested helper shape:

```rust
pub(crate) fn grid_from_text(text: &str, policy: MechanicalSizing) -> OwnedGrid;
pub(crate) fn grid_to_text(grid: &OwnedGrid) -> String;
pub(crate) fn paired_grids(from: Option<&str>, to: &str, policy: MechanicalSizing)
    -> (OwnedGrid, OwnedGrid);
```

Implementation notes:

- Preserve newlines by computing rows with `split('\n')`; do not treat newline
  as a tile cell.
- Fill missing cells with `Cell::new(' ')`.
- Preserve row count under `PadToMax` so `from_message` and target boards do not
  collapse during animation.
- Document that multi-cell mechanical text is character-cell based; grapheme and
  styled-cell support are separate future work.

### Blocker 2 — replace `ContentEffect::Odometer`; do not preserve the unit variant

The current enum has a unit variant:

```rust
ContentEffect::Odometer
```

That shape is not useful enough to protect. There are no important Odometer
consumers today, and the existing behavior was a candidate for removal before
this redesign. Treat the new Odometer as a replacement inside the same recipe
namespace, not as a backwards-compatible migration.

Required decision:

- Change `ContentEffect::Odometer` to an explicit struct variant.
- Do **not** support `{ "type": "odometer" }` as a meaningful default recipe.
  If required fields are missing, recipe validation/deserialization may fail.
- Do **not** preserve the old digit-interpolation behavior.
- Update every Rust construction site in the same implementation tranche:
  - `crates/tui-vfx-content/src/transformers/fnc_get_transformer.rs`
  - `crates/tui-vfx-content/src/types/cls_content_effect.rs`
  - `xtask/src/docs/effect_metadata.rs`
  - any tests or examples found by searching `ContentEffect::Odometer`.
- Update schema, annotated schema docs, hand-maintained docs, metadata, and all
  debug recipes to the new structured shape.

Alternative rejected for this slice: keep a legacy `Numeric`/unit-compatible path
or add `OdometerConfig` alongside the old variant. That would preserve behavior
that is already considered wrong and fragment the authoring vocabulary.

### Blocker 3 — Odometer cell-roll needs tile dimensions

The current proposed Odometer fields omit `tile_width` and `tile_height`, but the
design goal explicitly requires grid/cell roll modes. Without tile dimensions,
`CellRoll` can only roll whole rows/columns and cannot express a 3×3 typography
glyph or other multi-cell tile.

Add these fields to the Odometer mapping:

```rust
#[serde(default = "default_tile_width")]
tile_width: u16,
#[serde(default = "default_tile_height")]
tile_height: u16,
```

Validation:

- Odometer is tile-roll-first; all modes that remain in this tranche require `tile_width >= 1` and `tile_height >= 1`.
- Do not add a legacy numeric mode unless it is implemented as a thin preset over the same rolling mechanism and covered by new tests.
- If source dimensions are not divisible by tile size, the initial behavior is
  `PadToMax` and then clip partial edge tiles. Do not panic.

### Blocker 4 — decide wire shape for `RollTravel`

`RollTravel` has data (`Cells(u16)`), so a plain string enum is not enough. Use a
tagged object for schema regularity:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, ConfigSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum RollTravel {
    #[default]
    Axis,
    FullClear,
    Cells { cells: u16 },
}
```

Recipe examples must therefore use:

```json
"travel": { "type": "axis" }
```

not:

```json
"travel": "axis"
```

Adding a custom deserializer to accept both strings and objects is possible
later, but it complicates schema generation. Do not add it in the first slice.

### Blocker 5 — runtime validation must exist outside generated schema

The generated `ConfigSchema` surface describes fields and enum values, but it
does not currently express constraints like "`tile_height` must be one of
`2/4/6/8`" or "`tile_width` must be non-zero" for enum-variant fields. Do not
rely on generated schema alone.

Add an explicit validation layer:

```rust
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MechanicalValidationError {
    ZeroTileDimension,
    OddCenterHingeTileHeight { height: u16 },
    UnsupportedCenterHingeTileHeight { height: u16 },
    UnsupportedVerticalHinge,
}

pub(crate) fn validate_split_flap_tile(tile: MechanicalTile) -> Result<(), MechanicalValidationError> {
    if tile.width == 0 || tile.height == 0 {
        return Err(MechanicalValidationError::ZeroTileDimension);
    }
    if tile.height == 1 {
        return Ok(());
    }
    if tile.height % 2 != 0 {
        return Err(MechanicalValidationError::OddCenterHingeTileHeight { height: tile.height });
    }
    if !matches!(tile.height, 2 | 4 | 6 | 8) {
        return Err(MechanicalValidationError::UnsupportedCenterHingeTileHeight { height: tile.height });
    }
    Ok(())
}
```

Then call validation when building the concrete transformer in
`fnc_get_transformer.rs`. For invalid recipe input, prefer a no-op transformer
only if the existing content-effect dispatcher has no error channel. Otherwise
surface a validation error in the recipe validator. The plan must not leave
invalid tile sizes silently clamped.

### Blocker 6 — existing `cell_motion` overlap must be acknowledged

`tui-vfx-content` already has a public `cell_motion` module with
`CellMotionSpec`, `CellStagger`, `CellPlacement`, `CellMotionVisibility`, and
`apply_cell_motion`. Mechanical display primitives should not duplicate that
public scheduler accidentally.

Decision:

- Keep `mechanical` private and purpose-built for content transformers.
- Reuse vocabulary only where it reduces authoring friction (`cascade`,
  `dispersion`, `jitter`, `cycles`).
- Do not route Odometer/SplitFlap through `CellMotionSpec` in the first slice.
  Cell-motion moves independent actors through space; mechanical display effects
  sample a fixed viewport over paired source/target grids and have mechanism
  semantics (wheel roll, hinge phases) that are not generic cell placement.
- Add a note in rustdoc: "`mechanical` is mechanism-specific; `cell_motion` is
  general per-cell source remapping."

### Blocker 7 — `from_message` indexing is currently linear and newline-sensitive

Current SplitFlap uses `target.chars().enumerate()` and indexes
`from_message` by linear character index. The grid tile path must not inherit
that blindly. For multi-row tiles, `from_message` must be parsed into the same
row/column grid as the target.

Required behavior:

- `from_message` shorter than target pads with blanks by grid coordinate.
- `from_message` longer than target clips to the padded target viewport.
- Newline structure in either input participates in row parsing, not as a
  visible tile cell.
- Unchanged multi-cell tiles skip animation only when every cell in the tile is
  equal between `from` and `to` and `cycles == 0`.

Suggested helper:

```rust
pub(crate) fn tile_changed(from: &OwnedGrid, to: &OwnedGrid, rect: TileRect) -> bool {
    for y in rect.y..rect.y + rect.height {
        for x in rect.x..rect.x + rect.width {
            let a = from.get(x as usize, y as usize).map(|c| c.ch).unwrap_or(' ');
            let b = to.get(x as usize, y as usize).map(|c| c.ch).unwrap_or(' ');
            if a != b {
                return true;
            }
        }
    }
    false
}
```

## Existing SplitFlap facts to preserve

`crates/tui-vfx-content/src/transformers/cls_split_flap.rs` already provides the
best scheduling model in the codebase:

- `SplitFlapCharset::{Alpha, Digits, Uppercase}`
- `SplitFlapDispersion::{Legacy, Cascade, Authentic, Simultaneous, Random,
  CenterOut, EdgeIn, Shuffled}`
- `from_message`
- `cycles`, `cascade`, `jitter`
- `leading_blocks`, `settle_hinge`, `spring_settle`, `authentic_timing`
- `rolling_flip`, `flip_preview`, `flip_flicker`
- row-local `col_in_row` and `max_row_width` handling for multi-line boards
- deterministic FNV hashing for jitter/dispersion/flicker

The new design should reuse this vocabulary and, where possible, share the code.

## Proposed internal module

Add a small internal module under `tui-vfx-content`:

```text
crates/tui-vfx-content/src/mechanical/
├── mod.rs
├── cls_mechanical_source.rs
├── cls_mechanical_tile.rs
├── cls_mechanical_schedule.rs
├── enum_mechanical_dispersion.rs
├── enum_roll_direction.rs
├── enum_roll_travel.rs
├── enum_hinge_axis.rs
├── enum_blank_policy.rs
├── fnc_tile_progress.rs
├── fnc_roll_grid_window.rs
└── fnc_split_flap_tile_frame.rs
```

Keep the initial surface `pub(crate)` unless a consumer outside
`tui-vfx-content` needs it. Public schema-bearing aliases should be added only
where `ContentEffect` fields require them.

## Shared structures

### Source

Mechanical effects transition old content into new content.

```rust
pub(crate) struct MechanicalSource {
    pub from: OwnedGrid,
    pub to: OwnedGrid,
}
```

Source construction policies:

```rust
pub enum MechanicalSizing {
    /// Pad both grids to the max width/height with blanks. Default.
    PadToMax,
    /// Clip both grids to the requested viewport.
    ClipToViewport,
    /// Return an error when dimensions differ.
    ErrorOnMismatch,
}
```

Default: `PadToMax`. Mechanical displays are fixed windows; padding old/new
content is usually more useful than failing.

### Tile geometry

A tile is the mechanical unit that receives one local progress value.

```rust
pub struct MechanicalTile {
    pub width: u16,
    pub height: u16,
    pub gap_x: u16,
    pub gap_y: u16,
}
```

Common mappings:

| Use case | Tile |
|---|---:|
| Legacy SplitFlap | `1 x 1` |
| 2-row Solari card | `1 x 2` or wider |
| 4/6/8-row Solari card | `1 x 4`, `1 x 6`, `1 x 8` or wider |
| GTD 3x3 typography odometer | `3 x 3` |
| Whole-panel transition | `viewport width x viewport height` |

Mechanism-specific validation:

- Roll supports any non-zero tile width/height.
- Center-hinged Solari requires `height == 1` for legacy symbolic mode, or an
  even height in `{2, 4, 6, 8}` for grid mode.
- Reject `height == 3` for center hinge. A physical center hinge lies between
  rows, not through a row.

### Tile addressing

Scheduling needs row-local coordinates, not only a linear index.

```rust
pub(crate) struct MechanicalTileIndex {
    pub row: usize,
    pub col: usize,
    pub linear: usize,
}
```

This preserves the SplitFlap 3.2.1 lesson: cascade and dispersion must reset per
row for multi-line boards.

### Segmentation

```rust
pub enum MechanicalSegmentation {
    /// Divide the source into fixed-width/fixed-height tiles.
    TileGrid,
    /// One logical character per tile. Existing SplitFlap behavior.
    Character,
    /// Treat the whole source as one tile.
    WholeGrid,
}
```

Initial implementation can support only the segmentations required by the two
callers:

- `SplitFlap`: `Character` for legacy, `TileGrid` for multi-cell mode.
- `Odometer`: `TileGrid`.

### Schedule

Extract the common scheduling vocabulary from SplitFlap.

```rust
pub struct MechanicalSchedule {
    pub speed: SignalOrFloat,
    pub cascade: SignalOrFloat,
    pub cycles: SignalOrFloat,
    pub jitter: f32,
    pub dispersion: MechanicalDispersion,
    pub authentic_timing: bool,
}
```

`MechanicalDispersion` should match `SplitFlapDispersion` exactly:

```rust
pub enum MechanicalDispersion {
    Legacy,
    Cascade,
    Authentic,
    Simultaneous,
    Random,
    CenterOut,
    EdgeIn,
    Shuffled,
}
```

Compatibility path:

- Keep `SplitFlapDispersion` as the public serde type for existing SplitFlap
  fields in the first implementation slice.
- Add `From<SplitFlapDispersion> for MechanicalDispersion`.
- Consider renaming/unifying publicly only at a major-version boundary.

### Tile-local progress

The central reusable function:

```rust
pub(crate) fn tile_progress(
    global_progress: f64,
    tile: MechanicalTileIndex,
    row_width_tiles: usize,
    schedule: &MechanicalSchedule,
    signal_ctx: &SignalContext,
    distance_units: f64,
    max_distance_units: f64,
) -> f64
```

Responsibilities:

- evaluate `speed`, `cascade`, and `cycles`
- compute row-local dispersion delay
- apply deterministic jitter
- support authentic distance-proportional landing
- clamp to `[0.0, 1.0]`

Pseudo-code:

```rust
let speed = schedule.speed.evaluate(global_progress, signal_ctx).unwrap_or(1.0).max(0.0);
let cascade = schedule.cascade.evaluate(global_progress, signal_ctx).unwrap_or(0.0).max(0.0);
let jitter = jitter_factor(tile.linear, schedule.jitter);

if schedule.uses_authentic_timing() {
    let completion_ratio = (distance_units / max_distance_units.max(0.001)).max(0.001);
    return (global_progress * speed as f64 / completion_ratio * jitter).clamp(0.0, 1.0);
}

let delay = dispersion_delay(schedule.dispersion, tile.col, row_width_tiles);
(global_progress * speed as f64 - delay * cascade as f64 * jitter).clamp(0.0, 1.0)
```

For `Legacy`, SplitFlap may continue to use its existing raw-linear-index delay
for byte-for-byte compatibility. New Odometer/grid modes should use row-local
semantics only.

### Blank and overflow policy

```rust
pub enum MechanicalBlankPolicy {
    /// Do not write blank cells into the destination grid.
    Transparent,
    /// Write regular spaces.
    Space,
    /// Write a specific fill character.
    Fill(char),
}

pub enum MechanicalOverflow {
    Clip,
    Wrap,
    Blank,
}
```

Default for text recipes: `Space`.
Default intent for glyph/typography use cases: transparent blanks should preserve the surface underneath. In the current text-transformer boundary this means emitting spaces, but docs should name the semantic as transparent so GTD/styled-grid integration can preserve underlying cells later.

## Mechanisms

### Roll mechanism

```rust
pub enum RollDirection {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

pub enum RollTravel {
    Axis,
    FullClear,
    Cells { cells: u16 },
}

pub(crate) struct CellRollMechanism {
    pub direction: RollDirection,
    pub travel: RollTravel,
    pub wrap: bool,
}
```

Roll samples a fixed viewport over two grids. For `Up`, old content exits at the
top and new content enters from the bottom.

3-row example:

```text
from:     to:
AAA       111
BBB       222
CCC       333

p=0.00 -> AAA / BBB / CCC
p=0.34 -> BBB / CCC / 111
p=0.67 -> CCC / 111 / 222
p=1.00 -> 111 / 222 / 333
```

Implementation sketch:

```rust
pub(crate) fn roll_grid_window(
    from: &OwnedGrid,
    to: &OwnedGrid,
    local_progress: f64,
    direction: RollDirection,
    travel: RollTravel,
    cycles: f64,
    blank_policy: MechanicalBlankPolicy,
) -> OwnedGrid {
    let (dx, dy) = direction.vector();
    let travel_cells = travel.resolve(from.width(), from.height(), direction);
    let total = travel_cells as f64 * cycles.max(1.0);
    let offset = (local_progress * total).floor() as i32;
    let settle_offset = offset % travel_cells as i32;

    // For each output cell, sample old shifted by `settle_offset`; when old
    // is outside the viewport, sample new shifted in from the opposite side.
}
```

For `cycles > 1`, intermediate cycles can wrap through `from`/blank/noise, but
`local_progress >= 1.0` must return `to` exactly.

Concrete initial implementation for cardinal directions:

```rust
fn roll_sample_y(
    from: &OwnedGrid,
    to: &OwnedGrid,
    out_x: usize,
    out_y: usize,
    offset: i32,
    direction: RollDirection,
) -> Cell {
    let h = from.height() as i32;
    let source_y = match direction {
        RollDirection::Up => out_y as i32 + offset,
        RollDirection::Down => out_y as i32 - offset,
        _ => out_y as i32,
    };
    if (0..h).contains(&source_y) {
        return *from.get(out_x, source_y as usize).unwrap_or(&Cell::default());
    }
    let to_y = match direction {
        RollDirection::Up => source_y - h,
        RollDirection::Down => source_y + h,
        _ => out_y as i32,
    };
    if (0..h).contains(&to_y) {
        *to.get(out_x, to_y as usize).unwrap_or(&Cell::default())
    } else {
        Cell::default()
    }
}
```

Junior-dev note: implement `Up`, `Down`, `Left`, and `Right` first. Then add
diagonals by composing the x and y offsets. Do not hand-write eight unrelated
branches.

### Center-hinged SplitFlap tile mechanism

```rust
pub enum HingeAxis {
    Horizontal,
    Vertical,
}

pub(crate) struct SplitFlapTileMechanism {
    pub hinge_axis: HingeAxis,
    pub spring_settle: bool,
    pub flip_preview: bool,
    pub flip_flicker: bool,
}
```

Initial scope:

- Support `HingeAxis::Horizontal` for even-height cards.
- Keep legacy `height == 1` symbolic behavior unchanged.
- Reject or postpone vertical hinge until a recipe proves the need.

Even-height center hinge model:

```text
height = 4
row 0  old top half
row 1  old top half
────── hinge between row 1 and row 2
row 2  new bottom half
row 3  new bottom half
```

Frame phases should be simple and terminal-legible:

| Phase | Output policy |
|---|---|
| early | old tile visible |
| mid hinge | top half compresses toward hinge; bottom half pulls in from new tile |
| late | new tile expands from hinge |
| settle | new tile visible exactly |

Do not attempt perspective-correct 3D. Use terminal-native half-block/edge glyphs
where they improve the hinge line, but keep the core model grid-based and
predictable.

Concrete initial center-hinge policy:

```rust
pub(crate) fn split_flap_tile_frame(
    from: &OwnedGrid,
    to: &OwnedGrid,
    rect: TileRect,
    local_progress: f64,
    spring_settle: bool,
    blank_policy: MechanicalBlankPolicy,
) -> OwnedGrid {
    // p >= 1.0 must copy `to` exactly for the tile.
    // p <= 0.0 must copy `from` exactly for the tile.
    // 0.0..0.45: old tile remains readable.
    // 0.45..0.60: hinge line dominates; top half old, bottom half new.
    // 0.60..1.00: new tile expands from center outward.
}
```

Minimum viable frame semantics:

- `p < 0.45`: return old tile.
- `0.45 <= p < 0.60`: top half copies old tile; bottom half copies new tile;
  optionally draw `▀` on the row above the hinge and `▄` on the row below the
  hinge if it improves legibility.
- `0.60 <= p < 1.0`: reveal rows by distance from the hinge:
  - rows nearest the hinge use new cells first
  - outer rows stay old until their reveal threshold
- `p >= 1.0`: return new tile exactly.

This is intentionally terminal-native. The first slice needs deterministic,
readable tile phases more than fake 3D.

## Public effect mapping

### Odometer

Current:

```rust
ContentEffect::Odometer
```

Replacement shape. Rust callers and recipes must migrate from the unit variant
to the structured variant. No compatibility constructor or legacy unit/default
recipe shape is required.

```rust
Odometer {
    direction: RollDirection,
    travel: RollTravel,
    tile_width: u16,
    tile_height: u16,
    #[serde(default)]
    cascade: SignalOrFloat,
    #[serde(default)]
    cycles: SignalOrFloat,
    #[serde(default)]
    jitter: f32,
    #[serde(default)]
    dispersion: SplitFlapDispersion,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    from_message: Option<String>,
}
```

Primary semantics:

- Odometer is a mechanical tile-roll effect, not a digit interpolation effect.
- `direction`, `travel`, `tile_width`, and `tile_height` are the core authoring
  contract and should be explicit in debug recipes and docs.
- The old `Numeric` behavior should be deleted. If a future number-specific
  odometer preset is wanted, it must be implemented as a preset over this same
  tile-roll engine.
- Missing core fields may be rejected by serde or recipe validation; do not add
  defaults solely to keep `{ "type": "odometer" }` valid.

Example V3 recipe field:

```json
{
  "type": "odometer",
  "direction": "up",
  "travel": { "type": "axis" },
  "tile_width": 3,
  "tile_height": 3,
  "cycles": 3.0,
  "cascade": 0.04,
  "jitter": 0.08,
  "dispersion": "edge_in",
  "from_message": "099"
}
```

Recommended additional Odometer example:

```json
{
  "type": "odometer",
  "direction": "left",
  "travel": { "type": "cells", "cells": 3 },
  "tile_width": 3,
  "tile_height": 1,
  "from_message": "099"
}
```

### SplitFlap

Keep every existing field. Add optional grid-tile fields:

```rust
SplitFlap {
    // existing fields...
    #[serde(default = "default_tile_width")]
    tile_width: u16,
    #[serde(default = "default_tile_height")]
    tile_height: u16,
}
```

Defaults:

```rust
tile_width = 1
tile_height = 1
```

Behavior:

- `tile_width == 1 && tile_height == 1`: current single-cell symbolic SplitFlap
  path. This must remain byte-for-byte compatible where tests currently assert
  exact output.
- `tile_height in {2, 4, 6, 8}`: grid-aware center-hinged tile path.
- Any odd `tile_height > 1`: validation error in schema/constructor path.
- Any `tile_height > 8`: reject in the first implementation. Larger tiles can be
  added after performance/visual QA.
- `tile_width == 0` or `tile_height == 0`: validation error.
- `tile_width > 1 && tile_height == 1`: reject in the first implementation
  unless a concrete recipe needs wide single-row cards. Wide one-row cards have
  no center hinge, so accepting them now would blur semantics.

Example V3 recipe field:

```json
{
  "type": "split_flap",
  "from_message": "GATE 12",
  "cycles": 0.4,
  "jitter": 0.12,
  "settle_hinge": true,
  "spring_settle": true,
  "rolling_flip": true,
  "dispersion": "center_out",
  "tile_width": 3,
  "tile_height": 4
}
```

## Tests

Test-first requirement: write failing tests before implementation. Keep tests
close to the module they protect, but do not add more code to
`cls_split_flap.rs` than necessary; that file is already very large. New
mechanical helpers should have focused tests in mirrored `test_fnc_*` files where
possible.

### Odometer tests

Add or update tests near `cls_odometer.rs`.

Required unit tests:

```rust
#[test]
fn cell_roll_up_pulls_new_rows_from_bottom() {}

#[test]
fn cell_roll_down_pulls_new_rows_from_top() {}

#[test]
fn cell_roll_left_pulls_new_columns_from_right() {}

#[test]
fn cell_roll_right_pulls_new_columns_from_left() {}

#[test]
fn cell_roll_diagonal_up_right_combines_axis_motion() {}

#[test]
fn cell_roll_cycles_settle_exactly_on_target() {}

#[test]
fn cell_roll_dispersion_uses_row_local_columns() {}

#[test]
fn odometer_rejects_legacy_unit_variant_shape() {}

#[test]
fn odometer_deserializes_structured_cell_roll_shape() {}

#[test]
fn odometer_cell_roll_accepts_tile_dimensions() {}

#[test]
fn odometer_cell_roll_rejects_zero_tile_dimensions() {}
```

Canonical 3-row fixture:

```text
from = ["AAA", "BBB", "CCC"]
to   = ["111", "222", "333"]
```

Expected upward frames:

```rust
p = 0.0 => ["AAA", "BBB", "CCC"]
p ~= 0.34 => ["BBB", "CCC", "111"]
p ~= 0.67 => ["CCC", "111", "222"]
p = 1.0 => ["111", "222", "333"]
```

Concrete test helper:

```rust
fn lines(grid: &OwnedGrid) -> Vec<String> {
    (0..grid.height())
        .map(|y| {
            (0..grid.width())
                .map(|x| grid.get(x, y).map(|c| c.ch).unwrap_or(' '))
                .collect()
        })
        .collect()
}
```

Serde tests must reject the legacy unit-like shape and accept the new structured shape:

```rust
#[test]
fn odometer_rejects_legacy_unit_variant_shape() {
    let err = serde_json::from_str::<ContentEffect>(r#"{ "type": "odometer" }"#).unwrap_err();
    assert!(err.to_string().contains("missing"));
}

#[test]
fn odometer_deserializes_structured_shape() {
    let parsed: ContentEffect = serde_json::from_str(r#"
    {
      "type": "odometer",
      "direction": "up",
      "travel": { "type": "axis" },
      "tile_width": 3,
      "tile_height": 3,
      "from_message": "AAA\nBBB\nCCC"
    }
    "#).unwrap();
    assert!(matches!(parsed, ContentEffect::Odometer { .. }));
}

#[test]
fn odometer_rejects_unknown_fields() {
    let err = serde_json::from_str::<ContentEffect>(
        r#"{ "type": "odometer", "bogus": true }"#
    ).unwrap_err();
    assert!(err.to_string().contains("unknown field"));
}
```


### SplitFlap tests

Add tests without weakening existing ones.

```rust
#[test]
fn split_flap_default_tile_size_preserves_single_cell_output() {}

#[test]
fn split_flap_rejects_odd_multicell_tile_height() {}

#[test]
fn split_flap_accepts_even_tile_heights_2_4_6_8() {}

#[test]
fn split_flap_tile_height_above_8_is_rejected_initially() {}

#[test]
fn split_flap_multicell_center_hinge_settles_exactly_on_target() {}

#[test]
fn split_flap_multicell_from_message_preserves_unchanged_tiles() {}

#[test]
fn split_flap_rejects_wide_single_row_tiles_in_first_slice() {}

#[test]
fn split_flap_multicell_uses_grid_from_message_not_linear_char_index() {}

#[test]
fn split_flap_multicell_preserves_newline_structure() {}
```

Exact compatibility test to add before touching SplitFlap routing:

```rust
#[test]
fn split_flap_default_tile_size_preserves_single_cell_output_snapshot() {
    let old = SplitFlap::new_mechanical(
        SignalOrFloat::from(1.0),
        SignalOrFloat::from(0.05),
        SignalOrFloat::from(0.5),
        0.12,
        SplitFlapCharset::Alpha,
        false,
        0.05,
        true,
        true,
        true,
    )
    .with_from_message("LONDON")
    .with_rolling_flip(true)
    .with_flip_preview(true)
    .with_dispersion(SplitFlapDispersion::CenterOut);

    // Capture values before routing changes land. These assertions are allowed
    // to be exact because tile_width/tile_height default to 1 and must remain on
    // the old path.
    assert_eq!(old.transform("PARIS ", 0.0, &ctx()), old.transform("PARIS ", 0.0, &ctx()));
    assert_eq!(old.transform("PARIS ", 0.5, &ctx()), old.transform("PARIS ", 0.5, &ctx()));
    assert_eq!(old.transform("PARIS ", 1.0, &ctx()), "PARIS ");
}
```

Before implementing, replace the self-comparison with captured expected strings
from the current codebase. The point is to freeze existing output for a
representative mechanical SplitFlap configuration before adding the tile path.

### Schema/doc tests

Run existing schema tests and add assertions if there are focused schema tests for
content effect variants:

```bash
cargo test -p tui-vfx-core
cargo test -p tui-vfx-content
cargo test -p tui-vfx --test test_full_schema_dump
```

Required new schema assertions:

- `docs/generated/effect_schemas.json` content variant `split_flap` lists
  `tile_width` and `tile_height`.
- `docs/generated/effect_schemas.json` content variant `odometer` changes from
  `kind: "unit"` to `kind: "struct"` and lists `mode`, `direction`, `travel`,
  `tile_width`, `tile_height`, `cascade`, `cycles`, `jitter`, `dispersion`, and
  `from_message`.
- The generated schema for `RollTravel` shows tagged variants `axis`,
  `full_clear`, and `cells`.
- `ContentEffect::key_parameters()` includes new fields for debugging output:
  `Odometer` should no longer return an empty vec in structured mode.

Because generated schema cannot express the allowed tile-height set, add a
validator or unit test that directly rejects `{ "type": "split_flap",
"tile_height": 3 }`.

## API, schema, and rustdoc generation gaps to close

### New public types

Any type used by `ContentEffect` fields is public schema-bearing surface and
needs serde, `ConfigSchema`, rustdoc, and re-exports from `types/mod.rs`.

Expected public types:

```rust
pub enum OdometerMode {
    Numeric,
    CellRoll,
}

pub enum RollDirection {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

pub enum RollTravel {
    Axis,
    FullClear,
    Cells { cells: u16 },
}
```

Keep internal-only types (`MechanicalSource`, `MechanicalTile`,
`MechanicalTileIndex`, `MechanicalSchedule`, validation helpers) `pub(crate)`
unless they appear in `ContentEffect`.

### `ConfigSchema` and generated docs

Implementation must update all construction and generated-metadata paths:

- `crates/tui-vfx-content/src/types/cls_content_effect.rs`
- `crates/tui-vfx-content/src/types/mod.rs`
- `crates/tui-vfx-content/src/transformers/mod.rs`
- `crates/tui-vfx-content/src/transformers/fnc_get_transformer.rs`
- `xtask/src/docs/effect_metadata.rs`
- `docs/templates/capabilities.toml`
- `docs/CAPABILITIES_REFERENCE.md`
- `CAPABILITIES.md`
- generated files under `docs/generated/`

Common miss: `xtask/src/docs/effect_metadata.rs` manually constructs one value
per content effect. It currently constructs `ContentEffect::Odometer`. That must
change to the new structured variant; do not add a compatibility constructor just
for this path.

### Rustdoc text to add

Add rustdoc near `ContentEffect::Odometer`:

```rust
/// Mechanical tile-grid rolling display.
///
/// Odometer treats content as a fixed character-cell grid and rolls old cells out
/// while new cells enter from the opposite edge. Use `from_message` to provide
/// the previous visible content; otherwise the old grid is blank-padded.
///
/// `tile_width` and `tile_height` group cells for scheduling only. They do not
/// change the output dimensions. The previous unit-variant digit interpolation
/// Odometer was intentionally replaced and is not preserved.
```

Add rustdoc near `SplitFlap` tile fields:

```rust
/// Width of one mechanical tile in character cells.
///
/// Defaults to `1`, preserving the legacy one-character-per-flap path. Multi-cell
/// SplitFlap tiles are grid-rendered; `tile_width > 1` is supported only together
/// with an even `tile_height` in the first implementation.
#[serde(default = "default_tile_width")]
tile_width: u16,

/// Height of one mechanical tile in character cells.
///
/// `1` preserves legacy single-cell SplitFlap behavior. Even heights `2`, `4`,
/// `6`, and `8` enable center-hinged Solari-style cards. Odd heights greater
/// than one are rejected because a center hinge lies between rows.
#[serde(default = "default_tile_height")]
tile_height: u16,
```

### Capability docs text to add

Suggested `docs/CAPABILITIES_REFERENCE.md` content:

```md
| **Odometer** | Mechanical cell-grid roll |

#### Odometer tile roll

- Odometer treats the message as a fixed character-cell grid. Use `from_message`
  for the previous grid, `direction` for travel direction, and
  `tile_width`/`tile_height` to schedule multi-cell glyphs as one mechanical unit.
- The previous digit-interpolation Odometer is intentionally removed.
- `travel` is a tagged object: `{ "type": "axis" }`,
  `{ "type": "full_clear" }`, or `{ "type": "cells", "cells": 3 }`.
```

Suggested SplitFlap content:

```md
#### SplitFlap tile size

`tile_width: 1, tile_height: 1` is the legacy single-cell SplitFlap path and
preserves existing recipes. Even tile heights `2`, `4`, `6`, and `8` enable
grid-aware center-hinged Solari cards. Odd multi-cell heights are invalid because
the hinge must lie between rows.
```

## Documentation updates

Update all of these in the implementation phase:

- Rustdoc on new public fields and variants.
- `docs/CAPABILITIES_REFERENCE.md`
  - Odometer row: explain tile-roll semantics and note that legacy digit interpolation was removed.
  - SplitFlap row: explain `tile_width`/`tile_height` and even-height Solari.
  - Content transformer section: add examples and validation notes.
- `CAPABILITIES.md`
- generated docs:
  - `docs/generated/CAPABILITIES.md`
  - `docs/generated/capabilities.json`
  - `docs/generated/effect_schemas.json`
  - any API generated output changed by `just docs-all`
- V3 annotated schema references if the fields are manually described there.

Validation:

```bash
just docs-all
just docs-all-check
just docs-all-validate
```

## Debug recipes

Add V3 debug recipes in the sibling repo:

```text
/usr/projects/tui-vfx-recipes/recipes/debug_recipes/content/
```

Odometer recipes:

```text
content_odometer_cell_roll_up.json
content_odometer_cell_roll_down.json
content_odometer_cell_roll_left.json
content_odometer_cell_roll_diagonal.json
content_odometer_cell_roll_slot_machine.json
content_odometer_cell_roll_dispersion_edge_in.json
```

SplitFlap tile recipes:

```text
content_split_flap_tile_2row.json
content_split_flap_tile_4row.json
content_split_flap_tile_6row.json
content_split_flap_tile_8row.json
content_split_flap_tile_board.json
```

Recipe quality requirements:

- `schema_version: 3`
- clear `id`, `title`, `description`, `version`, `last_updated`
- metadata with `aesthetic_tags`, `use_cases`, `authoring_notes`, `last_reviewed`
- layout sized so the effect is visible, not clipped
- professional colors, not default-only styling
- where useful, pair with `GlyphStyle`, `BorderSweep`, or subtle `GlistenBand`
- validate with pipeline-validator and debug recipe QC

Concrete Odometer debug recipe payload:

```json
{
  "type": "odometer",
  "direction": "up",
  "travel": { "type": "axis" },
  "tile_width": 1,
  "tile_height": 3,
  "cycles": 1.0,
  "cascade": 0.04,
  "jitter": 0.0,
  "dispersion": "cascade",
  "from_message": "AAA\nBBB\nCCC"
}
```

The recipe message should be:

```text
111
222
333
```

Use a `height` large enough to show all rows. If the recipe layout clips the
effect, the fixture is not useful as a debug recipe.

Concrete SplitFlap tile recipe payload:

```json
{
  "type": "split_flap",
  "speed": 1.0,
  "cascade": 0.0,
  "cycles": 0.4,
  "jitter": 0.08,
  "settle_hinge": true,
  "spring_settle": true,
  "rolling_flip": true,
  "dispersion": "center_out",
  "tile_width": 3,
  "tile_height": 4,
  "from_message": "OLD OLD\nOLD OLD\nOLD OLD\nOLD OLD"
}
```

The target message should use the same grid dimensions. Add a short
`authoring_notes` sentence describing the expected hinge line between rows 1/2
for a 4-row tile, or between rows 2/3 for a 6-row tile.

Validation in `/usr/projects/tui-vfx-recipes`:

```bash
just fmt-check
cargo test --test test_debug_recipes_qc
cargo test -p pipeline-validator --test test_debug_recipes_qc
just check
```

## Implementation phases

### Phase 1 — tests and shared scheduling extraction

- Add failing tests for structured Odometer serde and grid-roll helper.
- Extract shared FNV hash, jitter, and dispersion delay logic from SplitFlap into
  `mechanical` without changing SplitFlap output.
- Prove SplitFlap existing tests still pass.

Detailed tasks:

1. Create `crates/tui-vfx-content/src/mechanical/mod.rs`.
2. Add only private helpers in Phase 1:
   - `fnc_fnv_hash.rs`
   - `fnc_jitter_factor.rs`
   - `fnc_dispersion_delay.rs`
   - `cls_mechanical_tile.rs`
   - `cls_mechanical_tile_index.rs`
3. Move logic, not semantics:
   - `SplitFlapDispersion::Legacy` remains legacy-index based.
   - Non-legacy dispersion uses row-local `col_in_row`.
4. Add a compatibility test around current SplitFlap output before rewiring.
5. Keep files OFPF-sized. Do not move all SplitFlap logic into one new
   mega-file.

### Phase 2 — Odometer tile roll replacement

- Replace the Odometer unit variant with structured tile-roll fields.
- Implement Odometer using `roll_grid_window` as the only first-class behavior.
- Add cardinal and diagonal tests.
- Update or replace the existing odometer debug recipe with the new structured shape.

Detailed tasks:

1. Add public `RollDirection` and `RollTravel` schema-bearing types where the
   `ContentEffect::Odometer` fields require them. Do not add `OdometerMode` unless
   a second real mode is implemented over the same roll engine.
2. Add required `direction`, `travel`, `tile_width`, and `tile_height` fields to
   Odometer.
3. Update all `ContentEffect::Odometer` matches and constructors.
4. Store config fields directly on `Odometer` instead of using the current unit
   struct.
5. Implement `grid_from_text`, `grid_to_text`, `paired_grids`, and
   `roll_grid_window`.
6. Use `from_message.unwrap_or(blank grid)` for the old/source grid.
7. Delete the old digit-interpolation code path and tests.

### Phase 3 — SplitFlap multi-cell tiles

- Add `tile_width` and `tile_height` fields with default `1`.
- Route `1 x 1` to the existing code path.
- Implement even-height grid tile path.
- Add tests for validation and settling.

Detailed tasks:

1. Add tile fields to `ContentEffect::SplitFlap`, `SplitFlap`, constructors, and
   builder methods.
2. Route exactly `tile_width == 1 && tile_height == 1` to existing `transform()`
   logic. This branch should run before grid conversion.
3. For `tile_height > 1`, parse target/from into paired grids.
4. Segment into tile rectangles.
5. For each tile:
   - compute `MechanicalTileIndex { row, col, linear }`
   - skip unchanged tiles when `cycles == 0`
   - compute local progress with shared schedule
   - sample `split_flap_tile_frame`
   - blit tile output into the destination grid
6. Convert destination grid back to text.
7. Reject invalid tile sizes through validation before rendering.

### Phase 4 — docs/schema/generated artifacts

- Update rustdoc.
- Update hand-maintained docs.
- Regenerate docs/schema outputs.
- Run docs checks.

Detailed tasks:

1. Update rustdoc in every public schema-bearing type.
2. Update `docs/templates/capabilities.toml` because generated capabilities use
   curated TOML input.
3. Update `docs/CAPABILITIES_REFERENCE.md` with mode/tile examples.
4. Run `just docs-all`.
5. Run `just docs-all-check`.
6. Inspect generated diff, especially:
   - `docs/generated/CAPABILITIES.md`
   - `docs/generated/capabilities.json`
   - `docs/generated/effect_schemas.json`

### Phase 5 — debug recipes and recipe validation

- Add new V3 debug recipes in `tui-vfx-recipes`.
- Validate with pipeline-validator and debug recipe QC.
- Update any debug recipe catalog/inventory if required by tests.

Detailed tasks:

1. Replace the old odometer recipe with the structured tile-roll recipe shape.
2. Add one primitive-first recipe for each roll direction family:
   - up/down
   - left/right
   - one diagonal
   - slot-machine/high-cycle case
   - dispersion case
3. Add 2/4/6/8-row SplitFlap tile recipes.
4. Run `cargo test -p pipeline-validator --test test_debug_recipes_qc`.
5. If QC fails on recipe metadata, update the recipe metadata rather than
   weakening QC.

## Junior developer file checklist

Use this as the implementation work queue. Complete one row at a time.

| File | Change |
|---|---|
| `crates/tui-vfx-content/src/lib.rs` | Add `mod mechanical;` as private unless public re-export becomes necessary. |
| `crates/tui-vfx-content/src/mechanical/mod.rs` | Register private mechanical helper modules. |
| `crates/tui-vfx-content/src/mechanical/fnc_grid_from_text.rs` | Convert text rows to `OwnedGrid`. |
| `crates/tui-vfx-content/src/mechanical/fnc_grid_to_text.rs` | Convert `OwnedGrid` back to newline-separated text. |
| `crates/tui-vfx-content/src/mechanical/fnc_roll_grid_window.rs` | Implement roll viewport sampling and tests. |
| `crates/tui-vfx-content/src/mechanical/fnc_split_flap_tile_frame.rs` | Implement even-height center-hinge tile frames. |
| `crates/tui-vfx-content/src/mechanical/fnc_tile_progress.rs` | Shared scheduling/progress logic. |
| `crates/tui-vfx-content/src/transformers/cls_odometer.rs` | Replace unit transformer with config-bearing tile-roll implementation; delete old digit interpolation path. |
| `crates/tui-vfx-content/src/transformers/cls_split_flap.rs` | Add tile fields and route non-1×1 to mechanical grid path only. |
| `crates/tui-vfx-content/src/types/cls_content_effect.rs` | Add public schema fields, rustdoc, key parameter output, default helpers. |
| `crates/tui-vfx-content/src/types/mod.rs` | Re-export public schema types. |
| `crates/tui-vfx-content/src/transformers/fnc_get_transformer.rs` | Build configured Odometer and SplitFlap tile settings. |
| `xtask/src/docs/effect_metadata.rs` | Update constructed Odometer/SplitFlap metadata values. |
| `docs/templates/capabilities.toml` | Update Odometer/SplitFlap hints. |
| `docs/CAPABILITIES_REFERENCE.md` | Add mode/tile examples and validation notes. |
| `/usr/projects/tui-vfx-recipes/recipes/debug_recipes/content/*.json` | Add/update debug recipes after core behavior lands. |

OFPF guidance:

- `fnc_*` files should contain one public helper and focused private leaf logic.
- If a helper grows past the soft limit, split leaf math into `col_*` helpers.
- Keep `cls_split_flap.rs` from growing further where practical; route to
  mechanical helpers instead of adding another large block inline.

## Definition of done

In `/usr/projects/tui-vfx`:

```bash
cargo fmt
cargo test -p tui-vfx-content
cargo test
just docs-all-check
just check-all
```

In `/usr/projects/tui-vfx-recipes`:

```bash
just fmt-check
cargo test --test test_debug_recipes_qc
cargo test -p pipeline-validator --test test_debug_recipes_qc
just check
```

The feature is complete only when:

- old `split_flap` recipes still render through the single-cell path
- old `odometer` recipes still deserialize
- Odometer has useful grid-roll behavior under the old namespace
- SplitFlap supports even-height multi-cell Solari tiles without regressing
  single-cell behavior
- docs and generated schema outputs describe the new fields
- V3 debug recipes demonstrate vertical, horizontal, diagonal, slot-machine, and
  2/4/6/8-row Solari cases

<!-- <FILE>docs/design/tui-vfx-mechanical-display-primitives-plan.md</FILE> - <DESC>Design plan for shared mechanical display primitives powering Odometer grid roll and multi-cell SplitFlap/Solari effects</DESC> -->
<!-- <VERS>END OF VERSION: 0.3.0</VERS> -->
