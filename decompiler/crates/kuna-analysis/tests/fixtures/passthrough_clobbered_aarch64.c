/*
 * passthrough_clobbered_aarch64 -- a register an intervening call may clobber
 * is not forwarded.
 *
 *     clang-21 -target aarch64-linux-gnu -O2 -fno-inline -fPIC \
 *         -c passthrough_clobbered_aarch64.c -o p.o
 *     ld.lld-21 -shared p.o -o passthrough_clobbered_aarch64
 *
 * dash `killcmd`, reduced. `check` takes two arguments and never sets x2, so
 * at the `vlog` call x2 is whatever the import `ext` left there. The AAPCS64
 * lets `ext` clobber x2 and the cspec gives the call an unknown effect on it,
 * so heritage carries the incoming x2 through the call's non-creating
 * INDIRECT. `vlog` saves x1..x7 for `va_start`, so its recovered prototype
 * reads x2, and forwarding through an unproven call printed
 * `int check(unsigned int a0,long a1,unsigned long a2)` with
 * `vlog("bad %s\n",..,a2)`.
 */
extern int ext(int);
extern int emit(int, int, int);
extern int vprintf(const char *, __builtin_va_list);
__attribute__((noinline)) int vlog(const char *fmt, ...) {
  __builtin_va_list ap;
  __builtin_va_start(ap, fmt);
  int r = vprintf(fmt, ap);
  __builtin_va_end(ap);
  return r;
}
__attribute__((noinline)) int check(int a, char **v) {
  int r = 0;
  for (int i = 0; i < a; i++) {
    int s = ext(i);
    if (s < 0) vlog("bad %s\n", v[i]);
    r += s;
  }
  emit(r, a, 5);
  return r;
}
