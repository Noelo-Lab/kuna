/* (kuna castsign) Variables the program only ever reads signed.  Each of the
 * first six functions keeps a length or an index in a variable whose value
 * comes from somewhere unsigned (strlen's size_t, an unsigned char table) and
 * then compares it signed; kuna declares it unsigned and prints `(long)v` at
 * every comparison unless `castsign` re-declares it.  The last two must stay
 * unsigned: one compares the value both ways, one shifts it logically.  The
 * round-trip test in kuna-cli/tests/decompile_all_cli.rs compiles the functions
 * as kuna prints them, with the option off and on, and checks both print what
 * this program prints:
 *
 *   -7 23 13
 *   2 7
 *   2 125
 *   -3 7 0
 *   -1 3
 *   6 9 3 0 -1
 *
 * Built with:
 *   gcc   -O0 -o castsign_gcc_O0_x86_64   castsign_x86_64.c
 *   clang -O0 -o castsign_clang_O0_x86_64 castsign_x86_64.c
 *   gcc   -O1 -o castsign_gcc_O1_x86_64   castsign_x86_64.c
 *   clang -O1 -o castsign_clang_O1_x86_64 castsign_x86_64.c
 */
#include <stdio.h>
#include <string.h>

#ifdef __clang__
#define KEEP __attribute__((noinline))
#else
#define KEEP __attribute__((noinline, noipa))
#endif

/* A trailing-space trim: the index runs below zero when the string is all
   spaces. */
KEEP long trim_right(char *s) {
  long i = strlen(s) - 1;
  while (i >= 0 && s[i] == ' ') {
    s[i] = 0;
    i--;
  }
  return i * 10 + 3;
}

/* A countdown over an int array, indexed by the counter. */
KEEP long count_down(const int *a, const char *s) {
  long k = strlen(s);
  long hits = 0;
  for (k = k - 1; k >= 0; k--)
    if (a[k] < 0)
      hits += k;
  return hits;
}

/* A run length read out of an unsigned char table and walked signed. */
KEEP long run_len(const unsigned char *tab, const char *s, long lim) {
  long len = tab[(unsigned char)s[0]];
  long i;
  for (i = 0; s[i]; i++) {
    if (len >= lim)
      len = tab[(unsigned char)s[i]];
    else
      len = len + s[len];
  }
  if (len < 0)
    return -len;
  return len;
}

/* An offset from an unsigned start, compared against zero. */
KEEP long word_end(const char *s, unsigned int start) {
  long k = start;
  while (s[k] != ' ' && s[k])
    k++;
  k = k - (long)strlen(s);
  if (k < 0)
    return k;
  return 0;
}

/* A length taken from an unsigned int table or from strlen, trimmed signed. */
KEEP long pick_len(const unsigned int *lens, const char *s, int n) {
  long len;
  if (n > 0)
    len = lens[n];
  else
    len = strlen(s);
  while (len > 0 && s[len - 1] == ' ')
    len--;
  return len * 3;
}

/* A position read out of an unsigned int and walked down signed. */
KEEP long zext_walk(const unsigned int *u, const char *s) {
  long k = *u;
  while (k > 0 && s[k] != 'q')
    k--;
  return k - 1;
}

/* Must stay unsigned: compared both ways. */
KEEP long both_ways(const char *s, unsigned long lim) {
  unsigned long n = strlen(s) - 3;
  if ((long)n < 0)
    return -2;
  if (n < lim)
    return (long)n;
  return (long)lim;
}

/* Must stay unsigned: shifted logically. */
KEEP long halved(const char *s) {
  unsigned long n = strlen(s) - 2;
  if ((long)n < 0)
    return -1;
  return (long)(n >> 1);
}

int main(void) {
  char a[] = "   ", b[] = "abc  ", c[] = "xy ";
  static const int arr[] = {-4, 5, -6, 7, 0, -1};
  static unsigned char tab[256];
  for (int i = 0; i < 256; i++)
    tab[i] = (unsigned char)(i % 7);
  printf("%ld %ld %ld\n", trim_right(a), trim_right(b), trim_right(c));
  printf("%ld %ld\n", count_down(arr, "abcd"), count_down(arr, "abcdef"));
  printf("%ld %ld\n", run_len(tab, "hello world", 3), run_len(tab, "zzzzzzz", 100));
  printf("%ld %ld %ld\n", word_end("ab cd", 0), both_ways("abcdefghij", 99), both_ways("abc", 1));
  printf("%ld %ld\n", halved("a"), halved("abcdefgh"));
  static const unsigned int lens[] = {0, 4, 9, 2};
  static const unsigned int at[] = {6, 0};
  printf("%ld %ld %ld %ld %ld\n", pick_len(lens, "ab  ", 0), pick_len(lens, "abc ", 1),
         pick_len(lens, "a ", 3), zext_walk(at, "aqbbbbbb"), zext_walk(at + 1, "q"));
  return 0;
}
