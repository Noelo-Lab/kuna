// Function: main @ 0x3b00
long long main(int a1, char **a2, char **a3)
{
  const char *v3; // rbp
  char **v4; // rbx
  char *v5; // rdi
  char *v6; // rax
  int v7; // eax
  long long v8; // r12
  char *v9; // rax
  char *v10; // rax
  FILE *v11; // rbp
  char *v12; // rax
  FILE *v13; // rsi
  const char *v14; // rsi
  _IO_FILE *v15; // rdi
  char *IO_write_ptr; // rax
  char *v17; // rax
  char *v18; // r12
  char *v19; // rax
  const char *v20; // rdx
  const char *v21; // rcx
  FILE *v22; // rdi
  char v24; // bp
  int *v25; // rbx
  int *v26; // r15
  int v27; // esi
  struct sigaction *v28; // r14
  int v29; // r15d
  char *v30; // rax
  char *v31; // r15
  char *v32; // rax
  char *v33; // rbx
  unsigned long long j; // rbp
  long long v35; // rax
  unsigned int v36; // r9d
  char *v37; // rax
  char *v38; // r13
  signed long long v39; // r14
  unsigned int v40; // ebp
  int *v41; // rax
  int *v42; // r14
  intmax_t v43; // rbx
  intmax_t v44; // r15
  intmax_t v45; // rsi
  char v46; // bl
  int *v47; // rax
  char **v48; // rbx
  char dest; // [rsp+0h] [rbp-218h]
  unsigned int desta; // [rsp+0h] [rbp-218h]
  unsigned int destb; // [rsp+0h] [rbp-218h]
  unsigned int destc; // [rsp+0h] [rbp-218h]
  long long i; // [rsp+8h] [rbp-210h]
  long long v54; // [rsp+10h] [rbp-208h]
  long long v55; // [rsp+18h] [rbp-200h]
  char v56; // [rsp+27h] [rbp-1F1h]
  long long v57; // [rsp+28h] [rbp-1F0h]
  long long v58; // [rsp+30h] [rbp-1E8h]
  long long v59; // [rsp+38h] [rbp-1E0h]
  long long v60; // [rsp+40h] [rbp-1D8h]
  FILE *v61; // [rsp+48h] [rbp-1D0h]
  char *endptr; // [rsp+58h] [rbp-1C0h] BYREF
  long long v63[4]; // [rsp+60h] [rbp-1B8h] BYREF
  long long v64[3]; // [rsp+80h] [rbp-198h] BYREF
  char *v65; // [rsp+98h] [rbp-180h]
  FILE *v66; // [rsp+A0h] [rbp-178h] BYREF
  char *v67; // [rsp+A8h] [rbp-170h]
  char *v68; // [rsp+B0h] [rbp-168h]
  char *v69; // [rsp+B8h] [rbp-160h]
  int pipedes[2]; // [rsp+C8h] [rbp-150h] BYREF
  char v71; // [rsp+D0h] [rbp-148h] BYREF
  char nptr[263]; // [rsp+D1h] [rbp-147h] BYREF
  unsigned long long v73; // [rsp+1D8h] [rbp-40h]

  v3 = (const char *)a1;
  v4 = a2;
  v5 = *a2;
  v73 = __readfsqword(0x28u);
  status = 2;
  sub_5B80(v5);
  setlocale(6, "");
  bindtextdomain("diffutils", "/usr/local/share/locale");
  textdomain("diffutils");
  sub_5AC0(sub_4990);
  sub_82B0();
  v6 = getenv("EDITOR");
  if ( v6 )
    file = v6;
  sub_4900("diff");
  while ( 2 )
  {
    v7 = getopt_long((int)v3, a2, "abBdEHiI:lo:stvw:WZ", &longopts, 0);
    if ( v7 != -1 )
    {
      switch ( v7 )
      {
        case 66:
          sub_4900("-B");
          continue;
        case 69:
          sub_4900("-E");
          continue;
        case 72:
          sub_4900("-H");
          continue;
        case 73:
          sub_4900("-I");
          sub_4900(optarg);
          continue;
        case 87:
          sub_4900("-w");
          continue;
        case 90:
          sub_4900("-Z");
          continue;
        case 97:
          sub_4900("-a");
          continue;
        case 98:
          sub_4900("-b");
          continue;
        case 100:
          sub_4900("-d");
          continue;
        case 105:
          sub_4900("-i");
          continue;
        case 108:
          sub_4900("--left-column");
          continue;
        case 111:
          qword_10168 = optarg;
          continue;
        case 115:
          byte_10161 = 1;
          continue;
        case 116:
          sub_4900("-t");
          continue;
        case 118:
          sub_6020("Thomas Lord");
          v22 = stdout;
          sub_7BF0(stdout, dest);
          sub_4CC0(v22, "sdiff");
          return 0;
        case 119:
          sub_4900("-W");
          sub_4900(optarg);
          continue;
        case 128:
          *argv = (char *)optarg;
          continue;
        case 129:
          v8 = qword_20620;
          v9 = dcgettext(0, "Usage: %s [OPTION]... FILE1 FILE2\n", 5);
          v4 = &off_F7A0;
          __printf_chk(1, v9, v8);
          v10 = dcgettext(0, "Side-by-side merge of differences between FILE1 and FILE2.", 5);
          __printf_chk(1, "%s\n\n", v10);
          v11 = stdout;
          v12 = dcgettext(0, "Mandatory arguments to long options are mandatory for short options too.\n", 5);
          v13 = v11;
          v3 = "  %s\n";
          fputs_unlocked(v12, v13);
          v14 = "-o, --output=FILE            operate interactively, sending output to FILE";
          goto LABEL_12;
        case 130:
          sub_4900("--strip-trailing-cr");
          continue;
        case 131:
          sub_4900("--tabsize");
          sub_4900(optarg);
          continue;
        default:
          goto LABEL_89;
      }
    }
    break;
  }
  if ( (int)v3 - optind == 2 )
  {
    if ( !qword_10168 )
    {
      if ( byte_10161 )
        sub_4900((char *)&stru_BAB5._IO_read_end + 3);
      sub_4900((char *)&stru_BAB5._IO_write_ptr + 3);
      sub_4900((char *)&stru_BAB5._IO_write_ptr + 6);
      sub_4900(a2[optind]);
      sub_4900(a2[optind + 1]);
      sub_4900(0);
      v48 = argv;
      execvp(*argv, argv);
      sub_4B30(*v48);
    }
    v24 = sub_48A0(a2[optind]);
    v56 = sub_48A0(a2[optind + 1]) & v24;
    if ( v56 )
      sub_4B00("both files to be compared are directories");
    v57 = sub_4BE0(a2[optind]);
    v59 = sub_4B60(v57, "r");
    v58 = sub_4BE0(a2[optind + 1]);
    v60 = sub_4B60(v58, "r");
    v55 = sub_4B60(qword_10168, "w");
    sub_4900((char *)&stru_BAB5._IO_write_end + 1);
    sub_4900((char *)&stru_BAB5._IO_write_ptr + 6);
    sub_4900(a2[optind]);
    v25 = (int *)&unk_BD30;
    v26 = (int *)&unk_BD30;
    sub_4900(a2[optind + 1]);
    sub_4900(0);
    act.sa_flags = 0x10000000;
    sigemptyset(&act.sa_mask);
    do
    {
      v27 = *v26++;
      sigaddset(&act.sa_mask, v27);
    }
    while ( &unk_BD4C != (_UNKNOWN *)v26 );
    v28 = &oact;
    do
    {
      v29 = *v25;
      sigaction(*v25, 0, v28);
      if ( v28->sa_handler != (__sighandler_t)((char *)&dword_0 + 1) )
      {
        act.sa_handler = (__sighandler_t)sub_4820;
        sigaction(v29, &act, 0);
      }
      ++v28;
      ++v25;
    }
    while ( v28 != (struct sigaction *)&pid );
    signal(17, 0);
    byte_10158 = 1;
    if ( pipe(pipedes) )
      sub_4B30((char *)&stru_BAB5._IO_buf_end + 6);
    pid = fork();
    if ( pid < 0 )
      sub_4B30("fork");
    if ( !pid )
    {
      if ( qword_10510 != 1 )
      {
        act.sa_handler = (__sighandler_t)(&dword_0 + 1);
        sigaction(2, &act, 0);
      }
      act.sa_handler = 0;
      sigaction(13, &act, 0);
      close(pipedes[0]);
      if ( pipedes[1] != 1 )
      {
        dup2(pipedes[1], 1);
        close(pipedes[1]);
      }
      execvp(*argv, argv);
      v47 = __errno_location();
      _exit((*v47 == 2) + 126);
    }
    close(pipedes[1]);
    v61 = fdopen(pipedes[0], "r");
    if ( !v61 )
      sub_4B30((char *)&stru_BAB5._IO_save_base + 3);
    v66 = v61;
    v30 = (char *)sub_7DF0(&unk_10001);
    *v30 = 10;
    v31 = v30;
    v69 = v30;
    v68 = v30;
    v67 = v30;
    v63[0] = v59;
    v32 = (char *)sub_7DF0(&unk_10001);
    *v32 = 10;
    v63[3] = v32;
    v63[2] = v32;
    v63[1] = v32;
    v64[0] = v60;
    v54 = 1;
    v65 = (char *)sub_7DF0(&unk_10001);
    v64[2] = v65;
    v64[1] = v65;
    *v65 = 10;
    for ( i = 1; ; i += v43 )
    {
      while ( 1 )
      {
        v33 = &v71;
        for ( j = 256; ; j -= v39 )
        {
          v37 = (char *)rawmemchr(v31, 10);
          v38 = v37;
          v39 = v37 - v31;
          if ( v37 - v31 >= j )
            goto LABEL_76;
          memcpy(v33, v31, v37 - v31);
          if ( v38 < v69 )
            break;
          v31 = v68;
          v35 = sub_4D80(v68, v66);
          v36 = sig;
          v67 = v31;
          v69 = &v31[v35];
          v31[v35] = 10;
          if ( v36 )
          {
            if ( pid > 0 )
            {
              desta = v36;
              kill(pid, 13);
              v36 = desta;
            }
            if ( filename )
            {
              destb = v36;
              unlink(filename);
              v36 = destb;
            }
            if ( byte_10158 )
            {
              destc = v36;
              sub_4A10(v36);
              v36 = destc;
            }
            raise(v36);
            exit(2);
          }
          if ( !v35 )
          {
            v56 = v39 == 0;
LABEL_76:
            v46 = v56;
LABEL_77:
            sub_4BC0(v59);
            sub_4BC0(v60);
            sub_4BC0(v55);
            sub_4BC0(v61);
            while ( waitpid(pid, (int *)&endptr, 0) < 0 )
            {
              if ( *__errno_location() != 4 )
                sub_4B30("waitpid");
              sub_4A90();
            }
            pid = 0;
            if ( filename )
            {
              unlink(filename);
              filename = 0;
            }
            if ( v46 )
            {
              sub_4DC0((unsigned int)endptr, 1, *argv);
              if ( byte_10158 )
                sub_4A10(0);
              sub_4A90();
              exit(BYTE1(endptr));
            }
LABEL_90:
            sub_4AD0();
          }
          v33 += v39;
        }
        v40 = sig;
        v31 = v38 + 1;
        v33[v39] = 0;
        v67 = v38 + 1;
        if ( v40 )
        {
          if ( pid > 0 )
            kill(pid, 13);
          if ( filename )
            unlink(filename);
          if ( byte_10158 )
            sub_4A10(v40);
          raise(v40);
          exit(2);
        }
        if ( v71 != 32 )
          break;
        puts(nptr);
      }
      v41 = __errno_location();
      *v41 = 0;
      v42 = v41;
      v43 = strtoimax(nptr, &endptr, 10);
      if ( v43 < 0 )
        break;
      if ( *v42 )
        break;
      if ( *endptr != 44 )
        break;
      v44 = strtoimax(endptr + 1, &endptr, 10);
      if ( v44 < 0 || *v42 || *endptr )
        break;
      v45 = v44;
      if ( v43 >= v44 )
        v45 = v43;
      if ( v71 == 99 )
      {
        sub_5000(&v66, v45, stdout);
        if ( !(unsigned char)sub_5120(v63, v57, i, v43, v64, v58, v54, v44, v55) )
        {
          v46 = 0;
          goto LABEL_77;
        }
      }
      else
      {
        if ( v71 != 105 )
          break;
        if ( byte_10161 )
          sub_4F30(&v66, v45);
        else
          sub_5000(&v66, v45, stdout);
        sub_5000(v63, v43, v55);
        sub_4F30(v64, v44);
      }
      v54 += v44;
      v31 = v67;
    }
    sub_4B00(&v71);
  }
  if ( (int)v3 - optind <= 1 )
  {
    sub_4840("missing operand after '%s'");
LABEL_89:
    sub_4840(0);
    goto LABEL_90;
  }
  v15 = (_IO_FILE *)&stru_BAB5;
  sub_4840((char *)&stru_BAB5);
LABEL_36:
  __overflow(v15, 10);
  while ( 1 )
  {
    v14 = v4[1];
    ++v4;
    if ( !v14 )
      break;
LABEL_12:
    while ( *v14 )
    {
      ++v4;
      v17 = dcgettext(0, v14, 5);
      __printf_chk(1, v3, v17);
      v14 = *v4;
      if ( !*v4 )
        goto LABEL_14;
    }
    v15 = stdout;
    IO_write_ptr = stdout->_IO_write_ptr;
    if ( IO_write_ptr >= stdout->_IO_write_end )
      goto LABEL_36;
    stdout->_IO_write_ptr = IO_write_ptr + 1;
    *IO_write_ptr = 10;
  }
LABEL_14:
  v18 = dcgettext(0, "Exit status is 0 if inputs are the same, 1 if different, 2 if trouble.", 5);
  v19 = dcgettext(0, "If a FILE is '-', read standard input.", 5);
  __printf_chk(1, "\n%s\n%s\n", v19, v18);
  sub_7CD0();
  sub_4CC0(1, "\n%s\n%s\n", v20, v21);
  return 0;
}



