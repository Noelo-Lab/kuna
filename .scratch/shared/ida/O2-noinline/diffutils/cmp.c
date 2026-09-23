// Function: main @ 0x2940
long long main(int a1, char **a2, char **a3)
{
  char **v4; // rbx
  char *v5; // rdi
  int v6; // eax
  char *v7; // rax
  int v9; // ebx
  long long v10; // rax
  int v11; // eax
  long long v12; // rsi
  long long v13; // rcx
  char *v14; // rcx
  char *v15; // rax
  char *v16; // rax
  long long v17; // rax
  long long i; // rbx
  const char *v19; // r15
  int v20; // edi
  int v21; // r15d
  int v22; // eax
  int v23; // r12d
  long long v24; // rbx
  int v25; // eax
  int v26; // eax
  int *v27; // rax
  long long v28; // r12
  long long v29; // rax
  long long v30; // rbx
  long long v31; // r12
  long long v32; // r8
  long long v33; // rax
  long long v34; // rdx
  long long v35; // rax
  int *v36; // rax
  const char *v37; // rsi
  long long v38; // [rsp+18h] [rbp-170h] BYREF
  struct stat buf; // [rsp+20h] [rbp-168h] BYREF
  struct stat v40; // [rsp+B0h] [rbp-D8h] BYREF
  unsigned long long v41; // [rsp+148h] [rbp-40h]

  v4 = a2;
  v5 = *a2;
  v41 = __readfsqword(0x28u);
  status = 2;
  sub_4180(v5);
  setlocale(6, "");
  bindtextdomain("diffutils", "/usr/local/share/locale");
  textdomain("diffutils");
  sub_3F90(0);
  sub_64F0();
  while ( 2 )
  {
    v6 = getopt_long(a1, a2, "bci:ln:sv", &longopts, 0);
    if ( v6 != -1 )
    {
      switch ( v6 )
      {
        case 98:
        case 99:
          byte_D0B2 = 1;
          continue;
        case 105:
          sub_32D0(0, &optarg, 58);
          v7 = optarg++;
          if ( *v7 == 58 )
          {
            sub_32D0(1, &optarg, 0);
          }
          else if ( qword_D0C8 < qword_D0C0 )
          {
            qword_D0C8 = qword_D0C0;
          }
          continue;
        case 108:
          sub_32A0(1);
          continue;
        case 110:
          if ( (unsigned int)sub_6640(optarg) || v38 < 0 )
            sub_3240("invalid --bytes value '%s'");
          if ( qword_D010 < 0 || v38 <= qword_D010 )
            qword_D010 = v38;
          continue;
        case 115:
          sub_32A0(3);
          continue;
        case 118:
          v9 = sub_4490("David MacKenzie");
          v10 = sub_4520("Torbjorn Granlund", aTorbj);
          sub_5F90(
            (int)stdout,
            (unsigned int)"cmp",
            (unsigned int)"GNU diffutils",
            (int)off_D018,
            v10,
            v9,
            0,
            v10);
          sub_3D80();
          return 0;
        case 128:
          sub_3370();
          sub_3D80();
          return 0;
        default:
          v12 = 0;
          v11 = sub_3240(0);
          goto LABEL_45;
      }
    }
    break;
  }
  v11 = optind;
  if ( optind == a1 )
    goto LABEL_94;
  v12 = optind;
  v13 = (long long)v4[v12];
  ++optind;
  qword_D230[0] = v13;
  v14 = (char *)"-";
  if ( v11 + 1 < a1 )
  {
LABEL_45:
    v14 = v4[v12 + 1];
    optind = v11 + 2;
  }
  s2 = v14;
  if ( a1 > optind )
  {
    v15 = v4[optind++];
    v38 = (long long)v15;
    sub_32D0(0, &v38, 0);
    if ( optind < a1 )
    {
      v16 = v4[optind++];
      v38 = (long long)v16;
      sub_32D0(1, &v38, 0);
      v17 = optind;
      if ( optind < a1 )
      {
LABEL_95:
        v37 = v4[v17];
        sub_3240("extra operand '%s'");
        start("extra operand '%s'", v37);
      }
    }
  }
  for ( i = 0; ; i = 1 )
  {
    if ( i && qword_D0C0 == qword_D0C8 && !strcmp((const char *)qword_D230[0], s2) )
      return 0;
    v19 = (const char *)qword_D230[i];
    v20 = strcmp(v19, "-");
    if ( v20 )
    {
      v25 = open(v19, 0, 0);
      *(&fd + i) = v25;
      v20 = v25;
      if ( v25 < 0 )
      {
        if ( dword_D0B4 == 3 )
          exit(2);
LABEL_76:
        v27 = __errno_location();
        v21 = i;
        error(2, *v27, "%s", (const char *)qword_D230[(int)i]);
        goto LABEL_77;
      }
    }
    else
    {
      *(&fd + i) = 0;
    }
    v21 = fstat(v20, (struct stat *)&unk_D100 + i);
    if ( v21 )
      goto LABEL_76;
    if ( i == 1 )
      break;
  }
  if ( qword_D108 != qword_D198 || unk_D100 != qword_D190 )
  {
    v22 = dword_D118 & 0xF000;
    if ( v22 == 24576 )
    {
      if ( (dword_D1A8 & 0xF000) == 0x6000 )
      {
LABEL_67:
        if ( qword_D128 == qword_D1B8 )
          goto LABEL_47;
LABEL_34:
        if ( dword_D0B4 == 3 )
        {
LABEL_35:
          if ( (dword_D118 & 0xF000) != 0x8000 || (dword_D1A8 & 0xF000) != 0x8000 )
            goto LABEL_37;
LABEL_77:
          v28 = qword_D130;
          v29 = sub_34C0(0);
          v30 = qword_D1C0;
          v31 = v28 - v29;
          v32 = sub_34C0(1);
          v33 = 0;
          if ( v31 >= 0 )
            v33 = v31;
          v34 = v33;
          v35 = v30 - v32;
          if ( v30 - v32 < 0 )
            v35 = 0;
          if ( v35 != v34 )
          {
            if ( qword_D010 < 0 )
              goto LABEL_86;
            if ( v35 > v34 )
              v35 = v34;
            if ( qword_D010 > v35 )
LABEL_86:
              exit(1);
          }
          goto LABEL_37;
        }
        goto LABEL_58;
      }
    }
    else
    {
      if ( v22 != 0x2000 )
        goto LABEL_34;
      if ( (dword_D1A8 & 0xF000) == 0x2000 )
        goto LABEL_67;
    }
    if ( dword_D0B4 == 3 )
      goto LABEL_37;
LABEL_58:
    if ( !fstat(1, &buf) && !stat("/dev/null", &v40) )
    {
      if ( buf.st_ino == v40.st_ino && buf.st_dev == v40.st_dev )
      {
LABEL_75:
        dword_D0B4 = 2;
        goto LABEL_59;
      }
      v26 = buf.st_mode & 0xF000;
      if ( v26 == 24576 )
      {
        if ( (v40.st_mode & 0xF000) == 0x6000 )
          goto LABEL_74;
      }
      else if ( v26 == 0x2000 && (v40.st_mode & 0xF000) == 0x2000 )
      {
LABEL_74:
        if ( buf.st_rdev != v40.st_rdev )
          goto LABEL_59;
        goto LABEL_75;
      }
    }
LABEL_59:
    if ( dword_D0B4 == 3 )
      goto LABEL_35;
LABEL_37:
    qword_D0D0 = sub_6AC0(qword_D138, qword_D1C8, 0x7FFFFFFFFFFFFFF7LL);
    v4 = (char **)((qword_D0D0 + 15) & 0xFFFFFFFFFFFFFFF8LL);
    s1 = (void *)sub_6100(16 * ((unsigned long long)(qword_D0D0 + 15) >> 3));
    qword_D0E8 = (char *)s1 + (long long)v4;
    v23 = sub_35E0();
    if ( !close(fd) )
    {
      if ( !close(dword_D224) )
      {
        if ( v23 && (unsigned int)dword_D0B4 <= 1 )
          sub_3D80();
        exit(v23);
      }
      v21 = 1;
    }
    v36 = __errno_location();
    error(2, *v36, "%s", (const char *)qword_D230[v21]);
LABEL_94:
    v17 = sub_3240("missing operand after '%s'");
    goto LABEL_95;
  }
LABEL_47:
  if ( dword_D118 != dword_D1A8 )
    goto LABEL_34;
  if ( qword_D110 != qword_D1A0 )
    goto LABEL_34;
  if ( dword_D11C != dword_D1AC )
    goto LABEL_34;
  if ( dword_D120 != dword_D1B0 )
    goto LABEL_34;
  if ( qword_D130 != qword_D1C0 )
    goto LABEL_34;
  if ( qword_D158 != qword_D1E8 )
    goto LABEL_34;
  if ( qword_D168 != qword_D1F8 )
    goto LABEL_34;
  v24 = sub_34C0(0);
  if ( v24 != sub_34C0(1) )
    goto LABEL_34;
  return 0;
}



