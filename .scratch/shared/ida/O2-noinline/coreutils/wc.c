// Function: main @ 0x28a0
long long main(int a1, char **a2, char **a3)
{
  int *v3; // rbx
  const char *v4; // r15
  const char *v5; // r14
  char **v6; // r13
  FILE *v7; // rbp
  char *v8; // rdi
  char *v9; // rax
  char *v10; // r12
  char **v11; // rsi
  long long v12; // rax
  int v13; // eax
  int v14; // eax
  char **v15; // r13
  char **v16; // rax
  char *v17; // rax
  int *v18; // rax
  char *v19; // rax
  long long v20; // r13
  const char *v21; // rax
  int *v22; // rax
  char *v23; // rax
  double v25; // xmm1_8
  long long v26; // r12
  char *v27; // rax
  char *v28; // rax
  long long v29; // r12
  char *v30; // rax
  long long v31; // [rsp+0h] [rbp-228h]
  double st_size_low; // [rsp+0h] [rbp-228h]
  char v33; // [rsp+8h] [rbp-220h]
  long long v34; // [rsp+8h] [rbp-220h]
  char *v35; // [rsp+8h] [rbp-220h]
  int *ptr; // [rsp+10h] [rbp-218h]
  char v37; // [rsp+1Fh] [rbp-209h]
  int v38; // [rsp+2Ch] [rbp-1FCh] BYREF
  long long v39; // [rsp+30h] [rbp-1F8h] BYREF
  char **v40; // [rsp+38h] [rbp-1F0h]
  struct stat buf; // [rsp+150h] [rbp-D8h] BYREF
  unsigned long long v42; // [rsp+1E8h] [rbp-40h]

  v4 = (const char *)&longopts;
  v5 = "clLmw";
  v6 = a2;
  LODWORD(v7) = a1;
  v8 = *a2;
  v42 = __readfsqword(0x28u);
  sub_4EA0(v8);
  setlocale(6, "");
  bindtextdomain("coreutils", "/usr/local/share/locale");
  textdomain("coreutils");
  sub_8CD0(sub_4870);
  qword_E120 = getpagesize();
  setvbuf(stdout, 0, 1, 0);
  v9 = getenv("POSIXLY_CORRECT");
  byte_E131 = 0;
  byte_E132 = 0;
  byte_E133 = 0;
  byte_E134 = 0;
  byte_E130 = 0;
  qword_E138 = 0;
  qword_E140 = 0;
  qword_E148 = 0;
  qword_E150 = 0;
  qword_E158 = 0;
  byte_E118 = v9 != 0;
  v10 = 0;
  while ( 2 )
  {
    v11 = v6;
    LODWORD(v12) = getopt_long((int)v7, v6, "clLmw", &longopts, 0);
    if ( (int)v12 != -1 )
    {
      if ( (int)v12 <= 129 )
      {
        if ( (int)v12 > 75 )
        {
          switch ( (int)v12 )
          {
            case 76:
              byte_E130 = 1;
              continue;
            case 99:
              byte_E131 = 1;
              continue;
            case 108:
              byte_E134 = 1;
              continue;
            case 109:
              byte_E132 = 1;
              continue;
            case 119:
              byte_E133 = 1;
              continue;
            case 128:
              byte_E160 = 1;
              continue;
            case 129:
              v10 = (char *)optarg;
              continue;
            default:
              goto LABEL_74;
          }
        }
        if ( (int)v12 == -131 )
        {
          sub_7D20(
            (int)stdout,
            (unsigned int)"wc",
            (unsigned int)"GNU coreutils",
            (int)off_E018,
            (unsigned int)"Paul Rubin",
            (unsigned int)"David MacKenzie",
            0,
            v12);
          exit(0);
        }
        if ( (int)v12 == -130 )
          sub_45C0(0);
      }
LABEL_74:
      sub_45C0(1);
    }
    break;
  }
  if ( !byte_E134 && !byte_E133 && !byte_E132 && !byte_E131 && !byte_E130 )
  {
    byte_E131 = 1;
    byte_E133 = 1;
    byte_E134 = 1;
  }
  v13 = optind;
  if ( v10 )
  {
    if ( (int)v7 > optind )
    {
      v26 = sub_7110(4, v6[optind]);
      v27 = dcgettext(0, "extra operand %s", 5);
      error(0, 0, v27, v26);
      v28 = dcgettext(0, "file operands cannot be combined with --files0-from", 5);
      __fprintf_chk(stderr, 1, "%s\n", v28);
      goto LABEL_74;
    }
    if ( !strcmp(v10, "-") )
    {
      v7 = stdin;
    }
    else
    {
      v7 = fopen(v10, "r");
      if ( !v7 )
      {
        v6 = (char **)sub_7110(4, v10);
        v10 = dcgettext(0, "cannot open %s for reading", 5);
        v18 = __errno_location();
        error(1, *v18, v10, v6);
        goto LABEL_45;
      }
    }
    v14 = fileno(v7);
    if ( fstat(v14, &buf) || (buf.st_mode & 0xF000) != 0x8000 )
      goto LABEL_23;
    st_size_low = (double)SLODWORD(buf.st_size);
    v25 = 10485760.0;
    if ( sub_4D60() * 0.5 <= 10485760.0 )
      v25 = 0.5 * sub_4D60();
    if ( v25 < st_size_low )
    {
LABEL_23:
      v15 = 0;
      v37 = 0;
      v31 = 0;
      v7 = (FILE *)sub_4730(v7);
    }
    else
    {
      sub_74D0(&v39);
      v37 = sub_7580(v7);
      if ( !v37 || (unsigned int)sub_4960(v7) )
      {
        v29 = sub_7110(4, v10);
        v30 = dcgettext(0, "cannot read file names from %s", 5);
        error(1, 0, v30, v29);
      }
      v15 = v40;
      v31 = v39;
      v7 = (FILE *)sub_4700(v40, &v39);
    }
  }
  else
  {
LABEL_63:
    if ( (int)v7 > v13 )
    {
      v15 = &v6[v13];
      v31 = (int)v7 - v13;
    }
    else
    {
      v31 = 1;
      v15 = (char **)&unk_E110;
    }
    v37 = 0;
    v7 = (FILE *)sub_4700(v15, v11);
  }
  if ( !v7 )
    goto LABEL_77;
  v5 = (const char *)&v38;
  v4 = "-";
  ptr = (int *)sub_32F0(v31, v15);
  v3 = ptr;
  v33 = 1;
  dword_E12C = sub_3230(v31, ptr);
  while ( 1 )
  {
    v16 = (char **)sub_4770(v7, v5);
    v6 = v16;
    if ( !v16 )
      break;
    if ( !v10 )
    {
      if ( *(char *)v16 )
      {
LABEL_29:
        v33 &= sub_4500((char *)v6);
        goto LABEL_30;
      }
      v17 = dcgettext(0, "invalid zero-length file name", 5);
      error(0, 0, format, v17);
LABEL_37:
      v33 = 0;
      goto LABEL_30;
    }
    if ( !strcmp(v10, v4) && !strcmp((const char *)v6, v4) )
    {
LABEL_45:
      v34 = sub_7110(4, v6);
      v19 = dcgettext(0, "when reading file names from stdin, no file name of %s allowed", 5);
      error(0, 0, v19, v34);
      if ( *(char *)v6 )
        goto LABEL_37;
    }
    else if ( *(char *)v6 )
    {
      goto LABEL_29;
    }
    v20 = sub_4810(v7);
    v35 = dcgettext(0, "invalid zero-length file name", 5);
    v21 = (const char *)sub_7210(0, 3, v10);
    error(0, 0, "%s:%lu: %s", v21, v20, v35);
    v33 = 0;
LABEL_30:
    if ( !v31 )
      *ptr = 1;
    v3 += 38;
  }
  switch ( v38 )
  {
    case 3:
LABEL_77:
      sub_8250();
    case 4:
      v6 = (char **)sub_7210(0, 3, v10);
      v10 = dcgettext(0, "%s: read error", 5);
      v22 = __errno_location();
      error(0, *v22, v10, v6);
      v33 = 0;
      break;
    case 2:
      if ( ((unsigned char)v33 & (v10 == 0)) != 0 )
      {
        v33 &= v10 == 0;
        if ( !sub_4810(v7) )
          v33 = sub_4500(0);
      }
      break;
    default:
      __assert_fail("!\"unexpected error code from argv_iter\"", "src/wc.c", 0x3AAu, "main");
  }
  if ( v37 )
    sub_7550(&v39);
  if ( (unsigned long long)sub_4810(v7) > 1 )
  {
    v23 = dcgettext(0, "total", 5);
    sub_35D0(qword_E158, qword_E150, qword_E148, qword_E140, qword_E138, v23);
  }
  sub_4830(v7);
  free(ptr);
  if ( byte_E128 && close(0) )
  {
    v11 = (char **)(unsigned int)*__errno_location();
    error(1, (int)v11, "-");
    goto LABEL_63;
  }
  return (unsigned char)v33 ^ 1u;
}



