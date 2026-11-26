# 10 — Deobfuscation

```yaml
Anchors:
  - decompiler/crates/kuna-decomp/src/deobf
```

Deobfuscation is a plane of the decompiler, not a phase of it. Obfuscation is
undone at the stage whose representation can undo it: a protector that replaces
a function's code (a VM, a packer, a dispatcher loop) has to be undone before
anything is lifted, because every later stage takes the flow graph for the
program's, while an obfuscation that only respells values (mixed
boolean-arithmetic, an opaque predicate) is cheapest to undo on SSA, where
constants are already folded. The plane is therefore a set of deobfuscators,
each acting at the stage hooks it needs, and one contract that every stage
after the flow reads. Its code lives in one folder whatever stage it acts at.

## 10.1 The plane

**Stage hooks.** The pipeline calls the plane at fixed points
(`decompiler/crates/kuna-decomp/src/deobf/kuna_deobf.rs`), and a deobfuscator
(`Deobfuscator`) implements the ones it needs:

| Hook | Called from | What a deobfuscator does there |
|---|---|---|
| `front` | `decompile_drive.rs (generate_flow_once)`, before the `FlowInfo` is built | recovers a body: a synthetic p-code program lifted in place of the function's bytes |
| `after_flow` | `decompile_drive.rs (generate_flow)`, once the plain flow is built | the same, when its detection has to read the function's own ops; the flow is then followed again from the body |
| rule pool | `universalaction.rs (universal_sched)` | contributes rules, each gated by the body's policy |
| `audit` | `decompile_drive.rs`, once the function is decompiled | writes header warnings for what the result still holds |

