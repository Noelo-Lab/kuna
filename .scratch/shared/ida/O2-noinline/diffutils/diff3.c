// Function: main @ 0x3ac0
long long main(int a1, char **a2, char **a3)
{
  int v3; // r15d
  char *v5; // rdi
  int v6; // eax
  int v8; // eax
  char v9; // dl
  bool v10; // cl
  char v11; // al
  char v12; // al
  long long v13; // rax
  const char **v14; // rbp
  long long i; // rax
  int v16; // ebx
  int v17; // r12d
  long long v18; // r14
  const char **v19; // rbx
  const char *v20; // r12
  const char *v21; // r13
  long long v22; // r12
  long long v23; // rax
  unsigned char v24; // bl
  int v25; // [rsp+Ch] [rbp-12Ch]
  char v26[8]; // [rsp+10h] [rbp-128h] BYREF
  void *ptr; // [rsp+18h] [rbp-120h] BYREF
  void *v28; // [rsp+20h] [rbp-118h] BYREF
  int v29; // [rsp+28h] [rbp-110h]
  int v30; // [rsp+2Ch] [rbp-10Ch]
  int v31; // [rsp+30h] [rbp-108h]
  int v32[5]; // [rsp+34h] [rbp-104h]
  long long v33; // [rsp+48h] [rbp-F0h]
  long long v34; // [rsp+50h] [rbp-E8h]
  struct stat buf; // [rsp+60h] [rbp-D8h] BYREF
  unsigned long long v36; // [rsp+F8h] [rbp-40h]

  v3 = 0;
  v5 = *a2;
  v36 = __readfsqword(0x28u);
  status = 2;
  sub_62E0(v5);
  setlocale(6, "");
  bindtextdomain("diffutils", "/usr/local/share/locale");
  textdomain("diffutils");
  sub_6220(0);
  sub_86A0();
  v25 = 0;
  while ( 2 )
  {
    v6 = getopt_long(a1, a2, "aeimvx3AEL:TX", &longopts, 0);
    if ( v6 == -1 )
    {
      v9 = byte_130E9;
      v10 = v3 != 0;
      byte_130F0 = (v3 != 0) & (byte_130E9 ^ 1);
      v11 = byte_130E9 & (v3 == 0);
      byte_130EB |= v11;
      v12 = byte_130EF | v11;
      byte_130EF = v12;
      if ( (v3 & (v3 - 1)) != 0 || byte_130EA && byte_130E9 || v25 && !v12 )
      {
LABEL_52:
        sub_41B0("incompatible options");
      }
      else
      {
        v13 = optind;
        if ( a1 - optind != 3 )
        {
          if ( a1 - optind <= 2 )
          {
            sub_41B0("missing operand after '%s'");
LABEL_29:
            sub_41B0(0);
          }
          v13 = sub_41B0("extra operand '%s'");
        }
        v14 = (const char **)&a2[v13];
        for ( i = v25; (int)i <= 2; ++i )
          *(long long *)&v32[2 * i + 3] = v14[i];
        v16 = (unsigned char)(v9 | v10);
        v17 = 2 - v16;
        v18 = 2 - v16;
        if ( strcmp(v14[v18], "-") || (v17 = v16 + 1, strcmp(*v14, "-")) && (v18 = v17, strcmp(v14[v17], "-")) )
        {
          v31 = v17;
          v19 = v14;
          v29 = 0;
          v32[0] = 0;
          v30 = 3 - v17;
          v32[3 - v17] = 1;
          v32[v18] = 2;
          while ( 1 )
          {
            v20 = *v19;
            if ( strcmp(*v19, "-") )
            {
              if ( stat(v20, &buf) < 0 )
              {
                sub_43A0(*v19);
LABEL_51:
                sub_41B0("too many file label options");
                goto LABEL_52;
              }
              if ( (buf.st_mode & 0xF000) == 0x4000 )
              {
                error(2, 21, "%s", *v19);
LABEL_47:
                sub_4210("read failed");
              }
            }
            if ( v14 + 3 == ++v19 )
            {
              signal(17, 0);
              v21 = v14[v32[2]];
              v22 = sub_5D70(v14[v32[1]], v21, v26, &v28);
              v23 = sub_5D70(v14[v32[0]], v21, v26, &ptr);
              sub_4E40(v23, v22);
              if ( byte_130F0 )
              {
                v24 = sub_50F0(stdout, v34);
              }
              else
              {
                v24 = byte_130E9;
                if ( byte_130E9 )
                {
                  sub_8570(v14[v32[0]], "r", stdin);
                  v24 = sub_57D0(stdin, stdout, v33, v34);
                  if ( ferror_unlocked(stdin) )
                    goto LABEL_47;
                }
                else
                {
                  sub_5520(stdout);
                }
              }
              free(ptr);
              free(v28);
              sub_5C30();
              exit(v24);
            }
          }
        }
      }
      sub_4210("'-' specified for more than one input file");
      start();
    }
    switch ( v6 )
    {
      case 51:
        byte_130ED = 1;
        v3 |= 1u;
        continue;
      case 65:
        byte_130EB = 1;
        v3 |= 2u;
        byte_130EF = 1;
        continue;
      case 69:
        byte_130EF = 1;
        v3 |= 4u;
        continue;
      case 76:
        if ( v25 > 2 )
          goto LABEL_51;
        *(long long *)&v32[2 * v25++ + 3] = optarg;
        continue;
      case 84:
        byte_130EE = 1;
        continue;
      case 88:
        byte_130EC = 1;
        v3 |= 8u;
        continue;
      case 97:
        byte_130F2 = 1;
        continue;
      case 101:
        v3 |= 0x10u;
        continue;
      case 105:
        byte_130EA = 1;
        continue;
      case 109:
        byte_130E9 = 1;
        continue;
      case 118:
        v8 = sub_65F0("Randy Smith");
        sub_8010((int)stdout, (unsigned int)"diff3", (unsigned int)"GNU diffutils", (int)off_13018, v8, 0);
        sub_5C30();
        return 0;
      case 120:
        byte_130EC = 1;
        v3 |= 0x20u;
        continue;
      case 128:
        file = (char *)optarg;
        continue;
      case 129:
        sub_4240();
        sub_5C30();
        return 0;
      case 130:
        byte_130F1 = 1;
        continue;
      default:
        goto LABEL_29;
    }
  }
}



