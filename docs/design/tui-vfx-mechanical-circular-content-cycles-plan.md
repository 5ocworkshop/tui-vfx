<!-- <FILE>docs/design/tui-vfx-mechanical-circular-content-cycles-plan.md</FILE> - <DESC>Reviewed implementation plan for shared circular mechanical content cycles powering odometer drums, Solari flap stacks, slot reels, and explicit old/new Pair transitions</DESC> -->
<!-- <VERS>VERSION: 0.7.0</VERS> -->
<!-- <WCTX>Correct two L2 framing errors after sibling Claude's clarification: (1) `{"binding": "name"}` is V3 canon, not something L2 locks; L2 just types the requires_bindings declaration block + adds the strict-contracts gate. (2) Phase 7 host-resolver decisions and rocketsplash routing are V3 scene-layer territory, orthogonal to L2.</WCTX> -->
<!-- <CLOG>0.7.0: drop the "gated on L2" framing wherever it appears; L2 doesn't gate runtime threading or asset-reference syntax. Replace Slice 6.5 with Slice 6.6 (TextTransformer ↔ ShaderRuntimeParams threading — its own gap, not L2-gated). Split Phase 7 deferral into the schema lift (image_name: String → BindableString on VfxImageSource — independently actionable) vs. the runtime end-to-end (V3 scene-layer composition territory). Remove asset-reference-syntax misframing — `{"binding": "name"}` is canon. 0.6.0: mark Slices 6.1-6.4 + Phase 7 actionable scope complete. 0.5.0: introduce Phase 6 + Phase 7 breadcrumb. 0.4.0: Phases 1-3 + 5 complete; Phase 4 deferred. 0.3.0: per-tile settle composition with cascade.</CLOG> -->

# Mechanical circular content cycles: drums, flap stacks, and reels

## Implementation status (v0.7.0)

| Phase | Scope | Status | Landing commits |
| --- | --- | --- | --- |
| 1 | Public schema vocabulary (source / route / cascade / settle / config) | **Complete** | `f0f5879` (vocabulary + plan v0.3.0), `dcc98f0` (effect_metadata header) |
| 2 | Route resolver (preset, normalize, weighted shuffle, resolve, route_between) | **Complete** | `145ff36` |
| 3 | Odometer schema attach + cycle rendering + cascade + per-tile settle | **Complete** | `c9a0182` |
| — | Intention 36 + Line 3x3 default font (canonical glyph table home) | **Complete** | `b23838e` |
| — | Intention 37 + loopback-required rule (every binding declaration is preview-playable) | **Complete** | `fe4db42` |
| 5 | Docs (CAPABILITIES_REFERENCE, capabilities.toml, V3 schema draft) + debug recipes (3 new) + font assets | **Complete** | tui-vfx `91a1c0a`; tui-vfx-recipes `50ab1c1` |
| 4 | SplitFlap schema attach + cycle rendering + Spring/spring_settle precedence | **Deferred** | Sub-plan below. Warrants a dedicated session given `cls_split_flap.rs` complexity (1642 LOC, 9-variant dispersion enum, hinge frames, rolling-flip, flicker). |
| 6 | Font as a bindable field — Slices 6.1-6.4 | **Complete (Literal + Binding authoring shapes both accepted)** | Slice 6.1 `BindableString` `e1de449`; Slices 6.2-6.3 `FontRegistry` + `font` field on Preset + runtime wiring `932ed98`; Slice 6.4 recipe migration in tui-vfx-recipes `b08dfa5`. Authors today write `font: "line-3x3"` (Literal) **or** `font: { "binding": "drum_font" }` (Binding) — both shapes parse and route through the registry. The `{"binding": ...}` reference shape is V3 canon, used everywhere step payloads carry runtime-bound parameters. |
| 6.6 | Thread `ShaderRuntimeParams` to `TextTransformer` | **Deferred (architectural)** | The remaining gap for Binding-form font references to resolve to host-supplied values: `TextTransformer::transform` doesn't currently receive `&ShaderRuntimeParams`. Until that threading lands, `cls_odometer.rs` calls `resolve_mechanical_cycle` with empty params; Binding-form references fall back to the registry default per Intention 36. **Not gated on the binding-loopback work** — it's its own architectural change touching every transformer, not just Odometer. Worth its own coordinated slice once binding-form recipes start needing live host injection. |
| —  | Binding-loopback design (`requires_bindings` declaration + strict-contracts gate) | **In flight by sibling** | `docs/design/tui-vfx-binding-loopback.md` (design v0.3.0) + `docs/design/tui-vfx-binding-loopback-implementation-plan.md`. Adds the *declaration* layer (`requires_bindings.<name>: { type, loopback }`) and the strict-contracts validator gate. Does not change the `{"binding": "..."}` reference shape (which is V3 canon) and does not touch host-side resolver concerns. When L2 commits, recipes can declare `requires_bindings.drum_font: { type: "string", default: "default_font" }` and the strict-contracts gate enforces declarations. |
| 7.schema | `image_name: String` → `BindableString` on `VfxImageSource` | **Independently actionable** | One-line schema change in `tui-vfx-recipes/src/recipe_schema/scene/cls_ra_image_source.rs` plus a thin runtime adjustment to evaluate the `BindableString` in the scene-layer composer. Independent of L2 (the `{"binding": ...}` reference shape is V3 canon) and independent of host-resolver consolidation. Lands the *authoring* surface for asset binding without committing to the runtime end-to-end. |
| 7.bytes | Asset bytes routing — `BindableString` → bytes → `RocketsplashImage::from_bytes` → blit | **Deferred (V3 scene-layer territory)** | The runtime end-to-end requires three orthogonal V3 scene-layer composition decisions: (a) the relationship between `AssetRegistry` (just landed in `tui-vfx-content/src/assets/`), `ImagePool` (existing in `tui-vfx-content/src/pool/cls_image_pool.rs`), and the verbal `AssetMap` contract referenced in docstrings — consolidate, compose, or leave parallel? (b) Whether rocketsplash decoding extends the existing `Image` source variant (dispatch by detected payload format) or adds a sibling `RocketsplashImage` variant. (c) How the scene-layer composer reconciles `aspect: VfxImageAspect` with rocketsplash's native-dimensions contract. None of these depend on the binding-loopback work. |
| 7.byte-side | `AssetRegistry` (byte-supplying side) | **Complete** | `486de1d`. Parallel to `FontRegistry`; `default_logo` sentinel routing; hosts populate, consumers resolve. Where it fits in the consolidation question above is the scene-layer composer's call. |

