/* (kuna callrettype) A call returns the type its callee's own recovery gave it.
 * Each caller keeps a callee's result at the type the callee returns it as:
 * a char * kept in a char * and handed to a char * parameter, a char * found
 * by strchr and handed back, a long compared signed, a FILE * handed back from
 * a global.  The controls keep a callee whose unsigned result shares a
 * variable with a signed libc result, and a void callee whose caller must not
 * start reading a result.
 * The round-trip test in kuna-cli/tests/decompile_all_cli.rs compiles the
 * functions as kuna prints them, with the option off and on, and checks both
 * print what this program prints:
 *
 *   202 205 2
 *   104 -1
 *   1 0 -2
 *   1 -1 0
 *   7 6 6
 *   10 21 9
 *   5 507 18446744073709551615
 *
 * Built with:
 *   gcc   -O0 -o callrettype_gcc_O0_x86_64   callrettype_x86_64.c
 *   clang -O0 -o callrettype_clang_O0_x86_64 callrettype_x86_64.c
 *   gcc   -O2 -o callrettype_gcc_O2_x86_64   callrettype_x86_64.c
 */
#include <stdio.h>
#include <string.h>
#define NI __attribute__((noinline))

/* a char * result kept in a char * and handed to a char * parameter */
NI char *skip_blanks(char *s) {
  while (*s == ' ')
    s++;
  return s;
}
NI long count_upper(char *s) {
  long n = 0;
  for (; *s; s++)
    n += (*s >= 'A' && *s <= 'Z');
  return n;
}
NI long upper_after_blanks(char *s) {
  char *p = skip_blanks(s);
  return count_upper(p) * 100 + (p - s);
}
NI long upper_of_rest(char *s) { return count_upper(skip_blanks(s)) * 100 + 5; }

/* a void * result kept in a char * */
NI void *first_of(void *p, long n) { return n ? p : NULL; }
NI int first_char(char *s, long n) {
  char *c = first_of(s, n);
  return c ? *c : -1;
}

/* a long result compared signed */
NI long signed_delta(long a, long b) { return a - b; }
NI int is_behind(long a, long b) { return signed_delta(a, b) < 0; }
NI long clamp_delta(long a, long b) {
  long d = signed_delta(a, b);
  return d < -2 ? -2 : d;
}

/* control: an unsigned result sharing a variable with a signed libc result */
NI unsigned int hash_of(char *s) {
  unsigned int h = 5381;
  while (*s)
    h = h * 33 + (unsigned char)*s++;
  return h;
}
NI int pick(char *a, char *b, int mode) {
  int r;
  if (mode)
    r = hash_of(a) % 7;
  else
    r = strcmp(a, b);
  return r < 0 ? -1 : r;
}

/* a pointer found by a declared libc call, handed back and kept */
char *g_fallback = "fallback";
NI char *after_colon(char *s) {
  char *p = strchr(s, ':');
  return p ? p + 1 : s;
}
NI char *fallback_name(void) {
  char *p = strchr(g_fallback, 'l');
  return p ? p : g_fallback;
}
NI long name_len(char *s) {
  char *n = s ? after_colon(s) : fallback_name();
  return (long)strlen(n) + 1;
}

/* a stream handed back from a global */
NI FILE *pick_stream(int e) { return e ? stderr : stdout; }
NI int stream_no(int e) { return fileno(pick_stream(e)) * 10 + e; }

/* control: a pointer result that shares the return register with the numbers
   the function returns, so the merge ties them into one variable */
NI long *table_of(long *c) { return c[1] ? &c[2] : NULL; }
NI unsigned long lookup(long *t, unsigned long k) { return (unsigned long)t[0] + k; }
NI unsigned long cached(long *c, unsigned long k) {
  if (k && k < 100)
    return k;
  if (!c[0]) {
    c[0] = (long)table_of(c);
    if (!c[0])
      return (unsigned long)-1;
  }
  return lookup((long *)c[0], k);
}

/* control: a void callee; its caller reads no result */
NI void mark(char *s) { s[0] = '#'; }
NI long marked_len(char *s) {
  mark(s);
  return (long)strlen(s) + 2;
}

int main(void) {
  char buf[] = "  AbC";
  char buf2[] = "xyzw";
  char buf3[] = "Hi!Hi!!";
  printf("%ld %ld %ld\n", upper_after_blanks(buf), upper_of_rest(buf), (long)(skip_blanks(buf) - buf));
  printf("%d %d\n", first_char(buf2 + 1, 3) - 17, first_char(buf2, 0));
  printf("%d %d %ld\n", is_behind(3, 9), is_behind(9, 3), clamp_delta(1, 9));
  printf("%d %d %d\n", pick("ab", "cd", 1), pick("ab", "cd", 0), pick("ab", "ab", 0));
  printf("%ld %ld %ld\n", name_len(NULL), name_len("key:value"), name_len("plain"));
  printf("%d %d %ld\n", stream_no(0), stream_no(1), marked_len(buf3));
  long c1[4] = {0, 1, 7, 0}, c2[4] = {0, 0, 0, 0};
  printf("%lu %lu %lu\n", cached(c1, 5), cached(c1, 500), cached(c2, 500));
  return 0;
}
