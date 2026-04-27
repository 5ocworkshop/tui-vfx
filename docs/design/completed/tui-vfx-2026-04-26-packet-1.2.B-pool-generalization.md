<!-- <FILE>docs/design/tui-vfx-2026-04-26-packet-1.2.B-pool-generalization.md</FILE> - <DESC>Implementation packet for buy-once sweep finding 1.2.B (Pool&lt;T&gt; generalization). Self-contained execution brief: current-state audit, step-by-step plan, code sketches, test plan, acceptance criteria, verification commands. Captures that the sweep work landed at commit 8cad7a2 — packet acts as verification + follow-on workbook for the next dev. -->
<!-- <VERS>VERSION: 1.0.0</VERS> -->
<!-- <WCTX>Convert sweep finding 1.2.B (Pool&lt;T&gt; consolidation) into a runnable implementation packet. The handoff doc lists 1.2.B as queued; my OFPF audit shows the consolidation already landed at commit 8cad7a2 with five files moved to recyclebin. Packet documents the verification path, the open ConfigSchema-bound caveat from cls_pool.rs CLOG, and the alias-rustdoc preservation question from sweep §6.3.</WCTX> -->
<!-- <CLOG>1.0.0: initial packet — pre-flight, current-state audit (work landed; verify), step-by-step plan written as if work were re-executable, code sketches matching the in-tree shape, test plan, acceptance criteria, verification commands.</CLOG> -->

# Packet 1.2.B — Pool<T> generalization

> **Source finding.** `docs/design/tui-vfx-buy-once-architecture-sweep.md` §1.2.B (lines 189–232) and §6.3 (lines 645–668).
>
> **Status note (2026-04-26).** OFPF audit at packet-write time confirms the consolidation already shipped at commit `8cad7a2` ("Collapse five sibling pool types into Pool<T> with aliases (1.2.B)"). Five hand-rolled pool files are in `recyclebin/crates/tui-vfx-content/src/pool/`. `cls_pool.rs` exists with the canonical generic. The handoff doc `tui-vfx-2026-04-26-handoff-outstanding.md:20` predates the commit and lists 1.2.B as queued — it is stale. This packet is therefore a **verification + follow-on** workbook: the junior dev confirms the in-tree shape matches this packet, runs the verification commands, and addresses the residual items in §Open architectural questions.
>
> **Risk tier (per sweep).** S — five dependents per pool, mechanical migration.

---

## Goal & motivation

Five sibling pool types in `crates/tui-vfx-content/src/pool/` (`ImagePool`, `TextPool`, `FontPool`, `EffectPool`, `PresetPool`) shared the same `{ items: Vec<T>, policy: PoolPolicy }` shape and the same `new` / `pick` / `is_empty` four-method API. Adding a sixth pool re-paid 100–280 lines of identical scaffolding per type. The consolidation defines one `Pool<T>` and aliases four of the five concrete pools; `TextPool` stays as a thin newtype because its constructor calls `sanitize()` per item — behavior the other four pools do not want and per Intention 24 rule 1 must not be forced into the generic.

## Scope

**In scope.**

- `crates/tui-vfx-content/src/pool/cls_pool.rs` — new generic `Pool<T>`.
- `crates/tui-vfx-content/src/pool/cls_preset.rs` — new sibling for the `Preset` item type so `PresetPool = Pool<Preset>` works as an alias.
- `crates/tui-vfx-content/src/pool/cls_text_pool.rs` — rewritten as a newtype wrapping `Pool<String>`.
- `crates/tui-vfx-content/src/pool/mod.rs` — declares `Pool<T>` + the four type aliases (`ImagePool`, `FontPool`, `EffectPool`, `PresetPool`).
- `recyclebin/crates/tui-vfx-content/src/pool/` — receives the five hand-rolled files (4 retired + the old `cls_text_pool.rs` v0.1.0 if a fresh re-execution).

**Out of scope.**

- Any changes to `tui-vfx-recipes` or `gt-design` consumer call sites. The aliases preserve every public type name; downstream `use tui_vfx_content::pool::ImagePool;` keeps working unchanged.
- `PoolPolicy` (already a single SSOT in `col_pool_policy.rs`).
- `pick_index` (already a single SSOT in `fnc_pick_index.rs`).
- The `cls_filter_spec.rs` size violation (sweep finding 1.6.A — separate packet).