// Function: block_compare @ 0x31f0
signed long long block_compare(long long *a1, long long *a2)
{
  long long *v2; // rax

  v2 = a1;
  if ( *a1 == *a2 )
  {
    do
    {
      ++v2;
      ++a2;
    }
    while ( *v2 == *a2 );
    if ( *(char *)v2 == *(char *)a2 )
      goto LABEL_6;
  }
  else
  {
    while ( *(char *)v2 == *(char *)a2 )
    {
LABEL_6:
      v2 = (long long *)((char *)v2 + 1);
      a2 = (long long *)((char *)a2 + 1);
    }
  }
  return (char *)v2 - (char *)a1;
}



// Function: try_help @ 0x3240
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
  v3 = qword_1D280;
  v4 = dcgettext(0, "Try '%s --help' for more information.", 5);
  error(2, 0, v4, v3);
  return sub_32A0(2);
}



// Function: specify_comparison_type @ 0x32a0
long long specify_comparison_type(int a1)
{
  long long result; // rax
  long long v2; // rdx

  result = (unsigned int)dword_D0B4;
  if ( dword_D0B4 && dword_D0B4 != a1 )
  {
    sub_3240("options -l and -s are incompatible", 0);
    return sub_32D0("options -l and -s are incompatible", 0, v2);
  }
  else
  {
    dword_D0B4 = a1;
  }
  return result;
}