// Function: iswnbspace @ 0x31f0
long long iswnbspace(int a1)
{
  long long result; // rax

  result = 0;
  if ( !byte_E118 )
  {
    if ( a1 == 160 || a1 == 8199 )
      return 1;
    else
      return (a1 == 8288) | (unsigned char)(a1 == 8239);
  }
  return result;
}



// Function: compute_number_width @ 0x3230
long long compute_number_width(long long a1, long long a2)
{
  long long result; // rax
  unsigned long long v3; // rdx
  int v4; // r8d
  long long v5; // rdi
  int v6; // ecx
  unsigned long long v7; // rsi

  result = 1;
  if ( a1 && *(int *)a2 <= 0 )
  {
    v3 = 0;
    v4 = 1;
    v5 = a2 + 152 * a1;
    do
    {
      if ( !*(int *)a2 )
      {
        if ( (*(int *)(a2 + 32) & 0xF000) == 0x8000 )
          v3 += *(long long *)(a2 + 56);
        else
          v4 = 7;
      }
      a2 += 152;
    }
    while ( a2 != v5 );
    v6 = 1;
    if ( v3 > 9 )
    {
      do
      {
        v7 = v3;
        ++v6;
        v3 /= 0xAu;
      }
      while ( v7 > 0x63 );
    }
    result = (unsigned int)v4;
    if ( v6 >= v4 )
      return (unsigned int)v6;
  }
  return result;
}



