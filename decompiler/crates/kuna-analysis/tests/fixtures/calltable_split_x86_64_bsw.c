/* (kuna) Fixture calltable_split_x86_64, file 2 of 3: a range-checked switch
 * reached only through file 1's `ops`; `bsw_end` closes its table. */
extern volatile int sink;
const long bsw_end = 0x1234;
int bsw(unsigned x) {
    switch (x) {
    case 0: sink = 10; return 1;
    case 1: sink = 13; return 8;
    case 2: sink = 16; return 15;
    case 3: sink = 19; return 22;
    case 4: sink = 22; return 29;
    case 5: sink = 25; return 36;
    case 6: sink = 28; return 43;
    case 7: sink = 31; return 50;
    default: return -1;
    }
}
