/* Pointer constants that land in sections the toolchain and the run-time
 * loader own.
 *
 * In this PIE, `0x4a3` is an offset into `.dynstr`, three bytes into the
 * `"puts"` entry, so the bytes there are the NUL-terminated run `uts`;
 * `0x320` is eight bytes into `.interp` (`/lib64/ld-linux-x86-64.so.2` at
 * `0x318`); and `0x3a0` is inside `.note.ABI-tag`.  The readonly-range scan
 * painted all three read-only because they are allocated and not writable,
 * which is all `PrintC::pushPtrCharConstant` asks before it replaces a
 * constant with the characters at it, so the calls printed `puts("uts")`,
 * `puts("d-linux-x86-64.so.2")` and `puts("\x03")`.
 *
 * Build (the addresses are offsets in THIS link; keep the committed binary):
 *   gcc -O0 -fPIE -pie -o loadertablestring_x86_64 loadertablestring_x86_64.c
 */
#include <stdio.h>

int main(void)
{
  puts((char *)0x4a3);
  puts((char *)0x320);
  puts((char *)0x3a0);
  return 0;
}
