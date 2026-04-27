<!-- <FILE>docs/design/tui-vfx-2026-04-26-packet-signal-facade-alpha-beta.md</FILE> - <DESC>Junior-ready implementation packet for signal-facade phases α (autogen SIGNALS_REFERENCE.md) + β (curated Core 12 cheatsheet). Doc-only scope. Captures the autogen mechanism (extend the existing capability extractor; new docs/templates/signals.toml overlay; new cargo xtask docs signals subcommand) and the cheatsheet content. Self-contained brief: pre-flight, current-state audit of every Signal impl in mixed-signals (call-site evidence for which Core 12 candidates are real), the existing capabilities-pipeline shape used as the model, the signals.toml schema sketch, the xtask subcommand structure, the generated-output structure (Sine worked example), three open questions with recommended defaults, step-by-step plan, test plan (catalog completeness, unknown-signal failure, Core 12 subset check), acceptance criteria, verification commands, rollback plan, risks. Independent of phase γ; can land any time.</DESC> -->
<!-- <VERS>VERSION: 1.0.0</VERS> -->
<!-- <WCTX>2026-04-26 acceptance of mixed-signals recipe-surface proposal v0.3.0 green-lit phases α + β as doc-only and unblocked. This packet bundles them so a junior can land both in one focused session. Phase γ (the actual tui_vfx_recipes::signals module) is in flight as a separate packet and is independent of α/β.</WCTX> -->
<!-- <CLOG>1.0.0: initial packet — pre-flight, current-state audit (every Signal impl, the existing capabilities pipeline, capabilities.toml + CAPABILITIES.md as models), phase α spec (signals.toml + cargo xtask docs signals + extraction mechanism + output structure), phase β spec (Core 12 list confirmed via ofpf-defs against mixed-signals; flags Bounce and plain Noise as not-real; recommends DampedSpring, PerlinNoise, SpatialNoise as the corrected names), three open questions with recommended defaults (Q1 extend-existing extractor, Q2 only-overrides toml, Q3 toml-driven Core 12 list), step-by-step plan, code snippets (signals.toml entry for Sine, xtask subcommand structure, generated Sine section), test plan, acceptance criteria, verification commands, rollback, risks (rustdoc-quality audit may surface upstream improvements — flag as a sibling packet, do not block).</CLOG> -->

# Packet — signal-facade phases α + β (autogen SIGNALS_REFERENCE.md + Core 12 cheatsheet)

> **Source proposal.** `docs/design/tui-vfx-mixed-signals-recipe-surface-proposal.md` v0.3.0 — phases α (Move 4) and β (Move 2) per §8.6 phase ordering. The Decision section at the foot of the proposal green-lights both as doc-only and unblocked.
>
> **Status.** Genuinely queued. OFPF audit confirms there is no `docs/templates/signals.toml`, no `docs/generated/SIGNALS_REFERENCE.md`, and no `cargo xtask docs signals` subcommand today. Both phases are net-new doc-only work. Phase γ (the `tui_vfx_recipes::signals` module) is in flight as a separate packet and is independent of α/β — α and β can land before, after, or alongside γ.
>
> **Risk tier.** S — doc generation only; no runtime code changes; no schema changes. The only ripple is into the `cargo xtask docs generate` chain (one new step) and one new editorial overlay file.
>
> **Sequencing.** Independent of every other in-flight packet. Lands cleanly in one focused junior session.

---

## Goal & motivation

A recipe author writing JSON today has no canonical reference for the signal palette they can reach. The mental model is "go grep recipes for examples" (proposal §1.4). `mixed-signals` exposes ~50 Signal-trait implementations (audited below); ~40 of them are reachable through `SignalSpec` JSON deserialization. There is a high-quality rustdoc layer on every primitive (`Sine`, `DampedSpring`, etc. all carry summary + parameters + formula + example), but nothing harvests it into a recipe-author-facing cheatsheet.

Phase α reuses the same pipeline that already generates `docs/generated/CAPABILITIES.md` from `docs/templates/capabilities.toml` + extracted effect metadata. Phase β is one curated section at the top of α's output — the "Core 12 — start here" entry list with a one-line description and a representative JSON snippet per signal.

Both phases are doc-only. There is **no new code in `mixed-signals`, no new code in `tui-vfx-recipes`, no schema change, no wire-format change**. The deliverables are:

- `docs/templates/signals.toml` — editorial overlay (only-overrides shape per Q2 default below).
- `xtask/src/docs/extract_signals_rustdoc.rs` (or equivalent) — extends the existing extractor to walk every `Signal` impl in mixed-signals.
- `xtask/src/docs/gen_signals_markdown.rs` — generator for `docs/generated/SIGNALS_REFERENCE.md`.
- `cargo xtask docs signals` subcommand wired into `xtask/src/main.rs`.
- The generated `docs/generated/SIGNALS_REFERENCE.md` itself (the "Core 12 — start here" section at the top, then per-family sections).
- Inclusion in the existing `cargo xtask docs generate` and `cargo xtask docs check` orchestration steps so freshness is enforced.

The combined packet exists because β is one section of α's output. Splitting them costs more (two scaffolding passes, two acceptance gates) than landing them together.

## Scope

**In scope.**

- The new `docs/templates/signals.toml` editorial overlay file. Shape mirrors `docs/templates/capabilities.toml` but smaller (only-overrides per Q2).
- A new xtask subcommand `cargo xtask docs signals` that produces `docs/generated/SIGNALS_REFERENCE.md`.
- A new extractor module that walks every `Signal`-impl `pub struct` in `mixed_signals::{generators,envelopes,physics,composition,noise,random,processing}` and harvests its rustdoc, parameter docs, and serde shape.
- The generated `docs/generated/SIGNALS_REFERENCE.md` with three structural elements:
  - **Top section: "Core 12 — start here"** — the curated list of twelve high-leverage signals, each with one-line summary and representative JSON snippet.
  - **Per-family sections** (Oscillators, Spatial coordinates, Envelopes, Physics, Noise, Composition operators, Processing) — every Signal impl, organized by family.
  - **Per-entry shape**: name, type signature, rustdoc summary, parameter table, JSON example.
- Wiring the new subcommand into the orchestration (`docs::generate()` calls signals; `docs::check()` validates signals freshness).
- Rustdoc audit of the Core 12 source files in mixed-signals — verify each carries a summary and parameter docs sufficient for the autogen output. **Flag** any gaps as a sibling packet (`mixed-signals-rustdoc-audit`); **do not block α/β** on upstream rustdoc fixes (per Risks).

**Out of scope.**

