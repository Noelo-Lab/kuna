/* Fixture for the call-site stack placeholder strip.
 *
 * `ActionFuncLink` hangs a stack-pointer placeholder on every call so the site's
 * stack offset can be read out of the data flow. An alloca'd buffer means the
 * stack pointer at these call sites is not a constant offset from the frame
 * base, so that recovery never resolves; both callees carry a declared
 * prototype, so nothing later rewrites the call's input list either. Until the
 * placeholder is stripped when the stack space is heritaged, it renders as one
 * argument past the declared arity, reading the slot the `call` pushed its
 * return address into.
 *
 * gcc -O0 -g0 -fno-stack-protector -o callplaceholder_x86_64 callplaceholder_x86_64.c
 */
#include <string.h>

int probe (const char *name, int n)
{
  char *buf = __builtin_alloca (n);
  strncpy (buf, name, n - 1);
  buf[n - 1] = '\0';
  return (int) strlen (buf);
}

int main (int argc, char **argv)
{
  return argc > 2 ? probe (argv[1], argc) : 0;
}
