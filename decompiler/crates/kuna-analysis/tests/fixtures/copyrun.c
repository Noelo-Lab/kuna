/* Fixture for `copyrun`: fixed-size copies gcc -O2 expands into SSE moves.
   gcc -O2 -fno-pic -nostdlib -static -Wl,-e,copy_struct
     -Wl,--unresolved-symbols=ignore-all copyrun.c -o copyrun_x86_64 */
struct big { long a[8]; };
struct pair { struct big first, second; };
void copy_struct(struct big *d, const struct big *s) { *d = *s; }
void shift(struct pair *p) { p->second = p->first; }
void copy31(char *d, const char *s) { __builtin_memcpy(d, s, 31); }
void small(long *d, const long *s) { d[0] = s[0]; d[1] = s[1]; }
void use(struct big *b);
void local(const struct big *s) { struct big b = *s; use(&b); }
