/* Source of riscv64.o, x64pic.o and a64pic.o (tests/stages/kuna-reloc-families.xml):
 *   zig cc -target riscv64-freestanding-none -fno-pic -mcmodel=medlow -mno-relax -O1 -g0 -c t.c -o riscv64.o
 *   zig cc -target x86_64-linux-gnu -fpic -O1 -g0 -c t.c -o x64pic.o
 *   zig cc -target aarch64-linux-gnu -fpic -O1 -g0 -c t.c -o a64pic.o
 */

extern int ext_fn(int);
extern int ext_tab[8];
int counter;
static int local_tab[4] = {1,2,3,4};
int pick(int x) {
    if (x < 0) return ext_fn(-x);
    counter += x;
    return local_tab[x & 3] + ext_tab[x & 7];
}
int wide(void) { return (int)(long)&ext_tab; }
