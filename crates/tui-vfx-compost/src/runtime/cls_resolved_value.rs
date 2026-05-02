// <FILE>crates/tui-vfx-compost/src/runtime/cls_resolved_value.rs</FILE> - <DESC>Resolved canonical value wrapper</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Runtime values may resolve to borrowed literals or context/fallback-owned values.</WCTX>
// <CLOG>0.2.0: MINOR — store resolved values as borrowed-or-owned data for non-literal sources.</CLOG>

use std::borrow::Cow;

use tui_vfx_contract::Value;

#[derive(Clone, Debug)]
pub(crate) struct ResolvedValue<'a> {
    value: Cow<'a, Value>,
}

impl<'a> ResolvedValue<'a> {
    pub(crate) fn literal(value: &'a Value) -> Self {
        Self {
            value: Cow::Borrowed(value),
        }
    }

    pub(crate) fn owned(value: Value) -> Self {
        Self {
            value: Cow::Owned(value),
        }
    }

    pub(crate) fn value(&self) -> &Value {
        self.value.as_ref()
    }
}

// <FILE>crates/tui-vfx-compost/src/runtime/cls_resolved_value.rs</FILE> - <DESC>Resolved canonical value wrapper</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
