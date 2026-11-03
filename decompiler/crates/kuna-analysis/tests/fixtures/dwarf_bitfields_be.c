/* Big-endian DWARF bitfield fixture.
 *
 * On a big-endian target the first bitfield declared takes the most significant
 * bits of its storage unit: `lo` and `hi` share byte 0, `wide` is bytes 1-2 and
 * `b` spans bytes 4-5. GCC's -gdwarf-4 counts DW_AT_bit_offset from the unit's
 * most significant bit; clang's -gdwarf-5 counts DW_AT_data_bit_offset from the
 * most significant bit of the struct's first byte.
 *
 * Build (Ubuntu gcc 15.2.0, Ubuntu clang 21.1.8):
 *   powerpc-linux-gnu-gcc -O1 -gdwarf-4 -gno-record-gcc-switches \
 *       -fdebug-prefix-map=$PWD=. -c dwarf_bitfields_be.c \
 *       -o dwarf_bitfields_ppc_dwarf4.o
 *   clang --target=mips-linux-gnu -O1 -gdwarf-5 -fdebug-prefix-map=$PWD=. \
 *       -c dwarf_bitfields_be.c -o dwarf_bitfields_mips_dwarf5.o
 */
struct bits {
  unsigned lo : 3, hi : 5, wide : 16, top : 8;
  unsigned a : 4, b : 12, c : 7, d : 9;
  int count;
};

unsigned get_lo(struct bits *p) { return p->lo; }
unsigned get_hi(struct bits *p) { return p->hi; }
unsigned get_wide(struct bits *p) { return p->wide; }
unsigned get_b(struct bits *p) { return p->b; }
int get_count(struct bits *p) { return p->count; }
void set_b(struct bits *p, unsigned v) { p->b = v; }
