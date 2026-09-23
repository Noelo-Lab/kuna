// Function: avx2_supported @ 0x2daf
long long avx2_supported()
{
  char *v0; // rax
  char *v2; // rax
  char *v3; // rax
  char v4; // [rsp+6h] [rbp-1Ah]
  bool v5; // [rsp+7h] [rbp-19h]
  int v6; // [rsp+8h] [rbp-18h] BYREF
  int v7; // [rsp+Ch] [rbp-14h] BYREF
  int v8; // [rsp+10h] [rbp-10h] BYREF
  int v9; // [rsp+14h] [rbp-Ch] BYREF
  unsigned long long v10; // [rsp+18h] [rbp-8h]

  v10 = __readfsqword(0x28u);
  v6 = 0;
  v7 = 0;
  v8 = 0;
  v9 = 0;
  v4 = 0;
  v5 = 0;
  if ( (unsigned int)sub_2CB7(1, &v6, &v7, &v8, &v9) )
  {
    v4 = 1;
    v5 = (v8 & 0x8000000) != 0;
  }
  if ( v5 )
  {
    v9 = 0;
    v8 = 0;
    v7 = 0;
    v6 = 0;
    if ( (unsigned int)sub_2D2F(7, 0, &v6, &v7, &v8, &v9) )
    {
      if ( (v7 & 0x20) == 0 )
        v5 = 0;
    }
    else
    {
      v4 = 0;
    }
  }
  if ( v4 != 1 )
  {
    if ( byte_F110 )
    {
      v0 = gettext("failed to get cpuid");
      error(0, 0, "%s", v0);
    }
    return 0;
  }
  else if ( !v5 )
  {
    if ( byte_F110 )
    {
      v2 = gettext("avx2 support not detected");
      error(0, 0, "%s", v2);
    }
    return 0;
  }
  else
  {
    if ( byte_F110 )
    {
      v3 = gettext("using avx2 hardware support");
      error(0, 0, "%s", v3);
    }
    return 1;
  }
}



// Function: usage @ 0x2f7b
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

  if ( a1 )
  {
    v1 = qword_F178;
    v2 = gettext("Try '%s --help' for more information.\n");
    fprintf(stderr, v2, v1);
  }
  else
  {
    v3 = qword_F178;
    v4 = qword_F178;
    v5 = gettext("Usage: %s [OPTION]... [FILE]...\n  or:  %s [OPTION]... --files0-from=F\n");
    printf(v5, v4, v3);
    v6 = stdout;
    v7 = gettext(
           "Print newline, word, and byte counts for each FILE, and a total line if\n"
           "more than one FILE is specified.  A word is a non-zero-length sequence of\n"
           "printable characters delimited by white space.\n");
    fputs_unlocked(v7, v6);
    sub_2978();
    v8 = stdout;
    v9 = gettext(
           "\n"
           "The options below may be used to select which counts are printed, always in\n"
           "the following order: newline, word, character, byte, maximum line length.\n"
           "  -c, --bytes            print the byte counts\n"
           "  -m, --chars            print the character counts\n"
           "  -l, --lines            print the newline counts\n");
    fputs_unlocked(v9, v8);
    v10 = stdout;
    v11 = gettext(
            "      --files0-from=F    read input from the files specified by\n"
            "                           NUL-terminated names in file F;\n"
            "                           If F is - then read names from standard input\n"
            "  -L, --max-line-length  print the maximum display width\n"
            "  -w, --words            print the word counts\n");
    fputs_unlocked(v11, v10);
    v12 = stdout;
    v13 = gettext("      --help        display this help and exit\n");
    fputs_unlocked(v13, v12);
    v14 = stdout;
    v15 = gettext("      --version     output version information and exit\n");
    fputs_unlocked(v15, v14);
    sub_29A9("wc");
  }
  exit(a1);
}



// Function: iswnbspace @ 0x30be
long long iswnbspace(int a1)
{
  return byte_F158 != 1 && (a1 == 160 || a1 == 8199 || a1 == 8239 || a1 == 8288);
}



// Function: isnbspace @ 0x3109
long long isnbspace(int a1)
{
  wint_t v1; // eax

  v1 = btowc(a1);
  return sub_30BE(v1);
}



// Function: write_counts @ 0x312b
unsigned long long write_counts(long long a1, long long a2, long long a3, long long a4, long long a5, const char *a6)
{
  const char *v6; // rax
  long long v7; // rax
  long long v8; // rax
  long long v9; // rax
  long long v10; // rax
  const char *v11; // rax
  char *format; // [rsp+38h] [rbp-28h]
  char v18[24]; // [rsp+40h] [rbp-20h] BYREF
  unsigned long long v19; // [rsp+58h] [rbp-8h]

  v19 = __readfsqword(0x28u);
  format = "%*s";
  if ( byte_F140 )
  {
    v6 = (const char *)sub_5729(a1, v18);
    printf("%*s", dword_F148, v6);
    format = " %*s";
  }
  if ( byte_F141 )
  {
    v7 = sub_5729(a2, v18);
    printf(format, (unsigned int)dword_F148, v7);
    format = " %*s";
  }
  if ( byte_F142 )
  {
    v8 = sub_5729(a3, v18);
    printf(format, (unsigned int)dword_F148, v8);
    format = " %*s";
  }
  if ( byte_F143 )
  {
    v9 = sub_5729(a4, v18);
    printf(format, (unsigned int)dword_F148, v9);
    format = " %*s";
  }
  if ( byte_F144 )
  {
    v10 = sub_5729(a5, v18);
    printf(format, (unsigned int)dword_F148, v10);
  }
  if ( a6 )
  {
    if ( strchr(a6, 10) )
      v11 = (const char *)sub_79A7(0, 3, a6);
    else
      v11 = a6;
    printf(" %s", v11);
  }
  putchar_unlocked(10);
  return v19 - __readfsqword(0x28u);
}