- Any code-level facade. The `tui_vfx_recipes::signals` module is **phase γ**, not this packet.
- Any change to the `SignalSpec` enum in mixed-signals (e.g. promoting physics into SignalSpec — that was Move 1 of the original proposal, replaced by the in-crate facade in §8 of the proposal and now phase γ).
- Any change to `Bindable*` types (`BindableF32`, `BindableColor`, signal arms — that's phase δ, gated on packet 1.2.A).
- Any rustdoc edits **inside mixed-signals** as part of α/β. The audit may identify gaps; addressing them is a separate sibling packet so this one stays doc-pipeline-only.
- Any new dependency for xtask. The extractor walks already-reachable types via the workspace's existing `tui-vfx-core → mixed-signals` dep edge.
- Recipe-side validation (e.g. cap on signal nesting depth — proposal §6 Q3). Stays for a future packet.

**Crates touched.**

- **`xtask`:** new modules under `xtask/src/docs/` (extractor, generator, parser for signals.toml). One new subcommand variant in `xtask/src/main.rs`. New step in `docs::generate()` and `docs::check()` in `xtask/src/docs/mod.rs`.
- **`docs/`:** new `templates/signals.toml`, new `generated/SIGNALS_REFERENCE.md`.
- **`Cargo.toml` (xtask):** if the extractor cannot reach mixed-signals through `tui-vfx-core`'s public re-exports, add a direct `mixed-signals = { workspace = true }` dep. Verify in pre-flight before assuming.
- **No source-code change in any tui-vfx crate. No source-code change in mixed-signals.**

## Pre-work checklist

```bash
# Daemon health.
ofpf-status
ofpf-stats

# Load the mixed-signals graph (the audit walks both repos).
ofpf-load --root /usr/projects/mixed-signals

# Read the source proposal — phases α and β are §8.6 + the Decision section.
sed -n '344,355p'  /usr/projects/tui-vfx/docs/design/tui-vfx-mixed-signals-recipe-surface-proposal.md
sed -n '514,569p'  /usr/projects/tui-vfx/docs/design/tui-vfx-mixed-signals-recipe-surface-proposal.md

# Read the model files α + β imitate.
ofpf-inspect xtask/src/docs/mod.rs
ofpf-inspect xtask/src/docs/extract_rustdoc.rs
ofpf-inspect xtask/src/docs/parse_toml.rs
ofpf-inspect xtask/src/docs/gen_markdown.rs
ofpf-inspect xtask/src/docs/merge.rs

# Confirm the existing capabilities pipeline outputs.
wc -l docs/templates/capabilities.toml docs/generated/CAPABILITIES.md

# Confirm xtask main.rs subcommand shape (model for adding `Signals` variant).
ofpf-extract xtask/src/main.rs DocsAction
ofpf-extract xtask/src/main.rs main

# Audit every Signal impl in mixed-signals — input to the autogen catalog.
ofpf-content "impl Signal for " --root /usr/projects/mixed-signals

# Confirm Core 12 candidate types exist (with their actual Rust names).
ofpf-defs Sine          --root /usr/projects/mixed-signals
ofpf-defs Triangle      --root /usr/projects/mixed-signals
ofpf-defs Ramp          --root /usr/projects/mixed-signals
ofpf-defs Keyframes     --root /usr/projects/mixed-signals
ofpf-defs Adsr          --root /usr/projects/mixed-signals
ofpf-defs DampedSpring  --root /usr/projects/mixed-signals     # NOT "Spring"
ofpf-defs PerlinNoise   --root /usr/projects/mixed-signals     # NOT "Perlin" or "Noise"
ofpf-defs SpatialNoise  --root /usr/projects/mixed-signals
ofpf-defs Add           --root /usr/projects/mixed-signals
ofpf-defs Multiply      --root /usr/projects/mixed-signals
ofpf-defs Mix           --root /usr/projects/mixed-signals
ofpf-defs Clamp         --root /usr/projects/mixed-signals

# Spot-check rustdoc quality on the Core 12 source files (input to the autogen).
ofpf-extract /usr/projects/mixed-signals/src/generators/cls_sine.rs Sine
ofpf-extract /usr/projects/mixed-signals/src/physics/cls_spring.rs DampedSpring
ofpf-extract /usr/projects/mixed-signals/src/noise/cls_perlin.rs PerlinNoise

# Confirm xtask's transitive reach to mixed-signals.
grep -n "mixed-signals\|mixed_signals" xtask/Cargo.toml
grep -n "mixed-signals\|mixed_signals" crates/tui-vfx-core/Cargo.toml
```

## Current-state audit

Captured 2026-04-26 from the librarian.

### The mixed-signals public surface — every Signal impl

Source: `ofpf-content "impl Signal for " --root /usr/projects/mixed-signals` returns the complete list of types that implement the `mixed_signals::Signal` trait. The audit found **~50 distinct Signal impls**, organized by family.

**Generators (`src/generators/`)** — periodic + utility:

| Type | File | Rustdoc summary | Notes |
|---|---|---|---|
| `Sine` | `cls_sine.rs:20` | ✓ "Sine wave oscillator." + formula + example | Core 12 |
| `Triangle` | `cls_triangle.rs:17` | ✓ | Core 12 |
| `Square` | `cls_square.rs:17` | ✓ | |
| `Sawtooth` | `cls_sawtooth.rs:17` | ✓ | |
| `Ramp` | `cls_ramp.rs:17` | ✓ | Core 12 |
| `Step` | `cls_step.rs:?` | ✓ (verify in pre-flight) | |
| `Pulse` | `cls_pulse.rs:?` | ✓ | |
| `Constant` | `cls_constant.rs:?` | ✓ | |
| `Keyframes` | `cls_keyframes.rs:61` | ✓ | Core 12 |
| `PhaseSine` | `cls_phase_sine.rs:31` | ✓ | composition operator |
| `PhaseAccumulator` | `cls_phase_accumulator.rs:?` | ✓ | composition operator |
| `CellDistanceSignal` | `cls_cell_distance.rs:?` | ✓ | spatial leaf |
| `SpatialCoordinateSignal` | `cls_spatial_coordinate.rs:148` | ✓ | spatial leaf |
| `SurfaceAngleSignal` | `cls_surface_angle.rs:37` | ✓ | spatial leaf |
| `SurfaceDistanceSignal` | `cls_surface_distance.rs:35` | ✓ | spatial leaf |

**Envelopes (`src/envelopes/`)**:

| Type | File | Notes |
|---|---|---|
| `Adsr` | `cls_adsr.rs:20` | Core 12 (Rust casing: `Adsr`, JSON: `adsr`) |
| `Impact` | `cls_impact.rs:?` | |
| `LinearEnvelope` | `cls_linear.rs:63` | |
| `LinearDecay` | `cls_linear_decay.rs:?` | |
| `ExponentialDecay` | `cls_exponential_decay.rs:?` | |

**Physics (`src/physics/`)** — 7 primitives, all serializable, **none in `SignalSpec` today**:

| Type | File | Notes |
|---|---|---|
| `DampedSpring` | `cls_spring.rs:34` | **Core 12** — note: actual name is `DampedSpring`, not `Spring` (proposal text used the JSON-ish "spring" verbiage). |
| `BouncingDrop` | `cls_bounce.rs:37` | The `Bounce` name from the proposal does not exist as a struct; `BouncingDrop` is the closest physics primitive. |
| `FrictionDecay` | `cls_decay.rs:?` | |
| `SimplePendulum` | `cls_pendulum.rs:?` | |
| `CircularOrbit` | `cls_orbit.rs:?` | |
| `BallisticTrajectory` | `cls_projectile.rs:?` | |
| `PointAttractor` | `cls_attractor.rs:?` | |

**Noise (`src/noise/` and `src/random/`)**:

| Type | File | Notes |
|---|---|---|
| `WhiteNoise` | `noise/cls_white_noise.rs` | |
| `PerlinNoise` | `noise/cls_perlin.rs:16` | **Core 12** — note: actual name is `PerlinNoise`, not `Perlin`. |
| `SeededRandom` | `random/cls_seeded_random.rs:?` | |
| `FastSeededRandom` | `random/?` | |
| `SpatialNoise` | `random/cls_spatial_noise.rs:20` | **Core 12** |
| `GaussianNoise` | `random/?` | |
| `PoissonNoise` | `random/?` | |
| `CorrelatedNoise` | `random/cls_correlated_noise.rs:20` | |
| `FastCorrelatedNoise` | `random/cls_fast_correlated_noise.rs:17` | |
| `PinkNoise` | `random/?` | |
| `FastPinkNoise` | `random/?` | |
| `PerCharacterNoise` | `random/?` | |
| `StudentTNoise` | `random/?` | |
| `ImpulseNoise` | `random/?` | |

**Composition (`src/composition/`)**:

| Type | File | Notes |
|---|---|---|
| `Add` | `cls_add.rs:14` | **Core 12** |
| `Multiply` | `cls_multiply.rs:14` | **Core 12** |
| `Mix` | `cls_mix.rs:17` | **Core 12** |
| `WeightedMix` | `cls_weighted_mix.rs:?` | |
| `VcaCentered` | `cls_vca_centered.rs:?` | |
| `FrequencyMod` | (not present as struct — composition op via SignalSpec arm only) | |

**Processing (`src/processing/`)**:

| Type | File | Notes |
|---|---|---|
| `Clamp` | `cls_clamp.rs:10` | **Core 12** |
| Quantize / Remap / Invert / Abs / Normalized / Clipper / Lowpass / SVF / Biquad | various | |

**Test-only / construction-helper Signal impls** (must be **excluded** from the autogen catalog):

| Type | File | Why exclude |
|---|---|---|
| `RawFrequency` | `generators/test_cls_phase_accumulator.rs:11` | test fixture |
| `NanSignal` | `generators/test_cls_phase_accumulator.rs:118` | test fixture |
| `OverflowSignal`, `UnderflowSignal`, `LinearSignal`, `StepSignal`, `RawSignal`, `RawValue`, `BipolarConstant`, `UnitConstant`, `UnitSignal`, `PositiveSignal`, `ContextSignal`, `AnalyticSlopeSignal`, `ConstantSignal` | various test/internal | wrappers, decorators, not author-facing |
| `Box<dyn Signal>`, `std::sync::Arc<dyn Signal>` | trait blanket impls | not types |

**Rustdoc quality.** Spot-checks on `Sine` (`cls_sine.rs:11`), `DampedSpring` (`cls_spring.rs:10`), and the SignalSpec enum (`signal_spec.rs:39`) confirm the rustdoc layer is **already in good shape** for the Core 12 — every checked file carries a doc-comment summary, per-field parameter docs, and (for physics + the more involved generators) a worked example. The autogen will land cleanly on this surface; the audit's purpose is to flag the few entries that are weaker so a sibling rustdoc-audit packet can fill them in.

### The existing `cargo xtask docs generate` pipeline (the model for α)

`xtask/src/docs/mod.rs` exposes the `generate()` entry point at line 32. The pipeline shape is:

```
extract_rustdoc::extract()  → RustdocData      (runtime ConfigSchema introspection over effect crates)
parse_toml::parse()         → CapabilitiesManifest  (editorial overlay from docs/templates/capabilities.toml)
validate_coverage::validate(...)                (every variant in code has a TOML entry)
merge::merge(rustdoc, toml) → MergedManifest
gen_markdown::generate(merged)   → docs/generated/CAPABILITIES.md
gen_json::generate(merged)       → docs/generated/capabilities.json
gen_effect_schemas::generate(merged)
gen_ai_context::generate(merged)
```

Key insight from `extract_rustdoc.rs:67–110`: the existing extractor does **not** parse rustdoc JSON. It uses **runtime ConfigSchema introspection** (`MaskSpec::schema()`, `FilterSpec::schema()`, etc.) plus `terse_description()` / `key_parameters()` calls on effect types. The "rustdoc" in the file name refers to the documentation that lives **alongside** these schema-bridge methods.

**Implication for the signals extractor.** mixed-signals primitives do NOT carry `ConfigSchema` impls. The signals extractor cannot reuse the existing runtime-introspection mechanism directly — it must walk the `SignalSpec` enum's serde shape (already exposed as runtime-readable via `mixed_signals::types::SignalSpec`'s `Serialize` derive) and pair each variant with its corresponding `pub struct` rustdoc. See Q1 below for the recommended approach.

