// Function: main @ 0x4dd0
long long main(int a1, char **a2, char **a3)
{
  char *v5; // rdi
  long long v6; // rbx
  int v7; // eax
  char *v8; // rax
  char v9; // al
  long long v10; // rax
  int v11; // r13d
  char *v12; // rdi
  const char *v13; // rsi
  void *v14; // rax
  void *v15; // rbp
  void *v16; // r12
  void *v17; // rdi
  long long v18; // rax
  long long v19; // rdx
  void *v20; // rdi
  int i; // ebx
  void *v22; // rbp
  int v24; // eax
  const char *v25; // rax
  long long v26[11]; // [rsp+0h] [rbp-58h] BYREF

  v5 = *a2;
  v26[3] = __readfsqword(0x28u);
  sub_13390(v5);
  setlocale(6, "");
  bindtextdomain("coreutils", "/usr/local/share/locale");
  textdomain("coreutils");
  sub_8DD0();
  sub_18230(sub_DF00);
  dword_27210 = 0;
  byte_272B8 = 1;
  qword_27380 = 0;
  tp.tv_sec = 0x8000000000000000LL;
  tp.tv_nsec = -1;
  v6 = (int)sub_BDA0(a1, a2);
  if ( !byte_27312 || (sub_6AF0(), !byte_27312) )
  {
    if ( !byte_272F4 )
      goto LABEL_3;
    goto LABEL_44;
  }
  qword_272C0 = 0;
  if ( byte_272F4
    || (unsigned char)sub_6420(13)
    || (unsigned char)sub_6420(14) && byte_27390
    || (unsigned char)sub_6420(12) && !dword_27338 )
  {
LABEL_44:
    byte_272FD = 1;
  }
LABEL_3:
  if ( !dword_272F8 )
  {
    v7 = 1;
    if ( !byte_272F5 && dword_27314 != 3 )
      v7 = dword_27338 == 0 ? 1 : 3;
    dword_272F8 = v7;
  }
  if ( byte_272F6 )
  {
    qword_273C8 = (void *)sub_F380(30, 0, sub_56D0, sub_56E0, j_free);
    if ( !qword_273C8 )
      sub_16DA0();
    obstack_begin(&unk_270E0, 0, 0, &malloc, &free);
  }
  v8 = getenv("TZ");
  qword_272A8 = sub_15D80(v8);
  if ( ((dword_27330 - 3) & 0xFFFFFFFD) == 0 || !dword_27338 || byte_27365 || byte_2732C )
  {
    byte_272A1 = 1;
    v9 = 0;
  }
  else
  {
    byte_272A1 = 0;
    v9 = 1;
    if ( !byte_272F6 && !byte_27312 && !dword_27314 )
      v9 = byte_272F4;
  }
  byte_272A0 = v9 & 1;
  if ( byte_27318 )
  {
    obstack_begin(&unk_271A0, 0, 0, &malloc, &free);
    obstack_begin(&unk_27140, 0, 0, &malloc, &free);
  }
  if ( byte_27311 )
  {
    sub_5670();
    v25 = (const char *)sub_16F20();
    if ( !v25 )
      v25 = "";
    qword_27388 = (long long)v25;
  }
  qword_273B8 = 100;
  v10 = sub_16B00(100, 208);
  v11 = a1 - v6;
  qword_273B0 = 0;
  qword_273C0 = v10;
  sub_6E20();
  if ( a1 - (int)v6 <= 0 )
  {
    if ( byte_272F5 )
    {
      v13 = (char *)(&dword_0 + 3);
      v12 = ".";
      sub_A550(".");
    }
    else
    {
      v13 = 0;
      v12 = ".";
      sub_6840((void *)".", 0);
    }
    v15 = qword_27380;
    if ( !qword_273B0 )
      goto LABEL_57;
    goto LABEL_60;
  }
  do
  {
    v12 = a2[v6];
    v13 = 0;
    ++v6;
    sub_A550(v12);
  }
  while ( a1 > (int)v6 );
  if ( qword_273B0 )
  {
LABEL_60:
    sub_89F0(v12, v13);
    if ( !byte_272F5 )
    {
      v13 = (char *)(&dword_0 + 1);
      v12 = 0;
      sub_A1E0(0);
    }
    if ( qword_273B0 )
    {
      sub_9F20();
      if ( !qword_27380 )
        goto LABEL_25;
      v12 = byte_9 + 1;
      sub_7460(10);
      v14 = qword_27380;
      goto LABEL_18;
    }
  }
  v14 = qword_27380;
  v15 = qword_27380;
  if ( v11 > 1 )
  {
LABEL_18:
    v15 = v14;
    if ( v14 )
      goto LABEL_21;
    goto LABEL_25;
  }
LABEL_57:
  if ( v15 )
  {
    if ( !*((long long *)v15 + 3) )
      byte_272B8 = 0;
    do
    {
LABEL_21:
      while ( 1 )
      {
        v16 = qword_273C8;
        v17 = *(void **)v15;
        qword_27380 = (void *)*((long long *)v15 + 3);
        if ( qword_273C8 )
        {
          if ( !v17 )
            break;
        }
        v13 = (const char *)*((long long *)v15 + 1);
        sub_B180(v17);
        v12 = (char *)v15;
        sub_61B0(v15);
        v15 = qword_27380;
        byte_272B8 = 1;
        if ( !qword_27380 )
          goto LABEL_25;
      }
      v18 = sub_6330(0, v13);
      v13 = (const char *)v26;
      v26[0] = v18;
      v26[1] = v19;
      v20 = (void *)sub_FA00(v16, v26);
      if ( !v20 )
        __assert_fail("found", "src/ls.c", 0x70Du, "main");
      j_free(v20);
      v12 = (char *)v15;
      sub_61B0(v15);
      v15 = qword_27380;
    }
    while ( qword_27380 );
  }
LABEL_25:
  if ( byte_27312 && byte_27310 )
  {
    if ( qword_26060 != 2
      || (v12 = (char *)s1, v13 = "\x1B[", memcmp(s1, "\x1B[", 2u))
      || qword_26070 != 1
      || *(char *)off_26078 != 109 )
    {
      sub_7CC0(v12, v13);
    }
    fflush_unlocked(stdout);
    sub_7DD0();
    for ( i = dword_27214; i; --i )
      raise(19);
    if ( sig )
      raise(sig);
  }
  if ( byte_27318 )
  {
    sub_8060("//DIRED//", &unk_271A0);
    sub_8060("//SUBDIRED//", &unk_27140);
    v24 = sub_151E0(qword_272D0);
    v13 = "//DIRED-OPTIONS// --quoting-style=%s\n";
    __printf_chk(1, "//DIRED-OPTIONS// --quoting-style=%s\n", off_259E0[v24]);
  }
  v22 = qword_273C8;
  if ( qword_273C8 )
  {
    if ( sub_EF20(qword_273C8, v13) )
      __assert_fail("hash_get_n_entries (active_dir_set) == 0", "src/ls.c", 0x741u, "main");
    sub_F540(v22);
  }
  return (unsigned int)dword_27210;
}



// Function: file_or_link_mode @ 0x55e0
long long file_or_link_mode(long long a1)
{
  if ( byte_27390 && *(char *)(a1 + 185) )
    return *(unsigned int *)(a1 + 172);
  else
    return *(unsigned int *)(a1 + 48);
}



// Function: first_percent_b @ 0x5620
char *first_percent_b(char *a1)
{
  char v1; // dl
  char *result; // rax
  char v3; // cl

  v1 = *a1;
  for ( result = a1; v1; ++result )
  {
    v3 = result[1];
    if ( v1 == 37 )
    {
      if ( v3 == 37 )
      {
        v1 = result[2];
        ++result;
        continue;
      }
      if ( v3 == 98 )
        return result;
    }
    v1 = result[1];
  }
  return 0;
}



// Function: file_escape_init @ 0x5670
bool file_escape_init()
{
  long long i; // rbx
  bool result; // al
  char v2; // r8

  for ( i = 0; i != 256; ++i )
  {
    v2 = sub_D3F0((unsigned int)i);
    result = 1;
    if ( !v2 )
      result = (unsigned int)(i - 45) <= 1 || (int)i == 126 || (int)i == 95;
    byte_26FE0[i] |= result;
  }
  return result;
}



// Function: dev_ino_hash @ 0x56d0
unsigned long long dev_ino_hash(long long *a1, unsigned long long a2)
{
  return *a1 % a2;
}



// Function: dev_ino_compare @ 0x56e0
bool dev_ino_compare(long long *a1, long long *a2)
{
  bool result; // al

  result = 0;
  if ( *a1 == *a2 )
    return a1[1] == a2[1];
  return result;
}



// Function: sighandler @ 0x5700
long long sighandler(int a1)
{
  long long result; // rax

  result = (unsigned int)sig;
  if ( !sig )
    sig = a1;
  return result;
}



// Function: stophandler @ 0x5720
long long stophandler()
{
  long long result; // rax

  result = (unsigned int)sig;
  if ( !sig )
    return (unsigned int)++dword_27214;
  return result;
}



// Function: get_funky_string @ 0x5740
long long get_funky_string(long long *a1, unsigned char **a2, unsigned int a3, long long *a4)
{
  long long result; // rax
  char v7; // r12
  unsigned char *v9; // rcx
  long long v10; // rbx
  long long v11; // rdi
  long long v12; // r8
  long long v13; // rsi
  long long v14; // r9
  bool v15; // cc
  char v16; // dl
  unsigned char *v17; // r13
  unsigned char v18; // dl
  unsigned char v19; // r8
  int v20; // r8d
  unsigned char *v21; // r9

  result = a3;
  v7 = a3;
  v9 = *a2;
  v10 = *a1;
  v11 = 1;
  LOBYTE(a3) = **a2;
  v12 = 0;
  v13 = v10 + 1;
  v14 = v10;
  v15 = (char)a3 <= 92;
  if ( (char)a3 == 92 )
    goto LABEL_8;
LABEL_2:
  if ( v15 )
  {
    if ( (char)a3 == 61 )
    {
      if ( v7 )
        goto LABEL_13;
    }
    else if ( (char)a3 <= 61 && (!(char)a3 || (char)a3 == 58) )
    {
      result = 1;
      goto LABEL_13;
    }
LABEL_5:
    ++v9;
    goto LABEL_6;
  }
  if ( (char)a3 != 94 )
    goto LABEL_5;
  v18 = v9[1];
  if ( (unsigned char)(v18 - 64) <= 0x3Eu )
  {
    v9 += 2;
    ++v11;
    *(char *)(++v13 - 2) = v18 & 0x1F;
    goto LABEL_7;
  }
  ++v9;
  if ( v18 == 63 )
  {
    ++v11;
    *(char *)(++v13 - 2) = 127;
    while ( 1 )
    {
LABEL_7:
      LOBYTE(a3) = *v9;
      v14 = v13 - 1;
      v12 = v11 - 1;
      v15 = (char)*v9 <= 92;
      if ( *v9 != 92 )
        goto LABEL_2;
LABEL_8:
      v16 = v9[1];
      v17 = v9 + 2;
      if ( !v16 )
        break;
      switch ( v16 )
      {
        case '0':
        case '1':
        case '2':
        case '3':
        case '4':
        case '5':
        case '6':
        case '7':
          v9 += 2;
          LOBYTE(a3) = v16 - 48;
          v19 = *v17;
          if ( (unsigned char)(*v17 - 48) > 7u )
            goto LABEL_6;
          do
          {
            ++v9;
            LOBYTE(a3) = v19 + 8 * a3 - 48;
            v19 = *v9;
          }
          while ( (unsigned char)(*v9 - 48) <= 7u );
          *(char *)(v13 - 1) = a3;
          ++v11;
          ++v13;
          continue;
        case '?':
          v16 = 127;
          v9 += 2;
          goto LABEL_19;
        case 'X':
        case 'x':
          v20 = v9[2];
          v21 = v9 + 3;
          a3 = 0;
          v9 += 2;
          if ( (char)v20 <= 70 )
            goto LABEL_29;
          break;
        case '_':
          v16 = 32;
          v9 += 2;
          goto LABEL_19;
        case 'a':
          v16 = 7;
          goto LABEL_18;
        case 'b':
          v16 = 8;
          goto LABEL_18;
        case 'e':
          v16 = 27;
          v9 += 2;
          goto LABEL_19;
        case 'f':
          v16 = 12;
          v9 += 2;
          goto LABEL_19;
        case 'n':
          v16 = 10;
          v9 += 2;
          goto LABEL_19;
        case 'r':
          v16 = 13;
          v9 += 2;
          goto LABEL_19;
        case 't':
          v16 = 9;
          v9 += 2;
          goto LABEL_19;
        case 'v':
          v16 = 11;
          goto LABEL_18;
        default:
LABEL_18:
          v9 += 2;
LABEL_19:
          *(char *)(v13 - 1) = v16;
          ++v11;
          ++v13;
          continue;
      }
      while ( (unsigned char)(v20 - 97) <= 5u )
      {
        a3 = v20 + 16 * a3 - 87;
        while ( 1 )
        {
          v20 = *v21++;
          v9 = v21 - 1;
          if ( (char)v20 > 70 )
            break;
LABEL_29:
          if ( (char)v20 > 64 )
          {
            a3 = v20 + 16 * a3 - 55;
          }
          else
          {
            if ( (unsigned char)(v20 - 48) > 9u )
              goto LABEL_6;
            a3 = v20 + 16 * a3 - 48;
          }
        }
      }
LABEL_6:
      *(char *)(v13 - 1) = a3;
      ++v11;
      ++v13;
    }
    v9 += 2;
    result = 0;
  }
  else
  {
    result = 0;
  }
LABEL_13:
  *a1 = v14;
  *a2 = v9;
  *a4 = v12;
  return result;
}



// Function: set_exit_status @ 0x59b0
long long set_exit_status(char a1)
{
  long long result; // rax

  if ( a1 )
  {
    dword_27210 = 2;
  }
  else
  {
    result = (unsigned int)dword_27210;
    if ( !dword_27210 )
      dword_27210 = 1;
  }
  return result;
}



// Function: unsigned_file_size @ 0x59e0
long long unsigned_file_size(long long a1)
{
  return a1;
}



// Function: errno_unsupported @ 0x59f0
long long errno_unsupported(int a1)
{
  if ( ((a1 - 22) & 0xFFFFFFEF) != 0 )
    return sub_55C0();
  else
    return 1;
}



// Function: is_linked_directory @ 0x5a10
bool is_linked_directory(long long a1)
{
  bool result; // al

  result = *(int *)(a1 + 168) == 9 || *(int *)(a1 + 168) == 3;
  if ( !result )
    return (*(int *)(a1 + 172) & 0xF000) == 0x4000;
  return result;
}



// Function: dirfirst_check @ 0x5a40
long long dirfirst_check(long long a1, long long a2)
{
  long long v2; // r9
  bool v3; // r8
  long long (*v4)(void); // r10
  unsigned char v5; // cl
  long long result; // rax

  sub_5A10(a2);
  v3 = sub_5A10(v2);
  result = v5 - (unsigned int)v3;
  if ( !(int)result )
    return v4();
  return result;
}



// Function: cmp_ctime @ 0x5a80
long long cmp_ctime(long long *a1, long long *a2, long long (*a3)(long long, long long))
{
  long long v4; // r14
  long long v5; // rdx
  long long v6; // r15
  long long v7; // rax
  long long v8; // rdx
  long long result; // rax

  v4 = sub_15CA0(a1 + 3);
  v6 = v5;
  v7 = sub_15CA0(a2 + 3);
  result = sub_162A0(v7, v8, v4, v6);
  if ( !(int)result )
    return a3(*a1, *a2);
  return result;
}



// Function: cmp_mtime @ 0x5af0
long long cmp_mtime(long long *a1, long long *a2, long long (*a3)(long long, long long))
{
  long long v4; // r14
  long long v5; // rdx
  long long v6; // r15
  long long v7; // rax
  long long v8; // rdx
  long long result; // rax

  v4 = sub_15CB0(a1 + 3);
  v6 = v5;
  v7 = sub_15CB0(a2 + 3);
  result = sub_162A0(v7, v8, v4, v6);
  if ( !(int)result )
    return a3(*a1, *a2);
  return result;
}



// Function: cmp_atime @ 0x5b60
long long cmp_atime(long long *a1, long long *a2, long long (*a3)(long long, long long))
{
  long long v4; // r14
  long long v5; // rdx
  long long v6; // r15
  long long v7; // rax
  long long v8; // rdx
  long long result; // rax

  v4 = sub_15C90(a1 + 3);
  v6 = v5;
  v7 = sub_15C90(a2 + 3);
  result = sub_162A0(v7, v8, v4, v6);
  if ( !(int)result )
    return a3(*a1, *a2);
  return result;
}



// Function: cmp_btime @ 0x5bd0
long long cmp_btime(long long *a1, long long *a2, long long (*a3)(long long, long long))
{
  long long v4; // r14
  long long v5; // rdx
  long long v6; // r15
  long long v7; // rax
  long long v8; // rdx
  long long result; // rax

  v4 = sub_5610(a1 + 3);
  v6 = v5;
  v7 = sub_5610(a2 + 3);
  result = sub_162A0(v7, v8, v4, v6);
  if ( !(int)result )
    return a3(*a1, *a2);
  return result;
}



// Function: off_cmp @ 0x5c40
long long off_cmp(long long a1, long long a2)
{
  long long result; // rax

  result = a1 > a2;
  if ( a1 < a2 )
    return 0xFFFFFFFFLL;
  return result;
}



// Function: xstrcoll_ctime @ 0x5c60
long long xstrcoll_ctime(long long *a1, long long *a2)
{
  return sub_5A80(a1, a2, (long long (*)(long long, long long))sub_72A0);
}



// Function: strcmp_ctime @ 0x5c70
long long strcmp_ctime(long long *a1, long long *a2)
{
  return sub_5A80(a1, a2, (long long (*)(long long, long long))&strcmp);
}



