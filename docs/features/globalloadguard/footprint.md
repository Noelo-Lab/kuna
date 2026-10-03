# Footprint of the global LOAD guard (issue #825)

`decompile-all` over 18 stripped binaries from the decbench `full_run_address_2026-09-11`
corpus, main 511bd082d against this branch, both at the default `indexaliasguard global`:
O2 gzip, dash, bzip2, sort, ls, grep, diff, find, mirai, kmod, crond, libedit, crazyflie
`cf2.elf`, nuttx, chibios `ch.elf`, x0r-usb (i386 PE), and O0 cp and dash.

25 of 8,410 functions change; the `goto` count goes from 6,730 to 6,732, both in
`cf2.elf` `sub_803228c`. Lines go from 266,663 to 266,821.

## Classes

| Class | Functions | What changes |
|---|---|---|
| Store kept | 18 | A store to a global the binary makes, which main dropped as dead because the global is stored again after a load through a pointer, now prints where the binary makes it. |
| Store moved to the binary's position | 1 | dash `sub_f8c0`: `dat_21a60 = 0xffffffff00000001;` prints before the loop that loads through `v5`, where the binary stores it (`f9b8`, `faa1`); main printed it after the loop. |
| Load printed ahead of a store | 4 | A pointer load whose value is live across a store to a global keeps its own variable and prints where the binary loads it. |
| Structure only | 1 | `cf2.elf` `sub_803228c`: no statement changes; the sign test of `a4 << 0x1f` at `0x80322ac` comes out of the rules in the other orientation, the branches swap, and the third angle-normalising loop prints as `if (...) goto label_8032322; do { ... } while (...);` where main printed an empty `for`. Same semantics, two more `goto`s. |
| Load printed ahead of a store, plus a cast | 1 | coreutils `sort` `main`: the `localeconv()` field loads print ahead of the `dat_1d868`/`dat_1d864` stores, and `dat_1d864 = (int)v15;` replaces `(unsigned int)`, the same bits for a `char`. |

## Functions

| Binary | Function | Class | Checked against the disassembly |
|---|---|---|---|
| O2 crond | `main` | store kept: `dat_502e0 = v8;` before `dat_502e0 = &v8[1];` | `3de4` stores the `strrchr` result, `473b` loads `argv[0]` on the NULL path, `3e04` stores again |
| O2 dash | `sub_f8c0` | store moved | `f9b8`, `faa1` |
| O2 dash | `sub_105e0` | load ahead of store: `v12 = *(unsigned long *)&v34[4];` before `*v34 = 0xe; dat_21a90 = 7;` | `1261d` loads, `12621`/`12629` store |
| O2 gzip | `sub_8460` | load ahead of store: `v7 = *(dat_1a008 + 0x9c000); dat_1a008 += 1;` (gzip's `get_byte`) | `8555` loads, `855a` stores |
| O2 sort | `main` | load ahead of store, cast | |
| O2 cf2.elf | `sub_801699c` | store kept: `dat_20000a54 = 0; dat_20000a58 = 0xffffffff;` on each path, where main printed them once at the join after the loops' loads | `8016a08` `strd` on the path, join at `8016af2` stores other words |
| O2 cf2.elf | `sub_802b5f0`, `sub_802cca0`, `sub_803d828` | store kept | |
| O2 cf2.elf | `sub_803228c` | structure only (above) | `8032304`..`8032322` |
| O2 cf2.elf | `sub_804ad24` | store kept (stores to low addresses, data decoded as code) | |
| O2 cf2.elf | `sub_803fbc0`, `sub_8040158`, `sub_8040c2c`, `sub_80423ec`, `sub_8042acc`, `sub_8042e48`, `sub_8043804`, `sub_80440cc`, `sub_8044d68`, `sub_80450b4`, `sub_8045658`, `sub_8045a10` | store kept, in the run of data decoded as code at `0x803fbc0..0x8045a10` | |
| O2 cf2.elf | `sub_80404b0`, `sub_8044bec` | load ahead of store, in the same run | |

The guard only marks writes the function already has as live; it never adds a write.
A kept store prints at its own op, which is the binary's store; where Merge used to
print a loaded value at the load by joining it with the global, the load now keeps its
own variable instead (`x = *p; gj = 0; gi = x;` prints `v1 = *a0; gj = 0; gi = v1;`).

## Round trips

Eleven sets of small functions that store to globals around pointer stores and loads,
built with gcc and clang at -O0 to -O3, each function compiled from its printed C
with gcc and clang at -O0 and -O2 and run with its pointers aimed at the globals: 48
(set, build, function) cases that main prints wrong print right here, and none goes the other way.
