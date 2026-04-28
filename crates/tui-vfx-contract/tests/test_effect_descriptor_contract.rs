// <FILE>crates/tui-vfx-contract/tests/test_effect_descriptor_contract.rs</FILE> - <DESC>Minimal effect descriptor contract tests</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase E1: lock descriptor capability validation before implementation.</WCTX>
// <CLOG>0.1.0: INIT — prove descriptor access, scope, write policy, and lifecycle capability behavior.</CLOG>

use tui_vfx_contract::{
    CellAccess, CellChannel, CellWritePolicy, CoordinateSpace, EffectCompletion, EffectDescriptor,
    EffectDomain, EffectId, EffectLifecycle, RoleSpace, RoleWritePolicy, RoleWritePolicyKind,
    ScopeKind, ScopeSpec, ScopeSupport, WriteSupport,
};
use tui_vfx_types::RoleTag;

fn visual_dim_descriptor() -> EffectDescriptor {
    EffectDescriptor {
        id: EffectId::new("terminal.dim"),
        version: "0.1.0".to_string(),
        display_name: "Terminal dim".to_string(),
        category: Some("visual".to_string()),
        domain: EffectDomain::FrameFilter,
        cell_access: CellAccess {
            reads: vec![CellChannel::Foreground, CellChannel::Background],
            writes: vec![CellChannel::Foreground, CellChannel::Background],
        },
        scope_support: ScopeSupport {
            kinds: vec![
                ScopeKind::All,
                ScopeKind::Role,
                ScopeKind::Rect,
                ScopeKind::RowRange,
                ScopeKind::ColumnRange,
            ],
            coordinate_spaces: vec![
                CoordinateSpace::DestinationLocal,
                CoordinateSpace::SampledSource,
            ],
            role_spaces: vec![RoleSpace::SampledSource, RoleSpace::Destination],
        },
        write_support: WriteSupport {
            cell_policies: vec![
                CellWritePolicy::WriteCell,
                CellWritePolicy::SkipTransparentEmpty,
            ],
            role_policies: vec![RoleWritePolicyKind::PreserveDestination],
        },
        lifecycle: EffectLifecycle {
            completion: EffectCompletion::Instant,
            resettable: true,
            seekable: true,
            deterministic_with_seed: true,
        },
    }
}

fn role_writer_descriptor() -> EffectDescriptor {
    EffectDescriptor {
        id: EffectId::new("terminal.explicitRoleWrite"),
        version: "0.1.0".to_string(),
        display_name: "Explicit role write".to_string(),
        category: Some("proof".to_string()),
        domain: EffectDomain::ContentGenerator,
        cell_access: CellAccess {
            reads: vec![],
            writes: vec![
                CellChannel::Glyph,
                CellChannel::Foreground,
                CellChannel::Background,
                CellChannel::Role,
            ],
        },
        scope_support: ScopeSupport {
            kinds: vec![ScopeKind::All, ScopeKind::Role],
            coordinate_spaces: vec![CoordinateSpace::DestinationLocal],
            role_spaces: vec![RoleSpace::Destination],
        },
        write_support: WriteSupport {
            cell_policies: vec![CellWritePolicy::WriteCell],
            role_policies: vec![RoleWritePolicyKind::SetExplicit],
        },
        lifecycle: EffectLifecycle {
            completion: EffectCompletion::Instant,
            resettable: true,
            seekable: false,
            deterministic_with_seed: true,
        },
    }
}

fn sampler_descriptor() -> EffectDescriptor {
    EffectDescriptor {
        id: EffectId::new("terminal.shiftSampler"),
        version: "0.1.0".to_string(),
        display_name: "Shift sampler".to_string(),
        category: Some("sampler".to_string()),
        domain: EffectDomain::CoordinateSampler,
        cell_access: CellAccess {
            reads: vec![],
            writes: vec![],
        },
        scope_support: ScopeSupport {
            kinds: vec![ScopeKind::All],
            coordinate_spaces: vec![CoordinateSpace::DestinationLocal],
            role_spaces: vec![RoleSpace::SampledSource],
        },
        write_support: WriteSupport {
            cell_policies: vec![],
            role_policies: vec![],
        },
        lifecycle: EffectLifecycle {
            completion: EffectCompletion::Instant,
            resettable: false,
            seekable: false,
            deterministic_with_seed: true,
        },
    }
}

