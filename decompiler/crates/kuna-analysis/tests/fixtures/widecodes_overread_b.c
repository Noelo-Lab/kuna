/* The wide arrays that follow widecodes_overread_a.c's lookup table. */
#include <wchar.h>
void sink(const void *p);
const char *name(int k);
static const wchar_t w1[] = L"Pest";
static const wchar_t w2[] = L"aaa";
static const wchar_t w3[] = L"word";
__attribute__((noinline)) void use1(void) { sink(w1); }
__attribute__((noinline)) void use2(void) { sink(w2); }
__attribute__((noinline)) void use3(void) { sink(w3); }
int main(int c, char **v) { use1(); use2(); use3(); sink(name(c)); return 0; }
