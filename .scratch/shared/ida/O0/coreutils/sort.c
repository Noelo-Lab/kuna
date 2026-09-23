// Function: async_safe_die @ 0x439b
void async_safe_die(unsigned int a1, const char *a2)
{
  size_t v2; // rax
  size_t v3; // rax
  char *v4; // [rsp+18h] [rbp-38h]
  char v5[12]; // [rsp+3Ch] [rbp-14h] BYREF
  unsigned long long v6; // [rsp+48h] [rbp-8h]

  v6 = __readfsqword(0x28u);
  v2 = strlen(a2);
  write(2, a2, v2);
  if ( a1 )
  {
    v4 = (char *)sub_126FE(a1, v5);
    write(2, ": errno ", 8u);
    v3 = strlen(v4);
    write(2, v4, v3);
  }
  write(2, "\n", 1u);
  _exit(2);
}



// Function: sort_die @ 0x4464
void sort_die(const char *a1, char *a2)
{
  char *v2; // rax
  const char *v3; // rbx
  int *v4; // rax

  if ( a2 )
    v2 = a2;
  else
    v2 = gettext("standard output");
  v3 = (const char *)sub_151EB(0, 3, v2);
  v4 = __errno_location();
  error(2, *v4, "%s: %s", a1, v3);
  sub_44D3();
}



// Function: usage @ 0x44d3
void usage(int a1)
{
  long long v1; // rbx
  char *v2; // rax
  long long v3; // r12
  long long v4; // rbx
  char *v5; // rax
  FILE *v6; // rbx
  char *v7; // rdi
  FILE *v8; // rbx
  char *v9; // rax
  FILE *v10; // rbx
  char *v11; // rax
  FILE *v12; // rbx
  char *v13; // rax
  FILE *v14; // rbx
  char *v15; // rax
  FILE *v16; // rbx
  char *v17; // rax
  FILE *v18; // rbx
  char *v19; // rax
  FILE *v20; // rbx
  char *v21; // rax
  FILE *v22; // rbx
  char *v23; // rax
  FILE *v24; // rbx
  char *v25; // rax
  FILE *v26; // rbx
  char *v27; // rax
  FILE *v28; // rbx
  char *v29; // rax
  FILE *v30; // rbx
  char *v31; // rax
  char *v32; // rax
  FILE *v33; // rbx
  char *v34; // rax
  FILE *v35; // rbx
  char *v36; // rax
  FILE *v37; // rbx
  char *v38; // rax
  FILE *v39; // rbx
  char *v40; // rax
  FILE *v41; // rbx
  char *v42; // rax

  if ( a1 )
  {
    v1 = qword_23868;
    v2 = gettext("Try '%s --help' for more information.\n");
    fprintf(stderr, v2, v1);
  }
  else
  {
    v3 = qword_23868;
    v4 = qword_23868;
    v5 = gettext("Usage: %s [OPTION]... [FILE]...\n  or:  %s [OPTION]... --files0-from=F\n");
    printf(v5, v4, v3);
    v6 = stdout;
    v7 = gettext("Write sorted concatenation of all FILE(s) to standard output.\n");
    fputs_unlocked(v7, v6);
    sub_40A2();
    sub_40D3();
    v8 = stdout;
    v9 = gettext("Ordering options:\n\n");
    fputs_unlocked(v9, v8);
    v10 = stdout;
    v11 = gettext(
            "  -b, --ignore-leading-blanks  ignore leading blanks\n"
            "  -d, --dictionary-order      consider only blanks and alphanumeric characters\n"
            "  -f, --ignore-case           fold lower case to upper case characters\n");
    fputs_unlocked(v11, v10);
    v12 = stdout;
    v13 = gettext(
            "  -g, --general-numeric-sort  compare according to general numerical value\n"
            "  -i, --ignore-nonprinting    consider only printable characters\n"
            "  -M, --month-sort            compare (unknown) < 'JAN' < ... < 'DEC'\n");
    fputs_unlocked(v13, v12);
    v14 = stdout;
    v15 = gettext("  -h, --human-numeric-sort    compare human readable numbers (e.g., 2K 1G)\n");
    fputs_unlocked(v15, v14);
    v16 = stdout;
    v17 = gettext(
            "  -n, --numeric-sort          compare according to string numerical value\n"
            "  -R, --random-sort           shuffle, but group identical keys.  See shuf(1)\n"
            "      --random-source=FILE    get random bytes from FILE\n"
            "  -r, --reverse               reverse the result of comparisons\n");
    fputs_unlocked(v17, v16);
    v18 = stdout;
    v19 = gettext(
            "      --sort=WORD             sort according to WORD:\n"
            "                                general-numeric -g, human-numeric -h, month -M,\n"
            "                                numeric -n, random -R, version -V\n"
            "  -V, --version-sort          natural sort of (version) numbers within text\n"
            "\n");
    fputs_unlocked(v19, v18);
    v20 = stdout;
    v21 = gettext("Other options:\n\n");
    fputs_unlocked(v21, v20);
    v22 = stdout;
    v23 = gettext(
            "      --batch-size=NMERGE   merge at most NMERGE inputs at once;\n"
            "                            for more use temp files\n");
    fputs_unlocked(v23, v22);
    v24 = stdout;
    v25 = gettext(
            "  -c, --check, --check=diagnose-first  check for sorted input; do not sort\n"
            "  -C, --check=quiet, --check=silent  like -c, but do not report first bad line\n"
            "      --compress-program=PROG  compress temporaries with PROG;\n"
            "                              decompress them with PROG -d\n");
    fputs_unlocked(v25, v24);
    v26 = stdout;
    v27 = gettext(
            "      --debug               annotate the part of the line used to sort,\n"
            "                              and warn about questionable usage to stderr\n"
            "      --files0-from=F       read input from the files specified by\n"
            "                            NUL-terminated names in file F;\n"
            "                            If F is - then read names from standard input\n");
    fputs_unlocked(v27, v26);
    v28 = stdout;
    v29 = gettext(
            "  -k, --key=KEYDEF          sort via a key; KEYDEF gives location and type\n"
            "  -m, --merge               merge already sorted files; do not sort\n");
    fputs_unlocked(v29, v28);
    v30 = stdout;
    v31 = gettext(
            "  -o, --output=FILE         write result to FILE instead of standard output\n"
            "  -s, --stable              stabilize sort by disabling last-resort comparison\n"
            "  -S, --buffer-size=SIZE    use SIZE for main memory buffer\n");
    fputs_unlocked(v31, v30);
    v32 = gettext(
            "  -t, --field-separator=SEP  use SEP instead of non-blank to blank transition\n"
            "  -T, --temporary-directory=DIR  use DIR for temporaries, not $TMPDIR or %s;\n"
            "                              multiple options specify multiple directories\n"
            "      --parallel=N          change the number of sorts run concurrently to N\n"
            "  -u, --unique              with -c, check for strict ordering;\n"
            "                              without -c, output only the first of an equal run\n");
    printf(v32, "/tmp");
    v33 = stdout;
    v34 = gettext("  -z, --zero-terminated     line delimiter is NUL, not newline\n");
    fputs_unlocked(v34, v33);
    v35 = stdout;
    v36 = gettext("      --help        display this help and exit\n");
    fputs_unlocked(v36, v35);
    v37 = stdout;
    v38 = gettext("      --version     output version information and exit\n");
    fputs_unlocked(v38, v37);
    v39 = stdout;
    v40 = gettext(
            "\n"
            "KEYDEF is F[.C][OPTS][,F[.C][OPTS]] for start and stop position, where F is a\n"
            "field number and C a character position in the field; both are origin 1, and\n"
            "the stop position defaults to the line's end.  If neither -t nor -b is in\n"
            "effect, characters in a field are counted from the beginning of the preceding\n"
            "whitespace.  OPTS is one or more single-letter ordering options [bdfgiMhnRrV],\n"
            "which override global ordering options for that key.  If no key is given, use\n"
            "the entire line as the key.  Use --debug to diagnose incorrect key usage.\n"
            "\n"
            "SIZE may be followed by the following multiplicative suffixes:\n");
    fputs_unlocked(v40, v39);
    v41 = stdout;
    v42 = gettext(
            "% 1% of memory, b 1, K 1024 (default), and so on for M, G, T, P, E, Z, Y.\n"
            "\n"
            "*** WARNING ***\n"
            "The locale specified by the environment affects sort order.\n"
            "Set LC_ALL=C to get the traditional sort order that uses\n"
            "native byte values.\n");
    fputs_unlocked(v42, v41);
    sub_4104("sort");
  }
  exit(a1);
}



// Function: cs_enter @ 0x47f1
long long cs_enter(long long a1)
{
  bool v1; // dl

  v1 = pthread_sigmask(0, &set, (__sigset_t *)(a1 + 8)) == 0;
  *(char *)a1 = v1;
  return a1;
}



// Function: cs_leave @ 0x4833
int cs_leave(unsigned char *a1)
{
  int result; // eax

  result = *a1;
  if ( (char)result )
    return pthread_sigmask(2, (const __sigset_t *)(a1 + 8), 0);
  return result;
}



// Function: proctab_hasher @ 0x486b
unsigned long long proctab_hasher(long long a1, unsigned long long a2)
{
  return *(int *)(a1 + 8) % a2;
}



// Function: proctab_comparator @ 0x489a
bool proctab_comparator(long long a1, long long a2)
{
  return *(int *)(a1 + 8) == *(int *)(a2 + 8);
}



// Function: reap @ 0x48cf
long long reap(int a1)
{
  __pid_t v1; // eax
  long long v2; // r12
  char *v3; // rbx
  int *v4; // rax
  long long v5; // rbx
  char *v6; // rax
  int stat_loc; // [rsp+10h] [rbp-20h] BYREF
  __pid_t v9; // [rsp+14h] [rbp-1Ch]
  unsigned long long v10; // [rsp+18h] [rbp-18h]

  v10 = __readfsqword(0x28u);
  if ( a1 )
    v1 = a1;
  else
    v1 = -1;
  v9 = waitpid(v1, &stat_loc, a1 == 0);
  if ( v9 < 0 )
  {
    v2 = sub_15059(4, s1);
    v3 = gettext("waiting for %s [-d]");
    v4 = __errno_location();
    error(2, *v4, v3, v2);
  }
  if ( v9 > 0 && (a1 > 0 || (unsigned char)sub_4A90((unsigned int)v9)) )
  {
    if ( (stat_loc & 0x7F) != 0 || BYTE1(stat_loc) )
    {
      v5 = sub_15059(4, s1);
      v6 = gettext("%s [-d] terminated abnormally");
      error(2, 0, v6, v5);
    }
    --dword_23730;
  }
  return (unsigned int)v9;
}



// Function: register_proc @ 0x4a08
long long register_proc(long long a1)
{
  long long result; // rax

  if ( !qword_23728 )
  {
    qword_23728 = sub_1120D(47, 0, sub_486B, sub_489A, 0);
    if ( !qword_23728 )
      sub_1865C();
  }
  *(char *)(a1 + 12) = 1;
  result = sub_11F4E(qword_23728, a1);
  if ( !result )
    sub_1865C();
  return result;
}



// Function: delete_proc @ 0x4a90
long long delete_proc(int a1)
{
  long long v2; // [rsp+18h] [rbp-28h]
  char v3[8]; // [rsp+20h] [rbp-20h] BYREF
  int v4; // [rsp+28h] [rbp-18h]
  unsigned long long v5; // [rsp+38h] [rbp-8h]

  v5 = __readfsqword(0x28u);
  v4 = a1;
  v2 = sub_11FBE(qword_23728, v3);
  if ( !v2 )
    return 0;
  *(char *)(v2 + 12) = 2;
  return 1;
}



// Function: wait_proc @ 0x4aff
long long wait_proc(int a1)
{
  long long result; // rax

  result = sub_4A90(a1);
  if ( (char)result )
    return sub_48CF(a1);
  return result;
}



// Function: reap_exited @ 0x4b29
long long reap_exited()
{
  long long result; // rax

  do
  {
    result = (unsigned int)dword_23730;
    if ( dword_23730 <= 0 )
      break;
    result = sub_48CF(0);
  }
  while ( (int)result );
  return result;
}



// Function: reap_some @ 0x4b4f
long long reap_some()
{
  sub_48CF(-1);
  return sub_4B29();
}



// Function: reap_all @ 0x4b69
long long reap_all()
{
  long long result; // rax

  while ( 1 )
  {
    result = (unsigned int)dword_23730;
    if ( dword_23730 <= 0 )
      break;
    sub_48CF(-1);
  }
  return result;
}



// Function: cleanup @ 0x4b8b
long long cleanup()
{
  long long result; // rax
  long long i; // [rsp+8h] [rbp-8h]

  result = qword_23720;
  for ( i = qword_23720; i; i = *(long long *)i )
  {
    unlink((const char *)(i + 13));
    result = *(long long *)i;
  }
  qword_23720 = 0;
  return result;
}



// Function: exit_cleanup @ 0x4bd4
unsigned long long exit_cleanup()
{
  unsigned char v1[136]; // [rsp+0h] [rbp-90h] BYREF
  unsigned long long v2; // [rsp+88h] [rbp-8h]

  v2 = __readfsqword(0x28u);
  if ( qword_23720 )
  {
    sub_47F1((long long)v1);
    sub_4B8B();
    sub_4833(v1);
  }
  sub_FA34();
  return v2 - __readfsqword(0x28u);
}



// Function: create_temp_file @ 0x4c3d
long long *create_temp_file(int *a1, char a2)
{
  long long v2; // r12
  char *v3; // rbx
  int *v4; // rax
  int v6; // [rsp+18h] [rbp-C8h]
  int v7; // [rsp+1Ch] [rbp-C4h]
  long long *ptr; // [rsp+20h] [rbp-C0h]
  char *s; // [rsp+28h] [rbp-B8h]
  size_t n; // [rsp+30h] [rbp-B0h]
  unsigned char v11[136]; // [rsp+40h] [rbp-A0h] BYREF
  unsigned long long v12; // [rsp+C8h] [rbp-18h]

  v12 = __readfsqword(0x28u);
  s = *(char **)(qword_23668 + 8 * qword_237A0);
  n = strlen(s);
  ptr = (long long *)sub_18024((n + 32) & 0xFFFFFFFFFFFFFFF8LL);
  memcpy((char *)ptr + 13, s, n);
  memcpy((char *)ptr + n + 13, "/sortXXXXXX", 0xCu);
  *ptr = 0;
  if ( ++qword_237A0 == qword_23670 )
    qword_237A0 = 0;
  sub_47F1((long long)v11);
  v6 = sub_1704D((char *)ptr + 13, 0x80000);
  if ( v6 >= 0 )
  {
    *off_23100 = (long long)ptr;
    off_23100 = ptr;
  }
  v7 = *__errno_location();
  sub_4833(v11);
  *__errno_location() = v7;
  if ( v6 < 0 )
  {
    if ( a2 != 1 || *__errno_location() != 24 )
    {
      v2 = sub_15059(4, s);
      v3 = gettext("cannot create temporary file in %s");
      v4 = __errno_location();
      error(2, *v4, v3, v2);
    }
    free(ptr);
    ptr = 0;
  }
  *a1 = v6;
  return ptr;
}



// Function: get_outstatus @ 0x4e8c
struct stat *get_outstatus()
{
  int v0; // eax

  if ( !dword_237A8 )
  {
    if ( fstat(1, &buf) )
      v0 = *__errno_location();
    else
      v0 = -1;
    dword_237A8 = v0;
  }
  if ( dword_237A8 >= 0 )
    return 0;
  else
    return &buf;
}



// Function: stream_open @ 0x4ee4
FILE *stream_open(const char *a1, const char *a2)
{
  FILE *v2; // rax
  long long v3; // rbx
  char *v4; // rax
  int errnum; // [rsp+18h] [rbp-28h]
  int fd; // [rsp+1Ch] [rbp-24h]
  long long v8; // [rsp+20h] [rbp-20h]
  struct stat *v9; // [rsp+28h] [rbp-18h]

  if ( *a2 == 114 )
  {
    if ( !strcmp(a1, "-") )
    {
      byte_23683 = 1;
      v8 = stdin;
      sub_FED8(stdin, 2);
    }
    else
    {
      fd = open(a1, 0x80000);
      if ( fd < 0 )
        v2 = 0;
      else
        v2 = fdopen(fd, a2);
      v8 = (long long)v2;
      sub_FED8(v2, 2);
    }
  }
  else
  {
    if ( *a2 != 119 )
      __assert_fail("!\"unexpected mode passed to stream_open\"", "src/sort.c", 0x3D5u, "stream_open");
    if ( a1 )
    {
      if ( ftruncate(1, 0) )
      {
        errnum = *__errno_location();
        v9 = sub_4E8C();
        if ( !v9 || (v9->st_mode & 0xF000) == 0x8000 )
        {
          v3 = sub_151EB(0, 3, a1);
          v4 = gettext("%s: error truncating");
          error(2, errnum, v4, v3);
        }
      }
    }
    return stdout;
  }
  return (FILE *)v8;
}



// Function: xfopen @ 0x505e
FILE *xfopen(char *a1, const char *a2)
{
  char *v2; // rax
  FILE *v4; // [rsp+18h] [rbp-8h]

  v4 = sub_4EE4(a1, a2);
  if ( !v4 )
  {
    v2 = gettext("open failed");
    sub_4464(v2, a1);
  }
  return v4;
}



// Function: xfclose @ 0x50b7
void xfclose(FILE *a1, char *a2)
{
  int v2; // eax
  char *v3; // rax
  char *v5; // rax

  v2 = fileno(a1);
  if ( v2 )
  {
    if ( v2 == 1 )
    {
      if ( fflush_unlocked(a1) )
      {
        v3 = gettext("fflush failed");
        sub_4464(v3, a2);
      }
    }
    else if ( (unsigned int)sub_FF17(a1) )
    {
      v5 = gettext("close failed");
      sub_4464(v5, a2);
    }
  }
  else
  {
    clearerr_unlocked(a1);
  }
}



// Function: move_fd @ 0x515d
int move_fd(int a1, int a2)
{
  int result; // eax

  result = a1;
  if ( a1 != a2 )
  {
    dup2(a1, a2);
    return close(a1);
  }
  return result;
}



// Function: pipe_fork @ 0x5193
long long pipe_fork(int *a1, long long a2)
{
  __pid_t v5; // [rsp+18h] [rbp-A8h]
  int v6; // [rsp+1Ch] [rbp-A4h]
  int v7; // [rsp+1Ch] [rbp-A4h]
  double v8; // [rsp+20h] [rbp-A0h]
  long long v9; // [rsp+28h] [rbp-98h]
  unsigned char v10[136]; // [rsp+30h] [rbp-90h] BYREF
  unsigned long long v11; // [rsp+B8h] [rbp-8h]

  v11 = __readfsqword(0x28u);
  v8 = 0.25;
  if ( (int)sub_1311D(a1, 0x80000) < 0 )
    return 0xFFFFFFFFLL;
  if ( dword_23024 + 1 < (unsigned int)dword_23730 )
    sub_4B4F();
  while ( a2-- )
  {
    sub_47F1((long long)v10);
    v9 = qword_23720;
    qword_23720 = 0;
    v5 = fork();
    v6 = *__errno_location();
    if ( v5 )
      qword_23720 = v9;
    sub_4833(v10);
    *__errno_location() = v6;
    if ( v5 >= 0 || *__errno_location() != 11 )
      break;
    sub_18863(v8);
    v8 = v8 + v8;
    sub_4B29();
  }
  if ( v5 >= 0 )
  {
    if ( v5 )
    {
      ++dword_23730;
    }
    else
    {
      close(0);
      close(1);
    }
  }
  else
  {
    v7 = *__errno_location();
    close(*a1);
    close(a1[1]);
    *__errno_location() = v7;
  }
  return (unsigned int)v5;
}



// Function: maybe_create_temp @ 0x5372
long long *maybe_create_temp(FILE **a1, char a2)
{
  int v3; // eax
  int *v4; // rax
  char *v5; // rbx
  char *v6; // rax
  int fd; // [rsp+14h] [rbp-2Ch] BYREF
  long long *v8; // [rsp+18h] [rbp-28h]
  int v9; // [rsp+20h] [rbp-20h] BYREF
  int v10; // [rsp+24h] [rbp-1Ch]
  unsigned long long v11; // [rsp+28h] [rbp-18h]

  v11 = __readfsqword(0x28u);
  v8 = sub_4C3D(&fd, a2);
  if ( !v8 )
    return 0;
  *((char *)v8 + 12) = 0;
  if ( s1 )
  {
    v3 = sub_5193(&v9, 4);
    *((int *)v8 + 2) = v3;
    if ( *((int *)v8 + 2) <= 0 )
    {
      if ( !*((int *)v8 + 2) )
      {
        close(v10);
        sub_515D(fd, 1);
        sub_515D(v9, 0);
        execlp(s1, s1, 0);
        v4 = __errno_location();
        sub_439B(*v4, "couldn't execute compress program");
      }
    }
    else
    {
      close(fd);
      close(v9);
      fd = v10;
      sub_4A08((long long)v8);
    }
  }
  *a1 = fdopen(fd, "w");
  if ( !*a1 )
  {
    v5 = (char *)v8 + 13;
    v6 = gettext("couldn't create temporary file");
    sub_4464(v6, v5);
  }
  return v8;
}



// Function: create_temp @ 0x54f9
long long *create_temp(FILE **a1)
{
  return sub_5372(a1, 0);
}



// Function: open_temp @ 0x551c
long long open_temp(long long a1)
{
  long long v2; // r12
  char *v3; // rbx
  int *v4; // rax
  int *v5; // rax
  FILE *v6; // rax
  int fd; // [rsp+1Ch] [rbp-34h]
  int v8; // [rsp+20h] [rbp-30h]
  char v9[12]; // [rsp+24h] [rbp-2Ch]
  int v10; // [rsp+30h] [rbp-20h] BYREF
  int v11; // [rsp+34h] [rbp-1Ch]
  unsigned long long v12; // [rsp+38h] [rbp-18h]

  v12 = __readfsqword(0x28u);
  *(long long *)&v9[4] = 0;
  if ( *(char *)(a1 + 12) == 1 )
    sub_4AFF(*(int *)(a1 + 8));
  fd = open((const char *)(a1 + 13), 0);
  if ( fd < 0 )
    return 0;
  v8 = sub_5193(&v10, 9);
  if ( v8 == -1 )
  {
    if ( *__errno_location() != 24 )
    {
      v2 = sub_15059(4, s1);
      v3 = gettext("couldn't create process for %s -d");
      v4 = __errno_location();
      error(2, *v4, v3, v2);
    }
    close(fd);
    *__errno_location() = 24;
  }
  else
  {
    if ( !v8 )
    {
      close(v10);
      sub_515D(fd, 0);
      sub_515D(v11, 1);
      execlp(s1, s1, "-d", 0);
      v5 = __errno_location();
      sub_439B(*v5, "couldn't execute compress program (with -d)");
    }
    *(int *)(a1 + 8) = v8;
    sub_4A08(a1);
    close(fd);
    close(v11);
    v6 = fdopen(v10, "r");
    *(long long *)&v9[4] = v6;
    if ( !v6 )
    {
      *(long long *)v9 = (unsigned int)*__errno_location();
      close(v10);
      *__errno_location() = *(int *)v9;
    }
  }
  return *(long long *)&v9[4];
}



// Function: add_temp_dir @ 0x570f
long long add_temp_dir(long long a1)
{
  long long v1; // rax
  long long *v2; // rdx

  if ( qword_23670 == qword_23678 )
    qword_23668 = sub_18221(qword_23668, &qword_23678, 8);
  v1 = qword_23670++;
  v2 = (long long *)(qword_23668 + 8 * v1);
  *v2 = a1;
  return a1;
}



