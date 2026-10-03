# bejoin: a big-endian two-register value joined in the ABI's order

## The defect

A 32-bit ABI returns a 64-bit value in two registers. Every big-endian ABI kuna
supports puts the HIGH word in the first register, because a register pair holds
a wide value as a load from memory would, lower address first:

| ABI | pair | high word | evidence |
|---|---|---|---|
| PowerPC SVR4 | r3:r4 | r3 | clang `-O2 wide_mul`: `mulhw 5,4,3; mullw 4,4,3; mr 3,5` |
| MIPS o32 (BE) | $2:$3 | $2 | `mult $4,$5; mflo $3; jr $31; mfhi $2` |
| SPARC | %o0:%o1 | %o0 | `smul %i1,%i0,%i1; rd %y,%i0` (the product's high word from `%y`) |
| ARM AAPCS (BE) | r0:r1 | r0 | `smull r2,r3,r1,r0; mov r0,r3; mov r1,r2` |
| AArch64 AAPCS64 (BE) | x0:x1 (`__int128`) | x0 | `smulh x8,x1,x0; mul x1,x1,x0; mov x0,x8` |

The cspecs say so: a `<join/>` output rule consumes the most significant piece
first on a big-endian register space, and records it on the trial container
(`ParamActive::isJoinReverse`). AVR's gcc spec, little-endian, sets the same flag
through `reversesignif` (its `int` comes back with the high byte in `R25`, the
first register). Return recovery (`ActionReturnRecovery::buildReturnOutput`,
upstream too) and kuna's call-output pair ignored the flag and joined the first
register as the LOW word, so clang `-O2 long long wide_mul(int,int)` printed its
halves swapped on PowerPC, MIPS, SPARC and ARM big-endian, and AVR `negate`
printed `CONCAT11(a1,a0)`.

## Why the order cannot simply follow the ABI

Ancestor realism makes a pair of any function whose second register reaches the
RETURN holding something, and a function returning one `int` often leaves
something there:

* SPARC's `restore` copies every in-register back to its out-register, so `%o1`
  arrives holding whatever the function left in `%i1`: the second argument, a
  pointer it read through, a loop position, a byte it stored, a dead reload.
* clang -O0 on MIPS materializes a zero in `$2` and `$3` and overwrites `$2`
  (`both`, `realeof`), leaving a literal zero in `$3` nothing reads.
* On any target, a scratch value the function also used.

Joined first register low, such a pair narrows back to the first register
everywhere a later pass drops the phantom half: the uncomputed-half repair, a
`bool` or byte return, C's own truncation of `CONCAT44(junk, value)` in a narrow
prototype. That is the right value. Joined in the ABI's order every one of those
keeps the wrong register. Earlier versions of this change joined every pair in
the ABI's order and repaired the leftovers afterwards (a register-window
classification, late narrowing, literal and zero-extension look-throughs); each
review found a class they turned from right to wrong (MIPS `both` as
`v1 << 0x20`, zlib `deflateBound`'s pairs swapped under an `int` return, SPARC
byte readers returning `char`, dash `__pgetc` widened to a pair). All of that is
gone.

## The fix as built

The order is decided once, at return recovery, by asking whether the second
register's low word is returned on purpose (`p4_calls/kuna_bejoin.rs`):

* walk back from the second register at every live RETURN through copies, phis,
  indirects and the pieces heritage splits a register pair into (following the
  low word's byte offset), to where its value was made;
* the halves of one wider value made by an operation (an 8-byte load, a product
  split by `mflo`/`mfhi`) are a wide value;
* the register's own entry value with no instruction moving it (untouched, or
  only the window's `save`/`restore` copies) is a leftover;
* a value also used for anything but the pair's RETURN slots (a store, a settled
  call argument, a branch, an address, the stack pointer), one reaching the
  first register other than through its sign, a comparison or as the other half
  of a wider value, or the first register's own value, is scratch;
