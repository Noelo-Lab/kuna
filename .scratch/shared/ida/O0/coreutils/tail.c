// Function: usage @ 0x4115
void usage(int a1)
{
  long long v1; // rbx
  char *v2; // rax
  char *v3; // rax
  char *v4; // rax
  FILE *v5; // rbx
  char *v6; // rax
  FILE *v7; // rbx
  char *v8; // rax
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

  v1 = qword_18150;
  if ( a1 )
  {
    v2 = gettext("Try '%s --help' for more information.\n");
    fprintf(stderr, v2, v1);
  }
  else
  {
    v3 = gettext("Usage: %s [OPTION]... [FILE]...\n");
    printf(v3, v1);
    v4 = gettext(
           "Print the last %d lines of each FILE to standard output.\n"
           "With more than one FILE, precede each with a header giving the file name.\n");
    printf(v4, 10);
    sub_2A69();
    sub_2A9A();
    v5 = stdout;
    v6 = gettext(
           "  -c, --bytes=[+]NUM       output the last NUM bytes; or use -c +NUM to\n"
           "                             output starting with byte NUM of each file\n");
    fputs_unlocked(v6, v5);
    v7 = stdout;
    v8 = gettext(
           "  -f, --follow[={name|descriptor}]\n"
           "                           output appended data as the file grows;\n"
           "                             an absent option argument means 'descriptor'\n"
           "  -F                       same as --follow=name --retry\n");
    fputs_unlocked(v8, v7);
    v9 = gettext(
           "  -n, --lines=[+]NUM       output the last NUM lines, instead of the last %d;\n"
           "                             or use -n +NUM to output starting with line NUM\n"
           "      --max-unchanged-stats=N\n"
           "                           with --follow=name, reopen a FILE which has not\n"
           "                             changed size after N (default %d) iterations\n"
           "                             to see if it has been unlinked or renamed\n"
           "                             (this is the usual case of rotated log files);\n"
           "                             with inotify, this option is rarely useful\n");
    printf(v9, 10, 5);
    v10 = stdout;
    v11 = gettext(
            "      --pid=PID            with -f, terminate after process ID, PID dies\n"
            "  -q, --quiet, --silent    never output headers giving file names\n"
            "      --retry              keep trying to open a file if it is inaccessible\n");
    fputs_unlocked(v11, v10);
    v12 = stdout;
    v13 = gettext(
            "  -s, --sleep-interval=N   with -f, sleep for approximately N seconds\n"
            "                             (default 1.0) between iterations;\n"
            "                             with inotify and --pid=P, check process P at\n"
            "                             least once every N seconds\n"
            "  -v, --verbose            always output headers giving file names\n");
    fputs_unlocked(v13, v12);
    v14 = stdout;
    v15 = gettext("  -z, --zero-terminated    line delimiter is NUL, not newline\n");
    fputs_unlocked(v15, v14);
    v16 = stdout;
    v17 = gettext("      --help        display this help and exit\n");
    fputs_unlocked(v17, v16);
    v18 = stdout;
    v19 = gettext("      --version     output version information and exit\n");
    fputs_unlocked(v19, v18);
    v20 = stdout;
    v21 = gettext(
            "\n"
            "NUM may have a multiplier suffix:\n"
            "b 512, kB 1000, K 1024, MB 1000*1000, M 1024*1024,\n"
            "GB 1000*1000*1000, G 1024*1024*1024, and so on for T, P, E, Z, Y.\n"
            "Binary prefixes can be used, too: KiB=K, MiB=M, and so on.\n"
            "\n");
    fputs_unlocked(v21, v20);
    v22 = stdout;
    v23 = gettext(
            "With --follow (-f), tail defaults to following the file descriptor, which\n"
            "means that even if a tail'ed file is renamed, tail will continue to track\n"
            "its end.  This default behavior is not desirable when you really want to\n"
            "track the actual name of the file, not the file descriptor (e.g., log\n"
            "rotation).  Use --follow=name in that case.  That causes tail to track the\n"
            "named file in a way that accommodates renaming, removal and creation.\n");
    fputs_unlocked(v23, v22);
    sub_2ACB("tail");
  }
  exit(a1);
}



// Function: die_pipe @ 0x431c
void die_pipe()
{
  raise(13);
  exit(1);
}



// Function: check_output_alive @ 0x4338
unsigned long long check_output_alive()
{
  struct pollfd fds; // [rsp+0h] [rbp-10h] BYREF
  unsigned long long v2; // [rsp+8h] [rbp-8h]

  v2 = __readfsqword(0x28u);
  if ( byte_1812F == 1 )
  {
    fds.fd = 1;
    fds.revents = 0;
    fds.events = 0;
    if ( poll(&fds, 1u, 0) >= 0 && (fds.revents & 0x18) != 0 )
      sub_431C();
  }
  return v2 - __readfsqword(0x28u);
}



// Function: valid_file_spec @ 0x43ba
bool valid_file_spec(long long a1)
{
  return (*(int *)(a1 + 56) == -1) != (*(int *)(a1 + 60) == 0);
}



// Function: pretty_name @ 0x43ee
char *pretty_name(const char **a1)
{
  if ( !strcmp(*a1, "-") )
    return gettext("standard input");
  else
    return (char *)*a1;
}



// Function: record_open_fd @ 0x4435
long long record_open_fd(long long a1, int a2, long long a3, long long a4, int a5)
{
  long long v5; // rdx

  *(int *)(a1 + 56) = a2;
  *(long long *)(a1 + 8) = a3;
  *(long long *)(a1 + 16) = sub_EE31(a4);
  *(long long *)(a1 + 24) = v5;
  *(long long *)(a1 + 32) = *(long long *)a4;
  *(long long *)(a1 + 40) = *(long long *)(a4 + 8);
  *(int *)(a1 + 48) = *(int *)(a4 + 24);
  *(int *)(a1 + 64) = a5;
  *(long long *)(a1 + 88) = 0;
  *(char *)(a1 + 52) = 0;
  return a1;
}



// Function: close_fd @ 0x44d5
void close_fd(unsigned int a1, long long a2)
{
  long long v2; // r12
  char *v3; // rbx
  int *v4; // rax

  if ( a1 != -1 && a1 )
  {
    if ( close(a1) )
    {
      v2 = sub_E8CA(4, a2);
      v3 = gettext("closing %s (fd=%d)");
      v4 = __errno_location();
      error(0, *v4, v3, v2, a1);
    }
  }
}



// Function: write_header @ 0x4559
int write_header(const char *a1)
{
  const char *v1; // rax
  int result; // eax

  if ( byte_18020 )
    v1 = locale;
  else
    v1 = "\n";
  result = printf("%s==> %s <==\n", v1, a1);
  byte_18020 = 0;
  return result;
}



// Function: xwrite_stdout @ 0x45a9
void xwrite_stdout(const void *a1, size_t a2)
{
  long long v2; // r12
  char *v3; // rbx
  int *v4; // rax

  if ( a2 )
  {
    if ( a2 > fwrite_unlocked(a1, 1u, a2, stdout) )
    {
      clearerr_unlocked(stdout);
      v2 = sub_E8CA(4, "standard output");
      v3 = gettext("error writing %s");
      v4 = __errno_location();
      error(1, *v4, v3, v2);
    }
  }
}



// Function: dump_remainder @ 0x4649
long long dump_remainder(char a1, const char *a2, unsigned int a3, unsigned long long a4)
{
  long long v4; // rax
  long long v5; // r12
  char *v6; // rbx
  int *v7; // rax
  long long v12; // [rsp+20h] [rbp-2040h]
  unsigned long long v13; // [rsp+28h] [rbp-2038h]
  size_t v14; // [rsp+38h] [rbp-2028h]
  char v15[16]; // [rsp+40h] [rbp-2020h] BYREF
  unsigned long long v16; // [rsp+2048h] [rbp-18h]

  v16 = __readfsqword(0x28u);
  v13 = a4;
  v12 = 0;
  while ( 1 )
  {
    v4 = v13;
    if ( v13 > 0x2000 )
      v4 = 0x2000;
    v14 = sub_ED34(a3, v15, v4);
    if ( v14 == -1 )
    {
      if ( *__errno_location() == 11 )
        return v12;
      v5 = sub_E8CA(4, a2);
      v6 = gettext("error reading %s");
      v7 = __errno_location();
      error(1, *v7, v6, v5);
    }
    if ( !v14 )
      break;
    if ( a1 )
    {
      sub_4559(a2);
      a1 = 0;
    }
    sub_45A9(v15, v14);
    v12 += v14;
    if ( a4 != -1 )
    {
      v13 -= v14;
      if ( !v13 || a4 == -2 )
        break;
    }
  }
  return v12;
}



// Function: xlseek @ 0x4804
__off_t xlseek(int a1, __off_t a2, int a3, long long a4)
{
  long long v5; // r12
  char *v6; // rbx
  int *v7; // rax
  long long v8; // r12
  char *v9; // rbx
  int *v10; // rax
  long long v11; // r12
  char *v12; // rbx
  int *v13; // rax
  __off_t v16; // [rsp+20h] [rbp-40h]
  long long v17; // [rsp+28h] [rbp-38h]
  char v18[24]; // [rsp+30h] [rbp-30h] BYREF
  unsigned long long v19; // [rsp+48h] [rbp-18h]

  v19 = __readfsqword(0x28u);
  v16 = lseek(a1, a2, a3);
  if ( v16 < 0 )
  {
    v17 = sub_CADE(a2, v18);
    if ( a3 == 2 )
    {
      v11 = sub_EA5C(0, 3, a4);
      v12 = gettext("%s: cannot seek to end-relative offset %s");
      v13 = __errno_location();
      error(0, *v13, v12, v11, v17);
    }
    else
    {
      if ( a3 > 2 )
        goto LABEL_11;
      if ( a3 )
      {
        if ( a3 != 1 )
LABEL_11:
          abort();
        v8 = sub_EA5C(0, 3, a4);
        v9 = gettext("%s: cannot seek to relative offset %s");
        v10 = __errno_location();
        error(0, *v10, v9, v8, v17);
      }
      else
      {
        v5 = sub_EA5C(0, 3, a4);
        v6 = gettext("%s: cannot seek to offset %s");
        v7 = __errno_location();
        error(0, *v7, v6, v5, v17);
      }
    }
    exit(1);
  }
  return v16;
}



