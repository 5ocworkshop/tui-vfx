// <FILE>tui-vfx-core/src/mixed_signals_schema.rs</FILE> - <DESC>ConfigSchema implementations for mixed_signals types</DESC>
// <VERS>VERSION: 1.4.0</VERS>
// <WCTX>Expand SignalSpec schema coverage</WCTX>
// <CLOG>Model mixed_signals variants for recipe validation</CLOG>

use crate::schema::{ConfigSchema, FieldMeta, ScalarValue, SchemaField, SchemaNode, SchemaVariant};
use mixed_signals::prelude::{EasingType, SignalOrFloat, SignalSpec};

fn schema_field(name: &str, schema: SchemaNode) -> SchemaField {
    SchemaField::new(name, schema, FieldMeta::default())
}

fn prim(type_name: &str) -> SchemaNode {
    SchemaNode::Primitive {
        type_name: type_name.to_string(),
        range: None,
    }
}

fn signal_spec_ref() -> SchemaNode {
    SchemaNode::Opaque {
        type_name: "SignalSpec".to_string(),
    }
}

fn opaque(type_name: &str) -> SchemaNode {
    SchemaNode::Opaque {
        type_name: type_name.to_string(),
    }
}

impl ConfigSchema for SignalOrFloat {
    fn schema() -> SchemaNode {
        SchemaNode::Enum {
            name: "SignalOrFloat".to_string(),
            description: None,
            json_name: None,
            tag_field: None,
            variants: vec![
                SchemaVariant::Tuple {
                    name: "Static".to_string(),
                    description: None,
                    json_value: None,
                    items: vec![SchemaField::new(
                        "value",
                        SchemaNode::Primitive {
                            type_name: "f32".to_string(),
                            range: None,
                        },
                        FieldMeta {
                            help: Some("Constant float value".to_string()),
                            description: None,
                            default: Some(ScalarValue::number("0.0")),
                            range: None,
                            json_key: None,
                            optional: false,
                        },
                    )],
                },
                SchemaVariant::Tuple {
                    name: "Signal".to_string(),
                    description: None,
                    json_value: None,
                    items: vec![SchemaField::new(
                        "spec",
                        SignalSpec::schema(),
                        FieldMeta {
                            help: Some("Dynamic signal specification".to_string()),
                            description: None,
                            default: None,
                            range: None,
                            json_key: None,
                            optional: false,
                        },
                    )],
                },
            ],
        }
    }
}

