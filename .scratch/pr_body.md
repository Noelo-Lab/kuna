## The problem

On a stripped ELF, `main` is reported as one more `sub_<addr>`, with a return
type kuna guessed and parameters that appear only if the body happens to read
them. kuna already knows the address — entry oracle 4 recovered it from
`_start` — and uses it for nothing but discovery.

```console
$ kuna decompile-all decompiler/crates/kuna-analysis/tests/fixtures/stripped_dynamic_x86_64 --addr 0x1405
// Function: sub_1405 @ 0x1405
unsigned long sub_1405(int a0,unsigned long *a1) // early-return
{
  unsigned long v1; // rax

  if (a0 <= 1) {
    fprintf(stderr,"Usage: %s <binary>\n",*a1);
    return 1;
  }
  v1 = sub_1357(a1[1]);
  return v1;
}
```

`0x1405` is what crt1 hands `__libc_start_main` (`lea rdi,[rip+0x286]` at
`0x1178`), and `nm` on the unstripped twin agrees. `kuna functions` on that file
lists no `main` at all.

## The fix

New option **`elfmain`** (`on|off`, default **on**), a P1 analysis pass in
`decompiler/crates/kuna-analysis/src/analyzers/entry/kuna_elfmain.rs`:

- names that address `main` through the existing `entry_names` overlay and parks
  `int main(int argc, char **argv)` on it — the ELF counterpart of `machomain`,
  and the reason the real `int`/`char **` is honest here where
  `entrymainproto` reports only the widths a PE call site establishes.
- takes the address from oracle 4 itself, never a second decode, so it cannot
  disagree with the entry discovery already contains and covers x86-64, AArch64,
  ARM and RISC-V by construction. The non-PIE ARM32 shape oracle 4 cannot see
  stays `armlibcmain`'s; only the prototype is added there, so that option still
  restores its own inventory exactly.
- requires the image to name `__libc_start_main`. Oracle 4's evidence is enough
  to add an entry but not to assert the function IS the C `main`, and a fully
  stripped static image states nothing that separates a glibc crt1 from a
  hand-rolled entry point. It also refuses a non-ELF image, an address that
  already carries a function symbol, and an image that already spells `main`.
- a `--define-function <addr>=<name>` spelling now outranks every discovered name
  at that address. Two names reaching one address was latent before; `main` (4
  chars) would have displaced a caller's `stage1` (6) on `entry_name_rank`'s
  length tie-break.

## The tests

`tests/stages/kuna-elfmain.xml` (4 assertions, two-pass: off = the bug, default =
the fix), 5 unit tests over the claim and its refusals, and
`tests/cli/elf-libc-start-main-unnamed.json`. Cross-checked against ground truth
on 758 stripped decbench ELFs with unstripped twins: **645 match `nm`'s `main`,
0 mismatch, 113 declined** — 90 bare-metal firmware with no glibc crt1, 23 shared
libraries with no `main`. Whole-corpus `decompile-all` diff over 143 in-repo ELF
fixtures: 13 change, 130 byte-identical; every hunk is the declaration, the name
at the `__libc_start_main` call site, the `argv` element type reaching a call, or
the return statement of a tail call (classified in `docs/features/elfmain/`).
Five expectations move, all of them naming the same addresses:
`verify_crossarch_entry_main` now runs both arms (off = the pre-existing
`sub_<addr>` gate verbatim), `verify_s1_entry` keeps its subject with the option
off, `verify_assertflow` spells aif_gap's `0x13c9` `main`, and the ghidra-sim
faillog placeholder pin drops 27 → 26 with every other measurement on that
fixture unchanged. Speed, min of 31 interleaved: fmt O2 `main` 236 → 238 ms
(+0.85%), control 107 → 106 ms. Metric effect is ~0 by construction — `type_match` scores
`variables[]`, where `argc`/`argv` already counted; the gain is the name, the
caller-visible return type and the `char *` that now reaches `argv[i]`.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01MAeimyWm5oSewpeCgAwY29
