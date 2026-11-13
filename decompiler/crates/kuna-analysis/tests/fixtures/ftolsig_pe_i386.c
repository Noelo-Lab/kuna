int f(const float *p, const float *q, float s, int *out) {
    out[0] = (int)(p[0] * s + p[1] * q[0] + p[2] * q[1]);
    out[1] = (int)(p[3] * s + p[4] * q[0] + p[5] * q[1]);
    return out[1] - out[0];
}
