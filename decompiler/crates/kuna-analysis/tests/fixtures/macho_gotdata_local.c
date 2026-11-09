/* macho_gotdata_local fixture: an arm64 Mach-O whose read_thrice keeps the
   __got slot of the imported DATA symbol in a callee-saved register (x19) and
   reads through it across three calls, so the slot pointer is a named local. */
extern int imported_counter;
void sink(int);
void read_thrice(void) { sink(imported_counter); sink(imported_counter); sink(imported_counter); }
int main(void) { read_thrice(); return 0; }
