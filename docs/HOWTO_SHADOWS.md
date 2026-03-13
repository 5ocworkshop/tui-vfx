# HOWTO: Shadow Rendering in TUI-VFX

Quick reference for integrating shadow rendering into TUI applications.

## Architecture: Why Rect-Based Shadows

Shadows in `tui-vfx` are **rect-based by design**. This is intentional:

1. **Terminal UIs are fundamentally rectangular.** Every element (modals, panels, buttons, menus) occupies a rectangular region. There are no curved or irregular shapes in a terminal - only character cells in a grid.

2. **Shadows are element-level effects.** A modal casts a shadow. A popup casts a shadow. You shadow the *container*, not individual characters. This matches CSS `box-shadow` semantics.

3. **The compositor pipeline already works with rects.** The `render_pipeline()` function takes `width` and `height` parameters - it's already rect-bounded. Shadow simply extends those bounds.

4. **Passive when not used.** When `shadow: None`, the pipeline takes the non-shadow path with zero overhead.

## Overview

The `tui-vfx-shadow` crate provides drop shadow effects for terminal UIs. There are **two ways** to use shadows:

1. **Direct rendering** - Call `render_shadow()` before your element (simple, for static elements)
2. **Compositor integration** - Add shadow to `CompositionOptions` (recommended for animated elements)

## When to Use Which

| Scenario | Approach | Why |
|----------|----------|-----|
| Static modals/popups | Direct | Simpler, no extra allocation |
| Wipe/dissolve animations | Compositor | Shadow animates in sync with element |
| Fade animations only | Either | Both work, direct is simpler |
| Complex mask combinations | Compositor | Shadow respects all masks |

---

## Compositor Integration (Recommended for Animation)

### How It Works

When you provide a `ShadowSpec` to `render_pipeline()`:

1. **Extended area calculation**: The compositor creates a buffer larger than your source:
   - `extended_width = element_width + |shadow.offset_x|`
   - `extended_height = element_height + |shadow.offset_y|`

2. **Buffer rendering order**:
   - First: Shadow is rendered into the buffer at the offset position
   - Second: Element content is copied on top (obscuring shadow where they overlap)

3. **Mask application**: Masks evaluate over the EXTENDED dimensions, so the shadow wipes/dissolves in sync with the element.

### Visual: Extended Area Layout

```
For a 10x5 element with shadow offset (2, 1):

Extended buffer (12x6):
┌──────────────┐
│ Element      │ ← Element at (0,0) in buffer
│ (10x5)       │
│              │
│              │
│              │
│  ┌───────────┼──┐
│  │ Shadow    │  │ ← Shadow at (2,1) in buffer
│  │ (10x5)    │  │
└──┼───────────┘  │
   │              │
   │              │
   └──────────────┘

The overlap region: shadow is behind element (element overwrites)
The exposed region: bottom 1 row and right 2 columns show shadow
```

### Implementation Example

```rust
use tui_vfx::prelude::*;

// Your modal content - 10 columns × 5 rows
let modal_source: OwnedGrid = create_modal_content(10, 5);

// Shadow configuration
let shadow = ShadowSpec::new(
    ShadowConfig::new(Color::BLACK.with_alpha(150))
        .with_offset(2, 1)  // 2 cells right, 1 cell down
        .with_edges(ShadowEdges::BOTTOM_RIGHT)
);

// Render with compositor
render_pipeline(
    &modal_source,
    &mut dest,
    10,                    // Element width (NOT including shadow)
    5,                     // Element height (NOT including shadow)
    offset_x,              // Where to place in dest
    offset_y,
    CompositionOptions {
        t: animation_progress,  // 0.0-1.0
        shadow: Some(shadow),   // Shadow wipes with the element!
        masks: vec![MaskSpec::Wipe {
            reveal: Some(WipeDirection::TopToBottom),
            ..Default::default()
        }].into(),
        ..Default::default()
    },
    None,  // Optional inspector for debugging
);
```

**Critical**: The rendered area is 12×6, not 10×5. Your destination grid must have room at `(offset_x, offset_y)` for the extended area.

### Wipe Animation Behavior

With `WipeDirection::TopToBottom` and `t=0.5`:
- The mask reveals the top 50% of the *extended* area (rows 0-2 of 6)
- Both element AND shadow cells in that region are revealed
- Result: Shadow appears to wipe in from the top, synchronized with the modal

---

## Direct Rendering (Simple, for Static Elements)

For elements that don't animate (or only fade), render the shadow directly:

