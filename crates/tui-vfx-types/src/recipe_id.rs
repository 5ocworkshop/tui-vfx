// <FILE>crates/tui-vfx-types/src/recipe_id.rs</FILE> - <DESC>Opaque interned identifier for a recipe instance</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan A Phase A.1 — foundation primitive for trace-selector / recipe identity</WCTX>
// <CLOG>0.1.0: initial opaque newtype around InternedString; From<&str>/From<String>; as_str accessor; serde via cfg_attr.</CLOG>

//! Opaque identifier for a recipe instance.
//!
//! Produced by the recipe envelope (`RaRecipeConfig::id`) and consumed
//! by trace selectors / inspection sinks. `RecipeId` is opaque: foreign
//! code compares two `RecipeId`s for equality or extracts the name via
//! `as_str()`, but cannot otherwise inspect or mutate the inner string.
//!
//! Lives in `tui-vfx-types` rather than `tui-vfx-recipes` so that
//! downstream inspection code (`tui-vfx-debug`, trace tooling) can
//! select trace events by recipe without depending on the recipe crate.
//!
//! # Examples
//!
//! ```
//! use tui_vfx_types::RecipeId;
//!
//! let a: RecipeId = "splash.v2".into();
//! let b = RecipeId::from(String::from("splash.v2"));
//! assert_eq!(a, b);
//! assert_eq!(a.as_str(), "splash.v2");
//! ```

use crate::InternedString;

/// Opaque identifier for a recipe instance.
#[repr(transparent)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct RecipeId(InternedString);

impl RecipeId {
    /// Borrow the inner name as a `&str`.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl From<&str> for RecipeId {
    fn from(s: &str) -> Self {
        Self(InternedString::new(s))
    }
}

impl From<String> for RecipeId {
    fn from(s: String) -> Self {
        Self(InternedString::from(s))
    }
}

// <FILE>crates/tui-vfx-types/src/recipe_id.rs</FILE> - <DESC>Opaque RecipeId</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
