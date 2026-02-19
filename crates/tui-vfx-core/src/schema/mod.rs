// <FILE>tui-vfx-core/src/schema/mod.rs</FILE> - <DESC>Schema model for configuration introspection</DESC>
// <VERS>VERSION: 0.5.0</VERS>
// <WCTX>OFPF refactoring: extract JSON schema handlers</WCTX>
// <CLOG>Add fnc_node_to_json_schema, fnc_variant_to_tagged_schema, fnc_variant_to_untagged_schema</CLOG>

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
impl ConfigSchema for String {
    fn schema() -> SchemaNode {
        SchemaNode::Primitive {
            type_name: "String".to_string(),
            range: None,
        }
    }
}
impl ConfigSchema for &str {
    fn schema() -> SchemaNode {
        SchemaNode::Primitive {
            type_name: "&str".to_string(),
            range: None,
        }
    }
}
impl<T: ConfigSchema> ConfigSchema for Option<T> {
    fn schema() -> SchemaNode {
        SchemaNode::Option {
            inner: Box::new(T::schema()),
        }
    }
}
impl<T: ConfigSchema> ConfigSchema for Vec<T> {
    fn schema() -> SchemaNode {
        SchemaNode::Vec {
            item: Box::new(T::schema()),
        }
    }
}
impl<T: ConfigSchema> ConfigSchema for Box<T> {
    fn schema() -> SchemaNode {
        SchemaNode::Box {
            inner: Box::new(T::schema()),
        }
    }
}

// <FILE>tui-vfx-core/src/schema/mod.rs</FILE> - <DESC>Schema model for configuration introspection</DESC>
// <VERS>END OF VERSION: 0.5.0</VERS>
