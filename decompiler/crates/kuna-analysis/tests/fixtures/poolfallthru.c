/* `probe` ends in a call that does not return, so the compiler emits no
   epilogue and its PC-relative literal pool follows the call immediately. The
   first pool words decode as instructions; `0x0002cbb2` decodes as nothing.
   CFLAGS='-O2 -march=armv5te -shared -fPIC -nostdlib -fno-builtin
     -Wl,--build-id=none -Wl,-z,max-page-size=0x1000 -Wl,-z,noseparate-code'
   arm-linux-gnueabi-gcc $CFLAGS poolfallthru.c -o poolfallthru_arm_le32
   arm-linux-gnueabi-gcc $CFLAGS -mbig-endian -Wl,-EB poolfallthru.c -o poolfallthru_arm_be32 */
extern void die(void) __attribute__((noreturn));
extern void use(unsigned);
unsigned probe(unsigned n) {
    unsigned a = 0;
    if (n & 1u) a += 0x0007943cu;
    if (n & 2u) a += 0x00021ba1u;
    if (n & 4u) a += 0x0002cea6u;
    if (n & 8u) a += 0x0005f9fbu;
    if (n & 16u) a += 0x0002cbb2u;
    use(a);
    die();
}
