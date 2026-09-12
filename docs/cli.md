# The `kuna` CLI reference

The user-facing commands are the single Rust binary `kuna`
(`decompiler/crates/kuna-cli`, built to `decompiler/target/release/kuna` by
`make binaries`). This is the full reference; the one-screen version is in
`docs/agents.md`.

**Every subcommand describes itself.** `kuna <subcommand> -h|--help` prints that
command's own usage block — its flags, what they take, and the shape of its `--json`
document — and exits `0` before anything is loaded, so it needs neither an input binary
nor a compiled `.sla`. `kuna --help` lists the subcommands. Help goes to stderr, as
every usage block in this CLI does.

All command output goes through a fallible stdout boundary. A downstream reader that closes
the pipe early is a normal terminal condition, not the `println!` panic (exit `101`) it used
to be: no panic text, no broken-pipe diagnostic. It suppresses the *diagnostic*, not the
*verdict* — the command still exits with the code its own work earned, so `kuna test | head`
on a regressed baseline exits `1` and the DIV-45 failure contract below holds with or without
a reader. Other stdout write failures are real errors: reported, and exit `1`.

An ELF whose **section table is unusable** is loaded anyway, from its program headers.
A corrupt `e_shoff`/`e_shnum`/`e_shstrndx` used to reject the whole file — every
command exited `1` with `not in recognized object file format: Invalid ELF section
header offset/size/alignment` on an image `readelf -l` reads happily. The section
table is link-time metadata; the entry point and the `PT_LOAD` map are not, so it is
dropped and the run continues, printing one line on stderr naming what was dropped
and what survived:

```
$ kuna functions ./sstripped --json
[kuna] ELF section table unusable (57007 section headers at e_shoff 0xdead run
2176129 bytes past the end of a 161156-byte file); continuing from the program
headers (entry 0x80492d0, 2 load segment(s))
```

Discovery then works from the executable `PT_LOAD` segments, so `functions`,
`decompile-all`, `disassemble` and `strings` all answer (`strings` reports
`"scanned": "segments"`). A UPX-packed image is left alone: it is section-less too,
but its load segments are a decompressor stub, and the zero-discovery diagnostic
pointing at `kuna unpack` is the more useful answer.

A PE whose **data-directory count does not fit its own optional header** is loaded
anyway, from the directories that are really there. `NumberOfRvaAndSizes` is
declared separately from the room `SizeOfOptionalHeader` leaves for the array;
Windows reads whichever is smaller, and packers overwrite the field, so a count of
1531532893 in a header holding 16 directories used to exit `1` with `not in
recognized object file format: Invalid PE number of RVA and sizes` before any code
was mapped. The count is clamped to what fits — the real import table, not a
fabricated one — and the run continues, printing one line on stderr:

```
$ kuna functions ./packed.exe --json
[kuna] PE NumberOfRvaAndSizes is 1531532893, but only 16 data directories fit in a
224-byte optional header; clamped to 16 (entry 0x40908e, 8 section(s))
```

## Where kuna finds the engine and the specs

`kuna` drives two sibling binaries — `decomp_dbg` (the engine behind `decompile`
and `catalog`) and `slacomp` (behind `kuna specs`) — plus a compiled SLEIGH tree.
Both shipped layouts resolve with no configuration:

- **a release archive**: `kuna`, `decomp_dbg` and `slacomp` extracted side by side
  (on Windows as `kuna.exe`, `decomp_dbg.exe`, `slacomp.exe`), with the
  `kuna-v<ver>-specs` archive's `specs/` tree beside or inside that directory;
- **a repo checkout**: `decompiler/target/<profile>/`, with the repo's `specs/`.

Each probe tries the bare name and the platform's executable suffix, so a Windows
install resolves the same way a unix one does. A probe that finds nothing names
the directories it looked in.

| Variable | What it pins |
|---|---|
| `KUNA_SPECS` | The SLEIGH tree passed as `-sleighpath` (`SLEIGHHOME` is the engine's own name for the same thing). |
| `KUNA_DECOMP_DBG` | The `decomp_dbg` binary. `kuna decompile --decomp-dbg PATH` does the same for one run. |
| `KUNA_DECOMP_TEST` | The `decomp_test_dbg` datatest harness (`kuna test --binary PATH`). |
| `KUNA_SLACOMP` | The `slacomp` binary `kuna specs` runs. |
| `KUNA_ROOT` | The repo root, when the binaries are not in-tree: `specs/`, `tests/datatests/` and `decompiler/target/<profile>/` are all read from it. |
| `KUNA_RUST_PROFILE` | The `decompiler/target/<profile>` directory the in-tree fallback reads (default `release`). |

An override wins over both layouts, and one pointing at nothing is reported as
such rather than silently re-probed.

## `kuna test` — the parity gates

```bash
kuna test --all --baseline docs/baseline.json          # expect: PARITY OK
kuna test --datatests --json                           # machine-readable
kuna test --datatests --datatests-dir tests/stages \
    --baseline docs/baseline-stages.json               # the stage-issue corpus (= make test-stages)
```

`kuna test` parses the harness's two streams separately (unit results on **stderr**,
datatest results on **stdout**) and exits nonzero on any failure or baseline regression.
`--save-baseline PATH` re-records a baseline. Routine use: `docs/baseline-stages.json`
when adding stage tests. `docs/baseline.json` is re-pinned only for sanctioned intentional
changes (an upstream sync per `docs/history.md`, a DIV-recorded default flip) — never to
absorb a regression.

## `kuna decompile` — one function

```bash
kuna decompile ./a.out main
kuna decompile ./stripped.bin 0x401040 --addr
kuna decompile ./a.out main --option compareform canonical
kuna decompile ./sparc.elf main --option returnpair single
kuna decompile ./a.out main --language rust
```

Drives `decomp_dbg` as a subprocess and captures `print C` via `openfile write`, so
interactive prompts never pollute the output. `--option NAME VALUE` (repeatable) and
`--kassert "<args>"` flip phase-model sub-phase assertions per run; `--mode
auto|reliable|aggressive|fast` applies an option preset (`docs/modes.md`).

An option **name** is checked before anything is loaded, on every surface that takes
`--option`, and an unrecognized one exits 2 rather than running as if it had not been
given. Names are case- and separator-sensitive, so the message names the nearest
catalogued spelling when there is one:

```bash
kuna decompile ./a.out main --option LOWEREDSWITCH off
#   error: option LOWEREDSWITCH: Unknown option (did you mean "loweredswitch"?);
#          `kuna catalog` lists every settable name
```

**A generated name is a selector.** kuna calls a function no symbol covers
`sub_<addr>`, and it prints such a name for entries the whole-binary inventory
does not hold — a recovered tail call renders `sub_1170(a0)`, and `kuna strings`
names a literal's owner from the reference walk's own flow attribution. Those
names now select:

```bash
# `kuna strings` reported sub_100a3be as the owner of "No error information";
# before, only the address form reached it
kuna decompile ./graphy sub_100a3be
#   char * sub_100a3be(unsigned int a0) { ... }
```

The name is read as the address it spells only when some naming style would
*mint* it there, and only when that address holds mapped bytes — so it lands on
exactly what `--addr` on the same address lands on, and a real symbol spelled
that way still wins. All three styles count, not just the run's own, because
`--option namestyle` decides how a generated name is *printed* and `kuna
functions` reports the default spelling whatever the run asks for:

```bash
kuna functions ./a.out --json          # ... "name": "sub_15dc" ...
kuna decompile ./a.out sub_15dc --option namestyle ghidra
#   unsigned long sub_15dc(void) { ... }   # same function as func_0x000015dc
```

Anything else keeps the by-name miss:

```bash
kuna decompile ./graphy sub_deadbeef
#   error: no function "sub_deadbeef" in ./graphy; for a stripped binary pass
#          an address with --addr
```

**An import name selects the code half.** A dynamically linked image spells an
import's name twice — on the forwarding veneer a direct `call` targets, and on
the IAT/GOT slot that veneer reads — so `strcmp` matches two entries. The
executable one wins: the veneer is the only candidate that has a body, so the
selector is answered there rather than refused.

```bash
kuna decompile ./mach-o-crackme strcmp
#   int strcmp(char *a0,char *a1) { ... }      # the __TEXT,__stubs veneer
```

Both entries stay in `kuna functions` — a slot with no veneer (`__DATA,__got`,
`__DATA,__nl_symbol_ptr`, an IAT entry a `call [slot]` reaches directly) is the
only place its name appears at all — and every surface that takes a name
(`decompile`, `decompile-all --functions`, `disassemble`, `read`, `xrefs`)
resolves it to the same veneer. The narrowing needs **exactly one** executable
candidate, so two same-named definitions in different code sections of a
relocatable object still report the ambiguity with every candidate listed.

**The instruction budget.** Flow following decodes at most `maxinstruction`
instructions per function — 100000 by default, which no ordinary function comes
near and an obfuscated one blows through. Past the budget the decompiling
surfaces **truncate** the flow and emit the body they did decode, under a
warning header that says so:

```bash
# a 1.8M-instruction MBA-obfuscated checker: a truncated body, not a failure
kuna decompile-all ./crackme.exe --functions sub_140001000 --json
#   "code": "unsigned int sub_140001000(...)  // warn: Exceeded the 100000
#            instruction budget: some flow is truncated ..."

# ask for more of it (the cost is roughly linear, in time and in memory)
kuna decompile-all ./crackme.exe --functions sub_140001000 --option maxinstruction 400000 --json

# or make the overrun fatal again, which is upstream's policy and the engine default
kuna decompile ./crackme.exe main --option errortoomanyinstructions on
#   error: Flow exceeded maximum allowable instructions
```

Both options are upstream `OptionDatabase` names rather than phase-model ones, so
they are reachable through `--option` on every surface but do not appear in `kuna
catalog`. `--max-fn-seconds` (see `decompile-all` below) is the wall-clock half of
the same budget.

**`--define-function <start[-end][=name] | @file>`** (repeatable) tells kuna where a
function starts and ends. Every boundary kuna knows is otherwise *derived* —
discovery finds the entries, and the extent is the address-contiguous clip to the
next one over an unbounded flow follow — which is the wrong answer on exactly the
images where reverse engineering is hard. A missed entry merges two functions into
one; a phantom one invents a function that is not there.

```bash
# an entry discovery missed: name it and decompile it
kuna decompile ./packed.bin 0x4014a0 --addr --define-function 0x4014a0=stage2

# two functions merged into one: say where the first really ends
kuna decompile ./packed.bin --addr 0x4013c9 --define-function 0x4013c9-0x401420=stage1

# keep the boundaries you worked out, and pass them to every later command
cat > bounds.txt <<'EOF'
# recovered by hand from the unpacked image
0x4013c9-0x401420 = stage1
0x401420-0x401500 = stage2
EOF
kuna functions ./packed.bin --json --define-function @bounds.txt
```

`start` declares the entry: it gets a function symbol (so call sites name it), it
enumerates in `kuna functions`, and it resolves by name. `end` is **exclusive** and
declares the extent: flow following stops there, so the body no longer swallows its
neighbours, and the extent reported by `kuna functions --json` is the declared one
rather than the clip. `=name` is optional and names the entry (an entry the image
already named keeps its name unless you supply one); `end` is optional too — a bare
`--define-function 0x4014a0` asserts an entry and leaves the extent natural.
Addresses are hexadecimal with or without `0x`.

A name that one of kuna's built-in libc signature tables knows also carries that
signature to the entry, so declaring the callee gives its call sites a prototype
and not just a spelling — `--define-function 0x8048968=ptrace` on a stripped,
statically linked image turns `ptrace()` into
`ptrace(v18[-0xc],v18[-0xb],(void *)v18[-10],(void *)v18[-9])`. The load-time
prototype passes cannot reach this case: they match a name the *image* carries,
and a stripped image carries none. An explicit `--assert prototype` on the same
function still wins, and `--option declaredlibcproto off` declares the name alone.

A declared `end` that cuts real control flow is reported rather than silently
truncating the body: the function carries a `// warn: Function flows out of bounds`
comment on its prototype and one at each cut edge, naming the address the edge left
for. That holds for a conditional branch over the end as much as for fall-through
past it — including a branch to the exclusive end itself, which is what a
tail-clipped `if (err) goto fail;` looks like. A correct boundary ends in a return
and produces no warning, so that comment is the signal to widen the range.

A declaration also survives the whole-binary filter. Unfiltered `decompile-all`,
`decompile-project` and the browser inventory decompile only entries inside a
section flagged executable, which is a guess a packer defeats simply by not
setting the bit — a NEOLite-packed PE flags all six of its sections
`INITIALIZED_DATA|READ|WRITE`, `.text` included. There `kuna functions` listed the
declared entry and `kuna decompile` emitted its body while `kuna decompile-all`
with the same flag answered `count: 0`. A declared entry is now kept whatever the
section flags say; nothing undeclared is lifted with it. And a run that finds
nothing in an image which declares where execution starts is reported as the
failure it is, rather than as an empty success:

```console
$ kuna decompile-all ./packed.exe --json
error: no functions discovered in ./packed.exe: its entry point 0x4f7001 lies in
section .NEOpack, which the image does not flag executable -- pass
`--define-function 0x4f7001` to decompile there anyway
```

The `@file` form is the durable one: one declaration per line, `#` comments and
blank lines skipped. kuna does not write boundaries back into the image, so the file
is the artifact — generate it, diff it, and pass it to every invocation. The flag is
accepted by `decompile`, `decompile-all`, `functions`, `decompile-project` and
`disassemble`; a declaration is applied after analysis has had its say, so it
overrides discovery rather than competing with it. The console spelling, for a
hand-driven `decomp_dbg` session, is `function bounds <start> [<end>] [as <name>]`.

**`--assert <directive> | @file`** (repeatable) is the other half: where
`--define-function` tells kuna where a function *is*, `--assert` tells it what
anything *is*. Everything kuna knows it derived, and until this flag the only
levers the `kuna` binary offered were `--option` and `--kassert` — the console has
carried `rename`, `retype`, `map param`, `map return`, `map address`,
`comment instruction` and `parse line extern` all along, unreachable.

```bash
kuna decompile ./a.out authenticate --json \
  --assert 'prototype authenticate int authenticate(char *user,char *pass)' \
  --assert 'type v2 char[16]' \
  --assert 'name v2 credbuf'
```

```text
- unsigned long authenticate(char *a0,char *a1)     - char v2 [8];
+ int authenticate(char *user,char *pass)           + char credbuf [16];
```

One directive per `--assert`, keyed by intent rather than by phase:

| directive | what it states |
|---|---|
| `prototype <func> <C declaration>` | the signature of `<func>` — a name or an entry address (parameter names and calling convention included) |
| `param [<func>::]<i> <storage> <C typedecl>` | the storage and type of one input |
| `return [<func>::]<storage> <C typedecl>` | the storage and type of the return value |
| `name [<func>::]<symbol> <newname>` | rename a local |
| `type [<func>::]<symbol> <C type>` | retype a local |
| `typedef <C declaration>` | intern a `struct`/`union`/`enum`/`typedef` so `type` can name it |
| `data <addr> <C typedeclaration>` | a named, typed global at an address |
| `comment [<func>::]<addr> <text>` | a comment rendered into the C at that instruction |
| `flow [<func>::]<addr> branch\|call\|callreturn\|return` | the flow out of this instruction is not what kuna decided |
| `function <start>[-<end>][=<name>]` | the `--define-function` spelling, on this plane |
| `readonly <addr>+<size>` | the bytes in this range never change at run time |
| `volatile <addr>+<size>` | device memory: every access is a real access |
| `bytes <addr> <hex\|@FILE>` | the bytes mapped here are these, whatever the file holds |

Storage is a register name (`RDI`), the console's `%RDI`, or its address grammar
(`[stack,-0x18,8]`). Addresses are hexadecimal with or without `0x`. A size is
decimal unless it carries a `0x`, and `<addr> <size>` is accepted wherever
`<addr>+<size>` is. A C type may be anything the console's `parse line` accepts,
including a `typedef` you asserted earlier in the same run.

**`name`/`type` reach the register locals, not just the stack ones.** A local
kuna prints with a storage comment — `unsigned long v6; // rax` — has no symbol
behind it; naming one used to answer `No symbol named: v6` no matter how it was
spelled. It now resolves: the directive maps a locked symbol over that
variable's storage at its use point, and the second pass types it there.

