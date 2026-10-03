/* SPARC functions that return one int in %o0 while %i1, which restore hands
 * back in %o1, holds something else. */
void ext(void);
unsigned bound(int *s, unsigned n) {
  unsigned a = n + (n >> 3) + 4;
  unsigned b = n + (n >> 5) + 7;
  ext();
  if (s[3]) return (a > b ? a : b) + 6;
  if (s[0] != 15) return (s[1] ? a : b) + s[2];
  return n + (n >> 12) + 13 + s[2];
}
int pgetc_like(int *p) {
  ext();
  if (p[0]) return p[2 + --p[0]];
  if (--p[1] >= 0) return p[5 + p[1]];
  return -1;
}
int expand(int *it, int n) {
  ext();
  if (n < 0) return 2;
  for (int i = 0; i < n; i++) {
    if (it[i] < 0) return 2;
    it[i] += 1;
  }
  return 0;
}
int ibyte(unsigned char *p) { ext(); return *p; }
unsigned ubyte(unsigned char *p) { ext(); return *p; }
int ihalf(unsigned short *p) { ext(); return *p; }
int ibyte_leaf(unsigned char *p) { return *p; }
