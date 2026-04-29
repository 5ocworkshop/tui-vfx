// <FILE>crates/tui-vfx-next/src/cls_number_cell_field.rs</FILE> - <DESC>Proof-only spatial scalar field value</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase G4: prove cell-field graph values without real effect ports.</WCTX>
// <CLOG>0.1.0: INIT — add row-major numeric cell field helper.</CLOG>

/// Proof-only per-cell numeric field stored in destination-local row-major order.
#[derive(Clone, Debug, PartialEq)]
pub struct NumberCellField {
    width: usize,
    height: usize,
    values: Vec<f64>,
}

impl NumberCellField {
    /// Build a field from explicit row-major samples.
    pub fn new(width: usize, height: usize, values: Vec<f64>) -> Self {
        assert_eq!(
            values.len(),
            width * height,
            "field sample count must match dimensions"
        );
        Self {
            width,
            height,
            values,
        }
    }

    /// Build a normalized x-coordinate field in the inclusive 0..1 range.
    pub fn normalized_x(width: usize, height: usize) -> Self {
        let denom = width.saturating_sub(1).max(1) as f64;
        let mut values = Vec::with_capacity(width * height);
        for _y in 0..height {
            for x in 0..width {
                values.push(x as f64 / denom);
            }
        }
        Self::new(width, height, values)
    }

    /// Surface width represented by this field.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Surface height represented by this field.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Sample one coordinate if it is in bounds.
    pub fn sample(&self, x: usize, y: usize) -> Option<f64> {
        (x < self.width && y < self.height).then(|| self.values[y * self.width + x])
    }

    /// Map every numeric sample through a pure transform.
    pub fn map(&self, mut f: impl FnMut(f64) -> f64) -> Self {
        Self::new(
            self.width,
            self.height,
            self.values.iter().copied().map(&mut f).collect(),
        )
    }
}

// <FILE>crates/tui-vfx-next/src/cls_number_cell_field.rs</FILE> - <DESC>Proof-only spatial scalar field value</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