// Function: wc_lines @ 0x3328
// bad sp value at call has been detected, the output may be wrong!
long long wc_lines(long long a1, unsigned int a2, long long *a3, long long *a4)
{
  const char *v5; // rbx
  int *v6; // rax
  char *v7; // rax
  bool v10; // [rsp+2Fh] [rbp-4051h]
  long long i; // [rsp+30h] [rbp-4050h]
  long long v12; // [rsp+38h] [rbp-4048h]
  char *s; // [rsp+40h] [rbp-4040h]
  char *sa; // [rsp+40h] [rbp-4040h]
  unsigned long long v15; // [rsp+48h] [rbp-4038h]
  char *v16; // [rsp+50h] [rbp-4030h]
  long long v17; // [rsp+58h] [rbp-4028h]
  char v18[24]; // [rsp+60h] [rbp-4020h] BYREF
  char v19; // [rsp+78h] [rbp-4008h] BYREF
  long long v20[512]; // [rsp+3078h] [rbp-1008h] BYREF

  while ( v20 != (long long *)&v19 )
    ;
  v20[510] = __readfsqword(0x28u);
  v10 = 0;
  if ( !a3 || !a4 )
    return 0;
  v12 = 0;
  for ( i = 0; ; v10 = i - v17 <= v15 / 0xF )
  {
    v15 = sub_8412(a2, v18, 0x4000);
    if ( !v15 )
      break;
    if ( v15 == -1 )
    {
      v5 = (const char *)sub_79A7(0, 3, a1);
      v6 = __errno_location();
      error(0, *v6, "%s", v5);
      return 0;
    }
    v12 += v15;
    s = v18;
    v16 = &v18[v15];
    v17 = i;
    if ( !v10 )
    {
      while ( s != v16 )
      {
        v7 = s++;
        i += *v7 == 10;
      }
    }
    else
    {
      *v16 = 10;
      while ( 1 )
      {
        sa = (char *)rawmemchr(s, 10);
        if ( sa >= v16 )
          break;
        s = sa + 1;
        ++i;
      }
    }
  }
  *a4 = v12;
  *a3 = i;
  return 1;
}



