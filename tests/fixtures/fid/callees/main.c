int wrap_a(int), wrap_b(int), bump(int);

void _start(void) {
    int r = wrap_a(1) + wrap_b(2) + bump(3);
    __asm__ volatile("mov %0, %%edi\n\tmov $60, %%eax\n\tsyscall" : : "r"(r) : "rdi", "rax");
    __builtin_unreachable();
}
