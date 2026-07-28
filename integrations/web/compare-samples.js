// compare-samples.js — the data behind the "compare" section of the landing page.
//
// Each sample is one function, decompiled by kuna, shown next to the same
// function as rendered by something else (another decompiler, or the original
// source). The page reads only this file — adding a sample is a data edit, no
// markup changes.
//
//   RIVALS   the right-hand pane's options, in dropdown order.
//   SAMPLES  the left dropdown. `kuna` is the left pane; `vs[<rival id>]` the
//            right one. A rival with no entry renders a "not recorded yet"
//            placeholder, so a half-filled sample is honest rather than empty.
//
// To add a comparison:
//   1. kuna side (verbatim, no hand-editing — that is the whole point):
//        kuna decompile ./binary <function>
//   2. rival side: paste that decompiler's output for the same function.
//   3. drop both into SAMPLES below as `kuna:` and `vs: { <rival>: ... }`.
//
// Everything committed here is real tool output. Do not paste anything
// hand-written or reconstructed into a pane — the section is a claim about
// what these tools actually print.

export const RIVALS = [
  { id: 'source', name: 'Original C source' },
  { id: 'ghidra', name: 'Ghidra' },
  { id: 'ida', name: 'IDA (Hex-Rays)' },
  { id: 'binja', name: 'Binary Ninja' },
  { id: 'angr', name: 'angr' },
];

// The right-hand pane defaults to this rival (it should be one that has output
// recorded for most samples, so the section does not open on a placeholder).
export const DEFAULT_RIVAL = 'source';

export const SAMPLES = [
  {
    id: 'x86-sum_to',
    name: 'sum_to() — counted loop, x86-64',
    meta: 'ELF · x86-64 · gcc 11.4, PIE, DWARF · 0x1161',
    note: 'The accumulator stays a single variable and the induction loop comes back as a for.',
    kuna:
`int8 sum_to(int4 a0)

{
  int8 acc; // stack - 0x10
  int4 v1; // stack - 0x14

  acc = 0;
  for (v1 = 1; v1 <= a0; v1 = v1 + 1) {
    acc = acc + v1;
  }
  return acc;
}`,
    vs: {
      source:
`long sum_to(int n) {
    long acc = 0;
    for (int i = 1; i <= n; i++) acc += i;
    return acc;
}`,
    },
  },

  {
    id: 'x86-main',
    name: 'main() — nested calls, x86-64',
    meta: 'ELF · x86-64 · gcc 11.4, PIE, DWARF · 0x1198',
    note: 'Call results feed straight into the next call instead of spilling to a temporary per step.',
    kuna:
`int8 main(int4 a0)

{
  int8 v1; // rax

  v1 = sum_to(add(a0,3));
  printf("%ld\\n",v1);
  return v1;
}`,
    vs: {
      source:
`int main(int argc, char **argv) {
    int x = add(argc, 3);
    long s = sum_to(x);
    printf("%ld\\n", s);
    return (int)s;
}`,
    },
  },

  {
    id: 'arm-sum_to',
    name: 'sum_to() — the same loop on AArch64',
    meta: 'ELF relocatable · AArch64 · clang -O0, freestanding · 0x400048',
    note: 'Same engine, same shape, different instruction set — the SLEIGH spec is the only thing that changed.',
    kuna:
`int8 sum_to(int4 a0)

{
  int4 v1; // stack - 0x14
  int8 v2; // stack - 0x10

  v2 = 0;
  for (v1 = 1; v1 <= a0; v1 = v1 + 1) {
    v2 = v2 + v1;
  }
  return v2;
}`,
    vs: {
      source:
`static long sum_to(int n) {
    long acc = 0;
    for (int i = 1; i <= n; i++) acc += i;
    return acc;
}`,
    },
  },
];
