## Verdict

Yes, breaking the aliases and redesigning top-down is a good idea **if you draw a hard boundary between legacy loading and the new canonical model**.

I would not keep dragging legacy aliases into the new architecture. They will poison the SSOT goal. Instead:

```text
Old world:
    many aliases
    effect-specific ad hoc fields
    hidden defaults
    hard-coded runtime knobs
    recipe structure doubles as public API

New world:
    strict v3.1 schema
    one canonical name for every concept
    effect descriptors define capabilities
    recipes bind public parameters to effect inputs
    runtime overrides are layered, typed, and validated
    studio consumes a generated manifest, not raw recipe internals
```

The best migration strategy is not “big bang rewrite everything at once.” It is:

```text
Design v3.1 cleanly.
Build a compiler/validator around it.
Build a v1-to-v3.1 migrator or rewrite recipes manually.
Keep legacy aliases only in a quarantine loader or migration tool.
Do not allow aliases in v3.1.
```

---

# ANSI block diagram

```text
+====================================================================================+
|                           TERMINAL ANIMATION SYSTEM v3.1                              |
+====================================================================================+

  AUTHORING / TOOLS
  ──────────────────────────────────────────────────────────────────────────────────

        +------------------+        +-------------------+        +------------------+
        |  Recipe Author   |        |  Dynamic Studio   |        |   Demo Player    |
        |  writes JSON v3.1  |        |  builds UI from   |        |  scripts time,   |
        |                  |        |  manifest         |        |  signals, input  |
        +---------+--------+        +---------+---------+        +---------+--------+
                  |                           ^                            |
                  | recipe.json               | studio manifest            | demo.json
                  v                           |                            v

  DOCUMENT LAYER
  ──────────────────────────────────────────────────────────────────────────────────

        +------------------+        +-------------------+        +------------------+
        | Recipe Document  |        | Preset / Profile  |        | Runtime Binding  |
        | schema v3.1        |        | overrides         |        | config           |
        +---------+--------+        +---------+---------+        +---------+--------+
                  |                           |                            |
                  +---------------------------+----------------------------+
                                              |
                                              v

  COMPILER / VALIDATION
  ──────────────────────────────────────────────────────────────────────────────────

        +-------------------------------------------------------------------------+
        | Recipe Compiler                                                         |
        |                                                                         |
        |  1. JSON Schema validation                                              |
        |  2. Serde deserialize into RawRecipe                                    |
        |  3. Normalize/default into CanonicalRecipe                              |
        |  4. Semantic validation                                                 |
        |  5. Effect registry validation                                          |
        |  6. Binding/type/range validation                                       |
        |  7. Trigger/phase graph validation                                      |
        |  8. Compile to RuntimeGraph                                             |
        +-------------------------------+-----------------------------------------+
                                        |
                                        v

  SSOT EFFECT REGISTRY / COMPOSITOR
  ──────────────────────────────────────────────────────────────────────────────────

        +-------------------------------------------------------------------------+
        | Effect Registry                                                         |
        |                                                                         |
        |  terminal.typewriter                                                    |
        |      inputs: text, speed, cursor, revealMode, jitter, ...               |
        |      events: started, charEmitted, completed, cancelled                 |
        |                                                                         |
        |  terminal.scanlines                                                     |
        |      inputs: opacity, spacing, phase, color                             |
        |      events: none / completed?                                          |
        |                                                                         |
        |  terminal.glitch                                                        |
        |      inputs: intensity, seed, probability, duration                     |
        |      events: completed                                                  |
        +-------------------------------+-----------------------------------------+
                                        |
                                        v

  RUNTIME INSTANCE
  ──────────────────────────────────────────────────────────────────────────────────

        +------------------+        +-------------------+        +------------------+
        | Parameter Store  |        | Signal Store      |        | Event Bus        |
        | defaults         |        | app/game/studio   |        | node.completed   |
        | preset layer     |        | values            |        | phase.completed  |
        | app bindings     |        |                   |        | user.advance     |
        | live overrides   |        |                   |        |                  |
        +---------+--------+        +---------+---------+        +---------+--------+
                  |                           |                            |
                  +---------------------------+----------------------------+
                                              |
                                              v
        +-------------------------------------------------------------------------+
        | Phase Engine                                                            |
        |                                                                         |
        | enter  --completeWhen-->  dwell  --completeWhen-->  exit                |
        |                                                                         |
        | triggers: time, signal predicates, events, effect completion,            |
        |           all/any/not, latches, windows                                 |
        +-------------------------------+-----------------------------------------+
                                        |
                                        v
        +-------------------------------------------------------------------------+
        | Compositor / Renderer                                                   |
        |                                                                         |
        | terminal cells -> effect nodes -> shade nodes -> composed frame          |
        +-------------------------------+-----------------------------------------+
                                        |
                                        v

  CONSUMING APPLICATIONS
  ──────────────────────────────────────────────────────────────────────────────────

        +------------------+        +-------------------+        +------------------+
        | Game/App Host    |        | Studio Host       |        | CLI Preview      |
        | send signals     |        | set params        |        | render demos     |
        | set params       |        | hot reload        |        | validate packs   |
        | tick runtime     |        | inspect manifest  |        | capture frames   |
        +------------------+        +-------------------+        +------------------+
```

---

# The SSOT rule

Your **Rust effect/compositor layer should be the source of truth for what effects can do**.

Your recipe schema should not independently invent the available fields. It should reference capabilities declared by the effect registry.

The ownership should look like this:

```text
Effect implementation owns:
    effect id
    effect version
    input names
    input value types
    defaults
    ranges
    runtime mutability
    emitted events
    completion semantics

Recipe owns:
    which effect nodes exist
    how public parameters feed effect inputs
    which nodes activate in enter/dwell/exit
    which triggers move phases forward

Preset/session owns:
    parameter overrides
    runtime bindings
    signal mappings

Studio owns:
    presentation of controls
    but not the meaning of controls
```

A useful phrase here:

> SSOT does not mean “all metadata comes from one file.”
> It means every fact has exactly one owner.

For example, “scanline opacity is a number from 0 to 1” should live in the effect descriptor. The recipe may choose to expose a parameter called `crt.scanlineOpacity`, but it should not redefine incompatible rules for opacity.

---

# Recommended Rust crate layout

I would split this into explicit crates/modules:

```text
crates/
  terminal-core/
    cell model
    frame model
    compositor traits
    runtime value model
    event model

  terminal-effects/
    built-in effect implementations
    effect descriptors
    effect registry

  terminal-recipe/
    recipe v3.1 Rust types
    serde models
    validation
    compiler
    schema generation

  terminal-runtime/
    parameter store
    signal store
    trigger engine
    phase engine
    runtime graph execution

  terminal-studio-protocol/
    studio manifest types
    UI control descriptors
    maybe TypeScript generation

  terminal-demo/
    demo document schema
    scripted signals/events
    CLI preview/capture

  xtask/
    generate schemas
    validate recipes
    migrate recipes
    check generated artifacts
```

