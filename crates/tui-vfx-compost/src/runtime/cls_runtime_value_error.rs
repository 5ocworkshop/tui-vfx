// <FILE>crates/tui-vfx-compost/src/runtime/cls_runtime_value_error.rs</FILE> - <DESC>Runtime value source rejection diagnostics</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Runtime value errors give unsupported ValueSource variants one consistent diagnostic.</WCTX>
// <CLOG>0.1.0: INIT — add runtime value resolver error.</CLOG>

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct RuntimeValueError {
    source_kind: &'static str,
}

impl RuntimeValueError {
    pub(crate) fn unsupported(source_kind: &'static str) -> Self {
        Self { source_kind }
    }

    pub(crate) fn reason(&self) -> String {
        format!(
            "runtime value resolver currently supports literal values only; `{}` value sources require the runtime resolver substrate to bind parameters, signals, graph values, sampled fields, lifecycle progress, or clocks",
            self.source_kind
        )
    }
}

// <FILE>crates/tui-vfx-compost/src/runtime/cls_runtime_value_error.rs</FILE> - <DESC>Runtime value source rejection diagnostics</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
