/* `probe` keeps the address of a stack subobject in a callee-saved register,
   the local a `type` directive retypes to a pointer to a record (README.md).
   CFLAGS='-O2 -shared -fPIC -nostdlib -fno-builtin -fno-stack-protector
     -Wl,--build-id=none -Wl,-z,max-page-size=0x1000 -Wl,-z,noseparate-code'
   arm-linux-gnueabi-gcc $CFLAGS '-DSAVED="r4"' retypestackptr.c -o retypestackptr_arm_le32
   powerpc-linux-gnu-gcc $CFLAGS '-DSAVED="r29"' retypestackptr.c -o retypestackptr_ppc32 */
_Static_assert(sizeof(unsigned) == 4, "four-byte fixture fields");
extern void consume(void *);
unsigned probe(unsigned n) {
    unsigned frame[6] = {0};
    register void *cursor __asm__(SAVED) = &frame[1];
    __asm__ volatile("" : "+r"(cursor));
    consume(cursor);
    consume(cursor);
    consume(cursor);
    consume(cursor);
    consume(frame);
    return n + frame[0] + frame[5];
}
