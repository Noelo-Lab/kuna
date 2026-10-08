#include <math.h>
#include <stdio.h>

const float one = 1.0f;
const double two = 2.0;

extern int above(float);
extern int at_least(float);
extern int not_above(float);
extern int not_at_least(float);
extern int pick(double, double);
extern int not_above_pair(double, double);
extern int above_two(double);

static int failures;

static void check(int got, int want, const char *name, double x, double y) {
    if (got == want) return;
    fprintf(stderr, "%s(%g, %g): got %d, expected %d\n", name, x, y, got, want);
    failures++;
}

int main(void) {
    const double inputs[] = {NAN, -NAN, 0.0, -0.0, INFINITY, -INFINITY, 1.0, -1.0, 2.0, 2.5, 1e-40};
    const int count = sizeof(inputs) / sizeof(inputs[0]);
    for (int i = 0; i < count; i++) {
        float f = (float)inputs[i];
        double x = inputs[i];
        check(above(f), f > one, "above", f, 0);
        check(at_least(f), f >= one, "at_least", f, 0);
        check(not_above(f), !(f > one), "not_above", f, 0);
        check(not_at_least(f), !(f >= one), "not_at_least", f, 0);
        check(above_two(x), x > two ? 3 : 5, "above_two", x, 0);
        for (int j = 0; j < count; j++) {
            double y = inputs[j];
            check(pick(x, y), x >= y ? 3 : 5, "pick", x, y);
            check(not_above_pair(x, y), !(x > y), "not_above_pair", x, y);
        }
    }
    return failures != 0;
}
