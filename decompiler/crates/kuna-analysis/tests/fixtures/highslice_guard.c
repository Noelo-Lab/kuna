#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
extern unsigned int narrow_guard(uint64_t);
extern unsigned int wide_guard(uint64_t);
extern unsigned int explicit_zext(uint64_t);
extern unsigned int byte_guard(uint64_t);
extern unsigned int word_guard(uint64_t);
extern unsigned int signed_guard(uint64_t);
int main(int argc, char **argv) {
    if (argc != 2) return 1;
    uint64_t x = strtoull(argv[1], 0, 0);
    printf("%u %u %u %u %u %u\n", narrow_guard(x),wide_guard(x),explicit_zext(x),byte_guard(x),word_guard(x),signed_guard(x));
    return 0;
}
