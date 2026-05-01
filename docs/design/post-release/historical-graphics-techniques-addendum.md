<!-- <FILE>docs/design/post-release/historical-graphics-techniques-addendum.md</FILE> - <DESC>Design-inspiration addendum to the dynamic-light-shadow-primitive spec capturing transferable techniques from ANSI art, the C64/Amiga demoscene, Myst, Diablo, FFVII, Mode 7, Wolfenstein, Crash Bandicoot, and the Hypercard/Director/Flash compositing lineage; covers virtual-buffer composition, camera/viewport separation, and 60fps transition primitives that bear on multiple post-release specs (weather-ambient-field, glyph-actor-procedural, braille-dotfield-toolkit, relative-motion-spatial-constraints) beyond the original light/shadow scope.</DESC> -->
<!-- <VERS>VERSION: 0.1.0-draft</VERS> -->
<!-- <WCTX>Preserve a session-derived design-inspiration brainstorm so the techniques surfaced (palette rotation, temporal dithering, Mode 7, virtual-buffer composition, named transition primitives, camera abstraction, ANSI art shading conventions) are durable and cross-referable rather than lost in conversation history; reads as supplementary inspiration for the light/shadow primitive spec and as a cross-cutting reference for sibling post-release specs.</WCTX> -->
<!-- <CLOG>0.1.0-draft: initial capture of two design-brainstorm responses — (1) ten transferable techniques mined from ANSI art, the Amiga/C64 demoscene, Myst, Diablo, FFVII, Sonic, Star Fox, Wolfenstein 3D, and Crash Bandicoot, with a ranked "where to act" close; (2) virtual-screen composition, camera/viewport separation, and 60fps transition primitives in the Hypercard/Director/Flash/After Effects lineage with concrete primitive-level recommendations. Lightly edited from chat form to doc form; technical content preserved; a short References section added.</CLOG> -->

# Historical graphics-techniques addendum

**Status: design-inspiration addendum.** Captured 2026-04-30. This document is a supplementary brainstorm captured to file from a session-level discussion of historical graphics techniques that bear on the post-release `tui-vfx` work. It is not a spec or PRD in its own right; it is a reservoir of ideas that the main `dynamic-light-shadow-primitive-spec.md` and its sibling specs (`weather-ambient-field-spec.md`, `glyph-actor-procedural-spec.md`, `braille-dotfield-toolkit-plan.md`, `relative-motion-spatial-constraints-spec.md`) can draw from.

The framing that motivates the document: **a modern terminal is a C64 with truecolor and a free CPU.** The cell-grid visual constraint that shaped C64/Amiga/SNES/early-PC graphics still shapes terminal output, but the compute, memory, and refresh-rate constraints those eras fought against are now lifted. Techniques that were *barely* feasible at 1 MHz are trivial at 4 GHz; techniques that were impossible become easy. This document mines that asymmetry.

---

## Part 1: Techniques from prior visual-computing eras

Ranked by directness of application to a TUI VFX system, leading with the most direct ancestor (which is not a game).

### 1. The ANSI art / textmode scene (BBS era through ACiD/iCE in the 90s)

This is the *direct lineage* of TUI VFX, more than any game engine. Artists worked in 80×25 with 16 colors and developed conventions that any modern TUI system would otherwise reinvent:

- **Block-character anti-aliasing**: at color boundaries, ▓ on the bright side, ░ on the dark side, ▒ in the transition row. The character density does the smoothing without any sub-pixel work.
- **Stipple-pattern intermediate colors**: alternating ▒ at sub-cell resolution gives the eye a perceived intensity between the two source colors. Combined with truecolor headroom, this produces apparent dynamic range *higher* than 24-bit because the spatial dither adds a free bit or two.
- **Character-as-texture**: artists picked specific characters as surface textures — `~∽≈` for water, `▌▐│║` for walls, `▒░` for fabric. The character choice is a *texture-selection axis* most modern TUI work ignores. Samplers should expose this as a first-class knob (`texture_glyph_set` or equivalent) per shader.

