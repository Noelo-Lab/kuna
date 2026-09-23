// Function: file_or_link_mode @ 0x53e7
long long file_or_link_mode(long long a1)
{
  if ( byte_2B358 && *(char *)(a1 + 185) )
    return *(unsigned int *)(a1 + 172);
  else
    return *(unsigned int *)(a1 + 48);
}



// Function: dired_outbyte @ 0x5422
int dired_outbyte(char a1)
{
  ++qword_2B500;
  return putchar_unlocked(a1);
}



// Function: dired_outbuf @ 0x5453
size_t dired_outbuf(const void *a1, size_t a2)
{
  qword_2B500 += a2;
  return fwrite_unlocked(a1, 1u, a2, stdout);
}



// Function: dired_outstring @ 0x549e
size_t dired_outstring(const char *a1)
{
  size_t v1; // rax

  v1 = strlen(a1);
  return sub_5453(a1, v1);
}



// Function: dired_indent @ 0x54cf
size_t dired_indent()
{
  size_t result; // rax

  result = (unsigned char)byte_2B3CC;
  if ( byte_2B3CC )
    return sub_549E("  ");
  return result;
}



// Function: push_current_dired_pos @ 0x54f4
long long push_current_dired_pos(long long a1)
{
  long long result; // rax

  result = (unsigned char)byte_2B3CC;
  if ( byte_2B3CC )
  {
    if ( *(long long *)(a1 + 32) - *(long long *)(a1 + 24) < 8u )
      obstack_newchunk(a1, 8);
    memcpy(*(void **)(a1 + 24), &qword_2B500, 8u);
    *(long long *)(a1 + 24) += 8LL;
    return a1;
  }
  return result;
}



// Function: dev_ino_push @ 0x558e
long long *dev_ino_push(long long a1, long long a2)
{
  long long *v3; // [rsp+38h] [rbp-8h]

  if ( (unsigned long long)(unk_2B600 - qword_2B5F8) < 0x10 )
    obstack_newchunk(&unk_2B5E0, 16);
  qword_2B5F8 += 16;
  v3 = (long long *)(qword_2B5F8 - 16);
  *(long long *)(qword_2B5F8 - 16 + 8) = a1;
  *v3 = a2;
  return v3;
}



// Function: dev_ino_pop @ 0x563d
long long dev_ino_pop()
{
  if ( (unsigned long long)(qword_2B5F8 - unk_2B5F0) < 0x10 )
    __assert_fail("dev_ino_size <= obstack_object_size (&dev_ino_obstack)", "src/ls.c", 0x41Du, "dev_ino_pop");
  qword_2B5F8 -= 16;
  return *(long long *)qword_2B5F8;
}



// Function: assert_matching_dev_ino @ 0x56dc
unsigned long long assert_matching_dev_ino(const char *a1, long long a2, long long a3)
{
  struct stat buf; // [rsp+20h] [rbp-A0h] BYREF
  unsigned long long v6; // [rsp+B8h] [rbp-8h]

  v6 = __readfsqword(0x28u);
  if ( !a1 )
    __assert_fail("name", "src/ls.c", 0x428u, "assert_matching_dev_ino");
  if ( stat(a1, &buf) < 0 )
    __assert_fail("0 <= stat (name, &sb)", "src/ls.c", 0x429u, "assert_matching_dev_ino");
  if ( buf.st_dev != a3 )
    __assert_fail("sb.st_dev == di.st_dev", "src/ls.c", 0x42Au, "assert_matching_dev_ino");
  if ( buf.st_ino != a2 )
    __assert_fail("sb.st_ino == di.st_ino", "src/ls.c", 0x42Bu, "assert_matching_dev_ino");
  return v6 - __readfsqword(0x28u);
}



// Function: dired_dump_obstack @ 0x581f
int dired_dump_obstack(const char *a1, long long a2)
{
  unsigned long long v2; // rax
  unsigned long long i; // [rsp+18h] [rbp-38h]
  unsigned long long v5; // [rsp+28h] [rbp-28h]
  long long v6; // [rsp+38h] [rbp-18h]

  v2 = (*(long long *)(a2 + 24) - *(long long *)(a2 + 16)) >> 3;
  v5 = v2;
  if ( v2 )
  {
    v6 = *(long long *)(a2 + 16);
    if ( v6 == *(long long *)(a2 + 24) )
      *(char *)(a2 + 80) |= 2u;
    *(long long *)(a2 + 24) = (*(long long *)(a2 + 48) + *(long long *)(a2 + 24)) & ~*(long long *)(a2 + 48);
    if ( *(long long *)(a2 + 24) - *(long long *)(a2 + 8) > *(long long *)(a2 + 32) - *(long long *)(a2 + 8) )
      *(long long *)(a2 + 24) = *(long long *)(a2 + 32);
    *(long long *)(a2 + 16) = *(long long *)(a2 + 24);
    fputs_unlocked(a1, stdout);
    for ( i = 0; i < v5; ++i )
      printf(" %ld", *(long long *)(8 * i + v6));
    LODWORD(v2) = putchar_unlocked(10);
  }
  return v2;
}



// Function: get_stat_btime @ 0x5990
long long get_stat_btime(long long a1)
{
  return sub_1B491(a1);
}



// Function: time_type_to_statx @ 0x59be
long long time_type_to_statx()
{
  if ( dword_2B3B0 == 3 )
    return 2048;
  if ( (unsigned int)dword_2B3B0 > 3 )
    goto LABEL_10;
  if ( dword_2B3B0 == 2 )
    return 32;
  if ( (unsigned int)dword_2B3B0 > 2 )
LABEL_10:
    abort();
  if ( dword_2B3B0 )
    return 128;
  else
    return 64;
}



// Function: calc_req_mask @ 0x5a0c
long long calc_req_mask()
{
  unsigned int v1; // [rsp+Ch] [rbp-4h]

  v1 = 2;
  if ( byte_2B3E9 )
    v1 = 258;
  if ( byte_2B3BB )
    v1 |= 0x400u;
  if ( !dword_2B3AC )
  {
    v1 |= sub_59BE() | 0x204;
    if ( byte_2B022 || byte_2B3B9 )
      v1 |= 8u;
    if ( byte_2B023 )
      v1 |= 0x10u;
  }
  switch ( dword_2B3B4 )
  {
    case 0:
    case 1:
    case 2:
    case 4:
    case 6:
      return v1;
    case 3:
      v1 |= 0x200u;
      break;
    case 5:
      v1 |= sub_59BE();
      break;
    default:
      abort();
  }
  return v1;
}



// Function: do_statx @ 0x5ad2
long long do_statx(unsigned int a1, long long a2, long long a3, unsigned int a4, unsigned int a5)
{
  unsigned int v5; // eax
  long long v6; // rdx
  bool v9; // [rsp+2Bh] [rbp-125h]
  int v10; // [rsp+2Ch] [rbp-124h]
  short v11[40]; // [rsp+30h] [rbp-120h] BYREF
  long long v12; // [rsp+80h] [rbp-D0h]
  long long v13; // [rsp+88h] [rbp-C8h]
  unsigned long long v14; // [rsp+138h] [rbp-18h]

  v14 = __readfsqword(0x28u);
  v9 = (a5 & 0x800) != 0;
  v5 = a4;
  BYTE1(v5) = BYTE1(a4) | 8;
  v10 = statx(a1, a2, v5, a5, v11);
  if ( v10 >= 0 )
  {
    sub_5297(v11, a3);
    if ( v9 )
    {
      if ( (v11[0] & 0x800) != 0 )
      {
        *(long long *)(a3 + 88) = sub_5267(v12, v13);
        *(long long *)(a3 + 96) = v6;
      }
      else
      {
        *(long long *)(a3 + 96) = -1;
        *(long long *)(a3 + 88) = *(long long *)(a3 + 96);
      }
    }
  }
  return (unsigned int)v10;
}



// Function: do_stat @ 0x5c0e
long long do_stat(long long a1, long long a2)
{
  unsigned int v2; // eax

  v2 = sub_5A0C();
  return sub_5AD2(0xFFFFFF9C, a1, a2, 0, v2);
}



// Function: do_lstat @ 0x5c48
long long do_lstat(long long a1, long long a2)
{
  unsigned int v2; // eax

  v2 = sub_5A0C();
  return sub_5AD2(0xFFFFFF9C, a1, a2, 0x100u, v2);
}



// Function: stat_for_mode @ 0x5c82
long long stat_for_mode(long long a1, long long a2)
{
  return sub_5AD2(0xFFFFFF9C, a1, a2, 0, 2u);
}



// Function: stat_for_ino @ 0x5cb8
long long stat_for_ino(long long a1, long long a2)
{
  return sub_5AD2(0xFFFFFF9C, a1, a2, 0, 0x100u);
}



// Function: fstat_for_ino @ 0x5cee
long long fstat_for_ino(unsigned int a1, long long a2)
{
  return sub_5AD2(a1, (long long)locale, a2, 0x1000u, 0x100u);
}



// Function: first_percent_b @ 0x5d23
char *first_percent_b(char *a1)
{
  int v1; // eax

  while ( *a1 )
  {
    if ( *a1 == 37 )
    {
      v1 = (char)a1[1];
      if ( v1 == 37 )
      {
        ++a1;
      }
      else if ( v1 == 98 )
      {
        return a1;
      }
    }
    ++a1;
  }
  return 0;
}



// Function: file_escape_init @ 0x5d77
void file_escape_init()
{
  bool v0; // cl
  int i; // [rsp+Ch] [rbp-4h]

  for ( i = 0; i <= 255; ++i )
  {
    v0 = (unsigned char)sub_10016((unsigned int)i) || i == 126 || i == 45 || i == 46 || i == 95;
    byte_2B640[i] |= v0;
  }
}



// Function: abmon_init @ 0x5df4
long long abmon_init(long long a1)
{
  const unsigned short *v2; // rbx
  unsigned char v3; // al
  unsigned long long v4; // rax
  int i; // [rsp+18h] [rbp-48h]
  unsigned long long v6; // [rsp+20h] [rbp-40h] BYREF
  unsigned long long v7; // [rsp+28h] [rbp-38h]
  unsigned long long v8; // [rsp+30h] [rbp-30h]
  char *s; // [rsp+38h] [rbp-28h]
  unsigned long long v10; // [rsp+40h] [rbp-20h]
  unsigned long long v11; // [rsp+48h] [rbp-18h]

  v11 = __readfsqword(0x28u);
  v7 = 12;
  do
  {
    v8 = v7;
    v7 = 0;
    for ( i = 0; i <= 11; ++i )
    {
      v6 = v8;
      s = nl_langinfo(i + 131086);
      if ( strchr(s, 37) )
        return 0;
      v2 = *__ctype_b_loc();
      v3 = sub_4EB2((unsigned int)*s);
      v10 = sub_15291(s, ((long long)i << 7) + a1, 128, &v6, (v2[v3] & 0x800) != 0, 0);
      if ( v10 > 0x7F )
        return 0;
      v4 = v6;
      if ( v7 >= v6 )
        v4 = v7;
      v7 = v4;
    }
  }
  while ( v8 > v7 );
  return 1;
}



// Function: abformat_init @ 0x5f43
unsigned long long abformat_init()
{
  int i; // [rsp+Ch] [rbp-644h]
  int j; // [rsp+10h] [rbp-640h]
  int k; // [rsp+14h] [rbp-63Ch]
  unsigned int v4; // [rsp+18h] [rbp-638h]
  const char *v5; // [rsp+20h] [rbp-630h]
  char *s; // [rsp+28h] [rbp-628h]
  long long v7[2]; // [rsp+30h] [rbp-620h]
  char v8[1544]; // [rsp+40h] [rbp-610h] BYREF
  unsigned long long v9; // [rsp+648h] [rbp-8h]

  v9 = __readfsqword(0x28u);
  for ( i = 0; i <= 1; ++i )
    v7[i] = sub_5D23(off_2B200[i]);
  if ( (v7[0] || v7[1]) && (unsigned char)sub_5DF4((long long)v8) == 1 )
  {
    for ( j = 0; j <= 1; ++j )
    {
      v5 = off_2B200[j];
      for ( k = 0; k <= 11; ++k )
      {
        s = (char *)&unk_2B740 + 128 * (k + 12LL * j);
        if ( v7[j] )
        {
          if ( (long long)(v7[j] - (long long)v5) > 128 )
            return v9 - __readfsqword(0x28u);
          v4 = snprintf(
                 s,
                 0x80u,
                 "%.*s%s%s",
                 v7[j] - (int)v5,
                 v5,
                 &v8[128 * (long long)k],
                 (const char *)(v7[j] + 2LL));
        }
        else
        {
          v4 = snprintf(s, 0x80u, "%s", v5);
        }
        if ( v4 >= 0x80 )
          return v9 - __readfsqword(0x28u);
      }
    }
    byte_2C340 = 1;
  }
  return v9 - __readfsqword(0x28u);
}



// Function: dev_ino_hash @ 0x61bd
unsigned long long dev_ino_hash(long long *a1, unsigned long long a2)
{
  return *a1 % a2;
}



// Function: dev_ino_compare @ 0x61ea
long long dev_ino_compare(long long *a1, long long *a2)
{
  return *a1 == *a2 && a1[1] == a2[1];
}



// Function: dev_ino_free @ 0x6243
void dev_ino_free(void *a1)
{
  free(a1);
}



// Function: visit_dir @ 0x6262
long long visit_dir(long long a1, long long a2)
{
  long long *ptr; // [rsp+20h] [rbp-10h]
  long long v4; // [rsp+28h] [rbp-8h]

  ptr = (long long *)sub_1C68B(16);
  *ptr = a2;
  ptr[1] = a1;
  v4 = sub_136D0(qword_2B320, ptr);
  if ( !v4 )
    sub_1CCC3();
  if ( (long long *)v4 != ptr )
    free(ptr);
  return v4 != (long long)ptr;
}



// Function: free_pending_ent @ 0x62e7
void free_pending_ent(void **a1)
{
  free(*a1);
  free(a1[1]);
  free(a1);
}



// Function: is_colored @ 0x6325
long long is_colored(unsigned int a1)
{
  long long v3; // [rsp+10h] [rbp-10h]
  char *s1; // [rsp+18h] [rbp-8h]

  v3 = qword_2B080[2 * a1];
  s1 = (char *)*(&::s1 + 2 * a1);
  return v3 && (v3 != 1 || strncmp(s1, "0", 1u)) && (v3 != 2 || strncmp(s1, "00", 2u));
}



// Function: restore_default_color @ 0x63ca
long long restore_default_color()
{
  sub_E4CA(qword_2B080);
  return sub_E4CA(&qword_2B090);
}



// Function: set_normal_color @ 0x63f3
long long set_normal_color()
{
  long long result; // rax

  result = (unsigned char)byte_2B3D4;
  if ( byte_2B3D4 )
  {
    result = sub_6325(4u);
    if ( (char)result )
    {
      sub_E4CA(qword_2B080);
      sub_E4CA(&unk_2B0C0);
      return sub_E4CA(&qword_2B090);
    }
  }
  return result;
}



// Function: sighandler @ 0x6444
long long sighandler(unsigned int a1)
{
  long long result; // rax

  result = (unsigned int)dword_2B4E0;
  if ( !dword_2B4E0 )
  {
    dword_2B4E0 = a1;
    return a1;
  }
  return result;
}



// Function: stophandler @ 0x6465
long long stophandler()
{
  long long result; // rax

  result = (unsigned int)dword_2B4E0;
  if ( !dword_2B4E0 )
    return (unsigned int)++dword_2B4E4;
  return result;
}



// Function: process_signals @ 0x648c
unsigned long long process_signals()
{
  int sig; // [rsp+8h] [rbp-98h]
  sigset_t oset; // [rsp+10h] [rbp-90h] BYREF
  unsigned long long v3; // [rsp+98h] [rbp-8h]

  v3 = __readfsqword(0x28u);
  while ( dword_2B4E0 || dword_2B4E4 )
  {
    if ( byte_2B3D6 )
      sub_63CA();
    fflush_unlocked(stdout);
    sigprocmask(0, &set, &oset);
    sig = dword_2B4E0;
    if ( dword_2B4E4 )
    {
      --dword_2B4E4;
      raise(19);
    }
    else
    {
      signal(dword_2B4E0, 0);
      raise(sig);
    }
    sigprocmask(2, &oset, 0);
  }
  return v3 - __readfsqword(0x28u);
}



// Function: signal_setup @ 0x6593
unsigned long long signal_setup(char a1)
{
  void (*v1)(int); // rax
  int i; // [rsp+1Ch] [rbp-A4h]
  int j; // [rsp+1Ch] [rbp-A4h]
  int k; // [rsp+1Ch] [rbp-A4h]
  struct sigaction oact; // [rsp+20h] [rbp-A0h] BYREF
  unsigned long long v7; // [rsp+B8h] [rbp-8h]

  v7 = __readfsqword(0x28u);
  if ( a1 )
  {
    sigemptyset(&set);
    for ( i = 0; i <= 11; ++i )
    {
      sigaction(dword_23380[i], 0, &oact);
      if ( oact.sa_handler != (__sighandler_t)((char *)&dword_0 + 1) )
        sigaddset(&set, dword_23380[i]);
    }
    oact.sa_mask = set;
    oact.sa_flags = 0x10000000;
    for ( j = 0; j <= 11; ++j )
    {
      if ( sigismember(&set, dword_23380[j]) )
      {
        if ( dword_23380[j] == 20 )
          v1 = (void (*)(int))sub_6465;
        else
          v1 = (void (*)(int))sub_6444;
        oact.sa_handler = v1;
        sigaction(dword_23380[j], &oact, 0);
      }
    }
  }
  else
  {
    for ( k = 0; k <= 11; ++k )
    {
      if ( sigismember(&set, dword_23380[k]) )
        signal(dword_23380[k], 0);
    }
  }
  return v7 - __readfsqword(0x28u);
}



// Function: signal_init @ 0x6858
unsigned long long signal_init()
{
  return sub_6593(1);
}



// Function: signal_restore @ 0x686d
unsigned long long signal_restore()
{
  return sub_6593(0);
}



// Function: main @ 0x6882
long long main(unsigned int a1, char **a2, char **a3)
{
  int v3; // eax
  char *v4; // rax
  bool v5; // al
  bool v6; // al
  int v7; // eax
  long long v8; // rdx
  int v9; // eax
  int v11; // [rsp+14h] [rbp-3Ch]
  int sig; // [rsp+18h] [rbp-38h]
  int v13; // [rsp+1Ch] [rbp-34h]
  void **v14; // [rsp+20h] [rbp-30h]
  void *v15; // [rsp+28h] [rbp-28h]
  long long v16[4]; // [rsp+30h] [rbp-20h] BYREF

  v16[3] = __readfsqword(0x28u);
  sub_18E89(*a2, a2, a3);
  setlocale(6, locale);
  bindtextdomain("coreutils", "/usr/local/share/locale");
  textdomain("coreutils");
  sub_4E99(2);
  sub_1EC10(sub_110AE);
  dword_2B4E8 = 0;
  byte_2B428 = 1;
  qword_2B368 = 0;
  qword_2B370 = 0x8000000000000000LL;
  qword_2B378 = -1;
  v11 = sub_70AF(a1, a2);
  if ( byte_2B3D4 )
    sub_88AB();
  if ( byte_2B3D4 )
    qword_2B420 = 0;
  if ( byte_2B3F2 )
  {
    byte_2B3E8 = 1;
  }
  else if ( byte_2B3D4 && (sub_6325(0xDu) || sub_6325(0xEu) && byte_2B358 || sub_6325(0xCu) && !dword_2B3AC) )
  {
    byte_2B3E8 = 1;
  }
  if ( !dword_2B3EC )
  {
    if ( byte_2B3F1 || dword_2B3D0 == 3 || !dword_2B3AC )
      v3 = 1;
    else
      v3 = 3;
    dword_2B3EC = v3;
  }
  if ( byte_2B3F0 )
  {
    qword_2B320 = sub_1298F(30, 0, sub_61BD, sub_61EA, sub_6243);
    if ( !qword_2B320 )
      sub_1CCC3();
    obstack_begin(&unk_2B5E0, 0, 0, &malloc, &free);
  }
  v4 = getenv("TZ");
  qword_2B438 = sub_1B525(v4);
  v5 = dword_2B3B4 == 5 || dword_2B3B4 == 3 || !dword_2B3AC || byte_2B380 || byte_2B3BB;
  byte_2B440 = v5;
  v6 = !v5 && (byte_2B3F0 || byte_2B3D4 || dword_2B3D0 || byte_2B3F2);
  byte_2B441 = v6;
  if ( byte_2B3CC )
  {
    obstack_begin(&unk_2B520, 0, 0, &malloc, &free);
    obstack_begin(&unk_2B580, 0, 0, &malloc, &free);
  }
  if ( byte_2B3D5 )
  {
    sub_5D77();
    qword_2B360 = sub_1CE96();
    if ( !qword_2B360 )
      qword_2B360 = (long long)locale;
  }
  qword_2B330 = 100;
  qword_2B328 = sub_1C804(100, 208);
  qword_2B338 = 0;
  sub_96F3();
  v13 = a1 - v11;
  if ( (int)(a1 - v11) > 0 )
  {
    do
    {
      v7 = v11++;
      sub_9A40(a2[v7], 0, 0, 1, locale);
    }
    while ( v11 < (int)a1 );
  }
  else if ( byte_2B3F1 )
  {
    sub_9A40(".", 3, 0, 1, locale);
  }
  else
  {
    sub_8E46(".", 0, 1);
  }
  if ( qword_2B338 )
  {
    sub_BFC5();
    if ( byte_2B3F1 != 1 )
      sub_AA65(0, 1);
  }
  if ( qword_2B338 )
  {
    sub_C12D();
    if ( qword_2B368 )
      sub_5422(10);
  }
  else if ( v13 <= 1 && qword_2B368 && !*(long long *)(qword_2B368 + 24) )
  {
    byte_2B428 = 0;
  }
  while ( qword_2B368 )
  {
    v14 = (void **)qword_2B368;
    qword_2B368 = *(long long *)(qword_2B368 + 24);
    if ( !qword_2B320 || *v14 )
    {
      sub_8EDE(*v14, v14[1], *((unsigned char *)v14 + 16));
      sub_62E7(v14);
      byte_2B428 = 1;
    }
    else
    {
      v16[0] = sub_563D();
      v16[1] = v8;
      v15 = (void *)sub_13740(qword_2B320, v16);
      if ( !v15 )
        __assert_fail("found", "src/ls.c", 0x70Du, "main");
      sub_6243(v15);
      sub_62E7(v14);
    }
  }
  if ( byte_2B3D4 && byte_2B3D6 )
  {
    if ( qword_2B080[0] != 2 || memcmp(s1, &unk_20638, 2u) || qword_2B090 != 1 || *(char *)off_2B098 != 109 )
      sub_63CA();
    fflush_unlocked(stdout);
    sub_686D();
    for ( sig = dword_2B4E4; sig; --sig )
      raise(19);
    if ( dword_2B4E0 )
      raise(dword_2B4E0);
  }
  if ( byte_2B3CC )
  {
    sub_581F("//DIRED//", (long long)&unk_2B520);
    sub_581F("//SUBDIRED//", (long long)&unk_2B580);
    v9 = sub_18FC3(qword_2B410);
    printf("//DIRED-OPTIONS// --quoting-style=%s\n", off_2A9E0[v9]);
  }
  if ( qword_2B320 )
  {
    if ( sub_12002(qword_2B320) )
      __assert_fail("hash_get_n_entries (active_dir_set) == 0", "src/ls.c", 0x741u, "main");
    sub_12C0F(qword_2B320);
  }
  return (unsigned int)dword_2B4E8;
}



