// <FILE>crates/tui-vfx-compost/tests/test_primitive_registry.rs</FILE> - <DESC>Primitive registry substrate tests</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Phase 0.5/1 of Rust-SSOT primitive migration starts the v3.1 primitive pack with filter.dim and shared color/channel helpers.</WCTX>
// <CLOG>0.1.0: INIT — prove descriptor/runtime registration, domain mismatch rejection, source runtime registration, and CellView debug assertions.</CLOG>
// <CLOG>0.2.0: ADD — prove filter.dim descriptor parity with the migration seed, pack installation, and v3.1 runtime color semantics.</CLOG>

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use tui_vfx_compost::primitive::{
    CellView, EffectPrimitive, EffectRegistry, EffectRuntimeContext, EffectRuntimeError,
    EffectRuntimeKind, FilterDim, FilterDimInputs, FrameFilterRuntime, MaskRuntime, MaskVisibility,
    NoInputs, NoOutputs, PrimitiveRegistryError, SourcePrimitive, SourceRuntime, SourceSurface,
    dim_color, install_v31_primitive_pack,
};
use tui_vfx_contract::{
    CellAccess, CellChannel, CellWritePolicy, CoordinateSpace, DescriptorPack, DescriptorPackId,
    EffectCompletion, EffectDescriptor, EffectDomain, EffectId, EffectInputId, EffectLifecycle,
    RoleSpace, RoleWritePolicyKind, ScopeKind, ScopeSupport, SourceDescriptor, SourceId,
    SourceKind, SourceLifecycle, SourceOutputSize, SourceOutputSpec, SourceRolePolicy, Value,
    WriteSupport,
};
use tui_vfx_types::{Cell, Color, RoleTag};

fn effect_descriptor(id: &str, domain: EffectDomain, writes: Vec<CellChannel>) -> EffectDescriptor {
    EffectDescriptor {
        id: EffectId::new(id),
        version: "0.1.0".to_string(),
        display_name: id.to_string(),
        category: Some("test".to_string()),
        domain,
        cell_access: CellAccess {
            reads: writes.clone(),
            writes,
        },
        scope_support: ScopeSupport {
            kinds: vec![ScopeKind::All],
            coordinate_spaces: vec![CoordinateSpace::DestinationLocal],
            role_spaces: vec![RoleSpace::Destination],
        },
        write_support: WriteSupport {
            cell_policies: vec![CellWritePolicy::WriteCell],
            role_policies: vec![RoleWritePolicyKind::PreserveDestination],
        },
        inputs: BTreeMap::new(),
        outputs: BTreeMap::new(),
        lifecycle: EffectLifecycle {
            completion: EffectCompletion::Instant,
            resettable: true,
            seekable: true,
            deterministic_with_seed: true,
        },
    }
}

fn source_descriptor(id: &str) -> SourceDescriptor {
    SourceDescriptor {
        id: SourceId::new(id),
        version: "0.1.0".to_string(),
        display_name: id.to_string(),
        category: Some("test".to_string()),
        kind: SourceKind::Card,
        inputs: BTreeMap::new(),
        assets: BTreeMap::new(),
        output: SourceOutputSpec {
            size: SourceOutputSize::HostDriven,
            roles: SourceRolePolicy::DefaultRole {
                role: RoleTag::Text,
            },
        },
        lifecycle: SourceLifecycle {
            deterministic_with_seed: true,
            time_aware: false,
            resize_aware: true,
        },
    }
}

struct TestFrameFilter;

impl EffectPrimitive for TestFrameFilter {
    type Inputs = NoInputs;
    type Outputs = NoOutputs;

    fn descriptor() -> EffectDescriptor {
        effect_descriptor(
            "filter.testFrame",
            EffectDomain::FrameFilter,
            vec![CellChannel::Foreground],
        )
    }
}

impl FrameFilterRuntime for TestFrameFilter {
    fn filter_cell(
        _inputs: &Self::Inputs,
        cell: &mut CellView<'_, Self>,
        _context: &EffectRuntimeContext<'_>,
    ) -> Result<(), EffectRuntimeError> {
        cell.set_foreground(Color::RED);
        Ok(())
    }
}

struct WrongDomainMask;

impl EffectPrimitive for WrongDomainMask {
    type Inputs = NoInputs;
    type Outputs = NoOutputs;

    fn descriptor() -> EffectDescriptor {
        effect_descriptor("mask.wrongDomain", EffectDomain::FrameFilter, vec![])
    }
}

impl MaskRuntime for WrongDomainMask {
    fn visibility(
        _inputs: &Self::Inputs,
        _context: &EffectRuntimeContext<'_>,
    ) -> Result<MaskVisibility, EffectRuntimeError> {
        Ok(MaskVisibility::VISIBLE)
    }
}

struct ReadOnlyFilter;

impl EffectPrimitive for ReadOnlyFilter {
    type Inputs = NoInputs;
    type Outputs = NoOutputs;

    fn descriptor() -> EffectDescriptor {
        effect_descriptor("filter.readOnly", EffectDomain::FrameFilter, vec![])
    }
}

struct TestSource;

impl SourcePrimitive for TestSource {
    type Inputs = ();

    fn descriptor() -> SourceDescriptor {
        source_descriptor("source.testCard")
    }
}

