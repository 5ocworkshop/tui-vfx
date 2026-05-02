// <FILE>crates/tui-vfx-compost/src/primitive/cls_effect_registry.rs</FILE> - <DESC>Rust-owned primitive descriptor/runtime registry</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 0 registry keeps one descriptor view plus per-domain runtime id tables so codegen and runtime dispatch can evolve independently.</WCTX>
// <CLOG>0.1.0: INIT — add registry installation, domain validation, runtime-id tables, and descriptor-pack export.</CLOG>

use std::collections::{BTreeMap, BTreeSet};

use tui_vfx_contract::{
    DescriptorPack, DescriptorPackId, EffectDescriptor, EffectId, SourceDescriptor, SourceId,
};

use super::{
    CellShaderRuntime, ContentTransformRuntime, CoordinateSamplerRuntime, EffectPrimitive,
    EffectRuntimeKind, FrameFilterRuntime, MaskRuntime, PrimitiveRegistryError, SourceRuntime,
};

/// Registry of Rust-owned primitive descriptors and their domain runtime tables.
#[derive(Clone, Debug, Default)]
pub struct EffectRegistry {
    effects: BTreeMap<EffectId, EffectDescriptor>,
    sources: BTreeMap<SourceId, SourceDescriptor>,
    cell_shader_runtimes: BTreeSet<EffectId>,
    frame_filter_runtimes: BTreeSet<EffectId>,
    coordinate_sampler_runtimes: BTreeSet<EffectId>,
    mask_runtimes: BTreeSet<EffectId>,
    content_transform_runtimes: BTreeSet<EffectId>,
    source_runtimes: BTreeSet<SourceId>,
}

impl EffectRegistry {
    /// Construct an empty primitive registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a descriptor-only effect, used for future/unsupported domains during migration.
    pub fn install_effect_descriptor(
        &mut self,
        descriptor: EffectDescriptor,
    ) -> Result<(), PrimitiveRegistryError> {
        self.install_effect_descriptor_with_runtime(descriptor, EffectRuntimeKind::DescriptorOnly)
    }

    /// Register a cell-shader primitive runtime.
    pub fn install_cell_shader<P>(&mut self) -> Result<(), PrimitiveRegistryError>
    where
        P: CellShaderRuntime,
    {
        self.install_effect::<P>(EffectRuntimeKind::CellShader)
    }

    /// Register a frame-filter primitive runtime.
    pub fn install_frame_filter<P>(&mut self) -> Result<(), PrimitiveRegistryError>
    where
        P: FrameFilterRuntime,
    {
        self.install_effect::<P>(EffectRuntimeKind::FrameFilter)
    }

    /// Register a coordinate-sampler primitive runtime.
    pub fn install_coordinate_sampler<P>(&mut self) -> Result<(), PrimitiveRegistryError>
    where
        P: CoordinateSamplerRuntime,
    {
        self.install_effect::<P>(EffectRuntimeKind::CoordinateSampler)
    }

    /// Register a mask primitive runtime.
    pub fn install_mask<P>(&mut self) -> Result<(), PrimitiveRegistryError>
    where
        P: MaskRuntime,
    {
        self.install_effect::<P>(EffectRuntimeKind::Mask)
    }

    /// Register a content-transform primitive runtime.
    pub fn install_content_transform<P>(&mut self) -> Result<(), PrimitiveRegistryError>
    where
        P: ContentTransformRuntime,
    {
        self.install_effect::<P>(EffectRuntimeKind::ContentTransform)
    }

    /// Register a descriptor-only source.
    pub fn install_source_descriptor(
        &mut self,
        descriptor: SourceDescriptor,
    ) -> Result<(), PrimitiveRegistryError> {
        let id = descriptor.id.clone();
        descriptor.validate_contract().map_err(|error| {
            PrimitiveRegistryError::InvalidSourceDescriptor {
                id: id.clone(),
                error,
            }
        })?;
        if self.sources.contains_key(&id) {
            return Err(PrimitiveRegistryError::DuplicateSource { id });
        }
        self.sources.insert(id, descriptor);
        Ok(())
    }

    /// Register a source primitive runtime.
    pub fn install_source_runtime<P>(&mut self) -> Result<(), PrimitiveRegistryError>
    where
        P: SourceRuntime,
    {
        let descriptor = P::descriptor();
        let id = descriptor.id.clone();
        self.install_source_descriptor(descriptor)?;
        self.source_runtimes.insert(id);
        Ok(())
    }

