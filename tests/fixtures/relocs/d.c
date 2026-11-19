/* Source of lld-nosht.elf (tests/stages/kuna-dynrelocs-segment.xml):
 *   zig cc -target x86_64-linux-gnu -O0 -fno-sanitize=all -g0 -fPIE -pie d.c -o lld.elf
 * then e_shoff, e_shnum and e_shstrndx zeroed, as sstrip does.
 */

static int f1(int x) { return x + 11; }
static int f2(int x) { return x * 3; }
static int f3(int x) { return x - 7; }
int (*const tab[])(int) = { f1, f2, f3 };
const char *const names[] = { "one", "two", "three" };
static int impl_a(int x) { return x ^ 0x55; }
static void *resolve_foo(void) { return (void *)impl_a; }
int foo(int) __attribute__((ifunc("resolve_foo")));
int dispatch(int i) { return tab[i](i) + foo(i); }
const char *name_of(int i) { return names[i]; }
const char *second(void) { return names[1]; }
int call_two(int x) { return tab[1](x); }
int main(int argc, char **argv) { return dispatch(argc & 1); }
