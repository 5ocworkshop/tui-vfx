// <FILE>examples/direct_api_signal_strength.rs</FILE> - <DESC>Direct-API signal example — constructs a Vignette filter whose `strength` is a sine signal in Rust (no recipe JSON), renders frames across t, and prints the sampled signal value below each frame.</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Direct-API signal example backing Intention 44.</WCTX>
// <CLOG>0.1.0: initial — Vignette + sine sampled at t = 0.0, 0.25, 0.5, 0.75, 1.0; sampled strength printed below each frame.</CLOG>

//! Direct-API signal example.
//!
//! Recipe authors reach signals through the `tui-vfx-recipes` facade
//! (`VfxRecipeSignalSpec`); direct-API consumers do not — they construct
//! [`mixed_signals::types::SignalSpec`] values in Rust and pass them into
//! engine field types (e.g. `FilterSpec::Vignette { strength:
//! SignalOrFloat::Signal(...), ... }`). This example exercises the
//! direct-API path end to end.
//!
//! Run with:
//!   cargo run -p tui-vfx --example direct_api_signal_strength
//!
//! See **Intention 44** in `steering/INTENTIONS.md` for the full rule.

use mixed_signals::prelude::SignalContext;
use mixed_signals::types::{SignalOrFloat, SignalSpec};
use tui_vfx::prelude::*;
use tui_vfx_types::{RoleMap, RoleTag, SemanticScene};

fn main() {
    println!("tui-vfx direct-API signal example\n");
    println!(
        "FilterSpec::Vignette {{ strength: SignalOrFloat::Signal(SignalSpec::Sine \
         {{ frequency: 0.5, amplitude: 0.4, offset: 0.5, phase: 0.0 }}), radius: \
         SignalOrFloat::Static(0.55), .. }}\n"
    );
    println!(
        "The signal samples to a sinusoidal strength as t advances; the printed \
         strength value below each frame proves the engine is sampling the signal \
         on every render.\n"
    );

    let probe_signal = SignalOrFloat::from(SignalSpec::Sine {
        frequency: 0.5,
        amplitude: 0.4,
        offset: 0.5,
        phase: 0.0,
    });
    let ctx = SignalContext::new(0, 0);

    for &progress in &[0.0_f64, 0.25, 0.5, 0.75, 1.0] {
        let sampled = probe_signal
            .evaluate(progress, &ctx)
            .expect("sine signal evaluates");
        let frame = snapshot_vignette(progress);
        println!("-- t = {progress:.2}   sampled strength = {sampled:.4} --");
        println!("{frame}\n");
    }
}

fn snapshot_vignette(progress: f64) -> String {
    let width = 36;
    let height = 7;

    // Build a simple bordered card as the source.
    let source = text_card(
        width,
        height,
        "DIRECT API VIGNETTE",
        Color::rgb(220, 235, 255),
        Color::rgb(20, 26, 36),
    );
    let dest = blank_grid(width, height, Color::rgb(8, 12, 18));

    // Build the signal directly from mixed_signals — no facade.
    let strength = SignalOrFloat::from(SignalSpec::Sine {
        frequency: 0.5,
        amplitude: 0.4,
        offset: 0.5,
        phase: 0.0,
    });

    let options = CompositionOptions::default().with_filter(FilterSpec::Vignette {
        strength,
        radius: SignalOrFloat::Static(0.55),
        sides: Vec::new(),
        dither_amount: 0.0,
        temporal_dither_hz: 0.0,
    });

    let source_roles = RoleMap::all_background(width as u16, height as u16);
    let mut dest_scene = SemanticScene::from_grid_with_default_role(dest, RoleTag::Background);
    render_pipeline(
        &source,
        &source_roles,
        &mut dest_scene,
        width,
        height,
        0,
        0,
        CompositionOptions {
            t: progress,
            ..options
        },
        None,
    );
    grid_to_string(dest_scene.grid())
}

fn text_card(width: usize, height: usize, text: &str, fg: Color, bg: Color) -> OwnedGrid {
    let mut grid = blank_grid(width, height, bg);
    for x in 0..width {
        set_cell(&mut grid, x, 0, '─', fg, bg);
        set_cell(&mut grid, x, height - 1, '─', fg, bg);
    }
    for y in 0..height {
        set_cell(&mut grid, 0, y, '│', fg, bg);
        set_cell(&mut grid, width - 1, y, '│', fg, bg);
    }
    set_cell(&mut grid, 0, 0, '╭', fg, bg);
    set_cell(&mut grid, width - 1, 0, '╮', fg, bg);
    set_cell(&mut grid, 0, height - 1, '╰', fg, bg);
    set_cell(&mut grid, width - 1, height - 1, '╯', fg, bg);
    let center_x = (width.saturating_sub(text.chars().count())) / 2;
    draw_text(&mut grid, center_x, height / 2, text, fg, bg);
    grid
}

fn blank_grid(width: usize, height: usize, bg: Color) -> OwnedGrid {
    let mut grid = OwnedGrid::new(width, height);
    for y in 0..height {
        for x in 0..width {
            set_cell(&mut grid, x, y, ' ', Color::rgb(220, 220, 220), bg);
        }
    }
    grid
}

fn draw_text(grid: &mut OwnedGrid, x: usize, y: usize, text: &str, fg: Color, bg: Color) {
    for (idx, ch) in text.chars().enumerate() {
        if x + idx >= grid.width() || y >= grid.height() {
            break;
        }
        set_cell(grid, x + idx, y, ch, fg, bg);
    }
}

fn set_cell(grid: &mut OwnedGrid, x: usize, y: usize, ch: char, fg: Color, bg: Color) {
    if let Some(cell) = grid.get_mut(x, y) {
        cell.ch = ch;
        cell.fg = fg;
        cell.bg = bg;
    }
}

fn grid_to_string(grid: &OwnedGrid) -> String {
    let mut out = String::new();
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let ch = grid.get(x, y).map(|c| c.ch).unwrap_or(' ');
            out.push(ch);
        }
        if y + 1 < grid.height() {
            out.push('\n');
        }
    }
    out
}

// <FILE>examples/direct_api_signal_strength.rs</FILE> - <DESC>Direct-API signal example</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
