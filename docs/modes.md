# Decompiler modes (`auto` | `reliable` | `aggressive` | `fast`)

A **mode** is a named preset over kuna's runtime option surface: an ordered list
of `(option, value)` overrides layered on top of the shipped defaults. Modes are
**not** catalog options (`kuna catalog` covers the per-option settable surface);
they reference existing option names and live in one table,
`decompiler/crates/kuna-decomp/src/p0_knowledge/modes.rs` (`MODE_TABLE`), applied
by `Architecture::apply_mode` (a fan-out over `set_kuna_option`).

Select a mode with `--mode <name>` on `kuna decompile`, `kuna decompile-all`,
`kuna decompile-project`, or `kuna functions`. If the flag is omitted, these
file-based front-ends use `auto`. The console `mode <name>` command accepts the
three concrete presets; `auto` must be resolved by a front-end that knows the
input file size. A mode's overrides are applied **before** the user's
`--option`/`option` lines, so an explicit `--option` always wins (last-write).
Modes are override batches, not resets: applying a second mode in one console
session changes only the options that second mode names. Discover them with
`kuna modes` (or `kuna modes --json`).

| Mode | What it does |
|---|---|
| `auto` | The default file-front-end policy: `aggressive` below 500 KiB, `reliable` from 500 KiB up to 2 MiB, and `fast` at 2 MiB or larger, with `ctypes` and `partialaccess` on at every size. |
| `reliable` | The shipped, well-tested defaults — the safe, stable baseline. An **empty** override list. |
| `aggressive` | Maximum recovery: turns **on** every off-by-default quality, structuring, and analysis pass except `v850indirectbranch` and `dwarf_lines`. Slower and more speculative (may over-recover); best for readability and for measuring the recovery ceiling on the benchmark, not for guaranteed faithfulness. |
| `fast` | Latency first: disables the exhaustive Listing consumers, prologue-pattern scan, and AIF gap walk, then substitutes rooted direct-call discovery and conservative pointer-table validation. Keeps the shipped per-function transforms and explicit selectors, but may still discover fewer functions and recover fewer program-wide facts than `reliable`. Spells core types and partial accesses as C (`ctypes`, `partialaccess`). |

## `auto` (default)

`auto` selects a concrete preset from the raw input file length:

| File size | Selected preset |
|---:|---|
| `< 512,000` bytes (500 KiB) | `aggressive` |
| `512,000` through `2,097,151` bytes | `reliable` |
| `>= 2,097,152` bytes (2 MiB) | `fast` |

The boundary comparisons are exact: a 500 KiB file is `reliable`, and a 2 MiB
file is `fast`. “Standard mode” in the size policy means the existing
`reliable` preset. The native CLI reads filesystem metadata; the browser/WASI
front-end classifies the uploaded virtual file's byte length through the same
Rust function. An explicit concrete `--mode` bypasses size selection, and a
later explicit `--option` still overrides the selected preset.

`auto` also carries two overrides of its own, `ctypes=on` and
`partialaccess=on`, applied before the selected preset's list (`AUTO_OVERRIDES`, resolved together with the preset by
`resolve_overrides_for_size`). The size policy trades discovery and recovery
depth for latency; how the emitted C spells a type is not part of that trade,
and `ctypes` costs nothing. Without it a binary of 500 KiB or more printed
`int4`, `uint8` and `float8` where a smaller one printed `int`,
`unsigned long` and `double`. An 8-byte element cast such as `((int8 *)a0)[4]`
reads like a byte offset of 4 when the offset is 0x20 (GH-1063). `kuna modes`
`partialaccess` is the same kind of choice: it prints a partial access of a
variable as a typed pointer access (`((short *)&v1)[2]`) instead of Ghidra's
`v1._4_2_`, which is not C. `kuna modes` lists both overrides under `auto`. An
explicit `--mode reliable` does not get them:
that preset stays the catalog defaults, which is also what the datatest corpus
runs.