* what a 64-bit comparison leaves behind: the first register holds a computed
  0 or 1 (a comparison, a carry, a sign bit, a leading-zero count shifted to
  its top bit, 0s and 1s chosen between), or a literal 0 or 1 where the
  function's live RETURNs hold both (a boolean a branch chose), or the low word
  is a sum or difference and no carry, comparison or value of the low word
  reaches a first register that is more than one literal. ARM big-endian
  `return x >= 0x80000000ULL` computes `0x7fffffff - lo` into r1 for its flags;
  SQLite's `v == (int)v` on SPARC -O2 (`addcc; addxcc; cmp; be`, then `restore
  %g0,1,%o0` or `restore %g0,%g0,%o0`) leaves `lo + 0x80000000` in `%o1`, and
  PowerPC the same sum beside `li 3,1`/`li 3,0` or `cntlzw 3,3; srwi 3,3,5`; the
  halves of a real 64-bit sum are tied by its carry. The same leftover sits
  beside any literal or flag-derived value the check returns (`return 34` or
  `return 0` after a range check, `-1`/`0`, `neg 3,3` of `cntlzw; srwi 5`), so
  a low word that is a sum or difference (`0 - x` included) whose carry or
  borrow reaches a branch or the first register is also a comparison's
  leftover when the first register is worked out from comparisons and literals
  alone (a literal, a value with at most one settable bit, or such values
  negated, offset, shifted, masked, added or chosen between; a choice's input
  its edge's branch just tested equal to a literal counts as that literal, as
  in ARM's `adcs r0,r0,#0; movne r0,#22`) and the sum's own carry is not one
  of those flags (a real 64-bit sum adds it into its high word);
* an `int` truncation of a stack temporary: clang -O0 reloads a 64-bit value it
  spilled to two stack words for `(int)(c ? a : b)`, `(int)MIN(len, INT_MAX)` or
  an absolute value, the low word into the first register (`lwz 4,12(31); lwz
  3,16(31)` on PowerPC), the reverse of the `long long` (`lwz 3,12(31); lwz
  4,16(31)`). Both registers loaded from adjacent words of one stack object,
  the first from the less significant one, is a truncation, unless each word
  is the home its register's own argument was stored to (-O0 `((u64)a << 32) |
  b`). This and the comparison tests apply only where one register holds a
  whole `int` (not AVR's bytes);
* a division by a constant: the low word is the low half of a product at
  least two words wide whose high half reaches the first register shifted
  right by a literal. ARM big-endian `a / 10` at -O0 is `umull r1,r0,r2,r3;
  lsr r0,r0,#3`, `a / 7` on ARMv6/v7 `umull r1,r2,r0,r1; sub; add ...,lsr #1;
  lsr r0,r0,#2`, signed `a / 3` `smull r1,r0,r2,r3; add r0,r0,r0,lsr #31`:
  `umull` and `smull` leave the product's low half in r1. A `long long`
  product returns its high half as it is or with more terms added, never
  shifted down. A divisor whose magic number needs no shift (`a / 641`, `a /
  6700417`: a bare `umull r1,r0,r0,r1`) leaves exactly a product's registers
  and reads as the product (the trade-off table below);
* the high half of a 64-bit temporary held the reverse way round: the low word
  is a right shift by a literal (short of the top bit or sign alone) of a value
  whose dropped bits reach the first register. ARM big-endian computes an `int`
  that is a 64-bit sum, difference or product shifted right by one in the
  reverse pair, low half in r0: `(u32)(((u64)a + b) >> 1)` is `adds r0,r1,r0;
  adc r1,r2,#0; lsrs r1,r1,#1; rrx r0,r0`, and clang -O0 shifts a 64-bit
  argument the same way (`movs r1,r1,lsr #1; mov r0,r0,rrx`), leaving the
  stale `r1 >> 1`. A `long long` shifted right moves the bit from the first
  register into the second, so its low word is never the bare shift. The walk
  from the first register follows a stack reload to what the function stored
  there (-O0 keeps the result in a local);