Phase 4 follow-up scoping notes:

- Wire mechanical 1x1 path first (per-column route through cycle, preserving legacy timing fields). Validator rejects ambiguous double-spin (`cycles + extra_rotations` both non-zero) and Spring + `spring_settle` both true.
- Multi-cell mechanical (2/4/6/8) follows in a sub-phase: per-tile routes feed adjacent route faces into the existing `split_flap_tile_frame` center-hinge helper.
- Decision still needed: when `mechanical` is set, do legacy `cascade` / `dispersion` fields still apply, or does mechanical own all timing? Current lean: mechanical wins; non-Simultaneous mechanical cascade overrides legacy dispersion.
- Tests must lock byte-equivalent legacy 1x1 snapshots for absent-mechanical recipes.

## Phase 6 sub-plan — font as a bindable field

**The problem.** The three debug recipes that landed in Phase 5 (`content_odometer_3x3_count.json`, `content_odometer_decimal_preset_carry.json`, `content_odometer_slot_reel.json`) carry their visual size as **literal multi-line glyph strings** in `mechanical.source.ordered.faces`. That bakes the player's font choice into every recipe; swapping the project default would require editing every recipe. Phase 6 lifts the font into a binding so recipes stay semantic ("count 099 → 100") and the player owns the glyph-table mapping.

**The declaration home.** The recipe envelope already carries three sibling declaration blocks at the top level — `requires_tokens` (mustache substitutions), `requires_bindings` (typed runtime-bindable values), `requires_assets` (image/font asset slots). Sibling Claude's binding-loopback L2 work types the `requires_bindings` declaration shape and adds a strict-contracts validator gate that enforces every `{"binding": "name"}` reference declares `name` in the block.

