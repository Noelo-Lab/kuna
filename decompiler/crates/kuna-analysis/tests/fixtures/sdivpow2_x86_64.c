/* sdivpow2 fixture: signed divisions by a power of two, whose correction of a
 * negative dividend gcc and clang emit as a conditional move.  Built with
 * gcc/clang -std=gnu11 -fno-inline -O0/-O2 -c; the round trip is
 * kuna-cli/tests/sdivpow2_round_trip.rs. */
int div2_i(int x) { return x / 2; }
int div8_i(int x) { return x / 8; }
int div1024_i(int x) { return x / 1024; }
int div_top_i(int x) { return x / 0x40000000; }
long div2_l(long x) { return x / 2; }
long div8_l(long x) { return x / 8; }
long div1024_l(long x) { return x / 1024; }
long div_top_l(long x) { return x / 0x4000000000000000L; }
int scaled(int bytes) { return (bytes + 512) / 1024; }
int round8(int x) { return x / 8 * 8; }
int split(int x, int *rem) { int q = x / 16; *rem = x - q * 16; return q; }
long word_bit(const unsigned long *bits, int n) { return (bits[n / 64] >> (n % 64)) & 1; }