// Function: specify_ignore_initial @ 0x32d0
unsigned long long specify_ignore_initial(int a1, char **a2, char a3)
{
  char *v4; // r13
  int v5; // eax
  long long v7; // [rsp+0h] [rbp-38h]
  unsigned long long v8; // [rsp+8h] [rbp-30h]

  v4 = *a2;
  v8 = __readfsqword(0x28u);
  v5 = sub_6640(*a2);
  if ( v5 && (v5 != 2 || **a2 != a3) || v7 < 0 )
    sub_3240("invalid --ignore-initial value '%s'", (long long)v4);
  if ( v7 > qword_D0C0[a1] )
    qword_D0C0[a1] = v7;
  return v8 - __readfsqword(0x28u);
}



// Function: usage @ 0x3370
long long usage()
{
  long long v0; // r12
  const char **v1; // rbx
  char *v2; // rax
  char *v3; // rax
  char *v4; // rax
  FILE *v5; // rbp
  char *v6; // rax
  const char *v7; // rsi
  char *v8; // rax
  char *v9; // rbx
  char *v10; // r12
  char *v11; // rax

  v0 = qword_1D280;
  v1 = (const char **)off_C9E0;
  v2 = dcgettext(0, "Usage: %s [OPTION]... FILE1 [FILE2 [SKIP1 [SKIP2]]]\n", 5);
  __printf_chk(1, v2, v0);
  v3 = dcgettext(0, "Compare two files byte by byte.", 5);
  __printf_chk(1, "%s\n", v3);
  v4 = dcgettext(
         0,
         "The optional SKIP1 and SKIP2 specify the number of bytes to skip\n"
         "at the beginning of each file (zero by default).",
         5);
  __printf_chk(1, "\n%s\n\n", v4);
  v5 = stdout;
  v6 = dcgettext(0, "Mandatory arguments to long options are mandatory for short options too.\n", 5);
  fputs_unlocked(v6, v5);
  v7 = "-b, --print-bytes          print differing bytes";
  do
  {
    ++v1;
    v8 = dcgettext(0, v7, 5);
    __printf_chk(1, "  %s\n", v8);
    v7 = *v1;
  }
  while ( *v1 );
  v9 = dcgettext(0, "Exit status is 0 if inputs are the same, 1 if different, 2 if trouble.", 5);
  v10 = dcgettext(0, "If a FILE is '-' or missing, read standard input.", 5);
  v11 = dcgettext(
          0,
          "SKIP values may be followed by the following multiplicative suffixes:\n"
          "kB 1000, K 1024, MB 1,000,000, M 1,048,576,\n"
          "GB 1,000,000,000, G 1,073,741,824, and so on for T, P, E, Z, Y.",
          5);
  __printf_chk(1, "\n%s\n\n%s\n%s\n", v11, v10, v9);
  return sub_6040();
}



