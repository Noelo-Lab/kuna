# calleearitylive — whole-corpus before/after (standing requirement 7)

Method identical to `docs/features/calleearity/sweep.md`: `kuna decompile-all <bin> --json`
with `--option calleearitylive off` and `--option calleearitylive on`, per-function `code`
diffed. 65 binaries from `kuna-re-dataset/challenges`, up to 6 per (format, arch) over
ELF x86-64 / i386 / ARM / ARM64 / MIPS / PowerPC / SPARC, PE x86-64 / x86 / ARM and
Mach-O x86-64 / arm64, plus the three witnesses (`graphy`, and `ObfuscationFiesta.exe` /
`linker64`, the two regressions the `calleearity` sweep measured). 14 of the 65 are
DOS/packed images that do not load identically in both arms and are excluded.

| | binaries | functions | changed |
|---|---|---|---|
| **shipped** | 51 | 9753 | **42 (0.43%)** |

Full per-function diff: `before-after.txt`.

## The two regressions this rule had to refuse

`calleearity`'s sweep measured what happens when the empty-only gate is dropped:

```
=== ObfuscationFiesta.exe :: sub_140002530     Sleep(200)  ->  Sleep(200,0)
=== linker64 :: sub_18798                      sub_1b11c(5,0,"Zip: empty archive?")  ->  +2 arguments
```

Both are byte-identical with `calleearitylive on`, and for the reason the rule was
designed around rather than by accident. `Sleep` is an import: there is no body at its
entry to decode, so the probe declines. `sub_1b11c` is an AArch64 variadic logger whose
prologue is a register-save area — `str x3,[sp,#136]; stp x4,x5,[sp,#144]; stp
x6,x7,[sp,#160]` — so it reads `x5`, `x6` and `x7`, argument registers a five-argument
witness does not claim, and the callee's own body contradicts the witness.

## The 42 that changed

Every one is an argument **added** at a call; no statement is deleted or re-anchored, and
no call loses an argument. Where the callee is a named libc/libgcc function the recovered
arity is checkable against its real signature, and all fourteen are right:

| callee | before -> after | real signature |
|---|---|---|
| `mempcpy` | 2 -> 3 | `(dest, src, n)` |
| `__rawmemchr` | 1 -> 2 | `(s, c)` |
| `__overflow` / `__woverflow` | 1 -> 2 | `(FILE *, int)` |
| `mbrtowc` | 3 -> 4 | `(pwc, s, n, ps)` |
| `__mpn_rshift` | 3 -> 4 | `(rp, up, n, cnt)` |
| `__mpn_impn_mul_n` | 4 -> 5 | `(prodp, up, vp, size, tspace)` |
| `__mpn_impn_sqr_n` | 3 -> 4 | `(prodp, up, size, tspace)` |
| `read_encoded_value_with_base` | 3 -> 4 | `(encoding, base, p, val)` |
| `execute_cfa_program` | 3 -> 4 | `(insn_ptr, insn_end, context, fs)` |
| `add_fdes` | 2 -> 3 | `(ob, accu, this_fde)` |
| `frame_heapsort` | 2 -> 3 | `(ob, fde_compare, erratic)` |
| `__correctly_grouped_prefixmb` | 3 -> 4 | `(begin, end, thousands, grouping)` |
| `char_buffer_add_slow` | 1 -> 2 | `(buf, ch)` |
| `__udivsi3` / `__aeabi_idiv` | 1 -> 2 | `(num, den)` |

The rest are internal `sub_<addr>` callees in `linker64`, `LOL`, `crackme.prx`,
`Secrety_x64.exe` and the witness itself, each gaining one argument (two in one case) at
the sites that were short of their siblings. The collateral is the renumbering and the
declaration-list churn that keeping a value alive causes — one ARM fragment
(`libmodplug sub_134dc`) also gains its own parameter list, because the argument the call
now consumes makes the enclosing function's own input trial active.

No hunk was classified as a regression.

## Residual risk

The rule still copies from a witness, and its blind spot is a callee decoded in the wrong
instruction mode (an ARM/Thumb boundary), where the bytes the probe reads are not the
instructions that run. That is the flip-back condition named in the option's `use_when`,
and it is shared with `calleedeadarg`, which takes the same decode for the subtractive
direction.
