// Function: main @ 0xbb10
long long main(long long a1, char **a2, char **a3)
{
  char *v3; // rax
  void *v4; // rax
  char *v6; // rax
  char *v7; // rax
  char *v8; // rax

  sub_CD20(a1, a2, a3);
  sub_48760(*a2);
  setlocale(6, "");
  bindtextdomain("tar", "/usr/local/share/locale");
  textdomain("tar");
  status = 2;
  qword_84D50 = sub_10820;
  dword_84D58 = 0;
  sub_4A590(0, 7);
  v3 = dcgettext(0, "stdout", 5);
  sub_3E420(v3);
  if ( (unsigned int)sub_4BD80() )
  {
    if ( qword_84D50 )
      qword_84D50();
    v8 = dcgettext(0, "failed to assert availability of the standard file descriptors", 5);
    error(0, 0, "%s", v8);
    sub_163D0();
  }
  qword_84C18 = 10;
  v4 = (void *)sub_4C5B0(80);
  qword_84830 = 0;
  ptr = v4;
  signal(17, 0);
  sub_2E690((unsigned int)a1, a2);
  sub_26C90();
  if ( filename )
    sub_D850();
  switch ( dword_84BA8 )
  {
    case 0:
      if ( qword_84D50 )
        qword_84D50();
      v7 = dcgettext(0, "You must specify one of the '-Acdtrux', '--delete' or '--test-label' options", 5);
      error(0, 0, v7);
      sub_2C970(2);
    case 1:
    case 2:
    case 8:
      sub_30EF0();
      break;
    case 3:
      sub_143D0();
      break;
    case 4:
      sub_15DC0();
      break;
    case 5:
      sub_10AF0();
      sub_22C60(sub_11730);
      break;
    case 6:
      sub_19B30();
      sub_22C60(sub_19C10);
      sub_18640();
      break;
    case 7:
      sub_22C60(sub_23260);
      break;
    case 9:
      sub_232E0();
      break;
    default:
      break;
  }
  sub_10910();
  if ( byte_84A69 )
    sub_D150();
  if ( dword_84C20 )
    sub_14030();
  if ( filename )
    sub_D960();
  free(ptr);
  sub_31B40();
  sub_26CC0();
  if ( dword_84D58 == 2 )
  {
    v6 = dcgettext(0, "Exiting with failure status due to previous errors", 5);
    error(0, 0, v6);
  }
  if ( stream == stdout )
  {
    sub_3E440();
  }
  else if ( ferror_unlocked(stderr) || fclose(stderr) )
  {
    sub_2F7D0(2);
  }
  return (unsigned int)dword_84D58;
}



// Function: bufmap_locate @ 0xbed0
void *bufmap_locate(unsigned long long a1)
{
  void *i; // r8

  for ( i = qword_84148; i && *(long long *)i && *(long long *)(*(long long *)i + 8LL) << 9 <= a1; i = *(void **)i )
    ;
  return i;
}



// Function: find_zip_program @ 0xbf10
char *find_zip_program(int a1, int *a2)
{
  long long v2; // rax
  int *v3; // rdx
  int v4; // ecx
  long long v5; // r8

  v2 = *a2;
  v3 = (int *)((char *)&unk_81020 + 24 * v2);
  v4 = *v3;
  if ( *v3 )
  {
    while ( 1 )
    {
      v5 = (int)v2;
      LODWORD(v2) = v2 + 1;
      if ( a1 == v4 )
        break;
      v4 = v3[6];
      v3 += 6;
      if ( !v4 )
        goto LABEL_6;
    }
    *a2 = v2;
    return (char *)&unk_81020 + 24 * v5;
  }
  else
  {
LABEL_6:
    *a2 = v2;
    return 0;
  }
}



// Function: compress_option @ 0xbf70
char *compress_option(int a1)
{
  char *result; // rax
  int v2; // [rsp+4h] [rbp-14h] BYREF
  unsigned long long v3; // [rsp+8h] [rbp-10h]

  v3 = __readfsqword(0x28u);
  v2 = 0;
  result = sub_BF10(a1, &v2);
  if ( result )
    return (char *)*((long long *)result + 2);
  return result;
}



// Function: bufmap_free @ 0xbfc0
void bufmap_free(void **a1)
{
  void **v1; // rbx
  void **v2; // rbp

  v1 = (void **)qword_84148;
  if ( qword_84148 && qword_84148 != a1 )
  {
    do
    {
      v2 = v1;
      v1 = (void **)*v1;
      free(v2[2]);
      free(v2);
    }
    while ( v1 && a1 != v1 );
  }
  qword_84148 = v1;
  if ( !v1 )
    qword_84140 = 0;
}



// Function: bufmap_reset @ 0xc030
void bufmap_reset(void **a1, long long a2)
{
  void **v2; // rbx

  v2 = a1;
  sub_BFC0(a1);
  if ( a1 )
  {
    do
    {
      v2[1] = (char *)v2[1] + a2;
      v2[5] = 0;
      v2 = (void **)*v2;
    }
    while ( v2 );
  }
}



// Function: set_volume_start_time @ 0xc070
long long set_volume_start_time()
{
  long long result; // rax

  result = sub_411A0(&tp);
  stru_84A00 = (struct timespec)_mm_loadu_si128((const __m128i *)&tp);
  return result;
}



// Function: print_stats @ 0xc0a0
long long print_stats(long long a1, const char *a2, double a3)
{
  int v4; // ebx
  double v5; // xmm2_8
  const char *v6; // r12
  unsigned long long v7; // rdi
  const char *v8; // rax
  char *v10; // rax
  unsigned long long v11; // rdi
  char v12[656]; // [rsp+10h] [rbp-548h] BYREF
  char v13[664]; // [rsp+2A0h] [rbp-2B8h] BYREF
  unsigned long long v14; // [rsp+538h] [rbp-20h]

  v4 = 0;
  v14 = __readfsqword(0x28u);
  if ( a2 && *a2 )
  {
    v10 = dcgettext(0, a2, 5);
    v4 = __fprintf_chk(a1, 1, "%s: ", v10);
  }
  if ( *(double *)&qword_83F78 > 0.0
    && (v5 = a3 / *(double *)&qword_83F78, a3 / *(double *)&qword_83F78 < 1.844674407370955e19) )
  {
    if ( v5 >= 9.223372036854776e18 )
      v11 = (unsigned int)(int)(v5 - 9.223372036854776e18) ^ 0x8000000000000000LL;
    else
      v11 = (unsigned int)(int)v5;
    v6 = (const char *)sub_426B0(v11, v13, 432, 1, 1);
  }
  else
  {
    v6 = "?";
  }
  if ( a3 >= 9.223372036854776e18 )
    v7 = (unsigned int)(int)(a3 - 9.223372036854776e18) ^ 0x8000000000000000LL;
  else
    v7 = (unsigned int)(int)a3;
  v8 = (const char *)sub_426B0(v7, v12, 432, 1, 1);
  return v4 + (unsigned int)__fprintf_chk(a1, (char *)&dword_0 + 1, "%.0f (%s, %s/s)", a3, v8, v6);
}



// Function: increase_volume_number @ 0xc240
void increase_volume_number(long long a1)
{
  char *v1; // rax

  if ( ++dword_81010 < 0 )
  {
    if ( qword_84D50 )
      qword_84D50(a1);
    v1 = dcgettext(0, "Volume number overflow", 5);
    error(0, 0, v1);
    sub_163D0(0);
  }
  ++dword_81014;
}



// Function: backspace_output @ 0xc290
void *backspace_output()
{
  bool v0; // al
  int *v1; // rax
  int v2; // edi
  __off_t v3; // rbx
  long long v4; // rdi
  __off_t v5; // rbx
  __off_t v6; // rax
  char *v7; // rax
  bool v9; // al
  short v10[2]; // [rsp+0h] [rbp-28h] BYREF
  int v11; // [rsp+4h] [rbp-24h]
  unsigned long long v12; // [rsp+8h] [rbp-20h]

  v12 = __readfsqword(0x28u);
  v11 = 1;
  v10[0] = 4;
  if ( fd <= 0x3FFFFFFF )
    v0 = ioctl(fd, 0x40086D01u, v10) >= 0;
  else
    v0 = (int)sub_34080((unsigned int)(fd - 0x40000000), 1074294017, v10) >= 0;
  if ( v0 )
    return (void *)(v12 - __readfsqword(0x28u));
  v1 = __errno_location();
  v2 = fd;
  if ( *v1 == 5 )
  {
    v9 = fd > 0x3FFFFFFF
       ? (int)sub_34080((unsigned int)(fd - 0x40000000), 1074294017, v10) >= 0
       : ioctl(fd, 0x40086D01u, v10) >= 0;
    v2 = fd;
    if ( v9 )
      return (void *)(v12 - __readfsqword(0x28u));
  }
  if ( v2 <= 0x3FFFFFFF )
    v3 = lseek(v2, 0, 1);
  else
    v3 = sub_33F90((unsigned int)(v2 - 0x40000000), 0, 1);
  v4 = (unsigned int)fd;
  v5 = v3 - ((char *)qword_83F50 - (char *)s);
  if ( v5 < 0 )
    v5 = 0;
  if ( fd > 0x3FFFFFFF )
  {
    v4 = (unsigned int)(fd - 0x40000000);
    v6 = sub_33F90(v4, v5, 0);
  }
  else
  {
    v6 = lseek(fd, v5, 0);
  }
  if ( v6 == v5 )
    return (void *)(v12 - __readfsqword(0x28u));
  if ( qword_84D50 )
    qword_84D50(v4);
  v7 = dcgettext(0, "Cannot backspace archive file; it may be unreadable without -i", 5);
  error(0, 0, v7);
  if ( s == (void *)qword_84CF0 )
    return (void *)(v12 - __readfsqword(0x28u));
  else
    return memset(s, 0, qword_84CF0 - (long long)s);
}



// Function: _flush_write @ 0xc490
unsigned long long _flush_write()
{
  unsigned long long v0; // r12
  void **v1; // rax
  void **v2; // rdi
  long long v3; // rsi
  unsigned long long v4; // rdx
  unsigned long long v5; // rax
  unsigned long long v6; // rcx
  void *v7; // rax

  sub_108D0(1);
  if ( *(double *)&qword_84A70 == 0.0 || *(double *)&qword_841A0 < *(double *)&qword_84A70 )
  {
    v0 = nbytes;
    if ( !byte_84A30 )
      v0 = sub_2AF00(*(double *)&qword_84A70);
    if ( v0 )
    {
      if ( byte_84B10 )
      {
        if ( !dword_84138 )
        {
          v1 = (void **)sub_BED0(v0);
          v2 = v1;
          if ( v1 )
          {
            v3 = (long long)v1[1];
            v4 = v0 - (v3 << 9);
            v5 = (unsigned long long)v1[4];
            v6 = (unsigned long long)v2[5] + (v4 >> 9);
            if ( v5 <= v4 )
              v4 = (unsigned long long)v2[4];
            v2[5] = (void *)v6;
            v7 = (void *)(v5 - v4);
            v2[4] = v7;
            if ( !v7 )
            {
              v2 = (void **)*v2;
              v3 += v6;
            }
            sub_C030(v2, -v3);
          }
        }
      }
    }
    return v0;
  }
  else
  {
    *__errno_location() = 28;
    return 0;
  }
}



// Function: change_tape_menu @ 0xc580
unsigned long long change_tape_menu(FILE *stream)
{
  long long v2; // r14
  unsigned int v3; // r15d
  char *v4; // rax
  char *v5; // r8
  char v6; // al
  const char *v7; // rsi
  char *v8; // rax
  char v9; // al
  char *v10; // rdi
  char *v11; // rax
  char v12; // dl
  char *v13; // rax
  const char *v14; // rsi
  char *v15; // rax
  char *v16; // rax
  char *v17; // rax
  char *v18; // rax
  long long *v19; // rbx
  char *lineptr; // [rsp+8h] [rbp-50h] BYREF
  size_t n; // [rsp+10h] [rbp-48h] BYREF
  unsigned long long v23; // [rsp+18h] [rbp-40h]

  v23 = __readfsqword(0x28u);
  lineptr = 0;
  n = 0;
  while ( 1 )
  {
    while ( 1 )
    {
      while ( 1 )
      {
        fputc_unlocked(7, stderr);
        v2 = sub_4AC50(*(long long *)qword_84828);
        v3 = dword_81010 + 1;
        v4 = dcgettext(0, "Prepare volume #%d for %s and hit return: ", 5);
        __fprintf_chk(stderr, 1, v4, v3, v2);
        fflush_unlocked(stderr);
        if ( getline(&lineptr, &n, stream) <= 0 )
        {
          if ( qword_84D50 )
            qword_84D50(&lineptr);
          v14 = "EOF where user reply was expected";
          goto LABEL_27;
        }
        v5 = lineptr;
        v6 = *lineptr;
        if ( (*lineptr & 0xDF) == 0x59 || v6 == 10 )
          goto LABEL_37;
        if ( v6 != 63 )
          break;
        v17 = dcgettext(
                0,
                " n name        Give a new file name for the next (and subsequent) volume(s)\n"
                " q             Abort tar\n"
                " y or newline  Continue operation\n",
                5);
        __fprintf_chk(stderr, 1, v17);
        if ( !byte_84A58 )
        {
          v18 = dcgettext(0, " !             Spawn a subshell\n", 5);
          __fprintf_chk(stderr, 1, v18);
        }
        v7 = " ?             Print this list\n";
LABEL_10:
        v8 = dcgettext(0, v7, 5);
        __fprintf_chk(stderr, 1, v8);
      }
      if ( v6 > 63 )
        break;
      if ( v6 != 33 || byte_84A58 )
        goto LABEL_9;
      sub_2ADD0();
    }
    if ( v6 != 110 )
    {
      if ( v6 != 113 )
      {
LABEL_9:
        v7 = "Invalid input. Type ? for help.\n";
        goto LABEL_10;
      }
      if ( qword_84D50 )
        qword_84D50(&lineptr);
      v14 = "No new volume; exiting.\n";
LABEL_27:
      v15 = dcgettext(0, v14, 5);
      error(0, 0, v15);
      if ( (unsigned int)(dword_84BA8 - 5) > 2 )
      {
        if ( qword_84D50 )
          qword_84D50(0);
        v16 = dcgettext(0, "WARNING: Archive is incomplete", 5);
        error(0, 0, v16);
      }
      sub_163D0(0);
    }
    v9 = lineptr[1];
    v10 = lineptr + 1;
    if ( v9 == 9 || v9 == 32 )
    {
      do
      {
        do
          v9 = *++v10;
        while ( v9 == 32 );
      }
      while ( v9 == 9 );
    }
    if ( v9 == 10 || !v9 )
    {
      v11 = v10;
    }
    else
    {
      v11 = v10;
      do
        v12 = *++v11;
      while ( v12 && v12 != 10 );
    }
    *v11 = 0;
    if ( *v10 )
      break;
    v13 = dcgettext(0, "File name not specified. Try again.\n", 5);
    __fprintf_chk(stderr, 1, "%s", v13);
  }
  v19 = (long long *)qword_84828;
  *v19 = sub_4C7A0(v10);
  v5 = lineptr;
LABEL_37:
  free(v5);
  return v23 - __readfsqword(0x28u);
}



// Function: guess_seekable_archive @ 0xc8d0
unsigned long long guess_seekable_archive()
{
  bool v0; // al
  struct stat v2; // [rsp+0h] [rbp-A8h] BYREF
  unsigned long long v3; // [rsp+98h] [rbp-10h]

  v3 = __readfsqword(0x28u);
  if ( dword_84BA8 == 4 )
    byte_847F8 = 0;
  if ( dword_847FC == -1 )
    v0 = !byte_84B10 && !qword_84B70 && !fstat(fd, &v2) && (v2.st_mode & 0xF000) == 0x8000;
  else
    v0 = dword_847FC != 0;
  byte_847F8 = v0;
  return v3 - __readfsqword(0x28u);
}



// Function: init_buffer @ 0xc980
char *init_buffer()
{
  long long v0; // rbp
  char *v1; // rax
  char *result; // rax

  v0 = dword_84178;
  v1 = (char *)qword_84180[dword_84178];
  if ( !v1 )
  {
    qword_84180[v0] = sub_25000(&qword_84190 + dword_84178, nbytes);
    v1 = (char *)qword_84180[dword_84178];
  }
  s = v1;
  src = v1;
  result = &v1[512 * (long long)dword_84BA0];
  qword_83F50 = result;
  return result;
}



// Function: archive_is_dev @ 0xca00
bool archive_is_dev()
{
  struct stat v1; // [rsp+0h] [rbp-A8h] BYREF
  unsigned long long v2; // [rsp+98h] [rbp-10h]

  v2 = __readfsqword(0x28u);
  if ( !fstat(fd, &v1) )
    return (v1.st_mode & 0xB000) == 0x2000;
  sub_24EC0(*(long long *)qword_84828);
  return 0;
}



// Function: check_tty @ 0xca80
int check_tty(int a1)
{
  int result; // eax
  long long v3; // rdi
  char *v4; // rdx

  result = strcmp(*(const char **)ptr, "-");
  if ( !result )
  {
    v3 = a1 != 0;
    result = isatty(v3);
    if ( result )
    {
      if ( qword_84D50 )
        qword_84D50(v3);
      if ( a1 )
        v4 = dcgettext(0, "Refusing to write archive contents to terminal (missing -f option?)", 5);
      else
        v4 = dcgettext(0, "Refusing to read archive contents from terminal (missing -f option?)", 5);
      error(0, 0, v4);
      sub_163D0(0);
    }
  }
  return result;
}



// Function: add_chunk_header @ 0xcb00
unsigned long long add_chunk_header(long long a1)
{
  void *v2; // rax
  long long v3; // rdx
  long long v4; // rax
  void *ptr[55]; // [rsp+0h] [rbp-1D8h] BYREF
  unsigned long long v6; // [rsp+1B8h] [rbp-20h]

  v6 = __readfsqword(0x28u);
  if ( dword_84BA4 == 4 )
  {
    memset(ptr, 0, sizeof(ptr));
    v2 = *(void **)(a1 + 16);
    LODWORD(ptr[14]) = 33188;
    ptr[1] = v2;
    ptr[0] = v2;
    HIDWORD(ptr[14]) = getuid();
    LODWORD(ptr[15]) = getgid();
    ptr[0] = (void *)sub_1C080(ptr, "%d/GNUFileParts/%f.%n", dword_81014);
    ptr[1] = ptr[0];
    ptr[17] = *(void **)(a1 + 32);
    ptr[35] = ptr[17];
    v3 = sub_132D0(ptr);
    if ( !v3 )
      abort();
    v4 = sub_12DA0(0, ptr, v3);
    sub_12F00(v4);
    free(ptr[0]);
  }
  return v6 - __readfsqword(0x28u);
}



// Function: mv_begin_write @ 0xcc00
void mv_begin_write(void *src, long long a2, long long a3)
{
  long long *v4; // rbx
  signed long long v5; // rax
  long long v6; // rax

  if ( byte_84B10 )
  {
    v4 = (long long *)sub_4C5B0(48);
    if ( qword_84140 )
      *(long long *)qword_84140 = v4;
    else
      qword_84148 = v4;
    qword_84140 = (long long)v4;
    v5 = ((char *)::src - (char *)s) >> 9;
    *v4 = 0;
    v4[1] = v5;
    v6 = sub_4C7A0(src);
    v4[3] = a2;
    v4[4] = a3;
    v4[2] = v6;
    v4[5] = 0;
  }
}



// Function: buffer_write_global_xheader @ 0xcca0
long long buffer_write_global_xheader()
{
  return sub_1CEB0(&unk_840E8);
}



// Function: mv_begin_read @ 0xccb0
void mv_begin_read(long long a1)
{
  sub_CC00(*(void **)a1, *(long long *)(a1 + 136), *(long long *)(a1 + 136));
}



// Function: mv_end @ 0xccd0
void mv_end()
{
  if ( byte_84B10 )
    sub_BFC0(0);
}



// Function: mv_size_left @ 0xccf0
void *mv_size_left(long long a1)
{
  void *result; // rax

  result = qword_84148;
  if ( qword_84148 )
    *((long long *)qword_84148 + 4) = a1;
  return result;
}



// Function: clear_read_error_count @ 0xcd10
void clear_read_error_count()
{
  dword_84164 = 0;
}



// Function: set_start_time @ 0xcd20
long long set_start_time()
{
  long long result; // rax

  result = sub_411A0(&stru_84A20);
  tp = (struct timespec)_mm_loadu_si128((const __m128i *)&stru_84A20);
  stru_84A00 = tp;
  return result;
}



// Function: compute_duration @ 0xcd50
unsigned long long compute_duration()
{
  struct timespec v1; // [rsp+0h] [rbp-28h] BYREF
  unsigned long long v2; // [rsp+18h] [rbp-10h]

  v2 = __readfsqword(0x28u);
  sub_411A0(&v1);
  *(double *)&qword_83F78 = (double)(LODWORD(v1.tv_nsec) - LODWORD(stru_84A00.tv_nsec)) / 1000000000.0
                          + (double)(LODWORD(v1.tv_sec) - LODWORD(stru_84A00.tv_sec))
                          + *(double *)&qword_83F78;
  sub_411A0(&stru_84A00);
  return v2 - __readfsqword(0x28u);
}



// Function: first_decompress_program @ 0xcdf0
char *first_decompress_program(int *a1)
{
  char *v1; // r10
  int v3; // r8d
  char *v4; // rax

  v1 = qword_84B70;
  if ( !qword_84B70 && (v3 = dword_83F70) != 0 && (*a1 = 0, (v4 = sub_BF10(v3, a1)) != 0) )
    return (char *)*((long long *)v4 + 1);
  else
    return v1;
}



// Function: next_decompress_program @ 0xce40
long long next_decompress_program(int *a1)
{
  char *v1; // rax

  if ( qword_84B70 )
    return 0;
  v1 = sub_BF10(dword_83F70, a1);
  if ( !v1 )
    return 0;
  else
    return *((long long *)v1 + 1);
}



// Function: format_total_stats @ 0xce80
long long format_total_stats(FILE *stream, const char **a2, int a3, int a4)
{
  int v6; // eax
  double v8; // xmm0_8
  int v9; // r12d
  int v10; // r12d
  const char *v11; // rsi
  int v12; // r12d
  signed long long v13; // rax
  double v14; // xmm0_8
  double v15; // xmm0_8
  unsigned long long v16; // rdi
  const char *v17; // rax
  unsigned int v18; // r12d
  int v20; // eax
  double v21; // xmm0_8
  char *v22; // rax
  size_t v23; // rdx
  size_t v24; // rdx
  size_t v25; // rdx
  char *v26; // rax
  char v27[24]; // [rsp+10h] [rbp-48h] BYREF
  unsigned long long v28; // [rsp+28h] [rbp-30h]

  v28 = __readfsqword(0x28u);
  if ( (unsigned int)dword_84BA8 > 7 )
  {
    if ( dword_84BA8 != 8 )
      goto LABEL_20;
    goto LABEL_17;
  }
  if ( (unsigned int)dword_84BA8 > 4 )
  {
    v20 = nbytes * qword_83F38;
    if ( ((nbytes * qword_83F38) & 0x8000000000000000LL) != 0LL )
    {
      v23 = v20 & 1 | ((nbytes * qword_83F38) >> 1);
      v21 = (double)(int)v23 + (double)(int)v23;
    }
    else
    {
      v21 = (double)v20;
    }
    v22 = dcgettext(0, *a2, 5);
    v18 = sub_C0A0((long long)stream, v22, v21);
    goto LABEL_14;
  }
  if ( dword_84BA8 != 4 )
  {
    if ( !dword_84BA8 )
LABEL_20:
      abort();
LABEL_17:
    v18 = sub_C0A0((long long)stream, a2[1], *(double *)&qword_841A8 + *(double *)&qword_841A0);
    if ( !a4 )
      return v18;
LABEL_18:
    ++v18;
    fputc_unlocked(a4, stream);
    return v18;
  }
  v6 = nbytes * qword_83F38;
  if ( ((nbytes * qword_83F38) & 0x8000000000000000LL) != 0LL )
  {
    v24 = v6 & 1 | ((nbytes * qword_83F38) >> 1);
    v8 = (double)(int)v24 + (double)(int)v24;
  }
  else
  {
    v8 = (double)v6;
  }
  v9 = sub_C0A0((long long)stream, *a2, v8);
  fputc_unlocked(a3, stream);
  v10 = sub_C0A0((long long)stream, a2[1], *(double *)&qword_841A8 + *(double *)&qword_841A0) + v9 + 1;
  fputc_unlocked(a3, stream);
  v11 = a2[2];
  v12 = v10 + 1;
  if ( v11 && *v11 )
  {
    v26 = dcgettext(0, v11, 5);
    v12 += __fprintf_chk(stream, 1, "%s: ", v26);
  }
  v13 = nbytes * (qword_83F38 - qword_842D8);
  if ( v13 < 0 )
  {
    v25 = v13 & 1 | ((nbytes * (qword_83F38 - qword_842D8)) >> 1);
    v14 = (double)(int)v25 + (double)(int)v25;
  }
  else
  {
    v14 = (double)(int)v13;
  }
  v15 = v14 - (*(double *)&qword_841A8 + *(double *)&qword_841A0);
  if ( v15 >= 9.223372036854776e18 )
    v16 = (unsigned int)(int)(v15 - 9.223372036854776e18) ^ 0x8000000000000000LL;
  else
    v16 = (unsigned int)(int)v15;
  v17 = (const char *)sub_43090(v16, v27);
  v18 = __fprintf_chk(stream, 1, "%s", v17) + v12;
LABEL_14:
  if ( a4 )
    goto LABEL_18;
  return v18;
}



// Function: print_total_stats @ 0xd150
long long print_total_stats()
{
  return sub_CE80(stderr, (const char **)off_7F0E0, 10, 10);
}



// Function: current_block_ordinal @ 0xd180
long long current_block_ordinal()
{
  return qword_84170 + (((char *)src - (char *)s) >> 9);
}



// Function: reset_eof @ 0xd1a0
char *reset_eof()
{
  char *result; // rax

  if ( byte_84161 )
  {
    byte_84161 = 0;
    dword_83F40 = 1;
    src = s;
    result = (char *)s + 512 * (long long)dword_84BA0;
    qword_83F50 = result;
  }
  return result;
}



// Function: set_next_block_after @ 0xd1f0
char *set_next_block_after(unsigned long long a1)
{
  char *result; // rax

  result = (char *)src;
  if ( a1 >= (unsigned long long)src )
  {
    result = (char *)src + ((a1 - (long long)src) & 0xFFFFFFFFFFFFFE00LL) + 512;
    src = result;
  }
  if ( qword_83F50 < result )
    abort();
  return result;
}



// Function: available_space_after @ 0xd2b0
char *available_space_after(long long a1)
{
  return (char *)qword_83F50 - a1;
}



// Function: xclose @ 0xd2c0
int xclose(int a1)
{
  int result; // eax
  char *v2; // rax

  result = close(a1);
  if ( result )
  {
    v2 = dcgettext(0, "(pipe)", 5);
    return sub_32680(v2);
  }
  return result;
}



// Function: archive_read_error @ 0xd300
long long archive_read_error()
{
  long long v0; // rdi
  long long result; // rax
  const char *v2; // rsi
  char *v3; // rax

  v0 = *(long long *)qword_84828;
  sub_32820(*(long long *)qword_84828);
  if ( !qword_84170 )
  {
    if ( qword_84D50 )
      qword_84D50(v0);
    v2 = "At beginning of tape, quitting now";
LABEL_7:
    v3 = dcgettext(0, v2, 5);
    error(0, 0, v3);
    sub_163D0(0);
  }
  result = (unsigned int)dword_84164++;
  if ( (int)result > 10 )
  {
    if ( qword_84D50 )
      qword_84D50(v0);
    v2 = "Too many errors, quitting";
    goto LABEL_7;
  }
  return result;
}



// Function: short_read @ 0xd390
char *short_read(unsigned long long a1)
{
  long long v1; // rbx
  char *v2; // r12
  size_t v3; // rbp
  long long v4; // rdi
  char *result; // rax
  char *v6; // rax
  unsigned long long v7; // r12
  char *v8; // rax

  v1 = a1;
  v2 = (char *)s + a1;
  v3 = nbytes - a1;
  if ( nbytes != a1 && (dword_83B80 & 0x400000) != 0 && !(qword_84170 | v3 & 0x1FF) && a1 && sub_CA00() )
  {
    if ( qword_84D50 )
      qword_84D50(a1);
    v6 = dcngettext(0, "Record size = %lu block", "Record size = %lu blocks", a1 >> 9, 5);
    error(0, 0, v6, a1 >> 9);
  }
  if ( (v3 & 0x1FF) != 0 )
    goto LABEL_15;
LABEL_4:
  if ( v3 && v1 && byte_84160 )
  {
    do
    {
      while ( 1 )
      {
        v4 = (unsigned int)fd;
        if ( fd <= 0x3FFFFFFF )
          break;
        v4 = (unsigned int)(fd - 0x40000000);
        v1 = sub_33DA0(v4, v2, v3);
        if ( v1 != -1 )
          goto LABEL_12;
LABEL_9:
        sub_D300();
      }
      v1 = sub_4AF10(fd, v2, v3);
      if ( v1 == -1 )
        goto LABEL_9;
LABEL_12:
      if ( !v1 )
        break;
      if ( !byte_84160 )
      {
        v7 = nbytes - v3;
        if ( qword_84D50 )
          qword_84D50(v4);
        v8 = dcngettext(0, "Unaligned block (%lu byte) in archive", "Unaligned block (%lu bytes) in archive", v7, 5);
        error(0, 0, v8, v7);
        sub_163D0(0);
      }
      v3 -= v1;
      v2 += v1;
      if ( (v3 & 0x1FF) == 0 )
        goto LABEL_4;
LABEL_15:
      ;
    }
    while ( v1 );
  }
  ++qword_83F38;
  result = (char *)s + ((nbytes - v3) & 0xFFFFFFFFFFFFFE00LL);
  qword_83F50 = result;
  return result;
}



// Function: seek_archive @ 0xd540
__off_t seek_archive(signed long long a1)
{
  signed long long v1; // rax
  size_t v2; // rsi
  size_t v3; // rdi
  long long v4; // rbx
  long long v5; // rdx
  unsigned __int128 v6; // rtt
  __off_t v7; // rsi
  long long v8; // rdi
  __off_t v9; // r8
  long long v10; // r8
  long long v11; // rcx
  char *v13; // rax

  v1 = (dword_84BA0 - (((char *)src - (char *)s) >> 9)) << 9;
  if ( v1 >= a1 )
    return 0;
  v2 = nbytes;
  v3 = a1 - v1;
  if ( v3 < nbytes )
    return 0;
  v4 = sub_D180(v3, nbytes, 0);
  *(long long *)&v6 = v3;
  *((long long *)&v6 + 1) = v5;
  v7 = v6 / v2 * v2;
  if ( fd <= 0x3FFFFFFF )
  {
    v8 = (unsigned int)fd;
    v9 = lseek(fd, v7, 1);
  }
  else
  {
    v8 = (unsigned int)(fd - 0x40000000);
    v9 = sub_33F90(v8, v7, 1);
  }
  if ( v9 >= 0 )
  {
    if ( v9 % nbytes )
    {
      if ( qword_84D50 )
        qword_84D50(v8);
      v13 = dcgettext(0, "rmtlseek not stopped at a record boundary", 5);
      error(0, 0, v13);
      sub_163D0(0);
    }
    v10 = v9 >> 9;
    v11 = v10 - dword_84BA0;
    v9 = v10 - v4;
    qword_84170 = v11;
    qword_83F38 += v9 / dword_84BA0;
    src = qword_83F50;
  }
  return v9;
}



// Function: write_fatal_details @ 0xd670
void write_fatal_details()
{
  long long v0; // rdi

  sub_32E30();
  if ( fd <= 0x3FFFFFFF )
  {
    if ( !close(fd) )
      goto LABEL_3;
  }
  else if ( !(unsigned int)sub_33D60((unsigned int)(fd - 0x40000000)) )
  {
    goto LABEL_3;
  }
  sub_32680(*(long long *)qword_84828);
LABEL_3:
  v0 = (unsigned int)dword_84168;
  sub_2ACA0(dword_84168);
  sub_163D0(v0);
}



// Function: archive_write_error @ 0xd6d0
void archive_write_error(long long a1)
{
  int *v1; // rax
  int v2; // r12d
  int *v3; // rbx

  if ( byte_84A69 )
  {
    v1 = __errno_location();
    v2 = *v1;
    v3 = v1;
    sub_D150();
    *v3 = v2;
  }
  sub_D670(*(long long *)qword_84828, a1, nbytes);
}



// Function: simple_flush_write @ 0xd720
unsigned long long simple_flush_write()
{
  unsigned long long result; // rax

  result = sub_C490();
  if ( nbytes != result )
    sub_D6D0(result);
  ++qword_83F30;
  *(double *)&qword_841A0 = (double)(int)result + *(double *)&qword_841A0;
  return result;
}



// Function: simple_flush_read @ 0xd770
char *simple_flush_read(double a1)
{
  int v1; // edi
  size_t v2; // rdx
  char *result; // rax
  long long v4; // rax

  sub_108D0(0);
  dword_84164 = 0;
  if ( byte_83F20 && qword_84170 )
  {
    fd = 1;
    v4 = sub_2AF00(a1);
    fd = 0;
    v2 = v4;
    if ( nbytes != v4 )
      sub_D6D0(v4);
    v1 = 0;
    goto LABEL_4;
  }
  while ( 1 )
  {
    v1 = fd;
    v2 = nbytes;
LABEL_4:
    result = (char *)(v1 <= 0x3FFFFFFF ? sub_4AF10(v1, s, v2) : sub_33DA0((unsigned int)(v1 - 0x40000000), s, v2));
    if ( (char *)nbytes == result )
      break;
    if ( result != (char *)-1LL )
      return sub_D390((unsigned long long)result);
    sub_D300();
  }
  ++qword_83F38;
  return result;
}



// Function: init_volume_number @ 0xd850
int init_volume_number()
{
  FILE *v0; // rax
  FILE *v1; // rbp
  FILE *v2; // rdi
  int *v3; // rax
  long long v4; // r12
  char *v5; // rax

  v0 = fopen(filename, "r");
  if ( v0 )
  {
    v1 = v0;
    v2 = v0;
    if ( (unsigned int)__isoc99_fscanf(v0, "%d", &dword_81010) != 1 || dword_81010 < 0 )
    {
      if ( qword_84D50 )
        qword_84D50(v2);
      v4 = sub_4AA70(filename);
      v5 = dcgettext(0, "%s: contains invalid volume number", 5);
      error(0, 0, v5, v4);
      sub_163D0(0);
    }
    if ( ferror_unlocked(v1) )
    {
      sub_32820(filename);
      LODWORD(v3) = fclose(v1);
      if ( !(int)v3 )
        return (int)v3;
    }
    else
    {
      LODWORD(v3) = fclose(v1);
      if ( !(int)v3 )
        return (int)v3;
    }
    LODWORD(v3) = sub_32680(filename);
  }
  else
  {
    v3 = __errno_location();
    if ( *v3 != 2 )
      LODWORD(v3) = sub_327C0(filename);
  }
  return (int)v3;
}



// Function: closeout_volume_number @ 0xd960
int closeout_volume_number()
{
  FILE *v0; // rax
  FILE *v1; // rbp
  int result; // eax

  v0 = fopen(filename, "w");
  if ( !v0 )
    return sub_327C0(filename);
  v1 = v0;
  __fprintf_chk(v0, 1, "%d\n", dword_81010);
  if ( !ferror_unlocked(v1) )
  {
    result = fclose(v1);
    if ( !result )
      return result;
    return sub_32680(filename);
  }
  sub_32E10(filename);
  result = fclose(v1);
  if ( result )
    return sub_32680(filename);
  return result;
}



// Function: new_volume @ 0xda00
long long new_volume(int a1)
{
  char **v2; // rbx
  char *v3; // rbp
  long long result; // rax
  long long v5; // rdi
  char *v6; // rax
  int v7; // eax
  int v8; // edx
  char *v9; // rax
  FILE *v10; // rax
  char *v11; // rax
  long long v12; // r12
  char *v13; // rax

  if ( !qword_83F68 && !qword_84B48 )
  {
    v10 = (FILE *)stdin;
    if ( !fd )
      v10 = fopen("/dev/tty", "r");
    qword_83F68 = v10;
  }
  if ( byte_84280 )
    return 0;
  if ( byte_84A50 )
    sub_118D0();
  sub_23BE0(&qword_83F18, 0);
  sub_23BE0(&s1, 0);
  qword_83F00 = 0;
  qword_83F08 = 0;
  src = s;
  if ( fd > 0x3FFFFFFF )
  {
    if ( !(unsigned int)sub_33D60((unsigned int)(fd - 0x40000000)) )
      goto LABEL_7;
LABEL_13:
    sub_32680(*(long long *)qword_84828);
    goto LABEL_7;
  }
  if ( close(fd) )
    goto LABEL_13;
LABEL_7:
  v2 = (char **)(qword_84828 + 8);
  qword_84828 = (long long)v2;
  if ( v2 == (char **)((char *)ptr + 8 * qword_84830) )
  {
    qword_84828 = (long long)ptr;
    dword_83F60 = 1;
    goto LABEL_15;
  }
  if ( dword_83F60 )
    goto LABEL_15;
  while ( 1 )
  {
    v3 = *v2;
    if ( !strcmp(*v2, "-") )
      break;
    if ( byte_84A50 )
      goto LABEL_30;
    if ( a1 == 1 )
    {
      if ( byte_84B84 )
      {
        sub_24710(v3);
        v2 = (char **)qword_84828;
        v3 = *(char **)qword_84828;
      }
      if ( byte_84D70
        || (v11 = strchr(v3, 58), qword_84D78 = (long long)v11, v3 = *v2, !v11)
        || v11 <= v3
        || memchr(*v2, 47, v11 - v3) )
      {
        v8 = creat(v3, 0x1B6u);
      }
      else
      {
        v8 = sub_338D0(v3, 65, 0x40000000, qword_84AB0);
      }
      goto LABEL_34;
    }
    if ( a1 == 2 )
    {
LABEL_30:
      if ( byte_84D70
        || (v9 = strchr(v3, 58), qword_84D78 = (long long)v9, v3 = *v2, !v9)
        || v9 <= v3
        || memchr(*v2, 47, v9 - v3) )
      {
        v8 = open(v3, 66, 438);
      }
      else
      {
        v8 = sub_338D0(v3, 66, 0x40000000, qword_84AB0);
      }
LABEL_34:
      fd = v8;
      goto LABEL_35;
    }
    if ( byte_84D70
      || (v6 = strchr(v3, 58), qword_84D78 = (long long)v6, v3 = *v2, !v6)
      || v6 <= v3
      || memchr(*v2, 47, v6 - v3) )
    {
      v7 = open(v3, 0, 438);
    }
    else
    {
      v7 = sub_338D0(v3, 0, 0x40000000, qword_84AB0);
    }
    fd = v7;
    sub_C8D0();
    v8 = fd;
LABEL_35:
    result = 1;
    if ( v8 >= 0 )
      return result;
    sub_32800(*(long long *)qword_84828);
    if ( byte_84A50 != 1 && a1 == 1 && byte_84B84 )
      sub_245D0();
LABEL_15:
    if ( qword_84B48 )
    {
      if ( filename )
        sub_D960();
      v5 = qword_84828;
      if ( (unsigned int)sub_2B8B0(qword_84828, (unsigned int)(dword_81010 + 1)) )
      {
        if ( qword_84D50 )
          qword_84D50(v5);
        v12 = sub_4AC50(qword_84B48);
        v13 = dcgettext(0, "%s command failed", 5);
        error(0, 0, v13, v12);
        sub_163D0(0);
      }
    }
    else
    {
      sub_C580(qword_83F68);
    }
    v2 = (char **)qword_84828;
  }
  byte_84160 = 1;
  result = 1;
  fd = 0;
  return result;
}



// Function: drop_volume_label_suffix @ 0xde80
void *drop_volume_label_suffix(const char *src)
{
  size_t v1; // rax
  const char *v2; // rbx
  const unsigned short *v3; // rdx
  const char *v4; // rbx
  size_t v6; // rbx
  void *v7; // r12

  v1 = strlen(src);
  if ( !v1 )
    return 0;
  v2 = &src[v1 - 1];
  if ( src >= v2 )
    return 0;
  v3 = *__ctype_b_loc();
  while ( (v3[*(unsigned char *)v2] & 0x800) != 0 )
  {
    if ( src == --v2 )
      return 0;
  }
  if ( src >= v2 )
    return 0;
  v4 = v2 - 7;
  if ( v4 <= src || memcmp(v4, " Volume ", 8u) )
    return 0;
  v6 = v4 - src;
  v7 = (void *)sub_4C5B0(v6 + 1);
  memcpy(v7, src, v6);
  *((char *)v7 + v6) = 0;
  return v7;
}



// Function: check_label_pattern @ 0xdf40
long long check_label_pattern(const char *src)
{
  int v1; // r8d
  long long result; // rax
  const char *v3; // rax
  char *v4; // rbp
  bool v5; // [rsp+Fh] [rbp-9h]

  v1 = fnmatch(name, src, 0);
  result = 1;
  if ( v1 )
  {
    if ( byte_84B10 && (v3 = (const char *)sub_DE80(src), (v4 = (char *)v3) != 0) )
    {
      v5 = fnmatch(v3, name, 0) == 0;
      free(v4);
      return v5;
    }
    else
    {
      return 0;
    }
  }
  return result;
}



// Function: flush_archive @ 0xdfd0
long long flush_archive()
{
  long long result; // rax
  char *v1; // rsi
  long long v2; // rdx
  signed long long v3; // rcx
  char *v4; // rdi
  signed long long v5; // rax

  result = (unsigned int)dword_83F40;
  if ( dword_83F40 )
  {
    v1 = (char *)s;
    v2 = dword_84BA0;
    v3 = ((char *)qword_83F50 - (char *)s) >> 9;
  }
  else
  {
    if ( !byte_84CF8 )
    {
      src = s;
      qword_84170 += ((char *)qword_83F50 - (char *)s) >> 9;
      qword_83F50 = (char *)s + 512 * (long long)dword_84BA0;
      return sub_DFC0();
    }
    dword_83F40 = 1;
    byte_84CF8 = 0;
    sub_C290();
    v1 = (char *)s;
    v2 = dword_84BA0;
    v5 = ((char *)qword_83F50 - (char *)s) >> 9;
    v3 = v5;
    if ( v5 < dword_84BA0 )
    {
      memset(qword_83F50, 0, (dword_84BA0 - v5) << 9);
      result = (long long)s + 512 * (long long)dword_84BA0;
      qword_83F50 = (void *)result;
      return result;
    }
    result = (unsigned int)dword_83F40;
  }
  v4 = src;
  qword_84170 += v3;
  src = v1;
  qword_83F50 = &v1[512 * v2];
  switch ( (int)result )
  {
    case 1:
      return qword_84158(v4 - v1);
    case 2:
      abort();
    case 0:
      return sub_DFC0();
  }
  return result;
}



// Function: find_next_block @ 0xe130
void *find_next_block()
{
  void *result; // rax

  result = src;
  if ( src == qword_83F50 )
  {
    result = 0;
    if ( !byte_84161 )
    {
      sub_DFD0();
      result = src;
      if ( src == qword_83F50 )
      {
        byte_84161 = 1;
        return 0;
      }
    }
  }
  return result;
}



// Function: _write_volume_label @ 0xe190
char *_write_volume_label(long long a1)
{
  long long *v1; // rbp
  char v2; // al

  if ( dword_84BA4 == 4 )
    return (char *)sub_1CA30("GNU.volume.label");
  v1 = sub_E130();
  *v1 = 0;
  v1[63] = 0;
  memset(
    (void *)((unsigned long long)(v1 + 1) & 0xFFFFFFFFFFFFFFF8LL),
    0,
    8LL * (((unsigned int)v1 - (((int)v1 + 8) & 0xFFFFFFF8) + 512) >> 3));
  __strcpy_chk(v1, a1, 100);
  sub_23BE0(&path, v1);
  v2 = sub_3E630(path);
  *((char *)v1 + 156) = 86;
  byte_84850 = v2;
  sub_12C10(stru_84A20.tv_sec, v1 + 17, 12);
  sub_12F60(&qword_84840, v1, -1);
  return sub_D1F0((unsigned long long)v1);
}



// Function: add_volume_label @ 0xe280
unsigned long long add_volume_label()
{
  const char *v0; // rbx
  size_t v1; // rbp
  size_t v2; // rax
  void *v3; // rbp
  long long v5; // [rsp+0h] [rbp-38h] BYREF
  unsigned long long v6; // [rsp+18h] [rbp-20h]

  v6 = __readfsqword(0x28u);
  v0 = (const char *)sub_43090(dword_81014, &v5);
  v1 = strlen(name);
  v2 = strlen(v0);
  v3 = (void *)sub_4C5B0(v1 + v2 + 9);
  __sprintf_chk(v3, 1, -1, "%s %s %s", name, "Volume", v0);
  sub_E190((long long)v3);
  free(v3);
  return v6 - __readfsqword(0x28u);
}



// Function: write_volume_label @ 0xe330
char *write_volume_label()
{
  if ( byte_84B10 )
    return (char *)sub_E280();
  else
    return sub_E190((long long)name);
}



// Function: gnu_add_multi_volume_header @ 0xe350
char *gnu_add_multi_volume_header(long long *a1)
{
  void *v1; // rax
  const char *v2; // r12
  unsigned long long v3; // rbp
  size_t v4; // rax
  unsigned int v5; // edx
  long long v6; // r12
  char *v7; // rax
  long long *v8; // rsi
  int v9; // ebx

  v1 = sub_E130();
  v2 = (const char *)a1[2];
  v3 = (unsigned long long)v1;
  v4 = strlen(v2);
  v5 = v4;
  if ( v4 > 0x64 )
  {
    if ( qword_84D50 )
    {
      qword_84D50(v2);
      v2 = (const char *)a1[2];
    }
    v6 = sub_4AA70(v2);
    v7 = dcgettext(0, "%s: file name too long to be stored in a GNU multivolume header, truncated", 5);
    error(0, 0, v7, v6);
    v5 = 100;
  }
  *(long long *)v3 = 0;
  *(long long *)(v3 + 504) = 0;
  memset(
    (void *)((v3 + 8) & 0xFFFFFFFFFFFFFFF8LL),
    0,
    8LL * (((unsigned int)v3 - (((int)v3 + 8) & 0xFFFFFFF8) + 512) >> 3));
  v8 = (long long *)a1[2];
  if ( v5 >= 8 )
  {
    *(long long *)v3 = *v8;
    *(long long *)(v3 + v5 - 8) = *(long long *)((char *)v8 + v5 - 8);
    qmemcpy(
      (void *)((v3 + 8) & 0xFFFFFFFFFFFFFFF8LL),
      (char *)v8 - (v3 - ((v3 + 8) & 0xFFFFFFFFFFFFFFF8LL)),
      8LL * ((v5 + (int)v3 - (((int)v3 + 8) & 0xFFFFFFF8)) >> 3));
  }
  else if ( (v5 & 4) != 0 )
  {
    *(int *)v3 = *(int *)v8;
    *(int *)(v3 + v5 - 4) = *(int *)((char *)v8 + v5 - 4);
  }
  else if ( v5 )
  {
    *(char *)v3 = *(char *)v8;
    if ( (v5 & 2) != 0 )
      *(short *)(v3 + v5 - 2) = *(short *)((char *)v8 + v5 - 2);
  }
  *(char *)(v3 + 156) = 77;
  sub_12BE0(a1[4], v3 + 124, 12);
  sub_12BE0(a1[3] - a1[4], v3 + 369, 12);
  v9 = dword_84A54;
  dword_84A54 = 0;
  sub_12F60(&qword_84840, v3, -1);
  dword_84A54 = v9;
  return sub_D1F0(v3);
}



// Function: add_multi_volume_header @ 0xe4d0
char *add_multi_volume_header(long long *a1)
{
  unsigned long long v2; // [rsp+8h] [rbp-20h]

  v2 = __readfsqword(0x28u);
  if ( dword_84BA4 != 4 )
    return sub_E350(a1);
  sub_1CA30("GNU.volume.filename");
  sub_1CA30("GNU.volume.size");
  sub_1CA30("GNU.volume.offset");
  return (char *)(v2 - __readfsqword(0x28u));
}



// Function: _gnu_flush_write @ 0xe580
void *_gnu_flush_write(long long a1)
{
  long long v1; // r12
  void *result; // rax
  double v3; // xmm0_8
  long long v4; // rbx
  void **v5; // r14
  int v6; // eax
  size_t v7; // rbp
  double v8; // xmm0_8
  char *v9; // r12
  void *v10; // rax
  char *v11; // r13
  char *v12; // rax
  size_t i; // rbx
  const void *v14; // rsi
  void *v15; // rax
  char *v16; // rax

  v1 = sub_C490();
  result = (void *)nbytes;
  if ( nbytes == v1 )
  {
    v3 = (double)(int)nbytes + *(double *)&qword_841A0;
    if ( nbytes )
    {
      ++qword_83F30;
      *(double *)&qword_841A0 = (double)(int)nbytes + *(double *)&qword_841A0;
      return result;
    }
    goto LABEL_10;
  }
  if ( !byte_84B10 )
    goto LABEL_8;
  v3 = (double)(int)v1 + *(double *)&qword_841A0;
  if ( !v1 )
  {
LABEL_10:
    qword_841A0 = *(long long *)&v3;
    if ( !nbytes )
      return result;
    v4 = 0;
    v5 = (void **)sub_BED0(0);
    goto LABEL_12;
  }
  ++qword_83F30;
  v4 = v1;
  *(double *)&qword_841A0 = (double)(int)v1 + *(double *)&qword_841A0;
  v5 = (void **)sub_BED0(v1);
  if ( (v1 & 0x1FF) != 0 )
  {
    if ( qword_84D50 )
      qword_84D50(v1);
    v16 = dcgettext(0, "write did not end on a block boundary", 5);
    error(0, 0, v16, v3);
    dword_84D58 = 2;
LABEL_8:
    sub_D6D0(v1);
  }
  if ( v1 < 0 )
  {
    v6 = *__errno_location();
    if ( (unsigned int)(v6 - 5) > 1 && v6 != 28 )
      goto LABEL_8;
  }
LABEL_12:
  result = (void *)sub_DA00(1);
  if ( (char)result )
  {
    v7 = a1 - v4;
    sub_2F6F0(&unk_83F80, v3);
    sub_C240((long long)&unk_83F80);
    v8 = *(double *)&qword_841A8 + *(double *)&qword_841A0;
    qword_841A0 = 0;
    v9 = (char *)s + v4;
    dword_84178 = dword_84178 == 0;
    qword_841A8 = *(long long *)&v8;
    sub_C980();
    dword_84138 = 1;
    if ( name )
      sub_E280();
    if ( v5 )
    {
      sub_E4D0(v5);
      v10 = sub_E130();
      sub_12DA0(1, &unk_83F80, v10);
      sub_2F6F0(&unk_83F80, v8);
      sub_CB00((long long)v5);
    }
    else
    {
      v15 = sub_E130();
      sub_12DA0(1, &unk_83F80, v15);
      sub_2F6F0(&unk_83F80, v8);
    }
    v11 = sub_E130();
    sub_C030(v5, (v11 - (char *)s) >> 9);
    v12 = sub_D2B0((long long)v11);
    dword_84138 = 0;
    for ( i = (size_t)v12; v7 > i; i = (size_t)sub_D2B0((long long)v11) )
    {
      v14 = v9;
      v9 += i;
      memcpy(v11, v14, i);
      v7 -= i;
      sub_D1F0((unsigned long long)&v11[(i - 1) & 0xFFFFFFFFFFFFFE00LL]);
      v11 = sub_E130();
    }
    memcpy(v11, v9, v7);
    memset(&v11[v7], 0, i - v7);
    sub_D1F0((unsigned long long)&v11[(v7 - 1) & 0xFFFFFFFFFFFFFE00LL]);
    return sub_E130();
  }
  return result;
}



// Function: gnu_flush_write @ 0xe850
long long (*gnu_flush_write(long long a1))()
{
  qword_84158 = (long long (*)(long long))sub_D720;
  sub_E580(a1);
  qword_84158 = (long long (*)(long long))gnu_flush_write;
  return gnu_flush_write;
}



// Function: try_new_volume @ 0xe880
long long try_new_volume(double a1)
{
  unsigned int v1; // r12d
  long long v2; // rdi
  unsigned long long v3; // rax
  char *v4; // rax
  char v5; // al
  long long *v6; // rbx
  const char *v7; // rbp
  const char *v8; // r13
  void (*v9)(void); // rbx
  long long v10; // r13
  char *v11; // rax
  long long v12; // rdx
  long long v13; // r13
  long long v14; // rbp
  long long v15; // rbx
  long long v16; // r12
  char *v17; // rax
  char *v19; // rax
  char *v20; // rax
  bool v21; // zf
  double v22; // xmm0_8
  int v23; // eax
  const char *v24; // rdi
  long long v25; // r12
  char *v26; // rax
  long long v27; // rbp
  long long v28; // rbx
  long long v29; // r12
  char *v30; // rax
  long long v31; // [rsp+8h] [rbp-90h] BYREF
  char v32[32]; // [rsp+10h] [rbp-88h] BYREF
  char v33[32]; // [rsp+30h] [rbp-68h] BYREF
  char v34[24]; // [rsp+50h] [rbp-48h] BYREF
  unsigned long long v35; // [rsp+68h] [rbp-30h]

  v35 = __readfsqword(0x28u);
  if ( (unsigned int)dword_84BA8 <= 2 )
  {
    v1 = sub_DA00(2 * (unsigned int)(dword_84BA8 != 0));
    if ( (char)v1 )
      goto LABEL_6;
    return 1;
  }
  v1 = sub_DA00(2 * (unsigned int)(dword_84BA8 == 8));
  if ( !(char)v1 )
    return 1;
LABEL_6:
  while ( 1 )
  {
    v2 = (unsigned int)fd;
    if ( fd <= 0x3FFFFFFF )
      break;
    v2 = (unsigned int)(fd - 0x40000000);
    v3 = sub_33DA0(v2, s, nbytes);
    if ( v3 != -1 )
      goto LABEL_8;
LABEL_5:
    sub_D300();
  }
  v3 = sub_4AF10(fd, s, nbytes);
  if ( v3 == -1 )
    goto LABEL_5;
LABEL_8:
  if ( nbytes != v3 )
  {
    v2 = v3;
    sub_D390(v3);
    v4 = sub_E130();
    v31 = (long long)v4;
    if ( !v4 )
      goto LABEL_30;
LABEL_10:
    v5 = v4[156];
    switch ( v5 )
    {
      case 'V':
        if ( !(unsigned char)sub_D230() )
          return 0;
        sub_2F6F0(&unk_83F80, a1);
        sub_23C20(&qword_83F18, qword_84408, 100);
        v2 = v31;
        sub_D1F0(v31);
        v20 = sub_E130();
        v21 = v20[156] == 77;
        v31 = (long long)v20;
        if ( !v21 )
          goto LABEL_13;
        break;
      case 'g':
        sub_2F630(&unk_83F80);
        v2 = (long long)&v31;
        if ( (unsigned int)sub_219E0(&v31, &unk_83F80, 2) != 2 )
          goto LABEL_30;
        v22 = sub_1C8C0(&unk_83F80);
        sub_2F6F0(&unk_83F80, v22);
        v2 = (long long)&v31;
        v23 = sub_219E0(&v31, &unk_83F80, 0);
        if ( v23 == 1 )
        {
          v2 = v31;
          sub_D1F0(v31);
        }
        else if ( v23 != 5 )
        {
          goto LABEL_30;
        }
        goto LABEL_13;
      case 'M':
        break;
      default:
LABEL_13:
        v6 = qword_84148;
        if ( !qword_84148 )
          goto LABEL_46;
        v7 = s1;
        if ( s1 )
        {
          v8 = (const char *)*((long long *)qword_84148 + 2);
          if ( !strcmp(s1, v8) )
          {
LABEL_22:
            v2 = qword_83F00;
            v12 = v6[3];
            if ( (qword_83F08 + qword_83F00 != v12) | __CFADD__(qword_83F08, qword_83F00) )
            {
              if ( qword_84D50 )
              {
                qword_84D50(qword_83F00);
                v2 = qword_83F00;
              }
              v13 = sub_43090(v2, v34);
              v14 = sub_43090(qword_83F08, v33);
              v15 = sub_43090(*((long long *)qword_84148 + 3), v32);
              v16 = sub_4AC50(s1);
              v17 = dcgettext(0, "%s is the wrong size (%s != %s + %s)", 5);
              error(0, 0, v17, v16, v15, v14, v13);
              return 0;
            }
            if ( v12 - v6[4] != qword_83F00 )
            {
              if ( qword_84D50 )
              {
                qword_84D50(qword_83F00);
                v2 = qword_83F00;
              }
              v27 = sub_43090(v2, v34);
              v28 = sub_43090(*((long long *)qword_84148 + 4), v33);
              v29 = sub_43090(*((long long *)qword_84148 + 3), v32);
              v30 = dcgettext(0, "This volume is out of sequence (%s - %s != %s)", 5);
              error(0, 0, v30, v29, v28, v27);
              return 0;
            }
LABEL_46:
            sub_C240(v2);
            return v1;
          }
          v9 = (void (*)(void))qword_84D50;
          if ( (dword_84BA4 & 0xFFFFFFFB) == 2 && strlen(v8) > 0x63 && !strncmp(v7, v8, 0x64u) )
          {
            if ( v9 )
            {
              v9();
              v8 = (const char *)*((long long *)qword_84148 + 2);
            }
            v10 = sub_4AC50(v8);
            v11 = dcgettext(0, "%s is possibly continued on this volume: header contains truncated name", 5);
            error(0, 0, v11, v10);
            v6 = qword_84148;
            goto LABEL_22;
          }
          if ( v9 )
          {
            v9();
            v8 = (const char *)*((long long *)qword_84148 + 2);
          }
          v24 = v8;
        }
        else
        {
          if ( qword_84D50 )
          {
            qword_84D50(v2);
            v6 = qword_84148;
          }
          v24 = (const char *)v6[2];
        }
        v25 = sub_4AC50(v24);
        v26 = dcgettext(0, "%s is not continued on this volume", 5);
        error(0, 0, v26, v25);
        return 0;
    }
    if ( !(unsigned char)sub_D230() )
      return 0;
    sub_2F6F0(&unk_83F80, a1);
    sub_23C20(&s1, qword_84408, 100);
    qword_83F08 = sub_22050(qword_84408 + 124, 12);
    v2 = qword_84408 + 369;
    qword_83F00 = sub_22050(qword_84408 + 369, 12);
    goto LABEL_13;
  }
  v4 = sub_E130();
  v31 = (long long)v4;
  if ( v4 )
    goto LABEL_10;
LABEL_30:
  if ( qword_84D50 )
    qword_84D50(v2);
  v1 = 0;
  v19 = dcgettext(0, "This does not look like a tar archive", 5);
  error(0, 0, v19);
  return v1;
}



// Function: check_compressed_archive @ 0xedb0
long long check_compressed_archive(bool *a1)
{
  bool *v1; // rbx
  char v2; // bp
  void *v3; // rax
  const char *v4; // r12
  const char *v5; // rbp
  unsigned int *v6; // rbx
  const void *v7; // rsi
  size_t v8; // rdx
  long long result; // rax
  char v10; // [rsp+7h] [rbp-21h] BYREF
  unsigned long long v11; // [rsp+8h] [rbp-20h]

  v1 = a1;
  v2 = byte_84160;
  v11 = __readfsqword(0x28u);
  byte_84160 = 1;
  if ( !a1 )
    v1 = (bool *)&v10;
  qword_83F50 = s;
  v3 = sub_E130();
  byte_84160 = v2;
  v4 = (const char *)s;
  v5 = (const char *)qword_83F50;
  *v1 = v3 == 0;
  if ( v4 != v5 )
  {
    if ( !strcmp(v4 + 257, "ustar") || !strcmp(v4 + 257, "ustar  ") )
    {
      result = sub_214D0(v4, 1);
      if ( (int)result == 1 )
        return result;
      v5 = (const char *)s;
    }
    else
    {
      v5 = v4;
    }
  }
  v6 = (unsigned int *)&unk_7F130;
  v7 = &unk_60127;
  v8 = 2;
  while ( memcmp(v5, v7, v8) )
  {
    v6 += 6;
    if ( v6 == (unsigned int *)algn_7F1F0 )
      return 0;
    v8 = *((long long *)v6 + 1);
    v7 = (const void *)*((long long *)v6 + 2);
  }
  return *v6;
}



// Function: open_compressed_archive @ 0xeed0
int open_compressed_archive()
{
  const char **v0; // rbx
  const char *v1; // rdi
  char *v2; // rax
  void *v3; // rax
  int result; // eax
  __pid_t v5; // eax
  int v6; // eax
  char *v7; // rax
  char *v8; // rax
  char *file; // [rsp+8h] [rbp-20h]
  bool v10; // [rsp+17h] [rbp-11h] BYREF
  unsigned long long v11; // [rsp+18h] [rbp-10h]

  v0 = (const char **)ptr;
  v11 = __readfsqword(0x28u);
  v1 = *(const char **)ptr;
  if ( byte_84D70
    || (v2 = strchr(v1, 58), qword_84D78 = (long long)v2, v1 = *v0, !v2)
    || v2 <= v1
    || (file = (char *)*v0, v3 = memchr(v1, 47, v2 - v1), v1 = file, v3) )
  {
    result = open(v1, 0, 438);
  }
  else
  {
    result = sub_338D0(file, 0, 0x40000000, qword_84AB0);
  }
  fd = result;
  if ( result != -1 )
  {
    if ( byte_84B10 )
    {
LABEL_12:
      qword_83F38 = 0;
      qword_83F50 = s;
      return result;
    }
    if ( qword_84B70 )
    {
LABEL_9:
      if ( result <= 0x3FFFFFFF )
        close(result);
      else
        sub_33D60((unsigned int)(result - 0x40000000));
      byte_84161 = 0;
      v5 = sub_2B310();
      byte_84160 = 1;
      dword_84168 = v5;
      result = fd;
      goto LABEL_12;
    }
    v6 = sub_EDB0(&v10);
    if ( v6 )
    {
      if ( v6 == 1 )
      {
        if ( v10 )
        {
          if ( qword_84D50 )
            qword_84D50(&v10);
          v7 = dcgettext(0, "This does not look like a tar archive", 5);
          error(0, 0, v7);
          dword_84D58 = 2;
        }
        return fd;
      }
      dword_83F70 = v6;
    }
    else
    {
      if ( v10 )
      {
        if ( qword_84D50 )
          qword_84D50(&v10);
        v8 = dcgettext(0, "This does not look like a tar archive", 5);
        error(0, 0, v8);
        dword_84D58 = 2;
      }
      sub_2A260(*(long long *)ptr, 0);
      if ( !qword_84B70 )
        return fd;
    }
    result = fd;
    goto LABEL_9;
  }
  return result;
}



// Function: _open_archive @ 0xf0f0
void *_open_archive(long long a1, double a2)
{
  int v2; // ebx
  __pid_t v3; // eax
  int *v5; // rax
  int v6; // r12d
  const void **v7; // r12
  const char *v8; // rbp
  char *v9; // rdi
  unsigned int v10; // eax
  char *v11; // rax
  const char *v12; // rsi
  char *v13; // rax
  char *v14; // rax
  int v15; // eax
  char **v16; // r12
  int v17; // ebp
  char *v18; // rax
  void *v19; // rax
  int v20; // eax
  char *v21; // rax
  void *v22; // rax
  char *v23; // r12
  char *v24; // rax
  char *s; // [rsp+8h] [rbp-30h]
  char *sb; // [rsp+8h] [rbp-30h]
  char *sa; // [rsp+8h] [rbp-30h]
  unsigned int sc; // [rsp+8h] [rbp-30h]
  bool v29; // [rsp+17h] [rbp-21h] BYREF
  unsigned long long v30; // [rsp+18h] [rbp-20h]

  v30 = __readfsqword(0x28u);
  if ( !nbytes )
  {
    if ( qword_84D50 )
      qword_84D50(a1);
    v12 = "Invalid value for record_size";
    goto LABEL_35;
  }
  if ( !qword_84830 )
  {
    if ( qword_84D50 )
      qword_84D50(a1);
    v12 = "No archive name given";
    goto LABEL_35;
  }
  v2 = a1;
  sub_2F6F0(&qword_84840, a2);
  dword_84178 = 0;
  sub_C980();
  if ( (int)a1 == 2 )
  {
    dword_83F40 = 0;
    sub_CA80(0);
    qword_83F38 = 0;
    byte_84160 = byte_84AB9;
    if ( qword_84B70 )
      abort();
    v7 = (const void **)ptr;
    v8 = *(const char **)ptr;
    v9 = *(char **)ptr;
    if ( strcmp(*(const char **)ptr, "-") )
    {
      if ( byte_84D70
        || (v14 = strchr(v8, 58), qword_84D78 = (long long)v14, v8 = (const char *)*v7, !v14)
        || v14 <= v8
        || memchr(*v7, 47, v14 - v8) )
      {
        v15 = open(v8, 66, 438);
      }
      else
      {
        v15 = sub_338D0(v8, 66, 0x40000000, qword_84AB0);
      }
      fd = v15;
      if ( (unsigned int)sub_EDB0(0) <= 1 )
        goto LABEL_8;
      if ( qword_84D50 )
        qword_84D50(0);
      v12 = "Cannot update compressed archives";
      goto LABEL_35;
    }
LABEL_22:
    byte_84160 = 1;
    if ( !byte_84A50 )
    {
      if ( v2 == 1 )
      {
        fd = 1;
        if ( qword_84820 )
        {
LABEL_47:
          if ( !(unsigned char)sub_2AB10() )
            goto LABEL_15;
          goto LABEL_48;
        }
      }
      else
      {
        if ( v2 != 2 )
        {
          if ( !v2 )
          {
            fd = 0;
            v10 = sub_EDB0(&v29);
            if ( v10 > 1 )
            {
              if ( qword_84D50 )
              {
                sc = v10;
                qword_84D50(&v29);
                v10 = sc;
              }
              v23 = sub_BF70(v10);
              v24 = dcgettext(0, "Archive is compressed. Use %s option", 5);
              error(0, 0, v24, v23);
              sub_163D0(0);
            }
            if ( v29 )
            {
              if ( qword_84D50 )
                qword_84D50(&v29);
              v11 = dcgettext(0, "This does not look like a tar archive", 5);
              error(0, 0, v11);
              dword_84D58 = 2;
            }
          }
          goto LABEL_8;
        }
        fd = 0;
        byte_83F20 = 1;
        qword_83F50 = ::s;
        if ( qword_84820 )
          goto LABEL_14;
      }
      stream = stderr;
      goto LABEL_47;
    }
    if ( qword_84D50 )
      qword_84D50(v9);
    v12 = "Cannot verify stdin/stdout archive";
LABEL_35:
    v13 = dcgettext(0, v12, 5);
    error(0, 0, v13);
    sub_163D0(0);
  }
  dword_83F40 = a1;
  sub_CA80(a1);
  qword_83F38 = 0;
  byte_84160 = byte_84AB9;
  if ( !qword_84B70 )
  {
    v16 = (char **)ptr;
    v9 = *(char **)ptr;
    if ( strcmp(*(const char **)ptr, "-") )
    {
      if ( v2 != 1 )
      {
        if ( !v2 )
        {
          fd = sub_EED0();
          if ( fd < 0 )
            goto LABEL_15;
          sub_C8D0();
        }
        goto LABEL_8;
      }
      v17 = 0;
      if ( byte_84B84 )
      {
        v17 = 1;
        sub_24710(v9);
        v16 = (char **)ptr;
        v9 = *(char **)ptr;
      }
      if ( byte_84A50 )
      {
        if ( byte_84D70
          || (v18 = strchr(v9, 58), qword_84D78 = (long long)v18, v9 = *v16, !v18)
          || v18 <= v9
          || (s = *v16, v19 = memchr(v9, 47, v18 - v9), v9 = s, v19) )
        {
          v20 = open(v9, 66, 438);
        }
        else
        {
          v20 = sub_338D0(s, 66, 0x40000000, qword_84AB0);
        }
      }
      else if ( byte_84D70
             || (v21 = strchr(v9, 58), qword_84D78 = (long long)v21, v9 = *v16, !v21)
             || v21 <= v9
             || (sa = *v16, v22 = memchr(v9, 47, v21 - v9), v9 = sa, v22) )
      {
        v20 = creat(v9, 0x1B6u);
      }
      else
      {
        v20 = sub_338D0(sa, 65, 0x40000000, qword_84AB0);
      }
      fd = v20;
      if ( v20 < 0 )
        goto LABEL_75;
      if ( v20 > 0x3FFFFFFF )
      {
        sub_2ABD0();
        sub_2AB80();
        goto LABEL_12;
      }
      if ( !(unsigned char)sub_2AB10() )
      {
LABEL_75:
        v5 = __errno_location();
        v6 = *v5;
        if ( v17 )
        {
          sb = (char *)v5;
          sub_245D0();
          v5 = (int *)sb;
        }
LABEL_16:
        *v5 = v6;
        sub_327E0(*(long long *)ptr);
      }
LABEL_48:
      sub_2ABD0();
      sub_2AB80();
LABEL_11:
      if ( (unsigned int)(v2 - 1) > 1 )
        return (void *)(v30 - __readfsqword(0x28u));
LABEL_12:
      qword_83F30 = 0;
      return (void *)(v30 - __readfsqword(0x28u));
    }
    goto LABEL_22;
  }
  if ( (int)a1 == 1 )
  {
    dword_84168 = sub_2AF40();
    if ( !qword_84820 && !strcmp(*(const char **)ptr, "-") )
      stream = stderr;
  }
  else if ( !(int)a1 )
  {
    v3 = sub_2B310();
    byte_84160 = 1;
    dword_84168 = v3;
    qword_83F50 = ::s;
  }
LABEL_8:
  if ( fd < 0 )
  {
LABEL_15:
    v5 = __errno_location();
    v6 = *v5;
    goto LABEL_16;
  }
  if ( fd > 0x3FFFFFFF )
    goto LABEL_10;
LABEL_14:
  if ( !(unsigned char)sub_2AB10() )
    goto LABEL_15;
LABEL_10:
  sub_2ABD0();
  sub_2AB80();
  if ( v2 )
    goto LABEL_11;
  return sub_E130();
}



// Function: match_volume_label @ 0xf760
unsigned long long match_volume_label()
{
  const char *v0; // rdi
  char *v2; // rax
  char *v3; // rbp
  char v4; // al
  long long v5; // r12
  char *v6; // rax
  long long v7; // rax
  double v8; // xmm0_8
  long long v9; // rbp
  long long v10; // r12
  char *v11; // rax
  char v12[360]; // [rsp+0h] [rbp-1D8h] BYREF
  char v13[80]; // [rsp+168h] [rbp-70h] BYREF
  unsigned long long v14; // [rsp+1B8h] [rbp-20h]

  v0 = (const char *)qword_83F18;
  v14 = __readfsqword(0x28u);
  if ( !qword_83F18 )
  {
    v2 = sub_E130();
    v3 = v2;
    if ( !v2 )
      goto LABEL_9;
    v4 = v2[156];
    if ( v4 == 86 )
    {
      sub_23C20(&qword_83F18, v3, 100);
    }
    else if ( v4 == 103 )
    {
      sub_2F630(v12);
      v7 = sub_219B0(v3 + 124, 12);
      sub_1CAC0(v13, v3, v7);
      v8 = sub_1C8C0(v12);
      sub_2F6F0(v12, v8);
    }
    v0 = (const char *)qword_83F18;
    if ( !qword_83F18 )
    {
LABEL_9:
      if ( qword_84D50 )
        qword_84D50(v0);
      v5 = sub_4AC50(name);
      v6 = dcgettext(0, "Archive not labeled to match %s", 5);
      error(0, 0, v6, v5);
      sub_163D0(0);
    }
  }
  if ( !sub_DF40(v0) )
  {
    if ( qword_84D50 )
      qword_84D50(v0);
    v9 = sub_4AC40(1, name);
    v10 = sub_4AC40(0, qword_83F18);
    v11 = dcgettext(0, "Volume %s does not match %s", 5);
    error(0, 0, v11, v10, v9);
    sub_163D0(0);
  }
  return v14 - __readfsqword(0x28u);
}



// Function: close_archive @ 0xf900
void close_archive(double a1)
{
  if ( byte_84CF8 || dword_83F40 == 1 )
  {
    do
      sub_DFD0();
    while ( src > s );
  }
  sub_CD50();
  if ( byte_84A50 )
    sub_118D0();
  if ( fd <= 0x3FFFFFFF )
  {
    if ( !close(fd) )
      goto LABEL_7;
LABEL_11:
    sub_32680(*(long long *)qword_84828);
    goto LABEL_7;
  }
  if ( (unsigned int)sub_33D60((unsigned int)(fd - 0x40000000)) )
    goto LABEL_11;
LABEL_7:
  sub_2ACA0(dword_84168);
  sub_2F6F0(&qword_84840, a1);
  free(qword_84190);
  free(qword_84198);
  sub_BFC0(0);
}



// Function: _gnu_flush_read @ 0xf9e0
char *_gnu_flush_read(double a1)
{
  int v1; // edi
  size_t v2; // rdx
  char *result; // rax
  long long v4; // rax

  sub_108D0(0);
  dword_84164 = 0;
  if ( byte_83F20 && qword_84170 )
  {
    fd = 1;
    v4 = sub_2AF00(a1);
    fd = 0;
    v2 = v4;
    if ( nbytes != v4 )
      sub_D6D0(v4);
    v1 = 0;
    goto LABEL_4;
  }
  while ( 1 )
  {
    v1 = fd;
    v2 = nbytes;
LABEL_4:
    if ( v1 <= 0x3FFFFFFF )
      result = (char *)sub_4AF10(v1, s, v2);
    else
      result = (char *)sub_33DA0((unsigned int)(v1 - 0x40000000), s, v2);
    if ( (char *)nbytes == result )
    {
      ++qword_83F38;
      return result;
    }
    if ( !result )
      break;
    if ( result != (char *)-1LL )
      return sub_D390((unsigned long long)result);
    if ( *__errno_location() == 28 && byte_84B10 )
      goto LABEL_12;
    sub_D300();
  }
  if ( !byte_84B10 )
    return sub_D390((unsigned long long)result);
LABEL_12:
  while ( !(unsigned char)sub_E880(a1) )
    ;
  result = (char *)qword_83F50;
  if ( src == qword_83F50 )
    return (char *)sub_DFD0();
  return result;
}



// Function: gnu_flush_read @ 0xfb20
long long (*gnu_flush_read(double a1))()
{
  qword_84150 = (long long)sub_D770;
  sub_F9E0(a1);
  qword_84150 = (long long)gnu_flush_read;
  return gnu_flush_read;
}



// Function: flush_write @ 0xfb50
long long flush_write()
{
  return qword_84158(nbytes);
}



// Function: open_archive @ 0xfb70
long long open_archive(long long a1, double a2)
{
  qword_84150 = (long long)sub_FB20;
  qword_84158 = (long long (*)(long long))sub_E850;
  sub_F0F0(a1, a2);
  if ( (int)a1 == 1 )
  {
    qword_83F30 = 0;
    if ( name )
      sub_E330();
  }
  else if ( (a1 & 0xFFFFFFFD) == 0 && name )
  {
    sub_F760();
  }
  return sub_C070();
}



// Function: copy_string_unquote @ 0xfbf0
char *copy_string_unquote(void *a1)
{
  const char *v1; // rax
  char v2; // bl
  char *v3; // r12
  size_t v5; // rax
  size_t v6; // rbp

  v1 = (const char *)sub_4C7A0(a1);
  v2 = *v1;
  v3 = (char *)v1;
  if ( (*v1 == 34 || v2 == 39) && (v5 = strlen(v1), v6 = v5, v3[v5 - 1] == v2) )
  {
    memmove(v3, v3 + 1, v5 - 2);
    v3[v6 - 2] = 0;
    sub_23C70(v3);
    return v3;
  }
  else
  {
    sub_23C70(v3);
    return v3;
  }
}



// Function: alloc_action @ 0xfc60
long long alloc_action(int a1)
{
  long long result; // rax

  result = sub_4C760(24);
  if ( qword_84268 )
  {
    *(long long *)qword_84268 = result;
    *(int *)(result + 8) = a1;
  }
  else
  {
    *(int *)(result + 8) = a1;
    qword_84270 = result;
  }
  qword_84268 = result;
  return result;
}



// Function: getwidth @ 0xfcb0
long long getwidth(FILE *a1)
{
  int v1; // eax
  long long result; // rax
  char *v3; // rdi
  long long v4; // [rsp+0h] [rbp-18h] BYREF
  unsigned long long v5; // [rsp+8h] [rbp-10h]

  v5 = __readfsqword(0x28u);
  v1 = fileno(a1);
  if ( ioctl(v1, 0x5413u, &v4) || (result = WORD1(v4), !WORD1(v4)) )
  {
    v3 = getenv("COLUMNS");
    if ( !v3 )
      return 80;
    result = strtol(v3, 0, 10);
    if ( result <= 0 )
      return 80;
  }
  return result;
}



// Function: format_checkpoint_string @ 0xfdf0
long long format_checkpoint_string(FILE *a1, unsigned long long a2, char *a3, unsigned char a4, unsigned int a5, double a6)
{
  long long v7; // r13
  const char *v11; // r14
  char v12; // al
  char v13; // al
  char *v14; // rax
  const char *v16; // r12
  struct tm *v17; // rax
  long long v18; // rax
  char *v19; // rax
  const char **v20; // rsi
  unsigned long long v21; // r15
  long long v22; // rcx
  long long v23; // rdi
  const char *s; // [rsp+8h] [rbp-1A0h]
  unsigned int v26; // [rsp+14h] [rbp-194h]
  long long v27; // [rsp+18h] [rbp-190h]
  char *v28; // [rsp+28h] [rbp-180h] BYREF
  struct timeval tv; // [rsp+30h] [rbp-178h] BYREF
  long long v30[4]; // [rsp+40h] [rbp-168h] BYREF
  long long v31[30]; // [rsp+60h] [rbp-148h] BYREF
  char v32[24]; // [rsp+150h] [rbp-58h] BYREF
  unsigned long long v33; // [rsp+168h] [rbp-40h]

  v7 = a5;
  v33 = __readfsqword(0x28u);
  if ( a4 )
  {
    s = dcgettext(0, "write", 5);
    v11 = (const char *)sub_43090(v7, v32);
    if ( !a3 )
      a3 = dcgettext(0, "Write checkpoint %u", 5);
  }
  else
  {
    s = dcgettext(0, "read", 5);
    v11 = (const char *)sub_43090(v7, v32);
    if ( !a3 )
      a3 = dcgettext(0, "Read checkpoint %u", 5);
  }
  v28 = a3;
  v12 = *a3;
  if ( *a3 )
  {
    v26 = a5;
    while ( 1 )
    {
      if ( v12 != 37 )
      {
        ++a2;
        fputc_unlocked(v12, a1);
        v14 = v28;
        if ( *v28 == 13 )
        {
          dword_841D8 = 1;
          a2 = 0;
        }
        goto LABEL_12;
      }
      v28 = a3 + 1;
      v13 = a3[1];
      if ( v13 != 123 )
        break;
      v16 = (const char *)sub_FD40(a3 + 1, &v28);
      if ( v16 )
      {
        if ( *v28 != 42 )
        {
          switch ( *v28 )
          {
            case 'T':
              sub_CD50();
              v31[7] = ",";
              LODWORD(v18) = sub_382C0(v16, v31, 17988);
              if ( (int)v18 )
              {
                if ( qword_84D50 )
                  qword_84D50(v16);
                v27 = sub_354B0(v31);
                v19 = dcgettext(0, "cannot split string '%s': %s", 5);
                error(0, 0, v19, v16, v27);
                v20 = (const char **)&off_81140;
                dword_84D58 = 2;
              }
              else
              {
                v22 = v31[0];
                v20 = (const char **)v30;
                if ( !v31[0] )
                  goto LABEL_42;
                v23 = v31[1];
                v18 = 0;
                v20 = (const char **)v30;
                do
                {
                  v30[v18] = *(long long *)(v23 + 8 * v18);
                  ++v18;
                }
                while ( v18 != v22 );
                if ( (int)v18 != 3 )
                {
LABEL_42:
                  v18 = (int)v18;
                  do
                    v30[v18++] = 0;
                  while ( (int)v18 != 3 );
                }
              }
              a2 += (int)sub_CE80(a1, v20, 44, 0);
              sub_35420(v31);
              goto LABEL_17;
            case 'c':
LABEL_21:
              a2 += (int)format_checkpoint_string(a1, a2, "%{%Y-%m-%d %H:%M:%S}t: %ds, %{read,wrote}T%*\r", a4, v26);
              v14 = v28;
              break;
            case 'd':
LABEL_20:
              sub_CD50();
              a2 += (int)__fprintf_chk(a1, 1, "%.0f", a6);
              v14 = v28;
              break;
            case 's':
LABEL_19:
              fputs_unlocked(s, a1);
              a2 += strlen(s);
              v14 = v28;
              break;
            case 't':
LABEL_26:
              gettimeofday(&tv, 0);
              v17 = localtime(&tv.tv_sec);
              a2 += sub_41100(a1, v16, v17, 0, (unsigned int)(1000 * LODWORD(tv.tv_usec)));
              goto LABEL_17;
            case 'u':
LABEL_18:
              fputs_unlocked(v11, a1);
              a2 += strlen(v11);
              v14 = v28;
              break;
            default:
              goto LABEL_16;
          }
          goto LABEL_12;
        }
        v21 = strtol(v16, 0, 10);
        goto LABEL_34;
      }
LABEL_16:
      a2 += 2LL;
      fputc_unlocked(37, a1);
      fputc_unlocked(*v28, a1);
LABEL_17:
      v14 = v28;
LABEL_12:
      a3 = v14 + 1;
      v28 = v14 + 1;
      v12 = v14[1];
      if ( !v12 )
        goto LABEL_13;
    }
    if ( v13 != 42 )
    {
      switch ( v13 )
      {
        case 'T':
          sub_CD50();
          a2 += (int)sub_CE80(a1, (const char **)&off_81140, 44, 0);
          goto LABEL_17;
        case 'c':
          goto LABEL_21;
        case 'd':
          goto LABEL_20;
        case 's':
          goto LABEL_19;
        case 't':
          v16 = "%c";
          goto LABEL_26;
        case 'u':
          goto LABEL_18;
        default:
          goto LABEL_16;
      }
    }
    v21 = sub_FCB0(a1);
LABEL_34:
    if ( v21 > a2 )
    {
      do
      {
        ++a2;
        fputc_unlocked(32, a1);
      }
      while ( v21 != a2 );
    }
    goto LABEL_17;
  }
LABEL_13:
  fflush_unlocked(a1);
  return (unsigned int)a2;
}



// Function: run_checkpoint_actions @ 0x10270
unsigned long long run_checkpoint_actions(unsigned char a1, double a2)
{
  long long v2; // rbx
  FILE *v5; // rdi
  int v6; // eax
  FILE *v7; // rsi
  int sig; // [rsp+4h] [rbp-44h] BYREF
  unsigned long long i; // [rsp+8h] [rbp-40h]

  v2 = qword_84270;
  for ( i = __readfsqword(0x28u); v2; v2 = *(long long *)v2 )
  {
    switch ( *(int *)(v2 + 8) )
    {
      case 0:
        fputc_unlocked(46, stream);
        fflush_unlocked(stream);
        break;
      case 1:
        v7 = qword_841D0;
        if ( qword_841D0 || (qword_841D0 = fopen("/dev/tty", "w"), (v7 = qword_841D0) != 0) )
        {
          fputc_unlocked(7, v7);
          fflush_unlocked(qword_841D0);
        }
        break;
      case 2:
        a2 = __fprintf_chk(stderr, 1, "%s: ", (const char *)qword_84DD0);
        sub_FDF0(stderr, v6, *(char **)(v2 + 16), a1, dword_84278, a2);
        fputc_unlocked(10, stderr);
        break;
      case 3:
        v5 = qword_841D0;
        if ( qword_841D0 || (qword_841D0 = fopen("/dev/tty", "w"), (v5 = qword_841D0) != 0) )
          sub_FDF0(v5, 0, *(char **)(v2 + 16), a1, dword_84278, a2);
        break;
      case 4:
        sleep(*(int *)(v2 + 16));
        break;
      case 5:
        sub_2BB40(*(long long *)(v2 + 16), *(long long *)qword_84828, (unsigned int)dword_84278);
        break;
      case 6:
        sub_CD50();
        sub_D150();
        break;
      case 7:
        sigwait(&set, &sig);
        break;
      default:
        continue;
    }
  }
  return i - __readfsqword(0x28u);
}



// Function: checkpoint_compile_action @ 0x10490
int checkpoint_compile_action(char *s1)
{
  long long v2; // rax
  int v3; // edi
  long long v4; // rbx
  int v5; // eax
  long long v6; // rbx
  long long v7; // rbx
  unsigned long long v8; // rbx
  const char *v9; // rsi
  char *v10; // rax
  char *v12; // [rsp+0h] [rbp-28h] BYREF
  unsigned long long v13; // [rsp+8h] [rbp-20h]

  v13 = __readfsqword(0x28u);
  if ( !dword_84260 )
  {
    sigemptyset(&set);
    dword_84260 = 1;
  }
  if ( strcmp(s1, ".") && strcmp(s1, "dot") )
  {
    if ( !strcmp(s1, "bell") )
    {
      LODWORD(v2) = sub_FC60(1);
      return v2;
    }
    if ( !strcmp(s1, "echo") )
    {
      LODWORD(v2) = sub_FC60(2);
      return v2;
    }
    v3 = 2;
    if ( strncmp(s1, "echo=", 5u) )
    {
      if ( strncmp(s1, "exec=", 5u) )
      {
        if ( !strncmp(s1, "ttyout=", 7u) )
        {
          v7 = sub_FC60(3);
          *(long long *)(v7 + 16) = sub_FBF0(s1 + 7);
        }
        else
        {
          if ( strncmp(s1, "sleep=", 6u) )
          {
            if ( !strcmp(s1, "totals") )
            {
              LODWORD(v2) = sub_FC60(6);
              return v2;
            }
            if ( !strncmp(s1, "wait=", 5u) )
            {
              v4 = sub_FC60(7);
              v5 = sub_2C8A0(s1 + 5);
              *(int *)(v4 + 16) = v5;
              LODWORD(v2) = sigaddset(&set, v5);
              return v2;
            }
            if ( qword_84D50 )
              qword_84D50(s1);
            v9 = "%s: unknown checkpoint action";
LABEL_27:
            v10 = dcgettext(0, v9, 5);
            error(0, 0, v10, s1);
            sub_163D0(0);
          }
          v8 = strtoul(s1 + 6, &v12, 10);
          if ( *v12 )
          {
            if ( qword_84D50 )
              qword_84D50(s1 + 6);
            v9 = "%s: not a valid timeout";
            goto LABEL_27;
          }
          *(long long *)(sub_FC60(4) + 16) = v8;
        }
        return v13 - __readfsqword(0x28u);
      }
      v3 = 5;
    }
    v6 = sub_FC60(v3);
    *(long long *)(v6 + 16) = sub_FBF0(s1 + 5);
    return v13 - __readfsqword(0x28u);
  }
  LODWORD(v2) = sub_FC60(0);
  return v2;
}



// Function: checkpoint_finish_compile @ 0x107a0
long long checkpoint_finish_compile()
{
  long long result; // rax

  result = (unsigned int)dword_84260;
  if ( !dword_84260 )
  {
    if ( !dword_84B78 || qword_84270 )
      return result;
    sub_10490("echo");
    result = (unsigned int)dword_84260;
  }
  if ( (int)result == 1 )
  {
    sigprocmask(0, &set, 0);
    result = (unsigned int)dword_84B78;
    if ( !dword_84B78 )
      dword_84B78 = 10;
    dword_84260 = 2;
  }
  return result;
}



// Function: checkpoint_flush_actions @ 0x10820
void checkpoint_flush_actions()
{
  long long i; // rbx
  long long v1; // rax
  FILE *v2; // rsi
  long long j; // rbp
  FILE *v4; // [rsp+8h] [rbp-20h]

  for ( i = qword_84270; i; i = *(long long *)i )
  {
    while ( *(int *)(i + 8) != 3 || !qword_841D0 || !dword_841D8 )
    {
      i = *(long long *)i;
      if ( !i )
        return;
    }
    v4 = qword_841D0;
    v1 = sub_FCB0(qword_841D0);
    v2 = v4;
    if ( v1 )
    {
      for ( j = v1 - 2; ; --j )
      {
        fputc_unlocked(32, v2);
        v2 = qword_841D0;
        if ( j == -1 )
          break;
      }
    }
    fputc_unlocked(13, v2);
    fflush_unlocked(qword_841D0);
  }
}



// Function: checkpoint_run @ 0x108d0
unsigned long long checkpoint_run(unsigned char a1, double a2)
{
  unsigned long long result; // rax

  if ( dword_84B78 )
  {
    result = ++dword_84278 / (unsigned int)dword_84B78;
    if ( !(dword_84278 % (unsigned int)dword_84B78) )
      return sub_10270(a1, a2);
  }
  return result;
}



// Function: checkpoint_finish @ 0x10910
void checkpoint_finish()
{
  if ( dword_84B78 )
  {
    sub_10820();
    if ( qword_841D0 )
      fclose(qword_841D0);
  }
}



// Function: process_noop @ 0x10950
long long process_noop()
{
  return 1;
}



// Function: dumpdir_cmp @ 0x10960
long long dumpdir_cmp(char *s, char *s2)
{
  size_t v4; // rax

  while ( 1 )
  {
    do
    {
      if ( !*s )
        return (unsigned int)*s2;
    }
    while ( (unsigned char)(*s - 68) > 0x15u );
    if ( ((1LL << (*s - 68)) & 0x114000) != 0 )
      return (unsigned int)*s2;
    if ( ((1LL << (*s - 68)) & 0x200400) != 0 )
      break;
    if ( ((1 << (*s - 68)) & 1) != 0 )
    {
      if ( strcmp(s, s2) )
        return 1;
LABEL_7:
      v4 = strlen(s) + 1;
      s += v4;
      s2 += v4;
    }
  }
  if ( strchr("YN", *s2) && !strcmp(s + 1, s2 + 1) )
    goto LABEL_7;
  return 1;
}



// Function: diff_init @ 0x10af0
unsigned long long diff_init()
{
  long long v1; // [rsp+0h] [rbp-18h] BYREF
  unsigned long long v2; // [rsp+8h] [rbp-10h]

  v2 = __readfsqword(0x28u);
  qword_84288 = (void *)sub_25000(&v1, nbytes);
  if ( qword_84B28 )
    sub_1FD80();
  return v2 - __readfsqword(0x28u);
}



// Function: report_difference @ 0x10b50
unsigned long long report_difference(long long a1, long long a2, ...)
{
  const char *v2; // rax
  double v3; // xmm0_8
  gcc_va_list va; // [rsp+0h] [rbp-D8h] BYREF
  unsigned long long v6; // [rsp+18h] [rbp-C0h]

  v6 = __readfsqword(0x28u);
  if ( a2 )
  {
    v2 = (const char *)sub_23BB0(1, *(long long *)(a1 + 8));
    v3 = __fprintf_chk(stream, 1, "%s: ", v2);
    va_start(va, a2);
    ((void (*)(FILE *, long long, long long, __va_list_tag *, double))__vfprintf_chk)(stream, 1, a2, va, v3);
    __fprintf_chk(stream, 1, "\n");
  }
  sub_2F7D0(1);
  return v6 - __readfsqword(0x28u);
}



// Function: process_rawdata @ 0x10c80
long long process_rawdata(size_t nbytes, void *s1)
{
  long long v2; // rax
  long long v3; // r13
  char *v4; // rax
  long long result; // rax
  int v6; // r8d
  char *v7; // rax

  v2 = sub_24960(dword_84290, qword_84288, nbytes);
  if ( nbytes == v2 )
  {
    v6 = memcmp(s1, qword_84288, nbytes);
    result = 1;
    if ( v6 )
    {
      v7 = dcgettext(0, "Contents differ", 5);
      sub_10B50((long long)&qword_84840, (long long)v7);
      return 0;
    }
  }
  else
  {
    v3 = v2;
    if ( v2 == -1 )
    {
      sub_32820(path);
      sub_10B50((long long)&qword_84840, 0);
      return 0;
    }
    else
    {
      v4 = dcngettext(0, "Could only read %lu of %lu byte", "Could only read %lu of %lu bytes", nbytes, 5);
      sub_10B50((long long)&qword_84840, (long long)v4, v3, nbytes);
      return 0;
    }
  }
  return result;
}



// Function: get_stat_data @ 0x10d70
long long get_stat_data(char *a1, struct stat *a2)
{
  int v2; // r8d
  long long result; // rax

  v2 = sub_246F0(a1, a2);
  result = 1;
  if ( v2 )
  {
    if ( *__errno_location() == 2 )
      sub_32D50();
    else
      sub_32D30();
    sub_10B50((long long)&qword_84840, 0);
    return 0;
  }
  return result;
}



// Function: diff_file @ 0x10dd0
unsigned long long diff_file()
{
  char *v0; // r12
  long long v1; // rax
  long long v2; // rdx
  const char *v3; // rsi
  int v4; // eax
  char *v5; // rax
  char *v7; // rax
  char *v8; // rax
  char *v9; // rax
  char *v10; // rax
  long long v11; // rax
  long long v12; // rdx
  struct stat _0; // [rsp+0h] [rbp+0h] BYREF
  unsigned long long vars98; // [rsp+98h] [rbp+98h]

  v0 = path;
  vars98 = __readfsqword(0x28u);
  if ( !(unsigned int)sub_10D70(path, &_0) )
    goto LABEL_21;
  if ( (_0.st_mode & 0xF000) != 0x8000 )
  {
    v3 = "File type differs";
LABEL_20:
    v5 = dcgettext(0, v3, 5);
    sub_10B50((long long)&qword_84840, (long long)v5);
LABEL_21:
    sub_22BD0();
    return vars98 - __readfsqword(0x28u);
  }
  if ( ((buf.st_mode ^ _0.st_mode) & 0xFFF) != 0 )
  {
    v10 = dcgettext(0, "Mode differs", 5);
    sub_10B50((long long)&qword_84840, (long long)v10);
  }
  if ( !(unsigned char)sub_2AE90(&_0, &buf) )
  {
    v9 = dcgettext(0, "Uid differs", 5);
    sub_10B50((long long)&qword_84840, (long long)v9);
  }
  if ( !(unsigned char)sub_2AEA0(&_0, &buf) )
  {
    v8 = dcgettext(0, "Gid differs", 5);
    sub_10B50((long long)&qword_84840, (long long)v8);
  }
  v1 = sub_4BD20(&_0);
  if ( (unsigned int)sub_2F7B0(v1, v2, file, *(&file + 1)) )
  {
    v7 = dcgettext(0, "Mod time differs", 5);
    sub_10B50((long long)&qword_84840, (long long)v7);
  }
  if ( *(char *)(qword_84408 + 156) != 83 )
  {
    v3 = "Size differs";
    if ( _0.st_size != buf.st_size )
      goto LABEL_20;
  }
  v4 = __openat_2((unsigned int)oldfd, v0, (unsigned int)dword_84808);
  dword_84290 = v4;
  if ( v4 < 0 )
  {
    sub_327C0(v0);
    sub_22BD0();
    sub_10B50((long long)&qword_84840, 0);
  }
  else
  {
    if ( byte_84960 )
      sub_2A070((unsigned int)v4, &qword_84840);
    else
      sub_10A20(sub_10C80);
    if ( dword_84B88 == 1 )
    {
      if ( _0.st_size )
      {
        v11 = sub_4BD00(&_0);
        if ( (unsigned int)sub_24A80((unsigned int)dword_84290, (unsigned int)oldfd, v0, v11, v12) )
          sub_32DD0(v0);
      }
    }
    if ( close(dword_84290) )
      sub_32680(v0);
  }
  return vars98 - __readfsqword(0x28u);
}



// Function: diff_link @ 0x11090
unsigned long long diff_link()
{
  long long v1; // r12
  char *v2; // rax
  struct stat vars0; // [rsp+0h] [rbp+0h] BYREF
  struct stat vars90; // [rsp+90h] [rbp+90h] BYREF
  unsigned long long vars128; // [rsp+128h] [rbp+128h]

  vars128 = __readfsqword(0x28u);
  if ( (unsigned int)sub_10D70(path, &vars0)
    && (unsigned int)sub_10D70(from, &vars90)
    && !(unsigned char)sub_2AEB0(&vars0, &vars90) )
  {
    v1 = sub_23BB0(0, from);
    v2 = dcgettext(0, "Not linked to %s", 5);
    sub_10B50((long long)&qword_84840, (long long)v2, v1);
  }
  return vars128 - __readfsqword(0x28u);
}



// Function: diff_symlink @ 0x11160
unsigned long long diff_symlink()
{
  size_t v0; // rax
  char *v1; // rbp
  size_t v2; // r12
  size_t v3; // r13
  ssize_t v4; // rax
  char *v5; // rax
  char v7[1032]; // [rsp+0h] [rbp-438h] BYREF
  unsigned long long v8; // [rsp+408h] [rbp-30h]

  v8 = __readfsqword(0x28u);
  v0 = strlen(from);
  v1 = v7;
  v2 = v0;
  v3 = v0 + 1;
  if ( v0 > 0x3FF )
    v1 = (char *)sub_4C5B0(v0 + 1);
  v4 = readlinkat(oldfd, path, v1, v3);
  if ( v4 < 0 )
  {
    if ( *__errno_location() == 2 )
      sub_32A80(path);
    else
      sub_32A60(path);
    sub_10B50((long long)&qword_84840, 0);
  }
  else if ( v2 != v4 || memcmp(from, v1, v2) )
  {
    v5 = dcgettext(0, "Symlink differs", 5);
    sub_10B50((long long)&qword_84840, (long long)v5);
  }
  if ( v1 != v7 )
    free(v1);
  return v8 - __readfsqword(0x28u);
}



// Function: diff_special @ 0x11280
unsigned long long diff_special()
{
  char v0; // dl
  int v1; // ecx
  const char *v3; // rsi
  char *v4; // rax
  char *v5; // rax
  struct stat v6; // [rsp+0h] [rbp-A8h] BYREF
  unsigned long long v7; // [rsp+98h] [rbp-10h]

  v7 = __readfsqword(0x28u);
  if ( (unsigned int)sub_10D70(path, &v6) )
  {
    v0 = *(char *)(qword_84408 + 156);
    v1 = v6.st_mode & 0xF000;
    if ( v0 == 51 )
    {
      if ( v1 != 0x2000 )
        goto LABEL_12;
    }
    else
    {
      if ( v0 != 52 )
      {
        if ( v1 == 4096 )
        {
LABEL_5:
          if ( ((LOWORD(buf.st_mode) ^ LOWORD(v6.st_mode)) & 0xFFF) == 0 )
            return v7 - __readfsqword(0x28u);
          v3 = "Mode differs";
LABEL_9:
          v4 = dcgettext(0, v3, 5);
          sub_10B50((long long)&qword_84840, (long long)v4);
          return v7 - __readfsqword(0x28u);
        }
LABEL_12:
        v5 = dcgettext(0, "File type differs", 5);
        sub_10B50((long long)&qword_84840, (long long)v5);
        return v7 - __readfsqword(0x28u);
      }
      if ( v1 != 24576 )
        goto LABEL_12;
    }
    v3 = "Device number differs";
    if ( buf.st_rdev != v6.st_rdev )
      goto LABEL_9;
    goto LABEL_5;
  }
  return v7 - __readfsqword(0x28u);
}



// Function: diff_dir @ 0x113b0
unsigned long long diff_dir()
{
  const char *v1; // rsi
  char *v2; // rax
  struct stat v3; // [rsp+0h] [rbp-A8h] BYREF
  unsigned long long v4; // [rsp+98h] [rbp-10h]

  v4 = __readfsqword(0x28u);
  if ( (unsigned int)sub_10D70(path, &v3) )
  {
    if ( (v3.st_mode & 0xF000) == 0x4000 )
    {
      if ( ((LOWORD(buf.st_mode) ^ LOWORD(v3.st_mode)) & 0xFFF) == 0 )
        return v4 - __readfsqword(0x28u);
      v1 = "Mode differs";
    }
    else
    {
      v1 = "File type differs";
    }
    v2 = dcgettext(0, v1, 5);
    sub_10B50((long long)&qword_84840, (long long)v2);
  }
  return v4 - __readfsqword(0x28u);
}



// Function: diff_multivol @ 0x11470
unsigned long long diff_multivol()
{
  const char *v0; // rsi
  long long v1; // rax
  __off_t v2; // rbp
  bool v3; // of
  __off_t v4; // rax
  int v5; // eax
  int v6; // r13d
  char *v8; // rax
  struct stat v9; // [rsp+0h] [rbp-B8h] BYREF
  unsigned long long v10; // [rsp+98h] [rbp-20h]

  v10 = __readfsqword(0x28u);
  if ( !byte_84850 )
  {
    if ( (unsigned int)sub_10D70(path, &v9) )
    {
      v0 = "File type differs";
      if ( (v9.st_mode & 0xF000) == 0x8000 )
      {
        v1 = sub_219B0(qword_84408 + 369, 12);
        v2 = v1;
        if ( v1 >= 0 )
        {
          v3 = __OFADD__(buf.st_size, v1);
          v4 = buf.st_size + v1;
          if ( !v3 && v9.st_size == v4 )
          {
            v5 = __openat_2((unsigned int)oldfd, path, (unsigned int)dword_84808);
            v6 = v5;
            if ( v5 < 0 )
            {
              sub_327C0(path);
              sub_10B50((long long)&qword_84840, 0);
              sub_22BD0();
            }
            else
            {
              if ( lseek(v5, v2, 0) < 0 )
              {
                sub_32B20(path, v2);
                sub_10B50((long long)&qword_84840, 0);
              }
              else
              {
                sub_10A20(sub_10C80);
              }
              if ( close(v6) )
                sub_32680(path);
            }
            return v10 - __readfsqword(0x28u);
          }
        }
        v0 = "Size differs";
      }
      v8 = dcgettext(0, v0, 5);
      sub_10B50((long long)&qword_84840, (long long)v8);
      sub_22BD0();
    }
    return v10 - __readfsqword(0x28u);
  }
  return sub_113B0();
}



// Function: diff_archive @ 0x11730
unsigned long long diff_archive()
{
  long long *v0; // rdi
  unsigned char v1; // bl
  unsigned long long result; // rax
  long long v3; // r12
  char *v4; // rax
  char *v5; // rax

  v0 = (long long *)qword_84408;
  sub_D1F0(qword_84408);
  if ( dword_84A54 )
  {
    if ( byte_84280 )
    {
      v5 = dcgettext(0, "Verify ", 5);
      __fprintf_chk(stream, 1, v5);
    }
    v0 = &qword_84840;
    sub_22990(&qword_84840, qword_84408, -1);
  }
  v1 = *(char *)(qword_84408 + 156);
  result = v1;
  switch ( v1 )
  {
    case 0u:
    case 0x30u:
    case 0x37u:
    case 0x53u:
      goto LABEL_9;
    case 0x31u:
      return sub_11090();
    case 0x32u:
      return sub_11160();
    case 0x33u:
    case 0x34u:
    case 0x36u:
      return sub_11280();
    case 0x35u:
    case 0x44u:
      if ( (unsigned char)sub_201B0(&qword_84840) )
        sub_11650();
      return sub_113B0();
    case 0x4Du:
      return sub_11470();
    case 0x56u:
      return result;
    default:
      if ( qword_84D50 )
      {
        qword_84D50(v0);
        v1 = *(char *)(qword_84408 + 156);
      }
      v3 = sub_4AA70(path);
      v4 = dcgettext(0, "%s: Unknown file type '%c', diffed as normal file", 5);
      error(0, 0, v4, v3, (unsigned int)(char)v1);
      dword_84D58 = 2;
LABEL_9:
      if ( byte_84850 )
        return sub_113B0();
      else
        return sub_10DD0();
  }
}



// Function: verify_volume @ 0x118d0
unsigned long long verify_volume(long long a1, double a2)
{
  int v2; // eax
  int *v3; // rax
  int v4; // edi
  int v5; // r14d
  char *v6; // rax
  char *v7; // rax
  char *v8; // rax
  long long v9; // rdx
  long long v10; // rax
  long long v11; // r12
  char *v12; // rax
  unsigned int v13; // r12d
  char *v14; // rax
  int v16; // eax
  short v17[2]; // [rsp+8h] [rbp-60h] BYREF
  int v18; // [rsp+Ch] [rbp-5Ch]
  char v19[24]; // [rsp+10h] [rbp-58h] BYREF
  unsigned long long v20; // [rsp+28h] [rbp-40h]

  v20 = __readfsqword(0x28u);
  if ( !(unsigned char)sub_32FD0() )
  {
    if ( !(unsigned char)sub_30840() )
    {
      sub_1F510();
      if ( qword_84288 )
        goto LABEL_4;
      goto LABEL_24;
    }
    goto LABEL_18;
  }
  if ( qword_84D50 )
    qword_84D50(a1);
  v6 = dcgettext(0, "Archive contains file names with leading prefixes removed.", 5);
  a1 = 0;
  error(0, 0, v6);
  if ( (unsigned char)sub_30840() )
  {
LABEL_18:
    if ( qword_84D50 )
      qword_84D50(a1);
    v7 = dcgettext(0, "Archive contains transformed file names.", 5);
    error(0, 0, v7);
  }
  if ( qword_84D50 )
    qword_84D50(0);
  v8 = dcgettext(0, "Verification may fail to locate original files.", 5);
  error(0, 0, v8);
  sub_1F510();
  if ( !qword_84288 )
LABEL_24:
    sub_10AF0();
LABEL_4:
  fsync(fd);
  ioctl(fd, 0x24Bu);
  v18 = 1;
  v17[0] = 2;
  if ( fd <= 0x3FFFFFFF )
    v2 = ioctl(fd, 0x40086D01u, v17);
  else
    v2 = sub_34080((unsigned int)(fd - 0x40000000), 1074294017, v17);
  if ( v2 >= 0 )
    goto LABEL_10;
  v3 = __errno_location();
  v4 = fd;
  if ( *v3 != 5 )
  {
    if ( fd > 0x3FFFFFFF )
      goto LABEL_9;
LABEL_39:
    if ( !lseek(v4, 0, 0) )
      goto LABEL_10;
LABEL_40:
    sub_32BD0(*(long long *)ptr);
    return v20 - __readfsqword(0x28u);
  }
  if ( fd > 0x3FFFFFFF )
    v16 = sub_34080((unsigned int)(fd - 0x40000000), 1074294017, v17);
  else
    v16 = ioctl(fd, 0x40086D01u, v17);
  v4 = fd;
  if ( v16 >= 0 )
    goto LABEL_10;
  if ( fd <= 0x3FFFFFFF )
    goto LABEL_39;
LABEL_9:
  if ( sub_33F90((unsigned int)(v4 - 0x40000000), 0, 0) )
    goto LABEL_40;
LABEL_10:
  byte_84280 = 1;
  dword_83F40 = 0;
  sub_DFC0();
  while ( 1 )
  {
    v5 = sub_219E0(&qword_84408, &qword_84840, 0);
    if ( v5 != 5 )
    {
      if ( v5 == 4 )
        break;
      goto LABEL_13;
    }
    v13 = 0;
    do
    {
      ++v13;
      sub_D1F0(qword_84408);
      v5 = sub_219E0(&qword_84408, &qword_84840, 0);
    }
    while ( v5 == 5 );
    if ( qword_84D50 )
      qword_84D50(&qword_84408);
    v14 = dcngettext(
            0,
            "VERIFY FAILURE: %d invalid header detected",
            "VERIFY FAILURE: %d invalid headers detected",
            (int)v13,
            5);
    error(0, 0, v14, v13);
    dword_84D58 = 2;
    if ( v5 == 4 )
      break;
LABEL_13:
    if ( v5 == 3 )
    {
      sub_D1F0(qword_84408);
      if ( !byte_84B51 )
      {
        if ( (unsigned int)sub_219E0(&qword_84408, &qword_84840, 0) == 3 )
          break;
        if ( (dword_83B80 & 1) != 0 )
        {
          if ( qword_84D50 )
            qword_84D50(&qword_84408);
          v10 = sub_D180(&qword_84408, &qword_84840, v9);
          v11 = sub_43090(v10, v19);
          v12 = dcgettext(0, "A lone zero block at %s", 5);
          error(0, 0, v12, v11);
        }
      }
    }
    else
    {
      sub_215C0(qword_84408, &qword_84840, &dword_84400, 1);
      sub_11730();
      sub_2F6F0(&qword_84840, a2);
    }
  }
  dword_83F40 = 1;
  byte_84280 = 0;
  return v20 - __readfsqword(0x28u);
}



// Function: to_octal @ 0x11ce0
long long to_octal(unsigned long long a1, long long a2, long long a3)
{
  char v3; // al
  long long result; // rax

  do
  {
    v3 = a1;
    --a3;
    a1 >>= 3;
    result = (v3 & 7u) + 48;
    *(char *)(a2 + a3) = result;
  }
  while ( a3 );
  return result;
}



// Function: tar_copy_str @ 0x11d00
long long tar_copy_str(long long a1, long long a2, long long a3)
{
  long long result; // rax
  char v4; // cl

  result = 0;
  do
  {
    v4 = *(char *)(a2 + result);
    *(char *)(a1 + result) = v4;
    if ( !v4 )
      break;
    ++result;
  }
  while ( a3 != result );
  return result;
}



// Function: to_base256 @ 0x11d20
void to_base256(int a1, unsigned long long a2, long long a3, long long a4)
{
  unsigned long long v4; // rdi

  v4 = (unsigned long long)(unsigned int)-a1 << 56;
  do
  {
    --a4;
    *(char *)(a3 + a4) = a2;
    a2 = v4 | (a2 >> 8);
  }
  while ( a4 );
}



// Function: file_dumpable_p @ 0x11d50
char file_dumpable_p(long long a1)
{
  int v1; // ecx
  char result; // al
  long long v3; // rdx
  long long v4; // rax

  v1 = *(int *)(a1 + 24);
  result = 1;
  if ( (v1 & 0xF000) != 0x4000 )
  {
    result = 0;
    if ( (v1 & 0xF000) == 0x8000 )
    {
      if ( byte_84A30 )
      {
        result = byte_84A69;
        if ( byte_84A69 )
        {
          result = byte_84A88;
          if ( byte_84A88 )
          {
            v3 = *(long long *)(a1 + 48);
            v4 = v3 / 512;
            if ( (v3 & 0x1FF) != 0 && (unsigned long long)(v3 + 511) > 0x3FE )
              ++v4;
            return *(long long *)(a1 + 64) < v4;
          }
        }
      }
      else
      {
        result = 1;
        if ( !*(long long *)(a1 + 48) )
          return (v1 & 0x124) != 292;
      }
    }
  }
  return result;
}



// Function: split_long_name @ 0x11e00
long long split_long_name(long long a1, unsigned long long a2)
{
  long long result; // rax

  if ( a2 > 0x9C )
  {
    result = 155;
  }
  else
  {
    result = a2 - 1;
    if ( *(char *)(a1 + a2 - 1) == 47 )
    {
      result = a2 - 2;
      if ( a2 == 2 )
        return result;
    }
    else if ( a2 == 1 )
    {
      return result;
    }
  }
  do
  {
    if ( *(char *)(a1 + result) == 47 )
      break;
    --result;
  }
  while ( result );
  return result;
}



// Function: hash_link @ 0x11e50
unsigned long long hash_link(long long *a1, unsigned long long a2)
{
  return (a1[1] ^ *a1) % a2;
}



// Function: compare_links @ 0x11e70
bool compare_links(_OWORD *a1, _OWORD *a2)
{
  return *a2 - *a1 == 0;
}



// Function: to_chars_subst @ 0x11e90
long long to_chars_subst(
        int a1,
        int a2,
        long long a3,
        int a4,
        long long (*a5)(unsigned int *),
        int a6,
        long long a7,
        long long a8)
{
  long long v10; // r15
  long long v11; // r12
  long long v12; // rbp
  long long v13; // rax
  const char *v14; // r15
  long long v15; // rdi
  long long v16; // rax
  long long v17; // r13
  long long v18; // r12
  long long v19; // rax
  long long v20; // rdi
  long long v21; // rbx
  char *v22; // rax
  char *v24; // rax
  unsigned int v27; // [rsp+3Ch] [rbp-BCh] BYREF
  char v28[32]; // [rsp+40h] [rbp-B8h] BYREF
  char v29; // [rsp+60h] [rbp-98h] BYREF
  char v30[32]; // [rsp+61h] [rbp-97h] BYREF
  char v31[32]; // [rsp+81h] [rbp-77h] BYREF
  char v32[23]; // [rsp+A1h] [rbp-57h] BYREF
  unsigned long long v33; // [rsp+B8h] [rbp-40h]

  v33 = __readfsqword(0x28u);
  if ( a2 )
  {
    if ( (unsigned long long)(8 * (a7 - 1)) > 0x3F )
    {
      v11 = -1;
      v10 = 0x8000000000000000LL;
      v12 = sub_43090(-1, v28);
    }
    else
    {
      v10 = 1LL << (8 * ((unsigned char)a7 - 1));
      v11 = v10 - 1;
      v12 = sub_43090(v10 - 1, v28);
    }
    v13 = sub_43090(v10, v31);
    *(char *)(v13 - 1) = 45;
    v14 = (const char *)(v13 - 1);
LABEL_5:
    if ( a1 )
      goto LABEL_6;
LABEL_16:
    v15 = a3;
    v17 = sub_43090(a3, &v29);
    if ( a5 )
      goto LABEL_7;
    goto LABEL_17;
  }
  if ( (unsigned long long)(3 * a7 - 3) > 0x3F )
  {
    v11 = -1;
    v14 = "0";
    v12 = sub_43090(-1, v28);
    goto LABEL_5;
  }
  v14 = "0";
  v11 = (1LL << (3 * (unsigned char)a7 - 3)) - 1;
  v12 = sub_43090(v11, v28);
  if ( !a1 )
    goto LABEL_16;
LABEL_6:
  v15 = -a3;
  v16 = sub_43090(-a3, v30);
  *(char *)(v16 - 1) = 45;
  v17 = v16 - 1;
  if ( a5 )
  {
LABEL_7:
    v18 = a5(&v27) & v11;
    v27 = (unsigned char)v27 & (dword_84BA4 == 6);
    if ( v27 )
      v18 = -v18;
    v19 = sub_43090(v18, v32);
    v20 = v27;
    v21 = v19;
    if ( v27 )
    {
      *(char *)(v19 - 1) = 45;
      v21 = v19 - 1;
    }
    if ( qword_84D50 )
      qword_84D50(v20);
    v22 = dcgettext(0, "value %s out of %s range %s..%s; substituting %s", 5);
    error(0, 0, v22, v17, a8, v14, v12, v21);
    return sub_12150(v27, v18, a4, 0, a6, a7, a8);
  }
LABEL_17:
  if ( qword_84D50 )
    qword_84D50(v15);
  v24 = dcgettext(0, "value %s out of %s range %s..%s", 5);
  error(0, 0, v24, v17, a8, v14, v12);
  dword_84D58 = 2;
  return 0;
}



// Function: to_chars @ 0x12150
void to_chars(
        long long a1,
        unsigned long long a2,
        long long a3,
        long long (*a4)(unsigned int *),
        char *a5,
        long long a6,
        long long a7)
{
  unsigned long long v9; // r12
  unsigned int v10; // eax
  long long v11; // r14
  char v12; // al
  char *v13; // rax
  char *v14; // [rsp+0h] [rbp-38h]
  char *v15; // [rsp+0h] [rbp-38h]
  long long v16; // [rsp+8h] [rbp-30h]
  long long v17; // [rsp+8h] [rbp-30h]

  v9 = a2;
  v10 = dword_84BA4 & 0xFFFFFFFB;
  if ( (int)a1 )
  {
    if ( v10 != 2 )
      goto LABEL_20;
    v11 = a6 - 1;
    if ( (unsigned long long)(8 * (a6 - 1)) > 0x3F || ~a2 <= (1LL << (8 * (unsigned char)v11)) - 1 )
    {
      v12 = -1;
      goto LABEL_8;
    }
    if ( 8 * a3 > (unsigned long long)(3 * a6 - 3) )
      goto LABEL_21;
    if ( !dword_842B8 )
    {
      dword_842B8 = 1;
      if ( qword_84D50 )
      {
        v16 = a6;
        v14 = a5;
        qword_84D50(a1);
        a6 = v16;
        a5 = v14;
      }
      v17 = a6;
      v15 = a5;
      v13 = dcgettext(0, "Generating negative octal headers", 5);
      error(0, 0, v13);
      a6 = v17;
      a5 = v15;
    }
    a5[a6 - 1] = 0;
    if ( (unsigned long long)(8 * a3) <= 0x3F )
      v9 = ~(-1LL << (8 * (unsigned char)a3)) & a2;
    sub_11CE0(v9, (long long)a5, v11);
  }
  else
  {
    if ( (unsigned long long)(3 * a6 - 3) <= 0x3F && (1LL << (3 * (unsigned char)a6 - 3)) - 1 < a2 )
    {
      if ( v10 == 2 )
      {
        v11 = a6 - 1;
        if ( (unsigned long long)(8 * (a6 - 1)) > 0x3F || (1LL << (8 * (unsigned char)v11)) - 1 >= a2 )
        {
          v12 = 0x80;
LABEL_8:
          *a5 = v12;
          sub_11D20(a1, a2, (long long)(a5 + 1), v11);
          return;
        }
        goto LABEL_21;
      }
LABEL_20:
      a4 = 0;
LABEL_21:
      sub_11E90(a1, v10 == 2, a2, a3, a4, (int)a5, a6, a7);
      return;
    }
    a5[a6 - 1] = 0;
    sub_11CE0(a2, (long long)a5, a6 - 1);
  }
}



// Function: uid_substitute @ 0x12380
long long uid_substitute(int *a1)
{
  long long result; // rax

  result = (unsigned int)dword_842B4;
  if ( dword_842B4 )
  {
    *a1 = 0;
  }
  else
  {
    if ( (unsigned int)sub_269F0("nobody") )
    {
      result = (unsigned int)dword_842B4;
    }
    else
    {
      dword_842B4 = -2;
      result = 4294967294LL;
    }
    *a1 = 0;
  }
  return result;
}



// Function: gid_substitute @ 0x123e0
long long gid_substitute(int *a1)
{
  long long result; // rax

  result = (unsigned int)dword_842B0;
  if ( dword_842B0 )
  {
    *a1 = 0;
  }
  else
  {
    if ( (unsigned int)sub_26AA0("nobody") )
    {
      result = (unsigned int)dword_842B0;
    }
    else
    {
      dword_842B0 = -2;
      result = 4294967294LL;
    }
    *a1 = 0;
  }
  return result;
}



// Function: write_ustar_long_name @ 0x12440
long long *write_ustar_long_name(const char *src)
{
  size_t v1; // rax
  size_t v2; // r12
  long long v3; // rax
  size_t v4; // rbx
  size_t v5; // r12
  long long *v6; // r13
  long long v8; // r12
  char *v9; // rax
  long long v10; // r12
  char *v11; // rax

  v1 = strlen(src);
  if ( v1 > 0x100 )
  {
    if ( qword_84D50 )
      qword_84D50(src);
    v10 = sub_4AA70(src);
    v11 = dcgettext(0, "%s: file name is too long (max %d); not dumped", 5);
    error(0, 0, v11, v10, 256);
    dword_84D58 = 2;
    return 0;
  }
  else
  {
    v2 = v1;
    v3 = sub_11E00((long long)src, v1);
    v4 = v3;
    if ( v3 && (v5 = v2 - 1 - v3, v5 - 1 <= 0x63) )
    {
      v6 = sub_E130();
      *v6 = 0;
      v6[63] = 0;
      memset(
        (void *)((unsigned long long)(v6 + 1) & 0xFFFFFFFFFFFFFFF8LL),
        0,
        8LL * (((unsigned int)v6 - (((int)v6 + 8) & 0xFFFFFFF8) + 512) >> 3));
      memcpy((char *)v6 + 345, src, v4);
      memcpy(v6, &src[v4 + 1], v5);
      return v6;
    }
    else
    {
      if ( qword_84D50 )
        qword_84D50(src);
      v8 = sub_4AA70(src);
      v9 = dcgettext(0, "%s: file name is too long (cannot be split); not dumped", 5);
      error(0, 0, v9, v8);
      dword_84D58 = 2;
      return 0;
    }
  }
}



// Function: open_failure_recover @ 0x125b0
long long open_failure_recover(long long a1)
{
  int *v1; // rax
  unsigned int v2; // r12d
  long long v4; // rax
  long long v5; // rax
  int v6; // edx
  long long v7; // rdi

  v1 = __errno_location();
  LOBYTE(v1) = a1 != 0 && *v1 == 24;
  v2 = (unsigned int)v1;
  if ( !(char)v1 )
    return v2;
  v4 = *(long long *)(a1 + 408);
  if ( v4 )
  {
    v5 = *(long long *)(v4 + 408);
    while ( v5 )
    {
      while ( 1 )
      {
        v6 = *(int *)(v5 + 424);
        v7 = v5;
        v5 = *(long long *)(v5 + 408);
        if ( v6 <= 0 )
          break;
        if ( !v5 || *(int *)(v5 + 424) <= 0 )
        {
          sub_2F670(v7);
          return v2;
        }
      }
    }
  }
  return 0;
}



// Function: unknown_file_error @ 0x12640
void unknown_file_error(long long a1)
{
  long long v1; // r12
  char *v2; // rax

  if ( (dword_83B80 & 0x20) == 0 )
  {
    if ( byte_84B52 )
      return;
    goto LABEL_7;
  }
  if ( qword_84D50 )
    qword_84D50(a1);
  v1 = sub_4AA70(a1);
  v2 = dcgettext(0, "%s: Unknown file type; file ignored", 5);
  error(0, 0, v2, v1);
  if ( !byte_84B52 )
LABEL_7:
    sub_2F7D0(2);
}



// Function: ensure_slash @ 0x126e0
const char *ensure_slash(const char **a1)
{
  const char *v1; // rbp
  size_t i; // rbx
  char *v3; // rax
  long long v4; // r13
  long long v5; // rax
  const char *result; // rax

  v1 = *a1;
  for ( i = strlen(*a1); ; --i )
  {
    if ( !i )
    {
      v3 = (char *)v1;
      v4 = 1;
      goto LABEL_6;
    }
    if ( v1[i - 1] != 47 )
      break;
  }
  v3 = (char *)&v1[i];
  v4 = i + 1;
LABEL_6:
  if ( *v3 != 47 )
  {
    v5 = sub_4C610(v1, i + 2);
    *a1 = (const char *)v5;
    v3 = (char *)(i + v5);
  }
  *v3 = 47;
  result = *a1;
  (*a1)[v4] = 0;
  return result;
}



// Function: file_count_links @ 0x12760
unsigned long long file_count_links(long long *a1)
{
  long long v3; // rdi
  long long v4; // rax
  size_t v5; // rax
  long long v6; // rax
  char *v7; // r12
  long long v8; // rbp
  char *v9; // rsi
  long long v10; // rdi
  long long v11; // rax
  char *s; // [rsp+0h] [rbp-28h] BYREF
  unsigned long long v13; // [rsp+8h] [rbp-20h]

  v13 = __readfsqword(0x28u);
  if ( !byte_84B68 && a1[13] > (unsigned long long)qword_842C8 )
  {
    v3 = *a1;
    s = 0;
    v4 = sub_33020(v3, 1, (unsigned char)byte_84B92);
    sub_23BE0(&s, v4);
    sub_30830(&s, 2);
    v5 = strlen(s);
    v6 = sub_4C5B0(v5 + 25);
    v7 = s;
    v8 = v6;
    v9 = s;
    *(long long *)(v6 + 8) = a1[12];
    *(long long *)v6 = a1[11];
    *(long long *)(v6 + 16) = a1[13];
    strcpy((char *)(v6 + 24), v9);
    free(v7);
    v10 = qword_842C0;
    if ( !qword_842C0 && (qword_842C0 = sub_41B30(0, 0, sub_11E50, sub_11E70, 0), (v10 = qword_842C0) == 0)
      || (v11 = sub_42140(v10, v8)) == 0 )
    {
      sub_16420(v10);
    }
    if ( v8 != v11 )
      abort();
    --*(long long *)(v8 + 16);
  }
  return v13 - __readfsqword(0x28u);
}



// Function: write_short_name @ 0x12990
long long write_short_name(long long a1)
{
  long long *v1; // rax
  long long v2; // r8

  v1 = sub_E130();
  *v1 = 0;
  v1[63] = 0;
  memset(
    (void *)((unsigned long long)(v1 + 1) & 0xFFFFFFFFFFFFFFF8LL),
    0,
    8LL * (((unsigned int)v1 - (((int)v1 + 8) & 0xFFFFFFF8) + 512) >> 3));
  sub_12970(v1, *(long long *)(a1 + 8));
  return v2;
}



// Function: add_exclusion_tag @ 0x12a70
size_t add_exclusion_tag(char *s, int a2, long long a3)
{
  long long v4; // rbx
  long long v5; // rax
  size_t result; // rax

  v4 = sub_4C5B0(40);
  v5 = qword_842D0;
  *(long long *)v4 = s;
  *(int *)(v4 + 16) = a2;
  *(long long *)(v4 + 24) = a3;
  *(long long *)(v4 + 32) = v5;
  result = strlen(s);
  qword_842D0 = v4;
  *(long long *)(v4 + 8) = result;
  return result;
}



// Function: exclusion_tag_warning @ 0x12ad0
void exclusion_tag_warning(long long a1, long long a2, long long a3)
{
  long long v4; // r13
  long long v5; // r12
  char *v6; // rax

  if ( dword_84A54 && (dword_83B80 & 4) != 0 )
  {
    if ( qword_84D50 )
      qword_84D50(a1);
    v4 = sub_4A870(1);
    v5 = sub_4AA70(a1);
    v6 = dcgettext(0, "%s: contains a cache directory tag %s; %s", 5);
    error(0, 0, v6, v5, v4, a3);
  }
}



// Function: cachedir_file_p @ 0x12b70
bool cachedir_file_p(int a1)
{
  ssize_t v1; // r8
  bool result; // al
  long long _0[9]; // [rsp+0h] [rbp+0h] BYREF

  _0[7] = __readfsqword(0x28u);
  v1 = read(a1, _0, 0x2Bu);
  result = 0;
  if ( v1 == 43 )
    return memcmp(_0, "Signature: 8a477f597d28d172789f06886806bc55", 0x2Bu) == 0;
  return result;
}



// Function: off_to_chars @ 0x12be0
void off_to_chars(unsigned long long a1, char *a2, long long a3)
{
  sub_12150(a1 >> 63, a1, 8, 0, a2, a3, (long long)"off_t");
}



// Function: time_to_chars @ 0x12c10
void time_to_chars(unsigned long long a1, char *a2, long long a3)
{
  sub_12150(a1 >> 63, a1, 8, 0, a2, a3, (long long)"time_t");
}



// Function: write_eot @ 0x12c40
char *write_eot()
{
  long long *v0; // r8
  void *v1; // rbp
  char *v2; // rax

  v0 = sub_E130();
  *v0 = 0;
  v0[63] = 0;
  memset(
    (void *)((unsigned long long)(v0 + 1) & 0xFFFFFFFFFFFFFFF8LL),
    0,
    8LL * (((unsigned int)v0 - (((int)v0 + 8) & 0xFFFFFFF8) + 512) >> 3));
  sub_D1F0((unsigned long long)v0);
  v1 = sub_E130();
  v2 = sub_D2B0((long long)v1);
  memset(v1, 0, (size_t)v2);
  return sub_D1F0((unsigned long long)v1);
}



// Function: start_private_header @ 0x12cb0
char *start_private_header(long long a1, unsigned long long a2, long long a3)
{
  char *v4; // r12
  unsigned long long v5; // rdi

  v4 = (char *)sub_E130();
  *(long long *)v4 = 0;
  *((long long *)v4 + 63) = 0;
  memset(
    (void *)((unsigned long long)(v4 + 8) & 0xFFFFFFFFFFFFFFF8LL),
    0,
    8LL * (((unsigned int)v4 - (((int)v4 + 8) & 0xFFFFFFF8) + 512) >> 3));
  sub_12970(v4, a1);
  sub_12BE0(a2, v4 + 124, 12);
  v5 = 0x1FFFFFFFFLL;
  if ( (unsigned long long)a3 <= 0x1FFFFFFFFLL )
    v5 = a3;
  if ( a3 < 0 )
    v5 = 0;
  sub_12C10(v5, v4 + 136, 12);
  sub_129E0(33188, v4 + 100);
  sub_128B0(0, v4 + 108);
  sub_12940(0, v4 + 116);
  strcpy(v4 + 257, "ustar");
  *(short *)(v4 + 263) = 12336;
  return v4;
}



// Function: write_extended @ 0x12da0
long long *write_extended(char a1, long long *a2, long long *a3)
{
  void *v5; // rbp
  __time_t tv_sec; // rdx
  long long v7; // rdi
  long long v8; // rax
  long long v9[71]; // [rsp+0h] [rbp-238h] BYREF

  v9[65] = __readfsqword(0x28u);
  if ( !a2[47] && a2[45] )
  {
    sub_1CC10(a2 + 45);
    qmemcpy(v9, a3, 0x200u);
    if ( a1 )
    {
      v8 = sub_1C4B0();
      tv_sec = stru_84A20.tv_sec;
      v7 = 103;
      v5 = (void *)v8;
    }
    else
    {
      v5 = (void *)sub_1C460(a2);
      if ( dword_84AF0 )
        tv_sec = xmmword_84AE0;
      else
        tv_sec = a2[22];
      v7 = 120;
    }
    sub_1CCF0(v7, v5, tv_sec, a2 + 45);
    free(v5);
    a3 = sub_E130();
    *a3 = v9[0];
    a3[63] = v9[63];
    qmemcpy(
      (void *)((unsigned long long)(a3 + 1) & 0xFFFFFFFFFFFFFFF8LL),
      (const void *)((char *)v9 - ((char *)a3 - ((unsigned long long)(a3 + 1) & 0xFFFFFFFFFFFFFFF8LL))),
      8LL * (((unsigned int)a3 - (((int)a3 + 8) & 0xFFFFFFF8) + 512) >> 3));
  }
  return a3;
}



// Function: simple_finish_header @ 0x12f00
char *simple_finish_header(unsigned long long a1)
{
  int v1; // edx
  unsigned char *v2; // rax
  int v3; // ecx

  v1 = 0;
  *(long long *)(a1 + 148) = 0x2020202020202020LL;
  v2 = (unsigned char *)a1;
  do
  {
    v3 = *v2++;
    v1 += v3;
  }
  while ( (unsigned char *)(a1 + 512) != v2 );
  sub_12A40(v1, a1 + 148);
  return sub_D1F0(a1);
}



// Function: finish_header @ 0x12f60
char *finish_header(long long *a1, long long a2, long long a3)
{
  int v3; // eax
  unsigned long long v4; // rax
  long long v5; // rcx
  long long *v6; // rax

  if ( dword_84A54 )
  {
    v3 = *(unsigned char *)(a2 + 156);
    if ( (char)v3 != 75 )
    {
      v4 = (unsigned int)(v3 - 76);
      if ( (unsigned char)v4 > 0x2Cu || (v5 = 0x100008000001LL, !_bittest64(&v5, v4)) )
      {
        dword_84400 = dword_84BA4;
        sub_22990(a1, a2, a3);
      }
    }
  }
  v6 = sub_12DA0(0, a1, (long long *)a2);
  return sub_12F00((unsigned long long)v6);
}



// Function: write_gnu_long_link @ 0x12fd0
char *write_gnu_long_link(long long *a1, const char *a2, char a3)
{
  const char *v4; // r12
  size_t v6; // rax
  size_t v7; // rbp
  size_t v8; // r13
  char *v9; // r14
  long long v10; // rdi
  char *v11; // r14
  size_t v12; // rbx
  const char *v13; // rsi

  v4 = a2;
  v6 = strlen(a2);
  v7 = v6 + 1;
  v8 = v6;
  v9 = sub_12CB0((long long)"././@LongLink", v6 + 1, 0);
  if ( !byte_84AD2 )
  {
    v10 = *(long long *)&uid;
    if ( !*(long long *)&uid )
    {
      sub_268D0(0);
      sub_26960(0);
      v10 = *(long long *)&uid;
    }
    sub_12890(v10, v9 + 265);
    sub_12890(qword_842A0, v9 + 297);
  }
  v9[156] = a3;
  *(long long *)(v9 + 257) = 0x20207261747375LL;
  sub_12F60(a1, (long long)v9, -1);
  v11 = (char *)sub_E130();
  v12 = (size_t)sub_D2B0((long long)v11);
  if ( v7 > v12 )
  {
    do
    {
      v13 = v4;
      v4 += v12;
      memcpy(v11, v13, v12);
      v7 -= v12;
      sub_D1F0((unsigned long long)&v11[(v12 - 1) & 0xFFFFFFFFFFFFFE00LL]);
      v11 = (char *)sub_E130();
      v12 = (size_t)sub_D2B0((long long)v11);
    }
    while ( v7 > v12 );
    v8 = v7 - 1;
  }
  memcpy(v11, v4, v7);
  memset(&v11[v7], 0, v12 - v7);
  return sub_D1F0((unsigned long long)&v11[v8 & 0xFFFFFFFFFFFFFE00LL]);
}



// Function: write_long_name @ 0x13140
long long write_long_name(long long a1)
{
  long long result; // rax
  const char *v2; // r12
  long long v3; // r12
  char *v4; // rax

  switch ( dword_84BA4 )
  {
    case 1:
      v2 = *(const char **)(a1 + 8);
      if ( strlen(v2) <= 0x63 )
        goto LABEL_4;
      if ( qword_84D50 )
      {
        qword_84D50(v2);
        v2 = *(const char **)(a1 + 8);
      }
      v3 = sub_4AA70(v2);
      v4 = dcgettext(0, "%s: file name is too long (max %d); not dumped", 5);
      error(0, 0, v4, v3, 99);
      dword_84D58 = 2;
      result = 0;
      break;
    case 2:
    case 6:
      sub_12FD0((long long *)a1, *(const char **)(a1 + 8), 76);
LABEL_4:
      result = sub_12990(a1);
      break;
    case 3:
    case 5:
      result = (long long)sub_12440(*(const char **)(a1 + 8));
      break;
    case 4:
      sub_1CA30((char *)"path");
      result = sub_12990(a1);
      break;
    default:
      abort();
  }
  return result;
}



// Function: write_header_name @ 0x13240
long long write_header_name(long long a1)
{
  int v2; // eax
  const char *v3; // rdi

  v2 = dword_84BA4;
  v3 = *(const char **)(a1 + 8);
  if ( dword_84BA4 == 4 )
  {
    if ( !(unsigned char)sub_31440(v3) )
    {
      sub_1CA30((char *)"path");
      return sub_12990(a1);
    }
    v2 = dword_84BA4;
    v3 = *(const char **)(a1 + 8);
  }
  if ( strlen(v3) <= (unsigned long long)(v2 != 2) + 99 )
    return sub_12990(a1);
  else
    return sub_13140(a1);
}



// Function: start_header @ 0x132d0
long long start_header(long long a1)
{
  long long v2; // rax
  long long v3; // r12
  unsigned int v4; // ebp
  long long v5; // rdi
  long long v6; // rsi
  long long v7; // rdi
  long long v8; // rdi
  unsigned long long v9; // rdi
  char v10; // al
  unsigned long long v11; // rdi
  char v12; // al
  unsigned int v13; // ebp
  const char *v14; // rbp
  unsigned long long v15; // rax
  bool v16; // zf
  long long v17; // rbp
  unsigned long long v19; // r13
  unsigned int v20; // r14d
  unsigned int v21; // r13d
  long long v22; // rdi
  __int128 v23; // kr00_16
  const char *v24; // rbp
  void *src; // [rsp+0h] [rbp-68h] BYREF
  void *v26; // [rsp+8h] [rbp-60h] BYREF
  __m128i v27; // [rsp+10h] [rbp-58h]
  unsigned long long v28; // [rsp+28h] [rbp-40h]

  v28 = __readfsqword(0x28u);
  src = 0;
  v26 = 0;
  v2 = sub_13240(a1);
  if ( !v2 )
    return 0;
  v3 = v2;
  sub_239D0(*(unsigned int *)(a1 + 116), a1 + 116, &src);
  sub_23AA0(*(unsigned int *)(a1 + 120), a1 + 120, &v26);
  if ( qword_84B18 )
  {
    v4 = *(int *)(a1 + 112);
    v5 = (unsigned int)sub_43970(v4, (v4 & 0xF000) == 0x4000, (unsigned int)dword_84B14, qword_84B18, 0)
       | v4 & 0xFFFFF000;
    *(int *)(a1 + 112) = v5;
  }
  else
  {
    v5 = *(unsigned int *)(a1 + 112);
  }
  v6 = v3 + 100;
  if ( (dword_84BA4 & 0xFFFFFFFD) == 1 )
    sub_129E0(v5 & 0xFFF, v6);
  else
    sub_129E0(v5, v6);
  v7 = *(unsigned int *)(a1 + 116);
  if ( dword_84BA4 == 4 && (unsigned int)v7 > 0x1FFFFF )
  {
    sub_1CA30("uid");
    v7 = 0;
  }
  if ( !(unsigned char)sub_128B0(v7, v3 + 108) )
    return 0;
  v8 = *(unsigned int *)(a1 + 120);
  if ( dword_84BA4 == 4 && (unsigned int)v8 > 0x1FFFFF )
  {
    sub_1CA30("gid");
    v8 = 0;
  }
  if ( !(unsigned char)sub_12940(v8, v3 + 116) )
    return 0;
  v9 = *(long long *)(a1 + 136);
  if ( dword_84BA4 == 4 && v9 >> 33 )
  {
    sub_1CA30("size");
    v9 = 0;
  }
  sub_12BE0(v9, (char *)(v3 + 124), 12);
  if ( !v10 )
    return 0;
  if ( dword_84AF0 == 1 )
  {
    v27 = _mm_loadu_si128((const __m128i *)&xmmword_84AE0);
  }
  else
  {
    if ( dword_84AF0 != 2 )
    {
      if ( dword_84AF0 )
        goto LABEL_20;
      goto LABEL_19;
    }
    v23 = xmmword_84AE0;
    if ( (int)sub_4BE80(*(long long *)(a1 + 248), *(long long *)(a1 + 256), xmmword_84AE0, *((long long *)&xmmword_84AE0 + 1)) <= 0 )
    {
LABEL_19:
      v27 = _mm_loadu_si128((const __m128i *)(a1 + 248));
      goto LABEL_20;
    }
    v27 = (__m128i)v23;
  }
LABEL_20:
  v11 = v27.m128i_i64[0];
  if ( dword_84BA4 == 4 && ((unsigned long long)v27.m128i_i64[0] >> 33 || v27.m128i_i64[1]) )
  {
    sub_1CA30("mtime");
    v11 = v27.m128i_i64[0];
    if ( (unsigned long long)v27.m128i_i64[0] >> 33 )
    {
      v27.m128i_i64[0] = 0;
      v11 = 0;
    }
  }
  sub_12C10(v11, (char *)(v3 + 136), 12);
  if ( !v12 )
    return 0;
  v13 = dword_84BA4;
  if ( (*(int *)(a1 + 112) & 0xB000) == 0x2000 )
  {
    v19 = *(long long *)(a1 + 128);
    v20 = gnu_dev_major(v19);
    v21 = gnu_dev_minor(v19);
    if ( v20 > 0x1FFFFF && v13 == 4 )
    {
      v20 = 0;
      sub_1CA30("devmajor");
    }
    if ( !(unsigned char)sub_12910(v20, v3 + 329) )
      return 0;
    if ( dword_84BA4 == 4 && v21 > 0x1FFFFF )
    {
      sub_1CA30("devminor");
      v22 = 0;
    }
    else
    {
      v22 = v21;
    }
    if ( !(unsigned char)sub_128E0(v22, v3 + 337) )
      return 0;
  }
  else
  {
    if ( (dword_84BA4 & 0xFFFFFFFB) == 2 )
      goto LABEL_24;
    if ( !(unsigned char)sub_12910(0, v3 + 329) || !(unsigned char)sub_128E0(0, v3 + 337) )
      return 0;
  }
  v13 = dword_84BA4;
LABEL_24:
  if ( v13 == 4 )
  {
    sub_1CA30("atime");
    sub_1CA30("ctime");
    v13 = dword_84BA4;
  }
  else if ( byte_84B50 && (v13 & 0xFFFFFFFB) == 2 )
  {
    sub_12C10(*(long long *)(a1 + 232), (char *)(v3 + 345), 12);
    sub_12C10(*(long long *)(a1 + 264), (char *)(v3 + 357), 12);
    v13 = dword_84BA4;
  }
  if ( v13 == 1 )
  {
    *(char *)(v3 + 156) = 0;
    return v3;
  }
  *(char *)(v3 + 156) = 48;
  if ( v13 > 4 )
  {
    if ( v13 == 6 )
      goto LABEL_68;
    goto LABEL_70;
  }
  if ( v13 <= 2 )
  {
    if ( v13 == 2 )
    {
LABEL_68:
      *(long long *)(v3 + 257) = 0x20207261747375LL;
      goto LABEL_32;
    }
LABEL_70:
    abort();
  }
  strcpy((char *)(v3 + 257), "ustar");
  *(short *)(v3 + 263) = 12336;
LABEL_32:
  if ( !byte_84AD2 )
  {
    if ( src )
      *(long long *)(a1 + 32) = sub_4C7A0(src);
    else
      sub_268D0(*(int *)(a1 + 116));
    if ( v26 )
      *(long long *)(a1 + 40) = sub_4C7A0(v26);
    else
      sub_26960(*(int *)(a1 + 120));
    v14 = *(const char **)(a1 + 32);
    if ( dword_84BA4 == 4 )
    {
      if ( strlen(v14) <= 0x20 && (unsigned char)sub_31440(v14) )
      {
        v14 = *(const char **)(a1 + 32);
      }
      else
      {
        sub_1CA30("uname");
        v14 = *(const char **)(a1 + 32);
      }
    }
    sub_12890(v14, v3 + 265);
    if ( dword_84BA4 == 4 )
    {
      v24 = *(const char **)(a1 + 40);
      if ( strlen(v24) > 0x20 || !(unsigned char)sub_31440(v24) )
        sub_1CA30("gname");
    }
    sub_12890(*(long long *)(a1 + 40), v3 + 297);
    v13 = dword_84BA4;
  }
  if ( v13 == 4 )
  {
    if ( dword_84A9C > 0 )
    {
      if ( *(long long *)(a1 + 56) )
        sub_1CA30("SCHILY.acl.access");
      if ( *(long long *)(a1 + 72) )
        sub_1CA30("SCHILY.acl.default");
    }
    if ( dword_84AA0 > 0 && *(long long *)(a1 + 48) )
      sub_1CA30("RHT.security.selinux");
    if ( dword_84A98 > 0 )
    {
      v15 = 0;
      v16 = *(long long *)(a1 + 344) == 0;
      v27.m128i_i64[0] = 0;
      v17 = *(long long *)(a1 + 352);
      if ( !v16 )
      {
        do
        {
          sub_1CA30(*(char **)(v17 + 24 * v15));
          v15 = v27.m128i_i64[0] + 1;
          v27.m128i_i64[0] = v15;
        }
        while ( v15 < *(long long *)(a1 + 344) );
      }
    }
  }
  return v3;
}



// Function: write_long_link @ 0x13a40
void write_long_link(long long a1)
{
  long long v1; // r12
  char *v2; // rax

  if ( (unsigned int)dword_84BA4 > 6 )
    goto LABEL_10;
  if ( ((1LL << dword_84BA4) & 0x2A) == 0 )
  {
    if ( ((1LL << dword_84BA4) & 0x44) != 0 )
    {
      sub_12FD0((long long *)a1, *(const char **)(a1 + 24), 75);
      return;
    }
    if ( dword_84BA4 == 4 )
    {
      sub_1CA30("linkpath");
      return;
    }
LABEL_10:
    abort();
  }
  if ( qword_84D50 )
    qword_84D50(a1);
  v1 = sub_4AA70(*(long long *)(a1 + 24));
  v2 = dcgettext(0, "%s: link name is too long; not dumped", 5);
  error(0, 0, v2, v1);
  dword_84D58 = 2;
}



// Function: dump_hard_link @ 0x13b00
long long dump_hard_link(void **a1)
{
  long long result; // rax
  long long v2; // rbp
  long long v3; // rdx
  const char *v4; // r12
  long long v5; // rax
  long long v6; // r13
  size_t v7; // rbp
  long long v8; // r8
  long long v9; // r8
  unsigned char v10; // [rsp+Fh] [rbp-59h]
  long long v11[11]; // [rsp+10h] [rbp-58h] BYREF

  v11[5] = __readfsqword(0x28u);
  result = 0;
  if ( qword_842C0 )
  {
    if ( (unsigned long long)a1[13] > qword_842C8 || (result = (unsigned char)byte_84AB8, byte_84AB8) )
    {
      v11[1] = a1[12];
      v11[0] = a1[11];
      v2 = sub_418D0(qword_842C0, v11);
      result = 0;
      if ( v2 )
      {
        v4 = (const char *)sub_33020(v2 + 24, 1, (unsigned char)byte_84B92);
        v5 = *(long long *)(v2 + 16);
        if ( v5 )
          *(long long *)(v2 + 16) = v5 - 1;
        v6 = sub_D180(v2 + 24, 1, v3);
        sub_23BE0(a1 + 3, v4);
        v7 = (dword_84BA4 != 2) + 99LL;
        if ( strlen(v4) > v7 )
          sub_13A40((long long)a1);
        a1[17] = 0;
        v8 = sub_132D0((long long)a1);
        result = 0;
        if ( v8 )
        {
          sub_11D00(v8 + 157, (long long)v4, 100);
          *(char *)(v9 + 156) = 49;
          sub_12F60(a1, v9, v6);
          if ( byte_84AB8 )
          {
            v10 = byte_84AB8;
            sub_30B70(*a1);
            return v10;
          }
          else
          {
            return 1;
          }
        }
      }
    }
  }
  return result;
}



// Function: pad_archive @ 0x13c80
void pad_archive(long long a1)
{
  long long i; // r12
  long long *v2; // rax

  if ( a1 > 0 )
  {
    for ( i = a1 - 512; ; i -= 512 )
    {
      v2 = sub_E130();
      *v2 = 0;
      v2[63] = 0;
      memset(
        (void *)((unsigned long long)(v2 + 1) & 0xFFFFFFFFFFFFFFF8LL),
        0,
        8LL * (((unsigned int)v2 - (((int)v2 + 8) & 0xFFFFFFF8) + 512) >> 3));
      sub_D1F0((unsigned long long)v2);
      if ( i == a1 - 512 - ((a1 - 1) & 0xFFFFFFFFFFFFFE00LL) )
        break;
    }
  }
}



// Function: dump_regular_file @ 0x13d10
long long dump_regular_file(long long fd, long long *a2, long long a3)
{
  long long v3; // r14
  long long v4; // r12
  long long v5; // rax
  long long v6; // rbp
  char *v7; // r15
  size_t v8; // rax
  size_t v9; // r12
  long long v11; // rax
  long long v12; // rbx
  long long v13; // r13
  char *v14; // rax
  unsigned int v15; // eax
  char *v16; // rdx
  long long v17; // [rsp+0h] [rbp-68h]
  char v18[24]; // [rsp+10h] [rbp-58h] BYREF
  unsigned long long v19; // [rsp+28h] [rbp-40h]

  v3 = a2[17];
  v19 = __readfsqword(0x28u);
  v4 = sub_D180(fd, a2, a3);
  v5 = sub_132D0((long long)a2);
  if ( !v5 )
    return 2;
  sub_12F60(a2, v5, v4);
  sub_CC00((void *)a2[1], a2[17], a2[17]);
  do
  {
    while ( 1 )
    {
      if ( v3 <= 0 )
        return 0;
      v6 = v3;
      v7 = (char *)sub_E130();
      v8 = (size_t)sub_D2B0((long long)v7);
      v9 = v8;
      if ( v3 < v8 )
        break;
      if ( (int)fd > 0 )
        goto LABEL_12;
      v3 -= v8;
      sub_D1F0((unsigned long long)&v7[(v8 - 1) & 0xFFFFFFFFFFFFFE00LL]);
    }
    if ( (v3 & 0x1FF) != 0 )
    {
      v15 = 512 - (v3 & 0x1FF);
      v16 = &v7[v3];
      if ( v15 >= 8 )
      {
        *(long long *)v16 = 0;
        *(long long *)&v16[v15 - 8] = 0;
        memset(
          (void *)((unsigned long long)(v16 + 8) & 0xFFFFFFFFFFFFFFF8LL),
          0,
          8LL * ((v15 + (int)v16 - (((int)v16 + 8) & 0xFFFFFFF8)) >> 3));
      }
      else if ( (-(char)v3 & 4) != 0 )
      {
        *(int *)v16 = 0;
        *(int *)&v16[v15 - 4] = 0;
      }
      else if ( v15 )
      {
        *v16 = 0;
        if ( (-(char)v3 & 2) != 0 )
          *(short *)&v16[v15 - 2] = 0;
      }
    }
    if ( (int)fd <= 0 )
    {
      sub_D1F0((unsigned long long)&v7[(v3 - 1) & 0xFFFFFFFFFFFFFE00LL]);
      return 0;
    }
    v9 = v3;
LABEL_12:
    v11 = sub_24960(fd, v7, v9);
    if ( v11 == -1 )
    {
      sub_24E00(*a2, a2[17] - v3, v9);
      sub_13C80(v3);
      return 1;
    }
    v3 -= v11;
    v17 = v11;
    sub_D1F0((unsigned long long)&v7[(v9 - 1) & 0xFFFFFFFFFFFFFE00LL]);
  }
  while ( v17 == v9 );
  memset(&v7[v17], 0, v9 - v17);
  if ( (dword_83B80 & 0x80u) != 0 )
  {
    if ( qword_84D50 )
      qword_84D50(&v7[v17]);
    v12 = sub_43090(v3, v18);
    v13 = sub_4AA70(*a2);
    v14 = dcngettext(
            0,
            "%s: File shrank by %s byte; padding with zeros",
            "%s: File shrank by %s bytes; padding with zeros",
            v3,
            5);
    error(0, 0, v14, v13, v12);
  }
  if ( !byte_84B52 )
    sub_2F7D0(1);
  sub_13C80(v6 - v9);
  return 1;
}



// Function: get_directory_entries @ 0x13fe0
long long get_directory_entries(long long a1)
{
  DIR *v1; // rax

  do
  {
    v1 = fdopendir(*(int *)(a1 + 424));
    *(long long *)(a1 + 416) = v1;
    if ( v1 )
      return sub_4B0D0(v1);
  }
  while ( (unsigned char)sub_125B0(a1) );
  return 0;
}



// Function: check_links @ 0x14030
void check_links()
{
  long long v0; // rdi
  long long i; // rbp
  long long v2; // r12
  char *v3; // rax

  v0 = qword_842C0;
  if ( qword_842C0 )
  {
    for ( i = sub_41930(); i; i = sub_41980(qword_842C0, i) )
    {
      if ( *(long long *)(i + 16) )
      {
        if ( qword_84D50 )
          qword_84D50(v0);
        v2 = sub_4AC50(i + 24);
        v3 = dcgettext(0, "Missing links to %s.", 5);
        error(0, 0, v3, v2);
      }
      v0 = qword_842C0;
    }
  }
}



// Function: subfile_open @ 0x140d0
long long subfile_open(long long a1, long long a2, unsigned int a3)
{
  long long v5; // rdi
  int v6; // r12d

  if ( !byte_84298 )
  {
    byte_84298 = 1;
    strerror(2);
    dcgettext(0, "", 5);
  }
  do
  {
    v5 = (unsigned int)oldfd;
    if ( a1 )
      v5 = *(unsigned int *)(a1 + 424);
    v6 = __openat_2(v5, a2, a3);
  }
  while ( v6 < 0 && (unsigned char)sub_125B0(a1) );
  return (unsigned int)v6;
}



// Function: check_exclusion_tags @ 0x14160
long long check_exclusion_tags(long long a1, long long *a2)
{
  long long v2; // rbx
  int v3; // ebp
  unsigned char (*v4)(long long); // rax

  v2 = qword_842D0;
  if ( !qword_842D0 )
    return 0;
  while ( 1 )
  {
    v3 = sub_140D0(a1, *(long long *)v2, dword_84808);
    if ( v3 < 0 )
      goto LABEL_4;
    v4 = *(unsigned char (**)(long long))(v2 + 24);
    if ( !v4 || v4((unsigned int)v3) )
      break;
    close(v3);
LABEL_4:
    v2 = *(long long *)(v2 + 32);
    if ( !v2 )
      return 0;
  }
  close(v3);
  if ( a2 )
    *a2 = *(long long *)v2;
  return *(unsigned int *)(v2 + 16);
}



// Function: restore_parent_fd @ 0x14200
unsigned long long restore_parent_fd(long long a1)
{
  long long v1; // rbx
  int v2; // eax
  int v3; // ebp
  int v4; // eax
  int v6; // r12d
  struct stat v7; // [rsp+0h] [rbp-B8h] BYREF
  unsigned long long v8; // [rsp+98h] [rbp-20h]

  v1 = *(long long *)(a1 + 408);
  v8 = __readfsqword(0x28u);
  if ( v1 && !*(int *)(v1 + 424) )
  {
    v2 = __openat_2(*(unsigned int *)(a1 + 424), "..", (unsigned int)dword_84804);
    v3 = v2;
    if ( v2 >= 0 )
    {
      if ( fstat(v2, &v7) || *(long long *)(v1 + 96) != v7.st_ino || *(long long *)(v1 + 88) != v7.st_dev )
      {
        close(v3);
        *(int *)(v1 + 424) = 2;
        return v8 - __readfsqword(0x28u);
      }
    }
    else
    {
      v4 = *__errno_location();
      v3 = -v4;
      if ( v4 > 0 )
      {
        v6 = __openat_2((unsigned int)oldfd, *(long long *)v1, (unsigned int)dword_84804);
        if ( v6 >= 0 )
        {
          if ( !fstat(v3, &v7) && *(long long *)(v1 + 96) == v7.st_ino && *(long long *)(v1 + 88) == v7.st_dev )
            v3 = v6;
          else
            close(v6);
        }
      }
    }
    *(int *)(v1 + 424) = v3;
  }
  return v8 - __readfsqword(0x28u);
}



// Function: dump_file @ 0x14330
unsigned long long dump_file(long long a1, long long a2, long long a3, double a4)
{
  long long v6[55]; // [rsp+0h] [rbp-1E8h] BYREF
  unsigned long long v7; // [rsp+1B8h] [rbp-30h]

  v7 = __readfsqword(0x28u);
  sub_2F630(v6);
  v6[51] = a1;
  sub_14CA0(v6, a2, a3);
  if ( a1 && qword_84B28 )
    a4 = sub_1F580(a1);
  sub_2F6F0(v6, a4);
  return v7 - __readfsqword(0x28u);
}



// Function: create_archive @ 0x143d0
long long create_archive(double a1)
{
  long long v1; // rax
  long long v2; // rax
  long long v3; // rbp
  long long v5; // rax
  long long *v6; // r15
  long long v7; // rax
  long long v8; // rbx
  const char *v9; // rbp
  const char *v10; // rbp
  char v11; // r14
  size_t v12; // r12
  int v13; // eax
  size_t n; // [rsp+0h] [rbp-218h]
  size_t v15; // [rsp+18h] [rbp-200h] BYREF
  long long v16[11]; // [rsp+20h] [rbp-1F8h] BYREF
  struct stat buf; // [rsp+78h] [rbp-1A0h] BYREF
  int v18; // [rsp+1C8h] [rbp-50h]
  unsigned long long v19; // [rsp+1D8h] [rbp-40h]

  v19 = __readfsqword(0x28u);
  v1 = 0;
  if ( dword_84630 != 2 )
    v1 = (unsigned char)byte_84B69 ^ 1u;
  qword_842C8 = v1;
  sub_FB70(1, a1);
  sub_CCA0();
  if ( byte_84B50 )
  {
    v15 = 0;
    sub_276A0();
    while ( 1 )
    {
      v5 = sub_27B00();
      v6 = (long long *)v5;
      if ( !v5 )
        break;
      if ( !(unsigned char)sub_16B00(*(long long *)(v5 + 16), 0) )
        sub_14330(0, v6[2], v6[2], a1);
    }
    sub_27B80();
    while ( 1 )
    {
      v7 = sub_27B00();
      v8 = v7;
      if ( !v7 )
        break;
      if ( !(unsigned char)sub_16B00(*(long long *)(v7 + 16), 0) )
      {
        v9 = *(const char **)(v8 + 16);
        n = strlen(v9);
        if ( n >= v15 )
        {
          do
            v6 = (long long *)sub_4C710(v6, &v15);
          while ( v15 <= n );
          v9 = *(const char **)(v8 + 16);
        }
        memcpy(v6, v9, n);
        if ( *((char *)v6 + n - 1) != 47 )
          *((char *)v6 + n++) = 47;
        sub_2F630(v16);
        v10 = (const char *)sub_1FA60(*(long long *)(v8 + 56));
        if ( v10 )
        {
          while ( 1 )
          {
            v11 = *v10;
            if ( !*v10 )
              break;
            v12 = strlen(v10);
            if ( v11 == 89 )
            {
              if ( !v16[0] )
              {
                v13 = __openat_2((unsigned int)oldfd, *(long long *)(v8 + 16), (unsigned int)dword_84804);
                if ( v13 < 0 )
                {
                  sub_24EF0(*(long long *)(v8 + 16), *(long long *)(v8 + 64) == 0, sub_24DD0);
                  break;
                }
                v18 = v13;
                if ( fstat(v13, &buf) )
                {
                  sub_24EF0(*(long long *)(v8 + 16), *(long long *)(v8 + 64) == 0, sub_24EC0);
                  break;
                }
                v16[0] = sub_4C7A0(*(void **)(v8 + 16));
              }
              for ( ; v15 < n + v12; v6 = (long long *)sub_4C710(v6, &v15) )
                ;
              strcpy((char *)v6 + n, v10 + 1);
              sub_14330((long long)v16, (long long)(v10 + 1), (long long)v6, a1);
            }
            v10 += v12 + 1;
          }
        }
        sub_2F6F0(v16, a1);
      }
    }
    free(v6);
  }
  else
  {
    while ( 1 )
    {
      v2 = sub_26CD0(1);
      v3 = v2;
      if ( !v2 )
        break;
      if ( !(unsigned char)sub_16B00(v2, 0) )
        sub_14330(0, v3, v3, a1);
    }
  }
  sub_12C40();
  sub_F900(a1);
  sub_30B30();
  if ( qword_84B28 )
    return sub_20010();
  else
    return v19 - __readfsqword(0x28u);
}



// Function: dump_dir0 @ 0x14710
void dump_dir0(long long *a1, const char *a2, long long a3, double a4)
{
  long long *v4; // r13
  const char *v5; // rbx
  long long v6; // rbp
  long long v7; // rax
  long long v8; // r12
  long long v9; // rax
  long long v10; // r14
  long long v11; // rdx
  long long v12; // r12
  char *v13; // rdx
  long long v14; // rbp
  char *v15; // r12
  long long v16; // r15
  char *v17; // r13
  size_t v18; // rax
  size_t v19; // rbx
  int v20; // eax
  char *v21; // r12
  size_t v22; // r14
  size_t v23; // r15
  size_t v24; // rax
  size_t v25; // rbp
  unsigned int v26; // eax
  char *v27; // rdx
  char *v28; // rdx
  char *v29; // rax
  const char *v30; // r12
  size_t v31; // rbx
  size_t v32; // rax
  char *v33; // rbp
  char *s[9]; // [rsp+10h] [rbp-48h] BYREF

  v4 = a1;
  v5 = a2;
  v6 = a1[51];
  s[1] = (char *)__readfsqword(0x28u);
  v7 = sub_D180(a1, a2, a3);
  a1[17] = 0;
  v8 = v7;
  v9 = sub_132D0((long long)a1);
  if ( !v9 )
    return;
  v10 = v9;
  sub_16890(a1);
  if ( !byte_84B50 )
  {
    *(char *)(v10 + 156) = 53;
    sub_12F60(a1, v10, v8);
    goto LABEL_4;
  }
  a1 = *(long long **)(qword_84620 + 56);
  if ( dword_84BA4 == 4 )
  {
    *(char *)(v10 + 156) = 53;
    if ( a1 )
    {
      ((void (*)(void))sub_1FA90)();
      sub_1CA30("GNU.dumpdir");
      sub_12F60(v4, v10, v8);
      return;
    }
    goto LABEL_4;
  }
  *(char *)(v10 + 156) = 68;
  if ( !a1 )
  {
LABEL_4:
    if ( dword_84AD4 )
    {
      if ( byte_84AD1 && v6 && *(long long *)(v4[51] + 88) != v4[11] )
      {
        if ( dword_84A54 && (dword_83B80 & 0x40000) != 0 )
        {
          if ( qword_84D50 )
            qword_84D50(a1);
          v12 = sub_4AA70(*v4);
          v13 = dcgettext(0, "%s: file is on a different filesystem; not dumped", 5);
          error(0, 0, v13, v12);
        }
      }
      else
      {
        v20 = sub_14160((long long)v4, s);
        switch ( v20 )
        {
          case 1:
            v29 = dcgettext(0, "contents not dumped", 5);
            v30 = s[0];
            sub_12AD0(*v4, (long long)s[0], (long long)v29);
            v31 = strlen((const char *)*v4);
            v32 = strlen(v30);
            v33 = (char *)sub_4C5B0(v31 + v32 + 1);
            strcpy(v33, (const char *)*v4);
            strcat(v33, v30);
            sub_14330((long long)v4, (long long)v30, (long long)v33, a4);
            free(v33);
            break;
          case 2:
            v28 = dcgettext(0, "contents not dumped", 5);
            sub_12AD0(*v4, (long long)s[0], (long long)v28);
            break;
          case 0:
            v21 = (char *)sub_4C7A0((void *)*v4);
            v22 = strlen(v21);
            v23 = v22;
            while ( 1 )
            {
              v24 = strlen(v5);
              v25 = v24;
              if ( !v24 )
                break;
              if ( v22 + v24 > v23 )
              {
                v23 = v22 + v24;
                v21 = (char *)sub_4C610(v21, v22 + v24 + 1);
              }
              strcpy(&v21[v22], v5);
              if ( !(unsigned char)sub_16B00(v21, v4) )
                sub_14330((long long)v4, (long long)v5, (long long)v21, a4);
              v5 += v25 + 1;
            }
            free(v21);
            break;
        }
      }
    }
    return;
  }
  v16 = sub_D180(a1, a2, v11);
  v15 = (char *)sub_1FA90(*(long long *)(qword_84620 + 56));
  v14 = sub_1F4C0(v15);
  sub_12BE0(v14, (char *)(v10 + 124), 12);
  sub_12F60(v4, v10, v16);
  sub_CC00((void *)v4[1], v14, v14);
  if ( v14 > 0 )
  {
    while ( 1 )
    {
      v17 = (char *)sub_E130();
      v18 = (size_t)sub_D2B0((long long)v17);
      v19 = v18;
      if ( v14 < v18 )
        break;
      v14 -= v18;
      memcpy(v17, v15, v18);
      v15 += v19;
      sub_D1F0((unsigned long long)&v17[(v19 - 1) & 0xFFFFFFFFFFFFFE00LL]);
      if ( v14 <= 0 )
        return;
    }
    if ( (v14 & 0x1FF) != 0 )
    {
      v26 = 512 - (v14 & 0x1FF);
      v27 = &v17[v14];
      if ( v26 >= 8 )
      {
        *(long long *)v27 = 0;
        *(long long *)&v27[v26 - 8] = 0;
        memset(
          (void *)((unsigned long long)(v27 + 8) & 0xFFFFFFFFFFFFFFF8LL),
          0,
          8LL * ((v26 + (int)v27 - (((int)v27 + 8) & 0xFFFFFFF8)) >> 3));
      }
      else if ( (-(char)v14 & 4) != 0 )
      {
        *(int *)v27 = 0;
        *(int *)&v27[v26 - 4] = 0;
      }
      else if ( v26 )
      {
        *v27 = 0;
        if ( (-(char)v14 & 2) != 0 )
          *(short *)&v27[v26 - 2] = 0;
      }
    }
    memcpy(v17, v15, v14);
    sub_D1F0((unsigned long long)&v17[(v14 - 1) & 0xFFFFFFFFFFFFFE00LL]);
  }
}



// Function: dump_dir @ 0x14c40
long long dump_dir(long long *a1, double a2)
{
  const char *v2; // rax
  long long v3; // rdx
  char *v4; // r12

  v2 = (const char *)sub_13FE0((long long)a1);
  if ( v2 )
  {
    v4 = (char *)v2;
    sub_14710(a1, v2, v3, a2);
    sub_14200((long long)a1);
    free(v4);
    return 1;
  }
  else
  {
    sub_24E60(*a1);
    return 0;
  }
}



// Function: dump_file0 @ 0x14ca0
void dump_file0(long long a1, char *a2, void *a3)
{
  long long v5; // rbx
  unsigned int v6; // r14d
  const char **v7; // r15
  long long v8; // rax
  int *v9; // rax
  long long (*v10)(); // rdx
  long long v11; // rsi
  long long v12; // rax
  __off_t v13; // rax
  long long v14; // rdx
  long long v15; // rdx
  __m128i v16; // rax
  char v17; // r8
  __m128i v18; // xmm0
  int v19; // eax
  long long v20; // rdi
  long long v21; // rsi
  char v22; // al
  int v23; // ebx
  long long v24; // rax
  int v25; // eax
  int v26; // eax
  const char *v27; // rsi
  long long v28; // r12
  char *v29; // rdx
  long long v30; // rax
  size_t v31; // rbx
  long long v32; // rdx
  long long v33; // rax
  long long v34; // r13
  long long v35; // rax
  long long v36; // r8
  char *v37; // rdx
  int v38; // ebx
  long long v39; // rdx
  unsigned long long v40; // rcx
  long long v41; // rax
  unsigned int v42; // eax
  unsigned int v43; // ebx
  char v44; // r8
  int v45; // eax
  char *v46; // rsi
  long long v47; // rdi
  char v48; // bl
  long long v49; // rdx
  long long v50; // rax
  long long v51; // r14
  long long v52; // rax
  long long v53; // r13
  unsigned int v54; // eax
  unsigned int v55; // eax
  int v56; // r13d
  bool v57; // bl
  long long v58; // rdi
  long long v59; // rdx
  long long v60; // r13
  char *v61; // rax
  unsigned int v62; // eax
  char v63; // [rsp+6h] [rbp-122h]
  bool v64; // [rsp+7h] [rbp-121h]
  __m128i v66; // [rsp+10h] [rbp-118h] BYREF
  __off_t v67; // [rsp+20h] [rbp-108h]
  int fd; // [rsp+28h] [rbp-100h]
  int v69; // [rsp+2Ch] [rbp-FCh]
  __int128 v70; // [rsp+30h] [rbp-F8h]
  long long v71; // [rsp+48h] [rbp-E0h] BYREF
  struct stat buf; // [rsp+50h] [rbp-D8h] BYREF
  unsigned long long v73; // [rsp+E8h] [rbp-40h]

  v5 = *(long long *)(a1 + 408);
  v73 = __readfsqword(0x28u);
  v64 = v5 == 0;
  v63 = byte_84B40;
  if ( v5 )
  {
    v6 = *(int *)(v5 + 424);
    if ( byte_84B40 && !(unsigned int)sub_2CA20("add", a3) )
      return;
    v7 = (const char **)(a1 + 8);
    sub_23BE0(a1, a3);
    v8 = sub_33020(a3, 0, (unsigned char)byte_84B92);
    sub_23BE0(a1 + 8, v8);
    sub_30830(a1 + 8, 1);
    if ( (v6 & 0x80000000) != 0 )
    {
      v9 = __errno_location();
      v10 = sub_24DD0;
      *v9 = -v6;
LABEL_6:
      v11 = v64;
LABEL_7:
      sub_24EF0(a3, v11, v10);
      return;
    }
    v63 = 1;
  }
  else
  {
    v6 = oldfd;
    if ( byte_84B40 )
    {
      if ( !(unsigned int)sub_2CA20("add", a3) )
        return;
      v7 = (const char **)(a1 + 8);
      sub_23BE0(a1, a3);
      v12 = sub_33020(a3, 0, (unsigned char)byte_84B92);
      sub_23BE0(a1 + 8, v12);
      sub_30830(a1 + 8, 1);
      v63 = 0;
    }
    else
    {
      v7 = (const char **)(a1 + 8);
      sub_23BE0(a1, a3);
      v24 = sub_33020(a3, 0, (unsigned char)byte_84B92);
      sub_23BE0(a1 + 8, v24);
      sub_30830(a1 + 8, 1);
    }
  }
  fd = fstatat(v6, a2, (struct stat *)(a1 + 88), flag);
  if ( fd )
    goto LABEL_33;
  if ( sub_11D50(a1 + 88) )
  {
    v25 = sub_140D0(v5, (long long)a2, dword_84808);
    fd = v25;
    if ( v25 < 0 )
    {
      v10 = sub_24DD0;
      goto LABEL_6;
    }
    *(int *)(a1 + 424) = v25;
    if ( fstat(v25, (struct stat *)(a1 + 88)) )
    {
LABEL_33:
      v10 = (long long (*)())sub_24EC0;
      goto LABEL_6;
    }
  }
  v13 = *(long long *)(a1 + 136);
  *(long long *)(a1 + 280) = v13;
  v67 = v13;
  *(long long *)(a1 + 232) = sub_4BD00(a1 + 88);
  *(long long *)(a1 + 240) = v14;
  *(long long *)(a1 + 248) = sub_4BD20(a1 + 88);
  *(long long *)(a1 + 256) = v15;
  v16.m128i_i64[0] = sub_4BD10(a1 + 88);
  v17 = byte_84B50;
  v66 = v16;
  v18 = _mm_load_si128(&v66);
  *(__m128i *)(a1 + 264) = v18;
  if ( v17 != 1 || !v5 )
  {
    v19 = *(int *)(a1 + 112);
    LOBYTE(v69) = v17;
    if ( (v19 & 0xF000) != 0x4000 )
    {
      v20 = *(long long *)(a1 + 248);
      v21 = *(long long *)(a1 + 256);
      v70 = xmmword_84B00;
      if ( (int)sub_4BE80(v20, v21, xmmword_84B00, *((long long *)&xmmword_84B00 + 1)) < 0 )
      {
        v44 = v69;
        if ( !dword_84B8C
          || (v20 = *(long long *)(a1 + 264),
              v45 = sub_4BE80(v20, *(long long *)(a1 + 272), v70, *((long long *)&v70 + 1)),
              v44 = v69,
              v45 < 0) )
        {
          if ( !v44 && dword_84A54 && (dword_83B80 & 0x100) != 0 )
          {
            if ( qword_84D50 )
              qword_84D50(v20);
            v27 = "%s: file is unchanged; not dumped";
            v28 = sub_4AA70(a3);
LABEL_45:
            v29 = dcgettext(0, v27, 5);
            error(0, 0, v29, v28);
            return;
          }
          return;
        }
      }
    }
  }
  if ( (unsigned char)sub_2AB40(a1) )
  {
    if ( (dword_83B80 & 0x400) != 0 )
    {
      if ( qword_84D50 )
        qword_84D50(a1);
      v27 = "%s: file is the archive; not dumped";
      v28 = sub_4AA70(a3);
      goto LABEL_45;
    }
    return;
  }
  v69 = *(int *)(a1 + 112) & 0xF000;
  if ( v69 == 0x4000 )
  {
    sub_319F0(v6, a2, a1, 0, 0);
    sub_31DE0(v6, a2, a1, (unsigned int)fd);
    sub_31B80(v6, a2);
    sub_126E0((const char **)a1);
    sub_126E0(v7);
    if ( (unsigned int)sub_14160(a1, &v71) == 3 )
    {
      v37 = dcgettext(0, "directory not dumped", 5);
      sub_12AD0(*(long long *)a1, v71, (long long)v37);
      return;
    }
    v22 = sub_14C40((long long *)a1, *(double *)v18.m128i_i64);
    v6 = oldfd;
    fd = *(int *)(a1 + 424);
    if ( v5 )
      v6 = *(int *)(v5 + 424);
    if ( v22 )
    {
      v23 = fd;
      if ( fd < 0 )
      {
        *__errno_location() = -v23;
LABEL_28:
        sub_24EF0(a3, v64, sub_24EC0);
        goto LABEL_29;
      }
LABEL_65:
      if ( fd )
      {
        if ( fstat(fd, &buf) )
          goto LABEL_28;
      }
      else
      {
        if ( (v6 & 0x80000000) != 0 && v63 )
        {
          *__errno_location() = -v6;
          goto LABEL_28;
        }
        if ( fstatat(v6, a2, &buf, flag) )
          goto LABEL_28;
      }
      v56 = v69;
      v57 = v69 == 0x4000;
      v58 = sub_4BD10(&buf);
      if ( (!(unsigned int)sub_4BE80(v58, v59, v66.m128i_i64[0], v66.m128i_i64[1]) || byte_84AB8 == 1 && v56 == 0x4000)
        && buf.st_size <= v67 )
      {
        if ( dword_84B88 == 1
          && fd
          && (v67 || v57)
          && (unsigned int)sub_24A80((unsigned int)fd, v6, a2, *(long long *)(a1 + 232), *(long long *)(a1 + 240)) )
        {
          sub_32DD0(a3);
        }
      }
      else
      {
        if ( (dword_83B80 & 0x10) != 0 )
        {
          if ( qword_84D50 )
            qword_84D50(v58);
          v60 = sub_4AA70(a3);
          v61 = dcgettext(0, "%s: file changed as we read it", 5);
          error(0, 0, v61, v60);
        }
        sub_2F7D0(1);
      }
      if ( (unsigned char)sub_2F670(a1) )
      {
        if ( byte_84AB8 )
          sub_30B70(a3);
      }
      return;
    }
LABEL_29:
    sub_2F670(a1);
    return;
  }
  if ( (unsigned char)sub_13B00((void **)a1) )
    return;
  v26 = *(int *)(a1 + 112) & 0xF000;
  switch ( v26 )
  {
    case 32768:
      sub_319F0(v6, a2, a1, 0, 1);
      v38 = fd;
      sub_31DE0(v6, a2, a1, (unsigned int)fd);
      sub_31B80(v6, a2);
      if ( !v38 || !byte_84A88 )
        goto LABEL_63;
      v39 = *(long long *)(a1 + 136);
      v40 = v39 + 511;
      v41 = v39 / 512;
      if ( (v39 & 0x1FF) != 0 )
      {
        v39 = v41 + 1;
        if ( v40 > 0x3FE )
          ++v41;
      }
      if ( *(long long *)(a1 + 152) < v41 && (v62 = sub_29CD0((unsigned int)fd, a1), v43 = v62, v62 != 3) )
      {
        if ( v62 > 1 )
          goto LABEL_29;
      }
      else
      {
LABEL_63:
        v42 = sub_13D10((unsigned int)fd, (long long *)a1, v39);
        v43 = v42;
        if ( v42 > 1 )
        {
          if ( v42 == 3 )
            abort();
          goto LABEL_29;
        }
      }
      sub_12760((long long *)a1);
      if ( !v43 )
        goto LABEL_65;
      goto LABEL_29;
    case 40960:
      v30 = sub_39200(v6, a2);
      *(long long *)(a1 + 24) = v30;
      if ( !v30 )
      {
        if ( *__errno_location() == 12 )
          sub_16420(v6);
        v11 = v64;
        v10 = sub_24E30;
        goto LABEL_7;
      }
      sub_30830(a1 + 24, 4);
      v31 = (dword_84BA4 != 2) + 99LL;
      if ( strlen(*(const char **)(a1 + 24)) > v31 )
        sub_13A40(a1);
      sub_31DE0(v6, a2, a1, 0);
      sub_31B80(v6, a2);
      v33 = sub_D180(v6, a2, v32);
      *(long long *)(a1 + 136) = 0;
      v34 = v33;
      v35 = sub_132D0(a1);
      if ( v35 )
      {
        sub_11D00(v35 + 157, *(long long *)(a1 + 24), 100);
        *(char *)(v36 + 156) = 50;
        sub_12F60((long long *)a1, v36, v34);
        if ( byte_84AB8 )
          sub_30B70(a3);
        sub_12760((long long *)a1);
      }
      return;
    case 8192:
      sub_319F0(v6, a2, a1, 0, 1);
      sub_31DE0(v6, a2, a1, 0);
      v46 = a2;
      v47 = v6;
      v48 = 51;
      sub_31B80(v6, a2);
      goto LABEL_82;
    case 24576:
      sub_319F0(v6, a2, a1, 0, 1);
      sub_31DE0(v6, a2, a1, 0);
      v46 = a2;
      v47 = v6;
      v48 = 52;
      sub_31B80(v6, a2);
      goto LABEL_82;
    case 4096:
      sub_319F0(v6, a2, a1, 0, 1);
      sub_31DE0(v6, a2, a1, 0);
      v46 = a2;
      v47 = v6;
      v48 = 54;
      sub_31B80(v6, a2);
LABEL_82:
      if ( dword_84BA4 != 1 )
      {
        v50 = sub_D180(v47, v46, v49);
        *(long long *)(a1 + 136) = 0;
        v51 = v50;
        v52 = sub_132D0(a1);
        v53 = v52;
        if ( v52 )
        {
          *(char *)(v52 + 156) = v48;
          if ( v48 != 54 )
          {
            v54 = gnu_dev_major(*(long long *)(a1 + 128));
            sub_12910(v54, v53 + 329);
            v55 = gnu_dev_minor(*(long long *)(a1 + 128));
            sub_128E0(v55, v53 + 337);
          }
          sub_12F60((long long *)a1, v53, v51);
          if ( byte_84AB8 )
            sub_30B70(a3);
        }
        return;
      }
LABEL_80:
      sub_12640((long long)a3);
      return;
  }
  if ( v26 != 49152 )
    goto LABEL_80;
  if ( (dword_83B80 & 0x20) != 0 )
  {
    if ( qword_84D50 )
      qword_84D50(a1);
    v27 = "%s: socket ignored";
    v28 = sub_4AA70(a3);
    goto LABEL_45;
  }
}



// Function: flush_file @ 0x15860
signed long long flush_file(long long a1, long long a2)
{
  long long v2; // rdi
  char *v3; // rdx
  __off_t v4; // rbx
  signed long long result; // rax
  signed long long i; // rbx

  v2 = qword_84408;
  sub_D1F0(qword_84408);
  v3 = (char *)src;
  v4 = buf.st_size + 1022;
  if ( buf.st_size + 511 >= 0 )
    v4 = buf.st_size + 511;
  result = (char *)qword_83F50 - (char *)src;
  for ( i = v4 >> 9; ((char *)qword_83F50 - (char *)src) >> 9 <= i; result = (char *)qword_83F50 - (char *)src )
  {
    i -= result >> 9;
    sub_DFD0(v2, a2, v3);
    v3 = (char *)src;
  }
  src = &v3[512 * i];
  return result;
}



// Function: move_archive @ 0x158f0
long long move_archive(long long a1)
{
  int v2; // eax
  short v3; // cx
  bool v4; // dl
  int v5; // edi
  int *v6; // rax
  __off_t v7; // rax
  signed long long v8; // rbx
  __int128 v9; // rt2
  unsigned long long v10; // kr00_8
  long long v11; // rdx
  __off_t v12; // rsi
  long long v14; // rax
  short v15[2]; // [rsp+10h] [rbp-28h] BYREF
  int v16; // [rsp+14h] [rbp-24h]
  unsigned long long v17; // [rsp+18h] [rbp-20h]

  v17 = __readfsqword(0x28u);
  if ( !a1 )
    return v17 - __readfsqword(0x28u);
  v2 = a1;
  if ( a1 < 0 )
  {
    v2 = -(int)a1;
    v3 = 4;
    v4 = a1 + -(int)a1 == 0;
  }
  else
  {
    v3 = 3;
    v4 = (int)a1 == a1;
  }
  v15[0] = v3;
  v5 = fd;
  v16 = v2;
  if ( v4 )
  {
    if ( fd > 0x3FFFFFFF )
    {
      if ( (int)sub_34080((unsigned int)(fd - 0x40000000), 1074294017, v15) >= 0 )
        return v17 - __readfsqword(0x28u);
    }
    else if ( ioctl(fd, 0x40086D01u, v15) >= 0 )
    {
      return v17 - __readfsqword(0x28u);
    }
    v6 = __errno_location();
    v5 = fd;
    if ( *v6 == 5 )
    {
      if ( fd > 0x3FFFFFFF )
      {
        if ( (int)sub_34080((unsigned int)(fd - 0x40000000), 1074294017, v15) >= 0 )
          return v17 - __readfsqword(0x28u);
      }
      else if ( ioctl(fd, 0x40086D01u, v15) >= 0 )
      {
        return v17 - __readfsqword(0x28u);
      }
      v5 = fd;
    }
  }
  if ( v5 <= 0x3FFFFFFF )
    v7 = lseek(v5, 0, 1);
  else
    v7 = sub_33F90((unsigned int)(v5 - 0x40000000), 0, 1);
  v10 = a1;
  v9 = (long long)nbytes * (__int128)a1;
  v8 = nbytes * a1;
  v11 = *((long long *)&v9 + 1) != 0;
  v12 = v7 + v8;
  if ( !is_mul_ok(nbytes, v10) || v7 > v12 != v8 < 0 )
    return sub_32B20(*(long long *)ptr, v12);
  if ( v12 < 0 )
    v12 = 0;
  v14 = fd > 0x3FFFFFFF ? sub_33F90((unsigned int)(fd - 0x40000000), v12, v11) : lseek(fd, v12, 0);
  if ( v14 != v12 )
    return sub_32B20(*(long long *)ptr, v12);
  return v17 - __readfsqword(0x28u);
}



// Function: write_record @ 0x15b10
long long write_record(int a1)
{
  void *v1; // rbp
  long long result; // rax

  v1 = s;
  s = (void *)qword_842E8;
  if ( byte_842E0 )
  {
    fd = 1;
    result = sub_FB50();
    fd = 0;
  }
  else
  {
    sub_158F0(qword_83F30 + qword_842D8 - qword_83F38);
    result = sub_FB50();
  }
  s = v1;
  if ( a1 && !byte_842E0 )
    result = sub_158F0(qword_83F38 - (qword_83F30 + qword_842D8));
  dword_842E4 = 0;
  return result;
}



// Function: write_recent_blocks @ 0x15bc0
long long write_recent_blocks(long long a1, long long a2)
{
  long long i; // rbx
  char *v4; // rsi
  long long result; // rax
  long long *v6; // rcx
  long long v7; // rdx
  bool v8; // zf

  if ( a2 )
  {
    for ( i = 0; i != a2; ++i )
    {
      while ( 1 )
      {
        v4 = (char *)(a1 + (i << 9));
        v6 = (long long *)(qword_842E8 + ((long long)dword_842E4 << 9));
        v7 = *(long long *)v4;
        result = (unsigned int)++dword_842E4;
        *v6 = v7;
        v6[63] = *((long long *)v4 + 63);
        v8 = (int)result == dword_84BA0;
        qmemcpy(
          (void *)((unsigned long long)(v6 + 1) & 0xFFFFFFFFFFFFFFF8LL),
          (const void *)(v4 - ((char *)v6 - ((unsigned long long)(v6 + 1) & 0xFFFFFFFFFFFFFFF8LL))),
          8LL * (((unsigned int)v6 - (((int)v6 + 8) & 0xFFFFFFF8) + 512) >> 3));
        if ( v8 )
          break;
        if ( a2 == ++i )
          return result;
      }
      result = sub_15B10(1);
    }
  }
  return result;
}



// Function: write_recent_bytes @ 0x15c70
long long *write_recent_bytes(long long a1, unsigned long long a2)
{
  long long v2; // r12
  char *v3; // rsi
  int v4; // edx
  long long *v5; // rax
  long long *result; // rax
  unsigned int v7; // ecx
  void *v8; // rdi
  unsigned int v9; // ecx

  v2 = a2 & 0x1FF;
  sub_15BC0(a1, a2 >> 9);
  v3 = (char *)(a1 + (a2 & 0xFFFFFFFFFFFFFE00LL));
  v4 = dword_842E4;
  v5 = (long long *)(qword_842E8 + ((long long)dword_842E4 << 9));
  if ( (unsigned int)v2 >= 8 )
  {
    *v5 = *(long long *)v3;
    *(long long *)((char *)v5 + v2 - 8) = *(long long *)&v3[v2 - 8];
    qmemcpy(
      (void *)((unsigned long long)(v5 + 1) & 0xFFFFFFFFFFFFFFF8LL),
      (const void *)(v3 - ((char *)v5 - ((unsigned long long)(v5 + 1) & 0xFFFFFFFFFFFFFFF8LL))),
      8LL * (((unsigned int)v2 + (int)v5 - (((int)v5 + 8) & 0xFFFFFFF8)) >> 3));
  }
  else if ( (v2 & 4) != 0 )
  {
    *(int *)v5 = *(int *)v3;
    *(int *)((char *)v5 + v2 - 4) = *(int *)&v3[v2 - 4];
  }
  else if ( (int)v2 )
  {
    *(char *)v5 = *v3;
    if ( (v2 & 2) != 0 )
      *(short *)((char *)v5 + v2 - 2) = *(short *)&v3[v2 - 2];
  }
  result = (long long *)((char *)v5 + v2);
  v7 = 512 - v2;
  if ( (unsigned int)(512 - v2) >= 8 )
  {
    *result = 0;
    *(long long *)((char *)result + v7 - 8) = 0;
    v8 = (void *)((unsigned long long)(result + 1) & 0xFFFFFFFFFFFFFFF8LL);
    v9 = (int)result - (int)v8 + v7;
    result = 0;
    memset(v8, 0, 8LL * (v9 >> 3));
  }
  else if ( (-(char)v2 & 4) != 0 )
  {
    *(int *)result = 0;
    *(int *)((char *)result + v7 - 4) = 0;
  }
  else if ( v7 )
  {
    *(char *)result = 0;
    if ( (-(char)v2 & 2) != 0 )
      *(short *)((char *)result + v7 - 2) = 0;
  }
  dword_842E4 = v4 + 1;
  if ( v4 + 1 == dword_84BA0 )
    return (long long *)sub_15B10(1);
  return result;
}



// Function: fatal_exit @ 0x163d0
void fatal_exit()
{
  char *v0; // rax

  if ( qword_842F0 )
    qword_842F0();
  v0 = dcgettext(0, "Error is not recoverable: exiting now", 5);
  error(2, 0, v0);
  abort();
}



// Function: xalloc_die @ 0x16420
void xalloc_die()
{
  char *v0; // rax

  v0 = dcgettext(0, "memory exhausted", 5);
  error(0, 0, "%s", v0);
  sub_163D0();
}



// Function: hg_initfn @ 0x16460
int *hg_initfn(int *a1)
{
  int *result; // rax

  result = a1;
  if ( !a1 )
    result = &unk_842F8;
  *result = 0x8000000;
  return result;
}



// Function: get_vcs_ignore_file @ 0x16480
char **get_vcs_ignore_file(char *s2)
{
  char **v1; // r12
  const char *i; // rdi

  v1 = &off_81160;
  for ( i = off_81160; i; v1 += 5 )
  {
    if ( !strcmp(i, s2) )
      break;
    i = v1[5];
  }
  return v1;
}



// Function: bzr_addfn @ 0x164d0
const unsigned short **bzr_addfn(long long a1, const char *a2, unsigned int a3)
{
  const unsigned short **result; // rax
  long long v6; // rdx

  result = __ctype_b_loc();
  while ( 1 )
  {
    v6 = *a2;
    if ( ((*result)[v6] & 0x2000) == 0 )
      break;
    ++a2;
  }
  if ( (char)v6 && (char)v6 != 35 )
  {
    if ( (char)v6 == 33 )
    {
      if ( a2[1] == 33 )
      {
        a2 += 2;
      }
      else
      {
        ++a2;
        a3 |= 0x20000000u;
      }
    }
    if ( !strncmp(a2, "RE:", 3u) )
    {
      a2 += 3;
      a3 = a3 & 0xE7FFFFFF | 0x8000000;
    }
    return (const unsigned short **)sub_3EE30(a1, a2, a3);
  }
  return result;
}



// Function: git_addfn @ 0x16580
const unsigned short **git_addfn(long long a1, char *a2, unsigned int a3)
{
  const unsigned short **result; // rax
  long long v6; // rcx

  result = __ctype_b_loc();
  while ( 1 )
  {
    v6 = *a2;
    if ( ((*result)[v6] & 0x2000) == 0 )
      break;
    ++a2;
  }
  if ( (char)v6 && (char)v6 != 35 )
  {
    if ( (char)v6 == 92 )
      a2 += a2[1] == 35;
    return (const unsigned short **)sub_3EE30(a1, a2, a3);
  }
  return result;
}



// Function: hg_addfn @ 0x16600
int hg_addfn(long long a1, const char *a2, int a3, int *a4)
{
  const unsigned short **v7; // rax
  const unsigned short *v8; // r14
  long long v9; // rdx
  size_t v10; // rax
  size_t v11; // r14
  int v12; // eax
  unsigned int v13; // ebx
  unsigned int v14; // eax
  const char *v15; // r13
  long long v16; // rax
  char *v17; // r15
  size_t n; // [rsp+8h] [rbp-40h]

  v7 = __ctype_b_loc();
  v8 = *v7;
  while ( 1 )
  {
    v9 = *a2;
    if ( (v8[v9] & 0x2000) == 0 )
      break;
    ++a2;
  }
  if ( (char)v9 && (char)v9 != 35 )
  {
    if ( !strncmp(a2, "syntax:", 7u) )
    {
      v15 = a2 + 7;
      if ( (v8[a2[7]] & 0x2000) != 0 )
      {
        do
          v16 = *++v15;
        while ( (v8[v16] & 0x2000) != 0 );
      }
      LODWORD(v7) = strcmp(v15, "regexp");
      if ( (int)v7 )
      {
        LODWORD(v7) = strcmp(v15, "glob");
        if ( !(int)v7 )
          *a4 = 0x10000000;
      }
      else
      {
        *a4 = 0x8000000;
      }
    }
    else
    {
      v10 = strlen(a2);
      v11 = v10;
      if ( a2[v10 - 1] == 47 )
      {
        n = v10 - 1;
        a3 |= 0x4000008u;
        v17 = (char *)sub_4C5B0(v10);
        memcpy(v17, a2, n);
        a2 = v17;
        v17[v11 - 1] = 0;
        sub_3EAA0(a1, v17);
      }
      v12 = a3;
      v13 = a3 & 0xF7FFFFFF;
      v14 = v12 & 0xEFFFFFFF;
      if ( *a4 == 0x8000000 )
        v13 = v14;
      LODWORD(v7) = sub_3EE30(a1, a2, v13 | *a4);
    }
  }
  return (int)v7;
}



// Function: cvs_addfn @ 0x16780
unsigned long long cvs_addfn(long long a1, long long a2, unsigned int a3)
{
  unsigned long long v4; // rbx
  long long v5; // rsi
  long long v7[29]; // [rsp+0h] [rbp-118h] BYREF
  unsigned long long v8; // [rsp+E8h] [rbp-30h]

  v8 = __readfsqword(0x28u);
  if ( !(unsigned int)sub_382C0(a2, v7, 2116) )
  {
    if ( v7[0] )
    {
      v4 = 0;
      do
      {
        v5 = *(long long *)(v7[1] + 8 * v4++);
        sub_3EE30(a1, v5, a3);
      }
      while ( v7[0] > v4 );
    }
    sub_35420(v7);
  }
  return v8 - __readfsqword(0x28u);
}



// Function: excfile_add @ 0x16820
long long excfile_add(char *src, int a2)
{
  size_t v2; // rax
  long long v3; // rax
  long long v4; // rbx
  long long result; // rax

  v2 = strlen(src);
  v3 = sub_4C5B0(v2 + 16);
  *(long long *)v3 = 0;
  v4 = v3;
  *(int *)(v3 + 8) = a2;
  strcpy((char *)(v3 + 12), src);
  result = qword_84300;
  if ( qword_84300 )
    *(long long *)qword_84300 = v4;
  else
    qword_84308 = v4;
  qword_84300 = v4;
  return result;
}



// Function: info_attach_exclist @ 0x16890
long long info_attach_exclist(long long a1)
{
  long long result; // rax
  long long v2; // rbx
  long long *v4; // r15
  unsigned int v5; // eax
  int v6; // r13d
  long long v7; // rdi
  FILE *v8; // r14
  char **v9; // r13
  char *v10; // rax
  long long v11; // rax
  char *v12; // r9
  char *v13; // rdi
  long long v14; // rax
  int v15; // edx
  char *v16; // r14
  int *v17; // rax
  int v18; // r12d
  const char *v19; // rax
  long long v20; // [rsp+0h] [rbp-48h]
  long long v21; // [rsp+8h] [rbp-40h]

  result = *(long long *)(a1 + 432);
  v21 = result;
  if ( !result )
  {
    v2 = qword_84308;
    if ( qword_84308 )
    {
      v4 = 0;
      do
      {
        while ( faccessat(*(int *)(a1 + 424), (const char *)(v2 + 12), 0, 0) )
        {
LABEL_4:
          v2 = *(long long *)v2;
          if ( !v2 )
            goto LABEL_15;
        }
        v5 = sub_140D0(a1, v2 + 12, 0);
        v6 = v5;
        if ( v5 == -1 )
        {
          sub_327C0(v2 + 12);
          goto LABEL_4;
        }
        v7 = v5;
        v8 = fdopen(v5, "r");
        if ( !v8 )
        {
          if ( qword_84D50 )
            qword_84D50(v7);
          v16 = dcgettext(0, "%s: fdopen failed", 5);
          v17 = __errno_location();
          error(0, *v17, v16, v2 + 12);
          dword_84D58 = 2;
          close(v6);
          goto LABEL_4;
        }
        v20 = sub_3EBA0();
        v9 = sub_16480((char *)(v2 + 12));
        v10 = v9[3];
        if ( v10 )
        {
          v11 = ((long long (*)(char *))v10)(v9[4]);
          v9[4] = (char *)v11;
          v12 = (char *)v11;
        }
        else
        {
          v12 = v9[4];
        }
        v13 = v9[2];
        if ( (unsigned int)sub_3F0C0(v13, v20, v8, 1342177281, 10, v12) )
        {
          v18 = *__errno_location();
          if ( qword_84D50 )
            qword_84D50(v13);
          v19 = (const char *)sub_4AA70(v2 + 12);
          error(0, v18, "%s", v19);
          sub_163D0();
        }
        fclose(v8);
        v14 = sub_4C5B0(32);
        v15 = *(int *)(v2 + 8);
        *(long long *)(v14 + 24) = v20;
        if ( v15 )
          v15 = *((int *)v9 + 2);
        *(int *)(v14 + 16) = v15;
        *(long long *)(v14 + 8) = v4;
        *(long long *)v14 = 0;
        if ( !v4 )
        {
          v21 = v14;
          v4 = (long long *)v14;
          goto LABEL_4;
        }
        v2 = *(long long *)v2;
        *v4 = v14;
        v4 = (long long *)v14;
      }
      while ( v2 );
    }
LABEL_15:
    *(long long *)(a1 + 432) = v21;
    return v21;
  }
  return result;
}



// Function: info_free_exclist @ 0x16ab0
void info_free_exclist(long long a1)
{
  void **v1; // rbx
  void **v2; // rbp

  v1 = *(void ***)(a1 + 432);
  while ( v1 )
  {
    v2 = v1;
    v1 = (void **)*v1;
    sub_3EBB0(v2[3]);
    free(v2);
  }
  *(long long *)(a1 + 432) = 0;
}



// Function: excluded_name @ 0x16b00
long long excluded_name(char *a1, long long a2)
{
  long long v2; // r14
  int v3; // r12d
  void *v4; // rbp
  char *j; // r15
  long long **i; // rbx
  unsigned char v8; // [rsp+Fh] [rbp-39h]

  v2 = a2;
  v8 = sub_3ED70(qword_84B60, a1);
  if ( !v8 && a2 )
  {
    v3 = 0;
    v4 = 0;
    j = 0;
    do
    {
      for ( i = *(long long ***)(v2 + 432); i; i = (long long **)*i )
      {
        if ( ((int)i[2] & v3) == 0 )
        {
          if ( (unsigned char)sub_3ED70(i[3], a1) )
            goto LABEL_14;
          if ( !j )
          {
            for ( j = a1; *j == 46; j += 2 )
            {
              if ( j[1] != 47 )
                break;
            }
          }
          if ( (unsigned char)sub_3ED70(i[3], j) )
          {
LABEL_14:
            v8 = 1;
            goto LABEL_15;
          }
          if ( v4 )
          {
            if ( (unsigned char)sub_3ED70(i[3], v4) )
              goto LABEL_14;
          }
          else
          {
            v4 = (void *)sub_3E510(a1);
            if ( (unsigned char)sub_3ED70(i[3], v4) )
              goto LABEL_14;
          }
        }
      }
      v2 = *(long long *)(v2 + 408);
      v3 = 2;
    }
    while ( v2 );
LABEL_15:
    free(v4);
  }
  return v8;
}



// Function: exclude_vcs_ignores @ 0x16c40
long long exclude_vcs_ignores()
{
  char *v0; // rdi
  char **v1; // rbx
  long long result; // rax

  v0 = off_81160;
  if ( off_81160 )
  {
    v1 = &off_81160;
    do
    {
      v1 += 5;
      result = sub_16820(v0, 0);
      v0 = *v1;
    }
    while ( *v1 );
  }
  return result;
}



// Function: implemented @ 0x16c80
long long implemented(int a1, long long a2, int a3)
{
  unsigned int v3; // eax

  LOBYTE(v3) = a1 != 38;
  LOBYTE(a3) = a1 != 95;
  return a3 & v3;
}



// Function: free_delayed_set_stat @ 0x16c90
void free_delayed_set_stat(void **ptr)
{
  free(ptr[20]);
  sub_1C600(ptr[18]);
  free(ptr[12]);
  free(ptr[13]);
  free(ptr[15]);
  free(ptr);
}



// Function: is_directory_link @ 0x16ce0
bool is_directory_link(char *file)
{
  int *v1; // rax
  int v2; // r13d
  int *v3; // rbx
  bool result; // al
  struct stat v5; // [rsp+0h] [rbp-C8h] BYREF
  unsigned long long v6; // [rsp+98h] [rbp-30h]

  v6 = __readfsqword(0x28u);
  v1 = __errno_location();
  v2 = *v1;
  v3 = v1;
  result = !fstatat(oldfd, file, &v5, 256)
        && (v5.st_mode & 0xF000) == 0xA000
        && !fstatat(oldfd, file, &v5, 0)
        && (v5.st_mode & 0xF000) == 0x4000;
  *v3 = v2;
  return result;
}



// Function: find_delayed_link_source @ 0x16da0
long long *find_delayed_link_source(char *file)
{
  long long *v1; // r12
  struct stat v3; // [rsp+0h] [rbp-B8h] BYREF
  unsigned long long v4; // [rsp+98h] [rbp-20h]

  v1 = qword_84318;
  v4 = __readfsqword(0x28u);
  if ( !qword_84318 )
    return 0;
  if ( fstatat(oldfd, file, &v3, 256) )
  {
    if ( *__errno_location() != 2 )
    {
      v1 = 0;
      sub_32D30(file);
      return v1;
    }
    return 0;
  }
  while ( v1[1] != v3.st_dev || v1[2] != v3.st_ino )
  {
    v1 = (long long *)*v1;
    if ( !v1 )
      return 0;
  }
  return v1;
}



// Function: repair_delayed_set_stat @ 0x16e60
unsigned long long repair_delayed_set_stat(long long a1, long long *a2)
{
  long long v2; // r13
  const char *v3; // rbx
  __m128i v4; // xmm0
  __m128i v5; // xmm1
  __gid_t st_gid; // eax
  __mode_t st_mode; // eax
  long long v8; // r12
  char *v9; // rax
  struct stat v11; // [rsp+0h] [rbp-C8h] BYREF
  unsigned long long v12; // [rsp+98h] [rbp-30h]

  v2 = a1;
  v3 = (const char *)qword_84320;
  v12 = __readfsqword(0x28u);
  if ( qword_84320 )
  {
    while ( 1 )
    {
      a1 = (unsigned int)oldfd;
      if ( fstatat(oldfd, *((const char **)v3 + 20), &v11, *((int *)v3 + 21)) )
        break;
      if ( v11.st_dev == *a2 && v11.st_ino == a2[1] )
      {
        v4 = _mm_loadu_si128((const __m128i *)&xmmword_84928);
        v5 = _mm_loadu_si128((const __m128i *)&file);
        *((long long *)v3 + 1) = buf.st_dev;
        *((long long *)v3 + 2) = buf.st_ino;
        *((int *)v3 + 6) = buf.st_mode;
        *((int *)v3 + 7) = buf.st_uid;
        st_gid = buf.st_gid;
        *((int *)v3 + 19) = -1;
        *((int *)v3 + 8) = st_gid;
        st_mode = v11.st_mode;
        *((char *)v3 + 80) = 0;
        *((int *)v3 + 18) = st_mode;
        *(__m128i *)(v3 + 40) = v4;
        *(__m128i *)(v3 + 56) = v5;
        return v12 - __readfsqword(0x28u);
      }
      v3 = *(const char **)v3;
      if ( !v3 )
        goto LABEL_8;
    }
    sub_32D30(*((long long *)v3 + 20));
  }
  else
  {
LABEL_8:
    if ( qword_84D50 )
      qword_84D50(a1);
    v8 = sub_4AA70(v2);
    v9 = dcgettext(0, "%s: Unexpected inconsistency when making directory", 5);
    error(0, 0, v9, v8);
    dword_84D58 = 2;
  }
  return v12 - __readfsqword(0x28u);
}



// Function: check_time @ 0x16fd0
void check_time(long long a1, long long a2, long long a3)
{
  long long v5; // rbp
  char *v6; // rdx
  __time_t tv_sec; // rdi
  long long v8; // r15
  __syscall_slong_t v9; // r14
  long long v10; // r14
  long long v11; // rbp
  char *v12; // rax
  struct timespec v13; // [rsp+0h] [rbp-68h] BYREF
  char v14[40]; // [rsp+10h] [rbp-58h] BYREF
  unsigned long long v15; // [rsp+38h] [rbp-30h]

  v15 = __readfsqword(0x28u);
  if ( a2 < 0 )
  {
    if ( (dword_83B80 & 0x8000) != 0 )
    {
      if ( qword_84D50 )
        qword_84D50(a1);
      v5 = sub_22080(a2, a3, 1);
      v6 = dcgettext(0, "%s: implausibly old time stamp %s", 5);
      error(0, 0, v6, a1, v5);
    }
  }
  else if ( (int)sub_4BE80(tp.tv_sec, tp.tv_nsec, a2, a3) < 0 )
  {
    sub_411A0(&v13);
    tv_sec = v13.tv_sec;
    if ( (int)sub_4BE80(v13.tv_sec, v13.tv_nsec, a2, a3) < 0 )
    {
      v8 = a2 - v13.tv_sec;
      v9 = a3 - v13.tv_nsec;
      if ( a3 - v13.tv_nsec < 0 )
      {
        v9 += 1000000000LL;
        --v8;
      }
      if ( (dword_83B80 & 0x8000) != 0 )
      {
        if ( qword_84D50 )
          qword_84D50(tv_sec);
        v10 = sub_24270(v8, v9, v14);
        v11 = sub_22080(a2, a3, 1);
        v12 = dcgettext(0, "%s: time stamp %s is %s s in the future", 5);
        error(0, 0, v12, a1, v11, v10);
      }
    }
  }
}



// Function: fd_chown @ 0x17180
int fd_chown(int a1, const char *a2, __uid_t a3, __gid_t a4, int a5)
{
  int v8; // r14d
  int *v9; // rax
  int v10; // edx

  if ( a1 >= 0
    && ((v8 = fchown(a1, a3, a4)) == 0 || (v9 = __errno_location(), (unsigned char)sub_16C80(*v9, a3, v10))) )
  {
    return v8;
  }
  else
  {
    return fchownat(oldfd, a2, a3, a4, a5);
  }
}



// Function: fd_stat @ 0x171f0
int fd_stat(int a1, const char *a2, struct stat *a3, int a4)
{
  if ( a1 < 0 )
    return fstatat(oldfd, a2, a3, a4);
  else
    return fstat(a1, a3);
}



// Function: fd_i_chmod @ 0x17210
int fd_i_chmod(int a1, const char *a2, __mode_t a3, int a4)
{
  long long v7; // rsi
  int v8; // r14d
  int *v9; // rax
  int v10; // edx

  if ( a1 >= 0
    && ((v7 = a3, (v8 = fchmod(a1, a3)) == 0) || (v9 = __errno_location(), (unsigned char)sub_16C80(*v9, v7, v10))) )
  {
    return v8;
  }
  else
  {
    return fchmodat(oldfd, a2, a3, a4);
  }
}



// Function: fd_chmod @ 0x17280
long long fd_chmod(long long a1, const char *a2, __mode_t a3, int a4, int a5)
{
  int v7; // ebp
  int v9; // eax
  int v10; // edx
  bool v11; // r15
  int *v13; // rax
  long long v14; // rcx
  long long v15; // rdx
  int *v16; // rax
  int v17; // edx
  int *v18; // r8
  int v19; // edi
  char v20; // al
  char v21; // al
  int *v22; // rax
  int v23; // [rsp+0h] [rbp-48h]
  int v24; // [rsp+0h] [rbp-48h]
  int *v25; // [rsp+0h] [rbp-48h]
  int v26; // [rsp+0h] [rbp-48h]
  int v27; // [rsp+Ch] [rbp-3Ch]

  v7 = a1;
  v9 = sub_17210(a1, a2, a3, a4);
  LOBYTE(v10) = a4 != 0;
  v11 = a4 != 0 && a5 != 50;
  if ( !v9 )
  {
LABEL_2:
    if ( !v11 || (unsigned char)sub_16C80(0, (long long)a2, v10) )
      return 0;
    goto LABEL_12;
  }
  v13 = __errno_location();
  v15 = (unsigned int)*v13;
  if ( (int)v15 != 1 )
    goto LABEL_5;
  if ( (a3 & 0x800) == 0 )
  {
    if ( !v11 )
      goto LABEL_7;
LABEL_19:
    v24 = v15;
    v21 = sub_16C80(1, (long long)a2, v15);
    v17 = v24;
    if ( v21 )
      goto LABEL_10;
    goto LABEL_12;
  }
  v27 = *v13;
  v25 = v13;
  if ( (unsigned int)sub_48750(a1, a2, v15, v14, v13) )
  {
    LODWORD(v15) = v27;
    if ( !v11 )
      goto LABEL_7;
    goto LABEL_19;
  }
  if ( !sub_17210(a1, a2, a3, a4) )
    goto LABEL_2;
  LODWORD(v15) = *v25;
LABEL_5:
  if ( !v11 )
    goto LABEL_6;
  v19 = v15;
  v20 = sub_16C80(v15, (long long)a2, v15);
  v17 = v19;
  if ( v20 )
  {
    if ( v19 )
      goto LABEL_10;
    return 0;
  }
LABEL_12:
  if ( !sub_17210(v7, a2, a3, 0) )
    return 0;
  LODWORD(v15) = *__errno_location();
LABEL_6:
  if ( !(int)v15 )
    return 0;
LABEL_7:
  if ( a5 != 50 )
  {
    v26 = v15;
    v22 = __errno_location();
    v17 = v26;
    v18 = v22;
    goto LABEL_10;
  }
  v23 = v15;
  if ( !(unsigned char)sub_16C80(v15, (long long)a2, v15) )
    return 0;
  v16 = __errno_location();
  v17 = v23;
  v18 = v16;
LABEL_10:
  *v18 = v17;
  return 0xFFFFFFFFLL;
}



// Function: set_mode @ 0x17410
long long set_mode(const char *a1, int a2, int a3, unsigned int a4, int a5, int a6, char a7, int a8)
{
  int v10; // r8d
  int v11; // ebx
  int v13; // eax
  unsigned int v14; // ebp
  struct stat v15; // [rsp+0h] [rbp-D8h] BYREF
  unsigned long long v16; // [rsp+98h] [rbp-40h]

  v16 = __readfsqword(0x28u);
  if ( (a3 & (~a6 | a2 ^ a5)) == 0 )
    return v16 - __readfsqword(0x28u);
  if ( (~((unsigned short)a3 | (unsigned short)a6) & 0xFFF) != 0 )
  {
    v13 = sub_171F0(a4, a1, &v15, a8);
    LOWORD(a5) = v15.st_mode;
    if ( v13 )
    {
      sub_32D30(a1);
      return v16 - __readfsqword(0x28u);
    }
  }
  v10 = a5 & 0xFFF;
  v11 = a3 & (v10 ^ a2);
  if ( !v11 )
    return v16 - __readfsqword(0x28u);
  v14 = v11 ^ v10;
  if ( !(unsigned int)sub_17280(a4, a1, v11 ^ (unsigned int)v10, a8, a7) )
    return v16 - __readfsqword(0x28u);
  return sub_32550(a1, v14);
}



// Function: set_stat @ 0x17530
long long set_stat(const char *a1, long long a2, unsigned int a3, int a4, int a5, char a6, char a7, int a8)
{
  unsigned int v10; // r13d
  char v13; // r10
  int v14; // r8d
  int v15; // edx
  long long v17; // rsi
  int v18; // eax
  int v19; // eax
  int *v20; // rax
  int v21; // edx
  char v22; // al
  __uid_t v23; // r10d
  __gid_t v24; // r11d
  int *v25; // rax
  int v26; // edx
  char v27; // al
  char v28; // [rsp+0h] [rbp-78h]
  int v29; // [rsp+0h] [rbp-78h]
  char v30; // [rsp+0h] [rbp-78h]
  __uid_t v31; // [rsp+0h] [rbp-78h]
  int v32; // [rsp+0h] [rbp-78h]
  char v33; // [rsp+4h] [rbp-74h]
  char v34; // [rsp+4h] [rbp-74h]
  __uid_t v35; // [rsp+4h] [rbp-74h]
  char v36; // [rsp+4h] [rbp-74h]
  __gid_t v37; // [rsp+4h] [rbp-74h]
  int v38; // [rsp+8h] [rbp-70h]
  __gid_t v39; // [rsp+8h] [rbp-70h]
  int v40; // [rsp+8h] [rbp-70h]
  int v41; // [rsp+8h] [rbp-70h]
  char v42; // [rsp+Ch] [rbp-6Ch]
  __m128i v43; // [rsp+10h] [rbp-68h] BYREF
  __m128i v44; // [rsp+20h] [rbp-58h]
  unsigned long long v45; // [rsp+38h] [rbp-40h]

  v10 = a6;
  v45 = __readfsqword(0x28u);
  v13 = a7;
  v14 = a8;
  if ( byte_84A68 )
  {
    if ( dword_84AA8 > 0 )
    {
      if ( !a7 )
        goto LABEL_11;
LABEL_19:
      v15 = 511;
      goto LABEL_3;
    }
LABEL_20:
    if ( dword_84AA4 > 0 && !v13 )
    {
      v15 = 4095;
      goto LABEL_3;
    }
    goto LABEL_19;
  }
  v15 = 511;
  if ( a7 )
    goto LABEL_3;
  if ( byte_84B50 )
    v43 = _mm_loadu_si128((const __m128i *)(a2 + 232));
  else
    v43.m128i_i64[1] = 1073741822;
  v17 = (unsigned int)oldfd;
  v33 = a6;
  v44 = _mm_loadu_si128((const __m128i *)(a2 + 248));
  v18 = sub_3F360(a3, (unsigned int)oldfd, a1, &v43);
  v14 = a8;
  a6 = v33;
  v13 = a7;
  if ( v18 )
  {
    if ( v33 != 50 || (v20 = __errno_location(), v22 = sub_16C80(*v20, v17, v21), a6 = 50, v13 = a7, v14 = a8, v22) )
    {
      v40 = v14;
      v36 = v13;
      v30 = a6;
      sub_32DD0(a1);
      v14 = v40;
      v13 = v36;
      a6 = v30;
    }
  }
  else
  {
    if ( byte_84B50 )
    {
      sub_16FD0((long long)a1, v43.m128i_i64[0], v43.m128i_i64[1]);
      v14 = a8;
      v13 = a7;
      a6 = v33;
    }
    v38 = v14;
    v34 = v13;
    v28 = a6;
    sub_16FD0((long long)a1, v44.m128i_i64[0], v44.m128i_i64[1]);
    a6 = v28;
    v13 = v34;
    v14 = v38;
  }
  if ( dword_84AA8 <= 0 )
    goto LABEL_20;
LABEL_11:
  v29 = v14;
  v42 = a6;
  v39 = *(int *)(a2 + 120);
  v35 = *(int *)(a2 + 116);
  v19 = sub_17180(a3, a1, v35, v39, v14);
  v14 = v29;
  if ( v19 )
  {
    v23 = v35;
    v24 = v39;
    if ( v42 != 50 )
      goto LABEL_29;
    v41 = v29;
    v37 = v24;
    v31 = v23;
    v25 = __errno_location();
    v27 = sub_16C80(*v25, (long long)a1, v26);
    v23 = v31;
    v24 = v37;
    v14 = v41;
    if ( v27 )
    {
LABEL_29:
      v32 = v14;
      sub_32600(a1, v23, v24);
      v14 = v32;
    }
  }
  else if ( (((unsigned char)a4 | (unsigned char)~(char)a5) & 0x49) != 0 )
  {
    a5 &= ~(a4 & 0xC00);
  }
  v15 = 4095;
  if ( dword_84AA4 <= 0 )
    v15 = 511;
LABEL_3:
  sub_17410(a1, *(int *)(a2 + 112) & ~dword_84328, v15, a3, a4, a5, v10, v14);
  sub_31F20(a2, a1, v10, 1);
  sub_31A70(a2, a1, v10);
  return sub_31E70(a2, a1, v10);
}



// Function: apply_nonancestor_delayed_set_stat @ 0x17870
unsigned long long apply_nonancestor_delayed_set_stat(const char *s1, unsigned char a2)
{
  size_t v3; // rax
  unsigned int *v4; // r15
  char v5; // bl
  __m128i v6; // xmm0
  __m128i v7; // xmm1
  const char *v8; // rdi
  unsigned int v9; // eax
  long long v10; // rax
  unsigned char v11; // al
  __mode_t st_mode; // r12d
  int v13; // r13d
  size_t v14; // rdx
  char v15; // al
  long long v16; // rdi
  long long v17; // r12
  char *v18; // rax
  size_t v20; // [rsp+8h] [rbp-290h]
  struct stat buf; // [rsp+10h] [rbp-288h] BYREF
  char v22[48]; // [rsp+A0h] [rbp-1F8h] BYREF
  long long v23; // [rsp+D0h] [rbp-1C8h]
  long long v24; // [rsp+D8h] [rbp-1C0h]
  long long v25; // [rsp+E0h] [rbp-1B8h]
  long long v26; // [rsp+E8h] [rbp-1B0h]
  long long v27; // [rsp+F0h] [rbp-1A8h]
  unsigned int v28; // [rsp+110h] [rbp-188h]
  unsigned int v29; // [rsp+114h] [rbp-184h]
  unsigned int v30; // [rsp+118h] [rbp-180h]
  __m128i v31; // [rsp+188h] [rbp-110h]
  __m128i v32; // [rsp+198h] [rbp-100h]
  long long v33; // [rsp+1F8h] [rbp-A0h]
  long long v34; // [rsp+200h] [rbp-98h]
  unsigned long long v35; // [rsp+258h] [rbp-40h]

  v35 = __readfsqword(0x28u);
  v3 = strlen(s1);
  v4 = (unsigned int *)qword_84320;
  v20 = v3;
  if ( qword_84320 )
  {
    v5 = 0;
    do
    {
      v11 = *((char *)v4 + 88);
      st_mode = v4[18];
      v13 = v4[19];
      v5 |= v11;
      if ( v11 > a2 )
        return v35 - __readfsqword(0x28u);
      v14 = *((long long *)v4 + 19);
      if ( v14 < v20 )
      {
        v15 = s1[v14];
        if ( v15 )
        {
          if ( (v15 == 47 || s1[v14 - 1] == 47) && !memcmp(s1, *((const void **)v4 + 20), v14) )
            return v35 - __readfsqword(0x28u);
        }
      }
      sub_24C80(v4[23]);
      if ( v5 )
      {
        v16 = (unsigned int)oldfd;
        if ( fstatat(oldfd, *((const char **)v4 + 20), &buf, v4[21]) )
        {
          sub_32D30(*((long long *)v4 + 20));
          goto LABEL_4;
        }
        st_mode = buf.st_mode;
        if ( buf.st_dev != *((long long *)v4 + 1) || buf.st_ino != *((long long *)v4 + 2) )
        {
          if ( qword_84D50 )
            qword_84D50(v16);
          v17 = sub_4AA70(*((long long *)v4 + 20));
          v18 = dcgettext(0, "%s: Directory renamed before its status could be extracted", 5);
          error(0, 0, v18, v17);
          dword_84D58 = 2;
          goto LABEL_4;
        }
        v13 = -1;
      }
      v6 = _mm_loadu_si128((const __m128i *)(v4 + 10));
      v7 = _mm_loadu_si128((const __m128i *)(v4 + 14));
      v8 = (const char *)*((long long *)v4 + 20);
      v28 = v4[6];
      v29 = v4[7];
      v9 = v4[8];
      v31 = v6;
      v30 = v9;
      v10 = *((long long *)v4 + 12);
      v32 = v7;
      v23 = v10;
      v24 = *((long long *)v4 + 13);
      v25 = *((long long *)v4 + 14);
      v26 = *((long long *)v4 + 15);
      v27 = *((long long *)v4 + 16);
      v34 = *((long long *)v4 + 18);
      v33 = *((long long *)v4 + 17);
      sub_17530(v8, (long long)v22, 0xFFFFFFFF, st_mode, v13, 53, *((char *)v4 + 80), v4[21]);
LABEL_4:
      qword_84320 = *(void **)v4;
      sub_16C90((void **)v4);
      v4 = (unsigned int *)qword_84320;
    }
    while ( qword_84320 );
  }
  return v35 - __readfsqword(0x28u);
}



// Function: mark_after_links @ 0x17b10
unsigned long long mark_after_links(long long *a1)
{
  struct stat vars0; // [rsp+0h] [rbp+0h] BYREF
  unsigned long long vars98; // [rsp+98h] [rbp+98h]

  vars98 = __readfsqword(0x28u);
  do
  {
    *((char *)a1 + 88) = 1;
    if ( !(unsigned int)sub_246F0((char *)a1[20], &vars0) )
    {
      a1[1] = vars0.st_dev;
      a1[2] = vars0.st_ino;
      a1 = (long long *)*a1;
      if ( !a1 )
        return vars98 - __readfsqword(0x28u);
      continue;
    }
    sub_32D30(a1[20]);
    a1 = (long long *)*a1;
    if ( !a1 )
      break;
  }
  while ( !*((char *)a1 + 88) );
  return vars98 - __readfsqword(0x28u);
}



// Function: fixup_delayed_set_stat @ 0x17bb0
void fixup_delayed_set_stat(char *s2, void *src)
{
  void *v2; // rbx
  int v3; // ebp
  char *v4; // r12

  v2 = qword_84320;
  if ( qword_84320 )
  {
    v3 = dword_84580;
    while ( 1 )
    {
      if ( *((int *)v2 + 23) == v3 )
      {
        v4 = (char *)*((long long *)v2 + 20);
        if ( !strcmp(v4, s2) )
          break;
      }
      v2 = *(void **)v2;
      if ( !v2 )
        return;
    }
    free(v4);
    *((long long *)v2 + 20) = sub_4C7A0(src);
    *((long long *)v2 + 19) = strlen((const char *)src);
  }
}



// Function: delay_set_stat @ 0x17c30
unsigned long long delay_set_stat(char *src, long long a2, int a3, int a4, int a5, int a6)
{
  size_t v10; // rax
  long long *v11; // r15
  const char *v12; // r13
  long long v13; // rsi
  __m128i v14; // xmm0
  __m128i v15; // xmm1
  int v16; // eax
  int v17; // eax
  void *v18; // rdi
  long long v19; // rax
  void *v20; // rdi
  long long v21; // rax
  int v23; // eax
  void *v24; // rax
  long long v25; // rax
  size_t v28; // [rsp+8h] [rbp-E0h]
  struct stat buf; // [rsp+10h] [rbp-D8h] BYREF
  unsigned long long v30; // [rsp+A8h] [rbp-40h]

  v30 = __readfsqword(0x28u);
  v10 = strlen(src);
  v11 = qword_84320;
  v28 = v10;
  if ( !qword_84320 )
  {
LABEL_20:
    v11 = (long long *)sub_4C5B0(168);
    v24 = qword_84320;
    qword_84320 = v11;
    *v11 = v24;
    v11[19] = v28;
    v25 = sub_4C7A0(src);
    *((char *)v11 + 88) = 0;
    v11[20] = v25;
    if ( a2 )
    {
      v11[1] = *(long long *)(a2 + 88);
      v11[2] = *(long long *)(a2 + 96);
      *((int *)v11 + 6) = a5;
      goto LABEL_7;
    }
    *((int *)v11 + 6) = a5;
LABEL_15:
    *((int *)v11 + 18) = a3;
    *((char *)v11 + 80) = 1;
    *((int *)v11 + 19) = a4;
    v23 = dword_84580;
    *((int *)v11 + 21) = a6;
    *((int *)v11 + 23) = v23;
    v11[12] = 0;
    v11[13] = 0;
    v11[14] = 0;
    v11[15] = 0;
    v11[16] = 0;
    v11[18] = 0;
    v11[17] = 0;
    if ( !(unsigned char)sub_244E0(src) )
      return v30 - __readfsqword(0x28u);
    return sub_17B10(v11);
  }
  while ( 1 )
  {
    v12 = (const char *)v11[20];
    if ( !strcmp(v12, src) )
      break;
    v11 = (long long *)*v11;
    if ( !v11 )
      goto LABEL_20;
  }
  if ( *((char *)v11 + 80) )
  {
    if ( fstatat(oldfd, v12, &buf, *((int *)v11 + 21)) )
    {
      sub_32D30(v11[20]);
    }
    else
    {
      v11[1] = buf.st_dev;
      v11[2] = buf.st_ino;
    }
  }
  *((int *)v11 + 6) = a5;
  if ( !a2 )
    goto LABEL_15;
LABEL_7:
  v13 = *(long long *)(a2 + 48);
  v14 = _mm_loadu_si128((const __m128i *)(a2 + 232));
  v15 = _mm_loadu_si128((const __m128i *)(a2 + 248));
  *((int *)v11 + 7) = *(int *)(a2 + 116);
  v16 = *(int *)(a2 + 120);
  *((int *)v11 + 18) = a3;
  *((int *)v11 + 8) = v16;
  *((char *)v11 + 80) = 0;
  *((int *)v11 + 19) = a4;
  v17 = dword_84580;
  *((int *)v11 + 21) = a6;
  *((int *)v11 + 23) = v17;
  v11[12] = 0;
  *(__m128i *)(v11 + 5) = v14;
  *(__m128i *)(v11 + 7) = v15;
  sub_23BE0(v11 + 12, v13);
  v18 = *(void **)(a2 + 56);
  if ( v18 )
  {
    v19 = sub_4C770(v18, *(long long *)(a2 + 64) + 1LL);
    v20 = *(void **)(a2 + 72);
    v11[13] = v19;
    v11[14] = *(long long *)(a2 + 64);
    if ( v20 )
    {
LABEL_9:
      v11[15] = sub_4C770(v20, *(long long *)(a2 + 80) + 1LL);
      v21 = *(long long *)(a2 + 80);
      goto LABEL_10;
    }
  }
  else
  {
    v20 = *(void **)(a2 + 72);
    v11[13] = 0;
    v11[14] = 0;
    if ( v20 )
      goto LABEL_9;
  }
  v11[15] = 0;
  v21 = 0;
LABEL_10:
  v11[16] = v21;
  sub_1C840(a2, v11 + 18, v11 + 17);
  if ( !(unsigned char)sub_244E0(src) )
    return v30 - __readfsqword(0x28u);
  return sub_17B10(v11);
}



// Function: make_directories @ 0x17f70
long long make_directories(char *src, char *a2)
{
  char *v2; // rbx
  char v3; // al
  char v4; // al
  short v5; // r12
  unsigned int v6; // r12d
  __mode_t v7; // r15d
  int *v8; // rax
  int v9; // r13d
  int *v10; // r12
  unsigned int v11; // r15d
  char v13; // al
  struct stat buf; // [rsp+10h] [rbp-D8h] BYREF
  unsigned long long v15; // [rsp+A8h] [rbp-40h]

  v2 = src;
  v15 = __readfsqword(0x28u);
  if ( *src )
  {
LABEL_2:
    while ( 1 )
    {
      v3 = *++v2;
      if ( !v3 )
        break;
      while ( src != v2 )
      {
        if ( v3 != 47 )
          break;
        v4 = *(v2 - 1);
        if ( v4 == 47 )
          break;
        if ( v4 == 46 )
        {
          if ( v2 == src + 1 )
            break;
          v13 = *(v2 - 2);
          if ( v13 == 47 || v13 == 46 && (v2 == src + 2 || *(v2 - 3) == 47) )
            break;
        }
        v5 = dword_8432C;
        *v2 = 0;
        v6 = ~v5 & 0x1FF;
        v7 = v6;
        if ( !byte_84330 )
          LOBYTE(v7) = v6 | 0xC0;
        if ( !mkdirat(oldfd, src, v7) )
        {
          sub_17C30(src, 0, v7 & ~dword_84328, 511, v6, 256);
          sub_22A00(src, (unsigned int)((int)v2 - (int)src), v6);
          *a2 = 1;
          *v2 = 47;
          goto LABEL_2;
        }
        v8 = __errno_location();
        v9 = *v8;
        v10 = v8;
        if ( *v8 != 17 )
        {
          v11 = fstatat(oldfd, src, &buf, 0);
          if ( v11 )
          {
            *v10 = v9;
            sub_32760(src);
            *v2 = 47;
            return v11;
          }
        }
        *v2 = 47;
        v3 = *++v2;
        if ( !v3 )
          return 0;
      }
    }
  }
  return 0;
}



// Function: open_output_file @ 0x18140
long long open_output_file(char *file, int a2, unsigned int a3, int a4, __mode_t *a5, int *a6)
{
  int v7; // r12d
  int v10; // r15d
  int v11; // edx
  int v12; // eax
  unsigned int v13; // r12d
  char *v15; // rax
  int *v16; // rax
  int v17; // ebp
  int *v18; // rbx
  struct stat v19; // [rsp+0h] [rbp-D8h] BYREF
  unsigned long long v20; // [rsp+98h] [rbp-40h]

  v7 = 526785;
  v10 = dword_84B34;
  v20 = __readfsqword(0x28u);
  if ( dword_84B34 == 2 )
    v7 = byte_84B69 == 0 ? 658241 : 527169;
  v11 = v7;
  if ( a4 )
  {
    LOBYTE(v11) = v7 & 0x7F;
    v7 = v11;
  }
  if ( a2 == 55 && !dword_84310 )
  {
    dword_84310 = 1;
    if ( (dword_83B80 & 8) != 0 )
    {
      if ( qword_84D50 )
        qword_84D50(file);
      v15 = dcgettext(0, "Extracting contiguous files as regular files", 5);
      error(0, 0, v15);
    }
  }
  v12 = openat(oldfd, file, v7, a3);
  v13 = v12;
  if ( v12 >= 0 )
  {
    if ( v10 == 2 )
    {
      if ( fstat(v12, &v19) )
      {
        v16 = __errno_location();
        v17 = *v16;
        v18 = v16;
        close(v13);
        *v18 = v17;
      }
      else
      {
        if ( (v19.st_mode & 0xF000) == 0x8000 )
        {
          *a5 = v19.st_mode;
          *a6 = -1;
          return v13;
        }
        close(v13);
        *__errno_location() = 17;
      }
      return (unsigned int)-1;
    }
    *a5 = a3 & ~dword_84328;
    *a6 = 511;
  }
  return v13;
}



// Function: find_direct_ancestor @ 0x182e0
void *find_direct_ancestor(char *s1)
{
  void *i; // r12
  size_t v2; // rbx

  for ( i = qword_84320; i; i = *(void **)i )
  {
    if ( !*((char *)i + 88) )
    {
      v2 = *((long long *)i + 19);
      if ( !strncmp(s1, *((const char **)i + 20), v2) && s1[v2] == 47 && (char *)sub_3E390(s1) == &s1[v2 + 1] )
        break;
    }
  }
  return i;
}



// Function: apply_delayed_links @ 0x18350
unsigned long long apply_delayed_links()
{
  unsigned int *v0; // r12
  const char *v1; // r14
  const char *v2; // rbx
  long long v3; // rax
  long long v4; // rdx
  int v5; // edi
  int v6; // eax
  long long *v7; // rbx
  void *v8; // rdi
  unsigned int *v9; // rbx
  __m128i v11; // xmm0
  __m128i v12; // xmm1
  unsigned int v13; // eax
  long long v14; // rax
  struct stat v15; // [rsp+0h] [rbp-288h] BYREF
  char v16[48]; // [rsp+90h] [rbp-1F8h] BYREF
  long long v17; // [rsp+C0h] [rbp-1C8h]
  long long v18; // [rsp+C8h] [rbp-1C0h]
  long long v19; // [rsp+D0h] [rbp-1B8h]
  long long v20; // [rsp+D8h] [rbp-1B0h]
  long long v21; // [rsp+E0h] [rbp-1A8h]
  unsigned int v22; // [rsp+100h] [rbp-188h]
  unsigned int v23; // [rsp+104h] [rbp-184h]
  unsigned int v24; // [rsp+108h] [rbp-180h]
  __m128i v25; // [rsp+178h] [rbp-110h]
  __m128i v26; // [rsp+188h] [rbp-100h]
  long long v27; // [rsp+1E8h] [rbp-A0h]
  long long v28; // [rsp+1F0h] [rbp-98h]
  unsigned long long v29; // [rsp+248h] [rbp-40h]

  v0 = (unsigned int *)qword_84318;
  v29 = __readfsqword(0x28u);
  if ( !qword_84318 )
    goto LABEL_19;
  while ( 2 )
  {
    v1 = 0;
    sub_24C80(v0[22]);
    v2 = (const char *)*((long long *)v0 + 12);
    if ( !v2 )
      goto LABEL_17;
    do
    {
      while ( 1 )
      {
        if ( fstatat(oldfd, v2 + 8, &v15, 256) )
          goto LABEL_4;
        if ( v15.st_dev != *((long long *)v0 + 1) )
          goto LABEL_4;
        if ( v15.st_ino != *((long long *)v0 + 2) )
          goto LABEL_4;
        v3 = sub_4BD30(&v15);
        if ( (unsigned int)sub_4BE80(v3, v4, *((long long *)v0 + 3), *((long long *)v0 + 4)) )
          goto LABEL_4;
        if ( unlinkat(oldfd, v2 + 8, 0) )
        {
          sub_32DB0(v2 + 8);
          goto LABEL_4;
        }
        v5 = oldfd;
        if ( v1 )
        {
          v6 = linkat(oldfd, v1, oldfd, v2 + 8, 0);
          v5 = oldfd;
          if ( !v6 )
            goto LABEL_4;
        }
        if ( *((char *)v0 + 40) )
        {
          if ( symlinkat((const char *)v0 + 160, v5, v2 + 8) )
          {
            sub_32C90(v0 + 40, v2 + 8);
          }
          else
          {
            v11 = _mm_loadu_si128((const __m128i *)(v0 + 14));
            v12 = _mm_loadu_si128((const __m128i *)(v0 + 18));
            v1 = v2 + 8;
            v22 = v0[11];
            v23 = v0[12];
            v13 = v0[13];
            v25 = v11;
            v24 = v13;
            v14 = *((long long *)v0 + 13);
            v26 = v12;
            v17 = v14;
            v18 = *((long long *)v0 + 14);
            v19 = *((long long *)v0 + 15);
            v20 = *((long long *)v0 + 16);
            v21 = *((long long *)v0 + 17);
            v28 = *((long long *)v0 + 19);
            v27 = *((long long *)v0 + 18);
            sub_17530(v2 + 8, (long long)v16, 0xFFFFFFFF, 0, 0, 50, 0, 256);
          }
          goto LABEL_4;
        }
        if ( linkat(v5, (const char *)v0 + 160, v5, v2 + 8, 0) )
          break;
LABEL_4:
        v2 = *(const char **)v2;
        if ( !v2 )
          goto LABEL_15;
      }
      sub_326E0(v0 + 40, v2 + 8);
      v2 = *(const char **)v2;
    }
    while ( v2 );
LABEL_15:
    v7 = (long long *)*((long long *)v0 + 12);
    while ( v7 )
    {
      v8 = v7;
      v7 = (long long *)*v7;
      free(v8);
    }
LABEL_17:
    sub_1C600(*((void **)v0 + 19));
    free(*((void **)v0 + 13));
    v9 = *(unsigned int **)v0;
    free(v0);
    if ( v9 )
    {
      v0 = v9;
      continue;
    }
    break;
  }
LABEL_19:
  qword_84318 = 0;
  return v29 - __readfsqword(0x28u);
}



// Function: extract_finish @ 0x18640
unsigned long long extract_finish()
{
  sub_17870("", 0);
  sub_18350();
  return sub_17870("", 1u);
}



// Function: maybe_recoverable @ 0x18750
long long maybe_recoverable(char *src, char a2, char *a3)
{
  long long result; // rax
  int *v5; // rax
  int v6; // ebp
  int *v7; // r12
  struct stat *v8; // rsi
  char *v9; // rax
  struct stat v10; // [rsp+0h] [rbp-C8h] BYREF
  unsigned long long v11; // [rsp+98h] [rbp-30h]

  v11 = __readfsqword(0x28u);
  result = 0;
  if ( *a3 )
    return result;
  v5 = __errno_location();
  v6 = *v5;
  v7 = v5;
  if ( *v5 == 17 )
  {
    v8 = 0;
  }
  else
  {
    if ( v6 <= 17 )
    {
      if ( v6 != 2 )
      {
LABEL_8:
        *v7 = v6;
        return 0;
      }
      goto LABEL_11;
    }
    if ( v6 != 40 && (v6 & 0xFFFFFFBF) != 0x1F || !a2 || dword_84B34 != 2 || byte_84B69 )
      goto LABEL_8;
    if ( !strchr(src, 47) )
      goto LABEL_22;
    v8 = &v10;
    if ( (unsigned int)sub_246F0(src, &v10) )
      goto LABEL_8;
  }
  if ( dword_84B34 == 5 )
  {
    result = 2;
    if ( (dword_83B80 & 0x100000) != 0 )
    {
      if ( qword_84D50 )
        qword_84D50(src);
      v9 = dcgettext(0, "%s: skipping existing file", 5);
      error(0, 0, v9, src);
      return 2;
    }
    return result;
  }
  if ( (unsigned int)dword_84B34 > 5 )
  {
    if ( dword_84B34 != 6 || (unsigned char)sub_18670(src, v8) )
    {
LABEL_11:
      if ( (unsigned int)sub_17F70(src, a3) || !*a3 )
        goto LABEL_8;
      return 1;
    }
LABEL_22:
    if ( (int)sub_25490(src, 0) > 0 )
      return 1;
    goto LABEL_11;
  }
  if ( (unsigned int)dword_84B34 <= 2 )
    goto LABEL_22;
  result = 0;
  if ( dword_84B34 != 4 )
    goto LABEL_11;
  return result;
}



// Function: extract_fifo @ 0x18900
long long extract_fifo(char *src, char a2)
{
  short st_mode; // bx
  __mode_t v3; // ebx
  int v4; // eax
  char v6; // [rsp+17h] [rbp-31h] BYREF
  unsigned long long v7; // [rsp+18h] [rbp-30h]

  st_mode = buf.st_mode;
  v7 = __readfsqword(0x28u);
  v6 = 0;
  if ( dword_84AA8 > 0 )
    st_mode = buf.st_mode & 0xFFC0;
  v3 = st_mode & 0x1FF;
  while ( mkfifoat(oldfd, src, v3) )
  {
    v4 = sub_18750(src, 0, &v6);
    if ( !v4 )
    {
      sub_32780(src);
      return 0xFFFFFFFFLL;
    }
    if ( v4 == 2 )
      return 0;
  }
  sub_17530(src, (long long)&qword_84840, 0xFFFFFFFF, v3 & ~dword_84328, 511, a2, 0, 256);
  return 0;
}



// Function: extract_node @ 0x189e0
long long extract_node(char *src, char a2)
{
  short st_mode; // bx
  __mode_t v3; // ebx
  int v4; // eax
  char v6; // [rsp+17h] [rbp-31h] BYREF
  unsigned long long v7; // [rsp+18h] [rbp-30h]

  st_mode = buf.st_mode;
  v7 = __readfsqword(0x28u);
  v6 = 0;
  if ( dword_84AA8 > 0 )
    st_mode = buf.st_mode & 0xFFC0;
  v3 = st_mode & 0x61FF;
  while ( mknodat(oldfd, src, v3, buf.st_rdev) )
  {
    v4 = sub_18750(src, 0, &v6);
    if ( !v4 )
    {
      sub_327A0(src);
      return 0xFFFFFFFFLL;
    }
    if ( v4 == 2 )
      return 0;
  }
  sub_17530(src, (long long)&qword_84840, 0xFFFFFFFF, v3 & ~dword_84328, 511, a2, 0, 256);
  return 0;
}



// Function: create_placeholder_file @ 0x18ad0
long long create_placeholder_file(char *src, char a2, char *a3, void **a4)
{
  int v6; // eax
  int v7; // r14d
  int v8; // eax
  unsigned int v9; // r14d
  size_t v11; // rax
  long long *v12; // rbx
  void *v13; // rax
  long long v14; // rax
  long long v15; // rdx
  __m128i v16; // xmm0
  __m128i v17; // xmm1
  long long v18; // rax
  size_t v19; // rax
  long long v20; // rax
  long long v21; // rsi
  long long *v22; // rdi
  int v23; // edi
  struct stat v24; // [rsp+0h] [rbp-D8h] BYREF
  unsigned long long v25; // [rsp+98h] [rbp-40h]

  v25 = __readfsqword(0x28u);
  while ( 1 )
  {
    v6 = openat(oldfd, src, 193, 0);
    v7 = v6;
    if ( v6 >= 0 )
      break;
    if ( *__errno_location() == 17 && sub_16DA0(src) )
      return 0;
    v8 = sub_18750(src, 0, a3);
    if ( !v8 )
    {
      v9 = -1;
      sub_327C0(src);
      return v9;
    }
    if ( v8 == 2 )
      return 0;
  }
  if ( fstat(v6, &v24) )
  {
    sub_32D30(src);
    v23 = v7;
    v9 = -1;
    close(v23);
  }
  else
  {
    v9 = close(v7);
    if ( v9 )
    {
      v9 = -1;
      sub_32680(src);
    }
    else
    {
      v11 = strlen(from);
      v12 = (long long *)sub_4C5B0(v11 + 161);
      if ( a4 )
      {
        v13 = *a4;
        *a4 = v12;
      }
      else
      {
        v13 = qword_84318;
        qword_84318 = v12;
      }
      *v12 = v13;
      v12[1] = v24.st_dev;
      v12[2] = v24.st_ino;
      v14 = sub_4BD30(&v24);
      *((char *)v12 + 40) = a2;
      v12[3] = v14;
      v12[4] = v15;
      if ( a2 )
      {
        v16 = _mm_loadu_si128((const __m128i *)&xmmword_84928);
        v17 = _mm_loadu_si128((const __m128i *)&file);
        *((int *)v12 + 11) = buf.st_mode;
        v18 = *(long long *)&buf.st_uid;
        *(__m128i *)(v12 + 7) = v16;
        v12[6] = v18;
        *(__m128i *)(v12 + 9) = v17;
      }
      *((int *)v12 + 22) = dword_84580;
      v19 = strlen(src);
      v20 = sub_4C5B0(v19 + 9);
      v12[12] = v20;
      *(long long *)v20 = 0;
      strcpy((char *)(v20 + 8), src);
      v21 = qword_84870;
      v12[13] = 0;
      sub_23BE0(v12 + 13, v21);
      v12[14] = 0;
      v12[15] = 0;
      v12[16] = 0;
      v12[17] = 0;
      sub_1C840(&qword_84840, v12 + 19, v12 + 18);
      strcpy((char *)v12 + 160, from);
      v22 = (long long *)sub_182E0(src);
      if ( !v22 )
        return 0;
      sub_17B10(v22);
    }
  }
  return v9;
}



// Function: extract_link @ 0x18d40
long long extract_link(char *src)
{
  char *v1; // r12
  void **v2; // rax
  int *v3; // rbx
  int v4; // eax
  int v5; // eax
  int v6; // r14d
  unsigned int v7; // r8d
  long long *v9; // rbx
  int v10; // r12d
  long long v11; // rax
  long long v12; // rdx
  size_t v13; // rax
  long long v14; // r12
  char v15; // [rsp+Fh] [rbp-159h] BYREF
  struct stat buf; // [rsp+10h] [rbp-158h] BYREF
  struct stat v17; // [rsp+A0h] [rbp-C8h] BYREF
  unsigned long long v18; // [rsp+138h] [rbp-30h]

  v1 = from;
  v18 = __readfsqword(0x28u);
  v15 = 0;
  if ( !byte_84B92 && (unsigned char)sub_27CC0(from) )
  {
    return (unsigned int)sub_18AD0(src, 0, &v15, 0);
  }
  else
  {
    v2 = (void **)sub_16DA0(v1);
    if ( v2 )
    {
      return (unsigned int)sub_18AD0(src, 0, &v15, v2);
    }
    else
    {
      v3 = __errno_location();
      do
      {
        v5 = linkat(oldfd, v1, oldfd, src, 0);
        v6 = *v3;
        if ( !v5 )
        {
          v9 = qword_84318;
          if ( qword_84318 && !fstatat(oldfd, v1, &buf, 256) )
          {
            v10 = dword_84580;
            while ( 1 )
            {
              if ( *((int *)v9 + 22) == v10 && v9[1] == buf.st_dev && v9[2] == buf.st_ino )
              {
                v11 = sub_4BD30(&buf);
                if ( !(unsigned int)sub_4BE80(v9[3], v9[4], v11, v12) )
                  break;
              }
              v9 = (long long *)*v9;
              if ( !v9 )
                return 0;
            }
            v13 = strlen(src);
            v14 = sub_4C5B0(v13 + 9);
            strcpy((char *)(v14 + 8), src);
            *(long long *)v14 = v9[12];
            v9[12] = v14;
          }
          return 0;
        }
        if ( v6 == 17 && !strcmp(v1, src)
          || !fstatat(oldfd, v1, &buf, 256)
          && !fstatat(oldfd, src, &v17, 256)
          && buf.st_dev == v17.st_dev
          && buf.st_ino == v17.st_ino )
        {
          return 0;
        }
        *v3 = v6;
        v4 = sub_18750(src, 0, &v15);
      }
      while ( v4 == 1 );
      v7 = 0;
      if ( v4 != 2 && (!byte_84B50 || *v3 != 17) )
      {
        sub_326E0(v1, src);
        return 1;
      }
    }
  }
  return v7;
}



// Function: extract_symlink @ 0x18fc0
long long extract_symlink(char *src)
{
  const char *v2; // rdi
  char v3; // al
  int v4; // eax
  char v6; // [rsp+17h] [rbp-21h] BYREF
  unsigned long long v7; // [rsp+18h] [rbp-20h]

  v2 = from;
  v7 = __readfsqword(0x28u);
  v6 = 0;
  if ( !byte_84B92 )
  {
    if ( *from == 47 )
      return sub_18AD0(src, 1, &v6, 0);
    v3 = sub_27CC0(from);
    v2 = from;
    if ( v3 )
      return sub_18AD0(src, 1, &v6, 0);
  }
  while ( symlinkat(v2, oldfd, src) )
  {
    v4 = sub_18750(src, 0, &v6);
    if ( !v4 )
    {
      sub_32C90(from, src);
      return 0xFFFFFFFFLL;
    }
    if ( v4 == 2 )
      return 0;
    v2 = from;
  }
  sub_17530(src, (long long)&qword_84840, 0xFFFFFFFF, 0, 0, 50, 0, 256);
  return 0;
}



// Function: extract_file @ 0x191d0
long long extract_file(void *src, char a2)
{
  unsigned int v3; // r14d
  int v4; // ebp
  __off_t st_size; // rdi
  void *v6; // rax
  void *v7; // rbx
  unsigned long long v8; // r13
  long long v9; // rax
  char *v10; // r15
  int v11; // r13d
  int v13; // r15d
  int v14; // eax
  char *v15; // rax
  int v16; // [rsp+Ch] [rbp-6Ch]
  char v17; // [rsp+27h] [rbp-51h] BYREF
  int v18; // [rsp+28h] [rbp-50h] BYREF
  int v19; // [rsp+2Ch] [rbp-4Ch] BYREF
  unsigned long long v20; // [rsp+30h] [rbp-48h]
  unsigned long long v21; // [rsp+38h] [rbp-40h]

  v21 = __readfsqword(0x28u);
  v17 = 0;
  v3 = buf.st_mode & 0x1C0;
  if ( dword_84AA8 <= 0 )
    v3 = buf.st_mode & 0x1FF;
  v18 = 0;
  v19 = 0;
  if ( byte_84A6A )
  {
    v4 = 1;
  }
  else if ( qword_84A60 )
  {
    v4 = sub_2B640(src, 102, &qword_84840);
    if ( v4 < 0 )
    {
      sub_22BD0();
      return 0;
    }
  }
  else
  {
    LODWORD(v20) = 0;
    v11 = sub_190D0(src);
    if ( v11 )
      return 1;
    v13 = v20;
    while ( 1 )
    {
      v4 = sub_18140((char *)src, a2, v3, v13, (__mode_t *)&v18, &v19);
      if ( v4 >= 0 )
        break;
      v14 = sub_18750((char *)src, 1, &v17);
      if ( v14 != 1 )
      {
        v16 = v14;
        sub_22BD0();
        if ( v16 != 2 )
        {
          v11 = 1;
          sub_327C0(src);
        }
        return (unsigned int)v11;
      }
    }
  }
  sub_CCB0((long long)&qword_84840);
  if ( byte_84960 )
  {
    sub_29ED0(v4);
  }
  else
  {
    st_size = buf.st_size;
    v20 = buf.st_size;
    while ( st_size > 0 )
    {
      sub_CCF0(st_size);
      v6 = sub_E130();
      v7 = v6;
      if ( !v6 )
      {
        if ( qword_84D50 )
          qword_84D50(st_size);
        v15 = dcgettext(0, "Unexpected EOF in archive", 5);
        error(0, 0, v15);
        dword_84D58 = 2;
        break;
      }
      v8 = (unsigned long long)sub_D2B0((long long)v6);
      if ( v8 > v20 )
        v8 = v20;
      *__errno_location() = 0;
      v9 = sub_249F0(v4);
      v20 -= v8;
      v10 = (char *)v9;
      sub_D1F0((unsigned long long)v7 + v8 - 1);
      if ( (char *)v8 != v10 )
      {
        if ( !qword_84A60 )
          sub_32E30(src, v10, v8);
        break;
      }
      st_size = v20;
    }
  }
  sub_22B10();
  v11 = 0;
  sub_CCD0();
  if ( !byte_84A6A )
  {
    if ( qword_84A60 )
    {
      v11 = close(v4);
      if ( v11 >= 0 )
        goto LABEL_17;
    }
    else
    {
      sub_17530((const char *)src, (long long)&qword_84840, v4, v18, v19, a2, 0, (dword_84B34 != 2) << 8);
      v11 = close(v4);
      if ( v11 >= 0 )
        goto LABEL_17;
    }
    sub_32680(src);
LABEL_17:
    if ( qword_84A60 )
      sub_2B6F0();
  }
  return (unsigned int)v11;
}



// Function: prepare_to_extract @ 0x19540
long long prepare_to_extract(long long a1, unsigned int a2, long long (**a3)(void *src, char a2))
{
  long long (*v4)(void *, char); // rbx
  long long v6; // r13
  char *v7; // rax
  long long v8; // r12
  char *v9; // rax
  char *v10; // rax
  long long v11; // r12
  char *v12; // rax

  switch ( a2 )
  {
    case 0u:
    case 0x30u:
    case 0x37u:
      if ( !byte_84850 )
        goto LABEL_11;
      v4 = (long long (*)(void *, char))sub_19850;
      goto LABEL_3;
    case 0x31u:
      v4 = (long long (*)(void *, char))sub_18D40;
      goto LABEL_3;
    case 0x32u:
      v4 = (long long (*)(void *, char))sub_18FC0;
      goto LABEL_3;
    case 0x33u:
      buf.st_mode |= 0x2000u;
      v4 = (long long (*)(void *, char))sub_189E0;
      goto LABEL_3;
    case 0x34u:
      buf.st_mode |= 0x6000u;
      v4 = (long long (*)(void *, char))sub_189E0;
      goto LABEL_3;
    case 0x35u:
    case 0x44u:
      v4 = (long long (*)(void *, char))sub_19850;
      if ( byte_849C8 )
        byte_847E0 = 1;
      goto LABEL_3;
    case 0x36u:
      v4 = (long long (*)(void *, char))sub_18900;
LABEL_3:
      if ( byte_84A6A )
        return 0;
      if ( !qword_84A60 )
        goto LABEL_15;
      if ( v4 != sub_191D0 )
        return 0;
      goto LABEL_12;
    case 0x4Bu:
    case 0x4Cu:
      if ( qword_84D50 )
        qword_84D50(a1);
      v10 = dcgettext(0, "Unexpected long name header", 5);
      error(0, 0, v10);
      dword_84D58 = 2;
      return 0;
    case 0x4Du:
      if ( qword_84D50 )
        qword_84D50(a1);
      v11 = sub_4AA70(path);
      v12 = dcgettext(0, "%s: Cannot extract -- file is continued from another volume", 5);
      error(0, 0, v12, v11);
      dword_84D58 = 2;
      return 0;
    case 0x53u:
      goto LABEL_11;
    case 0x56u:
      return 0;
    default:
      if ( (dword_83B80 & 0x10000) != 0 )
      {
        if ( qword_84D50 )
          qword_84D50(a1);
        v6 = sub_4AA70(a1);
        v7 = dcgettext(0, "%s: Unknown file type '%c', extracted as normal file", 5);
        error(0, 0, v7, v6, a2);
      }
LABEL_11:
      if ( byte_84A6A || (v4 = sub_191D0, qword_84A60) )
      {
LABEL_12:
        v4 = sub_191D0;
        goto LABEL_13;
      }
LABEL_15:
      if ( dword_84B34 == 3 )
      {
        if ( !(unsigned int)sub_25490(a1, (unsigned char)byte_84ABA) && (*__errno_location() & 0xFFFFFFFD) != 0 )
        {
          sub_32DB0(a1);
          return 0;
        }
LABEL_13:
        *a3 = v4;
        return 1;
      }
      if ( dword_84B34 != 6 || !(unsigned char)sub_18670(a1, 0) )
        goto LABEL_13;
      if ( (dword_83B80 & 0x800) != 0 )
      {
        if ( qword_84D50 )
          qword_84D50(a1);
        v8 = sub_4AC50(a1);
        v9 = dcgettext(0, "Current %s is newer or same age", 5);
        error(0, 0, v9, v8);
      }
      return 0;
  }
}



// Function: extract_dir @ 0x19850
long long extract_dir(char *src, int a2)
{
  int v3; // r15d
  int st_mode; // r14d
  __mode_t v5; // r13d
  unsigned int v6; // r12d
  int *v7; // rbx
  int v8; // eax
  int v9; // eax
  int v10; // r9d
  int v12; // r8d
  struct stat *p_buf; // rsi
  long long v14; // rcx
  struct stat *v15; // rdi
  char v16; // [rsp+Fh] [rbp-E9h]
  char v17; // [rsp+1Fh] [rbp-D9h] BYREF
  struct stat buf; // [rsp+20h] [rbp-D8h] BYREF
  unsigned long long v19; // [rsp+B8h] [rbp-40h]

  v19 = __readfsqword(0x28u);
  v17 = 0;
  if ( byte_84AD1 && !qword_847F0 )
  {
    if ( fstatat(oldfd, ".", &buf, 0) )
      sub_24EC0(".");
    else
      qword_847F0 = buf.st_dev;
  }
  if ( byte_84B50 )
  {
    sub_206E0(src);
  }
  else if ( a2 == 68 )
  {
    sub_22BD0();
  }
  v3 = 0;
  st_mode = 0;
  v5 = sub_19820(::buf.st_mode);
  while ( 1 )
  {
    v6 = mkdirat(oldfd, src, v5);
    if ( !v6 )
      goto LABEL_37;
    v7 = __errno_location();
    if ( *v7 != 17 )
      goto LABEL_15;
    v16 = v17;
    if ( v17 )
      break;
    if ( byte_84B30 )
      goto LABEL_23;
    if ( (unsigned int)dword_84B34 <= 2 )
      goto LABEL_11;
LABEL_15:
    v8 = sub_18750(src, 0, &v17);
    if ( !v8 )
    {
      if ( *v7 != 17 )
      {
        v6 = 1;
        sub_32760(src);
        return v6;
      }
LABEL_17:
      v9 = dword_84B34;
LABEL_18:
      if ( (v9 & 0xFFFFFFFD) != 0 )
        return v6;
      v10 = 0;
LABEL_20:
      sub_17C30(src, (long long)&qword_84840, st_mode, v3, ::buf.st_mode, v10);
      return v6;
    }
    if ( v8 != 1 )
      goto LABEL_17;
  }
  if ( !byte_84B30 )
    goto LABEL_11;
LABEL_23:
  if ( sub_16CE0(src) )
    return 0;
LABEL_11:
  if ( (unsigned int)sub_246F0(src, &buf) )
  {
LABEL_14:
    *v7 = 17;
    goto LABEL_15;
  }
  st_mode = buf.st_mode;
  if ( (buf.st_mode & 0xF000) != 0x4000 )
  {
    v3 = -1;
    goto LABEL_14;
  }
  if ( !v16 )
  {
    v9 = dword_84B34;
    if ( dword_84B34 == 1 )
    {
      v5 = sub_19820(buf.st_mode);
      v6 = sub_17280(0xFFFFFFFFLL, src, v5, 256, v12);
      if ( !v6 )
      {
        p_buf = &buf;
        v14 = 36;
        v15 = &::buf;
        while ( v14 )
        {
          LODWORD(v15->st_dev) = p_buf->st_dev;
          p_buf = (struct stat *)((char *)p_buf + 4);
          v15 = (struct stat *)((char *)v15 + 4);
          --v14;
        }
LABEL_37:
        v10 = 256;
        v3 = 511;
        st_mode = v5 & ~dword_84328;
        goto LABEL_20;
      }
      sub_32550(src, v5);
      v9 = dword_84B34;
    }
    v3 = -1;
    goto LABEL_18;
  }
  sub_16E60((long long)src, &buf);
  return 0;
}



// Function: extr_init @ 0x19b30
int extr_init()
{
  int result; // eax
  int v1; // ebx

  byte_84330 = geteuid() == 0;
  dword_84AA4 += (unsigned char)byte_84330;
  dword_84AA8 += (unsigned char)byte_84330;
  result = umask(0);
  dword_8432C = result;
  if ( dword_84AA4 <= 0 )
  {
    v1 = result;
    result = umask(result);
    dword_84328 = v1;
  }
  else
  {
    dword_84328 = 0;
  }
  return result;
}



// Function: remove_delayed_set_stat @ 0x19b90
void remove_delayed_set_stat(char *s2)
{
  void *v1; // rbp
  int v2; // r13d
  long long *v3; // r12
  void *v4; // rbx

  v1 = qword_84320;
  if ( qword_84320 )
  {
    v2 = dword_84580;
    v3 = 0;
    while ( 1 )
    {
      v4 = *(void **)v1;
      if ( *((int *)v1 + 23) == v2 && !strcmp(*((const char **)v1 + 20), s2) )
        break;
      v3 = v1;
      if ( !v4 )
        return;
      v1 = v4;
    }
    sub_16C90((void **)v1);
    if ( v3 )
      *v3 = v4;
    else
      qword_84320 = v4;
  }
}



// Function: extract_archive @ 0x19c10
long long extract_archive()
{
  char *v0; // rdi
  int v1; // r12d
  long long v3; // rdi
  long long v4; // r12
  char *v5; // rax
  unsigned int v6; // r12d
  int v7; // ebp
  long long v8; // r12
  char *v9; // rax
  long long (*v10)(void *, char); // [rsp+0h] [rbp-28h] BYREF
  unsigned long long v11; // [rsp+8h] [rbp-20h]

  v11 = __readfsqword(0x28u);
  qword_842F0 = (long long (*)(void))sub_18640;
  sub_D1F0(qword_84408);
  if ( !byte_84B92 )
  {
    v3 = qword_84840;
    if ( (unsigned char)sub_27CC0(qword_84840) )
    {
      if ( qword_84D50 )
        qword_84D50(v3);
      v4 = sub_4AA70(qword_84840);
      v5 = dcgettext(0, "%s: Member name contains '..'", 5);
      error(0, 0, v5, v4);
      dword_84D58 = 2;
      return sub_22BD0();
    }
  }
  if ( !*path )
    return sub_22BD0();
  if ( !byte_84B40 )
  {
    if ( !dword_84A54 )
      goto LABEL_5;
LABEL_21:
    sub_22990(&qword_84840, qword_84408, -1);
    if ( byte_847E0 )
      goto LABEL_6;
LABEL_22:
    v6 = dword_84580;
    sub_17870(path, 0);
    sub_24C80(v6);
    goto LABEL_6;
  }
  if ( !(unsigned int)sub_2CA20("extract", path) )
    return sub_22BD0();
  if ( dword_84A54 )
    goto LABEL_21;
LABEL_5:
  if ( !byte_847E0 )
    goto LABEL_22;
LABEL_6:
  if ( byte_84B84 )
  {
    v0 = path;
    if ( !(unsigned char)sub_24710(path) )
    {
      v7 = *__errno_location();
      if ( qword_84D50 )
        qword_84D50(v0);
      v8 = sub_4AA70(path);
      v9 = dcgettext(0, "%s: Was unable to backup this file", 5);
      error(0, v7, v9, v8);
      dword_84D58 = 2;
      return sub_22BD0();
    }
  }
  v1 = 83;
  if ( !(unsigned char)sub_29E10(&qword_84840) )
    v1 = *(char *)(qword_84408 + 156);
  if ( (unsigned char)sub_19540((long long)path, v1, &v10) )
  {
    if ( !(unsigned int)v10(path, v1) )
      return v11 - __readfsqword(0x28u);
  }
  else
  {
    sub_22BD0();
  }
  if ( !byte_84B84 )
    return v11 - __readfsqword(0x28u);
  return sub_245D0();
}



// Function: rename_directory @ 0x19ec0
long long rename_directory(char *s2, void *src)
{
  unsigned long long v6; // rdi
  int *v7; // rax
  int v8; // r13d
  int *v9; // rbx
  long long v10; // rbx
  long long v11; // r12
  char *v12; // rax
  long long result; // rax
  int v14; // r8d
  char v15; // [rsp+7h] [rbp-31h] BYREF
  unsigned long long v16; // [rsp+8h] [rbp-30h]

  v6 = (unsigned int)oldfd;
  v16 = __readfsqword(0x28u);
  if ( !renameat(oldfd, s2, oldfd, (const char *)src) )
  {
    sub_17BB0(s2, src);
    return 1;
  }
  v7 = __errno_location();
  v8 = *v7;
  v9 = v7;
  if ( *v7 == 2 )
  {
    v6 = (unsigned long long)src;
    if ( !(unsigned int)sub_17F70((char *)src, &v15) )
    {
      v6 = (unsigned int)oldfd;
      v14 = renameat(oldfd, s2, oldfd, (const char *)src);
      result = 1;
      if ( !v14 )
        return result;
      v8 = *v9;
    }
  }
  if ( qword_84D50 )
    qword_84D50(v6);
  v10 = sub_4AC40(1, src);
  v11 = sub_4AC40(0, s2);
  v12 = dcgettext(0, "Cannot rename %s to %s", 5);
  error(0, v8, v12, v11, v10);
  result = 0;
  dword_84D58 = 2;
  return result;
}



// Function: xattr_decode_keyword @ 0x19ff0
long long xattr_decode_keyword(unsigned char *a1)
{
  unsigned char *i; // rdx
  long long result; // rax
  unsigned char v3; // al

  for ( i = a1 + 1; ; ++i )
  {
    result = *a1;
    if ( (char)result != 37 )
      break;
    v3 = a1[1];
    if ( v3 == 51 )
    {
      if ( a1[2] != 68 )
        goto LABEL_9;
      *(i - 1) = 61;
      a1 += 3;
    }
    else
    {
      if ( v3 != 50 || a1[2] != 53 )
      {
LABEL_9:
        *(i - 1) = 37;
        goto LABEL_3;
      }
      *(i - 1) = 37;
      a1 += 3;
    }
LABEL_4:
    ;
  }
  *(i - 1) = result;
  if ( (char)result )
  {
LABEL_3:
    ++a1;
    goto LABEL_4;
  }
  return result;
}



// Function: dummy_coder @ 0x1a060
void dummy_coder()
{
  ;
}



// Function: dummy_decoder @ 0x1a070
void dummy_decoder()
{
  ;
}



// Function: xheader_protected_pattern_p @ 0x1a080
long long xheader_protected_pattern_p(char *pattern)
{
  const char *v1; // rsi
  char **v2; // rbx

  v1 = "atime";
  v2 = &off_7F200;
  do
  {
    if ( !*((char *)v2 + 28) && ((char)v2[3] & 1) != 0 && !fnmatch(pattern, v1, 0) )
      return 1;
    v1 = v2[4];
    v2 += 4;
  }
  while ( v1 );
  return 0;
}



// Function: xheader_set_single_keyword @ 0x1a0f0
void xheader_set_single_keyword(long long a1)
{
  char *v1; // rax

  if ( qword_84D50 )
    qword_84D50(a1);
  v1 = dcgettext(0, "Keyword %s is unknown or not yet implemented", 5);
  error(0, 0, v1, a1);
  sub_2C970(2);
}



// Function: xheader_protected_keyword_p @ 0x1a140
long long xheader_protected_keyword_p(char *s2)
{
  const char *v2; // rdi
  char **i; // rbx

  v2 = "atime";
  for ( i = &off_7F200; *((char *)i + 28) || ((char)i[3] & 1) == 0 || strcmp(v2, s2); i += 4 )
  {
    v2 = i[4];
    if ( !v2 )
      return 0;
  }
  return 1;
}



// Function: xheader_keyword_override_p @ 0x1a1a0
long long xheader_keyword_override_p(char *s2)
{
  long long v1; // rbx

  v1 = qword_84380;
  if ( !qword_84380 )
    return 0;
  while ( strcmp(*(const char **)(v1 + 8), s2) )
  {
    v1 = *(long long *)v1;
    if ( !v1 )
      return 0;
  }
  return 1;
}



// Function: assign_time_option @ 0x1a200
unsigned long long assign_time_option(long long a1, long long *a2, char *a3)
{
  long long v5; // rdi
  long long v6; // rdx
  char *v7; // rax
  char *v9; // [rsp+0h] [rbp-38h] BYREF
  unsigned long long v10; // [rsp+8h] [rbp-30h]

  v10 = __readfsqword(0x28u);
  v5 = sub_242F0(a3, &v9);
  if ( (unsigned char)sub_23BA0(v5, v6) && !*v9 )
  {
    *a2 = v5;
    sub_23BE0(a1, a3);
  }
  else
  {
    if ( qword_84D50 )
      qword_84D50(v5);
    v7 = dcgettext(0, "Time stamp is out of allowed range", 5);
    error(0, 0, v7);
    dword_84D58 = 2;
  }
  return v10 - __readfsqword(0x28u);
}



// Function: xheader_list_append @ 0x1a2c0
long long xheader_list_append(long long *a1, void *a2, void *a3)
{
  long long *v4; // rbx
  long long result; // rax

  v4 = (long long *)sub_4C5B0(24);
  v4[1] = sub_4C7A0(a2);
  if ( a3 )
    a3 = (void *)sub_4C7A0(a3);
  result = *a1;
  v4[2] = a3;
  *a1 = (long long)v4;
  *v4 = result;
  return result;
}



// Function: xheader_set_keyword_equal @ 0x1a320
unsigned long long xheader_set_keyword_equal(char *s2, char *a2)
{
  char *v3; // rbx
  char v4; // r12
  const unsigned short *v5; // rdx
  long long v6; // rbx
  char *v7; // rbp
  const unsigned short *v8; // rax
  char *v9; // rsi
  long long *v10; // rdi
  int v11; // eax
  char *v12; // rdx
  long long *v13; // rsi
  long long *v14; // rdi
  char *v15; // rdx
  char *v16; // rsi
  long long *v17; // rdi
  char *v19; // rax
  long long v20; // r12
  char *v21; // rax
  long long v22; // r12
  char *v23; // rax

  if ( a2 == s2 )
  {
    if ( qword_84D50 )
      qword_84D50(s2);
    v22 = sub_4AC50(s2);
    v23 = dcgettext(0, "Malformed pax option: %s", 5);
    error(0, 0, v23, v22);
    goto LABEL_30;
  }
  v3 = a2;
  v4 = 1;
  if ( *(a2 - 1) == 58 )
  {
    v3 = a2 - 1;
    v4 = 0;
  }
  if ( s2 < v3 )
  {
    v5 = *__ctype_b_loc();
    do
    {
      if ( (v5[(unsigned char)*v3] & 0x2000) == 0 )
        break;
      --v3;
    }
    while ( s2 != v3 );
  }
  *v3 = 0;
  v6 = (unsigned char)a2[1];
  v7 = a2 + 1;
  if ( (char)v6 )
  {
    v8 = *__ctype_b_loc();
    do
    {
      if ( (v8[v6] & 0x2000) == 0 )
        break;
      v6 = (unsigned char)*++v7;
    }
    while ( (char)v6 );
  }
  if ( !strcmp(s2, "delete") )
  {
    if ( !(unsigned char)sub_1A080(v7) )
    {
      v15 = 0;
      v16 = v7;
      v17 = &qword_84390;
      return sub_1A2C0(v17, v16, v15);
    }
    if ( qword_84D50 )
      qword_84D50(v7);
    v20 = sub_4AC50(v7);
    v21 = dcgettext(0, "Pattern %s cannot be used", 5);
    error(0, 0, v21, v20);
LABEL_30:
    sub_2C970(2);
  }
  v9 = v7;
  v10 = &qword_84370;
  if ( !strcmp(s2, "exthdr.name") )
    return sub_23BE0(v10, v9);
  if ( !strcmp(s2, "globexthdr.name") )
  {
    v9 = v7;
    v10 = &qword_84358;
    return sub_23BE0(v10, v9);
  }
  v11 = strcmp(s2, "exthdr.mtime");
  v12 = v7;
  v13 = &qword_84360;
  v14 = &qword_84368;
  if ( v11 )
  {
    if ( strcmp(s2, "globexthdr.mtime") )
    {
      if ( !(unsigned char)sub_1A140(s2) )
      {
        v15 = v7;
        v16 = s2;
        v17 = &qword_84388;
        if ( !v4 )
          v17 = &qword_84380;
        return sub_1A2C0(v17, v16, v15);
      }
      if ( qword_84D50 )
        qword_84D50(s2);
      v19 = dcgettext(0, "Keyword %s cannot be overridden", 5);
      error(0, 0, v19, s2);
      goto LABEL_30;
    }
    v12 = v7;
    v13 = &qword_84348;
    v14 = &qword_84350;
  }
  return sub_1A200((long long)v14, v13, v12);
}



// Function: dumpdir_decoder @ 0x1a560
void *dumpdir_decoder(long long a1, long long a2, const void *a3, size_t a4)
{
  void *v6; // rax

  v6 = (void *)sub_4C5B0(a4);
  *(long long *)(a1 + 400) = v6;
  return memcpy(v6, a3, a4);
}



// Function: xheader_init @ 0x1a5a0
void xheader_init(long long *a1)
{
  long long v1; // rax

  if ( !*a1 )
  {
    v1 = sub_4C5B0(88);
    *a1 = v1;
    obstack_begin(v1, 0, 0, sub_4C5B0, &free);
  }
}



// Function: decode_string @ 0x1a5e0
long long decode_string(void **a1, long long a2)
{
  void *v3; // rdi
  long long result; // rax

  v3 = *a1;
  if ( v3 )
  {
    free(v3);
    *a1 = 0;
  }
  result = sub_31340(0, a2, a1);
  if ( !(char)result )
    return sub_23BE0(a1, a2);
  return result;
}



// Function: xattr_selinux_decoder @ 0x1a640
long long xattr_selinux_decoder(long long a1, long long a2, long long a3)
{
  return sub_1A5E0((void **)(a1 + 48), a3);
}



// Function: xattr_encode_keyword @ 0x1a6b0
char *xattr_encode_keyword(char *a1)
{
  char *v2; // rdi
  char v3; // bp
  long long v4; // rbx
  int *v5; // rax

  v2 = (char *)qword_84340;
  if ( qword_84340 )
  {
    *(char *)qword_84340 = 0;
  }
  else
  {
    qword_84338 = 256;
    qword_84340 = sub_4C5B0(256);
    v2 = (char *)qword_84340;
  }
  v3 = *a1;
  v4 = 0;
  if ( !*a1 )
  {
    *v2 = 0;
    return v2;
  }
  do
  {
    if ( v4 + 2 < (unsigned long long)qword_84338 )
    {
      v5 = &v2[v4];
      if ( v3 == 37 )
        goto LABEL_11;
    }
    else
    {
      v2 = (char *)sub_4C710(v2, &qword_84338);
      qword_84340 = (long long)v2;
      v5 = &v2[v4];
      if ( v3 == 37 )
      {
LABEL_11:
        *v5 = 3486245;
        v4 += 2;
        goto LABEL_8;
      }
    }
    if ( v3 == 61 )
    {
      *v5 = 4469541;
      v4 += 2;
    }
    else
    {
      *(char *)v5 = v3;
    }
LABEL_8:
    v3 = *++a1;
    ++v4;
  }
  while ( v3 );
  v2[v4] = 0;
  return v2;
}



// Function: x_obstack_grow @ 0x1a7c0
void *x_obstack_grow(long long *a1, const void *a2, size_t a3)
{
  long long v5; // r12
  void *v6; // rdi
  void *result; // rax

  v5 = *a1;
  v6 = *(void **)(*a1 + 24LL);
  if ( a3 > *(long long *)(v5 + 32) - (long long)v6 )
  {
    obstack_newchunk(v5, a3);
    v6 = *(void **)(v5 + 24);
  }
  result = memcpy(v6, a2, a3);
  *(long long *)(v5 + 24) += a3;
  a1[1] += a3;
  return result;
}



// Function: x_obstack_1grow @ 0x1a830
char *x_obstack_1grow(long long *a1, char a2)
{
  long long v2; // rbp
  char *result; // rax

  v2 = *a1;
  result = *(char **)(*a1 + 24LL);
  if ( *(char **)(*a1 + 32LL) == result )
  {
    obstack_newchunk(*a1, 1);
    result = *(char **)(v2 + 24);
  }
  *(long long *)(v2 + 24) = result + 1;
  *result = a2;
  ++a1[1];
  return result;
}



// Function: xheader_print_n @ 0x1a880
unsigned long long xheader_print_n(long long *a1, char *a2, const void *a3, size_t a4)
{
  char *v4; // r15
  const char *v6; // r14
  long long v7; // rsi
  char *v8; // rax
  size_t v10; // [rsp+8h] [rbp-70h]
  char v13[20]; // [rsp+20h] [rbp-58h] BYREF
  char v14[4]; // [rsp+34h] [rbp-44h] BYREF
  unsigned long long v15; // [rsp+38h] [rbp-40h]

  v4 = 0;
  v15 = __readfsqword(0x28u);
  v6 = sub_1A6B0(a2);
  v10 = strlen(v6);
  do
  {
    v7 = sub_43090(&v4[v10 + 3 + a4], v13);
    v8 = v4;
    v4 = &v14[-v7];
  }
  while ( v8 != &v14[-v7] );
  sub_1A7C0(a1, (const void *)v7, (size_t)&v14[-v7]);
  sub_1A830(a1, 32);
  sub_1A7C0(a1, v6, v10);
  sub_1A830(a1, 61);
  sub_1A7C0(a1, a3, a4);
  sub_1A830(a1, 10);
  return v15 - __readfsqword(0x28u);
}



// Function: xheader_print @ 0x1a980
unsigned long long xheader_print(long long *a1, char *a2, const char *a3)
{
  size_t v4; // rax

  v4 = strlen(a3);
  return sub_1A880(a1, a2, a3, v4);
}



// Function: code_string @ 0x1a9b0
unsigned long long code_string(void *src, char *a2, long long *a3)
{
  char v4; // al
  const char *v5; // rdx
  void *ptr; // [rsp+0h] [rbp-28h] BYREF
  unsigned long long v8; // [rsp+8h] [rbp-20h]

  v8 = __readfsqword(0x28u);
  v4 = sub_31340(1, src, &ptr);
  v5 = (const char *)ptr;
  if ( !v4 )
  {
    ptr = (void *)sub_4C7A0(src);
    v5 = (const char *)ptr;
  }
  sub_1A980(a3, a2, v5);
  free(ptr);
  return v8 - __readfsqword(0x28u);
}



// Function: xattr_selinux_coder @ 0x1aa40
unsigned long long xattr_selinux_coder(long long a1, char *a2, long long *a3)
{
  return sub_1A9B0(*(void **)(a1 + 48), a2, a3);
}



// Function: x_obstack_blank @ 0x1ab10
long long x_obstack_blank(long long *a1, unsigned long long a2)
{
  long long v2; // r12
  long long v3; // rax
  long long result; // rax

  v2 = *a1;
  v3 = *(long long *)(*a1 + 24LL);
  if ( a2 > *(long long *)(*a1 + 32LL) - v3 )
  {
    obstack_newchunk(*a1, a2);
    v3 = *(long long *)(v2 + 24);
  }
  result = a2 + v3;
  *(long long *)(v2 + 24) = result;
  a1[1] += a2;
  return result;
}



// Function: xheader_xattr__add @ 0x1ab50
long long xheader_xattr__add(long long *a1, long long *a2, void *a3, void *a4, long long a5)
{
  long long v9; // rdi
  long long v10; // rax
  long long v11; // rbx
  long long v12; // rax
  long long v13; // r12
  long long result; // rax

  v9 = *a1;
  v10 = *a2 + 1;
  *a2 = v10;
  v11 = 24 * v10 - 24;
  v12 = sub_4C610(v9, 24 * v10);
  *a1 = v12;
  *(long long *)(v12 + v11) = sub_4C7A0(a3);
  v13 = v11 + *a1;
  *(long long *)(v13 + 8) = sub_4C770(a4, a5 + 1);
  result = *a1;
  *(long long *)(*a1 + v11 + 16) = a5;
  return result;
}



// Function: xattr_acls_d_decoder @ 0x1abd0
long long xattr_acls_d_decoder(long long a1, long long a2, void *a3, long long a4)
{
  long long result; // rax

  result = sub_4C770(a3, a4 + 1);
  *(long long *)(a1 + 80) = a4;
  *(long long *)(a1 + 72) = result;
  return result;
}



// Function: xattr_acls_a_decoder @ 0x1ac00
long long xattr_acls_a_decoder(long long a1, long long a2, void *a3, long long a4)
{
  long long result; // rax

  result = sub_4C770(a3, a4 + 1);
  *(long long *)(a1 + 64) = a4;
  *(long long *)(a1 + 56) = result;
  return result;
}



// Function: locate_handler @ 0x1ac30
char **locate_handler(char *s2)
{
  char **v1; // r12
  const char *v2; // rbp
  size_t v3; // rax

  v1 = &off_7F200;
  v2 = "atime";
  do
  {
    if ( *((char *)v1 + 28) )
    {
      v3 = strlen(v2);
      if ( !strncmp(v2, s2, v3) )
        return v1;
    }
    else if ( !strcmp(v2, s2) )
    {
      return v1;
    }
    v2 = v1[4];
    v1 += 4;
  }
  while ( v2 );
  return 0;
}



// Function: run_override_list @ 0x1acb0
void run_override_list(long long **a1, long long a2)
{
  long long **v2; // rbp
  char **v3; // rbx
  const char *v4; // r13
  size_t v5; // rax

  if ( a1 )
  {
    v2 = a1;
    do
    {
      v3 = sub_1AC30((char *)v2[1]);
      if ( v3 )
      {
        v4 = (const char *)v2[2];
        v5 = strlen(v4);
        ((void (*)(long long, char *, const char *, size_t))v3[2])(a2, *v3, v4, v5);
      }
      v2 = (long long **)*v2;
    }
    while ( v2 );
  }
}



// Function: decg @ 0x1ad20
long long decg(long long *a1, char *a2, void *a3, long long a4)
{
  char **v6; // rax

  v6 = sub_1AC30(a2);
  if ( v6 && ((char)v6[3] & 2) != 0 )
    return ((long long (*)(long long *, char *, void *, long long))v6[2])(a1, a2, a3, a4);
  else
    return sub_1A2C0(a1, a2, a3);
}



// Function: out_of_range_header @ 0x1ad90
unsigned long long out_of_range_header(long long a1, long long a2, long long a3, long long a4)
{
  long long v6; // rbp
  long long v7; // r13
  char *v8; // rax
  long long v10; // [rsp+0h] [rbp-68h] BYREF
  char v11[24]; // [rsp+20h] [rbp-48h] BYREF
  unsigned long long v12; // [rsp+38h] [rbp-30h]

  v12 = __readfsqword(0x28u);
  v6 = sub_42F50(a3, &v10);
  v7 = sub_43090(a4, v11);
  if ( qword_84D50 )
    qword_84D50(a4);
  v8 = dcgettext(0, "Extended header %s=%s is out of range %s..%s", 5);
  error(0, 0, v8, a1, a2, v6, v7);
  dword_84D58 = 2;
  return v12 - __readfsqword(0x28u);
}



// Function: decode_time @ 0x1ae40
long long decode_time(long long *a1, char *a2, long long a3)
{
  long long v5; // rdi
  long long v6; // rdx
  long long v7; // rbx
  unsigned int v8; // r12d
  char *v10; // rax
  char *v11[9]; // [rsp+0h] [rbp-48h] BYREF

  v11[1] = (char *)__readfsqword(0x28u);
  v5 = sub_242F0(a2, v11);
  v7 = v6;
  v8 = sub_23BA0(v5, v6);
  if ( (char)v8 )
  {
    *a1 = v5;
    a1[1] = v7;
  }
  else if ( v11[0] <= a2 || *v11[0] )
  {
    if ( qword_84D50 )
      qword_84D50(v5);
    v10 = dcgettext(0, "Malformed extended header: invalid %s=%s", 5);
    error(0, 0, v10, a3, a2);
    dword_84D58 = 2;
  }
  else
  {
    sub_1AD90(a3, (long long)a2, 0x8000000000000000LL, 0x7FFFFFFFFFFFFFFFLL);
  }
  return v8;
}



// Function: mtime_decoder @ 0x1af40
unsigned long long mtime_decoder(long long a1, long long a2, char *a3)
{
  __m128i v4; // [rsp+0h] [rbp-28h] BYREF
  unsigned long long v5; // [rsp+18h] [rbp-10h]

  v5 = __readfsqword(0x28u);
  if ( (unsigned char)sub_1AE40(v4.m128i_i64, a3, a2) )
    *(__m128i *)(a1 + 248) = _mm_load_si128(&v4);
  return v5 - __readfsqword(0x28u);
}



// Function: ctime_decoder @ 0x1afa0
unsigned long long ctime_decoder(long long a1, long long a2, char *a3)
{
  __m128i v4; // [rsp+0h] [rbp-28h] BYREF
  unsigned long long v5; // [rsp+18h] [rbp-10h]

  v5 = __readfsqword(0x28u);
  if ( (unsigned char)sub_1AE40(v4.m128i_i64, a3, a2) )
    *(__m128i *)(a1 + 264) = _mm_load_si128(&v4);
  return v5 - __readfsqword(0x28u);
}



// Function: atime_decoder @ 0x1b000
unsigned long long atime_decoder(long long a1, long long a2, char *a3)
{
  __m128i v4; // [rsp+0h] [rbp-28h] BYREF
  unsigned long long v5; // [rsp+18h] [rbp-10h]

  v5 = __readfsqword(0x28u);
  if ( (unsigned char)sub_1AE40(v4.m128i_i64, a3, a2) )
    *(__m128i *)(a1 + 232) = _mm_load_si128(&v4);
  return v5 - __readfsqword(0x28u);
}



// Function: dumpdir_coder @ 0x1b060
unsigned long long dumpdir_coder(long long a1, char *a2, long long *a3, char *a4)
{
  size_t v6; // rax

  v6 = sub_1F4C0(a4);
  return sub_1A880(a3, a2, a4, v6);
}



// Function: sparse_map_decoder @ 0x1b090
unsigned long long sparse_map_decoder(const char *a1, long long a2, const char *a3)
{
  intmax_t v3; // r12
  const char *v4; // r13
  const char *v5; // rbx
  int v6; // r14d
  int *v7; // r15
  char v8; // r8
  intmax_t v9; // rax
  unsigned long long v10; // rdx
  unsigned long long v11; // rcx
  intmax_t *v12; // rdx
  const char *v13; // rsi
  char *v14; // rax
  char *v16; // rax
  char *v17; // rax
  char v19; // [rsp+Fh] [rbp-49h]
  char *endptr; // [rsp+10h] [rbp-48h] BYREF
  unsigned long long v21; // [rsp+18h] [rbp-40h]

  v4 = a3;
  v21 = __readfsqword(0x28u);
  *((long long *)a1 + 38) = 0;
  if ( (unsigned int)(*a3 - 48) <= 9 )
  {
    v5 = a1;
    v6 = 1;
    v7 = __errno_location();
    while ( 1 )
    {
      *v7 = 0;
      a1 = v4;
      v9 = strtoimax(v4, &endptr, 10);
      if ( v6 )
      {
        if ( *v7 == 34 )
          goto LABEL_18;
        v3 = v9;
      }
      else
      {
        if ( *v7 == 34 )
        {
LABEL_18:
          sub_1AD90(a2, (long long)v4, 0, 0x7FFFFFFFFFFFFFFFLL);
          return v21 - __readfsqword(0x28u);
        }
        v10 = *((long long *)v5 + 38);
        if ( v10 >= *((long long *)v5 + 39) )
        {
          if ( qword_84D50 )
            qword_84D50(v4);
          v13 = "Malformed extended header: excess %s=%s";
          goto LABEL_16;
        }
        v11 = v10 + 1;
        v12 = (intmax_t *)(*((long long *)v5 + 40) + 16 * v10);
        *((long long *)v5 + 38) = v11;
        *v12 = v3;
        v12[1] = v9;
      }
      v8 = *endptr;
      if ( !*endptr )
      {
        if ( v6 )
        {
          if ( qword_84D50 )
            qword_84D50(v4);
          v16 = dcgettext(0, "Malformed extended header: invalid %s: odd number of values", 5);
          error(0, 0, v16, a2);
          dword_84D58 = 2;
        }
        return v21 - __readfsqword(0x28u);
      }
      if ( v8 != 44 )
      {
        if ( qword_84D50 )
        {
          qword_84D50(v4);
          v8 = *endptr;
        }
        v19 = v8;
        v17 = dcgettext(0, "Malformed extended header: invalid %s: unexpected delimiter %c", 5);
        error(0, 0, v17, a2, (unsigned int)v19);
        dword_84D58 = 2;
        return v21 - __readfsqword(0x28u);
      }
      v4 = endptr + 1;
      if ( (unsigned int)(endptr[1] - 48) > 9 )
        break;
      v6 ^= 1u;
    }
  }
  if ( qword_84D50 )
    qword_84D50(a1);
  v13 = "Malformed extended header: invalid %s=%s";
LABEL_16:
  v14 = dcgettext(0, v13, 5);
  error(0, 0, v14, a2, v4);
  dword_84D58 = 2;
  return v21 - __readfsqword(0x28u);
}



// Function: raw_path_decoder @ 0x1b2e0
long long raw_path_decoder(long long a1, long long a2)
{
  long long result; // rax

  sub_1A5E0((void **)a1, a2);
  sub_1A5E0((void **)(a1 + 8), a2);
  result = sub_3E630(*(long long *)(a1 + 8));
  *(char *)(a1 + 16) = result;
  return result;
}



// Function: sparse_path_decoder @ 0x1b310
long long sparse_path_decoder(long long a1, long long a2, long long a3)
{
  *(char *)(a1 + 337) = 1;
  return sub_1B2E0(a1, a3);
}



// Function: code_time @ 0x1b340
unsigned long long code_time(long long a1, long long a2, char *a3, long long *a4)
{
  const char *v6; // rax
  long long v8; // [rsp+0h] [rbp-48h] BYREF
  unsigned long long v9; // [rsp+28h] [rbp-20h]

  v9 = __readfsqword(0x28u);
  v6 = (const char *)sub_24270(a1, a2, &v8);
  sub_1A980(a4, a3, v6);
  return v9 - __readfsqword(0x28u);
}



// Function: mtime_coder @ 0x1b390
unsigned long long mtime_coder(long long a1, char *a2, long long *a3, long long *a4)
{
  if ( a4 )
    return sub_1B340(*a4, a4[1], a2, a3);
  else
    return sub_1B340(*(long long *)(a1 + 248), *(long long *)(a1 + 256), a2, a3);
}



// Function: gid_decoder @ 0x1b530
unsigned long long gid_decoder(long long a1, long long a2, long long a3)
{
  long long v4; // [rsp+0h] [rbp-18h] BYREF
  unsigned long long v5; // [rsp+8h] [rbp-10h]

  v5 = __readfsqword(0x28u);
  if ( (unsigned char)sub_1B440(&v4, a3, 0xFFFFFFFFLL, a2) )
    *(int *)(a1 + 120) = v4;
  return v5 - __readfsqword(0x28u);
}



// Function: uid_decoder @ 0x1b590
unsigned long long uid_decoder(long long a1, long long a2, long long a3)
{
  long long v4; // [rsp+0h] [rbp-18h] BYREF
  unsigned long long v5; // [rsp+8h] [rbp-10h]

  v5 = __readfsqword(0x28u);
  if ( (unsigned char)sub_1B440(&v4, a3, 0xFFFFFFFFLL, a2) )
    *(int *)(a1 + 116) = v4;
  return v5 - __readfsqword(0x28u);
}



// Function: volume_offset_decoder @ 0x1b640
unsigned long long volume_offset_decoder(long long a1, long long a2, long long a3)
{
  long long v4; // [rsp+0h] [rbp-18h] BYREF
  unsigned long long v5; // [rsp+8h] [rbp-10h]

  v5 = __readfsqword(0x28u);
  if ( (unsigned char)sub_1B5F0(&v4, a3, -1, a2) )
    qword_83F00 = v4;
  return v5 - __readfsqword(0x28u);
}



// Function: volume_size_decoder @ 0x1b6a0
unsigned long long volume_size_decoder(long long a1, long long a2, long long a3)
{
  long long v4; // [rsp+0h] [rbp-18h] BYREF
  unsigned long long v5; // [rsp+8h] [rbp-10h]

  v5 = __readfsqword(0x28u);
  if ( (unsigned char)sub_1B5F0(&v4, a3, -1, a2) )
    qword_83F08 = v4;
  return v5 - __readfsqword(0x28u);
}



// Function: sparse_numbytes_decoder @ 0x1b700
unsigned long long sparse_numbytes_decoder(long long *a1, long long a2, long long a3)
{
  unsigned long long v4; // rax
  unsigned long long v5; // rdx
  long long v6; // rax
  char *v8; // rax
  long long v9; // [rsp+0h] [rbp-28h] BYREF
  unsigned long long v10; // [rsp+8h] [rbp-20h]

  v10 = __readfsqword(0x28u);
  if ( (unsigned char)sub_1B5F0(&v9, a3, 0x7FFFFFFFFFFFFFFFLL, a2) )
  {
    v4 = a1[38];
    if ( v4 >= a1[39] )
    {
      if ( qword_84D50 )
        qword_84D50(&v9);
      v8 = dcgettext(0, "Malformed extended header: excess %s=%s", 5);
      error(0, 0, v8, a2, a3);
      dword_84D58 = 2;
    }
    else
    {
      v5 = v4 + 1;
      v6 = a1[40] + 16 * v4;
      a1[38] = v5;
      *(long long *)(v6 + 8) = v9;
    }
  }
  return v10 - __readfsqword(0x28u);
}



// Function: sparse_offset_decoder @ 0x1b7e0
unsigned long long sparse_offset_decoder(long long *a1, long long a2, long long a3)
{
  unsigned long long v4; // rax
  char *v6; // rax
  long long v7; // [rsp+0h] [rbp-28h] BYREF
  unsigned long long v8; // [rsp+8h] [rbp-20h]

  v8 = __readfsqword(0x28u);
  if ( (unsigned char)sub_1B5F0(&v7, a3, 0x7FFFFFFFFFFFFFFFLL, a2) )
  {
    v4 = a1[38];
    if ( v4 >= a1[39] )
    {
      if ( qword_84D50 )
        qword_84D50(&v7);
      v6 = dcgettext(0, "Malformed extended header: excess %s=%s", 5);
      error(0, 0, v6, "GNU.sparse.offset", a3);
      dword_84D58 = 2;
    }
    else
    {
      *(long long *)(a1[40] + 16 * v4) = v7;
    }
  }
  return v8 - __readfsqword(0x28u);
}



// Function: sparse_size_decoder @ 0x1b8b0
unsigned long long sparse_size_decoder(long long a1, long long a2, long long a3)
{
  long long v3; // rax
  long long v5; // [rsp+0h] [rbp-18h] BYREF
  unsigned long long v6; // [rsp+8h] [rbp-10h]

  v6 = __readfsqword(0x28u);
  if ( (unsigned char)sub_1B5F0(&v5, a3, 0x7FFFFFFFFFFFFFFFLL, a2) )
  {
    v3 = v5;
    *(char *)(a1 + 336) = 1;
    *(long long *)(a1 + 328) = v3;
  }
  return v6 - __readfsqword(0x28u);
}



// Function: sparse_minor_decoder @ 0x1b920
unsigned long long sparse_minor_decoder(long long a1, long long a2, long long a3)
{
  long long v4; // [rsp+0h] [rbp-18h] BYREF
  unsigned long long v5; // [rsp+8h] [rbp-10h]

  v5 = __readfsqword(0x28u);
  if ( (unsigned char)sub_1B5F0(&v4, a3, 0xFFFFFFFFLL, a2) )
    *(int *)(a1 + 296) = v4;
  return v5 - __readfsqword(0x28u);
}



// Function: sparse_major_decoder @ 0x1b980
unsigned long long sparse_major_decoder(long long a1, long long a2, long long a3)
{
  long long v4; // [rsp+0h] [rbp-18h] BYREF
  unsigned long long v5; // [rsp+8h] [rbp-10h]

  v5 = __readfsqword(0x28u);
  if ( (unsigned char)sub_1B5F0(&v4, a3, 0xFFFFFFFFLL, a2) )
    *(int *)(a1 + 292) = v4;
  return v5 - __readfsqword(0x28u);
}



// Function: size_decoder @ 0x1b9e0
unsigned long long size_decoder(long long a1, long long a2, long long a3)
{
  long long v4; // [rsp+0h] [rbp-18h] BYREF
  unsigned long long v5; // [rsp+8h] [rbp-10h]

  v5 = __readfsqword(0x28u);
  if ( (unsigned char)sub_1B5F0(&v4, a3, 0x7FFFFFFFFFFFFFFFLL, a2) )
    *(long long *)(a1 + 136) = v4;
  return v5 - __readfsqword(0x28u);
}



// Function: sparse_numblocks_decoder @ 0x1ba40
unsigned long long sparse_numblocks_decoder(long long *a1, long long a2, long long a3)
{
  long long v5; // rdi
  long long v6; // rax
  long long v7; // [rsp+0h] [rbp-18h] BYREF
  unsigned long long v8; // [rsp+8h] [rbp-10h]

  v8 = __readfsqword(0x28u);
  if ( (unsigned char)sub_1B5F0(&v7, a3, -1, a2) )
  {
    v5 = v7;
    a1[39] = v7;
    v6 = sub_4C720(v5, 16);
    a1[38] = 0;
    a1[40] = v6;
  }
  return v8 - __readfsqword(0x28u);
}



// Function: gid_coder @ 0x1bb20
long long gid_coder(long long a1, long long a2, long long a3)
{
  return sub_1BAC0(*(unsigned int *)(a1 + 120), a2, 0xFFFFFFFFLL, a3);
}



// Function: uid_coder @ 0x1bb40
long long uid_coder(long long a1, long long a2, long long a3)
{
  return sub_1BAC0(*(unsigned int *)(a1 + 116), a2, 0xFFFFFFFFLL, a3);
}



// Function: xheader_keyword_deleted_p @ 0x1bef0
long long xheader_keyword_deleted_p(char *name)
{
  long long v1; // rbx

  v1 = qword_84390;
  if ( !qword_84390 )
    return 0;
  while ( fnmatch(*(const char **)(v1 + 8), name, 0) )
  {
    v1 = *(long long *)v1;
    if ( !v1 )
      return 0;
  }
  return 1;
}



// Function: decx @ 0x1bf50
void decx(long long a1, char *a2, long long a3, long long a4)
{
  char **v6; // rax
  char *v7; // rax

  if ( !(unsigned char)sub_1BEF0(a2) && !(unsigned char)sub_1A1A0(a2) )
  {
    v6 = sub_1AC30(a2);
    if ( v6 )
    {
      ((void (*)(long long, char *, long long, long long))v6[2])(a1, a2, a3, a4);
    }
    else if ( (dword_83B80 & 0x20000) != 0 )
    {
      if ( qword_84D50 )
        qword_84D50(a2);
      v7 = dcgettext(0, "Ignoring unknown extended header keyword '%s'", 5);
      error(0, 0, v7, a2);
    }
  }
}



// Function: xheader_set_option @ 0x1c010
char *xheader_set_option(char *a1)
{
  char *result; // rax
  char *v2; // rbp
  char *v3; // rsi

  result = strtok(a1, ",");
  if ( result )
  {
    v2 = result;
    do
    {
      v3 = strchr(v2, 61);
      if ( !v3 )
        sub_1A0F0((long long)v2);
      sub_1A320(v2, v3);
      result = strtok(0, ",");
      v2 = result;
    }
    while ( result );
  }
  return result;
}



// Function: xheader_format_name @ 0x1c080
unsigned long long xheader_format_name(long long *a1, char *a2, long long a3)
{
  char *v3; // r13
  char v4; // al
  char *v6; // rbx
  const char *v7; // r15
  const char *v8; // r14
  void *v9; // rbp
  long long v10; // r12
  char v11; // dl
  char *v12; // r10
  size_t v13; // rax
  long long v14; // rdi
  unsigned long long v15; // r12
  char v16; // al
  char *v17; // rbx
  char v18; // dl
  __pid_t v20; // eax
  char *v21; // rcx
  char *v22; // rax
  size_t v23; // rax
  long long v24; // rax
  char *src; // [rsp+0h] [rbp-A8h]
  char *v26; // [rsp+8h] [rbp-A0h]
  long long *v28; // [rsp+20h] [rbp-88h]
  long long *v29; // [rsp+20h] [rbp-88h]
  long long *v30; // [rsp+20h] [rbp-88h]
  long long *v31; // [rsp+20h] [rbp-88h]
  char *v32; // [rsp+28h] [rbp-80h]
  char v33[20]; // [rsp+30h] [rbp-78h] BYREF
  char v34; // [rsp+44h] [rbp-64h] BYREF
  char v35[20]; // [rsp+50h] [rbp-58h] BYREF
  char v36; // [rsp+64h] [rbp-44h] BYREF
  unsigned long long v37; // [rsp+68h] [rbp-40h]

  v3 = a2;
  v37 = __readfsqword(0x28u);
  v4 = *a2;
  if ( *a2 )
  {
    v6 = a2;
    v7 = 0;
    src = 0;
    v8 = 0;
    v9 = 0;
    v10 = 0;
    v26 = 0;
    while ( 1 )
    {
      v11 = v6[1];
      v12 = v6 + 1;
      if ( v4 != 37 || !v11 )
      {
        ++v10;
        v4 = v6[1];
        goto LABEL_4;
      }
      if ( v11 == 102 )
        break;
      if ( v11 > 102 )
      {
        if ( v11 == 110 )
        {
          v29 = a1;
          v26 = (char *)sub_43090(a3, v35);
          v21 = v26;
          v22 = &v36;
        }
        else
        {
          if ( v11 != 112 )
            goto LABEL_48;
          v29 = a1;
          v20 = getpid();
          src = (char *)sub_43090(v20, v33);
          v21 = src;
          v22 = &v34;
        }
        a1 = v29;
        v12 = v6 + 2;
        v10 += v22 - v21;
        v4 = v6[2];
LABEL_4:
        v6 = v12;
        if ( !v4 )
          goto LABEL_15;
      }
      else
      {
        if ( v11 == 37 )
        {
          v4 = v6[2];
          ++v10;
          v12 = v6 + 2;
          goto LABEL_4;
        }
        if ( v11 != 100 )
        {
LABEL_48:
          v4 = v6[2];
          v10 += 2;
          v12 = v6 + 2;
          goto LABEL_4;
        }
        if ( !a1 )
          goto LABEL_55;
        if ( !v9 )
        {
          v31 = a1;
          v24 = sub_3E4F0(*a1);
          v12 = v6 + 1;
          a1 = v31;
          v9 = (void *)v24;
        }
        v32 = v12;
        v28 = a1;
        v8 = (const char *)sub_33020(v9, 0, (unsigned char)byte_84B92);
        v13 = strlen(v8);
        a1 = v28;
        v10 += v13;
        v4 = v6[2];
        v6 = v32 + 1;
        if ( !v4 )
        {
LABEL_15:
          v14 = v10 + 1;
          goto LABEL_16;
        }
      }
    }
    if ( !a1 )
    {
LABEL_55:
      v4 = v6[2];
      v12 = v6 + 2;
      goto LABEL_4;
    }
    v30 = a1;
    v7 = (const char *)sub_3E390(*a1);
    v23 = strlen(v7);
    a1 = v30;
    v12 = v6 + 2;
    v10 += v23;
    v4 = v6[2];
    goto LABEL_4;
  }
  src = 0;
  v7 = 0;
  v8 = 0;
  v9 = 0;
  v26 = 0;
  v14 = 1;
LABEL_16:
  v15 = sub_4C5B0(v14);
  v16 = *a2;
  if ( !*a2 )
  {
    v17 = (char *)v15;
    free(v9);
    goto LABEL_33;
  }
  v17 = (char *)v15;
  while ( 1 )
  {
    while ( 1 )
    {
      v18 = v3[1];
      if ( v16 == 37 )
        break;
      *v17 = v16;
      ++v3;
      v16 = v18;
      ++v17;
LABEL_19:
      if ( !v16 )
        goto LABEL_29;
    }
    if ( v18 != 102 )
      break;
    if ( v7 )
    {
      v17 = stpcpy(v17, v7);
      goto LABEL_41;
    }
LABEL_27:
    v16 = v3[2];
    v3 += 2;
LABEL_28:
    if ( !v16 )
      goto LABEL_29;
  }
  if ( v18 > 102 )
  {
    if ( v18 == 110 )
    {
      v17 = stpcpy(v17, v26);
      goto LABEL_41;
    }
    if ( v18 == 112 )
    {
      v17 = stpcpy(v17, src);
LABEL_41:
      v16 = v3[2];
      v3 += 2;
      goto LABEL_28;
    }
    *v17 = 37;
LABEL_50:
    v17[1] = v18;
    v16 = v3[2];
    v17 += 2;
    v3 += 2;
    goto LABEL_19;
  }
  if ( v18 == 37 )
  {
    *v17 = 37;
    v16 = v3[2];
    ++v17;
    v3 += 2;
    goto LABEL_19;
  }
  if ( v18 == 100 )
  {
    if ( v8 )
      v17 = stpcpy(v17, v8);
    goto LABEL_27;
  }
  *v17 = 37;
  if ( v18 )
    goto LABEL_50;
  ++v17;
LABEL_29:
  free(v9);
  if ( v15 < (unsigned long long)v17 )
  {
    do
    {
      if ( *(v17 - 1) != 47 )
        break;
      --v17;
    }
    while ( (char *)v15 != v17 );
  }
LABEL_33:
  *v17 = 0;
  return v15;
}



// Function: xheader_xhdr_name @ 0x1c460
unsigned long long xheader_xhdr_name(long long *a1)
{
  if ( !qword_84370 )
    sub_23BE0(&qword_84370, off_7F5E0[(unsigned char)byte_84A38]);
  return sub_1C080(a1, (char *)qword_84370, 0);
}



// Function: xheader_ghdr_name @ 0x1c4b0
unsigned long long xheader_ghdr_name()
{
  char *v0; // r8
  const char *v2; // r12
  char *v3; // rbp
  size_t v4; // rbx
  size_t v5; // rax
  char *v6; // rax

  v0 = (char *)qword_84358;
  if ( !qword_84358 )
  {
    v2 = off_7F5E0[(unsigned char)byte_84A38 + 2];
    v3 = getenv("TMPDIR");
    if ( !v3 )
      v3 = "/tmp";
    v4 = strlen(v3);
    v5 = strlen(v2);
    qword_84358 = sub_4C5B0(v4 + v5 + 1);
    v6 = strcpy((char *)qword_84358, v3);
    v0 = strcat(v6, v2);
  }
  return sub_1C080(0, v0, qword_84398 + 1);
}



// Function: xheader_forbid_global @ 0x1c560
void xheader_forbid_global(long long a1)
{
  char *v1; // rax

  if ( qword_84388 )
  {
    if ( qword_84D50 )
      qword_84D50(a1);
    v1 = dcgettext(0, "can't update global extended header record", 5);
    error(0, 0, v1);
    sub_2C970(2);
  }
}



// Function: xheader_xattr_init @ 0x1c5b0
void xheader_xattr_init(long long *a1)
{
  a1[44] = 0;
  a1[43] = 0;
  a1[7] = 0;
  a1[8] = 0;
  a1[9] = 0;
  a1[10] = 0;
  a1[6] = 0;
}



// Function: xheader_xattr_free @ 0x1c600
void xheader_xattr_free(void **ptr, long long a2)
{
  void **v3; // rbx
  long long v4; // rbp
  void *v5; // rdi

  if ( a2 )
  {
    v3 = ptr;
    v4 = 0;
    do
    {
      v5 = *v3;
      ++v4;
      v3 += 3;
      free(v5);
      free(*(v3 - 2));
    }
    while ( a2 != v4 );
  }
  free(ptr);
}



// Function: xheader_xattr_add @ 0x1c650
void xheader_xattr_add(long long a1, const char *a2, void *a3, long long a4)
{
  size_t v6; // rbp
  size_t v7; // rax
  char *v8; // rbp

  v6 = strlen(a2);
  v7 = strlen("SCHILY.xattr.");
  v8 = (char *)sub_4C5B0(v6 + v7 + 1);
  qmemcpy(v8, "SCHILY.xattr.", 13);
  strcpy(v8 + 13, a2);
  sub_1AB50((long long *)(a1 + 352), (long long *)(a1 + 344), v8, a3, a4);
  free(v8);
}



// Function: xattr_decoder @ 0x1c6e0
unsigned long long xattr_decoder(long long a1, const char *a2, const void *a3, long long a4)
{
  size_t v6; // rax
  long long *v7; // rcx
  signed long long v8; // rdx
  void *v9; // rsp
  unsigned char *v10; // rcx
  signed long long v11; // rdx
  void *v12; // rsp
  size_t v13; // rax
  long long v15; // [rsp+0h] [rbp-2048h]
  long long v16; // [rsp+8h] [rbp-2040h] BYREF
  char v17[4088]; // [rsp+10h] [rbp-2038h] BYREF
  long long v18; // [rsp+1008h] [rbp-1040h] BYREF
  unsigned char v19[7]; // [rsp+1010h] [rbp-1038h] BYREF
  long long v20; // [rsp+2008h] [rbp-40h] BYREF
  unsigned long long v21; // [rsp+2010h] [rbp-38h]

  v21 = __readfsqword(0x28u);
  v6 = strlen(a2);
  v7 = (long long *)((char *)&v20 - ((v6 + 24) & 0xFFFFFFFFFFFFF000LL));
  if ( &v20 != v7 )
  {
    while ( &v18 != v7 )
      ;
  }
  v8 = ((short)v6 + 24) & 0xFF0;
  v9 = alloca(v8);
  if ( (((short)v6 + 24) & 0xFF0) != 0 )
    *(long long *)&v17[v8 + 4080] = *(long long *)&v17[v8 + 4080];
  memcpy(v19, a2, v6 + 1);
  v10 = &v19[-((a4 + 24) & 0xFFFFFFFFFFFFF000LL) - 8];
  if ( &v18 != (long long *)v10 )
  {
    while ( &v16 != (long long *)v10 )
      ;
  }
  v11 = ((short)a4 + 24) & 0xFF0;
  v12 = alloca(v11);
  if ( (((short)a4 + 24) & 0xFF0) != 0 )
    *(long long *)((char *)&v15 + v11) = *(long long *)((char *)&v15 + v11);
  memcpy(v17, a3, a4 + 1);
  sub_19FF0(v19);
  v13 = strlen("SCHILY.xattr.");
  sub_1C650(a1, (const char *)&v19[v13], v17, a4);
  return v21 - __readfsqword(0x28u);
}



// Function: xheader_xattr_copy @ 0x1c840
long long xheader_xattr_copy(long long a1, long long *a2, long long *a3)
{
  unsigned long long v4; // rbx
  long long v5; // rdx
  long long result; // rax

  *a2 = 0;
  *a3 = 0;
  if ( *(long long *)(a1 + 344) )
  {
    v4 = 0;
    do
    {
      v5 = 3 * v4++;
      result = sub_1AB50(
                 a2,
                 a3,
                 *(void **)(*(long long *)(a1 + 352) + 8 * v5),
                 *(void **)(*(long long *)(a1 + 352) + 8 * v5 + 8),
                 *(long long *)(*(long long *)(a1 + 352) + 8 * v5 + 16));
    }
    while ( *(long long *)(a1 + 344) > v4 );
  }
  return result;
}



// Function: xheader_decode @ 0x1c8c0
unsigned long long xheader_decode(long long a1)
{
  long long v2; // rdi
  long long v3; // rsi
  bool v4; // zf
  long long v6; // [rsp+0h] [rbp-28h] BYREF
  unsigned long long v7; // [rsp+8h] [rbp-20h]

  v7 = __readfsqword(0x28u);
  sub_1ACB0((long long **)qword_84388, a1);
  sub_1ACB0((long long **)qword_84378, a1);
  v2 = *(long long *)(a1 + 368);
  if ( v2 )
  {
    v3 = *(long long *)(a1 + 376);
    v6 = v3 + 512;
    while ( (unsigned char)sub_1BC70(v2, v3, &v6, sub_1BF50, a1) )
    {
      v2 = *(long long *)(a1 + 368);
      v3 = *(long long *)(a1 + 376);
    }
  }
  sub_1ACB0((long long **)qword_84380, a1);
  v4 = *(char *)(a1 + 336) == 0;
  *(long long *)(a1 + 280) = *(long long *)(a1 + 136);
  if ( !v4 )
    *(long long *)(a1 + 136) = *(long long *)(a1 + 328);
  return v7 - __readfsqword(0x28u);
}



// Function: xheader_decode_global @ 0x1c9a0
unsigned long long xheader_decode_global(long long a1)
{
  long long v2; // [rsp+0h] [rbp-38h] BYREF
  unsigned long long v3; // [rsp+8h] [rbp-30h]

  v3 = __readfsqword(0x28u);
  if ( *(long long *)(a1 + 8) )
  {
    v2 = *(long long *)(a1 + 16) + 512LL;
    sub_1BC20(a1);
    while ( (unsigned char)sub_1BC70(*(long long *)(a1 + 8), *(long long *)(a1 + 16), &v2, sub_1AD20, &qword_84378) )
      ;
  }
  return v3 - __readfsqword(0x28u);
}



// Function: xheader_store @ 0x1ca30
void xheader_store(char *name, long long a2, long long a3)
{
  char **v4; // rax
  char **v5; // rbx

  if ( !*(long long *)(a2 + 376) )
  {
    v4 = sub_1AC30(name);
    v5 = v4;
    if ( v4 )
    {
      if ( v4[1] && !(unsigned char)sub_1BEF0(name) )
      {
        sub_1A5A0((long long *)(a2 + 360));
        if ( !(unsigned char)sub_1A1A0(name) )
          ((void (*)(long long, char *, long long, long long))v5[1])(a2, name, a2 + 360, a3);
      }
    }
  }
}



// Function: xheader_read @ 0x1cac0
char *xheader_read(long long a1, char *a2, long long a3)
{
  long long v3; // rax
  long long v4; // r14
  long long v6; // r13
  long long v7; // rbp
  char *v9; // rdi
  long long v10; // rax
  char *result; // rax
  long long v12; // rbp
  int *v13; // rcx
  char *v14; // rax

  v3 = 0;
  if ( a3 >= 0 )
    v3 = a3;
  v4 = 0;
  v6 = v3 + 512;
  v7 = v3;
  *(long long *)(a1 + 8) = v3 + 512;
  v9 = (char *)(v3 + 513);
  v10 = sub_4C5B0(v3 + 513);
  *(long long *)(a1 + 16) = v10;
  *(char *)(v10 + v7 + 512) = 0;
  do
  {
    v12 = 512;
    if ( (unsigned long long)v6 <= 0x200 )
      v12 = v6;
    if ( !a2 )
    {
      if ( qword_84D50 )
        qword_84D50(v9);
      v14 = dcgettext(0, "Unexpected EOF in archive", 5);
      error(0, 0, v14);
      sub_163D0();
    }
    v13 = (int *)(v4 + *(long long *)(a1 + 16));
    if ( (unsigned int)v12 < 8 )
    {
      if ( (v12 & 4) != 0 )
      {
        *v13 = *(int *)a2;
        *(int *)((char *)v13 + (unsigned int)v12 - 4) = *(int *)&a2[(unsigned int)v12 - 4];
      }
      else if ( (int)v12 )
      {
        *(char *)v13 = *a2;
        if ( (v12 & 2) != 0 )
          *(short *)((char *)v13 + (unsigned int)v12 - 2) = *(short *)&a2[(unsigned int)v12 - 2];
      }
    }
    else
    {
      *(long long *)v13 = *(long long *)a2;
      *(long long *)((char *)v13 + (unsigned int)v12 - 8) = *(long long *)&a2[(unsigned int)v12 - 8];
      qmemcpy(
        (void *)((unsigned long long)(v13 + 2) & 0xFFFFFFFFFFFFFFF8LL),
        (const void *)(a2 - ((char *)v13 - ((unsigned long long)(v13 + 2) & 0xFFFFFFFFFFFFFFF8LL))),
        8LL * (((unsigned int)v12 + (int)v13 - (((int)v13 + 8) & 0xFFFFFFF8)) >> 3));
    }
    v9 = a2;
    v6 -= v12;
    v4 += v12;
    sub_D1F0((unsigned long long)a2);
    result = (char *)sub_E130();
    a2 = result;
  }
  while ( v6 > 0 );
  return result;
}



// Function: xheader_finish @ 0x1cc10
long long xheader_finish(long long *a1)
{
  long long i; // rbx
  long long result; // rax
  long long v3; // rsi
  long long v4; // rdx
  long long v5; // rdx
  long long v6; // rcx

  for ( i = qword_84380; i; i = *(long long *)i )
    sub_1A9B0(*(void **)(i + 16), *(char **)(i + 8), a1);
  result = *a1;
  v3 = *(long long *)(*a1 + 16LL);
  v4 = *(long long *)(*a1 + 24LL);
  if ( v4 == v3 )
    *(char *)(result + 80) |= 2u;
  v5 = ~*(long long *)(result + 48) & (*(long long *)(result + 48) + v4);
  v6 = *(long long *)(result + 32);
  if ( v5 - *(long long *)(result + 8) <= (unsigned long long)(v6 - *(long long *)(result + 8)) )
    v6 = v5;
  *(long long *)(result + 24) = v6;
  *(long long *)(result + 16) = v6;
  a1[2] = v3;
  return result;
}



// Function: xheader_destroy @ 0x1cca0
void xheader_destroy(long long a1)
{
  struct obstack *v2; // rdi

  v2 = *(struct obstack **)a1;
  if ( v2 )
  {
    obstack_free(v2, 0);
    free(*(void **)a1);
    *(long long *)a1 = 0;
  }
  else
  {
    free(*(void **)(a1 + 16));
  }
  *(long long *)(a1 + 16) = 0;
  *(long long *)(a1 + 8) = 0;
}



// Function: xheader_write @ 0x1ccf0
void xheader_write(char a1, long long a2, long long a3, long long a4)
{
  size_t v5; // r14
  char *v6; // rax
  int v7; // ebp
  size_t v8; // rbx
  long long *v9; // rax
  char *v10; // r15
  char *v11; // r8
  unsigned int v12; // eax
  char *v13; // r14

  v5 = *(long long *)(a4 + 8);
  if ( a1 == 103 )
  {
    if ( qword_84350 )
      a3 = qword_84348;
  }
  else if ( a1 == 120 && qword_84368 )
  {
    a3 = qword_84360;
  }
  v6 = sub_12CB0(a2, *(long long *)(a4 + 8), a3);
  v7 = v5 & 0x1FF;
  v6[156] = a1;
  sub_12F00((unsigned long long)v6);
  v8 = v5 + *(long long *)(a4 + 16);
  while ( 1 )
  {
    v9 = sub_E130();
    v10 = (char *)(v8 - v5);
    if ( v5 == v7 )
      break;
    v5 -= 512LL;
    *v9 = *(long long *)v10;
    v9[63] = *((long long *)v10 + 63);
    qmemcpy(
      (void *)((unsigned long long)(v9 + 1) & 0xFFFFFFFFFFFFFFF8LL),
      (const void *)(v10 - ((char *)v9 - ((unsigned long long)(v9 + 1) & 0xFFFFFFFFFFFFFFF8LL))),
      8LL * (((unsigned int)v9 - (((int)v9 + 8) & 0xFFFFFFF8) + 512) >> 3));
    sub_D1F0((unsigned long long)v9);
    if ( !v5 )
      goto LABEL_13;
  }
  v11 = (char *)memcpy(v9, (const void *)(v8 - v5), v5);
  v12 = 512 - v5;
  v13 = &v11[v5];
  if ( v12 >= 8 )
  {
    *(long long *)v13 = 0;
    *(long long *)&v13[v12 - 8] = 0;
    memset(
      (void *)((unsigned long long)(v13 + 8) & 0xFFFFFFFFFFFFFFF8LL),
      0,
      8LL * ((v12 + (int)v13 - (((int)v13 + 8) & 0xFFFFFFF8)) >> 3));
  }
  else if ( (v12 & 4) != 0 )
  {
    *(int *)v13 = 0;
    *(int *)&v13[v12 - 4] = 0;
  }
  else if ( v12 )
  {
    *v13 = 0;
    if ( (v12 & 2) != 0 )
      *(short *)&v13[v12 - 2] = 0;
  }
  sub_D1F0((unsigned long long)v11);
LABEL_13:
  sub_1CCA0(a4);
  if ( a1 == 103 )
    ++qword_84398;
}



// Function: xheader_write_global @ 0x1ceb0
void xheader_write_global(long long *a1)
{
  long long i; // rbx
  void *v2; // rbp

  if ( qword_84388 )
  {
    sub_1A5A0(a1);
    for ( i = qword_84388; i; i = *(long long *)i )
      sub_1A9B0(*(void **)(i + 16), *(char **)(i + 8), a1);
  }
  if ( *a1 )
  {
    sub_1CC10(a1);
    v2 = (void *)sub_1C4B0();
    sub_1CCF0(103, (long long)v2, stru_84A20.tv_sec, (long long)a1);
    free(v2);
  }
}



// Function: xheader_string_begin @ 0x1cf40
void xheader_string_begin(long long a1)
{
  *(long long *)(a1 + 24) = 0;
}



// Function: xheader_string_add @ 0x1cf50
void xheader_string_add(long long *a1, const char *a2)
{
  size_t v2; // rax

  if ( !a1[2] )
  {
    sub_1A5A0(a1);
    a1[3] += strlen(a2);
    v2 = strlen(a2);
    sub_1A7C0(a1, a2, v2);
  }
}



// Function: xheader_string_end @ 0x1cfa0
long long xheader_string_end(long long *a1, const char *a2)
{
  long long result; // rax
  char *v4; // rbx
  size_t v5; // r15
  long long v6; // r8
  char *v7; // rax
  unsigned long long v8; // rbx
  size_t v9; // rdx
  long long v10; // rdi
  long long v11; // rax
  void *v12; // rbp
  char *v13; // rax
  char *src; // [rsp+8h] [rbp-60h]
  char v15[20]; // [rsp+10h] [rbp-58h] BYREF
  char v16[4]; // [rsp+24h] [rbp-44h] BYREF
  unsigned long long v17; // [rsp+28h] [rbp-40h]

  v17 = __readfsqword(0x28u);
  result = 0;
  if ( !a1[2] )
  {
    v4 = 0;
    sub_1A5A0(a1);
    v5 = a1[3] + strlen(a2) + 3;
    do
    {
      v6 = sub_43090(&v4[v5], v15);
      v7 = v4;
      v4 = &v16[-v6];
    }
    while ( v7 != &v16[-v6] );
    src = (char *)v6;
    v8 = (unsigned long long)&v4[strlen(a2) + 2];
    sub_1AB10(a1, v8);
    sub_1A830(a1, 10);
    v9 = a1[3];
    v10 = *(long long *)(*a1 + 24);
    v11 = ~a1[3];
    v12 = (void *)(v10 + v11 - v8);
    memmove((void *)(v11 + v10), v12, v9);
    v13 = stpcpy((char *)v12, src);
    *v13 = 32;
    *stpcpy(v13 + 1, a2) = 61;
    return 1;
  }
  return result;
}



// Function: hash_directory_meta @ 0x1d0c0
unsigned long long hash_directory_meta(long long a1, unsigned long long a2)
{
  return (*(long long *)(a1 + 32) + *(long long *)(a1 + 24)) % a2;
}



// Function: compare_directory_meta @ 0x1d0e0
bool compare_directory_meta(long long a1, long long a2)
{
  bool result; // al

  result = 0;
  if ( *(long long *)(a1 + 24) == *(long long *)(a2 + 24) )
    return *(long long *)(a1 + 32) == *(long long *)(a2 + 32);
  return result;
}



// Function: dumpdir_next @ 0x1d110
char *dumpdir_next(long long a1)
{
  unsigned long long v1; // rbp
  long long *v2; // rax
  char *v3; // r12
  long long v5; // r12

  v1 = *(long long *)(a1 + 16);
  v2 = *(long long **)a1;
  if ( *(int *)(a1 + 8) )
  {
    v3 = (char *)(v1 + *v2);
    if ( !*v3 )
      return 0;
    *(long long *)(a1 + 16) = v1 + strlen((const char *)(v1 + *v2)) + 1;
    return v3;
  }
  v3 = 0;
  if ( v2[2] <= v1 )
    return v3;
  v5 = *(long long *)(v2[3] + 8 * v1);
  *(long long *)(a1 + 16) = v1 + 1;
  return (char *)(v5 - 1);
}



// Function: free_directory @ 0x1d180
void free_directory(void **ptr)
{
  free(ptr[10]);
  free(ptr[11]);
  free(ptr);
}



// Function: dumpdir_free @ 0x1d1a0
void dumpdir_free(void **ptr)
{
  free(ptr[3]);
  free(ptr);
}



// Function: dirlist_replace_prefix @ 0x1d1c0
size_t dirlist_replace_prefix(const char *a1, const char *a2)
{
  size_t v2; // r14
  size_t result; // rax
  long long *v4; // rbx
  size_t v5; // r13

  v2 = strlen(a1);
  result = strlen(a2);
  v4 = qword_843C0;
  if ( qword_843C0 )
  {
    v5 = result;
    do
    {
      result = sub_23FC0(v4 + 11, a1, v2, a2, v5);
      v4 = (long long *)*v4;
    }
    while ( v4 );
  }
  return result;
}



// Function: dumpdir_first @ 0x1d220
char *dumpdir_first(long long a1, int a2, long long *a3)
{
  long long v4; // rax

  v4 = sub_4C5B0(24);
  *(long long *)v4 = a1;
  *(int *)(v4 + 8) = a2;
  *(long long *)(v4 + 16) = 0;
  *a3 = v4;
  return sub_1D110(v4);
}



// Function: make_directory @ 0x1d260
long long *make_directory(const char *src, long long a2)
{
  size_t v3; // rbx
  long long v4; // rax
  long long *v5; // r12
  size_t v6; // rdi
  void *v7; // rax
  char *v8; // rax

  v3 = strlen(src);
  v4 = sub_4C5B0(96);
  *(long long *)v4 = 0;
  v5 = (long long *)v4;
  *(long long *)(v4 + 48) = 0;
  *(long long *)(v4 + 40) = 0;
  *(long long *)(v4 + 64) = 0;
  *(int *)(v4 + 60) = 0;
  if ( v3 > 1 && src[v3 - 1] == 47 )
    v6 = v3--;
  else
    v6 = v3 + 1;
  v7 = (void *)sub_4C5B0(v6);
  v5[11] = v7;
  v8 = memcpy(v7, src, v3);
  v5[10] = a2;
  v8[v3] = 0;
  v5[9] = 0;
  return v5;
}



// Function: find_directory @ 0x1d310
long long find_directory(const char *src)
{
  long long v1; // r12
  long long v2; // rax
  void **v3; // rbp

  v1 = qword_843B0;
  if ( qword_843B0 )
  {
    v2 = sub_25340((unsigned int)dword_84580, src);
    v3 = (void **)sub_1D260(src, v2);
    v1 = sub_418D0(qword_843B0, v3);
    sub_1D180(v3);
  }
  return v1;
}



// Function: find_directory_meta @ 0x1d370
long long find_directory_meta(void *a1, void *a2)
{
  long long v2; // r12
  void **v4; // rax
  long long v5; // rdi
  void **v6; // rbp

  v2 = qword_843A8;
  if ( qword_843A8 )
  {
    v4 = (void **)sub_1D260("", 0);
    v5 = qword_843A8;
    v4[3] = a1;
    v6 = v4;
    v4[4] = a2;
    v2 = sub_418D0(v5, v4);
    sub_1D180(v6);
  }
  return v2;
}



// Function: attach_directory @ 0x1d3d0
long long *attach_directory(const char *src)
{
  long long v1; // rax
  long long *result; // rax

  v1 = sub_25340((unsigned int)dword_84580, src);
  result = sub_1D260(src, v1);
  if ( qword_843B8 )
    *(long long *)qword_843B8 = result;
  else
    qword_843C0 = result;
  qword_843B8 = (long long)result;
  return result;
}



// Function: compare_dirnames @ 0x1d420
int compare_dirnames(const char **a1, const char **a2)
{
  return strcmp(*a1, *a2);
}



// Function: compare_directory_canonical_names @ 0x1d430
bool compare_directory_canonical_names(long long a1, long long a2)
{
  return strcmp(*(const char **)(a1 + 80), *(const char **)(a2 + 80)) == 0;
}



// Function: dumpdir_locate @ 0x1d450
long long dumpdir_locate(long long a1, long long a2)
{
  long long *v2; // rax
  long long key; // [rsp+8h] [rbp-10h] BYREF

  key = a2;
  if ( a1 && (v2 = bsearch(&key, *(const void **)(a1 + 24), *(long long *)(a1 + 16), 8u, (__compar_fn_t)compar)) != 0 )
    return *v2 - 1LL;
  else
    return 0;
}



// Function: dumpdir_create0 @ 0x1d4a0
const char **dumpdir_create0(char *s, char *a2)
{
  char v3; // r13
  const char *v4; // r15
  size_t v5; // r12
  const char *v6; // rbx
  size_t v7; // r14
  long long v8; // rdi
  long long v9; // r8
  const char **v10; // r14
  long long v11; // rax
  const char *v12; // rbx
  long long *v13; // r13
  int v14; // esi
  long long v15; // r12
  const char *v17; // [rsp+8h] [rbp-50h]
  long long v19; // [rsp+18h] [rbp-40h]

  v3 = *s;
  if ( *s )
  {
    v17 = 0;
    v4 = s;
    v5 = 1;
    v6 = 0;
    do
    {
      v7 = strlen(v4) + 1;
      v5 += v7;
      if ( !a2 || strchr(a2, v3) )
        ++v17;
      v4 += v7;
      ++v6;
      v3 = *v4;
    }
    while ( *v4 );
    v8 = v5 + 32;
    v9 = (long long)(v17 + 1);
  }
  else
  {
    v17 = 0;
    v9 = 1;
    v8 = 33;
    v6 = 0;
    v5 = 1;
  }
  v19 = v9;
  v10 = (const char **)sub_4C5B0(v8);
  *v10 = (const char *)(v10 + 4);
  memcpy(v10 + 4, s, v5);
  v10[1] = v6;
  v10[2] = v17;
  v11 = sub_4C720(v19, 8);
  v12 = *v10;
  v10[3] = (const char *)v11;
  v13 = (long long *)v11;
  v14 = *v12;
  if ( *v12 )
  {
    v15 = 0;
    do
    {
      if ( !a2 || strchr(a2, v14) )
        v13[v15++] = v12 + 1;
      v12 += strlen(v12) + 1;
      v14 = *v12;
    }
    while ( *v12 );
    v13 += v15;
  }
  *v13 = 0;
  return v10;
}



// Function: makedumpdir @ 0x1d600
void makedumpdir(long long a1, const char *a2)
{
  long long v2; // r14
  const char *v3; // rbx
  long long v4; // rax
  const char *v5; // r12
  long long v6; // r15
  size_t v7; // rbp
  size_t v8; // rax
  const char **v9; // r12
  const char **v10; // r15
  long long v11; // r15
  char *v12; // rbx
  int v13; // eax
  const char *v14; // rax
  char v15; // dl
  long long v16; // rax
  const char **v17; // rax
  void *ptr; // [rsp+8h] [rbp-40h]
  char *ptra; // [rsp+8h] [rbp-40h]

  v2 = 0;
  v3 = a2;
  if ( *(int *)(a1 + 56) != 2 )
  {
    v4 = *(long long *)(a1 + 64);
    if ( !v4 )
    {
      v2 = *(long long *)(a1 + 40);
      if ( *a2 )
        goto LABEL_6;
LABEL_20:
      v7 = 0;
      v17 = (const char **)sub_4C720(0, 8);
      ptr = &dword_0 + 1;
      v9 = v17;
      if ( !*a2 )
      {
        qsort(v17, 0, 8u, (__compar_fn_t)compar);
        ptra = (char *)sub_4C5B0(1);
        v12 = ptra;
        goto LABEL_17;
      }
      goto LABEL_9;
    }
    v2 = *(long long *)(v4 + 48);
    if ( !v2 )
      v2 = *(long long *)(v4 + 40);
  }
  if ( !*a2 )
    goto LABEL_20;
LABEL_6:
  v5 = a2;
  v6 = 0;
  v7 = 0;
  do
  {
    ++v7;
    v8 = strlen(v5);
    v5 += v8 + 1;
    v6 += v8 + 2;
  }
  while ( *v5 );
  ptr = (void *)(v6 + 1);
  v9 = (const char **)sub_4C720(v7, 8);
  if ( *a2 )
  {
LABEL_9:
    v10 = v9;
    do
    {
      *v10++ = v3;
      v3 += strlen(v3) + 1;
    }
    while ( *v3 );
  }
  v11 = 0;
  qsort(v9, v7, 8u, (__compar_fn_t)compar);
  ptra = (char *)sub_4C5B0(ptr);
  v12 = ptra;
  if ( v7 )
  {
    do
    {
      if ( sub_1D450(v2, (long long)v9[v11]) )
        v13 = *(long long *)(a1 + 72) == 0 ? 0xFFFFFFD7 : 0;
      else
        v13 = *(long long *)(a1 + 72) == 0 ? 0x10 : 0;
      *v12 = v13 + 73;
      v14 = v9[v11];
      ++v12;
      do
      {
        v15 = *v14++;
        *v12++ = v15;
      }
      while ( v15 );
      ++v11;
    }
    while ( v7 != v11 );
  }
LABEL_17:
  v16 = *(long long *)(a1 + 40);
  *v12 = 0;
  *(long long *)(a1 + 48) = v16;
  *(long long *)(a1 + 40) = sub_1D4A0(ptra, 0);
  free(ptra);
  free(v9);
}



// Function: dumpdir_create @ 0x1d7f0
const char **dumpdir_create(char *a1)
{
  return sub_1D4A0(a1, "YND");
}



// Function: maketagdumpdir @ 0x1d800
void maketagdumpdir(long long a1)
{
  size_t v1; // r12
  char *v2; // rax
  const void *v3; // rsi
  char *v4; // rbp
  long long v5; // rax

  v1 = strlen(*(const char **)(a1 + 72));
  v2 = (char *)sub_4C5B0(v1 + 3);
  v3 = *(const void **)(a1 + 72);
  *v2 = 89;
  v4 = v2;
  memcpy(v2 + 1, v3, v1 + 1);
  v5 = *(long long *)(a1 + 40);
  v4[v1 + 2] = 0;
  *(long long *)(a1 + 48) = v5;
  *(long long *)(a1 + 40) = sub_1D4A0(v4, 0);
  free(v4);
}



// Function: dumpdir_ok @ 0x1d860
long long dumpdir_ok(char *s)
{
  char v1; // bl
  char *v2; // rbp
  unsigned int v3; // r12d
  int v4; // r13d
  long long result; // rax
  char *v6; // rax
  char *v7; // rax
  char *v8; // rax
  const char *v9; // rsi
  char *v10; // rax

  v1 = *s;
  if ( !*s )
    return 1;
  v2 = s;
  v3 = 0;
  v4 = 0;
  while ( 1 )
  {
    if ( !v3 )
    {
      if ( v1 == 84 )
      {
        if ( qword_84D50 )
          qword_84D50(s);
        v9 = "Malformed dumpdir: 'T' not preceded by 'R'";
        goto LABEL_35;
      }
LABEL_11:
      if ( v1 == 88 )
      {
        if ( v4 )
        {
          if ( qword_84D50 )
            qword_84D50(s);
          v9 = "Malformed dumpdir: 'X' duplicated";
          goto LABEL_35;
        }
        v4 = 1;
      }
      else if ( v1 == 82 )
      {
        if ( v2[1] )
        {
          v3 = 84;
        }
        else
        {
          if ( !v4 )
          {
            if ( qword_84D50 )
              qword_84D50(s);
            v9 = "Malformed dumpdir: empty name in 'R'";
LABEL_35:
            v10 = dcgettext(0, v9, 5);
            error(0, 0, v10);
            dword_84D58 = 2;
            return 0;
          }
          v3 = 84;
          v4 = 0;
        }
      }
      goto LABEL_8;
    }
    if ( v1 != v3 )
      break;
    if ( v1 != 84 )
      goto LABEL_11;
    if ( !v2[1] && !v4 )
    {
      if ( qword_84D50 )
        qword_84D50(s);
      v9 = "Malformed dumpdir: empty name in 'T'";
      goto LABEL_35;
    }
    v3 = 0;
LABEL_8:
    s = v2;
    v2 += strlen(v2) + 1;
    v1 = *v2;
    if ( !*v2 )
    {
      if ( v3 )
      {
        if ( qword_84D50 )
          qword_84D50(s);
        v8 = dcgettext(0, "Malformed dumpdir: expected '%c' but found end of data", 5);
        error(0, 0, v8, v3);
        dword_84D58 = 2;
        return 0;
      }
      else
      {
        result = 1;
        if ( v4 && (dword_83B80 & 2) != 0 )
        {
          if ( qword_84D50 )
            qword_84D50(s);
          v6 = dcgettext(0, "Malformed dumpdir: 'X' never used", 5);
          error(0, 0, v6);
          return 1;
        }
      }
      return result;
    }
  }
  if ( qword_84D50 )
    qword_84D50(s);
  v7 = dcgettext(0, "Malformed dumpdir: expected '%c' but found %#3o", 5);
  error(0, 0, v7, v3, (unsigned char)v1);
  dword_84D58 = 2;
  return 0;
}



// Function: note_directory @ 0x1dac0
long long *note_directory(
        const char *a1,
        long long a2,
        long long a3,
        long long a4,
        long long a5,
        char a6,
        char a7,
        char *a8)
{
  const char **v12; // rbp
  long long *v13; // rax
  long long *v14; // r12
  long long v15; // rdi
  long long v16; // rdi

  v12 = (const char **)a8;
  v13 = sub_1D3D0(a1);
  v13[2] = a3;
  v14 = v13;
  v13[1] = a2;
  v13[3] = a4;
  v13[4] = a5;
  *((int *)v13 + 14) = 1;
  if ( a6 )
    *((int *)v13 + 15) |= 2u;
  if ( a7 )
    *((int *)v13 + 15) |= 4u;
  if ( a8 )
    v12 = sub_1D7F0(a8);
  v15 = qword_843B0;
  v14[5] = v12;
  if ( !v15 && (qword_843B0 = sub_41B30(0, 0, sub_1E1A0, sub_1D430, 0), (v15 = qword_843B0) == 0)
    || !sub_42140(v15, v14)
    || (v16 = qword_843A8) == 0 && (qword_843A8 = sub_41B30(0, 0, sub_1D0C0, sub_1D0E0, 0), (v16 = qword_843A8) == 0)
    || !sub_42140(v16, v14) )
  {
    sub_16420();
  }
  return v14;
}



// Function: procdir @ 0x1dbe0
long long *procdir(char *s2, long long a2, char a3, char *a4)
{
  const char *v4; // r12
  long long v7; // r14
  long long v8; // rax
  long long *v9; // r13
  void *v10; // rsi
  long long v11; // rax
  long long v12; // r15
  char *v13; // rax
  char v14; // dl
  int v15; // eax
  long long v17; // rax
  long long v18; // r14
  char *v19; // rax
  void **v20; // rdi
  int v21; // eax
  int v22; // eax
  char *v23; // rax
  long long v24; // rax
  int v25; // ebx
  long long v26; // r14
  char *v27; // rax
  long long v28; // rax
  long long v29; // r13
  long long v30; // r15
  long long v31; // rax
  long long v32; // rdx
  long long *v33; // rax
  char *v34; // r14
  long long v35; // r14
  char *v36; // rax
  long long v37; // rax
  long long v38; // r14
  char *v39; // rax
  int v40; // eax
  long long v41; // rdx
  long long v42; // rdx
  char *v43; // rax
  char *v44; // rax
  long long v45; // [rsp+0h] [rbp-68h]
  long long v46; // [rsp+0h] [rbp-68h]
  long long v47; // [rsp+0h] [rbp-68h]
  __int128 v48; // [rsp+0h] [rbp-68h]
  long long v50; // [rsp+18h] [rbp-50h]
  long long v51; // [rsp+18h] [rbp-50h]
  long long v52[9]; // [rsp+20h] [rbp-48h] BYREF

  v4 = s2;
  v7 = *(long long *)(a2 + 88);
  v52[1] = __readfsqword(0x28u);
  v8 = sub_1D310(s2);
  if ( !v8 )
  {
    v51 = a2 + 88;
    v28 = sub_1D370(*(void **)(a2 + 88), *(void **)(a2 + 96));
    v29 = *(long long *)(a2 + 88);
    v30 = v28;
    v46 = *(long long *)(a2 + 96);
    v31 = sub_4BD20(a2 + 88);
    v33 = sub_1DAC0(s2, v31, v32, v29, v46, v7 < 0, 1, 0);
    v9 = v33;
    if ( v30 )
    {
      v34 = *(char **)(v30 + 88);
      s2 = v34;
      if ( strcmp(v34, v4) )
      {
        if ( (dword_83B80 & 0x2000) != 0 )
        {
          if ( qword_84D50 )
          {
            qword_84D50(v34);
            v34 = *(char **)(v30 + 88);
          }
          v47 = sub_4AC40(1, v34);
          v35 = sub_4AA70(v4);
          v36 = dcgettext(0, "%s: Directory has been renamed from %s", 5);
          error(0, 0, v36, v35, v47);
          v34 = *(char **)(v30 + 88);
        }
        *((int *)v9 + 15) |= 0x10u;
        s2 = v34;
        v9[8] = v30;
        *(int *)(v30 + 60) &= ~0x10u;
        sub_1D1C0(v34, v4);
      }
      *((int *)v9 + 14) = 1;
      v14 = 0;
    }
    else
    {
      *((int *)v33 + 15) |= 8u;
      if ( (dword_83B80 & 0x1000) != 0 )
      {
        if ( qword_84D50 )
          qword_84D50(s2);
        v38 = sub_4AA70(s2);
        v39 = dcgettext(0, "%s: Directory is new", 5);
        s2 = 0;
        error(0, 0, v39, v38);
      }
      if ( qword_84B28
        || (s2 = (char *)sub_4BD20(v51),
            v48 = xmmword_84B00,
            (int)sub_4BE80(s2, v41, xmmword_84B00, *((long long *)&xmmword_84B00 + 1)) < 0)
        || dword_84B8C && (s2 = (char *)sub_4BD10(v51), (int)sub_4BE80(s2, v42, v48, *((long long *)&v48 + 1)) < 0) )
      {
        v40 = 2;
      }
      else
      {
        v40 = 1;
      }
      *((int *)v9 + 14) = v40;
      v14 = 0;
    }
    goto LABEL_27;
  }
  v9 = (long long *)v8;
  if ( (*(char *)(v8 + 60) & 1) != 0 )
  {
    if ( (a3 & 0x20) == 0 )
    {
LABEL_23:
      *a4 = 78;
      return v9;
    }
    sub_23BE0(v8 + 88, s2);
  }
  s2 = (char *)v9[11];
  if ( strcmp(s2, v4) )
    goto LABEL_23;
  v10 = *(void **)(a2 + 96);
  if ( byte_84B20 && ((*((char *)v9 + 60) & 2) == 0 || v7 >= 0) )
  {
    s2 = *(char **)(a2 + 88);
    if ( (char *)v9[3] != s2 )
      goto LABEL_12;
  }
  if ( (void *)v9[4] != v10 )
  {
    s2 = *(char **)(a2 + 88);
LABEL_12:
    v11 = sub_1D370(s2, v10);
    v12 = v11;
    if ( v11 )
    {
      s2 = *(char **)(v11 + 88);
      if ( strcmp(s2, v4) )
      {
        if ( (dword_83B80 & 0x2000) != 0 )
        {
          if ( qword_84D50 )
          {
            qword_84D50(s2);
            s2 = *(char **)(v12 + 88);
          }
          v50 = sub_4AC40(1, s2);
          v45 = sub_4AA70(v4);
          v13 = dcgettext(0, "%s: Directory has been renamed from %s", 5);
          error(0, 0, v13, v45, v50);
          s2 = *(char **)(v12 + 88);
        }
        *((int *)v9 + 15) |= 0x10u;
        v9[8] = v12;
        *(int *)(v12 + 60) &= ~0x10u;
        sub_1D1C0(s2, v4);
      }
      *((int *)v9 + 14) = 1;
      v14 = 0;
    }
    else
    {
      v37 = *(long long *)(a2 + 88);
      *((int *)v9 + 14) = 2;
      v14 = 1;
      v9[3] = v37;
      v9[4] = *(long long *)(a2 + 96);
    }
    v15 = *((int *)v9 + 15);
    if ( v7 < 0 )
      v15 = *((int *)v9 + 15) | 2;
    goto LABEL_26;
  }
  *((int *)v9 + 14) = 1;
  v15 = *((int *)v9 + 15);
  v14 = 0;
LABEL_26:
  *((int *)v9 + 15) = v15 | 4;
LABEL_27:
  if ( byte_84AD1 && (v17 = *(long long *)(a2 + 408)) != 0 && *(long long *)(a2 + 88) != *(long long *)(v17 + 88) )
  {
    if ( (dword_83B80 & 0x40000) != 0 )
    {
      if ( qword_84D50 )
        qword_84D50(s2);
      v18 = sub_4AA70(v9[11]);
      v19 = dcgettext(0, "%s: directory is on a different filesystem; not dumped", 5);
      error(0, 0, v19, v18);
    }
    v20 = (void **)v9[5];
    *((int *)v9 + 14) = 0;
    if ( !v20 )
    {
      *((int *)v9 + 15) |= 1u;
      return v9;
    }
    sub_1D1A0(v20);
    v9[5] = 0;
    v21 = *((int *)v9 + 14);
  }
  else
  {
    if ( (a3 & 0x10) != 0 )
    {
      v25 = a3 & 3;
      *((int *)v9 + 14) = v25;
      if ( !v25 )
        *a4 = 78;
    }
    if ( v14 && (dword_83B80 & 0x2000) != 0 )
    {
      if ( qword_84D50 )
        qword_84D50(s2);
      v26 = sub_4AA70(v4);
      v27 = dcgettext(0, "%s: Directory has been renamed", 5);
      error(0, 0, v27, v26);
    }
    v21 = *((int *)v9 + 14);
  }
  *((int *)v9 + 15) |= 1u;
  if ( v21 )
  {
    v22 = sub_14160(a2, v52);
    switch ( v22 )
    {
      case 2:
        v44 = dcgettext(0, "contents not dumped", 5);
        sub_12AD0((long long)v4, v52[0], (long long)v44);
        v9[9] = v52[0];
        break;
      case 3:
        v43 = dcgettext(0, "directory not dumped", 5);
        sub_12AD0((long long)v4, v52[0], (long long)v43);
        *a4 = 78;
        *((int *)v9 + 14) = 0;
        break;
      case 1:
        v23 = dcgettext(0, "contents not dumped", 5);
        sub_12AD0((long long)v4, v52[0], (long long)v23);
        v24 = v52[0];
        *((int *)v9 + 14) = 0;
        v9[9] = v24;
        break;
    }
  }
  return v9;
}



// Function: hash_directory_canonical_name @ 0x1e1a0
long long hash_directory_canonical_name(long long a1)
{
  return sub_41AC0(*(long long *)(a1 + 80));
}



// Function: obstack_code_rename @ 0x1e1b0
void *obstack_code_rename(long long a1, const char *a2, const char *a3)
{
  const char *v3; // r13
  char *v6; // rax
  size_t v7; // rax
  void *v8; // rdi
  size_t v9; // rbp
  char *v10; // rax
  size_t v11; // rax
  void *v12; // rdi
  size_t v13; // rbp
  void *result; // rax

  v3 = a2;
  if ( *a2 )
  {
    v3 = (const char *)sub_33020(a2, 0, (unsigned char)byte_84B92);
    v6 = *(char **)(a1 + 24);
    if ( *(char **)(a1 + 32) != v6 )
      goto LABEL_3;
  }
  else
  {
    v6 = *(char **)(a1 + 24);
    if ( *(char **)(a1 + 32) != v6 )
      goto LABEL_3;
  }
  obstack_newchunk(a1, 1);
  v6 = *(char **)(a1 + 24);
LABEL_3:
  *(long long *)(a1 + 24) = v6 + 1;
  *v6 = 82;
  v7 = strlen(v3);
  v8 = *(void **)(a1 + 24);
  v9 = v7 + 1;
  if ( v7 + 1 > *(long long *)(a1 + 32) - (long long)v8 )
  {
    obstack_newchunk(a1, v9);
    v8 = *(void **)(a1 + 24);
  }
  memcpy(v8, v3, v9);
  v10 = (char *)(v9 + *(long long *)(a1 + 24));
  *(long long *)(a1 + 24) = v10;
  if ( *a3 )
  {
    a3 = (const char *)sub_33020(a3, 0, (unsigned char)byte_84B92);
    v10 = *(char **)(a1 + 24);
    if ( *(char **)(a1 + 32) != v10 )
      goto LABEL_7;
  }
  else if ( *(char **)(a1 + 32) != v10 )
  {
    goto LABEL_7;
  }
  obstack_newchunk(a1, 1);
  v10 = *(char **)(a1 + 24);
LABEL_7:
  *(long long *)(a1 + 24) = v10 + 1;
  *v10 = 84;
  v11 = strlen(a3);
  v12 = *(void **)(a1 + 24);
  v13 = v11 + 1;
  if ( v11 + 1 > *(long long *)(a1 + 32) - (long long)v12 )
  {
    obstack_newchunk(a1, v13);
    v12 = *(void **)(a1 + 24);
  }
  result = memcpy(v12, a3, v13);
  *(long long *)(a1 + 24) += v13;
  return result;
}



// Function: store_rename @ 0x1e320
void store_rename(long long a1, long long a2)
{
  long long v2; // rbp
  long long v3; // rbx
  long long v4; // rax
  char *v5; // r13
  char *v6; // rax
  size_t v7; // rax
  void *v8; // rdi
  size_t v9; // r14

  v2 = a1;
  v3 = a1;
  while ( 1 )
  {
    if ( !v2 )
    {
      while ( v3 )
      {
        v4 = *(long long *)(v3 + 64);
        if ( !v4 )
          break;
        sub_1E1B0(a2, *(const char **)(v4 + 88), *(const char **)(v3 + 88));
        v3 = *(long long *)(v3 + 64);
      }
      return;
    }
    if ( *(long long *)(v2 + 64) == a1 )
      break;
    v2 = *(long long *)(v2 + 64);
  }
  v5 = (char *)sub_3E4F0(*(long long *)(a1 + 88));
  v6 = *(char **)(a2 + 24);
  if ( *(char **)(a2 + 32) == v6 )
  {
    obstack_newchunk(a2, 1);
    v6 = *(char **)(a2 + 24);
  }
  *(long long *)(a2 + 24) = v6 + 1;
  *v6 = 88;
  v7 = strlen(v5);
  v8 = *(void **)(a2 + 24);
  v9 = v7 + 1;
  if ( v7 + 1 > *(long long *)(a2 + 32) - (long long)v8 )
  {
    obstack_newchunk(a2, v9);
    v8 = *(void **)(a2 + 24);
  }
  memcpy(v8, v5, v9);
  *(long long *)(a2 + 24) += v9;
  sub_1E1B0(a2, *(const char **)(v3 + 88), "");
  for ( ; v2 != v3; v3 = *(long long *)(v3 + 64) )
    sub_1E1B0(a2, *(const char **)(*(long long *)(v3 + 64) + 88LL), *(const char **)(v3 + 88));
  sub_1E1B0(a2, "", *(const char **)(v2 + 88));
  free(v5);
}



// Function: read_incr_db_01 @ 0x1e470
unsigned long long read_incr_db_01(int a1, const char *a2)
{
  __int128 v2; // rax
  long long v3; // rdi
  long long v4; // r12
  long long v5; // rax
  const char *v6; // rbx
  int v7; // eax
  char *v8; // rax
  char *v9; // r12
  long long v10; // rbp
  uintmax_t v11; // r13
  int *v12; // rbx
  long long v13; // rdi
  long long v14; // rdx
  int v15; // eax
  const char *v16; // rsi
  char *v17; // rbp
  const char *v18; // rax
  __int128 v20; // rax
  const char *v21; // rbx
  int *v22; // rax
  int *v23; // rbp
  uintmax_t v24; // rax
  char *v25; // rbx
  const char *v26; // rax
  long long v27; // rbp
  char *v28; // rbx
  const char *v29; // r12
  int *v30; // rax
  char *nptra; // [rsp+0h] [rbp-78h]
  char *nptr; // [rsp+0h] [rbp-78h]
  long long v33; // [rsp+10h] [rbp-68h]
  bool v34; // [rsp+18h] [rbp-60h]
  void *ptr; // [rsp+20h] [rbp-58h] BYREF
  size_t n; // [rsp+28h] [rbp-50h] BYREF
  char *endptr; // [rsp+30h] [rbp-48h] BYREF
  unsigned long long v39; // [rsp+38h] [rbp-40h]

  v39 = __readfsqword(0x28u);
  ptr = 0;
  n = 0;
  if ( a1 != 1 )
  {
    ptr = strdup(a2);
    n = strlen((const char *)ptr) + 1;
    *(long long *)&v2 = sub_242F0((char *)ptr, &endptr);
    v3 = v2;
    xmmword_84B00 = v2;
    v33 = 1;
    if ( (unsigned char)sub_23BA0(v2, *((long long *)&v2 + 1)) )
      goto LABEL_9;
    v27 = 1;
LABEL_40:
    if ( qword_84D50 )
      qword_84D50(v3);
    v28 = dcgettext(0, "Invalid time stamp", 5);
    v29 = (const char *)sub_4AA70(qword_84B28);
    v30 = __errno_location();
    error(0, *v30, "%s:%ld: %s", v29, v27, v28);
    sub_163D0();
  }
  if ( getline((char **)&ptr, &n, qword_843A0) <= 0 )
  {
    sub_32820(qword_84B28);
    free(ptr);
  }
  else
  {
    *(long long *)&v20 = sub_242F0((char *)ptr, &endptr);
    v3 = v20;
    xmmword_84B00 = v20;
    if ( !(unsigned char)sub_23BA0(v20, *((long long *)&v20 + 1)) )
    {
      v27 = 2;
      goto LABEL_40;
    }
    v33 = 2;
    if ( !*endptr )
      goto LABEL_9;
    v21 = endptr + 1;
    v22 = __errno_location();
    *v22 = 0;
    v23 = v22;
    v24 = strtoumax(v21, &endptr, 10);
    if ( !*v23 )
    {
      if ( v24 > 0x3B9AC9FF )
      {
        *v23 = 34;
      }
      else if ( endptr != v21 )
      {
        *((long long *)&xmmword_84B00 + 1) = v24;
        v33 = 2;
        goto LABEL_9;
      }
    }
    if ( qword_84D50 )
      qword_84D50(v21);
    v25 = dcgettext(0, "Invalid time stamp", 5);
    v26 = (const char *)sub_4AA70(qword_84B28);
    error(0, *v23, "%s:%ld: %s", v26, 2, v25);
    dword_84D58 = 2;
    *(long long *)&xmmword_84B00 = 0x8000000000000000LL;
    *((long long *)&xmmword_84B00 + 1) = -1;
    v33 = 2;
LABEL_9:
    while ( 1 )
    {
      v7 = getline((char **)&ptr, &n, qword_843A0);
      if ( v7 <= 0 )
        break;
      v8 = (char *)ptr + v7 - 1;
      v34 = *(char *)ptr == 43;
      ++v33;
      v9 = (char *)ptr + v34;
      if ( *v8 == 10 )
        *v8 = 0;
      v10 = 0;
      v11 = 0;
      v12 = __errno_location();
      if ( a1 == 1 )
      {
        v13 = sub_242F0(v9, &endptr);
        v10 = v13;
        nptr = endptr;
        if ( !(unsigned char)sub_23BA0(v13, v14) || *nptr != 32 )
        {
          if ( qword_84D50 )
            qword_84D50(v13);
          v16 = "Invalid modification time";
          goto LABEL_21;
        }
        *v12 = 0;
        v11 = strtoumax(nptr, &endptr, 10);
        v15 = *v12;
        if ( v11 > 0x3B9AC9FF )
        {
          if ( !v15 )
            *v12 = 34;
LABEL_18:
          if ( qword_84D50 )
            qword_84D50(nptr);
          v16 = "Invalid modification time (nanoseconds)";
LABEL_21:
          v17 = dcgettext(0, v16, 5);
          v18 = (const char *)sub_4AA70(qword_84B28);
          error(0, *v12, "%s:%ld: %s", v18, v33, v17);
          sub_163D0();
        }
        if ( v15 )
          goto LABEL_18;
        v9 = endptr;
        if ( endptr == nptr || *endptr != 32 )
          goto LABEL_18;
      }
      v4 = sub_240A0(v9, &endptr);
      if ( *v12 || *endptr != 32 )
      {
        if ( qword_84D50 )
          qword_84D50(endptr);
        v16 = "Invalid device number";
        goto LABEL_21;
      }
      v5 = sub_240A0(endptr, &endptr);
      if ( *v12 || *endptr != 32 )
      {
        if ( qword_84D50 )
          qword_84D50(endptr);
        v16 = "Invalid inode number";
        goto LABEL_21;
      }
      v6 = endptr + 1;
      nptra = (char *)v5;
      sub_23C70(endptr + 1);
      sub_1DAC0(v6, v10, v11, v4, (long long)nptra, v34, 0, 0);
    }
    free(ptr);
  }
  return v39 - __readfsqword(0x28u);
}



// Function: read_obstack @ 0x1e960
long long read_obstack(FILE *stream, long long a2, long long *a3)
{
  long long v3; // r13
  unsigned int i; // r12d
  char *v6; // rax
  char *v7; // rax

  v3 = 0;
  for ( i = getc_unlocked(stream); i + 1 > 1; i = getc_unlocked(stream) )
  {
    v6 = *(char **)(a2 + 24);
    if ( *(char **)(a2 + 32) == v6 )
    {
      obstack_newchunk(a2, 1);
      v6 = *(char **)(a2 + 24);
    }
    ++v3;
    *(long long *)(a2 + 24) = v6 + 1;
    *v6 = i;
  }
  v7 = *(char **)(a2 + 24);
  if ( *(char **)(a2 + 32) == v7 )
  {
    obstack_newchunk(a2, 1);
    v7 = *(char **)(a2 + 24);
  }
  *(long long *)(a2 + 24) = v7 + 1;
  *v7 = 0;
  *a3 = v3;
  return i;
}



// Function: read_num @ 0x1ea10
long long read_num(FILE *stream, long long a2, long long a3, long long a4, long long *a5)
{
  FILE *v6; // r13
  int v7; // ecx
  bool v8; // bp
  long long i; // rbx
  __off_t v10; // rax
  long long v11; // rbx
  long long v12; // r13
  char *v13; // rdx
  int v14; // ebp
  unsigned int v15; // r8d
  __off_t v17; // rax
  long long v18; // rbx
  long long v19; // r13
  char *v20; // rax
  int v21; // eax
  long long v22; // rdx
  long long v23; // rcx
  char *v24; // rbx
  const char *v25; // rax
  long long v26; // rbp
  long long v27; // r14
  __off_t v28; // rax
  long long v29; // rbx
  long long v30; // r13
  char *v31; // rax
  long long v32; // rcx
  __off_t v33; // rax
  char *v34; // [rsp-10h] [rbp-D8h]
  char *v35; // [rsp-8h] [rbp-D0h]
  int v37; // [rsp+0h] [rbp-C8h]
  char nptr[32]; // [rsp+10h] [rbp-B8h] BYREF
  char v40[32]; // [rsp+30h] [rbp-98h] BYREF
  char v41[32]; // [rsp+50h] [rbp-78h] BYREF
  char v42[24]; // [rsp+70h] [rbp-58h] BYREF
  unsigned long long v43; // [rsp+88h] [rbp-40h]

  v6 = stream;
  v43 = __readfsqword(0x28u);
  v7 = getc_unlocked(stream);
  v8 = v7 == 45;
  for ( i = 0; ; ++i )
  {
    if ( v8 && (int)i == 0 )
    {
      nptr[0] = v7;
      goto LABEL_4;
    }
    if ( (unsigned int)(v7 - 48) > 9 )
      break;
    nptr[i] = v7;
    if ( i == 20 )
    {
      if ( qword_84D50 )
        qword_84D50(stream);
      v10 = ftello(v6);
      v11 = sub_42FF0(v10, v40);
      v12 = sub_4AA70(qword_84B28);
      v35 = nptr;
      v34 = (char *)(&dword_14 + 1);
      v13 = dcgettext(0, "%s: byte %s: %s %.*s... too long", 5);
      goto LABEL_10;
    }
LABEL_4:
    stream = v6;
    v7 = getc_unlocked(v6);
  }
  nptr[(int)i] = 0;
  if ( v7 < 0 )
  {
    v21 = ferror_unlocked(v6);
    v15 = 0;
    if ( v21 )
      sub_329B0(qword_84B28, a2, v22, v23, 0);
    if ( (int)i )
    {
      if ( qword_84D50 )
        qword_84D50(v6);
      v24 = dcgettext(0, "Unexpected EOF in snapshot file", 5);
      v25 = (const char *)sub_4AA70(qword_84B28);
      error(0, 0, "%s: %s", v25, v24);
      sub_163D0();
    }
  }
  else
  {
    if ( v7 )
    {
      if ( qword_84D50 )
      {
        v37 = v7;
        qword_84D50(stream);
        v7 = v37;
      }
      LODWORD(a3) = v7;
      v33 = ftello(v6);
      v11 = sub_42FF0(v33, v40);
      v12 = sub_4AA70(qword_84B28);
      v13 = dcgettext(0, "%s: byte %s: %s %s followed by invalid byte 0x%02x", 5);
      v35 = (char *)(unsigned int)a3;
      v34 = nptr;
LABEL_10:
      error(0, 0, v13, v12, v11, a2, v34, v35, a3);
      sub_163D0();
    }
    *a5 = sub_240A0(nptr, 0);
    v14 = *__errno_location();
    if ( v14 )
    {
      if ( v14 != 34 )
      {
        if ( qword_84D50 )
          qword_84D50(nptr);
        v17 = ftello(v6);
        v18 = sub_42FF0(v17, v40);
        v19 = sub_4AA70(qword_84B28);
        v20 = dcgettext(0, "%s: byte %s: %s %s", 5);
        error(0, v14, v20, v19, v18, a2, nptr, v20, a3);
        sub_163D0();
      }
      if ( qword_84D50 )
        qword_84D50(nptr);
      v26 = sub_43090(a4, v42);
      v27 = sub_42F50(a3, v41);
      v28 = ftello(v6);
      v29 = sub_42FF0(v28, v40);
      v30 = sub_4AA70(qword_84B28);
      v31 = dcgettext(0, "%s: byte %s: (valid range %s..%s)\n\t%s %s", 5);
      error(0, 34, v31, v30, v29, v27, v26, a2, nptr, v32);
      sub_163D0();
    }
    return 1;
  }
  return v15;
}



// Function: read_timespec @ 0x1ed70
unsigned long long read_timespec(FILE *stream, long long *a2)
{
  char *v3; // rbx
  const char *v4; // rax
  long long v5; // [rsp+8h] [rbp-30h] BYREF
  long long v6; // [rsp+10h] [rbp-28h] BYREF
  unsigned long long v7; // [rsp+18h] [rbp-20h]

  v7 = __readfsqword(0x28u);
  if ( !(unsigned char)sub_1EA10(stream, (long long)"sec", 0x8000000000000000LL, 0x7FFFFFFFFFFFFFFFLL, &v5)
    || !(unsigned char)sub_1EA10(stream, (long long)"nsec", 0, 999999999, &v6) )
  {
    if ( qword_84D50 )
      qword_84D50(stream);
    v3 = dcgettext(0, "Unexpected EOF in snapshot file", 5);
    v4 = (const char *)sub_4AA70(qword_84B28);
    error(0, 0, "%s: %s", v4, v3);
    sub_163D0();
  }
  *a2 = v5;
  a2[1] = v6;
  return v7 - __readfsqword(0x28u);
}



// Function: read_incr_db_2 @ 0x1ee50
unsigned long long read_incr_db_2()
{
  FILE *v0; // rdi
  long long v1; // r14
  long long v2; // r15
  char *object_base; // r13
  char *chunk_limit; // rdx
  FILE *v5; // rdi
  char *v6; // r10
  char *v7; // rdx
  char *v9; // rbx
  const char *v10; // rax
  char *v11; // rbp
  __off_t v12; // rax
  long long v13; // rbx
  long long v14; // r12
  char *v15; // rax
  bool v16; // [rsp+Ch] [rbp-ECh]
  struct _obstack_chunk *block; // [rsp+10h] [rbp-E8h]
  long long v18; // [rsp+20h] [rbp-D8h] BYREF
  unsigned long long v19; // [rsp+28h] [rbp-D0h] BYREF
  long long v20[2]; // [rsp+30h] [rbp-C8h] BYREF
  struct obstack obstack; // [rsp+40h] [rbp-B8h] BYREF
  char v22[24]; // [rsp+A0h] [rbp-58h] BYREF
  unsigned long long v23; // [rsp+B8h] [rbp-40h]

  v23 = __readfsqword(0x28u);
  obstack_begin(&obstack, 0, 0, sub_4C5B0, &free);
  sub_1ED70(qword_843A0, &xmmword_84B00);
  while ( (unsigned char)sub_1EA10(qword_843A0, (long long)"nfs", 0, 1, &v18) )
  {
    v16 = v18 != 0;
    sub_1ED70(qword_843A0, v20);
    v0 = qword_843A0;
    if ( !(unsigned char)sub_1EA10(qword_843A0, (long long)"dev", 0, -1, &v18)
      || (v0 = qword_843A0, v1 = v18, !(unsigned char)sub_1EA10(qword_843A0, (long long)"ino", 0, -1, &v18))
      || (v0 = qword_843A0, v2 = v18, (unsigned int)sub_1E960(qword_843A0, (long long)&obstack, &v19)) )
    {
      if ( qword_84D50 )
        qword_84D50(v0);
      v9 = dcgettext(0, "Unexpected EOF in snapshot file", 5);
      v10 = (const char *)sub_4AA70(qword_84B28);
      error(0, 0, "%s: %s", v10, v9);
      sub_163D0();
    }
    object_base = obstack.object_base;
    if ( obstack.next_free == obstack.object_base )
      *((char *)&obstack + 80) |= 2u;
    chunk_limit = obstack.chunk_limit;
    if ( (~*(long long *)&obstack.alignment_mask & (long long)&obstack.next_free[*(long long *)&obstack.alignment_mask])
       - (unsigned long long)obstack.chunk <= obstack.chunk_limit - (char *)obstack.chunk )
      chunk_limit = (char *)(~*(long long *)&obstack.alignment_mask
                           & (long long)&obstack.next_free[*(long long *)&obstack.alignment_mask]);
    obstack.next_free = chunk_limit;
    obstack.object_base = chunk_limit;
    while ( !(unsigned int)sub_1E960(qword_843A0, (long long)&obstack, &v19) && v19 > 1 )
      ;
    v5 = qword_843A0;
    if ( getc_unlocked(qword_843A0) )
    {
      if ( qword_84D50 )
        qword_84D50(v5);
      v11 = dcgettext(0, "Missing record terminator", 5);
      v12 = ftello(qword_843A0);
      v13 = sub_42FF0(v12, v22);
      v14 = sub_4AA70(qword_84B28);
      v15 = dcgettext(0, "%s: byte %s: %s", 5);
      error(0, 0, v15, v14, v13, v11);
      sub_163D0();
    }
    v6 = obstack.object_base;
    if ( obstack.next_free == obstack.object_base )
      *((char *)&obstack + 80) |= 2u;
    v7 = obstack.chunk_limit;
    if ( (~*(long long *)&obstack.alignment_mask & (long long)&obstack.next_free[*(long long *)&obstack.alignment_mask])
       - (unsigned long long)obstack.chunk <= obstack.chunk_limit - (char *)obstack.chunk )
      v7 = (char *)(~*(long long *)&obstack.alignment_mask & (long long)&obstack.next_free[*(long long *)&obstack.alignment_mask]);
    obstack.next_free = v7;
    obstack.object_base = v7;
    block = (struct _obstack_chunk *)v6;
    sub_1DAC0(object_base, v20[0], v20[1], v1, v2, v16, 0, v6);
    if ( obstack.chunk >= block || obstack.chunk_limit <= (char *)block )
    {
      obstack_free(&obstack, block);
    }
    else
    {
      obstack.object_base = (char *)block;
      obstack.next_free = (char *)block;
    }
  }
  return v23 - __readfsqword(0x28u);
}



// Function: write_directory_file_entry @ 0x1f1c0
bool write_directory_file_entry(long long a1, FILE *a2)
{
  int v2; // eax
  const char *v4; // rdi
  const char *v5; // r14
  size_t v6; // rax
  const char *v7; // r14
  size_t v8; // rax
  const char *v9; // r14
  size_t v10; // rax
  const char *v11; // r13
  size_t v12; // rax
  const char *v13; // r13
  size_t v14; // rax
  long long v15; // rdi
  const char *v16; // rbp
  size_t v17; // rax
  void *v18; // r13
  void *ptr; // [rsp+8h] [rbp-50h] BYREF
  char v21[24]; // [rsp+10h] [rbp-48h] BYREF
  unsigned long long v22; // [rsp+28h] [rbp-30h]

  v22 = __readfsqword(0x28u);
  v2 = *(int *)(a1 + 60);
  if ( (v2 & 4) != 0 )
  {
    v4 = "1";
    if ( (v2 & 2) == 0 )
      v4 = "0";
    fwrite_unlocked(v4, 2u, 1u, a2);
    v5 = (const char *)sub_24080(*(long long *)(a1 + 8), 0x8000000000000000LL, 0x7FFFFFFFFFFFFFFFLL, v21);
    v6 = strlen(v5);
    fwrite_unlocked(v5, v6 + 1, 1u, a2);
    v7 = (const char *)sub_42F50(*(long long *)(a1 + 16), v21);
    v8 = strlen(v7);
    fwrite_unlocked(v7, v8 + 1, 1u, a2);
    v9 = (const char *)sub_24080(*(long long *)(a1 + 24), 0, -1, v21);
    v10 = strlen(v9);
    fwrite_unlocked(v9, v10 + 1, 1u, a2);
    v11 = (const char *)sub_24080(*(long long *)(a1 + 32), 0, -1, v21);
    v12 = strlen(v11);
    fwrite_unlocked(v11, v12 + 1, 1u, a2);
    v13 = *(const char **)(a1 + 88);
    v14 = strlen(v13);
    fwrite_unlocked(v13, v14 + 1, 1u, a2);
    v15 = *(long long *)(a1 + 40);
    if ( v15 )
    {
      v16 = sub_1D220(v15, 0, (long long *)&ptr);
      if ( v16 )
      {
        do
        {
          v17 = strlen(v16);
          fwrite_unlocked(v16, v17 + 1, 1u, a2);
          v18 = ptr;
          v16 = sub_1D110((long long)ptr);
        }
        while ( v16 );
      }
      else
      {
        v18 = ptr;
      }
      free(v18);
    }
    fwrite_unlocked(&unk_61EEC, 2u, 1u, a2);
  }
  return ferror_unlocked(a2) == 0;
}



// Function: get_gnu_dumpdir @ 0x1f3c0
long long get_gnu_dumpdir(long long a1)
{
  unsigned long long v1; // rbp
  char *v2; // r12
  char *v3; // rax
  char *v4; // rbx
  void *v5; // r14
  char *v6; // rax
  long long v8; // [rsp+8h] [rbp-40h]

  v1 = *(long long *)(a1 + 136);
  v8 = sub_4C5B0(v1);
  v2 = (char *)v8;
  sub_D1F0(qword_84408);
  sub_CCB0(a1);
  for ( ; v1; v1 -= (unsigned long long)v4 )
  {
    sub_CCF0(v1);
    v5 = sub_E130();
    if ( !v5 )
    {
      if ( qword_84D50 )
        qword_84D50(v1);
      v6 = dcgettext(0, "Unexpected EOF in archive", 5);
      error(1, 0, v6);
      dword_84D58 = 2;
    }
    v3 = sub_D2B0((long long)v5);
    if ( (unsigned long long)v3 > v1 )
      v3 = (char *)v1;
    v4 = v3;
    memcpy(v2, v5, (size_t)v3);
    v2 = &v2[(long long)v4];
    sub_D1F0((unsigned long long)&v4[(long long)v5 - 1]);
  }
  sub_CCD0();
  *(char *)(a1 + 393) = 1;
  *(long long *)(a1 + 400) = v8;
  return v8;
}



// Function: dumpdir_size @ 0x1f4c0
long long dumpdir_size(char *s)
{
  long long v1; // rbp
  const char *v2; // rbx
  size_t v3; // rax

  if ( !*s )
    return 1;
  v1 = 0;
  v2 = s;
  do
  {
    v3 = strlen(v2) + 1;
    v2 += v3;
    v1 += v3;
  }
  while ( *v2 );
  return v1 + 1;
}



// Function: clear_directory_table @ 0x1f510
void clear_directory_table()
{
  void **v0; // rbx
  void **v1; // rdi

  if ( qword_843B0 )
    sub_41C30();
  if ( qword_843A8 )
    sub_41C30();
  v0 = (void **)qword_843C0;
  if ( qword_843C0 )
  {
    do
    {
      v1 = v0;
      v0 = (void **)*v0;
      sub_1D180(v1);
    }
    while ( v0 );
  }
  qword_843B8 = 0;
  qword_843C0 = 0;
}



// Function: rebase_directory @ 0x1f570
long long rebase_directory(long long a1, long long a2, long long a3, long long a4, long long a5)
{
  return sub_23FC0(a1 + 88, a2, a3, a4, a5);
}



// Function: update_parent_directory @ 0x1f580
unsigned long long update_parent_directory(long long a1)
{
  const char *v2; // rdi
  long long v3; // rax
  long long *v4; // rbx
  long long v5; // rdx
  struct stat v7; // [rsp+0h] [rbp-B8h] BYREF
  unsigned long long v8; // [rsp+98h] [rbp-20h]

  v2 = *(const char **)a1;
  v8 = __readfsqword(0x28u);
  v3 = sub_1D310(v2);
  if ( v3 )
  {
    v4 = (long long *)v3;
    if ( fstat(*(int *)(a1 + 424), &v7) )
    {
      sub_24EC0(v4[11]);
    }
    else
    {
      v4[1] = sub_4BD20(&v7);
      v4[2] = v5;
    }
  }
  return v8 - __readfsqword(0x28u);
}



// Function: scan_directory @ 0x1f610
long long *scan_directory(long long a1, double a2)
{
  void *v2; // r15
  long long v3; // rax
  long long v4; // rbx
  char *v5; // r14
  char *v6; // r13
  char v7; // dl
  long long *v8; // r12
  void *v9; // r13
  char *v11; // rbx
  void *v12; // r15
  char *v13; // rax
  char *v14; // r10
  int *v15; // rax
  long long (*v16)(); // rdx
  char *v17; // r10
  int v18; // eax
  char *v19; // r13
  char v20; // dl
  int v21; // eax
  long long v22; // rax
  long long v23; // rdx
  long long v24; // rax
  long long v25; // rdx
  int v26; // eax
  char v27; // r11
  char *fd; // [rsp+0h] [rbp-238h]
  int fda; // [rsp+0h] [rbp-238h]
  char *fdc; // [rsp+0h] [rbp-238h]
  __int128 fdb; // [rsp+0h] [rbp-238h]
  long long v32; // [rsp+18h] [rbp-220h]
  char *v33; // [rsp+28h] [rbp-210h]
  int v34; // [rsp+28h] [rbp-210h]
  char v35; // [rsp+37h] [rbp-201h] BYREF
  void *ptr; // [rsp+38h] [rbp-200h] BYREF
  char v37[88]; // [rsp+40h] [rbp-1F8h] BYREF
  struct stat buf; // [rsp+98h] [rbp-1A0h] BYREF
  long long v39; // [rsp+1D8h] [rbp-60h]
  int v40; // [rsp+1E8h] [rbp-50h]
  unsigned long long v41; // [rsp+1F8h] [rbp-40h]

  v2 = *(void **)a1;
  v41 = __readfsqword(0x28u);
  v3 = sub_13FE0(a1);
  v4 = *(long long *)(a1 + 408);
  v5 = (char *)v3;
  v32 = *(long long *)(a1 + 88);
  if ( v3 )
  {
    sub_16890(a1);
    v6 = (char *)sub_4C7A0(v2);
    sub_23E60(v6);
    v7 = 0;
    if ( !v4 )
      v7 = 32;
    v8 = sub_1DBE0(v6, a1, v7, &v35);
    free(v6);
    v9 = (void *)sub_25040((char *)v2);
    if ( !*((int *)v8 + 14) )
    {
      if ( v8[9] )
        sub_1D800((long long)v8);
      goto LABEL_7;
    }
    sub_1D600((long long)v8, v5);
    v11 = sub_1D220(v8[5], 1, (long long *)&ptr);
    if ( v11 )
    {
      while ( 1 )
      {
        v13 = (char *)sub_250D0(v9, v11 + 1);
        if ( *v11 != 73 )
        {
          fd = v13;
          if ( !(unsigned char)sub_16B00(v13, a1) )
            break;
        }
        *v11 = 78;
LABEL_11:
        v12 = ptr;
        v11 = sub_1D110((long long)ptr);
        if ( !v11 )
          goto LABEL_19;
      }
      v33 = fd;
      fda = *(int *)(a1 + 424);
      sub_2F630(v37);
      v14 = v33;
      if ( fda < 0 )
      {
        v34 = fda;
        fdc = v14;
        v15 = __errno_location();
        v16 = sub_24DD0;
        v17 = fdc;
        *v15 = -v34;
LABEL_16:
        sub_24EF0(v17, 0, v16);
        *v11 = 78;
        goto LABEL_17;
      }
      v18 = fstatat(fda, v11 + 1, &buf, flag);
      v17 = v33;
      if ( v18 )
      {
LABEL_21:
        v16 = (long long (*)())sub_24EC0;
        goto LABEL_16;
      }
      if ( (buf.st_mode & 0xF000) == 0x4000 )
      {
        v21 = sub_140D0(a1, (long long)(v11 + 1), dword_84808);
        v17 = v33;
        if ( v21 < 0 )
        {
          v16 = sub_24DD0;
          goto LABEL_16;
        }
        v40 = v21;
        v26 = fstat(v21, &buf);
        v17 = v33;
        v27 = v26;
        if ( v26 )
          goto LABEL_21;
        if ( (buf.st_mode & 0xF000) == 0x4000 )
        {
          if ( dword_84AD4 )
          {
            if ( *((int *)v8 + 14) == 2 )
              v27 = 18;
          }
          else
          {
            v27 = 16;
          }
          *v11 = 68;
          v39 = a1;
          sub_1DBE0(v33, (long long)v37, v27, v11);
          sub_14200((long long)v37);
          goto LABEL_17;
        }
      }
      if ( !byte_84AD1 || buf.st_dev == v32 )
      {
        if ( *v11 == 89 )
          goto LABEL_17;
        v22 = sub_4BD20(&buf);
        fdb = xmmword_84B00;
        if ( (int)sub_4BE80(v22, v23, xmmword_84B00, *((long long *)&xmmword_84B00 + 1)) >= 0
          || dword_84B8C && (v24 = sub_4BD10(&buf), (int)sub_4BE80(v24, v25, fdb, *((long long *)&fdb + 1)) >= 0) )
        {
          *v11 = 89;
          goto LABEL_17;
        }
      }
      *v11 = 78;
LABEL_17:
      sub_2F6F0(v37, a2);
      goto LABEL_11;
    }
    v12 = ptr;
LABEL_19:
    free(v12);
  }
  else
  {
    sub_32AC0(v2);
    sub_16890(a1);
    v19 = (char *)sub_4C7A0(v2);
    sub_23E60(v19);
    v20 = 0;
    if ( !v4 )
      v20 = 32;
    v8 = sub_1DBE0(v19, a1, v20, &v35);
    free(v19);
    v9 = (void *)sub_25040((char *)v2);
  }
LABEL_7:
  sub_250B0(v9);
  free(v5);
  return v8;
}



// Function: directory_contents @ 0x1fa60
long long directory_contents(long long a1)
{
  long long result; // rax

  if ( !a1 )
    return 0;
  result = *(long long *)(a1 + 40);
  if ( result )
    return *(long long *)result;
  return result;
}



// Function: safe_directory_contents @ 0x1fa90
void *safe_directory_contents(long long a1)
{
  void *result; // rax

  result = (void *)sub_1FA60(a1);
  if ( !result )
    return &unk_61EEF;
  return result;
}



// Function: append_incremental_renames @ 0x1fab0
unsigned long long append_incremental_renames(long long a1)
{
  size_t v2; // r14
  char *v3; // rax
  char *v4; // r13
  long long v5; // rax
  char *next_free; // rdi
  char *v7; // rbx
  char *v8; // rax
  char *object_base; // rdi
  char *chunk_limit; // rdx
  struct obstack v12; // [rsp+0h] [rbp-88h] BYREF
  unsigned long long v13; // [rsp+58h] [rbp-30h]

  v13 = __readfsqword(0x28u);
  if ( qword_843C0 )
  {
    v2 = 0;
    obstack_begin(&v12, 0, 0, sub_4C5B0, &free);
    v3 = (char *)sub_1FA60(a1);
    v4 = v3;
    if ( v3 )
    {
      v5 = sub_1F4C0(v3);
      next_free = v12.next_free;
      v2 = v5 - 1;
      if ( (char *)(v5 - 1) > (char *)(v12.chunk_limit - v12.next_free) )
      {
        obstack_newchunk(&v12, v2);
        next_free = v12.next_free;
      }
      memcpy(next_free, v4, v2);
      v12.next_free += v2;
    }
    v7 = qword_843C0;
    if ( qword_843C0 )
    {
      do
      {
        while ( (v7[60] & 0x10) == 0 )
        {
          v7 = *(char **)v7;
          if ( !v7 )
            goto LABEL_11;
        }
        sub_1E320((long long)v7, (long long)&v12);
        v7 = *(char **)v7;
      }
      while ( v7 );
    }
LABEL_11:
    if ( a1 )
    {
      v8 = v12.next_free;
      if ( v2 != v12.next_free - v12.object_base )
      {
        if ( v12.next_free == v12.chunk_limit )
        {
          obstack_newchunk(&v12, 1);
          v8 = v12.next_free;
        }
        v12.next_free = v8 + 1;
        *v8 = 0;
        sub_1D1A0(*(void ***)(a1 + 40));
        object_base = v12.object_base;
        if ( v12.next_free == v12.object_base )
          *((char *)&v12 + 80) |= 2u;
        chunk_limit = v12.chunk_limit;
        if ( (~*(long long *)&v12.alignment_mask & (long long)&v12.next_free[*(long long *)&v12.alignment_mask])
           - (unsigned long long)v12.chunk <= v12.chunk_limit - (char *)v12.chunk )
          chunk_limit = (char *)(~*(long long *)&v12.alignment_mask
                               & (long long)&v12.next_free[*(long long *)&v12.alignment_mask]);
        v12.next_free = chunk_limit;
        v12.object_base = chunk_limit;
        *(long long *)(a1 + 40) = sub_1D7F0(object_base);
      }
    }
    obstack_free(&v12, 0);
  }
  return v13 - __readfsqword(0x28u);
}



// Function: show_snapshot_field_ranges @ 0x1fc70
unsigned long long show_snapshot_field_ranges()
{
  char **v0; // rbx
  char *v1; // rsi
  long long v2; // rdi
  const char *v3; // r14
  const char *v4; // rax
  long long v6; // [rsp+0h] [rbp-68h] BYREF
  char v7[24]; // [rsp+20h] [rbp-48h] BYREF
  unsigned long long v8; // [rsp+38h] [rbp-30h]

  v0 = &off_7F600;
  v8 = __readfsqword(0x28u);
  __printf_chk(1, "This tar's snapshot file field ranges are\n");
  __printf_chk(1, "   (%-15s => [ %s, %s ]):\n\n", "field name", "min", "max");
  v1 = 0;
  v2 = 1;
  while ( 1 )
  {
    v0 += 3;
    v3 = (const char *)sub_24080(v2, v1, v2, v7);
    v4 = (const char *)sub_24080(*(v0 - 2), *(v0 - 2), *(v0 - 1), &v6);
    __printf_chk(1, "    %-15s => [ %s, %s ],\n", *(v0 - 3), v4, v3);
    if ( !*v0 )
      break;
    v2 = (long long)v0[2];
    v1 = v0[1];
  }
  __printf_chk(1, "\n");
  return v8 - __readfsqword(0x28u);
}



// Function: read_directory_file @ 0x1fd80
unsigned long long read_directory_file()
{
  int v0; // eax
  int v1; // ebp
  char *v2; // rbp
  char *v3; // rdi
  char *v4; // rbx
  char *v5; // rax
  char i; // al
  char *v7; // rax
  uintmax_t v8; // rax
  uintmax_t v9; // r12
  int v11; // edi
  char *v12; // rax
  char *lineptr; // [rsp+8h] [rbp-30h] BYREF
  size_t n; // [rsp+10h] [rbp-28h] BYREF
  unsigned long long v15; // [rsp+18h] [rbp-20h]

  v15 = __readfsqword(0x28u);
  lineptr = 0;
  n = 0;
  v0 = open(qword_84B28, dword_84B24 == 0 ? 578 : 66, 438);
  if ( v0 < 0 )
  {
    sub_327C0(qword_84B28);
    return v15 - __readfsqword(0x28u);
  }
  v1 = v0;
  qword_843A0 = fdopen(v0, "r+");
  if ( !qword_843A0 )
  {
    sub_327C0(qword_84B28);
    close(v1);
    return v15 - __readfsqword(0x28u);
  }
  sub_27B00();
  sub_27B80();
  if ( getline(&lineptr, &n, qword_843A0) > 0 )
  {
    v2 = lineptr;
    v3 = lineptr;
    if ( !strncmp(lineptr, "GNU tar", 7u) )
    {
      v4 = v2 + 8;
      if ( v2[7] != 45 )
      {
        if ( qword_84D50 )
          qword_84D50(v3);
        v5 = dcgettext(0, "Bad incremental file format", 5);
        v3 = (char *)(&dword_0 + 1);
        error(1, 0, v5);
        dword_84D58 = 2;
      }
      for ( i = v2[8]; i != 45; ++v4 )
      {
        if ( !i )
        {
          if ( qword_84D50 )
            qword_84D50(v3);
          v7 = dcgettext(0, "Bad incremental file format", 5);
          v3 = (char *)(&dword_0 + 1);
          error(1, 0, v7);
          dword_84D58 = 2;
        }
        i = v4[1];
      }
      v8 = strtoumax(v4 + 1, 0, 10);
      v9 = v8;
      if ( v8 > 1 )
      {
        if ( v8 == 2 )
        {
          sub_1EE50();
        }
        else
        {
          if ( qword_84D50 )
            qword_84D50(v4 + 1);
          v12 = dcgettext(0, "Unsupported incremental format version: %lu", 5);
          error(1, 0, v12, v9);
          dword_84D58 = 2;
        }
        goto LABEL_18;
      }
      v2 = lineptr;
      v11 = v8;
    }
    else
    {
      v11 = 0;
    }
    sub_1E470(v11, v2);
    if ( !ferror_unlocked(qword_843A0) )
      goto LABEL_19;
    goto LABEL_21;
  }
LABEL_18:
  if ( ferror_unlocked(qword_843A0) )
LABEL_21:
    sub_32820(qword_84B28);
LABEL_19:
  free(lineptr);
  return v15 - __readfsqword(0x28u);
}



// Function: write_directory_file @ 0x20010
unsigned long long write_directory_file()
{
  FILE *v0; // rbp
  int v1; // eax
  double v2; // xmm0_8
  const char *v3; // r12
  size_t v4; // rax
  const char *v5; // r12
  size_t v6; // rax
  char v8[24]; // [rsp+0h] [rbp-38h] BYREF
  unsigned long long v9; // [rsp+18h] [rbp-20h]

  v0 = qword_843A0;
  v9 = __readfsqword(0x28u);
  if ( qword_843A0 )
  {
    if ( fseeko(qword_843A0, 0, 0) )
      sub_32B00(qword_84B28);
    v1 = fileno(v0);
    if ( (unsigned int)sub_2AED0(v1) )
      sub_32D70(qword_84B28);
    v2 = __fprintf_chk(v0, 1, "%s-%s-%d\n", "GNU tar", "1.34", 2);
    v3 = (const char *)sub_42F50(stru_84A20.tv_sec, v8);
    v4 = strlen(v3);
    fwrite_unlocked(v3, v4 + 1, 1u, v0);
    v5 = (const char *)sub_43090(stru_84A20.tv_nsec, v8);
    v6 = strlen(v5);
    fwrite_unlocked(v5, v6 + 1, 1u, v0);
    if ( !ferror_unlocked(v0) && qword_843B0 )
      sub_41A40(qword_843B0, sub_1F1C0, v0, v2);
    if ( ferror_unlocked(v0) )
    {
      sub_32E10(qword_84B28);
      if ( !fclose(v0) )
        return v9 - __readfsqword(0x28u);
    }
    else if ( !fclose(v0) )
    {
      return v9 - __readfsqword(0x28u);
    }
    sub_32680(qword_84B28);
  }
  return v9 - __readfsqword(0x28u);
}



// Function: is_dumpdir @ 0x201b0
long long is_dumpdir(long long a1)
{
  long long result; // rax

  result = *(unsigned char *)(a1 + 392);
  if ( (char)result )
  {
    if ( !*(long long *)(a1 + 400) )
    {
      sub_1F3C0(a1);
      return *(unsigned char *)(a1 + 392);
    }
  }
  return result;
}



// Function: try_purge_directory @ 0x201f0
long long try_purge_directory(void *src)
{
  void *v1; // r12
  unsigned int v2; // r13d
  char *v4; // rbx
  void *v5; // rbp
  char v6; // r15
  size_t v7; // rax
  long long v8; // r12
  char *v9; // r14
  char *v10; // r15
  size_t v11; // r14
  long long v12; // r13
  char *v13; // r12
  int *v14; // rax
  void *v15; // rdi
  void *v16; // rbp
  char *v17; // rbx
  void **v18; // r15
  const char *v19; // rsi
  long long v20; // r13
  char *v21; // rax
  char *v22; // rax
  char v23; // al
  int v24; // r13d
  char *v25; // rax
  long long v26; // r13
  char *v27; // rax
  char *ptr; // [rsp+8h] [rbp-F0h]
  unsigned char v29; // [rsp+10h] [rbp-E8h]
  long long v30; // [rsp+18h] [rbp-E0h]
  long long v31; // [rsp+18h] [rbp-E0h]
  struct stat buf; // [rsp+20h] [rbp-D8h] BYREF
  unsigned long long v33; // [rsp+B8h] [rbp-40h]

  v1 = src;
  v33 = __readfsqword(0x28u);
  if ( !(unsigned char)sub_201B0((long long)&qword_84840) )
    return 0;
  ptr = (char *)sub_253C0(src, 0);
  if ( !ptr )
    return 0;
  v2 = sub_1D860(qword_849D0);
  if ( !(char)v2 )
    return 0;
  v4 = qword_849D0;
  v5 = 0;
  v6 = *qword_849D0;
  if ( *qword_849D0 )
  {
    do
    {
      if ( v6 == 88 )
      {
        v11 = strlen(v4 + 1);
        v5 = (void *)sub_4C610(v5, v11 + 12);
        memcpy(v5, v4 + 1, v11);
        strcpy((char *)v5 + v11, "/tar.XXXXXX");
        if ( !mkdtemp((char *)v5) )
        {
          if ( qword_84D50 )
            qword_84D50(v5);
          v12 = sub_4AC50(v5);
          v13 = dcgettext(0, "Cannot create temporary directory using template %s", 5);
          v14 = __errno_location();
          error(0, *v14, v13, v12);
          dword_84D58 = 2;
LABEL_24:
          free(v5);
          free(ptr);
          return 0;
        }
      }
      else
      {
        v7 = strlen(v4);
        if ( v6 == 82 )
        {
          v8 = (long long)&v4[v7 + 1];
          v9 = v4 + 1;
          v10 = &v4[v7 + 2];
          if ( v4[1] )
            v9 = (char *)sub_33020(v4 + 1, 0, (unsigned char)byte_84B92);
          if ( *(char *)(v8 + 1) )
            v10 = (char *)sub_33020(v10, 0, (unsigned char)byte_84B92);
          if ( *v9 )
          {
            if ( !*v10 )
              v10 = (char *)v5;
          }
          else
          {
            v9 = (char *)v5;
          }
          if ( !(unsigned char)sub_19EC0(v9, v10) )
            goto LABEL_24;
          v4 = (char *)v8;
        }
      }
      v4 += strlen(v4) + 1;
      v6 = *v4;
    }
    while ( *v4 );
    v1 = src;
  }
  v15 = v5;
  v16 = 0;
  free(v15);
  v17 = ptr;
  v18 = (void **)sub_1D7F0(qword_849D0);
  if ( !*ptr )
    goto LABEL_53;
  v29 = v2;
  do
  {
    free(v16);
    v16 = (void *)sub_27BC0(v1, v17);
    if ( (unsigned int)sub_246F0((char *)v16, &buf) )
    {
      if ( *__errno_location() == 2 )
        goto LABEL_34;
      sub_24EC0(v16);
      if ( qword_84D50 )
        qword_84D50(v16);
      v19 = "%s: Not purging directory: unable to stat";
      v20 = sub_4AA70(v16);
      goto LABEL_33;
    }
    v22 = (char *)sub_1D450((long long)v18, (long long)v17);
    if ( v22 )
    {
      v23 = *v22;
      if ( v23 == 68 )
      {
        if ( (buf.st_mode & 0xF000) == 0x4000 )
          goto LABEL_34;
      }
      else if ( v23 != 89 || (buf.st_mode & 0xF000) != 0x4000 )
      {
        goto LABEL_34;
      }
    }
    if ( byte_84AD1 && buf.st_dev != qword_847F0 )
    {
      if ( qword_84D50 )
        qword_84D50(v18);
      v19 = "%s: directory is on a different device: not purging";
      v20 = sub_4AA70(v16);
LABEL_33:
      v21 = dcgettext(0, v19, 5);
      error(0, 0, v21, v20);
      goto LABEL_34;
    }
    if ( !byte_84B40 || (unsigned int)sub_2CA20("delete", v16) )
    {
      if ( dword_84A54 )
      {
        v26 = sub_4AC50(v16);
        v31 = qword_84DD0;
        v27 = dcgettext(0, "%s: Deleting %s\n", 5);
        __fprintf_chk(stream, 1, v27, v31, v26);
      }
      if ( !(unsigned int)sub_25490(v16, 1) )
      {
        v24 = *__errno_location();
        if ( qword_84D50 )
          qword_84D50(v16);
        v30 = sub_4AA70(v16);
        v25 = dcgettext(0, "%s: Cannot remove", 5);
        error(0, v24, v25, v30);
        dword_84D58 = 2;
      }
    }
LABEL_34:
    v17 += strlen(v17) + 1;
  }
  while ( *v17 );
  v2 = v29;
LABEL_53:
  free(v16);
  sub_1D1A0(v18);
  free(ptr);
  return v2;
}



// Function: purge_directory @ 0x206e0
long long purge_directory(void *a1)
{
  long long result; // rax

  result = sub_201F0(a1);
  if ( !(char)result )
    return sub_22BD0();
  return result;
}



// Function: list_dumpdir @ 0x20710
void list_dumpdir(char *a1, long long a2)
{
  char *v2; // rbp
  char *v3; // r15
  int v4; // r12d
  long long v5; // rbx
  long long v6; // rcx
  double v7; // xmm0_8

  v2 = &a1[a2];
  if ( a2 )
  {
    v3 = a1;
    v4 = 0;
    v5 = 3228673;
    do
    {
      while ( 1 )
      {
        v6 = (unsigned int)*v3++;
        if ( (char)v6 )
          break;
        v4 = 0;
        fputc_unlocked(10, stream);
        if ( v3 == v2 )
          return;
      }
      if ( (unsigned char)(v6 - 68) <= 0x15u && _bittest64(&v5, (unsigned int)(v6 - 68)) )
      {
        v7 = __fprintf_chk(stream, 1, "%c", v6);
        if ( !v4 )
          __fprintf_chk(stream, 1, " ", v7);
        v4 = 1;
      }
      else
      {
        fputc_unlocked(v6, stream);
      }
    }
    while ( v3 != v2 );
  }
}



// Function: decode_xform @ 0x207f0
const char *decode_xform(char *s, int *a2)
{
  int v2; // eax
  const char *v3; // r12
  size_t v5; // rax

  v2 = *a2;
  v3 = s;
  if ( *a2 == 2 )
  {
    v3 = (const char *)sub_33020(s, 1, (unsigned char)byte_84B92);
    if ( !qword_84A90 )
      return v3;
  }
  else
  {
    if ( v2 == 4 )
      return v3;
    if ( v2 == 1 )
      v3 = (const char *)sub_33020(s, 0, (unsigned char)byte_84B92);
    if ( !qword_84A90 )
      return v3;
  }
  v5 = sub_27C50(v3);
  if ( v5 == -1 )
    v5 = strlen(v3);
  return &v3[v5];
}



// Function: from_header @ 0x20890
long long from_header(
        long long src,
        size_t n,
        long long a3,
        unsigned long long a4,
        unsigned long long a5,
        int a6,
        int a7)
{
  unsigned char *v9; // rbx
  unsigned char *v11; // r12
  const unsigned short **v12; // rax
  unsigned char *v13; // rcx
  const unsigned short *v14; // rsi
  const unsigned short **v15; // r11
  long long v16; // rdx
  char v17; // al
  unsigned char *v18; // r9
  unsigned char *v19; // r15
  char v20; // r8
  bool v21; // zf
  int v22; // edx
  const char *v24; // rsi
  char *v25; // rax
  char *v26; // rax
  unsigned char *v27; // rcx
  bool v28; // di
  long long v29; // rdx
  long long v30; // rax
  unsigned long long v31; // rdi
  long long v32; // rbx
  long long v33; // rax
  long long v34; // r12
  long long v35; // r14
  char *v36; // rax
  long long v37; // r10
  int v38; // edx
  unsigned int v39; // eax
  char *v40; // rax
  long long v41; // rax
  unsigned char *v42; // rcx
  long long v43; // rdi
  long long v44; // rax
  char *v45; // rax
  long long *v46; // rcx
  signed long long v47; // rax
  void *v48; // rsp
  long long v49; // r12
  char *v50; // rax
  char *v51; // rax
  long long v52; // [rsp+0h] [rbp-1498h]
  long long v53; // [rsp+8h] [rbp-1490h] BYREF
  char v54[7]; // [rsp+10h] [rbp-1488h] BYREF
  long long v55; // [rsp+1008h] [rbp-490h] BYREF
  char *v56; // [rsp+1010h] [rbp-488h]
  const unsigned short **v57; // [rsp+1018h] [rbp-480h]
  unsigned long long v58; // [rsp+1020h] [rbp-478h]
  int v59; // [rsp+102Ch] [rbp-46Ch]
  unsigned long long v60; // [rsp+1030h] [rbp-468h]
  char v61[33]; // [rsp+1038h] [rbp-460h] BYREF
  char v62[31]; // [rsp+1059h] [rbp-43Fh] BYREF
  char v63; // [rsp+1078h] [rbp-420h] BYREF
  char v64[999]; // [rsp+1079h] [rbp-41Fh] BYREF
  unsigned long long v65; // [rsp+1460h] [rbp-38h]

  v9 = (unsigned char *)(src + n);
  v58 = a5;
  LODWORD(v57) = a6;
  v59 = a7;
  v65 = __readfsqword(0x28u);
  if ( src + n == (*(char *)src == 0) + src )
  {
LABEL_19:
    if ( a3 && (char)v59 != 1 )
    {
      if ( qword_84D50 )
        qword_84D50(src);
      v24 = "Blanks in header where numeric %s value expected";
LABEL_24:
      v25 = dcgettext(0, v24, 5);
      error(0, 0, v25, a3);
      dword_84D58 = 2;
    }
    return -1;
  }
  v60 = (*(char *)src == 0) + src;
  v11 = (unsigned char *)src;
  v12 = __ctype_b_loc();
  v13 = (unsigned char *)v60;
  v14 = *v12;
  v15 = v12;
  while ( 1 )
  {
    v16 = *v13;
    v17 = *v13;
    if ( (v14[v16] & 0x2000) == 0 )
      break;
    if ( v9 == ++v13 )
      goto LABEL_19;
  }
  v60 = -(long long)a4;
  LODWORD(a4) = (char)v16 - 48;
  if ( (unsigned int)a4 > 7 )
  {
    if ( !(char)v57 )
    {
      if ( (((char)v16 - 43) & 0xFD) == 0 )
      {
        if ( !(char)v59 && !byte_84450 )
        {
          byte_84450 = 1;
          if ( qword_84D50 )
          {
            v56 = (char *)v13;
            v57 = v15;
            qword_84D50(src);
            v13 = (unsigned char *)v56;
            v15 = v57;
          }
          v56 = (char *)v13;
          v57 = v15;
          v26 = dcgettext(0, "Archive contains obsolescent base-64 headers", 5);
          error(0, 0, v26);
          v13 = (unsigned char *)v56;
          v15 = v57;
          v17 = *v56;
        }
        v27 = v13 + 1;
        v28 = v17 == 45;
        a4 = 0;
        if ( v9 != v27 )
        {
          while ( 1 )
          {
            v29 = *v27;
            v30 = *((char *)&xmmword_84460 + v29);
            if ( (char)v30 > 63 )
            {
              if ( v27 != v9 && (char)v29 && ((*v15)[v29] & 0x2000) == 0 )
                goto LABEL_27;
              goto LABEL_44;
            }
            if ( (a4 & 0x3FFFFFFFFFFFFFFLL) != a4 )
              break;
            ++v27;
            a4 = v30 | (a4 << 6);
            if ( v9 == v27 )
              goto LABEL_44;
          }
          v46 = (long long *)((char *)&v55 - ((n + 24) & 0xFFFFFFFFFFFFF000LL));
          if ( &v55 != v46 )
          {
            while ( &v53 != v46 )
              ;
          }
          v47 = ((short)n + 24) & 0xFF0;
          v48 = alloca(v47);
          if ( (((short)n + 24) & 0xFF0) != 0 )
            *(long long *)((char *)&v52 + v47) = *(long long *)((char *)&v52 + v47);
          memcpy(v54, v11, n);
          v21 = (char)v59 == 1;
          v54[n] = 0;
          if ( !v21 && a3 )
          {
            if ( qword_84D50 )
              qword_84D50(v54);
            v49 = sub_4AC50(v54);
            v50 = dcgettext(0, "Archive signed base-64 string %s is out of %s range", 5);
            error(0, 0, v50, v49, a3);
            dword_84D58 = 2;
            return -1;
          }
          return -1;
        }
        return sub_23B90(a4);
      }
      if ( (char)v16 == 0x80 || (char)v16 == 0xFF )
      {
        v42 = v13 + 1;
        v43 = v16 & 0x40;
        a4 = (int)((v16 & 0x3F) - v43);
        while ( 1 )
        {
          v44 = *v42++;
          a4 = v44 + (a4 << 8);
          if ( v9 == v42 )
            break;
          if ( (((unsigned long long)(unsigned int)-(int)v43 << 50) | a4 & 0xFFFFFFFFFFFFFFLL) != a4 )
          {
            if ( !a3 || (char)v59 == 1 )
              return -1;
            if ( qword_84D50 )
              qword_84D50(v43);
            v24 = "Archive base-256 value is out of %s range";
            goto LABEL_24;
          }
        }
        v28 = (int)v43 != 0;
        if ( (v16 & 0x40) != 0 )
          a4 = -(long long)a4;
LABEL_44:
        if ( !v28 )
          goto LABEL_15;
        if ( v60 >= a4 )
          goto LABEL_75;
        if ( (char)v59 != 1 && a3 )
        {
          v31 = a4;
          v32 = sub_43090(v60, v62);
          v33 = sub_43090(a4, v64);
          *(char *)(v33 - 1) = 45;
          v34 = v33 - 1;
          goto LABEL_49;
        }
        return -1;
      }
    }
    if ( (char)v16 )
      goto LABEL_27;
    a4 = 0;
    return sub_23B90(a4);
  }
  v18 = v13 + 1;
  a4 = (int)a4;
  if ( v9 == v13 + 1 )
  {
    v19 = v9;
    v20 = 0;
LABEL_57:
    if ( v58 >= a4 && !v20 )
      return sub_23B90(a4);
  }
  else
  {
    v19 = v13 + 1;
    v20 = 0;
    while ( 1 )
    {
      src = (unsigned int)(char)*v19;
      v22 = src - 48;
      if ( (unsigned int)(src - 48) > 7 )
        break;
      src = a4 & 0x1FFFFFFFFFFFFFFFLL;
      v21 = (a4 & 0x1FFFFFFFFFFFFFFFLL) == a4;
      a4 = v22 + 8 * a4;
      LOBYTE(src) = !v21;
      ++v19;
      v20 |= !v21;
      if ( v9 == v19 )
        goto LABEL_57;
    }
    if ( v58 >= a4 && !v20 )
    {
      if ( v9 != v19 && *v19 )
      {
LABEL_14:
        if ( (v14[(unsigned char)src] & 0x2000) != 0 )
          goto LABEL_15;
LABEL_27:
        if ( a3 )
        {
LABEL_28:
          if ( !qword_84448 )
          {
            qword_84448 = sub_4A530(0);
            sub_4A590(qword_84448, 8);
          }
          while ( v11 != v9 && !*(v9 - 1) )
            --v9;
          sub_4A650(&v63, 1000, v11, v9 - v11, qword_84448);
          if ( !(char)v59 )
          {
            if ( qword_84D50 )
              qword_84D50(&v63);
            v45 = dcgettext(0, "Archive contains %.*s where numeric %s value expected", 5);
            error(0, 0, v45, 1000, &v63, a3);
            dword_84D58 = 2;
          }
        }
        return -1;
      }
      return sub_23B90(a4);
    }
  }
  if ( v17 <= 49 )
  {
LABEL_76:
    if ( !v20 )
    {
LABEL_77:
      if ( v9 == v19 || (LOBYTE(src) = *v19) == 0 )
      {
LABEL_15:
        if ( v58 < a4 )
        {
          if ( a3 && (char)v59 != 1 )
          {
            v31 = a4;
            v32 = sub_43090(v60, v62);
            v34 = sub_43090(a4, v64);
LABEL_49:
            if ( v60 )
              *(char *)--v32 = 45;
            if ( qword_84D50 )
              qword_84D50(v31);
            v35 = sub_43090(v58, v61);
            v36 = dcgettext(0, "Archive value %s is out of %s range %s..%s", 5);
            error(0, 0, v36, v34, a3, v32, v35);
            dword_84D58 = 2;
          }
          return -1;
        }
        return sub_23B90(a4);
      }
      goto LABEL_14;
    }
    goto LABEL_115;
  }
  if ( !a3 )
  {
    if ( v20 )
      return -1;
    goto LABEL_77;
  }
  v37 = 7 - (char)((v17 - 48) | 4);
  if ( v9 == v18 )
  {
    v19 = v9;
    src = 0;
  }
  else
  {
    src = 0;
    do
    {
      v39 = (char)*v18 - 48;
      if ( v39 > 7 )
      {
        v19 = v18;
        goto LABEL_66;
      }
      v38 = v37;
      LOBYTE(v38) = (v37 & 0x1FFFFFFFFFFFFFFFLL) != v37;
      ++v18;
      src = v38 | (unsigned int)src;
      v37 = (int)(7 - v39) + 8 * v37;
    }
    while ( v9 != v18 );
    v19 = v9;
  }
LABEL_66:
  a4 = v37 + 1;
  v20 = src | (v37 == -1);
  if ( v20 != 1 )
  {
    if ( v60 >= a4 )
    {
      if ( !(char)v59 )
      {
        if ( qword_84D50 )
        {
          v57 = (const unsigned short **)v13;
          v58 = v37;
          v60 = (unsigned long long)v15;
          qword_84D50(src);
          v13 = (unsigned char *)v57;
          v37 = v58;
          v15 = (const unsigned short **)v60;
        }
        v58 = v37;
        v60 = (unsigned long long)v15;
        v57 = (const unsigned short **)v13;
        v40 = dcgettext(0, "Archive octal value %.*s is out of %s range; assuming two's complement", 5);
        error(0, 0, v40, (unsigned int)((int)v19 - (int)v57), v57, a3);
        v15 = (const unsigned short **)v60;
        v37 = v58;
      }
      if ( v9 == v19 || (v41 = *v19, !(char)v41) )
      {
        a4 = ~v37;
      }
      else
      {
        if ( ((*v15)[v41] & 0x2000) == 0 )
          goto LABEL_28;
LABEL_75:
        a4 = -(long long)a4;
      }
      return sub_23B90(a4);
    }
    goto LABEL_76;
  }
LABEL_115:
  if ( !a3 || (char)v59 == 1 )
    return -1;
  if ( qword_84D50 )
  {
    v60 = (unsigned long long)v13;
    qword_84D50(src);
    v13 = (unsigned char *)v60;
  }
  v60 = (unsigned long long)v13;
  v51 = dcgettext(0, "Archive octal value %.*s is out of %s range", 5);
  error(0, 0, v51, (unsigned int)((int)v19 - v60), v60, a3);
  dword_84D58 = 2;
  return -1;
}



// Function: base64_init @ 0x21160
long long base64_init()
{
  long long result; // rax
  long long v1; // rdx

  result = 0;
  v1 = 65;
  xmmword_84460 = (__int128)_mm_load_si128((const __m128i *)&xmmword_62560);
  xmmword_84470 = xmmword_84460;
  xmmword_84480 = xmmword_84460;
  xmmword_84490 = xmmword_84460;
  xmmword_844A0 = xmmword_84460;
  xmmword_844B0 = xmmword_84460;
  xmmword_844C0 = xmmword_84460;
  xmmword_844D0 = xmmword_84460;
  xmmword_844E0 = xmmword_84460;
  xmmword_844F0 = xmmword_84460;
  xmmword_84500 = xmmword_84460;
  xmmword_84510 = xmmword_84460;
  xmmword_84520 = xmmword_84460;
  xmmword_84530 = xmmword_84460;
  xmmword_84540 = xmmword_84460;
  xmmword_84550 = xmmword_84460;
  while ( 1 )
  {
    *((char *)&xmmword_84460 + v1) = result++;
    if ( result == 64 )
      break;
    v1 = aAbcdefghijklmn[result];
  }
  return result;
}



// Function: transform_stat_info @ 0x21440
long long transform_stat_info(int a1, long long a2)
{
  long long result; // rax

  if ( a1 != 86 )
  {
    result = sub_21420(a2 + 8, 1);
    if ( a1 == 49 )
    {
      result = sub_21420(a2 + 24, 2);
      if ( byte_84AD0 )
        return sub_21360();
    }
    else
    {
      if ( a1 != 50 )
      {
        if ( !byte_84AD0 )
          return result;
        return sub_21360();
      }
      result = sub_21420(a2 + 24, 4);
      if ( byte_84AD0 )
        return sub_21360();
    }
  }
  return result;
}



// Function: tar_checksum @ 0x214d0
long long tar_checksum(unsigned char *a1, unsigned char a2)
{
  unsigned char *v4; // rdx
  int v5; // esi
  unsigned char *v6; // r8
  int v7; // ecx
  int v8; // edi
  unsigned char *v9; // rdx
  int v10; // edi
  int v11; // ebx
  int v12; // eax
  unsigned int v13; // r8d

  v4 = a1;
  v5 = 0;
  v6 = a1 + 512;
  v7 = 0;
  do
  {
    v8 = *v4++;
    v7 += v8;
    v5 += (char)v8;
  }
  while ( v6 != v4 );
  if ( !v7 )
    return 3;
  v9 = a1 + 155;
  do
  {
    v10 = *v9--;
    v7 -= v10;
    v5 -= (char)v10;
  }
  while ( a1 + 147 != v9 );
  v11 = v7 + 256;
  v12 = sub_20890((long long)(a1 + 148), 8u, 0, 0, 0x7FFFFFFFu, 1, a2);
  if ( v12 < 0 )
    return 5;
  if ( v11 == v12 )
    return 1;
  v13 = 5;
  if ( v5 + 256 == v12 )
    return 1;
  return v13;
}



// Function: decode_header @ 0x215c0
unsigned long long decode_header(long long a1, long long a2, int *a3, int a4)
{
  int v7; // r13d
  int v8; // r12d
  long long v9; // rax
  long long v10; // rsi
  bool v11; // zf
  long long v12; // rsi
  __m128i v13; // xmm0
  unsigned long long v14; // rax
  int v15; // eax
  __m128i v17; // xmm0
  unsigned int v18; // r12d
  unsigned int v19; // eax
  long long v20; // rax
  long long v21; // rax
  char v22; // [rsp+7h] [rbp-41h] BYREF
  unsigned long long v23; // [rsp+8h] [rbp-40h]

  v23 = __readfsqword(0x28u);
  v7 = sub_21210(a1 + 100, &v22);
  if ( !strcmp((const char *)(a1 + 257), "ustar") )
  {
    if ( *(char *)(a1 + 475)
      || (unsigned char)(*(char *)(a1 + 476) - 48) > 7u
      || *(char *)(a1 + 487) != 32
      || (unsigned char)(*(char *)(a1 + 488) - 48) > 7u
      || (v8 = 5, *(char *)(a1 + 499) != 32) )
    {
      v8 = 3 - ((*(long long *)(a2 + 368) == 0) - 1);
    }
  }
  else
  {
    v8 = 1;
    if ( !strcmp((const char *)(a1 + 257), "ustar  ") )
      v8 = v22 == 0 ? 6 : 2;
  }
  *a3 = v8;
  *(int *)(a2 + 112) = v7;
  v9 = sub_21320(a1 + 136);
  v10 = 0;
  v11 = *(char *)(a1 + 265) == 0;
  *(long long *)(a2 + 256) = 0;
  *(long long *)(a2 + 248) = v9;
  if ( !v11 )
    v10 = a1 + 265;
  sub_23C20(a2 + 32, v10, 32);
  v12 = 0;
  if ( *(char *)(a1 + 297) )
    v12 = a1 + 297;
  sub_23C20(a2 + 40, v12, 32);
  sub_1C5B0((long long *)a2);
  if ( v8 == 2 )
  {
    if ( byte_84B50 )
    {
      *(long long *)(a2 + 232) = sub_21320(a1 + 345);
      v21 = sub_21320(a1 + 357);
      *(long long *)(a2 + 272) = 0;
      *(long long *)(a2 + 264) = v21;
      *(long long *)(a2 + 240) = 0;
    }
    else
    {
      v17 = _mm_loadu_si128((const __m128i *)&stru_84A20);
      *(__m128i *)(a2 + 264) = v17;
      *(__m128i *)(a2 + 232) = v17;
    }
  }
  else if ( v8 == 5 )
  {
    *(long long *)(a2 + 232) = sub_21320(a1 + 476);
    v20 = sub_21320(a1 + 488);
    *(long long *)(a2 + 272) = 0;
    *(long long *)(a2 + 264) = v20;
    *(long long *)(a2 + 240) = 0;
  }
  else
  {
    v13 = _mm_loadu_si128((const __m128i *)&stru_84A20);
    *(__m128i *)(a2 + 264) = v13;
    *(__m128i *)(a2 + 232) = v13;
    if ( v8 == 1 )
    {
      *(int *)(a2 + 116) = sub_21260(a1 + 108);
      *(int *)(a2 + 120) = sub_21290(a1 + 116);
      v14 = 0;
      goto LABEL_20;
    }
  }
  if ( a4 )
  {
    if ( byte_84AD2 || !*(char *)(a1 + 265) || !(unsigned int)sub_269F0((char *)(a1 + 265)) )
      *(int *)(a2 + 116) = sub_21260(a1 + 108);
    if ( byte_84AD2 || !*(char *)(a1 + 297) || !(unsigned int)sub_26AA0((char *)(a1 + 297)) )
      *(int *)(a2 + 120) = sub_21290(a1 + 116);
  }
  v14 = 0;
  if ( (unsigned int)(*(char *)(a1 + 156) - 51) <= 1 )
  {
    v18 = sub_212C0(a1 + 337);
    v19 = sub_212F0(a1 + 329);
    v14 = gnu_dev_makedev(v19, v18);
  }
LABEL_20:
  *(long long *)(a2 + 128) = v14;
  sub_1C8C0(a2);
  if ( (unsigned char)sub_29E10(a2) )
  {
    sub_29E70(a2);
    *(char *)(a2 + 288) = 1;
  }
  else
  {
    v15 = dword_84400;
    *(char *)(a2 + 288) = 0;
    if ( (v15 & 0xFFFFFFFB) == 2 && *(char *)(qword_84408 + 156) == 68 || *(long long *)(a2 + 400) )
      *(char *)(a2 + 392) = 1;
  }
  return v23 - __readfsqword(0x28u);
}



// Function: off_from_header @ 0x219b0
long long off_from_header(long long a1, size_t a2)
{
  return sub_20890(a1, a2, (long long)"off_t", 0, 0x7FFFFFFFFFFFFFFFuLL, 0, 0);
}



// Function: read_header @ 0x219e0
long long read_header(unsigned char **a1, long long a2, int a3)
{
  unsigned char *v5; // rax
  unsigned long long v6; // r15
  unsigned int v7; // ebx
  char *v8; // rbp
  char v9; // al
  void *v10; // rdi
  char *v11; // rsi
  long long v13; // rax
  char v14; // dl
  long long v15; // rax
  long long *v16; // rax
  long long v17; // rax
  unsigned long long v18; // rbx
  long long v19; // rbp
  long long v20; // rcx
  void *v21; // rdi
  char *v22; // rbp
  long long v23; // rcx
  const void *v24; // rsi
  unsigned long long v25; // rdi
  char *v26; // rbx
  char *v27; // rax
  char *v28; // r15
  void *v29; // rax
  const void *v30; // r13
  char *v31; // rax
  int v32; // r9d
  __m128i *v33; // rax
  int v34; // edx
  __m128i v35; // xmm2
  int v36; // eax
  __m128i v37; // xmm3
  __m128i v38; // xmm4
  __m128i v39; // xmm5
  __m128i v40; // xmm6
  __m128i v41; // xmm7
  __m128i v42; // xmm1
  long long v43; // rdx
  __m128i v44; // xmm2
  __m128i v45; // xmm3
  __m128i v46; // xmm4
  __m128i v47; // xmm5
  __m128i v48; // xmm6
  char v49; // al
  __m128i v50; // xmm7
  __m128i v51; // xmm1
  __m128i v52; // xmm2
  size_t v53; // rax
  char *v55; // [rsp+10h] [rbp-188h]
  char *ptr; // [rsp+18h] [rbp-180h]
  long long v57; // [rsp+20h] [rbp-178h]
  long long v58; // [rsp+28h] [rbp-170h]
  _OWORD v59[2]; // [rsp+30h] [rbp-168h] BYREF
  char s[16]; // [rsp+50h] [rbp-148h] BYREF
  __m128i v61; // [rsp+60h] [rbp-138h]
  __m128i v62; // [rsp+70h] [rbp-128h]
  __m128i v63; // [rsp+80h] [rbp-118h]
  __m128i v64; // [rsp+90h] [rbp-108h]
  __m128i v65; // [rsp+A0h] [rbp-F8h]
  __m128i v66; // [rsp+B0h] [rbp-E8h]
  __m128i v67; // [rsp+C0h] [rbp-D8h]
  __m128i v68; // [rsp+D0h] [rbp-C8h]
  long long v69; // [rsp+E0h] [rbp-B8h]
  short v70; // [rsp+E8h] [rbp-B0h]
  char v71; // [rsp+EAh] [rbp-AEh]
  char v72; // [rsp+EBh] [rbp-ADh]
  unsigned long long v73; // [rsp+158h] [rbp-40h]

  v73 = __readfsqword(0x28u);
  v57 = 0;
  v58 = 0;
  v55 = 0;
  ptr = 0;
LABEL_2:
  v5 = (unsigned char *)sub_E130();
  *a1 = v5;
  v6 = (unsigned long long)v5;
  if ( v5 )
  {
    while ( 1 )
    {
      v7 = sub_214D0(v5, 0);
      if ( v7 != 1 )
        break;
      if ( *(char *)(v6 + 156) == 49 )
      {
        *(long long *)(a2 + 136) = 0;
LABEL_6:
        free(qword_843F8);
        v8 = ptr + 512;
        if ( !ptr )
        {
          if ( *(char *)(v6 + 345) )
          {
            v8 = s;
            v32 = strcmp((const char *)(v6 + 257), "ustar");
            v33 = (__m128i *)s;
            if ( !v32 )
            {
              v42 = _mm_loadu_si128((const __m128i *)(v6 + 345));
              v43 = *(long long *)(v6 + 489);
              v72 = 0;
              v44 = _mm_loadu_si128((const __m128i *)(v6 + 361));
              v45 = _mm_loadu_si128((const __m128i *)(v6 + 377));
              v46 = _mm_loadu_si128((const __m128i *)(v6 + 393));
              *(__m128i *)s = v42;
              v47 = _mm_loadu_si128((const __m128i *)(v6 + 409));
              v48 = _mm_loadu_si128((const __m128i *)(v6 + 425));
              v49 = *(char *)(v6 + 499);
              v69 = v43;
              v50 = _mm_loadu_si128((const __m128i *)(v6 + 441));
              LOWORD(v43) = *(short *)(v6 + 497);
              v61 = v44;
              v51 = _mm_loadu_si128((const __m128i *)(v6 + 457));
              v71 = v49;
              v52 = _mm_loadu_si128((const __m128i *)(v6 + 473));
              v70 = v43;
              v62 = v45;
              v63 = v46;
              v64 = v47;
              v65 = v48;
              v66 = v50;
              v67 = v51;
              v68 = v52;
              v53 = strlen(s);
              s[v53] = 47;
              v33 = (__m128i *)&s[v53 + 1];
            }
          }
          else
          {
            v8 = s;
            v33 = (__m128i *)s;
          }
          v58 = 0;
          *v33 = _mm_loadu_si128((const __m128i *)v6);
          v33[1] = _mm_loadu_si128((const __m128i *)(v6 + 16));
          v33[2] = _mm_loadu_si128((const __m128i *)(v6 + 32));
          v33[3] = _mm_loadu_si128((const __m128i *)(v6 + 48));
          v33[4] = _mm_loadu_si128((const __m128i *)(v6 + 64));
          v33[5] = _mm_loadu_si128((const __m128i *)(v6 + 80));
          v34 = *(int *)(v6 + 96);
          v33[6].m128i_i8[4] = 0;
          v33[6].m128i_i32[0] = v34;
        }
        qword_843F8 = ptr;
        qword_843E8 = v58;
        sub_23BE0(a2, v8);
        sub_23BE0(a2 + 8, v8);
        v9 = sub_3E630(*(long long *)(a2 + 8));
        v10 = qword_843F0;
        *(char *)(a2 + 16) = v9;
        free(v10);
        if ( v55 )
        {
          v11 = v55 + 512;
        }
        else
        {
          v35 = _mm_loadu_si128((const __m128i *)(v6 + 157));
          v36 = *(int *)(v6 + 253);
          v66.m128i_i8[4] = 0;
          v11 = s;
          v37 = _mm_loadu_si128((const __m128i *)(v6 + 173));
          v38 = _mm_loadu_si128((const __m128i *)(v6 + 189));
          v57 = 0;
          v39 = _mm_loadu_si128((const __m128i *)(v6 + 205));
          v66.m128i_i32[0] = v36;
          v40 = _mm_loadu_si128((const __m128i *)(v6 + 221));
          *(__m128i *)s = v35;
          v41 = _mm_loadu_si128((const __m128i *)(v6 + 237));
          v61 = v37;
          v62 = v38;
          v63 = v39;
          v64 = v40;
          v65 = v41;
        }
        qword_843F0 = v55;
        qword_843E0 = v57;
        sub_23BE0(a2 + 24, v11);
        v55 = 0;
        ptr = 0;
        break;
      }
      v13 = sub_219B0(v6 + 124, 0xCu);
      *(long long *)(a2 + 136) = v13;
      if ( v13 < 0 )
      {
        v7 = 5;
        break;
      }
      v14 = *(char *)(v6 + 156);
      if ( (unsigned char)(v14 - 75) <= 1u || v14 == 120 )
      {
        if ( a3 == 1 )
          goto LABEL_25;
        if ( (unsigned char)(v14 - 75) <= 1u )
        {
          v18 = v13 + 512;
          if ( (v13 & 0x1FF) != 0 )
          {
            v18 = v13 + 1024 - (v13 & 0x1FF);
            if ( v13 > v18 )
              sub_16420();
          }
          v19 = sub_4C5B0(v18 + 1);
          if ( *(char *)(v6 + 156) == 76 )
          {
            free(ptr);
            v58 = v18 >> 9;
            ptr = (char *)v19;
          }
          else
          {
            free(v55);
            v57 = v18 >> 9;
            v55 = (char *)v19;
          }
          sub_D1F0(v6);
          v20 = v19;
          v21 = (void *)((v19 + 8) & 0xFFFFFFFFFFFFFFF8LL);
          v22 = (char *)(v19 + 512);
          v23 = v20 - (long long)v21;
          *((long long *)v22 - 64) = *(long long *)v6;
          v24 = (const void *)(v6 - v23);
          *((long long *)v22 - 1) = *(long long *)(v6 + 504);
          LODWORD(v23) = (unsigned int)(v23 + 512) >> 3;
          qmemcpy(v21, v24, 8LL * (unsigned int)v23);
          v25 = (unsigned long long)v21 + 8 * (unsigned int)v23;
          v26 = (char *)(v18 - 512);
          if ( v26 )
          {
            while ( 1 )
            {
              v29 = sub_E130();
              v30 = v29;
              if ( !v29 )
                break;
              v27 = sub_D2B0((long long)v29);
              if ( v26 <= v27 )
                v27 = v26;
              v28 = v27;
              memcpy(v22, v30, (size_t)v27);
              v25 = (unsigned long long)&v28[(long long)v30 - 1];
              v22 = &v22[(long long)v28];
              sub_D1F0(v25);
              v26 -= (long long)v28;
              if ( !v26 )
                goto LABEL_39;
            }
            if ( qword_84D50 )
              qword_84D50(v25);
            v31 = dcgettext(0, "Unexpected EOF in archive", 5);
            error(0, 0, v31);
            dword_84D58 = 2;
          }
LABEL_39:
          *v22 = 0;
          goto LABEL_2;
        }
      }
      else
      {
        if ( v14 != 103 && v14 != 88 )
          goto LABEL_6;
        if ( a3 == 1 )
          goto LABEL_25;
        if ( v14 != 88 )
        {
          v16 = (long long *)qword_84560;
          if ( !qword_84560 )
          {
            v16 = (long long *)sub_4C5B0(512);
            qword_84560 = (long long)v16;
          }
          *v16 = *(long long *)v6;
          v16[63] = *(long long *)(v6 + 504);
          qmemcpy(
            (void *)((unsigned long long)(v16 + 1) & 0xFFFFFFFFFFFFFFF8LL),
            (const void *)(v6 - ((long long)v16 - ((unsigned long long)(v16 + 1) & 0xFFFFFFFFFFFFFFF8LL))),
            8LL * (((unsigned int)v16 - (((int)v16 + 8) & 0xFFFFFFF8) + 512) >> 3));
          memset(v59, 0, sizeof(v59));
          v17 = sub_219B0(v6 + 124, 0xCu);
          sub_1CAC0((long long)v59, (char *)v6, v17);
          sub_1C9A0((long long)v59);
          sub_1CCA0((long long)v59);
          if ( a3 == 2 )
          {
LABEL_25:
            v7 = 2;
            break;
          }
          goto LABEL_2;
        }
      }
      v15 = sub_219B0(v6 + 124, 0xCu);
      sub_1CAC0(a2 + 360, (char *)v6, v15);
      v5 = (unsigned char *)sub_E130();
      *a1 = v5;
      v6 = (unsigned long long)v5;
      if ( !v5 )
        goto LABEL_17;
    }
  }
  else
  {
LABEL_17:
    v7 = 4;
  }
  free(ptr);
  free(v55);
  return v7;
}



// Function: uintmax_from_header @ 0x22050
long long uintmax_from_header(long long a1, size_t a2)
{
  return sub_20890(a1, a2, (long long)"uintmax_t", 0, 0xFFFFFFFFFFFFFFFFLL, 0, 0);
}



// Function: tartime @ 0x22080
char *tartime(time_t a1, unsigned int a2, char a3)
{
  unsigned int v3; // ebp
  struct tm *v5; // rcx
  char *result; // rax
  char *v7; // rax
  char *v8; // rcx
  char *i; // rdx
  long long v10; // rdx
  size_t v11; // rax
  char *v12; // [rsp+8h] [rbp-30h]
  time_t timer; // [rsp+10h] [rbp-28h] BYREF
  unsigned long long v14; // [rsp+18h] [rbp-20h]

  v3 = a2;
  v14 = __readfsqword(0x28u);
  timer = a1;
  if ( a2 && a1 < 0 )
  {
    timer = a1 + 1;
    v3 = 1000000000 - a2;
  }
  if ( byte_84B91 )
  {
    v5 = gmtime(&timer);
    if ( v5 )
    {
LABEL_6:
      if ( a3 )
      {
        strftime(byte_84420, 0x25u, "%Y-%m-%d %H:%M:%S", v5);
        v11 = strlen(byte_84420);
        sub_24170(v3, &byte_84420[v11]);
      }
      else
      {
        strftime(byte_84420, 0x25u, "%Y-%m-%d %H:%M", v5);
      }
      return byte_84420;
    }
  }
  else
  {
    v5 = localtime(&timer);
    if ( v5 )
      goto LABEL_6;
  }
  if ( a1 < 0 )
  {
    v7 = (char *)(sub_43090(-timer, &unk_84426) - 1);
    *v7 = 45;
  }
  else
  {
    v7 = (char *)sub_43090(timer, &unk_84426);
  }
  v8 = (char *)&unk_84441;
  if ( !a3 )
    v8 = (char *)&unk_84441 - 13;
  for ( i = v7; i > v8; --i )
    *(i - 1) = 32;
  v10 = v8 - v7;
  if ( v8 > v7 )
    v10 = 0;
  result = &v7[v10];
  if ( a3 )
  {
    v12 = result;
    sub_24170(v3, &unk_8443A);
    return v12;
  }
  return result;
}



// Function: simple_print_header @ 0x22220
unsigned long long simple_print_header(FILE *a1, long long a2, long long a3)
{
  FILE *v3; // rbp
  char *IO_read_ptr; // r15
  long long v6; // r12
  char *v7; // rax
  int v8; // eax
  const char *IO_write_base; // r12
  const char *IO_write_ptr; // r14
  long long v11; // rax
  int v12; // esi
  int v13; // eax
  const char *v14; // rax
  double v15; // xmm0_8
  unsigned int v16; // eax
  long long v17; // rax
  unsigned int v18; // eax
  long long v19; // rax
  FILE *v20; // r12
  const char *v21; // rax
  const char *v23; // rsi
  long long v24; // r12
  char *v25; // rax
  char *v26; // rax
  const char *v27; // rsi
  char *v28; // rax
  long long v29; // rax
  long long v30; // rax
  char *v31; // rax
  const char *v32; // rax
  char *v33; // [rsp+8h] [rbp-110h]
  int v34; // [rsp+10h] [rbp-108h]
  int v35; // [rsp+20h] [rbp-F8h]
  char v36[2]; // [rsp+32h] [rbp-E6h] BYREF
  char v37; // [rsp+34h] [rbp-E4h] BYREF
  char v38[9]; // [rsp+35h] [rbp-E3h] BYREF
  char v39[2]; // [rsp+3Eh] [rbp-DAh] BYREF
  char v40[32]; // [rsp+40h] [rbp-D8h] BYREF
  char v41[32]; // [rsp+60h] [rbp-B8h] BYREF
  char v42[32]; // [rsp+80h] [rbp-98h] BYREF
  char s[56]; // [rsp+A0h] [rbp-78h] BYREF
  unsigned long long v44; // [rsp+D8h] [rbp-40h]

  v3 = a1;
  v44 = __readfsqword(0x28u);
  if ( byte_847E1 )
  {
    IO_read_ptr = a1->_IO_read_ptr;
    if ( !IO_read_ptr )
      IO_read_ptr = *(char **)&a1->_flags;
  }
  else
  {
    IO_read_ptr = *(char **)&a1->_flags;
    if ( !*(long long *)&a1->_flags )
      IO_read_ptr = a1->_IO_read_ptr;
  }
  if ( byte_84B7C )
  {
    if ( a3 < 0 )
      a3 = sub_D180(a1, a2, a3);
    v6 = sub_43090(a3 - qword_843E0 - qword_843E8, s);
    v7 = dcgettext(0, "block %s: ", 5);
    a1 = stream;
    __fprintf_chk(stream, 1, v7, v6);
  }
  if ( dword_84A54 <= 1 )
  {
    v20 = stream;
    v21 = (const char *)sub_4A8A0(IO_read_ptr);
    fputs_unlocked(v21, v20);
    if ( byte_847E1 && LOBYTE(v3->_IO_read_end) )
      fputc_unlocked(47, stream);
    fputc_unlocked(10, stream);
  }
  else
  {
    v37 = 63;
    switch ( *(char *)(a2 + 156) )
    {
      case 0:
      case 0x30:
      case 0x53:
        v37 = LOBYTE(v3->_IO_read_end) == 0 ? 45 : 100;
        break;
      case 0x31:
        v37 = 104;
        break;
      case 0x32:
        v37 = 108;
        break;
      case 0x33:
        v37 = 99;
        break;
      case 0x34:
        v37 = 98;
        break;
      case 0x35:
      case 0x44:
        v37 = 100;
        break;
      case 0x36:
        v37 = 112;
        break;
      case 0x37:
        v37 = 67;
        break;
      case 0x4B:
      case 0x4C:
        v37 = 76;
        if ( qword_84D50 )
          qword_84D50(a1);
        v26 = dcgettext(0, "Unexpected long name header", 5);
        error(0, 0, v26);
        dword_84D58 = 2;
        break;
      case 0x4D:
        v37 = 77;
        break;
      case 0x56:
        byte_84451 = 1;
        v37 = 86;
        break;
      default:
        break;
    }
    sub_32310((unsigned int)v3->_fileno, v38);
    sub_32010(v3, v39);
    v33 = sub_22080((time_t)v3[1]._IO_write_base, (unsigned int)v3[1]._IO_write_ptr, byte_84B90);
    v8 = strlen(v33);
    if ( dword_81228 < v8 )
      dword_81228 = v8;
    IO_write_base = v3->_IO_write_base;
    if ( !IO_write_base || !*IO_write_base || dword_84400 == 1 || byte_84AD2 )
      IO_write_base = (const char *)sub_43090((unsigned int)v3->_flags2, v40);
    IO_write_ptr = v3->_IO_write_ptr;
    if ( !IO_write_ptr || !*IO_write_ptr || dword_84400 == 1 || byte_84AD2 )
      IO_write_ptr = (const char *)sub_43090(LODWORD(v3->_old_offset), v41);
    if ( (unsigned int)(*(char *)(a2 + 156) - 51) <= 1 )
    {
      v16 = gnu_dev_major(*(long long *)&v3->_cur_column);
      v17 = sub_43090(v16, v42);
      __strcpy_chk(s, v17, 42);
      __strcat_chk(s, ",", 42);
      v18 = gnu_dev_minor(*(long long *)&v3->_cur_column);
      v19 = sub_43090(v18, v42);
      __strcat_chk(s, v19, 42);
    }
    else
    {
      v11 = sub_43090(v3->_lock, v42);
      __strcpy_chk(s, v11, 42);
    }
    v35 = strlen(s);
    v34 = strlen(IO_write_base);
    v12 = v35;
    v13 = v35 + v34 + strlen(IO_write_ptr) + 2;
    if ( dword_8122C < v13 )
      dword_8122C = v13;
    else
      v12 = dword_8122C - v13 + v35;
    __fprintf_chk(stream, 1, "%s %s/%s %*s %-*s", &v37, IO_write_base, IO_write_ptr, v12, s, dword_81228, v33);
    v14 = (const char *)sub_4A8A0(IO_read_ptr);
    v15 = __fprintf_chk(stream, 1, " %s", v14);
    if ( byte_847E1 && LOBYTE(v3->_IO_read_end) )
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
        v23 = " link to %s\n";
        v24 = sub_4A8A0(v3->_IO_read_base);
        goto LABEL_40;
      case 0x32:
        v32 = (const char *)sub_4A8A0(v3->_IO_read_base);
        __fprintf_chk(stream, 1, " -> %s\n", v32);
        break;
      case 0x4B:
        v27 = "--Long Link--\n";
        goto LABEL_50;
      case 0x4C:
        v27 = "--Long Name--\n";
        goto LABEL_50;
      case 0x4D:
        v29 = sub_22050(a2 + 369, 0xCu);
        v30 = sub_43090(v29, v42);
        __strcpy_chk(s, v30, 42);
        v31 = dcgettext(0, "--Continued at byte %s--\n", 5);
        __fprintf_chk(stream, 1, v31, s, v15);
        break;
      case 0x56:
        v27 = "--Volume Header--\n";
LABEL_50:
        v28 = dcgettext(0, v27, 5);
        __fprintf_chk(stream, 1, v28, v15);
        break;
      default:
        v36[0] = *(char *)(a2 + 156);
        v36[1] = 0;
        v23 = " unknown file type %s\n";
        v24 = sub_4AC50(v36);
LABEL_40:
        v25 = dcgettext(0, v23, 5);
        __fprintf_chk(stream, 1, v25, v24);
        break;
    }
  }
  fflush_unlocked(stream);
  sub_32150(v3);
  return v44 - __readfsqword(0x28u);
}



// Function: print_volume_label @ 0x228b0
unsigned long long print_volume_label(double a1)
{
  int v2; // [rsp+Ch] [rbp-3ECh] BYREF
  FILE v3[2]; // [rsp+10h] [rbp-3E8h] BYREF
  long long v4[65]; // [rsp+1D0h] [rbp-228h] BYREF
  unsigned long long v5; // [rsp+3D8h] [rbp-20h]

  v5 = __readfsqword(0x28u);
  memset(v4, 0, 0x200u);
  BYTE4(v4[19]) = 86;
  if ( qword_84560 )
  {
    v4[17] = *(long long *)(qword_84560 + 136);
    LODWORD(v4[18]) = *(int *)(qword_84560 + 144);
  }
  sub_2F630(v3);
  sub_23BE0(&v3[0]._IO_read_ptr, ".");
  sub_215C0((long long)v4, (long long)v3, &v2, 0);
  sub_23BE0(&v3[0]._IO_read_ptr, qword_83F18);
  sub_22220(v3, (long long)v4, 0);
  sub_2F6F0(v3, a1);
  return v5 - __readfsqword(0x28u);
}



// Function: print_header @ 0x22990
unsigned long long print_header(FILE *a1, long long a2, long long a3, double a4)
{
  if ( dword_84400 != 4 || byte_84451 || !qword_83F18 )
    return sub_22220(a1, a2, a3);
  sub_228B0(a4);
  byte_84451 = 1;
  return sub_22220(a1, a2, a3);
}



// Function: print_for_mkdir @ 0x22a00
unsigned long long print_for_mkdir(long long a1, long long a2, unsigned int a3)
{
  long long v5; // rdi
  long long v6; // rdx
  const char *v7; // rbp
  char *v8; // rax
  long long v9; // rax
  long long v10; // r12
  char *v11; // rax
  char v12; // [rsp+5h] [rbp-43h] BYREF
  char v13[10]; // [rsp+6h] [rbp-42h] BYREF
  char v14[24]; // [rsp+10h] [rbp-38h] BYREF
  unsigned long long v15; // [rsp+28h] [rbp-20h]

  v15 = __readfsqword(0x28u);
  if ( dword_84A54 > 1 )
  {
    v5 = a3;
    v12 = 100;
    sub_32310(a3, v13);
    if ( byte_84B7C )
    {
      v9 = sub_D180(v5, v13, v6);
      v10 = sub_43090(v9, v14);
      v11 = dcgettext(0, "block %s: ", 5);
      __fprintf_chk(stream, 1, v11, v10);
    }
    v7 = (const char *)sub_4A8A0(a1);
    v8 = dcgettext(0, "Creating directory:", 5);
    __fprintf_chk(stream, 1, "%s %*s %s\n", &v12, dword_8122C + dword_81228 + 1, v8, v7);
  }
  return v15 - __readfsqword(0x28u);
}



// Function: skip_file @ 0x22b10
void *skip_file(signed long long a1)
{
  long long v1; // rbx
  void *result; // rax
  unsigned long long v3; // rbp
  void *v4; // rdi
  __off_t v5; // rax
  char *v6; // rax

  v1 = a1;
  if ( byte_847F8 )
  {
    v5 = sub_D540(a1);
    if ( v5 < 0 )
      byte_847F8 = 0;
    else
      v1 = a1 - (v5 << 9);
  }
  result = sub_CCF0(v1);
  if ( v1 > 0 )
  {
    v3 = v1 - 512 - ((v1 - 1) & 0xFFFFFFFFFFFFFE00LL);
    do
    {
      v4 = sub_E130();
      if ( !v4 )
      {
        if ( qword_84D50 )
          qword_84D50(0);
        v6 = dcgettext(0, "Unexpected EOF in archive", 5);
        error(0, 0, v6);
        sub_163D0();
      }
      sub_D1F0((unsigned long long)v4);
      v1 -= 512;
      result = sub_CCF0(v1);
    }
    while ( v1 != v3 );
  }
  return result;
}



// Function: skip_member @ 0x22bd0
void skip_member()
{
  char v0; // bl

  if ( !byte_849C9 )
  {
    v0 = *(char *)(qword_84408 + 156);
    sub_D1F0(qword_84408);
    sub_CCB0((long long)&qword_84840);
    if ( byte_84960 )
    {
      sub_29FE0(&qword_84840);
    }
    else if ( v0 != 53 )
    {
      sub_22B10(buf.st_size);
    }
    sub_CCD0();
  }
}



// Function: read_and @ 0x22c60
long long read_and(void (*a1)(void))
{
  int v1; // r13d
  double v2; // xmm0_8
  long long *v3; // rsi
  long long v4; // rdx
  long long v6; // rdx
  long long v7; // rax
  long long v8; // r12
  char *v9; // rax
  long long v10; // rdi
  FILE *v11; // rdi
  long long v12; // rdx
  char *v13; // rax
  char *v14; // rax
  long long v15; // rax
  long long v16; // rax
  long long v17; // r12
  char *v18; // rax
  long long v19; // rax
  long long v20; // r13
  char *v21; // rax
  long long v22; // rax
  char *v23; // rax
  char *v24; // rax
  long long v26; // [rsp+8h] [rbp-60h]
  long long v27; // [rsp+8h] [rbp-60h]
  char v28[24]; // [rsp+10h] [rbp-58h] BYREF
  unsigned long long v29; // [rsp+28h] [rbp-40h]

  v1 = 0;
  v29 = __readfsqword(0x28u);
  sub_21160();
  v2 = sub_26D90();
  sub_FB70(0, v2);
  while ( 2 )
  {
    sub_2F6F0(&qword_84840, v2);
    v3 = &qword_84840;
    switch ( (unsigned int)sub_219E0((unsigned char **)&qword_84408, (long long)&qword_84840, 0) )
    {
      case 0u:
      case 2u:
LABEL_48:
        abort();
      case 1u:
LABEL_15:
        while ( 2 )
        {
          while ( 2 )
          {
            sub_215C0(qword_84408, (long long)&qword_84840, &dword_84400, 1);
            if ( (unsigned char)sub_27250(path) )
            {
              if ( xmmword_84B00 < 0
                || (v15 = sub_21320(qword_84408 + 136),
                    *(_OWORD *)&file = (unsigned long long)v15,
                    (int)sub_4BE80(v15, 0, xmmword_84B00, *((long long *)&xmmword_84B00 + 1)) >= 0) )
              {
                if ( !(unsigned char)sub_16B00(path, qword_849D8) )
                {
                  LODWORD(v10) = *(char *)(qword_84408 + 156);
                  goto LABEL_38;
                }
              }
            }
            v10 = (unsigned int)*(char *)(qword_84408 + 156);
            if ( *(char *)(qword_84408 + 156) == 77 || *(char *)(qword_84408 + 156) == 86 )
            {
LABEL_38:
              sub_21440(v10, (long long)&qword_84840);
              a1();
              if ( !(unsigned char)sub_273A0(&qword_84840) )
              {
                sub_2F6F0(&qword_84840, v2);
                v3 = &qword_84840;
                switch ( (unsigned int)sub_219E0((unsigned char **)&qword_84408, (long long)&qword_84840, 0) )
                {
                  case 0u:
                  case 2u:
                    goto LABEL_48;
                  case 1u:
                    continue;
                  case 3u:
                    goto LABEL_43;
                  case 4u:
                    goto LABEL_3;
                  case 5u:
                    goto LABEL_42;
                  default:
                    goto LABEL_5;
                }
              }
              goto LABEL_5;
            }
            break;
          }
          if ( *(char *)(qword_84408 + 156) == 53 && byte_84A89 )
          {
            if ( qword_84D50 )
              qword_84D50(v10);
            v27 = sub_4AA70(path);
            v24 = dcgettext(0, "%s: Omitting", 5);
            error(0, 0, v24, v27);
          }
          sub_22BD0();
          if ( !(unsigned char)sub_273A0(&qword_84840) )
          {
            sub_2F6F0(&qword_84840, v2);
            v3 = &qword_84840;
            switch ( (unsigned int)sub_219E0((unsigned char **)&qword_84408, (long long)&qword_84840, 0) )
            {
              case 0u:
              case 2u:
                goto LABEL_48;
              case 1u:
                continue;
              case 3u:
LABEL_43:
                v1 = 1;
                goto LABEL_7;
              case 4u:
                goto LABEL_3;
              case 5u:
LABEL_42:
                v11 = (FILE *)qword_84408;
                sub_D1F0(qword_84408);
                goto LABEL_25;
              default:
                goto LABEL_5;
            }
          }
          break;
        }
LABEL_5:
        sub_F900(v2);
        return sub_27430();
      case 3u:
LABEL_7:
        if ( byte_84B7C )
        {
          v22 = sub_D180(&qword_84408, &qword_84840, v4);
          v26 = sub_43090(v22, v28);
          v23 = dcgettext(0, "block %s: ** Block of NULs **\n", 5);
          v2 = __fprintf_chk(stream, 1, v23, v26);
        }
        sub_D1F0(qword_84408);
        if ( !byte_84B51 )
        {
          if ( (unsigned int)sub_219E0((unsigned char **)&qword_84408, (long long)&qword_84840, 0) != 3
            && (dword_83B80 & 1) != 0 )
          {
            if ( qword_84D50 )
              qword_84D50(&qword_84408);
            v7 = sub_D180(&qword_84408, &qword_84840, v6);
            v8 = sub_43090(v7, v28);
            v9 = dcgettext(0, "A lone zero block at %s", 5);
            error(0, 0, v9, v8);
          }
          goto LABEL_5;
        }
        if ( (unsigned char)sub_273A0(&qword_84840) )
          goto LABEL_5;
        continue;
      case 4u:
LABEL_3:
        if ( byte_84B7C )
        {
          v16 = sub_D180(&qword_84408, &qword_84840, v4);
          v17 = sub_43090(v16, v28);
          v18 = dcgettext(0, "block %s: ** End of File **\n", 5);
          v2 = __fprintf_chk(stream, 1, v18, v17);
        }
        goto LABEL_5;
      case 5u:
        v11 = (FILE *)qword_84408;
        sub_D1F0(qword_84408);
        if ( v1 )
        {
          if ( v1 != 1 )
          {
LABEL_32:
            if ( !(unsigned char)sub_273A0(&qword_84840) )
            {
              sub_2F6F0(&qword_84840, v2);
              switch ( (unsigned int)sub_219E0((unsigned char **)&qword_84408, (long long)&qword_84840, 0) )
              {
                case 0u:
                case 2u:
                  goto LABEL_48;
                case 1u:
                  goto LABEL_15;
                case 3u:
                  goto LABEL_6;
                case 4u:
                  goto LABEL_3;
                case 5u:
                  sub_D1F0(qword_84408);
                  if ( !(unsigned char)sub_273A0(&qword_84840) )
                  {
                    sub_2F6F0(&qword_84840, v2);
                    switch ( (unsigned int)sub_219E0((unsigned char **)&qword_84408, (long long)&qword_84840, 0) )
                    {
                      case 0u:
                      case 2u:
                        goto LABEL_48;
                      case 1u:
                        goto LABEL_15;
                      case 3u:
                        goto LABEL_6;
                      case 4u:
                        goto LABEL_3;
                      case 5u:
                        goto LABEL_51;
                      default:
                        goto LABEL_5;
                    }
                  }
                  goto LABEL_5;
                default:
                  goto LABEL_5;
              }
            }
            goto LABEL_5;
          }
        }
        else
        {
          if ( qword_84D50 )
            qword_84D50(v11);
          v13 = dcgettext(0, "This does not look like a tar archive", 5);
          v3 = 0;
          v11 = 0;
          error(0, 0, v13);
          dword_84D58 = 2;
        }
LABEL_25:
        if ( byte_84B7C )
        {
          v19 = sub_D180(v11, v3, v12);
          v20 = sub_43090(v19 - qword_843E0 - qword_843E8, v28);
          v21 = dcgettext(0, "block %s: ", 5);
          v11 = stream;
          v2 = __fprintf_chk(stream, 1, v21, v20);
        }
        if ( qword_84D50 )
          qword_84D50(v11);
        v14 = dcgettext(0, "Skipping to next header", 5);
        error(0, 0, v14);
        dword_84D58 = 2;
        if ( !(unsigned char)sub_273A0(&qword_84840) )
        {
          sub_2F6F0(&qword_84840, v2);
          switch ( (unsigned int)sub_219E0((unsigned char **)&qword_84408, (long long)&qword_84840, 0) )
          {
            case 0u:
            case 2u:
              goto LABEL_48;
            case 1u:
              goto LABEL_15;
            case 3u:
LABEL_6:
              v1 = 5;
              goto LABEL_7;
            case 4u:
              goto LABEL_3;
            case 5u:
LABEL_51:
              sub_D1F0(qword_84408);
              goto LABEL_32;
            default:
              goto LABEL_5;
          }
        }
        goto LABEL_5;
      default:
        goto LABEL_5;
    }
  }
}



// Function: list_archive @ 0x23260
void list_archive(long long a1, long long a2, long long a3, double a4)
{
  long long v4; // rax
  long long v5; // rax

  v4 = sub_D180(a1, a2, a3);
  if ( dword_84A54 )
  {
    sub_22990((FILE *)&qword_84840, qword_84408, v4, a4);
    if ( byte_84B50 )
    {
      if ( dword_84A54 > 2 && (unsigned char)sub_201B0((long long)&qword_84840) )
      {
        v5 = sub_1F4C0(qword_849D0);
        sub_20710(qword_849D0, v5);
      }
    }
  }
  sub_22BD0();
}



// Function: test_archive_label @ 0x232e0
long long test_archive_label()
{
  double v0; // xmm0_8
  void *v2; // rdi
  void *v3; // rbp

  sub_21160();
  v0 = sub_26D90();
  sub_FB70(0, v0);
  if ( (unsigned int)sub_219E0((unsigned char **)&qword_84408, (long long)&qword_84840, 0) == 1 )
  {
    sub_215C0(qword_84408, (long long)&qword_84840, &dword_84400, 0);
    if ( *(char *)(qword_84408 + 156) == 86 )
      sub_23C20(&qword_83F18, qword_84408, 100);
    v2 = qword_83F18;
    if ( qword_83F18 )
    {
      if ( dword_84A54 )
      {
        sub_228B0(v0);
        v2 = qword_83F18;
      }
      if ( !(unsigned char)sub_27250(v2) && byte_84B10 )
      {
        v3 = sub_DE80((const char *)qword_83F18);
        sub_27250(v3);
        free(v3);
      }
    }
  }
  sub_F900(v0);
  return sub_275B0();
}



// Function: map_hash @ 0x233c0
unsigned long long map_hash(long long *a1, unsigned long long a2)
{
  return *a1 % a2;
}



// Function: map_compare @ 0x233d0
bool map_compare(long long *a1, long long *a2)
{
  return *a1 == *a2;
}



// Function: name_to_uid @ 0x233e0
long long name_to_uid(const char *a1)
{
  struct passwd *v1; // rax

  v1 = getpwnam(a1);
  if ( v1 )
    return v1->pw_uid;
  else
    return -1;
}



// Function: name_to_gid @ 0x23410
long long name_to_gid(const char *a1)
{
  struct group *v1; // rax

  v1 = getgrnam(a1);
  if ( v1 )
    return v1->gr_gid;
  else
    return -1;
}



// Function: owner_map_read @ 0x239a0
long long owner_map_read(long long a1)
{
  return sub_23520(&qword_84570, a1, sub_233E0, "UID");
}



// Function: owner_map_translate @ 0x239d0
long long owner_map_translate(unsigned int a1, int *a2, long long *a3)
{
  long long v4; // rax
  long long v5; // rdx
  long long v6; // rax
  long long result; // rax
  long long v8[7]; // [rsp+0h] [rbp-38h] BYREF

  v8[3] = __readfsqword(0x28u);
  if ( qword_84570 && (v8[0] = a1, (v4 = sub_418D0(qword_84570, v8)) != 0) )
  {
    v5 = *(long long *)(v4 + 8);
    v6 = *(long long *)(v4 + 16);
    *a2 = v5;
    *a3 = v6;
    return 0;
  }
  else
  {
    result = 1;
    if ( dword_84ABC != -1 )
    {
      *a2 = dword_84ABC;
      result = 0;
    }
    if ( qword_84AC0 )
    {
      *a3 = qword_84AC0;
      return 0;
    }
  }
  return result;
}



// Function: group_map_read @ 0x23a70
long long group_map_read(long long a1)
{
  return sub_23520(&qword_84568, a1, sub_23410, "GID");
}



// Function: group_map_translate @ 0x23aa0
long long group_map_translate(unsigned int a1, int *a2, long long *a3)
{
  long long v4; // rax
  long long v5; // rdx
  long long v6; // rax
  long long result; // rax
  long long v8[7]; // [rsp+0h] [rbp-38h] BYREF

  v8[3] = __readfsqword(0x28u);
  if ( qword_84568 && (v8[0] = a1, (v4 = sub_418D0(qword_84568, v8)) != 0) )
  {
    v5 = *(long long *)(v4 + 8);
    v6 = *(long long *)(v4 + 16);
    *a2 = v5;
    *a3 = v6;
    return 0;
  }
  else
  {
    result = 1;
    if ( dword_84B54 != -1 )
    {
      *a2 = dword_84B54;
      result = 0;
    }
    if ( qword_84B58 )
    {
      *a3 = qword_84B58;
      return 0;
    }
  }
  return result;
}



// Function: ptr_align @ 0x23b40
unsigned long long ptr_align(long long a1, unsigned long long a2)
{
  return a2 + a1 - 1 - (a2 + a1 - 1) % a2;
}



// Function: namebuf_finish @ 0x23b60
long long namebuf_finish(long long *a1)
{
  long long v1; // rax
  long long v2; // r12

  v1 = a1[2];
  v2 = *a1;
  if ( *(char *)(*a1 + v1 - 1) == 47 )
    *(char *)(v2 + v1) = 0;
  free(a1);
  return v2;
}



// Function: quote_n_colon @ 0x23bb0
long long quote_n_colon(unsigned int a1, long long a2)
{
  unsigned int v2; // eax

  v2 = sub_4A570(0);
  return sub_4AA90(a1, v2, a2);
}



// Function: assign_string @ 0x23be0
void assign_string(void **a1, void *a2)
{
  long long v2; // rbp

  v2 = (long long)a2;
  free(*a1);
  if ( a2 )
    v2 = sub_4C7A0(a2);
  *a1 = (void *)v2;
}



// Function: assign_string_n @ 0x23c20
void assign_string_n(void **a1, const char *a2, size_t a3)
{
  size_t v4; // r12
  void *v5; // rax
  char *v6; // rax

  free(*a1);
  if ( a2 )
  {
    v4 = strnlen(a2, a3);
    v5 = (void *)sub_4C5B0(v4 + 1);
    v6 = memcpy(v5, a2, v4);
    v6[v4] = 0;
    *a1 = v6;
  }
}



// Function: unquote_string @ 0x23c70
long long unquote_string(char *a1)
{
  char v1; // dl
  char *v2; // rax
  unsigned int v3; // r8d
  char v4; // si
  char v6; // dl
  char v7; // si
  char v8; // si
  char v9; // dl

  v1 = *a1;
  v2 = a1;
  if ( !*a1 )
    return 1;
  v3 = 1;
  while ( v1 != 92 )
  {
    if ( v2 == a1 )
    {
      v1 = v2[1];
      ++a1;
      ++v2;
      if ( !v1 )
        goto LABEL_9;
    }
    else
    {
      *v2++ = v1;
      v1 = *++a1;
LABEL_5:
      if ( !v1 )
        goto LABEL_9;
    }
  }
  v4 = a1[1];
  switch ( v4 )
  {
    case '0':
    case '1':
    case '2':
    case '3':
    case '4':
    case '5':
    case '6':
    case '7':
      v7 = v4 - 48;
      if ( (unsigned char)(a1[2] - 48) > 7u )
      {
        *v2 = v7;
        a1 += 2;
        v1 = *a1;
        ++v2;
      }
      else
      {
        v8 = a1[2] + 8 * v7 - 48;
        v9 = a1[3];
        if ( (unsigned char)(v9 - 48) <= 7u )
        {
          a1 += 4;
          *v2++ = v9 + 8 * v8 - 48;
          v1 = *a1;
        }
        else
        {
          *v2 = v8;
          a1 += 3;
          v1 = *a1;
          ++v2;
        }
      }
      goto LABEL_5;
    case '?':
      *v2 = 127;
      a1 += 2;
      v1 = *a1;
      ++v2;
      goto LABEL_5;
    case '\\':
      *v2 = 92;
      a1 += 2;
      v1 = *a1;
      ++v2;
      goto LABEL_5;
    case 'a':
      *v2 = 7;
      a1 += 2;
      v1 = *a1;
      ++v2;
      goto LABEL_5;
    case 'b':
      *v2 = 8;
      a1 += 2;
      v1 = *a1;
      ++v2;
      goto LABEL_5;
    case 'f':
      *v2 = 12;
      a1 += 2;
      v1 = *a1;
      ++v2;
      goto LABEL_5;
    case 'n':
      *v2 = 10;
      a1 += 2;
      v1 = *a1;
      ++v2;
      goto LABEL_5;
    case 'r':
      *v2 = 13;
      a1 += 2;
      v1 = *a1;
      ++v2;
      goto LABEL_5;
    case 't':
      *v2 = 9;
      a1 += 2;
      v1 = *a1;
      ++v2;
      goto LABEL_5;
    case 'v':
      *v2 = 11;
      a1 += 2;
      v1 = *a1;
      ++v2;
      goto LABEL_5;
    default:
      *v2 = 92;
      v6 = a1[1];
      if ( v6 )
      {
        v2[1] = v6;
        a1 += 2;
        v1 = *a1;
        v2 += 2;
        v3 = 0;
        goto LABEL_5;
      }
      ++v2;
      ++a1;
      v3 = 0;
LABEL_9:
      if ( a1 != v2 )
        *v2 = 0;
      return v3;
  }
}



// Function: zap_slashes @ 0x23e60
const char *zap_slashes(const char *a1)
{
  const char *v1; // rax

  if ( a1 )
  {
    if ( *a1 )
    {
      v1 = &a1[strlen(a1) - 1];
      if ( a1 < v1 )
      {
        do
        {
          if ( *v1 != 47 )
            break;
          *v1-- = 0;
        }
        while ( a1 != v1 );
      }
    }
  }
  return a1;
}



// Function: normalize_filename_x @ 0x23ea0
long long normalize_filename_x(char *a1)
{
  char v1; // dl
  char *i; // rax
  char *v3; // rsi
  char v4; // dl
  char *v5; // rcx
  char *v6; // rax
  char *j; // rcx
  long long result; // rax

  v1 = *a1;
  i = a1;
  v3 = a1;
  if ( *a1 == 46 )
  {
    do
    {
      if ( i[1] != 47 )
        break;
      v4 = i[2];
      v5 = i + 2;
      i += 2;
      if ( v4 == 47 )
      {
        v4 = v5[1];
        for ( i = v5 + 1; v4 == 47; ++i )
          v4 = i[1];
      }
      v3 += v4 == 0;
      *v3 = v4;
    }
    while ( v4 == 46 );
    v1 = *i;
  }
  *v3 = v1;
  v6 = i + 1;
  for ( j = v3 + 1; v1; *(j - 1) = v1 )
  {
    while ( v1 != 47 )
    {
      v1 = *v6;
LABEL_13:
      ++j;
      ++v6;
      *(j - 1) = v1;
      if ( !v1 )
        goto LABEL_18;
    }
    v1 = *v6;
    if ( v6[*v6 == 46] != 47 )
      goto LABEL_13;
    do
    {
      v6 += (v1 == 46) + 1;
      v1 = *v6;
    }
    while ( v6[*v6 == 46] == 47 );
    ++j;
    ++v6;
  }
LABEL_18:
  result = j - a1;
  if ( j - a1 <= 2 )
    return result;
  result = (unsigned char)*(j - 2);
  if ( (char)result == 46 )
  {
    if ( *(j - 3) != 47 )
      goto LABEL_22;
    result = --j - a1;
    if ( j - a1 <= 2 )
      goto LABEL_22;
    goto LABEL_21;
  }
  if ( (char)result == 47 )
LABEL_21:
    --j;
LABEL_22:
  *(j - 1) = 0;
  return result;
}



// Function: replace_prefix @ 0x23fc0
int replace_prefix(const char **a1, const void *a2, size_t a3, const void *a4, size_t a5)
{
  char *v7; // r13
  size_t v8; // rax
  size_t v9; // rbp
  char *v10; // rsi
  long long v11; // rax

  v7 = (char *)*a1;
  v8 = strlen(*a1);
  if ( v8 > a3 )
  {
    v9 = v8;
    LODWORD(v8) = memcmp(v7, a2, a3);
    if ( !(int)v8 )
    {
      v10 = &v7[a3];
      if ( v7[a3] == 47 )
      {
        if ( a3 < a5 )
        {
          v11 = sub_4C610(v7, v9 - a3 + a5 + 1);
          *a1 = (const char *)v11;
          v7 = (char *)v11;
          v10 = (char *)(v11 + a3);
        }
        memmove(&v7[a5], v10, v9 - a3 + 1);
        LODWORD(v8) = (unsigned int)memcpy(v7, a4, a5);
      }
    }
  }
  return v8;
}



// Function: sysinttostr @ 0x24080
long long sysinttostr(unsigned long long a1, long long a2, unsigned long long a3, long long a4)
{
  if ( a1 > a3 )
    return sub_42F50(a1, a4);
  else
    return sub_43090(a1, a4);
}



// Function: strtosysint @ 0x240a0
intmax_t strtosysint(char *nptr, char **endptr, intmax_t a3, long long a4)
{
  int *v6; // rax
  int *v7; // rbx
  intmax_t result; // rax
  uintmax_t v9; // rdi
  bool v10; // cc

  v6 = __errno_location();
  *v6 = 0;
  v7 = v6;
  if ( a4 < 0 )
  {
    if ( (unsigned int)(*nptr - 48) > 9 )
      goto LABEL_3;
    v9 = strtoumax(nptr, endptr, 10);
    if ( a4 < v9 )
    {
      *v7 = 34;
      return a4;
    }
    else
    {
      return sub_23B90(v9);
    }
  }
  else
  {
    if ( (unsigned int)(nptr[*nptr == 45] - 48) > 9 )
    {
LABEL_3:
      *v6 = 22;
      return 0;
    }
    result = strtoimax(nptr, endptr, 10);
    if ( result < a3 || a4 < result )
    {
      v10 = result < a3;
      result = a4;
      *v7 = 34;
      if ( v10 )
        return a3;
    }
  }
  return result;
}



// Function: code_ns_fraction @ 0x24170
void code_ns_fraction(int a1, char *a2)
{
  int v2; // edx
  long long v3; // rax
  int v4; // edi
  char v5; // cl
  int v6; // r8d
  int v7; // r9d
  int v8; // r9d
  long long v9; // rdi

  v2 = a1;
  if ( !a1 )
  {
    *a2 = 0;
    return;
  }
  v3 = a1;
  *a2 = 46;
  v4 = 10 * (a1 / 10);
  v5 = v2 - v4;
  if ( v2 != v4 )
  {
    a2[10] = 0;
    v9 = 8;
    a2[9] = v5 + 48;
    do
    {
LABEL_7:
      v2 = ((int)((unsigned long long)(1717986919 * v3) >> 32) >> 2) - (v2 >> 31);
      v3 = v2;
      a2[v9--] = v2 % 10 + 48;
    }
    while ( (int)v9 );
    return;
  }
  v6 = 9;
  do
  {
    v7 = v6--;
    v2 = ((int)((unsigned long long)(1717986919 * v3) >> 32) >> 2) - (v2 >> 31);
    v3 = v2;
  }
  while ( !(v2 % 10) );
  v8 = v7 - 2;
  v9 = v8;
  a2[v6 + 1] = 0;
  a2[v8 + 1] = v2 % 10 + 48;
  if ( v8 )
    goto LABEL_7;
}



// Function: code_timespec @ 0x24270
long long code_timespec(long long a1, unsigned int a2, long long a3)
{
  long long v3; // rax
  int v5; // ebp
  long long v6; // r10

  v3 = a1;
  if ( a2 > 0x3B9AC9FF )
  {
    v5 = 0;
  }
  else
  {
    v5 = a2;
    if ( a2 && a1 < 0 )
    {
      v3 = a1 + 1;
      v5 = 1000000000 - a2;
      goto LABEL_5;
    }
  }
  if ( a1 >= 0 )
  {
    sub_43090(a1, a3 + 1);
    goto LABEL_6;
  }
LABEL_5:
  *(char *)(sub_43090(-v3, a3 + 1) - 1) = 45;
LABEL_6:
  sub_24170(v5, (char *)(a3 + 21));
  return v6;
}



// Function: decode_timespec @ 0x242f0
long long decode_timespec(char *nptr, char **endptr, char a3)
{
  char v4; // r14
  int *v6; // r12
  signed long long v8; // r8
  char *v9; // rbp
  char *v10; // rax
  int v11; // esi
  char v12; // cl
  unsigned char v13; // r9
  int v14; // eax
  int v15; // edx
  intmax_t v16; // rax

  v4 = *nptr;
  v6 = __errno_location();
  if ( (unsigned int)(nptr[v4 == 45] - 48) > 9 )
  {
    *v6 = 22;
    *endptr = nptr;
    return 0x8000000000000000LL;
  }
  *v6 = 0;
  if ( v4 == 45 )
  {
    v16 = strtoimax(nptr, endptr, 10);
    v9 = *endptr;
    v8 = v16;
    if ( !a3 )
      goto LABEL_16;
LABEL_6:
    v10 = v9;
    if ( *v9 == 46 )
      goto LABEL_7;
LABEL_16:
    *endptr = v9;
    return v8;
  }
  v8 = strtoumax(nptr, endptr, 10);
  if ( v8 >= 0 )
  {
    v9 = *endptr;
    if ( !a3 )
      goto LABEL_16;
    goto LABEL_6;
  }
  *v6 = 34;
  v9 = *endptr;
  if ( a3 && *v9 == 46 )
  {
    v10 = *endptr;
    v8 = 0x8000000000000000LL;
LABEL_7:
    v11 = v10[1];
    ++v9;
    v12 = v10[1];
    if ( (unsigned int)(v11 - 48) > 9 )
    {
      v15 = 0;
      v13 = 0;
      v14 = 0;
    }
    else
    {
      v13 = 0;
      v14 = 0;
      v15 = 0;
      do
      {
        if ( v14 <= 8 )
        {
          ++v14;
          v15 = v11 + 10 * v15 - 48;
        }
        else
        {
          v13 |= v12 != 48;
        }
        v11 = *++v9;
        v12 = v11;
      }
      while ( (unsigned int)(v11 - 48) <= 9 );
      if ( v14 > 8 )
        goto LABEL_15;
    }
    do
    {
      ++v14;
      v15 *= 10;
    }
    while ( v14 != 9 );
LABEL_15:
    if ( v4 == 45 && v13 + v15 && v8 != 0x8000000000000000LL )
      --v8;
    goto LABEL_16;
  }
  *endptr = v9;
  return 0x8000000000000000LL;
}



// Function: must_be_dot_or_slash @ 0x244e0
bool must_be_dot_or_slash(char *a1)
{
  char v1; // al

  v1 = *a1;
  if ( *a1 == 47 )
  {
    while ( 1 )
    {
      while ( 1 )
      {
        v1 = a1[1];
        if ( v1 != 47 )
          break;
        ++a1;
      }
      if ( v1 != 46 )
        break;
      a1 += (a1[2] == 46) + 2;
      if ( *a1 != 47 )
        return 0;
    }
  }
  else
  {
    while ( v1 == 46 )
    {
      v1 = a1[1];
      if ( v1 != 47 )
        break;
      v1 = a1[2];
      a1 += 2;
      if ( v1 != 47 )
        continue;
      do
        v1 = *++a1;
      while ( v1 == 47 );
    }
  }
  return v1 == 0;
}



// Function: safer_rmdir @ 0x24560
long long safer_rmdir(char *s2)
{
  unsigned int v1; // r12d

  if ( sub_244E0(s2) )
  {
    v1 = -1;
    *__errno_location() = 0;
  }
  else
  {
    v1 = unlinkat(oldfd, s2, 512);
    if ( v1 )
      return (unsigned int)-1;
    else
      sub_19B90(s2);
  }
  return v1;
}



// Function: undo_last_backup @ 0x245d0
void undo_last_backup()
{
  long long v0; // rdi
  int v1; // ebp
  long long v2; // rbx
  long long v3; // r12
  char *v4; // rax
  long long v5; // rbx
  long long v6; // r12
  char *v7; // rax

  if ( old )
  {
    v0 = (unsigned int)oldfd;
    if ( renameat(oldfd, old, oldfd, new) )
    {
      v1 = *__errno_location();
      if ( qword_84D50 )
        qword_84D50(v0);
      v2 = sub_4AC40(1, new);
      v3 = sub_4AA70(old);
      v4 = dcgettext(0, "%s: Cannot rename to %s", 5);
      error(0, v1, v4, v3, v2);
      dword_84D58 = 2;
    }
    if ( dword_84A54 )
    {
      v5 = sub_4AC40(1, new);
      v6 = sub_4AC40(0, old);
      v7 = dcgettext(0, "Renaming %s back to %s\n", 5);
      __fprintf_chk(stream, 1, v7, v6, v5);
    }
    sub_23BE0((void **)&old, 0);
  }
}



// Function: deref_stat @ 0x246f0
int deref_stat(char *file, struct stat *buf)
{
  return fstatat(oldfd, file, buf, flag);
}



// Function: maybe_backup_file @ 0x24710
long long maybe_backup_file(char *file, char a2)
{
  char *v2; // rax
  long long v4; // rdi
  int v5; // ebp
  long long v6; // rbx
  long long v7; // r13
  char *v8; // rax
  long long v9; // rbx
  long long v10; // r12
  char *v11; // rax
  struct stat v12; // [rsp+0h] [rbp-C8h] BYREF
  unsigned long long v13; // [rsp+98h] [rbp-30h]

  v13 = __readfsqword(0x28u);
  sub_23BE0((void **)&new, file);
  sub_23BE0((void **)&old, 0);
  if ( a2 )
  {
    if ( !byte_84D70 )
    {
      v2 = strchr(file, 58);
      qword_84D78 = (long long)v2;
      if ( v2 )
      {
        if ( v2 > file && !memchr(file, 47, v2 - file) )
          return 1;
      }
    }
  }
  if ( sub_246F0(file, &v12) )
  {
    if ( *__errno_location() != 2 )
    {
      sub_32D30(file);
      return 0;
    }
    return 1;
  }
  if ( (v12.st_mode & 0xF000) == 0x4000 || a2 && (v12.st_mode & 0xB000) == 0x2000 )
    return 1;
  old = (char *)sub_3E2F0((unsigned int)oldfd, file, (unsigned int)dword_84B80);
  if ( !old )
    sub_16420();
  v4 = (unsigned int)oldfd;
  if ( !renameat(oldfd, new, oldfd, old) )
  {
    if ( dword_84A54 )
    {
      v9 = sub_4AC40(1, old);
      v10 = sub_4AC40(0, new);
      v11 = dcgettext(0, "Renaming %s to %s\n", 5);
      __fprintf_chk(stream, 1, v11, v10, v9);
    }
    return 1;
  }
  v5 = *__errno_location();
  if ( qword_84D50 )
    qword_84D50(v4);
  v6 = sub_4AC40(1, old);
  v7 = sub_4AA70(new);
  v8 = dcgettext(0, "%s: Cannot rename to %s", 5);
  error(0, v5, v8, v7, v6);
  dword_84D58 = 2;
  sub_23BE0((void **)&old, 0);
  return 0;
}



// Function: blocking_read @ 0x24960
long long blocking_read(int fd, void *buf, size_t nbytes)
{
  long long v4; // r12
  int v6; // edx
  int v7; // ecx
  int v8; // r8d
  int v9; // r9d
  int v10; // eax
  int v11; // ecx
  int v12; // r8d
  int v13; // r9d

  v4 = sub_4AF10(fd, buf, nbytes);
  if ( v4 == -1
    && *__errno_location() == 11
    && (v10 = sub_4D180(fd, 3, v6, v7, v8, v9), v10 >= 0)
    && (v10 & 0x800) != 0
    && (BYTE1(v10) &= ~8u, (unsigned int)sub_4D180(fd, 4, v10, v11, v12, v13) != -1) )
  {
    return sub_4AF10(fd, buf, nbytes);
  }
  else
  {
    return v4;
  }
}



// Function: blocking_write @ 0x249f0
unsigned long long blocking_write(int fd, char *a2, size_t a3)
{
  unsigned long long v4; // r12
  int v5; // edx
  int v6; // ecx
  int v7; // r8d
  int v8; // r9d
  int v10; // eax
  int v11; // ecx
  int v12; // r8d
  int v13; // r9d

  v4 = sub_41120(fd, a2, a3);
  if ( a3 > v4 && *__errno_location() == 11 )
  {
    v10 = sub_4D180(fd, 3, v5, v6, v7, v8);
    if ( v10 >= 0 && (v10 & 0x800) != 0 )
    {
      BYTE1(v10) &= ~8u;
      if ( (unsigned int)sub_4D180(fd, 4, v10, v11, v12, v13) != -1 )
        v4 += sub_41120(fd, &a2[v4], a3 - v4);
    }
  }
  return v4;
}



// Function: set_file_atime @ 0x24a80
long long set_file_atime(long long a1, long long a2, long long a3, long long a4, long long a5)
{
  long long v6[7]; // [rsp+0h] [rbp-38h] BYREF

  v6[5] = __readfsqword(0x28u);
  v6[1] = a5;
  v6[0] = a4;
  v6[3] = 1073741822;
  return sub_3F360(a1, a2, a3, v6);
}



// Function: chdir_count @ 0x24ae0
long long chdir_count()
{
  long long result; // rax

  result = (unsigned int)(qword_845E8 - 1);
  if ( !qword_845E8 )
    return 0;
  return result;
}



// Function: chdir_arg @ 0x24b00
long long chdir_arg(char *a1)
{
  long long result; // rax
  char *v2; // rbx
  long long v3; // rdx
  char *v4; // rcx
  char v5; // dl
  char v6; // dl
  long long v7; // rdx

  result = qword_845E8;
  v2 = a1;
  if ( qword_845E8 == qword_845E0 )
  {
    if ( !qword_845E8 )
      qword_845E0 = 2;
    qword_845F0 = sub_4C680(qword_845F0, &qword_845E0, 24);
    v7 = qword_845F0;
    result = qword_845E8;
    if ( !qword_845E8 )
    {
      *(long long *)(qword_845F0 + 8) = 0;
      *(long long *)v7 = ".";
      result = 1;
      *(int *)(v7 + 16) = -100;
      qword_845E8 = 1;
    }
  }
  if ( *a1 == 46 )
  {
    while ( v2[1] == 47 )
    {
      v6 = v2[2];
      v4 = v2 + 2;
      v2 += 2;
      if ( v6 == 47 )
      {
        while ( 1 )
        {
          v5 = v4[1];
          v2 = v4 + 1;
          if ( v5 != 47 )
            break;
          ++v4;
        }
        if ( v5 != 46 )
        {
LABEL_10:
          if ( *v2 )
            goto LABEL_3;
          return (unsigned int)(result - 1);
        }
      }
      else if ( v6 != 46 )
      {
        goto LABEL_10;
      }
    }
    if ( !v2[1] )
      return (unsigned int)(result - 1);
  }
LABEL_3:
  v3 = qword_845F0 + 24 * result;
  *(long long *)v3 = v2;
  *(long long *)(v3 + 8) = 0;
  *(int *)(v3 + 16) = 0;
  qword_845E8 = result + 1;
  return result;
}



// Function: tar_dirname @ 0x24c30
long long tar_dirname()
{
  return *(long long *)(qword_845F0 + 24LL * dword_84580);
}



// Function: close_diag @ 0x24c50
long long close_diag(long long a1)
{
  long long result; // rax

  if ( !byte_84B52 )
    return sub_32680(a1);
  if ( (dword_83B80 & 0x800000) != 0 )
    return sub_324F0("close", a1);
  return result;
}



// Function: chdir_do @ 0x24c80
void chdir_do(int a1)
{
  long long v1; // r12
  int v2; // ebp
  char *v3; // rsi
  unsigned long long v4; // rax
  int v5; // edx
  int *v6; // rax
  int v7; // ecx
  long long v8; // r12

  if ( dword_84580 != a1 )
  {
    v1 = qword_845F0 + 24LL * a1;
    v2 = *(int *)(v1 + 16);
    if ( !v2 )
    {
      v3 = *(char **)v1;
      if ( **(char **)v1 != 47 )
      {
        chdir_do((unsigned int)(a1 - 1));
        v3 = *(char **)v1;
      }
      v2 = __openat_2((unsigned int)oldfd, v3, dword_84804 & 0xFFFDFFFF);
      if ( v2 < 0 )
        sub_327E0(*(long long *)v1);
      v4 = qword_84590;
      *(int *)(v1 + 16) = v2;
      if ( v4 > 0xF )
      {
        v8 = qword_845F0 + 24LL * dword_845DC;
        if ( close(*(int *)(v8 + 16)) )
          sub_24C50(*(long long *)v8);
        *(int *)(v8 + 16) = 0;
        dword_845DC = a1;
        if ( v2 <= 0 )
          goto LABEL_13;
        goto LABEL_9;
      }
      qword_84590 = v4 + 1;
      dword_845A0[v4] = a1;
    }
    if ( v2 <= 0 )
    {
LABEL_13:
      dword_84580 = a1;
      oldfd = v2;
      return;
    }
LABEL_9:
    v5 = dword_845A0[0];
    if ( a1 != dword_845A0[0] )
    {
      v6 = (int *)&unk_845A4;
      do
      {
        v7 = v5;
        v5 = *v6++;
        *(v6 - 1) = v7;
      }
      while ( a1 != v5 );
    }
    dword_845A0[0] = a1;
    goto LABEL_13;
  }
}



// Function: open_diag @ 0x24dd0
long long open_diag(long long a1)
{
  long long result; // rax

  if ( !byte_84B52 )
    return sub_327C0(a1);
  if ( (dword_83B80 & 0x800000) != 0 )
    return sub_32800(a1);
  return result;
}



// Function: read_diag_details @ 0x24e00
long long read_diag_details()
{
  long long result; // rax

  if ( !byte_84B52 )
    return sub_32840();
  if ( (dword_83B80 & 0x800000) != 0 )
    return sub_32900();
  return result;
}



// Function: readlink_diag @ 0x24e30
long long readlink_diag(long long a1)
{
  long long result; // rax

  if ( !byte_84B52 )
    return sub_32A60(a1);
  if ( (dword_83B80 & 0x800000) != 0 )
    return sub_32A80(a1);
  return result;
}



// Function: savedir_diag @ 0x24e60
long long savedir_diag(long long a1)
{
  long long result; // rax

  if ( !byte_84B52 )
    return sub_32AC0(a1);
  if ( (dword_83B80 & 0x800000) != 0 )
    return sub_324F0("savedir", a1);
  return result;
}



// Function: seek_diag_details @ 0x24e90
long long seek_diag_details(long long a1, long long a2)
{
  long long result; // rax

  if ( !byte_84B52 )
    return sub_32B20(a1, a2);
  if ( (dword_83B80 & 0x800000) != 0 )
    return sub_32BF0();
  return result;
}



// Function: stat_diag @ 0x24ec0
long long stat_diag(long long a1)
{
  long long result; // rax

  if ( !byte_84B52 )
    return sub_32D30(a1);
  if ( (dword_83B80 & 0x800000) != 0 )
    return sub_32D50();
  return result;
}



// Function: file_removed_diag @ 0x24ef0
long long file_removed_diag(long long a1, char a2, long long (*a3)(long long))
{
  long long v5; // r12
  char *v6; // rax

  if ( a2 || *__errno_location() != 2 )
    return a3(a1);
  if ( (dword_83B80 & 0x40) != 0 )
  {
    if ( qword_84D50 )
      qword_84D50(a1);
    v5 = sub_4AA70(a1);
    v6 = dcgettext(0, "%s: File removed before we read it", 5);
    error(0, 0, v6, v5);
  }
  return sub_2F7D0(1);
}



// Function: xfork @ 0x24f80
__pid_t xfork()
{
  __pid_t result; // eax
  char *v1; // rax

  result = fork();
  if ( result == -1 )
  {
    v1 = dcgettext(0, "child process", 5);
    sub_32490("fork", v1);
  }
  return result;
}



// Function: xpipe @ 0x24fc0
int xpipe(int *a1)
{
  int result; // eax
  char *v2; // rax

  result = pipe(a1);
  if ( result < 0 )
  {
    v2 = dcgettext(0, "interprocess channel", 5);
    sub_32490("pipe", v2);
  }
  return result;
}



// Function: page_aligned_alloc @ 0x25000
unsigned long long page_aligned_alloc(long long *a1, long long a2)
{
  unsigned long long v2; // rbp
  long long v3; // rax

  v2 = getpagesize();
  if ( __CFADD__(v2, a2) )
    sub_16420();
  v3 = sub_4C5B0(v2 + a2);
  *a1 = v3;
  return sub_23B40(v3, v2);
}



// Function: namebuf_create @ 0x25040
char **namebuf_create(char *src)
{
  char **v1; // r12
  size_t v2; // rax
  char *v3; // rbx
  size_t v4; // rax
  bool v5; // zf

  v1 = (char **)sub_4C5B0(24);
  v2 = strlen(src);
  v1[1] = (char *)(v2 + 2);
  v3 = (char *)sub_4C5B0(v2 + 2);
  *v1 = v3;
  strcpy(v3, src);
  v4 = strlen(v3);
  v5 = v3[v4 - 1] == 47;
  v1[2] = (char *)v4;
  if ( !v5 )
  {
    v3[v4] = 47;
    v1[2] = (char *)(v4 + 1);
  }
  return v1;
}



// Function: namebuf_free @ 0x250b0
void namebuf_free(void **ptr)
{
  free(*ptr);
  free(ptr);
}



// Function: namebuf_name @ 0x250d0
long long namebuf_name(long long *a1, const char *a2)
{
  size_t v3; // rbp
  long long v4; // rax
  long long v5; // rdi
  long long v6; // rax

  v3 = strlen(a2);
  v4 = a1[2];
  if ( a1[1] > v4 + v3 + 1 )
  {
    v5 = *a1;
  }
  else
  {
    v5 = *a1;
    do
    {
      v6 = sub_4C710(v5, a1 + 1);
      *a1 = v6;
      v5 = v6;
      v4 = a1[2];
    }
    while ( v4 + v3 + 1 >= a1[1] );
  }
  strcpy((char *)(v4 + v5), a2);
  return *a1;
}



// Function: namebuf_add_dir @ 0x25150
size_t namebuf_add_dir(long long *a1, const char *a2)
{
  size_t result; // rax

  if ( *(char *)(*a1 + a1[2] - 1) != 47 )
  {
    sub_250D0(a1, "/");
    ++a1[2];
  }
  sub_250D0(a1, a2);
  result = strlen(a2);
  a1[2] += result;
  return result;
}



// Function: tar_getcdpath @ 0x251a0
long long tar_getcdpath(int a1)
{
  long long v1; // r13
  long long result; // rax
  long long v3; // rax
  int v4; // edx
  int v5; // ebx
  long long *v6; // r15
  long long v7; // rbp
  long long v8; // rbx
  long long v9; // rax
  long long v10; // rax
  long long v11; // r14
  long long v12; // rbp
  long long v13; // r15
  int v14; // [rsp+Ch] [rbp-3Ch]

  if ( qword_845F0 )
  {
    v1 = 24LL * a1;
    result = *(long long *)(qword_845F0 + v1 + 8);
    if ( !result )
    {
      v14 = dword_84580;
      if ( a1 < 0 )
      {
        v5 = a1 + 1;
      }
      else
      {
        v3 = qword_845F0 + v1 - 16;
        v4 = a1;
        do
        {
          v5 = v4--;
          if ( !v5 )
            goto LABEL_15;
          v3 -= 24;
        }
        while ( !*(long long *)(v3 + 24) );
      }
      while ( a1 >= v5 )
      {
LABEL_15:
        sub_24C80(v5);
        if ( !v5 )
        {
          v8 = qword_845F0;
          v9 = sub_4C7C0();
          *(long long *)(v8 + 8) = v9;
          if ( !v9 )
            goto LABEL_12;
          if ( a1 <= 0 )
            break;
          v5 = 1;
          sub_24C80(1);
        }
        v10 = v5++;
        v11 = 3 * v10;
        v12 = 24 * v10;
        v13 = qword_845F0 + 24 * v10;
        if ( **(char **)v13 == 47 )
        {
          *(long long *)(v13 + 8) = sub_4C7A0(*(void **)v13);
        }
        else
        {
          v6 = (long long *)sub_25040(*(char **)(qword_845F0 + v12 - 16));
          sub_25150(v6, *(const char **)(qword_845F0 + 8 * v11));
          v7 = qword_845F0 + v12;
          *(long long *)(v7 + 8) = sub_23B60(v6);
        }
      }
      sub_24C80(v14);
      return *(long long *)(qword_845F0 + v1 + 8);
    }
  }
  else
  {
    result = qword_84588;
    if ( !qword_84588 )
    {
      result = sub_4C7C0();
      qword_84588 = result;
      if ( !result )
LABEL_12:
        sub_32490("getcwd", ".");
    }
  }
  return result;
}



// Function: normalize_filename @ 0x25340
char *normalize_filename(int a1, char *a2)
{
  const char *v2; // r14
  size_t v3; // rax
  size_t v4; // rbx
  size_t v5; // r13
  size_t v6; // rax
  char *v7; // r12

  if ( *a2 == 47 )
  {
    v7 = (char *)sub_4C7A0(a2);
  }
  else
  {
    v2 = (const char *)sub_251A0(a1);
    v3 = strlen(v2);
    v4 = v3 + 1;
    v5 = v3;
    v6 = strlen(a2);
    v7 = (char *)sub_4C5B0(v4 + v6 + 1);
    strcpy(v7, v2);
    v7[v5] = 47;
    strcpy(&v7[v4], a2);
  }
  sub_23EA0(v7);
  return v7;
}



// Function: tar_savedir @ 0x253c0
DIR *tar_savedir(long long a1, int a2)
{
  int v3; // eax
  DIR *v4; // r13
  int v6; // r12d
  DIR *v7; // rax
  long long v8; // r12
  DIR *v9; // rdi

  v3 = __openat_2((unsigned int)oldfd, a1, dword_84808 | 0x10000u);
  if ( v3 < 0 )
  {
    if ( a2 || *__errno_location() != 2 )
      sub_327C0(a1);
    return 0;
  }
  v6 = v3;
  v7 = fdopendir(v3);
  v4 = v7;
  if ( v7 )
  {
    v8 = sub_4B0D0(v7);
    if ( !v8 )
      sub_32AC0(a1);
    v9 = v4;
    v4 = (DIR *)v8;
    if ( !closedir(v9) )
      return v4;
  }
  else
  {
    sub_32AC0(a1);
    if ( !close(v6) )
      return 0;
  }
  sub_32AC0(a1);
  return v4;
}



// Function: remove_any_file @ 0x25490
long long remove_any_file(char *a1, int a2)
{
  int *v2; // rbx
  long long result; // rax
  unsigned long long v4; // rdx
  long long v5; // rcx
  DIR *v6; // rax
  DIR *v7; // r13
  DIR *v8; // r14
  size_t v9; // r12
  void *v10; // r15
  int v11; // [rsp+8h] [rbp-40h]
  int v12; // [rsp+Ch] [rbp-3Ch]

  if ( !unlinkat(oldfd, a1, 0) )
    return 1;
  v2 = __errno_location();
  if ( *v2 != 1 && *v2 != 21 )
    return 0;
  if ( !(unsigned int)sub_24560(a1) )
    return 1;
  v4 = (unsigned int)*v2;
  result = 0;
  if ( (unsigned int)v4 <= 0x27 )
  {
    v5 = 0x8000020001LL;
    if ( _bittest64(&v5, v4) )
    {
      if ( a2 != 1 )
        return (unsigned int)-(a2 == 2);
      v6 = sub_253C0((long long)a1, 0);
      v7 = v6;
      if ( !v6 )
        return 0;
      v8 = v6;
      v9 = strlen((const char *)v6);
      if ( v9 )
      {
        while ( 1 )
        {
          v10 = (void *)sub_27BC0(a1, v8);
          v11 = remove_any_file(v10, 1);
          v12 = *v2;
          free(v10);
          if ( !v11 )
            break;
          v8 = (DIR *)((char *)v8 + v9 + 1);
          v9 = strlen((const char *)v8);
          if ( !v9 )
            goto LABEL_16;
        }
        free(v7);
        *v2 = v12;
        return 0;
      }
      else
      {
LABEL_16:
        free(v7);
        return (unsigned int)sub_24560(a1) == 0;
      }
    }
  }
  return result;
}



// Function: file_selection_option @ 0x255e0
int *file_selection_option(int a1)
{
  int *result; // rax
  int v2; // edx

  for ( result = &unk_81280; ; result += 12 )
  {
    v2 = result[2];
    if ( !*(long long *)result && !v2 && !*((long long *)result + 4) )
      break;
    if ( a1 == v2 )
      return result;
  }
  return 0;
}



// Function: file_selection_option_name @ 0x25610
int *file_selection_option_name(int a1)
{
  int *result; // rax

  result = sub_255E0(a1);
  if ( result )
    return *(int **)result;
  return result;
}



// Function: is_file_selection_option @ 0x25620
bool is_file_selection_option(int a1)
{
  return sub_255E0(a1) != 0;
}



// Function: name_list_adjust @ 0x25630
void *name_list_adjust()
{
  void *result; // rax
  void *v1; // rdx

  result = qword_846B0;
  if ( qword_846B0 )
  {
    result = (void *)*((long long *)qword_846B0 + 1);
    if ( result )
    {
      do
      {
        v1 = result;
        result = (void *)*((long long *)result + 1);
      }
      while ( result );
      qword_846B0 = v1;
    }
  }
  return result;
}



// Function: unconsumed_option_push @ 0x25660
long long *unconsumed_option_push(long long *a1)
{
  long long *result; // rax

  result = qword_84628;
  *a1 = 0;
  a1[1] = result;
  if ( result )
    *result = a1;
  qword_84628 = a1;
  return result;
}



// Function: merge_sort_sll @ 0x25690
long long *merge_sort_sll(long long *a1, int a2, int (*a3)(long long *, long long *))
{
  long long *v4; // rax
  int v5; // edx
  long long *v6; // r14
  long long *v7; // rbp
  long long v8; // rax
  long long *v9; // rbx
  long long *i; // r13
  long long *v11; // rdx
  long long *v12; // rax
  long long *v13; // rdx
  long long *result; // rax
  long long v15[7]; // [rsp+10h] [rbp-38h] BYREF

  v15[1] = __readfsqword(0x28u);
  if ( a2 == 1 )
    return a1;
  if ( a2 == 2 )
  {
    if ( a3(a1, (long long *)*a1) > 0 )
    {
      result = (long long *)*a1;
      *(long long *)*a1 = a1;
      *a1 = 0;
      return result;
    }
    return a1;
  }
  v4 = a1;
  v5 = (a2 + 1) / 2 - 1;
  if ( (a2 + 1) / 2 != 1 )
  {
    do
    {
      v4 = (long long *)*v4;
      --v5;
    }
    while ( v5 );
  }
  v6 = (long long *)*v4;
  *v4 = 0;
  v7 = (long long *)merge_sort_sll(a1, (unsigned int)((a2 + 1) / 2), a3);
  v8 = merge_sort_sll(v6, (unsigned int)(a2 / 2), a3);
  v9 = (long long *)v8;
  if ( v7 && v8 )
  {
    for ( i = v15; ; i = v12 )
    {
      if ( a3(v7, v9) >= 0 )
      {
        v11 = v9;
        v12 = v9;
        v9 = (long long *)*v9;
        *i = v11;
      }
      else
      {
        v13 = v7;
        v12 = v7;
        v7 = (long long *)*v7;
        *i = v13;
        if ( !v7 )
          goto LABEL_12;
      }
      if ( !v9 )
        goto LABEL_12;
    }
  }
  v12 = v15;
LABEL_12:
  if ( !v7 )
    v7 = v9;
  *v12 = v7;
  return (long long *)v15[0];
}



// Function: merge_sort @ 0x25800
long long *merge_sort(long long *a1, int a2, int (*a3)(long long *, long long *))
{
  long long *result; // rax
  long long *v4; // rdx
  long long *v5; // rsi
  long long *v6; // rcx

  result = sub_25690(a1, a2, a3);
  if ( result )
  {
    v4 = result;
    v5 = 0;
    while ( 1 )
    {
      v6 = (long long *)*v4;
      v4[1] = v5;
      v5 = v4;
      if ( !v6 )
        break;
      v4 = v6;
    }
  }
  return result;
}



// Function: file_list_name @ 0x25830
char *file_list_name()
{
  int *v0; // rax

  v0 = qword_846B0;
  if ( !qword_846B0 )
    return dcgettext(0, "command line", 5);
  while ( v0[4] != 2 || !*((long long *)v0 + 6) )
  {
    v0 = *(int **)v0;
    if ( !v0 )
      return dcgettext(0, "command line", 5);
  }
  return (char *)*((long long *)v0 + 3);
}



// Function: name_elt_alloc @ 0x25880
long long *name_elt_alloc()
{
  long long *result; // rax
  long long *v1; // rdx
  long long *v2; // rcx

  result = (long long *)sub_4C5B0(56);
  v1 = qword_846B0;
  if ( !qword_846B0 )
  {
    *result = 0;
    result[1] = 0;
    *((int *)result + 4) = 3;
    qword_846B0 = result;
    result = (long long *)sub_4C5B0(56);
    v1 = qword_846B0;
  }
  v2 = (long long *)v1[1];
  result[1] = v2;
  if ( v2 )
    *v2 = result;
  *result = v1;
  v1[1] = result;
  return result;
}



// Function: name_add_dir @ 0x258f0
long long *name_add_dir(long long a1)
{
  long long *result; // rax

  result = sub_25880();
  *((int *)result + 4) = 1;
  result[3] = a1;
  return result;
}



// Function: name_add_file @ 0x25910
long long *name_add_file(long long a1)
{
  long long *result; // rax

  result = sub_25880();
  dword_84630 = 2;
  *((int *)result + 4) = 2;
  result[3] = a1;
  result[4] = 0;
  result[6] = 0;
  return result;
}



// Function: name_add_option @ 0x25940
long long *name_add_option(int a1, long long a2)
{
  long long *result; // rax

  result = sub_25880();
  *((int *)result + 4) = 4;
  *((int *)result + 6) = a1;
  result[4] = a2;
  return result;
}



// Function: compare_names_found @ 0x25970
int compare_names_found(long long a1, long long a2)
{
  unsigned long long v2; // rax
  unsigned long long v3; // rcx
  int result; // eax

  v2 = *(long long *)(a2 + 48);
  v3 = *(long long *)(a1 + 48);
  if ( !qword_84B38 )
  {
    result = (v2 != 0) - (v3 != 0);
    if ( result )
      return result;
    return strcmp(*(const char **)(a1 + 16), *(const char **)(a2 + 16));
  }
  result = (qword_84B38 <= v2) - (qword_84B38 <= v3);
  if ( !result )
    return strcmp(*(const char **)(a1 + 16), *(const char **)(a2 + 16));
  return result;
}



// Function: name_compare @ 0x259d0
bool name_compare(long long a1, long long a2)
{
  return strcmp(*(const char **)(a1 + 88), *(const char **)(a2 + 88)) == 0;
}



// Function: compare_names @ 0x259f0
int compare_names(long long a1, long long a2)
{
  return strcmp(*(const char **)(a1 + 16), *(const char **)(a2 + 16));
}



// Function: unconsumed_option_free @ 0x25a10
void unconsumed_option_free()
{
  long long *i; // rdi

  for ( i = qword_84628; qword_84628; i = qword_84628 )
  {
    qword_84628 = (void *)i[1];
    free(i);
  }
}



// Function: name_list_advance @ 0x25a50
void name_list_advance()
{
  int *v0; // rbp
  long long *v1; // rax
  int v2; // eax

  v0 = qword_846B0;
  v1 = *(long long **)qword_846B0;
  qword_846B0 = v1;
  if ( v1 )
    v1[1] = 0;
  v2 = v0[4];
  if ( v2 == 4 || v2 == 1 )
  {
    if ( dword_84BA8 == 3 || dword_84BA8 == 8 )
      sub_25660(v0);
  }
  else
  {
    if ( v2 != 3 )
      sub_25A10();
    free(v0);
  }
}



// Function: free_name @ 0x25ad0
void free_name(void **ptr)
{
  if ( ptr )
  {
    free(ptr[2]);
    free(ptr[11]);
    free(ptr);
  }
}



// Function: add_file_id @ 0x25b00
long long add_file_id(const char *a1)
{
  unsigned int v1; // r12d
  char *v2; // rax
  long long *v3; // rbx
  char *v4; // rbp
  unsigned int v5; // r12d
  long long v6; // rbx
  long long v7; // r13
  char *v8; // rax
  long long v9; // rdx
  long long *v10; // rax
  long long v11; // rdx
  struct stat v13; // [rsp+0h] [rbp-C8h] BYREF
  unsigned long long v14; // [rsp+98h] [rbp-30h]

  v14 = __readfsqword(0x28u);
  if ( stat(a1, &v13) )
    sub_32D10(a1);
  v1 = 0;
  v2 = sub_25830();
  v3 = (long long *)qword_84698;
  v4 = v2;
  if ( qword_84698 )
  {
    while ( v3[1] != v13.st_ino || v3[2] != v13.st_dev )
    {
      v3 = (long long *)*v3;
      if ( !v3 )
        goto LABEL_10;
    }
    v5 = sub_4A5B0(0, 58, 1);
    if ( qword_84D50 )
      qword_84D50(0);
    v6 = v3[3];
    v7 = sub_4A870(0);
    v8 = dcgettext(0, "%s: file list requested from %s already read from %s", 5);
    error(0, 0, v8, v7, v4, v6);
    v9 = v5;
    dword_84D58 = 2;
    v1 = 1;
    sub_4A5B0(0, 58, v9);
  }
  else
  {
LABEL_10:
    v10 = (long long *)sub_4C5B0(32);
    v11 = qword_84698;
    v10[3] = (long long)v4;
    *v10 = v11;
    qword_84698 = (long long)v10;
    v10[1] = v13.st_ino;
    v10[2] = v13.st_dev;
  }
  return v1;
}



// Function: chopslash @ 0x25c70
const char *chopslash(const char *a1)
{
  const char *result; // rax

  result = &a1[strlen(a1) - 1];
  if ( a1 < result )
  {
    do
    {
      if ( *result != 47 )
        break;
      *result-- = 0;
    }
    while ( a1 != result );
  }
  return result;
}



// Function: read_name_from_file @ 0x25ca0
long long read_name_from_file(long long a1)
{
  FILE *v1; // r13
  int v2; // r14d
  int v3; // ebx
  bool v4; // al
  long long i; // rbp
  long long v6; // r12
  unsigned int v7; // r8d
  bool v9; // bp
  const char *v10; // rdi

  ++*(long long *)(a1 + 32);
  v1 = *(FILE **)(a1 + 48);
  v2 = *(int *)(a1 + 40);
  v3 = getc_unlocked(v1);
  v4 = v3 != -1;
  if ( v2 == v3 || v3 == -1 )
  {
    v9 = 1;
    v6 = 0;
  }
  else
  {
    for ( i = 0; ; i = v6 )
    {
      if ( qword_846A0 == i )
      {
        v6 = i + 1;
        qword_846A8 = (void *)sub_4C710(qword_846A8, &qword_846A0);
        *((char *)qword_846A8 + i) = v3;
        if ( !v3 )
          return 2;
      }
      else
      {
        *((char *)qword_846A8 + i) = v3;
        v6 = i + 1;
        if ( !v3 )
          return 2;
      }
      v3 = getc_unlocked(v1);
      if ( v2 == v3 || v3 == -1 )
        break;
    }
    v9 = v6 == 0;
    v4 = v6 == 0 && v3 != -1;
  }
  v7 = 3;
  if ( !v4 )
  {
    v10 = (const char *)qword_846A8;
    if ( qword_846A0 == v6 )
    {
      qword_846A8 = (void *)sub_4C710(qword_846A8, &qword_846A0);
      v10 = (const char *)qword_846A8;
    }
    v10[v6] = 0;
    sub_25C70(v10);
    return v9 & (unsigned char)(v3 == -1);
  }
  return v7;
}



// Function: handle_option @ 0x25dc0
long long handle_option(char *a1, long long a2)
{
  char v2; // bl
  char *v3; // r12
  const unsigned short *v5; // rdx
  unsigned int v6; // r13d
  long long *v7; // rsi
  long long v8; // rdi
  long long v9; // rax
  long long *v10; // rax
  long long *v11; // rdx
  long long v13; // rbx
  char *v14; // rax
  int v15; // [rsp+0h] [rbp-138h] BYREF
  long long v16; // [rsp+8h] [rbp-130h]
  long long v17; // [rsp+10h] [rbp-128h]
  long long v18; // [rsp+20h] [rbp-118h] BYREF
  long long *v19; // [rsp+28h] [rbp-110h]
  long long v20; // [rsp+30h] [rbp-108h]
  unsigned long long v21; // [rsp+108h] [rbp-30h]

  v2 = *a1;
  v21 = __readfsqword(0x28u);
  if ( !v2 )
    return 1;
  v3 = a1;
  v5 = *__ctype_b_loc();
  while ( (v5[v2] & 0x2000) != 0 )
  {
    v2 = *++v3;
    if ( !v2 )
      return 1;
  }
  if ( v2 == 45 )
  {
    v20 = 1;
    v6 = sub_382C0(v3, &v18, 33558086);
    if ( v6 )
    {
      if ( qword_84D50 )
        qword_84D50(v3);
      v13 = sub_354B0(&v18);
      v14 = dcgettext(0, "cannot split string '%s': %s", 5);
      error(0, 0, v14, v3, v13);
      sub_163D0();
    }
    v7 = v19;
    v8 = (unsigned int)(v18 + v20);
    *v19 = qword_84DD0;
    v9 = *(long long *)(a2 + 24);
    v15 = 2;
    v16 = v9;
    v17 = *(long long *)(a2 + 32);
    ((void (*)(long long, long long *, int *))sub_2F5B0)(v8, v7, &v15);
    if ( v20 + v18 )
    {
      v10 = v19;
      v11 = &v19[v20 + v18];
      do
        *v10++ = 0;
      while ( v11 != v10 );
    }
    sub_35420(&v18);
  }
  else
  {
    return 1;
  }
  return v6;
}



// Function: add_exclude_array @ 0x25f40
long long add_exclude_array(long long *a1, unsigned int a2)
{
  long long v3; // rsi
  long long *v4; // rbx
  long long result; // rax

  v3 = *a1;
  if ( *a1 )
  {
    v4 = a1 + 1;
    do
    {
      ++v4;
      result = sub_3EE30(qword_84B60, v3, a2);
      v3 = *(v4 - 1);
    }
    while ( v3 );
  }
  return result;
}



// Function: handle_file_selection_option @ 0x25f80
long long handle_file_selection_option(long long a1, char *a2)
{
  long long result; // rax
  int v4; // esi
  int v5; // esi
  int v6; // esi
  int v7; // r12d
  const char *v8; // rax

  result = (unsigned int)(a1 - 88);
  switch ( (int)a1 )
  {
    case 88:
      result = sub_3F2A0(
                 sub_3EE30,
                 qword_84B60,
                 a2,
                 ((dword_846FC != 1) << 28) | dword_84AD4 | (unsigned int)dword_846F8,
                 10);
      if ( (int)result )
      {
        v7 = *__errno_location();
        if ( qword_84D50 )
          qword_84D50(sub_3EE30);
        v8 = (const char *)sub_4AA70(a2);
        error(0, v7, "%s", v8);
        sub_163D0();
      }
      return result;
    case 257:
      return sub_25F40((long long *)off_7F6A0, 0x10000000u);
    case 258:
      v6 = 1;
      return sub_12A70("CACHEDIR.TAG", v6, (long long)sub_12B70);
    case 259:
      v6 = 2;
      return sub_12A70("CACHEDIR.TAG", v6, (long long)sub_12B70);
    case 260:
      v6 = 3;
      return sub_12A70("CACHEDIR.TAG", v6, (long long)sub_12B70);
    case 261:
      return sub_3EE30(qword_84B60, a2, ((dword_846FC != 1) << 28) | dword_84AD4 | (unsigned int)dword_846F8);
    case 262:
      v5 = 2;
      return sub_16820(a2, v5);
    case 263:
      v5 = 1;
      return sub_16820(a2, v5);
    case 264:
      v4 = 1;
      goto LABEL_25;
    case 265:
      v4 = 2;
      goto LABEL_25;
    case 266:
      v4 = 3;
LABEL_25:
      result = sub_12A70(a2, v4, 0);
      break;
    case 267:
      result = sub_25F40((long long *)off_7F6C0, 0);
      break;
    case 268:
      result = sub_16C40();
      break;
    case 269:
      dword_846F8 |= 0x10u;
      break;
    case 270:
      dword_846F8 &= ~0x10u;
      break;
    case 271:
      dword_846F8 |= 0x40000000u;
      break;
    case 272:
      dword_846F8 &= ~0x40000000u;
      dword_81238 = 0;
      break;
    case 273:
      dword_84AD4 = 8;
      break;
    case 274:
      dword_84AD4 = 0;
      break;
    case 275:
      byte_847E8 = 1;
      break;
    case 276:
      byte_847E8 = 0;
      break;
    case 277:
      byte_84700 = 0;
      break;
    case 278:
      dword_846F8 |= 1u;
      break;
    case 279:
      dword_846FC = 1;
      break;
    case 280:
      byte_8123C = 0;
      byte_84700 = 1;
      break;
    case 281:
      byte_8123C = 10;
      byte_84700 = 0;
      break;
    case 282:
      byte_84700 = 1;
      break;
    case 283:
      dword_846F8 &= ~1u;
      break;
    case 284:
      dword_846FC = 2;
      break;
    default:
      if ( qword_84D50 )
        qword_84D50(a1);
      error(0, 0, "unhandled positional option %d", a1);
      sub_163D0();
  }
  return result;
}



// Function: unconsumed_option_report @ 0x26210
void unconsumed_option_report(long long a1)
{
  char *v1; // rax
  long long *v2; // rax
  long long *v3; // rbx
  long long v4; // rdi
  long long v5; // r15
  int *v6; // r14
  char *v7; // rax
  int v8; // eax
  const char *v9; // rsi
  int *v10; // r14
  char *v11; // rax

  if ( qword_84628 )
  {
    if ( qword_84D50 )
      qword_84D50(a1);
    v1 = dcgettext(
           0,
           "The following options were used after any non-optional arguments in archive create or update mode.  These opt"
           "ions are positional and affect only arguments that follow them.  Please, rearrange them properly.",
           5);
    error(0, 0, v1);
    v2 = qword_84628;
    dword_84D58 = 2;
    do
    {
      v3 = v2;
      v2 = (long long *)v2[1];
    }
    while ( v2 );
    while ( 1 )
    {
      while ( 1 )
      {
        v8 = *((int *)v3 + 4);
        if ( v8 != 1 )
          break;
        if ( qword_84D50 )
          qword_84D50(0);
        v9 = "-C %s has no effect";
        v10 = (int *)sub_4AC50(v3[3]);
LABEL_17:
        v11 = dcgettext(0, v9, 5);
        error(0, 0, v11, v10);
        v3 = (long long *)*v3;
        dword_84D58 = 2;
        if ( !v3 )
        {
LABEL_18:
          sub_25A10();
          return;
        }
      }
      if ( v8 == 4 )
      {
        v4 = v3[4];
        if ( !v4 )
        {
          if ( qword_84D50 )
            qword_84D50(0);
          v9 = "--%s has no effect";
          v10 = sub_25610(*((int *)v3 + 6));
          goto LABEL_17;
        }
        if ( qword_84D50 )
        {
          qword_84D50(v4);
          v4 = v3[4];
        }
        v5 = sub_4AC50(v4);
        v6 = sub_25610(*((int *)v3 + 6));
        v7 = dcgettext(0, "--%s %s has no effect", 5);
        error(0, 0, v7, v6, v5);
        dword_84D58 = 2;
      }
      v3 = (long long *)*v3;
      if ( !v3 )
        goto LABEL_18;
    }
  }
}



// Function: make_name @ 0x263a0
long long make_name(char *a1)
{
  char *v1; // rbp
  long long v2; // r12
  const char *v3; // rax

  v1 = a1;
  v2 = sub_4C760(96);
  if ( !a1 )
    v1 = "";
  v3 = (const char *)sub_4C7A0(v1);
  *(long long *)(v2 + 16) = v3;
  *(long long *)(v2 + 24) = strlen(v3);
  return v2;
}



// Function: regex_usage_warning @ 0x263f0
long long regex_usage_warning(long long a1)
{
  char *v1; // rax
  char *v2; // rax

  if ( dword_846FC )
    return (unsigned int)dword_84634;
  if ( (unsigned char)sub_3EAD0(a1, 0) )
  {
    dword_84634 = 1;
    if ( qword_84D50 )
      qword_84D50(a1);
    v1 = dcgettext(0, "Pattern matching characters used in file names", 5);
    error(0, 0, v1);
    if ( qword_84D50 )
      qword_84D50(0);
    v2 = dcgettext(0, "Use --wildcards to enable pattern matching, or --no-wildcards to suppress this warning", 5);
    error(0, 0, v2);
  }
  return (unsigned int)dword_84634;
}



// Function: name_hash @ 0x26490
long long name_hash(long long a1)
{
  return sub_41AC0(*(long long *)(a1 + 88));
}



// Function: name_next_elt @ 0x267d0
int *name_next_elt(long long a1)
{
  int v1; // ebp
  int *v2; // r12
  int v3; // eax
  char *v5; // rax
  unsigned int v6; // eax

  v1 = a1;
  while ( 1 )
  {
    v2 = qword_846B0;
LABEL_3:
    if ( !qword_846B0 )
      break;
    switch ( *((int *)qword_846B0 + 4) )
    {
      case 0:
        goto LABEL_7;
      case 1:
        if ( !v1 )
        {
LABEL_7:
          sub_26770(*((char **)qword_846B0 + 3));
          if ( byte_847E8 )
            sub_23C70(qword_846A8);
          v3 = v2[4];
          v2 = &unk_84660;
          dword_84670 = v3;
          qword_84678 = (long long)qword_846A8;
          sub_25A50();
          return v2;
        }
        v5 = (char *)sub_4C7A0(*((void **)qword_846B0 + 3));
        v6 = sub_24B00(v5);
        a1 = v6;
        sub_24C80(v6);
        sub_25A50();
        break;
      case 2:
        a1 = (long long)qword_846B0;
        if ( (unsigned int)sub_26500(qword_846B0) )
          continue;
        return &unk_84660;
      case 3:
        goto LABEL_6;
      case 4:
        a1 = *((unsigned int *)qword_846B0 + 6);
        sub_25F80(a1, *((char **)qword_846B0 + 4));
LABEL_6:
        sub_25A50();
        continue;
      default:
        goto LABEL_3;
    }
  }
  sub_26210(a1);
  return v2;
}



// Function: uid_to_uname @ 0x268d0
long long uid_to_uname(__uid_t uid, long long *a2)
{
  char *v4; // rdi
  struct passwd *v5; // rax
  char *pw_name; // rsi
  long long result; // rax

  if ( !uid || dword_846CC != uid )
  {
    v4 = qword_846F0;
    if ( qword_846F0 && dword_846E4 == uid )
      goto LABEL_7;
    v5 = getpwuid(uid);
    if ( v5 )
    {
      pw_name = v5->pw_name;
      dword_846E4 = uid;
      sub_23BE0((void **)&qword_846F0, pw_name);
      v4 = qword_846F0;
LABEL_7:
      result = sub_4C7A0(v4);
      *a2 = result;
      return result;
    }
    dword_846CC = uid;
  }
  result = sub_4C7A0("");
  *a2 = result;
  return result;
}



// Function: gid_to_gname @ 0x26960
long long gid_to_gname(__gid_t gid, long long *a2)
{
  char *v4; // rdi
  struct group *v5; // rax
  char *gr_name; // rsi
  long long result; // rax

  if ( !gid || dword_846C8 != gid )
  {
    v4 = qword_846E8;
    if ( qword_846E8 && dword_846E0 == gid )
      goto LABEL_7;
    v5 = getgrgid(gid);
    if ( v5 )
    {
      gr_name = v5->gr_name;
      dword_846E0 = gid;
      sub_23BE0((void **)&qword_846E8, gr_name);
      v4 = qword_846E8;
LABEL_7:
      result = sub_4C7A0(v4);
      *a2 = result;
      return result;
    }
    dword_846C8 = gid;
  }
  result = sub_4C7A0("");
  *a2 = result;
  return result;
}



// Function: uname_to_uid @ 0x269f0
int uname_to_uid(char *name, int *a2)
{
  int result; // eax
  struct passwd *v4; // rax
  char *pw_name; // rsi

  if ( !qword_846D8 || (result = strcmp(name, qword_846D8)) != 0 )
  {
    if ( qword_846F0 && *name == *qword_846F0 )
    {
      if ( !strcmp(name, qword_846F0) )
        goto LABEL_7;
      v4 = getpwnam(name);
      if ( v4 )
      {
LABEL_6:
        pw_name = v4->pw_name;
        dword_846E4 = v4->pw_uid;
        sub_23BE0((void **)&qword_846F0, pw_name);
LABEL_7:
        *a2 = dword_846E4;
        return 1;
      }
    }
    else
    {
      v4 = getpwnam(name);
      if ( v4 )
        goto LABEL_6;
    }
    sub_23BE0((void **)&qword_846D8, name);
    return 0;
  }
  return result;
}



// Function: gname_to_gid @ 0x26aa0
int gname_to_gid(char *name, int *a2)
{
  int result; // eax
  struct group *v3; // rax

  if ( !qword_846D0 || (result = strcmp(name, qword_846D0)) != 0 )
  {
    if ( qword_846E8 && *name == *qword_846E8 )
    {
      if ( !strcmp(name, qword_846E8) )
        goto LABEL_7;
      v3 = getgrnam(name);
      if ( v3 )
      {
LABEL_6:
        dword_846E0 = v3->gr_gid;
        sub_23BE0((void **)&qword_846E8, name);
LABEL_7:
        *a2 = dword_846E0;
        return 1;
      }
    }
    else
    {
      v3 = getgrnam(name);
      if ( v3 )
        goto LABEL_6;
    }
    sub_23BE0((void **)&qword_846D0, name);
    return 0;
  }
  return result;
}



// Function: name_add_name @ 0x26b50
long long name_add_name(long long a1)
{
  long long *v1; // rax
  long long result; // rax

  v1 = sub_25880();
  *((int *)v1 + 4) = 0;
  v1[3] = a1;
  result = (unsigned int)dword_84630;
  if ( dword_84630 )
  {
    if ( dword_84630 == 1 )
      dword_84630 = 2;
  }
  else
  {
    dword_84630 = 1;
  }
  return result;
}



// Function: names_parse_opt @ 0x26ba0
long long names_parse_opt(int a1, long long a2, long long a3)
{
  unsigned int v3; // r12d
  long long v5; // rax
  long long v6; // rbx
  long long v7; // r13
  char *v8; // rax

  if ( a1 == 256 )
  {
    sub_26B50(a2);
    return 0;
  }
  else
  {
    if ( a1 > 256 )
    {
      if ( a1 == 16777221 )
      {
        v3 = 7;
        v5 = **(long long **)(a3 + 40);
        if ( *(int *)v5 == 2 )
        {
          v6 = *(long long *)(v5 + 16);
          v7 = *(long long *)(v5 + 8);
          v8 = dcgettext(0, "%s:%lu: unrecognized option", 5);
          error(0, 0, v8, v7, v6);
          sub_2F7D0(2);
        }
        return v3;
      }
    }
    else
    {
      if ( a1 == 67 )
      {
        sub_258F0(a2);
        return 0;
      }
      if ( a1 == 84 )
      {
        v3 = 0;
        sub_25910(a2);
        return v3;
      }
    }
    v3 = 7;
    if ( !sub_25620(a1) )
      return v3;
    sub_25940(a1, a2);
    return 0;
  }
}



// Function: name_init @ 0x26c90
void *name_init()
{
  void *v0; // rax

  v0 = (void *)sub_4C5B0(102);
  qword_846A0 = 100;
  qword_846A8 = v0;
  return sub_25630();
}



// Function: name_term @ 0x26cc0
void name_term()
{
  free(qword_846A8);
}



// Function: name_next @ 0x26cd0
int *name_next(long long a1)
{
  int *result; // rax

  result = sub_267D0(a1);
  if ( result )
    return (int *)*((long long *)result + 3);
  return result;
}



// Function: addname @ 0x26cf0
int *addname(char *a1, int a2, char a3, long long a4)
{
  int *result; // rax
  int v8; // edx
  bool v9; // zf
  long long *v10; // rsi

  result = (int *)sub_263A0(a1);
  v8 = dword_84AD4 | dword_846F8 | dword_81238;
  v9 = dword_846FC == 2;
  *(long long *)result = 0;
  v10 = (long long *)qword_846B8;
  result[10] = a2;
  *((long long *)result + 6) = 0;
  *((long long *)result + 1) = v10;
  result[8] = (v9 << 28) | v8;
  *((long long *)result + 7) = 0;
  *((long long *)result + 8) = a4;
  *((char *)result + 36) = a3;
  if ( v10 )
    *v10 = result;
  else
    qword_846C0 = result;
  qword_846B8 = (long long)result;
  return result;
}



// Function: name_gather @ 0x26d90
int *name_gather()
{
  int v0; // ebx
  int i; // ebp
  char *v2; // rdi
  char *v3; // rax
  int *result; // rax
  char *v5; // rax
  int v6; // esi
  int v7; // edx
  int v8; // edx
  bool v9; // zf
  char **v10; // [rsp+8h] [rbp-20h]

  v0 = 0;
  if ( byte_84AAC )
  {
    while ( 1 )
    {
      result = sub_267D0(0);
      if ( !result )
        break;
      if ( result[4] != 1 )
      {
        v10 = (char **)result;
        sub_25AD0((void **)qword_84638);
        result = (int *)sub_263A0(v10[3]);
        v7 = dword_84640;
        *(long long *)result = 0;
        result[10] = v7;
        v8 = dword_84AD4 | dword_846F8 | dword_81238;
        *((long long *)result + 6) = 0;
        v9 = dword_846FC == 2;
        *((long long *)result + 7) = 0;
        *((long long *)result + 8) = 0;
        *((char *)result + 36) = 1;
        qword_84638 = result;
        result[8] = (v9 << 28) | v8;
        qword_846B8 = (long long)result;
        qword_846C0 = result;
        return result;
      }
      v5 = (char *)sub_4C7A0(*((void **)result + 3));
      dword_84640 = sub_24B00(v5);
    }
    v6 = dword_84640;
    if ( !dword_84640 )
      return result;
    return sub_26CF0(0, v6, 0, 0);
  }
LABEL_2:
  for ( i = v0; ; i = sub_24B00(v3) )
  {
    result = sub_267D0(0);
    if ( !result )
      break;
    v2 = (char *)*((long long *)result + 3);
    if ( result[4] != 1 )
    {
      v0 = i;
      sub_26CF0(v2, i, 1, 0);
      goto LABEL_2;
    }
    v3 = (char *)sub_4C7A0(v2);
  }
  if ( v0 != i )
  {
    v6 = i;
    return sub_26CF0(0, v6, 0, 0);
  }
  return result;
}



// Function: add_hierarchy_to_namelist @ 0x26ee0
unsigned long long add_hierarchy_to_namelist(long long a1, long long a2, double a3)
{
  long long *v3; // rax
  long long v4; // rax
  unsigned long long v5; // r15
  const char *v6; // rbx
  long long v7; // rdi
  char *v8; // r13
  char v9; // r14
  size_t v10; // rbp
  int *v12; // rax
  int *v13; // r15
  int v14; // edx
  int v15; // eax
  unsigned long long v16; // [rsp+0h] [rbp-238h]
  int *v17; // [rsp+8h] [rbp-230h]
  int *v19; // [rsp+20h] [rbp-218h]
  int v20; // [rsp+2Ch] [rbp-20Ch]
  unsigned long long v21; // [rsp+38h] [rbp-200h] BYREF
  long long v22[11]; // [rsp+40h] [rbp-1F8h] BYREF
  struct stat buf; // [rsp+98h] [rbp-1A0h] BYREF
  long long v24; // [rsp+1D8h] [rbp-60h]
  int v25; // [rsp+1E8h] [rbp-50h]
  unsigned long long v26; // [rsp+1F8h] [rbp-40h]

  v26 = __readfsqword(0x28u);
  v3 = sub_1F610(a1, a3);
  *(long long *)(a2 + 56) = v3;
  v4 = sub_1FA60((long long)v3);
  if ( v4 )
  {
    v5 = *(long long *)(a2 + 24);
    v6 = (const char *)v4;
    v7 = v5 + 102;
    v16 = v5;
    if ( v5 <= 0x63 )
      v7 = 102;
    v21 = v7;
    v8 = (char *)sub_4C5B0(v7);
    v20 = *(int *)(a2 + 40);
    strcpy(v8, *(const char **)(a2 + 16));
    if ( v8[v5 - 1] != 47 )
    {
      v8[v5] = 47;
      v8[v5 + 1] = 0;
      v16 = v5 + 1;
    }
    v17 = 0;
    v9 = *v6;
    if ( *v6 )
    {
      v19 = 0;
      while ( 1 )
      {
        v10 = strlen(v6);
        if ( v9 == 68 )
          break;
LABEL_9:
        v6 += v10 + 1;
        v9 = *v6;
        if ( !*v6 )
          goto LABEL_10;
      }
      for ( ; v21 < v16 + v10; v8 = (char *)sub_4C710(v8, &v21) )
        ;
      strcpy(&v8[v16], v6 + 1);
      v12 = sub_26CF0(v8, v20, 0, a2);
      v13 = v12;
      if ( v17 )
        *((long long *)v19 + 10) = v12;
      else
        v17 = v12;
      sub_2F630(v22);
      v14 = *(int *)(a1 + 424);
      v24 = a1;
      if ( v14 >= 0 )
      {
        v15 = sub_140D0(a1, (long long)(v6 + 1), dword_84808 | 0x10000u);
        if ( v15 >= 0 )
        {
          v25 = v15;
          if ( fstat(v15, &buf) )
          {
            sub_24EC0((long long)v8);
          }
          else
          {
            v22[0] = sub_4C7A0(v8);
            add_hierarchy_to_namelist(v22, v13);
            sub_14200((long long)v22);
          }
          goto LABEL_19;
        }
      }
      else
      {
        *__errno_location() = -v14;
      }
      sub_24DD0((long long)v8);
LABEL_19:
      sub_2F6F0(v22, a3);
      v19 = v13;
      goto LABEL_9;
    }
LABEL_10:
    free(v8);
    *(long long *)(a2 + 72) = v17;
  }
  return v26 - __readfsqword(0x28u);
}



// Function: remname @ 0x27150
long long *remname(long long *a1)
{
  long long *result; // rax
  long long *v2; // rdx

  result = (long long *)a1[1];
  v2 = (long long *)*a1;
  if ( result )
  {
    *result = v2;
    if ( v2 )
    {
LABEL_3:
      v2[1] = result;
      return result;
    }
  }
  else
  {
    qword_846C0 = (void *)*a1;
    if ( v2 )
      goto LABEL_3;
  }
  qword_846B8 = (long long)result;
  return result;
}



// Function: add_starting_file @ 0x27190
long long add_starting_file(char *a1)
{
  long long *v1; // rbx
  void *v2; // rax
  bool v3; // zf
  int v4; // eax
  int v5; // eax
  long long result; // rax
  void **v7; // rdi

  v1 = (long long *)sub_263A0(a1);
  if ( byte_84A78 )
  {
    v7 = (void **)qword_846C0;
    sub_27150(qword_846C0);
    sub_25AD0(v7);
  }
  v2 = qword_846C0;
  v3 = qword_846B8 == 0;
  v1[1] = 0;
  qword_846C0 = v1;
  *v1 = v2;
  if ( v3 )
    qword_846B8 = (long long)v1;
  v4 = dword_846F8 | dword_81238;
  v1[6] = 0;
  v5 = dword_84AD4 | v4;
  v3 = dword_846FC == 2;
  *((int *)v1 + 10) = 0;
  v1[7] = 0;
  v1[8] = 0;
  result = (v3 << 28) | (unsigned int)v5;
  *((char *)v1 + 36) = 1;
  *((int *)v1 + 8) = result;
  byte_84A78 = 1;
  return result;
}



// Function: name_match @ 0x27250
bool name_match(void *src)
{
  int *v1; // rax
  long long v2; // rax
  long long v3; // rdx
  long long v5; // [rsp+8h] [rbp-10h]

  v1 = qword_846C0;
  while ( 1 )
  {
    if ( !v1 )
      return 1;
    if ( !**((char **)v1 + 2) )
    {
      sub_24C80(v1[10]);
      qword_846C0 = 0;
      qword_846B8 = 0;
      return 1;
    }
    v2 = sub_264A0(src);
    if ( byte_84A78 )
      break;
    if ( v2 )
      goto LABEL_10;
LABEL_3:
    if ( byte_84AAC )
    {
      if ( *((long long *)qword_846C0 + 6) )
      {
        sub_26D90();
        v1 = qword_846C0;
        if ( !*((long long *)qword_846C0 + 6) )
          continue;
      }
    }
    return 0;
  }
  if ( qword_846C0 != (void *)v2 )
    goto LABEL_3;
  byte_84A78 = 0;
  if ( !v2 )
    goto LABEL_3;
LABEL_10:
  v3 = *(long long *)(v2 + 48);
  if ( *((char *)src + *(long long *)(v2 + 24)) != 47 || !dword_84AD4 || !v3 )
    *(long long *)(v2 + 48) = v3 + 1;
  v5 = v2;
  sub_24C80(*(int *)(v2 + 40));
  if ( qword_84B38 )
    return *(long long *)(v5 + 48) == qword_84B38;
  else
    return *(long long *)(v5 + 48) != 0;
}



// Function: all_names_found @ 0x273a0
long long all_names_found(long long a1)
{
  long long v1; // rbx
  unsigned long long v2; // rbp
  unsigned int v3; // r12d
  size_t v4; // rax
  long long *v5; // rdx
  size_t v6; // rcx

  v1 = *(long long *)(a1 + 8);
  if ( v1 )
  {
    v2 = qword_84B38;
    v3 = 0;
    if ( !qword_84B38 )
      return v3;
    v3 = *(unsigned char *)(a1 + 16);
    if ( !(char)v3 )
    {
      v4 = strlen(*(const char **)(a1 + 8));
      v5 = qword_846C0;
      if ( qword_846C0 )
      {
        while ( !*(char *)v5[2] || v2 <= v5[6] )
        {
          v6 = v5[3];
          if ( v6 <= v4 && *(char *)(v1 + v6) == 47 )
            break;
          v5 = (long long *)*v5;
          if ( !v5 )
            return 1;
        }
      }
      else
      {
        return 1;
      }
      return v3;
    }
  }
  return 0;
}



// Function: names_notfound @ 0x27430
void names_notfound()
{
  long long *v0; // rbx
  char *v1; // rdi
  unsigned long long v2; // rdx
  long long v3; // r13
  char *v4; // rdx
  int *v5; // rdi
  long long v6; // r12
  char *v7; // rax
  int *v8; // rax
  int *v9; // rbp

  v0 = qword_846C0;
  if ( qword_846C0 )
  {
    do
    {
      while ( 1 )
      {
        v2 = v0[6];
        if ( qword_84B38 )
          break;
        if ( !v2 )
        {
          v1 = (char *)v0[2];
          if ( *v1 )
            goto LABEL_9;
        }
LABEL_5:
        v0 = (long long *)*v0;
        if ( !v0 )
          goto LABEL_14;
      }
      if ( qword_84B38 <= v2 )
        goto LABEL_5;
      v1 = (char *)v0[2];
      if ( !*v1 )
        goto LABEL_5;
LABEL_9:
      sub_263F0((long long)v1);
      if ( qword_84D50 )
        qword_84D50(v1);
      v3 = sub_4AA70(v0[2]);
      if ( v0[6] )
        v4 = dcgettext(0, "%s: Required occurrence not found in archive", 5);
      else
        v4 = dcgettext(0, "%s: Not found in archive", 5);
      error(0, 0, v4, v3);
      v0 = (long long *)*v0;
      dword_84D58 = 2;
    }
    while ( v0 );
  }
LABEL_14:
  qword_846C0 = 0;
  qword_846B8 = 0;
  if ( byte_84AAC )
  {
    while ( 1 )
    {
      v8 = sub_26CD0(1);
      v9 = v8;
      if ( !v8 )
        break;
      v5 = v8;
      sub_263F0((long long)v8);
      if ( qword_84D50 )
        qword_84D50(v5);
      v6 = sub_4AA70(v9);
      v7 = dcgettext(0, "%s: Not found in archive", 5);
      error(0, 0, v7, v6);
      dword_84D58 = 2;
    }
  }
}



// Function: label_notfound @ 0x275b0
long long *label_notfound()
{
  long long *result; // rax
  unsigned long long v1; // rdx
  long long *v2; // rbx
  char *v3; // rax

  result = qword_846C0;
  if ( qword_846C0 )
  {
    do
    {
      v1 = result[6];
      if ( qword_84B38 )
      {
        if ( qword_84B38 <= v1 )
          return result;
      }
      else if ( v1 )
      {
        return result;
      }
      result = (long long *)*result;
    }
    while ( result );
    if ( dword_84A54 )
    {
      v3 = dcgettext(0, "Archive label mismatch", 5);
      error(0, 0, v3);
    }
    result = (long long *)sub_2F7D0(1);
    v2 = qword_846C0;
    if ( qword_846C0 )
    {
      do
      {
        result = (long long *)sub_263F0(v2[2]);
        if ( (int)result )
          break;
        v2 = (long long *)*v2;
      }
      while ( v2 );
    }
    qword_846C0 = 0;
    qword_846B8 = 0;
    if ( byte_84AAC )
    {
      do
      {
        result = sub_26CD0(1);
        if ( !result )
          break;
        result = (long long *)sub_263F0((long long)result);
      }
      while ( !(int)result );
    }
  }
  return result;
}



// Function: collect_and_sort_names @ 0x276a0
unsigned long long collect_and_sort_names(char *a1, double a2)
{
  long long *v2; // rbp
  int v3; // eax
  int v4; // r12d
  long long v5; // rax
  long long *v6; // rbx
  void *v7; // r14
  int v8; // r13d
  long long v9; // r15
  void **v10; // rax
  void **v11; // r12
  long long v12; // rdi
  long long *v13; // rbp
  long long *v14; // rax
  void *v16; // rdi
  int v17; // eax
  int v18; // eax
  void *v19; // rdi
  long long v20; // rax
  const char *v21; // rsi
  char *v22; // rax
  long long v23[11]; // [rsp+0h] [rbp-1F8h] BYREF
  struct stat buf; // [rsp+58h] [rbp-1A0h] BYREF
  int v25; // [rsp+1A8h] [rbp-50h]
  unsigned long long v26; // [rsp+1B8h] [rbp-40h]

  v26 = __readfsqword(0x28u);
  sub_26D90();
  v2 = qword_846C0;
  if ( qword_846C0 )
  {
    if ( !qword_84B28 )
    {
LABEL_8:
      v4 = 0;
      while ( 1 )
      {
        if ( v2[6] )
          goto LABEL_9;
        if ( v2[7] )
          goto LABEL_9;
        if ( (*((char *)v2 + 35) & 0x10) != 0 )
          goto LABEL_9;
        sub_24C80(*((int *)v2 + 10));
        if ( !*(char *)v2[2] )
          goto LABEL_9;
        sub_2F630(v23);
        if ( sub_246F0((char *)v2[2], &buf) )
        {
          sub_24EC0(v2[2]);
LABEL_9:
          v2 = (long long *)*v2;
          ++v4;
          if ( !v2 )
            goto LABEL_17;
        }
        else
        {
          if ( (buf.st_mode & 0xF000) == 0x4000 )
          {
            v17 = __openat_2((unsigned int)oldfd, v2[2], dword_84808 | 0x10000u);
            if ( v17 < 0 )
            {
              sub_24DD0(v2[2]);
            }
            else
            {
              v25 = v17;
              v18 = fstat(v17, &buf);
              v19 = (void *)v2[2];
              if ( v18 )
              {
                sub_24EC0((long long)v19);
              }
              else
              {
                v20 = sub_4C7A0(v19);
                ++v2[6];
                v23[0] = v20;
                sub_26EE0((long long)v23, (long long)v2, a2);
              }
            }
          }
          ++v4;
          sub_2F6F0(v23, a2);
          v2 = (long long *)*v2;
          if ( !v2 )
          {
LABEL_17:
            v2 = qword_846C0;
            goto LABEL_18;
          }
        }
      }
    }
LABEL_3:
    v3 = sub_24AE0();
    if ( v3 )
    {
      if ( v3 == 1 )
      {
        if ( *((int *)qword_846C0 + 10) )
          goto LABEL_6;
        if ( qword_84D50 )
          qword_84D50(a1);
        v21 = "Using -C option inside file list is not allowed with --listed-incremental";
      }
      else
      {
        if ( qword_84D50 )
          qword_84D50(a1);
        v21 = "Only one -C option is allowed with --listed-incremental";
      }
      v22 = dcgettext(0, v21, 5);
      error(0, 0, v22);
      sub_2C970(2);
    }
LABEL_6:
    sub_1FD80();
    goto LABEL_7;
  }
  a1 = ".";
  sub_26CF0(".", 0, 0, 0);
  if ( qword_84B28 )
    goto LABEL_3;
LABEL_7:
  v2 = qword_846C0;
  if ( qword_846C0 )
    goto LABEL_8;
  v4 = 0;
LABEL_18:
  qword_846C0 = sub_25800(v2, v4, (int (*)(long long *, long long *))sub_259F0);
  v5 = sub_41B30(0, 0, sub_26490, sub_259D0, 0);
  v6 = qword_846C0;
  v7 = (void *)v5;
  if ( qword_846C0 )
  {
    v8 = 0;
    v9 = 0;
    while ( 1 )
    {
      while ( 1 )
      {
        v13 = v6;
        v6 = (long long *)*v6;
        v13[11] = sub_25340(*((int *)v13 + 10), (char *)v13[2]);
        if ( v9 )
        {
          v10 = (void **)sub_418D0(v7, v13);
          v11 = v10;
          if ( v10 )
            break;
        }
LABEL_26:
        v13[6] = 0;
        if ( !sub_42140(v7, v13) )
          sub_16420();
        ++v8;
        v9 = (long long)v13;
        if ( !v6 )
          goto LABEL_28;
      }
      if ( !v13[8] )
      {
        v16 = v10[9];
        if ( v16 )
          sub_266C0(v16, v13[2], v13[3]);
        --v8;
        sub_421A0(v7, v13);
        sub_27150(v11);
        sub_25AD0(v11);
        goto LABEL_26;
      }
      v12 = v13[9];
      if ( v12 )
        sub_266C0(v12, v10[2], v10[3]);
      sub_27150(v13);
      sub_25AD0((void **)v13);
      if ( !v6 )
        goto LABEL_28;
    }
  }
  v9 = 0;
  v8 = 0;
LABEL_28:
  qword_846B8 = v9;
  sub_41CE0(v7);
  v14 = sub_25800(qword_846C0, v8, (int (*)(long long *, long long *))sub_25970);
  qword_846C0 = v14;
  if ( qword_84B28 )
  {
    while ( v14 )
    {
      if ( *(char *)v14[2] )
        return sub_1FAB0(v14[7]);
      v14 = (long long *)*v14;
    }
  }
  return v26 - __readfsqword(0x28u);
}



// Function: name_scan @ 0x27aa0
long long name_scan(void *src)
{
  long long v1; // r12

  do
  {
    v1 = sub_264A0(src);
    if ( v1 )
      break;
    if ( !byte_84AAC )
      break;
    if ( !qword_846C0 )
      break;
    if ( !*((long long *)qword_846C0 + 6) )
      break;
    sub_26D90();
  }
  while ( !*((long long *)qword_846C0 + 6) );
  return v1;
}



// Function: name_from_list @ 0x27b00
long long name_from_list()
{
  void *v0; // rax

  v0 = (void *)qword_84620;
  if ( !qword_84620 )
  {
    v0 = qword_846C0;
    qword_84620 = (long long)qword_846C0;
    if ( !qword_846C0 )
      return 0;
  }
  while ( *((long long *)v0 + 6) || !**((char **)v0 + 2) )
  {
    v0 = *(void **)v0;
    qword_84620 = (long long)v0;
    if ( !v0 )
      return 0;
  }
  *((long long *)v0 + 6) = 1;
  sub_24C80(*((int *)v0 + 10));
  return qword_84620;
}



// Function: blank_name_list @ 0x27b80
long long *blank_name_list()
{
  long long *result; // rax

  result = qword_846C0;
  qword_84620 = 0;
  if ( qword_846C0 )
  {
    do
    {
      result[6] = 0;
      result = (long long *)*result;
    }
    while ( result );
  }
  return result;
}



// Function: make_file_name @ 0x27bc0
char *make_file_name(const char *src, const char *a2)
{
  size_t v2; // rbp
  size_t v3; // rbx
  size_t v4; // r15
  char *v5; // r12

  v2 = 0;
  v3 = strlen(src);
  v4 = strlen(a2) + 1;
  if ( v3 )
  {
    v2 = v3;
    if ( src[v3 - 1] != 47 )
      v2 = v3 + 1;
  }
  v5 = (char *)sub_4C5B0(v4 + v2);
  memcpy(v5, src, v3);
  v5[v3] = 47;
  memcpy(&v5[v2], a2, v4);
  return v5;
}



// Function: stripped_prefix_len @ 0x27c50
long long stripped_prefix_len(char *a1, long long a2)
{
  char v2; // dl
  char *v3; // rax

  v2 = *a1;
  v3 = a1;
  if ( *a1 != 47 )
    goto LABEL_6;
  do
    v2 = *++v3;
  while ( v2 == 47 );
  if ( !v2 )
    return -1;
  while ( 1 )
  {
    ++v3;
    if ( v2 == 47 )
      break;
    v2 = *v3;
LABEL_6:
    if ( !v2 )
      return -1;
  }
  if ( --a2 )
  {
    while ( 1 )
    {
      v2 = *v3;
      if ( *v3 != 47 )
        break;
      ++v3;
    }
    goto LABEL_6;
  }
  return v3 - a1;
}



// Function: contains_dot_dot @ 0x27cc0
bool contains_dot_dot(char *a1)
{
  char v1; // dl
  bool result; // al

  while ( 1 )
  {
    v1 = *a1;
    if ( *a1 == 46 && a1[1] == 46 )
    {
      result = a1[2] == 0 || a1[2] == 47;
      if ( result )
        break;
    }
    while ( 1 )
    {
      ++a1;
      if ( v1 == 47 )
        break;
      if ( !v1 )
        return 0;
      v1 = *a1;
    }
  }
  return result;
}



// Function: tar_sparse_member_p @ 0x27d10
long long tar_sparse_member_p(long long a1)
{
  long long (*v1)(void); // rax

  v1 = *(long long (**)(void))(*(long long *)(a1 + 32) + 16LL);
  if ( v1 )
    return v1();
  else
    return 0;
}



// Function: tar_sparse_done @ 0x27d30
long long tar_sparse_done(long long a1)
{
  long long (*v1)(void); // rax

  v1 = *(long long (**)(void))(*(long long *)(a1 + 32) + 8LL);
  if ( v1 )
    return v1();
  else
    return 1;
}



// Function: tar_sparse_scan @ 0x27d50
long long tar_sparse_scan(long long a1)
{
  long long (*v1)(void); // rax

  v1 = *(long long (**)(void))(*(long long *)(a1 + 32) + 48LL);
  if ( v1 )
    return v1();
  else
    return 1;
}



// Function: tar_sparse_dump_region @ 0x27d70
long long tar_sparse_dump_region(long long a1)
{
  long long (*v1)(void); // rax

  v1 = *(long long (**)(void))(*(long long *)(a1 + 32) + 56LL);
  if ( v1 )
    return v1();
  else
    return 0;
}



// Function: tar_sparse_extract_region @ 0x27d90
long long tar_sparse_extract_region(long long a1)
{
  long long (*v1)(void); // rax

  v1 = *(long long (**)(void))(*(long long *)(a1 + 32) + 64LL);
  if ( v1 )
    return v1();
  else
    return 0;
}



// Function: tar_sparse_decode_header @ 0x27db0
long long tar_sparse_decode_header(long long a1)
{
  long long (*v1)(void); // rax

  v1 = *(long long (**)(void))(*(long long *)(a1 + 32) + 40LL);
  if ( v1 )
    return v1();
  else
    return 1;
}



// Function: tar_sparse_fixup_header @ 0x27dd0
long long tar_sparse_fixup_header(long long a1)
{
  long long (*v1)(void); // rax

  v1 = *(long long (**)(void))(*(long long *)(a1 + 32) + 32LL);
  if ( v1 )
    return v1();
  else
    return 1;
}



// Function: zero_block_p @ 0x27df0
long long zero_block_p(long long a1, long long a2)
{
  long long v2; // rsi

  v2 = a1 + a2;
  do
  {
    if ( a1 == v2 )
      return 1;
    ++a1;
  }
  while ( !*(char *)(a1 - 1) );
  return 0;
}



// Function: sparse_add_map @ 0x27e20
long long sparse_add_map(long long *a1, const __m128i *a2)
{
  long long v3; // rdi
  long long v4; // rbp
  long long v6; // rax

  v3 = a1[40];
  v4 = a1[38];
  if ( a1[39] == v4 )
  {
    v6 = sub_4C680(v3, a1 + 39, 16);
    a1[40] = v6;
    v3 = v6;
  }
  *(__m128i *)(v3 + 16 * v4) = _mm_loadu_si128(a2);
  a1[38] = v4 + 1;
  return 16 * v4;
}



// Function: sparse_select_optab @ 0x27e90
long long sparse_select_optab(long long a1)
{
  unsigned int v1; // eax
  unsigned int v2; // r8d

  v1 = dword_84400;
  if ( !dword_84400 )
    v1 = dword_84BA4;
  if ( v1 == 5 )
  {
    *(long long *)(a1 + 32) = &unk_7F7E0;
    return 1;
  }
  else
  {
    if ( v1 > 5 )
    {
      v2 = 0;
      if ( v1 != 6 )
        return v2;
    }
    else if ( v1 != 2 )
    {
      if ( v1 == 4 )
      {
        v2 = 1;
        *(long long *)(a1 + 32) = &unk_7F780;
      }
      else
      {
        return 0;
      }
      return v2;
    }
    *(long long *)(a1 + 32) = &unk_7F840;
    return 1;
  }
}



// Function: tar_sparse_init @ 0x27f70
long long tar_sparse_init(_OWORD *a1)
{
  long long result; // rax
  long long (*v2)(void); // rdx

  *a1 = 0;
  a1[1] = 0;
  a1[2] = 0;
  result = sub_27E90((long long)a1);
  if ( (char)result )
  {
    v2 = (long long (*)(void))**((long long **)a1 + 4);
    if ( v2 )
      return v2();
  }
  return result;
}



// Function: dump_zeros @ 0x27fb0
long long dump_zeros(long long a1, ssize_t a2)
{
  ssize_t v2; // rax
  size_t v3; // rdx
  ssize_t v4; // rax

  v2 = *(long long *)(a1 + 8);
  if ( v2 > a2 )
  {
    *__errno_location() = 22;
    return 0;
  }
  else if ( v2 < a2 )
  {
    while ( 1 )
    {
      v3 = a2 - v2;
      if ( a2 - v2 > 512 )
        v3 = 512;
      v4 = write(*(int *)a1, &unk_63760, v3);
      if ( v4 <= 0 )
        break;
      v2 = *(long long *)(a1 + 8) + v4;
      *(long long *)(a1 + 8) = v2;
      if ( v2 >= a2 )
        return 1;
    }
    if ( !v4 )
      *__errno_location() = 22;
    return 0;
  }
  else
  {
    return 1;
  }
}



// Function: lseek_or_error @ 0x28050
long long lseek_or_error(long long a1, __off_t a2)
{
  if ( *(char *)(a1 + 4) )
  {
    if ( lseek(*(int *)a1, a2, 0) >= 0 )
      return 1;
  }
  else if ( (unsigned char)sub_27FB0(a1, a2) )
  {
    return 1;
  }
  sub_24E90(**(long long **)(a1 + 24), a2);
  return 0;
}



// Function: sparse_extract_region @ 0x280b0
long long sparse_extract_region(long long a1, long long a2)
{
  long long v2; // rbx
  unsigned int v3; // r14d
  long long v4; // r13
  unsigned int v5; // ebp
  size_t v7; // r12
  char *v8; // rax
  char *v9; // rbp
  int v10; // edi
  unsigned long long v11; // rbp
  char *v12; // rax

  v2 = a1;
  v3 = sub_28050(a1, *(long long *)(*(long long *)(*(long long *)(a1 + 24) + 320LL) + 16 * a2));
  if ( (char)v3 )
  {
    v4 = *(long long *)(*(long long *)(*(long long *)(a1 + 24) + 320LL) + 16 * a2 + 8);
    if ( v4 )
    {
      while ( v4 > 0 )
      {
        v7 = 512;
        if ( v4 <= 512 )
          v7 = v4;
        v8 = (char *)sub_E130();
        v9 = v8;
        if ( !v8 )
        {
          if ( qword_84D50 )
            qword_84D50(a1);
          v12 = dcgettext(0, "Unexpected EOF in archive", 5);
          error(0, 0, v12);
          dword_84D58 = 2;
          return 0;
        }
        sub_D1F0((unsigned long long)v8);
        v10 = *(int *)v2;
        *(long long *)(v2 + 16) += 512LL;
        v11 = sub_249F0(v10, v9, v7);
        v4 -= v11;
        a1 = *(long long *)(*(long long *)(v2 + 24) + 280LL) - *(long long *)(v2 + 16);
        sub_CCF0(a1);
        *(long long *)(v2 + 8) += v11;
        if ( v7 != v11 )
        {
          sub_32E30(**(long long **)(v2 + 24), v11, v7);
          return 0;
        }
      }
    }
    else
    {
      v5 = *(unsigned char *)(a1 + 4);
      if ( (char)v5 )
      {
        if ( (unsigned int)sub_2AED0(*(int *)a1) )
        {
          v3 = v5;
          sub_32D90(**(long long **)(a1 + 24));
        }
      }
    }
  }
  return v3;
}



// Function: star_fixup_header @ 0x28220
long long star_fixup_header(long long a1)
{
  long long v1; // rax
  long long v2; // rdx
  long long v3; // rcx

  v1 = sub_219B0(qword_84408 + 452, 0xCu);
  v2 = *(long long *)(a1 + 24);
  *(long long *)(v2 + 280) = *(long long *)(v2 + 136);
  v3 = 0;
  if ( v1 >= 0 )
    v3 = v1;
  *(long long *)(v2 + 136) = v3;
  return v1 >= 0;
}



// Function: oldgnu_fixup_header @ 0x28270
long long oldgnu_fixup_header(long long a1)
{
  long long v1; // rax
  long long v2; // rdx
  long long v3; // rcx

  v1 = sub_219B0(qword_84408 + 483, 0xCu);
  v2 = *(long long *)(a1 + 24);
  *(long long *)(v2 + 280) = *(long long *)(v2 + 136);
  v3 = 0;
  if ( v1 >= 0 )
    v3 = v1;
  *(long long *)(v2 + 136) = v3;
  return v1 >= 0;
}



// Function: oldgnu_add_sparse @ 0x282c0
long long oldgnu_add_sparse(long long a1, long long a2)
{
  unsigned int v2; // r8d
  long long v4; // rax
  long long *v5; // rdi
  __m128i v6; // [rsp+0h] [rbp-38h] BYREF
  unsigned long long v7; // [rsp+18h] [rbp-20h]

  v2 = 1;
  v7 = __readfsqword(0x28u);
  if ( *(char *)(a2 + 12) )
  {
    v6.m128i_i64[0] = sub_219B0(a2, 0xCu);
    v4 = sub_219B0(a2 + 12, 0xCu);
    v2 = 2;
    v6.m128i_i64[1] = v4;
    if ( (v6.m128i_i64[0] | v4) >= 0 && !__OFADD__(v4, v6.m128i_i64[0]) )
    {
      v5 = *(long long **)(a1 + 24);
      if ( v5[17] >= v4 + v6.m128i_i64[0] && (long long)v5[35] >= 0 )
      {
        sub_27E20(v5, &v6);
        return 0;
      }
    }
  }
  return v2;
}



// Function: star_get_sparse_info @ 0x28370
long long star_get_sparse_info(long long a1)
{
  long long v1; // r12
  long long v2; // rbp
  bool v3; // zf
  char *v4; // rax
  char *v5; // r13
  long long v6; // rbp
  unsigned long long v7; // rbx
  int v8; // eax
  long long v10; // rbx
  char *v11; // rax
  long long v12; // r12
  char *v13; // rax

  v1 = a1;
  v2 = qword_84408;
  v3 = *(char *)(qword_84408 + 345) == 0;
  *(long long *)(*(long long *)(a1 + 24) + 304LL) = 0;
  if ( v3 && *(char *)(v2 + 366) )
  {
    v10 = v2 + 356;
    while ( 1 )
    {
      a1 = v1;
      v8 = sub_282C0(v1, v10);
      if ( v8 )
        break;
      v10 += 24;
      if ( v2 + 452 == v10 )
      {
        if ( *(char *)(v2 + 355) )
        {
          v4 = sub_E130();
          v5 = v4;
          if ( v4 )
            goto LABEL_4;
          goto LABEL_16;
        }
        return 1;
      }
    }
LABEL_9:
    if ( v8 != 2 )
      return 1;
    if ( qword_84D50 )
      qword_84D50(a1);
    v12 = **(long long **)(v1 + 24);
    v13 = dcgettext(0, "%s: invalid sparse archive member", 5);
    error(0, 0, v13, v12);
    dword_84D58 = 2;
    return 0;
  }
  else
  {
    while ( 1 )
    {
      v4 = sub_E130();
      v5 = v4;
      if ( !v4 )
        break;
LABEL_4:
      v6 = (long long)v5;
      v7 = 0;
      sub_D1F0((unsigned long long)v4);
      do
      {
        a1 = v1;
        ++v7;
        v8 = sub_282C0(v1, v6);
        v6 += 24;
      }
      while ( v7 <= 0x14 && !v8 );
      *(long long *)(v1 + 16) += 512LL;
      if ( !v5[504] || v8 )
        goto LABEL_9;
    }
LABEL_16:
    if ( qword_84D50 )
      qword_84D50(a1);
    v11 = dcgettext(0, "Unexpected EOF in archive", 5);
    error(0, 0, v11);
    dword_84D58 = 2;
    return 0;
  }
}



// Function: oldgnu_get_sparse_info @ 0x28500
long long oldgnu_get_sparse_info(long long a1)
{
  long long v2; // r13
  long long v3; // rbx
  long long v4; // rbp
  long long v5; // rdi
  int v6; // eax
  char *v7; // rax
  char *v8; // r13
  long long v9; // rbp
  unsigned long long v10; // rbx
  char *v12; // rax
  long long v13; // r12
  char *v14; // rax

  v2 = qword_84408;
  v3 = qword_84408 + 386;
  v4 = qword_84408 + 482;
  *(long long *)(*(long long *)(a1 + 24) + 304LL) = 0;
  do
  {
    v5 = a1;
    v6 = sub_282C0(a1, v3);
    if ( v6 )
    {
LABEL_11:
      if ( v6 == 2 )
      {
        if ( qword_84D50 )
          qword_84D50(v5);
        v13 = **(long long **)(a1 + 24);
        v14 = dcgettext(0, "%s: invalid sparse archive member", 5);
        error(0, 0, v14, v13);
        dword_84D58 = 2;
        return 0;
      }
      return 1;
    }
    v3 += 24;
  }
  while ( v3 != v4 );
  if ( !*(char *)(v2 + 482) )
    return 1;
  while ( 1 )
  {
    v7 = sub_E130();
    v8 = v7;
    if ( !v7 )
      break;
    v9 = (long long)v7;
    v10 = 0;
    sub_D1F0((unsigned long long)v7);
    do
    {
      v5 = a1;
      ++v10;
      v6 = sub_282C0(a1, v9);
      v9 += 24;
    }
    while ( v10 <= 0x14 && !v6 );
    if ( !v8[504] || v6 )
      goto LABEL_11;
  }
  if ( qword_84D50 )
    qword_84D50(v5);
  v12 = dcgettext(0, "Unexpected EOF in archive", 5);
  error(0, 0, v12);
  dword_84D58 = 2;
  return 0;
}



// Function: sparse_dump_region @ 0x28660
long long sparse_dump_region(unsigned int *a1, long long a2)
{
  __off_t *v3; // rsi
  long long v4; // rbx
  size_t v5; // r12
  char *v6; // r15
  long long v7; // rax
  long long v8; // r14
  long long v10; // rdi
  unsigned long long v11; // r12
  long long v12; // rbx
  long long v13; // r13
  char *v14; // rax
  unsigned char v15; // [rsp+7h] [rbp-F1h]
  long long v16; // [rsp+8h] [rbp-F0h]
  struct stat buf; // [rsp+10h] [rbp-E8h] BYREF
  char v18[24]; // [rsp+A0h] [rbp-58h] BYREF
  unsigned long long v19; // [rsp+B8h] [rbp-40h]

  v19 = __readfsqword(0x28u);
  v16 = 16 * a2;
  v3 = (__off_t *)(*(long long *)(*((long long *)a1 + 3) + 320LL) + 16 * a2);
  v4 = v3[1];
  v15 = sub_28050((long long)a1, *v3);
  if ( v15 && v4 > 0 )
  {
    while ( 1 )
    {
      v5 = 512;
      if ( v4 <= 512 )
        v5 = v4;
      v6 = (char *)sub_E130();
      v7 = sub_4AF10(*a1, v6, v5);
      v8 = v7;
      if ( v7 == -1 )
      {
        sub_24E00();
        return 0;
      }
      if ( !v7 )
        break;
      v4 -= v7;
      memset(&v6[v7], 0, 512 - v7);
      *((long long *)a1 + 2) += v8;
      sub_D1F0((unsigned long long)v6);
      if ( v4 <= 0 )
        return v15;
    }
    v10 = *a1;
    if ( fstat(v10, &buf) )
      v11 = *(long long *)(*((long long *)a1 + 3) + 136LL)
          + v4
          - (*(long long *)(*(long long *)(*((long long *)a1 + 3) + 320LL) + v16)
           + *(long long *)(*(long long *)(*((long long *)a1 + 3) + 320LL) + v16 + 8));
    else
      v11 = *(long long *)(*((long long *)a1 + 3) + 136LL) - buf.st_size;
    if ( (dword_83B80 & 0x80u) != 0 )
    {
      if ( qword_84D50 )
        qword_84D50(v10);
      v12 = sub_43090(v11, v18);
      v13 = sub_4AA70(**((long long **)a1 + 3));
      v14 = dcngettext(
              0,
              "%s: File shrank by %s byte; padding with zeros",
              "%s: File shrank by %s bytes; padding with zeros",
              v11,
              5);
      error(0, 0, v14, v13, v12);
    }
    if ( !byte_84B52 )
      sub_2F7D0(1);
    return 0;
  }
  return v15;
}



// Function: decode_num @ 0x28870
long long decode_num(uintmax_t *a1, const char *a2, uintmax_t a3)
{
  int *v5; // rax
  int *v6; // r12
  uintmax_t v7; // rax
  char *v8[7]; // [rsp+0h] [rbp-38h] BYREF

  v8[1] = (char *)__readfsqword(0x28u);
  if ( (unsigned int)(*a2 - 48) > 9 )
    return 0;
  v5 = __errno_location();
  *v5 = 0;
  v6 = v5;
  v7 = strtoumax(a2, v8, 10);
  if ( v7 > a3 || *v6 == 34 || *v8[0] )
    return 0;
  *a1 = v7;
  return 1;
}



// Function: pax_decode_header @ 0x28910
long long pax_decode_header(long long a1, long long a2, long long a3)
{
  unsigned int v3; // r15d
  long long v6; // rax
  long long *v7; // rdi
  char *v8; // rax
  unsigned long long v9; // r8
  char *v10; // rbx
  char *v11; // rcx
  char *v12; // r13
  char v13; // al
  char *v14; // rax
  const char *v15; // rsi
  long long v16; // r12
  char *v17; // rax
  long long v18; // r14
  long long v19; // rsi
  long long v20; // rax
  unsigned long long v21; // r8
  long long v22; // rax
  bool v23; // zf
  char *v24; // rcx
  char *v25; // rdx
  char v26; // al
  char *v27; // rax
  char v28; // al
  unsigned long long v29; // r8
  char *v30; // rcx
  char *v31; // rdx
  char v32; // al
  char *v33; // rax
  unsigned long long v34; // rdi
  long long v35; // rdx
  char *v36; // rax
  long long v37; // [rsp+8h] [rbp-A0h]
  unsigned long long v38; // [rsp+10h] [rbp-98h]
  char *v39; // [rsp+10h] [rbp-98h]
  unsigned long long v40; // [rsp+10h] [rbp-98h]
  char *v41; // [rsp+10h] [rbp-98h]
  unsigned long long v42; // [rsp+10h] [rbp-98h]
  char *v43; // [rsp+18h] [rbp-90h]
  char *v44; // [rsp+18h] [rbp-90h]
  char *v45; // [rsp+18h] [rbp-90h]
  unsigned long long v46; // [rsp+20h] [rbp-88h]
  long long v47; // [rsp+38h] [rbp-70h] BYREF
  __m128i v48; // [rsp+40h] [rbp-68h] BYREF
  char v49[20]; // [rsp+50h] [rbp-58h] BYREF
  char v50[4]; // [rsp+64h] [rbp-44h] BYREF
  unsigned long long v51; // [rsp+68h] [rbp-40h]

  v3 = 1;
  v51 = __readfsqword(0x28u);
  if ( *(int *)(*(long long *)(a1 + 24) + 292LL) )
  {
    v6 = sub_D180(a1, a2, a3);
    v7 = (long long *)qword_84408;
    v37 = v6;
    sub_D1F0(qword_84408);
    v8 = (char *)sub_E130();
    v9 = (unsigned long long)v8;
    if ( !v8 )
    {
LABEL_36:
      if ( qword_84D50 )
        qword_84D50(v7);
      v36 = dcgettext(0, "Unexpected EOF in archive", 5);
      error(0, 0, v36);
      sub_163D0();
    }
    v10 = v8;
    v11 = v8 + 512;
    v12 = v49;
    while ( 1 )
    {
      v14 = v10;
      if ( v10 == v11 )
      {
        v7 = (long long *)v9;
        sub_D1F0(v9);
        v14 = (char *)sub_E130();
        v9 = (unsigned long long)v14;
        if ( !v14 )
          goto LABEL_36;
        v11 = v14 + 512;
      }
      v10 = v14 + 1;
      v13 = *v14;
      *v12++ = v13;
      if ( v13 == 10 )
        break;
      if ( v12 == v50 )
      {
LABEL_10:
        if ( qword_84D50 )
          qword_84D50(v7);
        v15 = "%s: numeric overflow in sparse archive member";
        v16 = **(long long **)(a1 + 24);
LABEL_13:
        v3 = 0;
        v17 = dcgettext(0, v15, 5);
        error(0, 0, v17, v16);
        dword_84D58 = 2;
        return v3;
      }
    }
    *(v12 - 1) = 0;
    v43 = v11;
    v38 = v9;
    v3 = sub_28870((uintmax_t *)&v47, v49, 0xFFFFFFFFFFFFFFFFLL);
    if ( (char)v3 )
    {
      v18 = *(long long *)(a1 + 24);
      v7 = (long long *)v47;
      v19 = 16;
      *(long long *)(v18 + 312) = v47;
      v20 = sub_4C720(v7, 16);
      v21 = v38;
      *(long long *)(v18 + 320) = v20;
      v22 = *(long long *)(a1 + 24);
      v23 = *(long long *)(v22 + 312) == 0;
      *(long long *)(v22 + 304) = 0;
      if ( v23 )
      {
LABEL_35:
        v34 = v21;
        sub_D1F0(v21);
        *(long long *)(a1 + 16) += (sub_D180(v34, v19, v35) - v37) << 9;
        return v3;
      }
      v24 = v43;
      v46 = 0;
      while ( 1 )
      {
        v25 = v49;
        while ( 1 )
        {
          v27 = v10;
          if ( v10 == v24 )
          {
            v7 = (long long *)v21;
            v39 = v25;
            sub_D1F0(v21);
            v27 = (char *)sub_E130();
            v25 = v39;
            v21 = (unsigned long long)v27;
            if ( !v27 )
              goto LABEL_36;
            v24 = v27 + 512;
          }
          v10 = v27 + 1;
          v26 = *v27;
          *v25++ = v26;
          if ( v26 == 10 )
            break;
          if ( v25 == v50 )
            goto LABEL_10;
        }
        *(v25 - 1) = 0;
        v7 = &v47;
        v44 = v24;
        v40 = v21;
        v28 = sub_28870((uintmax_t *)&v47, v49, 0x7FFFFFFFFFFFFFFFuLL);
        v29 = v40;
        v30 = v44;
        if ( !v28 )
          break;
        v31 = v49;
        v48.m128i_i64[0] = v47;
        while ( 1 )
        {
          v33 = v10;
          if ( v10 == v30 )
          {
            v7 = (long long *)v29;
            v41 = v31;
            sub_D1F0(v29);
            v33 = (char *)sub_E130();
            v31 = v41;
            v29 = (unsigned long long)v33;
            if ( !v33 )
              goto LABEL_36;
            v30 = v33 + 512;
          }
          v10 = v33 + 1;
          v32 = *v33;
          *v31++ = v32;
          if ( v32 == 10 )
            break;
          if ( v31 == v50 )
            goto LABEL_10;
        }
        *(v31 - 1) = 0;
        v45 = v30;
        v42 = v29;
        if ( !(unsigned char)sub_28870((uintmax_t *)&v47, v49, 0x7FFFFFFFFFFFFFFFuLL) )
          break;
        v7 = *(long long **)(a1 + 24);
        v48.m128i_i64[1] = v47;
        sub_27E20(v7, &v48);
        v19 = ++v46;
        v21 = v42;
        v24 = v45;
        if ( *(long long *)(*(long long *)(a1 + 24) + 312LL) <= v46 )
          goto LABEL_35;
      }
    }
    if ( qword_84D50 )
      qword_84D50(&v47);
    v15 = "%s: malformed sparse archive member";
    v16 = **(long long **)(a1 + 24);
    goto LABEL_13;
  }
  return v3;
}



// Function: pax_start_header @ 0x28ce0
long long pax_start_header(long long a1)
{
  long long v1; // rbp
  long long result; // rax

  v1 = *(long long *)(a1 + 136);
  *(long long *)(a1 + 136) = *(long long *)(a1 + 280);
  result = sub_132D0(a1);
  *(long long *)(a1 + 136) = v1;
  return result;
}



// Function: pax_dump_header_0 @ 0x28d20
long long pax_dump_header_0(long long a1, long long a2, long long a3)
{
  long long v4; // rax
  long long v5; // rsi
  long long v6; // r12
  long long v7; // rbp
  long long v8; // rsi
  unsigned int v9; // r13d
  long long v10; // rax
  long long *v12; // r13
  long long v13; // rdi
  unsigned long long v14; // rax
  const char *v15; // rax
  const char *v16; // rax
  long long v17; // rax
  long long v18; // [rsp+8h] [rbp-70h]
  unsigned long long v19; // [rsp+18h] [rbp-60h] BYREF
  char v20[24]; // [rsp+20h] [rbp-58h] BYREF
  unsigned long long v21; // [rsp+38h] [rbp-40h]

  v21 = __readfsqword(0x28u);
  v4 = sub_D180(a1, a2, a3);
  v5 = *(long long *)(a1 + 24);
  v6 = v4;
  v7 = *(long long *)(v5 + 320);
  sub_1CA30("GNU.sparse.size", v5, 0);
  sub_1CA30("GNU.sparse.numblocks", *(long long *)(a1 + 24), 0);
  if ( !(unsigned char)sub_1BEF0("GNU.sparse.map") && dword_84A80 )
  {
    sub_1CA30("GNU.sparse.name", *(long long *)(a1 + 24), 0);
    v12 = *(long long **)(a1 + 24);
    v18 = v12[1];
    v12[1] = sub_1C080(v12, "%d/GNUSparseFile.%p/%f", 0);
    sub_1CF40(*(long long *)(a1 + 24) + 360LL);
    v13 = *(long long *)(a1 + 24);
    v14 = 0;
    v19 = 0;
    if ( *(long long *)(v13 + 304) )
    {
      while ( 1 )
      {
        v15 = (const char *)sub_43090(*(long long *)(v7 + 16 * v14), v20);
        sub_1CF50((long long *)(*(long long *)(a1 + 24) + 360LL), v15);
        sub_1CF50((long long *)(*(long long *)(a1 + 24) + 360LL), ",");
        v16 = (const char *)sub_43090(*(long long *)(v7 + 16 * v19 + 8), v20);
        sub_1CF50((long long *)(*(long long *)(a1 + 24) + 360LL), v16);
        v13 = *(long long *)(a1 + 24);
        v14 = v19 + 1;
        v19 = v14;
        if ( v14 >= *(long long *)(v13 + 304) )
          break;
        if ( v14 )
        {
          sub_1CF50((long long *)(v13 + 360), ",");
          v14 = v19;
        }
      }
    }
    v9 = sub_1CFA0((long long *)(v13 + 360), "GNU.sparse.map");
    if ( !(char)v9 || (v17 = sub_28CE0(*(long long *)(a1 + 24)), sub_12F60(*(long long **)(a1 + 24), v17, v6), v18) )
    {
      free(*(void **)(*(long long *)(a1 + 24) + 8LL));
      *(long long *)(*(long long *)(a1 + 24) + 8LL) = v18;
    }
  }
  else
  {
    v19 = 0;
    v8 = *(long long *)(a1 + 24);
    if ( *(long long *)(v8 + 304) )
    {
      do
      {
        sub_1CA30("GNU.sparse.offset", v8, (long long)&v19);
        sub_1CA30("GNU.sparse.numbytes", *(long long *)(a1 + 24), (long long)&v19);
        v8 = *(long long *)(a1 + 24);
        ++v19;
      }
      while ( v19 < *(long long *)(v8 + 304) );
    }
    v9 = 1;
    v10 = sub_28CE0(v8);
    sub_12F60(*(long long **)(a1 + 24), v10, v6);
  }
  return v9;
}



// Function: pax_dump_header_1 @ 0x28fd0
long long pax_dump_header_1(long long a1, long long a2, long long a3)
{
  long long *v4; // rax
  long long *v5; // r14
  const char *v6; // rax
  size_t v7; // rax
  long long v8; // rsi
  size_t v9; // rbp
  long long *v10; // r15
  unsigned long long v11; // rbx
  long long v12; // rdi
  const char *v13; // rax
  size_t v14; // r12
  const char *v15; // rax
  size_t v16; // rax
  long long v17; // rax
  long long v18; // rbp
  unsigned long long v19; // rax
  long long *v20; // rbp
  long long v21; // rbp
  long long v22; // rbx
  long long v23; // rax
  char *v24; // r12
  long long v25; // rax
  char *v26; // r8
  char *v27; // rbp
  char *i; // rcx
  char v29; // dl
  char *v30; // rax
  char *v31; // rbp
  unsigned long long v32; // r12
  long long v33; // rax
  char *v34; // r8
  char *v35; // rbx
  char *j; // rcx
  char v37; // si
  char *v38; // rax
  char *v39; // r15
  long long v40; // rax
  char *v41; // rcx
  char *k; // rbx
  char v43; // si
  char *v44; // rax
  long long v46; // [rsp+8h] [rbp-70h]
  char *v47; // [rsp+8h] [rbp-70h]
  char *v48; // [rsp+8h] [rbp-70h]
  unsigned long long v49; // [rsp+8h] [rbp-70h]
  long long v50; // [rsp+10h] [rbp-68h]
  char *v51; // [rsp+10h] [rbp-68h]
  char v52[24]; // [rsp+20h] [rbp-58h] BYREF
  unsigned long long v53; // [rsp+38h] [rbp-40h]

  v53 = __readfsqword(0x28u);
  v46 = sub_D180(a1, a2, a3);
  v4 = *(long long **)(a1 + 24);
  v5 = (long long *)v4[40];
  v50 = v4[1];
  v6 = (const char *)sub_43090(v4[38], v52);
  v7 = strlen(v6);
  v8 = *(long long *)(a1 + 24);
  v9 = v7 + 1;
  if ( *(long long *)(v8 + 304) )
  {
    v10 = v5;
    v11 = 0;
    do
    {
      v12 = *v10;
      ++v11;
      v10 += 2;
      v13 = (const char *)sub_43090(v12, v52);
      v14 = strlen(v13);
      v15 = (const char *)sub_43090(*(v10 - 1), v52);
      v16 = strlen(v15);
      v8 = *(long long *)(a1 + 24);
      v9 += v16 + v14 + 2;
    }
    while ( *(long long *)(v8 + 304) > v11 );
  }
  v17 = v9 + 1022;
  v18 = v9 + 511;
  if ( v18 >= 0 )
    v17 = v18;
  v19 = v17 & 0xFFFFFFFFFFFFFE00LL;
  *(long long *)(v8 + 280) += v19;
  *(long long *)(a1 + 16) += v19;
  sub_1CA30("GNU.sparse.major", v8, 0);
  sub_1CA30("GNU.sparse.minor", *(long long *)(a1 + 24), 0);
  sub_1CA30("GNU.sparse.name", *(long long *)(a1 + 24), 0);
  sub_1CA30("GNU.sparse.realsize", *(long long *)(a1 + 24), 0);
  v20 = *(long long **)(a1 + 24);
  v20[1] = sub_1C080(v20, "%d/GNUSparseFile.%p/%f", 0);
  v21 = *(long long *)(a1 + 24);
  v22 = *(long long *)(v21 + 8);
  if ( strlen((const char *)v22) > 0x64 )
  {
    *(char *)(v22 + 100) = 0;
    v21 = *(long long *)(a1 + 24);
  }
  v23 = sub_28CE0(v21);
  sub_12F60(*(long long **)(a1 + 24), v23, v46);
  free(*(void **)(*(long long *)(a1 + 24) + 8LL));
  *(long long *)(*(long long *)(a1 + 24) + 8LL) = v50;
  v24 = sub_E130();
  v25 = sub_43090(*(long long *)(*(long long *)(a1 + 24) + 304LL), v52);
  v26 = v24;
  v27 = (char *)v25;
  for ( i = v24 + 512; *v27; *v30 = v29 )
  {
    v30 = v24;
    if ( v24 == i )
    {
      sub_D1F0((unsigned long long)v26);
      v30 = sub_E130();
      v26 = v30;
      i = v30 + 512;
    }
    v29 = *v27++;
    v24 = v30 + 1;
  }
  if ( i == v24 )
  {
    sub_D1F0((unsigned long long)v26);
    v26 = sub_E130();
    v24 = v26;
  }
  *v24 = 10;
  v31 = v24 + 1;
  if ( *(long long *)(*(long long *)(a1 + 24) + 304LL) )
  {
    v32 = 0;
    do
    {
      v47 = v26;
      v33 = sub_43090(*v5, v52);
      v34 = v47;
      v35 = (char *)v33;
      for ( j = v47 + 512; *v35; *v38 = v37 )
      {
        v38 = v31;
        if ( v31 == j )
        {
          sub_D1F0((unsigned long long)v34);
          v38 = sub_E130();
          v34 = v38;
          j = v38 + 512;
        }
        v37 = *v35++;
        v31 = v38 + 1;
      }
      if ( j == v31 )
      {
        sub_D1F0((unsigned long long)v34);
        v34 = sub_E130();
        v31 = v34;
        j = v34 + 512;
      }
      *v31 = 10;
      v39 = v31 + 1;
      v51 = j;
      v48 = v34;
      v40 = sub_43090(v5[1], v52);
      v26 = v48;
      v41 = v51;
      for ( k = (char *)v40; *k; *v44 = v43 )
      {
        v44 = v39;
        if ( v39 == v41 )
        {
          sub_D1F0((unsigned long long)v26);
          v44 = sub_E130();
          v26 = v44;
          v41 = v44 + 512;
        }
        v43 = *k++;
        v39 = v44 + 1;
      }
      if ( v41 == v39 )
      {
        sub_D1F0((unsigned long long)v26);
        v26 = sub_E130();
        v39 = v26;
      }
      *v39 = 10;
      v31 = v39 + 1;
      ++v32;
      v5 += 2;
    }
    while ( *(long long *)(*(long long *)(a1 + 24) + 304LL) > v32 );
  }
  v49 = (unsigned long long)v26;
  memset(v31, 0, v26 - v31 + 512);
  sub_D1F0(v49);
  return 1;
}



// Function: pax_dump_header @ 0x29390
long long pax_dump_header(long long a1, long long a2)
{
  long long v2; // rdx
  int v3; // eax
  int v4; // ecx

  v2 = *(long long *)(a1 + 24);
  v3 = dword_84A84;
  v4 = dword_84A80;
  *(int *)(v2 + 292) = dword_84A84;
  *(int *)(v2 + 296) = v4;
  if ( v3 )
    return sub_28FD0(a1, a2, v2);
  else
    return sub_28D20(a1, a2, v2);
}



// Function: oldgnu_store_sparse_info @ 0x293d0
unsigned long long oldgnu_store_sparse_info(long long a1, unsigned long long *a2, char *a3, long long a4)
{
  long long v5; // rdx
  unsigned long long result; // rax
  char *v9; // rsi

  v5 = *(long long *)(a1 + 24);
  result = *a2;
  if ( *a2 < *(long long *)(v5 + 304) )
  {
    do
    {
      --a4;
      sub_12BE0(*(long long *)(*(long long *)(v5 + 320) + 16 * result), a3, 12);
      v9 = a3 + 12;
      a3 += 24;
      sub_12BE0(*(long long *)(*(long long *)(*(long long *)(a1 + 24) + 320LL) + 16 * *a2 + 8), v9, 12);
      v5 = *(long long *)(a1 + 24);
      result = *a2 + 1;
      *a2 = result;
    }
    while ( result < *(long long *)(v5 + 304) && a4 );
  }
  return result;
}



// Function: oldgnu_dump_header @ 0x29480
long long oldgnu_dump_header(long long a1, long long a2, long long a3)
{
  long long v4; // r13
  long long v5; // rax
  char *v6; // r12
  long long v7; // rax
  long long *v8; // rdi
  unsigned long long v9; // r14
  char *v10; // rax
  unsigned long long v11; // r13
  unsigned long long v12; // r14
  unsigned long long v14[7]; // [rsp+0h] [rbp-38h] BYREF

  v14[1] = __readfsqword(0x28u);
  v4 = sub_D180(a1, a2, a3);
  v5 = sub_132D0(*(long long *)(a1 + 24));
  *(char *)(v5 + 156) = 83;
  v6 = (char *)v5;
  v7 = *(long long *)(a1 + 24);
  if ( *(long long *)(v7 + 304) > 4u )
    v6[482] = 1;
  sub_12BE0(*(long long *)(v7 + 136), v6 + 483, 12);
  sub_12BE0(*(long long *)(*(long long *)(a1 + 24) + 280LL), v6 + 124, 12);
  v14[0] = 0;
  sub_293D0(a1, v14, v6 + 386, 4);
  v8 = *(long long **)(a1 + 24);
  v9 = v14[0];
  v6[482] = v8[38] > v14[0];
  sub_12F60(v8, (long long)v6, v4);
  if ( v9 < *(long long *)(*(long long *)(a1 + 24) + 304LL) )
  {
    do
    {
      v10 = (char *)sub_E130();
      *(long long *)v10 = 0;
      v11 = (unsigned long long)v10;
      *((long long *)v10 + 63) = 0;
      memset(
        (void *)((unsigned long long)(v10 + 8) & 0xFFFFFFFFFFFFFFF8LL),
        0,
        8LL * (((unsigned int)v10 - (((int)v10 + 8) & 0xFFFFFFF8) + 512) >> 3));
      sub_293D0(a1, v14, v10, 21);
      v12 = v14[0];
      if ( *(long long *)(*(long long *)(a1 + 24) + 304LL) > v14[0] )
        *(char *)(v11 + 504) = 1;
      sub_D1F0(v11);
    }
    while ( v12 < *(long long *)(*(long long *)(a1 + 24) + 304LL) );
  }
  return 1;
}



// Function: sparse_scan_file_raw @ 0x29600
long long sparse_scan_file_raw(long long a1)
{
  long long v1; // rbp
  long long *v2; // r12
  int v3; // r14d
  long long v4; // rax
  long long v5; // r15
  long long v6; // rdx
  __m128i v8; // [rsp+10h] [rbp-258h] BYREF
  char buf[520]; // [rsp+20h] [rbp-248h] BYREF
  unsigned long long v10; // [rsp+228h] [rbp-40h]

  v1 = 0;
  v2 = *(long long **)(a1 + 24);
  v3 = *(int *)a1;
  v10 = __readfsqword(0x28u);
  v8.m128i_i64[0] = 0;
  v2[35] = 0;
  v8.m128i_i64[1] = 0;
  if ( !(unsigned char)sub_27D50(a1) )
    return 0;
  while ( 1 )
  {
    v4 = sub_24960(v3, buf, 0x200u);
    v5 = v4;
    if ( !v4 || v4 == -1 )
      break;
    if ( (unsigned char)sub_27DF0((long long)buf, v4) )
    {
      if ( !v6 )
        goto LABEL_4;
      sub_27E20(v2, &v8);
      v8.m128i_i64[1] = 0;
    }
    else
    {
      if ( !v6 )
        v8.m128i_i64[0] = v1;
      v2[35] += v5;
      v8.m128i_i64[1] = v5 + v6;
    }
    if ( !(unsigned char)sub_27D50(a1) )
      return 0;
LABEL_4:
    v1 += v5;
  }
  if ( !v8.m128i_i64[1] )
    v8.m128i_i64[0] = v1;
  sub_27E20(v2, &v8);
  v2[35] += v5;
  return sub_27D50(a1);
}



// Function: check_sparse_region @ 0x29770
long long check_sparse_region(long long a1, __off_t a2, __off_t a3)
{
  __off_t v3; // r15
  unsigned int v5; // r13d
  long long v6; // rdx
  size_t v7; // r14
  long long v8; // rax
  char *v10; // rax
  long long v11; // r13
  char *v12; // rax
  long long v13; // [rsp+0h] [rbp-268h] BYREF
  char buf[520]; // [rsp+20h] [rbp-248h] BYREF
  unsigned long long v15; // [rsp+228h] [rbp-40h]

  v3 = a2;
  v15 = __readfsqword(0x28u);
  v5 = sub_28050(a1, a2);
  if ( (char)v5 && a2 < a3 )
  {
    while ( 1 )
    {
      v7 = a3 - v3;
      if ( a3 - v3 > 512 )
        v7 = 512;
      v8 = sub_4AF10(*(int *)a1, buf, v7);
      if ( v8 == -1 )
      {
        sub_24E00();
        return 0;
      }
      if ( !v8 )
      {
        v10 = dcgettext(0, "Size differs", 5);
        sub_10B50(*(long long *)(a1 + 24), (long long)v10);
        return 0;
      }
      if ( !(unsigned char)sub_27DF0((long long)buf, v8) )
        break;
      v3 += v6;
      if ( a3 <= v3 )
        return v5;
    }
    v11 = sub_42FF0(v3, &v13);
    v12 = dcgettext(0, "File fragment at %s is not a hole", 5);
    sub_10B50(*(long long *)(a1 + 24), (long long)v12, v11);
    return 0;
  }
  return v5;
}



// Function: check_data_region @ 0x298b0
long long check_data_region(long long a1, long long a2)
{
  long long v4; // rax
  long long v5; // rdi
  long long v6; // rbp
  int v7; // edi
  long long v8; // rax
  size_t v9; // r14
  size_t v10; // r12
  void *v11; // rax
  const void *v12; // r15
  char *v13; // rax
  char *v14; // rax
  char *v15; // rax
  unsigned char v16; // [rsp+7h] [rbp-251h]
  char s2[520]; // [rsp+10h] [rbp-248h] BYREF
  unsigned long long v18; // [rsp+218h] [rbp-40h]

  v18 = __readfsqword(0x28u);
  v16 = sub_28050(a1, *(long long *)(*(long long *)(*(long long *)(a1 + 24) + 320LL) + 16 * a2));
  if ( v16 )
  {
    v4 = *(long long *)(a1 + 24);
    v5 = *(long long *)(v4 + 280) - *(long long *)(a1 + 16);
    v6 = *(long long *)(*(long long *)(v4 + 320) + 16 * a2 + 8);
    sub_CCF0(v5);
    while ( 1 )
    {
      if ( v6 <= 0 )
        return v16;
      v10 = 512;
      if ( v6 <= 512 )
        v10 = v6;
      v11 = sub_E130();
      v12 = v11;
      if ( !v11 )
      {
        if ( qword_84D50 )
          qword_84D50(v5);
        v13 = dcgettext(0, "Unexpected EOF in archive", 5);
        error(0, 0, v13);
        dword_84D58 = 2;
        return 0;
      }
      sub_D1F0((unsigned long long)v11);
      v7 = *(int *)a1;
      *(long long *)(a1 + 16) += 512LL;
      v8 = sub_4AF10(v7, s2, v10);
      v9 = v8;
      if ( v8 == -1 )
        break;
      if ( !v8 )
      {
        v14 = dcgettext(0, "Size differs", 5);
        sub_10B50((long long)&qword_84840, (long long)v14);
        return 0;
      }
      v6 -= v8;
      sub_CCF0(*(long long *)(*(long long *)(a1 + 24) + 280LL) - *(long long *)(a1 + 16));
      v5 = (long long)v12;
      if ( memcmp(v12, s2, v9) )
      {
        v15 = dcgettext(0, "Contents differ", 5);
        sub_10B50(*(long long *)(a1 + 24), (long long)v15);
        return 0;
      }
    }
    sub_24E00();
    return 0;
  }
  return v16;
}



// Function: sparse_dump_file @ 0x29cd0
long long sparse_dump_file(int a1, long long a2)
{
  char v2; // r8
  long long result; // rax
  long long v4; // rax
  unsigned long long v5; // rbx
  __int128 vars0; // [rsp+0h] [rbp+0h] BYREF
  long long vars10; // [rsp+10h] [rbp+10h]
  long long vars18; // [rsp+18h] [rbp+18h]
  long long vars20; // [rsp+20h] [rbp+20h]
  unsigned long long vars38; // [rsp+38h] [rbp+38h]

  vars38 = __readfsqword(0x28u);
  v2 = sub_27F70(&vars0);
  result = 3;
  if ( v2 )
  {
    vars18 = a2;
    LODWORD(vars0) = a1;
    BYTE4(vars0) = 1;
    if ( (unsigned char)sub_29C60(&vars0) )
    {
      if ( *(long long *)(vars20 + 56) )
      {
        sub_29CB0(&vars0);
        v4 = vars18;
        if ( a1 >= 0 )
        {
          v5 = 0;
          sub_CC00(*(void **)(vars18 + 8), *(long long *)(vars18 + 136), *(long long *)(vars18 + 280) - vars10);
          while ( 1 )
          {
            v4 = vars18;
            if ( *(long long *)(vars18 + 304) <= v5 )
              break;
            ++v5;
            if ( !(unsigned char)sub_27D70((long long)&vars0) )
              goto LABEL_10;
          }
        }
      }
      else
      {
        v4 = vars18;
      }
      sub_13C80(*(long long *)(v4 + 280) - vars10);
      return (unsigned char)sub_27D30((long long)&vars0) ^ 1u;
    }
    else
    {
LABEL_10:
      sub_13C80(*(long long *)(vars18 + 280) - vars10);
      sub_27D30((long long)&vars0);
      return 1;
    }
  }
  return result;
}



// Function: sparse_member_p @ 0x29e10
long long sparse_member_p(long long a1)
{
  long long result; // rax
  __int128 _0; // [rsp+0h] [rbp+0h] BYREF
  long long vars18; // [rsp+18h] [rbp+18h]
  unsigned long long vars38; // [rsp+38h] [rbp+38h]

  vars38 = __readfsqword(0x28u);
  result = sub_27F70(&_0);
  if ( (char)result )
  {
    vars18 = a1;
    return sub_27D10((long long)&_0);
  }
  return result;
}



// Function: sparse_fixup_header @ 0x29e70
long long sparse_fixup_header(long long a1)
{
  long long result; // rax
  __int128 _0; // [rsp+0h] [rbp+0h] BYREF
  long long vars18; // [rsp+18h] [rbp+18h]
  unsigned long long vars38; // [rsp+38h] [rbp+38h]

  vars38 = __readfsqword(0x28u);
  result = sub_27F70(&_0);
  if ( (char)result )
  {
    vars18 = a1;
    return sub_27DD0((long long)&_0);
  }
  return result;
}



// Function: sparse_extract_file @ 0x29ed0
long long sparse_extract_file(int fd, long long a2, long long *a3)
{
  char v4; // r8
  long long result; // rax
  __off_t v6; // rax
  unsigned long long v7; // rbx
  __int128 vars0; // [rsp+0h] [rbp+0h] BYREF
  long long vars10; // [rsp+10h] [rbp+10h]
  long long vars18; // [rsp+18h] [rbp+18h]
  unsigned long long vars38; // [rsp+38h] [rbp+38h]

  vars38 = __readfsqword(0x28u);
  v4 = sub_27F70(&vars0);
  result = 3;
  if ( v4 )
  {
    vars18 = a2;
    LODWORD(vars0) = fd;
    v6 = lseek(fd, 0, 0);
    *((long long *)&vars0 + 1) = 0;
    BYTE4(vars0) = v6 == 0;
    v7 = 0;
    if ( (unsigned char)sub_27DB0((long long)&vars0) )
    {
      while ( *(long long *)(vars18 + 304) > v7 )
      {
        ++v7;
        if ( !(unsigned char)sub_27D90((long long)&vars0) )
          goto LABEL_8;
      }
      *a3 = *(long long *)(vars18 + 280) - vars10;
      return (unsigned char)sub_27D30((long long)&vars0) ^ 1u;
    }
    else
    {
LABEL_8:
      *a3 = *(long long *)(vars18 + 280) - vars10;
      sub_27D30((long long)&vars0);
      return 1;
    }
  }
  return result;
}



// Function: sparse_skip_file @ 0x29fe0
long long sparse_skip_file(long long a1)
{
  char v1; // r8
  long long result; // rax
  char v3; // bl
  __int128 vars0; // [rsp+0h] [rbp+0h] BYREF
  long long vars10; // [rsp+10h] [rbp+10h]
  long long vars18; // [rsp+18h] [rbp+18h]
  unsigned long long vars38; // [rsp+38h] [rbp+38h]

  vars38 = __readfsqword(0x28u);
  v1 = sub_27F70(&vars0);
  result = 3;
  if ( v1 )
  {
    vars18 = a1;
    LODWORD(vars0) = -1;
    v3 = sub_27DB0((long long)&vars0);
    sub_22B10(*(long long *)(vars18 + 280) - vars10);
    return (unsigned char)(v3 & sub_27D30((long long)&vars0)) ^ 1u;
  }
  return result;
}



// Function: sparse_diff_file @ 0x2a070
long long sparse_diff_file(int a1, long long a2)
{
  unsigned int v2; // r13d
  long long v4; // rax
  __off_t v5; // rsi
  unsigned long long v6; // rbp
  long long v7; // rbx
  long long v8; // rdx
  long long v9; // rsi
  char v10; // r8
  __int128 v11; // [rsp+0h] [rbp-68h] BYREF
  long long v12; // [rsp+10h] [rbp-58h]
  long long v13; // [rsp+18h] [rbp-50h]
  unsigned long long v14; // [rsp+38h] [rbp-30h]

  v14 = __readfsqword(0x28u);
  v2 = sub_27F70(&v11);
  if ( (char)v2 )
  {
    v13 = a2;
    LODWORD(v11) = a1;
    BYTE4(v11) = 1;
    v2 = sub_27DB0((long long)&v11);
    sub_CCB0(a2);
    if ( (char)v2 )
    {
      v4 = v13;
      v5 = 0;
      v6 = 0;
      while ( *(long long *)(v4 + 304) > v6 )
      {
        v7 = 16 * v6;
        if ( !(unsigned char)sub_29770((long long)&v11, v5, *(long long *)(*(long long *)(v4 + 320) + 16 * v6)) )
          goto LABEL_7;
        v9 = v6++;
        v10 = sub_298B0((long long)&v11, v9);
        v4 = v13;
        v8 = v13;
        v5 = *(long long *)(*(long long *)(v13 + 320) + v7) + *(long long *)(*(long long *)(v13 + 320) + v7 + 8);
        if ( !v10 )
          goto LABEL_8;
      }
    }
    else
    {
LABEL_7:
      v8 = v13;
LABEL_8:
      v2 = 0;
      sub_22B10(*(long long *)(v8 + 280) - v12);
    }
    sub_CCD0();
    sub_27D30((long long)&v11);
  }
  return v2;
}



// Function: find_compression_suffix @ 0x2a190
const void *find_compression_suffix(char *s, long long *a2)
{
  char *v2; // rax
  const void *v3; // rbp
  const void *v4; // r15
  size_t v5; // rax
  size_t v6; // r12
  void **i; // rbx

  v2 = strrchr(s, 46);
  v3 = v2;
  if ( v2 )
  {
    v4 = v2 + 1;
    v5 = strlen(v2 + 1);
    v3 = off_81920;
    v6 = v5;
    if ( off_81920 )
    {
      for ( i = &off_81920; i[1] != (void *)v6 || memcmp(v3, v4, v6); i += 3 )
      {
        v3 = i[3];
        if ( !v3 )
          return v3;
      }
      v3 = i;
      if ( a2 )
        *a2 = strlen(s) - v6 - 1;
    }
  }
  return v3;
}



// Function: find_compression_program @ 0x2a240
long long find_compression_program(char *a1, long long a2)
{
  long long *v2; // rdx
  long long result; // rax

  v2 = sub_2A190(a1, 0);
  result = a2;
  if ( v2 )
    return v2[2];
  return result;
}



// Function: set_compression_program_by_suffix @ 0x2a260
char *set_compression_program_by_suffix(char *a1, long long a2)
{
  char *result; // rax

  result = (char *)sub_2A240(a1, a2);
  if ( result )
    qword_84B70 = result;
  return result;
}



// Function: strip_compression_suffix @ 0x2a280
char **strip_compression_suffix(char *src)
{
  char **v1; // r12
  size_t v2; // rbx
  size_t n[7]; // [rsp+0h] [rbp-38h] BYREF

  n[1] = __readfsqword(0x28u);
  v1 = (char **)sub_2A190(src, n);
  if ( v1 )
  {
    v2 = n[0];
    if ( n[0] <= 4 )
    {
      v1 = 0;
      if ( !n[0] )
        return v1;
    }
    else if ( !strncmp(&src[n[0] - 4], ".tar", 4u) && **v1 != 116 )
    {
      v2 -= 4LL;
    }
    v1 = (char **)sub_4C5B0(v2 + 1);
    memcpy(v1, src, v2);
    *((char *)v1 + v2) = 0;
  }
  return v1;
}



// Function: is_regular_file @ 0x2a340
long long is_regular_file(const char *a1)
{
  struct stat v2; // [rsp+0h] [rbp-A8h] BYREF
  unsigned long long v3; // [rsp+98h] [rbp-10h]

  v3 = __readfsqword(0x28u);
  if ( stat(a1, &v2) )
    return *__errno_location() == 2;
  else
    return (v2.st_mode & 0xF000) == 0x8000;
}



// Function: xexec @ 0x2a3b0
void xexec(char *a1)
{
  char *v1[7]; // [rsp+0h] [rbp-38h] BYREF

  v1[5] = (char *)__readfsqword(0x28u);
  v1[0] = "/bin/sh";
  v1[1] = "-c";
  v1[2] = a1;
  v1[3] = 0;
  execv("/bin/sh", v1);
  sub_326C0(a1);
}



// Function: xdup2 @ 0x2a400
void xdup2(unsigned int fd, unsigned int a2)
{
  int v2; // r12d
  int v3; // eax
  int v4; // ebp
  char *v5; // rax
  char *v6; // rax

  if ( fd != a2 )
  {
    if ( close(a2) )
    {
      v2 = *__errno_location();
      if ( v2 != 9 )
      {
        if ( qword_84D50 )
          qword_84D50(a2);
        v6 = dcgettext(0, "Cannot close", 5);
        error(0, v2, v6);
        sub_163D0();
      }
    }
    v3 = dup(fd);
    if ( a2 != v3 )
    {
      if ( v3 < 0 )
      {
        v4 = *__errno_location();
        if ( qword_84D50 )
          qword_84D50(fd);
        v5 = dcgettext(0, "Cannot dup", 5);
        error(0, v4, v5);
        sub_163D0();
      }
      abort();
    }
    sub_D2C0(fd);
  }
}



// Function: wait_for_grandchild @ 0x2a4c0
void wait_for_grandchild(__pid_t pid)
{
  int v1; // edi
  int stat_loc; // [rsp+4h] [rbp-24h] BYREF
  unsigned long long v3; // [rsp+8h] [rbp-20h]

  v3 = __readfsqword(0x28u);
  do
  {
    if ( waitpid(pid, &stat_loc, 0) != -1 )
      goto LABEL_4;
  }
  while ( *__errno_location() == 4 );
  sub_32DF0(qword_84B70);
LABEL_4:
  v1 = BYTE1(stat_loc);
  if ( !__OFSUB__((stat_loc & 0x7F) + 1, 1) && (stat_loc & 0x7F) != 0 )
  {
    raise(stat_loc & 0x7F);
    v1 = 0;
  }
  exit(v1);
}



// Function: run_decompress_program @ 0x2a530
void run_decompress_program()
{
  char *v0; // rax
  long long v1; // r12
  long long i; // rdx
  const char **v3; // rax
  long long v4; // r14
  int *v5; // rax
  char *v6; // rax
  long long v7; // rbx
  char *v8; // rax
  char *v9; // rax
  const char *format; // [rsp+8h] [rbp-140h]
  int v11; // [rsp+1Ch] [rbp-12Ch] BYREF
  long long v12; // [rsp+20h] [rbp-128h] BYREF
  void *dest; // [rsp+28h] [rbp-120h]
  long long v14; // [rsp+30h] [rbp-118h]
  long long v15; // [rsp+90h] [rbp-B8h]
  unsigned long long v16; // [rsp+108h] [rbp-40h]

  v16 = __readfsqword(0x28u);
  v14 = 1;
  v15 = _environ;
  v0 = sub_CDF0(&v11);
  if ( v0 )
  {
    v1 = (long long)v0;
    for ( i = 34082310; !(unsigned int)sub_382C0(v1, &v12, i); i = 34082318 )
    {
      memmove(dest, (char *)dest + 8 * v14, 8 * v12);
      v3 = (const char **)dest;
      *((long long *)dest + v12) = "-d";
      execvp(*v3, (char *const *)v3);
      *((long long *)dest + v12) = 0;
      v4 = sub_CE40(&v11);
      if ( !v4 )
        sub_326C0(v1);
      if ( (dword_83B80 & 0x80000) != 0 )
      {
        if ( qword_84D50 )
          qword_84D50(&v11);
        format = dcgettext(0, "cannot run %s", 5);
        v5 = __errno_location();
        error(0, *v5, format, v1);
        if ( (dword_83B80 & 0x80000) != 0 )
        {
          if ( qword_84D50 )
            qword_84D50(0);
          v6 = dcgettext(0, "trying %s", 5);
          error(0, 0, v6, v4);
        }
      }
      v1 = v4;
    }
    if ( qword_84D50 )
      qword_84D50(v1);
    v7 = sub_354B0(&v12);
    v8 = dcgettext(0, "cannot split string '%s': %s", 5);
    error(0, 0, v8, v1, v7);
    sub_163D0();
  }
  if ( qword_84D50 )
    qword_84D50(&v11);
  v9 = dcgettext(0, "unable to run decompression program", 5);
  error(0, 0, v9);
  sub_163D0();
}



// Function: str_to_env @ 0x2a720
int str_to_env(const char *a1, const char *a2)
{
  int result; // eax

  if ( !a2 )
    return unsetenv(a1);
  result = setenv(a1, a2, 1);
  if ( result )
    sub_16420();
  return result;
}



// Function: time_to_env @ 0x2a750
unsigned long long time_to_env(char *name, long long a2, unsigned int a3)
{
  const char *v3; // rax
  long long v5; // [rsp+0h] [rbp-38h] BYREF
  unsigned long long v6; // [rsp+28h] [rbp-10h]

  v6 = __readfsqword(0x28u);
  v3 = (const char *)sub_24270(a2, a3, (long long)&v5);
  if ( setenv(name, v3, 1) )
    sub_16420();
  return v6 - __readfsqword(0x28u);
}



// Function: dec_to_env @ 0x2a7b0
unsigned long long dec_to_env(char *name, long long a2)
{
  const char *v2; // rax
  long long v4; // [rsp+0h] [rbp-28h] BYREF
  unsigned long long v5; // [rsp+18h] [rbp-10h]

  v5 = __readfsqword(0x28u);
  v2 = (const char *)sub_43090(a2, &v4);
  if ( setenv(name, v2, 1) )
    sub_16420();
  return v5 - __readfsqword(0x28u);
}



// Function: stat_to_env @ 0x2a8f0
int stat_to_env(const char *a1, char a2, long long a3)
{
  long long v6; // rdi
  const char *v7; // rsi
  unsigned int v8; // eax
  unsigned int v9; // eax

  sub_2A720("TAR_VERSION", "1.34");
  sub_2A720("TAR_ARCHIVE", *(const char **)qword_84828);
  sub_2A7B0("TAR_VOLUME", ((qword_84828 - (long long)ptr) >> 3) + 1);
  sub_2A7B0("TAR_BLOCKING_FACTOR", dword_84BA0);
  v6 = (unsigned int)dword_84400;
  if ( !dword_84400 )
    v6 = (unsigned int)dword_84BA4;
  v7 = (const char *)sub_2C230(v6);
  sub_2A720("TAR_FORMAT", v7);
  sub_2A810((unsigned int)a2);
  sub_2A870(*(unsigned int *)(a3 + 112));
  sub_2A720("TAR_FILENAME", a1);
  sub_2A720("TAR_REALNAME", *(const char **)(a3 + 8));
  sub_2A720("TAR_UNAME", *(const char **)(a3 + 32));
  sub_2A720("TAR_GNAME", *(const char **)(a3 + 40));
  sub_2A750("TAR_ATIME", *(long long *)(a3 + 232), *(long long *)(a3 + 240));
  sub_2A750("TAR_MTIME", *(long long *)(a3 + 248), *(long long *)(a3 + 256));
  sub_2A750("TAR_CTIME", *(long long *)(a3 + 264), *(long long *)(a3 + 272));
  sub_2A7B0("TAR_SIZE", *(long long *)(a3 + 136));
  sub_2A7B0("TAR_UID", *(unsigned int *)(a3 + 116));
  sub_2A7B0("TAR_GID", *(unsigned int *)(a3 + 120));
  if ( a2 != 104 )
  {
    if ( a2 <= 104 )
    {
      if ( (unsigned char)(a2 - 98) <= 1u )
      {
        v8 = gnu_dev_minor(*(long long *)(a3 + 128));
        sub_2A7B0("TAR_MINOR", v8);
        v9 = gnu_dev_major(*(long long *)(a3 + 128));
        sub_2A7B0("TAR_MAJOR", v9);
        return unsetenv("TAR_LINKNAME");
      }
      goto LABEL_8;
    }
    if ( a2 != 108 )
    {
LABEL_8:
      unsetenv("TAR_MINOR");
      unsetenv("TAR_MAJOR");
      return unsetenv("TAR_LINKNAME");
    }
  }
  unsetenv("TAR_MINOR");
  unsetenv("TAR_MAJOR");
  return sub_2A720("TAR_LINKNAME", *(const char **)(a3 + 24));
}



// Function: sys_get_archive_stat @ 0x2ab10
bool sys_get_archive_stat()
{
  return fstat(fd, &stru_84740) == 0;
}



// Function: sys_file_is_archive @ 0x2ab40
long long sys_file_is_archive(long long a1)
{
  unsigned int v1; // r8d

  v1 = 0;
  if ( !qword_84818 || qword_84818 != *(long long *)(a1 + 88) )
    return 0;
  LOBYTE(v1) = *(long long *)(a1 + 96) == qword_84810;
  return v1;
}



// Function: sys_save_archive_dev_ino @ 0x2ab80
__ino_t sys_save_archive_dev_ino()
{
  __ino_t result; // rax

  if ( fd <= 0x3FFFFFFF && (result = stru_84740.st_mode & 0xF000, (int)result == 0x8000) )
  {
    qword_84818 = stru_84740.st_dev;
    qword_84810 = stru_84740.st_ino;
    return stru_84740.st_ino;
  }
  else
  {
    qword_84818 = 0;
  }
  return result;
}



// Function: sys_detect_dev_null_output @ 0x2abd0
unsigned long long sys_detect_dev_null_output()
{
  int v0; // r8d
  bool v1; // al
  struct stat v3; // [rsp+0h] [rbp-A8h] BYREF
  unsigned long long v4; // [rsp+98h] [rbp-10h]

  v4 = __readfsqword(0x28u);
  v0 = strcmp(*(const char **)ptr, "/dev/null");
  v1 = 1;
  if ( v0 )
    v1 = fd <= 0x3FFFFFFF
      && (stru_84740.st_mode & 0xF000) == 0x2000
      && !stat("/dev/null", &v3)
      && stru_84740.st_dev == v3.st_dev
      && stru_84740.st_ino == v3.st_ino;
  byte_84A30 = v1;
  return v4 - __readfsqword(0x28u);
}



// Function: sys_wait_for_child @ 0x2aca0
unsigned long long sys_wait_for_child(unsigned int pid, char a2)
{
  char *v3; // rdi
  short v4; // cx
  int v5; // ecx
  unsigned int v7; // ebx
  char *v8; // rax
  char *v9; // rax
  int v10; // [rsp+Ch] [rbp-2Ch]
  unsigned int v11; // [rsp+Ch] [rbp-2Ch]
  int stat_loc; // [rsp+14h] [rbp-24h] BYREF
  unsigned long long v13; // [rsp+18h] [rbp-20h]

  v13 = __readfsqword(0x28u);
  if ( !pid )
    return v13 - __readfsqword(0x28u);
  do
  {
    v3 = (char *)pid;
    if ( waitpid(pid, &stat_loc, 0) != -1 )
    {
      v4 = stat_loc;
      if ( __OFSUB__((stat_loc & 0x7F) + 1, 1) || (stat_loc & 0x7F) == 0 )
        goto LABEL_9;
LABEL_6:
      v5 = v4 & 0x7F;
      if ( v5 != 13 || a2 )
      {
        if ( qword_84D50 )
        {
          v10 = v5;
          qword_84D50(v3);
          v5 = v10;
        }
        v11 = v5;
        v9 = dcgettext(0, "Child died with signal %d", 5);
        error(0, 0, v9, v11);
        goto LABEL_18;
      }
      return v13 - __readfsqword(0x28u);
    }
  }
  while ( *__errno_location() == 4 );
  v3 = qword_84B70;
  sub_32DF0(qword_84B70);
  v4 = stat_loc;
  if ( !__OFSUB__((stat_loc & 0x7F) + 1, 1) && (stat_loc & 0x7F) != 0 )
    goto LABEL_6;
LABEL_9:
  if ( HIBYTE(v4) )
  {
    if ( qword_84D50 )
    {
      qword_84D50(v3);
      HIBYTE(v4) = BYTE1(stat_loc);
    }
    v7 = HIBYTE(v4);
    v8 = dcgettext(0, "Child returned status %d", 5);
    error(0, 0, v8, v7);
LABEL_18:
    sub_163D0();
  }
  return v13 - __readfsqword(0x28u);
}



// Function: sys_spawn_shell @ 0x2add0
unsigned long long sys_spawn_shell()
{
  char *v0; // r12
  __pid_t v1; // eax
  __pid_t v2; // ebx
  int stat_loc; // [rsp+4h] [rbp-24h] BYREF
  unsigned long long v5; // [rsp+8h] [rbp-20h]

  v5 = __readfsqword(0x28u);
  v0 = getenv("SHELL");
  if ( !v0 )
    v0 = "/bin/sh";
  v1 = sub_24F80();
  if ( !v1 )
  {
    execlp(v0, "-sh", "-i", 0);
    sub_326C0(v0);
  }
  v2 = v1;
  while ( waitpid(v2, &stat_loc, 0) == -1 )
  {
    if ( *__errno_location() != 4 )
    {
      sub_32DF0(v0);
      return v5 - __readfsqword(0x28u);
    }
  }
  return v5 - __readfsqword(0x28u);
}



// Function: sys_compare_uid @ 0x2ae90
bool sys_compare_uid(long long a1, long long a2)
{
  return *(int *)(a1 + 28) == *(int *)(a2 + 28);
}



// Function: sys_compare_gid @ 0x2aea0
bool sys_compare_gid(long long a1, long long a2)
{
  return *(int *)(a1 + 32) == *(int *)(a2 + 32);
}



// Function: sys_compare_links @ 0x2aeb0
bool sys_compare_links(long long *a1, long long *a2)
{
  bool result; // al

  result = 0;
  if ( *a2 == *a1 )
    return a2[1] == a1[1];
  return result;
}



// Function: sys_truncate @ 0x2aed0
int sys_truncate(int fd)
{
  long long v1; // rax

  v1 = lseek(fd, 0, 1);
  if ( v1 < 0 )
    return -1;
  else
    return ftruncate(fd, v1);
}



// Function: sys_write_archive_buffer @ 0x2af00
long long sys_write_archive_buffer()
{
  if ( fd <= 0x3FFFFFFF )
    return sub_41120(fd, s, nbytes);
  else
    return sub_33EA0((unsigned int)(fd - 0x40000000), s, nbytes);
}



// Function: sys_child_open_for_compress @ 0x2af40
long long sys_child_open_for_compress()
{
  __pid_t v0; // eax
  char *v1; // rax
  const void **v2; // rbx
  char *v3; // rax
  char *v4; // rdi
  const char *v5; // rbp
  int *v6; // rax
  int v7; // ebp
  int *v8; // rbx
  long long *v9; // rax
  unsigned int v10; // r12d
  void *v12; // rax
  __pid_t v13; // ebp
  const void **v14; // rbx
  const char *v15; // r12
  size_t v16; // rdx
  char *v17; // r12
  unsigned long long v18; // rbx
  long long v19; // rax
  long long v20; // rdx
  long long v21; // rcx
  long long v22; // r8
  long long v23; // rdi
  char *v24; // rax
  int v25; // eax
  long long v26; // rax
  char *v27; // rax
  char *v28; // [rsp+8h] [rbp-40h]
  int fd; // [rsp+18h] [rbp-30h] BYREF
  int v30; // [rsp+1Ch] [rbp-2Ch]
  int v31; // [rsp+20h] [rbp-28h] BYREF
  int v32; // [rsp+24h] [rbp-24h]
  unsigned long long v33; // [rsp+28h] [rbp-20h]

  v33 = __readfsqword(0x28u);
  signal(13, (__sighandler_t)((char *)&dword_0 + 1));
  sub_24FC0(&fd);
  v0 = sub_24F80();
  if ( v0 <= 0 )
  {
    v1 = dcgettext(0, "tar (child)", 5);
    sub_48760(v1);
    signal(13, 0);
    sub_2A400(fd, 0);
    sub_D2C0(v30);
    if ( byte_84D70 )
    {
      v4 = *(char **)ptr;
    }
    else
    {
      v2 = (const void **)ptr;
      v3 = strchr(*(const char **)ptr, 58);
      qword_84D78 = (long long)v3;
      v4 = (char *)*v2;
      if ( v3 )
      {
        if ( v3 > v4 )
        {
          v28 = (char *)*v2;
          v12 = memchr(v4, 47, v3 - v4);
          v4 = v28;
          if ( !v12 )
            goto LABEL_15;
        }
      }
    }
    if ( sub_2A340(v4) )
    {
      if ( byte_84B84 )
        sub_24710(*(char **)ptr, 1);
      v5 = *(const char **)ptr;
      if ( strcmp(*(const char **)ptr, "-") )
      {
        ::fd = creat(v5, 0x1B6u);
        if ( ::fd < 0 )
        {
          v6 = __errno_location();
          v7 = *v6;
          v8 = v6;
          if ( byte_84B84 )
            sub_245D0();
          v9 = ptr;
          *v8 = v7;
          sub_327E0(*v9);
        }
        sub_2A400(::fd, 1u);
      }
      goto LABEL_34;
    }
LABEL_15:
    sub_24FC0(&v31);
    v13 = sub_24F80();
    if ( v13 )
    {
      sub_2A400(v31, 0);
      sub_D2C0(v32);
      v14 = (const void **)ptr;
      v15 = *(const char **)ptr;
      if ( !strcmp(*(const char **)ptr, "-") )
      {
        ::fd = 1;
      }
      else
      {
        if ( byte_84D70
          || (v24 = strchr(v15, 58), qword_84D78 = (long long)v24, v15 = (const char *)*v14, !v24)
          || v24 <= v15
          || memchr(*v14, 47, v24 - v15) )
        {
          v25 = creat(v15, 0x1B6u);
        }
        else
        {
          v25 = sub_338D0(v15, 65, 0x40000000, qword_84AB0);
        }
        ::fd = v25;
        if ( v25 < 0 )
          sub_327E0(*(long long *)ptr);
      }
      v16 = nbytes;
      while ( 1 )
      {
        v17 = (char *)s;
        if ( !v16 )
          goto LABEL_40;
        v18 = 0;
        do
        {
          v19 = sub_4AF10(0, v17, v16 - v18);
          if ( v19 == -1 )
            sub_329B0(qword_84B70, v17, v20, v21, v22);
          if ( !v19 )
          {
            if ( v18 )
            {
              memset((char *)s + v18, 0, nbytes - v18);
              v23 = sub_2AF00();
              if ( nbytes != v23 )
LABEL_26:
                sub_D6D0(v23);
            }
LABEL_40:
            sub_2A4C0(v13);
          }
          v16 = nbytes;
          v18 += v19;
          v17 += v19;
        }
        while ( nbytes > v18 );
        v26 = sub_2AF00();
        v16 = v26;
        if ( nbytes != v26 )
        {
          v23 = v26;
          goto LABEL_26;
        }
      }
    }
    v27 = dcgettext(0, "tar (grandchild)", 5);
    sub_48760(v27);
    sub_2A400(v32, 1u);
    sub_D2C0(v31);
LABEL_34:
    sub_2A3B0(qword_84B70);
  }
  v10 = v0;
  ::fd = v30;
  sub_D2C0(fd);
  return v10;
}



// Function: sys_child_open_for_uncompress @ 0x2b310
long long sys_child_open_for_uncompress()
{
  __pid_t v0; // eax
  char *v1; // rax
  const void **v2; // rbx
  const char *v3; // rbp
  char *v4; // rax
  __pid_t v5; // r12d
  const void **v6; // rbx
  const char *v7; // rbp
  unsigned long long v8; // rbx
  char *v9; // r13
  size_t v10; // rbp
  unsigned int v11; // r12d
  char *v13; // rax
  int v14; // eax
  char *v15; // rax
  int v16; // [rsp+8h] [rbp-40h] BYREF
  int fd; // [rsp+Ch] [rbp-3Ch]
  int v18; // [rsp+10h] [rbp-38h] BYREF
  int v19; // [rsp+14h] [rbp-34h]
  unsigned long long v20; // [rsp+18h] [rbp-30h]

  v20 = __readfsqword(0x28u);
  sub_24FC0(&v16);
  v0 = sub_24F80();
  if ( v0 <= 0 )
  {
    v1 = dcgettext(0, "tar (child)", 5);
    sub_48760(v1);
    signal(13, 0);
    sub_2A400(fd, 1u);
    sub_D2C0(v16);
    v2 = (const void **)ptr;
    v3 = *(const char **)ptr;
    if ( strcmp(*(const char **)ptr, "-")
      && (byte_84D70
       || (v4 = strchr(v3, 58), qword_84D78 = (long long)v4, v3 = (const char *)*v2, !v4)
       || v4 <= v3
       || memchr(*v2, 47, v4 - v3))
      && sub_2A340(v3) )
    {
      ::fd = open(*(const char **)ptr, 0, 438);
      if ( ::fd >= 0 )
      {
        sub_2A400(::fd, 0);
        sub_2A530();
      }
    }
    else
    {
      sub_24FC0(&v18);
      v5 = sub_24F80();
      if ( !v5 )
      {
        v15 = dcgettext(0, "tar (grandchild)", 5);
        sub_48760(v15);
        sub_2A400(v18, 0);
        sub_D2C0(v19);
        sub_2A530();
      }
      sub_2A400(v19, 1u);
      sub_D2C0(v18);
      v6 = (const void **)ptr;
      v7 = *(const char **)ptr;
      if ( !strcmp(*(const char **)ptr, "-") )
      {
        ::fd = 0;
        goto LABEL_11;
      }
      if ( byte_84D70
        || (v13 = strchr(v7, 58), qword_84D78 = (long long)v13, v7 = (const char *)*v6, !v13)
        || v13 <= v7
        || memchr(*v6, 47, v13 - v7) )
      {
        v14 = open(v7, 0, 438);
      }
      else
      {
        v14 = sub_338D0(v7, 0, 0x40000000, qword_84AB0);
      }
      ::fd = v14;
      if ( v14 >= 0 )
      {
LABEL_11:
        sub_CD10();
        while ( 1 )
        {
          if ( ::fd > 0x3FFFFFFF )
          {
            v8 = sub_33DA0((unsigned int)(::fd - 0x40000000), s, nbytes);
            if ( v8 != -1 )
              goto LABEL_16;
          }
          else
          {
            v8 = sub_4AF10(::fd, s, nbytes);
            if ( v8 != -1 )
            {
LABEL_16:
              if ( !v8 )
              {
                sub_D2C0(1);
                sub_2A4C0(v5);
              }
              v9 = (char *)s;
              do
              {
                v10 = 512;
                if ( v8 <= 0x200 )
                  v10 = v8;
                if ( sub_41120(1, v9, v10) != v10 )
                  sub_32E10(qword_84B70);
                v9 += v10;
                v8 -= v10;
              }
              while ( v8 );
              goto LABEL_11;
            }
          }
          sub_D300();
        }
      }
    }
    sub_327E0(*(long long *)ptr);
  }
  v11 = v0;
  ::fd = v16;
  sub_D2C0(fd);
  return v11;
}



// Function: sys_exec_command @ 0x2b640
long long sys_exec_command(const char *a1, char a2, long long a3)
{
  int fd; // [rsp+0h] [rbp-28h] BYREF
  unsigned int v6; // [rsp+4h] [rbp-24h]
  unsigned long long v7; // [rsp+8h] [rbp-20h]

  v7 = __readfsqword(0x28u);
  sub_24FC0(&fd);
  handler = signal(13, (__sighandler_t)((char *)&dword_0 + 1));
  pid = sub_24F80();
  if ( !pid )
  {
    sub_2A400(fd, 0);
    sub_D2C0(v6);
    sub_2A8F0(a1, a2, a3);
    sub_2A3B0((char *)qword_84A60);
  }
  sub_D2C0(fd);
  return v6;
}



// Function: sys_wait_command @ 0x2b6f0
unsigned long long sys_wait_command()
{
  unsigned int v0; // ebp
  unsigned int v1; // ebx
  long long v2; // r12
  char *v3; // rax
  unsigned int v5; // ebx
  long long v6; // r12
  char *v7; // rax
  long long v8; // r12
  char *v9; // rax
  int stat_loc; // [rsp+4h] [rbp-24h] BYREF
  unsigned long long v11; // [rsp+8h] [rbp-20h]

  v0 = pid;
  v11 = __readfsqword(0x28u);
  if ( pid >= 0 )
  {
    signal(13, handler);
    while ( waitpid(v0, &stat_loc, 0) == -1 )
    {
      if ( *__errno_location() != 4 )
      {
        pid = -1;
        sub_32DF0(qword_84A60);
        return v11 - __readfsqword(0x28u);
      }
      v0 = pid;
    }
    v1 = stat_loc & 0x7F;
    if ( (stat_loc & 0x7F) != 0 )
    {
      if ( (char)(v1 + 1) <= 1 )
      {
        if ( qword_84D50 )
          qword_84D50(v0);
        v8 = pid;
        v9 = dcgettext(0, "%lu: Child terminated on unknown reason", 5);
        error(0, 0, v9, v8);
        dword_84D58 = 2;
      }
      else
      {
        if ( qword_84D50 )
        {
          qword_84D50(v0);
          v1 = stat_loc & 0x7F;
        }
        v2 = pid;
        v3 = dcgettext(0, "%lu: Child terminated on signal %d", 5);
        error(0, 0, v3, v2, v1);
      }
    }
    else if ( !byte_84A59 )
    {
      v5 = BYTE1(stat_loc);
      if ( BYTE1(stat_loc) )
      {
        if ( qword_84D50 )
        {
          qword_84D50(v0);
          v5 = BYTE1(stat_loc);
        }
        v6 = pid;
        v7 = dcgettext(0, "%lu: Child returned status %d", 5);
        error(0, 0, v7, v6, v5);
        dword_84D58 = 2;
      }
    }
    pid = -1;
  }
  return v11 - __readfsqword(0x28u);
}



// Function: sys_exec_info_script @ 0x2b8b0
long long sys_exec_info_script(const char **a1, int a2)
{
  __pid_t v2; // eax
  __pid_t v3; // ebx
  FILE *v4; // r12
  int v5; // ebp
  int v6; // r14d
  char *v7; // rax
  unsigned int v8; // r12d
  const char *v10; // rax
  const char *v11; // rsi
  const char *v12; // rax
  long long v13; // rdi
  const char *v14; // rax
  const char *v15; // rax
  int stat_loc; // [rsp+4h] [rbp-64h] BYREF
  char *lineptr; // [rsp+8h] [rbp-60h] BYREF
  size_t n; // [rsp+10h] [rbp-58h] BYREF
  int fd; // [rsp+18h] [rbp-50h] BYREF
  int v20; // [rsp+1Ch] [rbp-4Ch]
  char v21[24]; // [rsp+20h] [rbp-48h] BYREF
  unsigned long long v22; // [rsp+38h] [rbp-30h]

  v22 = __readfsqword(0x28u);
  sub_24FC0(&fd);
  qword_84720 = signal(13, (__sighandler_t)((char *)&dword_0 + 1));
  v2 = sub_24F80();
  if ( !v2 )
  {
    setenv("TAR_VERSION", "1.34", 1);
    setenv("TAR_ARCHIVE", *a1, 1);
    v10 = (const char *)sub_43090(a2, v21);
    setenv("TAR_VOLUME", v10, 1);
    v11 = (const char *)sub_43090(dword_84BA0, v21);
    setenv("TAR_BLOCKING_FACTOR", v11, 1);
    v12 = (const char *)sub_2C7E0((unsigned int)dword_84BA8);
    setenv("TAR_SUBCOMMAND", v12, 1);
    v13 = (unsigned int)dword_84400;
    if ( !dword_84400 )
      v13 = (unsigned int)dword_84BA4;
    v14 = (const char *)sub_2C230(v13);
    setenv("TAR_FORMAT", v14, 1);
    v15 = (const char *)sub_43090(v20, v21);
    setenv("TAR_FD", v15, 1);
    sub_D2C0(fd);
    sub_2A3B0((char *)qword_84B48);
  }
  lineptr = 0;
  v3 = v2;
  n = 0;
  sub_D2C0(v20);
  v4 = fdopen(fd, "r");
  v5 = getline(&lineptr, &n, v4);
  v6 = v5;
  fclose(v4);
  if ( v5 > 0 )
  {
    v7 = &lineptr[v5 - 1];
    if ( *v7 == 10 )
    {
      *v7 = 0;
      v6 = v5 - 1;
    }
  }
  while ( 1 )
  {
    v8 = waitpid(v3, &stat_loc, 0);
    if ( v8 != -1 )
      break;
    if ( *__errno_location() != 4 )
    {
      signal(13, qword_84720);
      sub_32DF0(qword_84B48);
      return v8;
    }
  }
  signal(13, qword_84720);
  v8 = stat_loc & 0x7F;
  if ( (stat_loc & 0x7F) != 0 )
  {
    free(lineptr);
    return (unsigned int)-1;
  }
  else if ( BYTE1(stat_loc) || v6 <= 0 )
  {
    free(lineptr);
    return BYTE1(stat_loc);
  }
  else
  {
    *a1 = lineptr;
  }
  return v8;
}



// Function: sys_exec_checkpoint_script @ 0x2bb40
unsigned long long sys_exec_checkpoint_script(char *a1, const char *a2, int a3)
{
  __pid_t v5; // eax
  __pid_t v6; // ebx
  const char *v8; // rax
  const char *v9; // rax
  const char *v10; // rax
  long long v11; // rdi
  const char *v12; // rax
  int stat_loc; // [rsp+Ch] [rbp-4Ch] BYREF
  char v14[24]; // [rsp+10h] [rbp-48h] BYREF
  unsigned long long v15; // [rsp+28h] [rbp-30h]

  v15 = __readfsqword(0x28u);
  v5 = sub_24F80();
  if ( !v5 )
  {
    setenv("TAR_VERSION", "1.34", 1);
    setenv("TAR_ARCHIVE", a2, 1);
    v8 = (const char *)sub_43090(a3, v14);
    setenv("TAR_CHECKPOINT", v8, 1);
    v9 = (const char *)sub_43090(dword_84BA0, v14);
    setenv("TAR_BLOCKING_FACTOR", v9, 1);
    v10 = (const char *)sub_2C7E0((unsigned int)dword_84BA8);
    setenv("TAR_SUBCOMMAND", v10, 1);
    v11 = (unsigned int)dword_84400;
    if ( !dword_84400 )
      v11 = (unsigned int)dword_84BA4;
    v12 = (const char *)sub_2C230(v11);
    setenv("TAR_FORMAT", v12, 1);
    sub_2A3B0(a1);
  }
  v6 = v5;
  while ( waitpid(v6, &stat_loc, 0) == -1 )
  {
    if ( *__errno_location() != 4 )
    {
      sub_32DF0(a1);
      return v15 - __readfsqword(0x28u);
    }
  }
  return v15 - __readfsqword(0x28u);
}



// Function: optloc_lookup @ 0x2bcb0
long long optloc_lookup(int a1)
{
  return qword_84BC0[a1];
}



// Function: option_set_in_cl @ 0x2bcc0
long long option_set_in_cl(int a1)
{
  int *v1; // rax

  v1 = (int *)sub_2BCB0(a1);
  return v1 && *v1 == 1;
}



// Function: find_argp_option_key @ 0x2bce0
long long find_argp_option_key(long long a1, int a2)
{
  long long result; // rax
  int v3; // edx

  for ( result = a1; ; result += 48 )
  {
    v3 = *(int *)(result + 8);
    if ( !*(long long *)result
      && !v3
      && !*(long long *)(result + 16)
      && !*(int *)(result + 24)
      && !*(long long *)(result + 32) )
    {
      break;
    }
    if ( a2 == v3 )
      return result;
  }
  return 0;
}



// Function: optloc_eq @ 0x2bd20
long long optloc_eq(long long a1, long long a2)
{
  long long result; // rax

  result = 0;
  if ( *(int *)a1 == *(int *)a2 )
  {
    result = 1;
    if ( *(int *)a1 != 1 )
      return strcmp(*(const char **)(a1 + 8), *(const char **)(a2 + 8)) == 0;
  }
  return result;
}



// Function: optloc_save @ 0x2bd60
long long optloc_save(int a1, const char **a2)
{
  size_t v3; // rdi
  long long v5; // rax
  char *v6; // rsi
  long long v7; // rbx
  int v8; // eax
  long long v9; // r8

  v3 = 32;
  if ( a2[1] )
    v3 = strlen(a2[1]) + 33;
  v5 = sub_4C5B0(v3);
  v6 = (char *)a2[1];
  v7 = v5;
  if ( v6 )
    v6 = strcpy((char *)(v5 + 32), v6);
  v8 = *(int *)a2;
  *(long long *)(v7 + 8) = v6;
  *(int *)v7 = v8;
  *(long long *)(v7 + 16) = a2[2];
  v9 = qword_84BC0[a1];
  qword_84BC0[a1] = v7;
  *(long long *)(v7 + 24) = v9;
  return v9;
}



// Function: tar_set_quoting_style @ 0x2bde0
long long tar_set_quoting_style(char *s1)
{
  char *v1; // r12
  const char *v2; // rsi
  long long v3; // rbx
  long long v5; // rbx
  char *v6; // rax

  v1 = s1;
  v2 = "literal";
  if ( !"literal" )
  {
LABEL_6:
    if ( qword_84D50 )
      qword_84D50(s1);
    v5 = qword_84DD0;
    v6 = dcgettext(0, "Unknown quoting style '%s'. Try '%s --quoting-style=help' to get a list.", 5);
    error(0, 0, v6, v1, v5);
    sub_163D0();
  }
  v3 = 0;
  while ( 1 )
  {
    s1 = v1;
    if ( !strcmp(v1, v2) )
      return sub_4A590(0, (unsigned int)v3);
    v2 = (&off_80680)[++v3];
    if ( !v2 )
      goto LABEL_6;
  }
}



// Function: sigstat @ 0x2be90
void sigstat(int a1)
{
  sub_CD50();
  sub_D150();
  signal(a1, sigstat);
}



// Function: archive_format_string @ 0x2c230
const char *archive_format_string(int a1)
{
  int v1; // edx
  _UNKNOWN **v2; // rax
  char *v3; // r8

  v1 = 1;
  v2 = &off_7F9A0;
  v3 = "v7";
  while ( 1 )
  {
    if ( a1 == v1 )
      return v3;
    v3 = (char *)v2[2];
    v2 += 2;
    if ( !v3 )
      break;
    v1 = *((int *)v2 + 2);
  }
  return "unknown?";
}



// Function: format_default_settings @ 0x2c280
long long format_default_settings()
{
  unsigned int v0; // eax
  int v1; // ecx

  v0 = (unsigned int)sub_2C230(6);
  return sub_4CC50(
           (unsigned int)"--format=%s -f%s -b%d --quoting-style=%s --rmt-command=%s --rsh-command=%s",
           v0,
           (unsigned int)"-",
           v1,
           (unsigned int)"escape",
           (unsigned int)"/usr/local/libexec/rmt",
           (char)"/usr/bin/rsh");
}



// Function: tar_help_filter @ 0x2c2d0
long long tar_help_filter(int a1, long long a2, long long a3, long long a4, long long a5, long long a6, char a7)
{
  char *v7; // r13
  size_t v8; // rax
  char *next_free; // rdi
  size_t v10; // r12
  char *v11; // r12
  char *v12; // r13
  size_t v13; // rax
  char *v14; // rdi
  size_t v15; // r12
  const char *v16; // r13
  size_t v17; // rax
  char *v18; // rdi
  size_t v19; // r12
  char *v20; // rax
  char *v21; // rax
  char *object_base; // rdi
  char *chunk_limit; // rdx
  long long v24; // r12
  int v25; // edx
  int v26; // ecx
  unsigned int v27; // edi
  int v28; // r8d
  int v29; // r9d
  char *v30; // rsi
  struct obstack v32; // [rsp+0h] [rbp-78h] BYREF
  unsigned long long v33; // [rsp+58h] [rbp-20h]

  v33 = __readfsqword(0x28u);
  if ( a1 == 148 )
  {
    v27 = (unsigned int)dcgettext(0, "filter the archive through %s", 5);
    v30 = "lzip";
    return sub_4CC50(v27, (int)v30, v25, v26, v28, v29, a7);
  }
  if ( a1 <= 148 )
  {
    if ( a1 == 106 )
    {
      v27 = (unsigned int)dcgettext(0, "filter the archive through %s", 5);
      v30 = "bzip2";
    }
    else if ( a1 <= 106 )
    {
      if ( a1 == 74 )
      {
        v27 = (unsigned int)dcgettext(0, "filter the archive through %s", 5);
        v30 = "xz";
      }
      else
      {
        if ( a1 != 90 )
          return a2;
        v27 = (unsigned int)dcgettext(0, "filter the archive through %s", 5);
        v30 = "compress";
      }
    }
    else
    {
      if ( a1 != 122 )
        return a2;
      v27 = (unsigned int)dcgettext(0, "filter the archive through %s", 5);
      v30 = "gzip";
    }
    return sub_4CC50(v27, (int)v30, v25, v26, v28, v29, a7);
  }
  if ( a1 == 207 )
  {
    v27 = (unsigned int)dcgettext(0, "filter the archive through %s", 5);
    v30 = "zstd";
    return sub_4CC50(v27, (int)v30, v25, v26, v28, v29, a7);
  }
  if ( a1 <= 207 )
  {
    if ( a1 == 149 )
    {
      v27 = (unsigned int)dcgettext(0, "filter the archive through %s", 5);
      v30 = "lzma";
    }
    else
    {
      if ( a1 != 150 )
        return a2;
      v27 = (unsigned int)dcgettext(0, "filter the archive through %s", 5);
      v30 = "lzop";
    }
    return sub_4CC50(v27, (int)v30, v25, v26, v28, v29, a7);
  }
  if ( a1 == 33554436 )
  {
    obstack_begin(&v32, 0, 0, sub_4C5B0, &free);
    v7 = dcgettext(0, "Valid arguments for the --quoting-style option are:", 5);
    v8 = strlen(v7);
    next_free = v32.next_free;
    v10 = v8;
    if ( v8 > v32.chunk_limit - v32.next_free )
    {
      obstack_newchunk(&v32, v8);
      next_free = v32.next_free;
    }
    memcpy(next_free, v7, v10);
    v11 = &v32.next_free[v10];
    v32.next_free = v11;
    if ( (char *)(v32.chunk_limit - v11) <= (char *)&dword_0 + 1 )
    {
      obstack_newchunk(&v32, 2);
      v11 = v32.next_free;
    }
    *(short *)v11 = 2570;
    v32.next_free += 2;
    sub_2C050(&v32);
    v12 = dcgettext(0, "\n*This* tar defaults to:\n", 5);
    v13 = strlen(v12);
    v14 = v32.next_free;
    v15 = v13;
    if ( v13 > v32.chunk_limit - v32.next_free )
    {
      obstack_newchunk(&v32, v13);
      v14 = v32.next_free;
    }
    memcpy(v14, v12, v15);
    v32.next_free += v15;
    v16 = (const char *)sub_2C280();
    v17 = strlen(v16);
    v18 = v32.next_free;
    v19 = v17;
    if ( v17 > v32.chunk_limit - v32.next_free )
    {
      obstack_newchunk(&v32, v17);
      v18 = v32.next_free;
    }
    memcpy(v18, v16, v19);
    v20 = &v32.next_free[v19];
    v32.next_free = v20;
    if ( v20 == v32.chunk_limit )
    {
      obstack_newchunk(&v32, 1);
      v20 = v32.next_free;
    }
    v32.next_free = v20 + 1;
    *v20 = 10;
    v21 = v32.next_free;
    if ( v32.chunk_limit == v32.next_free )
    {
      obstack_newchunk(&v32, 1);
      v21 = v32.next_free;
    }
    v32.next_free = v21 + 1;
    *v21 = 0;
    object_base = v32.object_base;
    if ( v32.next_free == v32.object_base )
      *((char *)&v32 + 80) |= 2u;
    chunk_limit = v32.chunk_limit;
    if ( (~*(long long *)&v32.alignment_mask & (long long)&v32.next_free[*(long long *)&v32.alignment_mask])
       - (unsigned long long)v32.chunk <= v32.chunk_limit - (char *)v32.chunk )
      chunk_limit = (char *)(~*(long long *)&v32.alignment_mask & (long long)&v32.next_free[*(long long *)&v32.alignment_mask]);
    v32.next_free = chunk_limit;
    v32.object_base = chunk_limit;
    v24 = sub_4C7A0(object_base);
    obstack_free(&v32, 0);
    return v24;
  }
  return a2;
}



// Function: subcommand_string @ 0x2c7e0
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



// Function: decode_signal @ 0x2c8a0
long long decode_signal(char *s2)
{
  const char *v2; // rbp
  char **v3; // rbx
  int v4; // eax
  const char *v5; // rdi
  char *v7; // rax

  v2 = s2 + 3;
  v3 = &off_7F8E0;
  v4 = strncmp(s2, "SIG", 3u);
  v5 = "USR1";
  if ( v4 )
    v2 = s2;
  if ( strcmp("USR1", v2) )
  {
    do
    {
      v3 += 2;
      if ( v3 == &off_7F930 )
      {
        if ( qword_84D50 )
          qword_84D50(v5);
        v7 = dcgettext(0, "Unknown signal name: %s", 5);
        error(0, 0, v7, s2);
        sub_163D0();
      }
      v5 = *v3;
    }
    while ( strcmp(*v3, v2) );
  }
  return *((unsigned int *)v3 + 2);
}



// Function: set_stat_signal @ 0x2c950
__sighandler_t set_stat_signal(char *a1)
{
  int v2; // edi

  v2 = sub_2C8A0(a1);
  return signal(v2, sub_2BE90);
}



// Function: usage @ 0x2c970
void usage(int status)
{
  argp_help(&off_81B20, stderr, 4, qword_84DD0);
  sub_3E440();
  exit(status);
}



// Function: request_stdin @ 0x2c9b0
void request_stdin(long long a1)
{
  long long v1; // r12
  char *v2; // rax

  if ( qword_84C10 )
  {
    if ( qword_84D50 )
      qword_84D50(a1);
    v1 = qword_84C10;
    v2 = dcgettext(0, "Options '%s' and '%s' both want standard input", 5);
    error(0, 0, v2, v1, a1);
    sub_2C970(2);
  }
  qword_84C10 = a1;
}



// Function: confirm @ 0x2ca20
long long confirm(const char *a1, long long a2)
{
  const char *v2; // rax
  unsigned int v3; // r12d
  int v5; // eax
  int v6; // r13d
  char *lineptr; // [rsp+8h] [rbp-30h] BYREF
  size_t n[5]; // [rsp+10h] [rbp-28h] BYREF

  n[1] = __readfsqword(0x28u);
  if ( !qword_84BB0 )
  {
    if ( fd && !qword_84C10 )
    {
      sub_2C9B0((long long)"-w");
      qword_84BB0 = (FILE *)stdin;
    }
    else
    {
      qword_84BB0 = fopen("/dev/tty", "r");
      if ( !qword_84BB0 )
        sub_327E0("/dev/tty");
    }
  }
  v2 = (const char *)sub_4AC50(a2);
  __fprintf_chk(stream, 1, "%s %s?", a1, v2);
  fflush_unlocked(stream);
  v3 = dword_84BAC;
  if ( dword_84BAC )
  {
    v3 = 0;
  }
  else
  {
    lineptr = 0;
    n[0] = 0;
    if ( getline(&lineptr, n, qword_84BB0) >= 0 )
    {
      v5 = rpmatch(lineptr);
      v6 = dword_84BAC;
      v3 = v5 > 0;
      free(lineptr);
      if ( !v6 )
        return v3;
    }
    else
    {
      dword_84BAC = 1;
      free(lineptr);
    }
  }
  fputc_unlocked(10, stream);
  fflush_unlocked(stream);
  return v3;
}



// Function: get_date_or_file @ 0x2cbe0
long long get_date_or_file(long long a1, long long a2, char *a3, __m128i *a4)
{
  unsigned int v6; // r12d
  long long v7; // rdx
  long long v11; // rbp
  char *v12; // r12
  char *v13; // rax
  char *v14; // rcx
  long long v15; // rax
  __m128i v16; // xmm0
  long long *v17; // r14
  long long v18; // rax
  char *v19; // rax
  struct stat v20; // [rsp+0h] [rbp-C8h] BYREF
  unsigned long long v21; // [rsp+98h] [rbp-30h]

  v21 = __readfsqword(0x28u);
  if ( (unsigned char)(*a3 - 46) > 1u )
  {
    if ( (unsigned char)sub_486D0(a4, a3, 0) )
    {
      v15 = sub_4C5B0(40);
      v16 = _mm_loadu_si128(a4);
      *(long long *)(v15 + 24) = a2;
      v17 = (long long *)v15;
      v6 = 0;
      *(__m128i *)(v15 + 8) = v16;
      *(long long *)(v15 + 32) = sub_4C7A0(a3);
      v18 = *(long long *)(a1 + 8);
      *(long long *)(a1 + 8) = v17;
      *v17 = v18;
    }
    else
    {
      if ( qword_84D50 )
        qword_84D50(a4);
      v11 = sub_4AC50(a3);
      v12 = sub_22080(a4->m128i_i64[0], a4->m128i_i64[1], 0);
      v13 = dcgettext(0, "Substituting %s for unknown date format %s", 5);
      v14 = v12;
      v6 = 1;
      error(0, 0, v13, v14, v11);
      a4->m128i_i64[1] = 0;
    }
  }
  else
  {
    v6 = stat(a3, &v20);
    if ( v6 )
    {
      sub_32D30(a3);
      if ( qword_84D50 )
        qword_84D50(a3);
      v19 = dcgettext(0, "Date sample file not found", 5);
      error(0, 0, v19);
      sub_2C970(2);
    }
    a4->m128i_i64[0] = sub_4BD20(&v20);
    a4->m128i_i64[1] = v7;
  }
  return v6;
}



// Function: expand_pax_option @ 0x2cd70
long long expand_pax_option(long long a1, const char *a2)
{
  char *next_free; // r15
  char *chunk_limit; // r12
  unsigned long long v5; // r12
  size_t v6; // rbx
  char *v7; // rax
  char *v8; // rcx
  const char *v9; // rbp
  char v10; // al
  char *object_base; // rdi
  char *v12; // rdx
  long long v13; // r12
  size_t v15; // r14
  size_t v16; // r15
  char *v17; // r12
  char *v18; // r14
  const unsigned short **v19; // rax
  unsigned char v20; // dl
  char *v21; // r12
  char *v22; // rdi
  const char *v23; // r15
  size_t v24; // rax
  char *v25; // rdi
  size_t v26; // r14
  char *v27; // [rsp+8h] [rbp-100h]
  unsigned char v28; // [rsp+8h] [rbp-100h]
  char *v29; // [rsp+8h] [rbp-100h]
  __m128i v31; // [rsp+30h] [rbp-D8h] BYREF
  struct obstack v32; // [rsp+40h] [rbp-C8h] BYREF
  char v33[40]; // [rsp+A0h] [rbp-68h] BYREF
  unsigned long long v34; // [rsp+C8h] [rbp-40h]

  v34 = __readfsqword(0x28u);
  obstack_begin(&v32, 0, 0, sub_4C5B0, &free);
  while ( 1 )
  {
    next_free = v32.next_free;
    chunk_limit = v32.chunk_limit;
    if ( !*a2 )
      break;
    v5 = v32.chunk_limit - v32.next_free;
    v6 = strcspn(a2, ",");
    v7 = memchr(a2, 61, v6);
    v8 = v7;
    if ( !v7 )
    {
      if ( v6 > v5 )
      {
        obstack_newchunk(&v32, v6);
        next_free = v32.next_free;
      }
      memcpy(next_free, a2, v6);
      next_free = &v32.next_free[v6];
      v32.next_free += v6;
      goto LABEL_7;
    }
    v15 = v7 - a2 + 1;
    if ( v15 > v5 )
    {
      v29 = v7;
      obstack_newchunk(&v32, v7 - a2 + 1);
      next_free = v32.next_free;
      v8 = v29;
    }
    v27 = v8;
    memcpy(next_free, a2, v15);
    v16 = v6 - v15;
    v17 = &v32.next_free[v15];
    v18 = v27 + 1;
    v32.next_free = v17;
    v28 = v27[1];
    if ( v28 )
    {
      v19 = __ctype_b_loc();
      v20 = v28;
      while ( ((*v19)[v20] & 0x2000) != 0 )
      {
        v20 = *++v18;
        --v16;
        if ( !v20 )
          goto LABEL_24;
      }
      if ( v20 == 123 && v18[v16 - 1] == 125 )
      {
        v21 = (char *)sub_4C5B0(v16);
        memcpy(v21, v18 + 1, v16 - 2);
        v21[v16 - 2] = 0;
        if ( (unsigned int)sub_2CBE0(a1, (long long)"--pax-option", v21, &v31) )
        {
          v22 = v32.next_free;
          if ( v32.chunk_limit - v32.next_free < v16 )
          {
            obstack_newchunk(&v32, v16);
            v22 = v32.next_free;
          }
          memcpy(v22, v18, v16);
          v32.next_free += v16;
        }
        else
        {
          v23 = (const char *)sub_24270(v31.m128i_i64[0], v31.m128i_u32[2], (long long)v33);
          v24 = strlen(v23);
          v25 = v32.next_free;
          v26 = v24;
          if ( v24 > v32.chunk_limit - v32.next_free )
          {
            obstack_newchunk(&v32, v24);
            v25 = v32.next_free;
          }
          memcpy(v25, v23, v26);
          v32.next_free += v26;
        }
        free(v21);
        next_free = v32.next_free;
LABEL_7:
        v9 = &a2[v6];
        v10 = *v9;
        if ( !*v9 )
          goto LABEL_8;
        goto LABEL_27;
      }
    }
LABEL_24:
    if ( v16 > v32.chunk_limit - v17 )
    {
      obstack_newchunk(&v32, v16);
      v17 = v32.next_free;
    }
    v9 = &a2[v6];
    memcpy(v17, v18, v16);
    v10 = *v9;
    next_free = &v32.next_free[v16];
    v32.next_free = next_free;
    if ( !v10 )
    {
LABEL_8:
      chunk_limit = v32.chunk_limit;
      break;
    }
LABEL_27:
    if ( v32.chunk_limit == next_free )
    {
      obstack_newchunk(&v32, 1);
      next_free = v32.next_free;
      v10 = *v9;
    }
    a2 = v9 + 1;
    v32.next_free = next_free + 1;
    *next_free = v10;
  }
  if ( next_free == chunk_limit )
  {
    obstack_newchunk(&v32, 1);
    next_free = v32.next_free;
  }
  v32.next_free = next_free + 1;
  *next_free = 0;
  object_base = v32.object_base;
  if ( v32.next_free == v32.object_base )
    *((char *)&v32 + 80) |= 2u;
  v12 = v32.chunk_limit;
  if ( (~*(long long *)&v32.alignment_mask & (long long)&v32.next_free[*(long long *)&v32.alignment_mask])
     - (unsigned long long)v32.chunk <= v32.chunk_limit - (char *)v32.chunk )
    v12 = (char *)(~*(long long *)&v32.alignment_mask & (long long)&v32.next_free[*(long long *)&v32.alignment_mask]);
  v32.next_free = v12;
  v32.object_base = v12;
  v13 = sub_4C7A0(object_base);
  obstack_free(&v32, 0);
  return v13;
}



// Function: set_subcommand_option @ 0x2d160
long long set_subcommand_option(long long a1)
{
  long long result; // rax
  char *v2; // rax

  result = (unsigned int)dword_84BA8;
  if ( dword_84BA8 && dword_84BA8 != (int)a1 )
  {
    if ( qword_84D50 )
      qword_84D50(a1);
    v2 = dcgettext(0, "You may not specify more than one '-Acdtrux', '--delete' or  '--test-label' option", 5);
    error(0, 0, v2);
    sub_2C970(2);
  }
  dword_84BA8 = a1;
  return result;
}



// Function: option_conflict_error @ 0x2d1b0
void option_conflict_error(long long a1, long long a2)
{
  char *v2; // rax

  if ( qword_84D50 )
    qword_84D50(a1);
  v2 = dcgettext(0, "'%s' cannot be used with '%s'", 5);
  error(0, 0, v2, a1, a2);
  sub_2C970(2);
}



// Function: set_old_files_option @ 0x2d200
long long set_old_files_option(int a1, const char **a2)
{
  long long result; // rax

  result = sub_2BD60(9, a2);
  if ( result )
  {
    result = sub_2BD20((long long)a2, result);
    if ( (int)result )
    {
      result = (unsigned int)dword_84B34;
      if ( dword_84B34 != a1 )
        sub_2D1B0((long long)off_7F8A0[a1], (long long)off_7F8A0[dword_84B34]);
    }
  }
  dword_84B34 = a1;
  return result;
}



// Function: set_use_compress_program_option @ 0x2d260
int set_use_compress_program_option(char *s2, const char **a2)
{
  long long v3; // rax
  char *v4; // rdi
  int *v5; // rbp
  char *v6; // rax

  v3 = sub_2BD60(0, a2);
  v4 = qword_84B70;
  if ( qword_84B70 )
  {
    v5 = (int *)v3;
    LODWORD(v3) = strcmp(qword_84B70, s2);
    if ( (int)v3 )
    {
      if ( *v5 == 1 )
      {
        if ( qword_84D50 )
          qword_84D50(v4);
        v6 = dcgettext(0, "Conflicting compression options", 5);
        error(0, 0, v6);
        sub_2C970(2);
      }
    }
  }
  qword_84B70 = s2;
  return v3;
}



// Function: set_archive_format @ 0x2d2e0
long long set_archive_format(char *s2)
{
  const char *v2; // rdi
  _UNKNOWN **v3; // rbx
  long long result; // rax
  long long v5; // r12
  char *v6; // rax

  v2 = "v7";
  v3 = &off_7F9A0;
  while ( strcmp(v2, s2) )
  {
    v2 = (const char *)v3[2];
    v3 += 2;
    if ( !v2 )
    {
      if ( qword_84D50 )
        qword_84D50(0);
      v5 = sub_4AA70(s2);
      v6 = dcgettext(0, "%s: Invalid archive format", 5);
      error(0, 0, v6, v5);
      sub_2C970(2);
    }
  }
  result = *((unsigned int *)v3 + 2);
  dword_84BA4 = *((int *)v3 + 2);
  return result;
}



// Function: set_xattr_option @ 0x2d370
long long set_xattr_option(int a1)
{
  long long result; // rax

  if ( a1 == 1 )
  {
    result = sub_2D2E0("posix");
    dword_84A98 = 1;
  }
  else
  {
    dword_84A98 = a1;
  }
  return result;
}



// Function: parse_opt @ 0x2d3a0
long long parse_opt(long long a1, char *a2, long long *a3)
{
  const char ***v4; // r13
  char *v5; // r12
  long long *v6; // rax
  long long v7; // rdx
  long long v8; // rax
  long long v9; // rcx
  long long result; // rax
  long long v11; // rsi
  long long v12; // rsi
  unsigned int v13; // eax
  int v14; // edx
  int v15; // ecx
  int v16; // r8d
  int v17; // r9d
  long long v18; // rax
  char *v19; // rdi
  long long v20; // rax
  const char *v21; // rsi
  char *v22; // rax
  char *v23; // rbp
  long long v24; // rax
  int v25; // eax
  double v26; // xmm0_8
  long long v27; // rax
  long long *v28; // rdi
  char *v29; // rbp
  double v30; // xmm0_8
  const char **v31; // rax
  int v32; // edx
  long long v33; // r12
  char *v34; // rax
  long long v35; // rbx
  long long v36; // r12
  char *v37; // rax
  const char *v38; // rsi
  char *v39; // rbx
  const char *v40; // rax
  char *v41; // rbx
  const char *v42; // rax
  const char *v43; // rsi
  char *v44; // rax
  const char *v45; // rsi
  char *v46; // rax
  char *v47; // rax
  long long v48; // [rsp+8h] [rbp-40h]
  char *endptr[7]; // [rsp+10h] [rbp-38h] BYREF

  v4 = (const char ***)a3[5];
  endptr[1] = (char *)__readfsqword(0x28u);
  if ( (int)a1 > 207 )
  {
    if ( (int)a1 == 16777219 )
    {
      v6 = *(long long **)(*a3 + 32LL);
      if ( v6 && *v6 )
      {
        v7 = 32;
        v8 = 0;
        while ( 1 )
        {
          *(long long *)(a3[6] + v8) = v4;
          v8 += 8;
          v9 = *(long long *)(*(long long *)(*a3 + 32LL) + v7);
          v7 += 32;
          if ( !v9 )
            break;
          v4 = (const char ***)a3[5];
        }
      }
      return 0;
    }
    else
    {
      if ( (int)a1 == 16777221 )
      {
        v31 = *v4;
        v32 = *(int *)*v4;
        if ( v32 == 2 )
        {
          v35 = (long long)v31[2];
          v36 = (long long)v31[1];
          v37 = dcgettext(0, "%s:%lu: location of the error", 5);
          error(0, 0, v37, v36, v35);
        }
        else if ( !v32 )
        {
          v33 = (long long)v31[1];
          v34 = dcgettext(0, "error parsing %s", 5);
          error(0, 0, v34, v33);
        }
        exit(64);
      }
      return 7;
    }
  }
  else
  {
    if ( (int)a1 < 0 )
      return 7;
    v5 = a2;
    switch ( (int)a1 )
    {
      case 0:
        sub_26B50((long long)a2);
        return (unsigned int)a1;
      case 48:
      case 49:
      case 50:
      case 51:
      case 52:
      case 53:
      case 54:
      case 55:
        v13 = (unsigned int)dcgettext(0, "Options '-[0-7][lmh]' not supported by *this* tar", 5);
        argp_error((int)a3, v13, v14, v15, v16, v17);
        exit(64);
      case 65:
        sub_2D160(2);
        return 0;
      case 66:
        byte_84AB9 = 1;
        return 0;
      case 70:
        qword_84B48 = (long long)a2;
        result = 0;
        byte_84B10 = 1;
        return result;
      case 71:
        goto LABEL_24;
      case 72:
        sub_2D2E0(a2);
        return 0;
      case 73:
        sub_2D260(a2, *v4);
        return 0;
      case 74:
        sub_2D260("xz", *v4);
        return 0;
      case 75:
        sub_2BD60(5, *v4);
        sub_27190(a2);
        return 0;
      case 76:
        if ( !(unsigned int)sub_4C870(a2) )
        {
          if ( endptr[0] <= a2 || strchr("bBcGgkKMmPTtw", *(endptr[0] - 1)) )
          {
            if ( v48 < 0 )
              v26 = (double)(int)(v48 & 1 | ((unsigned long long)v48 >> 1))
                  + (double)(int)(v48 & 1 | ((unsigned long long)v48 >> 1));
            else
              v26 = (double)(int)v48;
          }
          else
          {
            if ( v48 < 0 )
              v30 = (double)(int)(v48 & 1 | ((unsigned long long)v48 >> 1))
                  + (double)(int)(v48 & 1 | ((unsigned long long)v48 >> 1));
            else
              v30 = (double)(int)v48;
            v26 = v30 * 1024.0;
          }
          byte_84B10 = 1;
          qword_84A70 = *(long long *)&v26;
          return 0;
        }
        if ( qword_84D50 )
          qword_84D50(a2);
        v38 = "Invalid tape length";
        goto LABEL_200;
      case 77:
        byte_84B10 = 1;
        return 0;
      case 78:
        dword_84B8C = 1;
        if ( xmmword_84B00 >= 0 )
          goto LABEL_211;
        v21 = "--after-date";
        goto LABEL_70;
      case 79:
        byte_84A6A = 1;
        return 0;
      case 80:
        sub_2BD60(8, *v4);
        byte_84B92 = 1;
        return 0;
      case 82:
        byte_84B7C = 1;
        return 0;
      case 83:
        byte_84A88 = 1;
        return 0;
      case 85:
        sub_2D200(3, *v4);
        return 0;
      case 86:
        name = a2;
        return 0;
      case 87:
        sub_2BD60(4, *v4);
        byte_84A50 = 1;
        return 0;
      case 90:
        sub_2D260("compress", *v4);
        return 0;
      case 97:
        *((char *)v4 + 18) = 1;
        return 0;
      case 98:
        if ( !(unsigned int)sub_4C870(a2) )
        {
          dword_84BA0 = (int)endptr[0];
          if ( (char *)SLODWORD(endptr[0]) == endptr[0] && SLODWORD(endptr[0]) > 0 )
          {
            nbytes = (long long)endptr[0] << 9;
            if ( endptr[0] == (char *)((unsigned long long)endptr[0] & 0x7FFFFFFFFFFFFFLL) )
              return 0;
          }
        }
        if ( qword_84D50 )
          qword_84D50(a2);
        v38 = "Invalid blocking factor";
        goto LABEL_200;
      case 99:
        sub_2D160(3);
        return 0;
      case 100:
        sub_2D160(5);
        return 0;
      case 102:
        v27 = qword_84830;
        v28 = ptr;
        if ( qword_84830 == qword_84C18 )
        {
          ptr = (void *)sub_4C680(ptr, &qword_84C18, 8);
          v28 = ptr;
          v27 = qword_84830;
        }
        qword_84830 = v27 + 1;
        v28[v27] = a2;
        return 0;
      case 103:
        sub_2BD60(2, *v4);
        qword_84B28 = a2;
        dword_84B8C = 1;
LABEL_24:
        byte_84B50 = 1;
        return 0;
      case 104:
        byte_84B69 = 1;
        return 0;
      case 105:
        byte_84B51 = 1;
        return 0;
      case 106:
        sub_2D260("bzip2", *v4);
        return 0;
      case 107:
        sub_2D200(4, *v4);
        return 0;
      case 108:
        dword_84C20 = 1;
        return 0;
      case 109:
        byte_84A68 = 1;
        return 0;
      case 110:
        dword_847FC = 1;
        return 0;
      case 111:
        *((char *)v4 + 16) = 1;
        return 0;
      case 112:
        dword_84AA4 = 1;
        return 0;
      case 114:
        sub_2D160(1);
        return 0;
      case 115:
        sub_2BD60(6, *v4);
        byte_84AAC = 1;
        return 0;
      case 116:
        sub_2D160(7);
        ++dword_84A54;
        return 0;
      case 117:
        sub_2D160(8);
        return 0;
      case 118:
        ++dword_84A54;
        result = 0;
        dword_83B80 |= 0x583000u;
        return result;
      case 119:
        byte_84B40 = 1;
        return 0;
      case 120:
        sub_2D160(6);
        return 0;
      case 122:
        sub_2D260("gzip", *v4);
        return 0;
      case 128:
        sub_2D2E0("posix");
        result = 0;
        dword_84A9C = 1;
        return result;
      case 129:
        v25 = 1;
        if ( a2 )
          v25 = dword_66A90[sub_39670("--atime-preserve", a2, &off_7F980, dword_66A90, 4, off_83DD8)];
        dword_84B88 = v25;
        return 0;
      case 130:
        byte_84B84 = 1;
        if ( !a2 )
          return 0;
        v4[4] = (const char **)a2;
        return 0;
      case 131:
        byte_84B20 = 1;
        return 0;
      case 132:
        if ( a2 )
        {
          if ( *a2 == 46 )
          {
            v5 = a2 + 1;
            sub_10490(".");
          }
          dword_84B78 = strtoul(v5, endptr, 0);
          if ( *endptr[0] )
          {
            if ( qword_84D50 )
              qword_84D50(v5);
            v43 = "--checkpoint value is not an integer";
LABEL_210:
            v44 = dcgettext(0, v43, 5);
            error(0, 0, v44);
            sub_163D0();
          }
          return 0;
        }
        else
        {
          dword_84B78 = 10;
          return 0;
        }
      case 133:
        sub_10490(a2);
        return 0;
      case 134:
        dword_84AF0 = 2;
        return 0;
      case 135:
        byte_847E0 = 1;
        return 0;
      case 136:
        byte_84B68 = 1;
        return 0;
      case 137:
        sub_2D160(4);
        return 0;
      case 138:
        byte_84D70 = 1;
        return 0;
      case 139:
        byte_84B90 = 1;
        return 0;
      case 140:
        v24 = sub_2BF00(a2);
        if ( v24 != -1 )
        {
          dword_84B54 = v24;
          return 0;
        }
        dword_84B54 = -1;
        if ( !qword_84B58 )
          return 0;
        sub_26AA0(qword_84B58, &dword_84B54);
        return 0;
      case 141:
        sub_23A70((long long)a2);
        return 0;
      case 142:
        byte_84A59 = 1;
        return 0;
      case 143:
        byte_84B52 = 1;
        return 0;
      case 144:
        qword_84820 = a2;
        return 0;
      case 145:
        byte_84B30 = 1;
        return 0;
      case 146:
        sub_2D200(6, *v4);
        return 0;
      case 147:
        dword_84B24 = strtoul(a2, endptr, 10);
        if ( !*endptr[0] )
          return 0;
        if ( qword_84D50 )
          qword_84D50(a2);
        v45 = "Invalid incremental level value";
        goto LABEL_214;
      case 148:
        sub_2D260("lzip", *v4);
        return 0;
      case 149:
        sub_2D260("lzma", *v4);
        return 0;
      case 150:
        sub_2D260("lzop", *v4);
        return 0;
      case 151:
        qword_84B18 = sub_435A0(a2);
        if ( qword_84B18 )
        {
          dword_84B14 = umask(0);
          umask(dword_84B14);
          return 0;
        }
        if ( qword_84D50 )
          qword_84D50(a2);
        v43 = "Invalid mode given on option";
        goto LABEL_210;
      case 152:
        sub_2CBE0((long long)v4, (long long)"--mtime", a2, (__m128i *)&xmmword_84AE0);
        if ( dword_84AF0 )
          return 0;
        dword_84AF0 = 1;
        return 0;
      case 153:
        if ( xmmword_84B00 >= 0 )
        {
LABEL_211:
          if ( qword_84D50 )
            qword_84D50(a1);
          v45 = "More than one threshold date";
LABEL_214:
          v46 = dcgettext(0, v45, 5);
          error(0, 0, v46);
          sub_2C970(2);
        }
        v21 = "--newer-mtime";
LABEL_70:
        sub_2CBE0((long long)v4, (long long)v21, v5, (__m128i *)&xmmword_84B00);
        sub_2BD60(3, *v4);
        return 0;
      case 154:
        dword_84A9C = -1;
        return 0;
      case 155:
        *((char *)v4 + 18) = 0;
        return 0;
      case 156:
        byte_84B20 = 0;
        return 0;
      case 157:
        byte_847E0 = 0;
        return 0;
      case 158:
        byte_84A59 = 0;
        return 0;
      case 159:
        sub_2D200(1, *v4);
        return 0;
      case 160:
        while ( 1 )
        {
          v12 = (unsigned int)*v5;
          if ( !*v5 )
            break;
          ++v5;
          sub_4A5B0(0, v12, 0);
        }
        return 0;
      case 161:
        dword_84AA8 = -1;
        return 0;
      case 162:
        dword_84AA4 = -1;
        return 0;
      case 163:
        dword_847FC = 0;
        return 0;
      case 164:
        dword_84AA0 = -1;
        return 0;
      case 165:
        sub_2D370(-1);
        return 0;
      case 166:
        byte_84AD2 = 1;
        return 0;
      case 167:
        sub_2BD60(1, *v4);
        if ( a2 )
        {
          if ( (unsigned int)sub_4C870(a2) )
          {
            if ( qword_84D50 )
              qword_84D50(a2);
            v41 = dcgettext(0, "Invalid number", 5);
            v42 = (const char *)sub_4AA70(a2);
            error(0, 0, "%s: %s", v42, v41);
            sub_163D0();
          }
          qword_84B38 = (long long)endptr[0];
          return 0;
        }
        else
        {
          qword_84B38 = 1;
          return 0;
        }
      case 168:
        sub_2D2E0("v7");
        return 0;
      case 169:
        byte_84AD1 = 1;
        return 0;
      case 170:
        sub_2BD60(7, *v4);
        byte_84AD0 = 1;
        result = 0;
        qword_84AC8 = a2;
        return result;
      case 171:
        sub_2D200(0, *v4);
        return 0;
      case 172:
        sub_2D200(2, *v4);
        return 0;
      case 173:
        v18 = sub_2BF00(a2);
        if ( v18 != -1 )
        {
          dword_84ABC = v18;
          return 0;
        }
        dword_84ABC = -1;
        if ( !qword_84AC0 )
          return 0;
        sub_269F0(qword_84AC0, &dword_84ABC);
        return 0;
      case 174:
        sub_239A0((long long)a2);
        return 0;
      case 175:
        v22 = (char *)sub_2CD70((long long)v4, a2);
        *((char *)v4 + 17) = 1;
        v23 = v22;
        sub_1C010(v22);
        free(v23);
        return 0;
      case 176:
        sub_2D2E0("posix");
        return 0;
      case 177:
        while ( 1 )
        {
          v11 = (unsigned int)*v5;
          if ( !*v5 )
            break;
          ++v5;
          sub_4A5B0(0, v11, 1);
        }
        return 0;
      case 178:
        sub_2BDE0(a2);
        return 0;
      case 179:
        if ( (unsigned int)sub_4C870(a2) )
        {
          if ( qword_84D50 )
            qword_84D50(a2);
          v38 = "Invalid record size";
LABEL_200:
          v39 = dcgettext(0, v38, 5);
          v40 = (const char *)sub_4AA70(v5);
          error(0, 0, "%s: %s", v40, v39);
          sub_2C970(2);
        }
        nbytes = (size_t)endptr[0];
        if ( ((long long)endptr[0] & 0x1FF) != 0 )
        {
          if ( qword_84D50 )
            qword_84D50(a2);
          v47 = dcgettext(0, "Record size must be a multiple of %d.", 5);
          error(0, 0, v47, 512);
          sub_2C970(2);
        }
        dword_84BA0 = (unsigned long long)endptr[0] >> 9;
        result = 0;
        break;
      case 180:
        byte_84ABA = 1;
        return 0;
      case 181:
        byte_84AB8 = 1;
        return 0;
      case 182:
        byte_84A58 = 1;
        return 0;
      case 183:
        off_83C60 = a2;
        return 0;
      case 184:
        qword_84AB0 = (long long)a2;
        return 0;
      case 185:
        dword_84AA8 = 1;
        return 0;
      case 186:
        sub_2D2E0("posix");
        result = 0;
        dword_84AA0 = 1;
        return result;
      case 187:
        v29 = (char *)sub_2C280();
        __printf_chk(1, "%s\n", v29);
        sub_3E440();
        free(v29);
        exit(0);
      case 188:
        byte_84A89 = 1;
        return 0;
      case 189:
        sub_1FC70();
        sub_3E440();
        exit(0);
      case 190:
        byte_847E1 = 1;
        return 0;
      case 191:
        sub_2D200(5, *v4);
        return 0;
      case 192:
        dword_847E4 = dword_81AE8[sub_39670("--sort", a2, &off_7F960, dword_81AE8, 4, off_83DD8)];
        return 0;
      case 193:
        v20 = sub_39670("--hole-detection", a2, &off_7F930, dword_66A88, 4, off_83DD8);
        byte_84A88 = 1;
        dword_84A7C = dword_66A88[v20];
        return 0;
      case 194:
        v19 = a2;
        byte_84A88 = 1;
        dword_84A84 = strtoul(a2, endptr, 10);
        if ( !*endptr[0] )
          return 0;
        if ( *endptr[0] == 46 )
        {
          v19 = endptr[0] + 1;
          dword_84A80 = strtoul(endptr[0] + 1, endptr, 10);
          if ( !*endptr[0] )
            return 0;
        }
        if ( qword_84D50 )
          qword_84D50(v19);
        v45 = "Invalid sparse version value";
        goto LABEL_214;
      case 195:
        if ( !(unsigned int)sub_4C870(a2) )
        {
          qword_84A90 = (long long)endptr[0];
          return 0;
        }
        if ( qword_84D50 )
          qword_84D50(a2);
        v38 = "Invalid number of elements";
        goto LABEL_200;
      case 196:
        byte_84B84 = 1;
        result = 0;
        v4[3] = (const char **)a2;
        return result;
      case 197:
        sub_2D160(9);
        return 0;
      case 198:
        if ( a2 )
          sub_2C950(a2);
        else
          byte_84A69 = 1;
        return 0;
      case 199:
        if ( !qword_84A60 )
        {
          qword_84A60 = (long long)a2;
          return 0;
        }
        if ( qword_84D50 )
          qword_84D50(a1);
        v45 = "Only one --to-command option allowed";
        goto LABEL_214;
      case 200:
        sub_30710(a2);
        return 0;
      case 201:
        byte_84B91 = 1;
        return 0;
      case 202:
        filename = a2;
        return 0;
      case 203:
        sub_31470(a2);
        return 0;
      case 204:
        sub_2D370(1);
        return 0;
      case 205:
      case 206:
        sub_2D370(1);
        sub_31AF0(a2, (int)a1 == 206);
        return 0;
      case 207:
        sub_2D260("zstd", *v4);
        return 0;
      default:
        return 7;
    }
  }
  return result;
}



// Function: parse_default_options @ 0x2e4b0
unsigned long long parse_default_options(long long *a1)
{
  char *v2; // rax
  char *v3; // rdi
  long long *v4; // rdx
  long long v5; // r12
  long long v6; // rsi
  long long v8; // r12
  char *v9; // rax
  const char *v10; // r12
  char *v11; // rax
  char v12[4]; // [rsp+Ch] [rbp-12Ch] BYREF
  int v13; // [rsp+10h] [rbp-128h] BYREF
  const char *v14; // [rsp+18h] [rbp-120h]
  long long v15; // [rsp+20h] [rbp-118h]
  long long v16; // [rsp+28h] [rbp-110h]
  long long v17; // [rsp+30h] [rbp-108h] BYREF
  long long *v18; // [rsp+38h] [rbp-100h]
  long long v19; // [rsp+40h] [rbp-F8h]
  unsigned long long v20; // [rsp+118h] [rbp-20h]

  v20 = __readfsqword(0x28u);
  v2 = getenv("TAR_OPTIONS");
  v13 = 0;
  v14 = "TAR_OPTIONS";
  v15 = 0;
  v16 = 0;
  if ( v2 )
  {
    v3 = v2;
    v19 = 1;
    if ( (unsigned int)sub_382C0(v2, &v17, 33558086) )
    {
      if ( qword_84D50 )
        qword_84D50(v3);
      v8 = sub_354B0(&v17);
      v9 = dcgettext(0, "cannot split TAR_OPTIONS: %s", 5);
      error(0, 0, v9, v8);
      sub_163D0();
    }
    if ( v17 )
    {
      v4 = v18;
      v5 = *a1;
      v6 = (unsigned int)(v19 + v17);
      *v18 = qword_84DD0;
      *a1 = (long long)&v13;
      if ( (unsigned int)argp_parse(&off_81B20, v6, v4, 40, v12, a1) )
        abort();
      *a1 = v5;
      if ( (unsigned char)sub_2BCA0() )
      {
        if ( qword_84D50 )
          qword_84D50(&off_81B20);
        v10 = v14;
        v11 = dcgettext(0, "non-option arguments in %s", 5);
        error(0, 0, v11, v10);
        sub_2C970(2);
      }
      v17 = 0;
    }
    sub_35420(&v17);
  }
  return v20 - __readfsqword(0x28u);
}



// Function: decode_options @ 0x2e690
unsigned long long decode_options(int a1, long long *a2)
{
  long long *v2; // r12
  long long v3; // rbp
  char *v4; // rax
  long long v5; // rax
  const char *v6; // rdi
  long long v7; // rsi
  char *v8; // rdi
  int v9; // ebx
  long long v10; // r8
  int j; // eax
  unsigned long long v12; // rax
  char *v13; // rbp
  char *v14; // rbp
  size_t v15; // r12
  long long v16; // rdx
  char *v17; // rbp
  size_t v18; // rdx
  long long v19; // rcx
  int v20; // ebx
  double v21; // xmm0_8
  double v22; // xmm1_8
  char *v23; // rbp
  int v24; // edx
  int v25; // edx
  int v26; // edx
  long long *v27; // r13
  int v28; // eax
  long long *v29; // rax
  char *v30; // rbx
  long long *v31; // r14
  long long *v32; // r12
  char i; // al
  long long v34; // rax
  long long v35; // rdi
  long long v36; // rax
  long long *v37; // rdi
  long long *v38; // rsi
  const char *v39; // rsi
  char *v40; // rbx
  long long v41; // r12
  const char **v42; // rax
  const char **v43; // rbx
  const char **v44; // r12
  const char *v45; // rdi
  char *v47; // rbp
  FILE *v48; // rax
  char **v49; // rbp
  char *v50; // rax
  char *v51; // rax
  unsigned int v52; // r12d
  char *v53; // rax
  const char *v54; // rax
  const char *v55; // rax
  const char *v56; // rax
  long long v57; // r13
  char *v58; // rax
  const char *v59; // rax
  int v60; // [rsp+4h] [rbp-B4h]
  long long *v61; // [rsp+8h] [rbp-B0h]
  int v62; // [rsp+1Ch] [rbp-9Ch] BYREF
  int v63; // [rsp+20h] [rbp-98h] BYREF
  long long v64; // [rsp+28h] [rbp-90h]
  long long v65; // [rsp+30h] [rbp-88h]
  long long v66; // [rsp+38h] [rbp-80h]
  int *v67; // [rsp+40h] [rbp-78h] BYREF
  void *ptr; // [rsp+48h] [rbp-70h]
  short v69; // [rsp+50h] [rbp-68h]
  char v70; // [rsp+52h] [rbp-66h]
  void *v71; // [rsp+58h] [rbp-60h]
  long long v72; // [rsp+60h] [rbp-58h]
  char src[3]; // [rsp+75h] [rbp-43h] BYREF
  unsigned long long v74; // [rsp+78h] [rbp-40h]

  v2 = a2;
  v3 = a1;
  v74 = __readfsqword(0x28u);
  v69 = 0;
  v67 = &v63;
  v63 = 1;
  v64 = 0;
  v65 = 0;
  v66 = 0;
  ptr = 0;
  v70 = 0;
  v71 = 0;
  v72 = 0;
  sub_3DA30("tar", off_81B00);
  v71 = getenv("SIMPLE_BACKUP_SUFFIX");
  v4 = getenv("POSIXLY_CORRECT");
  dword_84BA8 = 0;
  dword_84BA4 = 0;
  dword_84BA0 = 20;
  nbytes = 10240;
  byte_84A38 = v4 != 0;
  v5 = sub_3EBA0();
  byte_847E8 = 1;
  qword_84B60 = v5;
  dword_84A7C = 0;
  *(long long *)&xmmword_84B00 = 0x8000000000000000LL;
  *((long long *)&xmmword_84B00 + 1) = -1;
  *(long long *)&xmmword_84AE0 = 0x8000000000000000LL;
  *((long long *)&xmmword_84AE0 + 1) = -1;
  dword_84AD4 = 8;
  dword_84A84 = 1;
  dword_84A80 = 0;
  dword_847E4 = 0;
  dword_84ABC = -1;
  qword_84AC0 = 0;
  dword_84B54 = -1;
  qword_84B58 = 0;
  byte_84B20 = 1;
  dword_84B24 = -1;
  dword_847FC = -1;
  if ( a1 > 1 )
  {
    v6 = (const char *)a2[1];
    if ( *v6 != 45 )
    {
      src[0] = 45;
      v27 = a2 + 2;
      src[2] = 0;
      v28 = v3 + strlen(v6);
      v60 = v28 - 1;
      v29 = (long long *)sub_4C5B0(8LL * v28);
      v30 = (char *)a2[1];
      v61 = v29;
      v31 = v29 + 1;
      v32 = &a2[v3];
      *v29 = *a2;
      for ( i = *v30; *v30; i = *v30 )
      {
        while ( 1 )
        {
          src[1] = i;
          *v31 = sub_4C7A0(src);
          v35 = (unsigned int)*v30;
          v36 = sub_2BEB0(v35);
          if ( v36 )
          {
            if ( *(long long *)(v36 + 16) )
              break;
          }
          i = *++v30;
          ++v31;
          if ( !i )
            goto LABEL_95;
        }
        if ( v27 >= v32 )
        {
          if ( qword_84D50 )
            qword_84D50(v35);
          v52 = *v30;
          v53 = dcgettext(0, "Old option '%c' requires an argument.", 5);
          error(0, 0, v53, v52);
          sub_2C970(2);
        }
        v34 = *v27;
        ++v30;
        v31 += 2;
        ++v27;
        *(v31 - 1) = v34;
      }
LABEL_95:
      if ( v27 < v32 )
      {
        v37 = v31;
        v38 = v27;
        do
          *v37++ = *v38++;
        while ( v38 < v32 );
        v31 += ((unsigned long long)((char *)v32 - 1 - (char *)v27) >> 3) + 1;
      }
      v2 = v61;
      LODWORD(v3) = v60;
      *v31 = 0;
    }
  }
  sub_2E4B0((long long *)&v67);
  v7 = (unsigned int)v3;
  v8 = (char *)&off_81B20;
  v9 = argp_parse(&off_81B20, (unsigned int)v3, v2, 8, &v62, &v67);
  if ( v9 )
    exit(2);
  if ( (char)v69 )
  {
    if ( dword_84BA8 == 3 )
    {
      v8 = "v7";
      sub_2D2E0("v7");
    }
    else
    {
      dword_84AA8 = -1;
    }
  }
  for ( j = v62; j < (int)v3; v62 = j )
  {
    v8 = (char *)v2[j];
    sub_26B50((long long)v8);
    j = v62 + 1;
  }
  if ( !dword_84BA4 )
  {
    if ( HIBYTE(v69) )
      dword_84BA4 = 4;
    else
      dword_84BA4 = 6;
  }
  if ( name && dword_84BA8 == 3 || byte_84B50 || byte_84B10 || byte_84A88 )
    sub_2E640();
  if ( qword_84B38 )
  {
    if ( !(unsigned char)((long long (*)(void))sub_2BCA0)() )
    {
      if ( qword_84D50 )
        qword_84D50(v8);
      v39 = "--occurrence is meaningless without a file list";
      goto LABEL_159;
    }
    if ( (byte_66A60[4 * dword_84BA8] & 0x10) == 0 )
    {
      v8 = (char *)(&dword_0 + 1);
      if ( sub_2BCC0(1) )
      {
        v55 = sub_2C7E0(v26);
        sub_2D1B0((long long)"--occurrence", (long long)v55);
      }
      qword_84B38 = 0;
    }
  }
  v12 = qword_84830;
  if ( !qword_84830 )
  {
    v49 = (char **)::ptr;
    v8 = "TAPE";
    qword_84830 = 1;
    *v49 = getenv("TAPE");
    if ( !*(long long *)::ptr )
      *(long long *)::ptr = "-";
    v12 = qword_84830;
  }
  if ( v12 > 1 && !byte_84B10 )
  {
    if ( qword_84D50 )
      qword_84D50(v8);
    v39 = "Multiple archive files require '-M' option";
    goto LABEL_159;
  }
  if ( qword_84B28 )
  {
    if ( xmmword_84B00 < 0 )
      goto LABEL_26;
    v13 = (char *)sub_2BCB0(2);
    v8 = v13;
    v7 = sub_2BCB0(3);
    if ( sub_2BD20((long long)v13, v7) )
      sub_2D1B0((long long)"--listed-incremental", (long long)"--newer");
    if ( *(int *)v13 != 1 )
    {
      xmmword_84B00 = 0;
      goto LABEL_26;
    }
    qword_84B28 = 0;
  }
  if ( dword_84B24 != -1 )
  {
    if ( qword_84D50 )
      qword_84D50(v8);
    v51 = dcgettext(0, "--level is meaningless without --listed-incremental", 5);
    v7 = 0;
    v8 = 0;
    error(0, 0, v51);
  }
LABEL_26:
  v14 = name;
  if ( name )
  {
    if ( (dword_84BA4 & 0xFFFFFFFB) == 2 )
    {
      v8 = name;
      v15 = byte_84B10 == 0 ? 99LL : 81LL;
      if ( strlen(name) > v15 )
      {
        if ( qword_84D50 )
        {
          qword_84D50(v8);
          v14 = name;
        }
        v57 = sub_4AA70(v14);
        v58 = dcngettext(
                0,
                "%s: Volume label is too long (limit is %lu byte)",
                "%s: Volume label is too long (limit is %lu bytes)",
                v15,
                5);
        error(0, 0, v58, v57, v15);
        sub_2C970(2);
      }
    }
  }
  if ( byte_84A50 )
  {
    if ( byte_84B10 )
    {
      if ( qword_84D50 )
        qword_84D50(v8);
      v39 = "Cannot verify multi-volume archives";
      goto LABEL_159;
    }
    if ( qword_84B70 )
    {
      if ( qword_84D50 )
        qword_84D50(v8);
      v39 = "Cannot verify compressed archives";
      goto LABEL_159;
    }
    if ( (byte_66A60[4 * dword_84BA8] & 2) == 0 )
    {
      v8 = &byte_4;
      if ( sub_2BCC0(4) )
      {
        v54 = sub_2C7E0(v25);
        sub_2D1B0((long long)"--verify", (long long)v54);
      }
      byte_84A50 = 0;
    }
  }
  else if ( qword_84B70 )
  {
    if ( byte_84B10 )
    {
      if ( qword_84D50 )
        qword_84D50(v8);
      v39 = "Cannot use multi-volume compressed archives";
      goto LABEL_159;
    }
    if ( (byte_66A60[4 * dword_84BA8] & 4) != 0 )
    {
      if ( qword_84D50 )
        qword_84D50(v8);
      v39 = "Cannot update compressed archives";
      goto LABEL_159;
    }
    if ( dword_84BA8 == 2 )
    {
      if ( qword_84D50 )
        qword_84D50(v8);
      v39 = "Cannot concatenate compressed archives";
      goto LABEL_159;
    }
  }
  if ( dword_84AF0 == 2 && xmmword_84AE0 < 0 )
  {
    if ( qword_84D50 )
      qword_84D50(v8);
    v39 = "--clamp-mtime needs a date specified using --mtime";
    goto LABEL_159;
  }
  if ( !HIBYTE(v69) || dword_84BA4 == 4 )
  {
    if ( dword_84A9C > 0 )
    {
      if ( dword_84BA4 == 4 )
      {
        if ( dword_84AA0 > 0 )
          goto LABEL_71;
        goto LABEL_108;
      }
      if ( (byte_66A60[4 * dword_84BA8] & 1) != 0 )
        goto LABEL_39;
      if ( qword_84D50 )
        qword_84D50(v8);
      v39 = "--acls can be used only on POSIX archives";
LABEL_159:
      v50 = dcgettext(0, v39, 5);
      error(0, 0, v50);
      sub_2C970(2);
    }
    if ( dword_84AA0 <= 0 )
      goto LABEL_108;
    if ( dword_84BA4 == 4 )
      goto LABEL_71;
    v16 = (unsigned int)dword_84BA8;
LABEL_103:
    if ( (byte_66A60[4 * v16] & 1) != 0 )
      goto LABEL_40;
    if ( qword_84D50 )
      qword_84D50(v8);
    v39 = "--selinux can be used only on POSIX archives";
    goto LABEL_159;
  }
  v16 = (unsigned int)dword_84BA8;
  if ( (byte_66A60[4 * dword_84BA8] & 1) == 0 )
  {
    if ( qword_84D50 )
      qword_84D50(v8);
    v39 = "--pax-option can be used only on POSIX archives";
    goto LABEL_159;
  }
  if ( dword_84A9C <= 0 )
  {
    if ( dword_84AA0 <= 0 )
      goto LABEL_108;
    goto LABEL_103;
  }
LABEL_39:
  if ( dword_84AA0 > 0 )
    goto LABEL_40;
LABEL_108:
  if ( dword_84A98 > 0 && dword_84BA4 != 4 )
  {
    if ( (byte_66A60[4 * dword_84BA8] & 1) == 0 )
    {
      if ( qword_84D50 )
        qword_84D50(v8);
      v39 = "--xattrs can be used only on POSIX archives";
      goto LABEL_159;
    }
LABEL_40:
    if ( byte_84A78 )
      goto LABEL_45;
LABEL_41:
    if ( !byte_84AAC )
      goto LABEL_45;
    v10 = (unsigned int)dword_84BA8;
    if ( (byte_66A60[4 * dword_84BA8] & 1) != 0 )
      goto LABEL_45;
    goto LABEL_43;
  }
LABEL_71:
  if ( !byte_84A78 )
    goto LABEL_41;
  v10 = (unsigned int)dword_84BA8;
  if ( (byte_66A60[4 * dword_84BA8] & 1) == 0 )
  {
    v8 = &byte_5;
    if ( sub_2BCC0(5) )
    {
      v59 = sub_2C7E0(v24);
      sub_2D1B0((long long)"--starting-file", (long long)v59);
    }
    byte_84A78 = 0;
    if ( byte_84AAC )
    {
LABEL_43:
      v8 = &byte_6;
      if ( sub_2BCC0(6) )
      {
        v56 = sub_2C7E0(v10);
        sub_2D1B0((long long)"--same-order", (long long)v56);
      }
      byte_84AAC = 0;
    }
  }
LABEL_45:
  if ( !byte_84AD0 )
    goto LABEL_50;
  if ( byte_84B92 )
  {
    v17 = (char *)sub_2BCB0(7);
    v8 = v17;
    v7 = sub_2BCB0(8);
    if ( sub_2BD20((long long)v17, v7) )
      sub_2D1B0((long long)"--one-top-level", (long long)"--absolute-names");
    if ( *(int *)v17 != 1 )
    {
      byte_84AD0 = 0;
      goto LABEL_50;
    }
    byte_84B92 = 0;
  }
  if ( !qword_84AC8 )
  {
    v8 = (char *)sub_3E510(*(long long *)::ptr);
    qword_84AC8 = (char *)sub_2A280(v8);
    free(v8);
    if ( !qword_84AC8 )
    {
      if ( qword_84D50 )
        qword_84D50(v8);
      v39 = "Cannot deduce top-level directory name; please set it explicitly with --one-top-level=DIR";
      goto LABEL_159;
    }
  }
LABEL_50:
  if ( byte_84ABA )
    dword_84B34 = 3;
  if ( byte_84B69 )
  {
    v18 = (size_t)&qword_80900;
    if ( dword_84B88 != 2 )
    {
      v19 = 592128;
      goto LABEL_55;
    }
    v9 = 0x40000;
    goto LABEL_135;
  }
  if ( dword_84B88 != 2 )
  {
    LODWORD(v18) = 657664;
LABEL_135:
    v18 = v9 | (unsigned int)v18;
    v19 = (unsigned int)v18 | 0x10000;
    v9 = ((unsigned char)byte_84B69 ^ 1) << 8;
    goto LABEL_55;
  }
  v18 = 919808;
  v19 = 985344;
  v9 = 256;
LABEL_55:
  flag = v9;
  v20 = dword_84BA8;
  dword_84808 = v18;
  dword_84804 = v19;
  if ( dword_84BA8 == 9 )
  {
    if ( !(unsigned char)sub_2BCA0(v8, v7, v18, v19, v10) )
      ++dword_84A54;
  }
  else if ( byte_84B91 )
  {
    dword_84A54 = 2;
  }
  v21 = *(double *)&qword_84A70;
  if ( *(double *)&qword_84A70 != 0.0 )
  {
    if ( (nbytes & 0x8000000000000000LL) != 0LL )
    {
      v18 = nbytes & 1 | (nbytes >> 1);
      v22 = (double)(int)v18 + (double)(int)v18;
    }
    else
    {
      v22 = (double)(int)nbytes;
    }
    if ( v22 > *(double *)&qword_84A70 )
    {
      if ( qword_84D50 )
        qword_84D50(v8);
      v39 = "Volume length cannot be less than record size";
      goto LABEL_159;
    }
  }
  if ( byte_84AAC && qword_84B28 )
  {
    v23 = (char *)sub_2BCB0(6);
    v8 = v23;
    v7 = sub_2BCB0(2);
    if ( sub_2BD20((long long)v23, v7) )
      sub_2D1B0((long long)"--preserve-order", (long long)"--listed-incremental");
    if ( *(int *)v23 == 1 )
      qword_84B28 = 0;
    else
      byte_84AAC = 0;
  }
  switch ( v20 )
  {
    case 1:
    case 2:
    case 8:
      v43 = (const char **)::ptr;
      v44 = (const char **)((char *)::ptr + 8 * qword_84830);
      qword_84828 = (long long)::ptr;
      if ( ::ptr >= v44 )
        goto LABEL_122;
      while ( 1 )
      {
        v45 = *v43;
        if ( !strcmp(*v43, "-") )
          break;
        qword_84828 = (long long)++v43;
        if ( v43 >= v44 )
          goto LABEL_122;
      }
      if ( qword_84D50 )
        qword_84D50(v45);
      v39 = "Options '-Aru' are incompatible with '-f -'";
      goto LABEL_159;
    case 3:
      if ( !(unsigned char)sub_2BCA0(v8, v7, v18, v19, v10) )
      {
        if ( qword_84D50 )
          qword_84D50(v8);
        v39 = "Cowardly refusing to create an empty archive";
        goto LABEL_159;
      }
      if ( v70 )
      {
        if ( qword_84830 )
        {
          v47 = *(char **)::ptr;
          if ( strcmp(*(const char **)::ptr, "-") )
            sub_2A260(v47, (long long)qword_84B70);
        }
      }
LABEL_122:
      if ( qword_84820 )
      {
        stream = fopen(qword_84820, "w");
        if ( !stream )
          sub_327E0(qword_84820);
      }
      else
      {
        if ( byte_84A6A )
          v48 = stderr;
        else
          v48 = stdout;
        stream = v48;
      }
      qword_84828 = (long long)::ptr;
      if ( v71 )
        qword_84DB8 = (char *)sub_4C7A0(v71);
      if ( byte_84B84 )
      {
        dword_84B80 = sub_3E360("--backup", v72, v21);
        if ( !dword_84B80 || byte_84A6A || qword_84A60 )
          byte_84B84 = 0;
      }
      sub_107A0(v21);
      sub_2C160(ptr);
      return v74 - __readfsqword(0x28u);
    case 5:
    case 6:
    case 7:
    case 9:
      v40 = (char *)::ptr;
      v41 = qword_84830;
      qword_84828 = (long long)::ptr;
      v42 = (const char **)::ptr;
      if ( ::ptr < (char *)::ptr + 8 * qword_84830 )
      {
        do
        {
          if ( !strcmp(*v42, "-") )
          {
            sub_2C9B0((long long)"-f");
            v41 = qword_84830;
            v40 = (char *)::ptr;
          }
          v42 = (const char **)(qword_84828 + 8);
          qword_84828 = (long long)v42;
        }
        while ( v42 < (const char **)&v40[8 * v41] );
      }
      goto LABEL_122;
    default:
      goto LABEL_122;
  }
}



// Function: more_options @ 0x2f5b0
unsigned long long more_options(unsigned int a1, long long a2, long long a3)
{
  long long v4[2]; // [rsp+0h] [rbp-38h] BYREF
  short v5; // [rsp+10h] [rbp-28h]
  char v6; // [rsp+12h] [rbp-26h]
  long long v7; // [rsp+18h] [rbp-20h]
  long long v8; // [rsp+20h] [rbp-18h]
  unsigned long long v9; // [rsp+28h] [rbp-10h]

  v9 = __readfsqword(0x28u);
  v4[0] = a3;
  v5 = 0;
  v4[1] = 0;
  v6 = 0;
  v7 = 0;
  v8 = 0;
  argp_parse(&off_81240, a1, a2, 42, 0, v4);
  return v9 - __readfsqword(0x28u);
}



// Function: tar_stat_init @ 0x2f630
long long tar_stat_init(long long *a1)
{
  int v1; // ecx
  unsigned long long v2; // rdi

  *a1 = 0;
  v1 = (int)a1;
  v2 = (unsigned long long)(a1 + 1);
  *(long long *)(v2 + 424) = 0;
  memset((void *)(v2 & 0xFFFFFFFFFFFFFFF8LL), 0, 8 * ((v1 - (v2 & 0xFFFFFFF8) + 440) >> 3));
  return 0;
}



// Function: tar_stat_close @ 0x2f670
long long tar_stat_close(long long a1)
{
  DIR *v2; // rdi
  int v3; // eax
  unsigned int v4; // r8d
  int v6; // edi

  v2 = *(DIR **)(a1 + 416);
  if ( v2 )
  {
    v3 = closedir(v2);
  }
  else
  {
    v6 = *(int *)(a1 + 424);
    if ( v6 <= 0 )
    {
      *(int *)(a1 + 424) = 0;
      return 1;
    }
    v3 = close(v6);
  }
  *(int *)(a1 + 424) = 0;
  v4 = 1;
  *(long long *)(a1 + 416) = 0;
  if ( v3 )
  {
    sub_24C50(*(long long *)a1);
    return 0;
  }
  return v4;
}



// Function: tar_stat_destroy @ 0x2f6f0
long long tar_stat_destroy(long long a1)
{
  sub_2F670(a1);
  sub_1C600(*(void ***)(a1 + 352), *(long long *)(a1 + 344));
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
  sub_1CCA0(a1 + 360);
  sub_16AB0(a1);
  *(long long *)a1 = 0;
  *(long long *)(a1 + 432) = 0;
  memset(
    (void *)((a1 + 8) & 0xFFFFFFFFFFFFFFF8LL),
    0,
    8LL * (((unsigned int)a1 - (((int)a1 + 8) & 0xFFFFFFF8) + 440) >> 3));
  return 0;
}



// Function: tar_timespec_cmp @ 0x2f7b0
long long tar_timespec_cmp(long long a1, long long a2, long long a3, long long a4)
{
  if ( dword_84400 != 4 )
  {
    a2 = 0;
    a4 = 0;
  }
  return sub_4BE80(a1, a2, a3, a4);
}



// Function: set_exit_status @ 0x2f7d0
void set_exit_status(int a1)
{
  if ( dword_84D58 < a1 )
    dword_84D58 = a1;
}



// Function: parse_xform_flags @ 0x2f7f0
long long parse_xform_flags(int *a1, int a2)
{
  long long result; // rax

  switch ( a2 )
  {
    case 'H':
      *a1 &= ~2u;
      result = 0;
      break;
    case 'R':
      *a1 &= ~1u;
      result = 0;
      break;
    case 'S':
      *a1 &= ~4u;
      result = 0;
      break;
    case 'h':
      *a1 |= 2u;
      result = 0;
      break;
    case 'r':
      *a1 |= 1u;
      result = 0;
      break;
    case 's':
      *a1 |= 4u;
      result = 0;
      break;
    default:
      result = 1;
      break;
  }
  return result;
}



// Function: add_segment @ 0x2f880
long long *add_segment(long long *a1)
{
  long long *result; // rax
  long long *v2; // rdx

  result = (long long *)sub_4C5B0(32);
  v2 = (long long *)a1[12];
  *result = 0;
  if ( v2 )
  {
    *v2 = result;
    ++a1[13];
  }
  else
  {
    ++a1[13];
    a1[11] = result;
  }
  a1[12] = result;
  return result;
}



// Function: add_case_ctl_segment @ 0x2f8c0
long long *add_case_ctl_segment(long long *a1, int a2)
{
  long long *result; // rax

  result = sub_2F880(a1);
  *((int *)result + 2) = 2;
  *((int *)result + 4) = a2;
  return result;
}



// Function: add_backref_segment @ 0x2f8e0
long long *add_backref_segment(long long *a1, long long a2)
{
  long long *result; // rax

  result = sub_2F880(a1);
  *((int *)result + 2) = 1;
  result[2] = a2;
  return result;
}



// Function: add_char_segment @ 0x2f900
long long add_char_segment(long long *a1, char a2)
{
  long long *v2; // rax
  long long *v3; // rbx
  char *v4; // rax
  long long result; // rax

  v2 = sub_2F880(a1);
  *((int *)v2 + 2) = 0;
  v3 = v2;
  v4 = (char *)sub_4C5B0(2);
  v3[2] = v4;
  *v4 = a2;
  result = v3[2];
  *(char *)(result + 1) = 0;
  v3[3] = 1;
  return result;
}



// Function: new_transform @ 0x2f940
long long new_transform()
{
  long long result; // rax

  result = sub_4C760(112);
  if ( qword_84CC0 )
    *(long long *)qword_84CC0 = result;
  else
    qword_84CC8 = result;
  qword_84CC0 = result;
  return result;
}



// Function: add_literal_segment @ 0x2f990
void add_literal_segment(long long *a1, char *a2, char *a3)
{
  size_t v3; // rbx
  long long *v4; // rax
  long long *v5; // r12
  void *v6; // rax

  v3 = a3 - a2;
  if ( a3 != a2 )
  {
    v4 = sub_2F880(a1);
    *((int *)v4 + 2) = 0;
    v5 = v4;
    v6 = (void *)sub_4C5B0(v3 + 1);
    v5[2] = v6;
    memcpy(v6, a2, v3);
    *(char *)(v5[2] + v3) = 0;
    v5[3] = v3;
  }
}



// Function: parse_transform_expr @ 0x2f9f0
char *parse_transform_expr(char *s1)
{
  long long v2; // rax
  int v3; // esi
  char *v4; // r12
  int *v5; // rcx
  int *v6; // rdi
  long long v8; // rbp
  int v9; // r8d
  long long v10; // r12
  char v11; // al
  int v12; // r14d
  long long v13; // rdx
  int v14; // r15d
  char v15; // bl
  int v16; // eax
  char *v17; // rdi
  int v18; // esi
  char *v19; // rax
  char *v20; // rax
  char *v21; // r13
  unsigned int v22; // eax
  int v23; // r15d
  char *v24; // r12
  char *v25; // rsi
  char *v26; // rdx
  char v27; // al
  char *v28; // rdi
  int v29; // eax
  unsigned long long v30; // rsi
  int v31; // esi
  char v32; // si
  unsigned int v33; // r12d
  const char *v34; // rsi
  char *v35; // rax
  const char *v36; // rsi
  char *v37; // rax
  long long v38; // rdi
  char *v39; // rax
  unsigned int v40; // [rsp+0h] [rbp-278h]
  char *src; // [rsp+8h] [rbp-270h]
  char *v42; // [rsp+10h] [rbp-268h]
  unsigned int v43; // [rsp+18h] [rbp-260h]
  int v44; // [rsp+18h] [rbp-260h]
  char v45; // [rsp+1Ch] [rbp-25Ch]
  char *v46; // [rsp+20h] [rbp-258h] BYREF
  char *endptr; // [rsp+28h] [rbp-250h] BYREF
  char v48[2]; // [rsp+30h] [rbp-248h] BYREF
  char v49[518]; // [rsp+32h] [rbp-246h] BYREF
  unsigned long long v50; // [rsp+238h] [rbp-40h]

  v50 = __readfsqword(0x28u);
  v2 = sub_2F940();
  if ( *s1 != 115 )
  {
    if ( !strncmp(s1, "flags=", 6u) )
    {
      v3 = s1[6];
      v4 = s1 + 6;
      dword_83B60 = 0;
      if ( (char)v3 )
      {
        v5 = &dword_83B60;
        while ( (char)v3 != 59 )
        {
          v6 = v5;
          if ( (unsigned int)sub_2F7F0(v5, v3) )
          {
            if ( qword_84D50 )
              qword_84D50(v6);
            v33 = *v4;
            v34 = "Unknown transform flag: %c";
            goto LABEL_82;
          }
          v3 = *++v4;
          if ( !(char)v3 )
            return v4;
        }
        ++v4;
      }
      return v4;
    }
LABEL_86:
    if ( qword_84D50 )
      qword_84D50(s1);
    v36 = "Invalid transform expression";
    goto LABEL_89;
  }
  v8 = v2;
  v45 = s1[1];
  if ( !v45 )
    goto LABEL_86;
  v9 = 2;
  v10 = 2;
  src = s1 + 2;
  v11 = s1[2];
  if ( !v11 )
    goto LABEL_86;
  while ( 1 )
  {
    v12 = v9 + 1;
    if ( s1[1] == v11 )
      break;
    if ( v11 == 92 )
      v12 -= (s1[v10 + 1] == 0) - 1;
    v10 = v12;
    v11 = s1[v12];
    v9 = v12;
    if ( !v11 )
      goto LABEL_86;
  }
  v13 = v12;
  v14 = v9 + 1;
  v15 = s1[v12];
  v42 = &s1[v12];
  if ( !v15 )
    goto LABEL_86;
  while ( v15 != v11 )
  {
    if ( v15 == 92 )
      v14 -= (s1[v13 + 1] == 0) - 1;
    v13 = ++v14;
    v15 = s1[v14];
    if ( !v15 )
      goto LABEL_86;
  }
  v16 = dword_83B60;
  v17 = &s1[v13 + 1];
  *(int *)(v8 + 8) = 0;
  endptr = &s1[v13 + 1];
  *(int *)(v8 + 12) = v16;
  v18 = *v17;
  v40 = 0;
  if ( *v17 && (char)v18 != 59 )
  {
    while ( 1 )
    {
      if ( (char)v18 == 105 )
      {
        v19 = endptr;
        v40 |= 2u;
        v17 = endptr + 1;
        goto LABEL_29;
      }
      if ( (char)v18 > 105 )
        break;
      if ( (char)v18 <= 57 )
      {
        if ( (char)v18 <= 47 )
          goto LABEL_27;
        v44 = v9;
        v29 = strtoul(v17, &endptr, 0);
        v17 = endptr;
        v9 = v44;
        *(int *)(v8 + 16) = v29;
        v19 = v17 - 1;
        goto LABEL_29;
      }
      if ( (char)v18 != 103 )
        goto LABEL_27;
      v20 = endptr;
      *(int *)(v8 + 8) = 1;
      v17 = v20 + 1;
      endptr = v20 + 1;
      v18 = v20[1];
      if ( !v20[1] )
        goto LABEL_36;
LABEL_30:
      if ( (char)v18 == 59 )
        goto LABEL_77;
    }
    if ( (char)v18 == 120 )
    {
      v19 = endptr;
      v40 |= 1u;
      v17 = endptr + 1;
    }
    else
    {
LABEL_27:
      if ( (unsigned int)sub_2F7F0((int *)(v8 + 12), v18) )
      {
        if ( qword_84D50 )
          qword_84D50(v8 + 12);
        v34 = "Unknown flag in transform expression: %c";
        v33 = *endptr;
LABEL_82:
        v35 = dcgettext(0, v34, 5);
        error(0, 0, v35, v33);
        sub_2C970(2);
      }
      v19 = endptr;
      v17 = endptr + 1;
    }
LABEL_29:
    endptr = v17;
    v18 = v19[1];
    if ( !v19[1] )
      goto LABEL_36;
    goto LABEL_30;
  }
LABEL_36:
  if ( (char)v18 == 59 )
LABEL_77:
    endptr = v17 + 1;
  v43 = v9;
  v21 = (char *)sub_4C5B0(v9 - 1);
  memcpy(v21, src, (int)(v43 - 2));
  v21[v10 - 2] = 0;
  v22 = sub_5AE70(v8 + 24, v21, v40);
  if ( v22 )
  {
    v38 = v22;
    sub_5AF90(v22, v8 + 24, v48, 512, v43);
    if ( qword_84D50 )
      qword_84D50(v38);
    v39 = dcgettext(0, "Invalid transform expression: %s", 5);
    error(0, 0, v39, v48);
    sub_2C970(2);
  }
  if ( *v21 == 94 || v43 != 2 && v21[v10 - 3] == 36 )
    *(int *)(v8 + 8) = 0;
  v23 = v14 - v12;
  free(v21);
  v24 = (char *)sub_4C5B0(v23 + 1);
  memcpy(v24, v42, v23);
  v24[v23] = 0;
  v25 = v24;
  v26 = v24;
  v27 = *v24;
  v46 = v24;
  if ( v27 )
  {
    while ( 1 )
    {
      while ( v27 != 92 )
      {
        if ( v27 == 38 )
        {
          sub_2F990((long long *)v8, v25, v26);
          sub_2F8E0((long long *)v8, 0);
          v25 = v46 + 1;
          v46 = v25;
          v26 = v25;
        }
        else
        {
          v46 = ++v26;
        }
        v27 = *v26;
        if ( !*v26 )
          goto LABEL_58;
      }
      sub_2F990((long long *)v8, v25, v26);
      v28 = v46 + 1;
      v46 = v28;
      switch ( *v28 )
      {
        case '&':
          v32 = 38;
          goto LABEL_62;
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
          v30 = strtoul(v28, &v46, 10);
          if ( *(long long *)(v8 + 72) < v30 )
          {
            if ( qword_84D50 )
              qword_84D50(v28);
            v36 = "Invalid transform replacement: back reference out of range";
LABEL_89:
            v37 = dcgettext(0, v36, 5);
            error(0, 0, v37);
            sub_2C970(2);
          }
          sub_2F8E0((long long *)v8, v30);
          v25 = v46;
LABEL_57:
          v26 = v25;
          v27 = *v25;
          if ( !*v25 )
            goto LABEL_58;
          break;
        case 'E':
          v31 = 0;
          goto LABEL_60;
        case 'L':
          v31 = 4;
          goto LABEL_60;
        case 'U':
          v31 = 3;
          goto LABEL_60;
        case '\\':
          v32 = 92;
          goto LABEL_62;
        case 'a':
          v32 = 7;
          goto LABEL_62;
        case 'b':
          v32 = 8;
          goto LABEL_62;
        case 'f':
          v32 = 12;
          goto LABEL_62;
        case 'l':
          v31 = 2;
          goto LABEL_60;
        case 'n':
          v32 = 10;
          goto LABEL_62;
        case 'r':
          v32 = 13;
          goto LABEL_62;
        case 't':
          v32 = 9;
          goto LABEL_62;
        case 'u':
          v31 = 1;
LABEL_60:
          sub_2F8C0((long long *)v8, v31);
          v25 = ++v46;
          goto LABEL_57;
        case 'v':
          v32 = 11;
LABEL_62:
          sub_2F900((long long *)v8, v32);
          v25 = ++v46;
          goto LABEL_57;
        default:
          if ( *v28 == v15 )
          {
            sub_2F900((long long *)v8, v45);
          }
          else
          {
            v48[1] = *v28;
            v48[0] = 92;
            sub_2F990((long long *)v8, v48, v49);
          }
          v25 = ++v46;
          goto LABEL_57;
      }
    }
  }
LABEL_58:
  sub_2F990((long long *)v8, v25, v26);
  free(v24);
  return endptr;
}



// Function: run_case_conv @ 0x30070
unsigned char *run_case_conv(unsigned int a1, const void *a2, size_t a3)
{
  unsigned char *v4; // r12
  unsigned char *v6; // rbx
  const int_t **v7; // rax
  unsigned char *v8; // rdx
  long long v9; // rsi
  unsigned char *v10; // rbx
  const int_t **v11; // rax
  unsigned char *v12; // rdx
  long long v13; // rsi

  v4 = (unsigned char *)qword_84CB8;
  if ( qword_84C40 < a3 )
  {
    qword_84C40 = a3;
    qword_84CB8 = (void *)sub_4C610(qword_84CB8, a3);
    v4 = (unsigned char *)qword_84CB8;
  }
  memcpy(v4, a2, a3);
  if ( a1 == 3 )
  {
    v10 = &v4[a3];
    if ( v10 <= v4 )
      return v4;
    v11 = __ctype_toupper_loc();
    v12 = v4;
    do
    {
      v13 = *v12++;
      *(v12 - 1) = (*v11)[v13];
    }
    while ( v12 != v10 );
    return v4;
  }
  else if ( a1 > 3 )
  {
    if ( a1 != 4 )
      return v4;
    v6 = &v4[a3];
    if ( v6 <= v4 )
      return v4;
    v7 = __ctype_tolower_loc();
    v8 = v4;
    do
    {
      v9 = *v8++;
      *(v8 - 1) = (*v7)[v9];
    }
    while ( v8 != v6 );
    return v4;
  }
  else
  {
    if ( a1 != 1 )
    {
      if ( a1 == 2 )
        *v4 = (*__ctype_tolower_loc())[*v4];
      return v4;
    }
    *v4 = (*__ctype_toupper_loc())[*v4];
    return v4;
  }
}



// Function: _single_transform_name_to_obstack @ 0x301b0
void _single_transform_name_to_obstack(long long a1, char *a2)
{
  char *v3; // rbp
  size_t *v4; // r14
  unsigned int v5; // r12d
  int i; // eax
  size_t v7; // r15
  size_t v8; // rbx
  char *v9; // rdi
  unsigned long long v10; // rax
  char *v11; // rdi
  const void *v12; // rsi
  size_t v13; // rax
  char *v14; // rdi
  size_t v15; // rax
  char *v16; // rdi
  size_t v17; // rbx
  char *next_free; // rax
  bool v19; // zf
  long long *v20; // r15
  int v21; // eax
  unsigned int v22; // eax
  size_t *v23; // rdx
  size_t v24; // rax
  size_t v25; // rdx
  size_t v26; // rdx
  unsigned char *v27; // r8
  char *v28; // rdi
  unsigned char *v29; // rax
  unsigned long long v30; // [rsp+0h] [rbp-68h]
  size_t v31; // [rsp+8h] [rbp-60h]
  size_t v32; // [rsp+8h] [rbp-60h]
  size_t v33; // [rsp+8h] [rbp-60h]
  long long v34; // [rsp+10h] [rbp-58h]
  unsigned int v35; // [rsp+24h] [rbp-44h]
  unsigned char *v36; // [rsp+28h] [rbp-40h]

  v3 = a2;
  v4 = (size_t *)sub_4C5B0(16 * (*(long long *)(a1 + 72) + 1LL));
  if ( !*a2 )
  {
    next_free = obstack.next_free;
LABEL_53:
    if ( obstack.chunk_limit != next_free )
      goto LABEL_54;
    goto LABEL_22;
  }
  v35 = 0;
  v5 = 0;
  v34 = a1 + 24;
  v30 = 0;
  for ( i = sub_5B070(a1 + 24, a2, *(long long *)(a1 + 72) + 1LL, v4, 0);
        !i;
        i = sub_5B070(v34, v3, *(long long *)(a1 + 72) + 1LL, v4, 0) )
  {
    v7 = *v4;
    v8 = v4[1];
    if ( *v4 )
    {
      v9 = obstack.next_free;
      if ( v7 > obstack.chunk_limit - obstack.next_free )
      {
        obstack_newchunk(&obstack, *v4);
        v9 = obstack.next_free;
      }
      memcpy(v9, v3, v7);
      obstack.next_free += v7;
    }
    v10 = *(unsigned int *)(a1 + 16);
    ++v30;
    if ( !(int)v10 || v10 <= v30 )
    {
      v20 = *(long long **)(a1 + 88);
      if ( !v20 )
        goto LABEL_18;
      while ( 2 )
      {
        v21 = *((int *)v20 + 2);
        if ( v21 == 1 )
        {
          v23 = &v4[2 * v20[2]];
          v24 = *v23;
          if ( *v23 == -1 )
            goto LABEL_26;
          v25 = v23[1];
          if ( v25 == -1 )
            goto LABEL_26;
          v26 = v25 - v24;
          v27 = &v3[v24];
          if ( v5 )
          {
            v33 = v26;
            v29 = sub_30070(v5, &v3[v24], v26);
            v26 = v33;
            v27 = v29;
            if ( v5 - 1 <= 1 )
            {
              v5 = v35;
              v35 = 0;
            }
          }
          v28 = obstack.next_free;
          if ( v26 > obstack.chunk_limit - obstack.next_free )
            goto LABEL_40;
        }
        else
        {
          if ( v21 == 2 )
          {
            v22 = *((int *)v20 + 4);
            if ( v22 > 2 )
            {
              if ( v22 - 3 <= 1 )
                v5 = *((int *)v20 + 4);
            }
            else if ( v22 )
            {
              if ( v35 && v35 - 3 > 1 )
              {
                v5 = *((int *)v20 + 4);
              }
              else
              {
                v35 = v5;
                v5 = *((int *)v20 + 4);
              }
            }
            else
            {
              v5 = 0;
            }
LABEL_26:
            v20 = (long long *)*v20;
            if ( !v20 )
              goto LABEL_18;
            continue;
          }
          if ( v21 )
            goto LABEL_26;
          if ( v5 )
          {
            v27 = sub_30070(v5, (const void *)v20[2], v20[3]);
            if ( v5 - 1 <= 1 )
            {
              v5 = v35;
              v35 = 0;
            }
          }
          else
          {
            v27 = (unsigned char *)v20[2];
          }
          v28 = obstack.next_free;
          v26 = v20[3];
          if ( v26 > obstack.chunk_limit - obstack.next_free )
          {
LABEL_40:
            v36 = v27;
            v32 = v26;
            obstack_newchunk(&obstack, v26);
            v27 = v36;
            v26 = v32;
            v28 = obstack.next_free;
          }
        }
        break;
      }
      v31 = v26;
      memcpy(v28, v27, v26);
      obstack.next_free += v31;
      goto LABEL_26;
    }
    v11 = obstack.next_free;
    if ( v8 > obstack.chunk_limit - obstack.next_free )
    {
      obstack_newchunk(&obstack, v8);
      v11 = obstack.next_free;
    }
    v12 = v3;
    v3 += v8;
    memcpy(v11, v12, v8);
    obstack.next_free += v8;
LABEL_12:
    if ( !*v3 )
    {
      next_free = obstack.next_free;
      goto LABEL_53;
    }
  }
  v13 = strlen(v3);
  v14 = obstack.next_free;
  v8 = v13;
  if ( v13 > obstack.chunk_limit - obstack.next_free )
  {
    obstack_newchunk(&obstack, v13);
    v14 = obstack.next_free;
  }
  memcpy(v14, v3, v8);
  obstack.next_free += v8;
LABEL_18:
  v3 += v8;
  if ( *(int *)(a1 + 8) )
    goto LABEL_12;
  v15 = strlen(v3);
  v16 = obstack.next_free;
  v17 = v15;
  if ( v15 > obstack.chunk_limit - obstack.next_free )
  {
    obstack_newchunk(&obstack, v15);
    v16 = obstack.next_free;
  }
  memcpy(v16, v3, v17);
  next_free = &obstack.next_free[v17];
  v19 = obstack.chunk_limit == &obstack.next_free[v17];
  obstack.next_free += v17;
  if ( v19 )
  {
LABEL_22:
    obstack_newchunk(&obstack, 1);
    next_free = obstack.next_free;
  }
LABEL_54:
  obstack.next_free = next_free + 1;
  *next_free = 0;
  free(v4);
}



// Function: _transform_name_to_obstack @ 0x30600
long long _transform_name_to_obstack(int a1, char *object_base, char **a3)
{
  long long v5; // rbx
  unsigned int i; // r8d
  char *chunk_limit; // rdx

  if ( !byte_84C48 )
  {
    obstack_begin(&obstack, 0, 0, sub_4C5B0, &free);
    byte_84C48 = 1;
  }
  v5 = qword_84CC8;
  for ( i = 0; v5; obstack.object_base = chunk_limit )
  {
    while ( (*(int *)(v5 + 12) & a1) == 0 )
    {
      v5 = *(long long *)v5;
      if ( !v5 )
        goto LABEL_12;
    }
    sub_301B0(v5, object_base);
    object_base = obstack.object_base;
    if ( obstack.next_free == obstack.object_base )
      *((char *)&obstack + 80) |= 2u;
    i = 1;
    chunk_limit = obstack.chunk_limit;
    if ( (~*(long long *)&obstack.alignment_mask & (long long)&obstack.next_free[*(long long *)&obstack.alignment_mask])
       - (unsigned long long)obstack.chunk <= obstack.chunk_limit - (char *)obstack.chunk )
      chunk_limit = (char *)(~*(long long *)&obstack.alignment_mask
                           & (long long)&obstack.next_free[*(long long *)&obstack.alignment_mask]);
    obstack.next_free = chunk_limit;
    v5 = *(long long *)v5;
  }
LABEL_12:
  *a3 = object_base;
  return i;
}



// Function: set_transform_expr @ 0x30710
void set_transform_expr(char *a1)
{
  for ( ; *a1; a1 = sub_2F9F0(a1) )
    ;
}



// Function: transform_name_fp @ 0x30740
long long transform_name_fp(void **a1, int a2, long long (*a3)(void *, long long), long long a4)
{
  char *v8; // rsi
  unsigned int v9; // r12d
  struct _obstack_chunk *v10; // r13
  void *v11; // rsi
  void *v13; // r12
  void *v14; // rax
  void *v15; // rdi
  void *ptr[7]; // [rsp+0h] [rbp-38h] BYREF

  v8 = (char *)*a1;
  ptr[1] = (void *)__readfsqword(0x28u);
  v9 = sub_30600(a2, v8, (char **)ptr);
  if ( (char)v9 )
  {
    v10 = (struct _obstack_chunk *)ptr[0];
    v11 = ptr[0];
    if ( a3 )
      v11 = (void *)a3(ptr[0], a4);
    sub_23BE0(a1, v11);
    if ( obstack.chunk >= v10 || obstack.chunk_limit <= (char *)v10 )
    {
      obstack_free(&obstack, v10);
    }
    else
    {
      obstack.object_base = (char *)v10;
      obstack.next_free = (char *)v10;
    }
  }
  else if ( a3 )
  {
    v13 = ptr[0];
    *a1 = 0;
    v14 = (void *)a3(v13, a4);
    sub_23BE0(a1, v14);
    v15 = v13;
    v9 = 1;
    free(v15);
  }
  return v9;
}



// Function: transform_name @ 0x30830
long long transform_name(void **a1, int a2)
{
  return sub_30740(a1, a2, 0, 0);
}



// Function: transform_program_p @ 0x30840
bool transform_program_p()
{
  return qword_84CC8 != 0;
}



// Function: dunlink_insert @ 0x30850
long long dunlink_insert(long long *a1, long long *a2)
{
  long long result; // rax

  if ( a1 )
  {
    *a2 = *a1;
    *a1 = (long long)a2;
    result = *a2;
    if ( *a2 )
    {
LABEL_3:
      ++qword_84CD8;
      return result;
    }
  }
  else
  {
    result = qword_84CE8;
    qword_84CE8 = (long long)a2;
    *a2 = result;
    if ( result )
      goto LABEL_3;
  }
  ++qword_84CD8;
  qword_84CE0 = (long long)a2;
  return result;
}



// Function: dunlink_reclaim @ 0x308a0
void *dunlink_reclaim(void **a1)
{
  void *result; // rax

  free(a1[2]);
  result = qword_84CD0;
  qword_84CD0 = a1;
  *a1 = result;
  return result;
}



// Function: flush_deferred_unlinks @ 0x308c0
void flush_deferred_unlinks(char a1)
{
  long long *v1; // rbx
  long long *v2; // r15
  long long *v3; // rbp
  const char *v4; // r13
  int v5; // eax
  long long *v6; // rbx
  long long *v7; // rbp
  const char *v8; // r12
  int v9; // [rsp+8h] [rbp-40h]

  v1 = 0;
  v2 = (long long *)qword_84CE8;
  v9 = dword_84580;
  if ( !qword_84CE8 )
    goto LABEL_35;
  do
  {
    while ( 1 )
    {
      v3 = v2;
      v2 = (long long *)*v2;
      if ( !a1 && v3[4] >= (unsigned long long)qword_83F30 )
        goto LABEL_3;
      sub_24C80(*((int *)v3 + 2));
      if ( !*((char *)v3 + 24) )
        break;
      v4 = (const char *)v3[2];
      if ( !*((int *)v3 + 2) || *v4 && strcmp((const char *)v3[2], ".") )
      {
        if ( !unlinkat(oldfd, v4, 512) )
          goto LABEL_16;
        v5 = *__errno_location();
        if ( v5 != 17 && v5 != 39 )
        {
          if ( v5 != 2 )
            sub_32AA0(v4);
          goto LABEL_16;
        }
      }
LABEL_3:
      v1 = v3;
LABEL_4:
      if ( !v2 )
        goto LABEL_18;
    }
    if ( unlinkat(oldfd, (const char *)v3[2], 0) && *__errno_location() != 2 )
      sub_32DB0(v3[2]);
LABEL_16:
    sub_308A0((void **)v3);
    --qword_84CD8;
    if ( !v1 )
    {
      qword_84CE8 = (long long)v2;
      goto LABEL_4;
    }
    *v1 = v2;
  }
  while ( v2 );
LABEL_18:
  v6 = (long long *)qword_84CE8;
  if ( !qword_84CE8 )
  {
LABEL_35:
    qword_84CE0 = 0;
    goto LABEL_30;
  }
  if ( a1 )
  {
    do
    {
      v7 = v6;
      v6 = (long long *)*v6;
      sub_24C80(*((int *)v7 + 2));
      v8 = (const char *)v7[2];
      if ( *((int *)v7 + 2) && *((char *)v7 + 24) && (!*v8 || !strcmp((const char *)v7[2], ".")) )
      {
        v8 = (const char *)sub_24C30();
        sub_24C80(*((int *)v7 + 2) - 1);
      }
      if ( unlinkat(oldfd, v8, 512) && *__errno_location() != 2 )
        sub_32AA0(v8);
      sub_308A0((void **)v7);
      --qword_84CD8;
    }
    while ( v6 );
    qword_84CE0 = 0;
    qword_84CE8 = 0;
  }
LABEL_30:
  sub_24C80(v9);
}



// Function: dunlink_alloc @ 0x30b00
void *dunlink_alloc()
{
  void *result; // rax
  void *v1; // rdx

  result = qword_84CD0;
  if ( !qword_84CD0 )
    return (void *)sub_4C5B0(40);
  v1 = *(void **)qword_84CD0;
  *(long long *)qword_84CD0 = 0;
  qword_84CD0 = v1;
  return result;
}



// Function: finish_deferred_unlinks @ 0x30b30
void finish_deferred_unlinks()
{
  long long *v0; // rbx
  void *v1; // rdi

  sub_308C0(1);
  v0 = qword_84CD0;
  if ( qword_84CD0 )
  {
    do
    {
      v1 = v0;
      v0 = (long long *)*v0;
      free(v1);
      qword_84CD0 = v0;
    }
    while ( v0 );
  }
}



// Function: queue_deferred_unlink @ 0x30b70
long long queue_deferred_unlink(void *src, char a2)
{
  void *v2; // rbp
  char *v3; // rax
  long long v4; // rax
  const char *v5; // rdi
  long long v6; // rbx
  long long *v7; // r12
  const char *v8; // rdi

  if ( qword_84CE8 && *(long long *)(qword_84CE8 + 32) < (unsigned long long)qword_83F30 )
    sub_308C0(0);
  v2 = sub_30B00();
  *(long long *)v2 = 0;
  *((int *)v2 + 2) = dword_84580;
  v3 = (char *)sub_4C7A0(src);
  *((long long *)v2 + 2) = v3;
  sub_23EA0(v3);
  v4 = qword_83F30;
  *((char *)v2 + 24) = a2;
  *((long long *)v2 + 4) = v4;
  if ( !a2 )
    return sub_30850((long long *)qword_84CE0, (long long *)v2);
  v5 = (const char *)*((long long *)v2 + 2);
  if ( !*v5 )
  {
    v6 = qword_84CE8;
    if ( qword_84CE8 )
      goto LABEL_7;
    return sub_30850((long long *)qword_84CE0, (long long *)v2);
  }
  if ( strcmp(v5, ".") )
    return sub_30850((long long *)qword_84CE0, (long long *)v2);
  v6 = qword_84CE8;
  if ( !qword_84CE8 )
    return sub_30850((long long *)qword_84CE0, (long long *)v2);
LABEL_7:
  v7 = 0;
  while ( 1 )
  {
    if ( *(char *)(v6 + 24) )
    {
      v8 = *(const char **)(v6 + 16);
      if ( (!*v8 || !strcmp(v8, ".")) && *(int *)(v6 + 8) < *((int *)v2 + 2) )
        break;
    }
    v7 = (long long *)v6;
    if ( !*(long long *)v6 )
      return sub_30850((long long *)qword_84CE0, (long long *)v2);
    v6 = *(long long *)v6;
  }
  return sub_30850(v7, (long long *)v2);
}



// Function: append_file @ 0x30ca0
unsigned long long append_file(char *file)
{
  int v1; // eax
  unsigned int v2; // ebp
  __off_t st_size; // r12
  long long v4; // rax
  char *v5; // r14
  char *v6; // r15
  unsigned int v7; // eax
  char *v8; // rdx
  long long v10; // rbx
  long long v11; // r13
  char *v12; // rax
  struct stat v13; // [rsp+0h] [rbp-E8h] BYREF
  char v14[24]; // [rsp+90h] [rbp-58h] BYREF
  unsigned long long v15; // [rsp+A8h] [rbp-40h]

  v15 = __readfsqword(0x28u);
  v1 = openat(oldfd, file, 0);
  if ( v1 < 0 )
    return sub_327C0(file);
  v2 = v1;
  if ( fstat(v1, &v13) )
  {
    sub_32D30(file);
  }
  else
  {
    st_size = v13.st_size;
    while ( st_size > 0 )
    {
      v5 = (char *)sub_E130();
      v6 = sub_D2B0((long long)v5);
      if ( st_size < (unsigned long long)v6 )
      {
        v6 = (char *)st_size;
        if ( (st_size & 0x1FF) != 0 )
        {
          v7 = 512 - (st_size & 0x1FF);
          v8 = &v5[st_size];
          if ( v7 >= 8 )
          {
            *(long long *)v8 = 0;
            *(long long *)&v8[v7 - 8] = 0;
            memset(
              (void *)((unsigned long long)(v8 + 8) & 0xFFFFFFFFFFFFFFF8LL),
              0,
              8LL * ((v7 + (int)v8 - (((int)v8 + 8) & 0xFFFFFFF8)) >> 3));
          }
          else if ( (-(char)st_size & 4) != 0 )
          {
            *(int *)v8 = 0;
            *(int *)&v8[v7 - 4] = 0;
          }
          else if ( v7 )
          {
            *v8 = 0;
            if ( (-(char)st_size & 2) != 0 )
              *(short *)&v8[v7 - 2] = 0;
          }
          v6 = (char *)st_size;
        }
      }
      v4 = sub_4AF10(v2, v5, (size_t)v6);
      if ( v4 == -1 )
        sub_329D0(file, v13.st_size - st_size, v6);
      if ( !v4 )
      {
        if ( qword_84D50 )
          qword_84D50(v2);
        v10 = sub_43090(st_size, v14);
        v11 = sub_4AA70(file);
        v12 = dcngettext(0, "%s: File shrank by %s byte", "%s: File shrank by %s bytes", st_size, 5);
        error(0, 0, v12, v11, v10);
        sub_163D0();
      }
      st_size -= v4;
      sub_D1F0((unsigned long long)&v5[(v4 - 1) & 0xFFFFFFFFFFFFFE00LL]);
    }
  }
  if ( close(v2) )
    sub_32680(file);
  return v15 - __readfsqword(0x28u);
}



// Function: update_archive @ 0x30ef0
void update_archive(double a1)
{
  unsigned int v1; // r12d
  unsigned int v2; // r15d
  long long i; // rax
  char *v4; // rbp
  long long v5; // rdi
  char *v6; // rax
  char *v7; // rax
  long long v8; // rax
  long long *v9; // r12
  long long v10; // rax
  long long v11; // rdx
  void **v12; // r11
  const char *v13; // r12
  int v14; // r15d
  char *v15; // rax
  long long *v16; // [rsp+10h] [rbp-F8h]
  long long *v17; // [rsp+18h] [rbp-F0h]
  char *s; // [rsp+20h] [rbp-E8h]
  unsigned int v19; // [rsp+2Ch] [rbp-DCh]
  struct stat v20; // [rsp+30h] [rbp-D8h] BYREF
  unsigned long long v21; // [rsp+C8h] [rbp-40h]

  v1 = 0;
  v21 = __readfsqword(0x28u);
  sub_26D90();
  sub_FB70(2, a1);
  sub_1C560(2);
  while ( 1 )
  {
    v2 = sub_219E0((unsigned char **)&qword_84408, (long long)&qword_84840, 0);
    switch ( v2 )
    {
      case 0u:
      case 2u:
        goto LABEL_40;
      case 1u:
        sub_215C0(qword_84408, (long long)&qword_84840, &dword_84400, 0);
        sub_21440(*(char *)(qword_84408 + 156), (long long)&qword_84840);
        dword_84BA4 = dword_84400;
        if ( dword_84BA8 == 8 )
        {
          v8 = sub_27AA0(path);
          v9 = (long long *)v8;
          if ( v8 )
          {
            sub_24C80(*(int *)(v8 + 40));
            if ( !sub_246F0(path, &v20) )
            {
              if ( (v20.st_mode & 0xF000) == 0x4000 )
              {
                s = (char *)sub_253C0(v9[2], 1);
                if ( s )
                {
                  v12 = (void **)sub_25040((char *)v9[2]);
                  if ( *s )
                  {
                    v19 = v2;
                    v17 = (long long *)v12;
                    v16 = v9;
                    v13 = s;
                    do
                    {
                      v14 = *((int *)v16 + 10);
                      v15 = (char *)sub_250D0(v17, v13);
                      sub_26CF0(v15, v14, 0, 0);
                      v13 += strlen(v13) + 1;
                    }
                    while ( *v13 );
                    v2 = v19;
                    v9 = v16;
                    v12 = (void **)v17;
                  }
                  sub_250B0(v12);
                  free(s);
                  sub_27150(v9);
                }
              }
              else
              {
                v10 = sub_4BD20(&v20);
                if ( (int)sub_2F7B0(v10, v11, (long long)file, (long long)*(&file + 1)) <= 0 )
                  sub_27150(v9);
              }
            }
          }
        }
        sub_22BD0();
        sub_2F6F0((long long)&qword_84840);
        goto LABEL_20;
      case 3u:
        src = (void *)qword_84408;
        sub_2F6F0((long long)&qword_84840);
        goto LABEL_4;
      case 4u:
        sub_2F6F0((long long)&qword_84840);
LABEL_4:
        sub_D1A0();
        byte_84CF8 = 1;
        qword_84CF0 = (long long)src;
LABEL_5:
        for ( i = sub_27B00(); i; i = sub_27B00() )
        {
          v4 = *(char **)(i + 16);
          if ( (unsigned char)sub_16B00(v4, 0) )
            goto LABEL_5;
          if ( byte_84B40 )
          {
            if ( !(unsigned int)sub_2CA20("add", (long long)v4) )
              goto LABEL_5;
            if ( dword_84BA8 == 2 )
            {
LABEL_28:
              sub_30CA0(v4);
              goto LABEL_5;
            }
          }
          else if ( dword_84BA8 == 2 )
          {
            goto LABEL_28;
          }
          sub_14330(0, (long long)v4, (long long)v4, a1);
        }
        sub_12C40();
        sub_F900(a1);
        sub_30B30();
        sub_27430();
        return;
      case 5u:
        v5 = qword_84408;
        sub_D1F0(qword_84408);
        if ( v1 == 1 )
          goto LABEL_17;
        if ( v1 <= 1 )
        {
          if ( qword_84D50 )
            qword_84D50(v5);
          v6 = dcgettext(0, "This does not look like a tar archive", 5);
          v5 = 0;
          error(0, 0, v6);
LABEL_17:
          if ( qword_84D50 )
            qword_84D50(v5);
          v7 = dcgettext(0, "Skipping to next header", 5);
          error(0, 0, v7);
          dword_84D58 = 2;
          sub_2F6F0((long long)&qword_84840);
          goto LABEL_20;
        }
        if ( v1 == 3 )
          goto LABEL_17;
        if ( v1 == 4 )
LABEL_40:
          abort();
LABEL_25:
        sub_2F6F0((long long)&qword_84840);
LABEL_20:
        v1 = v2;
        break;
      default:
        goto LABEL_25;
    }
  }
}



// Function: utf8_init @ 0x312d0
iconv_t utf8_init(unsigned char a1)
{
  iconv_t result; // rax
  const char *v2; // rax

  result = (iconv_t)qword_83B70[a1];
  if ( result == (iconv_t)-1LL )
  {
    v2 = (const char *)sub_430F0();
    if ( a1 )
    {
      result = iconv_open("UTF-8", v2);
      qword_83B78 = (long long)result;
    }
    else
    {
      result = iconv_open(v2, "UTF-8");
      qword_83B70[0] = (long long)result;
    }
  }
  return result;
}



// Function: utf8_convert @ 0x31340
long long utf8_convert(unsigned char a1, char *a2, char **a3)
{
  iconv_t v4; // rax
  void *v5; // r12
  unsigned int v6; // r14d
  char *v7; // r13
  char *inbuf; // [rsp+8h] [rbp-50h] BYREF
  char *outbuf; // [rsp+10h] [rbp-48h] BYREF
  size_t inbytesleft; // [rsp+18h] [rbp-40h] BYREF
  size_t outbytesleft[7]; // [rsp+20h] [rbp-38h] BYREF

  outbytesleft[1] = __readfsqword(0x28u);
  v4 = sub_312D0(a1);
  if ( v4 )
  {
    v5 = v4;
    v6 = 0;
    if ( v4 != (iconv_t)-1LL )
    {
      inbytesleft = strlen(a2) + 1;
      outbytesleft[0] = 16 * inbytesleft + 1;
      outbuf = (char *)sub_4C5B0(outbytesleft[0]);
      v7 = outbuf;
      inbuf = a2;
      if ( iconv(v5, &inbuf, &inbytesleft, &outbuf, outbytesleft) )
      {
        free(v7);
      }
      else
      {
        v6 = 1;
        *outbuf = 0;
        *a3 = v7;
      }
    }
  }
  else
  {
    v6 = 1;
    *a3 = (char *)sub_4C7A0(a2);
  }
  return v6;
}



// Function: string_ascii_p @ 0x31440
long long string_ascii_p(char *a1)
{
  while ( 1 )
  {
    if ( !*a1 )
      return 1;
    if ( *a1 < 0 )
      break;
    ++a1;
  }
  return 0;
}



// Function: set_warning_option @ 0x31470
int set_warning_option(const char *s1)
{
  int result; // eax
  size_t v2; // rax
  void (*v3)(); // rbx

  result = strcmp(s1, "none");
  if ( !result )
    goto LABEL_5;
  v2 = strlen(s1);
  v3 = off_83DD8;
  if ( v2 > 2 && !memcmp(s1, "no-", 3u) )
  {
    result = dword_83B80 & ~dword_83BA0[sub_39670("--warning", s1 + 3, &off_7FA20, dword_83BA0, 4, v3)];
LABEL_5:
    dword_83B80 = result;
    return result;
  }
  result = dword_83BA0[sub_39670("--warning", s1, &off_7FA20, dword_83BA0, 4, v3)] | dword_83B80;
  dword_83B80 = result;
  return result;
}



// Function: mask_map_realloc @ 0x31550
long long mask_map_realloc(long long *a1)
{
  long long result; // rax

  result = a1[1];
  if ( a1[2] == result )
  {
    if ( !result )
      a1[1] = 4;
    result = sub_4C680(*a1, a1 + 1, 8);
    *a1 = result;
  }
  return result;
}



// Function: xattrs_kw_included @ 0x31600
bool xattrs_kw_included(char *name, bool a2)
{
  long long v2; // rbx
  bool result; // al

  if ( qword_84D28 )
  {
    if ( unk_84D30 )
    {
      v2 = 0;
      while ( fnmatch(*(const char **)(qword_84D20 + 8 * v2), name, 0) )
      {
        if ( unk_84D30 <= (unsigned long long)++v2 )
          return 0;
      }
      return 1;
    }
    else
    {
      return 0;
    }
  }
  else
  {
    result = a2;
    if ( !a2 )
      return strncmp(name, "user.", 5u) == 0;
  }
  return result;
}



// Function: xattrs_masked_out @ 0x31730
long long xattrs_masked_out(char *a1, bool a2)
{
  if ( sub_31600(a1, a2) )
    return sub_31710(a1);
  else
    return 1;
}



// Function: xattrs_acls_get @ 0x319f0
void xattrs_acls_get(long long a1)
{
  char *v1; // rax

  if ( dword_84A9C > 0 )
  {
    if ( dword_84D14 )
    {
      dword_84D14 = 1;
    }
    else
    {
      if ( qword_84D50 )
        qword_84D50(a1);
      v1 = dcgettext(0, "POSIX ACL support is not available", 5);
      error(0, 0, v1);
      dword_84D14 = 1;
    }
  }
}



// Function: xattrs_acls_set @ 0x31a70
void xattrs_acls_set(long long a1, long long a2, char a3)
{
  char *v3; // rax

  if ( dword_84A9C > 0 && a3 != 50 )
  {
    if ( dword_84D10 )
    {
      dword_84D10 = 1;
    }
    else
    {
      if ( qword_84D50 )
        qword_84D50(a1);
      v3 = dcgettext(0, "POSIX ACL support is not available", 5);
      error(0, 0, v3);
      dword_84D10 = 1;
    }
  }
}



// Function: xattrs_mask_add @ 0x31af0
long long xattrs_mask_add(long long a1, char a2)
{
  long long *v2; // rbx
  long long result; // rax
  long long v4; // rdx

  v2 = &qword_84D20;
  if ( !a2 )
    v2 = &qword_84D38;
  sub_31550(v2);
  result = v2[2];
  v4 = *v2;
  v2[2] = result + 1;
  *(long long *)(v4 + 8 * result) = a1;
  return result;
}



// Function: xattrs_clear_setup @ 0x31b40
long long xattrs_clear_setup()
{
  sub_319D0(qword_84D20, qword_84D28);
  return sub_319D0(qword_84D38, qword_84D40);
}



// Function: xattrs_xattrs_get @ 0x31b80
ssize_t xattrs_xattrs_get(int fd, char *path, long long a3, int a4)
{
  ssize_t result; // rax
  char *v5; // r12
  long long v7; // rbp
  const char *v8; // rax
  size_t v9; // rax
  void *v10; // rdx
  ssize_t v11; // rcx
  int v12; // eax
  const char *v13; // rdi
  ssize_t v14; // [rsp+0h] [rbp-58h]
  long long v15; // [rsp+8h] [rbp-50h]
  const char *v17; // [rsp+18h] [rbp-40h]

  result = (unsigned int)dword_84A98;
  if ( dword_84A98 <= 0 )
    return result;
  v5 = list;
  if ( !list )
  {
    list = (char *)sub_4C680(0, &size, 1);
    v5 = list;
  }
  while ( a4 )
  {
    result = flistxattr(a4, v5, size);
    v7 = result;
    if ( result != -1 )
      goto LABEL_10;
LABEL_6:
    if ( *__errno_location() != 34 )
    {
      v13 = "llistxattrat";
      if ( a4 )
        v13 = "flistxattr";
      return sub_324F0(v13, path);
    }
    list = (char *)sub_4C680(list, &size, 1);
    v5 = list;
  }
  result = sub_38F90(fd, path, v5, size);
  v7 = result;
  if ( result == -1 )
    goto LABEL_6;
  v5 = list;
LABEL_10:
  if ( !qword_84D00 )
  {
    result = sub_4C680(0, &qword_83C08, 1);
    qword_84D00 = (void *)result;
  }
  if ( v7 > 0 )
  {
    v8 = "fgetxattr";
    if ( !a4 )
      v8 = "lgetxattrat";
    v17 = v8;
    do
    {
      v9 = strlen(v5);
      v10 = qword_84D00;
      v14 = v9;
      while ( 1 )
      {
        if ( a4 )
        {
          v11 = fgetxattr(a4, v5, v10, qword_83C08);
          if ( v11 != -1 )
          {
LABEL_22:
            v15 = v11;
            if ( !(unsigned char)sub_31730(v5, 1) )
              sub_1C650(a3, v5, qword_84D00, v15);
            goto LABEL_24;
          }
        }
        else
        {
          v11 = sub_38A90(fd, path, v5, v10, qword_83C08);
          if ( v11 != -1 )
            goto LABEL_22;
        }
        v12 = *__errno_location();
        if ( v12 != 34 )
          break;
        qword_84D00 = (void *)sub_4C680(qword_84D00, &qword_83C08, 1);
        v10 = qword_84D00;
      }
      if ( v12 != 61 )
        sub_324F0(v17, path);
LABEL_24:
      result = v14;
      v5 += v14 + 1;
      v7 = v7 - v14 - 1;
    }
    while ( v7 > 0 );
  }
  return result;
}



// Function: xattrs_selinux_get @ 0x31de0
long long xattrs_selinux_get(int a1, long long a2, long long a3, unsigned int a4)
{
  long long result; // rax
  long long v5; // rdx
  const char *v7; // rdi

  result = (unsigned int)dword_84AA0;
  if ( dword_84AA0 > 0 )
  {
    v5 = a3 + 48;
    if ( a4 )
    {
      result = sub_4D390(a4, v5);
      if ( (int)result != -1 )
        return result;
    }
    else
    {
      result = sub_4B660(a1);
      if ( (int)result != -1 )
        return result;
    }
    result = (unsigned int)*__errno_location();
    if ( (int)result != 95 && (int)result != 61 )
    {
      v7 = "fgetfilecon";
      if ( !a4 )
        v7 = "lgetfileconat";
      return sub_324F0(v7, a2);
    }
  }
  return result;
}



// Function: xattrs_selinux_set @ 0x31e70
void xattrs_selinux_set(long long a1, long long a2, char a3)
{
  long long v3; // rdi
  int v4; // eax
  char *v5; // r12
  char *v6; // r13
  int *v7; // rax

  if ( dword_84AA0 > 0 && *(long long *)(a1 + 48) )
  {
    v3 = (unsigned int)oldfd;
    if ( a3 == 50 )
    {
      v4 = sub_4BAA0(oldfd);
      v5 = "lsetfileconat";
    }
    else
    {
      v4 = sub_4B880(oldfd);
      v5 = "setfileconat";
    }
    if ( v4 == -1 && (dword_83B80 & 0x200000) != 0 )
    {
      if ( qword_84D50 )
        qword_84D50(v3);
      v6 = dcgettext(0, "%s: Cannot set SELinux context for file '%s'", 5);
      v7 = __errno_location();
      error(0, *v7, v6, v5, a2);
    }
  }
}



// Function: xattrs_xattrs_set @ 0x31f20
long long xattrs_xattrs_set(long long a1, char *a2, char a3, int a4)
{
  long long result; // rax
  unsigned long long v6; // rbx
  long long v7; // r14
  char *v8; // r14

  result = (unsigned int)dword_84A98;
  if ( dword_84A98 > 0 && *(long long *)(a1 + 344) )
  {
    v6 = 0;
    do
    {
      v7 = *(long long *)(*(long long *)(a1 + 352) + 24 * v6);
      v8 = (char *)(strlen("SCHILY.xattr.") + v7);
      if ( a3 != 48 || (result = strcmp(v8, "security.capability") != 0, (int)result != a4) )
      {
        result = sub_31730(v8, 0);
        if ( !(char)result )
          result = sub_31650(a2);
      }
      ++v6;
    }
    while ( *(long long *)(a1 + 344) > v6 );
  }
  return result;
}



// Function: xattrs_print_char @ 0x32010
void xattrs_print_char(long long *a1, char *a2)
{
  long long v2; // r13
  size_t v3; // rax

  if ( dword_84A54 <= 1 )
  {
    *a2 = 0;
    return;
  }
  if ( dword_84A98 > 0 || dword_84AA0 > 0 || dword_84A9C > 0 )
  {
    *(short *)a2 = 32;
    if ( dword_84A98 > 0 && a1[43] )
    {
      v2 = 0;
      while ( 1 )
      {
        v3 = strlen("SCHILY.xattr.");
        if ( !(unsigned char)sub_31730((char *)(*(long long *)(a1[44] + 24 * v2) + v3), 0) )
          break;
        if ( a1[43] <= (unsigned long long)++v2 )
          goto LABEL_16;
      }
      *a2 = 42;
      if ( dword_84AA0 <= 0 )
        goto LABEL_11;
    }
    else
    {
LABEL_16:
      if ( dword_84AA0 <= 0 )
      {
        if ( dword_84A9C <= 0 )
          return;
        goto LABEL_12;
      }
    }
    if ( a1[6] )
      *a2 = 46;
  }
LABEL_11:
  if ( dword_84A9C <= 0 )
    return;
LABEL_12:
  if ( a1[8] || a1[10] )
    *a2 = 43;
}



// Function: xattrs_print @ 0x32150
double xattrs_print(long long *a1, double result)
{
  const char *v2; // rcx
  double v3; // xmm0_8
  long long v4; // rdx
  unsigned long long v5; // rbp
  long long v6; // r13
  char *v7; // r12

  if ( dword_84A54 > 2 )
  {
    if ( dword_84AA0 > 0 )
    {
      v2 = (const char *)a1[6];
      if ( v2 )
        result = __fprintf_chk(stream, 1, "  s: %s\n", v2);
    }
    if ( dword_84A9C > 0 && (a1[8] || a1[10]) )
    {
      v3 = __fprintf_chk(stream, 1, "  a: ");
      sub_31760("", a1[7], a1[8], v3);
      v4 = a1[10];
      if ( a1[8] && v4 )
      {
        v3 = __fprintf_chk(stream, 1, ",");
        v4 = a1[10];
      }
      sub_31760("default:", a1[9], v4, v3);
      result = __fprintf_chk(stream, 1, "\n");
    }
    if ( dword_84A98 > 0 && a1[43] )
    {
      v5 = 0;
      do
      {
        while ( 1 )
        {
          v6 = 24 * v5;
          v7 = (char *)(*(long long *)(a1[44] + 24 * v5) + strlen("SCHILY.xattr."));
          if ( !(unsigned char)sub_31730(v7, 0) )
            break;
          if ( a1[43] <= ++v5 )
            return result;
        }
        ++v5;
        result = __fprintf_chk(stream, 1, "  x: %lu %s\n", *(long long *)(a1[44] + v6 + 16), v7);
      }
      while ( a1[43] > v5 );
    }
  }
  return result;
}


