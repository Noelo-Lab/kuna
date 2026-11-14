struct Vec { float x, y; float angle() const; };
struct Motor { virtual int message(int id, int flags, float value, int extra); };
__declspec(noinline) int __stdcall steer(Motor *m, const Vec *v, int extra) {
    return m->message(7, 0, v->angle(), extra);
}