* an `int` worked out from the high half of a 64-bit temporary, its low half
  left in the second register. When the low word is a sum or difference whose
  carry or borrow reaches the first register, the first register must be that
  sum's high word itself: additions and subtractions of values no flag reaches
  and of no literal but zero, each flag entering with the sign the low word's
  operation gives it (a carry added, a borrow subtracted, a "no borrow" such
  as PowerPC's `subfc` carry added, MIPS's `a < a - b` borrow subtracted), a
  literal allowed only where the low word adds one too and not on top of a
  word already holding the carry (`adde` of a register holding 7 is `x +
  0x700000005`, `addze; addi 3,3,7` is `(u32)((x + 5) >> 32) + 7`),
  through truncations (SPARC's `smul`) and loop choices; a shift, mask,
  product, literal or wrong-signed flag on the way is a rework, and a flag
  that reaches the first register only through a piece of a wider value
  counts for nothing. `(u32)((x + y) >> 48)` is `addc 4,6,4;
  adde 3,5,3; srwi 3,3,16` on PowerPC, `-(int)((x + y) >> 32)` negates
  `adde`'s result, while a 64-bit `-x` (`subfic; subfze`, ARM's `rsbs; rsc`)
  negates a sum whose flag is a borrow and keeps the sign right. Flags are
  found through copies and the pieces of a register pair heritage splits (SPARC
  -O0 adds the halves of an `ldd` out of `%i2:%i3`). With no carry between the
  registers, the first register may not be a literal operation (a shift short
  of a sign fill, a mask, an offset, a negation, a product, an `or`/`xor` with
  a literal) on a value the low word never reads, or on what that value is
  worked out from by literal operations, rotates (PowerPC's `srwi` is
  `rlwinm`) and stack reloads: gcc -O0 on MIPS computes `(u32)((x ^ y) >> 32)
  & 0xffff` in `$2:$3` and masks `$2` alone; ARM's `umull r1,r0` leaves the low
  half of `(u32)((x * k) >> 32) & 0xff` in r1. `((u64)(v & 0xff) << 32) |
  (u32)(v * 7)` reads `v` in both words and keeps its fix, and so does a
  complement or literal mask both words get (`~v`); neither the clean
  high word of a sum or difference the low word holds through moves, choices
  or -O0 locals (`i64 tabs64(i64 a)` stores `a` and `-a`, a negation's
  borrow spelled `b != 0`) nor a literal low word is judged this way. The two
  registers are paired through moves and through choices made in one block,
  input by input, so a loop's accumulator keeps its fix; a low word chosen
  where the first register's value is not (bash's `intmax_t` evaluator) is
  judged per sum, a sum whose own high word is the first register counting as
  64-bit, and the first register is followed into a stack slot only when the
  slot is stored once (openssh's -O0 `get_u64` builds its value in an
  accumulator);
* the high half of a product beside a first register that reads its low half:
  clang -O0 on MIPS spills the `mfhi` of a 64-bit hash and reloads it into
  `$3`, and `$2` is `(u32)x`, built from the `mflo`;
* a literal zero proves nothing; a callee's clobber or a never-written location
  is nothing;
* anything else (a value or nonzero literal only the RETURNs read) is returned
  on purpose.

A function with an indirect jump flow could not follow (`Treating indirect jump
as call`) has code return recovery does not see, so there a low word nothing
visible reads vetoes the join like scratch when that code can read it (an
input, a literal, or a value made in a block that dominates the jump; a value
made after the jump on the returning path still counts). gcc copies an argument into `$3`
before a switch for the case that passes it on (`move v1,a1`); in an object
whose jump table is left to relocations that case is hidden, and iproute2's
`rt_addr_n2a_r` on MIPS -O2 returned its buffer argument instead of `"???"`
(`return a3;`). Its registers on the default path are those of `u64 zb(u32 a,
u32 b) { return b; }`, which keeps its fix: the rule looks at the unfollowed
jump, not at the registers.

The pair joins in the ABI's order when some RETURN holds a wide value or returns
its low word on purpose and none holds a leftover, scratch, a comparison's
leftover, a truncation, a quotient's, a shift's, a reworked temporary's or a
product's leftover there; otherwise it joins first register low,
byte-identical to main. A function whose pair keeps
the old join builds its calls' output pairs the old way too, so a call's pair it
returns stays one value. Everything else is main's code: the uncomputed-half
repair skips a pair joined first register high, and little-endian targets never
set the flag.

AVR adds one thing: with the order right its two registers are contiguous and
name `R25R24`, a global (AVR maps registers into data memory), and a value built
there printed as `return R25R24;`. `pair_join_address` keeps a join record when
the parent register is global storage.

## Why it is an option

The rule is a prior, not a proof: the same registers can hold an `int` or a
`long long`, and the rule reads each shape one way. So the whole order decision
ships as `option bejoin`, off by default (GH-904); off joins every pair first
register low, byte-identical to main (checked on all 1,151 functions of the
witness set, and on the big-endian corpus differential in `record.json`).

