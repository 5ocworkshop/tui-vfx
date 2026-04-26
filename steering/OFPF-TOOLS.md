<!-- <FILE>steering/OFPF-TOOLS.md</FILE> - <DESC>Project-local practical reference for the ofpf-* tooling suite (a thin alias layer over librarian-cli, backed by a multi-tenant daemon). Decision matrix by intent, output-handling patterns, multi-repo workflow, response-guard semantics, common pitfalls, and the non-obvious flags that bite first-time users. Required reading per Intention 42.</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Initial project-local distillation of the ofpf-*/librarian-cli surface, written after a real fire-shader-prep session that exercised the tools against three loaded repos.</WCTX> -->
<!-- <CLOG>0.1.0: initial reference covering the 80/20 tools, decision matrix, output handling, multi-repo, response guard, exit codes, JSON mode, common pitfalls, and what NOT to use ofpf-* for.</CLOG> -->

# OFPF Tools — practical reference

The `ofpf-*` suite is a thin alias layer over `librarian-cli`, which talks to a long-running multi-tenant `librarian-daemon`. One daemon serves multiple loaded repositories at once (up to ten in parallel). This document is the **project-local** reference for *which tool answers which question*, *how to handle the output*, and *which non-obvious things bite first-time users*. Required reading per Intention 42.

The global standards in `~/.claude/rules/ofpf.md` and `~/.claude/CLAUDE.md` introduce the suite and its philosophy. This file complements them with the operational details a developer or AI agent needs in-flight.

When in doubt about a flag or behavior, run `librarian-cli --help-json` (canonical command schema) and `librarian-cli meta` (decoder for the abbreviated response keys). Both are authoritative; this document is curated.

---

## 1. The 80/20 tools

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
| `ofpf-refs <name>` | `references` (`find-refs`/`refs`) | Files that import / re-export a definition |
| `ofpf-orientation` | `orientation` | Architecture bundle: overview + hotspots + inspect — first call on a new repo |

---

## 2. Decision matrix by intent

| Intent | Tool | Notes |
|---|---|---|
| First contact with a repo | `ofpf-orientation` | Returns roles (hub/core/unit), top hotspots, fan-in/out metrics |
| "Where is symbol X defined?" | `ofpf-defs X` | Add `--kind function\|method\|class\|struct` to disambiguate |
| "Who uses symbol X?" | `ofpf-refs X` | File-level imports + re-exports + barrel files. *Not* call-site granularity |
| "What calls function `foo`?" | `ofpf-callers foo` (`call-hierarchy --direction incoming`) | Function-call edges; supports `--depth` |
| "What does function `foo` call?" | `ofpf-callees foo` (`call-hierarchy --direction outgoing`) | Same trait, opposite direction |
| "Is this function dead code?" | `ofpf-dead --scope file --path <p>` | Or `--scope project` for repo-wide pass |
| "Where is the literal string `X` mentioned?" | `ofpf-content X` | Literal by default. Add `--regex` for patterns, `--glob "**/*.json"` for non-indexed files |
| "All `X` touch sites in one file with context" | `ofpf-around <path> X -A 5 -B 1` | grep-style `-A`/`-B`. Default 5 each |
| "Read just one symbol from a big file" | `ofpf-extract <path> <symbol>` | Both args required |
| "Read lines N–M of a file" | `ofpf-read <path> --from N --to M` | Or `--paths a.rs b.rs --range a.rs:10:30 --range b.rs:1:50` for multi-file |
| "What does this file do?" | `ofpf-inspect <path>` | One call returns defs + callers + callees + role + metrics + tests |
| "Should I be worried about changing this file?" | `ofpf-inspect <path>` then `ofpf-blast <path>` | High fan-in → wide blast radius. `--why` shows the dependency chain |
| "Find dependency path between two files" | `ofpf-trace <from> <to>` | Single-repo with `--root`; federated with `--workspace-id` |
| "Is there a circular dependency?" | `ofpf-cycles` | Returns cycles with refactoring suggestions |
| "Which files are too big?" | `ofpf-loc 300` | Files with >300 lines (default threshold) |
| "What can run in parallel?" | `ofpf-dag` | Files grouped by execution tier |
| "What `<DESC>` text does this file's metadata header carry?" | `ofpf-meta X` | OFPF metadata search (`--tag desc\|wctx\|clog\|vers`) |
| "What tests exist for this source?" | `ofpf-tests <path>` (`verify`) | Returns path + relation + confidence |
| "Which files are imported by file `Y`?" | `ofpf-context <path>` (`dependencies`) | Imports graph for one file |

---

## 3. Aliases the standards docs do not list

`librarian-cli --help-json` reveals natural-English aliases that the standards docs do not surface. Use whichever feels native:

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
| `metadata` (extract) | (no alias) |
| `extract` | `snippet`, `show-symbol` |

