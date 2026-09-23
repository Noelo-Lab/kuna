/* (kuna castimplied) Conversions C performs by itself.  Each function widens a
 * value where C's own conversion rules already do it: into a libc argument, into
 * a variable of the wider type, out through `return`, or under another
 * conversion.  The last group changes the value (a sign change, a varargs
 * argument) and must keep its cast.  The round-trip test in
 * kuna-cli/tests/decompile_all_cli.rs compiles the functions as kuna prints them,
 * with the option off and on, and checks both print what this program prints:
 *
 *   2 2 -1 82 251 1
 *   1648 936 1300 50
 *   -7 4294967280 -100 1 0
 *   -89 27
 *   2147483645 -2 2147483647 1
 *   -42
 *
 * Built with:
 *   gcc   -O0 -o castimplied_gcc_O0_x86_64   castimplied_x86_64.c
 *   clang -O0 -o castimplied_clang_O0_x86_64 castimplied_x86_64.c
 */
#include <ctype.h>
#include <stdio.h>
#include <string.h>
#define NI __attribute__((noinline))

/* a call argument converted to the declared parameter type */
NI long arg_memchr(const char *s, char c, unsigned int n) {
  const char *p = memchr(s, c, n);
  return p ? p - s : -1;
}
NI int arg_toupper(unsigned char c) { int r = toupper(c); return r + 1; }
NI long arg_strchr(const char *s, short c) {
  const char *p = strchr(s, c);
  return p ? p - s : -1;
}

/* an assignment converts to the type of the left operand */
NI long asg_char(const char *s, int n) {
  long sum = 0;
  for (int i = 0; i < n; i++) { long x; x = s[i]; sum += x * x + x; }
  return sum;
}
NI long asg_uint(const unsigned int *s, int n) {
  long sum = 0;
  for (int i = 0; i < n; i++) { long x; x = s[i]; sum += x % 1000 + (x & 3); }
  return sum;
}
NI int asg_short(const short *s, int n) {
  int best = -100000;
  for (int i = 0; i < n; i++) { int x; x = s[i]; if (x > best) best = x; }
  return best;
}
NI int asg_uchar(const unsigned char *s, int n) {
  int best = -100000;
  for (int i = 0; i < n; i++) { int x; x = s[i]; if (x - 200 > best) best = x - 200; }
  return best;
}

/* return converts as if by assignment to the return type */
NI long ret_int(int i) { return i; }
NI unsigned long ret_uint(unsigned int u) { return u; }
NI int ret_char(char c) { return c; }
NI int ret_less(int a, int b) { return a < b; }

/* a widening under another conversion that fixes the type */
NI unsigned char to_uchar(char c) { return c; }
NI char lookup(const char *table, char c) { int i = to_uchar(c); return table[i]; }

/* these change the value, or C does not convert them: the cast stays */
NI long keep_inner(unsigned int u) { return (long)(int)u + (u >> 1); }
NI long keep_inner2(unsigned int u, unsigned int *o) { *o = u >> 1; return (long)(int)u; }
NI long keep_size(const char *s, char c, int n) {
  const char *p = memchr(s, c, n);
  return p ? p - s : -1;
}
NI void keep_vararg(int i) { printf("%ld\n", (long)i); }

int main(void) {
  static const char neg[] = {-5, 7, -40, 3};
  static const unsigned int big[] = {0xfffffff0u, 5, 0x80000001u};
  static const short sh[] = {-300, 1300, -2};
  static const unsigned char uc[] = {250, 3, 128};
  char table[256];
  for (int i = 0; i < 256; i++) table[i] = (char)(i ^ 0x5a);
  printf("%ld %ld %ld %d %d %ld\n", arg_memchr("ab\xfb" "c", (char)-5, 4), arg_memchr("abc", 'c', 3),
         arg_memchr("abc", 'z', 3), arg_toupper('q'), arg_toupper(250), arg_strchr("x\xfey", (short)-2));
  printf("%ld %ld %d %d\n", asg_char(neg, 4), asg_uint(big, 3), asg_short(sh, 3), asg_uchar(uc, 3));
  printf("%ld %lu %d %d %d\n", ret_int(-7), ret_uint(0xfffffff0u), ret_char((char)-100),
         ret_less(-1, 1), ret_less(2, 1));
  printf("%d %d\n", lookup(table, (char)-3), lookup(table, 'A'));
  unsigned int half;
  long k2 = keep_inner2(0xfffffffeu, &half);
  printf("%ld %ld %u %ld\n", keep_inner(0xfffffffeu), k2, half, keep_size("abc", 'b', 3));
  keep_vararg(-42);
  return 0;
}
