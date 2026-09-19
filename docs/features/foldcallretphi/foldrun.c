/* Compile-and-run check for the foldcallretphi default.
 *
 * Every rendering below is transcribed verbatim from `kuna decompile-all`
 * (stripped binaries of decbench full_run_address_2026-09-11, and the repo's
 * tests/bug-repro/faillog), once as `--option foldcallretphi off` prints it and
 * once as the default prints it.  Every callee is a stub that logs its call
 * (with arguments) to a trace and reads or writes the globals the site touches,
 * because a call may write any global.  A pair must agree on the trace, the
 * return value and every global afterwards.
 *
 * Folded sites (the default removes the spill):
 *   A  sort -O2 main        strcmp past a register self-op, reading optarg
 *   C  faillog main         !fclose(dat_6298) inside &&, then exit(dat_6276)
 *   D  faillog main         fstat(fileno(dat_6298),..)
 *   E  du -O0 sub_11107     while (getline(..) != -1), an 8-byte ssize_t -1
 * Narrowed sites (the default keeps the spill, so it prints what off prints):
 *   B  sort -O2 main        v15 = sub_c5b0(stdin), v15 == -1
 *   F  pr -O2-noinline main v4 = sub_60c0(stdin), v4 == -1
 * Negative controls (each must MISMATCH, so the check can see the defect):
 *   B', F'  the fold the default used to print there, `sub_c5b0(stdin) == -1`,
 *           against a stub that returns 0xffffffff as the binary's
 *           `mov $0xffffffff,%eax` and kuna's own `return 0xffffffff;` do;
 *   K       a call folded past a global write (stages target2).
 * Short-circuit and joined-return sites (review round 3):
 *   G  cx7 z1 -O2         ferror(stdin) folded into the LEFT operand of ||
 *   G  cx7 z3 -O2         default keeps the fflush spill feeding the right of &&
 *   G' (control)          the old fold 5 < a0 && !fflush(stdout) skips the call
 *   H  stages join i386   default keeps v1 = big(g), v1 != -1
 *   H' (control)          the old fold big(g) != -1 against 0xffffffff
 */
#include <stdio.h>
#include <string.h>
#include <stdbool.h>
#include <stdlib.h>
#undef stdin

typedef struct { int pos; int err; } FAKEFILE;
static FAKEFILE file_a = {0, 0}, file_b = {100, 0};
static FAKEFILE *g_stdin = &file_a;
static char trace[4096];
static int tlen;
#define LOG(...) (tlen += snprintf(trace + tlen, sizeof trace - tlen, __VA_ARGS__))
static FAKEFILE *other(FAKEFILE *f) { return f == &file_a ? &file_b : &file_a; }

/* ---- site A: coreutils sort -O2 main @0x3ec0, case 0x85 */
static char *g_optarg;
static char opt1[] = "x", opt2[] = "y";
static int s_strcmp(const char *a, const char *b) {
    LOG("strcmp(%s,%s) ", a, b);
    int r = strcmp(a, b);
    g_optarg = (g_optarg == opt1) ? opt2 : opt1;   /* the call writes the global it reads */
    return r;
}
static void s_error(int st, int e, const char *m) { LOG("error(%d,%d,%s) ", st, e, m); }
static const char *s_dcgettext(void *d, const char *m, int c) { (void)d; (void)c; LOG("dcgettext(%s) ", m); return m; }
#define optarg g_optarg
#define strcmp s_strcmp
#define error s_error
#define dcgettext s_dcgettext
static long siteA_off(char *v98, unsigned long *v31o, char **v33o) {
    int v15;
    unsigned long v31 = *v31o;
    char *v33;
        v33 = optarg;
        if (v98) {
          v15 = strcmp(v98,optarg);
          v31 &= 0xffffffff;
          if (v15) {
            error(2,0,dcgettext(NULL,"multiple random sources specified",5));
            goto label_699a;
          }
        }
    *v31o = v31; *v33o = v33; return 0;
label_699a:
    *v31o = v31; *v33o = v33; return 1;
}
static long siteA_on(char *v98, unsigned long *v31o, char **v33o) {
    unsigned long v31 = *v31o;
    char *v33;
        v33 = optarg;
        if (v98) {
          v31 &= 0xffffffff;
          if (strcmp(v98,optarg)) {
            error(2,0,dcgettext(NULL,"multiple random sources specified",5));
            goto label_699a;
          }
        }
    *v31o = v31; *v33o = v33; return 0;
label_699a:
    *v31o = v31; *v33o = v33; return 1;
}
#undef optarg
#undef strcmp

