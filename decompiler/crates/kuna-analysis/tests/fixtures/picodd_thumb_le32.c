/* Fixture source for `picodd_thumb_le32`: Thumb-2 code compiled -fPIC with its
   `.rodata` linked into `.text` (picodd_thumb_le32.ld), as bare-metal images
   often are, mapped at 0x10000.

   Every string is formed PC-relatively (`ldr r0,[pc,#..]; add r0,pc`).
   "ab" and "ij" start at odd addresses inside the executable section; the
   `$d` mapping symbol over the strings says they are data, so their odd
   addresses are filed as they are, not one byte low. `set_local` forms the
   Thumb pointer of the static `local_cb` PC-relatively and `set_global` reads
   `f3`'s from its GOT slot; both carry bit 0 and name the function entry.

   Built with GCC 10.3.1 (Ubuntu `gcc-arm-none-eabi` 15:10.3-2021.07-4):

       arm-none-eabi-gcc -Os -mcpu=cortex-m4 -mthumb -fPIC -nostdlib \
           -T picodd_thumb_le32.ld -o picodd_thumb_le32 picodd_thumb_le32.c

   SHA-256 57b53ac8260b2286f2f5de5d4986f97a35c59c42bbd35a6ed647054af8048fb4. */

void (*volatile out)(const char *);
void (*volatile hook)(void);

__attribute__((noinline)) void say(const char *s) { out(s); }
void f1(void) { say("ab"); }
void f2(void) { say("cd"); }
void f3(void) { say("efgh"); }
void f4(void) { say("ij"); }
static __attribute__((noinline)) void local_cb(void) { say("ab"); }
void set_local(void) { hook = local_cb; }
void set_global(void) { hook = f3; }

void _start(void)
{
    f1();
    f2();
    f3();
    f4();
    set_local();
    set_global();
    for (;;)
        ;
}
