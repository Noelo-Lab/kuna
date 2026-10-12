# Public corpus evaluation

The OFF/ON comparison uses aggressive mode, one decompiler job per file and
isolated base/patch builds. The base is `dd3dd8e4b5eb3cf8c1c738a3f198704387531ca8`.
The patch engine matches the implementation promoted into aggressive; its
explicit OFF/ON settings make preset membership independent of this comparison.
Input ELF, build and all 149 SLEIGH fingerprints matched at the start and end.
Only publicly distributed Debian packages and independent synthetic controls
were used.

The initial six-second-per-function pass covers 920 ELFs and 162,130 function
records per arm. Initial errors were 761 OFF and 739 ON; these are not successful
code coverage. All 93 files with a difference or run uncertainty were repeated
with BASE/BASE/OFF/OFF/ON at 30 seconds, escalating uncertainty to 120 seconds.
No selected-function difference remained. These narrowed runs do not preserve
whole-program type context; the stable `ls` change is established by the
whole-file confirmation below.

Whole-program context matters: 35 files with initial successful-C differences
were additionally repeated without narrowing the discovered functions. These
runs cover 9,957 records per arm. Three files required larger-budget confirmation;
the 120-second pass also repeats ON. Fifteen matched pre-existing function
failures remain excluded from positive coverage. There are no unresolved
comparisons or new stable failures. BASE versus patch OFF is identical.

| Architecture | ELF files |
| --- | ---: |
| amd64 | 60 |
| arm64 | 60 |
| armhf | 60 |
| i386 | 60 |
| mips | 60 |
| mips64el | 60 |
| mipsel | 60 |
| powerpc | 60 |
| ppc64el | 60 |
| riscv64 | 60 |
| sparc32 | 216 |
| sparc64 | 104 |

## Stable changed code

Exactly one Debian function changes: i386 `ls` from `coreutils_8.13-3.5`, at
`0x8049f30`. Both repeated ON runs agree; all 341 functions finish without errors
in each 120-second whole-file run. The entire function has one changed line:

```diff
-    v12 = "n"[dat_8064408];
+    v12 = "*=>@|"[dat_8064408 - 2];
```

The ELF contains contiguous `n\0*=>@|\0` at file offset 80357. The first
string is at `0x805b9e5`; the retained string-array base is `0x805b9e7`.
`0x805b9e5 + index = 0x805b9e7 + (index - 2)` preserves the loaded byte.
The function's two switches remain unchanged; no computed goto appears.
All other successful emitted lines agree. Transient signature/struct differences
in the initial pass disappear under repeated whole-file comparisons; MIPS
`vdir` and RISC-V `stty` also show budget variation in unchanged configurations
and agree at 120 seconds. No initial transient hunk is treated as an option effect.

## Positive synthetic coverage

Independent Clang O0/O2 objects contain 240 function records per arm across all
twelve architectures. Six functions change stably (five A64 loads/stores and
one i386 byte store); all six agree under 120-second repeated comparisons.
The remaining 234 outputs are unchanged controls, not evidence that the guard fires.
Three additional A64 loads verify unsigned widening from signed arrays and
integer-bit reinterpretation of a float array; both emitted C variants pass
48 native value comparisons. A duplicated signed-short index retains both
terms and the same halfword store; all 65,536 index addresses agree.

Two more A64 controls distinguish 64-bit `slot - 1UL` from 32-bit `slot - 1`.
Only the former changes; the latter is unchanged. The unsigned modulo-64 address
identity and 65,544 boundary/domain checks verify extension order. These checks
do not assert valid C array accesses outside the declared bounds.

The separate 128-function synthetic SPARC batch changes all 128 functions to
their mapped array name. Unaffected Debian outputs and these positive triggers
are reported separately. The permanent SPARC32/A64 stages cover the option
boundary, rejection conditions, integer widths and floating bit loads.