/* ---- sites B / B': coreutils sort -O2 main, label_55b4 */
static char dat_1d418;
static unsigned long ret_c5b0;
static unsigned long sub_c5b0(FAKEFILE *a0) { LOG("sub_c5b0(pos=%d) ", a0->pos); a0->err ^= 1; return ret_c5b0; }
static void sub_7480(const char *a0, const char *a1) { LOG("sub_7480(%s,%s) ", a0, a1); }
static void siteB_off(void) {
  int v15;
  #define stdin g_stdin
  if ((dat_1d418) && (v15 = sub_c5b0(stdin), v15 == -1))
    sub_7480(dcgettext(NULL,"close failed",5),"-"); // no-return
  #undef stdin
}
static void siteB_default(void) {   /* the default now prints exactly this */
  int v15;
  #define stdin g_stdin
  if ((dat_1d418) && (v15 = sub_c5b0(stdin), v15 == -1))
    sub_7480(dcgettext(NULL,"close failed",5),"-"); // no-return
  #undef stdin
}
static void siteB_oldfold(void) {   /* control: what the default printed before the width guard */
  #define stdin g_stdin
  if ((dat_1d418) && (sub_c5b0(stdin) == -1))
    sub_7480(dcgettext(NULL,"close failed",5),"-"); // no-return
  #undef stdin
}

/* ---- sites F / F': coreutils pr -O2-noinline main, the close-stdin check */
static char dat_151f6;
static unsigned int dat_151d0;
static unsigned long ret_60c0;
static int g_errno;
static unsigned long sub_60c0(FAKEFILE *a0) { LOG("sub_60c0(pos=%d) ", a0->pos); g_errno = 9; return ret_60c0; }
static int *s_errno_location(void) { return &g_errno; }
#define __errno_location s_errno_location
static unsigned int siteF_off(int v5) {
  int v4;
  char *v9;
  #define stdin g_stdin
  if ((dat_151f6) && (v4 = sub_60c0(stdin), v4 == -1)) {
    v9 = (char *)dcgettext(NULL,"standard input",5);
    error(1,*__errno_location(),v9);
    return v5;
  }
  return (unsigned int)dat_151d0;
  #undef stdin
}
static unsigned int siteF_default(int v5) {
  int v4;
  char *v9;
  #define stdin g_stdin
  if ((dat_151f6) && (v4 = sub_60c0(stdin), v4 == -1)) {
    v9 = (char *)dcgettext(NULL,"standard input",5);
    error(1,*__errno_location(),v9);
    return v5;
  }
  return (unsigned int)dat_151d0;
  #undef stdin
}
static unsigned int siteF_oldfold(int v4) {
  char *v8;
  #define stdin g_stdin
  if ((dat_151f6) && (sub_60c0(stdin) == -1)) {
    v8 = (char *)dcgettext(NULL,"standard input",5);
    error(1,*__errno_location(),v8);
    return v4;
  }
  return (unsigned int)dat_151d0;
  #undef stdin
}

