/* Fixture for `formatcallees`: format functions no library table declares.
   gcc -O2 -o fmtcallees_x86_64 fmtcallees.c && strip fmtcallees_x86_64 */
#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>

static int verbose = 1;

__attribute__((noinline)) void log_msg(int level, const char *fmt, ...) {
  va_list ap;
  if (level > verbose)
    return;
  va_start(ap, fmt);
  vfprintf(stderr, fmt, ap);
  va_end(ap);
}

__attribute__((noinline)) int parse_kv(const char *s, const char *fmt, ...) {
  va_list ap;
  va_start(ap, fmt);
  int n = vsscanf(s, fmt, ap);
  va_end(ap);
  return n;
}

__attribute__((noinline)) void vreport(const char *fmt, va_list ap) {
  vfprintf(stdout, fmt, ap);
}

__attribute__((noinline)) void report(const char *fmt, ...) {
  va_list ap;
  va_start(ap, fmt);
  vreport(fmt, ap);
  va_end(ap);
}

__attribute__((noinline)) void note(const char *what) {
  log_msg(0, "%s\n", what);
}

int main(int argc, char **argv) {
  int a = 0, b = 0;
  double d = argc * 1.5;
  if (argc < 2) {
    log_msg(0, "usage: %s <spec>\n", argv[0]);
    return 2;
  }
  log_msg(1, "args=%d first=%s d=%f\n", argc, argv[1], d);
  if (parse_kv(argv[1], "%d:%d", &a, &b) != 2)
    report("bad spec '%s' (%d)\n", argv[1], argc);
  note("50% done");
  log_msg(1, "a=%d b=%d c=%c\n", a, b, argv[1][0]);
  return a + b;
}
