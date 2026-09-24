/* castarith enum round-trip fixture: a pointer plus whole elements of an enum
 * whose width in the binary is not C's sizeof of the plain `enum` kuna prints.
 * Built with -std=gnu11 -g at gcc -O0 and gcc -O2.  The round trip compiles
 * kuna's own enum typedefs, the prelude with KUNA_RT defined, kuna's printing
 * of every function between the markers, and the original main, and compares
 * its output with this binary's.  The callees read the width the binary
 * passes, as the binary's own callees do. */
#include <stdio.h>

/* prelude */
#define KEEP __attribute__((noipa))
#ifndef KUNA_RT
typedef enum __attribute__((packed)) color { RED = 1, GREEN = 200 } color;
typedef enum __attribute__((packed)) mark { M0 = 1, M1 = 0x1234 } mark;
typedef enum level { L0 = 0, L1 = 7 } level;
#endif
KEEP void use_color(color c) { printf("color %d\n", (int)(unsigned char)c); }
KEEP void use_mark(mark m) { printf("mark %d\n", (int)(unsigned short)m); }
KEEP void use_level(level l) { printf("level %d\n", (int)l); }
KEEP void set_color(color *c) { printf("set %d\n", (int)*(unsigned char *)c); *(unsigned char *)c = GREEN; }
/* tested */
KEEP void rd_color(void *p) { use_color(*(color *)((char *)p + 3)); }
KEEP void rd_mark(void *p) { use_mark(*(mark *)((char *)p + 6)); }
KEEP void rd_level(void *p) { use_level(*(level *)((char *)p + 8)); }
KEEP void pass_color(void *p) { set_color((color *)((char *)p + 5)); }
/* main */
int main(void)
{
    static unsigned char b[64];
    for (int i = 0; i < 64; i++)
        b[i] = (unsigned char)(i * 7 + 1);
    b[8] = 7, b[9] = b[10] = b[11] = 0;
    rd_color(b);
    rd_mark(b);
    rd_level(b);
    pass_color(b);
    printf("%d %d\n", b[5], b[20]);
    return 0;
}
