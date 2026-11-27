extern void paird(double, int);
extern void intdbl(int, double, int);
extern void spill(double, int, double, int, double, int, double, int, int);
extern void many(int, int, int, int, int, int, int, int, float, int);
extern void ninth(int, int, int, int, int, int, int, int, int, int);

void float_first(void) { paird(1.125, 33); }
void int_first(void) { intdbl(2, 3.125, 91); }
void stack_arg(void) { spill(1.125, 2, 3.125, 4, 5.125, 6, 7.125, 8, 99); }
void stack_float(void) { many(1, 2, 3, 4, 5, 6, 7, 8, 9.5f, 10); }
void stack_int(void) { ninth(1, 2, 3, 4, 5, 6, 7, 8, 9, 10); }

double use_intdbl(int a, double b, int c) { return b * a + c; }
float use_many(int a, int b, int c, int d, int e, int f, int g, int h, float i, int j) { return i * a + j; }

extern void vlog(double, ...);
void var_call(void) { vlog(1.5, 2.5, 7); }
