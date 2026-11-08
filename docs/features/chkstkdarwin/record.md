# chkstkdarwin — the Darwin AArch64 stack probe keeps the registers it preserves

## The defect

Apple clang calls `___chkstk_darwin`, with the byte count in `x9`, before it
allocates a frame larger than a page or a variable-length array. On arm64 the
call goes through the GOT (`adrp x16; ldr x16,[x16,#off]; blr x16`). The
libsystem_pthread implementation saves and restores `x10`/`x11` and writes only
`x9` and the flags, and compiled code reads the argument registers after the
call. The AArch64 model kills `x0`, `x1` and `x8`-`x18` at every call, so those
reads became the probe's return value or a register nothing assigns:

```
int _big_frame(void)                       // source: int big_frame(int k)
{
  char v1 [19984]; // stack - 0x4e30
  int v2; // w0

  v2 = __chkstk_darwin();
  v1[v2] = (char)v2;
  return v1[v2 / 2] + 3;
}
```

## The change

`p2_lift/kuna_chkstkdarwin.rs` removes the probe call from the raw p-code
before FuncLink and heritage (`option chkstkdarwin`, default on, AArch64
languages only). A jump to the probe that the flow turned into a call (the
probe's own `__stubs` entry) is left alone. See `docs/spec/02-lift-and-flow.md`,
"Darwin AArch64 stack probes".

## Evidence

`tests/stages/kuna-chkstkdarwin.xml` over `chkstkdarwin_macho_arm64` has 8
assertions. Main does not know the option, so the test errors there; with the
option line removed main passes 1 of 8 (the `__stubs` entry check). The branch
passes 8 of 8. The datatest corpus is unchanged (675/675).

Sweep: `kuna decompile-all` of every function, main build against this branch,
with `--option chkstkdarwin off` and at the default.

| Corpus | Functions | Off vs main | Default vs main |
|---|---:|---|---|
| 18 in-repo arm64 Mach-O fixtures | 67 | identical | only `chkstkdarwin_macho_arm64`: its 3 probe callers and `_main` |
| 16 small Homebrew arm64 tools | 758 | identical | 21: the 16 probe callers, 3 callers of them, 2 renumbered `struct_N` |
| 6 medium Homebrew arm64 tools (metaflac, mpg123, gsort, opj_decompress, gpgtar, x265) | 2,015 | identical | 24: the 11 probe callers, 2 callers of them, 11 renumbered `struct_N` |

Every function that calls the probe loses the call and, where it reads one,
gains the argument it was given: `giftool`'s `_main(void)` becomes
`_main(int a0,char **a1)`, `ffhash`'s `file = (char *)__chkstk_darwin();`
becomes the `char *a0` parameter of `_check`, and gsort's `_compare(void)`
becomes `_compare(long *a0,long *a1)`. The other changed functions call one of
them and pass it a typed argument (gsort's `_sequential_sort` takes `long *`
where it took `unsigned long *`, and two 64-bit copies it makes through `d0`/`d1`
now share a `long` local), or only renumber synthesized structs.

On Swift code the probe sizes each resilient or generic local. In Homebrew's
xcodegen 2.45.4, the function at `0x100029ba4` called `__chkstk_darwin` three
times and sized its allocations from three locals nothing assigns
(`v21 = v19 - (v14 + 0xfU & 0xfffffffffffffff0);`, `long v14; // x8`); it now
reads each size from the type metadata
(`*(long *)(v22 + 0x40) + 0xfU & 0xfffffffffffffff0`). With the option off the
function is byte-identical to main.

The slot walk also follows the arm64e `adrp x17; add x17,x17,#off;
ldr x16,[x17]; blraa x16,x17` form. Main does not name that slot, so arm64e
output is unchanged; with the slot named by hand (`--define-function
0x100004010=__chkstk_darwin` on an arm64e build of the repro) the probe is
removed and the parameter returns.

## Speed

Median `decompile-all` wall time, main against the branch, interleaved runs
after a warm-up on an arm64 Mac:

| Binary | Runs | Main | Branch | Delta |
|---|---:|---:|---:|---:|
| `chkstkdarwin_macho_arm64` | 9 | 0.231 s | 0.226 s | -1.8% |
| cws2fws | 9 | 0.296 s | 0.309 s | +4.6% |
| vwebp | 9 | 0.907 s | 0.889 s | -2.0% |
| speexdec | 9 | 1.295 s | 1.318 s | +1.8% |
| giftool | 9 | 2.580 s | 2.604 s | +0.9% |
| x265 | 5 | 6.699 s | 6.658 s | -0.6% |
| metaflac | 5 | 16.436 s | 16.253 s | -1.1% |

The pass walks at most 32 raw ops back from each indirect call of an AArch64
function, and does nothing elsewhere.
