// Function: main @ 0x3b60
long long main(int a1, char **a2, char **a3)
{
  char *v4; // rdi
  char *v5; // rax
  int v6; // eax
  int v8; // eax
  long long v9; // rax
  char v10; // bp
  char v11; // al
  long long v12; // r13
  long long v13; // r12
  long long v14; // r14
  long long v15; // rbp
  FILE *v16; // rax
  FILE *v17; // r15
  char v18; // bl
  char **v19; // rbx
  int *v20; // rax
  long long v21; // [rsp+8h] [rbp-D0h]
  int stat_loc; // [rsp+2Ch] [rbp-ACh] BYREF
  char v23[32]; // [rsp+30h] [rbp-A8h] BYREF
  char v24[32]; // [rsp+50h] [rbp-88h] BYREF
  char v25[32]; // [rsp+70h] [rbp-68h] BYREF
  int pipedes[2]; // [rsp+90h] [rbp-48h] BYREF
  unsigned long long v27; // [rsp+98h] [rbp-40h]

  v4 = *a2;
  v27 = __readfsqword(0x28u);
  status = 2;
  sub_5960(v4);
  setlocale(6, "");
  bindtextdomain("diffutils", "/usr/local/share/locale");
  textdomain("diffutils");
  sub_58A0(sub_4300);
  sub_7BF0();
  v5 = getenv("EDITOR");
  if ( v5 )
    file = v5;
  sub_45B0("diff");
  while ( 2 )
  {
    v6 = getopt_long(a1, a2, "abBdEHiI:lo:stvw:WZ", &longopts, 0);
    if ( v6 == -1 )
    {
      v9 = optind;
      if ( a1 - optind != 2 )
      {
        if ( a1 - optind <= 1 )
        {
          sub_44C0("missing operand after '%s'");
LABEL_47:
          sub_44C0(0);
          goto LABEL_48;
        }
        v9 = sub_44C0("extra operand '%s'");
      }
      if ( !qword_F168 )
      {
        if ( byte_F161 )
          sub_45B0("--suppress-common-lines");
        sub_45B0("-y");
        sub_45B0("--");
        sub_45B0(a2[optind]);
        sub_45B0(a2[optind + 1]);
        sub_45B0(0);
        v19 = argv;
        execvp(*argv, argv);
        sub_49E0(*v19);
      }
      v10 = sub_4520(a2[v9]);
      v11 = sub_4520(a2[optind + 1]);
      if ( v10 && v11 )
        sub_48D0("both files to be compared are directories");
      v12 = sub_4900(a2[optind]);
      v13 = sub_4A60(v12, "r");
      v14 = sub_4900(a2[optind + 1]);
      v15 = sub_4A60(v14, "r");
      v21 = sub_4A60(qword_F168, "w");
      sub_45B0("--sdiff-merge-assist");
      sub_45B0("--");
      sub_45B0(a2[optind]);
      sub_45B0(a2[optind + 1]);
      sub_45B0(0);
      sub_46F0();
      if ( pipe(pipedes) )
        sub_49E0("pipe");
      pid = fork();
      if ( pid < 0 )
        sub_49E0("fork");
      if ( !pid )
      {
        if ( qword_F510 != 1 )
          sub_4640(2, 1);
        sub_4640(13, 0);
        close(pipedes[0]);
        if ( pipedes[1] != 1 )
        {
          dup2(pipedes[1], 1);
          close(pipedes[1]);
        }
        execvp(*argv, argv);
        v20 = __errno_location();
        _exit((*v20 == 2) + 126);
      }
      close(pipedes[1]);
      v16 = fdopen(pipedes[0], "r");
      v17 = v16;
      if ( !v16 )
        sub_49E0("fdopen");
      sub_4580(v25, v16);
      sub_4580(v23, v13);
      sub_4580(v24, v15);
      v18 = sub_5480(v25, v23, v12, v24, v14, v21);
      sub_4B40(v13);
      sub_4B40(v15);
      sub_4B40(v21);
      sub_4B40(v17);
      while ( waitpid(pid, &stat_loc, 0) < 0 )
      {
        if ( *__errno_location() != 4 )
          sub_49E0("waitpid");
        sub_4870();
      }
      pid = 0;
      if ( filename )
      {
        unlink(filename);
        filename = 0;
      }
      if ( !v18 )
        sub_48A0();
LABEL_48:
      sub_4E40((unsigned int)stat_loc, 1, *argv);
      sub_4660(0);
      sub_4870();
      exit(BYTE1(stat_loc));
    }
    switch ( v6 )
    {
      case 66:
        sub_45B0("-B");
        continue;
      case 69:
        sub_45B0("-E");
        continue;
      case 72:
        sub_45B0("-H");
        continue;
      case 73:
        sub_45B0("-I");
        sub_45B0(optarg);
        continue;
      case 87:
        sub_45B0("-w");
        continue;
      case 90:
        sub_45B0("-Z");
        continue;
      case 97:
        sub_45B0("-a");
        continue;
      case 98:
        sub_45B0("-b");
        continue;
      case 100:
        sub_45B0("-d");
        continue;
      case 105:
        sub_45B0("-i");
        continue;
      case 108:
        sub_45B0("--left-column");
        continue;
      case 111:
        qword_F168 = optarg;
        continue;
      case 115:
        byte_F161 = 1;
        continue;
      case 116:
        sub_45B0("-t");
        continue;
      case 118:
        v8 = sub_5C70("Thomas Lord");
        sub_7690((int)stdout, (unsigned int)"sdiff", (unsigned int)"GNU diffutils", (int)off_F018, v8, 0);
        sub_4A10();
        return 0;
      case 119:
        sub_45B0("-W");
        sub_45B0(optarg);
        continue;
      case 128:
        *argv = (char *)optarg;
        continue;
      case 129:
        sub_4380();
        sub_4A10();
        return 0;
      case 130:
        sub_45B0("--strip-trailing-cr");
        continue;
      case 131:
        sub_45B0("--tabsize");
        sub_45B0(optarg);
        continue;
      default:
        goto LABEL_47;
    }
  }
}



