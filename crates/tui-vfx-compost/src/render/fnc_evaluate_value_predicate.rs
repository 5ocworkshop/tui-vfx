// <FILE>crates/tui-vfx-compost/src/render/fnc_evaluate_value_predicate.rs</FILE> - <DESC>Evaluate scene visibility value predicates</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Scene element predicate visibility needs the same contract truth table at render time.</WCTX>
// <CLOG>0.1.0: INIT — add native ValuePredicate evaluation for element visibility.</CLOG>

use tui_vfx_contract::{Value, ValuePredicate};

pub(crate) fn evaluate_value_predicate(value: &Value, predicate: &ValuePredicate) -> bool {
    match predicate {
        ValuePredicate::IsTrue => matches!(value, Value::Boolean(true)),
        ValuePredicate::IsFalse => matches!(value, Value::Boolean(false)),
        ValuePredicate::NonZero => value.as_range_number().is_some_and(|number| number != 0.0),
        ValuePredicate::NonEmpty => match value {
            Value::String(text) | Value::Text(text) => !text.is_empty(),
            _ => false,
        },
        ValuePredicate::Equals { value: expected } => value == expected,
        ValuePredicate::NotEquals { value: expected } => value != expected,
        ValuePredicate::GreaterThan { value: expected } => {
            compare_numbers(value, expected, f64::gt)
        }
        ValuePredicate::LessThan { value: expected } => compare_numbers(value, expected, f64::lt),
        ValuePredicate::Truthy => truthy(value),
    }
}

fn compare_numbers(value: &Value, expected: &Value, compare: fn(&f64, &f64) -> bool) -> bool {
    let Some(value) = value.as_range_number() else {
        return false;
    };
    let Some(expected) = expected.as_range_number() else {
        return false;
    };
    compare(&value, &expected)
}

fn truthy(value: &Value) -> bool {
    match value {
        Value::Boolean(value) => *value,
        Value::Integer(value) => *value != 0,
        Value::Number(value) | Value::Duration(value) => value.is_finite() && *value != 0.0,
        Value::String(value) | Value::Text(value) => !value.is_empty(),
        Value::Color(_) | Value::Gradient(_) => true,
        _ => false,
    }
}

// <FILE>crates/tui-vfx-compost/src/render/fnc_evaluate_value_predicate.rs</FILE> - <DESC>Evaluate scene visibility value predicates</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
