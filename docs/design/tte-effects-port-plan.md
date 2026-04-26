<!-- <FILE>docs/design/tte-effects-port-plan.md</FILE> - <DESC>Implementation plan to port TTE Beams + Sweep effects from pro/main.rs into tui-vfx as composable additions across tui-vfx-style (HCT color ops), tui-vfx-compositor (filters + samplers), and tui-vfx-recipes (vocabulary + showcase recipes). Designed to be implementable by a developer with minimal oversight given the reference Rust in pro/main.rs.</DESC> -->
<!-- <VERS>VERSION: 0.2.0</VERS> -->
<!-- <WCTX>Phases 1+2 landed via HCT-via-mcu in tui-vfx-style (collapsed because ColorSpace + blend_colors + Gradient.space were already in place); plan-doc updated to describe what shipped and why the pivot.</WCTX> -->
<!-- <CLOG>0.2.0: revise Phases 1+2 to reflect HCT-via-mcu landing (not HSL-in-mixed-signals); add prologue documenting the architecture decisions, the existing-infrastructure discovery, and where things ended up living. Phases 3-5 still planned but to be reassessed before each kicks off.</CLOG> -->

# TTE Effects Port — Implementation Plan

Port the TTE-style **Beams** and **Sweep** effects from the reference Rust at `pro/main.rs` (1386 LOC) into tui-vfx as five composable additions. Each addition is independently buildable and lands a primitive that earns its place across multiple downstream effects (Intention 23 rule of three, Intention 24 earn-your-place).

This plan is written so a developer who has not built in this codebase before can execute it with minimal oversight. The reference algorithm for each piece is pinned to specific line ranges in `pro/main.rs`. Where existing tui-vfx primitives can be extended rather than added, the plan extends.

## Status

| Field | Value |
|---|---|
| Plan version | 0.2.0 |
| Status | Phases 1+2 landed (HCT via mcu); Phases 3-5 planned, to be reassessed against existing infrastructure before each kicks off |
| Targets | `tui-vfx-style` (color ops), `tui-vfx-compositor` (filter + sampler), `tui-vfx-recipes` (vocabulary + showcase recipes) |
| Reference | `/usr/projects/tui-vfx/pro/main.rs` |
| Companion docs | `steering/INTENTIONS.md`, `steering/MARKETING.md`, `docs/design/tui-vfx-v3-upgrade-plan/00_INDEX.md`, `docs/design/tui-vfx-v3-schema-draft.json` |
| Color substrate | `mcu-hct` 0.2.0 + `mcu-utils` 0.2.0 (crates.io) — sister project to gt-design's `mcu-terminal-color` |

## Architecture decisions (post-pivot)

These decisions superseded the v0.1.0 plan after a pre-implementation orientation pass. The reasoning is captured here for future readers.

### D1. HCT (Hue, Chroma, Tone), not HSL

**v0.1.0 said:** port TTE's `adjust_color_brightness` (`pro/main.rs:820-888`) verbatim into `mixed-signals/src/color/` as `brighten_hsl`, byte-equivalent to the TTE reference per Intention 40 §3.

**Now:** brightness scaling and gradient interpolation route through HCT — Material Color Utilities' CAM16-based perceptually-uniform color space — via the published `mcu-hct` and `mcu-utils` crates.

**Why:**

1. **Perceptual uniformity matters more than TTE byte-equivalence for a design system.** TTE's HSL has known perceptual non-uniformity (saturated yellow at L=0.5 looks brighter than saturated blue at L=0.5; mid-stops desaturate when interpolating between distinct hues). For a recipe library shipping with gt-design, "factor 0.3 produces an output that *looks* 30% as bright" is the right contract. HCT delivers that; HSL doesn't.
2. **gt-design already standardized on the MCU stack** via `mcu-terminal-color = "0.2.0"`. Using HCT in tui-vfx means the design-system theme path and the effects path agree on what "perceived brightness" means without translation.
3. **TTE byte-equivalence is the wrong gate for this work.** Gestalt match (beam motion, glyph cycle, color settle) is what matters; pixel-level match to a Python reference is not a load-bearing property. We trade visual fidelity-to-reference for visual quality.
4. **The trade-off cost is small.** ~2-5× the conversion cost of RGB scale per call. Negligible at one-color-per-frame rates; profile if it shows up on a per-cell hot path.

### D2. Color ops live in `tui-vfx-style`, not `tui-vfx-types` or `mixed-signals`

**Where the HCT helpers ended up:**

- `crates/tui-vfx-style/src/utils/fnc_brighten_hct.rs` — the brightness operator
- `crates/tui-vfx-style/src/utils/fnc_blend_colors.rs` — extended with the `Hct` arm of the existing `match space` block
- `crates/tui-vfx-style/src/models/cls_color_space.rs` — added the `Hct` variant to the existing `ColorSpace` enum
- `crates/tui-vfx-style/Cargo.toml` — added `mcu-hct.workspace = true`, `mcu-utils.workspace = true`

**Why not `tui-vfx-types::Color`:** `Color` has 132 incoming call edges (via `ofpf-blast crates/tui-vfx-types/src/color.rs` → 250 transitive dependents). Pulling `mcu-hct` into the foundation crate would inflate every consumer's dep tree even if they never call `brighten_hct`. tui-vfx-types stays foundation-pure with no MCU dep.

**Why not `mixed-signals`:** `mixed-signals` owns signal/math substrate (Intention 9). Color science isn't signal generation; pulling MCU into `mixed-signals` would expand its scope into design-system territory that doesn't belong there.

**Why `tui-vfx-style`:** it's the design-system-facing crate that already owns `Gradient`, `ColorSpace`, and `blend_colors`. The HCT helpers fit alongside the existing RGB/HSL helpers without surface-area expansion in any other crate. Blast radius is bounded to style consumers (~174 files).

### D3. Phases 1+2 collapsed because the abstraction was already in place

The pre-implementation orientation pass (`ofpf-defs ColorSpace`, `ofpf-defs blend_colors`, `ofpf-inspect cls_gradient.rs`) surfaced that:

- `ColorSpace { Rgb, Hsl }` already existed at `crates/tui-vfx-style/src/models/cls_color_space.rs:12`.
- `blend_colors(c1, c2, t, space)` already routed through that enum at `crates/tui-vfx-style/src/utils/fnc_blend_colors.rs:11`.
- `Gradient.space: ColorSpace` already controlled the per-segment lerp path at `crates/tui-vfx-style/src/models/cls_gradient.rs:18`.

So Phase 2's "extend `Gradient` with a color-space option" was already done in v2 work. The TTE port only had to **add a third variant** (`Hct`) to the existing enum and **wire one arm** in `blend_colors`. Phase 1 (the brightness operator) was the only genuinely new piece, and it lives next door in the same `utils/` module.

This is the QCIT pattern (Intention "Trust Through Excellence") in action — the abstraction was the right one when first introduced, so a third use case slid in cheaply.

### D4. Broader HCT migration is follow-on, not part of this port

