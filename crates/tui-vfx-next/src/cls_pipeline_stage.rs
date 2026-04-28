// <FILE>crates/tui-vfx-next/src/cls_pipeline_stage.rs</FILE> - <DESC>Phase C ordered pipeline stage enum</DESC>
// <VERS>VERSION: 0.4.1</VERS>
// <WCTX>New kernel Phase D0 verifier fix: make pipeline stage enum wire shape strict.</WCTX>
// <CLOG>0.4.1: PATCH — add deny_unknown_fields to the schema-visible stage enum.
// 0.4.0: PATCH — add Serde/Schemars schema-reference readiness while preserving runtime behavior.
// 0.1.0: ADD — copy, dim, explicit-role, and glyph-rewrite stages for pipeline semantics tests.</CLOG>

use crate::{
    ApplyOutcome, CoordinateSpace, DimEffect, ExplicitRoleWriteEffect, PipelineSampler, RoleSpace,
    ScopeSpec, ShiftSampler, Surface, SurfaceEngine, fnc_rewrite_glyph_cell::rewrite_glyph_cell,
};

/// One tiny ordered pipeline stage.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum PipelineStage {
    /// Copy/sample cells and sampled-source roles from the stage read surface.
    Copy {
        /// Stable stage name used in diagnostics.
        name: String,
        /// Stage scope.
        scope: ScopeSpec,
        /// Stage sampler.
        sampler: PipelineSampler,
    },
    /// Visual-only dim stage that preserves destination roles.
    Dim {
        /// Stable stage name used in diagnostics.
        name: String,
        /// Stage scope.
        scope: ScopeSpec,
        /// Dim effect.
        effect: DimEffect,
    },
    /// Explicit role writer stage.
    ExplicitRoleWrite {
        /// Stable stage name used in diagnostics.
        name: String,
        /// Stage scope.
        scope: ScopeSpec,
        /// Writer effect.
        effect: ExplicitRoleWriteEffect,
    },
    /// Test-helper glyph rewrite stage scoped by the stage read surface.
    ReplaceGlyph {
        /// Stable stage name used in diagnostics.
        name: String,
        /// Stage scope.
        scope: ScopeSpec,
        /// Glyph to replace.
        from: char,
        /// Replacement glyph.
        to: char,
    },
}

impl PipelineStage {
    /// Create an identity copy stage.
    pub fn copy(name: impl Into<String>, scope: ScopeSpec) -> Self {
        Self::Copy {
            name: name.into(),
            scope,
            sampler: PipelineSampler::Identity,
        }
    }

    /// Create a sampled copy stage.
    pub fn copy_with_sampler(
        name: impl Into<String>,
        scope: ScopeSpec,
        sampler: ShiftSampler,
    ) -> Self {
        Self::Copy {
            name: name.into(),
            scope,
            sampler: sampler.into(),
        }
    }

    /// Create a dim stage.
    pub fn dim(name: impl Into<String>, scope: ScopeSpec, effect: DimEffect) -> Self {
        Self::Dim {
            name: name.into(),
            scope,
            effect,
        }
    }

    /// Create an explicit-role writer stage.
    pub fn explicit_role_write(
        name: impl Into<String>,
        scope: ScopeSpec,
        effect: ExplicitRoleWriteEffect,
    ) -> Self {
        Self::ExplicitRoleWrite {
            name: name.into(),
            scope,
            effect,
        }
    }

    /// Create a scoped glyph rewrite helper stage.
    pub fn replace_glyph(name: impl Into<String>, scope: ScopeSpec, from: char, to: char) -> Self {
        Self::ReplaceGlyph {
            name: name.into(),
            scope,
            from,
            to,
        }
    }

    /// Stable stage name.
    pub fn name(&self) -> &str {
        match self {
            Self::Copy { name, .. }
            | Self::Dim { name, .. }
            | Self::ExplicitRoleWrite { name, .. }
            | Self::ReplaceGlyph { name, .. } => name,
        }
    }

    /// Apply this stage from a read surface into a next write surface.
    pub fn apply(&self, read: &Surface, next: &mut Surface) -> ApplyOutcome {
        match self {
            Self::Copy { scope, sampler, .. } => {
                SurfaceEngine::copy_with_sampler(read, next, scope, sampler)
            }
            Self::Dim { scope, effect, .. } => SurfaceEngine::apply_dim(read, next, scope, *effect),
            Self::ExplicitRoleWrite { scope, effect, .. } => {
                SurfaceEngine::apply_explicit_role_write(next, scope, effect)
            }
            Self::ReplaceGlyph {
                scope, from, to, ..
            } => SurfaceEngine::apply_from_source(
                read,
                next,
                scope,
                CoordinateSpace::default(),
                RoleSpace::default(),
                |sampled_cell, _sampled_role| rewrite_glyph_cell(sampled_cell, *from, *to),
            ),
        }
    }
}

// <FILE>crates/tui-vfx-next/src/cls_pipeline_stage.rs</FILE> - <DESC>Phase C ordered pipeline stage enum</DESC>
// <VERS>END OF VERSION: 0.4.1</VERS>
