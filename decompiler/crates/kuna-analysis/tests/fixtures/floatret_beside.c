/* A function that returns a packed global's float on one path and 0.0f on the
   other, in two RETURNs: a value read from a global refuses the float, and the
   constant beside it is returned in the same register.
   gcc -O2 -o floatret_beside_gcc_O2 floatret_beside.c && strip floatret_beside_gcc_O2 */
#include <stdio.h>
#define NI __attribute__((noinline))
struct __attribute__((packed)) st { char ready; char pad[6]; float f; };
struct st state = {1, {0}, 2.5f};
NI float getf(int k) { if (k) return state.f; return 0.0f; }
int main(int argc, char **argv) { (void)argv; printf("%g\n", (double)getf(argc)); return 0; }
