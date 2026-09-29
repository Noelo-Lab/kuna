# conststr — analysis

A pointer-typed constant reaches `PrintC::pushConstant`'s pointer arm. Upstream prints a
character pointer as the string at its address when the string manager accepts the bytes,
and everything else as a forced-hex integer behind a cast; `globalref` (#723) names program
data `&dat_<addr>` unless one of its refusals fires. This lane asked what the constants that
still print as `(T *)0x<addr>` address, and which of them can soundly print as what they
address.

## Census (main 632437155, castbench shared set)

`census.py` finds every `(T *)<constant>` cast kuna prints on the 4,815 functions kuna and IDA
both emit (45 binaries: coreutils fmt/ls/sort/du/cp/tail/wc, grep, gzip, the four diffutils,
tar, find at O0/O2/O2-noinline) and classifies the address against the stripped binary's
sections and its DWARF twin's object symbols; `trace_run.py` reruns the corpus with
`KUNA_GLOBALREF_TRACE=1` and `join.py` attaches the refusal `globalref` gave each one.
IDA prints none of them (0 `(T *)<const>` casts).

| what the constant addresses | casts | globalref's refusal | IDA at that address |
|---|---:|---|---|
| `.bss` object | 438 | direct access 365, two pointee types 71, numeric 1, other 1 | a `qword_`/`word_`/`byte_`/`unk_`/`stru_` name (354) or a symbol name (84) |
| `.data` / `.data.rel.ro` object | 28 | direct access 22, two types 6 | an `off_`/`qword_`/`unk_` name |
| not an address (`0x1`, `0x1000`, `-1`, `0x7fffffff`) | 222 | outside every data section (or a code pointee) | an integer |
| `.text` (tar `v22 = (char *)0x1362e`) | 6 | outside data | an integer |
| a `.dynstr` bound, a one-byte suffix typed `long *` | 7 | outside data / not traced | an integer |
| a NUL-terminated string kuna did not render | **0** | | |

The string classes the brief expected (an empty string, non-printable bytes, a string inside
a larger object, a wide string) do not occur as casts on this corpus: every string a
character pointer reaches already prints as a literal, or, when its bytes are not UTF-8, as
`&dat_<addr>` (no cast). `strcensus.py` sweeps every function (not only the shared set) for
read-only addresses printed as a cast, `&dat_` or a bare integer:

- 60 `&dat_<addr>` are gnulib `gettext_quote`'s GB18030 quotes `"\xa1\ae"`/`"\xa1\xaf"`
  (every coreutils, diffutils, findutils, grep and tar binary, all three levels), which the
  string manager rejects as UTF-8. No cast, but the name hides the source's literal. IDA's
  decbench output does not include `gettext_quote` at any of those addresses.
- The tail-merged `""` appears in small programs: gcc -O2 and clang store the only `""` at the
  terminating NUL of another literal, and `emptystrconst` (a guard against a blob pointer
  printing as `""`) reads the bytes AFTER it, finds the next literal's non-text bytes, and
  declines. Witness: clang -O2 `nanf("")` printed `nanf((char *)0x2011)`; a stripped gcc -O2
  program's `use("")` printed `use((char *)0x2014)`.
- `&dat_<addr>` of a read-only zero row (`default_tuning`, `zero_buf`) or of binary bytes
  (`memcmp(v, &dat_20638, 2)` for `"\x1b["`, gzip's magic) have `void *`, `float *` or table
  pointees: not character pointers, correctly left alone.

The direct-access refusals, by what the direct access is (`dpairs.py`):

| shape | casts | sound to name? |
|---|---:|---|
| pointee `undefinedN`, direct access a known integer or pointer of that size | 84 | no: the value is kept, but `&dat_<a>` is not the parameter's C type (below) |
| a record whose members the function reads directly (`obstack`, `sigset_t`, `stat`, `struct_N`) | 162 | no: `dat_<start>` and `dat_<member>` would be two objects; needs member printing |
| gzip `outbuf`, a byte array with a 2-byte direct store (`put_short`) | 54 | no: another width |
| a byte array, direct access an unknown byte | 28 | no: an unknown byte is `char` on one surface |
| a signed object, direct access `undefinedN` | 25 | no: a zero-extension of an unknown prints bare and means unsigned |
| a signedness conflict (`uint8` vs `int8`) | 16 | no |

Of the 77 two-type refusals, 18 are an `undefinedN` pointee beside a known one of the same
size (`(unsigned short *)`, i.e. `undefined2 *`, beside `(short *)` in gzip's `bl_count`
loop; `(unsigned long *)` beside `(unsigned char **)`).

## What changes

`conststr on|off`, P9, default `on`. A character-pointer constant at a read-only address
inside a program-data section prints

- `""` at a NUL that terminates a string: the run of bytes ending at it is text (valid UTF-8,
  no control but `\t\n\r`), at least two characters, beginning at a NUL or filling the 64
  bytes read back (`kuna_conststr::terminates_string`). `emptystrconst`'s blob witness (the
  maze's zero row) is preceded by other data, not by a string's characters, and keeps its
  address.
- the literal of its bytes when the UTF-8 check rejects them (`byte_literal`): a valid UTF-8
  character as upstream spells it when that spelling is its own bytes, every other byte a
  `\x` escape, the literal split where a hex digit follows an escape. Byte-exact by
  construction.

Code is read-only too: without the data-section bound, `tail`'s file-system magic numbers
(`a0 == (char *)0x6969`, a compare on a mistyped `char *`) and tar's jump labels printed as
literals of instruction bytes in a first draft. And a pointer-aligned address whose word is
the address of program data or code (`is_image_pointer`) is a pointer table: kmod's command
table, which a function walks through a `char *` it also reads eight bytes at a time,
printed `"\xba\xb1\x01"` in a second draft and now keeps `&dat_27d40`.

## What was tried and dropped: naming the unknown-pointee objects

The 84 direct-access refusals whose pointee is `undefinedN` (plus 18 two-type refusals of the
same kind) look like the lever: the pointee says nothing, the direct reads say `long` or
`char *`, and IDA prints `&qword_846E8`. A first draft declared such an object at its direct
type and printed `&dat_846e8`, which removed 108 casts on the shared set (32,073 -> 31,965,
53 functions fewer, 0 more) and passed the value round trips. It is not C's own conversion:
`undefined8 *` is `unsigned long *` in C, so `&dat_846e8` hands a `char **` (or a `long *`)
to an `unsigned long *` parameter, an incompatible-pointer (or pointer-sign) diagnostic that
gcc 14 makes an error, where the cast it removed was exactly the conversion. kuna's own
cast policy prints no cast for a pointer variable in that position, but that is upstream's
convention, not a conversion C performs, and IDA keeps the cast too
(`sub_23BE0((void **)&qword_846E8, name)`, `sub_22570((char *)&qword_82408, ...)`).
`globalref`'s refusals are therefore right under the campaign's rule; the lever for these
casts is the callee's parameter type (a caller-voted `char **`/`long *`), not the constant.

## What stays

- Non-addresses (222): integers kuna typed as pointers; the fix is the type, not the constant.
- Records read member by member (162): printing `dat_2b460.__val[0]` needs a global symbol
  of the record type before the body prints; a separate lever.
- Unknown pointees beside a known direct type (84 + 18): the callee's parameter type.
- Signed/unknown and width conflicts (123): naming would change what a direct read computes.

## Measured (both arms of one build, main 63dfb436c + this branch)

- castbench full: 32,073 = 32,073 casts on the 4,815 shared functions (0.848x IDA), 0
  functions fewer, 0 more: on this corpus the strings kuna did not render never carried a
  cast. `conststr off` is byte-identical to the castbench main-632437155 arm on all 45
  binaries (so is main 63dfb436c: #748 changes none of them).
- `hunks.py`: 60 changed lines over 36 of the 45 binaries, every one a `gettext_quote`
  GB18030 quote (`&dat_<a>` -> `"\xa1\ae"`/`"\xa1\xaf"`), 0 other. Over 11 more binaries
  (bash, dash, kmod, crontab, dpkg-divert, cf2.elf, bzip2, useradd, stty, ip): 4 lines, dash's
  `strpbrk(a0,"\x81\x88")`/`strchr("\x81\x88",v7)` (its `{CTLESC, CTLQUOTEMARK}`) and a
  GB18030 quote pair.
- 444-slice typesweep and speed: record.json.
