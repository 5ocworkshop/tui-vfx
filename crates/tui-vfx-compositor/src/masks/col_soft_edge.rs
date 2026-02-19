// <FILE>tui-vfx-compositor/src/masks/col_soft_edge.rs</FILE> - <DESC>Soft-edge visibility helpers for masks</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>OAI review: extract shared soft-edge logic</WCTX>
// <CLOG>Initial extraction of soft-edge helpers from wipe/diamond/radial masks</CLOG>

/// Check visibility with optional soft edge blending.
///
/// For masks that reveal based on distance from a threshold, this provides
/// a consistent soft-edge implementation. With soft edge enabled, pixels
/// within `edge_width` beyond the threshold are also considered visible.
///
/// # Arguments
/// * `distance` - The distance metric for the pixel (e.g., position, manhattan distance)
/// * `threshold` - The reveal threshold based on progress
/// * `soft_edge` - Whether to apply soft edge blending
/// * `edge_width` - The width of the soft edge transition zone
///
/// # Returns
/// `true` if the pixel should be visible
#[inline]
pub fn is_visible_with_soft_edge(
    distance: f32,
    threshold: f32,
    soft_edge: bool,
    edge_width: f32,
) -> bool {
    if soft_edge {
        distance < threshold + edge_width
    } else {
        distance < threshold
    }
}

/// Calculate edge width as a proportion of a dimension.
///
/// Standard edge width is 10% of the reference dimension, with a minimum of 1.0.
#[inline]
pub fn calc_edge_width(dimension: f32) -> f32 {
    (dimension * 0.1).max(1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hard_edge_visible() {
        assert!(is_visible_with_soft_edge(4.0, 5.0, false, 1.0));
    }

    #[test]
    fn test_hard_edge_not_visible() {
        assert!(!is_visible_with_soft_edge(5.0, 5.0, false, 1.0));
        assert!(!is_visible_with_soft_edge(6.0, 5.0, false, 1.0));
    }

    #[test]
    fn test_soft_edge_extends_visibility() {
        // At threshold boundary, hard edge is not visible
        assert!(!is_visible_with_soft_edge(5.0, 5.0, false, 1.0));
        // But soft edge makes it visible (5.0 < 5.0 + 1.0)
        assert!(is_visible_with_soft_edge(5.0, 5.0, true, 1.0));
    }

    #[test]
    fn test_soft_edge_boundary() {
        // Just inside soft edge boundary
        assert!(is_visible_with_soft_edge(5.9, 5.0, true, 1.0));
        // At soft edge boundary (6.0 < 6.0 is false)
        assert!(!is_visible_with_soft_edge(6.0, 5.0, true, 1.0));
    }

    #[test]
    fn test_calc_edge_width() {
        assert_eq!(calc_edge_width(10.0), 1.0);
        assert_eq!(calc_edge_width(100.0), 10.0);
        // Minimum of 1.0
        assert_eq!(calc_edge_width(5.0), 1.0);
    }
}

// <FILE>tui-vfx-compositor/src/masks/col_soft_edge.rs</FILE> - <DESC>Soft-edge visibility helpers for masks</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
