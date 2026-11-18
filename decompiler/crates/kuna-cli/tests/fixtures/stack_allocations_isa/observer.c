#include <stdarg.h>
#include "observer.h"

unsigned observed_calls, observed_digest;
static __UINTPTR_TYPE__ last_observed_address;
__attribute__((noinline)) unsigned observe(unsigned char *p, unsigned n) {
  last_observed_address = (__UINTPTR_TYPE__)p;
  observed_calls++;
  observed_digest = observed_digest * 1664525u + n * 31u + p[0];
  volatile unsigned char clobber[256];
  for (unsigned i = 0; i < 256; i++)
    clobber[i] = (unsigned char)i;
  p[n - 1] = 9;
  return p[0] + p[n - 1] + clobber[7] - 7;
}
unsigned observe_aligned(unsigned char *p, unsigned n) {
  return observe(p, n) + ((__UINTPTR_TYPE__)p & 63);
}
unsigned observe_many(unsigned char *p, unsigned n, unsigned a, unsigned b,
                      unsigned c, unsigned d, unsigned e, unsigned f,
                      unsigned g, unsigned h, unsigned i, unsigned j) {
  return observe(p, n) + a + 2 * b + 3 * c + 4 * d + 5 * e + 6 * f + 7 * g +
         8 * h + 9 * i + 10 * j;
}

unsigned char *escaped_vla_address;

__attribute__((noinline)) unsigned finish_vla(unsigned n) {
  volatile unsigned char scratch[1024];
  for (unsigned i = 0; i < sizeof(scratch); i++)
    scratch[i] = (unsigned char)(n + i);
  observed_calls++;
  observed_digest = observed_digest * 33u + n;
  return scratch[n & 1023];
}

__attribute__((noinline)) unsigned vla_variadic(unsigned count, unsigned a,
    unsigned b, unsigned c, unsigned d, unsigned e, unsigned f, unsigned g,
    unsigned h, ...) {
  va_list arguments;
  va_start(arguments, h);
  unsigned result = a + 2 * b + 3 * c + 4 * d + 5 * e + 6 * f + 7 * g + 8 * h;
  for (unsigned i = 0; i < count; i++)
    result += va_arg(arguments, unsigned) * (i + 9);
  va_end(arguments);
  observed_calls++;
  observed_digest = observed_digest * 33u + result;
  return result;
}

__attribute__((noinline)) unsigned vla_register_variadic(unsigned count, ...) {
  va_list arguments;
  va_start(arguments, count);
  unsigned result = 0;
  for (unsigned i = 0; i < count; i++)
    result += va_arg(arguments, unsigned) * (i + 3);
  va_end(arguments);
  observed_calls++;
  observed_digest = observed_digest * 33u + result;
  return result;
}

__attribute__((noinline)) void mutate_delimiter(void *storage) {
  unsigned char *delimiter = storage;
  delimiter[0] ^= 7;
  observed_calls++;
  observed_digest = observed_digest * 33u + delimiter[0];
}

__attribute__((noinline)) unsigned escape_vla(unsigned char *data, unsigned n) {
  return observe(data, n) + (escaped_vla_address == data);
}

__attribute__((noinline)) unsigned inspect_expired(void *address, unsigned n) {
  __UINTPTR_TYPE__ bits = (__UINTPTR_TYPE__)address;
  unsigned result = n + (unsigned)(bits & 3) + (bits == last_observed_address ? 31u : 97u);
  observed_calls++;
  observed_digest = observed_digest * 33u + result;
  return result;
}

__attribute__((noinline)) void *vla_identity(void *address) {
  observed_calls++;
  observed_digest = observed_digest * 33u + 17u;
  return address;
}

__attribute__((noinline)) address_bits vla_identity_bits(void *address) {
  observed_calls++;
  observed_digest = observed_digest * 33u + 17u;
  return (address_bits)(__UINTPTR_TYPE__)address;
}

__attribute__((noinline)) int signed_vla_result(unsigned char *data, unsigned n) {
  observed_calls++;
  observed_digest = observed_digest * 33u + 19u;
  return data[0] == 7 ? (int)(n & 7u) - 4 : -127;
}
