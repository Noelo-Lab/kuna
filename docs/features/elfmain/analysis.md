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
`main` as its first argument and the POSIX declaration of `main` is
`int main(int, char **)`. That is a restatement of the runtime, not an inference,
and it is exactly the shape `machomain` already applies on Mach-O from `LC_MAIN`.

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

| verdict | count |
|---|---:|
| MATCH (the address named `main` is the twin's `main`) | see record.json |
| MISMATCH (named a different address) | 0 |
| MISS (declined) | bare-metal firmware and shared libraries |

Every MISS is a refusal working as designed: an `.so` has no `main`, and a
libopencm3 / ChibiOS / FreeRTOS / betaflight firmware image has no glibc crt1, so
the `__libc_start_main` evidence the claim rests on is absent.

## 4. Metric effect

Approximately zero, and that is expected rather than disappointing. decbench's
`type_match` scores `variables[]` — arguments plus stack symbols — and on the
functions this pass fires on, `argc`/`argv` are already true positives at the
recovered widths wherever the body reads them. Return types are not scored at all,
and neither is the function's name. What this buys is the caller-visible return
type, the `char *` element type that reaches `argv[i]` uses, and a `main` an agent
can ask for by name.
