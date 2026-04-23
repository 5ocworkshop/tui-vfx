<!-- <FILE>docs/COMPOSED_CAPABILITIES.md</FILE> - <DESC>Curated catalog of composed capabilities harvested from the tui-vfx-recipes corpus</DESC> -->
<!-- <VERS>VERSION: 0.2.0</VERS> -->
<!-- <WCTX>Full corpus audit rewrite: 630 recipes read end-to-end across debug_recipes, gt-design (restrained/mid-range/bold), gt-design-codex, experimental/subtle-light, toolkit (core/sizzle/showcase), modern_design, vfx-probe-validation, root, wargames, haiku_recipes1, sonnet_recipes1, scandi-edge, scandi-inspired, midcentury-modern, dynamic, easing, examples. Designer-eye verdicts recorded per recipe in /tmp/audit_log.md.</WCTX> -->
<!-- <CLOG>MAJOR: Full rewrite from scratch after complete corpus reading. Replaces v0.1 shortlist-based draft that mis-cited Highlighter and omitted Motion Path as a composition axis. New structure: (1) Composition axes with example citations, (2) Foundational/subtle section with verified A+B→X recipes, (3) Variant-rich option-set treasure, (4) Hero/thematic showcase pruned of weak picks, (5) Refinement opportunities with specific parameter deltas.</CLOG> -->

# Composed Capabilities

> **Scope.** This catalog describes *composed* capabilities — the effects that emerge when two or more primitives from [CAPABILITIES_REFERENCE.md](CAPABILITIES_REFERENCE.md) are wired together in a recipe. Every entry below is grounded in a concrete recipe file harvested from `tui-vfx-recipes/recipes/**/*.json`, read end-to-end during the audit. When a single primitive produces a catalog-worthy family by varying only its own parameters (no composition with another primitive), it lives in the **Option-Set Treasure** section rather than here.
>
> **How to read a citation.** `recipes/<path>.json` names the source recipe; the axes in parentheses list which primitives the recipe leans on. Paths are relative to `/usr/projects/tui-vfx-recipes/recipes/`.

---

## 1. The six composition axes

Recipes compose effects along six orthogonal axes. Reading a recipe means noticing which of these axes it exercises — the distinctive "A + B → X" patterns almost always light up three or more axes simultaneously.

| # | Axis | What varies | Representative field(s) |
|---|------|-------------|-------------------------|
| 1 | **Mask** | *Where* the effect is visible during reveal | `pipeline.mask.{enter,dwell,exit}` |
| 2 | **Sampler** | *Where each cell samples from* on the canvas (non-identity UV) | `pipeline.sampler.{enter,dwell,exit}` |
| 3 | **Filter** | *What transformation* is applied in shader space before style | `pipeline.filter.{enter,dwell,exit}` |
| 4 | **Style (region × shader × effect)** | *How each region of the notification is coloured/animated* | `pipeline.style` or `pipeline.styles[]` |
| 5 | **Motion path** | *How the notification rectangle travels* in screen space | `pipeline.{enter,exit}.motion_path` and `from`/`to` |
| 6 | **Runtime bindings** | *External signals* that drive shader/filter parameters at runtime | `{ "binding": "<name>" }` on numeric fields |

The first five are present in recipe v1; the sixth (runtime bindings) is proven in production by `recipes/examples/digital_rain_matrix_*_runtime_bound.json` and by `examples/btop_focused_row_live_list.json` (row-target selection binding).

---

## 2. Foundational / subtle / reusable section

These are the composed patterns a GT Design adopter can adopt immediately: each one is restrained enough for professional surface use, each one demonstrably works, and each one uses a non-obvious combination of primitives that a single-primitive catalog would miss.

### 2.1 Palette-as-signal triads with direction-coded motion

**Pattern.** Three sibling recipes (success / error / warning — or push / pull / conflict) share structure but encode semantic meaning through coordinated palette + arc bulge direction + easing. The *only* axes that vary across siblings are palette and arc sign; everything else (width, anchor, border style, glisten band) is held constant so the designer reads the family as one.

- `recipes/scandi-edge/success.json` — emerald, `arc.bulge: -0.2` (upward thrust), `back_out`, 420 ms enter.
- `recipes/scandi-edge/error.json` — crimson, `motion_path: linear`, `expo_out`, 220 ms, diamond-iris exit.
- `recipes/scandi-edge/warning.json` — burnt sienna, `arc.bulge: 0.15` (downward settle), `elastic_out`, 580 ms, pulse_wave at 0.8 Hz.
- `recipes/scandi-edge/git_push.json` (outward arc `-0.14`, diagonal glisten `-40°`) vs. `recipes/scandi-edge/git_pull.json` (inward arc `+0.12`, diagonal glisten `+40°`) — the arc sign encodes direction even without reading the label.