// Function: decode_line_length @ 0x6fe6
long long decode_line_length(long long a1)
{
  int v1; // eax
  long long result; // rax
  long long v3[2]; // [rsp+10h] [rbp-10h] BYREF

  v3[1] = __readfsqword(0x28u);
  v1 = sub_1D251(a1, 0, 0, v3, locale);
  if ( v1 )
  {
    if ( v1 == 1 )
      return 0;
    else
      return -1;
  }
  else
  {
    result = v3[0];
    if ( v3[0] < 0LL )
      return 0;
  }
  return result;
}



// Function: stdout_isatty @ 0x706a
bool stdout_isatty()
{
  if ( byte_2B031 < 0 )
    byte_2B031 = isatty(1);
  return byte_2B031 != 0;
}



// Function: decode_switches @ 0x70af
long long decode_switches(int a1, char *const *a2)
{
  const char *v2; // rbx
  char *v3; // rax
  int v4; // eax
  bool v5; // al
  bool v6; // al
  const char *v7; // rax
  int v8; // eax
  long long v9; // rbx
  char *v10; // rax
  unsigned long long v11; // rax
  long long v12; // rbx
  char *v13; // rax
  bool v14; // al
  int v15; // eax
  bool v16; // al
  char *v17; // rax
  int v18; // eax
  long long v20; // rbx
  char *v21; // rax
  char *v22; // rax
  FILE *v23; // rbx
  char *v24; // rax
  const char **v25; // rax
  FILE *v26; // rbx
  char *v27; // rax
  char v28; // [rsp+13h] [rbp-BDh]
  int v29; // [rsp+14h] [rbp-BCh]
  int v30; // [rsp+18h] [rbp-B8h]
  int v31; // [rsp+1Ch] [rbp-B4h]
  int v32; // [rsp+20h] [rbp-B0h]
  int v33; // [rsp+24h] [rbp-ACh]
  int v34; // [rsp+28h] [rbp-A8h]
  int v35; // [rsp+2Ch] [rbp-A4h]
  int v36; // [rsp+30h] [rbp-A0h]
  int v37; // [rsp+30h] [rbp-A0h]
  int k; // [rsp+34h] [rbp-9Ch]
  int v39; // [rsp+38h] [rbp-98h]
  unsigned int v40; // [rsp+3Ch] [rbp-94h]
  long long v41; // [rsp+40h] [rbp-90h]
  long long v42; // [rsp+48h] [rbp-88h]
  long long v43; // [rsp+50h] [rbp-80h]
  long long v44; // [rsp+58h] [rbp-78h]
  char *i; // [rsp+60h] [rbp-70h]
  char *s1; // [rsp+68h] [rbp-68h]
  char *v47; // [rsp+70h] [rbp-60h]
  char *v48; // [rsp+70h] [rbp-60h]
  char **j; // [rsp+78h] [rbp-58h]
  long long *v50; // [rsp+80h] [rbp-50h]
  char *v51; // [rsp+88h] [rbp-48h]
  char *v52; // [rsp+90h] [rbp-40h]
  char *v53; // [rsp+98h] [rbp-38h]
  long long v54; // [rsp+A0h] [rbp-30h]
  int longind[2]; // [rsp+B0h] [rbp-20h] BYREF
  unsigned long long v56; // [rsp+B8h] [rbp-18h]

  v56 = __readfsqword(0x28u);
  v41 = 0;
  v28 = 0;
  v29 = -1;
  v30 = -1;
  v31 = -1;
  v32 = -1;
  v42 = -1;
  v43 = -1;
  while ( 1 )
  {
    longind[0] = -1;
    v39 = getopt_long(a1, a2, "abcdfghiklmnopqrstuvw:xABCDFGHI:LNQRST:UXZ1", &longopts, longind);
    if ( v39 == -1 )
      break;
    switch ( v39 )
    {
      case -131:
        if ( dword_2B210 == 1 )
        {
          v7 = "ls";
        }
        else if ( dword_2B210 == 2 )
        {
          v7 = "dir";
        }
        else
        {
          v7 = "vdir";
        }
        sub_1C4AB(
          (int)stdout,
          (int)v7,
          (unsigned int)"GNU coreutils",
          (int)off_2B218,
          (unsigned int)"Richard M. Stallman",
          (unsigned int)"David MacKenzie",
          0);
        exit(0);
      case -130:
        sub_F135(0);
      case 49:
        v29 = v29 != 0;
        continue;
      case 65:
        dword_2B3F4 = 1;
        continue;
      case 66:
        sub_94E9("*~");
        sub_94E9(".*~");
        continue;
      case 67:
        v29 = 2;
        continue;
      case 68:
        byte_2B3CC = 1;
        continue;
      case 70:
        if ( *(long long *)&optarg )
          v33 = dword_20980[sub_FE17(
                              (unsigned int)"--classify",
                              optarg,
                              (unsigned int)off_2A720,
                              (unsigned int)dword_20980,
                              4,
                              (int)off_2B220,
                              1)];
        else
          v33 = 1;
        if ( v33 == 1 || v33 == 2 && sub_706A() )
          dword_2B3D0 = 3;
        continue;
      case 71:
        byte_2B023 = 0;
        continue;
      case 72:
        dword_2B3EC = 2;
        continue;
      case 73:
        sub_94E9(*(long long *)&optarg);
        continue;
      case 76:
        dword_2B3EC = 4;
        continue;
      case 78:
        v31 = 0;
        continue;
      case 81:
        v31 = 5;
        continue;
      case 82:
        byte_2B3F0 = 1;
        continue;
      case 83:
        v32 = 3;
        continue;
      case 84:
        v4 = (unsigned int)gettext("invalid tab size");
        v42 = sub_1CD03(optarg, 0, 0, -1, (int)locale, v4, 2);
        continue;
      case 85:
        v32 = 6;
        continue;
      case 88:
        v32 = 1;
        continue;
      case 90:
        byte_2B380 = 1;
        continue;
      case 97:
        dword_2B3F4 = 2;
        continue;
      case 98:
        v31 = 7;
        continue;
      case 99:
        dword_2B3B0 = 1;
        continue;
      case 100:
        byte_2B3F1 = 1;
        continue;
      case 102:
        dword_2B3F4 = 2;
        v32 = 6;
        if ( !v29 )
          v29 = -1;
        byte_2B3D4 = 0;
        byte_2B3D5 = 0;
        byte_2B3BB = 0;
        continue;
      case 103:
        v29 = 0;
        byte_2B022 = 0;
        continue;
      case 104:
        dword_2B3BC = 176;
        dword_2B3C8 = 176;
        qword_2B3C0 = 1;
        qword_2B028 = 1;
        continue;
      case 105:
        byte_2B3E9 = 1;
        continue;
      case 107:
        v28 = 1;
        continue;
      case 108:
        v29 = 0;
        continue;
      case 109:
        v29 = 4;
        continue;
      case 110:
        byte_2B3BA = 1;
        v29 = 0;
        continue;
      case 111:
        v29 = 0;
        byte_2B023 = 0;
        continue;
      case 112:
        dword_2B3D0 = 1;
        continue;
      case 113:
        v30 = 1;
        continue;
      case 114:
        byte_2B3B8 = 1;
        continue;
      case 115:
        byte_2B3BB = 1;
        continue;
      case 116:
        v32 = 5;
        continue;
      case 117:
        dword_2B3B0 = 2;
        continue;
      case 118:
        v32 = 4;
        continue;
      case 119:
        v43 = sub_6FE6(*(long long *)&optarg);
        if ( v43 >= 0 )
          continue;
        v2 = (const char *)sub_1AECA(*(long long *)&optarg);
        v3 = gettext("invalid line width");
        error(2, 0, "%s: %s", v3, v2);
LABEL_31:
        v29 = 3;
        break;
      case 120:
        goto LABEL_31;
      case 128:
        byte_2B3B9 = 1;
        continue;
      case 129:
        v40 = sub_149FD(*(long long *)&optarg, &dword_2B3BC, &qword_2B3C0);
        if ( v40 )
          sub_1D130(v40, (unsigned int)longind[0], 0, &longopts, *(long long *)&optarg);
        dword_2B3C8 = dword_2B3BC;
        qword_2B028 = qword_2B3C0;
        continue;
      case 130:
        if ( *(long long *)&optarg )
          v34 = dword_20980[sub_FE17(
                              (unsigned int)"--color",
                              optarg,
                              (unsigned int)off_2A720,
                              (unsigned int)dword_20980,
                              4,
                              (int)off_2B220,
                              1)];
        else
          v34 = 1;
        v5 = v34 == 1 || v34 == 2 && sub_706A();
        byte_2B3D4 = v5;
        continue;
      case 131:
        dword_2B3EC = 3;
        continue;
      case 132:
        dword_2B3D0 = 2;
        continue;
      case 133:
        v29 = dword_208A0[sub_FE17(
                            (unsigned int)"--format",
                            optarg,
                            (unsigned int)off_2A660,
                            (unsigned int)dword_208A0,
                            4,
                            (int)off_2B220,
                            1)];
        continue;
      case 134:
        v29 = 0;
        v41 = sub_523E("full-iso");
        continue;
      case 135:
        byte_2B3F2 = 1;
        continue;
      case 136:
        v50 = (long long *)sub_1C68B(16);
        *v50 = *(long long *)&optarg;
        v50[1] = qword_2B400;
        qword_2B400 = (long long)v50;
        continue;
      case 137:
        if ( *(long long *)&optarg )
          v35 = dword_20980[sub_FE17(
                              (unsigned int)"--hyperlink",
                              optarg,
                              (unsigned int)off_2A720,
                              (unsigned int)dword_20980,
                              4,
                              (int)off_2B220,
                              1)];
        else
          v35 = 1;
        v6 = v35 == 1 || v35 == 2 && sub_706A();
        byte_2B3D5 = v6;
        continue;
      case 138:
        dword_2B3D0 = dword_205E0[sub_FE17(
                                    (unsigned int)"--indicator-style",
                                    optarg,
                                    (unsigned int)off_29FA0,
                                    (unsigned int)dword_205E0,
                                    4,
                                    (int)off_2B220,
                                    1)];
        continue;
      case 139:
        v31 = dword_238C0[sub_FE17(
                            (unsigned int)"--quoting-style",
                            optarg,
                            (unsigned int)off_2A9E0,
                            (unsigned int)dword_238C0,
                            4,
                            (int)off_2B220,
                            1)];
        continue;
      case 140:
        v30 = 0;
        continue;
      case 141:
        dword_2B3BC = 144;
        dword_2B3C8 = 144;
        qword_2B3C0 = 1;
        qword_2B028 = 1;
        continue;
      case 142:
        v32 = dword_208D0[sub_FE17(
                            (unsigned int)"--sort",
                            optarg,
                            (unsigned int)off_2A6A0,
                            (unsigned int)dword_208D0,
                            4,
                            (int)off_2B220,
                            1)];
        continue;
      case 143:
        dword_2B3B0 = dword_20920[sub_FE17(
                                    (unsigned int)"--time",
                                    optarg,
                                    (unsigned int)off_2A6E0,
                                    (unsigned int)dword_20920,
                                    4,
                                    (int)off_2B220,
                                    1)];
        continue;
      case 144:
        v41 = *(long long *)&optarg;
        continue;
      case 145:
        byte_2B030 = 0;
        v30 = 0;
        v29 = v29 != 0;
        byte_2B3D4 = 0;
        v31 = 0;
        continue;
      default:
        sub_F135(2);
    }
  }
  if ( !qword_2B3C0 )
  {
    v51 = getenv("LS_BLOCK_SIZE");
    sub_149FD(v51, &dword_2B3BC, &qword_2B3C0);
    if ( v51 || getenv("BLOCK_SIZE") )
    {
      dword_2B3C8 = dword_2B3BC;
      qword_2B028 = qword_2B3C0;
    }
    if ( v28 )
    {
      dword_2B3BC = 0;
      qword_2B3C0 = 1024;
    }
  }
  if ( v29 >= 0 )
  {
    v8 = v29;
  }
  else if ( dword_2B210 == 1 )
  {
    if ( sub_706A() )
      v8 = 2;
    else
      v8 = 1;
  }
  else if ( dword_2B210 == 2 )
  {
    v8 = 2;
  }
  else
  {
    v8 = 0;
  }
  dword_2B3AC = v8;
  v44 = v43;
  if ( v8 == 2 || dword_2B3AC == 3 || dword_2B3AC == 4 || byte_2B3D4 )
  {
    if ( v43 < 0 && sub_706A() && ioctl(1, 0x5413u, longind) >= 0 && HIWORD(longind[0]) )
      v44 = HIWORD(longind[0]);
    if ( v44 < 0 )
    {
      v52 = getenv("COLUMNS");
      if ( v52 )
      {
        if ( *v52 )
        {
          v44 = sub_6FE6((long long)v52);
          if ( v44 < 0 )
          {
            v9 = sub_1AECA(v52);
            v10 = gettext("ignoring invalid width in environment variable COLUMNS: %s");
            error(0, 0, v10, v9);
          }
        }
      }
    }
  }
  if ( v44 < 0 )
    v11 = 80;
  else
    v11 = v44;
  qword_2B430 = v11;
  qword_2B4F8 = (v11 % 3 != 0) + v11 / 3;
  if ( dword_2B3AC == 2 || dword_2B3AC == 3 || dword_2B3AC == 4 )
  {
    if ( v42 < 0 )
    {
      qword_2B420 = 8;
      v53 = getenv("TABSIZE");
      if ( v53 )
      {
        if ( (unsigned int)sub_1D251(v53, 0, 0, longind, locale) )
        {
          v12 = sub_1AECA(v53);
          v13 = gettext("ignoring invalid tab size in environment variable TABSIZE: %s");
          error(0, 0, v13, v12);
        }
        else
        {
          qword_2B420 = *(long long *)longind;
        }
      }
    }
    else
    {
      qword_2B420 = v42;
    }
  }
  if ( v30 >= 0 )
    v14 = v30 != 0;
  else
    v14 = dword_2B210 == 1 && sub_706A();
  byte_2B408 = v14;
  v36 = v31;
  if ( v31 < 0 )
    v36 = sub_8CFF();
  if ( v36 < 0 )
  {
    if ( dword_2B210 == 1 )
    {
      if ( sub_706A() )
        v15 = 3;
      else
        v15 = -1;
    }
    else
    {
      v15 = 7;
    }
    v36 = v15;
  }
  if ( v36 >= 0 )
    sub_18FE7(0, (unsigned int)v36);
  v37 = sub_18FC3(0);
  v16 = (!dword_2B3AC || (dword_2B3AC == 2 || dword_2B3AC == 3) && qword_2B430) && (v37 == 1 || v37 == 3 || v37 == 6);
  byte_2B341 = v16;
  qword_2B410 = sub_18F74(0);
  if ( v37 == 7 )
    sub_19012(qword_2B410, 32, 1);
  if ( (unsigned int)dword_2B3D0 > 1 )
  {
    for ( i = &asc_20C86[dword_2B3D0 - 2]; *i; ++i )
      sub_19012(qword_2B410, (unsigned int)*i, 1);
  }
  qword_2B418 = sub_18F74(0);
  sub_19012(qword_2B418, 58, 1);
  byte_2B3CC = ((unsigned char)byte_2B3CC & (dword_2B3AC == 0) & ((unsigned char)byte_2B3D5 ^ 1)) != 0;
  if ( byte_2B030 < (int)(unsigned char)byte_2B3CC )
  {
    v17 = gettext("--dired and --zero are incompatible");
    error(2, 0, v17);
  }
  if ( v32 >= 0 )
  {
    v18 = v32;
  }
  else if ( dword_2B3AC && (dword_2B3B0 == 1 || dword_2B3B0 == 2 || dword_2B3B0 == 3) )
  {
    v18 = 5;
  }
  else
  {
    v18 = 0;
  }
  dword_2B3B4 = v18;
  if ( !dword_2B3AC )
  {
    s1 = (char *)v41;
    if ( !v41 )
    {
      s1 = getenv("TIME_STYLE");
      if ( !s1 )
        s1 = (char *)sub_523E("locale");
    }
    while ( !strncmp(s1, "posix-", 6u) )
    {
      if ( (unsigned char)sub_11F2C(2) != 1 )
        return (unsigned int)optind;
      s1 += 6;
    }
    if ( *s1 == 43 )
    {
      v47 = strchr(s1 + 1, 10);
      if ( v47 )
      {
        if ( strchr(v47 + 1, 10) )
        {
          v20 = sub_1AECA(s1 + 1);
          v21 = gettext("invalid time style format %s");
          error(2, 0, v21, v20);
        }
        v22 = v47;
        v48 = v47 + 1;
        *v22 = 0;
      }
      else
      {
        v48 = s1 + 1;
      }
      off_2B200[0] = s1 + 1;
      off_2B208 = v48;
    }
    else
    {
      v54 = sub_FA7E(s1, off_29F60, &unk_205B0, 4);
      if ( v54 < 0 )
      {
        sub_FC27("time style", s1, v54);
        v23 = stderr;
        v24 = gettext("Valid arguments are:\n");
        fputs_unlocked(v24, v23);
        for ( j = off_29F60; *j; ++j )
        {
          v25 = (const char **)j;
          fprintf(stderr, "  - [posix-]%s\n", *v25);
        }
        v26 = stderr;
        v27 = gettext("  - +FORMAT (e.g., +%H:%M) for a 'date'-style format\n");
        fputs_unlocked(v27, v26);
        sub_F135(2);
      }
      if ( v54 == 3 )
      {
        if ( (unsigned char)sub_11F2C(2) )
        {
          for ( k = 0; k <= 1; ++k )
            off_2B200[k] = dcgettext(0, off_2B200[k], 2);
        }
      }
      else if ( v54 <= 3 )
      {
        if ( v54 == 2 )
        {
          off_2B200[0] = "%Y-%m-%d ";
          off_2B208 = "%m-%d %H:%M";
        }
        else if ( v54 )
        {
          off_2B208 = "%Y-%m-%d %H:%M";
          off_2B200[0] = "%Y-%m-%d %H:%M";
        }
        else
        {
          off_2B208 = "%Y-%m-%d %H:%M:%S.%N %z";
          off_2B200[0] = "%Y-%m-%d %H:%M:%S.%N %z";
        }
      }
    }
    sub_5F43();
  }
  return (unsigned int)optind;
}



