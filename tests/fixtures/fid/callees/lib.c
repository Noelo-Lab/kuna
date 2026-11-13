/* Callee fixture. `wrap_a` and `wrap_b` compile to the same
 * instructions and differ only in the function they call, so only their
 * callees tell them apart; `scale_a` and `scale_b` have bodies of their
 * own. `bump` is a stub of at most 16 bytes with no callee. */

__attribute__((noinline)) int scale_a(int x) { return ((x * 37 + 11) ^ (x >> 3)) - (x << 2); }
__attribute__((noinline)) int scale_b(int x) { return ((x ^ 0x5a5a) - 3) * (x | 1) + (x >> 5); }

__attribute__((noinline)) int wrap_a(int x) { return scale_a(x * 3 + 1) * 5 + (x ^ 9); }
__attribute__((noinline)) int wrap_b(int x) { return scale_b(x * 3 + 1) * 5 + (x ^ 9); }

__attribute__((noinline)) int bump(int x) { return (x * 5 + 3) * x - 1; }