// Function: rev_xstrcoll_ctime @ 0x5c80
long long rev_xstrcoll_ctime(long long *a1, long long *a2)
{
  return sub_5A80(a2, a1, (long long (*)(long long, long long))sub_72A0);
}



// Function: rev_strcmp_ctime @ 0x5ca0
long long rev_strcmp_ctime(long long *a1, long long *a2)
{
  return sub_5A80(a2, a1, (long long (*)(long long, long long))&strcmp);
}



// Function: xstrcoll_df_ctime @ 0x5cc0
long long xstrcoll_df_ctime(long long a1, long long a2)
{
  return sub_5A40(a1, a2);
}



// Function: strcmp_df_ctime @ 0x5cd0
long long strcmp_df_ctime(long long a1, long long a2)
{
  return sub_5A40(a1, a2);
}



// Function: rev_xstrcoll_df_ctime @ 0x5ce0
long long rev_xstrcoll_df_ctime(long long a1, long long a2)
{
  return sub_5A40(a1, a2);
}



// Function: rev_strcmp_df_ctime @ 0x5cf0
long long rev_strcmp_df_ctime(long long a1, long long a2)
{
  return sub_5A40(a1, a2);
}



// Function: xstrcoll_atime @ 0x5da0
long long xstrcoll_atime(long long *a1, long long *a2)
{
  return sub_5B60(a1, a2, (long long (*)(long long, long long))sub_72A0);
}



// Function: strcmp_atime @ 0x5db0
long long strcmp_atime(long long *a1, long long *a2)
{
  return sub_5B60(a1, a2, (long long (*)(long long, long long))&strcmp);
}



// Function: rev_xstrcoll_atime @ 0x5dc0
long long rev_xstrcoll_atime(long long *a1, long long *a2)
{
  return sub_5B60(a2, a1, (long long (*)(long long, long long))sub_72A0);
}



// Function: rev_strcmp_atime @ 0x5de0
long long rev_strcmp_atime(long long *a1, long long *a2)
{
  return sub_5B60(a2, a1, (long long (*)(long long, long long))&strcmp);
}



// Function: xstrcoll_df_atime @ 0x5e00
long long xstrcoll_df_atime(long long a1, long long a2)
{
  return sub_5A40(a1, a2);
}



// Function: strcmp_df_atime @ 0x5e10
long long strcmp_df_atime(long long a1, long long a2)
{
  return sub_5A40(a1, a2);
}



// Function: rev_xstrcoll_df_atime @ 0x5e20
long long rev_xstrcoll_df_atime(long long a1, long long a2)
{
  return sub_5A40(a1, a2);
}



// Function: rev_strcmp_df_atime @ 0x5e30
long long rev_strcmp_df_atime(long long a1, long long a2)
{
  return sub_5A40(a1, a2);
}



// Function: xstrcoll_btime @ 0x5e40
long long xstrcoll_btime(long long *a1, long long *a2)
{
  return sub_5BD0(a1, a2, (long long (*)(long long, long long))sub_72A0);
}



// Function: strcmp_btime @ 0x5e50
long long strcmp_btime(long long *a1, long long *a2)
{
  return sub_5BD0(a1, a2, (long long (*)(long long, long long))&strcmp);
}



// Function: rev_xstrcoll_btime @ 0x5e60
long long rev_xstrcoll_btime(long long *a1, long long *a2)
{
  return sub_5BD0(a2, a1, (long long (*)(long long, long long))sub_72A0);
}



// Function: rev_strcmp_btime @ 0x5e80
long long rev_strcmp_btime(long long *a1, long long *a2)
{
  return sub_5BD0(a2, a1, (long long (*)(long long, long long))&strcmp);
}



// Function: xstrcoll_df_btime @ 0x5ea0
long long xstrcoll_df_btime(long long a1, long long a2)
{
  return sub_5A40(a1, a2);
}



// Function: strcmp_df_btime @ 0x5eb0
long long strcmp_df_btime(long long a1, long long a2)
{
  return sub_5A40(a1, a2);
}



// Function: rev_xstrcoll_df_btime @ 0x5ec0
long long rev_xstrcoll_df_btime(long long a1, long long a2)
{
  return sub_5A40(a1, a2);
}



// Function: rev_strcmp_df_btime @ 0x5ed0
long long rev_strcmp_df_btime(long long a1, long long a2)
{
  return sub_5A40(a1, a2);
}



// Function: xstrcoll_df_size @ 0x5ee0
long long xstrcoll_df_size(long long a1, long long a2)
{
  return sub_5A40(a1, a2);
}



// Function: strcmp_df_size @ 0x5ef0
long long strcmp_df_size(long long a1, long long a2)
{
  return sub_5A40(a1, a2);
}



// Function: rev_xstrcoll_df_size @ 0x5f00
long long rev_xstrcoll_df_size(long long a1, long long a2)
{
  return sub_5A40(a1, a2);
}



// Function: rev_strcmp_df_size @ 0x5f10
long long rev_strcmp_df_size(long long a1, long long a2)
{
  return sub_5A40(a1, a2);
}



// Function: rev_strcmp_df_extension @ 0x5f90
long long rev_strcmp_df_extension(long long a1, long long a2)
{
  return sub_5A40(a1, a2);
}



// Function: xstrcoll_df_width @ 0x5fa0
long long xstrcoll_df_width(long long a1, long long a2)
{
  return sub_5A40(a1, a2);
}



// Function: strcmp_df_width @ 0x5fb0
long long strcmp_df_width(long long a1, long long a2)
{
  return sub_5A40(a1, a2);
}



// Function: rev_xstrcoll_df_width @ 0x5fc0
long long rev_xstrcoll_df_width(long long a1, long long a2)
{
  return sub_5A40(a1, a2);
}



// Function: rev_strcmp_df_width @ 0x5fd0
long long rev_strcmp_df_width(long long a1, long long a2)
{
  return sub_5A40(a1, a2);
}



// Function: xstrcoll_df_version @ 0x5fe0
long long xstrcoll_df_version(long long a1, long long a2)
{
  return sub_5A40(a1, a2);
}



// Function: rev_xstrcoll_df_version @ 0x5ff0
long long rev_xstrcoll_df_version(long long a1, long long a2)
{
  return sub_5A40(a1, a2);
}



// Function: initialize_ordering_vector @ 0x6000
void initialize_ordering_vector()
{
  long long *v0; // rax
  long long v1; // rdx
  long long *v2; // rcx

  if ( qword_273B0 )
  {
    v0 = (long long *)qword_273A0;
    v1 = qword_273C0;
    v2 = (long long *)((char *)qword_273A0 + 8 * qword_273B0);
    do
    {
      *v0++ = v1;
      v1 += 208;
    }
    while ( v0 != v2 );
  }
}



// Function: get_type_indicator @ 0x6040
long long get_type_indicator(char a1, short a2, int a3)
{
  int v3; // ecx
  long long result; // rax
  bool v5; // cl
  int v6; // esi

  if ( !a1 )
  {
    result = 0;
    if ( a3 == 5 )
      return result;
    if ( a3 == 3 || a3 == 9 )
      return 47;
    v5 = a3 == 6;
    if ( dword_27314 == 1 )
      return result;
LABEL_12:
    result = 64;
    if ( !v5 )
    {
      if ( a1 )
      {
        v6 = a2 & 0xF000;
        result = 124;
        if ( v6 == 4096 )
          return result;
        LOBYTE(result) = v6 == 49152;
      }
      else
      {
        result = 124;
        if ( a3 == 1 )
          return result;
        LOBYTE(result) = a3 == 7;
      }
      return -(int)result & 0x3D;
    }
    return result;
  }
  v3 = a2 & 0xF000;
  if ( v3 == 0x8000 )
  {
    result = 0;
    if ( dword_27314 == 3 )
      return (a2 & 0x49) != 0 ? 0x2A : 0;
    return result;
  }
  result = 47;
  if ( v3 != 0x4000 )
  {
    result = 0;
    v5 = v3 == 40960;
    if ( dword_27314 != 1 )
      goto LABEL_12;
  }
  return result;
}



// Function: attach @ 0x6140
long long attach(char *i, char *a2, unsigned char *a3)
{
  char v3; // al
  char *v4; // rcx
  char *v5; // r8
  long long result; // rax

  v3 = *a2;
  if ( *a2 == 46 )
  {
    if ( !a2[1] )
      goto LABEL_9;
  }
  else if ( !v3 )
  {
    goto LABEL_9;
  }
  v4 = a2;
  do
  {
    v5 = i;
    ++v4;
    *i++ = v3;
    v3 = *v4;
  }
  while ( *v4 );
  if ( a2 >= v4 || *(v4 - 1) == 47 )
    goto LABEL_9;
  *i = 47;
  result = *a3;
  for ( i = v5 + 2; (char)result; result = *a3 )
  {
    *i++ = result;
    ++a3;
LABEL_9:
    ;
  }
  *i = 0;
  return result;
}



// Function: free_pending_ent @ 0x61b0
void free_pending_ent(void **ptr)
{
  free(*ptr);
  free(ptr[1]);
  free(ptr);
}



// Function: init_column_info @ 0x61d0
void init_column_info(unsigned long long a1)
{
  long long v2; // rsi
  long long v3; // rdi
  unsigned long long v4; // rbp
  long long v5; // rax
  long long v6; // rdi
  long long v7; // rdx
  long long v8; // rdi
  long long v9; // rsi
  long long v10; // rcx
  long long *v11; // rax
  long long *v12; // rdx
  long long v13; // rax

  if ( qword_262E0 < a1 )
  {
    v2 = qword_27200;
    v3 = qword_27208;
    if ( qword_27200 && (unsigned long long)qword_27200 >> 1 <= a1 )
    {
      v13 = sub_16AD0(qword_27208, qword_27200, 24);
      v4 = qword_27200;
      qword_27208 = v13;
    }
    else
    {
      v2 = a1;
      v4 = 2 * a1;
      qword_27208 = sub_16AD0(qword_27208, a1, 48);
    }
    if ( __CFADD__(v4, qword_262E0 + 1) || !is_mul_ok(v4 + qword_262E0 + 1, v4 - qword_262E0) )
      sub_16DA0(v3, v2, ((v4 + qword_262E0 + 1) * (unsigned __int128)(v4 - qword_262E0)) >> 64 != 0);
    v5 = sub_16B00(((v4 + qword_262E0 + 1) * (v4 - qword_262E0)) >> 1, 8);
    if ( v4 > qword_262E0 )
    {
      v6 = qword_27208;
      v7 = 8 * qword_262E0 + 8;
      do
      {
        *(long long *)(v6 + 3 * v7 - 8) = v5;
        v5 += v7;
        v7 += 8;
      }
      while ( v7 != 8 * v4 + 8 );
    }
    qword_262E0 = v4;
  }
  if ( a1 )
  {
    v8 = qword_27208;
    v9 = 0;
    v10 = 3;
    do
    {
      v11 = *(long long **)(v8 + 8 * v10 - 8);
      *(char *)(v8 + 8 * v10 - 24) = 1;
      ++v9;
      *(long long *)(v8 + 8 * v10 - 16) = v10;
      v12 = &v11[v9];
      do
        *v11++ = 3;
      while ( v12 != v11 );
      v10 += 3;
    }
    while ( 3 * a1 + 3 != v10 );
  }
}



// Function: dev_ino_pop @ 0x6330
long long dev_ino_pop()
{
  long long v0; // rax

  v0 = qword_270F8;
  if ( (unsigned long long)(qword_270F8 - qword_270F0) <= 0xF )
    __assert_fail("dev_ino_size <= obstack_object_size (&dev_ino_obstack)", "src/ls.c", 0x41Du, "dev_ino_pop");
  qword_270F8 -= 16;
  return *(long long *)(v0 - 16);
}



// Function: add_ignore_pattern @ 0x6380
long long *add_ignore_pattern(long long a1)
{
  long long *result; // rax
  long long v2; // rdx

  result = (long long *)sub_169E0(16);
  v2 = qword_272E8;
  *result = a1;
  result[1] = v2;
  qword_272E8 = (long long)result;
  return result;
}



// Function: decode_line_length @ 0x63b0
long long decode_line_length(char *a1)
{
  int v1; // eax
  long long result; // rax
  long long v3; // [rsp+0h] [rbp-18h]

  v1 = sub_17190(a1);
  if ( v1 )
    return -(long long)(v1 != 1);
  result = v3;
  if ( v3 < 0 )
    return 0;
  return result;
}



// Function: is_colored @ 0x6420
long long is_colored(unsigned int a1)
{
  long long *v1; // rax
  long long v2; // rdx
  const char *v3; // rdi
  unsigned int v5; // r8d
  unsigned int v6; // r8d

  v1 = &qword_26060[2 * a1];
  v2 = *v1;
  if ( !*v1 )
    return 0;
  v3 = (const char *)v1[1];
  if ( v2 == 1 )
  {
    LOBYTE(v5) = strncmp(v3, "0", 1u) != 0;
    return v5;
  }
  else if ( v2 == 2 )
  {
    LOBYTE(v6) = strncmp(v3, "00", 2u) != 0;
    return v6;
  }
  else
  {
    return 1;
  }
}



// Function: getenv_quoting_style @ 0x64b0
long long getenv_quoting_style()
{
  char *v0; // rax
  char *v1; // rbp
  int v2; // eax
  long long v4; // r12
  char *v5; // rax

  v0 = getenv("QUOTING_STYLE");
  if ( !v0 )
    return 0xFFFFFFFFLL;
  v1 = v0;
  v2 = sub_CF70(v0);
  if ( v2 >= 0 )
    return dword_1D9C0[v2];
  v4 = sub_158C0(v1, off_259E0);
  v5 = dcgettext(0, "ignoring invalid value of environment variable QUOTING_STYLE: %s", 5);
  error(0, 0, v5, v4);
  return 0xFFFFFFFFLL;
}



// Function: stdout_isatty @ 0x6590
long long stdout_isatty()
{
  char v0; // al

  v0 = byte_26018;
  if ( byte_26018 < 0 )
  {
    v0 = isatty(1);
    byte_26018 = v0;
  }
  return v0 & 1;
}



// Function: abmon_init @ 0x65c0
long long abmon_init(long long a1)
{
  unsigned long long v1; // r13
  long long v2; // r12
  void *v3; // r15
  unsigned long long v4; // rbx
  char *v5; // rbp
  const unsigned short **v6; // rax
  long long v7; // r8
  unsigned char v8; // al
  long long *v9; // rdx
  long long v10; // rcx
  long long v12[9]; // [rsp+10h] [rbp-48h] BYREF

  v1 = 12;
  v12[1] = __readfsqword(0x28u);
  while ( 2 )
  {
    v2 = a1;
    v3 = &unk_2000E;
    v4 = 0;
    do
    {
      v12[0] = v1;
      v5 = nl_langinfo((nl_item)v3);
      if ( strchr(v5, 37) )
        return 0;
      v6 = __ctype_b_loc();
      v8 = sub_5570((unsigned int)*v5, v2, v6, v12, v7, 0);
      if ( (unsigned long long)sub_10DF0(v5, v2, 128, v10, (*(short *)(*v9 + 2LL * v8) & 0x800) != 0) > 0x7F )
        return 0;
      if ( v4 < v12[0] )
        v4 = v12[0];
      LODWORD(v3) = (int)v3 + 1;
      v2 += 128;
    }
    while ( (int)v3 != (int)&unk_2001A );
    if ( v1 > v4 )
    {
      v1 = v4;
      continue;
    }
    break;
  }
  return 1;
}



// Function: abformat_init @ 0x66c0
unsigned long long abformat_init()
{
  long long i; // r12
  const char *v1; // rbx
  const char *v2; // r13
  long long v3; // r15
  char *v4; // r14
  __int128 v6; // [rsp+10h] [rbp-658h]
  char v7[1536]; // [rsp+20h] [rbp-648h] BYREF
  char v8; // [rsp+620h] [rbp-48h] BYREF
  unsigned long long v9; // [rsp+628h] [rbp-40h]

  v9 = __readfsqword(0x28u);
  *(long long *)&v6 = sub_5620(off_26040);
  *((long long *)&v6 + 1) = sub_5620(off_26048);
  if ( v6 != 0 && (unsigned char)sub_65C0((long long)v7) )
  {
    for ( i = 0; ; i = 1 )
    {
      v1 = v7;
      v2 = (&off_26040)[i];
      v3 = *(long long *)&v7[8 * i - 16];
      v4 = (char *)&unk_263E0 + 1536 * i;
      do
      {
        if ( v3 )
        {
          if ( v3 - (long long)v2 > 128
            || (unsigned int)__snprintf_chk(
                               v4,
                               128,
                               1,
                               -1,
                               "%.*s%s%s",
                               (int)v3 - (int)v2,
                               v2,
                               v1,
                               (const char *)(v3 + 2)) > 0x7F )
          {
            return v9 - __readfsqword(0x28u);
          }
        }
        else if ( (unsigned int)snprintf(v4, 0x80u, "%s", v2) > 0x7F )
        {
          return v9 - __readfsqword(0x28u);
        }
        v1 += 128;
        v4 += 128;
      }
      while ( v1 != &v8 );
      if ( i == 1 )
        break;
    }
    byte_263C8 = 1;
  }
  return v9 - __readfsqword(0x28u);
}



// Function: queue_directory @ 0x6840
void *queue_directory(void *src, void *a2, char a3)
{
  long long v4; // r12
  long long v5; // rbp
  long long *v6; // rbx
  void *result; // rax

  v4 = (long long)a2;
  v5 = (long long)src;
  v6 = (long long *)sub_169E0(32);
  if ( a2 )
    v4 = sub_16D80(a2);
  v6[1] = v4;
  if ( src )
    v5 = sub_16D80(src);
  result = qword_27380;
  *v6 = v5;
  *((char *)v6 + 16) = a3;
  v6[3] = (long long)result;
  qword_27380 = v6;
  return result;
}