// Function: file_lines @ 0x49bd
long long file_lines(const char *a1, unsigned int a2, long long a3, __off_t a4, unsigned long long a5, long long *a6)
{
  long long v7; // r12
  char *v8; // rbx
  int *v9; // rax
  long long v11; // r12
  char *v12; // rbx
  int *v13; // rax
  long long v17; // [rsp+18h] [rbp-2058h]
  long long v18; // [rsp+30h] [rbp-2040h]
  long long v19; // [rsp+30h] [rbp-2040h]
  __off_t v20; // [rsp+38h] [rbp-2038h]
  size_t n; // [rsp+40h] [rbp-2030h]
  char *v22; // [rsp+48h] [rbp-2028h]
  char s[16]; // [rsp+50h] [rbp-2020h] BYREF
  unsigned long long v24; // [rsp+2058h] [rbp-18h]

  v17 = a3;
  v24 = __readfsqword(0x28u);
  if ( !a3 )
    return 1;
  v18 = (long long)(a5 - a4) % 0x2000;
  if ( ((((unsigned long long)((long long)(a5 - a4) >> 63) >> 51) + (unsigned short)a5 - (unsigned short)a4) & 0x1FFF) == (unsigned long long)((long long)(a5 - a4) >> 63) >> 51 )
    v18 = 0x2000;
  v20 = a5 - v18;
  sub_4804(a2, a5 - v18, 0, (long long)a1);
  v19 = sub_ED34(a2, s, v18);
  if ( v19 == -1 )
  {
    v7 = sub_E8CA(4, a1);
    v8 = gettext("error reading %s");
    v9 = __errno_location();
    error(0, *v9, v8, v7);
    return 0;
  }
  else
  {
    *a6 = v19 + v20;
    if ( v19 && s[v19 - 1] != byte_18132 )
      --v17;
    do
    {
      n = v19;
      while ( n )
      {
        v22 = (char *)memrchr(s, byte_18132, n);
        if ( !v22 )
          break;
        n = v22 - s;
        if ( !v17-- )
        {
          sub_45A9(v22 + 1, v19 - n - 1);
          *a6 += sub_4649(0, a1, a2, a5 - (v20 + v19));
          return 1;
        }
      }
      if ( v20 == a4 )
      {
        sub_4804(a2, a4, 0, (long long)a1);
        *a6 = sub_4649(0, a1, a2, a5) + a4;
        return 1;
      }
      v20 -= 0x2000LL;
      sub_4804(a2, v20, 0, (long long)a1);
      v19 = sub_ED34(a2, s, 0x2000);
      if ( v19 == -1 )
      {
        v11 = sub_E8CA(4, a1);
        v12 = gettext("error reading %s");
        v13 = __errno_location();
        error(0, *v13, v12, v11);
        return 0;
      }
      *a6 = v19 + v20;
    }
    while ( v19 );
    return 1;
  }
}



// Function: pipe_lines @ 0x4df7
long long pipe_lines(long long a1, unsigned int a2, size_t a3, long long *a4)
{
  size_t v4; // rax
  char *i; // rax
  long long v6; // r12
  char *v7; // rbx
  int *v8; // rax
  unsigned char v12; // [rsp+2Fh] [rbp-61h]
  size_t *ptr; // [rsp+30h] [rbp-60h]
  size_t *v14; // [rsp+38h] [rbp-58h]
  size_t *src; // [rsp+40h] [rbp-50h]
  size_t *srca; // [rsp+40h] [rbp-50h]
  size_t *srcb; // [rsp+40h] [rbp-50h]
  size_t *srcc; // [rsp+40h] [rbp-50h]
  unsigned long long v19; // [rsp+48h] [rbp-48h]
  size_t *v20; // [rsp+58h] [rbp-38h]
  long long j; // [rsp+60h] [rbp-30h]
  size_t v22; // [rsp+68h] [rbp-28h]
  char *v23; // [rsp+70h] [rbp-20h]
  char *v24; // [rsp+78h] [rbp-18h]

  v19 = 0;
  v12 = 1;
  v14 = (size_t *)sub_F96C(8216);
  ptr = v14;
  v14[1025] = 0;
  v14[1024] = v14[1025];
  v14[1026] = 0;
  src = (size_t *)sub_F96C(8216);
  while ( 1 )
  {
    v4 = sub_ED34(a2, src, 0x2000);
    v22 = v4;
    if ( !v4 || v4 == -1 )
      break;
    src[1024] = v4;
    *a4 += v4;
    src[1025] = 0;
    src[1026] = 0;
    v23 = (char *)src + v4;
    for ( i = (char *)memchr(src, byte_18132, v4); i; i = (char *)memchr(i + 1, byte_18132, v23 - (i + 1)) )
      ++src[1025];
    v19 += src[1025];
    if ( src[1024] + v14[1024] > 0x1FFF )
    {
      v14[1026] = (size_t)src;
      v14 = (size_t *)v14[1026];
      if ( a3 >= v19 - ptr[1025] )
      {
        src = (size_t *)sub_F96C(8216);
      }
      else
      {
        src = ptr;
        v19 -= ptr[1025];
        ptr = (size_t *)ptr[1026];
      }
    }
    else
    {
      memcpy((char *)v14 + v14[1024], src, src[1024]);
      v14[1024] += src[1024];
      v14[1025] += src[1025];
    }
  }
  free(src);
  if ( v22 == -1 )
  {
    v6 = sub_E8CA(4, a1);
    v7 = gettext("error reading %s");
    v8 = __errno_location();
    error(0, *v8, v7, v6);
    v12 = 0;
  }
  else if ( v14[1024] && a3 )
  {
    if ( *((char *)v14 + v14[1024] - 1) != byte_18132 )
    {
      ++v14[1025];
      ++v19;
    }
    for ( srca = ptr; a3 < v19 - srca[1025]; srca = (size_t *)srca[1026] )
      v19 -= srca[1025];
    v20 = srca;
    v24 = (char *)srca + srca[1024];
    if ( v19 > a3 )
    {
      for ( j = v19 - a3; j; --j )
        v20 = (size_t *)((char *)rawmemchr(v20, byte_18132) + 1);
    }
    sub_45A9(v20, v24 - (char *)v20);
    for ( srcb = (size_t *)srca[1026]; srcb; srcb = (size_t *)srcb[1026] )
      sub_45A9(srcb, srcb[1024]);
  }
  while ( ptr )
  {
    srcc = (size_t *)ptr[1026];
    free(ptr);
    ptr = srcc;
  }
  return v12;
}



// Function: pipe_bytes @ 0x5286
long long pipe_bytes(long long a1, unsigned int a2, size_t a3, long long *a4)
{
  long long v4; // rax
  long long v5; // r12
  char *v6; // rbx
  int *v7; // rax
  unsigned char v11; // [rsp+2Fh] [rbp-41h]
  size_t *ptr; // [rsp+30h] [rbp-40h]
  long long v13; // [rsp+38h] [rbp-38h]
  size_t *src; // [rsp+40h] [rbp-30h]
  size_t *srca; // [rsp+40h] [rbp-30h]
  size_t *srcb; // [rsp+40h] [rbp-30h]
  size_t *srcc; // [rsp+40h] [rbp-30h]
  long long v18; // [rsp+48h] [rbp-28h]
  unsigned long long v19; // [rsp+50h] [rbp-20h]
  long long v20; // [rsp+58h] [rbp-18h]

  v19 = 0;
  v11 = 1;
  v13 = sub_F96C(8208);
  ptr = (size_t *)v13;
  *(long long *)(v13 + 0x2000) = 0;
  *(long long *)(v13 + 8200) = 0;
  src = (size_t *)sub_F96C(8208);
  while ( 1 )
  {
    v4 = sub_ED34(a2, src, 0x2000);
    v20 = v4;
    if ( !v4 || v4 == -1 )
      break;
    *a4 += v4;
    src[1024] = v4;
    src[1025] = 0;
    v19 += src[1024];
    if ( src[1024] + *(long long *)(v13 + 0x2000) > 0x1FFF )
    {
      *(long long *)(v13 + 8200) = src;
      v13 = *(long long *)(v13 + 8200);
      if ( a3 >= v19 - ptr[1024] )
      {
        src = (size_t *)sub_F96C(8208);
      }
      else
      {
        src = ptr;
        v19 -= ptr[1024];
        ptr = (size_t *)ptr[1025];
      }
    }
    else
    {
      memcpy((void *)(*(long long *)(v13 + 0x2000) + v13), src, src[1024]);
      *(long long *)(v13 + 0x2000) += src[1024];
    }
  }
  free(src);
  if ( v20 == -1 )
  {
    v5 = sub_E8CA(4, a1);
    v6 = gettext("error reading %s");
    v7 = __errno_location();
    error(0, *v7, v6, v5);
    v11 = 0;
  }
  else
  {
    for ( srca = ptr; a3 < v19 - srca[1024]; srca = (size_t *)srca[1025] )
      v19 -= srca[1024];
    if ( v19 <= a3 )
      v18 = 0;
    else
      v18 = v19 - a3;
    sub_45A9((char *)srca + v18, srca[1024] - v18);
    for ( srcb = (size_t *)srca[1025]; srcb; srcb = (size_t *)srcb[1025] )
      sub_45A9(srcb, srcb[1024]);
  }
  while ( ptr )
  {
    srcc = (size_t *)ptr[1025];
    free(ptr);
    ptr = srcc;
  }
  return v11;
}



// Function: start_bytes @ 0x55ae
long long start_bytes(long long a1, unsigned int a2, unsigned long long a3, long long *a4)
{
  long long v5; // r12
  char *v6; // rbx
  int *v7; // rax
  unsigned long long v10; // [rsp+20h] [rbp-2030h]
  char v11[16]; // [rsp+30h] [rbp-2020h] BYREF
  unsigned long long v12; // [rsp+2038h] [rbp-18h]

  v12 = __readfsqword(0x28u);
  while ( a3 )
  {
    v10 = sub_ED34(a2, v11, 0x2000);
    if ( !v10 )
      return 0xFFFFFFFFLL;
    if ( v10 == -1 )
    {
      v5 = sub_E8CA(4, a1);
      v6 = gettext("error reading %s");
      v7 = __errno_location();
      error(0, *v7, v6, v5);
      return 1;
    }
    *a4 += v10;
    if ( v10 > a3 )
    {
      sub_45A9(&v11[a3], v10 - a3);
      return 0;
    }
    a3 -= v10;
  }
  return 0;
}



// Function: start_lines @ 0x5741
long long start_lines(long long a1, unsigned int a2, long long a3, long long *a4)
{
  long long v5; // r12
  char *v6; // rbx
  int *v7; // rax
  char *i; // rax
  long long v10; // [rsp+8h] [rbp-2058h]
  char *s; // [rsp+28h] [rbp-2038h]
  long long v12; // [rsp+30h] [rbp-2030h]
  char *v13; // [rsp+38h] [rbp-2028h]
  char v14[16]; // [rsp+40h] [rbp-2020h] BYREF
  unsigned long long v15; // [rsp+2048h] [rbp-18h]

  v10 = a3;
  v15 = __readfsqword(0x28u);
  if ( !a3 )
    return 0;
  while ( 1 )
  {
    v12 = sub_ED34(a2, v14, 0x2000);
    if ( !v12 )
      return 0xFFFFFFFFLL;
    if ( v12 == -1 )
      break;
    v13 = &v14[v12];
    *a4 += v12;
    for ( i = (char *)memchr(v14, byte_18132, v13 - v14); i; i = (char *)memchr(s, byte_18132, v13 - s) )
    {
      s = i + 1;
      if ( !--v10 )
      {
        if ( s < v13 )
          sub_45A9(s, v13 - s);
        return 0;
      }
    }
  }
  v5 = sub_E8CA(4, a1);
  v6 = gettext("error reading %s");
  v7 = __errno_location();
  error(0, *v7, v6, v5);
  return 1;
}



