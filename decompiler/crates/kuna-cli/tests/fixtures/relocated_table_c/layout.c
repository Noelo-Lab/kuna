unsigned char sample_prefix[256] = { 0 };
unsigned char sample_table[23] __attribute__((aligned(4096))) = {
    -128,127,-1,0,1,93,-93,42,-42,7,-7,63,-64,3,-3,99,-99,11,-11,17,-17,29,-29
};
short sample_halves[11] __attribute__((aligned(4096))) = {
    -32768,32767,-1,0,1,12345,-12345,999,-999,53,-211
};
