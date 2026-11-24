/* `undecodable` holds a byte x86-64 does not decode (0x06, `push es`, which
 * 64-bit mode removed), so its C calls halt_baddata(). `nan_bits` and
 * `inf_bits` return a NaN and an infinity (kuna reads both from `.rodata`).
 * Build: zig cc -target x86_64-linux-musl -O0 -g0 -fno-sanitize=undefined -nostdlib -static -o pseudo_names_x86_64 pseudo_names_x86_64.c
 *        llvm-objcopy --strip-debug pseudo_names_x86_64 */
__attribute__((noinline)) int undecodable(int x) {
    if (x > 3)
        __asm__ volatile(".byte 0x06");
    return x + 1;
}
__attribute__((noinline)) double nan_bits(void) { return __builtin_nan(""); }
__attribute__((noinline)) double inf_bits(void) { return __builtin_inf(); }
void _start(void) { undecodable(1); nan_bits(); inf_bits(); }
