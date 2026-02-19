// <FILE>crates/tui-vfx-types/src/geometry.rs</FILE> - <DESC>Geometry types: Rect, Point, Size, Anchor</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>L2/L3 abstraction - extract core types to shared crate</WCTX>
// <CLOG>Initial creation - migrate from mixed-animations adapter</CLOG>

//! Geometry types for cell-based layouts.

/// Rectangle in cell coordinates.
///
/// This is the primary geometry type for defining effect regions.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl Rect {
    /// Create a new rectangle.
    pub const fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Create from position and size.
    pub const fn from_pos_size(pos: Point, size: Size) -> Self {
        Self {
            x: pos.x,
            y: pos.y,
            width: size.width,
            height: size.height,
        }
    }

    /// Area in cells.
    #[inline]
    pub const fn area(&self) -> u32 {
        self.width as u32 * self.height as u32
    }

    /// Check if rectangle is empty (zero area).
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.width == 0 || self.height == 0
    }

    /// Check if point is inside the rectangle.
    #[inline]
    pub const fn contains(&self, x: u16, y: u16) -> bool {
        x >= self.x
            && x < self.x.saturating_add(self.width)
            && y >= self.y
            && y < self.y.saturating_add(self.height)
    }

    /// Get the right edge x coordinate (exclusive).
    #[inline]
    pub const fn right(&self) -> u16 {
        self.x.saturating_add(self.width)
    }

    /// Get the bottom edge y coordinate (exclusive).
    #[inline]
    pub const fn bottom(&self) -> u16 {
        self.y.saturating_add(self.height)
    }

    /// Intersection with another rect (for clipping).
    pub fn intersect(&self, other: &Rect) -> Option<Rect> {
        let x1 = self.x.max(other.x);
        let y1 = self.y.max(other.y);
        let x2 = self.right().min(other.right());
        let y2 = self.bottom().min(other.bottom());

        if x1 < x2 && y1 < y2 {
            Some(Rect::new(x1, y1, x2 - x1, y2 - y1))
        } else {
            None
        }
    }

    /// Get the center point.
    pub const fn center(&self) -> Point {
        Point {
            x: self.x.saturating_add(self.width / 2),
            y: self.y.saturating_add(self.height / 2),
        }
    }

    /// Get the size.
    pub const fn size(&self) -> Size {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    /// Get the top-left position.
    pub const fn position(&self) -> Point {
        Point {
            x: self.x,
            y: self.y,
        }
    }
}

/// Size without position.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

impl Size {
    /// Create a new size.
    pub const fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }

    /// Area in cells.
    pub const fn area(&self) -> u32 {
        self.width as u32 * self.height as u32
    }

    /// Check if size is empty.
    pub const fn is_empty(&self) -> bool {
        self.width == 0 || self.height == 0
    }
}

/// Point in cell coordinates.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl Point {
    /// Create a new point.
    pub const fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

/// Anchor point for positioning.
///
/// Used to specify where effects anchor relative to a target rect.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Anchor {
    #[default]
    TopLeft,
    TopCenter,
    TopRight,
    CenterLeft,
    Center,
    CenterRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

impl Anchor {
    /// Calculate the anchor point within a rect.
    pub fn point_in(&self, rect: &Rect) -> Point {
        let cx = rect.x + rect.width / 2;
        let cy = rect.y + rect.height / 2;
        let right = rect.right().saturating_sub(1);
        let bottom = rect.bottom().saturating_sub(1);

        let (x, y) = match self {
            Anchor::TopLeft => (rect.x, rect.y),
            Anchor::TopCenter => (cx, rect.y),
            Anchor::TopRight => (right, rect.y),
            Anchor::CenterLeft => (rect.x, cy),
            Anchor::Center => (cx, cy),
            Anchor::CenterRight => (right, cy),
            Anchor::BottomLeft => (rect.x, bottom),
            Anchor::BottomCenter => (cx, bottom),
            Anchor::BottomRight => (right, bottom),
        };
        Point::new(x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect_basics() {
        let rect = Rect::new(10, 20, 30, 40);
        assert_eq!(rect.x, 10);
        assert_eq!(rect.y, 20);
        assert_eq!(rect.width, 30);
        assert_eq!(rect.height, 40);
        assert_eq!(rect.area(), 1200);
        assert_eq!(rect.right(), 40);
        assert_eq!(rect.bottom(), 60);
    }

    #[test]
    fn test_rect_contains() {
        let rect = Rect::new(10, 10, 10, 10);
        assert!(rect.contains(10, 10));
        assert!(rect.contains(15, 15));
        assert!(rect.contains(19, 19));
        assert!(!rect.contains(20, 20));
        assert!(!rect.contains(9, 9));
    }

    #[test]
    fn test_rect_intersect() {
        let a = Rect::new(0, 0, 10, 10);
        let b = Rect::new(5, 5, 10, 10);
        let c = Rect::new(20, 20, 5, 5);

        let ab = a.intersect(&b);
        assert!(ab.is_some());
        let ab = ab.unwrap();
        assert_eq!(ab.x, 5);
        assert_eq!(ab.y, 5);
        assert_eq!(ab.width, 5);
        assert_eq!(ab.height, 5);

        assert!(a.intersect(&c).is_none());
    }

    #[test]
    fn test_point_and_size() {
        let point = Point::new(5, 10);
        let size = Size::new(20, 30);
        let rect = Rect::from_pos_size(point, size);
        assert_eq!(rect.x, 5);
        assert_eq!(rect.y, 10);
        assert_eq!(rect.width, 20);
        assert_eq!(rect.height, 30);
    }

    #[test]
    fn test_anchor_points() {
        let rect = Rect::new(0, 0, 10, 10);
        assert_eq!(Anchor::TopLeft.point_in(&rect), Point::new(0, 0));
        assert_eq!(Anchor::TopCenter.point_in(&rect), Point::new(5, 0));
        assert_eq!(Anchor::Center.point_in(&rect), Point::new(5, 5));
        assert_eq!(Anchor::BottomRight.point_in(&rect), Point::new(9, 9));
    }
}

// <FILE>crates/tui-vfx-types/src/geometry.rs</FILE> - <DESC>Geometry types: Rect, Point, Size, Anchor</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
