// <FILE>crates/tui-vfx-compost/src/primitive/cls_source_surface.rs</FILE> - <DESC>Source runtime materialization output</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>SourceRuntime materializes SemanticScene surfaces without routing through legacy source DTOs.</WCTX>
// <CLOG>0.1.0: INIT — add SourceSurface wrapper around the v3.1 semantic scene foundation type.</CLOG>

use tui_vfx_types::SemanticScene;

/// Materialized semantic surface returned by a source runtime.
#[derive(Clone, Debug)]
pub struct SourceSurface {
    scene: SemanticScene,
}

impl SourceSurface {
    /// Build a source surface from a semantic scene.
    pub fn new(scene: SemanticScene) -> Self {
        Self { scene }
    }

    /// Borrow the materialized semantic scene.
    pub fn scene(&self) -> &SemanticScene {
        &self.scene
    }

    /// Consume the wrapper and return the materialized semantic scene.
    pub fn into_scene(self) -> SemanticScene {
        self.scene
    }
}

// <FILE>crates/tui-vfx-compost/src/primitive/cls_source_surface.rs</FILE> - <DESC>Source runtime materialization output</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
