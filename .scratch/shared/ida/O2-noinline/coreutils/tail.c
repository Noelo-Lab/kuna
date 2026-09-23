// Function: main @ 0x29a0
long long main(unsigned int a1, char **a2, char **a3)
{
  char *v4; // rdi
  unsigned char v5; // al
  int v6; // ebx
  char **v7; // rbp
  char **v8; // rdx
  const char *v9; // rbx
  const char **v10; // rbp
  char v11; // r14
  const char *v12; // r12
  long long v13; // rax
  long long v14; // r15
  const char *v15; // rbp
  long long v16; // rdx
  const char *v17; // rdi
  long long result; // rax
  int v19; // ebp
  char *v20; // rax
  int *v21; // rax
  char *v22; // rax
  int *v23; // rax
  char *v24; // rax
  long long v25; // r12
  char *v26; // rax
  long long v27; // [rsp+8h] [rbp-100h]
  long long v28; // [rsp+18h] [rbp-F0h] BYREF
  double v29; // [rsp+20h] [rbp-E8h]
  void *ptr; // [rsp+28h] [rbp-E0h]
  struct stat buf; // [rsp+30h] [rbp-D8h] BYREF
  unsigned long long v32; // [rsp+C8h] [rbp-40h]

  v4 = *a2;
  v32 = __readfsqword(0x28u);
  v28 = 10;
  v29 = 1.0;
  sub_91A0(v4);
  setlocale(6, "");
  bindtextdomain("coreutils", "/usr/local/share/locale");
  textdomain("coreutils");
  sub_D460(sub_7B00);
  byte_1510E = 0;
  byte_15119 = 1;
  byte_15115 = 0;
  byte_15116 = 0;
  byte_15118 = 0;
  byte_15114 = 10;
  v5 = sub_3BC0(a1, a2, &v28);
  v6 = a1 - v5;
  v7 = &a2[v5];
  sub_7080(v6, v7);
  if ( byte_15116 && v28 )
    --v28;
  v8 = (char **)&off_15028;
  v27 = 1;
  if ( optind < v6 )
  {
    v8 = &v7[optind];
    v27 = v6 - optind;
  }
  v9 = (const char *)v8;
  v10 = (const char **)v8;
  v11 = 0;
  v12 = (const char *)&v8[v27];
  do
  {
    if ( !strcmp(*v10, "-") )
      v11 = 1;
    ++v10;
  }
  while ( v12 != (const char *)v10 );
  if ( v11 )
  {
    if ( dword_15020 == 1 )
    {
      v25 = sub_B410(4, "-");
      v26 = dcgettext(0, "cannot follow %s by name", 5);
      error(1, 0, v26, v25);
      start();
    }
    if ( !byte_15118 )
    {
      if ( v28 )
        goto LABEL_19;
      goto LABEL_29;
    }
    if ( (pid || dword_15020 != 2 || v27 != 1 || fstat(0, &buf) || (buf.st_mode & 0xF000) == 0x8000) && isatty(0) )
      goto LABEL_48;
  }
  while ( 1 )
  {
    if ( !v28 && !byte_15118 )
    {
LABEL_29:
      result = 0;
      if ( !byte_15116 )
        return result;
    }
LABEL_19:
    v13 = sub_C0C0(v27, 96);
    v14 = v13;
    v15 = (const char *)v13;
    do
    {
      v16 = *(long long *)v9;
      v9 += 8;
      v13 += 96;
      *(long long *)(v13 - 96) = v16;
    }
    while ( v9 != v12 );
    if ( v27 != 1 )
      byte_15115 = 1;
    LODWORD(v9) = 1;
    v12 = (const char *)(v14 + 96 * v27);
    do
    {
      v17 = v15;
      v15 += 96;
      v9 = (const char *)((unsigned int)sub_6AB0(v17, v28) & (unsigned int)v9);
    }
    while ( v12 != v15 );
    if ( byte_15118 && sub_3A50(v14, v27) )
    {
      if ( fstat(1, &buf) < 0 )
      {
        v12 = dcgettext(0, "standard output", 5);
        v23 = __errno_location();
        error(1, *v23, v12);
LABEL_53:
        v24 = dcgettext(0, "write error", 5);
        error(1, *(int *)v12, v24);
      }
      byte_15117 = (buf.st_mode & 0xF000) == 4096;
      if ( !byte_1510C
        && !(unsigned char)sub_3AE0(v14, v27)
        && !(unsigned char)sub_38D0(v14, v27)
        && (unsigned char)sub_3900(v14)
        && !(unsigned char)sub_4B40(v14)
        && !(unsigned char)sub_3930(v14, v27)
        && ((char)v9 || dword_15020 != 2) )
      {
        v19 = inotify_init();
        v12 = (const char *)__errno_location();
        if ( v19 >= 0 )
        {
          if ( fflush_unlocked(stdout) )
            goto LABEL_53;
          sub_5880(v19);
          sub_8A10(ptr);
          close(v19);
          *(int *)v12 = 0;
        }
        v20 = dcgettext(0, "inotify cannot be used, reverting to polling", 5);
        error(0, *(int *)v12, v20);
      }
      byte_1510C = 1;
      sub_6220(v14, v27, v29);
    }
    if ( !byte_1510E || close(0) >= 0 )
      return (unsigned char)v9 ^ 1u;
    v21 = __errno_location();
    error(1, *v21, "-");
LABEL_48:
    v22 = dcgettext(0, "warning: following standard input indefinitely is ineffective", 5);
    error(0, 0, v22);
  }
}



// Function: record_open_fd @ 0x3830
long long record_open_fd(long long a1, int a2, long long a3, long long *a4, int a5)
{
  long long v7; // rax
  long long v8; // rdx
  long long result; // rax

  *(int *)(a1 + 56) = a2;
  *(long long *)(a1 + 8) = a3;
  *(long long *)(a1 + 16) = sub_B7B0(a4);
  v7 = *a4;
  *(long long *)(a1 + 24) = v8;
  *(long long *)(a1 + 32) = v7;
  *(long long *)(a1 + 40) = a4[1];
  result = *((unsigned int *)a4 + 6);
  *(int *)(a1 + 64) = a5;
  *(int *)(a1 + 48) = result;
  *(long long *)(a1 + 88) = 0;
  *(char *)(a1 + 52) = 0;
  return result;
}



// Function: any_live_files @ 0x3880
char any_live_files(long long a1, long long a2)
{
  char result; // al
  long long v3; // rdi
  long long v4; // rcx

  if ( !byte_1511A || (result = byte_1511A, dword_15020 != 1) )
  {
    v3 = a1 + 52;
    v4 = v3 + 96 * a2;
    while ( *(int *)(v3 + 4) < 0 )
    {
      result = byte_1511A & (*(char *)v3 ^ 1);
      if ( !result )
      {
        v3 += 96;
        if ( v3 != v4 )
          continue;
      }
      return result;
    }
    return 1;
  }
  return result;
}



// Function: any_remote_file @ 0x38d0
long long any_remote_file(long long a1, long long a2)
{
  unsigned char *v2; // rdi
  unsigned char *v3; // rax
  unsigned int v4; // r8d

  v2 = (unsigned char *)(a1 + 53);
  v3 = &v2[96 * a2];
  while ( 1 )
  {
    if ( *(int *)(v2 + 3) >= 0 )
    {
      v4 = *v2;
      if ( (char)v4 )
        break;
    }
    v2 += 96;
    if ( v2 == v3 )
      return 0;
  }
  return v4;
}



// Function: any_non_remote_file @ 0x3900
long long any_non_remote_file(long long a1, long long a2)
{
  long long v2; // rdi
  long long v3; // rax

  v2 = a1 + 53;
  v3 = v2 + 96 * a2;
  while ( *(int *)(v2 + 3) < 0 || *(char *)v2 )
  {
    v2 += 96;
    if ( v2 == v3 )
      return 0;
  }
  return 1;
}



// Function: any_non_regular_fifo @ 0x3930
bool any_non_regular_fifo(long long a1, long long a2)
{
  int *v2; // rdi
  int *v3; // rcx
  bool result; // al

  v2 = (int *)(a1 + 48);
  v3 = &v2[24 * a2];
  while ( 1 )
  {
    if ( (int)v2[2] >= 0 )
    {
      result = (*v2 & 0xF000) != 0x8000 && (*v2 & 0xF000) != 4096;
      if ( result )
        break;
    }
    v2 += 24;
    if ( v3 == v2 )
      return 0;
  }
  return result;
}



// Function: wd_hasher @ 0x3980
unsigned long long wd_hasher(long long a1, unsigned long long a2)
{
  return *(int *)(a1 + 68) % a2;
}



// Function: wd_comparator @ 0x39a0
bool wd_comparator(long long a1, long long a2)
{
  return *(int *)(a1 + 68) == *(int *)(a2 + 68);
}



// Function: write_header @ 0x3a10
long long write_header(long long a1)
{
  const char *v1; // rdx
  long long result; // rax

  v1 = "";
  if ( !byte_15010 )
    v1 = "\n";
  result = __printf_chk(1, "%s==> %s <==\n", v1, a1);
  byte_15010 = 0;
  return result;
}



// Function: ignore_fifo_and_pipe @ 0x3a50
long long ignore_fifo_and_pipe(long long a1, long long a2)
{
  long long v2; // r12
  long long v3; // rbx

  v2 = 0;
  v3 = a1;
  do
  {
    if ( !strcmp(*(const char **)v3, "-")
      && !*(char *)(v3 + 52)
      && *(int *)(v3 + 56) >= 0
      && (*(int *)(v3 + 48) & 0xF000) == 0x1000 )
    {
      *(int *)(v3 + 56) = -1;
      *(char *)(v3 + 52) = 1;
    }
    else
    {
      ++v2;
    }
    v3 += 96;
  }
  while ( a1 + 96 * a2 != v3 );
  return v2;
}



// Function: tailable_stdin @ 0x3ae0
long long tailable_stdin(long long a1, long long a2)
{
  long long v2; // rbx

  v2 = a1;
  while ( *(char *)(v2 + 52) || strcmp(*(const char **)v2, "-") )
  {
    v2 += 96;
    if ( v2 == a1 + 96 * a2 )
      return 0;
  }
  return 1;
}



// Function: close_fd @ 0x3b30
void close_fd(unsigned int a1, long long a2)
{
  long long v2; // r13
  char *v3; // r12
  int *v4; // rax

  if ( a1 + 1 > 1 )
  {
    if ( close(a1) )
    {
      v2 = sub_B410(4, a2);
      v3 = dcgettext(0, "closing %s (fd=%d)", 5);
      v4 = __errno_location();
      error(0, *v4, v3, v2, a1);
    }
  }
}



// Function: parse_obsolete_option @ 0x3bc0
long long parse_obsolete_option(int a1, long long a2, long long *a3)
{
  long long v4; // rdi
  int v5; // eax
  char *v6; // rdx
  char *v7; // rdi
  char v9; // bp
  int v10; // eax
  char *v11; // rdx
  char v12; // cl
  int v13; // eax
  char *v14; // rax
  char v15; // r13
  unsigned int v16; // ecx
  char v17; // r14
  const char *v18; // rbx
  char *v19; // r12
  int *v20; // rax

  if ( a1 != 2 )
  {
    if ( a1 == 3 )
    {
      v4 = *(long long *)(a2 + 16);
      if ( *(char *)v4 != 45 || !*(char *)(v4 + 1) )
        goto LABEL_6;
    }
    else
    {
      if ( a1 != 4 )
        return 0;
      v4 = *(long long *)(a2 + 16);
    }
    if ( strcmp((const char *)v4, "--") )
      return 0;
  }
LABEL_6:
  v5 = sub_9110();
  v6 = *(char **)(a2 + 8);
  v7 = v6 + 1;
  if ( *v6 != 43 )
  {
    if ( *v6 == 45 && (v5 <= 200111 || v6[(v6[1] == 99) + 1]) )
    {
      v9 = 0;
      goto LABEL_12;
    }
    return 0;
  }
  v9 = 1;
  if ( (unsigned int)(v5 - 200112) <= 0x2B8 )
    return 0;
LABEL_12:
  v10 = (char)v6[1];
  v11 = v6 + 1;
  v12 = v10;
  if ( (unsigned int)(v10 - 48) <= 9 )
  {
    do
    {
      v13 = *++v11;
      v12 = v13;
    }
    while ( (unsigned int)(v13 - 48) <= 9 );
  }
  switch ( v12 )
  {
    case 'c':
      v15 = 0;
      v16 = 10;
      break;
    case 'l':
      v15 = 1;
      v16 = 10;
      break;
    case 'b':
      v15 = 0;
      v16 = 5120;
      break;
    default:
      v14 = v11;
      v15 = 1;
      v16 = 10;
      goto LABEL_18;
  }
  v14 = v11 + 1;
LABEL_18:
  v17 = 0;
  if ( *v14 == 102 )
  {
    ++v14;
    v17 = 1;
  }
  if ( *v14 )
    return 0;
  if ( v7 == v11 )
  {
    *a3 = v16;
    goto LABEL_23;
  }
  if ( (sub_C6C0(v7) & 0xFFFFFFFD) == 0 )
  {
LABEL_23:
    byte_15116 = v9;
    byte_15119 = v15;
    byte_15118 = v17;
    return 1;
  }
  v18 = (const char *)sub_B6D0(*(long long *)(a2 + 8), 0);
  v19 = dcgettext(0, "invalid number", 5);
  v20 = __errno_location();
  error(1, *v20, "%s: %s", v19, v18);
  return sub_3DA0();
}



