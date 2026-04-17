//! Printable showcase for the richer pipeline-level effects introduced after the
//! initial parity pass: Materialize, EdgeGrow, and GlyphCascade.
//!
//! Run with:
//!   cargo run -p tui-vfx --example pipeline_effects_showcase

use tui_vfx::compositor::types::{BindableValue, RadialOrigin};
use tui_vfx::content::types::DissolveDirection;
use tui_vfx::prelude::*;

fn main() {
    println!("tui-vfx pipeline effects showcase\n");

    print_demo(
        "Materialize",
        "Organic in-place resolve mask",
        &[
            snapshot_materialize(0.18),
            snapshot_materialize(0.52),
            snapshot_materialize(1.0),
        ],
    );

    print_demo(
        "EdgeGrow",
        "Generalized hover/stretch rail",
        &[
            snapshot_edge_grow(0.12),
            snapshot_edge_grow(0.55),
            snapshot_edge_grow(1.0),
        ],
    );

    print_demo(
        "GlyphCascade",
        "Glyph evolution landing on target text",
        &[
            snapshot_glyph_cascade(0.08),
            snapshot_glyph_cascade(0.50),
            snapshot_glyph_cascade(1.0),
        ],
    );
}

fn print_demo(title: &str, subtitle: &str, frames: &[String]) {
    println!("=== {title} ===");
    println!("{subtitle}\n");
    for (idx, frame) in frames.iter().enumerate() {
        println!("-- frame {} --", idx + 1);
        println!("{frame}");
    }
    println!();
}

fn snapshot_materialize(progress: f64) -> String {
    let width = 38;
    let height = 7;
    let source = text_card(
        width,
        height,
        "MATERIALIZE MASK",
        Color::rgb(220, 240, 255),
        Color::rgb(18, 26, 38),
        0,
    );
    let mut dest = blank_grid(width, height, Color::rgb(8, 12, 18));
    let options = CompositionOptions::default().with_mask(MaskSpec::Materialize {
        origin: RadialOrigin::Center,
        seed: 42,
        chunk_size: 1,
        noise: 0.18,
        soft_edge: true,
    });
    render_pipeline(
        &source,
        &mut dest,
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
    grid_to_string(&dest)
}

fn snapshot_edge_grow(progress: f64) -> String {
    let width = 30;
    let height = 5;
    let mut source = blank_grid(width, height, Color::rgb(14, 18, 26));
    draw_text(
        &mut source,
        2,
        2,
        "EDGE GROW LEFT",
        Color::rgb(230, 235, 245),
        Color::rgb(14, 18, 26),
    );
    let mut dest = blank_grid(width, height, Color::rgb(14, 18, 26));
    let options = CompositionOptions::default().with_filter(FilterSpec::EdgeGrow {
        rest_eighths: 1,
        peak_eighths: 14,
        edge: HoverBarPosition::Left,
        fill_color: ColorConfig::Rgb {
            r: 90,
            g: 220,
            b: 255,
        },
        bg_color: ColorConfig::Rgb {
            r: 14,
            g: 18,
            b: 26,
        },
        progress: BindableValue::static_f32(progress as f32),
        margin_width: 2,
    });
    render_pipeline(
        &source,
        &mut dest,
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
    grid_to_string(&dest)
}

fn snapshot_glyph_cascade(progress: f64) -> String {
    let width = 38;
    let height = 5;
    let effect = ContentEffect::GlyphCascade {
        alphabet: GlyphCascadeAlphabet::Circles,
        pattern: GlyphCascadePattern::Sequential,
        direction: DissolveDirection::LeftToRight,
        seed: 7,
        mode: GlyphCascadeMode::IntoTarget,
    };
    let transformed = effect.apply("GLYPH CASCADE INTO TARGET", progress);
    let mut source = blank_grid(width, height, Color::rgb(16, 18, 24));
    draw_text(
        &mut source,
        1,
        2,
        &transformed,
        Color::rgb(212, 240, 255),
        Color::rgb(16, 18, 24),
    );
    grid_to_string(&source)
}

fn text_card(
    width: usize,
    height: usize,
    text: &str,
    fg: Color,
    bg: Color,
    margin_x: usize,
) -> OwnedGrid {
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
    draw_text(&mut grid, 2 + margin_x, height / 2, text, fg, bg);
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