// Function: get_funky_string @ 0x842f
bool get_funky_string(char **a1, char **a2, char a3, long long *a4)
{
  int v4; // eax
  char *v5; // rdx
  char *v6; // rax
  int v7; // eax
  char *v8; // rax
  char *v9; // rax
  char *v10; // rax
  int v11; // eax
  char *v12; // rax
  char *v13; // rax
  char *v14; // rax
  char *v15; // rax
  char *v16; // rax
  char v17; // cl
  char *v18; // rax
  char *v19; // rax
  char v23; // [rsp+23h] [rbp-1Dh]
  int v24; // [rsp+24h] [rbp-1Ch]
  long long v25; // [rsp+28h] [rbp-18h]
  char *v26; // [rsp+30h] [rbp-10h]
  char *v27; // [rsp+38h] [rbp-8h]

  v26 = *a2;
  v27 = *a1;
  v25 = 0;
  v23 = 0;
  v24 = 0;
  while ( 1 )
  {
    switch ( v24 )
    {
      case 0:
        v4 = *v26;
        if ( v4 == 94 )
        {
          v24 = 4;
          ++v26;
          continue;
        }
        if ( v4 > 94 )
          goto LABEL_16;
        if ( v4 == 92 )
        {
          v24 = 1;
          ++v26;
          continue;
        }
        if ( v4 > 92 )
          goto LABEL_16;
        if ( v4 == 61 )
        {
          if ( a3 )
          {
            v24 = 5;
            continue;
          }
        }
        else if ( v4 <= 61 && (!*v26 || v4 == 58) )
        {
          v24 = 5;
          continue;
        }
LABEL_16:
        v5 = v26++;
        v6 = v27++;
        *v6 = *v5;
        ++v25;
        continue;
      case 1:
        v7 = *v26;
        if ( *v26 )
        {
          if ( (unsigned int)v7 > 0x78 || v7 < 48 )
          {
LABEL_34:
            v23 = *v26;
          }
          else
          {
            switch ( *v26 )
            {
              case '0':
              case '1':
              case '2':
              case '3':
              case '4':
              case '5':
              case '6':
              case '7':
                v24 = 2;
                v23 = *v26 - 48;
                break;
              case '?':
                v23 = 127;
                break;
              case 'X':
              case 'x':
                v24 = 3;
                v23 = 0;
                break;
              case '_':
                v23 = 32;
                break;
              case 'a':
                v23 = 7;
                break;
              case 'b':
                v23 = 8;
                break;
              case 'e':
                v23 = 27;
                break;
              case 'f':
                v23 = 12;
                break;
              case 'n':
                v23 = 10;
                break;
              case 'r':
                v23 = 13;
                break;
              case 't':
                v23 = 9;
                break;
              case 'v':
                v23 = 11;
                break;
              default:
                goto LABEL_34;
            }
          }
        }
        else
        {
          v24 = 6;
        }
        if ( v24 == 1 )
        {
          v8 = v27++;
          *v8 = v23;
          ++v25;
          v24 = 0;
        }
        ++v26;
        continue;
      case 2:
        if ( *v26 > 47 && *v26 <= 55 )
        {
          v10 = v26++;
          v23 = 8 * v23 + *v10 - 48;
        }
        else
        {
          v9 = v27++;
          *v9 = v23;
          ++v25;
          v24 = 0;
        }
        continue;
      case 3:
        v11 = *v26;
        if ( v11 > 102 )
          goto LABEL_51;
        if ( v11 >= 97 )
        {
          v13 = v26++;
          v23 = 16 * v23 + *v13 - 87;
        }
        else
        {
          if ( v11 > 57 )
          {
            if ( (unsigned int)(v11 - 65) <= 5 )
            {
              v14 = v26++;
              v23 = 16 * v23 + *v14 - 55;
              continue;
            }
          }
          else if ( v11 >= 48 )
          {
            v12 = v26++;
            v23 = 16 * v23 + *v12 - 48;
            continue;
          }
LABEL_51:
          v15 = v27++;
          *v15 = v23;
          ++v25;
          v24 = 0;
        }
        break;
      case 4:
        v24 = 0;
        if ( *v26 <= 63 || *v26 == 127 )
        {
          if ( *v26 == 63 )
          {
            v19 = v27++;
            *v19 = 127;
            ++v25;
          }
          else
          {
            v24 = 6;
          }
        }
        else
        {
          v16 = v26++;
          v17 = *v16;
          v18 = v27++;
          *v18 = v17 & 0x1F;
          ++v25;
        }
        continue;
      default:
        *a1 = v27;
        *a2 = v26;
        *a4 = v25;
        return v24 != 6;
    }
  }
}



// Function: known_term_type @ 0x87ec
long long known_term_type()
{
  char *s1; // [rsp+0h] [rbp-10h]
  char *name; // [rsp+8h] [rbp-8h]

  name = getenv("TERM");
  if ( !name || !*name )
    return 0;
  for ( s1 = "# Configuration file for dircolors, a utility to help you set the";
        (unsigned long long)(s1 - "# Configuration file for dircolors, a utility to help you set the") <= 0x12C7;
        s1 += strlen(s1) + 1 )
  {
    if ( !strncmp(s1, "TERM ", 5u) && !fnmatch(s1 + 5, name, 0) )
      return 1;
  }
  return 0;
}



// Function: parse_ls_color @ 0x88ab
unsigned long long parse_ls_color()
{
  int v0; // eax
  int v1; // eax
  char *v2; // rax
  char *v3; // rax
  char *v4; // rax
  int v5; // eax
  long long v6; // rbx
  char *v7; // rax
  char *v8; // rax
  int v9; // eax
  char *v10; // rax
  int v12; // [rsp+8h] [rbp-58h]
  int v13; // [rsp+Ch] [rbp-54h]
  char *v14; // [rsp+10h] [rbp-50h] BYREF
  char *v15; // [rsp+18h] [rbp-48h] BYREF
  long long *v16; // [rsp+20h] [rbp-40h]
  long long *v17; // [rsp+28h] [rbp-38h]
  void *ptr; // [rsp+30h] [rbp-30h]
  char *v19; // [rsp+38h] [rbp-28h]
  char dest[3]; // [rsp+45h] [rbp-1Bh] BYREF
  unsigned long long v21; // [rsp+48h] [rbp-18h]

  v21 = __readfsqword(0x28u);
  v14 = getenv("LS_COLORS");
  if ( !v14 || !*v14 )
  {
    v19 = getenv("COLORTERM");
    if ( (!v19 || !*v19) && (unsigned char)sub_87EC() != 1 )
      byte_2B3D4 = 0;
    return v21 - __readfsqword(0x28u);
  }
  v16 = 0;
  strcpy(dest, "??");
  ::ptr = (void *)sub_1CC92(v14);
  v15 = (char *)::ptr;
  v13 = 1;
  while ( 2 )
  {
    switch ( v13 )
    {
      case 1:
        v0 = *v14;
        if ( v0 == 58 )
        {
          ++v14;
          continue;
        }
        if ( v0 > 58 )
          goto LABEL_20;
        if ( *v14 )
        {
          if ( v0 == 42 )
          {
            v16 = (long long *)sub_1C68B(40);
            v16[4] = qword_2B3D8;
            qword_2B3D8 = (long long)v16;
            ++v14;
            v16[1] = v15;
            if ( sub_842F(&v15, &v14, 1, v16) )
              v1 = 4;
            else
              v1 = 6;
            v13 = v1;
          }
          else
          {
LABEL_20:
            v2 = v14++;
            dest[0] = *v2;
            v13 = 2;
          }
          continue;
        }
        v13 = 5;
LABEL_42:
        if ( v13 == 6 )
        {
          v10 = gettext("unparsable value for LS_COLORS environment variable");
          error(0, 0, v10);
          free(::ptr);
          v17 = (long long *)qword_2B3D8;
          while ( v17 )
          {
            ptr = v17;
            v17 = (long long *)v17[4];
            free(ptr);
          }
          byte_2B3D4 = 0;
        }
        if ( qword_2B0F0 == 6 && !strncmp(off_2B0F8, "target", 6u) )
          byte_2B358 = 1;
        return v21 - __readfsqword(0x28u);
      case 2:
        if ( *v14 )
        {
          v3 = v14++;
          dest[1] = *v3;
          v13 = 3;
        }
        else
        {
          v13 = 6;
        }
        continue;
      case 3:
        v13 = 6;
        v4 = v14++;
        if ( *v4 != 61 )
          continue;
        v12 = 0;
        while ( 2 )
        {
          if ( *(&off_29FE0 + v12) )
          {
            if ( strcmp(dest, (const char *)*(&off_29FE0 + v12)) )
            {
              ++v12;
              continue;
            }
            *(&s1 + 2 * v12) = v15;
            if ( sub_842F(&v15, &v14, 0, &qword_2B080[2 * v12]) )
              v5 = 1;
            else
              v5 = 6;
            v13 = v5;
          }
          break;
        }
        if ( v13 == 6 )
        {
          v6 = sub_1AECA(dest);
          v7 = gettext("unrecognized prefix: %s");
          error(0, 0, v7, v6);
        }
        continue;
      case 4:
        v8 = v14++;
        if ( *v8 == 61 )
        {
          v16[3] = v15;
          if ( sub_842F(&v15, &v14, 0, v16 + 2) )
            v9 = 1;
          else
            v9 = 6;
          v13 = v9;
        }
        else
        {
          v13 = 6;
        }
        continue;
      case 6:
        goto LABEL_42;
    }
  }
}



// Function: getenv_quoting_style @ 0x8cff
long long getenv_quoting_style()
{
  long long v1; // rbx
  char *v2; // rax
  int v3; // [rsp+4h] [rbp-1Ch]
  char *v4; // [rsp+8h] [rbp-18h]

  v4 = getenv("QUOTING_STYLE");
  if ( !v4 )
    return 0xFFFFFFFFLL;
  v3 = sub_FA7E(v4, off_2A9E0, dword_238C0, 4);
  if ( v3 >= 0 )
    return (unsigned int)dword_238C0[v3];
  v1 = sub_1AECA(v4);
  v2 = gettext("ignoring invalid value of environment variable QUOTING_STYLE: %s");
  error(0, 0, v2, v1);
  return 0xFFFFFFFFLL;
}



// Function: set_exit_status @ 0x8db1
long long set_exit_status(unsigned int a1)
{
  long long result; // rax

  result = a1;
  if ( (char)a1 )
  {
    dword_2B4E8 = 2;
  }
  else
  {
    result = (unsigned int)dword_2B4E8;
    if ( !dword_2B4E8 )
      dword_2B4E8 = 1;
  }
  return result;
}



// Function: file_failure @ 0x8de7
long long file_failure(unsigned char a1, const char *a2, long long a3)
{
  long long v3; // rbx
  int *v4; // rax

  v3 = sub_1AA83(4, a3);
  v4 = __errno_location();
  error(0, *v4, a2, v3);
  return sub_8DB1(a1);
}



// Function: queue_directory @ 0x8e46
long long queue_directory(long long a1, long long a2, char a3)
{
  long long v3; // rdx
  long long v4; // rdx
  long long v7; // [rsp+28h] [rbp-8h]

  v7 = sub_1C68B(32);
  if ( a2 )
    v3 = sub_1CC92(a2);
  else
    v3 = 0;
  *(long long *)(v7 + 8) = v3;
  if ( a1 )
    v4 = sub_1CC92(a1);
  else
    v4 = 0;
  *(long long *)v7 = v4;
  *(char *)(v7 + 16) = a3;
  *(long long *)(v7 + 24) = qword_2B368;
  qword_2B368 = v7;
  return v7;
}



// Function: print_dir @ 0x8ede
unsigned long long print_dir(const char *a1, long long a2, unsigned char a3)
{
  char *v3; // rax
  unsigned int v4; // eax
  char *v5; // rax
  long long v6; // rbx
  char *v7; // rax
  char *v8; // rax
  int v9; // eax
  char *v10; // rsi
  char *v11; // rsi
  char *v12; // rax
  unsigned int v15; // [rsp+28h] [rbp-378h]
  signed int v16; // [rsp+2Ch] [rbp-374h]
  long long v17; // [rsp+30h] [rbp-370h]
  void *ptr; // [rsp+38h] [rbp-368h]
  DIR *dirp; // [rsp+40h] [rbp-360h]
  struct dirent *v20; // [rsp+48h] [rbp-358h]
  char *s; // [rsp+50h] [rbp-350h]
  char *v22; // [rsp+58h] [rbp-348h]
  long long v23; // [rsp+60h] [rbp-340h] BYREF
  long long v24; // [rsp+68h] [rbp-338h]
  char v25[7]; // [rsp+F1h] [rbp-2AFh] BYREF
  unsigned long long v26; // [rsp+388h] [rbp-18h]

  v26 = __readfsqword(0x28u);
  v17 = 0;
  *__errno_location() = 0;
  dirp = opendir(a1);
  if ( !dirp )
  {
    v3 = gettext("cannot open directory %s");
    sub_8DE7(a3, v3, (long long)a1);
    return v26 - __readfsqword(0x28u);
  }
  if ( qword_2B320 )
  {
    v16 = dirfd(dirp);
    if ( v16 < 0 )
      v4 = (unsigned int)sub_5CB8((long long)a1, (long long)&v23) >> 31;
    else
      v4 = (unsigned int)sub_5CEE(v16, (long long)&v23) >> 31;
    if ( (char)v4 )
    {
      v5 = gettext("cannot determine device and inode of %s");
      sub_8DE7(a3, v5, (long long)a1);
      closedir(dirp);
      return v26 - __readfsqword(0x28u);
    }
    if ( sub_6262(v23, v24) )
    {
      v6 = sub_1AC15(0, 3, a1);
      v7 = gettext("%s: not listing already-listed directory");
      error(0, 0, v7, v6);
      closedir(dirp);
      sub_8DB1(1u);
      return v26 - __readfsqword(0x28u);
    }
    sub_558E(v23, v24);
  }
  sub_96F3();
  if ( byte_2B3F0 || byte_2B428 )
  {
    if ( byte_2B032 != 1 )
      sub_5422(10);
    byte_2B032 = 0;
    sub_54CF();
    ptr = 0;
    if ( byte_2B3D5 )
    {
      ptr = (void *)sub_1101B(a1, 2);
      if ( !ptr )
      {
        v8 = gettext("error canonicalizing %s");
        sub_8DE7(a3, v8, (long long)a1);
      }
    }
    if ( a2 )
      v9 = a2;
    else
      v9 = (int)a1;
    sub_D87F(v9, qword_2B418, -1, 0, 1, (unsigned int)&unk_2B580, (long long)ptr);
    free(ptr);
    sub_549E(":\n");
  }
  while ( 1 )
  {
    *__errno_location() = 0;
    v20 = readdir(dirp);
    if ( !v20 )
      break;
    if ( (unsigned char)sub_958D(v20->d_name) != 1 )
    {
      v15 = 0;
      switch ( v20->d_type )
      {
        case 1u:
          v15 = 1;
          break;
        case 2u:
          v15 = 2;
          break;
        case 4u:
          v15 = 3;
          break;
        case 6u:
          v15 = 4;
          break;
        case 8u:
          v15 = 5;
          break;
        case 0xAu:
          v15 = 6;
          break;
        case 0xCu:
          v15 = 7;
          break;
        case 0xEu:
          v15 = 8;
          break;
        default:
          break;
      }
      v17 += sub_9A40(v20->d_name, v15, 0, 0, a1);
      if ( dword_2B3AC == 1 && dword_2B3B4 == 6 && byte_2B3BB != 1 && byte_2B3F0 != 1 )
      {
        sub_BFC5();
        sub_C12D();
        sub_96F3();
      }
    }
LABEL_41:
    sub_648C();
  }
  if ( *__errno_location() )
  {
    v10 = gettext("reading directory %s");
    sub_8DE7(a3, v10, (long long)a1);
    if ( *__errno_location() == 75 )
      goto LABEL_41;
  }
  if ( closedir(dirp) )
  {
    v11 = gettext("closing directory %s");
    sub_8DE7(a3, v11, (long long)a1);
  }
  sub_BFC5();
  if ( byte_2B3F0 )
    sub_AA65(a1, 0);
  if ( !dword_2B3AC || byte_2B3BB )
  {
    s = (char *)sub_13D91(v17, v25, (unsigned int)dword_2B3BC, 512, qword_2B3C0);
    v22 = &s[strlen(s)];
    *--s = 32;
    *v22 = byte_2B030;
    sub_54CF();
    v12 = gettext("total");
    sub_549E(v12);
    sub_5453(s, v22 + 1 - s);
  }
  if ( qword_2B338 )
    sub_C12D();
  return v26 - __readfsqword(0x28u);
}



// Function: add_ignore_pattern @ 0x94e9
long long *add_ignore_pattern(long long a1)
{
  long long *result; // rax

  result = (long long *)sub_1C68B(16);
  *result = a1;
  result[1] = qword_2B3F8;
  qword_2B3F8 = (long long)result;
  return result;
}



// Function: patterns_match @ 0x952f
long long patterns_match(long long a1, const char *a2)
{
  while ( a1 )
  {
    if ( !fnmatch(*(const char **)a1, a2, 4) )
      return 1;
    a1 = *(long long *)(a1 + 8);
  }
  return 0;
}



// Function: file_ignored @ 0x958d
long long file_ignored(long long a1)
{
  long long v1; // rdx
  bool v2; // al

  v2 = dword_2B3F4 != 2
    && *(char *)a1 == 46
    && (!dword_2B3F4 || (*(char *)(a1 + 1) != 46 ? (v1 = 1) : (v1 = 2), !*(char *)(v1 + a1)))
    || !dword_2B3F4 && (unsigned char)sub_952F(qword_2B400, (const char *)a1)
    || (unsigned char)sub_952F(qword_2B3F8, (const char *)a1);
  return v2;
}



// Function: unsigned_file_size @ 0x9635
long long unsigned_file_size(long long a1)
{
  return a1;
}



// Function: has_capability @ 0x9647
long long has_capability()
{
  *__errno_location() = 95;
  return 0;
}



// Function: free_ent @ 0x9669
void free_ent(void **a1)
{
  free(*a1);
  free(a1[1]);
  free(a1[2]);
  if ( a1[22] != &unk_2B020 )
  {
    if ( (unsigned char)sub_525C() )
      free(a1[22]);
    else
      freecon(a1[22]);
  }
}



// Function: clear_files @ 0x96f3
long long clear_files()
{
  long long result; // rax
  unsigned long long i; // [rsp+0h] [rbp-10h]

  for ( i = 0; ; ++i )
  {
    result = qword_2B338;
    if ( i >= qword_2B338 )
      break;
    sub_9669(*((void ***)qword_2B348 + i));
  }
  qword_2B338 = 0;
  byte_2B340 = 0;
  byte_2B381 = 0;
  dword_2B384 = 0;
  dword_2B388 = 0;
  dword_2B38C = 0;
  dword_2B394 = 0;
  dword_2B398 = 0;
  dword_2B39C = 0;
  dword_2B390 = 0;
  dword_2B3A0 = 0;
  dword_2B3A4 = 0;
  dword_2B3A8 = 0;
  return result;
}



// Function: errno_unsupported @ 0x97c0
long long errno_unsupported(unsigned int a1)
{
  return a1 == 22 || a1 == 38 || (unsigned char)sub_524C(a1);
}



// Function: getfilecon_cache @ 0x97fa
long long getfilecon_cache(long long a1, long long a2, char a3)
{
  int v4; // eax
  int *v5; // rax
  unsigned int v6; // [rsp+2Ch] [rbp-4h]

  if ( *(long long *)(a2 + 24) == qword_2C428 )
  {
    *__errno_location() = 95;
    return 0xFFFFFFFFLL;
  }
  else
  {
    if ( a3 )
      v4 = sub_1B22B(a1, a2 + 176);
    else
      v4 = sub_1B268(a1, a2 + 176);
    v6 = v4;
    if ( v4 < 0 )
    {
      v5 = __errno_location();
      if ( sub_97C0(*v5) )
        qword_2C428 = *(long long *)(a2 + 24);
    }
    return v6;
  }
}



// Function: file_has_acl_cache @ 0x98ab
long long file_has_acl_cache(long long a1, long long a2)
{
  int *v3; // rax
  int v4; // [rsp+1Ch] [rbp-4h]

  if ( *(long long *)(a2 + 24) == qword_2C430 )
  {
    *__errno_location() = 95;
    return 0;
  }
  else
  {
    *__errno_location() = 0;
    v4 = sub_112C6(a1, a2 + 24);
    if ( v4 <= 0 )
    {
      v3 = __errno_location();
      if ( sub_97C0(*v3) )
        qword_2C430 = *(long long *)(a2 + 24);
    }
    return (unsigned int)v4;
  }
}



// Function: has_capability_cache @ 0x9936
long long has_capability_cache(long long a1, long long a2)
{
  int *v3; // rax
  unsigned char v4; // [rsp+1Fh] [rbp-1h]

  if ( *(long long *)(a2 + 24) == qword_2C438 )
  {
    *__errno_location() = 95;
    return 0;
  }
  else
  {
    v4 = sub_9647(a1);
    if ( v4 != 1 )
    {
      v3 = __errno_location();
      if ( sub_97C0(*v3) )
        qword_2C438 = *(long long *)(a2 + 24);
    }
    return v4;
  }
}



