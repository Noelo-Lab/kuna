struct Small { int value; };
union Large { long long words[3]; };

extern struct Small small_result(double, long long);
extern union Large large_result(float, double);
extern void sink(double, int);
extern void sink_pair(float, float, int);

int call_small(void) {
    return small_result(3.5, 0x1122334455667788LL).value;
}

long long call_large(void) {
    return large_result(1.25f, 6.5).words[0];
}

void forward(double x, int n) { sink(x, n); }
void forward_pair(float x, float y, int n) { sink_pair(x, y, n); }

volatile double saved;
__attribute__((noinline, visibility("hidden")))
void save_one(double x) { saved = x; }

void call_save(void) { save_one(3.75); }