```bash
kuna decompile ./graphy sub_1005350 --json --assert 'type v6 unsigned long *'
#   unsigned long *v6;  // rax        (was: unsigned long v6;)
```

Two consequences worth knowing before you use it. A bare `type <local> <T>`
states no name, so the retyped local may come back under a different `vN` — the
storage comment identifies it across the two passes, and
`type v6 unsigned long *vmtop` pins a name outright. And a local the decompiler
holds in a temporary rather than in a register or on the stack (kuna prints those
without a storage comment) has no location a symbol can be mapped to; the
directive is `rejected` with `Not addressable storage` rather than accepted and
dropped.

**Write the type in C.** The standard scalar keywords — `void`, `char`, `short`,
`int`, `long`, `float`, `double`, `signed`, `unsigned`, `_Bool`, `wchar_t` — are
accepted in any legal combination, in return position, in parameter position and
as a `type`/`param`/`return`/`data` operand, so a declaration kuna emitted can be
pasted straight back at it:

```bash
kuna decompile ./a.out sub_140004dcc --json \
  --assert 'prototype VirtualAlloc void *VirtualAlloc(void *p,unsigned int n,unsigned int a,unsigned int b)'
```

**A parameter may be named after a type.** kuna interns a type called `code`,
and a `-g` binary interns every DWARF type name it uses, so the name you want
for a parameter is often already a type name; the declaration still reads it as
the parameter's name, exactly as C does:

```bash
kuna decompile ./graphy 0x1005350 --addr --json \
  --assert 'prototype 0x1005350 unsigned long vm(unsigned char *code,unsigned int index,void *ctx)'
```

Only the parenthesised form stays a type: `int4 (code)` is a function of one
`code`, which is what C reads it as.

A tag you declared earlier in the same run is usable as a type, in the `struct
X` / `union X` / `enum X` spelling as well as by bare name — including as a
return type, which is how a function returning a small struct by value gets its
field accesses back:

```bash
kuna decompile ./qjs main \
  --assert 'typedef struct JSValue { unsigned long payload; long tag; };' \
  --assert 'prototype sub_875e0 struct JSValue JS_ReadObject(void *ctx,char *buf,unsigned long len,unsigned int flags)'
```

**`<func>` is what the prototype binds to, not the name inside the
declaration.** The reason to state a signature at all is usually that the
function has no name worth keeping, so the declaration gets written under the
name the function deserves:

```bash
kuna decompile ./rage.exe sub_1400055e0 \
  --assert 'prototype sub_1400055e0 void *sha256(void *out,void *input)'
```

```text
- void sub_1400055e0(unsigned int *a0,unsigned long long *a1)
+ void * sub_1400055e0(void *out,void *input)
```

The declaration supplies the types and the parameter names; `sha256` names
nothing (`--assert` cannot rename a function today — `name` renames a local).
The console spelling is `map prototype <func> <C declaration>`, which is why
`<func>` survives into a hand-driven `decomp_dbg` session too; `parse line
extern <decl>` binds by the declared name and can only confirm a signature for
a function that is already called that.

**`<func>` may be an entry address instead of a name**, in `prototype` and in a
`param`/`return` qualifier alike. A name is tried first, so nothing changes for
a function you can name; an address is what you need when you cannot, and it is
the only way to say which of two same-named functions you mean — a PE import
thunk and the IAT slot it jumps to are both called `sqrt`, and every call in the
program goes to the thunk:

```bash
kuna decompile ./KeyCheker.exe sub_140001890 \
  --assert 'prototype 0x140003ddf float8 sqrt(float8 x)'
```

```text
- v26 = (double)sqrt();
+ v26 = sqrt(v35._0_8_);
```

A `0x`-prefixed operand that starts no function is **rejected**, naming the
address, rather than accepted and dropped. The address you selected with `--addr`
always counts as one: pointing `--addr` at an address declares that a function
starts there, so a directive may name the very address this run is decompiling
even when discovery never found an entry at it.

```bash
kuna decompile ./illusion.exe 0x401571 --assert-strict \
  --assert 'prototype 0x401571 void decrypt(unsigned int key, unsigned int start, unsigned int end)'
```

```text
- void sub_401571(unsigned int a0,int4 a1,int4 a2)
+ void sub_401571(uint4 key,uint4 start,uint4 end)
```

**A prototype may name its calling convention**, in either of the two C
positions — before the return type, or between the return type's `*` and the
function name, which is where Windows headers put it:

```bash
kuna decompile ./Cube.exe sub_401ba0 \
  --assert 'prototype 0x4050a6 void * __stdcall LoadLibraryExW(unsigned short *n,void *f,unsigned int g)'
```

The conventions you may name are the ones the target's compiler spec declares —
`__stdcall`, `__cdecl`, `__fastcall` and `__thiscall` on x86 Windows,
`__stdcall`, `MSABI` and `syscall` on x86-64 gcc. The convention decides where
each argument lives, so declaring one on a callee changes what the caller's
arguments resolve to. A spelling the spec does not declare is **rejected**
rather than ignored, so a misremembered convention name cannot silently leave
the default in place.

Widths come from the target's own compiler spec, so `long` is eight bytes on LP64
and four on LLP64. Ghidra's sized spellings (`int4`, `uint8`, `float8`,
`undefined`) still work and take precedence for a name the type factory already
knows. A combination that is not a C type (`short long`, `float int`) is rejected
by name.

**The two range directives are for memory kuna cannot classify by itself**, which
on a hostile or embedded image is most of it. `--option readonly on|off` is a
program-wide switch, not a range, and the loader's own read-only markup stops at
what the section flags say:

```bash
# `.data` is writable, so the loader never calls it read-only -- but nothing in
# this program writes these eight bytes, and the agent has checked.
kuna decompile ./fw.elf sample --assert 'readonly 0x404028+8'
#   - return scale * a0 + bias;
#   + return a0 * 7 + 100;

# 0x50000000 is a device register. Two reads of it are two reads; without this
# they are two loads of one unwritten address and CSE merges them.
kuna decompile ./fw.elf sample --assert 'volatile 0x50000000+4'
#   - return dat_50000000 * 2;
#   + v1 = dat_50000000; return v1 + dat_50000000;
```

Asserting a `readonly` range turns read-only propagation on for the run, because
painting a range read-only and then not folding it would be a directive that is
accepted and does nothing. It is applied *before* your own `--option`s, so an
explicit `--option readonly off` still wins.

**`bytes` is for code that does not exist in the file.** A packer's plaintext
appears only once the packer has run, so nothing a loader can read will ever
contain it; without this directive the recovered layer had to be written back
into a *copy* of the executable with an external script, and every later run
started from that copy instead of from the original plus a statement.

```bash
# `decrypt_stage1` rewrites the 2967 bytes above itself. Hand the recovered
# plaintext back at the addresses it belongs to and the next layer decompiles.
kuna decompile ./crackme.exe 0x43d0c6 --addr --assert 'bytes 0x43d0c6 @notes/stage1.bin'
#   - void sub_43d0c6(void) { return; }        // the ciphertext decodes to nothing
#   + *(int *)(v2 + 0x403393) = v2 + 0x40294c; // the stage-2 body
```

The payload is one unbroken run of hex digits (an optional `0x` is allowed), or
`@FILE` for raw bytes — which is what makes a whole recovered layer a single
directive. The overlay lives in the load image, so **the file on disk is never
touched**, and `--assert @overrides.kuna` beside a `function <start>-<end>=<name>`
line for each recovered function is the durable form: state the bytes and the
boundaries once, replay them on every run.

The statement must land before anything reads those addresses, so it is applied
ahead of the analysis commit. Two consequences worth knowing: an address no
loaded segment maps is *rejected*, naming the span, rather than invented; and
function discovery has already run on the file image, so functions inside a
recovered layer are yours to declare (`function`/`--define-function`) rather than
kuna's to find.

**`flow` is the structuring lever**, and the one directive that changes which
bytes are even *in* the function. kuna decides at P2 whether an instruction
branches, calls, calls-and-does-not-return, or returns; on an obfuscated or
hand-written image it gets that wrong, and everything downstream inherits the
mistake. Stating the right answer costs one line:

```bash
# `sub_13c9` reaches an indirect `call *%rdx` that never comes back, so flow
# walks on into its twenty-four neighbours and the body is 25 dead temporaries.
kuna decompile ./a.out --addr 0x13c9 --json --assert 'flow 0x1405 return'
#   - v2 = (**(void **)(...))(dat_4014); v3 = sub_1129(v1); ... return v2 + v3 + ...;
#   + return dat_4014;
```

The four words are the console's own (`Override::stringToType`): `branch` reads
the instruction as a jump — which is what puts an indirect call back through
switch-table recovery — `call` as a call, `callreturn` as a call whose
fall-through is dead (the "does not return" case), and `return` as the end of the
function. A type the engine cannot apply at that instruction (`call` on an
indirect call has no destination to make direct) is neither silently dropped nor
fatal to the function: the override is rejected, the body you would have got
without the directive still comes back, and the run exits non-zero with the
engine's own reason.

```console
$ kuna decompile ./a.out --addr 0x13c9 --assert 'flow 0x1405 call'; echo "exit $?"
int sub_13c9(void)
...
error: --assert "flow 0x1405 call" refused by the pipeline: Could not apply flowoverride (the C below was produced WITHOUT it)
exit 1
```

**A `call` override follows the RET-call chain it starts.** `push
<continuation>; push <target>; ret` is a call written without a `call`
instruction: the `ret` pops `<target>` and jumps to it, and the callee returns to
`<continuation>`, which is the instruction right after the `ret`. Packers and
crackmes build whole bodies out of them, one link per callee. Reclassifying one
link recovers one call — and the next link is still a `ret`, so the rest of the
body is dead and prints as `return;`, with nothing to say the chain continues.
So `call` and `callreturn` walk the chain from the link you named and plant
themselves on the rest of it. The walk follows unconditional jumps, falls through
everything else, and stops at the first `ret` the run reaching it did not push a
continuation for — an ordinary epilogue, which never pushes the address of the
instruction after itself, so an ordinary `flow <addr> call` extends to nothing.
The console prints the links it added.

```console
$ kuna decompile ./bm3.exe 0x401757 --addr --assert 'flow 0x40176f call'
-  void sub_401757(void) { LoadLibraryA(s_40151e); }
+  dat_40151a = GetProcAddress(LoadLibraryA());
+  ...  VirtualProtect(...); CreateFileA(...); ReadFile(...);   /* 21 more links */
```

**Every directive's fate is reported.** `--json` grows an `assertions` array — one
row per directive, in the order you gave them, carrying the directive text, its
phase and sub-phase, `applied` or `rejected`, and a reason:

```json
{"directive": "name v9 credbuf", "kind": "name", "phase": "P9",
 "subphase": "naming-policy", "status": "rejected",
 "detail": "No symbol named: v9", "fatal": false}
```

A rejection is also printed on stderr, on both surfaces. It is **not** fatal by
default — a batch of forty renames against a re-decompiled binary must not lose the
other thirty-nine to one stale name — and `--assert-strict` makes any rejection
exit non-zero.

`"fatal": true` is the exception, and it exits non-zero on its own. It marks a
directive kuna accepted and then **refused while applying it** — today a `flow`
override the flow-follower could not apply. A rename that did not bind leaves a
correct body one annotation short and you can see which; a refused flow override
leaves C that describes a different control-flow graph than the one you asked for
and looks exactly like the C you wanted, so it is reported as the run's verdict.

**Order matters, and so does scoping.** Directives are applied in the order given:
`type v2 char[16]` then `name v2 credbuf` retypes and then renames, where the
reverse leaves the second naming a symbol the first renamed away. `name` and
`type` name a *local*, which does not exist until the function has been decompiled
once, so kuna decompiles it twice — but only when such a directive is present, so
nothing else pays for it. A directive that names no function binds to the function
being decompiled; on a run that decompiles more than one (`decompile-all`,
`decompile-project`) it is rejected rather than applied to every function that
happens to have a `v2`, so qualify it:

```bash
kuna decompile-all ./a.out --json --assert 'name authenticate::v2 credbuf'
```

**A qualifier names the function the directive describes, which need not be the
one being decompiled.** `param` and `return` describe a *signature*, and a
signature is exactly the thing a caller needs to know about its callee, so
qualifying one with a callee's name states that callee's prototype and the call
site renders against it:

```bash
kuna decompile ./maze.exe sub_402020 \
  --assert 'param sub_401c50::0 ECX char *maze' \
  --assert 'param sub_401c50::1 EDX char *moves'
```

```text
- v15 = sub_401c50();
+ v13 = sub_401c50((char *)maze,moves);
```

The declared storage is the storage the argument is read from, which is what
makes a non-default convention statable at all: `ECX`/`EDX` above is a
`__fastcall` callee, and the same two directives spelled against stack storage
render different arguments. Slots may be declared in any order and one you do
not name is `undefined`; declaring them all is the same statement as a
`prototype` directive with the storage added.

`comment`, `flow`, `name` and `type` describe the inside of one function body
and have no cross-function reading, so qualifying one with a function this run
did not decompile is **rejected** — with a `warning:` line naming it, and a
non-zero exit under `--assert-strict` — rather than quietly applied to the
function that was selected.

A range property is painted before the image's symbols are mapped, because
mapping a symbol folds the property into it and never consults the range again —
so a range you state is honoured even where the loader already gave the address a
name. There is deliberately **no** `global` directive: `global add` is the console
command that would carry it, and every stock cspec's `<global>` already claims the
whole default data space (`<range space="ram"/>`), so on any ordinary image the
range is global before you say anything. `global add`/`global remove` are wired
and usable from `decomp_dbg` (the removal direction is the one that moves the C),
but a directive that is accepted and inert has no place on this plane.

The `@file` form is the durable one, exactly as for `--define-function`: one
directive per line, `#` comments and blank lines skipped, and the file is the
artifact — kuna does not write assertions back into the image.

```bash
cat > overrides.kuna <<'EOF'
# worked out from the strings and the xrefs
prototype sub_401200 int check_license(char *key,int len)
name sub_401200::v3 keylen
type sub_401200::v2 char[32]
data 0x601048 char *expected_key
flow sub_401200::0x40123f return   # the dispatch tail never comes back
readonly 0x601050+16   # the key table, written only by the installer
volatile 0x40021000+4  # RCC->CR
EOF
kuna decompile ./a.out sub_401200 --json --assert @overrides.kuna
```

Accepted by `decompile`, `decompile-all`, `decompile-project` and `functions`.
The console spellings, for a hand-driven `decomp_dbg` session, are the commands in
the table's second column.

**Paths containing spaces work (DIV-100).** This is the one surface that reaches the engine
through a console *script* rather than an in-process call, and the console reads a
filename with `s >> filename` — whitespace-delimited. An unquoted path with a space
therefore split into two arguments: `load file` took the head as a BFD target and
loaded the tail, and `openfile write` truncated the redirect at the split, writing
the C to a file named after the first component. The CLI now quotes a path that
needs it, and the console's `read_filename` accepts a double-quoted argument
(`\"` and `\\` are escapes inside quotes; any other backslash is literal, so a
Windows path survives either spelling). Unquoted paths parse exactly as before.
Hand-written console scripts and interactive `decomp_dbg` sessions get the same
grammar — quote the path when it contains a space:

```
load file "/home/u/test dir/a.out"
openfile write "/tmp/out dir/main.c"
```