// Function: time_type_to_statx @ 0x68b0
long long time_type_to_statx()
{
  if ( (unsigned int)dword_27334 > 3 )
    abort();
  return dword_19620[dword_27334];
}



// Function: calc_req_mask @ 0x68d0
long long calc_req_mask()
{
  int v0; // r12d
  unsigned int v1; // r12d
  int v2; // r12d

  v0 = -(byte_272FC == 0);
  LOBYTE(v0) = 0;
  v1 = v0 + 258;
  if ( byte_2732C )
    v1 |= 0x400u;
  if ( !dword_27338 )
  {
    v2 = sub_68B0() | v1;
    if ( byte_26029 || byte_2732E )
      v1 = v2 | 0x20C;
    else
      v1 = v2 | 0x204;
    if ( byte_26028 )
      v1 |= 0x10u;
  }
  switch ( dword_27330 )
  {
    case 0:
    case 1:
    case 2:
    case 4:
    case 6:
      return v1;
    case 3:
      return v1 | 0x200;
    case 5:
      v1 |= sub_68B0();
      return v1;
    default:
      abort();
  }
}



// Function: patterns_match @ 0x6990
long long patterns_match(long long a1, const char *a2)
{
  long long v2; // rbx

  if ( !a1 )
    return 0;
  v2 = a1;
  do
  {
    if ( !fnmatch(*(const char **)v2, a2, 4) )
      return 1;
    v2 = *(long long *)(v2 + 8);
  }
  while ( v2 );
  return 0;
}



// Function: file_ignored @ 0x69f0
long long file_ignored(long long a1)
{
  if ( dword_272F0 != 2 )
  {
    if ( *(char *)a1 == 46 )
    {
      if ( !dword_272F0 || !*(char *)(a1 + (*(char *)(a1 + 1) == 46) + 1) )
        return 1;
    }
    else if ( !dword_272F0 && (unsigned char)sub_6990(qword_272E0, (const char *)a1) )
    {
      return 1;
    }
  }
  return sub_6990(qword_272E8, (const char *)a1);
}



// Function: known_term_type @ 0x6a50
long long known_term_type()
{
  char *v0; // rax
  const char *v1; // r13
  const char *v2; // rbx

  v0 = getenv("TERM");
  if ( !v0 )
    return 0;
  v1 = v0;
  if ( !*v0 )
    return 0;
  v2 = "# Configuration file for dircolors, a utility to help you set the";
  while ( strncmp(v2, "TERM ", 5u) || fnmatch(v2 + 5, v1, 0) )
  {
    v2 += strlen(v2) + 1;
    if ( (unsigned long long)(v2 - "# Configuration file for dircolors, a utility to help you set the") > 0x12C7 )
      return 0;
  }
  return 1;
}



// Function: parse_ls_color @ 0x6af0
unsigned long long parse_ls_color()
{
  unsigned char *v0; // rax
  char *v1; // rax
  unsigned char *v2; // rbx
  char v3; // al
  char v4; // al
  bool v5; // zf
  long long v6; // rbx
  const char *v7; // rsi
  long long *v8; // rcx
  long long v9; // r12
  char *v10; // rax
  char *v11; // rax
  void *v12; // rbx
  void *v13; // rdi
  unsigned char *v15; // rbx
  long long *v16; // rbp
  unsigned char *v17; // [rsp+10h] [rbp-58h] BYREF
  void *v18; // [rsp+18h] [rbp-50h] BYREF
  char s1[3]; // [rsp+25h] [rbp-43h] BYREF
  unsigned long long v20; // [rsp+28h] [rbp-40h]

  v20 = __readfsqword(0x28u);
  v0 = (unsigned char *)getenv("LS_COLORS");
  v17 = v0;
  if ( v0 && *v0 )
  {
    strcpy(s1, "??");
    ptr = (void *)sub_16D80(v0);
    v18 = ptr;
    while ( 1 )
    {
LABEL_8:
      while ( 1 )
      {
        v2 = v17;
        v3 = *v17;
        if ( *v17 != 42 )
          break;
        v15 = v17 + 1;
        v16 = (long long *)sub_169E0(40);
        v16[4] = (long long)qword_27308;
        qword_27308 = v16;
        v17 = v15;
        v16[1] = (long long)v18;
        if ( (unsigned char)sub_5740((long long *)&v18, &v17, 1u, v16) )
        {
          v5 = *v17++ == 61;
          if ( v5 )
          {
            v16[3] = (long long)v18;
            if ( (unsigned char)sub_5740((long long *)&v18, &v17, 0, v16 + 2) )
              continue;
          }
        }
        goto LABEL_18;
      }
      if ( v3 != 58 )
        break;
      ++v17;
    }
    if ( !v3 )
      goto LABEL_21;
    s1[0] = *v17;
    v4 = *++v17;
    if ( v4 )
    {
      s1[1] = v4;
      v5 = v2[2] == 61;
      v17 = v2 + 3;
      if ( v5 )
      {
        v6 = 0;
        v7 = "lc";
        while ( strcmp(s1, v7) )
        {
          v7 = off_25860[++v6];
          if ( !v7 )
            goto LABEL_17;
        }
        v7 = (const char *)&v17;
        v8 = &qword_26060[2 * (int)v6];
        v8[1] = (long long)v18;
        if ( (unsigned char)sub_5740((long long *)&v18, &v17, 0, v8) )
          goto LABEL_8;
LABEL_17:
        v9 = sub_158C0(s1, v7);
        v10 = dcgettext(0, "unrecognized prefix: %s", 5);
        error(0, 0, v10, v9);
      }
    }
LABEL_18:
    v11 = dcgettext(0, "unparsable value for LS_COLORS environment variable", 5);
    error(0, 0, v11);
    free(ptr);
    v12 = qword_27308;
    while ( v12 )
    {
      v13 = v12;
      v12 = (void *)*((long long *)v12 + 4);
      free(v13);
    }
    byte_27312 = 0;
LABEL_21:
    if ( qword_260D0 == 6 && !strncmp(off_260D8, "target", 6u) )
      byte_27390 = 1;
  }
  else
  {
    v1 = getenv("COLORTERM");
    if ( (!v1 || !*v1) && !(unsigned char)sub_6A50() )
      byte_27312 = 0;
  }
  return v20 - __readfsqword(0x28u);
}



// Function: free_ent @ 0x6dc0
void free_ent(long long a1)
{
  void *v1; // rdi

  free(*(void **)a1);
  free(*(void **)(a1 + 8));
  free(*(void **)(a1 + 16));
  v1 = *(void **)(a1 + 176);
  if ( v1 != &unk_2602A )
  {
    if ( (unsigned char)sub_55D0() )
      free(v1);
    else
      freecon();
  }
}



// Function: clear_files @ 0x6e20
void clear_files()
{
  unsigned long long v0; // rbx
  long long v1; // rdi

  if ( qword_273B0 )
  {
    v0 = 0;
    do
    {
      v1 = *((long long *)qword_273A0 + v0++);
      sub_6DC0(v1);
    }
    while ( qword_273B0 > v0 );
  }
  qword_273B0 = 0;
  byte_273A9 = 0;
  byte_27364 = 0;
  dword_27360 = 0;
  dword_2735C = 0;
  dword_27358 = 0;
  dword_27350 = 0;
  dword_2734C = 0;
  dword_27348 = 0;
  dword_27354 = 0;
  dword_27344 = 0;
  dword_27340 = 0;
  dword_2733C = 0;
}



// Function: format_user_or_group_width @ 0x6ed0
long long format_user_or_group_width(long long a1, long long a2)
{
  long long result; // rax

  if ( !a1 )
    return __snprintf_chk(0, 0, 1, -1, "%lu", a2);
  result = sub_113A0(a1, 0);
  if ( (int)result < 0 )
    return 0;
  return result;
}



// Function: format_inode @ 0x6f10
const char *format_inode(long long a1, long long a2, long long a3)
{
  const char *result; // rax

  if ( !*(char *)(a3 + 184) )
    return "?";
  result = "?";
  if ( *(long long *)(a3 + 32) )
    return (const char *)sub_10C20(*(long long *)(a3 + 32), a1);
  return result;
}



// Function: file_failure @ 0x6f50
long long file_failure(char a1, const char *a2, long long a3)
{
  long long v3; // r13
  int *v4; // rax

  v3 = sub_15600(4, a3);
  v4 = __errno_location();
  error(0, *v4, a2, v3);
  return sub_59B0(a1);
}



// Function: getfilecon_cache @ 0x6f90
long long getfilecon_cache(long long a1, long long a2, char a3)
{
  long long v4; // rsi
  unsigned int v5; // r12d
  int *v7; // rax

  if ( *(long long *)(a2 + 24) == qword_262F0 )
  {
    v5 = -1;
    *__errno_location() = 95;
    return v5;
  }
  v4 = a2 + 176;
  if ( a3 )
  {
    v5 = sub_15B10(a1, v4);
    if ( (v5 & 0x80000000) == 0 )
      return v5;
  }
  else
  {
    v5 = sub_15B30(a1, v4);
    if ( (v5 & 0x80000000) == 0 )
      return v5;
  }
  v7 = __errno_location();
  if ( !(unsigned char)sub_59F0(*v7) )
    return v5;
  qword_262F0 = *(long long *)(a2 + 24);
  return v5;
}



// Function: file_has_acl_cache @ 0x7020
long long file_has_acl_cache(long long a1, long long a2)
{
  int *v2; // rbp
  int v3; // r8d

  v2 = __errno_location();
  if ( *(long long *)(a2 + 24) == qword_262E8 )
  {
    *v2 = 95;
    return 0;
  }
  else
  {
    *v2 = 0;
    v3 = sub_E060(a1, a2 + 24);
    if ( v3 <= 0 && (unsigned char)sub_59F0(*v2) )
    {
      qword_262E8 = *(long long *)(a2 + 24);
      return (unsigned int)v3;
    }
    else
    {
      return (unsigned int)v3;
    }
  }
}



// Function: get_link_name @ 0x70b0
long long get_link_name(char *a1, long long a2, char a3)
{
  long long result; // rax
  char *v5; // rax

  result = sub_CDD0(a1);
  *(long long *)(a2 + 8) = result;
  if ( !result )
  {
    v5 = dcgettext(0, "cannot read symbolic link %s", 5);
    return sub_6F50(a3, v5, (long long)a1);
  }
  return result;
}



// Function: make_link_name @ 0x7100
char *make_link_name(char *src, const char *a2)
{
  char *v3; // rdi
  size_t v4; // r12
  size_t v5; // rax
  char *v6; // rax
  char *v7; // r14
  char *v8; // rax

  if ( !a2 )
    return 0;
  if ( *a2 == 47 )
  {
    v3 = (char *)a2;
  }
  else
  {
    v3 = (char *)a2;
    v4 = sub_DFB0();
    if ( v4 )
    {
      v5 = strlen(a2);
      v6 = (char *)sub_169E0(v4 + v5 + 2);
      v7 = v6;
      if ( src[v4 - 1] != 47 )
        ++v4;
      v8 = stpncpy(v6, src, v4);
      strcpy(v8, a2);
      return v7;
    }
  }
  return (char *)sub_16D80(v3);
}



// Function: needs_quoting @ 0x71a0
bool needs_quoting(char *s)
{
  long long v1; // rax
  char v3[2]; // [rsp+6h] [rbp-22h] BYREF
  unsigned long long v4; // [rsp+8h] [rbp-20h]

  v4 = __readfsqword(0x28u);
  v1 = sub_152C0(v3, 2, s, -1, qword_272D0);
  return *s != v3[0] || strlen(s) != v1;
}



// Function: format_group_width @ 0x7220
long long format_group_width(__gid_t a1)
{
  long long v2; // rax

  if ( byte_2732D )
    return sub_6ED0(0, a1);
  v2 = sub_109E0(a1);
  return sub_6ED0(v2, a1);
}



// Function: format_user_width @ 0x7260
long long format_user_width(__uid_t a1)
{
  long long v2; // rax

  if ( byte_2732D )
    return sub_6ED0(0, a1);
  v2 = sub_10840(a1);
  return sub_6ED0(v2, a1);
}



// Function: xstrcoll @ 0x72a0
int xstrcoll(char *s1, char *s2)
{
  *__errno_location() = 0;
  return strcoll(s1, s2);
}



// Function: cmp_extension @ 0x72d0
long long cmp_extension(const char **a1, const char **a2, long long (*a3)(char *, char *))
{
  char *v5; // rbp
  char *v6; // rsi
  long long result; // rax

  v5 = strrchr(*a1, 46);
  v6 = strrchr(*a2, 46);
  if ( !v6 )
    v6 = (char *)"";
  if ( !v5 )
    v5 = (char *)"";
  result = a3(v5, v6);
  if ( !(int)result )
    return a3((char *)*a1, (char *)*a2);
  return result;
}



// Function: rev_strcmp_extension @ 0x7350
long long rev_strcmp_extension(const char **a1, const char **a2)
{
  return sub_72D0(a2, a1, (long long (*)(char *, char *))&strcmp);
}



// Function: basename_is_dot_or_dotdot @ 0x73b0
long long basename_is_dot_or_dotdot()
{
  long long result; // rax
  char v1; // dl
  char *v2; // rdi

  v2 = (char *)sub_D360();
  result = 0;
  if ( *v2 == 46 )
  {
    v1 = v2[(v2[1] == 46) + 1];
    return (v1 == 47) | (unsigned int)(v1 == 0);
  }
  return result;
}



// Function: indent @ 0x73d0
int indent(unsigned long long a1, unsigned long long a2)
{
  unsigned long long v2; // rbx
  unsigned long long v3; // rax
  unsigned long long v4; // rbp

  if ( a1 < a2 )
  {
    v2 = a1;
    do
    {
      while ( 1 )
      {
        v4 = v2 + 1;
        if ( qword_272C0 )
        {
          if ( a2 / qword_272C0 > v4 / qword_272C0 )
            break;
        }
        ++v2;
        LODWORD(v3) = putchar_unlocked(32);
        if ( v4 >= a2 )
          return v3;
      }
      putchar_unlocked(9);
      v3 = v2 / qword_272C0;
      v2 = v2 + qword_272C0 - v2 % qword_272C0;
    }
    while ( v2 < a2 );
  }
  return v3;
}



// Function: dired_outbyte @ 0x7460
int dired_outbyte(char a1)
{
  ++qword_271F8;
  return putchar_unlocked(a1);
}



// Function: get_color_indicator @ 0x7480
char *get_color_indicator(long long a1, char a2)
{
  const char *v2; // rbp
  int v3; // r12d
  char v4; // dl
  int v5; // esi
  long long v6; // rax
  unsigned long long v7; // rax
  char *result; // rax
  short v9; // ax
  int v10; // ecx
  char v11; // r8
  long long v12; // rax
  size_t v13; // rax
  size_t *v14; // rbx
  size_t v15; // r12
  char v16; // r8

  if ( a2 )
  {
    v2 = *(const char **)(a1 + 8);
    v3 = *(int *)(a1 + 172);
    if ( *(char *)(a1 + 185) )
    {
      v4 = *(char *)(a1 + 184);
      v5 = 0;
      if ( !v4 )
        goto LABEL_4;
LABEL_14:
      v10 = v3 & 0xF000;
      if ( v10 != 0x8000 )
      {
        if ( v10 == 0x4000 )
        {
          if ( (v3 & 0x202) != 0x202 || (v16 = sub_6420(0x14u), v7 = 320, !v16) )
          {
            if ( (v3 & 2) == 0 || (v11 = sub_6420(0x13u), v7 = 304, !v11) )
            {
              v7 = 96;
              if ( (v3 & 0x200) != 0 )
              {
                v12 = -(long long)((unsigned char)sub_6420(0x12u) == 0);
                LOBYTE(v12) = v12 & 0x40;
                v7 = v12 + 288;
              }
            }
          }
          goto LABEL_9;
        }
        if ( v10 != 40960 )
        {
          v7 = 128;
          if ( v10 != 4096 )
          {
            v7 = 144;
            if ( v10 != 49152 )
            {
              v7 = 160;
              if ( v10 != 24576 )
              {
                v7 = 208;
                if ( v10 == 0x2000 )
                  v7 = 176;
              }
            }
          }
          goto LABEL_9;
        }
        v6 = 7;
        goto LABEL_5;
      }
      if ( (v3 & 0x800) != 0 && (unsigned char)sub_6420(0x10u) )
      {
        v7 = 256;
      }
      else if ( (v3 & 0x400) != 0 && (unsigned char)sub_6420(0x11u) )
      {
        v7 = 272;
      }
      else if ( (unsigned char)sub_6420(0x15u) && *(char *)(a1 + 192) )
      {
        v7 = 336;
      }
      else if ( (v3 & 0x49) != 0 && (unsigned char)sub_6420(0xEu) )
      {
        v7 = 224;
      }
      else
      {
        if ( *(long long *)(a1 + 40) <= 1u || !(unsigned char)sub_6420(0x16u) )
          goto LABEL_30;
        v7 = 352;
      }
LABEL_9:
      result = (char *)qword_26060 + v7;
      if ( !*((long long *)result + 1) )
        return 0;
      return result;
    }
    v5 = -1;
    if ( (unsigned char)sub_6420(0xCu) )
    {
      v7 = 192;
      goto LABEL_9;
    }
  }
  else
  {
    v9 = sub_55E0(a1);
    v2 = *(const char **)a1;
    v5 = *(unsigned char *)(a1 + 185);
    LOWORD(v3) = v9;
  }
  v4 = *(char *)(a1 + 184);
  if ( v4 )
    goto LABEL_14;
LABEL_4:
  v6 = dword_19680[*(unsigned int *)(a1 + 168)];
  v4 = (int)v6 == 7;
  if ( (int)v6 != 5 )
  {
LABEL_5:
    if ( v5 || !v4 )
    {
      v7 = 16 * v6;
    }
    else
    {
      v7 = 208;
      if ( !byte_27390 )
        v7 = (-(long long)((unsigned char)sub_6420(0xDu) == 0) & 0xFFFFFFFFFFFFFFA0LL) + 208;
    }
    goto LABEL_9;
  }
LABEL_30:
  v13 = strlen(v2);
  v14 = (size_t *)qword_27308;
  v15 = v13;
  if ( !qword_27308 )
  {
LABEL_34:
    v7 = 80;
    goto LABEL_9;
  }
  while ( *v14 > v15 || (unsigned int)sub_D5C0(&v2[v15 - *v14], v14[1]) )
  {
    v14 = (size_t *)v14[4];
    if ( !v14 )
      goto LABEL_34;
  }
  result = (char *)(v14 + 2);
  if ( !v14[3] )
    return 0;
  return result;
}