// Function: wc @ 0x3591
// bad sp value at call has been detected, the output may be wrong!
long long wc(unsigned int a1, const char *a2, long long a3, __off_t a4)
{
  char *v4; // rax
  bool v5; // al
  bool v6; // al
  long long v7; // rax
  unsigned long long v8; // rcx
  const char *v9; // rbx
  int *v10; // rax
  const char *v11; // rbx
  int *v12; // rax
  const unsigned short *v13; // rbx
  const unsigned short *v14; // rbx
  const char *v15; // rbx
  int *v16; // rax
  char *v17; // rax
  const unsigned short *v18; // rbx
  const unsigned short *v19; // rbx
  unsigned char v20; // al
  unsigned char v24; // [rsp+27h] [rbp-40B9h]
  char v25; // [rsp+28h] [rbp-40B8h]
  unsigned char v26; // [rsp+29h] [rbp-40B7h]
  char v27; // [rsp+2Ah] [rbp-40B6h]
  unsigned char v28; // [rsp+2Bh] [rbp-40B5h]
  bool v29; // [rsp+2Ch] [rbp-40B4h]
  char v30; // [rsp+2Dh] [rbp-40B3h]
  unsigned char v31; // [rsp+2Eh] [rbp-40B2h]
  bool v32; // [rsp+2Fh] [rbp-40B1h]
  wint_t wc; // [rsp+30h] [rbp-40B0h] BYREF
  int v34; // [rsp+34h] [rbp-40ACh]
  long long v35; // [rsp+38h] [rbp-40A8h] BYREF
  __off_t v36; // [rsp+40h] [rbp-40A0h] BYREF
  size_t n; // [rsp+48h] [rbp-4098h]
  long long v38; // [rsp+50h] [rbp-4090h]
  __off_t v39; // [rsp+58h] [rbp-4088h]
  unsigned long long v40; // [rsp+60h] [rbp-4080h]
  unsigned long long v41; // [rsp+68h] [rbp-4078h]
  size_t v42; // [rsp+70h] [rbp-4070h]
  void *src; // [rsp+78h] [rbp-4068h]
  long long v44; // [rsp+80h] [rbp-4060h]
  unsigned long long v45; // [rsp+88h] [rbp-4058h]
  char *v46; // [rsp+90h] [rbp-4050h]
  char *v47; // [rsp+98h] [rbp-4048h]
  unsigned long long v48; // [rsp+A0h] [rbp-4040h]
  __off_t offset; // [rsp+A8h] [rbp-4038h]
  mbstate_t ps; // [rsp+B0h] [rbp-4030h] BYREF
  mbstate_t v51; // [rsp+B8h] [rbp-4028h]
  char dest[24]; // [rsp+C0h] [rbp-4020h] BYREF
  char v53; // [rsp+D8h] [rbp-4008h] BYREF
  long long v54[512]; // [rsp+30D8h] [rbp-1008h] BYREF

  while ( v54 != (long long *)&v53 )
    ;
  v54[510] = __readfsqword(0x28u);
  v24 = 1;
  if ( a2 )
    v4 = (char *)a2;
  else
    v4 = gettext("standard input");
  v47 = v4;
  v40 = 0;
  v36 = 0;
  v39 = 0;
  v38 = 0;
  v35 = 0;
  if ( __ctype_get_mb_cur_max() <= 1 )
  {
    v5 = byte_F143 || byte_F142;
    v25 = v5;
    v26 = 0;
  }
  else
  {
    v25 = byte_F143;
    v26 = byte_F142;
  }
  v6 = byte_F141 || byte_F144;
  v32 = v6;
  if ( v25 != 1 || v26 || byte_F140 || v6 )
    sub_54B7(a1, 0, 0, 2);
  if ( !v25 || v26 == 1 || byte_F140 == 1 || v32 )
  {
    if ( v26 != 1 && !v32 )
    {
      if ( (unsigned char)sub_2DAF() )
        off_F010 = (long long (*)(long long, unsigned int, long long *, long long *))&sub_A1B1;
      v24 = off_F010((long long)v47, a1, &v35, &v36);
      goto LABEL_123;
    }
    if ( __ctype_get_mb_cur_max() <= 1 )
    {
      v31 = 0;
      v45 = 0;
      while ( 2 )
      {
        n = sub_8412(a1, dest, 0x4000);
        if ( n )
        {
          v46 = dest;
          if ( n != -1 )
          {
            v36 += n;
LABEL_106:
            v17 = v46++;
            switch ( *v17 )
            {
              case 9:
                v45 = (v45 & 0xFFFFFFFFFFFFFFF8LL) + 8;
                goto LABEL_113;
              case 0xA:
                ++v35;
                goto LABEL_108;
              case 0xB:
                goto LABEL_113;
              case 0xC:
              case 0xD:
LABEL_108:
                if ( v45 > v40 )
                  v40 = v45;
                v45 = 0;
                goto LABEL_113;
              case 0x20:
                ++v45;
                goto LABEL_113;
              default:
                v18 = *__ctype_b_loc();
                if ( (v18[(unsigned char)sub_2969((unsigned int)(char)*(v46 - 1))] & 0x4000) == 0 )
                  goto LABEL_118;
                ++v45;
                v19 = *__ctype_b_loc();
                if ( (v19[(unsigned char)sub_2969((unsigned int)(char)*(v46 - 1))] & 0x2000) != 0
                  || (v20 = sub_2969((unsigned int)(char)*(v46 - 1)), sub_3109(v20)) )
                {
LABEL_113:
                  v38 += v31;
                  v31 = 0;
                }
                else
                {
                  v31 = 1;
                }
LABEL_118:
                if ( !--n )
                  continue;
                goto LABEL_106;
            }
          }
          v15 = (const char *)sub_79A7(0, 3, v47);
          v16 = __errno_location();
          error(0, *v16, "%s", v15);
          v24 = 0;
        }
        break;
      }
      if ( v45 > v40 )
        v40 = v45;
      v38 += v31;
      goto LABEL_123;
    }
    v28 = 0;
    v41 = 0;
    ps = 0;
    v29 = 0;
    v42 = 0;
LABEL_98:
    n = sub_8412(a1, &dest[v42], 0x4000 - v42);
    if ( !n )
    {
LABEL_99:
      if ( v41 > v40 )
        v40 = v41;
      v38 += v28;
      goto LABEL_123;
    }
    if ( n == -1 )
    {
      v11 = (const char *)sub_79A7(0, 3, v47);
      v12 = __errno_location();
      error(0, *v12, "%s", v11);
      v24 = 0;
      goto LABEL_99;
    }
    v36 += n;
    src = dest;
    n += v42;
    while ( 1 )
    {
      v30 = 1;
      if ( !v29 )
      {
        if ( (unsigned char)sub_5893((unsigned int)*(char *)src) )
          break;
      }
      v29 = 1;
      v51 = ps;
      v44 = sub_58D8(&wc, src, n, &ps);
      if ( v44 == -2 )
      {
        ps = v51;
LABEL_93:
        if ( n )
        {
          if ( n == 0x4000 )
          {
            src = (char *)src + 1;
            n = 0x3FFF;
          }
          memmove(dest, src, n);
        }
        v42 = n;
        goto LABEL_98;
      }
      if ( v44 != -1 )
      {
        v29 = mbsinit(&ps) == 0;
        if ( !v44 )
        {
          wc = 0;
          v44 = 1;
        }
LABEL_71:
        switch ( wc )
        {
          case 9u:
            v41 = (v41 & 0xFFFFFFFFFFFFFFF8LL) + 8;
            goto LABEL_78;
          case 0xAu:
            ++v35;
            goto LABEL_73;
          case 0xBu:
            goto LABEL_78;
          case 0xCu:
          case 0xDu:
LABEL_73:
            if ( v41 > v40 )
              v40 = v41;
            v41 = 0;
            goto LABEL_78;
          case 0x20u:
            ++v41;
            goto LABEL_78;
          default:
            if ( v30 && iswprint(wc) )
            {
              if ( byte_F144 )
              {
                v34 = wcwidth(wc);
                if ( v34 > 0 )
                  v41 += v34;
              }
              if ( iswspace(wc) || sub_30BE(wc) )
              {
LABEL_78:
                v38 += v28;
                v28 = 0;
                goto LABEL_91;
              }
              v28 = 1;
            }
            else if ( v30 != 1 )
            {
              v13 = *__ctype_b_loc();
              if ( (v13[(unsigned char)sub_2969((unsigned int)*(char *)src)] & 0x4000) != 0 )
              {
                ++v41;
                v14 = *__ctype_b_loc();
                if ( (v14[(unsigned char)sub_2969((unsigned int)*(char *)src)] & 0x2000) != 0 )
                  goto LABEL_78;
                v28 = 1;
              }
            }
LABEL_91:
            src = (char *)src + v44;
            n -= v44;
            ++v39;
            break;
        }
        goto LABEL_92;
      }
      src = (char *)src + 1;
      --n;
LABEL_92:
      if ( !n )
        goto LABEL_93;
    }
    v44 = 1;
    wc = *(char *)src;
    v30 = 0;
    goto LABEL_71;
  }
  v27 = 0;
  if ( *(int *)a3 > 0 )
    *(int *)a3 = fstat(a1, (struct stat *)(a3 + 8));
  if ( !*(int *)a3 && (unsigned char)sub_2C40(a3 + 8) && *(long long *)(a3 + 56) >= 0 )
  {
    v48 = *(long long *)(a3 + 56);
    if ( a4 < 0 )
      a4 = lseek(a1, 0, 1);
    if ( v48 % qword_F150 )
    {
      if ( v48 < a4 )
        v7 = 0;
      else
        v7 = v48 - a4;
      v36 = v7;
      v27 = 1;
    }
    else
    {
      if ( *(long long *)(a3 + 64) <= 0 || *(long long *)(a3 + 64) > 0x2000000000000000uLL )
        v8 = 513;
      else
        v8 = *(long long *)(a3 + 64) + 1LL;
      offset = v48 - v48 % v8;
      if ( a4 >= 0 && a4 < offset && lseek(a1, offset, 1) >= 0 )
        v36 = offset - a4;
    }
  }
  if ( v27 != 1 )
  {
    sub_54B7(a1, 0, 0, 2);
    while ( 1 )
    {
      n = sub_8412(a1, dest, 0x4000);
      if ( !n )
        break;
      if ( n == -1 )
      {
        v9 = (const char *)sub_79A7(0, 3, v47);
        v10 = __errno_location();
        error(0, *v10, "%s", v9);
        v24 = 0;
        break;
      }
      v36 += n;
    }
  }
LABEL_123:
  if ( v26 < (unsigned char)byte_F142 )
    v39 = v36;
  sub_312B(v35, v38, v39, v36, v40, a2);
  qword_F118 += v35;
  qword_F120 += v38;
  qword_F128 += v39;
  qword_F130 += v36;
  if ( v40 > qword_F138 )
    qword_F138 = v40;
  return v24;
}