**`--language auto|c|rust`** selects the output language. **`auto` is the
default and follows the binary**: a Rust binary renders as Rust, because kuna
already detects one (`kuna-analysis`'s `sourcelang` pass, the port of Ghidra's
`SourceLanguageAnalyzer`) and rendering it as C is worse in a way the reader has
to undo by hand (DIV-80). Detection is high-precision, not heuristic; an
unreadable file leaves C in place; and `--language c` always wins, so the policy
can only ever add a language. It lowers to the upstream `option setlanguage`, so
`--option setlanguage rust-language` is equivalent; an unknown name is an error
rather than a silent fall back to C. `decompile-all --json` reports the resolved
choice in a top-level `"language"` key. The same recovered function is rendered
through a different profile — types, structuring and analysis are identical —
producing `unsafe fn n(mut a0: i64) -> u32`, `let mut v: T;` declarations, Rust
primitive spelling, `x as T` casts, `loop`/`while c {}`, and `match v { A | B =>
{ … } _ => {} }`. The contract is `syn::parse_file` validity, not `rustc`
compilation: the output calls functions that have no definition and does no type
checking. Constructs Rust cannot express — an unstructured `goto` the structurer
could not remove, a C switch fall-through — render as a comment plus a diverging
`panic!("kuna: …")` so a lossy site is never mistaken for a translation; grep
that marker to measure them, and `--option gotoreduce on --option taildup on
--option ifelseflatten on` to reduce them. `--language` also works on
`decompile-all`; `decompile-project` is C-only -- it never auto-selects, and errors on an
explicit non-C language -- and the Ghidra front-end pins its markup document to
C. The browser decompiler carries the same three choices in its **Language**
control. See `docs/spec/09-emission.md` §9.6.
Omitting `--mode` selects `auto`: files below 500 KiB use `aggressive`, files
from 500 KiB up to 2 MiB use `reliable`, and files at least 2 MiB use `fast`.
The raw on-disk byte length is used, with exact cutovers at 512,000 and
2,097,152 bytes. A later explicit `--option` wins over the resolved preset.
Address-selected single-function decompilation suppresses a preset-provided
`fast_funcdisc` whole-image walk because the requested entry is already known.
Name selection keeps it enabled so generated `sub_<addr>` names can resolve;
explicitly spelling `--option fast_funcdisc on` opts an address run back into
that analysis.

`--target <SLEIGH-language-id>` selects the decoder while a recognized object
container continues to own its section mappings and image base. `--target default`
requests automatic language selection, including the normal compiler-model fallback.
The decoder's instruction width, architecture and endianness need not match what
the container's header declares; for example, ELF32 can carry 16-bit x86 code. An
endian disagreement is reported on stderr and the requested decoder is still used.
This also permits a valid PE/COFF image whose machine value is newer than the object
parser's architecture table to load under an explicit language. PE/COFF machine
`0x01c2` is recognized directly as little-endian ARM32, including bare COFF
objects whose machine prefix the generic object parser does not recognize.

`--isa auto|arm|thumb` is available on `decompile`, `decompile-all`,
`decompile-project`, `decompile-graph`, `functions`, `disassemble`/`read`,
`strings`, and `xrefs`. It controls ARM32's per-address `TMode`
context, which a language id alone cannot select. `auto` is the default: kuna
uses ELF mapping/FUNC markers, Cortex-M metadata, and Thumb-specific PE/COFF
machine values when the resolved decoder is ARM32. These inferred hints preserve
an explicit non-ARM `--target`; explicit `--isa arm|thumb` still requires ARM32.
Explicit `arm` or `thumb` takes precedence over that metadata
during discovery, cross-reference analysis, and graph assembly as well as
decompilation. On ELF images without section headers, explicit ISA context covers
executable `PT_LOAD` ranges. Fixed-A32 languages without `TMode` accept `--isa arm`
without a context paint and reject `--isa thumb` with a target-selection diagnostic.
With no mode evidence, decoding uses the selected language's default context; use
`--isa` to select another mode. `--isa` is refused where it could not apply —
`strings --no-xrefs` decodes nothing, so the pair is a usage error rather than a
silently dropped flag.

### UEFI TE images

UEFI TE images are recognized directly on `decompile`,
`decompile-all`, `functions`, `disassemble`/`read`, `decompile-project`, the
interactive console, and the browser front-end, which all load through one
dispatcher. The TE header owns the image base, entry point, and section
mappings; `--target` may select a compatible SLEIGH language but does not
reinterpret those addresses. Kuna applies the TE stripped-header bias to
section file offsets while retaining their original loaded VMAs. Recognition reads the `VZ`
signature, the machine word, and `StrippedSize`, so a file that merely opens
with those two letters keeps the headerless-image guidance. The retained
TE header, section table, and padding are mapped read-only from
`ImageBase + StrippedSize - 40` up to `ImageBase + BaseOfCode`; the file backs
that region only up to the first section's adjusted raw offset, and the
alignment slack behind it reads as zero, so an ordinary 0x200-file-aligned
image loads. Kuna validates that the file-backed part lies inside the file and
that no section's raw data starts inside the section table. It validates the two data directories and every other
header/section/file range before allocation, and seeds the container entry as
the initial function using the active `namestyle`. A `bytes` assertion or
console `override bytes` can replace a complete span within one TE mapping,
including its zero-filled virtual tail; a span crossing the mapping boundary is
rejected without changing any bytes. A nonzero section `VirtualSize` bounds its
mapped extent even when `SizeOfRawData` includes file-alignment padding; a zero
`VirtualSize` falls back to the raw extent, and a virtual tail beyond the raw
bytes is zero-filled but never folded to a constant, since the image carries no
copy of it.

Language selection uses the same machine table as PE: `ARM`, `ARMTHUMB_MIXED`,
and `ARMNT` all select the Thumb-2-capable `ARM:LE:32:v8` language, with the
UEFI calling conventions as the compiler model (C/cdecl for IA-32, the UEFI x64
convention for x64, AAPCS for AArch32, AAPCS64/LP64 for AArch64); an explicit
compatible `--target` can request another model. The ARM decode-mode policy is
one table shared with the PE loader: `ARMNT` declares a wholly Thumb stream on
every container, `ARM` may interwork on every container, and machine `0x1c2` is
read the way its container family names it — `THUMB` on a PE (painted wholly
Thumb) and `ARMTHUMB_MIXED` on a TE (interworking). Where the policy leaves the
mode to the entry bit, on a TE or a PE alike, an odd entry proves Thumb at the
entry alone, and the `entrythumbflow` option (default on) carries that mode
along the flow reachable from the entry, after any byte overlays:
fall-through, direct branches, and direct `BL` calls, while an interworking
`BLX` target keeps the mode its own encoding selects. Exactly the instruction
ranges the walk decoded are painted Thumb, so A32 code elsewhere keeps its mode.
The walk covers only the bytes the file backs, does not fall through past a
direct call to a callee the loader already knows never returns, and decodes
with the language's own context writes suppressed, so an interworking `blx`
cannot flatten the Thumb code above its target.
The walk is bounded at 4096 instructions; reaching the bound keeps the ranges
walked so far, reports that once on stderr, and leaves the unreached code at the
language default. `--isa thumb` paints the whole image when it is wholly Thumb;
`--option entrythumbflow off` disables the walk. Unsupported machines and target
width/data-endianness conflicts are explicit errors.

A TE image has no `object`-crate view, so the discovery passes that re-parse the
input (`listing`, `aif`, `funcstart_patterns`, and the other Listing consumers)
do not run: `functions` reports the container entry plus any declared
boundaries, and every TE load says so once on stderr rather than leaving an
inventory of one to be inferred. For the same reason `strings`, `xrefs`, `decompile-graph`, and
the graph-backed `--summary` and `--reachable-from` filters reject a TE with a
capability error naming the supported commands. `--slice` names a Mach-O fat
slice and is ignored, as on any thin image. Project exports retain the TE entry
(reported through the inventory, at its even address) and the named section
table.

### Headerless raw images

`--raw-image` loads a file that has no object header. It is separate from
`decompile --raw`, which still means “also print raw p-code.” A raw image must
name its decoder with `--target <SLEIGH-language-id>`, map file offset zero with
`--base <address>`, and supply at least one numeric entry. `decompile` uses its
positional address; the other supported commands accept repeatable `--entry` or
`--addr` values.

```bash
kuna decompile payload.bin 0x4001 --raw-image \
  --target ARM:LE:32:v4t:default --base 0x4000 --isa thumb

kuna functions payload.bin --json --raw-image \
  --target ARM:LE:32:v4t:default --base 0x4000 \
  --entry 0x4001 --isa thumb
```

The whole nonempty file is one contiguous executable `CODE` mapping. Base and
entry values use the selected language's address units, so address `1` in a
two-byte word-addressed code space selects file byte offset `2`. The mapping is
checked for arithmetic and address-space overflow, and an entry outside its
half-open range is rejected. Duplicate entries collapse. ARM32 raw input requires
`--isa arm|thumb`; odd ARM function pointers are normalized to their underlying
even byte address before validation. Other addresses retain every input bit, so
an odd ARM data or property address still selects the odd byte.

Raw images carry no symbols or trustworthy boundary metadata, so `functions`
reports the explicit seeds. Named `--functions`, section-relative selectors,
`--slice`, `--summary`, and `--reachable-from` are rejected. Support is limited
to `decompile` (text and JSON), `decompile-all`, `functions`, and
`decompile-project`; `decompile-graph`, `disassemble`/`read`, `xrefs`, and
`strings` require object metadata. A headerless file used without `--raw-image`
reports the required raw-image command shape, including when its first byte is
`<`; only a parsed document containing `<binaryimage>` is treated as XML. Quoted
console filenames preserve paths containing whitespace; the interactive spelling
is `load raw <target> <base> <entry[,entry...]> <filename>`.

**Failure contract (DIV-45).** A function whose decompile pipeline aborts is
*loud*:

- **exit code `1`** — the same code as a run-level error (no such function, no
  architecture, no C at all). Exit `0` means the pipeline completed.
- **stderr** carries `error: decompilation failed for <fn> in <binary>:
  <reason>`, followed by `note: decomp_dbg stderr:` and the console's own
  stderr (the panic line and its source location), truncated at 2000 chars.
- **stdout still carries the recovered shell**, whose body comment names the
  same reason: `/* WARNING: decompilation failed: <reason> */`. A shell with
  the generic `/* WARNING: structured blocks unavailable (structuring
  declined) */` means the pipeline *ran* and produced no structured blocks —
  a different failure.

**Load and analysis failures (DIV-90).** `kuna decompile` runs the engine in a
subprocess, so it recovers *why* a run failed from the console transcript — and
reports it in the same words `decompile-all` / `functions` / `decompile-project`
use, so one failure reads identically from all four commands:

- **the binary could not be loaded** — `error: could not build an architecture
  for <binary>: <reason>` (e.g. `Non-global scope has empty name`, `No sleigh
  specification for x86:LE:64:default`, `not in recognized object file format`),
  exit `1`. The older `could not build an architecture for <binary>
  (unsupported/!recognized binary)` is now only the fallback for a transcript
  that carried no reason at all.
- **the analysis commit failed** — `error: read symbols (analysis commit)
  failed: <reason>`, exit `1`, **and no C**. The console keeps its session alive
  after a failed `read symbols`, so C *can* still be rendered, but from a program
  whose debug facts were applied only up to the failing step and cannot be
  re-committed; that C used to be printed with exit `0`, indistinguishable from a
  binary with no symbols at all. `--option datasyms off` (or naming whichever
  analysis pass is implicated) is the way to get a run through.

The abort itself is not fatal to the console session (`decomp_dbg` prints
`Skipping <fn>: <reason>` and keeps going, so datatest `<stringmatch>` rules
still evaluate); the CLI is what turns it into a non-zero exit.
`decompile-all` / `decompile-project` / the WASM front-end are unaffected: a
failed function stays a per-function `error` record and never aborts the batch
(its text now carries the real panic message instead of `panic with non-string
payload`).

## `kuna decompile-all` / `kuna functions` — whole binary, machine-readable

```bash
kuna functions ./a.out --summary --json                # where do I start?  (~1 KB)
kuna decompile-all ./a.out --json                      # every CODE-backed function
kuna decompile-all ./a.out --functions main,parse --json
kuna decompile-all ./module.o --addr .text+0x660 --json
kuna functions ./a.out --json                          # full callable-symbol inventory
kuna functions ./a.out --sort size --limit 10          # the ten biggest functions
kuna decompile-all ./a.out --reachable-from main --json    # only what main touches
kuna decompile-all ./a.out --json --jobs 12            # the same answer, 12 processes
kuna decompile-all ./a.out --functions main,parse --json
kuna decompile-all ./a.out --json                      # every CODE-backed function
```

The whole-binary surface (the benchmark + LLM path). Runs **in-process**
(`kuna_console::engine::bootstrap_from_object` → `commit_pending_analysis` → loop
`decompile_func` + `print_c`), loading + analyzing the binary **once** instead of
`kuna decompile`'s subprocess-per-function (≈10×+ faster on a many-function binary).

### Triage — narrowing the run

An unfiltered whole-binary answer is only usable if the caller can narrow it
*before* it is produced: a 211 KB PE crackme is 1,150 functions and **5.9 MB** of
`decompile-all --json`, which is more context than the question is worth. Both
surfaces therefore take the same selection flags, and they choose which entries
the run *has* — `decompile-all` decompiles only what survives them, so narrowing
is what makes the run cheap as well as small.

| Flag | Selects |
|---|---|
| `--filter REGEX` | functions whose name — or any alias — matches (unanchored [Rust `regex`](https://docs.rs/regex) syntax; `(?i)` for case-insensitive) |
| `--min-size N` / `--max-size N` | functions whose inventory `size` is within the inclusive bound |
| `--reachable-from <name\|0xaddr>` | the named function plus everything it reaches through the call graph |
| `--sort addr\|size\|name` | ordering — `addr` (default) ascending, `size` **largest first**, `name` ascending; every key breaks ties on the address, so a narrowed run is reproducible |
| `--limit N` | keep the first N after sorting |

Filters compose (they intersect), and a selection that matches nothing is an
answer, not a failure: it exits 0 with `count: 0`. The zero-discovery verdict
below stays attached to *discovery*, so it can never fire because a filter was
too narrow. `--filter` / `--min-size` / `--max-size` / `--limit` are pure
inventory arithmetic and cost nothing extra; `--reachable-from` additionally
walks the program once.

That verdict counts **bodies**, not inventory rows, on `functions` and
`decompile-all` alike. The inventory keeps an import pointer slot as an entry so
that a call to an import renders its name, and an image can consist of nothing
else: a NEOLite-packed PE that flags every section `INITIALIZED_DATA|READ|WRITE`
enumerates its six imported Win32 names and not one body. The six names are still
listed — they are what the packed stub is going to call, which is exactly what a
caller wants from that file — and the run still reports the failure:

```console
$ kuna functions ./packed.exe --json
{"binary":"./packed.exe","count":6,"total":6,
 "error":"no functions discovered in ./packed.exe: its entry point 0x4f7001 lies in
          section .NEOpack, which the image does not flag executable -- pass
          `--define-function 0x4f7001` to decompile there anyway",
 "functions":[{"name":"GetProcAddress", …}, …]}
$ echo $?
1
```

`--reachable-from` is the "what does the entry point actually touch" question,
answered with **`kuna xrefs`' own reference edges** (`kuna-analysis`'s
`listing::xrefs`) rather than a second call-graph model that could disagree with
them. A call, a tail jump, and an *address-taken function pointer* all count as
edges — the third one matters: on a glibc ELF `_start` reaches `main` only
through the pointer it hands `__libc_start_main`, and a callback registered with
`CreateThread` or `atexit` is likewise code the caller reaches. A `call qword ptr
[__imp_X]` reaches the import it names, so a Windows program's API calls are
edges like any other. A materialized
address that does not land on a known function entry is a string or a global, not
a callee, and is not an edge. The operand resolves as a name first and only then
as bare hex, so a function genuinely called `abc` is never read as `0xabc`. A
name that resolves to nothing exits 1.

On the 211 KB PE above, pointing `--reachable-from` at the one function that
references the challenge prompt (found with `kuna xrefs --to` on the string) cuts
the run from 1,036 decompiled functions to 307 — **5,943,701 bytes / 11.5 s down
to 876,577 bytes / 2.5 s**, with the answer still inside it. Adding `--min-size
256 --sort size --limit 10` brings it to 115,667 bytes / 1.8 s.

### `kuna functions --summary` — orientation in one call

```bash
kuna functions ./crakersme.exe --summary --json    # 2,820 bytes
```

The first call to make on an unknown binary: it answers *where do I start*
without emitting a function list at all, let alone pseudocode.

```json
{"binary":"…","count":1150,"total":1150,"error":null,
 "summary":{"entry":{"name","address","address_hex"},
            "reachable_from_entry":334,"no_callers":714,"code_bytes":171971,
            "size_buckets":[{"bucket":"0","min_size":0,"max_size":0,"count":114}, …],
            "largest":[{name,address,address_hex,aliases,size}, …]}}
```

- `entry` is the **image's declared entry point** (a PE `AddressOfEntryPoint` is
  the CRT startup, not `main`), named with kuna's best name for it, or `null`
  when the format declares none. Always a virtual address: a Mach-O `LC_MAIN`
  states its entry as a `__TEXT`-relative file offset, and it is rebased here
  (`0x1000005b0`, not `0x5b0`).
