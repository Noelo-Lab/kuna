/* SPARC's restore hands %i1 back in %o1 on every return, so a function that
 * returns one int in %o0 also seems to return %o1, holding something else:
 * clang -O0 zeroes %i1 to store a byte (mark), the loop in back4 exits on the
 * zero byte it last loaded into %i1, and zero_after and one_after hand the
 * second argument back after passing it to a call. sum_or returns -1 with the
 * second argument still in %i1 on one path, and on the other the loop counter
 * it counted down there. */
void ext2(int a, int b);
void ext(void);
int mark(char *s, int n) { s[n] = 0; s[0] = 0; return n + 1; }
int back4(const char *p) { while (p[-1] || p[-2] || p[-3] || p[-4]) p--; return p[-5] + 100; }
int zero_after(int a, int b) { ext2(b, a); return 0; }
int one_after(int a, int b) { ext2(b, a); return a > 0; }
int sum_or(int *p, int n) { if (!p) return -1; int s = 0; for (int i = 0; i < n; i++) s += p[i]; ext(); return s; }