// Function: catchsig @ 0x42e0
long long catchsig(int a1)
{
  long long result; // rax

  if ( a1 != 2 || (result = (unsigned char)byte_F160, !byte_F160) )
    sig = a1;
  return result;
}



// Function: cleanup @ 0x4300
int cleanup()
{
  int result; // eax

  if ( pid > 0 )
  {
    kill(pid, 13);
    result = (int)filename;
    if ( filename )
      return unlink(filename);
  }
  else
  {
    result = (int)filename;
    if ( filename )
      return unlink(filename);
  }
  return result;
}



// Function: usage @ 0x4380
long long usage()
{
  long long v0; // r12
  const char **v1; // rbx
  char *v2; // rax
  char *v3; // rax
  FILE *v4; // rbp
  char *v5; // rax
  char v6; // al
  const char *v7; // rsi
  char *v8; // rax
  char *v9; // r12
  char *v10; // rax

  v0 = qword_1F620;
  v1 = (const char **)&off_E780;
  v2 = dcgettext(0, "Usage: %s [OPTION]... FILE1 FILE2\n", 5);
  __printf_chk(1, v2, v0);
  v3 = dcgettext(0, "Side-by-side merge of differences between FILE1 and FILE2.", 5);
  __printf_chk(1, "%s\n\n", v3);
  v4 = stdout;
  v5 = dcgettext(0, "Mandatory arguments to long options are mandatory for short options too.\n", 5);
  fputs_unlocked(v5, v4);
  v6 = 45;
  v7 = "-o, --output=FILE            operate interactively, sending output to FILE";
  while ( v6 )
  {
    ++v1;
    v8 = dcgettext(0, v7, 5);
    __printf_chk(1, "  %s\n", v8);
    v7 = *v1;
    if ( !*v1 )
      goto LABEL_6;
LABEL_3:
    v6 = *v7;
  }
  ++v1;
  putchar_unlocked(10);
  v7 = *v1;
  if ( *v1 )
    goto LABEL_3;
LABEL_6:
  v9 = dcgettext(0, "Exit status is 0 if inputs are the same, 1 if different, 2 if trouble.", 5);
  v10 = dcgettext(0, "If a FILE is '-', read standard input.", 5);
  __printf_chk(1, "\n%s\n%s\n", v10, v9);
  return sub_7740();
}



