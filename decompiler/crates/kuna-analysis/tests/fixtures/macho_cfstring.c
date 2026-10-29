/* macho_cfstring fixture: a linked arm64 Mach-O with a hand-built CFString
   constant in __DATA_CONST,__cfstring, built so kuna's `cfstrings` pass has a
   __cfstring entry whose char pointer resolves to a __cstring literal. */
struct cfstr { const long *isa; long flags; const char *str; long len; };
extern const long __CFConstantStringClassReference[];

__attribute__((section("__DATA_CONST,__cfstring"), used))
static const struct cfstr kGreeting = {
    __CFConstantStringClassReference, 0x7c8, "cfstring hello", 14
};

const void *get_greeting(void) { return &kGreeting; }
int main(void) { return (int)(long)get_greeting(); }