impl ConfigSchema for SignalSpec {
    fn schema() -> SchemaNode {
        SchemaNode::Enum {
            name: "SignalSpec".to_string(),
            description: None,
            json_name: None,
            tag_field: Some("type".to_string()),
            variants: vec![
                SchemaVariant::Struct {
                    name: "Sine".to_string(),
                    description: None,
                    json_value: Some("sine".to_string()),
                    fields: vec![
                        schema_field("frequency", prim("f32")),
                        schema_field("amplitude", prim("f32")),
                        schema_field("offset", prim("f32")),
                        schema_field("phase", prim("f32")),
                    ],
                },
                SchemaVariant::Struct {
                    name: "Triangle".to_string(),
                    description: None,
                    json_value: Some("triangle".to_string()),
                    fields: vec![
                        schema_field("frequency", prim("f32")),
                        schema_field("amplitude", prim("f32")),
                        schema_field("offset", prim("f32")),
                        schema_field("phase", prim("f32")),
                    ],
                },
                SchemaVariant::Struct {
                    name: "Square".to_string(),
                    description: None,
                    json_value: Some("square".to_string()),
                    fields: vec![
                        schema_field("frequency", prim("f32")),
                        schema_field("amplitude", prim("f32")),
                        schema_field("offset", prim("f32")),
                        schema_field("phase", prim("f32")),
                        schema_field("duty", prim("f32")),
                    ],
                },
                SchemaVariant::Struct {
                    name: "Sawtooth".to_string(),
                    description: None,
                    json_value: Some("sawtooth".to_string()),
                    fields: vec![
                        schema_field("frequency", prim("f32")),
                        schema_field("amplitude", prim("f32")),
                        schema_field("offset", prim("f32")),
                        schema_field("phase", prim("f32")),
                        schema_field("inverted", prim("bool")),
                    ],
                },
                SchemaVariant::Struct {
                    name: "Constant".to_string(),
                    description: None,
                    json_value: Some("constant".to_string()),
                    fields: vec![SchemaField::new(
                        "value",
                        prim("f32"),
                        FieldMeta {
                            help: Some("Constant value".to_string()),
                            description: None,
                            default: Some(ScalarValue::number("1.0")),
                            range: None,
                            json_key: None,
                            optional: false,
                        },
                    )],
                },
                SchemaVariant::Struct {
                    name: "Ramp".to_string(),
                    description: None,
                    json_value: Some("ramp".to_string()),
                    fields: vec![
                        schema_field("start", prim("f32")),
                        schema_field("end", prim("f32")),
                        schema_field("duration", prim("f32")),
                    ],
                },
                SchemaVariant::Struct {
                    name: "Step".to_string(),
                    description: None,
                    json_value: Some("step".to_string()),
                    fields: vec![
                        schema_field("before", prim("f32")),
                        schema_field("after", prim("f32")),
                        schema_field("threshold", prim("f32")),
                    ],
                },
                SchemaVariant::Struct {
                    name: "Pulse".to_string(),
                    description: None,
                    json_value: Some("pulse".to_string()),
                    fields: vec![
                        schema_field("low", prim("f32")),
                        schema_field("high", prim("f32")),
                        schema_field("start", prim("f32")),
                        schema_field("end", prim("f32")),
                    ],
                },
                SchemaVariant::Struct {
                    name: "WhiteNoise".to_string(),
                    description: None,
                    json_value: Some("white_noise".to_string()),
                    fields: vec![
                        schema_field("seed", prim("u64")),
                        schema_field("amplitude", prim("f32")),
                        schema_field("sample_rate", prim("f32")),
                    ],
                },
                SchemaVariant::Struct {
                    name: "Perlin".to_string(),
                    description: None,
                    json_value: Some("perlin".to_string()),
                    fields: vec![
                        schema_field("seed", prim("u64")),
                        schema_field("scale", prim("f32")),
                        schema_field("amplitude", prim("f32")),
                        schema_field("octaves", prim("u8")),
                        schema_field("persistence", prim("f32")),
                    ],
                },
                SchemaVariant::Struct {
                    name: "SeededRandom".to_string(),
                    description: None,
                    json_value: Some("seeded_random".to_string()),
                    fields: vec![
                        schema_field("seed", prim("u64")),
                        schema_field("amplitude", prim("f32")),
                        schema_field("offset", prim("f32")),
                    ],
                },
                SchemaVariant::Struct {
                    name: "SpatialNoise".to_string(),
                    description: None,
                    json_value: Some("spatial_noise".to_string()),
                    fields: vec![
                        schema_field("seed", prim("u64")),
                        schema_field("frequency", prim("f32")),
                        schema_field("amplitude", prim("f32")),
                    ],
                },
                SchemaVariant::Struct {
                    name: "GaussianNoise".to_string(),
                    description: None,
                    json_value: Some("gaussian_noise".to_string()),
                    fields: vec![
                        schema_field("seed", prim("u64")),
                        schema_field("std_dev", prim("f32")),
                        schema_field("amplitude", prim("f32")),
                        schema_field("offset", prim("f32")),
                    ],
                },
                SchemaVariant::Struct {
                    name: "PoissonNoise".to_string(),
                    description: None,
                    json_value: Some("poisson_noise".to_string()),
                    fields: vec![
                        schema_field("seed", prim("u64")),
                        schema_field("lambda", prim("f32")),
                        schema_field("amplitude", prim("f32")),
                        schema_field("offset", prim("f32")),
                    ],
                },
                SchemaVariant::Struct {
                    name: "CorrelatedNoise".to_string(),
                    description: None,
                    json_value: Some("correlated_noise".to_string()),
                    fields: vec![
                        schema_field("seed", prim("u64")),
                        schema_field("correlation", prim("f32")),
                        schema_field("amplitude", prim("f32")),
                        schema_field("offset", prim("f32")),
                    ],
                },
                SchemaVariant::Struct {
                    name: "PinkNoise".to_string(),
                    description: None,
                    json_value: Some("pink_noise".to_string()),
                    fields: vec![
                        schema_field("seed", prim("u64")),
                        schema_field("amplitude", prim("f32")),
                        schema_field("offset", prim("f32")),
                    ],
                },
                SchemaVariant::Struct {
                    name: "PerCharacterNoise".to_string(),
                    description: None,
                    json_value: Some("per_character_noise".to_string()),
                    fields: vec![
                        schema_field("base_seed", prim("u64")),
                        schema_field("amplitude", prim("f32")),
                        schema_field("offset", prim("f32")),
                    ],
                },
                SchemaVariant::Struct {
                    name: "StudentTNoise".to_string(),
                    description: None,
                    json_value: Some("student_t_noise".to_string()),
                    fields: vec![
                        schema_field("seed", prim("u64")),
                        schema_field("degrees_of_freedom", prim("f32")),
                        schema_field("scale", prim("f32")),
                        schema_field("amplitude", prim("f32")),
                        schema_field("offset", prim("f32")),
                    ],
                },
                SchemaVariant::Struct {
                    name: "ImpulseNoise".to_string(),
                    description: None,
                    json_value: Some("impulse_noise".to_string()),
                    fields: vec![
                        schema_field("seed", prim("u64")),
                        schema_field("rate_hz", prim("f32")),
                        schema_field("impulse_width", prim("f32")),
                    ],
                },
                SchemaVariant::Struct {
                    name: "Adsr".to_string(),
                    description: None,
                    json_value: Some("adsr".to_string()),
                    fields: vec![
                        schema_field("attack", prim("f32")),
                        schema_field("decay", prim("f32")),
                        schema_field("sustain", prim("f32")),
                        schema_field("release", prim("f32")),
                        schema_field("peak", prim("f32")),
                    ],
                },
                SchemaVariant::Struct {
                    name: "Impact".to_string(),
                    description: None,
                    json_value: Some("impact".to_string()),
                    fields: vec![
                        schema_field("intensity", prim("f32")),
                        schema_field("decay", prim("f32")),
                    ],
                },
                SchemaVariant::Struct {
                    name: "LinearEnvelope".to_string(),
                    description: None,
                    json_value: Some("linear_envelope".to_string()),
                    fields: vec![
                        schema_field("attack", prim("f32")),
                        schema_field("release", prim("f32")),
                        schema_field("peak", prim("f32")),
                    ],
                },
                SchemaVariant::Struct {
                    name: "Add".to_string(),
                    description: None,
                    json_value: Some("add".to_string()),
                    fields: vec![
                        schema_field("a", signal_spec_ref()),
                        schema_field("b", signal_spec_ref()),
                    ],
                },
                SchemaVariant::Struct {
                    name: "Multiply".to_string(),
                    description: None,
                    json_value: Some("multiply".to_string()),
                    fields: vec![
                        schema_field("a", signal_spec_ref()),
                        schema_field("b", signal_spec_ref()),
                    ],
                },
                SchemaVariant::Struct {
                    name: "Scale".to_string(),
                    description: None,
                    json_value: Some("scale".to_string()),
                    fields: vec![
                        schema_field("a", signal_spec_ref()),
                        schema_field("b", signal_spec_ref()),
                    ],
                },
                SchemaVariant::Struct {
                    name: "Sum".to_string(),
                    description: None,
                    json_value: Some("sum".to_string()),
                    fields: vec![
                        schema_field("a", signal_spec_ref()),
                        schema_field("b", signal_spec_ref()),
                    ],
                },
                SchemaVariant::Struct {
                    name: "Mix".to_string(),
                    description: None,
                    json_value: Some("mix".to_string()),
                    fields: vec![
                        schema_field("a", signal_spec_ref()),
                        schema_field("b", signal_spec_ref()),
                        schema_field("mix", prim("f32")),
                    ],
                },
                SchemaVariant::Struct {
                    name: "FrequencyMod".to_string(),
                    description: None,
                    json_value: Some("frequency_mod".to_string()),
                    fields: vec![
                        schema_field("carrier", signal_spec_ref()),
                        schema_field("modulator", signal_spec_ref()),
                        schema_field("depth", prim("f32")),
                        schema_field("carrier_freq", prim("f32")),
                    ],
                },
                SchemaVariant::Struct {
                    name: "VcaCentered".to_string(),
                    description: None,
                    json_value: Some("vca_centered".to_string()),
                    fields: vec![
                        schema_field("carrier", signal_spec_ref()),
                        schema_field("amplitude", signal_spec_ref()),
                    ],
                },
                SchemaVariant::Struct {
                    name: "PhaseAccumulator".to_string(),
                    description: None,
                    json_value: Some("phase_accumulator".to_string()),
                    fields: vec![
                        schema_field("frequency", signal_spec_ref()),
                        schema_field("initial_phase", prim("f32")),
                    ],
                },
                SchemaVariant::Struct {
                    name: "PhaseSine".to_string(),
                    description: None,
                    json_value: Some("phase_sine".to_string()),
                    fields: vec![schema_field("phase", signal_spec_ref())],
                },
                SchemaVariant::Struct {
                    name: "Keyframes".to_string(),
                    description: None,
                    json_value: Some("keyframes".to_string()),
                    fields: vec![schema_field("keyframes", opaque("Keyframes"))],
                },
                SchemaVariant::Struct {
                    name: "Clamp".to_string(),
                    description: None,
                    json_value: Some("clamp".to_string()),
                    fields: vec![
                        schema_field("signal", signal_spec_ref()),
                        schema_field("min", prim("f32")),
                        schema_field("max", prim("f32")),
                    ],
                },
                SchemaVariant::Struct {
                    name: "Quantize".to_string(),
                    description: None,
                    json_value: Some("quantize".to_string()),
                    fields: vec![
                        schema_field("signal", signal_spec_ref()),
                        schema_field("levels", prim("u8")),
                    ],
                },
                SchemaVariant::Struct {
                    name: "Remap".to_string(),
                    description: None,
                    json_value: Some("remap".to_string()),
                    fields: vec![
                        schema_field("signal", signal_spec_ref()),
                        schema_field("in_min", prim("f32")),
                        schema_field("in_max", prim("f32")),
                        schema_field("out_min", prim("f32")),
                        schema_field("out_max", prim("f32")),
                    ],
                },
                SchemaVariant::Struct {
                    name: "Invert".to_string(),
                    description: None,
                    json_value: Some("invert".to_string()),
                    fields: vec![schema_field("signal", signal_spec_ref())],
                },
                SchemaVariant::Struct {
                    name: "Abs".to_string(),
                    description: None,
                    json_value: Some("abs".to_string()),
                    fields: vec![schema_field("signal", signal_spec_ref())],
                },
            ],
        }
    }
}

