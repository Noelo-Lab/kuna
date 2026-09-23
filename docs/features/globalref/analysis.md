# globalref — analysis

A constant address used as a pointer prints as the global it names.

## 1. The gap

kuna prints a cast on a constant address that IDA never prints:

```c
sub_c240((struct_4 *)0x220c0);         // kuna main, /usr/bin/ls sub_c2e0
sigprocmask(0,(sigset_t *)0x22780,&v1);
sub_E4CA(qword_2B080);                  // IDA, coreutils ls -O0
```

On the 4,815 functions kuna and IDA both emit (the castbench corpus, 45 binaries
at O0/O2/O2-noinline), constant-address casts were 572 at the campaign baseline
and 1,118 on main; IDA prints essentially none, because it names every
referenced static address.

## 2. Census (main `dbe854ba3`, every function, not only the shared set)

Classes of `(T *)0x<addr>` by where the address lands, from the ELF section and
symbol tables:

| class | fmt/ls/sort/du O0+O2 | 45-binary corpus |
|---|---:|---:|
| unnamed global in `.bss` | 187 | 1,598 |
| unnamed global in `.data` | 76 | 317 |
| unnamed global in `.rodata` | 59 | 306 |
| unnamed global in `.data.rel.ro` | 18 | 186 |
| in a loader table (`.dynstr`, `.dynsym`, `.rela.dyn`, version tables, `.eh_frame`) | 2 | 42 |
| not an address (`0x1`, `0xffffffff`, `0x2710`: outside every section) | 71 | 693 |
| a function address (`.text`, `.plt`) | 0 | 76 |
| inside an existing symbol | 0 | 0 |
| a string literal not rendered | 0 | 0 (14 are non-`char` pointers at printable bytes) |
| total | 413 | 3,232 |

By context on the small set: 183 call arguments, 90 assignments, 53
comparisons, 16 returns. No constant address is dereferenced: a load from a
constant becomes a direct `ram` Varnode, which already prints as `dat_<addr>`.

**Why none became a symbol reference.** The pointer arm of
`PrintC::push_vn_explicit_ir` (the port of `pushConstant`) has one spelling for
a pointer-typed constant: the forced-hex integer behind a cast. The only road to
a name is upstream's `ActionConstantPtr::isPointer`, which asks the global scope
for a Symbol covering the address. A stripped image has none for most of its
data, only the copy-relocated imports (`stdout`, `optarg`) and the `char[N]`
string symbols the analysis tier plants. A constant that lands inside one of
those IS linked (a pointer-typed constant may land mid-symbol), which is why the
"inside an existing symbol" row is empty. The `dat_<addr>` kuna prints two lines
away comes from the unnamed-location leaf, which names storage, a `ram` Varnode;
a constant is not storage, so that leaf is never reached for one. Minting a
symbol at P0 is not a lever: every stock cspec claims the whole `ram` space as
global scope, so `global add` is a measured no-op, and the type the object
should have is known only after the caller's decompile.

**Why the campaign doubled it.** The constant is pointer-typed because the
parameter it reaches is. Ablating one option at a time (globalref off,
fmt/ls/sort/du O0+O2, in-data constant casts): default 342; `protoorder off`
193; `ptrfromuse off` 315; `libcsigs off` 316; `structsynth off` 336;
`calleevote off` 336; `passthrough off` and `slotptr off` 342. `protoorder`, a
callee's recovered prototype stated to its callers, is 44% of it: at the
campaign baseline `sub_e4ca(0x2b080)` had no cast because its parameter was an
integer.

## 3. The refusals

Each is a case where the name would claim something the function does not show.
Unique (function, address, type) events on the corpus under
`KUNA_GLOBALREF_TRACE=1`: named 1,614; outside every data section 553; storage
also accessed directly at another start, width or type 209; a `void *` reader
C would not convert for 36; two pointed-to types in one function 21; a pointer
to code 19; the value also read as a number 1.

The direct-access refusal is the one that protects a value. `ls`'s
`sigemptyset((sigset_t *)0x2b460)` sits beside `v2[1] = (void *)dat_2b460;`
(the mask copied as eight-byte words); a `sigset_t dat_2b460` would contradict
that read. A direct `int` read beside an `unsigned int *` use would change what
a compare on it means once the header declares the global, so the types must be
the same C type; a two-, four- or eight-byte `undefinedN` is accepted against
the unsigned integer of its size, which is how every surface spells it (a
one-byte unknown is `char` on the CLI and `unsigned char` in the console, so it
matches only itself).

## 4. Value

`&dat_<addr>` has exactly the constant's pointer type and is the constant's
value wherever `dat_<addr>` is placed at `<addr>`. The round trip in
`kuna-cli/tests/decompile_all_cli.rs` places every name at its address
(`--defsym`), maps the fixture's data where the binary keeps it, and runs the
printed callers compiled with gcc and with clang: both arms print exactly what
the binary prints (`210 21 27 121 1 0 4 229 1 -8608764254683430263 2`). Moving
one name by eight bytes changes the output, so the test can fail.

## 5. The header, and what it cannot do

A project header holds one declaration per address, each function declares the
object at the type it uses it at, and other functions read the same
`dat_<addr>` directly at the type they printed. A scalar declaration that
disagrees with a direct access would silently change what that access computes
in a recompile (seen before the rule existed: `dat_1a420 = 0x69647473;` against
a `char dat_1a420` declared from a `strcpy` argument, truncated). So a record
some function takes the address of wins (a scalar access of it does not
compile), then the one type the direct accesses agree on, then the taken types;
direct accesses at two types leave the name undeclared. Over the 45 exports:
891 declarations, 173 with another type in the comment, 60 undeclared.
`gcc -fsyntax-only`: 36,648 -> 36,471 errors (direct reads of a now-declared
name compile), 14,962 -> 15,348 warnings. Every new diagnostic on a line the
option changed is an incompatible-pointer warning where the one declaration
cannot match that function's use, or (56) an undeclared name for one of the 60;
every new one on an unchanged line is a former "undeclared" error the compiler
now reports as a type error. None is a silent change.
