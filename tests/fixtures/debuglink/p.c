/* Source of link and link.debug (tests/stages/kuna-debuglink.xml):
 *   gcc -O1 -g -fno-inline -Wl,--build-id p.c -o link
 *   objcopy --only-keep-debug link link.debug
 *   strip --strip-all link
 *   objcopy --add-gnu-debuglink=link.debug link
 */

#include <stdio.h>
int helper_alpha(int x) { return x * 7 + 1; }
int helper_beta(int x) { return helper_alpha(x) - 3; }
int main(int argc, char **argv) { printf("%d\n", helper_beta(argc)); return 0; }
