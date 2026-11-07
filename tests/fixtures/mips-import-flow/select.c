/* Original automatic-target selection and indirect-call continuation witness. */
int callit(int (*f)(int, int, int), int x) { return f(x, 2, 3) + x; }
void __start(void) { for (;;) {} }