**Crates touched.** Only `tui-vfx-content`. Zero other crates change.

## Pre-work checklist

Run these before reading the rest of this packet. Each command is the orientation pass mandated by the global CLAUDE.md.

```bash
# Daemon health — required before any other ofpf-* call.
ofpf-status
ofpf-stats

# Read the source finding and the architectural decision recommendation.
sed -n '189,232p' /usr/projects/tui-vfx/docs/design/tui-vfx-buy-once-architecture-sweep.md
sed -n '645,668p' /usr/projects/tui-vfx/docs/design/tui-vfx-buy-once-architecture-sweep.md
sed -n '839,930p' /usr/projects/tui-vfx/docs/design/tui-vfx-buy-once-architecture-sweep.md

# Confirm the current in-tree state.
ls /usr/projects/tui-vfx/crates/tui-vfx-content/src/pool/
ls /usr/projects/tui-vfx/recyclebin/crates/tui-vfx-content/src/pool/

# Inspect every file that would be touched. ofpf-inspect is mandatory before edits.
ofpf-inspect crates/tui-vfx-content/src/pool/cls_pool.rs
ofpf-inspect crates/tui-vfx-content/src/pool/cls_text_pool.rs
ofpf-inspect crates/tui-vfx-content/src/pool/cls_preset.rs
ofpf-inspect crates/tui-vfx-content/src/pool/mod.rs

# Blast radius — confirms the alias-only consumer surface story.
ofpf-blast crates/tui-vfx-content/src/pool/cls_pool.rs
```

## Current-state audit

Captured 2026-04-26 directly from the librarian (`graph_loaded: true`, `definition_count: 6702`).

| Path | Role | LOC | Fan-in | Fan-out | Key callers | Key callees |
|---|---|---|---|---|---|---|
| `crates/tui-vfx-content/src/pool/cls_pool.rs` | unit | 153 | 2 | 3 | `pool/mod.rs`, `cls_text_pool.rs` | `col_pool_policy.rs`, `fnc_pick_index.rs`, `tui-vfx-core/src/schema/mod.rs` |
| `crates/tui-vfx-content/src/pool/cls_text_pool.rs` | unit | n/a (newtype) | 1 | 2 | `pool/mod.rs` | `cls_pool.rs`, `col_pool_policy.rs` |
| `crates/tui-vfx-content/src/pool/cls_preset.rs` | unit | n/a | 1 | 1 | `pool/mod.rs` | `crate::types` |
| `crates/tui-vfx-content/src/pool/col_pool_policy.rs` | unit | n/a | many | 0 | the four pool files + downstream | (none) |
| `crates/tui-vfx-content/src/pool/fnc_pick_index.rs` | unit | n/a | 1 | 0 | `cls_pool.rs` | (none) |
| `crates/tui-vfx-content/src/pool/mod.rs` | hub | n/a | many | 5 | crate consumers | the five sibling files |

**Symbol-level call counts.**

- `ofpf-blast crates/tui-vfx-content/src/pool/cls_pool.rs` returns 2 in-crate dependents (`mod.rs`, `cls_text_pool.rs`). Every external consumer reaches `Pool<T>` through the alias names declared in `mod.rs`.
- `ofpf-refs Pool` returns the canonical struct at `cls_pool.rs:37`. No other in-tree symbol shadows the name.
- The four-method API (`new`, `pick`, `is_empty`, plus `Default`) is enumerated at `cls_pool.rs:50–66`.

**Per-file method enumeration (the rule-of-three claim from the sweep doc).** The sweep §6.3 caveat warned that `EffectPool` and `PresetPool` "may have helper methods" beyond the four-method API. Verified against the current tree:

- `cls_pool.rs:50–66` — `new(items, policy)`, `pick()`, `is_empty()`. No other public methods. `Default` derived at line 36.
- `cls_text_pool.rs` — wraps `Pool<String>` and adds nothing public except the `sanitize()` step inside `new()`. Confirms TextPool is the one behavioral exception.
- `cls_preset.rs` — `Preset` carries `with_text`, `with_effect`, etc. constructors; `PresetPool = Pool<Preset>` adds nothing. The constructors live on `Preset`, not on the pool type, so the alias is sound.
- The five recycled files (in `recyclebin/`) carried only the four-method API. The sweep's caveat was over-cautious; nothing beyond the canonical four methods existed.

