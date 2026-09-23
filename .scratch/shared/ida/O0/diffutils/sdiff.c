// Function: try_help @ 0x3c09
long long try_help(const char *a1, long long a2)
{
  char *v2; // rax
  long long v3; // rbx
  char *v4; // rax

  if ( a1 )
  {
    v2 = gettext(a1);
    error(0, 0, v2, a2);
  }
  v3 = qword_225E8;
  v4 = gettext("Try '%s --help' for more information.");
  error(2, 0, v4, v3);
  return sub_3C7F();
}



// Function: check_stdout @ 0x3c7f
int check_stdout()
{
  int result; // eax
  char *v1; // rax

  if ( ferror_unlocked(stdout) )
    sub_3E69("write failed");
  result = fclose(stdout);
  if ( result )
  {
    v1 = gettext("standard output");
    sub_3EAB(v1);
  }
  return result;
}



// Function: usage @ 0x3cd6
long long usage()
{
  long long v0; // rbx
  char *v1; // rax
  char *v2; // rax
  FILE *v3; // rbx
  char *v4; // rax
  char *v5; // rax
  char *v6; // rbx
  char *v7; // rax
  const char **i; // [rsp+8h] [rbp-18h]

  v0 = qword_225E8;
  v1 = gettext("Usage: %s [OPTION]... FILE1 FILE2\n");
  printf(v1, v0);
  v2 = gettext("Side-by-side merge of differences between FILE1 and FILE2.");
  printf("%s\n\n", v2);
  v3 = stdout;
  v4 = gettext("Mandatory arguments to long options are mandatory for short options too.\n");
  fputs_unlocked(v4, v3);
  for ( i = (const char **)&off_11A40; *i; ++i )
  {
    if ( **i )
    {
      v5 = gettext(*i);
      printf("  %s\n", v5);
    }
    else
    {
      putchar_unlocked(10);
    }
  }
  v6 = gettext("Exit status is 0 if inputs are the same, 1 if different, 2 if trouble.");
  v7 = gettext("If a FILE is '-', read standard input.");
  printf("\n%s\n%s\n", v7, v6);
  return sub_8C7B();
}



// Function: cleanup @ 0x3df5
int cleanup()
{
  int result; // eax

  if ( pid > 0 )
    kill(pid, 13);
  result = (int)filename;
  if ( filename )
    return unlink(filename);
  return result;
}



// Function: exiterr @ 0x3e3e
void exiterr()
{
  sub_3DF5();
  sub_510D(0);
  sub_51E3();
  exit(2);
}



// Function: fatal @ 0x3e69
void fatal(const char *a1)
{
  char *v1; // rax

  v1 = gettext(a1);
  error(0, 0, off_D847, v1);
  sub_3E3E();
}



// Function: perror_fatal @ 0x3eab
void perror_fatal(long long a1)
{
  int errnum; // [rsp+1Ch] [rbp-4h]

  errnum = *__errno_location();
  sub_51E3();
  error(0, errnum, off_D847, a1);
  sub_3E3E();
}



// Function: check_child_status @ 0x3ef1
long long check_child_status(int a1, short a2, int a3, long long a4)
{
  int v4; // eax
  long long result; // rax
  const char *v6; // rax
  char *v7; // rax
  int v9; // [rsp+2Ch] [rbp-4h]

  if ( a1 || (a2 & 0x7F) != 0 )
    v4 = 0x7FFFFFFF;
  else
    v4 = HIBYTE(a2);
  v9 = v4;
  result = (unsigned int)a3;
  if ( a3 < v9 )
  {
    switch ( v9 )
    {
      case 0x7E:
        v6 = "subsidiary program '%s' could not be invoked";
        break;
      case 0x7F:
        v6 = "subsidiary program '%s' not found";
        break;
      case 0x7FFFFFFF:
        v6 = "subsidiary program '%s' failed";
        break;
      default:
        v6 = "subsidiary program '%s' failed (exit status %d)";
        break;
    }
    v7 = gettext(v6);
    error(0, a1, v7, a4, (unsigned int)v9);
    sub_3E3E();
  }
  return result;
}