// Function: skipwhite @ 0x4110
char *skipwhite(char *a1)
{
  char v1; // dl
  char *result; // rax

  v1 = *a1;
  result = a1;
  if ( *a1 != 32 )
    goto LABEL_3;
  do
  {
    do
      v1 = *++result;
    while ( v1 == 32 );
LABEL_3:
    ;
  }
  while ( v1 == 9 );
  return result;
}



// Function: readnum @ 0x4140
long long readnum(unsigned char *a1, long long *a2)
{
  int v2; // edx
  long long result; // rax

  v2 = *a1;
  result = 0;
  if ( (unsigned int)(v2 - 48) <= 9 )
  {
    do
    {
      ++a1;
      result = v2 - 48 + 10 * result;
      v2 = *a1;
    }
    while ( (unsigned int)(v2 - 48) <= 9 );
    *a2 = result;
    return (long long)a1;
  }
  return result;
}



// Function: reverse_diff3_blocklist @ 0x4180
long long reverse_diff3_blocklist(long long a1)
{
  long long result; // rax
  long long v2; // rcx
  long long v3; // rdx

  result = a1;
  if ( a1 )
  {
    v2 = 0;
    while ( 1 )
    {
      v3 = *(long long *)(result + 104);
      *(long long *)(result + 104) = v2;
      v2 = result;
      if ( !v3 )
        break;
      result = v3;
    }
  }
  return result;
}



// Function: try_help @ 0x41b0
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
  v3 = qword_23140;
  v4 = dcgettext(0, "Try '%s --help' for more information.", 5);
  error(2, 0, v4, v3);
  return sub_4210((char *)&dword_0 + 2);
}



// Function: fatal @ 0x4210
long long fatal(char *msgid)
{
  char *v1; // rax

  v1 = dcgettext(0, msgid, 5);
  error(2, 0, "%s", v1);
  return sub_4240();
}



// Function: usage @ 0x4240
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
  FILE *v9; // rbp
  char *v10; // rax
  char *v11; // r12
  char *v12; // rax

  v0 = qword_23140;
  v1 = (const char **)&off_12880;
  v2 = dcgettext(0, "Usage: %s [OPTION]... MYFILE OLDFILE YOURFILE\n", 5);
  __printf_chk(1, v2, v0);
  v3 = dcgettext(0, "Compare three files line by line.", 5);
  __printf_chk(1, "%s\n\n", v3);
  v4 = stdout;
  v5 = dcgettext(0, "Mandatory arguments to long options are mandatory for short options too.\n", 5);
  fputs_unlocked(v5, v4);
  v6 = 45;
  v7 = "-A, --show-all              output all changes, bracketing conflicts";
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
  v9 = stdout;
  v10 = dcgettext(
          0,
          "\n"
          "The default output format is a somewhat human-readable representation of\n"
          "the changes.\n"
          "\n"
          "The -e, -E, -x, -X (and corresponding long) options cause an ed script\n"
          "to be output instead of the default.\n"
          "\n"
          "Finally, the -m (--merge) option causes diff3 to do the merge internally\n"
          "and output the actual merged file.  For unusual input, this is more\n"
          "robust than using ed.\n",
          5);
  fputs_unlocked(v10, v9);
  v11 = dcgettext(0, "Exit status is 0 if successful, 1 if conflicts, 2 if trouble.", 5);
  v12 = dcgettext(0, "If a FILE is '-', read standard input.", 5);
  __printf_chk(1, "\n%s\n%s\n", v12, v11);
  return sub_80C0();
}



// Function: perror_with_exit @ 0x43a0
long long perror_with_exit(const char *a1)
{
  int *v1; // rax

  v1 = __errno_location();
  error(2, *v1, "%s", a1);
  return sub_43D0();
}



// Function: read_diff @ 0x43d0
long long read_diff(const char *a1, const char *a2, long long *a3)
{
  const char **v3; // rax
  int v4; // r12d
  __blksize_t st_blksize; // rax
  long long v6; // rbx
  unsigned long long v7; // r15
  long long i; // rbp
  long long v9; // r14
  long long v10; // rax
  int *v12; // rax
  char *v13; // r12
  unsigned int v14; // ebx
  const char *v15; // rsi
  char *v16; // rax
  char *v17; // rdi
  __pid_t pid; // [rsp+4h] [rbp-144h]
  int stat_loc; // [rsp+14h] [rbp-134h] BYREF
  int pipedes[2]; // [rsp+18h] [rbp-130h] BYREF
  char *argv; // [rsp+20h] [rbp-128h] BYREF
  const char *v23; // [rsp+28h] [rbp-120h] BYREF
  char v24; // [rsp+30h] [rbp-118h] BYREF
  struct stat buf; // [rsp+70h] [rbp-D8h] BYREF
  unsigned long long v26; // [rsp+108h] [rbp-40h]

  v26 = __readfsqword(0x28u);
  argv = file;
  v3 = &v23;
  if ( byte_130F2 )
  {
    v23 = "-a";
    v3 = (const char **)&v24;
  }
  if ( byte_130F1 )
    *v3++ = "--strip-trailing-cr";
  v3[2] = a1;
  *v3 = "--horizon-lines=100";
  v3[1] = "--";
  v3[3] = a2;
  v3[4] = 0;
  if ( pipe(pipedes) )
    sub_43A0("pipe");
  pid = fork();
  if ( !pid )
  {
LABEL_26:
    close(pipedes[0]);
    if ( pipedes[1] != 1 )
    {
      dup2(pipedes[1], 1);
      close(pipedes[1]);
    }
    execvp(file, &argv);
    v12 = __errno_location();
    _exit((*v12 == 2) + 126);
  }
  if ( pid == -1 )
  {
LABEL_25:
    sub_43A0("fork");
    goto LABEL_26;
  }
  close(pipedes[1]);
  v4 = pipedes[0];
  if ( fstat(pipedes[0], &buf) )
  {
    sub_43A0("fstat");
    goto LABEL_25;
  }
  st_blksize = 1;
  if ( buf.st_blksize > 0 )
    st_blksize = buf.st_blksize;
  v6 = 0;
  v7 = st_blksize;
  for ( i = sub_8180(st_blksize); ; i = sub_81D0(i, v7) )
  {
    v9 = v7 - v6;
    v10 = sub_8770(v4, (void *)(i + v6));
    v6 += v10;
    if ( v9 != v10 )
      break;
    if ( v7 > 0x3FFFFFFFFFFFFFFELL )
      sub_8530();
    v7 *= 2LL;
  }
  if ( v10 == -1 )
    goto LABEL_40;
  if ( v6 && *(char *)(i + v6 - 1) != 10 )
  {
LABEL_39:
    sub_4210("invalid diff format; incomplete last line");
LABEL_40:
    v17 = dcgettext(0, "read failed", 5);
    sub_43A0(v17);
    return sub_46D0((FILE *)v17);
  }
  *a3 = i;
  if ( close(v4) )
  {
LABEL_38:
    sub_43A0("close");
    goto LABEL_39;
  }
  if ( waitpid(pid, &stat_loc, 0) < 0 )
  {
    sub_43A0("waitpid");
    goto LABEL_38;
  }
  if ( (stat_loc & 0x7F) != 0 )
    goto LABEL_33;
  if ( (stat_loc & 0xFE00) != 0 )
  {
    v13 = file;
    v14 = BYTE1(stat_loc);
    if ( BYTE1(stat_loc) == 126 )
    {
      v15 = "subsidiary program '%s' could not be invoked";
    }
    else
    {
      v15 = "subsidiary program '%s' failed (exit status %d)";
      if ( BYTE1(stat_loc) == 127 )
        v15 = "subsidiary program '%s' not found";
    }
    while ( 1 )
    {
      v16 = dcgettext(0, v15, 5);
      error(2, 0, v16, v13, v14);
LABEL_33:
      v13 = file;
      v14 = 0x7FFFFFFF;
      v15 = "subsidiary program '%s' failed";
    }
  }
  return i + v6;
}