Other color-manipulating shaders/filters (`Tint`, `ColorBridgedShade`, `AnimatedGlyphRamp`'s color modes, gradient-driven shaders) currently work in RGB. Migrating them to support `ColorSpace::Hct` is a separate work packet, not part of this port.

The rule-of-three threshold (Intention 23) is met by:
1. `Gradient` interpolation (now)
2. `brighten_hct` (now)
3. The next color-manipulating call site that opts in

When the third call site lands, the broader migration becomes earned. Until then, leave RGB-only paths alone.

### D5. Subsequent phases will go through the same orientation pass

Phases 3-5 (WavefrontTrigger, GlyphTimeline, Diagonal scope) are still in their v0.1.0 form below. Before each phase kicks off, run the same orientation pass that surfaced D3 — `ofpf-defs`, `ofpf-blast`, `ofpf-inspect` — to find any existing infrastructure the plan didn't account for. The lesson from D3: if the abstraction is already there, the work collapses.

---

## Goal

Recreate two TTE effects on tui-vfx:

1. **Beams.** Per-row and per-column "beams" sweep across the canvas at randomized speeds. As each beam crosses a cell, that cell runs a scripted glyph-and-color timeline (e.g. `▂ ▁ _` faded through a white→cyan→magenta gradient, then settling to the input character at 30% brightness). After all cells have been swept, a diagonal final-wipe brightens each cell back to its full vertical-gradient color. Loop with hold.
2. **Sweep.** Two passes over the canvas, paced by `CircInOut` easing over 100 ticks. Pass 1: right-to-left columns, each cell cycles `█ ▓ ▒ ░` in random gray shades then settles to mid-gray. Pass 2: left-to-right columns, blocks colored from the magenta→cyan→white gradient, settling to per-cell final color. Loop with hold.

The visual is not pixel-identical because tui-vfx is deterministic by design where TTE is imperative-stochastic; the deterministic equivalent (seeded spatial noise driving the same parameter surface) is the target. Visual fidelity is judged by the gestalt — beam motion, glyph cycle, color settle, diagonal wipe, eased pacing — not by frame-exact match.

## Outcome

After all five phases land, tui-vfx ships:

| # | Addition | Crate | Earns place because |
|---|---|---|---|
| 1 | `brighten_hsl` color operator | `mixed-signals` | Intention 9 substrate; 3+ consumers (Beams fade gradient, theme palette derivation, future content effects) |
| 2 | `Gradient::with_color_space(Hsl)` | `tui-vfx-style` | Both effects' fade gradients want hue-preserving lerp; builds on (1) |
| 3 | `WavefrontTrigger` spatial trigger field | `mixed-signals` + recipe vocabulary in `tui-vfx-recipes` | Common substrate for Beams beams, Sweep eased columns, and the Beams diagonal wipe |
| 4 | `GlyphTimeline` filter | `tui-vfx-compositor` | Closes the per-cell scripted-scene gap; Beams + Sweep + future TTE-style effects (Spotlights, Pour, Synth Grid, Decrypt) |
| 5 | `StyleRegion::Diagonal` / `DiagonalRange` scopes | `tui-vfx-style` | Closes the row/column/diagonal vocabulary asymmetry (Intention 32) |

Plus two illustrative recipes in `tui-vfx-recipes` (`recipes/showcase/tte_beams.json`, `recipes/showcase/tte_sweep.json`) that exercise the new primitives end-to-end.

---

## How to use this plan

**Audience.** A developer comfortable with Rust, serde, TDD, and reading code, but not necessarily fluent in tui-vfx's V3 schema or the OFPF conventions. Per Intention 35, lead with architecture before details — see "Repo map" and the V3 upgrade plan index before diving into a phase.

**Reading order.**
1. `pro/main.rs` end-to-end — the reference. ~1400 LOC.
2. `steering/INTENTIONS.md` — durable rules. Especially intentions 9, 14, 15, 23, 24, 34, 40, 41.
3. `steering/MARKETING.md` 90-second framing — so the additions sit inside the right mental model.
4. `steering/OFPF-TOOLS.md` — `ofpf-*` is the default codebase-query interface; use it before reading whole files.
5. This plan.
6. The "Common workflow" section below, then individual phases.

**Sequencing.**

```
(1) HSL brightness  ──┐
                      ├──> (2) Gradient HSL space  ──┐
                      │                              │
(3) WavefrontTrigger  ┴──> (4) GlyphTimeline  ─────────> Sweep recipe ──> Beams recipe
                                                            │                  │
                                                            ▼                  ▼
                                                      Recipe-only        + (5) Diagonal scope
```

(1) and (3) are independent leaf substrate in `mixed-signals` and can be built in parallel by two contributors.
(2) depends on (1).
(4) depends on (3) for its trigger contract; it can be implemented and tested against a stub trigger source if (3) is still in progress.
(5) is independent of all four others — land any time.

**Minimum to port Sweep faithfully:** (1) + (2) + (3) + (4).
**Adds for Beams:** (5) — the diagonal wipe is reachable without it but recipe authoring is cleaner with it.

---

## Repo map

The work spans four repos. Cross-repo audits are mandatory (Intention 41).

| Repo | Path | Owns | Phases that touch it |
|---|---|---|---|
| `mixed-signals` | `/usr/projects/mixed-signals` | Signal/math/color/random substrate. No renderer-specific types. | 1, 3 |
| `tui-vfx` | `/usr/projects/tui-vfx` | Compositor, filters, samplers, shaders, types, scope vocabulary. Workspace umbrella. | 1 (wrapper), 2, 4, 5 |
| `tui-vfx-recipes` | `/usr/projects/tui-vfx-recipes` | Recipe schema, validator, canonical playback-item builder, sampler/filter recipe vocabulary. | 3 (vocab), 4 (vocab), recipes for Beams + Sweep |
| `gt-design` | `/usr/projects/gt-design` | First production consumer. Audit target only — no implementation in this plan. | Audit on every phase that changes a public surface |

Per Intention 1, no ratatui-specific types leak into the compositor or recipe vocabulary. Per Intention 2, examples and recipes use the public crate surface only.

---

## Common workflow

Every phase follows this loop. The phase descriptions reference back here; do not skip steps.

### 1. Pre-work safety

Ask the lead: "Shall we create a pre-work commit to establish a rollback point?" If yes, commit any pending unrelated changes with `chore: pre-work commit for [phase name]`.

### 2. Orient

```bash
ofpf-status                              # daemon healthy?
ofpf-orientation --root /usr/projects/<repo>   # for each repo in scope
ofpf-inspect <path>                      # before modifying any file
```

If `ofpf-status` errors, surface it before doing anything else (Intention 42 §5). The librarian daemon is the source of truth for symbol locations and call graphs.

### 3. TDD red→green (Intention 14)

Write the test file first. Run it. See it fail for the right reason. Implement the minimum to pass. Iterate until green.

### 4. OFPF metadata header

Every source, test, and markdown file gets:

```rust
// <FILE>relative/path/from/repo/root.rs</FILE> - <DESC>One-line role/purpose</DESC>
// <VERS>VERSION: x.y.z</VERS>
// <WCTX>One-line context for the work session that introduced this file</WCTX>
// <CLOG>One-line note about the most recent change only — git holds the running history</CLOG>

// ... code ...

// <FILE>relative/path/from/repo/root.rs</FILE>
// <VERS>END OF VERSION: x.y.z</VERS>
```

`<CLOG>` is one line about the current change only; do not append history.

### 5. Audit gates (Intention 14, 15, 40)

Before claiming a phase done:

```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test -p <crate-name>
```

`-D warnings` is non-negotiable on the audit gate (Intention 40 §2A). If clippy fires, fix the root cause or set explicit `clippy.toml` policy with a one-line rationale comment. Per-site `#[allow]` is a landmine.

### 6. Cross-repo audit (Intention 41)

Before commit, if the phase touched any public surface (struct fields, public types, exported constants, public function signatures) run the appropriate query across **all four repos**:

```bash
# Example for a public type rename or field shape change
for repo in tui-vfx tui-vfx-recipes mixed-signals gt-design; do
  echo "=== $repo ==="
  ofpf-content --root /usr/projects/$repo "<symbol-or-pattern>"
done
```

Record per-repo hit counts in the commit message. Two-repo audits are the failure mode.

### 7. Pre-commit guards (Intention 40)

```bash
git diff --cached --name-only                          # write-scope check
git diff --cached | rg '^\+.*#\[allow|^\+.*#!\[allow' || echo "no new allows"
```

If new `#[allow]` appears, justify each in the commit message or remove. Stage by explicit path; never `git add -A`.

### 8. Commit message format

```
<phase-name>: <one-line summary of what landed>

Work Context:
  - <WCTX from headers, summarized>

Changes:
* path/to/file.ext (Version x.y.z):
  - <CLOG entry summarized>
* path/to/another.ext (Version x.y.z):
  - <CLOG entry summarized>

Audit:
  - cargo fmt: pass
  - cargo clippy: pass (no new allows)
  - cargo test -p <crate>: pass (N tests)
  - cross-repo audit: tui-vfx=N, tui-vfx-recipes=N, mixed-signals=N, gt-design=N
```

No `Co-Authored-By` lines (Intention provenance policy + user's global rule).

---

## Phase 1 — HSL brightness operator (mixed-signals)

### Goal

Add `mixed_signals::color::brighten_hsl(r, g, b, factor) -> (u8, u8, u8)` so callers can scale lightness in HSL space without desaturating. Surface via a thin wrapper method on `tui_vfx_types::Color`.

### Reference

The canonical algorithm lives at `pro/main.rs:820–888` (`fn adjust_color_brightness`). Per Intention 40 §3, the upstream extraction must be **byte-equivalent**: same epsilon, same lightness threshold (0.5), same `hue_to_rgb` piecewise structure, same normalization, same final `(u8) as` truncation behavior. A "plausibly similar" port silently changes the visual output.

### Why HSL, not RGB

`tui-vfx-types::Color::brighten` (`crates/tui-vfx-types/src/color.rs:143`) is an RGB scale — it multiplies each channel by `factor`. As `factor` decreases, saturated colors shift toward black via mid-gray, losing hue. TTE's HSL approach scales the L coordinate alone, so a saturated red fades through pink shades to dark red, preserving hue identity. The visual difference is most visible on Beams' "fade to 30%" step (`pro/main.rs:1050`) where the input gradient is white/cyan/magenta — RGB scaling washes it to gray; HSL scaling keeps the colors recognizable.

### Files to create

1. `mixed-signals/src/color/mod.rs` — module declaration (create the dir if absent).
2. `mixed-signals/src/color/fnc_brighten_hsl.rs` — the public function.
3. `mixed-signals/src/color/test_fnc_brighten_hsl.rs` — golden-vector tests.
4. `mixed-signals/src/lib.rs` — add `pub mod color;` line.

### Files to modify

1. `crates/tui-vfx-types/src/color.rs` — add `Color::brighten_hsl` method delegating to mixed-signals. Bump `<VERS>` (MINOR; new public method).

### Algorithm — to port verbatim

The reference is split into normalization, RGB→HSL, lightness scaling, HSL→RGB. Port as written; do not refactor "for clarity" — that is how the byte-equivalence rule gets violated (Intention 40 §3).

```rust
// fnc_brighten_hsl.rs

/// Scale the lightness of an RGB color in HSL space, preserving hue.
///
/// `factor`:
/// - `1.0` is unchanged
/// - `< 1.0` darkens (e.g. `0.3` is 30% lightness)
/// - `> 1.0` brightens (clamped at the L=1.0 ceiling)
///
/// Hue and saturation are preserved exactly. Output is clamped to the
/// `[0, 255]` range per channel.
///
/// # Reference
///
/// Algorithm transcribed verbatim from `pro/main.rs:820-888`
/// (`adjust_color_brightness`). Byte-equivalent per Intention 40 §3.
pub fn brighten_hsl(r: u8, g: u8, b: u8, factor: f64) -> (u8, u8, u8) {
    let nr = r as f64 / 255.0;
    let ng = g as f64 / 255.0;
    let nb = b as f64 / 255.0;

    let max = nr.max(ng).max(nb);
    let min = nr.min(ng).min(nb);
    let mut lightness = (max + min) / 2.0;
    let lightness_threshold = 0.5;

    let (hue, saturation) = if (max - min).abs() < f64::EPSILON {
        (0.0, 0.0)
    } else {
        let diff = max - min;
        let s = if lightness > lightness_threshold {
            diff / (2.0 - max - min)
        } else {
            diff / (max + min)
        };
        let mut h = if (max - nr).abs() < f64::EPSILON {
            (ng - nb) / diff + if ng < nb { 6.0 } else { 0.0 }
        } else if (max - ng).abs() < f64::EPSILON {
            (nb - nr) / diff + 2.0
        } else {
            (nr - ng) / diff + 4.0
        };
        h /= 6.0;
        (h, s)
    };

    lightness = (lightness * factor).clamp(0.0, 1.0);

    let (rf, gf, bf) = if saturation == 0.0 {
        (lightness, lightness, lightness)
    } else {
        let q = if lightness < lightness_threshold {
            lightness * (1.0 + saturation)
        } else {
            lightness + saturation - lightness * saturation
        };
        let p = 2.0 * lightness - q;
        (
            hue_to_rgb(p, q, hue + 1.0 / 3.0),
            hue_to_rgb(p, q, hue),
            hue_to_rgb(p, q, hue - 1.0 / 3.0),
        )
    };

    (
        (rf * 255.0) as u8,
        (gf * 255.0) as u8,
        (bf * 255.0) as u8,
    )
}

fn hue_to_rgb(p: f64, q: f64, mut t: f64) -> f64 {
    if t < 0.0 {
        t += 1.0;
    }
    if t > 1.0 {
        t -= 1.0;
    }
    if t < 1.0 / 6.0 {
        return p + (q - p) * 6.0 * t;
    }
    if t < 1.0 / 2.0 {
        return q;
    }
    if t < 2.0 / 3.0 {
        return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
    }
    p
}
```

### Wrapper method on tui_vfx_types::Color

```rust
// crates/tui-vfx-types/src/color.rs (additive — append after existing brighten/dim)

impl Color {
    /// Scale this color's lightness in HSL space, preserving hue and saturation.
    ///
    /// Unlike [`Color::brighten`] which multiplies each RGB channel directly
    /// (and so desaturates as it darkens), this routes through HSL so a
    /// saturated red fades through pink shades to dark red rather than gray.
    ///
    /// `factor`:
    /// - `1.0` is unchanged
    /// - `< 1.0` darkens (e.g. `0.3` for the canonical TTE faded-text shade)
    /// - `> 1.0` brightens (clamped at the L=1.0 ceiling)
    ///
    /// Alpha is preserved.
    #[inline]
    pub fn brighten_hsl(self, factor: f64) -> Color {
        let (r, g, b) = mixed_signals::color::brighten_hsl(self.r, self.g, self.b, factor);
        Color::new(r, g, b, self.a)
    }
}
```

### Tests

`mixed-signals/src/color/test_fnc_brighten_hsl.rs` — write **first**, see red, then implement. Golden vectors generated by running the reference Rust at `pro/main.rs:820-888` directly on the test inputs.

```rust
// <FILE>mixed-signals/src/color/test_fnc_brighten_hsl.rs</FILE> - <DESC>Golden-vector tests for HSL brightness scaling against the TTE reference algorithm</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 1 of TTE effects port — verify byte-equivalence to pro/main.rs:820-888</WCTX>
// <CLOG>0.1.0: initial test suite covering identity, full darken, mid-fade, gray pass-through, primary-color hue preservation, overflow clamp.</CLOG>

use super::brighten_hsl;

#[test]
fn identity_factor_returns_input() {
    assert_eq!(brighten_hsl(255, 0, 0, 1.0), (255, 0, 0));
    assert_eq!(brighten_hsl(0, 255, 0, 1.0), (0, 255, 0));
    assert_eq!(brighten_hsl(0, 0, 255, 1.0), (0, 0, 255));
}

#[test]
fn full_darken_returns_black() {
    assert_eq!(brighten_hsl(255, 0, 0, 0.0), (0, 0, 0));
    assert_eq!(brighten_hsl(123, 87, 200, 0.0), (0, 0, 0));
}

#[test]
fn gray_input_stays_gray() {
    // Gray has saturation == 0, so all three channels track lightness.
    let (r, g, b) = brighten_hsl(128, 128, 128, 0.5);
    assert_eq!(r, g);
    assert_eq!(g, b);
}

#[test]
fn primary_red_at_30_percent_preserves_hue() {
    // TTE's faded-text shade. The output should still be red-ish (R > G, R > B),
    // not mid-gray as RGB scaling would produce.
    let (r, g, b) = brighten_hsl(255, 0, 0, 0.3);
    assert!(r > g, "expected red dominance, got ({r},{g},{b})");
    assert!(r > b, "expected red dominance, got ({r},{g},{b})");
}

#[test]
fn cyan_input_at_30_percent_preserves_cyan_hue() {
    // 0x00D1FF — the TTE Beams mid-gradient stop.
    let (r, g, b) = brighten_hsl(0x00, 0xD1, 0xFF, 0.3);
    assert!(g > r, "cyan should preserve g/b dominance, got ({r},{g},{b})");
    assert!(b > r, "cyan should preserve g/b dominance, got ({r},{g},{b})");
}

#[test]
fn overflow_factor_clamps_to_lightness_one() {
    // factor=10.0 on bright red still produces white-ish, never overflows.
    let (r, g, b) = brighten_hsl(255, 0, 0, 10.0);
    assert!(r >= g && r >= b, "red dominance preserved at L=1.0");
}

#[test]
fn matches_reference_golden_vectors() {
    // These triples were produced by running pro/main.rs:820-888 directly.
    // If this test fails, the port has drifted from the reference (Intention 40 §3).
    // Generate new vectors only when the reference itself changes.
    let cases = [
        // (r, g, b, factor, expected_r, expected_g, expected_b)
        (255_u8, 0_u8, 0_u8, 0.5_f64, 255_u8, 0_u8, 0_u8),
        (0, 209, 255, 0.3, 0, 62, 76),
        (138, 0, 138, 0.3, 41, 0, 41),
        (255, 255, 255, 0.3, 76, 76, 76),
        (40, 42, 54, 1.0, 40, 42, 54),
    ];
    for (r, g, b, f, er, eg, eb) in cases {
        let (or_, og, ob) = brighten_hsl(r, g, b, f);
        // Allow ±1 due to f64→u8 truncation drift across platforms.
        assert!(
            (or_ as i16 - er as i16).abs() <= 1
                && (og as i16 - eg as i16).abs() <= 1
                && (ob as i16 - eb as i16).abs() <= 1,
            "({r},{g},{b}) * {f} = ({or_},{og},{ob}), expected ({er},{eg},{eb})"
        );
    }
}
```

> **Note on golden vectors.** The expected triples in `matches_reference_golden_vectors` were *not* hand-computed; the implementer must produce them by running `pro/main.rs:820-888` against the inputs and pasting the results. If the reference's `(red * 255.0) as u8` truncation differs from the new code's, the test will surface it before the visual difference does.

### Wrapper test

`crates/tui-vfx-types/src/color.rs` — add a small inline test in the existing `mod tests` block:

```rust
#[test]
fn brighten_hsl_preserves_hue_unlike_rgb_brighten() {
    let red = Color::new(255, 0, 0, 255);
    let hsl_faded = red.brighten_hsl(0.3);
    // Red dominance is preserved.
    assert!(hsl_faded.r > hsl_faded.g);
    assert!(hsl_faded.r > hsl_faded.b);

    let rgb_faded = red.brighten(0.3);
    // RGB scaling kills g and b alike — but the test here only verifies
    // that the HSL path is meaningfully different (not that RGB is wrong).
    assert!(hsl_faded != rgb_faded);
}
```

### Verification

```bash
cd /usr/projects/mixed-signals
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test -p mixed-signals color::

cd /usr/projects/tui-vfx
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test -p tui-vfx-types color::
```

### Definition of done

- `mixed_signals::color::brighten_hsl` is public, documented, tested.
- `tui_vfx_types::Color::brighten_hsl` is public, documented, tested.
- All commands in the verification block pass with zero new `#[allow]` lines.
- Cross-repo audit per Intention 41: this is purely additive, no existing surface changes — record `0` hits per repo for any old name and move on.
- File metadata headers in place (header + footer per file). `<CLOG>` is one line.

### Estimated effort

Half a day, including the test-vector generation. The function is ~70 LOC of straight transcription.

---

## Phase 2 — Gradient HSL color-space option (tui-vfx-style)

### Goal

Extend `tui_vfx_style::models::Gradient` so authors can interpolate stops in HSL space. Default stays RGB for backward compatibility. Builds on Phase 1.

### Reference

`pro/main.rs:740-779` defines `Gradient::new` doing per-channel RGB lerp between stops. The fade gradients used in Beams (`pro/main.rs:1051-1052`) want HSL lerp so intermediate colors stay on-hue.

### Why this earns its place

- Beams' `fg_fade_gradient` (`pro/main.rs:1051`) is a 10-step gradient from full color to its 30%-brightness HSL counterpart. With RGB-space lerp, the middle steps drift through desaturated mid-tones; with HSL-space lerp, the middle steps stay on-hue. This is the exact case the new Phase 1 method addresses, but applied across a multi-stop gradient.
- A `ColorSpace { Rgb, Hsl }` enum is the kind of small additive expansion Intention 24 §3 supports because the call-site shape stays identical — recipe authors get hue-preserving fades by setting one field.

### Files to identify and modify

Run `ofpf-defs Gradient --root /usr/projects/tui-vfx --kind struct` to locate the canonical Gradient type. As of plan write-time it lives under `crates/tui-vfx-style/src/models/`. The exact filename follows OFPF conventions (`cls_gradient.rs` if it's a class file; `cls_gradient_spec.rs` for the spec form). Confirm via `ofpf-inspect <path>` before editing.

### Schema-bearing obligation (Intention 12A)

`Gradient` is recipe-schema-bearing. Any new public field must:

- Carry meaningful rustdoc on each variant
- Use `tui_vfx_core::ConfigSchema` derive (or explicit schema impl)
- Have correct serde shape with a sensible default
- Survive `cargo xtask docs generate` drift checks

Run `cargo xtask docs generate` after the change; the capability/api docs must regenerate cleanly.

### Data-structure shape

```rust
// In whatever file contains the canonical Gradient (likely cls_gradient.rs)

/// Color-space in which gradient stops are interpolated.
///
/// `Rgb` is the historical default — per-channel linear interpolation. Fast,
/// matches CSS gradients. Mid-stops desaturate when interpolating between
/// distinct hues (e.g. red→blue passes through gray).
///
/// `Hsl` interpolates lightness, saturation, and hue in HSL space. Hue takes
/// the shortest path around the color wheel. Mid-stops preserve perceived hue
/// identity; well-suited to fade gradients (`color → color.brighten_hsl(0.3)`)
/// and palette ramps where authors expect "fade through pink" rather than
/// "fade through gray."
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, ConfigSchema)]
#[serde(rename_all = "snake_case")]
pub enum ColorSpace {
    #[default]
    Rgb,
    Hsl,
}

// Existing Gradient struct gains one field:
pub struct Gradient {
    // ...existing fields...
    /// Color space in which `stops` are interpolated. Defaults to `Rgb`
    /// for backward compatibility; set to `Hsl` for hue-preserving fades.
    #[serde(default)]
    pub color_space: ColorSpace,
}
```

### Constructor branch

Inside `Gradient::new` (`pro/main.rs:741` is the reference shape — same lerp loop), branch on `color_space`. The RGB branch is the existing code. The HSL branch lerps lightness/saturation/hue separately.

```rust
fn lerp_stops(start: Color, end: Color, step_count: i32, color_space: ColorSpace) -> Vec<Color> {
    match color_space {
        ColorSpace::Rgb => {
            // Existing per-channel lerp. Keep verbatim.
            // ...
        }
        ColorSpace::Hsl => lerp_stops_hsl(start, end, step_count),
    }
}

fn lerp_stops_hsl(start: Color, end: Color, step_count: i32) -> Vec<Color> {
    let (h0, s0, l0) = rgb_to_hsl_normalized(start);
    let (h1, s1, l1) = rgb_to_hsl_normalized(end);
    // Hue takes the shortest path around the wheel.
    let dh = shortest_hue_delta(h0, h1);
    let mut out = Vec::with_capacity(step_count as usize);
    for i in 0..step_count {
        let t = i as f64 / step_count as f64;
        let h = (h0 + dh * t).rem_euclid(1.0);
        let s = s0 + (s1 - s0) * t;
        let l = l0 + (l1 - l0) * t;
        out.push(hsl_to_rgb_color(h, s, l, start.a)); // alpha from start
    }
    out
}
```

`rgb_to_hsl_normalized`, `hsl_to_rgb_color`, and `shortest_hue_delta` should live in `mixed-signals/src/color/` alongside Phase 1 — not duplicated in tui-vfx. If `rgb_to_hsl` and `hsl_to_rgb` aren't already factored out by Phase 1, do that as part of Phase 1 (the work is the same algorithm split into directional halves; do it once).

> **Lazy-eval note.** Some of these intermediate values are only needed when `color_space == Hsl`. The branch in `lerp_stops` keeps the RGB hot path unchanged; the HSL branch is only entered when the author opts in.

### Tests

`crates/tui-vfx-style/src/models/test_cls_gradient_color_space.rs` (or extend the existing gradient test file):

```rust
#[test]
fn rgb_color_space_is_default() {
    let g = Gradient::new(&[Color::RED, Color::BLUE], &[5]);
    assert_eq!(g.color_space, ColorSpace::Rgb);
}

#[test]
fn rgb_lerp_red_to_blue_passes_through_purple_or_gray() {
    let g = Gradient::new(&[Color::RED, Color::BLUE], &[5]);
    let mid = g.spectrum[g.spectrum.len() / 2];
    // RGB lerp produces (~127, 0, ~127) — purple-gray.
    assert!(mid.r < 200 && mid.b < 200);
}

#[test]
fn hsl_lerp_red_to_blue_passes_through_magenta() {
    let g = Gradient::new_in_space(&[Color::RED, Color::BLUE], &[5], ColorSpace::Hsl);
    let mid = g.spectrum[g.spectrum.len() / 2];
    // HSL lerp via the short hue path lands on magenta (~255, 0, ~255).
    assert!(mid.r > 200, "expected magenta-ish midpoint, got {mid:?}");
    assert!(mid.b > 200, "expected magenta-ish midpoint, got {mid:?}");
}

#[test]
fn hsl_fade_gradient_preserves_hue_dominance() {
    // The Beams fg_fade_gradient case: full magenta → 30%-brightness magenta.
    let full = Color::new(0x8A, 0x00, 0x8A, 0xFF);
    let faded = full.brighten_hsl(0.3);
    let g = Gradient::new_in_space(&[full, faded], &[10], ColorSpace::Hsl);
    for c in &g.spectrum {
        assert!(c.r > c.g, "magenta dominance preserved across fade: {c:?}");
        assert!(c.b > c.g, "magenta dominance preserved across fade: {c:?}");
    }
}

#[test]
fn serde_round_trip_with_color_space() {
    let g = Gradient::new_in_space(&[Color::RED, Color::BLUE], &[3], ColorSpace::Hsl);
    let json = serde_json::to_string(&g).unwrap();
    let g2: Gradient = serde_json::from_str(&json).unwrap();
    assert_eq!(g.color_space, g2.color_space);
}

#[test]
fn rgb_default_omits_color_space_in_json() {
    let g = Gradient::new(&[Color::RED, Color::BLUE], &[3]);
    let json = serde_json::to_string(&g).unwrap();
    // With #[serde(default)], the default-Rgb case can be omitted.
    // If serde is configured to skip-on-default, this assertion holds.
    // If not, drop this test — it's a serde policy choice, not a correctness one.
}
```

### Definition of done

- `ColorSpace` enum exists, has rustdoc, derives `ConfigSchema`.
- `Gradient` carries the new field with `#[serde(default)]`.
- New constructor `Gradient::new_in_space` (or equivalent) is public.
- Existing call sites are unchanged (they default to RGB).
- All tests pass; `cargo xtask docs generate` reports clean drift.
- Cross-repo audit: `Gradient::new` call sites surveyed across all four repos. Backward compat is the load-bearing claim — if any caller relies on positional args, the new field must be `#[serde(default)]` and not break the constructor.

### Estimated effort

Half a day to a day. Largest cost is the schema-generation drift check and validating that no V2-era recipe stops parsing.

---

## Phase 3 — WavefrontTrigger spatial trigger field (mixed-signals)

### Goal

Provide a spatial scalar field `t_trigger(x, y) -> seconds` so per-cell scripted effects know when each cell should "fire." This is the substrate that lets Beams' beams, Sweep's eased columns, and Beams' diagonal wipe all share one parameter shape.

### Why mixed-signals

Per Intention 9: signal/math substrate, applicable to 3+ use cases (Beams beam-sweep, Sweep eased-column-sweep, Beams diagonal-wipe — and any future TTE-style port), renderer-agnostic. Belongs upstream.

### Reference

- Beams' beam speed / direction: `pro/main.rs:949-963` (per-group speed jitter).
- Beams' beam advancement per-tick: `pro/main.rs:1115-1126` (`group.next_character_counter += group.speed`).
- Sweep's eased column pacing: `pro/main.rs:1313-1314` (`length = (eased * seq_len) as usize`).
- Sweep's `CircInOut` driver: `pro/main.rs:1380-1385` (`in_out_circ`).
- Beams' diagonal wipe ordering: `pro/main.rs:694-708` (the `column - row` axis function).

### API surface

Free-function form. If/when mixed-signals' `Signal2d` trait lands (per the flag-animation PRD referenced in Intention 9), refactor this into a trait impl in a separate packet.

```rust
// mixed-signals/src/spatial/fnc_wavefront_trigger.rs

use crate::easing::{ease, EasingType};
use crate::random::hash_to_index;

/// Axis along which the wavefront sweeps.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WavefrontAxis {
    /// Sweep right (positive X). Trigger time grows with column.
    LeftToRight,
    /// Sweep left.
    RightToLeft,
    /// Sweep down (positive Y).
    TopToBottom,
    /// Sweep up.
    BottomToTop,
    /// Diagonal top-left to bottom-right. Axis function = `column - row`.
    DiagonalTlBr,
    /// Diagonal top-right to bottom-left. Axis function = `column + row`.
    DiagonalTrBl,
}

/// Configuration for a wavefront trigger field.
///
/// Computes per-cell trigger-time-in-seconds as:
///
/// ```text
/// t_trigger(x, y) = base_offset_seconds
///                 + (axis_position(x, y) / total_extent) * total_duration_seconds
///                 + jitter(x, y, seed, jitter_amount_seconds)
/// ```
///
/// where `axis_position` is determined by `axis` and `total_extent` is the
/// canvas extent along that axis. An optional `easing` reshapes the
/// `axis_position / total_extent` ratio before it is multiplied by the
/// duration — so `EasingType::CircInOut` produces the eased pacing TTE Sweep
/// uses (`pro/main.rs:1380-1385`).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WavefrontTriggerConfig {
    pub axis: WavefrontAxis,
    /// Total time the wavefront takes to traverse the canvas.
    pub total_duration_seconds: f32,
    /// Base offset added to every cell. Typical use: stagger multiple
    /// wavefronts so they don't all start at t=0.
    #[serde(default)]
    pub base_offset_seconds: f32,
    /// If set, the axis ratio `[0,1]` is eased before scaling to duration.
    /// `None` = linear.
    #[serde(default)]
    pub easing: Option<EasingType>,
    /// Optional per-cell jitter (seeded, deterministic, position-keyed).
    /// `None` = no jitter.
    #[serde(default)]
    pub jitter: Option<JitterConfig>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct JitterConfig {
    pub seed: u64,
    /// Maximum absolute jitter in seconds applied per cell.
    pub amount_seconds: f32,
}

/// Compute trigger time in seconds for a cell at `(x, y)` on a canvas of
/// `(width, height)`.
///
/// Axis range is normalized to [0, 1] before optional easing and scaling.
pub fn trigger_time(
    config: &WavefrontTriggerConfig,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
) -> f32 {
    let ratio = axis_ratio(config.axis, x, y, width, height);
    let eased = match config.easing {
        Some(e) => ease(ratio as f64, e) as f32,
        None => ratio,
    };
    let mut t = config.base_offset_seconds + eased * config.total_duration_seconds;
    if let Some(j) = &config.jitter {
        // hash_to_index returns a deterministic index in [0, n); map to [-1, 1].
        let bucket = hash_to_index(j.seed, x as u32, y as u32, 2048) as f32;
        let signed = (bucket / 1024.0) - 1.0; // [-1, 1)
        t += signed * j.amount_seconds;
    }
    t.max(0.0)
}

fn axis_ratio(axis: WavefrontAxis, x: u16, y: u16, width: u16, height: u16) -> f32 {
    let w = width.max(1) as f32 - 1.0;
    let h = height.max(1) as f32 - 1.0;
    match axis {
        WavefrontAxis::LeftToRight => x as f32 / w.max(1.0),
        WavefrontAxis::RightToLeft => 1.0 - x as f32 / w.max(1.0),
        WavefrontAxis::TopToBottom => y as f32 / h.max(1.0),
        WavefrontAxis::BottomToTop => 1.0 - y as f32 / h.max(1.0),
        WavefrontAxis::DiagonalTlBr => {
            // Range of (x - y) on a (w x h) canvas: [-h, w]. Width of range: w + h.
            let raw = x as f32 - y as f32;
            (raw + h) / (w + h).max(1.0)
        }
        WavefrontAxis::DiagonalTrBl => {
            // Range of (x + y): [0, w + h].
            (x as f32 + y as f32) / (w + h).max(1.0)
        }
    }
}
```

### Files to create

1. `mixed-signals/src/spatial/mod.rs` — module decl (create dir if absent).
2. `mixed-signals/src/spatial/fnc_wavefront_trigger.rs` — config types + `trigger_time` function.
3. `mixed-signals/src/spatial/test_fnc_wavefront_trigger.rs` — tests.
4. `mixed-signals/src/lib.rs` — `pub mod spatial;`.

### Files to create in tui-vfx-recipes (sampler vocabulary)

Recipe authors reference the wavefront via a sampler step that emits a `HintRef<f32>` named `trigger_time`. The recipe schema needs a new sampler kind. Identify the existing sampler-spec module:

```bash
ofpf-defs SamplerSpec --root /usr/projects/tui-vfx-recipes
ofpf-content "kind: \"sampler\"" --root /usr/projects/tui-vfx-recipes --glob "**/*.rs"
```

Add a `SamplerKind::WavefrontTrigger { config: WavefrontTriggerConfig, emits_hint: String }` variant (or whatever the existing pattern is). Lower it to a per-cell `f32` field exposed through the `HintRef` system. Test that downstream filters can read the hint.

The recipe-side schema work is bounded but schema-bearing per Intention 12A: rustdoc, ConfigSchema derive, drift gates.

### Tests

```rust
// test_fnc_wavefront_trigger.rs

use super::*;

#[test]
fn left_to_right_linear_traverses_zero_to_duration() {
    let cfg = WavefrontTriggerConfig {
        axis: WavefrontAxis::LeftToRight,
        total_duration_seconds: 2.0,
        base_offset_seconds: 0.0,
        easing: None,
        jitter: None,
    };
    assert_eq!(trigger_time(&cfg, 0, 0, 10, 5), 0.0);
    assert!((trigger_time(&cfg, 9, 0, 10, 5) - 2.0).abs() < 1e-6);
    assert!((trigger_time(&cfg, 5, 2, 10, 5) - (5.0 / 9.0) * 2.0).abs() < 1e-3);
}

#[test]
fn right_to_left_inverts_left_to_right() {
    let mut cfg = WavefrontTriggerConfig {
        axis: WavefrontAxis::LeftToRight,
        total_duration_seconds: 1.0,
        base_offset_seconds: 0.0,
        easing: None,
        jitter: None,
    };
    let ltr = trigger_time(&cfg, 3, 0, 10, 5);
    cfg.axis = WavefrontAxis::RightToLeft;
    let rtl = trigger_time(&cfg, 3, 0, 10, 5);
    assert!((ltr + rtl - 1.0).abs() < 1e-6);
}

#[test]
fn diagonal_tl_br_origin_is_zero_far_corner_is_full() {
    let cfg = WavefrontTriggerConfig {
        axis: WavefrontAxis::DiagonalTlBr,
        total_duration_seconds: 1.0,
        base_offset_seconds: 0.0,
        easing: None,
        jitter: None,
    };
    // top-right (x=w-1, y=0) → x-y = w-1 (max), ratio = 1.0
    assert!((trigger_time(&cfg, 9, 0, 10, 5) - 1.0).abs() < 1e-6);
    // bottom-left (x=0, y=h-1) → x-y = -(h-1) (min), ratio = 0.0
    assert!((trigger_time(&cfg, 0, 4, 10, 5) - 0.0).abs() < 1e-6);
}

#[test]
fn easing_circ_in_out_matches_reference() {
    let cfg = WavefrontTriggerConfig {
        axis: WavefrontAxis::LeftToRight,
        total_duration_seconds: 1.0,
        base_offset_seconds: 0.0,
        easing: Some(EasingType::CircInOut),
        jitter: None,
    };
    // CircInOut(0.0) = 0, CircInOut(0.5) = 0.5, CircInOut(1.0) = 1.0
    let t_mid = trigger_time(&cfg, 5, 0, 11, 5); // ratio = 0.5 exactly
    assert!((t_mid - 0.5).abs() < 1e-3);
}

#[test]
fn jitter_is_deterministic_for_same_seed() {
    let cfg = WavefrontTriggerConfig {
        axis: WavefrontAxis::LeftToRight,
        total_duration_seconds: 1.0,
        base_offset_seconds: 0.0,
        easing: None,
        jitter: Some(JitterConfig { seed: 42, amount_seconds: 0.1 }),
    };
    let a = trigger_time(&cfg, 5, 2, 10, 5);
    let b = trigger_time(&cfg, 5, 2, 10, 5);
    assert_eq!(a, b);
}

#[test]
fn jitter_is_position_keyed() {
    let cfg = WavefrontTriggerConfig {
        axis: WavefrontAxis::LeftToRight,
        total_duration_seconds: 1.0,
        base_offset_seconds: 0.0,
        easing: None,
        jitter: Some(JitterConfig { seed: 42, amount_seconds: 0.1 }),
    };
    let a = trigger_time(&cfg, 5, 2, 10, 5);
    let b = trigger_time(&cfg, 6, 2, 10, 5); // adjacent column
    assert_ne!(a, b, "adjacent cells should jitter to different times");
}

#[test]
fn never_returns_negative_time() {
    let cfg = WavefrontTriggerConfig {
        axis: WavefrontAxis::LeftToRight,
        total_duration_seconds: 1.0,
        base_offset_seconds: 0.0, // small base
        easing: None,
        jitter: Some(JitterConfig { seed: 1, amount_seconds: 5.0 }), // huge jitter
    };
    for x in 0..10 {
        for y in 0..5 {
            assert!(trigger_time(&cfg, x, y, 10, 5) >= 0.0);
        }
    }
}
```

### Recipe vocabulary tests (in tui-vfx-recipes)

After the sampler kind lands, add a fixture that exercises it as a hint producer:

```json
{
  "step": {
    "kind": "sequence",
    "children": [
      {
        "kind": "sampler",
        "payload": {
          "type": "wavefront_trigger",
          "emits_hint": "trigger_time",
          "config": {
            "axis": "left_to_right",
            "total_duration_seconds": 2.0,
            "easing": "circ_in_out"
          }
        }
      },
      {
        "kind": "filter",
        "payload": {
          "type": "debug_hint_dump",
          "binds": { "trigger_time": "trigger_time" }
        }
      }
    ]
  }
}
```

(`debug_hint_dump` is a placeholder — use whichever existing probe/debug filter the codebase already provides for hint-trace verification, or add a minimal one if none exists.)

### Definition of done

- `WavefrontTriggerConfig`, `WavefrontAxis`, `JitterConfig`, `trigger_time` are public in `mixed-signals`.
- mixed-signals tests pass.
- The recipe-side sampler vocabulary entry exists and lowers correctly.
- A round-trip JSON test asserts that every `WavefrontAxis` variant deserializes from snake_case (`"left_to_right"`, `"diagonal_tl_br"`, etc.) and serializes back identically.
- Cross-repo audit: new public types, no existing surface changes — record that the audit ran.
- File metadata headers in place.

### Estimated effort

One to two days. The mixed-signals piece is mechanical; the recipe-vocabulary piece is the larger time sink because of the schema-bearing obligations (Intention 12A).

---

## Phase 4 — GlyphTimeline filter (tui-vfx-compositor)

### Goal

Add a per-cell discrete-frame, variable-dwell, one-shot timeline filter. Closes the per-cell scripted-scene paradigm gap.

### Reference

The reference algorithm is `Character::activate_scene` and `Character::tick` in `pro/main.rs:475-519`. The frame-list shape is `pro/main.rs:369-377` (`FrameSpec { visual: Visual { symbol, colors }, duration }`). The two effect call-sites that build per-cell timelines are `pro/main.rs:1054-1067` (Beams' beam-row, beam-column, brighten) and `pro/main.rs:1202-1222` (Sweep's initial_sweep, second_sweep).

### Why this is distinct from `AnimatedGlyphRamp`

`AnimatedGlyphRamp` (`crates/tui-vfx-compositor/src/filters/cls_animated_glyph_ramp.rs`) is continuous-phase, uniform per-frame dwell, infinite-loop. `GlyphTimeline` is discrete-frame, variable per-frame dwell, one-shot per cell when triggered. They cover the same visual space for *uniform* timelines; for *non-uniform* timelines (TTE allows `duration: 4` on one frame and `duration: 1` on the next) `AnimatedGlyphRamp` would need a Keyframes-driven phase remap, which is harder to author and less efficient than a direct filter.

This is the second use of TTE-style scripted scenes. Counting Sweep's two passes plus Beams' three scene kinds (beam_row, beam_column, brighten) plus the broader TTE corpus that would land later, this clearly clears Intention 23's rule-of-three threshold.

### Pipeline-touch obligations (Intention 34)

This is a V3 schema-bearing pipeline filter. Definition of done includes:

- Public schema-bearing types carry `ConfigSchema`, rustdoc, serde shape, drift gates.
- Vocabulary aligned with Intention 32 (use the schema field's canonical vocabulary in fixture and recipe directories).
- Debug recipe per Intention 31 — at least one primitive-first reference fixture under `recipes/debug/glyph_timeline/` showing a clear, legible timeline run.
- Updated rustdoc on every new public item.
- Generated-doc inputs refreshed (`cargo xtask docs generate` clean).
- 4-repo audit per Intention 41.

### Files to create

1. `crates/tui-vfx-compositor/src/filters/cls_glyph_timeline.rs` — the filter struct and `Filter` impl.
2. `crates/tui-vfx-compositor/src/filters/test_cls_glyph_timeline.rs` — unit tests.
3. `crates/tui-vfx-compositor/src/types/cls_glyph_timeline_spec.rs` — schema-bearing spec types.
4. `crates/tui-vfx-compositor/src/types/test_cls_glyph_timeline_spec.rs` — schema/serde tests.
5. `crates/tui-vfx-compositor/src/pipeline/fnc_prepare_glyph_timeline.rs` — lowering from spec to prepared filter.
6. `crates/tui-vfx-compositor/src/pipeline/test_fnc_prepare_glyph_timeline.rs` — lowering tests.
7. Recipe vocabulary entry in `tui-vfx-recipes` — follow the existing `AnimatedGlyphRamp` pattern (`ofpf-content "AnimatedGlyphRamp" --root /usr/projects/tui-vfx-recipes` to find).
8. `recipes/debug/glyph_timeline/glyph_timeline_one_shot_hold.json` — debug fixture (Intention 31).
9. Optional sibling fixtures for `Loop` and `Hide` completion modes.

### Data-structure shape

```rust
// cls_glyph_timeline_spec.rs

use serde::{Deserialize, Serialize};
use tui_vfx_core::ConfigSchema;
use tui_vfx_types::Color;

/// One frame in a glyph timeline: a glyph + optional fg/bg colors held for
/// `duration_ticks` ticks (60 ticks/second).
///
/// Mirrors TTE's `FrameSpec`/`Visual` pair (`pro/main.rs:369-377`).
#[derive(Debug, Clone, Serialize, Deserialize, ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct VfxGlyphTimelineFrame {
    /// Glyph rendered while this frame is active.
    pub glyph: char,
    /// Optional foreground color. `None` leaves the cell's existing fg.
    #[serde(default)]
    pub fg: Option<Color>,
    /// Optional background color.
    #[serde(default)]
    pub bg: Option<Color>,
    /// Tick count this frame holds for. Minimum 1.
    pub duration_ticks: u16,
}

/// What happens when a cell finishes the last frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, ConfigSchema)]
#[serde(rename_all = "snake_case")]
pub enum VfxGlyphTimelineCompletion {
    /// The last frame stays rendered indefinitely (TTE default).
    #[default]
    Hold,
    /// The cell's glyph + colors are unset, restoring the cell to its
    /// pre-timeline state.
    Hide,
    /// Timeline wraps to frame 0 and continues forever.
    Loop,
}

/// Where this timeline pulls its per-cell trigger time from.
///
/// `HintRef` is the V3-canonical inter-step composition path (Phase 3).
/// `Inline` is a convenience for one-shot recipes that don't need a
/// separate sampler step — equivalent to `AnimatedGlyphRamp`'s
/// `phase_offset_*_ms` shape.
#[derive(Debug, Clone, Serialize, Deserialize, ConfigSchema)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum VfxGlyphTimelineTrigger {
    /// Read trigger time per cell from a named hint produced upstream.
    HintRef {
        /// Hint name to read. Matches the `emits_hint` of an upstream
        /// sampler (typically a `WavefrontTrigger`).
        name: String,
    },
    /// Compute trigger time inline as `base + x * x_ms + y * y_ms`,
    /// in seconds.
    Inline {
        #[serde(default)]
        base_offset_seconds: f32,
        #[serde(default)]
        phase_offset_x_ms: f32,
        #[serde(default)]
        phase_offset_y_ms: f32,
    },
}

/// Apply mode mirrors AnimatedGlyphRamp.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, ConfigSchema)]
#[serde(rename_all = "snake_case")]
pub enum VfxGlyphTimelineApplyTo {
    Fg,
    Bg,
    #[default]
    Both,
}

/// Affect mode mirrors AnimatedGlyphRamp / CharsetNoise.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, ConfigSchema)]
#[serde(rename_all = "snake_case")]
pub enum VfxGlyphTimelineAffect {
    /// Replace every cell, including whitespace.
    All,
    /// Skip space (' ') and empty braille ('\u{2800}').
    #[default]
    NonEmpty,
}

/// Recipe-shape spec for a `GlyphTimeline` filter.
#[derive(Debug, Clone, Serialize, Deserialize, ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct VfxGlyphTimelineSpec {
    /// Frames in playback order. Must be non-empty.
    pub frames: Vec<VfxGlyphTimelineFrame>,
    /// Source of per-cell trigger time.
    pub trigger: VfxGlyphTimelineTrigger,
    /// What happens after the last frame.
    #[serde(default)]
    pub on_complete: VfxGlyphTimelineCompletion,
    /// Which color channel(s) the timeline writes into.
    #[serde(default)]
    pub apply_to: VfxGlyphTimelineApplyTo,
    /// Which cells are affected.
    #[serde(default)]
    pub affect: VfxGlyphTimelineAffect,
}
```

### Filter (prepared) form

```rust
// cls_glyph_timeline.rs

