/* GH-904: tail calls through a GOT slot, each caller placed before its callee. */
extern int ext(int);
int f(int x);
long three(long a, long b, long c);
void bump(void);
volatile int ticks;

int g(int x) { return f(x + 1); }
long fwd3(long a, long b, long c) { return three(a, b, c); }
void tick(void) { bump(); }
int h(int x) { return ext(x + 1); }
int f(int x) { return x * 3 + 7; }
long three(long a, long b, long c) { return a * b + c; }
void bump(void) { ticks++; }
