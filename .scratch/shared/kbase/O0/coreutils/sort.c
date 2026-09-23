// Function: sub_439b @ 0x439b
void sub_439b(int a0,char *a1)
{
  char *v1; // rax
  char v2 [12];
  long v3; // fs_offset
  unsigned long v4; // stack - 0x10
  
  v4 = *(unsigned long *)(v3 + 0x28);
  write(2,a1,strlen(a1));
  if (a0) {
    v1 = (char *)sub_126fe(a0,v2);
    write(2,": errno ",8);
    write(2,v1,strlen(v1));
  }
  write(2,"\n",1);
  _exit(2); // no-return
}


// Function: sub_4464 @ 0x4464
void sub_4464(unsigned long a0,char *a1)
{
  unsigned long v1; // rax
  
  if (!a1)
    a1 = gettext("standard output");
  v1 = sub_151eb(0,3,a1);
  error(2,*__errno_location(),"%s: %s",a0,v1); // no-return
}


// Function: sub_44d3 @ 0x44d3
void sub_44d3(int a0)
{
  void *v1;
  unsigned long v2;
  
  v2 = dat_23868;
  if (a0) // branch-flip
    fprintf(stderr,gettext("Try \'%s --help\' for more information.\n"),v2);
  else {
    printf(gettext("Usage: %s [OPTION]... [FILE]...\n  or:  %s [OPTION]... --files0-from=F\n"),v2,v2);
    v1 = stdout;
    fputs_unlocked(gettext("Write sorted concatenation of all FILE(s) to standard output.\n"),v1);
    sub_40a2();
    sub_40d3();
    v1 = stdout;
    fputs_unlocked(gettext("Ordering options:\n\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -b, --ignore-leading-blanks  ignore leading blanks\n  -d, --dictionary-order      consider only blanks and alphanumeric characters\n  -f, --ignore-case           fold lower case to upper case characters\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -g, --general-numeric-sort  compare according to general numerical value\n  -i, --ignore-nonprinting    consider only printable characters\n  -M, --month-sort            compare (unknown) < \'JAN\' < ... < \'DEC\'\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -h, --human-numeric-sort    compare human readable numbers (e.g., 2K 1G)\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -n, --numeric-sort          compare according to string numerical value\n  -R, --random-sort           shuffle, but group identical keys.  See shuf(1)\n      --random-source=FILE    get random bytes from FILE\n  -r, --reverse               reverse the result of comparisons\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --sort=WORD             sort according to WORD:\n                                general-numeric -g, human-numeric -h, month -M,\n                                numeric -n, random -R, version -V\n  -V, --version-sort          natural sort of (version) numbers within text\n\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("Other options:\n\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --batch-size=NMERGE   merge at most NMERGE inputs at once;\n                            for more use temp files\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -c, --check, --check=diagnose-first  check for sorted input; do not sort\n  -C, --check=quiet, --check=silent  like -c, but do not report first bad line\n      --compress-program=PROG  compress temporaries with PROG;\n                              decompress them with PROG -d\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --debug               annotate the part of the line used to sort,\n                              and warn about questionable usage to stderr\n      --files0-from=F       read input from the files specified by\n                            NUL-terminated names in file F;\n                            If F is - then read names from standard input\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -k, --key=KEYDEF          sort via a key; KEYDEF gives location and type\n  -m, --merge               merge already sorted files; do not sort\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -o, --output=FILE         write result to FILE instead of standard output\n  -s, --stable              stabilize sort by disabling last-resort comparison\n  -S, --buffer-size=SIZE    use SIZE for main memory buffer\n"),v1);
    printf(gettext("  -t, --field-separator=SEP  use SEP instead of non-blank to blank transition\n  -T, --temporary-directory=DIR  use DIR for temporaries, not $TMPDIR or %s;\n                              multiple options specify multiple directories\n      --parallel=N          change the number of sorts run concurrently to N\n  -u, --unique              with -c, check for strict ordering;\n                              without -c, output only the first of an equal run\n"),"/tmp");
    v1 = stdout;
    fputs_unlocked(gettext("  -z, --zero-terminated     line delimiter is NUL, not newline\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --help        display this help and exit\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --version     output version information and exit\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("\nKEYDEF is F[.C][OPTS][,F[.C][OPTS]] for start and stop position, where F is a\nfield number and C a character position in the field; both are origin 1, and\nthe stop position defaults to the line\'s end.  If neither -t nor -b is in\neffect, characters in a field are counted from the beginning of the preceding\nwhitespace.  OPTS is one or more single-letter ordering options [bdfgiMhnRrV],\nwhich override global ordering options for that key.  If no key is given, use\nthe entire line as the key.  Use --debug to diagnose incorrect key usage.\n\nSIZE may be followed by the following multiplicative suffixes:\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("% 1% of memory, b 1, K 1024 (default), and so on for M, G, T, P, E, Z, Y.\n\n*** WARNING ***\nThe locale specified by the environment affects sort order.\nSet LC_ALL=C to get the traditional sort order that uses\nnative byte values.\n"),v1);
    sub_4104("sort");
  }
  exit(a0); // no-return
}


// Function: sub_47f1 @ 0x47f1
void sub_47f1(long a0)
{
  *(bool *)a0 = pthread_sigmask(0,0x236a0,a0 + 8) == 0;
}


// Function: sub_4833 @ 0x4833
void sub_4833(char *a0) // return-dupe
{
  if (!*a0)
    return;
  pthread_sigmask(2,&a0[8],0);
}


// Function: sub_486b @ 0x486b
unsigned long sub_486b(long a0,unsigned long a1)
{
  return (unsigned long)(long)*(int *)(a0 + 8) % a1;
}


// Function: sub_489a @ 0x489a
unsigned int sub_489a(long a0,long a1)
{
  return CONCAT31((undefined3)((unsigned int)*(int *)(a1 + 8) >> 8),*(int *)(a0 + 8) == *(int *)(a1 + 8));
}


// Function: sub_48cf @ 0x48cf
int sub_48cf(int a0)
{
  char v1; // al
  int v2;
  unsigned long v3; // rax
  char *v4; // rax
  unsigned int v5; // stack - 0x28
  int v6; // stack - 0x24
  
  if (a0) // branch-flip
    v2 = a0;
  else {
    v2 = -1;
  }
  v6 = waitpid(v2,(int *)&v5,(unsigned int)(a0 == 0));
  if (0 <= v6) {
    if ((1 <= v6) && ((1 <= a0 || (v1 = sub_4a90(v6), v1)))) {
      if ((v5 & 0x7f) || ((int)v5 >> 8 & 0xffU)) {
        v3 = sub_15059(4,dat_23690);
        error(2,0,gettext("%s [-d] terminated abnormally"),v3); // no-return
      }
      dat_23730 -= 1;
    }
    return v6;
  }
  v3 = sub_15059(4,dat_23690);
  v4 = gettext("waiting for %s [-d]");
  error(2,*__errno_location(),v4,v3); // no-return
}


// Function: sub_4a08 @ 0x4a08
void sub_4a08(long a0)
{
  long v1; // rax
  
  if (!dat_23728) {
    dat_23728 = sub_1120d(0x2f,0,sub_486b,sub_489a,0);
    if (!dat_23728)
      sub_1865c(); // no-return
  }
  *(char *)(a0 + 0xc) = 1;
  v1 = sub_11f4e(dat_23728,a0);
  if (!v1)
    sub_1865c(); // no-return
}


// Function: sub_4a90 @ 0x4a90
bool sub_4a90(unsigned int a0)
{
  long v1; // rax
  char v2 [8];
  unsigned int v3; // stack - 0x20
  
  v3 = a0;
  v1 = sub_11fbe(dat_23728,v2);
  if (v1)
    *(char *)(v1 + 0xc) = 2;
  return v1 != 0;
}


// Function: sub_4aff @ 0x4aff
void sub_4aff(unsigned int a0) // return-dupe
{
  if (!sub_4a90(a0))
    return;
  sub_48cf(a0);
}


// Function: sub_4b29 @ 0x4b29
void sub_4b29(void)
{
  do {
    if (dat_23730 <= 0)
      return;
  } while (sub_48cf(0));
}


// Function: sub_4b4f @ 0x4b4f
void sub_4b4f(void)
{
  sub_48cf(0xffffffff);
  sub_4b29();
}


// Function: sub_4b69 @ 0x4b69
void sub_4b69(void)
{
  while (0 < dat_23730) {
    sub_48cf(0xffffffff);
  }
}


// Function: sub_4b8b @ 0x4b8b
void sub_4b8b(void)
{
  unsigned long *v1; // stack - 0x10
  
  for (v1 = dat_23720; v1; v1 = (unsigned long *)*v1) {
    unlink((char *)((long)v1 + 0xd));
  }
  dat_23720 = NULL;
}


// Function: sub_4bd4 @ 0x4bd4
void sub_4bd4(void)
{
  char v1 [136];
  
  if (dat_23720) {
    sub_47f1(v1);
    sub_4b8b();
    sub_4833(v1);
  }
  sub_fa34();
}


// Function: sub_4c3d @ 0x4c3d
unsigned long * sub_4c3d(int *a0,char a1)
{
  int v1;
  int v2; // eax
  unsigned long v3; // rax
  void *v4; // rax
  unsigned long v5; // rax
  char *v6;
  char v7 [136];
  unsigned long *v8; // stack - 0xc8
  
  v6 = *(char **)(dat_237a0 * 8 + dat_23668);
  v3 = strlen(v6);
  v8 = (unsigned long *)sub_18024(v3 + 0x20 & 0xfffffffffffffff8);
  v4 = (void *)((long)v8 + 0xd);
  memcpy(v4,v6,v3);
  memcpy((void *)(v3 + (long)v4),"/sortXXXXXX",0xc);
  *v8 = 0;
  dat_237a0 += 1;
  if (dat_237a0 == dat_23670)
    dat_237a0 = 0;
  sub_47f1(v7);
  v2 = sub_1704d(v4,0x80000);
  if (0 <= v2) {
    *dat_23100 = v8;
    dat_23100 = v8;
  }
  v1 = *__errno_location();
  sub_4833(v7);
  *__errno_location() = v1;
  if (v2 <= -1) {
    if ((a1 != '\x01') || (*__errno_location() != 0x18)) {
      v5 = sub_15059(4,v6);
      v6 = gettext("cannot create temporary file in %s");
      error(2,*__errno_location(),v6,v5); // no-return
    }
    free(v8);
    v8 = NULL;
  }
  *a0 = v2;
  return v8;
}


// Function: sub_4e8c @ 0x4e8c
unsigned long sub_4e8c(void) // return-dupe
{
  if (!dat_237a8) {
    if (fstat(1,(void *)0x237c0)) // branch-flip
      dat_237a8 = *__errno_location();
    else {
      dat_237a8 = -1;
    }
  }
  if (dat_237a8 > -1)
    return 0;
  return 0x237c0;
}


// Function: sub_4ee4 @ 0x4ee4
int * sub_4ee4(char *a0,char *a1)
{
  int v1;
  unsigned long v2;
  unsigned int v3; // eax
  int *v4; // rax
  void *v5; // stack - 0x28
  
  if (*a1 != 'r') { // branch-flip
    if (*a1 != 'w')
      __assert_fail("!\"unexpected mode passed to stream_open\"","src/sort.c",0x3d5,"stream_open"); // no-return
    if (a0) {
      v3 = ftruncate(1,0);
      if (v3) {
        v4 = __errno_location();
        v1 = *v4;
        v4 = (long)sub_4e8c();
        if ((!v4) || (v3 = *(unsigned int *)((long)v4 + 0x18) & 0xf000, v3 == 0x8000)) {
          v4 = (unsigned long)sub_151eb(0,3,a0);
          v2 = v4;
          v4 = (char *)gettext("%s: error truncating");
          error(2,v1,(char *)v4,v2); // no-return
        }
      }
    }
    v5 = stdout;
  }
  else {
    v3 = strcmp(a0,"-");
    if (v3) { // branch-flip
      v3 = open(a0,0x80000);
      if (0 <= (int)v3) // branch-flip
        v4 = fdopen(v3,a1);
      else {
        v4 = NULL;
      }
      v5 = v4;
    }
    else {
      dat_23683 = 1;
      v5 = stdin;
    }
    sub_fed8(v5,2);
  }
  v4 = v5;
  return v4;
}


// Function: sub_505e @ 0x505e
long sub_505e(unsigned long a0,unsigned long a1)
{
  long v1; // rax
  
  v1 = sub_4ee4(a0,a1);
  if (!v1)
    sub_4464(gettext("open failed"),a0); // no-return
  return v1;
}


// Function: sub_50b7 @ 0x50b7
void sub_50b7(void *a0,unsigned long a1) // return-dupe x2
{
  int v1; // eax
  
  v1 = fileno(a0);
  if (!v1) {
    clearerr_unlocked(a0);
    return;
  }
  if (v1 != 1) {
    if (!sub_ff17(a0))
      return;
    sub_4464(gettext("close failed"),a1); // no-return
  }
  if (!fflush_unlocked(a0))
    return;
  sub_4464(gettext("fflush failed"),a1); // no-return
}


// Function: sub_515d @ 0x515d
void sub_515d(int a0,int a1) // return-dupe
{
  if (a0 == a1)
    return;
  dup2(a0,a1);
  close(a0);
}


// Function: sub_5193 @ 0x5193
int sub_5193(int *a0,long a1) // early-return
{
  long v1;
  int v2;
  unsigned long v3;
  char v4 [136];
  long v5; // stack - 0xc8
  int v6; // stack - 0xb0
  
  if ((int)sub_1311d(a0,0x80000) <= -1)
    return -1;
  v5 = a1;
  if (dat_23024 + 1U < dat_23730) {
    sub_4b4f();
    v5 = a1;
  }
  while (v1 = v5 + -1, v5) {
    sub_47f1(v4);
    v3 = dat_23720;
    dat_23720 = 0;
    v6 = fork();
    v2 = *__errno_location();
    if (v6)
      dat_23720 = v3;
    sub_4833(v4);
    *__errno_location() = v2;
    if ((0 <= v6) || (*__errno_location() != 0xb)) break;
    sub_18863();
    sub_4b29();
    v5 = v1;
  }
  if (0 <= v6) { // branch-flip
    if (v6) // branch-flip
      dat_23730 += 1;
    else {
      close(0);
      close(1);
    }
  }
  else {
    v2 = *__errno_location();
    close(*a0);
    close(a0[1]);
    *__errno_location() = v2;
  }
  return v6;
}


// Function: sub_5372 @ 0x5372
long sub_5372(long *a0,char a1) // early-return
{
  long v1;
  unsigned int v2; // eax
  void *v3; // rax
  int v4; // stack - 0x34
  int v5; // stack - 0x28
  long v6; // stack - 0x30
  int v7; // stack - 0x24
  
  v6 = sub_4c3d(&v4,a1);
  if (!v6)
    return 0;
  *(char *)(v6 + 0xc) = 0;
  if (dat_23690) {
    v2 = sub_5193(&v5,4);
    *(unsigned int *)(v6 + 8) = v2;
    if (1 <= *(int *)(v6 + 8)) { // branch-flip
      close(v4);
      close(v5);
      v4 = v7;
      sub_4a08(v6);
    }
    else if (!*(int *)(v6 + 8)) {
      close(v7);
      sub_515d(v4,1);
      sub_515d(v5,0);
      execlp(dat_23690,dat_23690,0);
      sub_439b(*__errno_location(),"couldn\'t execute compress program"); // no-return
    }
  }
  v3 = fdopen(v4,"w");
  *a0 = (long)v3;
  if (*a0)
    return v6;
  v1 = v6 + 0xd;
  sub_4464(gettext("couldn\'t create temporary file"),v1); // no-return
}


// Function: sub_54f9 @ 0x54f9
void sub_54f9(unsigned long a0)
{
  sub_5372(a0,0);
}


// Function: sub_551c @ 0x551c
void * sub_551c(long a0) // early-return
{
  int v1;
  int v2; // eax
  unsigned long v3; // rax
  char *v4; // rax
  int v5; // stack - 0x28
  void *v6; // stack - 0x30
  int v7; // stack - 0x24
  
  v6 = NULL;
  if (*(char *)(a0 + 0xc) == '\x01')
    sub_4aff(*(unsigned int *)(a0 + 8));
  v1 = open((char *)(a0 + 0xd),0);
  if (v1 <= -1)
    return NULL;
  v2 = sub_5193(&v5,9);
  if (v2 != -1) { // branch-flip
    if (!v2) {
      close(v5);
      sub_515d(v1,0);
      sub_515d(v7,1);
      execlp(dat_23690,dat_23690,"-d",0);
      sub_439b(*__errno_location(),"couldn\'t execute compress program (with -d)"); // no-return
    }
    *(int *)(a0 + 8) = v2;
    sub_4a08(a0);
    close(v1);
    close(v7);
    v6 = fdopen(v5,"r");
    if (!v6) {
      v1 = *__errno_location();
      close(v5);
      *__errno_location() = v1;
    }
  }
  else {
    if (*__errno_location() != 0x18) {
      v3 = sub_15059(4,dat_23690);
      v4 = gettext("couldn\'t create process for %s -d");
      error(2,*__errno_location(),v4,v3); // no-return
    }
    close(v1);
    *__errno_location() = 0x18;
  }
  return v6;
}


// Function: sub_570f @ 0x570f
void sub_570f(unsigned long a0)
{
  long v1;
  
  if (dat_23670 == dat_23678)
    dat_23668 = sub_18221(dat_23668,0x23678,8);
  v1 = dat_23670;
  dat_23670 += 1;
  *(unsigned long *)(dat_23668 + v1 * 8) = a0;
}


// Function: sub_5782 @ 0x5782
void sub_5782(char *a0)
{
  int v1;
  long *v2;
  long v3;
  int v4; // eax
  unsigned long v5; // rax
  char v6 [136];
  long *v7; // stack - 0xc0
  
  v7 = (long *)0x23720;
  while (v2 = (long *)*v7, a0 != (char *)((long)v2 + 0xd)) {
    v7 = v2;
  }
  if (*(char *)((long)v2 + 0xc) == '\x01')
    sub_4aff((int)v2[1]);
  v3 = *v2;
  sub_47f1(v6);
  v4 = unlink(a0);
  v1 = *__errno_location();
  *v7 = v3;
  sub_4833(v6);
  if (!v4) {
    if (!v3)
      dat_23100 = v7;
    free(v2);
    return;
  }
  v5 = sub_151eb(0,3,a0);
  error(0,v1,gettext("warning: cannot remove: %s"),v5); // no-return
}


// Function: sub_590b @ 0x590b
void sub_590b(unsigned long *a0,unsigned long *a1)
{
  strcmp((char *)*a0,(char *)*a1);
}


// Function: sub_594a @ 0x594a
void sub_594a(void) // return-dupe, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_594a
{
  char v1;
  long v10; // stack - 0x38
  long v2;
  char v3; // al
  int v4; // eax
  char *v5; // rax
  unsigned long v6; // rax
  long v7; // rax
  unsigned long v8; // stack - 0x48
  unsigned long v9; // stack - 0x40
  
  for (v8 = 0; v8 <= 0xff; v8 = v8 + 1) {
    *(char *)(v8 + 0x23260) = sub_4051(v8 & 0xff);
    v4 = (int)v8;
    *(bool *)(v8 + 0x23360) = (*(unsigned short *)((long)v4 * 2 + *(long *)__ctype_b_loc()) & 0x4000) == 0;
    v3 = !((*(unsigned short *)((long)v4 * 2 + *(long *)__ctype_b_loc()) & 8) || (sub_4051(v8 & 0xff) == '\x01'));
    *(char *)(v8 + 0x23460) = v3;
    *(char *)(v8 + 0x23560) = (char)toupper(v4);
  }
  if (!dat_2322a)
    return;
  for (v8 = 0; v8 <= 0xb; v8 = v8 + 1) {
    v5 = nl_langinfo((int)v8 + 0x2000e);
    v6 = strlen(v5);
    v7 = sub_18024(v6 + 1);
    *(long *)(v8 * 0x10 + 0x23040) = v7;
    *(int *)(v8 * 0x10 + 0x23048) = (int)v8 + 1;
    v10 = 0;
    for (v9 = 0; v9 < v6; v9 = v9 + 1) {
      v2 = *(long *)__ctype_b_loc();
      v1 = v5[v9];
      if (!(*(unsigned short *)((unsigned long)(unsigned char)sub_4042((int)v1) * 2 + v2) & 1)) {
        v1 = v5[v9];
        v2 = v10 + 1;
        *(char *)(v7 + v10) = *(char *)((long)(int)(unsigned int)(unsigned char)sub_4042((int)v1) + 0x23560);
        v10 = v2;
      }
    }
    *(char *)(v10 + v7) = 0;
  }
  qsort(0x23040,0xc,0x10,sub_590b);
}


// Function: sub_5ba8 @ 0x5ba8
void sub_5ba8(int a0,char a1,unsigned long a2) // ternary
{
  unsigned long v1;
  unsigned int v2; // eax
  unsigned long v3; // rax
  unsigned long v4; // stack - 0x50
  int v5 [10]; // stack - 0x48
  int v6; // stack - 0x58
  
  v6 = sub_18b54(a2,0,10,&v4,0x1b1db);
  v2 = (getrlimit(7,v5)) ? 0x11 : v5[0] - 3; // branch-flip
  if (!v6) {
    dat_23024 = (unsigned int)v4;
    if ((v4 & 0xffffffff) != v4) // branch-flip
      v6 = 1;
    else {
      if (dat_23024 <= 1) {
        v3 = sub_154a0(a2);
        v1 = *(unsigned long *)((long)a0 * 0x20 + 0x22560);
        error(0,0,gettext("invalid --%s argument %s"),v1,v3); // no-return
      }
      if (dat_23024 <= v2)
        return;
      v6 = 1;
    }
  }
  if (v6 != 1)
    sub_18a33(v6,a0,(int)a1,0x22560,a2); // no-return
  v3 = sub_154a0(a2);
  v1 = *(unsigned long *)((long)a0 * 0x20 + 0x22560);
  error(0,0,gettext("--%s argument %s too large"),v1,v3); // no-return
}


// Function: sub_5e0c @ 0x5e0c
void sub_5e0c(unsigned int a0,char a1,unsigned long a2) // return-dupe, ternary
{
  unsigned long v1; // stack - 0x28
  char *v2; // stack - 0x20
  double v3; // xmm0_qa
  int v4; // stack - 0x2c
  double v5; // stack - 0x18
  
  v4 = sub_18b54(a2,&v2,10,&v1,"EgGkKmMPtTYZ");
  if ((!v4) && ((unsigned int)((int)v2[-1] - 0x30U) <= 9)) {
    if (0x40000000000000 <= v1) // branch-flip
      v4 = 1;
    else {
      v1 <<= 10;
    }
  }
  if (((v4 == 2) && ((unsigned int)((int)v2[-1] - 0x30U) <= 9)) && (!v2[1])) {
    if (*v2 != '%') { // branch-flip
      if (*v2 == 'b')
        v4 = 0;
    }
    else {
      sub_12e84();
      v5 = ((double)v1 * v3) / dat_1d140;
      if (dat_1d148 <= v5)
        v4 = 1;
      else {
        v1 = (dat_1d150 <= v5) ? (long)(v5 - dat_1d150) ^ 0x8000000000000000 : (unsigned long)v5;
        v4 = 0;
      }
    }
  }
  if (v4)
    sub_18a33(v4,a0,(int)a1,0x22560,a2); // no-return
  if (v1 < dat_23660)
    return;
  if ((unsigned long)dat_23024 * 0x22 < v1) {
    dat_23660 = v1;
    return;
  }
  dat_23660 = (unsigned long)dat_23024 * 0x22;
}


// Function: sub_6042 @ 0x6042
long sub_6042(unsigned int a0,char a1,unsigned long a2) // early-return
{
  int v1; // eax
  long v2; // stack - 0x18
  
  v1 = sub_18b54(a2,0,10,&v2,0x1b1db);
  if (v1 == 1)
    return -1;
  if (!v1) {
    if (v2)
      return v2;
    error(2,0,gettext("number in parallel must be nonzero")); // no-return
  }
  sub_18a33(v1,a0,(int)a1,0x22560,a2); // no-return
}


// Function: sub_610d @ 0x610d
unsigned long sub_610d(void)
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3 [3]; // stack - 0x28
  double v4; // xmm0_qa
  double v5; // xmm0_qa
  double v6;
  double v7; // xmm0_qa
  unsigned long v8; // stack - 0x48
  
  v8 = 0xffffffffffffffff;
  v1 = getrlimit(2,v3);
  if ((!v1) && (v3[0] != 0xffffffffffffffff))
    v8 = v3[0];
  v1 = getrlimit(9,v3);
  if ((!v1) && (v3[0] < v8))
    v8 = v3[0];
  v2 = v8 >> 1;
  v8 = v2;
  v1 = getrlimit(5,v3);
  if ((!v1) && (v2 = v3[0] >> 4, v2 * 0xf < v8)) {
    v2 = v3[0] >> 4;
    v8 = v2 * 0xf;
  }
  sub_12fab();
  sub_12e84();
  if (v4 <= v5 / dat_1d158)
    v6 = v5 / dat_1d158;
  else {
    v6 = v4;
  }
  if (v5 * dat_1d160 < (double)v8) {
    v7 = dat_1d160 * v5;
    if (dat_1d150 <= v7) {
      v2 = (unsigned long)(v7 - dat_1d150);
      v8 = v2 ^ 0x8000000000000000;
    }
    else {
      v2 = (unsigned long)v7;
      v8 = v2;
    }
  }
  if (v6 < (double)v8) {
    if (dat_1d150 <= v6) {
      v2 = (unsigned long)(v6 - dat_1d150);
      v8 = v2 ^ 0x8000000000000000;
    }
    else {
      v2 = (unsigned long)v6;
      v8 = v2;
    }
  }
  v2 = v8;
  if (v8 <= (unsigned long)dat_23024 * 0x22)
    v2 = (unsigned long)dat_23024 * 0x22;
  return v2;
}


// Function: sub_637c @ 0x637c
long sub_637c(long a0,unsigned long a1,long a2,unsigned long a3,long a4)
{
  char *v1;
  unsigned int v10; // stack - 0xa0
  unsigned long v11; // stack - 0x88
  unsigned long v2;
  int v3; // eax
  unsigned long v4; // rax
  char v5 [24];
  bool v6;
  long v7; // stack - 0xe0
  unsigned long v8; // stack - 0xd8
  unsigned long v9; // stack - 0xd0
  
  v7 = a4 + 2;
  v8 = 0;
  while( true ) {
    if (a3 <= v8)
      return v7;
    if (a1 <= v8) { // branch-flip
      v1 = *(char **)(a2 + v8 * 8);
      if (strcmp(v1,"-")) { // branch-flip
        v1 = *(char **)(a2 + v8 * 8);
        v6 = stat(v1,v5) != 0;
      }
      else {
        v6 = fstat(0,v5) != 0;
      }
    }
    else {
      v3 = fileno(*(void **)(a0 + v8 * 8));
      v6 = fstat(v3,v5) != 0;
    }
    if (v6) {
      v2 = *(unsigned long *)(a2 + v8 * 8);
      sub_4464(gettext("stat failed"),v2); // no-return
    }
    if ((v10 & 0xf000) != 0x8000) { // branch-flip
      if (dat_23660)
        return dat_23660;
      v9 = 0x20000;
    }
    else {
      v9 = v11;
    }
    if ((!dat_23850) && (dat_23850 = dat_23660, !dat_23660))
      dat_23850 = sub_610d();
    v4 = v9 * (a4 + 1U) + 1;
    if ((v4 / (a4 + 1U) != v9) || ((unsigned long)(dat_23850 - v7) <= v4)) break;
    v7 += v4;
    v8 += 1;
  }
  return dat_23850;
}


// Function: sub_6618 @ 0x6618
void sub_6618(long *a0,long a1,unsigned long a2)
{
  unsigned long v1; // stack - 0x20
  
  v1 = a2;
  do {
    v1 = (v1 & 0xffffffffffffffe0) + 0x20;
    *a0 = (long)malloc(v1);
    if (*a0) {
      a0[5] = a1;
      a0[3] = v1;
      a0[2] = 0;
      a0[4] = a0[2];
      a0[1] = a0[4];
      *(char *)&a0[6] = 0;
      return;
    }
    v1 >>= 1;
  } while (a1 + 1U < v1);
  sub_1865c(); // no-return
}


// Function: sub_66d0 @ 0x66d0
long sub_66d0(long *a0)
{
  return a0[3] + *a0;
}


// Function: sub_66f4 @ 0x66f4
char * sub_66f4(unsigned long *a0,long *a1)
{
  long v1;
  char v2;
  long v3;
  char *v4;
  char *v5; // stack - 0x28
  long v6; // stack - 0x20
  
  v5 = (char *)*a0;
  v4 = &v5[a0[1] + -1];
  v6 = *a1;
  v3 = a1[1];
  if (dat_23020 != 0x80) { // branch-flip
    while ((v5 < v4 && (v1 = v6 + -1, v6))) {
      for (; (v5 < v4 && (*v5 != dat_23020)); v5 = &v5[1]) {
      }
      v6 = v1;
      if (v5 < v4)
        v5 = &v5[1];
    }
  }
  else {
    while ((v5 < v4 && (v1 = v6 + -1, v6))) {
      while ((v5 < v4 && (v2 = *v5, *(char *)((long)(int)(unsigned int)(unsigned char)sub_4042((int)v2) + 0x23260)))) {
        v5 = &v5[1];
      }
      while ((v6 = v1, v5 < v4 && (v2 = *v5, *(char *)((long)(int)(unsigned int)(unsigned char)sub_4042((int)v2) + 0x23260) != '\x01'))) {
        v5 = &v5[1];
      }
    }
  }
  if ((char)a1[6]) {
    while ((v5 < v4 && (v2 = *v5, *(char *)((long)(int)(unsigned int)(unsigned char)sub_4042((int)v2) + 0x23260)))) {
      v5 = &v5[1];
    }
  }
  if (&v5[v3] < v4)
    v4 = &v5[v3];
  return v4;
}


// Function: sub_6898 @ 0x6898
char * sub_6898(unsigned long *a0,long a1)
{
  long v1;
  long v2;
  char *v3;
  unsigned char v4; // al
  unsigned int v5; // eax
  char *v6; // rax
  char *v7; // stack - 0x28
  long v8; // stack - 0x20
  
  v7 = (char *)*a0;
  v6 = &v7[a0[1] + -1];
  v3 = v6;
  v8 = *(long *)(a1 + 0x10);
  v2 = *(long *)(a1 + 0x18);
  if (!v2)
    v8 += 1;
  if (dat_23020 != 0x80) { // branch-flip
    while ((v7 < v6 && (v1 = v8 + -1, v8))) {
      for (; (v7 < v6 && (*v7 != dat_23020)); v7 = &v7[1]) {
      }
      v8 = v1;
      if ((v7 < v6) && ((v1 || (v2))))
        v7 = &v7[1];
    }
  }
  else {
    while ((v7 < v3 && (v1 = v8 + -1, v8))) {
      while ((v7 < v3 && (v5 = (int)*v7, v4 = sub_4042(v5), v5 = (unsigned int)v4, v6 = (long)(long)(int)v5, *(char *)((long)v6 + 0x23260)))) {
        v7 = &v7[1];
      }
      while ((v8 = v1, v7 < v3 && (v5 = (int)*v7, v4 = sub_4042(v5), v5 = (unsigned int)v4, v6 = (long)(long)(int)v5, *(char *)((long)v6 + 0x23260) != '\x01'))) {
        v7 = &v7[1];
      }
    }
  }
  if (v2) {
    if (*(char *)(a1 + 0x31)) {
      while ((v7 < v3 && (v5 = (int)*v7, v4 = sub_4042(v5), v5 = (unsigned int)v4, v6 = (long)(long)(int)v5, *(char *)((long)v6 + 0x23260)))) {
        v7 = &v7[1];
      }
    }
    v6 = v3;
    if (&v7[v2] < v3)
      v6 = &v7[v2];
    v7 = v6;
  }
  v6 = v7;
  return v6;
}


// Function: sub_6a5e @ 0x6a5e
unsigned long sub_6a5e(long *a0,void *a1,unsigned long a2)
{
  char *v1;
  char *v10; // stack - 0x68
  long *v11; // stack - 0x60
  unsigned long v12; // stack - 0x58
  char *v13; // stack - 0x50
  char *v14; // stack - 0x48
  long *v15; // stack - 0x40
  long v16; // stack - 0x38
  long v17; // stack - 0x30
  unsigned long v18; // stack - 0x28
  unsigned long v19; // stack - 0x20
  long *v2;
  char *v20; // stack - 0x18
  char v3;
  char v4;
  unsigned long v5;
  long v6;
  char *v7; // rax
  unsigned long v8; // stack - 0x78
  unsigned long v9; // stack - 0x70
  
  v4 = dat_23010;
  v15 = dat_23688;
  v16 = a0[5];
  v9 = dat_23018 - 0x22;
  if ((char)a0[6])
    return 0;
  if (a0[1] != a0[4]) {
    memmove((void *)*a0,(void *)((a0[1] - a0[4]) + *a0),a0[4]);
    a0[1] = a0[4];
    a0[2] = 0;
  }
  do {
    v10 = (char *)(a0[1] + *a0);
    v17 = sub_66d0(a0);
    v11 = (long *)(v17 + a0[2] * -0x20);
    v12 = (v17 - a0[2] * v16) - (long)v10;
    if (a0[2]) // branch-flip
      v13 = (char *)(v11[1] + *v11);
    else {
      v13 = (char *)*a0;
    }
    do {
      if (v12 <= v16 + 1U) break;
      v18 = (v12 - 1) / (v16 + 1U);
      v19 = fread_unlocked(v10,1,v18,a1);
      v14 = &v10[v19];
      v12 -= v19;
      if (v19 != v18) {
        if (ferror_unlocked(a1))
          sub_4464(gettext("read failed"),a2); // no-return
        if (feof_unlocked(a1)) {
          *(char *)&a0[6] = 1;
          if (v14 == (char *)*a0)
            return 0;
          if ((v13 != v14) && (v4 != v14[-1])) {
            v1 = &v14[1];
            *v14 = v4;
            v14 = v1;
          }
        }
      }
      while( true ) {
        v7 = memchr(v10,(int)v4,(long)v14 - (long)v10);
        v20 = v7;
        if (!v7) break;
        *v7 = 0;
        v10 = &v7[1];
        v2 = &v11[-4];
        *v2 = (long)v13;
        v11[-3] = (long)v10 - (long)v13;
        v5 = v11[-3];
        if ((unsigned long)v11[-3] <= v9)
          v5 = v9;
        v12 -= v16;
        v9 = v5;
        v11 = v2;
        if (v15) {
          if (v15[2] != -1)
            v7 = (char *)sub_6898(v2,v15);
          v11[3] = (long)v7;
          if (*v15 != -1) { // branch-flip
            v6 = sub_66f4(v11,v15);
            v11[2] = v6;
          }
          else {
            if ((char)v15[6]) {
              while (v3 = *v13, *(char *)((long)(int)(unsigned int)(unsigned char)sub_4042((int)v3) + 0x23260)) {
                v13 = &v13[1];
              }
            }
            v11[2] = (long)v13;
          }
        }
        v13 = v10;
      }
      v10 = v14;
    } while (!(char)a0[6]);
    a0[1] = (long)v10 - *a0;
    a0[2] = sub_66d0(a0) - (long)v11 >> 5;
    if (a0[2]) {
      a0[4] = (long)v10 - (long)v13;
      dat_23018 = v9 + 0x22;
      return 1;
    }
    v8 = (unsigned long)a0[3] >> 5;
    v6 = *a0;
    *a0 = sub_18221(v6,&v8,0x20);
    a0[3] = v8 << 5;
  } while( true );
}


// Function: sub_6f64 @ 0x6f64
char sub_6f64(long *a0)
{
  char *v1;
  char v2;
  bool v3;
  char v4; // stack - 0x13
  char *v5; // stack - 0x10
  
  v4 = '\0';
  v3 = 0;
  v1 = (char *)*a0;
  while( true ) {
    v5 = v1;
    v1 = &v5[1];
    v2 = *v5;
    if (10 <= (unsigned int)((int)v2 - 0x30U)) break;
    if (v4 < v2)
      v4 = v2;
    v3 = *v1 == dat_23224;
    if (v3)
      v1 = &v5[2];
  }
  if (v3)
    *a0 = (long)&v5[-1];
  else {
    v5 = v1;
    if (v2 == dat_23220) {
      while( true ) {
        v1 = &v5[1];
        v2 = *v5;
        v5 = v1;
        if (10 <= (unsigned int)((int)v2 - 0x30U)) break;
        if (v4 < v2)
          v4 = v2;
      }
    }
    *a0 = (long)&v5[-1];
  }
  return v4;
}


// Function: sub_7042 @ 0x7042
int sub_7042(char *a0) // early-return
{
  char v1;
  int v2; // eax
  unsigned char *v3; // stack - 0x18
  
  v1 = *a0;
  v3 = (unsigned char *)&a0[v1 == '-'];
  if ('1' > (char)sub_6f64(&v3))
    return 0;
  v2 = (int)*(char *)((long)(int)(unsigned int)*v3 + 0x1c5c0);
  if (v1 == '-')
    v2 = -v2;
  return v2;
}


// Function: sub_70e2 @ 0x70e2
unsigned long sub_70e2(char *a0,char *a1)
{
  int v1;
  unsigned char v2; // al
  unsigned int v3; // eax
  unsigned long v4; // rax
  char *v5; // stack - 0x38
  char *v6; // stack - 0x30
  
  v6 = a0;
  while (v3 = (int)*v6, v2 = sub_4042(v3), v3 = (unsigned int)v2, v4 = (long)(int)v3, v5 = a1, *(char *)(v4 + 0x23260)) {
    v6 = &v6[1];
  }
  while (v3 = (int)*v5, v2 = sub_4042(v3), v3 = (unsigned int)v2, v4 = (long)(int)v3, *(char *)(v4 + 0x23260)) {
    v5 = &v5[1];
  }
  v3 = sub_7042(v6);
  v1 = v3;
  v3 = sub_7042(v5);
  v3 = v1 - v3;
  if (v3) // branch-flip
    v4 = (unsigned long)v3;
  else {
    v4 = sub_1764c(v6,v5,(int)dat_23220,dat_23224);
  }
  return v4;
}


// Function: sub_71a3 @ 0x71a3
void sub_71a3(char *a0,char *a1)
{
  char v1;
  char *v2; // stack - 0x18
  char *v3; // stack - 0x10
  
  v3 = a0;
  while (v1 = *v3, v2 = a1, *(char *)((long)(int)(unsigned int)(unsigned char)sub_4042((int)v1) + 0x23260)) {
    v3 = &v3[1];
  }
  while (v1 = *v2, *(char *)((long)(int)(unsigned int)(unsigned char)sub_4042((int)v1) + 0x23260)) {
    v2 = &v2[1];
  }
  sub_1764c(v3,v2,(int)dat_23220,dat_23224);
}


// Function: sub_7231 @ 0x7231
void sub_7231(unsigned long a0,unsigned long a1)
{
  undefined10 v1; // stack - 0x38
  undefined10 v2; // stack - 0x28
  undefined10 v3; // st0
  undefined10 v4; // st0
  
  memset(&v1,0,0x10);
  strtold(a0,0);
  v1 = v3;
  memset(&v2,0,0x10);
  strtold(a1,0);
  v2 = v4;
  memcmp(&v1,&v2,0x10);
}


// Function: sub_72d6 @ 0x72d6
unsigned long sub_72d6(long a0,long a1) // early-return x8
{
  long v1; // stack - 0x48
  long v2; // stack - 0x40
  long double v3; // st0
  long double v4; // st0
  long double v5; // stack - 0x38
  
  strtold(a0,&v1);
  v5 = v3;
  strtold(a1,&v2);
  if (a0 == v1) {
    if (a1 == v2)
      return 0;
    return 0xffffffff;
  }
  if (a1 == v2)
    return 1;
  if (v5 < v4)
    return 0xffffffff;
  if (v4 < v5)
    return 1;
  if (v4 == v5)
    return 0;
  if (!NAN(v4) && !NAN(v4))
    return 0xffffffff;
  if (NAN(v5))
    return sub_7231(a0,a1);
  return 1;
}


// Function: sub_73f3 @ 0x73f3
unsigned int sub_73f3(char *a0,unsigned long *a1)
{
  char v1;
  unsigned char v2; // al
  unsigned long v3; // rax
  char *v4; // stack - 0x48
  unsigned long v5; // stack - 0x38
  unsigned long v6; // stack - 0x30
  char *v7; // stack - 0x28
  char *v8; // stack - 0x20
  
  v5 = 0;
  v6 = 0xc;
  v4 = a0;
  while (v1 = *v4, *(char *)((long)(int)(unsigned int)(unsigned char)sub_4042((int)v1) + 0x23260)) {
    v4 = &v4[1];
  }
  do {
    v3 = v6 + v5 >> 1;
    v7 = v4;
    v8 = *(char **)(v3 * 0x10 + 0x23040);
    while( true ) {
      if (!*v8) {
        if (a1)
          *a1 = v7;
        return *(unsigned int *)(v3 * 0x10 + 0x23048);
      }
      v1 = *v7;
      v2 = sub_4042((int)*(char *)((long)(int)(unsigned int)(unsigned char)sub_4042((int)v1) + 0x23560));
      v1 = *v8;
      if (v2 < (unsigned char)sub_4042((int)v1)) {
        v6 = v3;
        goto label_755a;
      }
      v1 = *v7;
      v2 = sub_4042((int)*(char *)((long)(int)(unsigned int)(unsigned char)sub_4042((int)v1) + 0x23560));
      v1 = *v8;
      if ((unsigned char)sub_4042((int)v1) < v2) break;
      v7 = &v7[1];
      v8 = &v8[1];
    }
    v5 = v3 + 1;
label_755a:
    if (v6 <= v5)
      return 0;
  } while( true );
}


// Function: sub_7573 @ 0x7573
void sub_7573(char *a0)
{
  long v1; // rax
  char v2 [24];
  
  v1 = sub_15668(a0,0x10);
  if (!v1) {
    if (!a0)
      a0 = "getrandom";
    sub_4464(gettext("open failed"),a0); // no-return
  }
  sub_159b4(v1,v2,0x10);
  if (sub_15a0f(v1))
    sub_4464(gettext("close failed"),a0); // no-return
  sub_fb2c(0x23740);
  sub_fb4b(v2,0x10,0x23740);
}


// Function: sub_7668 @ 0x7668
unsigned long sub_7668(unsigned long a0,unsigned long a1,unsigned long a2)
{
  unsigned long v1; // rax
  char *v2; // rax
  
  *__errno_location() = 0;
  v1 = strxfrm(a0,a1,a2);
  if (*__errno_location()) {
    v2 = gettext("string transformation failed");
    error(0,*__errno_location(),v2); // no-return
  }
  return v1;
}


// Function: sub_7755 @ 0x7755
int sub_7755(char *a0,unsigned long a1,char *a2,unsigned long a3) // ternary
{
  unsigned long v1;
  unsigned long v10; // stack - 0x1108
  char *v11; // stack - 0x1100
  unsigned long v12; // stack - 0x10f8
  char *v13; // stack - 0x10f0
  int v14; // stack - 0x10e0
  int v15; // stack - 0x10dc
  char *v16; // stack - 0x10d8
  unsigned long v17; // stack - 0x10d0
  char *v18; // stack - 0x10c8
  unsigned long v19; // stack - 0x1070
  unsigned long v2; // rax
  unsigned long v20; // stack - 0x1068
  unsigned long v21; // stack - 0x1060
  unsigned long v22; // stack - 0x1058
  unsigned long v23; // stack - 0x1050
  unsigned long v24; // stack - 0x1048
  unsigned long v25; // stack - 0x1040
  unsigned long v26; // stack - 0x1038
  unsigned long v27; // stack - 0x1030
  unsigned long v28; // stack - 0x1028
  unsigned int v29; // stack - 0x1020
  long v3;
  unsigned long v30; // stack - 0x1014
  unsigned int v31; // stack - 0x100c
  unsigned int v32; // stack - 0x1008
  unsigned int v33; // stack - 0x1004
  unsigned int v34; // stack - 0x1000
  unsigned long v35; // stack - 0xffc
  unsigned long v36; // stack - 0xff4
  unsigned long v37; // stack - 0xfec
  unsigned long v38; // stack - 0xfe4
  unsigned long v39; // stack - 0xfdc
  char *v4; // rcx
  unsigned long v40; // stack - 0xfd4
  unsigned long v41; // stack - 0xfcc
  unsigned int v42; // stack - 0xfc4
  char v5 [4008];
  unsigned long v6; // stack - 0x1078
  unsigned long v7; // stack - 0x101c
  char v8 [16];
  char v9 [16];
  
  v14 = 0;
  v16 = v5;
  v17 = 4000;
  v18 = NULL;
  v7 = dat_23740;
  v30 = dat_23748;
  v32 = (unsigned int)((unsigned long)dat_23750 >> 0x20);
  v31 = (unsigned int)dat_23750;
  v34 = (unsigned int)((unsigned long)dat_23758 >> 0x20);
  v33 = (unsigned int)dat_23758;
  v35 = dat_23760;
  v36 = dat_23768;
  v37 = dat_23770;
  v38 = dat_23778;
  v39 = dat_23780;
  v40 = dat_23788;
  v41 = dat_23790;
  v42 = dat_23798;
  v6 = dat_23740;
  v19 = dat_23748;
  v20 = dat_23750;
  v21 = dat_23758;
  v22 = dat_23760;
  v23 = dat_23768;
  v24 = dat_23770;
  v25 = dat_23778;
  v26 = dat_23780;
  v27 = dat_23788;
  v28 = dat_23790;
  v29 = dat_23798;
  v10 = a3;
  v11 = a2;
  v12 = a1;
  v13 = a0;
  if (dat_23229) {
    a0 = &a0[a1];
    a2 = &a2[a3];
    while( true ) {
      v1 = (a1 + a3) * 3 + 2;
      if (v17 < v1) {
        v2 = v17 * 3 >> 1;
        v17 = v1;
        if (v1 < v2)
          v17 = v2;
        free(v18);
        v18 = malloc(v17);
        v16 = v18;
        if (!v18) {
          v16 = v5;
          v17 = 4000;
        }
      }
      v12 = (a0 <= v13) ? 0 : sub_7668(v16,v13,v17) + 1; // branch-flip
      if (a2 <= v11) // branch-flip
        v10 = 0;
      else {
        if (v17 < v12) {
          v3 = 0;
          v4 = NULL;
        }
        else {
          v3 = v17 - v12;
          v4 = &v16[v12];
        }
        v10 = sub_7668(v4,v11,v3) + 1;
      }
      if ((v17 < v12) || (v17 < v10 + v12)) {
        v17 = v10 + v12;
        if (v17 <= 0x5555555555555554)
          v17 = v17 * 3 >> 1;
        free(v18);
        v16 = (char *)sub_18024(v17);
        v18 = v16;
        if (v13 < a0)
          strxfrm(v16,v13,v12);
        if (v11 < a2)
          strxfrm(&v16[v12],v11,v10);
      }
      if (v13 < a0)
        v13 = &v13[strlen(v13) + 1];
      if (v11 < a2)
        v11 = &v11[strlen(v11) + 1];
      if ((a0 <= v13) && (a2 <= v11)) break;
      sub_fb4b(v16,v12,&v6);
      sub_fb4b(&v16[v12],v10,&v7);
      if (!v14) {
        v1 = v10;
        if (v12 < v10)
          v1 = v12;
        v14 = memcmp(v16,&v16[v12],v1);
        if (!v14)
          v14 = (unsigned int)(v10 < v12) - (unsigned int)(v12 < v10);
      }
    }
    v13 = v16;
    v11 = &v16[v12];
  }
  sub_fb4b(v13,v12,&v6);
  sub_fbaf(&v6,v8);
  sub_fb4b(v11,v10,&v7);
  sub_fbaf(&v7,v9);
  v15 = memcmp(v8,v9,0x10);
  if (!v15) {
    if (!v14) {
      v1 = v10;
      if (v12 < v10)
        v1 = v12;
      v14 = memcmp(v13,v11,v1);
      if (!v14)
        v14 = (unsigned int)(v10 < v12) - (unsigned int)(v12 < v10);
    }
    v15 = v14;
  }
  free(v18);
  return v15;
}


// Function: sub_7ee4 @ 0x7ee4
long sub_7ee4(char *a0,char *a1)
{
  char *v1; // stack - 0x20
  long v2; // stack - 0x10
  
  v2 = (long)(int)sub_1293d(a0,(long)a1 - (long)a0,0);
  for (v1 = a0; v1 < a1; v1 = &v1[1]) {
    v2 += (int)(unsigned int)(*v1 == '\t');
  }
  return v2;
}


// Function: sub_7f4c @ 0x7f4c
void sub_7f4c(long a0,long a1) // return-dupe
{
  long v1;
  long v2; // stack - 0x18
  long v3; // stack - 0x10
  
  v3 = a0;
  while (v1 = v3 + -1, v3) {
    putchar_unlocked(0x20);
    v3 = v1;
  }
  v2 = a1;
  if (!a1) {
    printf(gettext("^ no match for key\n"));
    return;
  }
  do {
    putchar_unlocked(0x5f);
    v2 -= 1;
  } while (v2);
  putchar_unlocked(10);
}


// Function: sub_7fc5 @ 0x7fc5
unsigned int sub_7fc5(long a0) // return-dupe
{
  if (((!*(char *)(a0 + 0x32)) && (!*(char *)(a0 + 0x34))) && (!*(char *)(a0 + 0x35)))
    return 0;
  return 1;
}


// Function: sub_8002 @ 0x8002
void sub_8002(unsigned long *a0,long *a1) // warn: iteboolean: re-rolled 2 0/1 select diamond(s) to a boolean assignment in sub_8002
{
  unsigned char v1;
  unsigned long v10; // stack - 0x38
  unsigned long v11; // stack - 0x30
  undefined10 v12; // stack - 0x28
  unsigned char v2;
  int v3; // eax
  unsigned char *v4; // stack - 0x60
  unsigned char *v5; // stack - 0x58
  undefined10 v6; // st0
  unsigned char *v7; // stack - 0x50
  unsigned char *v8; // stack - 0x48
  unsigned char *v9; // stack - 0x40
  
  v7 = (unsigned char *)*a0;
  v8 = &v7[a0[1] + -1];
  v9 = v7;
  if (a1) {
    if (*a1 != -1)
      v7 = (unsigned char *)sub_66f4(a0,a1);
    if (a1[2] != -1)
      v8 = (unsigned char *)sub_6898(a0,a1);
    if ((((char)a1[6]) && (*a1 == -1)) || ((*(char *)((long)a1 + 0x36) || (sub_7fc5(a1))))) {
      v1 = *v8;
      *v8 = 0;
      while (v2 = *v7, *(char *)((long)(int)(unsigned int)(unsigned char)sub_4042((int)(char)v2) + 0x23260)) {
        v7 = &v7[1];
      }
      v4 = v7;
      if (v7 <= v8) { // branch-flip
        if (*(char *)((long)a1 + 0x36)) // branch-flip
          sub_73f3(v7,&v4);
        else if (*(char *)((long)a1 + 0x34)) { // branch-flip
          strtold(v7,&v4);
          v12 = v6;
        }
        else if ((*(char *)((long)a1 + 0x32)) || (*(char *)((long)a1 + 0x35))) {
          v3 = !((v8 <= v7) || (*v7 != 0x2d));
          v5 = &v7[v3];
          if ('0' <= (char)sub_6f64(&v5)) {
            v3 = ((*(char *)((long)a1 + 0x35)) && (*(char *)((long)(int)(unsigned int)*v5 + 0x1c5c0)));
            v4 = &v5[v3];
          }
        }
        else {
          v4 = v8;
        }
      }
      else {
        v4 = v8;
      }
      *v8 = v1;
      v8 = v4;
    }
  }
  v10 = sub_7ee4(v9,v7);
  v11 = sub_7ee4(v7,v8);
  sub_7f4c(v10,v11);
}


// Function: sub_829e @ 0x829e
void sub_829e(unsigned long a0)
{
  long v1; // stack - 0x10
  
  v1 = dat_23688;
  while( true ) {
    do {
      sub_8002(a0,v1);
      if (!v1)
        return;
      v1 = *(long *)(v1 + 0x40);
    } while (v1);
    if (dat_23682 == '\x01') break;
    if (dat_23681 == '\x01')
      return;
  }
}


// Function: sub_8305 @ 0x8305
unsigned int sub_8305(long a0) // return-dupe
{
  if (((((!*(long *)(a0 + 0x20)) && (!*(long *)(a0 + 0x28))) && (*(char *)(a0 + 0x30) != '\x01')) && ((*(char *)(a0 + 0x31) != '\x01' && (sub_7fc5(a0) != '\x01')))) && ((*(char *)(a0 + 0x36) != '\x01' && ((*(char *)(a0 + 0x38) != '\x01' && (*(char *)(a0 + 0x33) != '\x01'))))))
    return 1;
  return 0;
}


// Function: sub_839e @ 0x839e
void sub_839e(long a0,char *a1)
{
  char *v1;
  char *v2; // stack - 0x18
  
  if ((*(char *)(a0 + 0x30)) || (v2 = a1, *(char *)(a0 + 0x31))) {
    v2 = &a1[1];
    *a1 = 0x62;
  }
  if (*(long *)(a0 + 0x20) == 0x23460) {
    v1 = &v2[1];
    *v2 = 100;
    v2 = v1;
  }
  if (*(long *)(a0 + 0x28)) {
    v1 = &v2[1];
    *v2 = 0x66;
    v2 = v1;
  }
  if (*(char *)(a0 + 0x34)) {
    v1 = &v2[1];
    *v2 = 0x67;
    v2 = v1;
  }
  if (*(char *)(a0 + 0x35)) {
    v1 = &v2[1];
    *v2 = 0x68;
    v2 = v1;
  }
  if (*(long *)(a0 + 0x20) == 0x23360) {
    v1 = &v2[1];
    *v2 = 0x69;
    v2 = v1;
  }
  if (*(char *)(a0 + 0x36)) {
    v1 = &v2[1];
    *v2 = 0x4d;
    v2 = v1;
  }
  if (*(char *)(a0 + 0x32)) {
    v1 = &v2[1];
    *v2 = 0x6e;
    v2 = v1;
  }
  if (*(char *)(a0 + 0x33)) {
    v1 = &v2[1];
    *v2 = 0x52;
    v2 = v1;
  }
  if (*(char *)(a0 + 0x37)) {
    v1 = &v2[1];
    *v2 = 0x72;
    v2 = v1;
  }
  if (*(char *)(a0 + 0x38)) {
    v1 = &v2[1];
    *v2 = 0x56;
    v2 = v1;
  }
  *v2 = 0;
}


// Function: sub_84fe @ 0x84fe
void sub_84fe(unsigned long *a0,char a1) // warn: iteboolean: re-rolled 3 0/1 select diamond(s) to a boolean assignment in sub_84fe
{
  unsigned long v1;
  char *v10; // rax
  char v11 [48];
  char v12 [56]; // stack - 0x58
  char v13 [32];
  unsigned long v14; // stack - 0xf8
  unsigned long *v15; // stack - 0x138
  long v16; // stack - 0x130
  unsigned long v17; // stack - 0x128
  unsigned long v18; // stack - 0x120
  unsigned long v19; // stack - 0xf0
  bool v2;
  unsigned long v20; // stack - 0xe8
  unsigned long v21; // stack - 0xe0
  unsigned long v22; // stack - 0xd8
  unsigned long v23; // stack - 0xd0
  unsigned long v24; // stack - 0xc8
  unsigned long v25; // stack - 0xc0
  unsigned long v26; // stack - 0xb8
  bool v3;
  bool v4; // al
  bool v5; // al
  char *v6; // rax
  char *v7; // rax
  unsigned long v8; // rax
  unsigned long v9; // rax
  
  v14 = *a0;
  v19 = a0[1];
  v20 = a0[2];
  v21 = a0[3];
  v22 = a0[4];
  v23 = a0[5];
  v24 = a0[6];
  v25 = a0[7];
  v26 = a0[8];
  v16 = 1;
  v2 = 0;
  v3 = 0;
  v15 = dat_23688;
  while( true ) {
    if (!v15) {
      if ((v2) || (v3)) {
        v12[0] = dat_23220;
        v12[1] = 0;
        v9 = sub_154a0(v12);
        if (dat_23220 != dat_23020) // branch-flip
          v10 = gettext("note ");
        else {
          v10 = "";
        }
        error(0,0,gettext("%snumbers use %s as a decimal point in this locale"),v10,v9); // no-return
      }
      if ((sub_8305(&v14) == '\x01') && ((!v24._7_1_ || (((!dat_23681 && (!dat_23682)) || (!dat_23688)))))) {
        if ((((v24._7_1_) && (dat_23681 != '\x01')) && (dat_23682 != '\x01')) && (dat_23688))
          error(0,0,gettext("option \'-r\' only applies to last-resort comparison")); // no-return
        return;
      }
      if ((dat_23681 != '\x01') && (dat_23682 != '\x01'))
        v24 &= 0xffffffffffffff;
      sub_839e(&v14,v12);
      v9 = sub_4094(strlen(v12));
      error(0,0,(char *)ngettext("option \'-%s\' is ignored","options \'-%s\' are ignored",v9),v12); // no-return
    }
    if (sub_7fc5(v15)) {
      if (*(char *)((long)v15 + 0x34)) // branch-flip
        v3 = 1;
      else {
        v2 = 1;
      }
    }
    if (*(char *)((long)v15 + 0x39)) {
      v17 = *v15;
      v1 = v15[2];
      if (v17 == 0xffffffffffffffff)
        v17 = 0;
      v10 = (char *)sub_1287e(v17,v13);
      v10 = stpcpy(stpcpy(v11,"+"),v10);
      v6 = (char *)sub_1287e(v17 + 1,v13);
      v6 = stpcpy(stpcpy(v12,"-k "),v6);
      if (v15[2] != 0xffffffffffffffff) {
        v7 = (char *)sub_1287e(v1 + 1,v13);
        stpcpy(stpcpy(v10," -"),v7);
        v10 = (char *)sub_1287e(v1 + (v15[3] == 0xffffffffffffffff) + 1,v13);
        stpcpy(stpcpy(v6,","),v10);
      }
      v9 = sub_15473(1,v12);
      v8 = sub_15473(0,v11);
      error(0,0,gettext("obsolescent key %s used; consider %s instead"),v8,v9); // no-return
    }
    v4 = ((*v15 != 0xffffffffffffffff) && (v15[2] < *v15));
    if (v4) break;
    v4 = ((sub_7fc5(v15)) || (*(char *)((long)v15 + 0x36)));
    v5 = !((v15[2]) || (!v15[3]));
    if ((((a1 != '\x01') && (dat_23020 == 0x80)) && (!v5)) && (((((char)v15[6] != '\x01' && (!v4)) || (((char)v15[6] != '\x01' && (v15[1])))) || ((*(char *)((long)v15 + 0x31) != '\x01' && (v15[3]))))))
      error(0,0,gettext("leading blanks are significant in key %lu; consider also specifying \'b\'"),v16); // no-return
    if ((a1 != '\x01') && (sub_7fc5(v15))) {
      v18 = *v15 + 1;
      if (!v18)
        v18 = *v15 + 2;
      if ((v15[2] == 0xffffffffffffffff) || (v18 < v15[2] + 1))
        error(0,0,gettext("key %lu is numeric and spans multiple fields"),v16); // no-return
    }
    if ((v22) && (v22 == v15[4]))
      v22 = 0;
    if ((v23) && (v23 == v15[5]))
      v23 = 0;
    v24._0_2_ = CONCAT11(((*(unsigned char *)((long)v15 + 0x31) ^ 1) & v24._1_1_) != 0,(((unsigned char)v15[6] ^ 1) & (unsigned char)v24) != 0);
    v24._0_3_ = CONCAT12(((*(unsigned char *)((long)v15 + 0x32) ^ 1) & v24._2_1_) != 0,(unsigned short)v24);
    v24._0_4_ = CONCAT13(((*(unsigned char *)((long)v15 + 0x33) ^ 1) & v24._3_1_) != 0,(undefined3)v24);
    v24 = CONCAT44(CONCAT22(CONCAT11(v24._7_1_,((*(unsigned char *)((long)v15 + 0x36) ^ 1) & v24._6_1_) != 0),CONCAT11(((*(unsigned char *)((long)v15 + 0x35) ^ 1) & v24._5_1_) != 0,((*(unsigned char *)((long)v15 + 0x34) ^ 1) & v24._4_1_) != 0)),(unsigned int)v24);
    v25 = CONCAT71(v25._1_7_,(((unsigned char)v15[7] ^ 1) & (unsigned char)v25) != 0);
    v24 = CONCAT17(((*(unsigned char *)((long)v15 + 0x37) ^ 1) & v24._7_1_) != 0,(undefined7)v24);
    v15 = (unsigned long *)v15[8];
    v16 += 1;
  }
  error(0,0,gettext("key %lu has zero width and will be ignored"),v16); // no-return
}


// Function: sub_9116 @ 0x9116
unsigned int sub_9116(long *a0,long *a1) // ternary x2
{
  char *v1;
  unsigned long v10;
  unsigned long v11; // rax
  unsigned long v12;
  char v13 [4008];
  char v14; // stack - 0x104e
  char v15; // stack - 0x104d
  unsigned int v16; // stack - 0x104c
  long *v17; // stack - 0x1048
  char *v18; // stack - 0x1040
  char *v19; // stack - 0x1038
  char *v2;
  char *v20; // stack - 0x1030
  char *v21; // stack - 0x1028
  char *v22; // stack - 0x1020
  char *v23; // stack - 0x1018
  unsigned long v24; // stack - 0x1010
  unsigned long v25; // stack - 0x1008
  char *v26; // stack - 0x1000
  unsigned long v27; // stack - 0xff8
  long v3;
  long v4;
  char v5;
  unsigned char v6; // al
  int v7; // eax
  char *v8;
  char *v9;
  
  v17 = dat_23688;
  v18 = (char *)a0[2];
  v19 = (char *)a1[2];
  v20 = (char *)a0[3];
  v21 = (char *)a1[3];
  do {
    v3 = v17[5];
    v4 = v17[4];
    v8 = v18;
    if (v18 <= v20)
      v8 = v20;
    v9 = v19;
    if (v19 <= v21)
      v9 = v21;
    v10 = (long)v8 - (long)v18;
    v11 = (long)v9 - (long)v19;
    if ((((dat_23229) || (sub_7fc5(v17))) || (*(char *)((long)v17 + 0x36))) || ((*(char *)((long)v17 + 0x33) || ((char)v17[7])))) {
      if ((v4) || (v3)) {
        v12 = v11 + v10 + 2;
        if (0xfa1 <= v12) { // branch-flip
          v26 = (char *)sub_18024(v12);
          v22 = v26;
        }
        else {
          v22 = v13;
          v26 = NULL;
        }
        v23 = &v22[v10 + 1];
        v24 = 0;
        for (v27 = 0; v27 < v10; v27 = v27 + 1) {
          if (v4) { // branch-flip
            v5 = v18[v27];
            if (*(char *)(v4 + (unsigned long)(unsigned char)sub_4042((int)v5)) != '\x01') goto label_9370;
          }
          else {
label_9370:
            if (v3) { // branch-flip
              v5 = v18[v27];
              v5 = *(char *)(v3 + (unsigned long)(unsigned char)sub_4042((int)v5));
            }
            else {
              v5 = v18[v27];
            }
            v12 = v24 + 1;
            v22[v24] = v5;
            v24 = v12;
          }
        }
        v22[v24] = '\0';
        v25 = 0;
        for (v27 = 0; v27 < v11; v27 = v27 + 1) {
          if (v4) { // branch-flip
            v5 = v19[v27];
            if (*(char *)(v4 + (unsigned long)(unsigned char)sub_4042((int)v5)) != '\x01') goto label_9469;
          }
          else {
label_9469:
            if (v3) { // branch-flip
              v5 = v19[v27];
              v5 = *(char *)(v3 + (unsigned long)(unsigned char)sub_4042((int)v5));
            }
            else {
              v5 = v19[v27];
            }
            v10 = v25 + 1;
            v23[v25] = v5;
            v25 = v10;
          }
        }
        v23[v25] = '\0';
      }
      else {
        v22 = v18;
        v14 = v18[v10];
        v18[v10] = '\0';
        v23 = v19;
        v15 = v19[v11];
        v19[v11] = '\0';
        v24 = v10;
        v25 = v11;
      }
      if (*(char *)((long)v17 + 0x32)) // branch-flip
        v16 = sub_71a3(v22,v23);
      else if (*(char *)((long)v17 + 0x34)) // branch-flip
        v16 = sub_72d6(v22,v23);
      else if (*(char *)((long)v17 + 0x35)) // branch-flip
        v16 = sub_70e2(v22,v23);
      else if (*(char *)((long)v17 + 0x36)) { // branch-flip
        v7 = sub_73f3(v22,0);
        v16 = v7 - sub_73f3(v23,0);
      }
      else if (*(char *)((long)v17 + 0x33)) // branch-flip
        v16 = sub_7755(v22,v24,v23,v25);
      else if ((char)v17[7]) // branch-flip
        v16 = sub_10461(v22,v24,v23,v25);
      else if (v24) { // branch-flip
        v16 = (v25) ? sub_187ec(v22,v24 + 1,v23,v25 + 1) : 1; // branch-flip
      }
      else {
        v16 = -(unsigned int)(v25 != 0);
      }
      if ((v4) || (v3))
        free(v26);
      else {
        v22[v24] = v14;
        v23[v25] = v15;
      }
    }
    else if (v4) { // branch-flip
      if (v3) { // branch-flip
        while( true ) {
          while ((v18 < v8 && (v5 = *v18, *(char *)(v4 + (unsigned long)(unsigned char)sub_4042((int)v5))))) {
            v18 = &v18[1];
          }
          while ((v19 < v9 && (v5 = *v19, *(char *)(v4 + (unsigned long)(unsigned char)sub_4042((int)v5))))) {
            v19 = &v19[1];
          }
          if ((v8 <= v18) || (v9 <= v19)) break;
          v5 = *v18;
          v6 = sub_4042((int)*(char *)(v3 + (unsigned long)(unsigned char)sub_4042((int)v5)));
          v5 = *v19;
          v5 = *(char *)(v3 + (unsigned long)(unsigned char)sub_4042((int)v5));
          v16 = (unsigned int)v6 - (unsigned int)(unsigned char)sub_4042((int)v5);
          if (v16) goto label_9df1;
          v18 = &v18[1];
          v19 = &v19[1];
        }
        v16 = (unsigned int)(v18 < v8) - (unsigned int)(v19 < v9);
      }
      else {
        while( true ) {
          while ((v18 < v8 && (v5 = *v18, *(char *)(v4 + (unsigned long)(unsigned char)sub_4042((int)v5))))) {
            v18 = &v18[1];
          }
          while ((v19 < v9 && (v5 = *v19, *(char *)(v4 + (unsigned long)(unsigned char)sub_4042((int)v5))))) {
            v19 = &v19[1];
          }
          if ((v8 <= v18) || (v9 <= v19)) break;
          v6 = sub_4042((int)*v18);
          v5 = *v19;
          v16 = (unsigned int)v6 - (unsigned int)(unsigned char)sub_4042((int)v5);
          if (v16) goto label_9df1;
          v18 = &v18[1];
          v19 = &v19[1];
        }
        v16 = (unsigned int)(v18 < v8) - (unsigned int)(v19 < v9);
      }
    }
    else if (v10) { // branch-flip
      if (!v11) {
        v16 = 1;
        goto label_9df1;
      }
      if (v3) { // branch-flip
        while ((v18 < v8 && (v19 < v9))) {
          v1 = &v18[1];
          v5 = *v18;
          v6 = sub_4042((int)*(char *)(v3 + (unsigned long)(unsigned char)sub_4042((int)v5)));
          v2 = &v19[1];
          v5 = *v19;
          v5 = *(char *)(v3 + (unsigned long)(unsigned char)sub_4042((int)v5));
          v16 = (unsigned int)v6 - (unsigned int)(unsigned char)sub_4042((int)v5);
          v18 = v1;
          v19 = v2;
          if (v16) goto label_9df1;
        }
      }
      else {
        v12 = v11;
        if (v10 < v11)
          v12 = v10;
        v16 = memcmp(v18,v19,v12);
        if (v16) goto label_9df1;
      }
      v16 = (v11 <= v10) ? (unsigned int)(v10 != v11) : 0xffffffff; // branch-flip
    }
    else {
      v16 = -(unsigned int)(v11 != 0);
    }
    if (v16) {
label_9df1:
      if (!*(char *)((long)v17 + 0x37))
        return v16;
      return -v16;
    }
    v17 = (long *)v17[8];
    if (!v17)
      return 0;
    if (v17[2] != -1) { // branch-flip
      v20 = (char *)sub_6898(a0,v17);
      v21 = (char *)sub_6898(a1,v17);
    }
    else {
      v20 = (char *)(a0[1] + -1 + *a0);
      v21 = (char *)(a1[1] + -1 + *a1);
    }
    if (*v17 != -1) { // branch-flip
      v18 = (char *)sub_66f4(a0,v17);
      v19 = (char *)sub_66f4(a1,v17);
    }
    else {
      v18 = (char *)*a0;
      v19 = (char *)*a1;
      if ((char)v17[6]) {
        while ((v18 < v20 && (v5 = *v18, *(char *)((long)(int)(unsigned int)(unsigned char)sub_4042((int)v5) + 0x23260)))) {
          v18 = &v18[1];
        }
        while ((v19 < v21 && (v5 = *v19, *(char *)((long)(int)(unsigned int)(unsigned char)sub_4042((int)v5) + 0x23260)))) {
          v19 = &v19[1];
        }
      }
    }
  } while( true );
}


// Function: sub_9e2a @ 0x9e2a
unsigned int sub_9e2a(unsigned long *a0,unsigned long *a1) // ternary
{
  long v1;
  long v2;
  unsigned long v3;
  unsigned long v4;
  unsigned int v5; // eax
  unsigned long v6; // rax
  unsigned long v7; // rax
  unsigned long v8;
  unsigned int v9; // stack - 0x1c
  
  if ((dat_23688) && (((v5 = sub_9116(a0,a1), v5 || (dat_23682)) || (dat_23681))))
    return v5;
  v1 = a0[1];
  v6 = v1 - 1;
  v2 = a1[1];
  v7 = v2 - 1;
  if (v6) { // branch-flip
    if (v7) { // branch-flip
      if (dat_23229) { // branch-flip
        v3 = *a1;
        v4 = *a0;
        v9 = sub_187ec(v4,v1,v3,v2);
      }
      else {
        v8 = v7;
        if (v6 < v7)
          v8 = v6;
        v9 = memcmp((void *)*a0,(void *)*a1,v8);
        if (!v9) {
          v9 = (v7 <= v6) ? (unsigned int)(v6 != v7) : 0xffffffff; // branch-flip
        }
      }
    }
    else {
      v9 = 1;
    }
  }
  else {
    v9 = -(unsigned int)(v7 != 0);
  }
  if (dat_23680)
    v9 = -v9;
  return v9;
}


// Function: sub_9f73 @ 0x9f73
void sub_9f73(unsigned long *a0,void *a1,long a2) // return-dupe
{
  unsigned long v1;
  char *v2; // rax
  char v3; // stack - 0x29
  char *v4; // stack - 0x28
  
  v4 = (char *)*a0;
  v1 = a0[1];
  v2 = &v4[v1];
  if ((!a2) && (dat_23698)) {
    do {
      if (v2 <= v4) {
        sub_829e(a0);
        return;
      }
      v3 = *v4;
      if (v3 != '\t') { // branch-flip
        if (&v4[1] == v2)
          v3 = '\n';
      }
      else {
        v3 = '>';
      }
      v4 = &v4[1];
    } while (fputc_unlocked((int)v3,a1) != -1);
    sub_4464(gettext("write failed"),0); // no-return
  }
  v2[-1] = dat_23010;
  if (v1 == fwrite_unlocked(v4,1,v1,a1)) {
    v2[-1] = '\0';
    return;
  }
  sub_4464(gettext("write failed"),a2); // no-return
}


// Function: sub_a0b1 @ 0xa0b1
char sub_a0b1(unsigned long a0,char a1)
{
  long *v1;
  void *v10; // stack - 0x98
  char v11 [24];
  bool v12; // zf
  char v13; // stack - 0xda
  unsigned long v14; // stack - 0xd8
  long v15; // stack - 0xd0
  long *v16; // stack - 0xc8
  long v17; // stack - 0x90
  long v18; // stack - 0x88
  long v19; // stack - 0x80
  long *v2;
  long v20; // stack - 0x68
  long v3;
  unsigned long v4;
  unsigned long v5; // rax
  long *v6; // rax
  unsigned long v7; // rax
  unsigned long v8;
  void *v9 [2]; // stack - 0x78
  
  v5 = sub_505e(a0,"r");
  v3 = dat_23688;
  v14 = 0;
  v15 = 0;
  v12 = dat_23682 == '\0';
  v13 = 1;
  v8 = dat_23660;
  if (dat_23660 < dat_23018)
    v8 = dat_23018;
  sub_6618(v9,0x20,v8);
  v10 = NULL;
  do {
    if (!sub_6a5e(v9,v5,a0)) {
label_a446:
      sub_50b7(v5,a0);
      free(v9[0]);
      free(v10);
      return v13;
    }
    v16 = (long *)sub_66d0(v9);
    v6 = &v16[v20 * -4];
    if ((v14) && ((int)(unsigned int)v12 <= (int)sub_9e2a(&v10,&v16[-4]))) {
label_a1f4:
      if (a1 == 'c') {
        v7 = sub_1287e(v15 + (sub_66d0(v9) - (long)&v16[-4] >> 5),v11);
        v4 = dat_23868;
        fprintf(stderr,gettext("%s: %s:%s: disorder: "),v4,a0,v7);
        sub_9f73(&v16[-4],stderr,gettext("standard error"));
      }
      v13 = 0;
      goto label_a446;
    }
    while (v2 = &v16[-4], v6 < v2) {
      v1 = &v16[-8];
      v16 = v2;
      if ((int)(unsigned int)v12 <= (int)sub_9e2a(v2,v1)) goto label_a1f4;
    }
    v15 += v20;
    if (v14 < (unsigned long)v16[-3]) {
      do {
        v14 <<= 1;
        if (!v14) {
          v14 = v16[-3];
          break;
        }
      } while (v14 < (unsigned long)v16[-3]);
      free(v10);
      v10 = (void *)sub_18024(v14);
    }
    memcpy(v10,(void *)*v2,v16[-3]);
    v17 = v16[-3];
    if (v3) {
      v18 = (v16[-2] - *v2) + (long)v10;
      v19 = (v16[-1] - *v2) + (long)v10;
    }
  } while( true );
}


// Function: sub_a4a1 @ 0xa4a1
long sub_a4a1(long a0,unsigned long a1,long *a2)
{
  long v1;
  unsigned long v2; // rax
  int v3; // stack - 0x14
  
  *a2 = sub_1819d(a1,8);
  v1 = *a2;
  for (v3 = 0; (unsigned long)(long)v3 < a1; v3 = v3 + 1) {
    if ((*(long *)(a0 + (long)v3 * 0x10 + 8)) && (*(char *)(*(long *)(a0 + (long)v3 * 0x10 + 8) + 0xc)))
      v2 = sub_551c(*(unsigned long *)(a0 + (long)v3 * 0x10 + 8));
    else {
      v2 = sub_4ee4(*(unsigned long *)(a0 + (long)v3 * 0x10),"r");
    }
    *(unsigned long *)(v1 + (long)v3 * 8) = v2;
    if (!*(long *)(v1 + (long)v3 * 8)) break;
  }
  return (long)v3;
}


// Function: sub_a5c0 @ 0xa5c0
void sub_a5c0(long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,void *a5)
{
  long *v1;
  long v10; // rax
  unsigned long *v11; // rax
  unsigned long *v12; // rcx
  unsigned long v13;
  void *v14; // stack - 0x48
  unsigned long v15; // stack - 0x100
  unsigned long v16; // stack - 0xf8
  void **v17; // stack - 0xd8
  unsigned long v18; // stack - 0xd0
  unsigned long v19; // stack - 0xc8
  unsigned long v2;
  unsigned long v20; // stack - 0xc0
  unsigned long v21; // stack - 0xb8
  unsigned long v22; // stack - 0xb0
  unsigned long v23; // stack - 0xa8
  unsigned long v24; // stack - 0x40
  long v25; // stack - 0x38
  long v26; // stack - 0x30
  unsigned long v3;
  long v4;
  int v5; // eax
  void *v6; // rax
  void *v7; // rax
  void *v8; // rax
  unsigned long *v9; // rax
  
  v6 = (void *)sub_1819d(a2,0x38);
  v17 = NULL;
  v18 = 0;
  v7 = (void *)sub_1819d(a2,8);
  v8 = (void *)sub_1819d(a2,8);
  v9 = (unsigned long *)sub_1819d(a2,8);
  v4 = dat_23688;
  v14 = NULL;
  v19 = 0;
  v15 = a2;
  v16 = a1;
  while (v19 < v15) {
    v13 = dat_23660 / v15;
    if (dat_23660 / v15 < dat_23018)
      v13 = dat_23018;
    sub_6618((void *)((long)v6 + v19 * 0x38),0x20,v13);
    v2 = *(unsigned long *)(a0 + v19 * 0x10);
    v3 = *(unsigned long *)((long)a5 + v19 * 8);
    if (sub_6a5e((void *)((long)v6 + v19 * 0x38),v3,v2)) { // branch-flip
      v10 = sub_66d0((void *)((long)v6 + v19 * 0x38));
      *(long *)((long)v7 + v19 * 8) = v10 + -0x20;
      *(long *)((long)v8 + v19 * 8) = v10 + *(long *)((long)v6 + v19 * 0x38 + 0x10) * -0x20;
      v19 += 1;
    }
    else {
      sub_50b7(*(unsigned long *)((long)a5 + v19 * 8),*(unsigned long *)(a0 + v19 * 0x10));
      if (v19 < v16) {
        v16 -= 1;
        sub_5782(*(unsigned long *)(a0 + v19 * 0x10));
      }
      free(*(void **)((long)v6 + v19 * 0x38));
      v15 -= 1;
      for (v20 = v19; v20 < v15; v20 = v20 + 1) {
        v11 = (unsigned long *)(a0 + (v20 + 1) * 0x10);
        v12 = (unsigned long *)(v20 * 0x10 + a0);
        v2 = v11[1];
        *v12 = *v11;
        v12[1] = v2;
        *(unsigned long *)((long)a5 + v20 * 8) = *(unsigned long *)((long)a5 + (v20 + 1) * 8);
      }
    }
  }
  for (v19 = 0; v19 < v15; v19 = v19 + 1) {
    v9[v19] = v19;
  }
  for (v19 = 1; v19 < v15; v19 = v19 + 1) {
    v2 = *(unsigned long *)((long)v7 + v9[v19] * 8);
    v3 = *(unsigned long *)((long)v7 + v9[v19 - 1] * 8);
    if (1 <= (int)sub_9e2a(v3,v2)) {
      v13 = v9[v19 - 1];
      v9[v19 - 1] = v9[v19];
      v9[v19] = v13;
      v19 = 0;
    }
  }
label_b27c:
  do {
    if (!v15) {
      if ((dat_23682) && (v17)) {
        sub_9f73(&v14,a3,a4);
        free(v14);
      }
      sub_50b7(a3,a4);
      free(a5);
      free(v6);
      free(v9);
      free(v8);
      free(v7);
      return;
    }
    v1 = *(long **)((long)v7 + *v9 * 8);
    if (dat_23682) { // branch-flip
      if ((v17) && (sub_9e2a(v17,v1))) {
        v17 = NULL;
        sub_9f73(&v14,a3,a4);
      }
      if (!v17) {
        v17 = &v14;
        if (v18 < (unsigned long)v1[1]) {
          do {
            if (!v18) {
              v18 = v1[1];
              break;
            }
            v18 <<= 1;
          } while (v18 < (unsigned long)v1[1]);
          free(v14);
          v14 = (void *)sub_18024(v18);
        }
        v24 = v1[1];
        memcpy(v14,(void *)*v1,v24);
        if (v4) {
          v25 = (v1[2] - *v1) + (long)v14;
          v26 = (v1[3] - *v1) + (long)v14;
        }
      }
    }
    else {
      sub_9f73(v1,a3,a4);
    }
    if (v1 <= *(long **)((long)v8 + *v9 * 8)) { // branch-flip
      v2 = *(unsigned long *)(a0 + *v9 * 0x10);
      v3 = *(unsigned long *)((long)a5 + *v9 * 8);
      v13 = *v9;
      if (!sub_6a5e((void *)((long)v6 + v13 * 0x38),v3,v2)) {
        for (v19 = 1; v19 < v15; v19 = v19 + 1) {
          if (*v9 < v9[v19])
            v9[v19] = v9[v19] - 1;
        }
        v15 -= 1;
        sub_50b7(*(unsigned long *)((long)a5 + *v9 * 8),*(unsigned long *)(a0 + *v9 * 0x10));
        if (*v9 < v16) {
          v16 -= 1;
          sub_5782(*(unsigned long *)(a0 + *v9 * 0x10));
        }
        free(*(void **)((long)v6 + *v9 * 0x38));
        for (v19 = *v9; v19 < v15; v19 = v19 + 1) {
          *(unsigned long *)((long)a5 + v19 * 8) = *(unsigned long *)((long)a5 + (v19 + 1) * 8);
          v11 = (unsigned long *)(a0 + (v19 + 1) * 0x10);
          v12 = (unsigned long *)(v19 * 0x10 + a0);
          v2 = v11[1];
          *v12 = *v11;
          v12[1] = v2;
          v12 = (unsigned long *)((v19 + 1) * 0x38 + (long)v6);
          v11 = (unsigned long *)((long)v6 + v19 * 0x38);
          v2 = v12[1];
          *v11 = *v12;
          v11[1] = v2;
          v2 = v12[3];
          v11[2] = v12[2];
          v11[3] = v2;
          v2 = v12[5];
          v11[4] = v12[4];
          v11[5] = v2;
          v11[6] = v12[6];
          *(unsigned long *)((long)v7 + v19 * 8) = *(unsigned long *)((long)v7 + (v19 + 1) * 8);
          *(unsigned long *)((long)v8 + v19 * 8) = *(unsigned long *)((long)v8 + (v19 + 1) * 8);
        }
        for (v19 = 0; v19 < v15; v19 = v19 + 1) {
          v9[v19] = v9[v19 + 1];
        }
        goto label_b27c;
      }
      v10 = sub_66d0((void *)((long)v6 + *v9 * 0x38));
      *(long *)((long)v7 + *v9 * 8) = v10 + -0x20;
      *(long *)((long)v8 + *v9 * 8) = v10 + *(long *)((long)v6 + *v9 * 0x38 + 0x10) * -0x20;
    }
    else {
      *(long **)((long)v7 + *v9 * 8) = &v1[-4];
    }
    v21 = 1;
    v22 = v15;
    v23 = 1;
    v13 = *v9;
    while (v21 < v22) {
      v5 = sub_9e2a(*(unsigned long *)((long)v7 + v13 * 8),*(unsigned long *)((long)v7 + v9[v23] * 8));
      if ((0 <= v5) && ((v5 || (v9[v23] <= v13))))
        v21 = v23 + 1;
      else {
        v22 = v23;
      }
      v23 = v22 + v21 >> 1;
    }
    for (v20 = 0; v20 < v21 - 1; v20 = v20 + 1) {
      v9[v20] = v9[v20 + 1];
    }
    v9[v21 - 1] = v13;
  } while( true );
}


// Function: sub_b344 @ 0xb344
unsigned long sub_b344(long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4)
{
  unsigned long v1;
  unsigned long v2; // stack - 0x30
  unsigned long v3; // stack - 0x28
  
  v3 = sub_a4a1(a0,a2,&v2);
  if ((v3 < a2) && (v3 <= 1)) {
    v1 = *(unsigned long *)(a0 + v3 * 0x10);
    sub_4464(gettext("open failed"),v1); // no-return
  }
  sub_a5c0(a0,a1,v3,a3,a4,v2);
  return v3;
}


// Function: sub_b413 @ 0xb413
void sub_b413(unsigned long *a0,unsigned long a1,unsigned long *a2) // return-dupe
{
  unsigned long *v1;
  unsigned long v2;
  unsigned long *v3; // stack - 0x40
  unsigned long *v4; // stack - 0x30
  unsigned long v5; // stack - 0x20
  long v6; // stack - 0x18
  unsigned long *v7; // stack - 0x10
  
  v5 = a1 >> 1;
  v6 = a1 - v5;
  v3 = a2;
  v4 = a0;
  v7 = &a0[v5 * -4];
  do {
    while ((int)sub_9e2a(&v3[-4],&v7[-4]) <= 0) {
      v1 = &v4[-4];
      v2 = v3[-3];
      *v1 = v3[-4];
      v4[-3] = v2;
      v2 = v3[-1];
      v4[-2] = v3[-2];
      v4[-1] = v2;
      v5 -= 1;
      v3 = &v3[-4];
      v4 = v1;
      if (!v5)
        return;
    }
    v1 = &v4[-4];
    v2 = v7[-3];
    *v1 = v7[-4];
    v4[-3] = v2;
    v2 = v7[-1];
    v4[-2] = v7[-2];
    v4[-1] = v2;
    v6 -= 1;
    v4 = v1;
    v7 = &v7[-4];
  } while (v6);
  do {
    v1 = &v4[-4];
    v2 = v3[-3];
    *v1 = v3[-4];
    v4[-3] = v2;
    v2 = v3[-1];
    v4[-2] = v3[-2];
    v4[-1] = v2;
    v5 -= 1;
    v3 = &v3[-4];
    v4 = v1;
  } while (v5);
}


// Function: sub_b536 @ 0xb536
void sub_b536(long a0,unsigned long a1,long a2,char a3) // return-dupe x2
{
  unsigned long *v1;
  unsigned long v2;
  unsigned int v3; // eax
  unsigned long v4; // rax
  long v5; // rax
  long v6; // stack - 0x38
  long v7; // stack - 0x30
  
  if (a1 != 2) {
    v4 = a1 >> 1;
    if (a3) // branch-flip
      v5 = v4 * -0x20;
    else {
      v5 = 0;
    }
    sub_b536(a0 + v4 * -0x20,a1 - v4,v5 + a2,a3);
    if (2 <= v4) // branch-flip
      sub_b536(a0,v4,a2,a3 == '\0');
    else if (a3 != '\x01') {
      v2 = *(unsigned long *)(a0 + -0x18);
      *(unsigned long *)(a2 + -0x20) = *(unsigned long *)(a0 + -0x20);
      *(unsigned long *)(a2 + -0x18) = v2;
      v2 = *(unsigned long *)(a0 + -8);
      *(unsigned long *)(a2 + -0x10) = *(unsigned long *)(a0 + -0x10);
      *(unsigned long *)(a2 + -8) = v2;
    }
    if (a3) { // branch-flip
      v6 = a2;
      v7 = a0;
    }
    else {
      v6 = a0;
      v7 = a2;
    }
    sub_b413(v6,a1,v7);
    return;
  }
  v3 = (unsigned int)(0 < (int)sub_9e2a(a0 + -0x20,a0 + -0x40));
  if (a3) {
    v1 = (unsigned long *)((long)(int)~v3 * 0x20 + a0);
    v2 = v1[1];
    *(unsigned long *)(a2 + -0x20) = *v1;
    *(unsigned long *)(a2 + -0x18) = v2;
    v2 = v1[3];
    *(unsigned long *)(a2 + -0x10) = v1[2];
    *(unsigned long *)(a2 + -8) = v2;
    v1 = (unsigned long *)((long)(int)(v3 - 2) * 0x20 + a0);
    v2 = v1[1];
    *(unsigned long *)(a2 + -0x40) = *v1;
    *(unsigned long *)(a2 + -0x38) = v2;
    v2 = v1[3];
    *(unsigned long *)(a2 + -0x30) = v1[2];
    *(unsigned long *)(a2 + -0x28) = v2;
    return;
  }
  if (v3) {
    v2 = *(unsigned long *)(a0 + -0x18);
    *(unsigned long *)(a2 + -0x20) = *(unsigned long *)(a0 + -0x20);
    *(unsigned long *)(a2 + -0x18) = v2;
    v2 = *(unsigned long *)(a0 + -8);
    *(unsigned long *)(a2 + -0x10) = *(unsigned long *)(a0 + -0x10);
    *(unsigned long *)(a2 + -8) = v2;
    *(unsigned long *)(a0 + -0x20) = *(unsigned long *)(a0 + -0x40);
    *(unsigned long *)(a0 + -0x18) = *(unsigned long *)(a0 + -0x38);
    *(unsigned long *)(a0 + -0x10) = *(unsigned long *)(a0 + -0x30);
    *(unsigned long *)(a0 + -8) = *(unsigned long *)(a0 + -0x28);
    v2 = *(unsigned long *)(a2 + -0x18);
    *(unsigned long *)(a0 + -0x40) = *(unsigned long *)(a2 + -0x20);
    *(unsigned long *)(a0 + -0x38) = v2;
    v2 = *(unsigned long *)(a2 + -8);
    *(unsigned long *)(a0 + -0x30) = *(unsigned long *)(a2 + -0x10);
    *(unsigned long *)(a0 + -0x28) = v2;
    return;
  }
}


// Function: sub_b7b6 @ 0xb7b6
unsigned long * sub_b7b6(long a0,unsigned long a1,unsigned long a2)
{
  unsigned long *v1; // rax
  
  v1 = (unsigned long *)sub_18024(a0 << 8);
  v1[3] = 0;
  v1[2] = v1[3];
  v1[1] = v1[2];
  *v1 = v1[1];
  v1[4] = 0;
  v1[6] = a1;
  v1[5] = v1[6];
  v1[7] = 0;
  *(unsigned int *)&v1[10] = 0;
  *(char *)((long)v1 + 0x54) = 0;
  pthread_mutex_init(&v1[0xb],0);
  sub_b911(v1,&v1[0x10],a2,a0,a1,0);
  return v1;
}


// Function: sub_b8b3 @ 0xb8b3
void sub_b8b3(long a0,void *a1)
{
  long v1;
  long v2; // stack - 0x18
  void *v3; // stack - 0x10
  
  v2 = a0 * 2;
  v3 = a1;
  while( true ) {
    v1 = v2 + -1;
    if (!v2) break;
    pthread_mutex_destroy((long)v3 + 0x58);
    v3 = (void *)((long)v3 + 0x80);
    v2 = v1;
  }
  free(a1);
}


// Function: sub_b911 @ 0xb911
unsigned long * sub_b911(long a0,unsigned long *a1,long a2,unsigned long a3,long a4,char a5) // ternary x2
{
  unsigned long v1;
  unsigned long v2; // rax
  long v3; // rax
  long v4; // rax
  unsigned long v5; // rax
  unsigned long *v6; // stack - 0x68
  
  v1 = (a5) ? *(unsigned long *)(a0 + 0x28) : *(unsigned long *)(a0 + 0x30); // branch-flip
  v2 = v1 >> 1;
  a2 += a4 * -0x20;
  v3 = a2 + v2 * -0x20;
  v4 = (a5) ? a0 + 0x10 : a0 + 0x18; // branch-flip
  v6 = &a1[0x10];
  a1[2] = a2;
  *a1 = a1[2];
  a1[3] = v3;
  a1[1] = a1[3];
  a1[4] = v4;
  a1[5] = v2;
  a1[6] = v1 - v2;
  a1[7] = a0;
  *(int *)&a1[10] = *(int *)(a0 + 0x50) + 1;
  *(char *)((long)a1 + 0x54) = 0;
  pthread_mutex_init(&a1[0xb],0);
  if (2 <= a3) { // branch-flip
    a1[8] = v6;
    v5 = sub_b911(a1,v6,a2,a3 >> 1,a4,1);
    a1[9] = v5;
    v6 = (unsigned long *)sub_b911(a1,v5,v3,a3 - (a3 >> 1),a4,0);
  }
  else {
    a1[8] = 0;
    a1[9] = 0;
  }
  return v6;
}


// Function: sub_bb0c @ 0xbb0c
bool sub_bb0c(long a0,long a1) // ternary
{
  bool v1; // cf
  
  v1 = (*(int *)(a0 + 0x50) != *(int *)(a1 + 0x50)) ? *(unsigned int *)(a0 + 0x50) < *(unsigned int *)(a1 + 0x50) : (unsigned long)(*(long *)(a0 + 0x28) + *(long *)(a0 + 0x30)) < (unsigned long)(*(long *)(a1 + 0x30) + *(long *)(a1 + 0x28)); // branch-flip
  return v1;
}


// Function: sub_bb87 @ 0xbb87
void sub_bb87(long a0)
{
  pthread_mutex_lock((void *)(a0 + 0x58));
}


// Function: sub_bba6 @ 0xbba6
void sub_bba6(long a0)
{
  pthread_mutex_unlock((void *)(a0 + 0x58));
}


// Function: sub_bbc5 @ 0xbbc5
void sub_bbc5(unsigned long *a0)
{
  sub_12398(*a0);
  pthread_cond_destroy(&a0[6]);
  pthread_mutex_destroy(&a0[1]);
}


// Function: sub_bc07 @ 0xbc07
void sub_bc07(unsigned long *a0,long a1)
{
  *a0 = sub_122ed(sub_bb0c,a1 * 2);
  pthread_mutex_init(&a0[1],0);
  pthread_cond_init(&a0[6],0);
}


// Function: sub_bc6b @ 0xbc6b
void sub_bc6b(unsigned long *a0,long a1)
{
  pthread_mutex_lock(&a0[1]);
  sub_123c6(*a0,a1);
  *(char *)(a1 + 0x54) = 1;
  pthread_cond_signal(&a0[6]);
  pthread_mutex_unlock(&a0[1]);
}


// Function: sub_bcd0 @ 0xbcd0
long sub_bcd0(unsigned long *a0)
{
  long v1; // rax
  
  pthread_mutex_lock(&a0[1]);
  while( true ) {
    v1 = sub_12474(*a0);
    if (v1) break;
    pthread_cond_wait(&a0[6],&a0[1]);
  }
  pthread_mutex_unlock(&a0[1]);
  sub_bb87(v1);
  *(char *)(v1 + 0x54) = 0;
  return v1;
}


// Function: sub_bd51 @ 0xbd51
void sub_bd51(long *a0,unsigned long a1,unsigned long a2)
{
  if (dat_23682) {
    if ((dat_23240) && (!sub_9e2a(a0,0x23240)))
      return;
    dat_23240 = *a0;
    dat_23248 = a0[1];
    dat_23250 = a0[2];
    dat_23258 = a0[3];
  }
  sub_9f73(a0,a1,a2);
}


// Function: sub_bde5 @ 0xbde5
void sub_bde5(long *a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  long v1;
  long v10; // stack - 0x38
  unsigned long *v11; // stack - 0x30
  unsigned long *v2;
  long v3;
  long v4;
  long v5;
  unsigned long *v6;
  unsigned long v7;
  long v8;
  bool v9; // zf
  
  v3 = *a0;
  v4 = a0[1];
  v10 = (a1 >> (((char)(int)a0[10] + '\x01') * '\x02' & 0x3fU)) + 1;
  if (2 <= *(unsigned int *)&a0[10]) { // branch-flip
    v11 = *(unsigned long **)a0[4];
    while (((*a0 != a0[2] && (a0[1] != a0[3])) && (v1 = v10 + -1, v9 = v10 != 0, v10 = v1, v9))) {
      v1 = a0[1];
      v5 = *a0;
      if (1 <= (int)sub_9e2a(v5 + -0x20,v1 + -0x20)) { // branch-flip
        a0[1] = a0[1] + -0x20;
        v6 = (unsigned long *)a0[1];
        v2 = &v11[-4];
        v7 = v6[1];
        *v2 = *v6;
        v11[-3] = v7;
        v7 = v6[3];
        v11[-2] = v6[2];
        v11[-1] = v7;
        v11 = v2;
      }
      else {
        *a0 = *a0 + -0x20;
        v6 = (unsigned long *)*a0;
        v2 = &v11[-4];
        v7 = v6[1];
        *v2 = *v6;
        v11[-3] = v7;
        v7 = v6[3];
        v11[-2] = v6[2];
        v11[-1] = v7;
        v11 = v2;
      }
    }
    if (v4 - a0[1] >> 5 != a0[6]) { // branch-flip
      if (v3 - *a0 >> 5 == a0[5]) {
        while ((a0[1] != a0[3] && (v1 = v10 + -1, v10))) {
          a0[1] = a0[1] + -0x20;
          v6 = (unsigned long *)a0[1];
          v2 = &v11[-4];
          v7 = v6[1];
          *v2 = *v6;
          v11[-3] = v7;
          v7 = v6[3];
          v11[-2] = v6[2];
          v11[-1] = v7;
          v10 = v1;
          v11 = v2;
        }
      }
    }
    else {
      while ((*a0 != a0[2] && (v1 = v10 + -1, v10))) {
        *a0 = *a0 + -0x20;
        v6 = (unsigned long *)*a0;
        v2 = &v11[-4];
        v7 = v6[1];
        *v2 = *v6;
        v11[-3] = v7;
        v7 = v6[3];
        v11[-2] = v6[2];
        v11[-1] = v7;
        v10 = v1;
        v11 = v2;
      }
    }
    *(unsigned long **)a0[4] = v11;
  }
  else {
    while (((*a0 != a0[2] && (a0[1] != a0[3])) && (v1 = v10 + -1, v9 = v10 != 0, v10 = v1, v9))) {
      v5 = a0[1];
      v8 = *a0;
      if (1 <= (int)sub_9e2a(v8 + -0x20,v5 + -0x20)) { // branch-flip
        a0[1] = a0[1] + -0x20;
        sub_bd51(a0[1],a2,a3);
      }
      else {
        *a0 = *a0 + -0x20;
        sub_bd51(*a0,a2,a3);
      }
    }
    if (v4 - a0[1] >> 5 != a0[6]) { // branch-flip
      if (v3 - *a0 >> 5 == a0[5]) {
        while ((a0[1] != a0[3] && (v1 = v10 + -1, v10))) {
          a0[1] = a0[1] + -0x20;
          sub_bd51(a0[1],a2,a3);
          v10 = v1;
        }
      }
    }
    else {
      while ((*a0 != a0[2] && (v1 = v10 + -1, v10))) {
        *a0 = *a0 + -0x20;
        sub_bd51(*a0,a2,a3);
        v10 = v1;
      }
    }
  }
  a0[5] = a0[5] - (v3 - *a0 >> 5);
  a0[6] = a0[6] - (v4 - a0[1] >> 5);
}


// Function: sub_c28e @ 0xc28e
void sub_c28e(unsigned long a0,long *a1) // return-dupe x2
{
  bool v1;
  
  if (*(char *)((long)a1 + 0x54) == '\x01')
    return;
  if (*a1 != a1[2]) { // branch-flip
    if ((a1[1] != a1[3]) || (!a1[6]))
      v1 = 1;
    else {
      v1 = 0;
    }
  }
  else if ((a1[1] == a1[3]) || (a1[5]))
    v1 = 0;
  else {
    v1 = 1;
  }
  if (v1) {
    sub_bc6b(a0,a1);
    return;
  }
}


// Function: sub_c34c @ 0xc34c
void sub_c34c(unsigned long a0,long a1) // return-dupe
{
  if (2 <= *(unsigned int *)(a1 + 0x50)) {
    sub_bb87(*(unsigned long *)(a1 + 0x38));
    sub_c28e(a0,*(unsigned long *)(a1 + 0x38));
    sub_bba6(*(unsigned long *)(a1 + 0x38));
    return;
  }
  if (!(*(long *)(a1 + 0x30) + *(long *)(a1 + 0x28))) {
    sub_bc6b(a0,*(unsigned long *)(a1 + 0x38));
    return;
  }
}


// Function: sub_c3d7 @ 0xc3d7
void sub_c3d7(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  long v1; // rax
  
  while( true ) {
    v1 = sub_bcd0(a0);
    if (!*(int *)(v1 + 0x50)) break;
    sub_bde5(v1,a1,a2,a3);
    sub_c28e(a0,v1);
    sub_c34c(a0,v1);
    sub_bba6(v1);
  }
  sub_bba6(v1);
  sub_bc6b(a0,v1);
}


// Function: sub_c481 @ 0xc481
unsigned long sub_c481(unsigned long *a0)
{
  sub_c4e8(*a0,a0[1],a0[2],a0[3],a0[4],a0[5],a0[6]);
  return 0;
}


// Function: sub_c4e8 @ 0xc4e8
void sub_c4e8(long a0,unsigned long a1,long a2,long *a3,unsigned long a4,unsigned long a5,unsigned long a6) // return-dupe
{
  unsigned long v1;
  unsigned long v10; // stack - 0x40
  long v11; // stack - 0x38
  long v12; // stack - 0x30
  unsigned long v13; // stack - 0x28
  unsigned long v14; // stack - 0x20
  unsigned long v15; // stack - 0x18
  long v2; // stack - 0x48
  unsigned long v3; // stack - 0x80
  unsigned long v4; // stack - 0x78
  unsigned long v5; // stack - 0x70
  long v6; // stack - 0x68
  unsigned long v7; // stack - 0x60
  unsigned long v8; // stack - 0x58
  long v9; // stack - 0x50
  
  v1 = a6;
  v4 = a3[6] + a3[5];
  v5 = a1 >> 1;
  v6 = a1 - v5;
  v12 = a3[8];
  v15 = a6;
  v2 = a0;
  v10 = v5;
  v11 = a2;
  v13 = a4;
  v14 = a5;
  if (((2 <= a1) && (0x20000 <= v4)) && (!pthread_create(&v3,0,sub_c481,&v2))) {
    sub_c4e8(a3[5] * -0x20 + a0,v6,a2,a3[9],a4,a5,v1);
    pthread_join(v3,0);
    return;
  }
  v7 = a3[5];
  v8 = a3[6];
  v9 = a0 + a2 * -0x20;
  if (2 <= v8)
    sub_b536(v7 * -0x20 + a0,v8,(v7 >> 1) * -0x20 + v9,0);
  if (2 <= v7)
    sub_b536(a0,v7,v9,0);
  *a3 = a0;
  a3[1] = v7 * -0x20 + a0;
  a3[2] = v7 * -0x20 + a0;
  a3[3] = v7 * -0x20 + v8 * -0x20 + a0;
  sub_bc6b(a4,a3);
  sub_c3d7(a4,a2,a5,v1);
}


// Function: sub_c7fa @ 0xc7fa
void sub_c7fa(long a0,unsigned long a1,unsigned long a2,char *a3) // warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_c7fa
{
  char *v1;
  int v2; // eax
  long v3; // stack - 0xa8
  unsigned long v4; // stack - 0xc8
  bool v5;
  long v6; // stack - 0xc0
  unsigned long v7; // stack - 0xb8
  long *v8; // stack - 0xb0
  long v9; // stack - 0xa0
  
  v6 = 0;
  v7 = a1;
  do {
    if (a2 <= v7)
      return;
    v1 = *(char **)(a0 + v7 * 0x10);
    v5 = strcmp(v1,"-") == 0;
    if (((!a3) || (v1 = *(char **)(a0 + v7 * 0x10), strcmp(a3,v1))) || (v5)) {
      v8 = (long *)sub_4e8c();
      if (!v8)
        return;
      if (v5)
        v2 = fstat(0,&v3);
      else {
        v2 = stat(*(char **)(a0 + v7 * 0x10),&v3);
      }
      v5 = !(((v2) || (v9 != v8[1])) || (v3 != *v8));
    }
    else {
      v5 = 1;
    }
    if (v5) {
      if (!v6) {
        v6 = sub_54f9(&v4);
        sub_b344(v7 * 0x10 + a0,0,1,v4,v6 + 0xd);
      }
      *(long *)(a0 + v7 * 0x10) = v6 + 0xd;
      *(long *)(v7 * 0x10 + a0 + 8) = v6;
    }
    v7 += 1;
  } while( true );
}


// Function: sub_ca81 @ 0xca81
void sub_ca81(long a0,unsigned long a1)
{
  char *v1;
  unsigned long v2;
  unsigned long v3; // stack - 0x20
  
  v3 = 0;
  do {
    if (a1 <= v3)
      return;
    v1 = *(char **)(a0 + v3 * 8);
    if (strcmp(v1,"-")) {
      v2 = *(unsigned long *)(a0 + v3 * 8);
      if (euidaccess(v2,4)) {
        v2 = *(unsigned long *)(a0 + v3 * 8);
        sub_4464(gettext("cannot read"),v2); // no-return
      }
    }
    v3 += 1;
  } while( true );
}


// Function: sub_cb44 @ 0xcb44
void sub_cb44(char *a0) // return-dupe
{
  int v1; // eax
  
  if (!a0)
    return;
  v1 = open(a0,0x80041,0x1b6);
  if (0 <= v1) {
    sub_515d(v1,1);
    return;
  }
  sub_4464(gettext("open failed"),a0); // no-return
}


// Function: sub_cbb9 @ 0xcbb9
void sub_cbb9(long *a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  long v1;
  long v10; // stack - 0x68
  long v11; // stack - 0x60
  unsigned long v12; // stack - 0x58
  unsigned long v13; // stack - 0x50
  unsigned long v14; // stack - 0x48
  long v15; // stack - 0x40
  unsigned long v16; // stack - 0x38
  long v17; // stack - 0x30
  unsigned long v18; // stack - 0x28
  unsigned long v2;
  unsigned long v3; // stack - 0x88
  long v4; // stack - 0x90
  unsigned long v5; // stack - 0xb0
  unsigned long v6; // stack - 0xa8
  long v7; // stack - 0x80
  unsigned long v8; // stack - 0x78
  unsigned long v9; // stack - 0x70
  
  v6 = a1;
  for (v5 = a2; dat_23024 < v5; v5 = v5 + (v8 - v7)) {
    v8 = 0;
    for (v7 = 0; (unsigned long)dat_23024 <= v5 - v7; v7 = v7 + v18) {
      v17 = sub_54f9(&v3);
      v2 = v6;
      if (dat_23024 <= v6)
        v2 = (unsigned long)dat_23024;
      v18 = sub_b344(&a0[v7 * 2],v2,dat_23024,v3,v17 + 0xd);
      v2 = v6;
      if (v18 <= v6)
        v2 = v18;
      v6 -= v2;
      a0[v8 * 2] = v17 + 0xd;
      a0[v8 * 2 + 1] = v17;
      v8 += 1;
    }
    v12 = v5 - v7;
    v13 = (unsigned long)dat_23024 - v8 % (unsigned long)dat_23024;
    if (v13 < v12) {
      v14 = (v12 - v13) + 1;
      v15 = sub_54f9(&v3);
      v2 = v6;
      if (v14 <= v6)
        v2 = v14;
      v16 = sub_b344(&a0[v7 * 2],v2,v14,v3,v15 + 0xd);
      v2 = v6;
      if (v16 <= v6)
        v2 = v16;
      v6 -= v2;
      a0[v8 * 2] = v15 + 0xd;
      v2 = v8 + 1;
      a0[v8 * 2 + 1] = v15;
      v7 += v16;
      v8 = v2;
    }
    memmove(&a0[v8 * 2],&a0[v7 * 2],(v5 - v7) * 0x10);
    v6 += v8;
  }
  sub_c7fa(a0,v6,v5,a3);
  do {
    v9 = sub_a4a1(a0,v5,&v4);
    if (v9 != v5) { // branch-flip
      if (v9 <= 2) {
        v1 = a0[v9 * 2];
        sub_4464(gettext("open failed"),v1); // no-return
      }
    }
    else {
      v10 = sub_4ee4(a3,"w");
      if (v10) {
        sub_a5c0(a0,v6,v5,v10,a3,v4);
        return;
      }
      if ((*__errno_location() != 0x18) || (v9 <= 2))
        sub_4464(gettext("open failed"),a3); // no-return
    }
    do {
      v9 -= 1;
      sub_50b7(*(unsigned long *)(v9 * 8 + v4),a0[v9 * 2]);
      v11 = sub_5372(&v3,3 <= v9);
    } while (!v11);
    v2 = v6;
    if (v9 <= v6)
      v2 = v9;
    sub_a5c0(a0,v2,v9,v3,v11 + 0xd,v4);
    v2 = v6;
    if (v9 <= v6)
      v2 = v9;
    *a0 = v11 + 0xd;
    a0[1] = v11;
    memmove(&a0[2],&a0[v9 * 2],(v5 - v9) * 0x10);
    v6 = (v6 - v2) + 1;
    v5 = (v5 - v9) + 1;
  } while( true );
}


// Function: sub_d0f9 @ 0xd0f9
void sub_d0f9(unsigned long *a0,long a1,long a2,unsigned long a3)
{
  bool v1;
  long v10; // stack - 0x110
  long v11; // stack - 0x108
  long v12; // stack - 0x100
  unsigned long v13; // stack - 0xf8
  long v14; // stack - 0xf0
  unsigned long *v15; // stack - 0xe8
  long v16; // stack - 0xe0
  unsigned long v17; // stack - 0xd8
  long v18; // stack - 0xd0
  long v19; // stack - 0xc8
  char v2; // al
  void *v20; // stack - 0xc0
  long v21; // stack - 0xb0
  unsigned long v22; // stack - 0xa8
  long v23; // stack - 0xa0
  long v24; // stack - 0x98
  char v25; // stack - 0x88
  unsigned long v3; // rax
  unsigned long v4; // stack - 0x120
  void *v5; // stack - 0xb8
  unsigned long v6; // stack - 0x118
  char v7 [104];
  long v8; // stack - 0x138
  unsigned long *v9; // stack - 0x130
  
  v10 = 0;
  v1 = 0;
  v23 = 0;
  v8 = a1;
  v9 = a0;
  do {
    if (!v8) {
label_d4c6:
      free(v5);
      if (!v1) {
        v15 = dat_23720;
        v20 = (void *)sub_1819d(v10,0x10);
        v16 = 0;
        for (; v15; v15 = (unsigned long *)*v15) {
          *(long *)((long)v20 + v16 * 0x10) = (long)v15 + 0xd;
          *(unsigned long **)((long)v20 + v16 * 0x10 + 8) = v15;
          v16 += 1;
        }
        sub_cbb9(v20,v10,v10,a2);
        free(v20);
      }
      sub_4b69();
      return;
    }
    v17 = *v9;
    v4 = sub_505e(v17,"r");
    if (2 <= a3) { // branch-flip
      v14 = 1;
      for (v13 = 1; v13 < a3; v13 = v13 << 1) {
        v14 += 1;
      }
      v12 = v14 << 5;
    }
    else {
      v12 = 0x30;
    }
    if (!v23) {
      v3 = sub_637c(&v4,1,v9,v8,v12);
      sub_6618(&v5,v12,v3);
    }
    v25 = '\0';
    v9 = &v9[1];
    v8 -= 1;
    while (v2 = sub_6a5e(&v5,v4,v17), v2) {
      if (((v25) && (v8)) && (v12 + 1U < (v23 - v21) - v22 * v12)) {
        v24 = v21;
        break;
      }
      dat_23240 = 0;
      v18 = sub_66d0(&v5);
      if (((v25) && (!v8)) && ((!v10 && (!v24)))) {
        sub_50b7(v4,v17);
        v6 = sub_505e(a2,"w");
        v1 = 1;
        v11 = a2;
      }
      else {
        v10 += 1;
        v11 = sub_54f9(&v6) + 0xd;
      }
      if (2 <= v22) { // branch-flip
        sub_bc07(v7,a3);
        v19 = sub_b7b6(a3,v22,v18);
        sub_c4e8(v18,a3,v22,v19 + 0x80,v7,v6,v11);
        sub_b8b3(a3,v19);
        sub_bbc5(v7);
      }
      else {
        sub_bd51(v18 + -0x20,v6,v11);
      }
      sub_50b7(v6,v11);
      if (v1) goto label_d4c6;
    }
    sub_50b7(v4,v17);
  } while( true );
}


// Function: sub_d5d8 @ 0xd5d8
void sub_d5d8(unsigned long a0)
{
  long v1; // rax
  long *v2; // stack - 0x18
  
  v1 = sub_18564(a0,0x48);
  for (v2 = (long *)0x23688; *v2; v2 = (long *)(*v2 + 0x40)) {
  }
  *v2 = v1;
  *(unsigned long *)(v1 + 0x40) = 0;
}


// Function: sub_d640 @ 0xd640
void sub_d640(unsigned long a0,char *a1)
{
  unsigned long v1; // rax
  char *v2; // rax
  
  v1 = sub_154a0(a0);
  v2 = gettext(a1);
  error(2,0,gettext("%s: invalid field specification %s"),v2,v1); // no-return
}


// Function: sub_d6a1 @ 0xd6a1
void sub_d6a1(unsigned long a0)
{
  error(2,0,gettext("options \'-%s\' are incompatible"),a0); // no-return
}


// Function: sub_d6de @ 0xd6de
void sub_d6de(void)
{
  char v1 [40];
  long v2; // fs_offset
  long v3; // stack - 0x40
  unsigned long v4; // stack - 0x10
  
  v4 = *(unsigned long *)(v2 + 0x28);
  v3 = dat_23688;
  while( true ) {
    if (!v3)
      return;
    if (2 <= (unsigned int)(unsigned char)(*(long *)(v3 + 0x20) != 0 | *(unsigned char *)(v3 + 0x33) | *(unsigned char *)(v3 + 0x38)) + (unsigned int)*(unsigned char *)(v3 + 0x32) + (unsigned int)*(unsigned char *)(v3 + 0x34) + (unsigned int)*(unsigned char *)(v3 + 0x35) + (unsigned int)*(unsigned char *)(v3 + 0x36)) break;
    v3 = *(long *)(v3 + 0x40);
  }
  *(char *)(v3 + 0x37) = 0;
  *(char *)(v3 + 0x31) = *(char *)(v3 + 0x37);
  *(char *)(v3 + 0x30) = *(char *)(v3 + 0x31);
  sub_839e(v3,v1);
  sub_d6a1(v1); // no-return
}


// Function: sub_d7de @ 0xd7de
unsigned long sub_d7de(unsigned long a0,long *a1,char *a2)
{
  unsigned long v1; // rax
  char *v2; // rax
  long v3; // stack - 0x28
  unsigned long v4; // stack - 0x30
  
  switch(sub_18b54(a0,&v4,10,&v3,0x1b1db)) {
    case 0:
    case 2:
      *a1 = v3;
      if (*a1 == v3)
        return v4;
label_d872:
      *a1 = -1;
      return v4;
    case 1:
    case 3:
      goto label_d872;
    case 4:
      if (a2) {
        v1 = sub_154a0(a0);
        v2 = gettext(a2);
        error(2,0,gettext("%s: invalid count at start of %s"),v2,v1); // no-return
      }
      return 0;
    default:
      return v4;
    
  }
}


// Function: sub_d8f9 @ 0xd8f9
void sub_d8f9(int a0)
{
  sub_4b8b();
  signal(a0,0);
  raise(a0);
}


// Function: sub_d929 @ 0xd929
char * sub_d929(char *a0,long a1,int a2)
{
  char *v1; // stack - 0x10
  
  for (v1 = a0; *v1; v1 = &v1[1]) {
    switch(*v1) {
      case 0x4d:
        *(char *)(a1 + 0x36) = 1;
        break;
      default:
        return v1;
      case 0x52:
        *(char *)(a1 + 0x33) = 1;
        break;
      case 0x56:
        *(char *)(a1 + 0x38) = 1;
        break;
      case 0x62:
        if ((!a2) || (a2 == 2))
          *(char *)(a1 + 0x30) = 1;
        if ((a2 == 1) || (a2 == 2))
          *(char *)(a1 + 0x31) = 1;
        break;
      case 100:
        *(unsigned long *)(a1 + 0x20) = 0x23460;
        break;
      case 0x66:
        *(unsigned long *)(a1 + 0x28) = 0x23560;
        break;
      case 0x67:
        *(char *)(a1 + 0x34) = 1;
        break;
      case 0x68:
        *(char *)(a1 + 0x35) = 1;
        break;
      case 0x69:
        if (!*(long *)(a1 + 0x20))
          *(unsigned long *)(a1 + 0x20) = 0x23360;
        break;
      case 0x6e:
        *(char *)(a1 + 0x32) = 1;
        break;
      case 0x72:
        *(char *)(a1 + 0x37) = 1;
      
    }
  }
  return v1;
}


// Function: sub_da55 @ 0xda55
void * sub_da55(void *a0)
{
  memset(a0,0,0x48);
  *(unsigned long *)((long)a0 + 0x10) = 0xffffffffffffffff;
  return a0;
}


// Function: sub_da8d @ 0xda8d
unsigned long sub_da8d(int a0,char **a1) // warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_da8d
{
  unsigned int v1;
  long *v10; // rax
  unsigned long v11;
  long v12;
  unsigned long v13;
  void *v14; // stack - 0xc8
  unsigned long v15 [4]; // stack - 0x238
  int v16; // stack - 0x1e8
  char v17 [80];
  char v18; // stack - 0x22
  long v19; // fs_offset
  char *v2;
  bool v20; // zf
  bool v21; // zf
  bool v22;
  char v23; // stack - 0x338
  char v24; // stack - 0x333
  int v25; // stack - 0x330
  long *v26; // stack - 0x328
  char *v27; // stack - 0x320
  char *v28; // stack - 0x318
  unsigned long v29; // stack - 0x310
  bool v3;
  unsigned long v30; // stack - 0x308
  unsigned long *v31; // stack - 0x300
  char *v32; // stack - 0x2f8
  char *v33; // stack - 0x2f0
  unsigned long v34; // stack - 0x2e8
  char *v35; // stack - 0x2e0
  unsigned long v36; // stack - 0x2d8
  unsigned long v37; // stack - 0x2d0
  unsigned long v38; // stack - 0x218
  unsigned long v39; // stack - 0x210
  bool v4;
  char v40; // stack - 0x208
  char v41; // stack - 0x207
  char v42; // stack - 0x206
  char v43; // stack - 0x205
  char v44; // stack - 0x204
  char v45; // stack - 0x203
  char v46; // stack - 0x202
  char v47; // stack - 0x201
  char v48; // stack - 0x200
  unsigned int v49; // stack - 0x1e4
  bool v5;
  unsigned long *v50; // stack - 0x1e0
  unsigned long v51; // stack - 0xc0
  unsigned long v52; // stack - 0xb8
  unsigned long v53; // stack - 0xb0
  unsigned long v54; // stack - 0xa8
  unsigned long v55; // stack - 0xa0
  unsigned long v56; // stack - 0x98
  unsigned long v57; // stack - 0x90
  unsigned long v58; // stack - 0x88
  unsigned long v59; // stack - 0x80
  char v6; // al
  unsigned long v60; // stack - 0x78
  unsigned long v61; // stack - 0x70
  unsigned long v62; // stack - 0x68
  unsigned long v63; // stack - 0x60
  unsigned long v64; // stack - 0x58
  unsigned long v65; // stack - 0x50
  unsigned long v66; // stack - 0x48
  unsigned int v67; // stack - 0x40
  char v68; // stack - 0x21
  long v69; // stack - 0x20
  bool v7; // al
  int v8;
  char *v9; // rax
  
  v69 = *(long *)(v19 + 0x28);
  v25 = 0;
  v23 = '\0';
  v5 = 0;
  v28 = NULL;
  v22 = 0;
  v29 = 0;
  v30 = 0;
  v20 = getenv("POSIXLY_CORRECT") != NULL;
  v8 = sub_133ab();
  if ((0x30db0 <= v8) && (v8 <= 0x31068))
    v4 = 0;
  else {
    v4 = 1;
  }
  v32 = NULL;
  v33 = NULL;
  sub_1345f(*a1);
  v9 = setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  sub_4029(2);
  dat_23229 = sub_107aa(3);
  dat_2322a = sub_107aa(2);
  v10 = (long *)localeconv();
  dat_23220 = *(char *)*v10;
  if ((!dat_23220) || (*(char *)(*v10 + 1)))
    dat_23220 = '.';
  dat_23224 = (int)*(char *)v10[1];
  if ((dat_23224) && (*(char *)(v10[1] + 1)))
    dat_23228 = 1;
  if ((!dat_23224) || (*(char *)(v10[1] + 1)))
    dat_23224 = 0x80;
  dat_23683 = '\0';
  sub_594a();
  sigemptyset((void *)0x236a0);
  for (v34 = 0; v34 <= 10; v34 = v34 + 1) {
    sigaction(*(int *)(v34 * 4 + 0x1d100),NULL,&v14);
    if (v14 != (void *)0x1)
      sigaddset((void *)0x236a0,*(int *)(v34 * 4 + 0x1d100));
  }
  v14 = sub_d8f9;
  v51 = dat_236a0;
  v52 = dat_236a8;
  v53 = dat_236b0;
  v54 = dat_236b8;
  v55 = dat_236c0;
  v56 = dat_236c8;
  v57 = dat_236d0;
  v58 = dat_236d8;
  v59 = dat_236e0;
  v60 = dat_236e8;
  v61 = dat_236f0;
  v62 = dat_236f8;
  v63 = dat_23700;
  v64 = dat_23708;
  v65 = dat_23710;
  v66 = dat_23718;
  v67 = 0;
  for (v34 = 0; v34 <= 10; v34 = v34 + 1) {
    v1 = *(unsigned int *)(v34 * 4 + 0x1d100);
    if (sigismember(0x236a0,v1))
      sigaction(*(int *)(v34 * 4 + 0x1d100),&v14,NULL);
  }
  signal(0x11,0);
  sub_1a960(sub_4bd4);
  sub_da55(v15);
  v15[0] = 0xffffffffffffffff;
  v31 = (unsigned long *)sub_1819d((long)a0,8);
label_dee8:
  do {
    v16 = -1;
    if ((v25 != -1) && (((!v20 || (!v30)) || (((v4 && ((((!v23 && (a0 != optind)) && (*a1[optind] == '-')) && (a1[optind][1] == 'o')))) && ((a1[optind][2] || (a0 != optind + 1)))))))) {
      v25 = getopt_long(a0,a1,"-bcCdfghik:mMno:rRsS:t:T:uVy:z",(void *)0x22560,&v16);
      if (v25 != -1) {
        if (0x88 > v25) {
          if (1 <= v25) { // branch-flip
            v6 = (char)v25;
            switch(v25) {
              case 1:
                v26 = NULL;
                if (*optarg == '+') {
                  v7 = (((a0 != optind) && (*a1[optind] == '-')) && ((unsigned int)((int)a1[optind][1] - 0x30U) <= 9));
                  if ((!v7) || (v20))
                    v3 = 0;
                  else {
                    v3 = 1;
                  }
                  v21 = v3 || v4;
                  v3 = v3 || v4;
                  v4 = v21;
                  if (v3) {
                    v26 = (long *)sub_da55(v17);
                    v27 = (char *)sub_d7de(&optarg[1],v26,0);
                    if ((v27) && (*v27 == '.'))
                      v27 = (char *)sub_d7de(&v27[1],&v26[1],0);
                    if ((!*v26) && (!v26[1]))
                      *v26 = -1;
                    if ((v27) && (!*(char *)sub_d929(v27,v26,0))) {
                      if (v7) {
                        v8 = optind + 1;
                        v2 = a1[optind];
                        optind = v8;
                        v27 = (char *)sub_d7de(&v2[1],&v26[2],"invalid number after \'-\'");
                        if (!v27)
                          __assert_fail("s","src/sort.c",0x1161,"main"); // no-return
                        if (*v27 == '.')
                          v27 = (char *)sub_d7de(&v27[1],&v26[3],"invalid number after \'.\'");
                        if ((!v26[3]) && (v26[2]))
                          v26[2] = v26[2] + -1;
                        if (*(char *)sub_d929(v27,v26,1))
                          sub_d640(v2,"stray character in field spec"); // no-return
                      }
                      *(char *)((long)v26 + 0x39) = 1;
                      sub_d5d8(v26);
                    }
                    else {
                      v26 = NULL;
                    }
                  }
                }
                if (!v26) {
                  v13 = v30 + 1;
                  v31[v30] = optarg;
                  v30 = v13;
                }
                goto label_dee8;
              case 0x43:
              case 99:
label_e561:
                if ((v23) && (v25 != v23))
                  sub_d6a1("cC"); // no-return
                v23 = (char)v25;
                goto label_dee8;
              case 0x4d:
              case 0x52:
              case 0x56:
              case 0x62:
              case 100:
              case 0x66:
              case 0x67:
              case 0x68:
              case 0x69:
              case 0x6e:
              case 0x72:
label_e4c9:
                v18 = (char)v25;
                v68 = 0;
                sub_d929(&v18,v15,2);
                goto label_dee8;
              case 0x53:
                sub_5e0c(v16,(int)v6,optarg);
                goto label_dee8;
              case 0x54:
                sub_570f(optarg);
                goto label_dee8;
              case 0x6b:
                v10 = (long *)sub_da55(v17);
                v27 = (char *)sub_d7de(optarg,v10,"invalid number at field start");
                v12 = *v10;
                *v10 = v12 + -1;
                if (!v12)
                  sub_d640(optarg,"field number is zero"); // no-return
                if (*v27 == '.') {
                  v27 = (char *)sub_d7de(&v27[1],&v10[1],"invalid number after \'.\'");
                  v12 = v10[1];
                  v10[1] = v12 + -1;
                  if (!v12)
                    sub_d640(optarg,"character offset is zero"); // no-return
                }
                if ((!*v10) && (!v10[1]))
                  *v10 = -1;
                v27 = (char *)sub_d929(v27,v10,0);
                if (*v27 != ',') { // branch-flip
                  v10[2] = -1;
                  v10[3] = 0;
                }
                else {
                  v27 = (char *)sub_d7de(&v27[1],&v10[2],"invalid number after \',\'");
                  v12 = v10[2];
                  v10[2] = v12 + -1;
                  if (!v12)
                    sub_d640(optarg,"field number is zero"); // no-return
                  if (*v27 == '.')
                    v27 = (char *)sub_d7de(&v27[1],&v10[3],"invalid number after \'.\'");
                  v27 = (char *)sub_d929(v27,v10,1);
                }
                if (*v27)
                  sub_d640(optarg,"stray character in field spec"); // no-return
                sub_d5d8(v10);
                goto label_dee8;
              case 0x6d:
                v5 = 1;
                goto label_dee8;
              case 0x6f:
                if ((v33) && (v8 = strcmp(v33,optarg), v8))
                  error(2,0,gettext("multiple output files specified")); // no-return
                v33 = optarg;
                goto label_dee8;
              case 0x73:
                dat_23681 = 1;
                goto label_dee8;
              case 0x74:
                v24 = *optarg;
                if (!v24)
                  error(2,0,gettext("empty tab")); // no-return
                if (optarg[1]) {
                  v8 = strcmp(optarg,"\\0");
                  if (v8) {
                    v11 = sub_154a0(optarg);
                    error(2,0,gettext("multi-character tab %s"),v11); // no-return
                  }
                  v24 = '\0';
                }
                if ((dat_23020 != 0x80) && (v24 != dat_23020))
                  error(2,0,gettext("incompatible tabs")); // no-return
                dat_23020 = (int)v24;
                goto label_dee8;
              case 0x75:
                dat_23682 = 1;
                goto label_dee8;
              case 0x79:
                if (a1[(long)optind + -1] == optarg) {
                  for (v35 = optarg; (unsigned int)((int)*v35 - 0x30U) <= 9; v35 = &v35[1]) {
                  }
                  optind -= (unsigned int)(*v35 != '\0');
                }
                goto label_dee8;
              case 0x7a:
                dat_23010 = 0;
                goto label_dee8;
              case 0x80:
                if (optarg) { // branch-flip
                  v12 = sub_f8c1("--check",optarg,0x22940,"CCcgeneral-numeric",1,dat_23110,1);
                  v25 = (int)"CCcgeneral-numeric"[v12];
                }
                else {
                  v25 = 99;
                }
                goto label_e561;
              case 0x81:
                if ((dat_23690) && (v8 = strcmp(dat_23690,optarg), v8))
                  error(2,0,gettext("multiple compress programs specified")); // no-return
                dat_23690 = optarg;
                goto label_dee8;
              case 0x82:
                dat_23698 = '\x01';
                goto label_dee8;
              case 0x83:
                v32 = optarg;
                goto label_dee8;
              case 0x84:
                sub_5ba8(v16,(int)v6,optarg);
                goto label_dee8;
              case 0x85:
                if ((v28) && (v8 = strcmp(v28,optarg), v8))
                  error(2,0,gettext("multiple random sources specified")); // no-return
                v28 = optarg;
                goto label_dee8;
              case 0x86:
                v12 = sub_f8c1("--sort",optarg,0x22960,"ghMnRVwaiting for %s [-d]",1,dat_23110,1);
                v25 = (int)"ghMnRVwaiting for %s [-d]"[v12];
                goto label_e4c9;
              case 0x87:
                v29 = sub_6042(v16,(int)v6,optarg);
                goto label_dee8;
              
            }
          }
          else {
            if (v25 == -0x83) {
              sub_17e44(stdout,"sort","GNU coreutils",dat_23108,"Mike Haertel","Paul Eggert",0);
              exit(0); // no-return
            }
            if (v25 == -0x82)
              sub_44d3(0); // no-return
          }
        }
        sub_44d3(2); // no-return
      }
    }
    if (a0 <= optind) {
      if (v32) {
        if (v30) {
          v11 = sub_15059(4,*v31);
          error(0,0,gettext("extra operand %s"),v11); // no-return
        }
        v11 = sub_505e(v32,"r");
        sub_1676c(&v16);
        if (sub_16b8b(v11,&v16) != '\x01') {
          v11 = sub_15059(4,v32);
          error(2,0,gettext("cannot read file names from %s"),v11); // no-return
        }
        sub_50b7(v11,v32);
        if (!CONCAT44(v49,v16)) {
          v11 = sub_15059(4,v32);
          error(2,0,gettext("no input from %s"),v11); // no-return
        }
        free(v31);
        v31 = v50;
        v30 = CONCAT44(v49,v16);
        for (v36 = 0; v36 < v30; v36 = v36 + 1) {
          v2 = (char *)v50[v36];
          if (!strcmp(v2,"-")) {
            v11 = sub_15059(4,v50[v36]);
            error(2,0,gettext("when reading file names from stdin, no file name of %s allowed"),v11); // no-return
          }
          if (!*(char *)v50[v36]) {
            v11 = sub_151eb(0,3,v32);
            error(2,0,gettext("%s:%lu: invalid zero-length file name"),v11,v36 + 1); // no-return
          }
        }
      }
      for (v26 = (long *)dat_23688; v26; v26 = *(long **)((long)v26 + 0x40)) {
        if ((sub_8305(v26)) && (*(char *)((long)v26 + 0x37) != '\x01')) {
          *(unsigned long *)((long)v26 + 0x20) = v38;
          *(unsigned long *)((long)v26 + 0x28) = v39;
          *(char *)((long)v26 + 0x30) = v40;
          *(char *)((long)v26 + 0x31) = v41;
          *(char *)((long)v26 + 0x36) = v46;
          *(char *)((long)v26 + 0x32) = v42;
          *(char *)((long)v26 + 0x34) = v44;
          *(char *)((long)v26 + 0x35) = v45;
          *(char *)((long)v26 + 0x38) = v48;
          *(char *)((long)v26 + 0x33) = v43;
          *(char *)((long)v26 + 0x37) = v47;
        }
        v22 = *(char *)((long)v26 + 0x33) || v22;
      }
      if ((!dat_23688) && (sub_8305(v15) != '\x01')) {
        sub_d5d8(v15);
        v22 = v43 || v22;
      }
      sub_d6de();
      if (!dat_23698) {
        dat_23680 = v47;
        if (v22)
          sub_7573(v28);
        if (!dat_23670) {
          v9 = getenv("TMPDIR");
          if (!v9)
            v9 = "/tmp";
          sub_570f(v9);
        }
        if (!v30) {
          v30 = 1;
          free(v31);
          v31 = (unsigned long *)sub_18024(8);
          *v31 = "-";
        }
        if (dat_23660) {
          if (dat_23660 <= (unsigned long)dat_23024 * 0x22)
            dat_23660 = (unsigned long)dat_23024 * 0x22;
        }
        if (!v23) {
          sub_ca81(v31,v30);
          sub_cb44(v33);
          if (v5) {
            v12 = sub_18502(v30,0x10);
            for (v37 = 0; v37 < v30; v37 = v37 + 1) {
              *(unsigned long *)(v12 + v37 * 0x10) = v31[v37];
            }
            sub_cbb9(v12,0,v30,v33);
          }
          else {
            if ((!v29) && (v29 = sub_12de4(2), 9 <= v29))
              v29 = 8;
            v13 = 0xffffffffffffff;
            if (v29 <= 0xffffffffffffff)
              v13 = v29;
            sub_d0f9(v31,v30,v33,v13);
          }
          if ((dat_23683) && (v8 = sub_ff17(stdin), v8 == -1))
            sub_4464(gettext("close failed"),"-"); // no-return
          if (v69 == *(long *)(v19 + 0x28))
            return 0;
          __stack_chk_fail(); // no-return
        }
        if (2 <= v30) {
          v11 = sub_15059(4,v31[1]);
          error(2,0,gettext("extra operand %s not allowed with -%c"),v11,(unsigned long)(unsigned int)(int)v23); // no-return
        }
        if (!v33) {
          v11 = *v31;
          exit((sub_a0b1(v11,(int)v23) ^ 1) & 0xff); // no-return
        }
        dat_23032 = v23;
        sub_d6a1(0x23032); // no-return
      }
      if ((v23) || (v33)) {
        if (!v23)
          v23 = 'o';
        s_23028[0] = v23;
        sub_d6a1(s_23028); // no-return
      }
      v22 = 0;
      if (v9)
        v22 = setlocale(3,"") != NULL;
      if (v22) {
        if (!dat_23229)
          error(0,0,"%s",gettext("text ordering performed using simple byte comparison")); // no-return
        v11 = sub_154a0(setlocale(3,NULL));
        error(0,0,gettext("text ordering performed using %s sorting rules"),v11); // no-return
      }
      error(0,0,"%s",gettext("failed to set locale")); // no-return
    }
    v8 = optind + 1;
    v13 = v30 + 1;
    v31[v30] = a1[optind];
    optind = v8;
    v30 = v13;
  } while( true );
}


// Function: sub_10854 @ 0x10854
unsigned long sub_10854(long a0)
{
  return *(unsigned long *)(a0 + 0x10);
}


// Function: sub_1086a @ 0x1086a
unsigned long sub_1086a(long a0)
{
  return *(unsigned long *)(a0 + 0x18);
}


// Function: sub_10880 @ 0x10880
unsigned long sub_10880(long a0)
{
  return *(unsigned long *)(a0 + 0x20);
}


// Function: sub_10896 @ 0x10896
unsigned long sub_10896(unsigned long *a0)
{
  long *v1; // stack - 0x28
  unsigned long v2; // stack - 0x20
  long *v3; // stack - 0x18
  unsigned long v4; // stack - 0x10
  
  v2 = 0;
  for (v1 = (long *)*a0; v1 < (long *)a0[1]; v1 = &v1[2]) {
    if (*v1) {
      v3 = v1;
      v4 = 1;
      while (v3 = (long *)v3[1], v3) {
        v4 += 1;
      }
      if (v2 < v4)
        v2 = v4;
    }
  }
  return v2;
}


// Function: sub_10918 @ 0x10918
unsigned long sub_10918(unsigned long *a0) // return-dupe
{
  long *v1; // stack - 0x28
  long v2; // stack - 0x20
  long v3; // stack - 0x18
  long *v4; // stack - 0x10
  
  v2 = 0;
  v3 = 0;
  for (v1 = (long *)*a0; v1 < (long *)a0[1]; v1 = &v1[2]) {
    if (*v1) {
      v4 = v1;
      v2 += 1;
      do {
        v3 += 1;
        v4 = (long *)v4[1];
      } while (v4);
    }
  }
  if ((v2 == a0[3]) && (v3 == a0[4]))
    return 1;
  return 0;
}


// Function: sub_109b6 @ 0x109b6
void sub_109b6(unsigned long a0,void *a1)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // rax
  
  v1 = sub_10880(a0);
  v2 = sub_10854(a0);
  v3 = sub_1086a(a0);
  v4 = sub_10896(a0);
  fprintf(a1,"# entries:         %lu\n",v1);
  fprintf(a1,"# buckets:         %lu\n",v2);
  fprintf(a1,"# buckets used:    %lu (%.2f%%)\n",(double)v2,(dat_1d278 * (double)v3) / (double)v2,v3);
  fprintf(a1,"max bucket length: %lu\n",v4);
}


// Function: sub_10afd @ 0x10afd
long sub_10afd(long *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  v1 = (*(void *)a0[6])(a1,a0[2]);
  if ((unsigned long)a0[2] <= v1)
    abort(); // no-return
  return v1 * 0x10 + *a0;
}


// Function: sub_10b58 @ 0x10b58
long sub_10b58(long a0,long a1) // return-dupe
{
  void *v1;
  long v2;
  long *v3; // stack - 0x18
  
  v3 = (long *)sub_10afd(a0,a1);
  if (!*v3)
    return 0;
  while( true ) {
    if (!v3)
      return 0;
    if ((a1 == *v3) || (v1 = *(void **)(a0 + 0x38), v2 = *v3, (*v1)(a1,v2))) break;
    v3 = (long *)v3[1];
  }
  return *v3;
}


// Function: sub_10bef @ 0x10bef
long sub_10bef(unsigned long *a0) // early-return
{
  long *v1; // stack - 0x10
  
  if (!a0[4])
    return 0;
  v1 = (long *)*a0;
  while( true ) {
    if ((long *)a0[1] <= v1)
      abort(); // no-return
    if (*v1) break;
    v1 = &v1[2];
  }
  return *v1;
}


// Function: sub_10c4f @ 0x10c4f
long sub_10c4f(long a0,long a1) // early-return
{
  long *v1; // stack - 0x18
  long *v2; // stack - 0x10
  
  v1 = (long *)sub_10afd(a0,a1);
  v2 = v1;
  while ((a1 != *v2 || (!v2[1]))) {
    v2 = (long *)v2[1];
    if (!v2) {
      do {
        v1 = &v1[2];
        if (*(long **)(a0 + 8) <= v1)
          return 0;
      } while (!*v1);
      return *v1;
    }
  }
  return *(long *)v2[1];
}


// Function: sub_10ced @ 0x10ced
unsigned long sub_10ced(unsigned long *a0,long a1,unsigned long a2)
{
  unsigned long v1;
  unsigned long v2; // stack - 0x20
  long *v3; // stack - 0x18
  long *v4; // stack - 0x10
  
  v2 = 0;
  v3 = (long *)*a0;
  do {
    if ((long *)a0[1] <= v3)
      return v2;
    if (*v3) {
      v4 = v3;
      while (v4) {
        if (a2 <= v2)
          return v2;
        v1 = v2 + 1;
        *(long *)(v2 * 8 + a1) = *v4;
        v4 = (long *)v4[1];
        v2 = v1;
      }
    }
    v3 = &v3[2];
  } while( true );
}


// Function: sub_10d8d @ 0x10d8d
long sub_10d8d(unsigned long *a0,void *a1,unsigned long a2)
{
  long v1;
  long v2; // stack - 0x20
  long *v3; // stack - 0x18
  long *v4; // stack - 0x10
  
  v2 = 0;
  v3 = (long *)*a0;
  do {
    if ((long *)a0[1] <= v3)
      return v2;
    if (*v3) {
      for (v4 = v3; v4; v4 = (long *)v4[1]) {
        v1 = *v4;
        if ((*a1)(v1,a2) != '\x01')
          return v2;
        v2 += 1;
      }
    }
    v3 = &v3[2];
  } while( true );
}


// Function: sub_10e25 @ 0x10e25
unsigned long sub_10e25(unsigned char *a0,unsigned long a1)
{
  unsigned char *v1; // stack - 0x20
  unsigned long v2; // stack - 0x10
  
  v2 = 0;
  for (v1 = a0; *v1; v1 = &v1[1]) {
    v2 = ((unsigned long)*v1 + v2 * 0x1f) % a1;
  }
  return v2;
}


// Function: sub_10e7f @ 0x10e7f
unsigned long sub_10e7f(unsigned long a0)
{
  unsigned long v1; // stack - 0x18
  unsigned long v2; // stack - 0x10
  
  v2 = 9;
  for (v1 = 3; (v2 < a0 && (a0 % v1)); v1 = v1 + 2) {
    v2 += (v1 + 1) * 4;
  }
  return CONCAT71((undefined7)(a0 % v1 >> 8),a0 % v1 != 0);
}


// Function: sub_10eee @ 0x10eee
unsigned long sub_10eee(unsigned long a0)
{
  unsigned long v1; // stack - 0x10
  
  v1 = a0;
  if (a0 <= 9)
    v1 = 10;
  v1 |= 1;
  while( true ) {
    if (v1 == 0xffffffffffffffff)
      return 0xffffffffffffffff;
    if (sub_10e7f(v1) == '\x01') break;
    v1 += 2;
  }
  return v1;
}


// Function: sub_10f39 @ 0x10f39
void sub_10f39(unsigned int *a0)
{
  *a0 = 0;
  a0[1] = dat_1d280;
  a0[2] = dat_1d284;
  a0[3] = dat_1d288;
  *(char *)&a0[4] = 0;
}


// Function: sub_10f8f @ 0x10f8f
unsigned long sub_10f8f(unsigned long a0,unsigned long a1)
{
  return (unsigned long)sub_19063(a0,3) % a1;
}


// Function: sub_10fca @ 0x10fca
unsigned long sub_10fca(long a0,long a1)
{
  return CONCAT71((undefined7)((unsigned long)a0 >> 8),a0 == a1);
}


// Function: sub_10fe7 @ 0x10fe7
unsigned long sub_10fe7(long a0) // return-dupe x2
{
  float *v1;
  
  v1 = *(float **)(a0 + 0x28);
  if (v1 == (float *)0x1d1f0)
    return 1;
  if ((((dat_1d28c < v1[2]) && (v1[2] < dat_1d280 - dat_1d28c)) && (dat_1d28c + dat_1d280 < v1[3])) && (((0.0 <= *v1 && (*v1 + dat_1d28c < v1[1])) && ((v1[1] <= dat_1d280 && (*v1 + dat_1d28c < v1[2]))))))
    return 1;
  *(unsigned long *)(a0 + 0x28) = 0x1d1f0;
  return 0;
}


// Function: sub_110f4 @ 0x110f4
unsigned long sub_110f4(unsigned long a0,long a1) // early-return x2
{
  unsigned long v1; // rax
  float v2; // xmm0_da
  unsigned long v3; // stack - 0x20
  
  v3 = a0;
  if (*(char *)(a1 + 0x10) != '\x01') {
    v2 = (float)a0 / *(float *)(a1 + 8);
    if (dat_1d290 <= v2)
      return 0;
    if (dat_1d294 <= v2)
      v3 = (long)(v2 - dat_1d294) ^ 0x8000000000000000;
    else {
      v3 = (unsigned long)v2;
    }
  }
  v1 = sub_10eee(v3);
  if (!((long)(v1 << 3) <= -1 || v1 >> 0x3d))
    return v1;
  return 0;
}


// Function: sub_1120d @ 0x1120d
long * sub_1120d(unsigned long a0,long a1,void *a2,void *a3,long a4) // early-return x2
{
  unsigned long v1;
  long *v2; // rax
  void *v3; // stack - 0x38
  void *v4; // stack - 0x30
  long v5; // stack - 0x28
  
  v4 = a2;
  if (!a2)
    v4 = sub_10f8f;
  v3 = a3;
  if (!a3)
    v3 = sub_10fca;
  v2 = malloc(0x50);
  if (!v2)
    return NULL;
  v5 = a1;
  if (!a1)
    v5 = 0x1d1f0;
  v2[5] = v5;
  if (sub_10fe7(v2) == '\x01') {
    v2[2] = sub_110f4(a0,v5);
    if (v2[2]) {
      v1 = v2[2];
      *v2 = (long)calloc(v1,0x10);
      if (*v2) {
        v2[1] = *v2 + v2[2] * 0x10;
        v2[3] = 0;
        v2[4] = 0;
        v2[6] = (long)v4;
        v2[7] = (long)v3;
        v2[8] = a4;
        v2[9] = 0;
        return v2;
      }
    }
  }
  free(v2);
  return NULL;
}


// Function: sub_11382 @ 0x11382
void sub_11382(unsigned long *a0)
{
  unsigned long *v1;
  long *v2; // stack - 0x20
  unsigned long *v3; // stack - 0x18
  
  for (v2 = (long *)*a0; v2 < (long *)a0[1]; v2 = &v2[2]) {
    if (*v2) {
      v3 = (unsigned long *)v2[1];
      while (v3) {
        if (a0[8])
          (*(void *)a0[8])(*v3);
        *v3 = 0;
        v1 = (unsigned long *)v3[1];
        v3[1] = a0[9];
        a0[9] = v3;
        v3 = v1;
      }
      if (a0[8])
        (*(void *)a0[8])(*v2);
      *v2 = 0;
      v2[1] = 0;
    }
  }
  a0[3] = 0;
  a0[4] = 0;
}


// Function: sub_1148d @ 0x1148d
void sub_1148d(unsigned long *a0)
{
  void *v1;
  long *v2; // stack - 0x20
  long *v3; // stack - 0x18
  
  if ((a0[8]) && (a0[4])) {
    for (v2 = (long *)*a0; v2 < (long *)a0[1]; v2 = &v2[2]) {
      if (*v2) {
        for (v3 = v2; v3; v3 = (long *)v3[1]) {
          (*(void *)a0[8])(*v3);
        }
      }
    }
  }
  for (v2 = (unsigned long)*a0; v2 < a0[1]; v2 = (unsigned long)((long)v2 + 0x10)) {
    v3 = *(void **)((long)v2 + 8);
    while (v3) {
      v1 = *(void **)((long)v3 + 8);
      free(v3);
      v3 = v1;
    }
  }
  v3 = (void *)a0[9];
  while (v3) {
    v1 = *(void **)((long)v3 + 8);
    free(v3);
    v3 = v1;
  }
  free((void *)*a0);
  free(a0);
}


// Function: sub_115bc @ 0x115bc
void * sub_115bc(long a0)
{
  void *v1; // stack - 0x10
  
  if (*(long *)(a0 + 0x48)) { // branch-flip
    v1 = *(void **)(a0 + 0x48);
    *(unsigned long *)(a0 + 0x48) = *(unsigned long *)((long)v1 + 8);
  }
  else {
    v1 = malloc(0x10);
  }
  return v1;
}


// Function: sub_1160b @ 0x1160b
void sub_1160b(long a0,unsigned long *a1)
{
  *a1 = 0;
  a1[1] = *(unsigned long *)(a0 + 0x48);
  *(unsigned long **)(a0 + 0x48) = a1;
}


// Function: sub_11645 @ 0x11645
long sub_11645(long a0,long a1,unsigned long *a2,char a3) // early-return
{
  void *v1;
  long v2;
  long *v3;
  long v4;
  unsigned long v5;
  long *v6; // stack - 0x38
  
  v6 = (long *)sub_10afd(a0,a1);
  *a2 = v6;
  if (!*v6)
    return 0;
  if ((a1 != *v6) && (v1 = *(void **)(a0 + 0x38), v2 = *v6, !(*v1)(a1,v2))) {
    while( true ) {
      if (!v6[1])
        return 0;
      if ((a1 == *(long *)v6[1]) || (v1 = *(void **)(a0 + 0x38), v5 = *(unsigned long *)v6[1], (*v1)(a1,v5))) break;
      v6 = (long *)v6[1];
    }
    v2 = *(long *)v6[1];
    if (a3) {
      v4 = v6[1];
      v6[1] = *(long *)(v4 + 8);
      sub_1160b(a0,v4);
    }
    return v2;
  }
  v2 = *v6;
  if (a3) {
    if (v6[1]) { // branch-flip
      v3 = (long *)v6[1];
      v4 = v3[1];
      *v6 = *v3;
      v6[1] = v4;
      sub_1160b(a0,v3);
    }
    else {
      *v6 = 0;
    }
  }
  return v2;
}


// Function: sub_117de @ 0x117de
unsigned long sub_117de(long a0,unsigned long *a1,char a2)
{
  long v1;
  long *v2;
  long *v3; // rax
  long *v4; // stack - 0x38
  long *v5; // stack - 0x30
  
  v4 = (long *)*a1;
  do {
    if ((long *)a1[1] <= v4)
      return 1;
    if (*v4) {
      v5 = (long *)v4[1];
      while (v5) {
        v1 = *v5;
        v3 = (long *)sub_10afd(a0,v1);
        v2 = (long *)v5[1];
        if (*v3) { // branch-flip
          v5[1] = v3[1];
          v3[1] = (long)v5;
        }
        else {
          *v3 = v1;
          *(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + 1;
          sub_1160b(a0,v5);
        }
        v5 = v2;
      }
      v1 = *v4;
      v4[1] = 0;
      if (!a2) {
        v2 = (long *)sub_10afd(a0,v1);
        if (*v2) { // branch-flip
          v3 = (long *)sub_115bc(a0);
          if (!v3)
            return 0;
          *v3 = v1;
          v3[1] = v2[1];
          v2[1] = (long)v3;
        }
        else {
          *v2 = v1;
          *(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + 1;
        }
        *v4 = 0;
        a1[3] = a1[3] + -1;
      }
    }
    v4 = &v4[2];
  } while( true );
}


// Function: sub_119b2 @ 0x119b2
unsigned long sub_119b2(unsigned long *a0,unsigned long a1) // return-dupe x4
{
  void *v1; // stack - 0x68
  unsigned long v10; // stack - 0x20
  void *v2; // stack - 0x60
  unsigned long v3; // stack - 0x58
  unsigned long v4; // stack - 0x50
  unsigned long v5; // stack - 0x48
  unsigned long v6; // stack - 0x40
  unsigned long v7; // stack - 0x38
  unsigned long v8; // stack - 0x30
  unsigned long v9; // stack - 0x28
  
  v3 = sub_110f4(a1,a0[5]);
  if (!v3)
    return 0;
  if (v3 == a0[2])
    return 1;
  v1 = calloc(v3,0x10);
  if (!v1)
    return 0;
  v2 = (void *)((long)v1 + v3 * 0x10);
  v4 = 0;
  v5 = 0;
  v6 = a0[5];
  v7 = a0[6];
  v8 = a0[7];
  v9 = a0[8];
  v10 = a0[9];
  if (sub_117de(&v1,a0,0)) {
    free((void *)*a0);
    *a0 = v1;
    a0[1] = v2;
    a0[2] = v3;
    a0[3] = v4;
    a0[9] = v10;
    return 1;
  }
  a0[9] = v10;
  if ((sub_117de(a0,&v1,1) == '\x01') && (sub_117de(a0,&v1,0) == '\x01')) {
    free(v1);
    return 0;
  }
  abort(); // no-return
}


// Function: sub_11bdd @ 0x11bdd
unsigned long sub_11bdd(long a0,long a1,long *a2) // return-dupe x2, return-dupe, ternary x2
{
  unsigned long v1; // rax
  long *v2; // rax
  long *v3; // stack - 0x30
  float v4; // xmm0_da
  long v5; // stack - 0x28
  long v6; // stack - 0x20
  
  if (!a1)
    abort(); // no-return
  v5 = sub_11645(a0,a1,&v3,0);
  if (v5) {
    if (!a2)
      return 0;
    *a2 = v5;
    return 0;
  }
  if (((float)*(unsigned long *)(a0 + 0x10) * *(float *)(*(long *)(a0 + 0x28) + 8) < (float)*(unsigned long *)(a0 + 0x18)) && (sub_10fe7(a0), (float)*(unsigned long *)(a0 + 0x10) * *(float *)(*(long *)(a0 + 0x28) + 8) < (float)*(unsigned long *)(a0 + 0x18))) {
    v6 = *(long *)(a0 + 0x28);
    v4 = (*(char *)(v6 + 0x10)) ? (float)*(unsigned long *)(a0 + 0x10) * *(float *)(v6 + 0xc) : *(float *)(v6 + 8) * (*(float *)(v6 + 0xc) * (float)*(unsigned long *)(a0 + 0x10)); // branch-flip
    if (dat_1d290 <= v4)
      return 0xffffffff;
    v1 = (dat_1d294 <= v4) ? (long)(v4 - dat_1d294) ^ 0x8000000000000000 : (unsigned long)v4;
    if (sub_119b2(a0,v1) != '\x01')
      return 0xffffffff;
    if (sub_11645(a0,a1,&v3,0))
      abort(); // no-return
  }
  if (!*v3) {
    *v3 = a1;
    *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + 1;
    *(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + 1;
    return 1;
  }
  v2 = (long *)sub_115bc(a0);
  if (v2) {
    *v2 = a1;
    v2[1] = v3[1];
    v3[1] = (long)v2;
    *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + 1;
    return 1;
  }
  return 0xffffffff;
}


// Function: sub_11f4e @ 0x11f4e
unsigned long sub_11f4e(unsigned long a0,unsigned long a1) // early-return
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x18
  
  v1 = sub_11bdd(a0,a1,&v3);
  if (v1 != -1) {
    if (v1) // branch-flip
      v2 = a1;
    else {
      v2 = v3;
    }
    return v2;
  }
  v2 = 0;
  return v2;
}


// Function: sub_11fbe @ 0x11fbe
long sub_11fbe(long a0,unsigned long a1) // early-return, ternary x2
{
  char v1; // al
  long *v2; // stack - 0x40
  float v3; // xmm0_da
  void *v4; // stack - 0x38
  long v5; // stack - 0x30
  long v6; // stack - 0x28
  unsigned long v7; // stack - 0x20
  void *v8; // stack - 0x18
  
  v5 = sub_11645(a0,a1,&v2,1);
  if (!v5)
    return 0;
  *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + -1;
  if (((!*v2) && (*(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + -1, (float)*(unsigned long *)(a0 + 0x18) < (float)*(unsigned long *)(a0 + 0x10) * **(float **)(a0 + 0x28))) && (sub_10fe7(a0), (float)*(unsigned long *)(a0 + 0x18) < (float)*(unsigned long *)(a0 + 0x10) * **(float **)(a0 + 0x28))) {
    v6 = *(long *)(a0 + 0x28);
    if (*(char *)(v6 + 0x10)) { // branch-flip
      v3 = (float)*(unsigned long *)(a0 + 0x10) * *(float *)(v6 + 4);
      v7 = (dat_1d294 <= v3) ? (long)(v3 - dat_1d294) ^ 0x8000000000000000 : (unsigned long)v3;
    }
    else {
      v3 = *(float *)(v6 + 8) * (*(float *)(v6 + 4) * (float)*(unsigned long *)(a0 + 0x10));
      v7 = (dat_1d294 <= v3) ? (long)(v3 - dat_1d294) ^ 0x8000000000000000 : (unsigned long)v3;
    }
    v1 = sub_119b2(a0,v7);
    if (v1 != '\x01') {
      v4 = *(void **)(a0 + 0x48);
      while (v4) {
        v8 = *(void **)((long)v4 + 8);
        free(v4);
        v4 = v8;
      }
      *(unsigned long *)(a0 + 0x48) = 0;
    }
  }
  return v5;
}


// Function: sub_122c4 @ 0x122c4
void sub_122c4(unsigned long a0,unsigned long a1)
{
  sub_11fbe(a0,a1);
}


// Function: sub_12bc5 @ 0x12bc5
long sub_12bc5(void) // return-dupe
{
  long v1; // rax
  char v2 [136];
  
  if (sched_getaffinity(0,0x80,v2))
    return 0;
  v1 = (long)(int)__sched_cpucount(0x80,v2);
  if (!v1)
    return 0;
  return v1;
}


// Function: sub_12c4b @ 0x12c4b
unsigned long sub_12c4b(int a0) // return-dupe
{
  unsigned long v1; // rax
  unsigned long v2; // stack - 0x28
  
  if (a0 != 1) {
    v2 = sysconf(0x53);
    if (((v2 == 1) || (v2 == 2)) && (v1 = sub_12bc5(), v2 < v1))
      v2 = v1;
    if ((long)v2 <= 0)
      return 1;
    return v2;
  }
  v1 = sub_12bc5();
  if (v1)
    return v1;
  v1 = sysconf(0x54);
  if ((long)v1 <= 0)
    return 1;
  return v1;
}


// Function: sub_12cdc @ 0x12cdc
unsigned long sub_12cdc(char *a0)
{
  char v1;
  char *v2; // stack - 0x28
  char *v3; // stack - 0x30
  unsigned long v4; // stack - 0x20
  unsigned long v5; // stack - 0x18
  
  v4 = 0;
  v3 = a0;
  if (!a0)
    return 0;
  while ((*v3 && (v1 = *v3, sub_19332((int)v1)))) {
    v3 = &v3[1];
  }
  v1 = *v3;
  if (sub_19258((int)v1)) {
    v2 = NULL;
    v5 = strtoul(v3,&v2,10);
    if (v2) {
      while ((*v2 && (v1 = *v2, sub_19332((int)v1)))) {
        v2 = &v2[1];
      }
      if (!*v2)
        return v5;
      if (*v2 == ',')
        return v5;
    }
  }
  return v4;
}


// Function: sub_12de4 @ 0x12de4
unsigned long sub_12de4(int a0)
{
  unsigned long v1;
  int v2; // stack - 0x2c
  unsigned long v3; // stack - 0x20
  
  v3 = 0xffffffffffffffff;
  v2 = a0;
  if (a0 == 2) {
    v1 = sub_12cdc(getenv("OMP_NUM_THREADS"));
    v3 = sub_12cdc(getenv("OMP_THREAD_LIMIT"));
    if (!v3)
      v3 = 0xffffffffffffffff;
    if (v1) {
      if (v1 < v3)
        return v1;
      return v3;
    }
    v2 = 1;
  }
  v1 = sub_12c4b(v2);
  if (v3 <= v1)
    v1 = v3;
  return v1;
}