The front asks the selected deobfuscators of `DEOBFUSCATORS` in order and
takes the first body one recovers. A function no deobfuscator claims is lifted from its
bytes and pays only their detection. `after_flow` is skipped for a function
that already has a recovered body, and for a front-end whose translator is not
SLEIGH (Ghidra's), which cannot serve an overlay.

**Selecting techniques.** One option names what runs: `option deobfuscate
auto|off|<technique>[,<technique>...]`, default `auto`, which the CLI also
spells `--deobfuscate LIST` (`decompiler/crates/kuna-cli/src/args.rs (lower_deobfuscate)`
rewrites the flag into the option before any subcommand reads its arguments).
A technique is named for the obfuscation it undoes, not for the algorithm that
undoes it; the names are those of the registry (`Deobfuscator::name`), so a
new row in `DEOBFUSCATORS` is a new value. The value parses into a
`kuna_deobf.rs (Selection)`:

| Value | Which techniques act | How each is engaged |
|---|---|---|
| `auto` | all | `Engage::Detect`: on the functions its own detection finds |
| `off` | none | the protected code is decompiled as it is |
| `a,b` | the named ones alone | `Engage::Force`: on the function it is handed, without waiting for its detection |

The plane reads the option, and a technique does not: each hook is passed the
`Engage` it was selected with, so what "detect" and "force" mean is the only
thing a technique decides. A name the registry does not hold is an error that
lists the ones it does, at the CLI before anything is loaded and in the
engine's option dispatch. Setting the option clears the body cache. One
technique exists so far, `devirtualize` (§10.2).

**The recovered body.** A body (`decompiler/crates/kuna-decomp/src/deobf/kuna_body.rs (RecoveredBody)`)
is the overlay the flow follower lifts
(`decompiler/crates/kuna-sleigh/src/kuna_pcodeoverlay.rs`: synthetic
instructions at addresses no section maps, served by the translator before it
decodes any bytes), the string literals the program refers to, the lines the
function's header carries, and the deobfuscator's own record, which only its
audit reads. Adopting a body installs the overlay, plants the literals as
typelocked `char[N]` symbols, clears the function's declared extent, and hands
the function the body. Bodies are cached by function entry
(`Architecture::deobf_bodies`), since a restart follows the flow again; an
option that changes what a deobfuscator recovers clears the cache.

**The contract.** A recovered body is not compiler output, and the stages
after the flow have to know where it differs. They read that from
`Funcdata::kuna_body`, and never ask which deobfuscator produced the body. A
function lifted from its own bytes reads `NATIVE`: no facts, every policy off.
The contract has two parts.

Facts (`BodyFacts`) are true of the program:

- `call_kills`, `call_preserves`, `call_floor`: what every external call does
  to storage the prototype model cannot speak for. `Heritage::guardCalls` asks
  `kuna_body.rs (call_effect)` only where the model reports an unknown effect.
- `escapes`: the entry-stack offsets whose address a call receives. The
  local-alias mapping counts each as a pointer into the frame
  (`kuna_body.rs (frame_escaped)`, `kuna_body.rs (holds_escape)`).
- `incidental_from`, `incidental_to`: register ranges whose COPYs move no
  value of the program's (`kuna_body.rs (mark_incidental_copies)`).

Policy (`BodyPolicy`) says how a body of its kind is analysed. Each flag is
read by one stage:

| Flag | Stage | Effect |
|---|---|---|
| `guard_stack_stores` | heritage | every STORE that may reach a stack range guards it |
| `join_stack_inputs` | heritage | a stack argument slot split into halves is passed whole |
| `direct_spills` | heritage | a LOAD guard through a spilled pointer stops only at a directly spilled slot |
| `inputs_by_dataflow` | call prototypes | argument trials skip the local-alias test |
| `restructure_after_deadcode` | structuring | the structure is rebuilt once dead code may be removed in every space |
| `residue_rules` | rule pool | the arithmetic-residue rules apply |

A deobfuscator states the policy of the bodies it recovers; the reasons for
the VM devirtualizer's are in §10.2. A stage that needs to treat recovered
bodies differently adds a fact or a policy flag here, not a test for a
deobfuscator.

**The symbolic evaluation kit.** Three modules of the plane depend on nothing
else in the decompiler and are meant to be shared by its deobfuscators: the
expression arena (`decompiler/crates/kuna-decomp/src/deobf/kuna_symexpr.rs`),
the abstract machine state (`decompiler/crates/kuna-decomp/src/deobf/kuna_symstate.rs`)
and the p-code emulator over them (`decompiler/crates/kuna-decomp/src/deobf/kuna_symemu.rs`).
They read the image and lift instructions through two traits (`SymImage`,
`SymTarget`), which a deobfuscator implements over the architecture. §10.2
describes them where the VM devirtualizer uses them.

**Adding a deobfuscator.** A module in the plane's folder and a row in
`DEOBFUSCATORS`, which makes its name a value of `option deobfuscate`. One that recovers bodies states their facts
and policy; one that only respells values registers its rules and needs no
body at all. A stage the plane has no hook at yet gets one the same way: a
method on `Deobfuscator` with an empty default, and one call from that stage.

## 10.2 VM devirtualization (`devirtualize`): the Pushan port

**(angr) technique `devirtualize` of `option deobfuscate`, on under the
default `auto`,
`decompiler/crates/kuna-decomp/src/deobf/kuna_devirtualize.rs (Devirtualize)`.**

**The algorithm is Pushan's.** This technique is a port onto p-code of Pushan:
Sudhir, Basque, Gibbs et al., "Pushan: Trace-Free Deobfuscation of
Virtualization-Obfuscated Binaries", NDSS 2027, and of its reference
implementation, angr PR #7680 (`VMDeobfuscation` / `CFGVMDeobfuscation`). The
option and the code are named for what the technique does, but the method is
the paper's: symbolic emulation of the interpreter with no path constraints,
blocks keyed by native address and virtual program counter, symbolization of
the locations that differ where paths meet until the graph is stable, and a
flat graph handed to an ordinary decompiler. "The paper" below always means
that paper, and its stage numbers are the ones used here. What kuna adds is
the carrier: p-code, an evaluation-based identity test where the original
asks a solver, and emission as a synthetic overlay.

A function protected by a VM-based obfuscator (VMProtect, Themida) keeps only a
stub at its original entry: it pushes some context and jumps into an
interpreter in a section the protector added, which runs the function's
bytecode. Without the technique the flow follower lifts that stub and stops at the
interpreter's computed dispatch, so the function decompiles to an empty body or
to the interpreter's dispatch loop. With it, the flow follower is handed a
different program: the protected function's own control flow, recovered by
emulating the interpreter symbolically and emitted as synthetic p-code. Every
later phase runs unchanged on that program, and it does the cleanup that the
paper's stages 2 and 3 do (SSA, dead code, stack variables, types, structuring).
x86 and x86-64 are supported; any other target reports a header warning and
keeps the plain flow.

**Detection.** Under `deobfuscate auto` the technique acts on a function only when the image has a
candidate interpreter section other than the one holding the entry, and the
function's direct native flow reaches into such a section. A candidate is a
loaded executable section of at least 0x10000 bytes that the toolchain does
not emit for every program (`kuna_devirtualize.rs (toolchain_code_section)`): not
an ELF `.plt*`, `.iplt`, `.init` or `.fini`, not a section whose name contains
`text` in either case (`.text.unlikely`, the kernel's `.init.text`, Mach-O
`__text`, PE `.textbss`), not Mach-O `__stubs`, `__stub_helper` or
`__auth_stubs`, and not a Windows driver's `PAGE*` or `INIT`. A large library's PLT (PyQt5's QtWidgets has an
89 KiB `.plt`) and a kernel module's cold split (the `xe` driver's 69 KiB
`.text.unlikely`) would otherwise match every function that calls through the
PLT or jumps to its cold part. The names come from the loader
(`LoadImage::kuna_section_names`); a section without one is a candidate on
its size. An image with no candidate left returns before any instruction is
lifted, so those binaries pay nothing. `kuna_devirtualize.rs (detect_vm_entry)`
walks fall-throughs, direct jumps and both sides of conditionals from the
entry, without following calls, for up to 4096 instructions. The first branch
or call whose target lies in a candidate section other than the entry's is the
VM entry, unless the target is an import thunk, code whose first instruction
after an optional `endbr` is an indirect jump (`import_thunk`), or the
transfer is a call to a known function entry: the exploration summarizes such
a call instead of entering it, so it could never reach an interpreter that way
and a program with an unrecognised second code section of its own functions
is not explored for calling them. That covers
both the stub a protector leaves at a whole-function entry and an SDK-marked
region's `push imm; call vm_entry`. The sections come from the loader
(`ArchTarget::new`, falling back to segments), so a raw image or an XML
`<binaryimage>` reports none, and detection never fires on it. An interpreter
that is the function's own body (Tigress `Virtualize`: a `switch` or a
threaded `goto *pc` over bytecode in the image, no protector section) has no
such entry. Detection looks for one only after the plain flow has been followed,
and only in a function whose plain flow has an interpreter's shape
(`kuna_devirtualize.rs (interpreter_shape)`). Either a computed jump lies in a cycle
of the flow graph, as a switch dispatch does inside its loop, or the function
copies memory (a call to a `memcpy`-like routine, or a `rep movs`) and has a
jump the flow could not recover and treats as a call (a CALLIND with an
artificial RETURN at its address), as threaded code over a copied array
shows. A tail call through a pointer, or a `switch` outside a loop, is not a
candidate. Every other function pays one scan of its ops, and a front-end
whose translator is not SLEIGH (Ghidra's, which cannot serve the overlay)
pays nothing. Threaded code over bytecode that is not copied (its array a
global) is therefore found only when the technique is named (`deobfuscate
devirtualize`). For a candidate,
`kuna_devirtualize.rs (interpreter_after_flow)` runs the interpreter probe described
below. When the probe finds a dispatch and the exploration decides at least one
computed jump of the interpreter, the plane caches the recovered body and the
flow is followed again from it (`kuna_deobf.rs (after_flow)`, §10.1). An ordinary `switch`
stops the probe at its first computed jump, since no fetch through a pointer
cell precedes it. Naming the technique skips the detection and devirtualizes the requested
function, probing for an in-function interpreter first when the function does
not enter a protector section. Either way, an exploration that never reaches a block inside
the interpreter (`vm_nodes == 0`) is discarded and the plain flow is used, so a
false detection costs one exploration and changes nothing. `deobfuscate off` always uses
the plain flow. A successful result is cached per entry on the architecture
(`Architecture::deobf_bodies`, the plane's cache), so re-decompiling the same function does not explore it again.

**Exploration keys.** The flat graph is built by
`decompiler/crates/kuna-decomp/src/deobf/kuna_devirtflow.rs (Explorer)`. A
node is a native address plus the virtual program counter (VPC) the
interpreter holds when it gets there, plus the stack pointer's offset from its
entry value (`Explorer::at_depth`). As a result, every bytecode
instruction's run through its handler is a separate block, and the graph is
the protected program's control flow laid out flat through the interpreter.
The stack depth is part of the key because two paths can reach the same
bytecode at different depths and neither state can stand for the other.
VMProtect places its register context below the stack that the last re-entry
from a call found. After a call that left one more argument word on the
stack, the same loop head runs with its context 4 bytes lower, while the
virtual stack pointer and every program variable are where they were. angr
merges only states at the same stack pointer, too. Up to 4 depths are kept
apart per address and VPC. A further depth is reached at the first one and is
not merged, so a path that grows the stack on every iteration cannot unroll a
loop forever.
Where the VPC lives in a register rather than a memory cell, that register is
part of the key as well (`NodeKey::holder`). VMProtect 3 chooses the VPC
register and the direction it reads bytecode in per virtual block, so two
blocks can end at the same bytecode address from opposite sides, one reading
upward through R8 and one downward through R9, and both reach the shared
virtual-jump handler with the same VPC value. They are different instructions
that jump to different blocks. Keyed alike, the second path would merge into
the first, the first path's VPC register would be symbolized there, and the
next fetch would read bytecode at a symbolic address and leave the dispatch
unresolved (loop, SDL, libpng and the Ultra zlib under VMProtect). angr keeps
them apart because it re-keys a block by the VPC it holds at the block's end
(`updated_block_id_map` in `cfg_vm_deobfuscation.py`), by which point the
virtual jump has fetched from its own target. All paths into one instruction
of one virtual block hold the VPC in the same register, so the extra key part
separates no paths that the VM itself joins.
The VPC is found as angr's `find_vip` finds it (`Explorer::successor_ctx`),
from the state at the end of each block. A VPC value must point into a
section of the image that is not writable (angr excludes only the data
sections; VMProtect may place bytecode in `.pdata`), at bytes no path has
written, inside a 256-byte window whose byte entropy is at least 5.0 bits (the paper's bytecode test,
`bytecode_near`). The window is the one angr's `find_vip` reads: 128 bytes on
each side of the value, moved to start at the value or to end at it where it
would cross the section's start or end. A window aligned to 256 bytes fails at
the tail of a bytecode region that zero padding follows: zlib's last virtual
block under VMProtect ends 0x80 bytes before the zeros of `.pdata`, where an
aligned window measures 4.35 bits and a centred one 5.9, so the switch to
that block's VPC register went unseen and the walk kept a handler pointer as
its VPC. Whether a jump target is compiled code or the protector's
(`Explorer::jump_to`) is still decided on the aligned block (`bytecode_like`).
The value must be a constant or one of at most four possible values. A value that fails the test (a register that held the
bytecode pointer earlier in the block and now holds a scratch number, the
interpreter's writable context area, an unpacked handler table) never keys a
node.

Themida keeps the VPC in a memory cell of its VM context and copies it into a
different register in each handler. A concrete memory cell is therefore
checked first. When the block loaded a pointer from a written cell and
fetched bytecode within 16 bytes of that pointer, the cell is the VPC. It
stays the VPC while it holds unwritten image memory, even where a
handler's bytecode sits in a low-entropy stretch. Otherwise the VPC is a
general-purpose register the block dereferenced, with a displacement of at
most 16, into bytecode, and whose own value passes the test. The previous
VPC register wins while it qualifies. Next comes the candidate nearest
(within 0x100 bytes) to the previous VPC, then the register fetched through
most often. A block with no qualifying register follows the previous VPC
register's value while it stays unwritten image memory, and otherwise
carries the last VPC forward. Before the interpreter is entered, a register
that a block of the protector's code advanced just past a concrete memory
access (Themida's relocation of its handler table) keys the walk, so the walk
unrolls with constant pointers instead of merging into a loop over symbols. A
loop of the function's own code (a `rep movsb` copying a literal before the
protected region starts) is not keyed that way and stays a loop. Outside the
interpreter (a re-entry stub, an external call's return), the context sticks
to the last VPC, salted with the return address that a dispatch call pushed,
so two re-entry stubs never share the entry handler's node.

The interpreter handing control back to the function's own code ends that
(`Explorer::leaves_vm`, angr's `_vm_mode_should_end`). A computed jump or
return out of a block in the interpreter's context, with no return address
pushed, to code outside the protector's sections that is not a fragment of
its handlers, is an exit once the VPC register no longer holds a virtual PC:
the exit restored the native registers. A VPC kept in a memory cell keeps
its last value after the exit, so there the stack pointer must instead be
back at the depth where native code first jumped into the protector (angr's
`_on_vm_entry_frame`). The exit's target is keyed with no VPC, as angr keys
the blocks after an exit, so every exit to one address reaches one node.
What the path stored on the stack below the stack pointer, past the x86-64
red zone, is dropped there (`Explorer::forget_below_sp`): it is the
interpreter's context, dead to the program, and paths that left by different
exits would otherwise differ there at the node they join. A
function whose virtual branches each leave for the shared epilogue, its
`/GS` cookie check included, then runs that epilogue once in the recovered
program, as it does in the binary, instead of once per virtual exit. A jump
from there back into the protector is a re-entry: it is keyed by its site,
so it never shares the first entry's nodes (angr's re-entry region tag).

A node of the function's own code outside the interpreter ends where
another such node starts (`Explorer::emulate_node`), as a disassembler ends
a basic block at a jump target. Exits often land a few instructions apart
in one straight run (one at an SDK marker's leftover `or ebx, ebx` or at a
register restore, another just past it); without the split, each path would
run its own copy of the rest of the run, and the paths would never join.
The starts are kept over every round. A start found inside a run that was
already emulated forces another round, in which that run ends there.

**In-function interpreters.** A Tigress-virtualized function is its own
interpreter: its prologue points a local at the bytecode, and a loop fetches an
opcode through that local and dispatches on it, through a jump table (`switch`
dispatch) or through the handler address the bytecode holds (`direct`
dispatch, threaded code). The VPC is that local, a stack slot, and the
bytecode sits in writable `.data` with low entropy, so none of the protector
rules above applies. `Explorer::probe_interpreter` finds it before the first
round. It is a concrete run from the entry of at most 4096 instructions. Every
byte of file data (a loaded section that is neither code nor uninitialized)
reads as the file holds it, writable or not, and an undecided branch is followed
on its fall-through side. The run records each pointer-sized load from a memory
cell (a slot of the entry stack, or a concrete address) whose value points into
file data or into a copy of file data on the stack (see the copy model below).
A later load within 16 bytes past such a pointer is a fetch through the cell.
A computed jump to code that a fetch preceded is a dispatch event. A computed
jump with no fetch before it (an ordinary `switch`), a return, an
instruction run 64 times (gcc's stack probe for a 256 KiB frame runs 64
times), or 512 instructions without a bytecode pointer loaded from a cell end
the probe. The cell is the VPC once one computed
jump has seen fetches through it at 4 different places, and gone to at least two
targets. The explorer then keeps that cell as the VPC for the whole
exploration (`Explorer::data_vpc`). At the end of every block, wherever the
cell holds a bytecode pointer, the block's successors are in the interpreter,
keyed by that pointer. A pointer into file data keys by its address. A pointer
into a stack copy keys by its tagged stack offset (`Explorer::data_vpc_key`).
Elsewhere (the prologue before the cell is set) the context carries over. A
computed jump in the interpreter's context is its dispatch, never a call
through a pushed return address (`Explorer::jump_to`).

A fetch through the VPC reads the file's bytes even in a writable section
(`kuna_symemu.rs (Emulator::through_vpc)`). The rule is narrow: the load
must lie within 16 bytes of the pointer the VPC cell holds when the load runs,
and that pointer must point into file data or a copy of it. Every other read of
writable memory, the virtualized program's own globals included, stays the
symbol of its entry value, as above. A byte the path wrote reads what it wrote
(self-modifying bytecode is not read from the file). A symbol an earlier
ordinary read cached for the same byte is dropped (`State::read_mem_file`). The
emitter folds such a fetch to its constant, which is what removes the
bytecode from the output. Because the image value of writable memory is not a
fact, the header warning of a function whose fetches read writable memory says
that the recovered body assumes its bytecode in writable memory is what the
file holds. A function whose bytecode is in read-only memory reads it from
the image anyway and gets no such clause.

The copy model. Tigress's `direct` dispatch keeps its threaded code in a local
array, which gcc fills from an initializer in `.data` with `memcpy` (81 KB in
one sample), and the entries are handler addresses fixed by
`R_X86_64_RELATIVE` relocations whose addends the linker also stores in the
file. A call to a routine with `memcpy`'s contract, named `memcpy`, `memmove`,
`__memcpy_chk` or `__memmove_chk` (`ArchTarget::copy_routine`), that copies a
constant count of at most 16 MiB out of file data to the entry stack or a
concrete address, is modelled as a copy (`Explorer::copy_arguments`,
`State::copy_in`). The destination's stored bytes are replaced: what the path had
written to the source is copied, and the rest of the destination reads the
source lazily through one `CopyRegion` record, as the source read at the time
of the call. A large copy therefore costs one record per state, not one cell
per byte. The copy is forgotten wherever its destination is
(`State::forget_copies`): a store over a symbolic index, the frame below the
stack pointer after a VM exit, an external call for a concrete destination
in mutable memory, and a node that paths reach with different copies, which
starts without them. Where two sides of a merged conditional disagree on a
copy, the merged state keeps only the copies both sides have. The rest of the
call is summarized as usual and stays in the program. gcc copies a smaller
array inline with `rep movsq`. In the probe and in the exploration of an
in-function interpreter, a `rep movs` of the function's own code with a
constant count, the direction flag clear, a source in file data and a
destination on the entry stack or at a concrete address is the same copy
(`Explorer::rep_copy`): the count register is cleared and both pointers
advance past it, in place of one emulated iteration per element. When the last
round read the copy only through VPC fetches that fold to constants, and no
pointer into it reached a call that may return (a register left pointing into
the copy at `__stack_chk_fail` does not count), the emitter leaves the instruction out, since
no op of the recovered program reads what it wrote. Otherwise the function is explored again with every `rep movs`
run as an instruction, and the copied array stays a loop of the program
(`kuna_symemu.rs (Emulator::copy_read_unfolded)`).

The direct dispatch also starts with gcc's stack probe for its large frame
(`sub rsp, 0x1000; or [rsp], 0; cmp rsp, r11; jne`). A decided jump of the
function's own code back to an earlier instruction of the node, after which
the stack pointer sits at another constant depth than it did there, repeats
inside the node (`Explorer::moves_stack_back_to`), as a `rep` instruction in
the protector does. Keying each iteration at its own depth would exhaust the
four depths kept apart per address and join the rest unmerged.

Interleaved programs. Tigress can run bogus functions in the same interpreter,
round-robin with the real one: each has its own VPC and virtual stack
(`pc[select]`, `sp[select]`), and a selector register advances after every
instruction. Keying by every VPC would build the product of the programs'
graphs, and keying by the running program's VPC alone leaves the other VPC a
merged symbol whose dispatch no enumeration decides. The probe therefore
reports every cell fetched through at the dispatch. Cells a fixed stride apart
are the VPCs of interleaved programs. The selector is every register, or
4-byte slot of the entry stack, that holds the index of the fetched cell
where the straight run that loads the cell starts (the loop head before the
dispatch, which may branch further through a second table), on every turn the
probe saw there (`Explorer::infer_selector`). The explorer pins those locations to the
index of the program dispatched first, there and in every state reaching that
address (`Explorer::apply_pin`), as Pushan's Tigress driver pins the selector
register. Every turn then runs the first program, the key holds its VPC alone,
and the recovered graph is that program's own. This assumes the other
programs neither end the function first nor touch the first program's data.
The header warning says that the body follows the first of the interleaved
programs. Without a selector to pin, the other programs' dispatches stay
unresolved and the header reports the exploration as incomplete.

An instruction that branches back to itself (an x86 `rep` prefix) is handled
by where it lives (`Explorer::emulate_node`). In the protector's code, with a
decided count, it repeats inside its node, one instance per iteration, as
angr's rep-movsb shortcut does. In the function's own code it starts a node of
its own and re-reaches that node, so its counter and pointers are symbolized
and it stays the program's loop. Its guard on the remaining count is a
conditional branch out of the instruction, ahead of the copy, and the
fall-through side then jumps back to the instruction. The emulator reports
that as a conditional exit whose fall-through continues at the instruction
itself (`Emulator::run_from`), not as a select between the two targets, so
the emitted node keeps the branch where the instruction tests it, before the
store, and a count of zero copies nothing.

**The emulator and the expression arena.** Each node is emulated one native
instruction at a time over its SLEIGH p-code
(`decompiler/crates/kuna-decomp/src/deobf/kuna_symemu.rs (Emulator)`).
The state (`decompiler/crates/kuna-decomp/src/deobf/kuna_symstate.rs
(State)`) is byte-granular: registers, temporaries and memory hold
(expression, byte index) cells, so overlapping x86 sub-registers and
mixed-width stack accesses compose exactly. Memory is split into regions by
the address's symbolic base. Distinct symbolic bases never alias, which is the
paper's stack assumption. Unwritten read-only memory reads the image, and
unwritten symbolic memory and registers read a stable symbol, the same one on
every path. Unwritten memory in a writable data section is not the file's
either (`SymImage::is_mutable`): its image contents are only the value it
starts the process with, and another function may have changed it, so it reads
the stable symbol of its entry value, and the MSVC `/GS` cookie, a global flag
tested by the program, a counter or a pointer table read there stays a
variable instead of the number the file happens to hold. The one exception is
a fetch through an in-function interpreter's VPC, above. An import slot (a
function symbol at the address, as in a Themida image's writable `.idata`)
still reads the image, because a call through it is recognized by the pointer
the slot holds, and the protector's own sections are code and read the image
too. An external call may write any mutable memory, so it drops what the path
stored there, and later reads see a symbol of that call's output, one per
emulated call (`State::clobber_mutable`): a value read before the call and
the same global read after it are not the same variable. A store whose
address no split places, over a symbolic index (`base + c*i + k`, an array
element), forgets what it may have overwritten (`Emulator::havoc_store`): the
regions of the address's bases, and on the entry stack only the function's own
frame from `k` upward, the index taken as non-negative when every index term
scales up, below `k` too otherwise. An address with no base is an array in the
image, and all mutable memory is forgotten. A store whose address depends on
the entry stack pointer or on another variable whose region the state holds,
without that variable being one of the bases forgotten (the frame pointer
under a merge symbol's byte, `(fp + k) ^ (x & 8)`), is lost
(`Emulator::reaches_kept`, `AccessRecord::lost`): the exploration goes on
reading what that memory held before it, and the header reports it once the
body is decompiled (below). A constant fill from the image is not stored in the state,
because it reads the same on every path, so a node's entry state holds only
what the program wrote and not every bytecode byte its path fetched. A
symbolic fill is stored, so reads of the same bytes at different widths
agree. Each store keeps its cells in 16-byte pages shared copy-on-write
(`kuna_symstate.rs (ByteStore)`). A node's entry state is a copy of the
state its first path arrived with, so it shares every page that path's
predecessor did not write, and a write copies only the page it lands in. A
merge compares two states page by page and skips a page they share. A
Themida node's entry state holds about 940 cells, most of them the VM context
and the stack every handler leaves alone. On cairuh_themida (32-bit Themida)
the process held 2,261 MiB at the end of the first round (22,686 nodes) with
one map entry per byte, and 1,346 MiB with the pages, of which the 103,578
distinct pages of all 22,686 entry states are 19 MiB.
Values live in a hash-consed DAG
(`decompiler/crates/kuna-decomp/src/deobf/kuna_symexpr.rs (ExprArena)`)
that normalizes every node as it is built. Constants fold. Every
constructor remembers its result by its operands (`ExprArena::memo`; a
division by zero, which makes a fresh unknown, is not remembered). The rules
recurse into the arms of selects and into the terms of sums, and Themida's
handlers build selects and masked sums over the same subterms many levels
deep, so without the memo one 18-instruction handler on the plain Themida loop
sample made 569 million constructor calls (269 s) while adding 88 nodes; with
it, the whole exploration takes 3 s. The memo starts over after 2^21 entries.
ADD/SUB/negate/multiply-by-constant chains collapse to one linear normal form
over their non-linear atoms, so the interpreter's key arithmetic and
obfuscated stack offsets cancel structurally. A known-bits lattice prunes masks,
pushes them through shifts, and distributes truncations and zero-extensions
over disjoint sums, which is how a `pushfq` flags word or a De Morgan mask
over it stays decidable. A comparison of two offsets from the same pointer
base is decided by the offsets. A sum holding both a zero-extended flag and the
zero-extended negated flag rewrites the second (`zext(b ^ 1) == 1 - zext(b)`),
so a `pushf` word of a result and the word of its complement, which Themida
adds up into a context offset, cancel. Rewriting every negated flag that way
on its own would hide the power-of-two terms the mask rules over flags words
look for (VMProtect's `popf` words stopped folding). Mixed
boolean-arithmetic rewrites of the stack pointer that the normal form does not
see are checked by a probe (`ExprArena::affine_probe`). An expression over one
variable is evaluated with it set to three values that differ in the
variable's unknown bits. If it moves by the same amount as the variable each
time, it is `var + d`, and a stack-pointer write is stored in that form. If it
gives the same value each time and the variable is a pointer, it is that
constant. A variable with at most
two unknown bits (a merged loop counter seen only as 0 and 1 has one) is
evaluated at each of its possible values instead. Three probe points masked by
its known bits would coincide, and a select between two bytecode addresses
over that counter would pass for `counter + 0x47444c`, sending the next
bytecode fetch into a region that does not exist. Without boolean atoms the
three-point answer stands only if the identity test below agrees, and an
expression over several variables goes to the identity test alone. So does
an expression over one variable that is no pointer when the three points give
it no answer, and only a constant answer is taken from that test. Themida
decodes a context offset `k` through a key, `x ^ ((x & ~k) + (~x & k))` with
`x` the key plus a constant, which is `k` whatever the key holds. In
newstar_infect_themida the key is a loop's merge symbol, so the offset is a
one-variable expression the three points always reject (they never accept a
constant over a non-pointer), and the load of the guest's frame register
from its context cell went to an unknown address. The emitter then kept it as
a load of the cell's memory, which the decompiler's own rules placed on the
cell, while every store to the cell had been privatized into a register: the
loop read the cell's file contents, and `CopyFileW` got that value minus
0x32c as its destination instead of the frame buffer. just_if_themida lost
the store of `&input` into the VM register that becomes RDX the same way, so
the emulated call passed the caller's RDX to `scanf`.

**Identity by evaluation.** Themida computes every stack and context address
from the entry stack pointer and other entry registers through identities such
as `(y & M) | (N - (~y & N)) == y` (with `N == ~M`) and
`(x | K) - (x & ~K) == K`, with flags words of junk arithmetic mixed in and
cancelled again. Such an address has several variables and is not linear, so
the normal form sees nothing and every load and store through it went to an
unknown location, the exit's return target with them. `ExprArena::identity_probe`
decides whether `e == base + k` for one of `e`'s variables, or `e == k`, by
evaluating the DAG at 96 points that assign every input at once (one pass over
the nodes in id order, operands before users; `eval_samples`). The inputs are
the variables (their known bits kept, the rest drawn) and every boolean atom
the value reads (`ExprArena::bool_atoms`: comparisons and flags, one over
other comparisons included), each drawn as a free bit of its own instead of
computed from its operands. The
identity must then hold whatever the comparisons say, which asks for more than
the program needs but never less, and a guard on program data that holds at
one value only (`x == 5`) cannot hide behind samples that never hit that value.
20 of the points are structured: for every 8, 16, 32 and 64-bit lane of each
input, and for the whole value, all zeros, all ones, `0111..1` and `1000..0`
below random upper bits. The other 76 draw every open bit uniformly. The points
are fixed by the input's id and the point's index through splitmix64 under a
fixed seed, so two runs decide alike. When the inputs have at most 8 open bits
in total, every assignment is evaluated and the answer is exact. Otherwise a
wrong `yes` needs the difference `g = e - base - k` to vanish at all 96 points.
If `g` is nonzero on a fraction `d` of the inputs, the 76 uniform points all
miss with probability `(1 - d)^76`: below 2^-76 for `d = 1/2` (any difference
in one bit position, a parity, one flag), 4 * 10^-5 for a conjunction of three
independent flags, 0.7% for four. The structured points catch the small sets
obfuscated arithmetic does produce: a constant offset that carries or borrows
across a lane or the sign bit, such as a 64-bit pointer advanced in its low
half only, which uniform draws find with probability 2^-28 and the lane's
all-ones point always. What no finite test excludes is a value that differs at
a few inputs far from any edge, an arithmetic encoding of `x == c` for a magic
`c` or a conjunction of many independent conditions. A concrete run under a
preconstrained stack pointer (the original's) is exact for its one stack value
and says nothing about the others either. The expression's values at 4 further
points, with atoms computed from their operands, are cached per node
(`ExprArena::quick_vals`, at most 2^22 entries). The full test walks at most
4096 distinct nodes; a larger expression is no identity to it (the emitter
probes every op of an exploration that can reach 24,000 nodes, and at 2^16
nodes hunatcha_themida spent its whole 600 s budget there). A value the quick points already rule
out (neither constant nor any variable of its width plus a constant, found
through a map from each variable's differences) costs no walk of its
expression, so the probe on every XOR into a register stays cheap. An
identity found this way is used where the normal form's would be: load and
store addresses, stack-pointer writes, `frame + k` rematerialization in the
emitter. The emitter's constant folding stays with constants the normal form
found: folding an op whose value is constant only by this test (junk woven
into a real value) cleans Themida output, but it also rewrote 9 of 30 Tigress
samples' outputs in ways not verified, so it is not done. A stack-pointer write that is no variable plus a
constant by any test leaves every later push and pop without a place, and the
header counts it: "N stack pointer writes the emulation could not place".
When paths meet, two values of a location are the same when their
difference is the constant 0 by this test (`Explorer::same_on_stack`), so a
frame pointer that each path rebuilt through a different identity is not
symbolized. A guard, a load or store address and a computed target first have
each atom an identity decides replaced by its outcome (`ExprArena::decide_atoms`):
`x == y` when `x - y` is a constant, and an ordered comparison when both sides
are constants or one pointer plus constants. An opaque predicate built from
these identities then folds, and only through the identity of its operands: a
comparison is never decided by sampling the comparison itself.

**The platform's user-mode stack.** Some identities hold only for a stack
pointer where the platform puts one. A 32-bit Windows process without
`IMAGE_FILE_LARGE_ADDRESS_AWARE` in its file header has its whole user address
space below 2 GiB, and a 64-bit one below 128 TiB
(`SymTarget::user_stack_clear_bits`: bit 31, or bits 47 to 63). An image
whose PE subsystem is native (a driver) runs on a kernel stack, where neither
holds, so it gets no such bits, and neither does an image whose headers cannot
be read. When the
identity test fails over every stack pointer and the stack pointer is among
the variables, it is run once more with those bits clear (the quick points
already sample the stack pointer that way, so they are a necessary condition
of both runs). An answer found only then is counted
(`ExprArena::sp_bound_used`) in the exploration log. The header does not
mention it: for a user-mode image the bound is the operating system's
guarantee, not a guess about one run. No other target gets the second run. The entry stack pointer's low 16 bits stay known as
before (above), so every identity is one under that assumption as well.
Themida derives its image base by XOR-swapping it through the stack pointer,
so a register written by such an XOR is stored as the constant. The constant
reading is only taken over the entry stack pointer, whose value is a
user-mode address. The entry stack pointer carries its ABI
alignment as known bits. The entry values of the system flags (TF, IF, AC,
ID and the rest) and of DF are fixed to their user-mode values, so
anti-debugging checks fold.

There is no path condition and no SMT solver. The exploration asks two
questions: is this guard fixed, and which constants can this target take. Both
are answered by a case split over the boolean atoms of the expression
(`ExprArena::case_split`). The atoms are the comparisons and boolean variables
the value depends on. They are substituted innermost first, so a comparison
over flags that are computed from other flags folds under the inner
assignment. The innermost atoms are found through the operands of every atom
that holds one, so a guard that tests a materialized comparison (`setl`, then
`test` and `cmovz`, whose condition compares the overflow and sign atoms) is
decided by those two atoms. An atom whose flip never changes the result is dropped, and a case
that only fails through a shift or division made undefined by its own
assignment is infeasible. The split is bounded at 10 atoms per level, 4
nested levels and 4096 substitutions. A guard that folds takes one side. A
guard that stays symbolic keeps both, as a native conditional. A computed
target becomes a select over its enumerated destinations, each arm labelled
with the atom values that choose it, and up to 16 arms. A load or store
whose address selects between a few concrete locations becomes a select over
those locations, which is the paper's two-valued VIP/key handling.

**The symbolization fixpoint.** Exploration runs in rounds of at most 40
(`Explorer::run`). When a later path reaches an existing node, its incoming
state is compared with the node's recorded entry state
(`State::diff_constants`). Following angr's merge rule, every location whose
value differs is recorded against the node's key. A location is an aligned
8-byte slot. Two values are the same when they are one node or when the
identity test finds their difference to be the constant 0
(`kuna_symstate.rs (same_value)`). A slot that is symbolic on both sides
and differs in only one half is narrowed to that half, down to one byte, and
what is left is narrowed to the run from the first to the last byte the two
sides hold as different nodes, so a narrow variable sharing a slot with an
unchanged neighbour does not take the neighbour with it. Themida packs its
context cells at unaligned addresses, so one aligned slot can hold the top
byte of the interpreter's copy of the frame pointer, the same on both sides,
beside a cell the joining paths set differently. Symbolized whole, the slot
turned that byte into a merge symbol: no access through the copy could be
placed any more, and the stores among them were lost (zlib_themida's
`gz_read`, whose loop advances `buf` in its home slot through the copy, kept
the entry `buf` for `strm.next_out`). The halves are compared as nodes: splitting a slot around
a byte that is the same value through two different identities would keep the
top byte of Themida's frame pointer (saved in a context slot whose low bytes
hold junk that differs every loop iteration) through a loop merge, but costs
VMProtect samples symbolization rounds (blaster_ultra_vmp: 5 to 9, +70% time),
so it is not done; that Themida exit stays unresolved and says so. Each
recorded location keeps the constants it was seen to hold (up to 32) and the
bits all of them agreed on. A side that is not a constant contributes the
constants its atoms decide, nothing when it is the location's own symbol, and
otherwise opens the location: a loop counter that comes back as `i + 1` takes
values no merge saw, so its recorded constants no longer stand for all it
holds. A location under any symbolic base that names the same variable every
round (the entry stack pointer, a pointer argument or one loaded from the
caller's frame) is recorded like a register. Memory behind a base that is
fresh each round (a call's result, or a merge symbol, which a narrowing of
its known bits replaces) cannot be named again, so a node such memory reached with
different contents starts with all of it forgotten, and so does mutable
memory at a node that paths reached after different external calls. The VPC
register or cell is part of the key and is not symbolized while the incoming
path's VPC is the value the key names (`Explorer::keys_vpc`). A slot only partly
inside the cell is narrowed to its bytes outside it
(`Explorer::outside_vpc_cell`): Themida's branch handler sets a condition byte
right after the cell on one arm only, and dropping the whole slot with the
cell dropped the branch (curl's `if (p)` after `malloc`). A key carried
forward past a block that left no readable VPC symbolizes it like any other
location, so a path the key cannot tell apart is not dropped. The next round starts over. At a recorded node, each recorded
location becomes a fresh symbol that carries the agreed known bits. The fixpoint
is reached when a round records no new location, no new constant, no opened
location, no narrowed bits and no new split of a native run. The known bits only narrow, so each round's are
implied by every value a path brought, and at the fixpoint every guard folded
through them holds on every path. A pointer advanced in a loop (`sym + 1`
against `sym`) loses one more high bit to the carry every round, which would
cost one round per bit. The second time a location's agreed bits narrow,
every agreed bit at or above the lowest bit that was lost is dropped at once.
When the 40th round is reached without the fixpoint, it runs with every
recorded location fully unknown and opened (`Explorer::widen_all`), so a
location still widening folds nothing it cannot. A computed target that depends on exactly one symbolized
location whose recorded constants were never opened is enumerated over them instead of
atoms, by substituting each constant (`Explorer::enumerate_symbol_target`),
and each successor's state pins the symbol to its constant. A load whose
address depends on exactly one such location and lies at concrete
addresses for every recorded constant is read once per constant (up to 16)
and becomes a select over the reads, keyed by comparing the symbol with each
constant (`kuna_symemu.rs (Emulator::indexed_load)`). That is how a Themida
dispatch through a handler table, indexed by an opcode slot that a merge
symbolized, resolves. An array indexed by a loop counter, on the stack, behind
a pointer or in the image, is not read this way. The counter's location is
open, and a select over `v[0]` and `v[1]` would rebuild the program's
`v[i]` test as a test of those two elements; such a load is an unknown value,
and the test on it is witnessed where the program computes it. A loop counter or a virtual register that differs between iterations
therefore becomes a variable, while a value that is the same on every path
stays a constant and keeps its handler resolved.

**The virtual-PC fork.** An interpreter's conditional jump usually computes
both successors and selects one arithmetically (a `cmov`, a masked add), so at
the end of the handler the VPC register holds a value that depends on a guard,
not a constant. `Explorer::fork_on_vpc` splits the state at that point, as
angr's emulator does for a two-valued expression. For each of up to 4 VPC
values, the deciding atoms are pinned to the assignment that produces it in a
copy of the state. Each path then decodes its own bytecode with a concrete
context. The node's exit becomes an atom-labelled select over those
successors, and the node records which register or cell holds the VPC at its
exit and the VPC of each arm. When every assignment gives the same VPC,
there is no fork: that constant replaces the VPC expression throughout the
state, so the next handler fetches from a concrete VPC instead of carrying a
select whose value only a case split knows.
The deciding value may be a merge symbol rather than an atom: a virtual branch
on a flag that a join merged (VMProtect's hunatcha: a 0/1 stack slot recorded
at a join after an external call) leaves the VPC an `ite` over a byte of that
symbol. When the VPC does not reduce to constants under its atoms, it is split
over every recorded constant of each symbol it depends on whose location was
never opened, and then over the atoms that remain
(`kuna_symemu.rs (split_over_symbols)`, at most 16 combinations of
constants), as `Explorer::enumerate_symbol_target` does for one symbol in a
target. Each case's assignment includes the atom `sym == k`. In an arm where
such an atom holds on every assignment that reaches it, the symbol itself is
replaced by `k` in the arm's state. A load whose address splits this way into
concrete addresses is read once per address and becomes a select over the
reads (`Emulator::symbol_split_load`), which extends `indexed_load` to several
symbols and to atoms left after the substitution. A symbol whose location was
opened is never enumerated: a fetch or dispatch decided by it stays
unresolved, and the function carries the incomplete warning.

A node that ends in a native conditional reads each successor's context from
that successor's own state (`Explorer::emulate_node`). A conditional that only
skips to the next instruction, a `cmov`, first joins its two sides into one
state selecting on the guard, and the context is read from the joined state.
Read from one side, the key would name a single VPC that the state does not
hold: two virtual branches to one label would then reach the same key, and the
second branch's fall-through, which the key cannot tell apart, would be lost.

**External calls.** A call into the foreign code section to an address that
is not a known function is the interpreter's own dispatch. It is followed as a
jump, salted as above. Every other call is not followed, and neither is a jump
back into native code over a return address the interpreter has pushed (its
`push ret; jmp f` call form). A jump to another function's entry is a tail
call. A computed target that is not code but was read from a data slot of the
image is a call through that slot (an import or GOT entry). It is a call when
the interpreter pushed a return address, and a tail call through the slot otherwise.
A tail call leaves the caller's return address on top of the stack, so either
one is taken only with the stack pointer at its entry value
(`Explorer::misplaced_tail_call`). Anywhere else something was pushed that the
emulation could not place, typically the return address of a call whose push
went through an address it could not resolve, and ending the path there would
drop the rest of the function in silence (the plain Themida backdoor sample
lost everything after `WSAStartup` that way). The exit is then kept
unresolved, and the header counts it: "N jumps to another function with the
stack away from its entry depth". The emitted call loads the slot and
calls what it holds, so it is named the way a native `call [slot]` is: by the import symbol at the slot. Themida keeps the
program's import slots but rebuilds the import directory with one import per DLL, so most of them carry no name; an
`import <slot> <name>` assertion (00, *Caller assertions*) supplies it. `Explorer::apply_call_summary` models a call that is not followed. The return
register gets a fresh call-result symbol. Each ABI scratch register (RCX,
RDX, R8-R11 and the arithmetic flags on Windows x86-64, RSI and RDI as well
under the System V convention every other x86-64 image uses, ECX/EDX on x86)
gets a call-clobber symbol, except a register that a callee in the image never
writes, when its code was read to every return without calling out. The
return address is popped, together with the arguments the callee removes
itself (the stack purge below). A call the exploration does not enter may
write every location whose address its caller could have given it
(`State::clobber_call`), and a read after the call is a different value from
a read before it. The pointers it may have are the argument registers and the
words of its outgoing argument area: 16 stack words above the return address,
past the 0x20 bytes of home space on Windows x86-64, or more when the callee
pops more (`CreateProcessA` pops ten, the last its `PROCESS_INFORMATION`
pointer). Every entry-stack offset found there is recorded as escaped, over
all rounds, and a round that escapes a new offset is followed by another, so
the calls emulated before it in the round also see it. The call then leaves
its output in every byte of mutable image memory, in every region under a
symbolic base other than the entry stack pointer (a pointer argument, or one
reachable from it through memory, may point anywhere into any of them: libpng's
`png_inflate` writes `*output_size_ptr`, the caller's pointer parameter passed
in its seventh stack slot), in the frame from the lowest escaped local up
to the return address, and from the lowest escaped incoming stack argument up
(the same bytes the emitter keeps alive across a call), and in the whole stack
below the return address, where the callee's own frame lands. Stored bytes
there are dropped and unwritten ones read as that call's output. Only the
return address and the frame between it and every escaped offset keep their
values: no address the callee received reaches them. An interpreter leaves
copies of its context below the stack pointer when it calls out (VMProtect's
register context lies below the native stack once the exit has restored it),
and those copies hold nothing after the call. Before, the emulation kept them:
in a 32-bit VMProtect build the loop counter of the `connect` loop was still
at its copy 0xec bytes below the call's stack pointer after the call, the
rebuilt loop test (below) read the counter from there, and the decompiler,
which knows that a call overwrites its callee's frame, printed a test of a
local nothing assigns. Entered past a native prologue, the caller's
return address sits above the entry stack pointer, inside that range when an
incoming-area local escaped; a jump through such a slot the function never
wrote is still the return (`Explorer::is_caller_slot`), whether or not a call
may have written it since (the Dolphin and Tiger `just_if` builds). Before, only the 16 bytes behind each pointer
argument changed and, on x86-64, no stack-passed argument was looked at, so
libpng's `if (*a3 != size) ret = Z_DATA_ERROR` after `png_inflate` folded away,
and with it the `png_chunk_benign_error` call behind `a1 - a2 != avail` (the
local whose address `png_inflate` got in its fifth slot). A
`scanf("%d", &v)` writes `v` the same way: `v` escaped. An argument register that still holds an earlier call's clobber
symbol was never set by the program and points at nothing; taking it for a
pointer would open a fresh memory region in every later state for each call
site. Emulation continues at the return address. On a Windows image
(`ThreadBlock`) the segment register that addresses the thread block is
fixed to a synthetic TEB, with a PEB, process heap and image base behind it,
holding the values an ordinary undebugged process sees. The protector's TEB
and PEB checks (debugger flags, heap flags, OS version, the block's own
address) then fold instead of forking the exploration on an unknown.

**Stack purge.** A 32-bit `__stdcall` callee (nearly every Win32 API outside
the C runtime) removes its own arguments when it returns. The interpreter
re-enters with the real stack pointer, so a summary that popped only the
return address would leave the emulated stack pointer short by the callee's
purge, and every stack address computed after the call would land at the
wrong entry-stack offset: the buffer `recv` filled would be passed to `strcmp`
16 bytes away, a store and a load of one local would miss each other, and a
loop around an API call would come back to its head deeper than it left.
`Explorer::callee_pop` decides the purge where the default model leaves
cleanup to the callee (x86 Windows, `__stdcall` with an unknown extrapop;
everywhere else, x86-64 included, a callee pops nothing), the way angr's
`SimCC.teardown_callsite` pops the arguments of a callee-cleanup convention:

- Code in the image pops what its `ret imm16` says (`Explorer::scan_callee`).
  Its direct flow is read from the entry, and every return reached must pop
  the same amount.
- A native call whose return address holds an instruction that takes the
  arguments back (`add esp, n`, or a pop into a scratch register) is
  `__cdecl` and pops nothing (`Explorer::caller_cleans`).
- Otherwise what is declared about the target or slot decides
  (`ArchTarget::declared_pop`): the model its name was declared under (a C
  runtime import is `__cdecl`), or the parameters of its parked prototype (the
  built-in Win32 and libc tables). A variadic prototype pops nothing, and so
  does code in the image whose returns were not found.
- Otherwise the callee pops the arguments pushed for it
  (`Explorer::pushed_arg_bytes`). The emulator records in the state every
  store relative to the entry stack pointer (`State::note_store`), and each
  external call clears the record. The arguments are the stack words above
  the return address that the program stored to since the previous call, up
  to the first word it did not store to, which is the caller's own frame. A
  word holding a callee-saved register's entry value is the caller's too (the
  prologue's saves, before the first call) and also ends the count. A run of
  16 stored words with no such end counts as none. An argument pushed for an
  outer call before an inner call (`push b; call g; push eax; call f`) is
  counted for the inner one, and the outer call then counts one fewer, so the
  error ends at the outer call.

**Emission.** `decompiler/crates/kuna-decomp/src/deobf/kuna_devirtemit.rs
(emit)` turns the graph into a
`decompiler/crates/kuna-sleigh/src/kuna_pcodeoverlay.rs (PcodeOverlay)`. Each
node becomes a run of synthetic instructions, one per native instruction it
executed, at addresses past every mapped section (0x2000 bytes per node; the
function entry address maps to the entry node). The native p-code is copied
with these edits:

- Control flow is rewritten to the resolved exits. A pruned conditional
  becomes a jump. A dispatch call or `push; ret` becomes a jump. A return to
  the caller stays a return, even with the entry stub's extra pops folded in.
  A return through an untouched stack slot above the entry stack pointer is a
  return too: Themida starts the virtualized code after the native prologue,
  so the caller's return address sits past the prologue's frame.
  A symbolic guard keeps its CBRANCH, one per CBRANCH of the instruction in
  order, and a conditional the emulation decided is dropped wherever it sits.
  A guard that only skips to where the instruction continues anyway (a
  `cmov`) becomes a branch inside the instruction to its exit, so the move
  stays conditional and dies with the ops it skips when they are dead. A
  computed target that could not be enumerated keeps its original op, except
  that a dispatch's `ret` becomes a computed jump, since it is not the function
  returning; an unexplored node is a dead end.
  Within a node where nothing forked, the executed path is emitted (loops
  unrolled) and the decided internal branches are dropped.
- Folding. A data op whose result the emulation fixed to a constant becomes a
  COPY of that constant, which removes the bytecode fetches, key arithmetic and
  handler addresses. The exception is a load from writable memory, which stays
  a load, because the image value there is not a fact, unless it was a fetch
  through an in-function interpreter's VPC (`AccessRecord::trusted`).
- Stack-pointer rematerialization. The entry trampoline saves the entry stack
  pointer in a frame register, placed past the witness slots. A register result
  equal to the entry stack pointer plus a constant is rebuilt as `frame + k`
  (`Emitter::frame_op`), and a stack access is addressed through such a
  `frame + k`. This undoes the obfuscated stack arithmetic that the paper's
  stage 3 repairs later. Because the rebuilt value is absolute, the decompiler
  always knows where the stack pointer is: a merge of two paths, or a callee
  whose stack adjustment the prototype states differently from the emulation,
  cannot leave it unsure.
- Interpreter memory privatization (`Emitter::plan_private`). An access at a
  fixed address into the protector's own code sections (Themida keeps its VM
  context and rebuilt handler tables there) becomes a copy through a register
  placed past the frame register, one register byte per memory byte the
  emitted code touches. The address is fixed when it is a constant or when
  the emulation's own load or store placed it at one through the probe
  (`Emitter::access_addr`). Those registers are loaded from memory in the
  entry trampoline, for a path
  that reads a cell before the interpreter writes it. Every external call
  preserves them. The decompiler then forwards and kills the VM context like
  locals instead of printing hundreds of global stores. A byte that some
  emitted op still reaches as memory keeps every access overlapping it as
  memory, so the two views never mix: such ops are a forked instruction's raw
  ops, an access repeated inside one instruction, or an access the emulation
  could not place whose address is a constant inside the sections plus terms
  whose possible bits bound them (`Emitter::access_window`: a cell selected
  by a value the emulation left open), which keeps every cell of its window
  memory. An access through a pointer with no such bound is not considered;
  what it leaves in the body is reported (below).
- Lock prefixes. LOCK and UNLOCK are effect-free for liveness. They are dropped
  from an instruction whose remaining memory access, if any, is the function's
  own stack, because there is nothing left to guard once the shared access was
  folded, privatized or found dead.
- Dead-code elimination. Backward liveness runs over registers and the stack
  bytes from -0x8000 to +0x1000 relative to the entry stack pointer. An
  instruction dies when nothing it defines is read and it has no other effect.
  The liveness is strong: an op's inputs become live only where the op itself
  is live. One worklist fixpoint over the nodes therefore finds a chain of
  copies nothing reads (the interpreter moving a value through its context
  slots, handler after handler) in full, a loop-carried one included, and a
  final sweep marks it. Repeating plain liveness and removal until nothing
  changes would peel one node of such a chain per round (71 rounds on a
  32-bit VMProtect function).
  Inside a live instruction, a stack store nobody reads dies with the address
  arithmetic that fed it. In an instruction with branches among its own ops,
  everything before its trailing exit ops dies together when none of it has an
  effect or writes anything read afterwards (`dead_body`). A return keeps the return register and the
  callee-saved registers, minus those the emulation proved still hold their
  entry value. An external call keeps the argument registers (RCX, RDX, R8 and
  R9 on Windows x86-64; RDI, RSI, RDX, RCX, R8 and R9 under System V; ECX and
  EDX on x86, which `__fastcall` and `__thiscall` pass and in which MSVC's
  `__security_check_cookie` takes the cookie), its outgoing argument area (16
  stack arguments: 0xa8 bytes from the return address on x86-64, home space
  included, 0x44 on x86, or more when the callee pops more, as a 12-argument
  `CreateWindowExA` does), and every stack byte at
  or above the lowest entry-stack offset the function ever handed to a call,
  because an escaped pointer can reach any of that. It keeps nothing below.
  It also ends the life of what it overwrites. These are the scratch registers
  the call's summary replaced with clobber symbols (`Node::call_clobbers`:
  every one the callee was not shown to keep). The return register stays
  alive, because the decompiler knows callees that keep it, MSVC's
  `__security_check_cookie` among them; a `mov eax, -1` before that check is
  the function's return value. The call also ends every stack byte below the
  call floor, the lowest stack pointer any external call of the function is
  made with, or the lowest escaped offset if that lies deeper (`call_floor`).
  Only the interpreter's scratch and the callees' frames live below the floor,
  so every call may overwrite it, and a value stored there before a call is
  dead even when the interpreter reads the slot after it. The bytes between
  the floor and a particular call's stack pointer are not killed there: they
  can hold another call's arguments, and the decompiler models the same floor
  (see below). The loads the emitter adds itself are classified like the
  program's own: an import slot of a call, the VPC cell of a forked select,
  and a rebuilt condition's variable home. A load from a constant address
  reads no stack byte, and a load from `frame + k` reads only its own. Without
  that, the import-slot load in front of every call through the import table
  counted as a read of the whole stack. In a VMProtect build whose register
  context lives on the native stack (the plain build, not the Ultra one), the
  last copy of every context slot before each call then survived: hundreds of
  rebuilt flags words, saved virtual registers and bytecode constants.
- Undefined-argument placeholders. Before an external call, an argument
  register that holds an earlier call's leftovers on some path into the call,
  and on every other path its entry value or such leftovers, is one the
  program never defined. The same applies to the return register at a
  return, when the function merely restores the caller's value or returns a
  call's leftovers. A merge symbol counts as well when every value it stood
  for on the paths into its node was such a value (`Emitter::never_defined`). Each round records, for
  every merge symbol, the value its location held on each path that reached
  the node (`FlatCfg::sym_sources`, up to 64 values), and a cycle of symbols
  that only pass each other around adds nothing. That is the case in a loop
  of a virtualized function: the interpreter keeps the program's registers in
  its context and reloads all of them before every call, so at a loop head the
  context slot of R8 held one call's leftovers on the way in and another
  call's leftovers on the way around, and became a symbol that a
  two-argument `strcpy` inside the loop would otherwise take as its third and
  fourth arguments. Such a read is
  rewritten as a read of a placeholder register (`undef_register`) that no
  instruction writes, so the parameter analysis rejects it the way it rejects
  a register a call clobbered in native code, instead of inventing a parameter
  or a return value.
- Entry-value arguments. An argument register that holds its entry value
  on every path into a call is the caller's own argument, which the program
  may be passing on: zlib's `gz_read(state, ...)` calls `gz_load(state, buf,
  n, &n)` with its own `state` still in RCX. It is rewritten as a read of a
  register (`entry_register`) that the overlay's entry instruction sets from
  the argument register and every call preserves, so the parameter analysis
  sees exactly what it sees in native code that leaves the register alone:
  a realistic but inactive argument, which becomes one when a later
  argument does. An argument in a hole before a real one is then kept, as
  is everything after it. Taken for a never-defined register before, it was
  rejected as definitely unused, and the parameter list ended there:
  `gz_load` printed with no argument at all, so the C let no call change
  `n`, and `len -= n` after it read the byte count asked for instead of the
  count read. A value built from the entry value without being it is left
  as the program computed it.
- Rebuilt select conditions. When the deciding atoms of an atom-labelled select
  read only variables whose homes are recorded at the node's exit (a register,
  or a stack slot addressed as `frame + k`), each atom is compiled from those
  homes into fresh ops, and the exit branches on the compiled comparisons
  (`Emitter::rematerialize`). The condition is then the program's own
  comparison, not the interpreter's flag arithmetic that happened to compute
  it. Before that, `kuna_devirtflow.rs (fuse_signed_compares)` replaces a pair
  of sign and overflow atoms of one addition, which the assignments consult
  only through their exclusive or, by the single signed comparison it encodes
  (`a < -b`). That pair is the shape of a `jl`, `jge`, `jg` or `jle`.
- Variable witnesses. A variable that a select's atoms read but that no
  register or stack slot holds at the node's exit (a value loaded from the
  heap, tested and then dropped by the interpreter) gets a register slot past
  the boolean slots. The emitted op whose emulated result is exactly that
  variable also copies it into the slot, and the rebuilt condition reads the
  slot. `while (1 < *(int *)(heap + 4))` is then rebuilt from the program's
  own load, instead of from the VPC that the interpreter computed from its
  flags.
- Witness slots for selects. When a condition cannot be rebuilt, each deciding atom of an atom-labelled select gets
  a one-byte slot in a register range past the real registers. The emitted op
  that computes the atom, or its negation, also copies it into the slot. When
  every atom of a select has been witnessed, the exit is a chain of CBRANCHes on
  the slot conjunctions, so the decompiler sees the program's own comparison
  as the branch condition. Otherwise a select that forked the VPC compares the
  VPC's register or cell against each arm's VPC. Its arms usually continue at
  one native address, so their targets cannot tell them apart. Any other
  select compares the computed target against each destination's native
  address. Every external call preserves the witness slots, as it preserves
  the frame register.

**How the decompiler consumes it.** The program is handed over as a recovered
body (`kuna_devirtualize.rs (DevirtProgram::into_body)`), and the plane's front hook
(§10.1) is called by
`decompiler/crates/kuna-decomp/src/infra/decompile_drive.rs (generate_flow)`
before the `FlowInfo` is built. It installs the overlay on the SLEIGH
translator (`set_pcode_overlay`), which serves an overlay instruction's p-code
and listing text before decoding any bytes. It also clears the function's
declared extent, and the flow follower's mapped-byte checks accept overlay
addresses. The function gets a header warning with the node, instruction and
round counts, and an instruction budget of at least 4,000,000.

**The exploration's limits.** An exploration that does not reach its fixpoint
is ended by a work budget, a memory bound or a wall-clock backstop
(`Explorer::exhausted`), and a round that reaches 200,000 nodes ends at the node
limit (`KUNA_DEVIRT_MAX_NODES`). The work budget and the memory bound are
functions of the work done alone, so one input stops at the same step, with the
same output, on any machine and under any load. A wall-clock budget cut the
targets near it at a different point on every run, and a loaded machine cut
targets that reach their fixpoint on a quiet one. The work budget counts the
steps that take the exploration's CPU time (`ExprArena::charge`,
`kuna_symexpr.rs (Work)`): each p-code op emulated, each constructor call
the memo does not answer and each one it does, each node a substitution visits,
and each node the identity test evaluates at each sample point. They are
weighted 2, 34, 13, 2 and 1 (`WORK_WEIGHTS`, units of about 0.09 microseconds
of CPU on the loaded test machine), from a least-squares fit of each
exploration's CPU time on its counts (relative error, 43 explorations of the 48
test targets), which puts the emulation's cost into the constructor calls each
op makes; an op keeps a floor of two units. Over those explorations the weighted
work and the CPU time correlate at r = 0.98 (0.99 between their logarithms);
emulated ops alone reach 0.60, because the identity test on Themida's mixed
boolean-arithmetic stack addresses and the substitutions of its case splits
cost as much as all the emulation and have little to do with the number of
instructions. The budget is 2.3 billion units (`KUNA_DEVIRT_MAX_WORK`):
libpng_themida, the slowest target that reaches its fixpoint, needs 1.93
billion, which take about 105 s on a quiet machine, so the budget is about
125 s of such work. The memory bound is 3,328 MiB (`KUNA_DEVIRT_MAX_MEMORY`,
bytes) on an estimate made of counts too (`Explorer::memory_estimate`): the
capacities of the arena's node tables and caches, the store pages alive
(`kuna_symstate.rs (live_pages)`, counted by the pages themselves as they
are made, copied and dropped, a page several states share counted once), the
round's graph with its instruction traces, the lifted instructions, and what
the emission of the graph will hold on top: 164 bytes per op of the graph's
instructions (each op's rewrite, its memory reference and its liveness) and
6,600 per node, a least-squares fit of the emission's peak over the 48 targets.
Of the targets that reach their fixpoint, libpng_themida has the largest
estimate, 3,091 MiB (the process peaks at 3,467 MiB). The process holds the
loaded image and the decompiler besides, which the estimate leaves out; over
the 48 targets its peak lies between 754 MiB below and 540 MiB above the
estimate, so an exploration the bound stops peaks below about 3,900 MiB:
cairuh_themida (32-bit Themida) stops in its second round at 26,936 nodes with
the process at 3,752 MiB, where the wall-clock budget
let it run to 61,525 nodes and 12 to 16 GB on a quiet machine. The last
round's graph is dropped before the next round starts, since nothing of it is
read again, so two rounds' states and traces never coexist. The wall-clock
backstop (600 s, `KUNA_DEVIRT_SECONDS`) is five times what the work
budget takes on a quiet machine and only ends an exploration on a much slower
or overloaded one. The limits are checked between nodes, before each
instruction of a node, and at each location a merge compares, and a case split
gives up at the same point (`ExprArena::work_limit`, `ExprArena::deadline`,
both cleared when the exploration ends, so the emitter's splits depend on
neither). No single step therefore runs past a limit by more than one
instruction or one substitution. The round in progress is then the last: a
node stopped inside ends there as a path not followed. A second header warning
(`DevirtProgram::caveats`) says what the recovered program may miss or get
wrong: computed jumps left unresolved, paths not followed, an exploration cut
short by its work budget, its memory bound, its node limit or its wall-clock
limit (the warning names which) or ending without its fixpoint, paths joined at
another stack depth without merging (their values never reached the node), and
stores the emulation could not place. The exploration and the emission run
off kuna's per-function watchdog (`--max-fn-seconds`, 120 s by default on
`kuna decompile`): the time they take is added back to the drive's deadline
(`decompile_drive.rs (uncharge)`) and left out of the flow time a later drive
is charged, so the watchdog covers only the decompilation of the recovered
program. Their own limits bound them instead; a function whose exploration
ends at one of them still prints, with the warning.

A function
that is not devirtualized gets the overlay cleared. The program's call-kill
list (the scratch registers, the return register and the placeholders) is
stored on the `Funcdata`, and `Heritage::guardCalls`
(`decompiler/crates/kuna-decomp/src/p3_dataflow/heritage.rs`) consults
`kuna_body.rs (call_effect)` to turn an unknown effect on those registers
into killed-by-call, and it turns an unknown
effect on the frame register, the witness slots and the privatized
interpreter-memory registers into unaffected, so the saved VM context, the
entry stack pointer and a value witnessed before a call flow across it. The interpreter re-saves its whole context right after
each call, reading registers the callee has just clobbered. With the
prototype model's unknown effect, that read would look like a live use of the
argument the caller placed there. The emission's call floor is stored on the
`Funcdata` too (`BodyFacts::call_floor`), and
`call_effect` turns an unknown effect on a stack range
that lies entirely below it into killed-by-call, as the emission's liveness
assumes. The alias analysis counts the interpreter's stack-pointer arithmetic
as escaping pointers, so every stack slot the interpreter touches is
address-tied, and each call would otherwise guard each of them with an
address-forced INDIRECT. Whatever the interpreter last left in its scratch
area before a call (the return address it pushed and popped again, a pushed
flags word) then reached that INDIRECT and printed as an assignment. The
floor is the lowest call's stack pointer rather than each call's own. A
call's slots above the floor can hold arguments pushed for a later call, and
cutting their flow at every call shortens the covers the variable merge
compares, which merges values the program keeps apart (a call's result with a
loop temporary). The exploration's escaped frame offsets are stored on the
`Funcdata` as well (`BodyFacts::escapes`). The store that handed a
call such an address is often not taken as an argument (those calls print
without arguments), so it has no reader and is gone before the alias
analysis runs, which then sees no pointer to the slot. Each escaped offset is
therefore added to the local-alias mapping's alias list
(`restructure_varnode`), as a native pointer to the slot would be, so the
symbols at and above it keep their aliases (as far as the mapping lets an
alias reach: type-locked arrays and unmapped gaps stop it). A symbol that
starts below an escaped address but holds it is still marked unaliased by
that mapping, so a slot holding an escaped address never gets
`nolocalalias` (`kuna_body.rs (holds_escape)`), and neither does an
unmapped slot from the lowest escaped local up to the return address
(`kuna_body.rs (frame_escaped)`), both in `syncVarnodesWithSymbols`.
`RuleIndirectCollapse` then keeps the INDIRECT each call puts on such a
slot, and a read after the call is not folded to the value stored before it:
libpng's `if (r == 1 && a1 - a2 != avail_in) png_chunk_benign_error(...)`
after `png_inflate` disappeared that way even with the exploration fixed
(the read was of an 8-byte symbol starting 4 bytes below `avail_in`).
Refusing `nolocalalias` to every slot up to the return address instead also
kept the interpreter's dead pushes there (pushed flags words, a stub's key),
which then printed as assignments. After block generation,
`kuna_body.rs (mark_incidental_copies)` marks every COPY out of a placeholder
register as an incidental copy, so the parameter analysis walks through it to
the placeholder itself, an input nothing wrote, instead of taking it as a real
movement into the argument register. It marks every COPY into or out of an
entry-value register the same way, so the walk reaches the function's own
input, and the entry-value registers are among those every call preserves.
Once the body is decompiled, `kuna_devirtualize.rs (warn_unshown_escapes)` takes
the locals whose address a call left in the body receives (the offsets the
last round saw in that call's argument registers and outgoing words,
`Node::call_escapes`, keyed by the call's overlay address), counts those the
body still reads or writes while no op of it computes their address, and the
header says so ("N locals whose address a call receives printed as never
passed"): printed that way, a local looks as if no call could change it,
which is the claim the call model exists to avoid.
`kuna_devirtualize.rs (warn_protector_state)` then counts what the decompiled body
still reads or writes of the protector's own memory: the candidate
interpreter sections (`Section::vm_candidate`) the exploration ran VM code
in, recorded as `DevirtProgram::protector`. A protected program keeps its
own data where the compiler put it, so a location there is the
interpreter's state, unless it is an import slot a call or tail call of the
exploration went through (`DevirtProgram::slots`). The count runs over the
final ops, so dead code and heritage's own joins (MULTIEQUAL, an INDIRECT or
a COPY that keeps a location's value in place) count nothing: an op reading
or writing a memory varnode inside the sections, and a load or store whose
address is a constant inside them or adds one (through casts and copies).
A body with any such access carries
"devirtualize: incomplete: N reads and M writes of the protector's own memory left
in the body". Such a body computes with a VM context cell as if it were a
global of the program, which is what the exploration could not resolve (a
Themida loop symbolized past its fixpoint), or holds the interpreter state the
privatization had to leave in memory.
`kuna_devirtualize.rs (warn_lost_stores)` finally counts the stores the exploration
lost that lie on a path of the decompiled body: the emitter records the
overlay address of every live instruction whose store was lost and not found
dead in the overlay (`EmitStats::lost_stores`), and a store counts when a
basic block the final function keeps covers that address. The block, not the
store op, decides: a value the exploration read back from before the store is
folded into the program as a constant, so the decompiler finds the store
itself dead and removes it, and the body says something false with no trace
of the store left. Code the decompiler removed counts nothing. A body with
any such store carries "devirtualize: incomplete: N stores through an address the
emulation could not place on a path of the body". An unplaced load needs no
count: it reads an unknown, which the emitter never folds, so the overlay
keeps the program's own load.

`Heritage::guard` gives every stack range of a devirtualized body
upstream's store guard, an `indirect_store` INDIRECT at every STORE that may
reach it, whatever `option indexaliasguard` says
(`kuna_body.rs (guards_every_store)`). An in-function interpreter such as
Tigress's keeps its operands on a stack of unions in the frame. It pushes the
address of a variable's slot there, adds a scaled index to the address it pops,
and stores through the result, so in the emitted body a store into the
program's array goes through a frame address reloaded from a slot. On the
stack's first heritage pass the indexed-pointer discovery follows such reloads
only for LOADs, the frame's addresses do not resolve to the stack base plus
constants and indices, and the windowed policy guards an unbounded index for
only four elements. Without the guard, the SSA ties a later read of a slot to
the value from before the store, the earlier store loses its reader, and the
C reads values the program has overwritten. With the guard, 519 more of the 999
Tigress samples print C that computes what the source does. `tg_istore` in the
Tigress-style fixture pins it.

On the stack's first pass a LOAD guard found through a spilled pointer forces
the writes in its window only below the lowest slot above its base that a stack
pointer was spilled to (`heritage.rs (Heritage::spill_guard_maximum)`). In a
devirtualized body only a slot written by a path that did not itself pass
through a spill counts. The interpreter's operand-stack slots hold the address
of a variable's slot at one step and a value at the next. The spill walk is not
flow-sensitive, so it takes every read of such a slot for the pointer, and every
slot of the program's array that a value is copied into counted as a spill. The
window then ended at the array's second element, a write read only through an
indexed LOAD had no reader, and the dead-code pass deleted it, or the part of it
a later write did not cover (a Tigress loop that swaps bytes of a slot kept its
low half and zeroed the high half). The operand-stack slots themselves, which
the interpreter's own pushes write a frame address into, still bound the window,
so their traffic is not kept. `tg_carried` in the fixture pins it.

**String literals.** A short string the program passes to a call (`"123"` to
`strcmp`, `"cmd"` to `CreateProcessA`) would print as its address. The
always-on strings pass plants a typelocked `char[N]` only for runs of five
characters or more. A shorter literal gets one only from `operand_refs`, when
an instruction operand points at it, and in a protected image that operand is
inside the bytecode. The native reference of such a function prints the
literal only through `operand_refs`, which the auto mode turns on for small
images. The type of the argument does not help either. A 32-bit
`call [slot]` through the import table lifts as a COPY from the slot into a
temporary and a CALLIND on the temporary. When `ActionDefaultParams` runs,
that temporary is not yet linked to its COPY, so the call never receives the
prototype parked at the slot, and the constant stays an `undefined4` that the
printer's `char *` path never sees. So `devirtualize` collects the program's
literals (`kuna_devirtualize.rs (string_constants)`) as `operand_refs` does for a
native operand. These are the pointer-sized constants a COPY or STORE of the
emitted program writes (the values the emitter folded, and the immediates of
the native code it kept). A constant qualifies when it lands in a read-only,
non-executable section at a run of printable characters (tab, CR and LF
included) closed by a NUL within 1024 bytes (`ArchTarget::string_at`). Two
cases are rejected. A pointer-aligned word that holds a mapped address is a
pointer slot. A run read as the head of an array of 2- or 4-byte character
codes (`c 00 c 00 00 00`, a short UTF-16 literal) counts only when the second
element is also a constant of the program. When the body is adopted,
`kuna_deobf.rs (plant_strings)` plants a typelocked `char[N]` symbol
`s_<addr>` at each literal that no symbol covers and the symbol table marks
read-only, as the analysis commit does for `operand_refs`, and the printer
renders the reference as the literal. The function was built with a snapshot
of the symbol table taken before the plant, so when something was planted it
takes a fresh architecture view (`Funcdata::kuna_set_arch`) before its flow
is generated. A native function only sees the new symbols if it is decompiled
later in the same process (`decompile-all`), and there they render as the
literal that `operand_refs` would print.

Stack arguments (every argument of a 32-bit call) need two more allowances,
both part of the body's policy (`BodyPolicy::inputs_by_dataflow` and
`BodyPolicy::join_stack_inputs`).
First, the local-alias test of the input trials is off: the deferred alias
checker `ActionActiveParam` builds
(`decompiler/crates/kuna-decomp/src/p6_variables/funcdata_spacebase.rs (build_alias_checker_deferred)`)
answers there, and only there, as if no pointer into the stack had been
gathered. The same checker answers heritage (whether a call may reach a stack
slot) and the dead-store pass with every pointer it gathered, since a stack
address the program passes to a call does reach that call. The gather takes
every non-additive use of a stack address as a pointer escaping to all locals
above it, and the interpreter makes such uses of addresses deep in the frame
throughout: its virtual stack pointer compared against the native one, its
context copied down the frame, the flags of that arithmetic pushed. The
boundary would fall below every call's outgoing arguments and reject each stack
trial as aliased, so the trials are judged by their data flow alone. Second,
the interpreter reads and writes its stack in halves, so heritage refines the
slots the program's arguments later occupy into 2-byte pieces, and the ordinary
guard would pass a call only the justified low piece of each slot (a pointer
argument truncated to its low 16 bits). `Heritage::guard_joined_stack_input`
registers a slot-sized trial instead, whose argument is the PIECE of the
heritaged ranges of the pass that tile the slot.

A devirtualized body is structured twice. `ActionBlockStructure` first builds
the tree on the main loop's first pass, as for any function, and keeps it
unless the control-flow graph changes. At that pass the stack and RAM are not
yet heritaged, so each block still holds the interpreter's stores to its
context, the flag words it pushed and the copies it made down the frame. The
emission's liveness keeps them because an external call might read them, and
only the decompiler's own dead-code pass removes them, once those spaces are
heritaged. Until then, `Funcdata::bb_is_complex` counts them as statements. A
loop head that only compares a counter (`4 < i` and a CBRANCH, out of 1,800
ops in the function at that point and 16 one pass later) is then complex, and
`rule_block_while_do` gives the loop overflow syntax:
`while (true) { if (4 < i) break; ... }` instead of `for (i = 0; i <= 4; ...)`.
`kuna_body.rs (restructure_settled)` therefore has the structure built
again from a fresh `BlockCopy` mirror on the first pass at which dead code may
be removed in every heritaged space (`Heritage::deadcode_settled_pass`, the
largest dead-code delay plus one). This happens once, in the main loop, before
the transforms that read the tree (`ActionStructureTransform`'s for-loop
recovery, branch preference). Native functions keep the one structuring that
upstream does.

A structuring negates branches as it goes (a rule that needs its clause on the
true edge swaps the block's out-edges and flips its CBRANCH), and those
negations stay on the basic blocks after the tree is discarded, whether by this
rebuild or by an ordinary reset after a control-flow change. When both arms of
a split leave the function, `rule_block_if_no_exit` takes the false edge and
negates, so every rebuild picked the other arm: the early `if (!p) return;` of
a guard and the inverted `if (p) { ...rest... } return;` alternated with the
number of rebuilds. For a devirtualized body the blocks each structuring
negated are recorded on the `Funcdata`
(`kuna_body.rs (record_structure_flips)`), and `ActionBlockStructure`
negates them again before it builds the next tree
(`kuna_body.rs (begin_structure)`, `kuna_body.rs (unflip_structure)`),
skipping a block that no longer ends in a two-way CBRANCH. Every structuring
then starts from the orientation the emitter chose, so the result depends on
the program and not on how often it was structured. The action's own second
tries (the `loopcontinue auto` and `condfold noreturn` comparisons, which put
the out-edges back and structure again) start from that orientation as well,
and the record holds the negations of the try that was kept.

Pinned by `tests/stages/kuna-devirtualize.xml`. It runs two passes over the
synthetic `devirt_vm_x86_64` fixture, a push/ret-dispatched stack-machine
interpreter in a 64 KiB `.vmp0` whose bytecode computes `arg0 == 0 ? 7 :
arg0 + 5` with a cmov-selected virtual branch. With `deobfuscate off`, the function is an
empty body. With the default, the virtual branch and both returns are
recovered. `tests/stages/kuna-devirtualize-sound.xml` decompiles twenty
hand-written programs of the `devirt_vm2_x86_64` fixture, each aimed at a way
the recovered program could say something false: loops whose exit tests need
more than two observed counter values (a `setl` the JZ selects on by `cmov`,
an equality, a bound of 2^40), a branch on a comparison stored and reloaded,
correlated branches, two branches to one label, values across System V calls
and a local a call writes, a read-only table indexed by the loop counter, a
writable global read before and after a call, stores through a pointer
argument on two paths and in a loop, two programs the header reports as
incomplete (a loop that grows the stack, a virtual jump through an argument),
two virtual exits into one native epilogue, one of them an instruction
ahead of it, whose call the recovered program makes once, and a virtual branch
whose handler computes the next VPC from the low bit of a flag a join merged
and from a comparison (`f_sel`), which splits over the merge symbol's two
recorded values, a call to a function of the image that writes a local
whose address it got in a stack-passed argument and a field 32 bytes past its
pointer argument (`f_out`), both tested after it, a value kept in a context
cell of the interpreter's section and read back at an offset decoded through
a key (`f_ctx`, which returns the value), two cells read at an offset the
argument selects (`f_ctxi`, whose stores stay and whose header reports the
protector state left), and a local written through the interpreter's copy
of its frame pointer, kept in an unaligned context cell beside one that the
two arms of a branch set differently (`f_cell`, which returns the value
written), and a local written at an address no identity places (`f_lost`,
whose header reports the lost store).
`tests/stages/kuna-devirtualize-interp.xml` runs two passes over the
`devirt_interp_x86_64` fixture (gcc -O0 from its committed C source), six
functions that are their own interpreters of one virtual program with a loop
and a branch, and two that interpret other programs in Tigress's style, every
variable reached through its slot's address pushed on the virtual stack. With `deobfuscate off`, the switch-dispatched one decompiles to its
interpreter. With the default, switch dispatch over writable and read-only
bytecode, threaded code copied by `memcpy` into a stack-probed local or by an
inline `rep movsq` that leaves no copy loop behind, and two
programs interleaved through `pc[sel]` recover the loop and the branch, only
the writable ones carry the bytecode clause, the interleaved one says it
follows its first program, and bytecode the function patches with its
argument is reported incomplete. The Tigress-style program keeps its stores
to `st` and the branch around one of them, which only a loop through `&st[0]`
plus a computed offset reads (chapter 03's spilled stack pointers). The
second Tigress-style program writes its array through `&st[0]` plus a computed
offset and returns two slots it reads directly, and the C returns the slots,
not the values stored before the loop.
The expression arena and the state have unit tests in
`decompiler/crates/kuna-decomp/src/deobf/kuna_symexpr/tests.rs` and
`decompiler/crates/kuna-decomp/src/deobf/kuna_symstate/tests.rs`;
`decompiler/crates/kuna-decomp/src/deobf/kuna_devirtflow/tests.rs` explores a
counting loop under a tiny work budget, memory bound and wall-clock limit and
checks that each ends the exploration with the caveat naming it, and that a
budget cut stops two runs at the same step; and
`decompiler/crates/kuna-decomp/src/deobf/kuna_residuerules/tests.rs`
checks every rewrite of the four arithmetic rules on random byte-wide trees
against all 65,536 pairs of inputs.