// Function: catchsig @ 0x4820
long long catchsig(int a1)
{
  long long result; // rax

  if ( a1 != 2 || (result = (unsigned char)byte_10160, !byte_10160) )
    sig = a1;
  return result;
}



// Function: try_help @ 0x4840
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
  v3 = qword_20620;
  v4 = dcgettext(0, "Try '%s --help' for more information.", 5);
  error(2, 0, v4, v3);
  return sub_48A0(2);
}



// Function: diraccess @ 0x48a0
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



// Function: diffarg @ 0x4900
char **diffarg(char *a1)
{
  long long v1; // rsi
  char **result; // rax
  long long v3; // rax
  long long v4; // rsi

  v1 = qword_100A8;
  result = argv;
  if ( qword_100A8 == qword_100A0 )
  {
    if ( qword_100A8 )
    {
      if ( (unsigned long long)qword_100A8 > 0x7FFFFFFFFFFFFFELL )
        sub_8270();
      v3 = 2 * qword_100A8;
      v4 = 16 * qword_100A8;
    }
    else
    {
      v3 = 16;
      v4 = 128;
    }
    qword_100A0 = v3;
    result = (char **)sub_7E40(argv, v4);
    v1 = qword_100A8;
    argv = result;
  }
  result[v1] = a1;
  qword_100A8 = v1 + 1;
  return result;
}