This gives you a clean dependency direction:

```text
terminal-core
    ↑
terminal-effects
    ↑
terminal-recipe
    ↑
terminal-runtime
    ↑
terminal-demo / terminal-studio-protocol / consuming apps
```

Avoid letting the studio depend on renderer internals. The studio should depend on the **manifest protocol**.

---

# Core vocabulary

Use one vocabulary everywhere.

```text
Effect
    A compositor primitive implemented in Rust.
    Example: terminal.typewriter, terminal.scanlines, terminal.glitch.

Node
    A recipe instance of an effect.
    Example: node "introTypewriter" uses effect "terminal.typewriter".

Input
    A configurable field on an effect node.
    Example: speed, opacity, text, color.

Parameter
    A public recipe-level control.
    Example: content.message, crt.amount, glitch.intensity.

Value source
    A declarative expression that produces a value.
    Example: literal, parameter reference, signal reference, mapped value.

Signal
    A runtime-provided value from the host app, player, game, or studio.
    Example: player.damage, audio.beat, app.message.

Event
    A discrete occurrence.
    Example: node.completed, phase.completed, user.advance.

Trigger
    A condition over time, signals, and events.
    Example: after 2s OR typewriter completed.

Phase
    A lifecycle state of the recipe.
    Required standard phases: enter, dwell, exit.

Binding
    A runtime mapping from a signal/source to a parameter or input.

Preset
    A saved set of parameter overrides.

Manifest
    A compiled, normalized description for tooling and UI.
```

---

# End-to-end data model

## 1. Effect descriptor schema

Every effect should publish a descriptor.

This is the compositor-side contract.

```json
{
  "schemaVersion": "terminal.effectDescriptor.v3.1",
  "id": "terminal.typewriter",
  "version": "2.0.0",
  "category": "content",
  "displayName": "Typewriter",

  "lifecycle": {
    "completion": "eventual",
    "resettable": true,
    "seekable": false,
    "deterministicWithSeed": true
  },

  "inputs": {
    "text": {
      "type": "text",
      "default": "",
      "runtimeMutability": "runtime",
      "bindable": true,
      "ui": {
        "control": "textarea",
        "group": "Content"
      }
    },

    "charsPerSecond": {
      "type": "number",
      "default": 24,
      "min": 1,
      "max": 240,
      "step": 1,
      "unit": "chars/s",
      "runtimeMutability": "runtime",
      "bindable": true,
      "ui": {
        "control": "slider",
        "group": "Timing"
      }
    },

    "cursorEnabled": {
      "type": "boolean",
      "default": true,
      "runtimeMutability": "phaseStart",
      "bindable": true,
      "ui": {
        "control": "switch",
        "group": "Cursor"
      }
    },

    "revealMode": {
      "type": "enum",
      "default": "character",
      "values": ["character", "word", "line", "instant"],
      "runtimeMutability": "phaseStart",
      "bindable": true,
      "ui": {
        "control": "select",
        "group": "Timing"
      }
    },

    "seed": {
      "type": "integer",
      "default": 0,
      "runtimeMutability": "resetOnly",
      "bindable": true,
      "ui": {
        "control": "number",
        "group": "Advanced"
      }
    }
  },

  "events": {
    "started": {
      "payload": "none"
    },
    "charEmitted": {
      "payload": {
        "char": "char",
        "index": "integer"
      }
    },
    "completed": {
      "payload": "none"
    },
    "cancelled": {
      "payload": "none"
    }
  }
}
```

Important fields:

```text
runtimeMutability:
    compileTime   -> cannot change after compile
    phaseStart    -> can change only when node starts/reset
    resetOnly     -> requires node reset
    runtime       -> can change live while running

completion:
    never         -> continuous effects, cannot be waited on
    instant       -> considered complete immediately
    timeBound     -> completes after declared/derived duration
    eventual      -> completes when internal state reaches done
    external      -> completion is driven by external signal/event
```

This matters for triggers. A recipe should not be allowed to wait for `node.completed` on an effect whose descriptor says `completion: never`.

---

## 2. Runtime value schema

Use a closed set of value types.

```json
{
  "schemaVersion": "terminal.value.v3.1",

  "types": [
    "null",
    "boolean",
    "integer",
    "number",
    "string",
    "text",
    "color",
    "duration",
    "vec2",
    "vec3",
    "rect",
    "enum",
    "curve",
    "palette",
    "glyphSet"
  ]
}
```

I would keep recipe JSON values untyped at the raw layer, but compile them into typed values.

Rust-ish shape:

```rust
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", content = "value", rename_all = "camelCase")]
pub enum Value {
    Null,
    Bool(bool),
    Integer(i64),
    Number(f64),
    String(String),
    Text(String),
    Color(Color),
    Duration(DurationSpec),
    Vec2([f64; 2]),
    Vec3([f64; 3]),
    Rect(Rect),
    Enum(String),
    Curve(CurveSpec),
    Palette(PaletteSpec),
    GlyphSet(GlyphSetSpec),
}
```

Use semantic validation after deserialization for things JSON Schema cannot fully express, such as “this enum value must be one of the values declared by this specific effect input.”

---

## 3. Value source schema

Effect inputs should not be raw values only. They should be **value sources**.

```json
{
  "kind": "literal",
  "value": {
    "type": "number",
    "value": 0.8
  }
}
```

```json
{
  "kind": "param",
  "id": "crt.amount"
}
```

```json
{
  "kind": "signal",
  "id": "player.damage",
  "fallback": {
    "type": "number",
    "value": 0
  }
}
```

```json
{
  "kind": "map",
  "from": {
    "kind": "param",
    "id": "crt.amount"
  },
  "input": [0, 1],
  "output": [0, 0.75],
  "curve": "easeInOut"
}
```

Rust-ish shape:

```rust
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum ValueSource {
    Literal {
        value: Value,
    },

    Param {
        id: ParamId,
        fallback: Option<Value>,
    },

    Signal {
        id: SignalId,
        fallback: Option<Value>,
    },

    Map {
        from: Box<ValueSource>,
        input: [f64; 2],
        output: [f64; 2],
        curve: Option<CurveName>,
        clamp: Option<bool>,
    },

    Select {
        selector: Box<ValueSource>,
        cases: BTreeMap<String, ValueSource>,
        default: Box<ValueSource>,
    },
}
```

I would avoid arbitrary scripting expressions at first. Add a safe expression language later only if `literal`, `param`, `signal`, `map`, `select`, and `curve` are insufficient.

