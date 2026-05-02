// <FILE>crates/tui-vfx-compost/tests/test_primitive_registry.rs</FILE> - <DESC>Primitive registry substrate tests</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 0 of Rust-SSOT primitive migration locks domain-specific registry construction and runtime cell access enforcement before primitive ports begin.</WCTX>
// <CLOG>0.1.0: INIT — prove descriptor/runtime registration, domain mismatch rejection, source runtime registration, and CellView debug assertions.</CLOG>

use std::collections::BTreeMap;

use tui_vfx_compost::primitive::{
    CellView, EffectPrimitive, EffectRegistry, EffectRuntimeContext, EffectRuntimeError,
    EffectRuntimeKind, FrameFilterRuntime, MaskRuntime, MaskVisibility, NoInputs, NoOutputs,
    PrimitiveRegistryError, SourcePrimitive, SourceRuntime, SourceSurface,
};
use tui_vfx_contract::{
    CellAccess, CellChannel, CellWritePolicy, CoordinateSpace, DescriptorPackId, EffectCompletion,
    EffectDescriptor, EffectDomain, EffectId, EffectLifecycle, RoleSpace, RoleWritePolicyKind,
    ScopeKind, ScopeSupport, SourceDescriptor, SourceId, SourceKind, SourceLifecycle,
    SourceOutputSize, SourceOutputSpec, SourceRolePolicy, WriteSupport,
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

// <FILE>crates/tui-vfx-compost/tests/test_primitive_registry.rs</FILE> - <DESC>Primitive registry substrate tests</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