// Function: xwrite_stdout @ 0x3da0
void xwrite_stdout(const void *a1, size_t a2)
{
  long long v2; // r13
  char *v3; // r12
  int *v4; // rax

  if ( a2 )
  {
    if ( fwrite_unlocked(a1, 1u, a2, stdout) < a2 )
    {
      clearerr_unlocked(stdout);
      v2 = sub_B410(4, "standard output");
      v3 = dcgettext(0, "error writing %s", 5);
      v4 = __errno_location();
      error(1, *v4, v3, v2);
      sub_3E30();
    }
  }
}



// Function: start_lines @ 0x3e30
long long start_lines(long long a1, int a2, long long a3, long long *a4)
{
  long long result; // rax
  long long v5; // rbx
  long long v7; // rax
  int v8; // r12d
  char *v9; // rbp
  char *v10; // rdi
  char *v11; // rax
  long long v12; // r13
  char *v13; // r12
  int *v14; // rax
  long long v15; // [rsp+8h] [rbp-2050h]
  char s[24]; // [rsp+10h] [rbp-2048h] BYREF
  unsigned long long v17; // [rsp+2018h] [rbp-40h]

  v15 = a1;
  v17 = __readfsqword(0x28u);
  result = 0;
  if ( a3 )
  {
    v5 = a3;
    while ( 1 )
    {
      v7 = sub_B6E0(a2, s, 0x2000u);
      if ( !v7 )
        return 0xFFFFFFFFLL;
      if ( v7 == -1 )
        break;
      *a4 += v7;
      v8 = byte_15114;
      v9 = &s[v7];
      v10 = s;
      while ( 1 )
      {
        v11 = (char *)memchr(v10, v8, v9 - v10);
        if ( !v11 )
          break;
        v10 = v11 + 1;
        if ( !--v5 )
        {
          result = 0;
          if ( v9 > v10 )
          {
            sub_3DA0(v10, v9 - v10);
            return 0;
          }
          return result;
        }
      }
    }
    v12 = sub_B410(4, v15);
    v13 = dcgettext(0, "error reading %s", 5);
    v14 = __errno_location();
    error(0, *v14, v13, v12);
    return 1;
  }
  return result;
}



// Function: dump_remainder @ 0x3f80
long long dump_remainder(char a1, long long a2, int a3, unsigned long long a4)
{
  long long v4; // r12
  unsigned long long v7; // rbx
  size_t v8; // rdx
  long long v9; // rax
  size_t v10; // r14
  int *v11; // rbx
  long long v13; // r12
  char *v14; // rax
  char buf[24]; // [rsp+10h] [rbp-2048h] BYREF
  unsigned long long v17; // [rsp+2018h] [rbp-40h]

  v4 = 0;
  v7 = a4;
  v17 = __readfsqword(0x28u);
  while ( 1 )
  {
    v8 = 0x2000;
    if ( v7 <= 0x2000 )
      v8 = v7;
    v9 = sub_B6E0(a3, buf, v8);
    v10 = v9;
    if ( v9 == -1 )
      break;
    if ( !v9 )
      return v4;
    if ( a1 )
      sub_3A10(a2);
    v4 += v10;
    sub_3DA0(buf, v10);
    if ( a4 != -1 )
    {
      v7 -= v10;
      if ( !v7 || a4 == -2 )
        return v4;
    }
    a1 = 0;
  }
  v11 = __errno_location();
  if ( *v11 != 11 )
  {
    v13 = sub_B410(4, a2);
    v14 = dcgettext(0, "error reading %s", 5);
    error(1, *v11, v14, v13);
  }
  return v4;
}



// Function: start_bytes @ 0x40c0
long long start_bytes(long long a1, int a2, unsigned long long a3, long long *a4)
{
  unsigned long long v4; // r14
  unsigned long long v6; // rax
  long long v8; // r13
  char *v9; // r12
  int *v10; // rax
  char v11[16]; // [rsp+0h] [rbp-2038h] BYREF
  unsigned long long v12; // [rsp+2008h] [rbp-30h]

  v12 = __readfsqword(0x28u);
  if ( !a3 )
    return 0;
  v4 = a3;
  while ( 1 )
  {
    v6 = sub_B6E0(a2, v11, 0x2000u);
    if ( !v6 )
      return 0xFFFFFFFFLL;
    if ( v6 == -1 )
      break;
    *a4 += v6;
    if ( v6 > v4 )
    {
      sub_3DA0(&v11[v4], v6 - v4);
      return 0;
    }
    v4 -= v6;
    if ( !v4 )
      return 0;
  }
  v8 = sub_B410(4, a1);
  v9 = dcgettext(0, "error reading %s", 5);
  v10 = __errno_location();
  error(0, *v10, v9, v8);
  return 1;
}



// Function: xlseek @ 0x41c0
__off_t xlseek(int a1, __off_t a2, int a3, long long a4)
{
  __off_t result; // rax
  long long v7; // r13
  int *v8; // rbp
  long long v9; // rax
  const char *v10; // rsi
  long long v11; // r12
  char *v12; // rax
  long long v13[9]; // [rsp+0h] [rbp-48h] BYREF

  v13[3] = __readfsqword(0x28u);
  result = lseek(a1, a2, a3);
  if ( result < 0 )
  {
    v7 = sub_9070(a2, v13);
    v8 = __errno_location();
    v9 = sub_B510(0, 3, a4);
    if ( a3 == 1 )
      v10 = "%s: cannot seek to relative offset %s";
    else
      v10 = "%s: cannot seek to offset %s";
    v11 = v9;
    v12 = dcgettext(0, v10, 5);
    error(0, *v8, v12, v11, v7);
    exit(1);
  }
  return result;
}



// Function: file_lines @ 0x4290
long long file_lines(long long a1, int a2, long long a3, long long a4, unsigned long long a5, long long *a6)
{
  long long v6; // r14
  size_t v7; // rbx
  __off_t v8; // r12
  long long v9; // rax
  size_t v10; // rbp
  int v11; // ebx
  size_t v12; // rdx
  char *v13; // rax
  long long v14; // rax
  long long v16; // r13
  char *v17; // r12
  int *v18; // rax
  unsigned long long v21; // [rsp+18h] [rbp-2050h]
  char s[24]; // [rsp+20h] [rbp-2048h] BYREF
  unsigned long long v23; // [rsp+2028h] [rbp-40h]

  v21 = a5;
  v23 = __readfsqword(0x28u);
  if ( !a3 )
    return 1;
  v6 = a3;
  v7 = (long long)(a5 - a4) % 0x2000;
  if ( !v7 )
    v7 = 0x2000;
  v8 = a5 - v7;
  sub_41C0(a2, a5 - v7, 0, a1);
  v9 = sub_B6E0(a2, s, v7);
  v10 = v9;
  if ( v9 == -1 )
  {
LABEL_19:
    v16 = sub_B410(4, a1);
    v17 = dcgettext(0, "error reading %s", 5);
    v18 = __errno_location();
    error(0, *v18, v17, v16);
    return 0;
  }
  else
  {
    v11 = byte_15114;
    *a6 = v8 + v9;
    if ( v9 && s[v9 - 1] != (char)v11 )
      --v6;
    while ( 2 )
    {
      v12 = v10;
      while ( v12 )
      {
        v13 = (char *)memrchr(s, v11, v12);
        if ( !v13 )
          break;
        v12 = v13 - s;
        if ( !v6 )
        {
          sub_3DA0(v13 + 1, v10 - 1 - v12);
          *a6 += sub_3F80(0, a1, a2, v21 - v10 - v8);
          return 1;
        }
        --v6;
      }
      if ( v8 != a4 )
      {
        v8 -= 0x2000LL;
        sub_41C0(a2, v8, 0, a1);
        v14 = sub_B6E0(a2, s, 0x2000u);
        v10 = v14;
        if ( v14 == -1 )
          goto LABEL_19;
        *a6 = v8 + v14;
        if ( v14 )
        {
          v11 = byte_15114;
          continue;
        }
        return 1;
      }
      break;
    }
    sub_41C0(a2, v8, 0, a1);
    *a6 = sub_3F80(0, a1, a2, v21) + v8;
    return 1;
  }
}



// Function: pipe_bytes @ 0x4510
long long pipe_bytes(long long a1, int a2, size_t a3, long long *a4)
{
  size_t v4; // r15
  long long v5; // rax
  long long *v6; // r14
  size_t *v7; // rbx
  size_t *v8; // r13
  size_t *v9; // rbp
  size_t v10; // r12
  long long v11; // rdi
  size_t v12; // rsi
  size_t *v13; // rbp
  size_t i; // rax
  size_t *v15; // rdi
  size_t *j; // rbp
  unsigned int v17; // r12d
  size_t *v18; // rdi
  long long v20; // r13
  char *v21; // r12
  int *v22; // rax
  const char *v23; // rdx

  v4 = 0;
  v5 = sub_BFA0(8208);
  *(long long *)(v5 + 0x2000) = 0;
  v6 = (long long *)v5;
  *(long long *)(v5 + 8200) = 0;
  v7 = (size_t *)v5;
  v8 = (size_t *)sub_BFA0(8208);
  v9 = v8;
  while ( 1 )
  {
    v10 = sub_B6E0(a2, v9, 0x2000u);
    if ( v10 - 1 > 0xFFFFFFFFFFFFFFFDLL )
      break;
    v4 += v10;
    *a4 += v10;
    v8[1024] = v10;
    v11 = v6[1024];
    v8[1025] = 0;
    if ( v11 + v10 <= 0x1FFF )
    {
      memcpy((char *)v6 + v11, v9, v10);
      v6[1024] += v8[1024];
    }
    else
    {
      v6[1025] = v8;
      if ( v4 - v7[1024] <= a3 )
      {
        v6 = v8;
        v9 = (size_t *)sub_BFA0(8208);
        v8 = v9;
      }
      else
      {
        v9 = v7;
        v6 = v8;
        v4 -= v7[1024];
        v8 = v7;
        v7 = (size_t *)v7[1025];
      }
    }
  }
  free(v8);
  if ( v10 == -1 )
  {
    v20 = sub_B410(4, a1);
    v21 = dcgettext(0, "error reading %s", 5);
    v22 = __errno_location();
    v23 = v21;
    v17 = 0;
    error(0, *v22, v23, v20);
    if ( !v7 )
      return v17;
  }
  else
  {
    v12 = v7[1024];
    v13 = v7;
    for ( i = v4 - v12; i > a3; i -= v12 )
    {
      v13 = (size_t *)v13[1025];
      v4 = i;
      v12 = v13[1024];
    }
    v15 = v13;
    if ( a3 < v4 )
    {
      v12 += a3 - v4;
      v15 = (size_t *)((char *)v13 + v4 - a3);
    }
    sub_3DA0(v15, v12);
    for ( j = (size_t *)v13[1025]; j; j = (size_t *)j[1025] )
      sub_3DA0(j, j[1024]);
    v17 = 1;
  }
  do
  {
    v18 = v7;
    v7 = (size_t *)v7[1025];
    free(v18);
  }
  while ( v7 );
  return v17;
}