**This is the load-bearing finding for the rule-of-three claim**: the pools were genuinely identical except for `TextPool::new`'s sanitize step. Consolidation is safe.

## Open architectural questions

These are the §5 questions that block this finding (and one residual carried by the in-tree CLOG).

| # | Question | Recommended default | Source |
|---|---|---|---|
| 3 (sweep §5) | Alias rustdoc preservation. The five hand-rolled pools each carried domain-specific examples. When migrating to `Pool<T>` aliases, do those examples migrate to alias-level `#[doc(...)]`, or distill into one `Pool<T>` example with cross-references? | **Hybrid (Option C from §6.3).** `Pool<T>` carries the canonical four-method usage; each alias carries one `#[doc(...)]` line naming the domain (asset name vs. font name vs. preset bundle). Already in-tree at `pool/mod.rs:64–90` — verify no alias is missing rustdoc. | Sweep §6.3 |
| in-tree | `cls_pool.rs:4` CLOG flags: "Hand-written `ConfigSchema` impl gated on `T: ConfigSchema` (the derive macro does not yet emit the bound)." Should the macro learn to emit generic bounds? | **Defer.** Per Intention 24 rule 1 (real value, current scale) the hand-written impl is ~40 LOC and works. Only one type uses it. Promote to a macro feature when a second generic schema-bearing type lands. | `cls_pool.rs:4`, sweep finding 1.9.A |
| in-tree | `cls_pool.rs` carries `Eq via separate manual impl gated on T: Eq` (line 48). Is the separate impl needed, or can the derive handle it? | **Keep the manual impl.** `#[derive(Eq)]` requires `T: Eq` unconditionally; the separate `impl<T> Eq for Pool<T> where T: Eq` keeps `Pool<T>` instantiable for non-Eq T (e.g. types with floats). Standard pattern; do not change. | `cls_pool.rs:48` |

**Stop-and-ask trigger.** None for this packet. Every question above has a defensible default. If the in-tree state diverges from this packet (e.g. `cls_pool.rs` is missing or has a different shape than §Code snippets describes), surface the divergence to the user before continuing.

## Step-by-step implementation plan

Written as if the consolidation were being re-executed from a clean tree. If the in-tree state already matches this packet (which the audit confirms), the junior runs the verification commands in §Verification commands and stops.

OFPF discipline: edit one file at a time, write the test first (red), implement (green), verify clippy is clean before moving on.

### Step 1 — `cls_pool.rs` (new file, generic + tests)

1. **Pre-edit.** `ofpf-inspect crates/tui-vfx-content/src/pool/col_pool_policy.rs` and `ofpf-inspect crates/tui-vfx-content/src/pool/fnc_pick_index.rs` to confirm the existing primitives.
2. **Write the file.** Use the §Code snippets `Pool<T>` block. Include the inline `#[cfg(test)] mod tests` block (empty pool, FirstOnly, Random, serde roundtrip, Eq).
3. **Metadata envelope.**
   - `<DESC>Generic content-randomization pool. Five sibling pool types collapsed into one type; concrete pools are aliases (ImagePool / FontPool / EffectPool / PresetPool) — TextPool stays as a thin newtype because it sanitizes on construction.</DESC>`
   - `<VERS>VERSION: 0.1.0</VERS>`
   - `<WCTX>Buy-once sweep finding 1.2.B — five hand-rolled pool types collapse into one Pool&lt;T&gt; carrying the canonical { items, policy } shape and { new, pick, is_empty } API. Type aliases preserve the public names consumers import.</WCTX>`
   - `<CLOG>0.1.0: introduce Pool&lt;T&gt; with new / pick / is_empty / Default. Hand-written ConfigSchema impl gated on T: ConfigSchema (the derive macro does not yet emit the bound). Eq via separate manual impl gated on T: Eq.</CLOG>`
4. **Verify (red→green).** `cargo test -p tui-vfx-content cls_pool::tests`.

### Step 2 — `cls_preset.rs` (extract the Preset item)

