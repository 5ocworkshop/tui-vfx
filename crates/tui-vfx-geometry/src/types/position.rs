// <FILE>tui-vfx-geometry/src/types/position.rs</FILE> - <DESC>Signed position and rectangle types for offscreen motion</DESC>
// <VERS>VERSION: 1.0.0 - 2025-12-17T00:00:00Z</VERS>
// <WCTX>PRD_001 Anchoring & offscreen support</WCTX>
// <CLOG>Added signed Position and SignedRect</CLOG>

use serde::{Deserialize, Serialize};

/// A signed 2D position in terminal grid space.
///
/// This intentionally supports negative coordinates to allow offscreen motion.
#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    tui_vfx_core::ConfigSchema,
)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Converts to a Ratatui position by clamping negative values to 0.
    pub fn to_ratatui_clamped(self) -> tui_vfx_types::Point {
        tui_vfx_types::Point::new(
            self.x.clamp(0, u16::MAX as i32) as u16,
            self.y.clamp(0, u16::MAX as i32) as u16,
        )
    }
}

impl From<(i32, i32)> for Position {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

impl From<tui_vfx_types::Point> for Position {
    fn from(pos: tui_vfx_types::Point) -> Self {
        Self {
            x: pos.x as i32,
            y: pos.y as i32,
        }
    }
}

/// A signed rectangle that can represent offscreen areas.
#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    tui_vfx_core::ConfigSchema,
)]
pub struct SignedRect {
    pub x: i32,
    pub y: i32,
    pub width: u16,
    pub height: u16,
}

impl SignedRect {
    pub const fn new(x: i32, y: i32, width: u16, height: u16) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub const fn right(self) -> i32 {
        self.x + self.width as i32
    }

    pub const fn bottom(self) -> i32 {
        self.y + self.height as i32
    }

    /// Converts to a Ratatui `Rect` by clamping x/y to 0 and keeping size.
    pub fn to_ratatui_clamped(self) -> tui_vfx_types::Rect {
        tui_vfx_types::Rect::new(
            self.x.clamp(0, u16::MAX as i32) as u16,
            self.y.clamp(0, u16::MAX as i32) as u16,
            self.width,
            self.height,
        )
    }
}

impl From<tui_vfx_types::Rect> for SignedRect {
    fn from(rect: tui_vfx_types::Rect) -> Self {
        Self {
            x: rect.x as i32,
            y: rect.y as i32,
            width: rect.width,
            height: rect.height,
        }
    }
}

// <FILE>tui-vfx-geometry/src/types/position.rs</FILE> - <DESC>Signed position and rectangle types for offscreen motion</DESC>
// <VERS>END OF VERSION: 1.0.0 - 2025-12-17T00:00:00Z</VERS>
