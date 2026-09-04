RE-friction round 2 need `strings-json-fails-report` (major, 1 instance, `scope: large`).

## What was broken

> **strings JSON fails to report the owning function for a directly referenced prompt**
> `kuna strings --json --filter Password` found the string at 0x8049127 but returned
> `xrefs_count: 0` and `functions: []`, despite code directly forming its address for output.

The symptom is not one string and not the `strings` surface. On the witness
(crackmes.one `5ab77f6333c5d40ad448ca52`, *pancrackme v1.0*, i386 PIC ELF, stripped)
**all 23** strings reported `xrefs_count: 0`, while `kuna xrefs --to <function>` answered
normally on the same binary.

The cross-reference query reads a reference out of **one instruction's p-code**: a
decode-time constant that lands in a mapped section. That is the whole answer on x86-64,
where `lea rax,[rip+0x36a]` lifts to a COPY of the absolute address. It is no answer in
32-bit position-independent code:

```
80489ef:  e8 00 00 00 00    call  0x80489f4      ; push the next instruction's address
80489f4:  5b                pop   ebx            ; ebx = 0x80489f4
80489f5:  81 c3 4c 18 00 00 add   ebx,0x184c     ; ebx = 0x804a240 = _GLOBAL_OFFSET_TABLE_
   ...
8048ad3:  8d 83 e7 ee ff ff lea   eax,[ebx-0x1119]   ; eax = 0x8049127 = "Password: "
```

The captain's T_REFUTE established the hard part: the literal `0x8049127` occurs in **no**
4-byte little-endian position in the image, so no address-matching scheme can ever produce
this xref, and any heuristic that attributed the string to a *nearby* function would produce
wrong ownership with no gate in this repo able to see it.

## The mechanism — `option picbase` (analysis tier, default **on**, DIV-104)

New `decompiler/crates/kuna-analysis/src/listing/kuna_picbase.rs`, consumed by a gated seam
in the existing read-only query (`listing/xrefs.rs (build)`).

