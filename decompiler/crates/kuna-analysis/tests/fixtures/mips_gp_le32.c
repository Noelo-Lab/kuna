#include <stdio.h>

/* A global in .data and one in .sdata-eligible range: $gp-relative GOT/.sdata
   loads are what we want the prologue's $gp recovery to resolve. */
int counter = 7;
const char *banner = "kuna-mips";

int bump(int n) {
    counter += n;        /* $gp-relative load/store of `counter` */
    return counter;
}

int main(void) {
    int r = bump(3);
    printf("%s %d\n", banner, r);  /* libc call -> needs $gp set up */
    return r;
}
