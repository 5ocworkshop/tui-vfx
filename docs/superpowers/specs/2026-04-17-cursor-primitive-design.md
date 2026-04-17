<!-- <FILE>docs/superpowers/specs/2026-04-17-cursor-primitive-design.md</FILE> - <DESC>Design spec for the general Cursor primitive (grow-in + wake)</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>feat/cursor-primitive: brainstorm → design spec for new Cursor primitive with grow-in and wake effects</WCTX> -->
<!-- <CLOG>Initial spec from brainstorming session</CLOG> -->

# Cursor Primitive Design — Grow-In and Wake Effects

**Status:** Draft, pending user review
**Date:** 2026-04-17
**Scope:** New general-purpose `Cursor` primitive in `tui-vfx-content`, with two new animation capabilities (grow-in, wake). Typewriter integration via `#[serde(flatten)]`. Other consumers (standalone overlay, editor caret, other transformers) are future work using the same API.

---

## 1. Motivation

The only cursor concept in the repo today is `TypewriterCursor`, tied exclusively to the typewriter content transformer. Two new subtle text effects require cursor-level animation:

- **Cursor Wake** — a fading warmth trail at recent cursor positions, adding spatial memory without a loud highlight.
- **Cursor Grow-In** — the cursor materializes from vertical 1/8th blocks (`▁▂▃▄▅▆▇█`) while fading in alpha, giving arrivals a quiet weight.

Rather than bolt these onto `TypewriterCursor` (locking them to typewriter), we extract a general `Cursor` primitive that typewriter, standalone overlays, editor carets, and other content transformers can all own.

**Do-nothing guarantee:** all new animation fields default to values that preserve today's static-cursor rendering exactly. Existing recipes and Rust callers render identically after the refactor.

---

## 2. Architecture

### 2.1 Module layout

```
crates/tui-vfx-content/src/cursor/
  mod.rs
  cls_cursor.rs                 # Cursor config struct
  cls_cursor_blink.rs           # CursorBlink config
  cls_cursor_grow_in.rs         # GrowIn + GrowInMode + GrowDirection
  cls_cursor_wake.rs            # Wake + WakeMode
  cls_cursor_state.rs           # CursorState (runtime)
  cls_cursor_paint_ops.rs       # CursorPaintOps (per-frame output)
  fnc_advance_cursor.rs         # Position + time step → new state
  fnc_render_cursor.rs          # State → paint ops
  fnc_cursor_grow_in_glyph.rs   # Progress + base glyph + direction → (glyph, alpha)
```

### 2.2 Separation of concerns

| Type                | Owns             | Purpose                                                  |
|---------------------|------------------|----------------------------------------------------------|
| `Cursor`            | config only      | Serializable authoring surface                           |
| `CursorState`       | runtime state    | Position, history ring, in-flight animation phase        |
| `CursorPaintOps`    | per-frame output | Paint ops the consumer hands to the compositor           |

### 2.3 Caller contract

```rust
// Each frame the consumer calls:
fnc_advance_cursor(&mut state, &cursor, new_position: Option<(u16, u16)>, now: f64, dt: f64);
let paint_ops = fnc_render_cursor(&state, &cursor, now: f64);
// Consumer hands paint_ops to the compositor.
```

Both functions are pure reads/writes of the provided state — no globals, no clock assumptions beyond what the caller supplies.

---

## 3. Config Surface

### 3.1 Types

```rust
pub struct Cursor {
    /// Glyph to display. Empty string = no cursor.
    pub character: String,
    /// 0..1 alpha. Multiplied with any consumer-side phase visibility (e.g.,
    /// typewriter's show_while_typing/show_after_complete) to produce the
    /// effective visibility the grow-in state machine watches.
    pub visibility: SignalOrFloat,
    pub blink: CursorBlink,
    pub grow_in: GrowIn,
    pub wake: Wake,
}

pub struct CursorBlink {
    /// Milliseconds on + ms off. 0 = no blink (always visible).
    pub interval_ms: SignalOrFloat,
}

pub struct GrowIn {
    pub mode: GrowInMode,         // Never (default) | Once | EveryShow
    pub direction: GrowDirection, // Up (default) | Down | Center
    /// Duration ms for grow-in. 0 = instant (do-nothing default).
    pub duration_ms: SignalOrFloat,
    /// Duration ms for grow-out on hide. 0 = instant (default).
    pub grow_out_ms: SignalOrFloat,
    /// Curve sampled with t in 0..1 → eased progress in 0..1. Linear default.
    pub curve: SignalOrFloat,
}

pub enum GrowInMode    { Never, Once, EveryShow }
pub enum GrowDirection { Up, Down, Center }

pub struct Wake {
    pub mode: WakeMode, // Off (default) | Tint | Ghost
    /// Per-cell decay duration. 0 = off.
    pub decay_seconds: SignalOrFloat,
    /// Hard cap on trail length. 0 = no cap (time-only).
    pub max_cells: u32,
    /// Curve sampled with age-normalized t in 0..1 → alpha in 0..1. Default = 1 − t.
    pub curve: SignalOrFloat,
    /// Color tint for both Tint and Ghost modes. Theme-aware via ColorConfig.
    pub tint: ColorConfig,
}

pub enum WakeMode { Off, Tint, Ghost }
```

