/* Original import-flow witnesses. No target libc is needed to build these DSOs. */
typedef __SIZE_TYPE__ Size;
extern Size strlen(const char *);
extern void *malloc(Size);
extern void *memcpy(void *, const void *, Size);
extern char *strcpy(char *, const char *);
extern int external_transform(char *);

#ifndef EXPORTED
#define LOCAL static
#else
#define LOCAL
#endif

LOCAL __attribute__((noinline)) char *duplicate(const char *s) {
    Size n = strlen(s);
    char *r = malloc(2 * n + 1);
    memcpy(r, s, n);
    memcpy(r + n, s, n);
    r[2 * n] = 0;
    return r;
}
char *twice(const char *s) { return duplicate(s); }
int first(const char *s) { char b[64]; strcpy(b, s); return b[0]; }
int unknown_then_store(char *p) { int n = external_transform(p); p[0] = 90; return n + 7; }
#ifdef __SSP_STRONG__
extern void __stack_chk_fail(void) __attribute__((noreturn));
void fail(void) { __stack_chk_fail(); }
#endif
