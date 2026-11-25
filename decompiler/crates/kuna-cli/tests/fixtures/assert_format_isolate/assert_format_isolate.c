/* clang -O1 -fno-stack-protector -fcf-protection=none -c assert_format_isolate.c -o t.o
   ld.lld -shared --build-id=none -o assert_format_isolate.elf t.o */
int g(int);
void h(int);
void k(int);
void p(int *);

int is_rwx(unsigned m) { return (m & 0777) == 0755; }

int scale(int m) { return m * 8 + 7; }

int reuse(int a) {
    int t = g(a);
    h(t);
    k(t);
    int u = t * 3 + 1;
    h(u);
    k(u);
    return 0;
}

int triple(int a) {
    int t = a;
    p(&t);
    h(t);
    a = a * 3;
    h(a);
    h(a + t);
    return a;
}
