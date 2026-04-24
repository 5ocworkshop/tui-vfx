<!-- <FILE>docs/design/post-release/weather-ambient-field-spec.md</FILE> - <DESC>Post-release specification for ambient weather procedurals such as rain, snow, wind, fog, lightning, and time-of-day lighting.</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Capture a deferred weather/ambient-field procedural family that composes with V3 motion, style, spatial fields, and I/O chains.</WCTX> -->
<!-- <CLOG>0.1.0: capture initial weather ambient field spec.</CLOG> -->

# Weather ambient field spec

**Status: Post-release project.** This is not release-blocking V3 work. Keep it
as a deferred capability until the core V3 release gate, recipe migration, and
as-built docs are stable.

## 1. Purpose

Add a small family of ambient weather ingredients for terminal scenes:

- rain
- snow
- fog / mist
- wind
- lightning
- clouds
- time-of-day lighting
- ash / sparks / dust variants when earned by recipes

The goal is not a meteorology simulator. The goal is a composable environmental
layer that can create atmosphere, communicate state, and share fields with other
V3 ingredients.

Examples:

```text
Soft rain behind a notification.
Snow drifting across an idle screen.
Dusk tint on a splash screen.
Lightning flash on a warning.
A shared wind field driving rain and a flag.
A light fog mask over a disabled modal backdrop.
```

## 2. Design principles

1. **Weather is a set of ingredients, not one giant effect.** Authors should be
   able to use rain without time-of-day lighting, or wind without precipitation.
2. **Shared fields first.** Wind, density, brightness, and turbulence should be
   reusable fields that other ingredients can consume.
3. **Recipe-level control.** Density, speed, direction, palette, intensity, and
   lifetime should be authorable and bindable.
4. **Grid-first rendering.** Weather renders to the abstract cell grid and does
   not depend on ratatui types.
5. **Low-cost by default.** Ambient effects can run for long periods, so avoid
   expensive per-cell recomputation and repeated allocations.
6. **Readable before realistic.** If an effect looks like rain at terminal scale,
   it wins. Subtle, legible motion beats noisy simulation.

## 3. Conceptual model

```text
time signals ────┐
                 ├─ ambient field controller ─┬─ precipitation source ─┐
spatial fields ──┤                            ├─ lighting/tint source ─┼─> V3 cells
                 │                            ├─ fog/noise mask ───────┤
runtime inputs ──┘                            └─ burst events ─────────┘
```

A weather recipe evaluates:

1. global environmental fields, such as wind and light;
2. local particle/source layers, such as rain or snow;
3. optional burst events, such as lightning or gusts;
4. downstream style/effect chains.

## 4. Ingredients

### 4.1 Wind field

Reusable vector field that can drive multiple consumers.

Author controls:

- direction
- speed
- gust amount
- turbulence
- seed
- runtime binding overrides

Consumers:

- rain drift
- snow flutter
- flag wave speed/direction
- text trail displacement
- fog flow

```text
shared wind field
   ├─ rain slants right
   ├─ flag waves faster
   └─ fog crawls across backdrop
```

### 4.2 Rain source

Precipitation source that emits falling glyphs or short streaks.

Useful glyphs:

```text
|  /  ╱  ˈ  ⋮
```

Author controls:

- density
- fall speed
- slant
- length
- brightness
- spawn seed
- wind input
- splash/accent probability

Basic presentation:

```text
╱    ╱      ╱
   ╱    ╱
╱      ╱    ╱
```

### 4.3 Snow source

Slower precipitation source with flutter.

Useful glyphs:

```text
·  *  ❄  ✦  ⁕
```

Author controls:

- density
- fall speed
- flutter amplitude
- drift
- glyph mix
- brightness
- melt/fade behavior

Basic presentation:

```text
  ·      *
     ·       ❄
 *       ·
```

### 4.4 Fog / mist mask

Soft low-frequency field that dims, reveals, or tints content.

Author controls:

- opacity
- noise scale
- drift
- softness
- tint
- reveal threshold

Use cases:

- modal backdrops
- disabled surfaces
- atmospheric splash screens
- reveal transitions

### 4.5 Time-of-day lighting

Global or scoped lighting profile that changes foreground/background color,
contrast, and warmth.

Profiles:

- dawn
- noon
- dusk
- night
- storm
- custom curve

Author controls:

- profile
- intensity
- color temperature
- contrast
- cycle duration
- static sample time

This should remain palette-agnostic. tui-vfx can express RGB or semantic slots
provided by the host; downstream systems decide theme policy.

### 4.6 Lightning / burst events

Short-lived flash and optional branch glyphs.

Author controls:

- trigger mode: periodic, random, binding, phase start
- flash duration
- branch density
- color
- screen region
- accompanying shake/bell hooks if the host supports them

Basic presentation:

```text
       ╲
        ╲╱
         ╲
```

Lightning is a nudge. Use it for warning, alarm, storm atmosphere, or playful
moments. Do not make it a default background effect.

### 4.7 Clouds / shadow bands

Slow, broad masks or braille/block textures.

Author controls:

- scale
- drift
- opacity
- texture glyphs
- shadow/tint behavior

Clouds can be implemented as a fog/noise variant until recipes prove they need a
separate public ingredient.

