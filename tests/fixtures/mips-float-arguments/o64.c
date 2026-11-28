extern void leading(double, int);
extern void later(int, double, int);
extern void consecutive(float, float, double, int, float);

void float_first(void) { leading(2.5, 73); }
void int_first(void) { later(1, 3.5, 91); }
void two_floats(void) { consecutive(1.25f, 2.5f, 3.5, 4, 5.5f); }

double use_later(int a, double b, int c) { return b * a + c; }
double use_consecutive(float a, float b, double c, int d, float e) { return a + b + c + d + e; }

extern void vlog(double, ...);
void var_call(void) { vlog(1.5, 2.5, 7); }
