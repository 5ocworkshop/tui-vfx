// <FILE>crates/tui-vfx-next/src/cls_proof_value.rs</FILE> - <DESC>Proof graph value bus runtime value</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase G4: carry frame and cell-field values through proof execution.</WCTX>
// <CLOG>0.1.0: INIT — add frame literal and number-cell-field proof value variants.</CLOG>

use crate::{GraphValueShape, NumberCellField, Value, ValueKind, ValueSpec};

/// Runtime value carried by the proof-only graph value bus and node inputs.
#[derive(Clone, Debug, PartialEq)]
pub enum ProofValue {
    /// One contract literal value for the whole node/frame.
    Frame(Value),
    /// Per-cell numeric field.
    NumberCellField(NumberCellField),
}

impl ProofValue {
    /// Return the compatible contract value kind.
    pub fn kind(&self) -> ValueKind {
        match self {
            Self::Frame(value) => value.kind(),
            Self::NumberCellField(_) => ValueKind::Number,
        }
    }

    /// Return the graph value shape represented by this proof value.
    pub fn shape(&self) -> GraphValueShape {
        match self {
            Self::Frame(_) => GraphValueShape::FrameValue,
            Self::NumberCellField(_) => GraphValueShape::CellField,
        }
    }

    /// Validate this proof value against an effect input value spec.
    pub fn validate_against(
        &self,
        spec: &ValueSpec,
    ) -> Result<(), crate::DescriptorValidationError> {
        match self {
            Self::Frame(value) => spec.validate_value(value),
            Self::NumberCellField(_) if spec.kind == ValueKind::Number => Ok(()),
            Self::NumberCellField(_) => Err(crate::DescriptorValidationError::SourceKindMismatch {
                expected: spec.kind,
                actual: ValueKind::Number,
            }),
        }
    }

    /// Borrow the frame literal when this value is frame-shaped.
    pub fn frame(&self) -> Option<&Value> {
        match self {
            Self::Frame(value) => Some(value),
            Self::NumberCellField(_) => None,
        }
    }
}

// <FILE>crates/tui-vfx-next/src/cls_proof_value.rs</FILE> - <DESC>Proof graph value bus runtime value</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