// Function: dired_outbuf @ 0x77d0
size_t dired_outbuf(const void *a1, size_t a2)
{
  qword_271F8 += a2;
  return fwrite_unlocked(a1, 1u, a2, stdout);
}



// Function: dired_outstring @ 0x77f0
size_t dired_outstring(const char *a1)
{
  size_t v1; // rax

  v1 = strlen(a1);
  return sub_77D0(a1, v1);
}



// Function: dired_indent @ 0x7810
size_t dired_indent()
{
  size_t result; // rax

  if ( byte_27318 )
    return sub_77F0("  ");
  return result;
}



// Function: file_escape @ 0x7830
long long file_escape(const char *a1, char a2)
{
  const char *v2; // r14
  size_t v4; // rsi
  long long v5; // rax
  long long v6; // rcx
  long long v7; // r8
  long long v8; // r9
  long long v9; // rdx
  long long v10; // r12
  char *v11; // r15
  char v12; // dl
  char *v13; // rdi

  v2 = a1;
  v4 = strlen(a1) + 1;
  v5 = sub_16B00(3, v4);
  v9 = *(unsigned char *)a1;
  v10 = v5;
  v11 = (char *)v5;
  if ( (char)v9 )
  {
    while ( 1 )
    {
      ++v2;
      if ( (char)v9 == 47 )
      {
        if ( a2 )
          break;
      }
      v7 = (unsigned char)sub_5570((unsigned int)(char)v9, v4, v9, v6, v7, v8);
      if ( byte_26FE0[v7] )
      {
        *v11++ = v12;
LABEL_4:
        v9 = *(unsigned char *)v2;
        if ( !(char)v9 )
          goto LABEL_9;
      }
      else
      {
        v13 = v11;
        v4 = 1;
        v11 += 3;
        __sprintf_chk(v13, 1, -1, "%%%02x", v7);
        v9 = *(unsigned char *)v2;
        if ( !(char)v9 )
          goto LABEL_9;
      }
    }
    *v11++ = 47;
    goto LABEL_4;
  }
LABEL_9:
  *v11 = 0;
  return v10;
}



// Function: push_current_dired_pos @ 0x7900
long long push_current_dired_pos(long long a1)
{
  long long *v1; // rdx
  long long result; // rax

  if ( byte_27318 )
  {
    v1 = *(long long **)(a1 + 24);
    if ( *(long long *)(a1 + 32) - (long long)v1 <= 7u )
    {
      obstack_newchunk(a1, 8);
      result = qword_271F8;
      **(long long **)(a1 + 24) = qword_271F8;
    }
    else
    {
      result = qword_271F8;
      *v1 = qword_271F8;
    }
    *(long long *)(a1 + 24) += 8LL;
  }
  return result;
}



// Function: dev_ino_push @ 0x7970
long long *dev_ino_push(long long a1, long long a2)
{
  long long *result; // rax

  result = (long long *)qword_270F8;
  if ( (unsigned long long)(qword_27100 - qword_270F8) <= 0xF )
  {
    obstack_newchunk(&unk_270E0, 16);
    result = (long long *)qword_270F8;
  }
  qword_270F8 = (long long)(result + 2);
  result[1] = a1;
  *result = a2;
  return result;
}



// Function: signal_setup @ 0x79d0
unsigned long long signal_setup(char a1)
{
  int *v1; // rbx
  int i; // ebp
  int *v4; // rbx
  int v5; // r15d
  int *v6; // rbp
  int v7; // ebp
  void (*v8)(int); // rax
  struct sigaction v9; // [rsp+0h] [rbp-D8h] BYREF
  unsigned long long v10; // [rsp+98h] [rbp-40h]

  v10 = __readfsqword(0x28u);
  if ( !a1 )
  {
    v1 = (int *)&unk_19644;
    for ( i = 20; ; i = *v1++ )
    {
      if ( sigismember(&set, i) )
      {
        signal(i, 0);
        if ( v1 == (int *)&unk_19670 )
          return v10 - __readfsqword(0x28u);
      }
      else if ( v1 == (int *)&unk_19670 )
      {
        return v10 - __readfsqword(0x28u);
      }
    }
  }
  v4 = (int *)&unk_19644;
  v5 = 20;
  v6 = (int *)&unk_19644;
  sigemptyset(&set);
  while ( 1 )
  {
    sigaction(v5, 0, &v9);
    if ( v9.sa_handler != (__sighandler_t)((char *)&dword_0 + 1) )
      sigaddset(&set, v5);
    if ( v6 == (int *)&unk_19670 )
      break;
    v5 = *v6++;
  }
  v9.sa_flags = 0x10000000;
  v7 = 20;
  *(__m128i *)v9.sa_mask.__val = _mm_load_si128((const __m128i *)&set);
  *(__m128i *)&v9.sa_mask.__val[2] = _mm_load_si128((const __m128i *)&set.__val[2]);
  *(__m128i *)&v9.sa_mask.__val[4] = _mm_load_si128((const __m128i *)&set.__val[4]);
  *(__m128i *)&v9.sa_mask.__val[6] = _mm_load_si128((const __m128i *)&set.__val[6]);
  *(__m128i *)&v9.sa_mask.__val[8] = _mm_load_si128((const __m128i *)&set.__val[8]);
  *(__m128i *)&v9.sa_mask.__val[10] = _mm_load_si128((const __m128i *)&set.__val[10]);
  *(__m128i *)&v9.sa_mask.__val[12] = _mm_load_si128((const __m128i *)&set.__val[12]);
  *(__m128i *)&v9.sa_mask.__val[14] = _mm_load_si128((const __m128i *)&set.__val[14]);
  while ( 1 )
  {
    if ( sigismember(&set, v7) )
    {
      v8 = (void (*)(int))sub_5700;
      if ( v7 == 20 )
        v8 = (void (*)(int))sub_5720;
      v9.sa_handler = v8;
      sigaction(v7, &v9, 0);
    }
    if ( v4 == (int *)&unk_19670 )
      break;
    v7 = *v4++;
  }
  return v10 - __readfsqword(0x28u);
}



// Function: signal_init @ 0x7ba0
unsigned long long signal_init()
{
  return sub_79D0(1);
}



// Function: put_indicator @ 0x7bb0
size_t put_indicator(long long a1)
{
  if ( !byte_27310 )
  {
    byte_27310 = 1;
    if ( tcgetpgrp(1) >= 0 )
      sub_7BA0();
    sub_7C10();
  }
  return fwrite_unlocked(*(const void **)(a1 + 8), *(long long *)a1, 1u, stdout);
}



// Function: prep_non_filename_text @ 0x7c10
size_t prep_non_filename_text()
{
  if ( qword_26088 )
    return sub_7BB0((long long)&unk_26080);
  sub_7BB0((long long)qword_26060);
  sub_7BB0((long long)&unk_26090);
  return sub_7BB0((long long)&qword_26070);
}



// Function: set_normal_color @ 0x7c60
void set_normal_color()
{
  if ( byte_27312 )
  {
    if ( (unsigned char)sub_6420(4u) )
    {
      sub_7BB0((long long)qword_26060);
      sub_7BB0((long long)&unk_260A0);
      sub_7BB0((long long)&qword_26070);
    }
  }
}



// Function: restore_default_color @ 0x7cc0
size_t restore_default_color()
{
  sub_7BB0((long long)qword_26060);
  return sub_7BB0((long long)&qword_26070);
}



// Function: process_signals @ 0x7ce0
unsigned long long process_signals()
{
  int v0; // r12d
  sigset_t v2; // [rsp+0h] [rbp-A8h] BYREF
  unsigned long long v3; // [rsp+88h] [rbp-20h]

  v3 = __readfsqword(0x28u);
  while ( sig || dword_27214 )
  {
    if ( byte_27310 )
      sub_7CC0();
    fflush_unlocked(stdout);
    sigprocmask(0, &set, &v2);
    v0 = sig;
    if ( dword_27214 )
    {
      v0 = 19;
      --dword_27214;
    }
    else
    {
      signal(sig, 0);
    }
    raise(v0);
    sigprocmask(2, &v2, 0);
  }
  return v3 - __readfsqword(0x28u);
}



// Function: signal_restore @ 0x7dd0
unsigned long long signal_restore()
{
  return sub_79D0(0);
}



// Function: visit_dir @ 0x7df0
long long visit_dir(long long a1, long long a2)
{
  unsigned int v2; // r12d
  long long *v4; // rax
  void *v5; // rdi
  long long *v6; // rsi
  void *v7; // rbp
  long long v8; // rax
  long long v9; // rdx

  v2 = a2;
  v4 = (long long *)sub_169E0(16);
  v5 = qword_273C8;
  *v4 = a2;
  v6 = v4;
  v7 = v4;
  v4[1] = a1;
  v8 = sub_F9A0(v5, v4);
  if ( !v8 )
    sub_16DA0(v5, v6, v9);
  LOBYTE(v2) = v7 != (void *)v8;
  if ( v7 != (void *)v8 )
    free(v7);
  return v2;
}



// Function: dired_dump_obstack @ 0x8060
int dired_dump_obstack(const char *a1, long long a2)
{
  char *v2; // rax
  char *v3; // rbx
  signed long long v4; // rbp
  unsigned long long v5; // rax
  unsigned long long v6; // rdx
  char *v7; // rbp
  long long v8; // rdx

  v2 = *(char **)(a2 + 24);
  v3 = *(char **)(a2 + 16);
  v4 = v2 - v3;
  if ( (unsigned long long)(v2 - v3) > 7 )
  {
    if ( v2 == v3 )
      *(char *)(a2 + 80) |= 2u;
    v5 = ~*(long long *)(a2 + 48) & (unsigned long long)&v2[*(long long *)(a2 + 48)];
    v6 = *(long long *)(a2 + 32);
    if ( v5 - *(long long *)(a2 + 8) <= v6 - *(long long *)(a2 + 8) )
      v6 = v5;
    v7 = &v3[v4 & 0xFFFFFFFFFFFFFFF8LL];
    *(long long *)(a2 + 24) = v6;
    *(long long *)(a2 + 16) = v6;
    fputs_unlocked(a1, stdout);
    do
    {
      v8 = *(long long *)v3;
      v3 += 8;
      __printf_chk(1, " %ld", v8);
    }
    while ( v7 != v3 );
    LODWORD(v2) = putchar_unlocked(10);
  }
  return (int)v2;
}



// Function: format_user_or_group @ 0x8100
int format_user_or_group(const char *a1, long long a2, int a3)
{
  int v3; // ebx
  int result; // eax

  if ( a1 )
  {
    v3 = a3 - sub_113A0(a1, 0);
    if ( v3 < 0 )
      v3 = 0;
    sub_77F0(a1);
    do
      result = sub_7460(32);
    while ( v3-- != 0 );
  }
  else
  {
    result = __printf_chk(1, "%*lu ", a3, a2);
    qword_271F8 += result;
  }
  return result;
}



// Function: format_group @ 0x8180
int format_group(__gid_t a1, int a2, char a3)
{
  const char *v3; // r8
  long long v5; // rsi

  v3 = "?";
  v5 = a1;
  if ( a3 )
  {
    v3 = 0;
    if ( !byte_2732D )
    {
      v5 = a1;
      v3 = (const char *)sub_109E0(a1);
    }
  }
  return sub_8100(v3, v5, a2);
}



// Function: format_user @ 0x81d0
int format_user(__uid_t a1, int a2, char a3)
{
  const char *v3; // r8
  long long v5; // rsi

  v3 = "?";
  v5 = a1;
  if ( a3 )
  {
    v3 = 0;
    if ( !byte_2732D )
    {
      v5 = a1;
      v3 = (const char *)sub_10840(a1);
    }
  }
  return sub_8100(v3, v5, a2);
}



// Function: long_time_expected_width @ 0x82b0
long long long_time_expected_width()
{
  long long result; // rax
  long long v1; // rax
  long long v2; // [rsp+8h] [rbp-460h] BYREF
  char v3[64]; // [rsp+10h] [rbp-458h] BYREF
  char v4[1016]; // [rsp+50h] [rbp-418h] BYREF
  unsigned long long v5; // [rsp+448h] [rbp-20h]

  v5 = __readfsqword(0x28u);
  result = (unsigned int)dword_26014;
  if ( dword_26014 < 0 )
  {
    v2 = 0;
    if ( sub_160A0(qword_272A8, &v2, v3) && (v1 = sub_8240(v4, 0, v3, qword_272A8, 0)) != 0 )
    {
      result = sub_11180(v4, v1, 0);
      dword_26014 = result;
    }
    else
    {
      result = (unsigned int)dword_26014;
    }
    if ( (int)result < 0 )
    {
      dword_26014 = 0;
      return 0;
    }
  }
  return result;
}



// Function: quote_name_width @ 0x87f0
long long quote_name_width(void *a1, long long a2, unsigned int a3)
{
  unsigned char v4; // [rsp+Fh] [rbp-2039h] BYREF
  void *ptr; // [rsp+10h] [rbp-2038h] BYREF
  long long v6; // [rsp+18h] [rbp-2030h] BYREF
  char v7[24]; // [rsp+20h] [rbp-2028h] BYREF
  unsigned long long v8; // [rsp+2028h] [rbp-20h]

  v8 = __readfsqword(0x28u);
  ptr = v7;
  sub_8390(&ptr, a1, a2, a3, &v6, &v4);
  if ( ptr != v7 && ptr != a1 )
    free(ptr);
  return v6 + v4;
}



// Function: fileinfo_name_width @ 0x8890
long long fileinfo_name_width(long long a1)
{
  long long result; // rax

  result = *(long long *)(a1 + 200);
  if ( !result )
    return sub_87F0(*(void **)a1, qword_272D0, *(int *)(a1 + 196));
  return result;
}



// Function: cmp_width @ 0x88c0
long long cmp_width(long long *a1, long long *a2, long long (*a3)(long long, long long))
{
  int v4; // ebx
  long long result; // rax

  v4 = sub_8890((long long)a1);
  result = v4 - (unsigned int)sub_8890((long long)a2);
  if ( !(int)result )
    return a3(*a1, *a2);
  return result;
}



// Function: rev_strcmp_width @ 0x8920
long long rev_strcmp_width(long long *a1, long long *a2)
{
  return sub_88C0(a2, a1, (long long (*)(long long, long long))&strcmp);
}



// Function: strcmp_width @ 0x8940
long long strcmp_width(long long *a1, long long *a2)
{
  return sub_88C0(a1, a2, (long long (*)(long long, long long))&strcmp);
}



// Function: rev_xstrcoll_width @ 0x8950
long long rev_xstrcoll_width(long long *a1, long long *a2)
{
  return sub_88C0(a2, a1, (long long (*)(long long, long long))sub_72A0);
}



// Function: xstrcoll_width @ 0x8970
long long xstrcoll_width(long long *a1, long long *a2)
{
  return sub_88C0(a1, a2, (long long (*)(long long, long long))sub_72A0);
}



// Function: update_current_files_info @ 0x8980
void update_current_files_info()
{
  unsigned long long v0; // rbx
  long long v1; // rbp
  long long v2; // rax
  bool v3; // cc

  if ( dword_27330 == 2 || qword_272B0 && (unsigned int)(dword_27338 - 2) <= 1 )
  {
    v0 = 0;
    if ( qword_273B0 )
    {
      do
      {
        v1 = *((long long *)qword_273A0 + v0++);
        v2 = sub_8890(v1);
        v3 = qword_273B0 <= v0;
        *(long long *)(v1 + 200) = v2;
      }
      while ( !v3 );
    }
  }
}



// Function: sort_files @ 0x89f0
void sort_files()
{
  long long v0; // rbp
  int v1; // eax
  long long v2; // rsi

  v0 = qword_273B0;
  if ( qword_273B0 + ((unsigned long long)qword_273B0 >> 1) > qword_27398 )
  {
    free(qword_273A0);
    qword_273A0 = (void *)sub_16B00(v0, 24);
    qword_27398 = 3 * qword_273B0;
  }
  sub_6000();
  sub_8980();
  if ( dword_27330 != 6 )
  {
    v1 = _setjmp(env);
    v2 = (unsigned int)dword_27330;
    if ( v1 )
    {
      if ( dword_27330 == 4 )
        __assert_fail("sort_type != sort_version", "src/ls.c", 0x1008u, "sort_files");
      sub_6000();
      v1 = 1;
    }
    if ( (int)v2 == 5 )
      v2 = (unsigned int)(dword_27334 + 5);
    sub_11610(
      qword_273A0,
      qword_273B0,
      *(&off_24F60 + 8 * v2 + 4 * v1 + 2 * (unsigned char)byte_2732F + (unsigned char)byte_272F4));
  }
}