// Function: get_input_fstatus @ 0x32f0
int *get_input_fstatus(long long a1, long long a2)
{
  bool v3; // zf
  long long v4; // rdi
  long long v5; // rbx
  struct stat *v6; // rbp
  int v7; // eax
  const char *v8; // r14
  int *v10; // [rsp+8h] [rbp-40h]

  v3 = a1 == 0;
  v4 = 1;
  if ( !v3 )
    v4 = a1;
  v10 = (int *)sub_7FB0(v4, 152);
  if ( !a1
    || a1 == 1
    && (unsigned char)byte_E130
     + (unsigned char)byte_E131
     + (unsigned char)byte_E132
     + (unsigned char)byte_E133
     + (unsigned char)byte_E134 == 1 )
  {
    *v10 = 1;
  }
  else
  {
    v5 = 0;
    v6 = (struct stat *)(v10 + 2);
    do
    {
      v8 = *(const char **)(a2 + 8 * v5);
      if ( !v8 || !strcmp(*(const char **)(a2 + 8 * v5), "-") )
        v7 = fstat(0, v6);
      else
        v7 = stat(v8, v6);
      ++v5;
      LODWORD(v6[-1].__unused[2]) = v7;
      v6 = (struct stat *)((char *)v6 + 152);
    }
    while ( a1 != v5 );
  }
  return v10;
}



// Function: wc_lines @ 0x33e0
// positive sp value has been detected, the output may be wrong!
long long wc_lines(long long a1, int a2, long long *a3, long long *a4)
{
  char v4; // r14
  long long v5; // r13
  unsigned long long v6; // rax
  char *v7; // rbx
  long long v8; // rdx
  char *v9; // rax
  bool v10; // cf
  bool v11; // zf
  long long v12; // rbp
  char *v13; // rdi
  char *v14; // rax
  long long v16; // r12
  int *v17; // rax
  unsigned long long v18; // [rsp-48h] [rbp-4078h]
  long long v19; // [rsp-40h] [rbp-4070h]
  bool v20; // [rsp-31h] [rbp-4061h]
  long long *v21; // [rsp-30h] [rbp-4060h]
  long long *v22; // [rsp-28h] [rbp-4058h]
  long long v23; // [rsp-20h] [rbp-4050h]
  char v24[24]; // [rsp-18h] [rbp-4048h] BYREF
  char v25; // [rsp+0h] [rbp-4030h] BYREF
  long long v26[1542]; // [rsp+1000h] [rbp-3030h] BYREF

  while ( &v25 != (char *)&v26[-2048] )
    ;
  v21 = a3;
  v23 = a1;
  v22 = a4;
  v26[1534] = __readfsqword(0x28u);
  v20 = a4 == 0 || a3 == 0;
  if ( v20 )
  {
    return 0;
  }
  else
  {
    v4 = 0;
    v5 = 0;
    v19 = 0;
    while ( 1 )
    {
      v6 = sub_7770(a2, v24, 0x4000u);
      if ( !v6 )
      {
        v20 = 1;
        *v22 = v19;
        *v21 = v5;
        return v20;
      }
      if ( v6 == -1 )
        break;
      v19 += v6;
      v7 = &v24[v6];
      v18 = v6 / 0xF;
      if ( v4 )
      {
        *v7 = 10;
        v12 = v5;
        v13 = v24;
        while ( 1 )
        {
          v14 = (char *)rawmemchr(v13, 10);
          if ( v7 <= v14 )
            break;
          v13 = v14 + 1;
          ++v12;
        }
        v10 = v12 - v5 < v18;
        v11 = v12 - v5 == v18;
        v5 = v12;
        v4 = v10 || v11;
      }
      else if ( v7 == v24 )
      {
        v4 = 1;
      }
      else
      {
        v8 = v5;
        v9 = v24;
        do
          v8 += *v9++ == 10;
        while ( v7 != v9 );
        v10 = v8 - v5 < v18;
        v11 = v8 - v5 == v18;
        v5 = v8;
        v4 = v10 || v11;
      }
    }
    v16 = sub_7210(0, 3, v23);
    v17 = __errno_location();
    error(0, *v17, format, v16);
  }
  return v20;
}



// Function: isnbspace @ 0x35b0
long long isnbspace(int a1)
{
  wint_t v1; // eax

  v1 = btowc(a1);
  return sub_31F0(v1);
}