// Function: needs_quoting @ 0x99b1
long long needs_quoting(const char *a1)
{
  long long v3; // [rsp+18h] [rbp-18h]
  char v4[2]; // [rsp+26h] [rbp-Ah] BYREF
  unsigned long long v5; // [rsp+28h] [rbp-8h]

  v5 = __readfsqword(0x28u);
  v3 = sub_1A389(v4, 2, a1, -1, qword_2B410);
  return *a1 != v4[0] || v3 != strlen(a1);
}



// Function: gobble_file @ 0x9a40
long long gobble_file(char *a1, int a2, long long a3, unsigned char a4, char *a5)
{
  size_t v5; // rbx
  size_t v6; // rax
  void *v7; // rsp
  long long v8; // rax
  char *v9; // rax
  bool v10; // al
  char *v11; // rax
  long long v13; // rax
  char v14; // al
  int *v15; // rax
  int v16; // eax
  const char *v17; // rbx
  int *v18; // rax
  long long v19; // rax
  const char *v20; // rax
  unsigned int v21; // eax
  const char *v22; // rax
  unsigned int v23; // eax
  const char *v24; // rax
  long long v25; // rax
  const char *v26; // rax
  long long v27; // rax
  char *v28; // [rsp+8h] [rbp-3E0h] BYREF
  long long v29; // [rsp+10h] [rbp-3D8h]
  unsigned char v30; // [rsp+18h] [rbp-3D0h]
  int v31; // [rsp+1Ch] [rbp-3CCh]
  char *v32; // [rsp+20h] [rbp-3C8h]
  char v33; // [rsp+2Ch] [rbp-3BCh]
  bool v34; // [rsp+2Dh] [rbp-3BBh]
  bool v35; // [rsp+2Eh] [rbp-3BAh]
  bool v36; // [rsp+2Fh] [rbp-3B9h]
  int v37; // [rsp+30h] [rbp-3B8h]
  int v38; // [rsp+34h] [rbp-3B4h]
  int v39; // [rsp+38h] [rbp-3B0h]
  int v40; // [rsp+3Ch] [rbp-3ACh]
  int v41; // [rsp+40h] [rbp-3A8h]
  int v42; // [rsp+44h] [rbp-3A4h]
  int v43; // [rsp+48h] [rbp-3A0h]
  int v44; // [rsp+4Ch] [rbp-39Ch]
  int v45; // [rsp+50h] [rbp-398h]
  int v46; // [rsp+54h] [rbp-394h]
  int v47; // [rsp+58h] [rbp-390h]
  int v48; // [rsp+5Ch] [rbp-38Ch]
  long long v49; // [rsp+60h] [rbp-388h]
  void *v50; // [rsp+68h] [rbp-380h]
  void *s; // [rsp+70h] [rbp-378h]
  void *ptr; // [rsp+78h] [rbp-370h]
  long long v53; // [rsp+80h] [rbp-368h]
  char v54[24]; // [rsp+88h] [rbp-360h] BYREF
  int v55; // [rsp+A0h] [rbp-348h]
  char v56[32]; // [rsp+118h] [rbp-2D0h] BYREF
  char v57[664]; // [rsp+138h] [rbp-2B0h] BYREF
  unsigned long long v58; // [rsp+3D0h] [rbp-18h]

  v32 = a1;
  v31 = a2;
  v29 = a3;
  v28 = a5;
  v30 = a4;
  v58 = __readfsqword(0x28u);
  v49 = 0;
  if ( a4 == 1 && v29 )
    __assert_fail("! command_line_arg || inode == NOT_AN_INODE_NUMBER", "src/ls.c", 0xD07u, "gobble_file");
  if ( qword_2B338 == qword_2B330 )
  {
    qword_2B328 = sub_1C638(qword_2B328, qword_2B330, 416);
    qword_2B330 *= 2;
  }
  s = (void *)(qword_2B328 + 208 * qword_2B338);
  memset(s, 0, 0xD0u);
  *((long long *)s + 4) = v29;
  *((int *)s + 42) = v31;
  *((int *)s + 49) = -1;
  if ( byte_2B340 != 1 )
  {
    if ( byte_2B341 )
    {
      *((int *)s + 49) = sub_99B1(v32);
      if ( *((int *)s + 49) )
        byte_2B340 = 1;
    }
  }
  if ( !v30
    && !byte_2B3D5
    && !byte_2B440
    && (v31 != 3 || !byte_2B3D4 || !sub_6325(0x13u) && !sub_6325(0x12u) && !sub_6325(0x14u))
    && (!byte_2B3E9 && !byte_2B441 || v31 != 6 && v31 || dword_2B3EC != 4 && !byte_2B358 && !byte_2B3E8)
    && (!byte_2B3E9 || v29)
    && (!byte_2B441
     || v31
     && (v31 != 5
      || dword_2B3D0 != 3
      && (!byte_2B3D4 || !sub_6325(0xEu) && !sub_6325(0x10u) && !sub_6325(0x11u) && !sub_6325(0x15u)))) )
  {
    goto LABEL_140;
  }
  if ( *v32 != 47 && *v28 )
  {
    v5 = strlen(v32);
    v6 = 16 * ((v5 + strlen(v28) + 25) / 0x10);
    while ( &v28 != (char **)((char *)&v28 - (v6 & 0xFFFFFFFFFFFFF000LL)) )
      ;
    v7 = alloca(v6 & 0xFFF);
    if ( (v6 & 0xFFF) != 0 )
      *(char **)((char *)&v28 + (v6 & 0xFFF) - 8) = *(char **)((char *)&v28 + (v6 & 0xFFF) - 8);
    v50 = &v28;
    sub_EB94(&v28, v28, v32);
  }
  else
  {
    v50 = v32;
  }
  if ( byte_2B3D5 )
  {
    v8 = sub_1101B(v50, 2);
    *((long long *)s + 2) = v8;
    if ( !*((long long *)s + 2) )
    {
      v9 = gettext("error canonicalizing %s");
      sub_8DE7(v30, v9, (long long)v50);
    }
  }
  if ( (unsigned int)dword_2B3EC <= 3 )
  {
    if ( (unsigned int)dword_2B3EC >= 2 && v30 )
    {
      v37 = sub_5C0E((long long)v50, (long long)s + 24);
      v33 = 1;
      if ( dword_2B3EC == 2 )
        goto LABEL_59;
      v10 = v37 >= 0 ? (*((int *)s + 12) & 0xF000) != 0x4000 : *__errno_location() == 2;
      v36 = v10;
      if ( !v10 )
        goto LABEL_59;
    }
    goto LABEL_58;
  }
  if ( dword_2B3EC != 4 )
  {
LABEL_58:
    v37 = sub_5C48((long long)v50, (long long)s + 24);
    v33 = 0;
    goto LABEL_59;
  }
  v37 = sub_5C0E((long long)v50, (long long)s + 24);
  v33 = 1;
LABEL_59:
  if ( v37 )
  {
    v11 = gettext("cannot access %s");
    sub_8DE7(v30, v11, (long long)v50);
    *((long long *)s + 22) = &unk_2B020;
    if ( !v30 )
    {
      v13 = sub_1CC92(v32);
      *(long long *)s = v13;
      ++qword_2B338;
    }
    return 0;
  }
  *((char *)s + 184) = 1;
  if ( (v31 == 5 || (*((int *)s + 12) & 0xF000) == 0x8000) && byte_2B3D4 && sub_6325(0x15u) )
  {
    v14 = sub_9936((long long)v50, (long long)s);
    *((char *)s + 192) = v14;
  }
  if ( !dword_2B3AC || byte_2B380 )
  {
    v34 = 0;
    v35 = 0;
    v38 = sub_97FA((long long)v50, (long long)s, v33);
    v37 = v38 < 0;
    if ( v38 < 0 )
    {
      *((long long *)s + 22) = &unk_2B020;
      v15 = __errno_location();
      if ( (unsigned char)sub_524C((unsigned int)*v15) || *__errno_location() == 61 )
        v37 = 0;
    }
    else if ( (unsigned char)sub_525C() )
    {
      v34 = strcmp("_", *((const char **)s + 22)) != 0;
    }
    else
    {
      v34 = strcmp("unlabeled", *((const char **)s + 22)) != 0;
    }
    if ( !v37 && !dword_2B3AC )
    {
      v39 = sub_98AB((long long)v50, (long long)s);
      v37 = v39 < 0;
      v35 = v39 > 0;
    }
    if ( v34 || v35 )
    {
      if ( v34 && !v35 )
        v16 = 1;
      else
        v16 = 2;
    }
    else
    {
      v16 = 0;
    }
    *((int *)s + 47) = v16;
    byte_2B381 = ((unsigned char)byte_2B381 | (*((int *)s + 47) != 0)) != 0;
    if ( v37 )
    {
      v17 = (const char *)sub_1AC15(0, 3, v50);
      v18 = __errno_location();
      error(0, *v18, "%s", v17);
    }
  }
  if ( (*((int *)s + 12) & 0xF000) == 0xA000 && (!dword_2B3AC || byte_2B3E8) )
  {
    sub_A8F5(v50, s, v30);
    ptr = (void *)sub_A964(v50, *((long long *)s + 1));
    if ( ptr && !*((int *)s + 49) && sub_99B1(*((const char **)s + 1)) )
      *((int *)s + 49) = -1;
    if ( ptr && ((unsigned int)dword_2B3D0 > 1 || byte_2B3E8) && !(unsigned int)sub_5C82((long long)ptr, (long long)v54) )
    {
      *((char *)s + 185) = 1;
      *((int *)s + 43) = v55;
    }
    free(ptr);
  }
  if ( (*((int *)s + 12) & 0xF000) == 0xA000 )
  {
    *((int *)s + 42) = 6;
  }
  else if ( (*((int *)s + 12) & 0xF000) == 0x4000 )
  {
    if ( v30 && byte_2B3F1 != 1 )
      *((int *)s + 42) = 9;
    else
      *((int *)s + 42) = 3;
  }
  else
  {
    *((int *)s + 42) = 5;
  }
  v49 = *((long long *)s + 11);
  if ( !dword_2B3AC || byte_2B3BB )
  {
    v19 = sub_13D91(v49, v57, (unsigned int)dword_2B3BC, 512, qword_2B3C0);
    v40 = sub_1574E(v19, 0);
    if ( v40 > dword_2B388 )
      dword_2B388 = v40;
  }
  if ( !dword_2B3AC )
  {
    if ( byte_2B022 )
    {
      v41 = sub_C5C3(*((unsigned int *)s + 13));
      if ( v41 > dword_2B394 )
        dword_2B394 = v41;
    }
    if ( byte_2B023 )
    {
      v42 = sub_C603(*((unsigned int *)s + 14));
      if ( v42 > dword_2B398 )
        dword_2B398 = v42;
    }
    if ( byte_2B3B9 )
    {
      v43 = sub_C5C3(*((unsigned int *)s + 13));
      if ( v43 > dword_2B39C )
        dword_2B39C = v43;
    }
  }
  if ( byte_2B380 )
  {
    v44 = strlen(*((const char **)s + 22));
    if ( v44 > dword_2B390 )
      dword_2B390 = v44;
  }
  if ( !dword_2B3AC )
  {
    v20 = (const char *)sub_15035(*((long long *)s + 5), v56);
    v45 = strlen(v20);
    if ( v45 > dword_2B38C )
      dword_2B38C = v45;
    if ( (*((int *)s + 12) & 0xF000) == 0x2000 || (*((int *)s + 12) & 0xF000) == 0x6000 )
    {
      v21 = gnu_dev_major(*((long long *)s + 8));
      v22 = (const char *)sub_15035(v21, v57);
      v47 = strlen(v22);
      if ( v47 > dword_2B3A0 )
        dword_2B3A0 = v47;
      v23 = gnu_dev_minor(*((long long *)s + 8));
      v24 = (const char *)sub_15035(v23, v57);
      v47 = strlen(v24);
      if ( v47 > dword_2B3A4 )
        dword_2B3A4 = v47;
      v47 = dword_2B3A0 + 2 + dword_2B3A4;
      if ( v47 > dword_2B3A8 )
        dword_2B3A8 = v47;
    }
    else
    {
      v53 = sub_9635(*((long long *)s + 9));
      v25 = sub_13D91(v53, v57, (unsigned int)dword_2B3C8, 1, qword_2B028);
      v46 = sub_1574E(v25, 0);
      if ( v46 > dword_2B3A8 )
        dword_2B3A8 = v46;
    }
  }
LABEL_140:
  if ( byte_2B3E9 )
  {
    v26 = (const char *)sub_15035(*((long long *)s + 4), v57);
    v48 = strlen(v26);
    if ( v48 > dword_2B384 )
      dword_2B384 = v48;
  }
  v27 = sub_1CC92(v32);
  *(long long *)s = v27;
  ++qword_2B338;
  return v49;
}



// Function: is_directory @ 0xa869
long long is_directory(long long a1)
{
  return *(int *)(a1 + 168) == 3 || *(int *)(a1 + 168) == 9;
}



// Function: is_linked_directory @ 0xa8a4
long long is_linked_directory(long long a1)
{
  return *(int *)(a1 + 168) == 3 || *(int *)(a1 + 168) == 9 || (*(int *)(a1 + 172) & 0xF000) == 0x4000;
}



// Function: get_link_name @ 0xa8f5
long long get_link_name(long long a1, long long a2, unsigned char a3)
{
  long long result; // rax
  char *v4; // rax

  *(long long *)(a2 + 8) = sub_F7A0(a1, *(long long *)(a2 + 72));
  result = *(long long *)(a2 + 8);
  if ( !result )
  {
    v4 = gettext("cannot read symbolic link %s");
    return sub_8DE7(a3, v4, a1);
  }
  return result;
}



// Function: make_link_name @ 0xa964
long long make_link_name(const char *a1, const char *a2)
{
  size_t v3; // rax
  char *v4; // rax
  long long n; // [rsp+10h] [rbp-10h]
  char *dest; // [rsp+18h] [rbp-8h]

  if ( !a2 )
    return 0;
  if ( *a2 == 47 )
    return sub_1CC92(a2);
  n = sub_111A6(a1);
  if ( !n )
    return sub_1CC92(a2);
  v3 = strlen(a2);
  dest = (char *)sub_1C68B(n + v3 + 2);
  if ( a1[n - 1] != 47 )
    ++n;
  v4 = stpncpy(dest, a1, n);
  stpcpy(v4, a2);
  return (long long)dest;
}



// Function: basename_is_dot_or_dotdot @ 0xaa37
long long basename_is_dot_or_dotdot(long long a1)
{
  long long v2; // [rsp+18h] [rbp-8h]

  v2 = sub_FF56(a1);
  return sub_4EC1(v2);
}



// Function: extract_dirs_from_files @ 0xaa65
long long extract_dirs_from_files(long long a1, char a2)
{
  long long v4; // [rsp+18h] [rbp-28h]
  unsigned long long v5; // [rsp+18h] [rbp-28h]
  long long v6; // [rsp+20h] [rbp-20h]
  long long v7; // [rsp+28h] [rbp-18h]
  long long *v8; // [rsp+30h] [rbp-10h]
  void *ptr; // [rsp+38h] [rbp-8h]

  if ( a1 && qword_2B320 )
    sub_8E46(0, a1, 0);
  v4 = qword_2B338;
  while ( v4-- )
  {
    v8 = (long long *)*((long long *)qword_2B348 + v4);
    if ( sub_A869((long long)v8) && (!a1 || (unsigned char)sub_AA37(*v8) != 1) )
    {
      if ( a1 && *(char *)*v8 != 47 )
      {
        ptr = (void *)sub_116A7(a1, *v8, 0);
        sub_8E46((long long)ptr, v8[1], a2);
        free(ptr);
      }
      else
      {
        sub_8E46(*v8, v8[1], a2);
      }
      if ( *((int *)v8 + 42) == 9 )
        sub_9669((void **)v8);
    }
  }
  v5 = 0;
  v6 = 0;
  while ( v5 < qword_2B338 )
  {
    v7 = *((long long *)qword_2B348 + v5);
    *((long long *)qword_2B348 + v6) = v7;
    v6 += *(int *)(v7 + 168) != 9;
    ++v5;
  }
  qword_2B338 = v6;
  return v6;
}



// Function: xstrcoll @ 0xac36
long long xstrcoll(const char *a1, const char *a2)
{
  long long v2; // r13
  long long v3; // r12
  char *v4; // rbx
  int *v5; // rax
  unsigned int v7; // [rsp+1Ch] [rbp-24h]

  *__errno_location() = 0;
  v7 = strcoll(a1, a2);
  if ( *__errno_location() )
  {
    v2 = sub_1AE9D(1, a2);
    v3 = sub_1AE9D(0, a1);
    v4 = gettext("cannot compare file names %s and %s");
    v5 = __errno_location();
    error(0, *v5, v4, v3, v2);
    sub_8DB1(0);
    longjmp(env, 1);
  }
  return v7;
}



// Function: dirfirst_check @ 0xad02
long long dirfirst_check(long long a1, long long a2, long long (*a3)(long long, long long))
{
  int v3; // ebx
  int v6; // [rsp+2Ch] [rbp-14h]

  v3 = sub_A8A4(a2);
  v6 = v3 - sub_A8A4(a1);
  if ( v6 )
    return v6;
  else
    return a3(a1, a2);
}



// Function: cmp_ctime @ 0xad65
long long cmp_ctime(long long *a1, long long *a2, long long (*a3)(long long, long long))
{
  long long v3; // r12
  long long v4; // rdx
  long long v5; // r13
  long long v6; // rax
  long long v7; // rdx
  unsigned int v10; // [rsp+2Ch] [rbp-24h]

  v3 = sub_1B477(a1 + 3);
  v5 = v4;
  v6 = sub_1B477(a2 + 3);
  v10 = sub_1BC1F(v6, v7, v3, v5);
  if ( v10 )
    return v10;
  else
    return a3(*a1, *a2);
}



// Function: cmp_mtime @ 0xadf8
long long cmp_mtime(long long *a1, long long *a2, long long (*a3)(long long, long long))
{
  long long v3; // r12
  long long v4; // rdx
  long long v5; // r13
  long long v6; // rax
  long long v7; // rdx
  unsigned int v10; // [rsp+2Ch] [rbp-24h]

  v3 = sub_1B491(a1 + 3);
  v5 = v4;
  v6 = sub_1B491(a2 + 3);
  v10 = sub_1BC1F(v6, v7, v3, v5);
  if ( v10 )
    return v10;
  else
    return a3(*a1, *a2);
}



// Function: cmp_atime @ 0xae8b
long long cmp_atime(long long *a1, long long *a2, long long (*a3)(long long, long long))
{
  long long v3; // r12
  long long v4; // rdx
  long long v5; // r13
  long long v6; // rax
  long long v7; // rdx
  unsigned int v10; // [rsp+2Ch] [rbp-24h]

  v3 = sub_1B45D(a1 + 3);
  v5 = v4;
  v6 = sub_1B45D(a2 + 3);
  v10 = sub_1BC1F(v6, v7, v3, v5);
  if ( v10 )
    return v10;
  else
    return a3(*a1, *a2);
}



// Function: cmp_btime @ 0xaf1e
long long cmp_btime(long long *a1, long long *a2, long long (*a3)(long long, long long))
{
  long long v3; // r12
  long long v4; // rdx
  long long v5; // r13
  long long v6; // rax
  long long v7; // rdx
  unsigned int v10; // [rsp+2Ch] [rbp-24h]

  v3 = sub_5990((long long)(a1 + 3));
  v5 = v4;
  v6 = sub_5990((long long)(a2 + 3));
  v10 = sub_1BC1F(v6, v7, v3, v5);
  if ( v10 )
    return v10;
  else
    return a3(*a1, *a2);
}



// Function: off_cmp @ 0xafb1
long long off_cmp(long long a1, long long a2)
{
  if ( a1 < a2 )
    return 0xFFFFFFFFLL;
  else
    return a1 > a2;
}



// Function: cmp_size @ 0xafe2
long long cmp_size(long long *a1, long long *a2, long long (*a3)(long long, long long))
{
  unsigned int v5; // [rsp+2Ch] [rbp-4h]

  v5 = sub_AFB1(a2[9], a1[9]);
  if ( v5 )
    return v5;
  else
    return a3(*a1, *a2);
}



// Function: cmp_name @ 0xb03f
long long cmp_name(long long *a1, long long *a2, long long (*a3)(long long, long long))
{
  return a3(*a1, *a2);
}



// Function: cmp_extension @ 0xb073
long long cmp_extension(const char **a1, const char **a2, long long (*a3)(const char *, const char *))
{
  const char *v3; // rdx
  const char *v4; // rax
  unsigned int v7; // [rsp+2Ch] [rbp-14h]
  char *v8; // [rsp+30h] [rbp-10h]
  char *v9; // [rsp+38h] [rbp-8h]

  v8 = strrchr(*a1, 46);
  v9 = strrchr(*a2, 46);
  if ( v9 )
    v3 = v9;
  else
    v3 = locale;
  if ( v8 )
    v4 = v8;
  else
    v4 = locale;
  v7 = a3(v4, v3);
  if ( v7 )
    return v7;
  else
    return a3(*a1, *a2);
}