// Function: pipe_lines @ 0x4760
long long pipe_lines(long long a1, int a2, unsigned long long a3, long long *a4)
{
  unsigned long long v4; // r14
  long long *v5; // rax
  long long *v6; // r13
  long long *v7; // r12
  long long *v8; // r15
  long long v9; // rbx
  const void *v10; // rdi
  int v11; // ebp
  char *v12; // rbx
  char *v13; // rax
  size_t v14; // rdx
  long long v15; // rdi
  unsigned int v16; // edx
  long long v17; // rcx
  unsigned int v18; // r15d
  void *v19; // rdi
  char v21; // dl
  char *v22; // rbx
  unsigned long long i; // rax
  char *v24; // rdi
  char *v25; // rbp
  unsigned long long v26; // r14
  int v27; // r13d
  size_t *j; // rbx
  long long v29; // r14
  char *v30; // r13
  int *v31; // rax

  v4 = 0;
  v5 = (long long *)sub_BFA0(8216);
  v5[1025] = 0;
  v6 = v5;
  v5[1024] = 0;
  v7 = v5;
  v5[1026] = 0;
  v8 = (long long *)sub_BFA0(8216);
  while ( 1 )
  {
    v9 = sub_B6E0(a2, v8, 0x2000u);
    if ( (unsigned long long)(v9 - 1) > 0xFFFFFFFFFFFFFFFDLL )
      break;
    v8[1024] = v9;
    v10 = v8;
    v11 = byte_15114;
    *a4 += v9;
    v12 = (char *)v8 + v9;
    v8[1025] = 0;
    v8[1026] = 0;
    while ( 1 )
    {
      v13 = (char *)memchr(v10, v11, v12 - (char *)v10);
      if ( !v13 )
        break;
      ++v8[1025];
      v10 = v13 + 1;
    }
    v14 = v8[1024];
    v15 = v6[1024];
    v4 += v8[1025];
    if ( v14 + v15 <= 0x1FFF )
    {
      memcpy((char *)v6 + v15, v8, v14);
      v6[1024] += v8[1024];
      v6[1025] += v8[1025];
    }
    else
    {
      v6[1026] = v8;
      v6 = v8;
      if ( v4 - v7[1025] <= a3 )
      {
        v8 = (long long *)sub_BFA0(8216);
      }
      else
      {
        v4 -= v7[1025];
        v8 = v7;
        v7 = (long long *)v7[1026];
      }
    }
  }
  free(v8);
  if ( v9 == -1 )
  {
    v18 = 0;
    v29 = sub_B410(4, a1);
    v30 = dcgettext(0, "error reading %s", 5);
    v31 = __errno_location();
    error(0, *v31, v30, v29);
LABEL_13:
    if ( !v7 )
      return v18;
    goto LABEL_14;
  }
  v17 = v6[1024];
  LOBYTE(v16) = a3 == 0 || v17 == 0;
  v18 = v16;
  if ( (char)v16 )
    goto LABEL_13;
  v21 = byte_15114;
  if ( *((char *)v6 + v17 - 1) != byte_15114 )
  {
    ++v6[1025];
    ++v4;
  }
  v22 = (char *)v7;
  for ( i = v4 - v7[1025]; i > a3; i -= *((long long *)v22 + 1025) )
  {
    v22 = (char *)*((long long *)v22 + 1026);
    v4 = i;
  }
  v24 = v22;
  v25 = &v22[*((long long *)v22 + 1024)];
  if ( a3 < v4 )
  {
    v26 = v4 - a3;
    v27 = v21;
    do
    {
      v24 = (char *)rawmemchr(v24, v27) + 1;
      --v26;
    }
    while ( v26 );
  }
  sub_3DA0(v24, v25 - v24);
  for ( j = (size_t *)*((long long *)v22 + 1026); j; j = (size_t *)j[1026] )
    sub_3DA0(j, j[1024]);
  v18 = 1;
  do
  {
LABEL_14:
    v19 = v7;
    v7 = (long long *)v7[1026];
    free(v19);
  }
  while ( v7 );
  return v18;
}



// Function: fremote @ 0x4a40
long long fremote(int a1, long long a2)
{
  unsigned int v2; // r12d
  int *v3; // rbx
  int v5; // eax
  long long v6; // r13
  char *v7; // rax
  struct statfs v8; // [rsp+0h] [rbp-A8h] BYREF
  unsigned long long v9; // [rsp+78h] [rbp-30h]

  v9 = __readfsqword(0x28u);
  if ( !fstatfs(a1, &v8) )
  {
    v5 = sub_2FB0(v8.f_type);
    if ( v5 > 0 )
    {
      if ( v5 == 1 )
        return 0;
    }
    else if ( v5 >= -1 )
    {
      return 1;
    }
    __assert_fail("!\"unexpected return value from is_local_fs_type\"", "src/tail.c", 0x3CCu, "fremote");
  }
  v2 = 1;
  v3 = __errno_location();
  if ( *v3 != 38 )
  {
    v6 = sub_B410(4, a2);
    v7 = dcgettext(0, "cannot determine location of %s. reverting to polling", 5);
    error(0, *v3, v7, v6);
  }
  return v2;
}



// Function: any_symlinks @ 0x4b40
long long any_symlinks(const char **a1, long long a2)
{
  const char **v2; // rbx
  struct stat v4; // [rsp+0h] [rbp-B8h] BYREF
  unsigned long long v5; // [rsp+98h] [rbp-20h]

  v2 = a1;
  v5 = __readfsqword(0x28u);
  while ( lstat(*v2, &v4) || (v4.st_mode & 0xF000) != 0xA000 )
  {
    v2 += 12;
    if ( v2 == &a1[12 * a2] )
      return 0;
  }
  return 1;
}



// Function: die_pipe @ 0x4bd0
void die_pipe()
{
  raise(13);
  exit(1);
}



// Function: check_output_alive @ 0x4bf0
unsigned long long check_output_alive()
{
  long long v1; // [rsp+0h] [rbp-18h] BYREF
  unsigned long long v2; // [rsp+8h] [rbp-10h]

  v2 = __readfsqword(0x28u);
  if ( byte_15117 )
  {
    v1 = 1;
    if ( poll((struct pollfd *)&v1, 1u, 0) >= 0 && (v1 & 0x18000000000000LL) != 0 )
      sub_4BD0();
  }
  return v2 - __readfsqword(0x28u);
}



// Function: check_fspec @ 0x5090
unsigned long long check_fspec(long long a1, long long *a2)
{
  long long v3; // r13
  int v4; // eax
  char v5; // bp
  long long v6; // rax
  __off_t v8; // rax
  long long v9; // rax
  long long v10; // rdx
  char *v11; // r12
  int *v12; // rax
  int *v13; // rax
  unsigned int v14; // edi
  long long v15; // r14
  char *v16; // rax
  struct stat v17; // [rsp+0h] [rbp-C8h] BYREF
  unsigned long long v18; // [rsp+98h] [rbp-30h]

  v18 = __readfsqword(0x28u);
  if ( *(int *)(a1 + 56) != -1 )
  {
    v3 = sub_5050(*(long long *)a1);
    v4 = fstat(*(int *)(a1 + 56), &v17);
    v5 = v4;
    if ( v4 )
    {
LABEL_14:
      v13 = __errno_location();
      v14 = *(int *)(a1 + 56);
      *(int *)(a1 + 60) = *v13;
      sub_3B30(v14, v3);
      *(int *)(a1 + 56) = -1;
      return v18 - __readfsqword(0x28u);
    }
    if ( (*(int *)(a1 + 48) & 0xF000) == 0x8000 )
    {
      v8 = *(long long *)(a1 + 8);
      if ( v17.st_size < v8 )
      {
        v15 = sub_B510(0, 3, v3);
        v16 = dcgettext(0, "%s: file truncated", 5);
        error(0, 0, v16, v15);
        sub_41C0(*(int *)(a1 + 56), 0, 0, v3);
        *(long long *)(a1 + 8) = 0;
      }
      else if ( v17.st_size == v8 )
      {
        v9 = sub_B7B0(&v17);
        if ( !(unsigned int)sub_B800(*(long long *)(a1 + 16), *(long long *)(a1 + 24), v9, v10) )
          return v18 - __readfsqword(0x28u);
      }
    }
    if ( byte_15115 )
      v5 = *a2 != a1;
    v6 = sub_3F80(v5, v3, *(int *)(a1 + 56), 0xFFFFFFFFFFFFFFFFLL);
    *(long long *)(a1 + 8) += v6;
    if ( v6 )
    {
      *a2 = a1;
      if ( fflush_unlocked(stdout) )
      {
        v11 = dcgettext(0, "write error", 5);
        v12 = __errno_location();
        error(1, *v12, v11);
        goto LABEL_14;
      }
    }
  }
  return v18 - __readfsqword(0x28u);
}