// Function: ck_fopen @ 0x3fa3
FILE *ck_fopen(const char *a1, const char *a2)
{
  FILE *v3; // [rsp+18h] [rbp-8h]

  v3 = fopen(a1, a2);
  if ( !v3 )
    sub_3EAB((long long)a1);
  return v3;
}



// Function: ck_fclose @ 0x3fe7
int ck_fclose(FILE *a1)
{
  int result; // eax

  result = fclose(a1);
  if ( result )
    sub_3EAB((long long)"fclose");
  return result;
}



// Function: ck_fread @ 0x4019
size_t ck_fread(void *a1, size_t a2, FILE *a3)
{
  char *v3; // rax
  size_t v6; // [rsp+28h] [rbp-8h]

  v6 = fread_unlocked(a1, 1u, a2, a3);
  if ( !v6 && ferror_unlocked(a3) )
  {
    v3 = gettext("read failed");
    sub_3EAB((long long)v3);
  }
  return v6;
}



// Function: ck_fwrite @ 0x4082
size_t ck_fwrite(const void *a1, size_t a2, FILE *a3)
{
  size_t result; // rax
  char *v4; // rax

  result = fwrite_unlocked(a1, 1u, a2, a3);
  if ( a2 != result )
  {
    v4 = gettext("write failed");
    sub_3EAB((long long)v4);
  }
  return result;
}



// Function: ck_fflush @ 0x40d3
int ck_fflush(FILE *a1)
{
  int result; // eax
  char *v2; // rax

  result = fflush_unlocked(a1);
  if ( result )
  {
    v2 = gettext("write failed");
    sub_3EAB((long long)v2);
  }
  return result;
}



// Function: expand_name @ 0x410d
const char *expand_name(const char *a1, char a2, long long a3)
{
  bool v4; // al
  bool v6; // [rsp+2Fh] [rbp-21h]
  void *src; // [rsp+30h] [rbp-20h]
  size_t n; // [rsp+38h] [rbp-18h]
  long long v9; // [rsp+40h] [rbp-10h]
  char *dest; // [rsp+48h] [rbp-8h]

  if ( !strcmp(a1, "-") )
    sub_3E69("cannot interactively merge standard input");
  if ( a2 != 1 )
    return a1;
  src = (void *)sub_5F32(a3);
  n = strlen(a1);
  v9 = sub_5F9F(src);
  v4 = *(char *)sub_5F32(a1) && a1[n - 1] != 47;
  v6 = v4;
  dest = (char *)sub_8DC0(n + v4 + v9 + 1);
  memcpy(dest, a1, n);
  dest[n] = 47;
  memcpy(&dest[n + v6], src, v9);
  dest[v9 + n + v6] = 0;
  return dest;
}



// Function: lf_init @ 0x425e
char *lf_init(long long *a1, long long a2)
{
  char *result; // rax

  *a1 = a2;
  a1[3] = sub_8DC0(65537);
  a1[2] = a1[3];
  a1[1] = a1[2];
  result = (char *)a1[3];
  *result = 10;
  return result;
}



// Function: lf_refill @ 0x42c0
size_t lf_refill(long long a1)
{
  size_t v2; // [rsp+18h] [rbp-8h]

  v2 = sub_4019(*(void **)(a1 + 16), 0x10000u, *(FILE **)a1);
  *(long long *)(a1 + 8) = *(long long *)(a1 + 16);
  *(long long *)(a1 + 24) = v2 + *(long long *)(a1 + 16);
  **(char **)(a1 + 24) = 10;
  sub_51E3();
  return v2;
}



