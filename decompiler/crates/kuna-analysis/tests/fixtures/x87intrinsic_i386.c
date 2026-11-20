#include <math.h>

void polar(float angle, float r, float *out) {
    float rad = angle * 0.0174532925f;
    out[0] = (float)(cos(rad) * r);
    out[1] = (float)(sin(rad) * r);
    out[2] = (float)sqrt(out[0] * out[0] + out[1] * out[1]);
}

void angles(float y, float x, float *out) {
    out[0] = (float)atan2(y, x);
    out[1] = (float)pow(y, x);
    out[2] = (float)fmod(y, x);
}