// Function: zaptemp @ 0x5782
unsigned long long zaptemp(const char *a1)
{
  long long v1; // rbx
  char *v2; // rax
  int errnum; // [rsp+10h] [rbp-C0h]
  int v5; // [rsp+14h] [rbp-BCh]
  long long *i; // [rsp+18h] [rbp-B8h]
  int *ptr; // [rsp+20h] [rbp-B0h]
  long long v8; // [rsp+28h] [rbp-A8h]
  unsigned char v9[136]; // [rsp+30h] [rbp-A0h] BYREF
  unsigned long long v10; // [rsp+B8h] [rbp-18h]

  v10 = __readfsqword(0x28u);
  for ( i = &qword_23720; ; i = (long long *)*i )
  {
    ptr = (int *)*i;
    if ( a1 == (const char *)(*i + 13) )
      break;
  }
  if ( *((char *)ptr + 12) == 1 )
    sub_4AFF(ptr[2]);
  v8 = *(long long *)ptr;
  sub_47F1((long long)v9);
  v5 = unlink(a1);
  errnum = *__errno_location();
  *i = v8;
  sub_4833(v9);
  if ( v5 )
  {
    v1 = sub_151EB(0, 3, a1);
    v2 = gettext("warning: cannot remove: %s");
    error(0, errnum, v2, v1);
  }
  if ( !v8 )
    off_23100 = i;
  free(ptr);
  return v10 - __readfsqword(0x28u);
}



// Function: struct_month_cmp @ 0x590b
int struct_month_cmp(const char **a1, const char **a2)
{
  return strcmp(*a1, *a2);
}



// Function: inittables @ 0x594a
void inittables()
{
  bool v0; // al
  const unsigned short *v1; // rbx
  int v2; // ecx
  long long v3; // rax
  int c[2]; // [rsp+0h] [rbp-40h]
  int ca[2]; // [rsp+0h] [rbp-40h]
  size_t i; // [rsp+8h] [rbp-38h]
  long long v7; // [rsp+10h] [rbp-30h]
  const char *s; // [rsp+18h] [rbp-28h]
  size_t v9; // [rsp+20h] [rbp-20h]
  char *v10; // [rsp+28h] [rbp-18h]

  for ( *(long long *)c = 0; *(long long *)c <= 0xFFu; ++*(long long *)c )
  {
    byte_23260[*(long long *)c] = sub_4051(LOBYTE(c[0]));
    byte_23360[*(long long *)c] = ((*__ctype_b_loc())[c[0]] & 0x4000) == 0;
    v0 = ((*__ctype_b_loc())[c[0]] & 8) == 0 && (unsigned char)sub_4051(LOBYTE(c[0])) != 1;
    byte_23460[*(long long *)c] = v0;
    byte_23560[*(long long *)c] = toupper(c[0]);
  }
  if ( byte_2322A )
  {
    for ( *(long long *)ca = 0; *(long long *)ca <= 0xBu; ++*(long long *)ca )
    {
      s = nl_langinfo(ca[0] + 131086);
      v9 = strlen(s);
      v10 = (char *)sub_18024(v9 + 1);
      (&off_23040)[2 * *(long long *)ca] = v10;
      *((int *)&unk_23048 + 4 * *(long long *)ca) = ca[0] + 1;
      v7 = 0;
      for ( i = 0; i < v9; ++i )
      {
        v1 = *__ctype_b_loc();
        if ( (v1[(unsigned char)sub_4042((unsigned int)s[i])] & 1) == 0 )
        {
          v2 = (unsigned char)sub_4042((unsigned int)s[i]);
          v3 = v7++;
          v10[v3] = byte_23560[v2];
        }
      }
      v10[v7] = 0;
    }
    qsort(&off_23040, 0xCu, 0x10u, (__compar_fn_t)compar);
  }
}



// Function: specify_nmerge @ 0x5ba8
unsigned long long specify_nmerge(unsigned int a1, char a2, long long a3)
{
  int v3; // eax
  long long v4; // r12
  long long v5; // rbx
  char *v6; // rax
  long long v7; // r12
  long long v8; // rbx
  char *v9; // rax
  long long v10; // r12
  long long v11; // rbx
  char *v12; // rax
  long long v13; // r12
  long long v14; // rbx
  char *v15; // rax
  unsigned int v18; // [rsp+10h] [rbp-50h]
  unsigned int v19; // [rsp+14h] [rbp-4Ch]
  long long v20; // [rsp+18h] [rbp-48h] BYREF
  struct rlimit rlimits; // [rsp+20h] [rbp-40h] BYREF
  char v22[11]; // [rsp+3Dh] [rbp-23h] BYREF
  unsigned long long v23; // [rsp+48h] [rbp-18h]

  v23 = __readfsqword(0x28u);
  v18 = sub_18B54(a3, 0, 10, &v20, locale);
  if ( getrlimit(RLIMIT_NOFILE, &rlimits) )
    v3 = 17;
  else
    v3 = LODWORD(rlimits.rlim_cur) - 3;
  v19 = v3;
  if ( v18 )
    goto LABEL_11;
  dword_23024 = v20;
  if ( (unsigned int)v20 != v20 )
  {
    v18 = 1;
    goto LABEL_11;
  }
  if ( (unsigned int)dword_23024 <= 1 )
  {
    v4 = sub_154A0(a3);
    v5 = *((long long *)&longopts.name + 4 * (int)a1);
    v6 = gettext("invalid --%s argument %s");
    error(0, 0, v6, v5, v4);
    v7 = sub_154A0("2");
    v8 = *((long long *)&longopts.name + 4 * (int)a1);
    v9 = gettext("minimum --%s argument is %s");
    error(2, 0, v9, v8, v7);
  }
  if ( v19 < dword_23024 )
  {
    v18 = 1;
LABEL_11:
    if ( v18 == 1 )
    {
      v10 = sub_154A0(a3);
      v11 = *((long long *)&longopts.name + 4 * (int)a1);
      v12 = gettext("--%s argument %s too large");
      error(0, 0, v12, v11, v10);
      v13 = sub_12802(v19, v22);
      v14 = *((long long *)&longopts.name + 4 * (int)a1);
      v15 = gettext("maximum --%s argument with current rlimit is %s");
      error(2, 0, v15, v14, v13);
    }
    sub_18A33(v18, a1, (unsigned int)a2, &longopts, a3);
  }
  return v23 - __readfsqword(0x28u);
}



// Function: specify_sort_size @ 0x5e0c
unsigned long long specify_sort_size(unsigned int a1, char a2, long long a3)
{
  int v4; // eax
  double v5; // xmm1_8
  double v6; // xmm0_8
  unsigned long long v7; // rax
  long long v8; // rax
  unsigned int v12; // [rsp+1Ch] [rbp-24h]
  unsigned long long v13; // [rsp+20h] [rbp-20h] BYREF
  char *v14; // [rsp+28h] [rbp-18h] BYREF
  double v15; // [rsp+30h] [rbp-10h]
  unsigned long long v16; // [rsp+38h] [rbp-8h]

  v16 = __readfsqword(0x28u);
  v12 = sub_18B54(a3, &v14, 10, &v13, "EgGkKmMPtTYZ");
  if ( !v12 && (unsigned int)(*(v14 - 1) - 48) <= 9 )
  {
    if ( v13 > 0x3FFFFFFFFFFFFFLL )
      v12 = 1;
    else
      v13 <<= 10;
  }
  if ( v12 == 2 && (unsigned int)(*(v14 - 1) - 48) <= 9 && !v14[1] )
  {
    v4 = *v14;
    if ( v4 == 37 )
    {
      v5 = sub_12E84(a3, &v14);
      if ( (v13 & 0x8000000000000000LL) != 0LL )
        v6 = (double)(int)(v13 & 1 | (v13 >> 1)) + (double)(int)(v13 & 1 | (v13 >> 1));
      else
        v6 = (double)(int)v13;
      v15 = v6 * v5 / 100.0;
      if ( v15 >= 1.844674407370955e19 )
      {
        v12 = 1;
      }
      else
      {
        if ( v15 >= 9.223372036854776e18 )
          v7 = (unsigned int)(int)(v15 - 9.223372036854776e18) ^ 0x8000000000000000LL;
        else
          v7 = (unsigned int)(int)v15;
        v13 = v7;
        v12 = 0;
      }
    }
    else if ( v4 == 98 )
    {
      v12 = 0;
    }
  }
  if ( v12 )
    sub_18A33(v12, a1, (unsigned int)a2, &longopts, a3);
  if ( v13 >= qword_23660 )
  {
    qword_23660 = v13;
    v8 = v13;
    if ( 34 * (unsigned long long)(unsigned int)dword_23024 >= v13 )
      v8 = 34LL * (unsigned int)dword_23024;
    qword_23660 = v8;
  }
  return v16 - __readfsqword(0x28u);
}



// Function: specify_nthreads @ 0x6042
long long specify_nthreads(unsigned int a1, char a2, long long a3)
{
  char *v4; // rax
  unsigned int v6; // [rsp+1Ch] [rbp-14h]
  long long v7[2]; // [rsp+20h] [rbp-10h] BYREF

  v7[1] = __readfsqword(0x28u);
  v6 = sub_18B54(a3, 0, 10, v7, locale);
  if ( v6 == 1 )
    return -1;
  if ( v6 )
    sub_18A33(v6, a1, (unsigned int)a2, &longopts, a3);
  if ( !v7[0] )
  {
    v4 = gettext("number in parallel must be nonzero");
    error(2, 0, v4);
  }
  return v7[0];
}



// Function: default_sort_size @ 0x610d
long long default_sort_size()
{
  double v0; // xmm0_8
  double v1; // xmm0_8
  double v2; // xmm0_8
  double v3; // xmm0_8
  long long result; // rax
  rlim_t rlim_cur; // [rsp+0h] [rbp-40h]
  unsigned long long v6; // [rsp+0h] [rbp-40h]
  double v7; // [rsp+8h] [rbp-38h]
  double v8; // [rsp+10h] [rbp-30h]
  double v9; // [rsp+18h] [rbp-28h]
  struct rlimit rlimits; // [rsp+20h] [rbp-20h] BYREF
  unsigned long long v11; // [rsp+38h] [rbp-8h]

  v11 = __readfsqword(0x28u);
  rlim_cur = -1;
  if ( !getrlimit(RLIMIT_DATA, &rlimits) && rlimits.rlim_cur != -1 )
    rlim_cur = rlimits.rlim_cur;
  if ( !getrlimit(RLIMIT_AS, &rlimits) && rlim_cur > rlimits.rlim_cur )
    rlim_cur = rlimits.rlim_cur;
  v6 = rlim_cur >> 1;
  if ( !getrlimit(__RLIMIT_RSS, &rlimits) && v6 > 15 * (rlimits.rlim_cur >> 4) )
    v6 = 15 * (rlimits.rlim_cur >> 4);
  v7 = sub_12FAB();
  v8 = sub_12E84(5, &rlimits);
  if ( v7 <= v8 / 8.0 )
    v0 = v8 / 8.0;
  else
    v0 = v7;
  v9 = v0;
  if ( (v6 & 0x8000000000000000LL) != 0LL )
    v1 = (double)(int)(v6 & 1 | (v6 >> 1)) + (double)(int)(v6 & 1 | (v6 >> 1));
  else
    v1 = (double)(int)v6;
  if ( v1 > v8 * 0.75 )
  {
    v2 = 0.75 * v8;
    if ( 0.75 * v8 >= 9.223372036854776e18 )
      v6 = (unsigned int)(int)(v2 - 9.223372036854776e18) ^ 0x8000000000000000LL;
    else
      v6 = (unsigned int)(int)v2;
  }
  if ( (v6 & 0x8000000000000000LL) != 0LL )
    v3 = (double)(int)(v6 & 1 | (v6 >> 1)) + (double)(int)(v6 & 1 | (v6 >> 1));
  else
    v3 = (double)(int)v6;
  if ( v3 > v9 )
  {
    if ( v9 >= 9.223372036854776e18 )
      v6 = (unsigned int)(int)(v9 - 9.223372036854776e18) ^ 0x8000000000000000LL;
    else
      v6 = (unsigned int)(int)v9;
  }
  result = v6;
  if ( 34 * (unsigned long long)(unsigned int)dword_23024 >= v6 )
    return 34LL * (unsigned int)dword_23024;
  return result;
}



// Function: sort_buffer_size @ 0x637c
long long sort_buffer_size(long long a1, unsigned long long a2, long long a3, unsigned long long a4, long long a5)
{
  int v5; // eax
  bool v6; // al
  char *v7; // rbx
  char *v8; // rax
  long long v12; // [rsp+38h] [rbp-D8h]
  unsigned long long i; // [rsp+40h] [rbp-D0h]
  __off_t st_size; // [rsp+48h] [rbp-C8h]
  unsigned long long v15; // [rsp+50h] [rbp-C0h]
  unsigned long long v16; // [rsp+58h] [rbp-B8h]
  struct stat buf; // [rsp+60h] [rbp-B0h] BYREF
  unsigned long long v18; // [rsp+F8h] [rbp-18h]

  v18 = __readfsqword(0x28u);
  v15 = a5 + 1;
  v12 = a5 + 2;
  for ( i = 0; i < a4; ++i )
  {
    if ( i >= a2 )
    {
      if ( !strcmp(*(const char **)(8 * i + a3), "-") )
        v6 = fstat(0, &buf) != 0;
      else
        v6 = stat(*(const char **)(8 * i + a3), &buf) != 0;
    }
    else
    {
      v5 = fileno(*(FILE **)(8 * i + a1));
      v6 = fstat(v5, &buf) != 0;
    }
    if ( v6 )
    {
      v7 = *(char **)(8 * i + a3);
      v8 = gettext("stat failed");
      sub_4464(v8, v7);
    }
    if ( (buf.st_mode & 0xF000) == 0x8000 )
    {
      st_size = buf.st_size;
    }
    else
    {
      if ( qword_23660 )
        return qword_23660;
      st_size = (__off_t)&unk_20000;
    }
    if ( !qword_23850 )
    {
      qword_23850 = qword_23660;
      if ( !qword_23660 )
        qword_23850 = sub_610D();
    }
    v16 = v15 * st_size + 1;
    if ( v16 / v15 != st_size || v16 >= qword_23850 - v12 )
      return qword_23850;
    v12 += v16;
  }
  return v12;
}



// Function: initbuf @ 0x6618
long long initbuf(long long a1, long long a2, unsigned long long a3)
{
  unsigned long long sizea; // [rsp+8h] [rbp-18h]

  while ( 1 )
  {
    sizea = (a3 & 0xFFFFFFFFFFFFFFE0LL) + 32;
    *(long long *)a1 = malloc(sizea);
    if ( *(long long *)a1 )
      break;
    a3 = sizea >> 1;
    if ( a3 <= a2 + 1 )
      sub_1865C();
  }
  *(long long *)(a1 + 40) = a2;
  *(long long *)(a1 + 24) = sizea;
  *(long long *)(a1 + 16) = 0;
  *(long long *)(a1 + 32) = *(long long *)(a1 + 16);
  *(long long *)(a1 + 8) = *(long long *)(a1 + 32);
  *(char *)(a1 + 48) = 0;
  return a1;
}



// Function: buffer_linelim @ 0x66d0
long long buffer_linelim(long long *a1)
{
  return *a1 + a1[3];
}



// Function: begfield @ 0x66f4
char *begfield(char **a1, long long *a2)
{
  char *result; // rax
  char *v5; // [rsp+10h] [rbp-20h]
  long long v6; // [rsp+18h] [rbp-18h]
  unsigned long long v7; // [rsp+20h] [rbp-10h]
  long long v8; // [rsp+28h] [rbp-8h]

  v5 = *a1;
  v7 = (unsigned long long)&(*a1)[(long long)(a1[1] - 1)];
  v6 = *a2;
  v8 = a2[1];
  if ( dword_23020 == 128 )
  {
    while ( (unsigned long long)v5 < v7 )
    {
      if ( !v6-- )
        break;
      while ( (unsigned long long)v5 < v7 && byte_23260[(unsigned char)sub_4042((unsigned int)*v5)] )
        ++v5;
      while ( (unsigned long long)v5 < v7 && byte_23260[(unsigned char)sub_4042((unsigned int)*v5)] != 1 )
        ++v5;
    }
  }
  else
  {
    while ( (unsigned long long)v5 < v7 )
    {
      if ( !v6-- )
        break;
      while ( (unsigned long long)v5 < v7 && *v5 != dword_23020 )
        ++v5;
      if ( (unsigned long long)v5 < v7 )
        ++v5;
    }
  }
  if ( *((char *)a2 + 48) )
  {
    while ( (unsigned long long)v5 < v7 && byte_23260[(unsigned char)sub_4042((unsigned int)*v5)] )
      ++v5;
  }
  result = (char *)v7;
  if ( v7 > (unsigned long long)&v5[v8] )
    return &v5[v8];
  return result;
}



// Function: limfield @ 0x6898
char *limfield(char **a1, long long a2)
{
  unsigned long long v4; // rax
  char *v6; // [rsp+10h] [rbp-20h]
  long long v7; // [rsp+18h] [rbp-18h]
  unsigned long long v8; // [rsp+20h] [rbp-10h]
  long long v9; // [rsp+28h] [rbp-8h]

  v6 = *a1;
  v8 = (unsigned long long)&(*a1)[(long long)(a1[1] - 1)];
  v7 = *(long long *)(a2 + 16);
  v9 = *(long long *)(a2 + 24);
  if ( !v9 )
    ++v7;
  if ( dword_23020 == 128 )
  {
    while ( (unsigned long long)v6 < v8 )
    {
      if ( !v7-- )
        break;
      while ( (unsigned long long)v6 < v8 && byte_23260[(unsigned char)sub_4042((unsigned int)*v6)] )
        ++v6;
      while ( (unsigned long long)v6 < v8 && byte_23260[(unsigned char)sub_4042((unsigned int)*v6)] != 1 )
        ++v6;
    }
  }
  else
  {
    while ( (unsigned long long)v6 < v8 )
    {
      if ( !v7-- )
        break;
      while ( (unsigned long long)v6 < v8 && *v6 != dword_23020 )
        ++v6;
      if ( (unsigned long long)v6 < v8 && (v7 || v9) )
        ++v6;
    }
  }
  if ( v9 )
  {
    if ( *(char *)(a2 + 49) )
    {
      while ( (unsigned long long)v6 < v8 && byte_23260[(unsigned char)sub_4042((unsigned int)*v6)] )
        ++v6;
    }
    v4 = v8;
    if ( v8 > (unsigned long long)&v6[v9] )
      return &v6[v9];
    return (char *)v4;
  }
  return v6;
}



// Function: fillbuf @ 0x6a5e
long long fillbuf(long long *a1, FILE *a2, char *a3)
{
  char *v4; // rax
  char *v5; // rax
  char *v6; // rax
  char *v7; // rax
  char *v8; // rax
  char *v9; // rax
  long long v10; // rax
  char v12; // [rsp+2Fh] [rbp-71h]
  long long v13; // [rsp+30h] [rbp-70h] BYREF
  unsigned long long v14; // [rsp+38h] [rbp-68h]
  void *ptr; // [rsp+40h] [rbp-60h]
  char **v16; // [rsp+48h] [rbp-58h]
  unsigned long long v17; // [rsp+50h] [rbp-50h]
  char *v18; // [rsp+58h] [rbp-48h]
  char *v19; // [rsp+60h] [rbp-40h]
  long long *v20; // [rsp+68h] [rbp-38h]
  long long v21; // [rsp+70h] [rbp-30h]
  long long v22; // [rsp+78h] [rbp-28h]
  size_t n; // [rsp+80h] [rbp-20h]
  size_t v24; // [rsp+88h] [rbp-18h]
  char *v25; // [rsp+90h] [rbp-10h]
  unsigned long long v26; // [rsp+98h] [rbp-8h]

  v26 = __readfsqword(0x28u);
  v20 = (long long *)qword_23688;
  v12 = byte_23010;
  v21 = a1[5];
  v14 = qword_23018 - 34;
  if ( *((char *)a1 + 48) )
    return 0;
  if ( a1[1] != a1[4] )
  {
    memmove((void *)*a1, (const void *)(*a1 + a1[1] - a1[4]), a1[4]);
    a1[1] = a1[4];
    a1[2] = 0;
  }
  while ( 1 )
  {
    ptr = (void *)(*a1 + a1[1]);
    v22 = sub_66D0(a1);
    v16 = (char **)(-32LL * a1[2] + v22);
    v17 = v22 - v21 * a1[2] - (long long)ptr;
    v4 = a1[2] ? &v16[1][(long long)*v16] : (char *)*a1;
    v18 = v4;
    do
    {
      if ( v17 <= v21 + 1 )
        break;
      n = (v17 - 1) / (v21 + 1);
      v24 = fread_unlocked(ptr, 1u, n, a2);
      v19 = (char *)ptr + v24;
      v17 -= v24;
      if ( v24 != n )
      {
        if ( ferror_unlocked(a2) )
        {
          v5 = gettext("read failed");
          sub_4464(v5, a3);
        }
        if ( feof_unlocked(a2) )
        {
          *((char *)a1 + 48) = 1;
          if ( v19 == (char *)*a1 )
            return 0;
          if ( v18 != v19 && v12 != *(v19 - 1) )
          {
            v6 = v19++;
            *v6 = v12;
          }
        }
      }
      while ( 1 )
      {
        v25 = (char *)memchr(ptr, v12, v19 - (char *)ptr);
        if ( !v25 )
          break;
        *v25 = 0;
        ptr = v25 + 1;
        v16 -= 4;
        *v16 = v18;
        v16[1] = (char *)((char *)ptr - v18);
        v7 = v16[1];
        if ( v14 >= (unsigned long long)v7 )
          v7 = (char *)v14;
        v14 = (unsigned long long)v7;
        v17 -= v21;
        if ( v20 )
        {
          if ( v20[2] == -1 )
            v8 = v25;
          else
            v8 = sub_6898(v16, (long long)v20);
          v16[3] = v8;
          if ( *v20 == -1 )
          {
            if ( *((char *)v20 + 48) )
            {
              while ( byte_23260[(unsigned char)sub_4042((unsigned int)*v18)] )
                ++v18;
            }
            v16[2] = v18;
          }
          else
          {
            v9 = sub_66F4(v16, v20);
            v16[2] = v9;
          }
        }
        v18 = (char *)ptr;
      }
      ptr = v19;
    }
    while ( !*((char *)a1 + 48) );
    a1[1] = (char *)ptr - *a1;
    v10 = sub_66D0(a1);
    a1[2] = (v10 - (long long)v16) >> 5;
    if ( a1[2] )
      break;
    v13 = a1[3] >> 5;
    *a1 = sub_18221(*a1, &v13, 32);
    a1[3] = 32 * v13;
  }
  a1[4] = (char *)ptr - v18;
  qword_23018 = v14 + 34;
  return 1;
}



// Function: traverse_raw_number @ 0x6f64
long long traverse_raw_number(char **a1)
{
  int v1; // edx
  char *v2; // rax
  char *v4; // rax
  char v5; // [rsp+Dh] [rbp-Bh]
  bool v6; // [rsp+Eh] [rbp-Ah]
  char v7; // [rsp+Fh] [rbp-9h]
  char *v8; // [rsp+10h] [rbp-8h]

  v8 = *a1;
  v5 = 0;
  v6 = 0;
  while ( 1 )
  {
    v2 = v8++;
    v7 = *v2;
    if ( (unsigned int)(*v2 - 48) > 9 )
      break;
    if ( v5 < v7 )
      v5 = *v2;
    v1 = *v8;
    v6 = v1 == dword_23224;
    if ( v1 == dword_23224 )
      ++v8;
  }
  if ( v6 )
  {
    *a1 = v8 - 2;
    return (unsigned char)v5;
  }
  else
  {
    if ( v7 == byte_23220 )
    {
      while ( 1 )
      {
        v4 = v8++;
        if ( (unsigned int)(*v4 - 48) > 9 )
          break;
        if ( v5 < *v4 )
          v5 = *v4;
      }
    }
    *a1 = v8 - 1;
    return (unsigned char)v5;
  }
}



// Function: find_unit_order @ 0x7042
long long find_unit_order(char *a1)
{
  bool v2; // [rsp+19h] [rbp-17h]
  unsigned int v3; // [rsp+1Ch] [rbp-14h]
  char *v4[2]; // [rsp+20h] [rbp-10h] BYREF

  v4[1] = (char *)__readfsqword(0x28u);
  v2 = *a1 == 45;
  v4[0] = &a1[v2];
  if ( (char)sub_6F64(v4) <= 48 )
    return 0;
  v3 = byte_1C5C0[(unsigned char)*v4[0]];
  if ( v2 )
    return -v3;
  else
    return v3;
}