---

## 4. Recipe parameter schema

Recipe parameters are the public API of a recipe.

```json
{
  "id": "crt.amount",
  "type": "number",
  "label": "CRT Amount",
  "description": "Overall CRT intensity.",
  "default": 0.55,
  "min": 0,
  "max": 1,
  "step": 0.01,
  "semantic": "ratio",
  "bindable": true,
  "runtime": {
    "smoothingMs": 100,
    "interpolation": "linear",
    "clamp": true
  },
  "ui": {
    "control": "slider",
    "group": "CRT",
    "order": 10
  }
}
```

Distinguish:

```text
Parameter default:
    recipe author’s intended value

Preset override:
    saved user/profile value

Runtime binding:
    live source of value

Runtime override:
    studio/game/app direct set
```

Resolution order:

```text
live override
    >
runtime binding
    >
preset/profile override
    >
recipe default
    >
effect input default
```

---

## 5. Trigger schema

You need a real trigger AST.

The major design issue is that **events are impulses** and **signals are levels**. An `AND` over two signal predicates is straightforward. An `AND` over two instantaneous events is ambiguous unless you define latching/windowing.

So I would define triggers like this:

```json
{
  "kind": "any",
  "children": [
    {
      "kind": "timeElapsed",
      "scope": "phase",
      "duration": "3s"
    },
    {
      "kind": "event",
      "source": {
        "kind": "node",
        "node": "introTypewriter"
      },
      "event": "completed",
      "latch": "phase"
    }
  ]
}
```

An `all` trigger with a latch:

```json
{
  "kind": "all",
  "latch": "phase",
  "children": [
    {
      "kind": "event",
      "source": {
        "kind": "node",
        "node": "introTypewriter"
      },
      "event": "completed"
    },
    {
      "kind": "signalPredicate",
      "signal": "player.ready",
      "op": "eq",
      "value": true
    }
  ]
}
```

A windowed event condition:

```json
{
  "kind": "all",
  "window": "500ms",
  "children": [
    {
      "kind": "event",
      "source": {
        "kind": "node",
        "node": "glitchBurst"
      },
      "event": "completed"
    },
    {
      "kind": "event",
      "source": {
        "kind": "app"
      },
      "event": "confirm"
    }
  ]
}
```

Rust-ish shape:

```rust
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum TriggerExpr {
    All {
        children: Vec<TriggerExpr>,
        latch: Option<LatchMode>,
        window: Option<DurationSpec>,
    },

    Any {
        children: Vec<TriggerExpr>,
    },

    Not {
        child: Box<TriggerExpr>,
    },

    TimeElapsed {
        scope: TimeScope,
        duration: DurationSpec,
    },

    SignalPredicate {
        signal: SignalId,
        op: ComparisonOp,
        value: Value,
        edge: Option<EdgeMode>,
    },

    Event {
        source: EventSource,
        event: EventName,
        latch: Option<LatchMode>,
    },

    NodeCompleted {
        node: NodeId,
        latch: Option<LatchMode>,
    },

    PhaseCompleted {
        phase: PhaseId,
        latch: Option<LatchMode>,
    },

    Manual {
        id: String,
    },
}
```

Where:

```rust
pub enum LatchMode {
    None,
    Phase,
    Recipe,
    Window,
}

pub enum EdgeMode {
    Level,
    Rising,
    Falling,
    Changed,
}

pub enum TimeScope {
    Recipe,
    Phase,
    Node,
}
```

The validation rule should be:

```text
If a trigger combines event leaves with all/and:
    require either latch or window semantics,
    otherwise reject it as ambiguous.
```

That one rule will save you a lot of bugs.

---

## 6. Phase schema: enter/dwell/exit

Use a phase graph, even if the default recipe has exactly three phases.

```json
{
  "phases": {
    "enter": {
      "kind": "enter",
      "activate": [
        {
          "node": "introTypewriter",
          "mode": "runToCompletion",
          "reset": "onPhaseEnter"
        },
        {
          "node": "crtBase",
          "mode": "continuous",
          "reset": "never"
        }
      ],
      "completeWhen": {
        "kind": "nodeCompleted",
        "node": "introTypewriter",
        "latch": "phase"
      },
      "next": "dwell"
    },

    "dwell": {
      "kind": "dwell",
      "activate": [
        {
          "node": "crtBase",
          "mode": "continuous",
          "reset": "never"
        },
        {
          "node": "idleFlicker",
          "mode": "continuous",
          "reset": "onPhaseEnter"
        }
      ],
      "completeWhen": {
        "kind": "any",
        "children": [
          {
            "kind": "manual",
            "id": "exit"
          },
          {
            "kind": "signalPredicate",
            "signal": "app.dismissed",
            "op": "eq",
            "value": true
          }
        ]
      },
      "next": "exit"
    },

    "exit": {
      "kind": "exit",
      "activate": [
        {
          "node": "exitGlitch",
          "mode": "runToCompletion",
          "reset": "onPhaseEnter"
        }
      ],
      "completeWhen": {
        "kind": "nodeCompleted",
        "node": "exitGlitch",
        "latch": "phase"
      },
      "next": null
    }
  }
}
```

This supports the simple lifecycle:

```text
enter -> dwell -> exit -> done
```

But it also leaves room for richer recipes later:

```text
enter -> dwell -> alert -> dwell -> exit
```

without breaking your model.

---

## 7. Canonical recipe document schema

This is the clean v3.1 authoring shape I would aim for.