// Function: wc_file @ 0x4263
long long wc_file(const char *a1, long long a2)
{
  const char *v3; // rbx
  int *v4; // rax
  unsigned char v5; // [rsp+1Bh] [rbp-15h]
  unsigned int fd; // [rsp+1Ch] [rbp-14h]

  if ( a1 && strcmp(a1, "-") )
  {
    fd = open(a1, 0);
    if ( fd == -1 || (v5 = sub_3591(fd, a1, a2, 0), close(fd)) )
    {
      v3 = (const char *)sub_79A7(0, 3, a1);
      v4 = __errno_location();
      error(0, *v4, "%s", v3);
      return 0;
    }
    else
    {
      return v5;
    }
  }
  else
  {
    byte_F14C = 1;
    sub_945E(0, 0);
    return sub_3591(0, a1, a2, -1);
  }
}



// Function: get_input_fstatus @ 0x43a5
int *get_input_fstatus(unsigned long long a1, long long a2)
{
  long long v2; // rax
  int v3; // edx
  unsigned long long i; // [rsp+10h] [rbp-10h]
  int *v6; // [rsp+18h] [rbp-8h]

  if ( a1 )
    v2 = a1;
  else
    v2 = 1;
  v6 = (int *)sub_8F54(v2, 152);
  if ( !a1
    || a1 == 1
    && (unsigned char)byte_F143
     + (unsigned char)byte_F142
     + (unsigned char)byte_F141
     + (unsigned char)byte_F140
     + (unsigned char)byte_F144 == 1 )
  {
    *v6 = 1;
  }
  else
  {
    for ( i = 0; i < a1; ++i )
    {
      if ( *(long long *)(8 * i + a2) && strcmp(*(const char **)(8 * i + a2), "-") )
        v3 = stat(*(const char **)(8 * i + a2), (struct stat *)&v6[38 * i + 2]);
      else
        v3 = fstat(0, (struct stat *)&v6[38 * i + 2]);
      v6[38 * i] = v3;
    }
  }
  return v6;
}



// Function: compute_number_width @ 0x4549
long long compute_number_width(unsigned long long a1, int *a2)
{
  int v3; // [rsp+18h] [rbp-18h]
  int v4; // [rsp+1Ch] [rbp-14h]
  unsigned long long v5; // [rsp+20h] [rbp-10h]
  unsigned long long i; // [rsp+28h] [rbp-8h]

  v3 = 1;
  if ( a1 && *a2 <= 0 )
  {
    v4 = 1;
    v5 = 0;
    for ( i = 0; i < a1; ++i )
    {
      if ( !a2[38 * i] )
      {
        if ( (a2[38 * i + 8] & 0xF000) == 0x8000 )
          v5 += *(long long *)&a2[38 * i + 14];
        else
          v4 = 7;
      }
    }
    while ( v5 > 9 )
    {
      ++v3;
      v5 /= 0xAu;
    }
    if ( v3 < v4 )
      return (unsigned int)v4;
  }
  return (unsigned int)v3;
}



