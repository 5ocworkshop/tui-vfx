// <FILE>crates/tui-vfx-compost/src/runtime/cls_runtime_value_error.rs</FILE> - <DESC>Runtime value source diagnostics</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Runtime value errors distinguish unsupported fields from unavailable context values.</WCTX>
// <CLOG>0.2.0: MINOR — update diagnostics for real non-literal resolution.</CLOG>

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct RuntimeValueError {
    source_kind: &'static str,
    detail: String,
}

impl RuntimeValueError {
    pub(crate) fn unsupported(source_kind: &'static str) -> Self {
        Self {
            source_kind,
            detail: "source is not supported by the native runtime resolver".to_string(),
        }
    }

    pub(crate) fn unavailable(source_kind: &'static str, detail: impl Into<String>) -> Self {
        Self {
            source_kind,
            detail: detail.into(),
        }
    }

    pub(crate) fn reason(&self) -> String {
        format!(
            "runtime value resolver could not resolve `{}` value source: {}",
            self.source_kind, self.detail
        )
    }
}

// <FILE>crates/tui-vfx-compost/src/runtime/cls_runtime_value_error.rs</FILE> - <DESC>Runtime value source diagnostics</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