// Function: write_counts @ 0x35d0
unsigned long long write_counts(long long a1, long long a2, long long a3, long long a4, long long a5, const char *a6)
{
  const char *v10; // rbp
  long long v12; // rax
  long long v13; // rax
  const char *v14; // rsi
  long long v15; // rax
  const char *v16; // rsi
  long long v17; // rax
  const char *v18; // rsi
  const char *v19; // rax
  char v20[24]; // [rsp+0h] [rbp-58h] BYREF
  unsigned long long v21; // [rsp+18h] [rbp-40h]

  v10 = "%*s";
  v21 = __readfsqword(0x28u);
  if ( byte_E134 )
  {
    v19 = (const char *)sub_4AD0(a1, v20);
    v10 = " %*s";
    __printf_chk(1, "%*s", dword_E12C, v19);
  }
  if ( byte_E133 )
  {
    v17 = sub_4AD0(a2, v20);
    v18 = v10;
    v10 = " %*s";
    __printf_chk(1, v18, (unsigned int)dword_E12C, v17);
  }
  if ( byte_E132 )
  {
    v15 = sub_4AD0(a3, v20);
    v16 = v10;
    v10 = " %*s";
    __printf_chk(1, v16, (unsigned int)dword_E12C, v15);
  }
  if ( byte_E131 )
  {
    v13 = sub_4AD0(a4, v20);
    v14 = v10;
    v10 = " %*s";
    __printf_chk(1, v14, (unsigned int)dword_E12C, v13);
  }
  if ( byte_E130 )
  {
    v12 = sub_4AD0(a5, v20);
    __printf_chk(1, v10, (unsigned int)dword_E12C, v12);
  }
  if ( a6 )
  {
    if ( strchr(a6, 10) )
      a6 = (const char *)sub_7210(0, 3, a6);
    __printf_chk(1, &aS_0[3], a6);
  }
  putchar_unlocked(10);
  return v21 - __readfsqword(0x28u);
}



// Function: avx2_supported @ 0x3830
long long avx2_supported()
{
  int v0; // eax
  long long v1; // r11
  unsigned int v2; // r12d
  const char *v4; // rsi
  char *v5; // rax
  char *v6; // rax
  int v7; // [rsp+8h] [rbp-40h] BYREF
  int v8; // [rsp+Ch] [rbp-3Ch] BYREF
  int v9; // [rsp+10h] [rbp-38h] BYREF
  int v10; // [rsp+14h] [rbp-34h] BYREF
  unsigned long long v11; // [rsp+18h] [rbp-30h]

  v11 = __readfsqword(0x28u);
  v7 = 0;
  v8 = 0;
  v9 = 0;
  v10 = 0;
  v0 = sub_37B0(&v7, &v8, &v9, &v10);
  v2 = (unsigned char)byte_E160;
  if ( !v0 )
    goto LABEL_6;
  if ( (v9 & 0x8000000) == 0 )
  {
LABEL_3:
    if ( !(char)v2 )
      return 0;
    v4 = "avx2 support not detected";
LABEL_8:
    v5 = dcgettext(0, v4, 5);
    error(0, 0, format, v5);
    return 0;
  }
  v10 = 0;
  v9 = 0;
  v8 = 0;
  v7 = 0;
  if ( !(unsigned int)sub_37F0(v1, &v8, &v9, &v10) )
  {
LABEL_6:
    if ( !(char)v2 )
      return 0;
    v4 = "failed to get cpuid";
    goto LABEL_8;
  }
  if ( (v8 & 0x20) == 0 )
    goto LABEL_3;
  if ( (char)v2 )
  {
    v6 = dcgettext(0, "using avx2 hardware support", 5);
    error(0, 0, format, v6);
  }
  else
  {
    return 1;
  }
  return v2;
}