// Function: fileinfo_name_width @ 0xb119
long long fileinfo_name_width(long long a1)
{
  if ( *(long long *)(a1 + 200) )
    return *(long long *)(a1 + 200);
  else
    return sub_D671(*(long long *)a1, qword_2B410, *(unsigned int *)(a1 + 196));
}



// Function: cmp_width @ 0xb16b
long long cmp_width(long long *a1, long long *a2, long long (*a3)(long long, long long))
{
  int v3; // ebx
  unsigned int v6; // [rsp+2Ch] [rbp-14h]

  v3 = sub_B119((long long)a1);
  v6 = v3 - sub_B119((long long)a2);
  if ( v6 )
    return v6;
  else
    return a3(*a1, *a2);
}



// Function: xstrcoll_ctime @ 0xb1d2
long long xstrcoll_ctime(long long *a1, long long *a2)
{
  return sub_AD65(a1, a2, (long long (*)(long long, long long))sub_AC36);
}



// Function: strcmp_ctime @ 0xb202
long long strcmp_ctime(long long *a1, long long *a2)
{
  return sub_AD65(a1, a2, (long long (*)(long long, long long))&strcmp);
}



// Function: rev_xstrcoll_ctime @ 0xb232
long long rev_xstrcoll_ctime(long long *a1, long long *a2)
{
  return sub_AD65(a2, a1, (long long (*)(long long, long long))sub_AC36);
}



// Function: rev_strcmp_ctime @ 0xb262
long long rev_strcmp_ctime(long long *a1, long long *a2)
{
  return sub_AD65(a2, a1, (long long (*)(long long, long long))&strcmp);
}



// Function: xstrcoll_df_ctime @ 0xb292
long long xstrcoll_df_ctime(long long a1, long long a2)
{
  return sub_AD02(a1, a2, (long long (*)(long long, long long))sub_B1D2);
}



// Function: strcmp_df_ctime @ 0xb2c2
long long strcmp_df_ctime(long long a1, long long a2)
{
  return sub_AD02(a1, a2, (long long (*)(long long, long long))sub_B202);
}



// Function: rev_xstrcoll_df_ctime @ 0xb2f2
long long rev_xstrcoll_df_ctime(long long a1, long long a2)
{
  return sub_AD02(a1, a2, (long long (*)(long long, long long))sub_B232);
}



// Function: rev_strcmp_df_ctime @ 0xb322
long long rev_strcmp_df_ctime(long long a1, long long a2)
{
  return sub_AD02(a1, a2, (long long (*)(long long, long long))sub_B262);
}



// Function: xstrcoll_mtime @ 0xb352
long long xstrcoll_mtime(long long *a1, long long *a2)
{
  return sub_ADF8(a1, a2, (long long (*)(long long, long long))sub_AC36);
}



// Function: strcmp_mtime @ 0xb382
long long strcmp_mtime(long long *a1, long long *a2)
{
  return sub_ADF8(a1, a2, (long long (*)(long long, long long))&strcmp);
}



// Function: rev_xstrcoll_mtime @ 0xb3b2
long long rev_xstrcoll_mtime(long long *a1, long long *a2)
{
  return sub_ADF8(a2, a1, (long long (*)(long long, long long))sub_AC36);
}



// Function: rev_strcmp_mtime @ 0xb3e2
long long rev_strcmp_mtime(long long *a1, long long *a2)
{
  return sub_ADF8(a2, a1, (long long (*)(long long, long long))&strcmp);
}



// Function: xstrcoll_df_mtime @ 0xb412
long long xstrcoll_df_mtime(long long a1, long long a2)
{
  return sub_AD02(a1, a2, (long long (*)(long long, long long))sub_B352);
}



// Function: strcmp_df_mtime @ 0xb442
long long strcmp_df_mtime(long long a1, long long a2)
{
  return sub_AD02(a1, a2, (long long (*)(long long, long long))sub_B382);
}



// Function: rev_xstrcoll_df_mtime @ 0xb472
long long rev_xstrcoll_df_mtime(long long a1, long long a2)
{
  return sub_AD02(a1, a2, (long long (*)(long long, long long))sub_B3B2);
}



// Function: rev_strcmp_df_mtime @ 0xb4a2
long long rev_strcmp_df_mtime(long long a1, long long a2)
{
  return sub_AD02(a1, a2, (long long (*)(long long, long long))sub_B3E2);
}



// Function: xstrcoll_atime @ 0xb4d2
long long xstrcoll_atime(long long *a1, long long *a2)
{
  return sub_AE8B(a1, a2, (long long (*)(long long, long long))sub_AC36);
}



// Function: strcmp_atime @ 0xb502
long long strcmp_atime(long long *a1, long long *a2)
{
  return sub_AE8B(a1, a2, (long long (*)(long long, long long))&strcmp);
}



// Function: rev_xstrcoll_atime @ 0xb532
long long rev_xstrcoll_atime(long long *a1, long long *a2)
{
  return sub_AE8B(a2, a1, (long long (*)(long long, long long))sub_AC36);
}



// Function: rev_strcmp_atime @ 0xb562
long long rev_strcmp_atime(long long *a1, long long *a2)
{
  return sub_AE8B(a2, a1, (long long (*)(long long, long long))&strcmp);
}



// Function: xstrcoll_df_atime @ 0xb592
long long xstrcoll_df_atime(long long a1, long long a2)
{
  return sub_AD02(a1, a2, (long long (*)(long long, long long))sub_B4D2);
}



// Function: strcmp_df_atime @ 0xb5c2
long long strcmp_df_atime(long long a1, long long a2)
{
  return sub_AD02(a1, a2, (long long (*)(long long, long long))sub_B502);
}



// Function: rev_xstrcoll_df_atime @ 0xb5f2
long long rev_xstrcoll_df_atime(long long a1, long long a2)
{
  return sub_AD02(a1, a2, (long long (*)(long long, long long))sub_B532);
}



// Function: rev_strcmp_df_atime @ 0xb622
long long rev_strcmp_df_atime(long long a1, long long a2)
{
  return sub_AD02(a1, a2, (long long (*)(long long, long long))sub_B562);
}



// Function: xstrcoll_btime @ 0xb652
long long xstrcoll_btime(long long *a1, long long *a2)
{
  return sub_AF1E(a1, a2, (long long (*)(long long, long long))sub_AC36);
}



// Function: strcmp_btime @ 0xb682
long long strcmp_btime(long long *a1, long long *a2)
{
  return sub_AF1E(a1, a2, (long long (*)(long long, long long))&strcmp);
}



// Function: rev_xstrcoll_btime @ 0xb6b2
long long rev_xstrcoll_btime(long long *a1, long long *a2)
{
  return sub_AF1E(a2, a1, (long long (*)(long long, long long))sub_AC36);
}



// Function: rev_strcmp_btime @ 0xb6e2
long long rev_strcmp_btime(long long *a1, long long *a2)
{
  return sub_AF1E(a2, a1, (long long (*)(long long, long long))&strcmp);
}



// Function: xstrcoll_df_btime @ 0xb712
long long xstrcoll_df_btime(long long a1, long long a2)
{
  return sub_AD02(a1, a2, (long long (*)(long long, long long))sub_B652);
}



// Function: strcmp_df_btime @ 0xb742
long long strcmp_df_btime(long long a1, long long a2)
{
  return sub_AD02(a1, a2, (long long (*)(long long, long long))sub_B682);
}



// Function: rev_xstrcoll_df_btime @ 0xb772
long long rev_xstrcoll_df_btime(long long a1, long long a2)
{
  return sub_AD02(a1, a2, (long long (*)(long long, long long))sub_B6B2);
}



// Function: rev_strcmp_df_btime @ 0xb7a2
long long rev_strcmp_df_btime(long long a1, long long a2)
{
  return sub_AD02(a1, a2, (long long (*)(long long, long long))sub_B6E2);
}



// Function: xstrcoll_size @ 0xb7d2
long long xstrcoll_size(long long *a1, long long *a2)
{
  return sub_AFE2(a1, a2, (long long (*)(long long, long long))sub_AC36);
}



// Function: strcmp_size @ 0xb802
long long strcmp_size(long long *a1, long long *a2)
{
  return sub_AFE2(a1, a2, (long long (*)(long long, long long))&strcmp);
}



// Function: rev_xstrcoll_size @ 0xb832
long long rev_xstrcoll_size(long long *a1, long long *a2)
{
  return sub_AFE2(a2, a1, (long long (*)(long long, long long))sub_AC36);
}



// Function: rev_strcmp_size @ 0xb862
long long rev_strcmp_size(long long *a1, long long *a2)
{
  return sub_AFE2(a2, a1, (long long (*)(long long, long long))&strcmp);
}



// Function: xstrcoll_df_size @ 0xb892
long long xstrcoll_df_size(long long a1, long long a2)
{
  return sub_AD02(a1, a2, (long long (*)(long long, long long))sub_B7D2);
}



// Function: strcmp_df_size @ 0xb8c2
long long strcmp_df_size(long long a1, long long a2)
{
  return sub_AD02(a1, a2, (long long (*)(long long, long long))sub_B802);
}



// Function: rev_xstrcoll_df_size @ 0xb8f2
long long rev_xstrcoll_df_size(long long a1, long long a2)
{
  return sub_AD02(a1, a2, (long long (*)(long long, long long))sub_B832);
}



// Function: rev_strcmp_df_size @ 0xb922
long long rev_strcmp_df_size(long long a1, long long a2)
{
  return sub_AD02(a1, a2, (long long (*)(long long, long long))sub_B862);
}



// Function: xstrcoll_name @ 0xb952
long long xstrcoll_name(long long *a1, long long *a2)
{
  return sub_B03F(a1, a2, (long long (*)(long long, long long))sub_AC36);
}



// Function: strcmp_name @ 0xb982
long long strcmp_name(long long *a1, long long *a2)
{
  return sub_B03F(a1, a2, (long long (*)(long long, long long))&strcmp);
}



// Function: rev_xstrcoll_name @ 0xb9b2
long long rev_xstrcoll_name(long long *a1, long long *a2)
{
  return sub_B03F(a2, a1, (long long (*)(long long, long long))sub_AC36);
}



// Function: rev_strcmp_name @ 0xb9e2
long long rev_strcmp_name(long long *a1, long long *a2)
{
  return sub_B03F(a2, a1, (long long (*)(long long, long long))&strcmp);
}



// Function: xstrcoll_df_name @ 0xba12
long long xstrcoll_df_name(long long a1, long long a2)
{
  return sub_AD02(a1, a2, (long long (*)(long long, long long))sub_B952);
}



// Function: strcmp_df_name @ 0xba42
long long strcmp_df_name(long long a1, long long a2)
{
  return sub_AD02(a1, a2, (long long (*)(long long, long long))sub_B982);
}



// Function: rev_xstrcoll_df_name @ 0xba72
long long rev_xstrcoll_df_name(long long a1, long long a2)
{
  return sub_AD02(a1, a2, (long long (*)(long long, long long))sub_B9B2);
}



// Function: rev_strcmp_df_name @ 0xbaa2
long long rev_strcmp_df_name(long long a1, long long a2)
{
  return sub_AD02(a1, a2, (long long (*)(long long, long long))sub_B9E2);
}



// Function: xstrcoll_extension @ 0xbad2
long long xstrcoll_extension(const char **a1, const char **a2)
{
  return sub_B073(a1, a2, sub_AC36);
}



// Function: strcmp_extension @ 0xbb02
long long strcmp_extension(const char **a1, const char **a2)
{
  return sub_B073(a1, a2, (long long (*)(const char *, const char *))&strcmp);
}



// Function: rev_xstrcoll_extension @ 0xbb32
long long rev_xstrcoll_extension(const char **a1, const char **a2)
{
  return sub_B073(a2, a1, sub_AC36);
}



// Function: rev_strcmp_extension @ 0xbb62
long long rev_strcmp_extension(const char **a1, const char **a2)
{
  return sub_B073(a2, a1, (long long (*)(const char *, const char *))&strcmp);
}



// Function: xstrcoll_df_extension @ 0xbb92
long long xstrcoll_df_extension(long long a1, long long a2)
{
  return sub_AD02(a1, a2, (long long (*)(long long, long long))sub_BAD2);
}



// Function: strcmp_df_extension @ 0xbbc2
long long strcmp_df_extension(long long a1, long long a2)
{
  return sub_AD02(a1, a2, (long long (*)(long long, long long))sub_BB02);
}



// Function: rev_xstrcoll_df_extension @ 0xbbf2
long long rev_xstrcoll_df_extension(long long a1, long long a2)
{
  return sub_AD02(a1, a2, (long long (*)(long long, long long))sub_BB32);
}



// Function: rev_strcmp_df_extension @ 0xbc22
long long rev_strcmp_df_extension(long long a1, long long a2)
{
  return sub_AD02(a1, a2, (long long (*)(long long, long long))sub_BB62);
}



// Function: xstrcoll_width @ 0xbc52
long long xstrcoll_width(long long *a1, long long *a2)
{
  return sub_B16B(a1, a2, (long long (*)(long long, long long))sub_AC36);
}



// Function: strcmp_width @ 0xbc82
long long strcmp_width(long long *a1, long long *a2)
{
  return sub_B16B(a1, a2, (long long (*)(long long, long long))&strcmp);
}



// Function: rev_xstrcoll_width @ 0xbcb2
long long rev_xstrcoll_width(long long *a1, long long *a2)
{
  return sub_B16B(a2, a1, (long long (*)(long long, long long))sub_AC36);
}



// Function: rev_strcmp_width @ 0xbce2
long long rev_strcmp_width(long long *a1, long long *a2)
{
  return sub_B16B(a2, a1, (long long (*)(long long, long long))&strcmp);
}



// Function: xstrcoll_df_width @ 0xbd12
long long xstrcoll_df_width(long long a1, long long a2)
{
  return sub_AD02(a1, a2, (long long (*)(long long, long long))sub_BC52);
}



// Function: strcmp_df_width @ 0xbd42
long long strcmp_df_width(long long a1, long long a2)
{
  return sub_AD02(a1, a2, (long long (*)(long long, long long))sub_BC82);
}



// Function: rev_xstrcoll_df_width @ 0xbd72
long long rev_xstrcoll_df_width(long long a1, long long a2)
{
  return sub_AD02(a1, a2, (long long (*)(long long, long long))sub_BCB2);
}



// Function: rev_strcmp_df_width @ 0xbda2
long long rev_strcmp_df_width(long long a1, long long a2)
{
  return sub_AD02(a1, a2, (long long (*)(long long, long long))sub_BCE2);
}



// Function: cmp_version @ 0xbdd2
int cmp_version(const char **a1, const char **a2)
{
  int v3; // [rsp+1Ch] [rbp-4h]

  v3 = sub_11C04(*a1, *a2);
  if ( v3 )
    return v3;
  else
    return strcmp(*a1, *a2);
}



// Function: xstrcoll_version @ 0xbe28
int xstrcoll_version(const char **a1, const char **a2)
{
  return sub_BDD2(a1, a2);
}



// Function: rev_xstrcoll_version @ 0xbe51
int rev_xstrcoll_version(const char **a1, const char **a2)
{
  return sub_BDD2(a2, a1);
}



// Function: xstrcoll_df_version @ 0xbe7a
long long xstrcoll_df_version(long long a1, long long a2)
{
  return sub_AD02(a1, a2, (long long (*)(long long, long long))sub_BE28);
}



// Function: rev_xstrcoll_df_version @ 0xbeaa
long long rev_xstrcoll_df_version(long long a1, long long a2)
{
  return sub_AD02(a1, a2, (long long (*)(long long, long long))sub_BE51);
}



// Function: initialize_ordering_vector @ 0xbeda
long long initialize_ordering_vector()
{
  long long result; // rax
  unsigned long long i; // [rsp+0h] [rbp-8h]

  for ( i = 0; ; ++i )
  {
    result = qword_2B338;
    if ( i >= qword_2B338 )
      break;
    *((long long *)qword_2B348 + i) = qword_2B328 + 208 * i;
  }
  return result;
}



// Function: update_current_files_info @ 0xbf3d
long long update_current_files_info()
{
  long long result; // rax
  unsigned long long i; // [rsp+0h] [rbp-10h]
  long long v2; // [rsp+8h] [rbp-8h]

  if ( dword_2B3B4 == 2
    || (result = qword_2B430) != 0 && (dword_2B3AC == 2 || (result = (unsigned int)dword_2B3AC, dword_2B3AC == 3)) )
  {
    for ( i = 0; ; ++i )
    {
      result = qword_2B338;
      if ( i >= qword_2B338 )
        break;
      v2 = *((long long *)qword_2B348 + i);
      *(long long *)(v2 + 200) = sub_B119(v2);
    }
  }
  return result;
}



// Function: sort_files @ 0xbfc5
long long sort_files()
{
  long long result; // rax
  int v1; // eax
  unsigned char v2; // [rsp+Fh] [rbp-1h]

  if ( qword_2B338 + ((unsigned long long)qword_2B338 >> 1) > qword_2B350 )
  {
    free(qword_2B348);
    qword_2B348 = (void *)sub_1C804(qword_2B338, 24);
    qword_2B350 = 3 * qword_2B338;
  }
  sub_BEDA();
  sub_BF3D();
  result = (unsigned int)dword_2B3B4;
  if ( dword_2B3B4 != 6 )
  {
    if ( _setjmp(env) )
    {
      v2 = 1;
      if ( dword_2B3B4 == 4 )
        __assert_fail("sort_type != sort_version", "src/ls.c", 0x1008u, "sort_files");
      sub_BEDA();
    }
    else
    {
      v2 = 0;
    }
    if ( dword_2B3B4 == 5 )
      v1 = dword_2B3B0;
    else
      v1 = 0;
    return sub_15DFC(
             qword_2B348,
             qword_2B338,
             *(&off_2A780[8 * (unsigned int)(v1 + dword_2B3B4)]
             + 4 * v2
             + 2 * (unsigned char)byte_2B3B8
             + (unsigned char)byte_2B3F2));
  }
  return result;
}



// Function: print_current_files @ 0xc12d
long long print_current_files()
{
  long long result; // rax
  unsigned long long j; // [rsp+8h] [rbp-8h]
  unsigned long long i; // [rsp+8h] [rbp-8h]

  switch ( dword_2B3AC )
  {
    case 0:
      for ( i = 0; ; ++i )
      {
        result = qword_2B338;
        if ( i >= qword_2B338 )
          break;
        sub_63F3();
        sub_C6C8(*((long long *)qword_2B348 + i));
        sub_5422(byte_2B030);
      }
      break;
    case 1:
      for ( j = 0; ; ++j )
      {
        result = qword_2B338;
        if ( j >= qword_2B338 )
          break;
        sub_DD38(*((long long *)qword_2B348 + j), 0);
        putchar_unlocked(byte_2B030);
      }
      break;
    case 2:
      if ( !qword_2B430 )
        goto LABEL_7;
      result = sub_E708();
      break;
    case 3:
      if ( qword_2B430 )
        result = sub_E86A();
      else
LABEL_7:
        result = sub_E9D6(32);
      break;
    case 4:
      result = sub_E9D6(44);
      break;
    default:
      result = (unsigned int)dword_2B3AC;
      break;
  }
  return result;
}



// Function: align_nstrftime @ 0xc25f
long long align_nstrftime(long long a1, long long a2, unsigned char a3, long long a4, long long a5, unsigned int a6)
{
  char *v6; // rax

  if ( byte_2C340 )
    v6 = (char *)&unk_2B740 + 128 * (*(int *)(a4 + 16) + 12LL * a3);
  else
    v6 = off_2B200[a3];
  return sub_15F45(a1, a2, v6, a4, a5, a6);
}



// Function: long_time_expected_width @ 0xc308
long long long_time_expected_width()
{
  long long v1; // [rsp+0h] [rbp-450h] BYREF
  long long v2; // [rsp+8h] [rbp-448h]
  char v3[64]; // [rsp+10h] [rbp-440h] BYREF
  char v4[1016]; // [rsp+50h] [rbp-400h] BYREF
  unsigned long long v5; // [rsp+448h] [rbp-8h]

  v5 = __readfsqword(0x28u);
  if ( dword_2B034 < 0 )
  {
    v1 = 0;
    if ( sub_1B9A5(qword_2B438, &v1, v3) )
    {
      v2 = sub_C25F((long long)v4, 1001, 0, (long long)v3, qword_2B438, 0);
      if ( v2 )
        dword_2B034 = sub_15784(v4, v2, 0);
    }
    if ( dword_2B034 < 0 )
      dword_2B034 = 0;
  }
  return (unsigned int)dword_2B034;
}



// Function: format_user_or_group @ 0xc3fc
long long format_user_or_group(const char *a1, long long a2, int a3)
{
  int v3; // eax
  long long result; // rax
  unsigned int v5; // [rsp+28h] [rbp-8h]

  if ( a1 )
  {
    v3 = a3 - sub_1574E(a1, 0);
    if ( v3 < 0 )
      v3 = 0;
    v5 = v3;
    sub_549E(a1);
    do
    {
      sub_5422(32);
      result = v5--;
    }
    while ( (int)result );
  }
  else
  {
    result = printf("%*lu ", a3, a2) + qword_2B500;
    qword_2B500 = result;
  }
  return result;
}



