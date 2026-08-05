# cppcallnames — a C++ call site renders `sub_<addr>` for a name kuna already knows

Filed against `kuna decompile` / `decompile-all` on unstripped C++ ELFs. The
witness suite is a purpose-built probe (`probe.cpp`, `g++ -O0`/`-O2`) plus three
vendored fixtures already in the tree: `cpp_noreturn_x86_64`, `cpp_mangled_x86_64`,
`eh_lsda_x86_64`.

## The symptom

kuna knows the demangled C++ names — `kuna functions <bin> --json` lists them, and
`kuna decompile <bin> 'Account::deposit'` resolves and decompiles by that name — but
the printer emitted a placeholder at every call site:

```c
int4 probe_member_fields(void *a,int4 amount)
{
  d = sub_271c(a,amount);      /* Account::deposit   */
  v = sub_2776(a);             /* Account::available */
  o = 5;
  sub_279c(a,&o);              /* Account::bump      */
  return o + d + v;
}
```

IDA Pro 9.2 and Ghidra 12.1 both render the qualified names on the same binary
(`d = Account::deposit(a, amount);`). Unmangled C callees (`printf`,
`__cxa_allocate_exception`) rendered fine, which is what made this look like a
demangler problem. It is not: demangling works, and a demangled name with **no
namespace** (`_Z8topleveli` -> `toplevel`) also rendered fine.

A second, independent defect on the same surface: `kuna decompile <bin> --addr
0x271c` printed the header as `sub_271c` even on an **unstripped** binary, while the
by-name path printed `Account::deposit`.

## The real mechanism (verified, not assumed)

Three candidate causes were checked and all three are wrong: the name is *not*
stored only as an alias, there is *no* identifier filter rejecting `::`, and the
call-site path reads the *same* symbol table `kuna functions` does.

The defect is **scope shadowing introduced by the analysis commit seam**, and it is
mode-dependent:

| surface | `Account::deposit` call site |
|---|---|
| `kuna decompile ... --mode reliable` | `Account::deposit(a,amount)` (correct) |
| `kuna decompile ...` (default `auto` -> `aggressive` under 500 KiB) | `sub_271c(a,amount)` (wrong) |
| `--option fast_funcdisc off --option funcstart_patterns off` | `Account::deposit(a,amount)` (correct) |

Turning off *either* discovery pass alone was not enough — both re-report the same
VMA, so either one on is sufficient to install the duplicate.

The chain:

1. `read_loader_symbols` installs each demangled C++ funcsym in its **namespace
   scope**: `Account::deposit` becomes base `deposit` in scope `Account`;
   `std::terminate` becomes base `terminate` in scope `std`.
2. A discovery pass (`fast_funcdisc`, `funcstart_patterns` — both in
   `AGGRESSIVE_OVERRIDES`, `p0_knowledge/modes.rs`) re-reports the same VMA in
   `AnalysisOutput::entries`.
3. `commit_analysis_output`'s discovered-entry arm names it with the synthetic
   `sub_<addr>`, resolves that name to a scope — no `::`, so **GLOBAL** — and probed
   for an existing function *in that scope only*:

   ```rust
   let (scope, base) = arch.symboltab
       .find_create_scope_from_symbol_name(&name, "::", None, num_spaces)?; // -> GLOBAL
   if arch.symboltab.find_function(scope, &addr).is_none() {   // GLOBAL-only: misses scope `Account`
       arch.symboltab.add_function(scope, &addr, &base, min_size, type_code)?;  // duplicate
   }
   ```

   The real symbol lives in scope `Account`, so the probe missed it and a **duplicate
   generic FunctionSymbol** was installed in GLOBAL at the same address.
4. `FlowInfo::queryCall` -> `Database::function_display_name_across_scopes` ->
   `find_function_across_scopes`, which **searches the global scope first**. It found
   the duplicate. The call spec's name became `sub_<addr>`, and `PrintC::opCall`
   printed it.

The in-source comment claiming the probe "still lets a real `.symtab`/`.dynsym` name
win on a non-stripped binary" was false for every `::`-qualified symbol, and
`database.rs` already documented the hazard verbatim on
`find_function_across_scopes` — the two facts had simply never been connected.

The `--addr` defect is unrelated to the above and simpler: `IfcAddrrangeLoad` jumped
straight to `Architecture::name_function` when no explicit name token was given,
never asking the symbol table what was installed at that address.

## Why it was invisible to every gate

* The datatest and stage corpora are XML `<binaryimage>` documents, and that
  bootstrap never produces an `AnalysisOutput` — `commit_analysis_output` is
  structurally a no-op there, so neither corpus can reach the defect.
* `verify_noreturn_demangle` exercises the **console** with the shipped defaults
  (no discovery pass), where the duplicate is never created. It printed
  `std::terminate()` and passed, while the CLI on the same fixture printed
  `sub_401070()`. No test asserted the CLI render.

## The fix

Two one-idea changes, both in the driver tier (`kuna-console`):

1. `engine.rs`, discovered-entry commit arm — resolve the idempotence probe
   **across scopes** (`find_function_across_scopes`, the port of C++
   `Scope::queryFunction`, which spans the scope tree). A function already known
   under a namespaced name is recognized as present, so no placeholder is installed
   over it. Scope creation moves inside the branch, so a skipped add no longer
   creates an empty namespace scope.
