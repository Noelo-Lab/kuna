extern const float one;
extern const double two;

int above(float x) { return x > one; }
int at_least(float x) { return x >= one; }
int not_above(float x) { return !(x > one); }
int not_at_least(float x) { return !(x >= one); }
int pick(double x, double y) { return x >= y ? 3 : 5; }
int not_above_pair(double x, double y) { return !(x > y); }
int above_two(double x) { if (x > two) return 3; return 5; }