// Function: human_numcompare @ 0x70e2
long long human_numcompare(char *a1, char *a2)
{
  int v2; // ebx
  unsigned int v6; // [rsp+1Ch] [rbp-14h]

  while ( byte_23260[(unsigned char)sub_4042((unsigned int)(char)*a1)] )
    ++a1;
  while ( byte_23260[(unsigned char)sub_4042((unsigned int)(char)*a2)] )
    ++a2;
  v2 = sub_7042(a1);
  v6 = v2 - sub_7042(a2);
  if ( v6 )
    return v6;
  else
    return sub_1764C(a1, a2, (unsigned int)byte_23220, (unsigned int)dword_23224);
}



// Function: numcompare @ 0x71a3
long long numcompare(char *a1, char *a2)
{
  while ( byte_23260[(unsigned char)sub_4042((unsigned int)*a1)] )
    ++a1;
  while ( byte_23260[(unsigned char)sub_4042((unsigned int)*a2)] )
    ++a2;
  return sub_1764C(a1, a2, (unsigned int)byte_23220, (unsigned int)dword_23224);
}



// Function: nan_compare @ 0x7231
int nan_compare(const char *a1, const char *a2)
{
  long double s; // [rsp+10h] [rbp-30h] BYREF
  long double s2; // [rsp+20h] [rbp-20h] BYREF
  unsigned long long v5; // [rsp+38h] [rbp-8h]

  v5 = __readfsqword(0x28u);
  memset(&s, 0, sizeof(s));
  s = strtold(a1, 0);
  memset(&s2, 0, sizeof(s2));
  s2 = strtold(a2, 0);
  return memcmp(&s, &s2, 0x10u);
}



// Function: general_numcompare @ 0x72d6
long long general_numcompare(const char *a1, const char *a2)
{
  char *endptr; // [rsp+10h] [rbp-40h] BYREF
  char *v4; // [rsp+18h] [rbp-38h] BYREF
  long double v5; // [rsp+20h] [rbp-30h]
  long double v6; // [rsp+30h] [rbp-20h]
  unsigned long long v7; // [rsp+48h] [rbp-8h]

  v7 = __readfsqword(0x28u);
  v5 = strtold(a1, &endptr);
  v6 = strtold(a2, &v4);
  if ( a1 == endptr )
  {
    if ( a2 == v4 )
      return 0;
    else
      return 0xFFFFFFFFLL;
  }
  else if ( a2 == v4 )
  {
    return 1;
  }
  else if ( v6 <= v5 )
  {
    if ( v5 <= v6 )
    {
      if ( v6 == v5 )
        return 0;
      else
        return 0xFFFFFFFFLL;
    }
    else
    {
      return 1;
    }
  }
  else
  {
    return 0xFFFFFFFFLL;
  }
}



// Function: getmonth @ 0x73f3
long long getmonth(char *a1, long long *a2)
{
  unsigned char v3; // al
  unsigned char v4; // bl
  unsigned char v5; // al
  unsigned char v6; // bl
  unsigned long long v8; // [rsp+18h] [rbp-30h]
  unsigned long long v9; // [rsp+20h] [rbp-28h]
  char *v10; // [rsp+28h] [rbp-20h]
  char *i; // [rsp+30h] [rbp-18h]
  unsigned long long v12; // [rsp+38h] [rbp-10h]

  v8 = 0;
  v9 = 12;
  while ( byte_23260[(unsigned char)sub_4042((unsigned int)*a1)] )
    ++a1;
  while ( 2 )
  {
    v12 = (v8 + v9) >> 1;
    v10 = a1;
    for ( i = (&off_23040)[2 * v12]; ; ++i )
    {
      if ( !*i )
      {
        if ( a2 )
          *a2 = v10;
        return *((unsigned int *)&unk_23048 + 4 * v12);
      }
      v3 = sub_4042((unsigned int)*v10);
      v4 = sub_4042((unsigned int)(char)byte_23560[v3]);
      if ( v4 < (unsigned char)sub_4042((unsigned int)*i) )
      {
        v9 = (v8 + v9) >> 1;
        goto LABEL_14;
      }
      v5 = sub_4042((unsigned int)*v10);
      v6 = sub_4042((unsigned int)(char)byte_23560[v5]);
      if ( v6 > (unsigned char)sub_4042((unsigned int)*i) )
        break;
      ++v10;
    }
    v8 = v12 + 1;
LABEL_14:
    if ( v8 < v9 )
      continue;
    return 0;
  }
}



// Function: random_md5_state_init @ 0x7573
unsigned long long random_md5_state_init(char *a1)
{
  char *v1; // rbx
  char *v2; // rax
  char *v3; // rax
  long long v5; // [rsp+18h] [rbp-38h]
  char v6[24]; // [rsp+20h] [rbp-30h] BYREF
  unsigned long long v7; // [rsp+38h] [rbp-18h]

  v7 = __readfsqword(0x28u);
  v5 = sub_15668(a1, 16);
  if ( !v5 )
  {
    if ( a1 )
      v1 = a1;
    else
      v1 = "getrandom";
    v2 = gettext("open failed");
    sub_4464(v2, v1);
  }
  sub_159B4(v5, v6, 16);
  if ( (unsigned int)sub_15A0F(v5) )
  {
    v3 = gettext("close failed");
    sub_4464(v3, a1);
  }
  sub_FB2C(&qword_23740);
  sub_FB4B(v6, 16, &qword_23740);
  return v7 - __readfsqword(0x28u);
}



// Function: xstrxfrm @ 0x7668
size_t xstrxfrm(char *a1, const char *a2, size_t a3)
{
  char *v3; // rbx
  int *v4; // rax
  char *v5; // rax
  long long v6; // rbx
  char *v7; // rax
  size_t v10; // [rsp+28h] [rbp-18h]

  *__errno_location() = 0;
  v10 = strxfrm(a1, a2, a3);
  if ( *__errno_location() )
  {
    v3 = gettext("string transformation failed");
    v4 = __errno_location();
    error(0, *v4, v3);
    v5 = gettext("set LC_ALL='C' to work around the problem");
    error(0, 0, v5);
    v6 = sub_14F8B(0, 8, a2);
    v7 = gettext("the untransformed string was %s");
    error(2, 0, v7, v6);
  }
  return v10;
}



// Function: compare_random @ 0x7755
long long compare_random(char *a1, size_t a2, char *a3, size_t a4)
{
  unsigned long long v4; // rax
  size_t v5; // rax
  size_t v6; // rax
  char *v7; // rcx
  size_t v8; // rax
  size_t v9; // rdx
  size_t v10; // rdx
  size_t v12; // [rsp+0h] [rbp-1100h]
  char *s; // [rsp+8h] [rbp-10F8h]
  size_t v14; // [rsp+10h] [rbp-10F0h]
  char *src; // [rsp+18h] [rbp-10E8h]
  bool v16; // [rsp+27h] [rbp-10D9h]
  int v17; // [rsp+28h] [rbp-10D8h]
  unsigned int v18; // [rsp+2Ch] [rbp-10D4h]
  char *dest; // [rsp+30h] [rbp-10D0h]
  unsigned long long size; // [rsp+38h] [rbp-10C8h]
  void *ptr; // [rsp+40h] [rbp-10C0h]
  unsigned long long v22; // [rsp+48h] [rbp-10B8h]
  unsigned long long v23; // [rsp+50h] [rbp-10B0h]
  unsigned long long v24; // [rsp+58h] [rbp-10A8h]
  size_t n; // [rsp+60h] [rbp-10A0h]
  size_t v26; // [rsp+68h] [rbp-1098h]
  char s1[16]; // [rsp+70h] [rbp-1090h] BYREF
  char v28[16]; // [rsp+80h] [rbp-1080h] BYREF
  long long v29[2]; // [rsp+90h] [rbp-1070h] BYREF
  __int128 v30; // [rsp+A0h] [rbp-1060h]
  long long v31; // [rsp+B0h] [rbp-1050h]
  long long v32; // [rsp+B8h] [rbp-1048h]
  long long v33; // [rsp+C0h] [rbp-1040h]
  long long v34; // [rsp+C8h] [rbp-1038h]
  long long v35; // [rsp+D0h] [rbp-1030h]
  long long v36; // [rsp+D8h] [rbp-1028h]
  long long v37; // [rsp+E0h] [rbp-1020h]
  int v38; // [rsp+E8h] [rbp-1018h]
  long long v39[2]; // [rsp+ECh] [rbp-1014h] BYREF
  __int128 v40; // [rsp+FCh] [rbp-1004h]
  long long v41; // [rsp+10Ch] [rbp-FF4h]
  long long v42; // [rsp+114h] [rbp-FECh]
  long long v43; // [rsp+11Ch] [rbp-FE4h]
  long long v44; // [rsp+124h] [rbp-FDCh]
  long long v45; // [rsp+12Ch] [rbp-FD4h]
  long long v46; // [rsp+134h] [rbp-FCCh]
  long long v47; // [rsp+13Ch] [rbp-FC4h]
  int v48; // [rsp+144h] [rbp-FBCh]
  char v49[4008]; // [rsp+150h] [rbp-FB0h] BYREF
  unsigned long long v50; // [rsp+10F8h] [rbp-8h]

  src = a1;
  v14 = a2;
  s = a3;
  v12 = a4;
  v50 = __readfsqword(0x28u);
  v17 = 0;
  dest = v49;
  size = 4000;
  ptr = 0;
  v39[0] = qword_23740;
  v39[1] = qword_23748;
  v40 = xmmword_23750;
  v41 = qword_23760;
  v42 = qword_23768;
  v43 = qword_23770;
  v44 = qword_23778;
  v45 = qword_23780;
  v46 = qword_23788;
  v47 = qword_23790;
  v48 = dword_23798;
  v29[0] = qword_23740;
  v29[1] = qword_23748;
  v30 = xmmword_23750;
  v31 = qword_23760;
  v32 = qword_23768;
  v33 = qword_23770;
  v34 = qword_23778;
  v35 = qword_23780;
  v36 = qword_23788;
  v37 = qword_23790;
  v38 = dword_23798;
  if ( byte_23229 )
  {
    v22 = (unsigned long long)&a1[a2];
    v23 = (unsigned long long)&a3[a4];
    while ( 1 )
    {
      v24 = 3 * (v12 + a2) + 2;
      if ( size < v24 )
      {
        v4 = 3 * (v12 + a2) + 2;
        if ( v24 < (3 * size) >> 1 )
          v4 = (3 * size) >> 1;
        size = v4;
        free(ptr);
        ptr = malloc(size);
        dest = (char *)ptr;
        if ( !ptr )
        {
          dest = v49;
          size = 4000;
        }
      }
      if ( (unsigned long long)src >= v22 )
        v5 = 0;
      else
        v5 = sub_7668(dest, src, size) + 1;
      n = v5;
      v16 = v5 <= size;
      if ( (unsigned long long)s >= v23 )
      {
        v8 = 0;
      }
      else
      {
        if ( v5 > size )
          v6 = 0;
        else
          v6 = size - v5;
        if ( v16 )
          v7 = &dest[n];
        else
          v7 = 0;
        v8 = sub_7668(v7, s, v6) + 1;
      }
      v26 = v8;
      if ( !v16 || size < n + v8 )
      {
        size = n + v8;
        if ( n + v8 <= 0x5555555555555554LL )
          size = (3 * size) >> 1;
        free(ptr);
        ptr = (void *)sub_18024(size);
        dest = (char *)ptr;
        if ( (unsigned long long)src < v22 )
          strxfrm((char *)ptr, src, n);
        if ( (unsigned long long)s < v23 )
          strxfrm((char *)ptr + n, s, v26);
      }
      if ( (unsigned long long)src < v22 )
        src += strlen(src) + 1;
      if ( (unsigned long long)s < v23 )
        s += strlen(s) + 1;
      if ( (unsigned long long)src >= v22 && (unsigned long long)s >= v23 )
        break;
      sub_FB4B(dest, n, v29);
      sub_FB4B(&dest[n], v26, v39);
      if ( !v17 )
      {
        v9 = v26;
        if ( v26 > n )
          v9 = n;
        v17 = memcmp(dest, &dest[n], v9);
        if ( !v17 )
          v17 = (n > v26) - (n < v26);
      }
    }
    v14 = n;
    src = dest;
    v12 = v26;
    s = &dest[n];
    sub_FB4B(dest, n, v29);
  }
  else
  {
    sub_FB4B(a1, a2, v29);
  }
  sub_FBAF(v29, s1);
  sub_FB4B(s, v12, v39);
  sub_FBAF(v39, v28);
  v18 = memcmp(s1, v28, 0x10u);
  if ( !v18 )
  {
    if ( !v17 )
    {
      v10 = v12;
      if ( v12 > v14 )
        v10 = v14;
      v17 = memcmp(src, s, v10);
      if ( !v17 )
        v17 = (v14 > v12) - (v14 < v12);
    }
    v18 = v17;
  }
  free(ptr);
  return v18;
}



// Function: debug_width @ 0x7ee4
long long debug_width(char *a1, unsigned long long a2)
{
  char *v2; // rax
  char *v4; // [rsp+8h] [rbp-18h]
  long long v5; // [rsp+18h] [rbp-8h]

  v4 = a1;
  v5 = (int)sub_1293D(a1, a2 - (long long)a1, 0);
  while ( (unsigned long long)v4 < a2 )
  {
    v2 = v4++;
    v5 += *v2 == 9;
  }
  return v5;
}



// Function: mark_key @ 0x7f4c
int mark_key(long long a1, long long a2)
{
  char *v3; // rax
  long long v5; // [rsp+0h] [rbp-10h]

  v5 = a2;
  while ( a1-- )
    putchar_unlocked(32);
  if ( a2 )
  {
    do
    {
      putchar_unlocked(95);
      --v5;
    }
    while ( v5 );
    return putchar_unlocked(10);
  }
  else
  {
    v3 = gettext("^ no match for key\n");
    return printf(v3);
  }
}



// Function: key_numeric @ 0x7fc5
long long key_numeric(char *a1)
{
  return a1[50] || a1[52] || a1[53];
}



// Function: debug_key @ 0x8002
unsigned long long debug_key(long long a1, long long a2)
{
  int v2; // eax
  int v3; // eax
  char v5; // [rsp+15h] [rbp-5Bh]
  char *endptr; // [rsp+18h] [rbp-58h] BYREF
  char *v7; // [rsp+20h] [rbp-50h] BYREF
  char *nptr; // [rsp+28h] [rbp-48h]
  char *v9; // [rsp+30h] [rbp-40h]
  char *v10; // [rsp+38h] [rbp-38h]
  long long v11; // [rsp+40h] [rbp-30h]
  long long v12; // [rsp+48h] [rbp-28h]
  long double v13; // [rsp+50h] [rbp-20h]
  unsigned long long v14; // [rsp+68h] [rbp-8h]

  v14 = __readfsqword(0x28u);
  v10 = *(char **)a1;
  nptr = v10;
  v9 = &v10[*(long long *)(a1 + 8) - 1];
  if ( a2 )
  {
    if ( *(long long *)a2 != -1 )
      nptr = sub_66F4((char **)a1, (long long *)a2);
    if ( *(long long *)(a2 + 16) != -1 )
      v9 = sub_6898((char **)a1, a2);
    if ( *(char *)(a2 + 48) && *(long long *)a2 == -1 || *(char *)(a2 + 54) || sub_7FC5((char *)a2) )
    {
      v5 = *v9;
      *v9 = 0;
      while ( byte_23260[(unsigned char)sub_4042((unsigned int)*nptr)] )
        ++nptr;
      endptr = nptr;
      if ( v9 >= nptr )
      {
        if ( *(char *)(a2 + 54) )
        {
          sub_73F3(nptr, &endptr);
        }
        else if ( *(char *)(a2 + 52) )
        {
          v13 = strtold(nptr, &endptr);
        }
        else if ( *(char *)(a2 + 50) || *(char *)(a2 + 53) )
        {
          v2 = nptr < v9 && *nptr == 45;
          v7 = &nptr[v2];
          if ( (char)sub_6F64(&v7) > 47 )
          {
            v3 = *(char *)(a2 + 53) && byte_1C5C0[(unsigned char)*v7];
            endptr = &v7[v3];
          }
        }
        else
        {
          endptr = v9;
        }
      }
      else
      {
        endptr = v9;
      }
      *v9 = v5;
      v9 = endptr;
    }
  }
  v11 = sub_7EE4(v10, (unsigned long long)nptr);
  v12 = sub_7EE4(nptr, (unsigned long long)v9);
  sub_7F4C(v11, v12);
  return v14 - __readfsqword(0x28u);
}



// Function: debug_line @ 0x829e
unsigned long long debug_line(long long a1)
{
  unsigned long long result; // rax
  long long v2; // [rsp+18h] [rbp-8h]

  v2 = qword_23688;
  while ( 1 )
  {
    result = sub_8002(a1, v2);
    if ( !v2 )
      break;
    v2 = *(long long *)(v2 + 64);
    if ( !v2 )
    {
      result = (unsigned char)byte_23682 ^ 1u;
      if ( byte_23682 == 1 )
        break;
      result = (unsigned char)byte_23681 ^ 1u;
      if ( byte_23681 == 1 )
        break;
    }
  }
  return result;
}



// Function: default_key_compare @ 0x8305
long long default_key_compare(long long a1)
{
  return !*(long long *)(a1 + 32)
      && !*(long long *)(a1 + 40)
      && *(char *)(a1 + 48) != 1
      && *(char *)(a1 + 49) != 1
      && !sub_7FC5((char *)a1)
      && *(char *)(a1 + 54) != 1
      && *(char *)(a1 + 56) != 1
      && *(char *)(a1 + 51) != 1;
}



// Function: key_to_opts @ 0x839e
char *key_to_opts(long long a1, char *a2)
{
  char *v2; // rax
  char *v3; // rax
  char *v4; // rax
  char *v5; // rax
  char *v6; // rax
  char *v7; // rax
  char *v8; // rax
  char *v9; // rax
  char *v10; // rax
  char *v11; // rax
  char *v13; // [rsp+0h] [rbp-10h]

  v13 = a2;
  if ( *(char *)(a1 + 48) || *(char *)(a1 + 49) )
  {
    v13 = a2 + 1;
    *a2 = 98;
  }
  if ( *(char **)(a1 + 32) == byte_23460 )
  {
    v2 = v13++;
    *v2 = 100;
  }
  if ( *(long long *)(a1 + 40) )
  {
    v3 = v13++;
    *v3 = 102;
  }
  if ( *(char *)(a1 + 52) )
  {
    v4 = v13++;
    *v4 = 103;
  }
  if ( *(char *)(a1 + 53) )
  {
    v5 = v13++;
    *v5 = 104;
  }
  if ( *(char **)(a1 + 32) == byte_23360 )
  {
    v6 = v13++;
    *v6 = 105;
  }
  if ( *(char *)(a1 + 54) )
  {
    v7 = v13++;
    *v7 = 77;
  }
  if ( *(char *)(a1 + 50) )
  {
    v8 = v13++;
    *v8 = 110;
  }
  if ( *(char *)(a1 + 51) )
  {
    v9 = v13++;
    *v9 = 82;
  }
  if ( *(char *)(a1 + 55) )
  {
    v10 = v13++;
    *v10 = 114;
  }
  if ( *(char *)(a1 + 56) )
  {
    v11 = v13++;
    *v11 = 86;
  }
  *v13 = 0;
  return v13;
}