// Function: length_of_file_name_and_frills @ 0x8af0
long long length_of_file_name_and_frills(long long a1)
{
  long long v2; // rbx
  long long v3; // rax
  long long v4; // r8
  char v5; // al
  long long v6; // r8
  const char *v8; // rdi
  long long v9; // rax
  const char *v10; // rax
  long long v11[87]; // [rsp+0h] [rbp-2B8h] BYREF

  v11[83] = __readfsqword(0x28u);
  if ( !byte_272FC )
  {
    v2 = 0;
    goto LABEL_11;
  }
  if ( dword_27338 == 4 )
  {
    v10 = (const char *)sub_10C20(*(long long *)(a1 + 32), v11);
    v2 = strlen(v10) + 1;
LABEL_11:
    if ( !byte_2732C )
    {
LABEL_6:
      if ( !byte_27365 )
        goto LABEL_7;
      if ( dword_27338 == 4 )
      {
        v9 = strlen(*(const char **)(a1 + 176)) + 1;
        goto LABEL_18;
      }
LABEL_17:
      v9 = dword_27354 + 1LL;
LABEL_18:
      v2 += v9;
      goto LABEL_7;
    }
    if ( dword_27338 == 4 )
    {
      v8 = "?";
      if ( *(char *)(a1 + 184) )
        v8 = (const char *)sub_FFA0(*(long long *)(a1 + 88), v11, (unsigned int)dword_27328, 512, qword_27320);
      v3 = strlen(v8) + 1;
      goto LABEL_5;
    }
LABEL_4:
    v3 = dword_2735C + 1LL;
LABEL_5:
    v2 += v3;
    goto LABEL_6;
  }
  v2 = dword_27360 + 1LL;
  if ( byte_2732C )
    goto LABEL_4;
  if ( byte_27365 )
    goto LABEL_17;
LABEL_7:
  v4 = v2 + sub_8890(a1);
  if ( dword_27314 )
  {
    v5 = sub_6040(*(char *)(a1 + 184), *(int *)(a1 + 48), *(int *)(a1 + 168));
    return v6 - ((v5 == 0) - 1LL);
  }
  return v4;
}



// Function: calculate_columns @ 0x8c90
unsigned long long calculate_columns(char a1)
{
  unsigned long long v1; // r12
  long long v3; // rdx
  unsigned long long v4; // rbx
  long long v5; // rax
  unsigned long long v6; // r10
  long long v7; // r9
  unsigned long long v8; // r11
  long long v9; // rsi
  unsigned long long v10; // rcx
  unsigned long long v11; // r8
  unsigned long long *v12; // rdx
  unsigned long long v13; // rax
  long long v14; // rdi
  char *v15; // rax

  v1 = qword_273B0;
  v3 = qword_27200;
  if ( qword_273B0 <= (unsigned long long)qword_27200 )
    v3 = qword_273B0;
  if ( qword_27200 )
    v1 = v3;
  v4 = 0;
  sub_61D0(v1);
  if ( qword_273B0 )
  {
    do
    {
      v5 = sub_8AF0(*((long long *)qword_273A0 + v4));
      v6 = qword_273B0;
      v7 = v5;
      if ( v1 )
      {
        v8 = qword_272B0;
        v9 = qword_27208;
        v10 = 0;
        do
        {
          v14 = v10++;
          if ( *(char *)v9 )
          {
            v11 = a1 ? v4 / ((v6 + v10 - 1) / v10) : v4 % v10;
            v12 = (unsigned long long *)(*(long long *)(v9 + 16) + 8 * v11);
            v13 = v7 + 2LL * (v11 != v14);
            if ( *v12 < v13 )
            {
              *(long long *)(v9 + 8) += v13 - *v12;
              *v12 = v13;
              *(char *)v9 = *(long long *)(v9 + 8) < v8;
            }
          }
          v9 += 24;
        }
        while ( v1 != v10 );
      }
      ++v4;
    }
    while ( v4 < v6 );
  }
  if ( v1 > 1 )
  {
    v15 = (char *)(qword_27208 + 24 * v1 - 24);
    do
    {
      if ( *v15 )
        break;
      --v1;
      v15 -= 24;
    }
    while ( v1 != 1 );
  }
  return v1;
}



// Function: quote_name @ 0x8eb0
long long quote_name(void *a1, long long a2, unsigned int a3, long long a4, char a5, long long a6, const char *a7)
{
  long long v9; // rbx
  char v10; // r14
  size_t v11; // r10
  const char *v12; // rax
  const char *v13; // rcx
  size_t v14; // r10
  long long v17; // [rsp+0h] [rbp-2078h]
  char *ptr; // [rsp+8h] [rbp-2070h]
  void *ptra; // [rsp+8h] [rbp-2070h]
  char *v20; // [rsp+10h] [rbp-2068h]
  size_t n; // [rsp+18h] [rbp-2060h]
  unsigned char v22; // [rsp+27h] [rbp-2051h] BYREF
  void *v23; // [rsp+28h] [rbp-2050h] BYREF
  char v24[24]; // [rsp+30h] [rbp-2048h] BYREF
  unsigned long long v25; // [rsp+2038h] [rbp-40h]

  v25 = __readfsqword(0x28u);
  v23 = v24;
  v9 = sub_8390(&v23, a1, a2, a3, 0, &v22);
  if ( v22 && a5 )
    sub_7460(32);
  if ( a4 )
    sub_8E50(a4);
  if ( a7 )
  {
    v10 = byte_273A8;
    if ( byte_273A8 && (v10 = byte_273A9) != 0 )
    {
      if ( v22 )
      {
        v17 = 0;
        v11 = v9;
        v10 = 0;
      }
      else
      {
        putchar_unlocked(*(char *)v23);
        v17 = 1;
        v11 = v9 - 2;
      }
    }
    else
    {
      v17 = 0;
      v11 = v9;
    }
    n = v11;
    ptr = (char *)sub_7830((const char *)qword_27388, 0);
    v12 = (const char *)sub_7830(a7, 1);
    v13 = "";
    if ( *v12 != 47 )
      v13 = "/";
    v20 = (char *)v12;
    __printf_chk(1, "\x1B]8;;file://%s%s%s\a", ptr, v13, v12);
    free(ptr);
    free(v20);
    v14 = n;
  }
  else
  {
    v17 = 0;
    v14 = v9;
    v10 = 0;
  }
  if ( a6 )
  {
    ptra = (void *)v14;
    sub_7900(a6);
    fwrite_unlocked((char *)v23 + v17, 1u, (size_t)ptra, stdout);
    qword_271F8 += v9;
    sub_7900(a6);
  }
  else
  {
    fwrite_unlocked((char *)v23 + v17, 1u, v14, stdout);
    qword_271F8 += v9;
  }
  if ( a7 )
  {
    fputs_unlocked("\x1B]8;;\a", stdout);
    if ( v10 )
      putchar_unlocked(*((char *)v23 + v9 - 1));
  }
  if ( v23 != v24 && v23 != a1 )
    free(v23);
  return v9 + v22;
}



// Function: print_name_with_quoting @ 0x9160
long long print_name_with_quoting(long long a1, char a2, long long a3, unsigned long long a4)
{
  void *v5; // rdx
  void *v6; // r14
  long long v7; // r12
  char *v10; // r12

  v5 = *(void **)(a1 + 8);
  if ( !a2 )
    v5 = *(void **)a1;
  v6 = v5;
  if ( byte_27312 && ((v10 = sub_7480(a1, a2)) != 0 || (unsigned char)sub_6420(4u)) )
  {
    v7 = sub_8EB0(v6, qword_272D0, *(int *)(a1 + 196), (long long)v10, a2 ^ 1u, a3, *(const char **)(a1 + 16));
    sub_7CE0();
    sub_7C10();
    if ( qword_272B0 && a4 / qword_272B0 != (v7 + a4 - 1) / qword_272B0 )
      sub_7BB0((long long)&unk_261D0);
  }
  else
  {
    v7 = sub_8EB0(v6, qword_272D0, *(int *)(a1 + 196), 0, a2 ^ 1u, a3, *(const char **)(a1 + 16));
    sub_7CE0();
  }
  return v7;
}



// Function: print_long_format @ 0x92b0
unsigned long long print_long_format(long long a1)
{
  int v1; // eax
  bool v2; // cc
  bool v3; // r14
  long long v4; // rdx
  char *v5; // rbx
  const char *v6; // r15
  int v7; // ebp
  int v8; // eax
  char *v9; // rax
  char v10; // al
  const char *v11; // rax
  char *v12; // rbx
  const char *v13; // r15
  int v14; // ebp
  int v15; // ebp
  char *v16; // rbp
  char v17; // al
  const char *v18; // rbx
  int v19; // eax
  char *v20; // rbx
  size_t v21; // rbx
  long long v22; // rax
  long long v23; // rdx
  long long v24; // rbp
  long long v26; // rdx
  int v27; // eax
  long long v28; // rdx
  long long v29; // rax
  long long v30; // rcx
  const char *v31; // rax
  int v32; // ebp
  unsigned int v33; // eax
  const char *v34; // r15
  unsigned int v35; // eax
  const char *v36; // r9
  int v37; // eax
  int v38; // eax
  long long v39; // r8
  long long v40; // r9
  long long v41; // rsi
  long long v42; // rax
  int v43; // [rsp+0h] [rbp-11A8h]
  long long v44; // [rsp+0h] [rbp-11A8h]
  long long v45; // [rsp+8h] [rbp-11A0h]
  long long v46; // [rsp+10h] [rbp-1198h] BYREF
  long long v47; // [rsp+18h] [rbp-1190h]
  char v48[68]; // [rsp+20h] [rbp-1188h] BYREF
  char v49[12]; // [rsp+64h] [rbp-1144h] BYREF
  long long v50; // [rsp+70h] [rbp-1138h] BYREF
  char v51[232]; // [rsp+90h] [rbp-1118h] BYREF
  char v52[3656]; // [rsp+320h] [rbp-E88h] BYREF
  unsigned long long v53; // [rsp+1168h] [rbp-40h]

  v53 = __readfsqword(0x28u);
  if ( *(char *)(a1 + 184) )
  {
    sub_E2F0(a1 + 24, v49);
  }
  else
  {
    v49[0] = aPcdbLswd[*(unsigned int *)(a1 + 168)];
    memset(&v49[1], 63, 10);
  }
  if ( !byte_27364 )
  {
    v49[10] = 0;
    goto LABEL_5;
  }
  v27 = *(int *)(a1 + 188);
  if ( v27 == 1 )
  {
    v49[10] = 46;
LABEL_5:
    v1 = dword_27334;
    v2 = (unsigned int)dword_27334 <= 2;
    if ( dword_27334 != 2 )
      goto LABEL_6;
LABEL_49:
    v3 = 1;
    v46 = sub_15C90(a1 + 24);
    v47 = v28;
    goto LABEL_10;
  }
  if ( v27 != 2 )
    goto LABEL_5;
  v1 = dword_27334;
  v49[10] = 43;
  v2 = (unsigned int)dword_27334 <= 2;
  if ( dword_27334 == 2 )
    goto LABEL_49;
LABEL_6:
  if ( v2 )
  {
    v3 = 1;
    if ( v1 )
      v46 = sub_15CA0(a1 + 24);
    else
      v46 = sub_15CB0(a1 + 24);
    v47 = v4;
  }
  else
  {
    if ( v1 != 3 )
      abort();
    v46 = sub_5610(a1 + 24);
    v47 = v26;
    v3 = (v26 & v46) != -1;
  }
LABEL_10:
  v5 = v52;
  if ( byte_272FC )
  {
    v31 = sub_6F10((long long)v51, 21, a1);
    v5 = &v52[(int)__sprintf_chk(v52, 1, 3643, "%*s ", dword_27360, v31)];
  }
  if ( byte_2732C )
  {
    v6 = "?";
    if ( *(char *)(a1 + 184) )
      v6 = (const char *)sub_FFA0(*(long long *)(a1 + 88), v51, (unsigned int)dword_27328, 512, qword_27320);
    v7 = dword_2735C;
    v8 = v7 - sub_113A0(v6, 0);
    if ( v8 <= 0 )
      goto LABEL_62;
    v9 = &v5[v8];
    do
      *v5++ = 32;
    while ( v9 != v5 );
    while ( 1 )
    {
      ++v6;
      v5 = v9 + 1;
      v10 = *(v6 - 1);
      *(v5 - 1) = v10;
      if ( !v10 )
        break;
LABEL_62:
      v9 = v5;
    }
    *(v5 - 1) = 32;
  }
  v11 = "?";
  if ( *(char *)(a1 + 184) )
    v11 = (const char *)sub_10C20(*(long long *)(a1 + 40), v51);
  v12 = &v5[(int)__sprintf_chk(v5, 1, -1, "%s %*s ", v49, dword_27358, v11)];
  sub_7810();
  if ( byte_26029 || byte_26028 || byte_2732E || byte_27365 )
  {
    sub_77D0(v52, v12 - v52);
    if ( byte_26029 )
    {
      sub_81D0(*(int *)(a1 + 52), dword_27350, *(char *)(a1 + 184));
      if ( !byte_26028 )
      {
LABEL_26:
        if ( !byte_2732E )
          goto LABEL_27;
LABEL_61:
        sub_81D0(*(int *)(a1 + 52), dword_27348, *(char *)(a1 + 184));
LABEL_27:
        v12 = v52;
        if ( byte_27365 )
          sub_8100(*(const char **)(a1 + 176), 0, dword_27354);
        goto LABEL_29;
      }
    }
    else if ( !byte_26028 )
    {
      goto LABEL_26;
    }
    sub_8180(*(int *)(a1 + 56), dword_2734C, *(char *)(a1 + 184));
    if ( !byte_2732E )
      goto LABEL_27;
    goto LABEL_61;
  }
LABEL_29:
  if ( !*(char *)(a1 + 184) )
  {
    v13 = "?";
LABEL_31:
    v14 = dword_2733C;
    v15 = v14 - sub_113A0(v13, 0);
    if ( v15 <= 0 )
    {
      v16 = v12;
    }
    else
    {
      v16 = &v12[v15];
      do
        *v12++ = 32;
      while ( v16 != v12 );
    }
    do
    {
      v17 = *v13++;
      *v16++ = v17;
    }
    while ( v17 );
    *(v16 - 1) = 32;
    goto LABEL_36;
  }
  if ( (*(int *)(a1 + 48) & 0xB000) != 0x2000 )
  {
    v29 = sub_59E0(*(long long *)(a1 + 72));
    v13 = (const char *)sub_FFA0(v29, v51, (unsigned int)dword_2731C, v30, qword_26020);
    goto LABEL_31;
  }
  v32 = dword_2733C - (dword_27344 + dword_27340 + 2);
  v33 = gnu_dev_minor(*(long long *)(a1 + 64));
  v34 = (const char *)sub_10C20(v33, v51);
  v43 = dword_27340;
  v35 = gnu_dev_major(*(long long *)(a1 + 64));
  v36 = (const char *)sub_10C20(v35, &v50);
  v37 = 0;
  if ( v32 >= 0 )
    v37 = v32;
  v16 = &v12[(int)__sprintf_chk(v12, 1, -1, "%*s, %*s ", dword_27344 + v37, v36, v43, v34)];
LABEL_36:
  *v16 = 1;
  if ( !*(char *)(a1 + 184) || !v3 )
    goto LABEL_38;
  if ( !sub_160A0(qword_272A8, &v46, v48) )
    goto LABEL_83;
  v44 = v46;
  v45 = v47;
  v38 = sub_162A0(tp.tv_sec, tp.tv_nsec, v46, v47);
  v39 = v44;
  v40 = v45;
  if ( v38 < 0 )
  {
    sub_E960(&tp);
    v39 = v46;
    v40 = v47;
  }
  v41 = 0;
  if ( (int)sub_162A0(tp.tv_sec - 15778476, tp.tv_nsec, v39, v40) < 0 )
    v41 = (unsigned int)sub_162A0(v46, v47, tp.tv_sec, tp.tv_nsec) >> 31;
  v42 = sub_8240(v16, v41, v48, qword_272A8, (unsigned int)v47);
  if ( v42 )
  {
    v16 += v42;
  }
  else
  {
LABEL_83:
    if ( *v16 )
    {
      if ( *(char *)(a1 + 184) )
      {
        v18 = (const char *)sub_7DE0(v46, v51);
        goto LABEL_39;
      }
LABEL_38:
      v18 = "?";
LABEL_39:
      v19 = sub_82B0();
      v20 = &v16[(int)__sprintf_chk(v16, 1, -1, "%*s ", v19, v18)];
      goto LABEL_40;
    }
  }
  *v16 = 32;
  v20 = v16 + 1;
LABEL_40:
  v21 = v20 - v52;
  sub_77D0(v52, v21);
  v22 = sub_9160(a1, 0, (long long)&unk_271A0, v21);
  v23 = *(unsigned int *)(a1 + 168);
  v24 = v22;
  if ( (int)v23 == 6 )
  {
    if ( *(long long *)(a1 + 8) )
    {
      sub_77F0(" -> ");
      sub_9160(a1, 1, 0, v21 + v24 + 4);
      if ( dword_27314 )
        sub_9280(1, *(unsigned int *)(a1 + 172), 0);
    }
  }
  else if ( dword_27314 )
  {
    sub_9280(*(unsigned char *)(a1 + 184), *(unsigned int *)(a1 + 48), v23);
  }
  return v53 - __readfsqword(0x28u);
}



// Function: print_with_separator @ 0x9c00
int print_with_separator(char a1)
{
  unsigned long long v1; // r13
  unsigned long long v2; // rbx
  unsigned long long v3; // rbp
  long long v4; // r12
  long long v5; // rax
  int v6; // r15d

  if ( qword_273B0 )
  {
    v1 = 0;
    v2 = 0;
    while ( 1 )
    {
      v4 = *((long long *)qword_273A0 + v2);
      if ( qword_272B0 )
        break;
      v3 = v1;
      if ( v2 )
      {
        v3 = v1 + 2;
        v1 += 2LL;
LABEL_11:
        v6 = 32;
        goto LABEL_12;
      }
LABEL_4:
      ++v2;
      sub_9A80(v4, v3);
      if ( qword_273B0 <= v2 )
        return putchar_unlocked(byte_26019);
    }
    v5 = sub_8AF0(*((long long *)qword_273A0 + v2));
    if ( !v2 )
    {
      v3 = v1;
      v1 += v5;
      goto LABEL_4;
    }
    v3 = v1 + 2;
    if ( !qword_272B0 || qword_272B0 > v5 + v1 + 2 && -3 - v5 >= v1 )
    {
      v1 += v5 + 2;
      goto LABEL_11;
    }
    v6 = byte_26019;
    v1 = v5;
    v3 = 0;
LABEL_12:
    putchar_unlocked(a1);
    putchar_unlocked(v6);
    goto LABEL_4;
  }
  return putchar_unlocked(byte_26019);
}