// Function: recheck @ 0x5270
unsigned long long recheck(long long *a1, unsigned char a2)
{
  const char *v3; // rbp
  const char *v4; // rdi
  int v5; // eax
  int v6; // edx
  int v7; // ecx
  int v8; // r8d
  int v9; // r9d
  unsigned int v10; // r14d
  int v11; // r12d
  unsigned int v12; // eax
  long long v13; // rsi
  unsigned int v14; // ebp
  char v15; // r15
  char v16; // al
  int *v17; // rax
  char v18; // r13
  const char *v19; // r15
  int *v20; // r12
  int v21; // eax
  long long v22; // rax
  long long v23; // rax
  long long v25; // rax
  char v26; // al
  int v27; // eax
  long long v28; // rax
  const char *v29; // rsi
  long long v30; // r14
  char *v31; // rax
  int v32; // r8d
  long long v33; // rax
  long long v34; // rax
  long long v35; // r12
  char *v36; // rax
  long long v37; // rax
  const char *v38; // rax
  const char *v39; // rdi
  long long v40; // rax
  long long v41; // rax
  long long v42; // r14
  char *v43; // rax
  long long v44; // rax
  char *v45; // r13
  long long v46; // rax
  long long v47; // r12
  char *v48; // rax
  const char *v49; // rdi
  long long v50; // rax
  long long v51; // r12
  char *v52; // rax
  long long v53; // rax
  long long v54; // r12
  char *v55; // rax
  long long v56; // rax
  char v57; // [rsp+Bh] [rbp-DDh]
  struct stat buf; // [rsp+10h] [rbp-D8h] BYREF
  unsigned long long v60; // [rsp+A8h] [rbp-40h]

  v3 = (const char *)*a1;
  v4 = (const char *)*a1;
  v60 = __readfsqword(0x28u);
  v5 = strcmp(v4, "-");
  v10 = *((int *)a1 + 15);
  v11 = v5;
  v57 = *((char *)a1 + 54);
  if ( v5 )
  {
    v12 = sub_7EA0((int)v3, (unsigned char)(a2 ^ 1) << 11, v6, v7, v8, v9);
    v13 = *((unsigned int *)a1 + 15);
    v14 = v12;
  }
  else
  {
    v13 = v10;
    v14 = 0;
  }
  if ( !(unsigned char)sub_5260(*((unsigned int *)a1 + 14), v13) )
    __assert_fail("valid_file_spec (f)", "src/tail.c", 0x3E2u, "recheck");
  v15 = byte_1511A;
  v16 = byte_1510C;
  if ( byte_1511A )
  {
    if ( v14 == -1 )
    {
      *((char *)a1 + 54) = 0;
      v19 = (const char *)*a1;
      if ( v16 )
      {
        *((int *)a1 + 15) = *__errno_location();
        goto LABEL_29;
      }
      if ( !lstat((const char *)*a1, &buf) && (buf.st_mode & 0xF000) == 0xA000 )
        goto LABEL_53;
LABEL_7:
      v17 = __errno_location();
      v18 = *((char *)a1 + 54);
      v19 = (const char *)*a1;
      v20 = v17;
      v21 = *v17;
      *((int *)a1 + 15) = v21;
      if ( v18 )
      {
        if ( v21 != v10 )
        {
          v37 = sub_5050(v19);
          v38 = (const char *)sub_B510(0, 3, v37);
          error(0, *v20, "%s", v38);
        }
        goto LABEL_10;
      }
LABEL_29:
      if ( v57 )
      {
        v34 = sub_5050(v19);
        v35 = sub_B410(4, v34);
        v36 = dcgettext(0, "%s has become inaccessible", 5);
        error(0, *((int *)a1 + 15), v36, v35);
      }
      goto LABEL_10;
    }
    *((char *)a1 + 54) = 1;
    if ( !v16 && !lstat((const char *)*a1, &buf) && (buf.st_mode & 0xF000) == 0xA000 )
      goto LABEL_53;
  }
  else
  {
    *((char *)a1 + 54) = 1;
    if ( !v16 && !lstat((const char *)*a1, &buf) && (buf.st_mode & 0xF000) == 0xA000 )
    {
LABEL_53:
      *((int *)a1 + 15) = -1;
      v49 = (const char *)*a1;
      *((char *)a1 + 52) = 1;
      v50 = sub_5050(v49);
      v51 = sub_B410(4, v50);
      v52 = dcgettext(0, "%s has been replaced with an untailable symbolic link", 5);
      error(0, 0, v52, v51);
      goto LABEL_10;
    }
    if ( v14 == -1 )
      goto LABEL_7;
  }
  if ( fstat(v14, &buf) < 0 )
    goto LABEL_7;
  if ( (((buf.st_mode & 0xF000) - 4096) & 0xFFFFE000) != 0 && (buf.st_mode & 0xB000) != 0x8000 )
  {
    *((int *)a1 + 15) = -1;
    *((char *)a1 + 54) = 0;
    if ( v15 && dword_15020 == 1 )
    {
      *((char *)a1 + 52) = 0;
      if ( v57 )
      {
        v45 = (char *)"";
      }
      else
      {
        v45 = (char *)"";
        if ( v10 == -1 )
          goto LABEL_10;
      }
    }
    else
    {
      *((char *)a1 + 52) = 1;
      if ( !v57 && v10 == -1 )
        goto LABEL_10;
      v45 = dcgettext(0, "; giving up on this name", 5);
    }
    v46 = sub_5050(*a1);
    v47 = sub_B410(4, v46);
    v48 = dcgettext(0, "%s has been replaced with an untailable file%s", 5);
    error(0, 0, v48, v47, v45);
LABEL_10:
    v22 = sub_5050(*a1);
    sub_3B30(v14, v22);
    v23 = sub_5050(*a1);
    sub_3B30(*((int *)a1 + 14), v23);
    *((int *)a1 + 14) = -1;
    return v60 - __readfsqword(0x28u);
  }
  v25 = sub_5050(*a1);
  v26 = sub_4A40(v14, v25);
  *((char *)a1 + 53) = v26;
  if ( v26 && !byte_1510C )
  {
    *((int *)a1 + 15) = -1;
    v53 = sub_5050(*a1);
    v54 = sub_B410(4, v53);
    v55 = dcgettext(0, "%s has been replaced with an untailable remote file", 5);
    error(0, 0, v55, v54);
    *((short *)a1 + 26) = 257;
    goto LABEL_10;
  }
  *((int *)a1 + 15) = 0;
  v27 = *((int *)a1 + 14);
  if ( (v10 & 0xFFFFFFFD) != 0 )
  {
    if ( v27 != -1 )
      __assert_fail("f->fd == -1", "src/tail.c", 0x42Fu, "recheck");
    v28 = sub_5050(*a1);
    v29 = "%s has become accessible";
    v30 = sub_B410(4, v28);
  }
  else
  {
    v39 = (const char *)*a1;
    if ( v27 != -1 )
    {
      if ( a1[5] == buf.st_ino && a1[4] == buf.st_dev )
      {
        v40 = sub_5050(v39);
        sub_3B30(v14, v40);
        return v60 - __readfsqword(0x28u);
      }
      v41 = sub_5050(v39);
      v42 = sub_B410(4, v41);
      v43 = dcgettext(0, "%s has been replaced;  following new file", 5);
      error(0, 0, v43, v42);
      v44 = sub_5050(*a1);
      sub_3B30(*((int *)a1 + 14), v44);
      goto LABEL_24;
    }
    v56 = sub_5050(v39);
    v29 = "%s has appeared;  following new file";
    v30 = sub_B410(4, v56);
  }
  v31 = dcgettext(0, v29, 5);
  error(0, 0, v31, v30);
LABEL_24:
  v32 = a2;
  if ( !v11 )
    v32 = -1;
  sub_3830((long long)a1, v14, 0, (long long *)&buf, v32);
  v33 = sub_5050(*a1);
  sub_41C0(v14, 0, 0, v33);
  return v60 - __readfsqword(0x28u);
}



// Function: tail_forever_inotify @ 0x5880
void tail_forever_inotify(int fd, long long a2, size_t a3, long long *a4, double a5)
{
  int v7; // ebx
  int v8; // eax
  size_t v9; // r13
  long long v10; // rax
  long long *v11; // rbp
  long long *v12; // r12
  const char *v13; // r14
  size_t v14; // rax
  long long v15; // r15
  const char *v16; // rsi
  int v17; // eax
  char v18; // al
  int *v19; // r14
  size_t v20; // rbp
  long long v21; // r15
  const char *v22; // rsi
  char *v23; // rdx
  char *v24; // rax
  size_t v25; // rax
  char v26; // r13
  int *v27; // r12
  long long v28; // rcx
  long long v29; // rdx
  int v30; // r14d
  long long v31; // rdx
  const char *v32; // r12
  int v33; // ebx
  long long v34; // r14
  char *v35; // r12
  int v36; // eax
  char *v37; // rax
  int v38; // edx
  int v39; // eax
  long long v40; // rax
  int *v41; // rax
  int v42; // r14d
  int *v43; // r8
  char *v44; // rax
  int v45; // esi
  long long *v46; // rax
  long long *v47; // r14
  long long v48; // rax
  int *v49; // rax
  long long v50; // rax
  long long v51; // r13
  char *v52; // r12
  int *v53; // rax
  int *v54; // rbx
  long long v55; // r12
  char *v56; // rdx
  char *v57; // r12
  long long v58; // rsi
  double v59; // [rsp+8h] [rbp-1B0h]
  int v60; // [rsp+10h] [rbp-1A8h]
  long long v61; // [rsp+10h] [rbp-1A8h]
  unsigned long long v62; // [rsp+10h] [rbp-1A8h]
  long long v63; // [rsp+18h] [rbp-1A0h]
  char v64; // [rsp+20h] [rbp-198h]
  unsigned long long v65; // [rsp+20h] [rbp-198h]
  char v66; // [rsp+28h] [rbp-190h]
  int v67; // [rsp+28h] [rbp-190h]
  long long v68; // [rsp+28h] [rbp-190h]
  int *v70; // [rsp+38h] [rbp-180h]
  char fda; // [rsp+40h] [rbp-178h]
  int fdb; // [rsp+40h] [rbp-178h]
  int *fdc; // [rsp+40h] [rbp-178h]
  long long v74; // [rsp+48h] [rbp-170h]
  char *v75; // [rsp+48h] [rbp-170h]
  size_t nbytes; // [rsp+50h] [rbp-168h]
  size_t nbytesa; // [rsp+50h] [rbp-168h]
  uint32_t mask; // [rsp+58h] [rbp-160h]
  int v79; // [rsp+5Ch] [rbp-15Ch]
  int *v80; // [rsp+60h] [rbp-158h]
  long long v81; // [rsp+78h] [rbp-140h] BYREF
  char v82[68]; // [rsp+80h] [rbp-138h] BYREF
  int v83; // [rsp+C4h] [rbp-F4h]
  struct stat buf; // [rsp+E0h] [rbp-D8h] BYREF
  unsigned long long v85; // [rsp+178h] [rbp-40h]

  v7 = fd;
  v59 = a5;
  v85 = __readfsqword(0x28u);
  v63 = sub_8850(a3, 0, sub_3980, sub_39A0, 0);
  if ( !v63 )
LABEL_90:
    sub_C360();
  v64 = 0;
  fda = 0;
  *a4 = v63;
  nbytes = a3;
  v60 = dword_15020;
  v8 = 2;
  if ( dword_15020 == 1 )
    v8 = 3078;
  v9 = 0;
  mask = v8;
  v10 = 3 * a3;
  v11 = (long long *)a2;
  v74 = 32 * v10;
  v12 = (long long *)(32 * v10 + a2);
  do
  {
    while ( 1 )
    {
      if ( *((char *)v11 + 52) )
        goto LABEL_19;
      v13 = (const char *)*v11;
      v14 = strlen((const char *)*v11);
      *((int *)v11 + 17) = -1;
      if ( v9 < v14 )
        v9 = v14;
      if ( v60 == 1 )
      {
        v15 = sub_7BB0(v13);
        v66 = v13[v15];
        v16 = ".";
        v11[10] = sub_7990(v13) - (long long)v13;
        v13[v15] = 0;
        if ( v15 )
          v16 = (const char *)*v11;
        *((int *)v11 + 18) = inotify_add_watch(fd, v16, 0x784u);
        *(char *)(*v11 + v15) = v66;
        if ( *((int *)v11 + 18) < 0 )
        {
          v54 = __errno_location();
          if ( *v54 != 28 )
          {
            v55 = sub_B410(4, *v11);
            v56 = dcgettext(0, "cannot watch parent directory of %s", 5);
            error(0, *v54, v56, v55);
            return;
          }
LABEL_34:
          v22 = "inotify resources exhausted";
          goto LABEL_35;
        }
        v13 = (const char *)*v11;
      }
      v17 = inotify_add_watch(fd, v13, mask);
      *((int *)v11 + 17) = v17;
      if ( v17 >= 0 )
        break;
      v18 = v64;
      if ( *((int *)v11 + 14) != -1 )
        v18 = 1;
      v64 = v18;
      v19 = __errno_location();
      if ( (*v19 & 0xFFFFFFEF) == 0xC )
        goto LABEL_34;
      if ( *((int *)v11 + 15) != *v19 )
      {
        v61 = sub_B410(4, *v11);
        v24 = dcgettext(0, "cannot watch %s", 5);
        error(0, *v19, v24, v61);
        v60 = dword_15020;
      }
LABEL_19:
      v11 += 12;
      if ( v11 == v12 )
        goto LABEL_20;
    }
    if ( !sub_8E70(v63, v11) )
      goto LABEL_90;
    v11 += 12;
    fda = 1;
    v60 = dword_15020;
  }
  while ( v11 != v12 );
LABEL_20:
  v20 = nbytes;
  if ( v60 == 2 )
  {
    if ( v64 )
      return;
    if ( !fda )
      exit(1);
  }
  v21 = a2;
  v81 = a2 + v74 - 96;
  do
  {
    if ( !*(char *)(v21 + 52) )
    {
      if ( dword_15020 == 1 )
      {
        sub_5270((long long *)v21, 0);
      }
      else if ( *(int *)(v21 + 56) != -1
             && !stat(*(const char **)v21, &buf)
             && (*(long long *)(v21 + 32) != buf.st_dev || *(long long *)(v21 + 40) != buf.st_ino) )
      {
        v50 = sub_5050(*(long long *)v21);
        v51 = sub_B410(4, v50);
        v52 = dcgettext(0, "%s was replaced", 5);
        v53 = __errno_location();
        error(0, *v53, v52, v51);
        return;
      }
      sub_5090(v21, &v81);
    }
    v21 += 96;
  }
  while ( (long long *)v21 != v12 );
  v25 = v9 + 17;
  v26 = 0;
  nbytesa = v25;
  v65 = 0;
  v70 = (int *)sub_BFA0(v25);
  v62 = 0;
  v79 = 3;
  v80 = (int *)(a2 + 72 + v74);
LABEL_37:
  while ( 2 )
  {
    if ( dword_15020 == 1 )
      goto LABEL_54;
    while ( v62 >= v65 )
    {
      do
      {
        if ( pid )
        {
          if ( v26 )
            exit(0);
          if ( kill(pid, 0) )
            goto LABEL_57;
          goto LABEL_63;
        }
LABEL_66:
        v38 = -1;
LABEL_59:
        LODWORD(buf.st_dev) = v7;
        WORD2(buf.st_dev) = 1;
        buf.st_ino = 1;
        v39 = poll((struct pollfd *)&buf, (unsigned char)byte_15117 + 1LL, v38);
      }
      while ( !v39 );
      if ( v39 < 0 )
      {
        v57 = dcgettext(0, "error waiting for inotify and output events", 5);
        v58 = (unsigned int)*__errno_location();
        error(1, v58, v57);
        sub_6220(1, v58, a5);
        return;
      }
      if ( HIWORD(buf.st_ino) )
        sub_4BD0();
      v27 = v70;
      v40 = sub_B6E0(v7, v70, nbytesa);
      v65 = v40;
      if ( !v40 )
        goto LABEL_91;
      v28 = 16;
      if ( v40 != -1 )
        goto LABEL_40;
      if ( *__errno_location() != 22 )
        goto LABEL_77;
LABEL_91:
      if ( v79 )
      {
        --v79;
        nbytesa *= 2LL;
        v65 = 0;
        v70 = (int *)sub_BFF0(v70, nbytesa);
        v62 = 0;
        goto LABEL_37;
      }
LABEL_77:
      v35 = dcgettext(0, "error reading inotify event", 5);
      v41 = __errno_location();
      error(1, *v41, v35);
LABEL_78:
      v42 = inotify_add_watch(v7, *(const char **)v21, mask);
      if ( v42 < 0 )
      {
        v43 = __errno_location();
        if ( (*v43 & 0xFFFFFFEF) == 0xC )
          goto LABEL_34;
        fdc = v43;
        v68 = sub_B410(4, *(long long *)v21);
        v44 = dcgettext(0, "cannot watch %s", 5);
        error(0, *fdc, v44, v68);
      }
      v45 = *(int *)(v21 + 68);
      if ( v45 != v42 )
      {
        if ( v45 >= 0 )
        {
          inotify_rm_watch(v7, v45);
          sub_8ED0(v63, v21);
        }
LABEL_83:
        *(int *)(v21 + 68) = v42;
        if ( v42 == -1 )
          goto LABEL_37;
        v46 = (long long *)sub_8ED0(v63, v21);
        v47 = v46;
        if ( v46 && (long long *)v21 != v46 )
        {
          if ( dword_15020 == 1 )
            sub_5270(v46, 0);
          *((int *)v47 + 17) = -1;
          v48 = sub_5050(*v47);
          sub_3B30(*((int *)v47 + 14), v48);
        }
        if ( !sub_8E70(v63, v21) )
          goto LABEL_90;
        goto LABEL_48;
      }
      if ( v45 < 0 )
        goto LABEL_83;
LABEL_48:
      if ( dword_15020 == 1 )
        sub_5270((long long *)v21, 0);
      v36 = *((int *)v35 + 1);
      if ( (v36 & 0xE04) == 0 )
        goto LABEL_70;
LABEL_51:
      if ( (v36 & 0x400) != 0 )
      {
        inotify_rm_watch(v7, *(int *)(v21 + 68));
        sub_8ED0(v63, v21);
      }
      sub_5270((long long *)v21, 0);
      if ( dword_15020 == 1 )
      {
LABEL_54:
        if ( !byte_1511A && !sub_83F0(v63) )
        {
          v37 = dcgettext(0, "no files remaining", 5);
          error(1, 0, v37);
LABEL_57:
          if ( *__errno_location() != 1 )
          {
            v38 = 0;
            v26 = 1;
            goto LABEL_59;
          }
LABEL_63:
          v38 = 0;
          if ( v59 > 0.0 )
          {
            if ( v59 < 2147482.0 )
            {
              a5 = 1000.0 * v59;
              v38 = (int)(1000.0 * v59) + (1000.0 * v59 > (double)(int)(1000.0 * v59));
              goto LABEL_59;
            }
            goto LABEL_66;
          }
          goto LABEL_59;
        }
      }
    }
    v27 = (int *)((char *)v70 + v62);
    v28 = v62 + 16;
LABEL_40:
    v29 = (unsigned int)v27[3];
    v30 = *v27;
    v62 = v29 + v28;
    v67 = v27[1];
    if ( (v67 & 0x400) == 0 )
    {
      if ( !(int)v29 )
      {
LABEL_68:
        v83 = *v27;
        v21 = sub_85F0(v63, v82);
        if ( !v21 )
          continue;
        v36 = v27[1];
        if ( (v36 & 0xE04) == 0 )
        {
LABEL_70:
          sub_5090(v21, &v81);
          continue;
        }
        goto LABEL_51;
      }
      v31 = a2;
      v21 = 0;
LABEL_42:
      v75 = (char *)v27;
      fdb = v7;
      v32 = (const char *)(v27 + 4);
      v33 = v30;
      v34 = v31;
      while ( *(int *)(v34 + 72) != v33 || strcmp(v32, (const char *)(*(long long *)v34 + *(long long *)(v34 + 80))) )
      {
        ++v21;
        v34 += 96;
        if ( v20 == v21 )
        {
          v7 = fdb;
          goto LABEL_37;
        }
      }
      v35 = v75;
      v7 = fdb;
      if ( v20 == v21 )
        continue;
      v21 = a2 + 96 * v21;
      if ( (v67 & 0x200) != 0 )
        goto LABEL_48;
      goto LABEL_78;
    }
    break;
  }
  v31 = a2;
  v21 = 0;
  if ( v27[3] )
    goto LABEL_42;
  v49 = (int *)(a2 + 72);
  while ( *v49 != v30 )
  {
    v49 += 24;
    if ( v80 == v49 )
      goto LABEL_68;
  }
  v22 = "directory containing watched file was removed";
LABEL_35:
  v23 = dcgettext(0, v22, 5);
  error(0, 0, v23);
}