// Function: fremote @ 0x5938
long long fremote(int a1, long long a2)
{
  long long v2; // r12
  char *v3; // rbx
  int *v4; // rax
  int v5; // eax
  unsigned char v7; // [rsp+1Bh] [rbp-95h]
  struct statfs buf; // [rsp+20h] [rbp-90h] BYREF
  unsigned long long v9; // [rsp+98h] [rbp-18h]

  v9 = __readfsqword(0x28u);
  v7 = 1;
  if ( fstatfs(a1, &buf) )
  {
    if ( *__errno_location() != 38 )
    {
      v2 = sub_E8CA(4, a2);
      v3 = gettext("cannot determine location of %s. reverting to polling");
      v4 = __errno_location();
      error(0, *v4, v3, v2);
    }
  }
  else
  {
    v5 = sub_2DA1(buf.f_type);
    if ( v5 == 1 )
    {
      return 0;
    }
    else if ( v5 > 1 || v5 != -1 && v5 )
    {
      __assert_fail("!\"unexpected return value from is_local_fs_type\"", "src/tail.c", 0x3CCu, "fremote");
    }
  }
  return v7;
}



// Function: recheck @ 0x5a69
unsigned long long recheck(long long a1, unsigned char a2)
{
  int v2; // eax
  int v3; // ecx
  int v4; // r8d
  int v5; // r9d
  int v6; // edx
  int v7; // eax
  bool v8; // al
  char *v9; // rax
  long long v10; // rbx
  char *v11; // rax
  char *v12; // rax
  long long v13; // rbx
  char *v14; // rax
  char *v15; // rax
  const char *v16; // rbx
  int *v17; // rax
  bool v18; // al
  char *v19; // rbx
  char *v20; // rax
  long long v21; // r12
  char *v22; // rax
  char *v23; // rax
  char *v24; // rax
  long long v25; // rbx
  char *v26; // rax
  char *v27; // rax
  char *v28; // rax
  char *v29; // rax
  long long v30; // rbx
  char *v31; // rax
  char *v32; // rax
  long long v33; // rbx
  char *v34; // rax
  char *v35; // rax
  long long v36; // rbx
  char *v37; // rax
  char *v38; // rax
  char *v39; // rax
  int v40; // ecx
  char *v41; // rax
  char v43; // [rsp+14h] [rbp-BCh]
  char v44; // [rsp+15h] [rbp-BBh]
  bool v45; // [rsp+16h] [rbp-BAh]
  char v46; // [rsp+17h] [rbp-B9h]
  int v47; // [rsp+18h] [rbp-B8h]
  int fd; // [rsp+1Ch] [rbp-B4h]
  struct stat buf; // [rsp+20h] [rbp-B0h] BYREF
  unsigned long long v50; // [rsp+B8h] [rbp-18h]

  v50 = __readfsqword(0x28u);
  v43 = 1;
  v2 = strcmp(*(const char **)a1, "-");
  v45 = v2 == 0;
  v46 = *(char *)(a1 + 54);
  v47 = *(int *)(a1 + 60);
  if ( v2 )
  {
    if ( a2 )
      v6 = 0;
    else
      v6 = 2048;
    v7 = sub_AF09(*(long long *)a1, v6, v6, v3, v4, v5);
  }
  else
  {
    v7 = 0;
  }
  fd = v7;
  if ( !sub_43BA(a1) )
    __assert_fail("valid_file_spec (f)", "src/tail.c", 0x3E2u, "recheck");
  v8 = byte_1812C != 1 || fd != -1;
  *(char *)(a1 + 54) = v8;
  if ( byte_1813A != 1 && !lstat(*(const char **)a1, &buf) && (buf.st_mode & 0xF000) == 0xA000 )
  {
    v43 = 0;
    *(int *)(a1 + 60) = -1;
    *(char *)(a1 + 52) = 1;
    v9 = sub_43EE((const char **)a1);
    v10 = sub_E8CA(4, v9);
    v11 = gettext("%s has been replaced with an untailable symbolic link");
    error(0, 0, v11, v10);
  }
  else if ( fd == -1 || fstat(fd, &buf) < 0 )
  {
    v43 = 0;
    *(int *)(a1 + 60) = *__errno_location();
    if ( *(char *)(a1 + 54) != 1 )
    {
      if ( v46 )
      {
        v12 = sub_43EE((const char **)a1);
        v13 = sub_E8CA(4, v12);
        v14 = gettext("%s has become inaccessible");
        error(0, *(int *)(a1 + 60), v14, v13);
      }
    }
    else if ( v47 != *__errno_location() )
    {
      v15 = sub_43EE((const char **)a1);
      v16 = (const char *)sub_EA5C(0, 3, v15);
      v17 = __errno_location();
      error(0, *v17, "%s", v16);
    }
  }
  else if ( (buf.st_mode & 0xF000) == 0x8000
         || (buf.st_mode & 0xF000) == 0x1000
         || (buf.st_mode & 0xF000) == 0xC000
         || (buf.st_mode & 0xF000) == 0x2000 )
  {
    v23 = sub_43EE((const char **)a1);
    *(char *)(a1 + 53) = sub_5938(fd, (long long)v23);
    if ( *(char *)(a1 + 53) && byte_1813A != 1 )
    {
      v43 = 0;
      *(int *)(a1 + 60) = -1;
      v24 = sub_43EE((const char **)a1);
      v25 = sub_E8CA(4, v24);
      v26 = gettext("%s has been replaced with an untailable remote file");
      error(0, 0, v26, v25);
      *(char *)(a1 + 52) = 1;
      *(char *)(a1 + 53) = 1;
    }
    else
    {
      *(int *)(a1 + 60) = 0;
    }
  }
  else
  {
    v43 = 0;
    *(int *)(a1 + 60) = -1;
    *(char *)(a1 + 54) = 0;
    v18 = byte_1812C != 1 || dword_18010 != 1;
    *(char *)(a1 + 52) = v18;
    if ( v46 || v47 != *(int *)(a1 + 60) )
    {
      if ( *(char *)(a1 + 52) )
        v19 = gettext("; giving up on this name");
      else
        v19 = (char *)locale;
      v20 = sub_43EE((const char **)a1);
      v21 = sub_E8CA(4, v20);
      v22 = gettext("%s has been replaced with an untailable file%s");
      error(0, 0, v22, v21, v19);
    }
  }
  v44 = 0;
  if ( v43 != 1 )
  {
    v27 = sub_43EE((const char **)a1);
    sub_44D5(fd, (long long)v27);
    v28 = sub_43EE((const char **)a1);
    sub_44D5(*(int *)(a1 + 56), (long long)v28);
    *(int *)(a1 + 56) = -1;
  }
  else if ( !v47 || v47 == 2 )
  {
    if ( *(int *)(a1 + 56) == -1 )
    {
      v44 = 1;
      v32 = sub_43EE((const char **)a1);
      v33 = sub_E8CA(4, v32);
      v34 = gettext("%s has appeared;  following new file");
      error(0, 0, v34, v33);
    }
    else if ( *(long long *)(a1 + 40) == buf.st_ino && *(long long *)(a1 + 32) == buf.st_dev )
    {
      v39 = sub_43EE((const char **)a1);
      sub_44D5(fd, (long long)v39);
    }
    else
    {
      v44 = 1;
      v35 = sub_43EE((const char **)a1);
      v36 = sub_E8CA(4, v35);
      v37 = gettext("%s has been replaced;  following new file");
      error(0, 0, v37, v36);
      v38 = sub_43EE((const char **)a1);
      sub_44D5(*(int *)(a1 + 56), (long long)v38);
    }
  }
  else
  {
    v44 = 1;
    if ( *(int *)(a1 + 56) != -1 )
      __assert_fail("f->fd == -1", "src/tail.c", 0x42Fu, "recheck");
    v29 = sub_43EE((const char **)a1);
    v30 = sub_E8CA(4, v29);
    v31 = gettext("%s has become accessible");
    error(0, 0, v31, v30);
  }
  if ( v44 )
  {
    if ( v45 )
      v40 = -1;
    else
      v40 = a2;
    sub_4435(a1, fd, 0, (long long)&buf, v40);
    v41 = sub_43EE((const char **)a1);
    sub_4804(fd, 0, 0, (long long)v41);
  }
  return v50 - __readfsqword(0x28u);
}



// Function: any_live_files @ 0x6245
long long any_live_files(long long a1, unsigned long long a2)
{
  unsigned long long i; // [rsp+18h] [rbp-8h]

  if ( byte_1812C && dword_18010 == 1 )
    return 1;
  for ( i = 0; i < a2; ++i )
  {
    if ( *(int *)(96 * i + a1 + 56) >= 0 )
      return 1;
    if ( *(char *)(96 * i + a1 + 52) != 1 && byte_1812C )
      return 1;
  }
  return 0;
}



