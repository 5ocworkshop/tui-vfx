// <FILE>tui-vfx-compositor/src/filters/cls_pattern_fill.rs</FILE> - <DESC>Pattern fill filter for background textures</DESC>
// <VERS>VERSION: 2.0.0</VERS>
// <WCTX>L2/L3 abstraction: make compositor framework-agnostic</WCTX>
// <CLOG>Changed to use tui_vfx_types::Cell and Color for framework independence</CLOG>

use crate::traits::filter::Filter;
use serde::{Deserialize, Serialize};
use tui_vfx_types::{Cell, Color};

/// Pattern types for filling cells.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "PascalCase")]
pub enum PatternType {
    /// Single repeating character (e.g., '~' for water, '.' for sand)
    Single {
        /// The character to repeat
        char: char,
    },

    /// Checkerboard pattern alternating between two characters
    Checkerboard {
        /// Character for (x+y) % 2 == 0 positions
        char_a: char,
        /// Character for (x+y) % 2 == 1 positions
        char_b: char,
    },

    /// Horizontal line pattern (rows at regular intervals)
    HorizontalLines {
        /// Character for the lines
        line_char: char,
        /// Spacing between lines (line appears every N rows)
        spacing: u16,
    },

    /// Vertical line pattern (columns at regular intervals)
    VerticalLines {
        /// Character for the lines
        line_char: char,
        /// Spacing between lines (line appears every N columns)
        spacing: u16,
    },
}

impl Default for PatternType {
    fn default() -> Self {
        PatternType::Single { char: '.' }
    }
}

/// Filter that fills cells with a pattern.
///
/// Useful for creating background textures like:
/// - Water surfaces with `~`
/// - Sand or static with `.`
/// - Grid patterns with checkerboard
/// - Scan lines with horizontal lines
///
/// # Examples
///
/// ```ignore
/// // Water texture
/// let water = PatternFill::new(PatternType::Single('~'))
///     .with_fg(Color::Rgb(100, 150, 200));
///
/// // Checkerboard floor
/// let floor = PatternFill::new(PatternType::Checkerboard {
///     char_a: '█',
///     char_b: '░',
/// });
///
/// // Only fill empty cells (preserve content)
/// let subtle = PatternFill::new(PatternType::Single('·'))
///     .only_empty(true);
/// ```
#[derive(Default)]
pub struct PatternFill {
    /// The pattern to apply
    pub pattern: PatternType,
    /// If true, only fill cells that are empty (whitespace)
    pub only_empty: bool,
    /// Optional foreground color for the pattern
    pub fg_color: Option<Color>,
    /// Optional background color for the pattern
    pub bg_color: Option<Color>,
}

impl PatternFill {
    /// Create a new pattern fill with the specified pattern.
    pub fn new(pattern: PatternType) -> Self {
        Self {
            pattern,
            only_empty: false,
            fg_color: None,
            bg_color: None,
        }
    }

    /// Set whether to only fill empty cells.
    pub fn only_empty(mut self, only_empty: bool) -> Self {
        self.only_empty = only_empty;
        self
    }

    /// Set the foreground color for the pattern.
    pub fn with_fg(mut self, color: Color) -> Self {
        self.fg_color = Some(color);
        self
    }

    /// Set the background color for the pattern.
    #[allow(dead_code)]
    pub fn with_bg(mut self, color: Color) -> Self {
        self.bg_color = Some(color);
        self
    }

    /// Check if a cell is considered empty.
    fn is_cell_empty(cell: &Cell) -> bool {
        cell.ch.is_whitespace()
    }