// Function: tail_forever @ 0x6220
void tail_forever(long long a1, int *a2, double a3)
{
  int *v3; // r14
  int v4; // r15d
  __pid_t v5; // r13d
  long long v6; // rbx
  int *v7; // rbp
  long long v8; // rax
  int v9; // edx
  int v10; // ecx
  int v11; // r8d
  int v12; // r9d
  char *v13; // r12
  __pid_t v14; // eax
  int v15; // eax
  int v16; // ecx
  int v17; // r8d
  int v18; // r9d
  int v19; // edx
  unsigned long long v20; // rcx
  long long v21; // rax
  long long *v22; // rdi
  unsigned int v23; // eax
  int *v24; // rax
  __mode_t st_mode; // r11d
  long long v26; // rax
  long long v27; // rdx
  int *v28; // rax
  long long v29; // rax
  long long v30; // rdx
  int v31; // eax
  unsigned long long v32; // rax
  bool v33; // cf
  char *v34; // rax
  const char *v35; // rax
  char *v36; // rdx
  bool v37; // cl
  char *v38; // rax
  char *v39; // r12
  int *v40; // rax
  char v41; // [rsp+0h] [rbp-118h]
  int *v42; // [rsp+8h] [rbp-110h]
  unsigned char v43; // [rsp+14h] [rbp-104h]
  int v44; // [rsp+14h] [rbp-104h]
  int *v45; // [rsp+18h] [rbp-100h]
  __mode_t v46; // [rsp+20h] [rbp-F8h]
  long long v47; // [rsp+20h] [rbp-F8h]
  __mode_t v49; // [rsp+30h] [rbp-E8h]
  bool v50; // [rsp+36h] [rbp-E2h]
  char v51; // [rsp+37h] [rbp-E1h]
  struct stat buf; // [rsp+40h] [rbp-D8h] BYREF
  unsigned long long v53; // [rsp+D8h] [rbp-40h]

  v5 = pid;
  v42 = a2;
  v53 = __readfsqword(0x28u);
  if ( pid )
  {
    v50 = 0;
    v5 = 0;
  }
  else
  {
    v50 = a2 == (int *)((char *)&dword_0 + 1) && dword_15020 == 2;
    if ( v50 )
    {
      if ( *(int *)(a1 + 56) == -1 )
        goto LABEL_63;
      v5 = 0;
      v37 = a2 == (int *)((char *)&dword_0 + 1) && dword_15020 == 2;
      v44 = *(int *)(a1 + 48);
      if ( (v44 & 0xF000) == 0x8000 )
        v37 = 0;
      LOBYTE(v5) = (v44 & 0xF000) != 0x8000;
      v50 = v37;
    }
  }
LABEL_3:
  v51 = 0;
  v45 = (int *)((char *)v42 - 1);
  while ( 2 )
  {
    v43 = 0;
    v6 = a1;
    v7 = 0;
    do
    {
      while ( 1 )
      {
        if ( *(char *)(v6 + 52) )
          goto LABEL_13;
        v4 = *(int *)(v6 + 56);
        if ( v4 >= 0 )
          break;
        v22 = (long long *)v6;
        v7 = (int *)((char *)v7 + 1);
        v6 += 96;
        sub_5270(v22, v5);
        if ( v42 == v7 )
          goto LABEL_17;
      }
      v8 = sub_5050(*(long long *)v6);
      LODWORD(v3) = *(int *)(v6 + 48);
      v13 = (char *)v8;
      v14 = v5;
      if ( *(int *)(v6 + 64) != v5 )
      {
        v15 = sub_7D50(v4, 3, v9, v10, v11, v12, v41);
        if ( v5 )
        {
          if ( v15 < 0 )
            goto LABEL_36;
        }
        else if ( v15 < 0
               || (v19 = v15, BYTE1(v19) = BYTE1(v15) | 8, v15 != v19)
               && (unsigned int)sub_7D50(v4, 4, v19, v16, v17, v18, v41) == -1 )
        {
LABEL_36:
          v28 = __errno_location();
          if ( (*(int *)(v6 + 48) & 0xF000) != 0x8000 || *v28 != 1 )
          {
            v42 = v28;
            v13 = (char *)sub_B510(0, 3, v13);
            v38 = dcgettext(0, "%s: cannot change nonblocking mode", 5);
            error(1, *v42, v38, v13);
            goto LABEL_61;
          }
          v14 = *(int *)(v6 + 64);
          goto LABEL_11;
        }
        *(int *)(v6 + 64) = v5;
        v14 = v5;
      }
LABEL_11:
      v20 = -2;
      if ( v14 )
        goto LABEL_12;
LABEL_25:
      if ( fstat(v4, &buf) )
      {
        *(int *)(v6 + 56) = -1;
        v3 = __errno_location();
        *(int *)(v6 + 60) = *v3;
        v35 = (const char *)sub_B510(0, 3, v13);
        error(0, *v3, "%s", v35);
        close(v4);
      }
      else
      {
        st_mode = buf.st_mode;
        if ( *(int *)(v6 + 48) != buf.st_mode
          || (buf.st_mode & 0xF000) == 0x8000 && *(long long *)(v6 + 8) != buf.st_size
          || (v49 = buf.st_mode,
              v29 = sub_B7B0(&buf),
              v31 = sub_B800(*(long long *)(v6 + 16), *(long long *)(v6 + 24), v29, v30),
              st_mode = v49,
              v31) )
        {
          v46 = st_mode;
          LODWORD(v3) = (unsigned short)v3 & 0xF000;
          v26 = sub_B7B0(&buf);
          *(long long *)(v6 + 88) = 0;
          *(long long *)(v6 + 16) = v26;
          *(long long *)(v6 + 24) = v27;
          *(int *)(v6 + 48) = v46;
          if ( (int)v3 == 0x8000 && buf.st_size < *(long long *)(v6 + 8) )
          {
            v47 = sub_B510(0, 3, v13);
            v34 = dcgettext(0, "%s: file truncated", 5);
            error(0, 0, v34, v47);
            sub_41C0(v4, 0, 0, (long long)v13);
            *(long long *)(v6 + 8) = 0;
          }
          if ( v45 != v7 && byte_15115 )
            sub_3A10((long long)v13);
          v45 = v7;
          if ( !*(int *)(v6 + 64) )
          {
            v20 = -1;
            if ( (int)v3 == 0x8000 )
            {
              if ( *(char *)(v6 + 53) )
                v20 = buf.st_size - *(long long *)(v6 + 8);
            }
            goto LABEL_12;
          }
LABEL_61:
          v20 = -2;
LABEL_12:
          v21 = sub_3F80(0, (long long)v13, v4, v20);
          *(long long *)(v6 + 8) += v21;
          v43 |= v21 != 0;
          goto LABEL_13;
        }
        v32 = *(long long *)(v6 + 88);
        v33 = v32 < qword_15018;
        *(long long *)(v6 + 88) = v32 + 1;
        if ( !v33 && dword_15020 == 1 )
        {
          sub_5270((long long *)v6, *(int *)(v6 + 64) != 0);
          *(long long *)(v6 + 88) = 0;
        }
      }
LABEL_13:
      v7 = (int *)((char *)v7 + 1);
      v6 += 96;
    }
    while ( v42 != v7 );
LABEL_17:
    if ( sub_3880(a1, (long long)v42) )
    {
      v23 = v43 ^ 1;
      LOBYTE(v23) = v50 | v43 ^ 1;
      v6 = v23;
      if ( !(char)v23 )
      {
        sub_4BF0();
        continue;
      }
      if ( fflush_unlocked(stdout) )
      {
        v39 = dcgettext(0, "write error", 5);
        v40 = __errno_location();
        error(1, *v40, v39);
LABEL_63:
        v50 = 0;
        goto LABEL_3;
      }
      sub_4BF0();
      if ( v43 )
        continue;
      if ( v51 )
        return;
      if ( pid && kill(pid, 0) && *__errno_location() != 1 )
      {
        v51 = v6;
        continue;
      }
      if ( !(unsigned int)sub_C4E0(a3) )
        continue;
      v13 = dcgettext(0, "cannot read realtime clock", 5);
      v24 = __errno_location();
      error(1, *v24, v13);
      goto LABEL_25;
    }
    break;
  }
  v36 = dcgettext(0, "no files remaining", 5);
  error(0, 0, v36);
}



