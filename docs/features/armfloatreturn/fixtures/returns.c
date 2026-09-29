extern int sscanf(const char *, const char *, ...);
__attribute__((noinline)) double scale(double value) { return value * 1.5; }
__attribute__((noinline)) double fixed(void) { return 1.5; }
__attribute__((noinline)) double wrapper(double value) { return scale(value) + 0.5; }
unsigned integer_value(void) { return 7; }
double read_double(const char *text) {
    double value = 0;
    sscanf(text, "%lf", &value);
    return value;
}