// Function: undotlines @ 0x46d0
int undotlines(FILE *stream, char a2, long long a3, long long a4)
{
  int result; // eax

  result = fputs_unlocked(".\n", stream);
  if ( a2 )
  {
    if ( a4 == 1 )
      return __fprintf_chk(stream, 1, "%lds/^\\.//\n", a3);
    else
      return __fprintf_chk(stream, 1, "%ld,%lds/^\\.//\n", a3, a3 + a4 - 1);
  }
  return result;
}



// Function: scan_diff_line @ 0x4770
long long scan_diff_line(char *a1, long long *a2, long long *a3, unsigned long long a4, char a5)
{
  char *v5; // rdi
  char *i; // rbx
  char *v7; // r12
  char v9; // al

  if ( *a1 == a5 && a1[1] == 32 )
  {
    v5 = a1 + 2;
    *a2 = v5;
    for ( i = v5; ; ++i )
    {
      v7 = i + 1;
      if ( *i == 10 )
        break;
    }
    *a3 = v7 - v5;
    if ( (unsigned long long)v7 < a4 && *v7 == 92 )
    {
      v9 = byte_130F0;
      if ( byte_130F0 )
      {
        __fprintf_chk(stderr, 1, "%s:", (const char *)qword_23140);
        v9 = byte_130F0;
      }
      else
      {
        *a3 = i - v5;
      }
      v7 = i + 2;
      while ( 1 )
      {
        if ( v9 )
          putc_unlocked((char)*v7, stderr);
        if ( *v7++ == 10 )
          break;
        v9 = byte_130F0;
      }
    }
    return (long long)v7;
  }
  else
  {
    sub_4210("invalid diff format; incorrect leading line chars");
    return sub_4850();
  }
}



// Function: create_diff3_block @ 0x4850
long long *create_diff3_block(long long a1, long long a2, long long a3, long long a4, long long a5, long long a6)
{
  long long v9; // rax
  long long *v10; // r12
  long long v11; // rbp
  long long v12; // rbx
  long long v13; // rax
  long long v14; // rdi
  long long v15; // rdi
  long long v16; // rax
  long long v18; // rax
  long long v19; // rbx
  long long v20; // r13

  v9 = sub_8180(112);
  *(long long *)(v9 + 16) = a2;
  v10 = (long long *)v9;
  *(int *)v9 = 0;
  *(long long *)(v9 + 104) = 0;
  *(long long *)(v9 + 8) = a1;
  *(long long *)(v9 + 24) = a3;
  *(long long *)(v9 + 32) = a4;
  *(long long *)(v9 + 40) = a5;
  *(long long *)(v9 + 48) = a6;
  v11 = a2 - a1 + 1;
  if ( a2 - a1 == -1 )
  {
    *(long long *)(v9 + 56) = 0;
    *(long long *)(v9 + 80) = 0;
    v12 = a4 - a3 + 1;
    if ( !v12 )
    {
LABEL_3:
      v10[8] = 0;
      v13 = 0;
      goto LABEL_4;
    }
  }
  else
  {
    *(long long *)(v9 + 56) = sub_8410(v11, 8);
    v18 = sub_8410(v11, 8);
    v19 = v10[4];
    v20 = v10[3];
    v10[10] = v18;
    v12 = v19 - v20 + 1;
    if ( !v12 )
      goto LABEL_3;
  }
  v10[8] = sub_8410(v12, 8);
  v13 = sub_8410(v12, 8);
LABEL_4:
  v14 = v10[6] - v10[5];
  v10[11] = v13;
  v15 = v14 + 1;
  if ( v15 )
  {
    v10[9] = sub_8410(v15, 8);
    v16 = sub_8410(v15, 8);
  }
  else
  {
    v10[9] = 0;
    v16 = 0;
  }
  v10[12] = v16;
  return v10;
}



// Function: copy_stringlist @ 0x49b0
long long copy_stringlist(long long a1, long long a2, long long a3, long long a4, long long a5)
{
  long long v9; // rbx
  const void *v10; // rsi
  size_t v11; // rdx

  if ( !a5 )
    return 1;
  v9 = 0;
  while ( 1 )
  {
    while ( 1 )
    {
      v10 = *(const void **)(a3 + 8 * v9);
      v11 = *(long long *)(a2 + 8 * v9);
      if ( v10 )
        break;
      *(long long *)(a3 + 8 * v9) = *(long long *)(a1 + 8 * v9);
      *(long long *)(a4 + 8 * v9++) = v11;
      if ( v9 == a5 )
        return 1;
    }
    if ( *(long long *)(a4 + 8 * v9) != v11 || memcmp(*(const void **)(a1 + 8 * v9), v10, v11) )
      break;
    if ( ++v9 == a5 )
      return 1;
  }
  return 0;
}