```json
{
  "$schema": "https://schemas.example.com/terminal.recipe.v3.1.schema.json",
  "schemaVersion": "terminal.recipe.v3.1",

  "id": "neon-boot",
  "version": "2.0.0",
  "name": "Neon Boot",

  "requires": {
    "engine": ">=2.0.0",
    "effects": {
      "terminal.typewriter": ">=2.0.0",
      "terminal.scanlines": ">=2.0.0",
      "terminal.glitch": ">=2.0.0"
    }
  },

  "parameters": {
    "content.message": {
      "type": "text",
      "label": "Message",
      "default": "BOOT SEQUENCE READY",
      "bindable": true,
      "ui": {
        "control": "textarea",
        "group": "Content",
        "order": 10
      }
    },

    "crt.amount": {
      "type": "number",
      "label": "CRT Amount",
      "default": 0.55,
      "min": 0,
      "max": 1,
      "step": 0.01,
      "semantic": "ratio",
      "bindable": true,
      "runtime": {
        "smoothingMs": 120
      },
      "ui": {
        "control": "slider",
        "group": "CRT",
        "order": 20
      }
    },

    "typewriter.speed": {
      "type": "number",
      "label": "Typewriter Speed",
      "default": 24,
      "min": 1,
      "max": 240,
      "step": 1,
      "unit": "chars/s",
      "bindable": true,
      "ui": {
        "control": "slider",
        "group": "Timing",
        "order": 30
      }
    }
  },

  "signals": {
    "player.damage": {
      "type": "number",
      "default": 0,
      "min": 0,
      "max": 1,
      "description": "Optional host-provided damage amount."
    },

    "app.dismissed": {
      "type": "boolean",
      "default": false
    }
  },

  "nodes": {
    "introTypewriter": {
      "effect": "terminal.typewriter",
      "inputs": {
        "text": {
          "kind": "param",
          "id": "content.message"
        },
        "charsPerSecond": {
          "kind": "param",
          "id": "typewriter.speed"
        },
        "cursorEnabled": {
          "kind": "literal",
          "value": true
        }
      }
    },

    "crtBase": {
      "effect": "terminal.scanlines",
      "inputs": {
        "opacity": {
          "kind": "map",
          "from": {
            "kind": "param",
            "id": "crt.amount"
          },
          "input": [0, 1],
          "output": [0, 0.75],
          "curve": "linear",
          "clamp": true
        },
        "spacing": {
          "kind": "literal",
          "value": 2
        }
      }
    },

    "exitGlitch": {
      "effect": "terminal.glitch",
      "inputs": {
        "intensity": {
          "kind": "map",
          "from": {
            "kind": "signal",
            "id": "player.damage",
            "fallback": 0
          },
          "input": [0, 1],
          "output": [0.2, 1],
          "curve": "easeOut",
          "clamp": true
        },
        "duration": {
          "kind": "literal",
          "value": "650ms"
        }
      }
    }
  },

  "phases": {
    "enter": {
      "kind": "enter",
      "activate": [
        {
          "node": "introTypewriter",
          "mode": "runToCompletion",
          "reset": "onPhaseEnter"
        },
        {
          "node": "crtBase",
          "mode": "continuous",
          "reset": "never"
        }
      ],
      "completeWhen": {
        "kind": "nodeCompleted",
        "node": "introTypewriter",
        "latch": "phase"
      },
      "next": "dwell"
    },

    "dwell": {
      "kind": "dwell",
      "activate": [
        {
          "node": "crtBase",
          "mode": "continuous",
          "reset": "never"
        }
      ],
      "completeWhen": {
        "kind": "any",
        "children": [
          {
            "kind": "manual",
            "id": "exit"
          },
          {
            "kind": "signalPredicate",
            "signal": "app.dismissed",
            "op": "eq",
            "value": true
          }
        ]
      },
      "next": "exit"
    },

    "exit": {
      "kind": "exit",
      "activate": [
        {
          "node": "exitGlitch",
          "mode": "runToCompletion",
          "reset": "onPhaseEnter"
        }
      ],
      "completeWhen": {
        "kind": "nodeCompleted",
        "node": "exitGlitch",
        "latch": "phase"
      },
      "next": null
    }
  }
}
```

Important: this v3.1 schema should use strict names only. No aliases.

In Rust, that means using `deny_unknown_fields` on the v3.1 document types.

```rust
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RecipeDocument {
    pub schema_version: SchemaVersion,
    pub id: RecipeId,
    pub version: SemverString,
    pub name: Option<String>,
    pub requires: RequiresSpec,
    pub parameters: BTreeMap<ParamId, ParameterSpec>,
    pub signals: BTreeMap<SignalId, SignalSpec>,
    pub nodes: BTreeMap<NodeId, NodeSpec>,
    pub phases: BTreeMap<PhaseId, PhaseSpec>,
}
```

---

## 8. Preset schema

Presets should not duplicate recipe structure.

```json
{
  "$schema": "https://schemas.example.com/terminal.preset.v3.1.schema.json",
  "schemaVersion": "terminal.preset.v3.1",

  "id": "damaged-monitor",
  "recipe": {
    "id": "neon-boot",
    "version": "^2.0.0"
  },

  "values": {
    "crt.amount": 0.9,
    "typewriter.speed": 12,
    "content.message": "SYSTEM FAILURE"
  }
}
```

A preset is just named overrides.

---

## 9. Runtime binding schema

This is app/session-specific.

```json
{
  "$schema": "https://schemas.example.com/terminal.runtimeBindings.v3.1.schema.json",
  "schemaVersion": "terminal.runtimeBindings.v3.1",

  "bindings": {
    "crt.amount": {
      "target": {
        "kind": "parameter",
        "id": "crt.amount"
      },
      "source": {
        "kind": "signal",
        "id": "player.damage"
      },
      "transform": {
        "kind": "map",
        "input": [0, 1],
        "output": [0.2, 1],
        "curve": "easeInOut",
        "clamp": true
      },
      "mode": "replace",
      "fallback": 0.55
    }
  }
}
```

Supported modes:

```text
replace
add
multiply
min
max
mix
```

For example, `mix` needs an amount:

```json
{
  "mode": "mix",
  "amount": 0.35
}
```

Keep runtime bindings outside the recipe unless the binding is part of the recipe’s intended behavior.

---

## 10. Demo player schema

The demo player should exercise the same runtime API that a real consuming app uses.

```json
{
  "$schema": "https://schemas.example.com/terminal.demo.v3.1.schema.json",
  "schemaVersion": "terminal.demo.v3.1",

  "recipe": {
    "path": "recipes/neon-boot.recipe.json"
  },

  "preset": {
    "path": "presets/damaged-monitor.preset.json"
  },

  "terminal": {
    "cols": 80,
    "rows": 24,
    "fps": 60
  },

  "script": [
    {
      "at": "0ms",
      "setSignal": {
        "id": "player.damage",
        "value": 0
      }
    },
    {
      "at": "1500ms",
      "setSignal": {
        "id": "player.damage",
        "value": 0.8
      }
    },
    {
      "at": "3500ms",
      "emit": {
        "kind": "manual",
        "id": "exit"
      }
    }
  ],

  "capture": {
    "frames": ["0ms", "750ms", "1500ms", "3500ms"],
    "ansiOutput": "captures/neon-boot.ansi"
  }
}
```

The demo player should not have special powers. It should be a normal host app that sends time, signals, manual events, and parameter overrides.

---

## 11. Studio manifest schema

The studio should not parse the raw recipe and infer behavior itself.

It should call:

```rust
let manifest = engine.describe_recipe(&compiled_recipe)?;
```

And receive:

```json
{
  "schemaVersion": "terminal.studioManifest.v3.1",

  "recipe": {
    "id": "neon-boot",
    "version": "2.0.0",
    "name": "Neon Boot"
  },

  "controls": [
    {
      "id": "content.message",
      "label": "Message",
      "type": "text",
      "value": "BOOT SEQUENCE READY",
      "default": "BOOT SEQUENCE READY",
      "bindable": true,
      "source": "recipeDefault",
      "ui": {
        "control": "textarea",
        "group": "Content",
        "order": 10
      },
      "usedBy": [
        {
          "node": "introTypewriter",
          "input": "text"
        }
      ]
    },

    {
      "id": "crt.amount",
      "label": "CRT Amount",
      "type": "number",
      "value": 0.55,
      "default": 0.55,
      "min": 0,
      "max": 1,
      "step": 0.01,
      "semantic": "ratio",
      "bindable": true,
      "source": "recipeDefault",
      "ui": {
        "control": "slider",
        "group": "CRT",
        "order": 20
      },
      "usedBy": [
        {
          "node": "crtBase",
          "input": "opacity"
        }
      ]
    }
  ],

  "signals": [
    {
      "id": "player.damage",
      "type": "number",
      "default": 0,
      "min": 0,
      "max": 1
    },
    {
      "id": "app.dismissed",
      "type": "boolean",
      "default": false
    }
  ],

  "phases": [
    {
      "id": "enter",
      "kind": "enter"
    },
    {
      "id": "dwell",
      "kind": "dwell"
    },
    {
      "id": "exit",
      "kind": "exit"
    }
  ],

  "events": [
    {
      "source": "introTypewriter",
      "event": "completed"
    },
    {
      "source": "exitGlitch",
      "event": "completed"
    }
  ],

  "diagnostics": []
}
```

The studio UI rule becomes straightforward:

```text
boolean          -> switch
number + range   -> slider / knob
number no range  -> numeric field
integer          -> stepper
enum             -> select
color            -> color picker
text             -> textarea
duration         -> duration field
curve            -> curve editor
palette          -> palette editor
glyphSet         -> glyph picker
vec2             -> XY pad or two numeric inputs
```

The studio should be allowed to override presentation, but it should not redefine the underlying value contract.

---

# Validation layers

You want multiple validation passes. Each pass should catch a different class of error.

```text
+----------------------------------------------------------------------------------+
| Layer 0: File / JSON parse                                                       |
|   - valid JSON or JSON5                                                          |
|   - readable file                                                                |
|   - correct encoding                                                             |
+----------------------------------------------------------------------------------+
| Layer 1: JSON Schema validation                                                  |
|   - required fields                                                              |
|   - closed object shapes                                                         |
|   - primitive types                                                              |
|   - basic ranges                                                                 |
|   - no unknown v3.1 fields                                                         |
+----------------------------------------------------------------------------------+
| Layer 2: Serde typed deserialize                                                 |
|   - convert into RawRecipeDocument                                               |
|   - enums are valid                                                              |
|   - durations/colors parse if represented as typed custom strings                |
+----------------------------------------------------------------------------------+
| Layer 3: Canonicalization                                                        |
|   - inject defaults                                                              |
|   - normalize IDs                                                                |
|   - expand shorthand, if you decide to allow any                                 |
|   - produce CanonicalRecipeDocument                                              |
+----------------------------------------------------------------------------------+
| Layer 4: Semantic validation                                                     |
|   - unique IDs                                                                   |
|   - all referenced params/signals/nodes/phases exist                             |
|   - required enter/dwell/exit phases exist                                       |
|   - phase next references are valid                                              |
|   - no impossible phase graph unless intentionally allowed                       |
+----------------------------------------------------------------------------------+
| Layer 5: Effect registry validation                                              |
|   - every effect id exists                                                       |
|   - effect version satisfies recipe requirement                                  |
|   - every node input exists on that effect                                       |
|   - missing inputs have effect defaults                                          |
|   - unknown inputs rejected                                                      |
+----------------------------------------------------------------------------------+
| Layer 6: Type/range/binding validation                                           |
|   - param type compatible with effect input type                                 |
|   - signal type compatible with target                                           |
|   - mapped outputs fit target type/range                                         |
|   - enum values are legal                                                        |
|   - color/duration/glyph values valid                                            |
+----------------------------------------------------------------------------------+
| Layer 7: Trigger validation                                                      |
|   - trigger references valid events/signals/nodes                                |
|   - nodeCompleted only allowed for completable nodes                             |
|   - AND over event impulses requires latch/window                                |
|   - no dead transitions unless explicitly allowed                                |
+----------------------------------------------------------------------------------+
| Layer 8: Runtime policy validation                                               |
|   - runtime bindings target bindable fields only                                 |
|   - runtime mutation respects effect mutability                                  |
|   - smoothing/interpolation valid for value type                                 |
|   - missing signals have fallback or declared default                            |
+----------------------------------------------------------------------------------+
| Layer 9: Runtime validation                                                      |
|   - incoming signal values match declared signal specs                           |
|   - app overrides match parameter specs                                          |
|   - values clamp or reject according to policy                                   |
+----------------------------------------------------------------------------------+
| Layer 10: Render safety validation                                               |
|   - terminal size limits                                                         |
|   - maximum node counts                                                          |
|   - maximum duration if needed                                                   |
|   - bounded allocations                                                          |
|   - deterministic seeds where required                                           |
+----------------------------------------------------------------------------------+
```

JSON Schema catches shape errors. It will not catch enough by itself. The recipe compiler must do semantic validation.

For Rust tooling, `serde` is the natural serialization/deserialization layer; its docs describe the trait-based `Serialize`/`Deserialize` model and derive support. `schemars` can derive JSON Schema from Rust types and aims to describe the same JSON representation that `serde_json` would serialize; it also respects many Serde attributes. `jsonschema` can validate schema documents against their meta-schemas and supports draft-specific validation APIs. ([serde.rs][1])

---

# Migration plan

## Phase 0 — Inventory and naming freeze

Goal: know what exists before designing v3.1.

Actions:

```text
1. List every effect/shade.
2. List every current recipe field.
3. List every alias.
4. List every hard-coded variable.
5. List every runtime-adjustable variable.
6. List every implicit default.
7. List every effect that can complete.
8. List every effect that is continuous forever.
9. List every phase-like behavior already in recipes.
10. List every current trigger concept, even if hard-coded.
```

Deliverables:

```text
docs/v3.1-vocabulary.md
docs/legacy-field-map.md
docs/effect-inventory.md
docs/recipe-v3.1-goals.md
```

Acceptance criteria:

```text
Every existing alias has one of:
    - canonical v3.1 name
    - removed
    - replaced by a different concept

Every hard-coded variable has one of:
    - compile-time constant
    - effect input
    - recipe parameter
    - runtime signal
    - deleted
```

Important decision:

```text
No aliases in v3.1.
```

Keep aliases only here:

```text
legacy loader
migration tool
old compatibility tests
```

---

## Phase 1 — Build the core Rust model

Goal: define the canonical types before touching every effect.

Create the foundational types:

```rust
RecipeId
EffectId
NodeId
ParamId
SignalId
PhaseId
EventName
Value
ValueKind
ValueSpec
ValueSource
TriggerExpr
ParameterSpec
SignalSpec
UiHint
RuntimeHint
Diagnostic
```

Deliverables:

```text
crates/terminal-core/src/value.rs
crates/terminal-core/src/id.rs
crates/terminal-core/src/diagnostic.rs
crates/terminal-recipe/src/schema.rs
```

Rules:

```text
JSON fields use camelCase.
Rust fields use snake_case.
IDs are stable strings.
Display labels are never IDs.
All v3.1 structs use deny_unknown_fields unless there is a deliberate extension point.
```

Acceptance criteria:

```text
cargo test passes
schema generation works
sample minimal recipe deserializes
invalid unknown field fails
```

---

## Phase 2 — Define the effect descriptor contract

Goal: make the compositor/effects the root of truth.

Create a trait like:

```rust
pub trait Effect: Send + Sync + 'static {
    const ID: &'static str;
    const VERSION: &'static str;

    fn descriptor() -> EffectDescriptor;

    fn compile(
        &self,
        node: CompiledNodeConfig,
        ctx: &CompileContext,
    ) -> Result<Box<dyn EffectInstance>, EffectCompileError>;
}
```

And:

```rust
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EffectDescriptor {
    pub id: EffectId,
    pub version: String,
    pub display_name: String,
    pub category: EffectCategory,
    pub lifecycle: EffectLifecycle,
    pub inputs: BTreeMap<InputId, EffectInputSpec>,
    pub events: BTreeMap<EventName, EventSpec>,
}
```

For each effect input:

```rust
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EffectInputSpec {
    pub value_type: ValueKind,
    pub default: Option<Value>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub step: Option<f64>,
    pub unit: Option<String>,
    pub semantic: Option<String>,
    pub bindable: bool,
    pub runtime_mutability: RuntimeMutability,
    pub ui: Option<UiHint>,
}
```

Deliverables:

```text
effect registry
effect descriptor JSON export
one descriptor per existing effect
```

Acceptance criteria:

```text
Every effect has:
    id
    version
    descriptor
    input specs
    event specs
    lifecycle/completion declaration

Every current hard-coded variable is intentionally classified.
```

This is where you ask, effect by effect:

```text
Should this hidden variable become configurable?
Should this be bindable?
Can it change live?
Does changing it require reset?
Does this effect emit completion?
```

---

## Phase 3 — Design canonical recipe v3.1

Goal: strict clean recipe schema.

Build:

```text
RecipeDocument
ParameterSpec
SignalSpec
NodeSpec
PhaseSpec
ActivationSpec
TriggerExpr
```

Deliverables:

```text
schemas/terminal.recipe.v3.1.schema.json
examples/recipes/minimal.recipe.json
examples/recipes/neon-boot.recipe.json
```

Acceptance criteria:

```text
Recipe JSON validates against schema.
Recipe JSON deserializes into Rust.
Unknown fields fail.
Aliases fail.
All examples are canonical.
```

This is the point where you stop designing around old names.

---

## Phase 4 — Build the recipe compiler

Goal: convert JSON into a typed runtime graph.

Pipeline:

```text
Recipe JSON
    -> RawRecipeDocument
    -> CanonicalRecipe
    -> ValidatedRecipe
    -> CompiledRecipe
    -> RuntimeGraph
```

Compiler responsibilities:

```text
resolve ids
resolve defaults
resolve effect descriptors
type-check value sources
type-check parameter bindings
type-check signal references
validate phase graph
validate trigger graph
produce diagnostics
```

Deliverables:

```text
terminal-recipe::compile_recipe()
terminal-recipe::validate_recipe()
terminal-recipe::describe_recipe()
```

Acceptance criteria:

```text
Bad effect id gives diagnostic.
Bad input name gives diagnostic.
Bad parameter reference gives diagnostic.
Bad value type gives diagnostic.
Waiting for non-completable node gives diagnostic.
AND of unlatchable event triggers gives diagnostic.
```

---

## Phase 5 — Implement runtime parameter and signal stores

Goal: make runtime dynamism a first-class system.

Runtime stores:

```text
ParameterStore
    recipe defaults
    preset layer
    runtime binding layer
    live override layer

SignalStore
    latest app/game/studio signals
    defaults
    timestamps
    validity state

EventBus
    emitted events
    latched events
    phase-scoped event history
    windowed event history
```

API sketch:

```rust
pub struct RuntimeInstance {
    pub fn set_param(&mut self, id: &ParamId, value: Value) -> Result<()>;
    pub fn clear_param_override(&mut self, id: &ParamId) -> Result<()>;

    pub fn set_signal(&mut self, id: &SignalId, value: Value) -> Result<()>;
    pub fn emit_manual(&mut self, id: &str) -> Result<()>;

    pub fn tick(&mut self, dt: Duration, frame: &mut TerminalFrame) -> Result<RuntimeEvents>;

    pub fn manifest(&self) -> StudioManifest;
}
```

Acceptance criteria:

```text
Studio slider can update a parameter.
App signal can drive a parameter.
Fallbacks work.
Clamping works.
Smoothing works.
Invalid runtime values fail or clamp according to policy.
```

---

## Phase 6 — Implement phase and trigger engine

Goal: make enter/dwell/exit explicit and testable.

Phase engine behavior:

```text
1. Start at initial phase, usually enter.
2. Activate phase nodes.
3. Tick active nodes.
4. Collect node events.
5. Evaluate phase completeWhen trigger.
6. On complete, transition to next phase.
7. Reset/retain nodes according to activation policy.
8. Emit phase events.
```

Trigger engine must define:

```text
level predicate behavior
event latch behavior
windowed event behavior
time scope behavior
manual event behavior
not/all/any semantics
```

Acceptance criteria:

```text
enter -> dwell happens when typewriter completes.
dwell -> exit happens on manual exit.
dwell -> exit happens on signal app.dismissed.
all trigger with latched event works.
all trigger with unlatchable event is rejected.
timeElapsed scope phase resets when phase changes.
```

---

## Phase 7 — Build the demo player

Goal: demos become integration tests.

Demo player should use the same public runtime APIs as consuming applications.

Deliverables:

```text
terminal-demo CLI
terminal.demo.v3.1.schema.json
demo script runner
ANSI capture output
frame snapshot output
```

Acceptance criteria:

```text
Demo can load recipe.
Demo can apply preset.
Demo can send signals.
Demo can emit manual events.
Demo can capture ANSI frames.
Demo can fail CI on invalid recipe or changed output.
```

---

## Phase 8 — Build the studio manifest and dynamic UI contract

Goal: studio UI is generated from the compiled recipe manifest.