### 3.2 Defaults (static cursor, zero animation)

```rust
impl Default for Cursor {
    fn default() -> Self {
        Self {
            character: "█".to_string(),
            visibility: SignalOrFloat::Static(1.0),
            blink: CursorBlink { interval_ms: SignalOrFloat::Static(0.0) },
            grow_in: GrowIn::noop(),   // mode = Never, durations = 0
            wake: Wake::noop(),        // mode = Off, decay_seconds = 0
        }
    }
}
```

### 3.3 Convenience constructors

```rust
Cursor::block()      // "█" + defaults
Cursor::underscore() // "_" + defaults
Cursor::pipe()       // "|" + defaults
Cursor::caret()      // "▌" + defaults
Cursor::simple(ch)   // single-char custom + defaults

Cursor::with_grow_in(self, duration_ms)          // GrowInMode::Once, direction Up
Cursor::with_wake_tint(self, seconds, max_cells) // WakeMode::Tint
Cursor::with_wake_ghost(self, seconds, max_cells)// WakeMode::Ghost
```

### 3.4 JSON shape

All sub-blocks optional; missing fields default to no-ops.

```json
{
  "character": "█",
  "visibility": 1.0,
  "blink":   { "interval_ms": 0 },
  "grow_in": { "mode": "once", "direction": "up", "duration_ms": 180, "grow_out_ms": 0, "curve": 1.0 },
  "wake":    { "mode": "tint", "decay_seconds": 1.8, "max_cells": 8, "curve": 1.0, "tint": { "warm": true } }
}
```

---

## 4. Animation Mechanics

### 4.1 Grow-in state machine

```
Hidden ──(effective_visibility crosses >0)──▶ GrowingIn(t)
GrowingIn(t) ──(t ≥ duration_ms)──▶ Visible
Visible ──(effective_visibility crosses 0)──▶ GrowingOut(t)
GrowingOut(t) ──(t ≥ grow_out_ms)──▶ Hidden
```

Where `effective_visibility = cursor.visibility.eval(now) × consumer_phase × blink_phase`.

### 4.2 Progress → glyph mapping

`fnc_cursor_grow_in_glyph(base: char, progress: f32, direction) → (glyph, alpha)`

- **Block cursor `█`, direction Up:** progress bucketed into 9 steps → `[invisible, ▁, ▂, ▃, ▄, ▅, ▆, ▇, █]`.
- **Block cursor, direction Down:** upper-block equivalents, 9 steps ending at `█`.
- **Block cursor, direction Center:** 3-step expansion `▄` → `▆` → `█`.
- **Non-block cursor** (`|`, `_`, `◆`, `▌`): glyph is always the base; only alpha animates.

Alpha comes from `grow_in.curve.eval(progress)`.

### 4.3 Grow-in mode semantics

| Mode        | Behavior                                                                      |
|-------------|-------------------------------------------------------------------------------|
| `Never`     | Ignored. Cursor snaps to Visible on show, Hidden on hide.                     |
| `Once`      | Fires on first 0→1 visibility transition per `CursorState` lifetime. Later transitions snap. |
| `EveryShow` | Fires on every 0→1 visibility transition. Interacts with blink — see E6.      |

### 4.4 Wake mechanics

On each `fnc_advance_cursor`:
1. If `new_pos != state.position` and old was `Some`, push `(old_pos, now)` into history ring.
2. Age out entries where `now - first_seen > decay_seconds`.
3. Cap: drop oldest if `len > max_cells` and `max_cells > 0`.
4. Update `state.position = new_pos`.

On each `fnc_render_cursor`:
- For each history entry: `age_t = (now - first_seen) / decay_seconds`, `alpha = wake.curve.eval(age_t)`.
- Emit paint op:
  - `Tint` mode: `{ pos, tint_color, alpha }` — compositor blends tint with glyph beneath.
  - `Ghost` mode: `{ pos, glyph: cursor.character, tint_color, alpha }` — compositor overlays fading cursor glyph.

### 4.5 Clock source

Wall-clock ms from the compositor (`now`, `dt` passed in per frame). No per-frame counters. Pausing rendering and resuming produces correct elapsed time, not jump-scare catch-up.