// Function: print_horizontal @ 0x9d30
int print_horizontal()
{
  long long v0; // r14
  long long v1; // r12
  long long v2; // r15
  long long v3; // rbp
  unsigned long long v4; // r13
  unsigned long long v5; // rbx
  long long v6; // rbp
  unsigned long long v7; // r12
  unsigned long long v8; // rdi
  unsigned long long v10; // [rsp+8h] [rbp-40h]

  v10 = sub_8C90(0);
  v0 = qword_27208 + 24 * v10 - 24;
  v1 = *(long long *)qword_273A0;
  v2 = sub_8AF0(*(long long *)qword_273A0);
  v3 = **(long long **)(v0 + 16);
  sub_9A80(v1, 0);
  if ( (unsigned long long)qword_273B0 > 1 )
  {
    v4 = 0;
    v5 = 1;
    do
    {
      v7 = v5 % v10;
      if ( v5 % v10 )
      {
        v8 = v4 + v2;
        v4 += v3;
        sub_73D0(v8, v4);
      }
      else
      {
        v4 = 0;
        putchar_unlocked(byte_26019);
      }
      v6 = *((long long *)qword_273A0 + v5++);
      sub_9A80(v6, v4);
      v2 = sub_8AF0(v6);
      v3 = *(long long *)(*(long long *)(v0 + 16) + 8 * v7);
    }
    while ( qword_273B0 > v5 );
  }
  return putchar_unlocked(byte_26019);
}



// Function: print_many_per_line @ 0x9e30
unsigned long long print_many_per_line()
{
  unsigned long long v0; // rax
  long long v1; // r15
  unsigned long long result; // rax
  unsigned long long v3; // rbp
  long long v4; // r13
  unsigned long long i; // r14
  unsigned long long v6; // rdi
  long long v7; // r12
  long long v8; // rbx
  long long v9; // [rsp+8h] [rbp-50h]
  unsigned long long v10; // [rsp+10h] [rbp-48h]
  unsigned long long v11; // [rsp+18h] [rbp-40h]

  v0 = sub_8C90(1);
  v11 = 0;
  v1 = qword_27208 + 24 * v0 - 24;
  result = qword_273B0 / v0 - ((qword_273B0 % v0 == 0) - 1LL);
  v10 = result;
  if ( result )
  {
    do
    {
      v3 = v11;
      v4 = 0;
      for ( i = 0; ; sub_73D0(v6, i) )
      {
        v9 = *((long long *)qword_273A0 + v3);
        v7 = sub_8AF0(v9);
        v8 = *(long long *)(*(long long *)(v1 + 16) + v4);
        v4 += 8;
        sub_9A80(v9, i);
        v3 += v10;
        if ( qword_273B0 <= v3 )
          break;
        v6 = i + v7;
        i += v8;
      }
      putchar_unlocked(byte_26019);
      result = ++v11;
    }
    while ( v10 != v11 );
  }
  return result;
}



// Function: print_current_files @ 0x9f20
int print_current_files()
{
  int v0; // eax
  int result; // eax
  int v2; // kr00_4
  unsigned long long j; // rbx
  long long v4; // rdi
  unsigned long long i; // rbx
  long long v6; // rdi

  v2 = v0;
  result = dword_27338;
  switch ( dword_27338 )
  {
    case 0:
      for ( i = 0; qword_273B0 > i; result = sub_7460(byte_26019) )
      {
        sub_7C60();
        v6 = *((long long *)qword_273A0 + i++);
        sub_92B0(v6);
      }
      break;
    case 1:
      for ( j = 0; qword_273B0 > j; result = putchar_unlocked(byte_26019) )
      {
        v4 = *((long long *)qword_273A0 + j++);
        sub_9A80(v4, 0);
      }
      break;
    case 2:
      if ( !qword_272B0 )
        goto LABEL_13;
      result = sub_9E30();
      break;
    case 3:
      if ( qword_272B0 )
        result = sub_9D30();
      else
LABEL_13:
        result = sub_9C00(32);
      break;
    case 4:
      result = sub_9C00(44);
      break;
    default:
      result = v2;
      break;
  }
  return result;
}



// Function: xstrcoll_version @ 0xa050
long long xstrcoll_version(char **a1, char **a2)
{
  return sub_A010(*a1, *a2);
}



// Function: rev_xstrcoll_version @ 0xa060
long long rev_xstrcoll_version(char **a1, char **a2)
{
  return sub_A010(*a2, *a1);
}



// Function: rev_strcmp_size @ 0xa130
long long rev_strcmp_size(long long *a1, long long *a2)
{
  return sub_A100(*a2, a2[9], *a1, a1[9], &strcmp);
}



// Function: rev_xstrcoll_size @ 0xa150
long long rev_xstrcoll_size(long long *a1, long long *a2)
{
  return sub_A100(*a2, a2[9], *a1, a1[9], sub_72A0);
}



// Function: strcmp_size @ 0xa170
long long strcmp_size(long long *a1, long long *a2)
{
  return sub_A100(*a1, a1[9], *a2, a2[9], &strcmp);
}



// Function: xstrcoll_size @ 0xa1a0
long long xstrcoll_size(long long *a1, long long *a2)
{
  return sub_A100(*a1, a1[9], *a2, a2[9], sub_72A0);
}



// Function: extract_dirs_from_files @ 0xa1e0
void extract_dirs_from_files(void *a1, char a2)
{
  long long v2; // rbx
  void *v3; // r14
  bool v4; // cf
  long long v5; // rbp
  char *v6; // r14
  long long *v7; // rsi
  char *v8; // rdi
  char *v9; // rax
  long long v10; // rdx
  bool v11; // zf

  if ( a1 && qword_273C8 )
    sub_6840(0, a1, 0);
  v2 = qword_273B0 - 1;
  if ( !qword_273B0 )
    goto LABEL_18;
  do
  {
    while ( 1 )
    {
      v5 = *((long long *)qword_273A0 + v2);
      if ( !(unsigned char)sub_A1D0(*(unsigned int *)(v5 + 168)) )
        goto LABEL_9;
      v6 = *(char **)v5;
      if ( !a1 )
        break;
      if ( !(unsigned char)sub_73B0(*(long long *)v5) )
      {
        if ( *v6 == 47 )
          break;
        v3 = (void *)sub_E300(a1, v6, 0);
        sub_6840(v3, *(void **)(v5 + 8), a2);
        free(v3);
        if ( *(int *)(v5 + 168) == 9 )
          goto LABEL_13;
      }
LABEL_9:
      v4 = v2-- == 0;
      if ( v4 )
        goto LABEL_14;
    }
    sub_6840(v6, *(void **)(v5 + 8), a2);
    if ( *(int *)(v5 + 168) != 9 )
      goto LABEL_9;
LABEL_13:
    sub_6DC0(v5);
    v4 = v2-- == 0;
  }
  while ( !v4 );
LABEL_14:
  if ( !qword_273B0 )
  {
LABEL_18:
    qword_273B0 = 0;
    return;
  }
  v7 = qword_273A0;
  v8 = (char *)qword_273A0 + 8 * qword_273B0;
  v9 = (char *)qword_273A0;
  v10 = 0;
  do
  {
    v11 = *(int *)(*(long long *)v9 + 168LL) == 9;
    v7[v10] = *(long long *)v9;
    v9 += 8;
    v10 += !v11;
  }
  while ( v8 != v9 );
  qword_273B0 = v10;
}



// Function: do_statx @ 0xa400
long long do_statx(long long a1, long long a2, long long a3, unsigned int a4, unsigned int a5)
{
  char v6; // bh
  int v7; // r12d
  long long v9; // rdx
  char v10[80]; // [rsp+0h] [rbp-138h] BYREF
  long long v11; // [rsp+50h] [rbp-E8h]
  unsigned int v12; // [rsp+58h] [rbp-E0h]
  unsigned long long v13; // [rsp+108h] [rbp-30h]

  BYTE1(a4) |= 8u;
  v6 = BYTE1(a5);
  v13 = __readfsqword(0x28u);
  v7 = statx(a1, a2, a4, a5, v10);
  if ( v7 >= 0 )
  {
    sub_A340(v10, a3);
    if ( (v6 & 8) != 0 )
    {
      if ( (v10[1] & 8) != 0 )
      {
        *(long long *)(a3 + 88) = sub_A330(v11, v12);
        *(long long *)(a3 + 96) = v9;
      }
      else
      {
        *(long long *)(a3 + 96) = -1;
        *(long long *)(a3 + 88) = -1;
      }
    }
  }
  return (unsigned int)v7;
}



// Function: do_stat @ 0xa4c0
long long do_stat(long long a1, long long a2)
{
  unsigned int v2; // eax

  v2 = sub_68D0();
  return sub_A400(4294967196LL, a1, a2, 0, v2);
}



// Function: do_lstat @ 0xa4f0
long long do_lstat(long long a1, long long a2)
{
  unsigned int v2; // eax

  v2 = sub_68D0();
  return sub_A400(4294967196LL, a1, a2, 0x100u, v2);
}



// Function: stat_for_mode @ 0xa530
long long stat_for_mode(long long a1, long long a2)
{
  return sub_A400(4294967196LL, a1, a2, 0, 2u);
}



// Function: fstat_for_ino @ 0xb140
long long fstat_for_ino(long long a1, long long a2)
{
  return sub_A400(a1, (long long)"", a2, 0x1000u, 0x100u);
}



// Function: stat_for_ino @ 0xb160
long long stat_for_ino(long long a1, long long a2)
{
  return sub_A400(4294967196LL, a1, a2, 0, 0x100u);
}



// Function: print_dir @ 0xb180
int print_dir(char *a1, char *a2, char a3)
{
  int *v5; // rax
  int *v6; // rbx
  DIR *v7; // rax
  DIR *v8; // rbp
  int v9; // eax
  long long v10; // r15
  long long v11; // rsi
  char *v12; // r15
  char *d_name; // r14
  struct dirent *v14; // rax
  char *v15; // rax
  const char *v16; // rbp
  size_t v17; // rax
  const char *v18; // rbx
  char *v19; // rax
  long long v20; // rax
  char *v21; // rax
  char *v22; // rax
  long long v23; // r12
  char *v24; // rax
  char *v25; // rax
  char *v26; // rsi
  long long v28; // [rsp+0h] [rbp-378h]
  long long v30; // [rsp+10h] [rbp-368h] BYREF
  long long v31; // [rsp+18h] [rbp-360h]
  char v32[663]; // [rsp+A1h] [rbp-2D7h] BYREF
  unsigned long long v33; // [rsp+338h] [rbp-40h]

  v33 = __readfsqword(0x28u);
  v5 = __errno_location();
  *v5 = 0;
  v6 = v5;
  v7 = opendir(a1);
  if ( !v7 )
  {
    v26 = dcgettext(0, "cannot open directory %s", 5);
    LODWORD(v20) = sub_6F50(a3, v26, (long long)a1);
    return v20;
  }
  v8 = v7;
  if ( !qword_273C8 )
    goto LABEL_7;
  v9 = dirfd(v7);
  if ( v9 < 0 )
  {
    if ( (int)sub_B160((long long)a1, (long long)&v30) >= 0 )
      goto LABEL_5;
LABEL_33:
    v21 = dcgettext(0, "cannot determine device and inode of %s", 5);
    sub_6F50(a3, v21, (long long)a1);
    LODWORD(v20) = closedir(v8);
    return v20;
  }
  if ( (int)sub_B140((unsigned int)v9, (long long)&v30) < 0 )
    goto LABEL_33;
LABEL_5:
  v10 = v30;
  v11 = v31;
  if ( (unsigned char)sub_7DF0(v30, v31) )
  {
    v23 = sub_15700(0, 3, a1);
    v24 = dcgettext(0, "%s: not listing already-listed directory", 5);
    error(0, 0, v24, v23);
    closedir(v8);
    LODWORD(v20) = sub_59B0(1);
    return v20;
  }
  sub_7970(v10, v11);
LABEL_7:
  sub_6E20();
  if ( byte_272F6 )
  {
    if ( byte_26010 )
      goto LABEL_9;
LABEL_31:
    sub_7460(10);
LABEL_9:
    byte_26010 = 0;
    v12 = 0;
    sub_7810();
    if ( byte_27311 )
    {
      v12 = (char *)sub_DE90(a1, 2);
      if ( !v12 )
      {
        v25 = dcgettext(0, "error canonicalizing %s", 5);
        sub_6F50(a3, v25, (long long)a1);
      }
    }
    if ( !a2 )
      a2 = a1;
    sub_8EB0(a2, qword_272C8, 0xFFFFFFFF, 0, 1, (long long)&unk_27140, v12);
    free(v12);
    sub_77F0(":\n");
    goto LABEL_13;
  }
  if ( byte_272B8 )
  {
    if ( byte_26010 )
      goto LABEL_9;
    goto LABEL_31;
  }
LABEL_13:
  v28 = 0;
  while ( 1 )
  {
    *v6 = 0;
    v14 = readdir(v8);
    if ( !v14 )
      break;
    d_name = v14->d_name;
    if ( !(unsigned char)sub_69F0((long long)v14->d_name) )
    {
      v28 += sub_A550(d_name);
      if ( dword_27338 == 1 && dword_27330 == 6 && !byte_2732C && !byte_272F6 )
      {
        sub_89F0();
        sub_9F20();
        sub_6E20();
      }
    }
LABEL_16:
    sub_7CE0();
  }
  if ( *v6 )
  {
    v15 = dcgettext(0, "reading directory %s", 5);
    sub_6F50(a3, v15, (long long)a1);
    if ( *v6 == 75 )
      goto LABEL_16;
  }
  if ( closedir(v8) )
  {
    v22 = dcgettext(0, "closing directory %s", 5);
    sub_6F50(a3, v22, (long long)a1);
  }
  sub_89F0();
  if ( byte_272F6 )
    sub_A1E0(a1, 0);
  if ( !dword_27338 || byte_2732C )
  {
    v16 = (const char *)sub_FFA0(v28, v32, (unsigned int)dword_27328, 512, qword_27320);
    v17 = strlen(v16);
    *((char *)v16 - 1) = 32;
    v18 = &v16[v17];
    v16[v17] = byte_26019;
    sub_7810();
    v19 = dcgettext(0, "total", 5);
    sub_77F0(v19);
    sub_77D0(v16 - 1, v18 + 1 - (v16 - 1));
  }
  if ( qword_273B0 )
    LODWORD(v20) = sub_9F20();
  else
    return v33 - __readfsqword(0x28u);
  return v20;
}



