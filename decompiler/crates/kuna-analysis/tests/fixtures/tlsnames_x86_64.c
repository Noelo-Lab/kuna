
#include <stdio.h>
__thread int counter;
__thread char buf[64] = "init";
static __thread long hits = 5;
int bump(int n) { counter += n; hits++; return counter + buf[0]; }
int main(int argc, char **argv) { printf("%d\n", bump(argc)); return 0; }
