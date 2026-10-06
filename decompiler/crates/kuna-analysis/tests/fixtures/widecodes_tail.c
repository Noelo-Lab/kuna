/* A one-character literal that ends the merged string block at a 4-aligned
   address, followed by an int table: "A" 00 00 then t = {66, 67, 68} reads
   like an array of 4-byte character codes; only t's symbol says otherwise. */
__attribute__((noinline)) void sink(const char *s) { __asm__ volatile("" :: "r"(s) : "memory"); }
__attribute__((noinline)) void fa(void) { sink("A"); }
static const int t[3] = {66, 67, 68};
__attribute__((noinline)) int get(int i) { return t[i]; }
int main(int c, char **v) { fa(); return get(c); }
