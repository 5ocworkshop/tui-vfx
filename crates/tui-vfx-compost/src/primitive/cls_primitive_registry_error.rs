// <FILE>crates/tui-vfx-compost/src/primitive/cls_primitive_registry_error.rs</FILE> - <DESC>Primitive registry construction errors</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Registry construction must fail deterministically on duplicate ids, bad descriptor contracts, or runtime/domain mismatches.</WCTX>
// <CLOG>0.1.0: INIT — add structured registry error type.</CLOG>

use tui_vfx_contract::{DescriptorValidationError, EffectDomain, EffectId, SourceId};

use super::EffectRuntimeKind;

/// Errors returned while constructing the Rust-owned primitive registry.
#[derive(Clone, Debug, PartialEq)]
pub enum PrimitiveRegistryError {
    /// An effect descriptor id was registered more than once.
    DuplicateEffect { id: EffectId },
    /// A source descriptor id was registered more than once.
    DuplicateSource { id: SourceId },
    /// An effect runtime trait was registered for a descriptor with the wrong domain.
    EffectDomainMismatch {
        id: EffectId,
        runtime: EffectRuntimeKind,
        expected: EffectDomain,
        actual: EffectDomain,
    },
    /// An effect descriptor failed contract validation.
    InvalidEffectDescriptor {
        id: EffectId,
        error: DescriptorValidationError,
    },
    /// A source descriptor failed contract validation.
    InvalidSourceDescriptor {
        id: SourceId,
        error: DescriptorValidationError,
    },
}

impl std::fmt::Display for PrimitiveRegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DuplicateEffect { id } => {
                write!(
                    f,
                    "effect descriptor `{}` is registered more than once",
                    id.as_str()
                )
            }
            Self::DuplicateSource { id } => {
                write!(
                    f,
                    "source descriptor `{}` is registered more than once",
                    id.as_str()
                )
            }
            Self::EffectDomainMismatch {
                id,
                runtime,
                expected,
                actual,
            } => write!(
                f,
                "effect descriptor `{}` has domain {:?}, but runtime {:?} requires {:?}",
                id.as_str(),
                actual,
                runtime,
                expected
            ),
            Self::InvalidEffectDescriptor { id, error } => write!(
                f,
                "effect descriptor `{}` failed contract validation: {:?}",
                id.as_str(),
                error
            ),
            Self::InvalidSourceDescriptor { id, error } => write!(
                f,
                "source descriptor `{}` failed contract validation: {:?}",
                id.as_str(),
                error
            ),
        }
    }
}

impl std::error::Error for PrimitiveRegistryError {}

// <FILE>crates/tui-vfx-compost/src/primitive/cls_primitive_registry_error.rs</FILE> - <DESC>Primitive registry construction errors</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
