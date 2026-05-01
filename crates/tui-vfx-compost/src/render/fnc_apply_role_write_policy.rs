// <FILE>crates/tui-vfx-compost/src/render/fnc_apply_role_write_policy.rs</FILE> - <DESC>Apply supported role write policy after cell mutation</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Role write policy execution keeps preserveDestination explicit while other role policies remain load-rejected.</WCTX>
// <CLOG>0.1.0: INIT — add role write seam for destination role preservation.</CLOG>

use tui_vfx_contract::RoleWritePolicy;
use tui_vfx_types::SemanticScene;

pub(crate) fn apply_role_write_policy(
    _destination: &mut SemanticScene,
    _dest_x: usize,
    _dest_y: usize,
    policy: &RoleWritePolicy,
) {
    match policy {
        RoleWritePolicy::PreserveDestination => {}
        RoleWritePolicy::CopySampledSource | RoleWritePolicy::SetExplicit { .. } => {
            debug_assert!(false, "unsupported role policies are rejected at load time");
        }
    }
}

// <FILE>crates/tui-vfx-compost/src/render/fnc_apply_role_write_policy.rs</FILE> - <DESC>Apply supported role write policy after cell mutation</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
