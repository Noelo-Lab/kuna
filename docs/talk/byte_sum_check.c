#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

extern unsigned native_byte_sum(const unsigned char *, unsigned);
extern int32_t recovered_byte_sum(const unsigned char *, unsigned);

int main(void) {
    const unsigned capacity = 16843010;
    const unsigned edges[] = {0, 1, 2, 3, 4, 5, 7, 8, 9, 15, 16, 17,
        255, 256, 257, 4095, 4096, 4097, 8421504, 8421505,
        16843008, 16843009, 16843010};
    unsigned char *bytes = malloc(capacity);
    if (!bytes)
        return 2;
    unsigned checked = 0;
    for (unsigned mode = 0; mode < 3; ++mode) {
        uint32_t state = 0x31415926;
        for (unsigned i = 0; i < capacity; ++i) {
            state = state * 1664525u + 1013904223u;
            bytes[i] = mode == 0 ? 0 : mode == 1 ? 255 : (unsigned char)(state >> 24);
        }
        for (unsigned i = 0; i < sizeof(edges) / sizeof(edges[0]); ++i) {
            unsigned n = edges[i];
            unsigned expected = native_byte_sum(bytes, n);
            unsigned actual = (unsigned)recovered_byte_sum(bytes, n);
            if (actual != expected) {
                fprintf(stderr, "mismatch: mode %u length %u expected %u actual %u\n",
                    mode, n, expected, actual);
                return 1;
            }
            ++checked;
        }
        for (unsigned i = 0; i < 500; ++i) {
            state = state * 1664525u + 1013904223u;
            unsigned n = state % 65536u;
            unsigned expected = native_byte_sum(bytes, n);
            unsigned actual = (unsigned)recovered_byte_sum(bytes, n);
            if (actual != expected)
                return 1;
            ++checked;
        }
    }
    free(bytes);
    printf("%u comparisons passed\n", checked);
    return 0;
}
