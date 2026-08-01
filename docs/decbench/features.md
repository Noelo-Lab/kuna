# decbench campaign — the feature menu (round 1, 2026-08-01)

Output of `mine` → `triage` → cluster on the 2026-07-27 benchmark run. 16 cases triaged, one
agent each, verify-first on today's build; every feature-candidate verdict then handed to a
second agent briefed to **refute** it. 11 survived, 1 was refuted on its diagnosis (and turned
out to be worse than filed), 4 were not defects.

Two columns matter and they are not the same column:

- **GED value** — does fixing it move the benchmark? GED is a CFG edit distance, so it is blind
  to types, argument lists, declarations, operator spelling and anything else that does not
  change basic-block count.
- **Correctness value** — does the emitted C stop being wrong? Several items below score **zero**
  on the benchmark and are still the most important things in this table, because kuna already
  wins those panes and emits code that does not compile or does not compute the right value.

Ranking is by correctness first. A campaign that optimizes only the metric it can see will
happily ship a decompiler that scores well and lies.

## Tier 0 — kuna emits WRONG CODE (fix regardless of benchmark value)

| slug | phase | cases | what is wrong | GED | scope | status |
|---|---|---|---|---|---|---|
| **p6-lost-restore** | P6 | `O0-gnutls-ocsptool-port_to_service`, `O0-coreutils-ptx-output_one_dumb_line` | A value live across a merge point is lost or aliased. In `port_to_service` kuna hoists `v2 = a0` to the entry, clobbers it with the `getservbyport` result and never restores it, though the binary reloads `sport` at `0x114e8` — **kuna's C returns NULL where the binary returns a value**. Proven executable: the real binary prints `99999`, kuna's transcribed C segfaults. Ghidra, IDA, angr, binja and phoenix are all correct here. In `output_one_dumb_line` two distinct selects merge into one HighVariable, so the C subtracts 0 where the binary subtracts `truncation_string_length`. | 0 | investigating | agent running |
| **stackargs** / **callsite-stack-args** | P4 | `O0-mydoom-mydoom-msg_b64enc`, `O2-noinline-openssh-portable-ssh-add-parse_dest_constraint` | kuna never registers a **spacebase (stack) trial** at a call site, so stack-passed arguments do not exist: calls print with arguments missing, and the code that computes them is dead-code eliminated. **707 of 738 i386 call sites in `mydoom.exe` render with an empty argument list**; reproduced architecture-independently on x86-64 past the 6th argument. Only bites call sites with an unlocked callee prototype (DWARF/`libproto` hide it). The callee side is correct, so the cspec and trial map are fine — it is the caller-relative→callee-frame translation that is missing. | small | proposal | queued |

## Tier 1 — emitted C is not valid C (strict bug fixes, no option)

| slug | phase | case | what is wrong | GED | status |
|---|---|---|---|---|---|
| **spacebase-unnamed-location** | P9 | `O2-noinline-mydoom-mydoom-scan_textfile` | With no Symbol bound to the offset, `op_ptrsub_ir`'s SPACEBASE arm falls back to the functional render and prints `PTRSUB(ESP,8)` — the raw p-code operator plus the raw register, neither declared. Ghidra prints a stack-location leaf. Reproduces on ARM too (`PTRSUB(sp,0)`). | 0 | **PR in flight** |
| **realtypes-pointee-size** | P9 | `O2-noinline-libacl-libacl.so.1.1-set_acl_fd` | `realtype_unknown_base` returns `void` unconditionally under a pointer, collapsing `undefined1/4/8 *` to `void *` — but only in the DECLARATION. Index and cast expressions keep the original pointee size, so declaration and expression disagree about the stride, and `gcc -c` gives *invalid use of void expression*. 150 width-carrying casts collapse to `*(void *)` in one library. | 0 | queued |
| **symbol-keyed-local-decls** | P9 | `O2-noinline-betaflight-…-applyLedFixedLayers` | One declaration per HighVariable instead of one per ScopeLocal Symbol, so a single stack slot is declared **twice under the same name** with two different types. | 0 | queued |
| **jumptable-callother-inject** | P2 | `O2-noinline-betaflight-…-accDetect`, `O0-libopencm3-sdram-main` | `<callotherfixup>` injects are drained once, before jump-table recovery; the CALLOTHERs queued by the post-recovery `fallthru()` re-drain are never injected, so `setISAMode(1);` survives into the C as a call to a function that does not exist. kuna erases it correctly for 3703/3798 functions in the same binary — internally inconsistent. | 0 | **PR in flight** |
| **finalorder-entry-first** | P8 | `O0-libopencm3-sdram-main` | `BlockGraph::orderBlocks()` is an explicit STUB (`blockaction.rs:3792`), so `ActionFinalStructure` never orders the components. The function body is emitted starting at a mid-function label with the real entry after an unconditional `goto`, as unreachable code. The ordering key is already ported; only the sort is missing. | 0 | queued |

