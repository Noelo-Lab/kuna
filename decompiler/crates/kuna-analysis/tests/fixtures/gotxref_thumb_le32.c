/* Fixture source for `gotxref_thumb_le32`: Thumb-2 code compiled -fPIC and
   linked as a fixed-address (non-PIE) executable mapped at 0x10000, the
   Thumb counterpart of `gotxref_arm_le32`.

   Thumb lifts its literal loads (`ldr rX,[pc,#..]`, 16-bit, and `ldr.w`)
   as copies out of the image rather than as loads. `arm_timers` makes the
   GOT base once in r6 (`ldr r6,[pc,#..]; add r6,pc`), keeps it across a call
   and a branch join, and reads the slots of `on_expire` and `on_rearm`
   through it (`ldr r3,[pc,#..]; ldr r3,[r6,r3]`). `pick_timer` dispatches
   through a `tbb` table, and each case reads its callback's slot through
   the base the dispatch holds. `far_timer` is long enough that its literal
   loads are 32-bit `ldr.w`.

   `table` sits between the code and the strings so that every near string
   displacement is itself a mapped address.

   Built with GCC 14.2 for armhf musl (homebrew `musl-cross` 0.9.11); the
   SHA-256 below is that toolchain's output:

       arm-linux-musleabihf-gcc -O2 -mthumb -march=armv7-a+fp -fPIC -no-pie \
           -nostdlib -static -ffreestanding -fno-asynchronous-unwind-tables \
           -Wl,--build-id=none -Wl,--sort-section=name \
           -o gotxref_thumb_le32 gotxref_thumb_le32.c

   SHA-256 1c40aee5537ecf6b073a3278ca0bf5ba27eba51afd4a7188b95b3fbd5bf56047. */

const unsigned char table[0x11000] = { 1 };

void __attribute__((noinline)) log_msg(const char *s)
{
    __asm__ volatile("" : : "r"(s) : "memory");
}

void __attribute__((noinline)) set_timeout(int ms, void (*cb)(void *), void *arg)
{
    __asm__ volatile("" : : "r"(ms), "r"(cb), "r"(arg) : "memory");
}

void __attribute__((noinline)) set_alarm(void (*cb)(void *), int ms)
{
    __asm__ volatile("" : : "r"(cb), "r"(ms) : "memory");
}

void __attribute__((noinline)) set_watch(void *arg, void (*cb)(void *))
{
    __asm__ volatile("" : : "r"(arg), "r"(cb) : "memory");
}

void on_expire(void *arg) { log_msg("expire fired"); }
void on_rearm(void *arg) { log_msg("rearm fired"); }
void on_alarm(void *arg) { log_msg("alarm fired"); }
void on_snooze(void *arg) { log_msg("snooze fired"); }
void on_stop(void *arg) { log_msg("stop fired"); }
void on_far(void *arg) { log_msg("far fired"); }
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

int pick_timer(void *arg, int which)
{
    log_msg("picking");
    switch (which) {
    case 0: set_timeout(10, on_alarm, arg); return 3;
    case 1: set_alarm(on_snooze, 20); return 5;
    case 2: set_watch(arg, on_stop); return 8;
    case 3: log_msg("resetting"); set_alarm(on_expire, 7); return 13;
    case 5: set_watch(on_rearm, on_rearm); return 21;
    default: return 0;
    }
}

void far_timer(void *arg)
{
    log_msg("far away");
    __asm__ volatile(".rept 300\n\tnop.w\n\t.endr" ::: "memory");
    set_timeout(9, on_far, arg);
}

void _start(void)
{
    arm_timers((void *)table, table[0]);
    pick_timer((void *)table, table[1]);
    far_timer((void *)table);
    for (;;)
        ;
}
