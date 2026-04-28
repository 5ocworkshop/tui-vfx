<!-- <FILE>steering/OFPF-TOOLS.md</FILE> - <DESC>Project-local practical reference for the ofpf-* tooling suite (a thin alias layer over librarian-cli, backed by a multi-tenant daemon). Lead-with framing for code access, the 80/20 tool surface, decision matrix by intent, composition rules that prevent wrong answers, output handling, multi-repo workflow, the first-class sql subcommand, pitfalls, and the non-obvious flags that bite first-time users. Required reading per Intention 42.</DESC> -->
<!-- <VERS>VERSION: 0.4.2</VERS> -->
<!-- <WCTX>2026-04-27: capture lessons from a session that exercised ofpf-sql against the full schema — column-value enumerations, per-crate aggregation pattern, and the multi-repo iteration recipe using --root rather than federation.</WCTX> -->
<!-- <CLOG>0.4.2: enumerate kind/visibility values inline in the §9 schema table; add four §9 examples (per-crate aggregation, OFPF prefix audit, per-crate test coverage, metadata coverage); add three §9 surprises (visibility values, single-root SQL, path-slice trick); rewrite §7 multi-repo paragraph around per-`--root` iteration.</CLOG> -->

# OFPF Tools — practical reference

The `ofpf-*` suite is a thin alias layer over `librarian-cli`, which talks to a long-running multi-tenant `librarian-daemon`. One daemon serves up to ten loaded repositories in parallel. This document is the project-local reference for which tool answers which question, how to handle the output, and which non-obvious things bite first-time users. Required reading per Intention 42.

The global standards in `~/.claude/rules/ofpf.md` and `~/.claude/CLAUDE.md` introduce the suite and its philosophy. This file complements them with the operational detail a developer or AI agent needs in-flight.

When in doubt about a flag or behavior, run `librarian-cli --help-json` (canonical command schema) and `librarian-cli meta` / `ofpf-meta` (decoder for response wire keys — every key returned by every intent with a one-line description). Both are authoritative; this document is curated.

> **2026-04-27 release note.** This doc tracks a release with breaking wire renames: `loc`→`pos` (position string), `lines`→`loc` (LOC count), `l`→`lines`, `n`→`ln`, `def`→`definition` (on `references`), `depth`→`max_depth` (request echo on `blast-deep`), `tests`→`test_files` (on `blast-deep`), `path`→`trace_path` (federated trace), `barrel`→`re_export` (role), `is_barrel`→`is_re_export` (virtual SQL column). No deprecation aliases. Update parsers and prompts before pulling new binaries; `librarian-cli meta` is the source of truth for the current key set.

---

## 1. When to lead with `ofpf-*`

Default for repo-shape questions:

| Question | Tool |
|---|---|
| Where is symbol X defined? | `ofpf-defs X` |
| Who uses symbol X? | `ofpf-refs X` + `ofpf-content X` (pair them; see §4) |
| What does this file do? | `ofpf-inspect <path>` |
| What breaks if I change Z? | `ofpf-blast <path>` / `ofpf-blast-deep` / `ofpf-trace` |
| Read just one symbol from a big file | `ofpf-extract <path> <symbol>` |
| What is the *intent* of this file? | `ofpf-search-meta <q> --tag desc` (skip the source read) |
| Decode a wire key returned by another intent | `ofpf-meta` (now points at the canonical decoder, not metadata search) |
| Read-only SQL against the index | `librarian-cli sql "<query>"` / `ofpf-sql` (see §9) |
| First contact with a repo | `ofpf-orientation` (now embeds the decoder — no separate `meta` round-trip needed) |

Drop through to direct tools when:

- **Reading a file end-to-end you already know you need** → `Read`. Don't loop `extract` + `around` to reconstruct what one read gives.
- **Executing anything** (build, test, lint, format, git) → shell. The librarian indexes; it does not run.
- **Editing** → `Edit` / `Write`. The suite is read-only by design.
- **Tight write-then-read loops** → direct file IO. The watcher debounces in seconds.
- **Non-text content** (images, PDFs, archives) → appropriate viewer.

The win is context efficiency. `ofpf-inspect` returns defs + callers + callees + role + tests in a few hundred bytes; reading the file end-to-end burns kilobytes. Treat the orchestrator's context as ~10× more valuable than a query (the global standards make this explicit) and the lead-with default falls out.

---

## 2. The 80/20 tools

These five cover the majority of real queries. Internalize them first.

| Alias | librarian-cli | One-line purpose |
|---|---|---|
| `ofpf-status` | `status` | Is the daemon healthy and is the graph stale? |
| `ofpf-inspect <path>` | `inspect` | Defs + callers + callees + tests + metrics + role for one file |
| `ofpf-content <pattern>` | `search-content` (`grep`/`rg`) | Ripgrep over the loaded repo; works on any text file |
| `ofpf-defs <name>` | `search-defs` (`find-defs`) | Find definitions by symbol name |
| `ofpf-around <path> <query>` | `around` (`grep-context`) | Grep-style context around a pattern in one file |

Add these once you start mirroring patterns or planning edits:

| Alias | librarian-cli | When to use |
|---|---|---|
| `ofpf-extract <path> <symbol>` | `extract` | One named symbol's snippet — saves loading the whole file |
| `ofpf-read <path>` | `content` (`cat`/`read`) | Arbitrary line spans, multi-file batched reads (`--paths`, `--range`) |
| `ofpf-focus <path>` | `focus` (`show`) | File + deps + dependents + tests in one bundle |
| `ofpf-blast <path>` | `blast` (`affected`/`impact`) | What breaks if I change this? Add `--why` for the chain |
| `ofpf-refs <name>` | `references` (`find-refs`/`refs`) | Files that import the defining file — see §4 |
| `ofpf-orientation` | `orientation` | Architecture bundle: overview + hotspots + inspect — first call on a new repo. Now returns `data.bundle` (the original payload) plus `data.keys` (full wire-key decoder), `data.keys_version` (16-hex hash for cache invalidation), and `data.escape_hatches.sql` (pointer to `librarian-cli sql`). Self-decoding; cache by `keys_version` |
| `ofpf-sql <query>` | `sql` (intent: `raw_sql`) | First-class read-only SQL against the index. Positional, `-q/--query`, `--query-file <path>`, or stdin (`-`). Mutations and PRAGMA denied by binding contract. See §9 |
| `ofpf-meta` | `meta` | Wire-key decoder. Returns the `_keys` map of every wire key → one-line description. (Renamed: this alias used to mean OFPF metadata-header search; that capability moved to `ofpf-search-meta`) |

---

## 3. Decision matrix by intent

| Intent | Tool | Notes |
|---|---|---|
| First contact with a repo | `ofpf-orientation` | Returns roles (`hub`/`core`/`unit`/`re_export`), top hotspots, fan-in/out metrics, plus the embedded decoder (`data.keys`, `data.keys_version`) |
| "Where is symbol X defined?" | `ofpf-defs X` | Add `--kind function\|method\|class\|struct` to disambiguate |
| "Who uses symbol X?" | `ofpf-refs X` **and** `ofpf-content X` | Two halves of the answer — see §4 |
| "What calls function `foo`?" | `ofpf-callers foo` | Function-call edges; supports `--depth` |
| "What does function `foo` call?" | `ofpf-callees foo` | Same trait, opposite direction |
| "Is this function dead code?" | `ofpf-dead --scope file --path <p>` | Or `--scope project` for repo-wide pass |
| "Where is the literal string `X` mentioned?" | `ofpf-content X` | Literal by default. Add `--regex` for patterns, `--glob "**/*.json"` for non-indexed files |
| "All `X` touch sites in one file with context" | `ofpf-around <path> X -A 5 -B 1` | grep-style `-A`/`-B`. Default 5 each |
| "Read just one symbol from a big file" | `ofpf-extract <path> <symbol>` | Both args required |
| "Read lines N–M of a file" | `ofpf-read <path> --from N --to M` | Or `--paths a.rs b.rs --range a.rs:10:30 --range b.rs:1:50` for multi-file |
| "What does this file do?" | `ofpf-inspect <path>` | One call returns defs + callers + callees + role + metrics + tests |
| "Should I be worried about changing this file?" | `ofpf-inspect <path>` then `ofpf-blast <path>` | High fan-in → wide blast radius. `--why` shows the dependency chain |
| "Find dependency path between two files" | `ofpf-trace <from> <to>` | Pass `--root <path>` to target a specific loaded repo |
| "Is there a circular dependency?" | `ofpf-cycles` | Returns cycles with refactoring suggestions |
| "Which files are too big?" | `ofpf-loc 300 --filter <prefix>` | Files over the threshold; `--filter` scopes to a subtree |
| "What can run in parallel?" | `ofpf-dag` | Files grouped by execution tier |
| "What `<DESC>` text does this file's metadata header carry?" | `ofpf-search-meta X --tag desc` | OFPF metadata search (`--tag desc\|wctx\|clog\|vers`). **Note:** this used to be `ofpf-meta`; `ofpf-meta` now points at the wire-key decoder |
| "What tests exist for this source?" | `ofpf-tests <path>` | Returns path + relation + confidence |
| "Which files are imported by file `Y`?" | `ofpf-context <path>` | Imports graph for one file |
| "Compact line-shaped output instead of JSON" | any text-mode-supported command + `--output-format text` (or `--out text`) | Supported on `defs`, `search-content` (full and `--files-with-matches`), `references`, `blast` / `blast-deep`, `verify`, `loc`, `dag`, `dead-code`, `trace`, `around`. Nested commands (`inspect`, `orientation`, `status`, `cycles`) always emit JSON |
| "Files with matches plus per-file match counts" | `ofpf-content <q> --files-with-matches` | Each row carries `match_count`. Add `--with-match-lines` to also emit a `match_lines` array (line numbers). Lightweight mode stays lightweight unless you opt in |
| "Cycles, ignoring re-export aggregators" | `ofpf-cycles --exclude-roles re_export` | Drops SCCs in which every node is a re-export aggregator (`mod.rs`, `lib.rs`, `__init__.py`, `index.{ts,tsx,js,jsx}`) |
| "DAG tiers, ignoring re-export aggregators" | `ofpf-dag --exclude-roles re_export` | Discounts aggregator edges from tier computation. Typically widens parallelism several-fold on real repos |
| "Read-only SQL across the index" | `librarian-cli sql "<query>"` / `ofpf-sql` | First-class subcommand (replaces hand-rolled `raw_sql` JSON envelopes). See §9 |

---

## 4. Composition rules that prevent wrong answers

Two patterns to memorize. Each catches a class of mistake the rest of the suite cannot prevent on its own.

### Pair `ofpf-refs` and `ofpf-content` for usage questions

`ofpf-refs <symbol>` returns *files that import the symbol's defining file*. It is the import-graph half of the answer, and it does not enumerate every text occurrence.

`ofpf-content <symbol>` returns every text occurrence (rg-backed) — including doc comments, design notes, archived code under `recyclebin/`, and string literals.

Use both. `refs` answers "what would breaking this re-export break?"; `content` answers "what reads this name at all?" Either alone undercounts.

