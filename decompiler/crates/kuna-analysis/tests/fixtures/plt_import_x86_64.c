/* main calls strfry, an import no prototype table lists, through its PLT stub,
 * which jumps through strfry's GOT slot. The program has its own _start, so
 * strfry is its only import.
 * Build: zig cc -target x86_64-linux-musl -O0 -g0 -nostdlib -shared -fPIC -o libplt_import_stub.so plt_import_x86_64_stub.c
 *        zig cc -target x86_64-linux-musl -O0 -g0 -fno-sanitize=undefined -nostdlib -o plt_import_x86_64 plt_import_x86_64.c -L. -lplt_import_stub
 *        llvm-objcopy --strip-debug plt_import_x86_64 */
extern char *strfry(char *);

int main(void) {
    char s[] = "abc";
    return *strfry(s);
}

void _start(void) {
    long status = main();
    __asm__ volatile("syscall" : : "a"(60L), "D"(status));
    for (;;) {
    }
}
