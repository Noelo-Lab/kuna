float __stdcall dist(float s1, float t1, float s2, float t2, float t);
int __stdcall corte(float a, float b, float c, float d, float *os, float *ot);

__declspec(noinline) float __stdcall dist_impl(float s1, float t1, float s2, float t2, float t) {
    return s1 + (s2 - s1) * (t - t1) / (t2 - t1);
}

__declspec(noinline) int __stdcall corte_impl(float a, float b, float c, float d, float *os, float *ot) {
    *os = a - c;
    *ot = b - d;
    return a > b;
}

__declspec(noinline) float __stdcall split(float a, float b, float c, float d, float e) {
    float os, ot;
    float x = dist(a, b, c, d, e);
    float y = dist(b, c, d, e, a);
    if (corte(x, y, a, b, &os, &ot))
        return os + ot;
    return x - y;
}

volatile float sink;
void __cdecl start(void) { volatile float v = 1.0f; sink = split(v, v, v, v, v); }
