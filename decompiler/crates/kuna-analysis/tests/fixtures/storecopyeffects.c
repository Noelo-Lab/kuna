int gi;
void touch(void);
int s7(int b);
int s_repeat(int b);
int s_alias(int *p, int b);
int s_read(int b);
int s_branch(int b);
int s_dead(int b);
int s_two(int *p, int b);

#ifndef STORECOPY_HARNESS
int s7(int b) { gi = b; touch(); gi = 9; touch(); return 1; }
int s_repeat(int b) { gi = b; touch(); gi = 3; touch(); gi = 7; touch(); gi = 11; touch(); return gi; }
int s_alias(int *p, int b) { gi = b; touch(); *p = 13; touch(); gi = 9; touch(); return *p; }
int s_read(int b) { gi = b; touch(); int x = gi; gi = 9; touch(); return x; }
int s_branch(int b) { gi = b; touch(); if (b) gi = 9; else gi = 17; touch(); return 1; }
int s_dead(int b) { gi = b; gi = 9; touch(); return gi; }
int s_two(int *p, int b) { int *q = &gi; gi = b; touch(); *q = 9; *p = 21; touch(); return *q; }
#endif

#ifdef STORECOPY_DRIVER
#include <stdio.h>
static int seen[8], count;
void touch(void) { seen[count++] = gi; gi ^= 0x55; }
static void reset(void) { gi = -100; count = 0; }
static void result(const char *name, int value) {
    printf("%s %d %d", name, value, gi);
    for (int i = 0; i < count; ++i) printf(" %d", seen[i]);
    putchar('\n');
}
int main(void) {
    int v;
    reset(); result("s7", s7(4));
    reset(); result("s_repeat", s_repeat(4));
    reset(); result("s_alias", s_alias(&gi, 4));
    reset(); v = -3; result("s_other", s_alias(&v, 4));
    reset(); result("s_read", s_read(4));
    reset(); result("s_branch", s_branch(4));
    reset(); result("s_else", s_branch(0));
    reset(); result("s_dead", s_dead(4));
    reset(); result("s_two", s_two(&gi, 4));
    reset(); v = -3; result("s_distinct", s_two(&v, 4));
    return 0;
}
#endif
