extern void source(int, int) __asm__("machine");
extern int release_value(int, int) __asm__("free");
extern void observe_partial(int, int) __asm__("partial");
extern void narrow_double(int, double) __asm__("wider");
extern void narrow_long(int, long long) __asm__("wide_integer");

/* Each retained function has a call-site DIE referencing one declaration. */
void proof_machine(int x, int y) { source(x, y); }
int proof_free(int x, int y) { return release_value(x, y); }
void proof_partial(int x, int y) { observe_partial(x, y); }
void proof_double(int x, double y) { narrow_double(x, y); }
void proof_integer(int x, long long y) { narrow_long(x, y); }
extern void pair(int, int) __asm__("one_version");
void proof_version(int x, int y) { pair(x, y); }
extern void record_float(float) __asm__("strlen");
void proof_float(float x) { record_float(x); }
enum { max_value = 0xffffffffU } value;
extern void record_enum(__typeof__(value)) __asm__("unsigned_arg");
void proof_enum(void) { record_enum(max_value); }
