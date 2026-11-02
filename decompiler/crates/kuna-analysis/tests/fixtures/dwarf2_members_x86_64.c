/* DWARF 2 struct-layout fixture.
 *
 * GCC's -gdwarf-2 writes each DW_AT_data_member_location as a single
 * DW_OP_plus_uconst expression rather than a constant: `b` is at +8, and the
 * storage unit of the `lo`/`hi` bitfields at +4. Reading both offsets makes
 * `second` return `p->b` and `high_bits` return `f->hi`.
 *
 * Build (Ubuntu gcc 15.2.0):
 *   gcc -O1 -gdwarf-2 -gno-record-gcc-switches -fdebug-prefix-map=$PWD=. \
 *       -c dwarf2_members_x86_64.c -o dwarf2_members_x86_64.o
 */
struct pair { int a; long b; };
struct flags { int lead; unsigned lo : 3; unsigned hi : 5; };

long second(struct pair *p) { return p->b; }
unsigned high_bits(struct flags *f) { return f->hi; }