- `reachable_from_entry` counts *discovered* functions the entry point reaches,
  and is `null` when there is no entry point or nothing was decoded at it (a
  packed image); `no_callers` counts *selected* functions that no CALL site
  references — the roots and the dead code. Both come from the same xref edges
  `--reachable-from` walks.
- `size_buckets` partitions the whole extent domain (`0`, `1-15`, `16-63`,
  `64-255`, `256-1023`, `1024-4095`, `4096+`), so nothing falls between buckets;
  `max_size` is `null` on the open-ended one.
- `largest` holds the `--limit` biggest functions, 10 by default.
- The triage flags apply: `--summary --reachable-from main` summarizes just that
  subgraph. `count` is what was selected, `total` what discovery found.

Without `--json` the same measurements print as tab-separated lines. `--summary`
is accepted on `decompile-all` too, where it short-circuits the decompile loop
entirely — asking where to start must never cost a whole-binary decompile. Both
surfaces load through the `functions` (inventory) driver bundle for it, so the
numbers a caller orients by are the ones `kuna functions` reports.

### The JSON documents

`--json` emits
`{binary,count,functions:[{name,address,address_hex,aliases,object_location,size,code,error,
line_mappings:[{line_number,addresses}],variables:[{name,type,kind,arg_index,
stack_offset,size,line_numbers,addresses}]}]}` (`kuna functions --json` emits
`name`/`address`/`address_hex`/`aliases`/`object_location`/`size` per function).
`object_location` is `null` for linked images and undefined imports; for a relocatable
definition it is `{section_index,section,offset,offset_hex}`. `count` is what the
`functions` array holds. `kuna functions --json` also carries `total`, the count
before any triage narrowing; `decompile-all --json` carries `total` only when a
triage flag actually narrowed it, so an unfiltered whole-binary document — the one
the decbench backend and `kuna decompile --json` read — is byte-identical to
before. `line_mappings` maps 1-based
lines in `code` to sorted, unique machine-instruction VMAs. Variable `line_numbers`
come from the printer's `varref` tokens; variable `addresses` are the union of the
mapped instruction addresses on those lines. Both are empty when no backed use is
emitted. The references are captured from Kuna's markup emitter and resolved against
the live p-code IR, rather than inferred from the rendered text. The ordinary
plain-text renderer still produces `code`, so its bytes are unchanged.
Reported variables are joined to native varrefs by ABI or stack storage and recovered
high-variable identity. Multiple high-variable fragments are combined only when they
name the same exact stack location and size; ambiguous name-only matches stay empty.

Per-function `size` is the entry's byte extent, and both surfaces report the same
number with the same meaning — it is an **inventory** fact, measured without
decompiling, so `kuna functions --json` alone is enough to rank a binary's functions
by weight (the "decompile the three biggest functions" first move costs one call, not
a whole-binary run). It is an **upper bound**: the address-contiguous clip from the
entry to the next entry, or to the end of the containing CODE section, whichever comes
first — so inter-function alignment padding is counted in. Against ELF `st_size` over
the 1428 symbolized-fixture functions with ground truth it is never short, exact for
231, and overshoots by a median of 8 bytes (worst 52). An entry in no CODE section — an
import pointer slot, an undefined external — reports `0`, as does a function whose
extent could not be measured. An image with no usable section table at all (a
sectionless ELF; a corrupt one the loader recovers from the program headers) is clipped
against its executable load segments instead, so the last entry of a segment runs to
the end of that segment rather than to the end of a `.text` — looser, but still an
upper bound, where every entry used to report `0` and `--min-size 1` discarded the
whole binary. A caller needing the exact body must still decompile.

Per-function `code` matches `kuna decompile ... --option listing on` byte-for-byte on
x86-64 (elsewhere, see the injected defaults below), `error` isolates a single failed
function, and `variables` (params in ABI order + DWARF/stack locals) feed type-recovery
scoring. `--no-vars` leaves `variables` empty but still emits function line mappings.

Behaviors specific to `decompile-all`:

- **Executable default targets** — an unfiltered run decompiles canonical entries
  contained by loader sections marked `CODE`, minus the **import pointer slots**
  the loader resolved names at. Callable slots in PE IATs, Mach-O symbol-pointer
  sections, and similar data areas remain in `kuna functions`, remain installed
  for named calls and prototypes, and remain reachable through explicit `--addr`;
  they are not automatically decoded as function bodies. The slot test is what
  covers a packed PE that keeps its whole import directory inside its one
  `CODE|EXECUTE` section, where the section flags cannot tell the two apart.
  Analysis-discovered entries inside executable sections join
  this default set, and so does a PE entry point declared inside the image's own
  header page — the `SizeOfHeaders` bytes before the first section, where a
  packer is free to lay its stub and where no section flag speaks for the bytes
  at all. Nothing else in a header page is decompiled; an entry in a section the
  image flags non-executable still needs the `--define-function` above. A name that identifies entries at several addresses is rejected as
  ambiguous instead of selecting the first. Loaders without section metadata retain the
  complete inventory. A `--define-function` declaration outranks both tests, so an
  address the import directory claims can still be decompiled on request.

- **Relocatable-object selectors** — an `ET_REL`/`.obj` is loaded into a synthetic VMA
  space, but its original coordinates remain available. `--addr` accepts a synthetic
  `0xVMA`, `.section+0xOFFSET`, or `SECTION_INDEX:0xOFFSET`. A bare numeric address keeps
  backward compatibility: a mapped synthetic VMA wins; otherwise it resolves a defined
  function at that raw section offset only when unique. Ambiguities list every candidate
  with its section, raw offset, synthetic VMA, and symbol binding. Arbitrary unmapped
  addresses are errors. Only symbols marked undefined/import by the object are reported as
  external.

- **Relocation diagnostics** — supported relocations are applied before decoding. Entries
  that cannot be applied are grouped by architecture, relocation type, and failure reason,
  with exact totals, at most eight groups, and at most three samples per group. A public load
  emits that report at most once; successful loads are silent. Diagnostics remain on stderr,
  so JSON on stdout stays valid, and the fixed group/sample limits keep stderr bounded even
  for objects containing thousands of identical failures.

- **One record per function entry** — a whole-binary run reports (and decompiles) each
  entry address exactly once. A function can carry several names: a `.symtab` symbol
  plus a debug-info one (`macho_dwarf.o` has `_l0` and `first_byte` at `0x0`), a
  decorated/undecorated PE pair, or the generated `sub_<addr>` placeholder an analysis
  pass registers over an already-named entry. `name` reports the most informative of
  them — a real symbol beats a synthesized `_INIT_<i>`/`_FINI_<i>`/`_DT_INIT`/`_DT_FINI`
  table name, which beats a generated `sub_`/`func_`/`FUN_`/`LAB_` placeholder; ties
  prefer the unprefixed spelling (`main` over `_main`), then the shorter name — and
  `aliases` carries the rest (`[]` when there is only one). `--functions <name>` matches
  aliases too, so any name that used to select a function still does. On ARM the Thumb
  mode bit is folded out of symbol addresses, so a function whose ELF `st_value` is odd
  (`compute` at `0x100b9`) is reported once, at its real even entry — and `--addr` accepts
  either spelling, resolving an odd ARM address to the entry it belongs to instead of
  decompiling mid-instruction. The fold is ARM-only: an odd address on a byte-aligned ISA
  is a genuine entry and is left alone.

