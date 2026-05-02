# Memo response — implementer review of `SCHEMA_GAPS_MEMO.md`

Implementer read the memo, verified against contract code, and agreed all
three gaps are schema-level, not shorthand-only. Recording the response and
the architectural question it surfaced.

## Agreed actions

1. **Gap 2 — `Rows { indices }` / `Columns { indices }`.** Mechanical add.
   Low corpus count, high authoring value, avoids graph-node duplication.

2. **Gap 3 — Explicit contract field for shadow edge-crossing.** Add to
   `ShadowSpec`:

   ```rust
   pub enum ShadowEdgeCrossingPolicy { Default, Fade, Preserve }
   pub struct ShadowSpec {
       // ...existing fields...
       pub edge_crossing_policy: Option<ShadowEdgeCrossingPolicy>,
   }
   ```

   Reasoning: corpus author-intent varies (some recipes want `shadow:
   "fade"` while keeping `border: "preserve"`; others want both fade).
   Implicit-runtime detection would lose that signal.

3. **`MaskCombineMode` promotion** — track separately. Not blocking the
   three schema gaps. Likely needed before mask substrate work lands.

## Gap 1 — open architectural sub-question

The implementer agreed channel scoping must be addressed but flagged that
adding `ScopeSpec::Channel { channel }` is not "just add a variant." The
existing `ScopeSpec::matches(input, coordinate_space, role_space) -> bool`
is cell-position-and-role based. Channels are not cells. A casual
implementation either always matches or never matches — neither is right.

Two architectural shapes are on the table:

### Option A — Channel inside `ScopeSpec`

```rust
ScopeSpec::Channel { channel: CellChannel }
```

Requires either:

- Extending `ScopeEvalInput` with `active_channel` so `matches()` knows
  which channel it's evaluating, OR
- Adding a side-channel method like
  `ScopeSpec::write_channel_restriction(&self) -> Option<CellChannel>` that
  the runtime consults outside of `matches()`.

Keeps scope as the single home for "where does this effect apply." Breaks
the matches-is-pure-bool invariant.

### Option B — Separate `write_channel` field on `NodeSpec`

```rust
pub struct NodeSpec {
    pub scope: ScopeSpec,
    pub write_channel: Option<CellChannel>,  // new
    // ...
}
```

Scope answers "which cells." `write_channel` answers "which channel of
those cells." Existing scope evaluation semantics stay intact.

### Implementer's lean

Implementer's framing matches Option B: *"treat channel scope as a
write-channel filter outside boolean cell matching."*

Option B is cleaner from the contract side. The shorthand surface is
identical either way:

```json
"effects": [
  { "filter": "invert", "scope": { "channel": "foreground" } }
]
```

The canonical form changes:

- Option A → `{ scope: { kind: "channel", value: "foreground" }, ... }`
- Option B → `{ scope: { kind: "all" }, writeChannel: "foreground", ... }`

Decision pending. Either is a one-row entry in the alias/expansion table;
no shorthand-surface impact.

## Verification gate (cited from implementer)

> *"The correct fix needs tests proving foreground-only/background-only
> effects leave the other channel unchanged."*

Right gate regardless of which option ships. The 30+ channel-scoped
recipes in the corpus give a strong round-trip evidence base:

1. Take channel-scoped recipes from the corpus citation list (Gap 1
   memo: `scene_layer_full_stack`, `complex_diamond_highlight`,
   `B10_grimoire_incantation_bar_palette`,
   `L06_hygge_lantern_diffusion`, `M11_blueprint_circuit_pulse_info`).
2. Shorthand → canonicalize → load → render.
3. Confirm: foreground-channel scope leaves background pixels unchanged
   from the recipe-base color; background-channel scope leaves foreground
   glyphs unchanged.

Worth landing as a regression test suite alongside the implementation,
not after.

## Status update

| Gap | Status |
|---|---|
| Gap 1 (channel scoping) | Agreed must-do; **architectural sub-question open** (Option A vs B); test gate identified |
| Gap 2 (row/column index sets) | Approved — mechanical add |
| Gap 3 (shadow edge-crossing policy) | Approved — explicit contract field |
| `MaskCombineMode` promotion | Tracked separately |

The shorthand alias and expansion tables are unaffected by the Gap 1 A-vs-B
choice. Both options canonicalize to the same shorthand surface.
