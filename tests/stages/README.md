# Stage-model issue testcases

`kuna-armshiftguard.xml` checks option `armshiftguard` (default `wide`) on
clang -O2 Thumb code. A register LSL whose count is masked with `& 0x1f`, or is
a four-bit field, prints the plain `1 << (pin & 0x1f)` because its count cannot
reach 32; an unbounded count prints the plain `value << (count & 0xff)` too.
clang's inline 64-bit left and right shifts split the count into `n`,
`n - 32` and `32 - n` and rely on the machine's 0 for a word shift by 32 or
more, so their LSLs keep the `n < 32u ? x << n : 0` guard. `needed` guards the
unbounded 32-bit count as well, `never` drops the 64-bit shifts' guards,
`always` restores all three 32-bit guards, and with `armconstshift on` a
guard-free shift of a signed literal keeps its unsigned conversion.

`kuna-elfv1-symbolextent.xml` checks that the gap walk (`aif`) names no function
inside the code of a sized PowerPC64 ELFv1 symbol, 28 or 24 bytes long, in both byte
orders. It keeps a routine with no symbol, a symbol, a call target and a descriptor's
code inside other functions, and a routine behind a descriptor symbol that states the
descriptor's size beside a `.name` symbol.

`kuna-returndup-auto.xml` checks option `returndup` (default `auto`) on gcc -O0
x86-64: a guard clause `if (!c) return 0;` keeps its own return, a body that
shares the epilogue's `return 3` prints one return, a void `if (c) return;`
keeps its bare return and `if (c) return a; return b;` returns each
parameter. Under `option returndup on` both guard clauses print wrapped by the
size rule and the two parameter returns merge into one variable.

`kuna-iteregion-auto.xml` checks option `iteregion` (default `auto`) on gcc
-O0 x86-64: a conditional value that each arm loads into a register before one
store after the join prints as `?:`, also when one arm is computed, and an
if/else whose arms each store 1 or 0 keeps its `if`. Under `option iteregion on`
the first prints as an `if` without an `else`, the stores fold into a
conditional and the computed arm keeps its `if`.

`kuna-loopcontinue-auto.xml` checks option `loopcontinue` (default `auto`) on
gcc -O0 x86-64, and `kuna-loopcontinue-auto-thumb.xml` on gcc -O0 Thumb: in a
`for` loop whose first two tests share one bare `continue;` and whose third
has its own, the two print as one `if (...) continue;` and the third as
another, and the plain `if` after them stays an `if`; a loop test and a bare
`break;` test stay apart, so the loop keeps its `for` header and prints the
`break`; a `continue;` in a `do` loop prints as written. Under `option
loopcontinue on` the four tests fold into one chain, the break test folds into
the loop condition and the `do` loop nests its call.

`kuna-loweredswitch-auto.xml` checks option `loweredswitch` (default `auto`)
on gcc -O0 x86-64: a two-case `switch`, whose tests each jump to their case on
a match, re-rolls, and an if/else-if chain of three tests that each fall into
their own body keeps its tests. Under `option loweredswitch on` the two-case
switch prints as two tests and the three-test chain re-rolls. An installed
switch stays for the session, so the second pass reads a second copy of the
bytes. A three-case switch over a struct field that gcc loads into a register
once re-rolls too: the install reads the switch value from that load through
the register copy and its zero extension.

`kuna-loopcondhoist.xml` checks option `loopcondhoist` (default `on`) on
gcc x86-64. In three -O0 functions the loop's condition, read from the layout,
is the loop's exit: a `for` whose `break` carries two stores keeps them in the
break arm, a `while` with an early `return` prints as a `for` with the return
inside, and a `break` to the return after the loop stays a `break`. The -O2
build of the third prints as a `while` on its bottom test. Under `option
loopcondhoist off` the first two and the -O2 one are `while( true )` with the
condition inside (the third keeps its loop through `loopcontinue auto`); under
`auto` only the -O2 one is.

`kuna-latchjoin.xml` checks option `dedupitetail` (default `latch`) on gcc
-O2 x86-64. gcc copies `p = p->next; if (p) goto top;` into both paths of a
`while (p) { if (p->cls == cls) ...; p = p->next; }` loop; by default the
copies are joined and `visit` and `count` print that one loop, with `count`'s
sum (which gcc scheduled between the load and the test of one copy) left in the
`if`. A busy-wait inside a `for` loop shares the outer loop's header but has a
latch of its own, and stays a loop inside the loop. Under `option dedupitetail
on` the two copies print as a `while` loop inside a `do` loop.

`kuna-guardauto.xml` checks option `guardarm` (default `auto`) on two x86-64
functions. In a gcc -O0 `unget`, whose parameters live in frame slots, the
default prints the source's flat guard `if (c == -1 || ...) return -1;`, and
`option guardarm off` nests the body under the negated guard instead. A gcc
-O2 `backup` is not called unoptimized, so the default keeps its out-index arm
order, and only `option guardarm on` lets its layout put the else arm first.

`kuna-boolbranch.xml` checks option `boolbranch` on gcc -O0 x86-64: a `jle`
over `mov $1`/`mov $0` returns through a kept `if` (fold: `return 0 < x;`), an
if/else assignment of 1 and 0 keeps its branch, and a flag whose arm also
updates memory stays a flag. A `cmovg` and an `__builtin_add_overflow` flag
that is set by a jump and only tested still fold. `kuna-boolbranch-thumb.xml`
checks the same on Thumb: an `ite gt` block folds under both values, a `ble`
over two `movs` keeps its `if` under `keep`.

`kuna-boolret.xml` checks option `boolbranch` (default `return`) on gcc -O0
x86-64: a function that only returns `p != 0 && p[0] == '/'` prints one return
of the condition, and one whose guard returns 0 returns the condition's
negation, where `keep` prints a guard and two returns; a simple condition, two
guards and a guard after a store keep their `if`s. `kuna-boolret-thumb.xml`
checks the same on two hand-assembled Thumb functions in gcc -O2's layout, one
return per path.

