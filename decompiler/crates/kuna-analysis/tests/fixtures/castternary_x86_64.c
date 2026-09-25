/* (kuna castternary) Conversions the conditional operator performs by itself.
 * `b64_decode` is a textbook base64 decoder: every input character is looked up
 * in a signed decode table unless it is the '=' pad, so each quantum is four
 * `c != '=' ? table[c] : 0` conditionals.  The small functions put one widened
 * arm against the other kinds of arm: an int constant, a negative constant, an
 * unsigned constant, a long literal, an unsigned long, and a second cast.  The
 * last group must keep its cast: the conversions of the two bare arms would give
 * the conditional another type, or the cast changes the value.  The round-trip test
 * in kuna-cli/tests/decompile_all_cli.rs compiles the functions as kuna prints
 * them, with the option off and on, and checks both print what this program
 * prints:
 *
 *   13 72 101 108 108 111 32 119 111 114 108 100 63 251
 *   -1 -128 127 -1 0 255 128 -1
 *   -300 7 4294967295 4294967168 0
 *   -1 -7 -9 -128 -1 -9
 *   4294967295 5 4294967295 3000000000 -9 4294967295 18446744073709551615
 *   -5 4294967291 -1 -5 -7 3000000000
 *   4294967168 4294967295 1 4294967295 -3.5
 *
 * Built with:
 *   gcc   -O0 -o castternary_gcc_O0_x86_64   castternary_x86_64.c
 *   clang -O0 -o castternary_clang_O0_x86_64 castternary_x86_64.c
 */
#include <stdio.h>
#include <string.h>
#define NI __attribute__((noinline))

/* the table maps a base64 character to its six bits, anything else to -1 */
NI void b64_table(signed char *t) {
  static const char alphabet[] =
      "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
  memset(t, -1, 256);
  for (int i = 0; i < 64; i++) t[(unsigned char)alphabet[i]] = (signed char)i;
}

NI unsigned long b64_decode(const signed char *t, const char *in, unsigned long len,
                            unsigned char *out) {
  unsigned long n = 0;
  for (unsigned long i = 0; i + 3 < len; i += 4) {
    int a = in[i] != '=' ? t[(unsigned char)in[i]] : 0;
    int b = in[i + 1] != '=' ? t[(unsigned char)in[i + 1]] : 0;
    int c = in[i + 2] != '=' ? t[(unsigned char)in[i + 2]] : 0;
    unsigned int d = in[i + 3] != '=' ? t[(unsigned char)in[i + 3]] : 0;
    d += a * 0x40000 + b * 0x1000 + c * 0x40;
    out[n++] = (unsigned char)(d >> 16);
    if (in[i + 2] != '=') out[n++] = (unsigned char)(d >> 8);
    if (in[i + 3] != '=') out[n++] = (unsigned char)d;
  }
  return n;
}

/* one byte arm against an int arm: the conditional is an int either way */
NI int arm_char(const char *p, int k) { int v = k ? p[k] : 0; return v; }
NI int arm_uchar(const unsigned char *p, int k) { int v = k ? p[k] : -1; return v; }
NI int arm_short(const short *p, int k) { int v = k ? p[k] : 7; return v; }
NI unsigned int arm_char_uint(const char *p, int k) { unsigned int v = k ? p[k] : 0; return v; }

/* a narrower arm against a wider arm of the cast's type (the four loads
 * print as if/else, where castimplied already leaves the widening out) */
NI long arm_long(const int *p, const long *q, int k) { long v = k ? p[k] : q[k]; return v; }
NI long arm_char_long(const char *p, const long *q, int k) { long v = k ? p[k] : q[k]; return v; }
NI unsigned long arm_ulong(const unsigned int *p, const unsigned long *q, int k) {
  unsigned long v = k ? p[k] : q[k];
  return v;
}
NI long arm_uint_long(const unsigned int *p, const long *q, int k) { long v = k ? p[k] : q[k]; return v; }
NI unsigned long arm_uint_max(const unsigned int *p, int k) {
  unsigned long v = k ? p[k] : 0xffffffffffffffffUL;
  return v;
}
NI long arm_both(int i, unsigned int u, int k) { long v = k ? (long)i : (long)u; return v; }
NI long arm_wide_literal(const int *p, int k) { long v = k ? p[k] : 3000000000L; return v; }
NI unsigned int arm_all_ones(const char *p, int k) { unsigned int v = k ? (unsigned int)p[k] : 0xffffffffu; return v; }

/* these keep their cast: the bare arms would meet at another type, or the
 * cast changes the value */
NI long keep_narrow(const int *p, int k) { long v = k ? p[k] : -5L; return v; }
NI unsigned int keep_less(int a, int b, int k) { unsigned int v = k ? (unsigned int)(a < b) : 0xffffffffu; return v; }
NI double keep_float(float f, double d, int k) { double v = k ? (double)f : d; return v; }

int main(void) {
  signed char t[256];
  b64_table(t);
  static const char txt[] = "SGVsbG8gd29ybGQ/+w==";
  unsigned char out[16];
  unsigned long n = b64_decode(t, txt, strlen(txt), out);
  static const char bytes[] = {0, -1, -128, 0x7f, (char)0xff, 'H'};
  static const unsigned char ub[] = {0, 0xff, 0x80, 0xde};
  static const short sh[] = {0, -300, 7};
  static const int in[] = {0, -1, -7};
  static const unsigned int ui[] = {0, 0xffffffffu, 3000000000u};
  static const long lg[] = {-9, 5, -1};
  static const unsigned long ul[] = {5, 0xfffffffffffffff0UL, 1};
  printf("%lu", n);
  for (unsigned long i = 0; i < n; i++) printf(" %u", out[i]);
  printf("\n%d %d %d %d %d %d %d %d\n", arm_char(bytes, 1), arm_char(bytes, 2), arm_char(bytes, 3),
         arm_char(bytes, 4), arm_char(bytes, 0), arm_uchar(ub, 1), arm_uchar(ub, 2), arm_uchar(ub, 0));
  printf("%d %d %u %u %u\n", arm_short(sh, 1), arm_short(sh, 0), arm_char_uint(bytes, 1),
         arm_char_uint(bytes, 2), arm_char_uint(bytes, 0));
  printf("%ld %ld %ld %ld %ld %ld\n", arm_long(in, lg, 1), arm_long(in, lg, 2), arm_long(in, lg, 0),
         arm_char_long(bytes, lg, 2), arm_char_long(bytes, lg, 4), arm_char_long(bytes, lg, 0));
  printf("%lu %lu %ld %ld %ld %lu %lu\n", arm_ulong(ui, ul, 1), arm_ulong(ui, ul, 0), arm_uint_long(ui, lg, 1),
         arm_uint_long(ui, lg, 2), arm_uint_long(ui, lg, 0), arm_uint_max(ui, 1), arm_uint_max(ui, 0));
  printf("%ld %ld %ld %ld %ld %ld\n", arm_both(-5, 7, 1), arm_both(-5, 0xfffffffbu, 0), keep_narrow(in, 1),
         keep_narrow(in, 0), arm_wide_literal(in, 2), arm_wide_literal(in, 0));
  printf("%u %u %u %u %.1f\n", arm_all_ones(bytes, 2), arm_all_ones(bytes, 0), keep_less(1, 2, 1), keep_less(1, 2, 0),
         keep_float(-3.5f, 2.0, 1));
  return 0;
}