**Axes used.** Style (palette) × Motion path (arc sign) × Easing (character).

**Why it works.** The arc bulge sign gives the eye a proprioceptive cue that pairs with the palette. Identical layout, anchor, and border keep the notification family coherent so the signal is the bulge and the colour, not noise.

### 2.2 Whisper-first onboarding (offscreen margin-0 slide + fade_in both)

**Pattern.** Bottom-right toast that slides in from `margin_cells: 0` — flush with the screen edge — paired with `fade_in apply_to: "both"` so the foreground *and* background materialise together. No mask, no sampler, no shader. The notification reads as "already present" rather than "arriving."

- `recipes/whisper_toast.json` — 24×3 borderless, `cubic_out` 280 ms enter, 180 ms exit.
- `recipes/sonnet_recipes1/ambient_whisper.json` — same bottom-right slot, `hover` motion path (amplitude 0.5, frequency 2.0) during entry adds a breath.

**Axes used.** Motion path (origin/margin tuning) × Style (fade_in both). Two primitives; the combination is what makes it subtle.

**Why it works.** `margin_cells: 0` eliminates the "swoop" that full-offscreen slides produce. `fade_in both` prevents the background from popping in before the text finishes settling — a common failure mode in toasts.

### 2.3 Multi-region per-concern styling (the codex flagship)

**Pattern.** A single recipe defines three or four `styles[]` entries, each targeting a different region (`BorderOnly`, `TextOnly`, `BackgroundOnly`, or row-ranges), and each applies a different shader/effect to that region. The notification becomes a layered visual composition rather than a monochrome surface.

- `recipes/stress_test_multilayer.json` — `TextOnly` gets `glitch_lines` with `italic_on_flash: true`; `BorderOnly` gets a 45° `glisten_band`; `Rows: [1]` gets a `highlighter` swipe; `BackgroundOnly` gets a `barber_pole`. Four regions, four shaders, one notification.
- `recipes/gt-design-codex/*.json` family — codex recipes use three-region styling (border phosphor, text crest, background dwell) as their flagship pattern.

**Axes used.** Style (multi-region × shader). The composition is *within* the style axis — not across axes — which is why the single-primitive catalog can't describe it.

**Why it works.** A designer can now answer "draw attention to the top row only" or "give the border its own ambient colour" without fighting a global shader. Region targeting turns Style from a single layer into a stack.

### 2.4 Combined-mask AND/OR reveals

**Pattern.** `pipeline.mask.enter` is an **array** with `combine_mode: "all"` (intersection) or `"any"` (union). The reveal region is the logical combination of two masks, producing silhouettes a single mask cannot express.

- `recipes/debug_recipes/masks/*combine*.json` — exercised in debug corpus; see `mask_combine_all_blinds_wipe.json` and `mask_combine_any_*.json`.
- `recipes/recreated_logo.json` — vertical blinds (count 12) enter mask combined with a soft left-to-right wipe exit produces the signature N-logo unfold.

**Axes used.** Mask (×2 with combinator). The combinator itself is the composition.

**Why it works.** Masks are declarative; combining them is far cheaper than authoring a bespoke mask, and the vocabulary composes (blinds ∩ iris, dissolve ∪ wipe).

### 2.5 Motion Path as the sixth axis — physics as a primitive

**Pattern.** The nine `motion_path` types — `linear`, `rectilinear`, `hover`, `spring`, `bounce`, `friction`, `orbit`, `pendulum`, `projectile`, `spiral` — are not just "how long the slide takes." Each carries physics parameters (stiffness, damping, bounces/decay, amplitude/frequency, rotations) that change the *character* of arrival, independent of easing.

- `recipes/slingshot_elastic.json` — `motion_path: spring` with `stiffness: 300, damping: 15` + `back_in` exit on a `-1.2` bulge arc with stochastic snapping (seed 1337). Spring entry, parabolic exit.
- `recipes/slide_in_bounce.json` — `motion_path: bounce, bounces: 3, decay: 2.0`.
- `recipes/physics_pendulum_swing.json`, `physics_orbit_spinner.json`, `physics_projectile_toss.json`, `physics_friction_slide.json` — each isolates one physics type with minimal surrounding effects so the signature is unambiguous.
- `recipes/spring_disclosure.json` — `rect_scale_spring` pipeline-level transform (`origin: bottom_right`, `stiffness 15.0, damping 0.7`) combined with a `dissolve` mask gives a disclosure panel that springs open from its corner — far richer than scale alone.

