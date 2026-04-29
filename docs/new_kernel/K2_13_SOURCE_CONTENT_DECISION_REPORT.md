<!-- <FILE>docs/new_kernel/K2_13_SOURCE_CONTENT_DECISION_REPORT.md</FILE> - <DESC>K2.13 source and content decision report</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>K2.13 schema decision burn-down: settle source/content schema dispositions for debug_recipes evidence.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — record accepted source/content split, descriptor backlog, and command-capture holdback.</CLOG> -->

# K2.13 Source and Content Decision Report

## Decision

Source and content remain separate schema concepts.

```text
Source: produces an initial semantic surface.
Content effect: transforms or emits content over time inside a source/surface.
Graph effect: runs over a surface through mask/sampler/filter/shader/style semantics.
```

Accepted source families:

```text
source.card
source.text
source.ansi
source.image
source.procedural
```

Accepted content descriptor domain:

```text
content.typewriter
content.splitFlap
content.odometer
content.marquee
content.scramble
content.morph
content.redact
content.glyphCascade
content.glyphParticles
```

Command capture is `oracleOnly`: offline authoring evidence, never runtime command execution.

## Implementation

`descriptors/v3.1/packs/primitive.json` now declares source descriptor placeholders for:

- `source.ansi`
- `source.image`
- `source.procedural`

The schema-readiness resolver maps source/content gaps to `descriptorBacklog`, not unresolved schema blockers.

## Remaining backlog

The backlog is descriptor and adapter work, not schema design work:

- content descriptors need concrete input contracts and player evidence.
- source descriptors need real source adapters before canonical fixtures should rely on them.
- command-capture artifacts stay oracle-only until an offline authoring/export packet exists.

<!-- <FILE>docs/new_kernel/K2_13_SOURCE_CONTENT_DECISION_REPORT.md</FILE> - <DESC>K2.13 source and content decision report</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
