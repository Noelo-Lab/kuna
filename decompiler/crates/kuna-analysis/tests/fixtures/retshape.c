/* kuna `returndup auto` fixture: return shapes an optimizing compiler lays out
   the same whichever way the source spells them.
   Built with Ubuntu 22.04's gcc 11.4 (stack protector on by default):
     gcc -O2 -o retshape_gcc_O2_x86_64 retshape.c  */
#include <stdio.h>
#include <string.h>

struct buf {
    long base;
    long off;
    int readonly;
    unsigned refcount;
};

volatile int knob;

__attribute__((noipa)) int sane(const struct buf *b)
{
    return b->base == 0;
}

__attribute__((noipa)) int prepare(int fd, char *out)
{
    out[0] = (char)fd;
    return knob ? -1 : 0;
}

__attribute__((noipa)) int finish(char *out)
{
    return out[0] == 3 ? -1 : 0;
}

__attribute__((noipa)) int measure(const char *name, char *out, int len)
{
    return (int)strlen(name) + out[0] + len;
}

/* One guard with `||`: each test jumps to the shared `return 0`. */
__attribute__((noipa)) long mutable_ptr(struct buf *b)
{
    if (sane(b) != 0 || b->readonly || b->refcount > 1)
        return 0;
    return b->base + b->off;
}

/* Two guards whose tests each call something stay two guards. */
__attribute__((noipa)) void settle(int fd, char *out)
{
    if (!knob) {
        if (prepare(fd, out) == 0)
            return;
        if (finish(out) == 0)
            return;
        knob = 2;
    }
    out[2] = (char)fd;
    out[1] = out[0] + 1;
}

/* Each arm returns; the stack-protector check joins them before one exit. */
__attribute__((noipa)) int describe(int fd, const char *name, char *err, size_t errlen)
{
    char tmp[32];

    if (prepare(fd, tmp) == -1) {
        snprintf(err, errlen, "cannot prepare %s", name);
        return -1;
    }
    return measure(name, tmp, (int)errlen);
}

/* A one-armed update of the value returned stays one return. */
__attribute__((noipa)) unsigned long wrap_add(unsigned long res, unsigned long data)
{
    char tmp[16];

    res += data + prepare((int)data, tmp);
    if (res < data)
        res += knob;
    return res;
}

#ifndef RETSHAPE_NO_MAIN
int main(int argc, char **argv)
{
    char err[64], out[4] = { 0 };
    struct buf b = { (long)argv[0], 1, 0, 1 };

    settle(argc, out);
    return (mutable_ptr(&b) != 0) + out[1] + describe(argc, argv[0], err, sizeof err)
        + (int)wrap_add((unsigned long)argc, 7);
}
#endif
