/* charbyte compile-and-run oracle: every distinct value-bearing form from the
   decompile-all diffs (fmt/ls/sort/grep -O0/-O2, then 34 binaries), changed
   lines and the unchanged lines that read a retyped byte, evaluated before (off
   arm types) and after (on arm types) for every raw byte 0..255.  x86-64 gcc:
   plain char is signed, as on the targets. */
#include <stdio.h>
#include <string.h>
#include <stdint.h>
typedef uint8_t uint1; typedef int32_t int4; typedef uint32_t uint4; typedef int64_t int8; typedef uint64_t uint8;
static int fails = 0, checks = 0;
#define T1(name, BDECL, BEXPR, ADECL, AEXPR) do { for (int r = 0; r < 256; r++) { \
    unsigned char raw = (unsigned char)r; long long bv, av; \
    { BDECL; bv = (long long)(BEXPR); } { ADECL; av = (long long)(AEXPR); } checks++; \
    if (bv != av) { fails++; printf("FAIL %-28s raw=0x%02x before=%lld after=%lld\n", name, r, bv, av); } } } while (0)
#define UB unsigned char v; memcpy(&v, &raw, 1)
#define CB char v; memcpy(&v, &raw, 1)
#define UP unsigned char buf[2] = {raw, 0}; unsigned char *p = buf
#define CP char buf[2]; memcpy(buf, &raw, 1); buf[1] = 0; char *p = buf
#define UPM unsigned char buf[2] = {0, raw}; unsigned char *p = buf + 2
#define CPM char buf[2]; buf[0] = 0; memcpy(buf + 1, &raw, 1); char *p = buf + 2
int main(void) {
  /* comparisons of the byte */
  T1("*p != 0x2d / '-'",        UP, *p != 0x2d,                       CP, *p != '-');
  T1("*p == 0x2d / '-'",        UP, *p == 0x2d,                       CP, *p == '-');
  T1("*p == 9 / '\\t'",         UP, *p == 9,                          CP, *p == '\t');
  T1("v != 9 / '\\t'",          UB, v != 9,                           CB, v != '\t');
  T1("'`' <= (char)v",          UB, '`' <= (char)v,                   CB, '`' <= v);
  T1("0x1e <= (uc)(v+0x9f)",    UB, 0x1e <= (unsigned char)(v + 0x9f), CB, 0x1e <= (unsigned char)(v + 0x9fU));
  T1("'@' < (char)v",           UB, '@' < (char)v,                    CB, '@' < v);
  T1("(char)v <= '#'",          UB, (char)v <= '#',                   CB, v <= '#');
  T1("'\\x1f' >= (char)v",      UB, '\x1f' >= (char)v,                CB, '\x1f' >= v);
  T1("(uc)(v-0x25) <= 0x1a",    UB, (unsigned char)(v - 0x25) <= 0x1a, CB, (unsigned char)(v - 0x25U) <= 0x1a);
  T1("0x1b <= (uc)(v-0x25)",    UB, 0x1b <= (unsigned char)(v - 0x25), CB, 0x1b <= (unsigned char)(v - 0x25U));
  T1("'\\x1f' < (char)v",       UB, '\x1f' < (char)v,                 CB, '\x1f' < v);
  T1("'#' < (char)v",           UB, '#' < (char)v,                    CB, '#' < v);
  T1("v1 == '-' (index)",       CB, v == '-',                         CB, v == '-');
  /* dat_1d028 (a char global) == the byte: every pair */
  for (int d = 0; d < 256; d++) { unsigned char dr = (unsigned char)d; char dat; memcpy(&dat, &dr, 1);
    T1("dat == (char)*p / *p",  UP, dat == (char)*p,                  CP, dat == *p); }
  /* zero-extensions of the byte: table indexes and widened stores */
  T1("(ul)*p*2 / (ul)(uc)*p*2", UP, (unsigned long)*p * 2,            CP, (unsigned long)(unsigned char)*p * 2);
  T1("(ul)v*2 / (ul)(uc)v*2",   UB, (unsigned long)v * 2,             CB, (unsigned long)(unsigned char)v * 2);
  T1("(l)(i)(ui)*p+0x1c5c0",    UP, (long)(int)(unsigned int)*p + 0x1c5c0, CP, (long)(int)(unsigned int)(unsigned char)*p + 0x1c5c0);
  T1("(ui)*p / (ui)(uc)*p",     UP, (unsigned int)*p,                 CP, (unsigned int)(unsigned char)*p);
  T1("(ul)*p+0x1d740",          UP, (unsigned long)*p + 0x1d740,      CP, (unsigned long)(unsigned char)*p + 0x1d740);
  T1("*v+1+(ul)*p*2",           UP, 1 + (unsigned long)*p * 2,        CP, 1 + (unsigned long)(unsigned char)*p * 2);
  T1("&v5[v1] index",           UB, (long)v,                          CB, (long)(unsigned char)v);
  /* sign-extensions of the byte */
  T1("(int)(char)*p / (int)*p", UP, (int)(char)*p,                    CP, (int)*p);
  T1("(ui)(char)v / (ui)v",     UB, (unsigned int)(char)v,            CB, (unsigned int)v);
  T1("(int)(char)v / (int)v",   UB, (int)(char)v,                     CB, (int)v);
  T1("(int)(char)p[-1]",        UPM, (int)(char)p[-1],                CPM, (int)p[-1]);
  /* character constants stored as bytes: the stored byte must not move */
  T1("v = 10 / '\\n'",          UB; v = 10,   (unsigned char)v,       CB; v = '\n', (unsigned char)v);
  T1("*p = 0x3f / '?'",         UP; *p = 0x3f, (unsigned char)*p,     CP; *p = '?', (unsigned char)*p);
  T1("v = 0x3e / '>'",          UB; v = 0x3e, (unsigned char)v,       CB; v = '>',  (unsigned char)v);
  T1("*p = 0 / '\\0'",          UP; *p = 0,   (unsigned char)*p,      CP; *p = '\0', (unsigned char)*p);
  /* bash -O2 (the >500 KiB spelling: uint1/int4/int8), arithmetic on the byte */
  T1("(int4)(char)(v-0x30)",    UB, (int4)(char)(v - 0x30),          CB, (int4)(char)(v + '\xd0'));
  T1("(int8)(char)(v-0x37)",    UB, (int8)(char)(v - 0x37),          CB, (int8)(char)(v + '\xc9'));
  T1("(int8)(char)(v+0xa9)",    UB, (int8)(char)(v + 0xa9),          CB, (int8)(char)(v + '\xa9'));
  T1("(int8)((char)v+-0x30)",   UB, (int8)((char)v + -0x30),         CB, (int8)(v + -0x30));
  T1("v = v - 0x30 / v += '\\xd0'", UB; v = v - 0x30, (unsigned char)v, CB; v += '\xd0', (unsigned char)v);
  T1("v = v - 0x37 / v += '\\xc9'", UB; v = v - 0x37, (unsigned char)v, CB; v += '\xc9', (unsigned char)v);
  T1("v = v + 0xa9 / v += '\\xa9'", UB; v = v + 0xa9, (unsigned char)v, CB; v += '\xa9', (unsigned char)v);
  T1("!(*p-0x2b & 0xfd)",       UP, !(*p - 0x2b & 0xfd),             CP, !(*p - 0x2bU & 0xfdU));
  T1("10 <= (uint1)(*p-0x30)",  UP, 10 <= (uint1)(*p - 0x30),        CP, 10 <= (uint1)((char)*p - 0x30U));
  T1("6 <= (uint1)(v+0x9f)",    UB, 6 <= (uint1)(v + 0x9f),          CB, 6 <= (uint1)(v + 0x9fU));
  T1("6 <= (uint1)(v+0xbf)",    UB, 6 <= (uint1)(v + 0xbf),          CB, 6 <= (uint1)(v + 0xbfU));
  T1("8 <= (uint1)(v-0x30)",    UB, 8 <= (uint1)(v - 0x30),          CB, 8 <= (uint1)(v - 0x30U));
  T1("(uint1)((uint4)v-0x47) <= 0x33", UB, (uint1)((uint4)v - 0x47) <= 0x33, CB, (uint1)((uint4)(uint1)v - 0x47) <= 0x33);
  T1("bitset >> ((uint4)v-0x47)", UB, (uint1)((uint4)v - 0x47) <= 0x33 && (0xbfb93fc0019a1ULL >> ((uint8)((uint4)v - 0x47) & 0x3f) & 1),
                                CB, (uint1)((uint4)(uint1)v - 0x47) <= 0x33 && (0xbfb93fc0019a1ULL >> ((uint8)((uint4)(uint1)v - 0x47) & 0x3f) & 1));
  T1("(uint8)*(uint1 *)p",      UP, (uint8)*(uint1 *)p,              CP, (uint8)(uint1)*(char *)p);
  T1("(uint8)v / (uint8)(uint1)v", UB, (uint8)v,                     CB, (uint8)(uint1)v);
  T1("v == 0x40 || v == 0x2a",  UB, v == 0x40 || v == 0x2a,          CB, v == '@' || v == '*');
  T1("v != 0x74 / 't'",         UB, v != 0x74,                       CB, v != 't');
  T1("*p == 0x7f / '\\x7f'",  UP, *p == 0x7f,                      CP, *p == '\x7f');
  T1("(uint1)(v-0x30U) <= 9",   UB, (uint1)(v - 0x30) <= 9,          CB, (uint1)(v - 0x30U) <= 9);
  /* forms from the 34-binary census (analysis.md section 5), including every
     UNCHANGED line that reads a retyped byte or dereferences a retyped pointer */
  T1("(ui)(char)v / (ui)v sort", UB, (unsigned int)(char)v,           CB, (unsigned int)v);
  T1("(ui)(char)p[2] / (ui)p[2]", UP, (unsigned int)(char)p[0],       CP, (unsigned int)p[0]);
  T1("(int4)(char)v strchr",    UB, (int4)(char)v,                   CB, (int4)v);
  T1("(uint1)(*p - 0x23)",      UP, (uint1)(*p - 0x23),              CP, (uint1)(*p - 0x23));
  T1("byte = -v",               UB; v = -v, (unsigned char)v,        CB; v = -v, (unsigned char)v);
  T1("*q = v + 0x30",           UB; unsigned char q = v + 0x30, q,   CB; char q = v + 0x30, (unsigned char)q);
  T1("v & 0x10",                UB, (v & 0x10) != 0,                 CB, (v & 0x10) != 0);
  T1("copy v = *p (unchanged)", UP; unsigned char w = *p, w,         CP; char w = *p, (unsigned char)w);
  T1("store *p = v (unchanged)", UB; unsigned char d[1]; d[0] = v, d[0], CB; char d[1]; d[0] = v, (unsigned char)d[0]);
  T1("truth !v (unchanged)",    UB, !v,                              CB, !v);
  T1("truth *p (unchanged)",    UP, *p != 0,                         CP, *p != 0);
  T1("v1 == '\\x01'",           UB, v == 1,                          CB, v == '\x01');
  T1("*p == '\\x7f'",           UP, *p == 0x7f,                      CP, *p == '\x7f');
  T1("'\\x1f' < v",             UB, '\x1f' < (char)v,                CB, '\x1f' < v);
  T1("(uint8)(uint1)v * 4",     UB, (uint8)v * 4,                    CB, (uint8)(uint1)v * 4);
  T1("(char *)(uint8)(uint1)v", UB, (long long)(uint8)v,             CB, (long long)(uint8)(uint1)v);
  /* two retyped bytes compared with each other (ls 0x7920 *a1 == *v17, tar,
     bash v2 != v17): both sides move together, every pair */
  for (int d = 0; d < 256; d++) { unsigned char dr = (unsigned char)d;
    T1("*a == *b both retyped", UB; unsigned char w = dr, v == w,    CB; char w; memcpy(&w, &dr, 1), v == w); }
  /* tar 0x41370's switch over a byte (labels 0 .. 0x7a, default for the rest) */
  T1("switch labels < 0x80",    UB, v == 0x25 ? 1 : v == 0x7a ? 2 : v == 0 ? 3 : 4, CB, v == 0x25 ? 1 : v == 0x7a ? 2 : v == 0 ? 3 : 4);
  /* the stage fixture's two comparisons: '-' and a constant >= 0x80 */
  T1("stage isdash",            UB, v == 0x2d,                        CB, v == '-');
  T1("stage ishigh",            UB, v == 0xe9,                        CB, v == '\xe9');
  int real = fails;
  /* controls: the forms the rule avoids must be caught.  A char byte against an
     unsigned 0xe9; a byte passed to a variadic call (logit("%d",c)); a switch
     on a char byte with a case label past 0x7f (highcase). */
  int c0 = fails;
  T1("CONTROL char v == 0xe9",  UB, v == 0xe9,                        CB, v == 0xe9);
  int c1 = fails;
  T1("CONTROL call arg promote", UB, (int)v,                          CB, (int)v);
  int c2 = fails;
  T1("CONTROL switch case 0x80", UB, v == 0x80 ? 7 : 1,               CB, v == 0x80 ? 7 : 1);
  int c3 = fails;
  printf("checks=%d fails=%d controls_caught=%d/%d/%d\n", checks, real, c1 - c0, c2 - c1, c3 - c2);
  return real != 0 || c1 == c0 || c2 == c1 || c3 == c2;
}
