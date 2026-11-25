typedef __SIZE_TYPE__ size_t;
void free(void *);
size_t strlen(const char *);
void sincos(double, double *, double *);
void release(void *p) { free(p); }
size_t measure(const char *s) { return strlen(s) + 1; }
void turn(double x, double *s, double *c) { sincos(x, s, c); }