use crate::traits::filter::Filter;
use tui_vfx_types::{Cell, Color};

const TICKS_PER_SECOND: f32 = 60.0;

/// Prepared (compiled) form of a glyph timeline. Frames carry cumulative
/// end-tick offsets so the active-frame lookup is O(log N) via binary
/// search, not O(N) per cell per frame.
pub struct GlyphTimeline {
    frames: Vec<PreparedFrame>,
    total_duration_ticks: u32,
    trigger: PreparedTrigger,
    on_complete: Completion,
    apply_to: ApplyTo,
    affect: Affect,
}

struct PreparedFrame {
    glyph: char,
    fg: Option<Color>,
    bg: Option<Color>,
    /// Cumulative end-tick: `sum of duration_ticks for frames[0..=self_index]`.
    cumulative_end_ticks: u32,
}

pub enum PreparedTrigger {
    HintRef { name: String },
    Inline { base_offset_seconds: f32, phase_offset_x_ms: f32, phase_offset_y_ms: f32 },
}

pub enum Completion { Hold, Hide, Loop }

pub enum ApplyTo { Fg, Bg, Both }

pub enum Affect { All, NonEmpty }

impl Filter for GlyphTimeline {
    fn apply(&self, cells: &mut /* CellGrid type */, t_seconds: f32, hints: &/* HintContext */) {
        for (x, y, cell) in cells.iter_xy_mut() {
            // Affect filter — mirrors AnimatedGlyphRamp.
            if matches!(self.affect, Affect::NonEmpty) {
                if cell.symbol == ' ' || cell.symbol == '\u{2800}' {
                    continue;
                }
            }

            let t_trigger = match &self.trigger {
                PreparedTrigger::HintRef { name } => match hints.get_f32_at(name, x, y) {
                    Some(t) => t,
                    None => continue, // hint absent for this cell — skip
                },
                PreparedTrigger::Inline { base_offset_seconds, phase_offset_x_ms, phase_offset_y_ms } => {
                    *base_offset_seconds
                        + (x as f32) * (phase_offset_x_ms / 1000.0)
                        + (y as f32) * (phase_offset_y_ms / 1000.0)
                }
            };

            let t_local_seconds = t_seconds - t_trigger;
            if t_local_seconds < 0.0 {
                continue;
            }
            let t_local_ticks = (t_local_seconds * TICKS_PER_SECOND) as u32;

            let active_idx = match self.on_complete {
                Completion::Hold => self.find_frame_or_last(t_local_ticks),
                Completion::Hide => {
                    if t_local_ticks >= self.total_duration_ticks { continue; }
                    self.find_frame(t_local_ticks)
                }
                Completion::Loop => {
                    let wrapped = t_local_ticks % self.total_duration_ticks.max(1);
                    self.find_frame(wrapped)
                }
            };
            let frame = &self.frames[active_idx];

            cell.symbol = frame.glyph;
            match self.apply_to {
                ApplyTo::Fg => if let Some(c) = frame.fg { cell.set_fg(c); }
                ApplyTo::Bg => if let Some(c) = frame.bg { cell.set_bg(c); }
                ApplyTo::Both => {
                    if let Some(c) = frame.fg { cell.set_fg(c); }
                    if let Some(c) = frame.bg { cell.set_bg(c); }
                }
            }
        }
    }
}

