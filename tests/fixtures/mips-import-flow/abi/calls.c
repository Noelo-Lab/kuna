typedef float _Complex pair;
extern void putfloat(float);
extern void takecomplex(int, pair, int);
extern void takepointer(pair *);
extern int release_value(int, int) __asm__("free");
extern int old_thing(int, int);
__asm__(".symver old_thing,thing@VER_1");
extern void thing(int *);
extern void machine(int, int);
extern int source(int);

void float_arg(void) { putfloat(3.0f); }
void complex_arg(void) { takecomplex(11, 1.0f + 2.0fi, 22); }
void pointer_arg(pair *p) { takepointer(p); }
int release_result(int x) { return release_value(x + 1, 12345) + 1; }
int old_version(int x) { return old_thing(x + 1, 123) + 11; }
void new_version(int *p) { thing(p); }
void machine_args(int x, int y) { machine(x, y); }
int source_result(int x) { return source(x + 1) + 5; }

extern void partial(int, int);
void forward_partial(int x, int y) { partial(x, y); }
extern void wider(int, double);
extern void wide_integer(int, long long);
void send_double(void) { wider(7, 3.5); }
void send_integer(void) { wide_integer(9, 0x123456789abcdefLL); }
extern int orphan(int);
int orphan_result(int x) { return orphan(x + 2) + 7; }
extern void one_version(int, int);
void forward_version(int x, int y) { one_version(x, y); }
