int k = 1;
__attribute__((noinline)) int f(int x) { return x + k; }
int main(void) { int t = f(7); k = 42; return t + k; }