```rust
use tui_vfx::prelude::*;

// 1. Define where your element will be
let modal_rect = Rect::new(10, 5, 30, 12);  // x, y, width, height

// 2. Configure shadow
let shadow_config = ShadowConfig::new(Color::BLACK.with_alpha(128))
    .with_offset(2, 1)
    .with_style(ShadowStyle::HalfBlock)
    .with_edges(ShadowEdges::BOTTOM_RIGHT);

// 3. Render shadow FIRST (it goes behind the element)
render_shadow(&mut grid, modal_rect, &shadow_config, 1.0);

// 4. Now render your modal on top
draw_modal(&mut grid, modal_rect);
```

The shadow appears at `(modal_rect.x + 2, modal_rect.y + 1)` extending to the right and below.

---

## API Reference

### ShadowConfig (configuration)

```rust
ShadowConfig::new(color: Color)
    .with_offset(x: i8, y: i8)           // Shadow displacement (default: 1, 1)
    .with_style(style: ShadowStyle)      // Rendering style (default: HalfBlock)
    .with_edges(edges: ShadowEdges)      // Which edges to shadow (default: BOTTOM_RIGHT)
    .with_soft_edges(enabled: bool)      // Use half-blocks for transitions (default: true)
    .with_surface_color(color: Color)    // Background for blending (for HalfBlock)
    .with_composite_mode(mode)           // GlyphOverlay (default) or GradeUnderlying
    .with_grade(grade_config)            // Custom ShadowGradeConfig
    .with_dramatic_grade()               // Shorthand: GradeUnderlying + dramatic preset
```

### ShadowSpec (compositor wrapper)

```rust
// From ShadowConfig
let spec = ShadowSpec::new(shadow_config);

// Or simple creation (color, offset_x, offset_y)
let spec = ShadowSpec::simple(Color::BLACK.with_alpha(150), 2, 1);

// Query extended area dimensions
let extra_w = spec.extra_width();   // |offset_x|
let extra_h = spec.extra_height();  // |offset_y|
```

### Shadow Styles

| Style | Characters | Best For |
|-------|------------|----------|
| `ShadowStyle::HalfBlock` | `▐▄▌▀` + space | Default - sub-cell precision |
| `ShadowStyle::Braille { density: 0.7 }` | `⣿` patterns | Dithered/variable density |
| `ShadowStyle::Solid` | Space with bg color | Maximum compatibility |
| `ShadowStyle::Gradient { layers: 3 }` | Multi-layer | Soft drop shadows |

### Shadow Edges

```rust
ShadowEdges::BOTTOM_RIGHT  // Standard drop shadow (default)
ShadowEdges::ALL           // Shadow on all four sides
ShadowEdges::RIGHT | ShadowEdges::BOTTOM  // Custom combination
```

**Rule**: Edges only render when offset direction matches:
- `RIGHT` edge needs `offset_x > 0`
- `BOTTOM` edge needs `offset_y > 0`
- `LEFT` edge needs `offset_x < 0`
- `TOP` edge needs `offset_y < 0`

---

## Common Configurations

### Standard Drop Shadow
```rust
ShadowConfig::new(Color::BLACK.with_alpha(128))
    .with_offset(2, 1)
    .with_edges(ShadowEdges::BOTTOM_RIGHT)
```

### Soft Gradient Shadow
```rust
ShadowConfig::new(Color::BLACK.with_alpha(200))
    .with_offset(1, 1)
    .with_style(ShadowStyle::Gradient { layers: 3 })
```

### Floating Effect (All Sides)
```rust
ShadowConfig::new(Color::BLACK.with_alpha(100))
    .with_offset(1, 1)
    .with_edges(ShadowEdges::ALL)
```

### High Contrast (Solid)
```rust
ShadowConfig::new(Color::BLACK.with_alpha(200))
    .with_offset(1, 1)
    .with_style(ShadowStyle::Solid)
```

---

## Animation Examples

### Wipe Animation (Compositor)

```rust
fn render_animated_modal(
    modal_source: &impl Grid,
    dest: &mut impl Grid,
    width: usize,
    height: usize,
    offset: (usize, usize),
    progress: f64,  // 0.0 = closed, 1.0 = fully open
) {
    let shadow = ShadowSpec::new(
        ShadowConfig::new(Color::BLACK.with_alpha(150))
            .with_offset(2, 1)
    );

    render_pipeline(
        modal_source,
        dest,
        width,
        height,
        offset.0,
        offset.1,
        CompositionOptions {
            t: progress,
            shadow: Some(shadow),
            masks: vec![MaskSpec::Wipe {
                reveal: Some(WipeDirection::TopToBottom),
                ..Default::default()
            }].into(),
            ..Default::default()
        },
        None,
    );
}
```

