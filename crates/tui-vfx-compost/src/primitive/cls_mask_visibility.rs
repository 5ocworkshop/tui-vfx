// <FILE>crates/tui-vfx-compost/src/primitive/cls_mask_visibility.rs</FILE> - <DESC>Mask primitive visibility output</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Mask runtime traits return visibility instead of mutating style, proving the domain-specific split.</WCTX>
// <CLOG>0.1.0: INIT — add clamped mask visibility scalar.</CLOG>

/// Visibility contribution returned by a mask primitive, clamped to `0.0..=1.0`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MaskVisibility(f32);

impl MaskVisibility {
    /// Fully hidden output.
    pub const HIDDEN: Self = Self(0.0);
    /// Fully visible output.
    pub const VISIBLE: Self = Self(1.0);

    /// Build a visibility value and clamp it into the supported range.
    pub fn new(value: f32) -> Self {
        Self(value.clamp(0.0, 1.0))
    }

    /// Return the clamped visibility scalar.
    pub fn as_f32(self) -> f32 {
        self.0
    }
}

// <FILE>crates/tui-vfx-compost/src/primitive/cls_mask_visibility.rs</FILE> - <DESC>Mask primitive visibility output</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
