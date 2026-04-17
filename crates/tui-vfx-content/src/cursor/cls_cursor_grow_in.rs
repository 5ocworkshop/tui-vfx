// <FILE>tui-vfx-content/src/cursor/cls_cursor_grow_in.rs</FILE> - <DESC>GrowIn config for Cursor primitive</DESC>
// <VERS>VERSION: 0.1.1</VERS>
// <WCTX>feat/cursor-primitive T31: clippy clean-up — derive Default on GrowInMode and GrowDirection using #[default] attribute</WCTX>
// <CLOG>PATCH: derive Default on GrowInMode (Never) and GrowDirection (Up); remove manual impls</CLOG>

use mixed_signals::prelude::SignalOrFloat;
use serde::{Deserialize, Serialize};

/// Controls when the grow-in animation fires.
#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum GrowInMode {
    /// Never animate — cursor snaps to visible on show. (Default.)
    #[default]
    Never,
    /// Animate on the first 0→1 visibility transition per `CursorState` lifetime.
    Once,
    /// Animate on every 0→1 visibility transition. **Warning:** if blink is on,
    /// this fires every unblink, producing a wobbly cursor.
    EveryShow,
}

/// Direction of the grow-in animation for block cursors.
///
/// Non-block cursors (e.g. `|`, `_`, `◆`) ignore direction and animate alpha only.
#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum GrowDirection {
    /// Grow from bottom up (`▁▂▃▄▅▆▇█`). (Default.)
    #[default]
    Up,
    /// Grow from top down (`▔▀▇...█`).
    Down,
    /// Expand from middle outwards (`▄ → ▆ → █`).
    Center,
}

/// Grow-in animation config for a [`crate::cursor::Cursor`].
///
/// All fields default to a no-op: [`GrowInMode::Never`] + zero durations.
/// Calling [`GrowIn::default`] produces the same static-cursor behavior as
/// a cursor with no animation.
///
/// # Example
///
/// ```
/// use tui_vfx_content::cursor::{GrowIn, GrowInMode, GrowDirection};
/// use mixed_signals::prelude::SignalOrFloat;
///
/// let static_cursor = GrowIn::default();              // no animation
/// let animated = GrowIn {                              // 200ms bottom-up grow
///     mode: GrowInMode::Once,
///     direction: GrowDirection::Up,
///     duration_ms: SignalOrFloat::Static(200.0),
///     ..GrowIn::default()
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(default)]
pub struct GrowIn {
    pub mode: GrowInMode,
    pub direction: GrowDirection,
    /// Duration in ms. `0` = instant (do-nothing default).
    pub duration_ms: SignalOrFloat,
    /// Duration in ms for grow-out on hide. `0` = instant (default).
    pub grow_out_ms: SignalOrFloat,
    /// Easing curve sampled with `t in 0..1`, returning eased progress in `0..1`.
    /// Default `Static(1.0)` is treated as linear by `fnc_render_cursor`.
    pub curve: SignalOrFloat,
}

impl Default for GrowIn {
    fn default() -> Self {
        Self {
            mode: GrowInMode::Never,
            direction: GrowDirection::Up,
            duration_ms: SignalOrFloat::Static(0.0),
            grow_out_ms: SignalOrFloat::Static(0.0),
            curve: SignalOrFloat::Static(1.0),
        }
    }
}

impl GrowIn {
    /// Explicit alias for [`GrowIn::default`] — renders a static cursor.
    pub fn noop() -> Self {
        Self::default()
    }
}

// <FILE>tui-vfx-content/src/cursor/cls_cursor_grow_in.rs</FILE> - <DESC>GrowIn config for Cursor primitive</DESC>
// <VERS>END OF VERSION: 0.1.1</VERS>