// Function: key_warnings @ 0x84fe
unsigned long long key_warnings(long long *a1, char a2)
{
  long long v2; // rbx
  long long v3; // rbx
  long long v4; // rbx
  long long v5; // rbx
  const char *v6; // rbx
  char *v7; // rax
  const char *v8; // rbx
  char *v9; // rax
  const char *v10; // rbx
  char *v11; // rax
  const char *v12; // rbx
  char *v13; // rax
  long long v14; // r12
  long long v15; // rbx
  char *v16; // rax
  bool v17; // al
  char *v18; // rax
  bool v19; // al
  bool v20; // al
  char *v21; // rax
  char *v22; // rax
  const unsigned short *v23; // rbx
  bool v24; // al
  long long v25; // rbx
  char *v26; // rax
  const unsigned short *v27; // rbx
  bool v28; // al
  long long v29; // rbx
  char *v30; // rax
  long long v31; // rbx
  char *v32; // rax
  long long v33; // rbx
  char *v34; // rax
  long long v35; // r12
  char *v36; // rbx
  char *v37; // rax
  char *v38; // rax
  size_t v39; // rax
  unsigned long long v40; // rax
  char *v41; // rax
  char *v42; // rax
  char v44; // [rsp+17h] [rbp-139h]
  char v45; // [rsp+18h] [rbp-138h]
  char v46; // [rsp+19h] [rbp-137h]
  char v47; // [rsp+1Ah] [rbp-136h]
  char v48; // [rsp+1Bh] [rbp-135h]
  char v49; // [rsp+1Ch] [rbp-134h]
  bool v50; // [rsp+1Dh] [rbp-133h]
  bool v51; // [rsp+1Eh] [rbp-132h]
  long long v52; // [rsp+20h] [rbp-130h]
  long long v53; // [rsp+28h] [rbp-128h]
  long long v54; // [rsp+30h] [rbp-120h]
  unsigned long long v55; // [rsp+38h] [rbp-118h]
  long long v56; // [rsp+40h] [rbp-110h]
  char *dest; // [rsp+48h] [rbp-108h]
  char *v58; // [rsp+50h] [rbp-100h]
  long long v59[4]; // [rsp+60h] [rbp-F0h] BYREF
  long long v60; // [rsp+80h] [rbp-D0h]
  long long v61; // [rsp+88h] [rbp-C8h]
  long long v62; // [rsp+90h] [rbp-C0h]
  long long v63; // [rsp+98h] [rbp-B8h]
  long long v64; // [rsp+A0h] [rbp-B0h]
  char v65[32]; // [rsp+B0h] [rbp-A0h] BYREF
  char v66[48]; // [rsp+D0h] [rbp-80h] BYREF
  char s; // [rsp+100h] [rbp-50h] BYREF
  char v68; // [rsp+101h] [rbp-4Fh]
  unsigned long long v69; // [rsp+138h] [rbp-18h]

  v69 = __readfsqword(0x28u);
  v2 = a1[1];
  v59[0] = *a1;
  v59[1] = v2;
  v3 = a1[3];
  v59[2] = a1[2];
  v59[3] = v3;
  v4 = a1[5];
  v60 = a1[4];
  v61 = v4;
  v5 = a1[7];
  v62 = a1[6];
  v63 = v5;
  v64 = a1[8];
  v53 = 1;
  v44 = 0;
  v45 = 0;
  v46 = 0;
  v47 = 0;
  v52 = qword_23688;
  while ( v52 )
  {
    if ( sub_7FC5((char *)v52) )
    {
      if ( *(char *)(v52 + 52) )
        v45 = 1;
      else
        v44 = 1;
    }
    if ( *(char *)(v52 + 57) )
    {
      v54 = *(long long *)v52;
      v56 = *(long long *)(v52 + 16);
      if ( *(long long *)v52 == -1 )
        v54 = 0;
      v6 = (const char *)sub_1287E(v54, v65);
      v7 = stpcpy(v66, "+");
      dest = stpcpy(v7, v6);
      v8 = (const char *)sub_1287E(v54 + 1, v65);
      v9 = stpcpy(&s, "-k ");
      v58 = stpcpy(v9, v8);
      if ( *(long long *)(v52 + 16) != -1 )
      {
        v10 = (const char *)sub_1287E(v56 + 1, v65);
        v11 = stpcpy(dest, " -");
        stpcpy(v11, v10);
        v12 = (const char *)sub_1287E((*(long long *)(v52 + 24) == -1) + v56 + 1, v65);
        v13 = stpcpy(v58, ",");
        stpcpy(v13, v12);
      }
      v14 = sub_15473(1, &s);
      v15 = sub_15473(0, v66);
      v16 = gettext("obsolescent key %s used; consider %s instead");
      error(0, 0, v16, v15, v14);
    }
    v17 = *(long long *)v52 != -1 && *(long long *)(v52 + 16) < *(long long *)v52;
    v50 = v17;
    if ( v17 )
    {
      v18 = gettext("key %lu has zero width and will be ignored");
      error(0, 0, v18, v53);
    }
    v19 = sub_7FC5((char *)v52) || *(char *)(v52 + 54);
    v51 = v19;
    v20 = !*(long long *)(v52 + 16) && *(long long *)(v52 + 24);
    if ( !v50
      && a2 != 1
      && dword_23020 == 128
      && !v20
      && (*(char *)(v52 + 48) != 1 && !v51
       || *(char *)(v52 + 48) != 1 && *(long long *)(v52 + 8)
       || *(char *)(v52 + 49) != 1 && *(long long *)(v52 + 24)) )
    {
      v21 = gettext("leading blanks are significant in key %lu; consider also specifying 'b'");
      error(0, 0, v21, v53);
    }
    if ( a2 != 1 && sub_7FC5((char *)v52) )
    {
      v55 = *(long long *)v52 + 1LL;
      if ( *(long long *)v52 == -1 )
        v55 = 1;
      if ( *(long long *)(v52 + 16) == -1 || v55 < *(long long *)(v52 + 16) + 1LL )
      {
        v22 = gettext("key %lu is numeric and spans multiple fields");
        error(0, 0, v22, v53);
        if ( *(char *)(v52 + 52) )
          v47 = 1;
        else
          v46 = 1;
      }
    }
    if ( v60 && v60 == *(long long *)(v52 + 32) )
      v60 = 0;
    if ( v61 && v61 == *(long long *)(v52 + 40) )
      v61 = 0;
    LOBYTE(v62) = (unsigned char)(v62 & (*(char *)(v52 + 48) ^ 1)) != 0;
    BYTE1(v62) = (BYTE1(v62) & (*(char *)(v52 + 49) ^ 1)) != 0;
    BYTE6(v62) = (BYTE6(v62) & (*(char *)(v52 + 54) ^ 1)) != 0;
    BYTE2(v62) = (BYTE2(v62) & (*(char *)(v52 + 50) ^ 1)) != 0;
    BYTE4(v62) = (BYTE4(v62) & (*(char *)(v52 + 52) ^ 1)) != 0;
    BYTE5(v62) = (BYTE5(v62) & (*(char *)(v52 + 53) ^ 1)) != 0;
    BYTE3(v62) = (BYTE3(v62) & (*(char *)(v52 + 51) ^ 1)) != 0;
    LOBYTE(v63) = (unsigned char)(v63 & (*(char *)(v52 + 56) ^ 1)) != 0;
    HIBYTE(v62) = (HIBYTE(v62) & (*(char *)(v52 + 55) ^ 1)) != 0;
    v52 = *(long long *)(v52 + 64);
    ++v53;
  }
  v48 = 0;
  if ( v46 )
  {
    if ( dword_23020 == 128 )
    {
      v24 = 0;
      if ( dword_23224 != 128 )
      {
        v23 = *__ctype_b_loc();
        if ( (v23[(unsigned char)sub_4042((unsigned int)(char)dword_23224)] & 1) != 0 )
          v24 = 1;
      }
    }
    else
    {
      v24 = dword_23020 == dword_23224;
    }
    if ( v24 )
    {
      s = dword_23224;
      v68 = 0;
      v25 = sub_154A0(&s);
      v26 = gettext("field separator %s is treated as a group separator in numbers");
      error(0, 0, v26, v25);
      v48 = 1;
    }
  }
  if ( v46 || v47 )
  {
    if ( dword_23020 == 128 )
    {
      v28 = 0;
      if ( dword_23224 != 128 )
      {
        v27 = *__ctype_b_loc();
        if ( (v27[(unsigned char)sub_4042((unsigned int)byte_23220)] & 1) != 0 )
          v28 = 1;
      }
    }
    else
    {
      v28 = byte_23220 == dword_23020;
    }
    if ( v28 )
    {
      s = byte_23220;
      v68 = 0;
      v29 = sub_154A0(&s);
      v30 = gettext("field separator %s is treated as a decimal point in numbers");
      error(0, 0, v30, v29);
      v48 = 1;
    }
    else if ( dword_23020 == 45 )
    {
      s = 45;
      v68 = 0;
      v31 = sub_154A0(&s);
      v32 = gettext("field separator %s is treated as a minus sign in numbers");
      error(0, 0, v32, v31);
    }
    else if ( v47 && dword_23020 == 43 )
    {
      s = 43;
      v68 = 0;
      v33 = sub_154A0(&s);
      v34 = gettext("field separator %s is treated as a plus sign in numbers");
      error(0, 0, v34, v33);
    }
  }
  if ( (v44 || v45) && v48 != 1 )
  {
    s = byte_23220;
    v68 = 0;
    v35 = sub_154A0(&s);
    if ( byte_23220 == dword_23020 )
      v36 = (char *)locale;
    else
      v36 = gettext("note ");
    v37 = gettext("%snumbers use %s as a decimal point in this locale");
    error(0, 0, v37, v36, v35);
  }
  if ( v44 && byte_23228 )
  {
    v38 = gettext("the multi-byte number group separator in this locale is not supported");
    error(0, 0, v38);
  }
  if ( !sub_8305((long long)v59) || HIBYTE(v62) && (byte_23681 || byte_23682) && qword_23688 )
  {
    v49 = HIBYTE(v62);
    if ( byte_23681 != 1 && byte_23682 != 1 )
      HIBYTE(v62) = 0;
    sub_839E((long long)v59, &s);
    v39 = strlen(&s);
    v40 = sub_4094(v39);
    v41 = ngettext("option '-%s' is ignored", "options '-%s' are ignored", v40);
    error(0, 0, v41, &s);
    HIBYTE(v62) = v49;
  }
  if ( HIBYTE(v62) && byte_23681 != 1 && byte_23682 != 1 && qword_23688 )
  {
    v42 = gettext("option '-r' only applies to last-resort comparison");
    error(0, 0, v42);
  }
  return v69 - __readfsqword(0x28u);
}



// Function: keycompare @ 0x9116
long long keycompare(long long a1, long long a2)
{
  char *v2; // rax
  char *v3; // rax
  char v4; // al
  size_t v5; // rdx
  char v6; // al
  size_t v7; // rdx
  int v8; // ebx
  unsigned char v9; // al
  int v10; // ebx
  unsigned char v11; // al
  int v12; // ebx
  char *v13; // rax
  unsigned char v14; // al
  int v15; // ebx
  char *v16; // rax
  unsigned char v17; // al
  size_t v18; // rdx
  int v19; // eax
  char v21; // [rsp+1Ah] [rbp-1046h]
  char v22; // [rsp+1Bh] [rbp-1045h]
  unsigned int v23; // [rsp+1Ch] [rbp-1044h]
  long long v24; // [rsp+20h] [rbp-1040h]
  char *s1; // [rsp+28h] [rbp-1038h]
  char *s2; // [rsp+30h] [rbp-1030h]
  char *v27; // [rsp+38h] [rbp-1028h]
  char *v28; // [rsp+38h] [rbp-1028h]
  char *v29; // [rsp+40h] [rbp-1020h]
  unsigned long long v30; // [rsp+40h] [rbp-1020h]
  char *v31; // [rsp+48h] [rbp-1018h]
  char *v32; // [rsp+50h] [rbp-1010h]
  size_t v33; // [rsp+58h] [rbp-1008h]
  size_t v34; // [rsp+60h] [rbp-1000h]
  char *ptr; // [rsp+68h] [rbp-FF8h]
  size_t v36; // [rsp+70h] [rbp-FF0h]
  size_t v37; // [rsp+70h] [rbp-FF0h]
  long long v38; // [rsp+78h] [rbp-FE8h]
  long long v39; // [rsp+80h] [rbp-FE0h]
  size_t n; // [rsp+88h] [rbp-FD8h]
  size_t v41; // [rsp+90h] [rbp-FD0h]
  unsigned long long v42; // [rsp+98h] [rbp-FC8h]
  char v43; // [rsp+A0h] [rbp-FC0h] BYREF
  unsigned long long v44; // [rsp+1048h] [rbp-18h]

  v44 = __readfsqword(0x28u);
  v24 = qword_23688;
  s1 = *(char **)(a1 + 16);
  s2 = *(char **)(a2 + 16);
  v27 = *(char **)(a1 + 24);
  v29 = *(char **)(a2 + 24);
  while ( 1 )
  {
    v38 = *(long long *)(v24 + 40);
    v39 = *(long long *)(v24 + 32);
    v2 = s1;
    if ( v27 >= s1 )
      v2 = v27;
    v28 = v2;
    v3 = s2;
    if ( v29 >= s2 )
      v3 = v29;
    v30 = (unsigned long long)v3;
    n = v28 - s1;
    v41 = v3 - s2;
    if ( byte_23229 || sub_7FC5((char *)v24) || *(char *)(v24 + 54) || *(char *)(v24 + 51) || *(char *)(v24 + 56) )
    {
      if ( v39 || v38 )
      {
        v42 = n + v41 + 2;
        if ( v42 > 0xFA0 )
        {
          ptr = (char *)sub_18024(v42);
          v31 = ptr;
        }
        else
        {
          v31 = &v43;
          ptr = 0;
        }
        v32 = &v31[n + 1];
        v36 = 0;
        v33 = 0;
        while ( v36 < n )
        {
          if ( !v39 || *(char *)((unsigned char)sub_4042((unsigned int)s1[v36]) + v39) != 1 )
          {
            if ( v38 )
              v4 = *(char *)((unsigned char)sub_4042((unsigned int)s1[v36]) + v38);
            else
              v4 = s1[v36];
            v5 = v33++;
            v31[v5] = v4;
          }
          ++v36;
        }
        v31[v33] = 0;
        v37 = 0;
        v34 = 0;
        while ( v37 < v41 )
        {
          if ( !v39 || *(char *)((unsigned char)sub_4042((unsigned int)s2[v37]) + v39) != 1 )
          {
            if ( v38 )
              v6 = *(char *)((unsigned char)sub_4042((unsigned int)s2[v37]) + v38);
            else
              v6 = s2[v37];
            v7 = v34++;
            v32[v7] = v6;
          }
          ++v37;
        }
        v32[v34] = 0;
      }
      else
      {
        v31 = s1;
        v33 = v28 - s1;
        v21 = *v28;
        *v28 = 0;
        v32 = s2;
        v34 = v41;
        v22 = s2[v41];
        s2[v41] = 0;
      }
      if ( *(char *)(v24 + 50) )
      {
        v23 = sub_71A3(v31, v32);
      }
      else if ( *(char *)(v24 + 52) )
      {
        v23 = sub_72D6(v31, v32);
      }
      else if ( *(char *)(v24 + 53) )
      {
        v23 = sub_70E2(v31, v32);
      }
      else if ( *(char *)(v24 + 54) )
      {
        v8 = sub_73F3(v31, 0);
        v23 = v8 - sub_73F3(v32, 0);
      }
      else if ( *(char *)(v24 + 51) )
      {
        v23 = sub_7755(v31, v33, v32, v34);
      }
      else if ( *(char *)(v24 + 56) )
      {
        v23 = sub_10461(v31, v33, v32, v34);
      }
      else if ( v33 )
      {
        if ( v34 )
          v23 = sub_187EC(v31, v33 + 1, v32, v34 + 1);
        else
          v23 = 1;
      }
      else
      {
        v23 = -(v34 != 0);
      }
      if ( v39 || v38 )
      {
        free(ptr);
      }
      else
      {
        v31[v33] = v21;
        v32[v34] = v22;
      }
      goto LABEL_100;
    }
    if ( v39 )
    {
      if ( v38 )
      {
        while ( 1 )
        {
          while ( s1 < v28 && *(char *)((unsigned char)sub_4042((unsigned int)*s1) + v39) )
            ++s1;
          while ( (unsigned long long)s2 < v30 && *(char *)((unsigned char)sub_4042((unsigned int)*s2) + v39) )
            ++s2;
          if ( s1 >= v28 || (unsigned long long)s2 >= v30 )
            break;
          v9 = sub_4042((unsigned int)*s1);
          v10 = (unsigned char)sub_4042((unsigned int)*(char *)(v9 + v38));
          v11 = sub_4042((unsigned int)*s2);
          v23 = v10 - (unsigned char)sub_4042((unsigned int)*(char *)(v11 + v38));
          if ( v23 )
            goto LABEL_119;
          ++s1;
          ++s2;
        }
        v23 = (s1 < v28) - ((unsigned long long)s2 < v30);
      }
      else
      {
        while ( 1 )
        {
          while ( s1 < v28 && *(char *)((unsigned char)sub_4042((unsigned int)*s1) + v39) )
            ++s1;
          while ( (unsigned long long)s2 < v30 && *(char *)((unsigned char)sub_4042((unsigned int)*s2) + v39) )
            ++s2;
          if ( s1 >= v28 || (unsigned long long)s2 >= v30 )
            break;
          v12 = (unsigned char)sub_4042((unsigned int)*s1);
          v23 = v12 - (unsigned char)sub_4042((unsigned int)*s2);
          if ( v23 )
            goto LABEL_119;
          ++s1;
          ++s2;
        }
        v23 = (s1 < v28) - ((unsigned long long)s2 < v30);
      }
      goto LABEL_100;
    }
    if ( !n )
    {
      v23 = -(v41 != 0);
      goto LABEL_100;
    }
    if ( !v41 )
      break;
    if ( v38 )
    {
      while ( s1 < v28 && (unsigned long long)s2 < v30 )
      {
        v13 = s1++;
        v14 = sub_4042((unsigned int)*v13);
        v15 = (unsigned char)sub_4042((unsigned int)*(char *)(v14 + v38));
        v16 = s2++;
        v17 = sub_4042((unsigned int)*v16);
        v23 = v15 - (unsigned char)sub_4042((unsigned int)*(char *)(v17 + v38));
        if ( v23 )
          goto LABEL_119;
      }
    }
    else
    {
      v18 = v41;
      if ( v41 > n )
        v18 = v28 - s1;
      v23 = memcmp(s1, s2, v18);
      if ( v23 )
        goto LABEL_119;
    }
    if ( n < v41 )
      v19 = -1;
    else
      v19 = n != v41;
    v23 = v19;
LABEL_100:
    if ( v23 )
      goto LABEL_119;
    v24 = *(long long *)(v24 + 64);
    if ( !v24 )
      return 0;
    if ( *(long long *)(v24 + 16) == -1 )
    {
      v27 = (char *)(*(long long *)a1 + *(long long *)(a1 + 8) - 1LL);
      v29 = (char *)(*(long long *)a2 + *(long long *)(a2 + 8) - 1LL);
    }
    else
    {
      v27 = sub_6898((char **)a1, v24);
      v29 = sub_6898((char **)a2, v24);
    }
    if ( *(long long *)v24 == -1 )
    {
      s1 = *(char **)a1;
      s2 = *(char **)a2;
      if ( *(char *)(v24 + 48) )
      {
        while ( s1 < v27 && byte_23260[(unsigned char)sub_4042((unsigned int)*s1)] )
          ++s1;
        while ( s2 < v29 && byte_23260[(unsigned char)sub_4042((unsigned int)*s2)] )
          ++s2;
      }
    }
    else
    {
      s1 = sub_66F4((char **)a1, (long long *)v24);
      s2 = sub_66F4((char **)a2, (long long *)v24);
    }
  }
  v23 = 1;
LABEL_119:
  if ( *(char *)(v24 + 55) )
    return -v23;
  else
    return v23;
}



// Function: compare @ 0x9e2a
long long compare(long long *a1, long long *a2)
{
  size_t v3; // rdx
  int v4; // eax
  unsigned int v5; // [rsp+1Ch] [rbp-14h]
  unsigned int v6; // [rsp+1Ch] [rbp-14h]
  size_t n; // [rsp+20h] [rbp-10h]
  unsigned long long v8; // [rsp+28h] [rbp-8h]

  if ( qword_23688 )
  {
    v5 = sub_9116((long long)a1, (long long)a2);
    if ( v5 || byte_23682 || byte_23681 )
      return v5;
  }
  n = a1[1] - 1LL;
  v8 = a2[1] - 1LL;
  if ( a1[1] == 1 )
  {
    v6 = -(a2[1] != 1);
  }
  else if ( a2[1] == 1 )
  {
    v6 = 1;
  }
  else if ( byte_23229 )
  {
    v6 = sub_187EC(*a1, a1[1], *a2, a2[1]);
  }
  else
  {
    v3 = a2[1] - 1LL;
    if ( v8 > n )
      v3 = a1[1] - 1LL;
    v6 = memcmp((const void *)*a1, (const void *)*a2, v3);
    if ( !v6 )
    {
      if ( n < v8 )
        v4 = -1;
      else
        v4 = n != v8;
      v6 = v4;
    }
  }
  if ( byte_23680 )
    return -v6;
  else
    return v6;
}



// Function: write_line @ 0x9f73
unsigned long long write_line(void **a1, FILE *a2, char *a3)
{
  char *v3; // rax
  char *v4; // rax
  char *v6; // rax
  char v8; // [rsp+2Fh] [rbp-21h]
  char *v9; // [rsp+30h] [rbp-20h]
  void *ptr; // [rsp+38h] [rbp-18h]
  size_t n; // [rsp+40h] [rbp-10h]
  size_t v12; // [rsp+48h] [rbp-8h]

  ptr = *a1;
  n = (size_t)a1[1];
  v12 = (size_t)*a1 + n;
  if ( a3 || !byte_23698 )
  {
    *(char *)(v12 - 1) = byte_23010;
    if ( n != fwrite_unlocked(ptr, 1u, n, a2) )
    {
      v6 = gettext("write failed");
      sub_4464(v6, a3);
    }
    *(char *)(v12 - 1) = 0;
    return v12 - 1;
  }
  else
  {
    v9 = (char *)*a1;
    while ( (unsigned long long)v9 < v12 )
    {
      v3 = v9++;
      v8 = *v3;
      if ( *v3 == 9 )
      {
        v8 = 62;
      }
      else if ( v9 == (char *)v12 )
      {
        v8 = 10;
      }
      if ( fputc_unlocked(v8, a2) == -1 )
      {
        v4 = gettext("write failed");
        sub_4464(v4, 0);
      }
    }
    return sub_829E((long long)a1);
  }
}



// Function: check @ 0xa0b1
long long check(char *a1, char a2)
{
  unsigned long long v2; // rdx
  long long v3; // r12
  long long v4; // rbx
  char *v5; // rax
  char *v6; // rax
  unsigned char v8; // [rsp+1Eh] [rbp-D2h]
  char v9[9]; // [rsp+1Fh] [rbp-D1h]
  long long v10; // [rsp+28h] [rbp-C8h]
  void **v11; // [rsp+30h] [rbp-C0h]
  FILE *v12; // [rsp+38h] [rbp-B8h]
  long long v13; // [rsp+40h] [rbp-B0h]
  void **v14; // [rsp+48h] [rbp-A8h]
  long long v15; // [rsp+58h] [rbp-98h]
  void *ptr[4]; // [rsp+60h] [rbp-90h] BYREF
  void *v17[2]; // [rsp+80h] [rbp-70h] BYREF
  long long v18; // [rsp+90h] [rbp-60h]
  char v19[24]; // [rsp+C0h] [rbp-30h] BYREF
  unsigned long long v20; // [rsp+D8h] [rbp-18h]

  v20 = __readfsqword(0x28u);
  v12 = sub_505E(a1, "r");
  v9[8] = 0;
  v10 = 0;
  v13 = qword_23688;
  *(long long *)v9 = byte_23682 == 0;
  v9[0] = byte_23682 == 0;
  v8 = 1;
  v2 = qword_23660;
  if ( qword_23660 < (unsigned long long)qword_23018 )
    v2 = qword_23018;
  sub_6618((long long)v17, 32, v2);
  ptr[0] = 0;
  while ( (unsigned char)sub_6A5E(v17, v12, a1) )
  {
    v11 = (void **)sub_66D0(v17);
    v14 = &v11[-4 * v18];
    if ( *(long long *)&v9[1] && v9[0] <= (int)sub_9E2A(ptr, v11 - 4) )
    {
LABEL_6:
      if ( a2 == 99 )
      {
        v15 = ((sub_66D0(v17) - (long long)(v11 - 4)) >> 5) + v10;
        v3 = sub_1287E(v15, v19);
        v4 = qword_23868;
        v5 = gettext("%s: %s:%s: disorder: ");
        fprintf(stderr, v5, v4, a1, v3);
        v6 = gettext("standard error");
        sub_9F73(v11 - 4, stderr, v6);
      }
      v8 = 0;
      break;
    }
    while ( 1 )
    {
      v11 -= 4;
      if ( v11 <= v14 )
        break;
      if ( v9[0] <= (int)sub_9E2A(v11, v11 - 4) )
        goto LABEL_6;
    }
    v10 += v18;
    if ( *(long long *)&v9[1] < (unsigned long long)v11[1] )
    {
      while ( 1 )
      {
        *(long long *)&v9[1] *= 2LL;
        if ( !*(long long *)&v9[1] )
          break;
        if ( *(long long *)&v9[1] >= (unsigned long long)v11[1] )
          goto LABEL_15;
      }
      *(long long *)&v9[1] = v11[1];
LABEL_15:
      free(ptr[0]);
      ptr[0] = (void *)sub_18024(*(long long *)&v9[1]);
    }
    memcpy(ptr[0], *v11, (size_t)v11[1]);
    ptr[1] = v11[1];
    if ( v13 )
    {
      ptr[2] = (char *)ptr[0] + (char *)v11[2] - (char *)*v11;
      ptr[3] = (char *)ptr[0] + (char *)v11[3] - (char *)*v11;
    }
  }
  sub_50B7(v12, a1);
  free(v17[0]);
  free(ptr[0]);
  return v8;
}



// Function: open_input_files @ 0xa4a1
long long open_input_files(long long a1, unsigned long long a2, long long *a3)
{
  FILE *v3; // rax
  int i; // [rsp+24h] [rbp-Ch]
  long long v7; // [rsp+28h] [rbp-8h]

  *a3 = sub_1819D(a2, 8);
  v7 = *a3;
  for ( i = 0; a2 > i; ++i )
  {
    v3 = *(long long *)(16LL * i + a1 + 8) && *(char *)(*(long long *)(16LL * i + a1 + 8) + 12LL)
       ? (FILE *)sub_551C(*(long long *)(16LL * i + a1 + 8))
       : sub_4EE4(*(const char **)(16LL * i + a1), "r");
    *(long long *)(8LL * i + v7) = v3;
    if ( !*(long long *)(8LL * i + v7) )
      break;
  }
  return i;
}