impl GlyphTimeline {
    fn find_frame(&self, ticks: u32) -> usize {
        // Binary search by cumulative_end_ticks > ticks.
        match self.frames.binary_search_by(|f| {
            if f.cumulative_end_ticks <= ticks {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        }) {
            Ok(i) | Err(i) => i.min(self.frames.len() - 1),
        }
    }
    fn find_frame_or_last(&self, ticks: u32) -> usize {
        if ticks >= self.total_duration_ticks {
            self.frames.len() - 1
        } else {
            self.find_frame(ticks)
        }
    }
}
```

> The exact `cells.iter_xy_mut()` and `hints.get_f32_at(name, x, y)` shape will follow the existing compositor traits — find the canonical iteration form by reading `AnimatedGlyphRamp`'s `Filter::apply` impl and mirror it. Do not invent new iteration patterns; use what's already in `traits/filter.rs` and the hint-context surface.

### Lowering function

```rust
// fnc_prepare_glyph_timeline.rs

use super::cls_glyph_timeline::{GlyphTimeline, PreparedFrame, PreparedTrigger, /* ... */};
use crate::types::cls_glyph_timeline_spec::*;

#[derive(Debug, thiserror::Error)]
pub enum PrepareGlyphTimelineError {
    #[error("glyph timeline must have at least one frame")]
    EmptyFrames,
    #[error("glyph timeline frame {0} has duration_ticks=0; must be >=1")]
    ZeroDuration(usize),
}

pub fn prepare_glyph_timeline(
    spec: &VfxGlyphTimelineSpec,
) -> Result<GlyphTimeline, PrepareGlyphTimelineError> {
    if spec.frames.is_empty() {
        return Err(PrepareGlyphTimelineError::EmptyFrames);
    }
    let mut frames = Vec::with_capacity(spec.frames.len());
    let mut cumulative: u32 = 0;
    for (i, f) in spec.frames.iter().enumerate() {
        if f.duration_ticks == 0 {
            return Err(PrepareGlyphTimelineError::ZeroDuration(i));
        }
        cumulative += f.duration_ticks as u32;
        frames.push(PreparedFrame {
            glyph: f.glyph,
            fg: f.fg,
            bg: f.bg,
            cumulative_end_ticks: cumulative,
        });
    }
    Ok(GlyphTimeline {
        frames,
        total_duration_ticks: cumulative,
        trigger: prepare_trigger(&spec.trigger),
        on_complete: map_completion(spec.on_complete),
        apply_to: map_apply_to(spec.apply_to),
        affect: map_affect(spec.affect),
    })
}

// map_completion / map_apply_to / map_affect / prepare_trigger are
// trivial enum-to-enum conversions; omitted for brevity.
```

### Tests — schema/serde

`test_cls_glyph_timeline_spec.rs`:

```rust
#[test]
fn minimal_spec_round_trips() {
    let json = r#"{
        "frames": [{ "glyph": "X", "fg": {"r":255,"g":0,"b":0,"a":255}, "duration_ticks": 5 }],
        "trigger": { "kind": "inline", "phase_offset_x_ms": 20.0 }
    }"#;
    let spec: VfxGlyphTimelineSpec = serde_json::from_str(json).unwrap();
    assert_eq!(spec.frames.len(), 1);
    assert!(matches!(spec.on_complete, VfxGlyphTimelineCompletion::Hold)); // default
    assert!(matches!(spec.apply_to, VfxGlyphTimelineApplyTo::Both));        // default
    assert!(matches!(spec.affect, VfxGlyphTimelineAffect::NonEmpty));       // default
}

