## What was broken

RE-friction round 2, need `stack-string-invalid-cast` (1 instance, challenge
`68149b8a8f555589f353117c`, Mach-O arm64 `practice_2_rengr`). The tester filed:

> **Stack string initialization is emitted as invalid array casts and scalar assignments**
> It emitted expressions including `v1[0] = (char[8])s_100003f1d._0_8_;` and
> `v5[0] = (char[7])s_100003f1d._9_7_;`; array casts are not valid C and assigning them to
> individual chars misrepresents the multi-byte copies.

Two things are wrong at once. C has no array cast and `._0_9_` is member syntax applied to
an array object, so the export does not recompile; and the four lines spell nothing a reader
can use while `strings` reports `Hello, Reverse Engineer!` at that very address — a string
kuna had *already recovered* and planted a symbol for.

```
$ kuna decompile practice_2_rengr _main          # before
  v1[0] = (char[8])s_100003f1d._0_8_;
  v5 = s_100003f1d[8];
  v6[0] = (char[7])s_100003f1d._9_7_;
  v8._0_9_ = s_100003f1d._16_9_;
```

## Root cause — the filed hypothesis is right about the symptom, wrong about the mechanism

The record guessed "piecewise constant-to-stack stores typed as array-valued casts". They are
not piecewise constants at all.

`RuleStringCopy` (`constseq.rs`, ported from `constseq.cc`) gathers a stack string only from
COPYs whose inputs are p-code **CONSTANTS** — the character-at-a-time form a compiler emits
at `-O0`. A compiler that already has the literal in `.rodata`/`__cstring` emits a **block
copy** instead: one or more wide loads out of read-only memory, re-stored into the frame.
Those loads survive heritage as free **read-only memory varnodes**, never constants, so the
constant-input guard declines and the run falls straight through to the printer as
partial-symbol slices.

So this is a third shape of an idiom kuna already recovers twice (`stringcopy` for constant
stack strings, `memsetcopy` for constant-fill runs), and it had no owner.

## Mechanism

New S5 constsequence rule `RuleRodataStringCopy`
(`decompiler/crates/kuna-decomp/src/p5_types/kuna_rodatastring.rs`), option
**`rodatastring on|off`, default `on`** (DIV-113). It gathers the same-block run and claims
it only when every step is a fact rather than an inference:

* each COPY's source is `Varnode::isReadOnly` free memory — the image bytes *are* the
  run-time bytes;
* all the sources lie inside **one** covering data symbol typed as a char-printable array —
  the symbol the string-literal analysis planted;
* source and destination advance in lockstep, so the run is a straight block copy and not a
  shuffle;
* the COPYs tile the destination **exactly** — no gap, no overlap — across the symbol's whole
  length, so nothing is invented and nothing is dropped;
* the image bytes really are one NUL-terminated string of exactly that length;
* no LOAD/STORE/CALL interferes between members (the same `ArraySequence::interfereBetween`
  window the string driver demands), and a foreign write into the region between the first
  and last member also vetoes the run — the CALLOTHER is inserted at the *first* member and
  would be re-ordered against it.

A single-COPY run is deliberately left alone: the defect is the *split* copy, and a
whole-string single COPY already renders as one assignment.

Build and teardown are the string path's own `constructTypedPointer` + `removeCopyOps`,
reached through new `pub(crate)` entry points `from_rodata_run`/`transform_rodata`. The only
thing the caller supplies is the source byte count, because `RuleStringCopy` derives it as
one character per op and this run moves blocks.

## The acceptance probe now passes

```
$ python -m scripts.repipe.verify --need stack-string-invalid-cast --json
"counts": {"total": 1, "pass": 1, "fail": 0, "closed": 1, "regressed": 0}
"transition": "closed"
```

`a-d7fec4000cb6` wants `Hello, Reverse Engineer!` in `kuna decompile practice_2_rengr _main`:

```
  builtin_strncpy(v1,"Hello, Reverse Engineer!",0x19);
```

The reproduction arm `p-1d6eb3f72061` still pins the filed bug with `--option rodatastring
off`, so both arms are satisfied by the code and neither probe was weakened.

