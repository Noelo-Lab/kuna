# tail.o::tail_bytes — angr vs kuna (option `noreturn_extern`)

angr testcase: `test_tail_tail_bytes_ret_dup` (angr 9.2.213), function `tail_bytes`
binary: `/home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/tail.o`
(ELF **relocatable object**, ET_REL, `--arch x86_64`)

## The symptom

| metric | angr (reference) | kuna (default) |
|---|---|---|
| loc | 96 | **326** |
| gotos | 1 | **9** |
| labels | 1 | **9** |

kuna's `tail_bytes` is 3.4× longer and structurally garbage. The real `tail_bytes`
is a 615-byte function spanning `0x401e80..0x4020e7` (the ELF symbol carries that
size: `45: 0x1e80  615 FUNC LOCAL tail_bytes`). kuna's rendering keeps going *well
past* `0x4020e7`, swallowing several **adjacent functions** (`tail_lines` with its
`pipe_lines`/`start_lines`/`file_lines`, then a `fstatfs`/`is_local_fs_type`/
`__assert_fail` function, an `lstat` loop, a `raise`/`exit`/`poll`/`die_pipe`
function) all glued into one — even materialising a bogus `do { … } while
(dat_4045a1 == '\0')` outer loop from the spliced-together control flow.

## What angr does better — the exact construct

angr stops `tail_bytes` at its real end. The function's canary epilogue is:

```
v8 = canary ^ *(fs:0x28);
if (v8 == 0) return v4;
__stack_chk_fail();      // 0x4020e7 — NORETURN; nothing executes after it
```

`__stack_chk_fail` **never returns**, so the bytes after the call (which belong to
the *next* function) are not part of `tail_bytes`. angr knows this and bounds the
function correctly. kuna does not: it treats `__stack_chk_fail` as an ordinary
returning call, follows the fall-through edge past `0x4020e7`, and decodes straight
into the neighbouring function.

## Root cause (verified)

In this `.o`, `__stack_chk_fail` is an **undefined external** symbol:

```
197: 0x0  0 NOTYPE GLOBAL DEFAULT UND __stack_chk_fail
209: 0x0  0 NOTYPE GLOBAL DEFAULT UND __assert_fail
```

kuna's analysis-tier known-no-return pass (`noreturn_known`, default **on**;
`kuna-analysis/src/s1_loader/noreturn.rs`) scans **defined** `FUNC` symbols
(`.symtab`/`.dynsym` definitions) and marks their *addresses* no-return. An
**UND** extern has no definition and no stable address, so it is never marked —
even though its base name `stack_chk_fail` is squarely on the no-return list
(`exit`, `abort`, `stack_chk_fail`, `assert_fail`, `pthread_exit`, …).

At decompile flow time, `FlowInfo` asks `FlowEnvironment::query_call_no_return(entry)`
(→ `Architecture::symboltab.function_is_no_return_across_scopes`). The call to
`__stack_chk_fail` *does* resolve to a display name (the reloc gives the call its
name), but the symbol's no-return flag is false, so the seam returns false, the
artificial halt is **not** planted, and flow runs off the function's end.

### Proof the diagnosis is correct

Driving `decomp_dbg` with the manual override `option noreturn __stack_chk_fail`
(after `read symbols`) chops the function **328 → 87 lines**, correctly bounded and
structurally matching angr (residual 3 gotos vs angr's 1 — a separate
goto/return-duplication concern, **out of scope** for this feature, which targets
only the boundary blow-up).

## Owning stage / hook

S2 (lift / flow classification). The decision point is the **flow-time seam**
`FlowEnvironment::query_call_no_return`, implemented by `ArchFlowEnv` in
`infra/decompile_drive.rs` — the same seam that already reads `arch.tail_call_jumps`
for `is_tail_call_branch`. This is the kuna analog of the gap angr closes with its
no-return analysis over the call graph; here the missing piece is specifically the
**undefined-extern** case the address-keyed analysis pass cannot reach.

## Hypothesis / fix

Add an option-gated, name-based no-return fallback at `query_call_no_return`: when
the option is on **and** the address-keyed check already returned false, resolve the
callee's display name and return true if its base name (leading `_` stripped,
`std`-only namespace guard) matches the known ELF no-return list. This plants the
existing `artificialHalt(noreturn)`, so flow stops at the call and the function is
bounded correctly — closing the dominant 326→~87 loc / 9→3 goto gap. Default-OFF
opt-in unless the 675-datatest ablation is clean and speed is within budget.