#[test]
fn unknown_top_level_field_is_rejected() {
    let json = r#"{
        "frames": [{ "glyph": "X", "duration_ticks": 1 }],
        "trigger": { "kind": "inline" },
        "what_is_this": true
    }"#;
    let err = serde_json::from_str::<VfxGlyphTimelineSpec>(json).unwrap_err();
    assert!(err.to_string().contains("unknown field"));
}

#[test]
fn unknown_frame_field_is_rejected() {
    let json = r#"{
        "frames": [{ "glyph": "X", "duration_ticks": 1, "extra": 5 }],
        "trigger": { "kind": "inline" }
    }"#;
    serde_json::from_str::<VfxGlyphTimelineSpec>(json).unwrap_err();
}

#[test]
fn hint_ref_trigger_round_trips() {
    let json = r#"{
        "frames": [{ "glyph": "X", "duration_ticks": 1 }],
        "trigger": { "kind": "hint_ref", "name": "trigger_time" }
    }"#;
    let spec: VfxGlyphTimelineSpec = serde_json::from_str(json).unwrap();
    match spec.trigger {
        VfxGlyphTimelineTrigger::HintRef { name } => assert_eq!(name, "trigger_time"),
        _ => panic!("expected HintRef"),
    }
}

#[test]
fn snake_case_completion_modes_parse() {
    for mode_str in ["hold", "hide", "loop"] {
        let json = format!(r#"{{
            "frames": [{{ "glyph": "X", "duration_ticks": 1 }}],
            "trigger": {{ "kind": "inline" }},
            "on_complete": "{mode_str}"
        }}"#);
        serde_json::from_str::<VfxGlyphTimelineSpec>(&json)
            .unwrap_or_else(|e| panic!("failed to parse {mode_str}: {e}"));
    }
}
```

### Tests — lowering

`test_fnc_prepare_glyph_timeline.rs`:

```rust
#[test]
fn empty_frames_is_rejected_at_lowering() {
    let spec = VfxGlyphTimelineSpec {
        frames: vec![],
        trigger: VfxGlyphTimelineTrigger::Inline { base_offset_seconds: 0.0, phase_offset_x_ms: 0.0, phase_offset_y_ms: 0.0 },
        on_complete: Default::default(),
        apply_to: Default::default(),
        affect: Default::default(),
    };
    assert!(matches!(prepare_glyph_timeline(&spec), Err(PrepareGlyphTimelineError::EmptyFrames)));
}

