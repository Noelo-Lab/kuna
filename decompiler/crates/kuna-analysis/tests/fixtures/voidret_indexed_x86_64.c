/*
 * voidret_indexed_x86_64 -- a returned value the function also uses as an
 * index is still its return value.
 *
 *     gcc -O2 -fno-inline -fno-pie -no-pie -fcf-protection=none \
 *         voidret_indexed_x86_64.c -o voidret_indexed_x86_64
 *
 * gnulib base_len, which grep, find and gzip carry. `len` comes back from
 * strlen in rax, is decremented in rax, indexes `name`, and is returned in
 * rax; main reads it. kuna printed `void base_len(char *a0)` with the loop and
 * no return.
 */
#include <string.h>
__attribute__((noinline)) size_t base_len(char const *name) {
  size_t len;
  for (len = strlen(name); 1 < len && name[len - 1] == '/'; len--) continue;
  return len;
}
int main(int argc, char **argv) { (void)argc; return (int)base_len(argv[0]) + 1; }