// Function: compare_line_list @ 0x4a50
long long compare_line_list(long long a1, long long a2, long long a3, long long a4, long long a5)
{
  long long v10; // rbx
  const void *v11; // rdi
  const void *v12; // rsi
  size_t v13; // rdx

  v10 = 0;
  do
  {
    if ( a5 == v10 )
      return 1;
    v11 = *(const void **)(a1 + 8 * v10);
    if ( !v11 )
      break;
    v12 = *(const void **)(a3 + 8 * v10);
    if ( !v12 )
      break;
    v13 = *(long long *)(a2 + 8 * v10);
    if ( v13 != *(long long *)(a4 + 8 * v10) )
      break;
    ++v10;
  }
  while ( !memcmp(v11, v12, v13) );
  return 0;
}



// Function: using_to_diff3_block @ 0x4ae0
long long *using_to_diff3_block(long long *a1, long long a2, int a3, int a4, long long a5)
{
  long long v5; // r12
  long long v6; // r9
  unsigned long long i; // rax
  long long *v8; // rdx
  long long v9; // rdx
  long long v10; // rbp
  long long *v11; // r14
  long long *v12; // r15
  long long v13; // r13
  long long v14; // rbx
  long long v16; // rdi
  long long j; // r12
  long long v18; // rdx
  long long *v19; // rbx
  long long k; // rax
  long long v21; // rcx
  long long v22; // r15
  long long v23; // r13
  long long v24; // r12
  long long v25; // rbp
  long long v26; // rax
  long long v27; // rdx
  long long v28; // rdx
  long long m; // rax
  long long v30; // rcx
  long long v31; // rax
  long long v32; // [rsp+8h] [rbp-90h]
  long long v33; // [rsp+8h] [rbp-90h]
  long long v35; // [rsp+18h] [rbp-80h]
  long long v36; // [rsp+18h] [rbp-80h]
  long long v37[2]; // [rsp+30h] [rbp-68h]
  long long v38[11]; // [rsp+40h] [rbp-58h]

  v38[3] = __readfsqword(0x28u);
  v5 = *(long long *)(a1[a3] + 16LL);
  v6 = *(long long *)(*(long long *)(a2 + 8LL * a4) + 24LL);
  for ( i = 0; i != 16; i += 8LL )
  {
    v8 = (long long *)a1[i / 8];
    if ( v8 )
    {
      v37[i / 8] = *v8 + v5 - v8[2];
      v9 = *(long long *)(*(long long *)(a2 + i) + 8LL) + v6 - *(long long *)(*(long long *)(a2 + i) + 24LL);
    }
    else
    {
      v37[i / 8] = *(long long *)(a5 + 2 * i + 16) + v5 - *(long long *)(a5 + 48);
      v9 = *(long long *)(a5 + 2 * i + 16) + v6 - *(long long *)(a5 + 48);
    }
    v38[i / 8] = v9;
  }
  v10 = v37[0];
  v35 = v38[0];
  v32 = 0;
  v11 = sub_4850(v37[0], v38[0], v37[1], v38[1], v5, v6);
  while ( 1 )
  {
    v12 = (long long *)a1[v32];
    if ( v12 )
      break;
LABEL_13:
    if ( v32 == 1 )
    {
      v16 = v35;
      v33 = v5;
      for ( j = 0; ; v16 = *(long long *)((char *)v38 + j) )
      {
        v18 = v10;
        v19 = *(long long **)((char *)a1 + j);
        for ( k = 0; ; k += 8 )
        {
          v21 = v16 + 1;
          if ( v19 )
            v21 = *v19;
          if ( v21 <= v18 )
            break;
          ++v18;
          *(long long *)(*(long long *)((char *)v11 + j + 56) + k) = *(long long *)(v11[9] + k);
          *(long long *)(*(long long *)((char *)v11 + j + 80) + k) = *(long long *)(v11[12] + k);
        }
        if ( v19 )
          break;
LABEL_29:
        j += 8;
        if ( j == 16 )
        {
          if ( *a1 )
          {
            if ( a1[1] )
            {
              v31 = v11[2] - v11[1];
              if ( v31 == v11[4] - v11[3] && (unsigned char)sub_4A50(v11[7], v11[10], v11[8], v11[11], v31 + 1) )
                *(int *)v11 = 7;
              else
                *(int *)v11 = 4;
            }
            else
            {
              *(int *)v11 = 5;
            }
          }
          else
          {
            *(int *)v11 = 6;
          }
          return v11;
        }
        v10 = *(long long *)((char *)v37 + j);
      }
      v36 = j;
      v22 = *(long long *)((char *)v11 + j + 56);
      v23 = *(long long *)((char *)v11 + j + 80);
      v24 = v10;
      v25 = v16 + 1 - v10;
LABEL_22:
      if ( (unsigned char)sub_49B0(v19[4], v19[6], v22 + 8 * (*v19 - v24), v23 + 8 * (*v19 - v24), v19[1] - *v19 + 1) )
      {
        v26 = v19[1];
        v27 = v19[3] - v33;
        v19 = (long long *)v19[8];
        v28 = 8 * v27 + 8;
        for ( m = v26 + 1 - v24; ; ++m )
        {
          if ( v19 )
          {
            if ( *v19 - v24 <= m )
              goto LABEL_22;
          }
          else if ( v25 <= m )
          {
            j = v36;
            goto LABEL_29;
          }
          *(long long *)(v22 + 8 * m) = *(long long *)(v11[9] + v28);
          v30 = *(long long *)(v11[12] + v28);
          v28 += 8;
          *(long long *)(v23 + 8 * m) = v30;
        }
      }
      return 0;
    }
    v32 = 1;
  }
  v13 = v11[12];
  v14 = v11[9];
  while ( (unsigned char)sub_49B0(
                             v12[5],
                             v12[7],
                             v14 + 8 * (v12[2] - v5),
                             v13 + 8 * (v12[2] - v5),
                             v12[3] - v12[2] + 1LL) )
  {
    v12 = (long long *)v12[8];
    if ( !v12 )
      goto LABEL_13;
  }
  return 0;
}