// Function: format_user @ 0xc49e
long long format_user(unsigned int a1, int a2, char a3)
{
  const char *v3; // rax

  if ( a3 != 1 )
  {
    v3 = "?";
  }
  else if ( byte_2B3BA )
  {
    v3 = 0;
  }
  else
  {
    v3 = (const char *)sub_14A53(a1);
  }
  return sub_C3FC(v3, a1, a2);
}



// Function: format_group @ 0xc4fe
long long format_group(unsigned int a1, int a2, char a3)
{
  const char *v3; // rax

  if ( a3 != 1 )
  {
    v3 = "?";
  }
  else if ( byte_2B3BA )
  {
    v3 = 0;
  }
  else
  {
    v3 = (const char *)sub_14CA5(a1);
  }
  return sub_C3FC(v3, a1, a2);
}



// Function: format_user_or_group_width @ 0xc55e
int format_user_or_group_width(long long a1, long long a2)
{
  int result; // eax

  if ( !a1 )
    return snprintf(0, 0, "%lu", a2);
  result = sub_1574E(a1, 0);
  if ( result < 0 )
    return 0;
  return result;
}



// Function: format_user_width @ 0xc5c3
int format_user_width(unsigned int a1)
{
  long long v1; // rax

  if ( byte_2B3BA )
    v1 = 0;
  else
    v1 = sub_14A53(a1);
  return sub_C55E(v1, a1);
}



// Function: format_group_width @ 0xc603
int format_group_width(unsigned int a1)
{
  long long v1; // rax

  if ( byte_2B3BA )
    v1 = 0;
  else
    v1 = sub_14CA5(a1);
  return sub_C55E(v1, a1);
}



// Function: format_inode @ 0xc643
const char *format_inode(long long a1, unsigned long long a2, long long a3)
{
  if ( a2 <= 0x14 )
    __assert_fail("INT_BUFSIZE_BOUND (uintmax_t) <= buflen", "src/ls.c", 0x10BEu, "format_inode");
  if ( *(char *)(a3 + 184) && *(long long *)(a3 + 32) )
    return (const char *)sub_15035(*(long long *)(a3 + 32), a1);
  else
    return "?";
}



// Function: print_long_format @ 0xc6c8
unsigned long long print_long_format(long long a1)
{
  long long v1; // rdx
  long long v2; // rdx
  long long v3; // rdx
  const char *v4; // rax
  const char *v5; // rax
  int v6; // ebx
  char *v7; // rax
  char *v8; // rdx
  char *v9; // rax
  const char *v10; // rax
  unsigned int v11; // eax
  const char *v12; // rbx
  int v13; // r12d
  unsigned int v14; // eax
  const char *v15; // rcx
  int v16; // edx
  const char *v17; // rax
  long long v18; // r12
  unsigned int v19; // ebx
  long long v20; // rax
  int v21; // ebx
  char *v22; // rax
  char *v23; // rdx
  char *v24; // rax
  bool v25; // al
  char *v26; // rax
  const char *v27; // rbx
  int v28; // eax
  char v30; // [rsp+1Ah] [rbp-11B6h]
  int i; // [rsp+1Ch] [rbp-11B4h]
  int j; // [rsp+20h] [rbp-11B0h]
  int v33; // [rsp+24h] [rbp-11ACh]
  long long v34; // [rsp+28h] [rbp-11A8h]
  char *s; // [rsp+30h] [rbp-11A0h]
  char *sa; // [rsp+30h] [rbp-11A0h]
  char *sb; // [rsp+30h] [rbp-11A0h]
  const char *v38; // [rsp+38h] [rbp-1198h]
  const char *v39; // [rsp+40h] [rbp-1190h]
  long long v40; // [rsp+48h] [rbp-1188h]
  long long v41; // [rsp+50h] [rbp-1180h] BYREF
  long long v42; // [rsp+58h] [rbp-1178h]
  long long v43; // [rsp+60h] [rbp-1170h]
  long long v44; // [rsp+68h] [rbp-1168h]
  char v45[68]; // [rsp+70h] [rbp-1160h] BYREF
  char v46; // [rsp+B4h] [rbp-111Ch] BYREF
  char v47[3]; // [rsp+B5h] [rbp-111Bh] BYREF
  char v48; // [rsp+BEh] [rbp-1112h]
  char v49; // [rsp+BFh] [rbp-1111h]
  char v50[32]; // [rsp+C0h] [rbp-1110h] BYREF
  char v51[224]; // [rsp+E0h] [rbp-10F0h] BYREF
  char v52[3656]; // [rsp+370h] [rbp-E60h] BYREF
  unsigned long long v53; // [rsp+11B8h] [rbp-18h]

  v53 = __readfsqword(0x28u);
  v30 = 1;
  if ( *(char *)(a1 + 184) )
  {
    sub_1167B(a1 + 24, &v46);
  }
  else
  {
    v46 = aPcdbLswd[*(unsigned int *)(a1 + 168)];
    memset(v47, 63, 0xAu);
    v49 = 0;
  }
  if ( byte_2B381 != 1 )
  {
    v48 = 0;
  }
  else if ( *(int *)(a1 + 188) == 1 )
  {
    v48 = 46;
  }
  else if ( *(int *)(a1 + 188) == 2 )
  {
    v48 = 43;
  }
  if ( dword_2B3B0 == 3 )
  {
    v41 = sub_5990(a1 + 24);
    v42 = v3;
    if ( v41 == -1 && v42 == -1 )
      v30 = 0;
  }
  else
  {
    if ( (unsigned int)dword_2B3B0 > 3 )
      goto LABEL_22;
    if ( dword_2B3B0 == 2 )
    {
      v41 = sub_1B45D(a1 + 24);
      v42 = v2;
      goto LABEL_23;
    }
    if ( (unsigned int)dword_2B3B0 > 2 )
LABEL_22:
      abort();
    if ( dword_2B3B0 )
      v41 = sub_1B477(a1 + 24);
    else
      v41 = sub_1B491(a1 + 24);
    v42 = v1;
  }
LABEL_23:
  s = v52;
  if ( byte_2B3E9 )
  {
    v4 = sub_C643((long long)v51, 0x15u, a1);
    s = &v52[sprintf(v52, "%*s ", dword_2B384, v4)];
  }
  if ( byte_2B3BB )
  {
    if ( *(char *)(a1 + 184) != 1 )
      v5 = "?";
    else
      v5 = (const char *)sub_13D91(*(long long *)(a1 + 88), v51, (unsigned int)dword_2B3BC, 512, qword_2B3C0);
    v38 = v5;
    v6 = dword_2B388;
    for ( i = v6 - sub_1574E(v5, 0); i > 0; --i )
    {
      v7 = s++;
      *v7 = 32;
    }
    do
    {
      v8 = (char *)v38++;
      v9 = s++;
      *v9 = *v8;
    }
    while ( *v9 );
    *(s - 1) = 32;
  }
  if ( *(char *)(a1 + 184) != 1 )
    v10 = "?";
  else
    v10 = (const char *)sub_15035(*(long long *)(a1 + 40), v51);
  sa = &s[sprintf(s, "%s %*s ", &v46, dword_2B38C, v10)];
  sub_54CF();
  if ( byte_2B022 || byte_2B023 || byte_2B3B9 || byte_2B380 )
  {
    sub_5453(v52, sa - v52);
    if ( byte_2B022 )
      sub_C49E(*(int *)(a1 + 52), dword_2B394, *(char *)(a1 + 184));
    if ( byte_2B023 )
      sub_C4FE(*(int *)(a1 + 56), dword_2B398, *(char *)(a1 + 184));
    if ( byte_2B3B9 )
      sub_C49E(*(int *)(a1 + 52), dword_2B39C, *(char *)(a1 + 184));
    if ( byte_2B380 )
      sub_C3FC(*(const char **)(a1 + 176), 0, dword_2B390);
    sa = v52;
  }
  if ( *(char *)(a1 + 184) && ((*(int *)(a1 + 48) & 0xF000) == 0x2000 || (*(int *)(a1 + 48) & 0xF000) == 0x6000) )
  {
    v33 = dword_2B3A8 - (dword_2B3A0 + 2 + dword_2B3A4);
    v11 = gnu_dev_minor(*(long long *)(a1 + 64));
    v12 = (const char *)sub_15035(v11, v51);
    v13 = dword_2B3A4;
    v14 = gnu_dev_major(*(long long *)(a1 + 64));
    v15 = (const char *)sub_15035(v14, v50);
    v16 = 0;
    if ( v33 >= 0 )
      v16 = v33;
    sa += sprintf(sa, "%*s, %*s ", dword_2B3A0 + v16, v15, v13, v12);
  }
  else
  {
    if ( *(char *)(a1 + 184) != 1 )
    {
      v17 = "?";
    }
    else
    {
      v18 = qword_2B028;
      v19 = dword_2B3C8;
      v20 = sub_9635(*(long long *)(a1 + 72));
      v17 = (const char *)sub_13D91(v20, v51, v19, 1, v18);
    }
    v39 = v17;
    v21 = dword_2B3A8;
    for ( j = v21 - sub_1574E(v17, 0); j > 0; --j )
    {
      v22 = sa++;
      *v22 = 32;
    }
    do
    {
      v23 = (char *)v39++;
      v24 = sa++;
      *v24 = *v23;
    }
    while ( *v24 );
    *(sa - 1) = 32;
  }
  v34 = 0;
  *sa = 1;
  if ( *(char *)(a1 + 184) && v30 && sub_1B9A5(qword_2B438, &v41, v45) )
  {
    if ( (int)sub_1BC1F(qword_2B370, qword_2B378, v41, v42) < 0 )
      sub_11EC3(&qword_2B370);
    v43 = qword_2B370 - 15778476;
    v44 = qword_2B378;
    v25 = (int)sub_1BC1F(qword_2B370 - 15778476, qword_2B378, v41, v42) < 0
       && (int)sub_1BC1F(v41, v42, qword_2B370, qword_2B378) < 0;
    v34 = sub_C25F((long long)sa, 1001, v25, (long long)v45, qword_2B438, v42);
  }
  if ( !v34 && *sa )
  {
    if ( *(char *)(a1 + 184) && v30 )
      v27 = (const char *)sub_5219(v41, v51);
    else
      v27 = "?";
    v28 = sub_C308();
    sb = &sa[sprintf(sa, "%*s ", v28, v27)];
  }
  else
  {
    v26 = &sa[v34];
    sb = &sa[v34 + 1];
    *v26 = 32;
  }
  sub_5453(v52, sb - v52);
  v40 = sub_DB96(a1, 0, &unk_2B520, sb - v52);
  if ( *(int *)(a1 + 168) == 6 )
  {
    if ( *(long long *)(a1 + 8) )
    {
      sub_549E(" -> ");
      sub_DB96(a1, 1, 0, sb - v52 + v40 + 4);
      if ( dword_2B3D0 )
        sub_E05C(1, *(unsigned int *)(a1 + 172), 0);
    }
  }
  else if ( dword_2B3D0 )
  {
    sub_E05C(*(unsigned char *)(a1 + 184), *(unsigned int *)(a1 + 48), *(unsigned int *)(a1 + 168));
  }
  return v53 - __readfsqword(0x28u);
}



// Function: quote_name_buf @ 0xd10c
size_t quote_name_buf(void **a1, size_t a2, char *a3, long long a4, int a5, size_t *a6, bool *a7)
{
  bool v7; // al
  bool v8; // al
  int v9; // eax
  char *v10; // rdx
  char *v11; // rax
  char *v12; // rax
  char *v13; // rax
  char *v14; // rdx
  char *v15; // rax
  char *v16; // rax
  const unsigned short *v17; // rbx
  const unsigned short *v18; // rbx
  bool v19; // al
  bool v25; // [rsp+4Ah] [rbp-86h]
  bool v26; // [rsp+4Bh] [rbp-85h]
  wchar_t c; // [rsp+4Ch] [rbp-84h] BYREF
  int v28; // [rsp+50h] [rbp-80h]
  int v29; // [rsp+54h] [rbp-7Ch]
  void *dest; // [rsp+58h] [rbp-78h]
  size_t v31; // [rsp+60h] [rbp-70h]
  size_t v32; // [rsp+68h] [rbp-68h]
  char *v33; // [rsp+70h] [rbp-60h]
  char *v34; // [rsp+78h] [rbp-58h]
  long long v35; // [rsp+80h] [rbp-50h]
  char *v36; // [rsp+88h] [rbp-48h]
  char *v37; // [rsp+90h] [rbp-40h]
  char *v38; // [rsp+98h] [rbp-38h]
  char *v39; // [rsp+A0h] [rbp-30h]
  char *v40; // [rsp+A8h] [rbp-28h]
  mbstate_t ps; // [rsp+B0h] [rbp-20h] BYREF
  unsigned long long v42; // [rsp+B8h] [rbp-18h]

  v42 = __readfsqword(0x28u);
  dest = *a1;
  v32 = 0;
  v28 = sub_18FC3(a4);
  v7 = byte_2B408 && (v28 == 1 || v28 == 2 || !v28);
  v26 = v7;
  if ( a5 )
  {
    v32 = sub_1A389(dest, a2, a3, -1, a4);
    if ( a2 <= v32 )
    {
      dest = (void *)sub_1C68B(v32 + 1);
      sub_1A389(dest, v32 + 1, a3, -1, a4);
    }
    v8 = *a3 != *(char *)dest || v32 != strlen(a3);
    v25 = v8;
  }
  else if ( v7 )
  {
    v32 = strlen(a3);
    if ( a2 <= v32 )
      dest = (void *)sub_1C68B(v32 + 1);
    memcpy(dest, a3, v32 + 1);
    v25 = 0;
  }
  else
  {
    v32 = strlen(a3);
    dest = a3;
    v25 = 0;
  }
  if ( v26 )
  {
    if ( __ctype_get_mb_cur_max() > 1 )
    {
      v33 = (char *)dest;
      v40 = (char *)dest + v32;
      v34 = (char *)dest;
      v31 = 0;
      while ( 1 )
      {
LABEL_46:
        if ( v33 >= v40 )
        {
          v32 = v34 - (char *)dest;
          goto LABEL_62;
        }
        v9 = *v33;
        if ( v9 == 127 )
          goto LABEL_32;
        if ( v9 >= 97 )
          goto LABEL_31;
        if ( v9 > 95 )
          goto LABEL_32;
        if ( v9 >= 65 )
          goto LABEL_31;
        if ( v9 > 35 )
        {
          if ( (unsigned int)(v9 - 37) <= 0x1A )
            goto LABEL_31;
LABEL_32:
          ps = 0;
          while ( 1 )
          {
            v35 = sub_150BE(&c, v33, v40 - v33, &ps);
            if ( v35 == -1 )
            {
              ++v33;
              v12 = v34++;
              *v12 = 63;
              ++v31;
              goto LABEL_46;
            }
            if ( v35 == -2 )
              break;
            if ( !v35 )
              v35 = 1;
            v29 = wcwidth(c);
            if ( v29 < 0 )
            {
              v33 += v35;
              v16 = v34++;
              *v16 = 63;
              ++v31;
            }
            else
            {
              while ( v35 )
              {
                v14 = v33++;
                v15 = v34++;
                *v15 = *v14;
                --v35;
              }
              v31 += v29;
            }
            if ( mbsinit(&ps) )
              goto LABEL_46;
          }
          v33 = v40;
          v13 = v34++;
          *v13 = 63;
          ++v31;
        }
        else
        {
          if ( v9 < 32 )
            goto LABEL_32;
LABEL_31:
          v10 = v33++;
          v11 = v34++;
          *v11 = *v10;
          ++v31;
        }
      }
    }
    v36 = (char *)dest;
    v39 = (char *)dest + v32;
    while ( v36 < v39 )
    {
      v17 = *__ctype_b_loc();
      if ( (v17[(unsigned char)sub_4EB2((unsigned int)*v36)] & 0x4000) == 0 )
        *v36 = 63;
      ++v36;
    }
    v31 = v32;
  }
  else if ( a6 )
  {
    if ( __ctype_get_mb_cur_max() <= 1 )
    {
      v37 = (char *)dest;
      v38 = (char *)dest + v32;
      v31 = 0;
      while ( v37 < v38 )
      {
        v18 = *__ctype_b_loc();
        if ( (v18[(unsigned char)sub_4EB2((unsigned int)*v37)] & 0x4000) != 0 )
          ++v31;
        ++v37;
      }
    }
    else
    {
      v31 = (int)sub_15784(dest, v32, 0);
    }
  }
LABEL_62:
  v19 = byte_2B341 && byte_2B340 && !v25;
  *a7 = v19;
  if ( a6 )
    *a6 = v31;
  *a1 = dest;
  return v32;
}



// Function: quote_name_width @ 0xd671
size_t quote_name_width(char *a1, long long a2, int a3)
{
  bool v4; // [rsp+37h] [rbp-2021h] BYREF
  void *ptr; // [rsp+38h] [rbp-2020h] BYREF
  size_t v6; // [rsp+40h] [rbp-2018h] BYREF
  char v7[16]; // [rsp+48h] [rbp-2010h] BYREF
  unsigned long long v8; // [rsp+2050h] [rbp-8h]

  v8 = __readfsqword(0x28u);
  ptr = v7;
  sub_D10C(&ptr, 0x2000u, a1, a2, a3, &v6, &v4);
  if ( ptr != v7 && a1 != ptr )
    free(ptr);
  return v4 + v6;
}



// Function: file_escape @ 0xd775
long long file_escape(const char *a1, char a2)
{
  size_t v2; // rax
  char *v3; // rax
  char *v4; // rdx
  char *v5; // rax
  char *v6; // rax
  unsigned char v7; // al
  char *s; // [rsp+8h] [rbp-18h]
  char *v10; // [rsp+10h] [rbp-10h]
  long long v11; // [rsp+18h] [rbp-8h]

  s = (char *)a1;
  v2 = strlen(a1);
  v11 = sub_1C804(3, v2 + 1);
  v10 = (char *)v11;
  while ( *s )
  {
    if ( a2 && *s == 47 )
    {
      v3 = v10++;
      *v3 = 47;
      ++s;
    }
    else if ( byte_2B640[(unsigned char)sub_4EB2((unsigned int)*s)] )
    {
      v4 = s++;
      v5 = v10++;
      *v5 = *v4;
    }
    else
    {
      v6 = s++;
      v7 = sub_4EB2((unsigned int)*v6);
      v10 += sprintf(v10, "%%%02x", v7);
    }
  }
  *v10 = 0;
  return v11;
}



// Function: quote_name @ 0xd87f
size_t quote_name(char *a1, long long a2, int a3, long long a4, char a5, long long a6, const char *a7)
{
  const char *v7; // rax
  bool v12; // [rsp+46h] [rbp-2032h] BYREF
  unsigned char v13; // [rsp+47h] [rbp-2031h]
  void *v14; // [rsp+48h] [rbp-2030h] BYREF
  size_t v15; // [rsp+50h] [rbp-2028h]
  void *ptr; // [rsp+58h] [rbp-2020h]
  void *v17; // [rsp+60h] [rbp-2018h]
  char v18[16]; // [rsp+68h] [rbp-2010h] BYREF
  unsigned long long v19; // [rsp+2070h] [rbp-8h]

  v19 = __readfsqword(0x28u);
  v14 = v18;
  v15 = sub_D10C(&v14, 0x2000u, a1, a2, a3, 0, &v12);
  if ( v12 && a5 )
    sub_5422(32);
  if ( a4 )
    sub_E0A3(a4);
  v13 = 0;
  if ( a7 )
  {
    if ( byte_2B341 && byte_2B340 && !v12 )
    {
      v13 = 1;
      putchar_unlocked(*(char *)v14);
    }
    ptr = (void *)sub_D775((const char *)qword_2B360, 0);
    v17 = (void *)sub_D775(a7, 1);
    if ( *(char *)v17 == 47 )
      v7 = locale;
    else
      v7 = "/";
    printf("\x1B]8;;file://%s%s%s\a", (const char *)ptr, v7, (const char *)v17);
    free(ptr);
    free(v17);
  }
  if ( a6 )
    sub_54F4(a6);
  fwrite_unlocked((char *)v14 + v13, 1u, v15 - 2 * v13, stdout);
  qword_2B500 += v15;
  if ( a6 )
    sub_54F4(a6);
  if ( a7 )
  {
    fputs_unlocked("\x1B]8;;\a", stdout);
    if ( v13 )
      putchar_unlocked(*((char *)v14 + v15 - 1));
  }
  if ( v14 != v18 && a1 != v14 )
    free(v14);
  return v12 + v15;
}



// Function: print_name_with_quoting @ 0xdb96
size_t print_name_with_quoting(long long a1, unsigned char a2, long long a3, unsigned long long a4)
{
  char *v4; // rax
  long long v5; // rax
  bool v6; // al
  bool v10; // [rsp+27h] [rbp-19h]
  char *v11; // [rsp+28h] [rbp-18h]
  long long v12; // [rsp+30h] [rbp-10h]
  size_t v13; // [rsp+38h] [rbp-8h]

  if ( a2 )
    v4 = *(char **)(a1 + 8);
  else
    v4 = *(char **)a1;
  v11 = v4;
  if ( byte_2B3D4 )
    v5 = sub_E101(a1, a2);
  else
    v5 = 0;
  v12 = v5;
  v6 = byte_2B3D4 && (v5 || sub_6325(4u));
  v10 = v6;
  v13 = sub_D87F(v11, qword_2B410, *(int *)(a1 + 196), v12, a2 == 0, a3, *(const char **)(a1 + 16));
  sub_648C();
  if ( v10 )
  {
    sub_DCE3();
    if ( qword_2B430 )
    {
      if ( a4 / qword_2B430 != (a4 + v13 - 1) / qword_2B430 )
        sub_E4CA(&unk_2B1F0);
    }
  }
  return v13;
}