## Tier 2 — structure recovery (moves the metric)

| slug | phase | case | mechanism | GED win | scope |
|---|---|---|---|---|---|
| **iteboolean** | P3/P8 | `O0-bash-bash-time_command` | `RuleConditionalMove` requires every MULTIEQUAL input block to be the CBRANCH root or a single-predecessor pass-through; an `&&`/`||` chain gives the constant arm 2+ predecessors, so a short-circuit boolean assignment stays an explicit 0/1 diamond. Relaxing that bail is the whole fix. High breadth — `-O0` code materializes booleans everywhere. | 26 → 6 on this function | small |
| **iteregion-merged-dest** | P8 | `O0-coreutils-ptx-output_one_dumb_line` | `same_storage()` demands the two arms write the same RAW storage; P6 relocated one arm into a copy-shadow `unique` and unified it through the join MULTIEQUAL, so the arms write different storage but the **same HighVariable**. Key on the high, not the storage. | 66 → 0 | small |
| **paramcopyhoist** | P6 | `O0-e2fsprogs-e2fsck-save_output` | `trim_op_input_prep` always anchors the trim COPY at the tail of the phi's incoming predecessor block. **The record's own proposed fix is a no-op** (the skeptic showed `pc` only reaches `new_op`) — re-derive the mechanism before building. | — | small |
| **switchstagedindex** | P2 | `O0-cronie-crontab-load_env` | Jump-table recovery runs once over a partial flow in which the case bodies are not yet decoded, so the index slot has one reaching definition and the 7-case `switch (state)` collapses to a 0/1-entry table — **deleting the state-machine loop and 3 of 4 case bodies**. The skeptic REFUTED the record's proposed mechanism (upstream Ghidra recovers it without multistage), so re-derive before proposing. | large | proposal |

## Not defects

| case | verdict |
|---|---|
| `O0-coreutils-factor-factor` | **already-fixed** — today's default (`auto` → `aggressive`) emits the source's early-return shape; `returndup`, which `aggressive` turns on, closes the 12-point gap. |
| `O2-noinline-iproute2-ip-netns_add` | **covered-by-option** — `readonly` stops the 16 phantom byte-stores into a `.rodata` string. Default-flip candidate. |
| `O0-bash-bash-rl_vi_redo` | **covered-by-option** — `branchflip off` makes kuna's CFG isomorphic to the source (GED 40 → 0). `branchflip` is default-ON, so this is a default question worth measuring, not a new feature. |
| `O2-openssh-portable-sshd-mm_answer_auth2_read_banner` | **metric-artifact** — the GED floor for any output that recovers the CMOV null-check is 8; angr's 0 comes from dropping the check entirely. |

## What round 1 says about the campaign

Nine of the eleven survivors have **zero GED value**. Eight of them came from the pools that do
not rank by margin at all — the NOVEL pool (kuna is already best and the output is still bad)
and the recall pool. The angr/ida margin pools, which the campaign has mined for months,
produced mostly small structuring gaps plus one metric artifact.

The reading: kuna's *structural* recovery is now competitive (2nd of 11 on GED, and it wins
several of the panes where a case was mined against it), and the remaining defects are in what
the metric cannot see — argument lists, declarations, types, and values. Rank by correctness,
use GED as a filter, and keep mining the NOVEL and recall pools.
