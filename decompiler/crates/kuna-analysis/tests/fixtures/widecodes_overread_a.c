/* With widecodes_overread_b.c: clang -O2 lays name's lookup table of 32-bit
   offsets right before b's wide arrays, so reading past its seven entries
   takes L"Pest"'s 'P' (0x50) for an offset that lands on L"word"[1]. */
const void *volatile g;
__attribute__((noinline)) void sink(const void *p) { g = p; }
__attribute__((noinline)) const char *name(int k) {
  switch (k) { case 0: return "a"; case 1: return "b"; case 2: return "c"; case 3: return "d"; case 4: return "f"; case 5: return "g"; case 6: return "h"; default: return "e"; } }
