// <FILE>crates/tui-vfx-types/src/role_interner.rs</FILE> - <DESC>RoleTag &harr; RoleId interner with stable IDs for first-class variants</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan A Phase A.1 — foundation primitive for RoleMap dense storage</WCTX>
// <CLOG>0.1.0: initial interner; first-class variants preassigned 0..=11 at construction; Custom names start at 12 and grow monotonically.</CLOG>

//! `RoleTag` ↔ `RoleId` lookup.
//!
//! Each `RoleMap` owns a `RoleInterner` so its dense `Vec<RoleId>` cell
//! storage can be resolved back to `RoleTag` values on read.
//!
//! # Stable ID assignment
//!
//! First-class variants of `RoleTag` reserve IDs **0..=11** in the
//! declaration order of `RoleTag::FIRST_CLASS`:
//!
//! | ID | Variant    |
//! |----|------------|
//! | 0  | Background |
//! | 1  | Text       |
//! | 2  | Title      |
//! | 3  | Caption    |
//! | 4  | Border     |
//! | 5  | Image      |
//! | 6  | Icon       |
//! | 7  | Indicator  |
//! | 8  | Highlight  |
//! | 9  | Shadow     |
//! | 10 | Decoration |
//! | 11 | Procedural |
//!
//! `Custom(name)` IDs start at **12** and increase monotonically as new
//! names are first encountered. Re-interning a previously-seen name
//! returns the same `RoleId`.
//!
//! # Examples
//!
//! ```
//! use tui_vfx_types::{RoleInterner, RoleTag};
//!
//! let mut interner = RoleInterner::new();
//! assert_eq!(interner.intern(&RoleTag::Background).id(), 0);
//! assert_eq!(interner.intern(&RoleTag::Shadow).id(), 9);
//! ```

use crate::{InternedRoleName, RoleId, RoleTag};

/// Reserved first-class ID count.
const FIRST_CLASS_COUNT: u16 = 12;

/// `RoleTag` ↔ `RoleId` lookup backing a `RoleMap`.
///
/// First-class variants have stable preassigned IDs 0..=11; Custom IDs
/// start at 12 and grow monotonically.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RoleInterner {
    /// Indices [12..] hold custom role names in insertion order.
    custom_names: Vec<InternedRoleName>,
}

impl Default for RoleInterner {
    fn default() -> Self {
        Self::new()
    }
}

impl RoleInterner {
    /// Create an interner. First-class IDs 0..=11 are immediately valid
    /// for `resolve` even before any `intern` call.
    pub const fn new() -> Self {
        Self {
            custom_names: Vec::new(),
        }
    }

    /// Intern a `RoleTag` and return its `RoleId`.
    ///
    /// First-class variants always return their preassigned ID.
    /// Custom variants return an existing ID if the name was interned
    /// previously, or a fresh ID starting at 12.
    pub fn intern(&mut self, tag: &RoleTag) -> RoleId {
        match tag {
            RoleTag::Background => RoleId::new(0),
            RoleTag::Text => RoleId::new(1),
            RoleTag::Title => RoleId::new(2),
            RoleTag::Caption => RoleId::new(3),
            RoleTag::Border => RoleId::new(4),
            RoleTag::Image => RoleId::new(5),
            RoleTag::Icon => RoleId::new(6),
            RoleTag::Indicator => RoleId::new(7),
            RoleTag::Highlight => RoleId::new(8),
            RoleTag::Shadow => RoleId::new(9),
            RoleTag::Decoration => RoleId::new(10),
            RoleTag::Procedural => RoleId::new(11),
            RoleTag::Custom(name) => {
                if let Some(existing) =
                    self.custom_names.iter().position(|n| n == name)
                {
                    RoleId::new(FIRST_CLASS_COUNT + existing as u16)
                } else {
                    let idx = self.custom_names.len() as u16;
                    self.custom_names.push(name.clone());
                    RoleId::new(FIRST_CLASS_COUNT + idx)
                }
            }
        }
    }

    /// Resolve a `RoleId` back to its `RoleTag`.
    ///
    /// Returns `None` for IDs not currently registered (unknown Custom
    /// IDs). Known first-class IDs always resolve, regardless of intern
    /// history.
    pub fn resolve(&self, id: RoleId) -> Option<RoleTag> {
        match id.id() {
            0 => Some(RoleTag::Background),
            1 => Some(RoleTag::Text),
            2 => Some(RoleTag::Title),
            3 => Some(RoleTag::Caption),
            4 => Some(RoleTag::Border),
            5 => Some(RoleTag::Image),
            6 => Some(RoleTag::Icon),
            7 => Some(RoleTag::Indicator),
            8 => Some(RoleTag::Highlight),
            9 => Some(RoleTag::Shadow),
            10 => Some(RoleTag::Decoration),
            11 => Some(RoleTag::Procedural),
            n => {
                let custom_index = (n - FIRST_CLASS_COUNT) as usize;
                self.custom_names
                    .get(custom_index)
                    .cloned()
                    .map(RoleTag::Custom)
            }
        }
    }
}

// <FILE>crates/tui-vfx-types/src/role_interner.rs</FILE> - <DESC>RoleTag / RoleId interner</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