// Function: prep_non_filename_text @ 0xdce3
long long prep_non_filename_text()
{
  if ( qword_2B0A8 )
    return sub_E4CA(&unk_2B0A0);
  sub_E4CA(qword_2B080);
  sub_E4CA(&unk_2B0B0);
  return sub_E4CA(&qword_2B090);
}



// Function: print_file_name_and_frills @ 0xdd38
size_t print_file_name_and_frills(long long a1, unsigned long long a2)
{
  const char *v2; // rdx
  int v3; // eax
  const char *v4; // rax
  int v5; // ecx
  int v6; // eax
  size_t v8; // [rsp+18h] [rbp-2A8h]
  char v9[664]; // [rsp+20h] [rbp-2A0h] BYREF
  unsigned long long v10; // [rsp+2B8h] [rbp-8h]

  v10 = __readfsqword(0x28u);
  sub_63F3();
  if ( byte_2B3E9 )
  {
    v2 = sub_C643((long long)v9, 0x28Cu, a1);
    if ( dword_2B3AC == 4 )
      v3 = 0;
    else
      v3 = dword_2B384;
    printf("%*s ", v3, v2);
  }
  if ( byte_2B3BB )
  {
    if ( *(char *)(a1 + 184) != 1 )
      v4 = "?";
    else
      v4 = (const char *)sub_13D91(*(long long *)(a1 + 88), v9, (unsigned int)dword_2B3BC, 512, qword_2B3C0);
    if ( dword_2B3AC == 4 )
      v5 = 0;
    else
      v5 = dword_2B388;
    printf("%*s ", v5, v4);
  }
  if ( byte_2B380 )
  {
    if ( dword_2B3AC == 4 )
      v6 = 0;
    else
      v6 = dword_2B390;
    printf("%*s ", v6, *(const char **)(a1 + 176));
  }
  v8 = sub_DB96(a1, 0, 0, a2);
  if ( dword_2B3D0 )
    v8 += (unsigned char)sub_E05C(
                             *(unsigned char *)(a1 + 184),
                             *(unsigned int *)(a1 + 48),
                             *(unsigned int *)(a1 + 168));
  return v8;
}



// Function: get_type_indicator @ 0xdf23
long long get_type_indicator(char a1, short a2, int a3)
{
  bool v3; // al
  bool v4; // al
  bool v5; // al
  bool v6; // al
  bool v7; // al

  if ( a1 )
    v3 = (a2 & 0xF000) == 0x8000;
  else
    v3 = a3 == 5;
  if ( v3 )
  {
    if ( a1 && dword_2B3D0 == 3 && (a2 & 0x49) != 0 )
      return 42;
    else
      return 0;
  }
  else
  {
    if ( a1 )
      v4 = (a2 & 0xF000) == 0x4000;
    else
      v4 = a3 == 3 || a3 == 9;
    if ( v4 )
    {
      return 47;
    }
    else if ( dword_2B3D0 == 1 )
    {
      return 0;
    }
    else
    {
      if ( a1 )
        v5 = (a2 & 0xF000) == 40960;
      else
        v5 = a3 == 6;
      if ( v5 )
      {
        return 64;
      }
      else
      {
        if ( a1 )
          v6 = (a2 & 0xF000) == 4096;
        else
          v6 = a3 == 1;
        if ( v6 )
        {
          return 124;
        }
        else
        {
          if ( a1 )
            v7 = (a2 & 0xF000) == 49152;
          else
            v7 = a3 == 7;
          if ( v7 )
            return 61;
          else
            return 0;
        }
      }
    }
  }
}



// Function: print_type_indicator @ 0xe05c
bool print_type_indicator(char a1, short a2, int a3)
{
  char v4; // [rsp+1Fh] [rbp-1h]

  v4 = sub_DF23(a1, a2, a3);
  if ( v4 )
    sub_5422(v4);
  return v4 != 0;
}



// Function: print_color_indicator @ 0xe0a3
bool print_color_indicator(long long a1)
{
  if ( a1 )
  {
    if ( sub_6325(4u) )
      sub_63CA();
    sub_E4CA(qword_2B080);
    sub_E4CA(a1);
    sub_E4CA(&qword_2B090);
  }
  return a1 != 0;
}



// Function: get_color_indicator @ 0xe101
long long *get_color_indicator(long long a1, char a2)
{
  int v2; // eax
  long long *result; // rax
  unsigned int v4; // [rsp+14h] [rbp-2Ch]
  int v5; // [rsp+18h] [rbp-28h]
  int v6; // [rsp+1Ch] [rbp-24h]
  size_t *i; // [rsp+20h] [rbp-20h]
  char *s; // [rsp+28h] [rbp-18h]
  char *sa; // [rsp+28h] [rbp-18h]
  size_t v10; // [rsp+30h] [rbp-10h]

  if ( a2 )
  {
    s = *(char **)(a1 + 8);
    v5 = *(int *)(a1 + 172);
    if ( *(char *)(a1 + 185) )
      v2 = 0;
    else
      v2 = -1;
    v6 = v2;
  }
  else
  {
    s = *(char **)a1;
    LOWORD(v5) = sub_53E7(a1);
    v6 = *(unsigned char *)(a1 + 185);
  }
  if ( v6 == -1 && sub_6325(0xCu) )
  {
    v4 = 12;
  }
  else if ( *(char *)(a1 + 184) != 1 )
  {
    v4 = dword_2B040[*(unsigned int *)(a1 + 168)];
  }
  else
  {
    switch ( v5 & 0xF000 )
    {
      case 32768:
        v4 = 5;
        if ( (v5 & 0x800) != 0 && sub_6325(0x10u) )
        {
          v4 = 16;
        }
        else if ( (v5 & 0x400) != 0 && sub_6325(0x11u) )
        {
          v4 = 17;
        }
        else if ( sub_6325(0x15u) && *(char *)(a1 + 192) )
        {
          v4 = 21;
        }
        else if ( (v5 & 0x49) != 0 && sub_6325(0xEu) )
        {
          v4 = 14;
        }
        else if ( *(long long *)(a1 + 40) > 1u && sub_6325(0x16u) )
        {
          v4 = 22;
        }
        break;
      case 16384:
        v4 = 6;
        if ( (v5 & 0x200) != 0 && (v5 & 2) != 0 && sub_6325(0x14u) )
        {
          v4 = 20;
        }
        else if ( (v5 & 2) != 0 && sub_6325(0x13u) )
        {
          v4 = 19;
        }
        else if ( (v5 & 0x200) != 0 && sub_6325(0x12u) )
        {
          v4 = 18;
        }
        break;
      case 40960:
        v4 = 7;
        break;
      case 4096:
        v4 = 8;
        break;
      case 49152:
        v4 = 9;
        break;
      case 24576:
        v4 = 10;
        break;
      case 8192:
        v4 = 11;
        break;
      default:
        v4 = 13;
        break;
    }
  }
  i = 0;
  if ( v4 == 5 )
  {
    v10 = strlen(s);
    sa = &s[v10];
    for ( i = (size_t *)qword_2B3D8; i && (v10 < *i || (unsigned int)sub_102B5(&sa[-*i], i[1], *i)); i = (size_t *)i[4] )
      ;
  }
  if ( v4 == 7 && !v6 && (byte_2B358 || sub_6325(0xDu)) )
    v4 = 13;
  if ( i )
    result = (long long *)(i + 2);
  else
    result = &qword_2B080[2 * v4];
  if ( !result[1] )
    return 0;
  return result;
}



// Function: put_indicator @ 0xe4ca
size_t put_indicator(long long a1)
{
  if ( byte_2B3D6 != 1 )
  {
    byte_2B3D6 = 1;
    if ( tcgetpgrp(1) >= 0 )
      sub_6858();
    sub_DCE3();
  }
  return fwrite_unlocked(*(const void **)(a1 + 8), *(long long *)a1, 1u, stdout);
}



// Function: length_of_file_name_and_frills @ 0xe530
long long length_of_file_name_and_frills(long long a1)
{
  const char *v1; // rax
  long long v2; // rax
  const char *v3; // rax
  long long v4; // rax
  long long v5; // rax
  long long v7; // [rsp+18h] [rbp-2A8h]
  long long v8; // [rsp+18h] [rbp-2A8h]
  char v9[664]; // [rsp+20h] [rbp-2A0h] BYREF
  unsigned long long v10; // [rsp+2B8h] [rbp-8h]

  v10 = __readfsqword(0x28u);
  v7 = 0;
  if ( byte_2B3E9 )
  {
    if ( dword_2B3AC == 4 )
    {
      v1 = (const char *)sub_15035(*(long long *)(a1 + 32), v9);
      v2 = strlen(v1) + 1;
    }
    else
    {
      v2 = dword_2B384 + 1LL;
    }
    v7 = v2;
  }
  if ( byte_2B3BB )
  {
    if ( dword_2B3AC == 4 )
    {
      if ( *(char *)(a1 + 184) != 1 )
        v3 = "?";
      else
        v3 = (const char *)sub_13D91(*(long long *)(a1 + 88), v9, (unsigned int)dword_2B3BC, 512, qword_2B3C0);
      v4 = strlen(v3) + 1;
    }
    else
    {
      v4 = dword_2B388 + 1LL;
    }
    v7 += v4;
  }
  if ( byte_2B380 )
  {
    if ( dword_2B3AC == 4 )
      v5 = strlen(*(const char **)(a1 + 176)) + 1;
    else
      v5 = dword_2B390 + 1LL;
    v7 += v5;
  }
  v8 = sub_B119(a1) + v7;
  if ( dword_2B3D0 )
    v8 += (unsigned char)sub_DF23(*(char *)(a1 + 184), *(int *)(a1 + 48), *(int *)(a1 + 168)) != 0;
  return v8;
}



// Function: print_many_per_line @ 0xe708
unsigned long long print_many_per_line()
{
  long long v0; // rax
  unsigned long long result; // rax
  unsigned long long i; // [rsp+0h] [rbp-50h]
  long long v3; // [rsp+8h] [rbp-48h]
  unsigned long long v4; // [rsp+10h] [rbp-40h]
  unsigned long long j; // [rsp+18h] [rbp-38h]
  unsigned long long v6; // [rsp+20h] [rbp-30h]
  long long v7; // [rsp+28h] [rbp-28h]
  unsigned long long v8; // [rsp+30h] [rbp-20h]
  long long v9; // [rsp+38h] [rbp-18h]
  long long v10; // [rsp+40h] [rbp-10h]
  long long v11; // [rsp+48h] [rbp-8h]

  v6 = sub_EE73(1);
  v7 = qword_2B4F0 + 24 * v6 - 24;
  v8 = qword_2B338 / v6 + (qword_2B338 % v6 != 0);
  for ( i = 0; ; ++i )
  {
    result = i;
    if ( i >= v8 )
      break;
    v3 = 0;
    v4 = i;
    for ( j = 0; ; j += v11 )
    {
      v9 = *((long long *)qword_2B348 + v4);
      v10 = sub_E530(v9);
      v0 = v3++;
      v11 = *(long long *)(*(long long *)(v7 + 16) + 8 * v0);
      sub_DD38(v9, j);
      v4 += v8;
      if ( v4 >= qword_2B338 )
        break;
      sub_EAEF(j + v10, v11 + j);
    }
    putchar_unlocked(byte_2B030);
  }
  return result;
}



// Function: print_horizontal @ 0xe86a
int print_horizontal()
{
  unsigned long long i; // [rsp+0h] [rbp-40h]
  unsigned long long v2; // [rsp+8h] [rbp-38h]
  long long v3; // [rsp+10h] [rbp-30h]
  long long v4; // [rsp+18h] [rbp-28h]
  unsigned long long v5; // [rsp+20h] [rbp-20h]
  long long v6; // [rsp+28h] [rbp-18h]
  long long v7; // [rsp+30h] [rbp-10h]
  long long v8; // [rsp+30h] [rbp-10h]

  v2 = 0;
  v5 = sub_EE73(0);
  v6 = qword_2B4F0 + 24 * v5 - 24;
  v7 = *(long long *)qword_2B348;
  v3 = sub_E530(*(long long *)qword_2B348);
  v4 = **(long long **)(v6 + 16);
  sub_DD38(v7, 0);
  for ( i = 1; i < qword_2B338; ++i )
  {
    if ( i % v5 )
    {
      sub_EAEF(v2 + v3, v4 + v2);
      v2 += v4;
    }
    else
    {
      putchar_unlocked(byte_2B030);
      v2 = 0;
    }
    v8 = *((long long *)qword_2B348 + i);
    sub_DD38(v8, v2);
    v3 = sub_E530(v8);
    v4 = *(long long *)(*(long long *)(v6 + 16) + 8 * (i % v5));
  }
  return putchar_unlocked(byte_2B030);
}



// Function: print_with_separator @ 0xe9d6
int print_with_separator(char a1)
{
  long long v1; // rax
  char v3; // [rsp+1Fh] [rbp-21h]
  unsigned long long i; // [rsp+20h] [rbp-20h]
  unsigned long long v5; // [rsp+28h] [rbp-18h]
  long long v6; // [rsp+30h] [rbp-10h]
  long long v7; // [rsp+38h] [rbp-8h]

  v5 = 0;
  for ( i = 0; i < qword_2B338; ++i )
  {
    v6 = *((long long *)qword_2B348 + i);
    if ( qword_2B430 )
      v1 = sub_E530(v6);
    else
      v1 = 0;
    v7 = v1;
    if ( i )
    {
      if ( qword_2B430 && (v5 + v1 + 2 >= qword_2B430 || v5 > -3 - v1) )
      {
        v5 = 0;
        v3 = byte_2B030;
      }
      else
      {
        v5 += 2LL;
        v3 = 32;
      }
      putchar_unlocked(a1);
      putchar_unlocked(v3);
    }
    sub_DD38(v6, v5);
    v5 += v7;
  }
  return putchar_unlocked(byte_2B030);
}



// Function: indent @ 0xeaef
unsigned long long indent(unsigned long long a1, unsigned long long a2)
{
  unsigned long long result; // rax

  while ( 1 )
  {
    result = a1;
    if ( a1 >= a2 )
      break;
    if ( qword_2B420 && a2 / qword_2B420 > (a1 + 1) / qword_2B420 )
    {
      putchar_unlocked(9);
      a1 += qword_2B420 - a1 % qword_2B420;
    }
    else
    {
      putchar_unlocked(32);
      ++a1;
    }
  }
  return result;
}



// Function: attach @ 0xeb94
char *attach(char *a1, char *a2, char *a3)
{
  char *v3; // rdx
  char *v4; // rax
  char *v5; // rax
  char *v6; // rdx
  char *v7; // rax
  char *v11; // [rsp+20h] [rbp-8h]

  v11 = a2;
  if ( *a2 != 46 || a2[1] )
  {
    while ( *v11 )
    {
      v3 = v11++;
      v4 = a1++;
      *v4 = *v3;
    }
    if ( v11 > a2 && *(v11 - 1) != 47 )
    {
      v5 = a1++;
      *v5 = 47;
    }
  }
  while ( *a3 )
  {
    v6 = a3++;
    v7 = a1++;
    *v7 = *v6;
  }
  *a1 = 0;
  return a1;
}



// Function: init_column_info @ 0xec50
unsigned long long init_column_info(unsigned long long a1)
{
  unsigned long long result; // rax
  unsigned long long i; // [rsp+18h] [rbp-38h]
  unsigned long long j; // [rsp+18h] [rbp-38h]
  unsigned long long v4; // [rsp+20h] [rbp-30h]
  long long v5; // [rsp+28h] [rbp-28h]
  unsigned long long k; // [rsp+30h] [rbp-20h]
  unsigned long long v7; // [rsp+48h] [rbp-8h]

  if ( a1 > qword_2C440 )
  {
    if ( qword_2B4F8 && a1 >= (unsigned long long)qword_2B4F8 >> 1 )
    {
      qword_2B4F0 = sub_1C638(qword_2B4F0, qword_2B4F8, 24);
      v4 = qword_2B4F8;
    }
    else
    {
      qword_2B4F0 = sub_1C638(qword_2B4F0, a1, 48);
      v4 = 2 * a1;
    }
    v7 = (v4 - qword_2C440) * (qword_2C440 + v4 + 1);
    if ( qword_2C440 + v4 + 1 < v4 || qword_2C440 + v4 + 1 != v7 / (v4 - qword_2C440) )
      sub_1CCC3();
    v5 = sub_1C804(v7 >> 1, 8);
    for ( i = qword_2C440; i < v4; ++i )
    {
      *(long long *)(qword_2B4F0 + 24 * i + 16) = v5;
      v5 += 8 * (i + 1);
    }
    qword_2C440 = v4;
  }
  for ( j = 0; ; ++j )
  {
    result = j;
    if ( j >= a1 )
      break;
    *(char *)(qword_2B4F0 + 24 * j) = 1;
    *(long long *)(qword_2B4F0 + 24 * j + 8) = 3 * (j + 1);
    for ( k = 0; k <= j; ++k )
      *(long long *)(*(long long *)(qword_2B4F0 + 24 * j + 16) + 8 * k) = 3;
  }
  return result;
}



// Function: calculate_columns @ 0xee73
unsigned long long calculate_columns(char a1)
{
  unsigned long long v1; // rax
  unsigned long long v2; // rax
  long long v3; // rdx
  unsigned long long i; // [rsp+10h] [rbp-40h]
  unsigned long long k; // [rsp+18h] [rbp-38h]
  unsigned long long j; // [rsp+20h] [rbp-30h]
  unsigned long long v8; // [rsp+28h] [rbp-28h]
  long long v9; // [rsp+38h] [rbp-18h]
  long long v10; // [rsp+48h] [rbp-8h]

  if ( qword_2B4F8 && qword_2B4F8 < (unsigned long long)qword_2B338 )
    v1 = qword_2B4F8;
  else
    v1 = qword_2B338;
  v8 = v1;
  sub_EC50(v1);
  for ( i = 0; i < qword_2B338; ++i )
  {
    v9 = sub_E530(*((long long *)qword_2B348 + i));
    for ( j = 0; j < v8; ++j )
    {
      if ( *(char *)(qword_2B4F0 + 24 * j) )
      {
        v2 = a1 ? i / ((qword_2B338 + j) / (j + 1)) : i % (j + 1);
        v3 = v2 == j ? 0LL : 2LL;
        v10 = v3 + v9;
        if ( (unsigned long long)(v3 + v9) > *(long long *)(*(long long *)(qword_2B4F0 + 24 * j + 16) + 8 * v2) )
        {
          *(long long *)(qword_2B4F0 + 24 * j + 8) += v10 - *(long long *)(*(long long *)(qword_2B4F0 + 24 * j + 16) + 8 * v2);
          *(long long *)(8 * v2 + *(long long *)(qword_2B4F0 + 24 * j + 16)) = v10;
          *(char *)(qword_2B4F0 + 24 * j) = *(long long *)(qword_2B4F0 + 24 * j + 8) < (unsigned long long)qword_2B430;
        }
      }
    }
  }
  for ( k = v8; k > 1 && !*(char *)(qword_2B4F0 + 24 * k - 24); --k )
    ;
  return k;
}