// Function: make_3way_diff @ 0x4e40
long long make_3way_diff(long long *a1, long long *a2)
{
  long long *v2; // rdx
  long long *v3; // rax
  long long *v4; // r8
  long long *v5; // rbx
  long long v6; // rax
  int v7; // r10d
  long long v8; // rsi
  long long v9; // rax
  long long *v10; // rdi
  long long v11; // rcx
  int v12; // r9d
  long long *i; // rdx
  long long *v14; // rsi
  long long *v15; // rsi
  long long v16; // rdx
  int v17; // ecx
  long long *v18; // rax
  long long v20; // [rsp+8h] [rbp-50h] BYREF
  long long v21; // [rsp+10h] [rbp-48h] BYREF
  long long v22; // [rsp+18h] [rbp-40h]
  long long v23; // [rsp+20h] [rbp-38h] BYREF
  long long v24; // [rsp+28h] [rbp-30h]
  long long *v25; // [rsp+30h] [rbp-28h]
  long long *v26; // [rsp+38h] [rbp-20h]
  unsigned long long v27; // [rsp+48h] [rbp-10h]

  v2 = a1;
  v3 = a2;
  v4 = (long long *)&unk_DD60;
  v27 = __readfsqword(0x28u);
  v5 = &v20;
  v25 = a1;
  v20 = 0;
  v26 = a2;
  if ( !a1 )
    goto LABEL_15;
LABEL_2:
  v24 = 0;
  v23 = 0;
  v22 = 0;
  v21 = 0;
  if ( v3 )
  {
    v6 = v3[2];
    v7 = v2[2] > v6;
    v8 = v2[2] > v6;
    v9 = v2[2] <= v6;
    v2 = (&v25)[v8];
  }
  else
  {
    v9 = 1;
    v7 = 0;
    v8 = 0;
  }
  while ( 1 )
  {
    v10 = (long long *)v2[8];
    v11 = v2[3];
    *(&v23 + v8) = (long long)v2;
    v12 = v7;
    *(&v21 + v8) = (long long)v2;
    (&v25)[v8] = v10;
    v2[8] = 0;
    for ( i = (&v25)[v9]; i; i = (&v25)[(int)v9] )
    {
      if ( i[2] > v11 + 1 )
        break;
      if ( *(&v21 + v9) )
        *(long long *)(*(&v23 + v9) + 64) = i;
      else
        *(&v21 + v9) = (long long)i;
      v14 = (&v25)[v9];
      *(&v23 + v9) = (long long)i;
      v15 = (long long *)v14[8];
      i[8] = 0;
      v16 = i[3];
      (&v25)[v9] = v15;
      LODWORD(v9) = v12 ^ 1;
      if ( v16 > v11 )
      {
        v17 = v12;
        v12 ^= 1u;
        LODWORD(v9) = v17;
        v11 = v16;
      }
      v9 = (int)v9;
    }
    v18 = sub_4AE0(&v21, (long long)&v23, v7, v12, (long long)v4);
    v4 = v18;
    if ( !v18 )
      sub_4210("internal error: screwup in format of diff blocks");
    v2 = v25;
    *v5 = (long long)v18;
    v5 = v18 + 13;
    v3 = v26;
    if ( v2 )
      goto LABEL_2;
LABEL_15:
    if ( !v3 )
      return v20;
    v2 = v3;
    v8 = 1;
    v9 = 0;
    v24 = 0;
    v23 = 0;
    v7 = 1;
    v22 = 0;
    v21 = 0;
  }
}



// Function: dotlines @ 0x5030
long long dotlines(FILE *stream, long long a2, int a3)
{
  long long v3; // r13
  long long v4; // r14
  long long v5; // rbx
  long long v6; // rdx
  char *v7; // rbp
  unsigned char v9; // [rsp+7h] [rbp-41h]

  v3 = a3;
  v4 = a2 + 16LL * a3;
  if ( *(long long *)(v4 + 16) < *(long long *)(v4 + 8) )
  {
    return 0;
  }
  else
  {
    v9 = 0;
    v5 = 0;
    do
    {
      v6 = 8 * v5;
      v7 = *(char **)(*(long long *)(a2 + 8 * v3 + 56) + 8 * v5);
      if ( *v7 == 46 )
      {
        fputc_unlocked(46, stream);
        v9 = 1;
        v6 = 8 * v5;
      }
      ++v5;
      fwrite_unlocked(v7, 1u, *(long long *)(*(long long *)(a2 + 8 * v3 + 80) + v6), stream);
    }
    while ( *(long long *)(v4 + 16) - *(long long *)(v4 + 8) >= v5 );
  }
  return v9;
}



