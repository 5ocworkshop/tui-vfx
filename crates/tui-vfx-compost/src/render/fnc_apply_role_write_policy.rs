// <FILE>crates/tui-vfx-compost/src/render/fnc_apply_role_write_policy.rs</FILE> - <DESC>Apply supported role write policy after cell mutation</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Role write policy execution mirrors scene composition semantics for preserved, copied, and explicit roles.</WCTX>
// <CLOG>0.2.0: MINOR — write copied source roles and explicit roles after successful cell writes.
// 0.1.0: INIT — add role write seam for destination role preservation.</CLOG>

use tui_vfx_contract::RoleWritePolicy;
use tui_vfx_types::{RoleTag, SemanticScene};

pub(crate) fn apply_role_write_policy(
    destination: &mut SemanticScene,
    dest_x: usize,
    dest_y: usize,
    sampled_role: RoleTag,
    policy: &RoleWritePolicy,
) {
    match policy {
        RoleWritePolicy::PreserveDestination => {}
        RoleWritePolicy::CopySampledSource => {
            destination
                .roles_mut()
                .set((dest_x as u16, dest_y as u16), sampled_role);
        }
        RoleWritePolicy::SetExplicit { role } => {
            destination
                .roles_mut()
                .set((dest_x as u16, dest_y as u16), role.clone());
        }
    }
}

// <FILE>crates/tui-vfx-compost/src/render/fnc_apply_role_write_policy.rs</FILE> - <DESC>Apply supported role write policy after cell mutation</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
