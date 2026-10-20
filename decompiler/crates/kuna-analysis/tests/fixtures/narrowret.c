/* narrowret fixture: callees that return a narrow value zero-extended, and
   callers that hold their results.  `pick` merges a truth value with a byte
   truncated from a word, and `either_half` merges a zero-extended byte with a
   sign-extended half-word: neither is narrowed.
   Pointer-free, so the printed C of the ARM build recompiles and runs
   natively (kuna-cli tests/narrow_ret.rs).
     clang 14: clang --target=thumbv7em-none-eabi -mcpu=cortex-m4 -O2 -c narrowret.c -o narrowret_thumb_O2.o
     gcc 11:   gcc -O2 -c narrowret.c -o narrowret_gcc_O2_x86_64.o */
#include <stdbool.h>
#include <stdint.h>
#if defined(__clang__)
#define NI __attribute__((noinline))
#define NOTAIL
#else
#define NI __attribute__((noinline, noipa))
#define NOTAIL __attribute__((optimize("no-optimize-sibling-calls")))
#endif

NI uint8_t next_byte(uint32_t x) { return x + 1; }
NI uint16_t scaled(uint32_t x) { return x * 3; }
NI bool above(uint32_t a, uint32_t b) { return a > b; }
NI int8_t signed_byte(uint32_t x) { return x; }
NI uint32_t whole(uint32_t x) { return x * 5; }
NI uint32_t mix(uint32_t a, uint32_t b) { return a * 7 + b; }

NI uint32_t larger_byte(uint32_t x) {
  uint8_t a = next_byte(x), b = next_byte(x >> 8);
  if (a < b)
    a = b;
  return mix(a, a == 0x80) + 1;
}

NI uint32_t halfword_pair(uint32_t x) {
  uint16_t h = scaled(x), g = scaled(x >> 16);
  if (h > 0x8000)
    h = g;
  return mix(h, h >> 3) + 1;
}

NI uint32_t count_above(uint32_t x, uint32_t y) {
  bool b = above(x, y), c = above(y, x);
  if (!b)
    b = c;
  return mix(b, x) + 1;
}

NI int32_t keeps_sign(uint32_t x) {
  int8_t s = signed_byte(x), t = signed_byte(x >> 8);
  if (s < t)
    s = t;
  return mix(s, s < -3) + 1;
}

NI uint32_t keeps_width(uint32_t x) {
  uint32_t w = whole(x), v = whole(x >> 3);
  if (w < v)
    w = v;
  return mix(w, w > 0x80) + 1;
}

NI uint32_t returns_word(uint32_t x) {
  uint32_t w = whole(x);
  return w > 0x80 ? w : w * 2;
}

NI NOTAIL uint8_t pick(uint32_t a, uint32_t b) {
  if (a & 2)
    return mix(b, a);
  return above(a ^ b, b);
}

NI uint32_t use_pick(uint32_t a, uint32_t b) { return pick(a, b) * 3u + 1; }

NI int16_t signed_half(uint32_t x) { return x * 7; }

NI uint32_t either_half(uint32_t x) {
  uint32_t r = (x & 2) ? next_byte(x) : signed_half(x >> 1);
  return r ^ x;
}
