// <FILE>crates/tui-vfx-types/src/role_map.rs</FILE> - <DESC>Dense per-cell RoleTag storage (one entry per cell of a SemanticScene)</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan A Phase A.1 — foundation primitive for SemanticScene role layer</WCTX>
// <CLOG>0.1.0: initial RoleMap with row-major Vec<RoleId> storage, embedded interner, bounds-checked accessors, row-major iterator, serde round-trip via cfg_attr.</CLOG>

//! Dense per-cell `RoleTag` storage.
//!
//! A `RoleMap` holds one `RoleId` per cell of a `SemanticScene`, plus the
//! `RoleInterner` needed to resolve those IDs back into `RoleTag` values on
//! read. Cells are stored row-major at `y * width + x`.
//!
//! # Dimensions
//!
//! Dimensions are `u16` to match `Rect` in `geometry.rs` and the typical
//! terminal-cell coordinate space. Internally `len = width * height` is
//! computed in `usize` so maps larger than `u16::MAX` cells are allowed.
//!
//! # Out-of-bounds policy
//!
//! - `get` returns `None` for out-of-bounds coordinates.
//! - `set` silently no-ops for out-of-bounds coordinates (no panic).
//!
//! This matches the `Grid` trait's set semantics and makes RoleMap safe
//! to use alongside an `OwnedGrid` whose own `set` is also a silent no-op
//! out of bounds.
//!
//! # Examples
//!
//! ```
//! use tui_vfx_types::{RoleMap, RoleTag};
//!
//! let mut map = RoleMap::empty(4, 3);
//! map.set((1, 1), RoleTag::Border);
//! assert_eq!(map.get((1, 1)), Some(RoleTag::Border));
//! assert_eq!(map.get((0, 0)), Some(RoleTag::Background));
//! ```

use crate::{RoleId, RoleInterner, RoleTag};

/// Dense per-cell `RoleTag` storage backing a `SemanticScene`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RoleMap {
    width: u16,
    height: u16,
    /// Row-major: index `y * width + x`. Length must equal `width * height`.
    cells: Vec<RoleId>,
    /// Owns the `RoleTag` ↔ `RoleId` lookup for this map.
    interner: RoleInterner,
}

impl RoleMap {
    /// Construct a map of the given dimensions with every cell set to
    /// `RoleTag::Background`.
    ///
    /// Equivalent to `new_with_default(width, height, RoleTag::Background)`.
    pub fn empty(width: u16, height: u16) -> Self {
        Self::new_with_default(width, height, RoleTag::Background)
    }

    /// Construct a map of the given dimensions with every cell set to
    /// `RoleTag::Background`. Alias of `empty` provided for call-site
    /// readability where "all background" is the intent.
    pub fn all_background(width: u16, height: u16) -> Self {
        Self::empty(width, height)
    }

    /// Construct a map of the given dimensions with every cell set to the
    /// supplied `default` role.
    pub fn new_with_default(width: u16, height: u16, default: RoleTag) -> Self {
        let len = width as usize * height as usize;
        let mut interner = RoleInterner::new();
        let id = interner.intern(&default);
        Self {
            width,
            height,
            cells: vec![id; len],
            interner,
        }
    }

    /// Map width in cells.
    pub fn width(&self) -> u16 {
        self.width
    }

    /// Map height in cells.
    pub fn height(&self) -> u16 {
        self.height
    }

    /// Fetch the `RoleTag` at position `(x, y)`.
    ///
    /// Returns `None` if `(x, y)` is out of bounds. Bounds are
    /// `0 ≤ x < width` and `0 ≤ y < height`.
    pub fn get(&self, pos: (u16, u16)) -> Option<RoleTag> {
        let (x, y) = pos;
        let idx = self.index(x, y)?;
        self.interner.resolve(self.cells[idx])
    }

    /// Set the `RoleTag` at position `(x, y)`.
    ///
    /// Silently no-ops if `(x, y)` is out of bounds (no panic). This
    /// matches the `Grid` trait's set semantics.
    pub fn set(&mut self, pos: (u16, u16), tag: RoleTag) {
        let (x, y) = pos;
        if let Some(idx) = self.index(x, y) {
            let id = self.interner.intern(&tag);
            self.cells[idx] = id;
        }
    }

    /// Iterate every cell in row-major order, yielding `(x, y, RoleTag)`.
    ///
    /// Row-major means cells are yielded in the order `(0,0), (1,0), …,
    /// (width-1, 0), (0, 1), (1, 1), …, (width-1, height-1)`.
    pub fn iter(&self) -> RoleMapIter<'_> {
        RoleMapIter {
            map: self,
            index: 0,
        }
    }

    fn index(&self, x: u16, y: u16) -> Option<usize> {
        if x < self.width && y < self.height {
            Some(y as usize * self.width as usize + x as usize)
        } else {
            None
        }
    }
}

/// Row-major iterator yielding `(x, y, RoleTag)` for every cell in a
/// `RoleMap`.
pub struct RoleMapIter<'a> {
    map: &'a RoleMap,
    index: usize,
}

impl<'a> Iterator for RoleMapIter<'a> {
    type Item = (u16, u16, RoleTag);

    fn next(&mut self) -> Option<Self::Item> {
        let w = self.map.width as usize;
        let h = self.map.height as usize;
        if w == 0 || h == 0 || self.index >= w * h {
            return None;
        }
        let x = (self.index % w) as u16;
        let y = (self.index / w) as u16;
        let role = self.map.interner.resolve(self.map.cells[self.index])?;
        self.index += 1;
        Some((x, y, role))
    }
}

// <FILE>crates/tui-vfx-types/src/role_map.rs</FILE> - <DESC>Dense per-cell RoleTag storage</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
