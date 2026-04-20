// <FILE>crates/tui-vfx-types/src/layer_id.rs</FILE> - <DESC>Opaque interned identifier for a scene layer</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan A Phase A.1 — foundation primitive for trace-selector / scene-layer identity</WCTX>
// <CLOG>0.1.0: initial opaque newtype around InternedString; From<&str>/From<String>; as_str accessor; serde via cfg_attr.</CLOG>

//! Opaque identifier for a scene layer.
//!
//! Produced by the recipe schema (`RaSceneLayer::id`) and consumed by
//! trace selectors / inspection sinks. `LayerId` is opaque: foreign code
//! compares two `LayerId`s for equality or extracts the name via
//! `as_str()`, but cannot otherwise inspect or mutate the inner string.
//!
//! Lives in `tui-vfx-types` rather than `tui-vfx-recipes` so that
//! downstream inspection code (`tui-vfx-debug`, trace tooling) can select
//! trace events by layer without depending on the recipe crate. This
//! preserves the layered model defined in the spec.
//!
//! # Examples
//!
//! ```
//! use tui_vfx_types::LayerId;
//!
//! let a: LayerId = "logo".into();
//! let b = LayerId::from(String::from("logo"));
//! assert_eq!(a, b);
//! assert_eq!(a.as_str(), "logo");
//! ```

use crate::InternedString;

/// Opaque identifier for a scene layer.
#[repr(transparent)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct LayerId(InternedString);

impl LayerId {
    /// Borrow the inner name as a `&str`.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl From<&str> for LayerId {
    fn from(s: &str) -> Self {
        Self(InternedString::new(s))
    }
}

impl From<String> for LayerId {
    fn from(s: String) -> Self {
        Self(InternedString::from(s))
    }
}

// <FILE>crates/tui-vfx-types/src/layer_id.rs</FILE> - <DESC>Opaque LayerId</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
