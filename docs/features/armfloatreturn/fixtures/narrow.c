/* Synthetic scalar VFP conversion controls. */
float narrow(double x) { return x + x; }
double widen(float x) { return x + x; }
double keep_double(double x) { return x + x; }
float narrow_paths(double x) {
    if (x > 0.0) return (float)(x + x);
    return (float)x + 1.0f;
}
float narrow_load(double *p) { return *p; }