// Function: tail_forever @ 0x62f3
unsigned long long tail_forever(long long a1, unsigned long long a2, double a3)
{
  bool v3; // al
  int v4; // ecx
  int v5; // r8d
  int v6; // r9d
  int v7; // edx
  int v8; // ecx
  int v9; // r8d
  int v10; // r9d
  int v11; // eax
  long long v12; // r12
  char *v13; // rbx
  int *v14; // rax
  const char *v15; // rbx
  int *v16; // rax
  long long v17; // rax
  long long v18; // rdx
  long long v19; // rax
  unsigned long long v20; // rdx
  long long v21; // rbx
  long long v22; // rdx
  long long v23; // rbx
  char *v24; // rax
  long long v25; // rax
  char *v26; // rax
  char *v27; // rbx
  int *v28; // rax
  bool v29; // al
  char *v30; // rbx
  int *v31; // rax
  char v33; // [rsp+0h] [rbp-110h]
  bool v34; // [rsp+25h] [rbp-EBh]
  bool v35; // [rsp+26h] [rbp-EAh]
  unsigned char v36; // [rsp+27h] [rbp-E9h]
  int fd; // [rsp+28h] [rbp-E8h]
  int v38; // [rsp+2Ch] [rbp-E4h]
  int v39; // [rsp+30h] [rbp-E0h]
  int v40; // [rsp+34h] [rbp-DCh]
  unsigned long long v41; // [rsp+38h] [rbp-D8h]
  unsigned long long i; // [rsp+40h] [rbp-D0h]
  char *v43; // [rsp+50h] [rbp-C0h]
  struct stat buf; // [rsp+60h] [rbp-B0h] BYREF
  unsigned long long v45; // [rsp+F8h] [rbp-18h]

  v45 = __readfsqword(0x28u);
  v3 = !pid && dword_18010 == 2 && a2 == 1 && *(int *)(a1 + 56) != -1 && (*(int *)(a1 + 48) & 0xF000) != 0x8000;
  v36 = v3;
  v34 = 0;
  v41 = a2 - 1;
  do
  {
LABEL_9:
    v35 = 0;
    for ( i = 0; i < a2; ++i )
    {
      if ( *(char *)(96 * i + a1 + 52) )
        continue;
      if ( *(int *)(96 * i + a1 + 56) < 0 )
      {
        sub_5A69(96 * i + a1, v36);
        continue;
      }
      fd = *(int *)(96 * i + a1 + 56);
      v43 = sub_43EE((const char **)(96 * i + a1));
      v38 = *(int *)(96 * i + a1 + 48);
      v7 = *(int *)(96 * i + a1 + 64);
      if ( v7 != v36 )
      {
        v39 = sub_AA63(fd, 3, v7, v4, v5, v6, v33);
        if ( v36 )
          v11 = 0;
        else
          v11 = 2048;
        v40 = v39 | v11;
        if ( v39 >= 0 && (v40 == v39 || (unsigned int)sub_AA63(fd, 4, v40, v8, v9, v10, v33) != -1) )
        {
          *(int *)(a1 + 96 * i + 64) = v36;
        }
        else if ( (*(int *)(96 * i + a1 + 48) & 0xF000) != 0x8000 || *__errno_location() != 1 )
        {
          v12 = sub_EA5C(0, 3, v43);
          v13 = gettext("%s: cannot change nonblocking mode");
          v14 = __errno_location();
          error(1, *v14, v13, v12);
        }
      }
      if ( *(int *)(96 * i + a1 + 64) )
        goto LABEL_42;
      if ( fstat(fd, &buf) )
      {
        *(int *)(96 * i + a1 + 56) = -1;
        *(int *)(a1 + 96 * i + 60) = *__errno_location();
        v15 = (const char *)sub_EA5C(0, 3, v43);
        v16 = __errno_location();
        error(0, *v16, "%s", v15);
        close(fd);
        continue;
      }
      if ( *(int *)(96 * i + a1 + 48) != buf.st_mode
        || (buf.st_mode & 0xF000) == 0x8000 && *(long long *)(96 * i + a1 + 8) != buf.st_size
        || (v17 = sub_EE31(&buf),
            (unsigned int)sub_EEAF(*(long long *)(96 * i + a1 + 16), *(long long *)(96 * i + a1 + 24), v17, v18)) )
      {
        v21 = 96 * i + a1;
        *(long long *)(v21 + 16) = sub_EE31(&buf);
        *(long long *)(v21 + 24) = v22;
        *(int *)(v21 + 48) = buf.st_mode;
        *(long long *)(v21 + 88) = 0;
        if ( (v38 & 0xF000) == 0x8000 && buf.st_size < *(long long *)(96 * i + a1 + 8) )
        {
          v23 = sub_EA5C(0, 3, v43);
          v24 = gettext("%s: file truncated");
          error(0, 0, v24, v23);
          sub_4804(fd, 0, 0, (long long)v43);
          *(long long *)(96 * i + a1 + 8) = 0;
        }
        if ( i != v41 )
        {
          if ( byte_18131 )
            sub_4559(v43);
          v41 = i;
        }
LABEL_42:
        if ( *(int *)(96 * i + a1 + 64) )
        {
          v25 = sub_4649(0, v43, fd, 0xFFFFFFFFFFFFFFFELL);
        }
        else if ( (v38 & 0xF000) == 0x8000 && *(char *)(96 * i + a1 + 53) )
        {
          v25 = sub_4649(0, v43, fd, buf.st_size - *(long long *)(96 * i + a1 + 8));
        }
        else
        {
          v25 = sub_4649(0, v43, fd, 0xFFFFFFFFFFFFFFFFLL);
        }
        v35 = v35 || v25 != 0;
        *(long long *)(96 * i + a1 + 8) += v25;
        continue;
      }
      v19 = 96 * i + a1;
      v20 = *(long long *)(v19 + 88);
      *(long long *)(v19 + 88) = v20 + 1;
      if ( v20 >= qword_18018 && dword_18010 == 1 )
      {
        sub_5A69(96 * i + a1, *(int *)(96 * i + a1 + 64) != 0);
        *(long long *)(96 * i + a1 + 88) = 0;
      }
    }
    if ( (unsigned char)sub_6245(a1, a2) != 1 )
    {
      v26 = gettext("no files remaining");
      error(0, 0, v26);
      return v45 - __readfsqword(0x28u);
    }
    if ( (!v35 || v36) && fflush_unlocked(stdout) )
    {
      v27 = gettext("write error");
      v28 = __errno_location();
      error(1, *v28, v27);
    }
    sub_4338();
  }
  while ( v35 );
  if ( !v34 )
  {
    v29 = pid && kill(pid, 0) && *__errno_location() != 1;
    v34 = v29;
    if ( !v29 && (unsigned int)sub_101AF(a3) )
    {
      v30 = gettext("cannot read realtime clock");
      v31 = __errno_location();
      error(1, *v31, v30);
      return v45 - __readfsqword(0x28u);
    }
    goto LABEL_9;
  }
  return v45 - __readfsqword(0x28u);
}



// Function: any_remote_file @ 0x6d57
long long any_remote_file(long long a1, unsigned long long a2)
{
  unsigned long long i; // [rsp+18h] [rbp-8h]

  for ( i = 0; i < a2; ++i )
  {
    if ( *(int *)(96 * i + a1 + 56) >= 0 && *(char *)(96 * i + a1 + 53) )
      return 1;
  }
  return 0;
}



// Function: any_non_remote_file @ 0x6dd3
long long any_non_remote_file(long long a1, unsigned long long a2)
{
  unsigned long long i; // [rsp+18h] [rbp-8h]

  for ( i = 0; i < a2; ++i )
  {
    if ( *(int *)(96 * i + a1 + 56) >= 0 && *(char *)(96 * i + a1 + 53) != 1 )
      return 1;
  }
  return 0;
}



// Function: any_symlinks @ 0x6e52
long long any_symlinks(long long a1, unsigned long long a2)
{
  unsigned long long i; // [rsp+18h] [rbp-A8h]
  struct stat buf; // [rsp+20h] [rbp-A0h] BYREF
  unsigned long long v5; // [rsp+B8h] [rbp-8h]

  v5 = __readfsqword(0x28u);
  for ( i = 0; i < a2; ++i )
  {
    if ( !lstat(*(const char **)(96 * i + a1), &buf) && (buf.st_mode & 0xF000) == 0xA000 )
      return 1;
  }
  return 0;
}



// Function: any_non_regular_fifo @ 0x6f11
long long any_non_regular_fifo(long long a1, unsigned long long a2)
{
  unsigned long long i; // [rsp+18h] [rbp-8h]

  for ( i = 0; i < a2; ++i )
  {
    if ( *(int *)(96 * i + a1 + 56) >= 0
      && (*(int *)(96 * i + a1 + 48) & 0xF000) != 0x8000
      && (*(int *)(96 * i + a1 + 48) & 0xF000) != 0x1000 )
    {
      return 1;
    }
  }
  return 0;
}



// Function: tailable_stdin @ 0x6fc5
long long tailable_stdin(long long a1, unsigned long long a2)
{
  unsigned long long i; // [rsp+18h] [rbp-8h]

  for ( i = 0; i < a2; ++i )
  {
    if ( *(char *)(96 * i + a1 + 52) != 1 && !strcmp(*(const char **)(96 * i + a1), "-") )
      return 1;
  }
  return 0;
}



// Function: wd_hasher @ 0x705a
unsigned long long wd_hasher(long long a1, unsigned long long a2)
{
  return *(int *)(a1 + 68) % a2;
}



// Function: wd_comparator @ 0x7089
bool wd_comparator(long long a1, long long a2)
{
  return *(int *)(a1 + 68) == *(int *)(a2 + 68);
}



// Function: check_fspec @ 0x70be
unsigned long long check_fspec(long long a1, long long *a2)
{
  long long v2; // rbx
  char *v3; // rax
  long long v4; // rax
  long long v5; // rdx
  bool v6; // al
  char *v7; // rbx
  int *v8; // rax
  char *v10; // [rsp+20h] [rbp-C0h]
  long long v11; // [rsp+28h] [rbp-B8h]
  struct stat buf; // [rsp+30h] [rbp-B0h] BYREF
  unsigned long long v13; // [rsp+C8h] [rbp-18h]

  v13 = __readfsqword(0x28u);
  if ( *(int *)(a1 + 56) != -1 )
  {
    v10 = sub_43EE((const char **)a1);
    if ( fstat(*(int *)(a1 + 56), &buf) )
    {
      *(int *)(a1 + 60) = *__errno_location();
      sub_44D5(*(int *)(a1 + 56), (long long)v10);
      *(int *)(a1 + 56) = -1;
      return v13 - __readfsqword(0x28u);
    }
    if ( (*(int *)(a1 + 48) & 0xF000) == 0x8000 && buf.st_size < *(long long *)(a1 + 8) )
    {
      v2 = sub_EA5C(0, 3, v10);
      v3 = gettext("%s: file truncated");
      error(0, 0, v3, v2);
      sub_4804(*(int *)(a1 + 56), 0, 0, (long long)v10);
      *(long long *)(a1 + 8) = 0;
    }
    else if ( (*(int *)(a1 + 48) & 0xF000) == 0x8000 && buf.st_size == *(long long *)(a1 + 8) )
    {
      v4 = sub_EE31(&buf);
      if ( !(unsigned int)sub_EEAF(*(long long *)(a1 + 16), *(long long *)(a1 + 24), v4, v5) )
        return v13 - __readfsqword(0x28u);
    }
    v6 = byte_18131 && a1 != *a2;
    v11 = sub_4649(v6, v10, *(int *)(a1 + 56), 0xFFFFFFFFFFFFFFFFLL);
    *(long long *)(a1 + 8) += v11;
    if ( v11 )
    {
      *a2 = a1;
      if ( fflush_unlocked(stdout) )
      {
        v7 = gettext("write error");
        v8 = __errno_location();
        error(1, *v8, v7);
      }
    }
  }
  return v13 - __readfsqword(0x28u);
}



