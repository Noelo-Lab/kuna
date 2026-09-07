/* Fixture source for `armlibcmain` — a non-PIE ARM32 ELF whose crt1 `_start`
   hands `main` to `__libc_start_main` in a PC-relative literal pool word (the
   classic glibc `crt1.o` shape).  `_start` is written out here because a modern
   `-no-pie` link supplies the GOT-indexed variant instead; that variant is the
   `armlibcmain_got_le32` fixture.

   Built in the dev container (docs/devcontainer.md):

       arm-linux-gnueabi-gcc -O1 -no-pie -fno-pie -nostartfiles \
           -o armlibcmain_le32 armlibcmain_le32.c
       arm-linux-gnueabi-strip armlibcmain_le32

   `main` is reachable only through the pool word at `.Lmain`, and the prompt it
   prints only through a second pool word inside `main`. */
#include <stdio.h>

__asm__(
    "	.text\n"
    "	.globl _start\n"
    "	.type _start, %function\n"
    "_start:\n"
    "	mov	r11, #0\n"
    "	mov	lr, #0\n"
    "	ldr	r1, [sp], #4\n"
    "	mov	r2, sp\n"
    "	str	r2, [sp, #-4]!\n"
    "	str	r0, [sp, #-4]!\n"
    "	ldr	ip, .Lfini\n"
    "	ldr	r0, .Lmain\n"
    "	ldr	r3, .Linit\n"
    "	str	ip, [sp, #-4]!\n"
    "	bl	__libc_start_main\n"
    "	bl	abort\n"
    ".Lfini:	.word	fini_stub\n"
    ".Lmain:	.word	main\n"
    ".Linit:	.word	init_stub\n"
    "	.size _start, .-_start\n");

void init_stub(void) {}
void fini_stub(void) {}

int main(int argc, char **argv)
{
    char buf[32];

    printf("kuna armlibcmain prompt");
    if (fgets(buf, sizeof buf, stdin) && buf[0] == 'k')
        return 0;
    return 1;
}
