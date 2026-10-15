/* kuna `strlenscan` fixture: gcc 9 expands strlen inline as
   `or rcx,-1; xor eax,eax; repnz scasb; not rcx` (ECX on i386), which kuna
   printed as a do/while over the string plus `~count` arithmetic.  The
   length must stay where the scan was: before a store, a call or a free that
   follows it, and once on each side of a branch that joins.  The two
   functions at the end are hand-written scans the option must leave alone:
   one looks for '/' rather than the terminator, the other returns the raw
   count register.  Built with Ubuntu 20.04's gcc 9.4 (gcc-multilib for i386):
     gcc -O1 -fno-pie -no-pie -o strlenscan_gcc9_O1_x86_64 strlenscan.c
     gcc -Os -fno-pie -no-pie -o strlenscan_gcc9_Os_x86_64 strlenscan.c
     gcc -m32 -O1 -fno-pie -no-pie -o strlenscan_gcc9_O1_i386 strlenscan.c  */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

__attribute__((noinline)) size_t name_len(const char *s)
{
    return strlen(s);
}

__attribute__((noinline)) int has_suffix(const char *path, const char *ext)
{
    size_t n = strlen(path), m = strlen(ext);
    return n >= m && strcmp(path + n - m, ext) == 0;
}

__attribute__((noinline)) char *dup_text(const char *s)
{
    size_t n = strlen(s) + 1;
    char *d = malloc(n);
    if (d)
        memcpy(d, s, n);
    return d;
}

__attribute__((noinline)) int last_char(const char *s)
{
    size_t n = strlen(s);
    return n ? (unsigned char)s[n - 1] : -1;
}

__attribute__((noinline)) long weighted_len(const char *a, const char *b)
{
    return (long)strlen(a) * 3 + (long)strlen(b);
}

__attribute__((noinline)) void wipe(char *s)
{
    s[0] = 0;
}

__attribute__((noinline)) unsigned long len_then_clear(char *s)
{
    unsigned long n = strlen(s);
    s[0] = 0;
    return n;
}

__attribute__((noinline)) unsigned long len_then_wipe(char *s)
{
    unsigned long n = strlen(s);
    wipe(s);
    return n;
}

__attribute__((noinline)) unsigned long len_then_free(const char *s)
{
    char *p = malloc(strlen(s) + 8);
    strcpy(p, s);
    strcat(p, "-x");
    unsigned long n = strlen(p);
    free(p);
    return n;
}

__attribute__((noinline)) void add_slash(char *path, const char *name)
{
    strcat(path, "/");
    strcat(path, name);
}

__attribute__((noinline)) unsigned long len_join(char *s, int clear, unsigned long *cleared, unsigned long *kept)
{
    unsigned long n;
    if (clear) {
        n = strlen(s);
        *cleared = n;
        s[0] = 0;
    } else {
        n = strlen(s);
        *kept = n;
    }
    return n + 100;
}

__attribute__((noinline)) long scan_slash(const char *s)
{
    long left = -1;
    const char *p = s;
    __asm__ volatile("repnz scasb" : "+c"(left), "+D"(p) : "a"('/') : "memory", "cc");
    return left;
}

__attribute__((noinline)) unsigned long raw_count(const char *s)
{
    unsigned long left = -1;
    const char *p = s;
    __asm__ volatile("repnz scasb" : "+c"(left), "+D"(p) : "a"(0) : "memory", "cc");
    return left;
}

#ifndef STRLENSCAN_NO_MAIN
int main(int argc, char **argv)
{
    const char *w = argc > 1 ? argv[1] : "notes.log";
    char *d = dup_text(w);
    printf("%zu %d %d %ld %ld %lu %s\n", name_len(w), has_suffix(w, ".log"), last_char(w),
           weighted_len(w, "ab/cd"), scan_slash("ab/cd"), raw_count(w), d);
    free(d);
    return 0;
}
#endif
