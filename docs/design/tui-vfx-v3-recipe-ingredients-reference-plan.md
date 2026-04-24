<!-- <FILE>docs/design/tui-vfx-v3-recipe-ingredients-reference-plan.md</FILE> - <DESC>Plan and checklist skeleton for a standardized V3 recipe ingredients reference</DESC> -->
<!-- <VERS>VERSION: 0.2.0</VERS> -->
<!-- <WCTX>Keep this as an author-facing checklist/catalog skeleton until generated schema-backed ingredient entries exist.</WCTX> -->
<!-- <CLOG>0.2.0: turn the ingredients-reference plan into a practical author-facing checklist/catalog skeleton.</CLOG> -->

# V3 Recipe Ingredients Reference Plan

We do not yet have a complete ingredient encyclopedia. The information exists,
but it is scattered across the authoring guide, schema docs, generated V3 API
docs, capability catalog, debug recipes, procedural-source docs, and
implementation-specific examples.

This document is the practical skeleton for the reference we need. It gives
future hand-written and generated docs a shared entry format, a family checklist,
and a review path. It should help authors inventory the available **recipe
ingredients** before they design recipes, without pretending this page is already
the final catalog.

## 1. Goal

Create a standardized **recipe ingredients reference**: a catalog of the things
authors combine to make V3 recipes.

The reference should help humans and AI authors answer:

- What ingredients are available?
- What does each ingredient feel like?
- What is it good for?
- What should it be paired with?
- What should it not be used for?
- Which fields, bindings, clocks, tokens, sources, or assets matter?
- Where is the generated schema/API truth for exact spelling?
- Where is a working debug recipe or authoring-ladder example?
- Which creative ideas require future ingredients we do not have yet?

This is different from development tooling documentation. Validators, probes,
trace CLIs, normalized-IR dumps, and preview players are **development tools**.
Content transforms, placement choices, timing/clock treatment, `motion.route`,
easings, masks, shaders, filters, samplers, styles, bindings, procedural
sources, progress/timer treatments, shadows, border/envelope and outer-shape
treatments, icons/glyphs/symbols/emoji, terminal affordances, and I/O chains are
**recipe ingredients**.

## 2. How this pairs with generated schema docs and the authoring guide

Use three layers together:

1. **Authoring guide / theme prompt** — explains the recipe ladder, staged
   design-first workflow, defaults, validation posture, and when authors should
   move from ingredient discovery to technical translation.
2. **Ingredient reference** — explains designer-friendly meaning: what an
   ingredient looks like, feels like, communicates, pairs with, risks, and when
   it belongs in a recipe family or future-ingredient wishlist.
3. **Generated schema/API docs** — provide exact field names, accepted enum
   spellings, serde shape, generated rustdoc, and validation-relevant facts.

The ingredient reference must not duplicate generated schema docs as a hand-kept
field encyclopedia. Instead, each entry should point to generated facts for exact
spelling and use curated prose for author judgment. If generated docs and this
reference disagree on schema shape, generated schema/API docs win for field
truth; this reference should then be updated to fix the author-facing guidance.

Author workflow:

```text
1. Read the authoring guide and vocabulary page.
2. Walk this ingredient checklist in plain design language.
3. Choose combinations and write design-vision paragraphs.
4. Translate approved ideas with schema/API docs.
5. Validate with development tools and probe/preview evidence.
```

## 3. Standard ingredient entry format

Every future ingredient entry, whether generated, hand-written, or hybrid,
should use this format.