    /// Get the character for this position based on the pattern.
    fn char_at(&self, x: u16, y: u16) -> Option<char> {
        match &self.pattern {
            PatternType::Single { char: c } => Some(*c),

            PatternType::Checkerboard { char_a, char_b } => {
                if (x + y) % 2 == 0 {
                    Some(*char_a)
                } else {
                    Some(*char_b)
                }
            }

            PatternType::HorizontalLines { line_char, spacing } => {
                if *spacing == 0 {
                    return None;
                }
                if y % spacing == 0 {
                    Some(*line_char)
                } else {
                    None
                }
            }

            PatternType::VerticalLines { line_char, spacing } => {
                if *spacing == 0 {
                    return None;
                }
                if x % spacing == 0 {
                    Some(*line_char)
                } else {
                    None
                }
            }
        }
    }
}

impl Filter for PatternFill {
    fn apply(&self, cell: &mut Cell, x: u16, y: u16, _width: u16, _height: u16, _t: f64) {
        // Skip non-empty cells if only_empty is set
        if self.only_empty && !Self::is_cell_empty(cell) {
            return;
        }

        // Get the character for this position
        if let Some(c) = self.char_at(x, y) {
            // Set the character directly
            cell.ch = c;

            // Apply colors if specified
            if let Some(fg) = self.fg_color {
                cell.fg = fg;
            }
            if let Some(bg) = self.bg_color {
                cell.bg = bg;
            }
        }
    }
}

