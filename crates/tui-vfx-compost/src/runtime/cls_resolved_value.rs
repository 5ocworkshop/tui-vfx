// <FILE>crates/tui-vfx-compost/src/runtime/cls_resolved_value.rs</FILE> - <DESC>Resolved canonical value wrapper</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Resolved values keep ValueSource matching out of primitive validators.</WCTX>
// <CLOG>0.1.0: INIT — add borrowed resolved value wrapper.</CLOG>

use tui_vfx_contract::Value;

#[derive(Clone, Copy, Debug)]
pub(crate) struct ResolvedValue<'a> {
    value: &'a Value,
}

impl<'a> ResolvedValue<'a> {
    pub(crate) fn literal(value: &'a Value) -> Self {
        Self { value }
    }

    pub(crate) fn value(&self) -> &'a Value {
        self.value
    }
}

// <FILE>crates/tui-vfx-compost/src/runtime/cls_resolved_value.rs</FILE> - <DESC>Resolved canonical value wrapper</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