// Function: file_position @ 0x34c0
__off_t file_position(int a1)
{
  __off_t result; // rax

  if ( byte_D0B0[a1] )
    return qword_D0A0[a1];
  byte_D0B0[a1] = 1;
  result = lseek(*(&fd + a1), qword_D0C0[a1], 1);
  qword_D0A0[a1] = result;
  return result;
}



// Function: count_newlines @ 0x3510
long long count_newlines(char *a1, long long a2)
{
  long long v2; // r12
  char *v3; // rbx
  char v4; // bp
  char *v5; // rax

  v2 = 0;
  v3 = &a1[a2];
  v4 = a1[a2];
  a1[a2] = 10;
  while ( 1 )
  {
    v5 = (char *)rawmemchr(a1, 10);
    if ( v3 == v5 )
      break;
    ++v2;
    a1 = v5 + 1;
  }
  *v3 = v4;
  return v2;
}



// Function: sprintc @ 0x3550
long long sprintc(short *a1, unsigned int a2)
{
  unsigned int v2; // r12d
  char *v3; // rbp
  const unsigned short **v4; // r8
  long long result; // rax

  v2 = a2;
  v3 = a1;
  v4 = __ctype_b_loc();
  result = a2;
  if ( ((*v4)[(unsigned char)a2] & 0x4000) == 0 )
  {
    if ( (a2 & 0x80u) != 0 )
    {
      v2 = a2 - 128;
      v3 = a1 + 1;
      *a1 = 11597;
      if ( (unsigned char)(a2 + 0x80) <= 0x1Fu )
        goto LABEL_4;
    }
    else if ( (unsigned char)a2 <= 0x1Fu )
    {
LABEL_4:
      *v3 = 94;
      result = v2 + 64;
      ++v3;
      goto LABEL_5;
    }
    result = v2;
    if ( (char)v2 == 127 )
    {
      *v3 = 94;
      result = 63;
      ++v3;
    }
  }
LABEL_5:
  *v3 = result;
  v3[1] = 0;
  return result;
}