// Function: output_diff3_edscript @ 0x50f0
long long output_diff3_edscript(
        FILE *stream,
        long long a2,
        int *a3,
        long long a4,
        const char *a5,
        const char *a6,
        const char *a7)
{
  long long v9; // rax
  long long v10; // r15
  unsigned int v11; // r14d
  int v12; // edx
  long long v13; // rax
  long long v14; // r12
  long long v15; // r8
  long long v16; // rdx
  char v17; // al
  long long v19; // rax
  int v20; // r14d
  long long v21; // rax
  long long v22; // r8
  char v23; // r10
  long long v24; // rcx
  char v25; // r14
  long long v26; // r8
  char v27; // al
  char v28; // r14
  char v29; // si
  char v30; // al
  long long v31; // [rsp+0h] [rbp-58h]
  long long v32; // [rsp+0h] [rbp-58h]
  long long v33; // [rsp+0h] [rbp-58h]
  long long v34; // [rsp+0h] [rbp-58h]
  long long v35; // [rsp+0h] [rbp-58h]
  char v36; // [rsp+Ch] [rbp-4Ch]

  v9 = sub_4180(a2);
  if ( v9 )
  {
    v10 = v9;
    v11 = 0;
    while ( 1 )
    {
      while ( *(int *)v10 == 4 )
      {
LABEL_8:
        if ( byte_130ED )
          goto LABEL_6;
        v13 = v10 + 16LL * *a3;
        v14 = *(long long *)(v13 + 8);
        v15 = *(long long *)(v13 + 16);
        if ( byte_130EF )
        {
          v34 = *(long long *)(v13 + 16);
          __fprintf_chk(stream, 1, "%lda\n", v34);
          v25 = byte_130EB;
          v26 = v34;
          if ( byte_130EB )
          {
            __fprintf_chk(stream, 1, "||||||| %s\n", a6);
            v30 = sub_5030(stream, v10, a3[1]);
            v26 = v34;
            v25 = v30;
          }
          v35 = v26;
          fputs_unlocked("=======\n", stream);
          v27 = sub_5030(stream, v10, a3[2]);
          v22 = v35;
          v23 = v27 | v25;
          v20 = 4;
          goto LABEL_22;
        }
LABEL_10:
        if ( *(long long *)(v10 + 16LL * a3[2] + 16) - *(long long *)(v10 + 16LL * a3[2] + 8) == -1 )
        {
          if ( v14 == v15 )
            __fprintf_chk(stream, 1, "%ldd\n", v14);
          else
            __fprintf_chk(stream, 1, "%ld,%ldd\n", v14, v15);
          goto LABEL_6;
        }
        if ( v15 - v14 == -1 )
        {
          __fprintf_chk(stream, 1, "%lda\n", v15);
        }
        else if ( v15 == v14 )
        {
          __fprintf_chk(stream, 1, "%ldc\n", v15);
        }
        else
        {
          __fprintf_chk(stream, 1, "%ld,%ldc\n", v14, v15);
        }
        v16 = a3[2];
        v31 = *(long long *)(v10 + 16 * v16 + 16) - *(long long *)(v10 + 16 * v16 + 8) + 1LL;
        v17 = sub_5030(stream, v10, v16);
        sub_46D0(stream, v17, v14, v31);
        v10 = *(long long *)(v10 + 104);
        if ( !v10 )
          goto LABEL_15;
      }
      v12 = *(int *)(a4 + 4LL * (unsigned int)(*(int *)v10 - 5));
      switch ( v12 )
      {
        case 1:
          if ( byte_130EB )
          {
            v20 = 6;
            v21 = v10 + 16LL * *a3;
            v14 = *(long long *)(v21 + 8);
            v32 = *(long long *)(v21 + 16);
            __fprintf_chk(stream, 1, "%lda\n", v32);
            v22 = v32;
            v23 = 0;
LABEL_22:
            v36 = v23;
            v33 = v22;
            __fprintf_chk(stream, 1, ">>>>>>> %s\n", a7);
            sub_46D0(
              stream,
              v36,
              v33 + 2,
              *(long long *)(v10 + 16LL * a3[1] + 16)
            - *(long long *)(v10 + 16LL * a3[1] + 8)
            + *(long long *)(v10 + 16LL * a3[2] + 16)
            - *(long long *)(v10 + 16LL * a3[2] + 8)
            + 3LL);
            v24 = v14 - 1;
            if ( v20 == 4 )
            {
              __fprintf_chk(stream, 1, "%lda\n<<<<<<< %s\n", v24, a5);
              v29 = 0;
            }
            else
            {
              __fprintf_chk(stream, 1, "%lda\n<<<<<<< %s\n", v24, a6);
              v28 = sub_5030(stream, v10, a3[1]);
              fputs_unlocked("=======\n", stream);
              v29 = v28;
            }
            v11 = 1;
            sub_46D0(
              stream,
              v29,
              v14 + 1,
              *(long long *)(v10 + 16LL * a3[1] + 16) - *(long long *)(v10 + 16LL * a3[1] + 8) + 1LL);
          }
          break;
        case 2:
          if ( !byte_130EC )
          {
            v19 = v10 + 16LL * *a3;
            v14 = *(long long *)(v19 + 8);
            v15 = *(long long *)(v19 + 16);
            goto LABEL_10;
          }
          break;
        case -1:
          goto LABEL_8;
      }
LABEL_6:
      v10 = *(long long *)(v10 + 104);
      if ( !v10 )
        goto LABEL_15;
    }
  }
  v11 = 0;
LABEL_15:
  if ( byte_130EA )
    fputs_unlocked("w\nq\n", stream);
  return v11;
}



// Function: output_diff3 @ 0x5520
unsigned long long output_diff3(FILE *stream, long long a2, long long a3, long long a4)
{
  const char *v4; // rax
  long long v5; // r14
  int v6; // r15d
  long long v7; // rbp
  long long v8; // rax
  long long v9; // r12
  long long v10; // rbx
  long long v11; // rbx
  long long v12; // r12
  long long v13; // rax
  long long v14; // rbp
  long long v15; // r14
  char *v16; // rbx
  size_t v17; // r15
  char *v18; // rax
  long long v19; // rax
  long long v21; // [rsp+0h] [rbp-88h]
  long long v22; // [rsp+8h] [rbp-80h]
  int v23; // [rsp+14h] [rbp-74h]
  char *s; // [rsp+18h] [rbp-70h]
  long long v25; // [rsp+20h] [rbp-68h]
  int v27; // [rsp+30h] [rbp-58h]
  int v28; // [rsp+34h] [rbp-54h]
  char v30[2]; // [rsp+46h] [rbp-42h] BYREF
  unsigned long long v31; // [rsp+48h] [rbp-40h]

  v31 = __readfsqword(0x28u);
  v4 = "  ";
  if ( byte_130EE )
    v4 = "\t";
  s = (char *)v4;
  if ( a2 )
  {
    v5 = a2;
    do
    {
      if ( *(int *)v5 == 4 )
      {
        v30[0] = 0;
        v28 = 3;
        v27 = 3;
      }
      else
      {
        v19 = (unsigned int)(*(int *)v5 - 5);
        if ( (unsigned int)v19 > 2 )
          sub_4210("internal error: invalid diff type passed to output");
        v30[1] = 0;
        v27 = *(int *)(a4 + 4 * v19);
        v30[0] = v27 + 49;
        v28 = v27 == 0;
      }
      v6 = 0;
      __fprintf_chk(stream, 1, "====%s\n", v30);
      do
      {
        v25 = v6;
        v7 = *(int *)(a3 + 4LL * v6);
        v8 = v5 + 16 * v7;
        v9 = *(long long *)(v8 + 8);
        v10 = *(long long *)(v8 + 16);
        v23 = v6 + 1;
        __fprintf_chk(stream, 1, "%d:", v6 + 1);
        if ( v9 == v10 )
        {
          __fprintf_chk(stream, 1, "%ldc\n", v9);
        }
        else if ( v9 - v10 == 1 )
        {
          __fprintf_chk(stream, 1, "%lda\n", v9 - 1);
        }
        else
        {
          __fprintf_chk(stream, 1, "%ld,%ldc\n", v9, v10);
        }
        if ( v9 <= v10 && v28 != v6 )
        {
          v11 = v10 - v9;
          v12 = 0;
          v21 = v11;
          v22 = v7 + 6;
          v13 = v7;
          v14 = v5;
          v15 = v13;
          do
          {
            fputs_unlocked(s, stream);
            v16 = *(char **)(*(long long *)(v14 + 8 * v22 + 8) + 8 * v12);
            v17 = *(long long *)(*(long long *)(v14 + 8 * v15 + 80) + 8 * v12++);
            fwrite_unlocked(v16, 1u, v17, stream);
          }
          while ( v12 <= v21 );
          v5 = v14;
          if ( v16[v17 - 1] != 10 )
          {
            v18 = dcgettext(0, "No newline at end of file", 5);
            __fprintf_chk(stream, 1, "\n\\ %s\n", v18);
          }
        }
        v6 = v23;
        if ( v27 == 1 )
          v6 = dword_DD40[v25];
      }
      while ( v6 <= 2 );
      v5 = *(long long *)(v5 + 104);
    }
    while ( v5 );
  }
  return v31 - __readfsqword(0x28u);
}



