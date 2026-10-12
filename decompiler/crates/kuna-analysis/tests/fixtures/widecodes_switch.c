/* A one-character literal whose neighbour in the merged string section only
   a switch's lookup table reaches.  clang -O2 lays the table out as 32-bit
   offsets from its own start (.long .str - table), so no pointer to "b" is
   stored anywhere. */
const char *volatile g_tag;
__attribute__((noinline)) void settag(const char *t) { g_tag = t; }
__attribute__((noinline)) const char *name(int k) {
  switch (k) { case 0: return "a"; case 1: return "b"; case 2: return "c"; case 3: return "d"; case 4: return "f"; default: return "e"; } }
__attribute__((noinline)) void usea(void) { settag("a"); }
__attribute__((noinline)) void usee(void) { settag("e"); }
int main(int c, char **v) { usea(); usee(); settag(name(c)); return 0; }