1. The previous `cls_preset_pool.rs` mixed `Preset` (the item type) with `PresetPool` (the pool wrapper). Extract `Preset` plus its constructors (`with_text`, `with_effect`, `with_image_name`, `with_font_name`) to its own file so `PresetPool = Pool<Preset>` works as a one-line alias.
2. Tests preserved verbatim from the recycled `cls_preset_pool.rs`: partial-text, partial-effect, asset fields, serde-skips-None.
3. **Metadata envelope.** `<DESC>Curated text + effect + image + font bundle authored as a single Preset entry inside PresetPool.</DESC>` `<VERS>VERSION: 0.1.0</VERS>` `<WCTX>Sweep 1.2.B — extract Preset item type so PresetPool can become a Pool&lt;Preset&gt; alias.</WCTX>` `<CLOG>0.1.0: extracted from cls_preset_pool.rs. Body and constructors preserved verbatim.</CLOG>`
4. **Verify.** `cargo test -p tui-vfx-content cls_preset::tests`.

### Step 3 — `cls_text_pool.rs` (rewrite as newtype)

1. Rewrite as `pub struct TextPool(Pool<String>);` with `#[serde(transparent)]` so the wire format `{ items, policy }` is bit-identical to the previous shape.
2. `new(items, policy)` runs the existing `sanitize()` pass (per-item filter that strips control characters / collapses whitespace — see existing implementation, do not re-author the algorithm) before delegating to `Pool::new`.
3. `pick(&self) -> Option<&str>` delegates via `self.0.pick().map(String::as_str)`.
4. `is_empty()` delegates.
5. `Deref<Target = Pool<String>>` for read-only access to `items` and `policy`.
6. **Bump VERS** to 0.2.0 (the file existed at 0.1.0); CLOG entry: `0.2.0: rewritten as newtype over Pool<String>; sanitize-on-construct preserved; serde transparent so wire format unchanged.`
7. **Verify.** `cargo test -p tui-vfx-content cls_text_pool::tests` (existing tests must keep passing unchanged).

### Step 4 — `pool/mod.rs` (declare aliases, remove per-pool re-exports)

1. Replace `pub use cls_image_pool::ImagePool;` etc. with `pub type ImagePool = Pool<String>;` (plus the other three aliases).
2. Each alias carries a one-line `#[doc(...)]` rustdoc naming its domain (per Open question 3 hybrid recommendation).
3. Bump module CLOG to 0.3.0; reference sweep finding 1.2.B in `<WCTX>`.

### Step 5 — Recycle the four hand-rolled files

Per recyclebin protocol from `~/.claude/CLAUDE.md`:

```bash
# From repo root.
mkdir -p recyclebin/crates/tui-vfx-content/src/pool/
mv crates/tui-vfx-content/src/pool/cls_image_pool.rs   recyclebin/crates/tui-vfx-content/src/pool/
mv crates/tui-vfx-content/src/pool/cls_font_pool.rs    recyclebin/crates/tui-vfx-content/src/pool/
mv crates/tui-vfx-content/src/pool/cls_effect_pool.rs  recyclebin/crates/tui-vfx-content/src/pool/
mv crates/tui-vfx-content/src/pool/cls_preset_pool.rs  recyclebin/crates/tui-vfx-content/src/pool/
```

`recyclebin/` is in `.gitignore`; the moves are not committed. The audit confirms the moves already happened.

### Step 6 — Workspace verification

`cargo build --workspace`, `cargo test --workspace`, `cargo clippy --workspace -- -D warnings`. See §Verification commands.

## Code snippets

The exact `Pool<T>` shape that should be in `cls_pool.rs`. This matches the in-tree state at commit `8cad7a2`.

