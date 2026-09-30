/* Synthetic return-width controls. */
float constant_paths(double x) { return x*x > 1.5 ? 1.0f : 2.0f; }
float loaded_paths(double x, float *p) { return x*x > 1.5 ? p[0] : p[1]; }
float copied_paths(double x, float a, float b) { return x*x > 1.5 ? a : b; }
double double_paths(double x) { return x*x > 1.5 ? 1.0 : 2.0; }
double double_loaded_paths(double x, double *p) { return x*x > 1.5 ? p[0] : p[1]; }
double double_copied_paths(double x, double a, double b) { return x*x > 1.5 ? a : b; }
float integer_bits(double x, unsigned int y) { union { unsigned int u; float f; } v; v.u = y ^ 0x80000000u; return x*x > 1.5 ? v.f : 2.0f; }
double double_low_bits(double x) { union { double d; unsigned int u[2]; } v; v.d=x*x; v.u[0]=0; return v.d; }