Concrete: `ofpf-refs VfxBindableU16` returns one file (the typedef's parent `mod.rs`) because the alias lives in one file and only that file imports the canonical home. `ofpf-content "VfxBindableU16"` returns ~23 files (real call sites, tests, design docs, the consolidated `recyclebin/` history).

### File role ≠ symbol reach

`ofpf-inspect` returns `role: unit | core | hub | re_export` based on the *file's* fan-in/fan-out in the import graph. A file with `role: unit` and `fan_in: 1` may still define a type used in 100 files — if all 100 import via a re-exporting `mod.rs`, the file's own fan-in stays at 1.

When the question is "is this type widely used?", inspect the type, not just the file. `ofpf-content "<TypeName>"` is the second call. `ofpf-blast` likewise reports the file's direct dependents, not the type's reach.

The `re_export` role classifies pure-aggregator files (Rust `mod.rs` / `lib.rs`, Python `__init__.py`, JS/TS `index.{ts,tsx,js,jsx}`) after the base hub/core/unit pass. They inflate cycle and DAG output without representing real architectural coupling — pair `ofpf-cycles` and `ofpf-dag` with `--exclude-roles re_export` when the question is "what real cycles / parallelism do we have?"

---

## 5. Aliases the standards docs do not list

`librarian-cli --help-json` reveals natural-English aliases that the global standards docs do not surface. Use whichever feels native:

| Canonical | Aliases |
|---|---|
| `search-content` | `grep`, `rg`, `ripgrep` |
| `search` | `find` |
| `search-defs` | `find-defs`, `defs` |
| `references` | `find-refs`, `refs` |
| `content` | `cat`, `read`, `view` |
| `focus` | `show` |
| `inspect` | `info`, `details` |
| `verify` | `tests`, `find-tests` |
| `blast` | `affected`, `impact` |
| `blast-deep` | `impact-deep` |
| `dag` | `parallel`, `tiers` |
| `loc` | `big-files`, `large-files` |
| `around` | `grep-context`, `rg-context` |
| `dead-code` | `unused` |
| `symbol-blast` | `sym-blast` |
| `call-hierarchy` | `callers`, `callees` |
| `status` | `health` |
| `overview` | `summary` |
| `extract` | `snippet`, `show-symbol` |
| `sql` | (no rg/grep parallel — first-class read-only SQL) |

Both positional and `--flag <value>` forms work for primary args (`-q`/`--query`, `-p`/`--path`, `-k`/`--kind`, `-l`/`--lang`, `-d`/`--depth`, `-s`/`--symbol`, `-S`/`--scope`).

The global `--output-format <json|text>` flag (alias `--out`) selects compact line-shaped output for the commands listed in §3. Default remains `json`.

---

## 6. Output handling

Responses are JSON. The shape is `{ data, error, notices, guard?, req_id }`:

- `data` — command result; structure varies by command
- `error` — `null` on success, otherwise an error string
- `notices` — warnings (version mismatch, blacklisted noisy file, etc.)
- `guard` — present only when result count or size exceeded the limit
- `req_id` — daemon-side request id (useful when filing `ofpf-bug`)

JSON is directly readable. Treat the structured form as the canonical output, not as something to reformat. Filter or extract fields when the response is genuinely large (`ofpf-orientation`, deep blasts, content searches with hundreds of hits) or when you need a specific subset for a downstream step. Reformatting concise responses (a five-row `ofpf-defs`, a small `ofpf-trace`) is wasted work.

### Response shapes vary by command

A few examples worth knowing before you parse (post-2026-04-27 wire format):

- `ofpf-defs <q>` → `[{"def": "<kind> <name>", "pos": "path:line"}]` — readable strings.
- `ofpf-content <q>` → `[{"pos", "content", "kind", "match_start", "match_end"}]` (line hits) or, with `--files-with-matches`, `[{"pos": "<path>", "kind": "file", "match_count": N, ("match_lines": [...])}]`.
- `ofpf-loc <threshold>` → `[{"loc": <int>, "p": "<path>"}]`.
- `ofpf-inspect <p>` → `{"defs", "callers", "callees", "metrics": {"co", "in", "out", "role"}, ...}`.
- `ofpf-around <p> <q>` → `{"bl": [{"s", "e", "lines": [{"ln", "t", "m"}]}]}` — blocks of lines with start/end and per-line line-number/text/match-flag.
- `ofpf-blast-deep <p>` → `{"target", "max_depth", "breaks": {"1": [...], "2": [...]}, "test_files": [...], ...}`.
- `ofpf-trace <from> <to>` (federated) → `{"trace_path": [...], "synthetic_edges_used": N, ...}`.
- `ofpf-orientation` → `{"bundle": {...}, "keys": {...}, "keys_version": "<16-hex>", "escape_hatches": {"sql": {...}}}`.

When a key looks unfamiliar, `librarian-cli meta` (a.k.a. `ofpf-meta`) is the decoder — every wire key returned by every intent with a one-line description. `ofpf-orientation` now embeds the same map inline as `data.keys` so consumers can self-decode without a separate round-trip; cache by `data.keys_version`. The most common keys you'll see:

| Wire key | Meaning |
|---|---|
| `pos` | position string `<path>:<line>` (or just `<path>` for files-with-matches rows). **Renamed from `loc`.** |
| `loc` | lines-of-code count (per-row in `loc`; threshold echo). **Renamed from `lines`.** |
| `lines` | lines array inside `around` blocks (`bl[].lines[]`). **Renamed from `l`.** |
| `ln` | 1-indexed line number inside `lines[]`. **Renamed from `n`.** |
| `definition` | queried-definition info on `references`. **Renamed from `def`** to avoid colliding with the inner `<kind> <name>` `def` string. |
| `max_depth` | request-echo max depth on `blast-deep`. **Renamed from `depth`** (per-row `depth` is still the dependency-chain depth). |
| `test_files` | test files in a `blast-deep` blast radius. **Renamed from `tests`.** |
| `trace_path` | ordered trace nodes on federated `trace`. **Renamed from `path`** to avoid colliding with the canonical filesystem-path key. |
| `role` | `unit` / `hub` / `core` / `re_export`. **`re_export` replaces `barrel`.** |
| `co` / `in` / `out` | cohesion / fan_in / fan_out |
| `p` | path (relative to repo root) |
| `f` | file |
| `k` | kind |
| `mod` | module |
| `bl` | blocks (in `around` results) |
| `s` / `e` | start / end line numbers of an `around` block |
| `m` | match marker on a line |
| `ml` | matched-lines array |
| `match_count` | per-file match count on `--files-with-matches` rows |
| `match_lines` | per-file line-number list (only when `--with-match-lines` is also set) |
| `keys` / `keys_version` | embedded decoder + cache key on `orientation` |
| `escape_hatches` | `orientation`-level pointer to lower-level capabilities (today: `sql`) |

Run `librarian-cli meta` to refresh — the surface evolves and the decoder is the source of truth.

### Don't reformat the JSON — read it

The JSON output is optimized for direct reading. Keys are descriptive (`fan_in`, `fan_out`, `path`, `pos`, `role`, `match_count`); rows are short; the shape is consistent within a command. **Reading the raw response directly is the canonical path.** Piping to Python to "render it nicely" is the anti-pattern — it forces you to guess key names ahead of time, and a wrong guess (`KeyError`) inside a parallel tool batch cancels every sibling call. One assumption costs four queries.

The rules, in order:

1. **First time touching a command's response: just read the JSON.** It's terse. The keys are self-describing. The §6 keys table tells you what to expect. There is no "render step" to add — the JSON *is* the rendering.
2. **For list output you want to grep or pipe further, use `--out text`.** Supported on `defs`, `search-content`, `references`, `blast` / `blast-deep`, `verify`, `loc`, `dag`, `dead-code`, `trace`, `around`. `librarian-cli search-defs -q apply --out text` already returns `path:line\t<def>` lines.
3. **For filtering across many rows, use `jq`.** `jq` fails to `null` rather than throwing, so a wrong path produces empty output instead of cancelling parallel siblings. Example: `... | jq -r '.data[] | select(.fan_in > 30) | .path'`.
4. **Reach for Python only for genuine cross-row computation** (joins, aggregations, math the daemon's SQL surface didn't already do). And in that case, look at the response shape first — save to `/tmp/x.json`, inspect, then write the parser. Don't invoke Python with key paths you haven't verified in the actual current output.

The anti-pattern, named so it's recognizable: `librarian-cli <cmd> | python3 -c "import json,sys; d=json.load(sys.stdin)['data']; for r in d: print(r['guessed_key'])"`. Every word of that pipeline is unnecessary when the goal is just to see the result. The JSON above the pipe is already the answer.

### When filtering is worth it

Three patterns that earn the extra tokens:

```bash
# Drop recyclebin/ noise from a content search
ofpf-content "<pattern>" | python3 -c "
import json, sys
for r in json.load(sys.stdin)['data']:
    if not r['pos'].startswith('recyclebin/'):
        print(r['pos'])
"

# Same query, no Python — text mode is a one-liner
ofpf-content "<pattern>" --out text | grep -v '^recyclebin/'

# Compact view of a wide ofpf-around response
ofpf-around <path> "<query>" -A 4 -B 1 | python3 -c "
import json, sys
d = json.load(sys.stdin)['data']
for b in d['bl']:
    print('===', b['s'], '-', b['e'], '===')
    for ln in b['lines']:
        print(f\"{'*' if ln['m'] else ' '}{ln['ln']:4}: {ln['t']}\")
"

# Just the high-signal fields from an inspect
ofpf-inspect <path> | python3 -c "
import json, sys
d = json.load(sys.stdin)['data']
print('role:', d['metrics']['role'], 'in:', d['metrics']['in'], 'out:', d['metrics']['out'])
print('callers:', len(d['callers'].get('logic', [])))
"
```

`jq` works equivalently if you prefer it. The principle: filter to extract; don't filter to reformat.

### Response guard

Triggered when `result_count > 100` OR `response_size > 100KB` (configurable via `--max-kb`). The daemon returns a preview (first 5 items) plus options:

- `--force` — return the full result anyway (use only when you actually need it)
- `--limit N --offset M` — paginate
- Refine the query (better)

Applies to: `search`, `search-defs`, `search-content`, `search-meta`, `references`, `blast`, `blast-deep`. **Refine first, paginate second, force last.**

### Exit codes follow grep semantics

- `0` — success / matches found
- `1` — no matches (NOT an error). Treat as valid data.
- `2` — actual error (bad arguments, daemon down, etc.)

---

## 7. Multi-repo workflow

The daemon is multi-tenant. `--root <path>` selects which loaded repo a query targets; it does not swap a single active root. Up to ten repos can stay loaded in parallel.

```bash
ofpf-load --root /usr/projects/mixed-signals       # adds, doesn't swap
ofpf-defs --root /usr/projects/mixed-signals fbm3
ofpf-extract --root /usr/projects/tui-vfx-recipes <path> <symbol>
```

If you omit `--root`, the call resolves against the daemon's notion of the current root (typically the CWD when the daemon was started or the most recently loaded). Always pass `--root` explicitly when crossing repo boundaries — it removes ambiguity.

For multi-repo audits (the four-repo scope per Intention 41), iterate the same query across each `--root` rather than reaching for federation. A small shell loop over the canonical roots is the recommended pattern — it keeps each result set scoped to one repo (so paths, IDs, and counts stay unambiguous) and composes cleanly with the response guard.

```bash
for root in /usr/projects/tui-vfx /usr/projects/tui-vfx-recipes \
            /usr/projects/mixed-signals /usr/projects/gt-design; do
  echo "=== $root ==="
  ofpf-content --root "$root" "VfxBindableU16" --files-with-matches --out text
done
```

### When the graph is stale

`ofpf-status` reports `is_stale: true` and `stale_reason` when the indexed graph has fallen behind the working tree. The daemon auto-regenerates on file changes (rate-limited), but you can force it with `ofpf-load --root <path>`. Most queries are still useful while stale; treat the staleness flag as a hint, not a blocker. For audits where exactness matters, force a reload first.

### `recyclebin/` is indexed

Archived code in `recyclebin/` is part of the index and shows up in `ofpf-content`, `ofpf-defs`, and other text-grep results unless filtered. This is genuinely useful for archaeology — finding the pre-consolidation form of a type, or the previous implementation of an algorithm — but it can mislead audits scoped to live code. When the question is "where is X currently used?", filter `recyclebin/` out (see §6).

---

## 8. JSON mode for scripting

Pipe one or more requests as JSON to stdin with `--json`:

```bash
echo '{"q":"stats"}' | librarian-cli --json
```

NDJSON works for batches:

```bash
echo -e '{"q":"stats"}\n{"q":"overview"}' | librarian-cli --json
```

Common arg keys in JSON mode: `q` (query OR command name in batch mode), `p` (path), `kind`, `depth`, `gates`, `snippets`. Schema in `librarian-cli --help-json` under `json_mode`. Most interactive use is fine with the positional CLI form; reach for JSON mode for batch automation or when assembling queries programmatically.

---

## 9. `librarian-cli sql` — first-class read-only SQL

The daemon exposes a read-only SQL surface against the index database. As of the 2026-04-27 release this is a **first-class CLI subcommand** (`librarian-cli sql` / `ofpf-sql` / `ofpf-rawsql`); the underlying wire intent is still `raw_sql`, but you no longer assemble JSON envelopes by hand.

`sql` is the universal answer to questions the high-level commands can't compose: filtering by multiple criteria, JOINs across metrics + paths + edges, custom aggregations, schema introspection.

### When to reach for it

After exhausting:

- The locator commands (`defs`, `content`, `refs`, `around`)
- The traversal commands (`inspect`, `blast`, `trace`, `focus`, `context`)
- The composition rules in §4 (`refs` + `content`, `metrics` + `loc`)

If the question still doesn't fit any single command — for example "files with high fan_in AND >400 LOC AND no test peer," or "top external crates by import count," or "all public traits across the workspace" — drop to `librarian-cli sql`.

### Invocation

Four input forms are equivalent — pick whichever fits the shell context:

```bash
# Positional (most common)
librarian-cli --root /usr/projects/tui-vfx sql "SELECT COUNT(*) FROM files"
ofpf-sql "SELECT COUNT(*) FROM files"

# Flag form (handy when the query embeds quotes)
librarian-cli sql -q "SELECT lang, COUNT(*) FROM files GROUP BY lang"

# Stdin
echo "SELECT path FROM files LIMIT 5" | librarian-cli sql -

# File
librarian-cli sql --query-file /tmp/audit.sql
```

Pagination uses the global `--limit` / `--offset` flags. The legacy JSON-envelope form (`{"q":"raw_sql","args":{...}}` via `--json`) still works for batch automation but is no longer the recommended interactive path.

The query is auto-wrapped as `SELECT * FROM (<your-query>) AS _q LIMIT ?1 OFFSET ?2` for safety. Side effect: not every SQL form composes through that wrapper — see Surprises below.

Per-query timeout defaults to 1000ms, hard cap 5000ms (`timeout_ms` arg in JSON-mode invocations; not a CLI flag today). Default `--limit` is 100, hard cap 500.

### Schema (14 tables)

The 2026-04-27 release removed the `resolved_imports` table — it was scaffolded but never populated or queried; pre-1.0 status meant no migration was owed.

| Table | What it holds |
|---|---|
| `files` | id, path, kind, lang, lines, zero_deps, generated |
| `file_metrics` | file_id, fan_in, fan_out, cohesion, role (`unit` / `hub` / `core` / `re_export`), `is_re_export` (virtual: `role = 're_export'`) |
| `file_edges` | source_file_id, target_file_id, edge_type (`logic` / `crate_dep`) |
| `file_definitions` | id, file_id, name, kind (`function` / `method` / `module` / `struct` / `enum` / `constant` / `typealias` / `trait`), line, end_line, parent, doc, visibility (`public` / `private` / `crate`), is_test, test_attributes |
| `symbol_edges` | source_def_id, target_def_id, call_site_line, edge_type — call graph |
| `dependencies` | source_file_id, target_module_id, is_dynamic — import edges |
| `test_links` | source_file_id, test_file_id, relation |
| `ofpf_metadata` | file_id, file_path, description (DESC), version (VERS), work_context (WCTX), changelog (CLOG), is_ofpf |
| `type_info` | def_id, type_signature, return_type, is_async, is_unsafe, generic_params |
| `modules` | id, name, is_external |
| `symbols` | id, name (symbol pool) |
| `call_sites` | dependency_id ↔ symbol_id |
| `definitions` | module_id ↔ file_id (link table — NOT the symbol table; that is `file_definitions`) |
| `meta` | project_root, schema, schema_version |

Discover any table's columns directly:

```sql
SELECT name FROM sqlite_master WHERE type='table' ORDER BY name;
SELECT sql FROM sqlite_master WHERE name='file_metrics';
```

### Examples

```sql
-- File counts by language
SELECT lang, COUNT(*) FROM files GROUP BY lang ORDER BY 2 DESC;

-- File counts by role (now includes re_export)
SELECT role, COUNT(*) FROM file_metrics GROUP BY role ORDER BY 2 DESC;

-- Hubs by fan_out
SELECT f.path, m.fan_out, m.fan_in
FROM file_metrics m JOIN files f ON f.id = m.file_id
WHERE m.role = 'hub' ORDER BY m.fan_out DESC LIMIT 10;

-- Re-export aggregators (was role='barrel' before 2026-04-27)
SELECT f.path, m.fan_in, m.fan_out
FROM file_metrics m JOIN files f ON f.id = m.file_id
WHERE m.is_re_export = 1 ORDER BY m.fan_in DESC LIMIT 10;

-- Risky to change: large AND high fan_in
SELECT f.path, f.lines, m.fan_in
FROM file_metrics m JOIN files f ON f.id = m.file_id
WHERE f.lines > 300 AND m.fan_in > 30
ORDER BY f.lines * m.fan_in DESC LIMIT 10;

-- Files with no test peer (test orphans)
SELECT f.path FROM files f
WHERE f.lang = 'rs'
  AND f.path NOT LIKE '%/test_%' AND f.path NOT LIKE 'test%'
  AND NOT EXISTS (SELECT 1 FROM test_links tl WHERE tl.source_file_id = f.id)
ORDER BY f.path LIMIT 20;

-- Definitions per kind
SELECT kind, COUNT(*) FROM file_definitions GROUP BY kind ORDER BY 2 DESC;

-- All public traits across the workspace
SELECT f.path, d.name, d.line FROM file_definitions d
JOIN files f ON f.id = d.file_id
WHERE d.kind = 'trait' AND d.visibility = 'public' ORDER BY f.path;

-- Files with the most defs (a different "big" than LOC)
SELECT f.path, COUNT(d.id) AS n_defs FROM files f
JOIN file_definitions d ON d.file_id = f.id
GROUP BY f.id ORDER BY n_defs DESC LIMIT 10;

-- OFPF DESC text mentioning a topic (ofpf-search-meta is the higher-level form)
SELECT om.file_path, om.description FROM ofpf_metadata om
WHERE om.description LIKE '%bindable%' AND om.is_ofpf = 1;

-- Async or unsafe definitions (not surfaced by any high-level command)
SELECT f.path, d.name, ti.is_async, ti.is_unsafe
FROM type_info ti JOIN file_definitions d ON d.id = ti.def_id
JOIN files f ON f.id = d.file_id
WHERE ti.is_async = 1 OR ti.is_unsafe = 1;

-- Per-crate aggregation: slice path on the second '/' to extract `crates/<name>`
-- (instr finds the offset of '/' inside path[8..]; +6 lifts back to absolute).
-- Reusable for LOC-per-crate, file-count-per-crate, defs-per-crate, etc.
SELECT
  CASE WHEN path LIKE 'crates/%'
       THEN substr(path, 1, instr(substr(path, 8), '/') + 6)
       ELSE 'other' END AS crate,
  COUNT(*) AS files,
  SUM(lines) AS total_loc
FROM files WHERE lang='rs'
GROUP BY crate ORDER BY total_loc DESC;

-- OFPF prefix audit: files exceeding their per-prefix hard limit
-- (cls_ > 200, fnc_ > 120, orc_ > 250, ui_ > 200, col_ > 100).
SELECT path, lines AS loc,
  CASE
    WHEN path LIKE '%/cls_%' THEN 'cls_ (>200)'
    WHEN path LIKE '%/fnc_%' THEN 'fnc_ (>120)'
    WHEN path LIKE '%/orc_%' THEN 'orc_ (>250)'
    WHEN path LIKE '%/ui_%'  THEN 'ui_ (>200)'
    WHEN path LIKE '%/col_%' THEN 'col_ (>100)'
  END AS prefix
FROM files WHERE lang='rs' AND path LIKE 'crates/%' AND (
  (path LIKE '%/cls_%' AND lines > 200) OR
  (path LIKE '%/fnc_%' AND lines > 120) OR
  (path LIKE '%/orc_%' AND lines > 250) OR
  (path LIKE '%/ui_%'  AND lines > 200) OR
  (path LIKE '%/col_%' AND lines > 100)
) ORDER BY loc DESC LIMIT 20;

-- Per-crate test peer coverage (% of source files with a linked test_)
SELECT
  CASE WHEN f.path LIKE 'crates/%'
       THEN substr(f.path, 1, instr(substr(f.path, 8), '/') + 6)
       ELSE 'other' END AS crate,
  COUNT(*) AS src_files,
  SUM(CASE WHEN tl.test_file_id IS NOT NULL THEN 1 ELSE 0 END) AS with_tests,
  ROUND(100.0 * SUM(CASE WHEN tl.test_file_id IS NOT NULL THEN 1 ELSE 0 END) / COUNT(*), 1) AS pct
FROM files f LEFT JOIN test_links tl ON tl.source_file_id = f.id
WHERE f.lang='rs' AND f.path LIKE 'crates/%'
  AND f.path NOT LIKE '%/test_%' AND f.path NOT LIKE '%/tests/%'
  AND f.path NOT LIKE '%mod.rs' AND f.path NOT LIKE '%lib.rs'
GROUP BY crate ORDER BY pct ASC;

-- OFPF metadata header coverage (Intention 12 / 12A discipline check)
SELECT
  SUM(CASE WHEN om.is_ofpf = 1 THEN 1 ELSE 0 END) AS with_header,
  SUM(CASE WHEN om.is_ofpf IS NULL OR om.is_ofpf = 0 THEN 1 ELSE 0 END) AS without_header
FROM files f LEFT JOIN ofpf_metadata om ON om.file_id = f.id
WHERE f.lang='rs' AND f.path LIKE 'crates/%';
```

### Safety envelope

Verified by probing the daemon. The error codes are stable contract surface:

| Operation | Result |
|---|---|
| `SELECT`, `WITH RECURSIVE`, JOINs, subqueries | allowed |
| `INSERT` / `UPDATE` / `DELETE` / `DROP` / `CREATE` / `ALTER` / `ATTACH` | denied — `RAW_SQL_DENIED: Statement type is not permitted for raw_sql: <KIND>` |
| `PRAGMA <anything>` | denied — even read-only forms like `PRAGMA table_info(files)`. Use `sqlite_master` for schema introspection |
| Multiple statements (`;`-separated) | denied — `RAW_SQL_INVALID: Only a single SQL statement is allowed` |
| `EXPLAIN <query>` | broken — the auto-wrap builds `SELECT * FROM (EXPLAIN ...) AS _q ...` which is a syntax error |

The SQLite authorizer hook allows: `Read`, `Select`, `Function`, `Recursive`, `Transaction`, `Savepoint`. Everything else is denied. A timeout (default 1000ms, max 5000ms) fires via `progress_handler` for queries that genuinely run long.

### Pagination

Every response carries a `pagination` block: `{returned, next_offset, has_more}`. Default `limit` is 100; the hard cap is 500. For corpus-wide walks, drive the subcommand from a shell loop or a small Python helper:

```bash
# Bash form — subcommand + global --limit / --offset
offset=0
while :; do
  out=$(librarian-cli sql "SELECT path FROM files ORDER BY id" --limit 500 --offset "$offset")
  echo "$out" | jq -r '.data.records[].path'
  has_more=$(echo "$out" | jq -r '.data.pagination.has_more')
  [ "$has_more" = "true" ] || break
  offset=$(echo "$out" | jq -r '.data.pagination.next_offset')
done
```

```python
# Python form — wrap the subcommand
import json, subprocess
def sql(query, limit=500, offset=0):
    out = subprocess.check_output(
        ["librarian-cli", "sql", query, "--limit", str(limit), "--offset", str(offset)],
        text=True,
    )
    return json.loads(out)["data"]

offset = 0
while True:
    d = sql("SELECT path FROM files ORDER BY id", offset=offset)
    if not d["records"]: break
    for row in d["records"]: print(row["path"])
    if not d["pagination"]["has_more"]: break
    offset = d["pagination"]["next_offset"]
```

### Surprises

- **Default `limit` is 100, not 500.** A bare `SELECT id FROM file_definitions` returns 100 rows with `has_more: true`. Pass `limit: 500` to reach the cap; for full corpus walks, paginate with `offset`.
- **PRAGMA is universally denied.** Schema introspection uses `sqlite_master`, not `PRAGMA table_info`. Index list: `SELECT name FROM sqlite_master WHERE type='index'`.
- **`EXPLAIN` is unreachable** through this surface. The auto-wrap subquery is incompatible with `EXPLAIN <query>`. There is no current path to inspect query plans.
- **`definitions` is a link table** (module_id ↔ file_id) — it is *not* the symbol table. The symbol table is `file_definitions`. This is the most common mistake on first use.
- **`file_metrics.is_re_export` is a virtual generated column** (`role = 're_export'`). Renamed from `is_barrel` in the 2026-04-27 release; old `role='barrel'` filters return nothing. As of this release the role is populated — Rust `mod.rs`/`lib.rs`, Python `__init__.py`, and JS/TS `index.{ts,tsx,js,jsx}` aggregators are classified as `re_export` after the base hub/core/unit pass.
- **`zero_deps` and `generated` flags are always 0** in this repo. Either tui-vfx genuinely has no zero-dep / no generated files, or the indexer doesn't populate them. Check before filtering on them.
- **Default `timeout_ms` is 1000.** Aggressive multi-JOIN queries against the full corpus need an explicit `timeout_ms: 5000` to use the headroom.
- **Read-only side-effect functions are allowed.** `randomblob()`, `random()`, etc. work — they're SELECT-shaped and the authorizer permits them.
- **`visibility` values are `public` / `private` / `crate`** — not `pub` / `pub(crate)`. Filtering on `visibility='pub'` returns zero rows silently; the schema table above enumerates the full set. Same trap applies to `kind` (`function` / `method`, not `fn`).
- **SQL is scoped to one repo at a time.** Each query runs against a single `--root`. To audit across the canonical four-repo scope (Intention 41), loop the query over each `--root` (see §7) — there is no cross-repo `JOIN`. Counts and IDs are local to one root, so don't UNION raw IDs across runs; aggregate on `path` instead.
- **Path slicing extracts the crate name.** `substr(path, 1, instr(substr(path, 8), '/') + 6)` reconstructs `crates/<name>` from any path under it. Reusable for any per-crate aggregation; see Examples.

### Quoting from the shell

For interactive use, the positional form is the cleanest — wrap the SQL in double quotes and use single quotes for any embedded literals:

```bash
librarian-cli sql "SELECT path FROM files WHERE path LIKE 'crates/%' LIMIT 5"
```

When the query embeds many quotes or spans multiple lines, prefer `--query-file` or stdin to skip shell-quoting entirely:

```bash
librarian-cli sql --query-file /tmp/audit.sql
librarian-cli sql - <<'SQL'
SELECT f.path, COUNT(d.id) AS n_defs
FROM files f
JOIN file_definitions d ON d.file_id = f.id
GROUP BY f.id ORDER BY n_defs DESC LIMIT 10
SQL
```

The legacy JSON-envelope form (`{"q":"raw_sql","args":{...}}`) still works for batch automation, but the first-class subcommand is the recommended path for everything interactive.

---

## 10. Pitfalls and surprises

Real things that bit the first-time user on real sessions.

- **`ofpf-around` uses grep-style `-A`/`-B`.** The intuitive `--max-context` is wrong. The error message helpfully suggests `--max-matches` instead — which is also wrong but rhymes.
- **`ofpf-search-files` uses the indexed code DB.** It does not see `.json`, `.md`, `.toml`, shell scripts, or anything not in the language index. To find non-Rust files by name use `ofpf-content --files-with-matches --regex "." --glob "**/*.json"`.
- **`ofpf-content` is literal by default.** Special characters (`<`, `>`, `(`) are searched literally, which is usually what you want — but `From<TerminalWaterShader>` matched zero lines because the actual code uses `From<&TerminalWaterShader>` (with the reference). Broaden first (`impl From` + `--glob`) when literal queries surprise you.
- **`recyclebin/` is indexed and silently included** in `ofpf-content` / `ofpf-defs` results. Easy to mistake archived code for live code. Filter explicitly when the question is about live state (§6, §7).
- **`ofpf-blast` looks larger than it is** when re-exporting `lib.rs` files appear. Crate roots that `pub use` the type count as direct dependents. Read the list before treating the count as the change blast radius.
- **`ofpf-defs` uses readable names, not compact keys.** Response is `[{"def": "<kind> <name>", "pos": "path:line"}]` (post-2026-04-27 — `loc` was renamed to `pos`). Other commands use the abbreviated `{p, ln, k, lines}` keys; `defs` does not. `librarian-cli meta` decodes every wire key in scope; let the response shape tell you which command uses which.
- **`ofpf-meta` is now the wire-key decoder, not metadata-header search.** Pre-2026-04-27 the alias meant "search OFPF `<DESC>`/`<WCTX>`/`<CLOG>`/`<VERS>` headers"; that capability moved to `ofpf-search-meta`. Updating muscle memory here saves a confusing empty-result session.
- **Old wire keys are gone — no deprecation aliases.** If your tooling reads `loc` (for position), `lines` (for LOC count), `l`/`n` (in `around`), `def` (on `references`), `depth`/`tests` (on `blast-deep`), `path` (on federated `trace`), or `barrel` (role), it sees `null` / undefined until you rename. Run `librarian-cli meta | jq '._keys | keys'` to confirm the current shape.
- **`ofpf-load --root <path>` is additive, not destructive.** It loads or refreshes that repo; it does not unload anything else. Up to ten repos may sit in memory at once.
- **`ofpf-extract` requires both `<path>` AND `<symbol>`.** It is not a "show me anything in this file" tool. For that, use `ofpf-read --from N --to M` or `ofpf-inspect`.
- **The daemon may blacklist noisy files.** Look for `notices[].code == "watcher_noisy_file_blacklisted"` in `ofpf-status`. Auto-generated docs and watch-rebuilt artifacts commonly trip this. Side effect: changes to those files do not invalidate the graph immediately.
- **`ofpf-tests` returns a confidence score.** Low confidence usually means the test name is structurally similar but lives outside the conventional directory layout. Verify by reading.
- **`ofpf-blast` is direct dependents only.** Use `ofpf-blast-deep` for transitive analysis with grouping, tests, and `--depth` control. Pair with `--why` to get the chain.
- **`ofpf-refs` tracks file-level imports, not individual call sites.** "Which files import the defining file?" — yes. "Which call sites of this function pass argument X?" — no, that needs `ofpf-around` or `call-hierarchy`. See §4 for the composition rule.
- **Python repos have known indexing gaps** for class inheritance, decorators, dependency injection, dynamic imports, and metaclasses. The `--help-json notes.python_limitation` field documents this. Verify critical changes manually in framework-heavy code.
- **The `meta` command exists.** When a response key looks abbreviated and you cannot guess what it means, run `librarian-cli meta` instead of guessing.

---

## 11. What NOT to use `ofpf-*` for

- **Building, testing, formatting, linting** — `cargo`, `cargo test`, `cargo fmt`, `cargo clippy`, `just`, `cargo xtask`. The librarian indexes; it does not execute.
- **Editing files** — use `Edit` / `Write` tools. The librarian is read-only against the source tree.
- **Git operations** — `git status`, `git log`, `git diff`, `git show` directly. The librarian does not interpret refs or commits.
- **Shell automation** — pipelines, redirects, env vars, processes. The librarian is a query interface, not a shell.
- **Reading non-text binary content** — images, PDFs, archives. Use the appropriate viewer.
- **Anything time-sensitive that needs a fresh write-then-read cycle** — the watcher debounces and may take seconds to reflect a change. For tight loops use direct file IO.

---

## 12. Reference and escalation

- **Canonical command schema:** `librarian-cli --help-json`
- **Response key decoder:** `librarian-cli meta` / `ofpf-meta` (alias renamed in 2026-04-27 release; `ofpf-search-meta` is the new home for OFPF metadata-header search)
- **Embedded decoder + escape hatches:** `ofpf-orientation` (returns `data.keys`, `data.keys_version`, `data.escape_hatches.sql`)
- **Read-only SQL:** `librarian-cli sql "<query>"` / `ofpf-sql` (see §9)
- **Per-command help:** `librarian-cli <subcommand> --help` (e.g., `librarian-cli around --help`, `librarian-cli sql --help`)
- **Daemon health and graph age:** `ofpf-status`
- **Force regenerate the graph:** `ofpf-load --root <path>`
- **Daemon log file (default port 3333):** `/tmp/librarian-daemon-3333.log`
- **Bug reports and feature requests:** `ofpf-bug` (template) → `ofpf-submit-bug` (file). Three failed retries with corrected syntax = stop and submit.
- **Workflow templates:** `librarian-cli templates guide` and `librarian-cli templates report`.

After a CLI/daemon upgrade, both versions must match — re-install the binary alongside the daemon (the upstream release notes call this out explicitly).

---

## 13. Maintaining this document

Per Intention 42, when you discover a non-obvious flag, an empty-result interpretation, a tool combination that solves a recurring question, or a new pitfall, add it here. The reference is a living artifact whose value compounds with every session that contributes to it. Bump the file's `<VERS>` (PATCH for additions, MINOR for restructuring), update `<WCTX>` to one line about the current pass, and update `<CLOG>` to one line about the most recent change only — git holds the running history.

If a section grows beyond ~80 lines, consider splitting it into a sibling reference (e.g., `OFPF-SQL.md` for the §9 surface if the recipe library outgrows its block) and link from here. Keep this top-level document scannable.

<!-- <FILE>steering/OFPF-TOOLS.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.4.2</VERS> -->
