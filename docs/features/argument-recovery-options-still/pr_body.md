## The problem

One helper, fifteen call sites in one function, two different arities — from
machine code that is instruction-for-instruction identical at the good sites and
the bad ones.

```
$ kuna decompile ./graphy sub_1002c90 --option calleearity on --option varargstackargs on
    sub_1005250(&v24,(unsigned long)v16 & 0xffffffff,(unsigned long)v82 & 0xff);
    sub_1005250(&v24,(unsigned long)v16 & 0xffffffff,v8 & 0xff);
    sub_1005250(&v24,v22 & 0xffffffff,v8 & 0xff);
    sub_1005250(&v24,v22 & 0xffffffff,v8 & 0xff);
      sub_1005250(&v24,v22 & 0xffffffff,v8 & 0xff,(unsigned long)v16 & 0xffffffff,sub_1004ef0(v23,0));
      ...ten more with five arguments
```

The callee decompiles as `sub_1005250(long *,uint,uchar,uint,uchar)`, and both
shapes assemble the same way:

```
$ cstool x64 0 "0f b6 55 98 44 0f b6 c0 48 8d 7d c0 44 89 fe 44 89 e1 e8 84 1e 00 00"
 0  movzx edx, byte ptr [rbp - 0x68]
 4  movzx r8d, al
 8  lea   rdi, [rbp - 0x40]
 c  mov   esi, r15d
 f  mov   ecx, r12d          <- the argument that disappears
12  call  0x10051a6
```

`ecx` and `r8d` are written by dedicated `mov`s immediately before the call, at
the sites that keep them and at the sites that lose them alike. The three-argument
rendering is wrong output, not a shorter call.

## The fix

- New option `calleearitylive` (default on, DIV-123). A call site whose recovered
  argument list is a strict **prefix** of a sibling call's is extended to that
  list — the case `calleearity` and `calleearityfwd` both decline, because both
  refuse a site that recovered anything at all.
- That refusal was bought by measurement, so the relaxation needs new evidence,
  not a gate flip: dropping it turns `Sleep(200)` into `Sleep(200,0)` and gives a
  variadic logger two arguments its format string has no conversions for. The
  evidence is the **callee's own body**, read with the bounded entry decode
  `calleedeadarg` already takes, and read for two things — every register the
  witness claims beyond this site's list is read before written by the callee,
  **and no other argument register of the model is**.
- The second half is the whole discriminator. An import has no body to decode; a
  variadic register-save prologue (`str x3,[sp,#136]; stp x4,x5,[sp,#144]; stp
  x6,x7,[sp,#160]`) reads argument registers a five-argument witness does not
  claim. Both measured regressions stay byte-identical.
- Deferred to the end of `ActionActiveParam::apply` rather than reordering
  finalization: on the witness the four short sites are the *first* four, so an
  in-order rule has no witness at any of them.

## The tests

`tests/stages/kuna-calleearitylive.xml` — two callee/caller pairs that differ only
in the callee: one is extended, the other is declined because the callee also reads
`r8d`. Both are short with the option off. Plus `tests/cli/argument-recovery-options-still.json`
and five unit tests on the two decisions this module owns.

Gates: `make test` PARITY OK 675/675 (0 assertions changed), `make test-stages`
PARITY OK 644/644 (4 new, none pre-existing moved), `make rust-test` green,
`make check-spec` green, `kuna catalog --check` OK. Whole-corpus sweep: 51
binaries, 9753 functions, **42 changed (0.43%), zero regressions** — every hunk an
argument added, and the fourteen named-libc cases (`mempcpy` 2→3, `mbrtowc` 3→4,
`__mpn_impn_mul_n` 4→5, `__aeabi_idiv` 1→2, …) all match the callee's real
signature. Speed −14.79% on the witness over 7 repeats, inside the noise floor.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
