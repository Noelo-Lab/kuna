/* _start hands main to __libc_start_main through its GOT slot, as glibc's
 * crt1.o does, and main adds 1 to what got_value returns, also through its
 * slot: two imports no prototype table lists, which -fno-plt calls with
 * `call *slot(%rip)` rather than through a PLT stub.
 * Build: zig cc -target x86_64-linux-musl -O0 -g0 -nostdlib -shared -fPIC -o libgot_import_stub.so got_import_x86_64_stub.c
 *        zig cc -target x86_64-linux-musl -O0 -g0 -fno-plt -fno-sanitize=undefined -nostdlib -o got_import_x86_64 got_import_x86_64.c -L. -lgot_import_stub
 *        llvm-objcopy --strip-debug got_import_x86_64 */
extern void __libc_start_main(int (*)(void));
extern int got_value(void);

int main(void) {
    return got_value() + 1;
}

void _start(void) {
    __libc_start_main(main);
    for (;;) {
    }
}
