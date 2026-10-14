/* A local retyped to an enumeration prints its constants as enumerators.
   gcc -O0 -o retypeenum_x86_64 retypeenum_x86_64.c */
#include <stdio.h>

enum state { IDLE = 0, RUN = 3, DONE = 9 };
enum color { RED = 0, GREEN = 1, BLUE = 7 };

__attribute__((noinline)) int step(const int *ev, int n)
{
    int s = IDLE;
    int k = 0;
    for (int i = 0; i < n; i++) {
        if (s == IDLE && ev[i] > 0)
            s = RUN;
        else if (s == RUN && ev[i] < 0)
            s = DONE;
        if (s == DONE)
            k++;
    }
    return s * 100 + k;
}

__attribute__((noinline)) int pick(const int *a, int n)
{
    int c = RED;
    char seen = 0;
    for (int i = 0; i < n; i++) {
        if (a[i] > 10) {
            c = BLUE;
            seen = 1;
        } else if (a[i] > 5)
            c = GREEN;
    }
    return c * 10 + seen;
}

int main(void)
{
    int e[6] = {0, 2, 1, -1, 4, 0};
    int a[5] = {1, 6, 2, 11, 3};
    printf("%d %d\n", step(e, 6), pick(a, 5));
    return 0;
}
