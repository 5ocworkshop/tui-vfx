// <FILE>crates/tui-vfx-compost/src/runtime/cls_runtime_context.rs</FILE> - <DESC>Native value resolver context</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Runtime validation starts with load-time context and leaves runtime bindings explicit.</WCTX>
// <CLOG>0.1.0: INIT — add runtime context placeholder for future parameters/signals/graph values.</CLOG>

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct RuntimeContext;

impl RuntimeContext {
    pub(crate) fn load_time() -> Self {
        Self
    }
}

// <FILE>crates/tui-vfx-compost/src/runtime/cls_runtime_context.rs</FILE> - <DESC>Native value resolver context</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
