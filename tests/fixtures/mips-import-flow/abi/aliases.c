/* These source names resolve to different versioned symbols at link time. */
extern void stale(int, int) __asm__("source");
__asm__(".symver source,notify@VER_1");
void proof_alias(int x, int y) { stale(x, y); }

extern int machine(int, int);
__asm__(".symver machine,thing@VER_1");
int proof_source_only(int x, int y) { return machine(x, y); }

/* The linker drops this function but keeps its declaration in .debug_info. */
extern void discarded(int) __asm__("orphan");
static __attribute__((used)) void unused(int x) { discarded(x); }