Read as the `long long`, and so wrong for an `int` that main printed right: an
`int` holding the high word of a 64-bit temporary whose stale low half stays in
the second register. These compile to the same registers as the `long long`
returning the whole value:

| `int` source | target | same code as |
|---|---|---|
| `(int)((a + b) >> 32)`, `(int)((a - b) >> 32)` | PowerPC, ARM BE, SPARC -O1/-O2 (gcc -O0 on MIPS moves the high word down with `srl $17,$2,0` and reads as the `int`) | `a + b` (`addc 4,4,6; addze 3,3; add 3,3,5`), `a - b` |
| `((u64)a * b) >> 32`, `(u32)(((u64)a * b + c) >> 32)`, `(u32)((x * 3) >> 32)` of a `u64` | ARM BE at every -O (`umull r1,r0` writes both registers), MIPS gcc -O0; the sum also PowerPC and SPARC | `(u64)a * b`, `(u64)a * b + c`, `x * 3` |
| `a / 641`, `a / 6700417` (a magic number that needs no shift) | ARM BE at every -O (`umull r1,r0,r0,r1; bx lr`) | `(u64)a * 0x663d81` |
| `(u32)(*p >> 32)` | MIPS gcc -O0 (both words of `*p` loaded into `$2:$3`) | `*p` |
| `(u32)((x + K) >> 32) + M` | ARM BE (`adds r1,r1,#K; adc r0,r0,#M` folds the literal into the carry add) | `x + (M << 32 \| K)` |