**Axes used.** Motion path (physics parameters) × Rect scale (geometric origin) × Mask.

**Why it works.** Easing is one-dimensional (ease curve over [0,1]). Motion paths are two- or three-dimensional and carry intent: a hover indicator should breathe with `hover`, an alert should arrive with `spring`, a disclosure should open with `rect_scale_spring`. Treating them as a primitive axis unlocks a large design vocabulary that is otherwise buried inside "slide."

### 2.6 Offscreen-origin direction as a navigational cue

**Pattern.** Sibling notifications change only their `from.direction` (`from_top` / `from_bottom` / `from_left` / `from_right` / `from_top_right` etc.) to imply *where* the subject lives in the app. A push notification coming from `from_top_right` signals "system / status"; one from `from_bottom` signals "toast."

- `recipes/easing/easing_family.json` (variants block) — each of the 26 ease curves authors a different origin direction so a designer can read the family in a grid and see "this is a top-left feel, this is a bottom-center feel."
- `recipes/haiku_recipes1/success_pop.json` (top_center, back_out) vs. `recipes/haiku_recipes1/stacked_compact.json` (bottom_right, back_out) — same easing, different origin implies different affordance.

**Axes used.** Motion path (origin) × Anchor (layout). Layout semantics become part of composition.

**Why it works.** Position *is* meaning in notification design. Direction-coded origin lets the palette and shape stay constant while the notification still communicates where it belongs.

### 2.7 Runtime-bound composition

**Pattern.** Shader / filter numeric fields accept `{ "binding": "<signal-name>" }` in place of a literal. At runtime, a host surface drives the signal and the effect reshapes continuously. Bindings let the *same* recipe become a spectrum of behaviours without rebuilding.

- `recipes/examples/digital_rain_matrix_modern_runtime_bound.json` — `filter.matrix_rain` takes `density: { "binding": "density" }` and `speed_multiplier: { "binding": "speed_multiplier" }`. The classic variant does the same. This is the canonical demonstration of P0 runtime binding.
- `recipes/examples/btop_focused_row_live_list.json` — focused-row region binds to a `current_index` external signal, allowing a list's selection to drive a `highlighter` row-target live.
- Bindings exist on `shader_*`-level fields too (`progress_binding`, `direction_binding`, `position_binding`, `center_x/y_binding`, `num_shakes_binding`, `damping_scale_binding`) — see the debug_recipes/complex/ binding-exercise recipes.

**Axes used.** Runtime binding × any other primitive. This is the axis that makes a recipe *reactive* rather than parameter-frozen.

**Why it works.** A design system needs effects that respond to state (hover, focus, progress) without re-authoring. Bindings give you that with the same declarative recipe surface.

### 2.8 Rect_scale + origin-aware disclosure

**Pattern.** The pipeline-level `rect_scale` transform grows/shrinks the notification rectangle from a chosen `origin` (`center`, `bottom_center`, `bottom_right`, etc.) between a `min_width`/`min_height` and the final size. Combined with a mask, it reads as a disclosure panel opening or a badge expanding.

- `recipes/spring_disclosure.json` — `rect_scale_spring` from `bottom_right` min (3, 1), dissolve enter mask, corner-anchored panel that elastically expands.
- `recipes/pill_notification.json` — min (pill) → full size using `rect_scale` with `origin: center_top`.

**Axes used.** Rect_scale (geometric origin + springiness) × Mask (content reveal).

**Why it works.** A panel that expands from its anchor corner reads as "this belongs to the thing I clicked." Combined with `rect_scale_spring`, it picks up physics character too.

### 2.9 Canvas-aware fade_to_canvas exit

**Pattern.** Exit filter `fade_to_canvas` with `canvas_color_binding` tells the effect what the underlying terminal canvas colour is, so the notification's fade terminates at the actual background rather than a hard-coded default. The background "dissolves into" the app.

- `recipes/debug_recipes/filters/filter_fade_to_canvas_*.json` — exercises the canvas-color binding explicitly. Provides the example of how a host app wires its own background into the exit.

**Axes used.** Filter (fade_to_canvas) × Runtime binding (canvas color).

**Why it works.** Terminal notifications frequently over-darken or flash on exit because the fade target mismatches the terminal background. Canvas-awareness fixes the entire class of exit artefacts.

### 2.10 Typewriter + canvas + dim-bg triptych

**Pattern.** `content.effect.typewriter` with `speed_variance: 0.1–0.2` combined with `filter.dim { factor: 0.25–0.35, apply_to: "background" }` and a `mask.wipe` or `mask.blinds` for the reveal. The text types out, the background fades lower than the text, and the mask shapes the arrival.

