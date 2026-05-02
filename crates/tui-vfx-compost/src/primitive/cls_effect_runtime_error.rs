// <FILE>crates/tui-vfx-compost/src/primitive/cls_effect_runtime_error.rs</FILE> - <DESC>Primitive runtime error wrapper</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Domain runtime traits need a narrow error type before they are wired into render::RenderError.</WCTX>
// <CLOG>0.1.0: INIT — add primitive runtime error with display/error impls.</CLOG>

/// Error returned by primitive runtime implementations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectRuntimeError {
    message: String,
}

impl EffectRuntimeError {
    /// Build a runtime error from a human-readable message.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    /// Borrow the runtime error message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for EffectRuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for EffectRuntimeError {}

// <FILE>crates/tui-vfx-compost/src/primitive/cls_effect_runtime_error.rs</FILE> - <DESC>Primitive runtime error wrapper</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