---

## 5. Integration With `TypewriterCursor`

### 5.1 Post-refactor shape

```rust
pub struct TypewriterCursor {
    #[serde(flatten)]
    pub cursor: Cursor,
    /// Typewriter-specific visibility during reveal (0..1).
    pub show_while_typing: SignalOrFloat,
    /// Typewriter-specific visibility after reveal completes (0..1).
    pub show_after_complete: SignalOrFloat,
}
```

### 5.2 Typewriter per-frame drive

```rust
let base_visibility  = tcursor.cursor.visibility.eval(now);
let phase_visibility = match reveal_phase {
    Revealing => tcursor.show_while_typing.eval(now),
    Complete  => tcursor.show_after_complete.eval(now),
};
let effective_visibility = base_visibility * phase_visibility;
// Grow-in state machine watches effective_visibility for 0→1 and 1→0.

fnc_advance_cursor(&mut state, &tcursor.cursor, cursor_pos, now, dt);
let paint_ops = fnc_render_cursor(&state, &tcursor.cursor, now);
```

No behavior change when `cursor.visibility` stays at default `Static(1.0)`.

### 5.3 Backward compatibility

- `#[serde(flatten)]` keeps old JSON shape working — `character`, `blink_interval` lift from the flattened `Cursor`; `show_while_typing` / `show_after_complete` stay on `TypewriterCursor`.
- `#[serde(alias = "blink_interval")]` on `CursorBlink.interval_ms` accepts the old field name. Canonical serialized name becomes `blink.interval_ms`. Old name still documented as accepted.
- All new fields (`grow_in`, `wake`, `visibility`) default to no-ops.
- Existing `TypewriterCursor::block()` / `::underscore()` / `::pipe()` / `::caret()` / `::simple()` delegate to `Cursor` equivalents; `show_while_typing`/`show_after_complete` default to `1.0`. Rendering identical to v1.1.0.
- Frozen v1.1.0 JSON regression test asserts semantic equivalence.

### 5.4 Version bumps

- `tui-vfx-content`: MINOR (new public API, no breaking changes).
- `TypewriterCursor`: 1.1.0 → 2.0.0 (Rust struct layout changes, though serde-compatible).
- New cursor files: start at 0.1.0.

### 5.5 Future consumers (out of scope for this spec)

The `Cursor` primitive is ready for standalone overlay, editor caret, and other content transformers. Each is a separate spec.

---

## 6. Edge Cases & Behavior Rules

Each rule has a paired test.

| ID   | Case                                                             | Behavior                                                                                                                  |
|------|------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------|
| E1   | Cursor teleports (jump > 1 cell in one advance)                  | History records destination only. No intermediate interpolation.                                                          |
| E2   | Cursor stays on same cell across many frames                     | No new history entry. Stationary cursor produces no trail growth.                                                         |
| E3   | Cursor moves back onto a cell already in its trail               | Old entry for that cell removed, new entry inserted at current time. No double-brightness.                                |
| E4   | Character changes mid-grow-in                                    | Progress preserved, renders against new base. Non-block new character ⇒ alpha-only from that frame.                       |
| E5   | Position changes mid-grow-in                                     | Progress preserved at new position. No stutter on typewriter advance.                                                     |
| E6   | Rapid blink + `GrowInMode::EveryShow`                            | Grow-in fires every unblink as configured. Rustdoc warns of "wobbly" cursor. No silent clamp.                             |
| E7   | Wake while cursor is hidden (blink off / visibility = 0)         | Trail keeps decaying independently. Trail represents where cursor *was*.                                                  |
| E8   | Out-of-bounds trail cells                                        | Emitted as-is. Compositor handles clipping. Cursor module has no viewport knowledge.                                      |
| E9   | Wide-glyph / emoji under cursor                                  | Only logical cell gets trail entry. Wide-glyph awareness out of scope; limitation flagged in rustdoc.                     |
| E10  | Empty `character: ""`                                            | Cursor suppressed. No primary op, no new trail entries. Existing trail continues to decay.                                |
| E11  | `decay_seconds = 0` with WakeMode ≠ Off                          | Treated as `WakeMode::Off`. Canonical "disable wake" alongside `mode = Off`.                                              |
| E12  | `duration_ms = 0` with GrowInMode ≠ Never                        | Treated as `GrowInMode::Never`. Snap-to-visible on show.                                                                  |
| E13  | Signal returns NaN / Infinity                                    | Clamp to 0 for alpha/progress/durations. Matches existing shader defensive handling in this repo.                         |
| E14  | Deserializing a frozen pre-2.0 TypewriterCursor JSON             | Parses unchanged; renders identically. Covered by `test_typewriter_cursor_backcompat_json`.                               |
| E15  | Multiple cursors in one app                                      | Each `(Cursor, CursorState)` pair is independent. No shared globals.                                                      |