// Function: mergefps @ 0xa5c0
unsigned long long mergefps(
        long long a1,
        unsigned long long a2,
        unsigned long long a3,
        FILE *a4,
        char *a5,
        FILE **a6)
{
  long long v6; // rdx
  long long *v7; // rax
  long long *v8; // rcx
  long long v9; // rdx
  long long *v10; // rax
  long long *v11; // rcx
  long long v12; // rdx
  long long *v13; // rdx
  long long *v14; // rax
  long long v15; // rbx
  long long v16; // rbx
  long long v17; // rbx
  unsigned long long v22; // [rsp+18h] [rbp-F8h]
  int v24; // [rsp+3Ch] [rbp-D4h]
  void **p_ptr; // [rsp+40h] [rbp-D0h]
  unsigned long long v26; // [rsp+48h] [rbp-C8h]
  unsigned long long v27; // [rsp+50h] [rbp-C0h]
  unsigned long long j; // [rsp+50h] [rbp-C0h]
  unsigned long long k; // [rsp+50h] [rbp-C0h]
  unsigned long long m; // [rsp+50h] [rbp-C0h]
  unsigned long long ii; // [rsp+50h] [rbp-C0h]
  unsigned long long jj; // [rsp+50h] [rbp-C0h]
  unsigned long long i; // [rsp+58h] [rbp-B8h]
  unsigned long long kk; // [rsp+58h] [rbp-B8h]
  unsigned long long v35; // [rsp+60h] [rbp-B0h]
  unsigned long long v36; // [rsp+68h] [rbp-A8h]
  long long v37; // [rsp+70h] [rbp-A0h]
  long long *v38; // [rsp+78h] [rbp-98h]
  long long *v39; // [rsp+80h] [rbp-90h]
  long long *v40; // [rsp+88h] [rbp-88h]
  unsigned long long *v41; // [rsp+90h] [rbp-80h]
  long long v42; // [rsp+98h] [rbp-78h]
  void **v43; // [rsp+A0h] [rbp-70h]
  long long v44; // [rsp+A8h] [rbp-68h]
  unsigned long long v45; // [rsp+B0h] [rbp-60h]
  unsigned long long v46; // [rsp+B8h] [rbp-58h]
  unsigned long long v47; // [rsp+C0h] [rbp-50h]
  long long v48; // [rsp+C8h] [rbp-48h]
  void *ptr; // [rsp+D0h] [rbp-40h] BYREF
  size_t n; // [rsp+D8h] [rbp-38h]
  char *v51; // [rsp+E0h] [rbp-30h]
  char *v52; // [rsp+E8h] [rbp-28h]
  unsigned long long v53; // [rsp+F8h] [rbp-18h]

  v22 = a3;
  v53 = __readfsqword(0x28u);
  v38 = (long long *)sub_1819D(a3, 56);
  p_ptr = 0;
  v26 = 0;
  v39 = (long long *)sub_1819D(v22, 8);
  v40 = (long long *)sub_1819D(v22, 8);
  v41 = (unsigned long long *)sub_1819D(v22, 8);
  v42 = qword_23688;
  ptr = 0;
  v27 = 0;
  while ( v27 < v22 )
  {
    v6 = qword_23660 / v22;
    if ( qword_23660 / v22 < qword_23018 )
      v6 = qword_23018;
    sub_6618((long long)&v38[7 * v27], 32, v6);
    if ( (unsigned char)sub_6A5E(&v38[7 * v27], a6[v27], *(char **)(16 * v27 + a1)) )
    {
      v48 = sub_66D0(&v38[7 * v27]);
      v39[v27] = v48 - 32;
      v40[v27] = -32LL * v38[7 * v27 + 2] + v48;
      ++v27;
    }
    else
    {
      sub_50B7(a6[v27], *(char **)(16 * v27 + a1));
      if ( v27 < a2 )
      {
        --a2;
        sub_5782(*(const char **)(16 * v27 + a1));
      }
      free((void *)v38[7 * v27]);
      --v22;
      for ( i = v27; i < v22; ++i )
      {
        v7 = (long long *)(16 * (i + 1) + a1);
        v8 = (long long *)(a1 + 16 * i);
        v9 = v7[1];
        *v8 = *v7;
        v8[1] = v9;
        a6[i] = a6[i + 1];
      }
    }
  }
  for ( j = 0; j < v22; ++j )
    v41[j] = j;
  for ( k = 1; k < v22; ++k )
  {
    if ( (int)sub_9E2A((long long *)v39[v41[k - 1]], (long long *)v39[v41[k]]) > 0 )
    {
      v47 = v41[k - 1];
      v41[k - 1] = v41[k];
      v41[k] = v47;
      k = 0;
    }
  }
  while ( v22 )
  {
    v43 = (void **)v39[*v41];
    if ( byte_23682 )
    {
      if ( p_ptr && (unsigned int)sub_9E2A(p_ptr, v43) )
      {
        p_ptr = 0;
        sub_9F73(&ptr, a4, a5);
      }
      if ( !p_ptr )
      {
        p_ptr = &ptr;
        if ( v26 < (unsigned long long)v43[1] )
        {
          while ( v26 )
          {
            v26 *= 2LL;
            if ( v26 >= (unsigned long long)v43[1] )
              goto LABEL_30;
          }
          v26 = (unsigned long long)v43[1];
LABEL_30:
          free(ptr);
          ptr = (void *)sub_18024(v26);
        }
        n = (size_t)v43[1];
        memcpy(ptr, *v43, n);
        if ( v42 )
        {
          v51 = (char *)ptr + (char *)v43[2] - (char *)*v43;
          v52 = (char *)ptr + (char *)v43[3] - (char *)*v43;
        }
      }
    }
    else
    {
      sub_9F73(v43, a4, a5);
    }
    if ( (unsigned long long)v43 <= v40[*v41] )
    {
      if ( (unsigned char)sub_6A5E(&v38[7 * *v41], a6[*v41], *(char **)(16 * *v41 + a1)) )
      {
        v44 = sub_66D0(&v38[7 * *v41]);
        v39[*v41] = v44 - 32;
        v40[*v41] = -32LL * v38[7 * *v41 + 2] + v44;
        goto LABEL_52;
      }
      for ( m = 1; m < v22; ++m )
      {
        if ( v41[m] > *v41 )
          --v41[m];
      }
      --v22;
      sub_50B7(a6[*v41], *(char **)(16 * *v41 + a1));
      if ( a2 > *v41 )
      {
        --a2;
        sub_5782(*(const char **)(16 * *v41 + a1));
      }
      free((void *)v38[7 * *v41]);
      for ( ii = *v41; ii < v22; ++ii )
      {
        a6[ii] = a6[ii + 1];
        v10 = (long long *)(16 * (ii + 1) + a1);
        v11 = (long long *)(a1 + 16 * ii);
        v12 = v10[1];
        *v11 = *v10;
        v11[1] = v12;
        v13 = &v38[7 * ii + 7];
        v14 = &v38[7 * ii];
        v15 = v13[1];
        *v14 = *v13;
        v14[1] = v15;
        v16 = v13[3];
        v14[2] = v13[2];
        v14[3] = v16;
        v17 = v13[5];
        v14[4] = v13[4];
        v14[5] = v17;
        v14[6] = v13[6];
        v39[ii] = v39[ii + 1];
        v40[ii] = v40[ii + 1];
      }
      for ( jj = 0; jj < v22; ++jj )
        v41[jj] = v41[jj + 1];
    }
    else
    {
      v39[*v41] = v43 - 4;
LABEL_52:
      v35 = 1;
      v36 = v22;
      v37 = 1;
      v45 = *v41;
      while ( v35 < v36 )
      {
        v24 = sub_9E2A((long long *)v39[v45], (long long *)v39[v41[v37]]);
        if ( v24 >= 0 && (v24 || v45 >= v41[v37]) )
          v35 = v37 + 1;
        else
          v36 = v37;
        v37 = (v35 + v36) >> 1;
      }
      v46 = v35 - 1;
      for ( kk = 0; kk < v46; ++kk )
        v41[kk] = v41[kk + 1];
      v41[v46] = v45;
    }
  }
  if ( byte_23682 && p_ptr )
  {
    sub_9F73(&ptr, a4, a5);
    free(ptr);
  }
  sub_50B7(a4, a5);
  free(a6);
  free(v38);
  free(v41);
  free(v40);
  free(v39);
  return v53 - __readfsqword(0x28u);
}



// Function: mergefiles @ 0xb344
unsigned long long mergefiles(long long a1, unsigned long long a2, unsigned long long a3, FILE *a4, char *a5)
{
  char *v5; // rbx
  char *v6; // rax
  FILE **v11; // [rsp+38h] [rbp-28h] BYREF
  unsigned long long v12; // [rsp+40h] [rbp-20h]
  unsigned long long v13; // [rsp+48h] [rbp-18h]

  v13 = __readfsqword(0x28u);
  v12 = sub_A4A1(a1, a3, (long long *)&v11);
  if ( v12 < a3 && v12 <= 1 )
  {
    v5 = *(char **)(16 * v12 + a1);
    v6 = gettext("open failed");
    sub_4464(v6, v5);
  }
  sub_A5C0(a1, a2, v12, a4, a5, v11);
  return v12;
}



// Function: mergelines @ 0xb413
long long mergelines(long long *a1, unsigned long long a2, long long *a3)
{
  long long v3; // rdx
  long long result; // rax
  long long v5; // rdx
  long long v6; // rdx
  long long v7; // rdx
  long long v8; // rdx
  long long v9; // rdx
  long long *v11; // [rsp+18h] [rbp-28h]
  unsigned long long v12; // [rsp+28h] [rbp-18h]
  unsigned long long v13; // [rsp+30h] [rbp-10h]
  long long *v14; // [rsp+38h] [rbp-8h]

  v11 = a1;
  v12 = a2 >> 1;
  v13 = a2 - (a2 >> 1);
  v14 = &a1[-4 * (a2 >> 1)];
  do
  {
    while ( (int)sub_9E2A(a3 - 4, v14 - 4) <= 0 )
    {
      a3 -= 4;
      v11 -= 4;
      v3 = a3[1];
      *v11 = *a3;
      v11[1] = v3;
      result = a3[2];
      v5 = a3[3];
      v11[2] = result;
      v11[3] = v5;
      if ( !--v12 )
        return result;
    }
    v14 -= 4;
    v11 -= 4;
    v6 = v14[1];
    *v11 = *v14;
    v11[1] = v6;
    v7 = v14[3];
    v11[2] = v14[2];
    v11[3] = v7;
    --v13;
  }
  while ( v13 );
  do
  {
    a3 -= 4;
    v11 -= 4;
    v8 = a3[1];
    *v11 = *a3;
    v11[1] = v8;
    result = a3[2];
    v9 = a3[3];
    v11[2] = result;
    v11[3] = v9;
    --v12;
  }
  while ( v12 );
  return result;
}



// Function: sequential_sort @ 0xb536
long long sequential_sort(long long *a1, unsigned long long a2, long long *a3, unsigned char a4)
{
  long long result; // rax
  long long *v5; // rsi
  long long *v6; // rcx
  long long v7; // rdx
  long long v8; // rdx
  long long *v9; // rsi
  long long *v10; // rcx
  long long v11; // rdx
  long long v12; // rdx
  long long *v13; // rsi
  long long v14; // rdx
  long long v15; // rdx
  long long *v16; // rsi
  long long v17; // rdx
  long long v18; // rdx
  long long *v19; // rsi
  long long v20; // rdx
  long long v21; // rdx
  long long v22; // rcx
  long long *v23; // rsi
  long long v24; // rdx
  long long v25; // rdx
  unsigned long long v29; // [rsp+40h] [rbp-20h]

  if ( a2 == 2 )
  {
    result = (int)sub_9E2A(a1 - 4, a1 - 8) > 0;
    if ( a4 )
    {
      v5 = &a1[4 * ~(int)result];
      v6 = a3 - 4;
      v7 = v5[1];
      *v6 = *v5;
      v6[1] = v7;
      v8 = v5[3];
      v6[2] = v5[2];
      v6[3] = v8;
      v9 = &a1[4 * (int)result - 8];
      v10 = a3 - 8;
      v11 = v9[1];
      *v10 = *v9;
      v10[1] = v11;
      result = v9[2];
      v12 = v9[3];
      v10[2] = result;
      v10[3] = v12;
    }
    else if ( (int)result )
    {
      v13 = a3 - 4;
      v14 = *(a1 - 3);
      *v13 = *(a1 - 4);
      v13[1] = v14;
      v15 = *(a1 - 1);
      v13[2] = *(a1 - 2);
      v13[3] = v15;
      v16 = a1 - 4;
      v17 = *(a1 - 7);
      *v16 = *(a1 - 8);
      v16[1] = v17;
      v18 = *(a1 - 5);
      v16[2] = *(a1 - 6);
      v16[3] = v18;
      v19 = a1 - 8;
      v20 = *(a3 - 3);
      *v19 = *(a3 - 4);
      v19[1] = v20;
      result = *(a3 - 2);
      v21 = *(a3 - 1);
      v19[2] = result;
      v19[3] = v21;
    }
  }
  else
  {
    v29 = a2 >> 1;
    if ( a4 )
      v22 = 0x1FFFFFFFFFFFFFFCLL * v29;
    else
      v22 = 0;
    sequential_sort(&a1[-4 * (a2 >> 1)], a2 - (a2 >> 1), &a3[v22], a4);
    if ( v29 <= 1 )
    {
      if ( a4 != 1 )
      {
        v23 = a3 - 4;
        v24 = *(a1 - 3);
        *v23 = *(a1 - 4);
        v23[1] = v24;
        v25 = *(a1 - 1);
        v23[2] = *(a1 - 2);
        v23[3] = v25;
      }
    }
    else
    {
      sequential_sort(a1, v29, a3, a4 == 0);
    }
    if ( a4 )
      return sub_B413(a3, a2, a1);
    else
      return sub_B413(a1, a2, a3);
  }
  return result;
}



// Function: merge_tree_init @ 0xb7b6
long long merge_tree_init(long long a1, long long a2, long long a3)
{
  long long v5; // [rsp+28h] [rbp-8h]

  v5 = sub_18024(a1 << 8);
  *(long long *)(v5 + 24) = 0;
  *(long long *)(v5 + 16) = *(long long *)(v5 + 24);
  *(long long *)(v5 + 8) = *(long long *)(v5 + 16);
  *(long long *)v5 = *(long long *)(v5 + 8);
  *(long long *)(v5 + 32) = 0;
  *(long long *)(v5 + 48) = a2;
  *(long long *)(v5 + 40) = *(long long *)(v5 + 48);
  *(long long *)(v5 + 56) = 0;
  *(int *)(v5 + 80) = 0;
  *(char *)(v5 + 84) = 0;
  pthread_mutex_init((pthread_mutex_t *)(v5 + 88), 0);
  sub_B911(v5, v5 + 128, a3, a1, a2, 0);
  return v5;
}



// Function: merge_tree_destroy @ 0xb8b3
void merge_tree_destroy(long long a1, char *a2)
{
  long long v3; // [rsp+10h] [rbp-10h]
  char *i; // [rsp+18h] [rbp-8h]

  v3 = 2 * a1;
  for ( i = a2; v3--; i += 128 )
    pthread_mutex_destroy((pthread_mutex_t *)(i + 88));
  free(a2);
}



// Function: init_node @ 0xb911
long long init_node(long long a1, long long a2, long long a3, unsigned long long a4, long long a5, char a6)
{
  unsigned long long v6; // rax
  long long v7; // rax
  long long v8; // rax
  long long v12; // [rsp+20h] [rbp-60h]
  unsigned long long v13; // [rsp+40h] [rbp-40h]
  unsigned long long v14; // [rsp+48h] [rbp-38h]
  long long v15; // [rsp+50h] [rbp-30h]
  long long v16; // [rsp+58h] [rbp-28h]

  if ( a6 )
    v6 = *(long long *)(a1 + 40);
  else
    v6 = *(long long *)(a1 + 48);
  v13 = v6 >> 1;
  v14 = v6 - (v6 >> 1);
  v15 = -32 * a5 + a3;
  v16 = -32LL * (v6 >> 1) + v15;
  if ( a6 )
    v7 = a1 + 16;
  else
    v7 = a1 + 24;
  v12 = a2 + 128;
  *(long long *)(a2 + 16) = v15;
  *(long long *)a2 = *(long long *)(a2 + 16);
  *(long long *)(a2 + 24) = v16;
  *(long long *)(a2 + 8) = *(long long *)(a2 + 24);
  *(long long *)(a2 + 32) = v7;
  *(long long *)(a2 + 40) = v13;
  *(long long *)(a2 + 48) = v14;
  *(long long *)(a2 + 56) = a1;
  *(int *)(a2 + 80) = *(int *)(a1 + 80) + 1;
  *(char *)(a2 + 84) = 0;
  pthread_mutex_init((pthread_mutex_t *)(a2 + 88), 0);
  if ( a4 <= 1 )
  {
    *(long long *)(a2 + 64) = 0;
    *(long long *)(a2 + 72) = 0;
  }
  else
  {
    *(long long *)(a2 + 64) = v12;
    v8 = init_node(a2, v12, v15, a4 >> 1, a5, 1);
    *(long long *)(a2 + 72) = v8;
    return init_node(a2, v8, v16, a4 - (a4 >> 1), a5, 0);
  }
  return v12;
}



// Function: compare_nodes @ 0xbb0c
long long compare_nodes(long long a1, long long a2)
{
  if ( *(int *)(a1 + 80) == *(int *)(a2 + 80) )
    return *(long long *)(a1 + 48) + *(long long *)(a1 + 40) < *(long long *)(a2 + 40) + *(long long *)(a2 + 48);
  else
    return *(int *)(a1 + 80) < *(int *)(a2 + 80);
}



// Function: lock_node @ 0xbb87
int lock_node(long long a1)
{
  return pthread_mutex_lock((pthread_mutex_t *)(a1 + 88));
}



// Function: unlock_node @ 0xbba6
int unlock_node(long long a1)
{
  return pthread_mutex_unlock((pthread_mutex_t *)(a1 + 88));
}



// Function: queue_destroy @ 0xbbc5
int queue_destroy(long long a1)
{
  sub_12398(*(long long *)a1);
  pthread_cond_destroy((pthread_cond_t *)(a1 + 48));
  return pthread_mutex_destroy((pthread_mutex_t *)(a1 + 8));
}



// Function: queue_init @ 0xbc07
int queue_init(long long a1, long long a2)
{
  *(long long *)a1 = sub_122ED(sub_BB0C, 2 * a2);
  pthread_mutex_init((pthread_mutex_t *)(a1 + 8), 0);
  return pthread_cond_init((pthread_cond_t *)(a1 + 48), 0);
}



// Function: queue_insert @ 0xbc6b
int queue_insert(long long a1, long long a2)
{
  pthread_mutex_lock((pthread_mutex_t *)(a1 + 8));
  sub_123C6(*(long long *)a1, a2);
  *(char *)(a2 + 84) = 1;
  pthread_cond_signal((pthread_cond_t *)(a1 + 48));
  return pthread_mutex_unlock((pthread_mutex_t *)(a1 + 8));
}



// Function: queue_pop @ 0xbcd0
long long queue_pop(long long a1)
{
  long long v2; // [rsp+18h] [rbp-8h]

  pthread_mutex_lock((pthread_mutex_t *)(a1 + 8));
  while ( 1 )
  {
    v2 = sub_12474(*(long long *)a1);
    if ( v2 )
      break;
    pthread_cond_wait((pthread_cond_t *)(a1 + 48), (pthread_mutex_t *)(a1 + 8));
  }
  pthread_mutex_unlock((pthread_mutex_t *)(a1 + 8));
  sub_BB87(v2);
  *(char *)(v2 + 84) = 0;
  return v2;
}



// Function: write_unique @ 0xbd51
long long write_unique(long long a1, FILE *a2, char *a3)
{
  long long result; // rax
  long long v4; // rdx
  long long v5; // rdx

  if ( !byte_23682 )
    return sub_9F73((void **)a1, a2, a3);
  if ( !qword_23240 || (result = sub_9E2A((long long *)a1, &qword_23240), (int)result) )
  {
    v4 = *(long long *)(a1 + 8);
    qword_23240 = *(long long *)a1;
    qword_23248 = v4;
    v5 = *(long long *)(a1 + 24);
    qword_23250 = *(long long *)(a1 + 16);
    qword_23258 = v5;
    return sub_9F73((void **)a1, a2, a3);
  }
  return result;
}



// Function: mergelines_node @ 0xbde5
long long *mergelines_node(long long *a1, unsigned long long a2, FILE *a3, char *a4)
{
  long long *v4; // rsi
  long long v5; // rdx
  long long v6; // rdx
  long long v8; // rsi
  long long v9; // rdx
  long long v10; // rdx
  long long *v12; // rsi
  long long v13; // rdx
  long long v14; // rdx
  long long v22; // [rsp+20h] [rbp-30h]
  long long *i; // [rsp+28h] [rbp-28h]
  long long v24; // [rsp+30h] [rbp-20h]
  long long *v25; // [rsp+38h] [rbp-18h]
  long long v26; // [rsp+48h] [rbp-8h]

  v24 = *a1;
  v25 = (long long *)a1[1];
  v22 = (a2 >> (2 * (*((char *)a1 + 80) + 1))) + 1;
  if ( *((int *)a1 + 20) <= 1u )
  {
    while ( *a1 != a1[2] )
    {
      if ( a1[1] == a1[3] )
        break;
      if ( !v22-- )
        break;
      if ( (int)sub_9E2A((long long *)(*a1 - 32), (long long *)(a1[1] - 32)) > 0 )
      {
        a1[1] -= 32;
        sub_BD51(a1[1], a3, a4);
      }
      else
      {
        *a1 -= 32;
        sub_BD51(*a1, a3, a4);
      }
    }
    if ( ((long long)v25 - a1[1]) >> 5 == a1[6] )
    {
      while ( *a1 != a1[2] )
      {
        if ( !v22-- )
          break;
        *a1 -= 32;
        sub_BD51(*a1, a3, a4);
      }
    }
    else if ( (v24 - *a1) >> 5 == a1[5] )
    {
      while ( a1[1] != a1[3] )
      {
        if ( !v22-- )
          break;
        a1[1] -= 32;
        sub_BD51(a1[1], a3, a4);
      }
    }
  }
  else
  {
    for ( i = *(long long **)a1[4]; *a1 != a1[2]; i[3] = v6 )
    {
      if ( a1[1] == a1[3] )
        break;
      if ( !v22-- )
        break;
      if ( (int)sub_9E2A((long long *)(*a1 - 32), (long long *)(a1[1] - 32)) > 0 )
      {
        a1[1] -= 32;
        v4 = (long long *)a1[1];
      }
      else
      {
        *a1 -= 32;
        v4 = (long long *)*a1;
      }
      i -= 4;
      v5 = v4[1];
      *i = *v4;
      i[1] = v5;
      v6 = v4[3];
      i[2] = v4[2];
    }
    if ( ((long long)v25 - a1[1]) >> 5 == a1[6] )
    {
      while ( *a1 != a1[2] )
      {
        if ( !v22-- )
          break;
        *a1 -= 32;
        v8 = *a1;
        i -= 4;
        v9 = *(long long *)(*a1 + 8);
        *i = *(long long *)*a1;
        i[1] = v9;
        v10 = *(long long *)(v8 + 24);
        i[2] = *(long long *)(v8 + 16);
        i[3] = v10;
      }
    }
    else if ( (v24 - *a1) >> 5 == a1[5] )
    {
      while ( a1[1] != a1[3] )
      {
        if ( !v22-- )
          break;
        a1[1] -= 32;
        v12 = (long long *)a1[1];
        i -= 4;
        v13 = v12[1];
        *i = *v12;
        i[1] = v13;
        v14 = v12[3];
        i[2] = v12[2];
        i[3] = v14;
      }
    }
    *(long long *)a1[4] = i;
  }
  v26 = ((long long)v25 - a1[1]) >> 5;
  a1[5] -= (v24 - *a1) >> 5;
  a1[6] -= v26;
  return a1;
}



// Function: queue_check_insert @ 0xc28e
int queue_check_insert(long long a1, long long a2)
{
  int result; // eax
  bool v3; // al
  bool v4; // al

  result = *(unsigned char *)(a2 + 84) ^ 1;
  if ( *(char *)(a2 + 84) != 1 )
  {
    if ( *(long long *)a2 == *(long long *)(a2 + 16) )
    {
      v4 = *(long long *)(a2 + 8) != *(long long *)(a2 + 24) && !*(long long *)(a2 + 40);
      result = v4;
    }
    else
    {
      v3 = *(long long *)(a2 + 8) != *(long long *)(a2 + 24) || !*(long long *)(a2 + 48);
      result = v3;
    }
    if ( (char)result )
      return sub_BC6B(a1, a2);
  }
  return result;
}



// Function: queue_check_insert_parent @ 0xc34c
int queue_check_insert_parent(long long a1, long long a2)
{
  long long v2; // rax

  if ( *(int *)(a2 + 80) <= 1u )
  {
    v2 = *(long long *)(a2 + 40) + *(long long *)(a2 + 48);
    if ( !v2 )
      LODWORD(v2) = sub_BC6B(a1, *(long long *)(a2 + 56));
  }
  else
  {
    sub_BB87(*(long long *)(a2 + 56));
    sub_C28E(a1, *(long long *)(a2 + 56));
    LODWORD(v2) = sub_BBA6(*(long long *)(a2 + 56));
  }
  return v2;
}



// Function: merge_loop @ 0xc3d7
int merge_loop(long long a1, unsigned long long a2, FILE *a3, char *a4)
{
  long long v7; // [rsp+28h] [rbp-8h]

  while ( 1 )
  {
    v7 = sub_BCD0(a1);
    if ( !*(int *)(v7 + 80) )
      break;
    sub_BDE5((long long *)v7, a2, a3, a4);
    sub_C28E(a1, v7);
    sub_C34C(a1, v7);
    sub_BBA6(v7);
  }
  sub_BBA6(v7);
  return sub_BC6B(a1, v7);
}



// Function: sortlines_thread @ 0xc481
void *sortlines_thread(long long *a1)
{
  sub_C4E8(*a1, a1[1], a1[2], a1[3], a1[4], a1[5], a1[6]);
  return 0;
}



