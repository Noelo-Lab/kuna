/* A callee that pads an odd number of stack arguments with a push of a
   register it never set (gcc -Os, and gcc 9 at -O2), and a caller that holds
   an unrelated value in that register at the call. */
#include <stdio.h>

struct pic { unsigned char w, h; };

static unsigned int cells[64];

__attribute__((noinline, optimize("Os"))) void show(const struct pic *p, const unsigned int *c)
{
    char buf[32];
    for (int y = 0; y < p->h; y++) {
        for (int x = 0; x < p->w; x++) {
            unsigned int v = c[y * p->w + x];
            snprintf(buf, sizeof buf, "%c%c%c%c", v & 0xff, (v >> 8) & 0xff,
                     (v >> 16) & 0xff, v >> 24);
            fputs(buf, stdout);
        }
        putchar('\n');
    }
}

__attribute__((noinline)) int run(unsigned char w, unsigned char h)
{
    struct pic p = { w, h };
    int n = w * h;
    for (int i = 0; i < n; i++)
        if (cells[i] & 0xe0)
            return -1;
    show(&p, cells);
    return n;
}

int main(int argc, char **argv)
{
    (void)argv;
    return run(argc, 2) < 0;
}