/* ---- sites C / D: tests/bug-repro/faillog main @0x2620 */
static FAKEFILE *dat_6298;
static unsigned char dat_6276;
static int ret_fsync, ret_fclose, ret_fstat, ret_ferror, ret_fflush;
static int s_ferror(FAKEFILE *f) { LOG("ferror(%d) ", f->pos); return ret_ferror; }
static int s_fflush(FAKEFILE *f) { LOG("fflush(%d) ", f->pos); return ret_fflush; }
static int s_fileno(FAKEFILE *f) { LOG("fileno(%d) ", f->pos); dat_6298 = other(dat_6298); return f->pos + 3; }
static int s_fsync(int fd) { LOG("fsync(%d) ", fd); return ret_fsync; }
static int s_fclose(FAKEFILE *f) { LOG("fclose(%d) ", f->pos); dat_6276 += 7; dat_6298 = other(dat_6298); return ret_fclose; }
static int s_fstat(int fd, void *st) { LOG("fstat(%d,%p) ", fd, st); dat_6298 = other(dat_6298); return ret_fstat; }
#define ferror s_ferror
#define fflush s_fflush
#define fileno s_fileno
#define fsync s_fsync
#define fclose s_fclose
#define fstat s_fstat
#define exit(x) do { LOG("exit(%u) ", (unsigned int)(x)); return 1; } while (0)
typedef struct stat stat;
static long siteC_off(FAKEFILE *v13) {
  int v5;
    if ((!ferror(v13)) && (!fflush(v13))) {
      v5 = fileno(dat_6298);
      if ((!fsync(v5)) && (v5 = fclose(dat_6298), !v5)) {
        exit((unsigned int)dat_6276);
      }
    }
  return 0;
}
static long siteC_on(FAKEFILE *v13) {
  int v5;
    if ((!ferror(v13)) && (!fflush(v13))) {
      v5 = fileno(dat_6298);
      if ((!fsync(v5)) && (!fclose(dat_6298))) {
        exit((unsigned int)dat_6276);
      }
    }
  return 0;
}
static long siteD_off(unsigned long v10, unsigned long *v2o) {
  int v4;
  unsigned long v2;
    v4 = fileno(dat_6298);
    v4 = fstat(v4,(stat *)0x61e0);
    v2 = v10;
    if (v4) {
      *v2o = v2; return 1;
    }
  *v2o = v2; return 0;
}
static long siteD_on(unsigned long v10, unsigned long *v2o) {
  int v4;
  unsigned long v2;
    v4 = fstat(fileno(dat_6298),(stat *)0x61e0);
    v2 = v10;
    if (v4) {
      *v2o = v2; return 1;
    }
  *v2o = v2; return 0;
}
#undef exit

/* ---- site E: coreutils du -O0 sub_11107, the read loop's condition (body reduced to a counter) */
static long ret_getline[4];
static int ngetline;
static long s_getline(char **p, unsigned long *n, FAKEFILE *f) {
    LOG("getline(%d) ", f->pos); f->pos++; (void)p; (void)n;
    return ret_getline[ngetline++ & 3];
}
#define getline s_getline
static long siteE_off(FAKEFILE *v19) {
  long v6;
  char *v13 = NULL;
  unsigned long v12 = 0;
  long iters = 0;
    while (v6 = getline(&v13,&v12,v19), v6 != -1) {
      if (++iters > 8) break;
    }
  return iters;
}
static long siteE_on(FAKEFILE *v19) {
  char *v13 = NULL;
  unsigned long v12 = 0;
  long iters = 0;
    while (getline(&v13,&v12,v19) != -1) {
      if (++iters > 8) break;
    }
  return iters;
}

/* ---- negative control K: tests/stages/kuna-foldcallretphi.xml `target2` */
static int k, g, ok;
static int helper(int x) { LOG("helper(%d,k=%d) ", x, k); return k; }
static void ctl_good(void) { int v1 = ok, v2; v2 = helper(g); k = 42; ok = v1 & v2; }
static void ctl_bad(void)  { int v1 = ok; k = 42; ok = v1 & helper(g); }

/* ---- sites G / G': short-circuit operands (.scratch cx7.c at gcc -O2, x86-64).
 *   z1: int r = ferror(stdin); gflag = (a > 5) || (r != 0);
 *   z3: int r = fflush(stdout); gflag = (r == 0) && (a > 5);
 * The binary makes the call before either comparison.  The default folds z1's
 * call into the LEFT operand of || (always evaluated) and keeps z3's spill,
 * whose value is the RIGHT operand of &&.  G' is the fold the default printed
 * for z3 before the short-circuit guard, and it must not agree. */
static int gflag;
#define stdin g_stdin
#undef stdout
#define stdout g_stdin
static int siteG1_off(int a0) { int v1; v1 = ferror(stdin); gflag = (unsigned int)(v1 || 5 < a0); return gflag; }
static int siteG1_on(int a0) { gflag = (unsigned int)(ferror(stdin) || 5 < a0); return gflag; }
static int siteG3_off(int a0) { int v1; v1 = fflush(stdout); gflag = (unsigned int)(5 < a0 && !v1); return gflag; }
static int siteG3_default(int a0) { int v1; v1 = fflush(stdout); gflag = (unsigned int)(5 < a0 && !v1); return gflag; }
static int siteG3_oldfold(int a0) { gflag = (unsigned int)(5 < a0 && !fflush(stdout)); return gflag; }
#undef stdin
#undef stdout

/* ---- sites H / H': i386 edx:eax (tests/stages/kuna-foldcallretphi-join.xml).
 * kuna prints the callee `unsigned long long big(int4 a0)` ending in
 * `return 0xffffffff;`; the caller reads eax alone.  The default keeps the
 * spill; H' is the fold it printed before the joined-return probe. */