// Function: try_help @ 0x44c0
long long try_help(char *msgid, long long a2)
{
  char *v2; // rax
  long long v3; // r12
  char *v4; // rax

  if ( msgid )
  {
    v2 = dcgettext(0, msgid, 5);
    error(0, 0, v2, a2);
  }
  v3 = qword_1F620;
  v4 = dcgettext(0, "Try '%s --help' for more information.", 5);
  error(2, 0, v4, v3);
  return sub_4520(2);
}



// Function: diraccess @ 0x4520
bool diraccess(const char *a1)
{
  int v1; // r8d
  bool result; // al
  struct stat v3; // [rsp+0h] [rbp-A8h] BYREF
  unsigned long long v4; // [rsp+98h] [rbp-10h]

  v4 = __readfsqword(0x28u);
  v1 = stat(a1, &v3);
  result = 0;
  if ( !v1 )
    return (v3.st_mode & 0xF000) == 0x4000;
  return result;
}



// Function: lf_init @ 0x4580
char *lf_init(long long *a1, long long a2)
{
  char *result; // rax

  *a1 = a2;
  result = (char *)sub_7800(&unk_10001);
  a1[3] = result;
  a1[2] = result;
  a1[1] = result;
  *result = 10;
  return result;
}



// Function: diffarg @ 0x45b0
char **diffarg(char *a1)
{
  long long v1; // rsi
  char **result; // rax
  long long v3; // rax
  long long v4; // rsi

  v1 = qword_F0A8;
  result = argv;
  if ( qword_F0A8 == qword_F0A0 )
  {
    if ( qword_F0A8 )
    {
      if ( (unsigned long long)qword_F0A8 > 0x7FFFFFFFFFFFFFELL )
        sub_7BB0();
      v3 = 2 * qword_F0A8;
      v4 = 16 * qword_F0A8;
    }
    else
    {
      v3 = 16;
      v4 = 128;
    }
    qword_F0A0 = v3;
    result = (char **)sub_7850(argv, v4);
    v1 = qword_F0A8;
    argv = result;
  }
  result[v1] = a1;
  qword_F0A8 = v1 + 1;
  return result;
}



// Function: signal_handler @ 0x4640
int signal_handler(int a1, void (*a2)(int))
{
  act.sa_handler = a2;
  return sigaction(a1, &act, 0);
}



// Function: untrapsig @ 0x4660
int untrapsig(int a1)
{
  int *v2; // rbp
  __pid_t *v3; // rbx
  int v4; // edi
  const struct sigaction *v5; // rsi
  int result; // eax

  if ( byte_F158 )
  {
    v2 = (int *)&unk_AD30;
    v3 = (__pid_t *)&oact;
    do
    {
      while ( a1 && *v2 != a1 || *(long long *)v3 == 1 )
      {
        v3 += 38;
        ++v2;
        if ( v3 == &pid )
          return result;
      }
      v4 = *v2;
      v5 = (const struct sigaction *)v3;
      v3 += 38;
      ++v2;
      result = sigaction(v4, v5, 0);
    }
    while ( v3 != &pid );
  }
  return result;
}



// Function: trapsigs @ 0x46f0
__sighandler_t trapsigs()
{
  int *v0; // rbp
  int *v1; // rbx
  int i; // esi
  struct sigaction *v3; // rbx
  int j; // r12d
  __sighandler_t result; // rax

  v0 = (int *)&unk_AD34;
  v1 = (int *)&unk_AD34;
  act.sa_flags = 0x10000000;
  sigemptyset(&act.sa_mask);
  for ( i = 1; ; i = *v1++ )
  {
    sigaddset(&act.sa_mask, i);
    if ( v1 == (int *)&unk_AD4C )
      break;
  }
  v3 = &oact;
  for ( j = 1; ; j = *v0++ )
  {
    sigaction(j, 0, v3);
    if ( v3->sa_handler != (__sighandler_t)((char *)&dword_0 + 1) )
      sub_4640(j, (void (*)(int))sub_42E0);
    if ( ++v3 == (struct sigaction *)&pid )
      break;
  }
  result = signal(17, 0);
  byte_F158 = 1;
  return result;
}



// Function: temporary_file @ 0x47b0
int temporary_file()
{
  char *v0; // rbx
  size_t v1; // rax
  char *v2; // rbp
  int result; // eax

  v0 = getenv("TMPDIR");
  if ( !v0 )
    v0 = "/tmp";
  v1 = strlen(v0);
  v2 = (char *)sub_7800(v1 + 13);
  __sprintf_chk(v2, 1, -1, "%s/sdiffXXXXXX", v0);
  result = mkstemp(v2);
  if ( result >= 0 )
    filename = v2;
  return result;
}



