/* The import plt_import_x86_64 calls, so its call goes through the PLT the
 * way a call into libc does. Only linked against; not committed as a binary.
 * Build: zig cc -target x86_64-linux-musl -O0 -g0 -nostdlib -shared -fPIC -o libplt_import_stub.so plt_import_x86_64_stub.c */
char *strfry(char *s) { return s; }
