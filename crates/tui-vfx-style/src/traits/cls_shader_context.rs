// <FILE>tui-vfx-style/src/types/cls_shader_context.rs</FILE> - <DESC>Context passed to StyleShader for spatial effects</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Adding screen-space context to shaders</WCTX>
// <CLOG>Initial implementation with local coords, screen offset, and animation state</CLOG>

use mixed_signals::traits::Phase;

/// Context passed to StyleShader implementations for spatial effects.
///
/// This struct provides shaders with complete information about their rendering context,
/// including both local (widget-relative) and screen-absolute coordinates.
///
/// # Coordinate Systems
///
/// - **Local coordinates** (`local_x`, `local_y`): Position within the widget (0,0 = top-left of widget)
/// - **Screen coordinates**: `screen_x + local_x`, `screen_y + local_y` gives absolute screen position
///
/// # Use Cases
///
/// - **Widget-relative effects**: Use `local_x`, `local_y` for effects like highlights, sweeps
/// - **Screen-space effects**: Use screen coords for effects that span multiple widgets or align to screen edges
/// - **Phase-aware effects**: Use `phase` to vary behavior during enter/dwell/exit
#[derive(Debug, Clone, Copy)]
pub struct ShaderContext {
    /// Local X coordinate within widget (0 = left edge of widget)
    pub local_x: u16,
    /// Local Y coordinate within widget (0 = top edge of widget)
    pub local_y: u16,
    /// Widget width in cells
    pub width: u16,
    /// Widget height in cells
    pub height: u16,
    /// Screen X offset - widget's left edge in absolute screen coordinates
    pub screen_x: u16,
    /// Screen Y offset - widget's top edge in absolute screen coordinates
    pub screen_y: u16,
    /// Animation progress (0.0 to 1.0) - phase-based or loop time
    pub t: f64,
    /// Current animation phase (Entering/Dwelling/Exiting/Finished)
    pub phase: Option<Phase>,
}

impl ShaderContext {
    /// Create a new shader context.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        local_x: u16,
        local_y: u16,
        width: u16,
        height: u16,
        screen_x: u16,
        screen_y: u16,
        t: f64,
        phase: Option<Phase>,
    ) -> Self {
        Self {
            local_x,
            local_y,
            width,
            height,
            screen_x,
            screen_y,
            t,
            phase,
        }
    }

    /// Get absolute screen X coordinate for this cell.
    #[inline]
    pub fn screen_cell_x(&self) -> u16 {
        self.screen_x.saturating_add(self.local_x)
    }

    /// Get absolute screen Y coordinate for this cell.
    #[inline]
    pub fn screen_cell_y(&self) -> u16 {
        self.screen_y.saturating_add(self.local_y)
    }

    /// Get normalized local X position (0.0 = left, 1.0 = right).
    #[inline]
    pub fn normalized_x(&self) -> f32 {
        if self.width > 0 {
            self.local_x as f32 / self.width as f32
        } else {
            0.0
        }
    }

    /// Get normalized local Y position (0.0 = top, 1.0 = bottom).
    #[inline]
    pub fn normalized_y(&self) -> f32 {
        if self.height > 0 {
            self.local_y as f32 / self.height as f32
        } else {
            0.0
        }
    }

    /// Check if currently in entering/start phase.
    #[inline]
    pub fn is_entering(&self) -> bool {
        matches!(self.phase, Some(Phase::Start))
    }

    /// Check if currently in dwelling/active phase.
    #[inline]
    pub fn is_dwelling(&self) -> bool {
        matches!(self.phase, Some(Phase::Active))
    }

    /// Check if currently in exiting/end phase.
    #[inline]
    pub fn is_exiting(&self) -> bool {
        matches!(self.phase, Some(Phase::End))
    }
}

impl Default for ShaderContext {
    fn default() -> Self {
        Self {
            local_x: 0,
            local_y: 0,
            width: 0,
            height: 0,
            screen_x: 0,
            screen_y: 0,
            t: 0.0,
            phase: None,
        }
    }
}

// <FILE>tui-vfx-style/src/types/cls_shader_context.rs</FILE> - <DESC>Context passed to StyleShader for spatial effects</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
