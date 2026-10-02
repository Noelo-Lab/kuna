extern int pick(void);
extern const char *getname(int);
__attribute__((noinline)) const char *wrap(void) { int v = pick(); if (v) return getname(v); return getname(0x23); }
int main(void) { return wrap()[0]; }
