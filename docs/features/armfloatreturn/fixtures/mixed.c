/* Synthetic mixed float-return paths and full-double controls. */
float mixed_constant(double x) { return x > 0 ? 1.0f : (float)(x*x); }
float mixed_load(double x, float *p) { return x > 0 ? *p : (float)(x*x); }
float mixed_input(double x, float y) { return x > 0 ? y : (float)(x*x); }
double double_constant(double x) { return x > 0 ? 1.0 : x*x; }
double double_load(double x, double *p) { return x > 0 ? *p : x*x; }
double double_input(double x, double y) { return x > 0 ? y : x*x; }
