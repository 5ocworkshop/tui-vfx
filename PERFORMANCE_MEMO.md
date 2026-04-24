<!-- <FILE>PERFORMANCE_MEMO.md</FILE> - <DESC>Actionable per-frame performance wins for tui-vfx and siblings, scoped to real workloads on this stack today</DESC> -->
<!-- <VERS>VERSION: 1.1.0</VERS> -->
<!-- <WCTX>Phase 1a shipped. Move the buffer pool + shadow scratch + RoleMap generation counter + Arc cache entries out of "Recommended" into "Shipped"; add a deep-dive section that spells out, per optimization, what it does, why, which workload shapes win, and which do not — explicitly including motion animations and rapid cell content changes.</WCTX> -->
<!-- <CLOG>1.1.0: MINOR — document phase-1a landings (Commits 0, A, B, C, D). Add "Phase 1a deep dive" section with per-item What/Why/Where-it-wins/Where-it-does-not-win, covering motion animations and rapid-content workloads. Extend the "Shipped in this session" table with the five new commits. Remove items 1, 2, 3 from "Recommended"; keep 4 (color-math SIMD), 5 (filter prep split), 6 (rayon validators). Refresh the order-of-magnitude summary + priority list.
1.0.0: Full rewrite. Cut speculative SIMD extensions (oscillator/Gaussian/pink-noise batches) since audit found no consumer call sites at a batch size where SIMD pays off. Cut AVX-512 kernel tier and NEON (both have no kernels to land in). Cut AoS→SoA refactor (profile-gated speculation). Kept tui-vfx pooling/caching and color-math SIMD as the recommended body of work; kept offline rayon in recipe validators; referenced the three shipped mixed-signals commits. Net: memo is half as long, twice as pointed.</CLOG> -->

# Performance Memo — tui-vfx and siblings

**To:** tui-vfx maintainer
**Date:** 2026-04-24
**Subject:** Per-frame wins on the compositor critical path — phase 1a shipped, phase 1b / SIMD remaining

This memo tracks actionable compositor performance work. Phase 1a (allocation-kill
on the shadow render path + role-map Arc caching) landed in this session across
five commits. What remains is color-math SIMD (memo item 4), filter prep split
(item 5, still blocked on an in-flight edit elsewhere), and offline rayon in the
recipe validators (item 6).

All tui-vfx recommendations are internal (compositor / types / filters crates) —
no public API changes, no trait changes, no `Send`/`Sync` obligations propagated
to consumers.

---

## Shipped in this session

