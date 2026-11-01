/* Original probes: a narrow address does not constrain the full integer. */
typedef unsigned long long U;

U compare_bits(U raw, U expected) {
    return *(const U *)(__UINTPTR_TYPE__)raw + (raw == expected);
}
U return_bits(U raw, U *destination) {
    *destination = *(const U *)(__UINTPTR_TYPE__)raw;
    return raw;
}
U add_bits(U raw, U delta, U expected) {
    U address = raw + delta;
    return *(const U *)(__UINTPTR_TYPE__)address + (address == expected);
}
U sp_high(void) {
    U value;
    __asm__ volatile("move %0,$sp" : "=r"(value));
    return value >> 32;
}
U sp_full(void) {
    U value;
    __asm__ volatile("move %0,$sp" : "=r"(value));
    return value;
}
U sp_and_stack(U a, U b, U c, U d, U e, U f, U g, U h, U ninth) {
    U value;
    __asm__ volatile("move %0,$sp" : "=r"(value));
    return (value >> 32) + ninth;
}