// Function: wc @ 0x3bf0
// positive sp value has been detected, the output may be wrong!
long long wc(int a1, const char *a2, long long a3, long long a4)
{
  size_t mb_cur_max; // rax
  char v7; // r13
  unsigned int v8; // r12d
  int v9; // eax
  unsigned int v10; // r13d
  long long v11; // r14
  unsigned long long v12; // r15
  long long v13; // rcx
  __off_t v14; // rbp
  long long v15; // rax
  unsigned long long v16; // rcx
  unsigned long long v17; // rax
  bool v18; // cc
  unsigned long long v19; // rcx
  __off_t v20; // r14
  long long v21; // rax
  long long v22; // r13
  unsigned long long v23; // rbp
  long long v24; // rbx
  bool v26; // r14
  long long v27; // rbx
  long long v28; // r15
  long long v29; // rax
  char *v30; // r12
  unsigned char v31; // dl
  const unsigned short **v32; // rsi
  unsigned short v33; // ax
  long long v34; // rdx
  mbstate_t v35; // r14
  long long v36; // rax
  int v37; // eax
  wint_t v38; // edi
  int v39; // eax
  int v40; // eax
  int v41; // eax
  unsigned long long v42; // rax
  long long v43; // rbp
  unsigned long long v44; // rcx
  unsigned long long v45; // rbx
  long long v46; // r13
  long long v47; // rax
  char *v48; // r12
  char *v49; // rdx
  char v50; // r14
  const unsigned short **v51; // rsi
  unsigned char v52; // al
  unsigned short v53; // si
  int v54; // eax
  int v55; // eax
  unsigned long long v56; // rsi
  long long v57; // r12
  int *v58; // rax
  long long v59; // rcx
  unsigned char v60; // al
  unsigned long long v61; // r14
  long long v62; // r13
  long long v63; // rbx
  unsigned char v64; // al
  long long v65; // r12
  int *v66; // rax
  long long v67; // rcx
  long long v68; // r13
  int *v69; // rax
  long long v70; // rcx
  __off_t v71; // rax
  void *v72; // rax
  int v73; // eax
  unsigned long long v74; // [rsp-78h] [rbp-40A8h]
  unsigned long long v75; // [rsp-70h] [rbp-40A0h]
  long long v76; // [rsp-68h] [rbp-4098h]
  unsigned long long v77; // [rsp-60h] [rbp-4090h]
  int v78; // [rsp-50h] [rbp-4080h]
  unsigned char v79; // [rsp-4Ah] [rbp-407Ah]
  unsigned char v80; // [rsp-49h] [rbp-4079h]
  const char *v81; // [rsp-48h] [rbp-4078h]
  char *v82; // [rsp-40h] [rbp-4070h]
  wint_t v83; // [rsp-34h] [rbp-4064h] BYREF
  long long v84; // [rsp-30h] [rbp-4060h] BYREF
  long long v85; // [rsp-28h] [rbp-4058h] BYREF
  mbstate_t v86; // [rsp-20h] [rbp-4050h] BYREF
  char v87[24]; // [rsp-18h] [rbp-4048h] BYREF
  char v88; // [rsp+0h] [rbp-4030h] BYREF
  long long v89; // [rsp+1000h] [rbp-3030h] BYREF
  unsigned long long v90; // [rsp+3FF0h] [rbp-40h]

  while ( &v88 != (char *)(&v89 - 2048) )
    ;
  v78 = a1;
  v81 = a2;
  v90 = __readfsqword(0x28u);
  v82 = (char *)a2;
  if ( !a2 )
    v82 = dcgettext(0, "standard input", 5);
  v85 = 0;
  v84 = 0;
  mb_cur_max = __ctype_get_mb_cur_max();
  v7 = byte_E131;
  if ( mb_cur_max <= 1 )
  {
    v80 = 0;
    if ( !byte_E131 )
      v7 = byte_E132;
  }
  else
  {
    v80 = byte_E132;
  }
  if ( byte_E133 || (v8 = (unsigned char)byte_E130, byte_E130) )
  {
    j__posix_fadvise(a1, 0, 0, 2);
  }
  else
  {
    if ( v7 == 1 && !v80 )
    {
      if ( !byte_E134 )
        goto LABEL_11;
      j__posix_fadvise(a1, 0, 0, 2);
LABEL_120:
      if ( !byte_E134 )
      {
LABEL_11:
        v9 = *(int *)a3;
        if ( *(int *)a3 > 0 )
        {
          v9 = fstat(a1, (struct stat *)(a3 + 8));
          *(int *)a3 = v9;
        }
        if ( v9
          || (v10 = ((long long (*)(long long))sub_3BE0)(*(unsigned int *)(a3 + 32)), !(char)v10)
          || (v11 = *(long long *)(a3 + 56), v11 < 0) )
        {
LABEL_24:
          j__posix_fadvise(a1, 0, 0, 2);
          while ( 1 )
          {
            v21 = sub_7770(a1, v87, 0x4000u);
            if ( !v21 )
            {
              v13 = v85;
              v22 = 0;
              v23 = 0;
              v24 = 0;
              v8 = 1;
              goto LABEL_29;
            }
            if ( v21 == -1 )
              break;
            v85 += v21;
          }
          v23 = 0;
          v24 = 0;
          v68 = sub_7210(0, 3, v82);
          v69 = __errno_location();
          v70 = v68;
          v22 = 0;
          error(0, *v69, format, v70);
          v13 = v85;
          goto LABEL_29;
        }
        v12 = qword_E120;
        v13 = *(long long *)(a3 + 56);
        if ( a4 == -1 )
        {
          v74 = *(long long *)(a3 + 56);
          v71 = lseek(a1, 0, 1);
          v14 = v71;
          if ( !(v74 % v12) )
          {
            LOBYTE(v10) = v71 >= 0;
            goto LABEL_18;
          }
          if ( v71 <= v74 )
            v13 = v74 - v71;
          else
            v13 = 0;
        }
        else
        {
          v14 = 0;
          if ( !(v11 % (unsigned long long)qword_E120) )
          {
LABEL_18:
            v15 = *(long long *)(a3 + 64);
            v16 = v15 - 1;
            v17 = v15 + 1;
            v18 = v16 <= 0x1FFFFFFFFFFFFFFFLL;
            v19 = 513;
            if ( v18 )
              v19 = v17;
            v20 = v11 - v11 % v19;
            if ( v20 > v14 && (char)v10 && lseek(a1, v20, 1) >= 0 )
              v85 = v20 - v14;
            goto LABEL_24;
          }
        }
        v8 = v10;
        v85 = v13;
        v23 = 0;
        v22 = 0;
        v24 = 0;
        goto LABEL_29;
      }
LABEL_121:
      if ( (unsigned char)sub_3830() )
      {
        v72 = &sub_8A90;
        off_E010 = &sub_8A90;
      }
      else
      {
        v72 = off_E010;
      }
      v23 = 0;
      v22 = 0;
      v24 = 0;
      v73 = ((long long (*)(long long, int, long long *, long long *))v72)((long long)v82, a1, &v84, &v85);
      v13 = v85;
      v8 = v73;
      goto LABEL_29;
    }
    j__posix_fadvise(a1, 0, 0, 2);
    if ( v80 != 1 )
    {
      if ( v7 )
        goto LABEL_120;
      if ( !v80 )
        goto LABEL_121;
    }
  }
  if ( __ctype_get_mb_cur_max() <= 1 )
  {
    v43 = 0;
    v44 = 0;
    v45 = 0;
    v46 = 0;
    while ( 2 )
    {
      v74 = v44;
      v47 = sub_7770(a1, v87, 0x4000u);
      v44 = v74;
      if ( v47 )
      {
        if ( v47 != -1 )
        {
          v85 += v47;
          v48 = v87;
          v49 = &v87[v47];
LABEL_84:
          v50 = *v48++;
          switch ( v50 )
          {
            case 9:
              v44 = (v44 & 0xFFFFFFFFFFFFFFF8LL) + 8;
              goto LABEL_95;
            case 10:
              ++v84;
              goto LABEL_92;
            case 11:
              goto LABEL_95;
            case 12:
            case 13:
LABEL_92:
              if ( v45 < v44 )
                v45 = v44;
              v44 = 0;
              goto LABEL_95;
            case 32:
              ++v44;
              goto LABEL_95;
            default:
              v75 = v44;
              v74 = (unsigned long long)v49;
              v51 = __ctype_b_loc();
              v52 = ((long long (*)(long long, const unsigned short **))sub_31E0)((unsigned int)v50, v51);
              v49 = (char *)v74;
              v44 = v75;
              v53 = (*v51)[v52];
              if ( (v53 & 0x4000) == 0 )
                goto LABEL_89;
              v44 = v75 + 1;
              if ( (v53 & 0x2000) != 0 || (v54 = sub_35B0(v52), v49 = (char *)v74, v44 = v75 + 1, v54) )
              {
LABEL_95:
                v43 += v46;
                v46 = 0;
              }
              else
              {
                v46 = 1;
              }
LABEL_89:
              if ( v49 == v48 )
                continue;
              goto LABEL_84;
          }
        }
        v64 = v46;
        v61 = v74;
        v62 = v43;
        v23 = v45;
        v63 = v64;
        v65 = sub_7210(0, 3, v82);
        v66 = __errno_location();
        v67 = v65;
        v8 = 0;
        error(0, *v66, format, v67);
      }
      else
      {
        v60 = v46;
        v61 = v74;
        v62 = v43;
        v8 = 1;
        v23 = v45;
        v63 = v60;
      }
      break;
    }
    v13 = v85;
    if ( v23 < v61 )
      v23 = v61;
    v24 = v62 + v63;
    v22 = 0;
  }
  else
  {
    v26 = 0;
    v27 = 0;
    v86 = 0;
    v22 = 0;
    v28 = 0;
    v74 = 0;
    v77 = 0;
    v76 = 0;
    while ( 1 )
    {
LABEL_37:
      v29 = sub_7770(v78, &v87[v28], 0x4000 - v28);
      if ( !v29 )
      {
        v8 = 1;
        goto LABEL_102;
      }
      if ( v29 == -1 )
        break;
      v85 += v29;
      v30 = v87;
      v28 += v29;
      if ( v26 )
        goto LABEL_49;
LABEL_40:
      v79 = *v30;
      LODWORD(v75) = *v30;
      v31 = sub_4BC0();
      if ( v31 )
      {
        v83 = v75;
        switch ( v79 )
        {
          case 9u:
            v34 = 1;
LABEL_69:
            v74 = (v74 & 0xFFFFFFFFFFFFFFF8LL) + 8;
            goto LABEL_62;
          case 0xAu:
            v34 = 1;
LABEL_67:
            ++v84;
            goto LABEL_59;
          case 0xBu:
            goto LABEL_63;
          case 0xCu:
          case 0xDu:
            v34 = 1;
LABEL_59:
            v42 = v74;
            v74 = 0;
            if ( v77 >= v42 )
              v42 = v77;
            v77 = v42;
            goto LABEL_62;
          case 0x20u:
            v34 = 1;
LABEL_65:
            ++v74;
            goto LABEL_62;
          default:
            v79 = v31;
            v32 = __ctype_b_loc();
            v33 = (*v32)[(unsigned char)((long long (*)(long long, const unsigned short **))sub_31E0)(
                                            (unsigned int)v75,
                                            v32)];
            if ( (v33 & 0x4000) == 0 )
              goto LABEL_45;
            ++v74;
            if ( (v33 & 0x2000) != 0 )
            {
LABEL_63:
              v34 = 1;
              goto LABEL_62;
            }
            v27 = v79;
LABEL_45:
            v34 = 1;
            break;
        }
        goto LABEL_46;
      }
LABEL_49:
      while ( 1 )
      {
        v35 = v86;
        v36 = ((long long (*)(wint_t *, char *, long long, mbstate_t *))sub_4BE0)(&v83, v30, v28, &v86);
        if ( v36 == -2 )
          break;
        if ( v36 != -1 )
        {
          v75 = v36;
          v37 = mbsinit(&v86);
          v34 = v75;
          v26 = v37 == 0;
          if ( v75 )
          {
            switch ( v83 )
            {
              case 9u:
                goto LABEL_69;
              case 0xAu:
                goto LABEL_67;
              case 0xBu:
                goto LABEL_62;
              case 0xCu:
              case 0xDu:
                goto LABEL_59;
              case 0x20u:
                goto LABEL_65;
              default:
                v38 = v83;
                goto LABEL_53;
            }
          }
          v83 = 0;
          v38 = 0;
          v34 = 1;
LABEL_53:
          v75 = v34;
          v39 = iswprint(v38);
          v34 = v75;
          if ( v39 )
          {
            if ( byte_E130 )
            {
              v55 = wcwidth(v83);
              v34 = v75;
              v56 = v74 + v55;
              if ( v55 <= 0 )
                v56 = v74;
              v74 = v56;
            }
            v75 = v34;
            v40 = iswspace(v83);
            v34 = v75;
            if ( v40 || (v41 = sub_31F0(v83), v34 = v75, v41) )
            {
LABEL_62:
              v76 += v27;
              v27 = 0;
            }
            else
            {
              v27 = 1;
            }
          }
LABEL_46:
          v30 += v34;
          v28 -= v34;
          ++v22;
          goto LABEL_47;
        }
        ++v30;
        --v28;
        v26 = 1;
LABEL_47:
        if ( !v28 )
          goto LABEL_37;
        if ( !v26 )
          goto LABEL_40;
      }
      v86 = v35;
      if ( v28 )
      {
        if ( v28 == 0x4000 )
        {
          ++v30;
          v28 = 0x3FFF;
        }
        ((void (*)(char *, char *, long long, long long))__memmove_chk)(v87, v30, v28, 16385);
      }
      v26 = 1;
    }
    v57 = sub_7210(0, 3, v82);
    v58 = __errno_location();
    v59 = v57;
    v8 = 0;
    error(0, *v58, format, v59);
LABEL_102:
    v23 = v74;
    v13 = v85;
    if ( v77 >= v74 )
      v23 = v77;
    v24 = v76 + v27;
  }
LABEL_29:
  if ( (unsigned char)byte_E132 > v80 )
    v22 = v13;
  sub_35D0(v84, v24, v22, v13, v23, v81);
  qword_E150 += v24;
  qword_E158 += v84;
  qword_E148 += v22;
  qword_E140 += v85;
  if ( qword_E138 < v23 )
    qword_E138 = v23;
  return v8;
}