// Function: lf_copy @ 0x432d
size_t lf_copy(long long a1, long long a2, FILE *a3)
{
  size_t result; // rax
  const void *v6; // [rsp+28h] [rbp-8h]

  v6 = *(const void **)(a1 + 8);
  while ( a2 )
  {
    *(long long *)(a1 + 8) = rawmemchr(*(const void **)(a1 + 8), 10);
    if ( *(long long *)(a1 + 8) == *(long long *)(a1 + 24) )
    {
      sub_4082(v6, *(long long *)(a1 + 24) - (long long)v6, a3);
      result = sub_42C0(a1);
      if ( !result )
        return result;
      v6 = *(const void **)(a1 + 8);
    }
    else
    {
      --a2;
      ++*(long long *)(a1 + 8);
    }
  }
  return sub_4082(v6, *(long long *)(a1 + 8) - (long long)v6, a3);
}



// Function: lf_skip @ 0x4414
void lf_skip(long long a1, long long a2)
{
  while ( a2 )
  {
    *(long long *)(a1 + 8) = rawmemchr(*(const void **)(a1 + 8), 10);
    if ( *(long long *)(a1 + 8) == *(long long *)(a1 + 24) )
    {
      if ( !sub_42C0(a1) )
        return;
    }
    else
    {
      --a2;
      ++*(long long *)(a1 + 8);
    }
  }
}



// Function: lf_snarf @ 0x4493
long long lf_snarf(long long a1, char *a2, unsigned long long a3)
{
  void *s; // [rsp+28h] [rbp-18h]
  char *v7; // [rsp+30h] [rbp-10h]
  size_t n; // [rsp+38h] [rbp-8h]

  while ( 1 )
  {
    s = *(void **)(a1 + 8);
    v7 = rawmemchr(s, 10);
    n = v7 - (char *)s;
    if ( a3 <= v7 - (char *)s )
      return 0;
    memcpy(a2, s, n);
    if ( (unsigned long long)v7 < *(long long *)(a1 + 24) )
    {
      a2[n] = 0;
      *(long long *)(a1 + 8) = v7 + 1;
      return 1;
    }
    if ( !sub_42C0(a1) )
      break;
    a2 += n;
    a3 -= n;
  }
  if ( n )
    return 0;
  else
    return 0xFFFFFFFFLL;
}



