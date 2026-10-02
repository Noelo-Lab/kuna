/* Narrow integers a callee returns zero-extended (u*) or sign-extended (s*),
   callers that compute with the whole register (c_*), and a loop whose return
   kuna duplicates into each exit on PowerPC (h_loop).  Built with
   clang 14 -fno-inline -c at -O2 for arm-linux-gnueabi (zextreturn_arm.o, and
   -O0 for zextreturn_arm_O0.o), armv7a-linux-gnueabi (_armv7),
   thumbv7-linux-gnueabi (_thumb), powerpc-linux-gnu (_ppc), riscv32-linux-gnu
   (_rv32), riscv64-linux-gnu (_rv64) and mipsel-linux-gnu (_mipsel). */
unsigned short u16_mul(int k) { return k * 300; }
unsigned char u8_add(int k) { return k + 100; }
unsigned char u8_hi(unsigned k) { return k >> 8; }
unsigned short u16_inc(unsigned short *p) { return ++*p; }
unsigned short u16_sum(int n) {
  unsigned short s = 0x7ff0;
  for (int i = 0; i < n; i++)
    s += i * 7;
  return s;
}
short s16_mul(int k) { return k * 300; }
signed char s8_add(int k) { return k + 100; }
int c_u16_mul(int b) { return (b * u16_mul(b)) >> 4; }
int c_u8_add(int b) { return (b * u8_add(b)) >> 4; }
int c_u8_hi(int b) { return (b * u8_hi(b)) >> 4; }
int c_u16_inc(unsigned short *p, int b) { return (b * u16_inc(p)) >> 16; }
int c_u16_sum(int b) { return (b * u16_sum(b & 0xff)) >> 4; }
int c_s16_mul(int b) { return (b * s16_mul(b)) >> 4; }
int c_s8_add(int b) { return (b * s8_add(b)) >> 4; }
unsigned short h_loop(int n, short *a) {
  short s = 0;
  for (int i = 0; i < n; i++)
    s += a[i];
  return s;
}
