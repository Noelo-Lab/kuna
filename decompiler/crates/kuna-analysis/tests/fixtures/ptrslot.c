/* A table of function pointers whose first entry's low bytes are printable.
   Built with PTRSLOT_HARNESS it supplies everything but call, pass and keep,
   and a main that prints what each of them does. */
#define NI __attribute__((noinline))
typedef void (*fp)(void);
int hit;
NI void r1(void) { hit = 11; }
NI void r2(void) { hit = 22; }
NI void r3(void) { hit = 33; }
const fp tbl[4] = {r1, r2, r3, 0};
NI void pick(const fp *t, unsigned i) { t[i](); }
#ifndef PTRSLOT_HARNESS
NI void call(unsigned i) { tbl[i](); }
NI void pass(unsigned i) { pick(tbl, i); }
NI void keep(const fp **out) { *out = tbl; }
int main(int c, char **v) { const fp *t; keep(&t); call(c); pass(c); return hit + (t == tbl); }
#else
#include <stdio.h>
void call(unsigned);
void pass(unsigned);
void keep(const fp **);
int main(void)
{
    for (unsigned i = 0; i < 3; i++) {
        hit = 0;
        call(i);
        printf("call %u %d\n", i, hit);
        hit = 0;
        pass(i);
        printf("pass %u %d\n", i, hit);
    }
    const fp *t = 0;
    keep(&t);
    printf("keep %d\n", t == tbl);
    return 0;
}
#endif