// Function: cmp @ 0x35e0
long long cmp()
{
  unsigned char *v0; // r12
  unsigned char *v1; // rbx
  long long i; // r15
  unsigned long long v3; // rbp
  unsigned long long v4; // r13
  unsigned long long v5; // r14
  unsigned long long v6; // rax
  unsigned long long v7; // r15
  size_t v8; // r14
  unsigned char *v9; // rax
  unsigned char *v10; // rdx
  char v11; // al
  unsigned int v12; // eax
  long long v13; // rdx
  unsigned int v14; // eax
  long long v15; // rdx
  unsigned long long v16; // rax
  unsigned long long v17; // rdx
  long long v18; // rcx
  unsigned long long v19; // r13
  long long v20; // r14
  unsigned char *v21; // rbp
  unsigned char v22; // r12
  unsigned int v23; // r15d
  const char *v24; // rax
  long long v25; // rax
  long long v27; // r14
  long long v28; // rax
  int *v29; // rax
  long long v30; // rbp
  long long v31; // rdx
  long long v32; // r13
  char *v33; // rax
  const char *v34; // rsi
  long long v35; // rbp
  long long v36; // rax
  long long v37; // r12
  long long v38; // rbx
  long long v39; // r15
  char *v40; // rax
  long long v41; // r13
  long long v42; // rax
  long long v43; // r13
  long long v44; // rax
  long long v45; // rdx
  long long v46; // r12
  long long v47; // rdx
  char *v48; // rdx
  char *v49; // rax
  char *v50; // rax
  int *v51; // rax
  int *v52; // rax
  unsigned long long v53; // [rsp-10h] [rbp-F8h]
  long long v54; // [rsp+8h] [rbp-E0h]
  long long v55; // [rsp+8h] [rbp-E0h]
  char *v56; // [rsp+8h] [rbp-E0h]
  int v57; // [rsp+10h] [rbp-D8h]
  unsigned long long v58; // [rsp+10h] [rbp-D8h]
  int v59; // [rsp+1Ch] [rbp-CCh]
  long long v60; // [rsp+20h] [rbp-C8h]
  size_t v61; // [rsp+28h] [rbp-C0h]
  const char *v62; // [rsp+38h] [rbp-B0h]
  bool v63; // [rsp+47h] [rbp-A1h]
  unsigned char *v64; // [rsp+48h] [rbp-A0h]
  unsigned long long v65; // [rsp+50h] [rbp-98h]
  unsigned long long v66; // [rsp+58h] [rbp-90h]
  short v67[2]; // [rsp+66h] [rbp-82h] BYREF
  short v68[2]; // [rsp+6Bh] [rbp-7Dh] BYREF
  short v69[16]; // [rsp+70h] [rbp-78h] BYREF
  char v70[24]; // [rsp+90h] [rbp-58h] BYREF
  unsigned long long v71; // [rsp+A8h] [rbp-40h]

  v0 = (unsigned char *)s1;
  v1 = (unsigned char *)qword_D0E8;
  v71 = __readfsqword(0x28u);
  v54 = qword_D010;
  if ( dword_D0B4 == 1 )
  {
    v35 = 0x7FFFFFFFFFFFFFFFLL;
    if ( qword_D010 >= 0 )
      v35 = qword_D010;
    if ( (dword_D118 & 0xF000) == 0x8000 )
    {
      v43 = qword_D130;
      v44 = v43 - sub_34C0(0);
      if ( v35 > v44 )
        v35 = v44;
    }
    if ( (dword_D1A8 & 0xF000) == 0x8000 )
    {
      v41 = qword_D1C0;
      v42 = v41 - sub_34C0(1);
      if ( v35 > v42 )
        v35 = v42;
    }
    v59 = 1;
    v36 = v35;
    while ( 1 )
    {
      v36 /= 10;
      if ( !v36 )
        break;
      ++v59;
    }
  }
  for ( i = 0; ; i = 1 )
  {
    v3 = qword_D0C0[i];
    if ( v3 && sub_34C0(i) == -1 )
    {
      while ( 1 )
      {
        v27 = qword_D0D0;
        if ( v3 <= qword_D0D0 )
          v27 = v3;
        v28 = sub_6A10(*(&fd + i), v0);
        if ( v27 != v28 )
          break;
        v3 -= v27;
        if ( !v3 )
          goto LABEL_4;
      }
      if ( v28 == -1 )
      {
        v29 = __errno_location();
        error(2, *v29, "%s", (const char *)qword_D230[(int)i]);
LABEL_57:
        v55 = v18;
        v30 = sub_40E0(v3, v69, v17);
        v32 = sub_40E0(v60, v70, v31);
        if ( byte_D0B2 )
        {
          v37 = v0[v55];
          v38 = v1[v55];
          sub_3550(v67, v37);
          sub_3550(v68, v38);
          v39 = qword_D230[0];
          v56 = s2;
          v40 = dcgettext(0, "%s %s differ: byte %s, line %s is %3o %s %3o %s\n", 5);
          __printf_chk(1, v40, v39, v56, v30, v32, v37, v67, v38, v68);
        }
        else
        {
          v33 = dcgettext(0, "%s %s differ: byte %s, line %s\n", 5);
          v34 = v33;
          if ( v33 != "%s %s differ: byte %s, line %s\n" || (v34 = v33, (unsigned char)sub_4050(5, v33)) )
            __printf_chk(1, v34, qword_D230[0], s2, v30, v32);
          else
            __printf_chk(1, "%s %s differ: char %s, line %s\n", qword_D230[0], s2, v30, v32);
        }
        return 1;
      }
    }
LABEL_4:
    if ( i == 1 )
      break;
  }
  v63 = 1;
  v3 = 1;
  v57 = 0;
  v60 = 1;
  v4 = qword_D0D0;
  while ( 1 )
  {
    v5 = v4;
    if ( v54 >= 0 )
    {
      if ( v54 <= v4 )
        v5 = v54;
      v54 -= v5;
    }
    v4 = sub_6A10(fd, v0);
    if ( v4 == -1 )
      goto LABEL_88;
    v6 = sub_6A10(dword_D224, v1);
    v7 = v6;
    if ( v6 == -1 )
    {
      v51 = __errno_location();
      error(2, *v51, "%s", s2);
LABEL_88:
      v52 = __errno_location();
      error(2, *v52, "%s", (const char *)qword_D230[0]);
    }
    v8 = v6;
    if ( v4 <= v6 )
      v8 = v4;
    if ( !memcmp(v0, v1, v8) )
    {
      v3 += v8;
      if ( dword_D0B4 || !v8 )
        goto LABEL_37;
      v18 = v8;
LABEL_43:
      v61 = v18;
      v25 = sub_3510(v0, v18);
      v18 = v61;
      v60 += v25;
      if ( v61 < v8 )
        goto LABEL_57;
      v63 = v0[v61 - 1] == 10;
      goto LABEL_37;
    }
    v9 = &v1[v4];
    v10 = &v0[v7];
    if ( v4 < v7 || (*v9 = 85, v4 == v7) )
    {
      *v10 = 121;
      v11 = ~*v9;
    }
    else
    {
      v11 = -86;
    }
    v0[v4] = v11;
    v1[v7] = ~*v10;
    if ( (~(char)v4 & 7) != 0 )
    {
      v12 = 0;
      do
      {
        v13 = v12++;
        v0[v4 + 1 + v13] = 0;
      }
      while ( v12 < (~(char)v4 & 7u) );
    }
    if ( (~(char)v7 & 7) != 0 )
    {
      v14 = 0;
      do
      {
        v15 = v14++;
        v1[v7 + 1 + v15] = 0;
      }
      while ( v14 < (~(char)v7 & 7u) );
    }
    v16 = sub_31F0(v0, v1);
    v17 = v3;
    v18 = v16;
    v3 += v16;
    if ( v16 && !dword_D0B4 )
      goto LABEL_43;
    if ( v8 <= v16 )
      goto LABEL_37;
    if ( dword_D0B4 == 2 )
    {
      v57 = 1;
      if ( v4 == v7 )
        return v57 != 0;
      return 1;
    }
    if ( (unsigned int)dword_D0B4 <= 2 )
    {
      if ( !dword_D0B4 )
        goto LABEL_57;
      v65 = v4;
      v19 = v3;
      v58 = v17 + v8;
      v20 = -(long long)v17;
      v64 = v0;
      v66 = v7;
      v21 = &v0[-v17];
      while ( 1 )
      {
LABEL_33:
        v22 = v21[v19];
        v23 = v1[v20 + v19];
        if ( v22 == (char)v23 )
          goto LABEL_32;
        v24 = (const char *)sub_40E0(v19, v70, v17);
        if ( !byte_D0B2 )
          break;
        v62 = v24;
        ++v19;
        sub_3550(v68, v22);
        sub_3550(v69, v23);
        __printf_chk(1, "%*s %3o %-4s %3o %s\n", v59, v62, v22, (const char *)v68, v23, (const char *)v69);
        v17 = v53;
        if ( v58 == v19 )
        {
LABEL_36:
          v57 = -1;
          v3 = v19;
          v0 = v64;
          v4 = v65;
          v7 = v66;
          goto LABEL_37;
        }
      }
      __printf_chk(1, "%*s %3o %3o\n", v59, v24, v22, v23);
LABEL_32:
      if ( v58 == ++v19 )
        goto LABEL_36;
      goto LABEL_33;
    }
    if ( dword_D0B4 == 3 )
      return 1;
LABEL_37:
    if ( v4 != v7 )
      break;
    if ( qword_D0D0 != v4 )
      return v57 != 0;
  }
  if ( dword_D0B4 == 3 )
    return 1;
  v45 = v4 > v7;
  v46 = qword_D230[v45];
  if ( v3 != 1 )
  {
    sub_40E0(v3 - 1, v69, v45);
    if ( dword_D0B4 )
    {
      v50 = dcgettext(0, "cmp: EOF on %s after byte %s\n", 5);
      __fprintf_chk(stderr, 1, v50, v46);
    }
    else
    {
      sub_40E0(v60 - v63, v70, v47);
      if ( v63 )
        v48 = dcgettext(0, "cmp: EOF on %s after byte %s, line %s\n", 5);
      else
        v48 = dcgettext(0, "cmp: EOF on %s after byte %s, in line %s\n", 5);
      __fprintf_chk(stderr, 1, v48, v46);
    }
    return 1;
  }
  v49 = dcgettext(0, "cmp: EOF on %s which is empty\n", 5);
  __fprintf_chk(stderr, 1, v49, v46);
  return 1;
}



// Function: check_stdout @ 0x3d80
void check_stdout()
{
  char *v0; // rax
  char *v1; // r12
  int *v2; // rax

  if ( ferror_unlocked(stdout) )
  {
    v0 = dcgettext(0, "write failed", 5);
    error(2, 0, "%s", v0);
    goto LABEL_5;
  }
  if ( fclose(stdout) )
  {
LABEL_5:
    v1 = dcgettext(0, "standard output", 5);
    v2 = __errno_location();
    error(2, *v2, "%s", v1);
  }
}