// Function: exiterr @ 0x4ad0
void exiterr()
{
  sub_4990(0);
  if ( byte_10158 )
    sub_4A10(0);
  sub_4A90();
  exit(2);
}



// Function: fatal @ 0x4b00
void fatal(char *msgid)
{
  char *v1; // rax

  v1 = dcgettext(0, msgid, 5);
  error(0, 0, "%s", v1);
  sub_4AD0();
}



// Function: perror_fatal @ 0x4b30
void perror_fatal(const char *a1)
{
  int v1; // ebp

  v1 = *__errno_location();
  sub_4A90();
  error(0, v1, "%s", a1);
  sub_4AD0();
}



// Function: ck_fwrite @ 0x4b80
size_t ck_fwrite(const void *a1, size_t a2, FILE *a3)
{
  size_t result; // rax
  char *v4; // rax

  result = fwrite_unlocked(a1, 1u, a2, a3);
  if ( a2 != result )
  {
    v4 = dcgettext(0, "write failed", 5);
    sub_4B30(v4);
  }
  return result;
}



// Function: skip_white @ 0x4e40
int skip_white()
{
  const unsigned short **v0; // rbx
  const unsigned short *v1; // rcx
  int result; // eax
  unsigned int v3; // ebp
  char *IO_read_ptr; // rax
  char *v5; // rax

  v0 = __ctype_b_loc();
  while ( 1 )
  {
    IO_read_ptr = stdin->_IO_read_ptr;
    if ( IO_read_ptr >= stdin->_IO_read_end )
      break;
    v1 = *v0;
    stdin->_IO_read_ptr = IO_read_ptr + 1;
    result = (unsigned char)*IO_read_ptr;
    if ( (v1[result] & 0x2000) == 0 )
      goto LABEL_7;
LABEL_3:
    if ( result == 10 )
      goto LABEL_7;
    v3 = sig;
    if ( sig )
    {
      if ( pid > 0 )
        kill(pid, 13);
      if ( filename )
        unlink(filename);
      if ( byte_10158 )
        sub_4A10(v3);
      raise(v3);
      exit(2);
    }
  }
  result = __uflow(stdin);
  if ( ((*v0)[result] & 0x2000) != 0 )
    goto LABEL_3;
LABEL_7:
  if ( (stdin->_flags & 0x20) != 0 )
  {
    v5 = dcgettext(0, "read failed", 5);
    sub_4B30(v5);
  }
  return result;
}



