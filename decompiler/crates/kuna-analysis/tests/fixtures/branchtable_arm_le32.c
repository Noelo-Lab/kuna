/* Fixture source for `branchtable_arm_le32`: ARM (A32) code compiled -fPIC and
   linked as a fixed-address (non-PIE) executable mapped at 0x10000.

   `add_dispatch` and `ldr_dispatch` each make the GOT base in r6 before a
   guarded A32 jump table, and every case reads a callback's GOT slot through
   r6 (`ldr r3,[pc,#..]; ldr r3,[r6,r3]`), calls `set_timeout` and forms a
   string PC-relatively. The two dispatches are the two table shapes GCC used
   to emit for a dense switch in ARM state:

       cmp   r5,#4                       cmp   r5,#4
       addls pc,pc,r5,lsl #2             ldrls pc,[pc,r5,lsl #2]
       b     default                     b     default
       b     case0 ... b case4           .word case0 ... .word case4

   GCC 14 compresses a switch table to bytes or halfwords in ARM state at -O1
   and above, so the dispatch is written with `asm goto`; the case bodies are
   compiled C.

   Built with GCC 14.2 for armhf musl (homebrew `musl-cross` 0.9.11); the
   SHA-256 below is that toolchain's output:

       arm-linux-musleabihf-gcc -O2 -marm -fPIC -no-pie -nostdlib -static \
           -ffreestanding -fno-asynchronous-unwind-tables -Wl,--build-id=none \
           -Wl,--sort-section=name -o branchtable_arm_le32 branchtable_arm_le32.c

   SHA-256 81d77bd8afe7e7278e95d8a0ebe3dc155a974c93e9178d4312a0566a7ddd42cb. */

const unsigned char table[0x11000] = { 1 };

void __attribute__((noinline)) log_msg(const char *s)
{
    __asm__ volatile("" : : "r"(s) : "memory");
}

void __attribute__((noinline)) set_timeout(int ms, void (*cb)(void *), void *arg)
{
    __asm__ volatile("" : : "r"(ms), "r"(cb), "r"(arg) : "memory");
}

void on_open(void *arg) { log_msg("open fired"); }
void on_close(void *arg) { log_msg("close fired"); }
void on_read(void *arg) { log_msg("read fired"); }
void on_write(void *arg) { log_msg("write fired"); }
void on_flush(void *arg) { log_msg("flush fired"); }
void on_reset(void *arg) { log_msg("reset fired"); }

void __attribute__((noinline)) add_dispatch(unsigned op, void *arg)
{
    log_msg("add dispatch");
    __asm__ goto("cmp %0, #4\n\t"
                 "addls pc, pc, %0, lsl #2\n\t"
                 "b %l[dflt]\n\t"
                 "b %l[c0]\n\t"
                 "b %l[c1]\n\t"
                 "b %l[c2]\n\t"
                 "b %l[c3]\n\t"
                 "b %l[c4]"
                 : : "r"(op) : "cc" : dflt, c0, c1, c2, c3, c4);
c0: set_timeout(10, on_open, arg); log_msg("add open"); goto done;
c1: set_timeout(11, on_close, arg); log_msg("add close"); goto done;
c2: set_timeout(12, on_read, arg); log_msg("add read"); goto done;
c3: set_timeout(13, on_write, arg); log_msg("add write"); goto done;
c4: set_timeout(14, on_flush, arg); log_msg("add flush"); goto done;
dflt: log_msg("add unknown");
done:
    log_msg("add dispatched");
}

void __attribute__((noinline)) ldr_dispatch(unsigned op, void *arg)
{
    log_msg("ldr dispatch");
    __asm__ goto("cmp %0, #4\n\t"
                 "ldrls pc, [pc, %0, lsl #2]\n\t"
                 "b %l[dflt]\n\t"
                 ".word %l[c0]\n\t"
                 ".word %l[c1]\n\t"
                 ".word %l[c2]\n\t"
                 ".word %l[c3]\n\t"
                 ".word %l[c4]"
                 : : "r"(op) : "cc" : dflt, c0, c1, c2, c3, c4);
c0: set_timeout(20, on_close, arg); log_msg("ldr close"); goto done;
c1: set_timeout(21, on_read, arg); log_msg("ldr read"); goto done;
c2: set_timeout(22, on_write, arg); log_msg("ldr write"); goto done;
c3: set_timeout(23, on_flush, arg); log_msg("ldr flush"); goto done;
c4: set_timeout(24, on_reset, arg); log_msg("ldr reset"); goto done;
dflt: log_msg("ldr unknown");
done:
    log_msg("ldr dispatched");
}

void _start(void)
{
    add_dispatch(table[0], (void *)table);
    ldr_dispatch(table[1], (void *)table);
    for (;;)
        ;
}