The `{"binding": "name"}` reference shape itself is **V3 canon** — already used everywhere step payloads carry runtime-bound parameters. L2 doesn't change it; it adds the *contract layer* (declaration + gate). Phase 6 consumes that canon directly: `font: { "binding": "drum_font" }` (or its `BindableString::Literal` shorthand `font: "line-3x3"`) at the call site, with `requires_bindings.drum_font` declared at the envelope once L2 lands. **Phase 6 does not introduce a parallel `bindings:` block.**

**Authoring shape:**

```json
"requires_assets": {
  "drum_font": {
    "type": "font",
    "format": "rsf",
    "canonical_path": "fonts/line-3x3.rsf",
    "description": "Font for digit faces."
  }
},
"config": {
  "content": {
    "effect": {
      "type": "odometer",
      "tile_width": 3, "tile_height": 3,
      "from_message": "099", "message": "100",
      "mechanical": {
        "source": {
          "type": "preset",
          "preset": "decimal_digits",
          "font": <reference to drum_font — exact shape per L2>
        },
        "route":   { "direction": "numeric_delta" },
        "cascade": { "type": "numeric_carry", "stagger_fraction": 0.4, "unchanged": "hold" },
        "settle":  { "type": "spring", "overshoot": 0.16, "settle_fraction": 0.2 }
      }
    }
  }
}
```

The recipe says nothing about line-3x3 in the call site; it names the asset slot. The player's `FontRegistry` knows what `canonical_path` resolves to today; tomorrow that mapping changes once and every recipe inherits.

**Loopback is required (Intention 37).** Every `requires_bindings` entry that a recipe declares for a font binding must yield an effective loopback at validation time. The literal default (`default: "default_font"` lifting to a static loopback, or an explicit `loopback: "default_font"`) routes through the `FontRegistry`'s sentinel resolver to the registered default. Production-only bindings are not a valid category — every recipe ships preview-playable. The strict-contracts gate sibling's L2 lands enforces presence.

**Forward-compat sentinel pattern.** Sibling's L2 brief reserves a v2 form `loopback: { "player_default": "default_font" }` that delegates fallback selection to the player's authority instead of hardcoding asset names in the recipe. Phase 6 recipes today carry the literal `canonical_path` form and migrate to `player_default` when v2 ships. Reserved sentinel naming convention: `default_<noun>` snake_case (`default_font`, `default_logo`, future `default_<kind>`); case-sensitive lookups make typos silent failures, so the validator near-miss check (Levenshtein over declared names + known sentinels) ships as part of L2 strict-contracts.

**Sub-slices** (each independently shippable):

| # | Slice | Scope |
| --- | --- | --- |
| 6.1 | `BindableString` type | New file `tui-vfx-style/src/models/cls_bindable_string.rs` mirroring `BindableU16`'s shape: `Literal(String) \| Binding(String)`, lenient deserialization (bare string accepted as `Literal`), `evaluate(&ShaderRuntimeParams) -> Option<&str>`, `ConfigSchema` derive. Inline tests for serde roundtrip, lenient bare-string parse, binding lookup against `ShaderRuntimeParams`. Used at the call site (e.g. `MechanicalContentSource::Preset.font`); the declaration home stays `requires_assets`. |
| 6.2 | `FontRegistry` + `FontGlyphTable` | New surface in `tui-vfx-content/src/fonts/`: `FontGlyphTable` is the glyph-table contract (the existing Line 3x3 table satisfies it); `FontRegistry` holds `name -> FontGlyphTable` with one named entry as the default per Intention 36. `resolve(name)` short-circuits the `default_font` sentinel to the registered default once the v2 player_default form lands; v1 resolves by name from the `requires_assets` entry's canonical_path. Embedded Line 3x3 registers automatically; host code can register additional `.rsf`-backed tables. |
| 6.3 | `font: Option<BindableString>` on `MechanicalContentSource::Preset` | Schema-bearing field, optional with `skip_serializing_if = Option::is_none`. When None, current behavior (1-cell digit faces). When Some, runtime calls `font.evaluate(&runtime_params)` → `FontRegistry::resolve(...)` → glyph table → expanded multi-line digit faces sized to the tile. `resolve_mechanical_cycle` gains `&dyn FontResolver` and `&ShaderRuntimeParams` parameters; existing call sites pass an empty resolver + empty params for byte-identical behavior on Phase 3 paths. |
| 6.4 | Recipe migration | Rewrite the three Phase 5 recipes using the `BindableString` literal form (`font: "line-3x3"`) at the call site. Once L2 commits the declaration block, follow-on edit adds `requires_bindings.drum_font` and switches the call site to `font: { "binding": "drum_font" }`. The literal-glyph versions move to a `recyclebin/recipes/` mirror so the visual reference stays available for diffing. |
| 6.6 | Thread `ShaderRuntimeParams` through `TextTransformer` | The remaining gap for Binding-form references to resolve to host-supplied values: `TextTransformer::transform` doesn't currently receive `&ShaderRuntimeParams`. `cls_odometer.rs` falls through to `resolve_mechanical_cycle` (back-compat entry point with empty params), so Binding-form font references resolve via the registry default. Threading params requires an architectural change touching every transformer, not just Odometer. **Not gated on the binding-loopback work** — it's a separate slice with its own scope. Add when binding-form recipes start needing live host injection. |

