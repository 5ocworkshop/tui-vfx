// <FILE>crates/tui-vfx-next/src/cls_effect_descriptor.rs</FILE> - <DESC>Tiny effect descriptor DTO</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>New kernel Phase D0 schema/reference backfill after Phase C preflight OFPF split.</WCTX>
// <CLOG>0.4.0: DOC — mark the tiny descriptor proof artifact as intentionally outside D0 schemas.
// 0.3.0: REFACTOR — extract EffectDescriptor DTO.</CLOG>

use crate::EffectDomain;

/// Small effect descriptor used by surface contract tests and docs.
///
/// This Phase A proof artifact is intentionally not schema-visible in Phase D0;
/// the real descriptor model is deferred to the later descriptor phase.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EffectDescriptor {
    /// Stable canonical effect id.
    pub id: &'static str,
    /// Effect domain.
    pub domain: EffectDomain,
    /// Whether the effect can explicitly write roles.
    pub can_write_roles: bool,
}

// <FILE>crates/tui-vfx-next/src/cls_effect_descriptor.rs</FILE> - <DESC>Tiny effect descriptor DTO</DESC>
// <VERS>END OF VERSION: 0.4.0</VERS>
