// <FILE>crates/tui-vfx-compost/src/primitive/cls_effect_runtime_context.rs</FILE> - <DESC>Primitive runtime sample context</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Primitive runtime methods need stable sample, bounds, and per-cell coordinates without exposing compost's private RuntimeContext internals.</WCTX>
// <CLOG>0.1.0: INIT — add public primitive runtime context wrapper.</CLOG>

use crate::SampleContext;

/// Per-sample and per-cell context supplied to primitive runtime implementations.
#[derive(Clone, Copy, Debug)]
pub struct EffectRuntimeContext<'a> {
    sample: &'a SampleContext,
    local_x: u16,
    local_y: u16,
    width: u16,
    height: u16,
    seed: Option<u64>,
}

impl<'a> EffectRuntimeContext<'a> {
    /// Build a context for a primitive evaluation sample.
    pub fn new(
        sample: &'a SampleContext,
        local_x: u16,
        local_y: u16,
        width: u16,
        height: u16,
    ) -> Self {
        Self {
            sample,
            local_x,
            local_y,
            width,
            height,
            seed: None,
        }
    }

    /// Attach a deterministic seed for primitives that declare seeded determinism.
    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Borrow the render sample context.
    pub fn sample(&self) -> &'a SampleContext {
        self.sample
    }

    /// Return the cell-local x coordinate.
    pub fn local_x(&self) -> u16 {
        self.local_x
    }

    /// Return the cell-local y coordinate.
    pub fn local_y(&self) -> u16 {
        self.local_y
    }

    /// Return the source/evaluation width.
    pub fn width(&self) -> u16 {
        self.width
    }

    /// Return the source/evaluation height.
    pub fn height(&self) -> u16 {
        self.height
    }

    /// Return the optional deterministic seed.
    pub fn seed(&self) -> Option<u64> {
        self.seed
    }
}

// <FILE>crates/tui-vfx-compost/src/primitive/cls_effect_runtime_context.rs</FILE> - <DESC>Primitive runtime sample context</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