impl ConfigSchema for EasingType {
    fn schema() -> SchemaNode {
        SchemaNode::Enum {
            name: "EasingType".to_string(),
            description: None,
            json_name: None,
            tag_field: None,
            variants: vec![
                SchemaVariant::Unit {
                    name: "Linear".to_string(),
                    description: None,
                    json_value: None,
                },
                SchemaVariant::Unit {
                    name: "QuadIn".to_string(),
                    description: None,
                    json_value: None,
                },
                SchemaVariant::Unit {
                    name: "QuadOut".to_string(),
                    description: None,
                    json_value: None,
                },
                SchemaVariant::Unit {
                    name: "QuadInOut".to_string(),
                    description: None,
                    json_value: None,
                },
                SchemaVariant::Unit {
                    name: "CubicIn".to_string(),
                    description: None,
                    json_value: None,
                },
                SchemaVariant::Unit {
                    name: "CubicOut".to_string(),
                    description: None,
                    json_value: None,
                },
                SchemaVariant::Unit {
                    name: "CubicInOut".to_string(),
                    description: None,
                    json_value: None,
                },
                SchemaVariant::Unit {
                    name: "SineIn".to_string(),
                    description: None,
                    json_value: None,
                },
                SchemaVariant::Unit {
                    name: "SineOut".to_string(),
                    description: None,
                    json_value: None,
                },
                SchemaVariant::Unit {
                    name: "SineInOut".to_string(),
                    description: None,
                    json_value: None,
                },
                SchemaVariant::Unit {
                    name: "BackIn".to_string(),
                    description: None,
                    json_value: None,
                },
                SchemaVariant::Unit {
                    name: "BackOut".to_string(),
                    description: None,
                    json_value: None,
                },
                SchemaVariant::Unit {
                    name: "BackInOut".to_string(),
                    description: None,
                    json_value: None,
                },
                SchemaVariant::Unit {
                    name: "ElasticIn".to_string(),
                    description: None,
                    json_value: None,
                },
                SchemaVariant::Unit {
                    name: "ElasticOut".to_string(),
                    description: None,
                    json_value: None,
                },
                SchemaVariant::Unit {
                    name: "ElasticInOut".to_string(),
                    description: None,
                    json_value: None,
                },
                SchemaVariant::Unit {
                    name: "BounceIn".to_string(),
                    description: None,
                    json_value: None,
                },
                SchemaVariant::Unit {
                    name: "BounceOut".to_string(),
                    description: None,
                    json_value: None,
                },
                SchemaVariant::Unit {
                    name: "BounceInOut".to_string(),
                    description: None,
                    json_value: None,
                },
                SchemaVariant::Unit {
                    name: "ExpoIn".to_string(),
                    description: None,
                    json_value: None,
                },
                SchemaVariant::Unit {
                    name: "ExpoOut".to_string(),
                    description: None,
                    json_value: None,
                },
                SchemaVariant::Unit {
                    name: "ExpoInOut".to_string(),
                    description: None,
                    json_value: None,
                },
                SchemaVariant::Unit {
                    name: "CircIn".to_string(),
                    description: None,
                    json_value: None,
                },
                SchemaVariant::Unit {
                    name: "CircOut".to_string(),
                    description: None,
                    json_value: None,
                },
                SchemaVariant::Unit {
                    name: "CircInOut".to_string(),
                    description: None,
                    json_value: None,
                },
            ],
        }
    }
}

// <FILE>tui-vfx-core/src/mixed_signals_schema.rs</FILE> - <DESC>ConfigSchema implementations for mixed_signals types</DESC>
// <VERS>END OF VERSION: 1.3.0</VERS>