// Function: tail_forever_inotify @ 0x7379
unsigned long long tail_forever_inotify(unsigned int a1, long long a2, unsigned long long a3, long long *a4, double a5)
{
  const char *v5; // rcx
  long long v6; // r12
  char *v7; // rbx
  int *v8; // rax
  char *v9; // rax
  char *v10; // rax
  long long v11; // r12
  char *v12; // rbx
  int *v13; // rax
  char *v14; // rax
  long long v15; // r12
  char *v16; // rbx
  int *v17; // rax
  char *v18; // rax
  bool v19; // al
  char *v20; // rbx
  int *v21; // rax
  int v22; // eax
  char *v23; // rbx
  int *v24; // rax
  char *v25; // rax
  char *v26; // rax
  long long v27; // r12
  char *v28; // rbx
  int *v29; // rax
  char *v31; // rax
  char v35; // [rsp+3Ch] [rbp-1A4h]
  char v36; // [rsp+3Dh] [rbp-1A3h]
  char v37; // [rsp+3Eh] [rbp-1A2h]
  char v38; // [rsp+3Fh] [rbp-1A1h]
  bool v39; // [rsp+40h] [rbp-1A0h]
  char v40; // [rsp+41h] [rbp-19Fh]
  bool v41; // [rsp+42h] [rbp-19Eh]
  int v42; // [rsp+44h] [rbp-19Ch]
  uint32_t mask; // [rsp+48h] [rbp-198h]
  int timeout; // [rsp+4Ch] [rbp-194h]
  int v45; // [rsp+50h] [rbp-190h]
  int v46; // [rsp+54h] [rbp-18Ch]
  long long v47; // [rsp+58h] [rbp-188h] BYREF
  size_t v48; // [rsp+60h] [rbp-180h]
  long long v49; // [rsp+68h] [rbp-178h]
  unsigned long long v50; // [rsp+70h] [rbp-170h]
  unsigned long long v51; // [rsp+78h] [rbp-168h]
  unsigned long long i; // [rsp+80h] [rbp-160h]
  long long v53; // [rsp+88h] [rbp-158h]
  unsigned long long j; // [rsp+90h] [rbp-150h]
  long long v55; // [rsp+98h] [rbp-148h]
  size_t v56; // [rsp+A0h] [rbp-140h]
  long long v57; // [rsp+A8h] [rbp-138h]
  double v58; // [rsp+B0h] [rbp-130h]
  unsigned long long v59; // [rsp+B8h] [rbp-128h]
  unsigned long long v60; // [rsp+C0h] [rbp-120h]
  const char **v61; // [rsp+C8h] [rbp-118h]
  char v62[68]; // [rsp+D0h] [rbp-110h] BYREF
  int v63; // [rsp+114h] [rbp-CCh]
  struct stat buf; // [rsp+130h] [rbp-B0h] BYREF
  unsigned long long v65; // [rsp+1C8h] [rbp-18h]

  v65 = __readfsqword(0x28u);
  v42 = 3;
  v35 = 0;
  v36 = 0;
  v37 = 0;
  v38 = 0;
  v39 = 0;
  v48 = 0;
  v50 = 0;
  v51 = 0;
  v55 = sub_B9FE(a3, 0, sub_705A, sub_7089, 0);
  if ( !v55 )
    sub_FFA4();
  *a4 = v55;
  mask = 2;
  if ( dword_18010 == 1 )
    mask = 3078;
  for ( i = 0; i < a3; ++i )
  {
    if ( *(char *)(96 * i + a2 + 52) != 1 )
    {
      v56 = strlen(*(const char **)(96 * i + a2));
      if ( v48 < v56 )
        v48 = v56;
      *(int *)(96 * i + a2 + 68) = -1;
      if ( dword_18010 == 1 )
      {
        v57 = sub_A943(*(long long *)(96 * i + a2));
        v40 = *(char *)(*(long long *)(96 * i + a2) + v57);
        *(long long *)(96 * i + a2 + 80) = sub_A685(*(long long *)(96 * i + a2)) - *(long long *)(96 * i + a2);
        *(char *)(*(long long *)(96 * i + a2) + v57) = 0;
        v5 = v57 ? *(const char **)(96 * i + a2) : ".";
        *(int *)(96 * i + a2 + 72) = inotify_add_watch(a1, v5, 0x784u);
        *(char *)(v57 + *(long long *)(96 * i + a2)) = v40;
        if ( *(int *)(96 * i + a2 + 72) < 0 )
        {
          if ( *__errno_location() == 28 )
          {
            v9 = gettext("inotify resources exhausted");
            error(0, 0, v9);
          }
          else
          {
            v6 = sub_E8CA(4, *(long long *)(96 * i + a2));
            v7 = gettext("cannot watch parent directory of %s");
            v8 = __errno_location();
            error(0, *v8, v7, v6);
          }
          v37 = 1;
          break;
        }
      }
      *(int *)(96 * i + a2 + 68) = inotify_add_watch(a1, *(const char **)(96 * i + a2), mask);
      if ( *(int *)(96 * i + a2 + 68) >= 0 )
      {
        if ( !sub_C73F(v55, a2 + 96 * i) )
          sub_FFA4();
        v35 = 1;
      }
      else
      {
        if ( *(int *)(96 * i + a2 + 56) != -1 )
          v36 = 1;
        if ( *__errno_location() == 28 || *__errno_location() == 12 )
        {
          v38 = 1;
          v10 = gettext("inotify resources exhausted");
          error(0, 0, v10);
          break;
        }
        if ( *__errno_location() != *(int *)(96 * i + a2 + 60) )
        {
          v11 = sub_E8CA(4, *(long long *)(96 * i + a2));
          v12 = gettext("cannot watch %s");
          v13 = __errno_location();
          error(0, *v13, v12, v11);
        }
      }
    }
  }
  if ( !v38 && !v37 && (dword_18010 != 2 || !v36) )
  {
    if ( dword_18010 == 2 && v35 != 1 )
      exit(1);
    v47 = 96 * a3 - 96 + a2;
    for ( i = 0; i < a3; ++i )
    {
      if ( *(char *)(96 * i + a2 + 52) != 1 )
      {
        if ( dword_18010 == 1 )
        {
          sub_5A69(96 * i + a2, 0);
        }
        else if ( *(int *)(96 * i + a2 + 56) != -1
               && !stat(*(const char **)(96 * i + a2), &buf)
               && (*(long long *)(96 * i + a2 + 32) != buf.st_dev || *(long long *)(96 * i + a2 + 40) != buf.st_ino) )
        {
          v14 = sub_43EE((const char **)(96 * i + a2));
          v15 = sub_E8CA(4, v14);
          v16 = gettext("%s was replaced");
          v17 = __errno_location();
          error(0, *v17, v16, v15);
          return v65 - __readfsqword(0x28u);
        }
        sub_70BE(a2 + 96 * i, &v47);
      }
    }
    v48 += 17LL;
    v49 = sub_F96C(v48);
    while ( 1 )
    {
      if ( dword_18010 == 1 && byte_1812C != 1 && !sub_B071(v55) )
      {
        v18 = gettext("no files remaining");
        error(1, 0, v18);
      }
      if ( v51 > v50 )
        goto LABEL_81;
      do
      {
        timeout = -1;
        if ( pid )
        {
          if ( v39 )
            exit(0);
          v19 = kill(pid, 0) && *__errno_location() != 1;
          v39 = v19;
          if ( v19 || a5 <= 0.0 )
          {
            timeout = 0;
          }
          else if ( a5 < 2147482.0 )
          {
            v58 = 1000.0 * a5;
            timeout = (1000.0 * a5 > (double)(int)(1000.0 * a5)) + (int)(1000.0 * a5);
          }
        }
        LODWORD(buf.st_dev) = a1;
        WORD2(buf.st_dev) = 1;
        buf.st_ino = 1;
        v46 = poll((struct pollfd *)&buf, (unsigned char)byte_1812F + 1, timeout);
      }
      while ( !v46 );
      if ( v46 < 0 )
      {
        v20 = gettext("error waiting for inotify and output events");
        v21 = __errno_location();
        error(1, *v21, v20);
      }
      if ( HIWORD(buf.st_ino) )
        sub_431C();
      v51 = sub_ED34(a1, v49, v48);
      v50 = 0;
      if ( (!v51 || v51 == -1 && *__errno_location() == 22) && (v22 = v42, --v42, v22) )
      {
        v51 = 0;
        v48 *= 2LL;
        v49 = sub_F9D6(v49, v48);
      }
      else
      {
        if ( !v51 || v51 == -1 )
        {
          v23 = gettext("error reading inotify event");
          v24 = __errno_location();
          error(1, *v24, v23);
        }
LABEL_81:
        v59 = v49 + v50;
        v60 = v49 + v50;
        v50 += *(unsigned int *)(v49 + v50 + 12) + 16LL;
        if ( (*(int *)(v60 + 4) & 0x400) != 0 && !*(int *)(v60 + 12) )
        {
          for ( i = 0; i < a3; ++i )
          {
            if ( *(int *)v60 == *(int *)(96 * i + a2 + 72) )
            {
              v25 = gettext("directory containing watched file was removed");
              error(0, 0, v25);
              return v65 - __readfsqword(0x28u);
            }
          }
        }
        if ( !*(int *)(v60 + 12) )
        {
          v63 = *(int *)v60;
          v53 = sub_B349(v55, v62);
          goto LABEL_122;
        }
        for ( j = 0;
              j < a3
           && (*(int *)(96 * j + a2 + 72) != *(int *)v60
            || strcmp(
                 (const char *)(v60 + 16),
                 (const char *)(*(long long *)(96 * j + a2) + *(long long *)(96 * j + a2 + 80))));
              ++j )
        {
          ;
        }
        if ( j != a3 )
        {
          v53 = 96 * j + a2;
          v45 = -1;
          v41 = (*(int *)(v60 + 4) & 0x200) != 0;
          if ( (*(int *)(v60 + 4) & 0x200) == 0 )
            v45 = inotify_add_watch(a1, *(const char **)(96 * j + a2), mask);
          if ( !v41 && v45 < 0 )
          {
            if ( *__errno_location() == 28 || *__errno_location() == 12 )
            {
              v26 = gettext("inotify resources exhausted");
              error(0, 0, v26);
              return v65 - __readfsqword(0x28u);
            }
            v27 = sub_E8CA(4, *(long long *)(96 * j + a2));
            v28 = gettext("cannot watch %s");
            v29 = __errno_location();
            error(0, *v29, v28, v27);
          }
          if ( v41 || *(int *)(v53 + 68) >= 0 && v45 == *(int *)(v53 + 68) )
            goto LABEL_119;
          if ( *(int *)(v53 + 68) >= 0 )
          {
            inotify_rm_watch(a1, *(int *)(v53 + 68));
            sub_C7AF(v55, v53);
          }
          *(int *)(v53 + 68) = v45;
          if ( v45 != -1 )
          {
            v61 = (const char **)sub_C7AF(v55, v53);
            if ( v61 && v61 != (const char **)v53 )
            {
              if ( dword_18010 == 1 )
                sub_5A69((long long)v61, 0);
              *((int *)v61 + 17) = -1;
              v31 = sub_43EE(v61);
              sub_44D5(*((int *)v61 + 14), (long long)v31);
            }
            if ( !sub_C73F(v55, v53) )
              sub_FFA4();
LABEL_119:
            if ( dword_18010 == 1 )
              sub_5A69(v53, 0);
LABEL_122:
            if ( v53 )
            {
              if ( (*(int *)(v60 + 4) & 0xE04) != 0 )
              {
                if ( (*(int *)(v60 + 4) & 0x400) != 0 )
                {
                  inotify_rm_watch(a1, *(int *)(v53 + 68));
                  sub_C7AF(v55, v53);
                }
                sub_5A69(v53, 0);
              }
              else
              {
                sub_70BE(v53, &v47);
              }
            }
          }
        }
      }
    }
  }
  return v65 - __readfsqword(0x28u);
}



