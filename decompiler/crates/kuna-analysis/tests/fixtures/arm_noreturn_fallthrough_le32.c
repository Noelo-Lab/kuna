/* Interworking shapes after a call that never returns, built into two stripped
   armel shared objects (see fixtures/README.md). */
#include <stdlib.h>
#if PART == 1
__attribute__((visibility("hidden"))) int arm_hidden(int x);
int thumb_api(int x) { return arm_hidden(x) + 3; }
#elif PART == 2
__attribute__((noreturn)) void fatal(int);
int thumb_last(int x) { if (x > 5) return x * 7; fatal(x); }
#elif PART == 3
__attribute__((visibility("hidden"))) int arm_hidden(int x) { int s = 0; for (int i = 0; i < x; i++) s += (i * i) ^ 5; return s; }
int arm_api(int x) { return x * 3 - 1; }
#elif PART == 4
__attribute__((noreturn)) void fatal(int c) { exit(c + 1); }
#elif PART == 5
__attribute__((visibility("hidden"))) int thumb_hidden(int x);
int arm_api(int x) { return thumb_hidden(x) + 3; }
#elif PART == 6
__attribute__((noreturn)) void fatal(int);
int arm_last(int x) { if (x > 5) return x * 7; fatal(x); }
#elif PART == 7
__attribute__((visibility("hidden"))) int thumb_hidden(int x) { int s = 0; for (int i = 0; i < x; i++) s += (i * i) ^ 5; return s; }
#endif
