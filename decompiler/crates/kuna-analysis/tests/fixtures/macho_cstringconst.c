/* macho_cstringconst fixture: an arm64 Mach-O that passes string literals
   where nothing types them char *: an argument of a call through an untyped
   function pointer whose other call site passes an integer there, and a
   void * chosen between a literal and what an untyped callee returns. */
extern void (*sink)(void *, void *, void *);
extern void *lookup(void *);
void emit(void *ctx, long n) {
    sink(ctx, " (", 0);
    sink(ctx, (void *)n, 0);
}
void *choose(int level, void *node) {
    void *v = level == 1 ? (void *)".SH" : lookup(node);
    sink(node, v, 0);
    return v;
}
int main(void) { emit(choose(1, 0), 2); return 0; }
