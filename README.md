<!-- <FILE>README.md</FILE> - <DESC>Project overview and usage guide</DESC> -->
<!-- <VERS>VERSION: 1.0.3</VERS> -->
<!-- <WCTX>Public release prep</WCTX> -->
<!-- <CLOG>Fix metadata header to use HTML comments</CLOG> -->
# tui-vfx

Cell-based visual effects for terminal UIs.
Don't just draw text. Direct the screen.

`tui-vfx` is a framework-agnostic compositing engine for terminal UIs. It treats the
terminal grid like a texture: you provide content, then apply a chain of effects to
produce the final frame. The pipeline supports masks, filters, samplers, shaders, and
content transformers, all operating over a simple `Grid` trait that you can implement
for any terminal rendering framework.

[![Crates.io](https://img.shields.io/crates/v/tui-vfx.svg)](https://crates.io/crates/tui-vfx)
[![License](https://img.shields.io/crates/l/tui-vfx.svg)](https://github.com/5ocworkshop/tui-vfx/blob/main/LICENSE)

---

## Why tui-vfx

Most terminal apps feel static. When state changes, screens just swap. `tui-vfx` lets
you communicate context and hierarchy with motion and visual effects, without writing
custom render loops.

- Create focus: use `Dim` and `Vignette` filters to push background content back when a
  modal opens.
- Signal feedback: use color shaders or `Tint`/`Invert` filters to highlight warnings
  and errors.
- Guide the eye: use `Wipe`, `Iris`, or `Dissolve` masks to show where data is coming from.
- Add delight: use `Typewriter`, `Scramble`, or `Morph` to animate text changes.

---

## How it works: the pipeline

This is not a widget library. It is a compositor that transforms a source grid into a
rendered frame by applying effects in a configurable pipeline.

1. Samplers (vertex stage): warp the coordinate space (Ripple, SineWave, CRT distortion).
2. Masks (stencil stage): control visibility patterns (Dissolve, Iris, Wipe).
3. Style shaders (fragment stage): procedural color and style generation (Fade, Gradient).
4. Filters (post-processing): final color and character adjustments (Dim, Greyscale, Tint).
5. Content transformers: text-level mutations (Typewriter, Scramble, Morph).

Everything is configured through `CompositionOptions` and effect "specs" such as
`MaskSpec` and `FilterSpec`.

---

## Features at a glance

### Transitions and masks (10 types)
Dissolve, Wipe, Iris, Blinds, Checkers, Diamond, Cellular, Radial, PathReveal, NoiseDither

### Filters (8 types)
Dim, Brighten, Tint, Invert, Vignette, PatternFill, Greyscale, and more

### Samplers (6 types)
Ripple, SineWave, CRT, CRTJitter, FaultLine, Shredder

### Style shaders (14 types)
Fade, Gradient, ColorRamp, and various color effects

### Content transformers (12 types)
Typewriter, Scramble, Morph, and text manipulation effects

---

## Requirements

- **Rust 1.86.0+** (edition 2024)

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
tui-vfx = "0.2"
```

---

## Quick start

```rust
use tui_vfx::prelude::*;

// Create source and destination grids
let source = OwnedGrid::new(80, 24);
let mut dest = OwnedGrid::new(80, 24);

// Create composition options with effects
let options = CompositionOptions {
    t: 0.5, // halfway through animation
    masks: vec![MaskSpec::Dissolve { seed: 42, chunk_size: 1 }].into(),
    ..Default::default()
};

// Apply effects
render_pipeline(
    &source,
    &mut dest,
    80,   // width
    24,   // height
    0,    // offset_x
    0,    // offset_y
    options,
    None, // optional inspector
);
```

---

## Example: modal transition

Dim and desaturate the background while a modal dissolves in on top.

```rust
use tui_vfx::prelude::*;

fn render_modal_transition(
    bg_source: &OwnedGrid,
    modal_source: &OwnedGrid,
    dest: &mut OwnedGrid,
    t: f64
) {
    // 1. Render background: dim it and desaturate it
    let bg_opts = CompositionOptions::default()
        .with_filters(vec![
            FilterSpec::Greyscale {
                strength: SignalOrFloat::Static(1.0),
                apply_to: ApplyTo::Both
            },
            FilterSpec::Dim {
                factor: SignalOrFloat::Static(0.5),
                apply_to: ApplyTo::Both
            }
        ]);

    render_pipeline(bg_source, dest, 80, 24, 0, 0, bg_opts, None);

    // 2. Render modal: dissolve it in on top
    let modal_opts = CompositionOptions::default()
        .t(t)
        .with_mask(MaskSpec::Dissolve {
            seed: 12345,
            chunk_size: 2
        });

    render_pipeline(modal_source, dest, 40, 10, 20, 7, modal_opts, None);
}
```

---

## Integration and adapters

`tui-vfx` is designed to sit between your layout calculation and your terminal rendering.
It is framework-agnostic: implement the `Grid` trait for your buffer type and you are done.

```rust
impl Grid for MyFrameworkBuffer {
    fn width(&self) -> usize { self.cols }
    fn height(&self) -> usize { self.rows }
    fn get(&self, x: usize, y: usize) -> Option<&Cell> { /* ... */ }
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> { /* ... */ }
    fn set(&mut self, x: usize, y: usize, cell: Cell) { /* ... */ }
}
```

### Using with Ratatui

`tui-vfx` can wrap the Ratatui buffer with a thin adapter.

```rust
use tui_vfx::prelude::*;
use ratatui::buffer::Buffer;

struct RatatuiAdapter<'a>(&'a mut Buffer);

impl<'a> Grid for RatatuiAdapter<'a> {
    fn width(&self) -> usize { self.0.area.width as usize }
    fn height(&self) -> usize { self.0.area.height as usize }

    fn get(&self, x: usize, y: usize) -> Option<&Cell> {
        // ... mapping logic ...
    }
    fn set(&mut self, x: usize, y: usize, cell: Cell) {
        // ... mapping logic ...
    }
}
```

---

## Architecture

The library is organized into several sub-crates:

| Crate | Purpose |
| --- | --- |
| `tui-vfx-types` | Foundation types (Color, Style, Cell, Grid trait) |
| `tui-vfx-core` | Schema and introspection primitives |
| `tui-vfx-geometry` | Math, layout, and motion primitives |
| `tui-vfx-style` | Color interpolation and style effects |
| `tui-vfx-content` | Text manipulation primitives |
| `tui-vfx-compositor` | Pipeline and compositing effects |

### Data-driven configuration

Effect specs (`MaskSpec`, `FilterSpec`, and others) are data structures that can be
serialized. This enables JSON or TOML-defined animations, live-reload workflows, and
potential scripting bindings while keeping the heavy lifting in Rust.

---

## Documentation

See `docs/generated/API.md` for the complete API reference.

Machine-readable references:
- `docs/generated/capabilities.json` — effect inventory + key parameters
- `docs/generated/effect_schemas.json` — full ConfigSchema per effect

## Recipe validation

Validate JSON recipes in `../tui-vfx-recipes/recipes/**`:

```
just recipes-validate
```

Outputs:
- `docs/generated/recipes_validation.json`
- `docs/generated/recipes_validation.md`

## License

MIT

<!-- <FILE>README.md</FILE> - <DESC>Project overview and usage guide</DESC> -->
<!-- <VERS>END OF VERSION: 1.0.3</VERS> -->
