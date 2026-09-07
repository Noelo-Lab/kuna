/* Fixture source for `armlibcmain`, second `_start` shape — the crt1 a modern
   `-no-pie` ARM link supplies, which reaches `main` through a GOT slot the
   linker filled in statically (no `R_ARM_RELATIVE`, so the PIE oracle's
   relocation cross-check finds nothing).  The classic literal-pool shape is the
   `armlibcmain_le32` fixture.

   Built in the dev container (docs/devcontainer.md):

       arm-linux-gnueabi-gcc -O1 -no-pie -fno-pie \
           -o armlibcmain_got_le32 armlibcmain_got_le32.c
       arm-linux-gnueabi-strip armlibcmain_got_le32 */
#include <stdio.h>

int main(int argc, char **argv)
{
    char buf[32];

    printf("kuna armlibcmain got prompt");
    if (fgets(buf, sizeof buf, stdin) && buf[0] == 'k')
        return 0;
    return 1;
}
