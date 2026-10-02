/* A float handed on to a callee that takes its bits as an integer.
   clang --target=thumbv7em-none-eabihf -mcpu=cortex-m4 -mfloat-abi=hard -O2 -c -o floatret_put_cm4.o floatret_put.c
   clang --target=aarch64-linux-gnu -O2 -c -o floatret_put_a64.o floatret_put.c */
typedef unsigned int u32;
struct st { u32 a, b, c; };
__attribute__((noinline)) void put3(u32 x, u32 y, u32 z, struct st *s) { s->a = x; s->b = y; s->c = z; }
__attribute__((noinline)) void putf2(float x, float y, float z, struct st *s) {
  union { float f; u32 u; } a = {x}, b = {y}, c = {z};
  put3(a.u, b.u, c.u, s);
}
