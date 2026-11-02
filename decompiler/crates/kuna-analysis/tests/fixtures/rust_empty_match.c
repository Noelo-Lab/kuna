#include <stdio.h>
#include <stdlib.h>
extern int empty_case(unsigned long);
extern int empty_default(unsigned long);
extern int empty_shared(unsigned long);
extern int empty_loop(unsigned long, int);
extern int loop_break_control(int, int);
int main(int argc,char **argv) {
    if (argc != 3) return 1;
    unsigned long x = strtoul(argv[1],0,0);
    int n = atoi(argv[2]);
    printf("%d %d %d %d %d\n", empty_case(x),empty_default(x),empty_shared(x),empty_loop(x,n),loop_break_control(10,n));
    return 0;
}
