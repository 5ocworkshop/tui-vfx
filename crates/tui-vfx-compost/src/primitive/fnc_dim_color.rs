// <FILE>crates/tui-vfx-compost/src/primitive/fnc_dim_color.rs</FILE> - <DESC>Shared color dimming helper for primitive runtimes</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 0.5 commonality extraction: lock legacy-compatible color dim rounding once before filter ports copy it.</WCTX>
// <CLOG>0.1.0: INIT — add clamped dim helper that preserves alpha while rounding RGB channels.</CLOG>

use tui_vfx_types::Color;

/// Dim RGB channels by `amount` while preserving alpha.
///
/// `amount` is clamped to `0.0..=1.0`, where `0.0` is unchanged and `1.0` is black.
pub fn dim_color(color: Color, amount: f32) -> Color {
    let amount = amount.clamp(0.0, 1.0);
    let scale = 1.0 - amount;
    Color::new(
        (color.r as f32 * scale).round() as u8,
        (color.g as f32 * scale).round() as u8,
        (color.b as f32 * scale).round() as u8,
        color.a,
    )
}

// <FILE>crates/tui-vfx-compost/src/primitive/fnc_dim_color.rs</FILE> - <DESC>Shared color dimming helper for primitive runtimes</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