// Function: main @ 0x4573
long long main(int a1, char **a2, char **a3)
{
  int v3; // eax
  int v5; // eax
  char v6; // [rsp+19h] [rbp-B7h]
  char v7; // [rsp+1Ah] [rbp-B6h]
  char v8; // [rsp+1Bh] [rbp-B5h]
  int stat_loc; // [rsp+1Ch] [rbp-B4h] BYREF
  int v10; // [rsp+20h] [rbp-B0h]
  int v11; // [rsp+24h] [rbp-ACh]
  char *v12; // [rsp+28h] [rbp-A8h]
  const char *v13; // [rsp+30h] [rbp-A0h]
  FILE *v14; // [rsp+38h] [rbp-98h]
  const char *v15; // [rsp+40h] [rbp-90h]
  FILE *v16; // [rsp+48h] [rbp-88h]
  FILE *v17; // [rsp+50h] [rbp-80h]
  FILE *v18; // [rsp+58h] [rbp-78h]
  long long v19[4]; // [rsp+60h] [rbp-70h] BYREF
  long long v20[4]; // [rsp+80h] [rbp-50h] BYREF
  long long v21[4]; // [rsp+A0h] [rbp-30h] BYREF
  int pipedes[2]; // [rsp+C0h] [rbp-10h] BYREF
  unsigned long long v23; // [rsp+C8h] [rbp-8h]

  v23 = __readfsqword(0x28u);
  status = 2;
  sub_632F(*a2, a2, a3);
  setlocale(6, locale);
  bindtextdomain("diffutils", "/usr/local/share/locale");
  textdomain("diffutils");
  sub_626B(sub_3DF5);
  sub_93C4();
  v12 = getenv("EDITOR");
  if ( v12 )
    file = v12;
  sub_4EBD("diff");
  while ( 1 )
  {
    v10 = getopt_long(a1, a2, "abBdEHiI:lo:stvw:WZ", &longopts, 0);
    if ( v10 == -1 )
      break;
    switch ( v10 )
    {
      case 66:
        sub_4EBD("-B");
        continue;
      case 69:
        sub_4EBD("-E");
        continue;
      case 72:
        sub_4EBD("-H");
        continue;
      case 73:
        sub_4EBD("-I");
        sub_4EBD(optarg);
        continue;
      case 87:
        sub_4EBD("-w");
        continue;
      case 90:
        sub_4EBD("-Z");
        continue;
      case 97:
        sub_4EBD("-a");
        continue;
      case 98:
        sub_4EBD("-b");
        continue;
      case 100:
        sub_4EBD("-d");
        continue;
      case 105:
        sub_4EBD("-i");
        continue;
      case 108:
        sub_4EBD("--left-column");
        continue;
      case 111:
        qword_124E8 = optarg;
        continue;
      case 115:
        byte_124F0 = 1;
        continue;
      case 116:
        sub_4EBD("-t");
        continue;
      case 118:
        v3 = sub_68F0("Thomas Lord");
        sub_8B97(
          (int)stdout,
          (unsigned int)"sdiff",
          (unsigned int)"GNU diffutils",
          (int)off_12018,
          v3,
          0,
          (char)a2);
        sub_3C7F();
        return 0;
      case 119:
        sub_4EBD("-W");
        sub_4EBD(optarg);
        continue;
      case 128:
        *argv = (char *)optarg;
        continue;
      case 129:
        sub_3CD6();
        sub_3C7F();
        return 0;
      case 130:
        sub_4EBD("--strip-trailing-cr");
        break;
      case 131:
        sub_4EBD("--tabsize");
        sub_4EBD(optarg);
        break;
      default:
        sub_3C09(0, 0);
        break;
    }
  }
  if ( a1 - optind != 2 )
  {
    if ( a1 - optind <= 1 )
      sub_3C09("missing operand after '%s'", (long long)a2[a1 - 1]);
    sub_3C09("extra operand '%s'", (long long)a2[optind + 2]);
  }
  if ( !qword_124E8 )
  {
    if ( byte_124F0 )
      sub_4EBD("--suppress-common-lines");
    sub_4EBD("-y");
    sub_4EBD("--");
    sub_4EBD(a2[optind]);
    sub_4EBD(a2[optind + 1]);
    sub_4EBD(0);
    execvp(*argv, argv);
    sub_3EAB((long long)*argv);
  }
  v6 = sub_5E22(a2[optind]);
  v7 = sub_5E22(a2[optind + 1]);
  if ( ((unsigned char)v6 & (unsigned char)v7) != 0 )
    sub_3E69("both files to be compared are directories");
  v13 = sub_410D(a2[optind], v6, (long long)a2[optind + 1]);
  v14 = sub_3FA3(v13, "r");
  v15 = sub_410D(a2[optind + 1], v7, (long long)a2[optind]);
  v16 = sub_3FA3(v15, "r");
  v17 = sub_3FA3((const char *)qword_124E8, "w");
  sub_4EBD("--sdiff-merge-assist");
  sub_4EBD("--");
  sub_4EBD(a2[optind]);
  sub_4EBD(a2[optind + 1]);
  sub_4EBD(0);
  sub_4FDD();
  if ( pipe(pipedes) )
    sub_3EAB((long long)"pipe");
  pid = fork();
  if ( pid < 0 )
    sub_3EAB((long long)"fork");
  if ( !pid )
  {
    if ( qword_12450 != 1 )
      sub_4FA3(2, 1);
    sub_4FA3(13, 0);
    close(pipedes[0]);
    if ( pipedes[1] != 1 )
    {
      dup2(pipedes[1], 1);
      close(pipedes[1]);
    }
    execvp(*argv, argv);
    if ( *__errno_location() == 2 )
      v5 = 127;
    else
      v5 = 126;
    _exit(v5);
  }
  close(pipedes[1]);
  v18 = fdopen(pipedes[0], "r");
  if ( !v18 )
    sub_3EAB((long long)"fdopen");
  sub_425E(v21, (long long)v18);
  sub_425E(v19, (long long)v14);
  sub_425E(v20, (long long)v16);
  v8 = sub_5B03(v21, v19, v13, v20, v15, v17);
  sub_3FE7(v14);
  sub_3FE7(v16);
  sub_3FE7(v17);
  v11 = 0;
  sub_3FE7(v18);
  while ( waitpid(pid, &stat_loc, 0) < 0 )
  {
    if ( *__errno_location() != 4 )
      sub_3EAB((long long)"waitpid");
    sub_51E3();
  }
  pid = 0;
  if ( filename )
  {
    unlink(filename);
    filename = 0;
  }
  if ( v8 != 1 )
    sub_3E3E();
  sub_3EF1(v11, stat_loc, 1, (long long)*argv);
  sub_510D(0);
  sub_51E3();
  exit(BYTE1(stat_loc));
}