- `recipes/retro_terminal.json` — typewriter (variance 0.15) + blinds horizontal × 10 + dim bg 0.35.
- `recipes/scanner_wipe.json` — typewriter (variance 0.1) + wipe ltr soft + dim bg 0.25.
- `recipes/system_boot.json` — typewriter (variance 0.2) + wipe ttb soft + CRT sampler (scanline 0.35, jitter 0.12).

**Axes used.** Content (typewriter) × Filter (dim bg) × Mask (wipe/blinds) × Sampler (optional CRT).

**Why it works.** Dimming only the background keeps the text fully saturated while the structural border recedes — the notification reads as a *terminal surface* rather than a block of colour. Adding CRT sampler dwell gives the last percent of period-authenticity.

### 2.11 Border-only glisten with title integration

**Pattern.** A `BorderOnly` region gets a `glisten_band` shader (speed 0.35–1.0, band_width 3–6), while `TextOnly` gets a flat palette. The chrome animates; the content stays calm. Adding a `border.title` with `title_position` (top/bottom/left/right) + `title_alignment` turns the border into a meaningful device.

- `recipes/midcentury-modern/success.json` — `✦ SUCCESS` title top-center with a 5-wide glisten on the border.
- `recipes/midcentury-modern/git_pull.json` and `git_push.json` — `← PULL` title top-left / `→ PUSH` title top-right plus a direction-coded diagonal glisten angle (+35° / −35°) on a single-side region.
- `recipes/stress_test_multilayer.json` — `BorderOnly` + 45° glisten + `BackgroundOnly` barber_pole in the *same* notification; each region has its own animation axis.

**Axes used.** Region (border_only) × Shader (glisten_band, angle-coded) × Border title placement.

**Why it works.** The border *has* to be present for structure; giving it its own animation makes that structural presence into a feature rather than a cost. The title placement turns a monolithic border into a labelled object.

### 2.12 Interaction states with accessibility guarantees (`interaction_states`)

**Pattern.** A notification declares `interaction_states` (hover, focus, active) with `interaction_config` metadata including `focus_visible_required`, `min_contrast_ratio`, and `reduce_motion_compliant`. The same recipe renders differently under mouse hover vs. keyboard focus vs. active press, and declares its compliance stance inline so a design system can verify it.

- `recipes/hll_leave_server.json` and related hover-lift family — full declaration of the three interaction states with a contrast-ratio guarantee and a reduce-motion-compliant fallback.

**Axes used.** Interaction states × accessibility metadata. This is the only axis with accessibility guarantees woven into the composition.

**Why it works.** A design system can't adopt an effect it can't audit. `interaction_config` is the bridge between a visual recipe and a spec a design review can sign off on.

### 2.13 CRT filter + CRT sampler + pulse — period-authentic terminal

**Pattern.** Stack `filter.crt` (scanline_strength, glow) with `sampler.crt_jitter` (intensity, speed_hz, decay_ms) and `sampler.crt` (curvature, jitter) — jitter adds time noise, scanlines add space noise, curvature bends the grid. Combine with `pulse` or `neon_flicker` on the foreground for phosphor warm-up.

- `recipes/wargames/wargames_discovery.json` — `crt_jitter` (0.15 / 8 Hz) enter + (0.08 / 5 Hz) dwell, `filter.crt` (scanline 0.3 / 0.25 glow), `neon_flicker` foreground (stability 0.85 → 0.92). Three primitives layered; the effect reads as a 1983 CRT.
- `recipes/wargames/wargames_defcon.json` — `crt_jitter` 0.15 / 12 Hz / 400 ms decay + `pulse_wave` horizontal magenta (8.0 wavelength).
- `recipes/wargames/enhanced_*.json` family — uses the `enhanced_crt_computer` template for scanline 0.08 / glow 0.08 on all dialogue.

**Axes used.** Filter (CRT) × Sampler (crt_jitter) × Style (neon_flicker or pulse). Three axes; each by itself is incomplete.

**Why it works.** CRT authenticity is a *composition* of spatial noise (scanlines), temporal noise (jitter), and chromatic warm-up (flicker). Any one of them alone looks synthetic.

### 2.14 Template inheritance (`extends`)

**Pattern.** A recipe declares `"extends": "themes/<template>.json"` and overrides only the fields that differ. Families that share chrome (WOPR cyan, WOPR green, human-input typing) collapse from 56 recipe-files to 10 template-files + 56 terse specialisations.

- `recipes/wargames/wargames_greetings.json` extends `themes/wopr_green.json` and overrides only `message`, `layout.height`, and `pipeline.enter.duration_ms`.
- `recipes/wargames/themes/new_wopr_fullscreen_cyan.json` — the CRT base with `typewriter` (variance 0.0 for computer steady) + `crt` (scanline 0.08 / glow 0.08) + cyan RGB (190, 240, 255). All 25 dialogue/map/sequence recipes inherit this.