// Function: sortlines @ 0xc4e8
unsigned long long sortlines(
        long long *a1,
        unsigned long long a2,
        unsigned long long a3,
        long long *a4,
        long long a5,
        FILE *a6,
        char *a7)
{
  pthread_t newthread; // [rsp+48h] [rbp-78h] BYREF
  unsigned long long v13; // [rsp+50h] [rbp-70h]
  unsigned long long v14; // [rsp+58h] [rbp-68h]
  unsigned long long v15; // [rsp+60h] [rbp-60h]
  unsigned long long v16; // [rsp+68h] [rbp-58h]
  unsigned long long v17; // [rsp+70h] [rbp-50h]
  long long *v18; // [rsp+78h] [rbp-48h]
  long long arg[7]; // [rsp+80h] [rbp-40h] BYREF
  unsigned long long v20; // [rsp+B8h] [rbp-8h]

  v20 = __readfsqword(0x28u);
  v13 = a4[5] + a4[6];
  v14 = a2 >> 1;
  v15 = a2 - (a2 >> 1);
  arg[0] = a1;
  arg[1] = a2 >> 1;
  arg[2] = a3;
  arg[3] = a4[8];
  arg[4] = a5;
  arg[5] = a6;
  arg[6] = a7;
  if ( a2 <= 1
    || v13 <= (unsigned long long)&unk_1FFFF
    || pthread_create(&newthread, 0, (void *(*)(void *))start_routine, arg) )
  {
    v16 = a4[5];
    v17 = a4[6];
    v18 = &a1[-4 * a3];
    if ( v17 > 1 )
      sub_B536(&a1[-4 * v16], v17, &v18[-4 * (v16 >> 1)], 0);
    if ( v16 > 1 )
      sub_B536(a1, v16, v18, 0);
    *a4 = a1;
    a4[1] = &a1[-4 * v16];
    a4[2] = &a1[-4 * v16];
    a4[3] = (char *)a1 - 32 * v16 - 32 * v17;
    sub_BC6B(a5, (long long)a4);
    sub_C3D7(a5, a3, a6, a7);
  }
  else
  {
    sortlines(-32 * *((int *)a4 + 10) + (int)a1, v15, a3, a4[9], a5, (int)a6, (long long)a7);
    pthread_join(newthread, 0);
  }
  return v20 - __readfsqword(0x28u);
}



// Function: avoid_trashing_input @ 0xc7fa
unsigned long long avoid_trashing_input(long long a1, unsigned long long a2, unsigned long long a3, const char *a4)
{
  char v4; // al
  char v8; // [rsp+2Eh] [rbp-C2h]
  bool v9; // [rsp+2Fh] [rbp-C1h]
  FILE *v10; // [rsp+30h] [rbp-C0h] BYREF
  long long *v11; // [rsp+38h] [rbp-B8h]
  unsigned long long i; // [rsp+40h] [rbp-B0h]
  struct stat *v13; // [rsp+48h] [rbp-A8h]
  struct stat buf; // [rsp+50h] [rbp-A0h] BYREF
  unsigned long long v15; // [rsp+E8h] [rbp-8h]

  v15 = __readfsqword(0x28u);
  v11 = 0;
  for ( i = a2; i < a3; ++i )
  {
    v9 = strcmp(*(const char **)(16 * i + a1), "-") == 0;
    if ( a4 && !strcmp(a4, *(const char **)(16 * i + a1)) && !v9 )
    {
      v8 = 1;
      goto LABEL_16;
    }
    v13 = sub_4E8C();
    if ( !v13 )
      return v15 - __readfsqword(0x28u);
    if ( v9 )
    {
      if ( fstat(0, &buf) )
        goto LABEL_14;
    }
    else if ( stat(*(const char **)(16 * i + a1), &buf) )
    {
      goto LABEL_14;
    }
    if ( buf.st_ino != v13->st_ino || buf.st_dev != v13->st_dev )
    {
LABEL_14:
      v4 = 0;
      goto LABEL_15;
    }
    v4 = 1;
LABEL_15:
    v8 = v4;
LABEL_16:
    if ( v8 )
    {
      if ( !v11 )
      {
        v11 = sub_54F9(&v10);
        sub_B344(16 * i + a1, 0, 1u, v10, (char *)v11 + 13);
      }
      *(long long *)(16 * i + a1) = (char *)v11 + 13;
      *(long long *)(a1 + 16 * i + 8) = v11;
    }
  }
  return v15 - __readfsqword(0x28u);
}



// Function: check_inputs @ 0xca81
unsigned long long check_inputs(long long a1, unsigned long long a2)
{
  char *v2; // rbx
  char *v3; // rax
  unsigned long long result; // rax
  unsigned long long i; // [rsp+18h] [rbp-18h]

  for ( i = 0; ; ++i )
  {
    result = i;
    if ( i >= a2 )
      break;
    if ( strcmp(*(const char **)(8 * i + a1), "-") )
    {
      if ( euidaccess(*(const char **)(8 * i + a1), 4) )
      {
        v2 = *(char **)(8 * i + a1);
        v3 = gettext("cannot read");
        sub_4464(v3, v2);
      }
    }
  }
  return result;
}



// Function: check_output @ 0xcb44
void check_output(char *a1)
{
  char *v1; // rax
  int v2; // [rsp+1Ch] [rbp-4h]

  if ( a1 )
  {
    v2 = open(a1, 524353, 438);
    if ( v2 < 0 )
    {
      v1 = gettext("open failed");
      sub_4464(v1, a1);
    }
    sub_515D(v2, 1);
  }
}



// Function: merge @ 0xcbb9
unsigned long long merge(long long *a1, unsigned long long a2, unsigned long long a3, char *a4)
{
  unsigned long long v4; // rax
  unsigned long long v5; // rax
  unsigned long long v6; // rax
  unsigned long long v7; // rax
  long long v8; // rax
  char *v10; // rax
  char *v11; // rbx
  char *v12; // rax
  unsigned long long v13; // rax
  unsigned long long v14; // rax
  unsigned long long v18; // [rsp+10h] [rbp-A0h]
  FILE **v19; // [rsp+28h] [rbp-88h] BYREF
  FILE *v20; // [rsp+30h] [rbp-80h] BYREF
  long long v21; // [rsp+38h] [rbp-78h]
  unsigned long long v22; // [rsp+40h] [rbp-70h]
  unsigned long long v23; // [rsp+48h] [rbp-68h]
  FILE *v24; // [rsp+50h] [rbp-60h]
  long long *v25; // [rsp+58h] [rbp-58h]
  long long v26; // [rsp+60h] [rbp-50h]
  unsigned long long v27; // [rsp+68h] [rbp-48h]
  unsigned long long v28; // [rsp+70h] [rbp-40h]
  long long *v29; // [rsp+78h] [rbp-38h]
  unsigned long long v30; // [rsp+80h] [rbp-30h]
  long long *v31; // [rsp+88h] [rbp-28h]
  unsigned long long v32; // [rsp+90h] [rbp-20h]
  unsigned long long v33; // [rsp+98h] [rbp-18h]

  v33 = __readfsqword(0x28u);
  while ( a3 > (unsigned int)dword_23024 )
  {
    v21 = 0;
    v22 = 0;
    while ( (unsigned int)dword_23024 <= a3 - v21 )
    {
      v31 = sub_54F9(&v20);
      v4 = a2;
      if ( (unsigned int)dword_23024 <= a2 )
        v4 = (unsigned int)dword_23024;
      v32 = sub_B344((long long)&a1[2 * v21], v4, (unsigned int)dword_23024, v20, (char *)v31 + 13);
      v5 = a2;
      if ( v32 <= a2 )
        v5 = v32;
      a2 -= v5;
      a1[2 * v22] = (char *)v31 + 13;
      a1[2 * v22 + 1] = v31;
      v21 += v32;
      ++v22;
    }
    v26 = a3 - v21;
    v27 = (unsigned int)dword_23024 - v22 % (unsigned int)dword_23024;
    if ( v27 < a3 - v21 )
    {
      v28 = v26 - v27 + 1;
      v29 = sub_54F9(&v20);
      v6 = a2;
      if ( v28 <= a2 )
        v6 = v28;
      v30 = sub_B344((long long)&a1[2 * v21], v6, v28, v20, (char *)v29 + 13);
      v7 = a2;
      if ( v30 <= a2 )
        v7 = v30;
      a2 -= v7;
      a1[2 * v22] = (char *)v29 + 13;
      v8 = v22++;
      a1[2 * v8 + 1] = v29;
      v21 += v30;
    }
    memmove(&a1[2 * v22], &a1[2 * v21], 16 * (a3 - v21));
    a2 += v22;
    a3 += v22 - v21;
  }
  sub_C7FA((long long)a1, a2, a3, a4);
  while ( 1 )
  {
    v23 = sub_A4A1((long long)a1, a3, (long long *)&v19);
    if ( v23 == a3 )
      break;
    if ( v23 <= 2 )
    {
      v11 = (char *)a1[2 * v23];
      v12 = gettext("open failed");
      sub_4464(v12, v11);
    }
    do
    {
LABEL_26:
      --v23;
      sub_50B7(v19[v23], (char *)a1[2 * v23]);
      v25 = sub_5372(&v20, v23 > 2);
    }
    while ( !v25 );
    v13 = a2;
    if ( v23 <= a2 )
      v13 = v23;
    sub_A5C0((long long)a1, v13, v23, v20, (char *)v25 + 13, v19);
    v14 = a2;
    if ( v23 <= a2 )
      v14 = v23;
    v18 = a2 - v14;
    *a1 = (char *)v25 + 13;
    a1[1] = v25;
    memmove(a1 + 2, &a1[2 * v23], 16 * (a3 - v23));
    a2 = v18 + 1;
    a3 = a3 - v23 + 1;
  }
  v24 = sub_4EE4(a4, "w");
  if ( !v24 )
  {
    if ( *__errno_location() != 24 || v23 <= 2 )
    {
      v10 = gettext("open failed");
      sub_4464(v10, a4);
    }
    goto LABEL_26;
  }
  sub_A5C0((long long)a1, a2, a3, v24, a4, v19);
  return v33 - __readfsqword(0x28u);
}



// Function: sort @ 0xd0f9
unsigned long long sort(char **a1, unsigned long long a2, char *a3, unsigned long long a4)
{
  unsigned long long v4; // rax
  char v10; // [rsp+27h] [rbp-119h]
  FILE *v11; // [rsp+28h] [rbp-118h] BYREF
  FILE *v12; // [rsp+30h] [rbp-110h] BYREF
  unsigned long long v13; // [rsp+38h] [rbp-108h]
  char *v14; // [rsp+40h] [rbp-100h]
  long long v15; // [rsp+48h] [rbp-F8h]
  unsigned long long v16; // [rsp+50h] [rbp-F0h]
  long long v17; // [rsp+58h] [rbp-E8h]
  long long *v18; // [rsp+60h] [rbp-E0h]
  long long v19; // [rsp+68h] [rbp-D8h]
  char *v20; // [rsp+70h] [rbp-D0h]
  long long *v21; // [rsp+78h] [rbp-C8h]
  char *v22; // [rsp+80h] [rbp-C0h]
  void *v23; // [rsp+88h] [rbp-B8h]
  void *ptr; // [rsp+90h] [rbp-B0h] BYREF
  long long v25; // [rsp+98h] [rbp-A8h]
  unsigned long long v26; // [rsp+A0h] [rbp-A0h]
  long long v27; // [rsp+A8h] [rbp-98h]
  long long v28; // [rsp+B0h] [rbp-90h]
  char v29; // [rsp+C0h] [rbp-80h]
  char v30[104]; // [rsp+D0h] [rbp-70h] BYREF
  unsigned long long v31; // [rsp+138h] [rbp-8h]

  v31 = __readfsqword(0x28u);
  v13 = 0;
  v10 = 0;
  v27 = 0;
  while ( a2 )
  {
    v20 = *a1;
    v11 = sub_505E(v20, "r");
    if ( a4 <= 1 )
    {
      v15 = 48;
    }
    else
    {
      v16 = 1;
      v17 = 1;
      while ( v16 < a4 )
      {
        v16 *= 2LL;
        ++v17;
      }
      v15 = 32 * v17;
    }
    if ( !v27 )
    {
      v4 = sub_637C((long long)&v11, 1u, (long long)a1, a2, v15);
      sub_6618((long long)&ptr, v15, v4);
    }
    v29 = 0;
    ++a1;
    --a2;
    while ( (unsigned char)sub_6A5E(&ptr, v11, v20) )
    {
      if ( v29 && a2 && v15 + 1 < v27 - v25 - v15 * v26 )
      {
        v28 = v25;
        break;
      }
      qword_23240 = 0;
      v21 = (long long *)sub_66D0(&ptr);
      if ( !v29 || a2 || v13 || v28 )
      {
        ++v13;
        v14 = (char *)sub_54F9(&v12) + 13;
      }
      else
      {
        sub_50B7(v11, v20);
        v12 = sub_505E(a3, "w");
        v14 = a3;
        v10 = 1;
      }
      if ( v26 <= 1 )
      {
        sub_BD51((long long)(v21 - 4), v12, v14);
      }
      else
      {
        sub_BC07((long long)v30, a4);
        v22 = (char *)sub_B7B6(a4, v26, (long long)v21);
        sub_C4E8(v21, a4, v26, (long long *)v22 + 16, (long long)v30, v12, v14);
        sub_B8B3(a4, v22);
        sub_BBC5((long long)v30);
      }
      sub_50B7(v12, v14);
      if ( v10 )
        goto LABEL_28;
    }
    sub_50B7(v11, v20);
  }
LABEL_28:
  free(ptr);
  if ( v10 != 1 )
  {
    v18 = (long long *)qword_23720;
    v23 = (void *)sub_1819D(v13, 16);
    v19 = 0;
    while ( v18 )
    {
      *((long long *)v23 + 2 * v19) = (char *)v18 + 13;
      *((long long *)v23 + 2 * v19 + 1) = v18;
      v18 = (long long *)*v18;
      ++v19;
    }
    sub_CBB9(v23, v13, v13, a3);
    free(v23);
  }
  sub_4B69();
  return v31 - __readfsqword(0x28u);
}



// Function: insertkey @ 0xd5d8
long long insertkey(long long a1)
{
  long long *i; // [rsp+10h] [rbp-10h]
  long long v3; // [rsp+18h] [rbp-8h]

  v3 = sub_18564(a1, 72);
  for ( i = &qword_23688; *i; i = (long long *)(*i + 64) )
    ;
  *i = v3;
  *(long long *)(v3 + 64) = 0;
  return v3;
}



// Function: badfieldspec @ 0xd640
long long badfieldspec(long long a1, const char *a2)
{
  long long v2; // r12
  char *v3; // rbx
  char *v4; // rax

  v2 = sub_154A0(a1);
  v3 = gettext(a2);
  v4 = gettext("%s: invalid field specification %s");
  error(2, 0, v4, v3, v2);
  return sub_D6A1();
}



// Function: incompatible_options @ 0xd6a1
long long incompatible_options(long long a1)
{
  char *v1; // rax

  v1 = gettext("options '-%s' are incompatible");
  error(2, 0, v1, a1);
  return sub_D6DE();
}



// Function: check_ordering_compatibility @ 0xd6de
unsigned long long check_ordering_compatibility()
{
  long long i; // [rsp+8h] [rbp-38h]
  char v2[40]; // [rsp+10h] [rbp-30h] BYREF
  unsigned long long v3; // [rsp+38h] [rbp-8h]

  v3 = __readfsqword(0x28u);
  for ( i = qword_23688; i; i = *(long long *)(i + 64) )
  {
    if ( *(unsigned char *)(i + 53)
       + *(unsigned char *)(i + 52)
       + *(unsigned char *)(i + 50)
       + *(unsigned char *)(i + 54)
       + ((unsigned char)(*(char *)(i + 56) | *(char *)(i + 51)) | (*(long long *)(i + 32) != 0)) > 1 )
    {
      *(char *)(i + 55) = 0;
      *(char *)(i + 49) = *(char *)(i + 55);
      *(char *)(i + 48) = *(char *)(i + 49);
      sub_839E(i, v2);
      sub_D6A1((long long)v2);
    }
  }
  return v3 - __readfsqword(0x28u);
}



// Function: parse_field_count @ 0xd7de
long long parse_field_count(long long a1, long long *a2, const char *a3)
{
  long long v3; // r12
  char *v4; // rbx
  char *v5; // rax
  long long v8; // [rsp+28h] [rbp-28h] BYREF
  long long v9[4]; // [rsp+30h] [rbp-20h] BYREF

  v9[1] = __readfsqword(0x28u);
  switch ( (unsigned int)sub_18B54(a1, &v8, 10, v9, locale) )
  {
    case 0u:
    case 2u:
      *a2 = v9[0];
      if ( *a2 == v9[0] )
        return v8;
      goto LABEL_3;
    case 1u:
    case 3u:
LABEL_3:
      *a2 = -1;
      return v8;
    case 4u:
      if ( a3 )
      {
        v3 = sub_154A0(a1);
        v4 = gettext(a3);
        v5 = gettext("%s: invalid count at start of %s");
        error(2, 0, v5, v4, v3);
      }
      return 0;
    default:
      return v8;
  }
}



// Function: sighandler @ 0xd8f9
int sighandler(int a1)
{
  sub_4B8B();
  signal(a1, 0);
  return raise(a1);
}



// Function: set_ordering @ 0xd929
char *set_ordering(char *a1, long long a2, int a3)
{
  while ( 2 )
  {
    if ( *a1 )
    {
      switch ( *a1 )
      {
        case 'M':
          *(char *)(a2 + 54) = 1;
          goto LABEL_21;
        case 'R':
          *(char *)(a2 + 51) = 1;
          goto LABEL_21;
        case 'V':
          *(char *)(a2 + 56) = 1;
          goto LABEL_21;
        case 'b':
          if ( !a3 || a3 == 2 )
            *(char *)(a2 + 48) = 1;
          if ( a3 == 1 || a3 == 2 )
            *(char *)(a2 + 49) = 1;
          goto LABEL_21;
        case 'd':
          *(long long *)(a2 + 32) = byte_23460;
          goto LABEL_21;
        case 'f':
          *(long long *)(a2 + 40) = byte_23560;
          goto LABEL_21;
        case 'g':
          *(char *)(a2 + 52) = 1;
          goto LABEL_21;
        case 'h':
          *(char *)(a2 + 53) = 1;
          goto LABEL_21;
        case 'i':
          if ( !*(long long *)(a2 + 32) )
            *(long long *)(a2 + 32) = byte_23360;
          goto LABEL_21;
        case 'n':
          *(char *)(a2 + 50) = 1;
          goto LABEL_21;
        case 'r':
          *(char *)(a2 + 55) = 1;
LABEL_21:
          ++a1;
          continue;
        default:
          return a1;
      }
    }
    return a1;
  }
}



// Function: key_init @ 0xda55
long long *key_init(long long *a1)
{
  memset(a1, 0, 0x48u);
  a1[2] = -1;
  return a1;
}