This makes small binaries favor maximum recovery while bounding the work a
browser or bulk export attempts for large inputs. On private PE
`bc4c15d826aaebeace3fec6360eb687e5662cba8745605093254931dcdb3ae1b`
(3,457,296 bytes), `auto` resolves to `fast`. The original 3.23-second result was
not a valid successful export: its 351 executable targets were 350 import
veneers plus the PE entry, so the project omitted ordinary internal bodies such
as `0x402d80` even though an explicit decompile at that address produced code.
The rooted discovery correction below preserves the latency-first policy while
making the whole-project inventory useful. With a 10-second per-function
watchdog, now the default for unfiltered fast whole-binary runs, a no-flag
confirmation completed in 445.06 seconds at 1,476 MiB peak RSS: 3,140 real C
bodies, 13 isolated failures, and 3,153 definitions total.

## `reliable`

The current defaults are already a well-tuned, net-positive-on-benchmark set
(many angr structuring flags are default-on; see `docs/history.md`).
`reliable` is a stable, named alias for that set — its override list is
deliberately empty. `auto`'s 500 KiB–2 MiB tier selects it with `auto`'s own
`ctypes=on` and `partialaccess=on` in front; an explicit `--mode reliable` is
the catalog defaults exactly, Ghidra's `int4`/`float8` spellings and `._4_2_`
members included. Pinning options here (e.g. `listing off`) would *change*
behaviour versus the surface defaults (`decompile` and `decompile-all`
auto-enable the Listing), so the empty list is the faithful implementation. It
future-proofs the preset: if the defaults later drift more aggressive,
`reliable` can pin them back.

## `aggressive`

`aggressive` enables binary default-OFF options, subject to the exclusions
listed below. Multi-valued options fall outside this automatic binary-option
inclusion rule. `AGGRESSIVE_OVERRIDES` in `p0_knowledge/modes.rs` is the authority — the
invariant test `aggressive_carries_every_default_off_option` makes an omission a
build failure. The options it enables:

- **transform tier**: `switchmodbound`, `constselectjump`,
  `switchguardbound` (speed-costly),
  `unrolledguard` (speed-costly), `sparcstructret`,
  `globalarrayanchor` (retain mapped array bases in negatively biased element indices),
  `regionedgeorder`, `returndup`, `orchain`, `iteexpr`,
  `devectorize` (a proven vectorized integer reduction re-rolled into its scalar
  loop; off by default only because the rewrite hides the stride and the
  duplicated body), `ctypes`, `partialaccess` (a partial access prints as a
  typed pointer access instead of `name._<off>_<size>_`), `stackstruct` (an
  address-taken frame record read at several widths is one synthesized
  structure),
  `calltargettype` (an indirect call's target takes the function-pointer type
  the call states, so the call compiles as C),
  `libcnames` (a local takes the name of the library call it meets:
  `fd = open(a0,0);`, `n = read(fd,v1,0x200);`),
  `armfloatreturn` and `armfloatargs` (an ARM hard-float function returns its
  double whole in `d0` and takes its float and double parameters in their
  `s` and `d` registers on both sides of a call; inert unless the ELF states
  the VFP procedure-call standard, so soft-float and softfp images print
  byte-identically),
  `cortexmpriv` (DIV-99 - fold the Cortex-M `isCurrentModePrivileged()` guard;
  ARM-Cortex-M-gated by construction, so it is a measured no-op everywhere else)
- **analysis tier**: `listing` (the master gate that enables the
  Listing-consuming passes — `fid`, `aif`, the discovered-no-return family),
  `fast_funcdisc`, `eh_frame_full`, `funcstart_patterns`, `addrtable`,
  `operand_refs`, `widestrings32` (4-byte `wchar_t`/`char32_t` literals),
  `fid`, `rtti`, `itaniumrtti`, `aif`, `aifstrict`, `objc`, `pdb`,
  `macho-arm64e`
