/* 4-byte wide literals passed to the image's own functions, beside int tables
   of character codes.  "bind" is a suffix of "xbind", so a linker that merges
   string tails passes an address inside it; "second-msg" is reached only
   through the pointer table msgs.  codes is NUL-terminated and spells "Hello",
   weeks holds two characters, rows is indexed by a computed row, and code's
   switch becomes a lookup table of character codes that runs on past its zero
   case.  Built with WIDESTR32_HARNESS it supplies everything but wide, wide32,
   ornull, suffix, first, table and weekly, and a main that prints what each
   stores. */
#include <stddef.h>
typedef unsigned int char32;
#ifdef __clang__
#define NI __attribute__((noinline))
#else
#define NI __attribute__((noipa))
#endif
long out;
const int codes[6] = {72, 101, 108, 108, 111, 0};
const int weeks[8] = {52, 53, 52, 52, 52, 53, 52, 0};
NI long hashw(const wchar_t *s) { long h = 7; for (long i = 0; s[i]; i++) h = h * 31 + s[i]; return h; }
NI long hash32(const char32 *s) { long h = 3; for (long i = 0; s[i]; i++) h = h * 37 + (long)s[i]; return h; }
NI long sum(const int *p, int n) { long s = 0; for (int i = 0; i < n; i++) s += p[i]; return s; }
#ifndef WIDESTR32_HARNESS
static const wchar_t *const msgs[2] = {L"first-msg", L"second-msg"};
static const wchar_t rows[2][8] = {L"alpha", L"bravo"};
NI void wide(void) { out = hashw(L"hellow"); }
NI void wide32(void) { out = hash32(U"char32-text"); }
NI void ornull(const wchar_t *s) { if (!s) s = L"(NULL)"; out = hashw(s); }
NI void suffix(void) { out = hashw(L"xbind") * 3 + hashw(L"bind"); }
NI void first(void) { out = hashw(msgs[0]); }
NI void pick(int i) { out = hashw(msgs[i & 1]); }
NI void row(int i) { out = hashw(rows[i & 1]); }
NI void table(int n) { out = sum(codes, n); }
NI void weekly(int n) { out = sum(weeks, n); }
NI int code(unsigned x)
{
    switch (x) {
    case 0: return 'h';
    case 1: return 'e';
    case 2: return 'l';
    case 3: return 'p';
    case 4: return 'z';
    case 5: return 0;
    case 6: return 'q';
    default: return -1;
    }
}
int main(int c, char **v)
{
    wide(); wide32(); ornull(0); suffix(); first(); pick(c); row(c); table(c); weekly(c);
    return (int)out + code(c);
}
#else
#include <stdio.h>
void wide(void);
void wide32(void);
void ornull(const wchar_t *);
void suffix(void);
void first(void);
void table(int);
void weekly(int);
int main(void)
{
    wide();
    printf("wide %ld\n", out);
    wide32();
    printf("wide32 %ld\n", out);
    ornull(0);
    printf("ornull0 %ld\n", out);
    ornull(L"given");
    printf("ornull1 %ld\n", out);
    suffix();
    printf("suffix %ld\n", out);
    first();
    printf("first %ld\n", out);
    for (int n = 0; n <= 6; n++) {
        table(n);
        printf("table %d %ld\n", n, out);
        weekly(n);
        printf("weekly %d %ld\n", n, out);
    }
    return 0;
}
#endif