#[test]
fn zero_duration_frame_is_rejected_at_lowering() {
    let spec = VfxGlyphTimelineSpec {
        frames: vec![
            VfxGlyphTimelineFrame { glyph: 'X', fg: None, bg: None, duration_ticks: 1 },
            VfxGlyphTimelineFrame { glyph: 'Y', fg: None, bg: None, duration_ticks: 0 },
        ],
        trigger: VfxGlyphTimelineTrigger::Inline { base_offset_seconds: 0.0, phase_offset_x_ms: 0.0, phase_offset_y_ms: 0.0 },
        on_complete: Default::default(),
        apply_to: Default::default(),
        affect: Default::default(),
    };
    assert!(matches!(prepare_glyph_timeline(&spec), Err(PrepareGlyphTimelineError::ZeroDuration(1))));
}

#[test]
fn cumulative_end_ticks_are_computed_correctly() {
    // 3 frames of duration 4, 2, 5 → cumulative end ticks = [4, 6, 11], total = 11.
    let spec = VfxGlyphTimelineSpec {
        frames: vec![
            VfxGlyphTimelineFrame { glyph: 'A', fg: None, bg: None, duration_ticks: 4 },
            VfxGlyphTimelineFrame { glyph: 'B', fg: None, bg: None, duration_ticks: 2 },
            VfxGlyphTimelineFrame { glyph: 'C', fg: None, bg: None, duration_ticks: 5 },
        ],
        trigger: VfxGlyphTimelineTrigger::Inline { base_offset_seconds: 0.0, phase_offset_x_ms: 0.0, phase_offset_y_ms: 0.0 },
        on_complete: Default::default(),
        apply_to: Default::default(),
        affect: Default::default(),
    };
    let prepared = prepare_glyph_timeline(&spec).unwrap();
    // Inspect via getters or pub(crate) field access in the test module.
    assert_eq!(prepared.total_duration_ticks(), 11);
}
```

### Tests — filter behavior (the key correctness gate)

`test_cls_glyph_timeline.rs`:

```rust
fn make_test_grid(w: u16, h: u16) -> /* whatever the existing test grid type is */ {
    // Mirror AnimatedGlyphRamp's test fixture style.
    todo!("port from test_cls_animated_glyph_ramp.rs")
}

#[test]
fn cell_before_trigger_is_unchanged() {
    let timeline = make_inline_timeline_x_ms(20.0, vec![('A', 1), ('B', 1)]);
    let mut grid = make_test_grid(10, 1);
    timeline.apply(&mut grid, 0.0, &empty_hints());
    // No cell should have been touched at t=0 because all triggers are >= 0.
    for x in 0..10 { assert_eq!(grid.cell(x, 0).symbol, ' '); }
}

#[test]
fn cell_at_trigger_shows_first_frame() {
    let timeline = make_inline_timeline_x_ms(20.0, vec![('A', 5), ('B', 5)]);
    let mut grid = make_test_grid(10, 1);
    timeline.apply(&mut grid, 0.0, &empty_hints());
    assert_eq!(grid.cell(0, 0).symbol, 'A'); // x=0 fires at t=0
}

#[test]
fn timeline_advances_by_duration() {
    let timeline = make_inline_timeline_x_ms(0.0, vec![('A', 5), ('B', 5)]);
    let mut grid = make_test_grid(1, 1);
    // Tick 4 (5 ticks = 5/60 ≈ 0.083s, but at 4 we're still in frame 0).
    timeline.apply(&mut grid, 4.0 / 60.0, &empty_hints());
    assert_eq!(grid.cell(0, 0).symbol, 'A');
    // Tick 5: into frame 1.
    timeline.apply(&mut grid, 5.0 / 60.0, &empty_hints());
    assert_eq!(grid.cell(0, 0).symbol, 'B');
}

#[test]
fn hold_mode_keeps_last_frame() {
    let timeline = make_inline_with_completion(Completion::Hold, vec![('A', 1), ('B', 1)]);
    let mut grid = make_test_grid(1, 1);
    timeline.apply(&mut grid, 100.0, &empty_hints()); // way past end
    assert_eq!(grid.cell(0, 0).symbol, 'B');
}

#[test]
fn hide_mode_skips_after_end() {
    let timeline = make_inline_with_completion(Completion::Hide, vec![('A', 1)]);
    let mut grid = make_test_grid(1, 1);
    grid.cell_mut(0, 0).symbol = '#';
    timeline.apply(&mut grid, 100.0, &empty_hints());
    assert_eq!(grid.cell(0, 0).symbol, '#'); // unchanged
}

#[test]
fn loop_mode_wraps() {
    let timeline = make_inline_with_completion(Completion::Loop, vec![('A', 1), ('B', 1)]);
    let mut grid = make_test_grid(1, 1);
    // Total = 2 ticks. At tick 2 we're back at frame 0.
    timeline.apply(&mut grid, 2.0 / 60.0, &empty_hints());
    assert_eq!(grid.cell(0, 0).symbol, 'A');
    timeline.apply(&mut grid, 3.0 / 60.0, &empty_hints());
    assert_eq!(grid.cell(0, 0).symbol, 'B');
}

#[test]
fn affect_non_empty_skips_whitespace() {
    let timeline = make_inline_timeline_x_ms_with_affect(0.0, vec![('A', 5)], Affect::NonEmpty);
    let mut grid = make_test_grid(2, 1);
    grid.cell_mut(0, 0).symbol = 'X'; // non-empty
    grid.cell_mut(1, 0).symbol = ' '; // empty
    timeline.apply(&mut grid, 0.0, &empty_hints());
    assert_eq!(grid.cell(0, 0).symbol, 'A');
    assert_eq!(grid.cell(1, 0).symbol, ' '); // skipped
}

