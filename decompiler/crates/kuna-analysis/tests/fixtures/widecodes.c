/* Arrays of 2- and 4-byte character codes whose first element reads as a
   one-character string, passed by address beside genuine one-character
   literals.  The compilers lay "x", "y", "w" and "a" side by side, and only
   the table `modes` reaches "w" and "a".  The bytes of `mixed` say nothing
   about its element width; only its symbol does.  Built with
   WIDECODES_HARNESS it supplies everything but go, go16, gomixed, narrow and
   opened, and a main that prints what each stores. */
#include <wchar.h>
#ifdef __clang__
#define NI __attribute__((noinline))
#else
#define NI __attribute__((noipa))
#endif
const int codes[4] = {65, 66, 67, 68};
const short halves[4] = {104, 105, 106, 0};
const int mixed[2] = {65, 1000};
int out;
const char *mode(int i);
int pair(const char *a, const char *b);
#ifndef WIDECODES_HARNESS
NI void opened(int n) { out = pair("x", mode(n)); }
#endif
const char *const modes[3] = {"w", "a", "ab"};
NI const char *mode(int i) { return modes[i & 1]; }
NI int sum(const int *p, int n) { int s = 0; for (int i = 0; i < n; i++) s += p[i]; return s; }
NI int sum16(const short *p, int n) { int s = 0; for (int i = 0; i < n; i++) s += p[i]; return s; }
NI long lenw(const wchar_t *s) { long n = 0; while (s[n]) n++; return n; }
NI int pair(const char *a, const char *b) { return a[0] * 1000 + b[0] + a[1] + b[1]; }
#ifndef WIDECODES_HARNESS
NI void go(int n) { out = sum(codes, n); }
NI void go16(int n) { out = sum16(halves, n); }
NI void gomixed(int n) { out = sum(mixed, n); }
NI void wide(void) { out = lenw(L"hellow"); }
NI void narrow(void) { out = pair("x", "y"); }
int main(int c, char **v) { go(c); go16(c); gomixed(c); wide(); narrow(); opened(c); return out; }
#else
#include <stdio.h>
void go(int);
void go16(int);
void gomixed(int);
void narrow(void);
void opened(int);
int main(void)
{
    for (int n = 0; n <= 4; n++) {
        go(n);
        printf("go %d %d\n", n, out);
    }
    for (int n = 0; n <= 3; n++) {
        go16(n);
        printf("go16 %d %d\n", n, out);
    }
    for (int n = 0; n <= 2; n++) {
        gomixed(n);
        printf("gomixed %d %d\n", n, out);
    }
    narrow();
    printf("narrow %d\n", out);
    for (int n = 0; n <= 1; n++) {
        opened(n);
        printf("opened %d %d\n", n, out);
    }
    return 0;
}
#endif
