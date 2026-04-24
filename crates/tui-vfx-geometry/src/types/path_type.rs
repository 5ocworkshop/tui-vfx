// <FILE>tui-vfx-geometry/src/types/path_type.rs</FILE> - <DESC>Motion path types with physics integration</DESC>
// <VERS>VERSION: 3.0.0 - 2025-12-31</VERS>
// <WCTX>Architectural roadmap: Physics PathType variants</WCTX>
// <CLOG>Added Projectile, Friction, Orbit, Pendulum variants using mixed-signals physics</CLOG>

use serde::{Deserialize, Serialize};
use tui_vfx_core::{
    ConfigSchema, FieldMeta, Range, ScalarValue, SchemaField, SchemaNode, SchemaVariant,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum PathType {
    Linear,
    /// Compose a geometric carrier route with one or more dynamic treatments.
    ///
    /// The `route` defines the base travel through space. Each entry in
    /// `dynamics` contributes an offset relative to a linear baseline,
    /// allowing shapes such as "pendulum over arc" or "spring over bezier"
    /// without flattening every combination into its own top-level variant.
    Composed {
        route: Box<PathType>,
        #[serde(default)]
        dynamics: Vec<PathType>,
    },
    Arc {
        bulge: f32,
    },
    /// Quadratic Bezier curve with explicit control point.
    ///
    /// Unlike `Arc` which computes the control point from a bulge factor,
    /// `Bezier` allows specifying an arbitrary control point for precise
    /// curve shaping. The control point is typically resolved from a
    /// `via` PlacementSpec at animation setup time.
    ///
    /// Note: This is a spatial quadratic Bezier (single control point) used
    /// for motion paths. It is distinct from `EasingCurve::Bezier`, which is
    /// a cubic Bezier defined in normalized time/value space for easing.
    ///
    /// Example: An arc from mid-left to mid-right via top-center would
    /// have control_x at center and control_y near the top.
    Bezier {
        control_x: f32,
        control_y: f32,
    },
    Spring {
        stiffness: f32,
        damping: f32,
    },
    Bounce {
        bounces: u8,
        decay: f32,
    },
    Squash,
    Hover,
    Rectilinear {
        x_first: bool,
    },
    /// Spirals outward from start to end.
    Spiral {
        rotations: f32,
    },
    /// Quantized movement (stop-motion).
    Step {
        steps: u8,
    },

    // =========================================================================
    // Physics-based paths (using mixed-signals solvers)
    // =========================================================================
    /// Ballistic/projectile motion with gravity.
    ///
    /// Creates a parabolic arc like throwing or tossing an object.
    /// The element follows a realistic physics trajectory from start to end.
    ///
    /// # Parameters
    /// - `arc_height`: Peak height above the straight line (in cells). Negative for upward arc.
    /// - `gravity`: Gravity strength (higher = faster fall). Default: 500.0
    ///
    /// # Example
    /// A toast notification that arcs in from the side would use a negative arc_height
    /// to create an upward-then-down trajectory.
    Projectile {
        arc_height: f32,
        gravity: f32,
    },

    /// Friction/momentum decay for scroll-like motion.
    ///
    /// Element moves with initial momentum and decelerates due to friction.
    /// Creates natural-feeling scroll inertia or slide-to-stop effects.
    ///
    /// # Parameters
    /// - `drag`: Friction coefficient (higher = faster stop). Default: 4.0
    ///
    /// # Note
    /// The path travels from start toward end, decelerating naturally.
    /// At t=1.0, the element will be at or very near the end position.
    Friction {
        drag: f32,
    },

    /// Orbital/circular motion around a center point.
    ///
    /// Element orbits around the midpoint between start and end positions.
    /// Useful for spinner effects or circular reveal animations.
    ///
    /// # Parameters
    /// - `revolutions`: Number of complete orbits (can be fractional). Default: 1.0
    /// - `direction`: 1.0 for counter-clockwise, -1.0 for clockwise. Default: 1.0
    ///
    /// # Example
    /// A loading spinner that orbits once from start to end position.
    Orbit {
        revolutions: f32,
        direction: f32,
    },

    /// Pendulum/swinging motion.
    ///
    /// Element swings like a pendulum, oscillating around the end position.
    /// Useful for hanging notifications or "swing in" effects.
    ///
    /// # Parameters
    /// - `amplitude`: Initial swing amplitude in cells. Default: 30.0
    /// - `oscillations`: Number of back-and-forth swings. Default: 3.0
    /// - `damping`: How quickly oscillations decay (0 = no decay). Default: 2.0
    ///
    /// # Note
    /// The pendulum swings around the end position, settling there at t=1.0.
    Pendulum {
        amplitude: f32,
        oscillations: f32,
        damping: f32,
    },
}

impl ConfigSchema for PathType {
    fn schema() -> SchemaNode {
        fn f32_field(name: &str, description: &str) -> SchemaField {
            SchemaField::new(
                name,
                f32::schema(),
                FieldMeta {
                    description: Some(description.to_string()),
                    ..Default::default()
                },
            )
        }
        fn u8_field(
            name: &str,
            description: &str,
            default: Option<u8>,
            min: Option<u8>,
            max: Option<u8>,
        ) -> SchemaField {
            SchemaField::new(
                name,
                u8::schema(),
                FieldMeta {
                    description: Some(description.to_string()),
                    default: default.map(|v| ScalarValue::number(v.to_string())),
                    range: Some(Range::new(
                        min.map(|v| ScalarValue::number(v.to_string())),
                        max.map(|v| ScalarValue::number(v.to_string())),
                    )),
                    ..Default::default()
                },
            )
        }
        SchemaNode::Enum {
            name: "PathType".to_string(),
            description: Some("Motion path types with physics integration".to_string()),
            json_name: Some("type".to_string()),
            tag_field: Some("type".to_string()),
            variants: vec![
                SchemaVariant::Unit {
                    name: "Linear".to_string(),
                    description: Some("Straight-line motion".to_string()),
                    json_value: Some("linear".to_string()),
                },
                SchemaVariant::Struct {
                    name: "Composed".to_string(),
                    description: Some(
                        "Carrier route plus one or more dynamic treatments".to_string(),
                    ),
                    json_value: Some("composed".to_string()),
                    fields: vec![
                        SchemaField::new(
                            "route",
                            SchemaNode::Opaque {
                                type_name: "PathType".to_string(),
                            },
                            FieldMeta {
                                description: Some("Base carrier route".to_string()),
                                ..Default::default()
                            },
                        ),
                        SchemaField::new(
                            "dynamics",
                            SchemaNode::Vec {
                                item: Box::new(SchemaNode::Opaque {
                                    type_name: "PathType".to_string(),
                                }),
                            },
                            FieldMeta {
                                description: Some(
                                    "Dynamic path treatments layered over the route".to_string(),
                                ),
                                ..Default::default()
                            },
                        ),
                    ],
                },
                SchemaVariant::Struct {
                    name: "Arc".to_string(),
                    description: Some("Arc path with bulge factor".to_string()),
                    json_value: Some("arc".to_string()),
                    fields: vec![f32_field("bulge", "Arc bulge factor")],
                },
                SchemaVariant::Struct {
                    name: "Bezier".to_string(),
                    description: Some("Quadratic bezier motion path".to_string()),
                    json_value: Some("bezier".to_string()),
                    fields: vec![
                        f32_field("control_x", "Bezier control X"),
                        f32_field("control_y", "Bezier control Y"),
                    ],
                },
                SchemaVariant::Struct {
                    name: "Spring".to_string(),
                    description: Some("Spring-like settling path".to_string()),
                    json_value: Some("spring".to_string()),
                    fields: vec![
                        f32_field("stiffness", "Spring stiffness"),
                        f32_field("damping", "Spring damping"),
                    ],
                },
                SchemaVariant::Struct {
                    name: "Bounce".to_string(),
                    description: Some("Bouncy overshoot path".to_string()),
                    json_value: Some("bounce".to_string()),
                    fields: vec![
                        u8_field("bounces", "Number of bounces", Some(3), Some(0), Some(12)),
                        f32_field("decay", "Bounce decay factor"),
                    ],
                },
                SchemaVariant::Unit {
                    name: "Squash".to_string(),
                    description: Some("Squash-like settling path".to_string()),
                    json_value: Some("squash".to_string()),
                },
                SchemaVariant::Unit {
                    name: "Hover".to_string(),
                    description: Some("Hover-like motion treatment".to_string()),
                    json_value: Some("hover".to_string()),
                },
                SchemaVariant::Struct {
                    name: "Rectilinear".to_string(),
                    description: Some("Axis-aligned L-shaped travel".to_string()),
                    json_value: Some("rectilinear".to_string()),
                    fields: vec![SchemaField::new(
                        "x_first",
                        bool::schema(),
                        FieldMeta {
                            description: Some("Whether to travel X before Y".to_string()),
                            ..Default::default()
                        },
                    )],
                },
                SchemaVariant::Struct {
                    name: "Spiral".to_string(),
                    description: Some("Spiral path".to_string()),
                    json_value: Some("spiral".to_string()),
                    fields: vec![f32_field("rotations", "Number of spiral rotations")],
                },
                SchemaVariant::Struct {
                    name: "Step".to_string(),
                    description: Some("Stepped motion".to_string()),
                    json_value: Some("step".to_string()),
                    fields: vec![u8_field("steps", "Number of steps", Some(5), None, None)],
                },
                SchemaVariant::Struct {
                    name: "Projectile".to_string(),
                    description: Some("Ballistic projectile path".to_string()),
                    json_value: Some("projectile".to_string()),
                    fields: vec![
                        f32_field("arc_height", "Projectile arc height"),
                        f32_field("gravity", "Projectile gravity"),
                    ],
                },
                SchemaVariant::Struct {
                    name: "Friction".to_string(),
                    description: Some("Frictional deceleration path".to_string()),
                    json_value: Some("friction".to_string()),
                    fields: vec![f32_field("drag", "Friction drag coefficient")],
                },
                SchemaVariant::Struct {
                    name: "Orbit".to_string(),
                    description: Some("Orbital/circular path".to_string()),
                    json_value: Some("orbit".to_string()),
                    fields: vec![
                        f32_field("revolutions", "Orbit revolutions"),
                        f32_field("direction", "Orbit direction sign"),
                    ],
                },
                SchemaVariant::Struct {
                    name: "Pendulum".to_string(),
                    description: Some("Pendulum oscillation path".to_string()),
                    json_value: Some("pendulum".to_string()),
                    fields: vec![
                        f32_field("amplitude", "Pendulum amplitude"),
                        f32_field("oscillations", "Pendulum oscillations"),
                        f32_field("damping", "Pendulum damping"),
                    ],
                },
            ],
        }
    }
}

// <FILE>tui-vfx-geometry/src/types/path_type.rs</FILE> - <DESC>Motion path types with physics integration</DESC>
// <VERS>END OF VERSION: 3.0.0 - 2025-12-31</VERS>