```text
## Ingredient Name

Status:
  base primitive | variant | earned-name composition |
  factory-internal convention | deferred | future-ingredient wishlist

Kind:
  content | placement | timing_clock | motion_route | easing | mask |
  shader | filter | sampler | style | binding | procedural_source |
  progress_timer | shadow | border_envelope | icon_glyph | asset |
  io_chain | host_edge_affordance | debug_validation

Canonical schema/API anchors:
  - schema field / generated API type:
  - accepted spelling / enum value:
  - generated docs link:

One-line concept:
  ...

Designer-friendly meaning:
  - what it can look like:
  - what it can feel like:
  - what it helps communicate:
  - natural UI moments:

Inputs and knobs:
  - field / token:
  - binding:
  - timing / clock / lifecycle phase:
  - source / asset:
  - scope / region / role:

Pairs well with:
  - ingredient:
    why:

Avoid when:
  - readability risk:
  - tone risk:
  - accessibility risk:
  - similarity risk inside a recipe set:

Reduced-motion / fallback notes:
  - reduced motion:
  - Unicode/text fallback:
  - plain-border or non-procedural fallback:

Theme-fit prompts:
  - What theme signal could this express?
  - What user moment does it naturally serve?
  - Is this a standalone idea or a recipe-family variant?

Reference recipes or fixtures:
  - path:
    what to inspect:

Validation notes:
  - expected validator/probe evidence:
  - known bridge/probe caveats:
  - unresolved docs/schema questions:
```

Entry rules:

- Start with designer-friendly meaning, then point at technical translation.
- Keep `motion.route` and easing separate: route is geometry of travel; easing
  is timing feel.
- Use `enter`, `dwell`, and `exit` for recipe phases.
- Use **recipe ingredients** for author-facing capabilities and **development
  tools** for validators, probes, trace CLIs, and preview players.
- Do not claim runtime support unless the generated schema/API docs, debug
  fixtures, or implementation evidence prove it.
- Mark speculative items as `future-ingredient wishlist` rather than presenting
  them as available ingredients.

## 4. Catalog skeleton and author checklist

Each catalog section should start with a chooser table:

```text
If you want...        Consider...                 Avoid...
subtle focus          focused-row shader + bind   high-amplitude shake
ambient progress      sub-cell bar + pulse        fireworks for routine work
warning prominence    edge grow + tint + icon     chaotic motion everywhere
```

Then add entries using the standard format above. Until the final encyclopedia
exists, use the checklist below to ensure authoring rounds consider the whole
palette.

### 4.1 Content ingredients

Checklist:

- source text, cards, scene text layers, authored strings
- typewriter, scramble, split-flap, odometer, marquee, redact, morph, dissolve,
  glitch-shift-style content treatments where supported
- token-substituted copy and runtime-message binding patterns
- overflow, alignment, line breaks, and text hierarchy
- character animation, color animation inside words, and glyph scenes for the
  creative-lab lane

Author questions:

- Does the message need to appear all at once, reveal, count, flip, redact, or
  settle?
- Is the content treatment carrying meaning or just delaying readability?
- Can plain text remain clear if motion is reduced?

### 4.2 Placement, envelope, and layout ingredients

Checklist:

- anchor, width, height, fullscreen, z/prominence, scene layer placement
- local nudge versus screen-traveling presentation
- `enter` / `dwell` / `exit` placement choices
- responsive grid assumptions and current-terminal sizing
- region, scope, row, column, role, and named-region targeting

Author questions:

- Where should the user look first, and why there?
- Is this a surface above content, inside content, behind content, or at an
  adapter edge?
- Does fixed size apply only to the object, not the whole terminal?

### 4.3 Timing, clock, lifecycle, and progress ingredients

Checklist:

- `enter`, `dwell`, `exit` durations and phase ownership
- looping `clock` for ambient/continuous behavior
- completed-fraction `progress` semantics (`0.0` not started, `1.0` complete)
- named alternatives for `remaining_ms`, `eta_ms`, debt, queue depth, or count
- timer metaphors: bars, scanners, sand/braille timers, numeric/odometer cues

Author questions:

- Is the timing a nudge, an ambient loop, or a stateful progress indicator?
- Can the recipe communicate the same meaning when loops are shortened or
  disabled?
- Does every binding-driven progress ingredient have a stable default?

### 4.4 `motion.route` ingredients

Checklist:

- stable routes: linear, rectilinear, wipe-like, subtle arc, local nudge,
  spring/settle where supported
- expressive routes: Bezier route, orbit, radial twist, figure-eight, hover,
  pendulum, projectile, helix/corkscrew, attractor-style paths where supported