**Open questions Phase 6 closes only when implementation starts:**

1. Exact pipeline-side reference syntax for a `requires_assets` entry — `{"asset": "drum_font"}` (tagged), `"{{drum_font}}"` (token), bare `"drum_font"` (loader-aware), or another shape. Sibling's L2 types this; Phase 6 adopts whatever lands.
2. Should `MechanicalContentSource::Ordered` and `::Weighted` also gain a `font` field, or is `font` Preset-only? Lean: Preset-only initially (preset glyph expansion is the use case driving this); Ordered/Weighted carry literal face strings already, where the font choice is implicit in the strings the author writes.
3. Does the `FontRegistry` belong in `tui-vfx-content` or in a new `tui-vfx-fonts` crate? Per Intention 36 rule 5: `tui-vfx-content` until a second consumer arrives; promote later if/when a sibling crate genuinely needs the registry independent of content effects.

**Out of scope for Phase 6 (YAGNI).** Color, seed, locale, and easing bindings remain unbuilt — no real drivers today. The shape we're landing — `BindableString` at call sites + `requires_assets` for declarations — generalizes naturally when a real driver for another kind shows up. Phase 7 (asset binding) is the next real driver and gets its own breadcrumb sub-plan below.

## Phase 7 sub-plan — asset binding for rocketsplash images on a rect (BREADCRUMB)

**Status: deferred, breadcrumb only.** Real driver, no scheduled session yet. Captured here so the design doesn't get reinvented when the time comes.

**The real case.** A recipe wants to load a rocketsplash `.rss` image and paint it onto a rect — a logo on a splash surface, an empty-state graphic, a sprite face on a slot reel — concrete cases that exist or are imminent. The runtime is already there: `tui-vfx-content/src/sources/cls_rocketsplash_image.rs` wraps `rocketsplash_rt::Splash::from_bytes(...)` and exposes `blit_into_grid(grid, x, y)`. What's missing is a recipe surface that names an asset and a resolver that turns the name into bytes.

**Authoring shape:**

```json
"requires_bindings": {
  "splash_logo": {
    "type": "string",
    "description": "Rocketsplash .rss image painted on the splash surface.",
    "loopback": "default_logo"
  }
},
"config": {
  "scene": {
    "layers": [
      {
        "source": {
          "type": "image",
          "spec": {
            "image_name": { "binding": "splash_logo" }
          }
        },
        "layout": { "anchor": "center" }
      }
    ]
  }
}
```

The reference shape `{"binding": "splash_logo"}` is V3 canon; sibling's L2 doesn't choose it, V3 does. L2 adds the `requires_bindings.splash_logo` declaration and the strict-contracts gate. The schema lift to make `image_name: BindableString` (currently a plain `String`) is the only piece this Phase 7 sub-plan introduces on the recipe side.

**Reusing Phase 6's shape.** Same `requires_assets` declaration home. Same `BindableString` at the call site (the field consuming the resolved name). Same loopback rule (Intention 37 — every asset declaration must yield an effective loopback; canonical_path is the implicit one until L2 types it explicitly). Same lookup precedence (host > loopback > resolver default > missing). Only the resolver and the consuming source differ.

**What's new vs. Phase 6:**

