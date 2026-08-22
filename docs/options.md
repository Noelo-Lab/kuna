# kuna option catalog

**Generated** by `kuna catalog --markdown` from `settableTable` in `decompiler/crates/kuna-decomp/phases.toml` -- do not edit by hand; edit the registry row and regenerate.

Set any option per run: `kuna decompile <bin> <fn> --option <name> <value>` (repeatable), `kuna decompile-all <bin> --option <name> <value>`, or the console `option <name> <value>`. Defaults are the shipped values; every intentional default change is logged in `docs/history.md`. The algorithms behind these options are described in `docs/spec/`.

Three tiers:

- **Toggleable transforms** -- restructure, duplicate, remove, or insert code; near-always better on the right source shape and wrong on the wrong one. **This is the on/off control surface.**
- **Analysis & loader passes** -- what gets discovered/decoded/named before decompilation.
- **Core rendering defaults** -- near-always-better rendering/naming/peepholes; part of the decompiler, not the control surface. Flip only for upstream byte-parity or unusual consumers.

## Symptom index

| If the output shows... | Try |
|---|---|
| long run of per-element constant stores instead of a single memset | [`memsetrecover`](#memsetrecover) |
| unrolled or SIMD zeroing rendered as dozens of assignments | [`memsetrecover`](#memsetrecover) |
| builtin_memset call where the individual element stores are wanted (flip off) | [`memsetrecover`](#memsetrecover) |
| spurious CONCAT44 return value in a void or single-register function | [`returnpair`](#returnpair) |
| two return registers joined into one double-width return (e.g. sparc o0/o1) | [`returnpair`](#returnpair) |
| return type twice the natural width built from a register concat | [`returnpair`](#returnpair) |
| v850 jmp [reg] rendered as an indirect call instead of a recovered switch | [`v850indirectbranch`](#v850indirectbranch) |
| jump table on v850 never recovered because the dispatch decodes as a computed call | [`v850indirectbranch`](#v850indirectbranch) |
| __ftol() / __ftol2() called with no arguments | [`msvcftol`](#msvcftol) |
| the float computation feeding a float-to-int conversion is missing from an MSVC x86-32 binary | [`msvcftol`](#msvcftol) |
| a __thiscall method lost its `this` pointer and reads its fields off a stack argument instead | [`msvcftol`](#msvcftol) |
| leaf function ends in a (*dat_...)(...) computed call with a 'Treating indirect jump as call' warning | [`tailcalljump`](#tailcalljump) |
| jmp to a plt stub inlined into the caller instead of a named tail call | [`tailcalljump`](#tailcalljump) |
| plt thunk body absorbed where func(...) is expected | [`tailcalljump`](#tailcalljump) |
| a function's tail is really the body of the NEXT function | [`funcboundflow`](#funcboundflow) |
| dead/garbage code after a call to a die()/fatal()/throw wrapper that never returns | [`funcboundflow`](#funcboundflow) |
| two adjacent functions merged into one, the second also decompiled on its own | [`funcboundflow`](#funcboundflow) |
| uninitialized reads in the tail after an unrecognized no-return call | [`funcboundflow`](#funcboundflow) |
| core::ptr::drop_in_place(...) calls all over a decompiled rust function | [`cleanupcode`](#cleanupcode) |
| drop glue and deallocation calls that are nowhere in the rust source | [`cleanupcode`](#cleanupcode) |
| Drop::drop / __rust_dealloc / RawVecInner::deallocate noise at every scope exit | [`cleanupcode`](#cleanupcode) |
| rust output is twice as long as the source because of automatic resource management | [`cleanupcode`](#cleanupcode) |
| flip off to see when a rust value is actually dropped or freed | [`cleanupcode`](#cleanupcode) |
| garbage code after a call to __stack_chk_fail or abort in a relocatable .o | [`noreturn_extern`](#noreturn_extern) |
| function balloons by swallowing the next function after an extern no-return call | [`noreturn_extern`](#noreturn_extern) |
| dead fall-through after exit or assert-fail in an object file | [`noreturn_extern`](#noreturn_extern) |
| switch reports 'Could not recover jumptable ... Too many branches' and renders as a computed call | [`switchmodbound`](#switchmodbound) |
| indirect jump bounded only by a modulo or and-mask on its index never becomes a switch | [`switchmodbound`](#switchmodbound) |
| 'Too many branches' jumptable failure where a cbranch range guard bounds the index | [`switchguardbound`](#switchguardbound) |
| computed (code *)() call at a gcc sub/ja guarded dispatch with the index spilled to the stack | [`switchguardbound`](#switchguardbound) |
| getopt-style switch inside a loop degrades to a computed call with goto spaghetti | [`switchsharedcase`](#switchsharedcase) |
| pic relative-offset jump table with a loop-carried base register never recovered | [`switchsharedcase`](#switchsharedcase) |
| 'Too many branches' on a dispatch whose table base is set before the loop | [`switchsharedcase`](#switchsharedcase) |
| image-base-relative switch whose bound guard is duplicated across multiple predecessors renders as a computed call | [`switchmultipred`](#switchmultipred) |
| 'Too many branches' where the dispatch block has several guarded incoming paths | [`switchmultipred`](#switchmultipred) |
| msvc memmove small-count tail dispatch truncated to an indirect call | [`switchmultipred`](#switchmultipred) |
| one switch recovered but sibling interleaved jump tables degrade to computed calls | [`unrolledguard`](#unrolledguard) |
| 'Could not find op at target address' during recovery of an msvc optimized memcpy dispatch | [`unrolledguard`](#unrolledguard) |
| duff's-device tail dispatch rendered as 'Treating indirect jump as call' | [`unrolledguard`](#unrolledguard) |
| dozens of garbage *v = *v + c; lines after a __stack_chk_fail call in a .o | [`noreturn_externmatch`](#noreturn_externmatch) |
| inter-function alignment padding decoded as add [rax],al style instructions | [`noreturn_externmatch`](#noreturn_externmatch) |
| flow runs past an undefined-extern abort or exit call in a relocatable object | [`noreturn_externmatch`](#noreturn_externmatch) |
| spurious uninitialized local (xStack_N) returned after storing through a pointer to a local | [`stackalias`](#stackalias) |
| store through a take-address-of-local pointer dropped as dead so the later read is garbage | [`stackalias`](#stackalias) |
| bogus (*pcVar1)() indirect call after calling a struct-returning function on sparc | [`sparcstructret`](#sparcstructret) |
| function tail dropped after a sparc call followed by unimp | [`sparcstructret`](#sparcstructret) |
| switch rendered as nested if/else-if compare tree over one variable | [`loweredswitch`](#loweredswitch) |
| binary-search cascade of constant compares where source had a switch | [`loweredswitch`](#loweredswitch) |
| flip off to keep the compiler's lowered compare-tree rendering | [`loweredswitch`](#loweredswitch) |
| call rendered with fewer arguments than the disassembly passes when the missing argument register is spilled to the frame before the call | [`spillargtrial`](#spillargtrial) |
| floating-point argument dropped at the FIRST of several calls that reuse the same value | [`spillargtrial`](#spillargtrial) |
| a spurious 4-byte stack local appears where a call argument should be | [`spillargtrial`](#spillargtrial) |
| clang -O2 packed-double code calls atan2 with one argument instead of two | [`spillargtrial`](#spillargtrial) |
| argument register stored to a caller-frame slot and reloaded after the call is not recognized as an argument | [`spillargtrial`](#spillargtrial) |
| goto placement follows the sailr region walk instead of ghidra's collapse order (flip off to force CollapseStructure) | [`regionstructure`](#regionstructure) |
| uncompilable c where a loop body folded into a while condition | [`regionstructure`](#regionstructure) |
| structuring differs from upstream ghidra on the same function | [`regionstructure`](#regionstructure) |
| guard clause with a fatal no-return call comes out inverted and the whole body is nested inside the if | [`guardarm`](#guardarm) |
| the function's only return sits inside an if-arm and the body ends on a no-return call | [`guardarm`](#guardarm) |
| kuna negates a guard condition that ida and ghidra leave alone | [`guardarm`](#guardarm) |
| a leading validation guard is emitted at the tail of the function | [`guardarm`](#guardarm) |
| loop emitted as while( true ) with the head test as a guarded exit on the first statement | [`loopcondhoist`](#loopcondhoist) |
| a for/while header condition is left inside the loop body | [`loopcondhoist`](#loopcondhoist) |
| kuna writes while(true) where ida writes while (cond) | [`loopcondhoist`](#loopcondhoist) |
| a loop's early-exit return is the loop's first statement instead of its condition | [`loopcondhoist`](#loopcondhoist) |
| goto-heavy multi-exit loop where angr recovers a clean while/for with break/continue | [`regionlooprefine`](#regionlooprefine) |
| irreducible or multi-latch loop falls back to raw gotos instead of folding | [`regionlooprefine`](#regionlooprefine) |
| secondary loop exits rendered as gotos rather than break statements | [`regionlooprefine`](#regionlooprefine) |
| more gotos than angr on a function forced to virtualize several edges | [`regionedgeorder`](#regionedgeorder) |
| goto cut at an awkward edge where a dominance/post-dominator ordering exposes more structure | [`regionedgeorder`](#regionedgeorder) |
| crossing goto from the else arm back into the then arm's shared clause | [`condfold`](#condfold) |
| goto plus label into the shared body of a guard cascade whose arms reconverge | [`condfold`](#condfold) |
| nested if where angr emits one compound short-circuit condition | [`condfold`](#condfold) |
| second condition block assigns one value or calls something before its test and the fold is declined | [`condfold`](#condfold) |
| goto to a label that only returns | [`gotoreduce`](#gotoreduce) |
| label_N: return; shared tail after an if | [`gotoreduce`](#gotoreduce) |
| more gotos than angr for the same function | [`gotoreduce`](#gotoreduce) |
| else block kept although the matching if branch always returns or aborts | [`ifelseflatten`](#ifelseflatten) |
| guard-clause body indented one level deeper than the source | [`ifelseflatten`](#ifelseflatten) |
| redundant else after an early-return arm | [`ifelseflatten`](#ifelseflatten) |
| goto into the middle of a sibling branch's arm (compiler cross-jumping) | [`crossjumprevert`](#crossjumprevert) |
| shared non-return tail reached by goto from an else-if chain | [`crossjumprevert`](#crossjumprevert) |
| label planted inside an else block targeted from another branch | [`crossjumprevert`](#crossjumprevert) |
| goto to a shared free(p); return; style epilogue | [`taildup`](#taildup) |
| cleanup-call return tail reached via label instead of duplicated inline | [`taildup`](#taildup) |
| flip off when many error checks share one cleanup block (the goto out; idiom) | [`taildup`](#taildup) |
| both arms of an if/else begin or end with the same duplicated statements | [`dedupitetail`](#dedupitetail) |
| shared cleanup or help call repeated verbatim in the two branches | [`dedupitetail`](#dedupitetail) |
| compiler-duplicated tail emitted twice instead of hoisted out of the if | [`dedupitetail`](#dedupitetail) |
| if (c) v = A; else v = B; assignment diamond where a ternary v = c ? A : B is expected | [`iteregion`](#iteregion) |
| format/flag builder full of two-arm constant assignments to the same variable | [`iteregion`](#iteregion) |
| flip off when the source likely wrote an explicit if/else | [`iteregion`](#iteregion) |
| if (c) v = <expr1>; else v = <expr2>; where a ternary v = c ? expr1 : expr2 is expected | [`iteexpr`](#iteexpr) |
| kuna emits far fewer ?: ternaries than angr/ida on format/flag/size code | [`iteexpr`](#iteexpr) |
| computed-arm assignment diamond (v = *p / v = b + k) not folded to a ternary | [`iteexpr`](#iteexpr) |
| if (a && b) v = 1; else v = 0; where the source has a single boolean assignment v = a && b | [`iteboolean`](#iteboolean) |
| -O0 boolean materialization printed as an explicit 0/1 constant diamond | [`iteboolean`](#iteboolean) |
| extra CFG blocks/edges vs Hex-Rays around a short-circuit boolean assignment | [`iteboolean`](#iteboolean) |
| only every other ternary in a chain of identical ?: assignments is recovered | [`itecondlist`](#itecondlist) |
| a re-rolled v = c ? A : B; is immediately followed by an identical un-re-rolled if (c) v = A; else v = B; | [`itecondlist`](#itecondlist) |
| iteboolean re-rolls 1 of 3 identical short-circuit boolean assignments | [`itecondlist`](#itecondlist) |
| a diamond folds in isolation but declines when a structured if precedes it | [`itecondlist`](#itecondlist) |
| giant short-circuit if with comma-expression side effects merging several source early-return guards | [`returndup`](#returndup) |
| one trailing return shared by many guard paths where the source used per-guard early returns | [`returndup`](#returndup) |
| merged guard condition containing v = f(...) assignments inline | [`returndup`](#returndup) |
| a source boolean return renders as a cascade of if (c) return 0; / if (c) return 1; guards | [`orchain`](#orchain) |
| option returndup off collapses the whole function to one short-circuit expression | [`orchain`](#orchain) |
| a source guard testing several conditions with and/or becomes that many identical constant-return guards | [`orchain`](#orchain) |
| the return-dupe slug on a function whose source wrote one short-circuit condition | [`orchain`](#orchain) |
| inverted diamond: if (guard) { entire body } else { v = 0; } return v; | [`earlyreturn`](#earlyreturn) |
| leading argument-validity check wraps the whole body instead of an early return | [`earlyreturn`](#earlyreturn) |
| function opens with if (p != 0) { everything } where source wrote if (!p) return 0; | [`earlyreturn`](#earlyreturn) |
| wide switch where every case assigns a constant and breaks to one shared return | [`switchreturn`](#switchreturn) |
| switch (x) { case A: v = K; break; ... } return v; instead of per-case return K | [`switchreturn`](#switchreturn) |
| 17-plus-case const dispatch keeps a merged return variable | [`switchreturn`](#switchreturn) |
| call result spilled to a temp used exactly once: v5 = f(); if (v5 < 0) | [`foldcallret`](#foldcallret) |
| single-use call return not inlined into its use site | [`foldcallret`](#foldcallret) |
| flip off to force every call output into a named temporary (ghidra style) | [`foldcallret`](#foldcallret) |
| stack-protector canary compare against fs:0x28 and a __stack_chk_fail branch cluttering the epilogue | [`stackguard`](#stackguard) |
| shared-return goto forced by the canary check block | [`stackguard`](#stackguard) |
| flip off to keep the real canary instructions for auditing the protector | [`stackguard`](#stackguard) |
| Rust output littered with `panic_bounds_check()` / `slice_error_fail()` / `panic_const_div_by_zero()` calls | [`securitycheck`](#securitycheck) |
| every slice index guarded by an `if` whose arm only panics | [`securitycheck`](#securitycheck) |
| Rust function CFG roughly twice the size of the source control flow | [`securitycheck`](#securitycheck) |
| flip off to keep the real bounds-check instructions for auditing | [`securitycheck`](#securitycheck) |
| if (x == 0) guard with the common path in the else arm | [`branchflip`](#branchflip) |
| negated condition where angr renders the positive complement first | [`branchflip`](#branchflip) |
| if/else polarity inverted versus the source's reading order | [`branchflip`](#branchflip) |
| loop exit rendered as goto label_N; plus a synthesized label instead of break; | [`loopbreak_recovery`](#loopbreak_recovery) |
| switch-case exit gotos where break; is expected | [`loopbreak_recovery`](#loopbreak_recovery) |
| error paths leave a loop by goto to its successor label | [`loopbreak_recovery`](#loopbreak_recovery) |
| dead code kept after calls to exit/abort/panic when off | [`noreturn_known`](#noreturn_known) |
| call to a known no-return libc function still shows a fall-through path | [`noreturn_known`](#noreturn_known) |
| unreachable epilogue after std::terminate or a rust panic call | [`noreturn_known`](#noreturn_known) |
| windows api calls render as (*dat_411324)() with no name | [`peimportcall`](#peimportcall) |
| a PE function runs past ExitProcess or ExitThread and absorbs the next function's body | [`peimportcall`](#peimportcall) |
| the same loop is emitted twice, once inside a caller and once as its own sub_ function | [`peimportcall`](#peimportcall) |
| no Win32 API names anywhere in a decompiled PE | [`peimportcall`](#peimportcall) |
| heavily-called custom die()/fatal() wrapper still treated as returning | [`noreturn_disc`](#noreturn_disc) |
| dead code after a stripped sub_ wrapper that never falls through at 3+ call sites | [`noreturn_disc`](#noreturn_disc) |
| caller swallows the next function after a wrapper call | [`noreturn_disc`](#noreturn_disc) |
| unreachable code after a call to an exit/fatal wrapper | [`noreturn_propagate`](#noreturn_propagate) |
| function truncated after calling a cold wrapper | [`noreturn_propagate`](#noreturn_propagate) |
| spurious while(true) around a call that never returns | [`noreturn_propagate`](#noreturn_propagate) |
| stripped binary's unnamed exit wrapper swallows the functions after it | [`noreturn_propagate`](#noreturn_propagate) |
| caller keeps decoding the cold path after error(2, ...) into the next function | [`noreturn_error`](#noreturn_error) |
| function balloons past a wrapper that tail-calls error with nonzero status | [`noreturn_error`](#noreturn_error) |
| dead fall-through after a gnu error(nonzero, ...) call | [`noreturn_error`](#noreturn_error) |
| wrapper with a mid-body fatal call still treated as returning | [`noreturn_reach`](#noreturn_reach) |
| unreachable write()/return tail keeps a fatal wrapper looking like it returns | [`noreturn_reach`](#noreturn_reach) |
| switch whose every arm ends in a no-return call not concluded no-return | [`noreturn_reach`](#noreturn_reach) |
| caller decodes garbage past an sshpkt_fatal-style wrapper | [`noreturn_reach`](#noreturn_reach) |
| parameter copy-shadow assignment (v6 = a1;) emitted below an earlier guard instead of at the top of the function | [`paramcopyhoist`](#paramcopyhoist) |
| kuna places some parameter spill copies in the entry block and others after a guard in the same function | [`paramcopyhoist`](#paramcopyhoist) |
| extra CFG nodes versus the source around a cascade of if (p && !*p) p = NULL; parameter guards | [`paramcopyhoist`](#paramcopyhoist) |
| vN = aM; for an unmodified incoming parameter appearing in the middle of a guard cascade | [`paramcopyhoist`](#paramcopyhoist) |
| an inlined callee body is spliced into the caller and should read as a call | [`outline`](#outline) |
| want to pull a chosen region out of a function into a named pseudofunction | [`outline`](#outline) |
| acting on an almostregion inline-candidate report by hand or with an llm | [`outline`](#outline) |
| puts/printf arguments untyped so string literals render as bare constants or dat_ addresses | [`libproto`](#libproto) |
| imports carry no signatures and call arguments stay untyped | [`libproto`](#libproto) |
| a caller's parameter is unsigned long where it is only ever passed to a libc function that takes a char */int | [`libcsigs`](#libcsigs) |
| a libc call renders its string argument as a bare 0x... constant instead of a literal | [`libcsigs`](#libcsigs) |
| a wrapper around a libc function loses its return value or return type | [`libcsigs`](#libcsigs) |
| __printf_chk / __fprintf_chk arguments are shifted by the fortify flag | [`libcsigs`](#libcsigs) |
| an exit or __stack_chk_fail call shows phantom arguments | [`libcsigs`](#libcsigs) |
| string constants render as raw addresses or unnamed data instead of quoted char[N] literals | [`strings`](#strings) |
| no data symbols at ascii runs in rodata | [`strings`](#strings) |
| a stripped binary yields almost no functions (symbol stream only) | [`entry_disc`](#entry_disc) |
| functions discovered via e_entry/init_array/.eh_frame/prologues missing from the list | [`entry_disc`](#entry_disc) |
| c++ catch/cleanup landing pads missing from a stripped binary's function list | [`eh_frame_full`](#eh_frame_full) |
| exception-handler code never discovered as entries | [`eh_frame_full`](#eh_frame_full) |
| gcc_except_table call-site targets left unexplored | [`eh_frame_full`](#eh_frame_full) |
| spurious sub_<addr> functions inside a c++ function that uses try/catch | [`fdeinterior`](#fdeinterior) |
| a decompiled function body dereferences an uninitialised frame pointer so every local is garbage | [`fdeinterior`](#fdeinterior) |
| function count inflated by unwinder-only landing pads | [`fdeinterior`](#fdeinterior) |
| a function entry lands in the middle of an instruction | [`fdeinterior`](#fdeinterior) |
| extra entries between two real functions in a binary built with exceptions | [`fdeinterior`](#fdeinterior) |
| functions after nop padding missing in a stripped binary | [`funcstart_patterns`](#funcstart_patterns) |
| a push rbx; mov rbx,rdi prologue never discovered as a function start | [`funcstart_patterns`](#funcstart_patterns) |
| code-bearing gaps between discovered functions left undefined | [`funcstart_patterns`](#funcstart_patterns) |
| stripped cortex-m firmware yields only a handful of functions | [`cortexmvectors`](#cortexmvectors) |
| arm firmware body decodes as a32 garbage instead of thumb | [`cortexmvectors`](#cortexmvectors) |
| nmi/hardfault/systick exception handlers never discovered as entries | [`cortexmvectors`](#cortexmvectors) |
| isr_vector table present but no vector-derived function starts | [`cortexmvectors`](#cortexmvectors) |
| a small arm callback or isr handler produces no decompiled output at all | [`ptrentry`](#ptrentry) |
| kuna emits one function then the next with un-decompiled code in between | [`ptrentry`](#ptrentry) |
| bare bx lr exception handlers never discovered as entries | [`ptrentry`](#ptrentry) |
| vtable or fops-struct callbacks missing from a stripped arm binary | [`ptrentry`](#ptrentry) |
| driver op-struct function pointers left undiscovered | [`ptrentry`](#ptrentry) |
| a stripped arm function list carries sub_<addr> entries two bytes below the real function | [`poolentry`](#poolentry) |
| the same arm function appears twice at addresses two bytes apart | [`poolentry`](#poolentry) |
| a decompiled arm function returns a constant or drops its first parameter because of a dead leading movs | [`poolentry`](#poolentry) |
| kuna reports far more functions than the binary has on bare-metal arm | [`poolentry`](#poolentry) |
| an arm function entry lands one halfword before the real prologue | [`poolentry`](#poolentry) |
| a32 u-boot recall is far below the cortex-m images | [`poolentry`](#poolentry) |
| thumb code misdecoded as arm garbage instructions | [`arm_markers`](#arm_markers) |
| $t/$a mapping symbols ignored so the wrong decode mode applies | [`arm_markers`](#arm_markers) |
| unresolved *(gp + offset) loads on mips | [`mips_gp`](#mips_gp) |
| got/.sdata references never fold to real addresses in a pic mips binary | [`mips_gp`](#mips_gp) |
| i386 pie libc calls render as sub_<addr> instead of exit/dcgettext | [`i386_pie_plt`](#i386_pie_plt) |
| spurious do{}while(true) or goto loop after an unnamed exit stub in a 32-bit pie binary | [`i386_pie_plt`](#i386_pie_plt) |
| kuna functions on a .o or .obj lists phantom sub_<addr> entries below 0x400000 beside the real ones | [`relocrebase`](#relocrebase) |
| functions and decompile-all disagree on how many functions a relocatable object has | [`relocrebase`](#relocrebase) |
| a -g .o names one function at address 0 and leaves the rest sub_<addr> | [`relocrebase`](#relocrebase) |
| string literals and dwarf-named globals in a .o never attach to the loaded image | [`relocrebase`](#relocrebase) |
| a call in a pie binary renders as (*dat_<addr>)(...) with no name although the callee is a named function in the same image | [`dynrelocs`](#dynrelocs) |
| a got slot reads back as 0 in the decompiled output | [`dynrelocs`](#dynrelocs) |
| reading a relocated function-pointer table in a pie binary yields all zeroes | [`dynrelocs`](#dynrelocs) |
| a glibc math/mem/str wrapper tail-jumps to `(*dat_...)(...)` with the callee dropped | [`ifuncfpret`](#ifuncfpret) |
| an x86-64 IFUNC .plt.sec stub is not a discovered function | [`ifuncfpret`](#ifuncfpret) |
| xmm0 read uninitialized after calling a void-typed ifunc-dispatching wrapper | [`ifuncfpret`](#ifuncfpret) |
| mips16/micromips function body misdecoded as mips32 garbage | [`mips_isa`](#mips_isa) |
| odd-address mips functions decode in the wrong isa mode | [`mips_isa`](#mips_isa) |
| a -g binary still shows default names and inferred types | [`dwarf`](#dwarf) |
| dwarf function/global names and typed signatures ignored | [`dwarf`](#dwarf) |
| libc globals render as dat_<addr> where every other decompiler prints stderr/stdout/optind | [`datasyms`](#datasyms) |
| fprintf stream argument shows a raw dat_ address so error paths are indistinguishable from output paths | [`datasyms`](#datasyms) |
| a stripped binary's copy-relocated externs unnamed | [`datasyms`](#datasyms) |
| an unstripped dwarf-less binary's own statics render dat_<addr> | [`datasyms`](#datasyms) |
| decompile-project exports reference undeclared dat_<addr> identifiers where a stdio global belongs | [`datasyms`](#datasyms) |
| no /* file:line */ source-location comments in the output | [`dwarf_lines`](#dwarf_lines) |
| want each instruction annotated with its dwarf source line | [`dwarf_lines`](#dwarf_lines) |
| c++ member functions decompile with a0/a1 parameters despite -g debug info | [`cppproto`](#cppproto) |
| Foo *this renders as void * | [`cppproto`](#cppproto) |
| a namespaced or templated function loses its dwarf prototype | [`cppproto`](#cppproto) |
| one unmappable parameter type discards the whole dwarf signature | [`cppproto`](#cppproto) |
| a dwarf-typed local or parameter renders void * despite -g debug info | [`typedepth`](#typedepth) |
| const char ** or char *const [] loses its element type | [`typedepth`](#typedepth) |
| a typedef'd struct pointer degrades to void * | [`typedepth`](#typedepth) |
| a global array of string pointers is marked one byte wide | [`typedepth`](#typedepth) |
| deep pointer chains (char ***) truncate to void ** | [`typedepth`](#typedepth) |
| c++ member functions on a stripped binary decompile with a0/a1 parameters | [`cppsig`](#cppsig) |
| this renders as int8 * or unsigned long instead of the class type | [`cppsig`](#cppsig) |
| a mangled symbol names the function but not its parameter types | [`cppsig`](#cppsig) |
| no signature recovery on a stripped c++ shared library | [`cppsig`](#cppsig) |
| mcount/__fentry__ profiling calls clutter every -pg function prologue | [`callfixup`](#callfixup) |
| cspec call-fixup targets rendered as plain calls instead of dissolved | [`callfixup`](#callfixup) |
| absolute function-pointer table in rodata never recognized | [`addrtable`](#addrtable) |
| indirect calls through an unrecovered address table in a stripped target | [`addrtable`](#addrtable) |
| an immediate operand pointing at a rodata string renders as a bare number | [`operand_refs`](#operand_refs) |
| string argument of a no-prototype call not typed as char[N] | [`operand_refs`](#operand_refs) |
| printf/scanf variadic arguments render untyped at the call site | [`formatstring`](#formatstring) |
| %d and %s arguments carry generic types instead of int/char * | [`formatstring`](#formatstring) |
| noreturn discovery inert on a stripped binary | [`listing`](#listing) |
| analysis passes that need whole-image xrefs do nothing | [`listing`](#listing) |
| no program-wide instruction/xref/function model for consumer passes | [`listing`](#listing) |
| fast decompile-project emits only import veneers and the binary entry | [`fast_funcdisc`](#fast_funcdisc) |
| internal direct callees have no project bodies | [`fast_funcdisc`](#fast_funcdisc) |
| function reachable through a callback or vtable pointer is absent while explicit --addr decompiles it | [`fast_funcdisc`](#fast_funcdisc) |
| stripped static-linked library function stays sub_<addr> although its fingerprint is known | [`fid`](#fid) |
| no .fid database renames applied in a stripped binary | [`fid`](#fid) |
| msvc c++ class names missing and vftables left as unnamed data | [`rtti`](#rtti) |
| no Class::vftable or RTTI_ labels in a windows pe with polymorphic classes | [`rtti`](#rtti) |
| gcc/clang c++ class names missing and vtables left as unnamed DAT_ data | [`itaniumrtti`](#itaniumrtti) |
| no Class::vtable or typeinfo labels in a stripped elf with polymorphic classes | [`itaniumrtti`](#itaniumrtti) |
| virtual dispatch slot resolves to sub_<addr> with no class | [`itaniumrtti`](#itaniumrtti) |
| function reachable only through a rodata function-pointer table never discovered | [`aif`](#aif) |
| undefined gap between functions that clearly holds code | [`aif`](#aif) |
| call *reg targets missing from the function list | [`aif`](#aif) |
| a stripped binary reports far more functions than it has | [`aifstrict`](#aifstrict) |
| a discovered function starts in the middle of another functions body | [`aifstrict`](#aifstrict) |
| a sub_ entry lands on an ordinary call or mov instruction rather than a prologue | [`aifstrict`](#aifstrict) |
| two overlapping functions decompile the same instructions twice | [`aifstrict`](#aifstrict) |
| aif adds thousands of entries a reference decompiler does not agree are functions | [`aifstrict`](#aifstrict) |
| an arm function entry lands one halfword before the real prologue | [`aifstrict`](#aifstrict) |
| routine reached only by a tail B produces no decompiled output at all | [`tailcallentry`](#tailcallentry) |
| a shared ISR handler is swallowed by the two-instruction stub that branches to it | [`tailcallentry`](#tailcallentry) |
| function entry missing where the previous function ends in an unconditional branch | [`tailcallentry`](#tailcallentry) |
| kuna emits one oversized function where the ground truth has two | [`tailcallentry`](#tailcallentry) |
| stripped go binary renders sub_<addr> instead of main.main and runtime.* names | [`gopclntab`](#gopclntab) |
| go package function names missing | [`gopclntab`](#gopclntab) |
| objective-c methods render as sub_<addr> instead of -[Class sel] | [`objc`](#objc) |
| __objc_ metadata present but classes and selectors unnamed in a mach-o | [`objc`](#objc) |
| stripped windows pe functions stay FUN_<addr> although a matching .pdb exists | [`pdb`](#pdb) |
| pdb symbol names not applied to a pe | [`pdb`](#pdb) |
| a .o relocatable object fails with 'Unable to load N bytes at ...' | [`relocobjects`](#relocobjects) |
| ET_REL object maps zero bytes so nothing decompiles | [`relocobjects`](#relocobjects) |
| a COFF .obj lists only its first function (the rest collide at address 0) | [`relocobjects`](#relocobjects) |
| arm64e mach-o decoded with the generic v8A spec so pointer-auth ops are unmodeled | [`macho-arm64e`](#macho-arm64e) |
| pac instructions in an apple-silicon binary not modeled by the loaded spec | [`macho-arm64e`](#macho-arm64e) |
| comparison constant off by one versus upstream ghidra (x <= 9 vs x < 10) | [`compareform`](#compareform) |
| need the analysis-canonical compare form to diff against upstream ghidra output | [`compareform`](#compareform) |
| &base[index] rendering unwanted; consumer expects raw pointer arithmetic | [`arraynotation`](#arraynotation) |
| indexed array form where base + offset arithmetic is desired for diffing | [`arraynotation`](#arraynotation) |
| if (x != 0) where idiomatic C reads if (x) | [`truthycond`](#truthycond) |
| if (p == NULL) where if (!p) is wanted | [`truthycond`](#truthycond) |
| explicit zero comparison in conditions unwanted | [`truthycond`](#truthycond) |
| truthy condition rendering needs to match upstream ghidra for diffing | [`truthycond`](#truthycond) |
| if (x) { stmt; } where braceless if (x) stmt; is wanted | [`braceelide`](#braceelide) |
| single-statement if body keeps braces | [`braceelide`](#braceelide) |
| braced one-liner bodies unwanted | [`braceelide`](#braceelide) |
| brace style needs to match upstream ghidra for diffing | [`braceelide`](#braceelide) |
| full-line WARNING banner comments clutter the output | [`warnstyle`](#warnstyle) |
| warning text wanted inline at the end of the statement it describes | [`warnstyle`](#warnstyle) |
| terse warning slugs unwanted; full upstream warning text needed | [`warnstyle`](#warnstyle) |
| warning style needs to match upstream ghidra for diffing | [`warnstyle`](#warnstyle) |
| thumb function pointer renders as symbolic &fn[1] where the raw odd-address constant is wanted | [`thumbfuncptr`](#thumbfuncptr) |
| callback constant on arm resolves to a function symbol plus one instead of a bare hex literal | [`thumbfuncptr`](#thumbfuncptr) |
| a bare constant equal to a function entry renders as the named function pointer | [`inferfuncentry`](#inferfuncentry) |
| hex literal at a single-bit image base unexpectedly named as a function (flip off for the raw constant) | [`inferfuncentry`](#inferfuncentry) |
| want the raw (b<<k) s>> k sign-extension shift idiom visible instead of the folded comparison | [`booleanmask`](#booleanmask) |
| flag-as-high-bit lowering (8051 style) hidden by the cleaned boolean compare | [`booleanmask`](#booleanmask) |
| want the raw S/OV overflow-flag arithmetic visible instead of the folded signed compare | [`ovlesssimplify`](#ovlesssimplify) |
| v850-style explicit flag compare collapsed to a clean signed less-than | [`ovlesssimplify`](#ovlesssimplify) |
| want the raw CONCAT11(CARRY1(...)) intrinsics visible instead of one wide add | [`addcarrychain`](#addcarrychain) |
| 8-bit adc carry-chain pair (6502 style) folded into a single 16-bit addition | [`addcarrychain`](#addcarrychain) |
| boolean shifted into the sign bit tested with a less-than-zero compare | [`flagcompare`](#flagcompare) |
| N==V signed-overflow branch idiom rendered as raw flag arithmetic | [`flagcompare`](#flagcompare) |
| want the explicit condition-flag math visible (flip off) | [`flagcompare`](#flagcompare) |
| &pxVar[-0x1000] page-probe noise in a large-frame function | [`stackprobeloop`](#stackprobeloop) |
| calls rendered argument-less because the stack pointer never resolved to a constant offset | [`stackprobeloop`](#stackprobeloop) |
| gcc stack-clash probe loop leaves the frame layout unrecovered | [`stackprobeloop`](#stackprobeloop) |
| decompilation aborts with 'Unable to find unique hash for varnode' | [`dynamichashmax`](#dynamichashmax) |
| dense unrolled simd/neon loop (aarch64, go) fails to decompile at symbol mapping | [`dynamichashmax`](#dynamichashmax) |
| loop walks an array with a raw offset accumulator (iVar += 0x414) instead of an index | [`arraystride`](#arraystride) |
| strided induction offset never re-expressed as counter*stride array indexing | [`arraystride`](#arraystride) |
| malformed do/while whose body holds an out-of-place constant assignment | [`condexeplace`](#condexeplace) |
| spurious = 0 copy materialized inside a loop block | [`condexeplace`](#condexeplace) |
| function aborts with 'Cannot properly adjust input varnodes' | [`inputvarnodeadjust`](#inputvarnodeadjust) |
| overlapping stack parameters (mc68k link/unlk) kill the whole decompilation | [`inputvarnodeadjust`](#inputvarnodeadjust) |
| an x86 Windows function renders as (void) though it takes arguments | [`evalcurrentproto`](#evalcurrentproto) |
| a local carrying a // ecx or // edx storage comment is read before it is ever written | [`evalcurrentproto`](#evalcurrentproto) |
| __fastcall/__thiscall arguments missing from the signature | [`evalcurrentproto`](#evalcurrentproto) |
| calls pass fewer arguments than the callee reads | [`evalcurrentproto`](#evalcurrentproto) |
| call rendered with exactly six arguments on x86-64 | [`callsitestackargs`](#callsitestackargs) |
| call rendered with an empty argument list on i386 | [`callsitestackargs`](#callsitestackargs) |
| a for loop whose exit variable is written through a pointer argument disappears | [`callsitestackargs`](#callsitestackargs) |
| ternary guards feeding a logging call are dead-code eliminated | [`callsitestackargs`](#callsitestackargs) |
| outgoing-argument stack slots render as locals commented stack - 0xNN | [`callsitestackargs`](#callsitestackargs) |
| call emitted with no arguments or fewer arguments than the disassembly passes | [`calloverlap`](#calloverlap) |
| call result read from a local that is never assigned | [`calloverlap`](#calloverlap) |
| enclosing function loses its own parameter list and return type (void f(void)) | [`calloverlap`](#calloverlap) |
| argument register written whole-width by pxor movdqa movdqu por pand orpd or movq before the call | [`calloverlap`](#calloverlap) |
| second call binds a stale pre-call value where the first call should have defined it | [`calloverlap`](#calloverlap) |
| stack array subscripted past its own declared bound under a guard that proves the larger index | [`loadguardrange`](#loadguardrange) |
| stack scalar at the end of an array read but never assigned and never address-taken | [`loadguardrange`](#loadguardrange) |
| array written through a computed index capped at 4 elements regardless of the real extent | [`loadguardrange`](#loadguardrange) |
| decompile-project output recompiles into a stack overflow the binary does not have | [`loadguardrange`](#loadguardrange) |
| servo/config fields validated and stored from uninitialized stack locals | [`loadguardrange`](#loadguardrange) |
| iVar1/uVar2/param_1 ghidra-style names wanted instead of v1/a1 (set ghidra) | [`namestyle`](#namestyle) |
| v-numbered locals and sub_/dat_/label_ names in the default output | [`namestyle`](#namestyle) |
| byte-for-byte comparison against upstream ghidra naming | [`namestyle`](#namestyle) |
| undefined8/xunknownN placeholder types wanted for upstream comparison (flip off) | [`realtypes`](#realtypes) |
| size-guessed unsigned long/int/char types shown for values the inference never typed | [`realtypes`](#realtypes) |
| int4/uint1/uint4/float8/float10 appear in the emitted C instead of C type names | [`ctypes`](#ctypes) |
| the same function mixes `unsigned int` with `int4` | [`ctypes`](#ctypes) |
| `code *` appears as a function-pointer type | [`ctypes`](#ctypes) |
| the emitted C does not compile because its type names are not C | [`ctypes`](#ctypes) |
| the same local declared once although many HighVariables share the stack slot | [`dedupvardecls`](#dedupvardecls) |
| flip off to see one declaration line per high (e.g. int4 option_index repeated hundreds of times) | [`dedupvardecls`](#dedupvardecls) |

## Toggleable transforms

The control surface: each of these can make output worse on the wrong source shape, so each stays flippable.

### `memsetrecover` -- on | off, default `on`

- **Symptoms:** long run of per-element constant stores instead of a single memset; unrolled or SIMD zeroing rendered as dozens of assignments; builtin_memset call where the individual element stores are wanted (flip off).
- **What it does:** Collapse a run of constant-fill stores (incl. inlined/SIMD bzero) into a single builtin_memset.
- **When to flip:** Flip off only to see the individual element stores; on (default) reconstructs the memset.
- **Where / provenance:** P5/constsequence · ghidra-upstream · correctness-fix · GH-9230/1537
- **Example:** `option memsetrecover off`

### `returnpair` -- pair | single, default `pair` (destructive opt-in)

- **Symptoms:** spurious CONCAT44 return value in a void or single-register function; two return registers joined into one double-width return (e.g. sparc o0/o1); return type twice the natural width built from a register concat.
- **What it does:** Whether a passively-active second return register may be joined into a wide return (pair) or dropped (single).
- **When to flip:** Set single PER FUNCTION when a void/single-register function shows a spurious CONCAT44 return (e.g. SPARC); DESTRUCTIVE as a global default (truncates real multi-register returns).
- **Where / provenance:** P4/trial-budget · ghidra-upstream · opt-in-tool · GH-6990
- **Example:** `option returnpair single`

### `v850indirectbranch` -- on | off, default `off` (destructive opt-in)

- **Symptoms:** v850 jmp [reg] rendered as an indirect call instead of a recovered switch; jump table on v850 never recovered because the dispatch decodes as a computed call.
- **What it does:** Reclassify a V850 jmp [reg] CALLIND to BRANCHIND so switch-table recovery runs.
- **When to flip:** Set on PER V850 PROGRAM to recover jump-table switches; DESTRUCTIVE as a global default (matches register-indirect calls on other architectures).
- **Where / provenance:** P2/flow-classification · ghidra-upstream · opt-in-tool · GH-8817
- **Example:** `option v850indirectbranch on`

### `msvcftol` -- on | off, default `on`

- **Symptoms:** __ftol() / __ftol2() called with no arguments; the float computation feeding a float-to-int conversion is missing from an MSVC x86-32 binary; a __thiscall method lost its `this` pointer and reads its fields off a stack argument instead.
- **What it does:** Lower the MSVC x86-32 float-to-integer CRT helpers (`__ftol`, `__ftol2`, `__ftol2_sse`) to p-code via a synthesized `<callfixup>`, so the conversion prints as a C cast instead of an argument-less `__ftol()` call. MSVC passes the value in the x87 stack top ST0 and returns the __int64 in EDX:EAX, but no vendored x86 prototype model has an `<input>` pentry naming an x87 register (ST0 appears only as an `<output>` pentry), so the call characterizes ST0 as NoContainment, no argument trial is created, and the feeding FLD chain is dead-code eliminated together with every register it was based on -- including the ECX `this` pointer of a __thiscall method. The fixup body pops the CALL's pushed return address, truncates ST0 at the full 64-bit width into EDX:EAX, and pops the x87 stack.
- **When to flip:** An x86-32 MSVC binary shows bare `__ftol()` / `__ftol2()` calls with no arguments and the floating-point computation that fed them is missing (often together with a lost `this` pointer). On by default; flip OFF to restore the un-fixed call rendering.
- **Where / provenance:** P2/flow-classification · ida · correctness-fix · kuna-msvcftol
- **Example:** `option msvcftol off`

### `tailcalljump` -- on | off, default `on`

- **Symptoms:** leaf function ends in a (*dat_...)(...) computed call with a 'Treating indirect jump as call' warning; jmp to a plt stub inlined into the caller instead of a named tail call; plt thunk body absorbed where func(...) is expected.
- **What it does:** Recover an -O2 tail jump (a direct `jmp` to another function's entry, e.g. `jmp setlocale@plt`) as a tail call (CALL + RETURN) so the callee resolves by name and its return value flows out, instead of flowing into the callee (which inlines a PLT thunk and mis-renders it as a `(*dat_...)(...)` indirect call with a 'Treating indirect jump as call' warning). When it fires it logs a `tailcalljump: recovered tail call` warning at the branch site so the introduced call is observable.
- **When to flip:** A leaf function ends in `jmp <func>@plt` and kuna would emit `(*dat_...)(...)` + a 'Treating indirect jump as call' marker instead of `func(...)`. On by default (DIV-14) = the named call plus a `tailcalljump: recovered tail call` WARNING; flip OFF to restore the upstream flow-into-callee rendering (the two affected datatests, Long double #1/#2, opt out per-test).
- **Where / provenance:** P2/flow-classification · angr · structure-recovery · angr-tee-O2-tail-jumps
- **Example:** `option tailcalljump on`

### `funcboundflow` -- on | off, default `on`

- **Symptoms:** a function's tail is really the body of the NEXT function; dead/garbage code after a call to a die()/fatal()/throw wrapper that never returns; two adjacent functions merged into one, the second also decompiled on its own; uninitialized reads in the tail after an unrecognized no-return call.
- **What it does:** REMOVES CODE: bound a function's fall-through at the entry of another known function. A kuna FunctionSymbol has no extent and CALL is fall-through, so a function whose last act is a `call` to a no-return routine kuna cannot prove no-return (in a stripped static binary: the unnamed `exit`/`abort`/`__stack_chk_fail` bodies and the app-level `die()`/`throw` wrappers built on them) runs its flow past the compiler's inter-function padding straight into the next function's entry and decodes THAT function's body into itself (the merge bug). When a fall-through reaches a known function entry (`query_call(next).is_some()`), plant a no-return artificial RETURN and stop, emitting a `funcboundflow` truncation WARNING. Excludes the function's own entry.
- **When to flip:** A function shows a garbage tail that is really the body of the FOLLOWING function (dead code after a call to a die()/throw/exit wrapper, uninitialized reads, a second unrelated function inlined after an error call). On by default; flip OFF to restore the upstream flow-into-the-next-function behavior.
- **Where / provenance:** P2/flow-classification · ida · correctness-fix · interp-bee-func-merge
- **Example:** `option funcboundflow off`

### `cleanupcode` -- on | off, default `on` (destructive opt-in)

- **Symptoms:** core::ptr::drop_in_place(...) calls all over a decompiled rust function; drop glue and deallocation calls that are nowhere in the rust source; Drop::drop / __rust_dealloc / RawVecInner::deallocate noise at every scope exit; rust output is twice as long as the source because of automatic resource management; flip off to see when a rust value is actually dropped or freed.
- **What it does:** REMOVES CODE: delete the Rust drop/deallocate call sites. Rust emits a drop-glue call at every scope exit and every `?` early return, so decompiled Rust is dominated by `core::ptr::drop_in_place<...>(v)` lines that appear nowhere in the source and carry no meaning for a reader. A direct CALL whose recovered callee name normalizes to one of `core::ptr::drop_in_place`, `core::ops::drop::Drop::drop`, `alloc::raw_vec::RawVecInner::deallocate`, `__rust_dealloc` or `__rustc::__rust_dealloc` is destroyed in the PRE-SSA window (the top of mainloop, gated on heritage pass 0), which is the point of the timing: the register/stack writes that existed only to set up the drop's arguments lose their last reader and are collected by the ordinary ActionDeadCode fixpoint, so the argument setup goes with the call instead of being left behind as unexplained assignments. Normalization reproduces the Oxidizer matcher: un-escape the legacy rustc `$LT$`/`$GT$`/`$u20$` mangling, then collapse the innermost angle-bracket group repeatedly - deleting a plain generic argument list and replacing a `<T as Trait>` qualified path with the TRAIT name - so `_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$::drop` matches and `FakeCrypt::fileops::drop_ransom_note` does not. Oxidizer's `free`/`close`/`_close` entries are deliberately NOT carried: kuna's primary corpus is C binaries and deleting `free()` would be a catastrophically wrong answer. The remaining names cannot occur in a C program, which is what makes the pass structurally inert on a C binary and lets it default ON with no compiler-detection channel from the loader.
- **When to flip:** On by default (DIV-81). A Rust binary whose functions are dominated by drop_in_place/dealloc glue that is not in the source. Flip OFF to keep every drop and deallocation call site - e.g. when auditing a use-after-free, reasoning about when a resource is actually released, or matching the binary instruction-for-instruction. Inert on any binary with no Rust drop symbols, so C/C++ output is byte-identical either way.
- **Where / provenance:** P2/flow-classification · oxidizer · presentation-default · oxidizer-CleanupCodeRemover
- **Example:** `option cleanupcode off`

### `noreturn_extern` -- on | off, default `on`

- **Symptoms:** garbage code after a call to __stack_chk_fail or abort in a relocatable .o; function balloons by swallowing the next function after an extern no-return call; dead fall-through after exit or assert-fail in an object file.
- **What it does:** REMOVES CODE: marks a matched extern call no-return, so code after the call (the fall-through) is dropped as unreachable. Treat a direct CALL whose resolved callee display name matches a known ELF no-return name (`__stack_chk_fail`, `abort`, `exit`, `__assert_fail`, `pthread_exit`, `longjmp`, `_Unwind_Resume`, the C++ terminate/throw family, ...) as no-return at decompile flow time - planting the artificial halt so flow stops at the call - even when the address-keyed no-return flag is unset. This is the undefined-extern case the analysis-tier `noreturn_known` pass cannot reach: in an ELF relocatable object (ET_REL `.o`) a libc no-return is an `UND` symbol with no definition/address, so the address-keyed pass never marks it, and flow runs off the function's end into the next function.
- **When to flip:** On by default (DIV-14). Flip OFF only to restore the upstream fall-through past a matched extern no-return call (e.g. to study the raw flow). On = a direct call to a known ELF no-return name is treated as no-return, so the function is bounded at it and the post-call dead code is dropped (e.g. an ET_REL `.o` tail.o::tail_bytes renders 87 lines instead of ballooning to 326 lines / 9 gotos by swallowing adjacent `tail_lines`/`die_pipe` after `__stack_chk_fail()`). The name match overlaps `noreturn_known`'s for defined/imported symbols; default-on is byte-identical over the 675 datatests (no datatest call resolves to a no-return name).
- **Where / provenance:** P2/flow-classification · angr · correctness-fix · angr-tail-tail-bytes-ret-dup
- **Example:** `option noreturn_extern on`

### `switchmodbound` -- on | off, default `off` (destructive opt-in)

- **Symptoms:** switch reports 'Could not recover jumptable ... Too many branches' and renders as a computed call; indirect jump bounded only by a modulo or and-mask on its index never becomes a switch.
- **What it does:** Bound a LOAD-table jumptable by a modulo (index % N) or and-mask on its index when no guard bounds it.
- **When to flip:** Set on PER PROGRAM when a switch reports 'Could not recover jumptable ... Too many branches' and renders as a computed call; DESTRUCTIVE as a global default (may over-bound an unrelated indirect jump).
- **Where / provenance:** P2/switch-model · ghidra-upstream · opt-in-tool · GH-9191
- **Example:** `option switchmodbound on`

### `switchguardbound` -- on | off, default `off` (destructive opt-in)

- **Symptoms:** 'Too many branches' jumptable failure where a cbranch range guard bounds the index; computed (code *)() call at a gcc sub/ja guarded dispatch with the index spilled to the stack.
- **What it does:** Bound a LOAD-table jumptable by an out-of-band CBRANCH range guard (idx LEu N) when the basic model's guard analysis could not (e.g. a GCC `sub LOW; ja DEFAULT` dispatch with the index spilled to the stack and the flag idiom not yet simplified).
- **When to flip:** Set on PER PROGRAM when a switch reports 'Could not recover jumptable ... Too many branches' and renders as a computed (code *)() call AND the dispatch is range-guarded (vs the modulo form switchmodbound handles); DESTRUCTIVE as a global default (the guard-to-index correspondence is asserted across a stack spill it cannot prove in dataflow, so it may over-bound an unrelated indirect jump).
- **Where / provenance:** P2/switch-model · angr · opt-in-tool · angr-missing-function-call
- **Example:** `option switchguardbound on`

### `switchsharedcase` -- on | off, default `on` (destructive opt-in)

- **Symptoms:** getopt-style switch inside a loop degrades to a computed call with goto spaghetti; pic relative-offset jump table with a loop-carried base register never recovered; 'Too many branches' on a dispatch whose table base is set before the loop.
- **What it does:** Recover a GCC PIC relative-offset jump table (target = base + sext(load4(base + idx*4))) whose table-base register is a loop-carried MULTIEQUAL: the `lea .rodata` base is set before a getopt-style loop while the BRANCHIND is inside it, so the path-meld collapses to the final base+offset add and the CBRANCH range guard on the load index never bounds the table. Rebuilds the meld as a clean single path down to the guarded index so the table resolves and the switch structures. On by default (DIV-14): SLOWER on the functions whose loop-carried switch it recovers (the extra meld-rebuild + table resolution walk runs there), but kept on for the better recovery quality; byte-identical (0/675) on every datatest, which has no such loop-carried PIC switch.
- **When to flip:** On by default (DIV-14): recovers a getopt/dispatch switch inside a loop that would otherwise report 'Could not recover jumptable ... Too many branches' and render as a computed (code *)() call with goto spaghetti. Flip OFF only if the heuristic mis-sizes or accepts an unrelated indirect jump on a specific program (it reads a read-only table at a base it recovers across a loop phi) — it is marked destructive for that reason, but it is byte-identical over the 675 datatests and kept on for quality despite being slower on the functions it recovers.
- **Where / provenance:** P2/switch-model · angr · opt-in-tool · angr-switch-case-shared-case-node
- **Example:** `option switchsharedcase on`

### `switchmultipred` -- on | off, default `on` (destructive opt-in)

- **Symptoms:** image-base-relative switch whose bound guard is duplicated across multiple predecessors renders as a computed call; 'Too many branches' where the dispatch block has several guarded incoming paths; msvc memmove small-count tail dispatch truncated to an indirect call.
- **What it does:** Recover an image-base-relative jump table (target = imagebase + load4(table + idx*4)) whose bound guard is 'unrolled' / duplicated across MULTIPLE predecessors of the dispatch block: the BRANCHIND's parent has sizeIn() > 1, each incoming path ending in its OWN (often semantically different) bound CBRANCH, the per-path switch indices meeting in a MULTIEQUAL. The single-predecessor guard walk short-circuits to checkUnrolledGuard, so no dominating guard bounds the table and the dispatch is truncated to a computed (code *)() call. When on, finds the dispatch MULTIEQUAL, evaluates each predecessor guard as a function of its MULTIEQUAL input, and re-bounds the table to [0, max-over-paths) so the readonly-LOAD table model recovers the switch (angr's 'abnormal switch case').
- **When to flip:** Set on PER PROGRAM when an image-base-relative switch (e.g. an MSVC memmove/memcpy small-count tail dispatch) reports 'Could not recover jumptable ... Too many branches' and renders as a computed (code *)() call AND the dispatch block has multiple predecessors each with its own bound guard (vs the single-guard form switchguardbound handles); DESTRUCTIVE as a global default (it unions per-path guard prefixes across a MULTIEQUAL and re-binds the table from a heuristic max bound, so it may over-bind an unrelated multi-entry indirect jump).
- **Where / provenance:** P2/switch-model · angr · opt-in-tool · angr-abnormal-switch-case-case3
- **Example:** `option switchmultipred on`

### `unrolledguard` -- on | off, default `off` (destructive opt-in)

- **Symptoms:** one switch recovered but sibling interleaved jump tables degrade to computed calls; 'Could not find op at target address' during recovery of an msvc optimized memcpy dispatch; duff's-device tail dispatch rendered as 'Treating indirect jump as call'.
- **What it does:** Recover the interleaved jump tables of an MSVC optimized memcpy/memmove (Duff's device) at the partial-flow stage. The function holds several image-base-relative jump tables whose case bodies are reachable only as one another's case targets; kuna recovers them one at a time, each in its OWN fresh partial-flow clone. A later table's clone re-clones an already-recovered sibling table into its jumpvec, and that partial's collectEdges then calls target() on a sibling case body that was never decoded into THIS partial's visited snapshot (it is only decoded into the parent flow after the recovery pass returns), throwing 'Could not find op at target address' so the dispatch is truncated to a computed (code *)() call. Upstream avoids this by building one shared partial and running collectEdges once while the sibling tables are still empty. When on, the partial-clone collectEdges SKIPS an unresolvable recovered-table case-target edge (the same 'assume no branches out' shape the findJumpTable==0 partial path already uses) instead of throwing, so the interleaved tables all recover and the dispatches render as switches (angr's optimized memcpy).
- **When to flip:** Set on PER PROGRAM when a function holding several interleaved jump tables (e.g. an MSVC optimized memcpy/memmove Duff's-device tail dispatch) recovers ONE switch but degrades the OTHERS to 'Treating indirect jump as call' computed (code *)() calls, and the failures are flow-stage 'Could not find op at target address' on a sibling table's case body (NOT a guard-bound failure switchguardbound/switchmultipred would handle); DESTRUCTIVE as a global default (it suppresses an unresolved-case-target edge during partial-flow recovery, so on an unrelated truly-malformed table it could mask a real missing target instead of declining the table).
- **Where / provenance:** P2/switch-model · angr · opt-in-tool · angr-optimized-memcpy-6301a9
- **Example:** `option unrolledguard on`

### `noreturn_externmatch` -- on | off, default `on`

- **Symptoms:** dozens of garbage *v = *v + c; lines after a __stack_chk_fail call in a .o; inter-function alignment padding decoded as add [rax],al style instructions; flow runs past an undefined-extern abort or exit call in a relocatable object.
- **What it does:** Treat a direct CALL whose callee NAME matches the vendored ELF known-no-return list (exit/abort/__stack_chk_fail/...) as no-return at the flow `query_call_no_return` seam, even when the address-keyed noreturn_known scan emitted no fact. Closes the ET_REL `.o` gap: an undefined extern has symtab address()==0 and no PLT, so noreturn_known never marks it, and flow runs past a `__stack_chk_fail()` call into the inter-function alignment padding (00 00 -> `add [rax],al`), emitting dozens of garbage `*v = *v + c;` lines.
- **When to flip:** On by default (DIV-13): applies the SAME vendored name list and global/std namespace guard as the already-default-on noreturn_known, just at the flow query seam to reach the ET_REL `.o` undefined extern the address-keyed scan structurally misses; a no-op on a normal dynamically-linked ELF (the proto flag is already set). Set OFF to restore the prior byte-identical rendering (dead `add`-padding after a `__stack_chk_fail`/`abort`/`exit` call reappears).
- **Where / provenance:** P2/flow-follow · angr · correctness-fix · angr-incorrect-duplication-chcon
- **Example:** `option noreturn_externmatch off`

### `stackalias` -- on | off, default `off` (destructive opt-in)

- **Symptoms:** spurious uninitialized local (xStack_N) returned after storing through a pointer to a local; store through a take-address-of-local pointer dropped as dead so the later read is garbage.
- **What it does:** Hold a store-through-a-stack-pointer-alias (int *p=&x; *p=x; return *p) alive across the deadcode race so it is not dropped to an uninitialized stack read.
- **When to flip:** Set on PER FUNCTION when a take-address-of-local + store-through-pointer returns a spurious uninitialized local (xStack_*); DESTRUCTIVE as a global default (conservatively pins stack stores live, suppressing legitimate dead-store removal).
- **Where / provenance:** P6/alias-facets · ghidra-upstream · opt-in-tool · GH-8500
- **Example:** `option stackalias on`

### `sparcstructret` -- on | off, default `off` (destructive opt-in)

- **Symptoms:** bogus (*pcVar1)() indirect call after calling a struct-returning function on sparc; function tail dropped after a sparc call followed by unimp.
- **What it does:** Let the SPARC struct-return `unimp` after a call fall through instead of becoming a non-returning indirect call.
- **When to flip:** Set on PER SPARC PROGRAM when a struct-returning callee makes a function render a bogus (*pcVar1)() and drop its tail; DESTRUCTIVE as a global default (an IllegalInstructionTrap-fed BRANCHIND elsewhere is suppressed).
- **Where / provenance:** P2/flow-classification · ghidra-upstream · opt-in-tool · GH-6882
- **Example:** `option sparcstructret on`

### `loweredswitch` -- on | off, default `on`

- **Symptoms:** switch rendered as nested if/else-if compare tree over one variable; binary-search cascade of constant compares where source had a switch; flip off to keep the compiler's lowered compare-tree rendering.
- **What it does:** Reconstruct a compiler-lowered comparison cascade (a GCC binary-search if/else tree over one variable) back into a switch with a synthesized BRANCHIND + jump-table (the S2 artifact Ghidra renders switches from).
- **When to flip:** On by default (DIV-4); the required binary-search-structure guard (a range/jle split) keeps it off hand-written linear if/else-if chains. Set OFF to restore the upstream if/else-if rendering of a lowered switch.
- **Where / provenance:** P2/switch-model · angr · structure-recovery · angr-LoweredSwitchSimplifier
- **Example:** `option loweredswitch off`

### `spillargtrial` -- off | reload | spill, default `off`

- **Symptoms:** call rendered with fewer arguments than the disassembly passes when the missing argument register is spilled to the frame before the call; floating-point argument dropped at the FIRST of several calls that reuse the same value; a spurious 4-byte stack local appears where a call argument should be; clang -O2 packed-double code calls atan2 with one argument instead of two; argument register stored to a caller-frame slot and reloaded after the call is not recognized as an argument.
- **What it does:** Tolerate the caller's own caller-save spill of an argument when scoring an active input trial. Funcdata::onlyOpUse (funcdata_varnode.cc:1891) answers whether the Varnode reaching a CALL in a candidate parameter register is used ONLY as that call's argument, and it rejects on ANY CPUI_STORE among the Varnode's descendants; checkInputTrialUse then calls markInactive, which also sets CHECKED, so the trial is never re-scored and the argument-producing chain dies to dead-code elimination. That blanket rejection also rejects the opposite idiom: on x86-64 SysV NO xmm register is callee-saved, so a floating-point value that is both an argument and live across the call MUST be spilled by the caller, and the spill is a second descendant of the very Varnode the trial is scoring. At `reload` the STORE stops rejecting only when it writes the walked Varnode's own value (operand 2, never the pointer) into a caller-frame slot AND a later LOAD reads that slot back at the same width, i.e. a genuine caller-save spill/reload pair. Since ActionActiveParam runs before ActionStackPtrFlow, the frame reference is still the raw INT_ADD(stack pointer, #const) and the reload straddles the call that re-defines the stack pointer, so the search walks forward from the store's own base Varnode carrying the running offset delta rather than matching raw constants. At `spill` the reload requirement is dropped and any caller-frame store of the value is tolerated. This is a deliberate DIVERGENCE from upstream, not a port fix: the rule it relaxes exists precisely to stop a value the caller writes to its frame before a call from becoming a bogus extra argument.
- **When to flip:** Turn ON (`reload`) when a call is rendered with FEWER arguments than the disassembly passes and the missing argument's register is spilled to the frame just before the call -- the canonical rendering is `atan2(SUB168(v1,0));` from clang -O2 packed-double code that calls atan2 more than once, where `movapd XMMWORD PTR [rsp+0x20],xmm1` is the caller-save spill and `movaps xmm1,XMMWORD PTR [rsp+0x20]` after the call is its reload. The dropped argument often leaves a spurious 4-byte stack local in its place (`unsigned int v5; // stack - 0x18`). PREFER `reload` OVER `spill`: `spill` admits any frame store of the value, which fabricates arguments out of ordinary local writes -- measured on a clang -O2 inlined 64-byte memcpy, `spill` turns `sptr2(&v1,a0)` into `sptr2(v1,v5,v9,v13,&v1,a0)`, four arguments invented from the four `movaps` stores that fill the local buffer, while `reload` correctly declines because nothing reads those slots back. Use `spill` only to bisect whether the reload requirement is what is blocking a recovery. DEFAULT OFF: the failure mode of relaxing upstream's rule is a SPURIOUS TRAILING ARGUMENT, and no gate can see one -- the datatest corpus is prototype-declared and GED is blind to arity -- so the option ships as an opt-in with its evidence rather than as a default. Flip back off if a call gains an argument the disassembly does not pass, or if an argument appears whose value is a stale pre-call definition (that second shape is calloverlap's territory: the two options compose, and on the GH-275 copysign variant only `calloverlap full` plus `spillargtrial reload` gives both the right arity and the right values).
- **Where / provenance:** P4/active-input-trial-scoring · kuna · opt-in-tool · GH-275
- **Example:** `option spillargtrial reload`

### `regionstructure` -- on | off, default `on`

- **Symptoms:** goto placement follows the sailr region walk instead of ghidra's collapse order (flip off to force CollapseStructure); uncompilable c where a loop body folded into a while condition; structuring differs from upstream ghidra on the same function.
- **What it does:** Structure the CFG by walking the KunaRegionIdentifier region tree and matching angr Phoenix/SAILR schemas (acyclic sequence + ITE + cyclic loops [Inc 3] + acyclic switch-case [Inc 4], with the SAILR H1/H3-ordered virtualize-to-goto fallback) instead of running Ghidra's CollapseStructure. DIV-12 default-on: the primary structuring path. Byte-identical to Ghidra's CollapseStructure on reducible code; falls back to CollapseStructure on irreducible (multi-entry) loops.
- **When to flip:** On by default (DIV-12): the region-driven structurer is the primary S8 path. It is byte-identical to Ghidra on reducible code and falls back to CollapseStructure when a function cannot be collapsed to a single structured root (irreducible/multi-entry loops). Set OFF to force the upstream Ghidra CollapseStructure path unconditionally.
- **Where / provenance:** P8/goto-quality · angr · structure-recovery · angr-region-structurer
- **Example:** `option regionstructure on`

### `guardarm` -- on | off, default `off`

- **Symptoms:** guard clause with a fatal no-return call comes out inverted and the whole body is nested inside the if; the function's only return sits inside an if-arm and the body ends on a no-return call; kuna negates a guard condition that ida and ghidra leave alone; a leading validation guard is emitted at the tail of the function.
- **What it does:** Break the ARM TIE in CollapseStructure::ruleBlockIfNoExit by code layout instead of by out-index. The rule folds the condition block and ONE terminal out-arm into a BlockIf; the other arm survives as the fall-through, so the pick decides which half of the function is nested inside the `if`. Upstream walks the out-edges in index order and takes the first eligible one (in-degree 1, out-degree 0, a decision edge). Usually only one arm qualifies and there is no choice -- but when the condition guards a fatal no-return call and the other side is the function's own `return`, BOTH arms qualify and index order decides a coin flip. Index order carries no source information at that point: `negateCondition`'s `swapEdges` re-orients the block between the two `collapse_all` runs ActionBlockStructure performs (the pending-flip realization runs between them), so the arm sitting at out(0) in the deciding run is not the arm the branch was assembled with. When the `return` wins the tie kuna emits `if (ok) return v; fatal();` -- condition negated, body re-parented one level deeper, fatal call demoted to the function's trailing fall-through -- where the source, IDA and Ghidra all write `if (!ok) fatal(); return v;`. With this on, a tie is resolved to the arm whose front leaf lies EARLIER in the address space: an unoptimized compiler emits the taken clause of `if (c) A; B;` in front of `B`, and the `then` side of an if/else in front of the `else` side, so the earlier arm is the source's `if` body in both shapes. TIES ONLY -- a block with one eligible arm is byte-identical to upstream, which is why the great majority of guards are untouched. The predicate deliberately does NOT read the artificial-halt (op_mark_halt / pcodeop_flags::noreturn) bit: preferring the no-return arm fixes openssh scp xcalloc and dpkg-query control_list but INVERTS coreutils make-prime-list xalloc, whose source really is `if (p) return p; fprintf(...); exit(1);`. Layout order gets all three right because it reads what the compiler recorded about the source instead of guessing from the callee.
- **When to flip:** Turn ON when a guard clause that ends in a fatal/no-return call comes out INVERTED -- the condition negated, the whole rest of the function nested one level deeper inside the `if`, and the fatal call emitted as the function's last statement -- where IDA, Ghidra and the source all write the flat `if (bad) fatal(); <body>; return K;`. The tell is a function whose only `return` sits inside an if-arm while the body ends on a `// no-return` call. Verified on the decbench O0 corpus: dpkg-query control_list (both of its two guards come out flat; its second guard was already correct and is unchanged), openssh scp xcalloc, coreutils `[` posixtest. Equally verified NOT to move the two functions a no-return-arm preference would break: coreutils make-prime-list xalloc @0x1906 and coreutils tail xlseek @0x4804 are byte-identical with the option on. MEASURED BIDIRECTIONALLY over the whole decbench O0 slice (32,339 functions scored in both arms, 265 slices, `scripts.decbench.optsweep`): 6,821 bodies change, GED 230,291 -> 227,646 (-2,645), GED-perfect 14,785 -> 15,082 (+297), 316 functions moved TO perfect against 19 moved OFF (16.6:1, McNemar z = +16.2), improved 726 / worsened 243. Per architecture the gain is x86-64 (+305 perfect, -2,672 GED); ARM Cortex-M is -7 and i386 PE -1, on 7 and 1 functions respectively. Every one of the 6,821 changed bodies was audited mechanically: identical callee, string and statement multisets, gotos 1,021 -> 1,019, labels 620 -> 619, `while( true )` and `// no-return` counts unchanged -- the only content difference is the comparison constant a negated condition renders (`x <= 0xb` becoming `10 <= x`). TWO KNOWN FAILURE MODES, both in that 19. (1) The metric, not the option: on betaflight cliDumpPrintLinef and zlib test_inflate the ON pane matches the preprocessed source LITERALLY (`if (!((dumpMask & DO_DIFF) && equalsDefault)) {...} return false;` and `if (strcmp(...)) { fprintf; exit(1); } else { printf; }`) and GED still charges 8 and 6. (2) A real limit: when an arm is a return-split CLONE its start address is the SHARED EPILOGUE's, not the source position of the `return`, so the layout signal is destroyed -- e2fsprogs preenhalt's source `if (!(ctx->options & E2F_OPT_PREEN)) return;` inverts. Default OFF: it is a divergence from upstream Ghidra's out-index order, so the flip needs its own DIV row plus the 0/675 ablation and a speed number.
- **Where / provenance:** P8/goto-quality · kuna · structure-recovery · decbench-ifnoexit-guardarm
- **Example:** `option guardarm on`

### `loopcondhoist` -- on | off, default `off`

- **Symptoms:** loop emitted as while( true ) with the head test as a guarded exit on the first statement; a for/while header condition is left inside the loop body; kuna writes while(true) where ida writes while (cond); a loop's early-exit return is the loop's first statement instead of its condition.
- **What it does:** Give the non-loop-heads a first pass in CollapseStructure's DEFERRED ruleBlockIfNoExit scan, so ruleBlockWhileDo keeps the loop's head test. The deferred scan walks the live components in order and folds the FIRST block whose terminal arm qualifies, then restarts the whole cascade. A head-tested loop whose exit arm is a `return` (after ActionReturnSplit/returndup has given that return in-degree 1) is such a block, and it sits ahead of its own body in that order -- so the head folds to `if (!C) return X;` and drops to ONE out-edge, at which point ruleBlockWhileDo can never match it again and the loop is emitted as `while( true ) { if (!C) return X; BODY; }`. The body block that carries the loop's `break` is ALSO a candidate in the same scan and folding it first is strictly better: the break arm absorbs the loop's follower (which the rule's own `size_in() == 1` test already requires be reached from nowhere else), the body collapses to a single back-edge clause, and the head test hoists into the `while`/`for` header on the next cascade pass with no new machinery. With this on the scan makes one pass skipping blocks that are live loop heads (resolved through the collapsed graph exactly as LoopBody::update resolves them) and only falls back to the unrestricted upstream pass when that finds nothing, so a function with no loop-head candidate is byte-identical to upstream and the scan still terminates on the same fixpoint.
- **When to flip:** Turn ON when a loop comes back as `while( true ) {` with the real loop condition immediately inside it as a guarded exit -- `while( true ) { if (!C) return X; BODY; }` -- where the source and IDA write `while (C) { BODY }`. Witnesses: coreutils od read_char becomes `while (dat) { ... }`, and libacl getfacl walk_tree_visited reaches the source's exact `for (v1 = ...; v1 != a0; v1 = ...) { ... } return 0;`. The named hazard is handled by the existing rule rather than by a new predicate: libacl getfacl get_list, whose loop follower FREES the list and returns NULL, keeps its semantics exactly (the free+return moves into the `break` arm and the normal exit returns the list) because ruleBlockIfNoExit only folds a clause of in-degree 1; bash mksyntax main is byte-identical. MEASURED BIDIRECTIONALLY over the whole decbench O0 slice (32,339 functions scored in both arms, 265 slices, `scripts.decbench.optsweep`): 2,281 bodies change, GED 230,291 -> 224,670 (-5,621), GED-perfect 14,785 -> 15,454 (+669), 680 functions moved TO perfect against 11 moved OFF (61.8:1, McNemar z = +25.5), improved 1,347 / worsened 256. EVERY architecture gains: x86-64 +542, ARM Cortex-M +123, i386 PE +4. Over the 2,281 changed bodies the shape audit reads `while( true )` 2,087 -> 223, gotos 621 -> 409, labels 399 -> 251, lines 123,744 -> 118,891, and `// no-return` unchanged. Sixty-six of those functions lose duplicated callees (openssh addr_match_cidr_list emits `free(list)` once instead of five times) -- that is the returndup-cloned loop follower being re-shared as the single loop exit, and the resulting pane is the openssh source verbatim. Default OFF: it changes WHICH of two legal folds happens first, a divergence from upstream's component order, so the flip needs its own DIV row plus the 0/675 ablation and a speed number.
- **Where / provenance:** P8/goto-quality · kuna · structure-recovery · decbench-ifnoexit-loopcondhoist
- **Example:** `option loopcondhoist on`

### `regionlooprefine` -- on | off, default `on`

- **Symptoms:** goto-heavy multi-exit loop where angr recovers a clean while/for with break/continue; irreducible or multi-latch loop falls back to raw gotos instead of folding; secondary loop exits rendered as gotos rather than break statements.
- **What it does:** Extend the region structurer (regionstructure) with cyclic loop-successor refinement: a multi-exit / multi-latch / mid-entry (irreducible) loop that the base cyclic schemas cannot fold is refined by virtualizing its SECONDARY exits and latches to gotos (lowered to break;/continue; by the existing scopeBreak / loop-construction passes) so it folds into a structured while/do-while/for/inf-loop instead of falling back to CollapseStructure. A strict superset of regionstructure's cyclic schemas: a loop the base schemas already fold is untouched, so reducible code stays byte-identical (675/675). Port of angr RegionIdentifier._refine_loop_successors_to_guarded_successors / _ensure_jump_at_loop_exit_ends (the force_loop_single_exit path).
- **When to flip:** A function with a multi-exit or irreducible loop renders goto-heavy where angr recovers a clean while/for with break/continue (e.g. tr_O2.o::parse_str: 37 -> fewer gotos). On by default (DIV-14; requires regionstructure on, its prerequisite); flip OFF to restore regionstructure-without-refinement. Only loops that would otherwise fall back to CollapseStructure are refined; if refinement cannot make a loop foldable it still falls back (never worse than OFF).
- **Where / provenance:** P8/goto-quality · angr · structure-recovery · angr-tr-o2-parse-str
- **Example:** `option regionlooprefine on`

### `regionedgeorder` -- on | off, default `off`

- **Symptoms:** more gotos than angr on a function forced to virtualize several edges; goto cut at an awkward edge where a dominance/post-dominator ordering exposes more structure.
- **What it does:** Improve the region structurer's (regionstructure) last-resort edge-virtualization ORDERING: when no schema applies and the structurer MUST virtualize an edge to a goto, pick the order that minimizes the resulting goto count. Replaces the flat H1 (sibling-count) / H3 (return-edge) + address tiebreak with angr's _last_resort_refinement dominance-tiered bucketing (classify candidate edges into crossing / secondary / other via forward immediate-dominators over the acyclic graph, and virtualize the highest-priority bucket first) and the SAILR _order_virtualizable_edges H2 post-dominator heuristic (prefer the edge whose removal yields the most post-dominators, i.e. the most-linear structure), with the postdom_max_edges (10) / postdom_max_graph_size (50) caps so post-dominator computation stays bounded. Only changes WHICH goto is chosen when the structurer is already forced to virtualize, so on reducible code (where the structurer never virtualizes) output is byte-identical (675/675). Port of angr SAILR phoenix._last_resort_refinement + sailr._order_virtualizable_edges.
- **When to flip:** Set ON (with regionstructure on, its prerequisite) when a goto-heavy function would render with fewer gotos under a smarter virtualization order — i.e. the structurer is forced to virtualize multiple edges and the bucketing/post-dominator order exposes more structure than the address tiebreak. Default OFF: byte-identical to regionstructure's existing H1/H3 + address ordering. The caps bound the post-dominator cost; above them only the H1/H3 + node-order tiebreak applies (no regression).
- **Where / provenance:** P8/goto-quality · angr · structure-recovery · angr-sailr-edge-ordering
- **Example:** `option regionedgeorder on`

### `condfold` -- off | on | wide, default `off`

- **Symptoms:** crossing goto from the else arm back into the then arm's shared clause; goto plus label into the shared body of a guard cascade whose arms reconverge; nested if where angr emits one compound short-circuit condition; second condition block assigns one value or calls something before its test and the fold is declined.
- **What it does:** Relax the ONE gate that makes Ghidra decline a short-circuit fold: the is_complex(orblock) check inside ruleBlockOr (and its verbatim twin try_block_or in the region structurer). BlockBasic::isComplex is a >2-statement readability budget, so a single spill, address computation or extra call parked in front of the second test costs a crossing goto back into the first arm's clause (coreutils tr build_spec_list: `result_idx = &v9; if (es_match(es_00,v11,'=')) goto label_40242e;`) or a goto+label into a guard cascade's shared body (lighttpd server.c::main in the angr corpus binary newbury: 1 goto / 1 label where angr emits 0 / 0). With this on, a complex sibling is accepted when it fits the level's PRINTED-WIDTH BUDGET -- the operand renders at most 5 comma elements at `on` / 9 at `wide`, counted with the printer's own skip rules (not_printed, markers, implied outputs, the bare BRANCH), the terminal CBRANCH included -- AND either of two admission rules takes it. Rule A (bounded prefix): a BlockCopy of ONE basic block, branch-free, comment-free, with at most 1 statement-root call. Rule B (statement shape): every op is a marker, an op with an output, a void CALL/CALLIND, a STORE, or the single terminal CBRANCH (RETURN, a second branch, BRANCHIND, CALLOTHER and no-return calls all decline), with <=2 conservatively-scored statements and <=2 calls per block, no comment, and -- because Rule B also admits a NESTED BlockCondition so a guard cascade can fold -- <=4 condition leaves and <=4 total scored statements at the fold site. The absorbed statements render inside the &&/|| operand as a C comma expression, which the printer already supports (comma_separate). The fold moves no p-code: it re-parents two existing structuring nodes, and C short-circuit plus comma sequencing preserves the original execution paths and order, so predicates that call functions need no purity analysis. Applied at BOTH structuring paths. Port of the angr Phoenix MultiStatementExpression relaxation (_should_use_multistmtexprs / _build_multistatementexpr_statements) under its default MAX_ONE_CALL policy; `on` uses angr's statement threshold (5) and `wide` raises the budget to 9 for BOTH rules, because kuna's printed-statement granularity is finer than angr's AIL for the same block (argument recovery and address-tied copy shadows split one AIL statement into several printed ones). THREE ACCURACY NOTES. (1) Rule A's call cap counts only calls printed as their own comma-chain element: the eligibility walk mirrors the printer, whose implied-output skip necessarily runs before the call test, so a call inlined into the sibling's own condition is not charged and a folded operand can render more than one call (measured: mv -O2 copy_internal at `wide` renders cached_umask(...) counted and fchmod(...) not counted). This is deliberate and is a readability bound, not a soundness bound; Rule B counts every call. (2) Rule B's <=2 SCORED statements is a calc_explicit approximation, NOT a width: a block can score 2 and render 7 comma elements, which is exactly why the printed-width budget above is enforced on top of it. The scored count survives only because the <=4 leaf / <=4 statement cross-leaf caps are expressed in it, so those caps bound the relaxation at each site where it is exercised but do NOT bound the summed printed width of a folded cascade, nor the final condition's global width -- a purely upstream-legal fold (both operands non-complex) can still extend a cascade on the left without consulting them, exactly as it can with the option off. (3) The width is measured on the op list as it stands at structuring time: later passes can add or drop an op, and where a call's stack-effect ops are still live (unprototyped callee) it over-counts, which only ever declines a fold.
- **When to flip:** kuna emits a crossing `if (cond) goto label_X;` from the else arm back into the then arm's clause, or a goto+label into the shared body of a permission/validation guard cascade, or a nested if where angr short-circuits into `A || B` / `A && B` -- because the second condition block also assigns something (a spill, a pointer setup, a call whose result is the test). Default off: byte-identical to upstream (the eligibility precompute is skipped and every gate disjunct is dead). Try `on` first: it is the angr-parity width and it closes the guard-cascade case (newbury main reaches 0 gotos / 0 labels there). Go to `wide` when the sibling is a kuna-noisy block (copy shadows, unrecovered call arguments) that `on` still declines -- that is what tr build_spec_list needs to reach angr goto parity, and what true_ubuntu1804 usage needs for its ifs 9 -> 7 fold. THIS IS NOT A MONOTONE GOTO REDUCER: folding changes which edges the structurer can subsequently match, so individual functions can GAIN a goto even though the aggregate is a clear net win (measured on 2827 functions over 7 real binaries: gotos 2294 -> 2219 at `on` and -> 2191 at `wide` overall; 2 functions gain one -- clientloop.o process_escapes 26 -> 28 at both levels, true_ubuntu1804 usage 1 -> 2 at `wide` only). Flip back off when you prefer a nested if to a comma expression in the condition, when the folded operand reads as too dense (an operand can render two calls when one is inlined into the test), or when you need a kuna advisory comment that the fold can drop: the comment guard declines on any comment buffered at structuring time, but comma_separate suppresses emitCommentGroup, so an advisory produced by a LATER pass can vanish (newbury main emits 4 WARNING lines at off and 3 at on, losing its `WARNING: branchflip:` line). The emitted C is unaffected by that loss.
- **Where / provenance:** P8/goto-quality · angr · structure-recovery · angr-tr-build-spec-list
- **Example:** `option condfold wide`

### `gotoreduce` -- on | off, default `on`

- **Symptoms:** goto to a label that only returns; label_N: return; shared tail after an if; more gotos than angr for the same function.
- **What it does:** After structuring, rewrite `if (cond) goto T` into `if (cond) { <tail> }` when T is a small single-successor basic-block chain ending in `return`, by duplicating that return tail inline (a print-tree copy of the same ops — no p-code cloned). Eliminates the residual goto/label that Ghidra's CollapseStructure must emit for a shared multi-predecessor return tail.
- **When to flip:** kuna emits 1+ gotos to a shared `... return v;` tail that angr renders goto-free (e.g. `if (x) goto label; ... label: return 0;`). On by default (DIV-14); flip OFF to restore the upstream goto/label rendering. On, the bounded (<=3 blocks / <=8 ops, no call/store) return-constant tail is duplicated so the cross-edge becomes a structured early return.
- **Where / provenance:** P8/goto-quality · angr · structure-recovery · angr-ReturnDuplicator
- **Example:** `option gotoreduce on`

### `ifelseflatten` -- on | off, default `on`

- **Symptoms:** else block kept although the matching if branch always returns or aborts; guard-clause body indented one level deeper than the source; redundant else after an early-return arm.
- **What it does:** After structuring, flatten a 3-component `if/else` whose true-clause is statement-terminating (every endpoint returns / aborts / gotos away) by dropping the `else` and re-parenting the former else body as the `if`'s immediate follower (a print-tree edit of the structured `list`/`parent` — no p-code cloned). Mirrors angr's `IfElseFlattener`: `if (c) { ...return } else { body }` becomes `if (c) { ...return } body`, removing the syntactic `else` (and any residual goto/label it carried).
- **When to flip:** kuna keeps an `else { ... }` whose matching `if`-branch always returns/aborts (or ends in a goto), so the `else` is redundant guard-clause noise — the early-return form angr emits. On by default (DIV-14); flip OFF to restore the upstream explicit-`else` rendering. The region structurer already flattens most clean diamonds, so this catches the residual terminating-if/non-terminating-else shapes the structurer (or the CollapseStructure fallback) left with an explicit `else`.
- **Where / provenance:** P8/goto-quality · angr · structure-recovery · angr-IfElseFlattener
- **Example:** `option ifelseflatten on`

### `crossjumprevert` -- on | off, default `on`

- **Symptoms:** goto into the middle of a sibling branch's arm (compiler cross-jumping); shared non-return tail reached by goto from an else-if chain; label planted inside an else block targeted from another branch.
- **What it does:** After structuring, revert compiler cross-jumping (Irreducible Statement Condensing): rewrite `if (cond) goto T` into `if (cond) { <tail> }` when T is a small single-successor *non-return* basic block that falls through to a successor S, by duplicating that cross-jump tail inline (a print-tree copy of the same ops — no p-code cloned). Only fires when the if's own structured fall-through also converges on S (a next_flow_after convergence precondition), so duplication is semantics-preserving. Sibling of gotoreduce (return tails) for the non-return fall-through case.
- **When to flip:** Set on when kuna emits 1+ gotos to a shared, condensed *non-return* tail that angr's CrossJumpReverter renders goto-free (e.g. `else if (c) { f(); goto label; } ... else { label: v = *p; } if (v == 2) ...`). On by default (DIV-14); flip OFF to restore the upstream goto rendering. On, the bounded (one tail block, <=8 printed ops, no STORE, <=1 call) cross-jump tail is duplicated back into the goto source so both paths fall straight through and the goto/label vanish. Each duplication is logged (`crossjumprevert:`). Runs after gotoreduce (last deoptimization).
- **Where / provenance:** P8/goto-quality · angr · structure-recovery · angr-CrossJumpReverter
- **Example:** `option crossjumprevert on`

### `taildup` -- on | off, default `on`

- **Symptoms:** goto to a shared free(p); return; style epilogue; cleanup-call return tail reached via label instead of duplicated inline; flip off when many error checks share one cleanup block (the goto out; idiom).
- **What it does:** After structuring, rewrite `if (cond) goto T` into `if (cond) { <tail> }` when T is a small single-successor basic-block chain ending in `return` that *contains a call* (e.g. a `free(p); return;` epilogue), by duplicating that return tail inline (a print-tree copy of the same ops — no p-code cloned). The third SAILR goto-reduction pass: it fills the gap between gotoreduce (return tail, rejects ALL calls) and crossjumprevert (non-return fall-through tail) — neither handles a return tail with a call. Mirrors angr ReturnDuplicatorBase whose budget is calls (max_calls_in_regions=2), not call-rejection.
- **When to flip:** kuna emits 1+ gotos to a shared `... free(p); return;` style epilogue (a return tail WITH a call) that angr renders goto-free by duplicating it into each arm (e.g. morton my_message_callback: 1 goto -> 0). On by default (DIV-14); flip OFF to restore the upstream merged rendering — right when many early-exit error checks share ONE cleanup epilogue (the C `goto out;` idiom, e.g. openssh kex_choose_conf), where duplication forks the cleanup into source-absent copies. On, the bounded (<=3 blocks / <=12 printed ops / <=2 calls, no STORE) return-call tail is duplicated so the cross-edge becomes a structured early return. Safe because the tail ends in `return`: the duplicated call is reached on exactly one structured path (the early-return arm), never both. Each duplication is logged (`taildup:`). Runs right after gotoreduce.
- **Where / provenance:** P8/goto-quality · angr · structure-recovery · angr-ReturnDuplicatorLow-call-tail
- **Example:** `option taildup on`

### `dedupitetail` -- on | off, default `on`

- **Symptoms:** both arms of an if/else begin or end with the same duplicated statements; shared cleanup or help call repeated verbatim in the two branches; compiler-duplicated tail emitted twice instead of hoisted out of the if.
- **What it does:** After structuring, deduplicate a cloned `if/else` tail: when both arms of a 3-component `if` begin (prefix) or end (suffix) with one or more *leaf* blocks that emit identical C — same printed-op sequence, matching constants by value, named storage by location, and call targets by resolved callee — hoist that shared run out of the `if` (prefix before it, suffix after) so it is emitted ONCE instead of twice (a print-tree edit — no p-code cloned). The inverse of the SAILR duplication passes (gotoreduce/crossjumprevert/taildup, which DUPLICATE a shared tail to drop a goto); here kuna's collapse-based structurer over-duplicated a shared continuation and the readability win is to merge the copies. Mirrors angr's structurer ITE region deduplication (converging two predecessors onto one copy instead of cloning). Bounded (<=8 leaves, <=64 ops/leaf), exact-or-decline, and suffix-sound only when each arm's divergent middle falls through to the suffix.
- **When to flip:** kuna emits the SAME leaf statement(s) at the head or tail of BOTH arms of an if/else (a compiler-duplicated shared prefix/suffix the structurer cloned into each arm), e.g. a shared cleanup epilogue or a `--help`-style shared call repeated in both branches. On by default (DIV-14); flip OFF to restore the upstream duplicated-arm rendering. On, the shared leaf run is hoisted out of the `if` (one copy), each merge logged (`dedupitetail:`). Conservative: only flat C-equivalent leaf blocks are merged (never a nested if/loop or a labelled/goto block), and a suffix is hoisted only when both arms provably reach it. Deeply entangled duplication where the shared code shares a basic block with divergent code (e.g. true_1804 usage's online-help printf folded into the setlocale condition block) is out of scope — that needs the full label-convergence transform.
- **Where / provenance:** P8/goto-quality · angr · structure-recovery · angr-ITE-region-dedup
- **Example:** `option dedupitetail on`

### `iteregion` -- on | off, default `on`

- **Symptoms:** if (c) v = A; else v = B; assignment diamond where a ternary v = c ? A : B is expected; format/flag builder full of two-arm constant assignments to the same variable; flip off when the source likely wrote an explicit if/else.
- **What it does:** After structuring, rewrite a narrow two-arm assignment *diamond* — a 3-component `if` whose condition block is a single `CBRANCH`, whose true- and else-clauses are each a single `COPY` to the SAME variable (`if (c) v = A; else v = B;`), converging on one tail — into a single `?:` ternary statement `v = ( c ) ? A : B;`. A **print-only** mark on the condition `CBRANCH` (no p-code cloned/mutated); the two arm COPYs and the condition stay live so the merged variable's later uses render unchanged, only the `if`'s layout differs. The S9 emit hook is the one genuinely new piece (Ghidra's printc has no ternary operator). Mirrors angr's `ITERegionConverter` / `_find_ite_assignment_regions` (a conditional-jump region with two single-assignment children on one tail). Halves the branch count of format/print/flag code full of `flags ? "%s," : "%s"` diamonds (iproute2 print_link_flags: 41 ifs/0 ternaries -> 21 ifs/20 ternaries, GED 140 -> small).
- **When to flip:** Default ON (DIV-17). The diamond->ternary form matches the common format/print/flag source where the ternary dominates (compilers turn `cond ? "%s," : "%s"` into exactly this diamond; iproute2 print_link_flags, coreutils output helpers), so kuna emits `v = c ? A : B;` matching source/angr. Still a RUNTIME CHOICE an agent can flip OFF (`option iteregion off`, byte-identical to upstream) per function when explicit `if/else` is the likely source, to avoid inventing a ternary the author never wrote — the SAME object code is emitted for both, so the binary cannot distinguish them. A print-only mark; on, each rewrite is logged (`iteregion:`).
- **Where / provenance:** P8/goto-quality · angr · structure-recovery · angr-ITE-region-converter
- **Example:** `option iteregion on`

### `iteexpr` -- on | off, default `off`

- **Symptoms:** if (c) v = <expr1>; else v = <expr2>; where a ternary v = c ? expr1 : expr2 is expected; kuna emits far fewer ?: ternaries than angr/ida on format/flag/size code; computed-arm assignment diamond (v = *p / v = b + k) not folded to a ternary.
- **What it does:** Extend `iteregion` from single-`COPY` arms to any single-statement COMPUTED pure-value arm (`if (c) v = *p; else v = q;` -> `v = c ? *p : q;`, or `v = c ? b + 5 : b - 3;`). The arm's assignment op may be any pure value op (`LOAD`, `INT_*`, `PTR*`, `CAST`, `SUBPIECE`, …) whose second-level operands are single-use implied varnodes; only side-effecting/control ops (a `STORE`, a call, a branch, an `INDIRECT`/`MULTIEQUAL`) are rejected. The C ternary evaluates only the taken branch, so the rewrite is semantics-preserving. Print-only (same mechanism as iteregion). Matches angr's aggressive `?:` recovery: on decbench O0 coreutils angr emits ~1389 ternaries to kuna's ~177; this roughly doubles kuna's recovery (ls 27->46, sort 2->21, du 19->34).
- **When to flip:** Turn ON to recover `?:` over computed arms like angr/IDA do, when the source used ternaries (common in format/print/flag/size code). A RUNTIME CHOICE, default-off: it diverges when the source used an explicit if/else (the same object code is emitted either way). Requires `option iteregion on` (default) — it broadens iteregion's arm match. Print-only; on, each rewrite is logged (`iteregion:`).
- **Where / provenance:** P8/goto-quality · angr · structure-recovery · angr-ITE-region-converter-expr
- **Example:** `option iteexpr on`

### `iteboolean` -- on | off, default `on`

- **Symptoms:** if (a && b) v = 1; else v = 0; where the source has a single boolean assignment v = a && b; -O0 boolean materialization printed as an explicit 0/1 constant diamond; extra CFG blocks/edges vs Hex-Rays around a short-circuit boolean assignment.
- **What it does:** After structuring, re-roll a SHORT-CIRCUIT 0/1 select diamond back into a single boolean assignment: a 3-component `if` whose condition is the folded `&&`/`||` chain and whose two arms are each a single `COPY` of the constants `1` and `0` to the SAME integral variable becomes `v = ( a && b );` (arms `0`/`1` -> `v = !( a && b );`) instead of `if (a && b) v = 1; else v = 0;`. This is the case P3 `RuleConditionalMove` cannot fold: it requires each `MULTIEQUAL` input block to be the CBRANCH root or a SINGLE-predecessor pass-through (upstream `ruleaction.cc:9427` `inblock->sizeIn() != 1` -> bail), and a short-circuit chain gives the constant arm 2+ predecessors. Upstream's bail is also correct at the IR level -- there is no single dominating CBRANCH standing for the whole chain, and hoisting a guarded operand out of its branch would evaluate it unconditionally -- so the re-roll is only safe AFTER structuring, where the chain is already one `BlockCondition` that C's own `&&`/`||` renders with short-circuit semantics. A **print-only** mark on the condition's terminal `CBRANCH` (no p-code cloned/mutated); the condition is emitted by the SAME renderer that produced the `if (...)` header, so evaluation order, short-circuiting and comma-expression side effects are preserved verbatim. Declines a pointer/float destination, a labelled arm, a computed arm and any second statement in an arm.
- **When to flip:** Turn ON to recover the source shape of `-O0` boolean assignments (`x = a && b;`, `x = p && (p->flags & F);`), which gcc materializes as an explicit constant diamond that kuna otherwise prints as `if (...) v = 1; else v = 0;`. `-O0` C materializes booleans everywhere, so this is broad: it is the whole residual structural gap vs Hex-Rays on bash `time_command` (two diamonds = 6 CFG blocks / 8 edges of a 28-node function; GED 26 -> 6). On by default (DIV-51): the flip changes 0/675 datatest assertions and is inside the speed budget. Still a RUNTIME CHOICE an agent can flip OFF (`option iteboolean off`, byte-identical to the pre-DIV-51 render) for the same reason as `iteregion` (DIV-17): an explicit `if (c) x = 1; else x = 0;` in the source compiles to the SAME object code, so the re-roll diverges from a source that really did write the if/else. Print-only; on, each re-roll is logged (`iteboolean:`).
- **Where / provenance:** P8/goto-quality · ida · structure-recovery · decbench-shortcircuit-boolean-materialization
- **Example:** `option iteboolean on`

### `itecondlist` -- on | off, default `on`

- **Symptoms:** only every other ternary in a chain of identical ?: assignments is recovered; a re-rolled v = c ? A : B; is immediately followed by an identical un-re-rolled if (c) v = A; else v = B;; iteboolean re-rolls 1 of 3 identical short-circuit boolean assignments; a diamond folds in isolation but declines when a structured if precedes it.
- **What it does:** Let the `iteregion` and `iteboolean` diamond matchers see through a multi-component `BlockList` in the diamond's CONDITION position, by descending it to its LAST component. Their shared `leaf_bblock` helper descends a one-component list but bails on a list of two or more, and the collapse structurer concatenates a just-collapsed predecessor onto the next diamond's condition block — so in a RUN of structurally identical diamonds every other one is declined, and kuna re-rolls exactly ceil(N/2) of N (measured N=1..8: 1 1 2 2 3 3 4 4). Sound because the rewrite is print-only and the printer already renders this shape: `emit_block_if` emits the condition component once under NO_BRANCH (its statements) and once under ONLY_BRANCH (its branch), and `emit_block_ls` emits every component under NO_BRANCH but only the LAST under ONLY_BRANCH — so the leading components print as ordinary statements before the `if` header either way, and the ternary/boolean emitters do the same two emissions on the same block. The arms keep the strict single-statement match and the labelled-goto-target guard still applies to the descended leaf. Restricted to `BlockList`: a `BlockGraph` is excluded because `emit_block_graph` ignores ONLY_BRANCH and would print the whole graph twice.
- **When to flip:** kuna re-rolls only SOME of a run of structurally identical assignment/boolean diamonds -- one `v = c ? A : B;` (or `v = a || b;`) followed by an explicit `if (c) v = A; else { v = B; }` with the same condition and arms, alternating. The tell is that which diamonds are missed depends on the NEIGHBOURING structure, not on the diamond itself: an immediately preceding structured `if` suppresses the very next one. On by default (DIV-56): the flip changes 0/675 datatest assertions and is inside the speed budget. Flip OFF (`option itecondlist off`) to restore the pre-DIV-56 every-other rendering; it is a strictly narrower match, so OFF can only ever fold FEWER diamonds. Requires `option iteregion on` and/or `option iteboolean on` (both default) -- it widens their condition match only, and never fires on its own.
- **Where / provenance:** P8/goto-quality · ida · structure-recovery · decbench-ite-every-other-diamond
- **Example:** `option itecondlist on`

### `returndup` -- on | off, default `on`

- **Symptoms:** giant short-circuit if with comma-expression side effects merging several source early-return guards; one trailing return shared by many guard paths where the source used per-guard early returns; merged guard condition containing v = f(...) assignments inline.
- **What it does:** Before the final block structuring, duplicate a SHARED bare-epilogue RETURN block into each of its predecessors but one, so the classic guard shape `if (cond) { body; return X; } return Y;` -- which gcc -O0 compiles to a single multi-predecessor RETURN block -- structures as per-predecessor early returns instead of ONE merged, comma-folded exit. kuna (like upstream Ghidra) otherwise keeps the merged form: CollapseStructure::rule_block_or fuses the guards (which share the epilogue out-target) into one short-circuit condition with comma-expression side effects (e.g. `if ((A||B) && (v=f(..), C||D)) { rest }`), and the existing ActionReturnSplit (the goto-driven ReturnDuplicatorLow analog) only splits when structuring left a goto into the return -- which the clean guard collapse never does. This is the gotoless complement: it reuses the SAME `return_split_is_splittable` filter (only MULTIEQUAL/COPY/RETURN over constant/annotation/non-free inputs -- a side-effect-free epilogue, angr's `_is_simple_return_graph`) and the SAME `node_split` machinery ActionReturnSplit drives, splitting every in-edge but one. Mirrors angr SAILR `ReturnDuplicatorHigh` (the gotoless return duplicator). SELECTIVE like angr: it duplicates ONLY when the shared return returns a CONSTANT (or a phi/COPY chain resolving to constants) -- angr's `_should_duplicate_dst = dst_is_const_ret`. A `return <variable>` shared block is left merged (the unselective version diverged from the source's short-circuit form often enough to regress the decbench GED-perfect count ~976 across 21768 firings, which reverted DIV-18; the const-return gate drops ~40-60% of firings -- the over-eager variable-return splits -- while keeping the guard-clause wins like coreutils factor, and it is what turned the aggregate measurement positive again for DIV-54).
- **When to flip:** On by default (DIV-54), which SUPERSEDES the DIV-18 revert. It recovers the source early-return shape: where kuna/upstream Ghidra collapse two-or-more source early-return guards that share a compiled epilogue into ONE giant short-circuit `if` with comma side effects and a single trailing return, returndup duplicates the shared bare-epilogue RETURN block (<= 16 predecessors, <= 64 splits/function) into each predecessor so the structurer emits separate `if (c) return X;` clauses matching the source/angr/ida (coreutils factor: kuna 3 ifs / 0 early returns -> 4 ifs / early returns, GED 12 -> 0; also closes the ghidra-beats-kuna bash compspec_dispose shape). Bounded and side-effect-safe (the splittable filter admits only a return-only block, so no call/store is ever cloned), each duplication logged (`returndup:`). It shipped default-ON once before and was reverted 2026-07-05 at -976 GED-perfect; that measurement predates the const-return gate, and the SELECTIVE pass re-ablated on the same corpus (536 slices / 52,862 functions) is +417 GED-perfect and -7,756 aggregate GED, net-positive in all nine partitions tested (split count, void vs value return, distinct return values, return count, switch, all-returns-identical, body size, project, opt level). Still a RUNTIME CHOICE: flip OFF (`option returndup off`, byte-identical to upstream) per function where the source plainly used the merged short-circuit form -- the same object code is emitted either way, so the binary cannot distinguish them. The one measured net-negative subpopulation is a split that de-structures a loop (321 firings, -257 GED); a gate for it was built and scored at +2 perfect / -209 GED and NOT kept.
- **Where / provenance:** P8/goto-quality · angr · structure-recovery · angr-ReturnDuplicatorHigh
- **Example:** `option returndup on`

### `orchain` -- on | off, default `on`

- **Symptoms:** a source boolean return renders as a cascade of if (c) return 0; / if (c) return 1; guards; option returndup off collapses the whole function to one short-circuit expression; a source guard testing several conditions with and/or becomes that many identical constant-return guards; the return-dupe slug on a function whose source wrote one short-circuit condition.
- **What it does:** Decline a `returndup` split whose shared bare-epilogue RETURN block is the out-target that two conditionals must keep in COMMON for `CollapseStructure::rule_block_or` to fuse them -- i.e. the block is an operand chain of a short-circuit expression, not a set of independent source early-return guards. `returndup` gives every predecessor of a shared return its own private return; for guard clauses that is the source shape, but for the operand blocks of `a || (b && c)` it removes the shared out-target `rule_block_or` matches on, and because `ActionReturnDup` runs in the `returnsplit` group while `rule_block_or` runs later in `collapse_conditions`, the fold can never be recovered -- one source boolean expression comes back as a cascade of constant-return guards. The gate is a read-only replay of `collapse_conditions` on the bblocks CFG (`kuna_orchain::shortcircuit_shared_targets`), mirroring `rule_block_or`'s own admission test rather than approximating it: two condition nodes fuse when the sibling has exactly one in-edge, has two out-edges, is not a switch block, is not `BlockBasic::isComplex`, and shares one out-target with the head; each fuse records that shared target and rewrites the head's out-targets, so a chain folds inside-out exactly as the structurer's fixpoint does. The three upstream tests it omits cannot hold at this point in the schedule (no goto has been elected yet, and a back edge into a single-in-edge block would make that block unreachable). When any of the function's own split candidates is a recorded shared target the whole function's splits are declined, which is exactly the counterfactual the corpus measurement priced. The predicate additionally protects a block joining two or more single-exit arms that are themselves recorded fold targets: before the constants are propagated into the epilogue phi a chain's arms are one bare `v = K` block each and the RETURN sits one edge further on, which is the graph the FIRST returndup invocation of the schedule's fullloop sees.
- **When to flip:** On by default (DIV-69). Flip OFF (`option orchain off`) to restore the pre-DIV-69 rendering, where one source `return a || (b && c);` (or a source `if (A && B && C)` guard) comes back as a cascade of `if (a) return 1;` / `if (b) return 0;` constant-return guards -- the shape `option returndup off` also removes, at the cost of every OTHER early-return recovery in the function. It is returndup-local and one-directional: it can only ever make `returndup` fire LESS, never more, and it is inert with `option returndup off`. Measured over the full decbench corpus (85,195 functions / 795 slices, three optimisation levels; `docs/decbench/returndup-o0-measurement.md` for the motivating ablation): +611 GED-perfect at O0 for -13 at O2 and -15 at O2-noinline, +583 net and -967 aggregate GED, 781 functions moved to a perfect structural match against 198 moved off (3.94 : 1), and every architecture net-positive. It is 92% of the +630 ceiling the post-hoc 'a short-circuit fold was lost' signal prices, at 94% byte-identity with the `returndup off` body on the 3,059 -O0 functions it fires for. The residual cost is real and is not observable from the binary: where the SOURCE wrote the guard cascade and the compiler merged it, declining the split is wrong (coreutils factor, GED 0 -> 12).
- **Where / provenance:** P8/goto-quality · kuna · structure-recovery · decbench-returndup-o0-shortcircuit
- **Example:** `option orchain on`

### `earlyreturn` -- on | off, default `on`

- **Symptoms:** inverted diamond: if (guard) { entire body } else { v = 0; } return v;; leading argument-validity check wraps the whole body instead of an early return; function opens with if (p != 0) { everything } where source wrote if (!p) return 0;.
- **What it does:** Hoist a leading const-guard into an early return (`if (c) return K;`) by peeling only the CONSTANT arm of a MIXED return phi, so a function that opens with an argument-validity check recovers the source's early-return + de-indented body instead of kuna's inverted diamond (`if (guard) { entire body } else { v = default; } return v;`). This is the PER-EDGE narrowing of angr SAILR `ReturnDuplicatorHigh` that the whole-block `returndup` gate structurally cannot reach: the shared RETURN block is `v = MULTIEQUAL(#K /*guard-false arm, constant*/, <var> /*body arm, variable*/); return v`, and `returndup`'s `returndup_is_const_ret` requires the WHOLE return value const, so it rejects this mixed diamond outright. earlyreturn instead computes the per-in-edge const set (the phi input slots that resolve to constants) and splits ONLY those edges via the SAME `node_split` machinery, leaving the variable body return merged. Each peeled const arm becomes its own `return K` predecessor; `branchflip` then orients it as the `if`-true and `ifelseflatten` drops the else and de-indents the body -- yielding angr's exact `if (c) return K; ...body...; return <var>;` (coreutils realpath::relpath: kuna 73 loc diamond -> angr's 52 loc early-return). Because it only ever peels a CONSTANT arm (never a `return <variable>` share), it CANNOT re-introduce the variable-return over-firing that made broad returndup regress the aggregate GED-perfect count ~976; it targets the mixed const/variable diamond that whole-block returndup skips. Bounded: return-only epilogue (the splittable filter rejects calls/stores), never splits the last edge (body keeps its return), <= 16 predecessors, <= 32 splits/function; each hoist logged (`earlyreturn:`).
- **When to flip:** A function whose first action is an argument-validity check (`if (!p) return 0;`) renders as the inverted diamond `if (p != 0) { whole body } else { v = 0 } return v;`. On by default (DIV-23: the decbench ablation measured it NET-POSITIVE, +47 perfect / -576 summed GED / 158:54 improved:regressed); flip OFF to restore the upstream merged-diamond rendering. It is MORE permissive than the whole-block `returndup` gate (it fires on mixed const/variable diamonds returndup skips), but by construction it cannot cause the variable-return over-firing that reverted returndup (it splits only constant arms).
- **Where / provenance:** P8/goto-quality · angr · structure-recovery · angr-ReturnDuplicatorHigh-peredge
- **Example:** `option earlyreturn on`

### `switchreturn` -- on | off, default `on`

- **Symptoms:** wide switch where every case assigns a constant and breaks to one shared return; switch (x) { case A: v = K; break; ... } return v; instead of per-case return K; 17-plus-case const dispatch keeps a merged return variable.
- **What it does:** The direct continuation of `earlyreturn` (the per-edge const narrowing of angr SAILR `ReturnDuplicatorHigh`), extended from the narrow if/else-if diamond to the WIDE multi-way SWITCH const-phi return. An enum/switch dispatch whose every case assigns a constant to a shared temp and breaks to one exit compiles to `v = MULTIEQUAL(#K0, #K1, …); return v` with one `break` per case; kuna keeps the merged `switch (x) { case A: v = K0; break; … } return v;` (~2 CFG nodes/case more than the source's per-case `case A: return K0;`). `earlyreturn` ALREADY recovers per-case returns for a switch whose merge block has <= 16 predecessors (findutils get_fts_info_name, a 14-case switch, hoists to `case 1: return "FTS_D"; …`), but its MAX_EARLYRETURN_INEDGES=16 cap leaves the WIDE dispatch table merged: a 17-case switch merges 17 in-edges (> 16) so earlyreturn skips it (libedit tty__getcharindex). switchreturn is the SAME peel with the in-edge cap lifted (256): it splits each constant case in-edge (all but the last, which keeps the block alive) via the SAME `earlyreturn_const_edges` + `node_split` machinery, so every case ends in its own `return K`. Because it still only ever splits a CONSTANT case arm (never a `return <variable>` share), it inherits earlyreturn's safety and CANNOT re-introduce the variable-return over-firing that made broad returndup regress the aggregate GED-perfect count ~976; any non-const body/default/fall-through arm is left merged. Orthogonal to earlyreturn (a separate option): when both are on, the narrower earlyreturn consumes the <= 16-edge cases first, so switchreturn's incremental effect is precisely the wide dispatch tables earlyreturn's cap skips. Bounded: return-only epilogue (the splittable filter rejects calls/stores), never splits the last edge, <= 256 predecessors, <= 256 splits/function; each hoist logged (`switchreturn:`).
- **When to flip:** A WIDE multi-way const dispatch renders merged — `switch (x) { case A: v = K0; break; … } return v;` — instead of the source's per-case `switch (x) { case A: return K0; … }` (more cases than earlyreturn's 16-in-edge cap; e.g. libedit tty__getcharindex, iproute2 accept_msg, coreutils stat fmt_to_mask). On by default (DIV-25: the incremental wide-switch ablation on top of earlyreturn-on measured NET-POSITIVE, +2 perfect / -107 summed GED / zero regressions); flip OFF to restore the merged rendering. By construction it cannot cause the variable-return over-firing that reverted returndup (it splits only constant case arms).
- **Where / provenance:** P8/goto-quality · angr · structure-recovery · angr-ReturnDuplicatorHigh-switchphi
- **Example:** `option switchreturn on`

### `foldcallret` -- on | off, default `on`

- **Symptoms:** call result spilled to a temp used exactly once: v5 = f(); if (v5 < 0); single-use call return not inlined into its use site; flip off to force every call output into a named temporary (ghidra style).
- **What it does:** Fold an order-safe single-use call return value into its use site, inlining the call expression instead of spilling it to a named temporary (e.g. `if (timespec_cmp(...) <= -1)` instead of `v5 = timespec_cmp(); if (v5 <= -1)`).
- **When to flip:** kuna spills a call result to a `vN = call(); use(vN)` pair that is used exactly once where angr folds the call expression into its use site. On by default (DIV-14); flip OFF to restore the upstream explicit-temporary form (Ghidra forces every call output explicit). Only folds when the single use is in the same block with no intervening call/load/store, so the call's evaluation order is preserved.
- **Where / provenance:** P6/explicit-marking · angr · presentation-default · angr-call-return-variable-folding
- **Example:** `option foldcallret on`

### `stackguard` -- on | off, default `on` (destructive opt-in)

- **Symptoms:** stack-protector canary compare against fs:0x28 and a __stack_chk_fail branch cluttering the epilogue; shared-return goto forced by the canary check block; flip off to keep the real canary instructions for auditing the protector.
- **What it does:** REMOVES CODE: strips the -fstack-protector canary epilogue (the `if (canary != *(fs:0x28)) __stack_chk_fail()` check) from the output. Strips the glibc -fstack-protector canary epilogue (the check + its no-return call block) AND the entry-side canary init (`slot = *(fs:0x28)` and its TLS-base residue, released to dead-code once the last check is stripped), collapsing the shared single-return point so each path returns directly (no goto/label, no dead canary store).
- **When to flip:** On by default (DIV-14): removes compiler-inserted stack-protector boilerplate and the shared-return goto it forces. Flip OFF to keep the real canary-check instructions in the output (e.g. to audit the protector itself, or if an unusual non-glibc `ptr+0x28` compare guarding a call is being matched). It is marked destructive because it deletes those real instructions, but it is byte-identical over the 675 datatests (the `Partial splitting` cases opt out via `option stackguard off`).
- **Where / provenance:** P7/edge-virtualization · angr · opt-in-tool · angr-StackCanarySimplifier
- **Example:** `option stackguard on`

### `securitycheck` -- on | off, default `on` (destructive opt-in)

- **Symptoms:** Rust output littered with `panic_bounds_check()` / `slice_error_fail()` / `panic_const_div_by_zero()` calls; every slice index guarded by an `if` whose arm only panics; Rust function CFG roughly twice the size of the source control flow; flip off to keep the real bounds-check instructions for auditing.
- **What it does:** REMOVES CODE: strips rustc's bounds / slice / divide-by-zero panic branches from Rust output. Every checked slice index, string slice and non-constant `/`/`%` compiles to a conditional branch to a tiny diverging block that calls one of seven `core::panicking` / `core::slice::index` / `core::str` helpers; the branch doubles the CFG of ordinary Rust code and carries nothing a reader wants. The edge is severed and the orphaned handler collected, so the guarded access becomes a plain statement.
- **When to flip:** On by default (DIV-82): a Rust binary reads with half the branches and none of the `panic_bounds_check()` noise. Flip OFF to keep the real check instructions (auditing whether a particular access is actually bounds-checked, or reviewing a panic path). The trigger is the callee NAME, and all seven names are Rust-only, so the pass is structurally inert on a C binary -- it is byte-identical over the 675 datatests and over the C regression fixtures. It is marked destructive because it deletes real instructions, and there is one visible second-order effect: a length argument whose only reader was the removed compare becomes genuinely unused, so parameter recovery may trim it from the signature (the same trade `stackguard` makes with the canary slot). It cannot fire on a call whose callee name kuna did not recover: in a default (PIE, dynamically linked) Rust binary the helper is reached through an unrelocated GOT slot and has no name, so the pass is currently a no-op there.
- **Where / provenance:** P7/edge-virtualization · angr · opt-in-tool · oxidizer-SecurityCheckRemover
- **Example:** `option securitycheck off`

### `branchflip` -- on | off, default `on`

- **Symptoms:** if (x == 0) guard with the common path in the else arm; negated condition where angr renders the positive complement first; if/else polarity inverted versus the source's reading order.
- **What it does:** Flip a negated-guard if/else for linearity: when an `if (x == 0) {A} else {B}` (equality-to-zero / negated guard) can be flipped in place, rewrite it to the positive complement `if (x != 0) {B} else {A}` so the common path reads top-to-bottom (angr-style positive guard vs `if (x == 0)`).
- **When to flip:** An `if (x == 0)` negated guard reads inverted versus the angr-style positive form. On by default (DIV-14): the non-negated comparison becomes the `if` condition and the if/else arms swap to match; flip OFF to keep Ghidra's polarity. A flip is logged as a `branchflip:` warning comment at the if. Only fires on `if/else` (3-component) blocks whose condition flips cleanly in place.
- **Where / provenance:** P8/readability-rewrites · angr · opt-in-tool · angr-SAILR-condition-polarity
- **Example:** `option branchflip on`

### `loopbreak_recovery` -- on | off, default `on`

- **Symptoms:** loop exit rendered as goto label_N; plus a synthesized label instead of break;; switch-case exit gotos where break; is expected; error paths leave a loop by goto to its successor label.
- **What it does:** Lower loop-exit `goto <successor>` edges to structured `break;` (and switch-case exits to empty `break;`), a port of Ghidra BlockGraph::scopeBreak run in ActionFinalStructure between finalizePrinting and markUnstructured; the now-dead successor label is suppressed.
- **When to flip:** A loop's error/exit paths render as `goto <successor-label>;` plus a synthesized `label_NNNN:` (angr emits `break;`). On by default (DIV-10, clean ablation + converges to upstream Ghidra, which always runs scopeBreak); set OFF to restore kuna's prior byte-identical raw-goto rendering.
- **Where / provenance:** P8/goto-quality-acceptance · angr · structure-recovery · angr-1after909-doit
- **Example:** `option loopbreak_recovery off`

### `noreturn_known` -- on | off, default `on`

- **Symptoms:** dead code kept after calls to exit/abort/panic when off; call to a known no-return libc function still shows a fall-through path; unreachable epilogue after std::terminate or a rust panic call.
- **What it does:** Run the known-no-return analysis pass: mark exit/abort/_Unwind_Resume/std::terminate-style functions no-return (the kuna analog of Ghidra's NoReturnFunctionAnalyzer, plus the Rust panic/handle_alloc_error list).
- **When to flip:** On (default) suppresses dead fall-through after a no-return call; off leaves the call rendering as if it returns (dead code reappears).
- **Where / provenance:** P1/external-refinement · kuna · analysis-enablement · kuna-analysis-noreturn
- **Example:** `option noreturn_known off`

### `peimportcall` -- on | off, default `on`

- **Symptoms:** windows api calls render as (*dat_411324)() with no name; a PE function runs past ExitProcess or ExitThread and absorbs the next function's body; the same loop is emitted twice, once inside a caller and once as its own sub_ function; no Win32 API names anywhere in a decompiled PE.
- **What it does:** Bind a Windows `call dword ptr [IAT slot]` to the import the loader already resolved at that slot. The PE loader names every Import Address Table slot, but the call lifts to a CALLIND through a global and the only pass that resolves such a target (ActionDeindirect) requires `Varnode::externref`, which upstream sets from an ExternRefSymbol kuna never creates - so every Windows API call stayed an unnamed `(*dat_4112c4)(0)` with no prototype and no no-return flow effect. On: paint `externref` over the IAT slot ranges so the call deindirects to the import FunctionSymbol, carry the callee's no-return flag onto the prototype ActionDeindirect merges, and match upstream's PE-only no-return API list (ExitProcess/ExitThread/FreeLibraryAndExitThread/KeBugCheck/longjmp/...) that kuna's merged PE/Mach-O list never named. PE/COFF only; a no-op on every other object format.
- **When to flip:** On by default (DIV-57). On, a Windows PE/DLL renders its Win32 calls by name (`ExitThread(0)`, `HeapFree(...)`) instead of `(*dat_411324)()`, and a function ending in a no-return API stops there instead of running past it and swallowing the next function's whole body (mydoom.exe `mmsender_th` drops a `while` loop it never had: 50 lines/4 ifs/1 loop -> 13 lines/2 ifs/0 loops, matching IDA and Ghidra). Flip OFF to restore the unnamed indirect-call rendering byte for byte (every non-PE target is byte-identical either way).
- **Where / provenance:** P1/external-refinement · kuna · correctness-fix · decbench-O0-mydoom-mmsender_th
- **Example:** `option peimportcall on`

### `noreturn_disc` -- on | off, default `on`

- **Symptoms:** heavily-called custom die()/fatal() wrapper still treated as returning; dead code after a stripped sub_ wrapper that never falls through at 3+ call sites; caller swallows the next function after a wrapper call.
- **What it does:** Run the discovered-no-return consumer (the first Listing/xref consumer): the evidence-tally fixpoint over the Listing — a call target is no-return when at least 3 of its call sites show no valid fall-through (the instruction after the call is not a valid instruction start / fell into data / is another function's entry), iterated to a fixpoint (a function whose last act is a call to an already-discovered-no-return function is itself no-return). The kuna analog of Ghidra's FindNoReturnFunctionsAnalyzer. Requires the Listing (option listing on) — a no-op when the Listing is absent. On by default (DIV-22, matching Ghidra's default-on analyzer), but gated on the Listing (default-off), so every parity gate is byte-identical (real-ELF Listing path only).
- **When to flip:** On by default (DIV-22), but a no-op unless the Listing is built (option listing on) — with the Listing off there is zero behavior change. With listing on it marks heavily-called custom/tail-calling no-return wrappers — that the static name lists do not know — no-return by the >=3-call-site evidence tally, eliminating the post-call dead code on a real-ELF target. Flip OFF (with listing on) to keep the post-call fall-through code.
- **Where / provenance:** P1/external-refinement · kuna · analysis-enablement · kuna-analysis-noreturn-disc
- **Example:** `option listing on --option noreturn_disc on`

### `noreturn_propagate` -- on | off, default `on`

- **Symptoms:** unreachable code after a call to an exit/fatal wrapper; function truncated after calling a cold wrapper; spurious while(true) around a call that never returns; stripped binary's unnamed exit wrapper swallows the functions after it.
- **What it does:** REMOVES CODE: propagates no-return through the call graph, dropping unreachable code after no-return calls. Runs the structural no-return propagation consumer (the second Listing/xref consumer): the kuna analog of angr's CFGFast call-graph no-return propagation. Seed the terminal set from the Known no-return list and conclude a function no-return when its last real instruction (skipping trailing NOP alignment padding) is a CALL (or tail JMP) to an already-no-return callee, with no RETURN path, no computed jump, and no branch escaping the reachable body — iterated to a fixpoint, with NO evidence threshold (unlike noreturn_disc's >=3 call sites). Catches a custom no-return wrapper (e.g. coreutils xalloc_die, which ends in `call abort` followed by padding) that the static name list does not know and the >=3-evidence rule does not reach, so a caller (e.g. tee_O2 x2nrealloc) no longer grows a spurious while(true)/goto from the dead fall-through. Emits the existing NoReturnFact -> the existing set_function_no_return commit seam (no new commit arm, no S7 work). Requires the Listing (option listing on) — a no-op when the Listing is absent. On by default (DIV-14), but gated on the Listing (default-off), so every parity gate is byte-identical (real-ELF Listing path only).
- **When to flip:** On by default (DIV-14), but a no-op unless the Listing is built (option listing on) — with the Listing off there is zero behavior change. With listing on it concludes a custom no-return wrapper (a die()/fatal()/xalloc_die() that unconditionally ends in abort/exit) is no-return where it would otherwise be treated as returning — angr renders the caller cleanly but kuna would emit a spurious while(true) loop + goto + dead stack spills from the post-call fall-through (e.g. tee_O2 x2nrealloc). Broader than noreturn_disc: no >=3 call-site threshold and it seeds from the Known list. Flip OFF (with listing on) to keep the post-call fall-through code in the output.
- **Where / provenance:** P1/external-refinement · angr · structure-recovery · kuna-analysis-noreturn-propagate
- **Example:** `option listing on --option noreturn_propagate on`

### `noreturn_error` -- on | off, default `on`

- **Symptoms:** caller keeps decoding the cold path after error(2, ...) into the next function; function balloons past a wrapper that tail-calls error with nonzero status; dead fall-through after a gnu error(nonzero, ...) call.
- **What it does:** REMOVES CODE: concludes an error(nonzero,...) wrapper is no-return, dropping the dead fall-through at every caller. The value-conditional slice of Ghidra's discovered-no-return analyzer, folded into the noreturn_propagate consumer: glibc error(int status, int errnum, const char *fmt, ...) and error_at_line(int status, ...) call exit(status) and NEVER return when status != 0 — but DO return for status == 0 — so `error` cannot be a Known no-return. Yet an internal wrapper whose tail is `call error(2,...)` (GNU pfatal_with_name, and every die()-via-error helper) is unconditionally no-return. When on, the propagation treats such a tail call as terminal by checking the call site's first int-arg register (x86-64 SysV EDI/RDI = the `int status`) is a nonzero literal (MOV EDI,0x2), concludes the wrapper no-return, and the existing fixpoint propagates that to every caller — which then truncates at the wrapper call instead of decoding the cold fall-through / next-function bytes as live code (e.g. diffutils diff `sip` collapses from 99 lines to ghidra's ~43, GED 347 -> 0). A zero status (XOR EDI,EDI / MOV EDI,0x0 — error() returns) or any non-constant/unprovable status is rejected (conservative: a false positive would drop live caller code). Emits the existing NoReturnFact -> the existing set_function_no_return commit seam (no new commit arm, no S7 work). Requires the Listing (option listing on) AND noreturn_propagate on — a no-op otherwise. On by default (DIV-16), but gated on the Listing (default-off), so every parity gate is byte-identical (real-ELF Listing path only).
- **When to flip:** On by default (DIV-16), but a no-op unless the Listing is built (option listing on) and noreturn_propagate is on — with the Listing off there is zero behavior change. With listing on it concludes an internal wrapper that tail-calls error(nonzero,...) (e.g. GNU pfatal_with_name) is no-return where kuna would otherwise treat `error` as returning (since error(0,...) returns) and keep decoding the cold error path / adjacent function's bytes as live code past every wrapper call, ballooning the caller (diffutils diff `sip`: 99 lines / GED 347 vs ghidra 43). Ghidra's discovered-no-return analyzer catches exactly this; angr's callgraph returning-fixpoint (CFGBase._determine_function_returning) is the analog. Flip OFF (with listing on) to keep the post-call fall-through code (the wrapper is treated as returning again).
- **Where / provenance:** P1/external-refinement · ghidra · correctness-fix · kuna-analysis-noreturn-error
- **Example:** `option listing on --option noreturn_error on`

### `noreturn_reach` -- on | off, default `on`

- **Symptoms:** wrapper with a mid-body fatal call still treated as returning; unreachable write()/return tail keeps a fatal wrapper looking like it returns; switch whose every arm ends in a no-return call not concluded no-return; caller decodes garbage past an sshpkt_fatal-style wrapper.
- **What it does:** REMOVES CODE: the CFG-reachability no-return rule — the port of Ghidra's FindNoReturnFunctionsAnalyzer.targetOnlyCallsNoReturn ('Non-Returning Functions - Discovered' analyzer). kuna's noreturn_propagate concludes a wrapper no-return only when its LAST real instruction is a terminal call to an already-no-return callee (a tail-call subset). That misses three shapes Ghidra catches by walking the CFG: (1) a no-return call MID-body with a dead tail after it (e.g. openssh sshpkt_fatal, whose FIRST statement is `call sshpkt_vfatal` and whose write()/return tail is unreachable); (2) a RETURN that is present but UNREACHABLE (the path to it dies at a no-return call); (3) a SWITCH (indirect jump) whose every arm ends in a no-return call (openssh sshpkt_vfatal). When on, for each candidate the rule walks the instruction-level reachable CFG from entry, treats a call/jump to an already-no-return callee as terminal (its fall-through is dead), and concludes no-return iff NO RETURN is reachable and at least one path ends at a no-return transfer — iterated to a call-graph fixpoint by the same propagate loop, emitting the same NoReturnFact -> set_function_no_return seam. Conservative: a reachable RETURN, an unresolved indirect jump, a branch escaping the body to a possibly-returning neighbour, or a call with no modelled fall-through that is not itself terminal all reject (a false positive would drop live caller code). Requires the Listing (option listing on) AND noreturn_propagate on — a no-op otherwise. On by default (DIV-19), gated on the Listing (default-off), so every parity gate is byte-identical (real-ELF Listing path only).
- **When to flip:** On by default (DIV-19), a no-op unless the Listing is built (option listing on) and noreturn_propagate is on. With listing on it concludes internal fatal wrappers that kuna's tail-call rule (noreturn_propagate/noreturn_error) misses because the no-return call is mid-body, the return is dead, or the function routes through a switch — where kuna would otherwise decode the cold error path / adjacent function's bytes as live code past the wrapper call, ballooning the caller and inflating GED vs ghidra (openssh sshpkt_fatal/ssh_tun_confirm, shadow oom). This is the general 'Non-Returning Functions - Discovered' reachability that ghidra runs by default. Flip OFF (with listing on) to fall back to the strict tail-call rule (keeps the post-call fall-through code).
- **Where / provenance:** P1/external-refinement · ghidra · correctness-fix · kuna-analysis-noreturn-reach
- **Example:** `option listing on --option noreturn_reach on`

### `paramcopyhoist` -- on | off, default `off`

- **Symptoms:** parameter copy-shadow assignment (v6 = a1;) emitted below an earlier guard instead of at the top of the function; kuna places some parameter spill copies in the entry block and others after a guard in the same function; extra CFG nodes versus the source around a cascade of if (p && !*p) p = NULL; parameter guards; vN = aM; for an unmodified incoming parameter appearing in the middle of a guard cascade.
- **What it does:** Anchor the copy-shadow of an UNMODIFIED INCOMING PARAMETER in the function's entry block instead of at the tail of the MULTIEQUAL slot's predecessor, so a guarded parameter's `vN = aM;` prints where the source's spill is -- with the other parameter spills -- rather than below an earlier guard. gcc -O0 gives every parameter a stack home, and the source idiom `if (p && !*p) p = NULL;` leaves a MULTIEQUAL at the guard's join whose incoming slots are the raw input register. `Merge::mergeOp` cannot merge the input-register HighVariable with the address-tied stack-slot HighVariable, so `Merge::trimOpInput` snips each offending slot into a COPY and places it with `opInsertEnd(copyop, op->getParent()->getIn(slot))` -- the tail of that slot's PREDECESSOR -- and that COPY is what prints as `vN = aM;`. For the FIRST parameter the guard's join predecessors intersect at the entry block, so `Merge::buildDominantCopy`'s `findCommonBlock` lands the copy there and it renders correctly; for every later parameter the intersection is the previous guard's join, so the copy sinks below that guard and splits the source's single entry block in two. Both producing sub-cases are covered: two-or-more trimmed slots (later collapsed by buildDominantCopy) and exactly one trimmed slot (no dominant-copy pass at all, so the single opInsertEnd is final). Implemented as ActionParamCopyHoist, the LAST action in universal_sched: taking the decision inside `Merge::trimOpInput` itself defeats the trim's own purpose (the widened Cover fails mergeOp's test, mergeOp falls through to trimOpOutput, and markInternalCopies then hides both resulting COPYs, so the assignment DISAPPEARS from the emitted C), and running the move any earlier than ActionFinalStructure perturbs the P8 duplication passes. Legality is buildDominantCopy's own Cover test re-run against the hypothetical hoisted placement and the FINAL HighVariable (b_cover over the high's other instances vs an a_cover whose def point is taken at the START of the entry block, rejecting on `intersect > 1`), which over-approximates and errs toward rejecting; only a formal parameter's storage is ever hoisted, only when a MULTIEQUAL reads the copy, and only when the target variable has exactly one candidate.
- **When to flip:** Turn ON when a parameter's copy-shadow assignment (`v6 = a1;`) is emitted below an earlier guard instead of with the other parameter spills at the top of the function, adding CFG nodes the source does not have. Measured on decbench O0 e2fsprogs/e2fsck save_output (0x4eeb6): 53 nodes/77 edges GED 29.0 -> 51 nodes/77 edges GED 17.0 (-12), which is GED-OPTIMAL for the construct (deleting the copies entirely and writing through the parameter, which is literally what the source does, also scores 17.0). Broad on -O0: over four O0 binaries (e2fsprogs/e2fsck, coreutils/ls, gzip/gzip, bzip2/bzip2; 3031 functions) 121 functions (4.0%) emit at least one top-level parameter copy-shadow outside the entry block -- 200 sunk copies against 318 correctly in the entry -- and turning it on moves 92 of them (200 -> 108 sunk, 318 -> 479 in the entry) across 116 changed functions with zero new decompile errors. Every one of those 116 was diffed: 99 are pure re-orderings of `vN = aM;`, 17 have a secondary rendering effect (mostly a comma side-effect inside a short-circuit condition becoming a plain statement, which is an improvement; 2 leave an empty `if (c) { }` then-branch). Default-OFF opt-in: it is a deliberate DIVERGENCE from upstream Ghidra, which sinks the copy to exactly the same place, and although the default-ON ablation is clean on the datatests (0/675, PARITY OK) it regresses one stage assertion (condfold #3) and the decbench aggregate has not been measured, so the flip is left to a follow-up.
- **Where / provenance:** P6/merge-aggressiveness · kuna · structure-recovery · decbench-parameter-copy-shadow-placement
- **Example:** `option paramcopyhoist on`

### `outline` -- off | <fn>:<head>-<exit>[;...], default `off` (destructive opt-in)

- **Symptoms:** an inlined callee body is spliced into the caller and should read as a call; want to pull a chosen region out of a function into a named pseudofunction; acting on an almostregion inline-candidate report by hand or with an llm.
- **What it does:** Stage 2 of inline identification: excise a SUPPLIED single-entry region and emit a call to a synthesized pseudofunction in its place. Where almostregion finds candidate inlined regions, this acts on one - and the choice does not have to come from almostregion, so a human reading the code or an LLM agent reading almostregion's report can drive the transform directly. The value names the owning function entry as well as the region, because an option value is global to the run and every function is decompiled under it. The region is a block SET re-derived from the CFG with the same single-entry predicate almostregion uses; the value only names the head and the exit, and is re-verified against the live bblocks every time rather than trusted. Runs in the pre-SSA window immediately before heritage, so the edge surgery needs no MULTIEQUAL patching and ActionFuncLink still materializes the call's argument and return varnodes. v1 declines - leaving the function untouched - unless the head/exit really bound a single-entry region, exactly one edge enters the head from outside it, at most one value is live out, and the region contains no call and no STORE. Liveness is computed over the region's continuation (blocks reachable from the exit) with sub-register aliases merged, because an x86 32-bit write zero-extends and would otherwise count one register twice.
- **When to flip:** Off (default; inert, and inert also when a region is supplied for a different function). Supply a region to pull an inlined callee body back out into a named call - typically taking the head and exit from `--option almostregion report`.
- **Where / provenance:** P8/goto-quality · kuna · opt-in-tool · kuna-inline-identification
- **Example:** `--option outline 0x401000:0x40100a-0x401018`

## Analysis & loader passes

Program-prep enablement: what is discovered, decoded, and named before any function is decompiled.

### `libproto` -- on | off, default `on`

- **Symptoms:** puts/printf arguments untyped so string literals render as bare constants or dat_ addresses; imports carry no signatures and call arguments stay untyped.
- **What it does:** Run the library-prototype analysis pass: seed common libc signatures (puts(char*), printf(char*,...), ...) onto matching imports so call arguments get typed (the kuna analog of Ghidra's ApplyDataArchiveAnalyzer).
- **When to flip:** On (default) types call arguments so string literals render (puts("...")); off leaves the bare constant/untyped argument.
- **Where / provenance:** P1/external-refinement · kuna · analysis-enablement · kuna-analysis-libproto
- **Example:** `option libproto off`

### `libcsigs` -- on | off, default `on`

- **Symptoms:** a caller's parameter is unsigned long where it is only ever passed to a libc function that takes a char */int; a libc call renders its string argument as a bare 0x... constant instead of a literal; a wrapper around a libc function loses its return value or return type; __printf_chk / __fprintf_chk arguments are shifted by the fortify flag; an exit or __stack_chk_fail call shows phantom arguments.
- **What it does:** Seed the MEASURED extension of the built-in libc prototype table: the ~200 further signatures the 27-entry libproto table does not carry (exit, gettext, dcgettext, error, strerror, strdup, strcasecmp, close, open, read, write, stat, getenv, syslog, the FORTIFY __printf_chk/__fprintf_chk/__memcpy_chk family, __errno_location, __stack_chk_fail, ...), ranked by call-site frequency over the frozen decbench C corpus and reduced from the platform headers. Applied ONLY to names the image imports and does not itself define, so a program's own function that happens to share a libc spelling is never retyped.
- **When to flip:** On (default) types the arguments of every common libc call, so a caller whose parameter only flows into one gets a concrete type (char *path) instead of the inferred unsigned long, and the callee's return type is known. Flip OFF when a binary links a private library that reuses libc spellings with different signatures and is NOT statically linked (the pass already skips any name the image defines), or to ablate this table's contribution to a type-recovery difference; off is byte-identical to the 27-entry base table alone.
- **Where / provenance:** P1/external-refinement · kuna · correctness-fix · kuna-analysis-libcsigs
- **Example:** `option libcsigs off`

### `strings` -- on | off, default `on`

- **Symptoms:** string constants render as raw addresses or unnamed data instead of quoted char[N] literals; no data symbols at ascii runs in rodata.
- **What it does:** Run the string-literal analysis pass: detect NUL-terminated ASCII strings (min length 5) and plant a typelocked char[N] data symbol at each (the kuna analog of Ghidra's StringsAnalyzer).
- **When to flip:** On (default) lays char[N] data at detected strings; off leaves those addresses undefined.
- **Where / provenance:** P1/code-data-partition · kuna · analysis-enablement · kuna-analysis-strings
- **Example:** `option strings off`

### `entry_disc` -- on | off, default `on`

- **Symptoms:** a stripped binary yields almost no functions (symbol stream only); functions discovered via e_entry/init_array/.eh_frame/prologues missing from the list.
- **What it does:** Run the entry-discovery analysis pass: find function entry points for stripped targets (e_entry, DT_INIT/FINI + INIT_ARRAY tables, .eh_frame FDE starts, the _start->main libc idiom, prologue patterns).
- **When to flip:** On (default) discovers + names extra entry points; off limits functions to the symbol stream.
- **Where / provenance:** P1/code-data-partition · kuna · analysis-enablement · kuna-analysis-entry
- **Example:** `option entry_disc off`

### `eh_frame_full` -- on | off, default `off`

- **Symptoms:** c++ catch/cleanup landing pads missing from a stripped binary's function list; exception-handler code never discovered as entries; gcc_except_table call-site targets left unexplored.
- **What it does:** Extend the always-on entry-discovery pass with full .eh_frame + .gcc_except_table (LSDA) markup (GccExceptionAnalyzer): for each FDE, follow the CIE 'L' augmentation to its LSDA pointer in .gcc_except_table, decode the call-site table, and emit each exception landing-pad PC (lpStart + non-zero cs_landing_pad) as a discovered function entry. A landing pad is a real code target reached only by the unwinder, so a stripped C++ binary's entry-disc otherwise misses it. The DW_CFA_* call-frame instructions are NOT recovered at the decompiler tier (kuna's own S5/S7 frame analysis already recovers the stack frame from the code) — CFI is inherited, not rebuilt. Output-changing (adds entries), so default-OFF: a default run is byte-identical to before.
- **When to flip:** Off (default) limits .eh_frame use to FDE pcBegin function starts. Flip on to also discover exception-handler landing pads (catch/cleanup blocks) from the .gcc_except_table LSDA in a C++ try/catch binary.
- **Where / provenance:** P1/code-data-partition · kuna · analysis-enablement · kuna-analysis-ehframe-lsda
- **Example:** `--option eh_frame_full on`

### `fdeinterior` -- on | off, default `on`

- **Symptoms:** spurious sub_<addr> functions inside a c++ function that uses try/catch; a decompiled function body dereferences an uninitialised frame pointer so every local is garbage; function count inflated by unwinder-only landing pads; a function entry lands in the middle of an instruction; extra entries between two real functions in a binary built with exceptions.
- **What it does:** Reject a discovered function entry that falls strictly inside another function's .eh_frame FDE body. A kuna FunctionSymbol is an entry address with no extent, so the commit boundary cannot answer 'is this candidate already inside a known function?' and every discovery oracle plants a sub_<addr> in the middle of a body it cannot see: eh_frame_full promotes each .gcc_except_table landing pad to a top-level function, aif starts one at the first undecoded byte of an unwinder-only region (which is routinely mid-instruction), and the prologue patterns match a 16-byte-aligned push rbp; mov rbp,rsp inside a larger body. Such a function inherits its parent's live frame pointer, so it decompiles with an uninitialised rbp and every local is a garbage dereference. Each .eh_frame FDE records one function's [pcBegin, pcBegin+pcRange) by construction (one .cfi_startproc/.cfi_endproc pair), which is exactly the extent the symbol table never carried, so an entry strictly inside one is not a function on the unwinder's own authority - the model IDA Pro uses, where get_func of a landing pad returns the enclosing function taken from the FDE. Only ranges that describe a single function are used: no other named function start inside, no other FDE start inside, and no overlap with a linker stub section (.plt/.plt.sec/.plt.got/.iplt/.MIPS.stubs), so the linker's single whole-PLT FDE never suppresses an import stub. An entry AT an FDE pcBegin is always kept, so the .eh_frame FDE oracle's own product is preserved. ELF-only and inert on any image without .eh_frame FDEs, which includes 95 of the 98 decbench bare-metal ARM images (they unwind through .ARM.exidx).
- **When to flip:** On (default) keeps mid-function artifacts out of the function list: the tell-tale is a sub_<addr> whose body dereferences an undefined frame pointer, or a run of sub_<addr> entries between two real functions in a C++ binary built with exceptions. Flip off to restore the previous discovery set exactly - e.g. to inspect a landing pad as its own function, or on a hand-written-assembly image whose .eh_frame deliberately covers several entry points with one FDE.
- **Where / provenance:** P1/code-data-partition · kuna · analysis-enablement · kuna-analysis-fdeinterior
- **Example:** `--option fdeinterior off`

### `funcstart_patterns` -- on | off, default `off`

- **Symptoms:** functions after nop padding missing in a stripped binary; a push rbx; mov rbx,rdi prologue never discovered as a function start; code-bearing gaps between discovered functions left undefined.
- **What it does:** Run the FULL byte-pattern function-start pass: a faithful port of Ghidra's FunctionStartAnalyzer over the entire vendored pattern corpus (Ghidra/Processors/<P>/data/patterns/*.xml), parsing every <patternpairs> (the DittedBitSequence prepattern/postpattern pairs) and bare <funcstart/> pattern for x86/x86-64 (headline) plus AArch64/ARM/RISC-V/MIPS/PPC. A candidate is a function start iff a postpattern (the prologue shape) matches at it AND a prepattern (the preceding context: a RET/JMP/NOP/...) matches the bytes immediately before it, at the instruction alignment. This is the full superset of entry_disc's always-on oracle 5, which ports only a minimal three-prologue x86-64 subset. Default-OFF because it is output-changing (it discovers MORE functions): when off, the pass's facts are dropped at commit and every decompilation is byte-identical to the entry_disc-only baseline. The after=defined / validcode=N pattern post-rules and the <possiblefuncstart/> / thunk / label actions are dropped (they need a PseudoDisassembler the analyzer tier does not have) — a documented LOSS, the same wall entry_disc's oracle-5 docs record.
- **When to flip:** On discovers additional function starts in a stripped binary via the full Ghidra prologue pattern set (e.g. a `push rbx; mov rbx,rdi` function after NOP padding that the minimal oracle misses); off (default) keeps only the entry_disc + symbol-stream functions so the output is byte-identical to the baseline.
- **Where / provenance:** P1/code-data-partition · kuna · analysis-enablement · kuna-analysis-funcstart-patterns
- **Example:** `--option funcstart_patterns on`

### `cortexmvectors` -- on | off, default `off`

- **Symptoms:** stripped cortex-m firmware yields only a handful of functions; arm firmware body decodes as a32 garbage instead of thumb; nmi/hardfault/systick exception handlers never discovered as entries; isr_vector table present but no vector-derived function starts.
- **What it does:** Widen the ARM Cortex-M hardware vector-table signature (entry discovery oracle 6). The shipped signature confirms a table only when (a) it starts a section the loader maps executable, (b) its word 0 is in the architectural SRAM block 0x20000000-0x3FFFFFFF, and (c) its word 1 equals e_entry. All three over-constrain real firmware: the table is DATA the CPU reads, so a bare-metal link script normally emits .isr_vector as an A-only section in a read-only PT_LOAD; STM32F4 and -M7 parts put the initial stack in CCM/TCM at 0x10000000; and e_entry is the ELF start symbol, which a link script is free to point somewhere other than the reset vector. With this on, a candidate is any allocated section whose word 0 lands in 0x10000000-0x3FFFFFFF and whose slots from word 1 yield at least three Thumb handler pointers (odd, inside an executable section) — the run of handlers replaces the e_entry equality, and three consecutive conforming slots is a shape a .data structure does not reach by accident. Confirming a table arms the reset/exception handler seeds AND the whole-image Thumb (TMode=1) region paint, so a stripped Cortex-M image goes from a handful of functions to the whole firmware. The widened scan runs ONLY where the shipped signature found nothing, so an image that resolved a table before resolves the same section with the same harvest: the option can add discovered entries, never remove one. ARM-only and real-object-path only, so every XML datatest is structurally untouched. Output-changing (it discovers more functions) => default-OFF.
- **When to flip:** Flip on for a stripped bare-metal ARM Cortex-M firmware image (STM32/nRF/SAM/...) that decompiles to only a few functions, or whose code decodes as A32 garbage, because kuna never confirmed its vector table. The tell-tales: the .isr_vector section is flagged A (not AX) and sits in a read-only PT_LOAD, the first word is a 0x1000xxxx CCM/TCM stack pointer rather than 0x2000xxxx, or the second word does not match the ELF e_entry. Stands alone (it needs no other option), and composes with the listing + funcstart_patterns + aif set the decompile-all/functions drivers already inject on non-x86-64. Off (default) keeps the strict e_entry-matching signature, so any image that already worked is byte-identical either way.
- **Where / provenance:** P1/code-data-partition · kuna · analysis-enablement · kuna-analysis-cortexmvectors
- **Example:** `--option cortexmvectors on`

### `ptrentry` -- on | off, default `off`

- **Symptoms:** a small arm callback or isr handler produces no decompiled output at all; kuna emits one function then the next with un-decompiled code in between; bare bx lr exception handlers never discovered as entries; vtable or fops-struct callbacks missing from a stripped arm binary; driver op-struct function pointers left undiscovered.
- **What it does:** Discover ARM function entries that are reachable ONLY through a code-pointer word (a vtable / fops struct / ISR table / literal-pool constant). The shipped Stage-3 code-pointer scan already collects every 4-byte-aligned Thumb pointer in every allocated section, then accepts the target only if it opens with a stack-frame prologue AND disassembles into more than two instructions. Those two shape predicates reject the bulk of the pointer-referenced population on bare-metal firmware: 93 percent of the missed entries establish no frame at all, and 41 percent are leaves of 8 bytes or less (movs r0,#0; bx lr, or a bare bx lr, which is a perfectly valid Cortex-M exception handler). With this on, a target is admitted on CONTAINMENT evidence instead of shape: no word that references it may be the bytes of a decoded instruction (that word is an operand, not a table slot), and none may lie in the same discovered function as the target itself (that is the ldr pc,[pc,r] switch-table shape, whose slots point into their own body); the length floor is replaced by a terminating-routine check, so a one-instruction routine that reaches a clean return is accepted while a data word that merely decodes is not. Measured on the 48-binary decbench Cortex-M corpus: +1613 recovered ground-truth entries, 95.2 percent of the new entries are DWARF function addresses, ZERO split a real function body and ZERO previously-recovered entries are lost. The accepted entries are emitted as an additive fact stream and never re-seed the recursive-descent walk, which is what makes never-removes-an-entry a property of the wiring. ARM-only and Listing-tier, so it is a strict no-op on every other architecture and without --option listing on. Output-changing (it discovers more functions) => default-OFF.
- **When to flip:** Flip on for a stripped ARM firmware image whose callback / vtable / ISR-handler functions produce no output at all: the tell-tale is a hole in the decompilation, where kuna emits a function at one address and the next at a higher one with real code in between (a 4-byte movs r0,#0; bx lr driver stub, an exception handler that is a bare bx lr). Composes with the listing + funcstart_patterns + aif set the decompile-all/functions drivers already inject on non-x86-64, and with cortexmvectors; needs --option listing on to do anything. Off (default) keeps the prologue-plus-three-instruction shape test, so any binary that already worked is byte-identical either way.
- **Where / provenance:** P1/code-data-partition · kuna · analysis-enablement · kuna-analysis-ptrentry
- **Example:** `--option ptrentry on`

### `poolentry` -- on | off, default `off`

- **Symptoms:** a stripped arm function list carries sub_<addr> entries two bytes below the real function; the same arm function appears twice at addresses two bytes apart; a decompiled arm function returns a constant or drops its first parameter because of a dead leading movs; kuna reports far more functions than the binary has on bare-metal arm; an arm function entry lands one halfword before the real prologue; a32 u-boot recall is far below the cortex-m images.
- **What it does:** Reconstruct the ARM PC-relative literal pools that kuna's Listing never defines as data, and use them to fix the two defects that follow. `run_aif` slides its gap cursor ONE BYTE at a time with no instruction-alignment filter, because `first_undefined_after` is byte-granular by construction. A literal pool is data, so it is an undefined gap; the cursor probes every byte of it and accepts the first one whose 2-mnemonic fingerprint clears the histogram threshold. On STM32 Thumb the pool words are SRAM addresses 0x2000xxxx whose HIGH halfword decodes as movs r0,#0, so the accepted entry lands at pool_word+2, one halfword before the real function, and on accept the cursor jumps past the body so the true entry is never probed. In A32 the whole word decodes, so the phantom lands on the word itself. Upstream Ghidra does not have this: its reference analyzer defines pc-relative literal targets as DATA before AIF runs. This pass recovers the missing definition from the references that are there - every resolved [0xADDR] operand and every [pc,#imm] literal load, including the second word of a 64-bit vldr/ldrd - and takes maximal stride-4 runs of referenced words as the pools, which is strictly more conservative than an ELF $d mapping-symbol oracle. Two consumers. RECALL: emit an entry fact at the first address after a pool that abuts a return-class terminal, when it is undefined and passes AIF's own fingerprint and valid-subroutine tests; purely additive, so never-removes-an-entry is a property of the wiring as in ptrentry and tailcallentry. PRECISION: drop an AIF accept that lies inside an inferred pool, but ONLY when the pool end carries a replacement entry, which makes the removal a MOVE rather than a delete - unpaired it destroys 531 real bodies, paired it destroys zero. Measured on 98 stripped ARM images (48,553 DWARF addresses): 191 new entries, 188 ground truth (98.4 percent), ZERO ground-truth entries lost, ONE real body split, and 4,220 phantom entries retired of which zero are ground truth and 4,218 were strictly inside a real function body, with ZERO real bodies made unreachable. Cortex-M recall 97.25 to 97.43 percent, u-boot A32 58.35 to 60.65 percent, and the share of reported functions that are real DWARF entries 76.4 to 82.3 percent. ARM-only in effect: on 20 x86-64 ELFs and 8 i386 PE images it adds and removes exactly zero entries, because x86-64 RIP-relative loads target .rodata rather than .text interstices and i386 has no PC-relative addressing at all. Inert without --option listing on and without aif.
- **When to flip:** Flip on for a stripped ARM firmware image whose function list is full of sub_<addr> entries two bytes below a real function, or whose functions each appear twice at addresses two bytes apart. The tell-tale in the C is a leading dead movs r0,#imm that kuna then constant-folds through the body, so a function loses a parameter or returns a constant. Also the only thing in the ARM entry sequence that moves an A32 image such as u-boot. Off (default) keeps the current AIF accept list exactly, so any binary that already worked is byte-identical either way.
- **Where / provenance:** P1/code-data-partition · kuna · analysis-enablement · kuna-analysis-poolentry
- **Example:** `--option poolentry on`

### `arm_markers` -- on | off, default `on`

- **Symptoms:** thumb code misdecoded as arm garbage instructions; $t/$a mapping symbols ignored so the wrong decode mode applies.
- **What it does:** Run the ARM/Thumb decode-mode marker pass: paint the SLEIGH TMode context from ARM mapping symbols ($t/$a) + the STT_FUNC odd-address (LSB=1 => Thumb) convention so Thumb code decodes as Thumb. ARM-only; a no-op on every other language.
- **When to flip:** On (default) decodes Thumb regions as Thumb on ARM; off leaves the default (ARM) decode mode.
- **Where / provenance:** P1/code-data-partition · kuna · analysis-enablement · kuna-analysis-armmarkers
- **Example:** `option arm_markers off`

### `mips_gp` -- on | off, default `on`

- **Symptoms:** unresolved *(gp + offset) loads on mips; got/.sdata references never fold to real addresses in a pic mips binary.
- **What it does:** Run the MIPS $gp-recovery pass: seed t9 = func_entry as a tracked register value at each MIPS function entry (the PIC jalr t9 ABI convention) so a PIC prologue's addu gp,gp,t9 folds to the real $gp and $gp-relative GOT/.sdata loads resolve. MIPS-only; a no-op on every other language.
- **When to flip:** On (default) resolves $gp-relative loads on MIPS; off leaves the raw $gp (unresolved *(gp + offset)).
- **Where / provenance:** P1/code-data-partition · kuna · analysis-enablement · kuna-analysis-mipsgp
- **Example:** `option mips_gp off`

### `i386_pie_plt` -- on | off, default `on`

- **Symptoms:** i386 pie libc calls render as sub_<addr> instead of exit/dcgettext; spurious do{}while(true) or goto loop after an unnamed exit stub in a 32-bit pie binary.
- **What it does:** Decode i386 PIE GOT-relative (jmp *disp(%ebx), FF A3 <disp32>) PLT stubs so dynamic imports (exit/dcgettext/__printf_chk/...) are named and known-no-return functions (exit) are flagged. The i386-PIE analog of the x86-64/aarch64 PLT decoders; i386-only, a no-op on every other language. Loader-tier: read via the kuna_i386_pie_plt env var (the PLT->name map is baked at load file).
- **When to flip:** On (default) names i386-PIE libc calls and lets exit be marked no-return (collapsing the spurious do{}while(true)/goto and restoring stack recovery); off leaves the GOT-relative stubs as sub_<addr> and the bogus fall-through loop (the pre-fix rendering).
- **Where / provenance:** P1/external-refinement · angr · correctness-fix · kuna-analysis-i386pieplt
- **Example:** `option i386_pie_plt off`

### `relocrebase` -- on | off, default `on`

- **Symptoms:** kuna functions on a .o or .obj lists phantom sub_<addr> entries below 0x400000 beside the real ones; functions and decompile-all disagree on how many functions a relocatable object has; a -g .o names one function at address 0 and leaves the rest sub_<addr>; string literals and dwarf-named globals in a .o never attach to the loaded image.
- **What it does:** Rebase the load-time analysis facts of a relocatable object (ELF ET_REL .o, COFF .obj) into the loaded image's address space. The loader lays such an object out synthetically above RELOC_BASE (relocobjects), but the analysis passes re-parse the same file and compute PRE-LINK, section-relative addresses, so two address spaces mix in one inventory. This re-presents the object to the analyzer tier before any pass reads it: each laid-out section carries the loader's relocated bytes and its load VMA, each unlaid .debug_* section has its relocations applied, and every ELF symbol is shifted by its own section's delta (sections are laid out non-contiguously, so there is no single global offset). A fact that still lands in no laid-out section is dropped rather than passed through unrebased. Loader-tier: read via the kuna_relocrebase env var (the analyzer tier runs inside load file).
- **When to flip:** On (default) makes kuna functions and kuna decompile-all agree on a .o/.obj, and attaches that object's DWARF names, string literals and named globals to the loaded image. Flip off to restore the pre-fix behavior -- phantom sub_<section-offset> entries beside every real function, one DWARF function at address 0, and no string/data attachment.
- **Where / provenance:** P1/code-data-partition · kuna · correctness-fix · GH-289
- **Example:** `option relocrebase off`

### `dynrelocs` -- on | off, default `on`

- **Symptoms:** a call in a pie binary renders as (*dat_<addr>)(...) with no name although the callee is a named function in the same image; a got slot reads back as 0 in the decompiled output; reading a relocated function-pointer table in a pie binary yields all zeroes.
- **What it does:** Apply a LINKED image's dynamic relocations (.rela.dyn / .rel.dyn / .rela.plt) when the loader snapshots it, and mark the slots PT_GNU_RELRO freezes as constant. kuna maps the PT_LOAD bytes the linker wrote, but a PIE (ET_DYN) or dynamically linked ET_EXEC leaves every R_*_RELATIVE / GLOB_DAT / JUMP_SLOT slot at 0 for the run-time loader, so the whole .got reads back null and a call through a function pointer the linker itself resolved renders (*dat_<addr>)(...). This fills the slots in (RELATIVE = load bias + addend; GLOB_DAT/JUMP_SLOT only when the symbol is DEFINED in this same image, so imports and the PLT naming path are untouched) and reports the written slots that PT_GNU_RELRO covers as constant, which is what lets the engine fold the load and resolve the call to its name. x86-64, AArch64, i386 and ARM; both RELA and REL (in-place addend) tables. Distinct from relocobjects/relocrebase, which own the pre-link ET_REL .o path. Loader-tier: read via the kuna_dynrelocs env var (the image bytes are snapshotted at load file).
- **When to flip:** On (default) names calls made through a relocated GOT slot in any PIE, and gives the relocated value to anything that reads that memory; an unrelocated slot holds 0, which the run-time image never holds, so this is a correctness fix rather than a judgement call. Flip off to restore the pre-fix bytes exactly - every GOT slot back to 0 and every such call back to (*dat_<addr>)(...).
- **Where / provenance:** P1/code-data-partition · kuna · correctness-fix · kuna-analysis-dynrelocs
- **Example:** `option dynrelocs off`

### `ifuncfpret` -- on | off, default `off`

- **Symptoms:** a glibc math/mem/str wrapper tail-jumps to `(*dat_...)(...)` with the callee dropped; an x86-64 IFUNC .plt.sec stub is not a discovered function; xmm0 read uninitialized after calling a void-typed ifunc-dispatching wrapper.
- **What it does:** Name the x86-64 IFUNC (R_X86_64_IRELATIVE) .plt.sec/.iplt PLT stubs the symbol-keyed PLT pass leaves undiscovered (an IRELATIVE slot carries no symbol, only its resolver's address), so a tail `jmp` to one is recovered as a tail call (tailcalljump) to a discovered `ifunc_<resolver>` function instead of flowing into the stub and rendering `(*dat_...)(...)`. Loader-tier: read via the kuna_ifuncfpret env var (the PLT->name map is baked at load file). Stage A of the ifuncfpret chain; the FP-return-type recovery (a Ghidra-divergent indirect-tail-return change) is the documented follow-up.
- **When to flip:** A glibc math/mem/str wrapper (e.g. sub_15620 / log) tail-jumps to an IFUNC stub and renders `(*dat_...)(...)` with the callee dropped; on names the stub `ifunc_<resolver>` and recovers the tail call. x86-64 only; default off (opt-in, synthetic stub names).
- **Where / provenance:** P1/external-refinement · ida · correctness-fix · interp-bee-ifunc-fpret
- **Example:** `option ifuncfpret on`

### `mips_isa` -- on | off, default `on`

- **Symptoms:** mips16/micromips function body misdecoded as mips32 garbage; odd-address mips functions decode in the wrong isa mode.
- **What it does:** Run the MIPS16 ISA_MODE decode-mode marker pass: paint the SLEIGH ISA_MODE context at each MIPS16e/microMIPS function entry (marked by the STT_FUNC LSB-set address OR st_other & 0xf0 = STO_MIPS_MIPS16/MICROMIPS) so the body decodes in the alternate ISA instead of being misdecoded as MIPS32 (the MIPS analog of arm_markers' TMode painting; the kuna analog of Ghidra's MIPS_ElfExtension.applyIsaMode). MIPS-only; a no-op on every other language.
- **When to flip:** On (default) decodes MIPS16 regions as MIPS16 on MIPS; off leaves the default (MIPS32) decode mode (a MIPS16 function misdecodes).
- **Where / provenance:** P1/code-data-partition · kuna · analysis-enablement · kuna-analysis-mipsisa
- **Example:** `option mips_isa off`

### `dwarf` -- on | off, default `on`

- **Symptoms:** a -g binary still shows default names and inferred types; dwarf function/global names and typed signatures ignored.
- **What it does:** Run the DWARF recovery analysis pass: recover function/global names and typed signatures from .debug_* sections (the kuna analog of Ghidra's DWARFAnalyzer). Skips cleanly on a non-DWARF binary.
- **When to flip:** On (default) applies DWARF names/types; off ignores debug info (names/types come from symbols + inference only).
- **Where / provenance:** P1/external-refinement · kuna · analysis-enablement · kuna-analysis-dwarf
- **Example:** `option dwarf off`

### `datasyms` -- on | off, default `on`

- **Symptoms:** libc globals render as dat_<addr> where every other decompiler prints stderr/stdout/optind; fprintf stream argument shows a raw dat_ address so error paths are indistinguishable from output paths; a stripped binary's copy-relocated externs unnamed; an unstripped dwarf-less binary's own statics render dat_<addr>; decompile-project exports reference undeclared dat_<addr> identifiers where a stdio global belongs.
- **What it does:** Name data globals from the ELF symbol table: the loader reads the data half of the same .symtab/.dynsym walks that have always named functions -- every defined, named STT_OBJECT entry with a non-zero st_size, @VERSION stripped, deduplicated by address (.symtab before .dynsym) -- and the engine installs each as a named undefined<size> global with namelock only. The arm commits LAST, after the DWARF globals and the detected string literals, and skips any address a richer source already claimed, so what it fills is exactly the set neither reaches: a copy-relocated libc extern (stderr, stdout, stdin, optind, optarg, __progname) has a real .bss address and a .dynsym entry but no DIE in the program's own .debug_info, and a stripped or DWARF-less binary's own statics have only .symtab. Zero-size entries are dropped (the linker's section-boundary markers __bss_start/_edata/_end are exactly the sizeless ones). The st_size extent is what makes the covering-symbol query match at the real access width, so an 8-byte load of stderr finds the symbol instead of falling back to dat_<addr>.
- **When to flip:** On (default) renders symbol-table-named data globals by name, matching IDA Pro and Ghidra -- fprintf(stderr, ...) reads as an error path instead of fprintf(dat_61a0, ...). Flip off to restore raw dat_<addr> rendering for every global the DWARF pass does not name -- e.g. to see exactly which names came from the symbol table rather than from debug info, or if a hostile/corrupt symbol table plants misleading names.
- **Where / provenance:** P1/external-refinement · kuna · analysis-enablement · GH-184
- **Example:** `--option datasyms off`

### `dwarf_lines` -- on | off, default `off`

- **Symptoms:** no /* file:line */ source-location comments in the output; want each instruction annotated with its dwarf source line.
- **What it does:** Run the DWARF .debug_line source-line comment pass: parse the .debug_line program, map each instruction PC to its source file:line, and attach it as an instruction comment so the decompiled output carries the source location (the kuna analog of Ghidra's DWARFLineInfoCommentScript). DISABLED by default (it changes the output by adding /* file:line */ comment lines). Skips cleanly on a binary without .debug_line.
- **When to flip:** Off (default; byte-identical output). Flip on to annotate each decompiled instruction with its DWARF source file:line.
- **Where / provenance:** P1/external-refinement · kuna · analysis-enablement · kuna-analysis-dwarf-lines
- **Example:** `option dwarf_lines on`

### `cppproto` -- on | off, default `on`

- **Symptoms:** c++ member functions decompile with a0/a1 parameters despite -g debug info; Foo *this renders as void *; a namespaced or templated function loses its dwarf prototype; one unmappable parameter type discards the whole dwarf signature.
- **What it does:** Recover C++ function prototypes from DWARF: resolve a subprogram DEFINITION through its one-hop DW_AT_specification/DW_AT_abstract_origin link (an out-of-line member definition carries no DW_AT_name of its own, so the name-only walk drops it whole), qualify the name by its namespace/class ancestry, map DW_TAG_class_type like a structure and a C++ reference like a pointer, degrade a single unmappable parameter type instead of discarding the whole signature, and bind the recovered prototype by entry ADDRESS rather than by name.
- **When to flip:** On (default) gives a -g C++ binary its real signatures: Account::deposit(Account *this,int amount) instead of Account::deposit(int4 *a0,int4 a1). Flip off to restore the name-only DWARF walk (every out-of-line member function loses its parameter names, types and stack locals).
- **Where / provenance:** P1/external-refinement · kuna · analysis-enablement · kuna-analysis-cppproto
- **Example:** `option cppproto off`

### `typedepth` -- on | off, default `on`

- **Symptoms:** a dwarf-typed local or parameter renders void * despite -g debug info; const char ** or char *const [] loses its element type; a typedef'd struct pointer degrades to void *; a global array of string pointers is marked one byte wide; deep pointer chains (char ***) truncate to void **.
- **What it does:** Resolve a DWARF type DIE chain to its full depth, guarded by upstream's per-DIE re-entry counter (DWARFDataTypeImporter.trackRecursion) instead of a flat three-hop budget that counted the transparent typedef/const/volatile/restrict links as well. The budget was an unfaithful reduction of the cycle guard: it fired on ordinary declarations (const char **, const size_t *, char *const [], char ***, a const member function's this), which then fell back to void, so a -g binary lost the concrete type of stack locals, globals and deep-pointer parameters. Also collapses the transparent qualifier hops on the C path so an anonymous aggregate is interned under its typedef name (mbstate_t) rather than the shared anon_struct.
- **When to flip:** On (default) types a -g binary's locals/globals/parameters from the debug info: char **authors, const size_t *pn, mbstate_t *ps instead of void *. Flip off to restore the pre-fix three-hop budget (deep-enough DWARF types truncate to void).
- **Where / provenance:** P1/external-refinement · kuna · analysis-enablement · kuna-analysis-typedepth
- **Example:** `option typedepth off`

### `cppsig` -- off | proven | inferred, default `proven`

- **Symptoms:** c++ member functions on a stripped binary decompile with a0/a1 parameters; this renders as int8 * or unsigned long instead of the class type; a mangled symbol names the function but not its parameter types; no signature recovery on a stripped c++ shared library.
- **What it does:** Apply the DEMANGLED C++ signature - the class type for `this` plus the declared parameter types - to every function whose mangled symbol survives, which on a STRIPPED C++ shared library is the whole exported API in .dynsym and the only signature source left. Parses the full c++filt-style form (the same string the Ghidra GNU demangler parses), maps a class-shaped parameter to a named opaque structure reachable only as a pointee, refuses any by-value aggregate or overloaded operator, and parks the prototype by entry ADDRESS. The return type is deliberately NOT applied: Itanium encodes one only for a template function, so the function keeps whatever kuna's own recovery finds. Three-valued because Itanium mangling cannot tell a static member function from a non-static one and adding a `this` that is not there shifts every following parameter: `proven` applies only the shapes the mangling entails (constructors, destructors, cv-/ref-qualified members, unqualified globals, explicit MSVC access/static/convention), `inferred` also decides the ambiguous nested names from class evidence mined out of the binary's own symbols.
- **When to flip:** proven (default) is precision 1.0000 / recall 0.7093 on google/leveldb: `void leveldb::Cache::~Cache(Cache *this)` instead of `void leveldb::Cache::~Cache(unsigned long a0)`. Raise to inferred for precision 0.9278 / recall 0.9978 - it additionally recovers plain (non-const, non-ctor) member functions like `leveldb::TableBuilder::WriteBlock(TableBuilder *this,BlockBuilder *a1,BlockHandle *a2)` and namespaced free functions like `leveldb::NewMemEnv(Env *a0)`, at the cost of a spurious `this` on a static member. Flip off to restore name-only demangling.
- **Where / provenance:** P1/external-refinement · kuna · analysis-enablement · kuna-analysis-cppsig
- **Example:** `option cppsig inferred`

### `callfixup` -- on | off, default `on`

- **Symptoms:** mcount/__fentry__ profiling calls clutter every -pg function prologue; cspec call-fixup targets rendered as plain calls instead of dissolved.
- **What it does:** Run the call-fixup analysis pass: tag each function whose name matches a cspec call-fixup <target> (e.g. the -pg mcount/__fentry__ profiling stubs) so the engine replaces the CALL with the fixup body (the kuna analog of Ghidra's CallFixupAnalyzer).
- **When to flip:** On (default) dissolves matched fixup calls; off leaves the raw CALL (e.g. the mcount call stays).
- **Where / provenance:** P1/external-refinement · kuna · analysis-enablement · kuna-analysis-callfixup
- **Example:** `option callfixup off`

### `addrtable` -- on | off, default `off`

- **Symptoms:** absolute function-pointer table in rodata never recognized; indirect calls through an unrecovered address table in a stripped target.
- **What it does:** Run the address-table analysis pass: scan .rodata/.data for a run of pointer-width values landing inside an executable section (an absolute address/jump table) and emit Data symbols + a readonly range. DISABLED by default (matches Ghidra AddressTableAnalyzer.setDefaultEnablement(false); the pointer-run scanner over-accepts).
- **When to flip:** Off (default, parity + over-acceptance risk); flip on to recover absolute pointer tables in a stripped target.
- **Where / provenance:** P1/code-data-partition · kuna · analysis-enablement · kuna-analysis-addrtable
- **Example:** `option addrtable on`

### `operand_refs` -- on | off, default `off`

- **Symptoms:** an immediate operand pointing at a rodata string renders as a bare number; string argument of a no-prototype call not typed as char[N].
- **What it does:** Run the scalar/operand reference-markup pass (ScalarOperandAnalyzer/ElfScalarOperandAnalyzer): linear-decode the executable sections and, for each scalar immediate operand that points into allocated read-only data (passing the <4096/byte-mask value filter and the .got/.plt exclusion), plant a typed char[N]+readonly fact so the printer renders the reference as a string literal. DISABLED by default (Ghidra ScalarOperandAnalyzer.getDefaultEnablement = !isElf, i.e. off for every ELF; the ELF subclass only removes bad .got/.plt refs kuna never creates; the useful product is already delivered by s1_strings + libproto typing; a per-instruction immediate scan over-accepts).
- **When to flip:** Off (default; net-negative for an ELF decompiler per docs/history/analysis-port-buildplan.md §1.2 — ELF-default-off upstream, covered-elsewhere, over-acceptance-prone). Flip on to type residual short / s1_strings-missed read-only string operands pointed at by a no-prototype call.
- **Where / provenance:** P1/code-data-partition · kuna · analysis-enablement · kuna-analysis-operand-refs
- **Example:** `option operand_refs on`

### `formatstring` -- on | off, default `off`

- **Symptoms:** printf/scanf variadic arguments render untyped at the call site; %d and %s arguments carry generic types instead of int/char *.
- **What it does:** Type printf/scanf-family variadic arguments per call site (FormatStringAnalyzer half B, DecompilerDependent): after the first decompile, read the format-string constant at each printf/scanf call, parse its specifiers, and install a per-call-site prototype override (fixed types = format-derived, varargs closed), then re-decompile so the variadic args render typed. DISABLED by default (matches Ghidra FormatStringAnalyzer.setDefaultEnablement(false)).
- **When to flip:** Off (default; the decompile→override→re-decompile loop is inert and every parity gate is byte-identical). Flip on to recover typed printf/scanf varargs (e.g. %d→int, %s→char*) in a real-ELF target.
- **Where / provenance:** P1/external-refinement · kuna · analysis-enablement · kuna-analysis-formatstring
- **Example:** `option formatstring on`

### `listing` -- on | off, default `off`

- **Symptoms:** noreturn discovery inert on a stripped binary; analysis passes that need whole-image xrefs do nothing; no program-wide instruction/xref/function model for consumer passes.
- **What it does:** Build the Listing/xref disassembly tier: a program-wide recursive-descent disassembly over the loadimage bytes (reusing the SLEIGH decoder + the lifted S2 flow classifier) producing three read-only sub-models (instruction / cross-reference / discovered-function) shared with the consumer analysis passes. DISABLED by default (the Listing is never built; real-ELF path only, so every parity gate is byte-identical).
- **When to flip:** Off (default; the Listing is not built and there is zero decode cost). Flip on to make the program-wide instruction/xref/function model available to consumer passes (e.g. discovered-no-return) on a real-ELF target.
- **Where / provenance:** P1/code-data-partition · kuna · analysis-enablement · kuna-analysis-listing
- **Example:** `option listing on`

### `fast_funcdisc` -- on | off, default `off`

- **Symptoms:** fast decompile-project emits only import veneers and the binary entry; internal direct callees have no project bodies; function reachable through a callback or vtable pointer is absent while explicit --addr decompiles it.
- **What it does:** Discover real function bodies for latency-sensitive whole-project exports without the exhaustive prologue and AIF gap scans. Starting from loader-backed entries, build one recursive Listing walk that promotes every direct CALL target. Then recover indirect-only callbacks from absolute code pointers in non-executable initialized data. On non-ARM objects, admit a target only when its first two decoded mnemonics match a fingerprint shared by at least four already-reached functions and a bounded control-flow probe validates a >2-instruction subroutine with a terminating path or informative call/edge into known code; ignore tables above 256 slots, require two distinct eligible tables when the image has more than 512 unique candidates, and validate at most 4096 candidates. ARM instead reuses the established Thumb-pointer prologue oracle. Pointer-derived roots are emitted without recursively walking their disconnected callees. Default-off as an output-changing analysis; the fast preset enables it while keeping listing, full prologue patterns, and exhaustive AIF off.
- **When to flip:** Enabled by fast mode for unfiltered or name-selected decompile-all/decompile-project/functions so a stripped binary does not collapse to imports plus its entry point. Flip off to restore the old metadata-only fast inventory, or when even the rooted whole-image recursive decode is too costly. Explicit --addr selections skip the preset's discovery work unless this option is explicitly turned on.
- **Where / provenance:** P1/code-data-partition · kuna · correctness-fix · kuna-fast-function-discovery
- **Example:** `--mode fast --option fast_funcdisc on`

### `fid` -- on | off, default `off`

- **Symptoms:** stripped static-linked library function stays sub_<addr> although its fingerprint is known; no .fid database renames applied in a stripped binary.
- **What it does:** Run the FID fingerprint matcher (a Listing/xref consumer): the kuna analog of Ghidra's FID (Function-ID) identification analyzer. Over the built Listing, fingerprint each function with the byte-exact operand-masked FNV-1a64 hash (the MessageDigestFidHasher port, driving Sleigh::instruction_mask) and look the full hash up in a kuna `.fid` fingerprint database (named by the `kuna_fid_db` environment variable). When the full-hash bucket collapses to exactly one name (conservative — never guess on a tie), RENAME the matched function — but only when it still carries an engine FUN_*/sub_* placeholder (the label gate; a real .symtab/DWARF name is never overwritten). This re-identifies a function in a STRIPPED binary purely by its instruction-stream fingerprint (e.g. sub_4017c0 -> kuna_crc32). Requires the Listing (option listing on) and a configured `.fid` DB — a no-op when either is absent. DISABLED by default (real-ELF path only, so every parity gate is byte-identical).
- **When to flip:** Off (default; no Listing consumer runs and there is zero behavior change). Flip on (with option listing on, and `kuna_fid_db` pointing at a `.fid` database built by `kuna fid build`) to re-identify library functions in a STRIPPED real-ELF target by full-hash fingerprint — renaming a FUN_*/sub_* placeholder back to its known library name.
- **Where / provenance:** P1/external-refinement · kuna · analysis-enablement · kuna-analysis-fid
- **Example:** `option listing on --option fid on`

### `rtti` -- on | off, default `off`

- **Symptoms:** msvc c++ class names missing and vftables left as unnamed data; no Class::vftable or RTTI_ labels in a windows pe with polymorphic classes.
- **What it does:** Run the MSVC RTTI / vftable class-name recovery pass: the kuna analog of Ghidra's RttiAnalyzer (a Microsoft-PE analyzer). On a Windows PE, parse the run-time-type-information graph entirely in the loaded image (.rdata/.data): find the common type_info vftable (the pointer every RTTI0 TypeDescriptor shares), locate the TypeDescriptors carrying the `.?A...@@`-mangled class names, byte-search back to each CompleteObjectLocator (COL = ref - 12), validate the COL -> RTTI3 ClassHierarchyDescriptor -> RTTI2 BaseClassArray -> RTTI1 BaseClassDescriptor -> RTTI0 TypeDescriptor reachability chain, and demangle each class name via the existing MSVC demangler (the Ghidra RttiUtil `??_R0...@8` wrap; zero new demangler code). Handles BOTH x86 (raw-VA inter-structure refs, RTTI0 name at offset 8) AND x64 (image-base-relative IBO32 disp32 refs, RTTI0 name at offset 16) via a refkind dispatch on is64Bit. Emits Data symbols labelling each class's vftable (`<Class>::vftable`), CompleteObjectLocator (`<Class>::RTTI_Complete_Object_Locator`), and TypeDescriptor (`<Class>::RTTI_Type_Descriptor`), so the C++ class names (Box/Shape) surface as recovered symbols and the virtual-dispatch metadata graph is named. PE-only (registered in passes_for only for a PE image, and the pass self-gates on PE in run). DISABLED by default (real-PE path only, output-changing: it adds named data symbols, so every ELF/XML parity gate is byte-identical).
- **When to flip:** Off (default; no RTTI pass runs and there is zero behavior change). Flip on for a Windows PE built by MSVC (or clang -target ...-windows-msvc) carrying polymorphic C++ classes, to recover the C++ class names and label the RTTI/vftable metadata graph (the raw .rdata bytes / unnamed DAT_* vtable become `Box`/`Shape` class names + `<Class>::vftable` / `<Class>::RTTI_*` labels).
- **Where / provenance:** P1/external-refinement · kuna · analysis-enablement · kuna-analysis-rtti
- **Example:** `option rtti on`

### `itaniumrtti` -- on | off, default `off`

- **Symptoms:** gcc/clang c++ class names missing and vtables left as unnamed DAT_ data; no Class::vtable or typeinfo labels in a stripped elf with polymorphic classes; virtual dispatch slot resolves to sub_<addr> with no class.
- **What it does:** Run the Itanium (GCC/Clang) RTTI + vtable recovery pass -- a capability Ghidra does NOT have (its RttiAnalyzer is a Microsoft-PE analyzer, and its GCC class recovery is script-tier: RecoverClassesFromRTTIScript / RTTIGccClassRecoverer, never auto-analysis), so on a stripped g++ binary Ghidra leaves the Shape vtable as DAT_00107a80. IDA Pro does reconstruct it, and this is kuna's analog, built against the Itanium C++ ABI. On an ELF, discovery is EXACT rather than a byte-pattern scan: the three __cxxabiv1 typeinfo vtables live in libstdc++, so every _ZTI... typeinfo object's leading vptr word is an UND dynamic relocation naming __class_type_info / __si_class_type_info / __vmi_class_type_info with addend 2*ptr -- the relocation's r_offset IS the typeinfo address and its symbol IS the flavour, and `.rela.dyn` is a loader input that `strip --strip-all` cannot remove from a shared object. A defined _ZTI... OBJECT symbol is a second discovery source (unstripped / static link), its flavour sniffed from the object's shape. Each typeinfo's _ZTS... type-name string is demangled to the class name by wrapping it back into the _ZTS symbol form and reusing the existing cpp_demangle Itanium arm (the exact analog of the MSVC ??_R0...@8 wrap; ZERO new demangler code), and its base list gives the inheritance DISPLACEMENTS. Vtables are then reached from the typeinfo, not guessed: one scan finds every pointer slot holding a discovered typeinfo address (a sub-vtable's second header word), and two exact ABI constraints reject the coincidental hits -- offset-to-top is always <= 0, and a real sub-vtable has >= 1 slot pointing into an executable section. A slot whose file word is 0 but which carries a dynamic relocation is an IMPORTED virtual method (__cxa_pure_virtual, a base method in another image); the walk steps over it instead of terminating, so an abstract interface keeps its true extent. Emits Data symbols `<C>::typeinfo`, `<C>::typeinfo_name`, `<C>::vtable`, and for a secondary sub-vtable the base subobject its displacement identifies (`Widget::vtable_for_Drawable`); a Function symbol `<C>::vtable_<i>` per virtual slot, with an inherited slot attributed to the class that DEFINES it rather than to whichever vtable the scan reached first (so Shape::perimeter, repeated in Circle's and Square's tables, is named once for Shape); and the slot arrays as read-only ranges. ELF-only (registered in passes_for only for BinaryFormat::Elf, and the pass self-gates on ELF in run). DISABLED by default (real-ELF path only, output-changing: it adds named data and function symbols, so every XML parity gate is byte-identical).
- **When to flip:** Off (default; no Itanium RTTI pass runs and there is zero behavior change). Flip on for a GCC/Clang-built C++ ELF carrying polymorphic classes -- especially a STRIPPED one, where the relocation anchor is the only surviving type information -- to recover the class names, the inheritance graph and the vtables (an unnamed DAT_* vtable slot becomes `Shape::vtable_2`, the raw .data.rel.ro bytes become `Shape::typeinfo` / `Shape::vtable`). It recovers NOTHING from a binary built -fno-rtti (google/leveldb is such a build: 18 _ZTV symbols, zero _ZTI/_ZTS, zero __cxxabiv1 relocations), and nothing from plain C.
- **Where / provenance:** P1/external-refinement · kuna · analysis-enablement · kuna-analysis-itaniumrtti
- **Example:** `option itaniumrtti on`

### `aif` -- on | off, default `off`

- **Symptoms:** function reachable only through a rodata function-pointer table never discovered; undefined gap between functions that clearly holds code; call *reg targets missing from the function list.
- **What it does:** Run the Aggressive Instruction Finder gap-walk (the third Listing/xref consumer): the kuna analog of Ghidra's AggressiveInstructionFinderAnalyzer, a speculative gap-filler. Over the undefined gaps between discovered functions, speculatively decode each gap start and accept it as a NEW function entry when it BOTH (a) disassembles into a valid subroutine (followSubFlows reaches a clean RET without hitting a bad/undecodable byte or an out-of-range flow, > 2 instructions) AND (b) matches a function-start byte fingerprint (mask-histogram) shared by >= 4 already-discovered functions. Finds functions reachable ONLY through an indirect/data path (a .rodata function-pointer table) that entry discovery + funcsyms + .eh_frame miss. Emits the discovered starts as the existing entries fact -> the existing name_function/add_function commit seam (no new commit arm). Requires the Listing (option listing on) — a no-op when the Listing is absent. DISABLED by default (Ghidra ships setDefaultEnablement(false) with the warning 'IT MAY CREATE A LOT OF BAD CODE!'; a speculative gap-filler can create false-positive functions; real-ELF path only, so every parity gate is byte-identical).
- **When to flip:** Off (default; no Listing consumer runs and there is zero behavior change, and a speculative gap-filler can create false-positive functions). Flip on (with option listing on) when a function is reachable ONLY through an indirect/data path (its address taken into a .rodata function-pointer table and called via call *reg with an opaque index) — so it is in no symbol table, has no .eh_frame FDE, and no static CALL edge points at it, leaving it an undefined gap that entry discovery cannot reach. AIF's fingerprint+validity gap-walk recovers it as a named sub_<addr>.
- **Where / provenance:** P1/code-data-partition · kuna · analysis-enablement · kuna-analysis-aif
- **Example:** `option listing on --option aif on`

### `aifstrict` -- on | off, default `off`

- **Symptoms:** a stripped binary reports far more functions than it has; a discovered function starts in the middle of another functions body; a sub_ entry lands on an ordinary call or mov instruction rather than a prologue; two overlapping functions decompile the same instructions twice; aif adds thousands of entries a reference decompiler does not agree are functions; an arm function entry lands one halfword before the real prologue.
- **What it does:** Advance the Aggressive Instruction Finder gap cursor to the next 4-byte boundary instead of the next byte, so it stops planting function entries in the middle of function bodies (GH-299). run_aif walks the undefined-byte partition ONE BYTE at a time, because first_undefined_after is byte-granular by construction, so every byte of every hole is a candidate function start and the two acceptance tests - a 2-mnemonic prologue fingerprint shared by at least 4 discovered functions, and a valid-subroutine decode - are applied to addresses that cannot be instruction boundaries at all. A candidate starting mid-instruction reads the tail of one encoding plus the head of the next, and that synthetic pair matches a common prologue about as often as a real one does. On the reported 3.4 MB stripped i386 PE, AIF contributes 5,913 entries of which 2,067 land inside a function IDA decodes as one body, and 737 of those are inside a function kuna itself already has an entry for. With this on, a candidate is probed only when it is 4-byte aligned OR is the first byte of its hole: a hole boundary is evidence, since the recursive-descent walk decoded up to exactly there and stopped, while an interior byte the cursor slid onto is a guess. The exemption is also what keeps the aif_gap_x86_64 fixture green, whose indirect-only function at 0x13ae is 2-mod-4 but IS a hole start. 4 bytes on every architecture, deliberately: 16-byte alignment kills 91 percent of the bad Cortex-M entries but takes 80 percent of the real ones with it. The removal is not only a precision win: because an accept advances the cursor past the accepted body, a phantom accepted one halfword inside a literal pool CONSUMES the real function behind it, so declining that probe recovers the true entry. Measured bidirectionally over 110 stripped non-x86-64 decbench binaries against each stripped image unstripped twin symbol table: mid-body entries 11,010 to 6,728 (a 38.9 percent cut) and ground-truth recall 44,613 to 44,957 of 50,724 (plus 344). On the two large PE witnesses, mid-body entries 339 to 135 on a 590 KB i386 crackme scored against its unstripped twin (zero of its 636 AIF-recovered real entries lost, 9 gained) and 421 to 293 on IDA Pro 9.2 win32_remote32.exe (3 of 157 lost). u-boot A32, the image DIV-20 exists for, gains 56 functions at O2 and is unchanged at O2-noinline. The worst single-image cost is CMSIS_DAP.axf, where only 73.5 percent of ground-truth entries are 4-aligned and three Thumb functions at 2-mod-4 interior addresses are lost. This does NOT reach the acceptance bar pre-registered for the fix (a 60 percent corpus cut and 100 or fewer crackme mid-body entries), so it ships opt-in rather than as the default and GH-299 stays open. The issues other direction - reject a candidate bracketed by a known function - was implemented in two forms and measured OUT: on the Listings entry-ordered function model it costs u-boot 195 of 1,759 recovered functions, and A32 recovery is AIFs entire remaining justification. Inert without aif and without --option listing on.
- **When to flip:** Off (default) keeps the byte-granular cursor and reproduces the GH-299 measurement exactly. The `aggressive` preset carries it, which is where every number above was measured. Flip on for a stripped image whose function list is far longer than the binary can hold, or that contains a sub_<addr> sitting on an ordinary call or mov in the middle of another function - and especially when two overlapping entries decompile the same instructions twice. Flip it back off if a function you know is real disappears on an architecture whose function starts are not 4-byte aligned (Thumb allows a 2-mod-4 start; three are lost on CMSIS_DAP.axf).
- **Where / provenance:** P1/code-data-partition · kuna · analysis-enablement · GH-299
- **Example:** `--option aifstrict on`

### `tailcallentry` -- on | off, default `off`

- **Symptoms:** routine reached only by a tail B produces no decompiled output at all; a shared ISR handler is swallowed by the two-instruction stub that branches to it; function entry missing where the previous function ends in an unconditional branch; kuna emits one oversized function where the ground truth has two.
- **What it does:** Recover the function entries the recursive-descent Listing walk absorbs at a tail call. walk.rs makes a new function only at a CALL target and treats every other flow target as a same-function successor, so a routine reached only by a tail `B` is swallowed by whichever function branched to it and never becomes a function of its own — 17.4% of the ARM Cortex-M entry-recall gap. This reads the COMPLETED walk (splitting at a tail call provably cannot change which instructions the walk decodes, because a function entry is walked and therefore decoded either way) and admits an unconditional-branch target as a NEW function entry when four containment guards hold: every predecessor of the target is an unconditional branch (no fall-through and no conditional-branch predecessor, which would make it ordinary intra-function flow); the branch leaves the caller's entry-ordered function region, so at least one other discovered entry lies between the branch and its target; the target's flow region reaches a RETURN or computed jump (terminating-routine validity, with no length floor, so a one-instruction `bx lr` handler qualifies); and the target does not open with a stack restore (a function does not begin by tearing down a frame it never built — that is the caller's shared epilogue). Emits the accepted targets as the existing entries fact and never rebuilds the Listing, so no already-discovered entry can be removed. ARM-only; requires the Listing (option listing on) — a no-op when the Listing is absent, so every parity gate is byte-identical.
- **When to flip:** Off (default; it discovers more functions, so it changes emitted C by construction). Flip on (with option listing on) on stripped ARM firmware when a routine reached only by a tail `B` — a shared ISR stub's handler, a `pop {r4,lr} ; b helper` tail call, a jump-thunk target — produces no output at all because the walk absorbed it into an earlier function. Measured on 96 Cortex-M images: +561 ground-truth functions, 94.6% of the new entries are real function starts, zero real bodies split, zero entries lost.
- **Where / provenance:** P1/code-data-partition · kuna · analysis-enablement · kuna-analysis-tailcallentry
- **Example:** `option listing on --option tailcallentry on`

### `gopclntab` -- on | off, default `on`

- **Symptoms:** stripped go binary renders sub_<addr> instead of main.main and runtime.* names; go package function names missing.
- **What it does:** Run the Go pclntab function-name recovery pass: when the binary is Go (detect_compiler == Go), locate and parse the embedded pclntab (PC->line/name table) and emit a function symbol per Go function, so main.main / runtime.* / package functions render NAMED instead of sub_<addr> (the kuna analog of Ghidra's GolangSymbolAnalyzer, name-recovery half). Handles the go1.2/go1.16/go1.18/go1.20 header magics. Registered ONLY for a Go binary, so it is a structural no-op on every non-Go target. The existing symbol commit arm installs the names idempotently (a real .symtab name still wins; only a stripped Go binary's sub_<addr> functions take the recovered name).
- **When to flip:** On (default) names Go functions from the pclntab (main.main, runtime.gopanic, ...); off leaves a stripped Go binary's functions as sub_<addr>.
- **Where / provenance:** P1/code-data-partition · kuna · analysis-enablement · kuna-analysis-gopclntab
- **Example:** `option gopclntab off`

### `objc` -- on | off, default `off`

- **Symptoms:** objective-c methods render as sub_<addr> instead of -[Class sel]; __objc_ metadata present but classes and selectors unnamed in a mach-o.
- **What it does:** Run the Mach-O Objective-C metadata recovery pass: when the binary is a Mach-O, walk the __objc_* ObjC 2.0 metadata web (the kuna analog of Ghidra's ObjcTypeMetadataAnalyzer, name-recovery half). From __objc_classlist (array of class_t*), per class follow data & FAST_DATA_MASK (~0x7) to the class_ro_t, read .name (class name) and .baseMethods (the - instance method_list_t), and follow class_t.isa to the metaclass baseMethods (the + class methods). Walk method_list_t handling BOTH the large absolute-pointer form and the small/relative form (entsizeAndFlags & 0x80000000), per method_t resolving the selector (ASCII), type encoding, and IMP (function VA). RENAME each IMP function -[Class sel] / +[Class sel] — but only when it still carries an engine FUN_*/sub_* placeholder (the label gate, the FID precedent; a real symbol is never overwritten) — plus emit _OBJC_CLASS_$_<name> + selector symbols. Selectors are plain ASCII, so no demangler is needed. Registered ONLY for a Mach-O binary, so it is a structural no-op on every non-Mach-O target. x86-64, no-chained-fixups path (the arm64 + LC_DYLD_CHAINED_FIXUPS resolver is a deferred follow-on). DISABLED by default (output-changing: renames IMP functions + adds symbols; real-binary-path only, so every parity gate is byte-identical).
- **When to flip:** Off (default; no IMP is renamed and there is zero behavior change). Flip on for a Mach-O Objective-C binary to recover method names — a stripped IMP renders -[Greeter greet:] instead of sub_<addr>, and the class/selector strings are named.
- **Where / provenance:** P1/external-refinement · kuna · analysis-enablement · kuna-analysis-objc
- **Example:** `option objc on`

### `pdb` -- on | off, default `off`

- **Symptoms:** stripped windows pe functions stay FUN_<addr> although a matching .pdb exists; pdb symbol names not applied to a pe.
- **What it does:** Run the PE PDB (Program Database) metadata recovery pass: on a Windows PE, recover function names from the external .pdb debug file (the kuna analog of Ghidra's PdbUniversalAnalyzer, name-recovery half). PDB is Windows' DWARF, but the debug info lives in a SEPARATE .pdb file the PE only fingerprints. Read the PE CodeView debug record ({guid|sig, age, .pdb path}), locate the .pdb (tier-1: the kuna_pdb_path environment variable, the exact s1_fid kuna_fid_db external-artifact precedent), and apply the FINGERPRINT GATE: open the supplied .pdb via the `pdb` crate and verify its pdb_information().guid/age matches the PE's CodeView record. A MISMATCH or ABSENT/unreadable .pdb emits nothing — never apply a wrong/stale PDB (the FID full-hash-match discipline of never applying wrong external knowledge). On a match, walk the global symbol stream (S_PUB32 publics + S_GPROC32 procedures), resolve each to its VMA (segment:offset -> RVA + the PE ImageBase), and RENAME each stripped function to its real name — but only when it still carries an engine FUN_*/sub_* placeholder (the label gate, the FID precedent; a real symbol is never overwritten). Registered ONLY for a PE binary, so it is a structural no-op on every non-PE target. This PR is NAME-level (stripped FUN_<addr> -> the real name); typed prototypes, typed stack locals, and source lines are the deferred PR-P2/P3. DISABLED by default (output-changing: renames stripped functions; real-binary-path only and inert without a fingerprint-matching .pdb, so every parity gate is byte-identical).
- **When to flip:** Off (default; no function is renamed and there is zero behavior change). Flip on for a stripped Windows PE that has a matching .pdb supplied via the kuna_pdb_path env var — a stripped FUN_<addr> renders its real source name (e.g. WinMain) recovered from the PDB. A .pdb whose GUID/age does not match the PE is rejected (no rename).
- **Where / provenance:** P1/external-refinement · kuna · analysis-enablement · kuna-analysis-pdb
- **Example:** `option pdb on`

### `relocobjects` -- on | off, default `on`

- **Symptoms:** a .o relocatable object fails with 'Unable to load N bytes at ...'; ET_REL object maps zero bytes so nothing decompiles; a COFF .obj lists only its first function (the rest collide at address 0).
- **What it does:** Load relocatable objects — ELF `ET_REL` `.o` files (no PT_LOAD program headers, so they map zero bytes under the upstream loader) and COFF `.obj` files (every section at VMA 0, so they map one section on top of another): synthesize a section layout above 0x400000, apply the relocations (R_X86_64_PC32/PLT32/32/32S/64, COFF DIR32/REL32, with REL-style in-place addends), rebase defined symbols, and bind undefined externs to synthetic call targets — so an object decompiles fully instead of failing with 'Unable to load N bytes at ...' (ELF) or exposing only its first function (COFF).
- **When to flip:** On by default (a pure capability: linked images of every format are byte-identical — they keep the mapped-image path — and only a pre-link object takes the new path, which is otherwise ZERO output on ELF and first-function-only on COFF). Set off to restore the upstream mapped-image-only loader (an ELF `.o` then errors; a COFF `.obj` falls back to its first section). NB: this gates the loader, which runs at `load file` BEFORE per-function options, so flip it (or set env KUNA_RELOC_OBJECTS=0) before loading the object.
- **Where / provenance:** P1/code-data-partition · angr · structure-recovery · angr-CLE-ET_REL
- **Example:** `option relocobjects off`

### `macho-arm64e` -- on | off, default `off`

- **Symptoms:** arm64e mach-o decoded with the generic v8A spec so pointer-auth ops are unmodeled; pac instructions in an apple-silicon binary not modeled by the loaded spec.
- **What it does:** Select the Apple-Silicon pointer-authentication SLEIGH spec for an arm64e Mach-O. When on, a Mach-O whose header reports cpusubtype CPU_SUBTYPE_ARM64E loads with the AARCH64:LE:64:AppleSilicon language (the vendored Apple-Silicon v8.5-A spec, which models the pointer-auth/AMX extensions) instead of the generic AARCH64:LE:64:v8A. Pointer authentication does NOT change import naming or symbols (the __stubs indirect-symbol walk is unaffected) — only the spec selection differs (design 3.7). Spec selection is a LOAD-time decision (language_id_for), which runs before any console `option` command, so the live gate is the KUNA_MACHO_ARM64E env var the kuna CLI exports when it sees `--option macho-arm64e on`; this option name also records the requested state on the Architecture for catalog consistency. Default-OFF (opt-in, multi-format loader PR-8): a non-arm64e Mach-O, a non-Mach-O target, and every ELF/PE/COFF path are byte-identical, so all parity gates are structurally untouched.
- **When to flip:** Off (default; an arm64e Mach-O loads with the generic v8A spec, exactly as any arm64). Flip on to decompile an arm64e (Apple-Silicon, pointer-auth) Mach-O with the AppleSilicon spec that models its extensions.
- **Where / provenance:** P1/code-data-partition · kuna · analysis-enablement · kuna-multiformat-macho-arm64e
- **Example:** `--option macho-arm64e on`

## Core rendering defaults

Part of the decompiler; not the control surface. Flip only to reproduce upstream Ghidra output byte-for-byte or for unusual downstream consumers.

### `compareform` -- canonical | original, default `original`

- **Symptoms:** comparison constant off by one versus upstream ghidra (x <= 9 vs x < 10); need the analysis-canonical compare form to diff against upstream ghidra output.
- **What it does:** Whether comparisons keep their source form (V <= c) or the analysis-canonical rewrite (V < c+1).
- **When to flip:** Flip to canonical only to reproduce upstream Ghidra output; original (default) is more faithful to source.
- **Where / provenance:** P3/comparison-canonicalization · ghidra-upstream · presentation-default · GH-558
- **Example:** `option compareform canonical`

### `arraynotation` -- on | off, default `on`

- **Symptoms:** &base[index] rendering unwanted; consumer expects raw pointer arithmetic; indexed array form where base + offset arithmetic is desired for diffing.
- **What it does:** Render standalone pointer arithmetic as &base[index] (on) vs base + index (off).
- **When to flip:** Flip off if the consumer expects raw pointer-arithmetic rendering; on (default) is more readable.
- **Where / provenance:** P9/pointer-notation · ghidra-upstream · presentation-default · GH-558
- **Example:** `option arraynotation off`

### `truthycond` -- on | off, default `on`

- **Symptoms:** if (x != 0) where idiomatic C reads if (x); if (p == NULL) where if (!p) is wanted; explicit zero comparison in conditions unwanted; truthy condition rendering needs to match upstream ghidra for diffing.
- **What it does:** Render a boolean-context zero comparison in truthy form (if (x) / if (!p)) vs the explicit comparison (if (x != 0) / if (p == NULL)).
- **When to flip:** On by default (DIV-36): idiomatic C for if/while/for/ternary conditions and &&/||/! operands. Flip off to reproduce upstream Ghidra's explicit comparisons or to diff against Ghidra output. Float compares, enum-typed zeros, and equate-named zeros always keep the explicit form; value uses (v = (x != 0)) are never rewritten.
- **Where / provenance:** P9/condition-form · kuna · presentation-default · kuna-cnorm-fmt
- **Example:** `option truthycond off`

### `braceelide` -- on | off, default `on`

- **Symptoms:** if (x) { stmt; } where braceless if (x) stmt; is wanted; single-statement if body keeps braces; braced one-liner bodies unwanted; brace style needs to match upstream ghidra for diffing.
- **What it does:** Render a single-statement if body braceless with the statement indented on the next line (on) vs always braced (off).
- **When to flip:** On by default (DIV-37): idiomatic C for one-statement then-bodies. Flip off to reproduce upstream Ghidra's braced form or to diff against Ghidra output. Only plain single-statement bodies elide: labels, comments, and multi-statement bodies keep their braces; the if (cond) goto L; one-liner and else-if collapse are unaffected.
- **Where / provenance:** P9/brace-form · kuna · presentation-default · kuna-cnorm-fmt
- **Example:** `option braceelide off`

### `warnstyle` -- inline | banner, default `inline`

- **Symptoms:** full-line WARNING banner comments clutter the output; warning text wanted inline at the end of the statement it describes; terse warning slugs unwanted; full upstream warning text needed; warning style needs to match upstream ghidra for diffing.
- **What it does:** Render analysis warnings as terse end-of-line // slugs on the line they describe (inline) vs full /* WARNING: ... */ banner lines (banner).
- **When to flip:** Inline by default (DIV-38): usage(1); // no-return, if (v8) { // branch-flip, prototype-line // early-return x3. Flip to banner to reproduce upstream Ghidra's full warning text or to grep for the long-form messages. Unknown warning texts keep their full text behind a warn: marker; user/dwarf_lines comments always keep banner lines.
- **Where / provenance:** P9/warning-style · kuna · presentation-default · kuna-cnorm-fmt
- **Example:** `option warnstyle banner`

### `thumbfuncptr` -- on | off, default `on`

- **Symptoms:** thumb function pointer renders as symbolic &fn[1] where the raw odd-address constant is wanted; callback constant on arm resolves to a function symbol plus one instead of a bare hex literal.
- **What it does:** Preserve a Thumb function pointer (fn|1) as a symbolic &fn[1] rather than a raw hex literal.
- **When to flip:** Flip off only to see the raw constant; on (default) recovers the function symbol on ARM/Thumb.
- **Where / provenance:** P5/const-pointer · ghidra-upstream · correctness-fix · GH-8471
- **Example:** `option thumbfuncptr off`

### `inferfuncentry` -- on | off, default `on`

- **Symptoms:** a bare constant equal to a function entry renders as the named function pointer; hex literal at a single-bit image base unexpectedly named as a function (flip off for the raw constant).
- **What it does:** Infer a function pointer when a constant equals an exact function entry at a single-bit image base (e.g. 0x100000).
- **When to flip:** Flip off only to reproduce the bare-literal form; on (default) names the function.
- **Where / provenance:** P5/const-pointer · ghidra-upstream · correctness-fix · GH-6930
- **Example:** `option inferfuncentry off`

### `booleanmask` -- on | off, default `on`

- **Symptoms:** want the raw (b<<k) s>> k sign-extension shift idiom visible instead of the folded comparison; flag-as-high-bit lowering (8051 style) hidden by the cleaned boolean compare.
- **What it does:** Fold the (b<<k) s>>k boolean sign-extension-mask idiom (flag-as-high-bit lowering) into a clean comparison.
- **When to flip:** Flip off only to see the raw shift idiom; on (default) cleans flag-modelled comparisons (8051 etc.).
- **Where / provenance:** P3/simplification-quiescence · ghidra-upstream · correctness-fix · GH-1282
- **Example:** `option booleanmask off`

### `ovlesssimplify` -- on | off, default `on`

- **Symptoms:** want the raw S/OV overflow-flag arithmetic visible instead of the folded signed compare; v850-style explicit flag compare collapsed to a clean signed less-than.
- **What it does:** Simplify the explicit S/OV-flag compare idiom into a direct signed comparison.
- **When to flip:** Flip off only to see the raw overflow-flag arithmetic; on (default) cleans V850-style compares.
- **Where / provenance:** P3/simplification-quiescence · ghidra-upstream · correctness-fix · GH-7190
- **Example:** `option ovlesssimplify off`

### `addcarrychain` -- on | off, default `on`

- **Symptoms:** want the raw CONCAT11(CARRY1(...)) intrinsics visible instead of one wide add; 8-bit adc carry-chain pair (6502 style) folded into a single 16-bit addition.
- **What it does:** Recover an 8-bit carry-chain (ADC) pair into a single wide add instead of CONCAT11(CARRY1(...)).
- **When to flip:** Flip off only to see the raw carry intrinsics; on (default) recovers wide arithmetic (6502 etc.).
- **Where / provenance:** P5/simplification-quiescence · ghidra-upstream · correctness-fix · GH-8913
- **Example:** `option addcarrychain off`

### `flagcompare` -- on | off, default `on`

- **Symptoms:** boolean shifted into the sign bit tested with a less-than-zero compare; N==V signed-overflow branch idiom rendered as raw flag arithmetic; want the explicit condition-flag math visible (flip off).
- **What it does:** Fold flag-modelled comparison idioms into clean compares: a boolean shifted into the sign bit ((b<<k) s< 0) and the N==V signed-overflow idiom (bra ge).
- **When to flip:** On by default (DIV-3): cleans flag-as-bit comparisons on architectures that model condition flags explicitly (8051, PIC24, etc.); flip off to restore the upstream raw flag-arithmetic rendering.
- **Where / provenance:** P3/simplification-quiescence · ghidra-upstream · correctness-fix · GH-1276/8777
- **Example:** `option flagcompare on`

### `stackprobeloop` -- on | off, default `on`

- **Symptoms:** &pxVar[-0x1000] page-probe noise in a large-frame function; calls rendered argument-less because the stack pointer never resolved to a constant offset; gcc stack-clash probe loop leaves the frame layout unrecovered.
- **What it does:** Resolve a gcc -fstack-check / stack-clash probe loop's stack-pointer MULTIEQUAL to a fixed offset from the input SP, so the frame and call arguments recover cleanly.
- **When to flip:** Set on when a large-frame function shows &pxVar[-0x1000] page-probe noise or argument-less calls; shape-gated, so it is inert on functions without a probe loop.
- **Where / provenance:** P2/stack-pointer-normalization · ghidra-upstream · correctness-fix · GH-8017/6858
- **Example:** `option stackprobeloop on`

### `dynamichashmax` -- on | off, default `on`

- **Symptoms:** decompilation aborts with 'Unable to find unique hash for varnode'; dense unrolled simd/neon loop (aarch64, go) fails to decompile at symbol mapping.
- **What it does:** Raise the DynamicHash same-address collision budget 8->16 so dense unrolled code can still resolve a unique dynamic symbol hash.
- **When to flip:** Fixes a decompilation abort 'Unable to find unique hash for varnode' (e.g. AArch64/Go NEON byte-search loops). On by default (DIV-3); flip off to restore the upstream collision budget (the abort reappears).
- **Where / provenance:** P6/alias-facets · ghidra-upstream · correctness-fix · GH-8467
- **Example:** `option dynamichashmax on`

### `arraystride` -- on | off, default `on`

- **Symptoms:** loop walks an array with a raw offset accumulator (iVar += 0x414) instead of an index; strided induction offset never re-expressed as counter*stride array indexing.
- **What it does:** Re-express a strength-reduced array walk: rewrite a loop offset accumulator (acc += sizeof) as counter*stride so the array index is recovered.
- **When to flip:** A strided loop renders a raw offset accumulator (e.g. iVar += 0x414) instead of an index. On by default (DIV-3); flip off to restore the upstream accumulator rendering.
- **Where / provenance:** P3/simplification-quiescence · ghidra-upstream · structure-recovery · GH-8724
- **Example:** `option arraystride on`

### `condexeplace` -- on | off, default `on`

- **Symptoms:** malformed do/while whose body holds an out-of-place constant assignment; spurious = 0 copy materialized inside a loop block.
- **What it does:** Stop ActionConditionalConst from materializing a propagated constant as a COPY inside a loop predecessor block (a spurious `= 0` in the do/while body).
- **When to flip:** A malformed do/while holds an out-of-place constant assignment in its body. On by default (DIV-3); flip off to restore the upstream COPY placement.
- **Where / provenance:** P3/simplification-quiescence · ghidra-upstream · correctness-fix · GH-9203
- **Example:** `option condexeplace on`

### `inputvarnodeadjust` -- on | off, default `on`

- **Symptoms:** function aborts with 'Cannot properly adjust input varnodes'; overlapping stack parameters (mc68k link/unlk) kill the whole decompilation.
- **What it does:** Absorb an input Varnode overlapping the high end of a justified parameter container instead of aborting the function.
- **When to flip:** A frame aborts with 'Cannot properly adjust input varnodes' (overlapping stack params, e.g. mc68k link/unlk). On by default (DIV-3); flip off to preserve the upstream abort.
- **Where / provenance:** P6/stack-frame-layout · ghidra-upstream · correctness-fix · GH-9218
- **Example:** `option inputvarnodeadjust on`

### `evalcurrentproto` -- on | off, default `on`

- **Symptoms:** an x86 Windows function renders as (void) though it takes arguments; a local carrying a // ecx or // edx storage comment is read before it is ever written; __fastcall/__thiscall arguments missing from the signature; calls pass fewer arguments than the callee reads.
- **What it does:** Evaluate a function's OWN unknown prototype with the model its compiler spec nominates in `<eval_current_prototype>` (a `<resolveprototype>` merged model), instead of with `<default_proto>`. The merged model then resolves per function against the trials that are actually active (`FuncProto::resolveModel` -> `ProtoModelMerged::selectModel`), so an x86 Windows binary recovers `__fastcall`/`__thiscall` ECX/EDX arguments while a function that touches neither still comes out `__stdcall`. Six vendored specs nominate a model (x86win, x86borland, x86gcc, CR16, HCS12, HCS12X); every other language is untouched either way.
- **When to flip:** On by default: the spec's nomination is the compiler's own statement about how its functions are called, and without it a register-passed argument surfaces as a local READ BEFORE IT IS WRITTEN - not merely less informative, but not something a C function can do. Byte-identical on the 675-datatest corpus. Set off to restore the `<default_proto>`-only evaluation (stack-only inputs on x86 Windows). An explicit `option protoeval <model>` outranks both.
- **Where / provenance:** P4/prototype-source · ghidra-upstream · correctness-fix · kuna-eval-current-prototype
- **Example:** `option evalcurrentproto off`

### `callsitestackargs` -- on | off, default `on`

- **Symptoms:** call rendered with exactly six arguments on x86-64; call rendered with an empty argument list on i386; a for loop whose exit variable is written through a pointer argument disappears; ternary guards feeding a logging call are dead-code eliminated; outgoing-argument stack slots render as locals commented stack - 0xNN.
- **What it does:** Recover stack-passed call arguments at call sites whose callee prototype is unlocked, by testing the caller-relative argument Varnode address (not the callee-relative trial address) against the caller's local stack range when scoring an active input trial.
- **When to flip:** On by default: this restores upstream Ghidra behavior (fspec.cc:5618) that a mis-ported argument had disabled, so it is a correctness fix rather than a judgment call and needs no DIV row. With it OFF every stack trial is scored no-use, so calls truncate at the register budget (x86-64 six arguments, i386 none) and any computation whose only consumer was a dropped argument is dead-code eliminated - which deletes real basic blocks, including whole loops. Set OFF only to reproduce that pre-fix output for a bisect or an ablation.
- **Where / provenance:** P4/active-input-trial-scoring · ghidra-upstream · correctness-fix · decbench-callsite-stack-args
- **Example:** `option callsitestackargs off`

### `calloverlap` -- off | in | full, default `off`

- **Symptoms:** call emitted with no arguments or fewer arguments than the disassembly passes; call result read from a local that is never assigned; enclosing function loses its own parameter list and return type (void f(void)); argument register written whole-width by pxor movdqa movdqu por pand orpd or movq before the call; second call binds a stale pre-call value where the first call should have defined it.
- **What it does:** Complete the two upstream call-overlap guards kuna shipped as comment-only stubs: Heritage::guardCallOverlappingInput (heritage.cc:1210) and Heritage::tryOutputOverlapGuard / guardOutputOverlap (heritage.cc:1249/1293). Heritage characterizes each heritaged range against every call site's prototype; when the range is strictly LARGER than the ABI storage inside it the characterization is ContainedBy, and the whole-range guards see nothing. That is routine on x86-64 because SLEIGH models PXOR, MOVDQA, MOVDQU, POR, PAND, MOVQ-to-xmm and ORPD as ONE 128-bit write to the whole XmmReg, so Heritage::refinement's `size > 4 && maxw < size` gate never partitions the range and the 8-byte SysV parameter and return entries inside it stay invisible. At `in` the input guard locates the biggest contained input entry, translates its address from the callee's to the caller's perspective, inserts a SUBPIECE before the CALL that truncates a fresh whole-range Varnode down to that entry, registers the truncated Varnode as an input trial and appends it to the CALL - the guard PROPOSES storage, the P4 trial machinery still judges it. At `full` the output guard additionally turns the biggest contained return entry into an INDIRECT creation at the call and PIECEs the surrounding bytes of the range back around it, so the range still has a definition at the call while the return entry alone carries the output trial, and the range's effect is downgraded to unaffected. The two levels exist because they are separable: `in` alone recovers the dropped argument but leaves the call's return value read from a stale pre-call definition.
- **When to flip:** Turn ON (`full`, which is upstream Ghidra's behavior) when a call is rendered with missing arguments and/or its result is read from a variable that is never assigned -- the classic rendering is `u(); sink = v1; return v1 + 1.0;` where `v1` has no definition, from the canonical gcc -O2 int-to-double idiom `movapd xmm1,xmm0; pxor xmm0,xmm0; cvtsi2sd xmm0,rdi; call u`. The whole enclosing function can also lose its own signature (`void mix(void)` for a `double mix(int,double)`), and a printf-family call can lose every floating-point vararg its format string asks for. PREFER `full` OVER `in`: measured over 268 decbench O2 binaries / 77,014 functions, `full` changes 283 functions (0.37%) with 94 that stop reading a never-assigned local against 6 that start, while input-only changes 209 with only 10 fixed against 12 newly introduced -- recovering the argument without recovering the return value leaves the call result stale. Use `in` only to bisect which half of the guard moved an output. DEFAULT OFF even though the flip is clean on both gate corpora (0/675 datatests, 0/411 stage assertions with the option forced on) and adds no decompile errors (4,024 error-marked functions before and after): the guards make the whole over-wide range live at the call, which EXPOSES two pre-existing kuna defects on the functions they touch. (1) 54 of the 283 changed functions (19%) render an XMM range that used to be a clean `float4 v9` as a 16-byte `char v9 [16]` with `v9[0] = ...; SUB164(v9,0)` lane traffic -- faithful, but noisier C. (2) At 6 call sites the recovered floating-point argument is emitted in slot 0, ahead of the format string (`log_out((float8)x,(float8)y,a0,"time: %5.2f/%5.2f")`), because `build_input_from_trials` does not interleave the integer and float ParamEntry groups back into source order for an unprototyped mixed call. Both are worth fixing before the default flips. Flip back off if a call gains an argument the disassembly does not support, or if a 16-byte lane rendering appears where a scalar read better.
- **Where / provenance:** P3/call-guard · ghidra-upstream · correctness-fix · GH-275
- **Example:** `option calloverlap full`

### `loadguardrange` -- on | off, default `on`

- **Symptoms:** stack array subscripted past its own declared bound under a guard that proves the larger index; stack scalar at the end of an array read but never assigned and never address-taken; array written through a computed index capped at 4 elements regardless of the real extent; decompile-project output recompiles into a stack overflow the binary does not have; servo/config fields validated and stored from uninitialized stack locals.
- **What it does:** Complete the upstream LoadGuard/StoreGuard range-refinement machinery kuna shipped as stubs: at the end of each heritage pass, run the ValueSet solver (ValueSetSolver::establishValueSets/solve, rangeutil.cc:2416/2524) over the pointer of every newly discovered indexed-stack LOAD/STORE guard and record the real minimum/maximum/step of the addresses the op can touch (LoadGuard::establishRange/finalizeRange, heritage.cc:741/788, driven by Heritage::analyzeNewLoadGuards, heritage.cc:834). The refined guards feed two consumers. (1) P6 variable mapping: MapState::gatherOpen's addGuard loops (varmap.cc:1241-1248) turn a range-locked guard into an open RangeHint with highind = ((max-min)+1)/step - 1 (varmap.cc:1033) -- the ONLY source of a real index bound for a stack array written through a computed index. Without it the only hint is the fallback min_items = 3, so RangeHint::attempt_join rejects any element past index 3 (diffsz > highind) and splits it off as a separate never-assigned scalar while the array prints with a wrong extent and out-of-bounds subscripts. (2) Merge's StackAffectingOps: a refined store guard's isGuarded test now covers only the addresses the STORE can actually hit, so unrelated stack locals stop being treated as clobbered across the indexed STORE. OFF reproduces the pre-port behavior exactly: every guard keeps the whole-space range with step = 0, addGuard bails at its step == 0 gate, and no bound is ever produced.
- **When to flip:** ON by default (upstream Ghidra's stock behavior; 0/675 datatest ablation). The OFF symptom is self-contradictory C: a stack array declared with a too-small extent (int2 v5 [4]) subscripted by an index the enclosing guard proves reaches past it (if (5 < v2) ... v5[v2] = ...), with the tail elements split off as separate stack scalars that are read (range-checked, stored into structs) but never assigned -- a fake buffer overflow plus fake uninitialized reads, which also break the decompile-project recompile path. Flip OFF only to reproduce kuna's pre-GH-182 output or to bisect whether a stack-frame layout change came from guard refinement.
- **Where / provenance:** P3/load-guard-range · ghidra-upstream · correctness-fix · GH-182
- **Example:** `option loadguardrange off`

### `namestyle` -- angr | ghidra, default `angr`

- **Symptoms:** iVar1/uVar2/param_1 ghidra-style names wanted instead of v1/a1 (set ghidra); v-numbered locals and sub_/dat_/label_ names in the default output; byte-for-byte comparison against upstream ghidra naming.
- **What it does:** Master default-name scheme: angr-style (locals vN, args aN, global data dat_<addr>, unnamed functions sub_<addr>, labels label_<addr>, plus a // rax / // stack - 0x10 source-location comment per local) vs upstream Ghidra (iVar/uVar/pcVar, param_N, <type>Ram<addr>, func_, code_).
- **When to flip:** Set to ghidra to reproduce upstream Ghidra naming byte-for-byte; angr (default) makes default output read like the angr decompiler.
- **Where / provenance:** P9/naming-policy · angr · presentation-default · angr-naming
- **Example:** `option namestyle ghidra`

### `realtypes` -- on | off, default `on`

- **Symptoms:** undefined8/xunknownN placeholder types wanted for upstream comparison (flip off); size-guessed unsigned long/int/char types shown for values the inference never typed.
- **What it does:** Render residual TYPE_UNKNOWN (xunknownN) values as real C types by size — 1->char, 2/4/8->unsigned short/int/long (conservative on sign), pointer-to-unknown->void * — instead of the xunknownN/undefined<N> placeholder.
- **When to flip:** On (default) emits real C types for un-inferred values; off restores the upstream xunknownN/undefined<N> rendering for byte-for-byte comparison.
- **Where / provenance:** P9/literal-format · kuna · presentation-default · kuna-realtypes
- **Example:** `option realtypes off`

### `ctypes` -- on | off, default `off`

- **Symptoms:** int4/uint1/uint4/float8/float10 appear in the emitted C instead of C type names; the same function mixes `unsigned int` with `int4`; `code *` appears as a function-pointer type; the emitted C does not compile because its type names are not C.
- **What it does:** Spell the NAMED core types as the target's own C type names -- int4 -> int, uint1 -> unsigned char, float8 -> double, float10 -> long double, code * -> void * -- resolved against the compiler spec's decoded <data_organization>, so an 8-byte integer reads `long` on LP64 and `long long` on ILP32/LLP64. Extends `realtypes`, which relabels only residual TYPE_UNKNOWN; that split is why one function can declare `unsigned int v3;` and `int4 v1;` in the same block. Sizes no C type has (3/5/6/7, 16-byte integers) keep their undefined<N> form rather than being widened, because widening a truncating cast changes its meaning.
- **When to flip:** The emitted C uses Ghidra's internal type vocabulary (int4/uint1/float8/code) instead of C type names, or mixes the two within one function. ON in the `aggressive` preset, which `auto` selects for any binary under 500 KiB -- so this is the default rendering of `kuna decompile`, `decompile-all`, `decompile-project` and the web front-end. The shipped catalog default is OFF, which is what the XML datatest corpus runs at: 42 of its assertions pin the Ghidra spellings.
- **Where / provenance:** P9/literal-format · kuna · presentation-default · kuna-ctypes
- **Example:** `option ctypes on`

### `dedupvardecls` -- on | off, default `on`

- **Symptoms:** the same local declared once although many HighVariables share the stack slot; flip off to see one declaration line per high (e.g. int4 option_index repeated hundreds of times).
- **What it does:** Collapse local-variable declarations onto one line per mapped ScopeLocal symbol (same containing symbol + same identifier, declaring the symbol's own type when the highs disagree about it) and then onto one line per fully-rendered signature (type + name + array adornment + storage comment), so a stack slot mapped onto many scalar HighVariables is declared once instead of one line per high (the scalar analogue of the existing composite-symbol declaration collapse).
- **When to flip:** On by default (DIV-7): a stack slot is declared once even when many same-named scalar HighVariables share it (e.g. x86_64/cvs main, where the per-high rendering repeats `int4 option_index; // stack - 0x3c` ~200x), including when two live ranges of the slot recovered different types and so rendered two differently-typed declarations of one name -- invalid C (DIV-52). Set OFF to restore the one-declaration-per-high rendering.
- **Where / provenance:** P9/naming-policy · angr · presentation-default · angr-duplicate-decls
- **Example:** `option dedupvardecls off`

## Programmatic use

```bash
# discover (machine-readable; includes tier + symptoms):
kuna catalog --json

# transforms only:
kuna catalog --tier transform

# decompile with an option flipped (repeatable):
kuna decompile ./a.out main --option gotoreduce off
kuna decompile ./sparc.elf main --option returnpair single
```
