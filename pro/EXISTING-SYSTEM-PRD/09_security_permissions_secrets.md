<!-- <FILE>pro/EXISTING-SYSTEM-PRD/09_security_permissions_secrets.md</FILE> - <DESC>Chapter 9 of the evidence-backed Existing-System PRD: security-relevant facts — unsafe usage, network bindings, credential loading, input validation, trust boundaries.</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>US-009 — security, permissions, secrets.</WCTX> -->
<!-- <CLOG>0.1.0: initial population. The workspace is a local library; most security-property questions return "no evidence found" after inspection.</CLOG> -->

# 9. Security, Permissions, and Secrets

This chapter records facts only. It does not assert that the workspace is "secure" or "insecure"; per `pro/REVERSE-PRD.md` §"Phase 7", such claims require direct evidence and are not produced by an audit of this scope.

## 9.1 Authentication / authorization

**No evidence found** after inspecting the workspace. The workspace is a local Rust library + two CLI binaries that operate on local input files and stdout. There is no user-identity model, no permission model, no role-based access check beyond `RoleTag` (which is a *visual-rendering* role concept, not a security-permission concept — chapter 7 §7.1).

A workspace-wide search for authentication / authorization patterns (`auth`, `permission`, `password`, `bearer`, `oauth`, `jwt`) returned only the `RoleTag` / `RoleSpace` rendering-role usages catalogued in chapter 7.

## 9.2 Credentials / secrets loading

**No evidence found** after inspecting the workspace. No credential file path, environment-variable read for secrets, or in-tree credential parsing was observed:

- The only `std::env::var` call site reads `CARGO_MANIFEST_DIR` (chapter 5 OPT-017) — a build-time variable, not a credential.
- A workspace-wide grep for `dotenv`, `secrecy`, `keyring`, `password`, `secret`, `api_key`, `token` returned zero matches in production code at audit-time.

## 9.3 TLS / network security

**No evidence found.** The workspace makes no network calls (chapter 6 §6.4, chapter 8 §8.8). A workspace-wide search for `TlsConnector`, `TlsAcceptor`, `rustls::`, `native_tls::`, `openssl::` returned zero matches.

## 9.4 Network binding

**No evidence found.** A workspace-wide search for `TcpListener`, `UdpSocket`, `bind(`, `listen(`, `hyper::`, `axum::`, `tonic::` returned zero matches in production code (chapter 6 §6.4). The workspace does not open a port at audit-time.

## 9.5 Filesystem permissions

The two binaries read and write files under the user's working directory:

- `xtask` reads source files and `docs/templates/*.toml`; writes to `docs/generated/*` (chapter 7 §7.2 / §7.3).
- `pipeline-probe` reads `--input` JSON; optionally writes a SQLite database file (chapter 7 §7.5).

No setuid / sudo / capability-elevation patterns were observed. The workspace runs at the user's privilege.

## 9.6 Unsafe Rust usage

A workspace-wide grep for `unsafe ` returned exactly **5 lines**, all in a single file (`crates/tui-vfx-compositor/tests/test_alloc_budget.rs`):

| Line | Site | Purpose |
|---|---|---|
| `:36` | `unsafe impl GlobalAlloc for CountingAllocator` | Test-only allocator used to measure pipeline allocation budget |
| `:37` | `unsafe fn alloc(&self, layout: Layout) -> *mut u8` | `GlobalAlloc::alloc` is `unsafe` by trait contract |
| `:39` | `unsafe { System.alloc(layout) }` | Delegates to system allocator |
| `:42` | `unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout)` | `GlobalAlloc::dealloc` is `unsafe` by trait contract |
| `:43` | `unsafe { System.dealloc(ptr, layout) }` | Delegates to system allocator |

All five are inside a `tests/*.rs` file — a test fixture. **No production code path uses `unsafe`** at audit-time. The `CountingAllocator` exists to assert that hot-path renders allocate within a measured budget; it is a measurement tool, not a runtime allocator the library ships.

The `ofpf-sql` query that produced the count (chapter 14 cites the query) reports `is_unsafe = 1` on exactly two function definitions (`alloc`, `dealloc`) — both at the same site. The `Cargo.toml` audit count of 0 production unsafe matches the public-surface inspection at `unsafe` macro / `extern "C"` / `#[no_mangle]` — all zero (chapter 6 §6.3).

## 9.7 Input validation

The workspace has two declared `Error`-returning validation entry points:

| Validator | Location | Returns |
|---|---|---|
| `CellMotionSpec::validate` | `crates/tui-vfx-content/src/cell_motion/cls_cell_motion_spec.rs:86-94` | `Result<(), CellMotionError>` — currently the only enumerated variant returned is `CellMotionError::InvalidQuantizeSteps` (`:88`); `CellMotionError` is defined at `:96-105` with a `Display` impl at `:100-105` and `Error` impl at `:107` |
| `tui-vfx-recipes` validator (out of audit scope) | sibling crate | (recipe-schema validation; chapter 6 §6.5 cross-references) |

Beyond these, library crates use `Option<T>` returns or panic via `unwrap`/`expect` for invariant violations (chapter 8 §8.5.2 enumerates the top sites by count). The `ProbeError` enum (`crates/tui-vfx-probe/src/cls_probe_error.rs:10`) carries `InvalidRequest` per `:20` and converts from `std::io::Error` (`:30-35`) and `serde_json::Error` (`:36-...`).

## 9.8 Trust boundaries

The trust boundaries the workspace honours by construction:

| Boundary | Side | Mechanism |
|---|---|---|
| Recipe JSON (V2 / V3) input | host → engine | Recipe loader (in sibling `tui-vfx-recipes`) parses, substitutes (load-time `Substitutions`), validates (strict-contracts mode), and produces a typed playback item. Engine consumes the typed item; engine does not re-parse JSON. (Chapter 3 F011 / Intention 5.) |
| Asset bytes (`.rss`, `.rsf`, fonts) | host → engine | Byte-source loaders accept `&[u8]` (per Intention 27); the engine does not own the source path. The consumer is responsible for the bytes. |
| `RuntimeBindings` per-frame values | host → engine | Map of name → value; the `Binding(name)` arm of `VfxBindableValue` resolves names against this map. Names that fail to resolve return `None`; the engine does not panic on a missing binding. |
| `pipeline-probe` `--input` JSON | external file → binary | Read via `serde_json::from_str(&fs::read_to_string(input_path)?)?` (`crates/tui-vfx-probe/src/bin/pipeline-probe.rs:97`). serde_json deserialization is the validation; malformed JSON returns an error and exits via `:25-29`. |

The workspace does not enforce content-level safety on these boundaries (e.g., it does not bound recipe sizes, asset sizes, or binding map sizes at audit-time). That is the consumer's responsibility.

## 9.9 Confidence

**High** for every "no evidence found" claim — each is derived from an empty workspace-wide grep / `ofpf-content` search. **High** for the unsafe-count (verified twice: by direct grep and by `ofpf-sql` against the indexed definitions). **High** for the validator entry points and trust-boundary mechanisms.

This chapter does **not** evaluate the security properties of the boundaries — it only documents which boundaries exist and which do not (e.g., "no network port is opened" is a recorded fact; "the lack of a network port is secure" is not a claim this audit makes).

<!-- <FILE>pro/EXISTING-SYSTEM-PRD/09_security_permissions_secrets.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
