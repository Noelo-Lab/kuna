/* DWARF 4 static data member fixture.
 *
 * Before DWARF 5, a C++ static data member is a DW_TAG_member marked
 * DW_AT_external and DW_AT_declaration, with no DW_AT_data_member_location.
 * It is a global rather than part of the layout, so `first_of` reads
 * `c->first`, not `c->instances` at offset 0.
 *
 * Build (Ubuntu g++ 15.2.0):
 *   g++ -O1 -gdwarf-4 -gno-record-gcc-switches -fdebug-prefix-map=$PWD=. \
 *       -c dwarf_static_member_x86_64.cpp -o dwarf_static_member_x86_64.o
 */
struct Counter { static int instances; int first; int second; };
int Counter::instances;

extern "C" int first_of(Counter *c) { return c->first; }