// Function: usage @ 0xf135
void usage(int a1)
{
  long long v1; // rbx
  char *v2; // rax
  long long v3; // rbx
  char *v4; // rax
  FILE *v5; // rbx
  char *v6; // rax
  FILE *v7; // rbx
  char *v8; // rax
  FILE *v9; // rbx
  char *v10; // rax
  FILE *v11; // rbx
  char *v12; // rax
  FILE *v13; // rbx
  char *v14; // rax
  FILE *v15; // rbx
  char *v16; // rax
  FILE *v17; // rbx
  char *v18; // rax
  FILE *v19; // rbx
  char *v20; // rax
  FILE *v21; // rbx
  char *v22; // rax
  FILE *v23; // rbx
  char *v24; // rax
  FILE *v25; // rbx
  char *v26; // rax
  FILE *v27; // rbx
  char *v28; // rax
  FILE *v29; // rbx
  char *v30; // rax
  FILE *v31; // rbx
  char *v32; // rax
  FILE *v33; // rbx
  char *v34; // rax
  FILE *v35; // rbx
  char *v36; // rax
  FILE *v37; // rbx
  char *v38; // rax
  FILE *v39; // rbx
  char *v40; // rax
  FILE *v41; // rbx
  char *v42; // rax
  FILE *v43; // rbx
  char *v44; // rax
  FILE *v45; // rbx
  char *v46; // rax
  FILE *v47; // rbx
  char *v48; // rax
  FILE *v49; // rbx
  char *v50; // rax
  FILE *v51; // rbx
  char *v52; // rax
  FILE *v53; // rbx
  char *v54; // rax
  FILE *v55; // rbx
  char *v56; // rax
  FILE *v57; // rbx
  char *v58; // rax
  FILE *v59; // rbx
  char *v60; // rax
  FILE *v61; // rbx
  char *v62; // rax
  FILE *v63; // rbx
  char *v64; // rax
  FILE *v65; // rbx
  char *v66; // rax
  FILE *v67; // rbx
  char *v68; // rax
  FILE *v69; // rbx
  char *v70; // rax
  FILE *v71; // rbx
  char *v72; // rax
  FILE *v73; // rbx
  char *v74; // rax
  FILE *v75; // rbx
  char *v76; // rax
  FILE *v77; // rbx
  char *v78; // rax
  FILE *v79; // rbx
  char *v80; // rax
  FILE *v81; // rbx
  char *v82; // rax
  FILE *v83; // rbx
  char *v84; // rdi
  FILE *v85; // rbx
  char *v86; // rax
  FILE *v87; // rbx
  char *v88; // rax
  FILE *v89; // rbx
  char *v90; // rax
  FILE *v91; // rbx
  char *v92; // rax
  const char *v93; // rax

  if ( a1 )
  {
    v1 = qword_2C478;
    v2 = gettext("Try '%s --help' for more information.\n");
    fprintf(stderr, v2, v1);
  }
  else
  {
    v3 = qword_2C478;
    v4 = gettext("Usage: %s [OPTION]... [FILE]...\n");
    printf(v4, v3);
    v5 = stdout;
    v6 = gettext(
           "List information about the FILEs (the current directory by default).\n"
           "Sort entries alphabetically if none of -cftuvSUX nor --sort is specified.\n");
    fputs_unlocked(v6, v5);
    sub_4F20();
    v7 = stdout;
    v8 = gettext(
           "  -a, --all                  do not ignore entries starting with .\n"
           "  -A, --almost-all           do not list implied . and ..\n"
           "      --author               with -l, print the author of each file\n"
           "  -b, --escape               print C-style escapes for nongraphic characters\n");
    fputs_unlocked(v8, v7);
    v9 = stdout;
    v10 = gettext(
            "      --block-size=SIZE      with -l, scale sizes by SIZE when printing them;\n"
            "                             e.g., '--block-size=M'; see SIZE format below\n"
            "\n");
    fputs_unlocked(v10, v9);
    v11 = stdout;
    v12 = gettext("  -B, --ignore-backups       do not list implied entries ending with ~\n");
    fputs_unlocked(v12, v11);
    v13 = stdout;
    v14 = gettext(
            "  -c                         with -lt: sort by, and show, ctime (time of last\n"
            "                             modification of file status information);\n"
            "                             with -l: show ctime and sort by name;\n"
            "                             otherwise: sort by ctime, newest first\n"
            "\n");
    fputs_unlocked(v14, v13);
    v15 = stdout;
    v16 = gettext(
            "  -C                         list entries by columns\n"
            "      --color[=WHEN]         color the output WHEN; more info below\n"
            "  -d, --directory            list directories themselves, not their contents\n"
            "  -D, --dired                generate output designed for Emacs' dired mode\n");
    fputs_unlocked(v16, v15);
    v17 = stdout;
    v18 = gettext(
            "  -f                         list all entries in directory order\n"
            "  -F, --classify[=WHEN]      append indicator (one of */=>@|) to entries WHEN\n"
            "      --file-type            likewise, except do not append '*'\n");
    fputs_unlocked(v18, v17);
    v19 = stdout;
    v20 = gettext(
            "      --format=WORD          across -x, commas -m, horizontal -x, long -l,\n"
            "                             single-column -1, verbose -l, vertical -C\n"
            "\n");
    fputs_unlocked(v20, v19);
    v21 = stdout;
    v22 = gettext("      --full-time            like -l --time-style=full-iso\n");
    fputs_unlocked(v22, v21);
    v23 = stdout;
    v24 = gettext("  -g                         like -l, but do not list owner\n");
    fputs_unlocked(v24, v23);
    v25 = stdout;
    v26 = gettext(
            "      --group-directories-first\n"
            "                             group directories before files;\n"
            "                             can be augmented with a --sort option, but any\n"
            "                             use of --sort=none (-U) disables grouping\n"
            "\n");
    fputs_unlocked(v26, v25);
    v27 = stdout;
    v28 = gettext("  -G, --no-group             in a long listing, don't print group names\n");
    fputs_unlocked(v28, v27);
    v29 = stdout;
    v30 = gettext(
            "  -h, --human-readable       with -l and -s, print sizes like 1K 234M 2G etc.\n"
            "      --si                   likewise, but use powers of 1000 not 1024\n");
    fputs_unlocked(v30, v29);
    v31 = stdout;
    v32 = gettext(
            "  -H, --dereference-command-line\n"
            "                             follow symbolic links listed on the command line\n");
    fputs_unlocked(v32, v31);
    v33 = stdout;
    v34 = gettext(
            "      --dereference-command-line-symlink-to-dir\n"
            "                             follow each command line symbolic link\n"
            "                             that points to a directory\n"
            "\n");
    fputs_unlocked(v34, v33);
    v35 = stdout;
    v36 = gettext(
            "      --hide=PATTERN         do not list implied entries matching shell PATTERN\n"
            "                             (overridden by -a or -A)\n"
            "\n");
    fputs_unlocked(v36, v35);
    v37 = stdout;
    v38 = gettext("      --hyperlink[=WHEN]     hyperlink file names WHEN\n");
    fputs_unlocked(v38, v37);
    v39 = stdout;
    v40 = gettext(
            "      --indicator-style=WORD\n"
            "                             append indicator with style WORD to entry names:\n"
            "                             none (default), slash (-p),\n"
            "                             file-type (--file-type), classify (-F)\n"
            "\n");
    fputs_unlocked(v40, v39);
    v41 = stdout;
    v42 = gettext(
            "  -i, --inode                print the index number of each file\n"
            "  -I, --ignore=PATTERN       do not list implied entries matching shell PATTERN\n");
    fputs_unlocked(v42, v41);
    v43 = stdout;
    v44 = gettext(
            "  -k, --kibibytes            default to 1024-byte blocks for file system usage;\n"
            "                             used only with -s and per directory totals\n"
            "\n");
    fputs_unlocked(v44, v43);
    v45 = stdout;
    v46 = gettext("  -l                         use a long listing format\n");
    fputs_unlocked(v46, v45);
    v47 = stdout;
    v48 = gettext(
            "  -L, --dereference          when showing file information for a symbolic\n"
            "                             link, show information for the file the link\n"
            "                             references rather than for the link itself\n"
            "\n");
    fputs_unlocked(v48, v47);
    v49 = stdout;
    v50 = gettext("  -m                         fill width with a comma separated list of entries\n");
    fputs_unlocked(v50, v49);
    v51 = stdout;
    v52 = gettext(
            "  -n, --numeric-uid-gid      like -l, but list numeric user and group IDs\n"
            "  -N, --literal              print entry names without quoting\n"
            "  -o                         like -l, but do not list group information\n"
            "  -p, --indicator-style=slash\n"
            "                             append / indicator to directories\n");
    fputs_unlocked(v52, v51);
    v53 = stdout;
    v54 = gettext("  -q, --hide-control-chars   print ? instead of nongraphic characters\n");
    fputs_unlocked(v54, v53);
    v55 = stdout;
    v56 = gettext(
            "      --show-control-chars   show nongraphic characters as-is (the default,\n"
            "                             unless program is 'ls' and output is a terminal)\n"
            "\n");
    fputs_unlocked(v56, v55);
    v57 = stdout;
    v58 = gettext("  -Q, --quote-name           enclose entry names in double quotes\n");
    fputs_unlocked(v58, v57);
    v59 = stdout;
    v60 = gettext(
            "      --quoting-style=WORD   use quoting style WORD for entry names:\n"
            "                             literal, locale, shell, shell-always,\n"
            "                             shell-escape, shell-escape-always, c, escape\n"
            "                             (overrides QUOTING_STYLE environment variable)\n"
            "\n");
    fputs_unlocked(v60, v59);
    v61 = stdout;
    v62 = gettext(
            "  -r, --reverse              reverse order while sorting\n"
            "  -R, --recursive            list subdirectories recursively\n"
            "  -s, --size                 print the allocated size of each file, in blocks\n");
    fputs_unlocked(v62, v61);
    v63 = stdout;
    v64 = gettext("  -S                         sort by file size, largest first\n");
    fputs_unlocked(v64, v63);
    v65 = stdout;
    v66 = gettext(
            "      --sort=WORD            sort by WORD instead of name: none (-U), size (-S),\n"
            "                             time (-t), version (-v), extension (-X), width\n"
            "\n");
    fputs_unlocked(v66, v65);
    v67 = stdout;
    v68 = gettext(
            "      --time=WORD            change the default of using modification times;\n"
            "                               access time (-u): atime, access, use;\n"
            "                               change time (-c): ctime, status;\n"
            "                               birth time: birth, creation;\n"
            "                             with -l, WORD determines which time to show;\n"
            "                             with --sort=time, sort by WORD (newest first)\n"
            "\n");
    fputs_unlocked(v68, v67);
    v69 = stdout;
    v70 = gettext(
            "      --time-style=TIME_STYLE\n"
            "                             time/date format with -l; see TIME_STYLE below\n");
    fputs_unlocked(v70, v69);
    v71 = stdout;
    v72 = gettext(
            "  -t                         sort by time, newest first; see --time\n"
            "  -T, --tabsize=COLS         assume tab stops at each COLS instead of 8\n");
    fputs_unlocked(v72, v71);
    v73 = stdout;
    v74 = gettext(
            "  -u                         with -lt: sort by, and show, access time;\n"
            "                             with -l: show access time and sort by name;\n"
            "                             otherwise: sort by access time, newest first\n"
            "\n");
    fputs_unlocked(v74, v73);
    v75 = stdout;
    v76 = gettext("  -U                         do not sort; list entries in directory order\n");
    fputs_unlocked(v76, v75);
    v77 = stdout;
    v78 = gettext("  -v                         natural sort of (version) numbers within text\n");
    fputs_unlocked(v78, v77);
    v79 = stdout;
    v80 = gettext(
            "  -w, --width=COLS           set output width to COLS.  0 means no limit\n"
            "  -x                         list entries by lines instead of by columns\n"
            "  -X                         sort alphabetically by entry extension\n"
            "  -Z, --context              print any security context of each file\n"
            "      --zero                 end each output line with NUL, not newline\n"
            "  -1                         list one file per line\n");
    fputs_unlocked(v80, v79);
    v81 = stdout;
    v82 = gettext("      --help        display this help and exit\n");
    fputs_unlocked(v82, v81);
    v83 = stdout;
    v84 = gettext("      --version     output version information and exit\n");
    fputs_unlocked(v84, v83);
    sub_4F51();
    v85 = stdout;
    v86 = gettext(
            "\n"
            "The TIME_STYLE argument can be full-iso, long-iso, iso, locale, or +FORMAT.\n"
            "FORMAT is interpreted like in date(1).  If FORMAT is FORMAT1<newline>FORMAT2,\n"
            "then FORMAT1 applies to non-recent files and FORMAT2 to recent files.\n"
            "TIME_STYLE prefixed with 'posix-' takes effect only outside the POSIX locale.\n"
            "Also the TIME_STYLE environment variable sets the default style to use.\n");
    fputs_unlocked(v86, v85);
    v87 = stdout;
    v88 = gettext("\nThe WHEN argument defaults to 'always' and can also be 'auto' or 'never'.\n");
    fputs_unlocked(v88, v87);
    v89 = stdout;
    v90 = gettext(
            "\n"
            "Using color to distinguish file types is disabled both by default and\n"
            "with --color=never.  With --color=auto, ls emits color codes only when\n"
            "standard output is connected to a terminal.  The LS_COLORS environment\n"
            "variable can change the settings.  Use the dircolors(1) command to set it.\n");
    fputs_unlocked(v90, v89);
    v91 = stdout;
    v92 = gettext(
            "\n"
            "Exit status:\n"
            " 0  if OK,\n"
            " 1  if minor problems (e.g., cannot access subdirectory),\n"
            " 2  if serious trouble (e.g., cannot access command-line argument).\n");
    fputs_unlocked(v92, v91);
    if ( dword_2B210 == 1 )
    {
      v93 = "ls";
    }
    else if ( dword_2B210 == 2 )
    {
      v93 = "dir";
    }
    else
    {
      v93 = "vdir";
    }
    sub_4F82(v93);
  }
  exit(a1);
}



// Function: hash_get_n_buckets @ 0x11fd6
long long hash_get_n_buckets(long long a1)
{
  return *(long long *)(a1 + 16);
}



// Function: hash_get_n_buckets_used @ 0x11fec
long long hash_get_n_buckets_used(long long a1)
{
  return *(long long *)(a1 + 24);
}



// Function: hash_get_n_entries @ 0x12002
long long hash_get_n_entries(long long a1)
{
  return *(long long *)(a1 + 32);
}



// Function: hash_get_max_bucket_length @ 0x12018
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



// Function: hash_table_ok @ 0x1209a
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



// Function: hash_print_statistics @ 0x12138
int hash_print_statistics(long long a1, FILE *a2)
{
  double v2; // xmm0_8
  double v3; // xmm1_8
  double v4; // xmm0_8
  long long v6; // [rsp+10h] [rbp-20h]
  long long v7; // [rsp+18h] [rbp-18h]
  long long v8; // [rsp+20h] [rbp-10h]
  unsigned long long v9; // [rsp+28h] [rbp-8h]

  v6 = sub_12002(a1);
  v7 = sub_11FD6(a1);
  v8 = sub_11FEC(a1);
  v9 = sub_12018(a1);
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



// Function: safe_hasher @ 0x1227f
long long safe_hasher(long long a1, long long a2)
{
  unsigned long long v3; // [rsp+18h] [rbp-8h]

  v3 = (*(long long (**)(long long, long long))(a1 + 48))(a2, *(long long *)(a1 + 16));
  if ( v3 >= *(long long *)(a1 + 16) )
    abort();
  return *(long long *)a1 + 16 * v3;
}



// Function: hash_lookup @ 0x122da
long long hash_lookup(long long a1, long long a2)
{
  long long *i; // [rsp+10h] [rbp-10h]
  long long *v4; // [rsp+18h] [rbp-8h]

  v4 = (long long *)sub_1227F(a1, a2);
  if ( !*v4 )
    return 0;
  for ( i = v4; i; i = (long long *)i[1] )
  {
    if ( a2 == *i || (*(unsigned char (**)(long long, long long))(a1 + 56))(a2, *i) )
      return *i;
  }
  return 0;
}



// Function: hash_get_first @ 0x12371
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



// Function: hash_get_next @ 0x123d1
long long hash_get_next(long long a1, long long a2)
{
  unsigned long long v3; // [rsp+10h] [rbp-10h]
  unsigned long long v4; // [rsp+18h] [rbp-8h]

  v3 = sub_1227F(a1, a2);
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



// Function: hash_get_entries @ 0x1246f
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



// Function: hash_do_for_each @ 0x1250f
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



// Function: hash_string @ 0x125a7
unsigned long long hash_string(char *a1, unsigned long long a2)
{
  unsigned long long v4; // [rsp+18h] [rbp-8h]

  v4 = 0;
  while ( *a1 )
    v4 = (31 * v4 + (unsigned char)*a1++) % a2;
  return v4;
}



// Function: is_prime @ 0x12601
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



// Function: next_prime @ 0x12670
unsigned long long next_prime(unsigned long long a1)
{
  long long v2; // [rsp+0h] [rbp-8h]
  unsigned long long i; // [rsp+0h] [rbp-8h]

  v2 = a1;
  if ( a1 <= 9 )
    v2 = 10;
  for ( i = v2 | 1; i != -1 && !sub_12601(i); i += 2LL )
    ;
  return i;
}



// Function: hash_reset_tuning @ 0x126bb
long long hash_reset_tuning(long long a1)
{
  *(int *)a1 = 0;
  *(int *)(a1 + 4) = 1065353216;
  *(int *)(a1 + 8) = 1061997773;
  *(int *)(a1 + 12) = 1068826100;
  *(char *)(a1 + 16) = 0;
  return a1;
}



// Function: raw_hasher @ 0x12711
unsigned long long raw_hasher(long long a1, unsigned long long a2)
{
  return sub_1E664(a1, 3) % a2;
}



// Function: raw_comparator @ 0x1274c
bool raw_comparator(long long a1, long long a2)
{
  return a1 == a2;
}



// Function: check_tuning @ 0x12769
long long check_tuning(long long a1)
{
  long long v2; // [rsp+10h] [rbp-8h]

  v2 = *(long long *)(a1 + 40);
  if ( (_UNKNOWN *)v2 == &unk_23470 )
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
  *(long long *)(a1 + 40) = &unk_23470;
  return 0;
}



// Function: compute_bucket_size @ 0x12876
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
  v6 = sub_12670(v5);
  v4 = v6 >> 61 != 0;
  if ( (v6 & 0x1000000000000000LL) != 0 )
    v4 = 1;
  if ( v4 )
    return 0;
  else
    return v6;
}



// Function: hash_initialize @ 0x1298f
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
    v8 = sub_12711;
  if ( !a4 )
    v7 = sub_1274C;
  ptr = (size_t *)malloc(0x50u);
  if ( !ptr )
    return 0;
  if ( !a2 )
    v9 = &unk_23470;
  ptr[5] = (size_t)v9;
  if ( (unsigned char)sub_12769((long long)ptr) == 1
    && (ptr[2] = sub_12876(a1, (long long)v9)) != 0
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



// Function: hash_clear @ 0x12b04
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



// Function: hash_free @ 0x12c0f
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



// Function: allocate_entry @ 0x12d3e
void *allocate_entry(long long a1)
{
  long long v2; // [rsp+18h] [rbp-8h]

  if ( !*(long long *)(a1 + 72) )
    return malloc(0x10u);
  v2 = *(long long *)(a1 + 72);
  *(long long *)(a1 + 72) = *(long long *)(v2 + 8);
  return (void *)v2;
}



// Function: free_entry @ 0x12d8d
long long free_entry(long long a1, long long *a2)
{
  *a2 = 0;
  a2[1] = *(long long *)(a1 + 72);
  *(long long *)(a1 + 72) = a2;
  return a1;
}



// Function: hash_find_entry @ 0x12dc7
long long hash_find_entry(long long a1, long long a2, long long *a3, char a4)
{
  long long v5; // rdx
  long long *i; // [rsp+20h] [rbp-30h]
  long long *v9; // [rsp+28h] [rbp-28h]
  long long v10; // [rsp+30h] [rbp-20h]
  long long *v11; // [rsp+38h] [rbp-18h]
  long long v12; // [rsp+40h] [rbp-10h]
  long long *v13; // [rsp+48h] [rbp-8h]

  v9 = (long long *)sub_1227F(a1, a2);
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
        sub_12D8D(a1, v13);
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
      sub_12D8D(a1, v11);
    }
    return v10;
  }
}



// Function: transfer_entries @ 0x12f60
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
        v9 = (long long *)sub_1227F(a1, *j);
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
          sub_12D8D(a1, j);
        }
      }
      v8 = *i;
      i[1] = 0;
      if ( !a3 )
      {
        v10 = (long long *)sub_1227F(a1, v8);
        if ( *v10 )
        {
          v11 = sub_12D3E(a1);
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



// Function: hash_rehash @ 0x13134
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
  nmemb = sub_12876(a2, *(long long *)(a1 + 40));
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
  if ( (unsigned char)sub_12F60((long long)&v4, a1, 0) )
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
    if ( (unsigned char)sub_12F60(a1, (long long)&v4, 1) != 1 || (unsigned char)sub_12F60(a1, (long long)&v4, 0) != 1 )
      abort();
    free(v4);
    return 0;
  }
}



// Function: hash_insert_if_absent @ 0x1335f
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
  v26 = sub_12DC7((long long)a1, a2, &v25, 0);
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
      sub_12769((long long)a1);
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
        if ( (unsigned char)sub_13134((long long)a1, v23) != 1 )
          return 0xFFFFFFFFLL;
        if ( sub_12DC7((long long)a1, a2, &v25, 0) )
          abort();
      }
    }
    if ( *v25 )
    {
      v28 = sub_12D3E((long long)a1);
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



// Function: hash_insert @ 0x136d0
long long hash_insert(long long *a1, long long a2)
{
  int v3; // [rsp+1Ch] [rbp-14h]
  long long v4[2]; // [rsp+20h] [rbp-10h] BYREF

  v4[1] = __readfsqword(0x28u);
  v3 = sub_1335F(a1, a2, v4);
  if ( v3 == -1 )
    return 0;
  if ( v3 )
    return a2;
  return v4[0];
}



// Function: hash_remove @ 0x13740
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
  v26 = sub_12DC7(a1, a2, &v24, 1);
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
      sub_12769(a1);
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
        if ( (unsigned char)sub_13134(a1, v19) != 1 )
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



// Function: hash_delete @ 0x13a46
long long hash_delete(long long a1, long long a2)
{
  return sub_13740(a1, a2);
}


