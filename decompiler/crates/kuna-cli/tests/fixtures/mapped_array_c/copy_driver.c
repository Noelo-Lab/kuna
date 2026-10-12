#include <stdint.h>
unsigned int source_words[5] = {0x11223344, 0x55667788, 0x99aabbcc, 0xddeeff00, 0x01234567};
unsigned char mutable_tile_fallback[20];
void CopyFiveWords(void);
int main(void) {
    CopyFiveWords();
    for (unsigned i = 0; i < 20; ++i) {
        unsigned shift = 8 * (BIG_ENDIAN_TARGET ? (3 - i%4) : i%4);
        if (mutable_tile_fallback[i] != (unsigned char)(source_words[i/4] >> shift)) return 1;
    }
    return 0;
}