In the witness sets (w2 to w10, `big`, `ib`, w13 to w24 and `dv`: each source
built for PowerPC, SPARC, ARM big-endian at its default architecture, ARMv6
and ARMv7 and MIPS with gcc, w13 to w24 also with clang for MIPS, at -O0 to
-Os, and round-tripped with gcc and clang; the Thumb-2 big-endian builds count
for nothing, since kuna discovers no functions in them on main or here), these
are the 79 builds the option turns from right to wrong (`sum_hi` 18, `sub_hi`
9, `mulhi_k` 10, `xk_hi_add` 10, `umulh_sum` 8, `ud641` and `ud6700417` 7
each, `hi64k` 6, `mulhi`, `mulhi_s`, `mul_hi_s` and `hi_of_mul` 1 each),
against 1,193 it fixes. Every other `int` a 64-bit temporary's high half is
worked into prints as on main: the 106 builds of w22 (`(u32)((x + y) >> 48)`,
`>> 36`, `& 0xff`, `+ 5`, `-(int)(...)`, `* 7`, `((i64)a + b) >> 34`, ...),
w21's `sum3_shr33`, w18's `hash64` (MIPS clang -O0), the 20 builds of w23
(products and temporaries masked, offset or negated at -O0, `(u32)((x * k) >>
32) & 0xff` on ARMv6/v7 -O1 to -Os) and 63 of w24 (`(u32)((x + 5) >> 32)`
masked, shifted, negated or offset), 197 builds the previous version turned
wrong. Of its fixes, 54 builds go back to main's output: `ld_or` (`*p |
0xffULL << 32`, 28), `xy_hi_sh` (`((s >> 33) << 32) | (u32)s`, 23) and
`xk_max` (`(u64)a + 0x7fffffffffffffff` on MIPS gcc -O1 to -Os, which adds the
literal after the carry, 3); every function that moves against the previous
version prints exactly as on main. The comparison and truncation rules give
up `swap64` at PowerPC and ARM big-endian -O0, and the range-check rule
`zsum_chk`/`zs_chk2` on ARM big-endian -O1 to -Os. In 24,257 functions of nine corpora built for the
same targets (e2fsprogs, coreutils and gnulib, libexpat, libbsd, dpkg,
diffutils, findutils, sysvinit, Lua, libselinux, iproute2, rsyslog, openssh,
bash, cronie, libedit, grep, kmod, shadow, zlib, dash, gnutls, base-passwd,
gzip), 1,707 change against main (on 64d21a506; 1,708 on 72d2e9b6d, where
bash's `expbxor` on SPARC -O2 still kept its fix: from #832 on, its low word
reads as scratch on the previous version's code too). Against the previous
version two move,
both back to main's output (expat's `siphash24` on SPARC -O0, whose call
result neither version reads, and bash's `exp1` on MIPS clang -O2, whose call
pair and return are respelled together with the same value), and in the 6,573
functions of the focused big-endian set (SQLite, zlib, bzip2, AVR, Lua on
SPARC) none moves; so no function whose DWARF return type is `int` or a
pointer changes value, and every changed 64-bit return that returns both
words reads them in the right order, as the previous version's footprint
was checked. That is a measurement on these
corpora, not something the rule proves: an earlier version changed one such
function, iproute2's `rt_addr_n2a_r` (the unrecovered switch above), which
prints as on main. The rule still decides from the instructions alone, and
each new look at the `int` side found another shape (the table above grew one
row at a time), so the default is off (GH-904): a default must not turn a
correct `int` into a wrong `long long`. A sound default needs positive
evidence that the value is 64 bits wide (a declared, DWARF or asserted 8-byte
return type, callers that read both registers as one value, or a 64-bit
operation whose two halves both reach every RETURN unnarrowed).
`kuna-cli/tests/decompile_all_cli.rs`
`bejoin_reads_an_ints_stale_low_half_as_the_long_long_it_matches` records both
readings of `sum_hi`/`sum64` (PowerPC -O2) and `mulhi`/`uwide_mul` (MIPS gcc
-O0).

## What it does not fix

Read as the `int`, as on main, and so still wrong for the `long long`:

* `(u64)x << 32` leaves the same zero in the second register as an `int`
  function at -O0 (`hi_only` prints its high word, as on main).
* `(u64)a + b` of two 32-bit values keeps only the carry in the first register,
  the same code as the `int` carry test (`li 3,0; addc 4,4,5; addze 3,3` on
  PowerPC -O0), and reads as the comparison.
* A `long long` whose high word is a literal 0 or 1 a branch chose reads as the
  boolean beside a leftover, and one whose high word is a literal or a flag
  beside a low-word sum whose carry a branch tests (`u32 s = a + b; return s <
  a ? 0 : s;` returning `u64`) reads as the range check beside its leftover.
* -O0 `(v << 32) | (v >> 32)` of a 64-bit local reloads the local's two words
  in the order an `int` truncation of it does (`swap64` on PowerPC and ARM
  big-endian) and reads as the truncation.
* `((u64)(x ^ K) << 32) | K` (`K` also builds the first register) and
  `((i64)(n * K) << 32) | K` (`K` multiplied into it) look like scratch.
* A `long long` whose low word also feeds a call, a loop or a store
  (`long long k_cnt(int n)`'s accumulator) looks like scratch.
* A SPARC `int` function returning a constant or a value the rule pool folds
  into the pair before the repair runs (`return 0` with the second argument in
  `%i1`: `keep_zero`, `zero_after`, `one_after`) still prints the argument
  shifted into the high word, exactly as on main.
* An argument returned untouched as the low word (`k_b`: SPARC's `restore`
  hands `%i1` back unchanged, the same code as an `int` function that leaves
  its second argument there).
* A SPARC `long long` built from an argument moved into `%i1` in a call's
  delay slot (`long long widen(int x) { ext(); return x; }`:
  `call ext; mov %i0,%i1; sra %i0,31,%i0`). The call's p-code keeps a RETURN
  for a `restore` in its delay slot, never taken, and the argument also reaches
  that RETURN's first register, so it looks like scratch; the halves print
  swapped, as on main (#815).
* A `long long` sign-extended from an `int` at -O0 on MIPS, PowerPC and ARM
  big-endian (`lw $3,4($fp); sra $2,$3,31`): its reloaded low word never
  becomes an output trial, so only the high word is returned, as on main.
* A `long long` whose low word is a right shift of a value whose dropped bits
  also build its high word reads as the reverse-pair `int`.
* A `long long` returned in a function with an unfollowed indirect jump, whose
  low word nothing visible reads, reads as the `int` beside hidden scratch.
* A `long long` whose high word is a literal operation on a value its low word
  does not read (`*p | 0xffULL << 32`, which gcc -O0 on MIPS computes like
  `(u32)(*p >> 32) | 0xff`), or a reworked high word beside its sum's low word
  (`((s >> 33) << 32) | (u32)s`), reads as the `int` worked out from the high
  half of a temporary.
