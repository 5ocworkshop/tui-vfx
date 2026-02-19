// <FILE>tui-vfx-content/src/transformers/cls_wrap_indicator.rs</FILE>
// <DESC>Wraps text with prefix/suffix symbols based on progress</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Hover indicator effects implementation</WCTX>
// <CLOG>Initial implementation with progress-based prefix/suffix wrapping</CLOG>

use crate::traits::TextTransformer;
use mixed_signals::prelude::SignalContext;
use std::borrow::Cow;

/// Wraps text with prefix/suffix symbols based on progress.
///
/// At progress 0.0, returns the original text unchanged.
/// At progress 1.0, returns the text with full prefix and suffix applied.
/// Intermediate progress values show partial prefix/suffix (character by character).
///
/// # Examples
///
/// ```ignore
/// let wrap = WrapIndicator::new("» ".to_string(), " «".to_string());
///
/// // At progress 0.0
/// assert_eq!(wrap.transform("YES", 0.0, &ctx), "YES");
///
/// // At progress 0.5 (partial wrap)
/// assert_eq!(wrap.transform("YES", 0.5, &ctx), "»YES «");
///
/// // At progress 1.0
/// assert_eq!(wrap.transform("YES", 1.0, &ctx), "» YES «");
/// ```
pub struct WrapIndicator {
    /// Prefix to prepend (e.g., "» ")
    pub prefix: String,
    /// Suffix to append (e.g., " «")
    pub suffix: String,
}

impl WrapIndicator {
    /// Create a new WrapIndicator with the given prefix and suffix.
    pub fn new(prefix: String, suffix: String) -> Self {
        Self { prefix, suffix }
    }

    /// Create with common arrow brackets.
    pub fn arrows() -> Self {
        Self::new("» ".to_string(), " «".to_string())
    }

    /// Create with square brackets.
    pub fn brackets() -> Self {
        Self::new("[ ".to_string(), " ]".to_string())
    }

    /// Create with angle brackets.
    pub fn angles() -> Self {
        Self::new("< ".to_string(), " >".to_string())
    }
}

impl TextTransformer for WrapIndicator {
    fn transform<'a>(
        &self,
        target: &'a str,
        progress: f64,
        _signal_ctx: &SignalContext,
    ) -> Cow<'a, str> {
        if progress <= 0.0 {
            return Cow::Borrowed(target);
        }

        let progress = progress.min(1.0) as f32;

        // Calculate how many characters of prefix/suffix to show
        let prefix_chars: Vec<char> = self.prefix.chars().collect();
        let suffix_chars: Vec<char> = self.suffix.chars().collect();

        let prefix_len = prefix_chars.len();
        let suffix_len = suffix_chars.len();
        let total_wrap_chars = prefix_len + suffix_len;

        if total_wrap_chars == 0 {
            return Cow::Borrowed(target);
        }

        // At full progress, show complete wrap
        let chars_to_show = (total_wrap_chars as f32 * progress).ceil() as usize;

        // Distribute between prefix and suffix proportionally
        let prefix_show = chars_to_show.min(prefix_len);
        let suffix_show = chars_to_show.saturating_sub(prefix_len).min(suffix_len);

        let shown_prefix: String = prefix_chars[..prefix_show].iter().collect();
        let shown_suffix: String = suffix_chars[..suffix_show].iter().collect();

        Cow::Owned(format!("{}{}{}", shown_prefix, target, shown_suffix))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx() -> SignalContext {
        SignalContext::default()
    }

    #[test]
    fn zero_progress_unchanged() {
        let wrap = WrapIndicator::new("» ".to_string(), " «".to_string());
        let result = wrap.transform("YES", 0.0, &ctx());
        assert_eq!(result, "YES");
    }

    #[test]
    fn full_progress_complete_wrap() {
        let wrap = WrapIndicator::new("» ".to_string(), " «".to_string());
        let result = wrap.transform("YES", 1.0, &ctx());
        assert_eq!(result, "» YES «");
    }

    #[test]
    fn partial_progress_partial_wrap() {
        // "» " (2 chars) + " «" (2 chars) = 4 total
        let wrap = WrapIndicator::new("» ".to_string(), " «".to_string());

        // At 25% (1 char) - show just "»"
        let result = wrap.transform("YES", 0.25, &ctx());
        assert_eq!(result, "»YES");

        // At 50% (2 chars) - show "» " (full prefix)
        let result = wrap.transform("YES", 0.5, &ctx());
        assert_eq!(result, "» YES");

        // At 75% (3 chars) - show "» " + " "
        let result = wrap.transform("YES", 0.75, &ctx());
        assert_eq!(result, "» YES ");
    }

    #[test]
    fn arrows_preset() {
        let wrap = WrapIndicator::arrows();
        let result = wrap.transform("OK", 1.0, &ctx());
        assert_eq!(result, "» OK «");
    }

    #[test]
    fn brackets_preset() {
        let wrap = WrapIndicator::brackets();
        let result = wrap.transform("OK", 1.0, &ctx());
        assert_eq!(result, "[ OK ]");
    }

    #[test]
    fn angles_preset() {
        let wrap = WrapIndicator::angles();
        let result = wrap.transform("OK", 1.0, &ctx());
        assert_eq!(result, "< OK >");
    }

    #[test]
    fn empty_prefix_suffix() {
        let wrap = WrapIndicator::new(String::new(), String::new());
        let result = wrap.transform("TEXT", 1.0, &ctx());
        assert_eq!(result, "TEXT");
    }

    #[test]
    fn only_prefix() {
        let wrap = WrapIndicator::new("→ ".to_string(), String::new());
        let result = wrap.transform("Item", 1.0, &ctx());
        assert_eq!(result, "→ Item");
    }

    #[test]
    fn only_suffix() {
        let wrap = WrapIndicator::new(String::new(), " ←".to_string());
        let result = wrap.transform("Item", 1.0, &ctx());
        assert_eq!(result, "Item ←");
    }

    #[test]
    fn progress_clamped() {
        let wrap = WrapIndicator::new("» ".to_string(), " «".to_string());

        // Negative progress should be treated as 0
        let result = wrap.transform("X", -0.5, &ctx());
        assert_eq!(result, "X");

        // Progress > 1 should be clamped to 1
        let result = wrap.transform("X", 2.0, &ctx());
        assert_eq!(result, "» X «");
    }
}

// <FILE>tui-vfx-content/src/transformers/cls_wrap_indicator.rs</FILE>
// <DESC>Wraps text with prefix/suffix symbols based on progress</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