// Function: diffarg @ 0x4ebd
char *diffarg(char *a1)
{
  long long v1; // rax

  if ( qword_12598 == qword_125A0 )
  {
    if ( qword_125A0 )
    {
      if ( (unsigned long long)qword_125A0 > 0x7FFFFFFFFFFFFFELL )
        sub_9384();
      qword_125A0 *= 2;
    }
    else
    {
      qword_125A0 = 16;
    }
    argv = (char **)sub_8E0C(argv, 8 * qword_125A0);
  }
  v1 = qword_12598++;
  argv[v1] = a1;
  return a1;
}



// Function: catchsig @ 0x4f78
long long catchsig(unsigned int a1)
{
  long long result; // rax

  if ( a1 != 2 || (result = (unsigned char)byte_124F1 ^ 1u) != 0 )
  {
    dword_124F4 = a1;
    return a1;
  }
  return result;
}



// Function: signal_handler @ 0x4fa3
int signal_handler(int a1, void (*a2)(int))
{
  act.sa_handler = a2;
  return sigaction(a1, &act, 0);
}



// Function: trapsigs @ 0x4fdd
__sighandler_t trapsigs()
{
  __sighandler_t result; // rax
  int i; // [rsp+Ch] [rbp-4h]
  int j; // [rsp+Ch] [rbp-4h]

  act.sa_flags = 0x10000000;
  sigemptyset(&act.sa_mask);
  for ( i = 0; i <= 6; ++i )
    sigaddset(&act.sa_mask, dword_D020[i]);
  for ( j = 0; j <= 6; ++j )
  {
    sigaction(dword_D020[j], 0, (struct sigaction *)&unk_120C0 + j);
    if ( *((long long *)&unk_120C0 + 19 * j) != 1 )
      sub_4FA3(dword_D020[j], (void (*)(int))sub_4F78);
  }
  result = signal(17, 0);
  byte_124F8 = 1;
  return result;
}



// Function: untrapsig @ 0x510d
int untrapsig(int a1)
{
  long long v1; // rax
  int i; // [rsp+1Ch] [rbp-4h]

  LODWORD(v1) = (unsigned char)byte_124F8;
  if ( byte_124F8 )
  {
    for ( i = 0; i <= 6; ++i )
    {
      if ( a1 )
      {
        LODWORD(v1) = dword_D020[i];
        if ( a1 != (int)v1 )
          continue;
      }
      v1 = *((long long *)&unk_120C0 + 19 * i);
      if ( v1 != 1 )
        LODWORD(v1) = sigaction(dword_D020[i], (const struct sigaction *)&unk_120C0 + i, 0);
    }
  }
  return v1;
}



// Function: checksigs @ 0x51e3
long long checksigs()
{
  long long result; // rax
  int sig; // [rsp+Ch] [rbp-4h]

  result = (unsigned int)dword_124F4;
  sig = dword_124F4;
  if ( dword_124F4 )
  {
    sub_3DF5();
    sub_510D(sig);
    raise(sig);
    exit(2);
  }
  return result;
}