// Function: main @ 0xda8d
long long main(int a1, char **a2, char **a3)
{
  bool v3; // al
  int v4; // eax
  int v5; // eax
  char **v6; // rcx
  unsigned long long v7; // rax
  bool v8; // al
  int v9; // edx
  int v10; // eax
  int v11; // eax
  unsigned long long v12; // rax
  int v13; // eax
  char *v14; // rax
  long long v16; // rax
  long long v17; // rax
  char *v18; // rax
  char *v19; // rax
  char *v20; // rax
  long long v21; // rbx
  char *v22; // rax
  char *v23; // rax
  long long v24; // rbx
  char *v25; // rax
  char *v26; // rax
  long long v27; // rbx
  char *v28; // rax
  long long v29; // rbx
  char *v30; // rax
  long long v31; // rbx
  char *v32; // rax
  long long v33; // rbx
  char *v34; // rax
  char v35; // al
  char *v36; // rax
  char *v37; // rax
  long long v38; // rbx
  char *v39; // rax
  char *v40; // rax
  const char *v41; // rax
  long long v42; // rax
  long long v43; // rbx
  char *v44; // rax
  unsigned char v45; // al
  unsigned long long v46; // rax
  unsigned long long v47; // rax
  char *v48; // rax
  char v50; // [rsp+1Fh] [rbp-331h]
  char v51; // [rsp+20h] [rbp-330h]
  char v52; // [rsp+21h] [rbp-32Fh]
  bool v53; // [rsp+22h] [rbp-32Eh]
  bool v54; // [rsp+23h] [rbp-32Dh]
  bool v55; // [rsp+24h] [rbp-32Ch]
  char v56; // [rsp+25h] [rbp-32Bh]
  bool v57; // [rsp+26h] [rbp-32Ah]
  bool v58; // [rsp+27h] [rbp-329h]
  int v59; // [rsp+28h] [rbp-328h]
  int v60; // [rsp+2Ch] [rbp-324h]
  long long *v61; // [rsp+30h] [rbp-320h]
  long long *v62; // [rsp+30h] [rbp-320h]
  long long n; // [rsp+30h] [rbp-320h]
  char *v64; // [rsp+38h] [rbp-318h]
  char *v65; // [rsp+38h] [rbp-318h]
  char *v66; // [rsp+38h] [rbp-318h]
  char *v67; // [rsp+38h] [rbp-318h]
  char *v68; // [rsp+38h] [rbp-318h]
  char *v69; // [rsp+40h] [rbp-310h]
  unsigned long long v70; // [rsp+48h] [rbp-308h]
  unsigned long long v71; // [rsp+50h] [rbp-300h]
  long long *ptr; // [rsp+58h] [rbp-2F8h]
  char *v73; // [rsp+60h] [rbp-2F0h]
  char *s1; // [rsp+68h] [rbp-2E8h]
  unsigned long long i; // [rsp+70h] [rbp-2E0h]
  unsigned long long j; // [rsp+70h] [rbp-2E0h]
  char *k; // [rsp+78h] [rbp-2D8h]
  unsigned long long m; // [rsp+80h] [rbp-2D0h]
  unsigned long long ii; // [rsp+88h] [rbp-2C8h]
  struct lconv *v80; // [rsp+90h] [rbp-2C0h]
  char *v81; // [rsp+98h] [rbp-2B8h]
  FILE *v82; // [rsp+A0h] [rbp-2B0h]
  char *v83; // [rsp+B0h] [rbp-2A0h]
  long long *v84; // [rsp+C8h] [rbp-288h]
  long long v85[10]; // [rsp+D0h] [rbp-280h] BYREF
  long long v86[6]; // [rsp+120h] [rbp-230h] BYREF
  char v87; // [rsp+150h] [rbp-200h]
  char v88; // [rsp+151h] [rbp-1FFh]
  char v89; // [rsp+152h] [rbp-1FEh]
  char v90; // [rsp+153h] [rbp-1FDh]
  char v91; // [rsp+154h] [rbp-1FCh]
  char v92; // [rsp+155h] [rbp-1FBh]
  char v93; // [rsp+156h] [rbp-1FAh]
  char v94; // [rsp+157h] [rbp-1F9h]
  char v95; // [rsp+158h] [rbp-1F8h]
  int longind[2]; // [rsp+170h] [rbp-1E0h] BYREF
  long long *v97; // [rsp+178h] [rbp-1D8h]
  struct sigaction oact; // [rsp+290h] [rbp-C0h] BYREF
  char v99[2]; // [rsp+336h] [rbp-1Ah] BYREF
  unsigned long long v100; // [rsp+338h] [rbp-18h]

  v100 = __readfsqword(0x28u);
  v50 = 0;
  v59 = 0;
  v51 = 0;
  v52 = 0;
  v69 = 0;
  v53 = 0;
  v70 = 0;
  v71 = 0;
  v57 = getenv("POSIXLY_CORRECT") != 0;
  v60 = sub_133AB();
  v3 = v60 <= 200111 || v60 > 200808;
  v54 = v3;
  v73 = 0;
  s1 = 0;
  sub_1345F(*a2);
  v55 = setlocale(6, locale) != 0;
  bindtextdomain("coreutils", "/usr/local/share/locale");
  textdomain("coreutils");
  sub_4029(2);
  byte_23229 = sub_107AA(3);
  byte_2322A = sub_107AA(2);
  v80 = localeconv();
  byte_23220 = *v80->decimal_point;
  if ( !byte_23220 || v80->decimal_point[1] )
    byte_23220 = 46;
  dword_23224 = *v80->thousands_sep;
  if ( dword_23224 && v80->thousands_sep[1] )
    byte_23228 = 1;
  if ( !dword_23224 || v80->thousands_sep[1] )
    dword_23224 = 128;
  byte_23683 = 0;
  sub_594A();
  sigemptyset(&set);
  for ( i = 0; i <= 0xA; ++i )
  {
    sigaction(dword_1D100[i], 0, &oact);
    if ( oact.sa_handler != (__sighandler_t)((char *)&dword_0 + 1) )
      sigaddset(&set, dword_1D100[i]);
  }
  oact.sa_handler = (__sighandler_t)sub_D8F9;
  oact.sa_mask = set;
  oact.sa_flags = 0;
  for ( j = 0; j <= 0xA; ++j )
  {
    if ( sigismember(&set, dword_1D100[j]) )
      sigaction(dword_1D100[j], &oact, 0);
  }
  signal(17, 0);
  sub_1A960(sub_4BD4);
  sub_DA55(v86);
  v86[0] = -1;
  for ( ptr = (long long *)sub_1819D(a1, 8); ; ptr[v7] = *v6 )
  {
    while ( 2 )
    {
      longind[0] = -1;
      if ( v59 != -1
        && (!v57
         || !v71
         || v54
         && !v51
         && a1 != optind
         && *a2[optind] == 45
         && a2[optind][1] == 111
         && (a2[optind][2] || a1 != optind + 1)) )
      {
        v4 = getopt_long(a1, a2, "-bcCdfghik:mMno:rRsS:t:T:uVy:z", &longopts, longind);
        v59 = v4;
        if ( v4 != -1 )
        {
          if ( v4 <= 135 )
          {
            if ( v4 > 0 )
            {
              switch ( v4 )
              {
                case 1:
                  v61 = 0;
                  if ( *optarg == 43 )
                  {
                    v8 = a1 != optind && *a2[optind] == 45 && (unsigned int)(a2[optind][1] - 48) <= 9;
                    v58 = v8;
                    v9 = v54;
                    v10 = v8 && !v57;
                    v54 = v54 || v10;
                    if ( v9 || v10 )
                    {
                      v61 = sub_DA55(v85);
                      v64 = (char *)sub_D7DE((long long)(optarg + 1), v61, 0);
                      if ( v64 && *v64 == 46 )
                        v64 = (char *)sub_D7DE((long long)(v64 + 1), v61 + 1, 0);
                      if ( !*v61 && !v61[1] )
                        *v61 = -1;
                      if ( v64 && !*sub_D929(v64, (long long)v61, 0) )
                      {
                        if ( v58 )
                        {
                          v11 = optind++;
                          v81 = a2[v11];
                          v65 = (char *)sub_D7DE((long long)(v81 + 1), v61 + 2, "invalid number after '-'");
                          if ( !v65 )
                            __assert_fail("s", "src/sort.c", 0x1161u, "main");
                          if ( *v65 == 46 )
                            v65 = (char *)sub_D7DE((long long)(v65 + 1), v61 + 3, "invalid number after '.'");
                          if ( !v61[3] && v61[2] )
                            --v61[2];
                          if ( *sub_D929(v65, (long long)v61, 1) )
                            sub_D640((long long)v81, "stray character in field spec");
                        }
                        *((char *)v61 + 57) = 1;
                        sub_D5D8((long long)v61);
                      }
                      else
                      {
                        v61 = 0;
                      }
                    }
                  }
                  if ( !v61 )
                  {
                    v12 = v71++;
                    ptr[v12] = optarg;
                  }
                  continue;
                case 67:
                case 99:
                  goto LABEL_85;
                case 77:
                case 82:
                case 86:
                case 98:
                case 100:
                case 102:
                case 103:
                case 104:
                case 105:
                case 110:
                case 114:
                  goto LABEL_80;
                case 83:
                  sub_5E0C(longind[0], v4, (long long)optarg);
                  continue;
                case 84:
                  sub_570F((long long)optarg);
                  continue;
                case 107:
                  v62 = sub_DA55(v85);
                  v66 = (char *)sub_D7DE((long long)optarg, v62, "invalid number at field start");
                  if ( !(*v62)-- )
                    sub_D640((long long)optarg, "field number is zero");
                  if ( *v66 == 46 )
                  {
                    v66 = (char *)sub_D7DE((long long)(v66 + 1), v62 + 1, "invalid number after '.'");
                    v16 = v62[1];
                    v62[1] = v16 - 1;
                    if ( !v16 )
                      sub_D640((long long)optarg, "character offset is zero");
                  }
                  if ( !*v62 && !v62[1] )
                    *v62 = -1;
                  v67 = sub_D929(v66, (long long)v62, 0);
                  if ( *v67 == 44 )
                  {
                    v68 = (char *)sub_D7DE((long long)(v67 + 1), v62 + 2, "invalid number after ','");
                    v17 = v62[2];
                    v62[2] = v17 - 1;
                    if ( !v17 )
                      sub_D640((long long)optarg, "field number is zero");
                    if ( *v68 == 46 )
                      v68 = (char *)sub_D7DE((long long)(v68 + 1), v62 + 3, "invalid number after '.'");
                    v67 = sub_D929(v68, (long long)v62, 1);
                  }
                  else
                  {
                    v62[2] = -1;
                    v62[3] = 0;
                  }
                  if ( *v67 )
                    sub_D640((long long)optarg, "stray character in field spec");
                  sub_D5D8((long long)v62);
                  continue;
                case 109:
                  v52 = 1;
                  continue;
                case 111:
                  if ( s1 && strcmp(s1, optarg) )
                  {
                    v18 = gettext("multiple output files specified");
                    error(2, 0, v18);
                  }
                  s1 = optarg;
                  continue;
                case 115:
                  byte_23681 = 1;
                  continue;
                case 116:
                  v56 = *optarg;
                  if ( !*optarg )
                  {
                    v20 = gettext("empty tab");
                    error(2, 0, v20);
                  }
                  if ( optarg[1] )
                  {
                    if ( !strcmp(optarg, "\\0") )
                    {
                      v56 = 0;
                    }
                    else
                    {
                      v21 = sub_154A0(optarg);
                      v22 = gettext("multi-character tab %s");
                      error(2, 0, v22, v21);
                    }
                  }
                  if ( dword_23020 != 128 && v56 != dword_23020 )
                  {
                    v23 = gettext("incompatible tabs");
                    error(2, 0, v23);
                  }
                  dword_23020 = v56;
                  continue;
                case 117:
                  byte_23682 = 1;
                  continue;
                case 121:
                  if ( a2[optind - 1] == optarg )
                  {
                    for ( k = optarg; (unsigned int)(*k - 48) <= 9; ++k )
                      ;
                    optind -= *k != 0;
                  }
                  continue;
                case 122:
                  byte_23010 = 0;
                  continue;
                case 128:
                  if ( optarg )
                    v13 = aCccgeneralNume[sub_F8C1(
                                            (unsigned int)"--check",
                                            (int)optarg,
                                            (unsigned int)off_22940,
                                            (unsigned int)"CCcgeneral-numeric",
                                            1,
                                            (int)off_23110,
                                            1)];
                  else
                    v13 = 99;
                  v59 = v13;
LABEL_85:
                  if ( v51 && v59 != v51 )
                    sub_D6A1((long long)"cC");
                  v51 = v59;
                  continue;
                case 129:
                  if ( ::s1 && strcmp(::s1, optarg) )
                  {
                    v14 = gettext("multiple compress programs specified");
                    error(2, 0, v14);
                  }
                  ::s1 = optarg;
                  continue;
                case 130:
                  byte_23698 = 1;
                  continue;
                case 131:
                  v73 = optarg;
                  continue;
                case 132:
                  sub_5BA8(longind[0], v4, (long long)optarg);
                  continue;
                case 133:
                  if ( v69 && strcmp(v69, optarg) )
                  {
                    v19 = gettext("multiple random sources specified");
                    error(2, 0, v19);
                  }
                  v69 = optarg;
                  continue;
                case 134:
                  v59 = aGhmnrvwaitingF[sub_F8C1(
                                          (unsigned int)"--sort",
                                          (int)optarg,
                                          (unsigned int)&off_22960,
                                          (unsigned int)"ghMnRVwaiting for %s [-d]",
                                          1,
                                          (int)off_23110,
                                          1)];
LABEL_80:
                  v99[0] = v59;
                  v99[1] = 0;
                  sub_D929(v99, (long long)v86, 2);
                  continue;
                case 135:
                  v70 = sub_6042(longind[0], v4, (long long)optarg);
                  continue;
                default:
                  goto LABEL_146;
              }
            }
            if ( v4 == -131 )
            {
              sub_17E44(
                (int)stdout,
                (unsigned int)"sort",
                (unsigned int)"GNU coreutils",
                (int)off_23108,
                (unsigned int)"Mike Haertel",
                (unsigned int)"Paul Eggert",
                0);
              exit(0);
            }
            if ( v4 == -130 )
              sub_44D3(0);
          }
LABEL_146:
          sub_44D3(2);
        }
      }
      break;
    }
    if ( a1 <= optind )
      break;
    v5 = optind++;
    v6 = &a2[v5];
    v7 = v71++;
  }
  if ( v73 )
  {
    if ( v71 )
    {
      v24 = sub_15059(4, *ptr);
      v25 = gettext("extra operand %s");
      error(0, 0, v25, v24);
      v26 = gettext("file operands cannot be combined with --files0-from");
      fprintf(stderr, "%s\n", v26);
      sub_44D3(2);
    }
    v82 = sub_505E(v73, "r");
    sub_1676C(longind);
    if ( (unsigned char)sub_16B8B(v82, longind) != 1 )
    {
      v27 = sub_15059(4, v73);
      v28 = gettext("cannot read file names from %s");
      error(2, 0, v28, v27);
    }
    sub_50B7(v82, v73);
    if ( *(long long *)longind )
    {
      free(ptr);
      ptr = v97;
      v71 = *(long long *)longind;
      for ( m = 0; m < v71; ++m )
      {
        if ( !strcmp((const char *)ptr[m], "-") )
        {
          v29 = sub_15059(4, ptr[m]);
          v30 = gettext("when reading file names from stdin, no file name of %s allowed");
          error(2, 0, v30, v29);
        }
        if ( !*(char *)ptr[m] )
        {
          v31 = sub_151EB(0, 3, v73);
          v32 = gettext("%s:%lu: invalid zero-length file name");
          error(2, 0, v32, v31, m + 1);
        }
      }
    }
    else
    {
      v33 = sub_15059(4, v73);
      v34 = gettext("no input from %s");
      error(2, 0, v34, v33);
    }
  }
  for ( n = qword_23688; n; n = *(long long *)(n + 64) )
  {
    if ( sub_8305(n) && *(char *)(n + 55) != 1 )
    {
      *(long long *)(n + 32) = v86[4];
      *(long long *)(n + 40) = v86[5];
      *(char *)(n + 48) = v87;
      *(char *)(n + 49) = v88;
      *(char *)(n + 54) = v93;
      *(char *)(n + 50) = v89;
      *(char *)(n + 52) = v91;
      *(char *)(n + 53) = v92;
      *(char *)(n + 56) = v95;
      *(char *)(n + 51) = v90;
      *(char *)(n + 55) = v94;
    }
    v53 = (unsigned char)(v53 | *(char *)(n + 51)) != 0;
  }
  if ( !qword_23688 && !sub_8305((long long)v86) )
  {
    v50 = 1;
    sub_D5D8((long long)v86);
    v53 = (unsigned char)(v53 | v90) != 0;
  }
  sub_D6DE();
  if ( byte_23698 )
  {
    if ( v51 || s1 )
    {
      if ( v51 )
        v35 = v51;
      else
        v35 = 111;
      aXDebug[0] = v35;
      sub_D6A1((long long)aXDebug);
    }
    if ( v55 )
      v55 = setlocale(3, locale) != 0;
    if ( !v55 )
    {
      v36 = gettext("failed to set locale");
      error(0, 0, "%s", v36);
    }
    if ( byte_23229 )
    {
      v37 = setlocale(3, 0);
      v38 = sub_154A0(v37);
      v39 = gettext("text ordering performed using %s sorting rules");
      error(0, 0, v39, v38);
    }
    else
    {
      v40 = gettext("text ordering performed using simple byte comparison");
      error(0, 0, "%s", v40);
    }
    sub_84FE(v86, v50);
  }
  byte_23680 = v94;
  if ( v53 )
    sub_7573(v69);
  if ( !qword_23670 )
  {
    v83 = getenv("TMPDIR");
    if ( v83 )
      v41 = v83;
    else
      v41 = "/tmp";
    sub_570F((long long)v41);
  }
  if ( !v71 )
  {
    v71 = 1;
    free(ptr);
    ptr = (long long *)sub_18024(8);
    *ptr = "-";
  }
  if ( qword_23660 )
  {
    v42 = qword_23660;
    if ( 34 * (unsigned long long)(unsigned int)dword_23024 >= qword_23660 )
      v42 = 34LL * (unsigned int)dword_23024;
    qword_23660 = v42;
  }
  if ( v51 )
  {
    if ( v71 > 1 )
    {
      v43 = sub_15059(4, ptr[1]);
      v44 = gettext("extra operand %s not allowed with -%c");
      error(2, 0, v44, v43, (unsigned int)v51);
    }
    if ( s1 )
    {
      byte_23032 = v51;
      sub_D6A1((long long)&byte_23032);
    }
    v45 = sub_A0B1((char *)*ptr, v51);
    exit(v45 ^ 1);
  }
  sub_CA81((long long)ptr, v71);
  sub_CB44(s1);
  if ( v52 )
  {
    v84 = (long long *)sub_18502(v71, 16);
    for ( ii = 0; ii < v71; ++ii )
      v84[2 * ii] = ptr[ii];
    sub_CBB9(v84, 0, v71, s1);
  }
  else
  {
    if ( !v70 )
    {
      v46 = sub_12DE4(2);
      if ( v46 > 8 )
        v46 = 8;
      v70 = v46;
    }
    v47 = 0xFFFFFFFFFFFFFFLL;
    if ( v70 <= 0xFFFFFFFFFFFFFFLL )
      v47 = v70;
    sub_D0F9((char **)ptr, v71, s1, v47);
  }
  if ( byte_23683 && (unsigned int)sub_FF17(stdin) == -1 )
  {
    v48 = gettext("close failed");
    sub_4464(v48, "-");
  }
  return 0;
}



// Function: hash_get_n_buckets @ 0x10854
long long hash_get_n_buckets(long long a1)
{
  return *(long long *)(a1 + 16);
}



// Function: hash_get_n_buckets_used @ 0x1086a
long long hash_get_n_buckets_used(long long a1)
{
  return *(long long *)(a1 + 24);
}



// Function: hash_get_n_entries @ 0x10880
long long hash_get_n_entries(long long a1)
{
  return *(long long *)(a1 + 32);
}



// Function: hash_get_max_bucket_length @ 0x10896
unsigned long long hash_get_max_bucket_length(long long a1)
{
  long long *i; // [rsp+8h] [rbp-20h]
  unsigned long long v3; // [rsp+10h] [rbp-18h]
  long long *v4; // [rsp+18h] [rbp-10h]
  unsigned long long j; // [rsp+20h] [rbp-8h]

  v3 = 0;
  for ( i = *(long long **)a1; (unsigned long long)i < *(long long *)(a1 + 8); i += 2 )
  {
    if ( *i )
    {
      v4 = i;
      for ( j = 1; ; ++j )
      {
        v4 = (long long *)v4[1];
        if ( !v4 )
          break;
      }
      if ( j > v3 )
        v3 = j;
    }
  }
  return v3;
}



// Function: hash_table_ok @ 0x10918
long long hash_table_ok(long long a1)
{
  long long *i; // [rsp+8h] [rbp-20h]
  long long v3; // [rsp+10h] [rbp-18h]
  long long v4; // [rsp+18h] [rbp-10h]
  long long *v5; // [rsp+20h] [rbp-8h]

  v3 = 0;
  v4 = 0;
  for ( i = *(long long **)a1; (unsigned long long)i < *(long long *)(a1 + 8); i += 2 )
  {
    if ( *i )
    {
      v5 = i;
      ++v3;
      ++v4;
      while ( 1 )
      {
        v5 = (long long *)v5[1];
        if ( !v5 )
          break;
        ++v4;
      }
    }
  }
  return v3 == *(long long *)(a1 + 24) && v4 == *(long long *)(a1 + 32);
}



// Function: hash_print_statistics @ 0x109b6
int hash_print_statistics(long long a1, FILE *a2)
{
  double v2; // xmm0_8
  double v3; // xmm1_8
  double v4; // xmm0_8
  long long v6; // [rsp+10h] [rbp-20h]
  long long v7; // [rsp+18h] [rbp-18h]
  long long v8; // [rsp+20h] [rbp-10h]
  unsigned long long v9; // [rsp+28h] [rbp-8h]

  v6 = sub_10880(a1);
  v7 = sub_10854(a1);
  v8 = sub_1086A(a1);
  v9 = sub_10896(a1);
  fprintf(a2, "# entries:         %lu\n", v6);
  fprintf(a2, "# buckets:         %lu\n", v7);
  if ( v8 < 0 )
    v2 = (double)(int)(v8 & 1 | ((unsigned long long)v8 >> 1)) + (double)(int)(v8 & 1 | ((unsigned long long)v8 >> 1));
  else
    v2 = (double)(int)v8;
  v3 = 100.0 * v2;
  if ( v7 < 0 )
    v4 = (double)(int)(v7 & 1 | ((unsigned long long)v7 >> 1)) + (double)(int)(v7 & 1 | ((unsigned long long)v7 >> 1));
  else
    v4 = (double)(int)v7;
  fprintf(a2, "# buckets used:    %lu (%.2f%%)\n", v8, v3 / v4);
  return fprintf(a2, "max bucket length: %lu\n", v9);
}



// Function: safe_hasher @ 0x10afd
long long safe_hasher(long long a1, long long a2)
{
  unsigned long long v3; // [rsp+18h] [rbp-8h]

  v3 = (*(long long (**)(long long, long long))(a1 + 48))(a2, *(long long *)(a1 + 16));
  if ( v3 >= *(long long *)(a1 + 16) )
    abort();
  return *(long long *)a1 + 16 * v3;
}



// Function: hash_lookup @ 0x10b58
long long hash_lookup(long long a1, long long a2)
{
  long long *i; // [rsp+10h] [rbp-10h]
  long long *v4; // [rsp+18h] [rbp-8h]

  v4 = (long long *)sub_10AFD(a1, a2);
  if ( !*v4 )
    return 0;
  for ( i = v4; i; i = (long long *)i[1] )
  {
    if ( a2 == *i || (*(unsigned char (**)(long long, long long))(a1 + 56))(a2, *i) )
      return *i;
  }
  return 0;
}



// Function: hash_get_first @ 0x10bef
long long hash_get_first(unsigned long long *a1)
{
  unsigned long long i; // [rsp+18h] [rbp-8h]

  if ( !a1[4] )
    return 0;
  for ( i = *a1; ; i += 16LL )
  {
    if ( i >= a1[1] )
      abort();
    if ( *(long long *)i )
      break;
  }
  return *(long long *)i;
}



// Function: hash_get_next @ 0x10c4f
long long hash_get_next(long long a1, long long a2)
{
  unsigned long long v3; // [rsp+10h] [rbp-10h]
  unsigned long long v4; // [rsp+18h] [rbp-8h]

  v3 = sub_10AFD(a1, a2);
  v4 = v3;
  do
  {
    if ( a2 == *(long long *)v4 && *(long long *)(v4 + 8) )
      return **(long long **)(v4 + 8);
    v4 = *(long long *)(v4 + 8);
  }
  while ( v4 );
  while ( 1 )
  {
    v3 += 16LL;
    if ( v3 >= *(long long *)(a1 + 8) )
      break;
    if ( *(long long *)v3 )
      return *(long long *)v3;
  }
  return 0;
}



// Function: hash_get_entries @ 0x10ced
unsigned long long hash_get_entries(long long a1, long long a2, unsigned long long a3)
{
  long long v4; // rax
  unsigned long long v5; // [rsp+20h] [rbp-18h]
  long long *i; // [rsp+28h] [rbp-10h]
  long long *j; // [rsp+30h] [rbp-8h]

  v5 = 0;
  for ( i = *(long long **)a1; (unsigned long long)i < *(long long *)(a1 + 8); i += 2 )
  {
    if ( *i )
    {
      for ( j = i; j; j = (long long *)j[1] )
      {
        if ( v5 >= a3 )
          return v5;
        v4 = v5++;
        *(long long *)(a2 + 8 * v4) = *j;
      }
    }
  }
  return v5;
}



// Function: hash_do_for_each @ 0x10d8d
long long hash_do_for_each(long long a1, unsigned char (*a2)(long long, long long), long long a3)
{
  long long v5; // [rsp+28h] [rbp-18h]
  long long *i; // [rsp+30h] [rbp-10h]
  long long *j; // [rsp+38h] [rbp-8h]

  v5 = 0;
  for ( i = *(long long **)a1; (unsigned long long)i < *(long long *)(a1 + 8); i += 2 )
  {
    if ( *i )
    {
      for ( j = i; j; j = (long long *)j[1] )
      {
        if ( a2(*j, a3) != 1 )
          return v5;
        ++v5;
      }
    }
  }
  return v5;
}



// Function: hash_string @ 0x10e25
unsigned long long hash_string(char *a1, unsigned long long a2)
{
  unsigned long long v4; // [rsp+18h] [rbp-8h]

  v4 = 0;
  while ( *a1 )
    v4 = (31 * v4 + (unsigned char)*a1++) % a2;
  return v4;
}



// Function: is_prime @ 0x10e7f
bool is_prime(unsigned long long a1)
{
  unsigned long long v2; // [rsp+8h] [rbp-10h]
  unsigned long long v3; // [rsp+8h] [rbp-10h]
  unsigned long long v4; // [rsp+10h] [rbp-8h]

  v2 = 3;
  v4 = 9;
  while ( v4 < a1 && a1 % v2 )
  {
    v3 = v2 + 1;
    v4 += 4 * v3;
    v2 = v3 + 1;
  }
  return a1 % v2 != 0;
}



// Function: next_prime @ 0x10eee
unsigned long long next_prime(unsigned long long a1)
{
  long long v2; // [rsp+0h] [rbp-8h]
  unsigned long long i; // [rsp+0h] [rbp-8h]

  v2 = a1;
  if ( a1 <= 9 )
    v2 = 10;
  for ( i = v2 | 1; i != -1 && !sub_10E7F(i); i += 2LL )
    ;
  return i;
}



// Function: hash_reset_tuning @ 0x10f39
long long hash_reset_tuning(long long a1)
{
  *(int *)a1 = 0;
  *(int *)(a1 + 4) = 1065353216;
  *(int *)(a1 + 8) = 1061997773;
  *(int *)(a1 + 12) = 1068826100;
  *(char *)(a1 + 16) = 0;
  return a1;
}



// Function: raw_hasher @ 0x10f8f
unsigned long long raw_hasher(long long a1, unsigned long long a2)
{
  return sub_19063(a1, 3) % a2;
}



// Function: raw_comparator @ 0x10fca
bool raw_comparator(long long a1, long long a2)
{
  return a1 == a2;
}



// Function: check_tuning @ 0x10fe7
long long check_tuning(long long a1)
{
  long long v2; // [rsp+10h] [rbp-8h]

  v2 = *(long long *)(a1 + 40);
  if ( (_UNKNOWN *)v2 == &unk_1D1F0 )
    return 1;
  if ( *(float *)(v2 + 8) > 0.1
    && (float)(1.0 - 0.1) > *(float *)(v2 + 8)
    && *(float *)(v2 + 12) > (float)(0.1 + 1.0)
    && *(float *)v2 >= 0.0
    && *(float *)(v2 + 4) > (float)(*(float *)v2 + 0.1)
    && *(float *)(v2 + 4) <= 1.0
    && *(float *)(v2 + 8) > (float)(*(float *)v2 + 0.1) )
  {
    return 1;
  }
  *(long long *)(a1 + 40) = &unk_1D1F0;
  return 0;
}



