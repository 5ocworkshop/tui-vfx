// <FILE>tui-vfx-content/src/transformers/cls_mirror.rs</FILE> - <DESC>Mirror transformer for reversed text display</DESC>
// <VERS>VERSION: 2.0.0</VERS>
// <WCTX>feat-20251224-155211: Signal-driven content effects</WCTX>
// <CLOG>BREAKING: Updated transform() signature to accept SignalContext parameter</CLOG>

use crate::traits::TextTransformer;
use crate::types::MirrorAxis;
use mixed_signals::prelude::SignalContext;
use std::borrow::Cow;

/// Mirror transformer that reverses text horizontally or vertically.
///
/// Horizontal mirroring reverses character order within each line,
/// creating the effect of viewing text from behind a transparent screen.
///
/// Vertical mirroring reverses line order, flipping top to bottom.
#[derive(Debug, Clone)]
pub struct Mirror {
    axis: MirrorAxis,
}

impl Mirror {
    pub fn new(axis: MirrorAxis) -> Self {
        Self { axis }
    }

    pub fn horizontal() -> Self {
        Self::new(MirrorAxis::Horizontal)
    }

    pub fn vertical() -> Self {
        Self::new(MirrorAxis::Vertical)
    }

    /// Reverse characters in a single line, preserving grapheme clusters.
    fn reverse_line(line: &str) -> String {
        use unicode_segmentation::UnicodeSegmentation;
        line.graphemes(true).rev().collect()
    }
}

impl Default for Mirror {
    fn default() -> Self {
        Self {
            axis: MirrorAxis::Horizontal,
        }
    }
}

impl TextTransformer for Mirror {
    fn transform<'a>(
        &self,
        target: &'a str,
        progress: f64,
        _signal_ctx: &SignalContext,
    ) -> Cow<'a, str> {
        // At progress >= 1.0, show normal text (animation complete)
        // At progress < 1.0, show mirrored text
        // This allows the "flip" effect: mirrored -> squeeze -> normal
        if progress >= 1.0 {
            return Cow::Borrowed(target);
        }

        match self.axis {
            MirrorAxis::Horizontal => {
                // Reverse each line's character order
                let result: String = target
                    .lines()
                    .map(Self::reverse_line)
                    .collect::<Vec<_>>()
                    .join("\n");
                Cow::Owned(result)
            }
            MirrorAxis::Vertical => {
                // Reverse line order
                let lines: Vec<&str> = target.lines().collect();
                let result = lines.into_iter().rev().collect::<Vec<_>>().join("\n");
                Cow::Owned(result)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_horizontal_mirror_simple() {
        let mirror = Mirror::horizontal();
        let result = mirror.transform("HELLO", 0.5, &SignalContext::default());
        assert_eq!(result, "OLLEH");
    }

    #[test]
    fn test_horizontal_mirror_multiline() {
        let mirror = Mirror::horizontal();
        let result = mirror.transform("ABC\nDEF", 0.5, &SignalContext::default());
        assert_eq!(result, "CBA\nFED");
    }

    #[test]
    fn test_vertical_mirror() {
        let mirror = Mirror::vertical();
        let result = mirror.transform("LINE1\nLINE2\nLINE3", 0.5, &SignalContext::default());
        assert_eq!(result, "LINE3\nLINE2\nLINE1");
    }

    #[test]
    fn test_mirror_at_full_progress_returns_original() {
        let mirror = Mirror::horizontal();
        let result = mirror.transform("HELLO", 1.0, &SignalContext::default());
        assert_eq!(result, "HELLO");
    }

    #[test]
    fn test_mirror_unicode() {
        let mirror = Mirror::horizontal();
        // Test with emoji/unicode
        let result = mirror.transform("A🎉B", 0.5, &SignalContext::default());
        assert_eq!(result, "B🎉A");
    }
}

// <FILE>tui-vfx-content/src/transformers/cls_mirror.rs</FILE> - <DESC>Mirror transformer for reversed text display</DESC>
// <VERS>END OF VERSION: 2.0.0</VERS>
