/* macho_cfstring fixture: a linked arm64 Mach-O with three hand-built CFString
   constants in __DATA_CONST,__cfstring, for kuna's `cfstrings` pass. The second
   string is longer than one 32-byte entry, so the third entry sits right behind
   an entry whose string would not fit in it. */
struct cfstr { const long *isa; long flags; const char *str; long len; };
extern const long __CFConstantStringClassReference[];
#define CF(name, text) __attribute__((section("__DATA_CONST,__cfstring"), used)) \
    static const struct cfstr name = { __CFConstantStringClassReference, 0x7c8, text, sizeof(text) - 1 }
CF(kGreeting, "cfstring hello");
CF(kLong, "a constant string longer than one entry");
CF(kAfter, "after the long one");
const void *get_greeting(void) { return &kGreeting; }
const void *get_long(void) { return &kLong; }
const void *get_after(void) { return &kAfter; }
int main(void) { return (int)(long)get_greeting(); }
