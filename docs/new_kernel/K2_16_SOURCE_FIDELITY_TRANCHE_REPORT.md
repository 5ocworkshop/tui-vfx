# K2.16 source fidelity tranche report

## Implemented this iteration

K2.16 preserves the K2.15 bounded source surface and carries source provenance into render IR. The current source fidelity status is:

- `source.text` and `source.card`: deterministic text rows plus IR provenance.
- `source.ansi`: bounded SGR stripping; not full styled ANSI parity.
- `source.image`: deterministic fallback text with warning; not rasterization.
- `source.procedural`: bounded dots-spinner registry evidence; no plugins or command execution.

## Holdbacks

The player still needs a formal asset resolver boundary and richer ANSI styled-cell extraction before claiming source visual fidelity.