impl SourceRuntime for TestSource {
    fn materialize(
        _inputs: &Self::Inputs,
        _context: &EffectRuntimeContext<'_>,
    ) -> Result<SourceSurface, EffectRuntimeError> {
        Err(EffectRuntimeError::new("not exercised by registry tests"))
    }
}

#[test]
fn registry_installs_domain_runtime_and_exports_descriptor_pack() {
    let mut registry = EffectRegistry::new();

    registry
        .install_frame_filter::<TestFrameFilter>()
        .expect("frame filter registers");

    let id = EffectId::new("filter.testFrame");
    assert_eq!(registry.effects().len(), 1);
    assert_eq!(
        registry.effect(&id).unwrap().domain,
        EffectDomain::FrameFilter
    );
    assert!(registry.has_runtime(&id, EffectRuntimeKind::FrameFilter));
    assert!(!registry.has_runtime(&id, EffectRuntimeKind::Mask));

    let pack = registry.to_descriptor_pack(
        DescriptorPackId::new("v3.1.primitive.test"),
        "0.1.0",
        "Test primitive pack",
    );
    pack.validate().expect("registry descriptor pack is valid");
    assert!(pack.effects.contains_key(&id));
}

#[test]
fn registry_rejects_runtime_domain_mismatch() {
    let mut registry = EffectRegistry::new();

    let error = registry
        .install_mask::<WrongDomainMask>()
        .expect_err("mask runtime cannot register frameFilter descriptor");

    assert!(matches!(
        error,
        PrimitiveRegistryError::EffectDomainMismatch {
            runtime: EffectRuntimeKind::Mask,
            expected: EffectDomain::Mask,
            actual: EffectDomain::FrameFilter,
            ..
        }
    ));
}

#[test]
fn registry_installs_source_runtime() {
    let mut registry = EffectRegistry::new();

    registry
        .install_source_runtime::<TestSource>()
        .expect("source runtime registers");

    let id = SourceId::new("source.testCard");
    assert!(registry.source(&id).is_some());
    assert!(registry.has_source_runtime(&id));
}

#[test]
fn cell_view_allows_declared_writes() {
    let mut cell = Cell::new('x');
    let mut view = CellView::<TestFrameFilter>::new(&mut cell);

    view.set_foreground(Color::BLUE);

    assert_eq!(view.cell().fg, Color::BLUE);
}

#[test]
#[should_panic(expected = "does not declare write access")]
fn cell_view_panics_on_undeclared_write_in_debug_builds() {
    let mut cell = Cell::new('x');
    let mut view = CellView::<ReadOnlyFilter>::new(&mut cell);

    view.set_foreground(Color::RED);
}

#[test]
fn filter_dim_descriptor_matches_current_migration_seed() {
    let pack_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crate lives under repo/crates/tui-vfx-compost")
        .join("descriptors/v3.1/packs/primitive.json");
    let pack: DescriptorPack = serde_json::from_str(
        &fs::read_to_string(pack_path).expect("read primitive descriptor seed"),
    )
    .expect("deserialize primitive descriptor seed");

    assert_eq!(
        FilterDim::descriptor(),
        pack.effects[&EffectId::new("filter.dim")]
    );
}

#[test]
fn v31_primitive_pack_installs_filter_dim() {
    let mut registry = EffectRegistry::new();

    install_v31_primitive_pack(&mut registry).expect("v3.1 primitive pack installs");

    let id = EffectId::new("filter.dim");
    let descriptor = registry
        .effect(&id)
        .expect("filter.dim descriptor is registered");
    assert_eq!(descriptor.domain, EffectDomain::FrameFilter);
    assert_eq!(
        descriptor.inputs[&EffectInputId::new("factor")]
            .value
            .default,
        Some(Value::Number(0.3))
    );
    assert_eq!(
        descriptor.inputs[&EffectInputId::new("channelTarget")]
            .value
            .allowed_values,
        vec!["both", "foreground", "background"]
    );
    assert!(registry.has_runtime(&id, EffectRuntimeKind::FrameFilter));
}

#[test]
fn dim_color_clamps_factor_and_preserves_alpha() {
    assert_eq!(
        dim_color(Color::new(100, 50, 25, 128), 2.0),
        Color::new(0, 0, 0, 128)
    );
    assert_eq!(
        dim_color(Color::new(100, 50, 25, 128), -1.0),
        Color::new(100, 50, 25, 128)
    );
}

#[test]
fn filter_dim_runtime_matches_legacy_channel_target_semantics() {
    let sample = tui_vfx_compost::SampleContext::default();
    let context = EffectRuntimeContext::new(&sample, 0, 0, 1, 1);
    let mut cell = Cell::styled(
        'x',
        Color::rgb(100, 50, 0),
        Color::rgb(10, 20, 30),
        tui_vfx_types::Modifiers::NONE,
    );
    let mut view = CellView::<FilterDim>::new(&mut cell);

    FilterDim::filter_cell(
        &FilterDimInputs {
            factor: 0.5,
            channel_target: tui_vfx_compost::primitive::ChannelTarget::Foreground,
        },
        &mut view,
        &context,
    )
    .expect("filter.dim applies");

    assert_eq!(view.cell().fg, Color::rgb(50, 25, 0));
    assert_eq!(view.cell().bg, Color::rgb(10, 20, 30));
}

// <FILE>crates/tui-vfx-compost/tests/test_primitive_registry.rs</FILE> - <DESC>Primitive registry substrate tests</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
