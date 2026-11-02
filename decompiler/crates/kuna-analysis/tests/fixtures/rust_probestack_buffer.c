#include <stdint.h>

uint32_t fill_probe_page(uint8_t *out, uint32_t x) {
    for (unsigned int i = 0; i < 4096; ++i)
        out[i] = (uint8_t)(x + i);
    return out[0] + (uint32_t)out[4095] * 257;
}
