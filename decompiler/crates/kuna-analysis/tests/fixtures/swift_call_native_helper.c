/* C code linked into a Swift binary: r12 is an ordinary callee-saved
   register here, zeroed and read back across an unrelated call, never the
   error register of a throwing call. */
__attribute__((noinline)) long other_fn(long x) { return x * 3 + 7; }
__attribute__((naked)) long native_counter(long n) {
  __asm__(
    "pushq %r12\n"
    "pushq %rbx\n"
    "movq %rdi, %rbx\n"
    "xorl %r12d, %r12d\n"
    "movq %rbx, %rdi\n"
    "callq other_fn\n"
    "addq %r12, %rax\n"
    "addq $5, %rax\n"
    "popq %rbx\n"
    "popq %r12\n"
    "retq\n");
}
__attribute__((naked)) long native_arm(long n, long m) {
  __asm__(
    "pushq %r12\n"
    "xorl %r12d, %r12d\n"
    "callq other_fn\n"
    "leaq 1(%r12), %rax\n"
    "popq %r12\n"
    "retq\n");
}
