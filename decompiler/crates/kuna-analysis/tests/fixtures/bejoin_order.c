/* Callers placed before the callees whose 64-bit return they read, and a
 * self-recursive function reading its own pair.  widen and self sign-extend
 * an int into the long long they return; D_mix and E_self, and self itself,
 * read both words of the pair back.  Built -DLINK for a linked MIPS image. */
typedef long long i64;
void ext(void);
static i64 widen(int x);
static i64 self(int n);
int D_mix(int x) { i64 v = widen(x); return (int)(v >> 32) * 3 + (int)v; }
int E_self(int n) { i64 v = self(n); return (int)(v >> 32) * 3 + (int)v; }
static __attribute__((noinline, used)) i64 widen(int x) { ext(); return x; }
static __attribute__((noinline, used)) i64 self(int n) {
  int r = n * 9;
  if (n > 0 && n < 8) { i64 v = self(n - 1); r = (int)(v >> 32) * 3 + (int)v - n; }
  return (i64)r;
}
#ifdef LINK
void ext(void) {}
void __start(void) {}
#endif
