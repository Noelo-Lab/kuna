# angr `test_decompiling_missing_function_call` :: adams/main — analysis

## The gap

angr recovers `main`'s getopt dispatch as a proper `switch` (6 cases:
`strdup`/`fprintf`/`atoi`/… inside a `while` getopt loop). kuna fails to recover the
jump table and emits:

```
                    /* WARNING: Treating indirect jump as call */
  v2 = (*(code *)((int8)*(int4 *)((uint8)(uint4)(m - 0x3fU) * 4 + 0x96ac) + 0x96ac))();
  return v2;
```

The entire `switch` body (every case, every `strdup`/`fprintf`) **and** the surrounding
`while (getopt(...) != -1)` loop collapse into a single bogus indirect call. The angr test
asserts exactly this is wrong: `code.count("fileno") == 1`, every `break` is followed by
`case`/`default`/`}`, and `emaillist = strdup(...)` is present.

## The construct

`main`'s switch is a textbook GCC PIC jump table (x86-64, `adams` is a PIE; binary vaddrs,
main @ 0x8e40, table @ 0x96ac in `.rodata`):

```
8eef: add $0xffffffc1,%eax    ; eax = m + (-0x3f) = idx        (m = getopt() result)
8ef2: mov %eax,%ecx           ; ecx = idx
8ef4: sub $0x36,%eax          ; eax = idx - 0x36
8ef7: mov %rcx,-0x48(%rbp)    ; SPILL idx to stack slot -0x48
8efe: ja  9106                ; if (idx >u 0x36) goto default     <- the range guard
8f04: lea 0x7a1(%rip),%rax    ; rax = &table (0x96ac)
8f0b: mov -0x48(%rbp),%rcx    ; RELOAD idx from stack slot -0x48
8f0f: movslq (%rax,%rcx,4),%rdx ; rdx = sext(table[idx])  (self-relative 4-byte offset)
8f13: add %rax,%rdx           ; rdx = table_base + table[idx]
8f16: jmp *%rdx
```

The bound is a single clean range guard `idx <=u 0x36` ⇒ **table size 0x37 (55 entries)**;
most entries point at the default, leaving 6 distinct case targets.

## Owning stage / why kuna fails (instrumented)

Owning stage: **S2 / switch-model** (jump-table recovery), in
`JumpBasicModel::recover_model_basic` (`s2_lift/jumptable.rs`), invoked from
`JumpTable::recover_addresses` during `FlowInfo::recover_jump_tables`.

Instrumentation (`KUNA_JT_DEBUG=1`) at the failure point shows:

```
basic model2=false jrange.size=2147483648 maxtablesize=1024 num_common_vn=8 vidx=2
  meldvn[2] u+0x23d00 sz=4 def=CPUI_LOAD          (table load result — the chosen vn, unbounded)
  meldvn[6] %+0x0    sz=4 def=CPUI_INT_ADD         (eax = idx = switchvar-0x3f — the real index)
  guard[2]  vn=%+0x0 sz=4 rangesize=4294967295 branch=true   (the range guard on idx — FULL range)
```

So:

1. `find_normalized` → `analyze_guards` DID collect the range guard (`guard[2]`, on `eax`),
   but its **range came back full** (2³²−1). The guard is realized as `sub 0x36; ja` — a
   *flag-form* unsigned comparison. Jump-table recovery runs **early, on a partial/truncated
   function**, before the flag idiom is simplified to `INT_LESS(0x36, idx)`, so
   `circlerange_pull_back` cannot extract the `<= 0x36` bound.
2. With no bounded guard, `calc_range`/`find_smallest_normal` leave the index range at the
   full 2³¹, which exceeds `maxtablesize` (1024).
3. `recover_model_basic` therefore declines the basic model; model-2 also declines; so
   `recover_model` yields no model and `recover_addresses` throws
   *"Could not recover jumptable … Too many branches"*.
4. `FlowInfo::recover_jump_tables` → `truncate_indirect_jump(FailNormal)` rewrites the
   `BRANCHIND` to a `CALLIND` (`/* WARNING: Treating indirect jump as call */`), which is why
   the whole switch + loop disappear.

The stack spill (`mov %rcx,-0x48(%rbp)` / `mov -0x48(%rbp),%rcx`) additionally means the
guarded value and the table-index value are distinct varnodes connected only through memory,
so even a non-flag guard would not `value_match` the normalized index — `analyze_guards`'
`usenzmask = !is_partial` conservatism on partial functions is by design.

This is the **same failure symptom** that kuna's existing `switchmodbound` option (GH-9191)
addresses for an *in-band modulo/and-mask* bound — here the bound is instead an
*out-of-band CBRANCH range guard* in unsimplified flag form.

## Hypothesis / fix

A new **default-OFF, option-gated** `JumpBasic` model-extension, exactly parallel to
`kuna_try_modulo_bound_table`: when the basic model fails to bound the table
(`jrange.size > maxtablesize`) and `option switchguardbound on`, look at the collected
`selectguards`; for a guard whose storage matches a meld index varnode, extract the
unsigned comparison constant `K` from the (possibly flag-form) guard CBRANCH and re-bound the
table index to `[0, K+1)`, then let the normal `buildAddresses` emulation enumerate the
targets. Gate off ⇒ byte-identical to today (the method is only reached in the
already-failed branch). See `plan.md`.