```rust
use serde::{Deserialize, Serialize};
use tui_vfx_core::schema::{ConfigSchema, FieldMeta, SchemaField, SchemaNode};

use super::col_pool_policy::PoolPolicy;
use super::fnc_pick_index::pick_index;

/// A pool of `T` values selected per pick under [`PoolPolicy`].
///
/// Use the type aliases ([`ImagePool`](super::ImagePool),
/// [`FontPool`](super::FontPool), [`EffectPool`](super::EffectPool),
/// [`PresetPool`](super::PresetPool)) for the canonical concrete shapes;
/// [`TextPool`](super::TextPool) is a sibling newtype that adds
/// sanitize-on-construct.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Pool<T> {
    /// Pool entries. Order is preserved; `pick` selects under [`PoolPolicy`].
    #[serde(default = "Vec::new")]
    pub items: Vec<T>,

    /// How [`Pool::pick`] selects an entry.
    #[serde(default)]
    pub policy: PoolPolicy,
}

impl<T> Eq for Pool<T> where T: Eq {}

impl<T> Pool<T> {
    /// Construct a new pool from items and a selection policy.
    pub fn new(items: Vec<T>, policy: PoolPolicy) -> Self {
        Self { items, policy }
    }

    /// Pick one entry under this pool's [`PoolPolicy`]. Returns `None`
    /// when the pool is empty.
    pub fn pick(&self) -> Option<&T> {
        pick_index(self.items.len(), self.policy).map(|idx| &self.items[idx])
    }

    /// True when the pool has no items.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

// Hand-written ConfigSchema. The derive macro at
// crates/tui-vfx-core-macros/src/fnc_impl_config_schema.rs does not yet
// emit a `where T: ConfigSchema` bound for generic structs (sweep
// finding 1.9.A is the queued macro improvement). Until then, gating the
// bound here lets `#[derive(ConfigSchema)]` on downstream structs that
// embed `Pool<String>` / `Pool<Preset>` etc. compile cleanly.
impl<T: ConfigSchema> ConfigSchema for Pool<T> {
    fn schema() -> SchemaNode {
        SchemaNode::Struct {
            name: "Pool".to_string(),
            description: Some(
                "Pool of items with a selection policy (Random / FirstOnly).".to_string(),
            ),
            json_name: None,
            fields: vec![
                SchemaField::new(
                    "items",
                    SchemaNode::Vec { item: Box::new(T::schema()) },
                    FieldMeta { help: Some("Pool entries.".to_string()), ..Default::default() },
                ),
                SchemaField::new(
                    "policy",
                    PoolPolicy::schema(),
                    FieldMeta { help: Some("How pick() selects an entry.".to_string()), ..Default::default() },
                ),
            ],
        }
    }
}
```

The aliases in `pool/mod.rs`:

```rust
mod cls_pool;
mod cls_preset;
mod cls_text_pool;
mod col_pool_policy;
mod fnc_pick_index;

pub use cls_pool::Pool;
pub use cls_preset::Preset;
pub use cls_text_pool::TextPool;
pub use col_pool_policy::PoolPolicy;
pub use fnc_pick_index::pick_index;

use crate::types::ContentEffect;

/// Pool of rocketsplash image asset *names* (strings the caller resolves
/// to `.rss` bytes via an AssetMap). Alias of [`Pool<String>`].
pub type ImagePool = Pool<String>;

/// Pool of rocketsplash font atlas *names*. Same name-not-bytes contract
/// as [`ImagePool`]. Alias of [`Pool<String>`].
pub type FontPool = Pool<String>;

/// Pool of [`ContentEffect`] values rotated per launch. For curated
/// text+effect pairings see [`PresetPool`]. Alias of `Pool<ContentEffect>`.
pub type EffectPool = Pool<ContentEffect>;

/// Pool of curated [`Preset`] bundles. When a `ContentConfig` carries
/// both a `PresetPool` and independent [`TextPool`] / [`EffectPool`]
/// fields, the preset pool wins if non-empty. Alias of [`Pool<Preset>`].
pub type PresetPool = Pool<Preset>;
```

`TextPool` newtype wrapper sketch:

```rust
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TextPool(Pool<String>);

impl TextPool {
    /// Construct a TextPool, sanitizing each input line at build time.
    pub fn new(items: Vec<String>, policy: PoolPolicy) -> Self {
        let sanitized = items.into_iter().map(sanitize).collect();
        Self(Pool::new(sanitized, policy))
    }

    /// Pick one sanitized line under the pool policy.
    pub fn pick(&self) -> Option<&str> {
        self.0.pick().map(String::as_str)
    }

    /// True when the pool has no items.
    pub fn is_empty(&self) -> bool { self.0.is_empty() }
}

impl std::ops::Deref for TextPool {
    type Target = Pool<String>;
    fn deref(&self) -> &Self::Target { &self.0 }
}