    /// Borrow all registered effect descriptors.
    pub fn effects(&self) -> &BTreeMap<EffectId, EffectDescriptor> {
        &self.effects
    }

    /// Borrow all registered source descriptors.
    pub fn sources(&self) -> &BTreeMap<SourceId, SourceDescriptor> {
        &self.sources
    }

    /// Borrow one registered effect descriptor.
    pub fn effect(&self, id: &EffectId) -> Option<&EffectDescriptor> {
        self.effects.get(id)
    }

    /// Borrow one registered source descriptor.
    pub fn source(&self, id: &SourceId) -> Option<&SourceDescriptor> {
        self.sources.get(id)
    }

    /// Return true when an effect id has a runtime for the requested domain table.
    pub fn has_runtime(&self, id: &EffectId, runtime: EffectRuntimeKind) -> bool {
        match runtime {
            EffectRuntimeKind::DescriptorOnly => self.effects.contains_key(id),
            EffectRuntimeKind::CellShader => self.cell_shader_runtimes.contains(id),
            EffectRuntimeKind::FrameFilter => self.frame_filter_runtimes.contains(id),
            EffectRuntimeKind::CoordinateSampler => self.coordinate_sampler_runtimes.contains(id),
            EffectRuntimeKind::Mask => self.mask_runtimes.contains(id),
            EffectRuntimeKind::ContentTransform => self.content_transform_runtimes.contains(id),
        }
    }

    /// Return true when a source id has a materialization runtime.
    pub fn has_source_runtime(&self, id: &SourceId) -> bool {
        self.source_runtimes.contains(id)
    }

    /// Export the descriptor view as a contract descriptor pack.
    pub fn to_descriptor_pack(
        &self,
        id: DescriptorPackId,
        version: impl Into<String>,
        display_name: impl Into<String>,
    ) -> DescriptorPack {
        DescriptorPack {
            id,
            version: version.into(),
            display_name: display_name.into(),
            category: Some("primitive".to_string()),
            source_descriptors: self.sources.clone(),
            effects: self.effects.clone(),
        }
    }

    fn install_effect<P>(
        &mut self,
        runtime: EffectRuntimeKind,
    ) -> Result<(), PrimitiveRegistryError>
    where
        P: EffectPrimitive,
    {
        self.install_effect_descriptor_with_runtime(P::descriptor(), runtime)
    }

    fn install_effect_descriptor_with_runtime(
        &mut self,
        descriptor: EffectDescriptor,
        runtime: EffectRuntimeKind,
    ) -> Result<(), PrimitiveRegistryError> {
        let id = descriptor.id.clone();
        descriptor.validate_io().map_err(|error| {
            PrimitiveRegistryError::InvalidEffectDescriptor {
                id: id.clone(),
                error,
            }
        })?;
        match runtime.required_domain() {
            Some(expected) if descriptor.domain != expected => {
                return Err(PrimitiveRegistryError::EffectDomainMismatch {
                    id,
                    runtime,
                    expected,
                    actual: descriptor.domain,
                });
            }
            _ => {}
        }
        if self.effects.contains_key(&id) {
            return Err(PrimitiveRegistryError::DuplicateEffect { id });
        }
        self.record_runtime(&id, runtime);
        self.effects.insert(id, descriptor);
        Ok(())
    }

    fn record_runtime(&mut self, id: &EffectId, runtime: EffectRuntimeKind) {
        match runtime {
            EffectRuntimeKind::DescriptorOnly => {}
            EffectRuntimeKind::CellShader => {
                self.cell_shader_runtimes.insert(id.clone());
            }
            EffectRuntimeKind::FrameFilter => {
                self.frame_filter_runtimes.insert(id.clone());
            }
            EffectRuntimeKind::CoordinateSampler => {
                self.coordinate_sampler_runtimes.insert(id.clone());
            }
            EffectRuntimeKind::Mask => {
                self.mask_runtimes.insert(id.clone());
            }
            EffectRuntimeKind::ContentTransform => {
                self.content_transform_runtimes.insert(id.clone());
            }
        }
    }
}

// <FILE>crates/tui-vfx-compost/src/primitive/cls_effect_registry.rs</FILE> - <DESC>Rust-owned primitive descriptor/runtime registry</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
