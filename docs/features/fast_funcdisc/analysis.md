# Fast project function discovery

## Report

`decompile-project --mode fast` completed on private PE
`bc4c15d826aaebeace3fec6360eb687e5662cba8745605093254931dcdb3ae1b`,
but its C file contained only import veneers and the image entry. In particular,
the project omitted `0x402d80` even though an explicit
`kuna decompile <binary> --addr 0x402d80` produced a 26-line function body.

The old fast inventory had 351 executable definitions: 350 import veneers and
the entry point. The binary has no usable PDB, exports, unwind function table, or
ordinary function-symbol inventory. Its internal code therefore depended on a
program-wide discovery path, but fast mode disabled all three existing paths:
`listing`, `funcstart_patterns`, and `aif`.

The explicit address command was not contradictory. It bypassed discovery by
constructing a function at the requested address. Project export can only
iterate functions that discovery has registered.

## Evidence

The private PE is 3,457,296 bytes. Its `.text` is roughly 2.8 MiB, its entry is
`0x6b7e21`, and the bytes at `0x402d80` form a normal x86 subroutine ending in
`ret 4`. The address also occurs in two distinct six-slot initialized-data
pointer tables. With the final implementation:

- `functions --mode fast --option fast_funcdisc off` has 693 canonical callable
  entries and omits `0x402d80`;
- `functions --mode fast --option fast_funcdisc on` has 3,495 canonical entries
  and includes `0x402d80`;
- the corrected project has 3,153 executable definitions, of which 3,140
  decompile successfully and 13 fail in isolation under a ten-second
  per-function watchdog;
- the full corrected export completes in 462.28 seconds at 1,475 MiB peak RSS;
- the reliable control remains incomplete after 935.91 seconds.

The public `pdb_prog.exe` fixture reproduces the direct-call half without private
bytes: its loader-known entry at `0x140001010` calls the otherwise undiscovered
`0x140001000`. The public `aif_gap_x86_64` fixture reproduces the
pointer-only half at `0x13ae`.

## Owning decision

This is P1 code/data partitioning. Fast mode needs a bounded function inventory,
not a different P2-P9 decompilation pipeline. The new analysis option is
`fast_funcdisc`; it is default-off as an output-changing discovery policy and is
enabled by the `fast` preset.