static unsigned long long s_big(FAKEFILE *f) { LOG("big(%d) ", f ? f->pos : -1); return f ? 0xffffffffULL : 1; }
static FAKEFILE *dat_1050;
static int siteH_off(void) { int v1; v1 = s_big(dat_1050); if (v1 != -1) return 7; return 1; }
static int siteH_default(void) { int v1; v1 = s_big(dat_1050); if (v1 != -1) return 7; return 1; }
static int siteH_oldfold(void) { if (s_big(dat_1050) != -1) return 7; return 1; }

static void snapshot(char *out, long r) {
    snprintf(out, 8192,
             "ret=%ld|%s|stdin=%s a=%d/%d b=%d/%d|optarg=%s|d418=%d 151f6=%d errno=%d|6298=%s 6276=%u|k=%d g=%d ok=%d|",
             r, trace, g_stdin == &file_a ? "a" : "b", file_a.pos, file_a.err, file_b.pos, file_b.err,
             g_optarg ? g_optarg : "-", dat_1d418, dat_151f6, g_errno,
             dat_6298 == &file_a ? "a" : "b", dat_6276, k, g, ok);
}
static void reset(void) {
    tlen = 0; trace[0] = 0; g_stdin = &file_a;
    file_a = (FAKEFILE){0, 0}; file_b = (FAKEFILE){100, 0};
    g_optarg = opt1; dat_1d418 = 0; dat_151f6 = 0; dat_151d0 = 3; g_errno = 0;
    dat_6298 = &file_a; dat_6276 = 1; ngetline = 0;
    k = 5; g = 3; ok = 0xff;
}
static int cases, fails;
static void check(const char *name, int expect_equal, long (*run)(int arm, int input), int input) {
    static char a[8192], b[8192];
    reset(); long ra = run(0, input); snapshot(a, ra);
    reset(); long rb = run(1, input); snapshot(b, rb);
    int eq = strcmp(a, b) == 0;
    cases++;
    if (getenv("FOLDRUN_V")) printf("  %s #%d off: %s\n", name, input, a);
    if (eq != expect_equal) { fails++; printf("FAIL %s input=%d\n  off: %s\n  2nd: %s\n", name, input, a, b); }
    else if (!eq) printf("ok   %s input=%d (control: mismatch detected as expected)\n  off: %s\n  2nd: %s\n", name, input, a, b);
}
#undef strcmp
static long runA(int arm, int in) {
    static char s1[] = "x", s2[] = "zz";
    char *v98 = (in & 1) ? ((in & 2) ? s1 : s2) : NULL;
    unsigned long v31 = 0x1234567890abcdefUL; char *v33;
    long r = arm ? siteA_on(v98, &v31, &v33) : siteA_off(v98, &v31, &v33);
    return r * 1000 + (long)(v31 & 0xffff) + (v33 == opt1 ? 1 : v33 == opt2 ? 2 : 0) * 100000;
}
static unsigned long wide_minus1(int in) { return (in & 2) ? 0xffffffffUL : 0; }
static long runB(int arm, int in) {
    dat_1d418 = in & 1; ret_c5b0 = wide_minus1(in);
    if (arm) siteB_default(); else siteB_off();
    return 0;
}
static long runBctl(int arm, int in) {
    dat_1d418 = in & 1; ret_c5b0 = wide_minus1(in);
    if (arm) siteB_oldfold(); else siteB_off();
    return 0;
}
static long runF(int arm, int in) {
    dat_151f6 = in & 1; ret_60c0 = wide_minus1(in);
    return arm ? siteF_default(0x55) : siteF_off(0x55);
}
static long runFctl(int arm, int in) {
    dat_151f6 = in & 1; ret_60c0 = wide_minus1(in);
    return arm ? siteF_oldfold(0x55) : siteF_off(0x55);
}
static long runC(int arm, int in) {
    ret_ferror = (in >> 2) & 1; ret_fflush = 0; ret_fsync = in & 1; ret_fclose = (in >> 1) & 1;
    return arm ? siteC_on(&file_b) : siteC_off(&file_b);
}
static long runD(int arm, int in) {
    unsigned long v2 = 0; ret_fstat = in & 1;
    long r = arm ? siteD_on(0x77 + in, &v2) : siteD_off(0x77 + in, &v2);
    return r * 1000 + (long)v2;
}
static long runE(int arm, int in) {
    long vals[4][4] = {{-1, 5, 5, 5}, {3, -1, 7, 7}, {3, 4, 2, -1}, {0x100000000L, 0xffffffffL, -1, 1}};
    memcpy(ret_getline, vals[in & 3], sizeof ret_getline);
    return arm ? siteE_on(&file_a) : siteE_off(&file_a);
}
static long runG1(int arm, int in) { ret_ferror = in & 1; return (arm ? siteG1_on(in >> 1) : siteG1_off(in >> 1)) * 1000L + gflag; }
static long runG3(int arm, int in) { ret_fflush = in & 1; return (arm ? siteG3_default(in >> 1) : siteG3_off(in >> 1)) * 1000L + gflag; }
static long runG3ctl(int arm, int in) { ret_fflush = in & 1; return (arm ? siteG3_oldfold(in >> 1) : siteG3_off(in >> 1)) * 1000L + gflag; }
static long runH(int arm, int in) { dat_1050 = in ? &file_a : NULL; return arm ? siteH_default() : siteH_off(); }
static long runHctl(int arm, int in) { dat_1050 = in ? &file_a : NULL; return arm ? siteH_oldfold() : siteH_off(); }
static long runK(int arm, int in) { (void)in; if (arm) ctl_bad(); else ctl_good(); return ok; }