**Axes used.** All of them — composition via inheritance rather than per-recipe wiring.

**Why it works.** The template *is* the composed recipe; variants are two-to-three-field deltas. Large families are authorable and reviewable in this pattern where they would be unmanageable otherwise.

### 2.15 Variants block: one template, N recipes (the P0.8 pattern)

**Pattern.** A single JSON file declares a `template` object and a `variants: []` array; each variant is merged into the template at load time to produce one concrete recipe per entry. The entire `easing/` directory has been collapsed this way from 30 files to 4.

- `recipes/easing/easing_family.json` — one template + 26 variants covering the full ease-curve catalog (BackIn/InOut/Out, Bounce×3, Bezier, Circ×3, Cubic×3, Elastic×3, Expo×3, Linear, Quad×3, Sine×3). The loader emits 26 `Recipe` instances.

**Axes used.** Variants schema (not a runtime primitive; a *loading-time* composition axis).

**Why it works.** A designer maintaining a 26-recipe family needs to diff one template, not 26 copies. This is the authoring-time equivalent of runtime bindings — compose at load rather than at render.

### 2.16 Paint-stroke accents (the Fuji / haze_lift family)

**Pattern.** A `highlighter` shader targeting a single-row region (or a narrow column range) with a warm yellow/vermillion paint color acts as a *paper accent* behind a calm text. The sweep is slow; the underline is what the eye catches.

- `recipes/gt-design-codex/haze_lift.json` — `highlighter` swipe on text-row plus a calm linear-gradient background. The composition reads as "accent stroke on washi paper."
- `recipes/search_match.json` — `region: TextOnly` + `style.spatial_shader: highlighter` with `color: yellow`. Time loops the sweep so a live search reflects its active state.

**Axes used.** Region (row-target) × Shader (highlighter) × Palette (warm accent).

**Why it works.** Highlighter produces a soft trailing paintbrush effect; row-scoping turns it into an accent that doesn't overwhelm. The v0.1 draft of this doc missed that the Fuji-family recipes *use* the highlighter — the composition is highlighter-on-row, not highlighter alone.

### 2.17 Sampler + shader — the spatial-distortion pair

**Pattern.** A sampler (`ripple`, `sine_wave`, `shredder`, `fault_line`) distorts the UV coordinates that the shader reads from. The sampler creates the *shape* of the distortion; the shader paints that shape. Neither alone expresses the composition.

- `recipes/sonnet_recipes1/temporal_rift.json` — `sampler.shredder` (stripe_width 3, odd_speed 2.0, even_speed -1.5) + `rainbow` shader + `scramble` content. Vertical strips slide at different speeds carrying rainbow cells; text scrambles underneath.
- `recipes/sonnet_recipes1/quantum_dissolve.json` — `sampler.sine_wave` (y-axis, amplitude 2.5, frequency 1.2) + `dissolve` mask (chunk 3) + `glitch` effect. Cells wobble, dissolve, glitch simultaneously.
- `recipes/security_breach.json` — `sampler.fault_line` (seed 666, intensity 1.0) + `scramble` content (binary charset) + `glitch` effect + `filter.dim` bg. A fault line splits the text; the split edges glitch; the background dims.
- `recipes/signal_organic_drift.json` — `sampler.sine_wave` y-axis with *Perlin* amplitude (scale 0.5, amplitude 2.5, octaves 3). Drifting-on-water motion from composed noise.

**Axes used.** Sampler × Shader/Filter × Content transformer. The sampler is the unlock.

**Why it works.** Samplers are the cheapest way to give a recipe kinetic energy without animating the text itself. Pairing with a shader that respects the sampled UVs (rainbow, linear_gradient, glisten_band) turns "shake" into a visual.

---

## 3. Option-set treasure — single primitives with rich variant families

These are single primitives worth surfacing to designers because their parameter spaces already cover a useful catalog. They are not *composed*, but they are what enables the composed patterns above.

### 3.1 `shader.glisten_band` — 9 profiles in the wild

Parameters: `speed` (0.3 – 2.0), `band_width` (2 – 20), `angle_deg` (0 / ±15 / ±25 / ±35 / ±40 / 45 / 90), `head` / `tail` colours, `direction` (`forward` / `ping_pong`), `blend_strength` (0.5 – 1.0). The corpus exercises nearly the full space.

