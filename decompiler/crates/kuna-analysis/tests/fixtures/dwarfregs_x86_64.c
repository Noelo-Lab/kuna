
#include <string.h>
struct pair { long a; long b; };
static inline int mix(int x, int y) { int t = x * 31 + y; int u = t ^ (t >> 7); return u + 1; }
__attribute__((noinline)) int hash(const char *s, int n) {
    int h = 17;
    for (int i = 0; i < n; i++) {
        int c = (unsigned char)s[i];
        h = mix(h, c);
    }
    return h;
}
__attribute__((noinline)) long sum_pair(struct pair p, long k) {
    struct pair q = p;
    q.a += k;
    q.b -= k;
    return q.a * q.b;
}
int main(int argc, char **argv) { struct pair p = { argc, 3 }; return hash(argv[0], argc) + (int)sum_pair(p, argc); }