- route fallbacks for readability and reduced motion
- separate route geometry from easing feel

Author questions:

- What spatial logic does the route express?
- Does the moment earn dramatic movement, or should it stay local?
- Are `enter` and `exit` same direction, opposite direction, or complementary
  geometry?

### 4.5 Easing ingredients

Checklist:

- linear, ease-in/out, cubic-out, overshoot, elastic, spring-like timing where
  supported
- custom cubic-bezier timing curves
- per-phase easing differences for `enter`, `dwell`, and `exit`
- reduced-motion alternatives with shorter distance or lower amplitude

Author questions:

- Should this feel mechanical, soft, crisp, weighty, playful, or urgent?
- Is overshoot serving tactility, or making routine UI feel noisy?
- Is the custom curve documented well enough for a reviewer to understand it?

### 4.6 Mask ingredients

Checklist:

- reveal geometry: wipe, radial, iris, diamond, path reveal
- segmented visibility: blinds, checkers, tiling/segment policies
- procedural visibility fields: dissolve, cellular, noise/dither, materialize
- direction, origin, shape, orientation, soft edge / hardness, path
- pairing with shaders, style fades, and procedural sources

Author questions:

- Is the mask revealing information, shaping attention, or adding texture?
- Is the geometry a theme signal or only a familiar transition?
- Does the mask preserve legibility during the phase where text matters?

### 4.7 Shader ingredients

Checklist:

- tint, glow, diffusion, glisten, border sweep, traveling band, reflect-like
  light treatments where supported
- focused-row gradients and soft-edge highlighters
- material cues: concealed light, sun patch, CRT/light spill, row falloff
- color source: explicit RGB, token/substitution, binding, generated field
- scope: content, border, role, named region, whole surface

Author questions:

- What material, light, or attention behavior does the shader provide?
- Is the shader doing the primary communication or supporting another
  ingredient?
- Does the color policy stay portable across downstream design systems?

### 4.8 Filter ingredients

Checklist:

- dim, tint-like treatments, scanlines, shade scanners, matrix rain, vignette,
  motion blur, rigid/sub-cell shake, pattern fill, interlace curtain
- progress emphasis: dot indicators, brackets, underline wipe, hover bar,
  sub-pixel bar, edge grow
- procedural texture filters: braille dust, charset noise, rain/trail effects
- motion-treatment filters with damping, amplitude, and pause semantics

Author questions:

- Is the filter changing existing content or becoming a competing visual layer?
- Does it support readability under common terminal fonts?
- Is a filter the right ingredient, or should a shader/source own the effect?

### 4.9 Sampler ingredients

Checklist:

- wave displacement: ripple, sine wave, frequency, amplitude, speed, phase
- fracture/segmented displacement: fault line, shredder, stripes, split bias
- field sampling for downstream shaders/filters
- reusable field producers for I/O chains

Author questions:

- What field is being sampled, and who consumes it?
- Does displacement improve the moment, or make the grid harder to read?
- Can amplitude be lowered for reduced motion while preserving the signal?

### 4.10 Style ingredients

Checklist:

- foreground/background fades, canvas-aware fades, color fade
- modifier changes, dimming, emphasis, typography-window effects
- style dwell modulation and role-based style treatment
- source/target fade policy and `apply_to` behavior

Author questions:

- Is the style treatment enough without geometry or motion?
- Which cells does the style apply to, and what role do they carry?
- Does the transition help phase readability?

### 4.11 Binding and token ingredients

Checklist:

- load-time tokens for copy or static configuration
- runtime bindings for selected row, progress, density, speed, visibility,
  severity, focus, ETA, asset path, and host-provided state
- defaults, ranges, narrow names, and preview-safe values
- binding declarations paired with host integration notes

Author questions:

- Which part of the recipe should respond to live host state?
- Is the binding name narrow enough to avoid semantic drift?
- Does validation/preview work with the declared default?

### 4.12 Procedural-source ingredients

Checklist:

- stock procedural sources and their determinism/fallback rules
- braille spinner, line/dot spinners, Matrix rain, flag/dotfield, waves,
  particles/fireworks where supported
- asset-backed procedural fields
- procedural params driven by bindings
- transparent-cell and compositing behavior

Author questions:

- Is the procedural source the scene content, a background, or a texture?
- Does it need assets, live bindings, or both?
- Is it deterministic enough for debugging and review?

### 4.13 Shadow ingredients

Checklist:

- card-shadow pattern using semantic layers and border roles
- shadow source role, destination role, offset, intensity, and falloff where
  supported
- subtle depth cues for modals, drawers, cards, and overlays
- plain fallback when shadow rendering is unavailable or too noisy

Author questions:

- Does the shadow clarify layering, or only add decoration?
- Is the source role explicit enough for generated docs and probes to explain?
- Does the effect remain visible on both dark and light palettes?

### 4.14 Border, envelope, and outer-shape ingredients

Checklist:

- built-in ratatui border styles
- custom single-glyph border overrides
- fully authored ASCII/Unicode borders
- border color, weight, timing, and emphasis
- fractional-cell, block, braille, and dense-glyph edge treatments
- parallelogram cards, angled banners, notched labels, pop-art bursts,
  soft-corner illusions
- plain-border fallback

Author questions:

- Is the border a meaningful ingredient or just default chrome?
- Does the outer shape improve tone, attention, or recognition?
- Is the dense-glyph craft legible in ordinary developer terminals?

### 4.15 Icon, emoji, symbol, and glyph ingredients

Checklist:

- Unicode symbols and emoji as baseline modern-terminal ingredients
- optional Nerd Font/profile variants
- icon language consistency across a recipe set
- text fallback for restrained/professional or unsupported contexts
- glyph scenes and tiny character moments for selective whimsy

Author questions:

- Does the icon improve recognition, hierarchy, or delight?
- Is it semantically distinct from other icons in the recipe family?
- Does the recipe still work without emoji or Nerd Font support?

### 4.16 I/O chain ingredients

Checklist:

- one producer and one or two visible consumers for first-pass chains
- scalar, color, vec2, and mask-bool hints where supported
- sampler emits field → shader consumes intensity
- filter emits scalar → mask or shader consumes threshold/intensity
- content writes text → filter publishes shade → shader consumes shade
- sequence-local hint visibility and duplicate-name validation

Author questions:

- What value flows through the chain, and why is that useful?
- Is the chain visible and explainable in a probe?
- Could the same result be clearer as a one-step ingredient?

### 4.17 Host-edge and terminal-affordance ingredients

Checklist:

- terminal bell, title/tab text, title/tab progress, and other adapter-owned
  edge signals where a host supports them
- opt-in secondary signal only; never the only visible feedback path
- restraint rules for high-value reminders, completion, and critical warnings
- host integration notes and fallback behavior

Author questions:

- Is the affordance supported by the adapter, or only a future idea?
- Does the visual recipe still communicate without it?
- Is this rare enough to remain meaningful?

### 4.18 Debug and validation expectations

Checklist:

- JSON parses
- V3 schema/contract validation passes
- normalized IR contains intended fields
- probe evidence shows the configured stage active
- preview matches `metadata.expected_visual`
- debug fixture uses schema-field directory vocabulary such as
  `motion_routes`, `easings`, `masks`, `samplers`, `filters`, `shaders`,
  `styles`, `content`, `scene`, or `complex`
- repeated patterns are labeled with promotion-ladder language

Author questions:

- What proves this ingredient is working?
- Is there a primitive-first debug recipe before a combination showcase?
- Did validation reveal schema wording that should be fixed in generated docs or
  this reference?

## 5. First-pass population order

Populate the catalog in an order that helps authoring runs quickly:

1. `motion.route` and easing ingredients.
2. Masks and reveal geometry.
3. Focused-row / selection shaders and soft-edge highlighters.
4. Progress, timer, and indicator ingredients.
5. Procedural sources.
6. I/O chain patterns.
7. Border/envelope and outer-shape ingredients.
8. Icon/glyph ingredients.
9. Debug/validation expectations and generated-doc anchors.