// Function: wc_file @ 0x4500
long long wc_file(char *file, long long a2)
{
  int v3; // eax
  int v4; // r13d
  unsigned int v5; // r12d
  long long v6; // r12
  int *v7; // rax

  if ( file && strcmp(file, "-") )
  {
    v3 = open(file, 0);
    v4 = v3;
    if ( v3 == -1 || (v5 = sub_3BF0(v3, file, a2, 0), close(v4)) )
    {
      v6 = sub_7210(0, 3, file);
      v7 = __errno_location();
      error(0, *v7, format, v6);
      return 0;
    }
    else
    {
      return v5;
    }
  }
  else
  {
    byte_E128 = 1;
    return sub_3BF0(0, file, a2, -1);
  }
}



// Function: usage @ 0x45c0
void usage(int status)
{
  long long v2; // r12
  char *v3; // rax
  char *v4; // rax
  FILE *v5; // r12
  char *v6; // rax
  FILE *v7; // r12
  char *v8; // rax
  FILE *v9; // r12
  char *v10; // rax
  FILE *v11; // r12
  char *v12; // rax
  FILE *v13; // rsi
  char *v14; // rdi

  v2 = qword_E178;
  if ( status )
  {
    v3 = dcgettext(0, "Try '%s --help' for more information.\n", 5);
    __fprintf_chk(stderr, 1, v3, v2);
  }
  else
  {
    v4 = dcgettext(0, "Usage: %s [OPTION]... [FILE]...\n  or:  %s [OPTION]... --files0-from=F\n", 5);
    __printf_chk(1, v4, v2, v2);
    v5 = stdout;
    v6 = dcgettext(
           0,
           "Print newline, word, and byte counts for each FILE, and a total line if\n"
           "more than one FILE is specified.  A word is a non-zero-length sequence of\n"
           "printable characters delimited by white space.\n",
           5);
    fputs_unlocked(v6, v5);
    sub_32C0();
    v7 = stdout;
    v8 = dcgettext(
           0,
           "\n"
           "The options below may be used to select which counts are printed, always in\n"
           "the following order: newline, word, character, byte, maximum line length.\n"
           "  -c, --bytes            print the byte counts\n"
           "  -m, --chars            print the character counts\n"
           "  -l, --lines            print the newline counts\n",
           5);
    fputs_unlocked(v8, v7);
    v9 = stdout;
    v10 = dcgettext(
            0,
            "      --files0-from=F    read input from the files specified by\n"
            "                           NUL-terminated names in file F;\n"
            "                           If F is - then read names from standard input\n"
            "  -L, --max-line-length  print the maximum display width\n"
            "  -w, --words            print the word counts\n",
            5);
    fputs_unlocked(v10, v9);
    v11 = stdout;
    v12 = dcgettext(0, "      --help        display this help and exit\n", 5);
    fputs_unlocked(v12, v11);
    v13 = stdout;
    v14 = dcgettext(0, "      --version     output version information and exit\n", 5);
    fputs_unlocked(v14, v13);
    sub_39B0();
  }
  exit(status);
}



