/* Globals a program moves only through floating-point registers, beside ones it
   does not. gd, gf, gd2 and gf2 are only ever loaded into or stored from a float
   register. gpun receives a float's bits through memcpy and use_gpun adds to
   them as an integer; gz is a double zeroed by an integer store at -O2.
   x86_64:  gcc -O2 / clang -O0 -nostdlib -static -fno-pie -no-pie -fno-asynchronous-unwind-tables
   aarch64: aarch64-linux-gnu-gcc -O2 (same flags) floatglobal.c -lgcc
   armhf:   arm-linux-gnueabi-gcc -O2 -marm -mfloat-abi=hard -mfpu=vfpv3-d16 (same flags) floatglobal.c -lgcc */
#define NI __attribute__((noinline))
double gd;
float gf;
double gd2;
float gf2;
int gi;
int gpun;
double gz;
NI void fy(int *p, double b) { gd = b; *p = 1; }
NI void fw(int a, double b) { gi = a; gd = b; }
NI void ff(int *p, float b) { gf = b; *p = 2; }
NI double sink(double x) { return x * 3.0; }
NI float sinkf(float x) { return x * 3.0f; }
NI double pass(void) { return sink(gd2); }
NI float passf(void) { return sinkf(gf2); }
NI void setd2(double v) { gd2 = v; gf2 = (float)v; }
NI void set_gpun(float f) { __builtin_memcpy(&gpun, &f, 4); }
NI int use_gpun(void) { return gpun + 1; }
NI void setz(double v) { gz = v; }
NI void clearz(void) { gz = 0.0; }
NI int run(int n) {
  fy(&gi, 2.5); fw(3, 4.5); ff(&gi, 1.5f); setd2(n * 0.5);
  set_gpun(n * 0.25f); setz(n); clearz();
  return (int)(gd + gf + pass() + passf() + gz) + use_gpun() + gi;
}
volatile int sinkv;
void _start(void) { sinkv = run(sinkv); for (;;) {} }
