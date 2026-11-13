struct Vec { float x, y; float angle() const; };
struct Motor { virtual int message(int id, int flags, float value, int extra); };
volatile float g_sink;
__declspec(noinline) float Vec::angle() const { return x * 2.0f + y; }
__declspec(noinline) int Motor::message(int id, int flags, float value, int extra) { g_sink = value; return id + flags + extra; }
int __stdcall steer(Motor *m, const Vec *v, int extra);
extern "C" void __cdecl start(void) { static Motor m; static Vec v = {1.0f, 2.0f}; volatile int e = 3; g_sink = (float)steer(&m, &v, e); }
