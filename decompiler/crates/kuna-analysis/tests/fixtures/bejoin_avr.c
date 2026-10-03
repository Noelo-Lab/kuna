/* AVR's gcc ABI returns an int in R25:R24 with the HIGH byte in R25, the
 * first register of avr8gcc.cspec's output list: its <join reversesignif>
 * rule consumes the most significant byte first although the target is
 * little-endian. The registers are memory mapped, so R25R24 is also a global.
 * Raw image: .text of clang 14 -O2 --target=avr -mmcu=atmega328p; negate at
 * word 0x0, addk at word 0x4. */
__attribute__((noinline)) int negate(int a) { return -a; }
int addk(int a) { return a + 0x1234; }
