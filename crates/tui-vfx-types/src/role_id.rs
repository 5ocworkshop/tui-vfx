// <FILE>crates/tui-vfx-types/src/role_id.rs</FILE> - <DESC>Compact numeric ID newtype for RoleTag lookup via RoleInterner</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan A Phase A.1 — foundation primitive for dense per-cell role storage</WCTX>
// <CLOG>0.1.0: initial u16 newtype with Copy + Eq + Ord + Hash + accessor.</CLOG>

//! Compact numeric identifier for a `RoleTag`.
//!
//! `RoleId` is produced by `RoleInterner::intern` and used as the dense
//! per-cell storage value in `RoleMap::cells`. First-class variants of
//! `RoleTag` reserve IDs 0–11 in the declaration order of
//! `RoleTag::FIRST_CLASS`; `Custom` IDs start at 12.
//!
//! # Examples
//!
//! ```
//! use tui_vfx_types::RoleId;
//!
//! let a = RoleId::new(4);
//! let b = RoleId::new(4);
//! assert_eq!(a, b);
//! assert_eq!(a.id(), 4);
//! ```

/// Compact numeric identifier for a `RoleTag`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct RoleId(u16);

impl RoleId {
    /// Construct a `RoleId` from a raw numeric value.
    pub const fn new(id: u16) -> Self {
        Self(id)
    }

    /// Return the raw numeric value.
    pub const fn id(&self) -> u16 {
        self.0
    }
}

// <FILE>crates/tui-vfx-types/src/role_id.rs</FILE> - <DESC>Compact numeric role identifier</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