fn sanitize(line: String) -> String {
    // Existing implementation — do NOT re-author the algorithm. See the
    // recycled cls_text_pool.rs v0.1.0 for the canonical body.
    todo!("preserve verbatim from previous cls_text_pool.rs")
}
```

## Test plan

### Existing tests that must keep passing unchanged

- `cargo test -p tui-vfx-content cls_pool::tests` — five inline tests in `cls_pool.rs`: empty pool, FirstOnly, Random, serde roundtrip, Pool<u32> Eq.
- `cargo test -p tui-vfx-content cls_preset::tests` — partial-text, partial-effect, asset fields, serde-skips-None.
- `cargo test -p tui-vfx-content cls_text_pool::tests` — every pre-existing TextPool test.
- `cargo test -p tui-vfx-recipes` — every recipe test that uses the pool aliases (the alias names are unchanged, so the test corpus passes without edits).
- `cargo test -p gt-design` (if accessible) — same reasoning.

### New tests added during the migration

Per OFPF every new file gets a paired test surface. In this case the inline `#[cfg(test)] mod tests` block in `cls_pool.rs` is the test surface; no separate `test_cls_pool.rs` file is required because OFPF allows inline tests for `cls_` files. The five tests in §Step 1 cover the four-method API plus serde and Eq.

### TDD red→green sequence

1. Red: `cargo test -p tui-vfx-content cls_pool::tests` — fails because `cls_pool.rs` does not exist.
2. Write `cls_pool.rs` per §Step 1.
3. Green: same command passes.
4. Red: `cargo test -p tui-vfx-content cls_text_pool::tests::sanitize_keeps_existing_lines_intact` (or whatever the existing TextPool test was — preserve verbatim).
5. Rewrite `cls_text_pool.rs` per §Step 3.
6. Green.
7. Red: `cargo build --workspace` — fails because `pool/mod.rs` still re-exports the recycled symbols.
8. Edit `pool/mod.rs` per §Step 4.
9. Green: `cargo build --workspace`.

### Integration test

`cargo test -p tui-vfx-content` covers all five inline test modules. `cargo test --workspace` is the final integration check.

## Acceptance criteria

Binary pass/fail. Every box must be ticked before declaring the packet done.

- [ ] `crates/tui-vfx-content/src/pool/cls_pool.rs` exists with a `Pool<T>` struct carrying `{ items: Vec<T>, policy: PoolPolicy }`.
- [ ] Four type aliases (`ImagePool`, `FontPool`, `EffectPool`, `PresetPool`) declared in `pool/mod.rs`. Each alias carries a `///` rustdoc line naming its domain (per Open question 3 hybrid).
- [ ] `cls_text_pool.rs` is a newtype `pub struct TextPool(Pool<String>)` with `sanitize()` preserved verbatim from the previous implementation.
- [ ] `cls_preset.rs` exists with the `Preset` item type and its constructors (`with_text`, `with_effect`, `with_image_name`, `with_font_name`).
- [ ] Four hand-rolled pool files moved to `recyclebin/crates/tui-vfx-content/src/pool/`. None remain in `crates/tui-vfx-content/src/pool/`.
- [ ] `cargo build --workspace` succeeds with **zero new warnings** (per `feedback_clean_build_no_warnings`).
- [ ] `cargo test --workspace` green.
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` clean.
- [ ] No `#[allow]` suppressions added (per `feedback_no_landmines`).
- [ ] No inert schema fields introduced (per `feedback_no_inert_schema`).
- [ ] Public type names unchanged: every consumer `use tui_vfx_content::pool::{ImagePool, FontPool, EffectPool, PresetPool, TextPool};` continues to work.
- [ ] Wire format preserved: the `{ items, policy }` JSON shape is bit-identical to the previous five files. The `TextPool` newtype uses `#[serde(transparent)]` to preserve this.
- [ ] Rustdoc improved on every public item touched (per `feedback_rustdoc_when_editing`): `Pool<T>` carries the consolidation rationale; each alias carries one domain line.
- [ ] `cargo doc --no-deps` succeeds with no broken intra-doc links.
- [ ] If `cls_pool.rs` or any alias appears in `docs/templates/capabilities.toml`, run `cargo xtask docs generate` and commit any churn to `docs/CAPABILITIES_REFERENCE.md`.

## Verification commands

Copy-paste-ready. Run after every step that touches a file; re-run the full block before declaring done.

