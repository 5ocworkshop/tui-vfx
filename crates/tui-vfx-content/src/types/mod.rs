// <FILE>tui-vfx-content/src/types/mod.rs</FILE> - <DESC>Types module</DESC>
// <VERS>VERSION: 1.12.0</VERS>
// <WCTX>Expose public Odometer schema enums for structured tile-roll recipes.</WCTX>
// <CLOG>1.12.0: re-export OdometerDirection and OdometerTravel.</CLOG>

pub mod cls_content_effect;
pub mod cls_dissolve_config;
pub mod cls_glyph_cascade;
pub mod cls_mirror_axis;
pub mod cls_morph_config;
pub mod cls_scramble_charset;
pub mod cls_slide_shift_flow_mode;
pub mod cls_slide_shift_line_mode;
pub mod cls_typewriter_cursor;
pub mod fnc_apply_content_effect;

pub use crate::cell_motion::{
    CellCollisionMode, CellMotionAffect, CellMotionCoord, CellMotionOptions, CellMotionPhase,
    CellMotionPhaseSpec, CellMotionScope, CellMotionSpec, CellMotionTiming, CellMotionVisibility,
    CellPlacement, CellPlacementBasis, CellStagger, CellStaggerAxis, CellStaggerDirection,
    CellVisibilityMode, apply_cell_motion,
};
pub use crate::glyph_particles::{
    GlyphParticleEmitterSpec, GlyphParticleResult, GlyphParticleStats, ParticleConcurrency,
    ParticleEndBehavior, emit_glyph_particles,
};
pub use cls_content_effect::{ContentEffect, OdometerDirection, OdometerTravel};
pub use cls_dissolve_config::{DissolveDirection, DissolvePattern, DissolveReplacement};
pub use cls_glyph_cascade::{GlyphCascadeAlphabet, GlyphCascadeMode, GlyphCascadePattern};
pub use cls_mirror_axis::MirrorAxis;
pub use cls_morph_config::{MorphDirection, MorphProgression};
pub use cls_scramble_charset::ScrambleCharset;
pub use cls_slide_shift_flow_mode::SlideShiftFlowMode;
pub use cls_slide_shift_line_mode::SlideShiftLineMode;
pub use cls_typewriter_cursor::TypewriterCursor;

// <FILE>tui-vfx-content/src/types/mod.rs</FILE> - <DESC>Types module</DESC>
// <VERS>END OF VERSION: 1.12.0</VERS>
