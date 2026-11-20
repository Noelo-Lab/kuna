/* Callers of the throwing Swift function `risky`, written the way a
   compiler may when the error register is needed for something else: zero
   r12, call, spill r12 to the stack, reuse r12, reload the spilled error
   and test it. `spill_moved` changes rsp between the spill and the reload,
   so the reload is not the slot the spill wrote. */
__attribute__((naked)) long spill_test(long n) {
  __asm__(
    "pushq %r12\n"
    "subq $16, %rsp\n"
    "xorl %r12d, %r12d\n"
    "callq risky\n"
    "movq %r12, 8(%rsp)\n"
    "movl $7, %r12d\n"
    "movq 8(%rsp), %rcx\n"
    "testq %rcx, %rcx\n"
    "jne 1f\n"
    "addq $16, %rsp\n"
    "popq %r12\n"
    "retq\n"
    "1:\n"
    "movq $-1, %rax\n"
    "addq $16, %rsp\n"
    "popq %r12\n"
    "retq\n");
}
__attribute__((naked)) long spill_moved(long n) {
  __asm__(
    "pushq %r12\n"
    "subq $16, %rsp\n"
    "xorl %r12d, %r12d\n"
    "callq risky\n"
    "movq %r12, 8(%rsp)\n"
    "movl $7, %r12d\n"
    "subq $8, %rsp\n"
    "movq 8(%rsp), %rcx\n"
    "addq $8, %rsp\n"
    "testq %rcx, %rcx\n"
    "jne 1f\n"
    "addq $16, %rsp\n"
    "popq %r12\n"
    "retq\n"
    "1:\n"
    "movq $-1, %rax\n"
    "addq $16, %rsp\n"
    "popq %r12\n"
    "retq\n");
}
