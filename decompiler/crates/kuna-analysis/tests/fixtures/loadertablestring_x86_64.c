/* A pointer constant that lands inside the dynamic loader's own tables.
 *
 * `0x4a3` is an offset into this PIE's `.dynstr`, three bytes into the
 * `"puts"` entry, so the bytes there are the NUL-terminated run `uts`.  The
 * readonly-range scan painted `.dynstr` read-only because it is allocated and
 * not writable, which is all `PrintC::pushPtrCharConstant` asks before it
 * replaces a constant with the characters at it, and the call printed
 * `puts("uts")`.
 *
 * Build (the address is an offset in THIS link; keep the committed binary):
 *   gcc -O0 -fPIE -pie -o loadertablestring_x86_64 loadertablestring_x86_64.c
 */
#include <stdio.h>

int main(void)
{
  puts((char *)0x4a3);
  return 0;
}
