/* Fixture source for the deferred entry pass — a RELOCATABLE x86-64 object whose
   local helpers carry a `<patternpairs>`-matching prologue but no symbol, so
   `entry::full_pattern_starts` is non-empty on it. The loader lays a relocatable
   object out synthetically, so the pass must see the REBASED view: on the raw
   object view its starts come back as section-relative 0x38/0x95, in an address
   space the engine does not have.

   Build:

       gcc -O0 -c -o deferredentry_rel_x86_64.o deferredentry_rel_x86_64.c
       strip -x deferredentry_rel_x86_64.o
*/
#include <stdio.h>

static int helper1(int a, int b) {
    int c = a * b;
    for (int i = 0; i < a; i++)
        c += i;
    return c;
}

static int helper2(int a) {
    char buf[64];
    snprintf(buf, sizeof buf, "%d", a);
    return buf[0];
}

static int helper3(int a, int b, int c) { return helper1(a, b) + helper2(c); }

int pub1(int a) { return helper3(a, a + 1, a + 2); }
int pub2(int a, int b) {
    int t = 0;
    for (int i = 0; i < b; i++)
        t += helper1(i, a);
    return t;
}
int pub3(void) { return pub1(3) + pub2(4, 5); }
int main(void) { return pub3(); }
