/* Fixture source for `predjump_arm_le32`: predicated computed jumps and an
   unguarded A32 jump table, linked as a fixed-address (non-PIE) executable
   mapped at 0x10000.

   `pool_skip` (A32) and `pool_skip_t` (Thumb, `it ne`) load a literal-pool
   offset, skip over a predicated `bxne r3` and form a string with
   `add rN,pc,r2` on the path where the jump did not run:

       ldr   r2,.Loff
       cmp   r3,#0
       bxne  r3
       add   r0,pc,r2

   `bail` exits through `bxne r0` or `bl abort`, and `bail_caller` calls it
   before its own `return`. `unguarded` is an `ldr pc,[pc,r0,lsl #2]` table
   with no range check, placed right after `pool_tail`'s literal-pool word, so
   nothing decoded ends at its entry.

   Built with GCC 14.2 for armhf musl (homebrew `musl-cross` 0.9.11); the
   SHA-256 below is that toolchain's output:

       arm-linux-musleabihf-gcc -O2 -marm -fPIC -no-pie -nostdlib -static \
           -ffreestanding -fno-asynchronous-unwind-tables -Wl,--build-id=none \
           -Wl,--sort-section=name -o predjump_arm_le32 predjump_arm_le32.c

   SHA-256 bae1f6cf535004360b6378a747f3d11217b6553d3fe1a5cd5b6cefaf8c2c1bfc. */

void __attribute__((noinline)) log_msg(const char *s)
{
    __asm__ volatile("" : : "r"(s) : "memory");
}

void __attribute__((noreturn, noinline)) abort(void)
{
    for (;;)
        __asm__ volatile("");
}

void bail(void (*cb)(void));
void pool_skip(int, int, int, void (*cb)(void));
void pool_skip_t(int, int, int, void (*cb)(void));
int unguarded(unsigned op);
int pool_tail(void);

__asm__(".syntax unified\n"
        ".arch armv7-a\n"
        ".section .rodata.pool, \"a\"\n"
        "pool_msg: .asciz \"skipped jump, arm\"\n"
        "pool_msg_t: .asciz \"skipped jump, thumb\"\n"
        ".text\n"

        ".arm\n"
        ".p2align 2\n"
        ".global pool_skip\n"
        ".type pool_skip, %function\n"
        "pool_skip:\n"
        "    ldr r2, .Lps_off\n"
        "    cmp r3, #0\n"
        "    bxne r3\n"
        ".Lps_pc:\n"
        "    add r0, pc, r2\n"
        "    b log_msg\n"
        ".Lps_off: .word pool_msg - (.Lps_pc + 8)\n"
        ".size pool_skip, . - pool_skip\n"

        ".thumb\n"
        ".p2align 2\n"
        ".global pool_skip_t\n"
        ".type pool_skip_t, %function\n"
        ".thumb_func\n"
        "pool_skip_t:\n"
        "    ldr r2, .Lpt_off\n"
        "    cmp r3, #0\n"
        "    it ne\n"
        "    bxne r3\n"
        ".Lpt_pc:\n"
        "    add r2, pc\n"
        "    mov r0, r2\n"
        "    bx lr\n"
        ".p2align 2\n"
        ".Lpt_off: .word pool_msg_t - (.Lpt_pc + 4)\n"
        ".size pool_skip_t, . - pool_skip_t\n"

        ".arm\n"
        ".p2align 2\n"
        ".global bail\n"
        ".type bail, %function\n"
        "bail:\n"
        "    cmp r0, #0\n"
        "    bxne r0\n"
        "    bl abort\n"
        ".size bail, . - bail\n"

        ".global pool_tail\n"
        ".type pool_tail, %function\n"
        "pool_tail:\n"
        "    ldr r0, .Ltail_word\n"
        "    bx lr\n"
        ".Ltail_word: .word 0x12345678\n"
        ".size pool_tail, . - pool_tail\n"

        ".global unguarded\n"
        ".type unguarded, %function\n"
        "unguarded:\n"
        "    ldr pc, [pc, r0, lsl #2]\n"
        "    b .Lu_default\n"
        "    .word .Lu0, .Lu1, .Lu2\n"
        ".Lu0: mov r0, #10\n"
        "    bx lr\n"
        ".Lu1: mov r0, #11\n"
        "    bx lr\n"
        ".Lu2: mov r0, #12\n"
        "    bx lr\n"
        ".Lu_default: mvn r0, #0\n"
        "    bx lr\n"
        ".size unguarded, . - unguarded\n");

int __attribute__((noinline)) bail_caller(void (*cb)(void), int x)
{
    bail(cb);
    return x * 7 + 3;
}

void _start(void)
{
    pool_skip(0, 0, 0, 0);
    pool_skip_t(0, 0, 0, 0);
    log_msg((const char *)(long)pool_tail());
    bail_caller(0, unguarded(1));
    for (;;)
        ;
}