| Commit | Crate | Change |
|---|---|---|
| `8097e63` | mixed-signals | **AVX2 batch random auto-opts-in on default `cargo build`.** Dropped a compile-time `#[cfg(target_feature = "avx2")]` gate that was excluding the AVX2 code from every build without `RUSTFLAGS=-C target-feature=+avx2`. Runtime `has_avx2()` now does the real gating via `std::arch::is_x86_feature_detected!`. Single portable binary; AVX2 on supporting CPUs, scalar fallback everywhere else. File: `src/math/fnc_fast_random_batch.rs`. 521 tests pass; `test_batch_matches_scalar` confirms bit-for-bit equivalence. |
| `30e0298` | mixed-signals | **AVX-512 F/DQ/VL detection.** Added `avx512f`, `avx512dq`, `avx512vl` fields and matching `has_*()` helpers to `CpuFeatures`. Populated via `is_x86_feature_detected!`. Implication hierarchy (F implies AVX2; DQ/VL imply F) enforced by test. Groundwork only — no kernel consumes these yet; they'll light up automatically when a kernel with an AVX-512 path is added. File: `src/math/fnc_cpu_features.rs`. |
| `d5a07a3` | mixed-signals | **`OnceLock` cleanup.** Replaced the `AtomicU8`-gated `unsafe static mut CACHED_FEATURES` pattern with `std::sync::OnceLock<CpuFeatures>::get_or_init`. Same caching behavior, zero unsafe, edition-2024 compatible (`static mut` becomes a hard error there). Public API unchanged. 522 tests pass. |
| `b06c810` | tui-vfx-compositor | **Resolve pre-existing clippy findings across the compositor.** Seven files: six `is_multiple_of` swaps, one `unwrap_after_is_none` refactor, seven nested-`if-let` blocks collapsed into Rust-2024 let-chains. No behavior change; cleared the clippy baseline so subsequent perf commits read as pure perf diffs. |
| `d166965` | tui-vfx-compositor | **Thread-local `OwnedGrid` pool.** New `cls_grid_pool.rs`: `GridPool` keyed on `(width, height)` with RAII `PooledGrid` guards. 5 inline unit tests. Introduced but not yet used — see `268760b`. |
| `268760b` | tui-vfx-compositor | **Pool the shadow-path working buffers.** `render_pipeline_with_shadow` now checks out two buffers from `GridPool` instead of allocating a fresh `OwnedGrid` and cloning it. The shadow-only snapshot copies via `cells_mut().copy_from_slice()` — single memcpy, no allocation. Steady-state per-shadowed-frame allocations: 2+ → 0 once pool buckets are warm. Memo items 1 + 3. |
| `a5f0fb3` | tui-vfx-types | **`RoleMap::generation()` mutation counter.** New `u64` field, skipped by serde, initialised to zero, bumps on every in-bounds `set`. `pub fn generation(&self) -> u64` exposes it as a cheap cache-invalidation signal. Five new tests cover zero-start, bump-on-set, no-bump-on-OOB, monotonic-on-repeated, preserved-through-clone. Public-API addition; MINOR version bump. |
| `cc6f7c9` | tui-vfx-compositor | **Cache the per-call `Arc<RoleMap>`.** Thread-local `ROLES_ARC_CACHE` keyed on `(source_ptr, generation, width, height)`; `cached_roles_arc()` returns an `Arc::clone` on hit or rebuilds and repopulates on miss. Wired into all three `roles_arc` construction sites: `render_pipeline_with_shadow`, `render_loop`, `render_loop_inspected`. Steady-state per-frame `source_roles.clone()`: full Vec memcpy → zero for workloads that keep a stable `RoleMap` across frames. Memo item 2. |

On the detection-library question: the public `cpufeatures` crate (RustCrypto) was evaluated and rejected — its AVX-512 detection is nightly-only, its MSRV is 1.85 (mixed-signals sits at 1.75), and it has no NEON support. Std's `is_x86_feature_detected!` is stable for every flag we use (AVX-512 family since Rust 1.69) and mixed-signals' 80-line custom module covers the ground cleanly.

---

## Phase 1a deep dive — what each change does and where it pays off

### Buffer pooling on the shadow render path (memo items 1 + 3)

**What we did.** `cls_grid_pool.rs` introduces a thread-local `GridPool` that hands
out `OwnedGrid` buffers keyed on their exact `(width, height)` and takes them back
when the caller's `PooledGrid` guard drops. `render_pipeline_with_shadow` now
checks out two buffers per call — one for the shadow + element composite, one
for the post-shadow snapshot — instead of allocating fresh with
`OwnedGrid::new(...)` and cloning the first into the second. The snapshot copy
is now `cells_mut().copy_from_slice(buffer.cells())`: a single memcpy, no
allocation.

**Why.** Before the change, each shadowed render allocated two `Vec<Cell>` of
`width × height` elements. At 80×24 that's ~7.6 KB per buffer, ~15 KB per frame;
at 60 fps with shadows on several elements, steady-state allocation pressure sat
in the low-MB/s range, and the allocator's per-call jitter showed up as
frame-time variance. Pooling turns both buffers into once-allocated, many-times-
reused storage.

**Where it pays off.**

- **Motion animations over a stable render extent.** Content traveling along a
  path (arc, bezier, spiral, spring) at 60 fps while the element dimensions
  stay fixed: every frame hits the same pool bucket. Pool warms in one frame,
  steady state is zero allocations from the shadow path. This is the canonical
  best case for the pool.
- **Long-lived shadowed recipes at 60 fps.** Splash surfaces, dashboard panels,
  card components — anything composing with `shadow_spec` set and a stable
  element box. Pool hit rate is 100% after the first frame.
- **Any repeated call at the same `(ext_width, ext_height)`.** Pool keys are
  exact. Same extent → same bucket → same reuse.

