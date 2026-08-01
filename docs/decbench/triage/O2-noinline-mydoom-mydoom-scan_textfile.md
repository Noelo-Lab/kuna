---
case_id: O2-noinline-mydoom-mydoom-scan_textfile
pool: novel
status: feature-candidate
tier: N
margin: 0
fresh_verdict: today's build still emits `PTRSUB(ESP,<off>)` 8x in the body — the raw p-code operator plus an undeclared register identifier; control flow is already best-in-class (GED 16, ties ghidra/ida, beats angr/binja/phoenix 18)
option_closing: null
feature_slug: spacebase-unnamed-location
scope: small
confidence: high
---

## Side-by-side

Ghidra renders the same eight spacebase references as named stack locations:

```c
  uStack_14 = 0x405afe;
  uVar2 = FUN_004098d0();
  iVar8 = -uVar2;
  *(undefined4 *)(&stack0x00000008 + iVar8) = 0;
  *(undefined4 *)(&stack0x00000004 + iVar8) = 0x80;
  *(undefined4 *)(&stack0x00000000 + iVar8) = 3;
  *(undefined4 *)(&stack0xfffffffc + iVar8) = 0;
  *(undefined4 *)(&stack0xfffffff8 + iVar8) = 3;
  *(undefined4 *)(&stack0xfffffff4 + iVar8) = 0x80000000;
  *(undefined4 *)(&stack0xfffffff0 + iVar8) = *(undefined4 *)(&stack0x00010040 + iVar8);
```

IDA recovers the actual buffer and the call, so the stores disappear entirely:

```c
  v1 = alloca(((int (*)())sub_4098D0)());
  FileA = (char *)CreateFileA(dwFlagsAndAttributes, 0x80000000, 3u, 0, 3u, 0x80u, 0);
```

kuna (stored 2026-07-27) and kuna (fresh, today, identical in structure under both
`--mode auto` and `--mode reliable`):

```c
unsigned int scan_textfile(void)
{
  int4 v1;
  int4 v2;
  unsigned int v3; // stack - 0x14
  void *v4; // esp
  ...
  v3 = 0x405afe;
  v2 = -sub_4098d0();
  *(void *)&PTRSUB(ESP,8)[v2] = 0;
  *(void *)&PTRSUB(ESP,4)[v2] = 0x80;
  *(void *)&PTRSUB(ESP,0)[v2] = 3;
  *(void *)&PTRSUB(ESP,-4)[v2] = 0;
  *(void *)&PTRSUB(ESP,-8)[v2] = 3;
  *(void *)&PTRSUB(ESP,-0xc)[v2] = 0x80000000;
  *(void *)&PTRSUB(ESP,-0x10)[v2] = *(void *)&PTRSUB(ESP,0x10040)[v2];
```

`ESP` appears 8 times in the body and is declared nowhere (the decl block is `v1`..`v9`;
`// esp` on `v4`..`v7` is a storage annotation, not a declaration). `PTRSUB` is likewise
undeclared. The body is not C.

Structural metrics are otherwise a tie — kuna fresh: 0 gotos, 0 labels, 2 ifs, 1 loop,
0 ternaries, 0 casts, same shape as ghidra and ida.

## Source

`~/github/decbench/results/full_run/O2-noinline/mydoom/compiled/scan.c:175`

```c
int scan_textfile(const char *filename)
{
	HANDLE hFile;
	DWORD dwRead, dwTotalRead, dwTotalFound;
	char buf[65535];

	hFile = CreateFile(filename, GENERIC_READ, FILE_SHARE_READ|FILE_SHARE_WRITE,
		NULL, OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL, NULL);
	if (hFile == NULL || hFile == INVALID_HANDLE_VALUE) return 1;
	...
}
```

The `char buf[65535]` is the whole story: a 64 KiB local forces MSVC to emit a
`__chkstk` / `_alloca_probe` call (`sub_4098d0`) that adjusts `esp` by a value the
decompiler cannot fold to a constant. Every frame reference therefore stays relative to
the *entry* `esp` with a dynamic `PTRADD` on top, and the offsets in question
(`+8`, `+4`, `0`, `-4`, `-8`, `-0xc`, `-0x10`, `+0x10040`) fall outside the mapped local
frame. Ghidra flags exactly this with `WARNING: Unable to track spacebase fully for stack`.

## Analysis

**Structural symptom (one).** The internal p-code operator `PTRSUB` and the raw register
name `ESP` leak verbatim into the emitted C — 8 occurrences in this function — producing a
body that references two identifiers it never declares. Ghidra emits a stack-location leaf
(`&stack0x00000008`) for the identical references; kuna emits the pre-render IR form.

**The miner hint is wrong and is disproven here.** The hint attributed the raw register
names to "register<->global copy-shadow merges rendering members by their own address"
(`p9_emit/printc.rs` `push_vn_explicit_ir`). That is a different, already-landed fix —
the mirror-`getSymbol()` global-resolution loop is present in the unnamed-location tail at
`decompiler/crates/kuna-decomp/src/p9_emit/printc.rs:6633-6706`. None of the 8 leaks here
are copy-shadow members; all 8 are the same `ESP` spacebase base operand, and the count
matches the hint's "8" exactly, so the miner was counting these.

**Root cause.** `PrintC::op_ptrsub_ir`'s SPACEBASE arm bails to the functional render when
no Symbol is bound to the offset constant —
`decompiler/crates/kuna-decomp/src/p9_emit/printc.rs:6969-6983`:

```rust
            // NOT the C++ `pushUnnamedLocation` `&stackNN` leaf — which would expose
            // an offset the kuna namerec layer has not yet resolved to a name.
            let name = match &sym_name {
                Some(n) => n.clone(),
                None => {
                    self.op_func_ir(fd, arch, op);
                    return;
                }
            };
```

`op_func_ir` (`printc.rs:4568`) then prints the opcode name as a call token and pushes
*both* inputs as arguments — input 0 is the `ESP` register varnode, which falls through
`push_vn_explicit_ir` to `get_register_name(...)` and renders `ESP`. Upstream Ghidra's
`PrintC::opPtrsub` handles the `symbol == 0` case with `pushUnnamedLocation(addr, ...)`
instead, which is why Ghidra never shows the operator or the register. kuna has the
`Space<hex>` unnamed-location formatter already (`printc.rs:6690-6700`, cited to
printc.cc:1964-1970) but no call site ever reaches it from a spacebase PTRSUB: a
whole-binary decompile of mydoom.exe renders `Stack<hex>` **0** times.

The docstring on `op_ptrsub_ir` (`printc.rs:6714-6720`) still calls the arm
`STUB(W4 spacebase symbol)` and says it "fall[s] through to a functional render" — the
stub is acknowledged in-tree; only the `Symbol != 0` half was completed.

**Owning phase: P9 emit.** The decision "no Symbol -> functional render" is made in
`p9_emit/printc.rs` and nowhere else. A contributing upstream difference sits in P6
(`substrate/funcdata_varnode.rs:1213` `link_symbol_reference` deliberately attaches only
*defined-named* Symbols, where C++ `linkSpacebaseSymbol` attaches undefined-named ones too
and lets `ActionNameVars` rename them), but P6 is not the fix site here: these offsets are
outside the local frame entirely, so no Symbol exists to bind under either policy — Ghidra
does not name them either. P9 owns the missing fallback.

**Blast radius.** Narrow but concentrated on the interesting functions: 4 of 161 functions
in mydoom.exe leak `PTRSUB(ESP,...)` (11 occurrences) — `sub_405af0` (this case),
`sub_408500`, `sub_4098d0` (the `__chkstk` routine itself), `sub_409a00`. All are the
large-stack-buffer / stack-probe pattern. No datatest or stage assertion contains a
`PTRSUB(` expected-output string (the single corpus hit,
`tests/stages/gh8471-thumbfuncptr.xml:9`, is prose in a comment).

**Why the reference wins — and how much.** Nothing here moves the GED: kuna is already
best-of-all at 16, tied with ghidra and ida, ahead of angr/binja/phoenix at 18. IDA wins on
*readability* for a reason outside this symptom (it recovers `char v9[65547]` and the
`CreateFileA` prototype, collapsing all seven argument stores). This is a validity /
readability defect, not a scoring gap — worth fixing on its own merit, not for the metric.

**Secondary observation, not this case's symptom.** kuna renders all 418 PE IAT calls in
this binary as `(*dat_4112a0)()` with zero arguments, where both ghidra and ida resolve the
import name and arguments (`CreateFileA(...)`). That is a separate P1/analysis-tier gap
(PE import-thunk symbol markup, which then unblocks P4 prototype/argument recovery) and
deserves its own case; it is a much larger readability lever than the `PTRSUB` leak.

## Proposed fix

**Mechanism.** In the SPACEBASE arm of `op_ptrsub_ir`, replace the `sym_name == None`
bail-to-`op_func_ir` with the upstream `pushUnnamedLocation` tail: construct the referenced
address as `Address(spacebase_space, address_to_byte(in1const, word_size))` — the same
address `resolve_spacebase_ref_addr` already computes for the link query — and push the
existing `Space<hex>` unnamed-location atom under the `&` / `valueon` handling that the arm
already codes above it. The formatter, the address resolution, and the `&`/`[0]` logic all
exist; only the wiring between them is missing. Result: `&Stack00000008` (or `&stack0x...`
if the shape is matched to upstream) in place of `PTRSUB(ESP,8)`, and no register operand
is ever pushed.

**Owning files.**
- `decompiler/crates/kuna-decomp/src/p9_emit/printc.rs` — `op_ptrsub_ir` SPACEBASE arm
  (lines ~6946-6990); reuse the unnamed-location formatter at ~6690-6700 and
  `Funcdata::resolve_spacebase_ref_addr`.
- Docstring at `printc.rs:6714-6720` drops its `STUB(W4 spacebase symbol)` marker.
- `docs/spec/` P9 chapter: describe the unnamed spacebase-reference render.

**Risks.**
- Low corpus risk: no test asserts `PTRSUB(` in expected output, and the arm is only
  reachable when *no* Symbol binds, which today already produces non-C.
- The one judgment call is the leaf's spelling. kuna's existing formatter capitalizes the
  space name (`Stack00000008`); upstream prints `stack0x00000008`. Matching upstream is
  better for diffability but is a second, cosmetic decision — pick one and state it.
- Scope is a strict output-correctness fix (undeclared identifiers today), so per
  `docs/agents.md` it needs no option flag; gate it only if a measured datatest delta
  appears. Expect 0/675 change — grep the corpus first to confirm no golden text moves.

**Scope: small.** One arm of one function, plus a stage testcase built from a binary with a
`__chkstk`-probed frame (this mydoom function, or any function with a >4 KiB local array
compiled by MSVC).
