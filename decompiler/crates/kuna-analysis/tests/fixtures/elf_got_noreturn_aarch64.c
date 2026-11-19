/* -fno-plt AArch64: die() ends in exit() through its .got slot (adrp/ldr/blr),
   check() calls die(). Linked with lld against a stub libc that defines
   exit and puts as functions, so .dynsym types them STT_FUNC. */
extern void exit(int) __attribute__((noreturn));
extern int puts(const char *);
__attribute__((noinline)) void die(const char *m) { puts(m); exit(2); }
__attribute__((noinline)) int check(int x) { if (x > 9) die("big"); return x * 3; }
int main(int c, char **v) { return check(c); }