// Function: usage @ 0xb690
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
  FILE *v25; // r12
  char *v26; // rax
  FILE *v27; // r12
  char *v28; // rax
  FILE *v29; // r12
  char *v30; // rax
  FILE *v31; // r12
  char *v32; // rax
  FILE *v33; // r12
  char *v34; // rax
  FILE *v35; // r12
  char *v36; // rax
  FILE *v37; // r12
  char *v38; // rax
  FILE *v39; // r12
  char *v40; // rax
  FILE *v41; // r12
  char *v42; // rax
  FILE *v43; // r12
  char *v44; // rax
  FILE *v45; // r12
  char *v46; // rax
  FILE *v47; // r12
  char *v48; // rax
  FILE *v49; // r12
  char *v50; // rax
  FILE *v51; // r12
  char *v52; // rax
  FILE *v53; // r12
  char *v54; // rax
  FILE *v55; // r12
  char *v56; // rax
  FILE *v57; // r12
  char *v58; // rax
  FILE *v59; // r12
  char *v60; // rax
  FILE *v61; // r12
  char *v62; // rax
  FILE *v63; // r12
  char *v64; // rax
  FILE *v65; // r12
  char *v66; // rax
  FILE *v67; // r12
  char *v68; // rax
  FILE *v69; // r12
  char *v70; // rax
  FILE *v71; // r12
  char *v72; // rax
  FILE *v73; // r12
  char *v74; // rax
  FILE *v75; // r12
  char *v76; // rax
  FILE *v77; // r12
  char *v78; // rax
  FILE *v79; // r12
  char *v80; // rax
  FILE *v81; // r12
  char *v82; // rax
  FILE *v83; // r12
  char *v84; // rax
  FILE *v85; // r12
  char *v86; // rax
  FILE *v87; // r12
  char *v88; // rax
  FILE *v89; // r12
  char *v90; // rax
  FILE *v91; // r12
  char *v92; // rax
  char *v93; // rdi

  v2 = qword_27400;
  if ( status )
  {
    v3 = dcgettext(0, "Try '%s --help' for more information.\n", 5);
    __fprintf_chk(stderr, 1, v3, v2);
  }
  else
  {
    v4 = dcgettext(0, "Usage: %s [OPTION]... [FILE]...\n", 5);
    __printf_chk(1, v4, v2);
    v5 = stdout;
    v6 = dcgettext(
           0,
           "List information about the FILEs (the current directory by default).\n"
           "Sort entries alphabetically if none of -cftuvSUX nor --sort is specified.\n",
           5);
    fputs_unlocked(v6, v5);
    sub_6530();
    v7 = stdout;
    v8 = dcgettext(
           0,
           "  -a, --all                  do not ignore entries starting with .\n"
           "  -A, --almost-all           do not list implied . and ..\n"
           "      --author               with -l, print the author of each file\n"
           "  -b, --escape               print C-style escapes for nongraphic characters\n",
           5);
    fputs_unlocked(v8, v7);
    v9 = stdout;
    v10 = dcgettext(
            0,
            "      --block-size=SIZE      with -l, scale sizes by SIZE when printing them;\n"
            "                             e.g., '--block-size=M'; see SIZE format below\n"
            "\n",
            5);
    fputs_unlocked(v10, v9);
    v11 = stdout;
    v12 = dcgettext(0, "  -B, --ignore-backups       do not list implied entries ending with ~\n", 5);
    fputs_unlocked(v12, v11);
    v13 = stdout;
    v14 = dcgettext(
            0,
            "  -c                         with -lt: sort by, and show, ctime (time of last\n"
            "                             modification of file status information);\n"
            "                             with -l: show ctime and sort by name;\n"
            "                             otherwise: sort by ctime, newest first\n"
            "\n",
            5);
    fputs_unlocked(v14, v13);
    v15 = stdout;
    v16 = dcgettext(
            0,
            "  -C                         list entries by columns\n"
            "      --color[=WHEN]         color the output WHEN; more info below\n"
            "  -d, --directory            list directories themselves, not their contents\n"
            "  -D, --dired                generate output designed for Emacs' dired mode\n",
            5);
    fputs_unlocked(v16, v15);
    v17 = stdout;
    v18 = dcgettext(
            0,
            "  -f                         list all entries in directory order\n"
            "  -F, --classify[=WHEN]      append indicator (one of */=>@|) to entries WHEN\n"
            "      --file-type            likewise, except do not append '*'\n",
            5);
    fputs_unlocked(v18, v17);
    v19 = stdout;
    v20 = dcgettext(
            0,
            "      --format=WORD          across -x, commas -m, horizontal -x, long -l,\n"
            "                             single-column -1, verbose -l, vertical -C\n"
            "\n",
            5);
    fputs_unlocked(v20, v19);
    v21 = stdout;
    v22 = dcgettext(0, "      --full-time            like -l --time-style=full-iso\n", 5);
    fputs_unlocked(v22, v21);
    v23 = stdout;
    v24 = dcgettext(0, "  -g                         like -l, but do not list owner\n", 5);
    fputs_unlocked(v24, v23);
    v25 = stdout;
    v26 = dcgettext(
            0,
            "      --group-directories-first\n"
            "                             group directories before files;\n"
            "                             can be augmented with a --sort option, but any\n"
            "                             use of --sort=none (-U) disables grouping\n"
            "\n",
            5);
    fputs_unlocked(v26, v25);
    v27 = stdout;
    v28 = dcgettext(0, "  -G, --no-group             in a long listing, don't print group names\n", 5);
    fputs_unlocked(v28, v27);
    v29 = stdout;
    v30 = dcgettext(
            0,
            "  -h, --human-readable       with -l and -s, print sizes like 1K 234M 2G etc.\n"
            "      --si                   likewise, but use powers of 1000 not 1024\n",
            5);
    fputs_unlocked(v30, v29);
    v31 = stdout;
    v32 = dcgettext(
            0,
            "  -H, --dereference-command-line\n"
            "                             follow symbolic links listed on the command line\n",
            5);
    fputs_unlocked(v32, v31);
    v33 = stdout;
    v34 = dcgettext(
            0,
            "      --dereference-command-line-symlink-to-dir\n"
            "                             follow each command line symbolic link\n"
            "                             that points to a directory\n"
            "\n",
            5);
    fputs_unlocked(v34, v33);
    v35 = stdout;
    v36 = dcgettext(
            0,
            "      --hide=PATTERN         do not list implied entries matching shell PATTERN\n"
            "                             (overridden by -a or -A)\n"
            "\n",
            5);
    fputs_unlocked(v36, v35);
    v37 = stdout;
    v38 = dcgettext(0, "      --hyperlink[=WHEN]     hyperlink file names WHEN\n", 5);
    fputs_unlocked(v38, v37);
    v39 = stdout;
    v40 = dcgettext(
            0,
            "      --indicator-style=WORD\n"
            "                             append indicator with style WORD to entry names:\n"
            "                             none (default), slash (-p),\n"
            "                             file-type (--file-type), classify (-F)\n"
            "\n",
            5);
    fputs_unlocked(v40, v39);
    v41 = stdout;
    v42 = dcgettext(
            0,
            "  -i, --inode                print the index number of each file\n"
            "  -I, --ignore=PATTERN       do not list implied entries matching shell PATTERN\n",
            5);
    fputs_unlocked(v42, v41);
    v43 = stdout;
    v44 = dcgettext(
            0,
            "  -k, --kibibytes            default to 1024-byte blocks for file system usage;\n"
            "                             used only with -s and per directory totals\n"
            "\n",
            5);
    fputs_unlocked(v44, v43);
    v45 = stdout;
    v46 = dcgettext(0, "  -l                         use a long listing format\n", 5);
    fputs_unlocked(v46, v45);
    v47 = stdout;
    v48 = dcgettext(
            0,
            "  -L, --dereference          when showing file information for a symbolic\n"
            "                             link, show information for the file the link\n"
            "                             references rather than for the link itself\n"
            "\n",
            5);
    fputs_unlocked(v48, v47);
    v49 = stdout;
    v50 = dcgettext(0, "  -m                         fill width with a comma separated list of entries\n", 5);
    fputs_unlocked(v50, v49);
    v51 = stdout;
    v52 = dcgettext(
            0,
            "  -n, --numeric-uid-gid      like -l, but list numeric user and group IDs\n"
            "  -N, --literal              print entry names without quoting\n"
            "  -o                         like -l, but do not list group information\n"
            "  -p, --indicator-style=slash\n"
            "                             append / indicator to directories\n",
            5);
    fputs_unlocked(v52, v51);
    v53 = stdout;
    v54 = dcgettext(0, "  -q, --hide-control-chars   print ? instead of nongraphic characters\n", 5);
    fputs_unlocked(v54, v53);
    v55 = stdout;
    v56 = dcgettext(
            0,
            "      --show-control-chars   show nongraphic characters as-is (the default,\n"
            "                             unless program is 'ls' and output is a terminal)\n"
            "\n",
            5);
    fputs_unlocked(v56, v55);
    v57 = stdout;
    v58 = dcgettext(0, "  -Q, --quote-name           enclose entry names in double quotes\n", 5);
    fputs_unlocked(v58, v57);
    v59 = stdout;
    v60 = dcgettext(
            0,
            "      --quoting-style=WORD   use quoting style WORD for entry names:\n"
            "                             literal, locale, shell, shell-always,\n"
            "                             shell-escape, shell-escape-always, c, escape\n"
            "                             (overrides QUOTING_STYLE environment variable)\n"
            "\n",
            5);
    fputs_unlocked(v60, v59);
    v61 = stdout;
    v62 = dcgettext(
            0,
            "  -r, --reverse              reverse order while sorting\n"
            "  -R, --recursive            list subdirectories recursively\n"
            "  -s, --size                 print the allocated size of each file, in blocks\n",
            5);
    fputs_unlocked(v62, v61);
    v63 = stdout;
    v64 = dcgettext(0, "  -S                         sort by file size, largest first\n", 5);
    fputs_unlocked(v64, v63);
    v65 = stdout;
    v66 = dcgettext(
            0,
            "      --sort=WORD            sort by WORD instead of name: none (-U), size (-S),\n"
            "                             time (-t), version (-v), extension (-X), width\n"
            "\n",
            5);
    fputs_unlocked(v66, v65);
    v67 = stdout;
    v68 = dcgettext(
            0,
            "      --time=WORD            change the default of using modification times;\n"
            "                               access time (-u): atime, access, use;\n"
            "                               change time (-c): ctime, status;\n"
            "                               birth time: birth, creation;\n"
            "                             with -l, WORD determines which time to show;\n"
            "                             with --sort=time, sort by WORD (newest first)\n"
            "\n",
            5);
    fputs_unlocked(v68, v67);
    v69 = stdout;
    v70 = dcgettext(
            0,
            "      --time-style=TIME_STYLE\n"
            "                             time/date format with -l; see TIME_STYLE below\n",
            5);
    fputs_unlocked(v70, v69);
    v71 = stdout;
    v72 = dcgettext(
            0,
            "  -t                         sort by time, newest first; see --time\n"
            "  -T, --tabsize=COLS         assume tab stops at each COLS instead of 8\n",
            5);
    fputs_unlocked(v72, v71);
    v73 = stdout;
    v74 = dcgettext(
            0,
            "  -u                         with -lt: sort by, and show, access time;\n"
            "                             with -l: show access time and sort by name;\n"
            "                             otherwise: sort by access time, newest first\n"
            "\n",
            5);
    fputs_unlocked(v74, v73);
    v75 = stdout;
    v76 = dcgettext(0, "  -U                         do not sort; list entries in directory order\n", 5);
    fputs_unlocked(v76, v75);
    v77 = stdout;
    v78 = dcgettext(0, "  -v                         natural sort of (version) numbers within text\n", 5);
    fputs_unlocked(v78, v77);
    v79 = stdout;
    v80 = dcgettext(
            0,
            "  -w, --width=COLS           set output width to COLS.  0 means no limit\n"
            "  -x                         list entries by lines instead of by columns\n"
            "  -X                         sort alphabetically by entry extension\n"
            "  -Z, --context              print any security context of each file\n"
            "      --zero                 end each output line with NUL, not newline\n"
            "  -1                         list one file per line\n",
            5);
    fputs_unlocked(v80, v79);
    v81 = stdout;
    v82 = dcgettext(0, "      --help        display this help and exit\n", 5);
    fputs_unlocked(v82, v81);
    v83 = stdout;
    v84 = dcgettext(0, "      --version     output version information and exit\n", 5);
    fputs_unlocked(v84, v83);
    sub_6560();
    v85 = stdout;
    v86 = dcgettext(
            0,
            "\n"
            "The TIME_STYLE argument can be full-iso, long-iso, iso, locale, or +FORMAT.\n"
            "FORMAT is interpreted like in date(1).  If FORMAT is FORMAT1<newline>FORMAT2,\n"
            "then FORMAT1 applies to non-recent files and FORMAT2 to recent files.\n"
            "TIME_STYLE prefixed with 'posix-' takes effect only outside the POSIX locale.\n"
            "Also the TIME_STYLE environment variable sets the default style to use.\n",
            5);
    fputs_unlocked(v86, v85);
    v87 = stdout;
    v88 = dcgettext(0, "\nThe WHEN argument defaults to 'always' and can also be 'auto' or 'never'.\n", 5);
    fputs_unlocked(v88, v87);
    v89 = stdout;
    v90 = dcgettext(
            0,
            "\n"
            "Using color to distinguish file types is disabled both by default and\n"
            "with --color=never.  With --color=auto, ls emits color codes only when\n"
            "standard output is connected to a terminal.  The LS_COLORS environment\n"
            "variable can change the settings.  Use the dircolors(1) command to set it.\n",
            5);
    fputs_unlocked(v90, v89);
    v91 = stdout;
    v92 = dcgettext(
            0,
            "\n"
            "Exit status:\n"
            " 0  if OK,\n"
            " 1  if minor problems (e.g., cannot access subdirectory),\n"
            " 2  if serious trouble (e.g., cannot access command-line argument).\n",
            5);
    fputs_unlocked(v92, v91);
    v93 = "ls";
    if ( dword_261E0 != 1 )
    {
      v93 = (char *)"dir";
      if ( dword_261E0 != 2 )
        v93 = "vdir";
    }
    sub_7E40(v93);
  }
  exit(status);
}