// Function: give_help @ 0x5229
int give_help()
{
  char *v0; // rax

  v0 = gettext(
         "ed:\tEdit then use both versions, each decorated with a header.\n"
         "eb:\tEdit then use both versions.\n"
         "el or e1:\tEdit then use the left version.\n"
         "er or e2:\tEdit then use the right version.\n"
         "e:\tDiscard both versions then edit a new one.\n"
         "l or 1:\tUse the left version.\n"
         "r or 2:\tUse the right version.\n"
         "s:\tSilently include common lines.\n"
         "v:\tVerbosely include common lines.\n"
         "q:\tQuit.\n");
  return fprintf(stderr, off_D847, v0);
}



// Function: skip_white @ 0x5264
long long skip_white()
{
  char *v0; // rax
  unsigned int v2; // [rsp+Ch] [rbp-4h]

  while ( 1 )
  {
    v2 = getchar_unlocked();
    if ( ((*__ctype_b_loc())[v2] & 0x2000) == 0 || v2 == 10 )
      break;
    sub_51E3();
  }
  if ( ferror_unlocked(stdin) )
  {
    v0 = gettext("read failed");
    sub_3EAB((long long)v0);
  }
  return v2;
}



// Function: flush_line @ 0x52d6
int flush_line()
{
  int result; // eax
  char *v1; // rax
  int v2; // [rsp+Ch] [rbp-4h]

  do
    v2 = getchar_unlocked();
  while ( v2 != 10 && v2 != -1 );
  result = ferror_unlocked(stdin);
  if ( result )
  {
    v1 = gettext("read failed");
    sub_3EAB((long long)v1);
  }
  return result;
}



