// Tiny C fixture for DWARF stack-LOCAL recovery (s1_dwarf subtask 3).
//
// compute_sum has two DW_OP_fbreg stack locals — `accumulator` and `counter`.
// `&accumulator` is taken (passed to scanf), so the decompiler CANNOT promote it
// to a register: it stays an addrtied stack slot and the DWARF name+type bind to
// it, rendering `int accumulator` instead of `local_10`. (cet_pie's locals are
// write-once spill slots the engine eliminates, so this is the fixture that
// proves the install actually renders.)
//
// Built: gcc -O0 -g -no-pie -fno-stack-protector stacklocal_x86_64.c
//        -o stacklocal_x86_64   (x86-64, DWARF; call_frame_cfa == 8).
//   compute_sum @ 0x401136; accumulator fbreg -24 -> stack -0x10,
//   counter fbreg -20 -> stack -0xc.
#include <stdio.h>

int compute_sum(int n)
{
    int accumulator = 0;            // DW_OP_fbreg stack local (address-taken)
    int counter;                    // DW_OP_fbreg stack local
    for (counter = 0; counter < n; counter++) {
        scanf("%d", &accumulator);  // &accumulator forces it onto the stack
        accumulator += counter;
    }
    return accumulator;
}

int main(int argc, char **argv) { return compute_sum(argc); }
