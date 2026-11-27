typedef unsigned long long u64;

volatile u64 counter = 0x123456789abcdef0ULL;
volatile u64 zeroed;

u64 add(u64 a, u64 b) {
    return a + b;
}

double add_double(double a, double b) {
    return a + b;
}

u64 pointer_update(u64 *p, u64 a) {
    u64 result = *p + a;
    *p = result;
    return result;
}

__attribute__((noinline)) static u64 inner(u64 a) {
    u64 result = counter + a;
    counter = result;
    return result;
}

u64 call_inner(u64 a) {
    return inner(a) + 1;
}

u64 store_loop(u64 a, u64 n) {
    u64 result = counter;
#pragma clang loop unroll(disable)
    for (n &= 7; n != 0; --n) {
        result = result * 3 + a;
        counter = result;
    }
    zeroed = result;
    return result;
}

__asm__(
    ".type clobber_rbx,@function\n"
    ".globl clobber_rbx\n"
    "clobber_rbx:\n"
    "  mov $42, %ebx\n"
    "  lea 1(%rdi), %rax\n"
    "  ret\n"
    ".size clobber_rbx, .-clobber_rbx\n"
    ".type system_boundary,@function\n"
    ".globl system_boundary\n"
    "system_boundary:\n"
    "  movq $7, counter(%rip)\n"
    "  mov $39, %eax\n"
    "  syscall\n"
    "  movq $42, counter(%rip)\n"
    "  ret\n"
    ".size system_boundary, .-system_boundary\n"
    ".type unmapped_read,@function\n"
    ".globl unmapped_read\n"
    "unmapped_read:\n"
    "  mov 0, %rax\n"
    "  ret\n"
    ".size unmapped_read, .-unmapped_read\n"
    ".type caller_write,@function\n"
    ".globl caller_write\n"
    "caller_write:\n"
    "  mov %rdi, 8(%rsp)\n"
    "  mov %rdi, %rax\n"
    "  ret\n"
    ".size caller_write, .-caller_write\n"
);