2. `ifacedecomp.rs`, `IfcAddrrangeLoad` — when no explicit name is given, prefer
   `function_display_name_across_scopes(offset)` (qualified) and fall back to
   `name_function` only for a genuinely unknown address. `load addr <vma> <name>`
   still wins.

Nothing about discovery changes: the passes still run, still find the same VMAs, and
a genuinely unnamed find still becomes `sub_<addr>`. `register_symbol` is untouched,
so `sub_<addr>` remains a working alias for name-keyed selection.

## Measured effect

Whole-binary `decompile-all` before/after on 13 binaries (8 C, 5 C++), every changed
line classified:

| | binaries | changed lines |
|---|---|---|
| plain C (incl. `/bin/grep` 35,070 lines, `/usr/bin/ls` 14,788 lines) | 8 | **0 — byte-identical** |
| C++ | 5 | 184 |

All 184 changed lines are name renders and nothing else — no structural change, no
signature change, every diff exactly `-N/+N`:

* **181 call-target renames** — the fix (`sub_2170` -> `std::runtime_error::runtime_error`,
  `sub_401136` -> `foo::Bar::baz`, `sub_4011fa` -> `app::Guard::throw_it`, ...).
* **3 address-taken renames** — a function's address used as a value:
  `sub_2ca0` -> `perimeter`, `sub_2d20` -> `emit`, `sub_401120` -> `~runtime_error`.
  These render the symbol's **base** name unqualified, because that render path
  (`Funcdata::link_spacebase_symbol` -> `Architecture::name_for_global_varnode`) uses
  the *unscoped* helper, unlike the `ActionNameVars` global path which applies
  `kuna_qualify_global_name`. That gap is **pre-existing and unrelated**: on `main`
  the same probe already emits `{virtual override thunk}` in exactly this position,
  and `void Account::~Account(void)` as a function header. This change only makes
  the path reachable for three more symbols; it is strictly more information than
  `sub_<addr>`. Qualifying it is a separate follow-up (see below).

## Recompilability

The concern is real but this change does not move it. The C project export was
already not C for a C++ binary, and the **header is bit-identical before and after**:

| artifact | `::` lines | `~ident` lines | `{...}` lines | `gcc -fsyntax-only` errors |
|---|---|---|---|---|
| `fmt_x86_64` (C) `.c` / `.h` — before | 0 / 0 | 0 / 0 | 0 / 0 | 8 |
| `fmt_x86_64` (C) `.c` / `.h` — after | 0 / 0 | 0 / 0 | 0 / 0 | 8 |
| `probe_O0` (C++) `.h` — before | 122 | 22 | 3 | — |
| `probe_O0` (C++) `.h` — after | **122** | **22** | **3** | — |
| `probe_O0` (C++) `.c` — before | 246 | 44 | 6 | 316 |
| `probe_O0` (C++) `.c` — after | 410 | 93 | 6 | 407 |

So: C output is untouched on every axis, and the `::`-in-header problem the survey
flagged is neither caused nor worsened here — the header is produced from the
canonical function names, which this change does not alter. The `.c` bodies gain
`::` at call sites, which is the point of the fix and is what IDA and Ghidra print.
No recompilable artifact becomes unrecompilable: a C++ export already failed to
compile as C in 316 places.

Making the C++ export recompilable is a genuinely separate feature — it must rewrite
identifiers consistently across `.h` *and* `.c` (including the `void Account::~Account(void)`
headers and the `{virtual override thunk}` names that predate this change), which is a
project-export policy decision, not a printer fix. Recorded as a follow-up.

## Follow-ups this did not close

1. **Project-export identifier sanitization** — a `::`/`~`/`{}`-free spelling for the
   `decompile-project` `.c`/`.h` pair, with the readable form kept in the pseudocode
   surfaces. Owner: `decompiler/crates/kuna-console/src/project.rs`
   (`build_header` / `build_c`).
2. **Address-taken function names are unqualified** —
   `Funcdata::link_spacebase_symbol`
   (`decompiler/crates/kuna-decomp/src/substrate/funcdata_varnode.rs`) binds
   `Architecture::name_for_global_varnode`'s *base* name onto the high, where the
   `ActionNameVars` path next to it uses `name_for_global_varnode_scoped` +
   `kuna_qualify_global_name`. Threading the scope path through would render
   `Shape::perimeter` instead of `perimeter`. Pre-existing; 3 lines in this sweep.
3. **The two sibling commit arms** (`out.symbols`, `read_loader_symbols`) still use a
   scope-local probe. Neither was implicated by any witness — the passes that emit
   `out.symbols` supply real names that resolve to the matching scope — so they were
   left alone rather than changed speculatively.
4. **A debug-info name does not displace a weaker loader name in the *Database***.
   Surfaced (not caused) by the `verify_multiformat_dwarf` baseline change: on
   `macho_dwarf.o` the symtab entry `_l0` is installed first, so the DWARF pass's
   `first_byte` fact is skipped by the existing overlap check and lives only in the
   `register_symbol` stream. `kuna functions` therefore reports `first_byte` (the
   canonical-entry ranking prefers it) while `--addr 0x0` reports `_l0` (the
   Database). Both are "a real symbol", so `entry_name_rank`'s preference and the
   Database's first-writer-wins disagree. Pre-existing on both sides of this change;
   deciding which name wins is a naming-policy question, not a lookup fix.
