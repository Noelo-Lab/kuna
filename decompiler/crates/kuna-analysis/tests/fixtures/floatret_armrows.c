double strtod(const char *, char **);
unsigned long long u, w;
__attribute__((noinline)) double parse(const char *s) { return strtod(s, 0); }
__attribute__((noinline)) double clampd(const char *s) { double x = strtod(s, 0); if (x < 0) return 0; return x; }
__attribute__((noinline)) void rd(const char *s) { double d = clampd(s); __builtin_memcpy(&u, &d, 8); }
__attribute__((noinline)) void rp(const char *s) { double d = parse(s); __builtin_memcpy(&w, &d, 8); }
void __start(void) { rd("2.25"); rp("2.25"); for (;;); }
