<!-- <FILE>docs/memo_pipeline_api.md</FILE> - <DESC>Memo on pipeline API usage</DESC> -->
<!-- <VERS>VERSION: 1.0.0</VERS> -->
<!-- <WCTX>Docs hygiene: add required metadata headers</WCTX> -->
<!-- <CLOG>Add metadata header and footer for documentation standards</CLOG> -->

# Memo: Mastering the TUI-VFX Pipeline API

**Date:** January 27, 2026
**Subject:** Demystifying the Top-Level Pipeline API for UI Animation

---

## The "Why" and "How" of the Pipeline

I've noticed some confusion around the `tui_vfx::render_pipeline` API. It might look intimidating with all its options, but it is effectively the **single point of truth** for all visual effects in our system. Instead of calling separate functions for shadows, masks, and filters, you configure a `CompositionOptions` struct and pass it once.

### The Mental Model

1.  **Source**: Your raw UI component (e.g., a Ratatui buffer or a temporary grid).
2.  **Options**: A recipe defining *how* to change the look (masks, filters, shaders, shadows).
3.  **Destination**: Where the final pixels go (the main screen buffer).
4.  **Time (`t`)**: The magic ingredient. You drive `t` (usually 0.0 to 1.0) to animate everything.

## Entry Point

Use `render_pipeline` for everything.

```rust
use tui_vfx::prelude::*;

// The "Superset" function
render_pipeline(
    source_grid,      // &dyn Grid (your widget content)
    dest_grid,        // &mut dyn Grid (screen buffer)
    width, height,    // Dimensions
    offset_x, offset_y, // Position on screen
    options,          // CompositionOptions (The Effect Recipe)
    None              // Optional Inspector
);
```

---

## Example 1: The "Juicy" Button

Let's animate a button that:
1.  Has a glowing border when focused.
2.  Uses the `PillButton` filter for rounded aesthetic.
3.  "Presses" down slightly when clicked (using `SamplerSpec`).

### The Setup

```rust
// Assume we have a `button_state` with:
// - is_focused: bool
// - is_pressed: bool
// - animation_progress: f64 (0.0 to 1.0, driven by an animation loop)

let mut options = CompositionOptions::default();

// 1. Base Styling (The Pill Shape)
// This filter renders the button with rounded, shaded edges.
options = options.with_filter(FilterSpec::PillButton {
    edge_width: 1, // 1 cell thick edge
    glisten: if button_state.is_focused { 
        // Subtle glisten when focused
        Some(SignalOrFloat::Static(0.5)) 
    } else { 
        None 
    },
    progress: button_state.animation_progress, // Animate the glisten
});

// 2. Focus State (Neon Glow)
if button_state.is_focused {
    // Add a spatial shader for the glow
    options = options.with_shader_layer(
        &tui_vfx_style::models::SpatialShaderType::Glow {
            radius: 2.0,
            intensity: 1.5,
            falloff: tui_vfx_style::models::FalloffType::Quadratic,
            pulse_speed: 0.5, // Throb gently
            color: ColorConfig::Cyan, // Neon Cyan glow
        },
        StyleRegion::BorderOnly, // Only glow the border
    );
}

// 3. Press Interaction (Physical Displacement)
if button_state.is_pressed {
    // Use a Sampler to physically "push" the pixels down
    options.sampler_spec = Some(SamplerSpec::SineWave {
        axis: Axis::Y,
        amplitude: 0.5, // Subtle shift
        frequency: 1.0,
        speed: 0.0,
        phase: 1.57, // Fixed phase for a static "squish" or animate it
    });
    
    // OR simply use translation if available, but Samplers are fun for distortion.
    // A better "press" might just be a shadow reduction:
    options = options.with_shadow(ShadowSpec::new(ShadowConfig {
        style: ShadowStyle::Solid,
        offset_x: 0, // No shadow when pressed (close to surface)
        offset_y: 0,
        color: Color::Black,
        ..Default::default()
    }));
} else {
    // Floating state
    options = options.with_shadow(ShadowSpec::new(ShadowConfig {
        style: ShadowStyle::HalfBlock, // Smooth shadows
        offset_x: 1,
        offset_y: 1,
        color: Color::DarkGray,
        edges: ShadowEdges::BOTTOM_RIGHT,
        soft_edges: true,
        surface_color: None, 
    }));
}

// Render!
render_pipeline(
    &button_source, 
    frame.buffer_mut(), 
    button_area.width as usize, 
    button_area.height as usize, 
    button_area.x as usize, 
    button_area.y as usize, 
    options, 
    None
);
```

