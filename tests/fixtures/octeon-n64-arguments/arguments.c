/* Original ABI probes; no target libc or runtime is needed. */
struct One { unsigned char x; };
struct Two { unsigned short x; };
struct Four { unsigned x; };
struct Eight { unsigned x, y; };
union Word { unsigned x; float f; };

unsigned get_one(struct One value) { return value.x; }
unsigned get_two(struct Two value) { return value.x; }
unsigned get_four(struct Four value) { return value.x; }
unsigned get_eight(struct Eight value) { return value.x; }
unsigned get_union(union Word value) { return value.x; }
float float_identity(float value) { return value; }
float float_second(unsigned long p0, float value) { return value; }
double double_identity(double value) { return value; }
struct Four make_four(unsigned value) { struct Four result = {value}; return result; }

#define SEVEN unsigned long p0, unsigned long p1, unsigned long p2, \
    unsigned long p3, unsigned long p4, unsigned long p5, unsigned long p6
#define EIGHT SEVEN, unsigned long p7
float float_last(SEVEN, float value) { return value; }
float float_stack(EIGHT, float value) { return value; }
unsigned four_last(SEVEN, struct Four value) { return value.x; }
unsigned four_stack(EIGHT, struct Four value) { return value.x; }
unsigned uint_stack(EIGHT, unsigned value) { return value; }
float float_stack_second(EIGHT, unsigned p8, float value) { return value; }

unsigned variadic_four(unsigned long p0, ...) {
    __builtin_va_list args;
    __builtin_va_start(args, p0);
    struct Four value = __builtin_va_arg(args, struct Four);
    __builtin_va_end(args);
    return value.x;
}
