typedef __SIZE_TYPE__ size_t;
void free(void *p) { (void)p; }
size_t strlen(const char *s) { size_t n = 0; while (s[n]) ++n; return n; }
void sincos(double x, double *s, double *c) { *s = x; *c = x; }