// Function: main @ 0x4670
long long main(int a1, char **a2, char **a3)
{
  long long v3; // rbx
  char *v4; // rax
  char *v5; // rax
  long long v6; // r12
  char *v7; // rbx
  int *v8; // rax
  int v9; // eax
  double v10; // xmm0_8
  long long v11; // rbx
  char *v12; // rax
  char **v13; // rax
  long long v14; // rax
  long long v15; // r12
  char *v16; // rbx
  int *v17; // rax
  long long v18; // rbx
  char *v19; // rax
  char *v20; // rax
  char *v21; // rbx
  const char *v22; // rax
  long long v23; // rdx
  char *v24; // rax
  int *v25; // rax
  double st_size_low; // [rsp+8h] [rbp-238h]
  bool v28; // [rsp+21h] [rbp-21Fh]
  char v29; // [rsp+22h] [rbp-21Eh]
  char v30; // [rsp+23h] [rbp-21Dh]
  unsigned int v31; // [rsp+24h] [rbp-21Ch] BYREF
  int i; // [rsp+28h] [rbp-218h]
  int v33; // [rsp+2Ch] [rbp-214h]
  unsigned long long v34; // [rsp+30h] [rbp-210h]
  long long v35; // [rsp+38h] [rbp-208h]
  char *s1; // [rsp+40h] [rbp-200h]
  long long v37; // [rsp+48h] [rbp-1F8h]
  FILE *stream; // [rsp+50h] [rbp-1F0h]
  void *ptr; // [rsp+58h] [rbp-1E8h]
  char *v40; // [rsp+60h] [rbp-1E0h]
  long long v41; // [rsp+68h] [rbp-1D8h]
  unsigned long long v42; // [rsp+70h] [rbp-1D0h] BYREF
  long long v43; // [rsp+78h] [rbp-1C8h]
  struct stat buf; // [rsp+190h] [rbp-B0h] BYREF
  unsigned long long v45; // [rsp+228h] [rbp-18h]

  v45 = __readfsqword(0x28u);
  s1 = 0;
  sub_5C1B(*a2, a2, a3);
  setlocale(6, locale);
  bindtextdomain("coreutils", "/usr/local/share/locale");
  textdomain("coreutils");
  sub_AC00(sub_53BF);
  qword_F150 = getpagesize();
  setvbuf(stdout, 0, 1, 0);
  byte_F158 = getenv("POSIXLY_CORRECT") != 0;
  byte_F143 = 0;
  byte_F142 = 0;
  byte_F141 = 0;
  byte_F140 = 0;
  byte_F144 = 0;
  qword_F138 = 0;
  qword_F130 = 0;
  qword_F128 = 0;
  qword_F120 = 0;
  qword_F118 = 0;
  while ( 2 )
  {
    v33 = getopt_long(a1, a2, "clLmw", &longopts, 0);
    if ( v33 != -1 )
    {
      if ( v33 <= 129 )
      {
        if ( v33 >= 76 )
        {
          switch ( v33 )
          {
            case 76:
              byte_F144 = 1;
              continue;
            case 99:
              byte_F143 = 1;
              continue;
            case 108:
              byte_F140 = 1;
              continue;
            case 109:
              byte_F142 = 1;
              continue;
            case 119:
              byte_F141 = 1;
              continue;
            case 128:
              byte_F110 = 1;
              continue;
            case 129:
              s1 = (char *)optarg;
              continue;
            default:
              goto LABEL_17;
          }
        }
        if ( v33 == -131 )
        {
          sub_8BFB(
            (int)stdout,
            (unsigned int)"wc",
            (unsigned int)"GNU coreutils",
            (int)off_F018,
            (unsigned int)"Paul Rubin",
            (unsigned int)"David MacKenzie",
            0);
          exit(0);
        }
        if ( v33 == -130 )
          sub_2F7B(0);
      }
LABEL_17:
      sub_2F7B(1);
    }
    break;
  }
  if ( byte_F140 != 1 && byte_F141 != 1 && byte_F142 != 1 && byte_F143 != 1 && byte_F144 != 1 )
  {
    byte_F143 = 1;
    byte_F141 = 1;
    byte_F140 = 1;
  }
  v29 = 0;
  if ( s1 )
  {
    if ( a1 > optind )
    {
      v3 = sub_7815(4, a2[optind]);
      v4 = gettext("extra operand %s");
      error(0, 0, v4, v3);
      v5 = gettext("file operands cannot be combined with --files0-from");
      fprintf(stderr, "%s\n", v5);
      sub_2F7B(1);
    }
    if ( !strcmp(s1, "-") )
    {
      stream = (FILE *)stdin;
    }
    else
    {
      stream = fopen(s1, "r");
      if ( !stream )
      {
        v6 = sub_7815(4, s1);
        v7 = gettext("cannot open %s for reading");
        v8 = __errno_location();
        error(1, *v8, v7, v6);
      }
    }
    v9 = fileno(stream);
    if ( fstat(v9, &buf)
      || (buf.st_mode & 0xF000) != 0x8000
      || ((st_size_low = (double)SLODWORD(buf.st_size), sub_5AA9() / 2.0 <= 10485760.0)
        ? (v10 = sub_5AA9() / 2.0)
        : (v10 = 10485760.0),
          v10 < st_size_low) )
    {
      v35 = 0;
      v34 = 0;
      v37 = sub_51BA(stream);
    }
    else
    {
      v29 = 1;
      sub_7C7F(&v42);
      if ( (unsigned char)sub_809E(stream, &v42) != 1 || (unsigned int)sub_552B(stream) )
      {
        v11 = sub_7815(4, s1);
        v12 = gettext("cannot read file names from %s");
        error(1, 0, v12, v11);
      }
      v35 = v43;
      v34 = v42;
      v37 = sub_5165(v43);
    }
  }
  else
  {
    if ( a1 <= optind )
      v13 = (char **)&unk_F160;
    else
      v13 = &a2[optind];
    v35 = (long long)v13;
    if ( a1 <= optind )
      v14 = 1;
    else
      v14 = a1 - optind;
    v34 = v14;
    v37 = sub_5165(v35);
  }
  if ( !v37 )
    sub_9413();
  ptr = sub_43A5(v34, v35);
  dword_F148 = sub_4549(v34, (int *)ptr);
  v28 = 1;
  for ( i = 0; ; ++i )
  {
    v30 = 0;
    v40 = (char *)sub_5227(v37, &v31);
    if ( !v40 )
      break;
    if ( s1 && !strcmp(s1, "-") && !strcmp(v40, "-") )
    {
      v18 = sub_7815(4, v40);
      v19 = gettext("when reading file names from stdin, no file name of %s allowed");
      error(0, 0, v19, v18);
      v30 = 1;
    }
    if ( !*v40 )
    {
      if ( s1 )
      {
        v41 = sub_5312(v37);
        v21 = gettext("invalid zero-length file name");
        v22 = (const char *)sub_79A7(0, 3, s1);
        error(0, 0, "%s:%lu: %s", v22, v41, v21);
      }
      else
      {
        v20 = gettext("invalid zero-length file name");
        error(0, 0, "%s", v20);
      }
      v30 = 1;
    }
    if ( v30 )
    {
      v28 = 0;
    }
    else
    {
      if ( v34 )
        v23 = 152LL * i;
      else
        v23 = 0;
      v28 = (v28 & (unsigned char)sub_4263(v40, (long long)ptr + v23)) != 0;
    }
    if ( !v34 )
      *(int *)ptr = 1;
  }
  if ( v31 == 4 )
  {
    v15 = sub_79A7(0, 3, s1);
    v16 = gettext("%s: read error");
    v17 = __errno_location();
    error(0, *v17, v16, v15);
    v28 = 0;
  }
  else
  {
    if ( v31 > 4 )
      goto LABEL_61;
    if ( v31 != 2 )
    {
      if ( v31 == 3 )
        sub_9413();
LABEL_61:
      __assert_fail("!\"unexpected error code from argv_iter\"", "src/wc.c", 0x3AAu, "main");
    }
  }
  if ( v28 && !s1 && !sub_5312(v37) )
    v28 = (v28 & (unsigned char)sub_4263(0, (long long)ptr)) != 0;
  if ( v29 )
    sub_7D41(&v42);
  if ( (unsigned long long)sub_5312(v37) > 1 )
  {
    v24 = gettext("total");
    sub_312B(qword_F118, qword_F120, qword_F128, qword_F130, qword_F138, v24);
  }
  sub_5350(v37);
  free(ptr);
  if ( byte_F14C && close(0) )
  {
    v25 = __errno_location();
    error(1, *v25, "-");
  }
  return !v28;
}