// Function: tail_bytes @ 0x8520
long long tail_bytes(const char *a1, unsigned int a2, long long a3, __off_t *a4)
{
  long long v4; // r12
  char *v5; // rbx
  int *v6; // rax
  long long v8; // rax
  __blksize_t st_blksize; // rax
  unsigned int v12; // [rsp+24h] [rbp-CCh]
  __off_t st_size; // [rsp+28h] [rbp-C8h]
  __off_t v14; // [rsp+30h] [rbp-C0h]
  struct stat buf; // [rsp+40h] [rbp-B0h] BYREF
  unsigned long long v16; // [rsp+D8h] [rbp-18h]

  v16 = __readfsqword(0x28u);
  if ( fstat(a2, &buf) )
  {
    v4 = sub_E8CA(4, a1);
    v5 = gettext("cannot fstat %s");
    v6 = __errno_location();
    error(0, *v6, v5, v4);
    return 0;
  }
  if ( byte_18130 )
  {
    if ( byte_18139 != 1
      && a3 >= 0
      && ((buf.st_mode & 0xF000) == 0x8000 && sub_4804(a2, a3, 1, (long long)a1) >= 0 || lseek(a2, a3, 1) != -1) )
    {
      *a4 += a3;
    }
    else
    {
      v12 = sub_55AE((long long)a1, a2, a3, a4);
      if ( v12 )
        return v12 >> 31;
    }
    v8 = sub_4649(0, a1, a2, 0xFFFFFFFFFFFFFFFFLL);
  }
  else
  {
    st_size = -1;
    v14 = -1;
    if ( byte_18139 != 1 && a3 >= 0 )
    {
      if ( (unsigned char)sub_2D62(&buf) )
      {
        st_size = buf.st_size;
      }
      else
      {
        v14 = lseek(a2, -a3, 2);
        if ( v14 != -1 )
          st_size = v14 + a3;
      }
    }
    if ( buf.st_blksize <= 0 || buf.st_blksize > 0x2000000000000000uLL )
      st_blksize = 512;
    else
      st_blksize = buf.st_blksize;
    if ( st_blksize >= st_size )
      return sub_5286((long long)a1, a2, a3, a4);
    if ( v14 == -1 )
      v14 = sub_4804(a2, 0, 1, (long long)a1);
    if ( v14 < st_size && a3 < (unsigned long long)(st_size - v14) )
    {
      v14 = st_size - a3;
      sub_4804(a2, st_size - a3, 0, (long long)a1);
    }
    *a4 = v14;
    v8 = sub_4649(0, a1, a2, a3);
  }
  *a4 += v8;
  return 1;
}



// Function: tail_lines @ 0x88c7
long long tail_lines(const char *a1, unsigned int a2, size_t a3, __off_t *a4)
{
  long long v4; // r12
  char *v5; // rbx
  int *v6; // rax
  unsigned int v10; // [rsp+2Ch] [rbp-C4h]
  __off_t v11; // [rsp+30h] [rbp-C0h]
  __off_t v12; // [rsp+38h] [rbp-B8h]
  struct stat buf; // [rsp+40h] [rbp-B0h] BYREF
  unsigned long long v14; // [rsp+D8h] [rbp-18h]

  v14 = __readfsqword(0x28u);
  if ( fstat(a2, &buf) )
  {
    v4 = sub_E8CA(4, a1);
    v5 = gettext("cannot fstat %s");
    v6 = __errno_location();
    error(0, *v6, v5, v4);
    return 0;
  }
  if ( byte_18130 )
  {
    v10 = sub_5741((long long)a1, a2, a3, a4);
    if ( v10 )
      return v10 >> 31;
    *a4 += sub_4649(0, a1, a2, 0xFFFFFFFFFFFFFFFFLL);
    return 1;
  }
  v11 = -1;
  if ( byte_18139 != 1 && (buf.st_mode & 0xF000) == 0x8000 )
  {
    v11 = lseek(a2, 0, 1);
    if ( v11 != -1 )
    {
      v12 = lseek(a2, 0, 2);
      if ( v12 > v11 )
      {
        *a4 = v12;
        return !v12 || (unsigned char)sub_49BD(a1, a2, a3, v11, v12, a4) == 1;
      }
    }
  }
  if ( v11 != -1 )
    sub_4804(a2, v11, 0, (long long)a1);
  return sub_4DF7((long long)a1, a2, a3, a4);
}



// Function: tail @ 0x8b67
long long tail(const char *a1, unsigned int a2, size_t a3, __off_t *a4)
{
  *a4 = 0;
  if ( byte_1812D )
    return sub_88C7(a1, a2, a3, a4);
  else
    return sub_8520(a1, a2, a3, a4);
}



// Function: tail_file @ 0x8bca
long long tail_file(long long a1, size_t a2)
{
  int v2; // eax
  int v3; // edx
  int v4; // ecx
  int v5; // r8d
  int v6; // r9d
  bool v7; // al
  char *v8; // rax
  long long v9; // r12
  char *v10; // rbx
  int *v11; // rax
  char *v12; // rax
  char *v13; // rax
  char *v14; // rax
  long long v15; // r12
  char *v16; // rbx
  int *v17; // rax
  char *v18; // rbx
  char *v19; // rax
  long long v20; // r12
  char *v21; // rax
  char *v22; // rax
  int v23; // ecx
  char *v24; // rax
  char *v25; // rax
  long long v26; // r12
  char *v27; // rbx
  int *v28; // rax
  unsigned char v30; // [rsp+12h] [rbp-BEh]
  bool v31; // [rsp+13h] [rbp-BDh]
  int fd; // [rsp+14h] [rbp-BCh]
  __off_t v33; // [rsp+18h] [rbp-B8h] BYREF
  struct stat buf; // [rsp+20h] [rbp-B0h] BYREF
  unsigned long long v35; // [rsp+B8h] [rbp-18h]

  v35 = __readfsqword(0x28u);
  v2 = strcmp(*(const char **)a1, "-");
  v31 = v2 == 0;
  if ( v2 )
  {
    fd = sub_AF09(*(long long *)a1, 0, v3, v4, v5, v6);
  }
  else
  {
    byte_18138 = 1;
    fd = 0;
    sub_FFEF(0, 0);
  }
  v7 = byte_1812C != 1 || fd != -1;
  *(char *)(a1 + 54) = v7;
  if ( fd == -1 )
  {
    if ( byte_1812E )
    {
      *(int *)(a1 + 56) = -1;
      *(int *)(a1 + 60) = *__errno_location();
      *(char *)(a1 + 52) = byte_1812C == 0;
      *(long long *)(a1 + 40) = 0;
      *(long long *)(a1 + 32) = 0;
    }
    v8 = sub_43EE((const char **)a1);
    v9 = sub_E8CA(4, v8);
    v10 = gettext("cannot open %s for reading");
    v11 = __errno_location();
    error(0, *v11, v10, v9);
    return 0;
  }
  else
  {
    if ( byte_18131 )
    {
      v12 = sub_43EE((const char **)a1);
      sub_4559(v12);
    }
    v13 = sub_43EE((const char **)a1);
    v30 = sub_8B67(v13, fd, a2, &v33);
    if ( byte_1812E )
    {
      *(int *)(a1 + 60) = v30 - 1;
      if ( fstat(fd, &buf) >= 0 )
      {
        if ( (buf.st_mode & 0xF000) != 0x8000
          && (buf.st_mode & 0xF000) != 0x1000
          && (buf.st_mode & 0xF000) != 0xC000
          && (buf.st_mode & 0xF000) != 0x2000 )
        {
          v30 = 0;
          *(int *)(a1 + 60) = -1;
          *(char *)(a1 + 54) = 0;
          *(char *)(a1 + 52) = byte_1812C == 0;
          if ( *(char *)(a1 + 52) )
            v18 = gettext("; giving up on this name");
          else
            v18 = (char *)locale;
          v19 = sub_43EE((const char **)a1);
          v20 = sub_EA5C(0, 3, v19);
          v21 = gettext("%s: cannot follow end of this type of file%s");
          error(0, 0, v21, v20, v18);
        }
      }
      else
      {
        v30 = 0;
        *(int *)(a1 + 60) = *__errno_location();
        v14 = sub_43EE((const char **)a1);
        v15 = sub_E8CA(4, v14);
        v16 = gettext("error reading %s");
        v17 = __errno_location();
        error(0, *v17, v16, v15);
      }
      if ( v30 != 1 )
      {
        *(char *)(a1 + 52) = byte_1812C == 0;
        v22 = sub_43EE((const char **)a1);
        sub_44D5(fd, (long long)v22);
        *(int *)(a1 + 56) = -1;
      }
      else
      {
        if ( v31 )
          v23 = -1;
        else
          v23 = 1;
        sub_4435(a1, fd, v33, (long long)&buf, v23);
        v24 = sub_43EE((const char **)a1);
        *(char *)(a1 + 53) = sub_5938(fd, (long long)v24);
      }
    }
    else if ( !v31 && close(fd) )
    {
      v25 = sub_43EE((const char **)a1);
      v26 = sub_E8CA(4, v25);
      v27 = gettext("error reading %s");
      v28 = __errno_location();
      error(0, *v28, v27, v26);
      return 0;
    }
  }
  return v30;
}



