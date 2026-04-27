<!-- <FILE>steering/OFPF-TOOLS.md</FILE> - <DESC>Project-local practical reference for the ofpf-* tooling suite (a thin alias layer over librarian-cli, backed by a multi-tenant daemon). Lead-with framing for code access, the 80/20 tool surface, decision matrix by intent, composition rules that prevent wrong answers, output handling, multi-repo workflow, the raw_sql escape hatch, pitfalls, and the non-obvious flags that bite first-time users. Required reading per Intention 42.</DESC> -->
<!-- <VERS>VERSION: 0.3.0</VERS> -->
<!-- <WCTX>Add §9 raw_sql escape hatch — the daemon-side SQL surface (dispatcher-only, no CLI subcommand yet) discovered during practice session. Documents schema, safety envelope, default-vs-max limits, EXPLAIN/PRAGMA pitfalls, and example queries.</WCTX> -->
<!-- <CLOG>0.3.0: add §9 raw_sql escape hatch with schema, safety envelope, examples, and surprises; renumber subsequent sections.</CLOG> -->

# OFPF Tools — practical reference

The `ofpf-*` suite is a thin alias layer over `librarian-cli`, which talks to a long-running multi-tenant `librarian-daemon`. One daemon serves up to ten loaded repositories in parallel. This document is the project-local reference for which tool answers which question, how to handle the output, and which non-obvious things bite first-time users. Required reading per Intention 42.

The global standards in `~/.claude/rules/ofpf.md` and `~/.claude/CLAUDE.md` introduce the suite and its philosophy. This file complements them with the operational detail a developer or AI agent needs in-flight.

When in doubt about a flag or behavior, run `librarian-cli --help-json` (canonical command schema) and `librarian-cli meta` (decoder for the abbreviated response keys). Both are authoritative; this document is curated.

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
| What is the *intent* of this file? | `ofpf-meta <q> --tag desc` (skip the source read) |
| First contact with a repo | `ofpf-orientation` |

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
| `ofpf-orientation` | `orientation` | Architecture bundle: overview + hotspots + inspect — first call on a new repo |

---

## 3. Decision matrix by intent

| Intent | Tool | Notes |
|---|---|---|
| First contact with a repo | `ofpf-orientation` | Returns roles (hub/core/unit), top hotspots, fan-in/out metrics |
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
| "Find dependency path between two files" | `ofpf-trace <from> <to>` | Single-repo with `--root`; federated with `--workspace-id` |
| "Is there a circular dependency?" | `ofpf-cycles` | Returns cycles with refactoring suggestions |
| "Which files are too big?" | `ofpf-loc 300 --filter <prefix>` | Files over the threshold; `--filter` scopes to a subtree |
| "What can run in parallel?" | `ofpf-dag` | Files grouped by execution tier |
| "What `<DESC>` text does this file's metadata header carry?" | `ofpf-meta X --tag desc` | OFPF metadata search (`--tag desc\|wctx\|clog\|vers`) |
| "What tests exist for this source?" | `ofpf-tests <path>` | Returns path + relation + confidence |
| "Which files are imported by file `Y`?" | `ofpf-context <path>` | Imports graph for one file |

---

## 4. Composition rules that prevent wrong answers

Two patterns to memorize. Each catches a class of mistake the rest of the suite cannot prevent on its own.

### Pair `ofpf-refs` and `ofpf-content` for usage questions

`ofpf-refs <symbol>` returns *files that import the symbol's defining file*. It is the import-graph half of the answer, and it does not enumerate every text occurrence.

`ofpf-content <symbol>` returns every text occurrence (rg-backed) — including doc comments, design notes, archived code under `recyclebin/`, and string literals.

Use both. `refs` answers "what would breaking this re-export break?"; `content` answers "what reads this name at all?" Either alone undercounts.

