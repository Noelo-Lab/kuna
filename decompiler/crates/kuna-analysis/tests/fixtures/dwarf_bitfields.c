/* DWARF bitfield fixture.
 *
 * The first bitfield declared takes the least significant bits of its storage
 * unit on a little-endian target and the most significant on a big-endian one.
 * On big-endian, `lo` and `hi` share byte 0, `wide` is bytes 1-2 and `b` spans
 * bytes 4-5. In `struct spill`, `c` and `d` end in the same byte in either byte
 * order. GCC's -gdwarf-4 counts DW_AT_bit_offset from the storage unit's most
 * significant bit; clang's -gdwarf-5 counts DW_AT_data_bit_offset from the
 * start of the struct in the target's bit order.
 *
 * Build (Ubuntu gcc 15.2.0, Ubuntu clang 21.1.8):
 *   powerpc-linux-gnu-gcc -O1 -gdwarf-4 -gno-record-gcc-switches \
 *       -fdebug-prefix-map=$PWD=. -c dwarf_bitfields.c \
 *       -o dwarf_bitfields_ppc_dwarf4.o
 *   clang --target=mips-linux-gnu -O1 -gdwarf-5 -fdebug-prefix-map=$PWD=. \
 *       -c dwarf_bitfields.c -o dwarf_bitfields_mips_dwarf5.o
 *   gcc -O1 -gdwarf-4 -gno-record-gcc-switches -fdebug-prefix-map=$PWD=. \
 *       -c dwarf_bitfields.c -o dwarf_bitfields_x86_64.o
 */
struct bits {
  unsigned lo : 3, hi : 5, wide : 16, top : 8;
  unsigned a : 4, b : 12, c : 7, d : 9;
  int count;
};
struct spill { unsigned a : 6, c : 4, d : 6; };

unsigned get_lo(struct bits *p) { return p->lo; }
unsigned get_hi(struct bits *p) { return p->hi; }
unsigned get_wide(struct bits *p) { return p->wide; }
unsigned get_b(struct bits *p) { return p->b; }
int get_count(struct bits *p) { return p->count; }
void set_b(struct bits *p, unsigned v) { p->b = v; }
unsigned get_c(struct spill *s) { return s->c; }
