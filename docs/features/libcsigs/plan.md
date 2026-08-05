# `libcsigs` — plan

## Mechanism chosen, and why not the other two

The brief offered three shapes:

**(a) Extend the hand-written table with the highest-frequency missing names.**
Chosen. It is one PR, it ships the whole measured head, and the risk per entry is
uniform and bounded.

**(b) Generate the table from a machine-readable source at build time.** Rejected
for this PR, deliberately. Making the build read `/usr/include` would make kuna's
output depend on the *build host's* libc — a kuna built on musl or on a glibc with
a different `_FORTIFY_SOURCE` default would emit different C for the same binary,
and CI would stop being reproducible. It also cannot work at all for the three
groups that have no header prototype (`__stack_chk_fail`, the `*_chk` builtins,
the `__isoc99_*` redirects), which are 45,000 corpus call sites between them.
Headers are used here as an **offline** source of truth for a checked-in table,
which keeps (b)'s rigour without (b)'s build coupling. A real header-derived type
*database* — one that also carries struct/enum layouts, so `struct stat *` stops
being `void *` — is a genuinely larger piece of work and stays a separate item.

**(c) Something else.** The one novel piece here is not the table but the
**imports-only** application rule (analysis.md §4), which is what makes 206 entries
safe where 27 were safe by luck.

## Shape

- New module `analyzers/protos/kuna_libcsigs.rs`: the 206-entry `LIBC_EXT` table,
  `imported_function_names`, and `LibcSigsPass` (`AnalysisPass`, P1, id `libcsigs`).
  Registered in `passes.rs` immediately after `LibProtoPass` and before the DWARF
  pass, so real debug info still wins.
- `Ty` gains four width-stable variants — `UInt`, `Long`, `IntPtr`, `CharPtrPtr` —
  and `build_ty` the matching arms. The base table is otherwise untouched.
- Gate: `Architecture::analysis_libcsigs`, default ON, read at the analysis commit
  boundary via `engine.rs::analysis_pass_enabled`, printed by
  `kuna_console::kuna_live_value`. No `live_field` in `phases.toml`, matching
  `libproto` and every other analysis-pass gate.

## Preset membership: deliberately NOT in `AGGRESSIVE_OVERRIDES`

`aggressive_carries_every_default_off_option` (added by #267) enforces that every
**default-off** on/off option is either in the preset or on a named exclusion list,
because `auto` resolves to `aggressive` under 500 KiB and an option that is
default-off and absent from the preset ships inert on the default path — how
`itaniumrtti` recovered 0 of 29 until it was caught. `libcsigs` ships **ON**, so it
is out of that test's scope by construction, but the question still deserves an
answer on evidence rather than on the test passing.

**Measured** on `dircolors` (55,680 bytes, so `auto` → `aggressive`), default vs
`--option libcsigs off`, whole-binary `decompile-all`:

| mode | functions differing | cast-removal witness |
|---|---:|---|
| (no `--mode`, i.e. `auto`) | 57 of 184 | yes |
| `--mode aggressive` | 57 of 184 | yes |
| `--mode reliable` | 58 of 183 | yes |
| `--mode fast` | 58 of 184 | yes |

So the feature is already live on **every** mode and every driver surface, and the
inertness failure mode cannot occur: no preset turns anything off except `fast`'s
three discovery gates, and an option absent from a preset keeps its default.

Adding `("libcsigs", "on")` would be worse than redundant. `AGGRESSIVE_OVERRIDES`'s
own doc records that the preset **owns** an option outright — "a user
`--option returndup off` earlier on the command line is re-enabled by the preset".
Every benchmark and sweep number in this PR was produced by `--option libcsigs off`
on binaries under 500 KiB, i.e. straight through the `auto` → `aggressive` path.
Putting the option in the preset would silently re-enable it there and destroy the
ablation hook that is half the reason the flag exists.

Because the option is not joining the preset, this PR needs no second DIV row for
preset membership (which is what #267's DIV-64 exists for); DIV-65 covers the
default-ON decision alone.

## Testing

`kuna-console/tests/verify_libcsigs.rs`, five two-pass assertions over the existing
vendored `fauxware` fixture. **No `tests/stages/*.xml` case**: the XML datatest
bootstrap builds its program from a `<binaryimage>` element and never runs the
analyzer tier, so no `kuna_analysis` pass can fire there — the same reason
`verify_cppsig.rs`, `verify_cppproto.rs` and `verify_fdeinterior.rs` exist. Both
gate settings are asserted in every test.

Five unit tests in the module pin the table's invariants, including the FORTIFY
argument slots and the imports-only rule.

## Follow-ups, deliberately not in this PR

1. **`VARIADIC_SUBSTRINGS`.** `formatstring/apply.rs` matches by `contains` over
   `{"printf", "scanf"}`, so `__printf_chk` is treated as a format-string callee
   with the format at slot 0 — it is at slot 1. Adding the `error`/`err`/`warn`/
   `syslog` family there is a second, independent behaviour change and belongs in
   its own PR with its own measurement.
2. **A zero-parameter prototype does not assert "no arguments" everywhere.**
   `FuncProto::update_all_types` clears `VOIDINPUTLOCK` and `set_pieces` re-locks
   it, so the path used for a callee rebuilt from parked pieces is correct — but
   `gzip -O2` `sub_f900` shows `abort(v24[-8])` where the base table's `free` and
   `memcpy` *already* show the same phantom trailing stack argument with this
   option OFF. That is a pre-existing frame-recovery defect in one function, not
   something this table introduces, and it wants its own investigation.
3. **A real header-derived type database** (struct/enum layouts travelling with a
   signature), which is what would close the remaining `void *` in `stat(path, sb)`.