// Function: edit @ 0x5326
// bad sp value at call has been detected, the output may be wrong!
long long edit(
        long long a1,
        const char *a2,
        long long a3,
        long long a4,
        long long a5,
        const char *a6,
        long long a7,
        long long a8,
        FILE *a9)
{
  char *v9; // rax
  int v11; // eax
  char v16; // [rsp+47h] [rbp-10071h]
  int stat_loc; // [rsp+48h] [rbp-10070h] BYREF
  int v18; // [rsp+4Ch] [rbp-1006Ch]
  int v19; // [rsp+50h] [rbp-10068h]
  int fd; // [rsp+54h] [rbp-10064h]
  int v21; // [rsp+58h] [rbp-10060h]
  __pid_t pid; // [rsp+5Ch] [rbp-1005Ch]
  long long v23; // [rsp+60h] [rbp-10058h]
  long long v24; // [rsp+68h] [rbp-10050h]
  long long v25; // [rsp+70h] [rbp-10048h]
  long long v26; // [rsp+78h] [rbp-10040h]
  size_t v27; // [rsp+80h] [rbp-10038h]
  char *argv[4]; // [rsp+88h] [rbp-10030h] BYREF
  char v29[16]; // [rsp+A8h] [rbp-10010h] BYREF
  char v30; // [rsp+B8h] [rbp-10000h] BYREF
  long long v31[512]; // [rsp+F0B8h] [rbp-1000h] BYREF

  while ( v31 != (long long *)&v30 )
    ;
  v31[511] = __readfsqword(0x28u);
  while ( 2 )
  {
    while ( 2 )
    {
      v16 = 0;
      while ( v16 != 1 )
      {
        if ( putchar_unlocked(37) != 37 )
        {
          v9 = gettext("write failed");
          sub_3EAB((long long)v9);
        }
        sub_40D3(stdout);
        v18 = sub_5264();
        if ( v18 == 118 )
          goto LABEL_17;
        if ( v18 >= 116 )
          goto LABEL_33;
        if ( v18 >= 113 || v18 == 108 )
        {
LABEL_17:
          if ( (unsigned int)sub_5264() != 10 )
            goto LABEL_18;
          v16 = 1;
        }
        else
        {
          if ( v18 > 108 )
            goto LABEL_33;
          if ( v18 == 101 )
          {
            v19 = sub_5264();
            if ( v19 > 114 )
              goto LABEL_18;
            if ( v19 >= 98 )
            {
              if ( ((0x10405uLL >> ((unsigned char)v19 - 98)) & 1) != 0 )
                goto LABEL_27;
LABEL_18:
              sub_5229();
              sub_52D6();
            }
            else if ( v19 == 10 )
            {
              v16 = 1;
            }
            else
            {
              if ( v19 < 10 || (unsigned int)(v19 - 49) > 1 )
                goto LABEL_18;
LABEL_27:
              if ( (unsigned int)sub_5264() != 10 )
                goto LABEL_18;
              v16 = 1;
            }
          }
          else
          {
            if ( v18 > 50 )
              goto LABEL_33;
            if ( v18 >= 49 )
              goto LABEL_17;
            if ( v18 != -1 )
            {
              if ( v18 == 10 )
                goto LABEL_34;
              goto LABEL_33;
            }
            if ( feof_unlocked(stdin) )
            {
              v16 = 1;
              v18 = 113;
            }
            else
            {
LABEL_33:
              sub_52D6();
LABEL_34:
              sub_5229();
            }
          }
        }
      }
      if ( v18 > 118 )
        goto LABEL_95;
      if ( v18 >= 101 )
      {
        switch ( v18 )
        {
          case 'e':
            if ( filename )
            {
              stream = fopen(filename, "w");
            }
            else
            {
              fd = sub_5E9B();
              if ( fd < 0 )
                sub_3EAB((long long)"mkstemp");
              stream = fdopen(fd, "w");
            }
            if ( !stream )
              sub_3EAB((long long)filename);
            if ( v19 == 108 )
              goto LABEL_65;
            if ( v19 > 108 )
              goto LABEL_66;
            if ( v19 == 100 )
            {
              if ( a4 )
              {
                v23 = a3;
                v24 = a3 + a4 - 1;
                if ( a4 == 1 )
                  fprintf(stream, "--- %s %ld\n", a2, v23);
                else
                  fprintf(stream, "--- %s %ld,%ld\n", a2, v23, v24);
              }
            }
            else if ( v19 > 100 || v19 != 49 && v19 != 98 )
            {
LABEL_66:
              sub_4414(a1, a4);
LABEL_67:
              if ( v19 != 114 )
              {
                if ( v19 > 114 )
                {
LABEL_79:
                  sub_4414(a5, a8);
                  goto LABEL_80;
                }
                if ( v19 == 100 )
                {
                  if ( a8 )
                  {
                    v25 = a7;
                    v26 = a7 + a8 - 1;
                    if ( a8 == 1 )
                      fprintf(stream, "+++ %s %ld\n", a6, v25);
                    else
                      fprintf(stream, "+++ %s %ld,%ld\n", a6, v25, v26);
                  }
                }
                else if ( v19 > 100 || v19 != 50 && v19 != 98 )
                {
                  goto LABEL_79;
                }
              }
              sub_432D(a5, a8, stream);
LABEL_80:
              sub_3FE7(stream);
              v21 = 0;
              byte_124F1 = 1;
              sub_51E3();
              argv[0] = file;
              argv[1] = filename;
              argv[2] = 0;
              pid = fork();
              if ( !pid )
              {
                execvp(file, argv);
                if ( *__errno_location() == 2 )
                  v11 = 127;
                else
                  v11 = 126;
                _exit(v11);
              }
              if ( pid < 0 )
                sub_3EAB((long long)"fork");
              while ( waitpid(pid, &stat_loc, 0) < 0 )
              {
                if ( *__errno_location() != 4 )
                  sub_3EAB((long long)"waitpid");
                sub_51E3();
              }
              byte_124F1 = 0;
              sub_3EF1(v21, stat_loc, 0, (long long)file);
              stream = sub_3FA3(filename, "r");
              while ( 1 )
              {
                v27 = sub_4019(v29, 0x10000u, stream);
                if ( !v27 )
                  break;
                sub_51E3();
                sub_4082(v29, v27, a9);
              }
              sub_3FE7(stream);
              return 1;
            }
LABEL_65:
            sub_432D(a1, a4, stream);
            goto LABEL_67;
          case 'l':
            goto LABEL_42;
          case 'q':
            return 0;
          case 'r':
            goto LABEL_43;
          case 's':
            byte_124F0 = 1;
            continue;
          case 'v':
            byte_124F0 = 0;
            continue;
          default:
            goto LABEL_95;
        }
      }
      break;
    }
    if ( v18 == 49 )
    {
LABEL_42:
      sub_432D(a1, a4, a9);
      sub_4414(a5, a8);
      return 1;
    }
    else
    {
      if ( v18 != 50 )
      {
LABEL_95:
        sub_5229();
        continue;
      }
LABEL_43:
      sub_432D(a5, a8, a9);
      sub_4414(a1, a4);
      return 1;
    }
  }
}



