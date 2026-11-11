/* Fixture source for `gotxref_arm_le32`: ARM (A32) code compiled -fPIC and
   linked as a fixed-address (non-PIE) executable mapped at 0x10000.

   `arm_timers` makes the GOT base once in a callee-saved register
   (`ldr r6,[pc,#..]; add r6,pc,r6`), keeps it across a call and a branch join,
   and reads two GOT slots (`ldr r3,[pc,#..]; ldr r3,[r6,r3]`) that hold the
   addresses of `on_expire` and `on_rearm`, which it passes to `set_timeout`.
   The linker fills those slots statically, so they carry no relocation. The
   static `on_idle` is formed PC-relatively instead (`add r1,pc,r1`).

   Its string literals are formed PC-relatively (`ldr r0,[pc,#..];
   add r0,pc,r0`). `table` sits between the code and the strings so that every
   such displacement is itself a mapped address.

   Built with GCC 14.2 for armhf musl (homebrew `musl-cross` 0.9.11); the
   SHA-256 below is that toolchain's output:

       arm-linux-musleabihf-gcc -O2 -marm -fPIC -no-pie -nostdlib -static \
           -ffreestanding -fno-asynchronous-unwind-tables -Wl,--build-id=none \
           -Wl,--sort-section=name -o gotxref_arm_le32 gotxref_arm_le32.c

   SHA-256 7fd54d44cf3d1e1464abd08a9d593f59d2ac94225a50dc2f2eff311b0aaecfb2. */

const unsigned char table[0x11000] = { 1 };

void __attribute__((noinline)) log_msg(const char *s)
{
    __asm__ volatile("" : : "r"(s) : "memory");
}

void __attribute__((noinline)) set_timeout(int ms, void (*cb)(void *), void *arg)
{
    __asm__ volatile("" : : "r"(ms), "r"(cb), "r"(arg) : "memory");
}

void on_expire(void *arg) { log_msg("expire fired"); }
void on_rearm(void *arg) { log_msg("rearm fired"); }
static void __attribute__((noinline)) on_idle(void *arg) { log_msg("idle fired"); }

void arm_timers(void *arg, int extend)
{
    log_msg("arming timers");
    if (extend)
        set_timeout(300, on_expire, arg);
    log_msg("rearm next");
    set_timeout(51, on_rearm, arg);
    set_timeout(7, on_idle, arg);
}

void _start(void)
{
    arm_timers((void *)table, table[0]);
    for (;;)
        ;
}