---

## Example 2: The Notification Toast

This example demonstrates entry/exit transitions using **Masks** and **Motion**.

### Scenario
A notification slides in from the right (`Wipe` mask) while simultaneously fading in (`Dissolve` mask for texture). It stays for a few seconds, then exits.

```rust
let t = notification_state.enter_progress; // 0.0 -> 1.0

let mut options = CompositionOptions::default();
options.t = t; // Global time for the pipeline

// 1. Entry Animation: Wipe from Right
// As t goes 0->1, the mask reveals content from right to left.
options = options.with_mask(MaskSpec::Wipe {
    reveal: WipeDirection::FromRight, 
    soft_edge: true, // Soft gradient edge
    // "reveal" implies we are showing content. 
    // At t=0, it's hidden. At t=1, it's fully visible.
    // Note: ensure you handle 'hide' or 'reveal' correctly.
    hide: None, 
    direction: None, // Legacy alias, avoid if using reveal/hide
});

// 2. Secondary Effect: Digital Dissolve
// Combine it with a dissolve for a "techy" feel.
options = options.with_mask(MaskSpec::Dissolve {
    seed: 42,
    chunk_size: 2, // Blocky dissolve
});

// Combine mode: Both must be true to show pixel (allows complex reveals)
options = options.with_mask_combine_mode(MaskCombineMode::Blend { ratio: 0.5 });


// 3. Shadow that grows with the notification
// Shadows are computed *after* samplers but *before* masks in the pipeline,
// so the shadow will naturally mask in with the content!
options = options.with_shadow(ShadowSpec::new(ShadowConfig {
    style: ShadowStyle::Gradient { layers: 2 },
    offset_x: 2,
    offset_y: 2,
    color: Color::Black,
    edges: ShadowEdges::ALL,
    soft_edges: true,
    ..Default::default()
}));

// 4. Background Texture (Filter)
// Add a subtle scanline effect to the notification background
options = options.with_filter(FilterSpec::InterlaceCurtain {
    density: 2,
    dim_factor: 0.3,
    scroll_speed: 0.0, // Static texture
});


// Render
render_pipeline(
    &toast_source, 
    frame.buffer_mut(), 
    area.width as usize, 
    area.height as usize, 
    area.x as usize, 
    area.y as usize, 
    options, 
    None
);
```

---

## Key Takeaways for "Confused" Devs

1.  **Don't over-engineer manually**: If you want a shadow, don't draw it yourself. Use `.with_shadow()`. If you want a fade, use `.with_mask()`.
2.  **Order Matters**: 
    *   **Samplers** warp space (move pixels).
    *   **Shadows** are cast.
    *   **Masks** hide/reveal the result.
    *   **Filters** colorize/post-process.
    *   **Shaders** apply surface effects (gradients, gloss).
3.  **`t` is King**: Almost all effects are stateless functions of `t`. If you pass the same `t`, you get the same frame. This makes testing and debugging animations trivial.

Hope this helps clear up the pipeline usage!

<!-- <FILE>docs/memo_pipeline_api.md</FILE> - <DESC>Memo on pipeline API usage</DESC> -->
<!-- <VERS>END OF VERSION: 1.0.0</VERS> -->