// Function: give_help @ 0x4830
long long give_help()
{
  char *v0; // rax

  v0 = dcgettext(
         0,
         "ed:\tEdit then use both versions, each decorated with a header.\n"
         "eb:\tEdit then use both versions.\n"
         "el or e1:\tEdit then use the left version.\n"
         "er or e2:\tEdit then use the right version.\n"
         "e:\tDiscard both versions then edit a new one.\n"
         "l or 1:\tUse the left version.\n"
         "r or 2:\tUse the right version.\n"
         "s:\tSilently include common lines.\n"
         "v:\tVerbosely include common lines.\n"
         "q:\tQuit.\n",
         5);
  return __fprintf_chk(stderr, 1, "%s", v0);
}



// Function: checksigs @ 0x4870
void checksigs()
{
  int v0; // ebp

  v0 = sig;
  if ( sig )
  {
    sub_4300();
    sub_4660(v0);
    raise(v0);
    exit(2);
  }
}



// Function: exiterr @ 0x48a0
void exiterr()
{
  sub_4300();
  sub_4660(0);
  sub_4870();
  exit(2);
}



// Function: fatal @ 0x48d0
void fatal(char *msgid)
{
  char *v1; // rax

  v1 = dcgettext(0, msgid, 5);
  error(0, 0, "%s", v1);
  sub_48A0();
}



// Function: expand_name @ 0x4900
char *expand_name(const char *src, char a2, long long a3)
{
  char *v4; // r12
  long long v5; // r15
  const void *v6; // r13
  size_t v7; // rbx
  size_t v8; // r14
  size_t v9; // r15

  if ( !strcmp(src, "-") )
    sub_48D0("cannot interactively merge standard input");
  v4 = (char *)src;
  if ( a2 )
  {
    v5 = 0;
    v6 = (const void *)sub_5690(a3);
    v7 = strlen(src);
    v8 = sub_56F0(v6);
    if ( *(char *)sub_5690(src) )
      v5 = src[v7 - 1] != 47;
    v9 = v7 + v5;
    v4 = (char *)sub_7800(v9 + v8 + 1);
    memcpy(v4, src, v7);
    v4[v7] = 47;
    memcpy(&v4[v9], v6, v8);
    v4[v9 + v8] = 0;
  }
  return v4;
}



// Function: perror_fatal @ 0x49e0
void perror_fatal(const char *a1)
{
  int v1; // ebp

  v1 = *__errno_location();
  sub_4870();
  error(0, v1, "%s", a1);
  sub_48A0();
}



// Function: check_stdout @ 0x4a10
int check_stdout()
{
  int result; // eax
  char *v1; // rax

  if ( ferror_unlocked(stdout) )
    sub_48D0("write failed");
  result = fclose(stdout);
  if ( result )
  {
    v1 = dcgettext(0, "standard output", 5);
    sub_49E0(v1);
  }
  return result;
}



// Function: ck_fopen @ 0x4a60
FILE *ck_fopen(const char *a1, const char *a2)
{
  FILE *result; // rax

  result = fopen(a1, a2);
  if ( !result )
    sub_49E0(a1);
  return result;
}



// Function: ck_fwrite @ 0x4a80
size_t ck_fwrite(const void *a1, size_t a2, FILE *a3)
{
  size_t result; // rax
  char *v4; // rax

  result = fwrite_unlocked(a1, 1u, a2, a3);
  if ( result != a2 )
  {
    v4 = dcgettext(0, "write failed", 5);
    sub_49E0(v4);
  }
  return result;
}



// Function: ck_fflush @ 0x4ac0
int ck_fflush(FILE *a1)
{
  int result; // eax
  char *v2; // rax

  result = fflush_unlocked(a1);
  if ( result )
  {
    v2 = dcgettext(0, "write failed", 5);
    sub_49E0(v2);
  }
  return result;
}



// Function: flush_line @ 0x4af0
int flush_line()
{
  int v0; // eax
  int result; // eax
  char *v2; // rax

  do
    v0 = getchar_unlocked();
  while ( v0 != -1 && v0 != 10 );
  result = ferror_unlocked(stdin);
  if ( result )
  {
    v2 = dcgettext(0, "read failed", 5);
    sub_49E0(v2);
  }
  return result;
}



