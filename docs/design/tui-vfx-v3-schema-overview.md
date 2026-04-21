<!-- <FILE>docs/design/tui-vfx-v3-schema-overview.md</FILE> - <DESC>Formal narrative overview of the V3 schema: top-down tree structure, renderer philosophy, subtree guidance, and design rationale that should live outside the comment-stripped JSON schema draft.</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Companion document to tui-vfx-v3-schema-draft.json. Captures the higher-level structure, philosophy, and reasoning that should remain available after comments are stripped from the JSON specification-by-example.</WCTX> -->
<!-- <CLOG>0.1.0: initial schema overview after the dual-auditor synthesis pass. Establishes the top-down tree model, capability-family guidance, hybrid/wrapper conventions, and authoring/tooling layers.</CLOG> -->

# tui-vfx V3 Schema Overview

This document is the **narrative companion** to:

- `docs/design/tui-vfx-v3-schema-draft.json`

The JSON schema draft is the specification-by-example.
This document explains the **why** behind the structure, the intended tree shape, and the authoring philosophy that should survive even after comments are stripped from the JSON.

---

## 1. Primary design goal

The goal of V3 is **not only** to encode the full recipe corpus.

The goal is to encode it in a way that is:

- **lean**
- **maintainable**
- **discoverable**
- **composable**
- **stable enough to build against**

That means V3 should prefer:

- a **small number of structural node types**
- **deep shared substrates / renderer trees**
- **policy variants inside families**
- **hybrid templates where needed**

and avoid:

- a flat explosion of top-level effect names
- one new step kind for every family nuance
- giant repeated region lists where the real concept is a band, segment, or patch

---

## 2. The top-down model

V3 is easiest to understand as five layers.

```text
Authoring / library scale
└─ Template + variants layer
   └─ emits concrete recipes

Concrete recipe envelope
└─ identity / metadata / contracts
   └─ config

Renderer trees inside config
├─ content.effect
├─ scene.layers[*]
└─ pipeline

Pipeline tree
├─ structural nodes
│  ├─ parallel
│  └─ sequence
└─ operational leaves
   ├─ mask
   ├─ sampler
   ├─ filter
   ├─ shader
   └─ style_effect

Payload / policy space
└─ family-specific parameters, wrappers, nested policy trees
```

This is the most important conceptual move in the V3 design.

Many current recipes that appear to be separate “families” are actually better modeled as:

- one **deeper renderer tree**, plus
- many **policy variants** inside it

---

## 3. Concrete recipe tree diagram

A single recipe should generally be read like this:

```text
Recipe
├─ schema_version
├─ id / title / description / version / last_updated
├─ extends?
├─ metadata?
├─ requires_tokens?
├─ requires_bindings?
├─ requires_assets?
└─ config
   ├─ message?
   ├─ content?
   │  ├─ mode
   │  └─ effect
   │     └─ deep renderer subtree
   ├─ layout
   ├─ lifecycle
   ├─ border
   ├─ clock?
   ├─ base_style?
   ├─ regions?
   ├─ scene?
   │  └─ layers[]
   │     ├─ id
   │     ├─ role_tag
   │     ├─ source
   │     ├─ placement?
   │     ├─ surface?
   │     └─ pipeline?
   └─ pipeline
      ├─ timing
      └─ step?
         ├─ parallel
         ├─ sequence
         ├─ mask
         ├─ sampler
         ├─ filter
         ├─ shader
         └─ style_effect
```

---

## 4. Why the StepKind algebra stays small

One of the strongest results of the full migration audit was this:

> The schema should keep a **small StepKind algebra**.

Structural kinds:

- `parallel`
- `sequence`

Operational leaves:

- `mask`
- `sampler`
- `filter`
- `shader`
- `style_effect`

That’s enough.

The audit repeatedly showed that new family discoveries should **usually not** become new step kinds.
Instead they usually belong to one of these:

- a deeper family subtree inside an existing payload
- a wrapper/router pattern
- a hybrid composition template
- a documentation/governance category rather than a schema node kind

This is the main reason the V3 tree can remain elegant.

---

## 5. What should collapse into deeper subtrees

The audit strongly supports the following grouping posture.

### 5.1 Reveal / visibility

```text
Reveal / visibility
├─ reveal geometry
│  ├─ direction
│  ├─ origin
│  ├─ shape
│  ├─ edge hardness / softness
│  └─ path
├─ segmented / tiled visibility
│  ├─ blinds
│  └─ checkers-like policies
├─ procedural breakup / visibility fields
│  ├─ cellular
│  ├─ dissolve
│  └─ noise_dither
└─ hybrid reveal-field compositions
   └─ materialize-like families
```

### 5.2 Displacement / resampling