Both positional and `--flag <value>` forms work for primary args (`-q`/`--query`, `-p`/`--path`, `-k`/`--kind`, `-l`/`--lang`, `-d`/`--depth`, `-s`/`--symbol`, `-S`/`--scope`).

---

## 4. Output handling

Responses are JSON. The shape is `{ data, error, notices, guard?, req_id }`:

- `data` — command result; structure varies by command
- `error` — `null` on success, otherwise an error string
- `notices` — warnings (version mismatch, blacklisted noisy file, etc.)
- `guard` — present only when result count or size exceeded the limit
- `req_id` — daemon-side request id (useful when filing `ofpf-bug`)

### Decoder for compact keys

`librarian-cli meta` returns the official mapping. The most common abbreviations:

- `co` → cohesion
- `in` → fan_in
- `out` → fan_out
- `f` → file
- `p` → path
- `n` → name (or, in `around`, line text — confusingly polymorphic)
- `l` → line (number) — but inside `around.bl[].l[]` it is the line array
- `k` → kind
- `mod` → module
- `bl` → blocks (in `around` results)
- `ml` → matched-lines
- `s`/`e` → start/end (block bounds)
- `m` (boolean) → match marker on the matched line itself

Run `librarian-cli meta` to refresh — the surface evolves.

### Filter the JSON before reading

Most useful calls produce more JSON than is worth reading raw. Pipe through `python3 -c` for surgical extraction. Examples that came up in real work:

```bash
# All TerminalWater touch sites in one file with context, formatted readably
ofpf-around <path> "TerminalWater" -A 4 -B 1 | python3 -c "
import json, sys
d = json.load(sys.stdin)
for b in d['data']['bl']:
    print('===', b['s'], '-', b['e'], '===')
    for ln in b['l']:
        marker = '*' if ln['m'] else ' '
        print(f'{marker}{ln[\"n\"]:4}: {ln[\"t\"]}')
"

# Just the locations from a content search
ofpf-content "<pattern>" | python3 -c "
import json, sys
d = json.load(sys.stdin)
for r in d['data']:
    print(r['loc'])
"
```

`jq` works too if you prefer it; the structure is consistent enough that either is fine.

### Response guard

Triggered when `result_count > 100` OR `response_size > 100KB` (configurable via `--max-kb`). The daemon returns a preview (first 5 items) plus options:

- `--force` — return the full result anyway (use only when you actually need it)
- `--limit N --offset M` — paginate
- Refine the query (better)

Applies to: `search`, `search-defs`, `search-content`, `search-meta`, `references`, `blast`, `blast-deep`. **Refine first, paginate second, force last** — the same playbook as the standards doc.

### Exit codes follow grep semantics

- `0` — success / matches found
- `1` — **no matches (NOT an error)**. Exit 1 from `search-content` or `search-defs` is "the search worked and found zero hits." Treat as valid data.
- `2` — actual error (bad arguments, daemon down, etc.)

Useful in pipelines: `ofpf-content X && echo found || [[ $? -eq 1 ]] && echo "no matches"`.

---

## 5. Multi-repo workflow

The daemon is multi-tenant. `--root <path>` selects which loaded repo a query targets; it does **not** swap a single active root. Up to ten repos can stay loaded in parallel.

```bash
ofpf-load --root /usr/projects/mixed-signals       # adds, doesn't swap
ofpf-defs --root /usr/projects/mixed-signals fbm3
ofpf-extract --root /usr/projects/tui-vfx-recipes <path> <symbol>
```

If you omit `--root`, the call resolves against the daemon's notion of the current root (typically the CWD when the daemon was started or the most recently loaded). Always pass `--root` explicitly when crossing repo boundaries — it removes ambiguity.

For genuine federation across many repos in one query, see `--workspace-id` in `librarian-cli --help-json` (mutually exclusive with `--root`). Workspaces are the right tool when you want one search to span all four canonical repos (Intention 41) without per-repo iteration.

### When the graph is stale

`ofpf-status` reports `is_stale: true` and `stale_reason` when the indexed graph has fallen behind the working tree. The daemon auto-regenerates on file changes (rate-limited), but you can force it with `ofpf-load --root <path>`. Most queries are still useful while stale; treat the staleness flag as a hint, not a blocker. For audits where exactness matters, force a reload first.

---

## 6. JSON mode for scripting

Pipe one or more requests as JSON to stdin with `--json`:

```bash
echo '{"q":"stats"}' | librarian-cli --json
```

NDJSON works for batches:

```bash
echo -e '{"q":"stats"}\n{"q":"overview"}' | librarian-cli --json
```

