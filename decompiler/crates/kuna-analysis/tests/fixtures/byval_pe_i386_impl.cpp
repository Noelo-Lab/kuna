struct Rect { int x, y, w, h, flags, a, b, c; };
volatile int g_sink;
__declspec(noinline) int __stdcall paint(int a, int b, int c) { g_sink = a + b; return a * c; }
__declspec(noinline) int __stdcall fill(Rect r, int color) { g_sink = color; return r.x + r.y * r.w + r.h - r.flags; }
int __stdcall draw(int a, int b, const Rect *src);
extern "C" void __cdecl start(void) { static Rect s = {1, 2, 3, 4, 5, 6, 7, 8}; volatile int x = 2; g_sink = draw(x, x, &s); }
