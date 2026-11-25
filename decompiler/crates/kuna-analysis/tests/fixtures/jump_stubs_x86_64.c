/* Three x86-64 jump stubs that forward through a function pointer, as PLT stubs
 * forward through a GOT slot: a CET .plt.sec stub (endbr64; jmp), GNU ld's form
 * of it (endbr64; bnd jmp) and a plain one. main passes each two arguments.
 * Build: zig cc -target x86_64-linux-musl -O0 -g0 -fno-sanitize=undefined -nostdlib -static -o jump_stubs_x86_64 jump_stubs_x86_64.c
 *        llvm-objcopy --strip-debug jump_stubs_x86_64 */
static int add(int a, int b) { return a + b; }

int (*slot)(int, int) = add;

__asm__(".text\n"
        ".globl ibt_stub\n.type ibt_stub,@function\nibt_stub:\n"
        "  endbr64\n  jmp *slot(%rip)\n.size ibt_stub,.-ibt_stub\n"
        ".globl bnd_stub\n.type bnd_stub,@function\nbnd_stub:\n"
        "  endbr64\n  .byte 0xf2\n  jmp *slot(%rip)\n.size bnd_stub,.-bnd_stub\n"
        ".globl plain_stub\n.type plain_stub,@function\nplain_stub:\n"
        "  jmp *slot(%rip)\n.size plain_stub,.-plain_stub\n");

int ibt_stub(int, int);
int bnd_stub(int, int);
int plain_stub(int, int);

int main(void) {
    return ibt_stub(1, 2) + bnd_stub(3, 4) + plain_stub(5, 6);
}

void _start(void) {
    long status = main();
    __asm__ volatile("syscall" : : "a"(60L), "D"(status));
    for (;;) {
    }
}