```text
Displacement / resampling
├─ wave displacement
│  ├─ ripple
│  └─ sine_wave
├─ fracture / segmented displacement
│  ├─ fault_line
│  └─ shredder
└─ CRT resampling
   ├─ crt
   └─ crt_jitter
```

### 5.3 Filter-side families

```text
Filter-side families
├─ indicator / progress emphasis
│  ├─ dot / bracket / underline / hover-bar / edge-grow
│  └─ sub-pixel / edge render modes
├─ traveling-band / sweep
│  ├─ glisten_sweep
│  ├─ kitt_scanner
│  └─ shade_scanner
├─ pattern / procedural texture
│  ├─ pattern_fill
│  ├─ interlace_curtain
│  ├─ braille_dust
│  ├─ charset_noise
│  └─ matrix_rain
├─ motion-treatment filters
│  ├─ motion_blur
│  ├─ rigid_shake
│  └─ sub_cell_shake (filter variant)
├─ field-rendering wrappers
│  └─ subcell_light family
├─ falloff / edge-treatment families
│  └─ vignette family
└─ rule-engine families
   └─ glyph_style
```

### 5.4 Style-side families

```text
Style-side families
├─ transition fades
├─ target-aware fades
├─ color transforms
├─ typography-window effects
├─ dwell modulation
├─ scoped style patches
└─ wrapper/router style nodes
   └─ spatial-as-style_effect
```

### 5.5 Content-side renderer trees

```text
Content-side renderer trees
├─ text transformations
├─ representational formatting
├─ typewriter + nested cursor subtree
│  ├─ cursor glyph policies
│  ├─ blink policies
│  ├─ grow policies
│  ├─ wake policies
│  └─ scan policies
└─ split-flap board / renderer system
   ├─ charset policies
   ├─ source/target transition policies
   ├─ physical rotation policies
   ├─ authenticity/timing policies
   └─ board / display variants
```

---

## 6. Hybrid templates and wrapper/router nodes

The audit showed that not everything is either:

- one primitive leaf, or
- one named family leaf

Some things are better modeled as **hybrid templates**.

Examples:

- wipe + fade transition templates
- materialize-style reveal + style pairings
- scene-layer flag wave sampler + shader pairings

And some things are best understood as **wrapper/router nodes**.

Examples:

- `style_effect(type = spatial, shader = ...)`
- field-rendering wrappers like `subcell_light`

These are important because they let the schema stay small **without** flattening real structure.

---

## 7. Scoped regions and compression

Large real recipes surfaced a practical truth:

> region verbosity can dominate recipe size even when structural complexity is modest.

This happened especially in:

- status-bar/powerline recipes
- staged banner recipes
- board/scrim patch recipes

So V3 now needs to think in two layers:

### 7.1 Primitive selectors

- `rows`
- `row_range`
- `columns`
- `column_range`
- `cell`
- `cells`
- `border`
- channel/content/role selectors

### 7.2 Compression / reuse aids

- `cell_run`
- `cell_runs`
- `region_ref`
- `config.regions`

This is probably only the first step.
Future large-corpus work may still want:

- segment abstractions
- glyph patch abstractions
- derived selectors

But the key design principle is already clear:

> large recipes should not have to express every semantic region as a giant raw cell list if a higher-level selector would be more truthful.

---

## 8. Template + variants sits above the concrete recipe tree

The easing-family artifact made this clear:

- some maintainability wins live **above** individual recipes
- one family template may emit many concrete recipes

That means template + variants authoring should be treated as an **authoring layer above the concrete schema**, not stuffed inside the StepKind algebra.

So the concrete recipe tree remains about one realized recipe.
The family/template layer lives above it.

---

## 9. Canonical normalized IR

The audit strongly supports this architectural stance:

- authors work with the raw ergonomic recipe syntax
- tooling should operate on a **canonical normalized IR**

That means:

- validator
- viewer
- migration equivalence checks
- later tooling

should normalize the authoring form first.

This matters because V3 intentionally permits multiple ergonomic authoring surfaces for related concepts:

- named composition vs primitive form
- wrapper/router nodes
- region refs vs inline selectors
- style-native spatial wrappers
- hybrid templates

A normalized IR lets the system keep those authoring conveniences **without** making tooling brittle.

---

## 10. What the schema should *not* over-promote

The schema should resist the temptation to over-promote concepts into new top-level kinds.

Things that should usually stay payload-internal or governance-level until there is a strong reason otherwise:

- tagged-union payload discriminators
- dual-color shader payload structure
- nested cursor policy spaces
- split-flap policy bundles
- rule-engine payload internals
- sub-cell render modes
- family-specific modulation flags

Promotion should happen when keeping something internal starts to harm:

- interoperability
- validation
- discovery
- authoring ergonomics

not just because a family is rich.

---

## 11. Important remaining gaps

At the end of this pass, the biggest still-open areas are not “we don’t know what the tree is.”
They are narrower than that.