Concrete: `ofpf-refs VfxBindableU16` returns one file (the typedef's parent `mod.rs`) because the alias lives in one file and only that file imports the canonical home. `ofpf-content "VfxBindableU16"` returns ~23 files (real call sites, tests, design docs, the consolidated `recyclebin/` history).

### File role ≠ symbol reach

`ofpf-inspect` returns `role: unit | core | hub` based on the *file's* fan-in/fan-out in the import graph. A file with `role: unit` and `fan_in: 1` may still define a type used in 100 files — if all 100 import via a re-exporting `mod.rs`, the file's own fan-in stays at 1.

When the question is "is this type widely used?", inspect the type, not just the file. `ofpf-content "<TypeName>"` is the second call. `ofpf-blast` likewise reports the file's direct dependents, not the type's reach.

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

Both positional and `--flag <value>` forms work for primary args (`-q`/`--query`, `-p`/`--path`, `-k`/`--kind`, `-l`/`--lang`, `-d`/`--depth`, `-s`/`--symbol`, `-S`/`--scope`).

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

A few examples worth knowing before you parse:

- `ofpf-defs <q>` → `[{"def": "<kind> <name>", "loc": "path:line"}]` — readable strings, no compact keys.
- `ofpf-content <q>` → `[{"loc", "content", "kind", "match_start", "match_end"}]`.
- `ofpf-inspect <p>` → `{"defs", "callers", "callees", "metrics": {"co", "in", "out", "role"}, "lines", ...}`.
- `ofpf-around <p> <q>` → `{"bl": [{"s", "e", "l": [{"n", "t", "m"}]}]}` — blocks of lines with start/end and per-line line-number/text/match-flag.

When a key looks unfamiliar, `librarian-cli meta` is the decoder. The most common abbreviations are:

- `co` → cohesion
- `in` → fan_in
- `out` → fan_out
- `f` → file
- `p` → path
- `n` → name (or, in `around`, line number)
- `l` → line (number) — but inside `around.bl[].l[]` it is the line array
- `k` → kind
- `mod` → module
- `bl` → blocks (in `around` results)
- `ml` → matched-lines
- `s`/`e` → start/end (block bounds)
- `m` (boolean) → match marker on the matched line itself

Run `librarian-cli meta` to refresh — the surface evolves.

### When filtering is worth it

Three patterns that earn the extra tokens:

```bash
# Drop recyclebin/ noise from a content search
ofpf-content "<pattern>" | python3 -c "
import json, sys
for r in json.load(sys.stdin)['data']:
    if not r['loc'].startswith('recyclebin/'):
        print(r['loc'])
"

# Compact view of a wide ofpf-around response
ofpf-around <path> "<query>" -A 4 -B 1 | python3 -c "
import json, sys
d = json.load(sys.stdin)['data']
for b in d['bl']:
    print('===', b['s'], '-', b['e'], '===')
    for ln in b['l']:
        print(f\"{'*' if ln['m'] else ' '}{ln['n']:4}: {ln['t']}\")
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

For genuine federation across many repos in one query, see `--workspace-id` in `librarian-cli --help-json` (mutually exclusive with `--root`). Workspaces are the right tool when one search should span all four canonical repos (Intention 41) without per-repo iteration.

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

## 9. The `raw_sql` escape hatch

The daemon exposes a `raw_sql` intent that runs read-only SQL directly against the index database. It is reachable today through the JSON protocol only — there is no `librarian-cli sql` subcommand yet, and `librarian-cli --help` does not list `raw_sql`. A pending feature request asks for first-class CLI exposure (`librarian-cli sql` / `ofpf-sql`).

`raw_sql` is the universal answer to questions the high-level commands can't compose: filtering by multiple criteria, JOINs across metrics + paths + edges, custom aggregations, schema introspection.

### When to reach for it

After exhausting:

- The locator commands (`defs`, `content`, `refs`, `around`)
- The traversal commands (`inspect`, `blast`, `trace`, `focus`, `context`)
- The composition rules in §4 (`refs` + `content`, `metrics` + `loc`)

If the question still doesn't fit any single command — for example "files with high fan_in AND >400 LOC AND no test peer," or "top external crates by import count," or "all public traits across the workspace" — drop to `raw_sql`.

### Invocation

JSON-API form (works today):

```bash
echo '{"q":"raw_sql","args":{"query":"SELECT COUNT(*) FROM files"}}' \
    | librarian-cli --json --root /usr/projects/tui-vfx
```

`args` keys:

- `query` (or `q`): the SQL string. Required.
- `limit`: max rows. Default 100. Hard max 500.
- `offset`: pagination offset.
- `timeout_ms`: per-query timeout. Default 1000. Hard max 5000.

The query is auto-wrapped as `SELECT * FROM (<your-query>) AS _q LIMIT ?1 OFFSET ?2` for safety. Side effect: not every SQL form composes through that wrapper — see Surprises below.

### Schema (15 tables)

| Table | What it holds |
|---|---|
| `files` | id, path, kind, lang, lines, zero_deps, generated |
| `file_metrics` | file_id, fan_in, fan_out, cohesion, role (`unit` / `hub` / `core` / `barrel`), is_barrel (virtual) |
| `file_edges` | source_file_id, target_file_id, edge_type (`logic` / `crate_dep`) |
| `file_definitions` | id, file_id, name, kind, line, end_line, parent, doc, visibility, is_test, test_attributes |
| `symbol_edges` | source_def_id, target_def_id, call_site_line, edge_type — call graph |
| `dependencies` | source_file_id, target_module_id, is_dynamic — import edges |
| `resolved_imports` | file_id, raw_statement, resolved_def_id, is_external, external_crate |
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

-- Hubs by fan_out
SELECT f.path, m.fan_out, m.fan_in
FROM file_metrics m JOIN files f ON f.id = m.file_id
WHERE m.role = 'hub' ORDER BY m.fan_out DESC LIMIT 10;

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

-- OFPF DESC text mentioning a topic (ofpf-meta is the higher-level form)
SELECT om.file_path, om.description FROM ofpf_metadata om
WHERE om.description LIKE '%bindable%' AND om.is_ofpf = 1;

-- Async or unsafe definitions (not surfaced by any high-level command)
SELECT f.path, d.name, ti.is_async, ti.is_unsafe
FROM type_info ti JOIN file_definitions d ON d.id = ti.def_id
JOIN files f ON f.id = d.file_id
WHERE ti.is_async = 1 OR ti.is_unsafe = 1;
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

Every response carries a `pagination` block: `{returned, next_offset, has_more}`. Default `limit` is 100; the hard cap is 500. For corpus-wide walks:

```python
offset = 0
while True:
    args = {"query": "SELECT path FROM files ORDER BY id", "limit": 500, "offset": offset}
    r = sql(args)
    rows = r["data"]["records"]
    if not rows: break
    for row in rows: process(row)
    if not r["data"]["pagination"]["has_more"]: break
    offset = r["data"]["pagination"]["next_offset"]
```

### Surprises

- **Default `limit` is 100, not 500.** A bare `SELECT id FROM file_definitions` returns 100 rows with `has_more: true`. Pass `limit: 500` to reach the cap; for full corpus walks, paginate with `offset`.
- **PRAGMA is universally denied.** Schema introspection uses `sqlite_master`, not `PRAGMA table_info`. Index list: `SELECT name FROM sqlite_master WHERE type='index'`.
- **`EXPLAIN` is unreachable** through this surface. The auto-wrap subquery is incompatible with `EXPLAIN <query>`. There is no current path to inspect query plans.
- **`definitions` is a link table** (module_id ↔ file_id) — it is *not* the symbol table. The symbol table is `file_definitions`. This is the most common mistake on first use.
- **`file_metrics.is_barrel` is a virtual generated column** (`role = 'barrel'`), but no files in this repo currently carry the `barrel` role. The classification exists in code without being populated by the indexer here. Verify before relying on it.
- **`zero_deps` and `generated` flags are always 0** in this repo. Either tui-vfx genuinely has no zero-dep / no generated files, or the indexer doesn't populate them. Check before filtering on them.
- **Default `timeout_ms` is 1000.** Aggressive multi-JOIN queries against the full corpus need an explicit `timeout_ms: 5000` to use the headroom.
- **Read-only side-effect functions are allowed.** `randomblob()`, `random()`, etc. work — they're SELECT-shaped and the authorizer permits them.

### Quoting from the shell

The JSON-mode form requires JSON-encoded SQL, which means standard JSON escaping (`\"` for quotes, `\\` for backslashes). For interactive use, write the JSON request to a file or stdin:

```bash
cat <<'EOF' | librarian-cli --json --root /usr/projects/tui-vfx
{"q":"raw_sql","args":{"query":"SELECT lang, COUNT(*) FROM files GROUP BY lang"}}
EOF
```

Or compose with Python (avoids all shell-quoting):

```python
import json, subprocess
def sql(query, **kw):
    args = {"query": query, **kw}
    p = subprocess.run(["librarian-cli","--json","--root","/usr/projects/tui-vfx"],
                       input=json.dumps({"q":"raw_sql","args":args}),
                       capture_output=True, text=True)
    return json.loads(p.stdout)
```

The pending CLI feature request proposes `librarian-cli sql "<query>"` (positional, double-quoted), `librarian-cli sql -` (stdin), and `librarian-cli sql --query-file <path>` to remove the JSON-encoding overhead for interactive use.

---

## 10. Pitfalls and surprises

Real things that bit the first-time user on real sessions.

- **`ofpf-around` uses grep-style `-A`/`-B`.** The intuitive `--max-context` is wrong. The error message helpfully suggests `--max-matches` instead — which is also wrong but rhymes.
- **`ofpf-search-files` uses the indexed code DB.** It does not see `.json`, `.md`, `.toml`, shell scripts, or anything not in the language index. To find non-Rust files by name use `ofpf-content --files-with-matches --regex "." --glob "**/*.json"`.
- **`ofpf-content` is literal by default.** Special characters (`<`, `>`, `(`) are searched literally, which is usually what you want — but `From<TerminalWaterShader>` matched zero lines because the actual code uses `From<&TerminalWaterShader>` (with the reference). Broaden first (`impl From` + `--glob`) when literal queries surprise you.
- **`recyclebin/` is indexed and silently included** in `ofpf-content` / `ofpf-defs` results. Easy to mistake archived code for live code. Filter explicitly when the question is about live state (§6, §7).
- **`ofpf-blast` looks larger than it is** when re-exporting `lib.rs` files appear. Crate roots that `pub use` the type count as direct dependents. Read the list before treating the count as the change blast radius.
- **`ofpf-defs` uses readable names, not compact keys.** Response is `[{"def": "<kind> <name>", "loc": "path:line"}]`. Other commands use the abbreviated `{p, n, k, l}` keys; `defs` does not. `librarian-cli meta` decodes the abbreviated forms but does not enumerate which commands use them — let the response shape tell you.
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
- **Response key decoder:** `librarian-cli meta`
- **Per-command help:** `librarian-cli <subcommand> --help` (e.g., `librarian-cli around --help`)
- **Daemon health and graph age:** `ofpf-status`
- **Force regenerate the graph:** `ofpf-load --root <path>`
- **Daemon log file (default port 3333):** `/tmp/librarian-daemon-3333.log`
- **Bug reports and feature requests:** `ofpf-bug` (template) → `ofpf-submit-bug` (file). Three failed retries with corrected syntax = stop and submit.
- **Workflow templates:** `librarian-cli templates guide` and `librarian-cli templates report`.

---

## 13. Maintaining this document

Per Intention 42, when you discover a non-obvious flag, an empty-result interpretation, a tool combination that solves a recurring question, or a new pitfall, add it here. The reference is a living artifact whose value compounds with every session that contributes to it. Bump the file's `<VERS>` (PATCH for additions, MINOR for restructuring), update `<WCTX>` to one line about the current pass, and update `<CLOG>` to one line about the most recent change only — git holds the running history.

If a section grows beyond ~80 lines, consider splitting it into a sibling reference (e.g., `OFPF-WORKSPACES.md` for federated multi-repo workflow) and link from here. Keep this top-level document scannable.

<!-- <FILE>steering/OFPF-TOOLS.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.3.0</VERS> -->