// Function: lf_skip @ 0x4f30
void lf_skip(long long a1, long long a2)
{
  long long v2; // rbp
  char *v3; // rax
  long long v4; // rax
  long long v5; // rdx
  unsigned int v6; // r12d
  char *v7; // rdx

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
      v4 = sub_4D80(*(long long *)(a1 + 16), *(long long *)a1);
      v5 = *(long long *)(a1 + 16);
      v6 = sig;
      *(long long *)(a1 + 8) = v5;
      v7 = (char *)(v4 + v5);
      *(long long *)(a1 + 24) = v7;
      *v7 = 10;
      if ( v6 )
      {
        if ( pid > 0 )
          kill(pid, 13);
        if ( filename )
          unlink(filename);
        if ( byte_10158 )
          sub_4A10(v6);
        raise(v6);
        exit(2);
      }
    }
    while ( v4 );
  }
}



// Function: lf_copy @ 0x5000
size_t lf_copy(long long *a1, long long a2, FILE *a3)
{
  const void *v5; // rdi
  char *v6; // rbp
  long long v7; // rbx
  const void *v8; // r12
  char *v9; // rax
  size_t v10; // rsi
  size_t result; // rax
  long long v12; // rdx
  unsigned int v13; // ebp
  char *v14; // rdx

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
        sub_4B80(v8, v6 - (char *)v8, a3);
        result = sub_4D80(a1[2], *a1);
        v12 = a1[2];
        v13 = sig;
        a1[1] = v12;
        v14 = (char *)(result + v12);
        a1[3] = v14;
        *v14 = 10;
        if ( v13 )
        {
          if ( pid > 0 )
            kill(pid, 13);
          if ( filename )
            unlink(filename);
          if ( byte_10158 )
            sub_4A10(v13);
          raise(v13);
          exit(2);
        }
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
  return sub_4B80(v8, v10, a3);
}