#[cfg(test)]
#[allow(clippy::field_reassign_with_default)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_fill_single_char() {
        let filter = PatternFill::new(PatternType::Single { char: '~' });

        let mut cell = Cell::default();
        cell.ch = ' '; // Empty cell

        filter.apply(&mut cell, 0, 0, 10, 10, 1.0);

        assert_eq!(cell.ch, '~');
    }

    #[test]
    fn test_pattern_fill_only_empty_cells() {
        let filter = PatternFill::new(PatternType::Single { char: '.' }).only_empty(true);

        // Cell with content should not be modified
        let mut cell_with_content = Cell::default();
        cell_with_content.ch = 'X';

        filter.apply(&mut cell_with_content, 0, 0, 10, 10, 1.0);
        assert_eq!(cell_with_content.ch, 'X');

        // Empty cell should be filled
        let mut empty_cell = Cell::default();
        empty_cell.ch = ' ';

        filter.apply(&mut empty_cell, 0, 0, 10, 10, 1.0);
        assert_eq!(empty_cell.ch, '.');
    }

    #[test]
    fn test_pattern_fill_checkerboard() {
        let filter = PatternFill::new(PatternType::Checkerboard {
            char_a: '#',
            char_b: ' ',
        });

        // Even positions (0,0), (1,1), (0,2) should use char_a
        let mut cell = Cell::default();
        cell.ch = ' ';
        filter.apply(&mut cell, 0, 0, 10, 10, 1.0);
        assert_eq!(cell.ch, '#');

        let mut cell = Cell::default();
        cell.ch = ' ';
        filter.apply(&mut cell, 1, 1, 10, 10, 1.0);
        assert_eq!(cell.ch, '#');

        // Odd positions (1,0), (0,1) should use char_b
        let mut cell = Cell::default();
        cell.ch = ' ';
        filter.apply(&mut cell, 1, 0, 10, 10, 1.0);
        assert_eq!(cell.ch, ' ');

        let mut cell = Cell::default();
        cell.ch = ' ';
        filter.apply(&mut cell, 0, 1, 10, 10, 1.0);
        assert_eq!(cell.ch, ' ');
    }

    #[test]
    fn test_pattern_fill_horizontal_lines() {
        let filter = PatternFill::new(PatternType::HorizontalLines {
            line_char: '-',
            spacing: 2,
        });

        // Row 0, 2, 4, ... should have the line char
        let mut cell = Cell::default();
        cell.ch = ' ';
        filter.apply(&mut cell, 5, 0, 10, 10, 1.0);
        assert_eq!(cell.ch, '-');

        let mut cell = Cell::default();
        cell.ch = ' ';
        filter.apply(&mut cell, 5, 2, 10, 10, 1.0);
        assert_eq!(cell.ch, '-');

        // Row 1, 3, 5, ... should not have the line char
        let mut cell = Cell::default();
        cell.ch = ' ';
        filter.apply(&mut cell, 5, 1, 10, 10, 1.0);
        assert_eq!(cell.ch, ' ');
    }

    #[test]
    fn test_pattern_fill_vertical_lines() {
        let filter = PatternFill::new(PatternType::VerticalLines {
            line_char: '|',
            spacing: 3,
        });

        // Column 0, 3, 6, ... should have the line char
        let mut cell = Cell::default();
        cell.ch = ' ';
        filter.apply(&mut cell, 0, 5, 10, 10, 1.0);
        assert_eq!(cell.ch, '|');

        let mut cell = Cell::default();
        cell.ch = ' ';
        filter.apply(&mut cell, 3, 5, 10, 10, 1.0);
        assert_eq!(cell.ch, '|');

        // Other columns should not have the line char
        let mut cell = Cell::default();
        cell.ch = ' ';
        filter.apply(&mut cell, 1, 5, 10, 10, 1.0);
        assert_eq!(cell.ch, ' ');
    }

    #[test]
    fn test_pattern_fill_with_color() {
        let filter =
            PatternFill::new(PatternType::Single { char: '*' }).with_fg(Color::rgb(100, 100, 100));

        let mut cell = Cell::default();
        cell.ch = ' ';

        filter.apply(&mut cell, 0, 0, 10, 10, 1.0);

        assert_eq!(cell.ch, '*');
        assert_eq!(cell.fg, Color::rgb(100, 100, 100));
    }

    #[test]
    fn test_pattern_fill_preserves_existing_colors() {
        let filter = PatternFill::new(PatternType::Single { char: '.' });

        let mut cell = Cell::default();
        cell.ch = ' ';
        cell.fg = Color::rgb(255, 0, 0);
        cell.bg = Color::rgb(0, 0, 255);

        filter.apply(&mut cell, 0, 0, 10, 10, 1.0);

        assert_eq!(cell.ch, '.');
        // Colors should be preserved when no explicit color is set
        assert_eq!(cell.fg, Color::rgb(255, 0, 0));
        assert_eq!(cell.bg, Color::rgb(0, 0, 255));
    }

    #[test]
    fn test_pattern_fill_default() {
        let filter = PatternFill::default();
        // Default should be a sensible pattern
        assert!(matches!(filter.pattern, PatternType::Single { .. }));
    }

    #[test]
    fn test_pattern_fill_dots_texture() {
        // Common use case: dot texture for backgrounds
        let filter = PatternFill::new(PatternType::Single { char: '·' })
            .only_empty(true)
            .with_fg(Color::rgb(50, 50, 50));

        let mut cell = Cell::default();
        cell.ch = ' ';
        cell.bg = Color::rgb(0, 0, 0);

        filter.apply(&mut cell, 0, 0, 10, 10, 1.0);

        assert_eq!(cell.ch, '·');
        assert_eq!(cell.fg, Color::rgb(50, 50, 50));
    }

    #[test]
    fn test_pattern_type_serde_roundtrip() {
        let patterns = [
            PatternType::Single { char: '~' },
            PatternType::Checkerboard {
                char_a: '#',
                char_b: '.',
            },
            PatternType::HorizontalLines {
                line_char: '-',
                spacing: 2,
            },
            PatternType::VerticalLines {
                line_char: '|',
                spacing: 3,
            },
        ];

        for pattern in patterns {
            let json = serde_json::to_string(&pattern).unwrap();
            let parsed: PatternType = serde_json::from_str(&json).unwrap();
            assert_eq!(pattern, parsed);
        }
    }
}

// <FILE>tui-vfx-compositor/src/filters/cls_pattern_fill.rs</FILE> - <DESC>Pattern fill filter for background textures</DESC>
// <VERS>END OF VERSION: 2.0.0</VERS>
