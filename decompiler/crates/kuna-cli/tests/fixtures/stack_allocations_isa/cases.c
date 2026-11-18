#include "observer.h"
#define KEEP __attribute__((noinline))
KEEP unsigned sample_one(unsigned n) {
  unsigned char *p = __builtin_alloca(n);
  p[0] = 7;
  return observe(p, n);
}
KEEP unsigned sample_two(unsigned n) {
  unsigned char *p = __builtin_alloca(n);
  unsigned char *q = __builtin_alloca(n + 17);
  p[0] = 3;
  q[0] = 7;
  return observe(p, n) + observe(q, n + 17);
}
KEEP unsigned sample_branch(unsigned n) {
  unsigned char *p;
  if (n & 1)
    p = __builtin_alloca(n);
  else
    p = __builtin_alloca(n + 17);
  p[0] = 7;
  return observe(p, n);
}
KEEP unsigned sample_loop(unsigned n) {
  unsigned total = 0;
  for (unsigned i = 0; i < 3; i++) {
    unsigned char *p = __builtin_alloca(n + i);
    p[0] = (unsigned char)(7 + i);
    total += observe(p, n + i);
  }
  return total;
}
KEEP unsigned sample_scoped(unsigned n) {
  unsigned total = 0;
  for (unsigned i = 0; i < 20000; i++) {
    unsigned char p[n + 4000];
    p[0] = (unsigned char)(7 + i);
    total += observe(p, n + 4000);
  }
  return total;
}
KEEP unsigned sample_aligned(unsigned n) {
  unsigned char *p = __builtin_alloca_with_align(n, 512);
  p[0] = 7;
  return observe_aligned(p, n);
}
KEEP unsigned sample_outgoing(unsigned n) {
  unsigned char *p = __builtin_alloca(n);
  p[0] = 7;
  return observe_many(p, n, n + 1, n + 2, n + 3, n + 4, n + 5, n + 6, n + 7,
                      n + 8, n + 9, n + 10);
}

KEEP unsigned sample_vla_block(unsigned n) {
  unsigned result;
  {
    unsigned char data[n + 1];
    data[0] = 7;
    result = observe(data, n + 1);
  }
  return result + finish_vla(n);
}

KEEP unsigned sample_vla_branch(unsigned n) {
  unsigned result = 19;
  if (n & 1) {
    unsigned char data[n + 3];
    data[0] = 11;
    result = observe(data, n + 3);
  }
  return result + finish_vla(n);
}

KEEP unsigned sample_vla_loop_exit(unsigned n) {
  unsigned result = 0;
  {
    unsigned char data[n + 1];
    for (unsigned i = 0; i < n; i++) {
      data[0] = (unsigned char)i;
      result += observe(data, n + 1);
      if (i == (n & 7))
        break;
    }
  }
  return result + finish_vla(n);
}

KEEP unsigned sample_vla_early(unsigned n) {
  unsigned char data[n + 1];
  data[0] = (unsigned char)n;
  if (n & 1)
    return observe(data, n + 1);
  return observe(data, n + 1) + finish_vla(n);
}

KEEP unsigned sample_vla_variadic(unsigned n) {
  unsigned result = vla_register_variadic(1, n);
  result += vla_variadic(0, n, 2u, 3u, 4u, 5u, 6u, 7u, 8u);
  unsigned char delimiter[2];
  delimiter[0] = (unsigned char)(n ^ ':');
  delimiter[1] = 0;
  {
    unsigned char data[n + 1];
    mutate_delimiter(delimiter);
    data[0] = delimiter[0];
    result += observe(data, n + 1);
    result += vla_register_variadic(1, n);
    result += vla_variadic(0, n, 11u, 12u, 13u, 14u, 15u, 16u, 17u);
  }
  return result + finish_vla(n);
}

KEEP unsigned sample_vla_escape(unsigned n) {
  unsigned result;
  {
    unsigned char data[n + 1];
    data[0] = 7;
    escaped_vla_address = data;
    result = escape_vla(data, n + 1);
  }
  return result + finish_vla(n);
}

