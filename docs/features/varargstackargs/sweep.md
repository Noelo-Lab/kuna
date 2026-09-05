# varargstackargs — whole-corpus before/after (standing requirement 7)

Method: `kuna decompile-all <bin> --json` with `--option varargstackargs off`
and `--option varargstackargs on`, per-function `code` diffed. 57 binaries from
`kuna-re-dataset/challenges`, stratified up to 3 per (format, arch) plus the four
observation witnesses and a set of libc-heavy ELF x86-64 crackmes. 11 of the 57
fail to load identically with the option off and on (packed/DOS/SPU images) and
are excluded from the totals.

Covered: PE x86 / PE x86-64 / PE ARM, ELF x86 / x86-64 / ARM / ARM64 / MIPS /
SPARC / PPC64, Mach-O x86-64 / PPC / ARM64.

## The shipped rule

| | binaries | functions | changed |
|---|---|---|---|
| **final** (`stack_section_split`, cut at a stack trial at offset 0) | 46 | 3857 | **1 (0.026%)** |

The single changed function is the witness, `practice_2_rengr::_main`
(`docs/features/varargstackargs/before-after.txt`). Every hunk in it is a
recovery, not a rewrite:

* three `printf("... %s\n")` calls gain the buffer the `%s` consumes;
* `scanf("%d")` gains `&v2`, and `v2` — read but never written in the OFF output —
  becomes the destination that is written;
* `v5 = 0` and `v4 = 'X'` reappear. Both are real stores (`str wzr,[sp,#0x1c]`,
  `strb w8,[sp,#0x1b]`); they had been dead-code eliminated as collateral of the
  dropped argument;
* `_encrypt_decrypt(v1,0x58)` becomes `_encrypt_decrypt(v1,(long)v4)` where
  `v4 == 'X' == 0x58` — the same value, now shown through the stack slot the
  machine code actually loads it from (`ldrsb w1,[sp,#0x1b]`).

## The counter-example the sweep found, and the rule it bought

An earlier revision cut the section at *any* first stack trial. It changed **2**
functions, and the second one was wrong:

```
=== TRVCrackme (ELF MIPS) :: main
+  unsigned int v1; // stack - 0x218
+  v1 = 0x4191f0;
-  printf("[+] Enter your name: ");
+  printf("[+] Enter your name: ",v1);
```

MIPS o32 PIC saves `$gp` across calls with `sw gp,16(sp)` / `lw gp,16(s8)`, and
`mips32be.cspec` places the first stack parameter at offset **16** — precisely
that slot, with the cspec's own comment reading *"This is backup storage space
for register params, but we treat as locals"*. Discounting the `a1`–`a3` hole
there turns the caller's own register save into a second argument to a `printf`
whose format string has no conversion at all.

The shipped rule therefore also requires the section's stack area to begin at
callee-relative offset 0. Re-measured on the same 46 binaries after the guard:
the MIPS function is byte-identical again and only the witness moves.

## Residual risk

ARM32 AAPCS puts its stack pentry at offset 0 but *does* fill `r0`–`r3` with
variadic arguments first, so a register hole there is real evidence and the guard
does not exclude it. Four ARM binaries in the sweep (`trap`, `arm_crack1`,
`arm_kgme1`, `Medusa.exe`) changed nothing, but the shape is not proven absent —
it is the reason the option ships opt-in rather than default-ON despite a clean
0/675 ablation.