// Function: wc_lines_avx2 @ 0xa1b1
long long wc_lines_avx2(long long a1, unsigned int a2, long long *a3, long long *a4, __m128 _XMM0)
{
  const char *v101; // rbx
  int *v102; // rax
  int v135; // edx
  int v139; // edx
  int v144; // edx
  int v158; // edx
  int v162; // edx
  int v167; // edx
  char *v172; // rax
  char v173; // [rsp+0h] [rbp-4028h] BYREF
  int v174; // [rsp+2C4Ch] [rbp-13DCh]
  int v175; // [rsp+2C50h] [rbp-13D8h]
  int v176; // [rsp+2C54h] [rbp-13D4h]
  int v177; // [rsp+2C58h] [rbp-13D0h]
  int v178; // [rsp+2C5Ch] [rbp-13CCh]
  int v179; // [rsp+2C60h] [rbp-13C8h]
  int v180; // [rsp+2C64h] [rbp-13C4h]
  int v181; // [rsp+2C68h] [rbp-13C0h]
  int v182; // [rsp+2C6Ch] [rbp-13BCh]
  int v183; // [rsp+2C70h] [rbp-13B8h]
  int v184; // [rsp+2C74h] [rbp-13B4h]
  int v185; // [rsp+2C78h] [rbp-13B0h]
  int v186; // [rsp+2C7Ch] [rbp-13ACh]
  int v187; // [rsp+2C80h] [rbp-13A8h]
  int v188; // [rsp+2C84h] [rbp-13A4h]
  int v189; // [rsp+2C88h] [rbp-13A0h]
  int v190; // [rsp+2C8Ch] [rbp-139Ch]
  long long *v191; // [rsp+2C90h] [rbp-1398h]
  long long *v192; // [rsp+2C98h] [rbp-1390h]
  int v193; // [rsp+2CA0h] [rbp-1388h]
  unsigned int v194; // [rsp+2CA4h] [rbp-1384h]
  long long v195; // [rsp+2CA8h] [rbp-1380h]
  char v196[33]; // [rsp+2CBFh] [rbp-1369h] BYREF
  char *v197; // [rsp+2CE0h] [rbp-1348h]
  long long v198; // [rsp+2CE8h] [rbp-1340h]
  long long v199; // [rsp+2CF0h] [rbp-1338h]
  unsigned long long v200; // [rsp+2CF8h] [rbp-1330h]
  char *v201; // [rsp+2D00h] [rbp-1328h]
  char *v202; // [rsp+2D08h] [rbp-1320h]
  char *v203; // [rsp+2D10h] [rbp-1318h]
  char *v204; // [rsp+2D18h] [rbp-1310h]
  __m256 v232; // [rsp+3000h] [rbp-1028h] BYREF
  char v233[4104]; // [rsp+3020h] [rbp-1008h] BYREF

  while ( &v232 != (__m256 *)&v173 )
    ;
  v195 = a1;
  v194 = a2;
  v192 = a3;
  v191 = a4;
  STACK[0x6FF8] = __readfsqword(0x28u);
  v198 = 0;
  v199 = 0;
  v200 = 0;
  if ( !v192 || !v191 )
    return 0;
  __asm
  {
    vpxor   xmm0, xmm0, xmm0
    vmovdqa [rsp+13E8h+var_1288], ymm0
    vpxor   xmm0, xmm0, xmm0
    vmovdqa [rsp+13E8h+var_1268], ymm0
    vpxor   xmm0, xmm0, xmm0
    vmovdqa [rsp+13E8h+var_1248], ymm0
  }
  memset(v196, 10, sizeof(v196));
  v190 = 10;
  v189 = 10;
  v188 = 10;
  v187 = 10;
  v193 = 10;
  v186 = 10;
  v185 = 10;
  v184 = 10;
  v183 = 10;
  v182 = 10;
  v181 = 10;
  v180 = 10;
  v179 = 10;
  v178 = 10;
  v177 = 10;
  v176 = 10;
  v175 = 10;
  v174 = 10;
  _EAX = 10;
  __asm { vmovd   xmm0, eax }
  __asm
  {
    vpinsrb xmm0, xmm0, eax, 1
    vmovdqa xmm1, xmm0
  }
  _EAX = 10;
  __asm { vmovd   xmm0, eax }
  __asm
  {
    vpinsrb xmm0, xmm0, eax, 1
    vmovdqa xmm8, xmm0
  }
  _EAX = 10;
  __asm { vmovd   xmm0, eax }
  __asm
  {
    vpinsrb xmm0, xmm0, eax, 1
    vmovdqa xmm4, xmm0
  }
  _EAX = 10;
  __asm { vmovd   xmm0, eax }
  __asm
  {
    vpinsrb xmm0, xmm0, eax, 1
    vmovdqa xmm7, xmm0
  }
  _EAX = 10;
  __asm { vmovd   xmm0, eax }
  __asm
  {
    vpinsrb xmm0, xmm0, eax, 1
    vmovdqa xmm2, xmm0
  }
  _EAX = 10;
  __asm { vmovd   xmm0, eax }
  __asm
  {
    vpinsrb xmm0, xmm0, eax, 1
    vmovdqa xmm6, xmm0
  }
  _EAX = 10;
  __asm { vmovd   xmm0, eax }
  __asm
  {
    vpinsrb xmm0, xmm0, eax, 1
    vmovdqa xmm3, xmm0
  }
  _EAX = 10;
  __asm { vmovd   xmm0, eax }
  __asm
  {
    vpinsrb xmm0, xmm0, eax, 1
    vmovdqa xmm5, xmm0
    vpunpcklwd xmm0, xmm1, xmm8
    vmovdqa xmm1, xmm0
    vpunpcklwd xmm0, xmm4, xmm7
    vmovdqa xmm4, xmm0
    vpunpcklwd xmm0, xmm2, xmm6
    vmovdqa xmm2, xmm0
    vpunpcklwd xmm0, xmm3, xmm5
    vmovdqa xmm3, xmm0
    vpunpckldq xmm0, xmm1, xmm4
    vmovdqa xmm1, xmm0
    vpunpckldq xmm0, xmm2, xmm3
    vpunpcklqdq xmm0, xmm1, xmm0
    vmovdqa xmm1, xmm0
  }
  _EAX = 10;
  __asm { vmovd   xmm0, eax }
  __asm
  {
    vpinsrb xmm0, xmm0, eax, 1
    vmovdqa xmm2, xmm0
  }
  _EAX = 10;
  __asm { vmovd   xmm0, eax }
  __asm
  {
    vpinsrb xmm0, xmm0, eax, 1
    vmovdqa xmm9, xmm0
  }
  _EAX = 10;
  __asm { vmovd   xmm0, eax }
  __asm
  {
    vpinsrb xmm0, xmm0, eax, 1
    vmovdqa xmm5, xmm0
  }
  _EAX = 10;
  __asm { vmovd   xmm0, eax }
  __asm
  {
    vpinsrb xmm0, xmm0, eax, 1
    vmovdqa xmm8, xmm0
  }
  _EAX = 10;
  __asm { vmovd   xmm0, eax }
  __asm
  {
    vpinsrb xmm0, xmm0, eax, 1
    vmovdqa xmm3, xmm0
  }
  _EAX = 10;
  __asm { vmovd   xmm0, eax }
  __asm
  {
    vpinsrb xmm0, xmm0, eax, 1
    vmovdqa xmm7, xmm0
  }
  _EAX = 10;
  __asm { vmovd   xmm0, eax }
  __asm
  {
    vpinsrb xmm0, xmm0, eax, 1
    vmovdqa xmm4, xmm0
  }
  _EAX = 10;
  __asm { vmovd   xmm0, eax }
  __asm
  {
    vpinsrb xmm0, xmm0, eax, 1
    vmovdqa xmm6, xmm0
    vpunpcklwd xmm0, xmm2, xmm9
    vmovdqa xmm2, xmm0
    vpunpcklwd xmm0, xmm5, xmm8
    vmovdqa xmm5, xmm0
    vpunpcklwd xmm0, xmm3, xmm7
    vmovdqa xmm3, xmm0
    vpunpcklwd xmm0, xmm4, xmm6
    vmovdqa xmm4, xmm0
    vpunpckldq xmm0, xmm2, xmm5
    vmovdqa xmm2, xmm0
    vpunpckldq xmm0, xmm3, xmm4
    vpunpcklqdq xmm0, xmm2, xmm0
    vinserti128 ymm0, ymm1, xmm0, 1
  }
  __asm { vmovdqa [rsp+13E8h+var_1228], ymm0 }
  while ( 1 )
  {
    v200 = sub_8412(v194, v233, 16320);
    if ( !v200 )
      break;
    if ( v200 == -1 )
    {
      v101 = (const char *)sub_79A7(0, 3, v195);
      v102 = __errno_location();
      error(0, *v102, "%s", v101);
      return 0;
    }
    v199 += v200;
    v197 = v233;
    v202 = &v233[v200];
    while ( v200 > 0x3F )
    {
      v204 = v197;
      _RAX = v197;
      __asm
      {
        vmovdqa ymm0, ymmword ptr [rax]
        vmovdqa [rsp+13E8h+var_1208], ymm0
      }
      v203 = v197 + 32;
      _RAX = v197 + 32;
      __asm
      {
        vmovdqa ymm0, ymmword ptr [rax]
        vmovdqa [rsp+13E8h+var_11E8], ymm0
        vmovdqa ymm0, [rsp+13E8h+var_1208]
        vmovdqa [rsp+13E8h+var_10C8], ymm0
        vmovdqa ymm0, [rsp+13E8h+var_1228]
        vmovdqa [rsp+13E8h+var_10A8], ymm0
        vmovdqa ymm1, [rsp+13E8h+var_10C8]
        vmovdqa ymm0, [rsp+13E8h+var_10A8]
        vpcmpeqb ymm0, ymm1, ymm0
        vmovdqa [rsp+13E8h+var_11C8], ymm0
        vmovdqa ymm0, [rsp+13E8h+var_11E8]
        vmovdqa [rsp+13E8h+var_1108], ymm0
        vmovdqa ymm0, [rsp+13E8h+var_1228]
        vmovdqa [rsp+13E8h+var_10E8], ymm0
        vmovdqa ymm1, [rsp+13E8h+var_1108]
        vmovdqa ymm0, [rsp+13E8h+var_10E8]
        vpcmpeqb ymm0, ymm1, ymm0
        vmovdqa [rsp+13E8h+var_11A8], ymm0
        vmovdqa ymm0, [rsp+13E8h+var_1288]
        vmovdqa [rsp+13E8h+var_1148], ymm0
        vmovdqa ymm0, [rsp+13E8h+var_11C8]
        vmovdqa [rsp+13E8h+var_1128], ymm0
        vmovdqa ymm0, [rsp+13E8h+var_1148]
        vmovdqa ymm1, [rsp+13E8h+var_1128]
        vpsubb  ymm0, ymm0, ymm1
        vmovdqa [rsp+13E8h+var_1288], ymm0
        vmovdqa ymm0, [rsp+13E8h+var_1268]
        vmovdqa [rsp+13E8h+var_1188], ymm0
        vmovdqa ymm0, [rsp+13E8h+var_11A8]
        vmovdqa [rsp+13E8h+var_1168], ymm0
        vmovdqa ymm0, [rsp+13E8h+var_1188]
        vmovdqa ymm1, [rsp+13E8h+var_1168]
        vpsubb  ymm0, ymm0, ymm1
      }
      __asm { vmovdqa [rsp+13E8h+var_1268], ymm0 }
      v197 += 64;
      v200 -= 64LL;
    }
    __asm
    {
      vmovdqa ymm0, [rsp+13E8h+var_1288]
      vmovdqa [rsp+13E8h+var_1048], ymm0
      vmovdqa ymm0, [rsp+13E8h+var_1248]
      vmovdqa [rsp+13E8h+var_1028], ymm0
      vmovdqa ymm1, [rsp+13E8h+var_1028]
      vmovdqa ymm0, [rsp+13E8h+var_1048]
      vpsadbw ymm0, ymm0, ymm1
      vmovdqa [rsp+13E8h+var_1288], ymm0
      vmovdqa ymm0, [rsp+13E8h+var_1288]
      vmovdqa [rsp+13E8h+var_1308], xmm0
      vmovdqa xmm0, [rsp+13E8h+var_1308]
      vpextrw rax, xmm0, 0
    }
    v135 = (unsigned short)_RAX;
    __asm
    {
      vmovdqa ymm0, [rsp+13E8h+var_1288]
      vmovdqa [rsp+13E8h+var_12F8], xmm0
      vmovdqa xmm0, [rsp+13E8h+var_12F8]
      vpextrw rax, xmm0, 4
    }
    v139 = (unsigned short)_RAX + v135;
    __asm
    {
      vmovdqa ymm0, [rsp+13E8h+var_1288]
      vextracti128 xmm0, ymm0, 1
      vmovdqa [rsp+13E8h+var_12E8], xmm0
      vmovdqa xmm0, [rsp+13E8h+var_12E8]
      vpextrw rax, xmm0, 0
    }
    v144 = (unsigned short)_RAX + v139;
    __asm
    {
      vmovdqa ymm0, [rsp+13E8h+var_1288]
      vextracti128 xmm0, ymm0, 1
      vmovdqa [rsp+13E8h+var_12D8], xmm0
      vmovdqa xmm0, [rsp+13E8h+var_12D8]
      vpextrw rax, xmm0, 4
    }
    v198 += v144 + (unsigned short)_RAX;
    __asm
    {
      vpxor   xmm0, xmm0, xmm0
      vmovdqa [rsp+13E8h+var_1288], ymm0
      vmovdqa ymm0, [rsp+13E8h+var_1268]
      vmovdqa [rsp+13E8h+var_1088], ymm0
      vmovdqa ymm0, [rsp+13E8h+var_1248]
      vmovdqa [rsp+13E8h+var_1068], ymm0
      vmovdqa ymm1, [rsp+13E8h+var_1068]
      vmovdqa ymm0, [rsp+13E8h+var_1088]
      vpsadbw ymm0, ymm0, ymm1
      vmovdqa [rsp+13E8h+var_1268], ymm0
      vmovdqa ymm0, [rsp+13E8h+var_1268]
      vmovdqa [rsp+13E8h+var_12C8], xmm0
      vmovdqa xmm0, [rsp+13E8h+var_12C8]
      vpextrw rax, xmm0, 0
    }
    v158 = (unsigned short)_RAX;
    __asm
    {
      vmovdqa ymm0, [rsp+13E8h+var_1268]
      vmovdqa [rsp+13E8h+var_12B8], xmm0
      vmovdqa xmm0, [rsp+13E8h+var_12B8]
      vpextrw rax, xmm0, 4
    }
    v162 = (unsigned short)_RAX + v158;
    __asm
    {
      vmovdqa ymm0, [rsp+13E8h+var_1268]
      vextracti128 xmm0, ymm0, 1
      vmovdqa [rsp+13E8h+var_12A8], xmm0
      vmovdqa xmm0, [rsp+13E8h+var_12A8]
      vpextrw rax, xmm0, 0
    }
    v167 = (unsigned short)_RAX + v162;
    __asm
    {
      vmovdqa ymm0, [rsp+13E8h+var_1268]
      vextracti128 xmm0, ymm0, 1
      vmovdqa [rsp+13E8h+var_1298], xmm0
      vmovdqa xmm0, [rsp+13E8h+var_1298]
      vpextrw rax, xmm0, 4
    }
    v198 += v167 + (unsigned short)_RAX;
    __asm
    {
      vpxor   xmm0, xmm0, xmm0
      vmovdqa [rsp+13E8h+var_1268], ymm0
    }
    v201 = v197;
    while ( v201 != v202 )
    {
      v172 = v201++;
      v198 += *v172 == 10;
    }
  }
  *v192 = v198;
  *v191 = v199;
  return 1;
}


