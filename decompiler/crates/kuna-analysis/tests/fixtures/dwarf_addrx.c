/* A DWARF 5 producer names a static global through DW_OP_addrx, an index into
   .debug_addr, where DWARF 4 writes the address itself in DW_OP_addr.
   clang 22.1 --target=<triple> -O1 -gdwarf-5 -ffreestanding -fno-pic -nostdlib
   -no-pie -fdebug-compilation-dir=. -fuse-ld=lld -Wl,--no-dynamic-linker -Wl,--discard-all
   for x86_64-linux-gnu (the image also has -fcf-protection=full's endbr64s)
   and aarch64_be-linux-gnu. */
static int counter;
static int table[4] = {1, 2, 3, 4};
int bump(int k) { counter += table[k & 3]; table[k & 3] = counter; return counter; }
void _start(void) { for (;;) bump(1); }
