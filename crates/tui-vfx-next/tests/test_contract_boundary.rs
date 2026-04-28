// <FILE>crates/tui-vfx-next/tests/test_contract_boundary.rs</FILE> - <DESC>Phase D3 logical contract/proof boundary checks</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase D3: verify logical module boundaries compile without moving files.</WCTX>
// <CLOG>0.1.0: INIT — assert contract, proof, and schema-root re-export lanes are available.</CLOG>

use tui_vfx_next::{
    contract::{CellWritePolicy, CoordinateSampler, RoleWritePolicy, ScopeSpec, Surface},
    proof::{PipelineStage, SurfaceEngine},
    schema_roots::SurfacePipeline,
};
use tui_vfx_types::RoleTag;

#[test]
fn contract_proof_and_schema_root_modules_are_available() {
    let mut destination = Surface::new(1, 1, RoleTag::Background);
    let source = Surface::new(1, 1, RoleTag::Text);

    let outcome = SurfaceEngine::copy(&source, &mut destination, &ScopeSpec::All);

    assert_eq!(outcome.written_cells, 1);
    assert_eq!(destination.role(0, 0), Some(&RoleTag::Text));
    let cell_policy = CellWritePolicy::WriteCell;
    let role_policy = RoleWritePolicy::CopySampledSource;
    assert!(matches!(cell_policy, CellWritePolicy::WriteCell));
    assert!(matches!(role_policy, RoleWritePolicy::CopySampledSource));
    let _pipeline_root = SurfacePipeline::new().then(PipelineStage::copy("copy", ScopeSpec::All));
    fn accepts_contract_sampler(_sampler: &impl CoordinateSampler) {}
    accepts_contract_sampler(&tui_vfx_next::proof::IdentitySampler);
}

// <FILE>crates/tui-vfx-next/tests/test_contract_boundary.rs</FILE> - <DESC>Phase D3 logical contract/proof boundary checks</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