**Where it does not pay off.**

- **Non-shadow recipes.** `render_pipeline` short-circuits to the non-shadow
  path when `options.shadow.is_none()`, and that path writes directly into
  `destination.grid_mut()` without an intermediate `OwnedGrid`. No allocation
  to avoid; GridPool is never reached.
- **Resize animations or scrolling layouts that change `(ext_width, ext_height)`
  every frame.** Each distinct extent opens its own pool bucket. The pool grows
  by one `OwnedGrid` per distinct extent ever seen until it tops out at the
  app's set of extents. An app that cycles through a near-continuous range
  (e.g. a height that sweeps through every integer during an animation) defeats
  the pool by construction — each frame's extent is a fresh bucket, and the
  cached entries for prior extents accumulate unused. In that case the pool is
  no worse than the previous behavior but no better either; memory grows
  linearly in the range of extents visited.
- **Single-shot render calls.** Startup splashes, test fixtures, one-shot
  offscreen exports — the pool is cold on the single call, so the fresh
  `OwnedGrid::new` still happens. No harm, no win.

### RoleMap generation counter + `Arc<RoleMap>` cache (memo item 2)

**What we did.** `RoleMap` gained a monotonic `generation: u64` that bumps on
every in-bounds `set`, with `pub fn generation()` as the public accessor. The
compositor's render pipeline now runs every call through a thread-local
`ROLES_ARC_CACHE`: on cache hit (same `source_ptr`, same `generation`, same
dimensions), the cached `Arc<RoleMap>` is cloned — a single atomic refcount
bump. On miss, the pipeline builds a fresh `Arc::new(source_roles.clone())` and
stores it. The cache covers all three per-call `roles_arc` sites —
`render_pipeline_with_shadow`, `render_loop`, `render_loop_inspected` — so
every shader-context construction path benefits.

**Why.** Before the change, each render call paid for a full `Vec<RoleId>`
clone — 1920 × `u32` = 7.5 KB at 80×24 — plus an `Arc` allocation. The typical
workload holds a long-lived `SemanticScene` across frames and mutates its
`RoleMap` rarely (role assignments usually track semantic state, not tick
state); cloning the whole map every tick is pure waste.

**Where it pays off.**

- **Motion animations with stable semantics.** A button that pulses via a
  gradient shader or moves along a spring path: the per-frame inputs are
  sampler / shader parameters, not the role map. Cache hits every frame after
  the first; the 7.5 KB memcpy collapses to one atomic refcount bump.
- **Recipes where roles are bound at load time.** Theme-role cells
  (`Scope::Role("primary")`) resolve once at recipe load and stay put. Per-
  frame `RoleMap.set` calls: zero. Cache hits every frame.
- **Dashboards with live data.** Cell content changes (text values, numeric
  updates) but the ROLE assignments are typically stable — "these cells are
  always `Data`, those cells are always `Label`." The content mutations hit
  the grid, not the role map; the Arc cache hits.

**Where it does not pay off.**

- **Rapid cell content changes that also retag per-cell roles.** A typewriter
  effect that writes `RoleTag::Text` on every newly-landed character
  (`role_map.set((x, y), Text)` per tick) bumps the generation each frame →
  cache misses every frame → full clone every frame. Item 2 gives zero lift
  on this shape. Item 1 still wins (the render extent is unchanged), so
  pooling is independent of this — the two optimizations degrade separately.
- **Fresh `SemanticScene` per frame.** If the consumer builds and drops the
  scene every tick, `source_ptr` changes every call → cache miss every call.
  Zero lift, but also zero harm beyond the original allocation.
- **Inter-thread handoff of the same map.** The cache is thread-local. A map
  rendered by thread A then by thread B forces B to rebuild its own cache
  entry on the first frame. Within either thread, steady state still applies.
- **Pathological pointer reuse.** If a `RoleMap` is dropped and a new one is
  allocated at the same address with coincidentally matching `(generation,
  width, height)`, the cache returns a stale `Arc`. Practically unreachable
  under normal `SemanticScene` ownership; the `cached_roles_arc` comments
  document the assumption.

### Compositor clippy cleanup (Commit 0)

