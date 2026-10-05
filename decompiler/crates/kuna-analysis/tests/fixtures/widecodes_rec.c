/* L"hellow" inside a struct whose two leading ints, 8 and 12, read as a table
   of 32-bit offsets from the struct's start would point at "h" and "e". */
#include <wchar.h>
const void *volatile g;
__attribute__((noinline)) void sink(const void *p) { g = p; }
struct rec { int first, second; wchar_t name[8]; };
static const struct rec r = { 8, 12, L"hellow" };
__attribute__((noinline)) void userec(void) { sink(&r); }
__attribute__((noinline)) void usename(void) { sink(r.name); }
int main(int c, char **v) { userec(); usename(); return 0; }