For each populated entry, link at least one generated schema/API anchor and one
working debug recipe, authoring-ladder example, or probe fixture when available.
If no fixture exists, mark that as a documentation/test gap rather than filling
with an unproven claim.

## 6. Quality bar

A good ingredient description is:

- **author-facing**, not implementation-first
- **specific about feel**, not just field names
- **honest about support level and limits**
- **paired with generated schema/API anchors**
- **paired with working examples or an explicit fixture gap**
- **consistent about canonical vocabulary**
- **useful for combination brainstorming**
- **clear about set-level similarity risks**

The reference should help an author form combinations before writing JSON, for
example:

```text
domestic timer dial + radial mask + numeric/odometer + sub-cell progress
focused row + area-rug metaphor + focused-row gradient + selected_row binding
sun patch + glisten band + concealed light + slow cubic-out settle
```

## 7. Recipe-set similarity checks

Ingredient references should help authors build a set of recipes, not only a
single recipe. A recipe set should be checked for accidental sameness:

- repeated metaphor without a reason
- same `motion.route` across unrelated events
- same icon/glyph language without semantic distinction
- same shader/filter combination with only color changes
- same anchor/timing/prominence for different attention levels
- same binding pattern where host responsiveness should differ

Consistency is desirable when it reinforces the design system. Similarity is a
problem when it shows the author did not explore the available ingredients.

Intentional variants are different. If alert/info/warning states share one
format and vary border color, icon, texture, position, or animation intensity,
document that as a **recipe family** with variants. Do not count each variant as
a distinct recipe concept unless the assignment asks for variant coverage.

## 8. Resolved defaults from staged authoring runs

Recurring questions from the Eichler/Stuttgart staged runs should be treated as
resolved authoring defaults unless a host integration explicitly overrides them:

- host-edge affordances are adapter-owned, opt-in secondary signals; never the
  only visible feedback path
- Unicode/text fallback is required; Nerd Font is an optional host/profile
  enhancement
- reduced motion must preserve meaning through lower amplitude, shorter travel,
  fewer loops, or static emphasis
- bindings need defaults and narrow names so previews and validators can run
  without live host data
- `progress` is completed fraction (`0.0` not started, `1.0` complete); use
  separate names for remaining time, debt, or queue depth
- stable route families are defaults; dramatic routes such as orbit,
  figure-eight, helix/corkscrew, attractor, and radial twist require an earned
  reason and a calmer fallback
- I/O chains should start with one producer and one or two visible consumers;
  avoid hidden timing coupling
- fractional-cell, braille, and custom-border treatments are accent craft with
  plain-border fallbacks
- assets should be replaceable declared inputs, not hardcoded demo assumptions

## 9. Future-ingredient wishlist

The creative lab should not be constrained to only what exists today. If an
author finds a compelling idea that requires missing support, record it as a
future-ingredient wishlist item:

```text
Future ingredient:
  name:
  missing primitive/math/capability:
  creative idea it unlocks:
  why existing ingredients are insufficient:
  possible recipe/API shape:
  example moment:
  generated schema/API anchor needed:
  debug fixture needed:
```

Wishlist entries are not implementation commitments. They are a discovery
surface for enriching the library.

## 10. Open questions

- Which ingredient facts can be generated directly from rustdoc/schema without
  losing the author-facing “feel” description?
- Should generated docs produce stub entries in this format, or should a
  separate generated catalog feed a hand-curated page?
- Which fields need human-written pairing, restraint, and theme-fit guidance?
- Should recipe examples link only to debug recipes, or also curated theme
  evaluation recipes after review?
- How should adapter-owned terminal affordances be represented while host
  support remains boundary-specific?

# <FILE>docs/design/tui-vfx-v3-recipe-ingredients-reference-plan.md</FILE> - <DESC>Plan and checklist skeleton for a standardized V3 recipe ingredients reference</DESC>
<!-- <VERS>END OF VERSION: 0.2.0</VERS> -->
