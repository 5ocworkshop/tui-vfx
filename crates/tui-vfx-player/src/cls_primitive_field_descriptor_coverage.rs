// <FILE>crates/tui-vfx-player/src/cls_primitive_field_descriptor_coverage.rs</FILE> - <DESC>Descriptor input coverage lookup DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Player evidence tooling: share descriptor input/domain lookup data for primitive field coverage.</WCTX>
// <CLOG>0.1.0: INIT — add descriptor coverage lookup data.</CLOG>

use std::collections::{BTreeMap, BTreeSet};

pub(crate) type DescriptorInputMap = BTreeMap<String, BTreeSet<String>>;
pub(crate) type DescriptorDomainMap = BTreeMap<String, Option<String>>;

/// Descriptor inputs and domains used by primitive field coverage scans.
#[derive(Debug, Default)]
pub(crate) struct PrimitiveFieldDescriptorCoverage {
    pub(crate) inputs: DescriptorInputMap,
    pub(crate) domains: DescriptorDomainMap,
}

// <FILE>crates/tui-vfx-player/src/cls_primitive_field_descriptor_coverage.rs</FILE> - <DESC>Descriptor input coverage lookup DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
