const char msg[] = "hello from a TE image";
__attribute__((noinline)) const char *greeting(void) { return msg; }
__attribute__((noinline)) int helper(int x) { return x * 3 + 1; }
int entry(int x) { return greeting()[x] + helper(x); }
