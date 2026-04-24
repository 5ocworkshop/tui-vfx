<!-- <FILE>docs/design/post-release/glyph-actor-procedural-spec.md</FILE> - <DESC>Post-release specification for a glyph actor procedural that can render animated ASCII/Unicode stick figures as V3 recipe ingredients.</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Capture the post-release glyph actor idea as a deferred V3 procedural ingredient using existing motion, phase, binding, and effect concepts.</WCTX> -->
<!-- <CLOG>0.1.0: capture initial glyph actor procedural spec.</CLOG> -->

# Glyph actor procedural spec

**Status: Post-release project.** This is not release-blocking V3 work. Keep it
as a deferred capability until the core V3 release gate, recipe migration, and
as-built docs are stable.

## 1. Purpose

Add a small procedural actor ingredient that renders a terminal-native
ASCII/Unicode figure and drives it through authored actions:

- walk
- run
- crawl
- sit
- lie down
- jump
- dive
- wave
- perch on a rect edge
- carry or point at a small icon

The goal is not a game engine. The goal is a compact procedural sprite actor that
can participate in normal V3 composition: motion routes, bindings, phases,
styles, shadows, trails, masks, filters, and I/O chains.

Example use cases:

```text
Build complete:  actor walks in, sits on the toast border, and raises a check.
Warning:         actor runs across the bottom edge carrying ⚠.
Empty state:     actor wanders through the panel and looks around.
Upload done:     actor climbs onto the progress bar and plants a flag.
Creative mode:   actor dives from a modal edge into a ripple shader.
```

## 2. Design principles

1. **Terminal-native, not terminal-apologetic.** The low cell resolution is part
   of the charm. Make the actor read clearly at small sizes.
2. **Small vocabulary first.** Start with a few useful actions and expand only
   when recipes prove the need.
3. **Compose through V3.** The actor should be an ingredient that emits cells; it
   should not own a private animation system.
4. **Anchor precisely.** Feet, hands, head, and center anchors make interaction
   with rects and routes legible.
5. **Keep math reusable.** Generic route or timing substrate belongs in
   `mixed-signals`; glyph rendering, pose selection, and terminal-cell output
   belong in `tui-vfx` / `tui-vfx-recipes`.
6. **Earn whimsy.** The actor should help the user notice, understand, or enjoy a
   moment. It should not become noise.

## 3. Conceptual model

```text
runtime bindings ─┐
                  ├─ target resolver ─┐
motion.route ─────┤                   │
                  │                   ▼
phase clock ──────┼──────────> glyph actor procedural ──> V3 output cells
                  │                   ▲
action sequence ──┘                   │
                                      └─ styles/effects/chains
```

The actor evaluates four things per frame:

1. where it is,
2. which action is active,
3. which pose frame that action selects,
4. how the emitted cells are anchored to the route or target.

## 4. Actor cell shapes

The actor should support multiple glyph packs. Start with two.

### 4.1 ASCII-safe pack

```text
stand      walk A     walk B     sit       lie

  o          o          o          o       o__
 /|\        /|\        /|\        /|\       /|
 / \        /          / \        _/        / 
```

### 4.2 Unicode expressive pack

```text
stand      run        jump       dive

  ●          ●         \●/        \●
╱│╲        ╱│╲          │          ╲│
╱ ╲        ╱ ╲         ╱ ╲          ╲
```

The exact frames should be tuned in debug recipes. The first implementation can
ship with simple frames and improve them after visual review.

## 5. Authoring sketch

This is illustrative, not a final schema commitment.

```json
{
  "kind": "procedural",
  "procedural": "glyph_actor",
  "actor": "stickman",
  "glyph_pack": "unicode",
  "action": "walk",
  "anchor": "feet",
  "motion": {
    "route": "line",
    "from": { "x": 0.05, "y": 0.80 },
    "to": { "x": 0.75, "y": 0.80 },
    "ease": "ease_in_out"
  },
  "phase": "dwell"
}
```

Targeted perch example:

```json
{
  "kind": "procedural",
  "procedural": "glyph_actor",
  "actor": "stickman",
  "action": "sit",
  "anchor": "seat",
  "target": {
    "binding": "toast_rect",
    "edge": "top",
    "align": "right",
    "offset": { "x": -2, "y": 0 }
  }
}
```

Action sequence example:

```json
{
  "procedural": "glyph_actor",
  "actor": "stickman",
  "actions": [
    { "action": "walk", "duration_ms": 900, "route": "in_from_left" },
    { "action": "sit", "duration_ms": 1200, "target": "toast.top.right" },
    { "action": "jump", "duration_ms": 450, "route": "arc_to_bottom" }
  ]
}
```

## 6. Required capabilities

### 6.1 Pose/action library

Minimum first slice:

- `stand`
- `walk`
- `run`
- `sit`
- `jump`
- `wave`

Second slice:

- `crawl`
- `lie_down`
- `dive`
- `carry_icon`
- `point`
- `trip`

Each action needs:

- one or more pose frames,
- gait/frame timing,
- default anchor,
- optional recommended route shapes,
- debug recipe coverage.