Common arg keys in JSON mode: `q` (query OR command name in batch mode), `p` (path), `kind`, `depth`, `gates`, `snippets`. Schema in `librarian-cli --help-json` under `json_mode`.

---

## 7. Pitfalls and surprises

Real things that bit the first-time user (me) on a real session.

- **`ofpf-around` uses grep-style `-A`/`-B`.** The intuitive `--max-context` is wrong. The error message helpfully suggests `--max-matches` instead — which is also wrong but rhymes.
- **`ofpf-search-files` uses the *indexed* code DB.** It does not see `.json`, `.md`, `.toml`, shell scripts, or anything not in the language index. To find non-Rust files by name use `ofpf-content --files-with-matches --regex "." --glob "**/*.json"`.
- **`ofpf-content` is literal by default.** Special characters (`<`, `>`, `(`) are searched literally, which is usually what you want — but `From<TerminalWaterShader>` matched zero lines because the actual code uses `From<&TerminalWaterShader>` (with the reference). Broaden first (`impl From` + `--glob`) when literal queries surprise you.
- **`ofpf-load --root <path>` is additive, not destructive.** It loads or refreshes that repo; it does not unload anything else. Up to ten repos may sit in memory at once.
- **`ofpf-extract` requires both `<path>` AND `<symbol>`.** It is not a "show me anything in this file" tool. For that, use `ofpf-read --from N --to M` or `ofpf-inspect`.
- **The daemon may blacklist noisy files.** Look for `notices[].code == "watcher_noisy_file_blacklisted"` in `ofpf-status`. Auto-generated docs and watch-rebuilt artifacts commonly trip this. Side effect: changes to those files do not invalidate the graph immediately.
- **`ofpf-tests` (`verify`) returns a confidence score.** Low confidence usually means the test name is structurally similar but lives outside the conventional directory layout. Verify by reading.
- **`ofpf-blast` is direct dependents only.** Use `ofpf-blast-deep` for transitive analysis with grouping, tests, and `--depth` control. Pair with `--why` to get the chain.
- **`ofpf-refs` tracks file-level imports, not individual call sites.** "Which files import this symbol?" — yes. "Which call sites of this function pass argument X?" — no, that needs `ofpf-around` or `call-hierarchy`.
- **Python repos have known indexing gaps** for class inheritance, decorators, dependency injection, dynamic imports, and metaclasses. The `--help-json notes.python_limitation` field documents this. Verify critical changes manually in framework-heavy code.
- **The `meta` command exists.** When a response key looks abbreviated and you cannot guess what it means, run `librarian-cli meta` instead of guessing. The canonical decoder is one call away.

---

## 8. What NOT to use `ofpf-*` for

- **Building, testing, formatting, linting** — `cargo`, `cargo test`, `cargo fmt`, `cargo clippy`, `just`, `cargo xtask`. The librarian indexes; it does not execute.
- **Editing files** — use `Edit` / `Write` tools. The librarian is read-only against the source tree.
- **Git operations** — `git status`, `git log`, `git diff`, `git show` directly. The librarian does not interpret refs or commits.
- **Shell automation** — pipelines, redirects, env vars, processes. The librarian is a query interface, not a shell.
- **Reading non-text binary content** — images, PDFs, archives. Use the appropriate viewer.
- **Anything time-sensitive that needs a fresh write-then-read cycle** — the watcher debounces and may take seconds to reflect a change. For tight loops use direct file IO.

---

## 9. Reference and escalation

- **Canonical command schema:** `librarian-cli --help-json`
- **Response key decoder:** `librarian-cli meta`
- **Per-command help:** `librarian-cli <subcommand> --help` (e.g., `librarian-cli around --help`)
- **Daemon health and graph age:** `ofpf-status`
- **Force regenerate the graph:** `ofpf-load --root <path>`
- **Daemon log file (default port 3333):** `/tmp/librarian-daemon-3333.log`
- **Bug reports:** `ofpf-bug` (template) → `ofpf-submit-bug` (file). Three failed retries with corrected syntax = stop and submit.
- **Workflow templates:** `librarian-cli templates guide` and `librarian-cli templates report`.

---

## 10. Maintaining this document

Per Intention 42, when you discover a non-obvious flag, an empty-result interpretation, a tool combination that solves a recurring question, or a new pitfall, **add it here**. The reference is a living artifact whose value compounds with every session that contributes to it. Bump the file's `<VERS>` (PATCH for additions, MINOR for restructuring), update `<WCTX>` to one line about the current pass, and update `<CLOG>` to one line about the most recent change only — git holds the running history.

If a section grows beyond ~80 lines, consider splitting it into a sibling reference (e.g., `OFPF-WORKSPACES.md` for federated multi-repo workflow) and link from here. Keep this top-level document scannable.

<!-- <FILE>steering/OFPF-TOOLS.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
