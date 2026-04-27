// <FILE>tui-vfx-core/src/schema/mod.rs</FILE> - <DESC>Schema model for configuration introspection</DESC>
// <VERS>VERSION: 0.5.1</VERS>
// <WCTX>Packet 1.9.A.followup US-004: lift the 5 foreign-type ConfigSchema baseline entries in this file (String, &str, Option, Vec, Box) into source-side CONFIGSCHEMA-JUSTIFICATION comments so the rationale lives next to the impl rather than only in xtask/data/configschema_baseline.toml.</WCTX>
// <CLOG>0.5.1: PATCH — add CONFIGSCHEMA-JUSTIFICATION comments above the 5 hand-written foreign-type impls (String/&str: kind=primitive-bridge; Option/Vec/Box: kind=derive-cannot-handle-generic-T). No behavior change. Baseline entries removed in same packet.</CLOG>

mod cls_json_writer;
mod cls_schema_registry;
mod fnc_json_write_field_meta;
mod fnc_json_write_range;
mod fnc_json_write_schema_field;
mod fnc_json_write_schema_node;
mod fnc_json_write_schema_variant;
mod fnc_node_to_json_schema;
mod fnc_schema_node_to_json_pretty;
mod fnc_to_json_schema;
mod fnc_to_markdown;
mod fnc_variant_to_tagged_schema;
mod fnc_variant_to_untagged_schema;
mod types;
pub use cls_schema_registry::{SchemaRegistry, global_registry};
pub use fnc_to_json_schema::to_json_schema;
pub use fnc_to_markdown::to_markdown;
pub use types::{
    ConfigSchema, FieldMeta, Range, ScalarValue, SchemaField, SchemaNode, SchemaVariant,
};
macro_rules! impl_primitive_schema {
    ($t:ty, $name:expr) => {
        impl ConfigSchema for $t {
            fn schema() -> SchemaNode {
                SchemaNode::Primitive {
                    type_name: $name.to_string(),
                    range: None,
                }
            }
        }
    };
}
macro_rules! impl_int_schema {
    ($t:ty, $name:expr) => {
        impl ConfigSchema for $t {
            fn schema() -> SchemaNode {
                SchemaNode::Primitive {
                    type_name: $name.to_string(),
                    range: Some(Range {
                        min: Some(ScalarValue::number(<$t>::MIN.to_string())),
                        max: Some(ScalarValue::number(<$t>::MAX.to_string())),
                    }),
                }
            }
        }
    };
}
impl_primitive_schema!(bool, "bool");
impl_primitive_schema!(char, "char");
impl_primitive_schema!(f32, "f32");
impl_primitive_schema!(f64, "f64");
impl_int_schema!(i8, "i8");
impl_int_schema!(i16, "i16");
impl_int_schema!(i32, "i32");
impl_int_schema!(i64, "i64");
impl_int_schema!(i128, "i128");
impl_int_schema!(isize, "isize");
impl_int_schema!(u8, "u8");
impl_int_schema!(u16, "u16");
impl_int_schema!(u32, "u32");
impl_int_schema!(u64, "u64");
impl_int_schema!(u128, "u128");
impl_int_schema!(usize, "usize");
// CONFIGSCHEMA-JUSTIFICATION: primitive-bridge: String is a foreign std type with no MIN/MAX; the schema is a SchemaNode::Primitive literal that orphan rules forbid us from deriving on the foreign type itself.
impl ConfigSchema for String {
    fn schema() -> SchemaNode {
        SchemaNode::Primitive {
            type_name: "String".to_string(),
            range: None,
        }
    }
}
// CONFIGSCHEMA-JUSTIFICATION: primitive-bridge: &str is a foreign std type; thin str wrapper alongside String. Same orphan-rule blocker.
impl ConfigSchema for &str {
    fn schema() -> SchemaNode {
        SchemaNode::Primitive {
            type_name: "&str".to_string(),
            range: None,
        }
    }
}
// CONFIGSCHEMA-JUSTIFICATION: derive-cannot-handle-generic-T: Option<T> is a foreign std generic; orphan rules forbid #[derive] on foreign types, and the live derive macro does not synthesize T: ConfigSchema bounds. Hand-written bridge into SchemaNode::Option.
impl<T: ConfigSchema> ConfigSchema for Option<T> {
    fn schema() -> SchemaNode {
        SchemaNode::Option {
            inner: Box::new(T::schema()),
        }
    }
}
// CONFIGSCHEMA-JUSTIFICATION: derive-cannot-handle-generic-T: Vec<T> is a foreign std generic; same blockers as Option<T>. Hand-written bridge into SchemaNode::Vec.
impl<T: ConfigSchema> ConfigSchema for Vec<T> {
    fn schema() -> SchemaNode {
        SchemaNode::Vec {
            item: Box::new(T::schema()),
        }
    }
}
// CONFIGSCHEMA-JUSTIFICATION: derive-cannot-handle-generic-T: Box<T> is a foreign std generic; same blockers as Option<T>. Hand-written bridge into SchemaNode::Box.
impl<T: ConfigSchema> ConfigSchema for Box<T> {
    fn schema() -> SchemaNode {
        SchemaNode::Box {
            inner: Box::new(T::schema()),
        }
    }
}

// <FILE>tui-vfx-core/src/schema/mod.rs</FILE> - <DESC>Schema model for configuration introspection</DESC>
// <VERS>END OF VERSION: 0.5.1</VERS>