### Fade Animation (Direct)

```rust
// In your animation loop
for frame in 0..60 {
    let t = frame as f64 / 60.0;

    // Shadow fades in with modal (progress controls opacity)
    render_shadow(&mut grid, rect, &config, t);

    // Draw modal with matching opacity
    draw_modal_with_opacity(&mut grid, rect, t);
}
```

---

## Shadow Compositing Modes

By default, shadows use **glyph overlay** compositing: shadow characters (half-blocks, braille patterns, etc.) replace whatever is underneath. This is the traditional approach.

For a more sophisticated look, **grade-underlying** compositing preserves destination glyphs and modifiers while applying color grading (desaturation, dimming, tinting) to the shadow region. Text beneath the shadow remains readable but visually recedes.

### ShadowCompositeMode

| Mode | Behavior |
|------|----------|
| `GlyphOverlay` (default) | Shadow glyphs replace destination content |
| `GradeUnderlying` | Destination glyphs preserved; color grading applied |

### Dramatic Grade-Underlying Example

```rust
let shadow_config = ShadowConfig::new(Color::BLACK.with_alpha(180))
    .with_offset(2, 1)
    .with_edges(ShadowEdges::BOTTOM_RIGHT)
    .with_style(ShadowStyle::Solid)
    .with_dramatic_grade();  // enables GradeUnderlying with visible preset

render_pipeline(
    &modal_source,
    &mut dest,
    10, 5,
    offset_x, offset_y,
    CompositionOptions {
        t: 1.0,
        shadow: Some(ShadowSpec::new(shadow_config)),
        ..Default::default()
    },
    None,
);
```

The `dramatic()` preset applies stronger background grading than foreground grading, making the shadow region clearly visible while keeping text legible. Background grading is intentionally stronger because backgrounds occupy more visual area and contribute more to the perception of depth.

### Custom Grade Parameters

For fine-tuned control, construct `ShadowGradeConfig` directly:

```rust
use tui_vfx_shadow::ShadowGradeConfig;

let config = ShadowConfig::new(Color::BLACK.with_alpha(180))
    .with_offset(2, 1)
    .with_composite_mode(ShadowCompositeMode::GradeUnderlying)
    .with_grade(ShadowGradeConfig {
        fg_dim_strength: 0.15,
        bg_dim_strength: 0.40,
        fg_desaturate_strength: 0.10,
        bg_desaturate_strength: 0.30,
        fg_tint_strength: 0.05,
        bg_tint_strength: 0.10,
        preserve_fg_alpha: true,
        preserve_bg_alpha: true,
    });
```

All strength values range from `0.0` (no effect) to `1.0` (maximum) and are further scaled by shadow coverage at each cell.

---

## Key Points

1. **Shadows are rect-based** - This matches terminal UI reality where everything is rectangular
2. **For animated elements (wipes, dissolves)** - Use compositor integration
3. **For static elements** - Direct rendering is simpler
4. **Compositor extends the render area** - Account for `|offset_x|` and `|offset_y|` extra cells
5. **Progress parameter** - Controls shadow opacity (0.0 = invisible, 1.0 = full)
6. **Surface color matters for HalfBlock** - Set it to match your background for proper blending
7. **Offset direction controls edge rendering** - Positive offset = bottom-right shadow

---

## Imports

```rust
// Full access via prelude
use tui_vfx::prelude::*;

// Compositor integration specifically
use tui_vfx_compositor::pipeline::{CompositionOptions, ShadowSpec, render_pipeline};

// Direct shadow rendering specifically
use tui_vfx::shadow::{render_shadow, ShadowConfig, ShadowEdges, ShadowStyle};
use tui_vfx::types::{Color, Rect, Grid};
```

---

## Troubleshooting

### Shadow not visible
- Check `color` has alpha > 0 (e.g., `Color::BLACK.with_alpha(150)`)
- Check `progress` parameter is > 0.0
- Ensure shadow offset matches edge flags (positive offset for BOTTOM_RIGHT)

### Shadow appears on wrong side
- Offset sign determines direction: positive = right/down, negative = left/up
- Edge flags must match offset direction to render

### HalfBlock looks wrong
- Set `surface_color` to match your actual background color
- The half-block characters blend `fg` (shadow) with `bg` (surface)

### Wipe animation: shadow not syncing
- Use compositor integration, not direct rendering
- Ensure `shadow` is passed in `CompositionOptions`, not rendered separately