---

## 7. Testing

TDD, peer test files mirror `/src/`:

```
crates/tui-vfx-content/tests/cursor/
  test_cls_cursor.rs                    # Defaults, convenience ctors, serde round-trip
  test_cls_cursor_blink.rs              # interval_ms = 0 → always visible; phase timing
  test_cls_cursor_grow_in.rs            # Mode state machine, direction, duration = 0 snaps
  test_cls_cursor_wake.rs               # Mode variants, decay_seconds = 0 disables
  test_cls_cursor_state.rs              # Advance-only behavior, no clock assumptions
  test_fnc_advance_cursor.rs            # E1–E3, E5, E7 position & trail mechanics
  test_fnc_render_cursor.rs             # Paint-op shape, tint vs ghost, no clipping
  test_fnc_cursor_grow_in_glyph.rs      # Up/Down/Center mapping, non-block alpha-only (E4)
```

Typewriter integration (extends existing):

```
crates/tui-vfx-content/tests/test_typewriter_cursor.rs
  - Existing tests pass unchanged
  - test_typewriter_cursor_backcompat_json                # Frozen v1.1.0 JSON parses identically (E14)
  - test_typewriter_cursor_grow_in_during_reveal
  - test_typewriter_cursor_wake_trails_revealed_text
  - test_typewriter_cursor_defaults_behave_like_v1_1_0    # Golden regression
```

Each edge case in Section 6 has a named test.

---

## 8. Documentation Deliverables

### 8.1 Hand-maintained docs to update

- `docs/CAPABILITIES_REFERENCE.md` — new "Cursor primitive" section; update TypewriterCursor section to reference it; note `blink_interval` → `blink.interval_ms` alias.
- `docs/API_HAND.md` — add Cursor primitive entry.
- `docs/templates/api_docs.toml` — add `[specs.Cursor]`, `[specs.CursorBlink]`, `[specs.GrowIn]`, `[specs.Wake]`; update `[specs.ContentEffect.types.TypewriterCursor]` to reflect flatten.
- `docs/INDEX.md` — link new cursor section.

### 8.2 Generated docs to regen (post-implementation)

- `docs/generated/API.md`
- `docs/generated/CAPABILITIES.md`
- `docs/generated/capabilities.json`
- `docs/generated/effect_schemas.json`
- `docs/generated/ai-context.md`
- `docs/generated/recipes_validation.md` / `.json`

Regen command (confirmed during implementation — likely `cargo xtask docs` or equivalent). Spec requires the regen step before the branch ships; CI freshness check must pass.

### 8.3 Rustdoc requirements

- Every new struct, enum, variant, and field has a doc comment.
- `Cursor`, `GrowIn`, `Wake` each have a module-level example showing the do-nothing default plus one opt-in example.
- `GrowInMode::EveryShow` documents the blink interaction (E6).
- `WakeMode::Ghost` documents the wide-glyph limitation (E9).
- `fnc_advance_cursor` / `fnc_render_cursor` document the caller contract (who owns position, when to call, clock source).
- `TypewriterCursor` rustdoc updated: flatten note, backward-compat guarantee, pointer to `Cursor` for richer behavior.

### 8.4 OFPF envelopes

Every new source, test, and markdown file gets the standard `<FILE>` / `<VERS>` / `<WCTX>` / `<CLOG>` header + footer.

---

## 9. Out of Scope

- Standalone cursor overlay (separate spec using this primitive).
- Editor-style multi-cursor with selection (separate spec).
- Integrating Cursor into other content transformers (split-flap, scramble, etc. — separate specs).
- Wide-glyph-aware wake painting (requires viewport query this primitive doesn't have; flagged in rustdoc as E9 limitation).
- Interpolated trail cells on teleport (explicit E1 decision: destination only).

---

## 10. Success Criteria

1. `Cursor` primitive shipped with grow-in and wake capabilities, defaults render identical to today's static cursor.
2. `TypewriterCursor` refactored to compose `Cursor` via `#[serde(flatten)]`; every existing recipe and Rust caller renders identically.
3. All E1–E15 edge cases have named passing tests.
4. All rustdoc items have doc comments; module-level examples for `Cursor`, `GrowIn`, `Wake`.
5. Hand-maintained docs updated; generated docs regenerated; CI freshness check green.
6. Frozen v1.1.0 JSON regression test passes (E14).

<!-- <FILE>docs/superpowers/specs/2026-04-17-cursor-primitive-design.md</FILE> - <DESC>Design spec for the general Cursor primitive (grow-in + wake)</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
