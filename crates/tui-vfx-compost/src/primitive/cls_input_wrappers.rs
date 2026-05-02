// <FILE>crates/tui-vfx-compost/src/primitive/cls_input_wrappers.rs</FILE> - <DESC>Primitive descriptor input wrapper markers</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Literal vs bindable is descriptor metadata only; runtime scheduling is resolved from ValueSource kind and RuntimeMutability.</WCTX>
// <CLOG>0.1.0: INIT — add lightweight wrapper markers for future PrimitiveInputs derive output.</CLOG>

/// Descriptor marker for an input that must be supplied as a literal/default value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Literal<T>(pub T);

/// Descriptor marker for an input that may be driven by an authored value source.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Bindable<T>(pub T);

impl<T> Literal<T> {
    /// Return the wrapped value.
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> Bindable<T> {
    /// Return the wrapped value.
    pub fn into_inner(self) -> T {
        self.0
    }
}

// <FILE>crates/tui-vfx-compost/src/primitive/cls_input_wrappers.rs</FILE> - <DESC>Primitive descriptor input wrapper markers</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
