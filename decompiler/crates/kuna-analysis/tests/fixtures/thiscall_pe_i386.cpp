struct V {
    float x, y, z;
    float len2() const;
    float len2_impl() const;
};
__declspec(noinline) float V::len2_impl() const { return x * x + y * y; }

struct B {
    int n;
    int k[3];
    B();
};
__declspec(noinline) B::B() { n = 7; k[0] = k[1] = k[2] = 0; }

__declspec(noinline) float __stdcall use(float a, float b) {
    V v;
    v.x = a;
    v.y = b;
    v.z = 0.0f;
    B buf;
    return v.len2() + (float)buf.n;
}

volatile float sink;
extern "C" void __cdecl start(void) { volatile float p = 1.0f; sink = use(p, p); }
