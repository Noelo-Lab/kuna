typedef unsigned int U;
U global_value = 17;
static const U table[4] = { 0x12345678U, 7, 0xffffffffU, 0x80000000U };
extern U external(U);
extern U optional(U) __attribute__((weak));

__attribute__((noinline)) U read_global(U x) { return global_value + x; }
__attribute__((noinline)) U read_table(U x) { return table[x & 3]; }
__attribute__((noinline)) U call_external(U x) { return external(x + 9); }
__attribute__((noinline)) static U local(U x) { return (x * 7) ^ 0x125; }
__attribute__((noinline)) U call_local(U x) { return local(x) + 5; }
__attribute__((noinline)) U call_optional(U x) { return optional ? optional(x) : (x ^ 0x42); }
__attribute__((noinline)) U *global_pointer(void) { return &global_value; }