// Function: parse_obsolete_option @ 0x90f5
long long parse_obsolete_option(int a1, long long a2, long long *a3)
{
  bool v4; // al
  char *v5; // rax
  int v6; // eax
  int v7; // eax
  const char *v8; // r12
  char *v9; // rbx
  int *v10; // rax
  char v12; // [rsp+2Bh] [rbp-35h]
  char v13; // [rsp+2Ch] [rbp-34h]
  char v14; // [rsp+2Dh] [rbp-33h]
  bool v15; // [rsp+2Fh] [rbp-31h]
  int v16; // [rsp+30h] [rbp-30h]
  int v17; // [rsp+34h] [rbp-2Ch]
  char *v18; // [rsp+38h] [rbp-28h]
  char *v19; // [rsp+40h] [rbp-20h]
  char *v20; // [rsp+48h] [rbp-18h]

  v16 = 10;
  v13 = 1;
  v14 = 0;
  if ( a1 != 2
    && (a1 != 3 || **(char **)(a2 + 16) == 45 && *(char *)(*(long long *)(a2 + 16) + 1LL))
    && (a1 <= 2 || a1 > 4 || strcmp(*(const char **)(a2 + 16), "--")) )
  {
    return 0;
  }
  v17 = sub_CC1C();
  v4 = v17 <= 200111 || v17 > 200808;
  v15 = v4;
  v5 = *(char **)(a2 + 8);
  v18 = v5 + 1;
  v6 = *v5;
  if ( v6 == 43 )
  {
    if ( !v15 )
      return 0;
    v12 = 1;
  }
  else
  {
    if ( v6 != 45 )
      return 0;
    if ( v17 > 200111 && !v18[*v18 == 99] )
      return 0;
    v12 = 0;
  }
  v19 = v18;
  while ( (unsigned int)((char)*v18 - 48) <= 9 )
    ++v18;
  v20 = v18;
  v7 = (char)*v18;
  if ( v7 == 108 )
  {
LABEL_33:
    ++v18;
    goto LABEL_34;
  }
  if ( v7 <= 108 )
  {
    if ( v7 == 98 )
    {
      v16 = 5120;
    }
    else if ( v7 != 99 )
    {
      goto LABEL_34;
    }
    v13 = 0;
    goto LABEL_33;
  }
LABEL_34:
  if ( *v18 == 102 )
  {
    v14 = 1;
    ++v18;
  }
  if ( *v18 )
    return 0;
  if ( v19 == v20 )
  {
    *a3 = v16;
  }
  else if ( (sub_10410(v19, 0, 10, a3, "b") & 0xFFFFFFFD) != 0 )
  {
    v8 = (const char *)sub_ED11(*(long long *)(a2 + 8));
    v9 = gettext("invalid number");
    v10 = __errno_location();
    error(1, *v10, "%s: %s", v9, v8);
  }
  byte_18130 = v12;
  byte_1812D = v13;
  byte_1812E = v14;
  return 1;
}



// Function: parse_options @ 0x9381
unsigned long long parse_options(int a1, char *const *a2, long long *a3, int *a4, double *a5)
{
  int v5; // eax
  char *v6; // rax
  char *v7; // rax
  char *v8; // rax
  long long v9; // rbx
  char *v10; // rax
  char *v11; // rax
  char *v12; // rax
  char *v13; // rax
  char *v14; // rax
  char *v15; // rax
  unsigned int v20; // [rsp+3Ch] [rbp-24h]
  double v21; // [rsp+40h] [rbp-20h] BYREF
  unsigned long long v22; // [rsp+48h] [rbp-18h]

  v22 = __readfsqword(0x28u);
  v5 = getopt_long(a1, a2, "c:n:fFqs:vz0123456789", &longopts, 0);
  while ( 2 )
  {
    v20 = v5;
    if ( v5 != -1 )
    {
      if ( v5 <= 133 )
      {
        if ( v5 >= 48 )
        {
          switch ( v5 )
          {
            case 48:
            case 49:
            case 50:
            case 51:
            case 52:
            case 53:
            case 54:
            case 55:
            case 56:
            case 57:
              v11 = gettext("option used in invalid context -- %c");
              error(1, 0, v11, v20);
              goto LABEL_35;
            case 70:
              byte_1812E = 1;
              dword_18010 = 1;
              byte_1812C = 1;
              goto LABEL_36;
            case 99:
            case 110:
              byte_1812D = v5 == 110;
              if ( *(char *)optarg == 43 )
              {
                byte_18130 = 1;
              }
              else if ( *(char *)optarg == 45 )
              {
                ++optarg;
              }
              if ( byte_1812D )
                v6 = gettext("invalid number of lines");
              else
                v6 = gettext("invalid number of bytes");
              *a3 = sub_10156(optarg, 0, -1, "bkKmMGTPEZY0", v6, 0);
              goto LABEL_36;
            case 102:
            case 132:
              byte_1812E = 1;
              if ( optarg )
                dword_18010 = dword_12210[sub_A546(
                                            (unsigned int)"--follow",
                                            optarg,
                                            (unsigned int)off_17900,
                                            (unsigned int)dword_12210,
                                            4,
                                            (int)off_18038,
                                            1)];
              else
                dword_18010 = 2;
              goto LABEL_36;
            case 113:
              *a4 = 2;
              goto LABEL_36;
            case 115:
              if ( (unsigned char)sub_10261(optarg, 0, &v21, sub_A745) != 1 || v21 < 0.0 )
              {
                v9 = sub_ED11(optarg);
                v10 = gettext("invalid number of seconds: %s");
                error(1, 0, v10, v9);
              }
              *a5 = v21;
              goto LABEL_36;
            case 118:
              *a4 = 1;
              goto LABEL_36;
            case 122:
              byte_18132 = 0;
              goto LABEL_36;
            case 128:
              byte_1812C = 1;
              goto LABEL_36;
            case 129:
              v7 = gettext("invalid maximum number of unchanged stats between opens");
              qword_18018 = sub_10156(optarg, 0, -1, locale, v7, 0);
              goto LABEL_36;
            case 130:
              v8 = gettext("invalid PID");
              pid = sub_10156(optarg, 0, 0x7FFFFFFF, locale, v8, 0);
              goto LABEL_36;
            case 131:
              byte_18139 = 1;
              goto LABEL_36;
            case 133:
              byte_1813A = 1;
LABEL_36:
              v5 = getopt_long(a1, a2, "c:n:fFqs:vz0123456789", &longopts, 0);
              continue;
            default:
              goto LABEL_35;
          }
        }
        if ( v5 == -131 )
        {
          sub_F78C(
            (int)stdout,
            (unsigned int)"tail",
            (unsigned int)"GNU coreutils",
            (int)off_18030,
            (unsigned int)"Paul Rubin",
            (unsigned int)"David MacKenzie",
            "Ian Lance Taylor",
            "Jim Meyering",
            0);
          exit(0);
        }
        if ( v5 == -130 )
          sub_4115(0);
      }
LABEL_35:
      sub_4115(1);
    }
    break;
  }
  if ( byte_1812C )
  {
    if ( byte_1812E != 1 )
    {
      byte_1812C = 0;
      v12 = gettext("warning: --retry ignored; --retry is useful only when following");
      error(0, 0, v12);
    }
    else if ( dword_18010 == 2 )
    {
      v13 = gettext("warning: --retry only effective for the initial open");
      error(0, 0, v13);
    }
  }
  if ( pid && byte_1812E != 1 )
  {
    v14 = gettext("warning: PID ignored; --pid=PID is useful only when following");
    error(0, 0, v14);
  }
  else if ( pid && kill(pid, 0) && *__errno_location() == 38 )
  {
    v15 = gettext("warning: --pid=PID is not supported on this system");
    error(0, 0, v15);
    pid = 0;
  }
  return v22 - __readfsqword(0x28u);
}



// Function: ignore_fifo_and_pipe @ 0x98cb
long long ignore_fifo_and_pipe(long long a1, unsigned long long a2)
{
  long long v4; // [rsp+20h] [rbp-10h]
  unsigned long long i; // [rsp+28h] [rbp-8h]

  v4 = 0;
  for ( i = 0; i < a2; ++i )
  {
    if ( !strcmp(*(const char **)(96 * i + a1), "-")
      && *(char *)(96 * i + a1 + 52) != 1
      && *(int *)(96 * i + a1 + 56) >= 0
      && (*(int *)(96 * i + a1 + 48) & 0xF000) == 0x1000 )
    {
      *(int *)(96 * i + a1 + 56) = -1;
      *(char *)(96 * i + a1 + 52) = 1;
    }
    else
    {
      ++v4;
    }
  }
  return v4;
}



// Function: main @ 0x9a14
long long main(int a1, char **a2, char **a3)
{
  long long v3; // rbx
  char *v4; // rax
  bool v5; // al
  char *v6; // rax
  char *v8; // rbx
  int *v9; // rax
  char *v10; // rbx
  int *v11; // rax
  char *v12; // rbx
  int *v13; // rax
  int *v14; // rax
  char *const *v15; // [rsp+0h] [rbp-110h]
  int v16; // [rsp+Ch] [rbp-104h]
  bool v17; // [rsp+1Ch] [rbp-F4h]
  char v18; // [rsp+1Dh] [rbp-F3h]
  unsigned char v19; // [rsp+1Eh] [rbp-F2h]
  int v20; // [rsp+20h] [rbp-F0h] BYREF
  int fd; // [rsp+24h] [rbp-ECh]
  size_t v22; // [rsp+28h] [rbp-E8h] BYREF
  double v23; // [rsp+30h] [rbp-E0h] BYREF
  long long v24; // [rsp+38h] [rbp-D8h] BYREF
  unsigned long long v25; // [rsp+40h] [rbp-D0h]
  char **v26; // [rsp+48h] [rbp-C8h]
  unsigned long long i; // [rsp+50h] [rbp-C0h]
  long long v28; // [rsp+58h] [rbp-B8h]
  struct stat buf; // [rsp+60h] [rbp-B0h] BYREF
  unsigned long long v30; // [rsp+F8h] [rbp-18h]

  v30 = __readfsqword(0x28u);
  v20 = 0;
  v17 = 1;
  v22 = 10;
  v23 = 1.0;
  sub_CCD0(*a2, a2, a3);
  setlocale(6, locale);
  bindtextdomain("coreutils", "/usr/local/share/locale");
  textdomain("coreutils");
  sub_11700(sub_A84B);
  byte_18138 = 0;
  byte_1812D = 1;
  byte_18131 = 0;
  byte_18130 = 0;
  byte_1812E = 0;
  byte_18132 = 10;
  v19 = sub_90F5(a1, (long long)a2, &v22);
  v16 = a1 - v19;
  v15 = &a2[v19];
  sub_9381(v16, v15, &v22, &v20, &v23);
  if ( byte_18130 && v22 )
    --v22;
  if ( v16 <= optind )
  {
    v25 = 1;
    v26 = &off_18028;
  }
  else
  {
    v25 = v16 - optind;
    v26 = (char **)&v15[optind];
  }
  v18 = 0;
  for ( i = 0; i < v25; ++i )
  {
    if ( !strcmp(v26[i], "-") )
      v18 = 1;
  }
  if ( v18 && dword_18010 == 1 )
  {
    v3 = sub_E8CA(4, "-");
    v4 = gettext("cannot follow %s by name");
    error(1, 0, v4, v3);
  }
  if ( byte_1812E && v18 )
  {
    v5 = !pid && dword_18010 == 2 && v25 == 1 && !fstat(0, &buf) && (buf.st_mode & 0xF000) != 0x8000;
    if ( !v5 && isatty(0) )
    {
      v6 = gettext("warning: following standard input indefinitely is ineffective");
      error(0, 0, v6);
    }
  }
  if ( !v22 && byte_1812E != 1 && byte_18130 != 1 )
    return 0;
  v28 = sub_FAE5(v25, 96);
  for ( i = 0; i < v25; ++i )
    *(long long *)(v28 + 96 * i) = v26[i];
  if ( v20 == 1 || !v20 && v25 > 1 )
    byte_18131 = 1;
  sub_FFEF(1, 0);
  for ( i = 0; i < v25; ++i )
    v17 = (v17 & (unsigned char)sub_8BCA(96 * i + v28, v22)) != 0;
  if ( byte_1812E && sub_98CB(v28, v25) )
  {
    if ( fstat(1, &buf) < 0 )
    {
      v8 = gettext("standard output");
      v9 = __errno_location();
      error(1, *v9, v8);
    }
    byte_1812F = (buf.st_mode & 0xF000) == 4096;
    if ( byte_1813A != 1
      && ((unsigned char)sub_6FC5(v28, v25)
       || (unsigned char)sub_6D57(v28, v25)
       || (unsigned char)sub_6DD3(v28, v25) != 1
       || (unsigned char)sub_6E52(v28, v25)
       || (unsigned char)sub_6F11(v28, v25)
       || !v17 && dword_18010 == 2) )
    {
      byte_1813A = 1;
    }
    if ( byte_1813A != 1 )
    {
      fd = inotify_init();
      if ( fd >= 0 )
      {
        if ( fflush_unlocked(stdout) )
        {
          v10 = gettext("write error");
          v11 = __errno_location();
          error(1, *v11, v10);
        }
        sub_7379(fd, v28, v25, &v24, v23);
        sub_BC7E(v24);
        close(fd);
        *__errno_location() = 0;
      }
      v12 = gettext("inotify cannot be used, reverting to polling");
      v13 = __errno_location();
      error(0, *v13, v12);
    }
    byte_1813A = 1;
    sub_62F3(v28, v25, v23);
  }
  if ( byte_18138 )
  {
    if ( close(0) < 0 )
    {
      v14 = __errno_location();
      error(1, *v14, "-");
    }
  }
  return !v17;
}