// Function: output_diff3_merge @ 0x57d0
long long output_diff3_merge(
        FILE *stream,
        FILE *a2,
        int *a3,
        int *a4,
        long long a5,
        const char *a6,
        long long a7,
        const char *a8)
{
  int *v9; // r15
  int v11; // eax
  long long v12; // r12
  int v13; // edx
  long long v14; // rdx
  int v15; // eax
  int v16; // r14d
  bool v17; // cf
  long long v18; // rax
  long long v19; // r12
  size_t v20; // rdx
  const void *v21; // rdi
  long long v22; // r14
  int v23; // eax
  int v24; // r13d
  int v25; // r12d
  long long v27; // rdx
  bool v28; // sf
  long long v29; // rdx
  long long v30; // rax
  long long v31; // r12
  size_t v32; // rdx
  const void *v33; // rdi
  long long v34; // r12
  const void *v35; // rdi
  size_t v36; // rdx
  long long v37; // rdx
  char *v38; // rax
  const char *v39; // [rsp+8h] [rbp-60h]
  long long v40; // [rsp+10h] [rbp-58h]
  int v41; // [rsp+18h] [rbp-50h]
  char v42; // [rsp+1Eh] [rbp-4Ah]
  unsigned char v43; // [rsp+1Fh] [rbp-49h]

  if ( a3 )
  {
    v9 = a3;
    v43 = 0;
    v11 = *a3;
    v12 = 0;
    if ( *a3 == 4 )
      goto LABEL_8;
    while ( 1 )
    {
      v13 = *(int *)(a5 + 4LL * (unsigned int)(v11 - 5));
      switch ( v13 )
      {
        case 1:
          v42 = byte_130EB;
          if ( !byte_130EB )
            break;
          v29 = *((long long *)v9 + 1);
          v41 = 6;
          v40 = v29 - 1;
          v28 = v29 - v12 - 2 < 0;
          v39 = "<<<<<<< %s\n";
          v12 = v29 - v12 - 2;
          if ( v28 )
            goto LABEL_46;
LABEL_11:
          while ( 2 )
          {
            v15 = getc_unlocked(stream);
            v16 = v15;
            if ( v15 == -1 )
            {
              if ( ferror_unlocked(stream) )
                goto LABEL_56;
              if ( !feof_unlocked(stream) )
              {
                putc_unlocked(-1, a2);
                continue;
              }
LABEL_40:
              sub_4210("input file shrank");
LABEL_41:
              v42 = byte_130EC;
              if ( byte_130EC )
                goto LABEL_6;
              v27 = *((long long *)v9 + 1);
              v41 = 7;
              v40 = v27 - 1;
              v28 = v27 - v12 - 2 < 0;
              v39 = "<<<<<<< %s\n";
              v12 = v27 - v12 - 2;
              if ( !v28 )
                continue;
            }
            else
            {
              putc_unlocked(v15, a2);
              if ( v16 != 10 )
                continue;
              v17 = v12-- == 0;
              if ( !v17 )
                continue;
              if ( v42 )
              {
                if ( v41 == 4 )
                  goto LABEL_52;
                goto LABEL_16;
              }
            }
            break;
          }
LABEL_18:
          v18 = a4[2];
          if ( *(long long *)&v9[4 * v18 + 4] >= *(long long *)&v9[4 * v18 + 2] )
          {
            v19 = 0;
            do
            {
              v20 = *(long long *)(*(long long *)&v9[2 * v18 + 20] + 8 * v19);
              v21 = *(const void **)(*(long long *)&v9[2 * v18 + 14] + 8 * v19++);
              fwrite_unlocked(v21, 1u, v20, a2);
              v18 = a4[2];
            }
            while ( *(long long *)&v9[4 * v18 + 4] - *(long long *)&v9[4 * v18 + 2] >= v19 );
          }
          if ( v42 )
            __fprintf_chk(a2, 1, ">>>>>>> %s\n", a8);
          v22 = *((long long *)v9 + 2) - *((long long *)v9 + 1);
          v12 = v40 + v22 + 1;
          if ( v22 >= 0 )
          {
            while ( 1 )
            {
              do
              {
                while ( 1 )
                {
                  v23 = getc_unlocked(stream);
                  if ( v23 != 10 )
                    break;
                  v17 = v22-- == 0;
                  if ( v17 )
                    goto LABEL_6;
                }
              }
              while ( v23 != -1 );
              if ( ferror_unlocked(stream) )
                break;
              if ( feof_unlocked(stream) )
              {
                if ( !v22 && !*((long long *)v9 + 13) )
                  return v43;
                goto LABEL_40;
              }
            }
LABEL_56:
            v38 = dcgettext(0, "read failed", 5);
            sub_43A0(v38);
            goto LABEL_57;
          }
          break;
        case 2:
          goto LABEL_41;
        case -1:
LABEL_8:
          if ( !byte_130ED )
          {
            v14 = *((long long *)v9 + 1);
            v42 = byte_130EF;
            v40 = v14 - 1;
            v12 = v14 - v12 - 2;
            if ( v12 >= 0 )
            {
              v41 = 4;
              v39 = "||||||| %s\n";
              goto LABEL_11;
            }
            if ( !byte_130EF )
              goto LABEL_18;
            v39 = "||||||| %s\n";
LABEL_52:
            v34 = 0;
            __fprintf_chk(a2, 1, "<<<<<<< %s\n", a6);
            while ( 1 )
            {
              v37 = *a4;
              if ( *(long long *)&v9[4 * v37 + 4] - *(long long *)&v9[4 * v37 + 2] < v34 )
                break;
              v35 = *(const void **)(*(long long *)&v9[2 * v37 + 14] + 8 * v34);
              v36 = *(long long *)(*(long long *)&v9[2 * v37 + 20] + 8 * v34++);
              fwrite_unlocked(v35, 1u, v36, a2);
            }
LABEL_16:
            if ( !byte_130EB )
            {
LABEL_17:
              fputs_unlocked("=======\n", a2);
              v42 = 1;
              v43 = 1;
              goto LABEL_18;
            }
LABEL_46:
            __fprintf_chk(a2, 1, v39, a7);
            v30 = a4[1];
            if ( *(long long *)&v9[4 * v30 + 4] >= *(long long *)&v9[4 * v30 + 2] )
            {
              v31 = 0;
              do
              {
                v32 = *(long long *)(*(long long *)&v9[2 * v30 + 20] + 8 * v31);
                v33 = *(const void **)(*(long long *)&v9[2 * v30 + 14] + 8 * v31++);
                fwrite_unlocked(v33, 1u, v32, a2);
                v30 = a4[1];
              }
              while ( *(long long *)&v9[4 * v30 + 4] - *(long long *)&v9[4 * v30 + 2] >= v31 );
            }
            goto LABEL_17;
          }
          break;
      }
LABEL_6:
      v9 = (int *)*((long long *)v9 + 13);
      if ( !v9 )
        goto LABEL_30;
      v11 = *v9;
      if ( *v9 == 4 )
        goto LABEL_8;
    }
  }
LABEL_57:
  v43 = 0;
LABEL_30:
  while ( 1 )
  {
    v24 = getc_unlocked(stream);
    if ( v24 == -1 )
    {
      v25 = ferror_unlocked(stream);
      if ( feof_unlocked(stream) | v25 )
        break;
    }
    putc_unlocked(v24, a2);
  }
  return v43;
}



