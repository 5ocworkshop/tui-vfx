<!-- <FILE>docs/design/tui-vfx-v3-capability-governance-decision.md</FILE> - <DESC>Accepted V3 capability catalog and factory-internal promotion governance decision.</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Record the project-owner-approved governance rules for promoting capabilities to primitives, variants, earned-name compositions, or public schema from factory-internal conventions.</WCTX> -->
<!-- <CLOG>0.1.1: standardize ladder wording and add checklist-level review hooks for factory-internal conventions.</CLOG> -->

# V3 capability governance decision

This document records accepted governance for the V3 capability catalog and for
promoting factory-internal conventions to public schema.

## 1. Capability promotion ladder

### Decision

V3 uses a promotion ladder for capabilities:

1. base primitive
2. variant
3. earned-name composition
4. factory-internal convention
5. deferred

Accepted wording:

> V3 uses a promotion ladder for capabilities: base primitive, variant,
> earned-name composition, factory-internal convention, or deferred. New public
> schema surface requires evidence of reuse or a genuinely new semantic class.
> Public promotions are sticky and later demotion uses deprecation, not silent
> removal.

### Tiers

| Tier | When to use |
|---|---|
| Base primitive | Existing primitives cannot express the behavior; it introduces a genuinely new rendering/math class. |
| Variant | Existing primitive class is right, but a reusable sub-shape is missing. |
| Earned-name composition | Behavior is expressible from primitives, but the parameter bundle encodes reusable design judgment. |
| Factory-internal convention | Shared by fewer than three factories or not yet stable enough for public schema. |
| Deferred | Interesting, but no clear recurring authoring need yet. |

### Promotion rules

- A base primitive requires a clear new semantic class.
- A variant requires at least two real authoring uses.
- An earned-name composition requires repeated design value, usually two to three
  recipes or one flagship use plus clear reuse.
- Public schema promotion is sticky.
- Demotion of public schema uses deprecation and migration guidance, not silent
  removal.
- Use the full ladder terms in authoring docs and capability catalog entries.
  Do not collapse them into generic "primitive" or "variant" language when the
  distinction matters for promotion review.

## 2. Factory-internal promotion process

### Decision

Factory-internal conventions remain private until they hit a rule-of-three
promotion trigger.

Accepted wording:

> Factory-internal conventions remain private until they hit a rule-of-three
> promotion trigger: three factories, or two factories plus a flagship recipe, or
> repeated author demand. Promotion requires design review,
> schema/rustdoc/docs/validator updates, and migration notes. If promoted, it
> becomes sticky public schema; if not, it remains documented factory-internal
> behavior.

### Promotion triggers

Review a factory-internal convention for schema promotion when:

1. it appears in three distinct factories, or
2. it appears in two factories plus one flagship recipe, or
3. recipe authors repeatedly need to control it directly.

### Promotion requirements

Promotion requires:

- clean public name
- reusable schema shape
- migration guidance
- validator support
- rustdocs for public/schema-bearing types and fields
- hand-maintained docs explaining author-facing use
- debug recipe coverage if the behavior is visually observable
- generated docs/schema refresh where applicable

### If not promoted

If the convention does not meet the bar:

- keep it out of public schema
- document it as factory-internal where relevant
- avoid teaching it as an authoring primitive
- leave a review hook in the owning authoring or evaluation doc with the factory
  count and the next trigger, so the next pass can revisit it without creating a
  registry
- revisit when usage evidence changes

## Examples

| Candidate | Likely tier | Reason |
|---|---|---|
| New coordinate/math basis reused across effects | Base primitive or substrate variant | New semantic class or reusable math surface. |
| A new radial pattern used by multiple shaders | Variant | Existing pattern primitive class remains right. |
| A named polished shader recipe with tuned defaults | Earned-name composition | Encodes design judgment over primitives. |
| `text_contrast` used only by highlighter | Factory-internal convention | Not enough reuse yet. |
| Stateful particle simulation with no immediate recipe need | Deferred | Interesting but not yet earned. |

## Plan impact

This resolves the active governance decisions for:

- Open Q #25 — primitive catalog governance
- Open Q #27 — factory payload opacity / promotion process
- capability catalog follow-on work
- authoring-guide governance rules

Implementation remains in the schema, validator, authoring docs, and generated
docs lanes.

<!-- <FILE>docs/design/tui-vfx-v3-capability-governance-decision.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