// Function: tail @ 0x6a90
long long tail(long long a1, long long a2, long long a3, long long *a4)
{
  bool v4; // zf

  v4 = byte_15119 == 0;
  *a4 = 0;
  if ( v4 )
    return sub_6820();
  else
    return sub_4C60();
}



// Function: tail_file @ 0x6ab0
long long tail_file(const char **a1, long long a2)
{
  const char *v3; // r12
  int v4; // edx
  int v5; // ecx
  int v6; // ebp
  int v7; // r8d
  int v8; // r9d
  unsigned int v9; // r14d
  long long v10; // rax
  unsigned int v11; // eax
  unsigned int v12; // r12d
  long long v14; // rax
  long long v15; // rax
  unsigned int v16; // eax
  char v17; // r15
  int *v18; // rax
  int *v19; // rbp
  int v20; // eax
  long long v21; // rax
  long long v22; // r12
  char *v23; // rax
  long long v24; // rcx
  int *v25; // rax
  const char *v26; // rdi
  int *v27; // rbp
  long long v28; // rax
  long long v29; // r12
  char *v30; // rax
  const char *v31; // rdi
  long long v32; // rax
  long long v33; // rax
  long long v34; // r13
  char *v35; // r12
  int *v36; // rax
  const char *v37; // rdx
  char v38; // al
  char *v39; // rbp
  long long v40; // rax
  long long v41; // r12
  char *v42; // rax
  long long v43; // [rsp+8h] [rbp-E0h] BYREF
  struct stat buf; // [rsp+10h] [rbp-D8h] BYREF
  unsigned long long v45; // [rsp+A8h] [rbp-40h]

  v3 = *a1;
  v45 = __readfsqword(0x28u);
  v6 = strcmp(v3, "-");
  if ( v6 )
  {
    v16 = sub_7EA0((int)v3, 0, v4, v5, v7, v8);
    v17 = byte_1511A;
    v9 = v16;
    if ( byte_1511A )
    {
      v3 = *a1;
      if ( v16 != -1 )
        goto LABEL_3;
      *((char *)a1 + 54) = 0;
    }
    else
    {
      *((char *)a1 + 54) = 1;
      v3 = *a1;
      if ( v16 != -1 )
        goto LABEL_4;
    }
    v18 = __errno_location();
    v19 = v18;
    if ( byte_15118 )
    {
      *((int *)a1 + 14) = -1;
      v20 = *v18;
      *((char *)a1 + 52) = v17 ^ 1;
      *((int *)a1 + 15) = v20;
      a1[5] = 0;
      a1[4] = 0;
    }
    v21 = sub_5050(*a1);
    v22 = sub_B410(4, v21);
    v23 = dcgettext(0, "cannot open %s for reading", 5);
    v24 = v22;
    v12 = 0;
    error(0, *v19, v23, v24);
    return v12;
  }
  byte_1510E = 1;
  v9 = 0;
LABEL_3:
  *((char *)a1 + 54) = 1;
LABEL_4:
  if ( byte_15115 )
  {
    v15 = sub_5050(v3);
    sub_3A10(v15);
    v3 = *a1;
  }
  v10 = sub_5050(v3);
  v11 = sub_6A90(v10, v9, a2, &v43);
  v12 = v11;
  if ( byte_15118 )
  {
    *((int *)a1 + 15) = (unsigned char)v11 - 1;
    if ( fstat(v9, &buf) < 0 )
    {
      v25 = __errno_location();
      v26 = *a1;
      v27 = v25;
      *((int *)a1 + 15) = *v25;
      v28 = sub_5050(v26);
      v29 = sub_B410(4, v28);
      v30 = dcgettext(0, "error reading %s", 5);
      error(0, *v27, v30, v29);
    }
    else if ( (((buf.st_mode & 0xF000) - 4096) & 0xFFFFE000) != 0 && (buf.st_mode & 0xB000) != 0x8000 )
    {
      v38 = byte_1511A;
      *((int *)a1 + 15) = -1;
      v39 = (char *)"";
      *((char *)a1 + 54) = 0;
      v38 ^= 1u;
      *((char *)a1 + 52) = v38;
      if ( v38 )
        v39 = dcgettext(0, "; giving up on this name", 5);
      v40 = sub_5050(*a1);
      v41 = sub_B510(0, 3, v40);
      v42 = dcgettext(0, "%s: cannot follow end of this type of file%s", 5);
      error(0, 0, v42, v41, v39);
    }
    else if ( (char)v12 )
    {
      sub_3830((long long)a1, v9, v43, (long long *)&buf, v6 == 0 ? -1 : 1);
      v14 = sub_5050(*a1);
      *((char *)a1 + 53) = sub_4A40(v9, v14);
      return v12;
    }
    v31 = *a1;
    v12 = 0;
    *((char *)a1 + 52) = byte_1511A ^ 1;
    v32 = sub_5050(v31);
    sub_3B30(v9, v32);
    *((int *)a1 + 14) = -1;
    return v12;
  }
  if ( v6 && close(v9) )
  {
    v33 = sub_5050(*a1);
    v34 = sub_B410(4, v33);
    v35 = dcgettext(0, "error reading %s", 5);
    v36 = __errno_location();
    v37 = v35;
    v12 = 0;
    error(0, *v36, v37, v34);
  }
  return v12;
}



// Function: usage @ 0x6e60
void usage(int status)
{
  long long v2; // r12
  char *v3; // rax
  char *v4; // rax
  long long v5; // rcx
  char *v6; // rax
  long long v7; // rcx
  FILE *v8; // r12
  char *v9; // rax
  FILE *v10; // r12
  char *v11; // rax
  char *v12; // rax
  FILE *v13; // r12
  char *v14; // rax
  FILE *v15; // r12
  char *v16; // rax
  FILE *v17; // r12
  char *v18; // rax
  FILE *v19; // r12
  char *v20; // rax
  FILE *v21; // r12
  char *v22; // rax
  FILE *v23; // r12
  char *v24; // rax
  FILE *v25; // rsi
  char *v26; // rdi

  v2 = qword_15138;
  if ( status )
  {
    v3 = dcgettext(0, "Try '%s --help' for more information.\n", 5);
    __fprintf_chk(stderr, 1, v3, v2);
  }
  else
  {
    v4 = dcgettext(0, "Usage: %s [OPTION]... [FILE]...\n", 5);
    __printf_chk(1, v4, v2, v5);
    v6 = dcgettext(
           0,
           "Print the last %d lines of each FILE to standard output.\n"
           "With more than one FILE, precede each with a header giving the file name.\n",
           5);
    __printf_chk(1, v6, 10, v7);
    sub_39B0();
    sub_39E0();
    v8 = stdout;
    v9 = dcgettext(
           0,
           "  -c, --bytes=[+]NUM       output the last NUM bytes; or use -c +NUM to\n"
           "                             output starting with byte NUM of each file\n",
           5);
    fputs_unlocked(v9, v8);
    v10 = stdout;
    v11 = dcgettext(
            0,
            "  -f, --follow[={name|descriptor}]\n"
            "                           output appended data as the file grows;\n"
            "                             an absent option argument means 'descriptor'\n"
            "  -F                       same as --follow=name --retry\n",
            5);
    fputs_unlocked(v11, v10);
    v12 = dcgettext(
            0,
            "  -n, --lines=[+]NUM       output the last NUM lines, instead of the last %d;\n"
            "                             or use -n +NUM to output starting with line NUM\n"
            "      --max-unchanged-stats=N\n"
            "                           with --follow=name, reopen a FILE which has not\n"
            "                             changed size after N (default %d) iterations\n"
            "                             to see if it has been unlinked or renamed\n"
            "                             (this is the usual case of rotated log files);\n"
            "                             with inotify, this option is rarely useful\n",
            5);
    __printf_chk(1, v12, 10, 5);
    v13 = stdout;
    v14 = dcgettext(
            0,
            "      --pid=PID            with -f, terminate after process ID, PID dies\n"
            "  -q, --quiet, --silent    never output headers giving file names\n"
            "      --retry              keep trying to open a file if it is inaccessible\n",
            5);
    fputs_unlocked(v14, v13);
    v15 = stdout;
    v16 = dcgettext(
            0,
            "  -s, --sleep-interval=N   with -f, sleep for approximately N seconds\n"
            "                             (default 1.0) between iterations;\n"
            "                             with inotify and --pid=P, check process P at\n"
            "                             least once every N seconds\n"
            "  -v, --verbose            always output headers giving file names\n",
            5);
    fputs_unlocked(v16, v15);
    v17 = stdout;
    v18 = dcgettext(0, "  -z, --zero-terminated    line delimiter is NUL, not newline\n", 5);
    fputs_unlocked(v18, v17);
    v19 = stdout;
    v20 = dcgettext(0, "      --help        display this help and exit\n", 5);
    fputs_unlocked(v20, v19);
    v21 = stdout;
    v22 = dcgettext(0, "      --version     output version information and exit\n", 5);
    fputs_unlocked(v22, v21);
    v23 = stdout;
    v24 = dcgettext(
            0,
            "\n"
            "NUM may have a multiplier suffix:\n"
            "b 512, kB 1000, K 1024, MB 1000*1000, M 1024*1024,\n"
            "GB 1000*1000*1000, G 1024*1024*1024, and so on for T, P, E, Z, Y.\n"
            "Binary prefixes can be used, too: KiB=K, MiB=M, and so on.\n"
            "\n",
            5);
    fputs_unlocked(v24, v23);
    v25 = stdout;
    v26 = dcgettext(
            0,
            "With --follow (-f), tail defaults to following the file descriptor, which\n"
            "means that even if a tail'ed file is renamed, tail will continue to track\n"
            "its end.  This default behavior is not desirable when you really want to\n"
            "track the actual name of the file, not the file descriptor (e.g., log\n"
            "rotation).  Use --follow=name in that case.  That causes tail to track the\n"
            "named file in a way that accommodates renaming, removal and creation.\n",
            5);
    fputs_unlocked(v26, v25);
    sub_4E20();
  }
  exit(status);
}



// Function: is_prime @ 0x7f20
bool is_prime(unsigned long long a1)
{
  unsigned long long v1; // rdx
  long long v2; // r8
  unsigned long long v3; // rsi
  unsigned long long v4; // rcx

  v1 = a1 - (a1 / 3 + (((0xAAAAAAAAAAAAAAABLL * (unsigned __int128)a1) >> 64) & 0xFFFFFFFFFFFFFFFELL));
  if ( a1 > 9 && v1 )
  {
    v2 = 16;
    v3 = 9;
    v4 = 3;
    do
    {
      v4 += 2LL;
      v3 += v2;
      v1 = a1 % v4;
      if ( a1 <= v3 )
        break;
      v2 += 8;
    }
    while ( v1 );
  }
  return v1 != 0;
}



