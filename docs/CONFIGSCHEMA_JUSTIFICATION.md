<!-- <FILE>docs/CONFIGSCHEMA_JUSTIFICATION.md</FILE> - <DESC>Policy doc for hand-written impl ConfigSchema for X justification requirements</DESC> -->
<!-- <VERS>VERSION: 1.0.0</VERS> -->
<!-- <WCTX>Packet 1.9.A — ConfigSchema justification lint</WCTX> -->
<!-- <CLOG>1.0.0: initial doc — format spec, canonical kinds, baseline allowlist, promotion schedule</CLOG> -->

# Hand-written `impl ConfigSchema` justification policy

Every `impl ConfigSchema for X` block that is written by hand (not emitted by
`#[derive(ConfigSchema)]`) must carry a justification comment that names an
explicit reason the derive macro cannot handle it. This requirement is enforced
by `cargo xtask audit configschema` and by the `just audit-configschema` recipe.

Per Intention 12A: "V3 schema-bearing types must support `ConfigSchema` derivation
or have an explicit reason not to." This doc defines what "explicit reason"
means mechanically.

---

## The rule

A hand-written `impl ConfigSchema for X` is allowed if and only if:

1. The `(file, type)` pair is in the baseline allowlist at
   `xtask/data/configschema_baseline.toml` (grandfathered pre-existing impls), OR
2. The impl is immediately preceded by a `// CONFIGSCHEMA-JUSTIFICATION: <kind>`
   comment (the inline form), OR
3. The impl is preceded by a `///` doc-comment block that contains a line
   starting with `CONFIGSCHEMA-JUSTIFICATION:` (the doc-comment form, preferred
   because it appears in `cargo doc` output).

New impls that satisfy neither condition are flagged by the lint (warn-only until
`2026-07-01`, hard failure after that date — see **Promotion schedule** below).

---

## Justification-comment format

### Inline form (preferred when there is no other doc comment)

```rust
// CONFIGSCHEMA-JUSTIFICATION: derive-cannot-handle-foreign-type: SignalOrFloat
// is defined in the mixed-signals crate; orphan rules forbid #[derive(ConfigSchema)]
// outside the trait's home crate.
impl ConfigSchema for SignalOrFloat {
```

### Doc-comment form (preferred when the impl already has a doc comment)

```rust
/// Hand-written schema for `SignalOrFloat`.
///
/// CONFIGSCHEMA-JUSTIFICATION: derive-cannot-handle-foreign-type
impl ConfigSchema for SignalOrFloat {
```

Both forms are accepted. The lint searches from the line immediately above
the `impl` line upward through adjacent `//` and `///` comment lines.

### Syntax

```
// CONFIGSCHEMA-JUSTIFICATION: <kind>[: <free-prose>]
```

- `<kind>` must be one of the canonical kinds listed below, OR the special
  `Other("...")` escape hatch.
- The `: <free-prose>` suffix is optional but strongly recommended — it helps
  reviewers understand the specific constraint without reading the impl body.

---

## Canonical exception kinds

| Kind | When to use |
|---|---|
| `derive-cannot-handle-foreign-type` | Type lives in another crate; orphan rules forbid `#[derive]`. Examples: `SignalOrFloat`, `SignalSpec`, `EasingType` (all in `mixed-signals`). |
| `derive-cannot-handle-generic-T` | Generic type requiring `ConfigSchema` bounds the macro does not currently emit. Examples: `Option<T>`, `Vec<T>`, `Box<T>`, `Pool<T>`. |
| `derive-cannot-handle-untagged-enum` | Enum uses `serde(untagged)` and the macro does not emit an untagged-walk schema. |
| `derive-cannot-handle-cross-crate-trait-dep` | The type's variants reference another crate's types in a way that breaks the derive's import resolution. Example: `StyleRegion::Role(RoleTag)` — `RoleTag` lives in `tui-vfx-types` which does not depend on `tui-vfx-core`. |
| `intentional-divergence-from-derive-output` | The hand-written impl produces a schema that intentionally differs from what derive would emit. Example: `Color` declares per-channel RGBA ranges that derive does not synthesise. |
| `primitive-bridge` | The type IS a primitive or thin wrapper; the schema is a `SchemaNode::Primitive` literal. Examples: `String`, `&str`. |
| `uninhabited-type` | The type has no constructable values; the schema is an empty or sentinel `SchemaNode::Enum`. Example: `Never`. |
| `Other("...")` | Catch-all escape hatch. **Passes the lint but emits a CI warning** urging the author to file a packet to add a canonical kind. Use when a legitimate hand-write falls outside all canonical kinds above. |

**Unrecognised kinds are hard errors** even in warn-only mode. If the kind you
need is not in the table, use `Other("...")` as a temporary measure and file
a packet to extend the enum.

---

## The baseline allowlist

`xtask/data/configschema_baseline.toml` enumerates the pre-existing hand-written
impls that were in the codebase when the lint landed (packet 1.9.A, 2026-04-26).
Each entry is exempt from the justification-comment requirement.

**No new entries may be added to the baseline file.** New hand-written impls
must satisfy the lint via a justification comment instead. Any change to the
baseline requires explicit code-review approval — the baseline is the only
path to silently allow an unjustified impl.

The followup audit packet (1.9.A.followup) will walk each baseline entry and
either:
- Add a `CONFIGSCHEMA-JUSTIFICATION` comment to the impl (then remove the
  baseline entry), or
- Migrate the impl to `#[derive(ConfigSchema)]` if the derive macro handles
  the case (then remove the baseline entry).

When the baseline is empty, the file will be removed and the lint becomes
universal — every hand-written impl in the workspace will require a justification
comment.

---

## Promotion schedule (warn-only → hard failure)

The lint currently runs in **warn-only mode**: missing-justification impls emit
a warning but do not break the build. The promotion to hard failure is scheduled
for **2026-07-01**.

The warn-only period gives contributors time to learn the format without
blocking PRs. The transition is a one-line change in
`xtask/src/audit/fnc_audit_configschema.rs` (set `WARN_ONLY = false`).

---

## When to expect the derive macro to grow

The canonical kinds above reflect the derive macro's current limitations. When
the same `derive-cannot-handle-X` justification appears repeatedly across the
codebase, that is the signal to extend the macro instead of adding the N+1th
hand-write.

The derive macro lives at:
`crates/tui-vfx-core-macros/src/fnc_impl_config_schema.rs`

Each macro extension is its own packet, motivated by the followup audit's
findings. Do not extend the macro speculatively.

---

## Quick reference

```rust
// Example 1 — foreign type (inline form)
// CONFIGSCHEMA-JUSTIFICATION: derive-cannot-handle-foreign-type: Foo is in crate bar.
impl ConfigSchema for Foo { ... }

// Example 2 — generic wrapper (inline form)
// CONFIGSCHEMA-JUSTIFICATION: derive-cannot-handle-generic-T: macro does not emit ConfigSchema bounds.
impl<T: ConfigSchema> ConfigSchema for Wrapper<T> { ... }

// Example 3 — intentional divergence (doc-comment form)
/// Schema for `Color`.
///
/// CONFIGSCHEMA-JUSTIFICATION: intentional-divergence-from-derive-output: per-channel RGBA ranges.
impl ConfigSchema for Color { ... }
```

<!-- <FILE>docs/CONFIGSCHEMA_JUSTIFICATION.md</FILE> -->
<!-- <VERS>END OF VERSION: 1.0.0</VERS> -->
