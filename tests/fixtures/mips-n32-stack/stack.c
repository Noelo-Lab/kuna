/* Original defined-behavior probes; no target libc or vendor source. */
typedef unsigned long long U;
#define EIGHT U a, U b, U c, U d, U e, U f, U g, U h

U sum_ten(EIGHT, U i, U j) { return a+b+c+d+e+f+g+h+i+j; }
U frame_ten(EIGHT, U i, U j) {
    volatile U values[2] = {a, i};
    return values[0] + values[1] + j;
}
U pointer_ninth(EIGHT, const U *p) { return *p; }
int signed_ninth(EIGHT, int value) { return value; }
unsigned float_ninth(EIGHT, float value) {
    union { float f; unsigned u; } bits = {value};
    return bits.u;
}
U address_and_bits(U raw) {
    return *(const U *)(__UINTPTR_TYPE__)raw + (raw >> 32);
}
U write_value(U *p, U value) { *p = value; return value; }
U indirect_frame(U (*callee)(U), U value) {
    volatile U saved = value;
    return callee(saved) + saved;
}
U upper_half(U value) { return value >> 32; }