// Function: ck_fclose @ 0x4b40
int ck_fclose(FILE *a1)
{
  int result; // eax

  result = fclose(a1);
  if ( result )
    sub_49E0("fclose");
  return result;
}



// Function: skip_white @ 0x4b60
long long skip_white()
{
  const unsigned short **v0; // rbx
  int v1; // edx
  unsigned int v2; // r12d
  char *v4; // rax

  v0 = __ctype_b_loc();
  while ( 1 )
  {
    v1 = getchar_unlocked();
    v2 = v1;
    if ( ((*v0)[v1] & 0x2000) == 0 || v1 == 10 )
      break;
    sub_4870();
  }
  if ( ferror_unlocked(stdin) )
  {
    v4 = dcgettext(0, "read failed", 5);
    sub_49E0(v4);
  }
  return v2;
}



// Function: lf_refill @ 0x4c30
long long lf_refill(long long *a1)
{
  long long v1; // rax
  long long v2; // rdx
  long long v3; // r12
  char *v4; // rdx

  v1 = sub_4BD0(a1[2], *a1);
  v2 = a1[2];
  v3 = v1;
  a1[1] = v2;
  v4 = (char *)(v1 + v2);
  a1[3] = v4;
  *v4 = 10;
  sub_4870();
  return v3;
}



// Function: lf_skip @ 0x4c70
void lf_skip(long long a1, long long a2)
{
  long long v2; // rbp
  char *v3; // rax

  v2 = a2;
  if ( a2 )
  {
    do
    {
      while ( 1 )
      {
        v3 = (char *)rawmemchr(*(const void **)(a1 + 8), 10);
        *(long long *)(a1 + 8) = v3;
        if ( v3 == *(char **)(a1 + 24) )
          break;
        *(long long *)(a1 + 8) = v3 + 1;
        if ( !--v2 )
          return;
      }
    }
    while ( sub_4C30((long long *)a1) );
  }
}



// Function: lf_copy @ 0x4cd0
size_t lf_copy(long long *a1, long long a2, FILE *a3)
{
  const void *v5; // rdi
  char *v6; // r12
  long long v7; // rbx
  const void *v8; // r13
  char *v9; // rax
  size_t v10; // rsi
  size_t result; // rax

  v5 = (const void *)a1[1];
  if ( a2 )
  {
    v6 = (char *)a1[3];
    v7 = a2;
    v8 = v5;
    do
    {
      while ( 1 )
      {
        v9 = (char *)rawmemchr(v5, 10);
        a1[1] = v9;
        if ( v9 != v6 )
          break;
        sub_4A80(v8, v6 - (char *)v8, a3);
        result = sub_4C30(a1);
        if ( !result )
          return result;
        v8 = (const void *)a1[1];
        v6 = (char *)a1[3];
        v5 = v8;
      }
      v5 = v9 + 1;
      a1[1] = v9 + 1;
      --v7;
    }
    while ( v7 );
    v10 = (char *)v5 - (char *)v8;
  }
  else
  {
    v8 = v5;
    v10 = 0;
  }
  return sub_4A80(v8, v10, a3);
}



