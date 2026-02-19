// <FILE>tui-vfx-compositor/src/traits/mask.rs</FILE>
// <DESC>Trait for visibility testing</DESC>
// <VERS>VERSION: 1.0.0</VERS>

pub trait Mask {
    /// Determines if a cell at (x, y) is visible given the total dimensions (w, h) and progress t.
    fn is_visible(&self, x: u16, y: u16, w: u16, h: u16, progress: f64) -> bool;
}
