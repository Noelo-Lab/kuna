/* pow2select fixture: signed division and remainder by 2^k, plus near misses */
int div4(int x) { return x / 4; }
long ldiv32(long x) { return x / 32; }
int mod8(int x) { return x % 8; }
int mod2(int x) { return x % 2; }
int near1(int x) { int v = x + 3; if (x > 5) v = x; return v >> 2; }
int near3(int x) { int v = x + 7; if (x >= 0) v = x; return v >> 2; }
int near4(int x) { return (x < -4) ? -(-x & 7) : (x & 7); }