// Function: edit @ 0x5120
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
  void *v9; // r13
  char *IO_write_ptr; // rax
  FILE *v11; // rdi
  int v12; // eax
  char *v13; // rax
  char *v14; // rax
  int v15; // eax
  int v16; // ebx
  FILE *v17; // rax
  unsigned int v18; // ebp
  __pid_t v19; // eax
  __pid_t v20; // ebx
  const char *v21; // rbp
  FILE *v22; // rax
  FILE *v23; // rsi
  size_t v24; // rsi
  unsigned long long v26; // rax
  char *v27; // rax
  char *v28; // rbp
  size_t v29; // rax
  char *v30; // r12
  int v31; // edi
  int *v32; // rax
  int v33; // [rsp-7Ch] [rbp-100ACh]
  long long v34; // [rsp-78h] [rbp-100A8h]
  long long *v35; // [rsp-70h] [rbp-100A0h]
  long long *v36; // [rsp-68h] [rbp-10098h]
  const char *v37; // [rsp-60h] [rbp-10090h]
  long long v38; // [rsp-58h] [rbp-10088h]
  const char *v39; // [rsp-50h] [rbp-10080h]
  unsigned int v40; // [rsp-3Ch] [rbp-1006Ch] BYREF
  char *v41[4]; // [rsp-38h] [rbp-10068h] BYREF
  char v42[24]; // [rsp-18h] [rbp-10048h] BYREF
  char v43; // [rsp+0h] [rbp-10030h] BYREF
  long long v44; // [rsp+1000h] [rbp-F030h] BYREF
  unsigned long long v45; // [rsp+FFF0h] [rbp-40h]

  while ( &v43 != (char *)(&v44 - 0x2000) )
    ;
  v36 = a1;
  v37 = a2;
  v38 = a3;
  v34 = a4;
  v35 = a5;
  v39 = a6;
  v45 = __readfsqword(0x28u);