**What we did.** Resolved 15 pre-existing clippy errors across 7 compositor
files: six `x % n == 0` → `x.is_multiple_of(n)` swaps, one `is_none()` + `unwrap()`
refactor into a `match`, seven nested `if-let` blocks collapsed into Rust-2024
let-chains. No behavior change.

**Why.** The compositor crate did not pass `cargo clippy -- -D warnings`
before phase 1a started. Landing perf commits on a dirty clippy baseline
surfaces unrelated errors in each diff and makes the perf changes harder to
review. Cleanup first, then perf.

**Where it pays off.** Every future `cargo clippy -p tui-vfx-compositor`
invocation. Zero runtime impact.

---

## Recommended work (remaining)

### 4. Vectorize color blending and alpha compositing

tui-vfx currently uses zero SIMD. The clearest downstream wins are in pure
color math that runs many times per frame and has no trait-object or pointer-
chasing hazards — vectorizable without touching the `Grid` abstraction.

**4a. `blend_colors` and RGB↔HSL conversion** — `crates/tui-vfx-style/src/utils/fnc_blend_colors.rs:11-48`. Called many times per frame for gradients, tint ramps, and shader-layer interpolation. Add a batch form `blend_colors_batch(c1s, c2s, ts, space, out)` that processes 8 colors per AVX2 iteration. Expected: **3–4× on the blend math.**

**4b. `Color::blend_over`** — `crates/tui-vfx-types/src/color.rs:119-139`. Porter-Duff "over" per RGBA channel; called per shadow cell and per tint/greyscale filter application — thousands of calls per frame on a dense grid. Add `blend_over_batch(fg, bg, out)` packing 8 RGBA quads per AVX2 iteration. Expected: **3.5–4× on the blend math.**

**4c. BT.601 luminance (greyscale filter)** — `crates/tui-vfx-compositor/src/filters/cls_greyscale.rs`. `0.299·R + 0.587·G + 0.114·B`. Trivial f32x8 vectorization, ~**2×**. Worth doing alongside 4b since both reuse the same batch infrastructure.

**Combined effect:** compositing is ~10–15 % of frame time on dense renders; vectorizing 4a + 4b + 4c recovers **~5–10 % of the 16.7 ms budget** on compositing-heavy frames.

**Dispatch pattern (mandatory; same shape as the shipped mixed-signals fix):**

```rust
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn blend_over_batch_avx2(fg: &[Color], bg: &[Color], out: &mut [Color]) { ... }

fn blend_over_batch(fg: &[Color], bg: &[Color], out: &mut [Color]) {
    #[cfg(target_arch = "x86_64")]
    {
        if std::arch::is_x86_feature_detected!("avx2") {
            // SAFETY: CPU support verified at runtime.
            unsafe { return blend_over_batch_avx2(fg, bg, out); }
        }
    }
    blend_over_batch_scalar(fg, bg, out);
}
```