// Function: decode_switches @ 0xbda0
long long decode_switches(unsigned int argc, char **argv)
{
  int *v3; // rsi
  int v4; // eax
  long long v5; // r10
  long long v6; // rsi
  long long v7; // rdx
  char v8; // al
  long long *v9; // rax
  long long v10; // rdx
  long long v11; // rsi
  long long v12; // rdx
  char v13; // al
  unsigned int v14; // eax
  const char *v15; // rbx
  char *v16; // rax
  int v17; // eax
  int v18; // eax
  long long v19; // rsi
  unsigned int v20; // eax
  char v21; // al
  int v22; // ebx
  long long v23; // rax
  char *v24; // rbx
  char i; // al
  int v26; // eax
  long long v28; // rax
  char *j; // rbp
  const char **v30; // rbx
  long long v31; // rax
  int v32; // eax
  char v33; // al
  char *v34; // rbx
  char *v35; // rax
  char *v36; // rbx
  char *v37; // r12
  long long v38; // rsi
  char *v39; // rax
  long long v40; // r12
  char *v41; // rax
  char *v42; // rax
  FILE *v43; // rbp
  char *v44; // rax
  const char *v45; // rcx
  FILE *v46; // rbp
  char *v47; // rax
  char *v48; // rsi
  long long v49; // r12
  char *v50; // rax
  char *v51; // rax
  long long v52; // r12
  char *v53; // rax
  long long v54; // [rsp-8h] [rbp-80h]
  int v55; // [rsp+4h] [rbp-74h]
  int v56; // [rsp+8h] [rbp-70h]
  int v57; // [rsp+Ch] [rbp-6Ch]
  long long v58; // [rsp+10h] [rbp-68h]
  char *s1; // [rsp+18h] [rbp-60h]
  long long v60; // [rsp+20h] [rbp-58h]
  int v61; // [rsp+28h] [rbp-50h]
  char v62; // [rsp+2Fh] [rbp-49h]
  int longind[2]; // [rsp+30h] [rbp-48h] BYREF
  unsigned long long v64; // [rsp+38h] [rbp-40h]

  v64 = __readfsqword(0x28u);
  v57 = -1;
  v58 = -1;
  v60 = -1;
  v56 = -1;
  v61 = -1;
  v55 = -1;
  v62 = 0;
  s1 = 0;
  while ( 2 )
  {
    v3 = (int *)argv;
    longind[0] = -1;
    v4 = getopt_long(argc, argv, "abcdfghiklmnopqrstuvw:xABCDFGHI:LNQRST:UXZ1", &longopts, longind);
    if ( v4 != -1 )
    {
      switch ( v4 )
      {
        case -131:
          v48 = "ls";
          if ( dword_261E0 != 1 )
          {
            v48 = (char *)"dir";
            if ( dword_261E0 != 2 )
              v48 = "vdir";
          }
          sub_16870(
            (int)stdout,
            (int)v48,
            (unsigned int)"GNU coreutils",
            (int)off_261E8,
            (unsigned int)"Richard M. Stallman",
            (unsigned int)"David MacKenzie",
            0,
            argc);
          exit(0);
        case -130:
          sub_B690(0);
        case 49:
          v55 = v55 != 0;
          continue;
        case 65:
          dword_272F0 = 1;
          continue;
        case 66:
          sub_6380((long long)"*~");
          sub_6380((long long)".*~");
          continue;
        case 67:
          v55 = 2;
          continue;
        case 68:
          byte_27318 = 1;
          continue;
        case 70:
          v19 = *(long long *)&optarg;
          if ( !*(long long *)&optarg
            || (v54 = v5,
                v20 = dword_196C0[sub_D270("--classify", *(long long *)&optarg, &off_251A0, dword_196C0, 4, off_261F0, 1)],
                v20 == 1)
            || v20 == 2 && (unsigned char)sub_6590("--classify", v19, v54) )
          {
            dword_27314 = 3;
          }
          continue;
        case 71:
          byte_26028 = 0;
          continue;
        case 72:
          dword_272F8 = 2;
          continue;
        case 73:
          sub_6380(*(long long *)&optarg);
          continue;
        case 76:
          dword_272F8 = 4;
          continue;
        case 78:
          v56 = 0;
          continue;
        case 81:
          v56 = 5;
          continue;
        case 82:
          byte_272F6 = 1;
          continue;
        case 83:
          v57 = 3;
          continue;
        case 84:
          v18 = (unsigned int)dcgettext(0, "invalid tab size", 5);
          v60 = sub_16DE0(optarg, 0, 0, -1, (int)"", v18, 2);
          continue;
        case 85:
          v57 = 6;
          continue;
        case 88:
          v57 = 1;
          continue;
        case 90:
          byte_27365 = 1;
          continue;
        case 97:
          dword_272F0 = 2;
          continue;
        case 98:
          v56 = 7;
          continue;
        case 99:
          dword_27334 = 1;
          continue;
        case 100:
          byte_272F5 = 1;
          continue;
        case 102:
          v17 = -1;
          dword_272F0 = 2;
          byte_27312 = 0;
          byte_27311 = 0;
          if ( v55 )
            v17 = v55;
          byte_2732C = 0;
          v57 = 6;
          v55 = v17;
          continue;
        case 103:
          byte_26029 = 0;
          v55 = 0;
          continue;
        case 104:
          dword_27328 = 176;
          dword_2731C = 176;
          qword_27320 = 1;
          qword_26020 = 1;
          continue;
        case 105:
          byte_272FC = 1;
          continue;
        case 107:
          v62 = 1;
          continue;
        case 108:
          v55 = 0;
          continue;
        case 109:
          v55 = 4;
          continue;
        case 110:
          byte_2732D = 1;
          v55 = 0;
          continue;
        case 111:
          byte_26028 = 0;
          v55 = 0;
          continue;
        case 112:
          dword_27314 = 1;
          continue;
        case 113:
          v61 = 1;
          continue;
        case 114:
          byte_2732F = 1;
          continue;
        case 115:
          byte_2732C = 1;
          continue;
        case 116:
          v57 = 5;
          continue;
        case 117:
          dword_27334 = 2;
          continue;
        case 118:
          goto LABEL_32;
        case 119:
          v58 = sub_63B0(*(char **)&optarg);
          if ( v58 >= 0 )
            continue;
          v15 = (const char *)sub_158C0(*(long long *)&optarg, argv);
          v16 = dcgettext(0, "invalid line width", 5);
          error(2, 0, "%s: %s", v16, v15);
LABEL_32:
          v57 = 4;
          continue;
        case 120:
          v55 = 3;
          continue;
        case 128:
          byte_2732E = 1;
          continue;
        case 129:
          v14 = sub_10810(*(long long *)&optarg, &dword_27328, &qword_27320);
          if ( v14 )
            sub_17110(v14, (unsigned int)longind[0], 0, &longopts, *(long long *)&optarg);
          dword_2731C = dword_27328;
          qword_26020 = qword_27320;
          continue;
        case 130:
          v11 = *(long long *)&optarg;
          if ( !*(long long *)&optarg
            || (v12 = dword_196C0[sub_D270("--color", *(long long *)&optarg, &off_251A0, dword_196C0, 4, off_261F0, 1)],
                (int)v12 == 1) )
          {
            v13 = 1;
          }
          else
          {
            v13 = 0;
            if ( (int)v12 == 2 )
              v13 = sub_6590("--color", v11, v12);
          }
          byte_27312 = v13 & 1;
          continue;
        case 131:
          dword_272F8 = 3;
          continue;
        case 132:
          dword_27314 = 2;
          continue;
        case 133:
          v55 = dword_19730[sub_D270("--format", *(long long *)&optarg, off_25280, dword_19730, 4, off_261F0, 1)];
          continue;
        case 134:
          v55 = 0;
          s1 = (char *)sub_55B0("full-iso");
          continue;
        case 135:
          byte_272F4 = 1;
          continue;
        case 136:
          v9 = (long long *)sub_169E0(16);
          *v9 = *(long long *)&optarg;
          v10 = qword_272E0;
          qword_272E0 = (long long)v9;
          v9[1] = v10;
          continue;
        case 137:
          v6 = *(long long *)&optarg;
          if ( !*(long long *)&optarg
            || (v7 = dword_196C0[sub_D270("--hyperlink", *(long long *)&optarg, &off_251A0, dword_196C0, 4, off_261F0, 1)],
                (int)v7 == 1) )
          {
            v8 = 1;
          }
          else
          {
            v8 = 0;
            if ( (int)v7 == 2 )
              v8 = sub_6590("--hyperlink", v6, v7);
          }
          byte_27311 = v8 & 1;
          continue;
        case 138:
          dword_27314 = dword_19750[sub_D270(
                                      "--indicator-style",
                                      *(long long *)&optarg,
                                      off_25940,
                                      dword_19750,
                                      4,
                                      off_261F0,
                                      1)];
          continue;
        case 139:
          v56 = dword_1D9C0[sub_D270("--quoting-style", *(long long *)&optarg, off_259E0, dword_1D9C0, 4, off_261F0, 1)];
          continue;
        case 140:
          goto LABEL_5;
        case 141:
          dword_27328 = 144;
          dword_2731C = 144;
          qword_27320 = 1;
          qword_26020 = 1;
          continue;
        case 142:
          v57 = dword_19710[sub_D270("--sort", *(long long *)&optarg, &off_25240, dword_19710, 4, off_261F0, 1)];
          continue;
        case 143:
          dword_27334 = dword_196F0[sub_D270("--time", *(long long *)&optarg, off_25200, dword_196F0, 4, off_261F0, 1)];
          continue;
        case 144:
          s1 = *(char **)&optarg;
          continue;
        case 145:
          byte_26019 = 0;
          byte_27312 = 0;
          v56 = 0;
          v55 = v55 != 0;
LABEL_5:
          v61 = 0;
          continue;
        default:
          goto LABEL_169;
      }
    }
    break;
  }
  if ( !qword_27320 )
  {
    v3 = &dword_27328;
    v34 = getenv("LS_BLOCK_SIZE");
    sub_10810(v34, &dword_27328, &qword_27320);
    if ( v34 || getenv("BLOCK_SIZE") )
    {
      dword_2731C = dword_27328;
      qword_26020 = qword_27320;
    }
    if ( v62 )
    {
      dword_27328 = 0;
      qword_27320 = 1024;
    }
  }
  if ( v55 >= 0 )
  {
    dword_27338 = v55;
    if ( (unsigned int)(v55 - 2) <= 2 )
      goto LABEL_81;
    goto LABEL_125;
  }
  if ( dword_261E0 == 1 )
  {
    if ( !(unsigned char)((long long (*)(void))sub_6590)() )
    {
      v32 = 1;
      goto LABEL_124;
    }
LABEL_154:
    dword_27338 = 2;
    goto LABEL_81;
  }
  if ( dword_261E0 == 2 )
    goto LABEL_154;
  v32 = 0;
LABEL_124:
  dword_27338 = v32;
LABEL_125:
  if ( !byte_27312 )
  {
    if ( v58 != -1 )
      goto LABEL_82;
    goto LABEL_127;
  }
LABEL_81:
  if ( v58 != -1 )
    goto LABEL_82;
  if ( (unsigned char)((long long (*)(void))sub_6590)() )
  {
    v3 = (int *)(&loc_540F + 4);
    if ( ioctl(1, 0x5413u, longind) >= 0 )
    {
      if ( HIWORD(longind[0]) )
      {
        v58 = HIWORD(longind[0]);
        goto LABEL_82;
      }
    }
  }
  v39 = getenv("COLUMNS");
  argv = (char **)v39;
  if ( !v39 || !*v39 )
  {
LABEL_127:
    v58 = 80;
    goto LABEL_82;
  }
  v58 = sub_63B0(v39);
  if ( v58 < 0 )
  {
    v40 = sub_158C0(argv, v3);
    v41 = dcgettext(0, "ignoring invalid width in environment variable COLUMNS: %s", 5);
    error(0, 0, v41, v40);
    v58 = 80;
  }
LABEL_82:
  qword_272B0 = v58;
  qword_27200 = v58 / 3uLL
              + (v58 != v58 / 3uLL
                      + (((0xAAAAAAAAAAAAAAABLL * (unsigned __int128)(unsigned long long)v58) >> 64) & 0xFFFFFFFFFFFFFFFELL));
  if ( (unsigned int)(dword_27338 - 2) > 2 )
    goto LABEL_83;
  v28 = v60;
  if ( v60 >= 0 )
    goto LABEL_107;
  qword_272C0 = 8;
  v42 = getenv("TABSIZE");
  argv = (char **)v42;
  if ( v42 )
  {
    if ( !(unsigned int)sub_17190(v42) )
    {
      v28 = *(long long *)longind;
LABEL_107:
      qword_272C0 = v28;
      goto LABEL_83;
    }
    v49 = sub_158C0(argv, 0);
    v50 = dcgettext(0, "ignoring invalid tab size in environment variable TABSIZE: %s", 5);
    error(0, 0, v50, v49);
  }
LABEL_83:
  v21 = v61 & 1;
  if ( v61 == -1 )
  {
    v33 = 0;
    if ( dword_261E0 == 1 )
      v33 = ((long long (*)(void))sub_6590)();
    v21 = v33 & 1;
  }
  byte_272D8 = v21;
  if ( v56 >= 0 )
    goto LABEL_85;
  v56 = sub_64B0();
  if ( v56 >= 0 )
    goto LABEL_85;
  v56 = 7;
  if ( dword_261E0 != 1 )
    goto LABEL_85;
  if ( (unsigned char)((long long (*)(void))sub_6590)() )
  {
    v56 = 3;
LABEL_85:
    sub_15200(0, (unsigned int)v56);
  }
  v22 = sub_151E0(0);
  if ( (!dword_27338 || (unsigned int)(dword_27338 - 2) <= 1 && qword_272B0) && (v22 == 3 || v22 == 6 || v22 == 1) )
  {
    byte_273A8 = 1;
    qword_272D0 = sub_151A0(0);
  }
  else
  {
    byte_273A8 = 0;
    v23 = sub_151A0(0);
    qword_272D0 = v23;
    if ( v22 == 7 )
      sub_15220(v23, 32, 1);
  }
  if ( (unsigned int)dword_27314 > 1 )
  {
    v24 = &asc_1AD24[dword_27314 - 2];
    for ( i = *v24; *v24; i = *v24 )
    {
      ++v24;
      sub_15220(qword_272D0, (unsigned int)i, 1);
    }
  }
  qword_272C8 = sub_151A0(0);
  sub_15220(qword_272C8, 58, 1);
  byte_27318 &= (dword_27338 == 0) & (byte_27311 ^ 1);
  if ( (unsigned char)byte_27318 > byte_26019 )
  {
    v51 = dcgettext(0, "--dired and --zero are incompatible", 5);
    v38 = 0;
    error(2, 0, v51);
    goto LABEL_182;
  }
  v26 = v57;
  if ( v57 < 0 )
  {
    if ( !dword_27338 )
    {
      dword_27330 = 0;
      goto LABEL_110;
    }
    if ( (unsigned int)(dword_27334 - 1) <= 2 )
    {
      dword_27330 = 5;
      return (unsigned int)optind;
    }
    v26 = 0;
  }
  dword_27330 = v26;
  if ( !dword_27338 )
  {
LABEL_110:
    if ( !s1 )
    {
      s1 = getenv("TIME_STYLE");
      if ( !s1 )
        s1 = (char *)sub_55B0("locale");
    }
    for ( j = s1; !strncmp(j, "posix-", 6u); j += 6 )
    {
      if ( !(unsigned char)sub_E9C0(2) )
        return (unsigned int)optind;
    }
    if ( *j != 43 )
    {
      v30 = (const char **)off_25980;
      v31 = sub_CF70(j);
      if ( v31 < 0 )
      {
        sub_D0E0("time style", j, v31);
        v43 = stderr;
        v44 = dcgettext(0, "Valid arguments are:\n", 5);
        fputs_unlocked(v44, v43);
        v45 = "full-iso";
        do
        {
          ++v30;
          __fprintf_chk(stderr, 1, "  - [posix-]%s\n", v45);
          v45 = *v30;
        }
        while ( *v30 );
        v46 = stderr;
        v47 = dcgettext(0, "  - +FORMAT (e.g., +%H:%M) for a 'date'-style format\n", 5);
        fputs_unlocked(v47, v46);
LABEL_169:
        sub_B690(2);
      }
      if ( v31 == 2 )
      {
        off_26040 = "%Y-%m-%d ";
        off_26048 = "%m-%d %H:%M";
      }
      else if ( v31 > 2 )
      {
        if ( v31 == 3 && (unsigned char)sub_E9C0(2) )
        {
          off_26040 = dcgettext(0, off_26040, 2);
          off_26048 = dcgettext(0, off_26048, 2);
        }
      }
      else if ( v31 )
      {
        off_26048 = "%Y-%m-%d %H:%M";
        off_26040 = "%Y-%m-%d %H:%M";
      }
      else
      {
        off_26048 = "%Y-%m-%d %H:%M:%S.%N %z";
        off_26040 = "%Y-%m-%d %H:%M:%S.%N %z";
      }
      goto LABEL_144;
    }
    argv = (char **)(j + 1);
    v35 = strchr((const char *)argv, 10);
    v36 = v35;
    if ( !v35 )
    {
      v37 = (char *)argv;
      goto LABEL_143;
    }
    v37 = v35 + 1;
    v38 = 10;
    if ( !strchr(v35 + 1, 10) )
    {
      *v36 = 0;
LABEL_143:
      off_26040 = (char *)argv;
      off_26048 = v37;
LABEL_144:
      sub_66C0();
      return (unsigned int)optind;
    }
LABEL_182:
    v52 = sub_158C0(argv, v38);
    v53 = dcgettext(0, "invalid time style format %s", 5);
    error(2, 0, v53, v52);
  }
  return (unsigned int)optind;
}



// Function: is_prime @ 0xea50
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



// Function: next_prime @ 0xeac0
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
      if ( sub_EA50(v2) )
        break;
      v2 += 2LL;
    }
    while ( v2 != -1 );
  }
  return v2;
}



// Function: raw_hasher @ 0xeb00
unsigned long long raw_hasher(long long a1, unsigned long long a2)
{
  return sub_17EB0(a1, 3) % a2;
}



// Function: raw_comparator @ 0xeb20
bool raw_comparator(long long a1, long long a2)
{
  return a2 == a1;
}



// Function: check_tuning @ 0xeb30
long long check_tuning(long long a1)
{
  long long v1; // rax
  float v2; // xmm0_4
  float v3; // xmm1_4
  float v4; // xmm2_4
  long long result; // rax

  v1 = *(long long *)(a1 + 40);
  if ( (_UNKNOWN *)v1 == &unk_1D2F0 )
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
    *(long long *)(a1 + 40) = &unk_1D2F0;
    return 0;
  }
  return result;
}



// Function: free_entry @ 0xebc0
long long free_entry(long long a1, long long *a2)
{
  long long result; // rax

  result = *(long long *)(a1 + 72);
  *a2 = 0;
  a2[1] = result;
  *(long long *)(a1 + 72) = a2;
  return result;
}



// Function: safe_hasher @ 0xebe0
long long safe_hasher(long long a1, long long a2)
{
  unsigned long long v2; // rax

  v2 = (*(long long (**)(long long, long long))(a1 + 48))(a2, *(long long *)(a1 + 16));
  if ( *(long long *)(a1 + 16) <= v2 )
    abort();
  return *(long long *)a1 + 16 * v2;
}



// Function: hash_find_entry @ 0xec10
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

  v6 = (__m128i *)sub_EBE0(a1, a2);
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
          sub_EBC0(a1, v11);
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
      sub_EBC0(a1, v9);
      return v12;
    }
    else
    {
      v7->m128i_i64[0] = 0;
    }
  }
  return result;
}



// Function: allocate_entry @ 0xed10
long long *allocate_entry(long long a1)
{
  long long *result; // rax

  result = *(long long **)(a1 + 72);
  if ( !result )
    return malloc(0x10u);
  *(long long *)(a1 + 72) = result[1];
  return result;
}



// Function: transfer_entries @ 0xed40
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
              v9 = (long long *)sub_EBE0(a1, *v7);
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
            sub_EBC0(a1, v10);
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
    v11 = (long long *)sub_EBE0(a1, v6);
    v12 = v11;
    if ( !*v11 )
    {
      *v11 = v6;
      ++*(long long *)(a1 + 24);
      goto LABEL_15;
    }
    v13 = sub_ED10(a1);
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



// Function: hash_get_n_buckets @ 0xef00
long long hash_get_n_buckets(long long a1)
{
  return *(long long *)(a1 + 16);
}



// Function: hash_get_n_buckets_used @ 0xef10
long long hash_get_n_buckets_used(long long a1)
{
  return *(long long *)(a1 + 24);
}



// Function: hash_get_n_entries @ 0xef20
long long hash_get_n_entries(long long a1)
{
  return *(long long *)(a1 + 32);
}



// Function: hash_get_max_bucket_length @ 0xef30
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



// Function: hash_print_statistics @ 0xf000
long long hash_print_statistics(long long a1, long long a2)
{
  long long v2; // rbx
  long long v3; // r13
  unsigned long long v4; // r12
  long long v5; // r9
  double v6; // xmm0_8
  double v7; // xmm1_8

  sub_EF20(a1);
  v2 = sub_EF00(a1);
  v3 = sub_EF10(a1);
  v4 = sub_EF30(a1);
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



// Function: hash_lookup @ 0xf120
long long hash_lookup(long long a1, long long a2)
{
  long long v2; // rbp
  long long *v3; // rax
  long long v4; // rsi
  long long *v5; // rbx

  v2 = a2;
  v3 = (long long *)sub_EBE0(a1, a2);
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



// Function: hash_get_next @ 0xf1d0
long long hash_get_next(long long a1, long long a2)
{
  long long *v2; // rax
  long long *v3; // rdx
  long long v4; // rcx
  long long v5; // r8

  v2 = (long long *)sub_EBE0(a1, a2);
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



// Function: hash_do_for_each @ 0xf290
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



// Function: hash_initialize @ 0xf380
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
    v7 = sub_EB00;
  if ( !a4 )
    v8 = sub_EB20;
  v9 = malloc(0x50u);
  if ( v9 )
  {
    if ( !a2 )
      v5 = (float *)&unk_1D2F0;
    v9[5] = v5;
    if ( (unsigned char)sub_EB30((long long)v9)
      && (v10 = sub_EE70(a1, *((unsigned char *)v5 + 16), v5[2]), (v9[2] = v10) != 0)
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



// Function: hash_clear @ 0xf490
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



// Function: hash_free @ 0xf540
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



// Function: hash_rehash @ 0xf630
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
  v2 = sub_EE70(a2, *(unsigned char *)(*(long long *)(a1 + 40) + 16LL), *(float *)(*(long long *)(a1 + 40) + 8LL));
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
  v6 = sub_ED40((long long)&ptr, a1, 0);
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
    if ( !(unsigned char)sub_ED40(a1, (long long)&ptr, 1) || !(unsigned char)sub_ED40(a1, (long long)&ptr, 0) )
      abort();
    free(ptr);
  }
  return v6;
}



// Function: hash_insert_if_absent @ 0xf7b0
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
  v5 = sub_EC10((long long)a1, a2, v22, 0);
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
      sub_EB30((long long)a1);
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
        if ( !(unsigned char)sub_F630((long long)a1, v21) )
          return (unsigned int)-1;
        if ( sub_EC10((long long)a1, a2, v22, 0) )
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
    v12 = sub_ED10((long long)a1);
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



// Function: hash_insert @ 0xf9a0
long long hash_insert(long long *a1, long long a2)
{
  int v2; // eax
  bool v3; // zf
  long long result; // rax
  long long v5[3]; // [rsp+0h] [rbp-18h] BYREF

  v5[1] = __readfsqword(0x28u);
  v2 = sub_F7B0(a1, a2, v5);
  if ( v2 == -1 )
    return 0;
  v3 = v2 == 0;
  result = a2;
  if ( v3 )
    return v5[0];
  return result;
}



// Function: hash_remove @ 0xfa00
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
  v3 = sub_EC10((long long)a1, a2, v18, 1);
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
    sub_EB30((long long)a1);
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
      if ( !(unsigned char)sub_F630((long long)a1, v14) )
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