The studio should not know:

```text
hard-coded effect internals
legacy aliases
recipe graph implementation details
private node fields
```

The studio should know:

```text
controls
types
defaults
current values
ranges
groups
visibility rules
bindability
usedBy links
signals
phases
diagnostics
```

Deliverables:

```text
terminal.studioManifest.v3.1.schema.json
StudioManifest Rust type
TypeScript bindings if studio is web/Tauri
control update API
diagnostics API
```

Acceptance criteria:

```text
Studio can render controls from manifest.
Studio can update params.
Studio can reset controls.
Studio can save preset.
Studio can inspect which node/input a control affects.
Studio never parses raw recipe internals to build controls.
```

For TypeScript generation, `ts-rs` is explicitly designed to generate TypeScript declarations from Rust structs so shared data structures can stay in one place. Specta is another Rust-oriented option that focuses on exporting Rust types for type-safe communication across a stack. ([docs.rs][2])

---

## Phase 9 — Migrate or rewrite recipes

Goal: get all authored content into canonical v3.1.

You have two acceptable paths.

### Path A: automated migrator

```text
v1 recipe
    -> LegacyRecipeDocument
    -> MigratedRecipeDocument
    -> canonical v3.1 JSON
    -> validate
    -> compile
```

The migrator should produce a report:

```text
renamed field "shade" -> "effect"
renamed "speedMs" -> "duration"
removed alias "fg"
inserted default "cursorEnabled": true
manual review required for "delay" because meaning was ambiguous
```

### Path B: manual rewrite

Since you said you do not mind rewriting recipes, this is probably cleaner.

Still build a validator first, then rewrite recipes against the strict schema.

Acceptance criteria:

```text
All recipes validate as v3.1.
All recipes compile.
No v3.1 recipe uses old aliases.
Legacy parser is removed from app runtime or feature-gated.
```

My recommendation:

```text
Use manual rewrite for quality.
Build a small legacy scanner anyway to catch old fields and suggest v3.1 names.
```

---

## Phase 10 — CI and release gates

Goal: keep everything synchronized.

CI should run:

```text
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features

cargo xtask schema --check
cargo xtask effect-manifest --check
cargo xtask validate-recipes recipes/**/*.json
cargo xtask validate-presets presets/**/*.json
cargo xtask validate-demos demos/**/*.json
cargo xtask compile-recipes recipes/**/*.json
cargo xtask render-demo-snapshots demos/**/*.json
```

`cargo xtask schema --check` should:

```text
1. Generate schemas from Rust types into a temp directory.
2. Compare generated files with checked-in schemas.
3. Fail if generated schemas differ.
```

`cargo xtask validate-recipes` should:

```text
1. Validate JSON against generated JSON Schema.
2. Deserialize with Serde.
3. Compile semantically against effect registry.
4. Fail with rich diagnostics.
```

Also add tests for:

```text
invalid unknown field
invalid effect id
invalid input id
wrong value type
missing required phase
bad trigger reference
nodeCompleted on non-completable effect
ambiguous event AND trigger
runtime binding to non-bindable input
runtime mutation of compile-time input
```

---

# Autogenerating schemas and keeping them in sync

Use Rust types as the SSOT for machine schemas.

Recommended stack:

```text
Serde:
    JSON serialization/deserialization

Schemars:
    JSON Schema generation from Rust types

jsonschema crate:
    schema validation and meta-schema validation

ts-rs or Specta:
    TypeScript bindings for studio, if needed

xtask:
    repeatable generation/check commands
```

The key is:

```text
Do not hand-maintain JSON Schema files.
Generate them from Rust model types.
Check generated schemas into the repo.
Fail CI if generated schemas are stale.
```

Schemars supports deriving JSON Schema and can use Serde attributes so the generated schema matches the Serde JSON representation; it also supports schema metadata such as title/description and can use doc comments as generated schema descriptions. That makes Rust doc comments useful for human-facing schema docs, but I would still treat Rust types plus derive attributes as the SSOT, not rustdoc itself. ([docs.rs][3])

Example:

```rust
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[schemars(title = "Terminal Recipe Document")]
pub struct RecipeDocument {
    /// Schema version. Must be "terminal.recipe.v3.1".
    pub schema_version: String,

    /// Stable recipe id.
    pub id: RecipeId,

    /// Semver-compatible recipe version.
    pub version: String,

    pub parameters: BTreeMap<ParamId, ParameterSpec>,
    pub signals: BTreeMap<SignalId, SignalSpec>,
    pub nodes: BTreeMap<NodeId, NodeSpec>,
    pub phases: BTreeMap<PhaseId, PhaseSpec>,
}
```

Schema export:

```rust
pub fn export_recipe_schema(path: &Path) -> anyhow::Result<()> {
    let schema = schemars::schema_for!(RecipeDocument);
    let json = serde_json::to_string_pretty(&schema)?;
    std::fs::write(path, json)?;
    Ok(())
}
```

CI check:

```rust
pub fn check_generated(path: &Path, generated: &str) -> anyhow::Result<()> {
    let existing = std::fs::read_to_string(path)?;

    if existing != generated {
        anyhow::bail!(
            "Generated schema is stale: {}. Run `cargo xtask schema`.",
            path.display()
        );
    }

    Ok(())
}
```

For generated TypeScript, choose one path:

```text
Rust desktop/native studio:
    no TypeScript needed

Web/Tauri studio:
    generate TypeScript from Rust types using ts-rs or Specta

External app integration:
    publish JSON Schemas and generated docs
```

---

# Effect audit checklist

For every effect, create a small table like this:

```text
Effect: terminal.typewriter

Current hard-coded variables:
    char delay
    cursor blink rate
    cursor glyph
    reveal granularity
    initial delay
    completion delay
    random jitter
    newline behavior
    word wrap behavior
    skip/fast-forward behavior

Should become effect inputs?
    text                  yes
    charsPerSecond        yes
    cursorEnabled         yes
    cursorGlyph           yes
    revealMode            yes
    initialDelay          yes
    completionHold        maybe
    jitterAmount          maybe
    seed                  yes if jitter exists
    wrapMode              yes
    skipMode              yes

Events:
    started
    charEmitted
    lineEmitted
    completed
    skipped
    cancelled

Completion:
    eventual

Runtime mutability:
    text                  phaseStart or runtime? decide
    charsPerSecond        runtime
    cursorEnabled         runtime or phaseStart
    revealMode            phaseStart
    seed                  resetOnly
```

For typewriter specifically, I would ask whether it needs these config options:

```text
text source:
    literal
    parameter
    signal
    content slot

reveal:
    character
    word
    line
    instant
    custom glyph stream

timing:
    charsPerSecond
    perCharacterDelay
    punctuationPause
    newlinePause
    initialDelay
    completionHold
    speedCurve

cursor:
    enabled
    glyph
    blinkRate
    style
    hideOnComplete

layout:
    wrapMode
    maxWidth
    alignment
    preserveWhitespace
    newlinePolicy

control:
    skipToEnd
    fastForwardMultiplier
    pause/resume
    resetOnTextChange

events:
    charEmitted
    wordEmitted
    lineEmitted
    completed
    skipped

randomness:
    jitterAmount
    seed
```

A good rule:

> If a field changes the authored look or timing, it should probably be an input.
> If a field changes algorithmic strategy or memory layout, it may be compile-time config.
> If a field is only an implementation detail, keep it private.

---

# Recommended diagnostics model

Make validation errors structured, not just strings.

```json
{
  "level": "error",
  "code": "recipe.node.input.unknown",
  "message": "Node 'crtBase' uses unknown input 'strengh' for effect 'terminal.scanlines'.",
  "path": "/nodes/crtBase/inputs/strengh",
  "hint": "Did you mean 'strength'?"
}
```

Rust-ish:

```rust
pub struct Diagnostic {
    pub level: DiagnosticLevel,
    pub code: DiagnosticCode,
    pub message: String,
    pub path: Option<JsonPointer>,
    pub hint: Option<String>,
}
```

This is valuable for the studio. The studio can show diagnostics beside the generated UI.

---

# Naming conventions

I would standardize on:

```text
JSON object fields:
    camelCase

Recipe parameter ids:
    dot.namespaced.lowerCamel
    examples:
        crt.amount
        content.message
        typewriter.speed
        scanlines.enabled

Node ids:
    lowerCamel
    examples:
        introTypewriter
        crtBase
        exitGlitch

Effect ids:
    reverse-domain-ish or crate-ish namespace
    examples:
        terminal.typewriter
        terminal.scanlines
        terminal.glitch

Events:
    lowerCamel
    examples:
        started
        completed
        charEmitted

Phases:
    lowerCamel
    required:
        enter
        dwell
        exit
```

Avoid:

```text
multiple names for same thing
abbreviations like cfg/conf/config interchangeably
recipe fields named after current implementation details
external bindings to JSON paths
```

---

# What to do with aliases

For v3.1:

```text
Do not support aliases.
Do not support deprecated field names.
Do not support unknown fields.
Do not support magic shorthand unless you truly need it.
```

For migration:

```text
support aliases only in LegacyRecipeDocument
emit warnings
write canonical v3.1
then validate v3.1 strictly
```

Recommended repo structure:

```text
schemas/
  terminal.recipe.v3.1.schema.json
  terminal.preset.v3.1.schema.json
  terminal.demo.v3.1.schema.json
  terminal.effectDescriptor.v3.1.schema.json
  terminal.studioManifest.v3.1.schema.json

recipes/
  *.recipe.json

presets/
  *.preset.json

demos/
  *.demo.json

legacy/
  v1/
    old recipes, if you need to keep them for migration tests
```

The application runtime should eventually only load v3.1.

---

# Key design questions to finish the model

These are the questions I would want answered before locking the schema.

## Compositor and rendering

1. Is the compositor a linear pipeline, a DAG, or layered stacks?
2. Are effects operating on terminal cells, glyph streams, ANSI text, pixel buffers, or some combination?
3. Do effects read/write the same frame in place, or do they produce layers that are blended?
4. Can effects be reordered safely, or is order always semantically important?
5. Do effects need z-index/layering?
6. Are there global frame inputs such as terminal size, time, theme, palette, or viewport?
7. Do you need deterministic playback for tests and demos?

## Effect lifecycle

8. Which effects are continuous and never complete?
9. Which effects complete naturally?
10. Which effects can be cancelled?
11. Which effects can be reset?
12. Which effects can be seeked/scrubbed in the studio?
13. What should happen to an active effect when leaving a phase?
14. Can the same node be active across enter, dwell, and exit?
15. Can one effect’s completion trigger another effect inside the same phase?

## Runtime values and bindings

16. What value types do you actually need: color, duration, vec2, rect, curve, palette, glyph set?
17. Should runtime values clamp automatically or reject invalid values?
18. Should smoothing/interpolation be per parameter, per binding, or per effect input?
19. Can app signals be absent?
20. If a signal is absent, should the recipe use fallback, default, previous value, or error?
21. Can a consuming app bind directly to effect inputs, or only to recipe parameters?
22. Should recipes be allowed to read external signals directly, or should all external input go through parameters?

My recommendation: allow both, but prefer parameters for public customization and direct signals for app/game state.

## Phases and triggers

23. Are enter/dwell/exit always required?
24. Can a recipe have additional phases?
25. Is dwell allowed to be infinite?
26. Should phases transition automatically by default when all run-to-completion nodes finish?
27. Should trigger `all` over events mean “same frame,” “within window,” or “latched since phase start”?
28. Do you need repeating timers or only one-shot timers?
29. Do you need trigger debouncing?
30. Do you need trigger priority if multiple transitions become true at once?

## Studio

31. Is the studio native Rust, egui, Tauri, web, or something else?
32. Does the studio need hot reload?
33. Does the studio need timeline scrubbing?
34. Does the studio need to edit recipes or only presets?
35. Should the studio expose every parameter, or support author-defined “basic/advanced” groups?
36. Should studio-authored changes save back to recipe defaults or to presets?
37. Should the studio show internal node inputs for debugging?

## Content

38. Is content always text, or do you need structured content blocks?
39. Does content come from app state, localization files, user input, or recipe literals?
40. Do recipes need multiple content slots?
41. Do effects need to react when content changes during dwell?
42. Should typewriter restart when its text changes?

## Security and trust

43. Are recipes trusted, local files only, or user-provided?
44. Should you forbid arbitrary expressions/scripts?
45. Do you need resource limits for recipe packs?
46. Do you need sandboxing for plugin effects?

## Versioning

47. Will effects be versioned independently?
48. Can recipes require effect version ranges?
49. Are old recipes expected to keep working in shipped apps?
50. Should v3.1 be a hard break with a migrator, or should the runtime support v1 and v3.1 side by side temporarily?

---

# The plan in one sentence

Build a strict v3.1 Rust-owned contract system where **effect descriptors define capabilities**, **recipes bind public parameters to effect inputs**, **phases and triggers form a validated state machine**, **runtime bindings layer on top without mutating recipes**, and **the studio consumes a generated manifest instead of reverse-engineering JSON**.

[1]: https://serde.rs/ "Overview · Serde"
[2]: https://docs.rs/ts-rs "ts_rs - Rust"
[3]: https://docs.rs/schemars/latest/schemars/derive.JsonSchema.html "JsonSchema in schemars - Rust"