int main(void) {
    for (int i = 0; i < 4; i++) check("A sort-O2 main   v31 &= 0xffffffff; if (strcmp(v98,optarg))", 1, runA, i);
    for (int i = 0; i < 8; i++) check("C faillog main   (!fsync(v5)) && (!fclose(dat_6298))", 1, runC, i);
    for (int i = 0; i < 2; i++) check("D faillog main   v4 = fstat(fileno(dat_6298),..)", 1, runD, i);
    for (int i = 0; i < 4; i++) check("E du-O0 sub_11107 while (getline(&v13,&v12,v19) != -1)", 1, runE, i);
    for (int i = 0; i < 4; i++) check("B sort-O2 main   default keeps v15 = sub_c5b0(stdin), v15 == -1", 1, runB, i);
    for (int i = 0; i < 4; i++) check("F pr-O2ni main   default keeps v4 = sub_60c0(stdin), v4 == -1", 1, runF, i);
    for (int i = 0; i < 3; i++) check("B' old fold sub_c5b0(stdin) == -1, callee 0 (agrees)", 1, runBctl, i);
    check("B' CONTROL old fold sub_c5b0(stdin) == -1, callee returns 0xffffffff", 0, runBctl, 3);
    for (int i = 0; i < 3; i++) check("F' old fold sub_60c0(stdin) == -1, callee 0 (agrees)", 1, runFctl, i);
    check("F' CONTROL old fold sub_60c0(stdin) == -1, callee returns 0xffffffff", 0, runFctl, 3);
    check("K CONTROL fold past a global write", 0, runK, 0);
    {   /* in = (a0 << 1) | ret: a0 in {0, 5, 6, 9} */
        int g_in[] = {0 << 1, (0 << 1) | 1, 5 << 1, (5 << 1) | 1, 6 << 1, (6 << 1) | 1, 9 << 1, (9 << 1) | 1};
        for (int i = 0; i < 8; i++) check("G z1-O2  (ferror(stdin) || 5 < a0), call in the left operand", 1, runG1, g_in[i]);
        for (int i = 0; i < 8; i++) check("G z3-O2  default keeps v1 = fflush(stdout); (5 < a0 && !v1)", 1, runG3, g_in[i]);
        for (int i = 4; i < 8; i++) check("G' old fold (5 < a0 && !fflush(stdout)), a0 > 5 (agrees)", 1, runG3ctl, g_in[i]);
        for (int i = 0; i < 4; i++) check("G' CONTROL old fold (5 < a0 && !fflush(stdout)), a0 <= 5 skips the call", 0, runG3ctl, g_in[i]);
    }
    for (int i = 0; i < 2; i++) check("H i386 default keeps v1 = big(dat_1050); if (v1 != -1)", 1, runH, i);
    check("H' old fold big(dat_1050) != -1, callee returns 1 (agrees)", 1, runHctl, 0);
    check("H' CONTROL old fold big(dat_1050) != -1, callee returns 0xffffffff", 0, runHctl, 1);
    printf("%s: %d/%d cases as expected\n", fails ? "FOLDRUN FAIL" : "FOLDRUN OK", cases - fails, cases);
    return fails != 0;
}
