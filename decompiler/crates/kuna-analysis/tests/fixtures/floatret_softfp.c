double sqrt(double); double pow(double,double); double ceil(double); double strtod(const char*,char**); float strtof(const char*,char**);
double g1, g2, g3; float gf; unsigned long long gbits;
__attribute__((noinline)) double hyp(double a, double b){ return sqrt(b); }
__attribute__((noinline)) double pw(double a, double k){ return pow(k, a); }
__attribute__((noinline)) double parse(const char *s){ return strtod(s, 0); }
__attribute__((noinline)) float parsef(const char *s){ return strtof(s, 0); }
__attribute__((noinline)) void rd(const char *s){ g1 = strtod(s,0); gf = strtof(s,0); g2 = ceil(g1); }
void __start(void){ rd("1"); g1 = hyp(g2, g3); g2 = pw(g1, g3); g3 = parse("x"); gf = parsef("y"); for(;;); }
