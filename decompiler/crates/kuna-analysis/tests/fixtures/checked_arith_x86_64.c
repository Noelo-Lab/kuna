/* `scale` checks its multiply and its add for signed overflow and traps with
   `ud1` when either overflows, so its C calls SCARRY4 and
   invalidInstructionException.

   zig cc -target x86_64-linux-musl -O0 -g0 -fsanitize=signed-integer-overflow -fsanitize-trap=signed-integer-overflow -nostdlib -static -o checked_arith_x86_64 checked_arith_x86_64.c */
__attribute__((noinline)) int scale(int value, int factor) { return value * factor + 1; }
void _start(void) { scale(3, 7); }
