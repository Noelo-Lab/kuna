extern const float one;
extern const double two;
extern const double huge;

int above(float x) { return x > one; }
int at_least(float x) { return x >= one; }
int not_above(float x) { return !(x > one); }
int not_at_least(float x) { return !(x >= one); }
int pick(double x, double y) { return x >= y ? 3 : 5; }
int not_above_pair(double x, double y) { return !(x > y); }
int above_two(double x) { if (x > two) return 3; return 5; }

extern double work(double);
double nan_or_huge(double x) { if (__builtin_isnan(x) || x > huge) return x + x; return work(x) + x; }
int ordered_below_pair(float x, float y) { return !(x >= y) & !__builtin_isunordered(x, y); }
