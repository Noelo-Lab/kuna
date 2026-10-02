/* Built twice for ARMv4T: PART 1 with -mthumb, PART 2 with -marm. ARMv4T has
   no blx, so the linker reaches the A32 function from Thumb through a
   `bx pc; nop; b` veneer. */
#if PART == 1
extern int arm_target(int x);
int thumb_caller(int x) { return arm_target(x) * 2 + 1; }
#else
extern int thumb_caller(int x);
int arm_target(int x) { int s = 0; for (int i = 0; i < x; i++) s += i ^ 0x2d; return s; }
__attribute__((noreturn)) void _start(void) { volatile int r = thumb_caller(5); for (;;) r++; }
#endif