### `docs/templates/capabilities.toml` (the model for `signals.toml`)

`capabilities.toml` is 1399 LOC. It is a **fully editorial** TOML — every effect variant has at least an `ai_hint` and `use_cases`. Coverage is enforced by `validate_coverage::validate()`: a variant in code without a TOML entry **fails the build**.

Sample entry (`docs/templates/capabilities.toml:50–56`):

```toml
[effects.masks.Wipe]
use_cases = ["transition", "reveal", "cinematic"]
energy = "calm"
complexity = "simple"
ai_hint = """Classic film-style linear reveal/hide.
reveal/hide: WipeDirection (16 variants - cardinal, diagonal, aliases, center-out).
soft_edge: blur transition (0.1-0.2 for smooth, 0 for hard edge)."""
```

The corresponding rustdoc-extracted fields (`name`, `description`, parameter list with `ty` and `doc`) come from the code; the TOML adds `use_cases`, `energy`, `complexity`, and `ai_hint`. **For α, signals.toml's editorial fields can be a smaller subset** (Q2 recommends only-overrides — entries appear only when the autogen output needs editorial enrichment).

### `docs/generated/CAPABILITIES.md` (the model for `SIGNALS_REFERENCE.md`)

216 LOC. Structure (verbatim from the file):

```
<!-- DO NOT EDIT - This file is generated by `cargo xtask docs generate` -->
<!-- <FILE>...</FILE> - <DESC>...</DESC> -->
<!-- <VERS>VERSION: 2.2.3</VERS> -->

# tui-vfx Capabilities Reference

[opening paragraph]

---

## Table of Contents
1. [Masks (Transition Shapes)](#masks-transition-shapes)
…

---

## Masks (Transition Shapes)

| Effect | Description | Use Cases |
|--------|-------------|----------|
| **Blinds** | Venetian blinds effect | transition, presentation |
…
```

The shape `SIGNALS_REFERENCE.md` should mirror — same metadata envelope (autogen banner + DESC + VERS), same `## Table of Contents`, same per-section table layout. The only structural addition is the **"Core 12 — start here" section** at the top, which is what β contributes.

## Phase α specification

### `docs/templates/signals.toml`

Shape mirrors `capabilities.toml` but smaller (only-overrides per Q2 default — entries only appear when needed for editorial enrichment beyond what rustdoc provides).

```toml
# tui-vfx Signals Manifest (editorial overlay)
#
# Editorial overlay for SIGNALS_REFERENCE.md autogen. Entries here are MERGED
# with rustdoc extracted from mixed-signals; rustdoc carries name + summary +
# parameter table; this file adds use_cases, recipe_hint, and the Core 12 list.
#
# Run `cargo xtask docs signals` (or `cargo xtask docs generate`) to regenerate
# docs/generated/SIGNALS_REFERENCE.md.

# <FILE>docs/templates/signals.toml</FILE> - <DESC>Editorial overlay for signal reference autogen</DESC>
# <VERS>VERSION: 0.1.0</VERS>
# <WCTX>Phase α + β: introduce signals editorial overlay; lists the Core 12 and adds recipe-author hints to entries that need them.</WCTX>
# <CLOG>0.1.0: initial overlay — Core 12 list (data-driven per Q3); editorial entries for the Core 12; advanced signals carry no entry (rustdoc-only autogen suffices per Q2 only-overrides).</CLOG>

[meta]
version = "0.1.0"
description = "tui-vfx signal reference editorial overlay"

# ═══════════════════════════════════════════════════════════════════════════════
# CORE 12 — driven by the signals.toml so editors can change the cheatsheet
# without xtask code changes (Q3 recommended default).
# Each name is the SignalSpec discriminant in snake_case (the JSON "type" value).
# ═══════════════════════════════════════════════════════════════════════════════

[core_12]
order = [
  "sine",
  "triangle",
  "ramp",
  "keyframes",
  "perlin",          # PerlinNoise — JSON discriminant is "perlin"
  "spatial_noise",
  "adsr",
  "spring",          # DampedSpring — JSON discriminant is "spring" (verify in SignalSpec; if it's "damped_spring", correct here)
  "add",
  "multiply",
  "mix",
  "clamp",
]

# ═══════════════════════════════════════════════════════════════════════════════
# EDITORIAL ENTRIES (only-overrides — entries here add hints beyond rustdoc)
# ═══════════════════════════════════════════════════════════════════════════════

[signals.sine]
use_cases = ["pulsing", "breathing", "musical-oscillation"]
recipe_hint = """Smooth periodic oscillation. Most-reached-for signal in tui-vfx recipes (50+ uses).
For TUI animations use frequency in cycles-per-second (0.5–2 Hz feels natural).
Pair with .normalized() upstream when you need [0, 1] instead of [-1, 1]."""

[signals.spring]
use_cases = ["physics", "tactile", "settle", "overshoot"]
recipe_hint = """Underdamped → bouncy; critically damped → snap-into-place; overdamped → glide.
Most popular physics primitive in production recipes (~9 uses).
Tune `damping` first; `stiffness` controls speed."""

# ... entries for the other 10 Core 12 signals follow the same shape.
# Advanced signals (Square, Sawtooth, GaussianNoise, etc.) get no entry —
# rustdoc-only output is sufficient.
```

### The `cargo xtask docs signals` subcommand

Wires into the existing `DocsAction` enum at `xtask/src/main.rs:37`:

```rust
#[derive(Subcommand)]
enum DocsAction {
    Generate,
    Check,
    AiContext,
    Markdown,
    Validate,
    Scaffold { #[arg(long)] write: bool },

    // ─── NEW (this packet) ────────────────────────────────────────────
    /// Generate SIGNALS_REFERENCE.md from mixed-signals rustdoc + signals.toml
    Signals,
    /// Check that SIGNALS_REFERENCE.md is up-to-date (CI gate)
    SignalsCheck,
    /// Validate signals.toml: every named signal exists in mixed-signals,
    /// and every Core 12 entry exists in the autogen catalog.
    SignalsValidate,
    // ──────────────────────────────────────────────────────────────────

    // API documentation
    Api,
    ApiCheck,
    ApiValidate,
    ApiScaffold { #[arg(long)] write: bool },
}
```

Match-arm body in `main()`:

```rust
DocsAction::Signals => docs::signals(),
DocsAction::SignalsCheck => docs::signals_check(),
DocsAction::SignalsValidate => docs::signals_validate(),
```

`docs::signals()` orchestrates:

```rust
pub fn signals() -> Result<()> {
    println!("{}", "Generating SIGNALS_REFERENCE.md...".bold());
    let signal_data = extract_signals_rustdoc::extract()?;
    let toml_data   = parse_signals_toml::parse()?;
    validate_signals::validate(&signal_data, &toml_data)?;
    let merged      = merge_signals::merge(signal_data, toml_data)?;
    gen_signals_markdown::generate(&merged)?;
    println!("{}", "✓ SIGNALS_REFERENCE.md generated".green().bold());
    Ok(())
}
```

`docs::generate()` at `xtask/src/docs/mod.rs:32` gains one new step right after `gen_ai_context::generate(&merged)?;`:

```rust
println!("  {} Generating SIGNALS_REFERENCE.md...", "→".dimmed());
let signal_data = extract_signals_rustdoc::extract()?;
let signal_toml = parse_signals_toml::parse()?;
validate_signals::validate(&signal_data, &signal_toml)?;
let signal_merged = merge_signals::merge(signal_data, signal_toml)?;
gen_signals_markdown::generate(&signal_merged)?;
```

`docs::check()` similarly gains one new `check_file(...)` call for `docs/generated/SIGNALS_REFERENCE.md`.

### The autogen mechanism

The existing `extract_rustdoc.rs` uses runtime `ConfigSchema` introspection on effect-spec types (MaskSpec, FilterSpec, etc.). Mixed-signals primitives carry **no `ConfigSchema` impl**. Two viable extractor strategies:

**Strategy A (recommended per Q1) — extend the existing extractor with a SignalSpec walker.**

`SignalSpec` (`mixed-signals/src/types/signal_spec.rs:45`) is a tagged enum with `#[serde(tag = "type", rename_all = "snake_case")]`. Each variant maps to a primitive struct (e.g. `SignalSpec::Sine{...}` → `mixed_signals::Sine`). The walker:

1. Use `serde_json` round-trip to enumerate every variant of `SignalSpec` (build one of each via the existing `orc_signal_spec_build` constructors, serialize, harvest the `"type"` discriminant + field shape).
2. For each variant, map the discriminant to a primitive struct name (table lives in `signals.toml [variant_to_struct]` overlay or hard-coded in the extractor — junior decision, lean on the extractor unless variant set is volatile).
3. Pair the variant + serde shape with the rustdoc text for the corresponding `pub struct` from a parsed `cargo doc --output-format json` blob (or, if that's heavy, the simpler approach: read the source files directly with the existing `walkdir` xtask dep and parse `///` doc comments above the `pub struct X` line and the `pub field_name: T,` lines).

The simple `walkdir` + line-parsing approach is the lowest-friction first cut. The existing extractor at `extract_rustdoc.rs` does similar pattern-based work and the precedent is set.

**Strategy B — parse `cargo doc --output-format json`.**

Heavier setup; produces structured rustdoc JSON. Worth doing if α/β grow into a multi-crate rustdoc audit pipeline; overkill for one cheatsheet. Defer.

### Output structure: `docs/generated/SIGNALS_REFERENCE.md`

```
<!-- DO NOT EDIT - This file is generated by `cargo xtask docs signals` -->
<!-- Edit docs/templates/signals.toml or source rustdoc in mixed-signals/src/ instead -->

<!-- <FILE>docs/generated/SIGNALS_REFERENCE.md</FILE> - <DESC>Recipe-author signal reference</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->

# tui-vfx Signals Reference

Recipe-author cheatsheet for the mixed-signals palette reachable through
`SignalSpec` JSON deserialization. The "Core 12" list at the top is the
recommended starting point. Per-family sections below document every
signal primitive available.

---

## Table of Contents

1. [Core 12 — start here](#core-12--start-here)
2. [Oscillators](#oscillators)
3. [Spatial coordinates](#spatial-coordinates)
4. [Envelopes](#envelopes)
5. [Physics](#physics)
6. [Noise](#noise)
7. [Composition operators](#composition-operators)
8. [Processing](#processing)

---

## Core 12 — start here

The twelve highest-leverage signals for recipe authoring. Pick from this list
first; everything else is in the per-family sections below.

| # | Signal | One-line | JSON snippet |
|---|--------|----------|--------------|
| 1 | **sine** | Smooth periodic oscillation | `{"type": "sine", "frequency": 1, "amplitude": 1}` |
| 2 | **triangle** | Linear triangular oscillation | `{"type": "triangle", "frequency": 1}` |
| 3 | **ramp** | Linear interpolation start→end over duration | `{"type": "ramp", "start": 0, "end": 1, "duration": 1}` |
| 4 | **keyframes** | Piecewise linear curve from (t, v) pairs | `{"type": "keyframes", "keyframes": [[0,0],[0.5,1],[1,0]]}` |
| 5 | **perlin** | Smooth coherent noise (organic textures) | `{"type": "perlin", "seed": 0, "scale": 1, "octaves": 3}` |
| 6 | **spatial_noise** | Noise indexed by sample (cell_x, cell_y) | `{"type": "spatial_noise", "seed": 0, "frequency": 1}` |
| 7 | **adsr** | Attack-Decay-Sustain-Release envelope | `{"type": "adsr", "attack": 0.1, "decay": 0.1, "sustain": 0.7, "release": 0.2}` |
| 8 | **spring** | Damped harmonic motion (bouncy / snap / glide) | `{"type": "spring", "stiffness": 12, "damping": 0.4, "target": 1}` |
| 9 | **add** | Sum two signals | `{"type": "add", "a": {...}, "b": {...}}` |
| 10 | **multiply** | Multiply two signals | `{"type": "multiply", "a": {...}, "b": {...}}` |
| 11 | **mix** | Linear blend between two signals | `{"type": "mix", "a": {...}, "b": {...}, "mix": 0.5}` |
| 12 | **clamp** | Constrain a signal to [min, max] | `{"type": "clamp", "signal": {...}, "min": 0, "max": 1}` |

---

## Oscillators

### sine

Sine wave oscillator. Produces a smooth periodic oscillation following the
sine function. Output is bipolar [-1, 1] scaled by amplitude and shifted
by offset.

Formula: `output = offset + amplitude * sin(2π * (frequency * t + phase))`

| Field | Type | Default | Description |
|---|---|---|---|
| `frequency` | f32 | 1.0 | Frequency in Hz (cycles per second) |
| `amplitude` | f32 | 1.0 | Output amplitude (scales the 0..1 range) |
| `offset` | f32 | 0.0 | DC offset (shifts the output) |
| `phase` | f32 | 0.0 | Phase shift (normalized 0..1) |

```jsonc
{"type": "sine", "frequency": 1.0, "amplitude": 1.0, "offset": 0.0, "phase": 0.0}
```

**Recipe hint** (from signals.toml): Smooth periodic oscillation. Most-reached-for signal in tui-vfx recipes (50+ uses). For TUI animations use frequency in cycles-per-second (0.5–2 Hz feels natural). Pair with .normalized() upstream when you need [0, 1] instead of [-1, 1].

**Use cases:** pulsing, breathing, musical-oscillation

### triangle
…
```

Each entry merges three sources:
- **From rustdoc** (mixed-signals source): summary, formula, parameter table.
- **From SignalSpec serde shape**: JSON snippet with default values.
- **From signals.toml** (only-overrides): `recipe_hint`, `use_cases`.

## Phase β specification

The "Core 12 — start here" section is **one section of the SIGNALS_REFERENCE.md output** (top of doc). The list itself is data — `signals.toml [core_12].order = [...]` per Q3.

### Confirmed Core 12 (verified via `ofpf-defs` against `/usr/projects/mixed-signals`)

The proposal-recommended list versus the actual struct names in mixed-signals:

| # | Proposal name | Actual mixed-signals type | SignalSpec discriminant (snake_case) | Status |
|---|---|---|---|---|
| 1 | Sine | `Sine` (`generators/cls_sine.rs:20`) | `sine` | ✓ exists |
| 2 | Triangle | `Triangle` (`generators/cls_triangle.rs:17`) | `triangle` | ✓ exists |
| 3 | Ramp | `Ramp` (`generators/cls_ramp.rs:17`) | `ramp` | ✓ exists |
| 4 | Keyframes | `Keyframes` (`generators/cls_keyframes.rs:61`) | `keyframes` | ✓ exists |
| 5 | Noise | **(does not exist as plain `Noise`)** | — | **✗ flag — substitute `PerlinNoise` (discriminant `perlin`) per next row** |
| 6 | SpatialNoise | `SpatialNoise` (`random/cls_spatial_noise.rs:20`) | `spatial_noise` | ✓ exists |
| 7 | ADSR | `Adsr` (`envelopes/cls_adsr.rs:20`) | `adsr` | ✓ exists (Rust casing `Adsr`, JSON `adsr`) |
| 8 | DampedSpring | `DampedSpring` (`physics/cls_spring.rs:34`) | likely `spring` (verify against SignalSpec — physics is currently the **parallel channel** outside SignalSpec; phase γ collapses this. Until γ lands, the JSON discriminant for spring is the *parallel-channel* field shape, not a SignalSpec arm. **Phase α/β must document this asymmetry** — see Risks.) | ✓ exists |
| 9 | Add | `Add` (`composition/cls_add.rs:14`) | `add` | ✓ exists |
| 10 | Multiply | `Multiply` (`composition/cls_multiply.rs:14`) | `multiply` | ✓ exists |
| 11 | Mix | `Mix` (`composition/cls_mix.rs:17`) | `mix` | ✓ exists |
| 12 | Clamp | `Clamp` (`processing/cls_clamp.rs:10`) | `clamp` | ✓ exists |

**Recommended Core 12 (corrected names):**

```
sine, triangle, ramp, keyframes, perlin, spatial_noise,
adsr, spring, add, multiply, mix, clamp
```

The proposal's "Noise" is replaced by `perlin` (canonical organic noise, the proposal's stated intent in §2.2). The proposal's "Bounce" was not in the recommendation but worth flagging: the closest actual type is `BouncingDrop`, not `Bounce` — if the cheatsheet ever needs a bounce primitive, use the actual struct name.