#[test]
fn hint_ref_trigger_reads_from_hint_context() {
    let timeline = make_hint_ref_timeline("trigger_time", vec![('A', 1)]);
    let mut grid = make_test_grid(2, 1);
    let mut hints = empty_hints_mut();
    hints.set_f32_at("trigger_time", 0, 0, 0.0);    // x=0 fires at t=0
    hints.set_f32_at("trigger_time", 1, 0, 1.0);    // x=1 fires at t=1
    timeline.apply(&mut grid, 0.0, &hints);
    assert_eq!(grid.cell(0, 0).symbol, 'A');
    assert_eq!(grid.cell(1, 0).symbol, ' ');        // not yet
}
```

### Debug recipe

`recipes/debug/glyph_timeline/glyph_timeline_one_shot_hold.json` — per Intention 31, this is a visual reference fixture, not a smoke test. The body text states what the viewer should see.

```json
{
  "metadata": {
    "name": "Glyph Timeline — one-shot hold",
    "description": "Demonstrates GlyphTimeline running once per cell as a left-to-right wave. Each cell cycles through '·•◆' over 30 ticks, then holds on '◆'. The reveal sweeps left-to-right at 20 ms per column.",
    "aesthetic_tags": ["primitive_reference", "wavefront", "glyph_timeline"]
  },
  "canvas": { "width": 40, "height": 5 },
  "content": {
    "kind": "text",
    "text": [
      "                                        ",
      "       Glyph Timeline                   ",
      "       reveal demo                      ",
      "                                        ",
      "                                        "
    ]
  },
  "step": {
    "kind": "filter",
    "payload": {
      "type": "glyph_timeline",
      "config": {
        "frames": [
          { "glyph": "·", "fg": {"r":120,"g":120,"b":120,"a":255}, "duration_ticks": 10 },
          { "glyph": "•", "fg": {"r":180,"g":180,"b":180,"a":255}, "duration_ticks": 10 },
          { "glyph": "◆", "fg": {"r":255,"g":255,"b":255,"a":255}, "duration_ticks": 10 }
        ],
        "trigger": {
          "kind": "inline",
          "phase_offset_x_ms": 20.0
        },
        "on_complete": "hold",
        "apply_to": "both",
        "affect": "non_empty"
      }
    }
  }
}
```

### Definition of done

- `VfxGlyphTimelineSpec` and all sibling types are public, derive `ConfigSchema`, and have rustdoc.
- `prepare_glyph_timeline` lowers correctly with explicit error variants.
- The `GlyphTimeline` filter's `Filter::apply` produces the expected behavior in unit tests across all completion modes, both trigger sources, and both affect modes.
- Recipe vocabulary entry in `tui-vfx-recipes` accepts the spec and round-trips. The validator's contract-discovery surface lists `glyph_timeline` as a known filter kind.
- `cargo xtask docs generate` regenerates schema/API/capability docs cleanly.
- The debug recipe renders correctly in the recipe browser; description is on-screen-quality.
- 4-repo audit recorded.
- File metadata headers in place on every new file. `<CLOG>` is one line.
- No new `#[allow]` lines.

### Estimated effort

Three to five days. The filter logic itself is small (~200 LOC). The bulk is the schema/lowering/serde-tests/docs/cross-repo-audit/debug-recipe surface. This is the largest phase.

---

## Phase 5 — Diagonal scope variants (tui-vfx-style)

### Goal

Add `StyleRegion::Diagonal { offset }` and `StyleRegion::DiagonalRange { start, end }` so authors can target diagonal bands by scope, mirroring `RowRange` / `ColumnRange`.

### Reference

`pro/main.rs:694-708` defines diagonal grouping as `cell.column - cell.row == constant`. The diagonal-iteration range is `(1 - height)..=(width - 1)`. Negative offsets matter; `BindableU16` is therefore not the right type — use `i16`.

### Files to identify

```bash
ofpf-defs StyleRegion --root /usr/projects/tui-vfx --kind enum
ofpf-content "StyleRegion::RowRange" --root /usr/projects/tui-vfx
```

The canonical `StyleRegion` enum and its companion functions live under `crates/tui-vfx-style/src/models/`. From the prior survey the relevant files are:

- `cls_style_region*.rs` — the enum definition (find via `ofpf-defs`).
- `fnc_style_region_resolved.rs` — Bindable→literal resolution.
- `fnc_style_region_should_style.rs` — predicate `(cell, region) → bool`.
- `fnc_style_region_bounding_rect.rs` — bounding-rect helper.
- `fnc_style_region_deserialize.rs` — V3 lift / shadow-deserialization path.

### Variant additions

```rust
// In whichever file defines StyleRegion (cls_style_region.rs likely):

pub enum StyleRegion {
    // ... existing variants ...
    /// Cells where `column - row == offset`. Negative offsets target
    /// diagonals above the main TL→BR axis; positive ones below.
    /// Range: `(1 - height) ..= (width - 1)`.
    Diagonal {
        /// Diagonal offset in the `column - row` axis. Use `i16` so
        /// negative offsets are first-class.
        offset: i16,
    },
    /// Inclusive range of diagonals: cells where
    /// `start <= (column - row) <= end`.
    DiagonalRange {
        start: i16,
        end: i16,
    },
}
```

> **Bindable consideration.** Per Intention 24 (earn-your-place), do **not** wrap `offset` / `start` / `end` in a hypothetical `BindableI16` unless there's an active call site that wants runtime binding. Ship as `i16` literal first; lift to bindable in a follow-on packet only when the binding need surfaces.

### Predicate

```rust
// fnc_style_region_should_style.rs — add cases to the existing match:

StyleRegion::Diagonal { offset } => {
    (cell.column as i32) - (cell.row as i32) == *offset as i32
}
StyleRegion::DiagonalRange { start, end } => {
    let d = (cell.column as i32) - (cell.row as i32);
    let lo = (*start as i32).min(*end as i32);
    let hi = (*start as i32).max(*end as i32);
    d >= lo && d <= hi
}
```

### Bounding rect

`fnc_style_region_bounding_rect.rs` — diagonals don't have a useful bounding rectangle smaller than the canvas (they touch every row and every column). Return `None`, matching the existing `RowRange`/`ColumnRange` behavior.

### Resolution / deserialization

`fnc_style_region_resolved.rs` and `fnc_style_region_deserialize.rs` — if the enum is non-bindable, the resolution is identity. The deserialize-shadow lift is a one-line forward like:

```rust
StyleRegionShadow::Diagonal { offset } => StyleRegion::Diagonal { offset },
StyleRegionShadow::DiagonalRange { start, end } => StyleRegion::DiagonalRange { start, end },
```

### Tests

```rust
// test_cls_style_region_diagonal.rs (or extend existing test file)

#[test]
fn diagonal_zero_matches_main_axis() {
    let region = StyleRegion::Diagonal { offset: 0 };
    assert!(should_style(&region, cell_at(3, 3)));
    assert!(should_style(&region, cell_at(7, 7)));
    assert!(!should_style(&region, cell_at(3, 4)));
}

#[test]
fn diagonal_positive_offset_matches_below_axis() {
    let region = StyleRegion::Diagonal { offset: 2 };
    // column - row == 2 → e.g. (5, 3), (10, 8)
    assert!(should_style(&region, cell_at(5, 3)));
    assert!(!should_style(&region, cell_at(3, 5)));
}

#[test]
fn diagonal_negative_offset_matches_above_axis() {
    let region = StyleRegion::Diagonal { offset: -2 };
    // column - row == -2 → e.g. (3, 5), (8, 10)
    assert!(should_style(&region, cell_at(3, 5)));
    assert!(!should_style(&region, cell_at(5, 3)));
}

#[test]
fn diagonal_range_inclusive() {
    let region = StyleRegion::DiagonalRange { start: -1, end: 1 };
    assert!(should_style(&region, cell_at(3, 3)));   // d=0
    assert!(should_style(&region, cell_at(4, 3)));   // d=1
    assert!(should_style(&region, cell_at(3, 4)));   // d=-1
    assert!(!should_style(&region, cell_at(5, 3)));  // d=2
    assert!(!should_style(&region, cell_at(3, 5)));  // d=-2
}

#[test]
fn diagonal_range_handles_reversed_endpoints() {
    let region = StyleRegion::DiagonalRange { start: 1, end: -1 };
    // Implementation should normalize, not crash.
    assert!(should_style(&region, cell_at(3, 3))); // d=0
}

#[test]
fn diagonal_serde_round_trip() {
    let r = StyleRegion::Diagonal { offset: -3 };
    let json = serde_json::to_string(&r).unwrap();
    let r2: StyleRegion = serde_json::from_str(&json).unwrap();
    assert_eq!(r, r2);
}

#[test]
fn diagonal_range_serde_round_trip() {
    let r = StyleRegion::DiagonalRange { start: -5, end: 7 };
    let json = serde_json::to_string(&r).unwrap();
    let r2: StyleRegion = serde_json::from_str(&json).unwrap();
    assert_eq!(r, r2);
}

#[test]
fn diagonal_bounding_rect_is_none() {
    let r = StyleRegion::Diagonal { offset: 0 };
    assert!(bounding_rect(&r, 80, 24).is_none());
}
```

### Debug recipe

Add `recipes/debug/scope/diagonal_band.json` showing a `DiagonalRange` band of cells styled differently from the surrounding canvas. Per Intention 31, the description spells out what the viewer should see.

### Cross-repo audit (Intention 41)

`StyleRegion` is constructed by recipe authors. Audit:

```bash
for repo in tui-vfx tui-vfx-recipes mixed-signals gt-design; do
  echo "=== $repo ==="
  ofpf-content --root /usr/projects/$repo "StyleRegion::"
  ofpf-content --root /usr/projects/$repo "\"row_range\"|\"column_range\""
done
```

Existing call sites are unchanged (purely additive); but record the per-repo counts in the commit message so the audit is visible.

### Definition of done

- `Diagonal` and `DiagonalRange` variants are public on `StyleRegion`.
- All four `fnc_style_region_*` files are updated.
- Tests cover positive/negative/zero offsets, inclusive range, reversed endpoints, serde round-trip, bounding-rect-None.
- Debug recipe lands and reads correctly.
- `cargo xtask docs generate` clean.
- 4-repo audit recorded.
- File metadata headers updated. `<CLOG>` is one line.

### Estimated effort

Half a day to a day. Multi-file but each edit is small.

---

## Sample recipes (illustrative)

These are recipe-shape sketches showing how the five additions compose end-to-end. The exact V3 schema may differ from these snippets; verify against `docs/design/tui-vfx-v3-schema-draft.json` and the canonical schema generator at implementation time. Treat the JSON below as a target shape, not a contract.

### TTE Sweep — uses Phases 1, 2, 3, 4

```json
{
  "metadata": {
    "name": "TTE Sweep",
    "description": "Two-pass sweep effect ported from TerminalTextEffects. Pass 1: right-to-left columns cycle gray block glyphs then settle to mid-gray. Pass 2: left-to-right columns cycle blocks colored from the magenta→cyan→white gradient and settle to per-cell final color. Eased by CircInOut over 100 ticks per pass.",
    "aesthetic_tags": ["showcase", "tte_port", "wavefront", "glyph_timeline"]
  },
  "requires_bindings": {},
  "canvas": { "width": 80, "height": 24 },
  "content": { "kind": "text", "text": "<the demo text from pro/main.rs:35-60>" },
  "step": {
    "kind": "sequence",
    "children": [
      {
        "kind": "parallel",
        "children": [
          {
            "kind": "sampler",
            "payload": {
              "type": "wavefront_trigger",
              "emits_hint": "trigger_pass_1",
              "config": {
                "axis": "right_to_left",
                "total_duration_seconds": 1.667,
                "easing": "circ_in_out"
              }
            }
          },
          {
            "kind": "filter",
            "payload": {
              "type": "glyph_timeline",
              "config": {
                "frames": [
                  { "glyph": "█", "fg": {"r":160,"g":160,"b":160,"a":255}, "duration_ticks": 5 },
                  { "glyph": "▓", "fg": {"r":128,"g":128,"b":128,"a":255}, "duration_ticks": 5 },
                  { "glyph": "▒", "fg": {"r":64,"g":64,"b":64,"a":255},   "duration_ticks": 5 },
                  { "glyph": "░", "fg": {"r":32,"g":32,"b":32,"a":255},   "duration_ticks": 5 }
                ],
                "trigger": { "kind": "hint_ref", "name": "trigger_pass_1" },
                "on_complete": "hold",
                "apply_to": "fg",
                "affect": "non_empty"
              }
            }
          }
        ]
      },
      {
        "kind": "parallel",
        "children": [
          {
            "kind": "sampler",
            "payload": {
              "type": "wavefront_trigger",
              "emits_hint": "trigger_pass_2",
              "config": {
                "axis": "left_to_right",
                "total_duration_seconds": 1.667,
                "base_offset_seconds": 1.667,
                "easing": "circ_in_out"
              }
            }
          },
          {
            "kind": "filter",
            "payload": {
              "type": "glyph_timeline",
              "config": {
                "frames": [
                  { "glyph": "█", "fg": {"r":138,"g":0,"b":138,"a":255}, "duration_ticks": 5 },
                  { "glyph": "▓", "fg": {"r":69,"g":104,"b":196,"a":255}, "duration_ticks": 5 },
                  { "glyph": "▒", "fg": {"r":0,"g":209,"b":255,"a":255}, "duration_ticks": 5 },
                  { "glyph": "░", "fg": {"r":127,"g":232,"b":255,"a":255}, "duration_ticks": 5 }
                ],
                "trigger": { "kind": "hint_ref", "name": "trigger_pass_2" },
                "on_complete": "hold",
                "apply_to": "fg",
                "affect": "non_empty"
              }
            }
          }
        ]
      }
    ]
  }
}
```

