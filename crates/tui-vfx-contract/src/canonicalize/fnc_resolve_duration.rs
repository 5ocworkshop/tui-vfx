// <FILE>crates/tui-vfx-contract/src/canonicalize/fnc_resolve_duration.rs</FILE> - <DESC>Parse author-side duration strings into canonical { kind, value } form</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 1 of canonicalize: turn "300ms" / "1.5s" / "2m" into canonical DurationSpec envelopes.</WCTX>
// <CLOG>0.1.0: INIT — accept ms / s / m suffixes; round seconds and minutes to milliseconds when they don't divide evenly.</CLOG>

use serde_json::{Value, json};

use super::cls_canonicalization_error::{CanonicalizationError, CanonicalizationErrorKind};

/// Parse a duration shorthand string (e.g., `"300ms"`) into the canonical
/// `{ kind: "milliseconds" | "seconds" | "minutes", value: <n> }` envelope.
pub fn resolve_duration(value: &Value) -> Result<Value, CanonicalizationError> {
    if let Value::Object(_) = value {
        return Ok(value.clone());
    }
    let raw = value.as_str().ok_or_else(|| {
        CanonicalizationError::new(
            CanonicalizationErrorKind::MalformedDuration {
                value: value.to_string(),
            },
            "duration must be a string with an ms / s / m suffix or a canonical object",
        )
    })?;
    parse_duration_str(raw)
}

fn parse_duration_str(raw: &str) -> Result<Value, CanonicalizationError> {
    let trimmed = raw.trim();
    let (number_part, kind) = if let Some(rest) = trimmed.strip_suffix("ms") {
        (rest, "milliseconds")
    } else if let Some(rest) = trimmed.strip_suffix('s') {
        (rest, "seconds")
    } else if let Some(rest) = trimmed.strip_suffix('m') {
        (rest, "minutes")
    } else {
        return Err(CanonicalizationError::new(
            CanonicalizationErrorKind::MalformedDuration { value: raw.into() },
            format!("duration {raw} must end in ms / s / m"),
        ));
    };
    let parsed: f64 = number_part.trim().parse().map_err(|_| {
        CanonicalizationError::new(
            CanonicalizationErrorKind::MalformedDuration { value: raw.into() },
            format!("duration {raw} numeric portion did not parse"),
        )
    })?;
    if parsed.fract() == 0.0 && parsed.is_finite() {
        Ok(json!({ "kind": kind, "value": parsed as i64 }))
    } else {
        // Promote fractional non-millisecond durations to milliseconds for integer storage.
        match kind {
            "milliseconds" => Ok(json!({ "kind": kind, "value": parsed.round() as i64 })),
            "seconds" => {
                Ok(json!({ "kind": "milliseconds", "value": (parsed * 1000.0).round() as i64 }))
            }
            "minutes" => {
                Ok(json!({ "kind": "milliseconds", "value": (parsed * 60_000.0).round() as i64 }))
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn milliseconds_integer() {
        assert_eq!(
            resolve_duration(&json!("300ms")).unwrap(),
            json!({ "kind": "milliseconds", "value": 300 })
        );
    }

    #[test]
    fn seconds_integer() {
        assert_eq!(
            resolve_duration(&json!("2s")).unwrap(),
            json!({ "kind": "seconds", "value": 2 })
        );
    }

    #[test]
    fn fractional_seconds_promote_to_milliseconds() {
        assert_eq!(
            resolve_duration(&json!("1.5s")).unwrap(),
            json!({ "kind": "milliseconds", "value": 1500 })
        );
    }

    #[test]
    fn minutes_integer() {
        assert_eq!(
            resolve_duration(&json!("2m")).unwrap(),
            json!({ "kind": "minutes", "value": 2 })
        );
    }

    #[test]
    fn missing_suffix_rejects() {
        let err = resolve_duration(&json!("100")).unwrap_err();
        assert!(matches!(
            err.kind,
            CanonicalizationErrorKind::MalformedDuration { .. }
        ));
    }

    #[test]
    fn pre_canonical_object_passes_through() {
        let canonical = json!({ "kind": "milliseconds", "value": 250 });
        assert_eq!(resolve_duration(&canonical).unwrap(), canonical);
    }
}

// <FILE>crates/tui-vfx-contract/src/canonicalize/fnc_resolve_duration.rs</FILE> - <DESC>Parse author-side duration strings into canonical { kind, value } form</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