// Function: edit @ 0x4ec0
// positive sp value has been detected, the output may be wrong!
long long edit(
        long long *a1,
        const char *a2,
        long long a3,
        long long a4,
        long long *a5,
        const char *a6,
        long long a7,
        long long a8,
        FILE *a9)
{
  void *v9; // rbp
  FILE *v12; // rdi
  int v13; // eax
  long long v14; // rdx
  char *v15; // rax
  int v17; // eax
  int v18; // ebx
  FILE *v19; // rax
  __pid_t v20; // eax
  __pid_t v21; // ebx
  long long i; // rsi
  size_t v23; // rbp
  unsigned long long v24; // rax
  int v25; // edi
  int *v26; // rax
  int v27; // [rsp-6Ch] [rbp-1009Ch]
  long long *v28; // [rsp-68h] [rbp-10098h]
  long long v29; // [rsp-58h] [rbp-10088h]
  const char *v30; // [rsp-50h] [rbp-10080h]
  unsigned int v31; // [rsp-3Ch] [rbp-1006Ch] BYREF
  char *v32[4]; // [rsp-38h] [rbp-10068h] BYREF
  char v33[24]; // [rsp-18h] [rbp-10048h] BYREF
  char v34; // [rsp+0h] [rbp-10030h] BYREF
  long long v35; // [rsp+1000h] [rbp-F030h] BYREF
  unsigned long long v36; // [rsp+FFF0h] [rbp-40h]

  while ( &v34 != (char *)(&v35 - 0x2000) )
    ;
  v9 = &unk_10405;
  v28 = a1;
  v29 = a3;
  v30 = a6;
  v36 = __readfsqword(0x28u);
  while ( 2 )
  {
    if ( putchar_unlocked(37) != 37 )
    {
LABEL_9:
      v15 = dcgettext(0, "write failed", 5);
      sub_49E0(v15);
    }
    while ( 1 )
    {
      v12 = stdout;
      sub_4AC0(stdout);
      v13 = sub_4B60();
      v14 = (unsigned int)v13;
      if ( v13 > 50 )
        break;
      if ( v13 > 48 )
        goto LABEL_19;
      if ( v13 == -1 )
      {
        v12 = stdin;
        if ( feof_unlocked(stdin) )
          return 0;
LABEL_12:
        sub_4AF0();
        goto LABEL_8;
      }
      if ( v13 != 10 )
        goto LABEL_12;
LABEL_8:
      ((void (*)(FILE *, const char *, long long))sub_4830)(v12, a2, v14);
      if ( putchar_unlocked(37) != 37 )
        goto LABEL_9;
    }
    if ( v13 != 101 )
    {
      if ( v13 > 100 )
      {
        if ( v13 > 115 )
        {
          if ( v13 == 118 )
          {
LABEL_19:
            v27 = v13;
            if ( (unsigned int)sub_4B60() != 10 )
              goto LABEL_17;
            if ( v27 > 100 )
            {
              switch ( v27 )
              {
                case 'q':
                  return 0;
                case 'r':
                  goto LABEL_54;
                case 's':
                  byte_F161 = 1;
                  continue;
                case 'v':
                  byte_F161 = 0;
                  continue;
              }
            }
            if ( v27 != 49 )
            {
LABEL_54:
              sub_4CD0(a5, a8, a9);
              sub_4C70((long long)v28, a4);
              return 1;
            }
            goto LABEL_57;
          }
        }
        else
        {
          if ( v13 > 112 )
            goto LABEL_19;
          if ( v13 == 108 )
          {
            if ( (unsigned int)((long long (*)(FILE *, const char *, long long))sub_4B60)(v12, a2, 108) != 10 )
            {
LABEL_17:
              sub_4830();
              sub_4AF0();
              continue;
            }
LABEL_57:
            sub_4CD0(v28, a4, a9);
            sub_4C70((long long)a5, a8);
            return 1;
          }
        }
      }
      goto LABEL_12;
    }
    break;
  }
  v17 = ((long long (*)(FILE *, const char *, long long))sub_4B60)(v12, a2, 101);
  v18 = v17;
  if ( v17 > 50 )
  {
    v24 = (unsigned int)(v17 - 98);
    if ( (unsigned int)v24 > 0x10 || !_bittest64((const long long *)&v9, v24) )
      goto LABEL_17;
LABEL_51:
    if ( (unsigned int)sub_4B60() != 10 )
      goto LABEL_17;
    goto LABEL_27;
  }
  if ( v17 > 48 )
    goto LABEL_51;
  if ( v17 != 10 )
    goto LABEL_17;
LABEL_27:
  if ( filename )
  {
    v19 = fopen(filename, "w");
  }
  else
  {
    v25 = sub_47B0();
    if ( v25 < 0 )
      sub_49E0("mkstemp");
    v19 = fdopen(v25, "w");
  }
  qword_F5B0 = (long long)v19;
  if ( !v19 )
    sub_49E0(filename);
  if ( v18 == 100 )
  {
    if ( a4 )
    {
      if ( a4 == 1 )
        __fprintf_chk(v19, 1, "--- %s %ld\n", a2, v29);
      else
        __fprintf_chk(v19, 1, "--- %s %ld,%ld\n", a2, v29, v29 + a4 - 1);
      v19 = (FILE *)qword_F5B0;
    }
    sub_4CD0(v28, a4, v19);
    if ( a8 )
    {
      if ( a8 == 1 )
        __fprintf_chk(qword_F5B0, 1, "+++ %s %ld\n", v30, a7);
      else
        __fprintf_chk(qword_F5B0, 1, "+++ %s %ld,%ld\n", v30, a7, a7 + a8 - 1);
    }
LABEL_38:
    sub_4CD0(a5, a8, (FILE *)qword_F5B0);
  }
  else
  {
    if ( v18 > 100 )
    {
      if ( v18 == 108 )
        goto LABEL_34;
    }
    else if ( v18 == 49 || v18 == 98 )
    {
LABEL_34:
      sub_4CD0(v28, a4, v19);
      goto LABEL_35;
    }
    sub_4C70((long long)v28, a4);
LABEL_35:
    if ( v18 > 100 )
    {
      if ( v18 == 114 )
        goto LABEL_38;
    }
    else if ( v18 == 50 || v18 == 98 )
    {
      goto LABEL_38;
    }
    sub_4C70((long long)a5, a8);
  }
  sub_4B40((FILE *)qword_F5B0);
  byte_F160 = 1;
  sub_4870();
  v32[2] = 0;
  v32[0] = file;
  v32[1] = filename;
  v20 = fork();
  v21 = v20;
  if ( !v20 )
  {
    execvp(file, v32);
    v26 = __errno_location();
    _exit((*v26 == 2) + 126);
  }
  if ( v20 < 0 )
    sub_49E0("fork");
  while ( waitpid(v21, (int *)&v31, 0) < 0 )
  {
    if ( *__errno_location() != 4 )
      sub_49E0("waitpid");
    sub_4870();
  }
  byte_F160 = 0;
  sub_4E40(v31, 0, file);
  qword_F5B0 = (long long)sub_4A60(filename, "r");
  for ( i = qword_F5B0; ; i = qword_F5B0 )
  {
    v23 = sub_4BD0(v33, i);
    if ( !v23 )
      break;
    sub_4870();
    sub_4A80(v33, v23, a9);
  }
  sub_4B40((FILE *)qword_F5B0);
  return 1;
}