`tests/cli` promotion is refused **by design** — the acceptance target is
`binary_source: dataset` and CI has no dataset, so `make test-cli` correctly stays at 27/27.
The permanent CI pin is the two-pass `tests/stages/kuna-rodatastring.xml` (default recovers
the `builtin_strncpy`; `rodatastring off` restores the filed invalid array-typed cast).

## Evidence for default-on

Both parity corpora are symbol-less bytechunks, so the covering-string-symbol guard can never
fire in them — **their being green is not evidence here** and is reported as a no-regression
check only. The real evidence is a whole-corpus collateral sweep.

`kuna decompile-all --json`, both arms, over every binary in the RE dataset, per-function C
compared verbatim: **135 binaries, 62,131 functions, 4 changed functions (0.0064%)**. 12
binaries fail to load whole-binary in *both* arms alike (pre-existing) and are excluded.
Every changed line was read; all 4 are the intended recovery, and in each the byte count
equals the tiled width and the quoted text is the image content:

| function | slices | becomes |
|---|---|---|
| `crackme0x00::_main` | 8+4+1 | `builtin_strncpy(v2,"NoxIsTheBest",0xd)` |
| `cursed::sub_4311c0` | 7+7+1 | `builtin_strncpy(v11,"/usr/lib/gconv",0xf)` |
| `practice_2_rengr::_main` (witness) | 8+1+7+9 | `builtin_strncpy(v1,"Hello, Reverse Engineer!",0x19)` |
| `crackme1.exe::sub_1400012f0` | 8+8+4 | `builtin_strncpy(v2,"rCC_ZPRGZ] UZ]Z@[VW",0x14)` |

No statement is moved or lost beyond the slices the CALLOTHER replaces, and no call site
changes. The only declaration lines anywhere in the sweep diff are a variable renumbering in
`crackme1.exe` — the destination declarations are otherwise byte-identical between arms.

**Residual, deliberately out of scope and stated plainly:** where the frame carving split the
buffer, the slices the run used to write survive as *unread* declarations (`char v5;`,
`char v6[7]`, `char v8[24]` beside `v1` in the witness). That is the local variable map still
seeing the frame the original wide stores carved — a P6 concern, present identically with the
option off, and not something a one-module S5 rule should reach into.

**Speed:** `-21.37%` on the witness (`_main`, 9 repeats, off 140.49 ms / on 110.47 ms). Read
as *no measurable cost*, not as a speedup: the magnitude is inside this machine's known
single-target noise floor (`docs/features/returncopysplit/record.json` measured `-20.23%` and
`-12.18%` on byte-identical output). The sign is favourable, so there is no budget question.

## Gates

| Gate | Result |
|---|---|
| `make test` | **PARITY OK — 675/675** |
| `make test-stages` | **PARITY OK — 615/615** (613 → 615, the two new `rodatastring` assertions) |
| `make rust-test` | **green** |
| `make check-spec` | **OK** |
| `make test-cli` | **27/27** |
| `kuna catalog --check` | **catalog OK** |

One note on `make rust-test`, since CI skips it on an internal-branch PR. Adding a rule +
option trips five hard-coded count/order asserts, all bumped here: `surface_count` 107→108,
`option_values` live-value present 46→47, `verify_w8_fw_universalaction` leaves 275→276, and
the `list_action_decompile_oracle.txt` fixture (new `rodatastringcopy` leaf, renumbered) which
backs both `universalaction_listing` and `verify_w8x_allowlist`. Separately,
`verify_w10_proto_unlock::w10_proto_unlock_const_return_collapses_no_tied_roundtrip` fails in
a repipe worktree for an **environment** reason unrelated to this change: `cpp_oracle_bin()`
reads `KUNA_DECOMP_TEST`, which the worker sets to kuna's own release `decomp_test_dbg`, so
the "C++ oracle" arm compares kuna against itself and kuna's DIV-6 realtypes rendering
(`unsigned int`) trips the fixture's `xunknown4` assert. Green with the variable unset, which
is how CI runs it.

## Also in this PR

`docs/spec/05-types.md` (the owning chapter) gains the prose for the pass and its guards;
`docs/options.md` is regenerated; `docs/history.md` gains the DIV-113 row.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
