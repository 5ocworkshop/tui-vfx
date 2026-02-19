// <FILE>tui-vfx-compositor/src/types/cls_hover_bar_position.rs</FILE>
// <DESC>Position enum for hover bar indicator effects</DESC>
// <VERS>VERSION: 1.1.1</VERS>
// <WCTX>Resolve clippy clone_on_copy warning in tests</WCTX>
// <CLOG>Replace unnecessary Copy clone in hover bar position tests</CLOG>

/// Position for hover bar indicators relative to content.
///
/// Used by `FilterSpec::HoverBar` and `FilterSpec::DotIndicator` to control
/// which side of the content the indicator appears on.
///
/// - `Left`/`Right`: Vertical bars using left-aligned partial blocks (▏▎▍▌▋▊▉█)
/// - `Top`/`Bottom`: Horizontal bars using lower partial blocks (▁▂▃▄▅▆▇█)
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum HoverBarPosition {
    /// Indicator appears on the left side of content (vertical bar)
    #[default]
    Left,
    /// Indicator appears on the right side of content (vertical bar)
    Right,
    /// Indicator appears on the top edge of content (horizontal bar)
    Top,
    /// Indicator appears on the bottom edge of content (horizontal bar)
    Bottom,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_left() {
        assert_eq!(HoverBarPosition::default(), HoverBarPosition::Left);
    }

    #[test]
    fn serde_roundtrip() {
        let positions = [
            (HoverBarPosition::Left, "\"left\""),
            (HoverBarPosition::Right, "\"right\""),
            (HoverBarPosition::Top, "\"top\""),
            (HoverBarPosition::Bottom, "\"bottom\""),
        ];

        for (pos, expected_json) in positions {
            let json = serde_json::to_string(&pos).unwrap();
            assert_eq!(json, expected_json);
            let back: HoverBarPosition = serde_json::from_str(&json).unwrap();
            assert_eq!(back, pos);
        }
    }

    #[test]
    fn equality() {
        assert_eq!(HoverBarPosition::Left, HoverBarPosition::Left);
        assert_eq!(HoverBarPosition::Right, HoverBarPosition::Right);
        assert_eq!(HoverBarPosition::Top, HoverBarPosition::Top);
        assert_eq!(HoverBarPosition::Bottom, HoverBarPosition::Bottom);
        assert_ne!(HoverBarPosition::Left, HoverBarPosition::Right);
        assert_ne!(HoverBarPosition::Top, HoverBarPosition::Bottom);
    }

    #[test]
    fn clone_and_copy() {
        let pos = HoverBarPosition::Right;
        let cloned = pos;
        let copied = pos;
        assert_eq!(pos, cloned);
        assert_eq!(pos, copied);
    }
}

// <FILE>tui-vfx-compositor/src/types/cls_hover_bar_position.rs</FILE>
// <DESC>Position enum for hover bar indicator effects</DESC>
// <VERS>END OF VERSION: 1.1.1</VERS>
