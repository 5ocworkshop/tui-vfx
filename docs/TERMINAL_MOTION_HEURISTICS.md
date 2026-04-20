<!-- <FILE>docs/TERMINAL_MOTION_HEURISTICS.md</FILE> - <DESC>Canonical terminal-centric heuristics for motion, depth, and recipe authoring</DESC> -->
<!-- <VERS>VERSION: 1.1.0</VERS> -->
<!-- <WCTX>Sub-plan A Phase A.3 — add heuristics 17 and 18 covering shadow output role tagging (RoleTag::Shadow write-back, stage ordering) and role-filtered extrusion (ShadowConfig.source_region) for fixing the "shadow-on-text" splash bug</WCTX> -->
<!-- <CLOG>1.1.0: MINOR — add heuristics 17 (shadow output carries RoleTag::Shadow; stage ordering observable in next pass / trace / direct inspection) and 18 (role-filtered shadow extrusion via source_region + fnc_extract_shadow_envelope fixes shadow-on-text bug).
1.0.0: Initial canonical heuristics doc covering cell aspect ratio, single-glyph cells, transparent shadows, compositing order, and 60 FPS perception limits</CLOG> -->

# Terminal Motion Heuristics

This document is the canonical guide to terminal-specific quirks that matter
when designing `tui-vfx` shaders, masks, filters, motion paths, shadows, and
recipes.

Use it when:
- authoring new recipes
- designing new shaders or samplers
- tuning timings and tails
- diagnosing why an effect feels "wrong" even when the math is technically correct

If a recipe idea conflicts with these heuristics, prefer the terminal reality
over the abstract animation idea.

## Core Heuristics

1. **Horizontal motion usually reads smoother than vertical motion.**
   Terminal cells are taller than they are wide, so horizontal travel gives
   you more perceptual steps over the same distance.

2. **Vertical travel often needs extra time or weighting.**
   Short vertical segments can read like jumps instead of motion. Either make
   them longer, slow them down, or weight vertical timing more heavily.

3. **Diagonals are fragile unless softened.**
   Hard diagonals alias into stair-steps. Prefer soft masks, eased reveals,
   mixed horizontal/vertical choreography, or samplers that disguise the stair
   pattern.

4. **A terminal cell only gets one glyph.**
   You cannot put high-resolution decorative glyph detail "behind" readable
   text in the same cell. If the glyph changes, the text is gone.

5. **Put high-resolution tricks in shell-owned cells, not text cells.**
   Use borders, padding, shadows, empty margins, or direct frame content for
   braille/block/quarter-cell detail. Keep text cells legible.

6. **Corners need explicit help.**
   A 90-degree turn is easy to miss. Add a corner hold, local glow, brightness
   bump, or segment-aware tail behavior if the turn is narratively important.

7. **Tail length must match route geometry.**
   If a tail is longer than the nearby segments, the viewer sees a whole route
   glowing instead of a signal moving through it.

8. **60 FPS does not guarantee 60 perceptual states.**
   A shader can update every frame and still look coarse if the visible
   geometry only changes every few cells. Temporal smoothness does not fix poor
   spatial resolution.

9. **Dense repeated patterns shimmer under motion.**
   Braille, checker, stripe, and scanline textures can alias when moved. Thin
   them out, slow them down, or reserve them for dwell phases.

10. **Transparent shadows are one of the strongest depth tools in terminal UI.**
    Transparent or grade-underlying shadows can create depth, preserve
    underlying glyph structure, and add atmosphere without moving the widget.
    Use them for detached overlays, premium surfaces, and subtle VFX layers.

11. **Transparent shadows depend on underlay preservation.**
    If the renderer clears the destination before compositing, transparency will
    expose terminal-default black instead of the intended canvas.

12. **Ratatui buffers are snapshots, not layer stacks.**
    Composition order matters. If you want masked or transparent areas to show
    the real canvas, you must preserve and restore the underlay intentionally.

13. **Canvas color matters.**
    Many effects look fine on terminal black and wrong on an application-owned
    canvas color. Preview them against a real background, not an implicit one.

14. **Style changes are often better than motion.**
    In terminals, bold, dim, tint, pulse, glow, and shadow changes can express
    state more clearly than large positional movement.

15. **Large-area full-field effects muddy hierarchy quickly.**
    Whole-panel motion or texture can overwhelm overlays. Prefer localized
    emphasis on edges, borders, titles, progress lanes, or selected rows.

16. **Font and terminal rendering vary.**
    Braille, half-block, quarter-block, and shade glyphs do not look identical
    across WezTerm, Kitty, Alacritty, GNOME Terminal, and Windows Terminal.

17. **Shadow output carries its own semantic role.**
    Since `tui-vfx` 0.8.0, the shadow stage tags every cell it produces
    with `RoleTag::Shadow` in the destination role map. Inspection tools,
    trace consumers, and subsequent pipeline passes can therefore address
    shadow output by role rather than by position. When authoring a
    recipe that targets an element by role (e.g. `StyleRegion::Role(
    RoleTag::Shadow)`), keep in mind the stage ordering: shadow runs
    AFTER filters in the per-cell pipeline, so a filter in the SAME frame
    cannot observe the shadow role tags. They become observable in the
    next pass, in trace output, or via direct inspection of
    `destination.roles()` after rendering completes.

18. **Role-filtered shadow extrusion fixes the "shadow-on-text" bug.**
    A text card without explicit borders casts a wrong-looking shadow
    when the extrusion envelope follows individual text cells. Set
    `ShadowConfig::source_region = Some(RoleTag::Border)` (or another
    structural role) to restrict extrusion to the tight bounding box of
    role-matched source cells. The `fnc_extract_shadow_envelope` pure
    function carries out the restriction and is reusable by any stage
    that wants role-filtered geometry.

17. **Snapping changes the feel.**
    `round`, `floor`, and stochastic snapping are not neutral implementation
    details. They are part of the motion language.

18. **Recipe ideas must respect ownership of cells.**
    If an effect needs custom frame geometry, use frame content or border-owned
    cells. If it needs readable copy, let text cells remain text cells.

## Design Guidance

- For route-following effects, prefer explicit authored paths before adding
  solver-based auto-routing.
- For overlays and notifications, put drama in the shell first and the message
  body second.
- For premium depth, try transparent shadows before adding more motion.
- For retro or signal-noise effects, validate readability against real text and
  a non-black canvas.

## Recommended Workflow

1. Sketch the effect in terms of owned cells: text, border, padding, shadow,
   empty background.
2. Decide whether the effect should read as motion, texture, depth, or state.
3. Tune geometry first, then tail length, then timing, then style polish.
4. Test against a non-default canvas color and at least one real terminal.

<!-- <FILE>docs/TERMINAL_MOTION_HEURISTICS.md</FILE> - <DESC>Canonical terminal-centric heuristics for motion, depth, and recipe authoring</DESC> -->
<!-- <VERS>END OF VERSION: 1.0.0</VERS> -->
