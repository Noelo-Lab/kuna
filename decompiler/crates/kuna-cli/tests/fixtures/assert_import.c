// Fixture for the `import` assertion: calls through pointer slots a run-time
// resolver fills, as a packer's rebuilt import table does.
// clang -O2 -fPIC -fno-inline -fcf-protection=none -c assert_import.c -o assert_import.o
// ld -shared -z noseparate-code -z norelro --build-id=none --hash-style=gnu \
//    -z max-page-size=0x10 -o assert_import.so assert_import.o
// strip -o assert_import_x86_64.elf assert_import.so
// (clang 14, GNU ld 2.38; the slots are at 0x3f8 and 0x400)
extern void *resolve(const char *name);
static int (*slot_puts)(const char *);
static int (*slot_atoi)(const char *);
void bind(void) { slot_puts = resolve("puts"); slot_atoi = resolve("atoi"); }
int greet(const char *s) { slot_puts("hello"); return slot_atoi(s) + 1; }