- **analysis tier, ARM entry discovery** (DIV-93): `cortexmvectors`, `ptrentry`,
  `tailcallentry`, `poolentry` — the four-step sequence that takes stripped ARM
  entry recall from 88.63% to 93.31% over the 110 non-x86-64 decbench twins while
  *lowering* mid-body false entries. All four are ARM-gated and the last three
  also need `listing`, which this preset supplies; off ARM they are measured
  no-ops (identical entry sets on 90 x86-64 twins and 12 i386 PE images).

`returndup` (DIV-54) and `pdb` (DIV-129) became shipped defaults and are
deliberately kept on the list: the preset **owns** the option, so an earlier
`--option returndup off` on the same command line is re-enabled by
`--mode aggressive`, and the preset's contract ("every recovery pass on") does not
silently change if a default drifts back.

### The deliberate exclusions

Most off-by-default options are safe to blanket-enable: the format-specific ones
are inert off their target (`rtti` = PE, `objc`/`macho-arm64e` = Mach-O,
`sparcstructret` = the SPARC `unimp`-trap idiom, which cannot occur off SPARC,
and since DIV-93 the four ARM entry passes, which are inert off ARM). These are
not, and the invariant test's `EXCLUDED_ON_PURPOSE` list is the authority:

- **`x86listingswitch`** — bounded i386 table coverage can expose tail jumps
  into independent functions before AIF discovers their entries. The measured
  Wine corpus lost seven real starts; keep this discovery tradeoff explicit
  with `--option x86listingswitch on` in every mode.

- **`v850indirectbranch`** — its predicate (`kuna_is_v850_indirect_jmp`,
  `p2_lift/kuna_v850indbranch.rs`) matches **any** register-indirect `CALLIND`,
  so on x86-64/ARM it would reclassify every `call reg` into an indirect branch
  — corruption, not recovery.
- **`dwarf_lines`** — it recovers nothing. It attaches every instruction's
  `.debug_line` `file:line` as a comment, and those survive into the emitted C,
  so on any `-g` binary the body renders interleaved with `/* src.c:NNN */`
  lines (often several per statement, since one C statement spans many
  instructions). Because `auto` selects `aggressive` under 500 KiB, including it
  here made that annotated form the *default* rendering for small debug
  binaries, which is a readability regression, not a recovery win (DIV-43).

- **`formatstring`** — it recovers real signal (printf/scanf varargs typed from
  the format string, and, through the read-only propagation it needs, every
  literal-pool pointer on ARM), but it buys that with a **second full decompile**
  of any caller whose call sites yield an override. Only ~5-15% of a binary's
  functions re-decompile, but they are the expensive ones, so whole-binary
  `decompile-all` measured **+77.5%** (cronie `crontab`), **+55.1%** (gnutls
  `psktool`) and **+43.7%** (`gzip`) — an order of magnitude over the 5% speed
  budget, which standing requirement 4 answers with a per-run opt-in. Until DIV-66
  the preset entry was *also* inert on every whole-binary surface, so it was
  charging `kuna decompile` for a feature `decompile-all` never delivered; now
  both surfaces honour `--option formatstring on` identically, and neither pays
  for it unless asked (DIV-66).

- **`ifuncfpret`** — Stage A of the IFUNC FP-return chain. On its own it only
  renames x86-64 IFUNC stubs to synthetic `ifunc_<resolver>` names and recovers a
  tail call to them; it does not yet fix the `xmm0` return the feature exists for,
  so it stays an opt-in until the chain is complete.

- **`msvcstrappend`** — it collapses an inlined MSVC `std::string` append into one
  `std::string::push_back`/`append` call, deleting the fast arm's stores and naming
  the grow call on the strength of its argument shape; the callee body is never
  read. That trust is the operator's call to make about a binary, not a default.

