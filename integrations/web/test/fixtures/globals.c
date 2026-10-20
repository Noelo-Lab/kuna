/* Global data for the Data tab: a string, an int table, a struct, pointers to
 * a function and to data, an uninitialized buffer and a read-only constant.
 *
 * Rebuild:  gcc -O0 -g -o globals.elf globals.c
 *           strip -o globals-stripped.elf globals.elf
 */
#include <stdio.h>
#include <string.h>

struct point { int x; int y; const char *label; };

char greeting[] = "hello, data";
int primes[8] = {2, 3, 5, 7, 11, 13, 17, 19};
struct point origin = {3, 4, "origin"};
const long limit = 1000;
const char *banner = "kuna data tab";
char scratch[64];

static int twice(int v) { return v * 2; }

int (*op)(int) = twice;
int *favorite = &primes[3];

int main(int argc, char **argv) {
    int total = 0;
    for (int i = 0; i < 8; i++) total += primes[i];
    strcpy(scratch, greeting);
    total += op(origin.x) + *favorite + argc;
    if (total > limit) puts(banner);
    printf("%s %d %s\n", scratch, total, origin.label);
    return 0;
}
