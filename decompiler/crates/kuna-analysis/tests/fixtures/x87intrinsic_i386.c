#include <math.h>

void polar(float angle, float r, float *out) {
    float rad = angle * 0.0174532925f;
    out[0] = (float)(cos(rad) * r);
    out[1] = (float)(sin(rad) * r);
    out[2] = (float)sqrt(out[0] * out[0] + out[1] * out[1]);
}