The proposal also implicitly recommended `sample_norm_x` and `sample_radius` (spatial leaves) for the Core 12 cheatsheet (proposal §2.2 table). The above list omits those in favor of the four composition + processing operators (`add`, `multiply`, `mix`, `clamp`) per evidence the proposal cited 12 categories with 2 spatial picks. Resolve at junior's discretion: if the user wants spatial leaves in Core 12 ahead of `mix`/`clamp`, swap them. The Q3 recommendation (data-driven Core 12 in `signals.toml`) makes this trivial to change post-landing.

## Open architectural questions

Three small questions; each carries a recommended default the junior can apply.

### Q1 — Where does the signals-rustdoc extractor live?

| Option | Trade-off |
|---|---|
| A — Extend the existing `xtask/src/docs/extract_rustdoc.rs` | One extractor module. Implies a `RustdocData` shape that holds both effect-metadata AND signal-metadata, which loses single-responsibility cohesion. |
| **B (recommended) — New parallel extractor module** `xtask/src/docs/extract_signals_rustdoc.rs` | Sibling to the existing one. Clean OFPF separation: signals pipeline (`extract_signals_rustdoc.rs`, `parse_signals_toml.rs`, `merge_signals.rs`, `gen_signals_markdown.rs`, `validate_signals.rs`) mirrors the capabilities pipeline shape one-for-one. Each module stays under the `fnc_`/`orc_` size budget. |

**Recommended default: Option B.** Per OFPF (one logical unit per file) the signals pipeline is its own pipeline; mixing it into the capabilities pipeline produces a sprawling extractor file that loses purpose-clarity. The new modules total ~5 files and stay independent.

### Q2 — Is `signals.toml` all-editorial or only-overrides?

| Option | Trade-off |
|---|---|
| A — All-editorial (every signal has a TOML entry, even if just a stub) | Mirrors `capabilities.toml` exactly; `validate_coverage` becomes the same shape. Larger TOML; every new mixed-signals primitive forces a TOML edit even if the rustdoc is sufficient. |
| **B (recommended) — Only-overrides (TOML entries only when editorial enrichment is needed)** | Smaller TOML (one entry per Core 12 signal + maybe a handful of advanced entries that need extra hints). Every other signal flows through rustdoc-only autogen. Every new mixed-signals primitive shows up in the catalog automatically without requiring a TOML edit. |

**Recommended default: Option B (only-overrides).** Per Intention 24 (library changes earn their place), an empty TOML stub does not earn its row. Per Intention 26 (SSOT) the rustdoc is the source of truth for non-editorial fields; the TOML adds only what rustdoc cannot: project-flavored use-cases and recipe-author hints.

### Q3 — Where does the Core 12 list itself live?

