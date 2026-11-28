typedef unsigned long long u64;

volatile u64 counter = 0x123456789abcdef0ULL;
volatile u64 zeroed;

u64 add(u64 a, u64 b) {
    return a + b;
}

u64 extend_signed(int a) {
    return (u64)(long long)a;
}

u64 extend_unsigned(unsigned a) {
    return (u64)a;
}

u64 bit_ops(u64 a, u64 b) {
    return ((a >> 13) + (b << 9)) ^ b;
}

u64 rotate(u64 a) {
    return (a >> 13) | (a << 51);
}

u64 select_cmov(u64 a, u64 yes, u64 no) {
    __asm__("testq %2, %2; cmovneq %1, %0" : "+r"(no) : "r"(yes), "r"(a) : "cc");
    return no;
}

u64 select_branch(u64 flag, u64 yes, u64 no) {
    return flag ? yes : no;
}

u64 store_if(u64 *p, u64 flag) {
    if (flag & 1)
        *p = 17;
    return *p;
}

__attribute__((noinline, optnone)) u64 frame_add(u64 a, u64 b) {
    return a + b;
}

u64 count_down(u64 n) {
    __asm__("1: decq %0; jne 1b" : "+r"(n) : : "cc");
    return n;
}

u64 add_with_loop(u64 a, u64 b, u64 n) {
    u64 result = a + b;
    __asm__("1: addq $3, %0; decq %1; jne 1b" : "+&r"(result), "+r"(n) : : "cc");
    return result;
}

u64 loop_with_duplicate_paths(u64 value, u64 count, u64 flag) {
    __asm__("1: testq %2, %2; jz 2f; addq $3, %0; jmp 3f; "
            "2: leaq 3(%0), %0; 3: decq %1; jne 1b"
            : "+&r"(value), "+&r"(count) : "r"(flag) : "cc");
    return value;
}

u64 cannot_rank_all_inputs(u64 n) {
    __asm__("1: addq $2, %0; jne 1b" : "+r"(n) : : "cc");
    return n;
}

u64 dispatch(u64 selector, u64 x) {
    switch (selector) {
    case 0: return x + 15;
    case 1: return x * 17;
    case 2: return x ^ 7923;
    case 3: return x << 4;
    case 4: return x - 16;
    case 5: return x | 0x13;
    default: return x ^ 0x7fff;
    }
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