// Function: wc_lines_avx2 @ 0x8a90
long long wc_lines_avx2(
        long long a1,
        int a2,
        long long *a3,
        long long *a4,
        __m128 _XMM0,
        __m128 _XMM1,
        __m128 _XMM2,
        __m128 _XMM3)
{
  unsigned int v8; // r13d
  long long v11; // rbx
  long long v12; // r14
  unsigned long long v13; // rax
  char *v15; // rdi
  char *v17; // rdx
  char *v19; // rax
  int v31; // r8d
  int v33; // r8d
  long long v36; // r12
  int *v37; // rax
  char buf[4064]; // [rsp+20h] [rbp-4008h] BYREF
  unsigned long long v42; // [rsp+3FF8h] [rbp-30h]

  v42 = __readfsqword(0x28u);
  LOBYTE(v8) = a4 == 0 || a3 == 0;
  if ( (char)v8 )
  {
    return 0;
  }
  else
  {
    v11 = 0;
    v12 = 0;
    while ( 1 )
    {
      __asm { vzeroupper }
      v13 = sub_7770(a2, buf, 0x3FC0u);
      __asm { vmovdqa ymm4, cs:ymmword_A200 }
      if ( !v13 )
      {
        v8 = 1;
        *a3 = v12;
        *a4 = v11;
        __asm { vzeroupper }
        return v8;
      }
      if ( v13 == -1 )
        break;
      v11 += v13;
      v15 = &buf[v13];
      if ( v13 <= 0x3F )
      {
        __asm { vpxor   xmm0, xmm0, xmm0 }
        v19 = buf;
        __asm { vmovdqa ymm1, ymm0 }
      }
      else
      {
        __asm { vpxor   xmm3, xmm3, xmm3 }
        v17 = buf;
        __asm { vmovdqa ymm2, ymm3 }
        v19 = &buf[((v13 - 64) & 0xFFFFFFFFFFFFFFC0LL) + 64];
        do
        {
          __asm
          {
            vpcmpeqb ymm1, ymm4, ymmword ptr [rdx]
            vpcmpeqb ymm0, ymm4, ymmword ptr [rdx+20h]
          }
          v17 += 64;
          __asm
          {
            vpsubb  ymm1, ymm2, ymm1
            vpsubb  ymm0, ymm3, ymm0
            vmovdqa ymm2, ymm1
            vmovdqa ymm3, ymm0
          }
        }
        while ( v17 != v19 );
      }
      __asm
      {
        vpxor   xmm2, xmm2, xmm2
        vpsadbw ymm1, ymm1, ymm2
        vpsadbw ymm0, ymm0, ymm2
        vmovdqa xmm3, xmm1
        vextracti128 xmm1, ymm1, 1
        vmovdqa xmm2, xmm0
        vpextrw r8d, xmm3, 0
        vpextrw rdx, xmm3, 4
        vextracti128 xmm0, ymm0, 1
      }
      LODWORD(_RDX) = _R8D + _RDX;
      __asm
      {
        vpextrw r8d, xmm1, 0
        vpextrw r9d, xmm2, 0
      }
      LODWORD(_RDX) = _R8D + _RDX;
      __asm { vpextrw r8d, xmm1, 4 }
      LODWORD(_RDX) = _R8D + _RDX;
      __asm { vpextrw r8d, xmm2, 4 }
      v31 = _R9D + _R8D;
      __asm { vpextrw r9d, xmm0, 0 }
      v33 = _R9D + v31;
      __asm { vpextrw r9d, xmm0, 4 }
      for ( v12 += _R9D + v33 + (long long)(int)_RDX; v15 != v19; v12 += *(v19 - 1) == 10 )
        ++v19;
    }
    __asm { vzeroupper }
    v36 = sub_7210(0, 3, a1);
    v37 = __errno_location();
    error(0, *v37, format, v36);
  }
  return v8;
}