The textmod.es archive and the ACiD/iCE portfolio archives are the canonical references. A half-day reading session there is worth more than any 3D-engine deep-dive for this medium.

### 2. Amiga demoscene: color cycling, palette rotation, plasma

The whole Amiga demoscene's signature trick was *animation by index permutation, not value recomputation*:

- **Color cycling**: the original "animated water" trick. Fix the pixel indices; rotate the palette per frame. The water flows; per-pixel work is zero. TUI version: if samplers can output palette *indices* rather than direct colors, per-frame palette rotation becomes a single-pointer increment that animates an entire scene. Dramatically cheaper than per-cell color recomputation; lets many things "move" simultaneously without per-cell cost.
- **Plasma**: `palette[(sin(x) + sin(y) + sin(t)) % N]`. Cheap, dazzling, infinitely tweakable. The demoscene optimized hard for *parametric beauty* — small parameter changes produce wildly different visuals. The recipe-system equivalent (parametric variants, P0.8) is in this lineage; the demoscene principle is that parameter space should be *exposed and sampleable*, not buried in defaults.
- **Rotozoomers**: rotate + zoom a texture using only an affine transform per scanline; precompute the per-row offsets once, increment per frame.
- **Copper bars**: declarative "on scanline N, set color to X" programs, almost always cheaper and prettier than per-pixel imperative code. Modern equivalent: declarative shader expressions, which the recipe system already provides in spirit.

### 3. C64 raster-interrupt — temporal dithering as apparent color depth

The C64 changed its palette mid-scanline to display thousands of colors on a 16-color machine. Modern terminals refresh at 60–120 Hz; the apparent color depth of an animation is 24-bit × frames. A "60% red" cell can alternate full-red and gray at 30 Hz and average to 60% on the retina (this is exactly what 6-bit panels do internally as FRC). Doing it *deliberately* at the cell level lets the system fake intermediate colors that the terminal's color quantization wouldn't otherwise reach — particularly useful at color-region boundaries where ANSI-style stippling can't carry alone.

Worth a small experiment to measure how much apparent color depth picks up before terminal flicker becomes noticeable; expectation is one or two effective bits at 60 Hz, more on a 120 Hz terminal.

### 4. Myst (1993) — render-time vs. author-time split, taken to the extreme

Myst's entire engine was Hypercard; the "graphics" were ray-traced JPEGs on the CD. Real-time rendering: 0%. The transferable principle is the *split*: anything determinable at recipe-author time or scene-load time should be precomputed; runtime should be cheap composition. The braille dotfields already follow this pattern; the Quake-lightmap item in §10 of the main spec applies the same lesson to lighting.

A stronger Myst lesson: **transitions are content, not garnish**. Myst's "click to move forward" was a QuickTime morph between two pre-rendered frames, and the *trip* sold the world more than the destination did. Scene-to-scene transitions deserve first-class primitive status — not "fade in/out" as a generic filter, but morph-with-controlled-interpolation as a recipe-authorable thing.

### 5. Diablo (1996) — radial torch light as the entire atmosphere

Diablo was 2D isometric tiles; what made it feel 3D and tense was *one* aggressively-tuned positional light with strong falloff, centered on the player. Everything outside a small circle was crushed dark. That is the whole trick. The light/shadow spec already has the primitive for this; the Diablo lesson is that the payoff of one well-tuned positional light is enormous and worth shipping with a curated default that demonstrates the effect at full intensity. The spec's §10 stochastic sampler can add Diablo-style torch-flicker via per-frame jittered samples.

### 6. Final Fantasy VII (1997) — pre-rendered backgrounds + dynamic actors

Static high-quality background + dynamic cheap foreground, composited via a baked depth map. For TUI: a pre-rendered background recipe + dynamic foreground actors with per-cell elevation matching a baked depth map = the same trick. This is what the light/shadow spec enables when its §10 lightmap-bake item lands. The FFVII team shipped a whole game on this trick; it is a fair architectural target.

### 7. Sonic / Genesis parallax — multiple background layers at different scroll rates