### 6.2 Anchors

Initial anchors:

- `feet`
- `center`
- `head`
- `hands`
- `seat`
- `leading_hand`
- `trailing_hand`

Anchors let the actor attach to screen geometry without hand-tuning every pose.

```text
feet anchor on route:

        o
       /|route ─/─\────────────
```

```text
seat anchor on rect edge:

┌───────────────┐
│ toast         │
└──────────o────┘
          /|          _/
```

### 6.3 Motion route compatibility

The actor should use existing V3 motion routes before adding anything new:

- line
- arc
- Bezier
- radial/figure-eight/helix-style routes where visually appropriate

A route positions the actor anchor. Pose frames do not own global movement.

### 6.4 Target geometry

The actor needs a way to resolve a target rect. Support this in two stages.

Stage 1: authored rects.

```json
"target": {
  "rect": { "x": 10, "y": 4, "w": 30, "h": 5 },
  "edge": "top",
  "align": "center"
}
```

Stage 2: runtime-bound rects.

```json
"target": {
  "binding": "modal_rect",
  "edge": "bottom",
  "align": "right"
}
```

Runtime-bound rects are more useful for gt-design and other host applications,
but authored rects are easier to prove in the first debug recipes.

### 6.5 Effects and I/O chaining

The actor should emit normal V3 output cells so downstream ingredients can chain:

```text
glyph_actor -> soft_shadow -> highlight -> trail
```

Examples:

- dust trail behind a running actor,
- soft shadow below a jump,
- ripple shader where the actor lands,
- highlighter glow around a wave,
- mask that reveals text as the actor walks across it.

## 7. Non-goals

- No physics engine.
- No skeletal animation system.
- No collision engine.
- No behavior tree.
- No dependency on ratatui types.
- No hidden runtime command execution.
- No theme/palette coupling inside tui-vfx.

If a later design needs physics, collision, or richer animation planning, that
should be a separate proposal after the first actor proves useful.

## 8. Recipe/debug requirements

When implemented, add debug recipes under `tui-vfx-recipes` with primitive-first
coverage before complex combinations.

Required debug recipes:

1. standing actor, no motion,
2. walk across screen,
3. run across screen,
4. sit on authored rect edge,
5. jump from authored rect edge,
6. runtime-bound rect perch if bindings are available,
7. actor plus shadow,
8. actor plus trail,
9. actor triggering another effect through V3 I/O.

Each debug recipe should include:

- `metadata.expected_visual`,
- body text that labels the primitive/action clearly,
- transparent or minimal background unless the background is required,
- one simple version before any showcase version.

## 9. Validation and tooling requirements

Before promotion from post-release idea to active implementation:

- validator accepts the authored shape or reports clear diagnostics,
- normalized IR emits explicit actor/action/anchor/target data,
- probe can confirm non-empty cells and route movement,
- frame-diff tooling can detect gait changes,
- thin player JSON mode can summarize action, anchor, route, and target,
- generated schema/rustdoc docs include all public fields.

## 10. Open design questions

1. Is `glyph_actor` the right public name, or should author-facing docs call it
   `actor` and reserve `glyph_actor` for the implementation kind?
2. Do action sequences belong inside the actor ingredient, or should they use the
   broader V3 sequencing model only?
3. Should target resolution use generic `placement.anchor` / `target.edge`, or a
   dedicated actor target block?
4. How much of pose timing should come from `mixed-signals` versus local frame
   tables?
5. Should glyph packs be built in, user-defined, or both?
6. Should emoji/modern terminal symbols be a glyph pack or a separate actor kind?
7. What is the smallest useful integration with runtime bindings for host rects?

## 11. Suggested implementation slices

### Slice A: static actor frames

- Define built-in stickman glyph pack.
- Render `stand`, `sit`, and `wave` at a fixed position.
- Add schema/rustdoc/docs and debug recipes.

### Slice B: route-driven movement

- Drive actor anchor along existing `motion.route`.
- Add `walk`, `run`, and `jump` frame timing.
- Prove route + pose frame interaction through probe/frame diff.

### Slice C: target rect anchoring

- Add authored rect target.
- Add `edge` and `align` semantics.
- Prove perch and jump-off recipes.

### Slice D: runtime binding targets

- Resolve host-provided rect bindings.
- Prove a gt-design-style toast/modal edge target without coupling tui-vfx to
  gt-design.

### Slice E: composition showcase

- Chain actor output into shadow, trail, highlighter, and landing ripple effects.
- Add one restrained professional recipe and one after-hours whimsical recipe.

## 12. Acceptance criteria

The capability is ready to leave post-release status only when:

- the first useful slice has schema/rustdoc/generated-doc coverage,
- every supported action has a debug recipe,
- actor output participates in normal V3 I/O chains,
- target anchoring works without ratatui-specific types,
- runtime-bound rects are either implemented or explicitly deferred,
- performance stays inside the 16.7 ms/frame budget for normal actor counts,
- docs explain when to use the actor and when not to.

<!-- <FILE>docs/design/post-release/glyph-actor-procedural-spec.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