#[test]
fn visual_only_descriptor_does_not_write_role() {
    let descriptor = visual_dim_descriptor();

    assert_eq!(descriptor.domain, EffectDomain::FrameFilter);
    assert!(descriptor.cell_access.can_write(CellChannel::Foreground));
    assert!(descriptor.cell_access.can_write(CellChannel::Background));
    assert!(!descriptor.cell_access.can_write(CellChannel::Role));
}

#[test]
fn role_writer_descriptor_declares_explicit_role_write() {
    let descriptor = role_writer_descriptor();

    assert!(descriptor.cell_access.can_write(CellChannel::Role));
    assert!(
        descriptor
            .write_support
            .supports_role_policy(&RoleWritePolicy::SetExplicit {
                role: RoleTag::Shadow
            })
    );
}

#[test]
fn sampler_descriptor_declares_coordinate_sampler_domain_without_cell_writes() {
    let descriptor = sampler_descriptor();

    assert_eq!(descriptor.domain, EffectDomain::CoordinateSampler);
    assert!(descriptor.cell_access.writes.is_empty());
    assert!(descriptor.write_support.cell_policies.is_empty());
}

#[test]
fn descriptor_accepts_supported_role_scope() {
    let descriptor = visual_dim_descriptor();
    let scope = ScopeSpec::Role {
        role: RoleTag::Text,
    };

    assert!(descriptor.validate_scope(&scope).is_ok());
}

#[test]
fn descriptor_rejects_unsupported_scope_kind() {
    let descriptor = role_writer_descriptor();
    let scope = ScopeSpec::Rect {
        rect: tui_vfx_types::Rect::new(0, 0, 2, 2),
    };

    let error = descriptor
        .validate_scope(&scope)
        .expect_err("rect scope is unsupported");
    assert_eq!(error.requested_scope_kind(), Some(ScopeKind::Rect));
}

#[test]
fn descriptor_accepts_supported_cell_write_policy() {
    let descriptor = visual_dim_descriptor();

    assert!(
        descriptor
            .validate_cell_write_policy(CellWritePolicy::SkipTransparentEmpty)
            .is_ok()
    );
}

#[test]
fn descriptor_rejects_unsupported_cell_write_policy() {
    let descriptor = role_writer_descriptor();

    let error = descriptor
        .validate_cell_write_policy(CellWritePolicy::SkipTransparentEmpty)
        .expect_err("role writer only declares writeCell support");
    assert!(matches!(
        error,
        tui_vfx_contract::DescriptorValidationError::UnsupportedCellWritePolicy {
            requested: CellWritePolicy::SkipTransparentEmpty
        }
    ));
}

#[test]
fn descriptor_accepts_supported_role_write_policy() {
    let descriptor = role_writer_descriptor();

    assert!(
        descriptor
            .validate_role_write_policy(&RoleWritePolicy::SetExplicit {
                role: RoleTag::Shadow,
            })
            .is_ok()
    );
}

#[test]
fn descriptor_rejects_unsupported_role_write_policy() {
    let descriptor = visual_dim_descriptor();

    let error = descriptor
        .validate_role_write_policy(&RoleWritePolicy::SetExplicit {
            role: RoleTag::Shadow,
        })
        .expect_err("visual dim must not support explicit role writes");
    assert_eq!(
        error.requested_role_policy_kind(),
        Some(RoleWritePolicyKind::SetExplicit)
    );
}

#[test]
fn descriptor_rejects_writing_channel_not_declared() {
    let descriptor = visual_dim_descriptor();

    let error = descriptor
        .validate_write_channel(CellChannel::Role)
        .expect_err("visual dim must not write role channel");
    assert_eq!(error.requested_channel(), Some(CellChannel::Role));
}

#[test]
fn descriptor_contract_does_not_import_proof_pipeline_stage() {
    let src_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut combined = String::new();
    for entry in std::fs::read_dir(src_dir).expect("contract src can be read") {
        let path = entry.expect("entry can be read").path();
        if path.extension().is_some_and(|extension| extension == "rs") {
            combined.push_str(&std::fs::read_to_string(path).expect("contract source can be read"));
        }
    }

    assert!(!combined.contains("PipelineStage"));
    assert!(!combined.contains("SurfacePipeline"));
}

// <FILE>crates/tui-vfx-contract/tests/test_effect_descriptor_contract.rs</FILE> - <DESC>Minimal effect descriptor contract tests</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