// Function: hash_get_n_buckets @ 0xb045
long long hash_get_n_buckets(long long a1)
{
  return *(long long *)(a1 + 16);
}



// Function: hash_get_n_buckets_used @ 0xb05b
long long hash_get_n_buckets_used(long long a1)
{
  return *(long long *)(a1 + 24);
}



// Function: hash_get_n_entries @ 0xb071
long long hash_get_n_entries(long long a1)
{
  return *(long long *)(a1 + 32);
}



// Function: hash_get_max_bucket_length @ 0xb087
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



// Function: hash_table_ok @ 0xb109
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



// Function: hash_print_statistics @ 0xb1a7
int hash_print_statistics(long long a1, FILE *a2)
{
  double v2; // xmm0_8
  double v3; // xmm1_8
  double v4; // xmm0_8
  long long v6; // [rsp+10h] [rbp-20h]
  long long v7; // [rsp+18h] [rbp-18h]
  long long v8; // [rsp+20h] [rbp-10h]
  unsigned long long v9; // [rsp+28h] [rbp-8h]

  v6 = sub_B071(a1);
  v7 = sub_B045(a1);
  v8 = sub_B05B(a1);
  v9 = sub_B087(a1);
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



// Function: safe_hasher @ 0xb2ee
long long safe_hasher(long long a1, long long a2)
{
  unsigned long long v3; // [rsp+18h] [rbp-8h]

  v3 = (*(long long (**)(long long, long long))(a1 + 48))(a2, *(long long *)(a1 + 16));
  if ( v3 >= *(long long *)(a1 + 16) )
    abort();
  return *(long long *)a1 + 16 * v3;
}



// Function: hash_lookup @ 0xb349
long long hash_lookup(long long a1, long long a2)
{
  long long *i; // [rsp+10h] [rbp-10h]
  long long *v4; // [rsp+18h] [rbp-8h]

  v4 = (long long *)sub_B2EE(a1, a2);
  if ( !*v4 )
    return 0;
  for ( i = v4; i; i = (long long *)i[1] )
  {
    if ( a2 == *i || (*(unsigned char (**)(long long, long long))(a1 + 56))(a2, *i) )
      return *i;
  }
  return 0;
}



// Function: hash_get_first @ 0xb3e0
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



// Function: hash_get_next @ 0xb440
long long hash_get_next(long long a1, long long a2)
{
  unsigned long long v3; // [rsp+10h] [rbp-10h]
  unsigned long long v4; // [rsp+18h] [rbp-8h]

  v3 = sub_B2EE(a1, a2);
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



// Function: hash_get_entries @ 0xb4de
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



// Function: hash_do_for_each @ 0xb57e
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



// Function: hash_string @ 0xb616
unsigned long long hash_string(char *a1, unsigned long long a2)
{
  unsigned long long v4; // [rsp+18h] [rbp-8h]

  v4 = 0;
  while ( *a1 )
    v4 = (31 * v4 + (unsigned char)*a1++) % a2;
  return v4;
}



// Function: is_prime @ 0xb670
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



// Function: next_prime @ 0xb6df
unsigned long long next_prime(unsigned long long a1)
{
  long long v2; // [rsp+0h] [rbp-8h]
  unsigned long long i; // [rsp+0h] [rbp-8h]

  v2 = a1;
  if ( a1 <= 9 )
    v2 = 10;
  for ( i = v2 | 1; i != -1 && !sub_B670(i); i += 2LL )
    ;
  return i;
}



// Function: hash_reset_tuning @ 0xb72a
long long hash_reset_tuning(long long a1)
{
  *(int *)a1 = 0;
  *(int *)(a1 + 4) = 1065353216;
  *(int *)(a1 + 8) = 1061997773;
  *(int *)(a1 + 12) = 1068826100;
  *(char *)(a1 + 16) = 0;
  return a1;
}



// Function: raw_hasher @ 0xb780
unsigned long long raw_hasher(long long a1, unsigned long long a2)
{
  return sub_10957(a1, 3) % a2;
}



// Function: raw_comparator @ 0xb7bb
bool raw_comparator(long long a1, long long a2)
{
  return a1 == a2;
}



// Function: check_tuning @ 0xb7d8
long long check_tuning(long long a1)
{
  long long v2; // [rsp+10h] [rbp-8h]

  v2 = *(long long *)(a1 + 40);
  if ( (_UNKNOWN *)v2 == &unk_135F0 )
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
  *(long long *)(a1 + 40) = &unk_135F0;
  return 0;
}



// Function: compute_bucket_size @ 0xb8e5
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
  v6 = sub_B6DF(v5);
  v4 = v6 >> 61 != 0;
  if ( (v6 & 0x1000000000000000LL) != 0 )
    v4 = 1;
  if ( v4 )
    return 0;
  else
    return v6;
}



// Function: hash_initialize @ 0xb9fe
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
    v8 = sub_B780;
  if ( !a4 )
    v7 = sub_B7BB;
  ptr = (size_t *)malloc(0x50u);
  if ( !ptr )
    return 0;
  if ( !a2 )
    v9 = &unk_135F0;
  ptr[5] = (size_t)v9;
  if ( (unsigned char)sub_B7D8((long long)ptr) == 1
    && (ptr[2] = sub_B8E5(a1, (long long)v9)) != 0
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



// Function: hash_clear @ 0xbb73
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



// Function: hash_free @ 0xbc7e
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



// Function: allocate_entry @ 0xbdad
void *allocate_entry(long long a1)
{
  long long v2; // [rsp+18h] [rbp-8h]

  if ( !*(long long *)(a1 + 72) )
    return malloc(0x10u);
  v2 = *(long long *)(a1 + 72);
  *(long long *)(a1 + 72) = *(long long *)(v2 + 8);
  return (void *)v2;
}



// Function: free_entry @ 0xbdfc
long long free_entry(long long a1, long long *a2)
{
  *a2 = 0;
  a2[1] = *(long long *)(a1 + 72);
  *(long long *)(a1 + 72) = a2;
  return a1;
}



// Function: hash_find_entry @ 0xbe36
long long hash_find_entry(long long a1, long long a2, long long *a3, char a4)
{
  long long v5; // rdx
  long long *i; // [rsp+20h] [rbp-30h]
  long long *v9; // [rsp+28h] [rbp-28h]
  long long v10; // [rsp+30h] [rbp-20h]
  long long *v11; // [rsp+38h] [rbp-18h]
  long long v12; // [rsp+40h] [rbp-10h]
  long long *v13; // [rsp+48h] [rbp-8h]

  v9 = (long long *)sub_B2EE(a1, a2);
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
        sub_BDFC(a1, v13);
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
      sub_BDFC(a1, v11);
    }
    return v10;
  }
}



// Function: transfer_entries @ 0xbfcf
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
        v9 = (long long *)sub_B2EE(a1, *j);
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
          sub_BDFC(a1, j);
        }
      }
      v8 = *i;
      i[1] = 0;
      if ( !a3 )
      {
        v10 = (long long *)sub_B2EE(a1, v8);
        if ( *v10 )
        {
          v11 = sub_BDAD(a1);
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



// Function: hash_rehash @ 0xc1a3
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
  nmemb = sub_B8E5(a2, *(long long *)(a1 + 40));
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
  if ( (unsigned char)sub_BFCF((long long)&v4, a1, 0) )
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
    if ( (unsigned char)sub_BFCF(a1, (long long)&v4, 1) != 1 || (unsigned char)sub_BFCF(a1, (long long)&v4, 0) != 1 )
      abort();
    free(v4);
    return 0;
  }
}



// Function: hash_insert_if_absent @ 0xc3ce
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
  v26 = sub_BE36((long long)a1, a2, &v25, 0);
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
      sub_B7D8((long long)a1);
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
        if ( (unsigned char)sub_C1A3((long long)a1, v23) != 1 )
          return 0xFFFFFFFFLL;
        if ( sub_BE36((long long)a1, a2, &v25, 0) )
          abort();
      }
    }
    if ( *v25 )
    {
      v28 = sub_BDAD((long long)a1);
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



// Function: hash_insert @ 0xc73f
long long hash_insert(long long *a1, long long a2)
{
  int v3; // [rsp+1Ch] [rbp-14h]
  long long v4[2]; // [rsp+20h] [rbp-10h] BYREF

  v4[1] = __readfsqword(0x28u);
  v3 = sub_C3CE(a1, a2, v4);
  if ( v3 == -1 )
    return 0;
  if ( v3 )
    return a2;
  return v4[0];
}



// Function: hash_remove @ 0xc7af
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
  v26 = sub_BE36(a1, a2, &v24, 1);
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
      sub_B7D8(a1);
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
        if ( (unsigned char)sub_C1A3(a1, v19) != 1 )
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



// Function: hash_delete @ 0xcab5
long long hash_delete(long long a1, long long a2)
{
  return sub_C7AF(a1, a2);
}


