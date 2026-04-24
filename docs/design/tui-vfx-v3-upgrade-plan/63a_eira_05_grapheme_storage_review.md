<!-- <FILE>docs/design/tui-vfx-v3-upgrade-plan/63a_eira_05_grapheme_storage_review.md</FILE> - <DESC>As-built EIRA-05 review of tui-vfx grapheme storage, wide-cell handling, and color-inert glyph behavior ahead of ANSI/source ingestion work.</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>EIRA-05 asks whether ANSI/source ingestion needs a deeper core grapheme model before landing. This note records the current implementation shape, the concrete seams inspected, and why deeper storage changes are deferred for now.</WCTX> -->
<!-- <CLOG>0.1.0: initial as-built review note for grapheme storage / wide-cell / color-inert behavior, with inspected source paths and defer-vs-adopt decision for Chapter 63.</CLOG> -->

# 63A — EIRA-05 grapheme storage review

This note records the **as-built** Unicode/storage behavior that exists today in
`tui-vfx` and `tui-vfx-recipes` before ANSI/source-ingestion work widens.
Chapter 63's boundary still holds: ANSI ingestion must normalize into the
existing grid/scene contracts unless a concrete, earned reason appears to change
those contracts.

## Decision

**Defer deeper core storage changes.**

No core grapheme-storage rewrite is adopted in this lane.

The current codebase already has two distinct layers:

1. **String-phase helpers are grapheme-aware** where text effects operate on
   owned strings before grid projection.
2. **Grid/cell storage is scalar-per-cell** once content becomes a
   `tui_vfx_types::Cell` inside an `OwnedGrid`.

ANSI/source ingestion should respect that split for now. A deeper storage model
would need cross-cutting changes across cell storage, grid indexing, text/card
source layout, ratatui ingress/egress, role tagging, and scene composition. The
review did not find a narrow bug whose safe fix would justify that broader
rewrite.

## Inspected source paths

### Core storage and scene shape

- `crates/tui-vfx-types/src/cell.rs`
- `crates/tui-vfx-types/src/grid.rs`
- `crates/tui-vfx-types/src/color_inert.rs`
- `crates/tui-vfx-types/src/semantic_scene.rs`

### String-phase grapheme helpers and existing Unicode tests

- `crates/tui-vfx-content/src/utils/fnc_graphemes.rs`
- `crates/tui-vfx-content/src/pool/cls_text_pool.rs`
- `crates/tui-vfx-content/tests/utils/test_fnc_graphemes.rs`
- `crates/tui-vfx-content/tests/transformers/test_cls_marquee.rs`
- `crates/tui-vfx-content/tests/transformers/test_cls_typewriter.rs`
- `crates/tui-vfx-content/tests/test_typewriter_cursor.rs`
- `crates/tui-vfx-content/tests/cursor/test_fnc_typewriter_cursor_position.rs`

### Source/layer projection and adapter seams

- `src/scene/layers/cls_text_layer.rs`
- `src/scene/layers/cls_card_layer.rs`
- `src/v3/compile/cls_v3_source_surface.rs`
- `src/v3/compile/fnc_build_scene_source_from_compiled_plan.rs`
- `src/compat.rs`
- `src/rendering/cls_ratatui_buffer_adapter.rs`

## As-built findings

### 1. Core grid storage is one Unicode scalar per cell

`Cell` stores `pub ch: char` and remains `Copy` (`crates/tui-vfx-types/src/cell.rs`).
`OwnedGrid` stores a flat `Vec<Cell>` indexed by `(x, y)` without extra width,
continuation, or grapheme metadata (`crates/tui-vfx-types/src/grid.rs`).

That means the core storage model can represent:

- ASCII and box-drawing scalars cleanly
- single-scalar emoji/CJK code points as a single `char`

It does **not** model:

- multi-scalar grapheme clusters as one stored unit
- explicit wide-cell continuation/occupancy
- display-width-aware layout metadata

### 2. Grapheme awareness exists before grid projection, not inside the grid

`tui-vfx-content` already uses `unicode_segmentation` for string-phase helpers
such as `len_graphemes` and `slice_graphemes`
(`crates/tui-vfx-content/src/utils/fnc_graphemes.rs`). Existing tests confirm
that authored text effects count and slice emoji/grapheme clusters before they
are projected into cells.

`TextPool` sanitization also preserves Unicode content while stripping control
bytes (`crates/tui-vfx-content/src/pool/cls_text_pool.rs`).

This is useful evidence for EIRA-02: ANSI/source ingestion does **not** need a
new core model just to preserve Unicode while content is still plain text.

### 3. Source/layer projection is char-oriented today

The main text-bearing source paths project strings into grids with `chars()` and
`chars().count()` rather than grapheme segmentation or display-width
measurement:

- `src/scene/layers/cls_text_layer.rs`
- `src/scene/layers/cls_card_layer.rs`
- `src/v3/compile/cls_v3_source_surface.rs`

That affects:

- natural-width measurement for alignment
- clip/wrap behavior
- per-cell placement into the destination grid

So the current layout contract is **"one scalar consumed per grid slot"**, not
"one grapheme cluster per slot" and not "one display column per slot."

### 4. Ratatui ingress/egress is explicitly lossy for multi-scalar symbols

`src/compat.rs` documents the current adapter seam directly:
`ratatui_cell_to_vfx` extracts `cell.symbol().chars().next()` and truncates
multi-character graphemes to their first scalar.

This is the clearest evidence that broad ANSI/source ingestion should **not**
claim cluster fidelity in the current core model. The ratatui bridge already
reduces external cell symbols to the same scalar-per-cell contract used by core
storage.

### 5. Wide-cell behavior is implicit, not tracked

Because width/alignment code counts `chars()` and core cells store only `char`,
wide glyph occupancy is not tracked as a first-class invariant.

Examples:

- a CJK scalar may render as width 2 in a terminal, but source layout code still
  measures it as `1`
- emoji that are a single scalar still occupy one stored cell, with no explicit
  continuation marker for any additional terminal column
- multi-scalar emoji/ZWJ clusters are not representable as one stored cell at
  all

This is a real limitation, but it is a **structural limitation**, not a narrow
bug in one touched function.

### 6. Color-inert handling is already explicit, but char-scoped

`tui-vfx-types` already has explicit color-inert detection in
`crates/tui-vfx-types/src/color_inert.rs`, and shadow grading uses it in
`crates/tui-vfx-compositor/src/pipeline/fnc_grade_shadow_cell.rs`.

Today that logic is still scalar-based:

- emoji/PUA/variation-selector/ZWJ-related scalars can be detected
- replacement during shadow grading is done per stored `char`
- there is no cluster-level "this whole grapheme is color-inert" object in core
  storage

This is sufficient for the current shadow-grading contract and does not, by
itself, justify a wider cell-model rewrite.

## Adopt now vs defer

### Adopted/kept as-is

- Keep grapheme-aware string helpers in `tui-vfx-content`.
- Keep the current scalar-per-cell `Cell`/`OwnedGrid` contract.
- Keep color-inert replacement as an explicit, char-scoped shading concern.
- Keep ANSI/source-ingestion work at the adapter/normalization boundary rather
  than redesigning core storage preemptively.

### Deferred

- multi-scalar grapheme storage in `Cell`
- explicit wide-cell continuation metadata in `OwnedGrid`
- display-width-aware text/card/source alignment in the core scene builders
- lossless ratatui/ANSI symbol round-tripping for cluster-bearing cells

## Why defer now

1. **Blast radius is large.** `Cell`, `OwnedGrid`, scene builders, role maps,
   snapshot adapters, and compiled/direct rendering paths all assume the current
   scalar-per-cell layout.
2. **The current model already serves authored VFX text work.** The existing
   grapheme-aware helpers live in the string-transform layer where they are most
   useful today.
3. **ANSI ingestion can still land as normalization.** Chapter 63 only requires
   adapters to normalize into current scene/grid contracts; it does not require
   lossless cluster storage on day one.
4. **Performance costs would be on hot paths.** Adding grapheme objects,
   continuation metadata, or per-cell display-width bookkeeping would increase
   storage and branch cost in the row-major grid and compositor paths that are
   currently simple `Copy`-cell loops.

## Concrete bug review result

No narrow source-handling fix was landed in this review.

The gaps found here are real, but they cluster around the current core storage
contract rather than a small localized defect. Changing behavior piecemeal in a
single text/source function would risk producing a half-lossy model that looks
more correct in one lane while remaining inconsistent everywhere else.

## Test status

No new tests were added in EIRA-05.

Existing tests already cover:

- grapheme-aware string helpers in `tui-vfx-content`
- Unicode-safe text-effect behavior before grid projection
- color-inert scalar classification in `tui-vfx-types`

What remains intentionally untested in this lane is a new wide-cell/grapheme
storage contract, because that contract is **deferred**, not adopted.

## Guidance for EIRA-02

ANSI/source ingestion should document the current normalization rule clearly:

- preserve text/style information as far upstream as possible
- normalize into the existing scalar-per-cell grid model before compositor use
- do not claim lossless grapheme-cluster or wide-cell fidelity without a later,
  explicit core-storage redesign

<!-- <FILE>docs/design/tui-vfx-v3-upgrade-plan/63a_eira_05_grapheme_storage_review.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
