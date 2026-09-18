/* formatstring static: two call sites the load-time window must not answer.
 *
 * report: the format reaching printf is chosen on a jump table, and the case
 * bodies jump back to the join (0x11cc-style) that the default path falls
 * into, so the window before the call sees only the default path's string.
 *
 * show: an alloca frame; the pushed return address becomes one more argument
 * of any closed prototype the drive gives the call.
 *
 * gcc -O2 -o fmtjoin_x86_64 fmtjoin_x86_64.c  (gcc 11.4, x86-64)
 */
#include <alloca.h>
#include <stdio.h>
#include <string.h>

void g(int);

__attribute__((noinline)) void report(unsigned k, const char *a, long b) {
    const char *fmt;
    if (__builtin_expect(k > 7, 1)) {
        fmt = "default %ld\n";
    } else switch (k) {
    case 0: g(0); fmt = "zero %s %ld\n"; break;
    case 1: g(1); fmt = "one %s %ld\n"; break;
    case 2: g(2); fmt = "two %s %ld\n"; break;
    case 3: g(3); fmt = "three %s %ld\n"; break;
    case 4: g(4); fmt = "four %s %ld\n"; break;
    case 5: g(5); fmt = "five %s %ld\n"; break;
    case 6: g(6); fmt = "six %s %ld\n"; break;
    case 7: g(7); fmt = "seven %s %ld\n"; break;
    default: __builtin_unreachable();
    }
    printf(fmt, a, b);
    g(99);
}

__attribute__((noinline)) void show(const char *s, size_t n) {
    char *buf = alloca(n + 1);
    memcpy(buf, s, n);
    buf[n] = 0;
    printf("got %s\n", buf);
    g((int)n);
}

__attribute__((noinline)) void g(int x) { if (x == 1234) puts("g"); }

int main(int argc, char **argv) {
    report(argc, argv[0], argc * 3L);
    show(argv[0], (size_t)argc);
    return 0;
}
