# calleearity — whole-corpus before/after (standing requirement 7)

Method and corpus identical to `docs/features/varargstackargs/sweep.md`:
`kuna decompile-all <bin> --json` with `--option calleearity off` and
`--option calleearity on`, per-function `code` diffed, 57 binaries stratified over
PE x86·x86-64·ARM, ELF x86·x86-64·ARM·ARM64·MIPS·SPARC·PPC64 and Mach-O
x86-64·PPC·ARM64; 11 fail to load identically in both arms and are excluded.

| revision | binaries | functions | changed |
|---|---|---|---|
| first cut ("same callee, same arity") | 46 | 3857 | 43 (1.11%) |
| **shipped** (only a call that recovered NOTHING) | 46 | 3857 | **25 (0.65%)** |

## What the first cut got wrong, and what it bought

Three of the 43 were over-recovery propagated *from* the witness, and one of them
is unambiguously wrong C:

```
=== ObfuscationFiesta.exe :: sub_140002530
-      Sleep(200);
+      Sleep(200,0);

=== linker64 :: sub_18798
-  sub_1b11c(5,0,"Zip: empty archive?");
+  sub_1b11c(5,0,"Zip: empty archive?",v16,v27);
```

`Sleep` takes one `DWORD`; the second argument came from a sibling site that had
over-recovered `rdx`, and "never subtractive" propagated it. `sub_1b11c` is an
internal *variadic* logger, so its arity legitimately differs per call site —
"same callee, same arity" is simply false for it, and the format string of that
particular call has no conversions at all.

Both shapes share one property: the call already had a self-consistent argument
list, and the witness only added to it. The shipped rule refuses that case
outright and rescues only a list that came out **empty** — a function other sites
pass arguments to does not take zero here, so an empty list is a recovery failure
rather than a shorter call. `Sleep` and `sub_1b11c` are byte-identical again.

## The 25 that remain

Every one is a call that had **no** arguments gaining the arguments its sibling
sites pass, plus the variable renumbering and expression re-anchoring that keeping
a value alive causes. Classified:

* **17 functions** — an argument restored at a `malloc`/`free`-shaped internal
  wrapper in the Android `linker64` C++ demangler: `sub_16698()` → `sub_16698(size)`,
  `sub_166dc()` → `sub_166dc(ptr)`, `sub_3b760()` → `sub_3b760(&v82[-2])`,
  `sub_1d6b0()` → `sub_1d6b0(v32)`, `sub_6814c()` → `sub_6814c(*(int8 *)(a0+0x10))`.
  Several also hoist the size expression into a named local
  (`v110 = v106 * 0x30`), which is a readability gain, not a move.
* **2 functions** — the witness itself: `sub_140008160()` → `sub_140008160(v33 + 0x27)`
  in `sub_1400024a0` and the same wrapper in `sub_140003150`.
* **3 functions** — `sub_140006e10(v3)` (PolyMLP), `sub_abc(dat_8dfc)`
  (MIPS `crackme.prx`), `sub_100006e80(v4)` (`-[ViewController setupUI]`, the
  argument being the `objc_retainAutoreleasedReturnValue()` result).
* **1 function** — `free()` → `free(0)` (`PolyMLP sub_140001400`). Verified against
  the disassembly: `mov rcx,rbx` with `rbx == 0` at `0x140001749`, so the promoted
  value is the one kuna's dataflow already believed; the OFF rendering hides it.
* **2 functions** — no call changes arity; only the variable map shifts, because a
  promoted argument keeps a value live (`sub_46d58`'s own parameter also retypes
  from `char *a0[16]` to `char *a0`, and its body loses a cast).

No hunk in the 25 was classified as a regression.

## Residual risk

The rule is still "the sibling knows better", and its blind spot is a *variadic*
internal callee whose zero-argument site is genuinely zero-argument. The empty-list
condition makes that shape rare — a variadic function is called with at least its
format argument — but it is not impossible, and it is the flip-back condition named
in the option's `use_when`.
