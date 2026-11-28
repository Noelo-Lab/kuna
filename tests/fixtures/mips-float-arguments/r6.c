extern void later(int, float, int);
extern void first(double, int);
extern void aligned(float, double, int);
extern void boundary(int, int, int, float, int);
extern void wide_first(long long, float, int);
extern void wide_later(int, double, int);

void float_double(void) { aligned(2.5f, 6.25, 41); }
void last_register(void) { boundary(1, 2, 3, 4.5f, 5); }
void pair_first(void) { wide_first(0x1122334455667788LL, 7.5f, 11); }
void pair_later(void) { wide_later(1, 2.5, 3); }
void mixed(void) { later(16, 9.5f, 34); }
void leading(void) { first(3.125, 23); }

double use_aligned(float a, double b, int c) { return a * b + c; }
float use_later(int a, float b, int c) { return b * a + c; }
double use_wide_later(int a, double b, int c) { return b * a + c; }

extern void vlog(double, ...);
void var_call(void) { vlog(1.5, 2.5, 7); }
