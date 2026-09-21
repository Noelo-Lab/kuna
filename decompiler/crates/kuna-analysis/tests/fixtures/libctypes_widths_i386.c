/* Fixture for the width gate of `option libctypes`
 * (tests/stages/kuna-libctypes.xml).
 *
 * Built (no 32-bit libc needed; the stub stands in for it):
 *   gcc -m32 -O0 -fno-pic -shared -nostdlib -Wl,-soname,libc.so.6 \
 *       -o libc.so.6 libctypes_widths_i386_stub.c
 *   gcc -m32 -O0 -fno-pie -no-pie -ffreestanding -fno-stack-protector -nostdlib \
 *       -o libctypes_widths_i386 libctypes_widths_i386.c ./libc.so.6 \
 *       -Wl,--hash-style=both
 *   strip libctypes_widths_i386
 *
 * The named slots carry glibc's x86-64 widths, and a named pointer handed a
 * frame object's address grows that object to the declared width. On i386 the
 * real objects are smaller (`struct timespec` is two 4-byte longs, 8 bytes, not
 * 16), so the grown object swallowed the locals beside it: `res_sum` printed
 * `v1._12_4_ = a0 + 2` for its own `y`, and `ob_sum`'s `int k[12]` became a
 * field of the obstack. The structs are declared by hand with the i386 layouts,
 * which is all the image needs; only their sizes matter.
 */
struct timespec { long tv_sec; long tv_nsec; };
struct statfs { int f_type, f_bsize; unsigned long f_blocks, f_bfree, f_bavail, f_files, f_ffree;
                int f_fsid[2]; int f_namelen, f_frsize, f_flags, f_spare[4]; };
struct obstack { long w[11]; };

extern int clock_getres(int, struct timespec *);
extern int statfs(const char *, struct statfs *);
extern int _obstack_begin(struct obstack *, int, int, void *(*)(long), void (*)(void *));

__attribute__((noinline)) int res_sum(int a0)
{
    int x = a0 + 1, y = a0 + 2, z = a0 + 3;
    struct timespec ts;
    clock_getres(a0, &ts);
    return (int)ts.tv_nsec + x + y + z;
}

__attribute__((noinline)) int fs_sum(const char *p, int a1)
{
    int k0 = a1 + 1, k1 = a1 * 3, k2 = a1 * 5, k3 = a1 * 7, k4 = a1 * 11, k5 = a1 * 13;
    struct statfs fs;
    if (statfs(p, &fs))
        return 0;
    return fs.f_bsize + k0 + k1 + k2 + k3 + k4 + k5;
}

__attribute__((noinline)) int ob_sum(int a0)
{
    int k[12];
    struct obstack ob;
    for (int i = 0; i < 12; i++)
        k[i] = a0 * i;
    _obstack_begin(&ob, 0, 0, 0, 0);
    return k[3] + k[7] + (int)ob.w[0];
}

void _start(void)
{
    int r = res_sum(1) + fs_sum("/", 2) + ob_sum(3);
    __asm__ volatile("int $0x80" : : "a"(1), "b"(r));
    for (;;) {}
}