## 5. Authoring sketch

Illustrative only; not a final schema commitment.

```json
{
  "kind": "procedural",
  "procedural": "weather.rain",
  "density": 0.22,
  "fall_speed": 1.4,
  "glyph_pack": "streaks",
  "wind": {
    "direction_degrees": 105,
    "speed": 0.18,
    "gust": 0.05
  },
  "style": {
    "fg": "#8fb7c9",
    "alpha": 0.65
  }
}
```

Shared wind with multiple consumers:

```json
{
  "requires_outputs": [
    { "id": "storm_wind", "kind": "vector_field" }
  ],
  "pipeline": {
    "sequence": [
      { "procedural": "weather.wind", "outputs": { "field": "storm_wind" } },
      { "procedural": "weather.rain", "inputs": { "wind": "storm_wind" } },
      { "procedural": "braille_flag_field", "inputs": { "wind": "storm_wind" } }
    ]
  }
}
```

Time-of-day tint:

```json
{
  "kind": "style_effect",
  "type": "ambient_light",
  "profile": "dusk",
  "intensity": 0.35,
  "scope": "background"
}
```

## 6. Runtime binding opportunities

Weather becomes more useful when host applications can bind values at runtime:

- `weather.intensity`
- `weather.wind_speed`
- `weather.wind_direction`
- `weather.time_of_day`
- `weather.storm_level`
- `weather.paused`
- `weather.seed`

Examples:

```text
Build failed      -> storm_level increases briefly.
Night theme       -> time_of_day = night.
Idle screen       -> snow density slowly rises.
User focus event  -> fog clears around focused panel.
```

## 7. Relationship to mixed-signals

Reusable signal/math substrate should live in `mixed-signals` when it is generic:

- vector fields
- noise fields
- gust envelopes
- temporal profiles
- seeded random streams
- particle spawn distributions

`tui-vfx` should own:

- recipe vocabulary
- terminal glyph emission
- style/tint/mask behavior
- probe/debug recipes
- V3 I/O composition semantics

## 8. Debug recipe requirements

When implemented, add primitive-first debug recipes before showcases.

Required basic recipes:

1. `weather_rain_basic.json`
2. `weather_snow_basic.json`
3. `weather_wind_field_basic.json`
4. `weather_fog_basic.json`
5. `weather_time_of_day_dusk_basic.json`
6. `weather_lightning_basic.json`

Required composition recipes:

1. rain driven by shared wind,
2. snow with time-of-day tint,
3. fog reveal over text,
4. lightning nudge on warning message,
5. shared wind driving rain and a flag-like surface.

Each recipe should include:

- `metadata.expected_visual`,
- body text naming the ingredient clearly,
- transparent/minimal background for basic recipes,
- one simple presentation before any showcase composition.

## 9. Validation and tooling requirements

Before promotion from post-release idea to active work:

- schema/rustdocs cover each public field,
- generated docs include the ingredient family,
- validator rejects invalid density/speed/glyph-pack combinations,
- probe can summarize density, non-empty cells, and motion direction,
- frame diff can detect falling/drifting motion,
- thin player JSON mode reports weather kind, field inputs/outputs, seed, and
  intensity,
- performance stays inside the 16.7 ms/frame target for normal grids.

## 10. Open design questions

1. Should public names be `weather.rain` / `weather.snow`, or `rain_field` /
   `snow_field`?
2. Is wind a standalone ingredient, a `mixed-signals` field source, or both?
3. Should time-of-day lighting be weather-owned or a broader ambient lighting
   style effect?
4. How should deterministic randomness be seeded across recipe playback, probe,
   trace, and movie export?
5. Should precipitation output be sparse cells, cell runs, braille subcells, or a
   mix selected by glyph pack?
6. How do scoped weather layers interact with modal z-level and prominence?
7. Should lightning be allowed to trigger host-level bell/title effects, or only
   visual cells?

## 11. Suggested implementation slices

### Slice A: deterministic rain

- Implement seeded sparse rain cells.
- Support density, speed, slant, glyph pack, and color.
- Add basic rain debug recipe and probe coverage.

### Slice B: snow and flutter

- Add slower falling particles with horizontal flutter.
- Reuse shared spawn/timing substrate.
- Add basic snow debug recipe.

### Slice C: shared wind field

- Add reusable field output.
- Drive rain and one existing recipe consumer from the same field.
- Keep reusable field math in `mixed-signals` if it proves generic.

### Slice D: fog and ambient light

- Add soft mask/tint layers.
- Prove dusk/night profiles without theme coupling.

### Slice E: lightning and gust events

- Add short burst envelope.
- Prove visual flash and optional branch glyphs.
- Keep host bell/title hooks out of runtime until separately approved.

## 12. Acceptance criteria

The capability is ready to leave post-release status only when:

- basic rain/snow/fog/light recipes are validated and documented,
- shared wind can feed at least two consumers,
- generated docs and rustdocs are current,
- probe/trace tooling can inspect the output deterministically,
- runtime bindings can vary intensity or wind without recipe rewrites,
- performance is acceptable for continuous ambient playback,
- docs explain when weather helps and when it becomes visual noise.

<!-- <FILE>docs/design/post-release/weather-ambient-field-spec.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