Cheapest perceived depth there is. The v3.1 layer system already supports layered scenes; if layers can scroll at different rates per-frame, parallax falls out. Implementation cost: trivial. Visual payoff: large.

### 8. Star Fox / low-poly stylization — embrace the medium

When the hardware can't deliver polygon counts, *style* the visual to celebrate the constraint rather than fight it. The Madeira flag's braille shading already does this — it doesn't aspire to photorealism; it aspires to *braille-aesthetic excellence*. The principle generalizes: don't try to make TUI shadows look like Pixar shadows; make them look like *the best possible TUI shadows*, which is a different design goal with different success criteria.

### 9. Wolfenstein 3D (1992) — column-by-column raycasting

Probably overkill for current scope, but: if a "first-person dungeon in the terminal" recipe ever becomes interesting (text-mode roguelike-meets-Wolf3D), the technique is direct. One ray per terminal column, vertical strip per ray, distance-shaded. Reachable in a long afternoon.

### 10. Crash Bandicoot (1996) — bake everything you can

Naughty Dog precomputed vertex animation at build time and shipped pre-baked streams. Anything deterministic should be precomputed, not computed at runtime. The sampler/shader pipeline can probably benefit from a "bake mode" that turns expensive parametric computations into LUT reads when the parameters are static.

### Where to act, ranked

1. **ANSI art reference dive.** Half a day reading textmod.es and ACiD/iCE archives yields five-plus concrete shading conventions worth implementing.
2. **Palette-index outputs from samplers + palette rotation.** Biggest perf-and-expressiveness win. Argues for adding an "indexed color" mode to samplers as a v-next.
3. **Parallax on the v3.1 layer system.** Cheapest atmospheric win; probably already half-built.
4. **A Diablo-mode demo recipe.** Single positional light, hard falloff, dark periphery. Validates the light/shadow primitive at full intensity; ships as a marquee example. First recipe to write once the post-release `Light` lands.

The **temporal dithering** idea (point 3) is the most novel — nobody in the modern TUI VFX space appears to be doing it, and it is a real edge over a static-color approach.

---

## Part 2: Virtual screens, perspective, and 60fps transitions

The framing question: how do we compose elements on the screen, support rapid perspective changes, and animate transitions smoothly at 60 fps?

### A. Virtual screens / compositing — the off-screen-buffer pattern