1. **`AssetResolver` trait** — separate registry from `FontRegistry` since assets are bytes (loaded via `impl Read` per Intention 27: byte-source loading at all recipe boundaries) and fonts are pre-parsed glyph tables. The trait shape is parallel: `resolve(&self, name: &str) -> Option<AssetBytes>` with `default_<noun>` sentinels short-circuiting to a registered default once v2's player_default form lands.

2. **A consuming source surface** — likely a new V3 scene-layer source variant (`type: "rocketsplash_image"`) carrying the bindable `asset` field plus optional anchor/offset. The runtime resolves the reference to a name, the resolver to bytes, decodes via `RocketsplashImage::from_bytes`, and blits via the existing `blit_into_grid` path.

3. **Reserved sentinel: `default_logo`** (snake_case per Phase 6's casing rule). Other asset-role sentinels (`default_background`, `default_empty_state`, etc.) get added when they earn drivers — not pre-listed.

**What stays unchanged:**

- The `requires_assets` declaration block, the strict-contracts validator (with the Levenshtein near-miss check from Phase 6), the loopback layer's Intention-37 enforcement — all reused as-is. Asset binding is the second consumer of the binding shape, not a new mechanism.
- Recipes never carry image bytes. They carry asset names; resolvers load.
- The byte-source abstraction means filesystem, embedded, http, and wasm asset paths all work without per-environment recipe forks.

**Sequencing.** The schema lift (`image_name: String → BindableString` on `VfxImageSource`) is independently actionable today — `BindableString` already accepts `{"binding": "name"}` (Slice 6.1, commit `e1de449`). The runtime end-to-end (decoded bytes → `RocketsplashImage::from_bytes` → `blit_into_grid`) intersects with three V3 scene-layer composition decisions: (a) `AssetRegistry`/`ImagePool`/`AssetMap` consolidation; (b) rocketsplash routing — extend the `Image` variant or add a sibling variant; (c) how `aspect: VfxImageAspect` reconciles with rocketsplash's native-dimension contract. None of those depend on the binding-loopback work; they're V3 scene-layer territory and warrant a coordinated session with sibling when scheduled.

**Stub-today path.** The literal form (a recipe naming `splash_logo` in `requires_assets` and referencing it at the call site once that shape lands) works as soon as Phase 6's `BindableString` lands and the source-variant gains the `asset` field — no resolver bytes loading yet, just static recipe-to-source plumbing. Bindable form lights up when the loopback layer ships and Intention 37 is enforced.

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

    /// Per-tile settle behavior applied at target arrival. Fully wired in the phase that introduces it; no parse-but-inert allowance. Composes with `cascade` so each tile gets its own detent in its own time window.
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

### Settle semantics: per-tile, composes with cascade

`MechanicalSettleConfig::Spring { overshoot, settle_fraction }` is the
spring/detent at target arrival. It is **per-tile**, not whole-cycle:

1. Each tile's local progress is derived from cascade scheduling
   (Simultaneous, Staggered, NumericCarry, Randomized).
2. When a tile's local progress crosses `1.0 - settle_fraction`, the tile
   enters its settle phase and overshoots its final face by `overshoot`
   before recovering on the target.
3. Tiles whose local progress has not yet reached the settle threshold are
   still mid-route; they do not preview the settle phase.
4. When `cascade = Simultaneous`, every tile lands and settles in lockstep,
   so the visual result is one whole-cycle settle. This is the same code
   path as the per-tile case; it is not a separate mode.

Why per-tile: an odometer ratchets each digit into place with its own
detent click-click-click as carries propagate. A single whole-cycle settle
loses the staggered click and reads as a single soft snap, which is wrong
for digit drums. Per-tile settle composes correctly with all cascade
policies and falls back to whole-cycle settle when cascade is simultaneous.

`MechanicalSettleConfig::Ease { easing }` follows the same per-tile rule:
the easing curve is applied to each tile's last `settle_fraction` window
of local progress. `MechanicalSettleConfig::None` skips both phases.

### Cycle-level Spring vs legacy `spring_settle: bool`

`SplitFlap` already carries a per-mechanism `spring_settle: bool` field that
remaps the hinge rotation through a DampedSpring curve. The new
`mechanical.settle = Spring { .. }` is a higher-level cycle-wide knob.
When both are present:

1. **Cycle-level Spring wins.** The validator and runtime route the cycle
   through `MechanicalSettleConfig::Spring`; the legacy `spring_settle`
   bool is ignored for tiles whose cycle is owned by `mechanical`.
2. **Validator rejects the ambiguous combo.** If a recipe sets both
   `mechanical.settle = Spring { .. }` and legacy `spring_settle: true`,
   the validator returns a structured error rather than picking silently.
   Authors who want the cycle-level spring drop `spring_settle: true`.
3. **Legacy `spring_settle` keeps working when `mechanical` is absent.**
   Recipes that have not opted into mechanical cycles see no behavior
   change; their `spring_settle: true` continues to remap hinge rotation
   through DampedSpring exactly as today.

This precedence rule is documented in the Validation rules section below
and exercised by a SplitFlap test in Phase 4.

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
5. **Settle ships wired, never inert.** A phase that introduces or extends `MechanicalSettleConfig` must wire it through to actual rendering for every mechanism that exposes it. Adding the field with a "parses but does nothing" runtime is not allowed; defer the schema change to the phase where the runtime is ready instead.

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

## Phase 1 — standalone public config vocabulary

This phase adds the schema-bearing types as a standalone public surface.
It does **not** attach them to `ContentEffect::Odometer` or `::SplitFlap`
yet — that happens in Phase 3 and Phase 4 alongside the runtime that
honors the field. The no-inert rule (see `<CLOG>`) forbids landing the
field on a variant that does not yet wire it through to rendering.

Files:

- new `crates/tui-vfx-content/src/types/cls_mechanical_cycle_source.rs`
- new `crates/tui-vfx-content/src/types/cls_mechanical_cycle_route.rs`
- new `crates/tui-vfx-content/src/types/cls_mechanical_cycle_cascade.rs`
- new `crates/tui-vfx-content/src/types/cls_mechanical_cycle_config.rs`
- `crates/tui-vfx-content/src/types/mod.rs` (register + re-export)

Tasks:

1. Add schema-bearing mechanical cycle types (source, route, cascade,
   settle, top-level config). Each cluster lives in its own OFPF-sized
   file with inline `#[cfg(test)]` serde-roundtrip tests.
2. Provide `MechanicalCycleConfig::is_default` so a future
   `skip_serializing_if` predicate produces the same JSON for absent and
   explicit-Pair configs.
3. Add rustdoc for every new public type, including the precedence rules
   (cycle-level `Spring` wins over legacy `spring_settle`, settle is
   per-tile and composes with cascade, etc.).
4. Add serde tests for default, ordered, preset, weighted, and unknown-
   field rejection on every tagged enum.
5. Do **not** modify `ContentEffect`, `fnc_get_transformer`,
   `effect_metadata`, or any test that destructures Odometer/SplitFlap
   variants. Those edits belong to Phase 3 and Phase 4.

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

## Phase 3 — Odometer schema attach + cycle rendering + settle

Phase 3 lands the `mechanical: Option<MechanicalCycleConfig>` field on
`ContentEffect::Odometer` **and** the runtime that honors it, in one
phase. Cascade and per-tile settle are wired in this same phase — no
parse-but-inert allowance.

Files:

- `crates/tui-vfx-content/src/types/cls_content_effect.rs`
  (add the `mechanical` field; bump VERS/CLOG)
- `crates/tui-vfx-content/src/transformers/cls_odometer.rs`
- `crates/tui-vfx-content/src/transformers/fnc_get_transformer.rs`
  (thread the config to the transformer constructor)
- `crates/tui-vfx-content/src/mechanical/fnc_roll_cycle_window.rs`
- `crates/tui-vfx-content/src/mechanical/fnc_tile_rects.rs` if needed
- `crates/tui-vfx-content/src/mechanical/fnc_tile_progress.rs`
  (cascade scheduling: derives per-tile local progress from frame
  progress, tile index, total tile count, and `MechanicalCascadePolicy`)
- `crates/tui-vfx-content/src/mechanical/fnc_apply_settle.rs`
  (settle phase: Spring overshoot/recovery and Ease curves over the
  final `settle_fraction` window of each tile's local progress)
- `crates/tui-vfx-content/tests/transformers/test_cls_odometer.rs`
- `xtask/src/docs/effect_metadata.rs`
  (add `mechanical: None` to the Odometer sample)

Tasks:

1. Keep existing whole-grid Pair path unchanged when `mechanical` is
   absent or set to the explicit-Pair default.
2. For non-Pair source, segment source/target into tile rects.
3. Resolve a route per tile via the Phase 2 helpers.
4. Apply cascade to produce per-tile local progress.
5. Apply settle to per-tile local progress (Spring/Ease/None).
6. Sample each route with `roll_cycle_window` and blit into the output
   grid.
7. Keep `progress >= 1.0` returning `Cow::Borrowed(target)`.

Required Odometer tests:

- current pair-mode row/column/diagonal tests still pass unchanged.
- absent `mechanical` and explicit-Pair `mechanical` produce identical
  output.
- decimal `099 -> 100` routes changed digits forward.
- decimal `100 -> 099` routes changed digits reverse.
- unchanged digits hold under `NumericCarry { unchanged: Hold }`.
- `SpinAndReturn` spins an unchanged tile and still lands on target.
- `extra_rotations` increases intermediate route length and still lands
  exactly.
- bounded cycle rejects impossible reverse/forward routes.
- **Per-tile spring settle.** With `Staggered` cascade and
  `Spring { overshoot: 0.25, settle_fraction: 0.2 }`, each tile briefly
  overshoots its target face and recovers before the next tile starts
  settling. A frame-by-frame snapshot test asserts the overshoot face
  appears in the expected tile at the expected frame.
- **Cycle-level Spring composes with NumericCarry.** Each changed digit
  ratchets and clicks; unchanged digits hold without spurious settle.

## Phase 4 — SplitFlap/Solari schema attach + cycle rendering + settle precedence

Phase 4 lands the `mechanical: Option<MechanicalCycleConfig>` field on
`ContentEffect::SplitFlap` **and** the runtime that honors it. The
cycle-level Spring vs legacy `spring_settle` precedence rule is enforced
in this same phase — no parse-but-inert allowance.

Files:

- `crates/tui-vfx-content/src/types/cls_content_effect.rs`
  (add the `mechanical` field on the SplitFlap variant; bump VERS/CLOG)
- `crates/tui-vfx-content/src/transformers/cls_split_flap.rs`
- `crates/tui-vfx-content/src/transformers/fnc_get_transformer.rs`
- `crates/tui-vfx-content/tests/transformers/test_cls_split_flap_tiles.rs`
- new focused cycle tests if large enough to avoid growing
  `cls_split_flap.rs` tests further
- `xtask/src/docs/effect_metadata.rs`
  (add `mechanical: None` to the SplitFlap sample)

Tasks:

1. If `mechanical` is absent, run current code unchanged.
2. If `mechanical` is present with `1x1`, route through ordered face
   stacks while preserving existing speed/cascade/jitter/dispersion/
   hinge phases.
3. If `mechanical` is present with `2/4/6/8` tile height, build per-tile
   routes and feed adjacent route faces into the center-hinge helper;
   apply cascade and settle from `MechanicalCycleConfig` per Phase 3.
4. Reject ambiguous double-spin configs (`cycles` and
   `mechanical.route.extra_rotations` both non-zero) at validation time.
5. **Enforce cycle-level Spring vs legacy `spring_settle` precedence.**
   When `mechanical.settle = Spring { .. }` and legacy `spring_settle:
   true` are both present, the validator rejects the recipe with a
   structured error. Legacy `spring_settle` continues to remap hinge
   rotation through DampedSpring when `mechanical` is absent.
6. Preserve `from_message` grid parsing and newline handling.

Required SplitFlap tests:

- absent `mechanical` preserves existing `1x1` snapshots.
- explicit Pair in `mechanical` matches current old/new tile behavior.
- ordered alphabet cycle yields expected intermediate face sequence.
- unknown char with `missing_face: pair_fallback` preserves legacy-ish
  fallback.
- strict missing face errors in validator tests.
- multi-cell Solari route settles exactly on target with cycle-level
  spring.
- invalid tile sizes remain rejected/no-op at transformer layer and
  rejected by validator layer.
- **Spring vs `spring_settle` precedence test.** Recipe with both is
  rejected at validation; recipe with only `mechanical.settle = Spring`
  uses cycle-level spring; recipe with only legacy `spring_settle: true`
  keeps the existing DampedSpring hinge remap.

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
