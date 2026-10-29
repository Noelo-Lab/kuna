struct pair {float real,imaginary;};
volatile float seen;
void observe(float a,float b) {seen=a+b;}
#ifdef REFERENCE
#define CALLEE_POP __attribute__((stdcall))
#else
#define CALLEE_POP
#endif
void CALLEE_POP fill(struct pair *output) {output->real=3.25f;output->imaginary=-0.5f;}