Cache `is_x86_feature_detected!` behind a `OnceLock<bool>` (or copy mixed-signals' `fnc_cpu_features.rs` pattern) so the check is a single atomic load per call, not a `cpuid` hit. Never annotate the AVX2 function with `#[inline]` into scalar callers — that can leak AVX2 instructions into code that runs on non-AVX2 CPUs (UB). Add a reference-equivalence test per kernel (`scalar(x) == simd(x)` within f32 tolerance) so the two paths can't silently diverge.

### 5. Pre-compute the static half of `prepare_filters`

`orc_render_pipeline.rs`. For most recipes the filter list is fixed for the lifetime of the recipe — only the `PrepareContext` (time) changes. Split filter preparation into a static pass (filter kind, parameters, cached once at bind time) and a per-frame pass (time-dependent state only).

**Status:** blocked at phase 1b entry on `cls_prepared_filter.rs` being edited by another worker. Resume when that file settles. Bundle the change with a small extension to `PreparedFilter` to separate bind-time from per-frame state.

### 6. Parallelize the offline recipe validators (tui-vfx-recipes)

`tools/recipe-validator`, `tools/pipeline-validator`, `tools/recipe-probe` do per-recipe batch work with no runtime determinism contract. A `rayon::par_iter` across the recipe list gives near-linear speedup on multi-core dev machines.

Applies **only** to these CLI tools. The library's runtime evaluation path stays single-threaded — scene composition is per-anchor sequential with an embedded determinism contract (`ProceduralSource` trait explicitly forbids interior mutability via `Mutex`/`RwLock`/`Atomic*`/`OnceCell`), and that's load-bearing for recipe semantics.

**Effect:** 8–16× on batch validation in CI/dev loops. Developer ergonomics, not runtime.

---

## Consumer audit — what this memo deliberately does *not* recommend

The first draft of this memo carried recommendations for mixed-signals SIMD extensions (oscillator batch fills, Gaussian Box-Muller, pink-noise octave summation) and a tui-vfx AoS→SoA render-loop refactor. The consumer audit ruled them out for today's workloads:

- **No downstream crate calls mixed-signals' `sample_into`, oscillator types, or noise generators.** Grep across all three consumer repos (tui-vfx, tui-vfx-recipes, gt-design) returned zero matches. Mixed-signals is pulled in for API glue (`SignalContext`, `SignalOrFloat`, `Phase`, `EasingType`), not hot compute.
- **The one concrete compute call site** (`crates/tui-vfx-compositor/src/samplers/cls_crt_jitter.rs:37-45`) uses scalar `fast_random` at per-row granularity — 24 values per frame on an 80×24 grid. That's far below the break-even point for batch SIMD; the scalar path is already optimal there.
- **Consumer batches are per-row / per-effect, not per-buffer.** SIMD wants 100+ elements per call to amortize setup; the TUI stack's per-frame compute is shaped the opposite way.

Accordingly:

- **Mixed-signals oscillator / Gaussian / pink-noise SIMD** — not recommended today. No consumer. Revisit if a future effect generates per-cell random/noise across a full grid (full-screen grain, dither, stochastic dissolves, etc.); at that point `fast_random_batch` (already AVX2-accelerated, auto-opt-in shipped) is a drop-in building block.
- **FMA wiring in mixed-signals** — not a standalone piece of work. `has_fma()` detection is live; consumption is coupled to future AVX2 float kernels. Scalar FMA retrofit across every `sample_into` / filter tap would require wrapping ~12 hot functions with `#[target_feature(enable = "fma")]` + runtime dispatch — the effort/reward is worse than shipping a SIMD kernel with FMA baked in, which we aren't prioritizing anyway.
- **AVX-512 kernel tier** — detection shipped as groundwork (`30e0298`); no kernel to apply it to. Will activate automatically on supported CPUs whenever a kernel with an AVX-512 path is written.
- **aarch64 NEON** — deferred. In-code TODO at `fnc_cpu_features.rs`'s non-x86_64 branch documents the wiring for when an aarch64 consumer needs SIMD parity. Current aarch64 hosts use the scalar path safely.
- **tui-vfx AoS→SoA render-loop refactor** — profile-gated speculation. Potential 2–4× on per-cell compositor work, but gated behind a significant refactor of the `Grid` abstraction. Pursue only if profiling after (4)–(5) confirms the render loop is still the bottleneck.
- **BMI2 `mulx`** — evaluated and rejected. `mulx` is scalar; the `avx2_mul64` emulation it was flagged to replace is a vector multiply. Different tool for a different job. AVX-512 `vpmullq` (via `avx512dq`) is the correct fix for the AVX2 64-bit-vector-multiply gap; detection is already shipped.
- **SSE4.1 mid-tier** — rejected. Any CPU with AVX2 implies SSE4.1. The only beneficiaries are pre-Haswell (pre-2013) CPUs without AVX2 — too small an audience to double the SIMD maintenance surface.

### gt-design

No recommended changes. Theme directory walks (`crates/gtd-ratatui/src/theme/fnc_list_theme_entries.rs`) are startup-only. The runtime-agnostic public API is a feature to preserve. The braille time-series chart's column-normalize loop is a marginal SIMD candidate (1.5–2×, bounded by scatter-writes through ratatui's buffer cells) — per-cell SIMD work belongs centralized in tui-vfx's compositor, not duplicated in individual widgets.

