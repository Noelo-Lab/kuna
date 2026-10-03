/* The second argument register is also the low word of a returned 64-bit
 * value on a big-endian ARM or PowerPC ABI (r1, r4). Carrying the argument
 * across a call into that word takes a callee-saved register and a move
 * back: a return value, not the register left as it arrived. The
 * little-endian ARM build is the control. */
void ext(void);
unsigned long long carry_b(unsigned int a, unsigned int b) { ext(); return b; }
unsigned long long carry_if(unsigned int a, unsigned int b) { if (a) ext(); return b; }
unsigned long long carry_low(unsigned long long x) { ext(); return x & 0xffffffffu; }
