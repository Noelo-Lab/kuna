/* Computed values must not move a global write across a load, a call, or
 * onto paths where the machine does not write. */
#include <stdio.h>

#if defined(GLOBALORDER_SAMPLE)
extern int gi, seen;
void sample(void) { seen = gi; }
#else
int gi, other, seen;
volatile int sink;
volatile unsigned char flag_a, flag_b;
volatile unsigned long uid_a, uid_b;
volatile long scaled;
void sample(void);

#if !defined(GLOBALORDER_HARNESS)
__attribute__((noinline)) int computed(int *p, int b) {
    int t = gi * 2 + b;
    int x = *p;
    gi = t;
    return x + t * 100;
}
__attribute__((noinline)) int called(int b) {
    int t = gi * 2 + b;
    sample();
    gi = t;
    return seen + t * 100;
}
__attribute__((noinline)) int conditional(int *p, int b, int c) {
    int t = gi * 2 + b;
    if (c) gi = t;
    return *p + t * 100;
}
__attribute__((noinline)) int before(int *p, int b) {
    int t = gi * 2 + b;
    gi = t;
    int x = *p;
    return x + t * 100;
}
__attribute__((noinline)) int repeated(int *p, int b) {
    int t = gi * 2 + b;
    gi = t;
    int x = *p;
    gi = t + 7;
    return x + t * 100;
}
__attribute__((noinline)) int nonalias(int b) {
    int t = gi * 2 + b;
    int x = other;
    gi = t;
    return x + t * 100;
}
__attribute__((noinline)) int pointer_store(int *p, int b) {
    int t = gi * 2 + b;
    *p = 77;
    gi = t;
    return t;
}
__attribute__((noinline)) int earlier_branch(int *p, int b, int c) {
    if (c) *p = 77;
    int old = gi;
    int t = old + b;
    if (t / 3 > 300) return -1;
    gi = t;
    return old + 1;
}
__attribute__((noinline)) int earlier_loop(int *p, int b) {
    int sum = 0;
    for (int i = 0; i < 2; i++) {
        *p = 77 + i;
        int old = gi;
        int t = old + b;
        if (t > 5000) return -1;
        gi = t;
        sum += old;
    }
    return sum;
}
__attribute__((noinline)) unsigned long uid_sequence(unsigned *p) {
    unsigned long uid = *p;
    flag_a = 1;
    flag_b = 1;
    uid_a = uid;
    uid_b = uid;
    return uid;
}
__attribute__((noinline)) long scaled_sequence(long *p) {
    long value = *p * 86400;
    flag_a = 1;
    scaled = value;
    return value;
}
__attribute__((noinline)) int pick(unsigned mode) {
    int v = (int)(mode & 0xffff);
    switch (v) {
        case 30: case 33: sink = v; return v * 2 + (int)mode;
        case 37: sink = v; return v * 3 + (int)mode;
        case 31: case 34: sink = v; return v * 4 + (int)mode;
        default: sink = 77; return 77;
    }
}
#else
int computed(int *, int);
int called(int);
int conditional(int *, int, int);
int before(int *, int);
int repeated(int *, int);
int nonalias(int);
int pointer_store(int *, int);
int earlier_branch(int *, int, int);
int earlier_loop(int *, int);
int pick(unsigned);
#endif

#if defined(GLOBALORDER_MAIN) || defined(GLOBALORDER_HARNESS)
int main(void) {
    for (int alias = 0; alias < 2; alias++) for (int init = -3; init < 4; init++) {
        int *p = alias ? &gi : &other;
        int result;
        gi = init; other = 37; result = computed(p, 5); printf("%d/%d ", result, gi);
        gi = init; other = 37; result = called(5); printf("%d/%d ", result, gi);
        for (int c = 0; c < 2; c++) {
            gi = init; other = 37; result = conditional(p, 5, c); printf("%d/%d ", result, gi);
        }
        gi = init; other = 37; result = before(p, 5); printf("%d/%d ", result, gi);
        gi = init; other = 37; result = repeated(p, 5); printf("%d/%d ", result, gi);
        gi = init; other = 37; result = nonalias(5); printf("%d/%d ", result, gi);
        gi = init; other = 37; result = pointer_store(p, 5); printf("%d/%d/%d ", result, gi, other);
        for (int c = 0; c < 2; c++) {
            gi = init; other = 37; result = earlier_branch(p, 5, c); printf("%d/%d/%d ", result, gi, other);
        }
        gi = init; other = 37; result = earlier_loop(p, 5); printf("%d/%d/%d\n", result, gi, other);
    }
    static const unsigned modes[] = {0,29,30,31,32,33,34,35,36,37,38,65566,65567,65568,65573,0xffffffff};
    for (unsigned i = 0; i < sizeof(modes) / sizeof(*modes); i++) {
        sink = 0;
        int result = pick(modes[i]);
        printf("%u/%d/%d\n", modes[i], result, sink);
    }
    return 0;
}
#endif
#endif
