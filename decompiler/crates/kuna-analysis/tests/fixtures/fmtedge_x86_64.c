/* gcc -O2 -o fmtedge_x86_64 fmtedge_x86_64.c
 * show: the format lives in writable .data and is rewritten before the call.
 * wide: %lc is a wint_t, not a char. */
#include <stdio.h>
#include <string.h>
#include <wchar.h>
char fmtbuf[16] = "v=%d\n";
__attribute__((noinline)) void setmode(int s) { if (s) strcpy(fmtbuf, "v=%s\n"); }
__attribute__((noinline)) void show(const char *p) { printf(fmtbuf, p); }
__attribute__((noinline)) void wide(wint_t c) { if (c > 0x4e00) puts("cjk"); printf("<%lc>\n", c); }
int main(int argc, char **argv) {
  setmode(1);
  show(argv[0]);
  wide(0x4e2d + argc);
  return 0;
}
