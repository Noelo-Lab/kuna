/* macho_gotdata fixture: an arm64 Mach-O that reads two imported DATA
   symbols through __got, so a data read goes through a slot that holds the
   import's address (not a call). */
extern int imported_counter;
extern const char *imported_name;

int read_counter(void) { return imported_counter; }
const char *read_name(void) { return imported_name; }
int main(void) { return read_counter() + (read_name() != 0); }