Architecturally central idea: **every layer / scene / effect is an addressable off-screen buffer**, and the visible terminal output is just one composite step at the end. This is the FBO / render-target model from modern engines, with roots in 80s graphics systems (X11 pixmaps, Amiga blitter targets, Bill Atkinson's 1986 QuickDraw GWorlds for the Mac).

Why it matters specifically for `tui-vfx`:

- **Decoupled refresh rates.** A slow-changing background renders at 5 Hz; a fast-changing foreground at 60 Hz; the compositor merges them every frame. The shadow pass becomes its own buffer that updates only when light or elevation changes. Cost: ~free. Win: large, because most cells in most frames don't change.
- **Render-ahead.** Animations whose timeline is known can be precomputed N frames ahead into a virtual-buffer ring, then played back at exactly 60 Hz regardless of computation jitter. Crucial for transitions where any frame skip is visible.
- **Picture-in-picture / inset / minimap.** A virtual buffer can be downsampled and composited as a small inset — useful for a scene preview while editing a recipe, or a minimap of a larger world.
- **Dirty-region tracking per buffer.** Each buffer reports its dirty rectangle; the compositor unions them; only the union actually gets emitted to the terminal. This is how 80s window systems hit acceptable framerates, and is why 60 Hz on a terminal is feasible despite throughput that would never permit a full-screen redraw at that rate.

The Director / Flash / After Effects lineage is the *authoring* version of the same idea: a **stage** populated with **symbols** (reusable elements) playing on a **timeline**, with each symbol having its own nested timeline. After Effects pushed it furthest — compositions-of-compositions, with effects layered between. Whether the timeline metaphor is adopted explicitly or not, the structural lesson is: virtual-buffer composition is recursive and the recursion should be inspectable. Authors should be able to drill into a layer, scrub its timeline, and edit at any frame.

The Porter-Duff compositing operators (1984: over, in, out, atop, xor, source, dest, clear) are the canonical math; if the layer system uses them, it is already in this lineage.

### B. Perspective / camera — separate world space from viewport space

The breakthrough idea from 2D game design (NES Zelda, Genesis Sonic, every scrolling shooter): **the world is a large 2D buffer; the viewport is a window into it**. Pan, zoom, follow, parallax are all transformations of the viewport→world map. The world doesn't move; the viewport does.

Tractability matrix in a cell grid, ranked by feasibility:

- **Pan** — trivial. Integer offset; works perfectly. Lerp the offset over N frames at 60 Hz for smooth scroll.
- **Parallax** — multiple layers at different scroll rates. (Covered in Part 1, point 7.)
- **Discrete zoom (1×, 2×, 4×)** — trivial as integer downsampling/upsampling. Smooth zoom *between* discrete levels needs dither-blending two zoom levels by interpolation weight.
- **Mode 7 / per-row affine** — the SNES 1990 trick: a 2D plane displayed with a different affine transform on each scanline produces a tilted ground plane. F-Zero, Mario Kart, FFVI overworld. For TUI: each row gets its own sampling rate and offset, computed from a perspective formula. A "ground plane receding into the distance" recipe is Mode 7 directly translated to characters; cheap and atmospheric.
- **Camera follow** — viewport tracks an actor with a deadband (no movement until the actor leaves a central rectangle) or spring-damper smoothing. Mario 64's camera papers have the canonical formulations; they translate to 2D directly.
- **90° rotations** — straightforward by glyph remapping if rotation-aware glyph sets exist.
- **Arbitrary rotation** — infeasible in a cell grid in general. Workarounds: (a) glyphs designed to look the same at every angle (centered dots, blocks); (b) rotational symmetry pre-baked into multi-frame asset sets, indexed by rotation angle.
- **True 3D perspective** — out of scope; Mode 7 covers ~90% of the perceived value.

The architectural upshot: a "camera" primitive that owns `(world_offset, zoom, rotation)` and produces viewport-to-world coordinate transforms. This composes with the virtual-buffer model — each layer is rendered in world space, the camera transform is applied at composition time. Perspective changes become parameter animations on the camera, not re-renders.

### C. Transitions at 60 fps — first-class authorables

The Hypercard / Director / Flash lineage made transitions a *named primitive* with knobs (duration, easing, direction). For TUI VFX they're cheap at 60 Hz because most transitions are spatially local. Promote-worthy as recipe-typed citizens:

- **Cross-fade.** Per-cell linear interpolation between two virtual buffers. With temporal dithering (Part 1, point 3), this fades smoothly across truecolor depths the terminal can't natively express.
- **Wipe.** Time-varying mask reveals B over A. Direction is a parameter (left, right, diagonal, radial). The mask itself can be any function of (x, y, t) — gradient, ripple, polygon expansion.
- **Push.** A scrolls off, B scrolls on. At 60 Hz, a 30-frame push (500 ms) over an 80-cell viewport gives ~2.7 cells/frame — visually smooth, well within terminal throughput.
- **Iris.** Circular reveal from a focal point. Exactly what Diablo's torch light does, animated. The same primitive serves both.
- **Dissolve / scatter.** Per-cell randomized timing — each cell crosses over at a different point in the transition window. Gives a "particles dispersing" feel; cheap because it is a noise function on cell coordinates added to a global progress.
- **Morph.** Element-level interpolation: same actors in both states, positions interpolated, colors interpolated. The Director-era trick. Hardest of the bunch but the most expressive.

The 60 fps angle: terminal stdout throughput is the actual ceiling, not CPU. A naive full-screen redraw at 80×25 with full ANSI color codes is ~10–30 KB/frame, ~600 KB/sec at 60 Hz — fine for any modern terminal. The bottleneck arrives when the cell-diff explodes (every cell changing, e.g., a full-screen plasma at 60 Hz). Mitigation: the virtual-buffer / dirty-region pattern from §A. A transition only dirties cells that are actually crossing over.

**Easing curves matter more than expected.** Linear transitions feel mechanical; ease-in-out feels alive. The classic Penner easing functions (1999, originally for Flash) — `easeInOutQuad`, `easeInOutCubic`, `easeOutBack`, `easeOutElastic`, etc. — are 30 lines of code, free, and are the difference between "amateur" and "professional" feel. They should ship as a named-curve library usable anywhere a value is animated, not just in transitions.

### Connecting architectural shape

Stack the three: **virtual buffers + camera abstraction + named transitions = scene-graph-with-timeline**. That is the model Macromedia Director established in 1988, Flash refined in 1996, After Effects perfected in 2000+, and every modern motion-design tool (Procreate Dreams, Rive, Lottie, Figma's prototype timeline) inherits. There is no more proven architectural shape for "compose elements with smooth transitions and rapid perspective changes."

---

## High-leverage actions, consolidated

Across both parts, ranked:

1. **Promote virtual buffer to a named recipe primitive**, with refresh-rate metadata. Lets shadow / lighting passes update lazily while the rest of the scene runs at 60 Hz. Foundational for everything else in Part 2.
2. **ANSI art reference dive.** Half a day, five+ concrete shading conventions.
3. **Add a `camera` block to the scene type** — `(world_offset, zoom, rotation)` with binding support. Animate any of those at runtime → pan/zoom/follow free.
4. **Lift transitions out of the "filter" namespace into their own type** — `transition.crossfade`, `transition.wipe`, `transition.iris`, with `from_buffer` and `to_buffer` references. Authors compose by naming buffers and the transitions between them, not by stacking generic filters.
5. **Add Penner easing curves as a named-curve library.**
6. **Palette-index outputs from samplers + palette rotation.** Demoscene-grade animation cheapness.
7. **A Diablo-mode demo recipe.** Marquee example for the post-release `Light` primitive.
8. **Parallax on the v3.1 layer system.** If not already supported.
9. **Temporal dithering experiment.** Measure apparent color-depth gain at 60 Hz and 120 Hz.
10. **Mode 7 ground-plane recipe.** Per-row affine transform; tilted plane that recedes; one of the most atmospheric single tricks available.

---

## References

- **Porter, T., & Duff, T. (1984).** "Compositing Digital Images." *SIGGRAPH '84.* The canonical compositing-operator reference.
- **Penner, R. (2002).** *Programming Macromedia Flash MX*, ch. 7 — easing equations. Free reference implementations widely available.
- **textmod.es** — ANSI / textmode art archive; the closest thing to a canonical TUI-aesthetic reference library.
- **ACiD Productions / iCE Advertisements portfolios** — late-90s artgroup releases, the canonical textmode-era references.
- **Atkinson, B. (1985).** QuickDraw / MacPaint internals — origin of the off-screen-bitmap (GWorld) pattern.
- **Carmack, J. (1996–98).** Quake `.plan` files, esp. notes on lightmap baking and surface caching.
- **Appel, A. (1968).** "Some Techniques for Shading Machine Renderings of Solids." *AFIPS Conf. Proc.* The original shadow-ray paper.
- **Whitted, T. (1980).** "An Improved Illumination Model for Shaded Display." *CACM 23(6).* Recursive ray tracing; the geometry/shading/visibility separation.
- **Cook, R. L. et al. (1984).** "Distributed Ray Tracing." *SIGGRAPH '84.* Stochastic / area-light shadow sampling.
- **Kajiya, J. T. (1986).** "The Rendering Equation." *SIGGRAPH '86.* Path tracing.
- **Blinn, J. F. (1988).** "Me and My (Fake) Shadow." *IEEE CG&A 8(1).* Planar shadow projection.

<!-- <FILE>docs/design/post-release/historical-graphics-techniques-addendum.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.0-draft</VERS> -->