A deliberately tiny abstract machine over the same whole p-code the query already keeps: a
value is a constant or an offset from the stack pointer, memory is modelled only at stack
offsets (enough to follow the `call`'s push into the `pop`), and a constant is tainted
PC-derived only when it equals **its own instruction's fall-through** — so a plain
`mov ebx,0x804a240` cannot establish a base. GCC's out-of-line `__x86.get_pc_thunk.bx` is
covered by the same machine, via a bounded two-instruction callee probe (the shape
`veneer_at` already uses for forwarding veneers).

Three shapes are read off each instruction **independently**, base seeded and nothing else
assumed, so no state ever crosses a control-flow edge: a `LOAD` address (read), a `STORE`
address (write), and a constant landing in a **register** (address-taken). A value computed
only into a temporary is deliberately not reported — in `mov al,[ebx+edx+0x87]` the array
base lands in one, and filing it would claim a reference the instruction never forms.

**Two claims, licensed differently.** A function that runs the idiom itself computes the
value and assumes nothing. A function that only *uses* an inherited base is relying on the
i386 SysV ABI reserving that register as the module's GOT pointer, so the recovered value is
cross-checked against the image's own `_GLOBAL_OFFSET_TABLE_` (`.got.plt`/`.got`) and every
idiom in the program must agree on one register and one value.

That module-wide half is **load-bearing, not decorative**, and it is the one thing the need
record did not anticipate: kuna's own function inventory splits the witness's prompt routine
into four entries at its `int3` traps, so the idiom that sets `ebx` up is in `sub_80489e6`
while the `lea` that forms the prompt is in `sub_8048ace`. A purely function-local
propagation does not close this need.

## Refusing wrong ownership, structurally

The base is offered to a function whose body **never writes** the register, or from its own
establishment **up to the next write of it** (in GCC output, the epilogue's
`mov ebx,[ebp-0x4]` restore), and to no other function. A body that reuses the register for
its own purposes contributes no references rather than wrong ones.

The `picbase_i386` fixture carries that as a standing negative: `clobbers` forms
`lea secret@GOTOFF(%ebx)` off a register it loaded with `0x11111111`, and `secret` must stay
referenced by nothing. A separate whole-address-range cargo assertion pins that the option
**only ever adds** edges — it never drops or re-points one the constant scan already found.

## The acceptance probe now passes

```
$ python -m scripts.repipe.verify --need strings-json-fails-report --json
strings-json-fails-report   passed: true   flaky: false
```

Witness output, every row checked against `objdump`:

| string | before | after |
|---|---|---|
| `0x8049127 "Password: "` | 0, `[]` | **1, `sub_8048ace`** |
| `0x8049132 "Use ^D next time ;)"` | 0, `[]` | **1, `sub_8048c4a`** |
| `0x8049147 "/proc/%d/cmdline"` | 0, `[]` | **1, `sub_8048df9`** |
| `0x8049114 "[pancrackme] v1.0"` | 0, `[]` | 0, `[]` — no instruction statically forms it (the crackme rewrites its own code); 0 is the right answer |

Promoted verbatim into `tests/cli/strings-json-fails-report.json`, re-pointed at the vendored
`picbase_i386` fixture because CI has no dataset. The extra owning-function clause is
load-bearing: asserting only that *some* reference exists would be satisfied by exactly the
nearby-address heuristic this option is built to refuse. `make test-cli` 15/15.

## Corpus sweep (standing requirement 7)

**Breadth — 460 binaries A/B'd** (`kuna strings --json` with the option on vs off): 260 crackmes
from the RE dataset (PE32/PE32+/ELF i386/x86-64) plus a 200-binary random sample of decbench
ELFs across x86-64 / ARM / Cortex-M at O0 and O2.

* **443 byte-identical.** Every non-i386-PIC target takes the pre-feature path unchanged —
  detection declines on any image with no `.got`/`.got.plt` and on any code that never runs
  the idiom.
* **1 changed: the witness**, with exactly the three expected attributions and nothing else.
* 16 could not be measured: kuna cannot load them **on either arm** (DOS `MZ`, IA-64, a
  format it does not recognise, one pre-existing timeout). These are unrelated loader gaps,
  identical on both sides.

**Depth — every changed edge classified.** A whole-binary edge dump on the witness
(`kuna xrefs --from` over all 49 functions, both arms):

```
edges off=109  on=151   added=42   removed=0
```

**Zero edges removed or re-pointed.** All 42 added edges were then checked *mechanically*:
each one is exactly `_GLOBAL_OFFSET_TABLE_ (0x804a240) + the referencing instruction's own
displacement` — **42/42**. They are the program's real GOT-relative traffic: the four
`.rodata` prompts, the `.data`/`.bss` globals the anti-debug logic reads and writes
(`MOV EAX,[EBX + 0xa8]` → `0x804a2e8`), the `.ctors` pointer
`__do_global_ctors_aux` walks (`LEA EAX,[EBX - 0xe4]` → `0x804a15c`), the two `signal()`
handler addresses (`LEA EAX,[EBX - 0x1865]` → `0x80489db`), and the routine's own address
that its `cmp [ebp+8],eax; int3` self-check compares against
(`LEA EAX,[EBX - 0x185a]` → `0x80489e6`). A read-modify-write
(`SUB byte ptr [EBX + 0xe2],0x4`) correctly files one `read` and one `write` at the same
address.

**One honest limit on this sweep**: i386 PIC is thinly represented in both corpora, so
"1 of 460 changed" measures inertness broadly and the *positive* direction on one binary.
The other positive coverage is the `picbase_i386` fixture, which is why it carries all four
cases (self-established, inherited, `get_pc_thunk`-delivered, clobbered) rather than only the
one the witness needed.

## Gates

All re-run on the rebase onto `cb357826`.

| gate | result |
|---|---|
| `make test` | **PARITY OK**, 675/675 — `docs/baseline.json` byte-identical to `origin/main` |
| `make test-stages` | **PARITY OK**, 600/600 (597 -> 600 with the new `tests/stages/kuna-picbase.xml`) |
| `make rust-test` | green — **5297 passed, 0 failed** (CI skips the workspace suite on an internal-branch PR, so this was run locally on the rebased tree; `full-ci` is set so it also runs here) |
| `make check-spec` | OK |
| `kuna catalog --check` | catalog OK |
| `make test-cli` | **15/15**, including the newly promoted `tests/cli/strings-json-fails-report.json` (`main` is 14/14) |
| `verify --need strings-json-fails-report` | **acceptance PASS** |
| `scripts.repipe.counters` | no drift: 135 settables, tiers (31, 54, 50), 230 corpus files |

Unit tests: 9 new in `kuna_picbase` (the folding, the read/write kinds, the temporary that is
*not* reported, the partial-write invalidation, the address floor, the return-address
exclusion, the stack round-trip, the untainted-constant refusal, sign extension) and 5
end-to-end in `verify_picbase.rs`. One further hidden count test needed updating:
`option_values_live_value_present_for_41_suppressed_for_90` -> `_for_91`, since `picbase` is
read console-side via `kuna_live_value` (like `unmappedentry`) and has no codegen
`live_value`.

## Notes for the reviewer

* **Query surface only.** `kuna xrefs`, `kuna strings` and `decompile-all`'s xref section are
  the three readers of this index; it is a read-only query that commits nothing, and no
  p-code, prototype or emitted C depends on it. That is *why* `docs/baseline.json` is
  untouched — structurally, not incidentally.
* **`mergecheck` shape C reports 11 rejects on `phases.toml`, all expected.** Every one is a
  scalar field of the single new `[[settable]]` block (`values = "on|off"`, `tier = "analysis"`, …)
  whose line count rose by exactly one; the multiset check cannot tell "one new record" from
  "a duplicated row" in an array-of-tables. Nothing was removed, and `[[settable]]` went
  134 → 135, matching the re-derived counter. The guard was read, not silenced.
* Two defects in the out-of-line-thunk arm were found only by the fixture and neither was
  visible on the witness (which uses the inline idiom): a `CALL` was clearing the stack
  pointer, which a callee's `ret` restores, and `holder_of` scanned every address space, so
  the unique temporary SLEIGH loads through outranked the register it copies into.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
