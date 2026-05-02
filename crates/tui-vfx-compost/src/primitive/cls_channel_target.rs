// <FILE>crates/tui-vfx-compost/src/primitive/cls_channel_target.rs</FILE> - <DESC>Shared foreground/background channel target primitive helper</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 0.5 commonality extraction: many primitives route effects to foreground, background, or both via canonical channelTarget.</WCTX>
// <CLOG>0.1.0: INIT — add canonical channel target enum and descriptor allowed-values helpers.</CLOG>

/// Canonical foreground/background color channel target used by color primitives.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChannelTarget {
    /// Apply to both foreground and background.
    Both,
    /// Apply to foreground only.
    Foreground,
    /// Apply to background only.
    Background,
}

impl ChannelTarget {
    /// Descriptor/default string for the target.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Both => "both",
            Self::Foreground => "foreground",
            Self::Background => "background",
        }
    }

    /// Return true when the foreground channel is selected.
    pub const fn affects_foreground(self) -> bool {
        matches!(self, Self::Both | Self::Foreground)
    }

    /// Return true when the background channel is selected.
    pub const fn affects_background(self) -> bool {
        matches!(self, Self::Both | Self::Background)
    }

    /// Allowed descriptor values in canonical order.
    pub fn allowed_values() -> Vec<String> {
        [Self::Both, Self::Foreground, Self::Background]
            .into_iter()
            .map(|target| target.as_str().to_string())
            .collect()
    }
}

// <FILE>crates/tui-vfx-compost/src/primitive/cls_channel_target.rs</FILE> - <DESC>Shared foreground/background channel target primitive helper</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