> **TODO at recipe-author time.** Sweep's per-cell *final* color (`pro/main.rs:1217-1219`) comes from a vertical gradient mapping. Express this as a final shader or filter that applies the per-row color *after* the second timeline holds — likely a `Tint` or vertical-gradient shader applied with `apply_to: fg, affect: non_empty` after the parallel block. The exact shader to use depends on what's in the existing `tui-vfx-style` family. Pick the closest existing shader or extend one; do not invent a new shader for this case.

### TTE Beams — uses Phases 1, 2, 3, 4, 5

The Beams recipe is more complex because it has stochastic per-row/per-column beams. Express the stochasticity through `WavefrontTrigger`'s `jitter` field — each row's beam fires at `(row_index / total_rows) * total_duration + hash_jitter(seed, row_index)`.

```json
{
  "metadata": {
    "name": "TTE Beams",
    "description": "Beams sweep across rows and columns at staggered times; each cell runs a beam-glyph timeline as the wavefront crosses it. After all cells fire, a diagonal wipe brightens each cell to its full color. Loops with hold.",
    "aesthetic_tags": ["showcase", "tte_port", "wavefront", "glyph_timeline", "diagonal_scope"]
  },
  "step": {
    "kind": "sequence",
    "children": [
      {
        "kind": "parallel",
        "children": [
          {
            "kind": "sampler",
            "payload": {
              "type": "wavefront_trigger",
              "emits_hint": "trigger_row_beams",
              "config": {
                "axis": "left_to_right",
                "total_duration_seconds": 3.0,
                "jitter": { "seed": 17, "amount_seconds": 0.6 }
              }
            }
          },
          {
            "kind": "filter",
            "payload": {
              "type": "glyph_timeline",
              "config": {
                "frames": [
                  { "glyph": "▂", "fg": {"r":255,"g":255,"b":255,"a":255}, "duration_ticks": 2 },
                  { "glyph": "▁", "fg": {"r":127,"g":232,"b":255,"a":255}, "duration_ticks": 2 },
                  { "glyph": "_", "fg": {"r":0,"g":209,"b":255,"a":255}, "duration_ticks": 2 }
                ],
                "trigger": { "kind": "hint_ref", "name": "trigger_row_beams" },
                "on_complete": "hide",
                "apply_to": "both",
                "affect": "all"
              }
            }
          }
        ]
      },
      {
        "kind": "filter",
        "payload": {
          "type": "shader",
          "scope": { "kind": "diagonal_range", "start": -23, "end": 79 },
          "config": {
            "type": "<existing reveal/brighten shader>",
            "...": "..."
          }
        }
      }
    ]
  }
}
```

> **The Beams recipe is intentionally truncated.** Authoring the full effect — column beams in addition to row beams, the per-character "fade then brighten" timeline, the final-gradient tint — is a separate authoring task that follows once the primitives land. The point of this snippet is to show that the new vocabulary composes cleanly, not to ship a complete recipe.

---

## Aggregate acceptance criteria

The full TTE port lands when:

1. **All five phases** have shipped per their individual definition-of-done.
2. **Two showcase recipes** (`recipes/showcase/tte_sweep.json`, `recipes/showcase/tte_beams.json`) live in the recipe library, validate clean against `pipeline-validator --rules --strict-contracts`, and render correctly in the recipe browser.
3. **Visual fidelity** is judged by side-by-side comparison with the reference Rust binary at `pro/main.rs` running on the same input text. Minor stochastic differences are expected (deterministic vs. random); gestalt match is the standard.
4. **Pipeline-touch obligations** (Intention 34) are met for every phase that touched a shader/filter/sampler/scope: rustdocs current, generated docs regenerate clean, debug recipes updated or added, validator/probe coverage extended where drift was discovered.
5. **No landmines** (Intention 40): no per-site `#[allow]`, byte-equivalent upstream extractions for the HSL algorithm, no half-finished consolidations.
6. **4-repo audit** recorded for every phase touching a public surface. Two-repo audits are the failure mode.
7. **`<CLOG>` discipline** (auto-memory rule): every touched file's CLOG is one line about the latest change only; running history lives in git.

---

## Risks and open questions

### Risk: V3 schema is in flux

The V3 upgrade plan (`docs/design/tui-vfx-v3-upgrade-plan/00_INDEX.md`) is in active planning. Details of the recipe-shape JSON in this plan may diverge from the canonical schema by the time the work executes.

**Mitigation.** Each phase ships its types as schema-bearing per Intention 12A; if the V3 cutover renames a field, the change is mechanical and the test suite catches drift. Authors of the showcase recipes should validate against the canonical schema generator at recipe-author time, not against the JSON snippets here.

### Risk: `Signal2d` trait may land mid-flight

If mixed-signals' planned `Signal2d` trait (Intention 9 / flag-animation PRD) lands while Phase 3 is in flight, the `WavefrontTrigger` free-function form should be lifted to a `Signal2d` impl in a follow-on packet. The free-function form is intentionally simple so the lift is mechanical.

**Mitigation.** Keep `WavefrontTrigger`'s public surface narrow — config struct + single function. Document the intent to lift in the file's `<WCTX>`.

### Risk: hint-context surface may not yet support per-cell f32 hints

Phase 4's `HintRef` trigger source assumes the hint-context system can carry a per-cell `f32` field readable by name. If that surface doesn't exist yet, ship Phase 4 with the `Inline` trigger form first; add the `HintRef` form once the hint surface lands. The schema split between `Inline` and `HintRef` is exactly so the work can be staged.

**Mitigation.** Phase 4 task packet should include a precondition check: confirm by reading `crates/tui-vfx-compositor/src/traits/filter.rs` and the hint-context module that per-cell f32 hints are supported. If not, scope Phase 4 to `Inline`-only and file a follow-on packet for the `HintRef` extension.

### Risk: visual fidelity gap on stochastic Beams orchestration

TTE's `gen_range(15..=60)` per-beam speed is unbounded randomness. The deterministic-jitter approach from Phase 3 (`hash_to_index`-keyed jitter) is visually similar but not identical. Authors who want exact TTE-match visuals will need to compose multiple wavefronts (one per row) with explicit jitter offsets — recipe-time work, not a primitive gap.

**Mitigation.** Document the determinism reframe in the showcase recipe metadata so reviewers know the delta is intentional.

### Open question: should `WavefrontTrigger` ship with a `WavefrontAxis::Custom { dx: f32, dy: f32 }` variant?

A linear-combination axis (`ratio = dx * x + dy * y`) generalizes the four cardinal directions and the two diagonals. Earns a place under Intention 23 only if a third use case wants a non-cardinal angle.

**Recommendation.** Defer. Ship the six concrete variants. Add `Custom` when a real third use case appears.

---

## Coordination

If multiple contributors execute phases in parallel:

- (1) and (3) can run concurrently — independent leaf substrate in `mixed-signals`.
- (2) follows (1) by a day or so; one contributor can do both back-to-back.
- (4) is the single largest phase; allocate three to five contiguous days. Can stub trigger source against `Inline` form until (3) merges.
- (5) is small and fully independent; can land at any point without blocking others.

The recipe authoring (Beams, Sweep showcase recipes) is a single contributor's work after (4) lands.

Per Intention 41, every phase's commit message records the 4-repo audit output. Per Intention 40 §6, every phase's pre-commit guard runs `git diff --cached | rg '^\+.*#\[allow|^\+.*#!\[allow'` and reports the result.

---

## Reference index

- **Source of truth (reference algorithm):** `/usr/projects/tui-vfx/pro/main.rs`
  - `adjust_color_brightness` — pro/main.rs:820-888 (Phase 1)
  - `Gradient::new` — pro/main.rs:740-779 (Phase 2)
  - `BeamGroup::new` — pro/main.rs:939-963 (Phase 3 random speed)
  - `EasingTracker::step` and `in_out_circ` — pro/main.rs:1359-1385 (Phase 3 easing)
  - `BeamsSim::tick` — pro/main.rs:1093-1156 (Phase 3 wavefront orchestration)
  - `Character::tick` and `activate_scene` — pro/main.rs:475-519 (Phase 4 timeline)
  - `Scenes` struct — pro/main.rs:423-442 (Phase 4 scene catalog)
  - `BeamsSim::new` scene construction — pro/main.rs:1044-1068 (Phase 4 frame lists)
  - `SweepSim::new` scene construction — pro/main.rs:1201-1223 (Phase 4 frame lists)
  - `SimTerminal::grouped` diagonal — pro/main.rs:694-708 (Phase 5 predicate)
  - `BeamsSim` final wipe — pro/main.rs:1135-1147 (Phase 5 use case)

- **tui-vfx existing primitives to mirror or extend:**
  - `AnimatedGlyphRamp` — `crates/tui-vfx-compositor/src/filters/cls_animated_glyph_ramp.rs` (Phase 4 pattern)
  - `CharsetNoise` — `crates/tui-vfx-compositor/src/filters/cls_charset_noise.rs` (Phase 4 affect mode)
  - `Color` — `crates/tui-vfx-types/src/color.rs:143` (Phase 1 wrapper)
  - `EasingType` and `ease` — `mixed-signals/src/easing/fnc_ease.rs:12,60-65` (Phase 3 reuse)
  - `EasingCurve` — `crates/tui-vfx-geometry/src/types/cls_easing_curve.rs` (Phase 3 reference)
  - `hash_to_index` — `mixed-signals/src/random/` (Phase 3 jitter)
  - `StyleRegion` and `fnc_style_region_*` — `crates/tui-vfx-style/src/models/` (Phase 5 extension surface)

- **Steering and process docs:**
  - `steering/INTENTIONS.md` — durable decisions; especially 9, 12A, 14, 15, 23, 24, 31, 32, 34, 40, 41, 42
  - `steering/MARKETING.md` — positioning frame; lead with architecture not effects (Intention 35)
  - `steering/OFPF-TOOLS.md` — `ofpf-*` tooling reference
  - `steering/ORCHESTRATION.md` — leader-only; do not read as a worker
  - `steering/TASK_PACKET_TEMPLATE.md` — packet shape if subagent dispatch is used
  - `~/.claude/CLAUDE.md` and `~/.claude/rules/ofpf.md` — global OFPF + librarian standards

- **V3 design context:**
  - `docs/design/tui-vfx-v3-upgrade-plan/00_INDEX.md` — V3 home
  - `docs/design/tui-vfx-v3-schema-draft.json` — specification-by-example
  - `docs/design/tui-vfx-binding-loopback-implementation-plan.md` — pattern-template implementation plan
  - `docs/design/tui-vfx-terminal-fire-shader-plan.md` — pattern-template shader plan

<!-- <FILE>docs/design/tte-effects-port-plan.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
