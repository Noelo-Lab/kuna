/*
 * Call-site prototype fixture (x86-64 shared object): two calls through one
 * variadic function pointer, the second a tail jump. Its target register is
 * read as a fifth argument unless that one call is given its own prototype.
 *
 * Build (clang and ld.lld 22.1.8; 21.1.8 reproduces every section but .comment):
 *        clang -O2 -fcf-protection=full -c callproto_x86_64.c -o callproto_x86_64.o
 *        ld.lld -shared -o callproto_x86_64 callproto_x86_64.o
 *
 * `report` is at 0x1370, the call at 0x1392, the tail jump at 0x13b0 and the
 * `logger` slot at 0x2458.
 */
extern void (*logger)(const char *, ...);
void report(int a, const char *s) { logger("%d", a); logger("%s %d %d", s, a, a + 1); }