// Function: compute_bucket_size @ 0x110f4
unsigned long long compute_bucket_size(long long a1, long long a2)
{
  float v2; // xmm0_4
  bool v4; // al
  unsigned long long v5; // [rsp+8h] [rbp-18h]
  unsigned long long v6; // [rsp+8h] [rbp-18h]
  float v7; // [rsp+1Ch] [rbp-4h]

  v5 = a1;
  if ( *(char *)(a2 + 16) != 1 )
  {
    if ( a1 < 0 )
      v2 = (float)(a1 & 1 | (unsigned int)((unsigned long long)a1 >> 1))
         + (float)(a1 & 1 | (unsigned int)((unsigned long long)a1 >> 1));
    else
      v2 = (float)(int)a1;
    v7 = v2 / *(float *)(a2 + 8);
    if ( v7 >= 1.8446744e19 )
      return 0;
    if ( v7 >= 9.223372e18 )
      v5 = (unsigned int)(int)(float)(v7 - 9.223372e18) ^ 0x8000000000000000LL;
    else
      v5 = (unsigned int)(int)v7;
  }
  v6 = sub_10EEE(v5);
  v4 = v6 >> 61 != 0;
  if ( (v6 & 0x1000000000000000LL) != 0 )
    v4 = 1;
  if ( v4 )
    return 0;
  else
    return v6;
}



// Function: hash_initialize @ 0x1120d
size_t *hash_initialize(
        long long a1,
        void *a2,
        unsigned long long (*a3)(long long a1, unsigned long long a2),
        bool (*a4)(long long a1, long long a2),
        size_t a5)
{
  bool (*v7)(long long, long long); // [rsp+10h] [rbp-30h]
  unsigned long long (*v8)(long long, unsigned long long); // [rsp+18h] [rbp-28h]
  void *v9; // [rsp+20h] [rbp-20h]
  size_t *ptr; // [rsp+38h] [rbp-8h]

  v9 = a2;
  v8 = a3;
  v7 = a4;
  if ( !a3 )
    v8 = sub_10F8F;
  if ( !a4 )
    v7 = sub_10FCA;
  ptr = (size_t *)malloc(0x50u);
  if ( !ptr )
    return 0;
  if ( !a2 )
    v9 = &unk_1D1F0;
  ptr[5] = (size_t)v9;
  if ( (unsigned char)sub_10FE7((long long)ptr) == 1
    && (ptr[2] = sub_110F4(a1, (long long)v9)) != 0
    && (*ptr = (size_t)calloc(ptr[2], 0x10u)) != 0 )
  {
    ptr[1] = 16 * ptr[2] + *ptr;
    ptr[3] = 0;
    ptr[4] = 0;
    ptr[6] = (size_t)v8;
    ptr[7] = (size_t)v7;
    ptr[8] = a5;
    ptr[9] = 0;
    return ptr;
  }
  else
  {
    free(ptr);
    return 0;
  }
}



// Function: hash_clear @ 0x11382
long long hash_clear(long long a1)
{
  long long *i; // [rsp+18h] [rbp-18h]
  long long *j; // [rsp+20h] [rbp-10h]
  long long *v4; // [rsp+28h] [rbp-8h]

  for ( i = *(long long **)a1; (unsigned long long)i < *(long long *)(a1 + 8); i += 2 )
  {
    if ( *i )
    {
      for ( j = (long long *)i[1]; j; j = v4 )
      {
        if ( *(long long *)(a1 + 64) )
          (*(void (**)(long long))(a1 + 64))(*j);
        *j = 0;
        v4 = (long long *)j[1];
        j[1] = *(long long *)(a1 + 72);
        *(long long *)(a1 + 72) = j;
      }
      if ( *(long long *)(a1 + 64) )
        (*(void (**)(long long))(a1 + 64))(*i);
      *i = 0;
      i[1] = 0;
    }
  }
  *(long long *)(a1 + 24) = 0;
  *(long long *)(a1 + 32) = 0;
  return a1;
}



// Function: hash_free @ 0x1148d
void hash_free(long long a1)
{
  long long *i; // [rsp+18h] [rbp-18h]
  long long *j; // [rsp+18h] [rbp-18h]
  long long *ptr; // [rsp+20h] [rbp-10h]
  long long *ptra; // [rsp+20h] [rbp-10h]
  long long *ptrb; // [rsp+20h] [rbp-10h]
  long long *v6; // [rsp+28h] [rbp-8h]
  long long *v7; // [rsp+28h] [rbp-8h]

  if ( *(long long *)(a1 + 64) && *(long long *)(a1 + 32) )
  {
    for ( i = *(long long **)a1; (unsigned long long)i < *(long long *)(a1 + 8); i += 2 )
    {
      if ( *i )
      {
        for ( ptr = i; ptr; ptr = (long long *)ptr[1] )
          (*(void (**)(long long))(a1 + 64))(*ptr);
      }
    }
  }
  for ( j = *(long long **)a1; (unsigned long long)j < *(long long *)(a1 + 8); j += 2 )
  {
    for ( ptra = (long long *)j[1]; ptra; ptra = v6 )
    {
      v6 = (long long *)ptra[1];
      free(ptra);
    }
  }
  for ( ptrb = *(long long **)(a1 + 72); ptrb; ptrb = v7 )
  {
    v7 = (long long *)ptrb[1];
    free(ptrb);
  }
  free(*(void **)a1);
  free((void *)a1);
}



// Function: allocate_entry @ 0x115bc
void *allocate_entry(long long a1)
{
  long long v2; // [rsp+18h] [rbp-8h]

  if ( !*(long long *)(a1 + 72) )
    return malloc(0x10u);
  v2 = *(long long *)(a1 + 72);
  *(long long *)(a1 + 72) = *(long long *)(v2 + 8);
  return (void *)v2;
}



// Function: free_entry @ 0x1160b
long long free_entry(long long a1, long long *a2)
{
  *a2 = 0;
  a2[1] = *(long long *)(a1 + 72);
  *(long long *)(a1 + 72) = a2;
  return a1;
}



// Function: hash_find_entry @ 0x11645
long long hash_find_entry(long long a1, long long a2, long long *a3, char a4)
{
  long long v5; // rdx
  long long *i; // [rsp+20h] [rbp-30h]
  long long *v9; // [rsp+28h] [rbp-28h]
  long long v10; // [rsp+30h] [rbp-20h]
  long long *v11; // [rsp+38h] [rbp-18h]
  long long v12; // [rsp+40h] [rbp-10h]
  long long *v13; // [rsp+48h] [rbp-8h]

  v9 = (long long *)sub_10AFD(a1, a2);
  *a3 = v9;
  if ( !*v9 )
    return 0;
  if ( a2 == *v9 || (*(unsigned char (**)(long long, long long))(a1 + 56))(a2, *v9) )
  {
    v12 = *v9;
    if ( a4 )
    {
      if ( v9[1] )
      {
        v13 = (long long *)v9[1];
        v5 = v13[1];
        *v9 = *v13;
        v9[1] = v5;
        sub_1160B(a1, v13);
      }
      else
      {
        *v9 = 0;
      }
    }
    return v12;
  }
  else
  {
    for ( i = v9; ; i = (long long *)i[1] )
    {
      if ( !i[1] )
        return 0;
      if ( a2 == *(long long *)i[1] || (*(unsigned char (**)(long long, long long))(a1 + 56))(a2, *(long long *)i[1]) )
        break;
    }
    v10 = *(long long *)i[1];
    if ( a4 )
    {
      v11 = (long long *)i[1];
      i[1] = v11[1];
      sub_1160B(a1, v11);
    }
    return v10;
  }
}



// Function: transfer_entries @ 0x117de
long long transfer_entries(long long a1, long long a2, char a3)
{
  long long *i; // [rsp+20h] [rbp-30h]
  long long *j; // [rsp+28h] [rbp-28h]
  long long v7; // [rsp+30h] [rbp-20h]
  long long v8; // [rsp+30h] [rbp-20h]
  long long *v9; // [rsp+38h] [rbp-18h]
  long long *v10; // [rsp+38h] [rbp-18h]
  long long *v11; // [rsp+40h] [rbp-10h]
  long long *v12; // [rsp+48h] [rbp-8h]

  for ( i = *(long long **)a2; (unsigned long long)i < *(long long *)(a2 + 8); i += 2 )
  {
    if ( *i )
    {
      for ( j = (long long *)i[1]; j; j = v12 )
      {
        v7 = *j;
        v9 = (long long *)sub_10AFD(a1, *j);
        v12 = (long long *)j[1];
        if ( *v9 )
        {
          j[1] = v9[1];
          v9[1] = j;
        }
        else
        {
          *v9 = v7;
          ++*(long long *)(a1 + 24);
          sub_1160B(a1, j);
        }
      }
      v8 = *i;
      i[1] = 0;
      if ( !a3 )
      {
        v10 = (long long *)sub_10AFD(a1, v8);
        if ( *v10 )
        {
          v11 = sub_115BC(a1);
          if ( !v11 )
            return 0;
          *v11 = v8;
          v11[1] = v10[1];
          v10[1] = v11;
        }
        else
        {
          *v10 = v8;
          ++*(long long *)(a1 + 24);
        }
        *i = 0;
        --*(long long *)(a2 + 24);
      }
    }
  }
  return 1;
}



// Function: hash_rehash @ 0x119b2
long long hash_rehash(long long a1, long long a2)
{
  unsigned long long nmemb; // [rsp+10h] [rbp-70h]
  char *v4; // [rsp+20h] [rbp-60h] BYREF
  char *v5; // [rsp+28h] [rbp-58h]
  unsigned long long v6; // [rsp+30h] [rbp-50h]
  long long v7; // [rsp+38h] [rbp-48h]
  long long v8; // [rsp+40h] [rbp-40h]
  long long v9; // [rsp+48h] [rbp-38h]
  long long v10; // [rsp+50h] [rbp-30h]
  long long v11; // [rsp+58h] [rbp-28h]
  long long v12; // [rsp+60h] [rbp-20h]
  long long v13; // [rsp+68h] [rbp-18h]
  unsigned long long v14; // [rsp+78h] [rbp-8h]

  v14 = __readfsqword(0x28u);
  nmemb = sub_110F4(a2, *(long long *)(a1 + 40));
  if ( !nmemb )
    return 0;
  if ( nmemb == *(long long *)(a1 + 16) )
    return 1;
  v4 = (char *)calloc(nmemb, 0x10u);
  if ( !v4 )
    return 0;
  v6 = nmemb;
  v5 = &v4[16 * nmemb];
  v7 = 0;
  v8 = 0;
  v9 = *(long long *)(a1 + 40);
  v10 = *(long long *)(a1 + 48);
  v11 = *(long long *)(a1 + 56);
  v12 = *(long long *)(a1 + 64);
  v13 = *(long long *)(a1 + 72);
  if ( (unsigned char)sub_117DE((long long)&v4, a1, 0) )
  {
    free(*(void **)a1);
    *(long long *)a1 = v4;
    *(long long *)(a1 + 8) = v5;
    *(long long *)(a1 + 16) = v6;
    *(long long *)(a1 + 24) = v7;
    *(long long *)(a1 + 72) = v13;
    return 1;
  }
  else
  {
    *(long long *)(a1 + 72) = v13;
    if ( (unsigned char)sub_117DE(a1, (long long)&v4, 1) != 1 || (unsigned char)sub_117DE(a1, (long long)&v4, 0) != 1 )
      abort();
    free(v4);
    return 0;
  }
}



// Function: hash_insert_if_absent @ 0x11bdd
long long hash_insert_if_absent(long long *a1, long long a2, long long *a3)
{
  long long v4; // rax
  float v5; // xmm0_4
  long long v6; // rdx
  long long v7; // rax
  float v8; // xmm1_4
  long long v9; // rdx
  long long v10; // rax
  float v11; // xmm0_4
  long long v12; // rdx
  long long v13; // rax
  float v14; // xmm1_4
  long long v15; // rdx
  long long v16; // rax
  float v17; // xmm0_4
  long long v18; // rdx
  float v19; // xmm0_4
  long long v20; // rax
  float v21; // xmm0_4
  long long v22; // rdx
  unsigned long long v23; // rax
  long long *v25; // [rsp+28h] [rbp-28h] BYREF
  long long v26; // [rsp+30h] [rbp-20h]
  long long v27; // [rsp+38h] [rbp-18h]
  long long *v28; // [rsp+40h] [rbp-10h]
  unsigned long long v29; // [rsp+48h] [rbp-8h]

  v29 = __readfsqword(0x28u);
  if ( !a2 )
    abort();
  v26 = sub_11645((long long)a1, a2, &v25, 0);
  if ( v26 )
  {
    if ( a3 )
      *a3 = v26;
    return 0;
  }
  else
  {
    v4 = a1[3];
    if ( v4 < 0 )
    {
      v6 = a1[3] & 1LL | ((unsigned long long)v4 >> 1);
      v5 = (float)(int)v6 + (float)(int)v6;
    }
    else
    {
      v5 = (float)(int)v4;
    }
    v7 = a1[2];
    if ( v7 < 0 )
    {
      v9 = a1[2] & 1LL | ((unsigned long long)v7 >> 1);
      v8 = (float)(int)v9 + (float)(int)v9;
    }
    else
    {
      v8 = (float)(int)v7;
    }
    if ( v5 > (float)(v8 * *(float *)(a1[5] + 8LL)) )
    {
      sub_10FE7((long long)a1);
      v10 = a1[3];
      if ( v10 < 0 )
      {
        v12 = a1[3] & 1LL | ((unsigned long long)v10 >> 1);
        v11 = (float)(int)v12 + (float)(int)v12;
      }
      else
      {
        v11 = (float)(int)v10;
      }
      v13 = a1[2];
      if ( v13 < 0 )
      {
        v15 = a1[2] & 1LL | ((unsigned long long)v13 >> 1);
        v14 = (float)(int)v15 + (float)(int)v15;
      }
      else
      {
        v14 = (float)(int)v13;
      }
      if ( v11 > (float)(v14 * *(float *)(a1[5] + 8LL)) )
      {
        v27 = a1[5];
        if ( *(char *)(v27 + 16) )
        {
          v16 = a1[2];
          if ( v16 < 0 )
          {
            v18 = a1[2] & 1LL | ((unsigned long long)v16 >> 1);
            v17 = (float)(int)v18 + (float)(int)v18;
          }
          else
          {
            v17 = (float)(int)v16;
          }
          v19 = v17 * *(float *)(v27 + 12);
        }
        else
        {
          v20 = a1[2];
          if ( v20 < 0 )
          {
            v22 = a1[2] & 1LL | ((unsigned long long)v20 >> 1);
            v21 = (float)(int)v22 + (float)(int)v22;
          }
          else
          {
            v21 = (float)(int)v20;
          }
          v19 = *(float *)(v27 + 8) * (float)(*(float *)(v27 + 12) * v21);
        }
        if ( v19 >= 1.8446744e19 )
          return 0xFFFFFFFFLL;
        if ( v19 >= 9.223372e18 )
          v23 = (unsigned int)(int)(float)(v19 - 9.223372e18) ^ 0x8000000000000000LL;
        else
          v23 = (unsigned int)(int)v19;
        if ( (unsigned char)sub_119B2((long long)a1, v23) != 1 )
          return 0xFFFFFFFFLL;
        if ( sub_11645((long long)a1, a2, &v25, 0) )
          abort();
      }
    }
    if ( *v25 )
    {
      v28 = sub_115BC((long long)a1);
      if ( v28 )
      {
        *v28 = a2;
        v28[1] = v25[1];
        v25[1] = (long long)v28;
        ++a1[4];
        return 1;
      }
      else
      {
        return 0xFFFFFFFFLL;
      }
    }
    else
    {
      *v25 = a2;
      ++a1[4];
      ++a1[3];
      return 1;
    }
  }
}



// Function: hash_insert @ 0x11f4e
long long hash_insert(long long *a1, long long a2)
{
  int v3; // [rsp+1Ch] [rbp-14h]
  long long v4[2]; // [rsp+20h] [rbp-10h] BYREF

  v4[1] = __readfsqword(0x28u);
  v3 = sub_11BDD(a1, a2, v4);
  if ( v3 == -1 )
    return 0;
  if ( v3 )
    return a2;
  return v4[0];
}



// Function: hash_remove @ 0x11fbe
long long hash_remove(long long a1, long long a2)
{
  long long v3; // rax
  float v4; // xmm0_4
  long long v5; // rdx
  long long v6; // rax
  float v7; // xmm1_4
  long long v8; // rdx
  long long v9; // rax
  float v10; // xmm0_4
  long long v11; // rdx
  long long v12; // rax
  float v13; // xmm1_4
  long long v14; // rdx
  long long v15; // rax
  float v16; // xmm0_4
  long long v17; // rdx
  float v18; // xmm0_4
  unsigned long long v19; // rax
  long long v20; // rax
  float v21; // xmm0_4
  long long v22; // rdx
  float v23; // xmm0_4
  long long *v24; // [rsp+18h] [rbp-38h] BYREF
  void *ptr; // [rsp+20h] [rbp-30h]
  long long v26; // [rsp+28h] [rbp-28h]
  long long v27; // [rsp+30h] [rbp-20h]
  unsigned long long v28; // [rsp+38h] [rbp-18h]
  void *v29; // [rsp+40h] [rbp-10h]
  unsigned long long v30; // [rsp+48h] [rbp-8h]

  v30 = __readfsqword(0x28u);
  v26 = sub_11645(a1, a2, &v24, 1);
  if ( !v26 )
    return 0;
  --*(long long *)(a1 + 32);
  if ( !*v24 )
  {
    v3 = --*(long long *)(a1 + 24);
    if ( v3 < 0 )
    {
      v5 = *(long long *)(a1 + 24) & 1LL | ((unsigned long long)v3 >> 1);
      v4 = (float)(int)v5 + (float)(int)v5;
    }
    else
    {
      v4 = (float)(int)v3;
    }
    v6 = *(long long *)(a1 + 16);
    if ( v6 < 0 )
    {
      v8 = *(long long *)(a1 + 16) & 1LL | ((unsigned long long)v6 >> 1);
      v7 = (float)(int)v8 + (float)(int)v8;
    }
    else
    {
      v7 = (float)(int)v6;
    }
    if ( (float)(v7 * **(float **)(a1 + 40)) > v4 )
    {
      sub_10FE7(a1);
      v9 = *(long long *)(a1 + 24);
      if ( v9 < 0 )
      {
        v11 = *(long long *)(a1 + 24) & 1LL | ((unsigned long long)v9 >> 1);
        v10 = (float)(int)v11 + (float)(int)v11;
      }
      else
      {
        v10 = (float)(int)v9;
      }
      v12 = *(long long *)(a1 + 16);
      if ( v12 < 0 )
      {
        v14 = *(long long *)(a1 + 16) & 1LL | ((unsigned long long)v12 >> 1);
        v13 = (float)(int)v14 + (float)(int)v14;
      }
      else
      {
        v13 = (float)(int)v12;
      }
      if ( (float)(v13 * **(float **)(a1 + 40)) > v10 )
      {
        v27 = *(long long *)(a1 + 40);
        if ( *(char *)(v27 + 16) )
        {
          v15 = *(long long *)(a1 + 16);
          if ( v15 < 0 )
          {
            v17 = *(long long *)(a1 + 16) & 1LL | ((unsigned long long)v15 >> 1);
            v16 = (float)(int)v17 + (float)(int)v17;
          }
          else
          {
            v16 = (float)(int)v15;
          }
          v18 = v16 * *(float *)(v27 + 4);
          if ( v18 >= 9.223372e18 )
            v19 = (unsigned int)(int)(float)(v18 - 9.223372e18) ^ 0x8000000000000000LL;
          else
            v19 = (unsigned int)(int)v18;
        }
        else
        {
          v20 = *(long long *)(a1 + 16);
          if ( v20 < 0 )
          {
            v22 = *(long long *)(a1 + 16) & 1LL | ((unsigned long long)v20 >> 1);
            v21 = (float)(int)v22 + (float)(int)v22;
          }
          else
          {
            v21 = (float)(int)v20;
          }
          v23 = *(float *)(v27 + 8) * (float)(*(float *)(v27 + 4) * v21);
          if ( v23 >= 9.223372e18 )
            v19 = (unsigned int)(int)(float)(v23 - 9.223372e18) ^ 0x8000000000000000LL;
          else
            v19 = (unsigned int)(int)v23;
        }
        v28 = v19;
        if ( (unsigned char)sub_119B2(a1, v19) != 1 )
        {
          for ( ptr = *(void **)(a1 + 72); ptr; ptr = v29 )
          {
            v29 = (void *)*((long long *)ptr + 1);
            free(ptr);
          }
          *(long long *)(a1 + 72) = 0;
        }
      }
    }
  }
  return v26;
}



// Function: hash_delete @ 0x122c4
long long hash_delete(long long a1, long long a2)
{
  return sub_11FBE(a1, a2);
}



// Function: num_processors_via_affinity_mask @ 0x12bc5
long long num_processors_via_affinity_mask()
{
  long long result; // rax
  cpu_set_t cpuset; // [rsp+10h] [rbp-90h] BYREF
  unsigned long long v2; // [rsp+98h] [rbp-8h]

  v2 = __readfsqword(0x28u);
  if ( sched_getaffinity(0, 0x80u, &cpuset) )
    return 0;
  LODWORD(result) = __sched_cpucount(0x80u, &cpuset);
  if ( !(int)result )
    return 0;
  else
    return (int)result;
}



// Function: num_processors_ignoring_omp @ 0x12c4b
long long num_processors_ignoring_omp(int a1)
{
  long long v2; // [rsp+10h] [rbp-20h]
  unsigned long long v3; // [rsp+18h] [rbp-18h]
  long long v4; // [rsp+20h] [rbp-10h]
  long long v5; // [rsp+28h] [rbp-8h]

  if ( a1 == 1 )
  {
    v4 = sub_12BC5();
    if ( v4 )
      return v4;
    v5 = sysconf(84);
    if ( v5 > 0 )
      return v5;
  }
  else
  {
    v2 = sysconf(83);
    if ( v2 == 1 || v2 == 2 )
    {
      v3 = sub_12BC5();
      if ( v3 > v2 )
        v2 = v3;
    }
    if ( v2 > 0 )
      return v2;
  }
  return 1;
}



// Function: parse_omp_threads @ 0x12cdc
unsigned long long parse_omp_threads(char *a1)
{
  char *nptr; // [rsp+8h] [rbp-28h]
  char *endptr; // [rsp+10h] [rbp-20h] BYREF
  long long v4; // [rsp+18h] [rbp-18h]
  unsigned long long v5; // [rsp+20h] [rbp-10h]
  unsigned long long v6; // [rsp+28h] [rbp-8h]

  nptr = a1;
  v6 = __readfsqword(0x28u);
  v4 = 0;
  if ( !a1 )
    return v4;
  while ( *nptr && (unsigned char)sub_19332((unsigned int)*nptr) )
    ++nptr;
  if ( !(unsigned char)sub_19258((unsigned int)*nptr) )
    return v4;
  endptr = 0;
  v5 = strtoul(nptr, &endptr, 10);
  if ( !endptr )
    return v4;
  while ( *endptr && (unsigned char)sub_19332((unsigned int)*endptr) )
    ++endptr;
  if ( !*endptr )
    return v5;
  if ( *endptr == 44 )
    return v5;
  else
    return v4;
}



// Function: num_processors @ 0x12de4
unsigned long long num_processors(int a1)
{
  char *v1; // rax
  char *v2; // rax
  unsigned long long result; // rax
  int v4; // [rsp+Ch] [rbp-24h]
  unsigned long long v5; // [rsp+18h] [rbp-18h]
  unsigned long long v6; // [rsp+20h] [rbp-10h]

  v4 = a1;
  v5 = -1;
  if ( a1 == 2 )
  {
    v1 = getenv("OMP_NUM_THREADS");
    v6 = sub_12CDC(v1);
    v2 = getenv("OMP_THREAD_LIMIT");
    v5 = sub_12CDC(v2);
    if ( !v5 )
      v5 = -1;
    if ( v6 )
    {
      result = v6;
      if ( v5 <= v6 )
        return v5;
      return result;
    }
    v4 = 1;
  }
  result = sub_12C4B(v4);
  if ( v5 <= result )
    return v5;
  return result;
}


