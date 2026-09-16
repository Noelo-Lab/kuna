# elfmain — analysis

## 1. The measured symptom

kuna already recovers the address of `main` on a stripped ELF. Entry-discovery
oracle 4 (`analyzers/entry/mod.rs`, `libc_start_main_target`) decodes the
`_start` → `__libc_start_main(main, …)` idiom on x86-64, AArch64, ARM and RISC-V
and seeds the recovered VMA as a function entry. It then does nothing else with
it, so the program's own starting point arrives in the inventory as one more
`sub_<addr>`, declared with whatever reading its body alone produces:

| stripped image | before | ground truth (unstripped twin) |
|---|---|---|
| decbench coreutils `fmt` O2 @0x26a0 | `unsigned long sub_26a0(int a0,char **a1)` | `int main(int argc, char **argv)` |
| `stripped_dynamic_x86_64` @0x1405 | `unsigned long sub_1405(int4 a0,unsigned long *a1)` | same |
| `eh_lsda_x86_64` @0x40137a | `unsigned int sub_40137a(int a0)` | same |
| `emptystrconst_x86_64` @0x401189 | `void sub_401189(void)` | same |

Three separate things are wrong and each one has its own cause.

**The name.** Nothing said `main`, although the oracle recovered the address from
the call whose first argument *is* `main` by the C runtime's contract. The cost is
not cosmetic: `kuna functions` on a stripped Linux binary lists no `main` at all,
so finding where the program starts means reading bodies.

**The parameters.** kuna reads a callee's parameters out of the callee's OWN body
— an ABI argument register read before it is written is a parameter — so the two
slots appear only when this particular `main` happens to read both. One that reads
only `argc` declares one anonymous slot; one that ignores its arguments declares
none. When a slot does appear it is untyped, so the argument vector is an
`unsigned long *` and the string it holds never reaches a `char *` use.

**The return type.** This is the half body-driven recovery can never supply:
`main`'s value is consumed by `__libc_start_main`, which is outside the image, so
nothing in the object constrains it and kuna types it from the widest register
write it can see — `unsigned long` where the declaration says `int`. It is also
the half that is visible at every call site.

## 2. What the container states

The C runtime's contract states all three. glibc's `__libc_start_main` takes
`main` as its first argument, and what it then calls is `main(argc, argv, envp)`.
That is a restatement of the runtime, not an inference, and it is the shape
`machomain` already applies on Mach-O from `LC_MAIN`.

All three arguments are declared, `envp` included, because the parked prototype is
applied LOCKED. Declaring two parameters is not a weaker version of the same
claim — it asserts that there is no third one, and on a `main` that reads `envp`
that deletes a parameter recovery had already found: the entry value stops being
an input, the read becomes an uninitialised local, and the emitted C hands that
undefined local on. The in-tree fixture `armlibcmain_le32` @0x103dc is exactly
that program, and under a two-argument declaration it rendered

```c
  unsigned int v4; // r2        <-- assigned nowhere in the body
  __printf_chk(2,"kuna armlibcmain prompt",v4,0);
```

Nothing readable at load time tells that `main` apart from one that truly ignores
its third argument. It never touches `r2`: it sets up `r0`/`r1`, branches to
`__printf_chk`, and the only thing that says `r2` carries a value is the callee's
signature. A body walk looking for a read of the third argument register finds
nothing in either program. Of the two possible mistakes only one is wrong output —
an unused `envp` is a parameter in a declaration that is true of every hosted C
program — so the declaration the runtime makes is the one applied.

The claim is stronger than "oracle 4 found an address", so it needs stronger
evidence than oracle 4 needs. Its x86-64 arm matches an argument-setup encoding
and a following `call`, which is enough to add a function entry (a wrong one costs
one spurious `sub_<addr>`) but not enough to assert that the function is the C
`main`. So the pass additionally requires the image to name `__libc_start_main` at
all — the C runtime's own name, in the static or dynamic symbol names with the GNU
version suffix stripped.

## 3. How often the claim is right

`kuna functions` on every stripped ELF of the decbench corpus that has an
unstripped twin, against the twin's `main` from `nm`:

758 images have a twin. 33 of them are stripped only of `.symtab` and still export
`main` from `.dynsym`, so the pass refuses and the name comes from the symbol
table as it always did (`bash` and its six build tools, `e2fsck`, `ip`, `rtmon`,
`rsyslogd`, at all three optimisation levels). They are not evidence about this
pass and are excluded; `kuna decompile-all` on one of them is byte-identical with
and without `--option elfmain off`. That leaves **725** images where the pass is
the only thing that could supply the name:

| verdict | count |
|---|---:|
| MATCH (the address named `main` is the twin's `main`) | 612 |
| MISMATCH (named a different address) | 0 |
| MISS (declined) | 113 |

Every MISS is a refusal working as designed: an `.so` has no `main`, and a
libopencm3 / ChibiOS / FreeRTOS / betaflight firmware image has no glibc crt1, so
the `__libc_start_main` evidence the claim rests on is absent.

## 4. Metric effect, measured

Approximately zero, as predicted, and non-negative on the metric this campaign is
judged by. Both arms of `--option elfmain off`, 18 slices of grep/gzip/diffutils/
bzip2/findutils at O0 and O2:

| instrument | n | result |
|---|---:|---|
| `typesweep` (type_match) | 1312 fns | perfect 154 → 154; aggregate 449.38 (default) vs 448.67 (off). **7 functions move, every one of them `main`, and all 7 are worse with the option off.** None is worse with it on. |
| `optsweep` (GED) | 1367 fns | **18 bodies change and all 18 are `main`.** Total GED 28772 (default) vs 28770 (off): the default costs +2 over the corpus, 0.007%, and the whole +2 is grep's `main` (113→112 at O0, 112→111 at O2, against a 291-node source graph). perfect 527 → 527, nothing moved on or off. |

The GED number is the interesting one, because it is the answer to "the typed
`argv` rewrote 682 lines of grep's `main`, is that better or worse?". Measured
against the ground-truth CFG it is worth one edit, in the wrong direction, in one
function — and every other `main` the rewrite touched scores identically. The
blast radius is measured rather than asserted: outside those 18 `main`s, 1349
bodies are byte-identical in both arms and score identically.

decbench's `type_match` scores `variables[]` — arguments plus stack symbols — and
on the functions this pass fires on, `argc`/`argv` are already true positives at
the recovered widths wherever the body reads them. Return types are not scored at
all, and neither is the function's name. What this buys is the caller-visible
return type, the `char *` element type that reaches `argv[i]` uses, and a `main`
an agent can ask for by name.
