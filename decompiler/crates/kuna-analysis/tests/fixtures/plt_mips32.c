#include <stdio.h>

/* Dynamic MIPS fixture for PLT/stub import-name recovery (Increment 27).
   main calls puts("hello") and printf("%d\n", argc) so the decompiled body
   should name those libc imports rather than sub_<addr>. */
int main(int argc, char **argv) {
    puts("hello");
    printf("%d\n", argc);
    return 0;
}