```bash
# Build clean across the workspace.
cargo build --workspace

# Per-crate tests for everything 1.2.B touches.
cargo test -p tui-vfx-content
cargo test -p tui-vfx-recipes
cargo test -p tui-vfx-compositor   # downstream — uses pool through the schema bridge
cargo test --workspace             # final integration check

# Clippy with denied warnings — the bar from feedback_clean_build_no_warnings.
cargo clippy --workspace --all-targets -- -D warnings

# Rustdoc clean.
cargo doc --no-deps

# Re-derive the capability manifest (only if pool types appear in the manifest).
cargo xtask docs generate

# Confirm the recyclebin moves are intact.
ls /usr/projects/tui-vfx/recyclebin/crates/tui-vfx-content/src/pool/
```

## Rollback plan

If the consolidation reveals a deal-breaker mid-execution (e.g. a downstream consumer turns out to depend on a method I missed during the §Per-file method enumeration audit, or `#[serde(transparent)]` on the TextPool wrapper changes a wire-format edge case), recover via the recyclebin protocol:

1. Stop. Do not commit.
2. Move the four hand-rolled files back from `recyclebin/crates/tui-vfx-content/src/pool/` to `crates/tui-vfx-content/src/pool/`.
3. Restore `pool/mod.rs` from git (`git restore crates/tui-vfx-content/src/pool/mod.rs`).
4. Delete `cls_pool.rs` and `cls_preset.rs` (or restore from git if a previous version existed).
5. Restore `cls_text_pool.rs` from git.
6. `cargo build --workspace` to confirm the restored state compiles.
7. File a finding in the sweep doc capturing what blocked the consolidation, then surface to the user.

Recyclebin contents are not in git, so the moves are reversible by hand. `git status` should show only the changes in `crates/tui-vfx-content/src/pool/` after rollback.

## Risks & gotchas

- **The `T: ConfigSchema` bound is hand-written.** The macro at `crates/tui-vfx-core-macros/src/fnc_impl_config_schema.rs` does not yet emit a `where T: ConfigSchema` bound for generic structs. The hand-written impl in `cls_pool.rs` carries this bound explicitly. If a future contributor tries to `#[derive(ConfigSchema)]` on `Pool<T>` instead of using the hand-written impl, the build will fail in a confusing way. Keep the hand-written impl until sweep finding 1.9.A lands.
- **`TextPool` is the one non-alias.** Reviewers may flag this as inconsistency. It is intentional — sanitize-on-construct is behavioral and per Intention 24 rule 1 must not be forced into the generic. The CLOG and the `cls_pool.rs` rustdoc both call this out.
- **`#[serde(transparent)]` on TextPool changes Debug output but preserves wire format.** The wrapper is invisible in JSON but visible in `Debug`. Tests that assert on debug output of TextPool will need updating; tests that assert on JSON are unchanged.
- **Five files moved to recyclebin, four lines of alias declarations gain.** The diff is asymmetric (large deletion, tiny addition). Reviewers may suspect missing logic — the `cls_pool.rs` audit in §Per-file method enumeration is the load-bearing evidence that nothing was lost.
- **Downstream `#[derive(ConfigSchema)]` on structs embedding the alias names.** Several types in `tui-vfx-recipes` derive ConfigSchema over fields like `image_pool: ImagePool`. Because `ImagePool` is a type alias, the derive still resolves to `Pool<String>::schema()` via the hand-written impl. Confirmed by `cargo build --workspace` passing.
- **The `TextPool` newtype's `Deref` impl exposes `&Pool<String>`.** Consumers that match on `text_pool.items` instead of going through the public API still work via deref coercion. This is intentional — the newtype is a behavioral wrapper, not an encapsulation boundary.

## Sequencing note

- This packet stands alone. No other queued sweep finding depends on `Pool<T>` landing first.
- The follow-on sweep finding 1.9.A (hand-written `ConfigSchema` audit) is the mechanism that would eventually let `cls_pool.rs` drop its hand-written impl in favor of the derive. Until then the hand-written impl is the load-bearing exception per the file's CLOG.
- Finding 1.2.A (Bindable<T>) and 1.7.A (BindableValue cross-crate home) are the related "generalize the family" packets but operate on the Bindable family, not the Pool family. No cross-dependency.
- The handoff doc `docs/design/tui-vfx-2026-04-26-handoff-outstanding.md:20` should be updated to mark 1.2.B done once a junior dev confirms the audit matches this packet.

<!-- <FILE>docs/design/tui-vfx-2026-04-26-packet-1.2.B-pool-generalization.md</FILE> -->
<!-- <VERS>END OF VERSION: 1.0.0</VERS> -->