| Option | Trade-off |
|---|---|
| A — Hard-coded in xtask source | Compile-time guarantee the list is a valid SignalSpec discriminant set (typo → build break). Editing the cheatsheet requires editing Rust + `cargo xtask docs signals`. |
| **B (recommended) — Data-driven via `[core_12].order = [...]` in `signals.toml`** | Editors adjust the cheatsheet without touching xtask. Validation (Q1's `validate_signals`) ensures every name in the list resolves to a real signal in the autogen catalog (typo → `cargo xtask docs signals` fails with a clear "unknown discriminant" error). |

**Recommended default: Option B (toml-driven).** The Core 12 is editorial. It will change as the cheatsheet matures (e.g. swapping `mix` for `sample_norm_x`). Data-driven is the lower-friction path; the validator covers the typo-safety case Option A would have provided structurally.

## Step-by-step implementation plan

OFPF discipline: one file at a time, write tests first where applicable, confirm clippy clean between phases, commit interim work between phases.

### Phase 1 — Audit + initial signals.toml

**Step 1.1.** Run the §Pre-work checklist. Capture the current state of every Signal impl and every SignalSpec discriminant (rough catalog already in this packet's §Current-state audit — verify against today's mixed-signals).

**Step 1.2.** Spot-check rustdoc quality on the Core 12 source files. For each, confirm the `pub struct` carries:
- a top-level doc comment (summary line, optional description),
- per-field doc comments on every public field,
- ideally a worked example.

If any Core 12 file is below this bar, **flag the gap to the user** (likely a sibling `mixed-signals-rustdoc-audit` packet) but **do not block α/β** on the fix — the autogen will produce a less-rich entry for that signal until the upstream rustdoc lands. Per `feedback_rustdoc_when_editing.md` the audit is a fair surface to surface.

**Step 1.3.** Create `docs/templates/signals.toml` with:
- `[meta]` block (version, description),
- `[core_12].order = [...]` per Q3 (the corrected Core 12 list above),
- editorial entries (`[signals.<name>]`) for each of the Core 12 with `use_cases` and `recipe_hint`.

Metadata envelope per CLAUDE.md conventions.

**Step 1.4.** Commit interim: `Add docs/templates/signals.toml editorial overlay (signal-facade phase α/β step 1)`.

### Phase 2 — Extractor + parser modules

**Step 2.1.** Pre-edit: `ofpf-inspect xtask/src/docs/extract_rustdoc.rs` and `ofpf-inspect xtask/src/docs/parse_toml.rs` to nail down the shape the signals pipeline mirrors.

**Step 2.2.** Create `xtask/src/docs/extract_signals_rustdoc.rs`. Public surface:

```rust
/// Extracted catalog of every Signal primitive in mixed-signals.
#[derive(Debug, Default)]
pub struct SignalsRustdocData {
    /// Signals organized by family.
    pub families: HashMap<SignalFamily, Vec<SignalDoc>>,
    /// Flat lookup by SignalSpec snake_case discriminant.
    pub by_discriminant: HashMap<String, SignalDoc>,
}

#[derive(Debug, Clone)]
pub struct SignalDoc {
    pub discriminant: String,         // e.g. "sine"
    pub struct_name: String,          // e.g. "Sine"
    pub source_path: String,          // e.g. "mixed-signals/src/generators/cls_sine.rs"
    pub family: SignalFamily,
    pub summary: String,              // first line of struct rustdoc
    pub description: String,          // full doc-comment text
    pub fields: Vec<SignalFieldDoc>,
}

#[derive(Debug, Clone)]
pub struct SignalFieldDoc {
    pub name: String,
    pub ty: String,
    pub doc: String,
    pub default: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SignalFamily { Oscillator, Spatial, Envelope, Physics, Noise, Composition, Processing }

pub fn extract() -> Result<SignalsRustdocData> { /* impl */ }
```

The implementation: walk `mixed-signals/src/{generators,envelopes,physics,composition,noise,random,processing}` with `walkdir` (already an xtask dep). For each `cls_*.rs`, parse the file for the `pub struct X { ... }` block, harvest `///` comments above the struct and each field. Map the file path to a `SignalFamily` via the directory name. Skip test-only files (`test_*.rs`, files in `tests/`) and the test-fixture types listed in §Current-state audit.

For the `discriminant` field: parse `mixed-signals/src/types/signal_spec.rs` for the `#[serde(tag = "type", rename_all = "snake_case")]` enum and harvest variant names. Map struct-name → discriminant via a small lookup table in the extractor (e.g. `Sine → "sine"`, `Adsr → "adsr"`, `PerlinNoise → "perlin"`); for variants where the struct name doesn't match the discriminant casing-wise (e.g. `PerlinNoise` → `perlin`), the SignalSpec definition is the source of truth.

**Step 2.3.** Create `xtask/src/docs/parse_signals_toml.rs`. Mirrors `parse_toml.rs:14–60`:

```rust
#[derive(Debug, Deserialize, Serialize)]
pub struct SignalsManifest {
    pub meta: MetaSection,
    #[serde(default)]
    pub core_12: Core12Section,
    #[serde(default)]
    pub signals: HashMap<String, SignalEntry>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Core12Section { pub order: Vec<String> }

#[derive(Debug, Deserialize, Serialize)]
pub struct SignalEntry {
    #[serde(default)]
    pub use_cases: Vec<String>,
    #[serde(default)]
    pub recipe_hint: Option<String>,
}

pub fn parse() -> Result<SignalsManifest> {
    let path = Path::new("docs/templates/signals.toml");
    let contents = fs::read_to_string(path)
        .with_context(|| format!("Failed to read {}", path.display()))?;
    toml::from_str(&contents).with_context(|| "Failed to parse signals.toml")
}
```

**Step 2.4.** Create `xtask/src/docs/validate_signals.rs`. Three checks:

1. Every entry in `[signals.<name>]` has a corresponding signal in the rustdoc catalog (typo guard).
2. Every name in `[core_12].order` is a valid discriminant in the rustdoc catalog (typo guard).
3. Optional warning (not error): every Core 12 entry has a `[signals.<name>]` block with at least a `recipe_hint` (Core 12 should be editorial-rich; advanced signals can be rustdoc-only).

Failure produces a clear error message: `"signals.toml [signals.foo] names a signal that does not exist in mixed-signals. Did you mean: <closest-match>?"`.

**Step 2.5.** Run `cargo build --workspace`. Confirm the new modules compile in isolation.

**Step 2.6.** Commit interim: `Add signals extractor + parser + validator (phase α step 2)`.

### Phase 3 — Merge + markdown generator

**Step 3.1.** Create `xtask/src/docs/merge_signals.rs`. Combine `SignalsRustdocData` with `SignalsManifest`:

```rust
pub struct MergedSignals {
    pub version: String,
    pub core_12: Vec<MergedSignal>,    // in [core_12].order
    pub by_family: BTreeMap<SignalFamily, Vec<MergedSignal>>,
}

pub struct MergedSignal {
    pub doc: SignalDoc,
    pub editorial: Option<SignalEntry>,
    pub example_json: String,           // generated from SignalSpec serde defaults
}

pub fn merge(rustdoc: SignalsRustdocData, toml: SignalsManifest) -> Result<MergedSignals> { /* impl */ }
```

The example JSON for each signal is built by serializing the SignalSpec variant with its default field values (the `default_*()` functions in `signal_spec.rs:399–465` provide them).

**Step 3.2.** Create `xtask/src/docs/gen_signals_markdown.rs`. Mirrors `gen_markdown.rs` shape:

```rust
const OUTPUT_PATH: &str = "docs/generated/SIGNALS_REFERENCE.md";

pub fn generate(merged: &MergedSignals) -> Result<()> { /* fs::write */ }
pub fn render(merged: &MergedSignals) -> Result<String> {
    // Header + ToC + Core 12 table + per-family sections
}
```

The `render` body builds the structure shown in §"Output structure" above.

**Step 3.3.** Run `cargo build --workspace`. Run `cargo xtask docs signals` (dry-run from a working tree). Inspect the output `docs/generated/SIGNALS_REFERENCE.md`.

**Step 3.4.** Commit interim: `Add signals merge + markdown generator; emit SIGNALS_REFERENCE.md (phase α step 3)`.

### Phase 4 — Wire into orchestration

**Step 4.1.** Edit `xtask/src/main.rs:37` to add the three new `DocsAction` variants (`Signals`, `SignalsCheck`, `SignalsValidate`) and their match arms in `main()`.

**Step 4.2.** Edit `xtask/src/docs/mod.rs`:
- Add three new top-level functions (`signals`, `signals_check`, `signals_validate`) mirroring the existing `markdown` / `check` / `validate` shape.
- Add the new step inside `generate()` so `cargo xtask docs generate` produces SIGNALS_REFERENCE.md as part of the existing workflow.
- Add a `check_file(...)` call inside `check()` for `docs/generated/SIGNALS_REFERENCE.md`.

**Step 4.3.** Bump `xtask/src/main.rs` and `xtask/src/docs/mod.rs` metadata versions per CLAUDE.md conventions; update CLOG.

**Step 4.4.** Run `cargo xtask docs generate`. Confirm SIGNALS_REFERENCE.md is regenerated and matches the standalone `cargo xtask docs signals` output (idempotent).

**Step 4.5.** Run `cargo xtask docs check`. Confirm it passes against the just-generated file (round-trip stability).

**Step 4.6.** Commit interim: `Wire SIGNALS_REFERENCE.md into docs orchestration (phase α step 4)`.

### Phase 5 — Final verification + commit the generated doc

**Step 5.1.** Run the full §Verification commands block.

**Step 5.2.** Commit `docs/generated/SIGNALS_REFERENCE.md` and `docs/templates/signals.toml`. Final commit message: `Phase α + β complete: SIGNALS_REFERENCE.md autogen + Core 12 cheatsheet`.

## Code snippets

### Editorial entry for `sine` (in `signals.toml`)

```toml
[signals.sine]
use_cases = ["pulsing", "breathing", "musical-oscillation"]
recipe_hint = """Smooth periodic oscillation. Most-reached-for signal in tui-vfx recipes (50+ uses).
For TUI animations use frequency in cycles-per-second (0.5–2 Hz feels natural).
Pair with `.normalized()` upstream when you need [0, 1] instead of [-1, 1]."""
```

### xtask subcommand structure (the signals-pipeline orchestrator in `mod.rs`)

```rust
/// Generate SIGNALS_REFERENCE.md from mixed-signals rustdoc + signals.toml overlay.
pub fn signals() -> Result<()> {
    println!("{}", "Generating SIGNALS_REFERENCE.md...".bold());

    println!("  {} Extracting Signal-impl rustdoc from mixed-signals...", "→".dimmed());
    let signal_data = extract_signals_rustdoc::extract()?;

    println!("  {} Parsing signals.toml...", "→".dimmed());
    let toml_data = parse_signals_toml::parse()?;

    println!("  {} Validating signals coverage...", "→".dimmed());
    validate_signals::validate(&signal_data, &toml_data)?;

    println!("  {} Merging sources...", "→".dimmed());
    let merged = merge_signals::merge(signal_data, toml_data)?;

    println!("  {} Generating SIGNALS_REFERENCE.md...", "→".dimmed());
    gen_signals_markdown::generate(&merged)?;

    println!("{}", "✓ SIGNALS_REFERENCE.md generated successfully".green().bold());
    Ok(())
}
```

### Generated entry for `sine` (one section of SIGNALS_REFERENCE.md showing the rustdoc + editorial merge)

```markdown
### sine

Sine wave oscillator. Produces a smooth periodic oscillation following the
sine function. Output is bipolar [-1, 1] scaled by amplitude and shifted
by offset.

Formula: `output = offset + amplitude * sin(2π * (frequency * t + phase))`

| Field | Type | Default | Description |
|---|---|---|---|
| `frequency` | f32 | 1.0 | Frequency in Hz (cycles per second) |
| `amplitude` | f32 | 1.0 | Output amplitude (scales the 0..1 range) |
| `offset` | f32 | 0.0 | DC offset (shifts the output) |
| `phase` | f32 | 0.0 | Phase shift (normalized 0..1) |

```jsonc
{"type": "sine", "frequency": 1.0, "amplitude": 1.0, "offset": 0.0, "phase": 0.0}
```

**Recipe hint:** Smooth periodic oscillation. Most-reached-for signal in tui-vfx recipes (50+ uses). For TUI animations use frequency in cycles-per-second (0.5–2 Hz feels natural). Pair with `.normalized()` upstream when you need [0, 1] instead of [-1, 1].

**Use cases:** pulsing, breathing, musical-oscillation
```

The first paragraph + formula + parameter table come from the `Sine` rustdoc in `mixed-signals/src/generators/cls_sine.rs:11–28`. The JSON snippet is built from `SignalSpec::Sine`'s serde defaults. The "Recipe hint" and "Use cases" lines come from `signals.toml [signals.sine]`.

## Test plan

### Round-trip tests for the signals pipeline

Three load-bearing tests live in `xtask/src/docs/extract_signals_rustdoc.rs` (or a peer `test_*` file per OFPF, depending on xtask's test layout — `ofpf-tests xtask/src/docs/extract_rustdoc.rs` to confirm the existing convention).

**Test 1 — catalog completeness.** Every `Signal`-impl `pub struct` in mixed-signals (excluding test fixtures) appears in the autogen catalog with at least a non-empty `summary`. Asserts the extractor does not silently drop primitives.

```rust
#[test]
fn every_signal_primitive_has_a_catalog_entry() {
    let data = extract().expect("extraction failed");
    let known_primitives: &[&str] = &[
        "Sine", "Triangle", "Square", "Sawtooth", "Ramp", "Step", "Pulse",
        "Constant", "Keyframes", "PhaseSine", "PhaseAccumulator",
        "SpatialCoordinateSignal", "SurfaceAngleSignal", "SurfaceDistanceSignal",
        "CellDistanceSignal",
        "Adsr", "Impact", "LinearEnvelope", "LinearDecay", "ExponentialDecay",
        "DampedSpring", "BouncingDrop", "FrictionDecay", "SimplePendulum",
        "CircularOrbit", "BallisticTrajectory", "PointAttractor",
        "WhiteNoise", "PerlinNoise", "SeededRandom", "FastSeededRandom",
        "SpatialNoise", "GaussianNoise", "PoissonNoise", "CorrelatedNoise",
        "FastCorrelatedNoise", "PinkNoise", "FastPinkNoise",
        "PerCharacterNoise", "StudentTNoise", "ImpulseNoise",
        "Add", "Multiply", "Mix", "WeightedMix", "VcaCentered",
        "Clamp", "Quantize", "Remap", "Invert", "Abs",
    ];
    let catalog: HashSet<&str> = data.by_discriminant.values()
        .map(|s| s.struct_name.as_str()).collect();
    for name in known_primitives {
        assert!(catalog.contains(name), "missing primitive: {name}");
    }
}
```

**Test 2 — unknown signal in toml fails fast.** A `[signals.<name>]` entry that names a signal not in the rustdoc catalog must fail `cargo xtask docs signals` with a clear error.

```rust
#[test]
fn unknown_signal_in_toml_fails_validation() {
    let mut data = SignalsRustdocData::default();
    data.by_discriminant.insert("sine".into(), make_signal("sine", "Sine"));
    let mut toml = SignalsManifest::default();
    toml.signals.insert("not_a_real_signal".into(), SignalEntry::default());
    let err = validate(&data, &toml).expect_err("validation should fail");
    assert!(err.to_string().contains("not_a_real_signal"));
    assert!(err.to_string().contains("does not exist"));
}
```

**Test 3 — Core 12 is a strict subset of the catalog.** Every name in `[core_12].order` resolves to a real signal.

```rust
#[test]
fn core_12_is_subset_of_catalog() {
    let data = extract().expect("extraction failed");
    let toml = parse().expect("parse failed");
    for name in &toml.core_12.order {
        assert!(data.by_discriminant.contains_key(name),
            "Core 12 names {name} which is not in the autogen catalog");
    }
    assert_eq!(toml.core_12.order.len(), 12, "Core 12 must be exactly 12");
}
```

### Per-phase test commands

```bash
# Phase 1 — TOML parses
cargo run -p xtask -- docs signals-validate || cargo xtask docs signals-validate

# Phase 2 — extractor + parser tests
cargo test -p xtask docs::extract_signals
cargo test -p xtask docs::parse_signals
cargo test -p xtask docs::validate_signals

# Phase 3 — generator produces non-empty markdown
cargo xtask docs signals
test -s docs/generated/SIGNALS_REFERENCE.md

# Phase 4 — orchestration integration
cargo xtask docs generate
cargo xtask docs check          # passes immediately after generate

# Phase 5 — workspace clean
cargo build --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

## Acceptance criteria

- [ ] `docs/templates/signals.toml` exists with `[meta]`, `[core_12].order = [...]` (twelve entries, each a real SignalSpec discriminant), and editorial entries for each Core 12 signal (`use_cases` + `recipe_hint`).
- [ ] `xtask/src/docs/extract_signals_rustdoc.rs` walks every `Signal`-impl `pub struct` in `mixed-signals/src/{generators,envelopes,physics,composition,noise,random,processing}`, harvests rustdoc + per-field docs, and excludes test fixtures.
- [ ] `xtask/src/docs/parse_signals_toml.rs`, `merge_signals.rs`, `validate_signals.rs`, and `gen_signals_markdown.rs` exist and form a parallel pipeline to the existing capabilities pipeline.
- [ ] `cargo xtask docs signals` produces `docs/generated/SIGNALS_REFERENCE.md` with the structure described in §"Output structure" — autogen banner + DESC + VERS, Table of Contents, **"Core 12 — start here" section at the top**, then per-family sections (Oscillators, Spatial, Envelopes, Physics, Noise, Composition, Processing).
- [ ] Each per-signal entry includes name, type signature, rustdoc summary, parameter table (field / type / default / description), and a JSON example built from SignalSpec serde defaults.
- [ ] `cargo xtask docs generate` includes the new step; `cargo xtask docs check` validates SIGNALS_REFERENCE.md freshness.
- [ ] `validate_signals` rejects unknown signal names in `signals.toml` with a clear error message (test 2 above).
- [ ] `validate_signals` rejects a Core 12 list whose entries are not in the autogen catalog (test 3 above).
- [ ] **Catalog completeness** — every `Signal`-impl `pub struct` in mixed-signals (excluding test fixtures and trait blanket impls) appears in `docs/generated/SIGNALS_REFERENCE.md` with at least a non-empty summary line (test 1 above).
- [ ] **Rustdoc audit complete** — the Core 12 source files in mixed-signals each carry a struct-level summary and per-field docs sufficient for the autogen output. Gaps surfaced as a sibling `mixed-signals-rustdoc-audit` packet for the user to triage; not a blocker for landing α/β.
- [ ] **Clean build** — `cargo build --workspace` succeeds with zero new warnings.
- [ ] **Clean clippy** — `cargo clippy --workspace --all-targets -- -D warnings` clean.
- [ ] **No `#[allow]` suppressions added.** No inert TOML fields. Every field declared in `parse_signals_toml.rs` is read by `merge_signals.rs`.
- [ ] **xtask metadata + CLOG bumped** on every file touched per `feedback_metadata_headers.md`.
- [ ] **Rustdoc improved on every public item touched** in the new xtask modules per `feedback_rustdoc_when_editing.md`.
- [ ] **Q1, Q2, Q3 decisions recorded** in the new xtask modules' CLOG entries (Q1: extend-existing-extractor=NO, parallel-pipeline=YES; Q2: only-overrides=YES; Q3: toml-driven Core 12=YES).
- [ ] **Phase γ unaffected.** This packet does not touch `tui-vfx-recipes` source. Phase γ remains independently landable.

## Verification commands

```bash
# Build clean across the workspace.
cargo build --workspace

# Run the new pipeline standalone.
cargo xtask docs signals

# Run the integrated pipeline (must include SIGNALS_REFERENCE.md).
cargo xtask docs generate

# Freshness check (must pass immediately after generate).
cargo xtask docs check

# Validation gates.
cargo xtask docs signals-validate

# xtask self-tests.
cargo test -p xtask

# Workspace tests + clippy.
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo doc --no-deps

# Confirm the output file exists and is non-empty.
test -s docs/generated/SIGNALS_REFERENCE.md && wc -l docs/generated/SIGNALS_REFERENCE.md

# Confirm the Core 12 section is at the top.
head -50 docs/generated/SIGNALS_REFERENCE.md | grep -i "core 12"

# Spot-check the Sine entry.
grep -A 20 "^### sine" docs/generated/SIGNALS_REFERENCE.md
```

## Rollback plan

The packet is structured as five interim commits (one per phase). If any phase reveals a deal-breaker:

1. Stop. Do not continue to the next phase.
2. `git revert <phase-commit-hash>` to back out the most recent phase. Earlier phases stay landed (they are additive — Phase 1 just adds a TOML, Phases 2–4 add new xtask modules + wiring, Phase 5 adds the generated doc).
3. If the deal-breaker is in the extractor (Phase 2), `git revert` Phase 2 too. Move the new xtask source files to `recyclebin/xtask/src/docs/` per the recyclebin protocol.
4. `cargo build --workspace` to confirm the restored state compiles.
5. The existing `cargo xtask docs generate` continues to work in the restored state — α/β are purely additive to the pipeline.
6. File a finding capturing what blocked the consolidation, then surface to the user. Common blockers to anticipate: (a) the doc-comment line parser in the extractor mishandles multi-line doc comments or hits a non-standard `///` shape; (b) the SignalSpec discriminant ↔ struct-name mapping has more edge cases than anticipated (e.g. variants with no struct backing); (c) a Core 12 candidate's rustdoc is too thin and the autogen output is below the cheatsheet quality bar — in which case the rustdoc-audit sibling packet becomes a hard dependency.

## Risks & gotchas

- **The Spring/physics asymmetry is real and will show up in the autogen.** Physics primitives (`DampedSpring`, `BouncingDrop`, etc.) are NOT reachable through `SignalSpec` JSON deserialization today (proposal §1.4 #1 — the parallel channel). Phase γ collapses this. Until γ lands, the signals reference must either: (a) document physics primitives separately as "reachable via the parallel effect-spec channel, JSON shape varies by effect" with a forward-reference to phase γ; (b) omit physics from the autogen entirely. **Recommended (a)** — the cheatsheet's value comes from being comprehensive; documenting the asymmetry is more useful than hiding it. Surface this to the user during Phase 1 audit if it's not already settled.

- **Mixed-signals' rustdoc may have inconsistent quality across the ~50 primitives.** The Core 12 spot-check found `Sine`, `DampedSpring`, and `PerlinNoise` in good shape. The audit step in Phase 1 walks every Core 12 source and flags gaps. Per `feedback_rustdoc_when_editing.md`, gaps in upstream rustdoc are normally addressed in the same change that touches the file — but this packet is doc-pipeline-only and does not edit mixed-signals source. **Flag identified gaps as a sibling packet `mixed-signals-rustdoc-audit`; do not block α/β.** The autogen will still produce a less-rich entry for under-documented signals, which is itself a useful surface for the audit.

- **Walking source files for `///` doc comments is brittle compared to parsing rustdoc JSON.** The simple line-parser approach in Strategy A handles the common case (struct above field above field with `///` lines preceding) but breaks on: (a) `#[doc = "..."]` attributes; (b) doc comments split across `cfg`-gated blocks; (c) re-exported types from sub-modules. Mitigation: the test-1 (catalog completeness) gate catches "primitive missed entirely"; manual review of the first generated SIGNALS_REFERENCE.md catches "primitive present but with garbled docs". If the line-parser proves too brittle, escalate to Strategy B (`cargo doc --output-format json`) — that's a Phase 2 deal-breaker per the rollback plan, and the rollback unwinds to Phase 1 cleanly.

- **`SignalSpec` does not include every `Signal`-impl struct.** SignalSpec is the recipe-author surface; it deliberately excludes physics (the parallel channel) and excludes internal/wrapper Signal impls (`PositiveSignal`, `OverflowSignal`, etc.). The extractor must distinguish the **author-facing catalog** (what to put in SIGNALS_REFERENCE.md) from the raw `impl Signal for X` set. The §Current-state audit's "Test-only / construction-helper" exclusion list is the starting point; the validator must reject silent additions of these types into the catalog.

- **The `[core_12].order` array length is structurally enforced (must equal 12).** Test 3 above asserts this. If a future editor wants to expand or shrink the curated list, the test must update too — flag in the toml-file CLOG so it's discoverable.

- **No new xtask dep should be needed.** xtask already depends on `tui-vfx-core`, which depends on `mixed-signals`. The extractor reaches `mixed_signals::types::SignalSpec` through the existing edge. If for any reason the extractor needs a direct `mixed-signals` dep (e.g. to build SignalSpec instances for the JSON-example generation), add it as `mixed-signals = { workspace = true }` in `xtask/Cargo.toml` — verify the workspace declaration exists at `tui-vfx/Cargo.toml:51` (`mixed-signals = { path = "../mixed-signals", version = "0.3.0" }`).

- **`docs::check()` must compare rendered output, not regenerate-and-diff.** The existing pattern at `xtask/src/docs/mod.rs:93–103` reads the on-disk file and compares against `gen_*::render(&merged)`. Apply the same pattern for SIGNALS_REFERENCE.md. Otherwise CI false-positives if the renderer's output is byte-stable on the same input but the on-disk file has a stale newline.

## Sequencing note

- This packet is **independent of γ.** Phase γ (`tui_vfx_recipes::signals` module) does not depend on α/β; α/β do not depend on γ. They can land in any order.
- This packet is **independent of packet 1.2.A** (`VfxBindable<T>`). 1.2.A is a hard dependency for phase δ but unrelated to α/β.
- The autogen catalog produced by α makes phase γ's life **easier** (γ can use the catalog as the canonical list of Signal primitives to wrap in `VfxRecipeSignalSpec`), but γ is not blocked on α landing first.
- The handoff doc `docs/design/tui-vfx-2026-04-26-handoff-outstanding.md` should be updated to reflect α + β as queued and unblocked (or DONE after this packet lands) in the same commit that lands Phase 5.

<!-- <FILE>docs/design/tui-vfx-2026-04-26-packet-signal-facade-alpha-beta.md</FILE> -->
<!-- <VERS>END OF VERSION: 1.0.0</VERS> -->