- **Injected default options**: under the concrete `reliable` preset it injects
  `option listing on` unless the caller names
  `listing` (DIV-15), so the default-on `noreturn_propagate` call-graph fixpoint fires and
  a stripped binary's unnamed exit/fatal wrappers no longer swallow the functions after
  them; on non-x86-64 binaries it likewise injects `funcstart_patterns on` and `aif on`
  unless the caller names them (see `docs/history.md`). `--option listing off` opts
  out. Single-function `kuna decompile` injects the Listing the same way, and
  reaches for the **discovery half on a second attempt**: a by-name selection that
  the console answers with `no function matches` is retried once with
  `funcstart_patterns on` + `aif on` on a non-x86-64 image, so a name that exists
  only because discovery generated it -- the `sub_<addr>` `kuna functions` and
  `kuna strings` print -- selects the same entry those surfaces report. Nothing
  that already resolved changes: the first attempt is the script it has always
  been, and the retry is skipped for `--addr`, for an ambiguous selector, and for
  a load or pipeline failure. The bundle is not injected up front because it
  changes the entry set and not every entry it adds is real -- on i386 and PPC64
  the prologue matcher seeds a start a few bytes inside a function it already
  knew (PPC64 ELFv2's local entry point), and `funcboundflow` then truncates the
  outer function at that seed. That trade is the whole-binary surfaces' to make,
  where the wider inventory is the point; a single-function request that already
  named its function gains nothing from it. (The gap was invisible under the
  default `auto` policy below 500 KiB, which resolves to `aggressive` and names
  all three options itself.)
  `kuna functions` shares the **discovery** half of that policy (DIV-68): on a
  non-x86-64 binary it injects `funcstart_patterns on`, `aif on`, and the
  `listing on` those two are gated behind, so the inventory always contains every
  entry `decompile-all` would decompile (stripped betaflight STM32F405 under
  `--mode reliable`: 1 entry listed before, 5,798 after, against the 5,797
  `decompile-all` decompiles). That costs a whole-program decode there — 0.08 s to
  5.27 s on that firmware — which is the price of a correct answer. On x86-64
  `kuna functions` injects nothing and is unchanged: the Listing is measured
  entry-neutral on that architecture, so it stays the decompiling surfaces'
  default. The interactive console keeps the engine default off; an auto-selected
  `aggressive` preset names all three itself, on either surface.
  Omitted `--mode` first resolves the size-based `auto` policy. `--mode fast`
  names and disables the three exhaustive program-wide decode/discovery options
  (`listing`, `funcstart_patterns`, `aif`), suppressing those injections, and
  enables `fast_funcdisc`. That bounded pass recursively promotes direct CALL
  targets from loader-backed roots and adds conservatively validated
  pointer-table targets, so a stripped project does not collapse to imports plus
  its entry point. An explicit `--addr` selector suppresses the preset-provided
  pass because the entry is already known; `--functions` keeps discovery active
  so generated names can resolve. Explicitly spelling `--option fast_funcdisc
  on` opts an address run back in. A later explicit `--option` always wins.
- **Per-function watchdog** — `--max-fn-seconds N` (`0` disables): an
  unfiltered `decompile-all`/`decompile-project` run in the resolved `fast`
  preset defaults to 10 seconds per function. On native, selected-function runs
  and the other presets retain 120 seconds; an explicit value always wins. WASM
  arms only the fast whole-binary 10-second policy and leaves its other commands
  unbudgeted. A function whose decompile drive exceeds the budget is cut off
  cooperatively (deadline probes at the action/rule-pool/heritage loop
  boundaries) and recorded as that function's `error` (`"per-function
  decompile budget exceeded (N s)"`), the batch continuing. This is not a hard
  process timer: it does not bound discovery, unprobed decoder work, C/variable
  rendering, artifact construction, total export time, or memory. Driver
  policy, not a stage-model settable — zero output change for a function whose
  drive completes before expiry; the console / `decomp_dbg` parity path never
  arms it.

- **`--jobs N|auto` — the worker pool.** The per-function loop is ~96% of a
  whole-binary run's wall clock and the engine is single-threaded, so `--jobs N`
  spreads it over `N` copies of the `kuna` binary. `--jobs 1` is the default and
  is today's in-process path, byte for byte. Also on `decompile-project` and
  `decompile-graph`; `--jobs-chunk N` fixes the functions per scheduling unit and
  `--jobs-full-load` makes each worker run its own whole-binary discovery instead
  of taking the parent's inventory.
  - **The output of a non-stream run does not depend on how the pool scheduled
    it.** Work is handed out in a longest-first order that is deliberately not
    output order, but every target owns a slot and results are merged
    positionally, so the document is identical to `--jobs 1` whatever order the
    workers finish in. The concrete `--mode`, every resolved `--option` and the
    watchdog budget are settled once by the parent and passed to every worker.
    `decompile-project --stream` is the one surface where the schedule *is*
    observable, and only in the order of the `.c`: a streamed export writes each
    function as it finishes, so under `--jobs N` the interleaving is worker
    completion order and is not reproducible run to run. The set of functions,
    the prototypes and the disassembly are what the same selection produces
    serially; `index.jsonl` indexes the order the run happened to take.
  - **It can depend on how the work was divided, wherever the engine's own output
    already does.** A few emission decisions are first-toucher-wins in the
    per-process type and symbol database, so they are a function of which *other*
    functions the same process decompiled, and sharding changes that set. On the
    18 MB PE above this is 2 of 32,777 functions, both a two-byte string constant
    rendering as `"BM"` where the serial run printed the UTF-16 `"䵂"`. The pool
    does not cause it: a serial `--filter '^sub_18073e690$'` over the same load
    prints `"BM"` too, and adding the function that reaches that address first
    turns it back into `"䵂"`. `--jobs 1` is the definition of the answer.
  - **Memory, not cores, is the limit.** Every worker loads the binary itself, so
    peak memory is roughly `N ×` one worker's resident size, on top of the
    parent's. On an 18 MB PE with 33,214 functions that is 469 MB per worker
    against the parent's 1.99 GB, because a worker skips the discovery the parent
    hands over; `--jobs-full-load` puts each worker back on the parent's own load
    and its memory. `--jobs auto` takes this machine's parallelism capped at 16,
    then lowers that to what free memory holds and says so; an explicit
    `--jobs N` is obeyed with a warning instead.
  - **Small runs are slower.** A worker pays a whole program load before its first
    function (17 s on that PE), so a run only wins once the work exceeds one such
    load — a few hundred functions there, and nothing at all on a handful of
    them.
  - **The first progress lines omit ETA.** The pool waits until every worker has
    completed its first chunk, then estimates from work completed after that
    point. Worker startup is therefore shown in elapsed time but is not projected
    across the remaining functions.
  - **The watchdog becomes a real one.** `--max-fn-seconds` is cooperative in
    process, so a function wedged where nothing polls the deadline runs through
    it; the parent, which is not the stuck process, kills a worker that has
    produced no record for well past the budget and records its unfinished
    functions as `error`. The same budget is a wall clock, so a function that
    finished just inside it serially can miss it under N-way contention: the run
    reports how many did and points at `--max-fn-seconds`.
  - **Cancelling the run stops the pool.** Each worker's stdin is a pipe whose
    only write end its parent holds; end-of-pipe means the parent is gone by any
    route including SIGKILL, and the worker deletes the shared scratch directory
    (0700, holding the symbol inventory and every function's C) and exits.
  - `--assert` and `--raw-image` are refused with `--jobs > 1`: assertion outcomes
    are per-load state a pool cannot merge, and a raw image's entry seeds are its
    load.

The decbench backend (`decbench/decompilers/raw/kuna_raw.py`) shells out to
`kuna decompile-all --json`.

## `kuna xrefs` — cross-references

```bash
kuna xrefs ./a.out --to authenticate            # who references this?
kuna xrefs ./a.out --to 0x1030 --json           # by address, machine-readable
kuna xrefs ./a.out --from main                  # what does this reference?
kuna xrefs ./a.out --from main --kind call      # call sites only
```

The navigation query: `--to` returns everything that references the target — call
sites, branches, and data references — and `--from` returns what the target
references: its callees, the functions it tail-jumps to, and the data it touches.
The two directions and the per-row `kind` mirror the DecLib CLI's
`xref_to`/`xref_from`, so an agent that knows one knows this.

The target is a **symbol name or an address** (`0x`-prefixed, or bare hex). A name
is always resolved as a symbol first, so a function really called `abc` is not
silently read as `0xabc`. Function names, the `s_<addr>` string symbols the
`strings` pass installs, and named data globals all resolve — which is what makes
the string-to-its-users hop work: `kuna xrefs ./a.out --to s_400915`. A function
name that identifies several *distinct* entries — two same-named locals in a
relocatable object — is reported as ambiguous with every candidate, never answered
for whichever one the symbol table holds first. A name that identifies several
addresses of the **same** callable is not ambiguous in any sense the caller can
act on and is answered; see below.

| `kind` | What it is |
|---|---|
| `call` | A CALL to the target (a call site): a direct `CALL 0x1030`, or a `CALL qword ptr [slot]` that reads its destination out of a fixed slot — the shape every imported Windows API call has. The slot is where the edge lands, because the slot is what carries the import's name. |
| `jump` | A direct branch to it: a tail call, a PLT thunk. Intra-function branches are control flow, not references, and are omitted from `--from`. |
| `data` | The target's address is materialized as a value — address-taken: a function pointer, a string pointer, a global's address. Also the value of a **literal pool** word an instruction loads (`ldr r0,[0x86e4]` where 0x86e4 holds the string's address), which is how an ARM literal gets an owning function at all; the pool word itself is a separate `read` row from the same instruction. Only pointer-sized reads of *non-writable* memory are followed. |
| `read` | The target is loaded from. |
| `write` | The target is stored to. |

### One import, two addresses

An imported function has two addresses and the import's name is on both: the
**IAT/GOT slot** the loader fills in, and the **forwarding veneer**
(`jmp qword ptr [slot]`) a direct `call` can target. `kuna functions --filter
VirtualProtect` on a MinGW PE therefore answers with two entries — a veneer at
`0x1400079b0` and a slot at `0x14000d234` — and which of the two a given call
site references is a compiler decision, not something the question was about.

`--to` is answered over both: the veneer, the slot it jumps through, and any
other veneer through that same slot are one **alias class**, and the answer is
the same whichever member is asked for. The class comes from the decoded
forwarding jump, never from a shared name, so two unrelated functions that happen
to be called `init` are never folded together. The veneer's own `jmp [slot]` is
excluded from the answer — it is the other half of the callable, not a caller of
it. `target.aliases` lists the other members (empty for everything that is not an
import, which is nearly everything), and every row still carries the real
`to_address` it landed on, so an agent can see whether a call site went through
the veneer or straight through the slot.

Because the name is on both addresses, `--to VirtualProtect` is a selector that
matches two entries. It is still answered: the candidates are settled against the
alias class the walk found, and candidates that are all one class are one callable,
so the query proceeds at the class's code half — the veneer, which is the address
an agent goes on to `kuna decompile`. Candidates that are *not* one class are
genuinely different functions and keep the ambiguity error with every candidate
listed. So the fold never rests on the name: it rests on the decoded forwarding
jump, and the name only has to point the walk at the addresses to check.

```
# 2 references to VirtualProtect @ 0x1400079b0
# same import at 0x14000d234 (VirtualProtect) - a forwarding veneer and the pointer slot it jumps through
0x140001a9e	call	__write_memory.part.0+0x18e	CALL qword ptr [0x14000d234]
0x140001cce	read	_pei386_runtime_relocator+0x19e	MOV R12,qword ptr [0x14000d234]
```

Flags: `--json`, `--kind call,jump,data,read,write` (repeatable-by-comma filter),
plus the shared `--mode`, `--option N V`, `--slice`, `--target`, `--sleighpath`.

`--json` emits

```json
{"binary": "...", "direction": "to", "count": N,
 "target": {"name","address","address_hex",
            "aliases": [{"name","address","address_hex"}]},
 "xrefs": [{"address","address_hex","kind",
            "from_address","from_address_hex","to_address","to_address_hex",
            "from_function": {"name","address","address_hex"},
            "to_function":   {"name","address","address_hex"},
            "instruction": "CALL 0x1030"}]}
```

Both ends of every edge are always spelled out, so a consumer never has to infer
which one `address` meant; `address` itself is the end the query did not already
name (the referencing site for `--to`, the referenced location for `--from`).
`from_function` / `to_function` are `null` when nothing owns that address — a
`.rodata` string has no containing function. Without `--json` the output is a `#`
header line naming the query followed by one tab-separated row per reference.

```
# 1 reference to __cxa_finalize @ 0x1030
0x1102	call	_FINI_0+0x22	CALL 0x1030
```

This is a query, not an engine change: it loads the binary once through the same
in-process seam `decompile-all` uses (`bootstrap_from_object` →
`commit_pending_analysis`), then reads the references out of the p-code the SLEIGH
lifter already emits for every discovered function
(`kuna-analysis/src/listing/xrefs.rs`). It commits nothing into the engine and
changes no emitted C. Function discovery is the `kuna functions` inventory, which
the walk then extends by following the call graph out of it, so a callee the
inventory missed is still covered.

`--mode` is **not** resolved through `auto` here, unlike the decompiling surfaces:
`auto` selects `aggressive` under 500 KiB, and `aggressive` is a preset for the
quality of emitted *C*. Two of the passes it turns on cost a whole extra decode of
the program apiece and answer nothing a reference query reads — the analysis-tier
Listing walk (whose recursive descent `xrefs` repeats itself over the same bytes)
and `operand_refs` (whose scalar markup `xrefs` recomputes from the p-code it
already has). So the query surface defaults to the shipped defaults, and
`kuna xrefs --mode aggressive` still asks for the full analysis bundle explicitly.
On a 466 KB obfuscated i386 image the two skipped decodes were 1.08 s and 0.58 s of
a 3.4 s answer that is byte-identical without them.

Dropping the Listing does **not** drop the discovery it fed. The query surface takes
the same DIV-20/DIV-68 discovery flags every other surface does (`funcstart_patterns`,
`aif`); it just consumes them itself, from its own decode:

* the `<patternpairs>` prologue starts go straight into the walk's seed set;
* the speculative gap-walk (`aif`) runs over the partition the walk leaves behind,
  and the functions it accepts are walked like any other, so their references join
  the answer. Without it, a function reached only through a function-pointer table
  is in no seed set and `--to` loses every call site inside it — measured on a
  stripped i386 PE as 61 of one function's 174 callers.

The address you ask about is itself a seed. A recursive descent answers for the code
it can reach, and an entry with no inbound CALL edge is not reachable from any seed
set, so `kuna xrefs --from <that entry>` used to answer `count: 0` about a function
that plainly has references. It is now walked last, after the seeded descent has
drained, so it can only add coverage — an address the walk already decoded is
attributed exactly as before, and an address that does not decode is not recorded as
a function at all.

The descent also follows a **jump table**. A computed jump has no static successor,
so the walk used to stop dead at every switch dispatch and the case bodies — with
every reference they form — were invisible: on a stripped i386 PE, `kuna strings
--filter "Product Already Registered"` answered `xrefs_count: 0` and no owning
function for a message the window procedure plainly pushes. The table base is the
address the dispatch itself materializes, and its entries are read out of read-only
image content while each lands in the same executable section as the dispatch; the
first word that is not one ends the table. The case bodies are walked as part of the
**dispatching function**, so the message above now answers with the handler rather
than with an address inside it. An import veneer's `jmp [slot]` is not a table (its
slot is a data operand, not a materialized constant) and is unaffected.

Real x86-64 compilers write neither half of that shape: the base lands in a register
one to a few instructions ahead of the jump and the entries are signed 32-bit
displacements from it, gcc measuring them from the table and MSVC from the image
base. The walk therefore also asks what the branch register holds, stepping back over
the already-decoded instructions that feed it, and reads a displacement table where
they compose one. How far the table runs comes from the switch's own range check —
`cmp $0xa,%r11d; ja default` is eleven cases — because two such tables laid back to
back are indistinguishable from one long one. Without a range check to read, nothing
is followed. This is what puts the callees inside a `switch` on
`kuna decompile-graph`'s callee list.

A target nothing references is exit `0` with `count: 0` — an answer, not a
failure. A name that resolves to nothing is exit `1` with the reason on stderr; a
malformed command line is exit `2` with the usage block.

## `kuna disassemble` / `kuna read` — instructions or bytes, when the pseudocode is not enough

```bash
kuna disassemble ./a.out main                    # a function, whole extent
kuna disassemble ./a.out main --json             # machine-readable
kuna disassemble ./stripped.bin 0x8049850 --addr # a raw address
kuna disassemble ./a.out 0x1140-0x11a0           # an explicit range
kuna disassemble ./a.out 0x2010 --addr --bytes 64  # bytes no function owns
kuna read ./a.out 0x100003f30 --addr --bytes 96  # a hexdump of a data address
kuna disassemble ./packed.bin 0x2010 --addr --as code   # decode data as code anyway
```

The floor to fall back to when the ceiling gives way. Every RE agent that asked
for this had already tried decompiling: a function with no recovered body, a
dispatcher emitted as `switch(0)`, an indirect call through a stack buffer the
program decrypts at runtime. When the pseudocode cannot answer, the instructions
still can — and until now the only way to see one was to leave kuna for
`objdump`.

The target is a **name**, an **address**, or a **range**:

| Target | What is listed |
|---|---|
| `main` | The function's extent — the same clip `kuna functions` reports as `size`. A name is resolved as a symbol first, so a function really called `abc` is never read as `0xabc`. |
| `0x8049850` (`--addr` for bare hex) | That function's extent if the address is a discovered entry; otherwise 64 bytes from exactly there. |
| `0x1140-0x11a0`, `0x1140..0x11a0` | Exactly that half-open span — the direct replacement for `objdump -d --start-address=.. --stop-address=..`. |

`--count N` stops after N listed entries and `--bytes N` after N bytes; either
overrides the derived extent, and a listing stops at whichever limit it reaches
first. Also accepted: `--as`, `--follow`, `--json`, plus the shared `--mode`,
`--option N V`, `--slice`, `--target`, `--sleighpath`.

```
$ kuna disassemble ./fauxware main --count 9
# 9 instructions at main @ 0x40071d (0x40071d..0x40073e, 33 bytes)
0x40071d      55                    PUSH RBP
0x40071e      4889e5                MOV RBP,RSP
0x400721      4883ec40              SUB RSP,0x40
0x400725      897dcc                MOV dword ptr [RBP + -0x34],EDI
0x400728      488975c0              MOV qword ptr [RBP + -0x40],RSI
0x40072c      c645f800              MOV byte ptr [RBP + -0x8],0x0
0x400730      c645e800              MOV byte ptr [RBP + -0x18],0x0
0x400734      bf15094000            MOV EDI,0x400915
0x400739      e8d2fdffff            CALL 0x400510
```

Address, raw bytes, instruction. The instruction text carries exactly **one**
space between mnemonic and operands — the same spelling `kuna xrefs` puts in its
`instruction` field, so one `grep 'CALL 0x400510'` matches both surfaces and the
JSON. `--json` emits

```json
{"binary": "...", "kind": "code", "target": {"name","address","address_hex"},
 "start": N, "start_hex": "0x..", "end": N, "end_hex": "0x..",
 "count": N, "bytes": N, "truncated": false, "notes": [],
 "instructions": [{"address","address_hex","size","bytes","mnemonic","operands","text"}]}
```

`bytes` on a row is that instruction's own bytes as contiguous lowercase hex
(`"4889e5"`); `end` is one past the last instruction actually listed, so a
truncated listing hands back the address to resume from. `kind` is `"code"` here
and `"data"` in the byte view below.

Bytes the translator will not decode are listed in place as `.byte` rows and the
walk continues — a listing that ran into inline data says so where it happened
instead of stopping silently. How far one such row reaches depends on the
architecture. Where any address can start an instruction the row is one byte, and
the walk tries the next. Where instructions must be aligned, code can only resume
on that grid, so the row runs to the next boundary and the listing stays on it:

```
0x10020       b8feffff              .byte 0xb8,0xfe,0xff,0xff
```

rather than a `.byte 0xb8` at `0x10020` followed by rows at `0x10021`, `0x10025`
and `0x10029` — addresses no ARM instruction can begin at. The grid is the
alignment the rows already decoded all share, so an ARM listing of 4-byte rows
resumes on 4 and a Thumb listing that has decoded a 2-byte row resumes on 2.
Staying on the grid is also what lets the literal pool below be recognized at
all: a pool word is only folded when it starts a decoded row.

### Literal pools are listed as data

A constant that will not fit an ARM/MIPS/PowerPC immediate is parked in `.text`
beside the code that uses it and loaded PC-relatively, so a function's own extent
contains words that are not instructions. Decoded, they are a lie: `main` in the
`1337ARM` crackme ended

```
0x8454        10a89de8              ldmia sp,{r4,r11,sp,pc}
0x8458        39050000              andeq r0,r0,r9, lsr r5
```

where `0x8458` is the success constant `0x539` the `ldr` at `0x8440` loads, and
`andeq` is four bytes nothing executes. Such a word now lists as the constant it
holds, with the reason on **stderr** (and in `notes`):

```
0x8458        39050000              .word 0x00000539
```

The evidence is inside the listing and nowhere else: some instruction **in the
listed range** spells the address out and reads it, and none branches to it. A
word is left decoded if it is in a writable section, if a function symbol sits on
it, if it is unaligned or not 1/2/4/8 bytes wide, or if its width does not tile a
whole number of decoded rows — so no address in the listing ever moves, folded or
not. The width is the width of the **access**, not of the address: `ldrh
r0,[0x1003c]` reads two bytes out of a four-byte slot, and folds nothing. Listing the word on its own contains no such load, which is the escape hatch
when the raw decode is what you want:

```
$ kuna disassemble ./1337ARM.bin 0x8458-0x845c
0x8458        39050000              andeq r0,r0,r9, lsr r5
```

The `mnemonic` is `.byte`/`.short`/`.word`/`.quad` by width and `operands` is the
value zero-padded to it; the row's `bytes` are the image's, as for any other row.

### Branch targets a straight line walks over — and `--follow`

Decoding forward from one end assumes every byte starts an instruction or is
inside one. Hand-written and obfuscated code breaks that on purpose: a two-byte
`EB 01` jumps over one junk byte, and from the junk byte on the listing is a
reading of instructions the program never executes. The rows that come back are
not merely missing one — they are wrong, and nothing in them says so:

```
$ kuna disassemble ./corrupt.exe 0x43d060 --addr --count 70
0x43d08c      eb04                  JMP 0x43d092
0x43d08f      eb01                  JMP 0x43d092
0x43d091      c2aceb                RET 0xebac          <- covers 0x43d092
...
0x43d0ad      e9c0c89504            JMP 0x4d99972       <- outside the image
0x43d0c0      e83492aae2            CALL -0x1d119d07    <- not in the bytes
```

Two things now happen. Every listing reports the addresses **its own branches
name** that no row of it starts at, on **stderr** and in `notes`:

```
note: the decode ran across 3 addresses this range's own branches name --
0x43d092, 0x43d0ae, 0x43d0c1 -- so no row starts at them; the instruction printed
over each one, and the rows after it until the decode re-synchronizes, spell bytes
the program never executes that way -- re-run with --follow to decode from those
addresses too
```

Only targets inside the listed span are reported: a branch out of the range says
nothing about the range. On ordinary compiler output there is nothing to report —
across ~800 vendored fixture functions the note fires on eight, every one of them
a real desynchronization.

`--follow` then decodes from those addresses as well as from the start, to a
fixpoint over whatever the new rows themselves branch to. One re-anchor is not
enough: at `0x43d092` the very next instruction is another `EB 01`, and only
following that one too reaches the real `0x43d096`.

```
$ kuna disassemble ./corrupt.exe 0x43d060 --addr --count 70 --follow
0x43d08f      eb01                  JMP 0x43d092
0x43d091      c2                    .byte 0xc2
0x43d092      ac                    LODSB ESI
0x43d093      eb01                  JMP 0x43d096
...
0x43d0c3      aa                    STOSB ES:EDI
0x43d0c4      e2cc                  LOOP 0x43d092
```

— a LODSB/STOSB decryption loop with its back edge, where the straight line had a
call and a jump that are not in the file.

`--follow` never lists **less** than the plain listing: bytes no flow reaches are
still listed, decoded straight-line in between and spelled `.byte` where they do
not tile an instruction, and the two listings cover the same span. Where two
reachable paths read the same bytes as different instructions — a conditional
branch into the middle of the instruction it skips — both rows are listed, so
rows can overlap under `--follow` and only under it. It is off by default because
it costs a second walk and a straight line is the right answer for compiler
output.

### The byte view

An instruction listing is the wrong answer for a data address, and for a while it
was the only one on offer. An agent that asked kuna for the encoded globals at
`0x100003f30` got `ADD byte ptr [RCX],AL` / `OR CL,byte ptr [RBX]` — a correct
decode of `00 01 02 03 ..` and a lie about the program — and left for `xxd`.

So the target picks its own rendering, and `--as` overrides it:

| `--as` | What is listed |
|---|---|
| `auto` (default for `disassemble`) | Instructions, unless the start address is in a section the loader marks as data and not as code (`.rdata`, `.rodata`, `__TEXT,__const`) — then bytes, with the reason on **stderr**. A discovered function entry is always code, wherever it was linked. |
| `code` | Instructions, whatever the section says. A packer puts real code in `.data`. |
| `data` (default for `kuna read`) | Bytes, whatever the section says. |

`kuna read` is the same command with `--as data` as its default — the spelling to
reach for when what you want is the bytes, not a view of them as instructions.

```
$ kuna read ./crackme 0x100003f30 --addr --bytes 96
# 96 bytes at 0x100003f30 (0x100003f30..0x100003f90)
0x100003f30   00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f  |................|
0x100003f40   10 10 10 10 10 10 10 10 10 10 10 10 10 10 10 10  |................|
0x100003f50   20 20 20 20 20 20 20 20 20 20 20 20 20 20 20 20  |                |
0x100003f60   25 73 00 43 72 61 63 6b 6d 65 20 4c 65 76 65 6c  |%s.Crackme Level|
```

Sixteen bytes a row, space-separated, with the printable-ASCII gutter — `xxd -g1`
with kuna's own address column, so the two are diffable. `--json` replaces
`instructions` with the contiguous span and its rows:

```json
{"binary": "...", "kind": "data", "target": {...},
 "start": N, "start_hex": "0x..", "end": N, "end_hex": "0x..",
 "count": N, "bytes": N, "truncated": false, "notes": ["..."],
 "hex": "000102030405060708090a0b0c0d0e0f",
 "rows": [{"address","address_hex","size","bytes","ascii"}]}
```

`hex` is the whole span in one piece and `rows[].bytes` is that same string cut
into sixteens — use either, never both. `count` is the number of listed entries
in both views (instructions, or hexdump rows); `bytes` is the span. A byte view
honors the requested end exactly, where an instruction listing overshoots to the
end of the instruction that straddles it. `notes` carries anything the command
would have said on stderr, so a `--json` caller never has to read two streams.

A listing stops at the end of **mapped memory**, whatever length was asked for.
The load image answers a read that starts on a mapped address for its whole
length, zero-filling anything the segments do not cover, so a window that runs
off the end of the code used to decode that fill: on a crackme whose executable
segment stops at `0x080d1904`, `kuna disassemble 0x80d18b0 --count 30` listed
eight `ADD byte ptr [EAX],AL` rows out of bytes that are not in the file, while
`kuna disassemble 0x80d190b` — an address in the same unmapped gap — correctly
refused. Both now agree: the listing clips to the mapped run holding the start,
an instruction that would straddle the end lists as `.byte`, and the stop is on
**stderr** and in `notes`, with the next mapped address to resume from.

```
$ kuna disassemble ./keygenme 0x80d18b0 --addr --count 30
note: the listing stops at 0x80d1904, where the segment holding 0x80d18b0 ends --
the bytes above it are not in the image, and the next mapped address is 0x80d2f50
```

A listing whose length nobody asked for is capped at 1024 instructions, flagged
`truncated` and marked in the header. The extent is only an upper bound — clipped
at the next discovered entry or the end of the CODE section — so where discovery
is thin one "function" can run to the end of `.text` (`main` in one unpacked
crackme clips to 19,106 instructions). An explicit `--count`, `--bytes` or range
is honored however long.

This is a query, not an engine change, and it reinvents nothing: the binary is
loaded once through the same in-process seam `decompile-all` uses
(`bootstrap_from_object` → `commit_pending_analysis`), and every row comes from
`Translate::print_assembly` — the seam the console's own `disassemble` command
(`IfcPrintdisasm`) and the `decompile-project` `.asm` export already print
through. Nothing is committed into the engine, nothing is decompiled, and no
emitted C changes. Verified against `objdump -d`: 19,368 instructions across four
binaries — a 32-bit x86 ELF, an x86-64 ELF, a stripped x86-64 PIE and an x86-64
PE — byte-identical at every address, with the same instruction boundaries.

A window the caller bounded — `--count`, `--bytes`, or an explicit `start-end`
range — does not pay for the program-wide function-discovery walk. That walk is
what `--mode` turns on for a whole-binary *decompilation*, and on a large image it
dominates: `auto` selects `fast` from 2 MiB up, whose retained discovery pass
decodes every executable byte, so listing 40 instructions of a 9.4 MB PE whose
`.text` is 99.4% of the file took 20.1 s, of which 40 instructions were 0.1 s.
Such a listing is answered from the load's own symbols instead (1.4 s, the same
bytes). The walk is not skipped, only deferred: when it is the only thing that can
answer — a name it invents (`sub_1190` on a stripped image), an address it alone
knows, a bare address in a data section whose view depends on there being an entry
— the command falls back to it and answers exactly as before. Naming `--option
listing` or `--option fast_funcdisc` yourself keeps your setting either way, and a
listing whose length came from the function extent always takes the full walk.

Exit codes follow the house contract: a listing is `0`; an unresolvable name or
an address with nothing mapped behind it is `1` with the reason on stderr (on a
packed image, run `kuna unpack` first — the original addresses do not exist until
you do); a malformed command line is `2` with the usage block.

## `kuna strings` — the string inventory

```bash
kuna strings ./a.out                                  # every literal, with the functions that use it
kuna strings ./crackme.exe --json                     # machine-readable
kuna strings ./a.out --filter '(?i)password|flag'     # regex over the text
kuna strings ./crackme.exe --encoding utf16           # wide Windows literals
kuna strings ./crackme --encoding utf8                # literals opening with a non-ASCII character
kuna strings ./a.out --section .rodata --min-length 8
kuna strings ./bundle --termination nul               # only the char[N] literals
```

The triage query: what text is in this binary, where does it live, and — the part
`strings(1)` cannot answer — **which function uses it**. Finding the prompt is
never the goal; opening the routine that prints it is, and that hop is one
command here because kuna already has both halves.

The rows come from the analyzer tier's own matcher, not a second scanner
(`kuna-analysis/src/analyzers/strings/`, the port of Ghidra's `StringsAnalyzer`),
over the address set that pass scans; under `--termination nul` a row here is
exactly a `char[N]` literal `kuna decompile` prints, at the same address. The
reference columns come from the same index behind `kuna xrefs`
(`kuna-analysis/src/listing/xrefs.rs`). Nothing is committed into the engine and
no emitted C changes.

| Column | What |
|---|---|
| `address` / `address_hex` | The **virtual** address of the first character byte — not a file offset, so it pastes straight into `kuna decompile --addr` or `kuna xrefs --to`. |
| `text` | The literal, terminator excluded. TAB/CR/LF are escaped in the text surface so a row stays one line; `--json` carries them verbatim. |
| `length` | Visible characters (code units for a UTF-16 row). `byte_length` is what it occupies, terminator included. |
| `encoding` | `ascii`, `utf8` or `utf16` — which reading found it. A row is `utf8` only when it actually holds a multi-byte sequence. |
| `nul_terminated` | The run ended at a NUL, so it is a C string and not merely printable text. Always `true` under `--termination nul`. |
| `section` | The section it lives in, `null` on an image scanned by segment. |
| `xrefs_count` | How many references land anywhere in the literal's extent, so `lea rax,[fmt+4]` still counts as a use. |
| `functions` | The functions those references come from, `{name, address, address_hex}` each. |

### Flags

`--encoding ascii\|utf8\|utf16\|all` (default `ascii`). `ascii` is the analyzer's
own 1-byte width. **`utf16` is not a convenience** — a UTF-16LE literal read at
1-byte width ends at the NUL after its first character, which is exactly why a
wide Windows API argument renders as `LoadLibraryW("n")` instead of
`L"ntdll.dll"`. The 2-byte matcher mirrors the 1-byte one exactly (same character
recognizer, same require-NUL-end rule, same minimum), over units on even
addresses. Scope is UTF-16**LE** whose units are in the 1-byte charset — the
Windows-API case; a big-endian or non-Latin wide literal is not recovered.

`utf8` is the other reading of the **1-byte** width, not a third width. The
recognizer is ASCII, so a byte `>= 0x80` ends a run and a literal that opens with
a non-ASCII character is reported from the byte after its last multi-byte
sequence — an address nothing in the image refers to, so that row also comes back
with `xrefs_count 0` and no `functions` while `kuna xrefs --to` the literal's real
start answers one. `utf8` decodes well-formed sequences as characters, which puts
the row back on the address the code loads and lets the existing reference walk
fill the last two columns:

```
$ kuna strings ./no-standards --encoding all --json --filter magical      # before
  "address_hex": "0x200c", "text": ")/ so what was the magical keycombination? ",
  "length": 43, "encoding": "ascii", "xrefs_count": 0, "functions": []
$ kuna strings ./no-standards --encoding all --json --filter magical      # now
  "address_hex": "0x2000", "text": "＿φ( °-°)/ so what was the magical keycombination? ",
  "length": 50, "encoding": "utf8", "xrefs_count": 1, "functions": [{"name": "sub_1011", …}]
```

It is a superset of the ASCII reading rather than a rival to it — a continuation
byte is never in the 1-byte charset, so no decoded sequence can swallow a byte
the ASCII matcher would have taken, and an ill-formed sequence costs one byte and
the scan carries on. So `all` takes it *in place of* the ASCII reading (running
both would report the same text twice, once at a truncated address), a row with
no multi-byte content is still reported as `ascii`, and an image with no
multi-byte content reads identically under `ascii` and `utf8`. `--min-length`
counts characters, so a 5-character kaomoji in 12 bytes is 5. Overlong encodings,
surrogates, out-of-range lead bytes and control characters are all declined, so
none of them joins two neighbouring runs into one.

`--termination nul|any` (default `any`). Ghidra's `requireNullEnd` takes only a
run closed by a NUL, because the markup it plants is a `char[N]` and nothing else
describes one. As a report that rule loses whole regions: a length-prefixed name
table — `\x0cout.js\x06std\x12_0x8ec6b3`, the shape a bundled JS or bytecode
payload carries — holds no NUL at all, so
`--section .rodata --filter '_0x|out.js'` over a 977 KB Node bundle answered
`count 0` where `strings -a` read 635 names. The default is therefore `strings(1)`'s
rule over kuna's address set, and `nul` restores the pass-faithful view. Both
report `nul_terminated` per row, so a C literal stays distinguishable from a
printable fragment, and both report the policy in the `termination` field and the
text header.

The cost is the same one `strings(1)` pays: a printable run inside code is
usually instruction bytes. On a 1 MB x86-64 image the relaxed scan reports 5,138
rows against 1,776 — 2,565 of the additions are in `.text`. `--section` and
`--filter` narrow it; `--termination nul` removes it entirely.

`--min-length N` (default `5`, the analyzer's own `minStringLength`).

`--filter REGEX` matches anywhere in the text. The flavor is
`. * + ? | () [] {n,m} ^ $`, the `\d \w \s` shorthands and their negations,
backslash escapes, and a leading `(?i)` for case-insensitive matching; groups are
always non-capturing. Anything outside that — a lookaround, `\b`, `\xNN` — is a
command-line error (exit `2`), never silently reinterpreted into a different
pattern. Backtracking is budgeted: a pathological
pattern reports those rows as non-matching with a warning on stderr rather than
hanging.

`--section NAME` restricts the scan to one section; the leading `.` is optional
(`--section rdata` finds `.rdata`). A section the image does not have is exit `1`
naming the ones it does.

`--no-xrefs` skips the reference walk — the expensive half, since it loads and
lifts the program. Rows still carry text, address, and section; `xrefs_count` is
`0` and `functions` empty.

Plus the shared `--json`, `--mode`, `--option N V`, `--slice`, `--target`,
`--sleighpath`.

### Output

```
# 8 strings in ./SCORPiON.exe (ascii, min length 5, termination any, scanned by sections)
0x416030	ascii	15	.data	1	sub_401160	Correct serial!
0x41605c	ascii	16	.data	1	sub_401160	E24546F5F6B39F59
0x41613c	ascii	14	.data	1	sub_401350	%[^-]-%[^-]-%s
```

A `#` header naming the query, then one tab-separated row per string: address,
encoding, length, section, reference count, referencing functions, text. Text is
last because it is the only unbounded column.

```json
{"binary": "...", "encoding": "ascii", "min_length": 5, "filter": null, "section": null,
 "termination": "any", "scanned": "sections", "xrefs": true, "count": N,
 "strings": [{"address","address_hex","text","length","byte_length","nul_terminated",
              "encoding","section","xrefs_count",
              "functions": [{"name","address","address_hex"}]}]}
```

### What it deliberately does not report

The scan covers the **loaded and initialized** address set — the allocated,
file-backed sections, which is Ghidra's `getLoadedAndInitializedAddressSet`. So
`.strtab`/`.symtab` symbol names and `.comment` are absent whatever
`--termination` says: those are not in the image the program runs, and the ones
that name something are already in `kuna functions`. That is the narrowing
against `strings -a`, which reads the whole file.

An image with no usable section table — a UPX-packed ELF keeps its program
headers and nothing else — falls back to its `PT_LOAD` segments, and the
`scanned` field says which set was walked (`sections` or `segments`). On a packed
image the answer is the packer's own data; unpack first and ask again:

```bash
kuna unpack ./packed -o ./unpacked && kuna strings ./unpacked --filter '(?i)flag'
```

A binary with no strings is exit `0` with `count: 0` — an answer, not a failure.
An unreadable or unparseable binary, or an unknown `--section`, is exit `1` with
the reason on stderr; a malformed command line is exit `2` with the usage block.

## `kuna unpack` — statically unpack a UPX- or NEOLite-packed executable

```bash
kuna unpack ./packed                                   # writes ./packed.unpacked
kuna unpack ./packed -o snake.bin --json
kuna unpack ./stripped --raw-lzma 0x1403ea000:0x14053e47c -o payload.bin --json
```

The first move on a packed binary, and the only one that helps: every other kuna
surface is honestly useless on one. A UPX-packed file contains a loader stub and a
compressed blob, so `kuna functions` finds nothing, and decompiling the entry point
gives you the decompressor. `kuna unpack` reconstructs the original image so the rest
of the CLI has a program to work on — on the witness that filed this gap, `kuna
functions` goes from `count: 0` to 70, `main` included.

Two packers are recognized, and which one is decided before anything is decoded, so
the answer on a file that is neither is unchanged. A **NEOLite** image announces
itself with a `.NEOpack` section that owns the entry point; everything else goes to
the UPX arm, whose `no UPX PackHeader found` is still what an unpacked file gets.

It runs **in-process**, with no external tooling: `upx -d` cannot be assumed present
wherever a release `kuna` runs, and handing a hostile binary to a packer to look at it
is not a thing an analyzer should do. The UCL NRV2B / NRV2D / NRV2E decompressors, the
LZMA1 decoder and the branch-target filters are reimplemented in
`kuna-analysis/src/upx/`.

Default output is `<binary>.unpacked`, overwritten if it exists (the name is
unambiguously this command's own artifact, and a command that fails its second
invocation is worse than one that rewrites its own output). `--json` emits
`{binary,output,packer,loader_version,format,format_name,method,method_name,level,
filter,filter_hex,pack_header_offset,pack_header_offset_hex,packed_size,
compressed_size,unpacked_size,count,blocks:[{offset,offset_hex,u_len,c_len,method,
method_name,filter,filter_hex,stored}]}`, where `count` is the number of compressed
blocks consumed.

### NEOLite

```bash
kuna unpack ./CryptoME.exe --json     # "packer": "neolite"
```

NEOLite (NeoWorx, ~1999) compresses each original section **in place** and appends
`.NEOpack`, the loader stub, and `.NEOdata`. The original section table survives
untouched, so every virtual address and virtual size still describes the original
image and nothing has to be inferred by running the stub — which is what the tester
who filed this had to do, in Unicorn. The codec is an LZX derivative (58 position
slots, three repeated offsets, `256 + slot * 8 + len_slot` main symbols) with
DEFLATE's length tables and code-length transmission; `kuna-analysis/src/neolite.rs`
decodes it.

Three things are restored beyond the bytes: the entry point moves off the stub to
the `push OEP; ret` operand the stub hands over with, the import data directory is
re-pointed at the program's own descriptor array (found in the recovered `.rdata`,
not the stub's decoy), and the section the entry point lands in gets its
`CNT_CODE | MEM_EXECUTE | MEM_READ` back — the packer rewrites every section to plain
read-write data, and a `.text` that does not say it is code is a section no
disassembler walks. A section the packer stored uncompressed — resources usually are
— is carried through as it stands and reported `"compressed": false`; it fails the
code-table validity test on its first block header, which is what keeps plaintext
out of the decoder.

`--json` emits `{binary,output,packer,entry,entry_hex,stub_entry,stub_entry_hex,
import_directory,packed_size,compressed_size,unpacked_size,count,sections:[{name,va,
va_hex,packed_size,unpacked_size,compressed}]}`, where `count` is the number of
sections that held a stream. On the witness (`CryptoME.exe`, 139,776 bytes) that is
3 sections and 447,488 bytes out, after which `kuna functions` goes from 0 to 387
with import names, and the recovered entry decompiles to the MSVC CRT startup.

Only 32-bit x86 PE32 images are rebuilt; a `.NEOpack` image with any other optional
header magic exits `1` naming it rather than rebuilding a layout nothing has been
checked against. The original imports come back but the packed image's own
`.NEOpack`/`.NEOdata` sections are kept, so the stub is still there to read.

**Coverage, and the failure contract.** Implemented: the ELF formats and 32-bit
`win32/pe`, methods 2–10 (NRV2B/NRV2D/NRV2E in their `_LE32`, `_LE16` and `_8` bit
layouts) and 14 (LZMA — what `upx --lzma` and `upx --best` write, and what most
recent packed binaries carry), and the x86 `cto`/`ctoj`/`ctok` and ARM/AArch64 branch
filters. A PE keeps
its `PackHeader` in the header padding rather than the tail, and its payload in one
block followed by a trailer the unpacker replays: the original PE header and section
table, the import descriptors and names UPX stripped out of the image, and the
resources it moved out of it all come back, so the recovered file has a working
import table and not just working code. Everything else — the remaining methods
(CL1B, DEFLATE, ZSTD, BZIP2), the 64-bit and ARM PE targets, the remaining non-ELF
targets, a packed shared library, a
PE whose original image had base relocations, TLS or delay-loaded imports, the
pre-12 loader block layout, the `ctojr`/PowerPC/RISC-V/delta filters — exits `1`
with the thing it cannot do **named**, and writes no file:

```text
error: ./x: unsupported UPX image: compression method 15 (DEFLATE)
error: ./x: unsupported UPX image: unimplemented UPX filter 0x80 (ctojr32: …)
error: ./x: no UPX PackHeader found
```

**`--raw-lzma` — when discovery cannot work.** A repacker that strips the
`PackHeader`, or a private packer that only borrows UPX's codec, leaves an image whose
payload is perfectly decodable and whose metadata is gone. `no UPX PackHeader found` is
then a correct answer and a dead end, because the one thing that gets past it —
*where the stream is* — is knowledge the reader already has:

```bash
kuna unpack ./keygenme.exe -o payload.bin --raw-lzma 0x1403ea000:0x14053e47c --json
#   "unpacked_size": 5489913,  "end_marker": false
```

`--raw-lzma START:END` skips discovery and decodes that range as one raw LZMA1 stream.
`END` is exclusive and `START+LENGTH` is accepted; endpoints are virtual addresses read
in hex — what you copy out of `kuna disassemble` or `kuna xrefs` — clamped to the bytes
the file actually stores for the section. `--raw-offsets` reads them as file offsets
instead, which is also the only reading available for an image no object parser
recognises.

Nothing declares the uncompressed size in this situation, so the decode runs to the end
of the *input* rather than to a length: **the size is a result, not a parameter**.
`--raw-max-size N` caps it (default 512 MiB) so a range that is not really a stream
fails instead of running until memory does. By default the range's first two bytes are
read as UPX's parameter prefix (`pb` in the low three bits of the first; `lc` and `lp`
in the low and high nibbles of the second); `--lzma-props pb,lp,lc` — or those two bytes
as hex, `--lzma-props 0x1a03` — overrides that for a stream carrying no prefix, and then
the range is stream body from its first byte. `--json` emits
`{binary,output,packer,codec,range_kind,range_start,range_start_hex,range_end,
range_end_hex,file_offset,file_offset_hex,lc,lp,pb,packed_size,compressed_size,
compressed_read,unpacked_size,end_marker}`.

What comes out is a payload, not a rebuilt executable — imports, relocations and a PE
header are not reconstructed, and `end_marker: false` is the normal answer because UPX
writes none. It is bytes to carve further, not a program to hand back to `kuna
decompile`.

That asymmetry is deliberate. A wrong unpacked binary is far more expensive than no
output at all: an unreversed filter leaves every call target in the file pointing
somewhere wrong while every size still adds up, the ELF still parses, and a reader has
no way to tell. So a run either produces the original file or refuses. Success is not
assumed from "it decoded" either — the walk requires the block stream to end on the
`UPX!` marker adjacent to the PackHeader (on a PE, to account for every byte of the
packer's trailer), to total exactly the original file size, and to reproduce **both**
of the packer's own Adler-32 checksums (a flipped literal byte
decodes to a wrong image of exactly the right length; only the checksum catches it).

## `kuna decompile-project` — recompile-oriented project export

```bash
kuna decompile-project ./a.out                         # writes ./a.out.kuna/
kuna decompile-project ./a.out -o proj --functions main,parse
kuna decompile-project ./a.out --jobs 12               # the same folder, 12 processes
kuna decompile-project ./a.out --stream --jobs 12      # readable while it fills
```

The project-export face of the same in-process core
(`decompiler/crates/kuna-cli/src/decompile_project.rs`, a thin wrapper over the shared
`kuna_console::project` module — the decompile loop + artifact builders also behind the
web UI's Download-Binary-Source zip and `kuna_wasm project`). Identical
load-once/decompile-many path and flags —
`--functions`/`--addr`/`--max-fn-seconds`/`--mode`/`--option`/`--isa`/`--slice`/
`--target`/`--sleighpath`/`--jobs`/`--stream`; no `--json`. Omitted mode is the same
size-based `auto` policy as the other file front-ends. In particular, a project input
at least 2 MiB automatically suppresses the exhaustive Listing consumers, prologue
scan, and AIF gap walk through the `fast` preset, while substituting rooted direct-call
and bounded pointer-table discovery. Its unfiltered per-function watchdog also
defaults to 10 seconds instead of 120; `--max-fn-seconds` overrides it,
including `0` to disable. Explicit `--addr` selections remain exact and
suppress that whole-image walk by default; named selections keep it so
generated names can resolve. Explicit `--option fast_funcdisc on` can restore
its program facts for an address-selected run, but does not add definitions
outside the selection.

Writes a project folder — default `<binary-filename>.kuna/` next to the binary,
`-o/--output DIR` overrides — of four artifacts designed so a human or LLM can study the
binary and attempt recompilation:

- `<name>.c` — every decompiled function, address-ordered (decompile order under
  `--stream`), under `// Function: <name> @ <addr>` headers, failures as comments,
  `#include "<name>.h"`.
  One definition per loader- or analysis-discovered executable entry address:
  the export shares
  `decompile-all`'s CODE-backed target policy and one-record-per-entry
  enumeration above, so data import slots are not rendered as functions and a
  function carrying several names cannot produce several identical definitions.
- `<name>.h` — include guard + a generated recompile prelude (core scalar and
  `undefined`-family typedefs), the recovered user-defined type definitions, and one
  prototype per decompiled function, token-identical to the `.c` definition line.
  These are recovered signatures, not source-language declarations invented by the
  exporter. In particular, a `main` whose return register is not recovered can appear
  as `void main(void)`, which strict C compilers reject because `main` is a reserved
  entry-point name. A syntax-only consumer can remap it while including the header
  (`#define main kuna_recovered_main`); making it `int` would assert ABI information
  the decompiler did not recover and would break the header/definition identity.
- `<name>.asm` — labeled linear disassembly of every CODE section: labels match the `.c`
  function names, per-function `; arg:`/`; stack:` comments map decompiled variables to
  storage, undecodable bytes as `db` lines, and a `; --- data ---` tail labeling named
  globals plus every `dat_<hex>` the `.c` references, with raw bytes. Data-tail addresses
  use their source address space's units; a `dat_<hex>` label retains the coordinate
  printed in C, and aliases a named symbol only in the same address space at the same
  displayed coordinate.
- `README.md` — size, arch id, entry point, function counts, sections table, file
  inventory. The entry point is the container's, reported through the inventory
  (an ARM entry carrying the Thumb bit prints at its even address) and
  `unavailable` for a container that declares none, such as a relocatable
  object. The sections table lists the loader's named, mapped sections at their
  load addresses — for a relocatable object, the synthetic layout the rest of
  the export uses, not every section at file offset zero.

The artifact format is purely additive and has no exporter-specific transform
(spec §9.7); the set of emitted definitions follows the selected P1 discovery
options, including `fast_funcdisc`.

Under `--jobs N` the `.h` needs one extra step. Its type block renders the
architecture's type factory *after* the loop, and a decompile can intern a type
into it, so a sharded parent's factory would be missing whatever the workers
recovered. Each worker therefore renders its own block and sends it back: when
they agree — which is what a shard that interned nothing renderable looks like,
and what every fixture measured here does — that block is the serial answer and is
emitted as is. When they disagree the parent says so on stderr and emits their
ordered union, so the `.h` still declares everything the `.c` uses.

### `--stream` — the folder is readable while it fills

```bash
kuna decompile-project ./big.bin --stream --jobs 14
```

An ordinary export writes its folder at the end, so the whole run is dead time for
whoever is waiting on it: on a 147 MB image with 392,814 functions that is about
twenty-one minutes (1,248 s) at `--jobs 14` on this box before the first line of C
exists, and hours serially. `--stream` writes the same folder as the run goes and
starts at the entry point, so the part of the program a reader asked about first is
the part that is there first. It is driver policy — a flag, not a phase-model option —
and it is off by default.

**What appears when.** Before the load, which on a large image is the longest single
wait in the run, the folder is created and two files are written into it: `README.md`,
carrying the binary's path and size with `pending` in every cell the load has not
answered yet, and `.streaming`, phase `loading`, which keeps ticking on its own clock
for as long as the load runs so `updated_at` and `elapsed_s` separate a live load from
a dead process. Nothing else is touched, so a previous export of the same binary keeps
its `.c`/`.h`/`.asm` — and, if this run fails while loading, its `README.md` — while a
new one loads. Once the program is loaded, the C-only check passes and the target set
is non-empty,
the four artifacts plus `index.jsonl` are truncated and created — the `.c` with its
`#include`, the `.h` with the prelude and a pending type block, the `.asm` with its
two header lines, `index.jsonl` empty — and `.streaming` moves to phase `decompiling`
carrying `functions_total`. From there the decompile, the disassembly sweep and the
writer run at once: the `.asm` completes early, and each function is appended to the
`.c` and then announced in `index.jsonl`. Finalisation puts the real type block in
the `.h`, appends the `.asm` tails, rewrites the README and removes `.streaming`.

**The order.** Two seeds start it: the image entry point (resolved through the
inventory, so an ARM entry carrying the Thumb bit lands on its even address) and
`main` by name, each taken only when it is one of this run's targets. Every finished
function reports the entries it reaches — direct `CALL` destinations, plus constants
that are known function entries inside a code section, which is the edge that reaches
`main` from `_start` through `__libc_start_main`. Intersected with this run's own
target set, those are the frontier the scheduler works breadth-first; when it empties, the
remaining targets follow in address order. The hints are a scheduling input and
nothing else: they enter no artifact, and an `--addr`/`--functions` export does not
grow a callee it was not asked for. When neither seed is a target the run says so on
stderr and the order is address order throughout. Under `--jobs N` the frontier is
served to N workers and the `.c` interleaves in completion order, which is not
reproducible run to run — and a frontier only leads while it is deep enough to feed
the pool. The 147 MB image's entry point calls three functions, which fills one chunk,
so at `--jobs 14` the other workers start on the address cursor at once and the seed
neighbourhood is a short prefix of the `.c` rather than its whole first wave. At
`--jobs 1` the order is the plain breadth-first walk.

**How the artifacts differ.** The function set is identical to a non-stream export of
the same selection; the layout differences below are what append-only costs, and they
are the contract:

- `<name>.c` — decompile order, not address order. A block is appended with one write
  and is never rewritten, so an offset a reader took stays valid; `index.jsonl` is the
  address-to-offset index that replaces the address ordering.
- `<name>.asm` — the sweep is byte-identical to a non-stream `.asm` with its
  `; arg:`/`; stack:` comment blocks removed from under the labels. Those blocks are
  appended afterwards in a `; --- variables ---` section, ahead of the unchanged
  `; --- data ---` tail, because the sweep finishes long before the variables exist
  and an append-only file cannot go back under a label. The `; --- variables ---`
  marker is written whether or not any function has variables — it is where a reader
  splits the sweep from the tails.
- `<name>.h` — the same prototype set, in the `.c`'s order. It is rewritten whole and
  atomically as prototypes accumulate, so it is always readable, but it carries no
  type definitions until the export completes and does not compile against the `.c`
  until then. At `--jobs N` its final type block is the worker-shard merge described
  above.
- `index.jsonl` — new, and kept after the run.
- `.streaming` — new, and removed when the run succeeds.
- `README.md` — rewritten whole at most every five seconds. While the export runs it
  opens with a streaming banner and a `## Streaming status` table; its file inventory
  describes the streamed layout, not the address-ordered one.

**`index.jsonl`** is append-only: one compact JSON object per line, written *after*
that function's `.c` block.

```
{"seq":0,"name":"_start","addr":"0x400580","size":44,"c_offset":23,"c_len":270,"error":null}
```

| Field | Meaning |
|---|---|
| `seq` | Zero-based, in write order. |
| `name` / `addr` | The function and its display address as a `0x…` string — the two the block's `// Function:` header prints. |
| `size` | The inventory extent in bytes, as a number; `0` when the extent is unknown. |
| `c_offset` / `c_len` | The block's byte span in the `.c`: `c[c_offset..c_offset+c_len]` is exactly `// Function: …` through the block's trailing blank line. |
| `error` | `null`, or this function's error string — the block is then the one-line `// Function: … (error: …)` comment form. |

A line exists only once its block is whole, which makes it the torn-read oracle for a
reader following the `.c`, the per-function progress feed, and the index a
decompile-ordered `.c` needs. The line itself is one write, but nothing promises a
write is atomic: a reader polling a running export ignores a final line that does not
end in a newline and picks it up on the next read.

**`.streaming`** is one line of compact JSON, replaced atomically, present only while
the export runs.

```
{"schema":1,"phase":"decompiling","pid":2927817,"started_at":1789162729,"updated_at":1789162747,
 "elapsed_s":17,"jobs":4,"seeds":1,"functions_total":3153,"functions_done":1570,
 "functions_failed":1,"seconds_since_last_result":0,"c_bytes":2316041,"asm":"complete","error":null}
```

| Field | Meaning |
|---|---|
| `schema` | `1`. Bumped whenever a field is added, removed or changes meaning. |
| `phase` | `loading`, `decompiling`, `finalizing` or `failed`. |
| `pid` | The exporting process, so a reader can tell a live export from an abandoned one. |
| `started_at` / `updated_at` | Unix seconds: when the export started, and when this file was last written. |
| `elapsed_s` | Seconds since `started_at`. |
| `jobs` | The `--jobs` request while `phase` is `loading`, and the workers actually spawned once the pool opens — `--jobs N` asks, the pool's memory trim answers. On a large image the load is most of the run, so a poller reading this during it is reading the request. `1` is the serial run. |
| `seeds` | How many seeds the order started from. `0` means neither the entry point nor `main` was a target, so the order is address order. |
| `functions_total` | Targets for this run. `null` while `phase` is `loading`. |
| `functions_done` | Results written so far, failures included. |
| `functions_failed` | How many of `functions_done` are error records. |
| `seconds_since_last_result` | How long the decompile has been quiet — the staleness signal. |
| `c_bytes` | Size of the `.c` as of this write. |
| `asm` | `pending` (the sweep has not started), `sweeping`, or `complete`. An image with no CODE section has nothing to sweep and reports `complete` from the first `decompiling` write — it is `pending` for as long as the phase is `loading`, since the target set that decides there is nothing to sweep is not resolved yet. |
| `error` | `null`, or why the run stopped. Non-null only with `phase: failed`. |

It is written on the writer's own 500 ms clock rather than per result, so it can trail
`index.jsonl` by up to one tick; the index is the live feed and the status file is the
summary.

**Failure is reported in the folder.** Any error after the folder exists — an
unloadable image, a non-C output language, an empty target set, an I/O error on the
`.c`, the `.h` or `index.jsonl` — rewrites `.streaming` with `phase: failed` and the
message, and exits `1`. `.streaming` always carries the message; the README carries it
too, in its `## Streaming status` table, except in the one case where a previous
export's README is put back instead. That case is a failure before the run has
truncated anything of its own, which leaves the folder as it found it: a previous
export's `.c`/`.h`/`.asm` are untouched and its `README.md` is restored byte for byte,
so `.streaming` is the only trace of the attempt. With no previous README to put back,
a run that failed that early writes one that says so and inventories nothing, because
nothing was created. A `.streaming` left behind whose `pid`
is dead and whose phase is not `failed` means the run was killed.

One case reports nothing in the folder: if whatever killed the writer also makes
`.streaming` unwritable — a full disk is both — the status file keeps its last good
contents and only stderr and the exit code say the run failed.

**Stopping takes a chunk.** When the writer dies the producers stop rather than
decompile the rest of the binary into a channel nobody reads, but they stop at a
boundary: `.streaming` flips to `failed` within a tick, while each worker first
finishes the chunk it is holding — up to 64 functions apiece, so a `--jobs N` run
ends within one chunk rather than instantly, and `--jobs-chunk K` bounds that
directly. At `--jobs 1` the producer pulls one function at a time and stops at the
next one. The pool's closing `done: N functions` line counts what it actually
delivered, which on a run that stopped early is less than `functions_total`.

Two things are deliberately not run failures. Per-function failures — including a
worker process that cannot be spawned, which degrades that whole chunk to error
records — are `error` records in the `.c` and in `index.jsonl`, and the run exits `0`
with `functions_failed` counting them, so a poller that sees no `failed` phase still
has to read that field. And a `.streaming` or `README.md` rewrite that fails mid-run,
since those report on the export rather than being it, warns once on stderr per file
per outage and is retried on the next tick — only the first pair, written at t=0 as the
proof the folder can be written at all, fails the run.

A binary that does not exist is refused before anything is created, and never leaves a
folder behind. Which refusal you get depends on the mode: the default `--mode auto`
stats the file to size the mode while arguments are parsed, so it exits `2` with the
usage block and `error: cannot read input binary metadata for mode auto: …` (this is
`decompile-project`'s own behaviour, with or without `--stream`); an explicit
`--mode fast|reliable|aggressive` skips that stat, and the streamed export's own check
exits `1` with `error: binary not found: …`. A folder another live export is already
streaming into is refused too (`.streaming` carrying the pid of a running process) —
the second run would truncate the first's `.c` and invalidate every offset a reader had
taken from it. That refusal names the way out: wait, pass `-o`, or delete the
`.streaming` if the process it names is gone.

**Refused.** `--stream` with `--assert` exits `2` without creating the folder: a
streamed export decompiles every function in turn, so an unqualified directive would
bind to all of them. `--stream` is a `decompile-project` flag only — `decompile-all` and
`decompile-graph` reject it as an unknown option, because each produces one document
that does not exist until it is complete. `--jobs`'s own refusals (`--assert`,
`--raw-image` with a pool) are unchanged.

**`--jobs 1` streams too.** A serial run of more than a few dozen functions prints one
line on stderr (`[kuna --stream] serial run; --jobs auto uses every core`) and then
interleaves: the
sweep is cut into sixteen steps alternating with decompile batches that grow from one
function to sixty-four, so the first `.c` block lands after the first function and the
`.asm` is complete within the first few hundred, after which the rest of the run is one
uninterrupted loop. The interleave is not what makes a serial run slow — on a 3.4 MB
PE with 3,153 functions it measured 1m50.8s streamed against 1m52.4s non-stream.

**The engine caveat `--jobs` carries applies here at every job count.** A handful of
type and string decisions are first-toucher-wins inside one process's database, so they
follow the *set* of functions that process decompiled and the order it saw them in.
`--stream` changes that order deliberately, including at `--jobs 1`, so a few bodies —
and with them their variable comments and the `dat_` tail they drive — can differ from a
non-stream export of the same binary. A non-stream `--jobs 1` run remains the definition
of the answer.

**What it buys, and what it costs.** The 147 MB stripped PIE x86-64 image the flag
was built for — 392,814 functions — at `--jobs 14`, against a non-stream run of the
same command on the same machine:

| Step | `--stream` | non-stream |
|---|---|---|
| output folder exists | 0.04 s | 1,248.3 s |
| first `.c` block readable (first `index.jsonl` line) | ~91 s | 1,248.3 s |
| `.asm` sweep complete (1.55 GB) | ~143 s | 1,248.3 s |
| export complete | 1,400.4 s | 1,249.8 s |
| parent peak RSS (with its 14 workers) | 12.8 GiB (34.7 GB) | 13.0 GiB (29.5 GB) |

Availability is what it buys and the tail is what it costs. The folder is worth
reading nineteen minutes before the non-stream one exists — everything before ~91 s
is the load, which is the same wait either way — and it finishes 12% later, because
the static longest-first plan a non-stream `--jobs N` run uses is close to the best
makespan available and entry-point-first order is not. At this size that is about
two and a half minutes of tail for twenty minutes of head start; on a small binary
the sweep's overlap with the pool wins the end-to-end comparison back.

## `kuna decompile-graph` — the whole program as one JSON graph

```bash
kuna decompile-graph ./a.out                           # to stdout
kuna decompile-graph ./a.out -o graph.json --label v3  # to a file
kuna decompile-graph ./a.out --functions main,parse    # every node, two bodies
kuna decompile-graph ./a.out --jobs 12                 # the same document, 12 processes
```

One document holding every discovered function — its recovered signature,
parameters, C body and assembly — plus the call edges between them
(`decompiler/crates/kuna-cli/src/decompile_graph.rs`). The same in-process
load-once path and the same flags as `decompile-project`
(`--functions`/`--addr`/`--max-fn-seconds`/`--mode`/`--define-function`/
`--option`/`--slice`/`--target`/`--sleighpath`/`--jobs`; no `--json`, the document
always is), plus `-o/--output FILE` and `--label TEXT`, which is copied verbatim into
`binary.label` for a consumer that wants to stamp the document with its own
version. Written to stdout when `-o` is absent; with `-o` the file is the only
output.

**The document is C.** `codeC` names its language, so this surface refuses any
other — `--language rust` (or `--option setlanguage rust-language`) is an error
rather than Rust in a field called `codeC`, and the auto policy that follows a
rustc-built binary is off here for the same reason it is off for
`decompile-project`. Use `kuna decompile` or `decompile-all --json` for the other
output languages.

**`address` is the key, not `name`.** A name repeats inside one document
whenever several addresses stand for one callable: a PLT thunk and the import
slot it forwards through are both `printf`, and a Mach-O image carries the two
plus its stub. A consumer keying rows or edges by name will collide.

**Every discovered function is a node.** A decoded forwarding relation also
supplies its pointer-slot node when the loader inventory omitted it, as ELF does
for a GOT slot behind a named PLT veneer. `--functions`/`--addr` narrow which
nodes get a decompiled *body*, not which appear — so `--functions main` buys the
whole call graph plus one body, at the price of one decompile. The bodies an
unfiltered run renders are exactly the ones `decompile-all` renders (the
CODE-backed target policy above); an address outside that policy is a labelled
row with no body even when `--addr` names it explicitly, and the run says so on
stderr.

**Both ends of every edge are rows of the same document.** Edges are the
`kuna xrefs` reference edges, walked through the same call-graph model
`--reachable-from` uses, and they carry that command's `kind` vocabulary: a
reference into the middle of a body resolves to the body, and one landing in no
discovered function (a `CALL 0x0` off a nulled relocation, a branch into a gap,
a materialized address that is a string) is not a call-graph edge and is not
emitted. Two runs of one command are byte-identical.

### The JSON document

```
{schemaVersion: 4,
 binary: {name,label,sourcePath,analysisImageBase,functionCount,edgeCount},
 functions: [{address,name,size,kind,parameters:[{ordinal,name,type}],signature,
              assembly,codeC,error,hasIndirectCalls,forwardsTo,isEntryPoint}],
 edges: [{callerAddress,calleeAddress,kind,calleeOrder}]}
```

| Field | Meaning |
|---|---|
| `schemaVersion` | `4`. Bumped whenever a field is added, removed or changes meaning. |
| `binary.label` | The `--label` string, `""` when not given. Never interpreted. |
| `binary.analysisImageBase` | The PE optional-header ImageBase, else the lowest non-empty loadable segment VMA — the same static VMA space as every address below. `null` for a relocatable object, which has no static base. |
| `address` / `size` | The node address and its byte extent. Loader-inventory nodes carry the same two numbers `kuna functions` reports; a forwarding pointer slot recovered only by the graph has size `0`. `address` is the document's only unique key — see above. |
| `kind` | `normal` a body of its own; `thunk` a body that only forwards (a PLT/stub-section entry, an imported name, or a lone jump); `import` a pointer slot the program calls through (a PE `.idata` entry, an ELF GOT slot recovered from its PLT veneer, or a Mach-O `__got`/stub slot); `data` any other named address that is not code (a Mach header symbol, an Objective-C class object); `external` a loader-defined undefined symbol with no bytes here at all. The last three are the rows with no body: this surface never decompiles an address that is not executable content, not even one `--addr` names. |
| `parameters` | The recovered parameters in ABI order. Empty for a row with no body. |
| `signature` | The `.h`-style prototype line, without the trailing `;`. `null` for a row with no body. |
| `assembly` | The function's instruction listing, one `<vma>  <MNEMONIC operands>` per line — the `kuna disassemble` walk, so an undecodable byte inside the body is a `.byte 0x..` row rather than the end of the listing. Present whenever a body was attempted, including when the decompile failed: the listing is what is left to look at. |
| `codeC` | The decompiled body, byte-identical to this function's `decompile-all --json` `code`. |
| `error` | Why this function has no `codeC`, when the decompile was attempted and failed. `null` with a `null` `codeC` means no body was attempted: a bodyless `kind`, or a `--functions`/`--addr` narrowing that did not select it. |
| `hasIndirectCalls` | The body contains a computed call (`CALLIND`). It files no edge when its destination is computed at run time; a `call qword ptr [slot]` through a fixed slot does file one, and sets this flag too. An indirect *branch* is not one — see `forwardsTo`. The call site is attributed to the row that contains it, the same rule that decides which function `kuna xrefs --from` lists an instruction under. |
| `forwardsTo` | Where a forwarding entry sends control: the destination of a direct lone jump, or the fixed pointer slot an indirect one reads. A non-null target is also a row and a `jump` edge endpoint. The slot half needs the jump to name it as a decode-time constant, which an x86 `jmp [rip+disp]` stub does and an AArch64 `adrp`/`ldr`/`br x16` stub does not — a Mach-O `__stubs` entry is therefore `kind` `thunk` with a `null` `forwardsTo`, and the import slot it reaches is a row of its own found by name. `null` for anything that does not forward. |
| `isEntryPoint` | This row is the image's declared entry point, resolved through the inventory so an ARM `e_entry` carrying the Thumb mode bit still lands on it, and rebased so a Mach-O `LC_MAIN` — which states a `__TEXT`-relative file offset rather than a VMA — marks the row it names. A format that declares no entry point marks no row. |
| `edges[].kind` | The `kuna xrefs` kind, so the two surfaces cannot disagree: `call` a call, direct or through the fixed slot an imported API is called through; `jump` a tail call or a branch into a neighbouring entry; `data` an address handed to something else to call — the edge that gives `main` a caller, since `_start` passes it to `__libc_start_main` as a pointer rather than calling it. A caller that both calls and mentions one callee gets one edge carrying the strongest of the two. |
| `edges[].calleeOrder` | Contiguous and zero-based per caller, in first-reference order, deduplicated on the callee. |

Rows are entry-VMA ordered, and each caller's edges follow that caller's order.
A field the program cannot supply is `null`, never a placeholder; a field that
could never be supplied is not carried at all, which is why there is no
module-qualified callee — the loader retains no library-module mapping.

```json
{
  "address": 4195940,
  "name": "authenticate",
  "size": 137,
  "kind": "normal",
  "parameters": [
    { "ordinal": 0, "name": "param_1", "type": "char *" },
    { "ordinal": 1, "name": "param_2", "type": "char *" }
  ],
  "signature": "unsigned long authenticate(char *a0,char *a1)",
  "assembly": "00400664  PUSH RBP\n00400665  MOV RBP,RSP\n...",
  "codeC": "unsigned long authenticate(char *a0,char *a1)\n{\n...",
  "error": null,
  "hasIndirectCalls": false,
  "forwardsTo": null,
  "isEntryPoint": false
}
```

Design notes and the reasoning behind each rule: spec §9.7.

## `kuna docs` — the manual, inside the binary

```bash
kuna docs                 # the topics, one per line, with a one-line summary
kuna docs cli             # print one of them
kuna docs --json          # [{topic, title, summary, bytes}]
kuna docs --all           # everything, concatenated, for piping into a context window
```

Every document is embedded at compile time with `include_str!`, so a release binary carries
its own manual and needs no checkout, no network and no `--sleighpath`. That is the point:
an agent handed only the binary can still discover the option catalog and the JSON schemas
it needs to drive it.

| Topic | Source | Why an agent wants it |
|---|---|---|
| `cli` | `docs/cli.md` | every subcommand, flag, exit code and JSON schema |
| `options` | `docs/options.md` | the generated option catalog with the symptom index — bad output → the flip that fixes it |
| `agents` | `docs/agents.md` | the repo rulebook and the doc map |
| `phases` | `docs/phases.md` | the P0–P9 model, for reasoning about *which* decision to flip |
| `modes` | `docs/modes.md` | the `--mode` presets and the size thresholds `auto` selects on |

`docs/options.md` is generated (`kuna catalog --markdown > docs/options.md`) and dominates
the embedded bytes at ~281 KB. A test asserts the embedded copy is byte-identical to the file
on disk, so a rebuild cannot ship a stale catalog — the same hazard
`kuna-decomp/tests/options_md_fresh.rs` guards for the file itself.

Exit codes: `0` ok, `2` unknown topic (the message lists the valid ones).

## `kuna catalog` — option discovery (the LLM control API)

```bash
kuna catalog --json              # the flippable assertion list, for an agent
kuna catalog --markdown          # regenerate docs/options.md
kuna catalog --check             # fail on catalog/registration drift (CI)
kuna catalog --tier transform    # filter to the transform-tier control surface
```

Parses the decompiler's `phase catalog` JSON (single source of truth: `settableTable`,
generated from `decompiler/crates/kuna-decomp/phases.toml`) into the documented, flippable
assertion list. `--markdown` output is tier-grouped and symptom-indexed; `--check`
cross-checks the catalog against `kuna_decomp::options::KUNA_OPTION_NAMES` in-process.
The rendered catalog is `docs/options.md`; the model behind it is `docs/phases.md` /
`docs/spec/`; the defaults are recorded in `docs/history.md`.

## `kuna specs` — the SLEIGH compiler

```bash
kuna specs -a specs/             # compile every .slaspec under a dir (slacomp's -a mode)
kuna specs <file.slaspec>        # compile one
```

A thin alias for `slacomp` (same CLI as upstream's `sleigh_opt`).

## `--slice ARCH` — which arch of a universal Mach-O

A Mach-O universal ("fat") binary is several thin images in one file, and every
kuna surface works on exactly one of them. With no override the pick is
deterministic: x86-64, else arm64, else the first arch present. `--slice ARCH`
names another (`x86_64`, `arm64`, `arm64e`, `i386`, `arm`, `ppc`, `ppc64`, …); a
`--target` SLEIGH id steers it too, by its leading arch stem. A slice the file
does not carry falls back to the default rather than failing.

```
$ kuna functions ./CrackMe --json | head -3          # the default slice
$ kuna strings ./CrackMe --json --slice i386         # the other one
```

It applies to every surface that reads the image — `decompile`, `functions`,
`decompile-all`, `strings`, `xrefs`, `disassemble`, `decompile-graph`,
`decompile-project` — so an inventory, its `--summary`, and a string scan of the
same file all describe the same slice.

## Everything else

`kuna modes` (list the option presets) and `kuna fid` (function identification) also
exist, plus minor flags not covered here (`--no-vars`, `--raw`, `--regions`, `--timeout`,
…) — ask the command itself with `kuna <subcommand> --help`.