`kuna-ptrchain.xml` checks option `truthycond` (default `ptrchain`) on gcc
-O0 x86-64: a pointer parameter tested as an operand of `&&` or `||` prints
`p != NULL`, or `p == NULL` for `!p`, in an `if` and in a returned chain, while
the integer, the call and the field `s->next` beside it, a stand-alone
`if (p)` and a chain that is a loop's condition stay truthy. Under `option
truthycond on` every test is truthy.

`kuna-ptrfromuse-faroffset.xml` checks option `ptrfromuse` on a parameter
read only at field offsets of 0x1000 or more, in an x86-64 shared object, the
same object stripped, an x86-64 non-PIE executable and an AArch64 shared
object (`tests/fixtures/field-offset-pointers/`). Reads at 0x1000 and at
0x48a0 make the parameter `void *`, as reads at 0xff8 always did, and the field
at 0x1000 in the x86-64 shared object, whose first function also sits at
0x1000, prints through the parameter instead of as `&near_leaf[a0]`. A
parameter that indexes a `.data`, `.bss` or `.text` table, named or not, stays
an integer. A fifth object has no data section at all, and its one function,
at 0x1000, reads its own record at that offset. Two more images
(`absolute.c`) pin the limits of the reading: a read of the absolute address
0x40020000 at an offset in the parameter, a byte table at the start of
`.rodata` indexed from a character (`letters[c - 'a']`), and a relocatable
object's extern table each keep an integer parameter.

`kuna-rangetest.xml` checks option `rangetest` (default `auto`) on gcc -O2
x86-64, which folds two comparisons of one value into one unsigned comparison
of a subtraction. By default an inside range on an `unsigned char` (95
values), an outside range on an `int` (30 values), a range test that is an
operand of `&&` and a returned one print as their two comparisons, while a
stand-alone test of ten values and a loop condition stay merged; `option
rangetest split` splits those two as well. `x >= -16 && x < 16`, whose range
wraps past zero, stays merged under every value, and `option rangetest merged`
prints every test as compiled. A split `||` that heads an `&&` chain nested as
the second clause of an `||` keeps its own parentheses, since that clause
prints none. `kuna-rangetest-thumb.xml` checks clang -O2
Thumb: the bound check that clang emits before a `tbb` switch table prints
before the recovered `switch` and keeps its one comparison under every value.

`kuna-sdivpow2.xml` checks signed divisions by a power of two whose
correction of a negative dividend gcc x86-64 emits as a conditional move
(`lea; test; cmovs; sar`): `x / 8` at -O0 and `x / 1024` of a `long` at -O2
print with no branch, as do `(bytes + 512) / 1024`, whose offset gcc folds
into both values, `x / 8 * 8`, which gcc rounds with `and $-8`, and a store of
`x / 256` through `mov %ah`. An add of 7 before a shift by 2 is not a division
and keeps its `if`. `kuna-sdivpow2-thumb.xml` checks gcc's two Thumb forms: an
`it lt; addlt` and an add taken out of line by `blt`.

`kuna-wsignload-a64.xml` checks that an AArch64 sign-extending byte or
halfword load into a `W` register (`ldrsb`, `ldrsh`, `ldursb`, `ldursh` in
the unsigned-offset, register, unscaled, pre- and post-index forms)
sign-extends to 32 bits and zero-extends to 64: a result read as 64 bits
prints `(uint4)p[2]`, not `(int8)p[2]`, and `ldursb w` is no longer a zero
extension. The `X`-destination forms still sign-extend to 64 bits.

`kuna-calleepreservesargs.xml` checks registers a leaf callee leaves alone:
an argument register set once before two calls of a leaf that reads it, and a
killed-by-call register kept across a leaf that only reads memory. Its control
leaves a stale register off a callee that never reads it.

`kuna-ppc32-computed-lr.xml` follows a local target stored in LR rather than
returning the input argument. Its CLI companion covers both byte orders,
conditional transfers, joins, stack reloads and ordinary ABI returns, and
compiles and executes the emitted C at O0/O2.

`kuna-stated-jump-placement.xml` checks stated computed-jump destinations
whose case labels the jump's value could never equal. A real-mode `jmp ax`
takes an offset in its code segment, a `jmp far [bx]` takes an offset in the
segment it loads, whether or not the destinations fit 16 bits, and DOS picks
the segment an MZ executable runs at. Each rejects its `override jumptable`
and keeps the unresolved jump.

`kuna-stated-jump-owners.xml` checks which stated destinations stay in the
function that holds the jump. A GCC-style `.cold` part of the jumping
function is a case, in a loaded ELF and in an XML image whose symbols are
named as written, and so is the entry of a caller-declared extent that
holds the site, beside a size-less inner symbol. A size-less exported symbol
holds no site, so a jump to its entry from an unsymbolized function is
refused, as is the entry of `ordinary` from `dispatch`. A later `function
bounds` that would make an installed destination another function's entry is
refused, and the installed set and its identical restatement stay in use.

`kuna-glibcvoidimport.xml` checks MIPS PIC wrappers that pass their pointer
to `free` without touching the argument register. Where the image's symbol
version binds `free` to glibc (`free@GLIBC_2.0` from `libc.so.6` or
`libpthread.so.0`), the N32, O32 and N64 wrappers forward the pointer, as they
do when a console declaration names the stub `free`. An unversioned `free`, one
versioned by another library or by `GLIBC_PRIVATE`, and a caller's
`void free(void)` declaration pass nothing, and a console declaration naming
the stub `exit` keeps today's `exit();`. The same glibc image keeps
`strlen(a0) + 1`, and `sincos`, which no table declares on MIPS, keeps today's
recovery.

`kuna-ppc64-plt-stubs.xml` checks PowerPC64 ELFv2 shared objects whose
wrappers call `free` and `strlen` through each linker stub form: GNU ld's
TOC stub with and without the `r2` save, its pc-relative stub for Power10
callers and its `pld` stub, lld's `addis` and `pld` stubs, big-endian
ELFv2, and GNU ld's stubs for an import that both TOC and pc-relative code
call, which pc-relative callers enter one word in. Every wrapper prints
`free(a0)` and `strlen(a0) + 1`. In the assembly
fixture, `outer` calls `free` through a real stub, then an exported function
shaped like a `pld` stub, which keeps its name, and a local one that loads a
GOT entry instead of a PLT slot, which stays `sub_`. The fixtures and their
recipe are in `tests/fixtures/ppc64-plt-stubs/`.

`kuna-ppc32picimports-aggressive.xml` loads a small PPC32 secure-PLT PIC
image twice. With the catalog defaults its PIC stub stays `sub_1800`; after
`mode aggressive`, which sets `ppc32picimports abi` and which `auto` selects
for files under 500 KiB, the same call prints as the import `extra`.

`kuna-mips-octeon-elf32-o64.xml` loads big-endian Octeon ELF32 images without a
target: one declaring O64, one declaring no ABI with GCC's `.mdebug.abiO64`
marker, and one declaring no ABI at all. Each now loads with the Octeon O64
model, so `bbit0` keeps the CRC's bit-15 test and `describe` passes its five
arguments, the fifth from the stack slot at SP + 32; they used to load as
generic MIPS32, which read `bbit0` as a COP2 load and stopped at `sd ra`. A
generic MIPS image with the same encoding keeps its COP2 load. A strict fix
with one pass. The fixtures and their recipe are in
`tests/fixtures/octeon-elf32/`.

`kuna-narrowed-call-input.xml` checks a call through a typed field whose
argument was narrowed to two bytes before the field's prototype was forced on
the call: the function decompiles and passes the sum widened to the parameter.

`kuna-expandload-opsize.xml` checks the final IR of a byte flag test that is
widened into a test of a declared record's 4-byte field: the AND and the compare
it feeds are 4 bytes wide, both for `cmp al` after `and eax` and for `testb`.

`kuna-calleefilledarg.xml` checks that a call to a stub that loads one of its
own argument registers (`adrp x1; ldr x1,[x1]; ... br x16`, the Objective-C
selector-stub shape) keeps the receiver, a stack address, a buffer and a count
the caller staged after that register, and that the stack object's loads after
the call are not folded. A stub that does not fill the register keeps the
one-argument positional rendering, and `option calleefilledarg off` restores it.

`kuna-calleefilledarg-ctramp.xml` checks that a C function that fills `rdi`
and `rsi` and tail-calls through a function pointer keeps the positional rule:
the `idiv` remainder a clang -O0 caller leaves in `rdx` is not passed to it.
Only an `objc_msgSend$<sel>` selector stub steps over the register it fills.

`kuna-pacmodifier-arg-a64e.xml` checks that the blended modifier an arm64e
caller builds in `x8` for a hidden `pacda` is not the next call's first
argument, with the modifier computed and folded to a constant. A tagged
address and a result buffer in `x8` are the controls.

`kuna-aif-slot-edges.xml` compares stock and section-relative AIF inventories
on stripped x86-64 and MIPS32 ELFs. The six-byte x86 grid covers padded
entries, section boundaries, overlapping decodes and terminal padding before
zero fill. The four-byte MIPS grid checks both direct and padding-adjusted
entry candidates with delay slots. The fixture languages reuse the stock
decoders and calling conventions through the stage corpus's `specs/` directory.

`ghangr-go-paniccheck-return.xml` checks that a returning Go panic helper
falls through to the terminal `runtime.gopanic` call. It also checks the
helper's conditional throw and shows the caller's sentinel with the known
no-return analysis disabled.

`ghangr-go-mcall-resumption.xml` checks the continuation after a Go scheduler
switch, independent terminal propagation through a lookalike wrapper, and
explicit function and call-site overrides.

`kuna-go-atomic-return.xml` checks exact AMD64 Go direct-return atomic additions,
including old-value, flag, branch, memory-effect and width refusals. The same
bytes retain C/Rust rendering and the non-Go source control.

`ghangr-go-abi0-model.xml` checks recovery of a Go stack-input leaf and the
arguments passed by both call orders. It also checks the disabled option,
an ordinary name lookalike, a wrapper with stack results, the signal
trampoline and two linked runtime functions with private or foreign exits.

`kuna-extrapop-function.xml` checks that `option extrapop 12 g` makes a caller
of the `ret 8` function g see both pushed arguments popped, so a later `[esp+4]`
reads the caller's own argument. A second caller reaches g through a register
and sees the same pop.

`kuna-varargsave.xml` checks option `varargsave` on gcc -O2 x86-64 variadic
functions. `report` and `scaled` hand their `va_list` to a callee, so the
register-save prologue folds: they print `...` after their named parameters
(`scaled` keeps its named `double`) and no `if (al)` block. `firstd` reads its
saved `xmm0` slot itself and `dsum` indexes the save area through a register,
so both keep the stores. The off run prints `report` with fourteen parameters.

`kuna-condexe-heritage.xml` checks that conditional execution removes a re-test
whose block holds unread flag writes once the register space has had a heritage
pass (`deadcode delay register 50` keeps those writes alive). Before the fix the
second `if (x)` stayed.

`kuna-looptest-order.xml` checks the ordering test of option `looptest` on four
x86-64 counted loops whose latch tests `!=` behind a guard that tests an order.
A count down to zero prints `while (n > 0)` and a byte walk to `&p[n]` with a
signed `n` prints `while (p < end)`, both without the guard. A step of 2 against
`n` (an odd `n` is never met) and an end pointer computed as `a + (n - 1) + 1` in
32 bits (far past `a` at `n == 0`) stay guarded do-while loops.
`option looptest binary` restores the guards and `!=` tests.

`kuna-looptest.xml` checks option `looptest` on six x86-64 loops laid out as
gcc -O2 rotates them. A pointer loop whose `n == 0` guard returns separately, a
counted loop wrapped in `if (n > 0)` and a list walk whose test reads the copy
of the next pointer print `while (c)` with the guard gone; `i = 0; do ...
while (i != 16)` prints `while` because its entry test folds to true. A guard
`n > 0` in front of a `!=` latch prints `while (i < n)` (the ordering test), and
a retry loop whose test reads a body value stays do-while. `option looptest
binary` restores every do-while and guard.

`kuna-looptest-zerotrip.xml` checks the guard drop of option `looptest` on gcc
-O2 x86-64 loops whose skip path has its own copy of the code after the loop.
A sum that returns `0.0` when skipped, a byte comparison that returns `0`, a
list count whose skip path calls `report(NULL, 0)` and a bucket walk whose
else arm sets its counter to 0 print `while (c)` without the guard, as does a
pair of counted loops sharing one counter register: each skip path is the exit
path with the carried variables at their start values. Controls whose skip
path returns `-1`, calls `report(head, 1)`, sets `-1`, or keeps a parameter the
loop would overwrite keep their guards. `option looptest binary` restores every
guard.

`kuna-looptest-entry.xml` checks the top tests of option `looptest` that need
no guard drop, on gcc -O2 x86-64 (one function at -O0). A polling loop with an
empty body prints `while (poll_busy()) {}`; counted loops whose guard reads a
field loaded in the `&&` block before it, or loaded before a store to the field
next to it, and one whose counter lives in the frame because its address is
taken, lose their guards; a walk from `s + 8` to `s + 0x48` tests at the top;
and a loop whose test recomputes the next row pointer prints the carried
pointer. A test of a body value, an unoptimized function's frame counter and a
walk to a pointer argument stay do-while loops. `option looptest binary`
restores every do-while.

`kuna-switchmerge.xml` checks option `switchmerge`: two AArch64 range-checked
jump tables on one selector, the second reached when the first's range check
fails, print as one `switch` whose holes take the one `default:` arm. In `f`
the first table's holes jump to the default code (`case 0x43: case 0x46:` and
an inner `default: goto` before); in `g` they go to the second table. The off
run keeps the nested form.

`kuna-switchmerge-values.xml` checks two rules for jump tables. Under option
`switchmerge`, an AArch64 table whose holes go where its `default:` code sends
them (through `x == 0x60` and `x == 0x70` tests that fail for the hole values)
prints no label for the holes: in `f` the holes jumped to a block the default
code also reaches (`case 0x43: case 0x46: goto`), in `g` they left the switch
(`break;`). The off run keeps `f`'s labels. `h` checks that `jumptablemax`
caps a guarded table's distinct arms, not its entries: an 1100-entry
range-checked table with four arms prints as a switch, not as `jump-as-call`.
`p` (clang -O2) keeps its hole labels: the arm before them ends in a jump to
the code after the switch that prints as nothing, so it runs into them.

`kuna-switchmerge-split.xml` checks two more `switchmerge` rules on AArch64
tables. In `k` a range test splits the selector between two range-checked
tables whose defaults both jump to the same code; it prints as one switch with
both tables' labels and one `default:`. In `m` the default arm starts with
`if (x == 0x60) return ...;`, which prints as `case 0x60:`. The off run keeps
the `if` forms.

`kuna-switcharmorder.xml` checks the order a switch prints its arms in and the
`default:` arm of a void function's tail switch. `force_str`'s `default:` arm
ends in a call to the no-return `die` and prints after the cases, with nothing
after it; it used to print between `case 0:` and `case 1:` and fall into
`case 1:`. `enable` ends with a switch the source gave no `default:`, so the
`default: return;` the duplicated epilogue leaves is not printed; `option
voidtailreturn off` keeps it.

`kuna-loweredswitch-small.xml` checks that small compare-lowered switches
re-roll by default. `nid_name` loads each case's string before the compare that
selects it and shares one `ret`; the switch is installed along the raw paths of
the compare tree and every case returns its own string. `enable`'s chain of
three tests with a body each re-rolls; the two-test chain in `two` stays an
`if` unless `option loweredswitch linear` is set, and `option loweredswitch
off` keeps the compiler's chain. `colon` (openssh `misc.c`) keeps its loop: re-rolled,
its arms continue the loop and the structurer lost the loop to gotos, so the
switch is withdrawn. The selector stays `int`, and `kind`'s labels over a
`char` print as character constants.

`kuna-switchcond.xml` checks that a folded switch is never printed where a
condition belongs. In gcc -O0 x86-64 `scan`, a jump-table switch inside a
loop, every case continues the loop except one that runs an inner loop first.
The region structurer folded the switch before that inner loop and printed
`if` on its own line followed by the whole switch a second time. The inner
loop now prints inside its case and the switch prints once.

`kuna-jumptableslots.xml` checks the slot bound on a guard-bounded table past
`jumptablemax`. gcc -O2 guards `x <= 0x28ff` and indexes a 41-entry table with
`x >> 8`, so the guard admits 0x2900 values over 41 slots. The table stays a
computed call instead of a switch with a `case` label for each value of `x`.

`kuna-switchsingle.xml` checks that a jump table whose four entries all name one
block becomes straight-line code with the "all go to same destination" warning
instead of a `switch` with only a `default` arm.

`ghangr-call-argument-extensions.xml` checks explicit and missing parameter
contracts. The CLI companion executes complete emitted C, Go and Rust callers
and callees, covering zero extension, sign extension, signedness and store canaries.

`ghangr-rust-boolean-expressions.xml` checks bitwise expressions over merged
Boolean locals. Its CLI companion executes complete emitted C and Rust functions
against independent result formulas, callback traces and store canaries at O0/O2.

`ghangr-scalar-call-results.xml` checks the no-record and explicit-prototype
controls for narrowed scalar calls. The CLI probes exercise callee-first recovery;
the native companion compiles and executes complete emitted C, Go and Rust callees
and callers across signed, unsigned, Boolean and full-width results.

`ghangr-rust-boolean-conversions.xml` checks byte-to-Boolean casts, masked values
and narrowed callback results. Its CLI companion compiles complete C and Rust
output under the same asserted prototypes and checks every byte value and call
counts. The original fixture is only decompiled.

`kuna-calleefloatreads.xml` checks calls to callees with no prototype that
read a float argument register: an x86-64 `xmm0` written after a `pxor`, an
ARM hard-float double forwarded or computed in `d0`, and a variadic callee
that must not receive a leftover `xmm0`.

`kuna-callproto.xml` checks a tail jump through a variadic function pointer with
nothing stated, with a call-site prototype, and with one parameter placed in a
register. Its CLI companion covers both decompile surfaces and the rejection of
an address that is not a call.

`kuna-e500-call-slots.xml` compares disabled and automatic call-overlap guards
for scalar arguments and results inside e500's wide registers, and decompiles a
conditional tail-call wrapper with `exclusivecallret` off and on. The CLI tests
cover both byte orders and execute the recovered wrappers.

`kuna-callwide.xml` loads the committed sealed MIPS32-BE ELF and checks the
four-byte OFF result against the proved eight-byte result and high-piece callers.
Its CLI companion executes the original bytes and untouched emitted C/header
under MIPS QEMU at O0/O2, including selected exports, workers and streaming.

`kuna-maxlinewidth.xml` checks that `option maxlinewidth 40` breaks a long
prototype and return statement at 40 columns, while the default prints each on
one line.

`kuna-stack-byte-copy.xml` checks pushed scratch bytes, partial reads and a
saved snapshot under the stack store guard. Its CLI companion compiles
and executes the exact C at O2 in default, stackviews and stackalias modes, and checks
that larger and data-dependent loops retain their control flow. The authored
assembly also includes a sequential overlapping-copy control.

`kuna-stackpointerfields.xml` checks that naming a logical frame object preserves
pointer-field dereferences. Its CLI companion executes the native fixture and the
unnamed and named C with GCC and Clang.

`kuna-stackversions.xml` checks conditional frame-pointer publication, a four-byte
seed and a real cached word. Its CLI companion executes an aliasing counterexample,
fast/growth branches and the cached return with GCC and Clang at O0/O2.

`kuna-stack-float-snapshot.xml` checks cached float lanes consumed by integer
bit operations and native float arithmetic. Its CLI companion executes both
Point lanes and a numeric conversion against the emitted C with GCC and Clang.

kuna-owned datatests derived from **real, open decompiler issues** (sourced from the
PHADE issue dataset), each demonstrating that the issue is fixable through the kuna
stage model (`docs/phases.md`): a named sub-stage decision point,
controlled by a durable P0 assertion, instead of a hardcoded heuristic.

These run with the same upstream harness as `tests/datatests/` but from a
separate directory so the upstream baseline (`docs/baseline.json`) is never touched:

```bash
make test-stages
# or, directly:
decompiler/target/release/kuna test --datatests --datatests-dir tests/stages \
    --baseline docs/baseline-stages.json
```

After adding a testcase, re-record the stages baseline (never `docs/baseline.json`):

```bash
decompiler/target/release/kuna test --datatests --datatests-dir tests/stages \
    --save-baseline docs/baseline-stages.json