// Function: next_prime @ 0x7f90
unsigned long long next_prime(unsigned long long a1)
{
  long long v1; // rax
  unsigned long long v2; // r9

  v1 = 10;
  if ( a1 >= 0xA )
    v1 = a1;
  v2 = v1 | 1;
  if ( (v1 | 1) != 0xFFFFFFFFFFFFFFFFLL )
  {
    do
    {
      if ( sub_7F20(v2) )
        break;
      v2 += 2LL;
    }
    while ( v2 != -1 );
  }
  return v2;
}



// Function: raw_hasher @ 0x7fd0
unsigned long long raw_hasher(long long a1, unsigned long long a2)
{
  return sub_CAF0(a1, 3) % a2;
}



// Function: raw_comparator @ 0x7ff0
bool raw_comparator(long long a1, long long a2)
{
  return a2 == a1;
}



// Function: check_tuning @ 0x8000
long long check_tuning(long long a1)
{
  long long v1; // rax
  float v2; // xmm0_4
  float v3; // xmm1_4
  float v4; // xmm2_4
  long long result; // rax

  v1 = *(long long *)(a1 + 40);
  if ( (_UNKNOWN *)v1 == &unk_F5D0 )
    return 1;
  v2 = *(float *)(v1 + 8);
  if ( v2 <= 0.1
    || v2 >= 0.89999998
    || *(float *)(v1 + 12) <= 1.1
    || *(float *)v1 < 0.0
    || (v3 = *(float *)v1 + 0.1, v4 = *(float *)(v1 + 4), v4 <= v3)
    || v4 > 1.0
    || (result = 1, v2 <= v3) )
  {
    *(long long *)(a1 + 40) = &unk_F5D0;
    return 0;
  }
  return result;
}



// Function: free_entry @ 0x8090
long long free_entry(long long a1, long long *a2)
{
  long long result; // rax

  result = *(long long *)(a1 + 72);
  *a2 = 0;
  a2[1] = result;
  *(long long *)(a1 + 72) = a2;
  return result;
}



// Function: safe_hasher @ 0x80b0
long long safe_hasher(long long a1, long long a2)
{
  unsigned long long v2; // rax

  v2 = (*(long long (**)(long long, long long))(a1 + 48))(a2, *(long long *)(a1 + 16));
  if ( *(long long *)(a1 + 16) <= v2 )
    abort();
  return *(long long *)a1 + 16 * v2;
}



// Function: hash_find_entry @ 0x80e0
long long hash_find_entry(long long a1, long long a2, __m128i **a3, char a4)
{
  __m128i *v6; // rax
  __m128i *v7; // rbx
  long long result; // rax
  __m128i *v9; // rsi
  long long *v10; // rax
  long long *v11; // rsi
  long long v12; // [rsp+8h] [rbp-30h]
  long long v13; // [rsp+8h] [rbp-30h]

  v6 = (__m128i *)sub_80B0(a1, a2);
  *a3 = v6;
  if ( !v6->m128i_i64[0] )
    return 0;
  v7 = v6;
  if ( v6->m128i_i64[0] != a2 )
  {
    if ( (*(unsigned char (**)(long long))(a1 + 56))(a2) )
    {
      result = v7->m128i_i64[0];
      goto LABEL_5;
    }
    while ( 1 )
    {
      v10 = (long long *)v7->m128i_i64[1];
      if ( !v10 )
        break;
      if ( *v10 == a2 || (*(unsigned char (**)(long long))(a1 + 56))(a2) )
      {
        v11 = (long long *)v7->m128i_i64[1];
        result = *v11;
        if ( a4 )
        {
          v13 = *v11;
          v7->m128i_i64[1] = v11[1];
          sub_8090(a1, v11);
          return v13;
        }
        return result;
      }
      v7 = (__m128i *)v7->m128i_i64[1];
    }
    return 0;
  }
  result = v6->m128i_i64[0];
LABEL_5:
  if ( a4 )
  {
    v9 = (__m128i *)v7->m128i_i64[1];
    if ( v9 )
    {
      v12 = result;
      *v7 = _mm_loadu_si128(v9);
      sub_8090(a1, v9);
      return v12;
    }
    else
    {
      v7->m128i_i64[0] = 0;
    }
  }
  return result;
}



// Function: allocate_entry @ 0x81e0
long long *allocate_entry(long long a1)
{
  long long *result; // rax

  result = *(long long **)(a1 + 72);
  if ( !result )
    return malloc(0x10u);
  *(long long *)(a1 + 72) = result[1];
  return result;
}



// Function: transfer_entries @ 0x8210
long long transfer_entries(long long a1, long long a2, char a3)
{
  long long *v5; // rbp
  long long v6; // r15
  long long *v7; // r14
  long long v8; // r15
  long long *v9; // rax
  long long *v10; // rsi
  long long *v11; // rax
  long long *v12; // r14
  long long *v13; // rax
  long long v14; // rdx

  v5 = *(long long **)a2;
  if ( *(long long *)a2 >= *(long long *)(a2 + 8) )
    return 1;
  while ( 1 )
  {
    while ( 1 )
    {
      v6 = *v5;
      if ( *v5 )
      {
        v7 = (long long *)v5[1];
        if ( v7 )
        {
          do
          {
            while ( 1 )
            {
              v8 = *v7;
              v9 = (long long *)sub_80B0(a1, *v7);
              v10 = v7;
              v7 = (long long *)v7[1];
              if ( !*v9 )
                break;
              v10[1] = v9[1];
              v9[1] = v10;
              if ( !v7 )
                goto LABEL_10;
            }
            *v9 = v8;
            ++*(long long *)(a1 + 24);
            sub_8090(a1, v10);
          }
          while ( v7 );
LABEL_10:
          v6 = *v5;
        }
        v5[1] = 0;
        if ( !a3 )
          break;
      }
      v5 += 2;
      if ( *(long long *)(a2 + 8) <= (unsigned long long)v5 )
        return 1;
    }
    v11 = (long long *)sub_80B0(a1, v6);
    v12 = v11;
    if ( !*v11 )
    {
      *v11 = v6;
      ++*(long long *)(a1 + 24);
      goto LABEL_15;
    }
    v13 = sub_81E0(a1);
    if ( !v13 )
      return 0;
    v14 = v12[1];
    *v13 = v6;
    v13[1] = v14;
    v12[1] = v13;
LABEL_15:
    *v5 = 0;
    v5 += 2;
    --*(long long *)(a2 + 24);
    if ( *(long long *)(a2 + 8) <= (unsigned long long)v5 )
      return 1;
  }
}



// Function: hash_get_n_buckets @ 0x83d0
long long hash_get_n_buckets(long long a1)
{
  return *(long long *)(a1 + 16);
}



// Function: hash_get_n_buckets_used @ 0x83e0
long long hash_get_n_buckets_used(long long a1)
{
  return *(long long *)(a1 + 24);
}



// Function: hash_get_n_entries @ 0x83f0
long long hash_get_n_entries(long long a1)
{
  return *(long long *)(a1 + 32);
}



// Function: hash_get_max_bucket_length @ 0x8400
unsigned long long hash_get_max_bucket_length(long long a1)
{
  long long *v1; // rcx
  unsigned long long v2; // rsi
  unsigned long long v3; // r8
  long long v4; // rax
  unsigned long long i; // rdx

  v1 = *(long long **)a1;
  v2 = *(long long *)(a1 + 8);
  v3 = 0;
  if ( *(long long *)a1 < v2 )
  {
    do
    {
      while ( !*v1 )
      {
        v1 += 2;
        if ( (unsigned long long)v1 >= v2 )
          return v3;
      }
      v4 = v1[1];
      for ( i = 1; v4; ++i )
        v4 = *(long long *)(v4 + 8);
      if ( v3 < i )
        v3 = i;
      v1 += 2;
    }
    while ( (unsigned long long)v1 < v2 );
  }
  return v3;
}



// Function: hash_print_statistics @ 0x84d0
long long hash_print_statistics(long long a1, long long a2)
{
  long long v2; // rbx
  long long v3; // r13
  unsigned long long v4; // r12
  long long v5; // r9
  double v6; // xmm0_8
  double v7; // xmm1_8

  sub_83F0(a1);
  v2 = sub_83D0(a1);
  v3 = sub_83E0(a1);
  v4 = sub_8400(a1);
  __fprintf_chk(a2, 1, "# entries:         %lu\n", v5);
  __fprintf_chk(a2, 1, "# buckets:         %lu\n", v2);
  if ( v3 < 0 )
  {
    v6 = ((double)(int)(v3 & 1 | ((unsigned long long)v3 >> 1)) + (double)(int)(v3 & 1 | ((unsigned long long)v3 >> 1)))
       * 100.0;
    if ( v2 >= 0 )
      goto LABEL_3;
LABEL_6:
    v7 = (double)(int)(v2 & 1 | ((unsigned long long)v2 >> 1)) + (double)(int)(v2 & 1 | ((unsigned long long)v2 >> 1));
    goto LABEL_4;
  }
  v6 = (double)(int)v3 * 100.0;
  if ( v2 < 0 )
    goto LABEL_6;
LABEL_3:
  v7 = (double)(int)v2;
LABEL_4:
  __fprintf_chk(a2, 1, "# buckets used:    %lu (%.2f%%)\n", v3, v6 / v7);
  return __fprintf_chk(a2, 1, "max bucket length: %lu\n", v4);
}



// Function: hash_lookup @ 0x85f0
long long hash_lookup(long long a1, long long a2)
{
  long long v2; // rbp
  long long *v3; // rax
  long long v4; // rsi
  long long *v5; // rbx

  v2 = a2;
  v3 = (long long *)sub_80B0(a1, a2);
  v4 = *v3;
  if ( !*v3 )
    return 0;
  v5 = v3;
  while ( v4 != v2 )
  {
    if ( (*(unsigned char (**)(long long))(a1 + 56))(v2) )
      return *v5;
    v5 = (long long *)v5[1];
    if ( !v5 )
      return 0;
    v4 = *v5;
  }
  return v2;
}



// Function: hash_get_next @ 0x86a0
long long hash_get_next(long long a1, long long a2)
{
  long long *v2; // rax
  long long *v3; // rdx
  long long v4; // rcx
  long long v5; // r8

  v2 = (long long *)sub_80B0(a1, a2);
  v3 = v2;
  while ( 1 )
  {
    v4 = *v3;
    v3 = (long long *)v3[1];
    if ( v4 == a2 )
      break;
    if ( !v3 )
      goto LABEL_7;
  }
  if ( v3 )
    return *v3;
LABEL_7:
  while ( 1 )
  {
    v2 += 2;
    if ( *(long long *)(a1 + 8) <= (unsigned long long)v2 )
      break;
    v5 = *v2;
    if ( *v2 )
      return v5;
  }
  return 0;
}



// Function: hash_do_for_each @ 0x8760
long long hash_do_for_each(long long a1, unsigned char (*a2)(long long, long long), long long a3)
{
  long long *v3; // r14
  long long v6; // r12
  long long v7; // rdi
  long long *v9; // rbx

  v3 = *(long long **)a1;
  if ( *(long long *)a1 >= *(long long *)(a1 + 8) )
    return 0;
  v6 = 0;
  while ( 1 )
  {
    v7 = *v3;
    if ( *v3 )
      break;
LABEL_4:
    v3 += 2;
    if ( *(long long *)(a1 + 8) <= (unsigned long long)v3 )
      return v6;
  }
  v9 = v3;
  while ( a2(v7, a3) )
  {
    v9 = (long long *)v9[1];
    ++v6;
    if ( !v9 )
      goto LABEL_4;
    v7 = *v9;
  }
  return v6;
}