- **`bejoin`** — its default, `evidence`, joins a big-endian or AVR
  two-register value high word first only on proof the value is that wide, in
  every mode. `on` joins it that way whenever its low word looks returned on
  purpose: the rule reads the instructions, not a type, and an `int` that works
  over the high half of a 64-bit temporary compiles to the same registers as the
  `long long`, so with it on that correct `int` prints as a wrong `long long`
  (GH-904). In the preset it would be the default output under 500 KiB.

- **`regsigned`** — it declares a register local whose every value is a sign
  extension at the narrow width it is extended from (`short v` instead of
  `int v` with a cast on every load). On decbench type_match it gained a few
  perfect functions on ARM firmware and none on x86-64, and a narrowed value
  that a wider reader takes back gains a cast (`(long)(int)v` on AArch64), so
  the preset would make a near-neutral judgment call the default output under
  500 KiB.

These therefore stay manual per-run opt-ins (`--option v850indirectbranch on`,
`--option dwarf_lines on`, `--option formatstring full`, `--option ifuncfpret on`,
`--option msvcstrappend on`, `--option bejoin on`, `--option regsigned on`)
even under `--mode aggressive`; a named `--option` still wins over the preset by
last-write precedence. For `formatstring` only the `full` loop above is excluded:
its `static` default reads the same format constants out of the image at load, so
there is no second decompile to pay for and every mode that builds a Listing
carries it.

`argclobber` used to be on this list, for the same one-sided-evidence reason: the
register it drops is a possible *output* location of the callee's model, so a
wrong drop is a deleted expression, which is the kind of wrong output a reader
cannot see. It now ships **default-on**, because the drop is declined unless the
callee's own recovered prototype says the register is not read — a callee that
really returns a 16-byte value in `rax:rdx` and forwards the high half into the
next call keeps its argument. Flip `--option argclobber off` to keep every
argument the register model produced, phantom or not.

Separately, these default-off options remain unevaluated for preset membership:
`guardarm`, `loopcondhoist`, `paramcopyhoist`, `switchselector`, `msvcstackguard`,
and `stackaddrargtrial`. Each needs the sweep and speed evidence that would
justify a preset change. The stack-address option has focused tests and a local
speed measurement, but no broad sweep; the MSVC option still needs a corpus with
real `/GS` cookies. The `UNEVALUATED` list beside the invariant test records this
follow-up. DIV-93 previously removed the four ARM entry options from that list.

### Caveats

`aggressive` is slower (the Listing build, `funcstart_patterns` discovery) and can
over-recover: `aif` is a speculative gap-walk ("may create bad code") and
`addrtable` over-accepts pointer tables. It no longer carries `formatstring`'s
re-decompile loop: that loop used to run only on `kuna decompile`, where it cost
up to +77% and on the whole-binary surfaces delivered nothing at all — DIV-66
made both surfaces honour the option and, at that measured price, made it an
opt-in on both.
That is by design — `aggressive` is the recovery-ceiling / measurement envelope,
not the faithful default. Use it to read maximally-recovered output or to A/B
which options net-help; promote an option to a default only when its own ablation
shows net-positive-zero-regression.

`returndup` used to be listed here as a known aggregate-GED regression (DIV-18).
That is no longer true: since #137 gave it angr's const-return gate, an isolating
ablation over the 52,862-function optimized decbench corpus puts it at **+417
GED-perfect functions and −7,756 aggregate GED**. **DIV-54 promoted it to a
shipped default**, so it is no longer an `aggressive`-only pass at all — the
preset merely keeps naming it. See
`docs/decbench/returndup-regression-triage.md`.

## `fast`

`fast` applies exactly six overrides:

```text
listing=off
funcstart_patterns=off
aif=off
fast_funcdisc=on
ctypes=on
partialaccess=on
```