### 11.1 Ballistic celebratory particles / fireworks

The Madeira crate’s fireworks are not faithfully representable using the currently covered existing primitives.

That suggests a real future capability family such as:

- `ballistic_fireworks`
- `celebratory_particles`

### 11.2 Further region compression

The first layer is clear (`cell_run`, `cell_runs`, `region_ref`), but bigger recipes may still want more.

### 11.3 Implementation of the normalized IR

The architectural decision is clear.
The implementation still needs to be built.

---

## 12. Build-against posture

The schema is ready to be treated as a serious build-target if consumers adopt these practical rules:

1. Keep StepKinds small.
2. Treat many families as policy variants inside deeper trees.
3. Use wrapper/router nodes instead of proliferating top-level kinds.
4. Prefer named reusable regions over repeated raw cell lists.
5. Keep template+variants above the concrete recipe tree.
6. Build validator/viewer around normalized IR, not raw syntax.

---



## 14. Reconciliation against the V2 schema

The strongest question at this stage is no longer “does the structure look elegant?”
It is “did we forget any important V2 surface?”

Current reconciliation stance:

| V2 surface | V3 stance |
|---|---|
| `message` | kept under `config.message` |
| `content` | kept, but explicitly treated as a deep renderer subtree |
| `layout` | kept under `config.layout` |
| `lifecycle` | kept under `config.lifecycle` |
| `border` | kept under `config.border` |
| `time` | normalized to `config.clock` |
| style-layer `clock` | normalized to per-step `clock` override |
| `theme` | explicit envelope-level home |
| `shadow` | explicit envelope-level home |
| `scene` | explicit first-class scene-layer home |
| `requires_primitives` | explicit contract/discovery home |
| singular `style` and plural `styles` | one normal form: tree of `style_effect` / `base_style_override` / sibling `shader` steps |
| `interaction_states` + `interaction_config` | explicit Step-level `interaction` home |
| legacy `spatial_shader` | migrate to `style_effect(type=spatial, shader=...)` or sibling `shader` |
| `text_pool`, `effect_pool`, `preset_pool`, `image_pool`, `font_pool` | treated as authoring-scale template/family/content-source machinery above the concrete recipe tree |
| `animation_type` | drop-recommended vestigial V2 surface |
| `continuous` | replaced by `phase = all` + clocked step / renderer-tree timing rather than a separate top-level mode |

The remaining intentionally unresolved V2-era pressure is no longer “where does this field go?”
It is mostly about:

- richer region compression
- future celebratory particle generators
- and implementation/tooling around normalized IR

## 15. Reconciliation against the multi-chapter V3 upgrade plan

This overview is intended to **tighten** the plan, not to contradict it.

### Still aligned with the plan's core decisions

- **Decision 1 — Unified Scope**: still core, now with stronger pressure for compression helpers and refs
- **Decision 2 — Pattern / primitive governance**: still core, but with a stronger emphasis on deep subtrees and renderer trees
- **Decision 3 — Tree authoring schema**: still core
- **Decision 4 — Naming refresh**: no contradiction; vocabulary cleanup remains selective
- **Decision 5 — Scene layers**: reinforced, especially by the code-derived Madeira Flag work
- **Decision 6 — ParamValue**: still core; wrapper form for signals is now the build-against stance
- **Decision 7 — Step output hints**: reinforced by the Madeira flag sampler→shader binding
- **Decision 8 — Canonical upstream semantic seam**: unchanged and still load-bearing

### Places where this synthesis makes the plan more explicit

- It turns **deep shared substrates / renderer trees** into the main organizing idea.
- It makes **hybrid templates** and **wrapper/router nodes** first-class design concepts.
- It treats **template + variants** as an authoring layer above the concrete recipe tree.
- It commits to the **normalized IR** as the recommended tooling-facing architecture.

### Places where this synthesis intentionally chooses a direction

- Motion-path / offscreen home: `pipeline.timing`
- No separate Timer primitive in core V3 for now
- Pools/presets above the concrete tree, not inside it
- Region compression starts with `cell_run`, `cell_runs`, and `region_ref`
- Scoped style patches stay inside `style_effect`, not a new top-level kind

These are not random deviations.
They are the explicit places where the dual-auditor pass produced enough signal to make a call.

## 13. Relationship to the JSON draft

The JSON draft remains the canonical specification-by-example.

This document is the explanation layer that should survive when comments are stripped from the JSON.

In practice, these two docs should be read together:

- use the JSON file to see the concrete field shape
- use this overview to understand why that shape is organized the way it is

<!-- <FILE>docs/design/tui-vfx-v3-schema-overview.md</FILE> - <DESC>Formal narrative overview of the V3 schema: top-down tree structure, renderer philosophy, subtree guidance, and design rationale that should live outside the comment-stripped JSON schema draft.</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
