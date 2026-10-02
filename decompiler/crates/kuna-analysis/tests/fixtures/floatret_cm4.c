/* A NaN returned in s0 on a Cortex-M4F and a caller that hands it back, the
   shape of crazyflie's libm error paths; NaNs `NAN` cannot spell (a payload,
   a signalling NaN, a negative payload); a double returned in d0; and a float
   result stored through an untyped pointer.
   clang --target=thumbv7em-none-eabihf -mcpu=cortex-m4 -mfloat-abi=hard -O2 -c floatret_cm4.c */
int err;
__attribute__((noinline, weak)) float qnanf_(void) { return __builtin_nanf(""); }
__attribute__((noinline, weak)) float core(float x) { return x * 0.5f; }
__attribute__((noinline)) float logish(float x) {
  if (x > 0)
    return core(x);
  if (x == 0) {
    err = 34;
    return -__builtin_inff();
  }
  err = 33;
  return qnanf_();
}
__attribute__((noinline)) float use(float x) { return logish(x) + 1.0f; }
__attribute__((noinline)) float qp(void) { return __builtin_nanf("0x123"); }
__attribute__((noinline)) float sn(void) { return __builtin_nansf(""); }
__attribute__((noinline)) float nn(void) { return -__builtin_nanf("0x5"); }
__attribute__((noinline)) double third(void) { return 1.0 / 3.0; }
__attribute__((noinline)) void put(void *out, int i, float x) { ((float *)out)[i] = core(x); }
__attribute__((noinline)) void put2(void *out, float x) { *(float *)((char *)out + 8) = core(x); }
