# `runtime/`

Native compost runtime value resolution lives here.

The resolver consumes canonical v3.1 `ValueSource` values directly. Literal
values are supported now; parameter, signal, graph-value, mapped,
sampled-field, signal-expression, phase-progress, and clock sources are
rejected through one shared diagnostic path until runtime binding support is
implemented.

Rules:

- do not duplicate `ValueSource` matching inside primitives;
- do not introduce compatibility DTOs or legacy-shaped runtime bindings;
- add one small resolver file per newly supported source family.
