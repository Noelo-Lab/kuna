/* s390x ELF ABI fixture for tests/stages/kuna-s390x.xml.
 *
 *   zig cc -target s390x-linux-gnu -mcpu=z196 -O2 -g0 -fomit-frame-pointer \
 *          -shared -nostdlib s390x_abi.c -o s390x_abi.elf
 */
extern unsigned long ext(unsigned long);
extern int ext2(int, int);

struct rec { int a; long b; char c[20]; unsigned short d; };

unsigned long twice(unsigned long a) { return ext(ext(a)); }
long scaled(long a, long b) { return ext(a * 3 + b) + 1; }
long larger(long a, long b) { return a > b ? a : b; }
unsigned below(unsigned a, unsigned b) { return a < b ? 7 : 9; }
int total(const int *p, int n) { int s = 0; for (int i = 0; i < n; i++) s += p[i]; return s; }
long fields(struct rec *r) { return r->a + r->b + r->c[3] + r->d; }
long seven(long a, long b, long c, long d, long e, long f, long g) { return a + b + c + d + e + f + g; }
unsigned long quotrem(unsigned long a, unsigned long b) { return a / b + a % b; }
int pick(int x) {
    switch (x) {
    case 0: return ext2(x, 10);
    case 1: return ext2(x, 22) + 1;
    case 2: return ext2(31, x);
    case 3: return 47;
    case 4: return ext2(x, x) * 2;
    case 5: return 66;
    default: return -1;
    }
}
int frame(int n) {
    int buf[16];
    for (int i = 0; i < 16; i++) buf[i] = ext2(i, n);
    return ext2(buf[n & 15], buf[3]);
}
double blend(double a, double b) { return a * b + 1.5; }
