/* cmpmerge fixture */
__attribute__((noinline)) int sink(int x) { return x * 7 + 3; }
int le(int a, int b) { if (a < b || a == b) return sink(1); return 0; }
int ge3(int x) { if (x == 3 || x > 3) return sink(2); return 1; }
int fge(double a, double b) { if (a > b || a == b) return sink(3); return 2; }
int fnle(double a, double b) { return a != b && !(a < b); }
unsigned ule(unsigned a, unsigned b) { return (a < b) | (a == b); }
