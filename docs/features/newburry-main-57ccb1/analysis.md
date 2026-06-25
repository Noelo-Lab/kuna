# newburry/main — angr-vs-kuna analysis (newburry-main-57ccb1)

- **angr testcase:** `test_decompiling_newburry_main :: main`
- **Binary:** `/home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/newbury` (lighttpd `server.c::main`)
- **angr version:** 9.2.213, function @ `0x40f696`
- **Selector:** `main` (arch x86_64)

## The gap (one concrete structural difference)

Metrics (reference = angr | kuna):

| metric | angr | kuna |
|---|---|---|
| gotos | **0** | **1** |
| labels | **0** | **1** |
| ifs | 20 | 16 |
| loops | 6 | 6 |
| ternaries | 1 | 1 |

The **only** structural defect is a single `goto` + `label`. Everything else
matches (both 6 loops, ~equal if/ternary counts). kuna's output is otherwise
clean and correct.

### kuna (abridged)

```c
  v2 = getuid();
  if (v2 == 0) {
label_f6ec:
    setlocale(2,0x43620);
    tzset();
    do { ... big body / server loop ... } while (dat_5f278 != 0);   // BODY
  }
  else {
    v2 = geteuid();
    v3 = getuid();
    if (v2 == v3) {
      v2 = getegid();
      v3 = getgid();
      if (v2 == v3) goto label_f6ec;       // <-- forward goto INTO the then-branch's BODY
    }
    v2 = -1;
    fputs("Are you nuts ? Don\'t apply a SUID bit to this binary\n",dat_5f140);
  }
  return v2;
```

### angr (abridged)

```c
  if (getuid()) {
      v1 = geteuid();
      if (v1 != getuid() || !(getegid() == getgid())) {
          fputs("Are you nuts ? ...", stderr);
          return 4294967295;               // exceptional path nested + early-return
      }
  }
  setlocale(2, "C");                        // BODY is the fall-through JOIN
  tzset();
  while (true) { ... big body / server loop ... }
```

## What angr does better

The SUID self-check has a body region **BODY** (`setlocale; tzset; <server loop>`)
that is reached from **two predecessors**:

1. the `getuid() == 0` path, and
2. the "euid==uid && egid==gid all pass" path.

- **kuna** (verbatim Ghidra `CollapseStructure`/`TraceDAG`) nests BODY inside the
  first `if`-then and reaches it from the second path via a **forward `goto` into
  that branch** (`goto label_f6ec`). Syntactically valid, but a goto + label.
- **angr** (SAILR/Phoenix condition-based structurer) **inverts the guard
  conditions** so the *exceptional* `fputs; return -1` path becomes the nested
  branch, and **BODY becomes the fall-through join** after the guard. No goto, no
  label.

## Owning stage

- **S8 structuring** — `decompiler/crates/kuna-decomp/src/s8_structure/blockaction.rs`
  (verbatim port of Ghidra `CollapseStructure` / `TraceDAG`). The goto is emitted
  here when the collapse cannot find a structured form for a multi-predecessor
  join, so it falls back to a goto edge.
- The would-be fix layer is **S7 region identification**
  (`s7_regions/kuna_regionid.rs`), kuna's port of angr's `RegionIdentifier`, which
  is currently a **read-only console surface** (`region tree`) and does **not**
  drive emit.

## Hypothesis for the kuna change (why it is LARGE)

To reproduce angr's output you must **invert the guard conditions** so the
exceptional path nests and the shared BODY becomes the fall-through join, then
re-parent BODY out of the first `if`-then. This is **condition-based block-graph
restructuring**, not a value-level Action/Rule:

- It cannot be done as a single gated early-return in `blockaction.rs` — that file
  can only *suppress* work, it cannot *synthesize* a new guard inversion and
  re-parent a join.
- The canonical small template (`kuna_loweredswitch.rs`) is an **S2 pcode-level**
  Action and is not applicable to block-graph restructuring.
- It is the **same root** as the known `gotoreduce` gap
  (morton/my_message_callback, proposal **PR #54**) and the `irreducibleloops`
  family (**PR #46**): all need kuna's read-only `RegionIdentifier` promoted into a
  real **emit-path SAILR/Phoenix condition-based structurer**. That is a new pass
  *type* / infrastructure → trips **Hard rule 7** (new pass type + touches S8/S7
  structuring beyond a gated early-return).

**Scope decider verdict:** `large` · family `gotoreduce` · recommended action
`propose`. See `record.json` `decisions`.

→ Filed as a `[PROPOSAL]` draft PR. No implementation in this session.
