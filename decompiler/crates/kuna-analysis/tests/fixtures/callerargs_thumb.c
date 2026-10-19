/* clang --target=thumbv7em-none-eabi -mcpu=cortex-m4 -O2 -c callerargs_thumb.c
   ld.lld -o callerargs_thumb callerargs_thumb.o --nmagic -Ttext=0x20000 -e run --strip-debug */
typedef unsigned char u8;
typedef short s16;

__attribute__((noinline)) unsigned scale(u8 c, unsigned k) { return c * k + (c >> 3); }
__attribute__((noinline)) int level(s16 v) { return v < 0 ? -v * 3 : v * 5; }
__attribute__((noinline)) unsigned pick(_Bool on, unsigned a, unsigned b) { return on ? a * 7 : b; }
__attribute__((noinline)) unsigned wide(int n) { return n * 9 + (n >> 4); }
__attribute__((noinline)) unsigned taken(u8 c) { return c * 11 + (c >> 1); }
unsigned (*hook)(u8) = taken;

void run(unsigned *out, unsigned x, unsigned y, const unsigned char *p) {
  out[0] = scale(x + 1, y);
  out[1] = scale(p[2] * 3, y);
  out[2] = level(x - 1000);
  out[3] = level((s16)y);
  out[4] = pick(x < y, x, y);
  out[5] = pick(p[0] == 7, y, x);
  out[6] = wide(x & 0xff);
  out[7] = wide(y);
  out[8] = taken(x + 3);
  out[9] = hook(y);
}
