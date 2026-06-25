# Analysis — `test_decompiling_nl_i386_pie::usage` (i386 PIE PLT import resolution)

## The gap

- **Binary:** `/home/mahaloz/github/angr-dev/binaries/tests/i386/nl` — ELF 32-bit i386 **PIE**, not stripped.
- **Function:** `usage` @ `0x1f60`.
- angr (9.2.213): ~80 loc, no goto, no spurious loop, libc calls resolved by name
  (`dcgettext`, `exit`, `__printf_chk`, `fputs_unlocked`, …).
- kuna: ~209 loc, a spurious `do { … } while(true)` loop, a `goto label_1fb0`, **three
  un-unified ESP SSA values** (`void *v10; // esp`, `v11`, `v12`), explicit return-address /
  argument stack stores (`*(void *)&v12[-0x14] = 0x1fb0;` …), a recovery-failure marker, and
  unresolved call names (`sub_fb0()`, `sub_eb0()`).

See `angr-vs-kuna.txt` for the full side-by-side and the metrics
(`loc 80|209`, `gotos 0|1`, `labels 0|1`, `loops 0|1`, plus "kuna emitted a recovery-failure marker").

## Root cause (one construct, fully pinned)

`usage` has a **shared `exit(a0)` tail** at `0x1fa7`:

```
1f80: je   1fb0              ; if (a0 == 0) goto help-body
1f82: …    push args; call dcgettext; call __fprintf_chk   ; a0 != 0 path
1fa4: add  $0x20,%esp
1fa7: sub  $0xc,%esp         ; <-- shared exit tail
1faa: push %ebp              ;     status = a0
1fab: call exit@plt          ;     NO RETURN
1fb0: …    print the help text … ; a0 == 0 path
…21..: jmp 1fa7              ;     help body ends by jumping to the shared exit tail
```

Both branches converge on the `exit(a0)` tail at `0x1fa7`.

kuna does **not** mark `exit@plt` no-return, so it treats `call exit` at `0x1fab` as falling
through to `0x1fb0`. That manufactures a bogus back-edge `0x1fa7 ⇄ 0x1fb0`:
- real edge `0x1fb0 → 0x1fa7` (the help body's `jmp` to the shared tail), plus
- bogus edge `0x1fab → 0x1fb0` (the assumed fall-through past the no-return `exit`).

The loop is the cause of everything downstream. In `print raw` the stack pointer becomes a
**mutually-referential MULTIEQUAL cycle** with no constant solution:

```
ESP:4bb = phi(ESP:992 [=SP_in-0x2c], ESP:4bc)
ESP:4bc = phi(ESP:991 [=SP_in-0x2c], ESP:8b2)
ESP:8b2 = ESP:4bb - 0x10          ; the back-edge decrement
```

Because the SP never resolves to a constant frame offset, every frame slot renders as an
explicit `*(void *)&v1x[-N] = …` store, call-argument stores at unmatched offsets are dropped
(so calls render with no args), and the structurer emits the `do {} while(true)` + `goto` +
recovery marker. This is the same *class* of pathology that `kuna_stackprobeloop` resolves —
an unresolvable spacebase MULTIEQUAL — but here the loop is **spurious** (created by the
missing no-return), so the right fix is upstream of the structurer.

## Why `exit` isn't marked no-return → the actual fix site

kuna already ships the no-return machinery:
- `kuna-analysis/src/s1_loader/noreturn.rs` (`NoReturnKnownPass`) flags any function whose name
  matches the vendored `ElfFunctionsThatDoNotReturn` list — including `exit` — **but only if
  the PLT stub for `exit` is first resolved to a name+address** by
- `kuna-analysis/src/s1_loader/elf_plt.rs::resolve_plt_imports → decode_i386`.

`decode_i386` (elf_plt.rs:261-280) decodes **only** the non-PIC stub form
`FF 25 <abs32>` (`jmp *abs32`). This i386 **PIE** binary's stubs are
`FF A3 <disp32>` (`jmp *disp32(%ebx)`, GOT-relative) — the code comment explicitly calls this
"not statically decodable — skipped (documented seam)".

It **is** statically decodable. `%ebx` holds the GOT base (set by the
`call __x86.get_pc_thunk.bx; add $0x7f73,%ebx` PIC prologue), which is the
`_GLOBAL_OFFSET_TABLE_` symbol / `.got` base. So `got_slot = GOT_base + disp32`. Verified on `nl`:

```
_GLOBAL_OFFSET_TABLE_ = 0x9edc
_exit@plt stub @ 0xe40:  jmp *0x14(%ebx)        disp = 0x14
0x9edc + 0x14 = 0x9ef0  ==  R_386_JUMP_SLOT r_offset for _exit   ✓
```

So **no** i386-PIE PLT import is named, `exit` is never flagged no-return, and the whole
structural failure cascades. The same one fix (teach `decode_i386` the `FF A3 <disp32>` PIE
form) both (a) names every i386-PIE PLT call (matching angr) and (b) — via the *existing*
no-return pass — marks `exit` no-return, collapsing the spurious loop and restoring stack
recovery.

## Owning stage

S1 loader / analysis tier — `kuna-analysis::s1_loader::elf_plt` (PLT/GOT import naming), feeding
`s1_loader::noreturn` (`NoReturnKnownPass`) and the engine's no-return flow-halt. **Not** an
S1–S9 decompiler Action/Rule.

## Hypothesis / minimal fix

Extend `decode_i386` to also match `0xFF 0xA3 <disp32>` and compute
`got = GOT_base + disp32`, where `GOT_base` comes from the `_GLOBAL_OFFSET_TABLE_` symbol
(fallback `.got.plt` / `.got` section base). It is the i386-PIE analog of the already-shipped
`decode_x86_64` (RIP-relative) and `decode_aarch64` (adrp/ldr veneer) decoders. Blast radius on
the 675 datatests is zero (all use raw `<binaryimage>` bytechunks that never reach
`resolve_plt_imports`; no i386-PIE binary in the corpus).

## Why this is routed as a `[PROPOSAL]` (not a direct feature PR)

The fix is small and clean, but it does **not** fit the feature-worker mold and changes default
behavior for a whole binary class — see `proposal.md` for the go/no-go writeup. In short:
the fix lives in the **analysis-tier loader** (not a `kuna-decomp` Action/Rule with an
Architecture flag / ElementId), it ships as loader fidelity that changes **default** output for
**every i386-PIE binary**, and the prescribed `tests/stages/*.xml` bytechunk harness **cannot**
exercise a PLT/no-return loader path — testing needs a vendored i386-PIE ELF fixture + a
`kuna-console` integration test (precedent: `verify_aarch64_plt.rs`). Those deviations from the
worker's Definition of Done warrant a human go/no-go before an implementation worker is spent.
