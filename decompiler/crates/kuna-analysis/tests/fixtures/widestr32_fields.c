/* structs whose int codes[6] end in a zero, followed by a field the code
   names: a negative delta in kd, a string pointer in kp, two negative bounds
   in kj (which read as offsets back into the code).  L"control" is a
   literal. */
#include <stdio.h>
#include <wchar.h>
#ifdef __clang__
#define NI __attribute__((noinline))
#else
#define NI __attribute__((noipa))
#endif
struct KD { int codes[6]; int delta; int extra; };
struct KP { int codes[6]; const char *name; };
struct KJ { int codes[6]; int lo; int hi; };
static const struct KD kd = {{97, 98, 99, 100, 101, 0}, -5, 9};
static const struct KP kp = {{97, 98, 99, 100, 101, 0}, "nm"};
static const struct KJ kj = {{97, 98, 99, 100, 101, 0}, -3840, -3900};
NI int used(const struct KD *p) { return p->codes[1] + p->delta * 100 + p->extra * 7; }
NI int showd(const int *c) { return *c * 3; }
NI int usep(const struct KP *p) { return p->codes[1] + p->name[0]; }
NI int showp(const char *const *c) { return (*c)[1]; }
NI int usej(const struct KJ *p) { return p->codes[1] + p->lo * 100 + p->hi * 7; }
NI long hashw(const wchar_t *w) { long r = 7; for (long i = 0; w[i]; i++) r = r * 31 + w[i]; return r; }
NI long control(void) { return hashw(L"control"); }
int main(void) { printf("%d %d %d %d %ld\n", used(&kd), showd(&kd.delta), usep(&kp), showp(&kp.name), control()); printf("%d %d\n", usej(&kj), showd(&kj.lo)); return 0; }