// Function: hash_initialize @ 0x8850
long long *hash_initialize(
        long long a1,
        float *a2,
        unsigned long long (*a3)(long long a1, unsigned long long a2),
        bool (*a4)(long long a1, long long a2),
        long long a5)
{
  float *v5; // r15
  unsigned long long (*v7)(long long, unsigned long long); // rbp
  bool (*v8)(long long, long long); // rbx
  long long *v9; // r12
  size_t v10; // rax
  char *v11; // rax
  long long v12; // rdx
  void *v14; // rdi

  v5 = a2;
  v7 = a3;
  v8 = a4;
  if ( !a3 )
    v7 = sub_7FD0;
  if ( !a4 )
    v8 = sub_7FF0;
  v9 = malloc(0x50u);
  if ( v9 )
  {
    if ( !a2 )
      v5 = (float *)&unk_F5D0;
    v9[5] = v5;
    if ( (unsigned char)sub_8000((long long)v9)
      && (v10 = sub_8340(a1, *((unsigned char *)v5 + 16), v5[2]), (v9[2] = v10) != 0)
      && (v11 = (char *)calloc(v10, 0x10u), (*v9 = v11) != 0) )
    {
      v12 = v9[2];
      v9[6] = v7;
      v9[3] = 0;
      v9[7] = v8;
      v9[8] = a5;
      v9[1] = &v11[16 * v12];
      v9[4] = 0;
      v9[9] = 0;
    }
    else
    {
      v14 = v9;
      v9 = 0;
      free(v14);
    }
  }
  return v9;
}



// Function: hash_clear @ 0x8960
void hash_clear(long long *a1)
{
  long long *v1; // r12
  long long *v2; // rbx
  void (*v3)(long long); // rdx
  long long *v4; // rax
  long long v5; // rcx

  v1 = (long long *)*a1;
  if ( *a1 < a1[1] )
  {
    do
    {
      while ( !*v1 )
      {
        v1 += 2;
        if ( a1[1] <= (unsigned long long)v1 )
          goto LABEL_14;
      }
      v2 = (long long *)v1[1];
      v3 = (void (*)(long long))a1[8];
      if ( v2 )
      {
        while ( 1 )
        {
          if ( v3 )
          {
            v3(*v2);
            v3 = (void (*)(long long))a1[8];
          }
          v4 = (long long *)v2[1];
          v5 = a1[9];
          *v2 = 0;
          v2[1] = v5;
          a1[9] = v2;
          if ( !v4 )
            break;
          v2 = v4;
        }
      }
      if ( v3 )
        v3(*v1);
      *v1 = 0;
      v1 += 2;
      *(v1 - 1) = 0;
    }
    while ( a1[1] > (unsigned long long)v1 );
  }
LABEL_14:
  a1[3] = 0;
  a1[4] = 0;
}



// Function: hash_free @ 0x8a10
void hash_free(long long *ptr)
{
  long long *v2; // r13
  unsigned long long v3; // rax
  long long *v4; // rbp
  long long *v5; // rbx
  long long *v6; // rbx
  void *v7; // rdi
  long long *v8; // rbx
  void *v9; // rdi

  v2 = (long long *)*ptr;
  v3 = ptr[1];
  v4 = (long long *)*ptr;
  if ( ptr[8] && ptr[4] )
  {
    if ( (unsigned long long)v2 >= v3 )
      goto LABEL_15;
    do
    {
      while ( !*v2 )
      {
        v2 += 2;
        if ( v3 <= (unsigned long long)v2 )
          goto LABEL_10;
      }
      v5 = v2;
      do
      {
        ((void (*)(void))ptr[8])();
        v5 = (long long *)v5[1];
      }
      while ( v5 );
      v3 = ptr[1];
      v2 += 2;
    }
    while ( v3 > (unsigned long long)v2 );
LABEL_10:
    v4 = (long long *)*ptr;
  }
  if ( v3 > (unsigned long long)v4 )
  {
    do
    {
      v6 = (long long *)v4[1];
      while ( v6 )
      {
        v7 = v6;
        v6 = (long long *)v6[1];
        free(v7);
      }
      v4 += 2;
    }
    while ( ptr[1] > (unsigned long long)v4 );
  }
LABEL_15:
  v8 = (long long *)ptr[9];
  while ( v8 )
  {
    v9 = v8;
    v8 = (long long *)v8[1];
    free(v9);
  }
  free((void *)*ptr);
  free(ptr);
}



// Function: hash_rehash @ 0x8b00
long long hash_rehash(long long a1, long long a2)
{
  size_t v2; // rax
  size_t v3; // rbx
  char *v4; // rax
  long long v5; // rax
  unsigned int v6; // r12d
  void *ptr; // [rsp+0h] [rbp-88h] BYREF
  char *v9; // [rsp+8h] [rbp-80h]
  size_t v10; // [rsp+10h] [rbp-78h]
  long long v11; // [rsp+18h] [rbp-70h]
  long long v12; // [rsp+20h] [rbp-68h]
  long long v13; // [rsp+28h] [rbp-60h]
  long long v14; // [rsp+30h] [rbp-58h]
  long long v15; // [rsp+38h] [rbp-50h]
  long long v16; // [rsp+40h] [rbp-48h]
  long long v17; // [rsp+48h] [rbp-40h]
  unsigned long long v18; // [rsp+58h] [rbp-30h]

  v18 = __readfsqword(0x28u);
  v2 = sub_8340(a2, *(unsigned char *)(*(long long *)(a1 + 40) + 16LL), *(float *)(*(long long *)(a1 + 40) + 8LL));
  if ( !v2 )
    return 0;
  v3 = v2;
  if ( *(long long *)(a1 + 16) == v2 )
    return 1;
  v4 = (char *)calloc(v2, 0x10u);
  ptr = v4;
  if ( !v4 )
    return 0;
  v10 = v3;
  v11 = 0;
  v9 = &v4[16 * v3];
  v5 = *(long long *)(a1 + 40);
  v12 = 0;
  v13 = v5;
  v14 = *(long long *)(a1 + 48);
  v15 = *(long long *)(a1 + 56);
  v16 = *(long long *)(a1 + 64);
  v17 = *(long long *)(a1 + 72);
  v6 = sub_8210((long long)&ptr, a1, 0);
  if ( (char)v6 )
  {
    free(*(void **)a1);
    *(long long *)a1 = ptr;
    *(long long *)(a1 + 8) = v9;
    *(long long *)(a1 + 16) = v10;
    *(long long *)(a1 + 24) = v11;
    *(long long *)(a1 + 72) = v17;
  }
  else
  {
    *(long long *)(a1 + 72) = v17;
    if ( !(unsigned char)sub_8210(a1, (long long)&ptr, 1) || !(unsigned char)sub_8210(a1, (long long)&ptr, 0) )
      abort();
    free(ptr);
  }
  return v6;
}



// Function: hash_insert_if_absent @ 0x8c80
long long hash_insert_if_absent(long long *a1, long long a2, long long *a3)
{
  long long v5; // rax
  unsigned int v6; // r8d
  long long v8; // rax
  float v9; // xmm5_4
  long long v10; // rax
  float v11; // xmm4_4
  long long *v12; // rax
  __m128i *v13; // rdx
  float v14; // xmm4_4
  long long v15; // rax
  float v16; // xmm0_4
  float v17; // xmm5_4
  float v18; // xmm4_4
  long long v19; // rdx
  long long v20; // rdx
  long long v21; // rsi
  __m128i *v22[7]; // [rsp+0h] [rbp-38h] BYREF

  v22[1] = (__m128i *)__readfsqword(0x28u);
  if ( !a2 )
LABEL_26:
    abort();
  v5 = sub_80E0((long long)a1, a2, v22, 0);
  if ( !v5 )
  {
    v8 = a1[3];
    if ( v8 < 0 )
    {
      v20 = a1[3] & 1LL | (a1[3] >> 1);
      v9 = (float)(int)v20 + (float)(int)v20;
    }
    else
    {
      v9 = (float)(int)v8;
    }
    v10 = a1[2];
    if ( v10 < 0 )
    {
      v19 = a1[2] & 1LL | (a1[2] >> 1);
      v11 = (float)(int)v19 + (float)(int)v19;
    }
    else
    {
      v11 = (float)(int)v10;
    }
    if ( v9 > (float)(*(float *)(a1[5] + 8LL) * v11) )
    {
      sub_8000((long long)a1);
      v15 = a1[5];
      v16 = *(float *)(v15 + 8);
      if ( v17 > (float)(v14 * v16) )
      {
        v18 = v14 * *(float *)(v15 + 12);
        if ( !*(char *)(v15 + 16) )
          v18 = v18 * v16;
        if ( v18 >= 1.8446744e19 )
          return (unsigned int)-1;
        v21 = v18 >= 9.223372e18
            ? (unsigned int)(int)(float)(v18 - 9.223372e18) ^ 0x8000000000000000LL
            : (unsigned int)(int)v18;
        if ( !(unsigned char)sub_8B00((long long)a1, v21) )
          return (unsigned int)-1;
        if ( sub_80E0((long long)a1, a2, v22, 0) )
          goto LABEL_26;
      }
    }
    if ( !*(long long *)v22[0] )
    {
      *(long long *)v22[0] = a2;
      v6 = 1;
      ++a1[4];
      ++a1[3];
      return v6;
    }
    v12 = sub_81E0((long long)a1);
    if ( v12 )
    {
      v13 = v22[0];
      *v12 = a2;
      v6 = 1;
      v12[1] = v13->m128i_i64[1];
      v13->m128i_i64[1] = (long long)v12;
      ++a1[4];
      return v6;
    }
    return (unsigned int)-1;
  }
  v6 = 0;
  if ( a3 )
    *a3 = v5;
  return v6;
}



// Function: hash_insert @ 0x8e70
long long hash_insert(long long *a1, long long a2)
{
  int v2; // eax
  bool v3; // zf
  long long result; // rax
  long long v5[3]; // [rsp+0h] [rbp-18h] BYREF

  v5[1] = __readfsqword(0x28u);
  v2 = sub_8C80(a1, a2, v5);
  if ( v2 == -1 )
    return 0;
  v3 = v2 == 0;
  result = a2;
  if ( v3 )
    return v5[0];
  return result;
}



// Function: hash_remove @ 0x8ed0
long long hash_remove(long long *a1, long long a2)
{
  long long v3; // r12
  __m128i *v4; // rax
  long long v6; // rax
  float v7; // xmm5_4
  long long v8; // rax
  float v9; // xmm4_4
  float v10; // xmm4_4
  long long v11; // rax
  float v12; // xmm5_4
  float v13; // xmm4_4
  unsigned long long v14; // rsi
  long long *v15; // rbp
  void *v16; // rdi
  unsigned long long v17; // rdx
  __m128i *v18[5]; // [rsp+0h] [rbp-28h] BYREF

  v18[1] = (__m128i *)__readfsqword(0x28u);
  v3 = sub_80E0((long long)a1, a2, v18, 1);
  if ( !v3 )
    return v3;
  v4 = v18[0];
  --a1[4];
  if ( v4->m128i_i64[0] )
    return v3;
  v6 = a1[3] - 1LL;
  a1[3] = v6;
  if ( v6 < 0 )
  {
    v17 = v6 & 1 | ((unsigned long long)v6 >> 1);
    v8 = a1[2];
    v7 = (float)(int)v17 + (float)(int)v17;
    if ( v8 >= 0 )
      goto LABEL_6;
LABEL_18:
    v9 = (float)(v8 & 1 | (unsigned int)((unsigned long long)v8 >> 1))
       + (float)(v8 & 1 | (unsigned int)((unsigned long long)v8 >> 1));
    goto LABEL_7;
  }
  v7 = (float)(int)v6;
  v8 = a1[2];
  if ( v8 < 0 )
    goto LABEL_18;
LABEL_6:
  v9 = (float)(int)v8;
LABEL_7:
  if ( (float)(*(float *)a1[5] * v9) > v7 )
  {
    sub_8000((long long)a1);
    v11 = a1[5];
    if ( (float)(*(float *)v11 * v10) > v12 )
    {
      v13 = v10 * *(float *)(v11 + 4);
      if ( !*(char *)(v11 + 16) )
        v13 = v13 * *(float *)(v11 + 8);
      if ( v13 >= 9.223372e18 )
        v14 = (unsigned int)(int)(float)(v13 - 9.223372e18) ^ 0x8000000000000000LL;
      else
        v14 = (unsigned int)(int)v13;
      if ( !(unsigned char)sub_8B00((long long)a1, v14) )
      {
        v15 = (long long *)a1[9];
        while ( v15 )
        {
          v16 = v15;
          v15 = (long long *)v15[1];
          free(v16);
        }
        a1[9] = 0;
      }
    }
  }
  return v3;
}


