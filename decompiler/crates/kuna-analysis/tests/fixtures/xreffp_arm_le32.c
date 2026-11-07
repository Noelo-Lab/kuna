/* Fixture source for `xreffp_arm_le32`: A32 code compiled -fPIC and linked as
   a dynamic, fixed-address (non-PIE) executable mapped at 0x10000.

   Each import is called through a GNU ld PLT stub (`add ip,pc,#0,12;
   add ip,ip,#N,20; ldr pc,[ip,#M]!`). `table` sits between the stubs and the
   GOT, so the stub's `N<<12` is itself a mapped address.

   `clamp` and `pick` hold predicated non-branch instructions (`movlt`,
   `ldrne`), each of which lifts to a skip to the next instruction.
   `pick_t` is their Thumb-2 counterpart (`it ne; ldrne`). `next_branch` holds
   a real conditional branch whose label is the next instruction.

   Built with GCC 14.2 for armhf musl (homebrew `musl-cross` 0.9.11); the
   SHA-256 below is that toolchain's output:

       arm-linux-musleabihf-gcc -O2 -marm -march=armv7-a+fp -fPIC -no-pie \
           -fno-asynchronous-unwind-tables -Wl,--build-id=none \
           -o xreffp_arm_le32 xreffp_arm_le32.c

   SHA-256 33c2504a9e043837b83c7668ab36b4e602bff2bb198728af6980a96839ff8e92. */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

const unsigned char table[0x11000] = { 1 };

int __attribute__((noinline)) clamp(int v, int lo, int hi)
{
    if (v < lo)
        v = lo;
    if (v > hi)
        v = hi;
    return v;
}

int __attribute__((noinline)) pick(const int *p, int c)
{
    return c ? p[1] : p[0];
}

int __attribute__((noinline, target("thumb"))) pick_t(const int *p, int c)
{
    int v = p[0];
    __asm__("cmp %2, #0\n\tit ne\n\tldrne %0, [%1, #4]" : "+r"(v) : "r"(p), "r"(c) : "cc");
    return v;
}

int __attribute__((noinline)) next_branch(int v)
{
    __asm__ volatile("cmp %0, #0\n\tbne 1f\n1:" : : "r"(v) : "cc");
    return v + table[v & 0xff];
}

int main(int argc, char **argv)
{
    int n = clamp(argc, 1, 4);
    puts("hello");
    printf("%d %zu\n", pick(&n, argc > 2), strlen(argv[0]));
    if (next_branch(n) == pick_t(&n, argc > 3))
        abort();
    return 0;
}
