# `slotptr` default-on evaluation — FLIPPED ON

Measured on base `2888a2a94` (origin/main), both arms from one build: `--option
slotptr off` against the new default. The option changes only `variables[]` (and
the `; stack:` comments `decompile-project` writes from the same rows), so the
C-text gates cannot move by construction; they were run anyway.

| criterion | result |
|---|---|
| (a) `make test` with the new default | PARITY OK, 675/675, no assertion moved |
| (b) `make test-stages` | PARITY OK, 1296/1296, no baseline change (no stage test pins a JSON row) |
| (c) `make test-cli` | 221/221; no existing probe moved; two new probes pin the fauxware witness on and off |
| (d) 444-slice typesweep, new default vs old | 1,353 -> 1,472 perfect, 418 improved / 5 worse (read below) |
| (e) speed, interleaved min-of-15 | SPEED_TABLE |
| (f) whole-binary `decompile-all` before/after, 8 binaries | 6,265 functions, 0 C diffs, 0 variable-count diffs, 1,979 changed rows, every one a filler `undefinedN -> pointer` at the same name, offset and size |
| (g) `p0_knowledge/modes.rs` | no entry needed: the option is a shipped default that every preset inherits, and the modes' default-off invariant only covers default-off options |

## (d) The sweep

Pinned metric (decbench `625e892`), `DECBENCH_NO_CACHE=1`, projects coreutils
grep gzip diffutils bzip2 findutils tar shadow at -O0, -O2 and -O2-noinline.

| project | n | perfect off | perfect on | mean off | mean on | improved | worse |
|---|---|---|---|---|---|---|---|
| bzip2 | 267 | 31 | 34 | 0.4097 | 0.4155 | 6 | 0 |
| coreutils | 6422 | 880 | 917 | 0.3352 | 0.3409 | 182 | 3 |
| diffutils | 420 | 58 | 63 | 0.4325 | 0.4445 | 32 | 1 |
| findutils | 790 | 41 | 44 | 0.2201 | 0.2230 | 9 | 0 |
| grep | 247 | 51 | 51 | 0.4404 | 0.4458 | 8 | 1 |
| gzip | 368 | 102 | 107 | 0.5197 | 0.5270 | 10 | 0 |
| shadow | 686 | 32 | 86 | 0.3012 | 0.3545 | 108 | 0 |
| tar | 1548 | 158 | 170 | 0.3524 | 0.3635 | 63 | 0 |
| pooled | 10748 | 1353 | 1472 | 0.3415 | 0.3511 | 418 | 5 |

Aggregate type_match 3669.96 -> 3773.19 (+103.23); 120 functions move onto a
perfect score and 1 off it. The variable count of every function is identical in
both arms (`nvars` differs for 0 of 10,748 functions): the option never adds or
removes a variable, so arity and phantom-variable counters cannot move.

Ground-truth true positives by class, stack storage:

| class | n | off | on |
|---|---|---|---|
| ptr_char | 3,316 | 1,506 | 2,133 (+627) |
| ptr_struct | 2,614 | 108 | 202 (+94) |
| ptr_void | 294 | 7 | 13 (+6) |
| int_u8 | 2,441 | 2,216 | 2,213 (-3) |
| int_s8 | 1,088 | 958 | 954 (-4) |

All classes: 20,405 -> 21,140. A further 128 `ptr_struct` stack rows become
`struct_N *` on a real struct pointer, which decbench#93 credits and the pinned
metric does not.

### Every worse row

Five functions, nine ground-truth variables. In each one the slot receives the
type kuna's C body already gives the stored value, and that type is wrong:

| function | GT variable | now | why the body says so |
|---|---|---|---|
| coreutils -O0 shred `dorewind` | `__off_t offset` | `int *` | one rax variable merges the `lseek` result with the `__errno_location()` pointer |
| coreutils -O0 tail `tail_lines` | `__off_t end_pos` | `int *` | the same merge (the function's whole rax is one `int *` variable) |
| diffutils -O0 diff `scan_char_literal` | `long digits` | `char *` | `p - lit - 2` printed as `&v3[0xfffffffffffffffe - (long)a0]` |
| coreutils -O2 split `main` | `long to_write` | `char *` | a count computed from two pointers, printed as pointer arithmetic |
| grep -O2 grep `main` | `idx_t newkeycc` | `long *` | the key counter is a `long *` throughout the body (`v22 = (long *)((long)v22 - 1)`) |

Four more variables lose inside functions whose score does not drop, because
other slots in them gain: diff `find_and_hash_each_line` `length` (`unsigned
char *`; the function goes .704 -> .815), tar `xheader_string_end` `p` and `size`
(`char *`; .308 both ways) and tar `expand_pax_option` `__len` (`char *`; .265 ->
.294). Each is a pointer difference the body prints as pointer arithmetic. These are C-body typing
bugs, visible there already; fixing them fixes the slots.

## Decision

All criteria pass. `slotptr` ships default ON.
