// Function: mv_begin_write @ 0xab09
unsigned long long mv_begin_write(long long a1, long long a2, long long a3)
{
  unsigned long long result; // rax
  long long *v5; // [rsp+28h] [rbp-8h]

  result = (unsigned char)byte_9F8FC;
  if ( byte_9F8FC )
  {
    v5 = (long long *)sub_60B5B(48);
    if ( qword_9F068 )
      *(long long *)qword_9F068 = v5;
    else
      qword_9F060 = (long long)v5;
    qword_9F068 = (long long)v5;
    *v5 = 0;
    v5[1] = ((char *)qword_9EFB0 - (char *)s1) >> 9;
    v5[2] = sub_60CC6(a1);
    v5[3] = a2;
    v5[4] = a3;
    v5[5] = 0;
    return (unsigned long long)v5;
  }
  return result;
}



// Function: bufmap_locate @ 0xabd9
long long bufmap_locate(unsigned long long a1)
{
  long long i; // [rsp+10h] [rbp-8h]

  for ( i = qword_9F060; i && *(long long *)i && a1 >= *(long long *)(*(long long *)i + 8LL) << 9; i = *(long long *)i )
    ;
  return i;
}



// Function: bufmap_free @ 0xac2b
void **bufmap_free(void **a1)
{
  void **result; // rax
  void **ptr; // [rsp+10h] [rbp-10h]
  void **v3; // [rsp+18h] [rbp-8h]

  for ( ptr = (void **)qword_9F060; ptr && ptr != a1; ptr = v3 )
  {
    v3 = (void **)*ptr;
    free(ptr[2]);
    free(ptr);
  }
  qword_9F060 = (long long)ptr;
  result = ptr;
  if ( !ptr )
  {
    qword_9F068 = 0;
    return 0;
  }
  return result;
}



// Function: bufmap_reset @ 0xacb0
void **bufmap_reset(void **a1, long long a2)
{
  void **result; // rax
  void **v3; // [rsp+8h] [rbp-8h]

  v3 = a1;
  result = sub_AC2B(a1);
  if ( a1 )
  {
    while ( v3 )
    {
      v3[1] = (char *)v3[1] + a2;
      v3[5] = 0;
      result = (void **)*v3;
      v3 = (void **)*v3;
    }
  }
  return result;
}



// Function: buffer_write_global_xheader @ 0xad11
long long buffer_write_global_xheader()
{
  return sub_1D3E6(&unk_9F1E8);
}



// Function: mv_begin_read @ 0xad2b
unsigned long long mv_begin_read(long long *a1)
{
  return sub_AB09(*a1, a1[17], a1[17]);
}



// Function: mv_end @ 0xad66
void **mv_end()
{
  void **result; // rax

  result = (void **)(unsigned char)byte_9F8FC;
  if ( byte_9F8FC )
    return sub_AC2B(0);
  return result;
}



// Function: mv_size_left @ 0xad86
long long mv_size_left(long long a1)
{
  long long result; // rax

  result = qword_9F060;
  if ( qword_9F060 )
  {
    result = qword_9F060;
    *(long long *)(qword_9F060 + 32) = a1;
  }
  return result;
}



// Function: clear_read_error_count @ 0xadb0
void clear_read_error_count()
{
  dword_9F044 = 0;
}



// Function: set_start_time @ 0xadc5
long long set_start_time()
{
  sub_521FA(&qword_9F9E0);
  qword_9F9F0 = qword_9F9E0;
  qword_9F9F8 = qword_9F9E8;
  qword_9FA00 = qword_9F9E0;
  qword_9FA08 = qword_9F9E8;
  return qword_9F9E0;
}



// Function: set_volume_start_time @ 0xae17
long long set_volume_start_time()
{
  sub_521FA(&qword_9F9F0);
  qword_9FA00 = qword_9F9F0;
  qword_9FA08 = qword_9F9F8;
  return qword_9F9F0;
}



// Function: compute_duration @ 0xae4d
double compute_duration()
{
  long long v1; // [rsp+0h] [rbp-20h] BYREF
  long long v2; // [rsp+8h] [rbp-18h]
  unsigned long long v3; // [rsp+18h] [rbp-8h]

  v3 = __readfsqword(0x28u);
  sub_521FA(&v1);
  *(double *)&qword_9F238 = *(double *)&qword_9F238
                          + (double)((int)v1 - (int)qword_9FA00)
                          + (double)((int)v2 - (int)qword_9FA08) / 1000000000.0;
  sub_521FA(&qword_9FA00);
  return *(double *)&qword_9F238;
}



// Function: find_zip_program @ 0xaefd
char *find_zip_program(int a1, int *a2)
{
  int i; // [rsp+1Ch] [rbp-4h]

  for ( i = *a2; *((int *)&unk_9C020 + 6 * i); ++i )
  {
    if ( a1 == *((int *)&unk_9C020 + 6 * i) )
    {
      *a2 = i + 1;
      return (char *)&unk_9C020 + 24 * i;
    }
  }
  *a2 = i;
  return 0;
}



// Function: first_decompress_program @ 0xafa2
char *first_decompress_program(int *a1)
{
  char *v2; // [rsp+10h] [rbp-8h]

  if ( qword_9F898 )
    return qword_9F898;
  if ( !dword_9F240 )
    return 0;
  *a1 = 0;
  v2 = sub_AEFD(dword_9F240, a1);
  if ( v2 )
    return (char *)*((long long *)v2 + 1);
  else
    return 0;
}



// Function: next_decompress_program @ 0xb012
long long next_decompress_program(int *a1)
{
  char *v2; // [rsp+10h] [rbp-8h]

  if ( qword_9F898 )
    return 0;
  v2 = sub_AEFD(dword_9F240, a1);
  if ( v2 )
    return *((long long *)v2 + 1);
  else
    return 0;
}



// Function: compress_option @ 0xb065
long long compress_option(int a1)
{
  int v2; // [rsp+1Ch] [rbp-14h] BYREF
  char *v3; // [rsp+20h] [rbp-10h]
  unsigned long long v4; // [rsp+28h] [rbp-8h]

  v4 = __readfsqword(0x28u);
  v2 = 0;
  v3 = sub_AEFD(a1, &v2);
  if ( v3 )
    return *((long long *)v3 + 2);
  else
    return 0;
}



// Function: check_compressed_archive @ 0xb0cb
long long check_compressed_archive(bool *a1)
{
  bool *v2; // [rsp+8h] [rbp-28h]
  char v3; // [rsp+1Eh] [rbp-12h] BYREF
  char v4; // [rsp+1Fh] [rbp-11h]
  const void **i; // [rsp+20h] [rbp-10h]
  unsigned long long v6; // [rsp+28h] [rbp-8h]

  v2 = a1;
  v6 = __readfsqword(0x28u);
  if ( !a1 )
    v2 = (bool *)&v3;
  s = s1;
  v4 = byte_9F049;
  byte_9F049 = 1;
  *v2 = sub_BB4B() == 0;
  byte_9F049 = v4;
  if ( s1 != s
    && (!strcmp((const char *)s1 + 257, "ustar") || !strcmp((const char *)s1 + 257, "ustar  "))
    && (unsigned int)sub_25D84(s1, 1) == 1 )
  {
    return 1;
  }
  for ( i = (const void **)&unk_9A170; i < (const void **)off_9A230; i += 3 )
  {
    if ( !memcmp(s1, i[2], (size_t)i[1]) )
      return *(unsigned int *)i;
  }
  return 0;
}



// Function: guess_seekable_archive @ 0xb214
unsigned long long guess_seekable_archive()
{
  struct stat buf; // [rsp+0h] [rbp-A0h] BYREF
  unsigned long long v2; // [rsp+98h] [rbp-8h]

  v2 = __readfsqword(0x28u);
  if ( dword_9F860 == 4 )
    byte_9FC18 = 0;
  if ( dword_9FC14 == -1 )
    byte_9FC18 = byte_9F8FC != 1 && !qword_9F898 && !fstat(fd, &buf) && (buf.st_mode & 0xF000) == 0x8000;
  else
    byte_9FC18 = dword_9FC14 != 0;
  return v2 - __readfsqword(0x28u);
}



// Function: open_compressed_archive @ 0xb2cf
long long open_compressed_archive()
{
  int v0; // eax
  char *v2; // rax
  char *v3; // rax
  bool v4; // [rsp+3h] [rbp-Dh] BYREF
  int v5; // [rsp+4h] [rbp-Ch]
  unsigned long long v6; // [rsp+8h] [rbp-8h]

  v6 = __readfsqword(0x28u);
  if ( byte_9FDF8 != 1
    && (qword_9FDF0 = (long long)strchr(*(const char **)qword_9FBD8, 58)) != 0
    && *(long long *)qword_9FBD8 < (unsigned long long)qword_9FDF0
    && !memchr(*(const void **)qword_9FBD8, 47, qword_9FDF0 - *(long long *)qword_9FBD8) )
  {
    v0 = sub_3F860(*(long long *)qword_9FBD8, 0, 0x40000000, qword_9F950);
  }
  else
  {
    v0 = open(*(const char **)qword_9FBD8, 0, 438);
  }
  fd = v0;
  if ( v0 == -1 )
    return 0xFFFFFFFFLL;
  if ( byte_9F8FC == 1 )
    goto LABEL_29;
  if ( qword_9F898 )
  {
LABEL_25:
    if ( fd <= 0x3FFFFFFF )
      close(fd);
    else
      sub_3FE0C((unsigned int)(fd - 0x40000000));
    byte_9F048 = 0;
    dword_9F040 = sub_3399B();
    byte_9F049 = 1;
LABEL_29:
    qword_9EFC0 = 0;
    s = s1;
    return (unsigned int)fd;
  }
  v5 = sub_B0CB(&v4);
  if ( !v5 )
  {
    if ( v4 )
    {
      if ( qword_9FDC8 )
        qword_9FDC8();
      v3 = gettext("This does not look like a tar archive");
      error(0, 0, v3);
      dword_9FDD0 = 2;
    }
    sub_32970(*(long long *)qword_9FBD8, 0);
    if ( !qword_9F898 )
      return (unsigned int)fd;
    goto LABEL_25;
  }
  if ( v5 != 1 )
  {
    dword_9F240 = v5;
    goto LABEL_25;
  }
  if ( v4 )
  {
    if ( qword_9FDC8 )
      qword_9FDC8();
    v2 = gettext("This does not look like a tar archive");
    error(0, 0, v2);
    dword_9FDD0 = 2;
  }
  return (unsigned int)fd;
}



// Function: print_stats @ 0xb570
long long print_stats(FILE *a1, const char *a2, double a3)
{
  char *v3; // rax
  double v4; // xmm0_8
  unsigned long long v5; // rax
  const char *v6; // rbx
  unsigned long long v7; // rax
  const char *v8; // rax
  int v11; // [rsp+28h] [rbp-548h]
  char v12[656]; // [rsp+30h] [rbp-540h] BYREF
  char v13[664]; // [rsp+2C0h] [rbp-2B0h] BYREF
  unsigned long long v14; // [rsp+558h] [rbp-18h]

  v14 = __readfsqword(0x28u);
  v11 = 0;
  if ( a2 && *a2 )
  {
    v3 = gettext(a2);
    v11 = fprintf(a1, "%s: ", v3);
  }
  if ( *(double *)&qword_9F238 <= 0.0 || a3 / *(double *)&qword_9F238 >= 1.844674407370955e19 )
  {
    v6 = "?";
  }
  else
  {
    v4 = a3 / *(double *)&qword_9F238;
    if ( a3 / *(double *)&qword_9F238 >= 9.223372036854776e18 )
      v5 = (unsigned int)(int)(v4 - 9.223372036854776e18) ^ 0x8000000000000000LL;
    else
      v5 = (unsigned int)(int)v4;
    v6 = (const char *)sub_53F1C(v5, v13, 432, 1, 1);
  }
  if ( a3 >= 9.223372036854776e18 )
    v7 = (unsigned int)(int)(a3 - 9.223372036854776e18) ^ 0x8000000000000000LL;
  else
    v7 = (unsigned int)(int)a3;
  v8 = (const char *)sub_53F1C(v7, v12, 432, 1, 1);
  return (unsigned int)(v11 + fprintf(a1, "%.0f (%s, %s/s)", a3, v8, v6));
}



// Function: format_total_stats @ 0xb768
long long format_total_stats(FILE *a1, const char **a2, int a3, int a4)
{
  unsigned long long v4; // rax
  double v5; // rax
  char *v6; // rax
  long long v7; // rax
  double v8; // xmm0_8
  double v9; // xmm0_8
  unsigned long long v10; // rax
  const char *v11; // rax
  unsigned long long v12; // rax
  double v13; // rbx
  char *v14; // rax
  unsigned int v18; // [rsp+2Ch] [rbp-34h]
  int v19; // [rsp+2Ch] [rbp-34h]
  int v20; // [rsp+2Ch] [rbp-34h]
  int v21; // [rsp+2Ch] [rbp-34h]
  char v22[24]; // [rsp+30h] [rbp-30h] BYREF
  unsigned long long v23; // [rsp+48h] [rbp-18h]

  v23 = __readfsqword(0x28u);
  if ( dword_9F860 == 8 )
  {
LABEL_8:
    v18 = sub_B570(a1, a2[1], *(double *)&qword_9F000 + *(double *)&qword_9F008);
    goto LABEL_27;
  }
  if ( (unsigned int)dword_9F860 >= 8 )
    goto LABEL_26;
  if ( (unsigned int)dword_9F860 >= 5 )
  {
    v12 = qword_9EFC0 * qword_9F870;
    if ( qword_9EFC0 * qword_9F870 < 0 )
      v13 = (double)(int)(v12 & 1 | (v12 >> 1)) + (double)(int)(v12 & 1 | (v12 >> 1));
    else
      v13 = (double)(int)v12;
    v14 = gettext(*a2);
    v18 = sub_B570(a1, v14, v13);
    goto LABEL_27;
  }
  if ( (unsigned int)dword_9F860 <= 3 )
  {
    if ( !dword_9F860 )
LABEL_26:
      abort();
    goto LABEL_8;
  }
  v4 = qword_9EFC0 * qword_9F870;
  if ( qword_9EFC0 * qword_9F870 < 0 )
    v5 = (double)(int)(v4 & 1 | (v4 >> 1)) + (double)(int)(v4 & 1 | (v4 >> 1));
  else
    v5 = (double)(int)v4;
  v19 = sub_B570(a1, *a2, v5);
  fputc_unlocked(a3, a1);
  v20 = sub_B570(a1, a2[1], *(double *)&qword_9F000 + *(double *)&qword_9F008) + v19 + 1;
  fputc_unlocked(a3, a1);
  v21 = v20 + 1;
  if ( a2[2] && *a2[2] )
  {
    v6 = gettext(a2[2]);
    v21 += fprintf(a1, "%s: ", v6);
  }
  v7 = (qword_9EFC0 - qword_9F370) * qword_9F870;
  if ( v7 < 0 )
    v8 = (double)(int)(v7 & 1 | ((unsigned long long)v7 >> 1)) + (double)(int)(v7 & 1 | ((unsigned long long)v7 >> 1));
  else
    v8 = (double)(int)v7;
  v9 = v8 - (*(double *)&qword_9F008 + *(double *)&qword_9F000);
  if ( v9 >= 9.223372036854776e18 )
    v10 = (unsigned int)(int)(v9 - 9.223372036854776e18) ^ 0x8000000000000000LL;
  else
    v10 = (unsigned int)(int)v9;
  v11 = (const char *)sub_54E5A(v10, v22);
  v18 = fprintf(a1, "%s", v11) + v21;
LABEL_27:
  if ( a4 )
  {
    fputc_unlocked(a4, a1);
    ++v18;
  }
  return v18;
}



// Function: print_total_stats @ 0xbaa2
long long print_total_stats()
{
  return sub_B768(stderr, (const char **)off_9A230, 10, 10);
}



// Function: current_block_ordinal @ 0xbacd
signed long long current_block_ordinal()
{
  return (((char *)qword_9EFB0 - (char *)s1) >> 9) + qword_9F038;
}



// Function: reset_eof @ 0xbaf9
unsigned long long reset_eof()
{
  unsigned long long result; // rax

  result = (unsigned char)byte_9F048;
  if ( byte_9F048 )
  {
    byte_9F048 = 0;
    qword_9EFB0 = s1;
    result = (unsigned long long)s1 + 512 * (long long)dword_9F868;
    s = (void *)result;
    dword_9EFB8 = 1;
  }
  return result;
}



// Function: find_next_block @ 0xbb4b
void *find_next_block()
{
  if ( qword_9EFB0 != s )
    return qword_9EFB0;
  if ( byte_9F048 )
    return 0;
  sub_CAC8();
  if ( qword_9EFB0 != s )
    return qword_9EFB0;
  byte_9F048 = 1;
  return 0;
}



// Function: set_next_block_after @ 0xbba7
void *set_next_block_after(unsigned long long a1)
{
  void *result; // rax

  while ( a1 >= (unsigned long long)qword_9EFB0 )
    qword_9EFB0 = (char *)qword_9EFB0 + 512;
  result = s;
  if ( qword_9EFB0 > s )
    abort();
  return result;
}



// Function: available_space_after @ 0xbbf5
char *available_space_after(long long a1)
{
  return (char *)s - a1;
}



// Function: xclose @ 0xbc11
int xclose(int a1)
{
  int result; // eax
  char *v2; // rax

  result = close(a1);
  if ( result )
  {
    v2 = gettext("(pipe)");
    return sub_3E1FF(v2);
  }
  return result;
}



// Function: init_buffer @ 0xbc48
char *init_buffer()
{
  int v0; // ebx
  char *result; // rax

  if ( !qword_9F020[dword_9F030] )
  {
    v0 = dword_9F030;
    qword_9F020[v0] = sub_2B601(&ptr + dword_9F030, qword_9F870);
  }
  s1 = (void *)qword_9F020[dword_9F030];
  qword_9EFB0 = s1;
  result = (char *)s1 + 512 * (long long)dword_9F868;
  s = result;
  return result;
}



// Function: check_tty @ 0xbd12
int check_tty(int a1)
{
  int result; // eax
  char *v2; // rax

  result = strcmp(*(const char **)qword_9FBD8, "-");
  if ( !result )
  {
    result = isatty(a1 != 0);
    if ( result )
    {
      if ( qword_9FDC8 )
        qword_9FDC8();
      if ( a1 )
        v2 = gettext("Refusing to write archive contents to terminal (missing -f option?)");
      else
        v2 = gettext("Refusing to read archive contents from terminal (missing -f option?)");
      error(0, 0, v2);
      sub_177F4();
    }
  }
  return result;
}



// Function: _open_archive @ 0xbdb0
unsigned long long _open_archive(unsigned int a1)
{
  char *v1; // rax
  char *v2; // rax
  int v3; // eax
  char *v4; // rax
  long long v5; // rbx
  char *v6; // rax
  char *v7; // rax
  int v8; // eax
  int v9; // eax
  int v10; // eax
  char *v11; // rax
  bool v13; // [rsp+1Bh] [rbp-25h] BYREF
  int v14; // [rsp+1Ch] [rbp-24h]
  unsigned int v15; // [rsp+20h] [rbp-20h]
  int v16; // [rsp+24h] [rbp-1Ch]
  unsigned long long v17; // [rsp+28h] [rbp-18h]

  v17 = __readfsqword(0x28u);
  v14 = 0;
  if ( !qword_9F870 )
  {
    if ( qword_9FDC8 )
      qword_9FDC8();
    v1 = gettext("Invalid value for record_size");
    error(0, 0, v1);
    sub_177F4();
  }
  if ( !qword_9FBE0 )
  {
    if ( qword_9FDC8 )
      qword_9FDC8();
    v2 = gettext("No archive name given");
    error(0, 0, v2);
    sub_177F4();
  }
  sub_39D1C(&qword_9FA20);
  dword_9F030 = 0;
  sub_BC48();
  if ( a1 == 2 )
    v3 = 0;
  else
    v3 = a1;
  dword_9EFB8 = v3;
  sub_BD12(v3);
  byte_9F049 = byte_9F94D;
  qword_9EFC0 = 0;
  if ( qword_9F898 )
  {
    if ( a1 == 2 )
      abort();
    if ( a1 <= 2 )
    {
      if ( a1 )
      {
        dword_9F040 = sub_3321B();
      }
      else
      {
        dword_9F040 = sub_3399B();
        byte_9F049 = 1;
        s = s1;
      }
    }
    if ( !qword_9FBF0 && a1 == 1 && !strcmp(*(const char **)qword_9FBD8, "-") )
      stream = stderr;
  }
  else if ( !strcmp(*(const char **)qword_9FBD8, "-") )
  {
    byte_9F049 = 1;
    if ( byte_9F9B0 )
    {
      if ( qword_9FDC8 )
        qword_9FDC8();
      v4 = gettext("Cannot verify stdin/stdout archive");
      error(0, 0, v4);
      sub_177F4();
    }
    if ( a1 == 2 )
    {
      fd = 0;
      byte_9EFD8 = 1;
      s = s1;
      if ( !qword_9FBF0 )
        stream = stderr;
    }
    else if ( a1 <= 2 )
    {
      if ( a1 )
      {
        fd = 1;
        if ( !qword_9FBF0 )
          stream = stderr;
      }
      else
      {
        fd = 0;
        v15 = sub_B0CB(&v13);
        if ( v15 >= 2 )
        {
          if ( qword_9FDC8 )
            qword_9FDC8();
          v5 = sub_B065(v15);
          v6 = gettext("Archive is compressed. Use %s option");
          error(0, 0, v6, v5);
          sub_177F4();
        }
        if ( v13 )
        {
          if ( qword_9FDC8 )
            qword_9FDC8();
          v7 = gettext("This does not look like a tar archive");
          error(0, 0, v7);
          dword_9FDD0 = 2;
        }
      }
    }
  }
  else if ( a1 == 2 )
  {
    if ( byte_9FDF8 != 1
      && (qword_9FDF0 = (long long)strchr(*(const char **)qword_9FBD8, 58)) != 0
      && *(long long *)qword_9FBD8 < (unsigned long long)qword_9FDF0
      && !memchr(*(const void **)qword_9FBD8, 47, qword_9FDF0 - *(long long *)qword_9FBD8) )
    {
      v10 = sub_3F860(*(long long *)qword_9FBD8, 66, 0x40000000, qword_9F950);
    }
    else
    {
      v10 = open(*(const char **)qword_9FBD8, 66, 438);
    }
    fd = v10;
    if ( (unsigned int)sub_B0CB(0) > 1 )
    {
      if ( qword_9FDC8 )
        qword_9FDC8();
      v11 = gettext("Cannot update compressed archives");
      error(0, 0, v11);
      sub_177F4();
    }
  }
  else if ( a1 <= 2 )
  {
    if ( a1 )
    {
      if ( byte_9F884 )
      {
        sub_2A571(*(long long *)qword_9FBD8, 1);
        v14 = 1;
      }
      if ( byte_9F9B0 )
      {
        if ( byte_9FDF8 != 1
          && (qword_9FDF0 = (long long)strchr(*(const char **)qword_9FBD8, 58)) != 0
          && *(long long *)qword_9FBD8 < (unsigned long long)qword_9FDF0
          && !memchr(*(const void **)qword_9FBD8, 47, qword_9FDF0 - *(long long *)qword_9FBD8) )
        {
          v8 = sub_3F860(*(long long *)qword_9FBD8, 66, 0x40000000, qword_9F950);
        }
        else
        {
          v8 = open(*(const char **)qword_9FBD8, 66, 438);
        }
        fd = v8;
      }
      else
      {
        if ( byte_9FDF8 != 1
          && (qword_9FDF0 = (long long)strchr(*(const char **)qword_9FBD8, 58)) != 0
          && *(long long *)qword_9FBD8 < (unsigned long long)qword_9FDF0
          && !memchr(*(const void **)qword_9FBD8, 47, qword_9FDF0 - *(long long *)qword_9FBD8) )
        {
          v9 = sub_3F860(*(long long *)qword_9FBD8, 65, 0x40000000, qword_9F950);
        }
        else
        {
          v9 = creat(*(const char **)qword_9FBD8, 0x1B6u);
        }
        fd = v9;
      }
    }
    else
    {
      fd = sub_B2CF();
      if ( fd >= 0 )
        sub_B214();
    }
  }
  if ( fd < 0 || fd <= 0x3FFFFFFF && (unsigned char)sub_32B11() != 1 )
  {
    v16 = *__errno_location();
    if ( v14 )
      sub_2A874();
    *__errno_location() = v16;
    sub_3E3B4(*(long long *)qword_9FBD8);
  }
  sub_32BDC();
  sub_32B88();
  if ( a1 )
  {
    if ( a1 - 1 <= 1 )
      qword_9EFC8 = 0;
  }
  else
  {
    sub_BB4B();
  }
  return v17 - __readfsqword(0x28u);
}



// Function: _flush_write @ 0xc54f
unsigned long long _flush_write()
{
  unsigned long long v1; // [rsp+0h] [rbp-20h]
  long long v2; // [rsp+8h] [rbp-18h]
  unsigned long long v3; // [rsp+10h] [rbp-10h]

  sub_106D8(1);
  if ( *(double *)&qword_9F990 == 0.0 || *(double *)&qword_9F008 < *(double *)&qword_9F990 )
  {
    if ( byte_9F9D0 )
      v1 = qword_9F870;
    else
      v1 = sub_3301A();
  }
  else
  {
    *__errno_location() = 28;
    v1 = 0;
  }
  if ( v1 )
  {
    if ( byte_9F8FC )
    {
      if ( !dword_9F070 )
      {
        v2 = sub_ABD9(v1);
        if ( v2 )
        {
          v3 = v1 - (*(long long *)(v2 + 8) << 9);
          *(long long *)(v2 + 40) += v3 >> 9;
          if ( v3 > *(long long *)(v2 + 32) )
            v3 = *(long long *)(v2 + 32);
          *(long long *)(v2 + 32) -= v3;
          if ( *(long long *)(v2 + 32) )
            sub_ACB0((void **)v2, -*(long long *)(v2 + 8));
          else
            sub_ACB0(*(void ***)v2, -(*(long long *)(v2 + 8) + *(long long *)(v2 + 40)));
        }
      }
    }
  }
  return v1;
}



// Function: archive_write_error @ 0xc6d5
void archive_write_error(long long a1)
{
  int v1; // [rsp+1Ch] [rbp-4h]

  if ( byte_9F999 )
  {
    v1 = *__errno_location();
    sub_BAA2();
    *__errno_location() = v1;
  }
  sub_D132(*(long long *)qword_9FBE8, a1, qword_9F870);
}



// Function: archive_read_error @ 0xc729
long long archive_read_error()
{
  char *v0; // rax
  long long result; // rax
  char *v2; // rax

  sub_3E403(*(long long *)qword_9FBE8);
  if ( !qword_9F038 )
  {
    if ( qword_9FDC8 )
      qword_9FDC8();
    v0 = gettext("At beginning of tape, quitting now");
    error(0, 0, v0);
    sub_177F4();
  }
  result = (unsigned int)dword_9F044++;
  if ( (int)result > 10 )
  {
    if ( qword_9FDC8 )
      qword_9FDC8();
    v2 = gettext("Too many errors, quitting");
    error(0, 0, v2);
    sub_177F4();
  }
  return result;
}



// Function: archive_is_dev @ 0xc7e6
long long archive_is_dev()
{
  struct stat buf; // [rsp+0h] [rbp-A0h] BYREF
  unsigned long long v3; // [rsp+98h] [rbp-8h]

  v3 = __readfsqword(0x28u);
  if ( fstat(fd, &buf) )
  {
    sub_2B444(*(long long *)qword_9FBE8);
    return 0;
  }
  else
  {
    return (buf.st_mode & 0xF000) == 0x6000 || (buf.st_mode & 0xF000) == 0x2000;
  }
}



// Function: short_read @ 0xc881
long long short_read(unsigned long long a1)
{
  char *v1; // rax
  long long v2; // rax
  char *v3; // rax
  long long v5; // [rsp+8h] [rbp-28h]
  unsigned long long v6; // [rsp+10h] [rbp-20h]
  char *v7; // [rsp+18h] [rbp-18h]
  unsigned long long n; // [rsp+20h] [rbp-10h]
  unsigned long long v9; // [rsp+28h] [rbp-8h]

  v5 = a1;
  v7 = (char *)s1 + a1;
  v6 = qword_9F870 - a1;
  if ( qword_9F870 != a1 && (v6 & 0x1FF) == 0 && (dword_9EC84 & 0x400000) != 0 && !qword_9F038 && a1 && sub_C7E6() )
  {
    n = a1 >> 9;
    if ( qword_9FDC8 )
      qword_9FDC8();
    v1 = ngettext("Record size = %lu block", "Record size = %lu blocks", n);
    error(0, 0, v1, n);
  }
  while ( (v6 & 0x1FF) != 0 || v6 && v5 && byte_9F049 )
  {
    if ( v5 )
    {
      while ( 1 )
      {
        v2 = fd <= 0x3FFFFFFF ? sub_5EC70((unsigned int)fd, v7, v6) : sub_3FE62((unsigned int)(fd - 0x40000000), v7, v6);
        v5 = v2;
        if ( v2 != -1 )
          break;
        sub_C729();
      }
    }
    if ( !v5 )
      break;
    if ( byte_9F049 != 1 )
    {
      v9 = qword_9F870 - v6;
      if ( qword_9FDC8 )
        qword_9FDC8();
      v3 = ngettext("Unaligned block (%lu byte) in archive", "Unaligned block (%lu bytes) in archive", v9);
      error(0, 0, v3, v9);
      sub_177F4();
    }
    v6 -= v5;
    v7 += v5;
  }
  s = (char *)s1 + ((qword_9F870 - v6) & 0xFFFFFFFFFFFFFE00LL);
  return ++qword_9EFC0;
}



// Function: flush_archive @ 0xcac8
long long flush_archive()
{
  long long result; // rax
  signed long long v1; // [rsp+8h] [rbp-8h]

  if ( dword_9EFB8
    || !byte_9FD70
    || (dword_9EFB8 = 1, byte_9FD70 = 0, sub_CC3A(), ((char *)s - (char *)s1) >> 9 >= dword_9F868) )
  {
    v1 = (char *)qword_9EFB0 - (char *)s1;
    qword_9F038 += ((char *)s - (char *)s1) >> 9;
    qword_9EFB0 = s1;
    s = (char *)s1 + 512 * (long long)dword_9F868;
    result = (unsigned int)dword_9EFB8;
    if ( dword_9EFB8 == 2 )
      abort();
    if ( (unsigned int)dword_9EFB8 <= 2 )
    {
      if ( dword_9EFB8 )
      {
        if ( dword_9EFB8 == 1 )
          return qword_9F050(v1);
      }
      else
      {
        return sub_F662();
      }
    }
  }
  else
  {
    memset(s, 0, (dword_9F868 - (((char *)s - (char *)s1) >> 9)) << 9);
    result = (long long)s1 + 512 * (long long)dword_9F868;
    s = (void *)result;
  }
  return result;
}



// Function: backspace_output @ 0xcc3a
unsigned long long backspace_output()
{
  bool v0; // al
  bool v1; // al
  __off_t v2; // rax
  __off_t v3; // rax
  char *v4; // rax
  long long offset; // [rsp+8h] [rbp-18h]
  short v7[2]; // [rsp+10h] [rbp-10h] BYREF
  int v8; // [rsp+14h] [rbp-Ch]
  unsigned long long v9; // [rsp+18h] [rbp-8h]

  v9 = __readfsqword(0x28u);
  v7[0] = 4;
  v8 = 1;
  if ( fd <= 0x3FFFFFFF )
    v0 = ioctl(fd, 0x40086D01u, v7) >= 0;
  else
    v0 = (int)sub_40254((unsigned int)(fd - 0x40000000), 1074294017, v7) >= 0;
  if ( !v0 )
  {
    if ( *__errno_location() != 5
      || (fd <= 0x3FFFFFFF
        ? (v1 = ioctl(fd, 0x40086D01u, v7) >= 0)
        : (v1 = (int)sub_40254((unsigned int)(fd - 0x40000000), 1074294017, v7) >= 0),
          !v1) )
    {
      if ( fd <= 0x3FFFFFFF )
        v2 = lseek(fd, 0, 1);
      else
        v2 = sub_400BE((unsigned int)(fd - 0x40000000), 0, 1);
      offset = v2 - ((char *)s - (char *)s1);
      if ( offset < 0 )
        offset = 0;
      if ( fd <= 0x3FFFFFFF )
        v3 = lseek(fd, offset, 0);
      else
        v3 = sub_400BE((unsigned int)(fd - 0x40000000), offset, 0);
      if ( v3 != offset )
      {
        if ( qword_9FDC8 )
          qword_9FDC8();
        v4 = gettext("Cannot backspace archive file; it may be unreadable without -i");
        error(0, 0, v4);
        if ( s1 != (void *)qword_9FD78 )
          memset(s1, 0, qword_9FD78 - (long long)s1);
      }
    }
  }
  return v9 - __readfsqword(0x28u);
}



// Function: seek_archive @ 0xce79
__off_t seek_archive(signed long long a1)
{
  __off_t result; // rax
  char *v2; // rax
  signed long long v3; // [rsp+18h] [rbp-28h]
  signed long long v4; // [rsp+20h] [rbp-20h]
  unsigned long long v5; // [rsp+28h] [rbp-18h]
  __off_t v6; // [rsp+38h] [rbp-8h]

  v3 = sub_BACD();
  v4 = (dword_9F868 - (((char *)qword_9EFB0 - (char *)s1) >> 9)) << 9;
  if ( a1 <= v4 )
    return 0;
  v5 = (a1 - v4) / (unsigned long long)qword_9F870;
  if ( !v5 )
    return 0;
  if ( fd <= 0x3FFFFFFF )
    result = lseek(fd, v5 * qword_9F870, 1);
  else
    result = sub_400BE((unsigned int)(fd - 0x40000000), v5 * qword_9F870, 1);
  if ( result >= 0 )
  {
    if ( result % (unsigned long long)qword_9F870 )
    {
      if ( qword_9FDC8 )
        qword_9FDC8();
      v2 = gettext("rmtlseek not stopped at a record boundary");
      error(0, 0, v2);
      sub_177F4();
    }
    v6 = result / 512 - v3;
    qword_9EFC0 += v6 / dword_9F868;
    qword_9F038 = result / 512 - dword_9F868;
    qword_9EFB0 = s;
    return v6;
  }
  return result;
}



// Function: close_archive @ 0xd046
void **close_archive()
{
  double v0; // xmm0_8
  bool v1; // al

  if ( byte_9FD70 || dword_9EFB8 == 1 )
  {
    do
      sub_CAC8();
    while ( qword_9EFB0 > s1 );
  }
  v0 = sub_AE4D();
  if ( byte_9F9B0 )
    sub_11A1C(v0);
  if ( fd <= 0x3FFFFFFF )
    v1 = close(fd) != 0;
  else
    v1 = (unsigned int)sub_3FE0C((unsigned int)(fd - 0x40000000)) != 0;
  if ( v1 )
    sub_3E1FF(*(long long *)qword_9FBE8);
  sub_32CA8((unsigned int)dword_9F040, (unsigned char)byte_9F048, v0);
  sub_39D1C(&qword_9FA20);
  free(ptr);
  free(qword_9F018);
  return sub_AC2B(0);
}



// Function: write_fatal_details @ 0xd132
void write_fatal_details(long long a1, long long a2, long long a3, double a4)
{
  bool v4; // al

  sub_3EB4B(a1, a2, a3);
  if ( fd <= 0x3FFFFFFF )
    v4 = close(fd) != 0;
  else
    v4 = (unsigned int)sub_3FE0C((unsigned int)(fd - 0x40000000)) != 0;
  if ( v4 )
    sub_3E1FF(*(long long *)qword_9FBE8);
  sub_32CA8((unsigned int)dword_9F040, 0, a4);
  sub_177F4();
}



// Function: init_volume_number @ 0xd1c6
int init_volume_number()
{
  long long v0; // rbx
  char *v1; // rax
  int result; // eax
  FILE *stream; // [rsp+8h] [rbp-18h]

  stream = fopen(filename, "r");
  if ( stream )
  {
    if ( (unsigned int)__isoc99_fscanf(stream, "%d", &dword_9C014) != 1 || dword_9C014 < 0 )
    {
      if ( qword_9FDC8 )
        qword_9FDC8();
      v0 = sub_5E548(filename);
      v1 = gettext("%s: contains invalid volume number");
      error(0, 0, v1, v0);
      sub_177F4();
    }
    if ( ferror_unlocked(stream) )
      sub_3E403(filename);
    result = fclose(stream);
    if ( result )
      return sub_3E1FF(filename);
  }
  else
  {
    result = *__errno_location();
    if ( result != 2 )
      return sub_3E38B(filename);
  }
  return result;
}



// Function: closeout_volume_number @ 0xd2e3
int closeout_volume_number()
{
  int result; // eax
  FILE *stream; // [rsp+8h] [rbp-8h]

  stream = fopen(filename, "w");
  if ( !stream )
    return sub_3E38B(filename);
  fprintf(stream, "%d\n", dword_9C014);
  if ( ferror_unlocked(stream) )
    sub_3EB22(filename);
  result = fclose(stream);
  if ( result )
    return sub_3E1FF(filename);
  return result;
}



// Function: increase_volume_number @ 0xd386
long long increase_volume_number()
{
  char *v0; // rax

  if ( ++dword_9C014 < 0 )
  {
    if ( qword_9FDC8 )
      qword_9FDC8();
    v0 = gettext("Volume number overflow");
    error(0, 0, v0);
    sub_177F4();
  }
  return (unsigned int)++dword_9C010;
}



// Function: change_tape_menu @ 0xd3f9
unsigned long long change_tape_menu(FILE *a1)
{
  long long v1; // r12
  unsigned int v2; // ebx
  char *v3; // rax
  char *v4; // rax
  char *v5; // rax
  int v6; // eax
  char *v7; // rax
  char *v8; // rax
  char *v9; // rax
  char *v10; // rax
  char *v11; // rax
  long long *v12; // rbx
  char *v13; // rax
  char *v14; // rax
  char v16; // [rsp+17h] [rbp-39h]
  char *lineptr; // [rsp+18h] [rbp-38h] BYREF
  size_t n; // [rsp+20h] [rbp-30h] BYREF
  char *i; // [rsp+28h] [rbp-28h]
  char *j; // [rsp+30h] [rbp-20h]
  unsigned long long v21; // [rsp+38h] [rbp-18h]

  v21 = __readfsqword(0x28u);
  lineptr = 0;
  n = 0;
  v16 = 0;
  while ( v16 != 1 )
  {
    fputc_unlocked(7, stderr);
    v1 = sub_5E84E(*(long long *)qword_9FBE8);
    v2 = dword_9C014 + 1;
    v3 = gettext("Prepare volume #%d for %s and hit return: ");
    fprintf(stderr, v3, v2, v1);
    fflush_unlocked(stderr);
    if ( getline(&lineptr, &n, a1) <= 0 )
    {
      if ( qword_9FDC8 )
        qword_9FDC8();
      v4 = gettext("EOF where user reply was expected");
      error(0, 0, v4);
      if ( dword_9F860 != 6 && dword_9F860 != 7 && dword_9F860 != 5 )
      {
        if ( qword_9FDC8 )
          qword_9FDC8();
        v5 = gettext("WARNING: Archive is incomplete");
        error(0, 0, v5);
      }
      sub_177F4();
    }
    if ( *lineptr == 10 || *lineptr == 121 || *lineptr == 89 )
      break;
    v6 = *lineptr;
    if ( v6 == 113 )
    {
      if ( qword_9FDC8 )
        qword_9FDC8();
      v10 = gettext("No new volume; exiting.\n");
      error(0, 0, v10);
      if ( dword_9F860 != 6 && dword_9F860 != 7 && dword_9F860 != 5 )
      {
        if ( qword_9FDC8 )
          qword_9FDC8();
        v11 = gettext("WARNING: Archive is incomplete");
        error(0, 0, v11);
      }
      sub_177F4();
    }
    if ( v6 > 113 )
    {
LABEL_46:
      v14 = gettext("Invalid input. Type ? for help.\n");
      fprintf(stderr, v14);
    }
    else if ( v6 == 110 )
    {
      for ( i = lineptr + 1; *i == 32 || *i == 9; ++i )
        ;
      for ( j = i; *j && *j != 10; ++j )
        ;
      *j = 0;
      if ( *i )
      {
        v12 = (long long *)qword_9FBE8;
        *v12 = sub_60CC6(i);
        v16 = 1;
      }
      else
      {
        v13 = gettext("File name not specified. Try again.\n");
        fprintf(stderr, "%s", v13);
      }
    }
    else
    {
      if ( v6 > 110 )
        goto LABEL_46;
      if ( v6 == 33 )
      {
        if ( byte_9F9A9 == 1 )
          goto LABEL_46;
        sub_32DF0();
      }
      else
      {
        if ( v6 != 63 )
          goto LABEL_46;
        v7 = gettext(
               " n name        Give a new file name for the next (and subsequent) volume(s)\n"
               " q             Abort tar\n"
               " y or newline  Continue operation\n");
        fprintf(stderr, v7);
        if ( byte_9F9A9 != 1 )
        {
          v8 = gettext(" !             Spawn a subshell\n");
          fprintf(stderr, v8);
        }
        v9 = gettext(" ?             Print this list\n");
        fprintf(stderr, v9);
      }
    }
  }
  free(lineptr);
  return v21 - __readfsqword(0x28u);
}



// Function: new_volume @ 0xd818
long long new_volume(unsigned int a1, double a2)
{
  FILE *v2; // rax
  bool v4; // al
  long long v5; // rbx
  char *v6; // rax
  int v7; // eax
  int v8; // eax
  int v9; // eax
  int v10; // eax
  int i; // [rsp+1Ch] [rbp-14h]

  if ( !qword_9F248 && !qword_9F8C0 )
  {
    if ( fd )
      v2 = (FILE *)stdin;
    else
      v2 = fopen("/dev/tty", "r");
    qword_9F248 = (long long)v2;
  }
  if ( byte_9F320 )
    return 0;
  if ( byte_9F9B0 )
    sub_11A1C(a2);
  sub_29547(&qword_9EFE0, 0);
  sub_29547(&qword_9EFE8, 0);
  qword_9EFF8 = 0;
  qword_9EFF0 = 0;
  qword_9EFB0 = s1;
  if ( fd <= 0x3FFFFFFF )
    v4 = close(fd) != 0;
  else
    v4 = (unsigned int)sub_3FE0C((unsigned int)(fd - 0x40000000)) != 0;
  if ( v4 )
    sub_3E1FF(*(long long *)qword_9FBE8);
  qword_9FBE8 += 8;
  if ( (char *)qword_9FBD8 + 8 * qword_9FBE0 == (void *)qword_9FBE8 )
  {
    qword_9FBE8 = (long long)qword_9FBD8;
    dword_9F250 = 1;
  }
  for ( i = dword_9F250; ; i = 1 )
  {
    if ( i )
    {
      if ( qword_9F8C0 )
      {
        if ( filename )
          sub_D2E3();
        if ( (unsigned int)sub_345E7(qword_9FBE8, (unsigned int)(dword_9C014 + 1)) )
        {
          if ( qword_9FDC8 )
            qword_9FDC8();
          v5 = sub_5E84E(qword_9F8C0);
          v6 = gettext("%s command failed");
          error(0, 0, v6, v5);
          sub_177F4();
        }
      }
      else
      {
        sub_D3F9((FILE *)qword_9F248);
      }
    }
    if ( !strcmp(*(const char **)qword_9FBE8, "-") )
    {
      byte_9F049 = 1;
      fd = 0;
    }
    else if ( byte_9F9B0 )
    {
      if ( byte_9FDF8 != 1
        && (qword_9FDF0 = (long long)strchr(*(const char **)qword_9FBE8, 58)) != 0
        && *(long long *)qword_9FBE8 < (unsigned long long)qword_9FDF0
        && !memchr(*(const void **)qword_9FBE8, 47, qword_9FDF0 - *(long long *)qword_9FBE8) )
      {
        v7 = sub_3F860(*(long long *)qword_9FBE8, 66, 0x40000000, qword_9F950);
      }
      else
      {
        v7 = open(*(const char **)qword_9FBE8, 66, 438);
      }
      fd = v7;
    }
    else if ( a1 == 2 )
    {
      if ( byte_9FDF8 != 1
        && (qword_9FDF0 = (long long)strchr(*(const char **)qword_9FBE8, 58)) != 0
        && *(long long *)qword_9FBE8 < (unsigned long long)qword_9FDF0
        && !memchr(*(const void **)qword_9FBE8, 47, qword_9FDF0 - *(long long *)qword_9FBE8) )
      {
        v10 = sub_3F860(*(long long *)qword_9FBE8, 66, 0x40000000, qword_9F950);
      }
      else
      {
        v10 = open(*(const char **)qword_9FBE8, 66, 438);
      }
      fd = v10;
    }
    else if ( a1 <= 2 )
    {
      if ( a1 )
      {
        if ( byte_9F884 )
          sub_2A571(*(long long *)qword_9FBE8, 1);
        if ( byte_9FDF8 != 1
          && (qword_9FDF0 = (long long)strchr(*(const char **)qword_9FBE8, 58)) != 0
          && *(long long *)qword_9FBE8 < (unsigned long long)qword_9FDF0
          && !memchr(*(const void **)qword_9FBE8, 47, qword_9FDF0 - *(long long *)qword_9FBE8) )
        {
          v9 = sub_3F860(*(long long *)qword_9FBE8, 65, 0x40000000, qword_9F950);
        }
        else
        {
          v9 = creat(*(const char **)qword_9FBE8, 0x1B6u);
        }
        fd = v9;
      }
      else
      {
        if ( byte_9FDF8 != 1
          && (qword_9FDF0 = (long long)strchr(*(const char **)qword_9FBE8, 58)) != 0
          && *(long long *)qword_9FBE8 < (unsigned long long)qword_9FDF0
          && !memchr(*(const void **)qword_9FBE8, 47, qword_9FDF0 - *(long long *)qword_9FBE8) )
        {
          v8 = sub_3F860(*(long long *)qword_9FBE8, 0, 0x40000000, qword_9F950);
        }
        else
        {
          v8 = open(*(const char **)qword_9FBE8, 0, 438);
        }
        fd = v8;
        sub_B214();
      }
    }
    if ( fd >= 0 )
      break;
    sub_3E3DA(*(long long *)qword_9FBE8);
    if ( byte_9F9B0 != 1 && a1 == 1 && byte_9F884 )
      sub_2A874();
  }
  return 1;
}



// Function: read_header0 @ 0xde84
long long read_header0(long long a1)
{
  char *v2; // rax

  sub_39C55(a1);
  if ( (unsigned int)sub_25ECF(&qword_9F460, a1, 0) == 1 )
  {
    sub_BBA7(qword_9F460);
    return 1;
  }
  else
  {
    if ( qword_9FDC8 )
      qword_9FDC8();
    v2 = gettext("This does not look like a tar archive");
    error(0, 0, v2);
    dword_9FDD0 = 2;
    return 0;
  }
}



// Function: try_new_volume @ 0xdf26
long long try_new_volume(double a1)
{
  char v1; // al
  long long v3; // rax
  char *v4; // rax
  int v5; // eax
  char *v6; // rax
  int v7; // eax
  long long v8; // rbx
  char *v9; // rax
  long long v10; // rbx
  char *v11; // rax
  long long v12; // rbx
  long long v13; // r14
  long long v14; // r13
  long long v15; // r12
  char *v16; // rax
  long long v17; // r13
  long long v18; // r12
  long long v19; // rbx
  char *v20; // rax
  char *v21; // [rsp+8h] [rbp-98h] BYREF
  unsigned long long v22; // [rsp+10h] [rbp-90h]
  unsigned long long v23; // [rsp+18h] [rbp-88h]
  char v24[32]; // [rsp+20h] [rbp-80h] BYREF
  char v25[32]; // [rsp+40h] [rbp-60h] BYREF
  char v26[24]; // [rsp+60h] [rbp-40h] BYREF
  unsigned long long v27; // [rsp+78h] [rbp-28h]

  v27 = __readfsqword(0x28u);
  if ( (unsigned int)dword_9F860 <= 2 )
  {
    if ( !dword_9F860 )
      goto LABEL_6;
LABEL_5:
    v1 = sub_D818(2u, a1);
    goto LABEL_7;
  }
  if ( dword_9F860 == 8 )
    goto LABEL_5;
LABEL_6:
  v1 = sub_D818(0, a1);
LABEL_7:
  if ( v1 != 1 )
    return 1;
  while ( 1 )
  {
    v3 = fd <= 0x3FFFFFFF
       ? sub_5EC70((unsigned int)fd, s1, qword_9F870)
       : sub_3FE62((unsigned int)(fd - 0x40000000), s1, qword_9F870);
    v22 = v3;
    if ( v3 != -1 )
      break;
    sub_C729();
  }
  if ( v22 != qword_9F870 )
    sub_C881(v22);
  v21 = sub_BB4B();
  if ( !v21 )
  {
    if ( qword_9FDC8 )
      qword_9FDC8();
    v4 = gettext("This does not look like a tar archive");
    error(0, 0, v4);
    return 0;
  }
  v5 = (char)v21[156];
  if ( v5 != 103 )
  {
    if ( v5 <= 103 )
    {
      if ( v5 == 77 )
        goto LABEL_71;
      if ( v5 == 86 )
      {
        if ( (unsigned char)sub_DE84((long long)&unk_9F080) != 1 )
          return 0;
        sub_39D1C(&unk_9F080);
        sub_29591(&qword_9EFE0, qword_9F460, 100);
        sub_BBA7((unsigned long long)v21);
        v21 = sub_BB4B();
        if ( v21[156] == 77 )
        {
LABEL_71:
          if ( (unsigned char)sub_DE84((long long)&unk_9F080) != 1 )
            return 0;
          sub_39D1C(&unk_9F080);
          sub_29591(&qword_9EFE8, qword_9F460, 100);
          qword_9EFF0 = sub_27A73(qword_9F460 + 124, 12);
          qword_9EFF8 = sub_27A73(qword_9F460 + 369, 12);
        }
      }
    }
    goto LABEL_43;
  }
  sub_39C55(&unk_9F080);
  if ( (unsigned int)sub_25ECF(&v21, &unk_9F080, 2) != 2 )
  {
    if ( qword_9FDC8 )
      qword_9FDC8();
LABEL_28:
    v6 = gettext("This does not look like a tar archive");
    error(0, 0, v6);
    return 0;
  }
  sub_1DF5F(&unk_9F080);
  sub_39D1C(&unk_9F080);
  v7 = sub_25ECF(&v21, &unk_9F080, 0);
  if ( v7 == 1 )
  {
    sub_BBA7((unsigned long long)v21);
  }
  else if ( v7 != 5 )
  {
    if ( qword_9FDC8 )
      qword_9FDC8();
    goto LABEL_28;
  }
LABEL_43:
  if ( !qword_9F060 )
    goto LABEL_68;
  if ( !qword_9EFE8 )
  {
    if ( qword_9FDC8 )
      qword_9FDC8();
LABEL_47:
    v8 = sub_5E84E(*(long long *)(qword_9F060 + 16));
    v9 = gettext("%s is not continued on this volume");
    error(0, 0, v9, v8);
    return 0;
  }
  if ( strcmp(qword_9EFE8, *(const char **)(qword_9F060 + 16)) )
  {
    if ( dword_9F864 != 6 && dword_9F864 != 2
      || strlen(*(const char **)(qword_9F060 + 16)) <= 0x63
      || strncmp(qword_9EFE8, *(const char **)(qword_9F060 + 16), 0x64u) )
    {
      if ( qword_9FDC8 )
        qword_9FDC8();
      goto LABEL_47;
    }
    if ( qword_9FDC8 )
      qword_9FDC8();
    v10 = sub_5E84E(*(long long *)(qword_9F060 + 16));
    v11 = gettext("%s is possibly continued on this volume: header contains truncated name");
    error(0, 0, v11, v10);
  }
  v23 = qword_9EFF0 + qword_9EFF8;
  if ( *(long long *)(qword_9F060 + 24) != qword_9EFF0 + qword_9EFF8 || v23 < qword_9EFF8 )
  {
    if ( qword_9FDC8 )
      qword_9FDC8();
    v12 = sub_54E5A(qword_9EFF8, v26);
    v13 = sub_54E5A(qword_9EFF0, v25);
    v14 = sub_54E5A(*(long long *)(qword_9F060 + 24), v24);
    v15 = sub_5E84E(qword_9EFE8);
    v16 = gettext("%s is the wrong size (%s != %s + %s)");
    error(0, 0, v16, v15, v14, v13, v12);
    return 0;
  }
  if ( *(long long *)(qword_9F060 + 24) - *(long long *)(qword_9F060 + 32) == qword_9EFF8 )
  {
LABEL_68:
    sub_D386();
    return 1;
  }
  if ( qword_9FDC8 )
    qword_9FDC8();
  v17 = sub_54E5A(qword_9EFF8, v26);
  v18 = sub_54E5A(*(long long *)(qword_9F060 + 32), v25);
  v19 = sub_54E5A(*(long long *)(qword_9F060 + 24), v24);
  v20 = gettext("This volume is out of sequence (%s - %s != %s)");
  error(0, 0, v20, v19, v18, v17);
  return 0;
}



// Function: drop_volume_label_suffix @ 0xe680
void *drop_volume_label_suffix(const char *a1)
{
  const char *s1; // [rsp+18h] [rbp-18h]
  unsigned char *s1a; // [rsp+18h] [rbp-18h]
  size_t n; // [rsp+20h] [rbp-10h]
  void *dest; // [rsp+28h] [rbp-8h]

  n = strlen(a1);
  if ( !n )
    return 0;
  for ( s1 = &a1[n - 1]; s1 > a1 && ((*__ctype_b_loc())[*(unsigned char *)s1] & 0x800) != 0; --s1 )
    ;
  if ( s1 <= a1 )
    return 0;
  if ( a1 >= s1 - 7 )
    return 0;
  s1a = (unsigned char *)(s1 - 7);
  if ( memcmp(s1a, " Volume ", 8u) )
    return 0;
  dest = (void *)sub_60B5B(s1a - (unsigned char *)a1 + 1);
  memcpy(dest, a1, s1a - (unsigned char *)a1);
  *((char *)dest + s1a - (unsigned char *)a1) = 0;
  return dest;
}



// Function: check_label_pattern @ 0xe78a
long long check_label_pattern(const char *a1)
{
  bool v2; // [rsp+17h] [rbp-9h]
  char *pattern; // [rsp+18h] [rbp-8h]

  v2 = 0;
  if ( !fnmatch(name, a1, 0) )
    return 1;
  if ( byte_9F8FC != 1 )
    return 0;
  pattern = (char *)sub_E680(a1);
  if ( pattern )
  {
    v2 = fnmatch(pattern, name, 0) == 0;
    free(pattern);
  }
  return v2;
}



// Function: match_volume_label @ 0xe825
unsigned long long match_volume_label()
{
  long long v0; // rbx
  char *v1; // rax
  long long v2; // rax
  long long v3; // rbx
  char *v4; // rax
  long long v5; // r12
  long long v6; // rbx
  char *v7; // rax
  char *v9; // [rsp+8h] [rbp-1D8h]
  char v10[360]; // [rsp+10h] [rbp-1D0h] BYREF
  long long v11; // [rsp+178h] [rbp-68h] BYREF
  unsigned long long v12; // [rsp+1C8h] [rbp-18h]

  v12 = __readfsqword(0x28u);
  if ( !qword_9EFE0 )
  {
    v9 = sub_BB4B();
    if ( !v9 )
    {
      if ( qword_9FDC8 )
        qword_9FDC8();
      v0 = sub_5E84E(name);
      v1 = gettext("Archive not labeled to match %s");
      error(0, 0, v1, v0);
      sub_177F4();
    }
    if ( v9[156] == 86 )
    {
      sub_29591(&qword_9EFE0, v9, 100);
    }
    else if ( v9[156] == 103 )
    {
      sub_39C55(v10);
      v2 = sub_2798B(v9 + 124, 12);
      sub_1E28E(&v11, v9, v2);
      sub_1DF5F(v10);
      sub_39D1C(v10);
    }
  }
  if ( !qword_9EFE0 )
  {
    if ( qword_9FDC8 )
      qword_9FDC8();
    v3 = sub_5E84E(name);
    v4 = gettext("Archive not labeled to match %s");
    error(0, 0, v4, v3);
    sub_177F4();
  }
  if ( (unsigned char)sub_E78A((const char *)qword_9EFE0) != 1 )
  {
    if ( qword_9FDC8 )
      qword_9FDC8();
    v5 = sub_5E821(1, name);
    v6 = sub_5E821(0, qword_9EFE0);
    v7 = gettext("Volume %s does not match %s");
    error(0, 0, v7, v6, v5);
    sub_177F4();
  }
  return v12 - __readfsqword(0x28u);
}



// Function: _write_volume_label @ 0xea79
void *_write_volume_label(const char *a1)
{
  char *s; // [rsp+18h] [rbp-8h]

  if ( dword_9F864 == 4 )
    return (void *)sub_1E1D3("GNU.volume.label", &unk_9F080, a1);
  s = sub_BB4B();
  memset(s, 0, 0x200u);
  strcpy(s, a1);
  sub_29547(&file, s);
  byte_9FA30 = sub_4E2E5(file);
  s[156] = 86;
  sub_12AF2(qword_9F9E0, s + 136, 12);
  sub_1422E(&qword_9FA20, s, -1);
  return sub_BBA7((unsigned long long)s);
}



// Function: add_volume_label @ 0xeb6f
unsigned long long add_volume_label()
{
  size_t v0; // rbx
  size_t v1; // rax
  char *s; // [rsp+0h] [rbp-40h]
  char *v4; // [rsp+8h] [rbp-38h]
  char v5[24]; // [rsp+10h] [rbp-30h] BYREF
  unsigned long long v6; // [rsp+28h] [rbp-18h]

  v6 = __readfsqword(0x28u);
  s = (char *)sub_54E5A(dword_9C010, v5);
  v0 = strlen(name);
  v1 = strlen(s);
  v4 = (char *)sub_60B5B(v0 + v1 + 9);
  sprintf(v4, "%s %s %s", name, "Volume", s);
  sub_EA79(v4);
  free(v4);
  return v6 - __readfsqword(0x28u);
}



// Function: add_chunk_header @ 0xec37
unsigned long long add_chunk_header(long long a1)
{
  long long v1; // rax
  long long v3; // [rsp+18h] [rbp-1C8h]
  void *s; // [rsp+20h] [rbp-1C0h] BYREF
  void *v5; // [rsp+28h] [rbp-1B8h]
  int v6; // [rsp+90h] [rbp-150h]
  __uid_t v7; // [rsp+94h] [rbp-14Ch]
  __gid_t v8; // [rsp+98h] [rbp-148h]
  long long v9; // [rsp+A8h] [rbp-138h]
  long long v10; // [rsp+138h] [rbp-A8h]
  unsigned long long v11; // [rsp+1D8h] [rbp-8h]

  v11 = __readfsqword(0x28u);
  if ( dword_9F864 == 4 )
  {
    memset(&s, 0, 0x1B8u);
    v5 = *(void **)(a1 + 16);
    s = v5;
    v6 = 33188;
    v7 = getuid();
    v8 = getgid();
    s = (void *)sub_1CD02(&s, "%d/GNUFileParts/%f.%n", dword_9C010);
    v5 = s;
    v9 = *(long long *)(a1 + 32);
    v10 = v9;
    v3 = sub_1381D(&s);
    if ( !v3 )
      abort();
    v1 = sub_135E3(0, &s, v3);
    sub_1418C(v1);
    free(s);
  }
  return v11 - __readfsqword(0x28u);
}



// Function: write_volume_label @ 0xed89
void *write_volume_label()
{
  if ( byte_9F8FC )
    return (void *)sub_EB6F();
  else
    return sub_EA79(name);
}



// Function: gnu_add_multi_volume_header @ 0xedb5
void *gnu_add_multi_volume_header(long long a1)
{
  long long v1; // rbx
  char *v2; // rax
  int v4; // [rsp+1Ch] [rbp-24h]
  size_t n; // [rsp+20h] [rbp-20h]
  char *s; // [rsp+28h] [rbp-18h]

  s = sub_BB4B();
  n = strlen(*(const char **)(a1 + 16));
  if ( n > 0x64 )
  {
    if ( qword_9FDC8 )
      qword_9FDC8();
    v1 = sub_5E548(*(long long *)(a1 + 16));
    v2 = gettext("%s: file name too long to be stored in a GNU multivolume header, truncated");
    error(0, 0, v2, v1);
    n = 100;
  }
  memset(s, 0, 0x200u);
  memcpy(s, *(const void **)(a1 + 16), n);
  s[156] = 77;
  sub_12A9A(*(long long *)(a1 + 32), s + 124, 12);
  sub_12A9A(*(long long *)(a1 + 24) - *(long long *)(a1 + 32), s + 369, 12);
  v4 = dword_9F9AC;
  dword_9F9AC = 0;
  sub_1422E(&qword_9FA20, s, -1);
  dword_9F9AC = v4;
  return sub_BBA7((unsigned long long)s);
}



// Function: add_multi_volume_header @ 0xef1c
unsigned long long add_multi_volume_header(long long *a1)
{
  long long v2; // [rsp+10h] [rbp-10h] BYREF
  unsigned long long v3; // [rsp+18h] [rbp-8h]

  v3 = __readfsqword(0x28u);
  if ( dword_9F864 == 4 )
  {
    v2 = a1[3] - a1[4];
    sub_1E1D3("GNU.volume.filename", &unk_9F080, a1[2]);
    sub_1E1D3("GNU.volume.size", &unk_9F080, a1 + 4);
    sub_1E1D3("GNU.volume.offset", &unk_9F080, &v2);
  }
  else
  {
    sub_EDB5((long long)a1);
  }
  return v3 - __readfsqword(0x28u);
}



// Function: simple_flush_read @ 0xeff1
long long simple_flush_read()
{
  unsigned long long v0; // rax
  long long v2; // [rsp+8h] [rbp-8h]

  sub_106D8(0);
  dword_9F044 = 0;
  if ( byte_9EFD8 )
  {
    if ( qword_9F038 )
    {
      fd = 1;
      v2 = sub_3301A();
      fd = 0;
      if ( v2 != qword_9F870 )
        sub_C6D5(v2);
    }
  }
  while ( 1 )
  {
    if ( fd <= 0x3FFFFFFF )
      v0 = sub_5EC70((unsigned int)fd, s1, qword_9F870);
    else
      v0 = sub_3FE62((unsigned int)(fd - 0x40000000), s1, qword_9F870);
    if ( v0 == qword_9F870 )
      return ++qword_9EFC0;
    if ( v0 != -1 )
      break;
    sub_C729();
  }
  return sub_C881(v0);
}



// Function: simple_flush_write @ 0xf0fb
long long simple_flush_write()
{
  long long result; // rax
  unsigned long long v1; // [rsp+18h] [rbp-8h]

  v1 = sub_C54F();
  if ( v1 != qword_9F870 )
    sub_C6D5(v1);
  result = ++qword_9EFC8;
  *(double *)&qword_9F008 = *(double *)&qword_9F008 + (double)(int)v1;
  return result;
}



// Function: _gnu_flush_read @ 0xf160
void *_gnu_flush_read(double a1)
{
  long long v1; // rax
  void *result; // rax
  long long v3; // [rsp+8h] [rbp-8h]
  unsigned long long v4; // [rsp+8h] [rbp-8h]

  sub_106D8(0);
  dword_9F044 = 0;
  if ( byte_9EFD8 )
  {
    if ( qword_9F038 )
    {
      fd = 1;
      v3 = sub_3301A();
      fd = 0;
      if ( v3 != qword_9F870 )
        sub_C6D5(v3);
    }
  }
  while ( 1 )
  {
    if ( fd <= 0x3FFFFFFF )
      v1 = sub_5EC70((unsigned int)fd, s1, qword_9F870);
    else
      v1 = sub_3FE62((unsigned int)(fd - 0x40000000), s1, qword_9F870);
    v4 = v1;
    if ( v1 == qword_9F870 )
      return (void *)++qword_9EFC0;
    if ( (!v1 || v1 == -1 && *__errno_location() == 28) && byte_9F8FC )
      break;
    if ( v4 != -1 )
      return (void *)sub_C881(v4);
    sub_C729();
  }
  while ( (unsigned char)sub_DF26(a1) != 1 )
    ;
  result = s;
  if ( qword_9EFB0 == s )
    return (void *)sub_CAC8();
  return result;
}



// Function: gnu_flush_read @ 0xf2b9
long long (*gnu_flush_read(double a1))()
{
  qword_9F058 = (long long)sub_EFF1;
  sub_F160(a1);
  qword_9F058 = (long long)gnu_flush_read;
  return gnu_flush_read;
}



// Function: _gnu_flush_write @ 0xf2e5
long long _gnu_flush_write(long long a1)
{
  double v1; // xmm0_8
  long long result; // rax
  char *v3; // rax
  void *v4; // rax
  char *dest; // [rsp+10h] [rbp-30h]
  char *src; // [rsp+18h] [rbp-28h]
  size_t v7; // [rsp+20h] [rbp-20h]
  size_t n; // [rsp+28h] [rbp-18h]
  long long v9; // [rsp+30h] [rbp-10h]
  void **v10; // [rsp+38h] [rbp-8h]

  v9 = sub_C54F();
  if ( v9 != qword_9F870 && byte_9F8FC != 1 )
    sub_C6D5(v9);
  if ( v9 )
    ++qword_9EFC8;
  v1 = *(double *)&qword_9F008 + (double)(int)v9;
  *(double *)&qword_9F008 = v1;
  result = qword_9F870;
  if ( v9 != qword_9F870 )
  {
    v10 = (void **)sub_ABD9(v9);
    if ( (v9 & 0x1FF) != 0 )
    {
      if ( qword_9FDC8 )
        qword_9FDC8();
      v3 = gettext("write did not end on a block boundary");
      error(0, 0, v3);
      dword_9FDD0 = 2;
      sub_C6D5(v9);
    }
    if ( v9 < 0 && *__errno_location() != 28 && *__errno_location() != 5 && *__errno_location() != 6 )
      sub_C6D5(v9);
    result = (unsigned int)sub_D818(1u, v1) ^ 1;
    if ( !(char)result )
    {
      sub_39D1C(&unk_9F080);
      sub_D386();
      *(double *)&qword_9F000 = *(double *)&qword_9F008 + *(double *)&qword_9F000;
      qword_9F008 = 0;
      src = (char *)s1 + v9;
      v7 = a1 - v9;
      dword_9F030 = dword_9F030 == 0;
      sub_BC48();
      dword_9F070 = 1;
      if ( name )
        sub_EB6F();
      if ( v10 )
        sub_EF1C(v10);
      v4 = sub_BB4B();
      sub_135E3(1, &unk_9F080, v4);
      sub_39D1C(&unk_9F080);
      if ( v10 )
        sub_EC37((long long)v10);
      dest = sub_BB4B();
      sub_ACB0(v10, (dest - (char *)s1) >> 9);
      n = (size_t)sub_BBF5((long long)dest);
      dword_9F070 = 0;
      while ( n < v7 )
      {
        memcpy(dest, src, n);
        src += n;
        v7 -= n;
        sub_BBA7((unsigned long long)&dest[(n - 1) & 0xFFFFFFFFFFFFFE00LL]);
        dest = sub_BB4B();
        n = (size_t)sub_BBF5((long long)dest);
      }
      memcpy(dest, src, v7);
      memset(&dest[v7], 0, n - v7);
      sub_BBA7((unsigned long long)&dest[(v7 - 1) & 0xFFFFFFFFFFFFFE00LL]);
      return (long long)sub_BB4B();
    }
  }
  return result;
}



// Function: gnu_flush_write @ 0xf627
long long (*gnu_flush_write(long long a1))()
{
  qword_9F050 = (long long (*)(long long))sub_F0FB;
  sub_F2E5(a1);
  qword_9F050 = (long long (*)(long long))gnu_flush_write;
  return gnu_flush_write;
}



// Function: flush_read @ 0xf662
long long flush_read()
{
  return qword_9F058();
}



// Function: flush_write @ 0xf676
long long flush_write()
{
  return qword_9F050(qword_9F870);
}



// Function: open_archive @ 0xf694
long long open_archive(unsigned int a1)
{
  qword_9F058 = (long long (*)(void))sub_F2B9;
  qword_9F050 = (long long (*)(long long))sub_F627;
  sub_BDB0(a1);
  if ( a1 == 2 )
    goto LABEL_4;
  if ( a1 > 2 )
    return sub_AE17();
  if ( a1 )
  {
    qword_9EFC8 = 0;
    if ( name )
      sub_ED89();
  }
  else
  {
LABEL_4:
    if ( name )
      sub_E825();
  }
  return sub_AE17();
}



// Function: alloc_action @ 0xf720
long long alloc_action(int a1)
{
  long long v2; // [rsp+18h] [rbp-8h]

  v2 = sub_60C06(24);
  if ( qword_9F270 )
    *(long long *)qword_9F270 = v2;
  else
    qword_9F268 = v2;
  qword_9F270 = v2;
  *(int *)(v2 + 8) = a1;
  return v2;
}



// Function: copy_string_unquote @ 0xf77f
char *copy_string_unquote(long long a1)
{
  char *s; // [rsp+10h] [rbp-10h]
  size_t v3; // [rsp+18h] [rbp-8h]

  s = (char *)sub_60CC6(a1);
  v3 = strlen(s);
  if ( (*s == 34 || *s == 39) && s[v3 - 1] == *s )
  {
    memmove(s, s + 1, v3 - 2);
    s[v3 - 2] = 0;
  }
  sub_2961D(s);
  return s;
}



// Function: checkpoint_compile_action @ 0xf825
unsigned long long checkpoint_compile_action(const char *a1)
{
  char *v1; // rax
  char *v2; // rax
  char *endptr; // [rsp+10h] [rbp-20h] BYREF
  long long v5; // [rsp+18h] [rbp-18h]
  unsigned long long v6; // [rsp+20h] [rbp-10h]
  unsigned long long v7; // [rsp+28h] [rbp-8h]

  v7 = __readfsqword(0x28u);
  if ( !dword_9F278 )
  {
    sigemptyset(&set);
    dword_9F278 = 1;
  }
  if ( !strcmp(a1, ".") || !strcmp(a1, "dot") )
  {
    sub_F720(0);
  }
  else if ( !strcmp(a1, "bell") )
  {
    sub_F720(1);
  }
  else if ( !strcmp(a1, "echo") )
  {
    sub_F720(2);
  }
  else if ( !strncmp(a1, "echo=", 5u) )
  {
    v5 = sub_F720(2);
    *(long long *)(v5 + 16) = sub_F77F((long long)(a1 + 5));
  }
  else if ( !strncmp(a1, "exec=", 5u) )
  {
    v5 = sub_F720(5);
    *(long long *)(v5 + 16) = sub_F77F((long long)(a1 + 5));
  }
  else if ( !strncmp(a1, "ttyout=", 7u) )
  {
    v5 = sub_F720(3);
    *(long long *)(v5 + 16) = sub_F77F((long long)(a1 + 7));
  }
  else if ( !strncmp(a1, "sleep=", 6u) )
  {
    v6 = strtoul(a1 + 6, &endptr, 10);
    if ( *endptr )
    {
      if ( qword_9FDC8 )
        qword_9FDC8();
      v1 = gettext("%s: not a valid timeout");
      error(0, 0, v1, a1);
      sub_177F4();
    }
    v5 = sub_F720(4);
    *(long long *)(v5 + 16) = v6;
  }
  else if ( !strcmp(a1, "totals") )
  {
    sub_F720(6);
  }
  else
  {
    if ( strncmp(a1, "wait=", 5u) )
    {
      if ( qword_9FDC8 )
        qword_9FDC8();
      v2 = gettext("%s: unknown checkpoint action");
      error(0, 0, v2, a1);
      sub_177F4();
    }
    v5 = sub_F720(7);
    *(int *)(v5 + 16) = sub_35562(a1 + 5);
    sigaddset(&set, *(int *)(v5 + 16));
  }
  return v7 - __readfsqword(0x28u);
}



// Function: checkpoint_finish_compile @ 0xfb72
long long checkpoint_finish_compile()
{
  long long result; // rax

  if ( !dword_9F278 && dword_9F890 && !qword_9F268 )
    sub_F825("echo");
  result = (unsigned int)dword_9F278;
  if ( dword_9F278 == 1 )
  {
    sigprocmask(0, &set, 0);
    result = (unsigned int)dword_9F890;
    if ( !dword_9F890 )
      dword_9F890 = 10;
    dword_9F278 = 2;
  }
  return result;
}



// Function: getwidth @ 0xfbee
long long getwidth(FILE *a1)
{
  int v1; // eax
  const char *nptr; // [rsp+10h] [rbp-20h]
  long long v4; // [rsp+18h] [rbp-18h]
  char v5[2]; // [rsp+20h] [rbp-10h] BYREF
  unsigned short v6; // [rsp+22h] [rbp-Eh]
  unsigned long long v7; // [rsp+28h] [rbp-8h]

  v7 = __readfsqword(0x28u);
  v1 = fileno(a1);
  if ( !ioctl(v1, 0x5413u, v5) && v6 )
    return v6;
  nptr = getenv("COLUMNS");
  if ( nptr && (v4 = strtol(nptr, 0, 10), v4 > 0) )
    return v4;
  else
    return 80;
}



// Function: getarg @ 0xfca5
void *getarg(char *a1, long long *a2, void **a3, size_t *a4)
{
  char *v7; // [rsp+20h] [rbp-10h]
  size_t n; // [rsp+28h] [rbp-8h]
  size_t na; // [rsp+28h] [rbp-8h]

  if ( *a1 == 123 && (v7 = strchr(a1 + 1, 125)) != 0 )
  {
    n = v7 - a1;
    if ( v7 - a1 > *a4 )
    {
      *a4 = n;
      *a3 = (void *)sub_60B8D(*a3, *a4);
    }
    na = n - 1;
    memcpy(*a3, a1 + 1, na);
    *((char *)*a3 + na) = 0;
    *a2 = v7 + 1;
    return *a3;
  }
  else
  {
    *a2 = a1;
    return 0;
  }
}



// Function: format_checkpoint_string @ 0xfd96
unsigned long long format_checkpoint_string(FILE *a1, unsigned long long a2, char *a3, unsigned char a4, unsigned int a5)
{
  char *v5; // rax
  int v6; // eax
  double v7; // xmm0_8
  double v8; // xmm0_8
  long long v9; // rbx
  char *v10; // rax
  char *v11; // rax
  long long v12; // rax
  int j; // [rsp+2Ch] [rbp-194h]
  char *i; // [rsp+30h] [rbp-190h] BYREF
  char *nptr; // [rsp+38h] [rbp-188h]
  const char **v21; // [rsp+40h] [rbp-180h]
  char *v22; // [rsp+48h] [rbp-178h]
  char *s; // [rsp+50h] [rbp-170h]
  char *v24; // [rsp+58h] [rbp-168h]
  struct tm *v25; // [rsp+60h] [rbp-160h]
  unsigned long long v26; // [rsp+68h] [rbp-158h]
  struct timeval tv; // [rsp+70h] [rbp-150h] BYREF
  long long v28[4]; // [rsp+80h] [rbp-140h] BYREF
  long long v29[30]; // [rsp+A0h] [rbp-120h] BYREF
  char v30[24]; // [rsp+190h] [rbp-30h] BYREF
  unsigned long long v31; // [rsp+1A8h] [rbp-18h]

  v31 = __readfsqword(0x28u);
  if ( a4 )
    v5 = gettext("write");
  else
    v5 = gettext("read");
  v22 = v5;
  s = (char *)sub_54E5A(a5, v30);
  nptr = 0;
  if ( !a3 )
  {
    if ( a4 )
      a3 = gettext("Write checkpoint %u");
    else
      a3 = gettext("Read checkpoint %u");
  }
  for ( i = a3; *i; ++i )
  {
    if ( *i == 37 )
    {
      if ( *++i != 123 || (nptr = (char *)sub_FCA5(i, &i, &qword_9F310, &qword_9F318)) != 0 )
      {
        v6 = *i;
        if ( v6 == 42 )
        {
          if ( nptr )
            v12 = strtol(nptr, 0, 10);
          else
            v12 = sub_FBEE(a1);
          v26 = v12;
          while ( a2 < v26 )
          {
            fputc_unlocked(32, a1);
            ++a2;
          }
        }
        else if ( v6 < 42 || v6 > 117 || v6 < 84 )
        {
LABEL_47:
          fputc_unlocked(37, a1);
          fputc_unlocked(*i, a1);
          a2 += 2LL;
        }
        else
        {
          switch ( *i )
          {
            case 'T':
              v21 = (const char **)off_9C140;
              v8 = sub_AE4D();
              if ( nptr )
              {
                v29[7] = ",";
                if ( (unsigned int)sub_461A6(nptr, v29, 17988, v8) )
                {
                  if ( qword_9FDC8 )
                    qword_9FDC8();
                  v9 = sub_4647F(v29);
                  v10 = gettext("cannot split string '%s': %s");
                  error(0, 0, v10, nptr, v9);
                  dword_9FDD0 = 2;
                }
                else
                {
                  for ( j = 0; (unsigned long long)j < v29[0]; ++j )
                    v28[j] = *(long long *)(v29[1] + 8LL * j);
                  while ( j <= 2 )
                    v28[j++] = 0;
                  v21 = (const char **)v28;
                }
              }
              a2 += (int)sub_B768(a1, v21, 44, 0);
              if ( nptr )
                sub_46399(v29, v8);
              break;
            case 'c':
              a2 += (int)format_checkpoint_string(a1, a2, off_9C158[0], a4, a5);
              break;
            case 'd':
              v7 = sub_AE4D();
              a2 += fprintf(a1, "%.0f", v7);
              break;
            case 's':
              fputs_unlocked(v22, a1);
              a2 += strlen(v22);
              break;
            case 't':
              if ( nptr )
                v11 = nptr;
              else
                v11 = "%c";
              v24 = v11;
              gettimeofday(&tv, 0);
              v25 = localtime(&tv.tv_sec);
              a2 += sub_4F679(a1, v24, v25, 0, (unsigned int)(1000 * LODWORD(tv.tv_usec)));
              break;
            case 'u':
              fputs_unlocked(s, a1);
              a2 += strlen(s);
              break;
            default:
              goto LABEL_47;
          }
        }
        nptr = 0;
      }
      else
      {
        fputc_unlocked(37, a1);
        fputc_unlocked(*i, a1);
        a2 += 2LL;
      }
    }
    else
    {
      fputc_unlocked(*i, a1);
      if ( *i == 13 )
      {
        a2 = 0;
        dword_9F300 = 1;
      }
      else
      {
        ++a2;
      }
    }
  }
  fflush_unlocked(a1);
  return a2;
}



// Function: run_checkpoint_actions @ 0x103d4
unsigned long long run_checkpoint_actions(unsigned char a1)
{
  double v1; // xmm0_8
  int sig; // [rsp+18h] [rbp-18h] BYREF
  int v4; // [rsp+1Ch] [rbp-14h]
  long long i; // [rsp+20h] [rbp-10h]
  unsigned long long v6; // [rsp+28h] [rbp-8h]

  v6 = __readfsqword(0x28u);
  for ( i = qword_9F268; i; i = *(long long *)i )
  {
    switch ( *(int *)(i + 8) )
    {
      case 0:
        fputc_unlocked(46, stream);
        fflush_unlocked(stream);
        break;
      case 1:
        if ( !qword_9F308 )
          qword_9F308 = fopen("/dev/tty", "w");
        if ( qword_9F308 )
        {
          fputc_unlocked(7, qword_9F308);
          fflush_unlocked(qword_9F308);
        }
        break;
      case 2:
        v4 = fprintf(stderr, "%s: ", (const char *)qword_9FE48);
        sub_FD96(stderr, v4, *(char **)(i + 16), a1, dword_9F260);
        fputc_unlocked(10, stderr);
        break;
      case 3:
        if ( !qword_9F308 )
          qword_9F308 = fopen("/dev/tty", "w");
        if ( qword_9F308 )
          sub_FD96(qword_9F308, 0, *(char **)(i + 16), a1, dword_9F260);
        break;
      case 4:
        sleep(*(long long *)(i + 16));
        break;
      case 5:
        sub_348D7(*(long long *)(i + 16), *(long long *)qword_9FBE8, (unsigned int)dword_9F260);
        break;
      case 6:
        v1 = sub_AE4D();
        sub_BAA2(v1);
        break;
      case 7:
        sigwait(&set, &sig);
        break;
      default:
        continue;
    }
  }
  return v6 - __readfsqword(0x28u);
}



// Function: checkpoint_flush_actions @ 0x10622
long long checkpoint_flush_actions()
{
  long long result; // rax
  long long i; // [rsp+0h] [rbp-10h]
  long long v3; // [rsp+8h] [rbp-8h]

  result = qword_9F268;
  for ( i = qword_9F268; i; i = *(long long *)i )
  {
    if ( *(int *)(i + 8) == 3 && qword_9F308 && dword_9F300 )
    {
      v3 = sub_FBEE(qword_9F308);
      while ( v3-- )
        fputc_unlocked(32, qword_9F308);
      fputc_unlocked(13, qword_9F308);
      fflush_unlocked(qword_9F308);
    }
    result = *(long long *)i;
  }
  return result;
}



// Function: checkpoint_run @ 0x106d8
unsigned long long checkpoint_run(unsigned char a1)
{
  unsigned long long result; // rax

  result = (unsigned int)dword_9F890;
  if ( dword_9F890 )
  {
    result = ++dword_9F260 % (unsigned int)dword_9F890;
    if ( !(int)result )
      return sub_103D4(a1);
  }
  return result;
}



// Function: checkpoint_finish @ 0x10729
int checkpoint_finish()
{
  int result; // eax

  result = dword_9F890;
  if ( dword_9F890 )
  {
    sub_10622();
    result = (int)qword_9F308;
    if ( qword_9F308 )
      return fclose(qword_9F308);
  }
  return result;
}



// Function: diff_init @ 0x1075e
unsigned long long diff_init()
{
  char v1[8]; // [rsp+0h] [rbp-10h] BYREF
  unsigned long long v2; // [rsp+8h] [rbp-8h]

  v2 = __readfsqword(0x28u);
  qword_9F328 = (void *)sub_2B601(v1, qword_9F870);
  if ( qword_9F8E0 )
    sub_24090();
  return v2 - __readfsqword(0x28u);
}



// Function: report_difference @ 0x107be
unsigned long long report_difference(long long a1, const char *a2, long long a3, long long a4, long long a5, long long a6, ...)
{
  const char *v6; // rax
  gcc_va_list arg; // [rsp+10h] [rbp-D0h] BYREF
  unsigned long long v9; // [rsp+28h] [rbp-B8h]
  long long v10; // [rsp+40h] [rbp-A0h]
  long long v11; // [rsp+48h] [rbp-98h]
  long long v12; // [rsp+50h] [rbp-90h]
  long long v13; // [rsp+58h] [rbp-88h]

  v10 = a3;
  v11 = a4;
  v12 = a5;
  v13 = a6;
  v9 = __readfsqword(0x28u);
  if ( a2 )
  {
    v6 = (const char *)sub_29516(1, *(long long *)(a1 + 8));
    fprintf(stream, "%s: ", v6);
    va_start(arg, a6);
    vfprintf(stream, a2, arg);
    fprintf(stream, "\n");
  }
  sub_39E98(1);
  return v9 - __readfsqword(0x28u);
}



// Function: process_noop @ 0x10903
long long process_noop()
{
  return 1;
}



// Function: process_rawdata @ 0x1091a
long long process_rawdata(size_t a1, const void *a2)
{
  long long v2; // rdx
  long long v3; // rcx
  long long v4; // r8
  long long v5; // r9
  char *v6; // rax
  long long v7; // r8
  long long v8; // r9
  char *v10; // rax
  long long v11; // rdx
  long long v12; // rcx
  long long v13; // r8
  long long v14; // r9
  long long v15; // [rsp+18h] [rbp-8h]

  v15 = sub_2A9E9((unsigned int)dword_9F324, qword_9F328, a1);
  if ( v15 == a1 )
  {
    if ( !memcmp(a2, qword_9F328, a1) )
    {
      return 1;
    }
    else
    {
      v10 = gettext("Contents differ");
      sub_107BE((long long)&qword_9FA20, v10, v11, v12, v13, v14);
      return 0;
    }
  }
  else
  {
    if ( v15 == -1 )
    {
      sub_3E403(file);
      sub_107BE((long long)&qword_9FA20, 0, v2, v3, v4, v5);
    }
    else
    {
      v6 = ngettext("Could only read %lu of %lu byte", "Could only read %lu of %lu bytes", a1);
      sub_107BE((long long)&qword_9FA20, v6, v15, a1, v7, v8);
    }
    return 0;
  }
}



// Function: read_and_process @ 0x10a26
void read_and_process(long long *a1, long long (*a2)())
{
  char *v2; // rax
  unsigned long long v4; // [rsp+18h] [rbp-18h]
  unsigned long long v5; // [rsp+20h] [rbp-10h]
  void *v6; // [rsp+28h] [rbp-8h]

  v5 = a1[17];
  sub_AD2B(a1);
  while ( 1 )
  {
    if ( !v5 )
    {
      sub_AD66();
      return;
    }
    v6 = sub_BB4B();
    if ( !v6 )
      break;
    v4 = (unsigned long long)sub_BBF5((long long)v6);
    if ( v4 > v5 )
      v4 = v5;
    if ( !((unsigned int (*)(unsigned long long, void *))a2)(v4, v6) )
      a2 = sub_10903;
    sub_BBA7((unsigned long long)v6 + v4 - 1);
    v5 -= v4;
    sub_AD86(v5);
  }
  if ( qword_9FDC8 )
    qword_9FDC8();
  v2 = gettext("Unexpected EOF in archive");
  error(0, 0, v2);
  dword_9FDD0 = 2;
}



// Function: get_stat_data @ 0x10b3a
long long get_stat_data(long long a1, long long a2)
{
  long long v2; // rdx
  long long v3; // rcx
  long long v4; // r8
  long long v5; // r9

  if ( !(unsigned int)sub_2A9B8(a1, a2) )
    return 1;
  if ( *__errno_location() == 2 )
    sub_3EA2C(a1);
  else
    sub_3EA03(a1);
  sub_107BE((long long)&qword_9FA20, 0, v2, v3, v4, v5);
  return 0;
}



// Function: diff_dir @ 0x10bb7
unsigned long long diff_dir()
{
  char *v0; // rax
  long long v1; // rdx
  long long v2; // rcx
  long long v3; // r8
  long long v4; // r9
  char *v5; // rax
  long long v6; // rdx
  long long v7; // rcx
  long long v8; // r8
  long long v9; // r9
  char v11[24]; // [rsp+0h] [rbp-A0h] BYREF
  int v12; // [rsp+18h] [rbp-88h]
  unsigned long long v13; // [rsp+98h] [rbp-8h]

  v13 = __readfsqword(0x28u);
  if ( (unsigned int)sub_10B3A((long long)file, (long long)v11) )
  {
    if ( (v12 & 0xF000) == 0x4000 )
    {
      if ( (((unsigned short)qword_9FA90 ^ (unsigned short)v12) & 0xFFF) != 0 )
      {
        v5 = gettext("Mode differs");
        sub_107BE((long long)&qword_9FA20, v5, v6, v7, v8, v9);
      }
    }
    else
    {
      v0 = gettext("File type differs");
      sub_107BE((long long)&qword_9FA20, v0, v1, v2, v3, v4);
    }
  }
  return v13 - __readfsqword(0x28u);
}



// Function: diff_file @ 0x10c82
unsigned long long diff_file()
{
  char *v0; // rax
  long long v1; // rdx
  long long v2; // rcx
  long long v3; // r8
  long long v4; // r9
  char *v5; // rax
  long long v6; // rdx
  long long v7; // rcx
  long long v8; // r8
  long long v9; // r9
  char *v10; // rax
  long long v11; // rdx
  long long v12; // rcx
  long long v13; // r8
  long long v14; // r9
  char *v15; // rax
  long long v16; // rdx
  long long v17; // rcx
  long long v18; // r8
  long long v19; // r9
  long long v20; // rax
  long long v21; // rdx
  char *v22; // rax
  long long v23; // rdx
  long long v24; // rcx
  long long v25; // r8
  long long v26; // r9
  char *v27; // rax
  long long v28; // rdx
  long long v29; // rcx
  long long v30; // r8
  long long v31; // r9
  long long v32; // rdx
  long long v33; // rcx
  long long v34; // r8
  long long v35; // r9
  long long v36; // rdx
  char *file; // [rsp+8h] [rbp-C8h]
  long long v39; // [rsp+10h] [rbp-C0h]
  char v40[24]; // [rsp+20h] [rbp-B0h] BYREF
  int v41; // [rsp+38h] [rbp-98h]
  long long v42; // [rsp+50h] [rbp-80h]
  unsigned long long v43; // [rsp+B8h] [rbp-18h]

  v43 = __readfsqword(0x28u);
  file = ::file;
  if ( (unsigned int)sub_10B3A((long long)::file, (long long)v40) )
  {
    if ( (v41 & 0xF000) == 0x8000 )
    {
      if ( (((unsigned short)qword_9FA90 ^ (unsigned short)v41) & 0xFFF) != 0 )
      {
        v5 = gettext("Mode differs");
        sub_107BE((long long)&qword_9FA20, v5, v6, v7, v8, v9);
      }
      if ( (unsigned char)sub_32EBF(v40, &qword_9FA78) != 1 )
      {
        v10 = gettext("Uid differs");
        sub_107BE((long long)&qword_9FA20, v10, v11, v12, v13, v14);
      }
      if ( (unsigned char)sub_32EE4(v40, &qword_9FA78) != 1 )
      {
        v15 = gettext("Gid differs");
        sub_107BE((long long)&qword_9FA20, v15, v16, v17, v18, v19);
      }
      v20 = sub_5FD9E(v40);
      if ( (unsigned int)sub_39E35(v20, v21, qword_9FB18, qword_9FB20) )
      {
        v22 = gettext("Mod time differs");
        sub_107BE((long long)&qword_9FA20, v22, v23, v24, v25, v26);
      }
      if ( *(char *)(qword_9F460 + 156) == 83 || v42 == qword_9FAA8 )
      {
        dword_9F324 = openat(newfd, file, oflag);
        if ( dword_9F324 >= 0 )
        {
          if ( byte_9FB40 )
            sub_30972((unsigned int)dword_9F324, &qword_9FA20);
          else
            sub_10A26(&qword_9FA20, (long long (*)())sub_1091A);
          if ( dword_9F880 == 1 )
          {
            if ( v42 )
            {
              v39 = sub_5FD6A(v40);
              if ( (unsigned int)sub_2AB5E((unsigned int)dword_9F324, (unsigned int)newfd, file, v39, v36) )
                sub_3EAD0(file);
            }
          }
          if ( close(dword_9F324) )
            sub_3E1FF(file);
        }
        else
        {
          sub_3E38B(file);
          sub_28891();
          sub_107BE((long long)&qword_9FA20, 0, v32, v33, v34, v35);
        }
      }
      else
      {
        v27 = gettext("Size differs");
        sub_107BE((long long)&qword_9FA20, v27, v28, v29, v30, v31);
        sub_28891();
      }
    }
    else
    {
      v0 = gettext("File type differs");
      sub_107BE((long long)&qword_9FA20, v0, v1, v2, v3, v4);
      sub_28891();
    }
  }
  else
  {
    sub_28891();
  }
  return v43 - __readfsqword(0x28u);
}



// Function: diff_link @ 0x10fea
unsigned long long diff_link()
{
  long long v0; // rbx
  char *v1; // rax
  long long v2; // rcx
  long long v3; // r8
  long long v4; // r9
  char v6[144]; // [rsp+0h] [rbp-140h] BYREF
  char v7[152]; // [rsp+90h] [rbp-B0h] BYREF
  unsigned long long v8; // [rsp+128h] [rbp-18h]

  v8 = __readfsqword(0x28u);
  if ( (unsigned int)sub_10B3A((long long)file, (long long)v6)
    && (unsigned int)sub_10B3A((long long)from, (long long)v7)
    && (unsigned char)sub_32F09(v6, v7) != 1 )
  {
    v0 = sub_29516(0, from);
    v1 = gettext("Not linked to %s");
    sub_107BE((long long)&qword_9FA20, v1, v0, v2, v3, v4);
  }
  return v8 - __readfsqword(0x28u);
}



// Function: diff_symlink @ 0x110be
unsigned long long diff_symlink()
{
  char *v0; // rax
  long long v1; // rdx
  long long v2; // rcx
  long long v3; // r8
  long long v4; // r9
  char *v5; // rax
  long long v6; // rdx
  long long v7; // rcx
  long long v8; // r8
  long long v9; // r9
  size_t n; // [rsp+8h] [rbp-428h]
  char *buf; // [rsp+10h] [rbp-420h]
  ssize_t v13; // [rsp+18h] [rbp-418h]
  char v14[1032]; // [rsp+20h] [rbp-410h] BYREF
  unsigned long long v15; // [rsp+428h] [rbp-8h]

  v15 = __readfsqword(0x28u);
  n = strlen(from);
  if ( n > 0x3FF )
    v0 = (char *)sub_60B5B(n + 1);
  else
    v0 = v14;
  buf = v0;
  v13 = readlinkat(newfd, file, v0, n + 1);
  if ( v13 >= 0 )
  {
    if ( v13 != n || memcmp(from, buf, n) )
    {
      v5 = gettext("Symlink differs");
      sub_107BE((long long)&qword_9FA20, v5, v6, v7, v8, v9);
    }
  }
  else
  {
    if ( *__errno_location() == 2 )
      sub_3E6DC(file);
    else
      sub_3E6B3(file);
    sub_107BE((long long)&qword_9FA20, 0, v1, v2, v3, v4);
  }
  if ( buf != v14 )
    free(buf);
  return v15 - __readfsqword(0x28u);
}



// Function: diff_special @ 0x11230
unsigned long long diff_special()
{
  bool v0; // al
  char *v1; // rax
  long long v2; // rdx
  long long v3; // rcx
  long long v4; // r8
  long long v5; // r9
  char *v6; // rax
  long long v7; // rdx
  long long v8; // rcx
  long long v9; // r8
  long long v10; // r9
  char *v11; // rax
  long long v12; // rdx
  long long v13; // rcx
  long long v14; // r8
  long long v15; // r9
  char v17[24]; // [rsp+0h] [rbp-A0h] BYREF
  int v18; // [rsp+18h] [rbp-88h]
  long long v19; // [rsp+28h] [rbp-78h]
  unsigned long long v20; // [rsp+98h] [rbp-8h]

  v20 = __readfsqword(0x28u);
  if ( (unsigned int)sub_10B3A((long long)file, (long long)v17) )
  {
    if ( *(char *)(qword_9F460 + 156) == 51 )
    {
      v0 = (v18 & 0xF000) != 0x2000;
    }
    else if ( *(char *)(qword_9F460 + 156) == 52 )
    {
      v0 = (v18 & 0xF000) != 24576;
    }
    else
    {
      v0 = (v18 & 0xF000) != 4096;
    }
    if ( v0 )
    {
      v1 = gettext("File type differs");
      sub_107BE((long long)&qword_9FA20, v1, v2, v3, v4, v5);
    }
    else if ( (*(char *)(qword_9F460 + 156) == 51 || *(char *)(qword_9F460 + 156) == 52) && dev != v19 )
    {
      v6 = gettext("Device number differs");
      sub_107BE((long long)&qword_9FA20, v6, v7, v8, v9, v10);
    }
    else if ( (((unsigned short)qword_9FA90 ^ (unsigned short)v18) & 0xFFF) != 0 )
    {
      v11 = gettext("Mode differs");
      sub_107BE((long long)&qword_9FA20, v11, v12, v13, v14, v15);
    }
  }
  return v20 - __readfsqword(0x28u);
}



// Function: dumpdir_cmp @ 0x113b1
long long dumpdir_cmp(const char *a1, const char *a2)
{
  unsigned int v2; // eax
  long long v3; // rax
  size_t v7; // [rsp+18h] [rbp-8h]

  while ( *a1 )
  {
    v2 = *a1 - 68;
    if ( v2 <= 0x15 )
    {
      v3 = 1LL << v2;
      if ( (v3 & 0x114000) != 0 )
        return (unsigned int)*a2;
      if ( (v3 & 0x200400) != 0 )
      {
        if ( !strchr("YN", *a2) )
          return 1;
        if ( strcmp(a1 + 1, a2 + 1) )
          return 1;
        goto LABEL_11;
      }
      if ( (v3 & 1) != 0 )
      {
        if ( strcmp(a1, a2) )
          return 1;
LABEL_11:
        v7 = strlen(a1) + 1;
        a1 += v7;
        a2 += v7;
      }
    }
  }
  return (unsigned int)*a2;
}



// Function: diff_dumpdir @ 0x11512
void diff_dumpdir(long long *a1)
{
  long long v1; // rax
  char *v2; // rax
  long long v3; // rcx
  long long v4; // r8
  long long v5; // r9
  char fd[12]; // [rsp+1Ch] [rbp-14h]
  const char *v7; // [rsp+28h] [rbp-8h]

  if ( *((int *)a1 + 106) )
    goto LABEL_9;
  *(int *)&fd[8] = 0;
  *(long long *)fd = (unsigned int)sub_157F9(a1[51], *a1, (unsigned int)oflag);
  if ( *(int *)fd >= 0 )
  {
    if ( fstat(*(int *)fd, (struct stat *)(a1 + 11)) )
    {
      *(long long *)&fd[4] = sub_2B444;
      close(*(int *)fd);
    }
    else
    {
      *((int *)a1 + 106) = *(int *)fd;
    }
  }
  else
  {
    *(long long *)&fd[4] = sub_2B2B1;
  }
  if ( !*(long long *)&fd[4] )
  {
LABEL_9:
    v1 = sub_21EB9(a1);
    v7 = (const char *)sub_2241B(v1);
    if ( v7 )
    {
      if ( (unsigned int)sub_113B1((const char *)a1[50], v7) )
      {
        v2 = gettext("Contents differ");
        sub_107BE((long long)a1, v2, (long long)v2, v3, v4, v5);
      }
    }
    else
    {
      sub_10A26(a1, sub_10903);
    }
  }
  else
  {
    sub_2B48B(*a1, 0, *(long long *)&fd[4]);
  }
}



// Function: diff_multivol @ 0x11650
unsigned long long diff_multivol()
{
  char *v0; // rax
  long long v1; // rdx
  long long v2; // rcx
  long long v3; // r8
  long long v4; // r9
  char *v5; // rax
  long long v6; // rdx
  long long v7; // rcx
  long long v8; // r8
  long long v9; // r9
  long long v10; // rdx
  long long v11; // rcx
  long long v12; // r8
  long long v13; // r9
  long long v14; // rdx
  long long v15; // rcx
  long long v16; // r8
  long long v17; // r9
  int fd; // [rsp+0h] [rbp-B0h]
  long long offset; // [rsp+8h] [rbp-A8h]
  char v21[24]; // [rsp+10h] [rbp-A0h] BYREF
  int v22; // [rsp+28h] [rbp-88h]
  long long v23; // [rsp+40h] [rbp-70h]
  unsigned long long v24; // [rsp+A8h] [rbp-8h]

  v24 = __readfsqword(0x28u);
  if ( byte_9FA30 )
  {
    sub_10BB7();
  }
  else if ( (unsigned int)sub_10B3A((long long)file, (long long)v21) )
  {
    if ( (v22 & 0xF000) == 0x8000 )
    {
      offset = sub_2798B(qword_9F460 + 369, 12);
      if ( offset >= 0 && !__OFADD__(qword_9FAA8, offset) && v23 == qword_9FAA8 + offset )
      {
        fd = openat(newfd, file, oflag);
        if ( fd >= 0 )
        {
          if ( lseek(fd, offset, 0) >= 0 )
          {
            sub_10A26(&qword_9FA20, (long long (*)())sub_1091A);
          }
          else
          {
            sub_3E7A9(file, offset);
            sub_107BE((long long)&qword_9FA20, 0, v14, v15, v16, v17);
          }
          if ( close(fd) )
            sub_3E1FF(file);
        }
        else
        {
          sub_3E38B(file);
          sub_107BE((long long)&qword_9FA20, 0, v10, v11, v12, v13, fd);
          sub_28891();
        }
      }
      else
      {
        v5 = gettext("Size differs");
        sub_107BE((long long)&qword_9FA20, v5, v6, v7, v8, v9);
        sub_28891();
      }
    }
    else
    {
      v0 = gettext("File type differs");
      sub_107BE((long long)&qword_9FA20, v0, v1, v2, v3, v4);
      sub_28891();
    }
  }
  return v24 - __readfsqword(0x28u);
}



// Function: diff_archive @ 0x11897
unsigned long long diff_archive()
{
  char *v0; // rax
  unsigned long long result; // rax
  unsigned int v2; // r12d
  long long v3; // rbx
  char *v4; // rax

  sub_BBA7(qword_9F460);
  if ( dword_9F9AC )
  {
    if ( byte_9F320 )
    {
      v0 = gettext("Verify ");
      fprintf(stream, v0);
    }
    sub_2864C(&qword_9FA20, qword_9F460, -1);
  }
  result = (unsigned int)*(char *)(qword_9F460 + 156);
  switch ( *(char *)(qword_9F460 + 156) )
  {
    case 0:
    case 0x30:
    case 0x37:
    case 0x53:
      goto LABEL_9;
    case 0x31:
      return sub_10FEA();
    case 0x32:
      return sub_110BE();
    case 0x33:
    case 0x34:
    case 0x36:
      return sub_11230();
    case 0x35:
    case 0x44:
      if ( (unsigned char)sub_2490B(&qword_9FA20) )
        sub_11512(&qword_9FA20);
      return sub_10BB7();
    case 0x4D:
      return sub_11650();
    case 0x56:
      return result;
    default:
      if ( qword_9FDC8 )
        qword_9FDC8();
      v2 = *(char *)(qword_9F460 + 156);
      v3 = sub_5E548(file);
      v4 = gettext("%s: Unknown file type '%c', diffed as normal file");
      error(0, 0, v4, v3, v2);
      dword_9FDD0 = 2;
LABEL_9:
      if ( byte_9FA30 )
        return sub_10BB7();
      else
        return sub_10C82();
  }
}



// Function: verify_volume @ 0x11a1c
unsigned long long verify_volume()
{
  char *v0; // rax
  char *v1; // rax
  char *v2; // rax
  int v3; // eax
  int v4; // eax
  bool v5; // al
  char *v6; // rax
  signed long long v7; // rax
  long long v8; // rbx
  char *v9; // rax
  int v11; // [rsp+8h] [rbp-48h]
  int v12; // [rsp+Ch] [rbp-44h]
  unsigned int v13; // [rsp+10h] [rbp-40h]
  short v14[2]; // [rsp+18h] [rbp-38h] BYREF
  int v15; // [rsp+1Ch] [rbp-34h]
  char v16[24]; // [rsp+20h] [rbp-30h] BYREF
  unsigned long long v17; // [rsp+38h] [rbp-18h]

  v17 = __readfsqword(0x28u);
  v11 = 0;
  if ( (unsigned char)sub_3ED80() )
  {
    if ( qword_9FDC8 )
      qword_9FDC8();
    v0 = gettext("Archive contains file names with leading prefixes removed.");
    error(0, 0, v0);
    v11 = 1;
  }
  if ( (unsigned char)sub_3BA20() )
  {
    if ( qword_9FDC8 )
      qword_9FDC8();
    v1 = gettext("Archive contains transformed file names.");
    error(0, 0, v1);
    v11 = 1;
  }
  if ( v11 )
  {
    if ( qword_9FDC8 )
      qword_9FDC8();
    v2 = gettext("Verification may fail to locate original files.");
    error(0, 0, v2);
  }
  sub_20F88();
  if ( !qword_9F328 )
    sub_1075E();
  fsync(fd);
  ioctl(fd, 0x24Bu);
  v14[0] = 2;
  v15 = 1;
  if ( fd <= 0x3FFFFFFF )
    v3 = ioctl(fd, 0x40086D01u, v14);
  else
    v3 = sub_40254((unsigned int)(fd - 0x40000000), 1074294017, v14);
  if ( v3 >= 0
    || *__errno_location() == 5
    && (fd <= 0x3FFFFFFF
      ? (v4 = ioctl(fd, 0x40086D01u, v14))
      : (v4 = sub_40254((unsigned int)(fd - 0x40000000), 1074294017, v14)),
        v4 >= 0)
    || (fd <= 0x3FFFFFFF ? (v5 = lseek(fd, 0, 0) != 0) : (v5 = sub_400BE((unsigned int)(fd - 0x40000000), 0, 0) != 0),
        !v5) )
  {
    dword_9EFB8 = 0;
    byte_9F320 = 1;
    sub_F662();
    while ( 1 )
    {
      v12 = sub_25ECF(&qword_9F460, &qword_9FA20, 0);
      if ( v12 == 5 )
      {
        v13 = 0;
        do
        {
          ++v13;
          sub_BBA7(qword_9F460);
          v12 = sub_25ECF(&qword_9F460, &qword_9FA20, 0);
        }
        while ( v12 == 5 );
        if ( qword_9FDC8 )
          qword_9FDC8();
        v6 = ngettext(
               "VERIFY FAILURE: %d invalid header detected",
               "VERIFY FAILURE: %d invalid headers detected",
               (int)v13);
        error(0, 0, v6, v13);
        dword_9FDD0 = 2;
      }
      if ( v12 == 4 )
        break;
      if ( v12 == 3 )
      {
        sub_BBA7(qword_9F460);
        if ( byte_9F8BD != 1 )
        {
          if ( (unsigned int)sub_25ECF(&qword_9F460, &qword_9FA20, 0) == 3 )
            break;
          if ( (dword_9EC84 & 1) != 0 )
          {
            if ( qword_9FDC8 )
              qword_9FDC8();
            v7 = sub_BACD();
            v8 = sub_54E5A(v7, v16);
            v9 = gettext("A lone zero block at %s");
            error(0, 0, v9, v8);
          }
        }
      }
      else
      {
        sub_26747(qword_9F460, &qword_9FA20, &dword_9F468, 1);
        sub_11897();
        sub_39D1C(&qword_9FA20);
      }
    }
    dword_9EFB8 = 1;
    byte_9F320 = 0;
  }
  else
  {
    sub_3E867(*(long long *)qword_9FBD8);
  }
  return v17 - __readfsqword(0x28u);
}



// Function: add_exclusion_tag @ 0x11ea1
long long add_exclusion_tag(const char *a1, int a2, long long a3)
{
  long long v5; // [rsp+28h] [rbp-8h]

  v5 = sub_60B5B(40);
  *(long long *)(v5 + 32) = qword_9F330;
  *(long long *)v5 = a1;
  *(int *)(v5 + 16) = a2;
  *(long long *)(v5 + 24) = a3;
  *(long long *)(v5 + 8) = strlen(a1);
  qword_9F330 = v5;
  return v5;
}



// Function: exclusion_tag_warning @ 0x11f18
void exclusion_tag_warning(long long a1, long long a2, long long a3)
{
  long long v3; // r12
  long long v4; // rbx
  char *v5; // rax

  if ( dword_9F9AC && (dword_9EC84 & 4) != 0 )
  {
    if ( qword_9FDC8 )
      qword_9FDC8();
    v3 = sub_5E285(1, a2);
    v4 = sub_5E548(a1);
    v5 = gettext("%s: contains a cache directory tag %s; %s");
    error(0, 0, v5, v4, v3, a3);
  }
}



// Function: check_exclusion_tags @ 0x11fbf
long long check_exclusion_tags(long long a1, long long *a2)
{
  bool v2; // al
  bool v4; // [rsp+13h] [rbp-Dh]
  int fd; // [rsp+14h] [rbp-Ch]
  long long i; // [rsp+18h] [rbp-8h]

  for ( i = qword_9F330; ; i = *(long long *)(i + 32) )
  {
    if ( !i )
      return 0;
    fd = sub_157F9(a1, *(long long *)i, (unsigned int)oflag);
    if ( fd >= 0 )
    {
      v2 = !*(long long *)(i + 24) || (*(unsigned char (**)(long long))(i + 24))((unsigned int)fd);
      v4 = v2;
      close(fd);
      if ( v4 )
        break;
    }
  }
  if ( a2 )
    *a2 = *(long long *)i;
  return *(unsigned int *)(i + 16);
}



// Function: cachedir_file_p @ 0x12087
long long cachedir_file_p(int a1)
{
  char buf[56]; // [rsp+10h] [rbp-40h] BYREF
  unsigned long long v4; // [rsp+48h] [rbp-8h]

  v4 = __readfsqword(0x28u);
  return read(a1, buf, 0x2Bu) == 43 && !memcmp(buf, "Signature: 8a477f597d28d172789f06886806bc55", 0x2Bu);
}



// Function: to_octal @ 0x12105
long long to_octal(unsigned long long a1, long long a2, long long a3)
{
  long long result; // rax

  do
  {
    --a3;
    result = a2 + a3;
    *(char *)(a2 + a3) = (a1 & 7) + 48;
    a1 >>= 3;
  }
  while ( a3 );
  return result;
}



// Function: tar_copy_str @ 0x12157
unsigned long long tar_copy_str(long long a1, long long a2, unsigned long long a3)
{
  unsigned long long result; // rax
  unsigned long long i; // [rsp+20h] [rbp-8h]

  for ( i = 0; ; ++i )
  {
    result = i;
    if ( i >= a3 )
      break;
    *(char *)(a1 + i) = *(char *)(i + a2);
    result = *(unsigned char *)(a1 + i);
    if ( !(char)result )
      break;
  }
  return result;
}



// Function: tar_name_copy_str @ 0x121ac
unsigned long long tar_name_copy_str(long long a1, long long a2, unsigned long long a3)
{
  unsigned long long result; // rax

  sub_12157(a1, a2, a3);
  result = (unsigned int)dword_9F864;
  if ( dword_9F864 == 2 )
  {
    result = a3 - 1 + a1;
    *(char *)result = 0;
  }
  return result;
}



// Function: to_base256 @ 0x121fb
long long to_base256(int a1, unsigned long long a2, long long a3, long long a4)
{
  long long result; // rax

  do
  {
    --a4;
    *(char *)(a3 + a4) = a2;
    result = ((long long)-a1 << 56) | (a2 >> 8);
    a2 = result;
  }
  while ( a4 );
  return result;
}



// Function: to_chars_subst @ 0x12262
long long to_chars_subst(
        int a1,
        int a2,
        long long a3,
        int a4,
        long long (*a5)(int *),
        int a6,
        long long a7,
        long long a8)
{
  long long v8; // rax
  unsigned long long v9; // rax
  long long v10; // rax
  long long v11; // rax
  char *v12; // rax
  char *v14; // rax
  int v19; // [rsp+3Ch] [rbp-D4h] BYREF
  const char *v20; // [rsp+40h] [rbp-D0h]
  long long v21; // [rsp+48h] [rbp-C8h]
  long long v22; // [rsp+50h] [rbp-C0h]
  long long v23; // [rsp+58h] [rbp-B8h]
  long long v24; // [rsp+60h] [rbp-B0h]
  unsigned long long v25; // [rsp+68h] [rbp-A8h]
  char *v26; // [rsp+70h] [rbp-A0h]
  char *v27; // [rsp+78h] [rbp-98h]
  long long v28; // [rsp+80h] [rbp-90h]
  long long v29; // [rsp+88h] [rbp-88h]
  char v30[32]; // [rsp+90h] [rbp-80h] BYREF
  char v31; // [rsp+B0h] [rbp-60h] BYREF
  char v32[7]; // [rsp+B1h] [rbp-5Fh] BYREF
  char v33[7]; // [rsp+D1h] [rbp-3Fh] BYREF
  char v34[31]; // [rsp+F1h] [rbp-1Fh] BYREF

  *(long long *)&v34[23] = __readfsqword(0x28u);
  if ( a2 )
  {
    if ( (unsigned long long)(8 * (a7 - 1)) > 0x3F )
      v8 = -1;
    else
      v8 = (1LL << (8 * ((unsigned char)a7 - 1))) - 1;
  }
  else if ( (unsigned long long)(3 * a7 - 3) > 0x3F )
  {
    v8 = -1;
  }
  else
  {
    v8 = (1LL << (3 * (unsigned char)a7 - 3)) - 1;
  }
  v23 = v8;
  v24 = sub_54E5A(v8, v30);
  if ( a2 )
  {
    if ( v23 == -1 )
      v9 = 0x8000000000000000LL;
    else
      v9 = v23 + 1;
    v25 = v9;
    v26 = (char *)(sub_54E5A(v9, v33) - 1);
    *v26 = 45;
    v20 = v26;
  }
  else
  {
    v20 = "0";
  }
  if ( a1 )
  {
    v27 = (char *)(sub_54E5A(-a3, v32) - 1);
    *v27 = 45;
    v21 = (long long)v27;
  }
  else
  {
    v21 = sub_54E5A(a3, &v31);
  }
  if ( a5 )
  {
    v10 = a5(&v19);
    v28 = v23 & v10;
    v19 = (dword_9F864 == 6) & (unsigned char)v19;
    if ( v19 )
      v11 = -v28;
    else
      v11 = v28;
    v29 = v11;
    v22 = sub_54E5A(v11, v34);
    if ( v19 )
      *(char *)--v22 = 45;
    if ( qword_9FDC8 )
      qword_9FDC8();
    v12 = gettext("value %s out of %s range %s..%s; substituting %s");
    error(0, 0, v12, v21, a8, v20, v24, v22);
    return sub_1262B(v19, v29, a4, 0, a6, a7, a8);
  }
  else
  {
    if ( qword_9FDC8 )
      qword_9FDC8();
    v14 = gettext("value %s out of %s range %s..%s");
    error(0, 0, v14, v21, a8, v20, v24);
    dword_9FDD0 = 2;
    return 0;
  }
}



// Function: to_chars @ 0x1262b
long long to_chars(
        int a1,
        unsigned long long a2,
        long long a3,
        long long (*a4)(int *),
        char *a5,
        long long a6,
        long long a7)
{
  int v7; // eax
  unsigned long long v8; // rax
  unsigned long long v10; // rax
  unsigned long long v11; // rdx
  char v12; // dl
  char *v13; // rax
  long long v14; // rax
  int v19; // [rsp+3Ch] [rbp-4h]

  v7 = dword_9F864 == 6 || dword_9F864 == 2;
  v19 = v7;
  if ( !a1 )
  {
    v8 = (unsigned long long)(3 * a6 - 3) > 0x3F ? -1LL : (1LL << (3 * (unsigned char)a6 - 3)) - 1;
    if ( v8 >= a2 )
    {
      a5[a6 - 1] = 0;
      sub_12105(a2, (long long)a5, a6 - 1);
      return 1;
    }
  }
  if ( !v19 )
  {
    a4 = 0;
    return sub_12262(a1, v19, a2, a3, a4, (int)a5, a6, a7);
  }
  if ( a1 )
    v10 = ~a2;
  else
    v10 = a2;
  if ( (unsigned long long)(8 * (a6 - 1)) > 0x3F )
    v11 = -1;
  else
    v11 = (1LL << (8 * ((unsigned char)a6 - 1))) - 1;
  if ( v10 <= v11 )
  {
    if ( a1 )
      v12 = -1;
    else
      v12 = 0x80;
    *a5 = v12;
    sub_121FB(a1, a2, (long long)(a5 + 1), a6 - 1);
    return 1;
  }
  if ( !a1 || 8 * a3 > (unsigned long long)(3 * a6 - 3) )
    return sub_12262(a1, v19, a2, a3, a4, (int)a5, a6, a7);
  if ( !dword_9F348 )
  {
    dword_9F348 = 1;
    if ( qword_9FDC8 )
      qword_9FDC8();
    v13 = gettext("Generating negative octal headers");
    error(0, 0, v13);
  }
  a5[a6 - 1] = 0;
  if ( (unsigned long long)(8 * a3) > 0x3F )
    v14 = -1;
  else
    v14 = (1LL << (8 * (unsigned char)a3)) - 1;
  sub_12105(a2 & v14, (long long)a5, a6 - 1);
  return 1;
}



// Function: gid_substitute @ 0x128a8
long long gid_substitute(int *a1)
{
  unsigned int v2; // [rsp+1Ch] [rbp-4h]

  if ( !dword_9F34C && !(unsigned int)sub_2C261("nobody", &dword_9F34C) )
    dword_9F34C = -2;
  v2 = dword_9F34C;
  *a1 = 0;
  return v2;
}



// Function: gid_to_chars @ 0x12901
long long gid_to_chars(unsigned int a1, char *a2, long long a3)
{
  return sub_1262B(0, a1, 4, sub_128A8, a2, a3, (long long)"gid_t");
}



// Function: major_to_chars @ 0x12957
long long major_to_chars(int a1, char *a2, long long a3)
{
  return sub_1262B(a1 < 0, a1, 4, 0, a2, a3, (long long)"major_t");
}



// Function: minor_to_chars @ 0x129ae
long long minor_to_chars(int a1, char *a2, long long a3)
{
  return sub_1262B(a1 < 0, a1, 4, 0, a2, a3, (long long)"minor_t");
}



// Function: mode_to_chars @ 0x12a05
long long mode_to_chars(unsigned int a1, char *a2, long long a3)
{
  if ( dword_9F864 == 4 || dword_9F864 == 3 || dword_9F864 == 6 )
    return sub_1262B(0, a1 & 0xFFF, 4, 0, a2, a3, (long long)"mode_t");
  else
    return sub_1262B(0, a1, 4, 0, a2, a3, (long long)"mode_t");
}



// Function: off_to_chars @ 0x12a9a
long long off_to_chars(signed long long a1, char *a2, long long a3)
{
  return sub_1262B(a1 < 0, a1, 8, 0, a2, a3, (long long)"off_t");
}



// Function: time_to_chars @ 0x12af2
long long time_to_chars(signed long long a1, char *a2, long long a3)
{
  return sub_1262B(a1 < 0, a1, 8, 0, a2, a3, (long long)"time_t");
}



// Function: uid_substitute @ 0x12b4a
long long uid_substitute(int *a1)
{
  unsigned int v2; // [rsp+1Ch] [rbp-4h]

  if ( !dword_9F350 && !(unsigned int)sub_2C173("nobody", &dword_9F350) )
    dword_9F350 = -2;
  v2 = dword_9F350;
  *a1 = 0;
  return v2;
}



// Function: uid_to_chars @ 0x12ba3
long long uid_to_chars(unsigned int a1, char *a2, long long a3)
{
  return sub_1262B(0, a1, 4, sub_12B4A, a2, a3, (long long)"uid_t");
}



// Function: uintmax_to_chars @ 0x12bf9
long long uintmax_to_chars(unsigned long long a1, char *a2, long long a3)
{
  return sub_1262B(0, a1, 8, 0, a2, a3, (long long)"uintmax_t");
}



// Function: string_to_chars @ 0x12c4c
char *string_to_chars(long long a1, long long a2, unsigned long long a3)
{
  char *result; // rax

  sub_12157(a2, a1, a3);
  result = (char *)(a3 - 1 + a2);
  *result = 0;
  return result;
}



// Function: file_dumpable_p @ 0x12c90
long long file_dumpable_p(long long a1)
{
  int v2; // eax
  bool v3; // al

  if ( (*(int *)(a1 + 24) & 0xF000) == 0x4000 )
    return 1;
  if ( (*(int *)(a1 + 24) & 0xF000) != 0x8000 )
    return 0;
  if ( byte_9F9D0 )
  {
    v3 = 0;
    if ( byte_9F999 && byte_9F979 )
    {
      v2 = (*(long long *)(a1 + 48) & 0x1FFLL) != 0 && (unsigned long long)(*(long long *)(a1 + 48) + 511LL) > 0x3FE;
      if ( *(long long *)(a1 + 64) < *(long long *)(a1 + 48) / 512LL + v2 )
        return 1;
    }
    return v3;
  }
  else
  {
    return *(long long *)(a1 + 48) || (*(int *)(a1 + 24) & 0x124) != 0x124;
  }
}



// Function: write_eot @ 0x12da0
void *write_eot()
{
  char *v0; // rax
  void *s; // [rsp+8h] [rbp-8h]
  void *sa; // [rsp+8h] [rbp-8h]

  s = sub_BB4B();
  memset(s, 0, 0x200u);
  sub_BBA7((unsigned long long)s);
  sa = sub_BB4B();
  v0 = sub_BBF5((long long)sa);
  memset(sa, 0, (size_t)v0);
  return sub_BBA7((unsigned long long)sa);
}



// Function: start_private_header @ 0x12e0f
char *start_private_header(long long a1, signed long long a2, long long a3)
{
  signed long long v3; // rax
  char *s; // [rsp+28h] [rbp-8h]

  s = (char *)sub_BB4B();
  memset(s, 0, 0x200u);
  sub_121AC((long long)s, a1, 0x64u);
  sub_12A9A(a2, s + 124, 12);
  if ( a3 < 0 )
  {
    v3 = 0;
  }
  else
  {
    v3 = a3;
    if ( (unsigned long long)a3 > 0x1FFFFFFFFLL )
      v3 = 0x1FFFFFFFFLL;
  }
  sub_12AF2(v3, s + 136, 12);
  sub_12A05(0x81A4u, s + 100, 8);
  sub_12BA3(0, s + 108, 8);
  sub_12901(0, s + 116, 8);
  memcpy(s + 257, "ustar", 6u);
  memcpy(s + 263, "00", 2u);
  return s;
}



// Function: write_short_name @ 0x12f4e
void *write_short_name(long long a1)
{
  void *s; // [rsp+18h] [rbp-8h]

  s = sub_BB4B();
  memset(s, 0, 0x200u);
  sub_121AC((long long)s, *(long long *)(a1 + 8), 0x64u);
  return s;
}



// Function: write_gnu_long_link @ 0x12f9f
void *write_gnu_long_link(long long a1, const char *a2, char a3)
{
  char *s; // [rsp+10h] [rbp-30h]
  size_t v6; // [rsp+28h] [rbp-18h]
  size_t n; // [rsp+30h] [rbp-10h]
  char *dest; // [rsp+38h] [rbp-8h]
  char *desta; // [rsp+38h] [rbp-8h]

  s = (char *)a2;
  v6 = strlen(a2) + 1;
  dest = sub_12E0F((long long)"././@LongLink", v6, 0);
  if ( byte_9F934 != 1 )
  {
    if ( !qword_9F358 )
    {
      sub_2BFD1(0, &qword_9F358);
      sub_2C0A2(0, &qword_9F360);
    }
    sub_12C4C(qword_9F358, (long long)(dest + 265), 0x20u);
    sub_12C4C(qword_9F360, (long long)(dest + 297), 0x20u);
  }
  strcpy(dest + 257, "ustar  ");
  dest[156] = a3;
  sub_1422E(a1, dest, -1);
  desta = (char *)sub_BB4B();
  for ( n = (size_t)sub_BBF5((long long)desta); n < v6; n = (size_t)sub_BBF5((long long)desta) )
  {
    memcpy(desta, s, n);
    s += n;
    v6 -= n;
    sub_BBA7((unsigned long long)&desta[(n - 1) & 0xFFFFFFFFFFFFFE00LL]);
    desta = (char *)sub_BB4B();
  }
  memcpy(desta, s, v6);
  memset(&desta[v6], 0, n - v6);
  return sub_BBA7((unsigned long long)&desta[(v6 - 1) & 0xFFFFFFFFFFFFFE00LL]);
}



// Function: split_long_name @ 0x13197
long long split_long_name(long long a1, unsigned long long a2)
{
  long long v3; // [rsp+0h] [rbp-20h]
  long long i; // [rsp+18h] [rbp-8h]

  v3 = a2;
  if ( a2 <= 0x9C )
  {
    if ( *(char *)(a2 - 1 + a1) == 47 )
      v3 = a2 - 1;
  }
  else
  {
    v3 = 156;
  }
  for ( i = v3 - 1; i && *(char *)(a1 + i) != 47; --i )
    ;
  return i;
}



// Function: write_ustar_long_name @ 0x1320b
char *write_ustar_long_name(const char *a1)
{
  long long v1; // rbx
  char *v2; // rax
  long long v4; // rbx
  char *v5; // rax
  size_t v6; // [rsp+10h] [rbp-30h]
  long long n; // [rsp+18h] [rbp-28h]
  char *dest; // [rsp+28h] [rbp-18h]

  v6 = strlen(a1);
  if ( v6 <= 0x100 )
  {
    n = sub_13197((long long)a1, v6);
    if ( n && v6 - n - 1 <= 0x64 && v6 - n != 1 )
    {
      dest = (char *)sub_BB4B();
      memset(dest, 0, 0x200u);
      memcpy(dest + 345, a1, n);
      memcpy(dest, &a1[n + 1], v6 - n - 1);
      return dest;
    }
    else
    {
      if ( qword_9FDC8 )
        qword_9FDC8();
      v4 = sub_5E548(a1);
      v5 = gettext("%s: file name is too long (cannot be split); not dumped");
      error(0, 0, v5, v4);
      dword_9FDD0 = 2;
      return 0;
    }
  }
  else
  {
    if ( qword_9FDC8 )
      qword_9FDC8();
    v1 = sub_5E548(a1);
    v2 = gettext("%s: file name is too long (max %d); not dumped");
    error(0, 0, v2, v1, 256);
    dword_9FDD0 = 2;
    return 0;
  }
}



// Function: write_long_link @ 0x133a8
void write_long_link(long long a1)
{
  long long v1; // rax
  long long v2; // rbx
  char *v3; // rax

  if ( (unsigned int)dword_9F864 > 6 )
    goto LABEL_10;
  v1 = 1LL << dword_9F864;
  if ( ((1LL << dword_9F864) & 0x2A) != 0 )
  {
    if ( qword_9FDC8 )
      qword_9FDC8();
    v2 = sub_5E548(*(long long *)(a1 + 24));
    v3 = gettext("%s: link name is too long; not dumped");
    error(0, 0, v3, v2);
    dword_9FDD0 = 2;
  }
  else
  {
    if ( (v1 & 0x44) == 0 )
    {
      if ( (v1 & 0x10) != 0 )
      {
        sub_1E1D3("linkpath", a1, 0);
        return;
      }
LABEL_10:
      abort();
    }
    sub_12F9F(a1, *(const char **)(a1 + 24), 75);
  }
}



// Function: write_long_name @ 0x134b6
char *write_long_name(long long a1)
{
  long long v1; // rbx
  char *v2; // rax
  char *result; // rax

  switch ( dword_9F864 )
  {
    case 1:
      if ( strlen(*(const char **)(a1 + 8)) <= 0x63 )
        goto LABEL_10;
      if ( qword_9FDC8 )
        qword_9FDC8();
      v1 = sub_5E548(*(long long *)(a1 + 8));
      v2 = gettext("%s: file name is too long (max %d); not dumped");
      error(0, 0, v2, v1, 99);
      dword_9FDD0 = 2;
      result = 0;
      break;
    case 2:
    case 6:
      sub_12F9F(a1, *(const char **)(a1 + 8), 76);
      goto LABEL_10;
    case 3:
    case 5:
      return sub_1320B(*(const char **)(a1 + 8));
    case 4:
      sub_1E1D3("path", a1, 0);
LABEL_10:
      result = (char *)sub_12F4E(a1);
      break;
    default:
      abort();
  }
  return result;
}



// Function: write_extended @ 0x135e3
void *write_extended(char a1, long long *a2, const void *a3)
{
  long long v4; // rax
  void *ptr; // [rsp+28h] [rbp-228h]
  void *v7; // [rsp+38h] [rbp-218h]
  char dest[520]; // [rsp+40h] [rbp-210h] BYREF
  unsigned long long v9; // [rsp+248h] [rbp-8h]

  v9 = __readfsqword(0x28u);
  if ( a2[47] || !a2[45] )
    return (void *)a3;
  sub_1E672(a2 + 45);
  memcpy(dest, a3, 0x200u);
  if ( a1 )
  {
    ptr = (void *)sub_1D18D();
    sub_1D280(103, ptr, qword_9F9E0, a2 + 45);
  }
  else
  {
    ptr = (void *)sub_1D123(a2);
    if ( dword_9F910 )
      v4 = qword_9F920;
    else
      v4 = a2[22];
    sub_1D280(120, ptr, v4, a2 + 45);
  }
  free(ptr);
  v7 = sub_BB4B();
  memcpy(v7, dest, 0x200u);
  return v7;
}



// Function: write_header_name @ 0x13775
char *write_header_name(long long a1)
{
  size_t v2; // rbx

  if ( dword_9F864 == 4 && (unsigned char)sub_3C9B7(*(long long *)(a1 + 8)) != 1 )
  {
    sub_1E1D3("path", a1, 0);
    return (char *)sub_12F4E(a1);
  }
  else
  {
    if ( dword_9F864 == 2 )
      v2 = 99;
    else
      v2 = 100;
    if ( v2 >= strlen(*(const char **)(a1 + 8)) )
      return (char *)sub_12F4E(a1);
    else
      return sub_134B6(a1);
  }
}



// Function: start_header @ 0x1381d
char *start_header(long long a1)
{
  unsigned int v2; // ebx
  long long v3; // rdx
  long long v4; // rdx
  char v5; // dl
  unsigned int v6; // [rsp+18h] [rbp-68h]
  unsigned int v7; // [rsp+1Ch] [rbp-64h]
  unsigned int v8; // [rsp+20h] [rbp-60h]
  unsigned int v9; // [rsp+24h] [rbp-5Ch]
  long long v10; // [rsp+28h] [rbp-58h] BYREF
  long long v11; // [rsp+30h] [rbp-50h] BYREF
  unsigned long long v12; // [rsp+38h] [rbp-48h]
  char *v13; // [rsp+40h] [rbp-40h]
  long long v14; // [rsp+48h] [rbp-38h]
  unsigned long long v15; // [rsp+50h] [rbp-30h] BYREF
  long long v16; // [rsp+58h] [rbp-28h]
  unsigned long long v17; // [rsp+68h] [rbp-18h]

  v17 = __readfsqword(0x28u);
  v10 = 0;
  v11 = 0;
  v13 = sub_13775(a1);
  if ( !v13 )
    return 0;
  sub_29274(*(unsigned int *)(a1 + 116), a1 + 116, &v10);
  sub_293CE(*(unsigned int *)(a1 + 120), a1 + 120, &v11);
  if ( qword_9F8F0 )
  {
    v2 = *(int *)(a1 + 112) & 0xFFFFF000;
    *(int *)(a1 + 112) = sub_55B51(
                              *(unsigned int *)(a1 + 112),
                              (*(int *)(a1 + 112) & 0xF000) == 0x4000,
                              dword_9F8F8,
                              qword_9F8F0,
                              0)
                          | v2;
  }
  if ( dword_9F864 == 1 || dword_9F864 == 3 )
    sub_12A05(*(int *)(a1 + 112) & 0xFFF, v13 + 100, 8);
  else
    sub_12A05(*(int *)(a1 + 112), v13 + 100, 8);
  v6 = *(int *)(a1 + 116);
  if ( dword_9F864 == 4 && v6 > 0x1FFFFF )
  {
    sub_1E1D3(&unk_80F28, a1, 0);
    v6 = 0;
  }
  if ( (unsigned char)sub_12BA3(v6, v13 + 108, 8) != 1 )
    return 0;
  v7 = *(int *)(a1 + 120);
  if ( dword_9F864 == 4 && v7 > 0x1FFFFF )
  {
    sub_1E1D3(&unk_80F2C, a1, 0);
    v7 = 0;
  }
  if ( (unsigned char)sub_12901(v7, v13 + 116, 8) != 1 )
    return 0;
  v12 = *(long long *)(a1 + 136);
  if ( dword_9F864 == 4 && v12 > 0x1FFFFFFFFLL )
  {
    sub_1E1D3("size", a1, 0);
    v12 = 0;
  }
  if ( (unsigned char)sub_12A9A(v12, v13 + 124, 12) != 1 )
    return 0;
  if ( dword_9F910 == 2 )
  {
    if ( (int)sub_5FF90(*(long long *)(a1 + 248), *(long long *)(a1 + 256), qword_9F920, qword_9F928) > 0 )
    {
      v15 = qword_9F920;
      v16 = qword_9F928;
    }
    else
    {
      v4 = *(long long *)(a1 + 256);
      v15 = *(long long *)(a1 + 248);
      v16 = v4;
    }
  }
  else if ( (unsigned int)dword_9F910 <= 2 )
  {
    if ( dword_9F910 )
    {
      if ( dword_9F910 == 1 )
      {
        v15 = qword_9F920;
        v16 = qword_9F928;
      }
    }
    else
    {
      v3 = *(long long *)(a1 + 256);
      v15 = *(long long *)(a1 + 248);
      v16 = v3;
    }
  }
  if ( dword_9F864 == 4 )
  {
    if ( v15 > 0x1FFFFFFFFLL || v16 )
      sub_1E1D3("mtime", a1, &v15);
    if ( v15 > 0x1FFFFFFFFLL )
      v15 = 0;
  }
  if ( (unsigned char)sub_12AF2(v15, v13 + 136, 12) != 1 )
    return 0;
  if ( (*(int *)(a1 + 112) & 0xF000) == 0x2000 || (*(int *)(a1 + 112) & 0xF000) == 0x6000 )
  {
    v8 = gnu_dev_major(*(long long *)(a1 + 128));
    v9 = gnu_dev_minor(*(long long *)(a1 + 128));
    if ( dword_9F864 == 4 && v8 > 0x1FFFFF )
    {
      sub_1E1D3("devmajor", a1, 0);
      v8 = 0;
    }
    if ( (unsigned char)sub_12957(v8, v13 + 329, 8) != 1 )
      return 0;
    if ( dword_9F864 == 4 && v9 > 0x1FFFFF )
    {
      sub_1E1D3("devminor", a1, 0);
      v9 = 0;
    }
    if ( (unsigned char)sub_129AE(v9, v13 + 337, 8) != 1 )
      return 0;
  }
  else if ( dword_9F864 != 6
         && dword_9F864 != 2
         && ((unsigned char)sub_12957(0, v13 + 329, 8) != 1 || (unsigned char)sub_129AE(0, v13 + 337, 8) != 1) )
  {
    return 0;
  }
  if ( dword_9F864 == 4 )
  {
    sub_1E1D3("atime", a1, 0);
    sub_1E1D3("ctime", a1, 0);
  }
  else if ( byte_9F8BE && (dword_9F864 == 2 || dword_9F864 == 6) )
  {
    sub_12AF2(*(long long *)(a1 + 232), v13 + 345, 12);
    sub_12AF2(*(long long *)(a1 + 264), v13 + 357, 12);
  }
  if ( dword_9F864 == 1 )
    v5 = 0;
  else
    v5 = 48;
  v13[156] = v5;
  if ( dword_9F864 == 6 )
  {
LABEL_73:
    strcpy(v13 + 257, "ustar  ");
    goto LABEL_76;
  }
  if ( (unsigned int)dword_9F864 > 4 )
LABEL_75:
    abort();
  if ( (unsigned int)dword_9F864 < 3 )
  {
    if ( dword_9F864 == 1 )
      goto LABEL_76;
    if ( dword_9F864 == 2 )
      goto LABEL_73;
    goto LABEL_75;
  }
  memcpy(v13 + 257, "ustar", 6u);
  memcpy(v13 + 263, "00", 2u);
LABEL_76:
  if ( dword_9F864 != 1 && !byte_9F934 )
  {
    if ( v10 )
      *(long long *)(a1 + 32) = sub_60CC6(v10);
    else
      sub_2BFD1(*(unsigned int *)(a1 + 116), a1 + 32);
    if ( v11 )
      *(long long *)(a1 + 40) = sub_60CC6(v11);
    else
      sub_2C0A2(*(unsigned int *)(a1 + 120), a1 + 40);
    if ( dword_9F864 == 4
      && (strlen(*(const char **)(a1 + 32)) > 0x20 || (unsigned char)sub_3C9B7(*(long long *)(a1 + 32)) != 1) )
    {
      sub_1E1D3("uname", a1, 0);
    }
    sub_12C4C(*(long long *)(a1 + 32), (long long)(v13 + 265), 0x20u);
    if ( dword_9F864 == 4
      && (strlen(*(const char **)(a1 + 40)) > 0x20 || (unsigned char)sub_3C9B7(*(long long *)(a1 + 40)) != 1) )
    {
      sub_1E1D3("gname", a1, 0);
    }
    sub_12C4C(*(long long *)(a1 + 40), (long long)(v13 + 297), 0x20u);
  }
  if ( dword_9F864 == 4 )
  {
    if ( dword_9F968 > 0 )
    {
      if ( *(long long *)(a1 + 56) )
        sub_1E1D3("SCHILY.acl.access", a1, 0);
      if ( *(long long *)(a1 + 72) )
        sub_1E1D3("SCHILY.acl.default", a1, 0);
    }
    if ( dword_9F964 > 0 && *(long long *)(a1 + 48) )
      sub_1E1D3("RHT.security.selinux", a1, 0);
    if ( dword_9F96C > 0 )
    {
      v15 = 0;
      v14 = *(long long *)(a1 + 352);
      while ( *(long long *)(a1 + 344) > v15 )
      {
        sub_1E1D3(*(long long *)(24 * v15 + v14), a1, &v15);
        ++v15;
      }
    }
  }
  return v13;
}



// Function: simple_finish_header @ 0x1418c
void *simple_finish_header(unsigned long long a1)
{
  unsigned char *v1; // rax
  int v4; // [rsp+1Ch] [rbp-14h]
  long long v5; // [rsp+20h] [rbp-10h]
  unsigned char *v6; // [rsp+28h] [rbp-8h]

  memcpy((void *)(a1 + 148), "        ", 8u);
  v4 = 0;
  v6 = (unsigned char *)a1;
  v5 = 512;
  while ( v5-- )
  {
    v1 = v6++;
    v4 += *v1;
  }
  sub_12BF9(v4, (char *)(a1 + 148), 7);
  return sub_BBA7(a1);
}



// Function: finish_header @ 0x1422e
void *finish_header(long long *a1, char *a2, long long a3)
{
  void *v4; // [rsp+10h] [rbp-10h]

  if ( dword_9F9AC && a2[156] != 75 && a2[156] != 76 && a2[156] != 120 && a2[156] != 103 )
  {
    dword_9F468 = dword_9F864;
    sub_2864C(a1, a2, a3);
  }
  v4 = sub_135E3(0, a1, a2);
  return sub_1418C((unsigned long long)v4);
}



// Function: pad_archive @ 0x142d7
void pad_archive(long long a1)
{
  void *s; // [rsp+18h] [rbp-8h]

  while ( a1 > 0 )
  {
    s = sub_BB4B();
    memset(s, 0, 0x200u);
    sub_BBA7((unsigned long long)s);
    a1 -= 512;
  }
}



// Function: dump_regular_file @ 0x14327
long long dump_regular_file(int a1, long long *a2)
{
  long long v3; // rax
  long long v4; // r12
  long long v5; // rbx
  char *v6; // rax
  long long n; // [rsp+18h] [rbp-58h]
  char *v8; // [rsp+20h] [rbp-50h]
  signed long long v9; // [rsp+28h] [rbp-48h]
  char *v10; // [rsp+30h] [rbp-40h]
  char *v11; // [rsp+30h] [rbp-40h]
  long long v12; // [rsp+38h] [rbp-38h]
  char v13[24]; // [rsp+40h] [rbp-30h] BYREF
  unsigned long long v14; // [rsp+58h] [rbp-18h]

  v14 = __readfsqword(0x28u);
  n = a2[17];
  v9 = sub_BACD();
  v10 = sub_1381D((long long)a2);
  if ( !v10 )
    return 2;
  sub_1422E(a2, v10, v9);
  sub_AB09(a2[1], a2[17], a2[17]);
  do
  {
    if ( n <= 0 )
      return 0;
    v11 = (char *)sub_BB4B();
    v8 = sub_BBF5((long long)v11);
    if ( (unsigned long long)v8 > n )
    {
      v8 = (char *)n;
      if ( (n & 0x1FF) != 0 )
        memset(&v11[n], 0, 512 - (n & 0x1FF));
    }
    if ( a1 <= 0 )
      v3 = (long long)v8;
    else
      v3 = sub_2A9E9((unsigned int)a1, v11, v8);
    v12 = v3;
    if ( v3 == -1 )
    {
      sub_2B2F8(*a2, a2[17] - n, v8);
      sub_142D7(n);
      return 1;
    }
    n -= v3;
    sub_BBA7((unsigned long long)&v11[(unsigned long long)(v8 - 1) & 0xFFFFFFFFFFFFFE00LL]);
  }
  while ( (char *)v12 == v8 );
  memset(&v11[v12], 0, (size_t)&v8[-v12]);
  if ( (dword_9EC84 & 0x80) != 0 )
  {
    if ( qword_9FDC8 )
      qword_9FDC8();
    v4 = sub_54E5A(n, v13);
    v5 = sub_5E548(*a2);
    v6 = ngettext(
           "%s: File shrank by %s byte; padding with zeros",
           "%s: File shrank by %s bytes; padding with zeros",
           n);
    error(0, 0, v6, v5, v4);
  }
  if ( byte_9F8BC != 1 )
    sub_39E98(1);
  sub_142D7(v12 - (long long)v8 + n);
  return 1;
}



// Function: dump_dir0 @ 0x145e3
unsigned long long dump_dir0(long long *a1, char *a2)
{
  long long v2; // rax
  long long v3; // rbx
  char *v4; // rax
  unsigned int v5; // eax
  char *v6; // rax
  size_t v7; // rbx
  char *v8; // rax
  bool v10; // [rsp+17h] [rbp-89h]
  char *v11; // [rsp+18h] [rbp-88h] BYREF
  signed long long v12; // [rsp+20h] [rbp-80h]
  size_t n; // [rsp+28h] [rbp-78h]
  void *src; // [rsp+30h] [rbp-70h]
  char *s; // [rsp+38h] [rbp-68h]
  size_t v16; // [rsp+40h] [rbp-60h]
  char *i; // [rsp+48h] [rbp-58h]
  void *dest; // [rsp+50h] [rbp-50h]
  long long v19; // [rsp+58h] [rbp-48h]
  void *v20; // [rsp+60h] [rbp-40h]
  long long v21; // [rsp+68h] [rbp-38h]
  signed long long v22; // [rsp+70h] [rbp-30h]
  size_t v23; // [rsp+78h] [rbp-28h]
  size_t v24; // [rsp+80h] [rbp-20h]
  unsigned long long v25; // [rsp+88h] [rbp-18h]

  v25 = __readfsqword(0x28u);
  v10 = a1[51] == 0;
  v19 = sub_BACD();
  a1[17] = 0;
  dest = sub_1381D((long long)a1);
  if ( !dest )
    return v25 - __readfsqword(0x28u);
  sub_17909(a1);
  if ( !byte_9F8BE || dword_9F864 == 4 )
    *((char *)dest + 156) = 53;
  else
    *((char *)dest + 156) = 68;
  if ( byte_9F8BE != 1 )
  {
    sub_1422E(a1, dest, v19);
    goto LABEL_18;
  }
  if ( !*(long long *)(qword_9F6B0 + 56) )
  {
LABEL_18:
    if ( dword_9F930 )
    {
      if ( byte_9F935 && !v10 && *(long long *)(a1[51] + 88) != a1[11] )
      {
        if ( dword_9F9AC && (dword_9EC84 & 0x40000) != 0 )
        {
          if ( qword_9FDC8 )
            qword_9FDC8();
          v3 = sub_5E548(*a1);
          v4 = gettext("%s: file is on a different filesystem; not dumped");
          error(0, 0, v4, v3);
        }
      }
      else
      {
        v5 = sub_11FBF((long long)a1, &v11);
        if ( v5 <= 2 )
        {
          if ( v5 == 2 )
          {
            v8 = gettext("contents not dumped");
            sub_11F18(*a1, (long long)v11, (long long)v8);
          }
          else if ( v5 )
          {
            v6 = gettext("contents not dumped");
            sub_11F18(*a1, (long long)v11, (long long)v6);
            v7 = strlen((const char *)*a1);
            v16 = v7 + strlen(v11) + 1;
            s = (char *)sub_60B5B(v16);
            strcpy(s, (const char *)*a1);
            strcat(s, v11);
            sub_16A25(a1, v11, s);
            free(s);
          }
          else
          {
            s = (char *)sub_60CC6(*a1);
            v23 = strlen(s);
            v16 = v23;
            for ( i = a2; ; i += v24 + 1 )
            {
              v24 = strlen(i);
              if ( !v24 )
                break;
              if ( v16 < v23 + v24 )
              {
                v16 = v23 + v24;
                s = (char *)sub_60B8D(s, v23 + v24 + 1);
              }
              strcpy(&s[v23], i);
              if ( (unsigned char)sub_17C43(s, a1) != 1 )
                sub_16A25(a1, i, s);
            }
            free(s);
          }
        }
      }
    }
    return v25 - __readfsqword(0x28u);
  }
  if ( dword_9F864 == 4 )
  {
    v2 = sub_22456(*(long long *)(qword_9F6B0 + 56));
    sub_1E1D3("GNU.dumpdir", a1, v2);
    sub_1422E(a1, dest, v19);
  }
  else
  {
    v19 = sub_BACD();
    v20 = (void *)sub_22456(*(long long *)(qword_9F6B0 + 56));
    v21 = sub_20BFE(v20);
    sub_12A9A(v21, (char *)dest + 124, 12);
    sub_1422E(a1, dest, v19);
    src = v20;
    v12 = v21;
    sub_AB09(a1[1], v21, v21);
    while ( v12 > 0 )
    {
      dest = sub_BB4B();
      n = (size_t)sub_BBF5((long long)dest);
      if ( n > v12 )
      {
        n = v12;
        v22 = v12 & 0x1FF;
        if ( (v12 & 0x1FF) != 0 )
          memset((char *)dest + v12, 0, 512 - v22);
      }
      memcpy(dest, src, n);
      v12 -= n;
      src = (char *)src + n;
      sub_BBA7((unsigned long long)dest + ((n - 1) & 0xFFFFFFFFFFFFFE00LL));
    }
  }
  return v25 - __readfsqword(0x28u);
}



// Function: ensure_slash @ 0x14bcf
const char *ensure_slash(const char **a1)
{
  const char *result; // rax
  size_t i; // [rsp+18h] [rbp-8h]

  for ( i = strlen(*a1); i && (*a1)[i - 1] == 47; --i )
    ;
  if ( (*a1)[i] != 47 )
    *a1 = (const char *)sub_60B8D(*a1, i + 2);
  (*a1)[i] = 47;
  result = &(*a1)[i + 1];
  *result = 0;
  return result;
}



// Function: open_failure_recover @ 0x14c7c
long long open_failure_recover(long long a1)
{
  long long i; // [rsp+18h] [rbp-8h]

  if ( *__errno_location() == 24 && a1 && *(long long *)(a1 + 408) )
  {
    for ( i = *(long long *)(*(long long *)(a1 + 408) + 408LL); i; i = *(long long *)(i + 408) )
    {
      if ( *(int *)(i + 424) > 0 && (!*(long long *)(i + 408) || *(int *)(*(long long *)(i + 408) + 424LL) <= 0) )
      {
        sub_39C7E(i);
        return 1;
      }
    }
    *__errno_location() = 24;
  }
  return 0;
}



// Function: get_directory_entries @ 0x14d3d
long long get_directory_entries(long long a1)
{
  while ( 1 )
  {
    *(long long *)(a1 + 416) = fdopendir(*(int *)(a1 + 424));
    if ( *(long long *)(a1 + 416) )
      break;
    if ( (unsigned char)sub_14C7C(a1) != 1 )
      return 0;
  }
  return sub_5EEE2(*(long long *)(a1 + 416), (unsigned int)dword_9FC2C);
}



// Function: dump_dir @ 0x14db4
long long dump_dir(long long *a1)
{
  char *ptr; // [rsp+18h] [rbp-8h]

  ptr = (char *)sub_14D3D((long long)a1);
  if ( ptr )
  {
    sub_145E3(a1, ptr);
    sub_1588E(a1);
    free(ptr);
    return 1;
  }
  else
  {
    sub_2B3A4(*a1);
    return 0;
  }
}



// Function: create_archive @ 0x14e23
unsigned long long create_archive()
{
  int v0; // eax
  size_t v1; // rax
  int fd; // [rsp+4h] [rbp-1FCh]
  size_t v4; // [rsp+8h] [rbp-1F8h] BYREF
  void *dest; // [rsp+10h] [rbp-1F0h]
  char *s; // [rsp+18h] [rbp-1E8h]
  size_t n; // [rsp+20h] [rbp-1E0h]
  long long v8; // [rsp+28h] [rbp-1D8h]
  long long v9; // [rsp+30h] [rbp-1D0h]
  size_t v10; // [rsp+38h] [rbp-1C8h]
  long long v11[11]; // [rsp+40h] [rbp-1C0h] BYREF
  struct stat v12[2]; // [rsp+98h] [rbp-168h] BYREF
  int v13; // [rsp+1E8h] [rbp-18h]
  unsigned long long v14; // [rsp+1F8h] [rbp-8h]

  v14 = __readfsqword(0x28u);
  v0 = dword_9F6A0 != 2 && byte_9F8A0 != 1;
  qword_9F338 = v0;
  sub_F694(1u);
  sub_AD11();
  if ( byte_9F8BE )
  {
    v4 = 0;
    dest = 0;
    sub_2E719();
    while ( 1 )
    {
      v9 = sub_2EDA0();
      if ( !v9 )
        break;
      if ( (unsigned char)sub_17C43(*(long long *)(v9 + 16), 0) != 1 )
        sub_16A25(0, *(long long *)(v9 + 16), *(long long *)(v9 + 16));
    }
    sub_2EE43();
    while ( 1 )
    {
      v9 = sub_2EDA0();
      if ( !v9 )
        break;
      if ( (unsigned char)sub_17C43(*(long long *)(v9 + 16), 0) != 1 )
      {
        n = strlen(*(const char **)(v9 + 16));
        while ( n >= v4 )
          dest = (void *)sub_60BD8(dest, &v4);
        memcpy(dest, *(const void **)(v9 + 16), n);
        if ( *((char *)dest + n - 1) != 47 )
        {
          v1 = n++;
          *((char *)dest + v1) = 47;
        }
        sub_39C55(v11);
        s = (char *)sub_2241B(*(long long *)(v9 + 56));
        if ( s )
        {
          while ( *s )
          {
            v10 = strlen(s);
            if ( *s == 89 )
            {
              if ( !v11[0] )
              {
                fd = openat(newfd, *(const char **)(v9 + 16), dword_9FC0C);
                if ( fd < 0 )
                {
                  sub_2B48B(*(long long *)(v9 + 16), *(long long *)(v9 + 64) == 0, sub_2B2B1);
                  break;
                }
                v13 = fd;
                if ( fstat(fd, v12) )
                {
                  sub_2B48B(*(long long *)(v9 + 16), *(long long *)(v9 + 64) == 0, sub_2B444);
                  break;
                }
                v11[0] = sub_60CC6(*(long long *)(v9 + 16));
              }
              while ( v10 + n > v4 )
                dest = (void *)sub_60BD8(dest, &v4);
              strcpy((char *)dest + n, s + 1);
              sub_16A25(v11, s + 1, dest);
            }
            s += v10 + 1;
          }
        }
        sub_39D1C(v11);
      }
    }
    free(dest);
  }
  else
  {
    while ( 1 )
    {
      v8 = sub_2D37C(1);
      if ( !v8 )
        break;
      if ( (unsigned char)sub_17C43(v8, 0) != 1 )
        sub_16A25(0, v8, v8);
    }
  }
  sub_12DA0();
  sub_D046();
  sub_3BE6B();
  if ( qword_9F8E0 )
    sub_24600();
  return v14 - __readfsqword(0x28u);
}



// Function: hash_link @ 0x1529f
unsigned long long hash_link(long long *a1, unsigned long long a2)
{
  return (*a1 ^ a1[1]) % a2;
}



// Function: compare_links @ 0x152df
bool compare_links(_OWORD *a1, _OWORD *a2)
{
  return *a2 == *a1;
}



// Function: unknown_file_error @ 0x15331
long long unknown_file_error(long long a1)
{
  long long v1; // rbx
  char *v2; // rax
  long long result; // rax

  if ( (dword_9EC84 & 0x20) != 0 )
  {
    if ( qword_9FDC8 )
      qword_9FDC8();
    v1 = sub_5E548(a1);
    v2 = gettext("%s: Unknown file type; file ignored");
    error(0, 0, v2, v1);
  }
  result = (unsigned char)byte_9F8BC ^ 1u;
  if ( byte_9F8BC != 1 )
    return sub_39E98(2);
  return result;
}



// Function: dump_hard_link @ 0x153bb
long long dump_hard_link(long long *a1)
{
  size_t v1; // rbx
  long long v3; // [rsp+10h] [rbp-60h]
  char *s; // [rsp+18h] [rbp-58h]
  signed long long v5; // [rsp+20h] [rbp-50h]
  char *v6; // [rsp+28h] [rbp-48h]
  long long v7[7]; // [rsp+30h] [rbp-40h] BYREF

  v7[5] = __readfsqword(0x28u);
  if ( !qword_9F340 || a1[13] <= (unsigned long long)qword_9F338 && !byte_9F94E )
    return 0;
  v7[1] = a1[12];
  v7[0] = a1[11];
  v3 = sub_52567(qword_9F340, v7);
  if ( !v3 )
    return 0;
  s = (char *)sub_3EDD9(v3 + 24, 1, (unsigned char)byte_9F878);
  if ( *(long long *)(v3 + 16) )
    --*(long long *)(v3 + 16);
  v5 = sub_BACD();
  sub_29547(a1 + 3, s);
  if ( dword_9F864 == 2 )
    v1 = 99;
  else
    v1 = 100;
  if ( v1 < strlen(s) )
    sub_133A8((long long)a1);
  a1[17] = 0;
  v6 = sub_1381D((long long)a1);
  if ( !v6 )
    return 0;
  sub_12157((long long)(v6 + 157), (long long)s, 0x64u);
  v6[156] = 49;
  sub_1422E(a1, v6, v5);
  if ( byte_9F94E )
    sub_3BEBB(*a1, 0);
  return 1;
}



// Function: file_count_links @ 0x15598
unsigned long long file_count_links(long long *a1)
{
  long long v1; // rax
  size_t v2; // rax
  char *s; // [rsp+10h] [rbp-20h] BYREF
  long long v5; // [rsp+18h] [rbp-18h]
  long long v6; // [rsp+20h] [rbp-10h]
  unsigned long long v7; // [rsp+28h] [rbp-8h]

  v7 = __readfsqword(0x28u);
  if ( !byte_9F8A1 && a1[13] > (unsigned long long)qword_9F338 )
  {
    s = 0;
    v1 = sub_3EDD9(*a1, 1, (unsigned char)byte_9F878);
    sub_29547(&s, v1);
    sub_3B9F2(&s, 2);
    v2 = strlen(s);
    v5 = sub_60B5B(v2 + 25);
    *(long long *)(v5 + 8) = a1[12];
    *(long long *)v5 = a1[11];
    *(long long *)(v5 + 16) = a1[13];
    strcpy((char *)(v5 + 24), s);
    free(s);
    if ( !qword_9F340 && (qword_9F340 = sub_52C1C(0, 0, sub_1529F, sub_152DF, 0)) == 0
      || (v6 = sub_5395D(qword_9F340, v5)) == 0 )
    {
      sub_1783C();
    }
    if ( v6 != v5 )
      abort();
    --*(long long *)(v5 + 16);
  }
  return v7 - __readfsqword(0x28u);
}



// Function: check_links @ 0x1573f
long long check_links()
{
  long long result; // rax
  long long v1; // rbx
  char *v2; // rax
  long long i; // [rsp+8h] [rbp-18h]

  result = qword_9F340;
  if ( qword_9F340 )
  {
    result = sub_525FE(qword_9F340);
    for ( i = result; i; i = result )
    {
      if ( *(long long *)(i + 16) )
      {
        if ( qword_9FDC8 )
          qword_9FDC8();
        v1 = sub_5E84E(i + 24);
        v2 = gettext("Missing links to %s.");
        error(0, 0, v2, v1);
      }
      result = sub_5265E(qword_9F340, i);
    }
  }
  return result;
}



// Function: subfile_open @ 0x157f9
long long subfile_open(long long a1, const char *a2, int a3)
{
  int v3; // eax
  int v6; // [rsp+2Ch] [rbp-4h]

  if ( byte_9F368 != 1 )
  {
    byte_9F368 = 1;
    strerror(2);
    gettext(&byte_81099);
  }
  do
  {
    if ( a1 )
      v3 = *(int *)(a1 + 424);
    else
      v3 = newfd;
    v6 = openat(v3, a2, a3);
  }
  while ( v6 < 0 && (unsigned char)sub_14C7C(a1) );
  return (unsigned int)v6;
}



// Function: restore_parent_fd @ 0x1588e
unsigned long long restore_parent_fd(long long a1)
{
  int fd; // [rsp+10h] [rbp-B0h]
  int v3; // [rsp+14h] [rbp-ACh]
  long long v4; // [rsp+18h] [rbp-A8h]
  struct stat buf; // [rsp+20h] [rbp-A0h] BYREF
  unsigned long long v6; // [rsp+B8h] [rbp-8h]

  v6 = __readfsqword(0x28u);
  v4 = *(long long *)(a1 + 408);
  if ( v4 && !*(int *)(v4 + 424) )
  {
    fd = openat(*(int *)(a1 + 424), "..", dword_9FC0C);
    if ( fd >= 0 )
    {
      if ( fstat(fd, &buf) || *(long long *)(v4 + 96) != buf.st_ino || *(long long *)(v4 + 88) != buf.st_dev )
      {
        close(fd);
        fd = 2;
      }
    }
    else
    {
      fd = -*__errno_location();
    }
    if ( fd < 0 )
    {
      v3 = openat(newfd, *(const char **)v4, dword_9FC0C);
      if ( v3 >= 0 )
      {
        if ( !fstat(fd, &buf) && *(long long *)(v4 + 96) == buf.st_ino && *(long long *)(v4 + 88) == buf.st_dev )
          fd = v3;
        else
          close(v3);
      }
    }
    *(int *)(v4 + 424) = fd;
  }
  return v6 - __readfsqword(0x28u);
}



// Function: dump_file0 @ 0x15a63
unsigned long long dump_file0(long long a1, const char *a2, long long a3)
{
  int v3; // eax
  long long v4; // rax
  long long v5; // rdx
  long long v6; // rdx
  long long v7; // rdx
  long long v8; // rbx
  char *v9; // rax
  long long v10; // rbx
  char *v11; // rax
  char *v12; // rax
  int v13; // eax
  int v14; // eax
  long long v15; // rax
  long long v16; // rdx
  long long v17; // rbx
  char *v18; // rax
  size_t v19; // rbx
  long long v20; // rbx
  char *v21; // rax
  unsigned int v22; // eax
  unsigned int v23; // eax
  unsigned char v26; // [rsp+20h] [rbp-100h]
  char v27; // [rsp+21h] [rbp-FFh]
  bool v28; // [rsp+22h] [rbp-FEh]
  bool v29; // [rsp+23h] [rbp-FDh]
  unsigned int fd; // [rsp+24h] [rbp-FCh]
  int v31; // [rsp+28h] [rbp-F8h]
  unsigned int v32; // [rsp+2Ch] [rbp-F4h]
  long long v33; // [rsp+30h] [rbp-F0h] BYREF
  long long (*v34)(); // [rsp+38h] [rbp-E8h]
  long long v35; // [rsp+40h] [rbp-E0h]
  long long v36; // [rsp+48h] [rbp-D8h]
  __off_t v37; // [rsp+50h] [rbp-D0h]
  char *v38; // [rsp+58h] [rbp-C8h]
  long long v39; // [rsp+60h] [rbp-C0h]
  long long v40; // [rsp+68h] [rbp-B8h]
  struct stat buf; // [rsp+70h] [rbp-B0h] BYREF
  unsigned long long v42; // [rsp+108h] [rbp-18h]

  v42 = __readfsqword(0x28u);
  v35 = -1;
  v31 = 0;
  v36 = *(long long *)(a1 + 408);
  v28 = v36 == 0;
  if ( v36 )
    v3 = *(int *)(v36 + 424);
  else
    v3 = newfd;
  fd = v3;
  v34 = 0;
  if ( byte_9F8C8 && !(unsigned int)sub_34AF4("add", a3) )
    return v42 - __readfsqword(0x28u);
  sub_29547(a1, a3);
  v4 = sub_3EDD9(a3, 0, (unsigned char)byte_9F878);
  sub_29547(a1 + 8, v4);
  sub_3B9F2(a1 + 8, 1);
  if ( (fd & 0x80000000) != 0 && v36 )
  {
    *__errno_location() = -fd;
    v34 = sub_2B2B1;
  }
  else if ( fstatat(fd, a2, (struct stat *)(a1 + 88), flag) )
  {
    v34 = (long long (*)())sub_2B444;
  }
  else if ( (unsigned char)sub_12C90(a1 + 88) )
  {
    v31 = sub_157F9(v36, a2, oflag);
    if ( v31 >= 0 )
    {
      *(int *)(a1 + 424) = v31;
      if ( fstat(v31, (struct stat *)(a1 + 88)) )
        v34 = (long long (*)())sub_2B444;
    }
    else
    {
      v34 = sub_2B2B1;
    }
  }
  if ( v34 )
  {
    sub_2B48B(a3, v28, v34);
    return v42 - __readfsqword(0x28u);
  }
  v37 = *(long long *)(a1 + 136);
  *(long long *)(a1 + 280) = v37;
  *(long long *)(a1 + 232) = sub_5FD6A(a1 + 88);
  *(long long *)(a1 + 240) = v5;
  *(long long *)(a1 + 248) = sub_5FD9E(a1 + 88);
  *(long long *)(a1 + 256) = v6;
  v39 = sub_5FD84(a1 + 88);
  v40 = v7;
  *(long long *)(a1 + 264) = v39;
  *(long long *)(a1 + 272) = v7;
  if ( (byte_9F8BE != 1 || v28)
    && (*(int *)(a1 + 112) & 0xF000) != 0x4000
    && (int)sub_5FF90(*(long long *)(a1 + 248), *(long long *)(a1 + 256), qword_9F900, qword_9F908) < 0
    && (!dword_9F87C || (int)sub_5FF90(*(long long *)(a1 + 264), *(long long *)(a1 + 272), qword_9F900, qword_9F908) < 0) )
  {
    if ( byte_9F8BE != 1 && dword_9F9AC && (dword_9EC84 & 0x100) != 0 )
    {
      if ( qword_9FDC8 )
        qword_9FDC8();
      v8 = sub_5E548(a3);
      v9 = gettext("%s: file is unchanged; not dumped");
      error(0, 0, v9, v8);
    }
    return v42 - __readfsqword(0x28u);
  }
  if ( (unsigned char)sub_32B37(a1) )
  {
    if ( (dword_9EC84 & 0x400) != 0 )
    {
      if ( qword_9FDC8 )
        qword_9FDC8();
      v10 = sub_5E548(a3);
      v11 = gettext("%s: file is the archive; not dumped");
      error(0, 0, v11, v10);
    }
    return v42 - __readfsqword(0x28u);
  }
  v29 = (*(int *)(a1 + 112) & 0xF000) == 0x4000;
  if ( (*(int *)(a1 + 112) & 0xF000) != 0x4000 && (unsigned char)sub_153BB((long long *)a1) )
    return v42 - __readfsqword(0x28u);
  if ( !v29 && (*(int *)(a1 + 112) & 0xF000) != 0x8000 )
  {
    switch ( *(int *)(a1 + 112) & 0xF000 )
    {
      case 40960:
        *(long long *)(a1 + 24) = sub_47649(fd, a2, *(long long *)(a1 + 136));
        if ( *(long long *)(a1 + 24) )
        {
          sub_3B9F2(a1 + 24, 4);
          if ( dword_9F864 == 2 )
            v19 = 99;
          else
            v19 = 100;
          if ( v19 < strlen(*(const char **)(a1 + 24)) )
            sub_133A8(a1);
          sub_3D566(fd, a2, a1, 0);
          sub_3D1A5(fd, a2, a1, 0);
          v35 = sub_BACD();
          *(long long *)(a1 + 136) = 0;
          v38 = sub_1381D(a1);
          if ( v38 )
          {
            sub_12157((long long)(v38 + 157), *(long long *)(a1 + 24), 0x64u);
            v38[156] = 50;
            sub_1422E((long long *)a1, v38, v35);
            if ( byte_9F94E )
              sub_3BEBB(a3, 0);
            sub_15598((long long *)a1);
          }
        }
        else
        {
          if ( *__errno_location() == 12 )
            sub_1783C();
          sub_2B48B(a3, v28, sub_2B35D);
        }
        return v42 - __readfsqword(0x28u);
      case 8192:
        v27 = 51;
        sub_3CF89(fd, a2, a1, 0, 1);
        sub_3D566(fd, a2, a1, 0);
        sub_3D1A5(fd, a2, a1, 0);
        break;
      case 24576:
        v27 = 52;
        sub_3CF89(fd, a2, a1, 0, 1);
        sub_3D566(fd, a2, a1, 0);
        sub_3D1A5(fd, a2, a1, 0);
        break;
      case 4096:
        v27 = 54;
        sub_3CF89(fd, a2, a1, 0, 1);
        sub_3D566(fd, a2, a1, 0);
        sub_3D1A5(fd, a2, a1, 0);
        break;
      case 49152:
        if ( (dword_9EC84 & 0x20) != 0 )
        {
          if ( qword_9FDC8 )
            qword_9FDC8();
          v20 = sub_5E548(a3);
          v21 = gettext("%s: socket ignored");
          error(0, 0, v21, v20);
        }
        return v42 - __readfsqword(0x28u);
      default:
LABEL_117:
        sub_15331(a3);
        return v42 - __readfsqword(0x28u);
    }
    if ( dword_9F864 != 1 )
    {
      v35 = sub_BACD();
      *(long long *)(a1 + 136) = 0;
      v38 = sub_1381D(a1);
      if ( v38 )
      {
        v38[156] = v27;
        if ( v27 != 54 )
        {
          v22 = gnu_dev_major(*(long long *)(a1 + 128));
          sub_12957(v22, v38 + 329, 8);
          v23 = gnu_dev_minor(*(long long *)(a1 + 128));
          sub_129AE(v23, v38 + 337, 8);
        }
        sub_1422E((long long *)a1, v38, v35);
        if ( byte_9F94E )
          sub_3BEBB(a3, 0);
      }
      return v42 - __readfsqword(0x28u);
    }
    goto LABEL_117;
  }
  sub_3CF89(fd, a2, a1, 0, !v29);
  sub_3D566(fd, a2, a1, (unsigned int)v31);
  sub_3D1A5(fd, a2, a1, (unsigned int)v31);
  if ( v29 )
  {
    sub_14BCF((const char **)a1);
    sub_14BCF((const char **)(a1 + 8));
    if ( (unsigned int)sub_11FBF(a1, &v33) == 3 )
    {
      v12 = gettext("directory not dumped");
      sub_11F18(*(long long *)a1, v33, (long long)v12);
      return v42 - __readfsqword(0x28u);
    }
    v26 = sub_14DB4((long long *)a1);
    v31 = *(int *)(a1 + 424);
    if ( v28 )
      v13 = newfd;
    else
      v13 = *(int *)(v36 + 424);
    fd = v13;
  }
  else
  {
    if ( !v31
      || !byte_9F979
      || ((*(long long *)(a1 + 136) & 0x1FFLL) == 0 || (unsigned long long)(*(long long *)(a1 + 136) + 511LL) <= 0x3FE
        ? (v14 = 0)
        : (v14 = 1),
          *(long long *)(a1 + 152) >= *(long long *)(a1 + 136) / 512LL + v14
       || (v32 = sub_30090((unsigned int)v31, a1), v32 == 3)) )
    {
      v32 = sub_14327(v31, (long long *)a1);
    }
    if ( v32 == 3 )
      abort();
    if ( v32 < 2 )
      sub_15598((long long *)a1);
    v26 = v32 == 0;
  }
  if ( v26 )
  {
    if ( v31 >= 0 )
    {
      if ( v31 )
      {
        v26 = fstat(v31, &buf) == 0;
      }
      else if ( (fd & 0x80000000) != 0 && !v28 )
      {
        *__errno_location() = -fd;
        v26 = 0;
      }
      else
      {
        v26 = fstatat(fd, a2, &buf, flag) == 0;
      }
    }
    else
    {
      *__errno_location() = -v31;
      v26 = 0;
    }
    if ( v26 != 1 )
      sub_2B48B(a3, v28, sub_2B444);
  }
  if ( v26 )
  {
    v15 = sub_5FD84(&buf);
    if ( (unsigned int)sub_5FF90(v15, v16, v39, v40) && (byte_9F94E != 1 || !v29) || v37 < buf.st_size )
    {
      if ( (dword_9EC84 & 0x10) != 0 )
      {
        if ( qword_9FDC8 )
          qword_9FDC8();
        v17 = sub_5E548(a3);
        v18 = gettext("%s: file changed as we read it");
        error(0, 0, v18, v17);
      }
      sub_39E98(1);
    }
    else if ( dword_9F880 == 1
           && v31
           && (v29 || v37)
           && (unsigned int)sub_2AB5E((unsigned int)v31, fd, a2, *(long long *)(a1 + 232), *(long long *)(a1 + 240)) )
    {
      sub_3EAD0(a3);
    }
  }
  if ( (v26 & (unsigned char)sub_39C7E(a1)) != 0 && byte_9F94E )
    sub_3BEBB(a3, v29);
  return v42 - __readfsqword(0x28u);
}



// Function: dump_file @ 0x16a25
unsigned long long dump_file(long long a1, const char *a2, long long a3)
{
  char v5[408]; // [rsp+20h] [rbp-1C0h] BYREF
  long long v6; // [rsp+1B8h] [rbp-28h]
  unsigned long long v7; // [rsp+1D8h] [rbp-8h]

  v7 = __readfsqword(0x28u);
  sub_39C55(v5);
  v6 = a1;
  sub_15A63((long long)v5, a2, a3);
  if ( a1 && qword_9F8E0 )
    sub_2130D(a1);
  sub_39D1C(v5);
  return v7 - __readfsqword(0x28u);
}



// Function: move_archive @ 0x16add
unsigned long long move_archive(long long a1)
{
  bool v1; // al
  bool v2; // al
  bool v3; // al
  __off_t v4; // rax
  __off_t v5; // rax
  __off_t v6; // rax
  __off_t offset; // [rsp+18h] [rbp-28h]
  short v9[2]; // [rsp+30h] [rbp-10h] BYREF
  int v10; // [rsp+34h] [rbp-Ch]
  unsigned long long v11; // [rsp+38h] [rbp-8h]

  v11 = __readfsqword(0x28u);
  if ( a1 )
  {
    if ( a1 >= 0 )
    {
      v9[0] = 3;
      v10 = a1;
      v1 = a1 == (int)a1;
    }
    else
    {
      v9[0] = 4;
      v10 = -(int)a1;
      v1 = -(int)a1 == -a1;
    }
    if ( !v1
      || (fd <= 0x3FFFFFFF
        ? (v2 = ioctl(fd, 0x40086D01u, v9) >= 0)
        : (v2 = (int)sub_40254((unsigned int)(fd - 0x40000000), 1074294017, v9) >= 0),
          !v2
       && (*__errno_location() != 5
        || (fd <= 0x3FFFFFFF
          ? (v3 = ioctl(fd, 0x40086D01u, v9) >= 0)
          : (v3 = (int)sub_40254((unsigned int)(fd - 0x40000000), 1074294017, v9) >= 0),
            !v3))) )
    {
      v4 = fd <= 0x3FFFFFFF ? lseek(fd, 0, 1) : sub_400BE((unsigned int)(fd - 0x40000000), 0, 1);
      offset = v4 + qword_9F870 * a1;
      if ( qword_9F870 * a1 / a1 != qword_9F870 || offset < v4 != qword_9F870 * a1 < 0 )
        goto LABEL_26;
      v5 = v4 + qword_9F870 * a1;
      if ( offset < 0 )
        v5 = 0;
      offset = v5;
      v6 = fd <= 0x3FFFFFFF ? lseek(fd, v5, 0) : sub_400BE((unsigned int)(fd - 0x40000000), v5, 0);
      if ( v6 != offset )
LABEL_26:
        sub_3E7A9(*(long long *)qword_9FBD8, offset);
    }
  }
  return v11 - __readfsqword(0x28u);
}



// Function: write_record @ 0x16d4a
unsigned long long write_record(int a1)
{
  unsigned long long result; // rax
  void *v2; // [rsp+18h] [rbp-8h]

  v2 = s1;
  s1 = dest;
  if ( byte_9F384 )
  {
    fd = 1;
    sub_F676();
    fd = 0;
  }
  else
  {
    sub_16ADD(qword_9EFC8 + qword_9F370 - qword_9EFC0);
    sub_F676();
  }
  result = (unsigned long long)v2;
  s1 = v2;
  if ( a1 )
  {
    result = (unsigned char)byte_9F384 ^ 1u;
    if ( byte_9F384 != 1 )
      result = sub_16ADD(qword_9EFC0 - (qword_9EFC8 + qword_9F370));
  }
  dword_9F380 = 0;
  return result;
}



// Function: write_recent_blocks @ 0x16e0f
unsigned long long write_recent_blocks(long long a1, unsigned long long a2)
{
  int v2; // eax
  long long *v3; // rdx
  char *v4; // rax
  unsigned long long result; // rax
  unsigned long long i; // [rsp+18h] [rbp-8h]

  for ( i = 0; ; ++i )
  {
    result = i;
    if ( i >= a2 )
      break;
    v2 = dword_9F380++;
    v3 = (char *)dest + 512 * (long long)v2;
    v4 = (char *)((i << 9) + a1);
    *v3 = *(long long *)v4;
    v3[63] = *((long long *)v4 + 63);
    qmemcpy(
      (void *)((unsigned long long)(v3 + 1) & 0xFFFFFFFFFFFFFFF8LL),
      (const void *)(v4 - ((char *)v3 - ((unsigned long long)(v3 + 1) & 0xFFFFFFFFFFFFFFF8LL))),
      8LL * ((((int)v3 - (((int)v3 + 8) & 0xFFFFFFF8) + 512) & 0xFFFFFFF8) >> 3));
    if ( dword_9F380 == dword_9F868 )
      sub_16D4A(1);
  }
  return result;
}



// Function: write_recent_bytes @ 0x16ee1
unsigned long long write_recent_bytes(long long a1, unsigned long long a2)
{
  unsigned long long result; // rax
  size_t n; // [rsp+18h] [rbp-8h]

  n = a2 - (a2 >> 9 << 9);
  sub_16E0F(a1, a2 >> 9);
  memcpy((char *)dest + 512 * (long long)dword_9F380, (const void *)((a2 >> 9 << 9) + a1), n);
  if ( n <= 0x1FF )
    memset((char *)dest + 512 * (long long)dword_9F380 + n, 0, 512 - n);
  ++dword_9F380;
  result = (unsigned int)dword_9F868;
  if ( dword_9F380 == dword_9F868 )
    return sub_16D4A(1);
  return result;
}



// Function: flush_file @ 0x16fd5
char *flush_file()
{
  long long v0; // rax
  char *result; // rax
  signed long long v2; // [rsp+8h] [rbp-8h]

  sub_BBA7(qword_9F460);
  v0 = qword_9FAA8 + 511;
  if ( qword_9FAA8 + 511 < 0 )
    v0 = qword_9FAA8 + 1022;
  v2 = v0 >> 9;
  while ( v2 >= ((char *)s - (char *)qword_9EFB0) >> 9 )
  {
    v2 -= ((char *)s - (char *)qword_9EFB0) >> 9;
    sub_CAC8();
  }
  result = (char *)qword_9EFB0 + 512 * v2;
  qword_9EFB0 = result;
  return result;
}



// Function: delete_archive_members @ 0x17069
long long delete_archive_members()
{
  char *v1; // rax
  char *v2; // rax
  bool v3; // al
  long long v4; // rax
  long long *v5; // rdx
  long long v6; // rax
  char *v7; // rax
  int v9; // [rsp+0h] [rbp-30h]
  unsigned int v10; // [rsp+4h] [rbp-2Ch]
  int v11; // [rsp+8h] [rbp-28h]
  int v12; // [rsp+Ch] [rbp-24h]
  int i; // [rsp+10h] [rbp-20h]
  unsigned int v14; // [rsp+14h] [rbp-1Ch]
  int v15; // [rsp+18h] [rbp-18h]
  long long v16; // [rsp+20h] [rbp-10h]
  long long v17; // [rsp+28h] [rbp-8h]
  long long v18; // [rsp+28h] [rbp-8h]

  v9 = 0;
  v10 = 0;
  sub_2D3B1();
  sub_F694(2u);
  byte_9F384 = strcmp(*(const char **)qword_9FBD8, "-") == 0;
  while ( 2 )
  {
    v14 = sub_25ECF(&qword_9F460, &qword_9FA20, 1);
    switch ( v14 )
    {
      case 0u:
        abort();
      case 1u:
        v17 = sub_2ED0E(file);
        if ( v17 )
        {
          ++*(long long *)(v17 + 48);
          if ( !(qword_9F8D0 ? *(long long *)(v17 + 48) != qword_9F8D0 : *(long long *)(v17 + 48) == 0) )
            goto LABEL_10;
        }
        sub_28891();
        goto LABEL_26;
      case 2u:
LABEL_10:
        v9 = v14;
        goto LABEL_26;
      case 3u:
        if ( !byte_9F8BD )
          goto LABEL_13;
        sub_BBA7(qword_9F460);
        goto LABEL_26;
      case 4u:
LABEL_13:
        v9 = 4;
        goto LABEL_26;
      case 5u:
        sub_BBA7(qword_9F460);
        if ( v10 > 4 )
          goto LABEL_26;
        if ( v10 == 4 )
          abort();
        if ( v10 )
        {
          if ( v10 - 1 > 2 )
            goto LABEL_26;
        }
        else
        {
          if ( qword_9FDC8 )
            qword_9FDC8();
          v1 = gettext("This does not look like a tar archive");
          error(0, 0, v1);
        }
        if ( qword_9FDC8 )
          qword_9FDC8();
        v2 = gettext("Skipping to next header");
        error(0, 0, v2);
        dword_9FDD0 = 2;
LABEL_26:
        v10 = v14;
        if ( !v9 )
          continue;
        qword_9F370 = qword_9EFC0 - 1;
        dest = (void *)sub_60B5B(qword_9F870);
        if ( v9 == 1 || v9 == 2 )
        {
          byte_9EFD8 = 0;
          dword_9F380 = ((char *)qword_9EFB0 - (char *)s1) >> 9;
          if ( (unsigned int)(((char *)qword_9EFB0 - (char *)s1) >> 9) )
            memcpy(dest, s1, dword_9F380 << 9);
          if ( v9 == 1 )
          {
            v9 = 0;
            sub_16FD5();
          }
          while ( v9 != 4 )
          {
            if ( qword_9EFB0 == s )
              sub_CAC8();
            switch ( (unsigned int)sub_25ECF(&qword_9F460, &qword_9FA20, 0) )
            {
              case 0u:
              case 2u:
                abort();
              case 1u:
                sub_1DF5F(&qword_9FA20);
                v18 = sub_2ED0E(file);
                if ( v18
                  && ((++*(long long *)(v18 + 48), qword_9F8D0)
                    ? (v3 = *(long long *)(v18 + 48) == qword_9F8D0)
                    : (v3 = *(long long *)(v18 + 48) != 0),
                      v3) )
                {
                  sub_16FD5();
                }
                else
                {
                  if ( qword_9FB90 )
                  {
                    sub_16EE1(qword_9FB98, qword_9FB90);
                  }
                  else
                  {
                    sub_16E0F((long long)qword_9F470, qword_9F480);
                    sub_16E0F((long long)qword_9F478, qword_9F488);
                  }
                  v4 = qword_9F460;
                  v5 = (char *)dest + 512 * (long long)dword_9F380;
                  *v5 = *(long long *)qword_9F460;
                  v5[63] = *(long long *)(v4 + 504);
                  qmemcpy(
                    (void *)((unsigned long long)(v5 + 1) & 0xFFFFFFFFFFFFFFF8LL),
                    (const void *)(v4 - ((long long)v5 - ((unsigned long long)(v5 + 1) & 0xFFFFFFFFFFFFFFF8LL))),
                    8LL * ((((int)v5 - (((int)v5 + 8) & 0xFFFFFFF8) + 512) & 0xFFFFFFF8) >> 3));
                  ++dword_9F380;
                  v6 = qword_9FAA8 + 511;
                  if ( qword_9FAA8 + 511 < 0 )
                    v6 = qword_9FAA8 + 1022;
                  v16 = v6 >> 9;
                  sub_BBA7(qword_9F460);
                  if ( dword_9F380 == dword_9F868 )
                    sub_16D4A(1);
                  v11 = ((char *)s - (char *)qword_9EFB0) >> 9;
                  if ( v16 < v11 )
                    v11 = v16;
                  while ( v16 )
                  {
                    if ( qword_9EFB0 == s )
                    {
                      sub_F662();
                      qword_9EFB0 = s1;
                      v11 = dword_9F868;
                      if ( v16 < dword_9F868 )
                        v11 = v16;
                    }
                    v12 = v11;
                    if ( v11 > dword_9F868 - dword_9F380 )
                      v12 = dword_9F868 - dword_9F380;
                    if ( !v12 )
                      abort();
                    memcpy((char *)dest + 512 * (long long)dword_9F380, qword_9EFB0, v12 << 9);
                    dword_9F380 += v12;
                    qword_9EFB0 = (char *)qword_9EFB0 + 512 * (long long)v12;
                    v16 -= v12;
                    v11 -= v12;
                    if ( dword_9F380 == dword_9F868 )
                      sub_16D4A(1);
                  }
                }
                break;
              case 3u:
                if ( byte_9F8BD )
                  sub_BBA7(qword_9F460);
                else
                  v9 = 4;
                break;
              case 4u:
                v9 = 4;
                break;
              case 5u:
                if ( qword_9FDC8 )
                  qword_9FDC8();
                v7 = gettext("Deleting non-header from archive");
                error(0, 0, v7);
                dword_9FDD0 = 2;
                sub_BBA7(qword_9F460);
                break;
              default:
                abort();
            }
            sub_39D1C(&qword_9FA20);
          }
          for ( i = 0; i <= 1; sub_16D4A(i <= 1) )
          {
            v15 = dword_9F868 - dword_9F380;
            memset((char *)dest + 512 * (long long)dword_9F380, 0, (dword_9F868 - dword_9F380) << 9);
            i += v15;
          }
          if ( byte_9F384 != 1 && fd <= 0x3FFFFFFF && (unsigned int)sub_32F52((unsigned int)fd) )
            sub_3EA7E(*(long long *)qword_9FBD8);
        }
        free(dest);
        sub_D046();
        return sub_2DBBA();
      default:
        goto LABEL_26;
    }
  }
}



// Function: fatal_exit @ 0x177f4
void fatal_exit()
{
  char *v0; // rax

  if ( qword_9F388 )
    qword_9F388();
  v0 = gettext("Error is not recoverable: exiting now");
  error(2, 0, v0);
  abort();
}



// Function: xalloc_die @ 0x1783c
void xalloc_die()
{
  char *v0; // rax

  v0 = gettext("memory exhausted");
  error(0, 0, "%s", v0);
  sub_177F4();
}



// Function: excfile_add @ 0x17879
long long excfile_add(const char *a1, int a2)
{
  size_t v2; // rax
  long long v4; // [rsp+18h] [rbp-8h]

  v2 = strlen(a1);
  v4 = sub_60B5B(v2 + 16);
  *(long long *)v4 = 0;
  *(int *)(v4 + 8) = a2;
  strcpy((char *)(v4 + 12), a1);
  if ( qword_9F398 )
    *(long long *)qword_9F398 = v4;
  else
    qword_9F390 = v4;
  qword_9F398 = v4;
  return v4;
}



// Function: info_attach_exclist @ 0x17909
long long info_attach_exclist(long long a1)
{
  long long result; // rax
  int v2; // eax
  char *v3; // r12
  int *v4; // rax
  const char *v5; // rax
  int v6; // eax
  int fd; // [rsp+10h] [rbp-50h]
  int errnum; // [rsp+14h] [rbp-4Ch]
  long long i; // [rsp+18h] [rbp-48h]
  long long v10; // [rsp+20h] [rbp-40h]
  long long *v11; // [rsp+28h] [rbp-38h]
  long long v12; // [rsp+30h] [rbp-30h]
  FILE *stream; // [rsp+38h] [rbp-28h]
  long long v14; // [rsp+40h] [rbp-20h]
  long long v15; // [rsp+48h] [rbp-18h]

  v10 = 0;
  v11 = 0;
  result = *(long long *)(a1 + 432);
  if ( !result )
  {
    for ( i = qword_9F390; i; i = *(long long *)i )
    {
      if ( a1 )
        v2 = *(int *)(a1 + 424);
      else
        v2 = newfd;
      if ( !faccessat(v2, (const char *)(i + 12), 0, 0) )
      {
        fd = sub_157F9(a1, (const char *)(i + 12), 0);
        if ( fd == -1 )
        {
          sub_3E38B(i + 12);
        }
        else
        {
          stream = fdopen(fd, "r");
          if ( stream )
          {
            v12 = sub_4E53E();
            v14 = sub_18242(i + 12);
            if ( *(long long *)(v14 + 24) )
              *(long long *)(v14 + 32) = (*(long long (**)(long long))(v14 + 24))(*(long long *)(v14 + 32));
            if ( (unsigned int)sub_4F189(*(long long *)(v14 + 16), v12, stream, 1342177281, 10, *(long long *)(v14 + 32)) )
            {
              errnum = *__errno_location();
              if ( qword_9FDC8 )
                qword_9FDC8();
              v5 = (const char *)sub_5E548(i + 12);
              error(0, errnum, "%s", v5);
              sub_177F4();
            }
            fclose(stream);
            v15 = sub_60B5B(32);
            *(long long *)(v15 + 24) = v12;
            if ( *(int *)(i + 8) )
              v6 = *(int *)(v14 + 8);
            else
              v6 = *(int *)(i + 8);
            *(int *)(v15 + 16) = v6;
            *(long long *)(v15 + 8) = v11;
            *(long long *)v15 = 0;
            if ( v11 )
              *v11 = v15;
            else
              v10 = v15;
            v11 = (long long *)v15;
          }
          else
          {
            if ( qword_9FDC8 )
              qword_9FDC8();
            v3 = gettext("%s: fdopen failed");
            v4 = __errno_location();
            error(0, *v4, v3, i + 12);
            dword_9FDD0 = 2;
            close(fd);
          }
        }
      }
    }
    *(long long *)(a1 + 432) = v10;
    return a1;
  }
  return result;
}



// Function: info_free_exclist @ 0x17bda
long long info_free_exclist(long long a1)
{
  long long *ptr; // [rsp+10h] [rbp-10h]
  long long *v3; // [rsp+18h] [rbp-8h]

  for ( ptr = *(long long **)(a1 + 432); ptr; ptr = v3 )
  {
    v3 = (long long *)*ptr;
    sub_4E904(ptr[3]);
    free(ptr);
  }
  *(long long *)(a1 + 432) = 0;
  return a1;
}



// Function: excluded_name @ 0x17c43
long long excluded_name(char *a1, long long a2)
{
  long long v3; // [rsp+0h] [rbp-30h]
  unsigned char v4; // [rsp+13h] [rbp-1Dh]
  int v5; // [rsp+14h] [rbp-1Ch]
  long long **i; // [rsp+18h] [rbp-18h]
  char *j; // [rsp+20h] [rbp-10h]
  void *ptr; // [rsp+28h] [rbp-8h]

  v3 = a2;
  j = 0;
  ptr = 0;
  v5 = 0;
  if ( (unsigned char)sub_4ED6A(qword_9F8A8, a1) )
    return 1;
  if ( !a2 )
    return 0;
  v4 = 0;
  while ( v3 && v4 != 1 )
  {
    for ( i = *(long long ***)(v3 + 432); i; i = (long long **)*i )
    {
      if ( (v5 & (int)i[2]) == 0 )
      {
        v4 = sub_4ED6A(i[3], a1);
        if ( v4 )
          break;
        if ( !j )
        {
          for ( j = a1; *j == 46 && j[1] == 47; j += 2 )
            ;
        }
        v4 = sub_4ED6A(i[3], j);
        if ( v4 )
          break;
        if ( !ptr )
          ptr = (void *)sub_4E13E(a1);
        v4 = sub_4ED6A(i[3], ptr);
        if ( v4 )
          break;
      }
    }
    v3 = *(long long *)(v3 + 408);
    v5 = 2;
  }
  free(ptr);
  return v4;
}



// Function: cvs_addfn @ 0x17ddd
unsigned long long cvs_addfn(long long a1, long long a2, unsigned int a3, double a4)
{
  unsigned long long i; // [rsp+28h] [rbp-F8h]
  long long v7[29]; // [rsp+30h] [rbp-F0h] BYREF
  unsigned long long v8; // [rsp+118h] [rbp-8h]

  v8 = __readfsqword(0x28u);
  if ( !(unsigned int)sub_461A6(a2, v7, 2116, a4) )
  {
    for ( i = 0; i < v7[0]; ++i )
      sub_4EE5D(a1, *(long long *)(v7[1] + 8 * i), a3);
    sub_46399(v7, a4);
  }
  return v8 - __readfsqword(0x28u);
}



// Function: git_addfn @ 0x17eb5
long long git_addfn(long long a1, unsigned char *a2, unsigned int a3)
{
  long long result; // rax

  while ( ((*__ctype_b_loc())[(char)*a2] & 0x2000) != 0 )
    ++a2;
  result = *a2;
  if ( (char)result )
  {
    result = *a2;
    if ( (char)result != 35 )
    {
      if ( *a2 == 92 && a2[1] == 35 )
        ++a2;
      return sub_4EE5D(a1, a2, a3);
    }
  }
  return result;
}



// Function: bzr_addfn @ 0x17f4f
long long bzr_addfn(long long a1, const char *a2, unsigned int a3)
{
  long long result; // rax

  while ( ((*__ctype_b_loc())[*a2] & 0x2000) != 0 )
    ++a2;
  result = *(unsigned char *)a2;
  if ( (char)result )
  {
    result = *(unsigned char *)a2;
    if ( (char)result != 35 )
    {
      if ( *a2 == 33 )
      {
        if ( *++a2 == 33 )
          ++a2;
        else
          a3 |= 0x20000000u;
      }
      if ( !strncmp(a2, "RE:", 3u) )
      {
        a2 += 3;
        a3 = a3 & 0xE7FFFFFF | 0x8000000;
      }
      return sub_4EE5D(a1, a2, a3);
    }
  }
  return result;
}



// Function: hg_initfn @ 0x18025
int *hg_initfn(int *a1)
{
  int *result; // rax

  if ( a1 )
    result = a1;
  else
    result = &unk_9F3A0;
  *result = 0x8000000;
  return result;
}



// Function: hg_addfn @ 0x18059
int hg_addfn(long long a1, const char *a2, int a3, int *a4)
{
  int result; // eax
  unsigned int v5; // edx
  char *s1a; // [rsp+10h] [rbp-30h]
  size_t n; // [rsp+30h] [rbp-10h]
  size_t na; // [rsp+30h] [rbp-10h]
  void *dest; // [rsp+38h] [rbp-8h]

  while ( ((*__ctype_b_loc())[*a2] & 0x2000) != 0 )
    ++a2;
  result = *(unsigned char *)a2;
  if ( (char)result )
  {
    result = *(unsigned char *)a2;
    if ( (char)result != 35 )
    {
      if ( !strncmp(a2, "syntax:", 7u) )
      {
        for ( s1a = (char *)(a2 + 7); ((*__ctype_b_loc())[*s1a] & 0x2000) != 0; ++s1a )
          ;
        if ( !strcmp(s1a, "regexp") )
        {
          *a4 = 0x8000000;
          return (int)a4;
        }
        else
        {
          result = strcmp(s1a, "glob");
          if ( !result )
          {
            *a4 = 0x10000000;
            return (int)a4;
          }
        }
      }
      else
      {
        n = strlen(a2);
        if ( a2[n - 1] == 47 )
        {
          na = n - 1;
          dest = (void *)sub_60B5B(na + 1);
          memcpy(dest, a2, na);
          *((char *)dest + na) = 0;
          a2 = (const char *)dest;
          sub_4E34B(a1, dest);
          a3 |= 0x4000008u;
        }
        if ( *a4 == 0x8000000 )
          v5 = a3 & 0xEFFFFFFF;
        else
          v5 = a3 & 0xF7FFFFFF;
        return sub_4EE5D(a1, a2, *a4 | v5);
      }
    }
  }
  return result;
}



// Function: get_vcs_ignore_file @ 0x18242
const char **get_vcs_ignore_file(const char *a1)
{
  const char **i; // [rsp+18h] [rbp-8h]

  for ( i = (const char **)&off_9C160; *i && strcmp(*i, a1); i += 5 )
    ;
  return i;
}



// Function: exclude_vcs_ignores @ 0x18293
const char *exclude_vcs_ignores()
{
  const char *result; // rax
  const char **i; // [rsp+8h] [rbp-8h]

  for ( i = (const char **)&off_9C160; ; i += 5 )
  {
    result = *i;
    if ( !*i )
      break;
    sub_17879(*i, 0);
  }
  return result;
}



// Function: implemented @ 0x182d5
long long implemented(int a1)
{
  return a1 != 38 && a1 != 95;
}



// Function: extr_init @ 0x182fd
long long extr_init()
{
  long long result; // rax

  byte_9F3A8 = geteuid() == 0;
  dword_9F960 += (unsigned char)byte_9F3A8;
  dword_9F95C += (unsigned char)byte_9F3A8;
  mask = umask(0);
  result = (unsigned int)dword_9F960;
  if ( dword_9F960 <= 0 )
  {
    umask(mask);
    dword_9F3B0 = mask;
    return mask;
  }
  else
  {
    dword_9F3B0 = 0;
  }
  return result;
}



// Function: fd_i_chmod @ 0x18387
int fd_i_chmod(int a1, const char *a2, __mode_t a3, int a4)
{
  int *v4; // rax
  int v8; // [rsp+2Ch] [rbp-4h]

  if ( a1 < 0 )
    return fchmodat(newfd, a2, a3, a4);
  v8 = fchmod(a1, a3);
  if ( v8 && (v4 = __errno_location(), !sub_182D5(*v4)) )
    return fchmodat(newfd, a2, a3, a4);
  else
    return v8;
}



// Function: fd_chmod @ 0x183ee
long long fd_chmod(int a1, const char *a2, __mode_t a3, int a4, int a5)
{
  int v5; // eax
  int v6; // eax
  int v7; // eax
  int v12; // [rsp+2Ch] [rbp-4h]

  if ( sub_18387(a1, a2, a3, a4) )
    v5 = *__errno_location();
  else
    v5 = 0;
  v12 = v5;
  if ( v5 == 1 && (a3 & 0x800) != 0 && !(unsigned int)sub_5C835() )
  {
    if ( sub_18387(a1, a2, a3, a4) )
      v6 = *__errno_location();
    else
      v6 = 0;
    v12 = v6;
    sub_5C826();
  }
  if ( a4 && a5 != 50 && !sub_182D5(v12) )
  {
    if ( sub_18387(a1, a2, a3, 0) )
      v7 = *__errno_location();
    else
      v7 = 0;
    v12 = v7;
  }
  if ( !v12 || a5 == 50 && !sub_182D5(v12) )
    return 0;
  *__errno_location() = v12;
  return 0xFFFFFFFFLL;
}



// Function: fd_chown @ 0x184f7
int fd_chown(int a1, const char *a2, __uid_t a3, __gid_t a4, int a5)
{
  int *v5; // rax
  int v10; // [rsp+2Ch] [rbp-4h]

  if ( a1 < 0 )
    return fchownat(newfd, a2, a3, a4, a5);
  v10 = fchown(a1, a3, a4);
  if ( v10 && (v5 = __errno_location(), !sub_182D5(*v5)) )
    return fchownat(newfd, a2, a3, a4, a5);
  else
    return v10;
}



// Function: fd_stat @ 0x1856b
int fd_stat(int a1, const char *a2, struct stat *a3, int a4)
{
  if ( a1 < 0 )
    return fstatat(newfd, a2, a3, a4);
  else
    return fstat(a1, a3);
}



// Function: set_mode @ 0x185b8
unsigned long long set_mode(const char *a1, int a2, int a3, int a4, int a5, int a6, char a7, int a8)
{
  short st_mode; // [rsp+8h] [rbp-B8h]
  int v10; // [rsp+8h] [rbp-B8h]
  __mode_t v13; // [rsp+14h] [rbp-ACh]
  struct stat v14; // [rsp+20h] [rbp-A0h] BYREF
  unsigned long long v15; // [rsp+B8h] [rbp-8h]

  st_mode = a5;
  v15 = __readfsqword(0x28u);
  if ( (a3 & (a2 ^ a5 | ~a6)) != 0 )
  {
    if ( (~((unsigned short)a6 | (unsigned short)a3) & 0xFFF) != 0 )
    {
      if ( sub_1856B(a4, a1, &v14, a8) )
      {
        sub_3EA03(a1);
        return v15 - __readfsqword(0x28u);
      }
      st_mode = v14.st_mode;
    }
    v10 = st_mode & 0xFFF;
    v13 = v10 & ~a3 | a3 & a2;
    if ( v10 != v13 && (unsigned int)sub_183EE(a4, a1, v13, a8, a7) )
      sub_3E0B2(a1, v13);
  }
  return v15 - __readfsqword(0x28u);
}



// Function: check_time @ 0x18719
unsigned long long check_time(long long a1, long long a2, long long a3)
{
  long long v3; // rbx
  char *v4; // rax
  long long v5; // r12
  long long v6; // rbx
  char *v7; // rax
  long long v10; // [rsp+20h] [rbp-60h] BYREF
  long long v11; // [rsp+28h] [rbp-58h]
  long long v12; // [rsp+30h] [rbp-50h]
  long long v13; // [rsp+38h] [rbp-48h]
  char v14[40]; // [rsp+40h] [rbp-40h] BYREF
  unsigned long long v15; // [rsp+68h] [rbp-18h]

  v15 = __readfsqword(0x28u);
  if ( a2 >= 0 )
  {
    if ( (int)sub_5FF90(qword_9F9F0, qword_9F9F8, a2, a3) < 0 )
    {
      sub_521FA(&v10);
      if ( (int)sub_5FF90(v10, v11, a2, a3) < 0 )
      {
        v12 = a2 - v10;
        v13 = a3 - v11;
        if ( a3 - v11 < 0 )
        {
          v13 += 1000000000;
          --v12;
        }
        if ( (dword_9EC84 & 0x8000) != 0 )
        {
          if ( qword_9FDC8 )
            qword_9FDC8();
          v5 = sub_29F2E(v12, v13, v14);
          v6 = sub_27ABC(a2, a3, 1);
          v7 = gettext("%s: time stamp %s is %s s in the future");
          error(0, 0, v7, a1, v6, v5);
        }
      }
    }
  }
  else if ( (dword_9EC84 & 0x8000) != 0 )
  {
    if ( qword_9FDC8 )
      qword_9FDC8();
    v3 = sub_27ABC(a2, a3, 1);
    v4 = gettext("%s: implausibly old time stamp %s");
    error(0, 0, v4, a1, v3);
  }
  return v15 - __readfsqword(0x28u);
}



// Function: set_stat @ 0x18913
unsigned long long set_stat(
        const char *a1,
        long long a2,
        unsigned int a3,
        int a4,
        int a5,
        char a6,
        char a7,
        unsigned int a8)
{
  long long v8; // rdx
  long long v9; // rdx
  int *v10; // rax
  int *v11; // rax
  int v12; // r10d
  __uid_t v18; // [rsp+38h] [rbp-38h]
  __gid_t v19; // [rsp+3Ch] [rbp-34h]
  long long v20; // [rsp+40h] [rbp-30h] BYREF
  long long v21; // [rsp+48h] [rbp-28h]
  long long v22; // [rsp+50h] [rbp-20h]
  long long v23; // [rsp+58h] [rbp-18h]
  unsigned long long v24; // [rsp+68h] [rbp-8h]

  v24 = __readfsqword(0x28u);
  if ( byte_9F99A != 1 && a7 != 1 )
  {
    if ( byte_9F8BE )
    {
      v8 = *(long long *)(a2 + 240);
      v20 = *(long long *)(a2 + 232);
      v21 = v8;
    }
    else
    {
      v21 = 1073741822;
    }
    v9 = *(long long *)(a2 + 256);
    v22 = *(long long *)(a2 + 248);
    v23 = v9;
    if ( (unsigned int)sub_4F4EC(a3, (unsigned int)newfd, a1, &v20, a8) )
    {
      if ( a6 != 50 || (v10 = __errno_location(), sub_182D5(*v10)) )
        sub_3EAD0(a1);
    }
    else
    {
      if ( byte_9F8BE )
        sub_18719((long long)a1, v20, v21);
      sub_18719((long long)a1, v22, v23);
    }
  }
  if ( dword_9F95C > 0 && a7 != 1 )
  {
    v18 = *(int *)(a2 + 116);
    v19 = *(int *)(a2 + 120);
    if ( sub_184F7(a3, a1, v18, v19, a8) )
    {
      if ( a6 != 50 || (v11 = __errno_location(), sub_182D5(*v11)) )
        sub_3E169(a1, v18, v19);
    }
    else if ( (((unsigned char)a4 | (unsigned char)~(char)a5) & 0x49) != 0 )
    {
      a5 &= ~(a4 & 0xC00);
    }
  }
  if ( dword_9F960 > 0 && a7 != 1 )
    v12 = 4095;
  else
    v12 = 511;
  sub_185B8(a1, ~dword_9F3B0 & *(int *)(a2 + 112), v12, a3, a4, a5, a6, a8);
  sub_3D8A1(a2, a1, (unsigned int)a6, 1);
  sub_3D003(a2, a1, (unsigned int)a6);
  sub_3D609(a2, a1, (unsigned int)a6);
  return v24 - __readfsqword(0x28u);
}



// Function: find_direct_ancestor @ 0x18b89
long long find_direct_ancestor(const char *a1)
{
  long long i; // [rsp+18h] [rbp-8h]

  for ( i = qword_9F3B8;
        i
     && (*(char *)(i + 88) == 1
      || strncmp(a1, *(const char **)(i + 160), *(long long *)(i + 152))
      || a1[*(long long *)(i + 152)] != 47
      || (const char *)sub_4DF20(a1) != &a1[*(long long *)(i + 152) + 1]);
        i = *(long long *)i )
  {
    ;
  }
  return i;
}



// Function: mark_after_links @ 0x18c40
unsigned long long mark_after_links(long long *a1)
{
  long long v3[19]; // [rsp+20h] [rbp-A0h] BYREF
  unsigned long long v4; // [rsp+B8h] [rbp-8h]

  v4 = __readfsqword(0x28u);
  do
  {
    *((char *)a1 + 88) = 1;
    if ( (unsigned int)sub_2A9B8(a1[20], v3) )
    {
      sub_3EA03(a1[20]);
    }
    else
    {
      a1[1] = v3[0];
      a1[2] = v3[1];
    }
    a1 = (long long *)*a1;
  }
  while ( a1 && *((char *)a1 + 88) != 1 );
  return v4 - __readfsqword(0x28u);
}



// Function: delay_set_stat @ 0x18d26
unsigned long long delay_set_stat(const char *a1, long long a2, int a3, int a4, int a5, int a6)
{
  long long v6; // rdx
  long long v7; // rdx
  long long i; // [rsp+20h] [rbp-B0h]
  size_t v14; // [rsp+28h] [rbp-A8h]
  struct stat buf; // [rsp+30h] [rbp-A0h] BYREF
  unsigned long long v16; // [rsp+C8h] [rbp-8h]

  v16 = __readfsqword(0x28u);
  v14 = strlen(a1);
  for ( i = qword_9F3B8; i && strcmp(*(const char **)(i + 160), a1); i = *(long long *)i )
    ;
  if ( i )
  {
    if ( *(char *)(i + 80) )
    {
      if ( fstatat(newfd, *(const char **)(i + 160), &buf, *(int *)(i + 84)) )
      {
        sub_3EA03(*(long long *)(i + 160));
      }
      else
      {
        *(long long *)(i + 8) = buf.st_dev;
        *(long long *)(i + 16) = buf.st_ino;
      }
    }
  }
  else
  {
    i = sub_60B5B(168);
    *(long long *)i = qword_9F3B8;
    qword_9F3B8 = i;
    *(long long *)(i + 152) = v14;
    *(long long *)(i + 160) = sub_60CC6(a1);
    *(char *)(i + 88) = 0;
    if ( a2 )
    {
      *(long long *)(i + 8) = *(long long *)(a2 + 88);
      *(long long *)(i + 16) = *(long long *)(a2 + 96);
    }
  }
  *(int *)(i + 24) = a5;
  if ( a2 )
  {
    *(int *)(i + 28) = *(int *)(a2 + 116);
    *(int *)(i + 32) = *(int *)(a2 + 120);
    v6 = *(long long *)(a2 + 240);
    *(long long *)(i + 40) = *(long long *)(a2 + 232);
    *(long long *)(i + 48) = v6;
    v7 = *(long long *)(a2 + 256);
    *(long long *)(i + 56) = *(long long *)(a2 + 248);
    *(long long *)(i + 64) = v7;
  }
  *(int *)(i + 72) = a3;
  *(int *)(i + 76) = a4;
  *(char *)(i + 80) = a2 == 0;
  *(int *)(i + 84) = a6;
  *(int *)(i + 92) = dword_9F600;
  *(long long *)(i + 96) = 0;
  if ( a2 && (sub_29547(i + 96, *(long long *)(a2 + 48)), *(long long *)(a2 + 56)) )
  {
    *(long long *)(i + 104) = sub_60C8E(*(long long *)(a2 + 56), *(long long *)(a2 + 64) + 1LL);
    *(long long *)(i + 112) = *(long long *)(a2 + 64);
  }
  else
  {
    *(long long *)(i + 104) = 0;
    *(long long *)(i + 112) = 0;
  }
  if ( a2 && *(long long *)(a2 + 72) )
  {
    *(long long *)(i + 120) = sub_60C8E(*(long long *)(a2 + 72), *(long long *)(a2 + 80) + 1LL);
    *(long long *)(i + 128) = *(long long *)(a2 + 80);
  }
  else
  {
    *(long long *)(i + 120) = 0;
    *(long long *)(i + 128) = 0;
  }
  if ( a2 )
  {
    sub_1D854(a2, i + 144, i + 136);
  }
  else
  {
    *(long long *)(i + 144) = 0;
    *(long long *)(i + 136) = 0;
  }
  if ( (unsigned char)sub_2A1FE(a1) )
    sub_18C40((long long *)i);
  return v16 - __readfsqword(0x28u);
}



// Function: repair_delayed_set_stat @ 0x191c5
unsigned long long repair_delayed_set_stat(long long a1, long long *a2)
{
  long long v2; // rdx
  long long v3; // rdx
  long long v4; // rbx
  char *v5; // rax
  long long i; // [rsp+18h] [rbp-B8h]
  struct stat buf; // [rsp+20h] [rbp-B0h] BYREF
  unsigned long long v9; // [rsp+B8h] [rbp-18h]

  v9 = __readfsqword(0x28u);
  for ( i = qword_9F3B8; i; i = *(long long *)i )
  {
    if ( fstatat(newfd, *(const char **)(i + 160), &buf, *(int *)(i + 84)) )
    {
      sub_3EA03(*(long long *)(i + 160));
      return v9 - __readfsqword(0x28u);
    }
    if ( buf.st_dev == *a2 && buf.st_ino == a2[1] )
    {
      *(long long *)(i + 8) = qword_9FA78;
      *(long long *)(i + 16) = qword_9FA80;
      *(long long *)(i + 24) = qword_9FA90;
      *(int *)(i + 32) = qword_9FA98;
      v2 = qword_9FB10;
      *(long long *)(i + 40) = qword_9FB08;
      *(long long *)(i + 48) = v2;
      v3 = qword_9FB20;
      *(long long *)(i + 56) = qword_9FB18;
      *(long long *)(i + 64) = v3;
      *(int *)(i + 72) = buf.st_mode;
      *(int *)(i + 76) = -1;
      *(char *)(i + 80) = 0;
      return v9 - __readfsqword(0x28u);
    }
  }
  if ( qword_9FDC8 )
    qword_9FDC8();
  v4 = sub_5E548(a1);
  v5 = gettext("%s: Unexpected inconsistency when making directory");
  error(0, 0, v5, v4);
  dword_9FDD0 = 2;
  return v9 - __readfsqword(0x28u);
}



// Function: free_delayed_set_stat @ 0x193d1
void free_delayed_set_stat(long long a1)
{
  free(*(void **)(a1 + 160));
  sub_1D55D(*(long long *)(a1 + 144), *(long long *)(a1 + 136));
  free(*(void **)(a1 + 96));
  free(*(void **)(a1 + 104));
  free(*(void **)(a1 + 120));
  free((void *)a1);
}



// Function: remove_delayed_set_stat @ 0x19454
long long remove_delayed_set_stat(const char *a1)
{
  long long result; // rax
  long long i; // [rsp+18h] [rbp-18h]
  long long *v3; // [rsp+20h] [rbp-10h]
  long long v4; // [rsp+28h] [rbp-8h]

  v3 = 0;
  result = qword_9F3B8;
  for ( i = qword_9F3B8; i; i = v4 )
  {
    v4 = *(long long *)i;
    if ( *(int *)(i + 92) == dword_9F600 && !strcmp(*(const char **)(i + 160), a1) )
    {
      sub_193D1(i);
      if ( v3 )
      {
        *v3 = v4;
        return (long long)v3;
      }
      else
      {
        qword_9F3B8 = v4;
        return v4;
      }
    }
    v3 = (long long *)i;
    result = v4;
  }
  return result;
}



// Function: fixup_delayed_set_stat @ 0x194f9
size_t fixup_delayed_set_stat(const char *a1, const char *a2)
{
  size_t result; // rax
  long long i; // [rsp+18h] [rbp-8h]

  result = qword_9F3B8;
  for ( i = qword_9F3B8; i; i = *(long long *)i )
  {
    if ( *(int *)(i + 92) == dword_9F600 && !strcmp(*(const char **)(i + 160), a1) )
    {
      free(*(void **)(i + 160));
      *(long long *)(i + 160) = sub_60CC6(a2);
      result = strlen(a2);
      *(long long *)(i + 152) = result;
      return result;
    }
    result = *(long long *)i;
  }
  return result;
}



// Function: make_directories @ 0x195aa
long long make_directories(const char *a1, char *a2)
{
  int v2; // eax
  unsigned int v4; // [rsp+10h] [rbp-C0h]
  unsigned int v5; // [rsp+14h] [rbp-BCh]
  __mode_t mode; // [rsp+18h] [rbp-B8h]
  int v7; // [rsp+1Ch] [rbp-B4h]
  char *i; // [rsp+20h] [rbp-B0h]
  struct stat buf; // [rsp+30h] [rbp-A0h] BYREF
  unsigned long long v10; // [rsp+C8h] [rbp-8h]

  v10 = __readfsqword(0x28u);
  for ( i = (char *)a1; *i; ++i )
  {
    if ( *i == 47
      && i != a1
      && *(i - 1) != 47
      && (*(i - 1) != 46 || i != a1 + 1 && *(i - 2) != 47 && (*(i - 2) != 46 || i != a1 + 2 && *(i - 3) != 47)) )
    {
      *i = 0;
      v5 = ~(short)mask & 0x1FF;
      v2 = byte_9F3A8 ? 0 : 192;
      mode = v5 | v2;
      v4 = mkdirat(newfd, a1, v5 | v2);
      if ( v4 )
      {
        if ( *__errno_location() == 17 )
        {
          v4 = 0;
        }
        else
        {
          v7 = *__errno_location();
          v4 = fstatat(newfd, a1, &buf, 0);
          if ( v4 )
          {
            *__errno_location() = v7;
            sub_3E310(a1);
          }
        }
      }
      else
      {
        sub_18D26(a1, 0, mode & ~dword_9F3B0, 511, v5, 256);
        sub_286AF(a1, (unsigned int)((int)i - (int)a1), v5);
        *a2 = 1;
      }
      *i = 47;
      if ( v4 )
        return v4;
    }
  }
  return 0;
}



// Function: file_newer_p @ 0x19851
long long file_newer_p(long long a1, char *a2, long long a3)
{
  long long v4; // rax
  long long v5; // rdx
  bool v6; // al
  char *v8; // [rsp+10h] [rbp-B0h]
  char v9[152]; // [rsp+20h] [rbp-A0h] BYREF
  unsigned long long v10; // [rsp+B8h] [rbp-8h]

  v8 = a2;
  v10 = __readfsqword(0x28u);
  if ( !a2 )
  {
    if ( (unsigned int)sub_2A9B8(a1, v9) )
    {
      if ( *__errno_location() == 2 )
        return 0;
      sub_3EA2C(a1);
      return 1;
    }
    v8 = v9;
  }
  v6 = 0;
  if ( (*((int *)v8 + 6) & 0xF000) != 0x4000 )
  {
    v4 = sub_5FD9E(v8);
    if ( (int)sub_39E35(*(long long *)(a3 + 248), *(long long *)(a3 + 256), v4, v5) <= 0 )
      return 1;
  }
  return v6;
}



// Function: maybe_recoverable @ 0x19950
long long maybe_recoverable(const char *a1, char a2, char *a3)
{
  long long result; // rax
  char *v4; // rax
  int v6; // [rsp+24h] [rbp-ACh]
  char *v7; // [rsp+28h] [rbp-A8h]
  char v8[152]; // [rsp+30h] [rbp-A0h] BYREF
  unsigned long long v9; // [rsp+C8h] [rbp-8h]

  v9 = __readfsqword(0x28u);
  v6 = *__errno_location();
  v7 = 0;
  if ( *a3 )
    return 0;
  if ( v6 == 95 )
    goto LABEL_14;
  if ( v6 > 95 )
    goto LABEL_31;
  if ( v6 == 40 )
    goto LABEL_14;
  if ( v6 > 40 )
    goto LABEL_31;
  if ( v6 == 31 )
  {
LABEL_14:
    if ( a2 != 1 || dword_9F8D8 != 2 || byte_9F8A0 )
      goto LABEL_31;
    if ( strchr(a1, 47) )
    {
      if ( (unsigned int)sub_2A9B8(a1, v8) )
        goto LABEL_31;
      v7 = v8;
    }
  }
  else
  {
    if ( v6 > 31 )
      goto LABEL_31;
    if ( v6 == 2 )
    {
LABEL_28:
      if ( !(unsigned int)sub_195AA(a1, a3) && *a3 )
        return 1;
      goto LABEL_31;
    }
    if ( v6 != 17 )
    {
LABEL_31:
      *__errno_location() = v6;
      return 0;
    }
  }
  switch ( dword_9F8D8 )
  {
    case 0:
    case 1:
    case 2:
      goto LABEL_26;
    case 4:
      return 0;
    case 5:
      if ( (dword_9EC84 & 0x100000) != 0 )
      {
        if ( qword_9FDC8 )
          qword_9FDC8();
        v4 = gettext("%s: skipping existing file");
        error(0, 0, v4, a1);
      }
      return 2;
    case 6:
      if ( (unsigned char)sub_19851((long long)a1, v7, (long long)&qword_9FA20) )
        goto LABEL_28;
LABEL_26:
      if ( (int)sub_2A36F(a1, 0) <= 0 )
        goto LABEL_28;
      result = 1;
      break;
    default:
      goto LABEL_28;
  }
  return result;
}



// Function: set_xattr @ 0x19bce
long long set_xattr(const char *a1, long long a2, int a3, char a4, int *a5)
{
  int v6; // eax
  char v10; // [rsp+23h] [rbp-Dh] BYREF
  int v11; // [rsp+24h] [rbp-Ch]
  unsigned long long v12; // [rsp+28h] [rbp-8h]

  v12 = __readfsqword(0x28u);
  v10 = 0;
  if ( dword_9F96C <= 0 || !*(long long *)(a2 + 344) )
    return 0;
  v11 = (unsigned short)qword_9FA90 & (unsigned short)~(short)dword_9F3B0 & 0x1FF;
  while ( 1 )
  {
    if ( !mknodat(newfd, a1, a3 ^ v11, 0) )
    {
      sub_3D8A1(a2, a1, (unsigned int)a4, 0);
      *a5 = 1;
      return 0;
    }
    v6 = sub_19950(a1, 0, &v10);
    if ( v6 == 2 )
      break;
    if ( !v6 )
    {
      sub_28891();
      sub_3E38B(a1);
      return 1;
    }
  }
  return 0;
}



// Function: apply_nonancestor_delayed_set_stat @ 0x19ce9
unsigned long long apply_nonancestor_delayed_set_stat(const char *a1, unsigned char a2)
{
  long long v2; // rbx
  char *v3; // rax
  long long v4; // rdx
  long long v5; // rdx
  bool v7; // [rsp+16h] [rbp-27Ah]
  char v8; // [rsp+17h] [rbp-279h]
  __mode_t st_mode; // [rsp+18h] [rbp-278h]
  int v10; // [rsp+1Ch] [rbp-274h]
  size_t v11; // [rsp+20h] [rbp-270h]
  long long v12; // [rsp+28h] [rbp-268h]
  struct stat buf; // [rsp+30h] [rbp-260h] BYREF
  char v14[48]; // [rsp+C0h] [rbp-1D0h] BYREF
  long long v15; // [rsp+F0h] [rbp-1A0h]
  long long v16; // [rsp+F8h] [rbp-198h]
  long long v17; // [rsp+100h] [rbp-190h]
  long long v18; // [rsp+108h] [rbp-188h]
  long long v19; // [rsp+110h] [rbp-180h]
  int v20; // [rsp+130h] [rbp-160h]
  int v21; // [rsp+134h] [rbp-15Ch]
  int v22; // [rsp+138h] [rbp-158h]
  long long v23; // [rsp+1A8h] [rbp-E8h]
  long long v24; // [rsp+1B0h] [rbp-E0h]
  long long v25; // [rsp+1B8h] [rbp-D8h]
  long long v26; // [rsp+1C0h] [rbp-D0h]
  long long v27; // [rsp+218h] [rbp-78h]
  long long v28; // [rsp+220h] [rbp-70h]
  unsigned long long v29; // [rsp+278h] [rbp-18h]

  v29 = __readfsqword(0x28u);
  v11 = strlen(a1);
  v7 = 0;
  while ( qword_9F3B8 )
  {
    v12 = qword_9F3B8;
    v8 = 0;
    st_mode = *(int *)(qword_9F3B8 + 72);
    v10 = *(int *)(qword_9F3B8 + 76);
    v7 = (unsigned char)(v7 | *(char *)(qword_9F3B8 + 88)) != 0;
    if ( a2 < *(char *)(qword_9F3B8 + 88) )
      break;
    if ( v11 > *(long long *)(qword_9F3B8 + 152)
      && a1[*(long long *)(qword_9F3B8 + 152)]
      && (a1[*(long long *)(qword_9F3B8 + 152)] == 47 || a1[*(long long *)(qword_9F3B8 + 152) - 1] == 47)
      && !memcmp(a1, *(const void **)(qword_9F3B8 + 160), *(long long *)(qword_9F3B8 + 152)) )
    {
      break;
    }
    sub_2ADF1(*(unsigned int *)(v12 + 92));
    if ( v7 )
    {
      if ( fstatat(newfd, *(const char **)(v12 + 160), &buf, *(int *)(v12 + 84)) )
      {
        sub_3EA03(*(long long *)(v12 + 160));
        v8 = 1;
      }
      else
      {
        st_mode = buf.st_mode;
        v10 = -1;
        if ( buf.st_dev != *(long long *)(v12 + 8) || buf.st_ino != *(long long *)(v12 + 16) )
        {
          if ( qword_9FDC8 )
            qword_9FDC8();
          v2 = sub_5E548(*(long long *)(v12 + 160));
          v3 = gettext("%s: Directory renamed before its status could be extracted");
          error(0, 0, v3, v2);
          dword_9FDD0 = 2;
          v8 = 1;
        }
      }
    }
    if ( v8 != 1 )
    {
      v20 = *(int *)(v12 + 24);
      v21 = *(int *)(v12 + 28);
      v22 = *(int *)(v12 + 32);
      v4 = *(long long *)(v12 + 48);
      v23 = *(long long *)(v12 + 40);
      v24 = v4;
      v5 = *(long long *)(v12 + 64);
      v25 = *(long long *)(v12 + 56);
      v26 = v5;
      v15 = *(long long *)(v12 + 96);
      v16 = *(long long *)(v12 + 104);
      v17 = *(long long *)(v12 + 112);
      v18 = *(long long *)(v12 + 120);
      v19 = *(long long *)(v12 + 128);
      v28 = *(long long *)(v12 + 144);
      v27 = *(long long *)(v12 + 136);
      sub_18913(
        *(const char **)(v12 + 160),
        (long long)v14,
        0xFFFFFFFF,
        st_mode,
        v10,
        53,
        *(char *)(v12 + 80),
        *(int *)(v12 + 84));
    }
    qword_9F3B8 = *(long long *)v12;
    sub_193D1(v12);
  }
  return v29 - __readfsqword(0x28u);
}



// Function: is_directory_link @ 0x1a10f
bool is_directory_link(const char *a1)
{
  int v1; // eax
  int v3; // [rsp+18h] [rbp-A8h]
  int v4; // [rsp+1Ch] [rbp-A4h]
  struct stat buf; // [rsp+20h] [rbp-A0h] BYREF
  unsigned long long v6; // [rsp+B8h] [rbp-8h]

  v6 = __readfsqword(0x28u);
  v3 = *__errno_location();
  v1 = !fstatat(newfd, a1, &buf, 256)
    && (buf.st_mode & 0xF000) == 0xA000
    && !fstatat(newfd, a1, &buf, 0)
    && (buf.st_mode & 0xF000) == 0x4000;
  v4 = v1;
  *__errno_location() = v3;
  return v4;
}



// Function: safe_dir_mode @ 0x1a1ec
long long safe_dir_mode(long long a1)
{
  int v1; // eax
  int v2; // edx
  int v3; // eax

  if ( dword_9F95C <= 0 && dword_9F960 <= 0 )
    v1 = 511;
  else
    v1 = 448;
  v2 = v1 & *(int *)(a1 + 24);
  if ( byte_9F3A8 )
    v3 = 0;
  else
    v3 = 192;
  return v2 | (unsigned int)v3;
}



// Function: extract_dir @ 0x1a238
long long extract_dir(const char *a1, int a2)
{
  int v3; // eax
  char v4; // [rsp+1Bh] [rbp-B5h] BYREF
  unsigned int v5; // [rsp+1Ch] [rbp-B4h]
  __mode_t st_mode; // [rsp+20h] [rbp-B0h]
  int v7; // [rsp+24h] [rbp-ACh]
  int v8; // [rsp+28h] [rbp-A8h]
  __mode_t mode; // [rsp+2Ch] [rbp-A4h]
  struct stat buf; // [rsp+30h] [rbp-A0h] BYREF
  unsigned long long v11; // [rsp+C8h] [rbp-8h]

  v11 = __readfsqword(0x28u);
  st_mode = 0;
  v7 = 0;
  v8 = 0;
  v4 = 0;
  if ( byte_9F935 && !qword_9FC20 )
  {
    if ( fstatat(newfd, ".", &buf, 0) )
      sub_2B444(".");
    else
      qword_9FC20 = buf.st_dev;
  }
  if ( byte_9F8BE )
  {
    sub_25377(a1);
  }
  else if ( a2 == 68 )
  {
    sub_28891();
  }
  mode = sub_1A1EC((long long)byte_9FA78);
  while ( 1 )
  {
    v5 = mkdirat(newfd, a1, mode);
    if ( !v5 )
    {
      st_mode = mode & ~dword_9F3B0;
      v7 = 511;
      v8 = 256;
      goto LABEL_35;
    }
    if ( *__errno_location() == 17 && (v4 || byte_9F8DC || (unsigned int)dword_9F8D8 <= 2) )
    {
      if ( byte_9F8DC && sub_1A10F(a1) )
        return 0;
      if ( !(unsigned int)sub_2A9B8(a1, &buf) )
      {
        st_mode = buf.st_mode;
        v7 = -1;
        if ( (buf.st_mode & 0xF000) == 0x4000 )
        {
          if ( v4 )
          {
            sub_191C5((long long)a1, &buf);
            return 0;
          }
          if ( dword_9F8D8 == 1 )
          {
            mode = sub_1A1EC((long long)&buf);
            v5 = sub_183EE(-1, a1, mode, 256, 53);
            if ( v5 )
            {
              sub_3E0B2(a1, mode);
            }
            else
            {
              *(struct stat *)byte_9FA78 = buf;
              st_mode = mode & ~dword_9F3B0;
              v7 = 511;
              v8 = 256;
            }
          }
          goto LABEL_35;
        }
      }
      *__errno_location() = 17;
    }
    v3 = sub_19950(a1, 0, &v4);
    if ( v3 > 1 )
      goto LABEL_35;
    if ( !v3 )
      break;
    if ( v3 != 1 )
      goto LABEL_35;
  }
  if ( *__errno_location() != 17 )
  {
    sub_3E310(a1);
    return 1;
  }
LABEL_35:
  if ( !v5 || !dword_9F8D8 || dword_9F8D8 == 2 )
    sub_18D26(a1, (long long)&qword_9FA20, st_mode, v7, *(int *)&byte_9FA78[24], v8);
  return v5;
}



// Function: open_output_file @ 0x1a6b5
long long open_output_file(const char *a1, int a2, unsigned int a3, int a4, __mode_t *a5, int *a6)
{
  int v6; // eax
  char *v7; // rax
  bool v12; // [rsp+33h] [rbp-ADh]
  unsigned int oflag; // [rsp+34h] [rbp-ACh]
  int fd; // [rsp+38h] [rbp-A8h]
  int v15; // [rsp+3Ch] [rbp-A4h]
  struct stat buf; // [rsp+40h] [rbp-A0h] BYREF
  unsigned long long v17; // [rsp+D8h] [rbp-8h]

  v17 = __readfsqword(0x28u);
  v12 = dword_9F8D8 == 2;
  if ( dword_9F8D8 == 2 )
  {
    if ( byte_9F8A0 )
      v6 = 527169;
    else
      v6 = 658241;
  }
  else
  {
    v6 = 526785;
  }
  oflag = v6;
  if ( a4 )
    oflag = v6 & 0xFFFFFF7F;
  if ( a2 == 55 && !dword_9F3C8 )
  {
    dword_9F3C8 = 1;
    if ( (dword_9EC84 & 8) != 0 )
    {
      if ( qword_9FDC8 )
        qword_9FDC8();
      v7 = gettext("Extracting contiguous files as regular files");
      error(0, 0, v7);
    }
  }
  fd = openat(newfd, a1, oflag, a3);
  if ( fd >= 0 )
  {
    if ( v12 )
    {
      if ( fstat(fd, &buf) )
      {
        v15 = *__errno_location();
        close(fd);
        *__errno_location() = v15;
        return 0xFFFFFFFFLL;
      }
      if ( (buf.st_mode & 0xF000) != 0x8000 )
      {
        close(fd);
        *__errno_location() = 17;
        return 0xFFFFFFFFLL;
      }
      *a5 = buf.st_mode;
      *a6 = -1;
    }
    else
    {
      *a5 = a3 & ~dword_9F3B0;
      *a6 = 511;
    }
  }
  return (unsigned int)fd;
}



// Function: extract_file @ 0x1a8d0
long long extract_file(const char *a1, char a2)
{
  short v2; // ax
  unsigned int v3; // eax
  char *v5; // rax
  unsigned int v6; // r8d
  char v7; // [rsp+1Bh] [rbp-45h] BYREF
  int v8; // [rsp+1Ch] [rbp-44h] BYREF
  int v9; // [rsp+20h] [rbp-40h] BYREF
  int fd; // [rsp+24h] [rbp-3Ch]
  unsigned int v11; // [rsp+28h] [rbp-38h]
  unsigned int v12; // [rsp+2Ch] [rbp-34h]
  int v13; // [rsp+30h] [rbp-30h]
  int v14; // [rsp+34h] [rbp-2Ch]
  unsigned long long v15; // [rsp+38h] [rbp-28h] BYREF
  unsigned long long v16; // [rsp+40h] [rbp-20h]
  void *v17; // [rsp+48h] [rbp-18h]
  long long v18; // [rsp+50h] [rbp-10h]
  unsigned long long v19; // [rsp+58h] [rbp-8h]

  v19 = __readfsqword(0x28u);
  v7 = 0;
  if ( dword_9F95C <= 0 )
    v2 = -1;
  else
    v2 = -64;
  v11 = *(short *)&byte_9FA78[24] & (unsigned short)v2 & 0x1FF;
  if ( dword_9F95C <= 0 )
    v3 = 0;
  else
    v3 = v11 & 0x3F;
  v12 = v3;
  v8 = 0;
  v9 = 0;
  if ( byte_9F998 )
  {
    fd = 1;
  }
  else if ( qword_9F9A0 )
  {
    fd = sub_34323(a1, 102, &qword_9FA20);
    if ( fd < 0 )
    {
      sub_28891();
      return 0;
    }
  }
  else
  {
    LODWORD(v15) = 0;
    if ( (unsigned int)sub_19BCE(a1, (long long)&qword_9FA20, v12, a2, &v15) )
      return 1;
    while ( 1 )
    {
      fd = sub_1A6B5(a1, a2, v11, v15, (__mode_t *)&v8, &v9);
      if ( fd >= 0 )
        break;
      v13 = sub_19950(a1, 1, &v7);
      if ( v13 != 1 )
      {
        sub_28891();
        if ( v13 == 2 )
          return 0;
        sub_3E38B(a1);
        return 1;
      }
    }
  }
  sub_AD2B(&qword_9FA20);
  if ( byte_9FB40 )
  {
    sub_302A4((unsigned int)fd, &qword_9FA20, &v15);
  }
  else
  {
    v15 = *(long long *)&byte_9FA78[48];
    while ( (long long)v15 > 0 )
    {
      sub_AD86(v15);
      v17 = sub_BB4B();
      if ( !v17 )
      {
        if ( qword_9FDC8 )
          qword_9FDC8();
        v5 = gettext("Unexpected EOF in archive");
        error(0, 0, v5);
        dword_9FDD0 = 2;
        break;
      }
      v16 = (unsigned long long)sub_BBF5((long long)v17);
      if ( v16 > v15 )
        v16 = v15;
      *__errno_location() = 0;
      v18 = sub_2AA95((unsigned int)fd, v17, v16);
      v15 -= v16;
      sub_BBA7((unsigned long long)v17 + v16 - 1);
      if ( v18 != v16 )
      {
        if ( !qword_9F9A0 )
          sub_3EB4B(a1, v18, v16);
        break;
      }
    }
  }
  sub_287C1(v15);
  sub_AD66();
  if ( byte_9F998 )
    return 0;
  if ( !qword_9F9A0 )
  {
    if ( dword_9F8D8 == 2 )
      v6 = 0;
    else
      v6 = 256;
    sub_18913(a1, (long long)&qword_9FA20, fd, v8, v9, a2, 0, v6);
  }
  v14 = close(fd);
  if ( v14 < 0 )
    sub_3E1FF(a1);
  if ( qword_9F9A0 )
    sub_343EA();
  return (unsigned int)v14;
}



// Function: find_delayed_link_source @ 0x1ac61
long long *find_delayed_link_source(const char *a1)
{
  long long *i; // [rsp+18h] [rbp-A8h]
  struct stat buf; // [rsp+20h] [rbp-A0h] BYREF
  unsigned long long v4; // [rsp+B8h] [rbp-8h]

  v4 = __readfsqword(0x28u);
  if ( !qword_9F3C0 )
    return 0;
  if ( fstatat(newfd, a1, &buf, 256) )
  {
    if ( *__errno_location() != 2 )
      sub_3EA03(a1);
    return 0;
  }
  else
  {
    for ( i = (long long *)qword_9F3C0; i && (i[1] != buf.st_dev || i[2] != buf.st_ino); i = (long long *)*i )
      ;
    return i;
  }
}



// Function: create_placeholder_file @ 0x1ad5b
long long create_placeholder_file(const char *a1, char a2, char *a3, long long *a4)
{
  int i; // eax
  int v6; // eax
  size_t v7; // rax
  long long v8; // rdx
  long long v9; // rdx
  long long v10; // rdx
  size_t v11; // rax
  int fd; // [rsp+2Ch] [rbp-C4h]
  long long v15; // [rsp+30h] [rbp-C0h]
  long long *v16; // [rsp+38h] [rbp-B8h]
  struct stat buf; // [rsp+40h] [rbp-B0h] BYREF
  unsigned long long v18; // [rsp+D8h] [rbp-18h]

  v18 = __readfsqword(0x28u);
  for ( i = openat(newfd, a1, 193, 0); ; i = openat(newfd, a1, 193, 0) )
  {
    fd = i;
    if ( i >= 0 )
      break;
    if ( *__errno_location() == 17 && sub_1AC61(a1) )
      return 0;
    v6 = sub_19950(a1, 0, a3);
    if ( v6 == 2 )
      return 0;
    if ( !v6 )
    {
      sub_3E38B(a1);
      return 0xFFFFFFFFLL;
    }
  }
  if ( fstat(i, &buf) )
  {
    sub_3EA03(a1);
    close(fd);
    return 0xFFFFFFFFLL;
  }
  if ( close(fd) )
  {
    sub_3E1FF(a1);
    return 0xFFFFFFFFLL;
  }
  v7 = strlen(from);
  v15 = sub_60B5B(v7 + 161);
  if ( a4 )
  {
    *(long long *)v15 = *a4;
    *a4 = v15;
  }
  else
  {
    *(long long *)v15 = qword_9F3C0;
    qword_9F3C0 = v15;
  }
  *(long long *)(v15 + 8) = buf.st_dev;
  *(long long *)(v15 + 16) = buf.st_ino;
  *(long long *)(v15 + 24) = sub_5FDB8(&buf);
  *(long long *)(v15 + 32) = v8;
  *(char *)(v15 + 40) = a2;
  if ( a2 )
  {
    *(int *)(v15 + 44) = *(int *)&byte_9FA78[24];
    *(int *)(v15 + 48) = *(int *)&byte_9FA78[28];
    *(int *)(v15 + 52) = *(int *)&byte_9FA78[32];
    v9 = qword_9FB10;
    *(long long *)(v15 + 56) = qword_9FB08;
    *(long long *)(v15 + 64) = v9;
    v10 = qword_9FB20;
    *(long long *)(v15 + 72) = qword_9FB18;
    *(long long *)(v15 + 80) = v10;
  }
  *(int *)(v15 + 88) = dword_9F600;
  v11 = strlen(a1);
  *(long long *)(v15 + 96) = sub_60B5B(v11 + 9);
  **(long long **)(v15 + 96) = 0;
  strcpy((char *)(*(long long *)(v15 + 96) + 8LL), a1);
  *(long long *)(v15 + 104) = 0;
  sub_29547(v15 + 104, qword_9FA50);
  *(long long *)(v15 + 112) = 0;
  *(long long *)(v15 + 120) = 0;
  *(long long *)(v15 + 128) = 0;
  *(long long *)(v15 + 136) = 0;
  sub_1D854(&qword_9FA20, v15 + 152, v15 + 144);
  strcpy((char *)(v15 + 160), from);
  v16 = (long long *)sub_18B89(a1);
  if ( v16 )
    sub_18C40(v16);
  return 0;
}



// Function: extract_link @ 0x1b161
long long extract_link(const char *a1)
{
  long long v2; // rax
  long long v3; // rdx
  size_t v4; // rax
  char v5; // [rsp+13h] [rbp-15Dh] BYREF
  int v6; // [rsp+14h] [rbp-15Ch]
  int v7; // [rsp+18h] [rbp-158h]
  int v8; // [rsp+1Ch] [rbp-154h]
  long long v9; // [rsp+20h] [rbp-150h]
  char *from; // [rsp+28h] [rbp-148h]
  long long *v11; // [rsp+30h] [rbp-140h]
  long long v12; // [rsp+38h] [rbp-138h]
  struct stat buf; // [rsp+40h] [rbp-130h] BYREF
  struct stat v14; // [rsp+D0h] [rbp-A0h] BYREF
  unsigned long long v15; // [rsp+168h] [rbp-8h]

  v15 = __readfsqword(0x28u);
  v5 = 0;
  from = ::from;
  if ( byte_9F878 != 1 && (unsigned char)sub_2EFDC(from) )
    return sub_1AD5B(a1, 0, &v5, 0);
  v11 = sub_1AC61(from);
  if ( v11 )
    return sub_1AD5B(a1, 0, &v5, v11);
  while ( 1 )
  {
    v6 = linkat(newfd, from, newfd, a1, 0);
    v7 = *__errno_location();
    if ( !v6 )
      break;
    if ( v7 == 17 && !strcmp(from, a1)
      || !fstatat(newfd, from, &buf, 256)
      && !fstatat(newfd, a1, &v14, 256)
      && buf.st_dev == v14.st_dev
      && buf.st_ino == v14.st_ino )
    {
      return 0;
    }
    *__errno_location() = v7;
    v8 = sub_19950(a1, 0, &v5);
    if ( v8 != 1 )
    {
      if ( v8 == 2 )
        return 0;
      if ( byte_9F8BE == 1 && *__errno_location() == 17 )
        return 0;
      sub_3E277(from, a1);
      return 1;
    }
  }
  v9 = qword_9F3C0;
  if ( qword_9F3C0 && !fstatat(newfd, from, &buf, 256) )
  {
    while ( v9 )
    {
      if ( *(int *)(v9 + 88) == dword_9F600
        && *(long long *)(v9 + 8) == buf.st_dev
        && *(long long *)(v9 + 16) == buf.st_ino )
      {
        v2 = sub_5FDB8(&buf);
        if ( !(unsigned int)sub_5FF90(*(long long *)(v9 + 24), *(long long *)(v9 + 32), v2, v3) )
        {
          v4 = strlen(a1);
          v12 = sub_60B5B(v4 + 9);
          strcpy((char *)(v12 + 8), a1);
          *(long long *)v12 = *(long long *)(v9 + 96);
          *(long long *)(v9 + 96) = v12;
          return 0;
        }
      }
      v9 = *(long long *)v9;
    }
  }
  return 0;
}



// Function: extract_symlink @ 0x1b508
long long extract_symlink(const char *a1)
{
  int v2; // eax
  char v3; // [rsp+17h] [rbp-9h] BYREF
  unsigned long long v4; // [rsp+18h] [rbp-8h]

  v4 = __readfsqword(0x28u);
  v3 = 0;
  if ( byte_9F878 != 1 && (*from == 47 || (unsigned char)sub_2EFDC(from)) )
    return sub_1AD5B(a1, 1, &v3, 0);
  while ( symlinkat(from, newfd, a1) )
  {
    v2 = sub_19950(a1, 0, &v3);
    if ( v2 == 2 )
      return 0;
    if ( !v2 )
    {
      sub_3E944(from, a1);
      return 0xFFFFFFFFLL;
    }
  }
  sub_18913(a1, (long long)&qword_9FA20, 0xFFFFFFFF, 0, 0, 50, 0, 0x100u);
  return 0;
}



// Function: extract_node @ 0x1b63a
long long extract_node(const char *a1, char a2)
{
  short v2; // ax
  int v3; // eax
  char v5; // [rsp+13h] [rbp-Dh] BYREF
  __mode_t mode; // [rsp+14h] [rbp-Ch]
  unsigned long long v7; // [rsp+18h] [rbp-8h]

  v7 = __readfsqword(0x28u);
  v5 = 0;
  if ( dword_9F95C <= 0 )
    v2 = -1;
  else
    v2 = -64;
  mode = *(short *)&byte_9FA78[24] & (unsigned short)v2 & 0x61FF;
  while ( mknodat(newfd, a1, mode, *(__dev_t *)&byte_9FA78[40]) )
  {
    v3 = sub_19950(a1, 0, &v5);
    if ( v3 == 2 )
      return 0;
    if ( !v3 )
    {
      sub_3E362(a1);
      return 0xFFFFFFFFLL;
    }
  }
  sub_18913(a1, (long long)&qword_9FA20, 0xFFFFFFFF, mode & ~dword_9F3B0, 511, a2, 0, 0x100u);
  return 0;
}



// Function: extract_fifo @ 0x1b745
long long extract_fifo(const char *a1, char a2)
{
  short v2; // ax
  int v3; // eax
  char v5; // [rsp+13h] [rbp-Dh] BYREF
  __mode_t mode; // [rsp+14h] [rbp-Ch]
  unsigned long long v7; // [rsp+18h] [rbp-8h]

  v7 = __readfsqword(0x28u);
  v5 = 0;
  if ( dword_9F95C <= 0 )
    v2 = -1;
  else
    v2 = -64;
  mode = *(short *)&byte_9FA78[24] & (unsigned short)v2 & 0x1FF;
  while ( mkfifoat(newfd, a1, mode) )
  {
    v3 = sub_19950(a1, 0, &v5);
    if ( v3 == 2 )
      return 0;
    if ( !v3 )
    {
      sub_3E339(a1);
      return 0xFFFFFFFFLL;
    }
  }
  sub_18913(a1, (long long)&qword_9FA20, 0xFFFFFFFF, mode & ~dword_9F3B0, 511, a2, 0, 0x100u);
  return 0;
}



// Function: prepare_to_extract @ 0x1b84c
long long prepare_to_extract(long long a1, unsigned int a2, long long (**a3)(const char *a1, char a2))
{
  int v3; // eax
  int v4; // eax
  long long v6; // rbx
  char *v7; // rax
  char *v8; // rax
  long long v9; // rbx
  char *v10; // rax
  long long v11; // rbx
  char *v12; // rax
  long long (*v14)(const char *, char); // [rsp+28h] [rbp-18h]

  switch ( a2 )
  {
    case 0u:
    case 0x30u:
    case 0x37u:
      if ( byte_9FA30 )
        v14 = (long long (*)(const char *, char))sub_1A238;
      else
        v14 = sub_1A8D0;
      goto LABEL_25;
    case 0x31u:
      v14 = (long long (*)(const char *, char))sub_1B161;
      goto LABEL_25;
    case 0x32u:
      v14 = (long long (*)(const char *, char))sub_1B508;
      goto LABEL_25;
    case 0x33u:
      v3 = *(int *)&byte_9FA78[24];
      BYTE1(v3) = byte_9FA78[25] | 0x20;
      *(int *)&byte_9FA78[24] = v3;
      v14 = sub_1B63A;
      goto LABEL_25;
    case 0x34u:
      v4 = *(int *)&byte_9FA78[24];
      BYTE1(v4) = byte_9FA78[25] | 0x60;
      *(int *)&byte_9FA78[24] = v4;
      v14 = sub_1B63A;
      goto LABEL_25;
    case 0x35u:
    case 0x44u:
      v14 = (long long (*)(const char *, char))sub_1A238;
      if ( byte_9FBA8 )
        byte_9FC31 = 1;
      goto LABEL_25;
    case 0x36u:
      v14 = sub_1B745;
      goto LABEL_25;
    case 0x4Bu:
    case 0x4Cu:
      if ( qword_9FDC8 )
        qword_9FDC8();
      v8 = gettext("Unexpected long name header");
      error(0, 0, v8);
      dword_9FDD0 = 2;
      return 0;
    case 0x4Du:
      if ( qword_9FDC8 )
        qword_9FDC8();
      v6 = sub_5E548(file);
      v7 = gettext("%s: Cannot extract -- file is continued from another volume");
      error(0, 0, v7, v6);
      dword_9FDD0 = 2;
      return 0;
    case 0x53u:
      v14 = sub_1A8D0;
      goto LABEL_25;
    case 0x56u:
      return 0;
    default:
      if ( (dword_9EC84 & 0x10000) != 0 )
      {
        if ( qword_9FDC8 )
          qword_9FDC8();
        v9 = sub_5E548(a1);
        v10 = gettext("%s: Unknown file type '%c', extracted as normal file");
        error(0, 0, v10, v9, a2);
      }
      v14 = sub_1A8D0;
LABEL_25:
      if ( byte_9F998 || qword_9F9A0 )
      {
        if ( v14 != sub_1A8D0 )
          return 0;
        goto LABEL_42;
      }
      if ( dword_9F8D8 == 3 )
      {
        if ( !(unsigned int)sub_2A36F(a1, (unsigned char)byte_9F94C)
          && *__errno_location()
          && *__errno_location() != 2 )
        {
          sub_3EAA7(a1);
          return 0;
        }
        goto LABEL_42;
      }
      if ( dword_9F8D8 != 6 || !(unsigned char)sub_19851(a1, 0, (long long)&qword_9FA20) )
      {
LABEL_42:
        *a3 = v14;
        return 1;
      }
      if ( (dword_9EC84 & 0x800) != 0 )
      {
        if ( qword_9FDC8 )
          qword_9FDC8();
        v11 = sub_5E84E(a1);
        v12 = gettext("Current %s is newer or same age");
        error(0, 0, v12, v11);
      }
      return 0;
  }
}



// Function: extract_archive @ 0x1bbcb
unsigned long long extract_archive()
{
  bool v0; // al
  long long v1; // rbx
  char *v2; // rax
  long long v3; // rbx
  char *v4; // rax
  char v5; // al
  bool v7; // [rsp+6h] [rbp-2Ah]
  char v8; // [rsp+7h] [rbp-29h]
  unsigned int v9; // [rsp+8h] [rbp-28h]
  int errnum; // [rsp+Ch] [rbp-24h]
  long long (*v11)(const char *, char); // [rsp+10h] [rbp-20h] BYREF
  unsigned long long v12; // [rsp+18h] [rbp-18h]

  v12 = __readfsqword(0x28u);
  qword_9F388 = sub_1C2EE;
  sub_BBA7(qword_9F460);
  v0 = byte_9F878 != 1 && (unsigned char)sub_2EFDC(qword_9FA20);
  v7 = v0;
  if ( v0 )
  {
    if ( qword_9FDC8 )
      qword_9FDC8();
    v1 = sub_5E548(qword_9FA20);
    v2 = gettext("%s: Member name contains '..'");
    error(0, 0, v2, v1);
    dword_9FDD0 = 2;
  }
  if ( *file && !v7 && (!byte_9F8C8 || (unsigned int)sub_34AF4("extract", file)) )
  {
    if ( dword_9F9AC )
      sub_2864C(&qword_9FA20, qword_9F460, -1);
    if ( byte_9FC31 != 1 )
    {
      v9 = dword_9F600;
      sub_19CE9(file, 0);
      sub_2ADF1(v9);
    }
    if ( byte_9F884 && (unsigned char)sub_2A571(file, 0) != 1 )
    {
      errnum = *__errno_location();
      if ( qword_9FDC8 )
        qword_9FDC8();
      v3 = sub_5E548(file);
      v4 = gettext("%s: Was unable to backup this file");
      error(0, errnum, v4, v3);
      dword_9FDD0 = 2;
      sub_28891();
    }
    else
    {
      if ( (unsigned char)sub_301DE(&qword_9FA20) )
        v5 = 83;
      else
        v5 = *(char *)(qword_9F460 + 156);
      v8 = v5;
      if ( (unsigned char)sub_1B84C((long long)file, v5, &v11) )
      {
        if ( !(unsigned int)v11(file, v8) )
          return v12 - __readfsqword(0x28u);
      }
      else
      {
        sub_28891();
      }
      if ( byte_9F884 )
        sub_2A874();
    }
  }
  else
  {
    sub_28891();
  }
  return v12 - __readfsqword(0x28u);
}



// Function: apply_delayed_links @ 0x1be65
unsigned long long apply_delayed_links()
{
  long long v0; // rax
  long long v1; // rdx
  long long v2; // rdx
  long long v3; // rdx
  unsigned int *i; // [rsp+0h] [rbp-290h]
  char *ptr; // [rsp+8h] [rbp-288h]
  long long *ptra; // [rsp+8h] [rbp-288h]
  char *from; // [rsp+10h] [rbp-280h]
  unsigned int *v9; // [rsp+18h] [rbp-278h]
  long long *v10; // [rsp+20h] [rbp-270h]
  char *file; // [rsp+28h] [rbp-268h]
  struct stat buf; // [rsp+30h] [rbp-260h] BYREF
  char v13[48]; // [rsp+C0h] [rbp-1D0h] BYREF
  long long v14; // [rsp+F0h] [rbp-1A0h]
  long long v15; // [rsp+F8h] [rbp-198h]
  long long v16; // [rsp+100h] [rbp-190h]
  long long v17; // [rsp+108h] [rbp-188h]
  long long v18; // [rsp+110h] [rbp-180h]
  unsigned int v19; // [rsp+130h] [rbp-160h]
  unsigned int v20; // [rsp+134h] [rbp-15Ch]
  unsigned int v21; // [rsp+138h] [rbp-158h]
  long long v22; // [rsp+1A8h] [rbp-E8h]
  long long v23; // [rsp+1B0h] [rbp-E0h]
  long long v24; // [rsp+1B8h] [rbp-D8h]
  long long v25; // [rsp+1C0h] [rbp-D0h]
  long long v26; // [rsp+218h] [rbp-78h]
  long long v27; // [rsp+220h] [rbp-70h]
  unsigned long long v28; // [rsp+278h] [rbp-18h]

  v28 = __readfsqword(0x28u);
  for ( i = (unsigned int *)qword_9F3C0; i; i = v9 )
  {
    from = 0;
    sub_2ADF1(i[22]);
    for ( ptr = (char *)*((long long *)i + 12); ptr; ptr = *(char **)ptr )
    {
      file = ptr + 8;
      if ( !fstatat(newfd, ptr + 8, &buf, 256) && buf.st_dev == *((long long *)i + 1) && buf.st_ino == *((long long *)i + 2) )
      {
        v0 = sub_5FDB8(&buf);
        if ( !(unsigned int)sub_5FF90(v0, v1, *((long long *)i + 3), *((long long *)i + 4)) )
        {
          if ( unlinkat(newfd, file, 0) )
          {
            sub_3EAA7(file);
          }
          else if ( !from || linkat(newfd, from, newfd, file, 0) )
          {
            if ( *((char *)i + 40) != 1 )
            {
              if ( linkat(newfd, (const char *)i + 160, newfd, file, 0) )
                sub_3E277(i + 40, file);
            }
            else if ( symlinkat((const char *)i + 160, newfd, file) )
            {
              sub_3E944(i + 40, file);
            }
            else
            {
              v19 = i[11];
              v20 = i[12];
              v21 = i[13];
              v2 = *((long long *)i + 8);
              v22 = *((long long *)i + 7);
              v23 = v2;
              v3 = *((long long *)i + 10);
              v24 = *((long long *)i + 9);
              v25 = v3;
              v14 = *((long long *)i + 13);
              v15 = *((long long *)i + 14);
              v16 = *((long long *)i + 15);
              v17 = *((long long *)i + 16);
              v18 = *((long long *)i + 17);
              v27 = *((long long *)i + 19);
              v26 = *((long long *)i + 18);
              sub_18913(file, (long long)v13, 0xFFFFFFFF, 0, 0, 50, 0, 0x100u);
              from = ptr + 8;
            }
          }
        }
      }
    }
    for ( ptra = (long long *)*((long long *)i + 12); ptra; ptra = v10 )
    {
      v10 = (long long *)*ptra;
      free(ptra);
    }
    sub_1D55D(*((long long *)i + 19), *((long long *)i + 18));
    free(*((void **)i + 13));
    v9 = *(unsigned int **)i;
    free(i);
  }
  qword_9F3C0 = 0;
  return v28 - __readfsqword(0x28u);
}



// Function: extract_finish @ 0x1c2ee
unsigned long long extract_finish()
{
  sub_19CE9((const char *)&unk_81623, 0);
  sub_1BE65();
  return sub_19CE9((const char *)&unk_81623, 1u);
}



// Function: rename_directory @ 0x1c326
long long rename_directory(const char *a1, const char *a2)
{
  long long v3; // r12
  long long v4; // rbx
  char *v5; // rax
  char v6; // [rsp+13h] [rbp-1Dh] BYREF
  int errnum; // [rsp+14h] [rbp-1Ch]
  unsigned long long v8; // [rsp+18h] [rbp-18h]

  v8 = __readfsqword(0x28u);
  if ( !renameat(newfd, a1, newfd, a2) )
  {
    sub_194F9(a1, a2);
    return 1;
  }
  errnum = *__errno_location();
  if ( errnum == 2 && !(unsigned int)sub_195AA(a2, &v6) )
  {
    if ( !renameat(newfd, a1, newfd, a2) )
      return 1;
    errnum = *__errno_location();
  }
  if ( qword_9FDC8 )
    qword_9FDC8();
  v3 = sub_5E821(1, a2);
  v4 = sub_5E821(0, a1);
  v5 = gettext("Cannot rename %s to %s");
  error(0, errnum, v5, v4, v3);
  dword_9FDD0 = 2;
  return 0;
}



// Function: x_obstack_grow @ 0x1c47d
long long *x_obstack_grow(long long *a1, const void *a2, size_t a3)
{
  long long v5; // [rsp+28h] [rbp-18h]

  v5 = *a1;
  if ( *(long long *)(*a1 + 32LL) - *(long long *)(*a1 + 24LL) < a3 )
    obstack_newchunk(v5, a3);
  memcpy(*(void **)(v5 + 24), a2, a3);
  *(long long *)(v5 + 24) += a3;
  a1[1] += a3;
  return a1;
}



// Function: x_obstack_1grow @ 0x1c52b
long long *x_obstack_1grow(long long *a1, char a2)
{
  char *v2; // rax
  long long v4; // [rsp+10h] [rbp-10h]

  v4 = *a1;
  if ( *(long long *)(*a1 + 32LL) == *(long long *)(*a1 + 24LL) )
    obstack_newchunk(v4, 1);
  v2 = *(char **)(v4 + 24);
  *(long long *)(v4 + 24) = v2 + 1;
  *v2 = a2;
  ++a1[1];
  return a1;
}



// Function: x_obstack_blank @ 0x1c5b0
long long *x_obstack_blank(long long *a1, unsigned long long a2)
{
  long long v3; // [rsp+18h] [rbp-18h]

  v3 = *a1;
  if ( *(long long *)(*a1 + 32LL) - *(long long *)(*a1 + 24LL) < a2 )
    obstack_newchunk(v3, a2);
  *(long long *)(v3 + 24) += a2;
  a1[1] += a2;
  return a1;
}



// Function: xheader_keyword_deleted_p @ 0x1c63f
long long xheader_keyword_deleted_p(const char *a1)
{
  long long i; // [rsp+18h] [rbp-8h]

  for ( i = qword_9F3D8; i; i = *(long long *)i )
  {
    if ( !fnmatch(*(const char **)(i + 8), a1, 0) )
      return 1;
  }
  return 0;
}



// Function: xheader_keyword_override_p @ 0x1c69c
long long xheader_keyword_override_p(const char *a1)
{
  long long i; // [rsp+18h] [rbp-8h]

  for ( i = qword_9F3E8; i; i = *(long long *)i )
  {
    if ( !strcmp(*(const char **)(i + 8), a1) )
      return 1;
  }
  return 0;
}



// Function: xheader_list_append @ 0x1c6f4
long long *xheader_list_append(long long *a1, long long a2, long long a3)
{
  long long v3; // rdx
  long long *v6; // [rsp+28h] [rbp-8h]

  v6 = (long long *)sub_60B5B(24);
  v6[1] = sub_60CC6(a2);
  if ( a3 )
    v3 = sub_60CC6(a3);
  else
    v3 = 0;
  v6[2] = v3;
  *v6 = *a1;
  *a1 = v6;
  return a1;
}



// Function: xheader_list_destroy @ 0x1c772
void **xheader_list_destroy(void **a1)
{
  void **result; // rax
  void **ptr; // [rsp+10h] [rbp-10h]
  void **v3; // [rsp+18h] [rbp-8h]

  if ( a1 )
  {
    for ( ptr = (void **)*a1; ptr; ptr = v3 )
    {
      v3 = (void **)*ptr;
      free(ptr[1]);
      free(ptr[2]);
      free(ptr);
    }
    *a1 = 0;
    return a1;
  }
  return result;
}



// Function: xheader_set_single_keyword @ 0x1c7ea
void xheader_set_single_keyword(long long a1)
{
  char *v1; // rax

  if ( qword_9FDC8 )
    qword_9FDC8();
  v1 = gettext("Keyword %s is unknown or not yet implemented");
  error(0, 0, v1, a1);
  sub_3814C(2);
}



// Function: assign_time_option @ 0x1c846
unsigned long long assign_time_option(long long a1, long long *a2, long long a3)
{
  long long v3; // rdx
  char *v4; // rax
  char *v7; // [rsp+28h] [rbp-28h] BYREF
  long long v8; // [rsp+30h] [rbp-20h]
  long long v9; // [rsp+38h] [rbp-18h]
  unsigned long long v10; // [rsp+48h] [rbp-8h]

  v10 = __readfsqword(0x28u);
  v8 = sub_29FF7(a3, &v7, 0);
  v9 = v3;
  if ( (unsigned char)sub_294F0(v8, v3) != 1 || *v7 )
  {
    if ( qword_9FDC8 )
      qword_9FDC8();
    v4 = gettext("Time stamp is out of allowed range");
    error(0, 0, v4);
    dword_9FDD0 = 2;
  }
  else
  {
    *a2 = v8;
    sub_29547(a1, a3);
  }
  return v10 - __readfsqword(0x28u);
}



// Function: xheader_set_keyword_equal @ 0x1c92e
long long *xheader_set_keyword_equal(const char *a1, const char *a2)
{
  long long v2; // rbx
  char *v3; // rax
  long long v4; // rbx
  char *v5; // rax
  char *v7; // rax
  char v8; // [rsp+17h] [rbp-19h]
  const char *v9; // [rsp+18h] [rbp-18h]
  char *i; // [rsp+18h] [rbp-18h]

  v8 = 1;
  v9 = a2;
  if ( a2 == a1 )
  {
    if ( qword_9FDC8 )
      qword_9FDC8();
    v2 = sub_5E84E(a1);
    v3 = gettext("Malformed pax option: %s");
    error(0, 0, v3, v2);
    sub_3814C(2);
  }
  if ( *(a2 - 1) == 58 )
  {
    v9 = a2 - 1;
    v8 = 0;
  }
  while ( v9 > a1 && ((*__ctype_b_loc())[*(unsigned char *)v9] & 0x2000) != 0 )
    --v9;
  *v9 = 0;
  for ( i = a2 + 1; *i && ((*__ctype_b_loc())[(unsigned char)*i] & 0x2000) != 0; ++i )
    ;
  if ( !strcmp(a1, "delete") )
  {
    if ( (unsigned char)sub_1D9D3(i) )
    {
      if ( qword_9FDC8 )
        qword_9FDC8();
      v4 = sub_5E84E(i);
      v5 = gettext("Pattern %s cannot be used");
      error(0, 0, v5, v4);
      sub_3814C(2);
    }
    return sub_1C6F4(&qword_9F3D8, (long long)i, 0);
  }
  else if ( !strcmp(a1, "exthdr.name") )
  {
    return (long long *)sub_29547(&qword_9F3F8, i);
  }
  else if ( !strcmp(a1, "globexthdr.name") )
  {
    return (long long *)sub_29547(&qword_9F410, i);
  }
  else if ( !strcmp(a1, "exthdr.mtime") )
  {
    return (long long *)sub_1C846((long long)&qword_9F400, &qword_9F408, (long long)i);
  }
  else if ( !strcmp(a1, "globexthdr.mtime") )
  {
    return (long long *)sub_1C846((long long)&qword_9F418, &qword_9F420, (long long)i);
  }
  else
  {
    if ( (unsigned char)sub_1DA4B(a1) )
    {
      if ( qword_9FDC8 )
        qword_9FDC8();
      v7 = gettext("Keyword %s cannot be overridden");
      error(0, 0, v7, a1);
      sub_3814C(2);
    }
    if ( v8 )
      return sub_1C6F4(&qword_9F3E0, (long long)a1, (long long)i);
    else
      return sub_1C6F4(&qword_9F3E8, (long long)a1, (long long)i);
  }
}



// Function: xheader_set_option @ 0x1cc78
char *xheader_set_option(char *a1)
{
  char *result; // rax
  char *i; // [rsp+10h] [rbp-10h]
  char *v3; // [rsp+18h] [rbp-8h]

  result = strtok(a1, ",");
  for ( i = result; i; i = result )
  {
    v3 = strchr(i, 61);
    if ( !v3 )
      sub_1C7EA((long long)i);
    sub_1C92E(i, v3);
    result = strtok(0, ",");
  }
  return result;
}



// Function: xheader_format_name @ 0x1cd02
unsigned long long xheader_format_name(long long *a1, char *a2, long long a3)
{
  int v3; // eax
  __pid_t v4; // eax
  int v5; // eax
  char *v6; // rax
  char *v7; // rdx
  char *v8; // rax
  char *v9; // rdx
  char *v10; // rax
  char *v11; // rdx
  char *v12; // rax
  long long v15; // [rsp+28h] [rbp-88h]
  char *dest; // [rsp+30h] [rbp-80h]
  char *i; // [rsp+38h] [rbp-78h]
  char *v18; // [rsp+38h] [rbp-78h]
  void *ptr; // [rsp+40h] [rbp-70h]
  char *s; // [rsp+48h] [rbp-68h]
  char *src; // [rsp+50h] [rbp-60h]
  char *v22; // [rsp+58h] [rbp-58h]
  char *v23; // [rsp+60h] [rbp-50h]
  unsigned long long v24; // [rsp+68h] [rbp-48h]
  char v25[20]; // [rsp+70h] [rbp-40h] BYREF
  int v26; // [rsp+84h] [rbp-2Ch] BYREF
  char v27[20]; // [rsp+90h] [rbp-20h] BYREF
  char v28[12]; // [rsp+A4h] [rbp-Ch] BYREF

  *(long long *)&v28[4] = __readfsqword(0x28u);
  ptr = 0;
  s = 0;
  src = 0;
  v22 = 0;
  v23 = 0;
  v15 = 0;
  for ( i = a2; *i; ++i )
  {
    if ( *i == 37 && i[1] )
    {
      v3 = (char)*++i;
      if ( v3 == 112 )
      {
        v4 = getpid();
        v22 = (char *)sub_54E5A(v4, v25);
        v15 += (char *)&v26 - v22;
        continue;
      }
      if ( v3 > 112 )
        goto LABEL_22;
      if ( v3 == 110 )
      {
        v23 = (char *)sub_54E5A(a3, v27);
        v15 += v28 - v23;
        continue;
      }
      if ( v3 > 110 )
      {
LABEL_22:
        v15 += 2;
        continue;
      }
      if ( v3 == 102 )
      {
        if ( a1 )
        {
          src = (char *)sub_4DF20(*a1);
          v15 += strlen(src);
        }
      }
      else
      {
        if ( v3 > 102 )
          goto LABEL_22;
        if ( v3 == 37 )
        {
          ++v15;
          continue;
        }
        if ( v3 != 100 )
          goto LABEL_22;
        if ( a1 )
        {
          if ( !ptr )
            ptr = (void *)sub_4E10C(*a1);
          s = (char *)sub_3EDD9(ptr, 0, (unsigned char)byte_9F878);
          v15 += strlen(s);
        }
      }
    }
    else
    {
      ++v15;
    }
  }
  v24 = sub_60B5B(v15 + 1);
  dest = (char *)v24;
  v18 = a2;
  while ( *v18 )
  {
    if ( *v18 == 37 )
    {
      v5 = v18[1];
      if ( v5 == 112 )
      {
        dest = stpcpy(dest, v22);
        v18 += 2;
      }
      else if ( v5 > 112 )
      {
LABEL_48:
        v7 = v18++;
        v8 = dest++;
        *v8 = *v7;
        if ( *v18 )
        {
          v9 = v18++;
          v10 = dest++;
          *v10 = *v9;
        }
      }
      else if ( v5 == 110 )
      {
        dest = stpcpy(dest, v23);
        v18 += 2;
      }
      else
      {
        if ( v5 > 110 )
          goto LABEL_48;
        if ( v5 == 102 )
        {
          if ( src )
            dest = stpcpy(dest, src);
          v18 += 2;
        }
        else
        {
          if ( v5 > 102 )
            goto LABEL_48;
          if ( v5 == 37 )
          {
            v6 = dest++;
            *v6 = *v18;
            v18 += 2;
          }
          else
          {
            if ( v5 != 100 )
              goto LABEL_48;
            if ( s )
              dest = stpcpy(dest, s);
            v18 += 2;
          }
        }
      }
    }
    else
    {
      v11 = v18++;
      v12 = dest++;
      *v12 = *v11;
    }
  }
  free(ptr);
  while ( (unsigned long long)dest > v24 && *(dest - 1) == 47 )
    --dest;
  *dest = 0;
  return v24;
}



// Function: xheader_xhdr_name @ 0x1d123
unsigned long long xheader_xhdr_name(long long *a1)
{
  if ( !qword_9F3F8 )
    sub_29547(&qword_9F3F8, off_9C240[(unsigned char)byte_9F9C8]);
  return sub_1CD02(a1, (char *)qword_9F3F8, 0);
}



// Function: xheader_ghdr_name @ 0x1d18d
unsigned long long xheader_ghdr_name()
{
  size_t v0; // rbx
  const char *s; // [rsp+8h] [rbp-28h]
  char *src; // [rsp+10h] [rbp-20h]
  size_t v4; // [rsp+18h] [rbp-18h]

  if ( !qword_9F410 )
  {
    src = off_9C240[(unsigned char)byte_9F9C8 + 2];
    s = getenv("TMPDIR");
    if ( !s )
      s = "/tmp";
    v0 = strlen(s);
    v4 = v0 + strlen(src) + 1;
    qword_9F410 = (char *)sub_60B5B(v4);
    strcpy(qword_9F410, s);
    strcat(qword_9F410, src);
  }
  return sub_1CD02(0, qword_9F410, qword_9F3D0 + 1);
}



// Function: xheader_write @ 0x1d280
long long xheader_write(char a1, long long a2, long long a3, long long a4)
{
  char *v4; // rax
  long long result; // rax
  unsigned long long v7; // [rsp+20h] [rbp-20h]
  char *src; // [rsp+28h] [rbp-18h]
  size_t n; // [rsp+30h] [rbp-10h]
  char *dest; // [rsp+38h] [rbp-8h]

  v7 = *(long long *)(a4 + 8);
  if ( a1 != 103 )
  {
    if ( a1 == 120 && qword_9F400 )
    {
      v4 = sub_12E0F(a2, v7, qword_9F408);
      goto LABEL_9;
    }
LABEL_8:
    v4 = sub_12E0F(a2, v7, a3);
    goto LABEL_9;
  }
  if ( !qword_9F418 )
    goto LABEL_8;
  v4 = sub_12E0F(a2, v7, qword_9F420);
LABEL_9:
  v4[156] = a1;
  sub_1418C((unsigned long long)v4);
  src = *(char **)(a4 + 16);
  do
  {
    dest = (char *)sub_BB4B();
    n = 512;
    if ( v7 < 0x200 )
      n = v7;
    memcpy(dest, src, n);
    if ( n <= 0x1FF )
      memset(&dest[n], 0, 512 - n);
    src += n;
    v7 -= n;
    sub_BBA7((unsigned long long)dest);
  }
  while ( v7 );
  result = sub_1E785(a4);
  if ( a1 == 103 )
    return ++qword_9F3D0;
  return result;
}



// Function: xheader_write_global @ 0x1d3e6
void xheader_write_global(long long *a1)
{
  long long *i; // [rsp+10h] [rbp-10h]
  void *ptr; // [rsp+18h] [rbp-8h]

  if ( qword_9F3E0 )
  {
    sub_1E173(a1);
    for ( i = (long long *)qword_9F3E0; i; i = (long long *)*i )
      sub_1ECD7(i[2], i[1], a1);
  }
  if ( *a1 )
  {
    sub_1E672(a1);
    ptr = (void *)sub_1D18D();
    sub_1D280(103, (long long)ptr, qword_9F9E0, (long long)a1);
    free(ptr);
  }
}



// Function: xheader_forbid_global @ 0x1d498
long long xheader_forbid_global()
{
  long long result; // rax
  char *v1; // rax

  result = qword_9F3E0;
  if ( qword_9F3E0 )
  {
    if ( qword_9FDC8 )
      qword_9FDC8();
    v1 = gettext("can't update global extended header record");
    error(0, 0, v1);
    sub_3814C(2);
  }
  return result;
}



// Function: xheader_xattr_init @ 0x1d4f4
long long *xheader_xattr_init(long long *a1)
{
  a1[44] = 0;
  a1[43] = 0;
  a1[7] = 0;
  a1[8] = 0;
  a1[9] = 0;
  a1[10] = 0;
  a1[6] = 0;
  return a1;
}



// Function: xheader_xattr_free @ 0x1d55d
void xheader_xattr_free(void **a1, unsigned long long a2)
{
  unsigned long long i; // [rsp+18h] [rbp-8h]

  for ( i = 0; i < a2; ++i )
  {
    free(a1[3 * i]);
    free(a1[3 * i + 1]);
  }
  free(a1);
}



// Function: xheader_xattr__add @ 0x1d5e6
long long xheader_xattr__add(long long *a1, long long *a2, long long a3, long long a4, long long a5)
{
  long long v5; // rax
  long long *v6; // rbx
  long long v7; // rbx
  long long v12; // [rsp+38h] [rbp-18h]

  v5 = (*a2)++;
  v12 = v5;
  *a1 = sub_60B8D(*a1, 24LL * *a2);
  v6 = (long long *)(*a1 + 24 * v12);
  *v6 = sub_60CC6(a3);
  v7 = *a1 + 24 * v12;
  *(long long *)(v7 + 8) = sub_60C8E(a4, a5 + 1);
  *(long long *)(*a1 + 24 * v12 + 16) = a5;
  return a5;
}



// Function: xattr_decode_keyword @ 0x1d6da
long long xattr_decode_keyword(char *a1)
{
  long long result; // rax
  unsigned char *i; // [rsp+8h] [rbp-10h]
  char *v3; // [rsp+10h] [rbp-8h]

  v3 = a1;
  for ( i = a1; ; ++i )
  {
    while ( 1 )
    {
      while ( 1 )
      {
        if ( *i != 37 )
          goto LABEL_9;
        if ( i[1] != 51 || i[2] != 68 )
          break;
        *v3 = 61;
        i += 3;
        ++v3;
      }
      if ( i[1] != 50 || i[2] != 53 )
        break;
      *v3 = 37;
      i += 3;
      ++v3;
    }
LABEL_9:
    *v3 = *i;
    result = *i;
    if ( !(char)result )
      break;
    ++v3;
  }
  return result;
}



// Function: xheader_xattr_add @ 0x1d78e
void xheader_xattr_add(long long a1, const char *a2, long long a3, long long a4)
{
  size_t v4; // rax
  size_t v7; // [rsp+28h] [rbp-18h]
  char *ptr; // [rsp+30h] [rbp-10h]
  char *dest; // [rsp+38h] [rbp-8h]

  v7 = strlen(a2);
  v4 = strlen("SCHILY.xattr.");
  ptr = (char *)sub_60B5B(v7 + v4 + 1);
  dest = stpcpy(ptr, "SCHILY.xattr.");
  stpcpy(dest, a2);
  sub_1D5E6((long long *)(a1 + 352), (long long *)(a1 + 344), (long long)ptr, a3, a4);
  free(ptr);
}



// Function: xheader_xattr_copy @ 0x1d854
unsigned long long xheader_xattr_copy(long long a1, long long *a2, long long *a3)
{
  unsigned long long result; // rax
  unsigned long long v5; // [rsp+20h] [rbp-20h]

  v5 = 0;
  *a2 = 0;
  *a3 = 0;
  while ( 1 )
  {
    result = *(long long *)(a1 + 344);
    if ( v5 >= result )
      break;
    sub_1D5E6(
      a2,
      a3,
      *(long long *)(*(long long *)(a1 + 352) + 24 * v5),
      *(long long *)(*(long long *)(a1 + 352) + 24 * v5 + 8),
      *(long long *)(*(long long *)(a1 + 352) + 24 * v5 + 16));
    ++v5;
  }
  return result;
}



// Function: locate_handler @ 0x1d940
char **locate_handler(const char *a1)
{
  size_t v1; // rax
  char **i; // [rsp+18h] [rbp-8h]

  for ( i = &off_9A260; *i; i += 4 )
  {
    if ( *((char *)i + 28) )
    {
      v1 = strlen(*i);
      if ( !strncmp(*i, a1, v1) )
        return i;
    }
    else if ( !strcmp(*i, a1) )
    {
      return i;
    }
  }
  return 0;
}



// Function: xheader_protected_pattern_p @ 0x1d9d3
long long xheader_protected_pattern_p(const char *a1)
{
  char **i; // [rsp+18h] [rbp-8h]

  for ( i = &off_9A260; *i; i += 4 )
  {
    if ( *((char *)i + 28) != 1 && ((int)i[3] & 1) != 0 && !fnmatch(a1, *i, 0) )
      return 1;
  }
  return 0;
}



// Function: xheader_protected_keyword_p @ 0x1da4b
long long xheader_protected_keyword_p(const char *a1)
{
  char **i; // [rsp+18h] [rbp-8h]

  for ( i = &off_9A260; *i; i += 4 )
  {
    if ( *((char *)i + 28) != 1 && ((int)i[3] & 1) != 0 && !strcmp(*i, a1) )
      return 1;
  }
  return 0;
}



// Function: decode_record @ 0x1dabe
long long decode_record(
        long long a1,
        char **a2,
        void (*a3)(long long, char *, char *, unsigned long long),
        long long a4)
{
  char *v4; // rax
  char *v6; // rax
  char *v7; // rax
  char *v8; // rax
  char *v9; // rax
  char *v10; // rax
  unsigned int v13; // [rsp+2Ch] [rbp-44h]
  char *endptr; // [rsp+30h] [rbp-40h] BYREF
  char *nptr; // [rsp+38h] [rbp-38h]
  char *v16; // [rsp+40h] [rbp-30h]
  uintmax_t v17; // [rsp+48h] [rbp-28h]
  uintmax_t v18; // [rsp+50h] [rbp-20h]
  unsigned long long v19; // [rsp+58h] [rbp-18h]
  char *v20; // [rsp+60h] [rbp-10h]
  unsigned long long v21; // [rsp+68h] [rbp-8h]

  v21 = __readfsqword(0x28u);
  v16 = *a2;
  nptr = v16;
  v17 = *(long long *)(a1 + 16) + *(long long *)(a1 + 8) - (long long)v16;
  while ( *nptr == 32 || *nptr == 9 )
    ++nptr;
  if ( (unsigned int)(*nptr - 48) <= 9 )
  {
    v18 = strtoumax(nptr, &endptr, 10);
    if ( v17 >= v18 )
    {
      v19 = (unsigned long long)&v16[v18];
      for ( nptr = endptr; *nptr == 32 || *nptr == 9; ++nptr )
        ;
      if ( nptr == endptr )
      {
        if ( qword_9FDC8 )
          qword_9FDC8();
        v7 = gettext("Malformed extended header: missing blank after length");
        error(0, 0, v7);
        dword_9FDD0 = 2;
        return 0;
      }
      else
      {
        v20 = nptr;
        nptr = strchr(nptr, 61);
        if ( nptr && (unsigned long long)nptr < v19 )
        {
          if ( *(char *)(v19 - 1) == 10 )
          {
            v10 = (char *)(v19 - 1);
            *v10 = 0;
            *nptr = *v10;
            a3(a4, v20, nptr + 1, v19 - (long long)nptr - 2);
            *nptr = 61;
            *(char *)(v19 - 1) = 10;
            *a2 = (char *)v19;
            return 1;
          }
          else
          {
            if ( qword_9FDC8 )
              qword_9FDC8();
            v9 = gettext("Malformed extended header: missing newline");
            error(0, 0, v9);
            dword_9FDD0 = 2;
            return 0;
          }
        }
        else
        {
          if ( qword_9FDC8 )
            qword_9FDC8();
          v8 = gettext("Malformed extended header: missing equal sign");
          error(0, 0, v8);
          dword_9FDD0 = 2;
          return 0;
        }
      }
    }
    else
    {
      v13 = (int)endptr - (int)nptr;
      if ( qword_9FDC8 )
        qword_9FDC8();
      v6 = gettext("Extended header length %*s is out of range");
      error(0, 0, v6, v13, nptr);
      dword_9FDD0 = 2;
      return 0;
    }
  }
  else
  {
    if ( *nptr )
    {
      if ( qword_9FDC8 )
        qword_9FDC8();
      v4 = gettext("Malformed extended header: missing length");
      error(0, 0, v4);
      dword_9FDD0 = 2;
    }
    return 0;
  }
}



// Function: run_override_list @ 0x1de07
void run_override_list(long long **a1, long long a2)
{
  char *v2; // rbx
  size_t v3; // rax
  char **v5; // [rsp+18h] [rbp-18h]

  while ( a1 )
  {
    v5 = sub_1D940((const char *)a1[1]);
    if ( v5 )
    {
      v2 = v5[2];
      v3 = strlen((const char *)a1[2]);
      ((void (*)(long long, char *, long long *, size_t))v2)(a2, *v5, a1[2], v3);
    }
    a1 = (long long **)*a1;
  }
}



// Function: decx @ 0x1de86
void decx(long long a1, const char *a2, long long a3, long long a4)
{
  char *v4; // rax
  char **v7; // [rsp+28h] [rbp-8h]

  if ( !(unsigned char)sub_1C63F(a2) && !(unsigned char)sub_1C69C(a2) )
  {
    v7 = sub_1D940(a2);
    if ( v7 )
    {
      ((void (*)(long long, const char *, long long, long long))v7[2])(a1, a2, a3, a4);
    }
    else if ( (dword_9EC84 & 0x20000) != 0 )
    {
      if ( qword_9FDC8 )
        qword_9FDC8();
      v4 = gettext("Ignoring unknown extended header keyword '%s'");
      error(0, 0, v4, a2);
    }
  }
}



// Function: xheader_decode @ 0x1df5f
unsigned long long xheader_decode(long long a1)
{
  char *v2; // [rsp+10h] [rbp-10h] BYREF
  unsigned long long v3; // [rsp+18h] [rbp-8h]

  v3 = __readfsqword(0x28u);
  sub_1DE07((long long **)qword_9F3E0, a1);
  sub_1DE07((long long **)qword_9F3F0, a1);
  if ( *(long long *)(a1 + 368) )
  {
    v2 = (char *)(*(long long *)(a1 + 376) + 512LL);
    while ( (unsigned char)sub_1DABE(
                               a1 + 360,
                               &v2,
                               (void (*)(long long, char *, char *, unsigned long long))sub_1DE86,
                               a1) )
      ;
  }
  sub_1DE07((long long **)qword_9F3E8, a1);
  *(long long *)(a1 + 280) = *(long long *)(a1 + 136);
  if ( *(char *)(a1 + 336) )
    *(long long *)(a1 + 136) = *(long long *)(a1 + 328);
  return v3 - __readfsqword(0x28u);
}



// Function: decg @ 0x1e063
long long *decg(long long *a1, const char *a2, long long a3, long long a4)
{
  char **v7; // [rsp+28h] [rbp-8h]

  v7 = sub_1D940(a2);
  if ( v7 && ((int)v7[3] & 2) != 0 )
    return (long long *)((long long (*)(long long *, const char *, long long, long long))v7[2])(a1, a2, a3, a4);
  else
    return sub_1C6F4(a1, (long long)a2, a3);
}



// Function: xheader_decode_global @ 0x1e0e7
unsigned long long xheader_decode_global(long long a1)
{
  char *v2; // [rsp+10h] [rbp-10h] BYREF
  unsigned long long v3; // [rsp+18h] [rbp-8h]

  v3 = __readfsqword(0x28u);
  if ( *(long long *)(a1 + 8) )
  {
    v2 = (char *)(*(long long *)(a1 + 16) + 512LL);
    sub_1C772((void **)&qword_9F3F0);
    while ( (unsigned char)sub_1DABE(
                               a1,
                               &v2,
                               (void (*)(long long, char *, char *, unsigned long long))sub_1E063,
                               (long long)&qword_9F3F0) )
      ;
  }
  return v3 - __readfsqword(0x28u);
}



// Function: xheader_init @ 0x1e173
long long xheader_init(long long *a1)
{
  long long result; // rax

  result = *a1;
  if ( !*a1 )
  {
    *a1 = sub_60B5B(88);
    return obstack_begin(*a1, 0, 0, sub_60B5B, &free);
  }
  return result;
}



// Function: xheader_store @ 0x1e1d3
long long xheader_store(const char *a1, long long a2, long long a3)
{
  long long result; // rax
  long long v5; // [rsp+28h] [rbp-8h]

  result = *(long long *)(a2 + 376);
  if ( !result )
  {
    result = (long long)sub_1D940(a1);
    v5 = result;
    if ( result )
    {
      result = *(long long *)(result + 8);
      if ( result )
      {
        result = sub_1C63F(a1);
        if ( !(char)result )
        {
          sub_1E173((long long *)(a2 + 360));
          result = (unsigned int)sub_1C69C(a1) ^ 1;
          if ( (char)result )
            return (*(long long (**)(long long, const char *, long long, long long))(v5 + 8))(a2, a1, a2 + 360, a3);
        }
      }
    }
  }
  return result;
}



// Function: xheader_read @ 0x1e28e
size_t xheader_read(long long a1, void *a2, long long a3)
{
  char *v3; // rax
  size_t result; // rax
  unsigned long long v5; // [rsp+8h] [rbp-28h]
  long long v6; // [rsp+8h] [rbp-28h]
  long long v8; // [rsp+20h] [rbp-10h]
  size_t n; // [rsp+28h] [rbp-8h]

  v5 = a3;
  v8 = 0;
  if ( a3 < 0 )
    v5 = 0;
  if ( v5 > 0xFFFFFFFFFFFFFDFELL )
    sub_1783C();
  v6 = v5 + 512;
  *(long long *)(a1 + 8) = v6;
  *(long long *)(a1 + 16) = sub_60B5B(v6 + 1);
  *(char *)(*(long long *)(a1 + 16) + v6) = 0;
  do
  {
    n = v6;
    if ( (unsigned long long)v6 > 0x200 )
      n = 512;
    if ( !a2 )
    {
      if ( qword_9FDC8 )
        qword_9FDC8();
      v3 = gettext("Unexpected EOF in archive");
      error(0, 0, v3);
      sub_177F4();
    }
    memcpy((void *)(v8 + *(long long *)(a1 + 16)), a2, n);
    sub_BBA7((unsigned long long)a2);
    a2 = sub_BB4B();
    v8 += n;
    result = v6 - n;
    v6 -= n;
  }
  while ( v6 > 0 );
  return result;
}



// Function: xattr_encode_keyword @ 0x1e3ca
long long xattr_encode_keyword(char *a1)
{
  char v3; // [rsp+17h] [rbp-9h]
  long long v4; // [rsp+18h] [rbp-8h]

  if ( qword_9F428 )
  {
    *(char *)qword_9F428 = 0;
  }
  else
  {
    qword_9F430 = 256;
    qword_9F428 = sub_60B5B(256);
  }
  v4 = 0;
  while ( *a1 )
  {
    v3 = *a1;
    if ( v4 + 2 >= (unsigned long long)qword_9F430 )
      qword_9F428 = sub_60BD8(qword_9F428, &qword_9F430);
    if ( v3 == 37 )
    {
      strcpy((char *)(qword_9F428 + v4), "%25");
      v4 += 2;
    }
    else if ( v3 == 61 )
    {
      strcpy((char *)(qword_9F428 + v4), "%3D");
      v4 += 2;
    }
    else
    {
      *(char *)(v4 + qword_9F428) = v3;
    }
    ++v4;
    ++a1;
  }
  *(char *)(qword_9F428 + v4) = 0;
  return qword_9F428;
}



// Function: xheader_print_n @ 0x1e4ff
unsigned long long xheader_print_n(long long *a1, char *a2, const void *a3, size_t a4)
{
  char *s; // [rsp+10h] [rbp-60h]
  char *v8; // [rsp+28h] [rbp-48h]
  size_t v9; // [rsp+30h] [rbp-40h]
  char *v10; // [rsp+40h] [rbp-30h]
  long long v11; // [rsp+48h] [rbp-28h]
  char v12[20]; // [rsp+50h] [rbp-20h] BYREF
  char v13[4]; // [rsp+64h] [rbp-Ch] BYREF
  unsigned long long v14; // [rsp+68h] [rbp-8h]

  v14 = __readfsqword(0x28u);
  v8 = 0;
  s = (char *)sub_1E3CA(a2);
  v9 = strlen(s);
  do
  {
    v10 = v8;
    v11 = sub_54E5A(&v8[v9 + 3 + a4], v12);
    v8 = &v13[-v11];
  }
  while ( &v13[-v11] != v10 );
  sub_1C47D(a1, (const void *)v11, (size_t)v8);
  sub_1C52B(a1, 32);
  sub_1C47D(a1, s, v9);
  sub_1C52B(a1, 61);
  sub_1C47D(a1, a3, a4);
  sub_1C52B(a1, 10);
  return v14 - __readfsqword(0x28u);
}



// Function: xheader_print @ 0x1e634
unsigned long long xheader_print(long long *a1, char *a2, const char *a3)
{
  size_t v3; // rax

  v3 = strlen(a3);
  return sub_1E4FF(a1, a2, a3, v3);
}



// Function: xheader_finish @ 0x1e672
long long *xheader_finish(long long *a1)
{
  long long *i; // [rsp+18h] [rbp-18h]
  long long v3; // [rsp+20h] [rbp-10h]
  long long v4; // [rsp+28h] [rbp-8h]

  for ( i = (long long *)qword_9F3E8; i; i = (long long *)*i )
    sub_1ECD7(i[2], i[1], a1);
  v3 = *a1;
  v4 = *(long long *)(*a1 + 16LL);
  if ( v4 == *(long long *)(*a1 + 24LL) )
    *(char *)(v3 + 80) |= 2u;
  *(long long *)(v3 + 24) = (*(long long *)(v3 + 48) + *(long long *)(v3 + 24)) & ~*(long long *)(v3 + 48);
  if ( *(long long *)(v3 + 24) - *(long long *)(v3 + 8) > *(long long *)(v3 + 32) - *(long long *)(v3 + 8) )
    *(long long *)(v3 + 24) = *(long long *)(v3 + 32);
  *(long long *)(v3 + 16) = *(long long *)(v3 + 24);
  a1[2] = v4;
  return a1;
}



// Function: xheader_destroy @ 0x1e785
long long xheader_destroy(long long a1)
{
  if ( *(long long *)a1 )
  {
    obstack_free(*(struct obstack **)a1, 0);
    free(*(void **)a1);
    *(long long *)a1 = 0;
  }
  else
  {
    free(*(void **)(a1 + 16));
  }
  *(long long *)(a1 + 16) = 0;
  *(long long *)(a1 + 8) = 0;
  return a1;
}



// Function: xheader_string_begin @ 0x1e848
long long xheader_string_begin(long long a1)
{
  *(long long *)(a1 + 24) = 0;
  return a1;
}



// Function: xheader_string_add @ 0x1e863
long long *xheader_string_add(long long *a1, const char *a2)
{
  long long *result; // rax
  long long v3; // rbx
  size_t v4; // rax

  result = (long long *)a1[2];
  if ( !result )
  {
    sub_1E173(a1);
    v3 = a1[3];
    a1[3] = v3 + strlen(a2);
    v4 = strlen(a2);
    return sub_1C47D(a1, a2, v4);
  }
  return result;
}



// Function: xheader_string_end @ 0x1e8dc
long long xheader_string_end(long long *a1, const char *a2)
{
  char *v3; // rax
  char *v4; // [rsp+10h] [rbp-70h]
  size_t v5; // [rsp+18h] [rbp-68h]
  char *v6; // [rsp+20h] [rbp-60h]
  unsigned long long v7; // [rsp+20h] [rbp-60h]
  char *v8; // [rsp+28h] [rbp-58h]
  void *src; // [rsp+38h] [rbp-48h]
  char v10[20]; // [rsp+60h] [rbp-20h] BYREF
  char v11[12]; // [rsp+74h] [rbp-Ch] BYREF

  *(long long *)&v11[4] = __readfsqword(0x28u);
  v4 = 0;
  if ( a1[2] )
    return 0;
  sub_1E173(a1);
  v5 = strlen(a2) + a1[3] + 3;
  do
  {
    v6 = v4;
    v8 = (char *)sub_54E5A(&v4[v5], v10);
    v4 = (char *)(v11 - v8);
  }
  while ( (char *)(v11 - v8) != v6 );
  v7 = (unsigned long long)&v4[strlen(a2) + 2];
  sub_1C5B0(a1, v7);
  sub_1C52B(a1, 10);
  src = (void *)(*(long long *)(*a1 + 24) + -a1[3] - v7 - 1);
  memmove((void *)(*(long long *)(*a1 + 24) + -a1[3] - 1), src, a1[3]);
  v3 = stpcpy((char *)src, v8);
  *v3 = 32;
  *stpcpy(v3 + 1, a2) = 61;
  return 1;
}



// Function: out_of_range_header @ 0x1ec02
unsigned long long out_of_range_header(long long a1, long long a2, long long a3, long long a4)
{
  char *v4; // rax
  long long v7; // [rsp+20h] [rbp-50h]
  long long v8; // [rsp+28h] [rbp-48h]
  char v9[32]; // [rsp+30h] [rbp-40h] BYREF
  char v10[24]; // [rsp+50h] [rbp-20h] BYREF
  unsigned long long v11; // [rsp+68h] [rbp-8h]

  v11 = __readfsqword(0x28u);
  v7 = sub_54BDE(a3, v9);
  v8 = sub_54E5A(a4, v10);
  if ( qword_9FDC8 )
    qword_9FDC8();
  v4 = gettext("Extended header %s=%s is out of range %s..%s");
  error(0, 0, v4, a1, a2, v7, v8);
  dword_9FDD0 = 2;
  return v11 - __readfsqword(0x28u);
}



// Function: code_string @ 0x1ecd7
unsigned long long code_string(long long a1, char *a2, long long *a3)
{
  void *ptr; // [rsp+20h] [rbp-10h] BYREF
  unsigned long long v6; // [rsp+28h] [rbp-8h]

  v6 = __readfsqword(0x28u);
  if ( (unsigned char)sub_3C8A0(1, a1, &ptr) != 1 )
    ptr = (void *)sub_60CC6(a1);
  sub_1E634(a3, a2, (const char *)ptr);
  free(ptr);
  return v6 - __readfsqword(0x28u);
}



// Function: decode_string @ 0x1ed64
long long decode_string(void **a1, long long a2)
{
  long long result; // rax

  if ( *a1 )
  {
    free(*a1);
    *a1 = 0;
  }
  result = (unsigned int)sub_3C8A0(0, a2, a1) ^ 1;
  if ( (char)result )
    return sub_29547(a1, a2);
  return result;
}



// Function: code_time @ 0x1edd0
unsigned long long code_time(long long a1, long long a2, char *a3, long long *a4)
{
  const char *v4; // rax
  char v8[40]; // [rsp+20h] [rbp-30h] BYREF
  unsigned long long v9; // [rsp+48h] [rbp-8h]

  v9 = __readfsqword(0x28u);
  v4 = (const char *)sub_29F2E(a1, a2, v8);
  sub_1E634(a4, a3, v4);
  return v9 - __readfsqword(0x28u);
}



// Function: decode_time @ 0x1ee4e
long long decode_time(long long *a1, unsigned long long a2, long long a3)
{
  long long v3; // rdx
  char *v4; // rax
  long long v6; // rdx
  char *v8; // [rsp+28h] [rbp-28h] BYREF
  long long v9; // [rsp+30h] [rbp-20h]
  long long v10; // [rsp+38h] [rbp-18h]
  unsigned long long v11; // [rsp+48h] [rbp-8h]

  v11 = __readfsqword(0x28u);
  v9 = sub_29FF7(a2, &v8, 1);
  v10 = v3;
  if ( (unsigned char)sub_294F0(v9, v3) != 1 )
  {
    if ( a2 >= (unsigned long long)v8 || *v8 )
    {
      if ( qword_9FDC8 )
        qword_9FDC8();
      v4 = gettext("Malformed extended header: invalid %s=%s");
      error(0, 0, v4, a3, a2);
      dword_9FDD0 = 2;
    }
    else
    {
      sub_1EC02(a3, a2, 0x8000000000000000LL, 0x7FFFFFFFFFFFFFFFLL);
    }
    return 0;
  }
  else
  {
    v6 = v10;
    *a1 = v9;
    a1[1] = v6;
    return 1;
  }
}



// Function: code_signed_num @ 0x1ef79
unsigned long long code_signed_num(long long a1, char *a2, long long a3, long long a4, long long *a5)
{
  const char *v5; // rax
  char v8[24]; // [rsp+30h] [rbp-20h] BYREF
  unsigned long long v9; // [rsp+48h] [rbp-8h]

  v9 = __readfsqword(0x28u);
  v5 = (const char *)sub_29C96(a1, a3, a4, v8);
  sub_1E634(a5, a2, v5);
  return v9 - __readfsqword(0x28u);
}



// Function: code_num @ 0x1efed
unsigned long long code_num(long long a1, char *a2, long long *a3)
{
  return sub_1EF79(a1, a2, 0, -1, a3);
}



// Function: decode_signed_num @ 0x1f02b
long long decode_signed_num(long long *a1, long long a2, long long a3, long long a4, long long a5)
{
  char *v5; // rax
  char *v10; // [rsp+38h] [rbp-18h] BYREF
  long long v11; // [rsp+40h] [rbp-10h]
  unsigned long long v12; // [rsp+48h] [rbp-8h]

  v12 = __readfsqword(0x28u);
  v11 = sub_29CFC(a2, &v10, a3, a4);
  if ( *__errno_location() == 22 || *v10 )
  {
    if ( qword_9FDC8 )
      qword_9FDC8();
    v5 = gettext("Malformed extended header: invalid %s=%s");
    error(0, 0, v5, a5, a2);
    dword_9FDD0 = 2;
    return 0;
  }
  else if ( *__errno_location() == 34 )
  {
    sub_1EC02(a5, a2, a3, a4);
    return 0;
  }
  else
  {
    *a1 = v11;
    return 1;
  }
}



// Function: dummy_coder @ 0x1f1bd
void dummy_coder()
{
  ;
}



// Function: dummy_decoder @ 0x1f1d8
void dummy_decoder()
{
  ;
}



// Function: atime_coder @ 0x1f1f3
unsigned long long atime_coder(long long a1, char *a2, long long *a3)
{
  return sub_1EDD0(*(long long *)(a1 + 232), *(long long *)(a1 + 240), a2, a3);
}



// Function: atime_decoder @ 0x1f237
unsigned long long atime_decoder(long long a1, long long a2, unsigned long long a3)
{
  long long v3; // rdx
  long long v5[3]; // [rsp+20h] [rbp-20h] BYREF
  unsigned long long v6; // [rsp+38h] [rbp-8h]

  v6 = __readfsqword(0x28u);
  if ( (unsigned char)sub_1EE4E(v5, a3, a2) )
  {
    v3 = v5[1];
    *(long long *)(a1 + 232) = v5[0];
    *(long long *)(a1 + 240) = v3;
  }
  return v6 - __readfsqword(0x28u);
}



// Function: gid_coder @ 0x1f2ae
unsigned long long gid_coder(long long a1, char *a2, long long *a3)
{
  return sub_1EF79(*(unsigned int *)(a1 + 120), a2, 0, 0xFFFFFFFFLL, a3);
}



// Function: gid_decoder @ 0x1f2f6
unsigned long long gid_decoder(long long a1, long long a2, long long a3)
{
  long long v4; // [rsp+20h] [rbp-10h] BYREF
  unsigned long long v5; // [rsp+28h] [rbp-8h]

  v5 = __readfsqword(0x28u);
  if ( (unsigned char)sub_1F02B(&v4, a3, 0, 0xFFFFFFFFLL, a2) )
    *(int *)(a1 + 120) = v4;
  return v5 - __readfsqword(0x28u);
}



// Function: gname_coder @ 0x1f36d
unsigned long long gname_coder(long long a1, char *a2, long long *a3)
{
  return sub_1ECD7(*(long long *)(a1 + 40), a2, a3);
}



// Function: gname_decoder @ 0x1f3a7
long long gname_decoder(long long a1, long long a2, long long a3)
{
  return sub_1ED64((void **)(a1 + 40), a3);
}



// Function: linkpath_coder @ 0x1f3dd
unsigned long long linkpath_coder(long long a1, char *a2, long long *a3)
{
  return sub_1ECD7(*(long long *)(a1 + 24), a2, a3);
}



// Function: linkpath_decoder @ 0x1f417
long long linkpath_decoder(long long a1, long long a2, long long a3)
{
  return sub_1ED64((void **)(a1 + 24), a3);
}



// Function: ctime_coder @ 0x1f44d
unsigned long long ctime_coder(long long a1, char *a2, long long *a3)
{
  return sub_1EDD0(*(long long *)(a1 + 264), *(long long *)(a1 + 272), a2, a3);
}



// Function: ctime_decoder @ 0x1f491
unsigned long long ctime_decoder(long long a1, long long a2, unsigned long long a3)
{
  long long v3; // rdx
  long long v5[3]; // [rsp+20h] [rbp-20h] BYREF
  unsigned long long v6; // [rsp+38h] [rbp-8h]

  v6 = __readfsqword(0x28u);
  if ( (unsigned char)sub_1EE4E(v5, a3, a2) )
  {
    v3 = v5[1];
    *(long long *)(a1 + 264) = v5[0];
    *(long long *)(a1 + 272) = v3;
  }
  return v6 - __readfsqword(0x28u);
}



// Function: mtime_coder @ 0x1f508
unsigned long long mtime_coder(long long a1, char *a2, long long *a3, long long *a4)
{
  long long v4; // rdx
  long long v5; // rax

  if ( a4 )
  {
    v4 = a4[1];
    v5 = *a4;
  }
  else
  {
    v4 = *(long long *)(a1 + 256);
    v5 = *(long long *)(a1 + 248);
  }
  return sub_1EDD0(v5, v4, a2, a3);
}



// Function: mtime_decoder @ 0x1f574
unsigned long long mtime_decoder(long long a1, long long a2, unsigned long long a3)
{
  long long v3; // rdx
  long long v5[3]; // [rsp+20h] [rbp-20h] BYREF
  unsigned long long v6; // [rsp+38h] [rbp-8h]

  v6 = __readfsqword(0x28u);
  if ( (unsigned char)sub_1EE4E(v5, a3, a2) )
  {
    v3 = v5[1];
    *(long long *)(a1 + 248) = v5[0];
    *(long long *)(a1 + 256) = v3;
  }
  return v6 - __readfsqword(0x28u);
}



// Function: path_coder @ 0x1f5eb
unsigned long long path_coder(long long a1, char *a2, long long *a3)
{
  return sub_1ECD7(*(long long *)(a1 + 8), a2, a3);
}



// Function: raw_path_decoder @ 0x1f625
long long raw_path_decoder(long long a1, long long a2)
{
  long long result; // rax

  sub_1ED64((void **)a1, a2);
  sub_1ED64((void **)(a1 + 8), a2);
  result = sub_4E2E5(*(long long *)(a1 + 8));
  *(char *)(a1 + 16) = result;
  return result;
}



// Function: path_decoder @ 0x1f67d
long long path_decoder(long long a1, long long a2, long long a3)
{
  long long result; // rax

  result = *(unsigned char *)(a1 + 337) ^ 1u;
  if ( *(char *)(a1 + 337) != 1 )
    return sub_1F625(a1, a3);
  return result;
}



// Function: sparse_path_decoder @ 0x1f6c1
long long sparse_path_decoder(long long a1, long long a2, long long a3)
{
  *(char *)(a1 + 337) = 1;
  return sub_1F625(a1, a3);
}



// Function: size_coder @ 0x1f6fe
unsigned long long size_coder(long long a1, char *a2, long long *a3)
{
  return sub_1EFED(*(long long *)(a1 + 136), a2, a3);
}



// Function: size_decoder @ 0x1f73e
unsigned long long size_decoder(long long a1, long long a2, long long a3)
{
  long long v4; // [rsp+20h] [rbp-10h] BYREF
  unsigned long long v5; // [rsp+28h] [rbp-8h]

  v5 = __readfsqword(0x28u);
  if ( (unsigned char)sub_1F138(&v4, a3, 0x7FFFFFFFFFFFFFFFLL, a2) )
    *(long long *)(a1 + 136) = v4;
  return v5 - __readfsqword(0x28u);
}



// Function: uid_coder @ 0x1f7b7
unsigned long long uid_coder(long long a1, char *a2, long long *a3)
{
  return sub_1EF79(*(unsigned int *)(a1 + 116), a2, 0, 0xFFFFFFFFLL, a3);
}



// Function: uid_decoder @ 0x1f7ff
unsigned long long uid_decoder(long long a1, long long a2, long long a3)
{
  long long v4; // [rsp+20h] [rbp-10h] BYREF
  unsigned long long v5; // [rsp+28h] [rbp-8h]

  v5 = __readfsqword(0x28u);
  if ( (unsigned char)sub_1F02B(&v4, a3, 0, 0xFFFFFFFFLL, a2) )
    *(int *)(a1 + 116) = v4;
  return v5 - __readfsqword(0x28u);
}



// Function: uname_coder @ 0x1f876
unsigned long long uname_coder(long long a1, char *a2, long long *a3)
{
  return sub_1ECD7(*(long long *)(a1 + 32), a2, a3);
}



// Function: uname_decoder @ 0x1f8b0
long long uname_decoder(long long a1, long long a2, long long a3)
{
  return sub_1ED64((void **)(a1 + 32), a3);
}



// Function: sparse_size_coder @ 0x1f8e6
unsigned long long sparse_size_coder(long long a1, char *a2, long long *a3)
{
  return sub_1F6FE(a1, a2, a3);
}



// Function: sparse_size_decoder @ 0x1f91d
unsigned long long sparse_size_decoder(long long a1, long long a2, long long a3)
{
  long long v4; // [rsp+20h] [rbp-10h] BYREF
  unsigned long long v5; // [rsp+28h] [rbp-8h]

  v5 = __readfsqword(0x28u);
  if ( (unsigned char)sub_1F138(&v4, a3, 0x7FFFFFFFFFFFFFFFLL, a2) )
  {
    *(char *)(a1 + 336) = 1;
    *(long long *)(a1 + 328) = v4;
  }
  return v5 - __readfsqword(0x28u);
}



// Function: sparse_numblocks_coder @ 0x1f9a1
unsigned long long sparse_numblocks_coder(long long a1, char *a2, long long *a3)
{
  return sub_1EFED(*(long long *)(a1 + 304), a2, a3);
}



// Function: sparse_numblocks_decoder @ 0x1f9de
unsigned long long sparse_numblocks_decoder(long long *a1, long long a2, long long a3, long long a4)
{
  long long v4; // rcx
  long long v5; // r8
  long long v6; // r9
  long long v7; // rdx
  long long v11; // [rsp+20h] [rbp-10h] BYREF
  unsigned long long v12; // [rsp+28h] [rbp-8h]

  v12 = __readfsqword(0x28u);
  if ( (unsigned char)sub_1F138(&v11, a3, -1, a2) )
  {
    v7 = v11;
    a1[39] = v11;
    a1[40] = sub_60C29(v11, 16, v7, v4, v5, v6, a4, a3, a2);
    a1[38] = 0;
  }
  return v12 - __readfsqword(0x28u);
}



// Function: sparse_offset_coder @ 0x1fa7f
unsigned long long sparse_offset_coder(long long a1, char *a2, long long *a3, long long *a4)
{
  return sub_1EFED(*(long long *)(*(long long *)(a1 + 320) + 16LL * *a4), a2, a3);
}



// Function: sparse_offset_decoder @ 0x1fad8
unsigned long long sparse_offset_decoder(long long *a1, long long a2, long long a3)
{
  char *v3; // rax
  long long v6; // [rsp+20h] [rbp-10h] BYREF
  unsigned long long v7; // [rsp+28h] [rbp-8h]

  v7 = __readfsqword(0x28u);
  if ( (unsigned char)sub_1F138(&v6, a3, 0x7FFFFFFFFFFFFFFFLL, a2) )
  {
    if ( a1[38] >= a1[39] )
    {
      if ( qword_9FDC8 )
        qword_9FDC8(&v6);
      v3 = gettext("Malformed extended header: excess %s=%s");
      error(0, 0, v3, "GNU.sparse.offset", a3);
      dword_9FDD0 = 2;
    }
    else
    {
      *(long long *)(a1[40] + 16LL * a1[38]) = v6;
    }
  }
  return v7 - __readfsqword(0x28u);
}



// Function: sparse_numbytes_coder @ 0x1fbda
unsigned long long sparse_numbytes_coder(long long a1, char *a2, long long *a3, long long *a4)
{
  return sub_1EFED(*(long long *)(*(long long *)(a1 + 320) + 16LL * *a4 + 8), a2, a3);
}



// Function: sparse_numbytes_decoder @ 0x1fc34
unsigned long long sparse_numbytes_decoder(long long *a1, long long a2, long long a3)
{
  long long v3; // rsi
  long long v4; // rdi
  long long v5; // rax
  char *v6; // rax
  long long v10; // [rsp+20h] [rbp-10h] BYREF
  unsigned long long v11; // [rsp+28h] [rbp-8h]

  v11 = __readfsqword(0x28u);
  if ( (unsigned char)sub_1F138(&v10, a3, 0x7FFFFFFFFFFFFFFFLL, a2) )
  {
    if ( a1[38] >= a1[39] )
    {
      if ( qword_9FDC8 )
        qword_9FDC8(&v10);
      v6 = gettext("Malformed extended header: excess %s=%s");
      error(0, 0, v6, a2, a3);
      dword_9FDD0 = 2;
    }
    else
    {
      v3 = v10;
      v4 = a1[40];
      v5 = a1[38];
      a1[38] = v5 + 1;
      *(long long *)(v4 + 16 * v5 + 8) = v3;
    }
  }
  return v11 - __readfsqword(0x28u);
}



// Function: sparse_map_decoder @ 0x1fd46
unsigned long long sparse_map_decoder(const char *a1, long long a2, const char *a3)
{
  char *v3; // rax
  long long v4; // rsi
  long long v5; // rax
  intmax_t *v6; // rcx
  intmax_t v7; // rdx
  char *v8; // rax
  unsigned int v9; // ebx
  char *v10; // rax
  char *v11; // rax
  const char *v15; // [rsp+18h] [rbp-58h]
  int v16; // [rsp+2Ch] [rbp-44h]
  char *endptr; // [rsp+30h] [rbp-40h] BYREF
  intmax_t v18; // [rsp+38h] [rbp-38h]
  intmax_t v19; // [rsp+40h] [rbp-30h]
  intmax_t v20; // [rsp+48h] [rbp-28h]
  unsigned long long v21; // [rsp+58h] [rbp-18h]

  v15 = a1;
  v21 = __readfsqword(0x28u);
  v16 = 1;
  *((long long *)a1 + 38) = 0;
  while ( 1 )
  {
    if ( (unsigned int)(*a3 - 48) > 9 )
    {
      if ( qword_9FDC8 )
        qword_9FDC8(a1);
      v3 = gettext("Malformed extended header: invalid %s=%s");
      error(0, 0, v3, a2, a3);
      dword_9FDD0 = 2;
      return v21 - __readfsqword(0x28u);
    }
    *__errno_location() = 0;
    a1 = a3;
    v18 = strtoimax(a3, &endptr, 10);
    if ( !v16 )
      break;
    v19 = v18;
    if ( *__errno_location() == 34 )
      goto LABEL_8;
LABEL_15:
    v16 = !v16;
    if ( !*endptr )
    {
      if ( !v16 )
      {
        if ( qword_9FDC8 )
          qword_9FDC8(a3);
        v11 = gettext("Malformed extended header: invalid %s: odd number of values");
        error(0, 0, v11, a2);
        dword_9FDD0 = 2;
      }
      return v21 - __readfsqword(0x28u);
    }
    if ( *endptr != 44 )
    {
      if ( qword_9FDC8 )
        qword_9FDC8(a3);
      v9 = *endptr;
      v10 = gettext("Malformed extended header: invalid %s: unexpected delimiter %c");
      error(0, 0, v10, a2, v9);
      dword_9FDD0 = 2;
      return v21 - __readfsqword(0x28u);
    }
    a3 = endptr + 1;
  }
  v20 = v18;
  if ( *__errno_location() == 34 )
  {
LABEL_8:
    sub_1EC02(a2, (long long)a3, 0, 0x7FFFFFFFFFFFFFFFLL);
    return v21 - __readfsqword(0x28u);
  }
  if ( *((long long *)v15 + 38) < *((long long *)v15 + 39) )
  {
    v4 = *((long long *)v15 + 40);
    v5 = *((long long *)v15 + 38);
    *((long long *)v15 + 38) = v5 + 1;
    v6 = (intmax_t *)(v4 + 16 * v5);
    v7 = v20;
    *v6 = v19;
    v6[1] = v7;
    goto LABEL_15;
  }
  if ( qword_9FDC8 )
    qword_9FDC8(a3);
  v8 = gettext("Malformed extended header: excess %s=%s");
  error(0, 0, v8, a2, a3);
  dword_9FDD0 = 2;
  return v21 - __readfsqword(0x28u);
}



// Function: dumpdir_coder @ 0x2004a
unsigned long long dumpdir_coder(long long a1, char *a2, long long *a3, const void *a4)
{
  size_t v4; // rax

  v4 = sub_20BFE(a4);
  return sub_1E4FF(a3, a2, a4, v4);
}



// Function: dumpdir_decoder @ 0x2008c
void *dumpdir_decoder(long long a1, long long a2, const void *a3, size_t a4)
{
  *(long long *)(a1 + 400) = sub_60B5B(a4);
  return memcpy(*(void **)(a1 + 400), a3, a4);
}



// Function: volume_label_coder @ 0x200e3
unsigned long long volume_label_coder(long long a1, char *a2, long long *a3, long long a4)
{
  return sub_1ECD7(a4, a2, a3);
}



// Function: volume_label_decoder @ 0x20119
long long volume_label_decoder(long long a1, long long a2, long long a3)
{
  return sub_1ED64((void **)&qword_9EFE0, a3);
}



// Function: volume_size_coder @ 0x2014e
unsigned long long volume_size_coder(long long a1, char *a2, long long *a3, long long *a4)
{
  return sub_1EFED(*a4, a2, a3);
}



// Function: volume_size_decoder @ 0x20192
unsigned long long volume_size_decoder(long long a1, long long a2, long long a3)
{
  long long v4; // [rsp+20h] [rbp-10h] BYREF
  unsigned long long v5; // [rsp+28h] [rbp-8h]

  v5 = __readfsqword(0x28u);
  if ( (unsigned char)sub_1F138(&v4, a3, -1, a2) )
    qword_9EFF0 = v4;
  return v5 - __readfsqword(0x28u);
}



// Function: volume_offset_coder @ 0x20201
unsigned long long volume_offset_coder(long long a1, char *a2, long long *a3, long long *a4)
{
  return sub_1EFED(*a4, a2, a3);
}



// Function: volume_offset_decoder @ 0x20245
unsigned long long volume_offset_decoder(long long a1, long long a2, long long a3)
{
  long long v4; // [rsp+20h] [rbp-10h] BYREF
  unsigned long long v5; // [rsp+28h] [rbp-8h]

  v5 = __readfsqword(0x28u);
  if ( (unsigned char)sub_1F138(&v4, a3, -1, a2) )
    qword_9EFF8 = v4;
  return v5 - __readfsqword(0x28u);
}



// Function: volume_filename_decoder @ 0x202b4
long long volume_filename_decoder(long long a1, long long a2, long long a3)
{
  return sub_1ED64((void **)&qword_9EFE8, a3);
}



// Function: xattr_selinux_coder @ 0x202e9
unsigned long long xattr_selinux_coder(long long a1, char *a2, long long *a3)
{
  return sub_1ECD7(*(long long *)(a1 + 48), a2, a3);
}



// Function: xattr_selinux_decoder @ 0x20323
long long xattr_selinux_decoder(long long a1, long long a2, long long a3)
{
  return sub_1ED64((void **)(a1 + 48), a3);
}



// Function: xattr_acls_a_coder @ 0x20359
unsigned long long xattr_acls_a_coder(long long a1, char *a2, long long *a3)
{
  return sub_1E4FF(a3, a2, *(const void **)(a1 + 56), *(long long *)(a1 + 64));
}



// Function: xattr_acls_a_decoder @ 0x20398
long long xattr_acls_a_decoder(long long a1, long long a2, long long a3, long long a4)
{
  *(long long *)(a1 + 56) = sub_60C8E(a3, a4 + 1);
  *(long long *)(a1 + 64) = a4;
  return a1;
}



// Function: xattr_acls_d_coder @ 0x203e2
unsigned long long xattr_acls_d_coder(long long a1, char *a2, long long *a3)
{
  return sub_1E4FF(a3, a2, *(const void **)(a1 + 72), *(long long *)(a1 + 80));
}



// Function: xattr_acls_d_decoder @ 0x20421
long long xattr_acls_d_decoder(long long a1, long long a2, long long a3, long long a4)
{
  *(long long *)(a1 + 72) = sub_60C8E(a3, a4 + 1);
  *(long long *)(a1 + 80) = a4;
  return a1;
}



// Function: xattr_coder @ 0x2046b
unsigned long long xattr_coder(long long a1, char *a2, long long *a3, long long *a4)
{
  return sub_1E4FF(
           a3,
           a2,
           *(const void **)(24LL * *a4 + *(long long *)(a1 + 352) + 8),
           *(long long *)(24LL * *a4 + *(long long *)(a1 + 352) + 16));
}



// Function: xattr_decoder @ 0x204f5
unsigned long long xattr_decoder(long long a1, char *a2, void *a3, long long a4)
{
  unsigned long long v4; // rax
  void *v5; // rsp
  unsigned long long v6; // rax
  void *v7; // rsp
  size_t v8; // rax
  long long v10; // [rsp+8h] [rbp-40h] BYREF
  void *src; // [rsp+10h] [rbp-38h]
  char *s; // [rsp+18h] [rbp-30h]
  long long v13; // [rsp+20h] [rbp-28h]
  size_t v14; // [rsp+28h] [rbp-20h]
  void *dest; // [rsp+30h] [rbp-18h]
  void *v16; // [rsp+38h] [rbp-10h]
  unsigned long long v17; // [rsp+40h] [rbp-8h]

  v13 = a1;
  s = a2;
  src = a3;
  v10 = a4;
  v17 = __readfsqword(0x28u);
  v14 = strlen(a2);
  v4 = 16 * ((v14 + 24) / 0x10);
  while ( &v10 != (long long *)((char *)&v10 - (v4 & 0xFFFFFFFFFFFFF000LL)) )
    ;
  v5 = alloca(v4 & 0xFFF);
  if ( (v4 & 0xFFF) != 0 )
    *(long long *)((char *)&v10 + (v4 & 0xFFF) - 8) = *(long long *)((char *)&v10 + (v4 & 0xFFF) - 8);
  dest = &v10;
  memcpy(&v10, s, v14 + 1);
  v6 = 16 * ((v10 + 24) / 0x10uLL);
  while ( &v10 != (long long *)((char *)&v10 - (v6 & 0xFFFFFFFFFFFFF000LL)) )
    ;
  v7 = alloca(v6 & 0xFFF);
  if ( (v6 & 0xFFF) != 0 )
    *(long long *)((char *)&v10 + (v6 & 0xFFF) - 8) = *(long long *)((char *)&v10 + (v6 & 0xFFF) - 8);
  v16 = &v10;
  memcpy(&v10, src, v10 + 1);
  sub_1D6DA(dest);
  v8 = strlen("SCHILY.xattr.");
  sub_1D78E(v13, (const char *)dest + v8, (long long)v16, v10);
  return v17 - __readfsqword(0x28u);
}



// Function: sparse_major_coder @ 0x206ce
unsigned long long sparse_major_coder(long long a1, char *a2, long long *a3)
{
  return sub_1EFED(*(unsigned int *)(a1 + 292), a2, a3);
}



// Function: sparse_major_decoder @ 0x2070c
unsigned long long sparse_major_decoder(long long a1, long long a2, long long a3)
{
  long long v4; // [rsp+20h] [rbp-10h] BYREF
  unsigned long long v5; // [rsp+28h] [rbp-8h]

  v5 = __readfsqword(0x28u);
  if ( (unsigned char)sub_1F138(&v4, a3, 0xFFFFFFFFLL, a2) )
    *(int *)(a1 + 292) = v4;
  return v5 - __readfsqword(0x28u);
}



// Function: sparse_minor_coder @ 0x2077e
unsigned long long sparse_minor_coder(long long a1, char *a2, long long *a3)
{
  return sub_1EFED(*(unsigned int *)(a1 + 296), a2, a3);
}



// Function: sparse_minor_decoder @ 0x207bc
unsigned long long sparse_minor_decoder(long long a1, long long a2, long long a3)
{
  long long v4; // [rsp+20h] [rbp-10h] BYREF
  unsigned long long v5; // [rsp+28h] [rbp-8h]

  v5 = __readfsqword(0x28u);
  if ( (unsigned char)sub_1F138(&v4, a3, 0xFFFFFFFFLL, a2) )
    *(int *)(a1 + 296) = v4;
  return v5 - __readfsqword(0x28u);
}



// Function: dumpdir_create0 @ 0x2082e
long long dumpdir_create0(char *a1, const char *a2)
{
  long long v2; // rcx
  long long v3; // r8
  long long v4; // r9
  long long v5; // rax
  char *v7; // [rsp+0h] [rbp-50h]
  long long v8; // [rsp+18h] [rbp-38h]
  long long v9; // [rsp+18h] [rbp-38h]
  long long v10; // [rsp+20h] [rbp-30h]
  size_t n; // [rsp+28h] [rbp-28h]
  char *i; // [rsp+30h] [rbp-20h]
  char *s; // [rsp+38h] [rbp-18h]
  long long v14; // [rsp+40h] [rbp-10h]
  size_t v15; // [rsp+48h] [rbp-8h]

  v8 = 0;
  v10 = 0;
  n = 1;
  for ( s = a1; *s; s += v15 )
  {
    v15 = strlen(s) + 1;
    n += v15;
    if ( !a2 || strchr(a2, *s) )
      ++v8;
    ++v10;
  }
  v14 = sub_60B5B(n + 32);
  *(long long *)v14 = v14 + 32;
  memcpy(*(void **)v14, a1, n);
  *(long long *)(v14 + 8) = v10;
  *(long long *)(v14 + 16) = v8;
  *(long long *)(v14 + 24) = sub_60C29(v8 + 1, 8, v8, v2, v3, v4, a2, a1);
  v9 = 0;
  for ( i = *(char **)v14; *i; i += strlen(i) + 1 )
  {
    if ( !v7 || strchr(v7, *i) )
    {
      v5 = v9++;
      *(long long *)(*(long long *)(v14 + 24) + 8 * v5) = i + 1;
    }
  }
  *(long long *)(*(long long *)(v14 + 24) + 8 * v9) = 0;
  return v14;
}



// Function: dumpdir_create @ 0x209d4
long long dumpdir_create(char *a1)
{
  return sub_2082E(a1, "YND");
}



// Function: dumpdir_free @ 0x209fc
void dumpdir_free(void **a1)
{
  free(a1[3]);
  free(a1);
}



// Function: compare_dirnames @ 0x20a2b
int compare_dirnames(const char **a1, const char **a2)
{
  return strcmp(*a1, *a2);
}



// Function: dumpdir_locate @ 0x20a6a
long long dumpdir_locate(long long a1, long long a2)
{
  long long key; // [rsp+0h] [rbp-20h] BYREF
  long long v4; // [rsp+8h] [rbp-18h]
  long long *v5; // [rsp+18h] [rbp-8h]

  v4 = a1;
  key = a2;
  if ( !a1 )
    return 0;
  v5 = bsearch(&key, *(const void **)(v4 + 24), *(long long *)(v4 + 16), 8u, (__compar_fn_t)compar);
  if ( v5 )
    return *v5 - 1LL;
  else
    return 0;
}



// Function: dumpdir_next @ 0x20ad3
char *dumpdir_next(long long *a1)
{
  long long v2; // rbx
  char *s; // [rsp+10h] [rbp-20h]
  unsigned long long v4; // [rsp+18h] [rbp-18h]

  v4 = a1[2];
  s = 0;
  if ( *((int *)a1 + 2) )
  {
    s = (char *)(*(long long *)*a1 + v4);
    if ( !*s )
      return 0;
    v2 = a1[2];
    a1[2] = v2 + strlen(s) + 1;
  }
  else if ( v4 < *(long long *)(*a1 + 16LL) )
  {
    s = (char *)(*(long long *)(*(long long *)(*a1 + 24LL) + 8 * v4) - 1LL);
    ++a1[2];
  }
  return s;
}



// Function: dumpdir_first @ 0x20b9f
char *dumpdir_first(long long a1, int a2, long long *a3)
{
  long long v5; // [rsp+28h] [rbp-8h]

  v5 = sub_60B5B(24);
  *(long long *)v5 = a1;
  *(int *)(v5 + 8) = a2;
  *(long long *)(v5 + 16) = 0;
  *a3 = v5;
  return sub_20AD3((long long *)v5);
}



// Function: dumpdir_size @ 0x20bfe
long long dumpdir_size(const char *a1)
{
  long long v3; // [rsp+10h] [rbp-10h]
  size_t v4; // [rsp+18h] [rbp-8h]

  v3 = 0;
  while ( *a1 )
  {
    v4 = strlen(a1) + 1;
    v3 += v4;
    a1 += v4;
  }
  return v3 + 1;
}



// Function: hash_directory_canonical_name @ 0x20c51
long long hash_directory_canonical_name(long long a1, long long a2)
{
  return sub_52834(*(long long *)(a1 + 80), a2);
}



// Function: compare_directory_canonical_names @ 0x20c86
bool compare_directory_canonical_names(long long a1, long long a2)
{
  return strcmp(*(const char **)(a1 + 80), *(const char **)(a2 + 80)) == 0;
}



// Function: hash_directory_meta @ 0x20ccc
unsigned long long hash_directory_meta(long long a1, unsigned long long a2)
{
  return (*(long long *)(a1 + 24) + *(long long *)(a1 + 32)) % a2;
}



// Function: compare_directory_meta @ 0x20d05
long long compare_directory_meta(long long a1, long long a2)
{
  return *(long long *)(a1 + 24) == *(long long *)(a2 + 24) && *(long long *)(a1 + 32) == *(long long *)(a2 + 32);
}



// Function: make_directory @ 0x20d60
long long make_directory(const char *a1, long long a2)
{
  size_t n; // [rsp+10h] [rbp-10h]
  long long v4; // [rsp+18h] [rbp-8h]

  n = strlen(a1);
  v4 = sub_60B5B(96);
  *(long long *)v4 = 0;
  *(long long *)(v4 + 48) = 0;
  *(long long *)(v4 + 40) = *(long long *)(v4 + 48);
  *(long long *)(v4 + 64) = 0;
  *(int *)(v4 + 60) = 0;
  if ( n > 1 && a1[n - 1] == 47 )
    --n;
  *(long long *)(v4 + 88) = sub_60B5B(n + 1);
  memcpy(*(void **)(v4 + 88), a1, n);
  *(char *)(*(long long *)(v4 + 88) + n) = 0;
  *(long long *)(v4 + 80) = a2;
  *(long long *)(v4 + 72) = 0;
  return v4;
}



// Function: free_directory @ 0x20e58
void free_directory(void **a1)
{
  free(a1[10]);
  free(a1[11]);
  free(a1);
}



// Function: attach_directory @ 0x20e97
long long attach_directory(const char *a1)
{
  long long v2; // [rsp+10h] [rbp-10h]
  long long v3; // [rsp+18h] [rbp-8h]

  v2 = sub_29AAC((unsigned int)dword_9F600, a1);
  v3 = sub_20D60(a1, v2);
  if ( qword_9F440 )
    *(long long *)qword_9F440 = v3;
  else
    qword_9F438 = v3;
  qword_9F440 = v3;
  return v3;
}



// Function: dirlist_replace_prefix @ 0x20f0e
long long dirlist_replace_prefix(const char *a1, const char *a2)
{
  long long result; // rax
  long long *i; // [rsp+18h] [rbp-18h]
  size_t v4; // [rsp+20h] [rbp-10h]
  size_t v5; // [rsp+28h] [rbp-8h]

  v4 = strlen(a1);
  v5 = strlen(a2);
  result = qword_9F438;
  for ( i = (long long *)qword_9F438; i; i = (long long *)*i )
  {
    sub_29B93(i + 11, a1, v4, a2, v5);
    result = *i;
  }
  return result;
}



// Function: clear_directory_table @ 0x20f88
long long clear_directory_table()
{
  void **i; // [rsp+0h] [rbp-10h]
  void **v2; // [rsp+8h] [rbp-8h]

  if ( qword_9F448 )
    sub_52D91(qword_9F448);
  if ( qword_9F450 )
    sub_52D91(qword_9F450);
  for ( i = (void **)qword_9F438; i; i = v2 )
  {
    v2 = (void **)*i;
    sub_20E58(i);
  }
  qword_9F440 = 0;
  qword_9F438 = 0;
  return 0;
}



// Function: note_directory @ 0x21019
long long note_directory(
        const char *a1,
        long long a2,
        long long a3,
        long long a4,
        long long a5,
        char a6,
        char a7,
        char *a8)
{
  long long v13; // [rsp+38h] [rbp-8h]

  v13 = sub_20E97(a1);
  *(long long *)(v13 + 8) = a2;
  *(long long *)(v13 + 16) = a3;
  *(long long *)(v13 + 24) = a4;
  *(long long *)(v13 + 32) = a5;
  *(int *)(v13 + 56) = 1;
  if ( a6 )
    *(int *)(v13 + 60) |= 2u;
  if ( a7 )
    *(int *)(v13 + 60) |= 4u;
  if ( a8 )
    *(long long *)(v13 + 40) = sub_209D4(a8);
  else
    *(long long *)(v13 + 40) = 0;
  if ( !qword_9F448 && (qword_9F448 = sub_52C1C(0, 0, sub_20C51, sub_20C86, 0)) == 0 || !sub_5395D(qword_9F448, v13) )
    sub_1783C();
  if ( !qword_9F450 && (qword_9F450 = sub_52C1C(0, 0, sub_20CCC, sub_20D05, 0)) == 0 || !sub_5395D(qword_9F450, v13) )
    sub_1783C();
  return v13;
}



// Function: find_directory @ 0x211c6
long long find_directory(const char *a1)
{
  long long v2; // [rsp+18h] [rbp-18h]
  void **v3; // [rsp+20h] [rbp-10h]
  long long v4; // [rsp+28h] [rbp-8h]

  if ( !qword_9F448 )
    return 0;
  v2 = sub_29AAC((unsigned int)dword_9F600, a1);
  v3 = (void **)sub_20D60(a1, v2);
  v4 = sub_52567(qword_9F448, v3);
  sub_20E58(v3);
  return v4;
}



// Function: rebase_directory @ 0x21244
long long rebase_directory(long long a1, long long a2, long long a3, long long a4, long long a5)
{
  return sub_29B93(a1 + 88, a2, a3, a4, a5);
}



// Function: find_directory_meta @ 0x2128a
long long find_directory_meta(long long a1, long long a2)
{
  long long v3; // [rsp+10h] [rbp-10h]
  long long v4; // [rsp+18h] [rbp-8h]

  if ( !qword_9F450 )
    return 0;
  v3 = sub_20D60(byte_81BE4, 0);
  *(long long *)(v3 + 24) = a1;
  *(long long *)(v3 + 32) = a2;
  v4 = sub_52567(qword_9F450, v3);
  sub_20E58((void **)v3);
  return v4;
}



// Function: update_parent_directory @ 0x2130d
unsigned long long update_parent_directory(long long a1)
{
  long long v1; // rdx
  long long *v3; // [rsp+18h] [rbp-B8h]
  struct stat buf; // [rsp+20h] [rbp-B0h] BYREF
  unsigned long long v5; // [rsp+B8h] [rbp-18h]

  v5 = __readfsqword(0x28u);
  v3 = (long long *)sub_211C6(*(const char **)a1);
  if ( v3 )
  {
    if ( fstat(*(int *)(a1 + 424), &buf) )
    {
      sub_2B444(v3[11]);
    }
    else
    {
      v3[1] = sub_5FD9E(&buf);
      v3[2] = v1;
    }
  }
  return v5 - __readfsqword(0x28u);
}



// Function: procdir @ 0x213c6
long long procdir(char *a1, long long a2, char a3, char *a4)
{
  long long v5; // r12
  long long v6; // rbx
  char *v7; // rax
  long long v8; // r12
  long long v9; // rbx
  long long v10; // rax
  long long v11; // rdx
  long long v12; // r12
  long long v13; // rbx
  char *v14; // rax
  long long v15; // rbx
  char *v16; // rax
  long long v17; // rdx
  long long v18; // rdx
  int v19; // edx
  long long v20; // rbx
  char *v21; // rax
  long long v22; // rbx
  char *v23; // rax
  unsigned int v24; // eax
  char *v25; // rax
  char *v26; // rax
  char *v27; // rax
  char *s2; // [rsp+18h] [rbp-68h]
  char v31; // [rsp+2Eh] [rbp-52h]
  char v32; // [rsp+2Fh] [rbp-51h]
  long long v33; // [rsp+30h] [rbp-50h] BYREF
  long long v34; // [rsp+38h] [rbp-48h]
  long long *v35; // [rsp+40h] [rbp-40h]
  long long v36; // [rsp+48h] [rbp-38h]
  long long v37; // [rsp+50h] [rbp-30h]
  unsigned long long v38; // [rsp+58h] [rbp-28h]

  s2 = a1;
  v38 = __readfsqword(0x28u);
  v35 = (long long *)(a2 + 88);
  v32 = *(long long *)(a2 + 88) < 0;
  v31 = 0;
  v34 = sub_211C6(a1);
  if ( v34 )
  {
    if ( (*(int *)(v34 + 60) & 1) != 0 )
    {
      if ( (a3 & 0x20) == 0 )
      {
        *a4 = 78;
        return v34;
      }
      sub_29547(v34 + 88, a1);
    }
    a1 = *(char **)(v34 + 88);
    if ( strcmp(a1, s2) )
    {
      *a4 = 78;
      return v34;
    }
    if ( (!byte_9F8EC || (*(int *)(v34 + 60) & 2) != 0 && v32 == 1 || *(long long *)(v34 + 24) == *v35)
      && *(long long *)(v34 + 32) == v35[1] )
    {
      *(int *)(v34 + 56) = 1;
    }
    else
    {
      a1 = (char *)*v35;
      v37 = sub_2128A(*v35, v35[1]);
      if ( v37 )
      {
        a1 = *(char **)(v37 + 88);
        if ( strcmp(a1, s2) )
        {
          if ( (dword_9EC84 & 0x2000) != 0 )
          {
            if ( qword_9FDC8 )
              qword_9FDC8(a1);
            v5 = sub_5E821(1, *(long long *)(v37 + 88));
            v6 = sub_5E548(s2);
            v7 = gettext("%s: Directory has been renamed from %s");
            error(0, 0, v7, v6, v5);
          }
          *(long long *)(v34 + 64) = v37;
          *(int *)(v34 + 60) |= 0x10u;
          *(int *)(v37 + 60) &= ~0x10u;
          a1 = *(char **)(v37 + 88);
          sub_20F0E(a1, s2);
        }
        *(int *)(v34 + 56) = 1;
      }
      else
      {
        v31 = 1;
        *(int *)(v34 + 56) = 2;
        *(long long *)(v34 + 24) = *v35;
        *(long long *)(v34 + 32) = v35[1];
      }
      if ( v32 )
        *(int *)(v34 + 60) |= 2u;
    }
    *(int *)(v34 + 60) |= 4u;
  }
  else
  {
    v36 = sub_2128A(*v35, v35[1]);
    v8 = v35[1];
    v9 = *v35;
    v10 = sub_5FD9E(v35);
    v34 = sub_21019(a1, v10, v11, v9, v8, v32, 1, 0);
    if ( v36 )
    {
      a1 = *(char **)(v36 + 88);
      if ( strcmp(a1, s2) )
      {
        if ( (dword_9EC84 & 0x2000) != 0 )
        {
          if ( qword_9FDC8 )
            qword_9FDC8(a1);
          v12 = sub_5E821(1, *(long long *)(v36 + 88));
          v13 = sub_5E548(s2);
          v14 = gettext("%s: Directory has been renamed from %s");
          error(0, 0, v14, v13, v12);
        }
        *(long long *)(v34 + 64) = v36;
        *(int *)(v34 + 60) |= 0x10u;
        *(int *)(v36 + 60) &= ~0x10u;
        a1 = *(char **)(v36 + 88);
        sub_20F0E(a1, s2);
      }
      *(int *)(v34 + 56) = 1;
    }
    else
    {
      *(int *)(v34 + 60) |= 8u;
      if ( (dword_9EC84 & 0x1000) != 0 )
      {
        if ( qword_9FDC8 )
          qword_9FDC8(a1);
        v15 = sub_5E548(a1);
        v16 = gettext("%s: Directory is new");
        a1 = 0;
        error(0, 0, v16, v15);
      }
      if ( qword_9F8E0
        || (a1 = (char *)sub_5FD9E(v35), (int)sub_5FF90(a1, v17, qword_9F900, qword_9F908) < 0)
        || dword_9F87C && (a1 = (char *)sub_5FD84(v35), (int)sub_5FF90(a1, v18, qword_9F900, qword_9F908) < 0) )
      {
        v19 = 2;
      }
      else
      {
        v19 = 1;
      }
      *(int *)(v34 + 56) = v19;
    }
  }
  if ( byte_9F935 && *(long long *)(a2 + 408) && *v35 != *(long long *)(*(long long *)(a2 + 408) + 88LL) )
  {
    if ( (dword_9EC84 & 0x40000) != 0 )
    {
      if ( qword_9FDC8 )
        qword_9FDC8(a1);
      v20 = sub_5E548(*(long long *)(v34 + 88));
      v21 = gettext("%s: directory is on a different filesystem; not dumped");
      a1 = 0;
      error(0, 0, v21, v20);
    }
    *(int *)(v34 + 56) = 0;
    if ( *(long long *)(v34 + 40) )
    {
      a1 = *(char **)(v34 + 40);
      sub_209FC((void **)a1);
      *(long long *)(v34 + 40) = 0;
    }
    v31 = 0;
  }
  else if ( (a3 & 0x10) != 0 )
  {
    *(int *)(v34 + 56) = a3 & 3;
    if ( !*(int *)(v34 + 56) )
      *a4 = 78;
  }
  if ( v31 && (dword_9EC84 & 0x2000) != 0 )
  {
    if ( qword_9FDC8 )
      qword_9FDC8(a1);
    v22 = sub_5E548(s2);
    v23 = gettext("%s: Directory has been renamed");
    error(0, 0, v23, v22);
  }
  *(int *)(v34 + 60) |= 1u;
  if ( *(int *)(v34 + 56) )
  {
    v24 = sub_11FBF(a2, &v33);
    if ( v24 == 3 )
    {
      v25 = gettext("directory not dumped");
      sub_11F18((long long)s2, v33, (long long)v25);
      *a4 = 78;
      *(int *)(v34 + 56) = 0;
    }
    else if ( v24 <= 3 )
    {
      if ( v24 == 2 )
      {
        v27 = gettext("contents not dumped");
        sub_11F18((long long)s2, v33, (long long)v27);
        *(long long *)(v34 + 72) = v33;
      }
      else if ( v24 )
      {
        v26 = gettext("contents not dumped");
        sub_11F18((long long)s2, v33, (long long)v26);
        *(int *)(v34 + 56) = 0;
        *(long long *)(v34 + 72) = v33;
      }
    }
  }
  return v34;
}



// Function: makedumpdir @ 0x21b74
void makedumpdir(long long a1, char *a2, long long a3, long long a4, long long a5, long long a6)
{
  long long v6; // rax
  char *v7; // rax
  char *v8; // rdx
  char *v9; // rax
  char *v10; // [rsp+0h] [rbp-60h]
  long long *v11; // [rsp+8h] [rbp-58h]
  long long v12; // [rsp+18h] [rbp-48h]
  size_t i; // [rsp+18h] [rbp-48h]
  size_t nmemb; // [rsp+20h] [rbp-40h]
  long long v15; // [rsp+28h] [rbp-38h]
  long long v16; // [rsp+28h] [rbp-38h]
  char *s; // [rsp+30h] [rbp-30h]
  char *sa; // [rsp+30h] [rbp-30h]
  char *sb; // [rsp+30h] [rbp-30h]
  char *v20; // [rsp+38h] [rbp-28h]
  long long v21; // [rsp+40h] [rbp-20h]
  long long *base; // [rsp+48h] [rbp-18h]
  char *ptr; // [rsp+50h] [rbp-10h]

  if ( *(int *)(a1 + 56) == 2 )
  {
    v21 = 0;
  }
  else if ( *(long long *)(a1 + 64) )
  {
    if ( *(long long *)(*(long long *)(a1 + 64) + 48LL) )
      v6 = *(long long *)(*(long long *)(a1 + 64) + 48LL);
    else
      v6 = *(long long *)(*(long long *)(a1 + 64) + 40LL);
    v21 = v6;
  }
  else
  {
    v21 = *(long long *)(a1 + 40);
  }
  nmemb = 0;
  v15 = 0;
  s = a2;
  while ( *s )
  {
    v15 += strlen(s) + 2;
    s += strlen(s) + 1;
    ++nmemb;
  }
  v16 = v15 + 1;
  base = (long long *)sub_60C29(nmemb, 8, a3, a4, a5, a6, a2, a1);
  v12 = 0;
  sa = v10;
  while ( *sa )
  {
    base[v12] = sa;
    sa += strlen(sa) + 1;
    ++v12;
  }
  qsort(base, nmemb, 8u, (__compar_fn_t)compar);
  ptr = (char *)sub_60B5B(v16);
  v20 = ptr;
  for ( i = 0; i < nmemb; ++i )
  {
    if ( sub_20A6A(v21, base[i]) )
    {
      if ( v11[9] )
        *v20 = 73;
      else
        *v20 = 32;
      ++v20;
    }
    else
    {
      v7 = v20++;
      if ( v11[9] )
        *v7 = 73;
      else
        *v7 = 89;
    }
    sb = (char *)base[i];
    do
    {
      v8 = sb++;
      v9 = v20++;
      *v9 = *v8;
    }
    while ( *v9 );
  }
  *v20 = 0;
  v11[6] = v11[5];
  v11[5] = sub_2082E(ptr, 0);
  free(ptr);
  free(base);
}



// Function: maketagdumpdir @ 0x21e0d
void maketagdumpdir(long long a1)
{
  size_t v1; // rax
  size_t n; // [rsp+10h] [rbp-10h]
  char *ptr; // [rsp+18h] [rbp-8h]

  v1 = strlen(*(const char **)(a1 + 72));
  n = v1 + 1;
  ptr = (char *)sub_60B5B(v1 + 3);
  *ptr = 89;
  memcpy(ptr + 1, *(const void **)(a1 + 72), n);
  ptr[n + 1] = 0;
  *(long long *)(a1 + 48) = *(long long *)(a1 + 40);
  *(long long *)(a1 + 40) = sub_2082E(ptr, 0);
  free(ptr);
}



// Function: scan_directory @ 0x21eb9
long long scan_directory(long long a1)
{
  char v1; // di
  long long v2; // rcx
  long long v3; // r8
  long long v4; // r9
  long long v5; // rax
  long long v6; // rdx
  long long v7; // rax
  long long v8; // rdx
  char v11; // [rsp+12h] [rbp-22Eh] BYREF
  bool v12; // [rsp+13h] [rbp-22Dh]
  int v13; // [rsp+14h] [rbp-22Ch]
  int fd; // [rsp+18h] [rbp-228h]
  int v15; // [rsp+1Ch] [rbp-224h]
  void *v16; // [rsp+20h] [rbp-220h] BYREF
  char *i; // [rsp+28h] [rbp-218h]
  long long (*v18)(); // [rsp+30h] [rbp-210h]
  long long v19; // [rsp+38h] [rbp-208h]
  void *v20; // [rsp+40h] [rbp-200h]
  long long v21; // [rsp+48h] [rbp-1F8h]
  void *ptr; // [rsp+50h] [rbp-1F0h]
  long long v23; // [rsp+58h] [rbp-1E8h]
  long long v24; // [rsp+60h] [rbp-1E0h]
  char *v25; // [rsp+68h] [rbp-1D8h]
  char v26[88]; // [rsp+70h] [rbp-1D0h] BYREF
  struct stat v27[2]; // [rsp+C8h] [rbp-178h] BYREF
  long long v28; // [rsp+208h] [rbp-38h]
  int v29; // [rsp+218h] [rbp-28h]
  unsigned long long v30; // [rsp+228h] [rbp-18h]

  v30 = __readfsqword(0x28u);
  v19 = *(long long *)a1;
  v20 = (void *)sub_14D3D(a1);
  v21 = *(long long *)(a1 + 88);
  v12 = *(long long *)(a1 + 408) == 0;
  if ( !v20 )
    sub_3E72E(v19);
  sub_17909(a1);
  ptr = (void *)sub_60CC6(v19);
  sub_298D7(ptr);
  if ( v12 )
    v1 = 32;
  else
    v1 = 0;
  v23 = sub_213C6((char *)ptr, a1, v1, &v11);
  free(ptr);
  v24 = sub_2B66C(v19);
  if ( v20 )
  {
    if ( *(int *)(v23 + 56) )
    {
      sub_21B74(v23, (char *)v20, (long long)v20, v2, v3, v4);
      for ( i = sub_20B9F(*(long long *)(v23 + 40), 1, (long long *)&v16); i; i = sub_20AD3(v16) )
      {
        v25 = (char *)sub_2B75B(v24, i + 1);
        if ( *i == 73 )
        {
          *i = 78;
        }
        else if ( (unsigned char)sub_17C43(v25, a1) )
        {
          *i = 78;
        }
        else
        {
          fd = *(int *)(a1 + 424);
          v18 = 0;
          sub_39C55(v26);
          if ( fd >= 0 )
          {
            if ( fstatat(fd, i + 1, v27, flag) )
            {
              v18 = (long long (*)())sub_2B444;
            }
            else if ( (v27[0].st_mode & 0xF000) == 0x4000 )
            {
              v15 = sub_157F9(a1, i + 1, oflag);
              if ( v15 >= 0 )
              {
                v29 = v15;
                if ( fstat(v15, v27) )
                  v18 = (long long (*)())sub_2B444;
              }
              else
              {
                v18 = sub_2B2B1;
              }
            }
          }
          else
          {
            *__errno_location() = -fd;
            v18 = sub_2B2B1;
          }
          if ( v18 )
          {
            sub_2B48B(v25, 0, v18);
            *i = 78;
          }
          else if ( (v27[0].st_mode & 0xF000) == 0x4000 )
          {
            v13 = 0;
            if ( dword_9F930 )
            {
              if ( *(int *)(v23 + 56) == 2 )
                v13 |= 0x12u;
            }
            else
            {
              v13 |= 0x10u;
            }
            *i = 68;
            v28 = a1;
            sub_213C6(v25, (long long)v26, v13, i);
            sub_1588E((long long)v26);
          }
          else if ( !byte_9F935 || v21 == v27[0].st_dev )
          {
            if ( *i != 89 )
            {
              v5 = sub_5FD9E(v27);
              if ( (int)sub_5FF90(v5, v6, qword_9F900, qword_9F908) >= 0
                || dword_9F87C && (v7 = sub_5FD84(v27), (int)sub_5FF90(v7, v8, qword_9F900, qword_9F908) >= 0) )
              {
                *i = 89;
              }
              else
              {
                *i = 78;
              }
            }
          }
          else
          {
            *i = 78;
          }
          sub_39D1C(v26);
        }
      }
      free(v16);
    }
    else if ( *(long long *)(v23 + 72) )
    {
      sub_21E0D(v23);
    }
  }
  sub_2B72D(v24);
  free(v20);
  return v23;
}



// Function: directory_contents @ 0x2241b
long long directory_contents(long long a1)
{
  if ( !a1 )
    return 0;
  if ( *(long long *)(a1 + 40) )
    return **(long long **)(a1 + 40);
  return 0;
}



// Function: safe_directory_contents @ 0x22456
void *safe_directory_contents(long long a1)
{
  long long v2; // [rsp+10h] [rbp-8h]

  v2 = sub_2241B(a1);
  if ( v2 )
    return (void *)v2;
  else
    return &unk_81CA8;
}



// Function: obstack_code_rename @ 0x2248c
long long obstack_code_rename(long long a1, char *a2, char *a3)
{
  long long v3; // rax
  char *v4; // rax
  long long v5; // rax
  char *v6; // rax
  char *s; // [rsp+28h] [rbp-58h]
  char *sa; // [rsp+28h] [rbp-58h]
  size_t n; // [rsp+48h] [rbp-38h]
  size_t v12; // [rsp+70h] [rbp-10h]

  if ( *a2 )
    v3 = sub_3EDD9(a2, 0, (unsigned char)byte_9F878);
  else
    v3 = (long long)a2;
  s = (char *)v3;
  if ( *(long long *)(a1 + 32) == *(long long *)(a1 + 24) )
    obstack_newchunk(a1, 1);
  v4 = *(char **)(a1 + 24);
  *(long long *)(a1 + 24) = v4 + 1;
  *v4 = 82;
  n = strlen(s) + 1;
  if ( *(long long *)(a1 + 32) - *(long long *)(a1 + 24) < n )
    obstack_newchunk(a1, n);
  memcpy(*(void **)(a1 + 24), s, n);
  *(long long *)(a1 + 24) += n;
  if ( *a3 )
    v5 = sub_3EDD9(a3, 0, (unsigned char)byte_9F878);
  else
    v5 = (long long)a3;
  sa = (char *)v5;
  if ( *(long long *)(a1 + 32) == *(long long *)(a1 + 24) )
    obstack_newchunk(a1, 1);
  v6 = *(char **)(a1 + 24);
  *(long long *)(a1 + 24) = v6 + 1;
  *v6 = 84;
  v12 = strlen(sa) + 1;
  if ( *(long long *)(a1 + 32) - *(long long *)(a1 + 24) < v12 )
    obstack_newchunk(a1, v12);
  memcpy(*(void **)(a1 + 24), sa, v12);
  *(long long *)(a1 + 24) += v12;
  return a1;
}



// Function: store_rename @ 0x226b7
void store_rename(long long a1, long long a2)
{
  char *v2; // rax
  long long i; // [rsp+10h] [rbp-40h]
  long long k; // [rsp+18h] [rbp-38h]
  long long j; // [rsp+18h] [rbp-38h]
  char *s; // [rsp+20h] [rbp-30h]
  size_t n; // [rsp+40h] [rbp-10h]

  for ( i = a1; i && a1 != *(long long *)(i + 64); i = *(long long *)(i + 64) )
    ;
  if ( i )
  {
    s = (char *)sub_4E10C(*(long long *)(a1 + 88));
    if ( *(long long *)(a2 + 32) == *(long long *)(a2 + 24) )
      obstack_newchunk(a2, 1);
    v2 = *(char **)(a2 + 24);
    *(long long *)(a2 + 24) = v2 + 1;
    *v2 = 88;
    n = strlen(s) + 1;
    if ( *(long long *)(a2 + 32) - *(long long *)(a2 + 24) < n )
      obstack_newchunk(a2, n);
    memcpy(*(void **)(a2 + 24), s, n);
    *(long long *)(a2 + 24) += n;
    sub_2248C(a2, *(char **)(a1 + 88), byte_81BE4);
    for ( j = a1; j != i; j = *(long long *)(j + 64) )
      sub_2248C(a2, *(char **)(*(long long *)(j + 64) + 88LL), *(char **)(j + 88));
    sub_2248C(a2, byte_81BE4, *(char **)(i + 88));
    free(s);
  }
  else
  {
    for ( k = a1; k && *(long long *)(k + 64); k = *(long long *)(k + 64) )
      sub_2248C(a2, *(char **)(*(long long *)(k + 64) + 88LL), *(char **)(k + 88));
  }
}



// Function: append_incremental_renames @ 0x228cd
unsigned long long append_incremental_renames(long long a1)
{
  char *v1; // rax
  size_t v3; // [rsp+18h] [rbp-C8h]
  long long i; // [rsp+20h] [rbp-C0h]
  const char *src; // [rsp+28h] [rbp-B8h]
  char *object_base; // [rsp+68h] [rbp-78h]
  struct obstack v7; // [rsp+80h] [rbp-60h] BYREF
  unsigned long long v8; // [rsp+D8h] [rbp-8h]

  v8 = __readfsqword(0x28u);
  if ( qword_9F438 )
  {
    obstack_begin(&v7, 0, 0, sub_60B5B, &free);
    src = (const char *)sub_2241B(a1);
    if ( src )
    {
      v3 = sub_20BFE(src) - 1;
      if ( v7.chunk_limit - v7.next_free < v3 )
        obstack_newchunk(&v7, v3);
      memcpy(v7.next_free, src, v3);
      v7.next_free += v3;
    }
    else
    {
      v3 = 0;
    }
    for ( i = qword_9F438; i; i = *(long long *)i )
    {
      if ( (*(int *)(i + 60) & 0x10) != 0 )
        sub_226B7(i, (long long)&v7);
    }
    if ( a1 && v7.next_free - v7.object_base != v3 )
    {
      if ( v7.chunk_limit == v7.next_free )
        obstack_newchunk(&v7, 1);
      v1 = v7.next_free++;
      *v1 = 0;
      sub_209FC(*(void ***)(a1 + 40));
      object_base = v7.object_base;
      if ( v7.object_base == v7.next_free )
        *((char *)&v7 + 80) |= 2u;
      v7.next_free = (char *)((long long)&v7.next_free[*(long long *)&v7.alignment_mask] & ~*(long long *)&v7.alignment_mask);
      if ( (char *)(v7.next_free - (char *)v7.chunk) > (char *)(v7.chunk_limit - (char *)v7.chunk) )
        v7.next_free = v7.chunk_limit;
      v7.object_base = v7.next_free;
      *(long long *)(a1 + 40) = sub_209D4(object_base);
    }
    obstack_free(&v7, 0);
  }
  return v8 - __readfsqword(0x28u);
}



// Function: read_incr_db_01 @ 0x22c72
unsigned long long read_incr_db_01(int a1, const char *a2)
{
  long long v2; // rdx
  long long v3; // rdi
  char *v4; // r12
  const char *v5; // rbx
  int *v6; // rax
  char *v7; // rdi
  char *v8; // r12
  const char *v9; // rbx
  int *v10; // rax
  long long v11; // rdx
  long long v12; // rdi
  char *v13; // r12
  const char *v14; // rbx
  int *v15; // rax
  char *v16; // rdi
  char *v17; // r12
  const char *v18; // rbx
  int *v19; // rax
  char *v20; // rdi
  char *v21; // r12
  const char *v22; // rbx
  int *v23; // rax
  char *v24; // rdi
  char *v25; // r12
  const char *v26; // rbx
  int *v27; // rax
  unsigned char v30; // [rsp+13h] [rbp-7Dh]
  int v31; // [rsp+14h] [rbp-7Ch]
  char *lineptr; // [rsp+18h] [rbp-78h] BYREF
  size_t n; // [rsp+20h] [rbp-70h] BYREF
  char *endptr; // [rsp+28h] [rbp-68h] BYREF
  long long v35; // [rsp+30h] [rbp-60h]
  char *v36; // [rsp+38h] [rbp-58h]
  char *nptr; // [rsp+40h] [rbp-50h]
  uintmax_t v38; // [rsp+48h] [rbp-48h]
  long long v39; // [rsp+50h] [rbp-40h]
  long long v40; // [rsp+58h] [rbp-38h]
  long long v41; // [rsp+60h] [rbp-30h]
  long long v42; // [rsp+68h] [rbp-28h]
  unsigned long long v43; // [rsp+78h] [rbp-18h]

  v43 = __readfsqword(0x28u);
  lineptr = 0;
  n = 0;
  v35 = 1;
  if ( a1 == 1 )
  {
    if ( getline(&lineptr, &n, qword_9F458) <= 0 )
    {
      sub_3E403(qword_9F8E0);
      free(lineptr);
      return v43 - __readfsqword(0x28u);
    }
    ++v35;
  }
  else
  {
    lineptr = strdup(a2);
    n = strlen(lineptr) + 1;
  }
  qword_9F900 = sub_29FF7(lineptr, &endptr, 0);
  qword_9F908 = v2;
  v3 = qword_9F900;
  if ( (unsigned char)sub_294F0(qword_9F900, v2) != 1 )
  {
    if ( qword_9FDC8 )
      qword_9FDC8(v3);
    v4 = gettext("Invalid time stamp");
    v5 = (const char *)sub_5E548(qword_9F8E0);
    v6 = __errno_location();
    error(0, *v6, "%s:%ld: %s", v5, v35, v4);
    sub_177F4();
  }
  if ( a1 == 1 && *endptr )
  {
    nptr = endptr + 1;
    *__errno_location() = 0;
    v7 = nptr;
    v38 = strtoumax(nptr, &endptr, 10);
    if ( !*__errno_location() && v38 > 0x3B9AC9FF )
      *__errno_location() = 34;
    if ( *__errno_location() || nptr == endptr )
    {
      if ( qword_9FDC8 )
        qword_9FDC8(v7);
      v8 = gettext("Invalid time stamp");
      v9 = (const char *)sub_5E548(qword_9F8E0);
      v10 = __errno_location();
      error(0, *v10, "%s:%ld: %s", v9, v35, v8);
      dword_9FDD0 = 2;
      qword_9F900 = 0x8000000000000000LL;
      qword_9F908 = -1;
    }
    else
    {
      qword_9F908 = v38;
    }
  }
  while ( 1 )
  {
    v31 = getline(&lineptr, &n, qword_9F458);
    if ( v31 <= 0 )
      break;
    v30 = *lineptr == 43;
    v36 = &lineptr[v30];
    ++v35;
    if ( lineptr[v31 - 1] == 10 )
      lineptr[v31 - 1] = 0;
    if ( a1 == 1 )
    {
      v41 = sub_29FF7(v36, &endptr, 0);
      v42 = v11;
      v36 = endptr;
      v12 = v41;
      if ( (unsigned char)sub_294F0(v41, v11) != 1 || *v36 != 32 )
      {
        if ( qword_9FDC8 )
          qword_9FDC8(v12);
        v13 = gettext("Invalid modification time");
        v14 = (const char *)sub_5E548(qword_9F8E0);
        v15 = __errno_location();
        error(0, *v15, "%s:%ld: %s", v14, v35, v13);
        sub_177F4();
      }
      *__errno_location() = 0;
      v16 = v36;
      v38 = strtoumax(v36, &endptr, 10);
      if ( !*__errno_location() && v38 > 0x3B9AC9FF )
        *__errno_location() = 34;
      if ( *__errno_location() || v36 == endptr || *endptr != 32 )
      {
        if ( qword_9FDC8 )
          qword_9FDC8(v16);
        v17 = gettext("Invalid modification time (nanoseconds)");
        v18 = (const char *)sub_5E548(qword_9F8E0);
        v19 = __errno_location();
        error(0, *v19, "%s:%ld: %s", v18, v35, v17);
        sub_177F4();
      }
      v42 = v38;
      v36 = endptr;
    }
    else
    {
      v42 = 0;
      v41 = 0;
    }
    v20 = v36;
    v39 = sub_29CFC(v36, &endptr, 0, -1);
    v36 = endptr;
    if ( *__errno_location() || *v36 != 32 )
    {
      if ( qword_9FDC8 )
        qword_9FDC8(v20);
      v21 = gettext("Invalid device number");
      v22 = (const char *)sub_5E548(qword_9F8E0);
      v23 = __errno_location();
      error(0, *v23, "%s:%ld: %s", v22, v35, v21);
      sub_177F4();
    }
    v24 = v36;
    v40 = sub_29CFC(v36, &endptr, 0, -1);
    v36 = endptr;
    if ( *__errno_location() || *v36 != 32 )
    {
      if ( qword_9FDC8 )
        qword_9FDC8(v24);
      v25 = gettext("Invalid inode number");
      v26 = (const char *)sub_5E548(qword_9F8E0);
      v27 = __errno_location();
      error(0, *v27, "%s:%ld: %s", v26, v35, v25);
      sub_177F4();
    }
    sub_2961D(++v36);
    sub_21019(v36, v41, v42, v39, v40, v30, 0, 0);
  }
  free(lineptr);
  return v43 - __readfsqword(0x28u);
}



// Function: read_obstack @ 0x232e4
long long read_obstack(FILE *a1, long long a2, long long *a3)
{
  char *v3; // rax
  char *v4; // rax
  unsigned int v7; // [rsp+24h] [rbp-2Ch]
  long long v8; // [rsp+28h] [rbp-28h]

  v8 = 0;
  v7 = getc_unlocked(a1);
  while ( v7 != -1 && v7 )
  {
    if ( *(long long *)(a2 + 32) == *(long long *)(a2 + 24) )
      obstack_newchunk(a2, 1);
    v3 = *(char **)(a2 + 24);
    *(long long *)(a2 + 24) = v3 + 1;
    *v3 = v7;
    v7 = getc_unlocked(a1);
    ++v8;
  }
  if ( *(long long *)(a2 + 32) == *(long long *)(a2 + 24) )
    obstack_newchunk(a2, 1);
  v4 = *(char **)(a2 + 24);
  *(long long *)(a2 + 24) = v4 + 1;
  *v4 = 0;
  *a3 = v8;
  return v7;
}



// Function: read_num @ 0x233ed
long long read_num(FILE *a1, long long a2, long long a3, long long a4, long long *a5)
{
  __off_t v5; // rax
  long long v6; // r12
  long long v7; // rbx
  char *v8; // rax
  char *v9; // rbx
  const char *v10; // rax
  __off_t v12; // rax
  long long v13; // r12
  long long v14; // rbx
  char *v15; // rax
  long long v16; // rbx
  long long v17; // r14
  __off_t v18; // rax
  long long v19; // r13
  long long v20; // r12
  char *v21; // rax
  __off_t v22; // rax
  long long v23; // r12
  long long v24; // rbx
  char *v25; // rax
  FILE *stream; // [rsp+28h] [rbp-C8h]
  bool v30; // [rsp+3Fh] [rbp-B1h]
  int i; // [rsp+40h] [rbp-B0h]
  int v32; // [rsp+44h] [rbp-ACh]
  int errnum; // [rsp+48h] [rbp-A8h]
  char v34[32]; // [rsp+50h] [rbp-A0h] BYREF
  char v35[32]; // [rsp+70h] [rbp-80h] BYREF
  char v36[32]; // [rsp+90h] [rbp-60h] BYREF
  char v37[24]; // [rsp+B0h] [rbp-40h] BYREF
  unsigned long long v38; // [rsp+C8h] [rbp-28h]

  stream = a1;
  v38 = __readfsqword(0x28u);
  v32 = getc_unlocked(a1);
  v30 = v32 == 45;
  for ( i = 0; !i && v30 || (unsigned int)(v32 - 48) <= 9; ++i )
  {
    v34[i] = v32;
    if ( i == 20 )
    {
      if ( qword_9FDC8 )
        qword_9FDC8(a1);
      v5 = ftello(stream);
      v6 = sub_54D1C(v5, v35);
      v7 = sub_5E548(qword_9F8E0);
      v8 = gettext("%s: byte %s: %s %.*s... too long");
      error(0, 0, v8, v7, v6, a2, 21, v34);
      sub_177F4();
    }
    a1 = stream;
    v32 = getc_unlocked(stream);
  }
  v34[i] = 0;
  if ( v32 >= 0 )
  {
    if ( v32 )
    {
      if ( qword_9FDC8 )
        qword_9FDC8(a1);
      v12 = ftello(stream);
      v13 = sub_54D1C(v12, v35);
      v14 = sub_5E548(qword_9F8E0);
      v15 = gettext("%s: byte %s: %s %s followed by invalid byte 0x%02x");
      error(0, 0, v15, v14, v13, a2, v34, (unsigned int)v32);
      sub_177F4();
    }
    *a5 = sub_29CFC(v34, 0, a3, a4);
    errnum = *__errno_location();
    if ( errnum )
    {
      if ( errnum == 34 )
      {
        if ( qword_9FDC8 )
          qword_9FDC8(v34);
        v16 = sub_54E5A(a4, v37);
        v17 = sub_54BDE(a3, v36);
        v18 = ftello(stream);
        v19 = sub_54D1C(v18, v35);
        v20 = sub_5E548(qword_9F8E0);
        v21 = gettext("%s: byte %s: (valid range %s..%s)\n\t%s %s");
        error(0, 34, v21, v20, v19, v17, v16, a2, v34);
        sub_177F4();
      }
      if ( qword_9FDC8 )
        qword_9FDC8(v34);
      v22 = ftello(stream);
      v23 = sub_54D1C(v22, v35);
      v24 = sub_5E548(qword_9F8E0);
      v25 = gettext("%s: byte %s: %s %s");
      error(0, errnum, v25, v24, v23, a2, v34);
      sub_177F4();
    }
    return 1;
  }
  else
  {
    if ( ferror_unlocked(stream) )
      sub_3E5D6(qword_9F8E0);
    if ( i )
    {
      if ( qword_9FDC8 )
        qword_9FDC8(stream);
      v9 = gettext("Unexpected EOF in snapshot file");
      v10 = (const char *)sub_5E548(qword_9F8E0);
      error(0, 0, "%s: %s", v10, v9);
      sub_177F4();
    }
    return 0;
  }
}



// Function: read_timespec @ 0x238b6
unsigned long long read_timespec(FILE *a1, long long *a2)
{
  char *v3; // rbx
  const char *v4; // rax
  long long v5; // [rsp+18h] [rbp-28h] BYREF
  long long v6; // [rsp+20h] [rbp-20h] BYREF
  unsigned long long v7; // [rsp+28h] [rbp-18h]

  v7 = __readfsqword(0x28u);
  if ( !(unsigned char)sub_233ED(a1, (long long)"sec", 0x8000000000000000LL, 0x7FFFFFFFFFFFFFFFLL, &v5)
    || !(unsigned char)sub_233ED(a1, (long long)"nsec", 0, 999999999, &v6) )
  {
    if ( qword_9FDC8 )
      qword_9FDC8(a1);
    v3 = gettext("Unexpected EOF in snapshot file");
    v4 = (const char *)sub_5E548(qword_9F8E0);
    error(0, 0, "%s: %s", v4, v3);
    sub_177F4();
  }
  *a2 = v5;
  a2[1] = v6;
  return v7 - __readfsqword(0x28u);
}



// Function: read_incr_db_2 @ 0x239ca
unsigned long long read_incr_db_2()
{
  FILE *v0; // rdi
  FILE *v1; // rdi
  char *v2; // r13
  __off_t v3; // rax
  long long v4; // r12
  long long v5; // rbx
  char *v6; // rax
  char *v7; // rbx
  const char *v8; // rax
  char v10; // [rsp+Fh] [rbp-111h]
  long long v11; // [rsp+10h] [rbp-110h] BYREF
  unsigned long long v12; // [rsp+18h] [rbp-108h] BYREF
  long long v13; // [rsp+20h] [rbp-100h]
  long long v14; // [rsp+28h] [rbp-F8h]
  char *v15; // [rsp+30h] [rbp-F0h]
  const char *v16; // [rsp+38h] [rbp-E8h]
  const char *v17; // [rsp+40h] [rbp-E0h]
  char *v18; // [rsp+48h] [rbp-D8h]
  char *v19; // [rsp+50h] [rbp-D0h]
  char *v20; // [rsp+58h] [rbp-C8h]
  struct obstack *obstack; // [rsp+60h] [rbp-C0h]
  void *block; // [rsp+68h] [rbp-B8h]
  long long v23[2]; // [rsp+70h] [rbp-B0h] BYREF
  char v24[8]; // [rsp+80h] [rbp-A0h] BYREF
  unsigned long long v25; // [rsp+88h] [rbp-98h]
  char *v26; // [rsp+90h] [rbp-90h]
  char *v27; // [rsp+98h] [rbp-88h]
  char v28[24]; // [rsp+E0h] [rbp-40h] BYREF
  unsigned long long v29; // [rsp+F8h] [rbp-28h]

  v29 = __readfsqword(0x28u);
  obstack_begin(v24, 0, 0, sub_60B5B, &free);
  sub_238B6(qword_9F458, &qword_9F900);
  while ( (unsigned char)sub_233ED(qword_9F458, (long long)"nfs", 0, 1, &v11) == 1 )
  {
    v10 = v11 != 0;
    sub_238B6(qword_9F458, v23);
    v0 = qword_9F458;
    if ( (unsigned char)sub_233ED(qword_9F458, (long long)"dev", 0, -1, &v11) != 1
      || (v13 = v11, v0 = qword_9F458, (unsigned char)sub_233ED(qword_9F458, (long long)"ino", 0, -1, &v11) ^ 1)
      || (v14 = v11, v0 = qword_9F458, (unsigned int)sub_232E4(qword_9F458, (long long)v24, &v12)) )
    {
      if ( qword_9FDC8 )
        qword_9FDC8(v0);
      v7 = gettext("Unexpected EOF in snapshot file");
      v8 = (const char *)sub_5E548(qword_9F8E0);
      error(0, 0, "%s: %s", v8, v7);
      sub_177F4();
    }
    v15 = v24;
    v16 = v26;
    if ( v26 == v27 )
      v15[80] |= 2u;
    *((long long *)v15 + 3) = (*((long long *)v15 + 6) + *((long long *)v15 + 3)) & ~*((long long *)v15 + 6);
    if ( *((long long *)v15 + 3) - *((long long *)v15 + 1) > *((long long *)v15 + 4) - *((long long *)v15 + 1) )
      *((long long *)v15 + 3) = *((long long *)v15 + 4);
    *((long long *)v15 + 2) = *((long long *)v15 + 3);
    v17 = v16;
    while ( !(unsigned int)sub_232E4(qword_9F458, (long long)v24, &v12) && v12 > 1 )
      ;
    v1 = qword_9F458;
    if ( getc_unlocked(qword_9F458) )
    {
      if ( qword_9FDC8 )
        qword_9FDC8(v1);
      v2 = gettext("Missing record terminator");
      v3 = ftello(qword_9F458);
      v4 = sub_54D1C(v3, v28);
      v5 = sub_5E548(qword_9F8E0);
      v6 = gettext("%s: byte %s: %s");
      error(0, 0, v6, v5, v4, v2);
      sub_177F4();
    }
    v18 = v24;
    v19 = v26;
    if ( v26 == v27 )
      v18[80] |= 2u;
    *((long long *)v18 + 3) = (*((long long *)v18 + 6) + *((long long *)v18 + 3)) & ~*((long long *)v18 + 6);
    if ( *((long long *)v18 + 3) - *((long long *)v18 + 1) > *((long long *)v18 + 4) - *((long long *)v18 + 1) )
      *((long long *)v18 + 3) = *((long long *)v18 + 4);
    *((long long *)v18 + 2) = *((long long *)v18 + 3);
    v20 = v19;
    sub_21019(v17, v23[0], v23[1], v13, v14, v10, 0, v19);
    obstack = (struct obstack *)v24;
    block = v20;
    if ( (unsigned long long)v20 <= v25 || block >= obstack->chunk_limit )
    {
      obstack_free(obstack, block);
    }
    else
    {
      obstack->object_base = (char *)block;
      obstack->next_free = obstack->object_base;
    }
  }
  return v29 - __readfsqword(0x28u);
}



// Function: show_snapshot_field_ranges @ 0x23f6b
unsigned long long show_snapshot_field_ranges()
{
  const char *v0; // rbx
  const char *v1; // rax
  char **i; // [rsp+8h] [rbp-58h]
  char v4[32]; // [rsp+10h] [rbp-50h] BYREF
  char v5[24]; // [rsp+30h] [rbp-30h] BYREF
  unsigned long long v6; // [rsp+48h] [rbp-18h]

  v6 = __readfsqword(0x28u);
  printf("This tar's snapshot file field ranges are\n");
  printf("   (%-15s => [ %s, %s ]):\n\n", "field name", "min", "max");
  for ( i = &off_9A640; *i; i += 3 )
  {
    v0 = (const char *)sub_29C96(i[2], i[1], i[2], v5);
    v1 = (const char *)sub_29C96(i[1], i[1], i[2], v4);
    printf("    %-15s => [ %s, %s ],\n", *i, v1, v0);
  }
  printf("\n");
  return v6 - __readfsqword(0x28u);
}



// Function: read_directory_file @ 0x24090
unsigned long long read_directory_file()
{
  char *v0; // rdi
  char *v1; // rax
  char *v2; // rax
  char *v3; // rax
  int oflag; // [rsp+0h] [rbp-30h]
  int fd; // [rsp+4h] [rbp-2Ch]
  char *lineptr; // [rsp+8h] [rbp-28h] BYREF
  size_t n; // [rsp+10h] [rbp-20h] BYREF
  char *v9; // [rsp+18h] [rbp-18h]
  uintmax_t v10; // [rsp+20h] [rbp-10h]
  unsigned long long v11; // [rsp+28h] [rbp-8h]

  v11 = __readfsqword(0x28u);
  lineptr = 0;
  n = 0;
  oflag = 66;
  if ( !dword_9F8E8 )
    oflag = 578;
  fd = open(qword_9F8E0, oflag, 438);
  if ( fd >= 0 )
  {
    qword_9F458 = fdopen(fd, "r+");
    if ( qword_9F458 )
    {
      sub_2EDA0();
      sub_2EE43();
      if ( getline(&lineptr, &n, qword_9F458) > 0 )
      {
        v0 = lineptr;
        if ( !strncmp(lineptr, "GNU tar", 7u) )
        {
          v9 = lineptr + 8;
          if ( lineptr[7] != 45 )
          {
            if ( qword_9FDC8 )
              qword_9FDC8(v0);
            v1 = gettext("Bad incremental file format");
            v0 = (char *)(&dword_0 + 1);
            error(1, 0, v1);
            dword_9FDD0 = 2;
          }
          while ( *v9 != 45 )
          {
            if ( !*v9 )
            {
              if ( qword_9FDC8 )
                qword_9FDC8(v0);
              v2 = gettext("Bad incremental file format");
              v0 = (char *)(&dword_0 + 1);
              error(1, 0, v2);
              dword_9FDD0 = 2;
            }
            ++v9;
          }
          v0 = v9 + 1;
          v10 = strtoumax(v9 + 1, 0, 10);
        }
        else
        {
          v10 = 0;
        }
        if ( v10 <= 1 )
        {
          sub_22C72(v10, lineptr);
        }
        else if ( v10 == 2 )
        {
          sub_239CA();
        }
        else
        {
          if ( qword_9FDC8 )
            qword_9FDC8(v0);
          v3 = gettext("Unsupported incremental format version: %lu");
          error(1, 0, v3, v10);
          dword_9FDD0 = 2;
        }
      }
      if ( ferror_unlocked(qword_9F458) )
        sub_3E403(qword_9F8E0);
      free(lineptr);
    }
    else
    {
      sub_3E38B(qword_9F8E0);
      close(fd);
    }
  }
  else
  {
    sub_3E38B(qword_9F8E0);
  }
  return v11 - __readfsqword(0x28u);
}



// Function: write_directory_file_entry @ 0x24350
bool write_directory_file_entry(long long a1, FILE *a2)
{
  short *v2; // rax
  size_t v3; // rax
  size_t v4; // rax
  size_t v5; // rax
  size_t v6; // rax
  size_t v7; // rax
  size_t v8; // rax
  void *v10; // [rsp+18h] [rbp-48h] BYREF
  char *s; // [rsp+20h] [rbp-40h]
  long long v12; // [rsp+28h] [rbp-38h]
  FILE *stream; // [rsp+30h] [rbp-30h]
  void *ptr; // [rsp+38h] [rbp-28h]
  char v15[24]; // [rsp+40h] [rbp-20h] BYREF
  unsigned long long v16; // [rsp+58h] [rbp-8h]

  v16 = __readfsqword(0x28u);
  v12 = a1;
  stream = a2;
  if ( (*(int *)(a1 + 60) & 4) != 0 )
  {
    if ( (*(int *)(v12 + 60) & 2) != 0 )
      v2 = L"10";
    else
      v2 = L"0";
    ptr = v2;
    fwrite_unlocked(v2, 2u, 1u, stream);
    ptr = (void *)sub_29C96(*(long long *)(v12 + 8), 0x8000000000000000LL, 0x7FFFFFFFFFFFFFFFLL, v15);
    v3 = strlen((const char *)ptr);
    fwrite_unlocked(ptr, v3 + 1, 1u, stream);
    ptr = (void *)sub_54BDE(*(long long *)(v12 + 16), v15);
    v4 = strlen((const char *)ptr);
    fwrite_unlocked(ptr, v4 + 1, 1u, stream);
    ptr = (void *)sub_29C96(*(long long *)(v12 + 24), 0, -1, v15);
    v5 = strlen((const char *)ptr);
    fwrite_unlocked(ptr, v5 + 1, 1u, stream);
    ptr = (void *)sub_29C96(*(long long *)(v12 + 32), 0, -1, v15);
    v6 = strlen((const char *)ptr);
    fwrite_unlocked(ptr, v6 + 1, 1u, stream);
    v7 = strlen(*(const char **)(v12 + 88));
    fwrite_unlocked(*(const void **)(v12 + 88), v7 + 1, 1u, stream);
    if ( *(long long *)(v12 + 40) )
    {
      for ( s = sub_20B9F(*(long long *)(v12 + 40), 0, (long long *)&v10); s; s = sub_20AD3(v10) )
      {
        v8 = strlen(s);
        fwrite_unlocked(s, v8 + 1, 1u, stream);
      }
      free(v10);
    }
    fwrite_unlocked(L"", 2u, 1u, stream);
  }
  return ferror_unlocked(stream) == 0;
}



// Function: write_directory_file @ 0x24600
unsigned long long write_directory_file()
{
  unsigned int v0; // eax
  size_t v1; // rax
  size_t v2; // rax
  FILE *stream; // [rsp+0h] [rbp-30h]
  char *s; // [rsp+8h] [rbp-28h]
  char *sa; // [rsp+8h] [rbp-28h]
  char v7[24]; // [rsp+10h] [rbp-20h] BYREF
  unsigned long long v8; // [rsp+28h] [rbp-8h]

  v8 = __readfsqword(0x28u);
  stream = qword_9F458;
  if ( qword_9F458 )
  {
    if ( fseeko(qword_9F458, 0, 0) )
      sub_3E780(qword_9F8E0);
    v0 = fileno(stream);
    if ( (unsigned int)sub_32F52(v0) )
      sub_3EA55(qword_9F8E0);
    fprintf(stream, "%s-%s-%d\n", "GNU tar", "1.34", 2);
    s = (char *)sub_54BDE(qword_9F9E0, v7);
    v1 = strlen(s);
    fwrite_unlocked(s, v1 + 1, 1u, stream);
    sa = (char *)sub_54E5A(qword_9F9E8, v7);
    v2 = strlen(sa);
    fwrite_unlocked(sa, v2 + 1, 1u, stream);
    if ( !ferror_unlocked(stream) && qword_9F448 )
      sub_5279C(qword_9F448, sub_24350, stream);
    if ( ferror_unlocked(stream) )
      sub_3EB22(qword_9F8E0);
    if ( fclose(stream) )
      sub_3E1FF(qword_9F8E0);
  }
  return v8 - __readfsqword(0x28u);
}



// Function: get_gnu_dumpdir @ 0x247c6
long long get_gnu_dumpdir(long long a1)
{
  char *v1; // rax
  unsigned long long v3; // [rsp+18h] [rbp-28h]
  size_t n; // [rsp+20h] [rbp-20h]
  char *dest; // [rsp+28h] [rbp-18h]
  long long v6; // [rsp+30h] [rbp-10h]
  const void *src; // [rsp+38h] [rbp-8h]

  v3 = *(long long *)(a1 + 136);
  v6 = sub_60B5B(v3);
  dest = (char *)v6;
  sub_BBA7(qword_9F460);
  sub_AD2B((long long *)a1);
  while ( v3 )
  {
    sub_AD86(v3);
    src = sub_BB4B();
    if ( !src )
    {
      if ( qword_9FDC8 )
        qword_9FDC8(v3);
      v1 = gettext("Unexpected EOF in archive");
      error(1, 0, v1);
      dword_9FDD0 = 2;
    }
    n = (size_t)sub_BBF5((long long)src);
    if ( n > v3 )
      n = v3;
    memcpy(dest, src, n);
    dest += n;
    sub_BBA7((unsigned long long)src + n - 1);
    v3 -= n;
  }
  sub_AD66();
  *(long long *)(a1 + 400) = v6;
  *(char *)(a1 + 393) = 1;
  return a1;
}



// Function: is_dumpdir @ 0x2490b
long long is_dumpdir(long long a1)
{
  if ( *(char *)(a1 + 392) && !*(long long *)(a1 + 400) )
    sub_247C6(a1);
  return *(unsigned char *)(a1 + 392);
}



// Function: dumpdir_ok @ 0x24953
long long dumpdir_ok(char *a1)
{
  char *v1; // rax
  char *v3; // rax
  char *v4; // rax
  char *v5; // rax
  char *v6; // rax
  char *v7; // rax
  char *v8; // rax
  unsigned char v9; // [rsp+1Fh] [rbp-21h]
  int v10; // [rsp+20h] [rbp-20h]
  unsigned int v11; // [rsp+24h] [rbp-1Ch]
  char *s; // [rsp+28h] [rbp-18h]

  v10 = 0;
  v11 = 0;
  s = a1;
  while ( 2 )
  {
    if ( !*s )
    {
      if ( v11 )
      {
        if ( qword_9FDC8 )
          qword_9FDC8(a1);
        v7 = gettext("Malformed dumpdir: expected '%c' but found end of data");
        error(0, 0, v7, v11);
        dword_9FDD0 = 2;
        return 0;
      }
      else
      {
        if ( v10 && (dword_9EC84 & 2) != 0 )
        {
          if ( qword_9FDC8 )
            qword_9FDC8(a1);
          v8 = gettext("Malformed dumpdir: 'X' never used");
          error(0, 0, v8);
        }
        return 1;
      }
    }
    if ( v11 && v11 != *s )
    {
      v9 = *s;
      if ( qword_9FDC8 )
        qword_9FDC8(a1);
      v1 = gettext("Malformed dumpdir: expected '%c' but found %#3o");
      error(0, 0, v1, v11, v9);
      dword_9FDD0 = 2;
      return 0;
    }
    switch ( *s )
    {
      case 'R':
        if ( s[1] )
          goto LABEL_19;
        if ( v10 )
        {
          v10 = 0;
LABEL_19:
          v11 = 84;
          goto LABEL_30;
        }
        if ( qword_9FDC8 )
          qword_9FDC8(a1);
        v4 = gettext("Malformed dumpdir: empty name in 'R'");
        error(0, 0, v4);
        dword_9FDD0 = 2;
        return 0;
      case 'T':
        if ( v11 != 84 )
        {
          if ( qword_9FDC8 )
            qword_9FDC8(a1);
          v5 = gettext("Malformed dumpdir: 'T' not preceded by 'R'");
          error(0, 0, v5);
          dword_9FDD0 = 2;
          return 0;
        }
        if ( !s[1] && !v10 )
        {
          if ( qword_9FDC8 )
            qword_9FDC8(a1);
          v6 = gettext("Malformed dumpdir: empty name in 'T'");
          error(0, 0, v6);
          dword_9FDD0 = 2;
          return 0;
        }
        v11 = 0;
        goto LABEL_30;
      case 'X':
        if ( !v10 )
        {
          v10 = 1;
LABEL_30:
          a1 = s;
          s += strlen(s) + 1;
          continue;
        }
        if ( qword_9FDC8 )
          qword_9FDC8(a1);
        v3 = gettext("Malformed dumpdir: 'X' duplicated");
        error(0, 0, v3);
        dword_9FDD0 = 2;
        return 0;
      default:
        goto LABEL_30;
    }
  }
}



// Function: try_purge_directory @ 0x24ca0
long long try_purge_directory(long long a1)
{
  long long v2; // r12
  char *v3; // rbx
  int *v4; // rax
  char v5; // al
  long long v6; // rbx
  char *v7; // rax
  long long v8; // rbx
  char *v9; // rax
  long long v10; // r12
  long long v11; // rbx
  char *v12; // rax
  long long v13; // rbx
  char *v14; // rax
  int errnum; // [rsp+1Ch] [rbp-104h]
  char *i; // [rsp+20h] [rbp-100h]
  char *s; // [rsp+28h] [rbp-F8h]
  void *v18; // [rsp+30h] [rbp-F0h]
  char *dest; // [rsp+38h] [rbp-E8h]
  const char *v20; // [rsp+40h] [rbp-E0h]
  const char *v21; // [rsp+48h] [rbp-D8h]
  char *ptr; // [rsp+50h] [rbp-D0h]
  void **v23; // [rsp+58h] [rbp-C8h]
  char *v24; // [rsp+60h] [rbp-C0h]
  size_t n; // [rsp+68h] [rbp-B8h]
  long long v26[3]; // [rsp+70h] [rbp-B0h] BYREF
  int v27; // [rsp+88h] [rbp-98h]
  unsigned long long v28; // [rsp+108h] [rbp-18h]

  v28 = __readfsqword(0x28u);
  dest = 0;
  if ( (unsigned char)sub_2490B((long long)&qword_9FA20) != 1 )
    return 0;
  ptr = (char *)sub_2B8E1(a1, 0);
  if ( !ptr )
    return 0;
  if ( (unsigned char)sub_24953((char *)qword_9FBB0) != 1 )
    return 0;
  for ( s = (char *)qword_9FBB0; ; s += strlen(s) + 1 )
  {
    if ( !*s )
    {
      free(dest);
      v23 = (void **)sub_209D4((char *)qword_9FBB0);
      v18 = 0;
      for ( i = ptr; *i; i += strlen(i) + 1 )
      {
        free(v18);
        v18 = (void *)sub_2EE85(a1, i);
        if ( (unsigned int)sub_2A9B8(v18, v26) )
        {
          if ( *__errno_location() != 2 )
          {
            sub_2B444(v18);
            if ( qword_9FDC8 )
              qword_9FDC8(v18);
            v6 = sub_5E548(v18);
            v7 = gettext("%s: Not purging directory: unable to stat");
            error(0, 0, v7, v6);
          }
        }
        else
        {
          v24 = (char *)sub_20A6A((long long)v23, (long long)i);
          if ( !v24 || *v24 == 68 && (v27 & 0xF000) != 0x4000 || *v24 == 89 && (v27 & 0xF000) == 0x4000 )
          {
            if ( !byte_9F935 || v26[0] == qword_9FC20 )
            {
              if ( byte_9F8C8 != 1 || (unsigned int)sub_34AF4("delete", v18) )
              {
                if ( dword_9F9AC )
                {
                  v10 = sub_5E84E(v18);
                  v11 = qword_9FE48;
                  v12 = gettext("%s: Deleting %s\n");
                  fprintf(stream, v12, v11, v10);
                }
                if ( !(unsigned int)sub_2A36F(v18, 1) )
                {
                  errnum = *__errno_location();
                  if ( qword_9FDC8 )
                    qword_9FDC8(v18);
                  v13 = sub_5E548(v18);
                  v14 = gettext("%s: Cannot remove");
                  error(0, errnum, v14, v13);
                  dword_9FDD0 = 2;
                }
              }
            }
            else
            {
              if ( qword_9FDC8 )
                qword_9FDC8(v23);
              v8 = sub_5E548(v18);
              v9 = gettext("%s: directory is on a different device: not purging");
              error(0, 0, v9, v8);
            }
          }
        }
      }
      free(v18);
      sub_209FC(v23);
      free(ptr);
      return 1;
    }
    if ( *s == 88 )
      break;
    if ( *s == 82 )
    {
      v20 = s + 1;
      s += strlen(s) + 1;
      v21 = s + 1;
      if ( *v20 )
        v20 = (const char *)sub_3EDD9(v20, 0, (unsigned char)byte_9F878);
      if ( *v21 )
        v21 = (const char *)sub_3EDD9(v21, 0, (unsigned char)byte_9F878);
      if ( *v20 )
      {
        if ( !*v21 )
          v21 = dest;
        v5 = sub_1C326(v20, v21);
      }
      else
      {
        v5 = sub_1C326(dest, v21);
      }
      if ( v5 != 1 )
      {
        free(dest);
        free(ptr);
        return 0;
      }
    }
LABEL_25:
    ;
  }
  n = strlen(s + 1);
  dest = (char *)sub_60B8D(dest, n + 12);
  memcpy(dest, s + 1, n);
  dest[n] = 47;
  memcpy(&dest[n + 1], "tar.XXXXXX", 0xBu);
  if ( mkdtemp(dest) )
    goto LABEL_25;
  if ( qword_9FDC8 )
    qword_9FDC8(dest);
  v2 = sub_5E84E(dest);
  v3 = gettext("Cannot create temporary directory using template %s");
  v4 = __errno_location();
  error(0, *v4, v3, v2);
  dword_9FDD0 = 2;
  free(dest);
  free(ptr);
  return 0;
}



// Function: purge_directory @ 0x25377
long long purge_directory(long long a1)
{
  long long result; // rax

  result = (unsigned int)sub_24CA0(a1) ^ 1;
  if ( (char)result )
    return sub_28891();
  return result;
}



// Function: list_dumpdir @ 0x253a2
void list_dumpdir(char *a1, long long a2)
{
  int v2; // eax
  int v5; // [rsp+1Ch] [rbp-4h]

  v5 = 0;
  while ( a2 )
  {
    v2 = (char)*a1;
    if ( *a1 )
    {
      if ( (unsigned int)v2 <= 0x59 && v2 >= 68 && ((0x314401uLL >> ((unsigned char)v2 - 68)) & 1) != 0 )
      {
        fprintf(stream, "%c", (unsigned int)(char)*a1);
        if ( !v5 )
        {
          fprintf(stream, " ");
          v5 = 1;
        }
        ++a1;
        --a2;
      }
      else
      {
        fputc_unlocked((char)*a1++, stream);
        --a2;
      }
    }
    else
    {
      fputc_unlocked(10, stream);
      ++a1;
      --a2;
      v5 = 0;
    }
  }
}



// Function: base64_init @ 0x254c9
void *base64_init()
{
  void *result; // rax
  int i; // [rsp+Ch] [rbp-4h]

  result = memset(byte_9F4A0, 64, sizeof(byte_9F4A0));
  for ( i = 0; i <= 63; ++i )
  {
    result = (void *)aAbcdefghijklmn[i];
    byte_9F4A0[(long long)result] = i;
  }
  return result;
}



// Function: decode_xform @ 0x25529
char *decode_xform(char *a1, int *a2)
{
  char *s; // [rsp+8h] [rbp-18h]
  int v4; // [rsp+14h] [rbp-Ch]
  size_t v5; // [rsp+18h] [rbp-8h]

  s = a1;
  v4 = *a2;
  if ( *a2 == 4 )
    return a1;
  if ( v4 <= 4 )
  {
    if ( v4 == 1 )
    {
      s = (char *)sub_3EDD9(a1, 0, (unsigned char)byte_9F878);
    }
    else if ( v4 == 2 )
    {
      s = (char *)sub_3EDD9(a1, 1, (unsigned char)byte_9F878);
    }
  }
  if ( qword_9F970 )
  {
    v5 = sub_2EF5C(s, qword_9F970);
    if ( v5 == -1 )
      v5 = strlen(s);
    s += v5;
  }
  return s;
}



// Function: transform_member_name @ 0x255f5
long long transform_member_name(long long a1, unsigned int a2)
{
  unsigned int v3; // [rsp+4h] [rbp-Ch] BYREF
  long long v4; // [rsp+8h] [rbp-8h]

  v4 = a1;
  v3 = a2;
  return sub_3B8AF(a1, a2, sub_25529, &v3);
}



// Function: enforce_one_top_level @ 0x25627
void enforce_one_top_level(char **a1)
{
  int v1; // [rsp+1Ch] [rbp-14h]
  char *s1; // [rsp+20h] [rbp-10h]
  char *ptr; // [rsp+28h] [rbp-8h]

  ptr = *a1;
  for ( s1 = *a1; *s1 && (*s1 == 47 || *s1 == 46); ++s1 )
    ;
  if ( !*s1 )
  {
    *a1 = (char *)sub_60CC6(qword_9F938);
    goto LABEL_12;
  }
  v1 = strlen(qword_9F938);
  if ( strncmp(s1, qword_9F938, v1) || s1[v1] != 47 && s1[v1] )
  {
    *a1 = (char *)sub_2EE85(qword_9F938, ptr);
    sub_29946(*a1);
LABEL_12:
    free(ptr);
  }
}



// Function: transform_stat_info @ 0x25733
void transform_stat_info(int a1, long long a2)
{
  if ( a1 != 86 )
  {
    sub_255F5(a2 + 8, 1u);
    if ( a1 == 49 )
    {
      sub_255F5(a2 + 24, 2u);
    }
    else if ( a1 == 50 )
    {
      sub_255F5(a2 + 24, 4u);
    }
    if ( byte_9F936 )
      sub_25627(&file);
  }
}



// Function: read_and @ 0x257b8
unsigned long long read_and(void (*a1)(void))
{
  char *v1; // rdi
  int v2; // eax
  long long v3; // rbx
  char *v4; // rax
  signed long long v5; // rax
  long long v6; // rbx
  char *v7; // rax
  signed long long v8; // rax
  long long v9; // rbx
  char *v10; // rax
  signed long long v11; // rax
  long long v12; // rbx
  char *v13; // rax
  FILE *v14; // rdi
  char *v15; // rax
  long long v16; // rbx
  char *v17; // rax
  char *v18; // rax
  int v21; // [rsp+10h] [rbp-50h]
  int v22; // [rsp+14h] [rbp-4Ch]
  signed long long v23; // [rsp+18h] [rbp-48h]
  char v24[24]; // [rsp+30h] [rbp-30h] BYREF
  unsigned long long v25; // [rsp+48h] [rbp-18h]

  v25 = __readfsqword(0x28u);
  v21 = 0;
  sub_254C9();
  sub_2D3B1();
  sub_F694(0);
  while ( 2 )
  {
    v22 = v21;
    sub_39D1C(&qword_9FA20);
    v21 = sub_25ECF(&qword_9F460, &qword_9FA20, 0);
    switch ( v21 )
    {
      case 0:
      case 2:
        abort();
      case 1:
        sub_26747(qword_9F460, &qword_9FA20, &dword_9F468, 1);
        v1 = file;
        if ( (unsigned char)sub_2D88A(file) == 1 )
        {
          if ( qword_9F908 < 0
            || (qword_9FB18 = sub_279D7(qword_9F460 + 136, 12),
                qword_9FB20 = 0,
                v1 = (char *)qword_9FB18,
                (int)sub_5FF90(qword_9FB18, 0, qword_9F900, qword_9F908) >= 0) )
          {
            v1 = file;
            if ( !(unsigned char)sub_17C43(file, qword_9FBB8) )
              goto LABEL_18;
          }
        }
        v2 = *(char *)(qword_9F460 + 156);
        if ( v2 == 86 )
          goto LABEL_18;
        if ( v2 > 86 )
          goto LABEL_17;
        if ( v2 == 53 )
        {
          if ( byte_9F978 )
          {
            if ( qword_9FDC8 )
              qword_9FDC8(v1);
            v3 = sub_5E548(file);
            v4 = gettext("%s: Omitting");
            error(0, 0, v4, v3);
          }
LABEL_17:
          sub_28891();
          goto LABEL_43;
        }
        if ( v2 != 77 )
          goto LABEL_17;
LABEL_18:
        sub_25733(*(char *)(qword_9F460 + 156), (long long)&qword_9FA20);
        a1();
        goto LABEL_43;
      case 3:
        if ( byte_9F88C )
        {
          v5 = sub_BACD();
          v6 = sub_54E5A(v5, v24);
          v7 = gettext("block %s: ** Block of NULs **\n");
          fprintf(stream, v7, v6);
        }
        sub_BBA7(qword_9F460);
        if ( byte_9F8BD == 1 )
        {
          v21 = v22;
LABEL_43:
          if ( (unsigned char)sub_2DA15(&qword_9FA20) == 1 )
            goto LABEL_44;
          continue;
        }
        if ( (unsigned int)sub_25ECF(&qword_9F460, &qword_9FA20, 0) != 3 && (dword_9EC84 & 1) != 0 )
        {
          if ( qword_9FDC8 )
            qword_9FDC8(&qword_9F460);
          v8 = sub_BACD();
          v9 = sub_54E5A(v8, v24);
          v10 = gettext("A lone zero block at %s");
          error(0, 0, v10, v9);
        }
LABEL_44:
        sub_D046();
        sub_2DBBA();
        return v25 - __readfsqword(0x28u);
      case 4:
        if ( byte_9F88C )
        {
          v11 = sub_BACD();
          v12 = sub_54E5A(v11, v24);
          v13 = gettext("block %s: ** End of File **\n");
          fprintf(stream, v13, v12);
        }
        goto LABEL_44;
      case 5:
        v14 = (FILE *)qword_9F460;
        sub_BBA7(qword_9F460);
        switch ( v22 )
        {
          case 0:
            if ( qword_9FDC8 )
              qword_9FDC8(v14);
            v15 = gettext("This does not look like a tar archive");
            v14 = 0;
            error(0, 0, v15);
            dword_9FDD0 = 2;
            goto LABEL_34;
          case 1:
          case 3:
LABEL_34:
            if ( byte_9F88C )
            {
              v23 = sub_BACD() - qword_9F480 - qword_9F488;
              v16 = sub_54E5A(v23, v24);
              v17 = gettext("block %s: ");
              v14 = stream;
              fprintf(stream, v17, v16);
            }
            if ( qword_9FDC8 )
              qword_9FDC8(v14);
            v18 = gettext("Skipping to next header");
            error(0, 0, v18);
            dword_9FDD0 = 2;
            break;
          case 2:
            abort();
          default:
            goto LABEL_43;
        }
        goto LABEL_43;
      default:
        goto LABEL_44;
    }
  }
}



// Function: list_archive @ 0x25cf3
long long list_archive()
{
  long long v0; // rsi
  signed long long v2; // [rsp+8h] [rbp-8h]

  v2 = sub_BACD();
  if ( dword_9F9AC )
    sub_2864C(&qword_9FA20, qword_9F460, v2);
  if ( byte_9F8BE && dword_9F9AC > 2 && (unsigned char)sub_2490B((long long)&qword_9FA20) )
  {
    v0 = sub_20BFE((const char *)qword_9FBB0);
    sub_253A2((char *)qword_9FBB0, v0);
  }
  return sub_28891();
}



// Function: tar_checksum @ 0x25d84
long long tar_checksum(unsigned char *a1, unsigned char a2)
{
  unsigned char *v2; // rax
  int v6; // [rsp+10h] [rbp-20h]
  int v7; // [rsp+10h] [rbp-20h]
  int v8; // [rsp+14h] [rbp-1Ch]
  int v9; // [rsp+14h] [rbp-1Ch]
  int v10; // [rsp+18h] [rbp-18h]
  long long v11; // [rsp+20h] [rbp-10h]
  long long i; // [rsp+20h] [rbp-10h]
  unsigned char *v13; // [rsp+28h] [rbp-8h]

  v6 = 0;
  v8 = 0;
  v13 = a1;
  v11 = 512;
  while ( v11-- )
  {
    v6 += *v13;
    v2 = v13++;
    v8 += (char)*v2;
  }
  if ( !v6 )
    return 3;
  for ( i = 8; i--; v8 -= (char)a1[i + 148] )
    v6 -= a1[i + 148];
  v7 = v6 + 256;
  v9 = v8 + 256;
  v10 = sub_26C90((int)a1 + 148, 8, 0, 0, 0x7FFFFFFF, 1, a2);
  if ( v10 < 0 )
    return 5;
  if ( v7 == v10 || v9 == v10 )
    return 1;
  return 5;
}



// Function: read_header @ 0x25ecf
long long read_header(long long *a1, long long a2, int a3)
{
  char *v3; // rdi
  unsigned int v4; // ecx
  unsigned long long v5; // rdi
  char *v6; // rax
  long long v7; // rax
  long long v8; // rax
  char *v9; // rax
  unsigned int v13; // [rsp+24h] [rbp-1ACh]
  char *dest; // [rsp+28h] [rbp-1A8h]
  unsigned long long v15; // [rsp+30h] [rbp-1A0h]
  size_t i; // [rsp+30h] [rbp-1A0h]
  size_t n; // [rsp+38h] [rbp-198h]
  char *ptr; // [rsp+40h] [rbp-190h]
  char *v19; // [rsp+48h] [rbp-188h]
  long long v20; // [rsp+50h] [rbp-180h]
  long long v21; // [rsp+58h] [rbp-178h]
  char *v22; // [rsp+60h] [rbp-170h]
  char *v23; // [rsp+60h] [rbp-170h]
  char *v24; // [rsp+68h] [rbp-168h]
  char *v25; // [rsp+70h] [rbp-160h]
  unsigned long long v26; // [rsp+80h] [rbp-150h]
  char *v27; // [rsp+90h] [rbp-140h]
  const void *src; // [rsp+98h] [rbp-138h]
  char s[32]; // [rsp+A0h] [rbp-130h] BYREF
  char v30[264]; // [rsp+C0h] [rbp-110h] BYREF
  unsigned long long v31; // [rsp+1C8h] [rbp-8h]

  v31 = __readfsqword(0x28u);
  ptr = 0;
  v19 = 0;
  v20 = 0;
  v21 = 0;
  while ( 1 )
  {
    v25 = (char *)sub_BB4B();
    *a1 = v25;
    if ( !v25 )
    {
      v13 = 4;
      goto LABEL_53;
    }
    v13 = sub_25D84((unsigned char *)v25, 0);
    if ( v13 != 1 )
      goto LABEL_53;
    if ( v25[156] == 49 )
    {
      *(long long *)(a2 + 136) = 0;
    }
    else
    {
      *(long long *)(a2 + 136) = sub_2798B(v25 + 124, 12);
      if ( *(long long *)(a2 + 136) < 0 )
      {
        v13 = 5;
        goto LABEL_53;
      }
    }
    if ( v25[156] != 76 && v25[156] != 75 && v25[156] != 120 && v25[156] != 103 && v25[156] != 88 )
      break;
    if ( a3 == 1 )
    {
      v13 = 2;
      goto LABEL_53;
    }
    switch ( v25[156] )
    {
      case 'L':
      case 'K':
        v26 = *(long long *)(a2 + 136);
        v15 = v26 + 512;
        if ( (v26 & 0x1FF) != 0 )
          v15 = v15 - (*(long long *)(a2 + 136) & 0x1FFLL) + 512;
        if ( v15 < v26 )
          sub_1783C();
        v27 = (char *)sub_60B5B(v15 + 1);
        if ( v25[156] == 76 )
        {
          free(ptr);
          ptr = v27;
          v20 = v15 >> 9;
        }
        else
        {
          free(v19);
          v19 = v27;
          v21 = v15 >> 9;
        }
        sub_BBA7((unsigned long long)v25);
        *(long long *)v27 = *(long long *)v25;
        *((long long *)v27 + 63) = *((long long *)v25 + 63);
        v3 = (char *)((unsigned long long)(v27 + 8) & 0xFFFFFFFFFFFFFFF8LL);
        v4 = (((int)v27 - (int)v3 + 512) & 0xFFFFFFF8) >> 3;
        qmemcpy(v3, &v25[-(v27 - v3)], 8LL * v4);
        v5 = (unsigned long long)&v3[8 * v4];
        dest = v27 + 512;
        for ( i = v15 - 512; i; i -= n )
        {
          src = sub_BB4B();
          if ( !src )
          {
            if ( qword_9FDC8 )
              qword_9FDC8(v5);
            v6 = gettext("Unexpected EOF in archive");
            error(0, 0, v6);
            dword_9FDD0 = 2;
            break;
          }
          n = (size_t)sub_BBF5((long long)src);
          if ( n > i )
            n = i;
          memcpy(dest, src, n);
          dest += n;
          v5 = (unsigned long long)src + n - 1;
          sub_BBA7(v5);
        }
        *dest = 0;
        break;
      case 'x':
      case 'X':
        v7 = sub_2798B(v25 + 124, 12);
        sub_1E28E(a2 + 360, v25, v7);
        break;
      case 'g':
        if ( !qword_9F490 )
          qword_9F490 = (void *)sub_60B5B(512);
        memcpy(qword_9F490, v25, 0x200u);
        memset(s, 0, sizeof(s));
        v8 = sub_2798B(v25 + 124, 12);
        sub_1E28E((long long)s, v25, v8);
        sub_1E0E7((long long)s);
        sub_1E785((long long)s);
        if ( a3 == 2 )
        {
          v13 = 2;
          goto LABEL_53;
        }
        break;
    }
  }
  free(qword_9F470);
  if ( ptr )
  {
    v22 = ptr + 512;
    qword_9F470 = ptr;
    qword_9F480 = v20;
    ptr = 0;
    sub_29547(a2, v22);
  }
  else
  {
    v24 = v30;
    if ( v25[345] && !strcmp(v25 + 257, "ustar") )
    {
      memcpy(v30, v25 + 345, 0x9Bu);
      v30[155] = 0;
      v9 = &v30[strlen(v30)];
      v24 = v9 + 1;
      *v9 = 47;
    }
    memcpy(v24, v25, 0x64u);
    v24[100] = 0;
    v22 = v30;
    qword_9F470 = 0;
    qword_9F480 = 0;
    sub_29547(a2, v30);
  }
  sub_29547(a2 + 8, v22);
  *(char *)(a2 + 16) = sub_4E2E5(*(long long *)(a2 + 8));
  free(qword_9F478);
  if ( v19 )
  {
    v23 = v19 + 512;
    qword_9F478 = v19;
    qword_9F488 = v21;
    v19 = 0;
    sub_29547(a2 + 24, v23);
  }
  else
  {
    memcpy(v30, v25 + 157, 0x64u);
    v30[100] = 0;
    qword_9F478 = 0;
    qword_9F488 = 0;
    sub_29547(a2 + 24, v30);
  }
LABEL_53:
  free(ptr);
  free(v19);
  return v13;
}



// Function: decode_header @ 0x26747
unsigned long long decode_header(long long a1, long long a2, int *a3, int a4)
{
  int v4; // eax
  long long v5; // rax
  long long v6; // rax
  long long v7; // rdx
  long long v8; // rdx
  unsigned int v9; // ebx
  unsigned int v10; // eax
  char v14; // [rsp+2Fh] [rbp-21h] BYREF
  int v15; // [rsp+30h] [rbp-20h]
  int v16; // [rsp+34h] [rbp-1Ch]
  unsigned long long v17; // [rsp+38h] [rbp-18h]

  v17 = __readfsqword(0x28u);
  v16 = sub_2790D(a1 + 100, 8, &v14);
  if ( !strcmp((const char *)(a1 + 257), "ustar") )
  {
    if ( !*(char *)(a1 + 475)
      && *(char *)(a1 + 476) > 47
      && *(char *)(a1 + 476) <= 55
      && *(char *)(a1 + 487) == 32
      && *(char *)(a1 + 488) > 47
      && *(char *)(a1 + 488) <= 55
      && *(char *)(a1 + 499) == 32 )
    {
      v15 = 5;
    }
    else if ( *(long long *)(a2 + 368) )
    {
      v15 = 4;
    }
    else
    {
      v15 = 3;
    }
  }
  else if ( !strcmp((const char *)(a1 + 257), "ustar  ") )
  {
    if ( v14 )
      v4 = 2;
    else
      v4 = 6;
    v15 = v4;
  }
  else
  {
    v15 = 1;
  }
  *a3 = v15;
  *(int *)(a2 + 112) = v16;
  *(long long *)(a2 + 248) = sub_279D7(a1 + 136, 12);
  *(long long *)(a2 + 256) = 0;
  if ( *(char *)(a1 + 265) )
    v5 = a1 + 265;
  else
    v5 = 0;
  sub_29591(a2 + 32, v5, 32);
  if ( *(char *)(a1 + 297) )
    v6 = a1 + 297;
  else
    v6 = 0;
  sub_29591(a2 + 40, v6, 32);
  sub_1D4F4((long long *)a2);
  if ( v15 == 2 && byte_9F8BE )
  {
    *(long long *)(a2 + 232) = sub_279D7(a1 + 345, 12);
    *(long long *)(a2 + 264) = sub_279D7(a1 + 357, 12);
    *(long long *)(a2 + 272) = 0;
    *(long long *)(a2 + 240) = *(long long *)(a2 + 272);
  }
  else if ( v15 == 5 )
  {
    *(long long *)(a2 + 232) = sub_279D7(a1 + 476, 12);
    *(long long *)(a2 + 264) = sub_279D7(a1 + 488, 12);
    *(long long *)(a2 + 272) = 0;
    *(long long *)(a2 + 240) = *(long long *)(a2 + 272);
  }
  else
  {
    v7 = qword_9F9E8;
    *(long long *)(a2 + 264) = qword_9F9E0;
    *(long long *)(a2 + 272) = v7;
    v8 = *(long long *)(a2 + 272);
    *(long long *)(a2 + 232) = *(long long *)(a2 + 264);
    *(long long *)(a2 + 240) = v8;
  }
  if ( v15 == 1 )
  {
    *(int *)(a2 + 116) = sub_27A2B(a1 + 108, 8);
    *(int *)(a2 + 120) = sub_27831(a1 + 116, 8);
    *(long long *)(a2 + 128) = 0;
  }
  else
  {
    if ( a4 )
    {
      if ( byte_9F934 || !*(char *)(a1 + 265) || !(unsigned int)sub_2C173(a1 + 265, a2 + 116) )
        *(int *)(a2 + 116) = sub_27A2B(a1 + 108, 8);
      if ( byte_9F934 || !*(char *)(a1 + 297) || !(unsigned int)sub_2C261(a1 + 297, a2 + 120) )
        *(int *)(a2 + 120) = sub_27831(a1 + 116, 8);
    }
    if ( (unsigned int)(*(char *)(a1 + 156) - 51) > 1 )
    {
      *(long long *)(a2 + 128) = 0;
    }
    else
    {
      v9 = sub_278C3(a1 + 337, 8);
      v10 = sub_27879(a1 + 329, 8);
      *(long long *)(a2 + 128) = gnu_dev_makedev(v10, v9);
    }
  }
  sub_1DF5F(a2);
  if ( (unsigned char)sub_301DE(a2) )
  {
    sub_30241(a2);
    *(char *)(a2 + 288) = 1;
  }
  else
  {
    *(char *)(a2 + 288) = 0;
    if ( (dword_9F468 == 6 || dword_9F468 == 2) && *(char *)(qword_9F460 + 156) == 68 || *(long long *)(a2 + 400) )
      *(char *)(a2 + 392) = 1;
  }
  return v17 - __readfsqword(0x28u);
}



// Function: from_header @ 0x26c90
long long from_header(char *a1, size_t a2, long long a3, long long a4, unsigned long long a5, char a6, char a7)
{
  char *v7; // rax
  char *v9; // rax
  char *v10; // rax
  char *v11; // rax
  char *v12; // rax
  char *v13; // rax
  size_t v14; // rax
  void *v15; // rsp
  long long v16; // rbx
  char *v17; // rax
  char *v18; // rax
  char *v19; // rax
  char *v20; // rax
  char *v21; // rax
  unsigned long long v22; // rax
  long long v23; // rax
  unsigned long long v24; // rdi
  long long v25; // rbx
  char *v26; // rax
  char v27[4]; // [rsp+8h] [rbp-4D0h] BYREF
  char v28; // [rsp+Ch] [rbp-4CCh]
  unsigned long long v29; // [rsp+10h] [rbp-4C8h]
  long long v30; // [rsp+18h] [rbp-4C0h]
  long long v31; // [rsp+20h] [rbp-4B8h]
  size_t n; // [rsp+28h] [rbp-4B0h]
  void *src; // [rsp+30h] [rbp-4A8h]
  bool v34; // [rsp+3Ah] [rbp-49Eh]
  bool v35; // [rsp+3Bh] [rbp-49Dh]
  int v36; // [rsp+3Ch] [rbp-49Ch]
  int v37; // [rsp+40h] [rbp-498h]
  int v38; // [rsp+44h] [rbp-494h]
  unsigned long long v39; // [rsp+48h] [rbp-490h]
  char *i; // [rsp+50h] [rbp-488h]
  char *v41; // [rsp+58h] [rbp-480h]
  long long v42; // [rsp+60h] [rbp-478h]
  long long v43; // [rsp+68h] [rbp-470h]
  long long v44; // [rsp+70h] [rbp-468h]
  unsigned long long v45; // [rsp+78h] [rbp-460h]
  long long v46; // [rsp+80h] [rbp-458h]
  void *dest; // [rsp+88h] [rbp-450h]
  char *v48; // [rsp+90h] [rbp-448h]
  char v49[32]; // [rsp+98h] [rbp-440h] BYREF
  char v50[7]; // [rsp+B9h] [rbp-41Fh] BYREF
  char v51; // [rsp+D8h] [rbp-400h] BYREF
  char v52[1015]; // [rsp+D9h] [rbp-3FFh] BYREF

  src = a1;
  n = a2;
  v31 = a3;
  v30 = a4;
  v29 = a5;
  v28 = a6;
  v27[0] = a7;
  *(long long *)&v52[999] = __readfsqword(0x28u);
  v44 = a4;
  v45 = -a4;
  i = a1;
  v41 = &a1[a2];
  v34 = 0;
  for ( i = &a1[*a1 == 0]; ; ++i )
  {
    if ( i == v41 )
    {
      if ( v31 && v27[0] != 1 )
      {
        if ( qword_9FDC8 )
          qword_9FDC8(a1);
        v7 = gettext("Blanks in header where numeric %s value expected");
        error(0, 0, v7, v31);
        dword_9FDD0 = 2;
      }
      return -1;
    }
    if ( ((*__ctype_b_loc())[(unsigned char)*i] & 0x2000) == 0 )
      break;
  }
  v39 = 0;
  if ( (unsigned int)(*i - 48) > 7 )
  {
    if ( !v28 )
    {
      if ( *i == 45 || *i == 43 )
      {
        if ( v27[0] != 1 && byte_9F5A1 != 1 )
        {
          byte_9F5A1 = 1;
          if ( qword_9FDC8 )
            qword_9FDC8(a1);
          v12 = gettext("Archive contains obsolescent base-64 headers");
          error(0, 0, v12);
        }
        v13 = i++;
        v34 = *v13 == 45;
        while ( i != v41 )
        {
          v38 = (char)byte_9F4A0[(unsigned char)*i];
          if ( v38 > 63 )
            break;
          if ( v39 != (v39 & 0x3FFFFFFFFFFFFFFLL) )
          {
            v14 = 16 * ((n + 24) / 0x10);
            while ( v27 != &v27[-(v14 & 0xFFFFFFFFFFFFF000LL)] )
              ;
            v15 = alloca(v14 & 0xFFF);
            if ( (v14 & 0xFFF) != 0 )
              *(long long *)&v27[(v14 & 0xFFF) - 8] = *(long long *)&v27[(v14 & 0xFFF) - 8];
            dest = v27;
            memcpy(v27, src, n);
            *((char *)dest + n) = 0;
            if ( v31 && v27[0] != 1 )
            {
              if ( qword_9FDC8 )
                qword_9FDC8(v27);
              v16 = sub_5E84E(dest);
              v17 = gettext("Archive signed base-64 string %s is out of %s range");
              error(0, 0, v17, v16, v31);
              dword_9FDD0 = 2;
            }
            return -1;
          }
          v39 = (v39 << 6) | v38;
          ++i;
        }
      }
      else if ( *i == (char)0x80 || *i == -1 )
      {
        v37 = *i & 0x40;
        v46 = (long long)-v37 << 50;
        v18 = i++;
        v39 = (*v18 & 0x3F) - v37;
        while ( 1 )
        {
          v19 = i++;
          v39 = (v39 << 8) + (unsigned char)*v19;
          if ( i == v41 )
            break;
          if ( v39 != (v46 | v39 & 0xFFFFFFFFFFFFFFLL) )
          {
            if ( v31 && v27[0] != 1 )
            {
              if ( qword_9FDC8 )
                qword_9FDC8(a1);
              v20 = gettext("Archive base-256 value is out of %s range");
              error(0, 0, v20, v31);
              dword_9FDD0 = 2;
            }
            return -1;
          }
        }
        v34 = v37 != 0;
        if ( v37 )
          v39 = -(long long)v39;
      }
    }
  }
  else
  {
    v48 = i;
    v35 = 0;
    while ( 1 )
    {
      v9 = i++;
      v39 += *v9 - 48;
      if ( i == v41 || (unsigned int)(*i - 48) > 7 )
        break;
      v35 = v35 || v39 != (v39 & 0x1FFFFFFFFFFFFFFFLL);
      v39 *= 8LL;
    }
    if ( (v35 || v29 < v39) && *v48 > 49 && v31 )
    {
      v36 = (*v48 - 48) | 4;
      v35 = 0;
      v39 = 0;
      i = v48;
      while ( 1 )
      {
        v39 += 7 - v36;
        if ( ++i == v41 || (unsigned int)(*i - 48) > 7 )
          break;
        v36 = *i - 48;
        v35 = v35 || v39 != (v39 & 0x1FFFFFFFFFFFFFFFLL);
        v39 *= 8LL;
      }
      v35 = (v35 | (++v39 == 0)) != 0;
      if ( !v35 && v39 <= v45 )
      {
        if ( v27[0] != 1 )
        {
          if ( qword_9FDC8 )
            qword_9FDC8(a1);
          v10 = gettext("Archive octal value %.*s is out of %s range; assuming two's complement");
          a1 = 0;
          error(0, 0, v10, (unsigned int)((int)i - (int)v48), v48, v31);
        }
        v34 = 1;
      }
    }
    if ( v35 )
    {
      if ( v31 && v27[0] != 1 )
      {
        if ( qword_9FDC8 )
          qword_9FDC8(a1);
        v11 = gettext("Archive octal value %.*s is out of %s range");
        error(0, 0, v11, (unsigned int)((int)i - (int)v48), v48, v31);
        dword_9FDD0 = 2;
      }
      return -1;
    }
  }
  if ( i == v41 || !*i || ((*__ctype_b_loc())[(unsigned char)*i] & 0x2000) != 0 )
  {
    if ( v34 )
      v22 = v45;
    else
      v22 = v29;
    if ( v22 < v39 )
    {
      if ( v31 && v27[0] != 1 )
      {
        v42 = sub_54E5A(v45, v50);
        v24 = v39;
        v43 = sub_54E5A(v39, v52);
        if ( v34 )
          *(char *)--v43 = 45;
        if ( v45 )
          *(char *)--v42 = 45;
        if ( qword_9FDC8 )
          qword_9FDC8(v24);
        v25 = sub_54E5A(v29, v49);
        v26 = gettext("Archive value %s is out of %s range %s..%s");
        error(0, 0, v26, v43, v31, v42, v25);
        dword_9FDD0 = 2;
      }
      return -1;
    }
    else
    {
      if ( v34 )
        v23 = -(long long)v39;
      else
        v23 = v39;
      return sub_294AD(v23);
    }
  }
  else
  {
    if ( v31 )
    {
      if ( !qword_9F5A8 )
      {
        qword_9F5A8 = sub_5C92F(0);
        sub_5C9A2(qword_9F5A8, 8);
      }
      while ( src != v41 && !*(v41 - 1) )
        --v41;
      sub_5DD44(&v51, 1000, src, v41 - (char *)src, qword_9F5A8);
      if ( v27[0] != 1 )
      {
        if ( qword_9FDC8 )
          qword_9FDC8(&v51);
        v21 = gettext("Archive contains %.*s where numeric %s value expected");
        error(0, 0, v21, 1000, &v51, v31);
        dword_9FDD0 = 2;
      }
    }
    return -1;
  }
}



// Function: gid_from_header @ 0x27831
long long gid_from_header(char *a1, size_t a2)
{
  return sub_26C90(a1, a2, (long long)"gid_t", 0, 0xFFFFFFFF, 0, 0);
}



// Function: major_from_header @ 0x27879
long long major_from_header(char *a1, size_t a2)
{
  return sub_26C90(a1, a2, (long long)"major_t", 0xFFFFFFFF80000000LL, 0x7FFFFFFFu, 0, 0);
}



// Function: minor_from_header @ 0x278c3
long long minor_from_header(char *a1, size_t a2)
{
  return sub_26C90(a1, a2, (long long)"minor_t", 0xFFFFFFFF80000000LL, 0x7FFFFFFFu, 0, 0);
}



// Function: mode_from_header @ 0x2790d
long long mode_from_header(char *a1, size_t a2, bool *a3)
{
  long long v5; // [rsp+30h] [rbp-8h]

  v5 = sub_26C90(a1, a2, (long long)"mode_t", 0x8000000000000000LL, 0xFFFFFFFFFFFFFFFFLL, 0, 0);
  *a3 = (v5 & 0xFFFFFFFFFFFFF000LL) != 0;
  return v5 & 0xFFF;
}



// Function: off_from_header @ 0x2798b
long long off_from_header(char *a1, size_t a2)
{
  return sub_26C90(a1, a2, (long long)"off_t", 0, 0x7FFFFFFFFFFFFFFFuLL, 0, 0);
}



// Function: time_from_header @ 0x279d7
long long time_from_header(char *a1, size_t a2)
{
  return sub_26C90(a1, a2, (long long)"time_t", 0x8000000000000000LL, 0x7FFFFFFFFFFFFFFFuLL, 0, 0);
}



// Function: uid_from_header @ 0x27a2b
long long uid_from_header(char *a1, size_t a2)
{
  return sub_26C90(a1, a2, (long long)"uid_t", 0, 0xFFFFFFFF, 0, 0);
}



// Function: uintmax_from_header @ 0x27a73
long long uintmax_from_header(char *a1, size_t a2)
{
  return sub_26C90(a1, a2, (long long)"uintmax_t", 0, 0xFFFFFFFFFFFFFFFFLL, 0, 0);
}



// Function: tartime @ 0x27abc
char *tartime(time_t a1, unsigned int a2, char a3)
{
  struct tm *v3; // rax
  size_t v4; // rax
  time_t v6; // rax
  void *v7; // rax
  unsigned int v9; // [rsp+2Ch] [rbp-34h]
  time_t timer; // [rsp+30h] [rbp-30h] BYREF
  unsigned long long v11; // [rsp+38h] [rbp-28h]
  struct tm *tp; // [rsp+40h] [rbp-20h]
  unsigned long long v13; // [rsp+48h] [rbp-18h]

  v13 = __readfsqword(0x28u);
  timer = a1;
  v9 = a2;
  if ( a1 < 0 && a2 )
  {
    ++timer;
    v9 = 1000000000 - a2;
  }
  if ( byte_9F879 )
    v3 = gmtime(&timer);
  else
    v3 = localtime(&timer);
  tp = v3;
  if ( v3 )
  {
    if ( a3 )
    {
      strftime(byte_9F5C0, 0x25u, "%Y-%m-%d %H:%M:%S", tp);
      v4 = strlen(byte_9F5C0);
      sub_29E23(v9, &byte_9F5C0[v4]);
    }
    else
    {
      strftime(byte_9F5C0, 0x25u, "%Y-%m-%d %H:%M", tp);
    }
    return byte_9F5C0;
  }
  else
  {
    if ( a1 >= 0 )
      v6 = timer;
    else
      v6 = -timer;
    v11 = sub_54E5A(v6, &unk_9F5C6);
    if ( a1 < 0 )
      *(char *)--v11 = 45;
    while ( 1 )
    {
      v7 = a3 ? &unk_9F5E1 : &unk_9F5D4;
      if ( (unsigned long long)v7 >= v11 )
        break;
      *(char *)--v11 = 32;
    }
    if ( a3 )
      sub_29E23(v9, &unk_9F5DA);
    return (char *)v11;
  }
}



// Function: simple_print_header @ 0x27c80
unsigned long long simple_print_header(FILE *a1, long long a2, long long a3)
{
  char *IO_read_ptr; // rax
  char *v4; // rax
  long long v5; // rbx
  char *v6; // rax
  FILE *v7; // rbx
  const char *v8; // rax
  char *v9; // rax
  char v10; // al
  unsigned int v11; // eax
  const char *v12; // rax
  unsigned int v13; // eax
  const char *v14; // rax
  const char *v15; // rax
  int v16; // ebx
  const char *v17; // rax
  const char *v18; // rax
  long long v19; // rbx
  char *v20; // rax
  long long v21; // rbx
  char *v22; // rax
  char *v23; // rax
  char *v24; // rax
  char *v25; // rax
  long long v26; // rax
  const char *v27; // rax
  char *v28; // rax
  signed long long v30; // [rsp+8h] [rbp-108h]
  FILE *v31; // [rsp+18h] [rbp-F8h]
  int v32; // [rsp+24h] [rbp-ECh]
  int v33; // [rsp+28h] [rbp-E8h]
  int v34; // [rsp+2Ch] [rbp-E4h]
  char *v35; // [rsp+30h] [rbp-E0h]
  char *IO_write_base; // [rsp+38h] [rbp-D8h]
  char *IO_write_ptr; // [rsp+40h] [rbp-D0h]
  const char *s; // [rsp+48h] [rbp-C8h]
  char v39[2]; // [rsp+52h] [rbp-BEh] BYREF
  char v40; // [rsp+54h] [rbp-BCh] BYREF
  char v41[3]; // [rsp+55h] [rbp-BBh] BYREF
  short v42; // [rsp+5Eh] [rbp-B2h] BYREF
  char v43[32]; // [rsp+60h] [rbp-B0h] BYREF
  char v44[32]; // [rsp+80h] [rbp-90h] BYREF
  char v45[32]; // [rsp+A0h] [rbp-70h] BYREF
  char dest[56]; // [rsp+C0h] [rbp-50h] BYREF
  unsigned long long v47; // [rsp+F8h] [rbp-18h]

  v31 = a1;
  v30 = a3;
  v47 = __readfsqword(0x28u);
  if ( byte_9FC30 )
  {
    if ( a1->_IO_read_ptr )
      IO_read_ptr = a1->_IO_read_ptr;
    else
      IO_read_ptr = *(char **)&a1->_flags;
    v35 = IO_read_ptr;
  }
  else
  {
    if ( *(long long *)&a1->_flags )
      v4 = *(char **)&a1->_flags;
    else
      v4 = a1->_IO_read_ptr;
    v35 = v4;
  }
  if ( byte_9F88C )
  {
    if ( a3 < 0 )
      v30 = sub_BACD();
    v5 = sub_54E5A(v30 - qword_9F480 - qword_9F488, dest);
    v6 = gettext("block %s: ");
    a1 = stream;
    fprintf(stream, v6, v5);
  }
  if ( dword_9F9AC > 1 )
  {
    v40 = 63;
    switch ( *(char *)(a2 + 156) )
    {
      case 0:
      case 0x30:
      case 0x53:
        if ( LOBYTE(v31->_IO_read_end) )
          v10 = 100;
        else
          v10 = 45;
        v40 = v10;
        break;
      case 0x31:
        v40 = 104;
        break;
      case 0x32:
        v40 = 108;
        break;
      case 0x33:
        v40 = 99;
        break;
      case 0x34:
        v40 = 98;
        break;
      case 0x35:
        v40 = 100;
        break;
      case 0x36:
        v40 = 112;
        break;
      case 0x37:
        v40 = 67;
        break;
      case 0x44:
        v40 = 100;
        break;
      case 0x4B:
      case 0x4C:
        v40 = 76;
        if ( qword_9FDC8 )
          qword_9FDC8(a1);
        v9 = gettext("Unexpected long name header");
        error(0, 0, v9);
        dword_9FDD0 = 2;
        break;
      case 0x4D:
        v40 = 77;
        break;
      case 0x56:
        byte_9F5A0 = 1;
        v40 = 86;
        break;
      default:
        break;
    }
    sub_3DD6F((unsigned int)v31->_fileno, v41);
    sub_3D9EC(v31, &v42);
    s = sub_27ABC((time_t)v31[1]._IO_write_base, (unsigned int)v31[1]._IO_write_ptr, byte_9F87A);
    v32 = strlen(s);
    if ( v32 > dword_9C264 )
      dword_9C264 = v32;
    if ( v31->_IO_write_base && *v31->_IO_write_base && dword_9F468 != 1 && byte_9F934 != 1 )
      IO_write_base = v31->_IO_write_base;
    else
      IO_write_base = (char *)sub_54E5A((unsigned int)v31->_flags2, v43);
    if ( v31->_IO_write_ptr && *v31->_IO_write_ptr && dword_9F468 != 1 && byte_9F934 != 1 )
      IO_write_ptr = v31->_IO_write_ptr;
    else
      IO_write_ptr = (char *)sub_54E5A(LODWORD(v31->_old_offset), v44);
    if ( (unsigned int)(*(char *)(a2 + 156) - 51) > 1 )
    {
      v15 = (const char *)sub_54E5A(v31->_lock, v45);
      strcpy(dest, v15);
    }
    else
    {
      v11 = gnu_dev_major(*(long long *)&v31->_cur_column);
      v12 = (const char *)sub_54E5A(v11, v45);
      strcpy(dest, v12);
      strcat(dest, ",");
      v13 = gnu_dev_minor(*(long long *)&v31->_cur_column);
      v14 = (const char *)sub_54E5A(v13, v45);
      strcat(dest, v14);
    }
    v33 = strlen(dest);
    v16 = strlen(IO_write_base);
    v34 = v16 + strlen(IO_write_ptr) + v33 + 2;
    if ( v34 > dword_9C260 )
      dword_9C260 = v34;
    fprintf(
      stream,
      "%s %s/%s %*s %-*s",
      &v40,
      IO_write_base,
      IO_write_ptr,
      dword_9C260 - v34 + v33,
      dest,
      dword_9C264,
      s);
    v17 = (const char *)sub_5E2EB(v35);
    fprintf(stream, " %s", v17);
    if ( byte_9FC30 && LOBYTE(v31->_IO_read_end) )
      fputc_unlocked(47, stream);
    switch ( *(char *)(a2 + 156) )
    {
      case 0:
      case 0x30:
      case 0x33:
      case 0x34:
      case 0x35:
      case 0x36:
      case 0x37:
      case 0x44:
      case 0x53:
        putc_unlocked(10, stream);
        break;
      case 0x31:
        v19 = sub_5E2EB(v31->_IO_read_base);
        v20 = gettext(" link to %s\n");
        fprintf(stream, v20, v19);
        break;
      case 0x32:
        v18 = (const char *)sub_5E2EB(v31->_IO_read_base);
        fprintf(stream, " -> %s\n", v18);
        break;
      case 0x4B:
        v23 = gettext("--Long Link--\n");
        fprintf(stream, v23);
        break;
      case 0x4C:
        v24 = gettext("--Long Name--\n");
        fprintf(stream, v24);
        break;
      case 0x4D:
        v26 = sub_27A73((char *)(a2 + 369), 0xCu);
        v27 = (const char *)sub_54E5A(v26, v45);
        strcpy(dest, v27);
        v28 = gettext("--Continued at byte %s--\n");
        fprintf(stream, v28, dest);
        break;
      case 0x56:
        v25 = gettext("--Volume Header--\n");
        fprintf(stream, v25);
        break;
      default:
        v39[0] = *(char *)(a2 + 156);
        v39[1] = 0;
        v21 = sub_5E84E(v39);
        v22 = gettext(" unknown file type %s\n");
        fprintf(stream, v22, v21);
        break;
    }
  }
  else
  {
    v7 = stream;
    v8 = (const char *)sub_5E2EB(v35);
    fputs_unlocked(v8, v7);
    if ( byte_9FC30 && LOBYTE(v31->_IO_read_end) )
      fputc_unlocked(47, stream);
    fputc_unlocked(10, stream);
  }
  fflush_unlocked(stream);
  sub_3DB33(v31);
  return v47 - __readfsqword(0x28u);
}



// Function: print_volume_label @ 0x28528
unsigned long long print_volume_label()
{
  int v1; // [rsp+Ch] [rbp-3D4h] BYREF
  FILE v2; // [rsp+10h] [rbp-3D0h] BYREF
  char s[136]; // [rsp+1D0h] [rbp-210h] BYREF
  char v4[384]; // [rsp+258h] [rbp-188h] BYREF
  unsigned long long v5; // [rsp+3D8h] [rbp-8h]

  v5 = __readfsqword(0x28u);
  memset(s, 0, 0x200u);
  v4[20] = 86;
  if ( qword_9F490 )
    memcpy(v4, (char *)qword_9F490 + 136, 0xCu);
  sub_39C55(&v2);
  sub_29547(&v2._IO_read_ptr, &unk_8289C);
  sub_26747((long long)s, (long long)&v2, &v1, 0);
  sub_29547(&v2._IO_read_ptr, qword_9EFE0);
  sub_27C80(&v2, (long long)s, 0);
  sub_39D1C(&v2);
  return v5 - __readfsqword(0x28u);
}



// Function: print_header @ 0x2864c
unsigned long long print_header(FILE *a1, long long a2, long long a3)
{
  if ( dword_9F468 == 4 && byte_9F5A0 != 1 && qword_9EFE0 )
  {
    sub_28528();
    byte_9F5A0 = 1;
  }
  return sub_27C80(a1, a2, a3);
}



// Function: print_for_mkdir @ 0x286af
unsigned long long print_for_mkdir(long long a1, long long a2, unsigned int a3)
{
  signed long long v3; // rax
  long long v4; // rbx
  char *v5; // rax
  const char *v6; // rbx
  char *v7; // rax
  char v9; // [rsp+15h] [rbp-3Bh] BYREF
  short v10; // [rsp+16h] [rbp-3Ah] BYREF
  char v11[24]; // [rsp+20h] [rbp-30h] BYREF
  unsigned long long v12; // [rsp+38h] [rbp-18h]

  v12 = __readfsqword(0x28u);
  if ( dword_9F9AC > 1 )
  {
    v9 = 100;
    sub_3DD6F(a3, &v10);
    if ( byte_9F88C )
    {
      v3 = sub_BACD();
      v4 = sub_54E5A(v3, v11);
      v5 = gettext("block %s: ");
      fprintf(stream, v5, v4);
    }
    v6 = (const char *)sub_5E2EB(a1);
    v7 = gettext("Creating directory:");
    fprintf(stream, "%s %*s %s\n", &v9, dword_9C260 + 1 + dword_9C264, v7, v6);
  }
  return v12 - __readfsqword(0x28u);
}



// Function: skip_file @ 0x287c1
long long skip_file(signed long long a1)
{
  long long v1; // rdi
  long long result; // rax
  char *v3; // rax
  long long v4; // [rsp+8h] [rbp-18h]
  __off_t v5; // [rsp+10h] [rbp-10h]
  void *v6; // [rsp+18h] [rbp-8h]

  v4 = a1;
  if ( byte_9FC18 )
  {
    v5 = sub_CE79(a1);
    if ( v5 < 0 )
      byte_9FC18 = 0;
    else
      v4 = a1 - (v5 << 9);
  }
  v1 = v4;
  result = sub_AD86(v4);
  while ( v4 > 0 )
  {
    v6 = sub_BB4B();
    if ( !v6 )
    {
      if ( qword_9FDC8 )
        qword_9FDC8(v1);
      v3 = gettext("Unexpected EOF in archive");
      error(0, 0, v3);
      sub_177F4();
    }
    sub_BBA7((unsigned long long)v6);
    v4 -= 512;
    v1 = v4;
    result = sub_AD86(v4);
  }
  return result;
}



// Function: skip_member @ 0x28891
void **skip_member()
{
  void **result; // rax
  char v1; // [rsp+Fh] [rbp-1h]

  result = (void **)((unsigned char)byte_9FBA9 ^ 1u);
  if ( byte_9FBA9 != 1 )
  {
    v1 = *(char *)(qword_9F460 + 156);
    sub_BBA7(qword_9F460);
    sub_AD2B(&qword_9FA20);
    if ( byte_9FB40 )
    {
      sub_303BD(&qword_9FA20);
    }
    else if ( v1 != 53 )
    {
      sub_287C1(*(signed long long *)&byte_9FA78[48]);
    }
    return sub_AD66();
  }
  return result;
}



// Function: test_archive_label @ 0x28913
long long test_archive_label()
{
  void *ptr; // [rsp+8h] [rbp-8h]

  sub_254C9();
  sub_2D3B1();
  sub_F694(0);
  if ( (unsigned int)sub_25ECF(&qword_9F460, (long long)&qword_9FA20, 0) == 1 )
  {
    sub_26747(qword_9F460, (long long)&qword_9FA20, &dword_9F468, 0);
    if ( *(char *)(qword_9F460 + 156) == 86 )
      sub_29591(&qword_9EFE0, qword_9F460, 100);
    if ( qword_9EFE0 )
    {
      if ( dword_9F9AC )
        sub_28528();
      if ( (unsigned char)sub_2D88A(qword_9EFE0) != 1 && byte_9F8FC )
      {
        ptr = sub_E680((const char *)qword_9EFE0);
        sub_2D88A(ptr);
        free(ptr);
      }
    }
  }
  sub_D046();
  return sub_2DD64();
}



// Function: map_hash @ 0x28a20
unsigned long long map_hash(long long *a1, unsigned long long a2)
{
  return *a1 % a2;
}



// Function: map_compare @ 0x28a4d
bool map_compare(long long *a1, long long *a2)
{
  return *a1 == *a2;
}



// Function: parse_id @ 0x28a83
long long parse_id(uintmax_t *a1, const char *a2, long long a3, uintmax_t a4, long long a5, unsigned int a6)
{
  char *v6; // rax
  char *v8; // rax
  char *endptr; // [rsp+38h] [rbp-18h] BYREF
  uintmax_t v14; // [rsp+40h] [rbp-10h]
  unsigned long long v15; // [rsp+48h] [rbp-8h]

  v15 = __readfsqword(0x28u);
  *__errno_location() = 0;
  v14 = strtoumax(a2, &endptr, 10);
  if ( *endptr || *__errno_location() )
  {
    v6 = gettext("%s:%u: invalid %s: %s");
    error(0, 0, v6, a5, a6, a3, a2);
    return 0xFFFFFFFFLL;
  }
  else if ( v14 <= a4 )
  {
    *a1 = v14;
    return 0;
  }
  else
  {
    v8 = gettext("%s:%u: %s out of range: %s");
    error(0, 0, v8, a5, a6, a3, a2);
    return 0xFFFFFFFFLL;
  }
}



// Function: map_read @ 0x28bbb
unsigned long long map_read(
        long long *a1,
        const char *a2,
        long long (*a3)(const char *),
        long long a4,
        uintmax_t a5,
        double a6)
{
  char *v6; // rdi
  long long v7; // rbx
  char *v8; // rax
  char *v9; // rax
  const char *v10; // rbx
  char *v11; // rax
  const char *v12; // rax
  const char *v13; // rbx
  char *v14; // rax
  long long v15; // rdx
  FILE *v16; // rdi
  char *v17; // rax
  unsigned int v23; // [rsp+3Ch] [rbp-154h]
  unsigned int v24; // [rsp+40h] [rbp-150h]
  int v25; // [rsp+44h] [rbp-14Ch]
  char *lineptr; // [rsp+48h] [rbp-148h] BYREF
  size_t n; // [rsp+50h] [rbp-140h] BYREF
  long long v28; // [rsp+58h] [rbp-138h] BYREF
  long long v29; // [rsp+60h] [rbp-130h] BYREF
  const char *v30; // [rsp+68h] [rbp-128h]
  FILE *stream; // [rsp+70h] [rbp-120h]
  __ssize_t v32; // [rsp+78h] [rbp-118h]
  char *v33; // [rsp+80h] [rbp-110h]
  long long *v34; // [rsp+88h] [rbp-108h]
  long long v35; // [rsp+90h] [rbp-100h] BYREF
  const char **v36; // [rsp+98h] [rbp-F8h]
  const char *v37; // [rsp+D0h] [rbp-C0h]
  unsigned long long v38; // [rsp+178h] [rbp-18h]

  v38 = __readfsqword(0x28u);
  lineptr = 0;
  n = 0;
  v25 = 0;
  stream = fopen(a2, "r");
  if ( !stream )
    sub_3E3B4(a2);
  v37 = "#";
  v23 = 36420;
  v24 = 0;
  while ( 1 )
  {
    v32 = getline(&lineptr, &n, stream);
    if ( v32 <= 0 )
      break;
    v30 = 0;
    ++v24;
    v6 = lineptr;
    if ( (unsigned int)sub_461A6(lineptr, &v35, v23, a6) )
    {
      if ( qword_9FDC8 )
        qword_9FDC8(v6);
      v7 = sub_4647F(&v35);
      v8 = gettext("%s:%u: cannot split line: %s");
      error(0, 0, v8, a2, v24, v7);
      sub_177F4();
    }
    v23 |= 8u;
    if ( v35 )
    {
      if ( v35 == 2 )
      {
        if ( **v36 == 43 )
        {
          if ( (unsigned int)sub_28A83((uintmax_t *)&v28, *v36 + 1, a4, a5, (long long)a2, v24) )
          {
            v25 = 1;
          }
          else
          {
LABEL_17:
            v33 = strchr(v36[1], 58);
            if ( v33 )
            {
              if ( v33 > v36[1] )
                v30 = v36[1];
              v12 = v33++;
              *v12 = 0;
              if ( (unsigned int)sub_28A83((uintmax_t *)&v29, v33, a4, a5, (long long)a2, v24) )
              {
                v25 = 1;
              }
              else
              {
LABEL_27:
                v34 = (long long *)sub_60B5B(24);
                *v34 = v28;
                v34[1] = v29;
                if ( v30 )
                  v15 = sub_60CC6(v30);
                else
                  v15 = 0;
                v34[2] = v15;
                if ( !*a1 && (*a1 = sub_52C1C(0, 0, sub_28A20, sub_28A4D, 0)) == 0 || !sub_5395D(*a1, v34) )
                  sub_1783C();
              }
            }
            else if ( *v36[1] == 43 )
            {
              if ( !(unsigned int)sub_28A83((uintmax_t *)&v29, v36[1], a4, a5, (long long)a2, v24) )
                goto LABEL_27;
              v25 = 1;
            }
            else
            {
              v30 = v36[1];
              v29 = a3(v36[1]);
              if ( v29 != -1 )
                goto LABEL_27;
              v13 = v36[1];
              v14 = gettext("%s:%u: can't obtain %s of %s");
              error(0, 0, v14, a2, v24, a4, v13);
              v25 = 1;
            }
          }
        }
        else
        {
          if ( !a3 )
            goto LABEL_17;
          v28 = a3(*v36);
          if ( v28 != -1 )
            goto LABEL_17;
          v10 = *v36;
          v11 = gettext("%s:%u: can't obtain %s of %s");
          error(0, 0, v11, a2, v24, a4, v10);
          v25 = 1;
        }
      }
      else
      {
        v9 = gettext("%s:%u: malformed line");
        error(0, 0, v9, a2, v24);
        v25 = 1;
      }
    }
  }
  if ( (v23 & 8) != 0 )
    sub_46399(&v35, a6);
  v16 = stream;
  fclose(stream);
  if ( v25 )
  {
    if ( qword_9FDC8 )
      qword_9FDC8(v16);
    v17 = gettext("errors reading map file");
    error(0, 0, v17);
    sub_177F4();
  }
  return v38 - __readfsqword(0x28u);
}



// Function: name_to_uid @ 0x291f9
long long name_to_uid(const char *a1)
{
  struct passwd *v2; // [rsp+18h] [rbp-8h]

  v2 = getpwnam(a1);
  if ( v2 )
    return v2->pw_uid;
  else
    return -1;
}



// Function: owner_map_read @ 0x29234
unsigned long long owner_map_read(const char *a1, double a2)
{
  return sub_28BBB(&qword_9F5E8, a1, sub_291F9, (long long)"UID", 0xFFFFFFFF, a2);
}



// Function: owner_map_translate @ 0x29274
long long owner_map_translate(unsigned int a1, int *a2, long long *a3)
{
  unsigned int v5; // [rsp+24h] [rbp-2Ch]
  long long v6; // [rsp+28h] [rbp-28h]
  long long v7[4]; // [rsp+30h] [rbp-20h] BYREF

  v7[3] = __readfsqword(0x28u);
  v5 = 1;
  if ( qword_9F5E8 && (v7[0] = a1, (v6 = sub_52567(qword_9F5E8, v7)) != 0) )
  {
    *a2 = *(long long *)(v6 + 8);
    *a3 = *(long long *)(v6 + 16);
    return 0;
  }
  else
  {
    if ( dword_9F948 != -1 )
    {
      *a2 = dword_9F948;
      v5 = 0;
    }
    if ( qword_9F940 )
    {
      *a3 = qword_9F940;
      return 0;
    }
    return v5;
  }
}



// Function: name_to_gid @ 0x29353
long long name_to_gid(const char *a1)
{
  struct group *v2; // [rsp+18h] [rbp-8h]

  v2 = getgrnam(a1);
  if ( v2 )
    return v2->gr_gid;
  else
    return -1;
}



// Function: group_map_read @ 0x2938e
unsigned long long group_map_read(const char *a1, double a2)
{
  return sub_28BBB(&qword_9F5F0, a1, sub_29353, (long long)"GID", 0xFFFFFFFF, a2);
}



// Function: group_map_translate @ 0x293ce
long long group_map_translate(unsigned int a1, int *a2, long long *a3)
{
  unsigned int v5; // [rsp+24h] [rbp-2Ch]
  long long v6; // [rsp+28h] [rbp-28h]
  long long v7[4]; // [rsp+30h] [rbp-20h] BYREF

  v7[3] = __readfsqword(0x28u);
  v5 = 1;
  if ( qword_9F5F0 && (v7[0] = a1, (v6 = sub_52567(qword_9F5F0, v7)) != 0) )
  {
    *a2 = *(long long *)(v6 + 8);
    *a3 = *(long long *)(v6 + 16);
    return 0;
  }
  else
  {
    if ( dword_9F8B8 != -1 )
    {
      *a2 = dword_9F8B8;
      v5 = 0;
    }
    if ( qword_9F8B0 )
    {
      *a3 = qword_9F8B0;
      return 0;
    }
    return v5;
  }
}



// Function: quote_n_colon @ 0x29516
long long quote_n_colon(unsigned int a1, long long a2)
{
  unsigned int v2; // eax

  v2 = sub_5C97E(0);
  return sub_5E599(a1, v2, a2);
}



// Function: assign_string @ 0x29547
void **assign_string(void **a1, long long a2)
{
  long long v2; // rdx

  free(*a1);
  if ( a2 )
    v2 = sub_60CC6(a2);
  else
    v2 = 0;
  *a1 = (void *)v2;
  return a1;
}



// Function: assign_string_n @ 0x29591
void assign_string_n(void **a1, const char *a2, size_t a3)
{
  size_t n; // [rsp+20h] [rbp-10h]
  void *dest; // [rsp+28h] [rbp-8h]

  free(*a1);
  if ( a2 )
  {
    n = strnlen(a2, a3);
    dest = (void *)sub_60B5B(n + 1);
    memcpy(dest, a2, n);
    *((char *)dest + n) = 0;
    *a1 = dest;
  }
}



// Function: unquote_string @ 0x2961d
long long unquote_string(char *a1)
{
  char *v1; // rax
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
  char *v12; // rax
  char *v13; // rax
  char *v14; // rax
  char *v15; // rax
  char *v16; // rax
  char *v17; // rdx
  char *v18; // rax
  char *v19; // rdx
  char *v20; // rax
  unsigned int v22; // [rsp+10h] [rbp-18h]
  char v23; // [rsp+14h] [rbp-14h]
  char v24; // [rsp+14h] [rbp-14h]
  char v25; // [rsp+14h] [rbp-14h]
  char *v26; // [rsp+18h] [rbp-10h]
  char *v27; // [rsp+20h] [rbp-8h]

  v22 = 1;
  v26 = a1;
  v27 = a1;
  while ( *v26 )
  {
    if ( *v26 == 92 )
    {
      switch ( *++v26 )
      {
        case '0':
        case '1':
        case '2':
        case '3':
        case '4':
        case '5':
        case '6':
        case '7':
          v10 = v26++;
          v23 = *v10 - 48;
          if ( *v26 > 47 && *v26 <= 55 )
          {
            v12 = v26++;
            v24 = 8 * v23 + *v12 - 48;
            if ( *v26 > 47 && *v26 <= 55 )
            {
              v14 = v26++;
              v25 = 8 * v24 + *v14 - 48;
              v15 = v27++;
              *v15 = v25;
            }
            else
            {
              v13 = v27++;
              *v13 = v24;
            }
          }
          else
          {
            v11 = v27++;
            *v11 = v23;
          }
          break;
        case '?':
          v9 = v27++;
          *v9 = 127;
          ++v26;
          break;
        case '\\':
          v1 = v27++;
          *v1 = 92;
          ++v26;
          break;
        case 'a':
          v2 = v27++;
          *v2 = 7;
          ++v26;
          break;
        case 'b':
          v3 = v27++;
          *v3 = 8;
          ++v26;
          break;
        case 'f':
          v4 = v27++;
          *v4 = 12;
          ++v26;
          break;
        case 'n':
          v5 = v27++;
          *v5 = 10;
          ++v26;
          break;
        case 'r':
          v6 = v27++;
          *v6 = 13;
          ++v26;
          break;
        case 't':
          v7 = v27++;
          *v7 = 9;
          ++v26;
          break;
        case 'v':
          v8 = v27++;
          *v8 = 11;
          ++v26;
          break;
        default:
          v22 = 0;
          v16 = v27++;
          *v16 = 92;
          if ( *v26 )
          {
            v17 = v26++;
            v18 = v27++;
            *v18 = *v17;
          }
          break;
      }
    }
    else if ( v26 == v27 )
    {
      ++v26;
      ++v27;
    }
    else
    {
      v19 = v26++;
      v20 = v27++;
      *v20 = *v19;
    }
  }
  if ( v26 != v27 )
    *v27 = 0;
  return v22;
}



// Function: zap_slashes @ 0x298d7
const char *zap_slashes(const char *a1)
{
  const char *v2; // rax
  const char *i; // [rsp+18h] [rbp-8h]

  if ( !a1 || !*a1 )
    return a1;
  for ( i = &a1[strlen(a1) - 1]; i > a1 && *i == 47; --i )
  {
    v2 = i;
    *v2 = 0;
  }
  return a1;
}



// Function: normalize_filename_x @ 0x29946
char *normalize_filename_x(char *a1)
{
  long long v1; // rax
  char *v2; // rax
  char *v3; // rax
  char *result; // rax
  int v5; // eax
  int v6; // eax
  char v7; // [rsp+Fh] [rbp-19h]
  char *v8; // [rsp+10h] [rbp-18h]
  char *v9; // [rsp+10h] [rbp-18h]
  char *i; // [rsp+18h] [rbp-10h]

  v8 = a1;
  for ( i = a1; ; v8 += *i == 0 )
  {
    *v8 = *i;
    if ( *v8 != 46 || i[1] != 47 )
      break;
    for ( i += 2; *i == 47; ++i )
      ;
  }
  while ( 1 )
  {
    v2 = i++;
    v7 = *v2;
    v3 = v8++;
    *v3 = v7;
    if ( !*v3 )
      break;
    if ( v7 == 47 )
    {
      while ( i[*i == 46] == 47 )
      {
        if ( *i == 46 )
          v1 = 2;
        else
          v1 = 1;
        i += v1;
      }
    }
  }
  result = (char *)(v8 - a1);
  if ( v8 - a1 > 2 )
  {
    v5 = *(v8 - 2) == 46 && *(v8 - 3) == 47;
    v9 = &v8[-v5];
    v6 = v9 - a1 > 2 && *(v9 - 2) == 47;
    result = &v9[-v6 - 1];
    *result = 0;
  }
  return result;
}



// Function: normalize_filename @ 0x29aac
char *normalize_filename(unsigned int a1, const char *a2)
{
  size_t v2; // rax
  char *dest; // [rsp+18h] [rbp-28h]
  char *s; // [rsp+20h] [rbp-20h]
  size_t v6; // [rsp+28h] [rbp-18h]

  dest = 0;
  if ( *a2 != 47 )
  {
    s = (char *)sub_2AFED(a1);
    v6 = strlen(s);
    v2 = strlen(a2);
    dest = (char *)sub_60B5B(v6 + 1 + v2 + 1);
    strcpy(dest, s);
    dest[v6] = 47;
    strcpy(&dest[v6 + 1], a2);
  }
  if ( !dest )
    dest = (char *)sub_60CC6(a2);
  sub_29946(dest);
  return dest;
}



// Function: replace_prefix @ 0x29b93
int replace_prefix(const char **a1, const void *a2, size_t a3, const void *a4, size_t a5)
{
  size_t v5; // rax
  char *s; // [rsp+30h] [rbp-10h]
  size_t v11; // [rsp+38h] [rbp-8h]

  s = (char *)*a1;
  v5 = strlen(*a1);
  v11 = v5;
  if ( v5 > a3 )
  {
    LODWORD(v5) = memcmp(s, a2, a3);
    if ( !(int)v5 )
    {
      LODWORD(v5) = (unsigned char)s[a3];
      if ( (char)v5 == 47 )
      {
        if ( a5 > a3 )
        {
          s = (char *)sub_60B8D(s, v11 - a3 + a5 + 1);
          *a1 = s;
        }
        memmove(&s[a5], &s[a3], v11 - a3 + 1);
        LODWORD(v5) = (unsigned int)memcpy(s, a4, a5);
      }
    }
  }
  return v5;
}



// Function: sysinttostr @ 0x29c96
long long sysinttostr(unsigned long long a1, long long a2, unsigned long long a3, long long a4)
{
  if ( a1 > a3 )
    return sub_54BDE(a1, a4);
  else
    return sub_54E5A(a1, a4);
}



// Function: strtosysint @ 0x29cfc
long long strtosysint(const char *a1, char **a2, intmax_t a3, long long a4)
{
  uintmax_t v7; // [rsp+28h] [rbp-18h]
  intmax_t v8; // [rsp+30h] [rbp-10h]

  *__errno_location() = 0;
  if ( a4 < 0 )
  {
    if ( (unsigned int)(*a1 - 48) <= 9 )
    {
      v7 = strtoumax(a1, a2, 10);
      if ( v7 <= a4 )
        return sub_294AD(v7);
      *__errno_location() = 34;
      return a4;
    }
LABEL_13:
    *__errno_location() = 22;
    return 0;
  }
  if ( (unsigned int)(a1[*a1 == 45] - 48) > 9 )
    goto LABEL_13;
  v8 = strtoimax(a1, a2, 10);
  if ( a3 <= v8 && v8 <= a4 )
    return v8;
  *__errno_location() = 34;
  if ( v8 >= a3 )
    return a4;
  else
    return a3;
}



// Function: code_ns_fraction @ 0x29e23
char *code_ns_fraction(int a1, char *a2)
{
  char *result; // rax
  char *v3; // [rsp+0h] [rbp-20h]
  int v4; // [rsp+Ch] [rbp-14h]
  int v5; // [rsp+1Ch] [rbp-4h]

  v4 = a1;
  if ( a1 )
  {
    v5 = 9;
    v3 = a2 + 1;
    *a2 = 46;
    while ( !(v4 % 10) )
    {
      v4 /= 10;
      --v5;
    }
    v3[v5] = 0;
    while ( 1 )
    {
      result = &v3[--v5];
      *result = v4 % 10 + 48;
      if ( !v5 )
        break;
      v4 /= 10;
    }
  }
  else
  {
    *a2 = 0;
    return a2;
  }
  return result;
}



// Function: code_timespec @ 0x29f2e
long long code_timespec(long long a1, unsigned int a2, long long a3)
{
  long long v3; // rax
  int v6; // [rsp+2Ch] [rbp-24h]
  long long v7; // [rsp+30h] [rbp-20h]
  long long v8; // [rsp+38h] [rbp-18h]

  v7 = a1;
  v6 = a2;
  if ( a2 >= 0x3B9ACA00 )
    v6 = 0;
  if ( a1 < 0 && v6 )
  {
    v7 = a1 + 1;
    v6 = 1000000000 - v6;
  }
  if ( a1 >= 0 )
    v3 = v7;
  else
    v3 = -v7;
  v8 = sub_54E5A(v3, a3 + 1);
  if ( a1 < 0 )
    *(char *)--v8 = 45;
  sub_29E23(v6, (char *)(a3 + 21));
  return v8;
}



// Function: decode_timespec @ 0x29ff7
intmax_t decode_timespec(const char *a1, char **a2, char a3)
{
  bool v5; // [rsp+26h] [rbp-3Ah]
  bool v6; // [rsp+27h] [rbp-39h]
  int v7; // [rsp+28h] [rbp-38h]
  int v8; // [rsp+2Ch] [rbp-34h]
  intmax_t v9; // [rsp+30h] [rbp-30h]
  char *v10; // [rsp+38h] [rbp-28h]
  signed long long v11; // [rsp+40h] [rbp-20h]

  v9 = 0x8000000000000000LL;
  v10 = (char *)a1;
  v6 = *a1 == 45;
  if ( (unsigned int)(a1[v6] - 48) <= 9 )
  {
    *__errno_location() = 0;
    if ( v6 )
    {
      v9 = strtoimax(a1, a2, 10);
    }
    else
    {
      v11 = strtoumax(a1, a2, 10);
      if ( v11 < 0 )
        *__errno_location() = 34;
      else
        v9 = v11;
    }
    v10 = *a2;
    v7 = 0;
    if ( a3 )
    {
      if ( *v10 == 46 )
      {
        v8 = 0;
        v5 = 0;
        while ( (unsigned int)(*++v10 - 48) <= 9 )
        {
          if ( v8 > 8 )
          {
            v5 = v5 || *v10 != 48;
          }
          else
          {
            ++v8;
            v7 = 10 * v7 + *v10 - 48;
          }
        }
        while ( v8 <= 8 )
        {
          ++v8;
          v7 *= 10;
        }
        if ( v6 && v5 + v7 && v9 != 0x8000000000000000LL )
          --v9;
      }
    }
    __errno_location();
  }
  else
  {
    *__errno_location() = 22;
  }
  *a2 = v10;
  return v9;
}



// Function: must_be_dot_or_slash @ 0x2a1fe
bool must_be_dot_or_slash(char *a1)
{
  long long v1; // rdx
  long long v2; // rax
  char *v5; // [rsp+0h] [rbp-8h]

  v5 = a1;
  if ( *a1 == 47 )
  {
    while ( 1 )
    {
      while ( v5[1] == 47 )
        ++v5;
      if ( v5[1] != 46 )
        break;
      v1 = v5[2] == 46 ? 3LL : 2LL;
      if ( v5[v1] != 47 )
        break;
      if ( v5[2] == 46 )
        v2 = 3;
      else
        v2 = 2;
      v5 += v2;
    }
    return v5[1] == 0;
  }
  else
  {
    while ( *v5 == 46 && v5[1] == 47 )
    {
      for ( v5 += 2; *v5 == 47; ++v5 )
        ;
    }
    return !*v5 || *v5 == 46 && !v5[1];
  }
}



// Function: safer_rmdir @ 0x2a306
long long safer_rmdir(char *a1)
{
  if ( sub_2A1FE(a1) )
  {
    *__errno_location() = 0;
    return 0xFFFFFFFFLL;
  }
  else if ( unlinkat(newfd, a1, 512) )
  {
    return 0xFFFFFFFFLL;
  }
  else
  {
    sub_19454(a1);
    return 0;
  }
}



// Function: remove_any_file @ 0x2a36f
long long remove_any_file(char *a1, unsigned int a2)
{
  int v3; // eax
  size_t i; // rax
  int v5; // [rsp+18h] [rbp-28h]
  int v6; // [rsp+1Ch] [rbp-24h]
  char *s; // [rsp+20h] [rbp-20h]
  char *v8; // [rsp+28h] [rbp-18h]
  size_t v9; // [rsp+30h] [rbp-10h]
  void *ptr; // [rsp+38h] [rbp-8h]

  if ( !unlinkat(newfd, a1, 0) )
    return 1;
  if ( *__errno_location() != 1 && *__errno_location() != 21 )
    return 0;
  if ( !(unsigned int)sub_2A306(a1) )
    return 1;
  v3 = *__errno_location();
  if ( v3 != 39 )
  {
    if ( v3 > 39 )
      return 0;
    if ( v3 == 20 )
      return 0;
    if ( v3 > 20 || v3 && v3 != 17 )
      return 0;
  }
  if ( a2 == 2 )
    return 0xFFFFFFFFLL;
  if ( a2 > 2 || !a2 )
    return 0;
  v8 = (char *)sub_2B8E1(a1, 0);
  if ( !v8 )
    return 0;
  s = v8;
  for ( i = strlen(v8); ; i = strlen(s) )
  {
    v9 = i;
    if ( !i )
      break;
    ptr = (void *)sub_2EE85(a1, s);
    v5 = remove_any_file(ptr, 1);
    v6 = *__errno_location();
    free(ptr);
    if ( !v5 )
    {
      free(v8);
      *__errno_location() = v6;
      return 0;
    }
    s += v9 + 1;
  }
  free(v8);
  return (unsigned int)sub_2A306(a1) == 0;
}



// Function: maybe_backup_file @ 0x2a571
long long maybe_backup_file(const char *a1, char a2)
{
  long long v3; // rdi
  long long v4; // r12
  long long v5; // rbx
  char *v6; // rax
  long long v7; // r12
  long long v8; // rbx
  char *v9; // rax
  int errnum; // [rsp+1Ch] [rbp-B4h]
  char v11[24]; // [rsp+20h] [rbp-B0h] BYREF
  int v12; // [rsp+38h] [rbp-98h]
  unsigned long long v13; // [rsp+B8h] [rbp-18h]

  v13 = __readfsqword(0x28u);
  sub_29547((void **)&old, (long long)a1);
  sub_29547((void **)&new, 0);
  if ( a2 )
  {
    if ( byte_9FDF8 != 1 )
    {
      qword_9FDF0 = (long long)strchr(a1, 58);
      if ( qword_9FDF0 )
      {
        if ( (unsigned long long)a1 < qword_9FDF0 && !memchr(a1, 47, qword_9FDF0 - (long long)a1) )
          return 1;
      }
    }
  }
  if ( (unsigned int)sub_2A9B8(a1, v11) )
  {
    if ( *__errno_location() == 2 )
    {
      return 1;
    }
    else
    {
      sub_3EA03(a1);
      return 0;
    }
  }
  else if ( (v12 & 0xF000) == 0x4000 )
  {
    return 1;
  }
  else if ( a2 && ((v12 & 0xF000) == 0x6000 || (v12 & 0xF000) == 0x2000) )
  {
    return 1;
  }
  else
  {
    new = (char *)sub_4DE0E((unsigned int)newfd, a1, (unsigned int)dword_9F888);
    if ( !new )
      sub_1783C();
    v3 = (unsigned int)newfd;
    if ( renameat(newfd, old, newfd, new) )
    {
      errnum = *__errno_location();
      if ( qword_9FDC8 )
        qword_9FDC8(v3);
      v7 = sub_5E821(1, new);
      v8 = sub_5E548(old);
      v9 = gettext("%s: Cannot rename to %s");
      error(0, errnum, v9, v8, v7);
      dword_9FDD0 = 2;
      sub_29547((void **)&new, 0);
      return 0;
    }
    else
    {
      if ( dword_9F9AC )
      {
        v4 = sub_5E821(1, new);
        v5 = sub_5E821(0, old);
        v6 = gettext("Renaming %s to %s\n");
        fprintf(stream, v6, v5, v4);
      }
      return 1;
    }
  }
}



// Function: undo_last_backup @ 0x2a874
void **undo_last_backup()
{
  void **result; // rax
  long long v1; // rdi
  long long v2; // r12
  long long v3; // rbx
  char *v4; // rax
  long long v5; // r12
  long long v6; // rbx
  char *v7; // rax
  int errnum; // [rsp+Ch] [rbp-14h]

  result = (void **)new;
  if ( new )
  {
    v1 = (unsigned int)newfd;
    if ( renameat(newfd, new, newfd, old) )
    {
      errnum = *__errno_location();
      if ( qword_9FDC8 )
        qword_9FDC8(v1);
      v2 = sub_5E821(1, old);
      v3 = sub_5E548(new);
      v4 = gettext("%s: Cannot rename to %s");
      error(0, errnum, v4, v3, v2);
      dword_9FDD0 = 2;
    }
    if ( dword_9F9AC )
    {
      v5 = sub_5E821(1, old);
      v6 = sub_5E821(0, new);
      v7 = gettext("Renaming %s back to %s\n");
      fprintf(stream, v7, v6, v5);
    }
    return sub_29547((void **)&new, 0);
  }
  return result;
}



// Function: deref_stat @ 0x2a9b8
int deref_stat(const char *a1, struct stat *a2)
{
  return fstatat(newfd, a1, a2, flag);
}



// Function: blocking_read @ 0x2a9e9
long long blocking_read(unsigned int a1, long long a2, long long a3)
{
  int v3; // edx
  int v4; // ecx
  int v5; // r8d
  int v6; // r9d
  int v7; // ecx
  int v8; // r8d
  int v9; // r9d
  int v10; // eax
  int v13; // [rsp+24h] [rbp-Ch]
  long long v14; // [rsp+28h] [rbp-8h]

  v14 = sub_5EC70(a1, a2, a3);
  if ( v14 == -1 && *__errno_location() == 11 )
  {
    v13 = sub_617EB(a1, 3, v3, v4, v5, v6);
    if ( v13 >= 0 && (v13 & 0x800) != 0 )
    {
      v10 = v13;
      BYTE1(v10) = BYTE1(v13) & 0xF7;
      if ( (unsigned int)sub_617EB(a1, 4, v10, v7, v8, v9) != -1 )
        return sub_5EC70(a1, a2, a3);
    }
  }
  return v14;
}



// Function: blocking_write @ 0x2aa95
unsigned long long blocking_write(unsigned int a1, long long a2, unsigned long long a3)
{
  int v3; // edx
  int v4; // ecx
  int v5; // r8d
  int v6; // r9d
  int v7; // ecx
  int v8; // r8d
  int v9; // r9d
  int v10; // eax
  int v13; // [rsp+2Ch] [rbp-14h]
  unsigned long long v14; // [rsp+30h] [rbp-10h]

  v14 = sub_52175(a1, a2, a3);
  if ( v14 < a3 && *__errno_location() == 11 )
  {
    v13 = sub_617EB(a1, 3, v3, v4, v5, v6);
    if ( v13 >= 0 && (v13 & 0x800) != 0 )
    {
      v10 = v13;
      BYTE1(v10) = BYTE1(v13) & 0xF7;
      if ( (unsigned int)sub_617EB(a1, 4, v10, v7, v8, v9) != -1 )
        v14 += sub_52175(a1, v14 + a2, a3 - v14);
    }
  }
  return v14;
}



// Function: set_file_atime @ 0x2ab5e
long long set_file_atime(unsigned int a1, unsigned int a2, long long a3, long long a4, long long a5)
{
  long long v6[6]; // [rsp+20h] [rbp-30h] BYREF

  v6[5] = __readfsqword(0x28u);
  v6[0] = a4;
  v6[1] = a5;
  v6[3] = 1073741822;
  return sub_4F4EC(a1, a2, a3, v6, (unsigned int)flag);
}



// Function: chdir_count @ 0x2abe0
long long chdir_count()
{
  if ( qword_9F620 )
    return (unsigned int)(qword_9F620 - 1);
  else
    return 0;
}



// Function: chdir_arg @ 0x2ac09
long long chdir_arg(char *a1)
{
  char *v2; // [rsp+8h] [rbp-8h]

  v2 = a1;
  if ( qword_9F620 == qword_9F628 )
  {
    if ( !qword_9F628 )
      qword_9F628 = 2;
    qword_9F618 = sub_60A5F(qword_9F618, &qword_9F628, 24);
    if ( !qword_9F620 )
    {
      *(long long *)qword_9F618 = ".";
      *(long long *)(qword_9F618 + 24 * qword_9F620 + 8) = 0;
      *(int *)(qword_9F618 + 24 * qword_9F620++ + 16) = -100;
    }
  }
  if ( *a1 )
  {
    while ( *v2 == 46 && v2[1] == 47 )
    {
      for ( v2 += 2; *v2 == 47; ++v2 )
        ;
    }
    if ( !v2[*v2 == 46] )
      return (unsigned int)(qword_9F620 - 1);
  }
  *(long long *)(qword_9F618 + 24 * qword_9F620) = v2;
  *(long long *)(qword_9F618 + 24 * qword_9F620 + 8) = 0;
  *(int *)(qword_9F618 + 24 * qword_9F620 + 16) = 0;
  return qword_9F620++;
}



// Function: chdir_do @ 0x2adf1
long long chdir_do(int a1)
{
  long long result; // rax
  long long v2; // rax
  int v3; // [rsp+1Ch] [rbp-24h]
  int v4; // [rsp+20h] [rbp-20h]
  int v5; // [rsp+24h] [rbp-1Ch]
  long long i; // [rsp+28h] [rbp-18h]
  long long *v7; // [rsp+30h] [rbp-10h]
  long long v8; // [rsp+38h] [rbp-8h]

  result = (unsigned int)dword_9F600;
  if ( a1 != dword_9F600 )
  {
    v7 = (long long *)(qword_9F618 + 24LL * a1);
    v3 = *((int *)v7 + 4);
    if ( !v3 )
    {
      if ( *(char *)*v7 != 47 )
        chdir_do((unsigned int)(a1 - 1));
      v3 = openat(newfd, (const char *)*v7, dword_9FC0C & 0xFFFDFFFF);
      if ( v3 < 0 )
        sub_3E3B4(*v7);
      *((int *)v7 + 4) = v3;
      if ( (unsigned long long)qword_9F680 > 0xF )
      {
        v8 = qword_9F618 + 24LL * dword_9F67C;
        if ( close(*(int *)(v8 + 16)) )
          sub_2B26A(*(long long *)v8);
        *(int *)(v8 + 16) = 0;
        dword_9F67C = a1;
      }
      else
      {
        v2 = qword_9F680++;
        dword_9F640[v2] = a1;
      }
    }
    if ( v3 > 0 )
    {
      v4 = dword_9F640[0];
      for ( i = 1; v4 != a1; ++i )
      {
        v5 = dword_9F640[i];
        dword_9F640[i] = v4;
        if ( v5 == a1 )
          break;
        v4 = v5;
      }
      dword_9F640[0] = a1;
    }
    dword_9F600 = a1;
    newfd = v3;
    return (unsigned int)v3;
  }
  return result;
}



// Function: tar_dirname @ 0x2afc0
long long tar_dirname()
{
  return *(long long *)(qword_9F618 + 24LL * dword_9F600);
}



// Function: tar_getcdpath @ 0x2afed
long long tar_getcdpath(int a1)
{
  long long v2; // rbx
  long long *v3; // rbx
  long long v4; // rbx
  int i; // [rsp+10h] [rbp-20h]
  int v6; // [rsp+14h] [rbp-1Ch]
  long long v7; // [rsp+18h] [rbp-18h]

  if ( qword_9F618 )
  {
    if ( !*(long long *)(qword_9F618 + 24LL * a1 + 8) )
    {
      v6 = dword_9F600;
      for ( i = a1; i >= 0 && !*(long long *)(qword_9F618 + 24LL * i + 8); --i )
        ;
      while ( ++i <= a1 )
      {
        sub_2ADF1(i);
        if ( i )
        {
          if ( **(char **)(qword_9F618 + 24LL * i) == 47 )
          {
            v3 = (long long *)(qword_9F618 + 24LL * i);
            v3[1] = sub_60CC6(*v3);
          }
          else
          {
            v7 = sub_2B66C(*(long long *)(qword_9F618 + 24LL * i - 24 + 8));
            sub_2B7EC(v7, *(long long *)(qword_9F618 + 24LL * i));
            v4 = qword_9F618 + 24LL * i;
            *(long long *)(v4 + 8) = sub_2B882(v7);
          }
        }
        else
        {
          v2 = qword_9F618;
          *(long long *)(v2 + 8) = sub_60CF7();
          if ( !*(long long *)(v2 + 8) )
            sub_3DFC0("getcwd", ".");
        }
      }
      sub_2ADF1(v6);
    }
    return *(long long *)(qword_9F618 + 24LL * a1 + 8);
  }
  else
  {
    if ( !qword_9F688 )
    {
      qword_9F688 = sub_60CF7();
      if ( !qword_9F688 )
        sub_3DFC0("getcwd", ".");
    }
    return qword_9F688;
  }
}



// Function: close_diag @ 0x2b26a
long long close_diag(long long a1)
{
  long long result; // rax

  if ( !byte_9F8BC )
    return sub_3E1FF(a1);
  result = dword_9EC84 & 0x800000;
  if ( (int)result )
    return sub_3E228(a1);
  return result;
}



// Function: open_diag @ 0x2b2b1
long long open_diag(long long a1)
{
  long long result; // rax

  if ( !byte_9F8BC )
    return sub_3E38B(a1);
  result = dword_9EC84 & 0x800000;
  if ( (int)result )
    return sub_3E3DA(a1);
  return result;
}



// Function: read_diag_details @ 0x2b2f8
long long read_diag_details(long long a1, long long a2, long long a3)
{
  long long result; // rax

  if ( !byte_9F8BC )
    return sub_3E42C(a1, a2, a3);
  result = dword_9EC84 & 0x800000;
  if ( (int)result )
    return sub_3E506(a1, a2, a3);
  return result;
}



// Function: readlink_diag @ 0x2b35d
long long readlink_diag(long long a1)
{
  long long result; // rax

  if ( !byte_9F8BC )
    return sub_3E6B3(a1);
  result = dword_9EC84 & 0x800000;
  if ( (int)result )
    return sub_3E6DC(a1);
  return result;
}



// Function: savedir_diag @ 0x2b3a4
long long savedir_diag(long long a1)
{
  long long result; // rax

  if ( !byte_9F8BC )
    return sub_3E72E(a1);
  result = dword_9EC84 & 0x800000;
  if ( (int)result )
    return sub_3E757(a1);
  return result;
}



// Function: seek_diag_details @ 0x2b3eb
long long seek_diag_details(long long a1, long long a2)
{
  long long result; // rax

  if ( !byte_9F8BC )
    return sub_3E7A9(a1, a2);
  result = dword_9EC84 & 0x800000;
  if ( (int)result )
    return sub_3E890(a1, a2);
  return result;
}



// Function: stat_diag @ 0x2b444
long long stat_diag(long long a1)
{
  long long result; // rax

  if ( !byte_9F8BC )
    return sub_3EA03(a1);
  result = dword_9EC84 & 0x800000;
  if ( (int)result )
    return sub_3EA2C(a1);
  return result;
}



// Function: file_removed_diag @ 0x2b48b
long long file_removed_diag(long long a1, char a2, long long (*a3)(long long))
{
  long long v3; // rbx
  char *v4; // rax

  if ( a2 == 1 || *__errno_location() != 2 )
    return a3(a1);
  if ( (dword_9EC84 & 0x40) != 0 )
  {
    if ( qword_9FDC8 )
      qword_9FDC8(a1);
    v3 = sub_5E548(a1);
    v4 = gettext("%s: File removed before we read it");
    error(0, 0, v4, v3);
  }
  return sub_39E98(1);
}



// Function: xfork @ 0x2b537
long long xfork()
{
  char *v0; // rax
  unsigned int v2; // [rsp+Ch] [rbp-4h]

  v2 = fork();
  if ( v2 == -1 )
  {
    v0 = gettext("child process");
    sub_3DFC0("fork", v0);
  }
  return v2;
}



// Function: xpipe @ 0x2b577
int xpipe(int *a1)
{
  int result; // eax
  char *v2; // rax

  result = pipe(a1);
  if ( result < 0 )
  {
    v2 = gettext("interprocess channel");
    sub_3DFC0("pipe", v2);
  }
  return result;
}



// Function: ptr_align @ 0x2b5bb
unsigned long long ptr_align(long long a1, unsigned long long a2)
{
  return a2 - 1 + a1 - (a2 - 1 + a1) % a2;
}



// Function: page_aligned_alloc @ 0x2b601
unsigned long long page_aligned_alloc(long long *a1, unsigned long long a2)
{
  unsigned long long v3; // [rsp+10h] [rbp-10h]

  v3 = getpagesize();
  if ( a2 + v3 < a2 )
    sub_1783C();
  *a1 = sub_60B5B(a2 + v3);
  return sub_2B5BB(*a1, v3);
}



// Function: namebuf_create @ 0x2b66c
long long namebuf_create(const char *a1)
{
  char *v1; // rsi
  long long v2; // rax
  long long v4; // [rsp+18h] [rbp-8h]

  v4 = sub_60B5B(24);
  *(long long *)(v4 + 8) = strlen(a1) + 2;
  *(long long *)v4 = sub_60B5B(*(long long *)(v4 + 8));
  strcpy(*(char **)v4, a1);
  *(long long *)(v4 + 16) = strlen(*(const char **)v4);
  if ( *(char *)(*(long long *)v4 + *(long long *)(v4 + 16) - 1LL) != 47 )
  {
    v1 = *(char **)v4;
    v2 = *(long long *)(v4 + 16);
    *(long long *)(v4 + 16) = v2 + 1;
    v1[v2] = 47;
  }
  return v4;
}



// Function: namebuf_free @ 0x2b72d
void namebuf_free(void **a1)
{
  free(*a1);
  free(a1);
}



// Function: namebuf_name @ 0x2b75b
long long namebuf_name(long long *a1, const char *a2)
{
  size_t v3; // [rsp+18h] [rbp-8h]

  v3 = strlen(a2);
  while ( a1[2] + v3 + 1 >= a1[1] )
    *a1 = sub_60BD8(*a1, a1 + 1);
  strcpy((char *)(a1[2] + *a1), a2);
  return *a1;
}



// Function: namebuf_add_dir @ 0x2b7ec
long long *namebuf_add_dir(long long *a1, const char *a2)
{
  long long v2; // rbx
  size_t v3; // rdx

  if ( *(char *)(*a1 + a1[2] - 1LL) != 47 )
  {
    sub_2B75B(a1, "/");
    ++a1[2];
  }
  sub_2B75B(a1, a2);
  v2 = a1[2];
  v3 = v2 + strlen(a2);
  a1[2] = v3;
  return a1;
}



// Function: namebuf_finish @ 0x2b882
long long namebuf_finish(long long *a1)
{
  long long v2; // [rsp+18h] [rbp-8h]

  v2 = *a1;
  if ( *(char *)(*a1 + a1[2] - 1) == 47 )
    *(char *)(*a1 + a1[2]) = 0;
  free(a1);
  return v2;
}



// Function: tar_savedir @ 0x2b8e1
long long tar_savedir(const char *a1, int a2)
{
  bool v3; // al
  int fd; // [rsp+1Ch] [rbp-14h]
  long long v5; // [rsp+20h] [rbp-10h]
  DIR *dirp; // [rsp+28h] [rbp-8h]

  v5 = 0;
  dirp = 0;
  fd = openat(newfd, a1, oflag | 0x10000);
  if ( fd >= 0 )
  {
    dirp = fdopendir(fd);
    if ( !dirp || (v5 = sub_5EEE2(dirp, (unsigned int)dword_9FC2C)) == 0 )
      sub_3E72E(a1);
  }
  else
  {
    if ( !a2 && *__errno_location() == 2 )
      return 0;
    sub_3E38B(a1);
  }
  if ( dirp )
    v3 = closedir(dirp) != 0;
  else
    v3 = fd >= 0 && close(fd);
  if ( v3 )
    sub_3E72E(a1);
  return v5;
}



// Function: file_selection_option @ 0x2b9f2
int *file_selection_option(int a1)
{
  int *i; // [rsp+Ch] [rbp-8h]

  for ( i = &unk_9C280; *(long long *)i || i[2] || *((long long *)i + 4); i += 12 )
  {
    if ( a1 == i[2] )
      return i;
  }
  return 0;
}



// Function: file_selection_option_name @ 0x2ba4c
long long file_selection_option_name(int a1)
{
  int *v2; // [rsp+10h] [rbp-8h]

  v2 = sub_2B9F2(a1);
  if ( v2 )
    return *(long long *)v2;
  else
    return 0;
}



// Function: is_file_selection_option @ 0x2ba80
bool is_file_selection_option(int a1)
{
  return sub_2B9F2(a1) != 0;
}



// Function: names_parse_opt @ 0x2baa1
long long names_parse_opt(int a1, long long a2, long long a3)
{
  long long v3; // r12
  long long v4; // rbx
  char *v5; // rax
  long long v7; // [rsp+28h] [rbp-18h]

  if ( a1 != 16777221 )
  {
    if ( a1 <= 16777221 )
    {
      if ( a1 == 256 )
      {
        sub_2C829(a2);
        return 0;
      }
      if ( a1 <= 256 )
      {
        if ( a1 == 67 )
        {
          sub_2C8C4(a2);
        }
        else
        {
          if ( a1 != 84 )
            goto LABEL_14;
          sub_2C8F7(a2);
        }
        return 0;
      }
    }
LABEL_14:
    if ( !sub_2BA80(a1) )
      return 7;
    sub_2C884((unsigned int)a1, a2);
    return 0;
  }
  v7 = *(long long *)(a3 + 40);
  if ( **(int **)v7 == 2 )
  {
    v3 = *(long long *)(*(long long *)v7 + 16LL);
    v4 = *(long long *)(*(long long *)v7 + 8LL);
    v5 = gettext("%s:%lu: unrecognized option");
    error(0, 0, v5, v4, v3);
    sub_39E98(2);
  }
  return 7;
}



// Function: add_exclude_array @ 0x2bbce
long long add_exclude_array(long long a1, unsigned int a2)
{
  long long result; // rax
  int i; // [rsp+1Ch] [rbp-4h]

  for ( i = 0; ; ++i )
  {
    result = *(long long *)(8LL * i + a1);
    if ( !result )
      break;
    sub_4EE5D(qword_9F8A8, *(long long *)(8LL * i + a1), a2);
  }
  return result;
}



// Function: handle_file_selection_option @ 0x2bc3a
long long handle_file_selection_option(long long a1, const char *a2)
{
  long long result; // rax
  int v3; // edx
  int v4; // edx
  const char *v5; // rax
  int errnum; // [rsp+1Ch] [rbp-4h]

  result = (unsigned int)(a1 - 88);
  switch ( (int)a1 )
  {
    case 88:
      if ( dword_9F6BC == 1 )
        v4 = 0;
      else
        v4 = 0x10000000;
      result = sub_4F407(sub_4EE5D, qword_9F8A8, a2, dword_9F930 | dword_9F6C0 | (unsigned int)v4, 10);
      if ( (int)result )
      {
        errnum = *__errno_location();
        if ( qword_9FDC8 )
          qword_9FDC8(sub_4EE5D);
        v5 = (const char *)sub_5E548(a2);
        error(0, errnum, "%s", v5);
        sub_177F4();
      }
      return result;
    case 257:
      result = sub_2BBCE((long long)off_9A7A0, 0x10000000u);
      break;
    case 258:
      result = sub_11EA1("CACHEDIR.TAG", 1, (long long)sub_12087);
      break;
    case 259:
      result = sub_11EA1("CACHEDIR.TAG", 2, (long long)sub_12087);
      break;
    case 260:
      result = sub_11EA1("CACHEDIR.TAG", 3, (long long)sub_12087);
      break;
    case 261:
      if ( dword_9F6BC == 1 )
        v3 = 0;
      else
        v3 = 0x10000000;
      result = sub_4EE5D(qword_9F8A8, a2, dword_9F930 | dword_9F6C0 | (unsigned int)v3);
      break;
    case 262:
      result = sub_17879(a2, 2);
      break;
    case 263:
      result = sub_17879(a2, 1);
      break;
    case 264:
      result = sub_11EA1(a2, 1, 0);
      break;
    case 265:
      result = sub_11EA1(a2, 2, 0);
      break;
    case 266:
      result = sub_11EA1(a2, 3, 0);
      break;
    case 267:
      result = sub_2BBCE((long long)off_9A6E0, 0);
      break;
    case 268:
      result = (long long)sub_18293();
      break;
    case 269:
      result = dword_9F6C0 | 0x10u;
      dword_9F6C0 |= 0x10u;
      break;
    case 270:
      result = dword_9F6C0 & 0xFFFFFFEF;
      dword_9F6C0 &= ~0x10u;
      break;
    case 271:
      result = dword_9F6C0 | 0x40000000u;
      dword_9F6C0 |= 0x40000000u;
      break;
    case 272:
      dword_9C274 = 0;
      result = dword_9F6C0 & 0xBFFFFFFF;
      dword_9F6C0 &= ~0x40000000u;
      break;
    case 273:
      dword_9F930 = 8;
      break;
    case 274:
      dword_9F930 = 0;
      break;
    case 275:
      byte_9FC28 = 1;
      break;
    case 276:
      byte_9FC28 = 0;
      break;
    case 277:
      byte_9F6B8 = 0;
      break;
    case 278:
      result = dword_9F6C0 | 1u;
      dword_9F6C0 |= 1u;
      break;
    case 279:
      dword_9F6BC = 1;
      break;
    case 280:
      byte_9C270 = 0;
      byte_9F6B8 = 1;
      break;
    case 281:
      byte_9C270 = 10;
      byte_9F6B8 = 0;
      break;
    case 282:
      byte_9F6B8 = 1;
      break;
    case 283:
      result = dword_9F6C0 & 0xFFFFFFFE;
      dword_9F6C0 &= ~1u;
      break;
    case 284:
      dword_9F6BC = 2;
      break;
    default:
      if ( qword_9FDC8 )
        qword_9FDC8(a1);
      error(0, 0, "unhandled positional option %d", a1);
      sub_177F4();
  }
  return result;
}



// Function: uid_to_uname @ 0x2bfd1
long long *uid_to_uname(__uid_t a1, long long *a2)
{
  long long *v3; // [rsp+18h] [rbp-8h]

  if ( a1 && a1 == dword_9F6F0 )
  {
    *a2 = sub_60CC6(&unk_83794);
    return a2;
  }
  if ( !qword_9F6C8 || a1 != dword_9F6D8 )
  {
    v3 = (long long *)getpwuid(a1);
    if ( !v3 )
    {
      dword_9F6F0 = a1;
      *a2 = sub_60CC6(&unk_83794);
      return a2;
    }
    dword_9F6D8 = a1;
    sub_29547((void **)&qword_9F6C8, *v3);
  }
  *a2 = sub_60CC6(qword_9F6C8);
  return a2;
}



// Function: gid_to_gname @ 0x2c0a2
long long *gid_to_gname(__gid_t a1, long long *a2)
{
  long long *v3; // [rsp+18h] [rbp-8h]

  if ( a1 && a1 == dword_9F6F4 )
  {
    *a2 = sub_60CC6(&unk_83794);
    return a2;
  }
  if ( !qword_9F6D0 || a1 != dword_9F6DC )
  {
    v3 = (long long *)getgrgid(a1);
    if ( !v3 )
    {
      dword_9F6F4 = a1;
      *a2 = sub_60CC6(&unk_83794);
      return a2;
    }
    dword_9F6DC = a1;
    sub_29547((void **)&qword_9F6D0, *v3);
  }
  *a2 = sub_60CC6(qword_9F6D0);
  return a2;
}



// Function: uname_to_uid @ 0x2c173
long long uname_to_uid(const char *a1, int *a2)
{
  struct passwd *v3; // [rsp+18h] [rbp-8h]

  if ( qword_9F6E0 && !strcmp(a1, qword_9F6E0) )
    return 0;
  if ( !qword_9F6C8 || *a1 != *qword_9F6C8 || strcmp(a1, qword_9F6C8) )
  {
    v3 = getpwnam(a1);
    if ( !v3 )
    {
      sub_29547((void **)&qword_9F6E0, (long long)a1);
      return 0;
    }
    dword_9F6D8 = v3->pw_uid;
    sub_29547((void **)&qword_9F6C8, (long long)v3->pw_name);
  }
  *a2 = dword_9F6D8;
  return 1;
}



// Function: gname_to_gid @ 0x2c261
long long gname_to_gid(const char *a1, int *a2)
{
  struct group *v3; // [rsp+18h] [rbp-8h]

  if ( qword_9F6E8 && !strcmp(a1, qword_9F6E8) )
    return 0;
  if ( !qword_9F6D0 || *a1 != *qword_9F6D0 || strcmp(a1, qword_9F6D0) )
  {
    v3 = getgrnam(a1);
    if ( !v3 )
    {
      sub_29547((void **)&qword_9F6E8, (long long)a1);
      return 0;
    }
    dword_9F6DC = v3->gr_gid;
    sub_29547((void **)&qword_9F6D0, (long long)a1);
  }
  *a2 = dword_9F6DC;
  return 1;
}



// Function: make_name @ 0x2c34c
long long make_name(void *a1)
{
  void *v2; // [rsp+8h] [rbp-18h]
  long long v3; // [rsp+18h] [rbp-8h]

  v2 = a1;
  v3 = sub_60C06(96);
  if ( !a1 )
    v2 = &unk_83794;
  *(long long *)(v3 + 16) = sub_60CC6(v2);
  *(long long *)(v3 + 24) = strlen(*(const char **)(v3 + 16));
  return v3;
}



// Function: free_name @ 0x2c3b1
void free_name(void **a1)
{
  if ( a1 )
  {
    free(a1[2]);
    free(a1[11]);
    free(a1);
  }
}



// Function: name_elt_alloc @ 0x2c3f7
long long *name_elt_alloc()
{
  long long *v1; // [rsp+8h] [rbp-8h]

  v1 = (long long *)sub_60B5B(56);
  if ( !qword_9F708 )
  {
    qword_9F708 = (long long)v1;
    *v1 = 0;
    *(long long *)(qword_9F708 + 8) = *v1;
    *(int *)(qword_9F708 + 16) = 3;
    v1 = (long long *)sub_60B5B(56);
  }
  v1[1] = *(long long *)(qword_9F708 + 8);
  if ( *(long long *)(qword_9F708 + 8) )
    **(long long **)(qword_9F708 + 8) = v1;
  *v1 = qword_9F708;
  *(long long *)(qword_9F708 + 8) = v1;
  return v1;
}



// Function: name_list_adjust @ 0x2c4b8
long long name_list_adjust()
{
  long long result; // rax

  result = qword_9F708;
  if ( qword_9F708 )
  {
    while ( 1 )
    {
      result = *(long long *)(qword_9F708 + 8);
      if ( !result )
        break;
      qword_9F708 = *(long long *)(qword_9F708 + 8);
    }
  }
  return result;
}



// Function: unconsumed_option_push @ 0x2c4f3
long long *unconsumed_option_push(long long *a1)
{
  *a1 = 0;
  a1[1] = qword_9F6A8;
  if ( qword_9F6A8 )
    *(long long *)qword_9F6A8 = a1;
  qword_9F6A8 = (long long)a1;
  return a1;
}



// Function: unconsumed_option_free @ 0x2c541
long long unconsumed_option_free()
{
  long long result; // rax
  void *ptr; // [rsp+8h] [rbp-8h]

  while ( 1 )
  {
    result = qword_9F6A8;
    if ( !qword_9F6A8 )
      break;
    ptr = (void *)qword_9F6A8;
    qword_9F6A8 = *(long long *)(qword_9F6A8 + 8);
    free(ptr);
  }
  return result;
}



// Function: unconsumed_option_report @ 0x2c588
long long unconsumed_option_report(long long a1)
{
  long long result; // rax
  char *v2; // rax
  int v3; // eax
  long long v4; // rbx
  char *v5; // rax
  long long v6; // r12
  long long v7; // rbx
  char *v8; // rax
  long long v9; // rbx
  char *v10; // rax
  long long i; // [rsp+8h] [rbp-18h]

  result = qword_9F6A8;
  if ( qword_9F6A8 )
  {
    if ( qword_9FDC8 )
      qword_9FDC8(a1);
    v2 = gettext(
           "The following options were used after any non-optional arguments in archive create or update mode.  These opt"
           "ions are positional and affect only arguments that follow them.  Please, rearrange them properly.");
    error(0, 0, v2);
    dword_9FDD0 = 2;
    for ( i = qword_9F6A8; *(long long *)(i + 8); i = *(long long *)(i + 8) )
      ;
    while ( i )
    {
      v3 = *(int *)(i + 16);
      if ( v3 == 1 )
      {
        if ( qword_9FDC8 )
          qword_9FDC8(0);
        v4 = sub_5E84E(*(long long *)(i + 24));
        v5 = gettext("-C %s has no effect");
        error(0, 0, v5, v4);
        dword_9FDD0 = 2;
      }
      else if ( v3 == 4 )
      {
        if ( *(long long *)(i + 32) )
        {
          if ( qword_9FDC8 )
            qword_9FDC8(0);
          v6 = sub_5E84E(*(long long *)(i + 32));
          v7 = sub_2BA4C(*(int *)(i + 24));
          v8 = gettext("--%s %s has no effect");
          error(0, 0, v8, v7, v6);
          dword_9FDD0 = 2;
        }
        else
        {
          if ( qword_9FDC8 )
            qword_9FDC8(0);
          v9 = sub_2BA4C(*(int *)(i + 24));
          v10 = gettext("--%s has no effect");
          error(0, 0, v10, v9);
          dword_9FDD0 = 2;
        }
      }
      i = *(long long *)i;
    }
    return sub_2C541();
  }
  return result;
}



// Function: name_list_advance @ 0x2c78a
void name_list_advance()
{
  int *ptr; // [rsp+8h] [rbp-8h]

  ptr = (int *)qword_9F708;
  qword_9F708 = *(long long *)qword_9F708;
  if ( qword_9F708 )
    *(long long *)(qword_9F708 + 8) = 0;
  if ( ptr[4] == 4 || ptr[4] == 1 )
  {
    if ( dword_9F860 == 3 || dword_9F860 == 8 )
      sub_2C4F3(ptr);
  }
  else
  {
    if ( ptr[4] != 3 )
      sub_2C541();
    free(ptr);
  }
}



// Function: name_add_name @ 0x2c829
long long name_add_name(long long a1)
{
  long long *v1; // rax
  long long result; // rax

  v1 = sub_2C3F7();
  *((int *)v1 + 4) = 0;
  v1[3] = a1;
  result = (unsigned int)dword_9F6A0;
  if ( dword_9F6A0 )
  {
    if ( dword_9F6A0 == 1 )
      dword_9F6A0 = 2;
  }
  else
  {
    dword_9F6A0 = 1;
  }
  return result;
}



// Function: name_add_option @ 0x2c884
long long *name_add_option(int a1, long long a2)
{
  long long *result; // rax

  result = sub_2C3F7();
  *((int *)result + 4) = 4;
  *((int *)result + 6) = a1;
  result[4] = a2;
  return result;
}



// Function: name_add_dir @ 0x2c8c4
long long *name_add_dir(long long a1)
{
  long long *result; // rax

  result = sub_2C3F7();
  *((int *)result + 4) = 1;
  result[3] = a1;
  return result;
}



// Function: name_add_file @ 0x2c8f7
long long *name_add_file(long long a1)
{
  long long *result; // rax

  result = sub_2C3F7();
  *((int *)result + 4) = 2;
  result[3] = a1;
  result[4] = 0;
  result[6] = 0;
  dword_9F6A0 = 2;
  return result;
}



// Function: name_init @ 0x2c94c
long long name_init()
{
  qword_9F710 = (char *)sub_60B5B(102);
  qword_9F718 = 100;
  return sub_2C4B8();
}



// Function: name_term @ 0x2c978
void name_term()
{
  free(qword_9F710);
}



// Function: file_list_name @ 0x2c992
char *file_list_name()
{
  long long i; // [rsp+8h] [rbp-8h]

  for ( i = qword_9F708; i; i = *(long long *)i )
  {
    if ( *(int *)(i + 16) == 2 && *(long long *)(i + 48) )
      return *(char **)(i + 24);
  }
  return gettext("command line");
}



// Function: add_file_id @ 0x2c9f1
long long add_file_id(const char *a1)
{
  long long v1; // r12
  long long v2; // rbx
  char *v3; // rax
  long long *v5; // rax
  unsigned int v6; // [rsp+1Ch] [rbp-C4h]
  long long *i; // [rsp+20h] [rbp-C0h]
  char *v8; // [rsp+28h] [rbp-B8h]
  struct stat buf; // [rsp+30h] [rbp-B0h] BYREF
  unsigned long long v10; // [rsp+C8h] [rbp-18h]

  v10 = __readfsqword(0x28u);
  if ( stat(a1, &buf) )
    sub_3E9DD(a1);
  v8 = sub_2C992();
  for ( i = (long long *)qword_9F720; ; i = (long long *)*i )
  {
    if ( !i )
    {
      v5 = (long long *)sub_60B5B(32);
      *v5 = qword_9F720;
      v5[1] = buf.st_ino;
      v5[2] = buf.st_dev;
      v5[3] = v8;
      qword_9F720 = (long long)v5;
      return 0;
    }
    if ( i[1] == buf.st_ino && i[2] == buf.st_dev )
      break;
  }
  v6 = sub_5C9CD(0, 58, 1);
  if ( qword_9FDC8 )
    qword_9FDC8(0);
  v1 = i[3];
  v2 = sub_5E285(0, a1);
  v3 = gettext("%s: file list requested from %s already read from %s");
  error(0, 0, v3, v2, v8, v1);
  dword_9FDD0 = 2;
  sub_5C9CD(0, 58, v6);
  return 1;
}



// Function: chopslash @ 0x2cbf6
unsigned char *chopslash(const char *a1)
{
  unsigned char *v1; // rax
  unsigned char *result; // rax
  unsigned char *i; // [rsp+18h] [rbp-8h]

  for ( i = (unsigned char *)&a1[strlen(a1) - 1]; ; --i )
  {
    result = i;
    if ( i <= (unsigned char *)a1 )
      break;
    result = (unsigned char *)*i;
    if ( (char)result != 47 )
      break;
    v1 = i;
    *v1 = 0;
  }
  return result;
}



// Function: read_name_from_file @ 0x2cc4a
long long read_name_from_file(long long a1)
{
  long long v1; // rax
  int i; // [rsp+18h] [rbp-18h]
  char v4[12]; // [rsp+1Ch] [rbp-14h]
  FILE *stream; // [rsp+28h] [rbp-8h]

  *(int *)&v4[8] = 0;
  stream = *(FILE **)(a1 + 48);
  *(long long *)v4 = *(unsigned int *)(a1 + 40);
  ++*(long long *)(a1 + 32);
  for ( i = getc_unlocked(stream); i != -1 && i != *(int *)v4; i = getc_unlocked(stream) )
  {
    if ( *(long long *)&v4[4] == qword_9F718 )
      qword_9F710 = (char *)sub_60BD8(qword_9F710, &qword_9F718);
    v1 = (*(long long *)&v4[4])++;
    qword_9F710[v1] = i;
    if ( !i )
      return 2;
  }
  if ( !*(long long *)&v4[4] && i != -1 )
    return 3;
  if ( *(long long *)&v4[4] == qword_9F718 )
    qword_9F710 = (char *)sub_60BD8(qword_9F710, &qword_9F718);
  qword_9F710[*(long long *)&v4[4]] = 0;
  sub_2CBF6(qword_9F710);
  return !*(long long *)&v4[4] && i == -1;
}



// Function: handle_option @ 0x2cd8e
long long handle_option(char *a1, long long a2, double a3)
{
  long long v4; // rbx
  char *v5; // rax
  int i; // [rsp+1Ch] [rbp-124h]
  int v8; // [rsp+20h] [rbp-120h] BYREF
  long long v9; // [rsp+28h] [rbp-118h]
  long long v10; // [rsp+30h] [rbp-110h]
  long long v11; // [rsp+40h] [rbp-100h] BYREF
  long long *v12; // [rsp+48h] [rbp-F8h]
  long long v13; // [rsp+50h] [rbp-F0h]
  unsigned long long v14; // [rsp+128h] [rbp-18h]

  v14 = __readfsqword(0x28u);
  while ( *a1 && ((*__ctype_b_loc())[(char)*a1] & 0x2000) != 0 )
    ++a1;
  if ( *a1 != 45 )
    return 1;
  v13 = 1;
  if ( (unsigned int)sub_461A6(a1, &v11, 33558086, a3) )
  {
    if ( qword_9FDC8 )
      qword_9FDC8(a1);
    v4 = sub_4647F(&v11);
    v5 = gettext("cannot split string '%s': %s");
    error(0, 0, v5, a1, v4);
    sub_177F4();
  }
  *v12 = qword_9FE48;
  v8 = 2;
  v9 = *(long long *)(a2 + 24);
  v10 = *(long long *)(a2 + 32);
  sub_3828E((unsigned int)(v11 + v13), v12, &v8);
  for ( i = 0; i < (unsigned long long)(v11 + v13); ++i )
    v12[i] = 0;
  sub_46399(&v11, a3);
  return 0;
}



// Function: read_next_name @ 0x2cf89
long long read_next_name(long long a1, long long a2, double a3)
{
  unsigned int v4; // eax
  const char *v5; // rax

  if ( !*(long long *)(a1 + 48) )
  {
    if ( !strcmp(*(const char **)(a1 + 24), "-") )
    {
      sub_34A6F("-T");
      *(long long *)(a1 + 48) = stdin;
    }
    else
    {
      if ( (unsigned int)sub_2C9F1(*(const char **)(a1 + 24)) )
      {
        sub_2C78A();
        return 1;
      }
      *(long long *)(a1 + 48) = fopen(*(const char **)(a1 + 24), "r");
      if ( !*(long long *)(a1 + 48) )
        sub_3E3B4(*(long long *)(a1 + 24));
    }
    *(int *)(a1 + 40) = byte_9C270;
    *(char *)(a1 + 44) = byte_9F6B8;
  }
  do
    v4 = sub_2CC4A(a1);
  while ( v4 > 2 );
  if ( v4 == 2 )
  {
    if ( (dword_9EC84 & 0x200) != 0 )
    {
      if ( qword_9FDC8 )
        qword_9FDC8(a1);
      v5 = (const char *)sub_5E548(*(long long *)(a1 + 24));
      error(0, 0, "%s: file name read contains nul character", v5);
    }
    *(int *)(a1 + 40) = 0;
  }
  else if ( v4 )
  {
    if ( strcmp(*(const char **)(a1 + 24), "-") )
      fclose(*(FILE **)(a1 + 48));
    *(long long *)(a1 + 48) = 0;
    sub_2C78A();
    return 1;
  }
  if ( *(char *)(a1 + 44) == 1 )
    goto LABEL_23;
  if ( byte_9FC28 )
    sub_2961D(qword_9F710);
  if ( (unsigned int)sub_2CD8E(qword_9F710, a1, a3) )
  {
LABEL_23:
    *(int *)(a2 + 16) = 0;
    *(long long *)(a2 + 24) = qword_9F710;
    return 0;
  }
  else
  {
    sub_2C4B8();
    return 1;
  }
}



// Function: copy_name @ 0x2d1c1
unsigned char *copy_name(long long a1)
{
  char *s; // [rsp+10h] [rbp-10h]
  size_t v3; // [rsp+18h] [rbp-8h]

  s = *(char **)(a1 + 24);
  v3 = strlen(s);
  while ( v3 >= qword_9F718 )
    qword_9F710 = (char *)sub_60BD8(qword_9F710, &qword_9F718);
  strcpy(qword_9F710, s);
  return sub_2CBF6(qword_9F710);
}



// Function: name_next_elt @ 0x2d244
void *name_next_elt(long long a1, double a2)
{
  char *v3; // rax
  unsigned int v4; // eax
  int v5; // [rsp+Ch] [rbp-14h]
  long long v6; // [rsp+18h] [rbp-8h]

  v5 = a1;
  while ( 2 )
  {
    v6 = qword_9F708;
    if ( !qword_9F708 )
    {
      sub_2C588(a1);
      return 0;
    }
    switch ( *(int *)(qword_9F708 + 16) )
    {
      case 0:
        goto LABEL_8;
      case 1:
        if ( v5 )
        {
          v3 = (char *)sub_60CC6(*(long long *)(qword_9F708 + 24));
          v4 = sub_2AC09(v3);
          a1 = v4;
          sub_2ADF1(v4);
          sub_2C78A();
          continue;
        }
LABEL_8:
        sub_2D1C1(qword_9F708);
        if ( byte_9FC28 )
          sub_2961D(qword_9F710);
        dword_9F750 = *(int *)(v6 + 16);
        qword_9F758 = (long long)qword_9F710;
        sub_2C78A();
        return &unk_9F740;
      case 2:
        a1 = qword_9F708;
        if ( (unsigned int)sub_2CF89(qword_9F708, (long long)&unk_9F740, a2) )
          continue;
        return &unk_9F740;
      case 3:
        sub_2C78A();
        continue;
      case 4:
        a1 = *(unsigned int *)(qword_9F708 + 24);
        sub_2BC3A(a1, *(const char **)(qword_9F708 + 32));
        sub_2C78A();
        continue;
      default:
        continue;
    }
  }
}



// Function: name_next @ 0x2d37c
long long name_next(unsigned int a1, double a2)
{
  long long *v3; // [rsp+18h] [rbp-8h]

  v3 = sub_2D244(a1, a2);
  if ( v3 )
    return v3[3];
  else
    return 0;
}



// Function: name_gather @ 0x2d3b1
long long name_gather(double a1)
{
  char *v1; // rax
  int v2; // edx
  long long result; // rax
  char *v4; // rax
  unsigned int i; // [rsp+0h] [rbp-10h]
  unsigned int v6; // [rsp+4h] [rbp-Ch]
  void *v7; // [rsp+8h] [rbp-8h]
  long long *v8; // [rsp+8h] [rbp-8h]

  if ( byte_9F958 )
  {
    while ( 1 )
    {
      v7 = sub_2D244(0, a1);
      if ( !v7 || *((int *)v7 + 4) != 1 )
        break;
      v1 = (char *)sub_60CC6(*((long long *)v7 + 3));
      dword_9F778 = sub_2AC09(v1);
    }
    if ( v7 )
    {
      sub_2C3B1((void **)qword_9F780);
      qword_9F780 = sub_2C34C(*((void **)v7 + 3));
      *(int *)(qword_9F780 + 40) = dword_9F778;
      *(long long *)qword_9F780 = 0;
      *(long long *)(qword_9F780 + 48) = 0;
      if ( dword_9F6BC == 2 )
        v2 = 0x10000000;
      else
        v2 = 0;
      *(int *)(qword_9F780 + 32) = dword_9F6C0 | dword_9C274 | v2 | dword_9F930;
      *(long long *)(qword_9F780 + 56) = 0;
      *(long long *)(qword_9F780 + 64) = 0;
      *(char *)(qword_9F780 + 36) = 1;
      qword_9F700 = qword_9F780;
      qword_9F6F8 = qword_9F780;
      return qword_9F780;
    }
    else
    {
      result = (unsigned int)dword_9F778;
      if ( dword_9F778 )
        return sub_2D5B5(0, (unsigned int)dword_9F778, 0, 0);
    }
  }
  else
  {
    for ( i = 0; ; sub_2D5B5(v8[3], i, 1, 0) )
    {
      v6 = i;
      while ( 1 )
      {
        v8 = sub_2D244(0, a1);
        if ( !v8 || *((int *)v8 + 4) != 1 )
          break;
        v4 = (char *)sub_60CC6(v8[3]);
        i = sub_2AC09(v4);
      }
      if ( !v8 )
        break;
    }
    result = i;
    if ( i != v6 )
      return sub_2D5B5(0, i, 0, 0);
  }
  return result;
}



// Function: addname @ 0x2d5b5
long long addname(void *a1, int a2, char a3, long long a4)
{
  int v4; // edx
  long long v8; // [rsp+28h] [rbp-8h]

  v8 = sub_2C34C(a1);
  *(long long *)(v8 + 8) = qword_9F700;
  *(long long *)v8 = 0;
  *(long long *)(v8 + 48) = 0;
  if ( dword_9F6BC == 2 )
    v4 = 0x10000000;
  else
    v4 = 0;
  *(int *)(v8 + 32) = dword_9F930 | dword_9F6C0 | dword_9C274 | v4;
  *(int *)(v8 + 40) = a2;
  *(long long *)(v8 + 56) = 0;
  *(long long *)(v8 + 64) = a4;
  *(char *)(v8 + 36) = a3;
  if ( qword_9F700 )
    *(long long *)qword_9F700 = v8;
  else
    qword_9F6F8 = v8;
  qword_9F700 = v8;
  return v8;
}



// Function: add_starting_file @ 0x2d6a2
long long add_starting_file(void *a1)
{
  int v1; // edx
  long long v3; // [rsp+10h] [rbp-10h]
  void **v4; // [rsp+18h] [rbp-8h]

  v3 = sub_2C34C(a1);
  if ( byte_9F988 )
  {
    v4 = (void **)qword_9F6F8;
    sub_2D817(qword_9F6F8);
    sub_2C3B1(v4);
  }
  *(long long *)(v3 + 8) = 0;
  *(long long *)v3 = qword_9F6F8;
  qword_9F6F8 = v3;
  if ( !qword_9F700 )
    qword_9F700 = qword_9F6F8;
  *(long long *)(v3 + 48) = 0;
  if ( dword_9F6BC == 2 )
    v1 = 0x10000000;
  else
    v1 = 0;
  *(int *)(v3 + 32) = dword_9F930 | dword_9F6C0 | dword_9C274 | v1;
  *(int *)(v3 + 40) = 0;
  *(long long *)(v3 + 56) = 0;
  *(long long *)(v3 + 64) = 0;
  *(char *)(v3 + 36) = 1;
  byte_9F988 = 1;
  return v3;
}



// Function: namelist_match @ 0x2d7a6
long long namelist_match(long long a1)
{
  long long i; // [rsp+18h] [rbp-8h]

  for ( i = qword_9F6F8; i; i = *(long long *)i )
  {
    if ( **(char **)(i + 16) && (unsigned char)sub_4EAD5(*(long long *)(i + 16), a1, *(unsigned int *)(i + 32)) )
      return i;
  }
  return 0;
}



// Function: remname @ 0x2d817
long long remname(long long *a1)
{
  long long result; // rax
  long long *v2; // [rsp+10h] [rbp-8h]

  v2 = (long long *)a1[1];
  if ( v2 )
    *v2 = *a1;
  else
    qword_9F6F8 = *a1;
  if ( *a1 )
  {
    result = *a1;
    *(long long *)(*a1 + 8) = a1[1];
  }
  else
  {
    result = a1[1];
    qword_9F700 = result;
  }
  return result;
}



// Function: name_match @ 0x2d88a
bool name_match(const char *a1, double a2)
{
  long long v3; // [rsp+10h] [rbp-10h]

  strlen(a1);
  while ( 1 )
  {
    if ( !qword_9F6F8 )
      return 1;
    if ( !**(char **)(qword_9F6F8 + 16) )
    {
      sub_2ADF1(*(int *)(qword_9F6F8 + 40));
      qword_9F6F8 = 0;
      qword_9F700 = 0;
      return 1;
    }
    v3 = sub_2D7A6((long long)a1);
    if ( byte_9F988 )
    {
      if ( v3 == qword_9F6F8 )
        byte_9F988 = 0;
      else
        v3 = 0;
    }
    if ( v3 )
      break;
    if ( !byte_9F958 || !*(long long *)(qword_9F6F8 + 48) )
      return 0;
    sub_2D3B1(a2);
    if ( *(long long *)(qword_9F6F8 + 48) )
      return 0;
  }
  if ( a1[*(long long *)(v3 + 24)] != 47 || !dword_9F930 || !*(long long *)(v3 + 48) )
    ++*(long long *)(v3 + 48);
  sub_2ADF1(*(int *)(v3 + 40));
  if ( qword_9F8D0 )
    return *(long long *)(v3 + 48) == qword_9F8D0;
  else
    return *(long long *)(v3 + 48) != 0;
}



// Function: all_names_found @ 0x2da15
long long all_names_found(long long a1)
{
  long long i; // [rsp+10h] [rbp-10h]
  size_t v3; // [rsp+18h] [rbp-8h]

  if ( !*(long long *)(a1 + 8) || !qword_9F8D0 || *(char *)(a1 + 16) )
    return 0;
  v3 = strlen(*(const char **)(a1 + 8));
  for ( i = qword_9F6F8; i; i = *(long long *)i )
  {
    if ( **(char **)(i + 16) )
    {
      if ( qword_9F8D0 )
      {
        if ( *(long long *)(i + 48) < (unsigned long long)qword_9F8D0 )
          return 0;
      }
      else if ( !*(long long *)(i + 48) )
      {
        return 0;
      }
    }
    if ( v3 >= *(long long *)(i + 24) && *(char *)(*(long long *)(a1 + 8) + *(long long *)(i + 24)) == 47 )
      return 0;
  }
  return 1;
}



// Function: regex_usage_warning @ 0x2dafb
long long regex_usage_warning(long long a1)
{
  char *v1; // rax
  char *v2; // rax

  if ( !dword_9F6BC && (unsigned char)sub_4E397(a1, 0) )
  {
    dword_9F788 = 1;
    if ( qword_9FDC8 )
      qword_9FDC8(a1);
    v1 = gettext("Pattern matching characters used in file names");
    error(0, 0, v1);
    if ( qword_9FDC8 )
      qword_9FDC8(0);
    v2 = gettext("Use --wildcards to enable pattern matching, or --no-wildcards to suppress this warning");
    error(0, 0, v2);
  }
  return (unsigned int)dword_9F788;
}



// Function: names_notfound @ 0x2dbba
long long names_notfound(double a1)
{
  bool v1; // al
  long long v2; // rdi
  long long v3; // rbx
  char *v4; // rax
  long long result; // rax
  long long v6; // rbx
  char *v7; // rax
  long long *i; // [rsp+0h] [rbp-20h]
  long long v9; // [rsp+8h] [rbp-18h]

  for ( i = (long long *)qword_9F6F8; i; i = (long long *)*i )
  {
    if ( qword_9F8D0 )
      v1 = i[6] < (unsigned long long)qword_9F8D0;
    else
      v1 = i[6] == 0;
    if ( v1 && *(char *)i[2] )
    {
      v2 = i[2];
      sub_2DAFB(v2);
      if ( qword_9FDC8 )
        qword_9FDC8(v2);
      v3 = sub_5E548(i[2]);
      if ( i[6] )
        v4 = gettext("%s: Required occurrence not found in archive");
      else
        v4 = gettext("%s: Not found in archive");
      error(0, 0, v4, v3);
      dword_9FDD0 = 2;
    }
  }
  qword_9F6F8 = 0;
  qword_9F700 = 0;
  result = (unsigned char)byte_9F958;
  if ( byte_9F958 )
  {
    while ( 1 )
    {
      result = sub_2D37C(1u, a1);
      v9 = result;
      if ( !result )
        break;
      sub_2DAFB(result);
      if ( qword_9FDC8 )
        qword_9FDC8(v9);
      v6 = sub_5E548(v9);
      v7 = gettext("%s: Not found in archive");
      error(0, 0, v7, v6);
      dword_9FDD0 = 2;
    }
  }
  return result;
}



// Function: label_notfound @ 0x2dd64
char label_notfound(double a1)
{
  long long v1; // rax
  char *v2; // rax
  long long *i; // [rsp+0h] [rbp-10h]
  long long *j; // [rsp+0h] [rbp-10h]

  LOBYTE(v1) = qword_9F6F8;
  if ( qword_9F6F8 )
  {
    for ( i = (long long *)qword_9F6F8; i; i = (long long *)*i )
    {
      if ( qword_9F8D0 )
        LOBYTE(v1) = i[6] >= (unsigned long long)qword_9F8D0;
      else
        LOBYTE(v1) = i[6] != 0;
      if ( (char)v1 )
        return v1;
    }
    if ( dword_9F9AC )
    {
      v2 = gettext("Archive label mismatch");
      error(0, 0, v2);
    }
    sub_39E98(1);
    for ( j = (long long *)qword_9F6F8; j && !(unsigned int)sub_2DAFB(j[2]); j = (long long *)*j )
      ;
    qword_9F6F8 = 0;
    qword_9F700 = 0;
    LOBYTE(v1) = byte_9F958;
    if ( byte_9F958 )
    {
      do
      {
        v1 = sub_2D37C(1u, a1);
        if ( !v1 )
          break;
        LODWORD(v1) = sub_2DAFB(v1);
      }
      while ( !(int)v1 );
    }
  }
  return v1;
}



// Function: merge_sort_sll @ 0x2de97
long long *merge_sort_sll(long long *a1, int a2, int (*a3)(long long **, long long **))
{
  int i; // [rsp+24h] [rbp-3Ch]
  long long *v6; // [rsp+30h] [rbp-30h] BYREF
  long long **v7; // [rsp+38h] [rbp-28h]
  long long **v8; // [rsp+40h] [rbp-20h]
  long long *v9; // [rsp+48h] [rbp-18h]
  long long *v10; // [rsp+50h] [rbp-10h]
  unsigned long long v11; // [rsp+58h] [rbp-8h]

  v11 = __readfsqword(0x28u);
  if ( a2 == 1 )
    return a1;
  if ( a2 == 2 )
  {
    if ( a3((long long **)a1, (long long **)*a1) <= 0 )
    {
      return a1;
    }
    else
    {
      v6 = (long long *)*a1;
      *v6 = a1;
      *a1 = 0;
      return v6;
    }
  }
  else
  {
    v7 = (long long **)a1;
    v10 = a1;
    for ( i = (a2 + 1) / 2 - 1; i; --i )
      v10 = (long long *)*v10;
    v8 = (long long **)*v10;
    *v10 = 0;
    v7 = (long long **)merge_sort_sll(v7, (unsigned int)((a2 + 1) / 2), a3);
    v8 = (long long **)merge_sort_sll(v8, (unsigned int)(a2 / 2), a3);
    v9 = &v6;
    while ( v7 && v8 )
    {
      if ( a3(v7, v8) >= 0 )
      {
        v10 = *v8;
        *v9 = v8;
        v9 = v8;
        v8 = (long long **)v10;
      }
      else
      {
        v10 = *v7;
        *v9 = v7;
        v9 = v7;
        v7 = (long long **)v10;
      }
    }
    if ( v7 )
      *v9 = v7;
    else
      *v9 = v8;
    return v6;
  }
}



// Function: merge_sort @ 0x2e071
long long *merge_sort(long long *a1, int a2, int (*a3)(long long **, long long **))
{
  long long *i; // [rsp+28h] [rbp-18h]
  long long *v5; // [rsp+30h] [rbp-10h]
  long long *v6; // [rsp+38h] [rbp-8h]

  v6 = sub_2DE97(a1, a2, a3);
  v5 = 0;
  for ( i = v6; i; i = (long long *)*i )
  {
    i[1] = v5;
    v5 = i;
  }
  return v6;
}



// Function: compare_names_found @ 0x2e0df
int compare_names_found(long long a1, long long a2)
{
  int v2; // eax
  int v3; // edx

  if ( qword_9F8D0 )
    v2 = *(long long *)(a2 + 48) >= (unsigned long long)qword_9F8D0;
  else
    v2 = *(long long *)(a2 + 48) != 0;
  if ( qword_9F8D0 )
    v3 = *(long long *)(a1 + 48) >= (unsigned long long)qword_9F8D0;
  else
    v3 = *(long long *)(a1 + 48) != 0;
  if ( v2 == v3 )
    return strcmp(*(const char **)(a1 + 16), *(const char **)(a2 + 16));
  else
    return v2 - v3;
}



// Function: compare_names @ 0x2e18e
int compare_names(long long a1, long long a2)
{
  return strcmp(*(const char **)(a1 + 16), *(const char **)(a2 + 16));
}



// Function: add_hierarchy_to_namelist @ 0x2e1bf
unsigned long long add_hierarchy_to_namelist(long long a1, long long a2)
{
  long long v2; // rax
  unsigned long long v3; // rax
  int v4; // ebx
  int fd; // [rsp+10h] [rbp-220h]
  int v7; // [rsp+14h] [rbp-21Ch]
  unsigned long long v8; // [rsp+18h] [rbp-218h] BYREF
  long long v9; // [rsp+20h] [rbp-210h]
  long long v10; // [rsp+28h] [rbp-208h]
  unsigned long long v11; // [rsp+30h] [rbp-200h]
  char *dest; // [rsp+38h] [rbp-1F8h]
  char *s; // [rsp+40h] [rbp-1F0h]
  char *v14; // [rsp+48h] [rbp-1E8h]
  size_t v15; // [rsp+50h] [rbp-1E0h]
  long long v16; // [rsp+58h] [rbp-1D8h]
  long long v17[11]; // [rsp+60h] [rbp-1D0h] BYREF
  struct stat v18[2]; // [rsp+B8h] [rbp-178h] BYREF
  long long v19; // [rsp+1F8h] [rbp-38h]
  int v20; // [rsp+208h] [rbp-28h]
  unsigned long long v21; // [rsp+218h] [rbp-18h]

  v21 = __readfsqword(0x28u);
  *(long long *)(a2 + 56) = sub_21EB9(a1);
  v14 = (char *)sub_2241B(*(long long *)(a2 + 56));
  if ( v14 )
  {
    v9 = 0;
    v10 = 0;
    v11 = *(long long *)(a2 + 24);
    if ( v11 <= 0x63 )
      v2 = 102;
    else
      v2 = v11 + 102;
    v8 = v2;
    dest = (char *)sub_60B5B(v2);
    v7 = *(int *)(a2 + 40);
    strcpy(dest, *(const char **)(a2 + 16));
    if ( dest[v11 - 1] != 47 )
    {
      v3 = v11++;
      dest[v3] = 47;
      dest[v11] = 0;
    }
    for ( s = v14; *s; s += v15 + 1 )
    {
      v15 = strlen(s);
      if ( *s == 68 )
      {
        while ( v15 + v11 > v8 )
          dest = (char *)sub_60BD8(dest, &v8);
        strcpy(&dest[v11], s + 1);
        v16 = sub_2D5B5(dest, v7, 0, a2);
        if ( v9 )
          *(long long *)(v10 + 80) = v16;
        else
          v9 = v16;
        v10 = v16;
        sub_39C55(v17);
        v19 = a1;
        if ( *(int *)(a1 + 424) >= 0 )
        {
          fd = sub_157F9(a1, s + 1, oflag | 0x10000u);
        }
        else
        {
          fd = -1;
          v4 = *(int *)(a1 + 424);
          *__errno_location() = -v4;
        }
        if ( fd >= 0 )
        {
          v20 = fd;
          if ( fstat(fd, v18) )
          {
            sub_2B444((long long)dest);
          }
          else
          {
            v17[0] = sub_60CC6(dest);
            add_hierarchy_to_namelist(v17, v16);
            sub_1588E((long long)v17);
          }
        }
        else
        {
          sub_2B2B1((long long)dest);
        }
        sub_39D1C(v17);
      }
    }
    free(dest);
    *(long long *)(a2 + 72) = v9;
  }
  return v21 - __readfsqword(0x28u);
}



// Function: name_hash @ 0x2e590
long long name_hash(long long a1, long long a2)
{
  return sub_52834(*(long long *)(a1 + 88), a2);
}



// Function: name_compare @ 0x2e5c5
bool name_compare(long long a1, long long a2)
{
  return strcmp(*(const char **)(a1 + 88), *(const char **)(a2 + 88)) == 0;
}



// Function: rebase_child_list @ 0x2e60b
char *rebase_child_list(long long a1, long long a2)
{
  char *result; // rax
  char *v3; // [rsp+8h] [rbp-38h]
  long long v4; // [rsp+18h] [rbp-28h]
  long long v5; // [rsp+20h] [rbp-20h]
  char *src; // [rsp+28h] [rbp-18h]
  long long v7; // [rsp+30h] [rbp-10h]
  char *dest; // [rsp+38h] [rbp-8h]

  v3 = (char *)a1;
  v4 = *(long long *)(*(long long *)(a1 + 64) + 24LL);
  v5 = *(long long *)(a2 + 24);
  result = *(char **)(a2 + 16);
  src = result;
  while ( v3 )
  {
    v7 = *((long long *)v3 + 3) - v4 + v5;
    dest = (char *)sub_60B5B(v7 + 1);
    strcpy(dest, src);
    strcat(dest, (const char *)(v4 + *((long long *)v3 + 2)));
    free(*((void **)v3 + 2));
    *((long long *)v3 + 2) = dest;
    *((long long *)v3 + 3) = v7;
    sub_21244(*((long long *)v3 + 7), *(long long *)(*((long long *)v3 + 8) + 16LL), v4, (long long)src, v5);
    result = (char *)*((long long *)v3 + 10);
    v3 = result;
  }
  return result;
}



// Function: collect_and_sort_names @ 0x2e719
unsigned long long collect_and_sort_names(const char *a1, double a2)
{
  int v2; // eax
  char *v3; // rax
  char *v4; // rax
  int v6; // [rsp+0h] [rbp-1F0h]
  int v7; // [rsp+0h] [rbp-1F0h]
  int fd; // [rsp+4h] [rbp-1ECh]
  long long v9; // [rsp+8h] [rbp-1E8h]
  long long i; // [rsp+8h] [rbp-1E8h]
  long long j; // [rsp+8h] [rbp-1E8h]
  long long v12; // [rsp+10h] [rbp-1E0h]
  long long v13; // [rsp+18h] [rbp-1D8h]
  long long v14; // [rsp+20h] [rbp-1D0h]
  long long *v15; // [rsp+28h] [rbp-1C8h]
  long long v16[11]; // [rsp+30h] [rbp-1C0h] BYREF
  struct stat v17[2]; // [rsp+88h] [rbp-168h] BYREF
  int v18; // [rsp+1D8h] [rbp-18h]
  unsigned long long v19; // [rsp+1E8h] [rbp-8h]

  v19 = __readfsqword(0x28u);
  v12 = 0;
  sub_2D3B1(a2);
  if ( !qword_9F6F8 )
  {
    a1 = ".";
    sub_2D5B5(".", 0, 0, 0);
  }
  if ( qword_9F8E0 )
  {
    v2 = sub_2ABE0();
    if ( v2 )
    {
      if ( v2 != 1 )
      {
        if ( qword_9FDC8 )
          qword_9FDC8(a1);
        v4 = gettext("Only one -C option is allowed with --listed-incremental");
        error(0, 0, v4);
        sub_3814C(2);
      }
      if ( !*(int *)(qword_9F6F8 + 40) )
      {
        if ( qword_9FDC8 )
          qword_9FDC8(a1);
        v3 = gettext("Using -C option inside file list is not allowed with --listed-incremental");
        error(0, 0, v3);
        sub_3814C(2);
      }
    }
    sub_24090();
  }
  v6 = 0;
  v9 = qword_9F6F8;
  while ( v9 )
  {
    if ( !*(long long *)(v9 + 48) && !*(long long *)(v9 + 56) && (*(int *)(v9 + 32) & 0x10000000) == 0 )
    {
      sub_2ADF1(*(int *)(v9 + 40));
      if ( **(char **)(v9 + 16) )
      {
        sub_39C55(v16);
        if ( sub_2A9B8(*(const char **)(v9 + 16), v17) )
        {
          sub_2B444(*(long long *)(v9 + 16));
        }
        else
        {
          if ( (v17[0].st_mode & 0xF000) == 0x4000 )
          {
            fd = openat(newfd, *(const char **)(v9 + 16), oflag | 0x10000);
            if ( fd >= 0 )
            {
              v18 = fd;
              if ( fstat(fd, v17) )
              {
                sub_2B444(*(long long *)(v9 + 16));
              }
              else
              {
                v16[0] = sub_60CC6(*(long long *)(v9 + 16));
                ++*(long long *)(v9 + 48);
                sub_2E1BF((long long)v16, v9);
              }
            }
            else
            {
              sub_2B2B1(*(long long *)(v9 + 16));
            }
          }
          sub_39D1C(v16);
        }
      }
    }
    v9 = *(long long *)v9;
    ++v6;
  }
  qword_9F6F8 = (long long)sub_2E071((long long *)qword_9F6F8, v6, (int (*)(long long **, long long **))sub_2E18E);
  v7 = 0;
  v13 = sub_52C1C(0, 0, sub_2E590, sub_2E5C5, 0);
  for ( i = qword_9F6F8; i; i = v14 )
  {
    v14 = *(long long *)i;
    *(long long *)(i + 88) = sub_29AAC(*(int *)(i + 40), *(const char **)(i + 16));
    if ( !v12 )
      goto LABEL_40;
    v15 = (long long *)sub_52567(v13, i);
    if ( !v15 )
      goto LABEL_40;
    if ( !*(long long *)(i + 64) )
    {
      if ( v15[9] )
        sub_2E60B(v15[9], i);
      sub_539CD(v13, i);
      sub_2D817(v15);
      sub_2C3B1((void **)v15);
      --v7;
LABEL_40:
      *(long long *)(i + 48) = 0;
      if ( !sub_5395D(v13, i) )
        sub_1783C();
      v12 = i;
      ++v7;
      continue;
    }
    if ( *(long long *)(i + 72) )
      sub_2E60B(*(long long *)(i + 72), (long long)v15);
    sub_2D817((long long *)i);
    sub_2C3B1((void **)i);
  }
  qword_9F700 = v12;
  sub_52E9C(v13);
  qword_9F6F8 = (long long)sub_2E071((long long *)qword_9F6F8, v7, (int (*)(long long **, long long **))sub_2E0DF);
  if ( qword_9F8E0 )
  {
    for ( j = qword_9F6F8; j && !**(char **)(j + 16); j = *(long long *)j )
      ;
    if ( j )
      sub_228CD(*(long long *)(j + 56));
  }
  return v19 - __readfsqword(0x28u);
}



// Function: name_scan @ 0x2ed0e
long long name_scan(const char *a1, double a2)
{
  long long v3; // [rsp+18h] [rbp-8h]

  strlen(a1);
  while ( 1 )
  {
    v3 = sub_2D7A6((long long)a1);
    if ( v3 )
      return v3;
    if ( !byte_9F958 || !qword_9F6F8 || !*(long long *)(qword_9F6F8 + 48) )
      break;
    sub_2D3B1(a2);
    if ( *(long long *)(qword_9F6F8 + 48) )
      return 0;
  }
  return 0;
}



// Function: name_from_list @ 0x2eda0
long long name_from_list()
{
  if ( !qword_9F6B0 )
    qword_9F6B0 = qword_9F6F8;
  while ( qword_9F6B0 && (*(long long *)(qword_9F6B0 + 48) || !**(char **)(qword_9F6B0 + 16)) )
    qword_9F6B0 = *(long long *)qword_9F6B0;
  if ( !qword_9F6B0 )
    return 0;
  ++*(long long *)(qword_9F6B0 + 48);
  sub_2ADF1(*(int *)(qword_9F6B0 + 40));
  return qword_9F6B0;
}



// Function: blank_name_list @ 0x2ee43
long long blank_name_list()
{
  long long result; // rax
  long long *i; // [rsp+0h] [rbp-8h]

  qword_9F6B0 = 0;
  result = qword_9F6F8;
  for ( i = (long long *)qword_9F6F8; i; i = (long long *)*i )
  {
    i[6] = 0;
    result = *i;
  }
  return result;
}



// Function: make_file_name @ 0x2ee85
char *make_file_name(const char *a1, const char *a2)
{
  int v2; // eax
  int v4; // [rsp+14h] [rbp-1Ch]
  size_t n; // [rsp+18h] [rbp-18h]
  size_t v6; // [rsp+20h] [rbp-10h]
  char *dest; // [rsp+28h] [rbp-8h]

  n = strlen(a1);
  v6 = strlen(a2) + 1;
  v2 = n && a1[n - 1] != 47;
  v4 = v2;
  dest = (char *)sub_60B5B(n + v2 + v6);
  memcpy(dest, a1, n);
  dest[n] = 47;
  memcpy(&dest[n + v4], a2, v6);
  return dest;
}



// Function: stripped_prefix_len @ 0x2ef5c
long long stripped_prefix_len(char *a1, long long a2)
{
  char *i; // [rsp+18h] [rbp-8h]

  for ( i = a1; *i == 47; ++i )
    ;
  while ( *i )
  {
    if ( *i++ == 47 )
    {
      if ( !--a2 )
        return i - a1;
      while ( *i == 47 )
        ++i;
    }
  }
  return -1;
}



// Function: contains_dot_dot @ 0x2efdc
long long contains_dot_dot(char *a1)
{
  char *v2; // rax

  while ( *a1 != 46 || a1[1] != 46 || a1[2] != 47 && a1[2] )
  {
    while ( *a1 != 47 )
    {
      v2 = a1++;
      if ( !*v2 )
        return 0;
    }
    ++a1;
  }
  return 1;
}



// Function: dump_zeros @ 0x2f05d
long long dump_zeros(long long a1, long long a2)
{
  signed long long v3; // rax
  ssize_t v4; // [rsp+18h] [rbp-8h]

  if ( a2 >= *(long long *)(a1 + 8) )
  {
    while ( 1 )
    {
      if ( a2 <= *(long long *)(a1 + 8) )
        return 1;
      v3 = a2 - *(long long *)(a1 + 8);
      if ( v3 > 512 )
        v3 = 512;
      v4 = write(*(int *)a1, &unk_83D00, v3);
      if ( v4 <= 0 )
        break;
      *(long long *)(a1 + 8) += v4;
    }
    if ( !v4 )
      *__errno_location() = 22;
    return 0;
  }
  else
  {
    *__errno_location() = 22;
    return 0;
  }
}



// Function: tar_sparse_member_p @ 0x2f126
long long tar_sparse_member_p(long long a1)
{
  if ( *(long long *)(*(long long *)(a1 + 32) + 16LL) )
    return (*(long long (**)(long long))(*(long long *)(a1 + 32) + 16LL))(a1);
  else
    return 0;
}



// Function: tar_sparse_init @ 0x2f165
long long tar_sparse_init(long long (***a1)(long long))
{
  memset(a1, 0, 0x30u);
  if ( (unsigned char)sub_2FA5C(a1) != 1 )
    return 0;
  if ( *a1[4] )
    return (*a1[4])(a1);
  return 1;
}



// Function: tar_sparse_done @ 0x2f1d2
long long tar_sparse_done(long long a1)
{
  if ( *(long long *)(*(long long *)(a1 + 32) + 8LL) )
    return (*(long long (**)(long long))(*(long long *)(a1 + 32) + 8LL))(a1);
  else
    return 1;
}



// Function: tar_sparse_scan @ 0x2f211
long long tar_sparse_scan(long long a1, unsigned int a2, long long a3)
{
  if ( *(long long *)(*(long long *)(a1 + 32) + 48LL) )
    return (*(long long (**)(long long, long long, long long))(*(long long *)(a1 + 32) + 48LL))(a1, a2, a3);
  else
    return 1;
}



// Function: tar_sparse_dump_region @ 0x2f261
long long tar_sparse_dump_region(long long a1, long long a2)
{
  if ( *(long long *)(*(long long *)(a1 + 32) + 56LL) )
    return (*(long long (**)(long long, long long))(*(long long *)(a1 + 32) + 56LL))(a1, a2);
  else
    return 0;
}



// Function: tar_sparse_extract_region @ 0x2f2ab
long long tar_sparse_extract_region(long long a1, long long a2)
{
  if ( *(long long *)(*(long long *)(a1 + 32) + 64LL) )
    return (*(long long (**)(long long, long long))(*(long long *)(a1 + 32) + 64LL))(a1, a2);
  else
    return 0;
}



// Function: tar_sparse_dump_header @ 0x2f2f5
long long tar_sparse_dump_header(long long a1)
{
  if ( *(long long *)(*(long long *)(a1 + 32) + 24LL) )
    return (*(long long (**)(long long))(*(long long *)(a1 + 32) + 24LL))(a1);
  else
    return 0;
}



// Function: tar_sparse_decode_header @ 0x2f334
long long tar_sparse_decode_header(long long a1)
{
  if ( *(long long *)(*(long long *)(a1 + 32) + 40LL) )
    return (*(long long (**)(long long))(*(long long *)(a1 + 32) + 40LL))(a1);
  else
    return 1;
}



// Function: tar_sparse_fixup_header @ 0x2f373
long long tar_sparse_fixup_header(long long a1)
{
  if ( *(long long *)(*(long long *)(a1 + 32) + 32LL) )
    return (*(long long (**)(long long))(*(long long *)(a1 + 32) + 32LL))(a1);
  else
    return 1;
}



// Function: lseek_or_error @ 0x2f3b2
long long lseek_or_error(long long a1, __off_t a2)
{
  unsigned long long v2; // rax

  if ( *(char *)(a1 + 4) )
    v2 = (unsigned long long)lseek(*(int *)a1, a2, 0) >> 63;
  else
    LOBYTE(v2) = sub_2F05D(a1, a2) ^ 1;
  if ( !(char)v2 )
    return 1;
  sub_2B3EB(**(long long **)(a1 + 24), a2);
  return 0;
}



// Function: zero_block_p @ 0x2f433
long long zero_block_p(char *a1, long long a2)
{
  char *v2; // rax

  while ( a2-- )
  {
    v2 = a1++;
    if ( *v2 )
      return 0;
  }
  return 1;
}



// Function: sparse_add_map @ 0x2f477
long long *sparse_add_map(long long *a1, long long *a2)
{
  long long *v2; // rcx
  long long v3; // rdx
  long long v5; // [rsp+10h] [rbp-10h]
  long long v6; // [rsp+18h] [rbp-8h]

  v5 = a1[40];
  v6 = a1[38];
  if ( v6 == a1[39] )
  {
    v5 = sub_60A5F(v5, a1 + 39, 16);
    a1[40] = v5;
  }
  v2 = (long long *)(16 * v6 + v5);
  v3 = a2[1];
  *v2 = *a2;
  v2[1] = v3;
  a1[38] = v6 + 1;
  return a1;
}



// Function: sparse_scan_file_raw @ 0x2f527
long long sparse_scan_file_raw(long long a1)
{
  unsigned int v2; // [rsp+14h] [rbp-23Ch]
  long long v3; // [rsp+18h] [rbp-238h]
  long long *v4; // [rsp+20h] [rbp-230h]
  long long v5; // [rsp+28h] [rbp-228h]
  long long v6; // [rsp+30h] [rbp-220h] BYREF
  long long v7; // [rsp+38h] [rbp-218h]
  char v8[520]; // [rsp+40h] [rbp-210h] BYREF
  unsigned long long v9; // [rsp+248h] [rbp-8h]

  v9 = __readfsqword(0x28u);
  v4 = *(long long **)(a1 + 24);
  v2 = *(int *)a1;
  v3 = 0;
  v6 = 0;
  v7 = 0;
  v4[35] = 0;
  if ( (unsigned char)sub_2F211(a1, 0, 0) != 1 )
    return 0;
  while ( 1 )
  {
    v5 = sub_2A9E9(v2, (long long)v8, 512);
    if ( !v5 || v5 == -1 )
      break;
    if ( (unsigned char)sub_2F433(v8, v5) )
    {
      if ( v7 )
      {
        sub_2F477(v4, &v6);
        v7 = 0;
        if ( (unsigned char)sub_2F211(a1, 1u, 0) != 1 )
          return 0;
      }
    }
    else
    {
      if ( !v7 )
        v6 = v3;
      v7 += v5;
      v4[35] += v5;
      if ( (unsigned char)sub_2F211(a1, 1u, (long long)v8) != 1 )
        return 0;
    }
    v3 += v5;
  }
  if ( !v7 )
    v6 = v3;
  sub_2F477(v4, &v6);
  v4[35] += v5;
  return sub_2F211(a1, 2u, 0);
}



// Function: sparse_scan_file_wholesparse @ 0x2f7c6
long long sparse_scan_file_wholesparse(long long a1)
{
  long long *v2; // [rsp+18h] [rbp-28h]
  long long v3[4]; // [rsp+20h] [rbp-20h] BYREF

  v3[3] = __readfsqword(0x28u);
  v2 = *(long long **)(a1 + 24);
  v3[0] = 0;
  v3[1] = 0;
  if ( v2[19] )
    return 0;
  v2[35] = 0;
  v3[0] = v2[17];
  sub_2F477(v2, v3);
  return 1;
}



// Function: sparse_scan_file_seek @ 0x2f864
long long sparse_scan_file_seek(long long a1)
{
  __off_t i; // rax
  int fd; // [rsp+1Ch] [rbp-44h]
  __off_t offset; // [rsp+20h] [rbp-40h]
  long long *v5; // [rsp+28h] [rbp-38h]
  __off_t v6; // [rsp+30h] [rbp-30h]
  __off_t v7; // [rsp+38h] [rbp-28h]
  __off_t v8; // [rsp+40h] [rbp-20h] BYREF
  __off_t v9; // [rsp+48h] [rbp-18h]
  unsigned long long v10; // [rsp+58h] [rbp-8h]

  v10 = __readfsqword(0x28u);
  v5 = *(long long **)(a1 + 24);
  fd = *(int *)a1;
  v8 = 0;
  v9 = 0;
  offset = 0;
  v5[35] = 0;
  for ( i = lseek(fd, 0, 3); ; i = lseek(fd, v7, 3) )
  {
    v6 = i;
    if ( i == -1 )
      break;
    v7 = lseek(fd, i, 4);
    if ( !offset && !v6 && v7 == v5[17] )
    {
      lseek(fd, 0, 0);
      return 0;
    }
    v8 = v6;
    v9 = v7 - v6;
    sub_2F477(v5, &v8);
    v5[35] += v9;
    offset = v7;
  }
  if ( *__errno_location() != 6 )
    return 0;
  v9 = 0;
  v8 = v5[17];
  sub_2F477(v5, &v8);
  return 1;
}



// Function: sparse_scan_file @ 0x2f9e5
long long sparse_scan_file(long long a1)
{
  if ( (unsigned char)sub_2F7C6(a1) )
    return 1;
  if ( dword_9F984 == 2 )
    goto LABEL_15;
  if ( (unsigned int)dword_9F984 > 2 )
    return 0;
  if ( !dword_9F984 )
  {
LABEL_15:
    if ( (unsigned char)sub_2F864(a1) )
      return 1;
  }
  else if ( dword_9F984 != 1 )
  {
    return 0;
  }
  return (unsigned char)sub_2F527(a1) != 0;
}



// Function: sparse_select_optab @ 0x2fa5c
long long sparse_select_optab(long long a1)
{
  int v1; // eax
  long long result; // rax

  if ( dword_9F468 )
    v1 = dword_9F468;
  else
    v1 = dword_9F864;
  switch ( v1 )
  {
    case 1:
    case 3:
      return 0;
    case 2:
    case 6:
      *(long long *)(a1 + 32) = &unk_9A7C0;
      goto LABEL_10;
    case 4:
      *(long long *)(a1 + 32) = &unk_9A880;
      goto LABEL_10;
    case 5:
      *(long long *)(a1 + 32) = &unk_9A820;
LABEL_10:
      result = 1;
      break;
    default:
      result = 0;
      break;
  }
  return result;
}



// Function: sparse_dump_region @ 0x2faf0
long long sparse_dump_region(unsigned int *a1, long long a2)
{
  long long v3; // rax
  long long v4; // rdi
  long long v5; // r12
  long long v6; // rbx
  char *v7; // rax
  long long v9; // [rsp+18h] [rbp-E8h]
  __off_t n; // [rsp+20h] [rbp-E0h]
  long long v11; // [rsp+28h] [rbp-D8h]
  char *v12; // [rsp+30h] [rbp-D0h]
  long long v13; // [rsp+38h] [rbp-C8h]
  struct stat buf; // [rsp+40h] [rbp-C0h] BYREF
  char v15[24]; // [rsp+D0h] [rbp-30h] BYREF
  unsigned long long v16; // [rsp+E8h] [rbp-18h]

  v16 = __readfsqword(0x28u);
  v9 = *(long long *)(*(long long *)(*((long long *)a1 + 3) + 320LL) + 16 * a2 + 8);
  if ( (unsigned char)sub_2F3B2((long long)a1, *(long long *)(*(long long *)(*((long long *)a1 + 3) + 320LL) + 16 * a2)) != 1 )
    return 0;
  while ( 1 )
  {
    if ( v9 <= 0 )
      return 1;
    v3 = v9;
    if ( v9 > 512 )
      v3 = 512;
    v11 = v3;
    v12 = (char *)sub_BB4B();
    v13 = sub_5EC70(*a1, v12, v11);
    if ( v13 == -1 )
    {
      sub_2B2F8(
        **((long long **)a1 + 3),
        *(long long *)(*(long long *)(*((long long *)a1 + 3) + 320LL) + 16 * a2)
      + *(long long *)(*(long long *)(*((long long *)a1 + 3) + 320LL) + 16 * a2 + 8)
      - v9,
        v11);
      return 0;
    }
    if ( !v13 )
      break;
    memset(&v12[v13], 0, 512 - v13);
    v9 -= v13;
    *((long long *)a1 + 2) += v13;
    sub_BBA7((unsigned long long)v12);
  }
  v4 = *a1;
  if ( fstat(v4, &buf) )
    n = *(long long *)(*((long long *)a1 + 3) + 136LL)
      - (*(long long *)(*(long long *)(*((long long *)a1 + 3) + 320LL) + 16 * a2)
       + *(long long *)(*(long long *)(*((long long *)a1 + 3) + 320LL) + 16 * a2 + 8)
       - v9);
  else
    n = *(long long *)(*((long long *)a1 + 3) + 136LL) - buf.st_size;
  if ( (dword_9EC84 & 0x80) != 0 )
  {
    if ( qword_9FDC8 )
      qword_9FDC8(v4);
    v5 = sub_54E5A(n, v15);
    v6 = sub_5E548(**((long long **)a1 + 3));
    v7 = ngettext(
           "%s: File shrank by %s byte; padding with zeros",
           "%s: File shrank by %s bytes; padding with zeros",
           n);
    error(0, 0, v7, v6, v5);
  }
  if ( byte_9F8BC != 1 )
    sub_39E98(1);
  return 0;
}



// Function: sparse_extract_region @ 0x2fe94
long long sparse_extract_region(unsigned int *a1, long long a2)
{
  long long v3; // rax
  char *v4; // rax
  unsigned int *v5; // [rsp+8h] [rbp-28h]
  long long v6; // [rsp+10h] [rbp-20h]
  unsigned long long v7; // [rsp+18h] [rbp-18h]
  void *v8; // [rsp+20h] [rbp-10h]
  unsigned long long v9; // [rsp+28h] [rbp-8h]

  v5 = a1;
  if ( (unsigned char)sub_2F3B2((long long)a1, *(long long *)(*(long long *)(*((long long *)a1 + 3) + 320LL) + 16 * a2)) != 1 )
    return 0;
  v6 = *(long long *)(*(long long *)(*((long long *)a1 + 3) + 320LL) + 16 * a2 + 8);
  if ( v6 )
  {
    while ( v6 > 0 )
    {
      v3 = v6;
      if ( v6 > 512 )
        v3 = 512;
      v7 = v3;
      v8 = sub_BB4B();
      if ( !v8 )
      {
        if ( qword_9FDC8 )
          qword_9FDC8(a1);
        v4 = gettext("Unexpected EOF in archive");
        error(0, 0, v4);
        dword_9FDD0 = 2;
        return 0;
      }
      sub_BBA7((unsigned long long)v8);
      *((long long *)v5 + 2) += 512LL;
      v9 = sub_2AA95(*v5, (long long)v8, v7);
      v6 -= v9;
      a1 = (unsigned int *)(*(long long *)(*((long long *)v5 + 3) + 280LL) - *((long long *)v5 + 2));
      sub_AD86((long long)a1);
      *((long long *)v5 + 1) += v9;
      if ( v9 != v7 )
      {
        sub_3EB4B(**((long long **)v5 + 3), v9, v7);
        return 0;
      }
    }
  }
  else if ( *((char *)a1 + 4) )
  {
    if ( (unsigned int)sub_32F52(*a1) )
      sub_3EA7E(**((long long **)a1 + 3));
  }
  return 1;
}



// Function: sparse_dump_file @ 0x30090
long long sparse_dump_file(int a1, long long *a2)
{
  char v3; // [rsp+17h] [rbp-49h]
  unsigned long long i; // [rsp+18h] [rbp-48h]
  long long (**v5[2])(long long); // [rsp+20h] [rbp-40h] BYREF
  long long v6; // [rsp+30h] [rbp-30h]
  long long *v7; // [rsp+38h] [rbp-28h]
  long long v8; // [rsp+40h] [rbp-20h]
  unsigned long long v9; // [rsp+58h] [rbp-8h]

  v9 = __readfsqword(0x28u);
  if ( (unsigned char)sub_2F165(v5) != 1 )
    return 3;
  v7 = a2;
  LODWORD(v5[0]) = a1;
  BYTE4(v5[0]) = 1;
  v3 = sub_2F9E5((long long)v5);
  if ( v3 )
  {
    if ( *(long long *)(v8 + 56) )
    {
      sub_2F2F5((long long)v5);
      if ( a1 >= 0 )
      {
        sub_AB09(v7[1], v7[17], v7[35] - v6);
        for ( i = 0; v3 && i < v7[38]; ++i )
          v3 = sub_2F261((long long)v5, i);
      }
    }
  }
  sub_142D7(v7[35] - v6);
  return !(unsigned char)sub_2F1D2((long long)v5) || !v3;
}



// Function: sparse_member_p @ 0x301de
long long sparse_member_p(long long (**a1)(long long))
{
  long long (**v2[8])(long long); // [rsp+10h] [rbp-40h] BYREF

  v2[7] = (long long (**)(long long))__readfsqword(0x28u);
  if ( (unsigned char)sub_2F165(v2) != 1 )
    return 0;
  v2[3] = a1;
  return sub_2F126((long long)v2);
}



// Function: sparse_fixup_header @ 0x30241
long long sparse_fixup_header(long long (**a1)(long long))
{
  long long (**v2[8])(long long); // [rsp+10h] [rbp-40h] BYREF

  v2[7] = (long long (**)(long long))__readfsqword(0x28u);
  if ( (unsigned char)sub_2F165(v2) != 1 )
    return 0;
  v2[3] = a1;
  return sub_2F373((long long)v2);
}



// Function: sparse_extract_file @ 0x302a4
long long sparse_extract_file(int a1, long long a2, long long *a3)
{
  char v5; // [rsp+27h] [rbp-49h]
  unsigned long long i; // [rsp+28h] [rbp-48h]
  long long (**v7[3])(long long); // [rsp+30h] [rbp-40h] BYREF
  long long v8; // [rsp+48h] [rbp-28h]
  unsigned long long v9; // [rsp+68h] [rbp-8h]

  v9 = __readfsqword(0x28u);
  if ( (unsigned char)sub_2F165(v7) != 1 )
    return 3;
  v8 = a2;
  LODWORD(v7[0]) = a1;
  BYTE4(v7[0]) = lseek(a1, 0, 0) == 0;
  v7[1] = 0;
  v5 = sub_2F334((long long)v7);
  for ( i = 0; v5 && i < *(long long *)(v8 + 304); ++i )
    v5 = sub_2F2AB((long long)v7, i);
  *a3 = *(long long *)(v8 + 280) - (unsigned long long)v7[2];
  return !(unsigned char)sub_2F1D2((long long)v7) || !v5;
}



// Function: sparse_skip_file @ 0x303bd
long long sparse_skip_file(long long a1)
{
  char v2; // [rsp+1Fh] [rbp-41h]
  long long (**v3[3])(long long); // [rsp+20h] [rbp-40h] BYREF
  long long v4; // [rsp+38h] [rbp-28h]
  unsigned long long v5; // [rsp+58h] [rbp-8h]

  v5 = __readfsqword(0x28u);
  if ( (unsigned char)sub_2F165(v3) != 1 )
    return 3;
  v4 = a1;
  LODWORD(v3[0]) = -1;
  v2 = sub_2F334((long long)v3);
  sub_287C1(*(long long *)(v4 + 280) - (unsigned long long)v3[2]);
  return !(unsigned char)sub_2F1D2((long long)v3) || !v2;
}



// Function: check_sparse_region @ 0x3046a
long long check_sparse_region(unsigned int *a1, __off_t a2, long long a3)
{
  long long v4; // rax
  char *v5; // rax
  long long v6; // rcx
  long long v7; // r8
  long long v8; // r9
  long long v9; // rbx
  char *v10; // rax
  long long v11; // r8
  long long v12; // r9
  long long v14; // [rsp+10h] [rbp-260h]
  long long v15; // [rsp+20h] [rbp-250h]
  long long v16; // [rsp+28h] [rbp-248h]
  char v17[32]; // [rsp+30h] [rbp-240h] BYREF
  char v18[520]; // [rsp+50h] [rbp-220h] BYREF
  unsigned long long v19; // [rsp+258h] [rbp-18h]

  v14 = a2;
  v19 = __readfsqword(0x28u);
  if ( (unsigned char)sub_2F3B2((long long)a1, a2) != 1 )
    return 0;
  while ( v14 < a3 )
  {
    v4 = a3 - v14;
    if ( a3 - v14 > 512 )
      v4 = 512;
    v15 = v4;
    v16 = sub_5EC70(*a1, v18, v4);
    if ( v16 == -1 )
    {
      sub_2B2F8(**((long long **)a1 + 3), v14, v15);
      return 0;
    }
    if ( !v16 )
    {
      v5 = gettext("Size differs");
      sub_107BE(*((long long *)a1 + 3), v5, (long long)v5, v6, v7, v8);
      return 0;
    }
    if ( (unsigned char)sub_2F433(v18, v16) != 1 )
    {
      v9 = sub_54D1C(v14, v17);
      v10 = gettext("File fragment at %s is not a hole");
      sub_107BE(*((long long *)a1 + 3), v10, v9, (long long)v10, v11, v12);
      return 0;
    }
    v14 += v16;
  }
  return 1;
}



// Function: check_data_region @ 0x3064f
long long check_data_region(unsigned int *a1, long long a2)
{
  long long v3; // rdi
  long long v4; // rax
  char *v5; // rax
  char *v6; // rax
  long long v7; // rdx
  long long v8; // rcx
  long long v9; // r8
  long long v10; // r9
  char *v11; // rax
  long long v12; // rcx
  long long v13; // r8
  long long v14; // r9
  long long v16; // [rsp+10h] [rbp-230h]
  long long v17; // [rsp+18h] [rbp-228h]
  const void *s1; // [rsp+20h] [rbp-220h]
  long long n; // [rsp+28h] [rbp-218h]
  char s2[520]; // [rsp+30h] [rbp-210h] BYREF
  unsigned long long v21; // [rsp+238h] [rbp-8h]

  v21 = __readfsqword(0x28u);
  if ( (unsigned char)sub_2F3B2((long long)a1, *(long long *)(*(long long *)(*((long long *)a1 + 3) + 320LL) + 16 * a2)) != 1 )
    return 0;
  v16 = *(long long *)(*(long long *)(*((long long *)a1 + 3) + 320LL) + 16 * a2 + 8);
  v3 = *(long long *)(*((long long *)a1 + 3) + 280LL) - *((long long *)a1 + 2);
  sub_AD86(v3);
  while ( 1 )
  {
    if ( v16 <= 0 )
      return 1;
    v4 = v16;
    if ( v16 > 512 )
      v4 = 512;
    v17 = v4;
    s1 = sub_BB4B();
    if ( !s1 )
      break;
    sub_BBA7((unsigned long long)s1);
    *((long long *)a1 + 2) += 512LL;
    n = sub_5EC70(*a1, s2, v17);
    if ( n == -1 )
    {
      sub_2B2F8(
        **((long long **)a1 + 3),
        *(long long *)(*(long long *)(*((long long *)a1 + 3) + 320LL) + 16 * a2)
      + *(long long *)(*(long long *)(*((long long *)a1 + 3) + 320LL) + 16 * a2 + 8)
      - v16,
        v17);
      return 0;
    }
    if ( !n )
    {
      v6 = gettext("Size differs");
      sub_107BE((long long)&qword_9FA20, v6, v7, v8, v9, v10);
      return 0;
    }
    v16 -= n;
    sub_AD86(*(long long *)(*((long long *)a1 + 3) + 280LL) - *((long long *)a1 + 2));
    v3 = (long long)s1;
    if ( memcmp(s1, s2, n) )
    {
      v11 = gettext("Contents differ");
      sub_107BE(*((long long *)a1 + 3), v11, (long long)v11, v12, v13, v14);
      return 0;
    }
  }
  if ( qword_9FDC8 )
    qword_9FDC8(v3);
  v5 = gettext("Unexpected EOF in archive");
  error(0, 0, v5);
  dword_9FDD0 = 2;
  return 0;
}



// Function: sparse_diff_file @ 0x30972
long long sparse_diff_file(int a1, long long *a2)
{
  bool v3; // al
  unsigned char v4; // [rsp+1Fh] [rbp-51h]
  unsigned long long i; // [rsp+20h] [rbp-50h]
  __off_t v6; // [rsp+28h] [rbp-48h]
  long long (**v7[3])(long long); // [rsp+30h] [rbp-40h] BYREF
  long long *v8; // [rsp+48h] [rbp-28h]
  unsigned long long v9; // [rsp+68h] [rbp-8h]

  v9 = __readfsqword(0x28u);
  v6 = 0;
  if ( (unsigned char)sub_2F165(v7) != 1 )
    return 0;
  v8 = a2;
  LODWORD(v7[0]) = a1;
  BYTE4(v7[0]) = 1;
  v4 = sub_2F334((long long)v7);
  sub_AD2B(a2);
  for ( i = 0; v4 && i < v8[38]; ++i )
  {
    v3 = (unsigned char)sub_3046A((unsigned int *)v7, v6, *(long long *)(v8[40] + 16 * i))
      && (unsigned char)sub_3064F((unsigned int *)v7, i);
    v4 = v3;
    v6 = *(long long *)(v8[40] + 16 * i) + *(long long *)(v8[40] + 16 * i + 8);
  }
  if ( v4 != 1 )
    sub_287C1(v8[35] - (unsigned long long)v7[2]);
  sub_AD66();
  sub_2F1D2((long long)v7);
  return v4;
}



// Function: oldgnu_sparse_member_p @ 0x30afb
bool oldgnu_sparse_member_p()
{
  return *(char *)(qword_9F460 + 156) == 83;
}



// Function: oldgnu_add_sparse @ 0x30b1c
long long oldgnu_add_sparse(long long a1, char *a2)
{
  long long v3; // [rsp+10h] [rbp-20h] BYREF
  long long v4; // [rsp+18h] [rbp-18h]
  unsigned long long v5; // [rsp+28h] [rbp-8h]

  v5 = __readfsqword(0x28u);
  if ( !a2[12] )
    return 1;
  v3 = sub_2798B(a2, 0xCu);
  v4 = sub_2798B(a2 + 12, 0xCu);
  if ( v3 < 0
    || v4 < 0
    || __OFADD__(v3, v4)
    || *(long long *)(*(long long *)(a1 + 24) + 136LL) < v3 + v4
    || *(long long *)(*(long long *)(a1 + 24) + 280LL) < 0 )
  {
    return 2;
  }
  sub_2F477(*(long long **)(a1 + 24), &v3);
  return 0;
}



// Function: oldgnu_fixup_header @ 0x30c22
long long oldgnu_fixup_header(long long a1)
{
  long long v1; // rax
  long long v3; // [rsp+18h] [rbp-8h]

  v1 = sub_2798B((char *)(qword_9F460 + 483), 0xCu);
  v3 = v1;
  *(long long *)(*(long long *)(a1 + 24) + 280LL) = *(long long *)(*(long long *)(a1 + 24) + 136LL);
  if ( v1 < 0 )
    v1 = 0;
  *(long long *)(*(long long *)(a1 + 24) + 136LL) = v1;
  return v3 >= 0;
}



// Function: oldgnu_get_sparse_info @ 0x30c9a
long long oldgnu_get_sparse_info(long long *a1)
{
  char *v1; // rax
  long long v3; // rbx
  char *v4; // rax
  long long v5; // [rsp+8h] [rbp-38h]
  int j; // [rsp+18h] [rbp-28h]
  int v7; // [rsp+1Ch] [rbp-24h]
  unsigned long long i; // [rsp+20h] [rbp-20h]
  unsigned long long k; // [rsp+20h] [rbp-20h]
  long long v10; // [rsp+28h] [rbp-18h]
  char *v11; // [rsp+28h] [rbp-18h]

  v5 = (long long)a1;
  v10 = qword_9F460;
  *(long long *)(a1[3] + 304LL) = 0;
  for ( i = 0; i <= 3; ++i )
  {
    a1 = (long long *)v5;
    v7 = sub_30B1C(v5, (char *)(24 * i + 384 + v10 + 2));
    if ( v7 )
      break;
  }
  for ( j = *(char *)(v10 + 482); ; j = (char)v11[504] )
  {
    if ( v7 || !j )
    {
      if ( v7 != 2 )
        return 1;
      if ( qword_9FDC8 )
        qword_9FDC8(a1);
      v3 = **(long long **)(v5 + 24);
      v4 = gettext("%s: invalid sparse archive member");
      error(0, 0, v4, v3);
      dword_9FDD0 = 2;
      return 0;
    }
    v11 = sub_BB4B();
    if ( !v11 )
      break;
    a1 = v11;
    sub_BBA7((unsigned long long)v11);
    for ( k = 0; k <= 0x14 && !v7; ++k )
    {
      a1 = (long long *)v5;
      v7 = sub_30B1C(v5, &v11[24 * k]);
    }
  }
  if ( qword_9FDC8 )
    qword_9FDC8(a1);
  v1 = gettext("Unexpected EOF in archive");
  error(0, 0, v1);
  dword_9FDD0 = 2;
  return 0;
}



// Function: oldgnu_store_sparse_info @ 0x30e70
unsigned long long oldgnu_store_sparse_info(long long a1, unsigned long long *a2, char *a3, long long a4)
{
  unsigned long long result; // rax

  while ( 1 )
  {
    result = *(long long *)(*(long long *)(a1 + 24) + 304LL);
    if ( *a2 >= result || !a4 )
      break;
    sub_12A9A(*(long long *)(*(long long *)(*(long long *)(a1 + 24) + 320LL) + 16 * *a2), a3, 12);
    sub_12A9A(*(long long *)(*(long long *)(*(long long *)(a1 + 24) + 320LL) + 16 * *a2 + 8), a3 + 12, 12);
    --a4;
    a3 += 24;
    ++*a2;
  }
  return result;
}



// Function: oldgnu_dump_header @ 0x30f43
long long oldgnu_dump_header(long long a1)
{
  unsigned long long v2; // [rsp+10h] [rbp-20h] BYREF
  long long v3; // [rsp+18h] [rbp-18h]
  void *s; // [rsp+20h] [rbp-10h]
  unsigned long long v5; // [rsp+28h] [rbp-8h]

  v5 = __readfsqword(0x28u);
  v3 = sub_BACD();
  s = sub_1381D(*(long long *)(a1 + 24));
  *((char *)s + 156) = 83;
  if ( *(long long *)(*(long long *)(a1 + 24) + 304LL) > 4u )
    *((char *)s + 482) = 1;
  sub_12A9A(*(long long *)(*(long long *)(a1 + 24) + 136LL), (char *)s + 483, 12);
  sub_12A9A(*(long long *)(*(long long *)(a1 + 24) + 280LL), (char *)s + 124, 12);
  v2 = 0;
  sub_30E70(a1, &v2, (char *)s + 386, 4);
  *((char *)s + 482) = *(long long *)(*(long long *)(a1 + 24) + 304LL) > v2;
  sub_1422E(*(long long **)(a1 + 24), s, v3);
  while ( *(long long *)(*(long long *)(a1 + 24) + 304LL) > v2 )
  {
    s = sub_BB4B();
    memset(s, 0, 0x200u);
    sub_30E70(a1, &v2, (char *)s, 21);
    if ( *(long long *)(*(long long *)(a1 + 24) + 304LL) > v2 )
      *((char *)s + 504) = 1;
    sub_BBA7((unsigned long long)s);
  }
  return 1;
}



// Function: star_sparse_member_p @ 0x310ff
bool star_sparse_member_p()
{
  return *(char *)(qword_9F460 + 156) == 83;
}



// Function: star_fixup_header @ 0x31120
long long star_fixup_header(long long a1)
{
  long long v1; // rax
  long long v3; // [rsp+18h] [rbp-8h]

  v1 = sub_2798B((char *)(qword_9F460 + 452), 0xCu);
  v3 = v1;
  *(long long *)(*(long long *)(a1 + 24) + 280LL) = *(long long *)(*(long long *)(a1 + 24) + 136LL);
  if ( v1 < 0 )
    v1 = 0;
  *(long long *)(*(long long *)(a1 + 24) + 136LL) = v1;
  return v3 >= 0;
}



// Function: star_get_sparse_info @ 0x31198
long long star_get_sparse_info(long long *a1)
{
  char *v1; // rax
  long long v3; // rbx
  char *v4; // rax
  long long v5; // [rsp+8h] [rbp-38h]
  int v6; // [rsp+18h] [rbp-28h]
  int v7; // [rsp+1Ch] [rbp-24h]
  unsigned long long i; // [rsp+20h] [rbp-20h]
  unsigned long long j; // [rsp+20h] [rbp-20h]
  char *v10; // [rsp+28h] [rbp-18h]
  char *v11; // [rsp+28h] [rbp-18h]

  v5 = (long long)a1;
  v10 = (char *)qword_9F460;
  v7 = 0;
  *(long long *)(a1[3] + 304LL) = 0;
  if ( v10[345] || !v10[366] )
  {
    v6 = 1;
  }
  else
  {
    for ( i = 0; i <= 3; ++i )
    {
      a1 = (long long *)v5;
      v7 = sub_30B1C(v5, &v10[24 * i + 356]);
      if ( v7 )
        break;
    }
    v6 = (char)v10[355];
  }
  while ( 1 )
  {
    if ( v7 || !v6 )
    {
      if ( v7 != 2 )
        return 1;
      if ( qword_9FDC8 )
        qword_9FDC8(a1);
      v3 = **(long long **)(v5 + 24);
      v4 = gettext("%s: invalid sparse archive member");
      error(0, 0, v4, v3);
      dword_9FDD0 = 2;
      return 0;
    }
    v11 = sub_BB4B();
    if ( !v11 )
      break;
    a1 = v11;
    sub_BBA7((unsigned long long)v11);
    for ( j = 0; j <= 0x14 && !v7; ++j )
    {
      a1 = (long long *)v5;
      v7 = sub_30B1C(v5, &v11[24 * j]);
    }
    *(long long *)(v5 + 16) += 512LL;
    v6 = (char)v11[504];
  }
  if ( qword_9FDC8 )
    qword_9FDC8(a1);
  v1 = gettext("Unexpected EOF in archive");
  error(0, 0, v1);
  dword_9FDD0 = 2;
  return 0;
}



// Function: pax_sparse_member_p @ 0x313b3
long long pax_sparse_member_p(long long a1)
{
  return *(long long *)(*(long long *)(a1 + 24) + 304LL) || *(int *)(*(long long *)(a1 + 24) + 292LL);
}



// Function: pax_start_header @ 0x313f6
char *pax_start_header(long long a1)
{
  char *result; // rax
  long long v2; // [rsp+10h] [rbp-10h]

  v2 = *(long long *)(a1 + 136);
  *(long long *)(a1 + 136) = *(long long *)(a1 + 280);
  result = sub_1381D(a1);
  *(long long *)(a1 + 136) = v2;
  return result;
}



// Function: pax_dump_header_0 @ 0x31450
long long pax_dump_header_0(long long a1)
{
  long long *v1; // rbx
  const char *v2; // rax
  const char *v3; // rax
  unsigned long long i; // [rsp+18h] [rbp-58h] BYREF
  long long v6; // [rsp+20h] [rbp-50h]
  long long v7; // [rsp+28h] [rbp-48h]
  long long v8; // [rsp+30h] [rbp-40h]
  char *v9; // [rsp+38h] [rbp-38h]
  char v10[24]; // [rsp+40h] [rbp-30h] BYREF
  unsigned long long v11; // [rsp+58h] [rbp-18h]

  v11 = __readfsqword(0x28u);
  v7 = sub_BACD();
  v8 = *(long long *)(*(long long *)(a1 + 24) + 320LL);
  v6 = 0;
  sub_1E1D3("GNU.sparse.size", *(long long *)(a1 + 24), 0);
  sub_1E1D3("GNU.sparse.numblocks", *(long long *)(a1 + 24), 0);
  if ( !(unsigned char)sub_1C63F("GNU.sparse.map") && dword_9F980 )
  {
    sub_1E1D3("GNU.sparse.name", *(long long *)(a1 + 24), 0);
    v6 = *(long long *)(*(long long *)(a1 + 24) + 8LL);
    v1 = *(long long **)(a1 + 24);
    v1[1] = sub_1CD02(v1, "%d/GNUSparseFile.%p/%f", 0);
    sub_1E848(*(long long *)(a1 + 24) + 360LL);
    for ( i = 0; *(long long *)(*(long long *)(a1 + 24) + 304LL) > i; ++i )
    {
      if ( i )
        sub_1E863((long long *)(*(long long *)(a1 + 24) + 360LL), ",");
      v2 = (const char *)sub_54E5A(*(long long *)(16 * i + v8), v10);
      sub_1E863((long long *)(*(long long *)(a1 + 24) + 360LL), v2);
      sub_1E863((long long *)(*(long long *)(a1 + 24) + 360LL), ",");
      v3 = (const char *)sub_54E5A(*(long long *)(16 * i + v8 + 8), v10);
      sub_1E863((long long *)(*(long long *)(a1 + 24) + 360LL), v3);
    }
    if ( (unsigned char)sub_1E8DC((long long *)(*(long long *)(a1 + 24) + 360LL), "GNU.sparse.map") != 1 )
    {
      free(*(void **)(*(long long *)(a1 + 24) + 8LL));
      *(long long *)(*(long long *)(a1 + 24) + 8LL) = v6;
      return 0;
    }
  }
  else
  {
    for ( i = 0; *(long long *)(*(long long *)(a1 + 24) + 304LL) > i; ++i )
    {
      sub_1E1D3("GNU.sparse.offset", *(long long *)(a1 + 24), (long long)&i);
      sub_1E1D3("GNU.sparse.numbytes", *(long long *)(a1 + 24), (long long)&i);
    }
  }
  v9 = sub_313F6(*(long long *)(a1 + 24));
  sub_1422E(*(long long **)(a1 + 24), v9, v7);
  if ( v6 )
  {
    free(*(void **)(*(long long *)(a1 + 24) + 8LL));
    *(long long *)(*(long long *)(a1 + 24) + 8LL) = v6;
  }
  return 1;
}



// Function: pax_dump_header_1 @ 0x3179e
long long pax_dump_header_1(long long a1)
{
  const char *v1; // rax
  long long v2; // rax
  long long *v3; // rbx
  char *v4; // rax
  char *v5; // rdx
  char *v6; // rax
  const char *v7; // rdx
  char *v8; // rax
  char *v9; // rdx
  char *v10; // rax
  const char *v11; // rdx
  char *v12; // rax
  char *v13; // rdx
  char *v14; // rax
  const char *v15; // rdx
  char *v16; // rax
  char *v18; // [rsp+10h] [rbp-D0h]
  char *v19; // [rsp+18h] [rbp-C8h]
  unsigned long long i; // [rsp+20h] [rbp-C0h]
  unsigned long long j; // [rsp+20h] [rbp-C0h]
  size_t v22; // [rsp+28h] [rbp-B8h]
  size_t v23; // [rsp+28h] [rbp-B8h]
  char *v24; // [rsp+30h] [rbp-B0h]
  char *v25; // [rsp+38h] [rbp-A8h]
  char *v26; // [rsp+40h] [rbp-A0h]
  const char *v27; // [rsp+48h] [rbp-98h]
  char *v28; // [rsp+50h] [rbp-90h]
  char *v29; // [rsp+58h] [rbp-88h]
  char *v30; // [rsp+60h] [rbp-80h]
  const char *v31; // [rsp+68h] [rbp-78h]
  char *v32; // [rsp+70h] [rbp-70h]
  char *v33; // [rsp+78h] [rbp-68h]
  char *v34; // [rsp+80h] [rbp-60h]
  const char *v35; // [rsp+88h] [rbp-58h]
  signed long long v36; // [rsp+90h] [rbp-50h]
  long long v37; // [rsp+98h] [rbp-48h]
  long long v38; // [rsp+A0h] [rbp-40h]
  char *s; // [rsp+A8h] [rbp-38h]
  char *sa; // [rsp+A8h] [rbp-38h]
  char v41[24]; // [rsp+B0h] [rbp-30h] BYREF
  unsigned long long v42; // [rsp+C8h] [rbp-18h]

  v42 = __readfsqword(0x28u);
  v36 = sub_BACD();
  v37 = *(long long *)(*(long long *)(a1 + 24) + 320LL);
  v38 = *(long long *)(*(long long *)(a1 + 24) + 8LL);
  s = (char *)sub_54E5A(*(long long *)(*(long long *)(a1 + 24) + 304LL), v41);
  v22 = strlen(s) + 1;
  for ( i = 0; i < *(long long *)(*(long long *)(a1 + 24) + 304LL); ++i )
  {
    v1 = (const char *)sub_54E5A(*(long long *)(16 * i + v37), v41);
    v23 = v22 + strlen(v1) + 1;
    sa = (char *)sub_54E5A(*(long long *)(16 * i + v37 + 8), v41);
    v22 = v23 + strlen(sa) + 1;
  }
  v2 = v22 + 511;
  if ( (long long)(v22 + 511) < 0 )
    v2 = v22 + 1022;
  *(long long *)(*(long long *)(a1 + 24) + 280LL) += v2 >> 9 << 9;
  *(long long *)(a1 + 16) += v2 >> 9 << 9;
  sub_1E1D3("GNU.sparse.major", *(long long *)(a1 + 24), 0);
  sub_1E1D3("GNU.sparse.minor", *(long long *)(a1 + 24), 0);
  sub_1E1D3("GNU.sparse.name", *(long long *)(a1 + 24), 0);
  sub_1E1D3("GNU.sparse.realsize", *(long long *)(a1 + 24), 0);
  v3 = *(long long **)(a1 + 24);
  v3[1] = sub_1CD02(v3, "%d/GNUSparseFile.%p/%f", 0);
  if ( strlen(*(const char **)(*(long long *)(a1 + 24) + 8LL)) > 0x64 )
    *(char *)(*(long long *)(*(long long *)(a1 + 24) + 8LL) + 100LL) = 0;
  v4 = sub_313F6(*(long long *)(a1 + 24));
  sub_1422E(*(long long **)(a1 + 24), v4, v36);
  free(*(void **)(*(long long *)(a1 + 24) + 8LL));
  *(long long *)(*(long long *)(a1 + 24) + 8LL) = v38;
  v18 = (char *)sub_BB4B();
  v19 = v18;
  v24 = v18 + 512;
  v25 = (char *)sub_54E5A(*(long long *)(*(long long *)(a1 + 24) + 304LL), v41);
  while ( *v25 )
  {
    if ( v19 == v24 )
    {
      sub_BBA7((unsigned long long)v18);
      v18 = (char *)sub_BB4B();
      v19 = v18;
      v24 = v18 + 512;
    }
    v5 = v25++;
    v6 = v19++;
    *v6 = *v5;
  }
  v26 = v18 + 512;
  v27 = "\n";
  while ( *v27 )
  {
    if ( v19 == v26 )
    {
      sub_BBA7((unsigned long long)v18);
      v18 = (char *)sub_BB4B();
      v19 = v18;
      v26 = v18 + 512;
    }
    v7 = v27++;
    v8 = v19++;
    *v8 = *v7;
  }
  for ( j = 0; j < *(long long *)(*(long long *)(a1 + 24) + 304LL); ++j )
  {
    v28 = v18 + 512;
    v29 = (char *)sub_54E5A(*(long long *)(16 * j + v37), v41);
    while ( *v29 )
    {
      if ( v19 == v28 )
      {
        sub_BBA7((unsigned long long)v18);
        v18 = (char *)sub_BB4B();
        v19 = v18;
        v28 = v18 + 512;
      }
      v9 = v29++;
      v10 = v19++;
      *v10 = *v9;
    }
    v30 = v18 + 512;
    v31 = "\n";
    while ( *v31 )
    {
      if ( v19 == v30 )
      {
        sub_BBA7((unsigned long long)v18);
        v18 = (char *)sub_BB4B();
        v19 = v18;
        v30 = v18 + 512;
      }
      v11 = v31++;
      v12 = v19++;
      *v12 = *v11;
    }
    v32 = v18 + 512;
    v33 = (char *)sub_54E5A(*(long long *)(16 * j + v37 + 8), v41);
    while ( *v33 )
    {
      if ( v19 == v32 )
      {
        sub_BBA7((unsigned long long)v18);
        v18 = (char *)sub_BB4B();
        v19 = v18;
        v32 = v18 + 512;
      }
      v13 = v33++;
      v14 = v19++;
      *v14 = *v13;
    }
    v34 = v18 + 512;
    v35 = "\n";
    while ( *v35 )
    {
      if ( v19 == v34 )
      {
        sub_BBA7((unsigned long long)v18);
        v18 = (char *)sub_BB4B();
        v19 = v18;
        v34 = v18 + 512;
      }
      v15 = v35++;
      v16 = v19++;
      *v16 = *v15;
    }
  }
  memset(v19, 0, 512 - (v19 - v18));
  sub_BBA7((unsigned long long)v18);
  return 1;
}



// Function: pax_dump_header @ 0x31fd5
bool pax_dump_header(long long a1)
{
  *(int *)(*(long long *)(a1 + 24) + 292LL) = dword_9F97C;
  *(int *)(*(long long *)(a1 + 24) + 296LL) = dword_9F980;
  if ( *(int *)(*(long long *)(a1 + 24) + 292LL) )
    return (unsigned char)sub_3179E(a1) != 0;
  else
    return (unsigned char)sub_31450(a1) != 0;
}



// Function: decode_num @ 0x3204b
long long decode_num(uintmax_t *a1, const char *a2, uintmax_t a3)
{
  char *endptr; // [rsp+28h] [rbp-18h] BYREF
  uintmax_t v6; // [rsp+30h] [rbp-10h]
  unsigned long long v7; // [rsp+38h] [rbp-8h]

  v7 = __readfsqword(0x28u);
  if ( (unsigned int)(*a2 - 48) > 9 )
    return 0;
  *__errno_location() = 0;
  v6 = strtoumax(a2, &endptr, 10);
  if ( v6 > a3 || *__errno_location() == 34 || *endptr )
    return 0;
  *a1 = v6;
  return 1;
}



// Function: pax_decode_header @ 0x32100
long long pax_decode_header(long long a1)
{
  long long v1; // rdi
  char *v2; // rax
  long long v3; // rbx
  char *v4; // rax
  char *v6; // rax
  char *v7; // rax
  char *v8; // rax
  long long v9; // rcx
  long long v10; // r8
  long long v11; // r9
  long long v12; // rbx
  long long *v13; // rdi
  long long v14; // rbx
  char *v15; // rax
  char *v16; // rax
  char *v17; // rax
  char *v18; // rax
  uintmax_t *v19; // rdi
  char *v20; // rax
  char *v21; // rax
  char *v22; // rax
  long long v23; // rbx
  char *v24; // rax
  long long v25; // [rsp+0h] [rbp-B0h]
  long long v26; // [rsp+8h] [rbp-A8h]
  uintmax_t v27; // [rsp+18h] [rbp-98h] BYREF
  char *v28; // [rsp+20h] [rbp-90h]
  char *v29; // [rsp+28h] [rbp-88h]
  unsigned long long v30; // [rsp+30h] [rbp-80h]
  char *v31; // [rsp+38h] [rbp-78h]
  char *v32; // [rsp+40h] [rbp-70h]
  char *v33; // [rsp+48h] [rbp-68h]
  char *v34; // [rsp+50h] [rbp-60h]
  char *v35; // [rsp+58h] [rbp-58h]
  char *v36; // [rsp+60h] [rbp-50h]
  signed long long v37; // [rsp+68h] [rbp-48h]
  long long v38[2]; // [rsp+70h] [rbp-40h] BYREF
  char v39[20]; // [rsp+80h] [rbp-30h] BYREF
  char v40[20]; // [rsp+94h] [rbp-1Ch] BYREF

  v26 = a1;
  *(long long *)&v40[4] = __readfsqword(0x28u);
  if ( *(int *)(*(long long *)(a1 + 24) + 292LL) )
  {
    v37 = sub_BACD();
    v1 = qword_9F460;
    sub_BBA7(qword_9F460);
    v28 = (char *)sub_BB4B();
    if ( !v28 )
    {
      if ( qword_9FDC8 )
        qword_9FDC8(v1);
      v2 = gettext("Unexpected EOF in archive");
      error(0, 0, v2);
      sub_177F4();
    }
    v29 = v28;
    v31 = v28 + 512;
    v32 = v39;
    do
    {
      if ( v32 == v40 )
      {
        if ( qword_9FDC8 )
          qword_9FDC8(v1);
        v3 = **(long long **)(v26 + 24);
        v4 = gettext("%s: numeric overflow in sparse archive member");
        error(0, 0, v4, v3);
        dword_9FDD0 = 2;
        return 0;
      }
      if ( v29 == v31 )
      {
        v1 = (long long)v28;
        sub_BBA7((unsigned long long)v28);
        v28 = (char *)sub_BB4B();
        if ( !v28 )
        {
          if ( qword_9FDC8 )
            qword_9FDC8(v1);
          v6 = gettext("Unexpected EOF in archive");
          error(0, 0, v6);
          sub_177F4();
        }
        v29 = v28;
        v31 = v28 + 512;
      }
      v7 = v29++;
      *v32 = *v7;
      v8 = v32++;
    }
    while ( *v8 != 10 );
    *(v32 - 1) = 0;
    if ( (unsigned char)sub_3204B(&v27, v39, 0xFFFFFFFFFFFFFFFFLL) != 1 )
    {
      if ( qword_9FDC8 )
        qword_9FDC8(&v27);
LABEL_54:
      v23 = **(long long **)(v26 + 24);
      v24 = gettext("%s: malformed sparse archive member");
      error(0, 0, v24, v23);
      dword_9FDD0 = 2;
      return 0;
    }
    *(long long *)(*(long long *)(v26 + 24) + 312LL) = v27;
    v12 = *(long long *)(v26 + 24);
    v13 = *(long long **)(v12 + 312);
    *(long long *)(v12 + 320) = sub_60C29(v13, 16, v26, v9, v10, v11, v25, v26);
    *(long long *)(*(long long *)(v26 + 24) + 304LL) = 0;
    v30 = 0;
    while ( 2 )
    {
      if ( v30 < *(long long *)(*(long long *)(v26 + 24) + 312LL) )
      {
        v33 = v28 + 512;
        v34 = v39;
        do
        {
          if ( v34 == v40 )
          {
            if ( qword_9FDC8 )
              qword_9FDC8(v13);
LABEL_27:
            v14 = **(long long **)(v26 + 24);
            v15 = gettext("%s: numeric overflow in sparse archive member");
            error(0, 0, v15, v14);
            dword_9FDD0 = 2;
            return 0;
          }
          if ( v29 == v33 )
          {
            v13 = v28;
            sub_BBA7((unsigned long long)v28);
            v28 = (char *)sub_BB4B();
            if ( !v28 )
            {
              if ( qword_9FDC8 )
                qword_9FDC8(v13);
              v16 = gettext("Unexpected EOF in archive");
              error(0, 0, v16);
              sub_177F4();
            }
            v29 = v28;
            v33 = v28 + 512;
          }
          v17 = v29++;
          *v34 = *v17;
          v18 = v34++;
        }
        while ( *v18 != 10 );
        *(v34 - 1) = 0;
        v19 = &v27;
        if ( (unsigned char)sub_3204B(&v27, v39, 0x7FFFFFFFFFFFFFFFuLL) != 1 )
        {
          if ( qword_9FDC8 )
            qword_9FDC8(&v27);
        }
        else
        {
          v38[0] = v27;
          v35 = v28 + 512;
          v36 = v39;
          do
          {
            if ( v36 == v40 )
            {
              if ( qword_9FDC8 )
                qword_9FDC8(v19);
              goto LABEL_27;
            }
            if ( v29 == v35 )
            {
              v19 = (uintmax_t *)v28;
              sub_BBA7((unsigned long long)v28);
              v28 = (char *)sub_BB4B();
              if ( !v28 )
              {
                if ( qword_9FDC8 )
                  qword_9FDC8(v19);
                v20 = gettext("Unexpected EOF in archive");
                error(0, 0, v20);
                sub_177F4();
              }
              v29 = v28;
              v35 = v28 + 512;
            }
            v21 = v29++;
            *v36 = *v21;
            v22 = v36++;
          }
          while ( *v22 != 10 );
          *(v36 - 1) = 0;
          if ( (unsigned char)sub_3204B(&v27, v39, 0x7FFFFFFFFFFFFFFFuLL) == 1 )
          {
            v38[1] = v27;
            v13 = *(long long **)(v26 + 24);
            sub_2F477(v13, v38);
            ++v30;
            continue;
          }
          if ( qword_9FDC8 )
            qword_9FDC8(&v27);
        }
        goto LABEL_54;
      }
      break;
    }
    sub_BBA7((unsigned long long)v28);
    *(long long *)(v26 + 16) += (sub_BACD() - v37) << 9;
  }
  return 1;
}



// Function: find_compression_suffix @ 0x3286e
char **find_compression_suffix(const char *a1, long long *a2)
{
  char **i; // [rsp+18h] [rbp-18h]
  char *v4; // [rsp+20h] [rbp-10h]
  char *v5; // [rsp+20h] [rbp-10h]
  size_t n; // [rsp+28h] [rbp-8h]

  v4 = strrchr(a1, 46);
  if ( v4 )
  {
    v5 = v4 + 1;
    n = strlen(v5);
    for ( i = &off_9C960; *i; i += 3 )
    {
      if ( (char *)n == i[1] && !memcmp(*i, v5, n) )
      {
        if ( a2 )
          *a2 = strlen(a1) - n - 1;
        return i;
      }
    }
  }
  return 0;
}



// Function: find_compression_program @ 0x32930
char *find_compression_program(const char *a1, long long a2)
{
  char **v3; // [rsp+18h] [rbp-8h]

  v3 = sub_3286E(a1, 0);
  if ( v3 )
    return v3[2];
  else
    return (char *)a2;
}



// Function: set_compression_program_by_suffix @ 0x32970
char *set_compression_program_by_suffix(const char *a1, long long a2)
{
  char *result; // rax

  result = sub_32930(a1, a2);
  if ( result )
    qword_9F898 = result;
  return result;
}



// Function: strip_compression_suffix @ 0x329b0
void *strip_compression_suffix(const char *a1)
{
  size_t n; // [rsp+10h] [rbp-20h] BYREF
  void *dest; // [rsp+18h] [rbp-18h]
  char **v4; // [rsp+20h] [rbp-10h]
  unsigned long long v5; // [rsp+28h] [rbp-8h]

  v5 = __readfsqword(0x28u);
  dest = 0;
  v4 = sub_3286E(a1, &n);
  if ( v4 )
  {
    if ( n > 4 && !strncmp(&a1[n - 4], ".tar", 4u) && **v4 != 116 )
      n -= 4LL;
    if ( !n )
      return 0;
    dest = (void *)sub_60B5B(n + 1);
    memcpy(dest, a1, n);
    *((char *)dest + n) = 0;
  }
  return dest;
}



// Function: xexec @ 0x32aaa
void xexec(char *a1)
{
  char *argv[6]; // [rsp+10h] [rbp-30h] BYREF

  argv[5] = (char *)__readfsqword(0x28u);
  argv[0] = "/bin/sh";
  argv[1] = "-c";
  argv[2] = a1;
  argv[3] = 0;
  execv("/bin/sh", argv);
  sub_3E251(a1);
}



// Function: sys_get_archive_stat @ 0x32b11
bool sys_get_archive_stat()
{
  return fstat(fd, &buf) == 0;
}



// Function: sys_file_is_archive @ 0x32b37
long long sys_file_is_archive(long long a1)
{
  return qword_9FBF8 && *(long long *)(a1 + 88) == qword_9FBF8 && *(long long *)(a1 + 96) == qword_9FC00;
}



// Function: sys_save_archive_dev_ino @ 0x32b88
__ino_t sys_save_archive_dev_ino()
{
  __ino_t result; // rax

  result = (unsigned int)fd;
  if ( fd <= 0x3FFFFFFF && (result = buf.st_mode & 0xF000, (int)result == 0x8000) )
  {
    qword_9FBF8 = buf.st_dev;
    qword_9FC00 = buf.st_ino;
    return buf.st_ino;
  }
  else
  {
    qword_9FBF8 = 0;
  }
  return result;
}



// Function: sys_detect_dev_null_output @ 0x32bdc
unsigned long long sys_detect_dev_null_output()
{
  bool v0; // al
  struct stat buf; // [rsp+0h] [rbp-A0h] BYREF
  unsigned long long v3; // [rsp+98h] [rbp-8h]

  v3 = __readfsqword(0x28u);
  v0 = !strcmp(*(const char **)qword_9FBD8, "/dev/null")
    || fd <= 0x3FFFFFFF
    && (::buf.st_mode & 0xF000) == 0x2000
    && !stat("/dev/null", &buf)
    && ::buf.st_dev == buf.st_dev
    && ::buf.st_ino == buf.st_ino;
  byte_9F9D0 = v0;
  return v3 - __readfsqword(0x28u);
}



// Function: sys_wait_for_child @ 0x32ca8
unsigned long long sys_wait_for_child(unsigned int a1, char a2)
{
  char *v2; // rdi
  char *v3; // rax
  unsigned int v4; // ebx
  char *v5; // rax
  int stat_loc; // [rsp+10h] [rbp-20h] BYREF
  unsigned int v9; // [rsp+14h] [rbp-1Ch]
  unsigned long long v10; // [rsp+18h] [rbp-18h]

  v10 = __readfsqword(0x28u);
  if ( a1 )
  {
    while ( 1 )
    {
      v2 = (char *)a1;
      if ( waitpid(a1, &stat_loc, 0) != -1 )
        break;
      if ( *__errno_location() != 4 )
      {
        v2 = qword_9F898;
        sub_3EAF9(qword_9F898);
        break;
      }
    }
    if ( (char)((stat_loc & 0x7F) + 1) >> 1 <= 0 )
    {
      if ( BYTE1(stat_loc) )
      {
        if ( qword_9FDC8 )
          qword_9FDC8(v2);
        v4 = BYTE1(stat_loc);
        v5 = gettext("Child returned status %d");
        error(0, 0, v5, v4);
        sub_177F4();
      }
    }
    else
    {
      v9 = stat_loc & 0x7F;
      if ( a2 || v9 != 13 )
      {
        if ( qword_9FDC8 )
          qword_9FDC8(v2);
        v3 = gettext("Child died with signal %d");
        error(0, 0, v3, v9);
        sub_177F4();
      }
    }
  }
  return v10 - __readfsqword(0x28u);
}



// Function: sys_spawn_shell @ 0x32df0
unsigned long long sys_spawn_shell()
{
  int stat_loc; // [rsp+8h] [rbp-18h] BYREF
  __pid_t pid; // [rsp+Ch] [rbp-14h]
  char *file; // [rsp+10h] [rbp-10h]
  unsigned long long v4; // [rsp+18h] [rbp-8h]

  v4 = __readfsqword(0x28u);
  file = getenv("SHELL");
  if ( !file )
    file = "/bin/sh";
  pid = sub_2B537();
  if ( !pid )
  {
    sub_5C835();
    execlp(file, "-sh", "-i", 0);
    sub_3E251(file);
  }
  while ( waitpid(pid, &stat_loc, 0) == -1 )
  {
    if ( *__errno_location() != 4 )
    {
      sub_3EAF9(file);
      return v4 - __readfsqword(0x28u);
    }
  }
  return v4 - __readfsqword(0x28u);
}



// Function: sys_compare_uid @ 0x32ebf
bool sys_compare_uid(long long a1, long long a2)
{
  return *(int *)(a1 + 28) == *(int *)(a2 + 28);
}



// Function: sys_compare_gid @ 0x32ee4
bool sys_compare_gid(long long a1, long long a2)
{
  return *(int *)(a1 + 32) == *(int *)(a2 + 32);
}



// Function: sys_compare_links @ 0x32f09
long long sys_compare_links(long long *a1, long long *a2)
{
  return *a2 == *a1 && a2[1] == a1[1];
}



// Function: sys_truncate @ 0x32f52
int sys_truncate(int a1)
{
  long long length; // [rsp+18h] [rbp-8h]

  length = lseek(a1, 0, 1);
  if ( length < 0 )
    return -1;
  else
    return ftruncate(a1, length);
}



// Function: is_regular_file @ 0x32f9a
long long is_regular_file(const char *a1)
{
  struct stat buf; // [rsp+10h] [rbp-A0h] BYREF
  unsigned long long v3; // [rsp+A8h] [rbp-8h]

  v3 = __readfsqword(0x28u);
  if ( stat(a1, &buf) )
    return *__errno_location() == 2;
  else
    return (buf.st_mode & 0xF000) == 0x8000;
}



// Function: sys_write_archive_buffer @ 0x3301a
long long sys_write_archive_buffer()
{
  if ( fd <= 0x3FFFFFFF )
    return sub_52175((unsigned int)fd, s1, qword_9F870);
  else
    return sub_3FF8C((unsigned int)(fd - 0x40000000), s1, qword_9F870);
}



// Function: xdup2 @ 0x3307b
int xdup2(unsigned int a1, unsigned int a2)
{
  int result; // eax
  char *v3; // rax
  char *v4; // rax
  int v5; // [rsp+14h] [rbp-Ch]
  int errnum; // [rsp+18h] [rbp-8h]
  int v7; // [rsp+1Ch] [rbp-4h]

  result = a1;
  if ( a1 != a2 )
  {
    if ( close(a2) && *__errno_location() != 9 )
    {
      errnum = *__errno_location();
      if ( qword_9FDC8 )
        qword_9FDC8(a2);
      v3 = gettext("Cannot close");
      error(0, errnum, v3);
      sub_177F4();
    }
    v5 = dup(a1);
    if ( v5 != a2 )
    {
      if ( v5 < 0 )
      {
        v7 = *__errno_location();
        if ( qword_9FDC8 )
          qword_9FDC8(a1);
        v4 = gettext("Cannot dup");
        error(0, v7, v4);
        sub_177F4();
      }
      abort();
    }
    return sub_BC11(a1);
  }
  return result;
}



// Function: wait_for_grandchild @ 0x33179
void wait_for_grandchild(__pid_t a1)
{
  __pid_t i; // eax
  int stat_loc; // [rsp+10h] [rbp-10h] BYREF
  int status; // [rsp+14h] [rbp-Ch]
  unsigned long long v4; // [rsp+18h] [rbp-8h]

  v4 = __readfsqword(0x28u);
  status = 0;
  for ( i = waitpid(a1, &stat_loc, 0); ; i = waitpid(a1, &stat_loc, 0) )
  {
    if ( i != -1 )
      goto LABEL_6;
    if ( *__errno_location() != 4 )
      break;
  }
  sub_3EAF9(qword_9F898);
LABEL_6:
  if ( (char)((stat_loc & 0x7F) + 1) >> 1 <= 0 )
  {
    if ( BYTE1(stat_loc) )
      status = BYTE1(stat_loc);
  }
  else
  {
    raise(stat_loc & 0x7F);
  }
  exit(status);
}



// Function: sys_child_open_for_compress @ 0x3321b
long long sys_child_open_for_compress()
{
  char *v1; // rax
  char *v2; // rax
  int v3; // eax
  int v4; // [rsp+Ch] [rbp-44h]
  int v5; // [rsp+10h] [rbp-40h]
  __pid_t v6; // [rsp+14h] [rbp-3Ch]
  long long v7; // [rsp+18h] [rbp-38h]
  long long v8; // [rsp+18h] [rbp-38h]
  long long v9; // [rsp+18h] [rbp-38h]
  char *i; // [rsp+20h] [rbp-30h]
  unsigned long long v11; // [rsp+28h] [rbp-28h]
  unsigned int v12; // [rsp+38h] [rbp-18h] BYREF
  int v13; // [rsp+3Ch] [rbp-14h]
  unsigned int v14; // [rsp+40h] [rbp-10h] BYREF
  unsigned int v15; // [rsp+44h] [rbp-Ch]
  unsigned long long v16; // [rsp+48h] [rbp-8h]

  v16 = __readfsqword(0x28u);
  signal(13, (__sighandler_t)((char *)&dword_0 + 1));
  sub_2B577((int *)&v12);
  v4 = sub_2B537();
  if ( v4 <= 0 )
  {
    v1 = gettext("tar (child)");
    sub_5C844(v1);
    signal(13, 0);
    sub_3307B(v12, 0);
    sub_BC11(v13);
    if ( (byte_9FDF8
       || (qword_9FDF0 = (long long)strchr(*(const char **)qword_9FBD8, 58)) == 0
       || *(long long *)qword_9FBD8 >= (unsigned long long)qword_9FDF0
       || memchr(*(const void **)qword_9FBD8, 47, qword_9FDF0 - *(long long *)qword_9FBD8))
      && sub_32F9A(*(const char **)qword_9FBD8) )
    {
      if ( byte_9F884 )
        sub_2A571(*(const char **)qword_9FBD8, 1);
      if ( strcmp(*(const char **)qword_9FBD8, "-") )
      {
        fd = creat(*(const char **)qword_9FBD8, 0x1B6u);
        if ( fd < 0 )
        {
          v5 = *__errno_location();
          if ( byte_9F884 )
            sub_2A874();
          *__errno_location() = v5;
          sub_3E3B4(*(long long *)qword_9FBD8);
        }
        sub_3307B(fd, 1u);
      }
      sub_5C835();
      sub_32AAA(qword_9F898);
    }
    sub_2B577((int *)&v14);
    v6 = sub_2B537();
    if ( !v6 )
    {
      v2 = gettext("tar (grandchild)");
      sub_5C844(v2);
      sub_3307B(v15, 1u);
      sub_BC11(v14);
      sub_5C835();
      sub_32AAA(qword_9F898);
    }
    sub_3307B(v14, 0);
    sub_BC11(v15);
    if ( !strcmp(*(const char **)qword_9FBD8, "-") )
    {
      fd = 1;
    }
    else
    {
      if ( byte_9FDF8 != 1
        && (qword_9FDF0 = (long long)strchr(*(const char **)qword_9FBD8, 58)) != 0
        && *(long long *)qword_9FBD8 < (unsigned long long)qword_9FDF0
        && !memchr(*(const void **)qword_9FBD8, 47, qword_9FDF0 - *(long long *)qword_9FBD8) )
      {
        v3 = sub_3F860(*(long long *)qword_9FBD8, 65, 0x40000000, qword_9F950);
      }
      else
      {
        v3 = creat(*(const char **)qword_9FBD8, 0x1B6u);
      }
      fd = v3;
      if ( v3 < 0 )
        sub_3E3B4(*(long long *)qword_9FBD8);
    }
    while ( 1 )
    {
      v7 = 0;
      v11 = 0;
      for ( i = (char *)s1; v11 < qword_9F870; i += v7 )
      {
        v7 = sub_5EC70(0, i, qword_9F870 - v11);
        if ( v7 == -1 )
          sub_3E5D6(qword_9F898);
        if ( !v7 )
          break;
        v11 += v7;
      }
      if ( !v7 )
      {
        if ( v11 )
        {
          memset((char *)s1 + v11, 0, qword_9F870 - v11);
          v8 = sub_3301A();
          if ( v8 != qword_9F870 )
            sub_C6D5(v8);
        }
        sub_33179(v6);
      }
      v9 = sub_3301A();
      if ( v9 != qword_9F870 )
        sub_C6D5(v9);
    }
  }
  fd = v13;
  sub_BC11(v12);
  return (unsigned int)v4;
}



// Function: run_decompress_program @ 0x336d5
void run_decompress_program(double a1)
{
  int *v1; // rdi
  char *v2; // rbx
  int *v3; // rax
  char *v4; // rax
  long long v5; // rdi
  long long v6; // rbx
  char *v7; // rax
  char *v8; // rax
  int v9; // [rsp+8h] [rbp-118h] BYREF
  unsigned int v10; // [rsp+Ch] [rbp-114h]
  long long i; // [rsp+10h] [rbp-110h]
  long long v12; // [rsp+18h] [rbp-108h]
  long long v13; // [rsp+20h] [rbp-100h] BYREF
  void *dest; // [rsp+28h] [rbp-F8h]
  long long v15; // [rsp+30h] [rbp-F0h]
  long long v16; // [rsp+90h] [rbp-90h]
  unsigned long long v17; // [rsp+108h] [rbp-18h]

  v17 = __readfsqword(0x28u);
  v12 = 0;
  v10 = 34082310;
  v16 = _environ;
  v15 = 1;
  v1 = &v9;
  for ( i = (long long)sub_AFA2(&v9); i; i = sub_B012(&v9) )
  {
    if ( v12 )
    {
      if ( (dword_9EC84 & 0x80000) != 0 )
      {
        if ( qword_9FDC8 )
          qword_9FDC8(&v9);
        v2 = gettext("cannot run %s");
        v3 = __errno_location();
        v1 = 0;
        error(0, *v3, v2, v12);
      }
      if ( (dword_9EC84 & 0x80000) != 0 )
      {
        if ( qword_9FDC8 )
          qword_9FDC8(v1);
        v4 = gettext("trying %s");
        error(0, 0, v4, i);
      }
    }
    v5 = i;
    if ( (unsigned int)sub_461A6(i, &v13, v10, a1) )
    {
      if ( qword_9FDC8 )
        qword_9FDC8(v5);
      v6 = sub_4647F(&v13);
      v7 = gettext("cannot split string '%s': %s");
      error(0, 0, v7, i, v6);
      sub_177F4();
    }
    v10 |= 8u;
    memmove(dest, (char *)dest + 8 * v15, 8 * v13);
    *((long long *)dest + v13) = "-d";
    v12 = i;
    execvp(*(const char **)dest, (char *const *)dest);
    *((long long *)dest + v13) = 0;
    v1 = &v9;
  }
  if ( !v12 )
  {
    if ( qword_9FDC8 )
      qword_9FDC8(&v9);
    v8 = gettext("unable to run decompression program");
    error(0, 0, v8);
    sub_177F4();
  }
  sub_3E251(v12);
}



// Function: sys_child_open_for_uncompress @ 0x3399b
long long sys_child_open_for_uncompress()
{
  char *v1; // rax
  double v2; // xmm0_8
  char *v3; // rax
  double v4; // xmm0_8
  int v5; // eax
  long long v6; // rax
  long long v7; // rax
  int v8; // [rsp+0h] [rbp-40h]
  __pid_t v9; // [rsp+4h] [rbp-3Ch]
  char *v10; // [rsp+8h] [rbp-38h]
  unsigned long long i; // [rsp+10h] [rbp-30h]
  long long v12; // [rsp+20h] [rbp-20h]
  int v13; // [rsp+28h] [rbp-18h] BYREF
  unsigned int v14; // [rsp+2Ch] [rbp-14h]
  unsigned int v15; // [rsp+30h] [rbp-10h] BYREF
  unsigned int v16; // [rsp+34h] [rbp-Ch]
  unsigned long long v17; // [rsp+38h] [rbp-8h]

  v17 = __readfsqword(0x28u);
  sub_2B577(&v13);
  v8 = sub_2B537();
  if ( v8 <= 0 )
  {
    v1 = gettext("tar (child)");
    sub_5C844(v1);
    signal(13, 0);
    sub_3307B(v14, 1u);
    sub_BC11(v13);
    if ( strcmp(*(const char **)qword_9FBD8, "-") )
    {
      if ( (byte_9FDF8
         || (qword_9FDF0 = (long long)strchr(*(const char **)qword_9FBD8, 58)) == 0
         || *(long long *)qword_9FBD8 >= (unsigned long long)qword_9FDF0
         || memchr(*(const void **)qword_9FBD8, 47, qword_9FDF0 - *(long long *)qword_9FBD8))
        && sub_32F9A(*(const char **)qword_9FBD8) )
      {
        fd = open(*(const char **)qword_9FBD8, 0, 438);
        if ( fd < 0 )
          sub_3E3B4(*(long long *)qword_9FBD8);
        sub_3307B(fd, 0);
        v2 = sub_5C835();
        sub_336D5(v2);
      }
    }
    sub_2B577((int *)&v15);
    v9 = sub_2B537();
    if ( !v9 )
    {
      v3 = gettext("tar (grandchild)");
      sub_5C844(v3);
      sub_3307B(v15, 0);
      sub_BC11(v16);
      v4 = sub_5C835();
      sub_336D5(v4);
    }
    sub_3307B(v16, 1u);
    sub_BC11(v15);
    if ( !strcmp(*(const char **)qword_9FBD8, "-") )
    {
      fd = 0;
    }
    else
    {
      if ( byte_9FDF8 != 1
        && (qword_9FDF0 = (long long)strchr(*(const char **)qword_9FBD8, 58)) != 0
        && *(long long *)qword_9FBD8 < (unsigned long long)qword_9FDF0
        && !memchr(*(const void **)qword_9FBD8, 47, qword_9FDF0 - *(long long *)qword_9FBD8) )
      {
        v5 = sub_3F860(*(long long *)qword_9FBD8, 0, 0x40000000, qword_9F950);
      }
      else
      {
        v5 = open(*(const char **)qword_9FBD8, 0, 438);
      }
      fd = v5;
    }
    if ( fd < 0 )
      sub_3E3B4(*(long long *)qword_9FBD8);
    while ( 1 )
    {
      sub_ADB0();
      while ( 1 )
      {
        v6 = fd <= 0x3FFFFFFF
           ? sub_5EC70((unsigned int)fd, s1, qword_9F870)
           : sub_3FE62((unsigned int)(fd - 0x40000000), s1, qword_9F870);
        if ( v6 != -1 )
          break;
        sub_C729();
      }
      if ( !v6 )
        break;
      v10 = (char *)s1;
      for ( i = v6; i; i -= v12 )
      {
        v7 = i;
        if ( i > 0x200 )
          v7 = 512;
        v12 = v7;
        if ( v7 != sub_52175(1, v10, v7) )
          sub_3EB22(qword_9F898);
        v10 += v12;
      }
    }
    sub_BC11(1);
    sub_33179(v9);
  }
  fd = v13;
  sub_BC11(v14);
  return (unsigned int)v8;
}



// Function: dec_to_env @ 0x33de6
unsigned long long dec_to_env(const char *a1, long long a2)
{
  char *value; // [rsp+18h] [rbp-28h]
  char v4[24]; // [rsp+20h] [rbp-20h] BYREF
  unsigned long long v5; // [rsp+38h] [rbp-8h]

  v5 = __readfsqword(0x28u);
  value = (char *)sub_54E5A(a2, v4);
  if ( setenv(a1, value, 1) )
    sub_1783C();
  return v5 - __readfsqword(0x28u);
}



// Function: time_to_env @ 0x33e58
unsigned long long time_to_env(const char *a1, long long a2, unsigned int a3)
{
  const char *v3; // rax
  char v5[40]; // [rsp+20h] [rbp-30h] BYREF
  unsigned long long v6; // [rsp+48h] [rbp-8h]

  v6 = __readfsqword(0x28u);
  v3 = (const char *)sub_29F2E(a2, a3, (long long)v5);
  if ( setenv(a1, v3, 1) )
    sub_1783C();
  return v6 - __readfsqword(0x28u);
}



// Function: oct_to_env @ 0x33ed9
unsigned long long oct_to_env(const char *a1, long long a2)
{
  char s[24]; // [rsp+10h] [rbp-20h] BYREF
  unsigned long long v4; // [rsp+28h] [rbp-8h]

  v4 = __readfsqword(0x28u);
  snprintf(s, 0x18u, "0%lo", a2);
  if ( setenv(a1, s, 1) )
    sub_1783C();
  return v4 - __readfsqword(0x28u);
}



// Function: str_to_env @ 0x33f58
int str_to_env(const char *a1, const char *a2)
{
  int result; // eax

  if ( !a2 )
    return unsetenv(a1);
  result = setenv(a1, a2, 1);
  if ( result )
    sub_1783C();
  return result;
}



// Function: chr_to_env @ 0x33fa3
unsigned long long chr_to_env(const char *a1, char a2)
{
  char value[2]; // [rsp+16h] [rbp-Ah] BYREF
  unsigned long long v4; // [rsp+18h] [rbp-8h]

  v4 = __readfsqword(0x28u);
  value[0] = a2;
  value[1] = 0;
  if ( setenv(a1, value, 1) )
    sub_1783C();
  return v4 - __readfsqword(0x28u);
}



// Function: stat_to_env @ 0x3400a
int stat_to_env(const char *a1, char a2, long long a3)
{
  unsigned int v3; // eax
  const char *v4; // rax
  unsigned int v5; // eax
  unsigned int v6; // eax

  sub_33F58("TAR_VERSION", "1.34");
  sub_33F58("TAR_ARCHIVE", *(const char **)qword_9FBE8);
  sub_33DE6("TAR_VOLUME", ((qword_9FBE8 - (long long)qword_9FBD8) >> 3) + 1);
  sub_33DE6("TAR_BLOCKING_FACTOR", dword_9F868);
  if ( dword_9F468 )
    v3 = dword_9F468;
  else
    v3 = dword_9F864;
  v4 = (const char *)sub_34D66(v3);
  sub_33F58("TAR_FORMAT", v4);
  sub_33FA3("TAR_FILETYPE", a2);
  sub_33ED9("TAR_MODE", *(unsigned int *)(a3 + 112));
  sub_33F58("TAR_FILENAME", a1);
  sub_33F58("TAR_REALNAME", *(const char **)(a3 + 8));
  sub_33F58("TAR_UNAME", *(const char **)(a3 + 32));
  sub_33F58("TAR_GNAME", *(const char **)(a3 + 40));
  sub_33E58("TAR_ATIME", *(long long *)(a3 + 232), *(long long *)(a3 + 240));
  sub_33E58("TAR_MTIME", *(long long *)(a3 + 248), *(long long *)(a3 + 256));
  sub_33E58("TAR_CTIME", *(long long *)(a3 + 264), *(long long *)(a3 + 272));
  sub_33DE6("TAR_SIZE", *(long long *)(a3 + 136));
  sub_33DE6("TAR_UID", *(unsigned int *)(a3 + 116));
  sub_33DE6("TAR_GID", *(unsigned int *)(a3 + 120));
  if ( a2 == 108 )
    goto LABEL_12;
  if ( a2 <= 108 )
  {
    if ( a2 <= 99 )
    {
      if ( a2 >= 98 )
      {
        v5 = gnu_dev_minor(*(long long *)(a3 + 128));
        sub_33DE6("TAR_MINOR", v5);
        v6 = gnu_dev_major(*(long long *)(a3 + 128));
        sub_33DE6("TAR_MAJOR", v6);
        return unsetenv("TAR_LINKNAME");
      }
      goto LABEL_13;
    }
    if ( a2 == 104 )
    {
LABEL_12:
      unsetenv("TAR_MINOR");
      unsetenv("TAR_MAJOR");
      return sub_33F58("TAR_LINKNAME", *(const char **)(a3 + 24));
    }
  }
LABEL_13:
  unsetenv("TAR_MINOR");
  unsetenv("TAR_MAJOR");
  return unsetenv("TAR_LINKNAME");
}



// Function: sys_exec_command @ 0x34323
long long sys_exec_command(const char *a1, char a2, long long a3)
{
  unsigned int v5; // [rsp+20h] [rbp-10h] BYREF
  unsigned int v6; // [rsp+24h] [rbp-Ch]
  unsigned long long v7; // [rsp+28h] [rbp-8h]

  v7 = __readfsqword(0x28u);
  sub_2B577((int *)&v5);
  handler = signal(13, (__sighandler_t)((char *)&dword_0 + 1));
  pid = sub_2B537();
  if ( !pid )
  {
    sub_3307B(v5, 0);
    sub_BC11(v6);
    sub_3400A(a1, a2, a3);
    sub_5C835();
    sub_32AAA((char *)qword_9F9A0);
  }
  sub_BC11(v5);
  return v6;
}



// Function: sys_wait_command @ 0x343ea
unsigned long long sys_wait_command()
{
  long long v0; // rdi
  unsigned int v1; // r12d
  long long v2; // rbx
  char *v3; // rax
  unsigned int v4; // r12d
  long long v5; // rbx
  char *v6; // rax
  long long v7; // rbx
  char *v8; // rax
  int stat_loc; // [rsp+4h] [rbp-1Ch] BYREF
  unsigned long long v11; // [rsp+8h] [rbp-18h]

  v11 = __readfsqword(0x28u);
  if ( pid >= 0 )
  {
    signal(13, handler);
    while ( 1 )
    {
      v0 = (unsigned int)pid;
      if ( waitpid(pid, &stat_loc, 0) != -1 )
        break;
      if ( *__errno_location() != 4 )
      {
        pid = -1;
        sub_3EAF9(qword_9F9A0);
        return v11 - __readfsqword(0x28u);
      }
    }
    if ( (stat_loc & 0x7F) != 0 )
    {
      if ( (char)((stat_loc & 0x7F) + 1) >> 1 <= 0 )
      {
        if ( qword_9FDC8 )
          qword_9FDC8(v0);
        v7 = pid;
        v8 = gettext("%lu: Child terminated on unknown reason");
        error(0, 0, v8, v7);
        dword_9FDD0 = 2;
      }
      else
      {
        if ( qword_9FDC8 )
          qword_9FDC8(v0);
        v4 = stat_loc & 0x7F;
        v5 = pid;
        v6 = gettext("%lu: Child terminated on signal %d");
        error(0, 0, v6, v5, v4);
      }
    }
    else if ( byte_9F9A8 != 1 && BYTE1(stat_loc) )
    {
      if ( qword_9FDC8 )
        qword_9FDC8(v0);
      v1 = BYTE1(stat_loc);
      v2 = pid;
      v3 = gettext("%lu: Child returned status %d");
      error(0, 0, v3, v2, v1);
      dword_9FDD0 = 2;
    }
    pid = -1;
  }
  return v11 - __readfsqword(0x28u);
}



// Function: sys_exec_info_script @ 0x345e7
long long sys_exec_info_script(const char **a1, int a2)
{
  const char *v3; // rax
  const char *v4; // rsi
  const char *v5; // rax
  unsigned int v6; // eax
  const char *v7; // rax
  const char *v8; // rsi
  int stat_loc; // [rsp+14h] [rbp-4Ch] BYREF
  int v10; // [rsp+18h] [rbp-48h]
  __pid_t pid; // [rsp+1Ch] [rbp-44h]
  char *lineptr; // [rsp+20h] [rbp-40h] BYREF
  size_t n; // [rsp+28h] [rbp-38h] BYREF
  FILE *stream; // [rsp+30h] [rbp-30h]
  int fd; // [rsp+38h] [rbp-28h] BYREF
  int v16; // [rsp+3Ch] [rbp-24h]
  char v17[24]; // [rsp+40h] [rbp-20h] BYREF
  unsigned long long v18; // [rsp+58h] [rbp-8h]

  v18 = __readfsqword(0x28u);
  sub_2B577(&fd);
  qword_9F840 = signal(13, (__sighandler_t)((char *)&dword_0 + 1));
  pid = sub_2B537();
  if ( !pid )
  {
    setenv("TAR_VERSION", "1.34", 1);
    setenv("TAR_ARCHIVE", *a1, 1);
    v3 = (const char *)sub_54E5A(a2, v17);
    setenv("TAR_VOLUME", v3, 1);
    v4 = (const char *)sub_54E5A(dword_9F868, v17);
    setenv("TAR_BLOCKING_FACTOR", v4, 1);
    v5 = (const char *)sub_34E1C((unsigned int)dword_9F860);
    setenv("TAR_SUBCOMMAND", v5, 1);
    if ( dword_9F468 )
      v6 = dword_9F468;
    else
      v6 = dword_9F864;
    v7 = (const char *)sub_34D66(v6);
    setenv("TAR_FORMAT", v7, 1);
    v8 = (const char *)sub_54E5A(v16, v17);
    setenv("TAR_FD", v8, 1);
    sub_BC11(fd);
    sub_5C835();
    sub_32AAA((char *)qword_9F8C0);
  }
  lineptr = 0;
  n = 0;
  sub_BC11(v16);
  stream = fdopen(fd, "r");
  v10 = getline(&lineptr, &n, stream);
  fclose(stream);
  if ( v10 > 0 && lineptr[v10 - 1] == 10 )
    lineptr[--v10] = 0;
  while ( waitpid(pid, &stat_loc, 0) == -1 )
  {
    if ( *__errno_location() != 4 )
    {
      signal(13, qword_9F840);
      sub_3EAF9(qword_9F8C0);
      return 0xFFFFFFFFLL;
    }
  }
  signal(13, qword_9F840);
  if ( (stat_loc & 0x7F) != 0 )
  {
    free(lineptr);
    return 0xFFFFFFFFLL;
  }
  else
  {
    if ( BYTE1(stat_loc) || v10 <= 0 )
      free(lineptr);
    else
      *a1 = lineptr;
    return BYTE1(stat_loc);
  }
}



// Function: sys_exec_checkpoint_script @ 0x348d7
unsigned long long sys_exec_checkpoint_script(char *a1, const char *a2, int a3)
{
  const char *v3; // rax
  const char *v4; // rax
  const char *v5; // rax
  unsigned int v6; // eax
  const char *v7; // rsi
  int stat_loc; // [rsp+28h] [rbp-28h] BYREF
  __pid_t pid; // [rsp+2Ch] [rbp-24h]
  char v12[24]; // [rsp+30h] [rbp-20h] BYREF
  unsigned long long v13; // [rsp+48h] [rbp-8h]

  v13 = __readfsqword(0x28u);
  pid = sub_2B537();
  if ( !pid )
  {
    setenv("TAR_VERSION", "1.34", 1);
    setenv("TAR_ARCHIVE", a2, 1);
    v3 = (const char *)sub_54E5A(a3, v12);
    setenv("TAR_CHECKPOINT", v3, 1);
    v4 = (const char *)sub_54E5A(dword_9F868, v12);
    setenv("TAR_BLOCKING_FACTOR", v4, 1);
    v5 = (const char *)sub_34E1C((unsigned int)dword_9F860);
    setenv("TAR_SUBCOMMAND", v5, 1);
    if ( dword_9F468 )
      v6 = dword_9F468;
    else
      v6 = dword_9F864;
    v7 = (const char *)sub_34D66(v6);
    setenv("TAR_FORMAT", v7, 1);
    sub_5C835();
    sub_32AAA(a1);
  }
  while ( waitpid(pid, &stat_loc, 0) == -1 )
  {
    if ( *__errno_location() != 4 )
    {
      sub_3EAF9(a1);
      return v13 - __readfsqword(0x28u);
    }
  }
  return v13 - __readfsqword(0x28u);
}



// Function: request_stdin @ 0x34a6f
long long request_stdin(long long a1)
{
  long long v1; // rbx
  char *v2; // rax

  if ( qword_9FC40 )
  {
    if ( qword_9FDC8 )
      qword_9FDC8(a1);
    v1 = qword_9FC40;
    v2 = gettext("Options '%s' and '%s' both want standard input");
    error(0, 0, v2, v1, a1);
    sub_3814C(2);
  }
  qword_9FC40 = a1;
  return a1;
}



// Function: confirm @ 0x34af4
long long confirm(const char *a1, long long a2)
{
  const char *v2; // rax
  bool v4; // [rsp+17h] [rbp-19h]
  char *lineptr; // [rsp+18h] [rbp-18h] BYREF
  size_t n[2]; // [rsp+20h] [rbp-10h] BYREF

  n[1] = __readfsqword(0x28u);
  v4 = 0;
  if ( !qword_9FCB0 )
  {
    if ( fd && !qword_9FC40 )
    {
      sub_34A6F((long long)"-w");
      qword_9FCB0 = (FILE *)stdin;
    }
    else
    {
      qword_9FCB0 = fopen("/dev/tty", "r");
      if ( !qword_9FCB0 )
        sub_3E3B4("/dev/tty");
    }
  }
  v2 = (const char *)sub_5E84E(a2);
  fprintf(stream, "%s %s?", a1, v2);
  fflush_unlocked(stream);
  if ( !dword_9FCB8 )
  {
    lineptr = 0;
    n[0] = 0;
    if ( getline(&lineptr, n, qword_9FCB0) >= 0 )
      v4 = rpmatch(lineptr) > 0;
    else
      dword_9FCB8 = 1;
    free(lineptr);
  }
  if ( dword_9FCB8 )
  {
    fputc_unlocked(10, stream);
    fflush_unlocked(stream);
  }
  return v4;
}



// Function: set_archive_format @ 0x34c7e
long long set_archive_format(const char *a1)
{
  long long v1; // rbx
  char *v2; // rax
  const char *v3; // rdi
  long long result; // rax
  char **v6; // [rsp+18h] [rbp-18h]

  v6 = &off_9A8E0;
  while ( 1 )
  {
    v3 = *v6;
    if ( !strcmp(*v6, a1) )
      break;
    v6 += 2;
    if ( !*v6 )
    {
      if ( qword_9FDC8 )
        qword_9FDC8(v3);
      v1 = sub_5E548(a1);
      v2 = gettext("%s: Invalid archive format");
      error(0, 0, v2, v1);
      sub_3814C(2);
    }
  }
  result = *((unsigned int *)v6 + 2);
  dword_9F864 = *((int *)v6 + 2);
  return result;
}



// Function: set_xattr_option @ 0x34d36
long long set_xattr_option(unsigned int a1)
{
  if ( a1 == 1 )
    sub_34C7E("posix");
  dword_9F96C = a1;
  return a1;
}



// Function: archive_format_string @ 0x34d66
const char *archive_format_string(int a1)
{
  char **i; // [rsp+Ch] [rbp-8h]

  for ( i = &off_9A8E0; *i; i += 2 )
  {
    if ( a1 == *((int *)i + 2) )
      return *i;
  }
  return "unknown?";
}



// Function: assert_format @ 0x34dad
long long assert_format(long long a1)
{
  long long result; // rax
  char *v2; // rax

  result = (unsigned int)a1 & (1 << dword_9F864);
  if ( !(int)result )
  {
    if ( qword_9FDC8 )
      qword_9FDC8(a1);
    v2 = gettext("GNU features wanted on incompatible archive format");
    error(0, 0, v2);
    sub_3814C(2);
  }
  return result;
}



// Function: subcommand_string @ 0x34e1c
const char *subcommand_string(int a1)
{
  const char *result; // rax

  switch ( a1 )
  {
    case 0:
      result = "unknown?";
      break;
    case 1:
      result = "-r";
      break;
    case 2:
      result = "-A";
      break;
    case 3:
      result = "-c";
      break;
    case 4:
      result = "-D";
      break;
    case 5:
      result = "-d";
      break;
    case 6:
      result = "-x";
      break;
    case 7:
      result = "-t";
      break;
    case 8:
      result = "-u";
      break;
    case 9:
      result = "--test-label";
      break;
    default:
      abort();
  }
  return result;
}



// Function: tar_list_quoting_styles @ 0x34eb6
char *tar_list_quoting_styles(long long a1, const char *a2)
{
  char *v2; // rax
  char *result; // rax
  int i; // [rsp+14h] [rbp-4Ch]
  size_t v5; // [rsp+18h] [rbp-48h]
  size_t v6; // [rsp+40h] [rbp-20h]

  v5 = strlen(a2);
  for ( i = 0; ; ++i )
  {
    result = off_9B6C0[i];
    if ( !result )
      break;
    if ( *(long long *)(a1 + 32) - *(long long *)(a1 + 24) < v5 )
      obstack_newchunk(a1, v5);
    memcpy(*(void **)(a1 + 24), a2, v5);
    *(long long *)(a1 + 24) += v5;
    v6 = strlen(off_9B6C0[i]);
    if ( *(long long *)(a1 + 32) - *(long long *)(a1 + 24) < v6 )
      obstack_newchunk(a1, v6);
    memcpy(*(void **)(a1 + 24), off_9B6C0[i], v6);
    *(long long *)(a1 + 24) += v6;
    if ( *(long long *)(a1 + 32) == *(long long *)(a1 + 24) )
      obstack_newchunk(a1, 1);
    v2 = *(char **)(a1 + 24);
    *(long long *)(a1 + 24) = v2 + 1;
    *v2 = 10;
  }
  return result;
}



// Function: tar_set_quoting_style @ 0x35084
long long tar_set_quoting_style(char *a1)
{
  long long v2; // rbx
  char *v3; // rax
  char *s1; // [rsp+8h] [rbp-28h]
  unsigned int i; // [rsp+1Ch] [rbp-14h]

  s1 = a1;
  for ( i = 0; ; ++i )
  {
    if ( !off_9B6C0[i] )
    {
      if ( qword_9FDC8 )
        qword_9FDC8(a1);
      v2 = qword_9FE48;
      v3 = gettext("Unknown quoting style '%s'. Try '%s --quoting-style=help' to get a list.");
      error(0, 0, v3, s1, v2);
      sub_177F4();
    }
    a1 = s1;
    if ( !strcmp(s1, off_9B6C0[i]) )
      break;
  }
  return sub_5C9A2(0, i);
}



// Function: format_default_settings @ 0x35152
long long format_default_settings()
{
  unsigned int v0; // eax

  v0 = (unsigned int)sub_34D66(6);
  return sub_61277(
           (unsigned int)"--format=%s -f%s -b%d --quoting-style=%s --rmt-command=%s --rsh-command=%s",
           v0,
           (unsigned int)"-",
           20,
           (unsigned int)"escape",
           (unsigned int)"/usr/local/libexec/rmt",
           (char)"/usr/bin/rsh");
}



// Function: option_conflict_error @ 0x351b6
void option_conflict_error(long long a1, long long a2)
{
  char *v2; // rax

  if ( qword_9FDC8 )
    qword_9FDC8(a1);
  v2 = gettext("'%s' cannot be used with '%s'");
  error(0, 0, v2, a1, a2);
  sub_3814C(2);
}



// Function: optloc_save @ 0x3521d
long long optloc_save(unsigned int a1, long long a2)
{
  size_t v3; // [rsp+18h] [rbp-18h]
  long long v4; // [rsp+20h] [rbp-10h]

  if ( a1 > 9 )
    abort();
  v3 = 32;
  if ( *(long long *)(a2 + 8) )
    v3 = strlen(*(const char **)(a2 + 8)) + 33;
  v4 = sub_60B5B(v3);
  if ( *(long long *)(a2 + 8) )
  {
    strcpy((char *)(v4 + 32), *(const char **)(a2 + 8));
    *(long long *)(v4 + 8) = v4 + 32;
  }
  else
  {
    *(long long *)(v4 + 8) = 0;
  }
  *(int *)v4 = *(int *)a2;
  *(long long *)(v4 + 16) = *(long long *)(a2 + 16);
  *(long long *)(v4 + 24) = qword_9FC60[a1];
  qword_9FC60[a1] = v4;
  return *(long long *)(v4 + 24);
}



// Function: optloc_lookup @ 0x35327
long long optloc_lookup(int a1)
{
  return qword_9FC60[a1];
}



// Function: option_set_in_cl @ 0x3534c
long long option_set_in_cl(int a1)
{
  int *v2; // [rsp+10h] [rbp-8h]

  v2 = (int *)sub_35327(a1);
  return v2 && *v2 == 1;
}



// Function: optloc_eq @ 0x35388
long long optloc_eq(long long a1, long long a2)
{
  if ( *(int *)a1 != *(int *)a2 )
    return 0;
  if ( *(int *)a1 == 1 )
    return 1;
  return strcmp(*(const char **)(a1 + 8), *(const char **)(a2 + 8)) == 0;
}



// Function: set_subcommand_option @ 0x353ea
long long set_subcommand_option(long long a1)
{
  char *v1; // rax

  if ( dword_9F860 && (int)a1 != dword_9F860 )
  {
    if ( qword_9FDC8 )
      qword_9FDC8(a1);
    v1 = gettext("You may not specify more than one '-Acdtrux', '--delete' or  '--test-label' option");
    error(0, 0, v1);
    sub_3814C(2);
  }
  dword_9F860 = a1;
  return (unsigned int)a1;
}



// Function: set_use_compress_program_option @ 0x3545f
const char *set_use_compress_program_option(const char *a1, long long a2)
{
  char *v2; // rdi
  char *v3; // rax
  int *v6; // [rsp+18h] [rbp-8h]

  v6 = (int *)sub_3521D(0, a2);
  if ( qword_9F898 )
  {
    v2 = qword_9F898;
    if ( strcmp(qword_9F898, a1) )
    {
      if ( *v6 == 1 )
      {
        if ( qword_9FDC8 )
          qword_9FDC8(v2);
        v3 = gettext("Conflicting compression options");
        error(0, 0, v3);
        sub_3814C(2);
      }
    }
  }
  qword_9F898 = (char *)a1;
  return a1;
}



// Function: sigstat @ 0x3550c
void sigstat(int a1)
{
  double v1; // xmm0_8

  v1 = sub_AE4D();
  sub_BAA2(v1);
  signal(a1, sigstat);
}



// Function: stat_on_signal @ 0x3553c
__sighandler_t stat_on_signal(int a1)
{
  return signal(a1, sub_3550C);
}



// Function: decode_signal @ 0x35562
long long decode_signal(const char *a1)
{
  char *v2; // rax
  const char *v3; // [rsp+8h] [rbp-18h]
  char **i; // [rsp+10h] [rbp-10h]
  char *s1; // [rsp+18h] [rbp-8h]

  v3 = a1;
  s1 = (char *)a1;
  if ( !strncmp(a1, "SIG", 3u) )
    s1 = (char *)(a1 + 3);
  for ( i = &off_9A9C0; ; i += 2 )
  {
    if ( i >= (char **)&unk_9AA10 )
    {
      if ( qword_9FDC8 )
        qword_9FDC8(a1);
      v2 = gettext("Unknown signal name: %s");
      error(0, 0, v2, v3);
      sub_177F4();
    }
    a1 = *i;
    if ( !strcmp(*i, s1) )
      break;
  }
  return *((unsigned int *)i + 2);
}



// Function: set_stat_signal @ 0x35629
__sighandler_t set_stat_signal(const char *a1)
{
  int v1; // eax

  v1 = sub_35562(a1);
  return sub_3553C(v1);
}



// Function: get_date_or_file @ 0x3564f
long long get_date_or_file(long long a1, long long a2, const char *a3, time_t *a4)
{
  char *v4; // rax
  time_t v5; // rdx
  long long v6; // r12
  char *v7; // rbx
  char *v8; // rax
  time_t v10; // rdx
  long long *v13; // [rsp+28h] [rbp-B8h]
  struct stat buf; // [rsp+30h] [rbp-B0h] BYREF
  unsigned long long v15; // [rsp+C8h] [rbp-18h]

  v15 = __readfsqword(0x28u);
  if ( *a3 == 47 || *a3 == 46 )
  {
    if ( stat(a3, &buf) )
    {
      sub_3EA03(a3);
      if ( qword_9FDC8 )
        qword_9FDC8(a3);
      v4 = gettext("Date sample file not found");
      error(0, 0, v4);
      sub_3814C(2);
    }
    *a4 = sub_5FD9E(&buf);
    a4[1] = v5;
    return 0;
  }
  if ( (unsigned char)sub_5A8ED(a4, a3, 0) == 1 )
  {
    v13 = (long long *)sub_60B5B(40);
    v10 = a4[1];
    v13[1] = *a4;
    v13[2] = v10;
    v13[3] = a2;
    v13[4] = sub_60CC6(a3);
    *v13 = *(long long *)(a1 + 8);
    *(long long *)(a1 + 8) = v13;
    return 0;
  }
  if ( qword_9FDC8 )
    qword_9FDC8(a4);
  v6 = sub_5E84E(a3);
  v7 = sub_27ABC(*a4, a4[1], 0);
  v8 = gettext("Substituting %s for unknown date format %s");
  error(0, 0, v8, v7, v6);
  a4[1] = 0;
  return 1;
}



// Function: report_textual_dates @ 0x3589e
void *report_textual_dates(long long a1)
{
  void *result; // rax
  const char *v2; // rdi
  long long v3; // r12
  long long v4; // rbx
  char *v5; // rax
  void *ptr; // [rsp+18h] [rbp-28h]
  void *v7; // [rsp+20h] [rbp-20h]
  const char *s2; // [rsp+28h] [rbp-18h]

  result = *(void **)(a1 + 8);
  for ( ptr = result; ptr; ptr = v7 )
  {
    v7 = *(void **)ptr;
    if ( dword_9F9AC )
    {
      s2 = sub_27ABC(*((long long *)ptr + 1), *((long long *)ptr + 2), 1);
      v2 = (const char *)*((long long *)ptr + 4);
      if ( strcmp(v2, s2) )
      {
        if ( qword_9FDC8 )
          qword_9FDC8(v2);
        v3 = *((long long *)ptr + 4);
        v4 = *((long long *)ptr + 3);
        v5 = gettext("Option %s: Treating date '%s' as %s");
        error(0, 0, v5, v4, v3, s2);
      }
    }
    free(*((void **)ptr + 4));
    free(ptr);
    result = v7;
  }
  return result;
}



// Function: tar_help_filter @ 0x359a8
long long tar_help_filter(int a1, long long a2)
{
  unsigned int v2; // eax
  int v3; // ecx
  int v4; // r8d
  int v5; // r9d
  unsigned int v6; // eax
  int v7; // ecx
  int v8; // r8d
  int v9; // r9d
  unsigned int v10; // eax
  int v11; // ecx
  int v12; // r8d
  int v13; // r9d
  unsigned int v14; // eax
  int v15; // ecx
  int v16; // r8d
  int v17; // r9d
  unsigned int v18; // eax
  int v19; // ecx
  int v20; // r8d
  int v21; // r9d
  unsigned int v22; // eax
  int v23; // ecx
  int v24; // r8d
  int v25; // r9d
  unsigned int v26; // eax
  int v27; // ecx
  int v28; // r8d
  int v29; // r9d
  unsigned int v30; // eax
  int v31; // ecx
  int v32; // r8d
  int v33; // r9d
  char *v34; // rax
  char *v35; // rax
  char *v38; // [rsp+20h] [rbp-110h]
  const char *s; // [rsp+28h] [rbp-108h]
  const char *sa; // [rsp+28h] [rbp-108h]
  unsigned long long n; // [rsp+38h] [rbp-F8h]
  unsigned long long v42; // [rsp+68h] [rbp-C8h]
  unsigned long long v43; // [rsp+80h] [rbp-B0h]
  char *object_base; // [rsp+B8h] [rbp-78h]
  struct obstack v45; // [rsp+D0h] [rbp-60h] BYREF
  unsigned long long v46; // [rsp+128h] [rbp-8h]

  v46 = __readfsqword(0x28u);
  if ( a1 == 33554436 )
  {
    obstack_begin(&v45, 0, 0, sub_60B5B, &free);
    s = gettext("Valid arguments for the --quoting-style option are:");
    n = strlen(s);
    if ( v45.chunk_limit - v45.next_free < n )
      obstack_newchunk(&v45, n);
    memcpy(v45.next_free, s, n);
    v45.next_free += n;
    if ( (char *)(v45.chunk_limit - v45.next_free) < (char *)&dword_0 + 2 )
      obstack_newchunk(&v45, 2);
    memcpy(v45.next_free, "\n\n", 2u);
    v45.next_free += 2;
    sub_34EB6((long long)&v45, "  ");
    sa = gettext("\n*This* tar defaults to:\n");
    v42 = strlen(sa);
    if ( v45.chunk_limit - v45.next_free < v42 )
      obstack_newchunk(&v45, v42);
    memcpy(v45.next_free, sa, v42);
    v45.next_free += v42;
    v38 = (char *)sub_35152();
    v43 = strlen(v38);
    if ( v45.chunk_limit - v45.next_free < v43 )
      obstack_newchunk(&v45, v43);
    memcpy(v45.next_free, v38, v43);
    v45.next_free += v43;
    if ( v45.chunk_limit == v45.next_free )
      obstack_newchunk(&v45, 1);
    v34 = v45.next_free++;
    *v34 = 10;
    if ( v45.chunk_limit == v45.next_free )
      obstack_newchunk(&v45, 1);
    v35 = v45.next_free++;
    *v35 = 0;
    object_base = v45.object_base;
    if ( v45.object_base == v45.next_free )
      *((char *)&v45 + 80) |= 2u;
    v45.next_free = (char *)((long long)&v45.next_free[*(long long *)&v45.alignment_mask] & ~*(long long *)&v45.alignment_mask);
    if ( (char *)(v45.next_free - (char *)v45.chunk) > (char *)(v45.chunk_limit - (char *)v45.chunk) )
      v45.next_free = v45.chunk_limit;
    v45.object_base = v45.next_free;
    a2 = sub_60CC6(object_base);
    obstack_free(&v45, 0);
  }
  else if ( a1 <= 33554436 )
  {
    if ( a1 == 207 )
    {
      v30 = (unsigned int)gettext("filter the archive through %s");
      return sub_61277(v30, (unsigned int)"zstd", v30, v31, v32, v33);
    }
    else if ( a1 <= 207 )
    {
      if ( a1 == 150 )
      {
        v22 = (unsigned int)gettext("filter the archive through %s");
        return sub_61277(v22, (unsigned int)"lzop", v22, v23, v24, v25);
      }
      else if ( a1 <= 150 )
      {
        switch ( a1 )
        {
          case 149:
            v18 = (unsigned int)gettext("filter the archive through %s");
            return sub_61277(v18, (unsigned int)"lzma", v18, v19, v20, v21);
          case 148:
            v14 = (unsigned int)gettext("filter the archive through %s");
            return sub_61277(v14, (unsigned int)"lzip", v14, v15, v16, v17);
          case 122:
            v6 = (unsigned int)gettext("filter the archive through %s");
            return sub_61277(v6, (unsigned int)"gzip", v6, v7, v8, v9);
          default:
            if ( a1 <= 122 )
            {
              if ( a1 == 106 )
              {
                v2 = (unsigned int)gettext("filter the archive through %s");
                return sub_61277(v2, (unsigned int)"bzip2", v2, v3, v4, v5);
              }
              else if ( a1 <= 106 )
              {
                if ( a1 == 74 )
                {
                  v26 = (unsigned int)gettext("filter the archive through %s");
                  return sub_61277(v26, (unsigned int)"xz", v26, v27, v28, v29);
                }
                else if ( a1 == 90 )
                {
                  v10 = (unsigned int)gettext("filter the archive through %s");
                  return sub_61277(v10, (unsigned int)"compress", v10, v11, v12, v13);
                }
              }
            }
            break;
        }
      }
    }
  }
  return a2;
}



// Function: expand_pax_option @ 0x361aa
long long expand_pax_option(long long a1, const char *a2)
{
  char *v2; // rax
  char *v3; // rax
  char *src; // [rsp+18h] [rbp-188h]
  unsigned char *srca; // [rsp+18h] [rbp-188h]
  size_t v8; // [rsp+20h] [rbp-180h]
  char *object_base; // [rsp+40h] [rbp-160h]
  long long v10; // [rsp+48h] [rbp-158h]
  unsigned long long n; // [rsp+60h] [rbp-140h]
  unsigned long long v12; // [rsp+88h] [rbp-118h]
  void *dest; // [rsp+98h] [rbp-108h]
  char *v14; // [rsp+B8h] [rbp-E8h]
  unsigned long long v15; // [rsp+C8h] [rbp-D8h]
  time_t v16; // [rsp+100h] [rbp-A0h] BYREF
  long long v17; // [rsp+108h] [rbp-98h]
  struct obstack v18; // [rsp+110h] [rbp-90h] BYREF
  char v19[40]; // [rsp+170h] [rbp-30h] BYREF
  unsigned long long v20; // [rsp+198h] [rbp-8h]

  v20 = __readfsqword(0x28u);
  obstack_begin(&v18, 0, 0, sub_60B5B, &free);
  while ( *a2 )
  {
    n = strcspn(a2, ",");
    src = (char *)memchr(a2, 61, n);
    if ( src )
    {
      v12 = src - a2 + 1;
      if ( v18.chunk_limit - v18.next_free < v12 )
        obstack_newchunk(&v18, v12);
      memcpy(v18.next_free, a2, v12);
      v18.next_free += v12;
      v8 = n - (src - a2 + 1);
      for ( srca = (unsigned char *)(src + 1); *srca && ((*__ctype_b_loc())[*srca] & 0x2000) != 0; ++srca )
        --v8;
      if ( *srca == 123 && srca[v8 - 1] == 125 )
      {
        dest = (void *)sub_60B5B(v8);
        memcpy(dest, srca + 1, v8 - 2);
        *((char *)dest + v8 - 2) = 0;
        if ( (unsigned int)sub_3564F(a1, (long long)"--pax-option", (const char *)dest, &v16) )
        {
          if ( v18.chunk_limit - v18.next_free < v8 )
            obstack_newchunk(&v18, v8);
          memcpy(v18.next_free, srca, v8);
          v18.next_free += v8;
        }
        else
        {
          v14 = (char *)sub_29F2E(v16, v17, (long long)v19);
          v15 = strlen(v14);
          if ( v18.chunk_limit - v18.next_free < v15 )
            obstack_newchunk(&v18, v15);
          memcpy(v18.next_free, v14, v15);
          v18.next_free += v15;
        }
        free(dest);
      }
      else
      {
        if ( v18.chunk_limit - v18.next_free < v8 )
          obstack_newchunk(&v18, v8);
        memcpy(v18.next_free, srca, v8);
        v18.next_free += v8;
      }
    }
    else
    {
      if ( v18.chunk_limit - v18.next_free < n )
        obstack_newchunk(&v18, n);
      memcpy(v18.next_free, a2, n);
      v18.next_free += n;
    }
    a2 += n;
    if ( *a2 )
    {
      if ( v18.chunk_limit == v18.next_free )
        obstack_newchunk(&v18, 1);
      v2 = v18.next_free++;
      *v2 = *a2++;
    }
  }
  if ( v18.chunk_limit == v18.next_free )
    obstack_newchunk(&v18, 1);
  v3 = v18.next_free++;
  *v3 = 0;
  object_base = v18.object_base;
  if ( v18.object_base == v18.next_free )
    *((char *)&v18 + 80) |= 2u;
  v18.next_free = (char *)((long long)&v18.next_free[*(long long *)&v18.alignment_mask] & ~*(long long *)&v18.alignment_mask);
  if ( (char *)(v18.next_free - (char *)v18.chunk) > (char *)(v18.chunk_limit - (char *)v18.chunk) )
    v18.next_free = v18.chunk_limit;
  v18.object_base = v18.next_free;
  v10 = sub_60CC6(object_base);
  obstack_free(&v18, 0);
  return v10;
}



// Function: parse_owner_group @ 0x369f0
unsigned long long parse_owner_group(const char *a1, unsigned long long a2, const char **a3)
{
  int v3; // eax
  char *v4; // rbx
  const char *v5; // rax
  unsigned long long v8; // [rsp+20h] [rbp-50h] BYREF
  char v9[8]; // [rsp+28h] [rbp-48h] BYREF
  unsigned long long v10; // [rsp+30h] [rbp-40h] BYREF
  const char *v11; // [rsp+38h] [rbp-38h]
  const char *v12; // [rsp+40h] [rbp-30h]
  char *v13; // [rsp+48h] [rbp-28h]
  const char *v14; // [rsp+50h] [rbp-20h]
  unsigned long long v15; // [rsp+58h] [rbp-18h]

  v15 = __readfsqword(0x28u);
  v8 = -1;
  v11 = 0;
  v12 = 0;
  v13 = strchr(a1, 58);
  if ( v13 )
  {
    v14 = v13 + 1;
    *v13 = 0;
    if ( *a1 )
      v11 = a1;
    if ( v14 )
    {
      a1 = v14;
      if ( (unsigned int)sub_60E19(v14, v9, 10, &v8, &locale) || a2 < v8 )
        v12 = v14;
    }
  }
  else
  {
    if ( *a1 <= 47 || *a1 > 57 )
      v3 = 4;
    else
      v3 = sub_60E19(a1, v9, 10, &v10, &locale);
    if ( v3 )
    {
      if ( v3 != 1 )
      {
        v11 = a1;
        goto LABEL_18;
      }
    }
    else if ( a2 >= v10 )
    {
      v8 = v10;
      goto LABEL_18;
    }
    v12 = a1;
  }
LABEL_18:
  if ( v12 )
  {
    if ( qword_9FDC8 )
      qword_9FDC8(a1);
    v4 = gettext("Invalid owner or group ID");
    v5 = (const char *)sub_5E548(v12);
    error(0, 0, "%s: %s", v5, v4);
    sub_177F4();
  }
  if ( v11 )
    *a3 = v11;
  return v8;
}



// Function: set_old_files_option @ 0x36bbb
long long set_old_files_option(unsigned int a1, long long a2)
{
  long long v3; // [rsp+18h] [rbp-8h]

  v3 = sub_3521D(9u, a2);
  if ( v3 && sub_35388(a2, v3) && a1 != dword_9F8D8 )
    sub_351B6((long long)off_9AA20[a1], (long long)off_9AA20[dword_9F8D8]);
  dword_9F8D8 = a1;
  return a1;
}



// Function: parse_opt @ 0x36c56
long long parse_opt(long long a1, char *a2, long long *a3, double a4)
{
  char *v4; // rbx
  const char *v5; // rax
  long long v6; // rax
  unsigned int v7; // eax
  int v8; // ecx
  int v9; // r8d
  int v10; // r9d
  char *v11; // rbx
  const char *v12; // rax
  double v13; // xmm0_8
  double v14; // xmm0_8
  char *v15; // rax
  char *v16; // rax
  const char *v17; // rax
  char *v18; // rax
  char *v19; // rdi
  char *v20; // rax
  int v21; // eax
  char *v22; // rax
  char *v23; // rax
  char *v24; // rbx
  const char *v25; // rax
  char *v26; // rbx
  const char *v27; // rax
  char *v28; // rax
  char *v29; // rbx
  const char *v30; // rax
  char *v31; // rax
  long long v32; // r12
  long long v33; // rbx
  char *v34; // rax
  long long v35; // rbx
  char *v36; // rax
  int v38; // [rsp+8h] [rbp-78h]
  char *nptr; // [rsp+10h] [rbp-70h]
  int i; // [rsp+2Ch] [rbp-54h]
  long long v41; // [rsp+30h] [rbp-50h] BYREF
  char *endptr; // [rsp+38h] [rbp-48h] BYREF
  long long v43; // [rsp+40h] [rbp-40h]
  void *v44; // [rsp+48h] [rbp-38h]
  void *ptr; // [rsp+50h] [rbp-30h]
  unsigned long long v46; // [rsp+58h] [rbp-28h]
  unsigned long long v47; // [rsp+60h] [rbp-20h]
  unsigned long long v48; // [rsp+68h] [rbp-18h]

  nptr = a2;
  v38 = (int)a3;
  v48 = __readfsqword(0x28u);
  v43 = a3[5];
  if ( (int)a1 == 16777221 )
  {
    if ( **(int **)v43 == 2 )
    {
      v32 = *(long long *)(*(long long *)v43 + 16LL);
      v33 = *(long long *)(*(long long *)v43 + 8LL);
      v34 = gettext("%s:%lu: location of the error");
      error(0, 0, v34, v33, v32);
    }
    else if ( !**(int **)v43 )
    {
      v35 = *(long long *)(*(long long *)v43 + 8LL);
      v36 = gettext("error parsing %s");
      error(0, 0, v36, v35);
    }
    exit(64);
  }
  if ( (int)a1 > 16777221 )
    return 7;
  if ( (int)a1 <= 207 )
  {
    if ( (int)a1 >= 0 )
    {
      switch ( (int)a1 )
      {
        case 0:
          sub_2C829((long long)a2);
          return 0;
        case 48:
        case 49:
        case 50:
        case 51:
        case 52:
        case 53:
        case 54:
        case 55:
          v7 = (unsigned int)gettext("Options '-[0-7][lmh]' not supported by *this* tar");
          argp_error(v38, v7, v7, v8, v9, v10);
          exit(64);
        case 65:
          sub_353EA(2);
          return 0;
        case 66:
          byte_9F94D = 1;
          return 0;
        case 70:
          qword_9F8C0 = (long long)a2;
          byte_9F8FC = 1;
          return 0;
        case 71:
          goto LABEL_34;
        case 72:
          sub_34C7E(a2);
          return 0;
        case 73:
          sub_3545F(a2, *(long long *)v43);
          return 0;
        case 74:
          sub_3545F("xz", *(long long *)v43);
          return 0;
        case 75:
          sub_3521D(5u, *(long long *)v43);
          sub_2D6A2(a2);
          return 0;
        case 76:
          if ( (unsigned int)sub_60E19(a2, &endptr, 10, &v41, "bBcGgkKMmPTtw") )
          {
            if ( qword_9FDC8 )
              qword_9FDC8(a2);
            v11 = gettext("Invalid tape length");
            v12 = (const char *)sub_5E548(a2);
            error(0, 0, "%s: %s", v12, v11);
            sub_3814C(2);
          }
          if ( a2 >= endptr || strchr("bBcGgkKMmPTtw", *(endptr - 1)) )
          {
            if ( v41 < 0 )
              v14 = (double)(int)(v41 & 1 | ((unsigned long long)v41 >> 1))
                  + (double)(int)(v41 & 1 | ((unsigned long long)v41 >> 1));
            else
              v14 = (double)(int)v41;
            qword_9F990 = *(long long *)&v14;
          }
          else
          {
            if ( v41 < 0 )
              v13 = (double)(int)(v41 & 1 | ((unsigned long long)v41 >> 1))
                  + (double)(int)(v41 & 1 | ((unsigned long long)v41 >> 1));
            else
              v13 = (double)(int)v41;
            *(double *)&qword_9F990 = v13 * 1024.0;
          }
          byte_9F8FC = 1;
          return 0;
        case 77:
          byte_9F8FC = 1;
          return 0;
        case 78:
          dword_9F87C = 1;
          goto LABEL_74;
        case 79:
          byte_9F998 = 1;
          return 0;
        case 80:
          sub_3521D(8u, *(long long *)v43);
          byte_9F878 = 1;
          return 0;
        case 82:
          byte_9F88C = 1;
          return 0;
        case 83:
          byte_9F979 = 1;
          return 0;
        case 85:
          sub_36BBB(3u, *(long long *)v43);
          return 0;
        case 86:
          name = a2;
          return 0;
        case 87:
          sub_3521D(4u, *(long long *)v43);
          byte_9F9B0 = 1;
          return 0;
        case 90:
          sub_3545F("compress", *(long long *)v43);
          return 0;
        case 97:
          *(char *)(v43 + 18) = 1;
          return 0;
        case 98:
          if ( (unsigned int)sub_60E19(a2, 0, 10, &endptr, &locale)
            || (dword_9F868 = (int)endptr, (char *)(int)endptr != endptr)
            || dword_9F868 <= 0
            || (qword_9F870 = (long long)endptr << 9, (char *)((unsigned long long)endptr & 0x7FFFFFFFFFFFFFLL) != endptr) )
          {
            if ( qword_9FDC8 )
              qword_9FDC8(a2);
            v4 = gettext("Invalid blocking factor");
            v5 = (const char *)sub_5E548(a2);
            error(0, 0, "%s: %s", v5, v4);
            sub_3814C(2);
          }
          return 0;
        case 99:
          sub_353EA(3);
          return 0;
        case 100:
          sub_353EA(5);
          return 0;
        case 102:
          if ( qword_9FBE0 == qword_9FC38 )
            qword_9FBD8 = (void *)sub_60A5F(qword_9FBD8, &qword_9FC38, 8);
          v6 = qword_9FBE0++;
          *((long long *)qword_9FBD8 + v6) = a2;
          return 0;
        case 103:
          sub_3521D(2u, *(long long *)v43);
          qword_9F8E0 = a2;
          dword_9F87C = 1;
LABEL_34:
          byte_9F8BE = 1;
          return 0;
        case 104:
          byte_9F8A0 = 1;
          return 0;
        case 105:
          byte_9F8BD = 1;
          return 0;
        case 106:
          sub_3545F("bzip2", *(long long *)v43);
          return 0;
        case 107:
          sub_36BBB(4u, *(long long *)v43);
          return 0;
        case 108:
          dword_9FC34 = 1;
          return 0;
        case 109:
          byte_9F99A = 1;
          return 0;
        case 110:
          dword_9FC14 = 1;
          return 0;
        case 111:
          *(char *)(v43 + 16) = 1;
          return 0;
        case 112:
          dword_9F960 = 1;
          return 0;
        case 114:
          sub_353EA(1);
          return 0;
        case 115:
          sub_3521D(6u, *(long long *)v43);
          byte_9F958 = 1;
          return 0;
        case 116:
          sub_353EA(7);
          ++dword_9F9AC;
          return 0;
        case 117:
          sub_353EA(8);
          return 0;
        case 118:
          ++dword_9F9AC;
          dword_9EC84 |= 0x583000u;
          return 0;
        case 119:
          byte_9F8C8 = 1;
          return 0;
        case 120:
          sub_353EA(6);
          return 0;
        case 122:
          sub_3545F("gzip", *(long long *)v43);
          return 0;
        case 128:
          sub_34C7E("posix");
          dword_9F968 = 1;
          return 0;
        case 129:
          if ( a2 )
            v21 = dword_867C8[sub_47C61("--atime-preserve", a2, off_9A950, dword_867C8, 4, off_9EE48)];
          else
            v21 = 1;
          dword_9F880 = v21;
          return 0;
        case 130:
          byte_9F884 = 1;
          if ( a2 )
            *(long long *)(v43 + 32) = a2;
          return 0;
        case 131:
          byte_9F8EC = 1;
          return 0;
        case 132:
          if ( a2 )
          {
            if ( *a2 == 46 )
            {
              sub_F825(".");
              nptr = a2 + 1;
            }
            dword_9F890 = strtoul(nptr, &endptr, 0);
            if ( *endptr )
            {
              if ( qword_9FDC8 )
                qword_9FDC8(nptr);
              v22 = gettext("--checkpoint value is not an integer");
              error(0, 0, v22);
              sub_177F4();
            }
          }
          else
          {
            dword_9F890 = 10;
          }
          return 0;
        case 133:
          sub_F825(a2);
          return 0;
        case 134:
          dword_9F910 = 2;
          return 0;
        case 135:
          byte_9FC31 = 1;
          return 0;
        case 136:
          byte_9F8A1 = 1;
          return 0;
        case 137:
          sub_353EA(4);
          return 0;
        case 138:
          byte_9FDF8 = 1;
          return 0;
        case 139:
          byte_9F87A = 1;
          return 0;
        case 140:
          v47 = sub_369F0(a2, 0xFFFFFFFF, (const char **)&qword_9F8B0);
          if ( v47 == -1 )
          {
            dword_9F8B8 = -1;
            if ( qword_9F8B0 )
              sub_2C261((const char *)qword_9F8B0, &dword_9F8B8);
          }
          else
          {
            dword_9F8B8 = v47;
          }
          return 0;
        case 141:
          sub_2938E(a2, a4);
          return 0;
        case 142:
          byte_9F9A8 = 1;
          return 0;
        case 143:
          byte_9F8BC = 1;
          return 0;
        case 144:
          qword_9FBF0 = a2;
          return 0;
        case 145:
          byte_9F8DC = 1;
          return 0;
        case 146:
          sub_36BBB(6u, *(long long *)v43);
          return 0;
        case 147:
          dword_9F8E8 = strtoul(a2, &endptr, 10);
          if ( *endptr )
          {
            if ( qword_9FDC8 )
              qword_9FDC8(a2);
            v15 = gettext("Invalid incremental level value");
            error(0, 0, v15);
            sub_3814C(2);
          }
          return 0;
        case 148:
          sub_3545F("lzip", *(long long *)v43);
          return 0;
        case 149:
          sub_3545F("lzma", *(long long *)v43);
          return 0;
        case 150:
          sub_3545F("lzop", *(long long *)v43);
          return 0;
        case 151:
          qword_9F8F0 = sub_55658(a2);
          if ( !qword_9F8F0 )
          {
            if ( qword_9FDC8 )
              qword_9FDC8(a2);
            v23 = gettext("Invalid mode given on option");
            error(0, 0, v23);
            sub_177F4();
          }
          dword_9F8F8 = umask(0);
          umask(dword_9F8F8);
          return 0;
        case 152:
          sub_3564F(v43, (long long)"--mtime", a2, &qword_9F920);
          if ( !dword_9F910 )
            dword_9F910 = 1;
          return 0;
        case 153:
LABEL_74:
          if ( qword_9F908 >= 0 )
          {
            if ( qword_9FDC8 )
              qword_9FDC8(a1);
            v16 = gettext("More than one threshold date");
            error(0, 0, v16);
            sub_3814C(2);
          }
          if ( (int)a1 == 153 )
            v17 = "--newer-mtime";
          else
            v17 = "--after-date";
          sub_3564F(v43, (long long)v17, a2, &qword_9F900);
          sub_3521D(3u, *(long long *)v43);
          return 0;
        case 154:
          dword_9F968 = -1;
          return 0;
        case 155:
          *(char *)(v43 + 18) = 0;
          return 0;
        case 156:
          byte_9F8EC = 0;
          return 0;
        case 157:
          byte_9FC31 = 0;
          return 0;
        case 158:
          byte_9F9A8 = 0;
          return 0;
        case 159:
          sub_36BBB(1u, *(long long *)v43);
          return 0;
        case 160:
          while ( *nptr )
            sub_5C9CD(0, (unsigned int)*nptr++, 0);
          return 0;
        case 161:
          dword_9F95C = -1;
          return 0;
        case 162:
          dword_9F960 = -1;
          return 0;
        case 163:
          dword_9FC14 = 0;
          return 0;
        case 164:
          dword_9F964 = -1;
          return 0;
        case 165:
          sub_34D36(0xFFFFFFFF);
          return 0;
        case 166:
          byte_9F934 = 1;
          return 0;
        case 167:
          sub_3521D(1u, *(long long *)v43);
          if ( a2 )
          {
            if ( (unsigned int)sub_60E19(a2, 0, 10, &endptr, &locale) )
            {
              if ( qword_9FDC8 )
                qword_9FDC8(a2);
              v24 = gettext("Invalid number");
              v25 = (const char *)sub_5E548(a2);
              error(0, 0, "%s: %s", v25, v24);
              sub_177F4();
            }
            qword_9F8D0 = (long long)endptr;
          }
          else
          {
            qword_9F8D0 = 1;
          }
          return 0;
        case 168:
          sub_34C7E("v7");
          return 0;
        case 169:
          byte_9F935 = 1;
          return 0;
        case 170:
          sub_3521D(7u, *(long long *)v43);
          byte_9F936 = 1;
          qword_9F938 = a2;
          return 0;
        case 171:
          sub_36BBB(0, *(long long *)v43);
          return 0;
        case 172:
          sub_36BBB(2u, *(long long *)v43);
          return 0;
        case 173:
          v46 = sub_369F0(a2, 0xFFFFFFFF, (const char **)&qword_9F940);
          if ( v46 == -1 )
          {
            dword_9F948 = -1;
            if ( qword_9F940 )
              sub_2C173((const char *)qword_9F940, &dword_9F948);
          }
          else
          {
            dword_9F948 = v46;
          }
          return 0;
        case 174:
          sub_29234(a2, a4);
          return 0;
        case 175:
          ptr = (void *)sub_361AA(v43, a2);
          *(char *)(v43 + 17) = 1;
          sub_1CC78((char *)ptr);
          free(ptr);
          return 0;
        case 176:
          sub_34C7E("posix");
          return 0;
        case 177:
          while ( *nptr )
            sub_5C9CD(0, (unsigned int)*nptr++, 1);
          return 0;
        case 178:
          sub_35084(a2);
          return 0;
        case 179:
          if ( (unsigned int)sub_60E19(a2, 0, 10, &endptr, "bBcGgkKMmPTtw") )
          {
            if ( qword_9FDC8 )
              qword_9FDC8(a2);
            v26 = gettext("Invalid record size");
            v27 = (const char *)sub_5E548(a2);
            error(0, 0, "%s: %s", v27, v26);
            sub_3814C(2);
          }
          qword_9F870 = (long long)endptr;
          if ( ((unsigned short)endptr & 0x1FF) != 0 )
          {
            if ( qword_9FDC8 )
              qword_9FDC8(a2);
            v28 = gettext("Record size must be a multiple of %d.");
            error(0, 0, v28, 512);
            sub_3814C(2);
          }
          dword_9F868 = (unsigned long long)qword_9F870 >> 9;
          return 0;
        case 180:
          byte_9F94C = 1;
          return 0;
        case 181:
          byte_9F94E = 1;
          return 0;
        case 182:
          byte_9F9A9 = 1;
          return 0;
        case 183:
          off_9ECE0 = a2;
          return 0;
        case 184:
          qword_9F950 = (long long)a2;
          return 0;
        case 185:
          dword_9F95C = 1;
          return 0;
        case 186:
          sub_34C7E("posix");
          dword_9F964 = 1;
          return 0;
        case 187:
          v44 = (void *)sub_35152();
          printf("%s\n", (const char *)v44);
          sub_4E014();
          free(v44);
          exit(0);
        case 188:
          byte_9F978 = 1;
          return 0;
        case 189:
          sub_23F6B();
          sub_4E014();
          exit(0);
        case 190:
          byte_9FC30 = 1;
          return 0;
        case 191:
          sub_36BBB(5u, *(long long *)v43);
          return 0;
        case 192:
          dword_9FC2C = dword_9CB40[sub_47C61("--sort", a2, off_9A980, dword_9CB40, 4, off_9EE48)];
          return 0;
        case 193:
          dword_9F984 = dword_86A30[sub_47C61("--hole-detection", a2, off_9A9A0, dword_86A30, 4, off_9EE48)];
          byte_9F979 = 1;
          return 0;
        case 194:
          byte_9F979 = 1;
          dword_9F97C = strtoul(a2, &endptr, 10);
          if ( *endptr )
          {
            if ( *endptr != 46 )
            {
              if ( qword_9FDC8 )
                qword_9FDC8(a2);
              v18 = gettext("Invalid sparse version value");
              error(0, 0, v18);
              sub_3814C(2);
            }
            v19 = endptr + 1;
            dword_9F980 = strtoul(endptr + 1, &endptr, 10);
            if ( *endptr )
            {
              if ( qword_9FDC8 )
                qword_9FDC8(v19);
              v20 = gettext("Invalid sparse version value");
              error(0, 0, v20);
              sub_3814C(2);
            }
          }
          return 0;
        case 195:
          if ( (unsigned int)sub_60E19(a2, 0, 10, &endptr, &locale) )
          {
            if ( qword_9FDC8 )
              qword_9FDC8(a2);
            v29 = gettext("Invalid number of elements");
            v30 = (const char *)sub_5E548(a2);
            error(0, 0, "%s: %s", v30, v29);
            sub_3814C(2);
          }
          qword_9F970 = (long long)endptr;
          return 0;
        case 196:
          byte_9F884 = 1;
          *(long long *)(v43 + 24) = a2;
          return 0;
        case 197:
          sub_353EA(9);
          return 0;
        case 198:
          if ( a2 )
            sub_35629(a2);
          else
            byte_9F999 = 1;
          return 0;
        case 199:
          if ( qword_9F9A0 )
          {
            if ( qword_9FDC8 )
              qword_9FDC8(a1);
            v31 = gettext("Only one --to-command option allowed");
            error(0, 0, v31);
            sub_3814C(2);
          }
          qword_9F9A0 = (long long)a2;
          return 0;
        case 200:
          sub_3AE0B(a2);
          return 0;
        case 201:
          byte_9F879 = 1;
          return 0;
        case 202:
          filename = a2;
          return 0;
        case 203:
          sub_3C9EE(a2);
          return 0;
        case 204:
          sub_34D36(1u);
          return 0;
        case 205:
        case 206:
          sub_34D36(1u);
          sub_3D0E5(a2, (int)a1 == 206);
          return 0;
        case 207:
          sub_3545F("zstd", *(long long *)v43);
          return 0;
        default:
          return 7;
      }
    }
    return 7;
  }
  if ( (int)a1 != 16777219 )
    return 7;
  if ( *(long long *)(*a3 + 32LL) )
  {
    for ( i = 0; *(long long *)(*(long long *)(*a3 + 32LL) + 32LL * i); ++i )
      *(long long *)(8LL * i + a3[6]) = a3[5];
  }
  return 0;
}



// Function: usage @ 0x3814c
void usage(int a1)
{
  argp_help(&off_9EB60, stderr, 4, qword_9FE48);
  sub_4E014();
  exit(a1);
}



// Function: find_argp_option_key @ 0x38192
long long find_argp_option_key(long long a1, int a2)
{
  while ( *(long long *)a1 || *(int *)(a1 + 8) || *(long long *)(a1 + 16) || *(int *)(a1 + 24) || *(long long *)(a1 + 32) )
  {
    if ( a2 == *(int *)(a1 + 8) )
      return a1;
    a1 += 48;
  }
  return 0;
}



// Function: find_argp_option @ 0x381fd
long long find_argp_option(long long *a1, int a2)
{
  long long v3; // [rsp+10h] [rbp-10h]
  long long **i; // [rsp+18h] [rbp-8h]

  v3 = sub_38192(*a1, a2);
  if ( !v3 && a1[4] )
  {
    for ( i = (long long **)a1[4]; *i; i += 4 )
    {
      v3 = sub_38192(**i, a2);
      if ( v3 )
        break;
    }
  }
  return v3;
}



// Function: more_options @ 0x3828e
unsigned long long more_options(unsigned int a1, long long a2, long long a3)
{
  long long v4[2]; // [rsp+20h] [rbp-30h] BYREF
  char v5; // [rsp+30h] [rbp-20h]
  char v6; // [rsp+31h] [rbp-1Fh]
  char v7; // [rsp+32h] [rbp-1Eh]
  long long v8; // [rsp+38h] [rbp-18h]
  long long v9; // [rsp+40h] [rbp-10h]
  unsigned long long v10; // [rsp+48h] [rbp-8h]

  v10 = __readfsqword(0x28u);
  v4[0] = a3;
  v4[1] = 0;
  v5 = 0;
  v6 = 0;
  v7 = 0;
  v8 = 0;
  v9 = 0;
  argp_parse(&off_9C920, a1, a2, 42, 0, v4);
  return v10 - __readfsqword(0x28u);
}



// Function: parse_default_options @ 0x38321
unsigned long long parse_default_options(long long *a1, double a2)
{
  long long v2; // rbx
  char *v3; // rax
  const char *v4; // rbx
  char *v5; // rax
  char v7[4]; // [rsp+1Ch] [rbp-134h] BYREF
  char *v8; // [rsp+20h] [rbp-130h]
  long long v9; // [rsp+28h] [rbp-128h]
  int v10; // [rsp+30h] [rbp-120h] BYREF
  const char *v11; // [rsp+38h] [rbp-118h]
  long long v12; // [rsp+40h] [rbp-110h]
  long long v13; // [rsp+48h] [rbp-108h]
  long long v14; // [rsp+50h] [rbp-100h] BYREF
  long long *v15; // [rsp+58h] [rbp-F8h]
  long long v16; // [rsp+60h] [rbp-F0h]
  unsigned long long v17; // [rsp+138h] [rbp-18h]

  v17 = __readfsqword(0x28u);
  v8 = getenv("TAR_OPTIONS");
  v10 = 0;
  v11 = "TAR_OPTIONS";
  v12 = 0;
  v13 = 0;
  if ( v8 )
  {
    v16 = 1;
    if ( (unsigned int)sub_461A6(v8, &v14, 33558086, a2) )
    {
      if ( qword_9FDC8 )
        qword_9FDC8(v8);
      v2 = sub_4647F(&v14);
      v3 = gettext("cannot split TAR_OPTIONS: %s");
      error(0, 0, v3, v2);
      sub_177F4();
    }
    if ( v14 )
    {
      *v15 = qword_9FE48;
      v9 = *a1;
      *a1 = (long long)&v10;
      if ( (unsigned int)argp_parse(&off_9EB60, (unsigned int)(v16 + v14), v15, 40, v7, a1) )
        abort();
      *a1 = v9;
      if ( (unsigned char)sub_34A5E() )
      {
        if ( qword_9FDC8 )
          qword_9FDC8(&off_9EB60);
        v4 = v11;
        v5 = gettext("non-option arguments in %s");
        error(0, 0, v5, v4);
        sub_3814C(2);
      }
      v14 = 0;
    }
    sub_46399(&v14, a2);
  }
  return v17 - __readfsqword(0x28u);
}



// Function: decode_options @ 0x3854d
unsigned long long decode_options(int a1, long long a2, double a3)
{
  int v3; // eax
  char **v4; // rax
  long long *v5; // rbx
  long long *v6; // rdx
  long long *v7; // rax
  unsigned int v8; // ebx
  char *v9; // rax
  long long *v10; // rdx
  long long *v11; // rax
  const char *v12; // rdi
  char *v13; // rax
  const char *v14; // rax
  char **v15; // rbx
  char *v16; // rax
  char *v17; // rax
  unsigned long long v18; // rax
  size_t v19; // rax
  long long v20; // rbx
  char *v21; // rax
  char *v22; // rax
  char *v23; // rax
  const char *v24; // rax
  char *v25; // rax
  char *v26; // rax
  char *v27; // rax
  char *v28; // rax
  char *v29; // rax
  char *v30; // rax
  char *v31; // rax
  char *v32; // rax
  const char *v33; // rax
  const char *v34; // rax
  char *v35; // rax
  int v36; // edx
  int v37; // eax
  int v38; // eax
  double v39; // xmm0_8
  char *v40; // rax
  char *v41; // rax
  long long v42; // rdi
  char *v43; // rax
  FILE *v44; // rax
  long long *v46; // [rsp+0h] [rbp-100h]
  int v47; // [rsp+Ch] [rbp-F4h]
  int v48; // [rsp+1Ch] [rbp-E4h] BYREF
  int v49; // [rsp+20h] [rbp-E0h]
  int v50; // [rsp+24h] [rbp-DCh]
  unsigned long long v51; // [rsp+28h] [rbp-D8h]
  long long *v52; // [rsp+30h] [rbp-D0h]
  char *i; // [rsp+38h] [rbp-C8h]
  long long *v54; // [rsp+40h] [rbp-C0h]
  long long v55; // [rsp+48h] [rbp-B8h]
  const char *v56; // [rsp+50h] [rbp-B0h]
  long long v57; // [rsp+58h] [rbp-A8h]
  unsigned long long n; // [rsp+60h] [rbp-A0h]
  const char *v59; // [rsp+68h] [rbp-98h]
  long long v60; // [rsp+70h] [rbp-90h]
  void *ptr; // [rsp+78h] [rbp-88h]
  const char *v62; // [rsp+80h] [rbp-80h]
  long long v63; // [rsp+88h] [rbp-78h]
  int v64; // [rsp+90h] [rbp-70h] BYREF
  long long v65; // [rsp+98h] [rbp-68h]
  long long v66; // [rsp+A0h] [rbp-60h]
  long long v67; // [rsp+A8h] [rbp-58h]
  long long v68[2]; // [rsp+B0h] [rbp-50h] BYREF
  char v69; // [rsp+C0h] [rbp-40h]
  char v70; // [rsp+C1h] [rbp-3Fh]
  char v71; // [rsp+C2h] [rbp-3Eh]
  char *v72; // [rsp+C8h] [rbp-38h]
  long long v73; // [rsp+D0h] [rbp-30h]
  char v74[3]; // [rsp+E5h] [rbp-1Bh] BYREF
  unsigned long long v75; // [rsp+E8h] [rbp-18h]

  v47 = a1;
  v46 = (long long *)a2;
  v75 = __readfsqword(0x28u);
  v64 = 1;
  v65 = 0;
  v66 = 0;
  v67 = 0;
  v68[0] = (long long)&v64;
  v68[1] = 0;
  v69 = 0;
  v70 = 0;
  v71 = 0;
  v72 = 0;
  v73 = 0;
  sub_4D302("tar", off_9EBA0);
  v72 = getenv("SIMPLE_BACKUP_SUFFIX");
  byte_9F9C8 = getenv("POSIXLY_CORRECT") != 0;
  dword_9F860 = 0;
  dword_9F864 = 0;
  dword_9F868 = 20;
  qword_9F870 = 10240;
  qword_9F8A8 = sub_4E53E();
  dword_9F984 = 0;
  qword_9F900 = 0x8000000000000000LL;
  qword_9F908 = -1;
  qword_9F920 = 0x8000000000000000LL;
  qword_9F928 = -1;
  dword_9F930 = 8;
  byte_9FC28 = 1;
  dword_9F97C = 1;
  dword_9F980 = 0;
  dword_9FC2C = 0;
  dword_9F948 = -1;
  qword_9F940 = 0;
  dword_9F8B8 = -1;
  qword_9F8B0 = 0;
  byte_9F8EC = 1;
  dword_9F8E8 = -1;
  dword_9FC14 = -1;
  if ( a1 > 1 && **(char **)(a2 + 8) != 45 )
  {
    v74[0] = 45;
    v74[2] = 0;
    v3 = strlen(*(const char **)(a2 + 8));
    v49 = v3 + a1 - 1;
    v54 = (long long *)sub_60B5B(8LL * (v3 + a1));
    v51 = a2 + 8;
    v52 = v54 + 1;
    *v54 = *(long long *)a2;
    v4 = (char **)v51;
    v51 += 8LL;
    for ( i = *v4; *i; ++i )
    {
      v74[1] = *i;
      v5 = v52++;
      *v5 = sub_60CC6(v74);
      v55 = sub_381FD((long long *)&off_9EB60, *i);
      if ( v55 && *(long long *)(v55 + 16) )
      {
        if ( v51 >= 8LL * a1 + a2 )
        {
          if ( qword_9FDC8 )
            qword_9FDC8(&off_9EB60);
          v8 = *i;
          v9 = gettext("Old option '%c' requires an argument.");
          error(0, 0, v9, v8);
          sub_3814C(2);
        }
        v6 = (long long *)v51;
        v51 += 8LL;
        v7 = v52++;
        *v7 = *v6;
      }
    }
    while ( v51 < 8LL * a1 + a2 )
    {
      v10 = (long long *)v51;
      v51 += 8LL;
      v11 = v52++;
      *v11 = *v10;
    }
    *v52 = 0;
    v47 = v49;
    v46 = v54;
  }
  sub_38321(v68, a3);
  v12 = (const char *)&off_9EB60;
  if ( (unsigned int)argp_parse(&off_9EB60, (unsigned int)v47, v46, 8, &v48, v68) )
    exit(2);
  if ( v69 )
  {
    if ( dword_9F860 == 3 )
    {
      v12 = "v7";
      sub_34C7E("v7");
    }
    else
    {
      dword_9F95C = -1;
    }
  }
  while ( v47 > v48 )
  {
    v12 = (const char *)v46[v48];
    sub_2C829((long long)v12);
    ++v48;
  }
  if ( !dword_9F864 )
  {
    if ( v70 )
      dword_9F864 = 4;
    else
      dword_9F864 = 6;
  }
  if ( name && dword_9F860 == 3 || byte_9F8BE || byte_9F8FC || byte_9F979 )
  {
    v12 = (char *)(off_50 + 4);
    sub_34DAD(84);
  }
  if ( qword_9F8D0 )
  {
    if ( (unsigned char)sub_34A5E() != 1 )
    {
      if ( qword_9FDC8 )
        qword_9FDC8(v12);
      v13 = gettext("--occurrence is meaningless without a file list");
      error(0, 0, v13);
      sub_3814C(2);
    }
    if ( (dword_9CB60[dword_9F860] & 0x10) == 0 )
    {
      v12 = (char *)(&dword_0 + 1);
      if ( sub_3534C(1) )
      {
        v14 = sub_34E1C(dword_9F860);
        sub_351B6((long long)"--occurrence", (long long)v14);
      }
      qword_9F8D0 = 0;
    }
  }
  if ( !qword_9FBE0 )
  {
    qword_9FBE0 = 1;
    v15 = (char **)qword_9FBD8;
    v12 = "TAPE";
    *v15 = getenv("TAPE");
    if ( !*(long long *)qword_9FBD8 )
      *(long long *)qword_9FBD8 = "-";
  }
  if ( (unsigned long long)qword_9FBE0 > 1 && byte_9F8FC != 1 )
  {
    if ( qword_9FDC8 )
      qword_9FDC8(v12);
    v16 = gettext("Multiple archive files require '-M' option");
    error(0, 0, v16);
    sub_3814C(2);
  }
  if ( qword_9F8E0 && qword_9F908 >= 0 )
  {
    v56 = (const char *)sub_35327(2);
    v57 = sub_35327(3);
    v12 = v56;
    if ( sub_35388((long long)v56, v57) )
      sub_351B6((long long)"--listed-incremental", (long long)"--newer");
    if ( *(int *)v56 == 1 )
    {
      qword_9F8E0 = 0;
    }
    else
    {
      v12 = (const char *)&qword_9F900;
      memset(&qword_9F900, 0, 0x10u);
    }
  }
  if ( dword_9F8E8 != -1 && !qword_9F8E0 )
  {
    if ( qword_9FDC8 )
      qword_9FDC8(v12);
    v17 = gettext("--level is meaningless without --listed-incremental");
    v12 = 0;
    error(0, 0, v17);
  }
  if ( name && (dword_9F864 == 6 || dword_9F864 == 2) )
  {
    v18 = byte_9F8FC ? 81LL : 99LL;
    n = v18;
    v12 = name;
    v19 = strlen(name);
    if ( n < v19 )
    {
      if ( qword_9FDC8 )
        qword_9FDC8(v12);
      v20 = sub_5E548(name);
      v21 = ngettext(
              "%s: Volume label is too long (limit is %lu byte)",
              "%s: Volume label is too long (limit is %lu bytes)",
              n);
      error(0, 0, v21, v20, n);
      sub_3814C(2);
    }
  }
  if ( byte_9F9B0 )
  {
    if ( byte_9F8FC )
    {
      if ( qword_9FDC8 )
        qword_9FDC8(v12);
      v22 = gettext("Cannot verify multi-volume archives");
      error(0, 0, v22);
      sub_3814C(2);
    }
    if ( qword_9F898 )
    {
      if ( qword_9FDC8 )
        qword_9FDC8(v12);
      v23 = gettext("Cannot verify compressed archives");
      error(0, 0, v23);
      sub_3814C(2);
    }
    if ( (dword_9CB60[dword_9F860] & 2) == 0 )
    {
      v12 = &byte_4;
      if ( sub_3534C(4) )
      {
        v24 = sub_34E1C(dword_9F860);
        sub_351B6((long long)"--verify", (long long)v24);
      }
      byte_9F9B0 = 0;
    }
  }
  if ( qword_9F898 )
  {
    if ( byte_9F8FC )
    {
      if ( qword_9FDC8 )
        qword_9FDC8(v12);
      v25 = gettext("Cannot use multi-volume compressed archives");
      error(0, 0, v25);
      sub_3814C(2);
    }
    if ( (dword_9CB60[dword_9F860] & 4) != 0 )
    {
      if ( qword_9FDC8 )
        qword_9FDC8(v12);
      v26 = gettext("Cannot update compressed archives");
      error(0, 0, v26);
      sub_3814C(2);
    }
    if ( dword_9F860 == 2 )
    {
      if ( qword_9FDC8 )
        qword_9FDC8(v12);
      v27 = gettext("Cannot concatenate compressed archives");
      error(0, 0, v27);
      sub_3814C(2);
    }
  }
  if ( dword_9F910 == 2 && qword_9F928 < 0 )
  {
    if ( qword_9FDC8 )
      qword_9FDC8(v12);
    v28 = gettext("--clamp-mtime needs a date specified using --mtime");
    error(0, 0, v28);
    sub_3814C(2);
  }
  if ( v70 && dword_9F864 != 4 && (dword_9CB60[dword_9F860] & 1) == 0 )
  {
    if ( qword_9FDC8 )
      qword_9FDC8(v12);
    v29 = gettext("--pax-option can be used only on POSIX archives");
    error(0, 0, v29);
    sub_3814C(2);
  }
  if ( dword_9F968 > 0 && dword_9F864 != 4 && (dword_9CB60[dword_9F860] & 1) == 0 )
  {
    if ( qword_9FDC8 )
      qword_9FDC8(v12);
    v30 = gettext("--acls can be used only on POSIX archives");
    error(0, 0, v30);
    sub_3814C(2);
  }
  if ( dword_9F964 > 0 && dword_9F864 != 4 && (dword_9CB60[dword_9F860] & 1) == 0 )
  {
    if ( qword_9FDC8 )
      qword_9FDC8(v12);
    v31 = gettext("--selinux can be used only on POSIX archives");
    error(0, 0, v31);
    sub_3814C(2);
  }
  if ( dword_9F96C > 0 && dword_9F864 != 4 && (dword_9CB60[dword_9F860] & 1) == 0 )
  {
    if ( qword_9FDC8 )
      qword_9FDC8(v12);
    v32 = gettext("--xattrs can be used only on POSIX archives");
    error(0, 0, v32);
    sub_3814C(2);
  }
  if ( byte_9F988 && (dword_9CB60[dword_9F860] & 1) == 0 )
  {
    v12 = &byte_5;
    if ( sub_3534C(5) )
    {
      v33 = sub_34E1C(dword_9F860);
      sub_351B6((long long)"--starting-file", (long long)v33);
    }
    byte_9F988 = 0;
  }
  if ( byte_9F958 && (dword_9CB60[dword_9F860] & 1) == 0 )
  {
    v12 = &byte_6;
    if ( sub_3534C(6) )
    {
      v34 = sub_34E1C(dword_9F860);
      sub_351B6((long long)"--same-order", (long long)v34);
    }
    byte_9F958 = 0;
  }
  if ( byte_9F936 )
  {
    if ( byte_9F878 )
    {
      v59 = (const char *)sub_35327(7);
      v60 = sub_35327(8);
      v12 = v59;
      if ( sub_35388((long long)v59, v60) )
        sub_351B6((long long)"--one-top-level", (long long)"--absolute-names");
      if ( *(int *)v59 == 1 )
        byte_9F878 = 0;
      else
        byte_9F936 = 0;
    }
    if ( byte_9F936 )
    {
      if ( !qword_9F938 )
      {
        ptr = (void *)sub_4E13E(*(long long *)qword_9FBD8);
        qword_9F938 = (char *)sub_329B0((const char *)ptr);
        v12 = (const char *)ptr;
        free(ptr);
        if ( !qword_9F938 )
        {
          if ( qword_9FDC8 )
            qword_9FDC8(v12);
          v35 = gettext("Cannot deduce top-level directory name; please set it explicitly with --one-top-level=DIR");
          error(0, 0, v35);
          sub_3814C(2);
        }
      }
    }
  }
  if ( byte_9F94C )
    dword_9F8D8 = 3;
  if ( byte_9F8A0 )
    v36 = 526592;
  else
    v36 = 657664;
  if ( dword_9F880 == 2 )
    v37 = 0x40000;
  else
    v37 = 0;
  v50 = v36 | v37;
  oflag = v36 | v37;
  dword_9FC0C = v36 | v37 | 0x10000;
  if ( byte_9F8A0 )
    v38 = 0;
  else
    v38 = 256;
  flag = v38;
  if ( dword_9F860 == 9 )
  {
    if ( (unsigned char)sub_34A5E() != 1 )
      ++dword_9F9AC;
  }
  else if ( byte_9F879 )
  {
    dword_9F9AC = 2;
  }
  if ( *(double *)&qword_9F990 != 0.0 )
  {
    v39 = qword_9F870 < 0
        ? (double)(int)(qword_9F870 & 1 | ((unsigned long long)qword_9F870 >> 1))
        + (double)(int)(qword_9F870 & 1 | ((unsigned long long)qword_9F870 >> 1))
        : (double)(int)qword_9F870;
    if ( v39 > *(double *)&qword_9F990 )
    {
      if ( qword_9FDC8 )
        qword_9FDC8(v12);
      v40 = gettext("Volume length cannot be less than record size");
      error(0, 0, v40);
      sub_3814C(2);
    }
  }
  if ( byte_9F958 && qword_9F8E0 )
  {
    v62 = (const char *)sub_35327(6);
    v63 = sub_35327(2);
    v12 = v62;
    if ( sub_35388((long long)v62, v63) )
      sub_351B6((long long)"--preserve-order", (long long)"--listed-incremental");
    if ( *(int *)v62 == 1 )
      qword_9F8E0 = 0;
    else
      byte_9F958 = 0;
  }
  switch ( dword_9F860 )
  {
    case 1:
    case 2:
    case 8:
      for ( qword_9FBE8 = (long long)qword_9FBD8;
            (unsigned long long)qword_9FBD8 + 8 * qword_9FBE0 > qword_9FBE8;
            qword_9FBE8 += 8 )
      {
        v42 = *(long long *)qword_9FBE8;
        if ( !strcmp(*(const char **)qword_9FBE8, "-") )
        {
          if ( qword_9FDC8 )
            qword_9FDC8(v42);
          v43 = gettext("Options '-Aru' are incompatible with '-f -'");
          error(0, 0, v43);
          sub_3814C(2);
        }
      }
      break;
    case 3:
      if ( (unsigned char)sub_34A5E() != 1 )
      {
        if ( qword_9FDC8 )
          qword_9FDC8(v12);
        v41 = gettext("Cowardly refusing to create an empty archive");
        error(0, 0, v41);
        sub_3814C(2);
      }
      if ( v71 && qword_9FBE0 && strcmp(*(const char **)qword_9FBD8, "-") )
        sub_32970(*(const char **)qword_9FBD8, (long long)qword_9F898);
      break;
    case 5:
    case 6:
    case 7:
    case 9:
      for ( qword_9FBE8 = (long long)qword_9FBD8;
            (unsigned long long)qword_9FBD8 + 8 * qword_9FBE0 > qword_9FBE8;
            qword_9FBE8 += 8 )
      {
        if ( !strcmp(*(const char **)qword_9FBE8, "-") )
          sub_34A6F((long long)"-f");
      }
      break;
    default:
      break;
  }
  if ( qword_9FBF0 )
  {
    stream = fopen(qword_9FBF0, "w");
    if ( !stream )
      sub_3E3B4(qword_9FBF0);
  }
  else
  {
    if ( byte_9F998 )
      v44 = stderr;
    else
      v44 = stdout;
    stream = v44;
  }
  qword_9FBE8 = (long long)qword_9FBD8;
  if ( v72 )
    qword_9FE30 = (char *)sub_60CC6(v72);
  if ( byte_9F884 )
  {
    dword_9F888 = sub_4DEC2("--backup", v73);
    if ( !dword_9F888 || byte_9F998 || qword_9F9A0 )
      byte_9F884 = 0;
  }
  sub_FB72();
  sub_3589E((long long)v68);
  return v75 - __readfsqword(0x28u);
}



// Function: main @ 0x39948
long long main(long long a1, char **a2, char **a3)
{
  char *v3; // rdi
  char *v4; // rax
  double v5; // xmm0_8
  char *v6; // rax
  char *v7; // rax
  unsigned int v9; // [rsp+Ch] [rbp-4h]

  v9 = a1;
  sub_ADC5(a1, a2, a3);
  sub_5C844(*a2);
  setlocale(6, &locale);
  bindtextdomain("tar", "/usr/local/share/locale");
  textdomain("tar");
  status = 2;
  dword_9FDD0 = 0;
  qword_9FDC8 = (long long (*)(long long))sub_10622;
  sub_5C9A2(0, 7);
  v3 = gettext("stdout");
  sub_4DFE0(v3);
  if ( (unsigned int)sub_5FE52() )
  {
    if ( qword_9FDC8 )
      qword_9FDC8(v3);
    v4 = gettext("failed to assert availability of the standard file descriptors");
    error(0, 0, "%s", v4);
    sub_177F4();
  }
  qword_9FC38 = 10;
  qword_9FBD8 = (void *)sub_60B5B(80);
  qword_9FBE0 = 0;
  signal(17, 0);
  v5 = sub_5C826();
  sub_3854D(v9, (long long)a2, v5);
  sub_2C94C();
  if ( filename )
    sub_D1C6();
  switch ( dword_9F860 )
  {
    case 0:
      if ( qword_9FDC8 )
        qword_9FDC8(v9);
      v6 = gettext("You must specify one of the '-Acdtrux', '--delete' or '--test-label' options");
      error(0, 0, v6);
      sub_3814C(2);
    case 1:
    case 2:
    case 8:
      sub_3C325();
      break;
    case 3:
      sub_14E23();
      break;
    case 4:
      sub_17069();
      break;
    case 5:
      sub_1075E();
      sub_257B8((void (*)(void))sub_11897);
      break;
    case 6:
      sub_182FD();
      sub_257B8((void (*)(void))sub_1BBCB);
      sub_1C2EE();
      break;
    case 7:
      sub_257B8((void (*)(void))sub_25CF3);
      break;
    case 9:
      sub_28913();
      break;
    default:
      break;
  }
  sub_10729();
  if ( byte_9F999 )
    sub_BAA2(v5);
  if ( dword_9FC34 )
    sub_1573F();
  if ( filename )
    sub_D2E3();
  free(qword_9FBD8);
  sub_3D17C();
  sub_2C978();
  if ( dword_9FDD0 == 2 )
  {
    v7 = gettext("Exiting with failure status due to previous errors");
    error(0, 0, v7);
  }
  if ( stream == stdout )
  {
    sub_4E014();
  }
  else if ( ferror_unlocked(stderr) || fclose(stderr) )
  {
    sub_39E98(2);
  }
  return (unsigned int)dword_9FDD0;
}



// Function: tar_stat_init @ 0x39c55
void *tar_stat_init(void *a1)
{
  return memset(a1, 0, 0x1B8u);
}



// Function: tar_stat_close @ 0x39c7e
long long tar_stat_close(long long a1)
{
  int v1; // eax

  if ( *(long long *)(a1 + 416) )
  {
    v1 = closedir(*(DIR **)(a1 + 416));
  }
  else if ( *(int *)(a1 + 424) <= 0 )
  {
    v1 = 0;
  }
  else
  {
    v1 = close(*(int *)(a1 + 424));
  }
  *(long long *)(a1 + 416) = 0;
  *(int *)(a1 + 424) = 0;
  if ( !v1 )
    return 1;
  sub_2B26A(*(long long *)a1);
  return 0;
}



// Function: tar_stat_destroy @ 0x39d1c
void *tar_stat_destroy(long long a1)
{
  sub_39C7E(a1);
  sub_1D55D(*(void ***)(a1 + 352), *(long long *)(a1 + 344));
  free(*(void **)a1);
  free(*(void **)(a1 + 8));
  free(*(void **)(a1 + 24));
  free(*(void **)(a1 + 32));
  free(*(void **)(a1 + 40));
  free(*(void **)(a1 + 48));
  free(*(void **)(a1 + 56));
  free(*(void **)(a1 + 72));
  free(*(void **)(a1 + 320));
  free(*(void **)(a1 + 400));
  sub_1E785(a1 + 360);
  sub_17BDA(a1);
  return memset((void *)a1, 0, 0x1B8u);
}



// Function: tar_timespec_cmp @ 0x39e35
long long tar_timespec_cmp(long long a1, long long a2, long long a3, long long a4)
{
  if ( dword_9F468 != 4 )
  {
    a4 = 0;
    a2 = 0;
  }
  return sub_5FF90(a1, a2, a3, a4);
}



// Function: set_exit_status @ 0x39e98
long long set_exit_status(int a1)
{
  long long result; // rax

  result = (unsigned int)dword_9FDD0;
  if ( a1 > dword_9FDD0 )
  {
    dword_9FDD0 = a1;
    return (unsigned int)a1;
  }
  return result;
}



// Function: new_transform @ 0x39eba
long long new_transform()
{
  long long v1; // [rsp+8h] [rbp-8h]

  v1 = sub_60C06(112);
  if ( qword_9FCC8 )
    *(long long *)qword_9FCC8 = v1;
  else
    qword_9FCC0 = v1;
  qword_9FCC8 = v1;
  return v1;
}



// Function: add_segment @ 0x39f0c
long long *add_segment(long long a1)
{
  long long *v2; // [rsp+18h] [rbp-8h]

  v2 = (long long *)sub_60B5B(32);
  *v2 = 0;
  if ( *(long long *)(a1 + 96) )
    **(long long **)(a1 + 96) = v2;
  else
    *(long long *)(a1 + 88) = v2;
  *(long long *)(a1 + 96) = v2;
  ++*(long long *)(a1 + 104);
  return v2;
}



// Function: add_literal_segment @ 0x39f85
long long *add_literal_segment(long long a1, char *a2, char *a3)
{
  long long *result; // rax
  size_t n; // [rsp+20h] [rbp-10h]
  long long *v5; // [rsp+28h] [rbp-8h]

  result = (long long *)(a3 - a2);
  n = a3 - a2;
  if ( a3 != a2 )
  {
    v5 = sub_39F0C(a1);
    *((int *)v5 + 2) = 0;
    v5[2] = sub_60B5B(n + 1);
    memcpy((void *)v5[2], a2, n);
    *(char *)(v5[2] + n) = 0;
    v5[3] = n;
    return v5;
  }
  return result;
}



// Function: add_char_segment @ 0x3a022
long long *add_char_segment(long long a1, unsigned char a2)
{
  long long *v3; // [rsp+18h] [rbp-8h]

  v3 = sub_39F0C(a1);
  *((int *)v3 + 2) = 0;
  v3[2] = sub_60B5B(2);
  *(short *)v3[2] = a2;
  v3[3] = 1;
  return v3;
}



// Function: add_backref_segment @ 0x3a090
long long *add_backref_segment(long long a1, long long a2)
{
  long long *result; // rax

  result = sub_39F0C(a1);
  *((int *)result + 2) = 1;
  result[2] = a2;
  return result;
}



// Function: parse_xform_flags @ 0x3a0ce
long long parse_xform_flags(int *a1, int a2)
{
  long long result; // rax

  switch ( a2 )
  {
    case 'H':
      *a1 &= ~2u;
      goto LABEL_9;
    case 'R':
      *a1 &= ~1u;
      goto LABEL_9;
    case 'S':
      *a1 &= ~4u;
      goto LABEL_9;
    case 'h':
      *a1 |= 2u;
      goto LABEL_9;
    case 'r':
      *a1 |= 1u;
      goto LABEL_9;
    case 's':
      *a1 |= 4u;
LABEL_9:
      result = 0;
      break;
    default:
      result = 1;
      break;
  }
  return result;
}



// Function: add_case_ctl_segment @ 0x3a18f
long long *add_case_ctl_segment(long long a1, int a2)
{
  long long *result; // rax

  result = sub_39F0C(a1);
  *((int *)result + 2) = 2;
  *((int *)result + 4) = a2;
  return result;
}



// Function: parse_transform_expr @ 0x3a1ca
char *parse_transform_expr(long long a1)
{
  unsigned int v1; // ebx
  char *v2; // rax
  char *v4; // rax
  char *v5; // rax
  char *v6; // rax
  char *v7; // rax
  int v8; // eax
  int v9; // eax
  long long v10; // rdi
  unsigned int v11; // ebx
  char *v12; // rax
  char *v13; // rax
  char *v14; // rdi
  char *v15; // rax
  char *s1a; // [rsp+8h] [rbp-278h]
  int i; // [rsp+1Ch] [rbp-264h]
  int v19; // [rsp+1Ch] [rbp-264h]
  int j; // [rsp+20h] [rbp-260h]
  unsigned int v21; // [rsp+24h] [rbp-25Ch]
  int v22; // [rsp+28h] [rbp-258h]
  unsigned int v23; // [rsp+2Ch] [rbp-254h]
  char *endptr; // [rsp+30h] [rbp-250h] BYREF
  char *nptr; // [rsp+38h] [rbp-248h] BYREF
  char *v26; // [rsp+40h] [rbp-240h]
  long long v27; // [rsp+48h] [rbp-238h]
  void *dest; // [rsp+50h] [rbp-230h]
  unsigned long long v29; // [rsp+58h] [rbp-228h]
  char v30[2]; // [rsp+60h] [rbp-220h] BYREF
  char v31[518]; // [rsp+62h] [rbp-21Eh] BYREF
  unsigned long long v32; // [rsp+268h] [rbp-18h]

  v32 = __readfsqword(0x28u);
  v21 = 0;
  v27 = sub_39EBA();
  if ( *(char *)a1 == 115 )
  {
    v22 = *(char *)(a1 + 1);
    if ( !*(char *)(a1 + 1) )
    {
      if ( qword_9FDC8 )
        qword_9FDC8(a1);
      v5 = gettext("Invalid transform expression");
      error(0, 0, v5);
      sub_3814C(2);
    }
    for ( i = 2; *(char *)(i + a1) && v22 != *(char *)(i + a1); ++i )
    {
      if ( *(char *)(i + a1) == 92 && *(char *)(i + 1LL + a1) )
        ++i;
    }
    if ( v22 != *(char *)(i + a1) )
    {
      if ( qword_9FDC8 )
        qword_9FDC8(a1);
      v6 = gettext("Invalid transform expression");
      error(0, 0, v6);
      sub_3814C(2);
    }
    for ( j = i + 1; *(char *)(j + a1) && v22 != *(char *)(j + a1); ++j )
    {
      if ( *(char *)(j + a1) == 92 && *(char *)(j + 1LL + a1) )
        ++j;
    }
    if ( v22 != *(char *)(j + a1) )
    {
      if ( qword_9FDC8 )
        qword_9FDC8(a1);
      v7 = gettext("Invalid transform expression");
      error(0, 0, v7);
      sub_3814C(2);
    }
    *(int *)(v27 + 8) = 0;
    *(int *)(v27 + 12) = dword_9EC00;
    for ( nptr = (char *)(j + 1LL + a1); ; ++nptr )
    {
      if ( !*nptr || *nptr == 59 )
      {
        if ( *nptr == 59 )
          ++nptr;
        dest = (void *)sub_60B5B(i - 1);
        memcpy(dest, (const void *)(a1 + 2), i - 2);
        *((char *)dest + i - 2) = 0;
        v23 = sub_67CCD(v27 + 24, dest, v21);
        if ( v23 )
        {
          sub_67E43(v23, v27 + 24, v30, 512);
          if ( qword_9FDC8 )
            qword_9FDC8(v23);
          v13 = gettext("Invalid transform expression: %s");
          error(0, 0, v13, v30);
          sub_3814C(2);
        }
        if ( *(char *)dest == 94 || i > 2 && *((char *)dest + i - 3) == 36 )
          *(int *)(v27 + 8) = 0;
        free(dest);
        v19 = i + 1;
        dest = (void *)sub_60B5B(j - v19 + 1);
        memcpy(dest, (const void *)(a1 + v19), j - v19);
        *((char *)dest + j - v19) = 0;
        v26 = (char *)dest;
        endptr = (char *)dest;
        while ( *endptr )
        {
          if ( *endptr == 92 )
          {
            sub_39F85(v27, v26, endptr++);
            switch ( *endptr )
            {
              case '&':
                sub_3A022(v27, 0x26u);
                ++endptr;
                break;
              case '0':
              case '1':
              case '2':
              case '3':
              case '4':
              case '5':
              case '6':
              case '7':
              case '8':
              case '9':
                v14 = endptr;
                v29 = strtoul(endptr, &endptr, 10);
                if ( v29 > *(long long *)(v27 + 72) )
                {
                  if ( qword_9FDC8 )
                    qword_9FDC8(v14);
                  v15 = gettext("Invalid transform replacement: back reference out of range");
                  error(0, 0, v15);
                  sub_3814C(2);
                }
                sub_3A090(v27, v29);
                break;
              case 'E':
                sub_3A18F(v27, 0);
                ++endptr;
                break;
              case 'L':
                sub_3A18F(v27, 4);
                ++endptr;
                break;
              case 'U':
                sub_3A18F(v27, 3);
                ++endptr;
                break;
              case '\\':
                sub_3A022(v27, 0x5Cu);
                ++endptr;
                break;
              case 'a':
                sub_3A022(v27, 7u);
                ++endptr;
                break;
              case 'b':
                sub_3A022(v27, 8u);
                ++endptr;
                break;
              case 'f':
                sub_3A022(v27, 0xCu);
                ++endptr;
                break;
              case 'l':
                sub_3A18F(v27, 2);
                ++endptr;
                break;
              case 'n':
                sub_3A022(v27, 0xAu);
                ++endptr;
                break;
              case 'r':
                sub_3A022(v27, 0xDu);
                ++endptr;
                break;
              case 't':
                sub_3A022(v27, 9u);
                ++endptr;
                break;
              case 'u':
                sub_3A18F(v27, 1);
                ++endptr;
                break;
              case 'v':
                sub_3A022(v27, 0xBu);
                ++endptr;
                break;
              default:
                if ( v22 == *endptr )
                {
                  sub_3A022(v27, v22);
                }
                else
                {
                  v30[0] = 92;
                  v30[1] = *endptr;
                  sub_39F85(v27, v30, v31);
                }
                ++endptr;
                break;
            }
            v26 = endptr;
          }
          else if ( *endptr == 38 )
          {
            sub_39F85(v27, v26, endptr);
            sub_3A090(v27, 0);
            v26 = ++endptr;
          }
          else
          {
            ++endptr;
          }
        }
        sub_39F85(v27, v26, endptr);
        free(dest);
        return nptr;
      }
      v8 = *nptr;
      if ( v8 == 120 )
      {
        v21 |= 1u;
        continue;
      }
      if ( v8 <= 120 )
      {
        if ( v8 == 105 )
        {
          v21 |= 2u;
          continue;
        }
        if ( v8 <= 105 )
        {
          if ( v8 > 57 )
          {
            if ( v8 == 103 )
            {
              *(int *)(v27 + 8) = 1;
              continue;
            }
          }
          else if ( v8 >= 48 )
          {
            v9 = strtoul(nptr, &nptr, 0);
            *(int *)(v27 + 16) = v9;
            --nptr;
            continue;
          }
        }
      }
      v10 = v27 + 12;
      if ( (unsigned int)sub_3A0CE((int *)(v27 + 12), *nptr) )
      {
        if ( qword_9FDC8 )
          qword_9FDC8(v10);
        v11 = *nptr;
        v12 = gettext("Unknown flag in transform expression: %c");
        error(0, 0, v12, v11);
        sub_3814C(2);
      }
    }
  }
  if ( strncmp((const char *)a1, "flags=", 6u) )
  {
    if ( qword_9FDC8 )
      qword_9FDC8(a1);
    v4 = gettext("Invalid transform expression");
    error(0, 0, v4);
    sub_3814C(2);
  }
  dword_9EC00 = 0;
  for ( s1a = (char *)(a1 + 6); *s1a; ++s1a )
  {
    if ( *s1a == 59 )
      return ++s1a;
    if ( (unsigned int)sub_3A0CE(&dword_9EC00, *s1a) )
    {
      if ( qword_9FDC8 )
        qword_9FDC8(&dword_9EC00);
      v1 = *s1a;
      v2 = gettext("Unknown transform flag: %c");
      error(0, 0, v2, v1);
      sub_3814C(2);
    }
  }
  return s1a;
}



// Function: set_transform_expr @ 0x3ae0b
long long set_transform_expr(char *a1)
{
  long long result; // rax

  while ( 1 )
  {
    result = (unsigned char)*a1;
    if ( !(char)result )
      break;
    a1 = sub_3A1CA((long long)a1);
  }
  return result;
}



// Function: run_case_conv @ 0x3ae3c
void *run_case_conv(int a1, const void *a2, size_t a3)
{
  char *i; // [rsp+28h] [rbp-8h]
  char *j; // [rsp+28h] [rbp-8h]

  if ( a3 > qword_9FD40 )
  {
    qword_9FD40 = a3;
    qword_9FCD0 = (void *)sub_60B8D(qword_9FCD0, a3);
  }
  memcpy(qword_9FCD0, a2, a3);
  switch ( a1 )
  {
    case 1:
      *(char *)qword_9FCD0 = toupper(*(unsigned char *)qword_9FCD0);
      break;
    case 2:
      *(char *)qword_9FCD0 = tolower(*(unsigned char *)qword_9FCD0);
      break;
    case 3:
      for ( i = qword_9FCD0; i < (char *)qword_9FCD0 + a3; ++i )
        *i = toupper((unsigned char)*i);
      break;
    case 4:
      for ( j = qword_9FCD0; j < (char *)qword_9FCD0 + a3; ++j )
        *j = tolower((unsigned char)*j);
      break;
    default:
      return qword_9FCD0;
  }
  return qword_9FCD0;
}



// Function: _single_transform_name_to_obstack @ 0x3afa7
void _single_transform_name_to_obstack(long long a1, char *a2)
{
  unsigned int v2; // eax
  unsigned int v3; // eax
  char *v4; // rax
  int v6; // [rsp+14h] [rbp-DCh]
  int v7; // [rsp+18h] [rbp-D8h]
  unsigned long long v8; // [rsp+20h] [rbp-D0h]
  size_t v9; // [rsp+28h] [rbp-C8h]
  const void *v10; // [rsp+30h] [rbp-C0h]
  char *v11; // [rsp+30h] [rbp-C0h]
  long long *i; // [rsp+38h] [rbp-B8h]
  size_t *ptr; // [rsp+40h] [rbp-B0h]
  size_t n; // [rsp+68h] [rbp-88h]
  size_t v15; // [rsp+90h] [rbp-60h]
  size_t v16; // [rsp+B8h] [rbp-38h]
  size_t v17; // [rsp+D0h] [rbp-20h]

  v8 = 0;
  v6 = 0;
  v7 = 0;
  ptr = (size_t *)sub_60B5B(16 * (*(long long *)(a1 + 72) + 1LL));
  do
  {
    while ( 1 )
    {
      if ( !*a2 )
        goto LABEL_52;
      if ( (unsigned int)sub_6F294(a1 + 24, a2, *(long long *)(a1 + 72) + 1LL, ptr, 0) )
        break;
      v9 = ptr[1];
      if ( *ptr )
      {
        n = *ptr;
        if ( unk_9FD00 - unk_9FCF8 < *ptr )
          obstack_newchunk(&unk_9FCE0, n);
        memcpy(unk_9FCF8, a2, n);
        unk_9FCF8 += n;
      }
      ++v8;
      if ( !*(int *)(a1 + 16) || v8 >= *(unsigned int *)(a1 + 16) )
      {
        for ( i = *(long long **)(a1 + 88); ; i = (long long *)*i )
        {
          if ( !i )
            goto LABEL_47;
          v2 = *((int *)i + 2);
          if ( v2 == 2 )
          {
            v3 = *((int *)i + 4);
            if ( v3 > 4 )
              continue;
            if ( v3 < 3 && v3 )
            {
              if ( v3 - 1 > 1 )
                continue;
              if ( !v7 || (unsigned int)(v7 - 3) <= 1 )
                v7 = v6;
            }
            v6 = *((int *)i + 4);
          }
          else if ( v2 <= 2 )
          {
            if ( v2 )
            {
              if ( ptr[2 * i[2]] != -1 && ptr[2 * i[2] + 1] != -1 )
              {
                v15 = ptr[2 * i[2] + 1] - ptr[2 * i[2]];
                v11 = &a2[ptr[2 * i[2]]];
                if ( v6 )
                {
                  v11 = sub_3AE3C(v6, v11, v15);
                  if ( v6 == 1 || v6 == 2 )
                  {
                    v6 = v7;
                    v7 = 0;
                  }
                }
                if ( unk_9FD00 - unk_9FCF8 < v15 )
                  obstack_newchunk(&unk_9FCE0, v15);
                memcpy(unk_9FCF8, v11, v15);
                unk_9FCF8 += v15;
              }
            }
            else
            {
              if ( v6 )
              {
                v10 = sub_3AE3C(v6, (const void *)i[2], i[3]);
                if ( v6 == 1 || v6 == 2 )
                {
                  v6 = v7;
                  v7 = 0;
                }
              }
              else
              {
                v10 = (const void *)i[2];
              }
              v16 = i[3];
              if ( unk_9FD00 - unk_9FCF8 < v16 )
                obstack_newchunk(&unk_9FCE0, v16);
              memcpy(unk_9FCF8, v10, v16);
              unk_9FCF8 += v16;
            }
          }
        }
      }
      if ( unk_9FD00 - unk_9FCF8 < v9 )
        obstack_newchunk(&unk_9FCE0, v9);
      memcpy(unk_9FCF8, a2, v9);
      unk_9FCF8 += v9;
      a2 += v9;
    }
    v9 = strlen(a2);
    if ( unk_9FD00 - unk_9FCF8 < v9 )
      obstack_newchunk(&unk_9FCE0, v9);
    memcpy(unk_9FCF8, a2, v9);
    unk_9FCF8 += v9;
LABEL_47:
    a2 += v9;
  }
  while ( *(int *)(a1 + 8) );
  v17 = strlen(a2);
  if ( unk_9FD00 - unk_9FCF8 < v17 )
    obstack_newchunk(&unk_9FCE0, v17);
  memcpy(unk_9FCF8, a2, v17);
  unk_9FCF8 += v17;
LABEL_52:
  if ( unk_9FD00 == unk_9FCF8 )
    obstack_newchunk(&unk_9FCE0, 1);
  v4 = (char *)unk_9FCF8++;
  *v4 = 0;
  free(ptr);
}



// Function: _transform_name_to_obstack @ 0x3b734
long long _transform_name_to_obstack(int a1, char *a2, long long *a3)
{
  unsigned char v6; // [rsp+27h] [rbp-19h]
  long long i; // [rsp+28h] [rbp-18h]
  char *v8; // [rsp+38h] [rbp-8h]

  v6 = 0;
  if ( byte_9FD38 != 1 )
  {
    obstack_begin(&unk_9FCE0, 0, 0, sub_60B5B, &free);
    byte_9FD38 = 1;
  }
  for ( i = qword_9FCC0; i; i = *(long long *)i )
  {
    if ( (a1 & *(int *)(i + 12)) != 0 )
    {
      sub_3AFA7(i, a2);
      v8 = (char *)unk_9FCF0;
      if ( unk_9FCF0 == unk_9FCF8 )
        unk_9FD30 |= 2u;
      unk_9FCF8 = (unk_9FD10 + unk_9FCF8) & ~unk_9FD10;
      if ( unk_9FCF8 - unk_9FCE8 > unk_9FD00 - unk_9FCE8 )
        unk_9FCF8 = unk_9FD00;
      unk_9FCF0 = unk_9FCF8;
      a2 = v8;
      v6 = 1;
    }
  }
  *a3 = a2;
  return v6;
}



// Function: transform_name_fp @ 0x3b8af
long long transform_name_fp(void **a1, int a2, long long (*a3)(void *, long long), long long a4)
{
  void *v4; // rax
  long long v5; // rax
  unsigned char v9; // [rsp+2Fh] [rbp-21h]
  void *ptr; // [rsp+30h] [rbp-20h] BYREF
  struct obstack *obstack; // [rsp+38h] [rbp-18h]
  void *block; // [rsp+40h] [rbp-10h]
  unsigned long long v13; // [rsp+48h] [rbp-8h]

  v13 = __readfsqword(0x28u);
  v9 = sub_3B734(a2, *a1, &ptr);
  if ( v9 )
  {
    if ( a3 )
      v4 = (void *)a3(ptr, a4);
    else
      v4 = ptr;
    sub_29547(a1, (long long)v4);
    obstack = (struct obstack *)&unk_9FCE0;
    block = ptr;
    if ( (unsigned long long)ptr <= unk_9FCE8 || block >= obstack->chunk_limit )
    {
      obstack_free(obstack, block);
    }
    else
    {
      obstack->object_base = (char *)block;
      obstack->next_free = obstack->object_base;
    }
  }
  else if ( a3 )
  {
    *a1 = 0;
    v5 = a3(ptr, a4);
    sub_29547(a1, v5);
    free(ptr);
    return 1;
  }
  return v9;
}



// Function: transform_name @ 0x3b9f2
long long transform_name(void **a1, int a2)
{
  return sub_3B8AF(a1, a2, 0, 0);
}



// Function: transform_program_p @ 0x3ba20
bool transform_program_p()
{
  return qword_9FCC0 != 0;
}



// Function: dunlink_alloc @ 0x3ba37
long long dunlink_alloc()
{
  long long *v1; // [rsp+8h] [rbp-8h]

  if ( !qword_9FD60 )
    return sub_60B5B(40);
  v1 = qword_9FD60;
  qword_9FD60 = *(void **)qword_9FD60;
  *v1 = 0;
  return (long long)v1;
}



// Function: dunlink_insert @ 0x3ba89
long long dunlink_insert(long long *a1, long long *a2)
{
  if ( a1 )
  {
    *a2 = *a1;
    *a1 = a2;
  }
  else
  {
    *a2 = qword_9FD48;
    qword_9FD48 = (long long)a2;
  }
  if ( !*a2 )
    qword_9FD50 = (long long)a2;
  return ++qword_9FD58;
}



// Function: dunlink_reclaim @ 0x3bb00
void **dunlink_reclaim(void **a1)
{
  free(a1[2]);
  *a1 = qword_9FD60;
  qword_9FD60 = a1;
  return a1;
}



// Function: flush_deferred_unlinks @ 0x3bb3c
long long flush_deferred_unlinks(char a1)
{
  int v1; // eax
  int v3; // [rsp+1Ch] [rbp-34h]
  long long v4; // [rsp+20h] [rbp-30h]
  long long i; // [rsp+20h] [rbp-30h]
  long long *v6; // [rsp+28h] [rbp-28h]
  char *v7; // [rsp+30h] [rbp-20h]
  long long v8; // [rsp+38h] [rbp-18h]
  long long v9; // [rsp+40h] [rbp-10h]
  char *name; // [rsp+48h] [rbp-8h]

  v6 = 0;
  v3 = dword_9F600;
  v4 = qword_9FD48;
  while ( v4 )
  {
    v9 = *(long long *)v4;
    if ( a1 || qword_9FD68 + *(long long *)(v4 + 32) < (unsigned long long)qword_9EFC8 )
    {
      sub_2ADF1(*(int *)(v4 + 8));
      if ( !*(char *)(v4 + 24) )
      {
        if ( unlinkat(newfd, *(const char **)(v4 + 16), 0) && *__errno_location() != 2 )
          sub_3EAA7(*(long long *)(v4 + 16));
        goto LABEL_20;
      }
      if ( *(int *)(v4 + 8)
        && *(char *)(v4 + 24)
        && (!**(char **)(v4 + 16) || !strcmp(*(const char **)(v4 + 16), ".")) )
      {
        v6 = (long long *)v4;
        v4 = v9;
      }
      else
      {
        name = *(char **)(v4 + 16);
        if ( !unlinkat(newfd, name, 512) )
          goto LABEL_20;
        v1 = *__errno_location();
        if ( v1 == 39 )
          goto LABEL_15;
        if ( v1 > 39 )
        {
LABEL_16:
          sub_3E705(name);
          goto LABEL_20;
        }
        if ( v1 == 2 )
        {
LABEL_20:
          sub_3BB00((void **)v4);
          --qword_9FD58;
          v4 = v9;
          if ( v6 )
            *v6 = v9;
          else
            qword_9FD48 = v9;
        }
        else
        {
          if ( v1 != 17 )
            goto LABEL_16;
LABEL_15:
          v6 = (long long *)v4;
          v4 = v9;
        }
      }
    }
    else
    {
      v6 = (long long *)v4;
      v4 = *(long long *)v4;
    }
  }
  if ( qword_9FD48 )
  {
    if ( a1 )
    {
      for ( i = qword_9FD48; i; i = v8 )
      {
        v8 = *(long long *)i;
        sub_2ADF1(*(int *)(i + 8));
        if ( *(int *)(i + 8)
          && *(char *)(i + 24)
          && (!**(char **)(i + 16) || !strcmp(*(const char **)(i + 16), ".")) )
        {
          v7 = (char *)sub_2AFC0();
          sub_2ADF1(*(int *)(i + 8) - 1);
        }
        else
        {
          v7 = *(char **)(i + 16);
        }
        if ( unlinkat(newfd, v7, 512) && *__errno_location() != 2 )
          sub_3E705(v7);
        sub_3BB00((void **)i);
        --qword_9FD58;
      }
      qword_9FD50 = 0;
      qword_9FD48 = 0;
    }
  }
  else
  {
    qword_9FD50 = 0;
  }
  return sub_2ADF1(v3);
}



// Function: finish_deferred_unlinks @ 0x3be6b
void *finish_deferred_unlinks()
{
  void *result; // rax
  void *v1; // [rsp+8h] [rbp-8h]

  sub_3BB3C(1);
  while ( 1 )
  {
    result = qword_9FD60;
    if ( !qword_9FD60 )
      break;
    v1 = *(void **)qword_9FD60;
    free(qword_9FD60);
    qword_9FD60 = v1;
  }
  return result;
}



// Function: queue_deferred_unlink @ 0x3bebb
long long queue_deferred_unlink(long long a1, char a2)
{
  long long v3; // [rsp+18h] [rbp-18h]
  long long *v4; // [rsp+20h] [rbp-10h]
  long long v5; // [rsp+28h] [rbp-8h]

  if ( qword_9FD48 && qword_9FD68 + *(long long *)(qword_9FD48 + 32) < (unsigned long long)qword_9EFC8 )
    sub_3BB3C(0);
  v5 = sub_3BA37();
  *(long long *)v5 = 0;
  *(int *)(v5 + 8) = dword_9F600;
  *(long long *)(v5 + 16) = sub_60CC6(a1);
  sub_29946(*(char **)(v5 + 16));
  *(char *)(v5 + 24) = a2;
  *(long long *)(v5 + 32) = qword_9EFC8;
  if ( !*(char *)(v5 + 24) || **(char **)(v5 + 16) && strcmp(*(const char **)(v5 + 16), ".") )
    return sub_3BA89((long long *)qword_9FD50, (long long *)v5);
  v3 = qword_9FD48;
  v4 = 0;
  while ( v3
       && (!*(char *)(v3 + 24)
        || **(char **)(v3 + 16) && strcmp(*(const char **)(v3 + 16), ".")
        || *(int *)(v3 + 8) >= *(int *)(v5 + 8)) )
  {
    v4 = (long long *)v3;
    v3 = *(long long *)v3;
  }
  if ( v3 )
    return sub_3BA89(v4, (long long *)v5);
  else
    return sub_3BA89((long long *)qword_9FD50, (long long *)v5);
}



// Function: append_file @ 0x3c078
unsigned long long append_file(const char *a1)
{
  long long v1; // r12
  long long v2; // rbx
  char *v3; // rax
  int fd; // [rsp+1Ch] [rbp-E4h]
  __off_t n; // [rsp+20h] [rbp-E0h]
  char *v7; // [rsp+28h] [rbp-D8h]
  char *v8; // [rsp+30h] [rbp-D0h]
  long long v9; // [rsp+38h] [rbp-C8h]
  struct stat buf; // [rsp+40h] [rbp-C0h] BYREF
  char v11[24]; // [rsp+D0h] [rbp-30h] BYREF
  unsigned long long v12; // [rsp+E8h] [rbp-18h]

  v12 = __readfsqword(0x28u);
  fd = openat(newfd, a1, 0);
  if ( fd >= 0 )
  {
    if ( fstat(fd, &buf) )
    {
      sub_3EA03(a1);
    }
    else
    {
      n = buf.st_size;
      while ( n > 0 )
      {
        v8 = (char *)sub_BB4B();
        v7 = sub_BBF5((long long)v8);
        if ( (unsigned long long)v7 > n )
        {
          v7 = (char *)n;
          if ( (n & 0x1FF) != 0 )
            memset(&v8[n], 0, 512 - (n & 0x1FF));
        }
        v9 = sub_5EC70((unsigned int)fd, v8, v7);
        if ( v9 == -1 )
          sub_3E5FC(a1, buf.st_size - n, v7);
        if ( !v9 )
        {
          if ( qword_9FDC8 )
            qword_9FDC8((unsigned int)fd);
          v1 = sub_54E5A(n, v11);
          v2 = sub_5E548(a1);
          v3 = ngettext("%s: File shrank by %s byte", "%s: File shrank by %s bytes", n);
          error(0, 0, v3, v2, v1);
          sub_177F4();
        }
        n -= v9;
        sub_BBA7((unsigned long long)&v8[(v9 - 1) & 0xFFFFFFFFFFFFFE00LL]);
      }
    }
    if ( close(fd) )
      sub_3E1FF(a1);
  }
  else
  {
    sub_3E38B(a1);
  }
  return v12 - __readfsqword(0x28u);
}



// Function: update_archive @ 0x3c325
unsigned long long update_archive(double a1)
{
  int v1; // ebx
  void *v2; // rax
  long long v3; // rax
  long long v4; // rdx
  long long v5; // rdi
  char *v6; // rax
  char *v7; // rax
  unsigned long long result; // rax
  char v9; // [rsp+7h] [rbp-E9h]
  int v10; // [rsp+8h] [rbp-E8h]
  int v11; // [rsp+Ch] [rbp-E4h]
  char *s; // [rsp+10h] [rbp-E0h]
  long long v13; // [rsp+18h] [rbp-D8h]
  char *v14; // [rsp+20h] [rbp-D0h]
  long long v15; // [rsp+28h] [rbp-C8h]
  char *ptr; // [rsp+30h] [rbp-C0h]
  void **v17; // [rsp+38h] [rbp-B8h]
  struct stat v18; // [rsp+40h] [rbp-B0h] BYREF
  unsigned long long v19; // [rsp+D8h] [rbp-18h]

  v19 = __readfsqword(0x28u);
  v10 = 0;
  v9 = 0;
  sub_2D3B1(a1);
  sub_F694(2u);
  sub_1D498();
  while ( v9 != 1 )
  {
    v11 = sub_25ECF(&qword_9F460, (long long)&qword_9FA20, 0);
    switch ( v11 )
    {
      case 0:
      case 2:
        abort();
      case 1:
        sub_26747(qword_9F460, (long long)&qword_9FA20, &dword_9F468, 0);
        sub_25733(*(char *)(qword_9F460 + 156), (long long)&qword_9FA20);
        dword_9F864 = dword_9F468;
        if ( dword_9F860 == 8 )
        {
          v15 = sub_2ED0E(file, a1);
          if ( v15 )
          {
            sub_2ADF1(*(int *)(v15 + 40));
            if ( !sub_2A9B8(file, &v18) )
            {
              if ( (v18.st_mode & 0xF000) == 0x4000 )
              {
                ptr = (char *)sub_2B8E1(*(const char **)(v15 + 16), 1);
                if ( ptr )
                {
                  v17 = (void **)sub_2B66C(*(const char **)(v15 + 16));
                  for ( s = ptr; *s; s += strlen(s) + 1 )
                  {
                    v1 = *(int *)(v15 + 40);
                    v2 = (void *)sub_2B75B(v17, s);
                    sub_2D5B5(v2, v1, 0, 0);
                  }
                  sub_2B72D(v17);
                  free(ptr);
                  sub_2D817((long long *)v15);
                }
              }
              else
              {
                v3 = sub_5FD9E(&v18);
                if ( (int)sub_39E35(v3, v4, qword_9FB18, qword_9FB20) <= 0 )
                  sub_2D817((long long *)v15);
              }
            }
          }
        }
        sub_28891();
        break;
      case 3:
        qword_9EFB0 = (void *)qword_9F460;
        v9 = 1;
        break;
      case 4:
        v9 = 1;
        break;
      case 5:
        v5 = qword_9F460;
        sub_BBA7(qword_9F460);
        switch ( v10 )
        {
          case 0:
            if ( qword_9FDC8 )
              qword_9FDC8(v5);
            v6 = gettext("This does not look like a tar archive");
            v5 = 0;
            error(0, 0, v6);
            goto LABEL_22;
          case 1:
          case 3:
LABEL_22:
            if ( qword_9FDC8 )
              qword_9FDC8(v5);
            v7 = gettext("Skipping to next header");
            error(0, 0, v7);
            dword_9FDD0 = 2;
            break;
          case 2:
          case 4:
            abort();
          default:
            goto LABEL_26;
        }
        return result;
      default:
        break;
    }
LABEL_26:
    sub_39D1C((long long)&qword_9FA20);
    v10 = v11;
  }
  sub_BAF9();
  byte_9FD70 = 1;
  qword_9FD78 = (long long)qword_9EFB0;
  while ( 1 )
  {
    v13 = sub_2EDA0();
    if ( !v13 )
      break;
    v14 = *(char **)(v13 + 16);
    if ( !(unsigned char)sub_17C43(v14, 0) && (!byte_9F8C8 || sub_34AF4("add", (long long)v14)) )
    {
      if ( dword_9F860 == 2 )
        sub_3C078(v14);
      else
        sub_16A25(0, v14, (long long)v14);
    }
  }
  sub_12DA0();
  sub_D046();
  sub_3BE6B();
  sub_2DBBA(a1);
  return v19 - __readfsqword(0x28u);
}



// Function: utf8_init @ 0x3c7e3
long long utf8_init(unsigned char a1)
{
  const char *v1; // rax

  if ( qword_9EC10[a1] == -1 )
  {
    v1 = (const char *)sub_54EE3();
    if ( a1 )
      qword_9EC10[a1] = iconv_open("UTF-8", v1);
    else
      qword_9EC10[0] = iconv_open(v1, "UTF-8");
  }
  return qword_9EC10[a1];
}



// Function: utf8_convert @ 0x3c8a0
long long utf8_convert(unsigned char a1, char *a2, void **a3)
{
  char *inbuf; // [rsp+28h] [rbp-38h] BYREF
  char *outbuf; // [rsp+30h] [rbp-30h] BYREF
  size_t inbytesleft; // [rsp+38h] [rbp-28h] BYREF
  size_t outbytesleft; // [rsp+40h] [rbp-20h] BYREF
  iconv_t cd; // [rsp+48h] [rbp-18h]
  void *ptr; // [rsp+50h] [rbp-10h]
  unsigned long long v11; // [rsp+58h] [rbp-8h]

  v11 = __readfsqword(0x28u);
  cd = (iconv_t)sub_3C7E3(a1);
  if ( cd )
  {
    if ( cd == (iconv_t)-1LL )
    {
      return 0;
    }
    else
    {
      inbytesleft = strlen(a2) + 1;
      outbytesleft = 16 * inbytesleft + 1;
      ptr = (void *)sub_60B5B(outbytesleft);
      outbuf = (char *)ptr;
      inbuf = a2;
      if ( iconv(cd, &inbuf, &inbytesleft, &outbuf, &outbytesleft) )
      {
        free(ptr);
        return 0;
      }
      else
      {
        *outbuf = 0;
        *a3 = ptr;
        return 1;
      }
    }
  }
  else
  {
    *a3 = (void *)sub_60CC6(a2);
    return 1;
  }
}



// Function: string_ascii_p @ 0x3c9b7
long long string_ascii_p(char *a1)
{
  while ( *a1 )
  {
    if ( *a1 < 0 )
      return 0;
    ++a1;
  }
  return 1;
}



// Function: set_warning_option @ 0x3c9ee
int set_warning_option(const char *a1)
{
  int result; // eax
  char *s1; // [rsp+8h] [rbp-18h]
  int v3; // [rsp+18h] [rbp-8h]
  int v4; // [rsp+1Ch] [rbp-4h]

  s1 = (char *)a1;
  v3 = 0;
  result = strcmp(a1, "none");
  if ( result )
  {
    if ( strlen(a1) > 2 && !memcmp(a1, "no-", 3u) )
    {
      v3 = 1;
      s1 = (char *)(a1 + 3);
    }
    v4 = dword_9EC20[sub_47C61("--warning", s1, off_9AA60, dword_9EC20, 4, off_9EE48)];
    if ( v3 )
    {
      result = ~v4 & dword_9EC84;
      dword_9EC84 = result;
    }
    else
    {
      result = v4 | dword_9EC84;
      dword_9EC84 |= v4;
    }
  }
  else
  {
    dword_9EC84 = 0;
  }
  return result;
}



// Function: acls_one_line @ 0x3cae5
unsigned long long acls_one_line(const char *a1, char a2, const char *a3, unsigned long long a4)
{
  char *v4; // rax
  char *v5; // rax
  int v9; // [rsp+2Ch] [rbp-E4h]
  int v10; // [rsp+30h] [rbp-E0h]
  int v11; // [rsp+34h] [rbp-DCh]
  const char *v12; // [rsp+38h] [rbp-D8h]
  char *object_base; // [rsp+98h] [rbp-78h]
  struct obstack v14; // [rsp+B0h] [rbp-60h] BYREF
  unsigned long long v15; // [rsp+108h] [rbp-8h]

  v15 = __readfsqword(0x28u);
  v10 = strlen(a1);
  v12 = a3;
  v9 = 0;
  if ( a3 && a4 )
  {
    obstack_begin(&v14, 0, 0, sub_60B5B, &free);
    while ( a4 >= v9 )
    {
      v11 = strcspn(a3, ",\n");
      if ( !v11 )
        break;
      if ( v12 != a3 )
      {
        if ( v14.chunk_limit == v14.next_free )
          obstack_newchunk(&v14, 1);
        v4 = v14.next_free++;
        *v4 = a2;
      }
      if ( (char *)(v14.chunk_limit - v14.next_free) < (char *)v10 )
        obstack_newchunk(&v14, v10);
      memcpy(v14.next_free, a1, v10);
      v14.next_free += v10;
      if ( (char *)(v14.chunk_limit - v14.next_free) < (char *)v11 )
        obstack_newchunk(&v14, v11);
      memcpy(v14.next_free, a3, v11);
      v14.next_free += v11;
      v9 += v11 + 1;
      a3 += v11 + 1;
    }
    if ( v14.chunk_limit == v14.next_free )
      obstack_newchunk(&v14, 1);
    v5 = v14.next_free++;
    *v5 = 0;
    object_base = v14.object_base;
    if ( v14.object_base == v14.next_free )
      *((char *)&v14 + 80) |= 2u;
    v14.next_free = (char *)((long long)&v14.next_free[*(long long *)&v14.alignment_mask] & ~*(long long *)&v14.alignment_mask);
    if ( (char *)(v14.next_free - (char *)v14.chunk) > (char *)(v14.chunk_limit - (char *)v14.chunk) )
      v14.next_free = v14.chunk_limit;
    v14.object_base = v14.next_free;
    fprintf(stream, "%s", object_base);
    obstack_free(&v14, 0);
  }
  return v15 - __readfsqword(0x28u);
}



// Function: xattrs_acls_get @ 0x3cf89
void xattrs_acls_get(long long a1)
{
  char *v1; // rax

  if ( dword_9F968 > 0 )
  {
    if ( !dword_9FDB0 )
    {
      if ( qword_9FDC8 )
        qword_9FDC8(a1);
      v1 = gettext("POSIX ACL support is not available");
      error(0, 0, v1);
    }
    dword_9FDB0 = 1;
  }
}



// Function: xattrs_acls_set @ 0x3d003
void xattrs_acls_set(long long a1, long long a2, char a3)
{
  char *v3; // rax

  if ( dword_9F968 > 0 && a3 != 50 )
  {
    if ( !dword_9FDB4 )
    {
      if ( qword_9FDC8 )
        qword_9FDC8(a1);
      v3 = gettext("POSIX ACL support is not available");
      error(0, 0, v3);
    }
    dword_9FDB4 = 1;
  }
}



// Function: mask_map_realloc @ 0x3d07e
long long mask_map_realloc(long long *a1)
{
  long long result; // rax

  result = a1[1];
  if ( a1[2] == result )
  {
    if ( !a1[1] )
      a1[1] = 4;
    result = sub_60A5F(*a1, a1 + 1, 8);
    *a1 = result;
  }
  return result;
}



// Function: xattrs_mask_add @ 0x3d0e5
long long xattrs_mask_add(long long a1, char a2)
{
  long long *v2; // rax
  long long v3; // rsi
  long long v4; // rax
  long long *v6; // [rsp+18h] [rbp-8h]

  if ( a2 )
    v2 = (long long *)&unk_9FD80;
  else
    v2 = (long long *)&unk_9FD98;
  v6 = v2;
  sub_3D07E(v2);
  v3 = *v6;
  v4 = v6[2];
  v6[2] = v4 + 1;
  *(long long *)(v3 + 8 * v4) = a1;
  return a1;
}



// Function: clear_mask_map @ 0x3d14d
void clear_mask_map(long long a1)
{
  if ( *(long long *)(a1 + 8) )
    free(*(void **)a1);
}



// Function: xattrs_clear_setup @ 0x3d17c
void xattrs_clear_setup()
{
  sub_3D14D((long long)&unk_9FD80);
  sub_3D14D((long long)&unk_9FD98);
}



// Function: xattrs_xattrs_get @ 0x3d1a5
long long xattrs_xattrs_get(unsigned int a1, long long a2, long long a3, int a4)
{
  long long result; // rax
  const char *v5; // rax
  const char *v6; // rax
  ssize_t v9; // [rsp+20h] [rbp-20h]
  char *s; // [rsp+28h] [rbp-18h]
  ssize_t v11; // [rsp+30h] [rbp-10h]
  size_t v12; // [rsp+38h] [rbp-8h]

  result = (unsigned int)dword_9F96C;
  if ( dword_9F96C > 0 )
  {
    if ( !list )
      list = (char *)sub_60A5F(0, &size, 1);
    while ( 1 )
    {
      if ( a4 )
      {
        v9 = flistxattr(a4, list, size);
        if ( v9 != -1 )
          break;
      }
      else
      {
        v9 = sub_47394(a1, a2, list, size);
        if ( v9 != -1 )
          break;
      }
      if ( *__errno_location() != 34 )
        break;
      list = (char *)sub_60A5F(list, &size, 1);
    }
    if ( v9 == -1 )
    {
      if ( a4 )
        v5 = "flistxattr";
      else
        v5 = "llistxattrat";
      return sub_3E038(v5, a2);
    }
    else
    {
      s = list;
      result = (long long)qword_9FDC0;
      if ( !qword_9FDC0 )
      {
        result = sub_60A5F(0, &qword_9EC90, 1);
        qword_9FDC0 = (void *)result;
      }
      while ( v9 > 0 )
      {
        v12 = strlen(s);
        while ( 1 )
        {
          if ( a4 )
          {
            v11 = fgetxattr(a4, s, qword_9FDC0, qword_9EC90);
            if ( v11 != -1 )
              break;
          }
          else
          {
            v11 = sub_46E17(a1, a2, s, qword_9FDC0, qword_9EC90);
            if ( v11 != -1 )
              break;
          }
          if ( *__errno_location() != 34 )
            break;
          qword_9FDC0 = (void *)sub_60A5F(qword_9FDC0, &qword_9EC90, 1);
        }
        if ( v11 == -1 )
        {
          if ( *__errno_location() != 61 )
          {
            if ( a4 )
              v6 = "fgetxattr";
            else
              v6 = "lgetxattrat";
            sub_3E038(v6, a2);
          }
        }
        else if ( (unsigned char)sub_3D84F(s, 1) != 1 )
        {
          sub_1D78E(a3, s, (long long)qword_9FDC0, v11);
        }
        s += v12 + 1;
        result = v9 - v12 - 1;
        v9 = result;
      }
    }
  }
  return result;
}



// Function: xattrs__fd_set @ 0x3d44b
void xattrs__fd_set(long long a1, long long a2, char a3, long long a4, long long a5, long long a6)
{
  long long v6; // rdi
  char *v7; // rbx
  int *v8; // rax
  int v10; // [rsp+34h] [rbp-1Ch]
  const char *v11; // [rsp+38h] [rbp-18h]

  if ( a5 )
  {
    v11 = "setxattrat";
    if ( a3 == 50 )
    {
      v11 = "lsetxattr";
      v6 = (unsigned int)newfd;
      v10 = sub_46870((unsigned int)newfd, a2, a4, a5, a6, 0);
    }
    else
    {
      v6 = (unsigned int)newfd;
      v10 = sub_46591((unsigned int)newfd, a2, a4, a5, a6, 0);
    }
    if ( v10 == -1 && (dword_9EC84 & 0x200000) != 0 )
    {
      if ( qword_9FDC8 )
        qword_9FDC8(v6);
      v7 = gettext("%s: Cannot set '%s' extended attribute for file '%s'");
      v8 = __errno_location();
      error(0, *v8, v7, v11, a4, a2);
    }
  }
}



// Function: xattrs_selinux_get @ 0x3d566
long long xattrs_selinux_get(unsigned int a1, long long a2, long long a3, unsigned int a4)
{
  long long result; // rax
  const char *v5; // rax

  result = (unsigned int)dword_9F964;
  if ( dword_9F964 > 0 )
  {
    result = a4 ? sub_61D8A(a4, a3 + 48) : sub_5F56B(a1, a2, a3 + 48);
    if ( (int)result == -1 )
    {
      result = (unsigned int)*__errno_location();
      if ( (int)result != 61 )
      {
        result = (unsigned int)*__errno_location();
        if ( (int)result != 95 )
        {
          if ( a4 )
            v5 = "fgetfilecon";
          else
            v5 = "lgetfileconat";
          return sub_3E038(v5, a2);
        }
      }
    }
  }
  return result;
}



// Function: xattrs_selinux_set @ 0x3d609
void xattrs_selinux_set(long long a1, long long a2, char a3)
{
  long long v3; // rdi
  char *v4; // rbx
  int *v5; // rax
  int v7; // [rsp+24h] [rbp-1Ch]
  const char *v8; // [rsp+28h] [rbp-18h]

  if ( dword_9F964 > 0 && *(long long *)(a1 + 48) )
  {
    v3 = (unsigned int)newfd;
    if ( a3 == 50 )
    {
      v7 = sub_5FA87((unsigned int)newfd, a2, *(long long *)(a1 + 48));
      v8 = "lsetfileconat";
    }
    else
    {
      v7 = sub_5F7F9((unsigned int)newfd, a2, *(long long *)(a1 + 48));
      v8 = "setfileconat";
    }
    if ( v7 == -1 && (dword_9EC84 & 0x200000) != 0 )
    {
      if ( qword_9FDC8 )
        qword_9FDC8(v3);
      v4 = gettext("%s: Cannot set SELinux context for file '%s'");
      v5 = __errno_location();
      error(0, *v5, v4, v8, a2);
    }
  }
}



// Function: xattrs_matches_mask @ 0x3d717
long long xattrs_matches_mask(const char *a1, long long *a2)
{
  int i; // [rsp+1Ch] [rbp-4h]

  if ( !a2[1] )
    return 0;
  for ( i = 0; (unsigned long long)i < a2[2]; ++i )
  {
    if ( !fnmatch(*(const char **)(*a2 + 8LL * i), a1, 0) )
      return 1;
  }
  return 0;
}



// Function: xattrs_kw_included @ 0x3d79b
char xattrs_kw_included(const char *a1, char a2)
{
  if ( qword_9FD88 )
    return sub_3D717(a1, &qword_9FD80);
  if ( a2 )
    return 1;
  return strncmp(a1, "user.", 5u) == 0;
}



// Function: xattrs_kw_excluded @ 0x3d803
long long xattrs_kw_excluded(const char *a1)
{
  return qword_9FDA0 && (unsigned char)sub_3D717(a1, &qword_9FD98);
}



// Function: xattrs_masked_out @ 0x3d84f
long long xattrs_masked_out(const char *a1, char a2)
{
  return !sub_3D79B(a1, a2) || sub_3D803(a1);
}



// Function: xattrs_xattrs_set @ 0x3d8a1
unsigned long long xattrs_xattrs_set(long long a1, long long a2, char a3, int a4)
{
  unsigned long long result; // rax
  unsigned long long v7; // [rsp+20h] [rbp-10h]
  char *s1a; // [rsp+28h] [rbp-8h]
  char *s1; // [rsp+28h] [rbp-8h]

  result = (unsigned int)dword_9F96C;
  if ( dword_9F96C > 0 )
  {
    v7 = 0;
    result = *(long long *)(a1 + 344);
    if ( result )
    {
      while ( 1 )
      {
        result = *(long long *)(a1 + 344);
        if ( v7 >= result )
          break;
        s1a = *(char **)(*(long long *)(a1 + 352) + 24 * v7);
        s1 = &s1a[strlen("SCHILY.xattr.")];
        if ( (a3 != 48 || a4 != (strcmp(s1, "security.capability") != 0)) && !sub_3D84F(s1, 0) )
          sub_3D44B(
            a1,
            a2,
            a3,
            (long long)s1,
            *(long long *)(*(long long *)(a1 + 352) + 24 * v7 + 8),
            *(long long *)(*(long long *)(a1 + 352) + 24 * v7 + 16));
        ++v7;
      }
    }
  }
  return result;
}



// Function: xattrs_print_char @ 0x3d9ec
unsigned long long xattrs_print_char(long long *a1, char *a2)
{
  unsigned long long result; // rax
  long long v3; // rbx
  int i; // [rsp+14h] [rbp-1Ch]
  const char *v5; // [rsp+18h] [rbp-18h]

  if ( dword_9F9AC > 1 )
  {
    if ( dword_9F96C > 0 || dword_9F964 > 0 || dword_9F968 > 0 )
    {
      *a2 = 32;
      a2[1] = 0;
    }
    if ( dword_9F96C > 0 && a1[43] )
    {
      for ( i = 0; (unsigned long long)i < a1[43]; ++i )
      {
        v3 = *(long long *)(a1[44] + 24LL * i);
        v5 = (const char *)(v3 + strlen("SCHILY.xattr."));
        if ( !sub_3D84F(v5, 0) )
        {
          *a2 = 42;
          break;
        }
      }
    }
    if ( dword_9F964 > 0 && a1[6] )
      *a2 = 46;
    result = (unsigned int)dword_9F968;
    if ( dword_9F968 > 0 && (a1[8] || (result = a1[10]) != 0) )
    {
      *a2 = 43;
      return (unsigned long long)a2;
    }
  }
  else
  {
    *a2 = 0;
    return (unsigned long long)a2;
  }
  return result;
}



// Function: xattrs_print @ 0x3db33
unsigned long long xattrs_print(long long a1)
{
  unsigned long long result; // rax
  long long v2; // rbx
  int i; // [rsp+14h] [rbp-1Ch]
  const char *v4; // [rsp+18h] [rbp-18h]

  result = (unsigned int)dword_9F9AC;
  if ( dword_9F9AC > 2 )
  {
    if ( dword_9F964 > 0 && *(long long *)(a1 + 48) )
      fprintf(stream, "  s: %s\n", *(const char **)(a1 + 48));
    if ( dword_9F968 > 0 && (*(long long *)(a1 + 64) || *(long long *)(a1 + 80)) )
    {
      fprintf(stream, "  a: ");
      sub_3CAE5((const char *)&unk_87CB4, 44, *(const char **)(a1 + 56), *(long long *)(a1 + 64));
      if ( *(long long *)(a1 + 64) && *(long long *)(a1 + 80) )
        fprintf(stream, ",");
      sub_3CAE5("default:", 44, *(const char **)(a1 + 72), *(long long *)(a1 + 80));
      fprintf(stream, "\n");
    }
    result = (unsigned int)dword_9F96C;
    if ( dword_9F96C > 0 )
    {
      result = *(long long *)(a1 + 344);
      if ( result )
      {
        for ( i = 0; ; ++i )
        {
          result = *(long long *)(a1 + 344);
          if ( i >= result )
            break;
          v2 = *(long long *)(*(long long *)(a1 + 352) + 24LL * i);
          v4 = (const char *)(v2 + strlen("SCHILY.xattr."));
          if ( !sub_3D84F(v4, 0) )
            fprintf(stream, "  x: %lu %s\n", *(long long *)(*(long long *)(a1 + 352) + 24LL * i + 16), v4);
        }
      }
    }
  }
  return result;
}