### tui-vfx-recipes (runtime path)

No recommended changes. Scene composition is thin orchestration (`fill_area` over 1–16 cells, `overlay` per-cell role-tag emission); batches are too small to justify SIMD. The one LRU layer cache (`src/scene/cls_stock_scene_composer.rs:18`, 128 entries behind `parking_lot::Mutex`) is correctly sized and scoped; lock contention is not a concern at current cardinality. Keep hit/miss telemetry visible so the 128-entry limit can be tuned if hit rate drops under real workloads. Only the offline validators recommendation in (6) above applies.

---

## Order-of-magnitude benefit summary

| Item | Status | Expected impact |
|---|---|---|
| AVX2 batch random auto-opt-in (mixed-signals) | **shipped** `8097e63` | ~4× throughput on `fast_random_batch` for every default-built consumer on AVX2 silicon |
| AVX-512 F/DQ/VL detection (mixed-signals) | **shipped** `30e0298` | groundwork; no immediate throughput, lights up future AVX-512 kernels automatically |
| `OnceLock` feature-cache cleanup (mixed-signals) | **shipped** `d5a07a3` | zero perf impact; hygiene + edition-2024 compatibility |
| Compositor clippy cleanup | **shipped** `b06c810` | zero perf impact; clean clippy baseline for the phase-1a perf commits |
| (1)+(3) Buffer pooling + shared scratch (tui-vfx) | **shipped** `d166965` + `268760b` | ~1–2 MB/s allocation pressure → 0 on stable-extent shadowed workloads; jitter compression toward the mean. Zero lift on non-shadow or resize-animation workloads. |
| (2) Role-map `Arc` cache (tui-vfx) | **shipped** `a5f0fb3` + `cc6f7c9` | 1920 B/frame memcpy → 0 for workloads that keep a stable RoleMap across frames. Zero lift for workloads that retag roles every frame (e.g. typewriter-with-role-tag). |
| (4) Color-math SIMD — blend_colors, blend_over, BT.601 (tui-vfx) | recommended | 3–4× on blend math; **~5–10 % of 16.7 ms budget recovered** on compositing-heavy frames |
| (5) Filter pre-compute split (tui-vfx) | recommended; blocked on in-flight edit to `cls_prepared_filter.rs` | small; bundle with file-settling |
| (6) Rayon in recipe validators (tui-vfx-recipes, offline) | recommended | 8–16× on batch validation; dev-ergonomics only, no runtime impact |

Net on phase 1a: the shadowed-recipe per-frame allocation budget drops to zero
in steady state for the canonical motion-animation and long-lived-shadow
workloads. For non-shadow recipes the changes are inert; for workloads that
retag roles every frame, item 2 gives no lift but item 1 still wins. The
mixed-signals AVX2 throughput was already delivering for every default-built
consumer before this session; no further action there.

---

## Priority (remaining)

1. **(4) Color-math SIMD (4a + 4b + 4c)** — pure arithmetic, clean wins, no API changes. Build the detection helper once (copy mixed-signals' `fnc_cpu_features.rs` pattern or factor to a shared crate once a third consumer needs it) and reuse across the three kernels. Single commit per kernel with a reference-equivalence test; start with `blend_over_batch` (hottest).
2. **(5) Filter pre-compute split** — resume once `cls_prepared_filter.rs` has been committed by the current worker editing it.
3. **(6) Rayon in recipe validators** — independent small PR; no runtime impact.
4. **Zero-alloc regression gate bench** — durable guard for the phase-1a wins. A counting `GlobalAlloc` in a dedicated integration test or criterion bench, warm up one frame, assert zero allocations over the next N. Deferred out of Commit B because the render fixture needed `tests/pipeline/test_helpers.rs` which was in-flight with another worker.

Beyond these, revisit only when profiling or a new downstream consumer creates real demand. Everything else previously on the table has been either shipped or explicitly deferred with a documented reason above.

<!-- <FILE>PERFORMANCE_MEMO.md</FILE> - <DESC>Actionable per-frame performance wins for tui-vfx and siblings, scoped to real workloads on this stack today</DESC> -->
<!-- <VERS>END OF VERSION: 1.1.0</VERS> -->