- `recipes/progress_minimal_line.json` — 0.6 / 4 / 0° / white→gray (understated).
- `recipes/progress_fire_trail.json` — 1.2 / 5 / 0° / yellow→red (blazing).
- `recipes/progress_matrix_rain.json` — 1.2 / 3 / **90°** / bright-green→mid-green (vertical cascade — angle is what makes it "rain").
- `recipes/progress_knight_rider.json` — 0.6 / 6 / 0° / amber / ping_pong (KITT's scanning bar — direction is what sells it).
- `recipes/spinner_orbit.json` — 1.5 / 2 / **45°** / pale-violet (diagonal orbit glint).
- `recipes/progress_breathing_bar.json` — **0.3** / **20** / 0° / ping_pong (the whole bar shimmers slowly).
- `recipes/stress_test_multilayer.json` border — 1.25 / 6 / **45°** / amber (diagonal border sheen).
- `recipes/spinner_pulse_dot.json` — 2.0 / 2 / 0° / ping_pong (single-cell breathing).
- `recipes/scandi-edge/git_push.json` — 0.9 / 5 / **−40°** / violet (direction-coded).

### 3.2 `motion_path` — 10 physics types

- `linear` — default identity.
- `rectilinear` — L-shaped (`x_first` chooses order).
- `hover` — periodic amplitude+frequency oscillation (breath / float).
- `spring` — stiffness+damping spring dynamics (crisp arrival).
- `bounce` — bounces+decay (ball settling).
- `friction` — drag-damped slide (deceleration).
- `orbit` — revolutions+direction circular path.
- `pendulum` — amplitude+oscillations+damping swing.
- `projectile` — arc_height+gravity parabolic throw.
- `spiral` — rotations in/out rotational path.
- `arc` (with signed `bulge`) — implicit in `motion_path` handling; the sign is navigationally meaningful (§2.1).

Isolated demos live in `recipes/physics_*.json`; composed demos in `slingshot_elastic.json`, `spring_disclosure.json`, `sonic_boom.json`.

### 3.3 `content.effect.typewriter` variance × cursor

`speed_variance` ∈ {0.0, 0.08, 0.1, 0.15, 0.2, 0.35} paired with `cursor` (`character`, `blink_interval`, `show_while_typing`, `show_after_complete`) gives 16 author-useful combinations.

- Variance **0.0** + solid block cursor → computer steady output (wargames WOPR).
- Variance **0.15** + block cursor → confident human typing (wargames human input).
- Variance **0.35** + blink cursor → hesitant human input (wargames legacy human template).
- `examples/typewriter_perlin_variance.json` + `examples/typewriter_gaussian_variance.json` demonstrate distribution-driven variance; the effect is markedly different even at identical mean variance.

### 3.4 `filter.matrix_rain` — modes and runtime knobs

- `mode: "classic"` (fixed-grid, Japanese kana glyph set) — `digital_rain_matrix_classic_static.json` / `_dynamic.json`.
- Free mode with `preset: "matrix"` and per-cell speed/trail min/max — `digital_rain_matrix_modern_static.json` / `_dynamic.json`.
- Full runtime bindings on `density` and `speed_multiplier` in the `_dynamic` variants (§2.7).

### 3.5 Progress-indicator palettes (25+ distinct treatments)

All driven by `shader.glisten_band` parameter variation but authored as a gallery. Subset:

- `progress_corporate_blue.json`, `progress_executive_silver.json`, `progress_aurora.json`, `progress_heartbeat.json` (EKG), `progress_knight_rider.json`, `progress_matrix_rain.json`, `progress_fire_trail.json`, `progress_retro_crt.json` (adds CRT filter for period look), `progress_arcade_marquee.json`, `progress_braille_{gradient,horizontal,vertical,wave}.json`, `progress_cylon_{scanner,scanner_minimal,braille_horizontal,braille_vertical}.json`, `progress_pixel_cascade.json`, `progress_minimal_line.json`, `progress_breathing_bar.json`, `progress_scanner.json`, `progress_scanner_row_target.json` (demonstrates row-scoping).

### 3.6 Border glisten presets × 9

Same primitive, nine named tonalities: see `recipes/border_{breathing_glow,danger_flash,gold_luxury,neon_chase,pulse_attention,racing_green,sweep_clockwise,sweep_counter}.json` and `recipes/edge_accent.json`.

### 3.7 Scandi-edge / scandi-inspired / midcentury-modern design systems

Each directory is a self-contained 13-recipe design system covering `success`, `error`, `warning`, `info`, `connected`, `disconnected`, `deploy`, `git_push`, `git_pull`, `git_conflict`, `tests_passed`, `tests_failed`, `reminder`. They are worth studying as *three distinct treatments of the same event vocabulary*:

- **scandi-edge** — high-saturation, thick borders, signed arc bulges, glisten accents. Opinionated, sharp.
- **scandi-inspired** — muted palette, softer motion, fewer shaders. Understated.
- **midcentury-modern** — border titles (`★ DEPLOY ★`, `◉ ONLINE`, `→ PUSH`), playful shapes, atomic-age colours. Narrative.

A design team picking one of these as its baseline gets a full notification vocabulary for free.

---

## 4. Hero / thematic / easter-egg showcase

These are the high-commitment recipes — period-authentic, long-dwell, identity-bearing. They are *not* recommended for routine notification surfaces; they are what a design system keeps on hand for launches, welcomes, and moments that deserve a strong narrative.

### 4.1 Cinema / film-reference recipes

- `recipes/bsod_crash.json` — full-screen Windows BSOD reproduction; `mode: fullscreen`, sad-face ASCII art, Consolas-equivalent CRT jitter.
- `recipes/digital_rain.json` + `recipes/digital_rain_matrix_classic_static.json` + `recipes/progress_matrix_rain.json` — the Matrix in three calibrations (full-rain / fixed-classic / 1-line progress).
- `recipes/wargames/wargames_defcon.json` — DEFCON 1 red alert with `crt_jitter` 0.15 / 12 Hz and horizontal `pulse_wave` magenta. The DEFCON screen from the film.
- `recipes/showcase_netflix_style.json` — the red N logo unfold (note: uses `rainbow speed: 120.0` which strobes — see §5.1).
- `recipes/recreated_logo.json` — a tuned version of the same logo (`glisten_band` dwell instead of high-speed rainbow). This is the one to show; the "showcase" one is the dev experiment.

### 4.2 Sci-fi / atmospheric recipes

- `recipes/cyber_transmission.json` + `cyber_transmission_glitch_lines.json` + `cyber_transmission_typewriter.json` — a three-part cyber-aesthetic family.
- `recipes/portal_entry.json` — spiral-motion portal opens.
- `recipes/fireworks.json`, `recipes/paper_shred_thriller.json`, `recipes/ink_in_water.json` — particle / fluid set-pieces.
- `recipes/sonnet_recipes1/gravity_well.json` — `motion_path.spiral` + `ripple` sampler + `pulse_wave` + `tint` filter. Spacetime-distortion arrival.
- `recipes/sonnet_recipes1/ocean_depths.json` — ripple enter, sine-wave y-axis exit, tint filter deep-blue, surface-from-below atmosphere.
- `recipes/sonnet_recipes1/spectral_haunting.json` — `noise_dither` mask (bayer8) + sine-wave sampler + dim filter + pulse effect on ghost-white. Phantom materialising from static.
- `recipes/sonnet_recipes1/electromagnetic_pulse.json` — horizontal `blinds` × 12 + `glitch_lines` (intensity 0.8, speed 3.0, max_lines 8, flash_chance 0.1). EMP visualisation.
- `recipes/sonnet_recipes1/prism_refraction.json` — arc enter + sine-x sampler + diamond exit + rainbow. Light through a prism.
- `recipes/sonnet_recipes1/crystalline_formation.json` — checkers enter + diamond exit + diagonal glisten. Crystals assemble, shatter into diamonds.

### 4.3 Period / nostalgic recipes

- `recipes/retro_terminal.json`, `recipes/system_boot.json`, `recipes/task_processing.json` — CRT-lineage family.
- `recipes/departure_board.json` — Solari split-flap departure board (driven by `SplitFlap` which itself has a 15-profile option set inside `content_transformers`).
- `recipes/wargames/**` — 56 recipes inheriting 10 templates; the most complete period-authentic family in the corpus.
- `recipes/torch_flame.json` — braille-dust particles + pendulum sampler + vertical linear_gradient fire palette + charset_noise filter (6-stop braille gradient). The most composed recipe in the corpus; six primitives cooperating to produce a living flame.

### 4.4 Haiku/Sonnet theatrical set

`recipes/haiku_recipes1/*.json` and `recipes/sonnet_recipes1/*.json` are short, demonstrative compositions designed to showcase one thematic beat at a time (Aurora Cascade, Crystalline Formation, Digital Rainfall, Neural Spark, Quantum Portal, Thermal Vision, Temporal Rift, etc.). Use them as reference compositions for writing new thematic recipes; don't adopt them into a production design system without palette tuning.

---

## 5. Refinement opportunities (designer's red-pencil)

Recipes that are in the right neighbourhood but would benefit from tuning. The corpus is iterative; these are the clearest places a designer's touch would raise the floor.

### 5.1 Rainbow-at-saturation strobes

`recipes/showcase_netflix_style.json`, `recipes/custom_netflix_spectrum_bars.json` and several `sonnet_recipes1/*.json` recipes set `rainbow speed: 120.0` (or similar 50+ values). At terminal refresh rates this reads as a strobe rather than a colour cycle. Recommend `speed ≤ 2.5`; the `recreated_logo.json` palette `1.2` is the reference.

### 5.2 Barber-pole defaults

`recipes/task_processing.json` uses `barber_pole speed: 2.0, stripe_width: 2, gap_width: 3`. The stripe reads as scrolling rather than a pole; designer reference point is `stripe_width: 3, gap_width: 2, speed: 1.5` (see `stress_test_multilayer.json` `BackgroundOnly`).

### 5.3 Radar green

`recipes/data_radar.json` uses `shader_radar` with lime-green; the reference palette for a radar in a serious UI is the desaturated teal of `scandi-inspired/connected.json` (`RGB(58, 102, 105)`) paired with a narrower sweep angle.

### 5.4 Filter dim defaults

`recipes/retro_terminal.json` uses `filter.dim factor: 0.35`. For calmer surfaces `0.25` (see `scanner_wipe.json`) reads as premium; `0.35+` borders on "ghosted."

### 5.5 Shader highlighter defaults

`recipes/search_match.json` highlighter with `color: yellow` at full saturation over white text can clash. The codex Fuji-family recipes use `rgb(255, 220, 60)` warm-vermillion which reads as "paper accent" rather than "sticky note."

### 5.6 `shader_glitch_lines` intensity

Default intensity `0.7` with `flash_chance: 0.15` is near the limit of tasteful use. For ambient glitch (cyber/hacker aesthetic that doesn't fatigue), recommend `intensity: 0.3–0.5, flash_chance: 0.05`. Reference: `sonnet_recipes1/electromagnetic_pulse.json` uses `intensity: 0.8` — appropriate for a one-shot EMP, not for dwell.

### 5.7 Linear-gradient angle conventions

Several recipes (e.g. `scandi-edge/deploy.json`) use `angle_deg: 45.0` without explicit horizon anchoring. When a linear_gradient spans a wide-short notification, a 0° (pure horizontal) reads cleaner; 45° is for square panes. Recommend a policy: angle = 0° for width > 2× height, else 45°.

### 5.8 Sampler faultline intensity

`recipes/security_breach.json` sets `intensity: 1.0`. At that intensity the split fully bisects the text; `0.55` (the codex flagship setting in `signal_glitch_storm.json`) reads as "crack" rather than "fault" and is more legible.

### 5.9 Stochastic snapping seeds

Recipes using `snapping: { type: "stochastic" }` either hardcode `seed: 1337` (slingshot_elastic) or omit the seed (neon_open). Authoring convention: always supply a seed; pick seeds from a named palette (e.g. 1980 for retro_arcade, 666 for security_breach, 42 for test.*). Unseeded stochastic is flaky under test.

### 5.10 Weak hero candidates to retire (dev experiments, not gallery)

- `recipes/custom_netflix_n_bars.json` and `custom_netflix_spectrum_bars.json` — the `_custom_` prefix is author shorthand for "dev scratch;" the cleaner sibling `recreated_logo.json` supersedes them.
- `recipes/gun_barrel_reveal.json` — shape is there, but has no distinctive shader dwell; reads as a generic arc entrance.
- `recipes/test_italic.json` — single-primitive italic flash demo; useful as a debug_recipe, not for a gallery.
- `recipes/stress_test_multilayer.json` — *excellent* as a development / probe-validation target; not a production recipe (too many axes for a single notification).

---

## 6. How to extend this catalog

When adding a new recipe to `tui-vfx-recipes/recipes/`:

1. **Decide which axis (or axes) it is *about*.** A good recipe has a distinct focus along one or two axes (per §1). Diffuse recipes that touch all six equally are usually better split.
2. **If the recipe is a variant-family member**, consider authoring it into an existing `*_family.json` `variants:` block (§2.15) rather than as a standalone file.
3. **If the recipe is an inheritance-family member**, author a theme in `<family>/themes/*.json` and use `extends:` (§2.14).
4. **Mark the composition axis in the recipe description**, using the axis-name vocabulary from §1. A reviewer should be able to read the description and anticipate which primitives will be present.
5. **If the composition is reusable**, propose a new §2.x entry here with a concrete citation.

---

<!-- <FILE>docs/COMPOSED_CAPABILITIES.md</FILE> - <DESC>Curated catalog of composed capabilities harvested from the tui-vfx-recipes corpus</DESC> -->
<!-- <VERS>END OF VERSION: 0.2.0</VERS> -->
