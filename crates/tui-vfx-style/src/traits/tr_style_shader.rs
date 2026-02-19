// <FILE>tui-vfx-style/src/traits/tr_style_shader.rs</FILE> - <DESC>Trait for spatial style effects</DESC>
// <VERS>VERSION: 1.1.0</VERS>
// <WCTX>Pipeline stage inspection implementation</WCTX>
// <CLOG>Add name() method for inspector support</CLOG>

use super::ShaderContext;
use std::fmt::Debug;
use tui_vfx_types::Style;

/// A shader that calculates style based on spatial coordinates and time.
///
/// This allows for effects like "scanners", "shimmers", or "gradient maps"
/// that depend on where the cell is located relative to the widget or screen.
///
/// # Context Information
///
/// The `ShaderContext` provides:
/// - **Local coords** (`ctx.local_x`, `ctx.local_y`): Position within widget (0,0 = top-left)
/// - **Widget size** (`ctx.width`, `ctx.height`): Widget dimensions
/// - **Screen offset** (`ctx.screen_x`, `ctx.screen_y`): Widget position on screen
/// - **Animation** (`ctx.t`, `ctx.phase`): Progress and current phase
///
/// # Example
///
/// ```ignore
/// impl StyleShader for MyShader {
///     fn style_at(&self, ctx: &ShaderContext, base: Style) -> Style {
///         // Use local coords for widget-relative effect
///         let progress = ctx.local_x as f32 / ctx.width as f32;
///
///         // Or use screen coords for screen-space effect
///         let screen_x = ctx.screen_cell_x();
///
///         // Return modified style
///         base.fg(Color::rgb(progress as u8 * 255, 0, 0))
///     }
/// }
/// ```
pub trait StyleShader: Debug + Send + Sync {
    /// Calculate the style for a specific cell.
    ///
    /// # Arguments
    /// * `ctx` - Context containing local coords, widget size, screen offset, and animation state
    /// * `base` - The base style of the widget
    ///
    /// # Returns
    /// The computed style for this cell
    fn style_at(&self, ctx: &ShaderContext, base: Style) -> Style;

    /// Returns the name of this shader for debugging/inspection purposes.
    ///
    /// Default implementation uses Debug formatting, but implementations
    /// should override this with a simple static name for efficiency.
    fn name(&self) -> &'static str {
        "Unknown"
    }
}

// <FILE>tui-vfx-style/src/traits/tr_style_shader.rs</FILE> - <DESC>Trait for spatial style effects</DESC>
// <VERS>END OF VERSION: 1.1.0</VERS>