KEEP unsigned sample_vla_aligned(unsigned n) {
  unsigned result;
  {
    _Alignas(64) unsigned char data[n + 1];
    data[0] = 7;
    result = observe_aligned(data, n + 1);
  }
  return result + finish_vla(n);
}

KEEP unsigned sample_vla_product(unsigned n) {
  unsigned result;
  {
    unsigned char data[(n & 63) * 3 + 1];
    data[0] = 7;
    result = observe(data, (n & 63) * 3 + 1);
  }
  return result + finish_vla(n);
}

KEEP unsigned sample_vla_sequential(unsigned n) {
  unsigned result;
  {
    unsigned char first[n + 1];
    first[0] = 7;
    result = observe(first, n + 1);
  }
  {
    unsigned char second[(n & 63) + 1];
    second[0] = 7;
    result += observe(second, (n & 63) + 1);
  }
  return result + finish_vla(n);
}

KEEP unsigned sample_vla_reentry(unsigned n) {
  unsigned result = 0;
  for (unsigned i = 0; i < 3; i++) {
    unsigned char data[n + i + 1];
    data[0] = 7;
    result += observe(data, n + i + 1);
  }
  return result + finish_vla(n);
}

KEEP unsigned sample_vla_liveout(unsigned n) {
  __UINTPTR_TYPE__ address;
  unsigned result;
  {
    _Alignas(4) unsigned char data[n + 1];
    data[0] = 7;
    address = (__UINTPTR_TYPE__)data;
    result = observe(data, n + 1);
  }
  result += inspect_expired((void *)address, n);
  result += inspect_expired((void *)address, n + 1);
  return result + finish_vla(n);
}

KEEP unsigned sample_vla_numeric_result(unsigned n) {
  __UINTPTR_TYPE__ address;
  unsigned result;
  {
    unsigned char data[n + 1];
    data[0] = 7;
    result = observe(data, n + 1);
    address = vla_identity_bits(data);
  }
  return result + inspect_expired((void *)address, n) + finish_vla(n);
}

KEEP unsigned sample_vla_pointer_result(unsigned n) {
  __UINTPTR_TYPE__ address;
  unsigned result;
  {
    unsigned char data[n + 1];
    data[0] = 7;
    result = observe(data, n + 1);
    address = (__UINTPTR_TYPE__)vla_identity(data);
  }
  return result + inspect_expired((void *)address, n) + finish_vla(n);
}

KEEP unsigned sample_vla_numeric_pair(unsigned n) {
  address_bits first, second;
  unsigned result;
  {
    unsigned char data[n + 2];
    data[0] = 7;
    result = observe(data, n + 1);
    first = vla_identity_bits(data);
    second = vla_identity_bits(data + 1);
  }
  result += inspect_expired((void *)(__UINTPTR_TYPE__)first, n);
  result += inspect_expired((void *)(__UINTPTR_TYPE__)second, n + 1);
  return result + finish_vla(n);
}

KEEP unsigned sample_vla_signed_result(unsigned n) {
  int result;
  {
    unsigned char data[n + 1];
    data[0] = 7;
    result = signed_vla_result(data, n);
  }
  return (unsigned)(result < 0) + finish_vla(n);
}

KEEP unsigned sample_vla_signed_branch(unsigned n) {
  int result = -3;
  if (n & 1) {
    unsigned char data[n + 1];
    data[0] = (unsigned char)n;
    result = signed_vla_result(data, n);
  }
  return (unsigned)(result < 0) + finish_vla(n);
}

KEEP unsigned sample_vla_sibling(unsigned n) {
  unsigned result;
  {
    unsigned char first[n + 1];
    first[0] = 7;
    result = observe(first, n + 1);
  }
  {
    unsigned char second[n + 1];
    second[0] = 11;
    result += observe(second, n + 1);
  }
  return result + finish_vla(n);
}

KEEP unsigned sample_vla_sibling_offset(unsigned n) {
  unsigned result;
  {
    unsigned char first[n + 1];
    first[0] = 7;
    result = observe(first, n + 1);
  }
  {
    unsigned char second[n + 2];
    second[0] = 11;
    result += observe(second, n + 2);
  }
  return result + finish_vla(n);
}

