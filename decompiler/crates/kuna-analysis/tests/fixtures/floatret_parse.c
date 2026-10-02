double strtod(const char *, char **); float strtof(const char *, char **);
#define NI __attribute__((noinline))
unsigned long long sinku; double sinkd; float sinkf; unsigned sinkuf;
NI double parse(const char *s) { return strtod(s, 0); }
NI double parse2(const char *s) { return strtod(s, 0); }
NI float parsef(const char *s) { return strtof(s, 0); }
NI void rp(const char *s) { double d = parse(s); unsigned long long u; __builtin_memcpy(&u, &d, 8); sinku = u; }
NI void rp2(const char *s) { sinkd = parse2(s); }
NI void rpf(const char *s) { float f = parsef(s); unsigned u; __builtin_memcpy(&u, &f, 4); sinkuf = u; }