```

## Testcase conventions

Each `.xml` is a standard `<decompilertest>` (bytechunks + console script +
`<stringmatch>` assertions) and must document in a leading comment:

- the **upstream issue** it derives from (`GH-<n>`),
- the **stage / sub-stage** (new model) that owns the decision,
- the **P0 assertion** exercised (e.g. an `option ...` command),
- which asserts prove the **bug reproduces** (default pipeline) and which prove the
  **stage-level intervention fixes it**.

A testcase that can only be made to pass by patching decompiler *behavior* (rather
than exposing a decision) is a negative result for the stage model and belongs in the
writeup, not here.

## Current testcases

| File | Issue | Stage / sub-stage | P0 assertion |
|---|---|---|---|
| `gh558-compareform.xml` | [GH-558](https://github.com/NationalSecurityAgency/ghidra/issues/558) | S3 simplification quiescence (`canonicalcompare`) → S9 presentation (`presentcompare`) | `option compareform canonical\|original` |
| `gh558-arraynotation.xml` | [GH-558](https://github.com/NationalSecurityAgency/ghidra/issues/558) (secondary) | S9 literal/format policy (pointer notation) | `option arraynotation on\|off` |
| `gh2786-unaryspacing.xml` | [GH-2786](https://github.com/NationalSecurityAgency/ghidra/issues/2786) | S9 surface (adjacent sign tokens) | default-flip (DIV-1, `docs/history.md`) |
| `gh8471-thumbfuncptr.xml` | [GH-8471](https://github.com/NationalSecurityAgency/ghidra/issues/8471) | S5 const-pointer (Thumb mode bit) | always on |
| `gh6930-inferfuncentry.xml` | [GH-6930](https://github.com/NationalSecurityAgency/ghidra/issues/6930) | S5 const-pointer (single-bit image base) | always on |
| `gh6990-returnpair.xml` | [GH-6990](https://github.com/NationalSecurityAgency/ghidra/issues/6990) | S4 trial finalization (return-register join) | `option returnpair pair\|single` |
| `gh1282-boolsignshift.xml` | [GH-1282](https://github.com/NationalSecurityAgency/ghidra/issues/1282) | S3 simplification quiescence (boolean sign mask) | always on |
| `gh7190-ovlesssimplify.xml` | [GH-7190](https://github.com/NationalSecurityAgency/ghidra/issues/7190) | S3 simplification quiescence (V850 OV-flag compare) | always on |
| `gh8817-v850indbranch.xml` | [GH-8817](https://github.com/NationalSecurityAgency/ghidra/issues/8817) | S2 flow classification → switch model | `option v850indirectbranch on\|off` |
| `gh8913-addcarrychain.xml` | [GH-8913](https://github.com/NationalSecurityAgency/ghidra/issues/8913) | S5 value recovery (carry-chain wide add) | always on |
| `gh9230-memsetseq.xml` | [GH-9230](https://github.com/NationalSecurityAgency/ghidra/issues/9230) | S5 const-sequence (constant-fill run) | `option memsetrecover on\|off` |
| `gh1537-simd-memset.xml` | [GH-1537](https://github.com/NationalSecurityAgency/ghidra/issues/1537) | S5 const-sequence (same sub-stage, global/SIMD form — pure routing, zero new code) | `option memsetrecover on\|off` |
| `gh9001-hcs12brn.xml` | [GH-9001](https://github.com/NationalSecurityAgency/ghidra/issues/9001) | S1 decode-table (HCS12 spec-fix, no option) | SLEIGH `:BRN rel8` consumes its operand |

| `gh1243-8051-addc.xml` | [GH-1243](https://github.com/NationalSecurityAgency/ghidra/issues/1243) | S2 pcode-lift (8051 spec-fix, no option) | SLEIGH ADDC carry-in computed in a 2-byte temp (`do_addc`) so the carry-out survives |

| `gh7418-x86-call-rel16-mask.xml` | [GH-7418](https://github.com/NationalSecurityAgency/ghidra/issues/7418) | S1 decode / S2 p-code lift (x86 spec-fix, no option) | SLEIGH `rel16` masks tempEIP to low 16 bits (32-bit code, 0x66 prefix; 16-bit code: kuna-x86-16-real-rel16.xml) |
| `kuna-x86-16-real-rel16.xml` | kuna [#1288](https://github.com/Noelo-Lab/kuna/issues/1288) (a DOS program loaded at segment 0x1000: the GH-7418 mask sent every real-mode `CALL rel16` above linear 0x10000 into the first 64 KiB, so discovery found 12 of 330 functions) | S1 decode / S2 p-code lift (x86-16 spec, no option) | x86-16.sla keeps the code segment `realCS` in the decode context: a near branch wraps IP inside its segment and passes `realCS` on, a far CALL states its target's, every CALL pushes IP in its own segment, and code outside the segment takes the linear sum |
| `kuna-x86-16-protected-rel16.xml` | kuna [#1288](https://github.com/Noelo-Lab/kuna/issues/1288) (companion: a 16-bit protected-mode branch lost its selector to the same mask) | S1 decode / S2 p-code lift (x86-16 spec, no option) | SLEIGH `rel16` keeps the selector and wraps the offset in 16-bit protected mode |

| `gh7139-x86-16-disp16-signed.xml` | [GH-7139](https://github.com/NationalSecurityAgency/ghidra/issues/7139) | S1 decode / operand rendering (x86 spec-fix, no option) | SLEIGH 16-bit disp16 (modrm mod=2) uses signed `simm16` |

| `gh8790-x86-prefetch-regform.xml` | [GH-8790](https://github.com/NationalSecurityAgency/ghidra/issues/8790) | S1 decode-table (x86 spec-fix, no option) | SLEIGH prefetch `0f 0d`/`0f 18` gain a mod=3 register-form variant |

| `gh1627-x86-senduipi-gatherpf.xml` | [Kuna #1627](https://github.com/Noelo-Lab/kuna/issues/1627) | S1 decode-table / S2 p-code lift (x86 spec-fix, no option) | `F3 0F C7 /6` reg-form is `SENDUIPI r64` (was RDRAND), with `UIRET`/`TESTUI`/`CLUI`/`STUI` in `uintr.sinc`; RIP-relative `0F 18 /7`,`/6` are `PREFETCHIT0/1`; `VGATHERPF*` use the VSIB operand tables so the index is a ZMM/YMM register and the user-op takes an address |

| `gh1628-x86-vmx-decode-flags.xml` | [Kuna #1628](https://github.com/Noelo-Lab/kuna/issues/1628) | S1 decode-table / S2 p-code lift (x86 spec-fix, no option) | `VMREAD`/`VMWRITE` decode without REX.W in 64-bit mode; every VMX user-op returns its status (0/1/2) and `vmxflags` sets ZF/CF from it, so `setna` after `vmlaunch` reads the VMX outcome; `VMPTRST` stores the VMCS pointer |

| `kuna-rex-before-prefix.xml` | Authored x86-64 prefix order | S1 decode-table (x86 spec-fix, no option) | a REX prefix that a legacy prefix follows is ignored, as the CPU ignores it (`f3 48 67 ab` is `rep stos` of dwords with a 32-bit address; `48 67 f3 48 a5` keeps the last REX.W), instead of the legacy prefix decoding as a bad opcode |

| `gh1674-subpiece-shadow-wide-const.xml` | [Kuna #1674](https://github.com/Noelo-Lab/kuna/issues/1674) | P6 merge (cover intersection, partial copy shadow; no option) | a constant truncated by 8 or more bytes reads 0 instead of shifting a 64-bit offset by 64 or more bits, so the function decompiles in a build with overflow checks |

| `gh9184-x86-fma-ymm256.xml` | [GH-9184](https://github.com/NationalSecurityAgency/ghidra/issues/9184) | S1/S2 decode + pcode-semantics (x86 spec-fix, no option) | SLEIGH YMM FMA uses 256-bit `local tmp:32` (was `tmp:16`) |

| `gh5897-arm-cpy-mov.xml` | [GH-5897](https://github.com/NationalSecurityAgency/ghidra/issues/5897) | S1 decode-table (ARM spec-fix, no option) | SLEIGH reg-to-reg move constructor renders UAL `mov`, not pre-UAL `cpy` |

| `gh8456-arm-mrs-banked.xml` | [GH-8456](https://github.com/NationalSecurityAgency/ghidra/issues/8456) | S1 decode-table (ARM spec-fix, no option) | SLEIGH MRS (Banked register) constructor added |

| `gh8391-arm-thumb-mrrc.xml` | [GH-8391](https://github.com/NationalSecurityAgency/ghidra/issues/8391) | S1 decode-table (ARM spec-fix, no option) | Thumb `mrrc`/`mcrr` un-guarded from `@ifndef CDE` so v8-m (CDE) decodes them |

| `gh7890-arm-vcvt.xml` | [GH-7890](https://github.com/NationalSecurityAgency/ghidra/issues/7890) | S2 p-code lift (ARM scalar VFP vcvt, spec-fix, no option) | SLEIGH `vcvt.*32` lifts to native `int2float` (no `VectorSignedToFloat` pseudo-op / fpscr read) |

| `gh1951-avr-rcallwrap.xml` | [GH-1951](https://github.com/NationalSecurityAgency/ghidra/issues/1951) | S1 decode-table / 02 P-code lift (avr8 SLEIGH spec-fix, no option) | SLEIGH rel12addr masks RJMP/RCALL target `& 0x0FFF` (4 KiW wrap) |

| `gh4788-v850-bcond-disp17.xml` | [GH-4788](https://github.com/NationalSecurityAgency/ghidra/issues/4788) | S1 decode-table (V850 spec-fix, no option) | SLEIGH adds the RH850 Bcond disp17 branch (`f9 07 49 fe` -> `bnc`, not `ld.hu`) |

| `gh6389-8085-undoc.xml` | [GH-6389](https://github.com/NationalSecurityAgency/ghidra/issues/6389) | S1 decode-table (8085 spec-fix, no option) | Nine undocumented 8085 instructions (DSUB/ARHL/RDEL/LDSI/SHLX/LHLX/RSTV/JNK/JK) now decode |

| `gh7451-riscvzfaflis.xml` | [GH-7451](https://github.com/NationalSecurityAgency/ghidra/issues/7451) | S1 decode-table (RISCV spec-fix, no option) | SLEIGH adds Zfa `fli.s` (constant table + `fli` pcodeop) |
| `gh831-riscv-fpmove.xml`, `gh831-riscv32-fpmove.xml` | [Kuna #831](https://github.com/Noelo-Lab/kuna/issues/831) | P2 p-code lift (RISCV spec-fix, no option) | `fmv.x.d`/`fmv.x.w` read the FP register itself, and `fmv.x.w`/`fcvt.w*` write the integer register (sign-extended on RV64) |
| `kuna-fcvtround-a64.xml` | [Kuna #1535](https://github.com/Noelo-Lab/kuna/issues/1535) | P2 p-code lift (AArch64 spec-fix, no option) + P5 typing of its rounding user ops (`p5_types/kuna_roundingops.rs`) | `fcvt{a,m,n,p}{s,u}` and `frint*` round in their own mode (`FLOAT_FLOOR`/`FLOAT_CEIL`/`FLOAT_ROUND`, typed `FLOAT_ROUNDEVEN`/`FLOAT_TRUNC`/`FLOAT_RINT`/`FLOAT_NEARBYINT`), so the issue's floor/ceil interpolation branch survives; `frint32/64*` zero the register bits above the result |
| `kuna-pacflow-a64.xml` | [Kuna #1621](https://github.com/Noelo-Lab/kuna/issues/1621) | P2 p-code lift (AArch64 spec-fix, no option) | with `ShowPAC=0` a `pac*`/`aut*`/`xpac*` that writes a general register lifts to a pass-through copy (`Rd = Rd`) instead of nothing, so a function whose only write to `x0` is the signing returns its pointer instead of decompiling as `void`; the `x30`/`sp` and branch forms stay empty |
| `kuna-rustversion-commit.xml` | [Kuna #1639](https://github.com/Noelo-Lab/kuna/issues/1639) | P0 knowledge (loader image facts, Rust compiler version; no option) | a Rust image without a `.comment` record takes its stable release from the commit in its `/rustc/<commit>/library/...` source paths, so the 1.93 `rustfmt` argument recovery still applies |
| `kuna-acleintrinsics.xml` | [Kuna #1626](https://github.com/Noelo-Lab/kuna/issues/1626) | P9 naming-policy + P2 context paint (`option acleintrinsics`, AArch64 only, default on) | pass 1 (`off`): `irg`/`addg` return the argument unchanged, `stg` leaves nothing, barriers and hints print as `DataMemoryBarrier(2,3)` / `WaitForInterrupt()`; pass 2 (default): `__arm_mte_create_random_tag(a0,0)` produces the returned pointer, `__dmb(11)`, `__dsb(15)`, `__isb(15)`, `__wfi()`, `__arm_mte_increment_tag`, `__arm_mte_exclude_tag`, `__arm_mte_get_tag`, `__arm_mte_set_tag`; a tag load or store of another granule keeps `LoadPtrTag` / `StorePtrTag` |

| `gh6904-ppc-e500-concat44.xml` | [GH-6904](https://github.com/NationalSecurityAgency/ghidra/issues/6904) | P-code lift (PowerPC e500 spec-fix, no option) | `or`/`or.` (`mr`/`mr.`) zero-extend their 32-bit result on the 64-bit-register e500, killing spurious CONCAT44 at function boundaries (partial) |

| `gh1276-flagcompare-boolsign.xml` | [GH-1276](https://github.com/NationalSecurityAgency/ghidra/issues/1276) | S3 simplification quiescence (boolean-into-sign-bit compare) | always on |

| `gh8777-flagcompare-sborrow.xml` | [GH-8777](https://github.com/NationalSecurityAgency/ghidra/issues/8777) | S3 simplification quiescence (N==V SBORROW signed compare) | always on |

| `gh9191-switchmodbound.xml` | [GH-9191](https://github.com/NationalSecurityAgency/ghidra/issues/9191) | S2 switch model (modulo/and-mask LOAD-table index bound) | `option switchmodbound on\|off` |

| `re-constselectjump.xml` | RE-friction need `conditional-indirect-branches-hide` | S2 switch model (destination selected between constant addresses by a conditional move) | `option constselectjump on\|off` |
| `kuna-entryretdispatch.xml` | RE-friction need `entry-point-ret-dispatch` | S2 flow classification (proven entry `push continuation; push target; ret` links become calls) | always on (P2 `flow-classification`, `kuna_entryretdispatch.rs`) |
| `kuna-extent-fallthrough.xml` | [Kuna #850](https://github.com/Noelo-Lab/kuna/issues/850) | S2 flow classification (caller-declared extent vs discovered interior entry) | `option funcboundflow on\|off` |
| `kuna-extent-direct-branch.xml` | [Kuna #850](https://github.com/Noelo-Lab/kuna/issues/850) | S2 flow classification (in-extent direct and recovered indirect branches vs tail-call inference; out-of-extent veneer control) | `option tailcalljump on\|off` |
| `kuna-conditional-tail.xml` | Authored bounded conditional forwarding | P2 flow classification (a known out-of-extent entry is called only on the taken path) | `option tailcalljump on\|direct\|off` |
| `kuna-swiftdemangle.xml` | Swift name demangling | P1 environment-binding (the loader's symbol-name demangle seam + the ObjC pass's class-name rendering) | always on |
| `kuna-itaniumvfunc.xml` | Authored Itanium slot naming | P1 external-refinement (a virtual slot is named `<C>::vfunc_<i>`, or `<C>::vfunc_for_<Base>_<i>` in a secondary sub-vtable, never after its vtable) | `option itaniumrtti off\|on` |
| `kuna-swiftmetadata.xml` | Authored Swift runtime-metadata recovery | P1 external-refinement (the `swift5_*` reflection-section walk names accessors, vtable slots, witnesses and descriptors) | `option swiftmetadata on\|off` |
| `kuna-itaniumctors.xml` | Authored Itanium constructor/destructor naming | P1 external-refinement (a function storing a class's vptr through `this` is `C::C`, or `C::~C` in the class's own vtable; a class without a virtual destructor, an inlining factory and optimized helpers a constructor or destructor was inlined into are left alone) | `option itaniumrtti off\|on` with `option listing on` |
| `kuna-swifttypes.xml` | Authored Swift type layouts | P1 external-refinement (struct and class layouts from static type metadata; the `self` of getters pinned to the `swiftself` register) | `option swifttypes on\|off` |
| `kuna-swifticf.xml` | Authored Swift names on folded code | P1 external-refinement (a function several metadata records claim is named after what they agree on, or not at all, and takes no type's `self`) | `option swiftmetadata on\|off` |
| `kuna-swiftcall.xml` | Swift calling convention | P1 external-refinement (methods, initializers, accessors and witnesses bound to the `__swiftcall` model derived from the compiler spec's default with a typed `self` hint; P4 throwing calls hand back the `swifterror` register) | `option swiftcall on\|off` |
| `kuna-thunkentry.xml` | [Kuna #992](https://github.com/Noelo-Lab/kuna/issues/992) | P1 code/data partition (the target of a walk entry that is one direct `jmp` becomes a function; MSVC `/INCREMENTAL` thunk table) | always on |
| `kuna-unwindstarts.xml` | Measured: a 249 MB third-party iOS app with no `LC_FUNCTION_STARTS` yields 23 real `__text` functions of the 204,800 its `__unwind_info` records | P1 code/data partition (a stripped Mach-O's `__TEXT,__unwind_info` second-level pages seed the function inventory) | `option unwindstarts on\|off` |
| `kuna-aifbracket.xml` | [Kuna #299](https://github.com/Noelo-Lab/kuna/issues/299) | P1 code/data partition (an AIF gap candidate whose own flow rejoins the known function around it is refused without boundary evidence) | always on with `option aif on` |
| `kuna-aifnoppad.xml` | [Kuna #299](https://github.com/Noelo-Lab/kuna/issues/299) | P1 code/data partition (no AIF gap entry starts on nop padding: padding in front of a known function is not probed, and an accept behind padding moves to the code after it) | always on with `option aif on` |
| `kuna-condcall.xml` | [Kuna #299](https://github.com/Noelo-Lab/kuna/issues/299) | P1 code/data partition (a predicated call's skip to the next instruction is its fall-through, not a second call target, so the next instruction is not a function and the callee is not no-return) | default correctness fix (no option) |
| `kuna-thumbswitch.xml` | [Kuna #299](https://github.com/Noelo-Lab/kuna/issues/299) | P1 code/data partition (the Listing walk reads a range-checked Thumb `tbb`/`tbh` offset table and decodes the case bodies as the dispatcher's own code, so no gap entry lands on a case) | always on |
| `kuna-x86listingswitch.xml` | [Kuna #299](https://github.com/Noelo-Lab/kuna/issues/299) | P1 code/data partition (a locally proved i386 switch decodes exactly four cases before the gap walk; the independent function between cases remains and a case's real callee is recovered) | `option x86listingswitch off\|on` |
| `kuna-aifpoolslide.xml` | [Kuna #299](https://github.com/Noelo-Lab/kuna/issues/299) | P1 code/data partition (an AIF gap candidate on a word a PC-relative load reads is pool data: its accept moves to the code after the pool) | `option aifpoolslide on\|off` |
| `kuna-pushimmediateret.xml` | RE-friction duplicate needs `entry-point-push-return` + `push-immediate-ret-transfer` | S2 flow classification (a proven one-store `push immediate; ret` becomes a terminal branch; no target function is synthesized) | always on (P2 `flow-classification`, `kuna_pushimmediateret.rs`) |
| `kuna-subcommuteshift.xml` | RE-friction need `cancelling-byte-arithmetic-splits` | S3 simplification quiescence (exact modulo-256 multiply/shift cancellation) | always on |
| `kuna-wideslice.xml` | clang x86-64 `-O3` byte-sum regression | P3 SSA reduction (push demanded slices through 9..15-byte `PIECE`, byte shifts, and bitwise operations; the artificial aggregates heritage and the rule pool leave behind, without recognizing the source algorithm) | always on |
| `kuna-devectorize.xml` | clang x86-64 `-O3` byte-sum regression (follow-up to `kuna-wideslice.xml`) | P3 SSA reduction (a proven vectorized integer reduction: guard, stride loop with one accumulator per lane, fold and scalar remainder loop, re-rolled into its scalar loop by removing the guard's vector edge) | `option devectorize on\|off` |
| `kuna-devectorize-effects.xml` | hand-written x86-64 byte sums whose exits disagree on a global store | P3 SSA reduction (`devectorize` refuses a reduction whose arm, remainder loop or exits write global storage) | `option devectorize off\|on` |
| `re-checker-stack-aggregate.xml` | RE-friction need `checker-uses-stack-aggregate` | P9 declaration emission (constant address-reference highs are not whole storage representatives) | default correctness fix (no option) |
| `ghdec-unsigned-byte-vm-selector.xml` | RE-friction need `unsigned-byte-vm-selector` | P2 switch model (range-comparison signedness carried through lowered-switch recovery) | always on (P2 `switch-model`, part of `loweredswitch`: `kuna_loweredswitchlabels.rs`) |
| `kuna-msvcstrappend.xml` | GH-468 finding 6 (inlined MSVC `std::string` appends) | P2 flow classification (a proven capacity diamond collapses to one `push_back`/`append` call; 29 one-defect near misses stay unrolled) | `option msvcstrappend on\|off` |
| `kuna-loweredswitchvalue.xml` | GH-468 finding 4 (scrutinee soundness) | P2 switch model (a re-rolled lowered switch dispatches on the compared value, or is withdrawn) | always on (P2 `switch-model`, part of `loweredswitch`: `kuna_loweredswitchvalue.rs`) |
| `kuna-loweredswitchexact.xml` | GH-468 finding 4 (install soundness) | P2 switch model (every case value keeps its label; a switch that differs from its compare tree is declined) | always on (P2 `switch-model`, part of `loweredswitch`: `kuna_loweredswitchexact.rs`) |
| `kuna-loweredswitchheads.xml` | GH-468 finding 4 | P2 switch model (a cascade behind a later head on the switch variable is re-rolled) | always on (P2 `switch-model`, part of `loweredswitch`: `kuna_loweredswitchheads.rs`) |
| `kuna-switchguardreload.xml` | [Kuna #1028](https://github.com/Noelo-Lab/kuna/issues/1028) | P2 switch model (a guard on a byte in memory bounds the index read by a second load of the same address; the adjacent table is not read) | default correctness fix (no option) |
| `kuna-lanedivide-terminator.xml` | [Kuna #1157](https://github.com/Noelo-Lab/kuna/issues/1157) | P3 laned-register split (a SUBPIECE narrower than a lane, read from a piece holding only upper lanes, terminates in the lane its byte offset reaches from the start of the whole register; upstream GP-7434) | default correctness fix (no option) |
| `kuna-duplabels.xml` | kuna (a block cloned by return splitting printed `label_<addr>:` once per copy in liblua5.3, libsqlite3, e2fsck, bash, ssh) | P9 emission (label naming: copies of one block that are all goto targets print `label_<addr>`, `label_<addr>_2`, ..., and each goto names its own copy) | default correctness fix (no option) |
| `kuna-swiftstrobj.xml` | kuna (a Swift string literal's object word, the literal's address minus 0x20 tagged in bit 63, printed as the text of the literal stored 32 bytes earlier) | P9 emission, const-pointer typing (`ActionConstantPtr` leaves a constant as a number when every use ORs in bit 63 and the bytes 32 past it open a large literal) | default correctness fix (no option) |
| `kuna-retcopyclone.xml` | [Kuna #1355](https://github.com/Noelo-Lab/kuna/issues/1355) (a store before a `cmp; cmov; ret`, or before a shared `leave; ret`, printed a second time on one return path) | P8 structure (a return block cloned per predecessor keeps each global's `return_copy` guard flag, and a phi on address-tied storage leaves no `g = g` copy, so neither is rewritten into a second store; the order the split prints stores its phis absorbed) | default correctness fix (no option); the clone's global phis print in address order |
| `kuna-switchunrolledsense.xml` | [Kuna #1028](https://github.com/Noelo-Lab/kuna/issues/1028) | P2 switch model (copies of the bound check on each predecessor of the dispatch, with opposite branch senses, bound the merged index) | `option switchmultipred on\|off` |
| `kuna-switchcond.xml` | kuna-owned (bash `man2html` `scan_format`, invalid C) | P8 region structurer (a folded switch keeps one successor; no statement in a condition slot) | default correctness fix (no option) |

| `gh8017-stackprobeloop.xml` | [GH-8017](https://github.com/NationalSecurityAgency/ghidra/issues/8017) | S2 stack-pointer normalization (stack-probe loop) | always on (P2 `stack-pointer-normalization`, `kuna_stackprobeloop.rs`) |

| `gh8467-dynamichashmax.xml` | [GH-8467](https://github.com/NationalSecurityAgency/ghidra/issues/8467) | S6 alias-facets (dynamic-symbol DynamicHash collision budget) | always on |

| `gh8500-stackalias.xml` | [GH-8500](https://github.com/NationalSecurityAgency/ghidra/issues/8500) | S6 alias-facets / heritage ordering (store-through stack-pointer-alias) | `option stackalias on\|off` |

| `gh8724-arraystride.xml` | [GH-8724](https://github.com/NationalSecurityAgency/ghidra/issues/8724) | S3 simplification quiescence (strided-induction array index → counter*stride) | `option arraystride on\|off` |

| `gh6882-sparcstructret.xml` | [GH-6882](https://github.com/NationalSecurityAgency/ghidra/issues/6882) | S2 flow classification (post-call `unimp` terminator) | `option sparcstructret on\|off` |

| `gh9203-condexe-loopcopy.xml` | [GH-9203](https://github.com/NationalSecurityAgency/ghidra/issues/9203) | S3 simplification quiescence (ActionConditionalConst const-COPY block placement) | always on |

| `gh9218-inputvarnodeadjust.xml` | [GH-9218](https://github.com/NationalSecurityAgency/ghidra/issues/9218) | S6 storage reconciliation (`stack-frame-layout`, overlapping input varnodes) | always on |
| `ghangr-ite-region-converter-missing-5db28e.xml` | angr `test_ite_region_converter_missing_break_statement` (StackCanarySimplifier) | S7 region recovery (`edge-virtualization`, strip the -fstack-protector canary epilogue so the shared-return goto is eliminated) | `option stackguard on\|off` |
| `ghdec-whiledo-complex.xml` | decbench `O0-iproute2-ip-lookup_flag_data_by_name` (invalid C: statement inside `while(...)` parens) | S8 structure recovery (whileDo overflow-syntax decision; `FlowBlock::isComplex` virtual-dispatch parity, porting-divergence correctness fix, no option) | default (upstream `isComplex`: BlockList/BlockIf unconditionally complex) |
| `ghdec-spacebase-unnamed.xml` | decbench `O2-noinline-mydoom-mydoom-scan_textfile`, NOVEL pool (invalid C: internal `PTRSUB(ESP,8)` p-code operator and raw register in the body) | P9 emission, leaf naming (symbol-less SPACEBASE `PTRSUB` takes the `PrintC::pushUnnamedLocation` stack-location leaf, correctness fix, no option) | default (DIV-46: `&Stack00000008`, never the functional render) |
| `ghdec-opcode-seam.xml` | decbench cases-missing pool (coreutils `timeout`/`split` main, bash, tar, grep, openssh: no output at all, one error record per function) | P3 simplification quiescence (the `glb->inst[opc]` opcode -> TypeOp seam is now total, resolved through `p5_types/typeop.rs (seam_type_op_for)`; correctness fix, no option) | default (the seam answers for every op-code instead of `panic!`-ing on an unlisted one) |
| `ghdec-isamode-inject.xml` | decbench `O2-noinline-betaflight-...-accDetect` + `O0-libopencm3-sdram-main` (ARM `setISAMode(1);` bookkeeping user-op emitted as a C statement inside jump-table-reachable blocks) | S2 lift/flow (`<callotherfixup>` injection drain in `generate_ops_with_jumptables`; porting-divergence correctness fix, no option) | default (upstream `flow.cc:819` drains `injectlist` on every jump-table round) |
| `ghdec-realtypes-pointee.xml` | decbench `O2-noinline-libacl-libacl.so.1.1-set_acl_fd`, NOVEL pool (invalid C: `void *a3` declared alongside `a3[1]`, which the expression built for byte offset 8; a 4-byte store rendered width-less as `*(void *)`) | P9 emission, type rendering (the `realtypes` relabel applies the scalar size table to a `TYPE_UNKNOWN` pointee, so `void` is only the residual-size fallback; correctness fix, no option) | default (DIV-47: `unsigned long *a3`); pass 2 pins the untouched `option realtypes off` path |
| `ghdec-stalejumptable.xml` | decbench cases-missing pool (coreutils `comm`/`join`/`uniq` main at -O0 and -O2: no output at all, a `JumpTable` outliving the BRANCHIND swept out from under it) | P2 switch model (`blockRemoveInternal` drops a removed BRANCHIND's jump table, and the `loweredswitch` install declines when its rewiring would strand a recovered table; correctness fix, no option) | default (`loweredswitch` on; pass 2 sets it off as the reference rendering) |
| `ghdec-finalorder-entryfirst.xml` | decbench `O0-libopencm3-sdram-main`, NOVEL pool (function entry emitted after an unconditional `goto`, i.e. as unreachable code; witness carried is diffutils `lines_differ` -O2) | P8 structure recovery, final component ordering (`BlockGraph::orderBlocks` port: entry component first, RETURN-terminated last, else by index; correctness fix, no option) | default (`ActionFinalStructure` sorts the root component list) |
| `ghdec-symbol-keyed-local-decls.xml` | decbench `O2-noinline-betaflight-betaflight_STM32F405-applyLedFixedLayers`, NOVEL pool (invalid C: one 4-byte stack slot declared twice under one name, `unsigned int v12;` and `uint4 v12;`, both `// stack - 0x24`) | P9 emission, declaration emission (the scalar declaration collapse is keyed on the ScopeLocal Symbol, as upstream `emitScopeVarDecls` is, instead of on the rendered line; correctness fix inside the default-on `dedupvardecls`, no new option) | default (DIV-52: one declaration per symbol, at least as wide as every access); pass 2 (`option dedupvardecls off`) confirms the unconditional unique-overlap-owner invariant still prevents a same-name shadow |
| `ghdec-subright.xml` | decbench `novel-concat-subpiece-soup`, NOVEL pool (invalid C: every SUBPIECE with a nonzero byte offset printed as the raw p-code operator `SUB41`/`SUB81`/`SUB84`, an undeclared identifier, where upstream and the source both have a shift) | P3 simplification quiescence, cleanup pool (`RuleSubRight`'s rewrite tail `sub(V,c) => sub(V>>c*8,0)` was a port stub returning 0; correctness fix restoring upstream, no option) | default (the fix); pass 2 (`option togglerule subright off`, the rule's own name in the action pool) pins the pre-fix rendering |
| `ghdec-returncopysplit.xml` | decbench `O2-noinline-iproute2-ip-netns_add`, NOVEL pool (a block of per-byte assignments **storing into a `.rodata` string literal** on a return path where the binary performs no store at all) | P3 simplification quiescence, cleanup pool (`SplitDatatype::testCopyConstraints` declines a copy split whose OUTPUT varnode is read-only; a cloned heritage return-copy carries neither `return_copy` nor its own address, so upstream's two guards miss it; correctness fix, no option) | default (the fix, DIV-55); pass 2 (`option togglerule splitcopy off`, the rule's own name in the action pool) pins the unsplit rendering |
| `ghdec-peimportcall.xml` | decbench `O0-mydoom-mydoom-mmsender_th`, ida pool (a Windows `call dword ptr [IAT slot]` kept no name, no prototype and no no-return flow effect, so a function ending in `ExitThread` absorbed the whole of the next function; IDA/Ghidra/angr all GED 0, kuna 37) | P1 program-prep, external-refinement (`kuna-analysis` `loader/kuna_peimportcall.rs` paints `Varnode::externref` over the IAT slot ranges, and `Architecture::query_function` carries the resolved callee's no-return flag onto the prototype `ActionDeindirect` merges) | always on |
| `ghdec-undefname.xml` | decbench `O2-tar-tar-xstrcat` (filed as `O2-noinline-iproute2-ip`), NOVEL pool (invalid C: an identifier containing `$$`, and one stack Symbol split across two identifiers so the body reads a name nothing writes) | P6 variable & storage model, local naming (the `ActionNameVars` tail wires upstream's `assignDefaultNames` catch-all and re-reads the Symbol names the HighVariable caches froze too early; correctness fix, no option) | default (DIV-58: no `$$undef` placeholder survives the naming pass); pass 2 (`option dedupvardecls off`) pins that neither witness depends on the P9 declaration collapse |
| `gh180-implied-cover-scarry.xml` | kuna [GH-180](https://github.com/Noelo-Lab/kuna/issues/180) (grep -O2 `nlscan`: `v3 = v3 + v1; if (SCARRY8(v3,v1))` tested the POST-add value; the pre-add SCARRY input and the ADD output merged into one HighVariable) | P6 variable & storage model, explicit/implied marking + merge legality (`ActionMarkImplied` goes through `Merge::mark_implied` so implied-marking dirties the operand Covers and the pre-add/post-add merge is illegal; correctness fix restoring upstream, no option, shipped with DIV-47) | default, single pass: the SCARRY8/CARRY8 guards keep their PRE-add operands and the collapsed self-update-then-test rendering does not appear |
| `gh181-snipreads-indirect.xml` | kuna [GH-181](https://github.com/Noelo-Lab/kuna/issues/181) (libselinux -O2 `lsetfilecon_raw`: the out-parameter copy `v1 = v4;` printed ABOVE the `lgetfilecon_raw(a0,&v4)` call that fills the slot, so `freecon(v1)` freed the pre-call NULL) | P6 variable & storage model, cover-trim insertion + foldcallret order-safety (`Merge::snip_reads` places an INDIRECT-defined value's trim COPY after the op CAUSING the effect per upstream merge.cc:461, and `foldcallret` refuses to sink a call past a read of its own INDIRECT output; correctness fixes, no option) | default pass proves both fixes; pass 2 (`option foldcallret off`) isolates the p-code-level snipReads fix |
| `kuna-callretmulti.xml` | RE-friction need `call-result-folding-duplicates` (one call result is narrowed and consumed twice, either in one block or across a loop; the old implied-expression multiplier printed the call at both sinks) | P6 variable & storage model, explicit/implied marking (`foldcallret` keeps a derived fan-out explicit, preserving one evaluation) | pass 1 (`option foldcallret off`) pins the conservative form; pass 2 (`option foldcallret on`) proves the shipped fold also evaluates each call once |
| `kuna-foldcallret-shortcircuit.xml` | kuna [GH-684](https://github.com/Noelo-Lab/kuna/issues/684) (a call combined with a comparison by a non-short-circuit `&` was folded into the right-hand operand of the `&&`/`||` the printer emits, so the printed C skipped a call the binary always makes) | P6 variable & storage model, explicit/implied marking (`foldcallret` declines a fold whose value reaches input 1 of a `BOOL_AND`/`BOOL_OR` on its way to the print point; correctness fix, no option) | default only: gcc -O0 and clang -O2 keep the spill, clang -O0 (call on the left) still folds |
| `kuna-msvcstackguard.xml` (long chain extension) | RE-friction pointer-return regression: a 271-call cookie chain lost the factory pointer held in RAX after the checker | P7 cookie provenance -> P0 exact-site restart -> P3 return-storage preservation | `msvcstackguard off/on`; exact preservation keeps the checker, and removal and preservation both keep the factory result |
| `kuna-msvcstackguard-return.xml` | RE-friction need `msvc-cookie-removal-changes` (P7 removal exposed one constant return at both arms of a shared `/GS` epilogue) | P7 exact cookie recognition -> P0 restart seed -> P3 call guard replay | `msvcstackguard on|off`; the checker tail reaches a nested ordinary call before fast-fail |
| `kuna-tiedphitrim.xml` | kuna [GH-468](https://github.com/Noelo-Lab/kuna/issues/468) follow-up (MSVC `/O2` `ab_o2.exe main`: a byte-sum loop over a stack buffer printed `while (v10[0]) { ... v10[0] = v10[v6]; }`, a store into the buffer the binary never performs; the gcc and clang-12 `-O2` builds print the same store as `v2 = *v1;`) | P6 variable & storage model, forced merge (`Merge::merge_op` trims a loop head's direct read of an aliased location, and `merge_opcode`/`merge_adjacent` do not fold the loop variable back) | `option tiedphitrim on\|off`, default on (DIV-182): pass 1 (default) pins the entering-edge copy for the MSVC, gcc and clang loops and gcc `outparam_walk`, pass 2 (off) pins the stores; a live store in a loop, a store after a join, an if/else join, a computed loop value, and gcc `xa11`/`xa13` (an in-loop value stored into the buffer after the loop) print as upstream does; gcc `t8`/`t1`/`t1b` pin the exit byte stored into a neighbouring slot only after the loop |
| `kuna-stackguard-tailcall.xml` | kuna [GH-866](https://github.com/Noelo-Lab/kuna/issues/866) (gcc `-O2`/`-Os` `-fstack-protector`: a canary checked right before a tail call kept `v1 = *(unsigned long *)(fs_offset + 0x28); ... v4 = v1;`) | P7 regions, stackguard (`ActionStripStackGuard` also resolves the canary slot forward from the `FS_OFFSET + 0x28` LOADs the compare reads) | `option stackguard on\|off`: pass 1 (default) pins t2/t2s/k2/w3 with no canary load and no `fs_offset`, pass 2 (off) keeps every check; t3 (check after a call, GH-183 shape) is the control; h6/h7 (no canary, a struct-field compare at offset 0x28) keep their stack stores and call arguments in both passes |
| `kuna-declaring-double-score-return.xml` | RE-friction need `declaring-double-score-return` (locking a later `/GS` checker void disconnected an earlier `double` accumulation from XMM0) | P7 exact cookie recognition -> P0 restart seed -> P3 locked-void exact-slice return-storage preservation | always on; loop-carried cookie proof, an XMM0 write before an unresolved nested call, and a real 16-byte XMM overlap whose low output lane survives while its upper scratch lane stays killed |
| `kuna-declaring-double-score-return.xml` (refined cells) | RE-friction need `explicit-double-score-prototype` (low-dword traffic refined one declared `double` return into two 4-byte cells across the `/GS` checker) | P3 exact-cookie unjustified-cell preservation, gated by the caller's locked declared output | inferred/unlocked caller remains truncated; upper XMM scratch stays killed |
| `gh182-loadguardrange.xml` | kuna [GH-182](https://github.com/Noelo-Lab/kuna/issues/182) (betaflight `cliServo`: `int16_t arguments[6]` written by one indexed `strh` printed as `int2 v5 [4]` subscripted at [0,5], with elements 4/5 split off as never-assigned scalars that are range-checked and stored into the servo config) | P3 SSA guards + P5 value sets + P6 variable & storage model (the LoadGuard/StoreGuard ValueSet range refinement: `Heritage::analyzeNewLoadGuards` runs `ValueSetSolver` over each new indexed-stack guard pointer, `LoadGuard::establishRange`/`finalizeRange` lock the real `[min,max,step]`, and `MapState::addGuard` turns a locked guard into the open hint bound `highind = ((max-min)+1)/step - 1`) | always on (DIV-77): asserts one in-bounds `int2 v7 [8]` and no split-off never-assigned tail scalars |
| `gh468-nulterminator.xml` | kuna [GH-468](https://github.com/Noelo-Lab/kuna/issues/468) (MSVC `/Od` `canary_check`: `char buf[64]` terminated by `buf[sizeof(buf) - 1] = 0` after `strncpy(buf, in, 63)` printed as `uint1 v3 [63]` plus a separate never-read `char v8 = 0` at stack - 0x19) | P6 variable & storage model, stack-frame layout (`ScopeLocal::restructure` closes an open integer array exactly one element after an unread zero store at its end; nothing after that element joins it) | `option nulterminator on\|off`: on = `uint1 v [64]` with `v[0x3f] = 0`, off = the `[63]` array beside the split-off char. Hand-assembled neighbours pin the extent: `pad_after` and `wide_str` gain only the terminator (`[34]` not `[40]`, `uint2 [15]`); a second zero two elements past the new end, a byte after alignment padding, a zeroed local read later, a slot read directly, a non-zero store, a zeroed int after an `int4` array, a two-byte zero after a `char` array and an address-taken byte all stay separate in both passes |
| `gh510-phiopflags.xml` | kuna [GH-510](https://github.com/Noelo-Lab/kuna/issues/510) (an 18 MB Windows PE: six functions produced no output at all, each one a `LOSS-131` record carrying `index out of bounds: the len is N but the index is N`) | P3 SSA construction, the op-property triple heritage caches on the ops it creates (`glb->inst[opc]`): the phi carries `special|marker|nocollapse` as upstream's `TypeOpMulti` does, so the raw-stack-pointer `PTRSUB(sp,#0)` placeholder is not spliced into a block's leading phi run and `ConditionalJoin::cutDownMultiequals` cannot stop short of a phi it has to cut; correctness fix, no option | default, single pass: 0 of 5 assertions pass pre-fix (the error stub `void scan_list(void)`), 5/5 post-fix |
| `kuna-floatgrouping.xml` | kuna [GH-641](https://github.com/Noelo-Lab/kuna/issues/641) (two x86-64 functions multiplying the same three floats in opposite orders decompiled to the same C, because `CPUI_FLOAT_MULT`/`CPUI_FLOAT_ADD` pushed the associative integer `*`/`+` tokens and the printer dropped the parentheses around a same-token right operand; one of the two renderings computed a different value than the function it came from) | P9 emit, the operator-token table and the `parentheses` binary case: the two float opcodes get their own singletons carrying `left_to_right_only`, which leaves a same-token left operand bare and parenthesises a right one; correctness fix, no option | default, single pass: the right-grouped product and sum keep `a * (b * c)` / `a + (b + c)` and the flat chain matches exactly one function of each pair, where pre-fix it matched both |
| `kuna-arm-register-shift.xml` | kuna [GH-849](https://github.com/Noelo-Lab/kuna/issues/849) (a variable ARM LSL can emit undefined C shifts for low-eight-bit counts 32 through 255) | P9 C emission, a guarded `CPUI_INT_LEFT` with unsigned shiftee, restricted to repeatable ARM operands; option `armshiftguard` (its counts are unbounded and no 64-bit shift's complement) | default: the plain shift and an in-place `<<=`; `needed`: preserves `count & 0xff`, checks `< 32`, emits an unsigned shift plus zero fallback, and avoids compound assignment when the guard applies |
| `kuna-arm-register-shift-signed.xml` | kuna [GH-849](https://github.com/Noelo-Lab/kuna/issues/849) follow-up (the shift's unsigned C repair must retain the signed value consumed by ARM signed compare and ASR) | P9 C emission, signed true/false conditional arms around the guarded unsigned LSL | default (unguarded) and `armshiftguard needed` (guarded): output retains signed comparison and arithmetic shift behavior |
| `kuna-call-array-extent.xml` | Synthetic call-escaped frame storage | P6 stack-frame-layout | `option callarrayextent on\|off` |
| `kuna-arraysize.xml` | decbench type_match census: stack buffers declared with the alignment padding below the next object | P6 stack-frame-layout (an escaped buffer ends at the byte count its library call states) | `option arraysize off\|on\|calls` |
| `kuna-narrowret.xml`, `kuna-narrowret-x86.xml` | decbench type_match census: a local holding a call result declared `unsigned int`/`unsigned long` beside a callee the listing declares `unsigned char`/`bool`/`unsigned int` (authored C, clang Thumb-2 and gcc x86-64 -O2) | P4 output-prototype (a call reads the narrow value its callee states, `decompile-all` only) | `option narrowret on\|off`; negative controls, both passes alike; positive witnesses in `tests/cli/narrowret-*.json` and `kuna-cli/tests/narrow_ret.rs` |
| `kuna-stackstruct.xml` | A local record read through `read(fd,&h,8)` printed its byte and short fields as `v1._4_2_`, which is not C, and no record type was declared (authored C, gcc/clang -O0/-O2) | P6 stack-frame-layout + P9 pointer-notation | `option stackstruct on\|off`, `option partialaccess on\|off` |
| `kuna-declorder.xml` | A function with ten or more locals declared them in string order, `v1, v10, v11, v2, ...` (authored C, gcc -O0) | P9 naming-policy, local declaration order | `option declorder natural\|name` |
| `kuna-strlenscan.xml` | gcc 9 expands `strlen` inline as `repnz scasb`, which printed as a do/while over the string and `~count - 1` (authored C, Ubuntu 20.04 gcc 9.4 -O1) | P3 simplification-quiescence, inline string length | `option strlenscan on\|off` |
| `kuna-opaquepredicate.xml` | a branch on a comparison that is constant for every input (`x * (x + 1) & 1`, `x * x & 3`, `(x \| y) - (x & y) != (x ^ y)`) kept its dead arm (authored C, clang -O0) | P3 simplification-quiescence, constant comparisons | `option opaquepredicate on\|off` |
| `kuna-rettriangle.xml` | `returndup` split the shared return after a one-block `if`, printing `if (c) f(); return 0;` as `if (!c) return 0; f(); return 0;` (authored C, gcc -O0, stack-protected) | P8 goto-quality, return duplication | `option rettriangle on\|off` |
| `kuna-retshape-guards.xml` | an optimized function printed its source's `if (a \|\| b \|\| c) return 0;` as three guards with the same `return 0` (authored C, gcc -O2, the fixture loaded whole); two guards each checking a call's result stay two | P8 goto-quality, print-tree edit after final structuring | `option returndup auto\|record` |
| `kuna-retshape-arms.xml` | an optimized function printed `if (c) v = f(); else { ...; v = -1; } return v;` where the source returns in each arm, the arms joined by the stack-protector check (authored C, gcc -O2, the fixture loaded whole); a one-armed update keeps its one return | P8 goto-quality, return-block split before the final structuring | `option returndup auto\|record` |
| `kuna-tailform.xml` | the function-tail `if` printed in the form the return layout picked: `if (a0) return; note();` for `if (a == 0) note();`, a guard and a second `return 0` for a wrapped two-call body, the arm repeating the final `return 0`, a six-statement void body wrapped, and a list walk whose guard `looptest` owns (authored C, gcc -O2) | P8 goto-quality, print-tree edit after final structuring | `option tailform off\|on\|<n>\|<void>,<value>` |
| `kuna-tailform-noreturn.xml` | a function ending in a call that does not return printed with that call last and the other side's return inside the `if`: `if (v2) return v2 + 7; __fprintf_chk(..); exit(2);`, a merged two-test `abort` and the test after a loop the same way, and a void store-and-return arm before `abort();` (authored C, gcc -O2, the fixture loaded whole) | P8 goto-quality, print-tree edit after final structuring | `option tailform off\|on` |
| `kuna-tailform-model.xml` | the C runtime's `__do_global_dtors_aux` (`if (completed) return; ...`) printed as the guard clause by the default `model`, which still wraps a one-call tail; `option tailform on` keeps the four-line tail wrapped by the size threshold (authored C, gcc -O2, the fixture loaded whole) | P8 goto-quality, print-tree edit after final structuring | `option tailform model\|on` |
| `kuna-tailform-voidreturn.xml` | a void function's last statement `if (!completed) { ...; completed = 1; return; }` printed with the arm's redundant `return;` (the C runtime's `__do_global_dtors_aux`, gcc -O0 fixture loaded whole) | P8 goto-quality, print-tree edit after final structuring | `option tailform off\|on` |

`kuna-minss-nan.xml` pins an authored scalar MINSS and ratio/clamp probe.
P3 and P9 preserve the negation of an ordered float comparison as `!(a < b)`
so the source wins for NaN and equal operands. This is an unconditional
correctness fix, exercised with defaults and `nanignore none`.
`kuna-cli/tests/sse_minss_cli.rs` compiles the emitted bodies with GCC and Clang
at `-O0` and `-O2` and checks NaN payloads, signed zeros and finite controls
against the original assembly.

`kuna-ucomiss-unordered.xml` pins x86 COMISS/COMISD flags read by `seta`,
`setae`, `setbe`, `setb`, an `sbb` borrow and a `jbe` branch, with a memory
operand. Each flag includes the unordered case, so `x > one` prints as
`one < x` and `setbe` as `!(one < x)`. A second image of gcc and clang -O2
`isnan`/`isunordered` idioms and a `jp`+`jbe` pair pins that a separate NaN
test beside such an exact comparison is kept, also after a conditional move
clones the comparison out of the guarded branch (gcc -O1 and -Os). This is an unconditional
correctness fix, exercised with defaults and `nanignore none`.
`kuna-cli/tests/float_unordered_compare.rs` compiles the emitted bodies with
GCC and Clang and checks NaN, signed zeros, infinities and finite values
against the original functions.

`kuna-fcmp-unordered.xml` pins every AArch64 condition read after `fcmp`, six
ARM branches after `vcmp` and `vmrs`, and three compiled functions. The flags
are NZCV=0011 for unordered operands, so `gt` prints as `b < a` and `le` as
`!(b < a)`, never `a != b && !(a < b)`, which a NaN makes true. A strict
correctness fix with one pass.

`kuna-framepublish-seh.xml` pins an MSVC SEH prologue that links its
registration record into `fs:[0]` and allocates a local below it with
`push ecx`. The frame publication holds the record's handler and state
stores, while the garbage ECX store to the local below the record, which the
`and` overwrites with only the `fs:[0]` link STORE in between, is still dead
and does not make ECX a parameter. It also pins a pointer into the middle of
a local array whose reader indexes below it: all four element stores print. A
strict fix with one pass.

`kuna-globalorder.xml` pins issue [#871](https://github.com/Noelo-Lab/kuna/issues/871)
and case-local global stores at the `indexaliasguard global` default: a
computed global update stays after an aliasing load, and a switch's stores
stay within its cases. This is a single-pass strict fix in P3 copy preservation
and P6 variable merging. `globalorder_cli.rs` additionally rebuilds printed C
with GCC and Clang and compares native values and hardware access traces.

C-surface normalization testcases (`kuna-cnorm-*.xml`, no GH issue; user-driven
readability defaults measured on the angr `fmt` corpus binary, DIV-34+ in
`docs/history.md`):

| File | Normalization | P0 assertion |
|---|---|---|
| `kuna-cnorm-protogap.xml` | no blank line between a function prototype and its `{` (DIV-34) | `option braceformat function skip\|next` (upstream option, kuna default flip) |
| `kuna-cnorm-nullprint.xml` | zero pointer constants render as `NULL`, not `(type *)0x0` (DIV-35) | `option nullprinting on\|off` (upstream option, kuna default flip) |
| `kuna-cnorm-compoundassign.xml` | `out = out OP y` statements render as `out OP= y`, `x += -c` folds to `x -= c` (DIV-36) | `option inplaceops on\|off` (upstream option, kuna default flip + ported `emitInplaceOp` consumer) |
| `kuna-cnorm-truthycond.xml` | boolean-context zero compares render truthy: `if (x)` / `if (!p)` (DIV-37) | `option truthycond on\|off` (new kuna settable, P9 `condition-form`) |
| `kuna-cnorm-braceelide.xml` | single-statement if bodies drop their braces (statement indented) (DIV-38) | `option braceelide on\|off` (new kuna settable, P9 `brace-form`) |
| `kuna-cnorm-warnstyle.xml` | warnings render as terse `// slug` end-of-line comments (DIV-39) | `option warnstyle inline\|banner` (new kuna settable, P9 `warning-style`) |
| `kuna-outlang-rust.xml` | the same recovered function rendered as Rust: `unsafe fn` shell, `let mut n: T` declarations, Rust primitive spelling, postfix `as` casts, `match` with a synthesised `_` arm | `option setlanguage c-language\|rust-language` (upstream option, newly live) |
| `oxidizer-securitycheck.xml` | SEFCOM Oxidizer `SecurityCheckRemover`: rustc emits a diverging `panic_bounds_check` branch in front of every checked slice index, string slice and non-constant `/`/`%` (DIV-82) | `option securitycheck on\|off` (new kuna settable, P7 `edge-virtualization`) |
| `kuna-varargstackargs.xml` | a variadic call on an ABI that passes varargs on the stack (Apple arm64) loses them: `fillinMap`'s positional section rules read the structurally empty `x1`-`x7` as the end of the argument list (DIV-101) | always on (P4 `active-input-trial-scoring`, `kuna_varargstackargs.rs`) |
| `kuna-calleearity.xml` | one callee, two arities: the call site whose argument is also the operand of an overflow check loses it to `onlyOpUse`'s branch rejection, while its sibling keeps it (DIV-102) | always on (P4 `trial-finalization`, `kuna_calleearity.rs`) |
| `kuna-rodatastring.xml` | a string block-copied out of read-only memory into the frame renders as invalid C: `v1[0] = (char[8])msg._0_8_;` (there is no array cast) and the literal never appears (DIV-113) | `option rodatastring on\|off` (new kuna settable, P5 `constsequence`) |
| `kuna-calleenamedlocals.xml` | kuna [#762](https://github.com/Noelo-Lab/kuna/issues/762): two distinct pointer locals passed to one `consume(Node *object)` both took the name `object`, `dedupvardecls` kept one declaration, and the store after the inner loop cleared the last member's flag instead of the representative's; the same defect through a struct-by-value argument (its fields stored into another struct, or into a same-named global), a user rename (`&v2` printed as the other slot's address), and a slot named like a global (its narrow views read the global), plus a global array a suffixed local must not capture | P6 local naming (a callee-parameter name is made unique against names already bound to other highs; struct roots share only among themselves) + P9 declaration emission (the identical-line collapse only merges lines of one Symbol, and every high printed through another declaration follows its final name); correctness fix, no option. Passes 2 and 3 pin `option dedupvardecls off` and `option namestyle ghidra` |
| `kuna-stacksolver-unsolved.xml` | kuna [#1288](https://github.com/Noelo-Lab/kuna/issues/1288) (a PKLITE-packed DOS executable decompiled as a raw real-mode image grew memory without bound: `DEC SP` in its packed body solved a stack variable to 65535, the `StackSolver` sentinel for "unsolved", and two such variables re-pushed each other forever; where the solve did finish, the genuine 65535 still read as unsolved and the function kept a raw stack-pointer local) | P6 variables, `ActionStackPtrFlow` extra-pop solve | none (strict fix): the solve terminates and the loop around the call is emitted; `h` reads its return address at entry + 2 with no raw stack-pointer local |
| `kuna-prefixed-noreturn.xml` | a defined stub named `<prefix>__<runtime name>` (`stub___stack_chk_fail`) read as returning, so flow ran past the caller's last call into padding: AArch64 printed a bogus `UndefinedInstructionException` jump-as-call and x86-64 tripped `funcboundflow` | P2 flow (`is_proxy_no_return_at` ends a call to `<prefix>__<entry>`, for an exact list entry holding an `_`, only when the callee's body is a lone return or one jump and the call is the last instruction of its sized caller; `machine__exit`, `mystack_chk_fail_count`, `handle_exit`, a `session__pthread_exit` with a real body and an `obj__stack_chk_fail` stub called with code after it stay returning) | `option noreturn_known on\|off` |
| `kuna-importdata.xml` | a data import read through its `__got` slot prints as a dereference of the anonymous slot (`*dat_100004000`; 921 such reads in an arm64e iOS system daemon, `*dat_1041cc6c8` for the stack guard in a large third-party iOS app) although the slot is named for the import and painted externref | P9 naming-policy (`PrintC::op_load` resolves a LOAD through an externref import slot as `ActionDeindirect` resolves a call through it) | `option importdata on\|off` (kuna settable, default on) |
| `kuna-objcrefs.xml` | A large third-party iOS app (84 MB arm64, Swift + ObjC): every message send read `objc_msgSendSuper2(&super, dat_1045b76f0)` even with `objc on`, and 274 of 1,622 calls in a 500-function sample were `sub_1034xxxxx(obj)` through an anonymous `__objc_stubs` entry. P1 external-refinement (`analyzers/objc/kuna_objcrefs.rs` names `__objc_selrefs` / `__objc_classrefs` / `__objc_superrefs` slots and the `objc_msgSend$<sel>` stubs with a pinned prototype; `loader/kuna_dynrelocs.rs (macho_chained)` unpacks the chained-fixup rebases under `dynrelocs`) → P3 fold of the constant slots | `option objc on\|off` (names and the slot fold) |
| `kuna-ptrauthtrap.xml` | An arm64e iOS system daemon: 205 functions printed `if ((v1 ^ v1 << 1) >> 0x3e & 1) { (*SoftwareBreakpoint(0xc471,..))(); }` before a tail call, the compiler's inline check of an `autibsp` result that SLEIGH hides. P2 flow-classification (`p2_lift/kuna_ptrauthtrap.rs` removes each CBRANCH edge into a `brk #0xc470..0xc473` trap block whose guard is the return-address bit test, pre-SSA) | `option ptrauthtrap on\|off` (new kuna settable) |
| `kuna-authptrcall.xml` | An arm64e iOS system daemon: three `___chkstk_darwin` calls through `__auth_ptr` rendered `(*dat_1000716c0)(...)` while every `__auth_got` call was named; `__auth_ptr` has no indirect-symbol entries. P1 external-refinement (`loader/kuna_authptrcall.rs` names each `__auth_ptr` bind slot from its chained bind and paints it `externref`) | always on |
| `kuna-cfstrings.xml` | a Mach-O `__cfstring` constant prints as its entry address (`NSLog(0x1044376b0)` in a large third-party iOS app): code references the CFString struct, and the constant-string route reads the bytes at that address | P1 external-refinement (the Mach-O `cfstrings` analysis pass; the commit types the entry an opaque `struct __CFString`, spelled `CFStringRef` through a pointer, and seeds the StringManager with its resolved string); `option cfstrings on\|off\|annotate` (kuna settable, default on; carried by `--mode aggressive`); on renders `CFSTR("...")`, annotate adds the entry address |
| `kuna-elfgotimports.xml` | an ELF `-fPIC` read of data another object defines (`stdout`, `environ`, a library global) loads a `GLOB_DAT` GOT slot and then loads through it, and printed as `*dat_<slot>`: PE/Mach-O import slots were painted externref but an ELF GOT slot never was | P1 environment-binding (`kuna_elfgotimports`: the undefined-symbol `GLOB_DAT` slots are import slots, named and painted externref) + P9 naming-policy (`importdata`) | `option importdata on\|off` (default on); x86-64 and AArch64 shared objects, with a defined-symbol control |
| `kuna-objcmeta.xml` | A third-party iOS app (1.9 MB arm64 ObjC): with `objc on` every category method stayed `sub_<addr>` (77: categories extend classes of other images, out of the class walk's reach), every ivar access read `*(long *)((long)self + dat_1001aea74)` (505 sites: the offset variable's 1-byte label never matched the 4-byte load) and a protocol-declared selector typed nothing. P1 external-refinement (`analyzers/objc/kuna_objccats.rs`, `kuna_objcprotos.rs`, `kuna_objcivars.rs`: `-[Class(Category) sel]` renames, `_OBJC_PROTOCOL_$_` names + declared encodings for the stubs, a struct per class from `class_ro_t.ivars` typing `self`) → P3 fold of the offset variables | `option objc on\|off` (names), `option objcivars on\|off` (new kuna settable; struct + typed self + offset fold) |
| `kuna-dyldcacheobjc.xml` | Foundation out of an iOS 16.4 dyld shared cache (Noelo-Lab/kuna#1623): `objc on` typed no `self` and every `objc_msgSend$<sel>` stub stayed `sub_<addr>`. A cache's small method lists hold selectors and type strings as offsets from the cache's selector base (`0x40000000`/`0x20000000`) and carry two flags in the low bits of the entry size, the strings and selector reference targets live in libobjc's `__OBJC_RO` outside the image, and `baseMethods` with its low bit set is a relative list of lists. P1 external-refinement (`loader/kuna_dyldcache.rs`: the base from the ObjCOptimizationHeader, else libobjc's `objc_opt_t`; `analyzers/objc/kuna_objccache.rs` reads through the cache; `methods.rs`/`classt.rs` decode the flags and the list of lists) on the synthetic caches of `dyld_cache_objc.py` | `option objc on\|off` (no new settable) |
| `kuna-objcdirect.xml` | UIKitCore out of an iOS 16.4 dyld shared cache: `-[UILabel _setAllowsDefaultTighteningForTruncation:]` printed its receiver as `objc_object *a0` and an ivar as `((objc_object **)a0)[0x56]`, where `objc off` at least showed `_OBJC_IVAR_$_UILabel._configuration`. The method is `objc_direct`: no method list names it, it takes no `_cmd`, and only its symbol says it is a method, so the pass never typed its `self` while still folding the offset variables it loads. P1 external-refinement (`analyzers/objc/kuna_objcdirect.rs` hints the first argument register of every method symbol no method list gave as `self`, typed `<Class> *` for an instance method under `objcivars`; the hint is unlocked, the other parameters are recovered) on `macho_objc_direct.m` | none (a correctness fix, always on under `objc`) |
| `kuna-objcsdk.xml` | A 1.9 MB third-party iOS app: 2,897 of 25,084 `objc_msgSend$<sel>` calls rendered with no argument (`setHidden:();`), `setAlpha:` showed `0x3ff0000000000000`, `setFrame:` one word for a CGRect — a framework selector is declared nowhere in the image. P1 external-refinement (`analyzers/objc/kuna_objcsdk.rs` reads `@interface`/`@protocol` methods, `@property` getter/setter pairs and `typedef`s from a user-named SDK or header directory into runtime-style encodings; the stub prototype path pins scalars to x/d registers, spreads an HFA `CGRect` over `d0..d3`, returns the flat struct) | `option objcsdk <directory>\|off` (new kuna settable, env-bridged load-time) |
| `kuna-objcforin.xml` | Perplexity iOS 26.46 (`-[SentryScope setAttributeValue:forKey:]`): every `for (x in c)` loop printed its `NSFastEnumerationState` as eight unrelated locals and read `state.itemsPtr[i]` as `*(objc_object **)(v6 + v5 * 8)`, because the header reader spelled `(NSFastEnumerationState *)` as a class pointer, could not spell the `(id [_Nonnull])` buffer or `NSDictionary`'s generic `(K [])`, and the decoder made any `^{...}` a `void *`, so the selector had no prototype. P1 external-refinement (`analyzers/objc/kuna_objcsdk.rs` lays out a pointer to a `typedef struct` with member names, an array parameter is a pointer, a generic parameter is `id`; `analyzers/objc/encoding.rs` decodes `^{Name=...}` to a pointer to that struct) → P6 `gatherOpen` types the stack object passed to the typelocked pointer | `option objcsdk <directory>\|off` (no new settable) |
| `kuna-objctypes.xml` | with the `objcivars` structs only `self` was typed: a getter returned `objc_object *`, an ivar of a local class was `objc_object *`, and a field read through either stayed `*(int *)((long)v + 8)` although the property attribute (`T@"Base"`) and ivar encoding (`@"Base"`) name the class. P1 external-refinement (`analyzers/objc/kuna_objcprops.rs` reads every `property_list_t` into class-naming getter/setter encodings; `Selectors::record` keeps the class-naming spelling of agreeing declarations; the decoder resolves `@"Base"` to the layout's struct, declared-record completion included) → P5 field resolution through the typed pointer | `option objc on\|off`, `option objcivars on\|off` (no new settable) |
| `kuna-opaquepointerretype.xml` | `probe` keeps `&frame[1]` in a saved register (ARM32 r4, PowerPC32 r29); `retype` to a pointer to a bodiless `struct Opaque` added a default one-byte frame range that split the array, the copy holding the address was propagated away, and the retype was refused with `int4 *v3` printed instead. P6 frame layout under the stable-names replay (`p6_variables/funcdata_spacebase.rs` `gather_open`): a value carrying a replayed retype's lock whose pointee has no size lays its range out by the pointee it was printed with (`kuna_stablenames.rs` `printed_pointee`) | always on (`retype` directive) |
| `kuna-stackpointerretype.xml` | `probe` keeps `&frame[1]` in a saved register (ARM32 r4, PowerPC32 r29); `retype` to a 16-byte `Probe *` re-laid the frame as a `Probe` record, which turned the pointer's definition into a copy that was propagated into its readers: the C printed `consume(&v4)` and the retype was refused. P3 copy propagation (`p3_dataflow/ruleaction_3.rs` `RulePropagateCopy`) keeps a copy that is a definition the replayed retype's lock matched (`p6_variables/kuna_stablenames.rs` `is_retyped_definition`) | always on (`retype` directive) |
| `kuna-poolfallthru.xml` | ARM32 `probe` (little- and big-endian) ends in a call kuna cannot prove no-return, followed by its PC-relative literal pool; the walk took the call's fall-through, decoded the constants as instructions and printed `halt_baddata(); // warn: Bad instruction` after the call. P2 flow classification (`p2_lift/kuna_funcboundflow.rs` `kuna_should_bound_at_pool`): a call's fall-through onto a literal pool the Listing's trusted loads prove (`kuna-analysis` `listing/kuna_poolextents.rs`) ends the function | `option funcboundflow off\|on` |
| `kuna-chkstkdarwin.xml` | Apple clang calls `___chkstk_darwin` before a frame larger than a page or a variable-length array, and reads the argument registers after it; the AArch64 model kills x0, x1 and x8-x18 at the call, so `_big_frame`, `_vla_last` and `_probe_stub` printed `(void)` and assigned the probe's result | P2 flow classification (`p2_lift/kuna_chkstkdarwin.rs` removes the probe call from the raw p-code of a Darwin AArch64 image, through the GOT slot, an arm64e `__auth_ptr` slot or the `__stubs` entry; an AArch64 ELF function merely named `___chkstk_darwin` keeps its call) | `option chkstkdarwin on\|off` (destructive, default on) |
| `kuna-typevote.xml` | `put_u32_be` shifts its `uint4` parameter through merge copies that type inference never saw, so each read printed a cast back to the declared type, `(uint4)a1 >> 0x18` | P5 type propagation (`p5_types/kuna_typevote.rs` types a variable left `undefinedN` by a vote over the ops on its instances) | `option typevote on\|off` (default on) |
| `kuna-gotonext.xml` | gcc and clang `-O2` switches whose cases fall through printed `goto label_X;` directly above `case 1: label_X:`, clang's chain printed `case 3:` first, and a loop exit printed `goto` where `break;` goes there | P8 goto quality (`p8_structure/kuna_gotonext.rs` censuses the final structured tree at print time; `Funcdata::chain_fallthru_cases` re-derives the fall-thru chains before the case sort) | `option gotonext on\|off` (default on) |
| `kuna-panicloc.xml` | A Rust panic site passes `&'static core::panic::Location`, which printed as a bare address: `panic_fmt(..,0x2002c0)` | P1 external refinement (`kuna-analysis` `strings/kuna_panicloc.rs` finds the records by shape, types them `core::panic::Location` and names them after file, line and column) | `option panicloc on\|off` (default on, Rust images only) |
| `kuna-brktrap.xml` | AArch64 SLEIGH lifts `brk #imm` to a jump through `SoftwareBreakpoint`, which flow truncated to a call that returns, so a function whose overflow or bounds check ends in a compiler trap printed as `void` and passed the value it returns to the trap, or returned a value fabricated from a call through it | P2 flow classification (`p2_lift/kuna_brktrap.rs` matches the lowered `brk` with a compiler trap immediate, `#1`, `#1000`, `#0x5500`-`#0x55ff`, `#0xc470`-`#0xc473` or `#0xf003`, and `flow.rs` ends the flow there with a no-return halt; `#0xf003` also reads its `__fastfail` code from `w0`, so the code prints; `brk #0xf000`, `brk #0` and `brk #0x800` are controls) | none (strict fix): one pass at the default |
| `kuna-ebpf-elf-map-reloc.xml` | kuna [#1652](https://github.com/Noelo-Lab/kuna/issues/1652): a `clang --target=bpf -c` object did not load (EM_BPF 247 had no language), and with `--target` the `R_BPF_64_64` map reference was written as one 64-bit word over the start of the 16-byte `lddw`, so `prog` printed `halt_baddata()` | P1 loader (`kuna-analysis` `loadimage_object.rs` maps EM_BPF to `eBPF:LE\|BE:64:default`; `loader/reloc_apply.rs` patches `R_BPF_64_64` into the two `imm` halves at +4 and +12, `R_BPF_64_32` into the call `imm` as `(S + A - P) / 8 - 1` with the REL addend `(imm + 1) * 8`, so a static callee named by the `.text` section symbol is the one its `imm` selects, in either byte order, and `R_BPF_64_ABS64/ABS32/NODYLD32` as data words; a pspec `<default_symbols>` label outside the default code space at a call target names the callee, so `call 1` prints `bpf_map_lookup_elem`) | none (strict fix): one pass at the default |
| `kuna-boundschecked.xml` | GCC passes a size given as both capacity and count to a C11 Annex K function by copying x3 into x1 (`mov x3,#64; mov w2,#0; mov x1,x3`), and call recovery dropped the later register, so `memset_s(p, 64, 0, 64)` printed `memset_s(v1,0x40,0)`; a Thumb `memcpy_s` also took its caller's pushed `r3` as a fifth argument (GCC 15 -O2 -fPIC shapes, AArch64 both byte orders and Thumb-2) | P1 external refinement (`kuna-analysis` `protos/kuna_libcsigs.rs` seeds the Annex K rows onto the imports) | none (always on) |

`kuna-storecopyeffects.xml` pins the AArch64 address-spill store across calls,
repeated stores, a load between calls, and a truly dead-store control. The
compiled round trips in `kuna-cli/tests/store_copy_effects.rs` also exercise
aliasing and distinct pointers on AArch64 and x86-64 at `-O0` and `-O2`.

`kuna-varsources.xml` pins register and stack source comments after copy
elimination, PHI and CMOV merging, byte narrowing, and paired-register returns.
It also checks explicit names, the absence of parameter source comments,
outgoing call copies versus genuine local definitions, and the `namestyle
ghidra` control. Its x86-64 instructions are independently
assembled synthetic examples; it contains no bytes from the reported binary.

`kuna-parameter-alias.xml` checks that a name/type assertion on a later EDX
local preserves the incoming parameter, its entry comparisons, and the value
passed to a callee across repeated decompilation. Its instructions are synthetic.

`kuna-custom-call-storage.xml` checks an authored MSABI caller whose getter
return is passed in EDX to a callee with one explicit formal. Repeated builds
retain that value and the one-formal signature; the ordinary two-slot prototype
passes the same value as argument two. The assertion API and callable-type
interning controls are in `kuna-console/tests/verify_assertplane.rs`.

Infrastructure testcases (no GH issue; they regression-test the kuna stage machinery
itself): `kuna-console.xml` (registry + `stage list/map/status`), `kuna-assert.xml`
(`kassert` routing + reported rewind scopes), `kuna-restarts.xml` (restart-reason
observability on the switchmulti multistage restart), `kuna-pipeline-quality.xml`
(reduced-pipeline sub-queries + the goto-quality metric), `kuna-regions.xml`
(S7 region-identification observability: `region tree/blocks/walk` from the angr
RegionIdentifier port over the REAL `bblocks` CFG — `build_from_block_graph`, the
W7 block-graph adapter closed in regionstructure Inc 0, including the per-block
`endsWithBranchindOrCbranch` `lastOp` probe — on a loop+diamond CFG;
`docs/spec/07-regions.md`), `kuna-listing-flag.xml`
(the Listing/xref disassembly tier flag `option listing on|off`, default-off: parses,
round-trips, and is inert on the XML path — PR1 of the Listing/xref tier).
`ghdec-fast-funcdisc.xml` clears that wrapper architecture and loads a real PE
twice: option off reproduces the missing direct callee, and option on discovers
and decompiles its body.

A script command may use `<com expecterror="true">...</com>` when failure is
the regression witness (for example, the option-off pass). Its actual console
error is included in the matchable output, and the next command runs. The
command must fail; success is a file error. Unexpected errors still abort.
Output and expected diagnostics remain matchable if the script closes its
output redirect; capturing them does not reopen it.
Use direct commands rather than nested `source` scripts with this attribute.

`kuna-calleeretpreserves.xml` also covers a declared-void direct helper with a
terminal failure call: marking and unmarking the leaf refreshes the cached body
summary.
`kuna-cli/tests/direct_call_return.rs` checks native and recompiled emitted C
at `-O0` and `-O2`, real return-register writes, unknown returning calls and an
incomplete helper. These are authored MSABI probes in an ELF container.

`kuna-floatprecision.xml` pins authored scalar and packed SSE binary32 inputs
widened for binary64 subtraction/addition before one final narrowing, plus
widened multiplication/division and float-only/conversion-only controls.
`kuna-cli/tests/float_precision.rs` recompiles the emitted C with GCC and Clang
at `-O0` and `-O2` and compares its bits against the assembly for cancellation,
subnormals, overflow, signed zeros, infinities and individual NaN payloads.

`kuna-arm-conditional-switch.xml` pins authored A32 `SUBS`/`ADDPL PC`
branch tables, including case bodies that share a saved-frame epilogue, with
`tailcalljump` on and off. Unconditional `ADD PC`, healthy `ADDLS PC`, and a
real indirect function-pointer call are controls. `kuna-cli/tests/arm_conditional_switch.rs`
compiles the untouched emitted bodies with GCC and Clang at `-O0` and `-O2`
with undefined-behavior checks and exercises every case and signed boundaries.

- `kuna-sparc-double-abs.xml`, `kuna-sparc64-double-abs.xml`: binary64 absolute value from a high-word `fabss`, without a numeric high-word cast.

- `kuna-sparc-double-sign.xml`, `kuna-sparc64-double-sign.xml`: binary64 negation from a high-word `fnegs`, without a numeric high-word cast.

- `kuna-sparc-register-pair.xml`: a SPARC32 register pair loaded whole by `ldd` on one path and set as two 32-bit values on the other keeps a loop index and a counter as separate locals (always on; without it they fuse into one 64-bit local updated with `CONCAT44`).

- `kuna-sparc-trap-resume.xml`: a `ta 0x10` in the delay slot of `ba` lifts to the trap's `call` followed by the branch's `goto`, and a function is declared at the branch target. The `goto` is where the trap resumes, so the function prints the trap once and returns the resumed code's `3`, with no call to the declared function (always on; one pass, a strict fix).

- `kuna-integer-float-rounding.xml`, `kuna-integer-float-rounding64.xml`, `kuna-integer-float-rounding-x87.xml`: an integer-to-float conversion keeps its rounding through a later float conversion or a narrower float trace (always on), while exact intermediates, x87 extended conversions and `fld1` still combine.

`kuna-global-load-call-order.xml` keeps a post-call memory-loaded global write after the getter, including a later call, a loop marker and a no-final-call control. The CLI roundtrips check an impure getter, a computed-value reload control and overlapping x86-64 SIMD reads using GCC/Clang at O0/O2.
- `kuna-register-pair-call-args.xml`: overwritten wide-return register components at an undeclared call, with single-half and unchanged-pair controls.

`kuna-pointer-storage-literal.xml` publishes a pointer through a mapped byte frame
and checks that its compound initializer converts the integer carrier back to a
pointer. The CLI companion compiles fresh PE32/ELF64 examples with Clang and runs
the Linux native/recompiled functions at O0/O2. Source: `src/pointer_storage_literal.c`.

`kuna-ppc32picimports.xml` changes the live option off/on/off after loading one
PPC32 ELF. Only the enabled pass names its proved terminal call and removes the
unreachable continuation. The shared stub remains anonymous. The CLI companion
covers both byte orders, both stub forms, direct and indirect calls, entry
contexts, returning callers and unproved memory or control flow.

`kuna-ppc32picimports-indirect.xml` checks an indirect caller selecting a returning
slot through the same stub as a terminal direct caller. Its return is preserved.

`kuna-ppc32picimports-runtime.xml` loads images with different relocations in one
process. Each caller uses its own image's metadata; returning callers keep their
original calls and continuations. Other PPC32 stages exercise callback entries,
spills, computed transfers, packed pointers and executable mapping boundaries.
All retained ELF fixtures are generated by `ppc32_import_fixture.rs` and checked
byte-for-byte by the CLI integration suite.

`kuna-ppc32picimports-reversed-save.xml` preserves the returning continuation
when an unaligned reversed store overlaps the saved PIC base. Its CLI companion
covers both byte orders and a neighboring-store control that still recovers
the terminal import.

`kuna-ppc32picimports-slot-clobber.xml` retains a return after an imported copy
overwrites a writable relocation slot, across off/on/off option toggles.

- `kuna-ppc32picimports-abi.xml`, `kuna-ppc32picimports-abi-indirect.xml`, and
  `kuna-ppc32picimports-driver.xml` exercise explicit `ppc32picimports abi`
  recovery, including ordinary returning callers and compiler start files.

`ghangr-godwarfsig.xml` checks opt-in Go DWARF scalar/pointer signatures,
compressed cross-unit references, result roles, caller propagation and a wrong-register
control. Its console companion executes hermetic and native recovered Go bodies.

`ghangr-go-dwarf-string-inputs.xml` checks the two physical Go string input
words, the ABI slot after an earlier scalar, numeric byte iteration, a retained
bounds branch, and explicit prototype precedence. The stage also exercises
malformed pairs and a word-identical scalar signature.

`ghangr-go-pair-results.xml` checks opt-in paired 64-bit Go integer results,
ordinary local struct storage, destructuring calls and nested result selection.
Its console companion covers the option gate, malformed cross-unit DWARF,
metadata collisions, indirect calls and complete emitted native Go bodies.

`ghangr-go-string-pointers.xml` exercises internal byte-string arguments in C and
Go output, including escapes, UTF-8 and the data-pointer conversion. Its emitted
Go bodies also compile and execute against a byte-checking sink.

`kuna-ppc32picimports-runtime.xml` covers both caller-local `on` bindings and
ABI-mode reloads of images whose legacy fingerprints collide, plus a TAR helper
whose skipped assignment must not become a no-return import contract.

`ghangr-joined-callback-return.xml` covers late typed callbacks returning an
integer pair: either component, a narrowed high component, both components,
whole forwarding, and scalar and void callback controls.

`ghangr-integer-field-extract.xml` checks narrow, narrow-then-widen, and shifted
integer fields in C, Go and Rust. Native CLI tests execute the complete emitted
callers against boundary values and check that each helper is called once.

`ghangr-go-store-value.xml` checks Go integer destination conversions for fields,
plain/indexed pointers and a side-effectful call, with all integer widths and
complete native bodies covered by the CLI integration fixture.
`kuna-kernelcache.xml` loads a synthetic arm64 kernel collection three ways: the
bare MH_FILESET with `kernelcache off` (one image, no symbols), its kext out of an
IMG4(IM4P(LZFSE)) wrapper, and its kernel out of a bare IM4P(complzss). The
fixtures and their generator live in
`decompiler/crates/kuna-analysis/tests/fixtures/kernelcache/`.
`kuna-kernelcacheboot.xml` loads the kext of a synthetic Auxiliary collection
twice: with `kernelcache-boot off` its calls into the kernel go through
unresolved slots, and by default they are named from the boot collection that
sits beside it (`linked/` in the same fixture directory).

`ghangr-go-pointer-arithmetic-32.xml` and `ghangr-go-pointer-arithmetic-64.xml`
check live pointer bases through constant, negative and signed dynamic byte
offsets in Go loads, stores and returns. Integer-address reconstruction remains
a control. The CLI companion builds both authored assembly fixtures and runs
the complete emitted functions with Go pointer checks and full-array canaries.

`kuna-elf-unwind-pcread.xml` checks an authored PowerPC PC-relative literal read:
with FDE ownership disabled it remains a call; with ownership enabled the
complete body returns `a0 * 3 + 7`. The fixture generator is
`decompiler/crates/kuna-cli/tests/fixtures/elf_unwind_fixture.rs`.

`kuna-assert-callee.xml` checks `override indirect`: an indirect call and an
indirect tail jump each print as a direct call to the stated function, and the
tail call to a callee marked no-return ends the function with no return. The CLI
companion covers `--assert callee` on both surfaces and every refusal. The
fixture source is `decompiler/crates/kuna-cli/tests/fixtures/assert_callee.c`.

`kuna-rust-process-exit.xml` preserves exit status and preceding stores while
removing dead trap flow after a known Rust standard-library process exit.
Different namespaces and a longer similarly named function retain their callers.

`kuna-piecepathology.xml` checks that a function handing back a call declared
`int` returns 4 bytes on x86-64, not the whole of rax: `next_byte` inlines
`getc_unlocked` and refills through `int __uflow(FILE *)`, whose upper half of
rax is undefined. `kuna-regpiece.xml` pins the same fix on `sh_openpipe`.

`gh1922-declared-caller-readarg-thumb.xml` checks Thumb callers with declared
fixed parameter lists that save `r3` as gcc's alignment pad (`push {r3, lr}`):
a newlib-shaped copy keeps the size the caller also multiplies with, and the
`data` and `len` a caller passes on untouched. A callee that reads only `r0`
and `r1` still gets two arguments.

`gh1922-pushpad-caller-readarg-thumb.xml` checks the same alignment pad with
no declaration: a Thumb caller whose `push {r3, r4, r5, r6, r7, lr}` stores
`r3` beneath `lr` keeps the length it also stores, a callee that reads only
`r0` and `r1` still gets two arguments, and a caller that saves `r0`-`r3` in
a push of their own (a variadic register save) still loses it, as does one
that pads with `push {r3, ..., lr}` and saves `r1`-`r3` with `stm` (clang
-Oz).

`kuna-retvalue.xml` checks that a function its callers read a result from
returns it though it also stores or loads through it, and that zero stores into
globals do not make a returned zero look like a `-fzero-call-used-regs`
epilogue. gcc -O2's `make_cell` stores through what `malloc` returned,
`skip_blanks` loads through the pointer it advances, and `lookup` walks a list
through its cursor, each read by `main`: they print their return, where they
printed `void`. `reset` (`movl $0,gi; xor %eax,%eax; movl $0,gj; ret`) returns
its zero. `push_cell`, whose result `main` ignores, stays `void`. A strict fix
with one pass.

`kuna-vcmp-carry.xml` checks the ARM conditions that read the carry after
`vcmp`/`vcmpe` and `vmrs APSR_nzcv, fpscr`. The hardware sets C for unordered
operands, so `hi` and `cs` are taken for a NaN and `ls` and `cc` are not; each
printed test is false exactly when its branch is taken, NaN included.