// Function: interact @ 0x5b03
bool interact(long long a1, long long a2, const char *a3, long long a4, const char *a5, FILE *a6)
{
  long long v7; // rax
  int v12; // [rsp+34h] [rbp-14Ch]
  char *endptr; // [rsp+38h] [rbp-148h] BYREF
  long long v14; // [rsp+40h] [rbp-140h]
  long long i; // [rsp+48h] [rbp-138h]
  intmax_t v16; // [rsp+50h] [rbp-130h]
  intmax_t v17; // [rsp+58h] [rbp-128h]
  intmax_t v18; // [rsp+60h] [rbp-120h]
  long long v19; // [rsp+68h] [rbp-118h]
  char v20; // [rsp+70h] [rbp-110h] BYREF
  char v21[263]; // [rsp+71h] [rbp-10Fh] BYREF
  unsigned long long v22; // [rsp+178h] [rbp-8h]

  v22 = __readfsqword(0x28u);
  v14 = 1;
  for ( i = 1; ; i += v18 )
  {
    while ( 1 )
    {
      v12 = sub_4493(a1, &v20, 0x100u);
      if ( v12 <= 0 )
        return v12 != 0;
      sub_51E3();
      if ( v20 != 32 )
        break;
      puts(v21);
    }
    *__errno_location() = 0;
    v16 = strtoimax(v21, &endptr, 10);
    if ( v16 < 0 || *__errno_location() || *endptr != 44 )
      sub_3E69(&v20);
    v17 = v16;
    v16 = strtoimax(endptr + 1, &endptr, 10);
    if ( v16 < 0 || *__errno_location() || *endptr )
      sub_3E69(&v20);
    v18 = v16;
    v7 = v16;
    if ( v17 >= v16 )
      v7 = v17;
    v19 = v7;
    if ( v20 == 99 )
      break;
    if ( v20 != 105 )
      sub_3E69(&v20);
    if ( byte_124F0 )
      sub_4414(a1, v19);
    else
      sub_432D(a1, v19, stdout);
    sub_432D(a2, v17, a6);
    sub_4414(a4, v18);
LABEL_25:
    v14 += v17;
  }
  sub_432D(a1, v19, stdout);
  if ( (unsigned char)sub_5326(a2, a3, v14, v17, a4, a5, i, v18, a6) == 1 )
    goto LABEL_25;
  return 0;
}



// Function: diraccess @ 0x5e22
long long diraccess(const char *a1)
{
  struct stat buf; // [rsp+10h] [rbp-A0h] BYREF
  unsigned long long v4; // [rsp+A8h] [rbp-8h]

  v4 = __readfsqword(0x28u);
  return !stat(a1, &buf) && (buf.st_mode & 0xF000) == 0x4000;
}



// Function: temporary_file @ 0x5e9b
long long temporary_file()
{
  const char *v0; // rax
  size_t v1; // rax
  int v3; // [rsp+4h] [rbp-1Ch]
  char *v4; // [rsp+8h] [rbp-18h]
  char *s; // [rsp+10h] [rbp-10h]
  char *v6; // [rsp+18h] [rbp-8h]

  v4 = getenv("TMPDIR");
  if ( v4 )
    v0 = v4;
  else
    v0 = "/tmp";
  s = (char *)v0;
  v1 = strlen(v0);
  v6 = (char *)sub_8DC0(v1 + 13);
  sprintf(v6, "%s/sdiffXXXXXX", s);
  v3 = mkstemp(v6);
  if ( v3 >= 0 )
    filename = v6;
  return (unsigned int)v3;
}