// Function: interact @ 0x5480
char interact(long long *a1, long long *a2, const char *a3, long long *a4, const char *a5, FILE *a6)
{
  int v6; // eax
  bool v7; // zf
  int *v8; // rax
  int *v9; // rbx
  intmax_t v10; // r12
  intmax_t v11; // r13
  long long v12; // rsi
  char result; // al
  int v14; // eax
  long long v15; // [rsp+8h] [rbp-190h]
  long long v16; // [rsp+10h] [rbp-188h]
  char *endptr; // [rsp+48h] [rbp-150h] BYREF
  char msgid; // [rsp+50h] [rbp-148h] BYREF
  char nptr[263]; // [rsp+51h] [rbp-147h] BYREF
  unsigned long long v25; // [rsp+158h] [rbp-40h]

  v25 = __readfsqword(0x28u);
  v16 = 1;
  v15 = 1;
  while ( 1 )
  {
    v6 = sub_4D80(a1, &msgid);
    v7 = v6 == 0;
    if ( v6 <= 0 )
      return !v7;
    while ( 1 )
    {
      sub_4870();
      if ( msgid == 32 )
        break;
      v8 = __errno_location();
      *v8 = 0;
      v9 = v8;
      v10 = strtoimax(nptr, &endptr, 10);
      if ( v10 < 0 || *v9 || *endptr != 44 || (v11 = strtoimax(endptr + 1, &endptr, 10), v11 < 0) || *v9 || *endptr )
LABEL_23:
        sub_48D0(&msgid);
      v12 = v11;
      if ( v10 >= v11 )
        v12 = v10;
      if ( msgid == 99 )
      {
        sub_4CD0(a1, v12, stdout);
        result = sub_4EC0(a2, a3, v15, v10, a4, a5, v16, v11, a6);
        if ( !result )
          return result;
      }
      else
      {
        if ( msgid != 105 )
          goto LABEL_23;
        if ( byte_F161 )
          sub_4C70((long long)a1, v12);
        else
          sub_4CD0(a1, v12, stdout);
        sub_4CD0(a2, v10, a6);
        sub_4C70((long long)a4, v11);
      }
      v15 += v10;
      v16 += v11;
      v14 = sub_4D80(a1, &msgid);
      v7 = v14 == 0;
      if ( v14 <= 0 )
        return !v7;
    }
    puts(nptr);
  }
}


