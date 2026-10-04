#include <stdint.h>
#include <stdio.h>
#include <string.h>

extern float scalar_min(float, float);
extern float ratio_clamp(float, float, float);
extern int ordered_above(float, float);

static float value(uint32_t bits) {
    float result;
    memcpy(&result, &bits, sizeof(result));
    return result;
}

static uint32_t bits(float value) {
    uint32_t result;
    memcpy(&result, &value, sizeof(result));
    return result;
}

static int check(float got, float want, const char *name) {
    if (bits(got) == bits(want)) return 0;
    fprintf(stderr, "%s: got %08x, expected %08x\n", name, bits(got), bits(want));
    return 1;
}

int main(void) {
    const uint32_t inputs[] = {
        0, 0x80000000, 0x3f800000, 0xbf800000, 0x3e800000,
        0x7f800000, 0xff800000, 0x7fc12345, 0xffc54321
    };
    for (unsigned i = 0; i < sizeof(inputs) / sizeof(*inputs); ++i) {
        for (unsigned j = 0; j < sizeof(inputs) / sizeof(*inputs); ++j) {
            float a = value(inputs[i]), b = value(inputs[j]);
            if (check(scalar_min(a, b), a < b ? a : b, "scalar_min")) return 1;
            if (ordered_above(a, b) != (a > b)) {
                fprintf(stderr, "ordered_above failed for %08x, %08x\n", inputs[i], inputs[j]);
                return 1;
            }
        }
    }
    const float finite[][3] = {{-1,0,1}, {2,0,1}, {.25f,0,1}, {-0.0f,0,1}};
    for (unsigned i = 0; i < sizeof(finite) / sizeof(*finite); ++i) {
        float q = (finite[i][0] - finite[i][1]) / (finite[i][2] - finite[i][1]);
        float upper = 1.0f < q ? 1.0f : q;
        if (check(ratio_clamp(finite[i][0], finite[i][1], finite[i][2]),
                  upper < 0.0f ? 0.0f : upper, "ratio_clamp")) return 1;
    }
    if (check(ratio_clamp(value(0x7fc12345), 0, 1), value(0x7fc12345), "ratio NaN")) return 1;
    if ((bits(ratio_clamp(0, 0, 0)) & 0x7fffffff) <= 0x7f800000) return 1;
    puts("MINSS values match");
    return 0;
}
