/* Wrappers that hand back a callee's result, and float bits kept in integers.
   gcc -O0 -o floatret_wrap_gcc_O0 floatret_wrap.c
   clang -O0 -o floatret_wrap_clang_O0 floatret_wrap.c
   gcc -O2 -o floatret_wrap_gcc_O2 floatret_wrap.c   (not stripped) */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#define NI __attribute__((noinline))
int gi = 0x3fc00000;
NI int set_tz(const char *tz) { if (tz) return setenv("TZ", tz, 1); return unsetenv("TZ"); }
NI int chdir_long(const char *name) { return (int)strlen(name) - 1; }
NI int restore_cwd(int fd, const char *name) { if (0 <= fd) return fchdir(fd); return chdir_long(name); }
NI float gi_as_f(void) { float f; memcpy(&f, &gi, 4); return f; }
NI void set_gi_bits(float f) { memcpy(&gi, &f, 4); }
NI int use_gi(void) { return gi + 1; }
NI float negf(float x) { return -x; }
NI float absf(float x) { return __builtin_fabsf(x); }
NI double negd(double x) { return -x; }
NI float wrapneg(float x) { return negf(x); }
NI float wrapabs(float x) { float r = absf(x); return r; }
NI double wrapnegd(double x) { return negd(x); }
NI float twice(float x) { return negf(negf(x)); }
static unsigned long long bits(const void *p, int n) { unsigned long long u = 0; memcpy(&u, p, n); return u; }
int main(int argc, char **argv) {
  (void)argv;
  int r = set_tz(argc > 1 ? "UTC" : NULL), s = restore_cwd(-argc, "/tmp");
  float g = gi_as_f();
  set_gi_bits(-7.5f);
  int u = use_gi();
  float a = wrapneg(argc + 0.5f), b = wrapabs(-argc - 0.25f), d = twice(argc * 3.0f);
  double c = wrapnegd(argc * 1.5);
  printf("%d %d %llx %x %llx %llx %llx %llx\n", r, s, bits(&g, 4), u, bits(&a, 4), bits(&b, 4), bits(&c, 8), bits(&d, 4));
  return 0;
}
