/* `choose` keeps the address of one of two stack subobjects in a callee-saved
   register; the first decompile lays out a frame variable at each, the locals
   a `type` directive's replay must keep (README.md).
   arm-linux-gnueabihf-gcc -O2 -shared -fPIC -nostdlib -fno-builtin -fno-stack-protector
     -Wl,--build-id=none -Wl,-z,max-page-size=0x1000 -Wl,-z,noseparate-code
     retypestackchoose.c -o retypestackchoose_arm_le32 */
_Static_assert(sizeof(unsigned) == 4, "four-byte fixture fields");
extern void consume(void *);
extern int pick(unsigned);
unsigned choose(unsigned n) {
    unsigned frame[12] = {0};
    register unsigned *cursor __asm__("r4") = pick(n) ? &frame[1] : &frame[6];
    __asm__ volatile("" : "+r"(cursor));
    consume(cursor);
    consume(cursor);
    consume(cursor);
    consume(frame);
    return frame[0] + frame[11];
}