`ctypes` and `partialaccess` are the printer's C spelling of the core types and
of a partial access and cost nothing, so a speed preset has no reason to print
`int8` or `v1._4_2_` where `aggressive` prints C.
The first three disable the exhaustive program-wide decode and speculative
discovery paths that dominate large-binary latency. Naming `listing` suppresses
its default injection on all three decompile surfaces; naming
`funcstart_patterns` and `aif` suppresses their non-x86-64 injections on
`decompile-all` and `decompile-project`. `funcstart_patterns` is independently
important: on private PE
`bc4c15d826aaebeace3fec6360eb687e5662cba8745605093254931dcdb3ae1b`,
enabling it with the other two disabled expands the inventory from 693 to 4,452
entries.

`fast_funcdisc` performs one recursive SLEIGH Listing walk from loader-backed
entries and promotes every statically reached direct-call target. It then scans
absolute code-pointer tables and admits an indirect-only target only after the
established AIF fingerprint and valid-subroutine checks corroborate it. On
non-ARM objects, tables above 256 slots are ignored; large candidate sets
require two independent table references and validate at most 4096 targets.
ARM uses the established Thumb-pointer prologue oracle. The pointer-derived
roots are not recursively expanded. This supplies real project bodies without
the full prologue corpus or byte-by-byte undefined-gap walk.

The tradeoff remains deliberate. With `listing=off`, ordinary Listing consumers
such as discovered no-return propagation and FID do not run. With full
function-start patterns and AIF disabled, functions that are neither directly
reachable nor represented by an accepted pointer table can remain absent.
Loader symbols, architecture context, import naming, explicit function/address
selection, and all shipped per-function transforms remain active.

The broken metadata-only fast policy completed quickly precisely because it did
not discover the private PE's internal program. The corrected `--mode fast`
export's unfiltered watchdog defaults to 10 seconds per function. The private
export completed in 445.06 seconds at 1,476 MiB peak RSS, producing 3,140 bodies
and 13 isolated failures without an explicit watchdog flag; the non-fast
control remained incomplete after 935.91 seconds. Native explicit name/address
selections retain the 120-second default, and `--max-fn-seconds` always wins
(including `0` to disable). The WASM front-end arms the 10-second budget only
for fast whole-binary decompile/project commands. Public regressions pin both
halves of the discovery replacement: `pdb_prog.exe` contributes a direct
internal callee, and `aif_gap_x86_64` contributes an indirect-only
pointer-table target, with real C bodies in both cases.

Explicit `--addr` commands already have their target set, so the file
front-ends suppress preset-provided `fast_funcdisc` work for those commands.
Text-surface `kuna decompile <name>` suppresses it on its first attempt as
well and restores it only when the name misses; a generated placeholder name
(`sub_<addr>`) keeps discovery enabled, because that entry may exist only
because discovery registered it. Callers can opt back in explicitly with
`--option fast_funcdisc on`; an explicit option always wins.

An explicit option selectively restores analysis while keeping the rest of the
preset:

```bash
kuna decompile-project ./a.out --mode fast --option listing on
kuna decompile-project ./a.out --mode fast --option fast_funcdisc off
```

Because explicit options are applied after the mode, the first command restores
ordinary Listing consumers while leaving prologue patterns and AIF disabled;
the second restores the old metadata-only fast inventory for measurement or an
especially tight latency budget.

## Measuring with modes

`aggressive` and `fast` are first-class measurement levers. Run the decbench GED
benchmark with a `kuna-aggressive` backend (or `kuna decompile-all --mode
aggressive`) and compare against an explicit `--mode reliable` baseline to find
options that net-help, which then become candidate default-on flips. The omitted-mode
`auto` policy intentionally changes
with input size and is therefore not a stable benchmark baseline.
Use `fast` for latency-sensitive bulk export and compare its function inventory
and output against `reliable` before adopting the coverage tradeoff.

`stackalias` and `stackviews` require an explicit per-function choice. Physical
frame-write preservation can change object layout and declaration types; neither
recovery policy is promoted by `aggressive` while that integration is incomplete.