LABEL_3:
  v9 = &unk_10405;
  while ( 1 )
  {
    while ( 1 )
    {
      IO_write_ptr = stdout->_IO_write_ptr;
      if ( IO_write_ptr >= stdout->_IO_write_end )
      {
        a2 = (char *)(&qword_20 + 5);
        if ( __overflow(stdout, 37) != 37 )
        {
LABEL_63:
          v27 = dcgettext(0, "write failed", 5);
          sub_4B30(v27);
        }
      }
      else
      {
        stdout->_IO_write_ptr = IO_write_ptr + 1;
        *IO_write_ptr = 37;
      }
      v11 = stdout;
      if ( fflush_unlocked(stdout) )
        goto LABEL_63;
      v12 = sub_4E40();
      if ( v12 > 50 )
        break;
      if ( v12 > 48 )
        goto LABEL_21;
      if ( v12 == -1 )
      {
        if ( (stdin->_flags & 0x10) != 0 )
          return 0;
      }
      else if ( v12 == 10 )
      {
        goto LABEL_11;
      }
LABEL_14:
      ((void (*)(FILE *, const char *, long long))sub_4D10)(v11, a2, (unsigned int)v12);
LABEL_11:
      v13 = dcgettext(
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
      a2 = (char *)(&dword_0 + 1);
      __fprintf_chk(stderr, 1, "%s", v13);
    }
    if ( v12 == 101 )
      break;
    if ( v12 <= 100 )
      goto LABEL_14;
    if ( v12 > 115 )
    {
      if ( v12 != 118 )
        goto LABEL_14;
LABEL_21:
      v33 = v12;
      if ( sub_4E40() == 10 )
      {
        if ( v33 > 100 )
        {
          switch ( v33 )
          {
            case 'q':
              return 0;
            case 'r':
              goto LABEL_61;
            case 's':
              byte_10161 = 1;
              goto LABEL_3;
            case 'v':
              byte_10161 = 0;
              goto LABEL_3;
          }
        }
        if ( v33 != 49 )
        {
LABEL_61:
          sub_5000(v35, a8, a9);
          sub_4F30((long long)v36, v34);
          return 1;
        }
LABEL_66:
        sub_5000(v36, v34, a9);
        sub_4F30((long long)v35, a8);
        return 1;
      }
      goto LABEL_19;
    }
    if ( v12 > 112 )
      goto LABEL_21;
    if ( v12 != 108 )
      goto LABEL_14;
    if ( sub_4E40() == 10 )
      goto LABEL_66;
LABEL_19:
    v14 = dcgettext(
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
    a2 = (char *)(&dword_0 + 1);
    __fprintf_chk(stderr, 1, "%s", v14);
    sub_4D10();
  }
  v15 = sub_4E40();
  v16 = v15;
  if ( v15 <= 50 )
  {
    if ( v15 > 48 )
      goto LABEL_58;
    if ( v15 == 10 )
      goto LABEL_27;
    goto LABEL_19;
  }
  v26 = (unsigned int)(v15 - 98);
  if ( (unsigned int)v26 > 0x10 || !_bittest64((const long long *)&v9, v26) )
    goto LABEL_19;
LABEL_58:
  if ( sub_4E40() != 10 )
    goto LABEL_19;
LABEL_27:
  if ( filename )
  {
    v17 = fopen(filename, "w");
  }
  else
  {
    v28 = getenv("TMPDIR");
    if ( !v28 )
      v28 = "/tmp";
    v29 = strlen(v28);
    v30 = (char *)sub_7DF0(v29 + 13);
    __sprintf_chk(v30, 1, -1, "%s/sdiffXXXXXX", v28);
    v31 = mkstemp(v30);
    if ( v31 < 0 )
      sub_4B30("mkstemp");
    filename = v30;
    v17 = fdopen(v31, "w");
  }
  stream = v17;
  if ( !v17 )
    sub_4B30(filename);
  if ( v16 == 100 )
  {
    if ( v34 )
    {
      if ( v34 == 1 )
        __fprintf_chk(v17, 1, "--- %s %ld\n", v37, v38);
      else
        __fprintf_chk(v17, 1, "--- %s %ld,%ld\n", v37, v38, v38 + v34 - 1);
      v17 = stream;
    }
    sub_5000(v36, v34, v17);
    if ( a8 )
    {
      if ( a8 == 1 )
        __fprintf_chk(stream, 1, "+++ %s %ld\n", v39, a7);
      else
        __fprintf_chk(stream, 1, "+++ %s %ld,%ld\n", v39, a7, a7 + a8 - 1);
    }
LABEL_38:
    sub_5000(v35, a8, stream);
  }
  else
  {
    if ( v16 > 100 )
    {
      if ( v16 == 108 )
        goto LABEL_34;
    }
    else if ( v16 == 49 || v16 == 98 )
    {
LABEL_34:
      sub_5000(v36, v34, v17);
      goto LABEL_35;
    }
    sub_4F30((long long)v36, v34);
LABEL_35:
    if ( v16 > 100 )
    {
      if ( v16 == 114 )
        goto LABEL_38;
    }
    else if ( v16 == 50 || v16 == 98 )
    {
      goto LABEL_38;
    }
    sub_4F30((long long)v35, a8);
  }
  if ( fclose(stream) )
    goto LABEL_97;
  byte_10160 = 1;
  v18 = sig;
  if ( sig )
  {
LABEL_95:
    sub_4990(0);
    goto LABEL_89;
  }
  v41[2] = 0;
  v41[0] = file;
  v41[1] = filename;
  v19 = fork();
  v20 = v19;
  if ( !v19 )
  {
    execvp(file, v41);
    v32 = __errno_location();
    _exit((*v32 == 2) + 126);
  }
  if ( v19 < 0 )
    sub_4B30("fork");
  while ( waitpid(v20, (int *)&v40, 0) < 0 )
  {
    if ( *__errno_location() != 4 )
      sub_4B30("waitpid");
    v18 = sig;
    if ( sig )
    {
      if ( pid > 0 )
        kill(pid, 13);
      if ( filename )
        unlink(filename);
LABEL_89:
      if ( byte_10158 )
        sub_4A10(v18);
      raise(v18);
      exit(2);
    }
  }
  byte_10160 = 0;
  sub_4DC0(v40, 0, file);
  v21 = filename;
  v22 = fopen(filename, "r");
  v23 = v22;
  if ( !v22 )
    sub_4B30(v21);
  for ( stream = v22; ; v23 = stream )
  {
    v24 = sub_4D80(v42, v23);
    if ( !v24 )
      break;
    v18 = sig;
    if ( sig )
      goto LABEL_95;
    sub_4B80(v42, v24, a9);
  }
  if ( fclose(stream) )
LABEL_97:
    sub_4B30("fclose");
  return 1;
}