// Function: check_stdout @ 0x5c30
int check_stdout()
{
  int result; // eax
  char *v1; // rdi

  if ( ferror_unlocked(stdout) )
  {
    sub_4210("write failed");
    goto LABEL_5;
  }
  result = fclose(stdout);
  if ( result )
  {
LABEL_5:
    v1 = dcgettext(0, "standard output", 5);
    sub_43A0(v1);
    return sub_5C80();
  }
  return result;
}



// Function: process_diff @ 0x5d70
long long process_diff(const char *a1, const char *a2, long long *a3, char **a4)
{
  long long *v4; // r15
  long long *v6; // r13
  unsigned long long v7; // rbp
  char *v8; // rax
  long long *v9; // rax
  int v10; // ebx
  char *v11; // r10
  long long v12; // r12
  long long v13; // r14
  long long v14; // rax
  long long v15; // rdx
  char *v16; // r10
  long long i; // r14
  char *v18; // rbx
  long long v19; // rbx
  long long v20; // r12
  long long v21; // rax
  char *v22; // rdi
  long long v23; // rdx
  long long j; // r12
  char *v26; // rdi
  long long v27; // r12
  char *v28; // rax
  char *v29; // rbx
  bool v30; // zf
  char *v31; // [rsp+10h] [rbp-68h]
  char *v33; // [rsp+20h] [rbp-58h] BYREF
  char *s1; // [rsp+28h] [rbp-50h] BYREF
  long long v35[9]; // [rsp+30h] [rbp-48h] BYREF

  v6 = v35;
  v35[1] = __readfsqword(0x28u);
  v7 = sub_43D0(a1, a2, (long long *)&v33);
  v8 = v33;
  *a4 = v33;
  s1 = v8;
  if ( (unsigned long long)v8 >= v7 )
  {
LABEL_21:
    *v6 = 0;
    *a3 = v4;
    return v35[0];
  }
  while ( 1 )
  {
    while ( 1 )
    {
      v9 = (long long *)sub_8180(72);
      v9[5] = 0;
      v4 = v9;
      v9[4] = 0;
      v9[7] = 0;
      v9[6] = 0;
      v10 = sub_5C80(&s1, v9);
      if ( !v10 || (v11 = s1 + 1, *s1 != 10) )
      {
        v27 = qword_23140;
        v28 = dcgettext(0, "%s: diff failed: ", 5);
        __fprintf_chk(stderr, 1, v28, v27);
        do
        {
          v29 = s1;
          putc_unlocked(*s1, stderr);
          v30 = *v29 == 10;
          s1 = v29 + 1;
        }
        while ( !v30 );
        exit(2);
      }
      ++s1;
      if ( v10 == 2 )
        break;
      if ( v10 == 3 )
      {
        ++v4[2];
        break;
      }
      if ( v10 == 1 )
      {
        ++*v4;
        goto LABEL_15;
      }
      sub_4210("internal error: invalid diff type in process_diff");
LABEL_24:
      if ( v10 != 3 )
        goto LABEL_15;
      v26 = s1;
      *v6 = v4;
      v6 = v4 + 8;
      if ( v7 <= (unsigned long long)v26 )
        goto LABEL_21;
    }
    v12 = v4[1] - *v4;
    v13 = v12 + 1;
    if ( (unsigned long long)(v12 + 1) > 0xFFFFFFFFFFFFFFELL )
      goto LABEL_31;
    v31 = v11;
    v4[4] = sub_8180(8 * v13);
    v14 = sub_8180(8 * v13);
    v4[6] = v14;
    v15 = v14;
    if ( v12 != -1 )
    {
      v16 = v31;
      for ( i = 0; ; ++i )
      {
        s1 = (char *)sub_4770(v16, (long long *)(v4[4] + 8 * i), (long long *)(8 * i + v15), v7, 60);
        v16 = s1;
        if ( v12 == i )
          break;
        v15 = v4[6];
      }
    }
    if ( v10 != 2 )
      goto LABEL_24;
    v18 = s1;
    if ( strncmp(s1, "---\n", 4u) )
      return sub_4210("invalid diff format; invalid change separator");
    s1 = v18 + 4;
LABEL_15:
    v19 = v4[3] - v4[2];
    v20 = v19 + 1;
    if ( (unsigned long long)(v19 + 1) > 0xFFFFFFFFFFFFFFELL )
LABEL_31:
      sub_8530();
    v4[5] = sub_8180(8 * v20);
    v21 = sub_8180(8 * v20);
    v22 = s1;
    v4[7] = v21;
    v23 = v21;
    if ( v19 != -1 )
    {
      for ( j = 0; ; ++j )
      {
        s1 = (char *)sub_4770(v22, (long long *)(v4[5] + 8 * j), (long long *)(8 * j + v23), v7, 62);
        v22 = s1;
        if ( v19 == j )
          break;
        v23 = v4[7];
      }
    }
    *v6 = v4;
    v6 = v4 + 8;
    if ( v7 <= (unsigned long long)v22 )
      goto LABEL_21;
  }
}


