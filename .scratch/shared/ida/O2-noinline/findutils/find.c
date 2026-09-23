// Function: main @ 0x7370
long long main(unsigned int a1, const char **a2, char **a3)
{
  const char *v4; // rdi
  unsigned int v5; // r12d
  long long v6; // r13
  char *v8; // rax
  char *v9; // r12
  int *v10; // rax
  char *v11; // r12
  int *v12; // rax

  v4 = *a2;
  if ( !*a2 )
    v4 = "find";
  sub_22CB0(v4, a2, a3);
  sub_D970();
  byte_3E618 = 0;
  status = 0;
  byte_3E608 = 0;
  dword_3E5F8 = -100;
  if ( (unsigned char)sub_15DD0() )
    sub_15C30();
  qword_3E610 = sub_E4D0("w");
  if ( !qword_3E610 )
  {
LABEL_15:
    v11 = dcgettext(0, "Failed to initialize shared-file hash table", 5);
    v12 = __errno_location();
    error(1, *v12, v11);
    start();
  }
  sub_DE90(&word_3E620);
  setlocale(6, "");
  bindtextdomain("findutils", "/usr/local/share/locale");
  textdomain("findutils");
  if ( (unsigned int)sub_2A3C0(sub_175A0) )
  {
    v9 = dcgettext(0, "The atexit library function failed", 5);
    v10 = __errno_location();
    error(1, *v10, v9);
    goto LABEL_15;
  }
  v5 = sub_DD00(a1, a2);
  if ( (qword_3E660 & 2) != 0 )
    qword_3E670 = (long long)sub_DB70;
  if ( (qword_3E660 & 0x80u) != 0LL )
  {
    v8 = ctime(&timer);
    __fprintf_chk(stderr, 1, "cur_day_start = %s", v8);
  }
  v6 = sub_C6F0(a1, a2, v5);
  if ( (unsigned char)sub_8210(a1 - v5, &a2[v5]) )
  {
    sub_9EE0(v6);
    sub_E360();
  }
  return (unsigned int)status;
}



// Function: symlink_loop @ 0x7640
bool symlink_loop(long long a1)
{
  int v1; // r8d
  bool result; // al
  long long v3[21]; // [rsp+0h] [rbp-A8h] BYREF

  v3[19] = __readfsqword(0x28u);
  v1 = qword_3E670(a1, v3);
  result = 0;
  if ( v1 )
    return *__errno_location() == 40;
  return result;
}



// Function: inside_dir @ 0x76a0
void inside_dir(int a1)
{
  if ( (byte_3E019 & 2) != 0 )
  {
    if ( a1 != -100 && a1 < 0 )
      __assert_fail("dir_fd == AT_FDCWD || dir_fd >= 0", "ftsfind.c", 0x6Bu, "inside_dir");
    dword_3E5F8 = a1;
    if ( fd < 0 )
    {
      if ( a1 == -100 )
        fd = -100;
      else
        fd = sub_17560();
    }
  }
}



// Function: left_dir @ 0x7720
int left_dir()
{
  int result; // eax

  if ( (byte_3E019 & 2) != 0 && fd >= 0 )
  {
    result = close(fd);
    fd = -1;
  }
  return result;
}



// Function: get_fts_info_name @ 0x7750
const char *get_fts_info_name(int a1)
{
  const char *result; // rax

  switch ( a1 )
  {
    case 1:
      result = "FTS_D";
      break;
    case 2:
      result = "FTS_DC";
      break;
    case 3:
      result = "FTS_DEFAULT";
      break;
    case 4:
      result = "FTS_DNR";
      break;
    case 5:
      result = "FTS_DOT";
      break;
    case 6:
      result = "FTS_DP";
      break;
    case 7:
      result = "FTS_ERR";
      break;
    case 8:
      result = "FTS_F";
      break;
    case 9:
      result = "FTS_INIT";
      break;
    case 10:
      result = "FTS_NS";
      break;
    case 11:
      result = "FTS_NSOK";
      break;
    case 12:
      result = "FTS_SL";
      break;
    case 13:
      result = "FTS_SLNONE";
      break;
    case 14:
      result = "FTS_W";
      break;
    default:
      __sprintf_chk(&unk_3E598, 1, 14, "[%d]", a1);
      result = (const char *)&unk_3E598;
      break;
  }
  return result;
}



// Function: visit @ 0x7880
long long visit(long long a1, long long a2, long long a3)
{
  char *v4; // rax
  long long v5; // rax
  long long result; // rax

  v4 = *(char **)(a2 + 48);
  LOBYTE(word_3E5E4) = (unsigned short)(*(short *)(a2 + 104) - 10) > 1u;
  src = v4;
  dword_3E5F8 = *(int *)(a1 + 44);
  v5 = sub_B5E0();
  result = sub_E2A0(*(long long *)(a2 + 56), a3, v5);
  if ( byte_3E600 )
    return sub_1A190(a1, a2, 4);
  return result;
}



// Function: issue_loop_warning @ 0x7950
void issue_loop_warning(long long a1)
{
  long long v1; // rbp
  long long v2; // r12
  char *v3; // rax
  long long v4; // r12
  char *v5; // rax

  if ( (*(int *)(a1 + 136) & 0xF000) == 0xA000 )
  {
    v4 = sub_DFB0(0, *(long long *)(a1 + 56));
    v5 = dcgettext(
           0,
           "Symbolic link %s is part of a loop in the directory hierarchy; we have already visited the directory to which it points.",
           5);
    error(0, 0, v5, v4);
  }
  else
  {
    v1 = sub_7900(*(long long *)(*(long long *)a1 + 56LL), *(long long *)(*(long long *)a1 + 72LL), (unsigned int)dword_3E68C);
    v2 = sub_DFB0(0, *(long long *)(a1 + 56));
    v3 = dcgettext(0, "File system loop detected; %s is part of the same file system loop as %s.", 5);
    error(0, 0, v3, v2, v1);
  }
}



// Function: consider_visiting @ 0x7a00
unsigned long long consider_visiting(long long a1, const __m128i *a2)
{
  long long v3; // rax
  long long v4; // rdx
  short v5; // ax
  __m128i v6; // xmm0
  __m128i v7; // xmm1
  __m128i v8; // xmm4
  __m128i v9; // xmm5
  __m128i v10; // xmm6
  __m128i v11; // xmm7
  __m128i v12; // xmm0
  unsigned short v13; // ax
  int v14; // edi
  long long v15; // rdx
  int v16; // r13d
  long long v17; // rbx
  char v18; // al
  long long v19; // rax
  const char *v21; // r15
  long long v22; // rax
  int v23; // r13d
  const char *v24; // r14
  const char *v25; // rbx
  const char *v26; // rax
  int v27; // edx
  int v28; // r15d
  const char *v29; // r14
  const char *v30; // rax
  long long v31; // r13
  char *v32; // rax
  long long v33; // [rsp+8h] [rbp-F0h]
  int v34; // [rsp+8h] [rbp-F0h]
  int v35; // [rsp+1Ch] [rbp-DCh] BYREF
  _OWORD v36[9]; // [rsp+20h] [rbp-D8h] BYREF
  unsigned long long v37; // [rsp+B8h] [rbp-40h]

  v37 = __readfsqword(0x28u);
  if ( (qword_3E660 & 4) != 0 )
  {
    v21 = (const char *)sub_24E50(2, (unsigned int)dword_3E68C, a2[3].m128i_i64[0]);
    v22 = sub_24E50(1, (unsigned int)dword_3E68C, a2[3].m128i_i64[1]);
    v23 = dword_3E014;
    v24 = (const char *)v22;
    v33 = a2[5].m128i_i64[1];
    v25 = sub_7750(a2[6].m128i_u16[4]);
    v26 = (const char *)sub_24E50(0, (unsigned int)dword_3E68C, a2[3].m128i_i64[1]);
    __fprintf_chk(
      stderr,
      1,
      "consider_visiting (early): %s: fts_info=%-6s, fts_level=%2d, prev_depth=%d fts_path=%s, fts_accpath=%s\n",
      v26,
      v25,
      v33,
      v23,
      v24,
      v21);
  }
  if ( a2[6].m128i_i16[4] == 6 || (v3 = a2[5].m128i_i64[1], v3 > dword_3E014) || !v3 )
    sub_7720();
  sub_76A0(*(int *)(a1 + 44));
  v4 = a2[5].m128i_i64[1];
  *((long long *)&v36[0] + 1) = a2[7].m128i_i64[1];
  v5 = a2[6].m128i_i16[4];
  dword_3E014 = v4;
  switch ( v5 )
  {
    case 7:
      goto LABEL_51;
    case 4:
      sub_E060(a2[4].m128i_i32[0]);
      if ( (char)word_3E620 )
        return v37 - __readfsqword(0x28u);
      goto LABEL_47;
    case 2:
      sub_7950((long long)a2);
      status = 1;
      return v37 - __readfsqword(0x28u);
  }
  if ( v5 != 13 )
  {
    if ( v5 != 10 )
      goto LABEL_11;
    if ( v4 )
    {
      if ( !sub_7640(a2[3].m128i_i64[0]) )
      {
        sub_E060(a2[4].m128i_i32[0]);
        goto LABEL_47;
      }
LABEL_62:
      sub_E060(40);
      return v37 - __readfsqword(0x28u);
    }
LABEL_51:
    sub_E060(a2[4].m128i_i32[0]);
    return v37 - __readfsqword(0x28u);
  }
  if ( sub_7640(a2[3].m128i_i64[0]) )
    goto LABEL_62;
LABEL_47:
  v5 = a2[6].m128i_i16[4];
LABEL_11:
  if ( (unsigned short)(v5 - 10) <= 1u )
  {
    if ( (char)word_3E5E4 )
      __assert_fail("!state.have_stat", "ftsfind.c", 0x17Cu, "consider_visiting");
    v27 = dword_3E5E8;
    if ( v5 != 11 && dword_3E5E8 )
      __assert_fail("ent->fts_info == FTS_NSOK || state.type == 0", "ftsfind.c", 0x17Du, "consider_visiting");
    v35 = dword_3E5E8;
  }
  else
  {
    v6 = _mm_loadu_si128(a2 + 7);
    v7 = _mm_loadu_si128(a2 + 8);
    word_3E5E4 = 257;
    v8 = _mm_loadu_si128(a2 + 11);
    v36[0] = v6;
    v9 = _mm_loadu_si128(a2 + 12);
    v10 = _mm_loadu_si128(a2 + 13);
    v36[1] = v7;
    v11 = _mm_loadu_si128(a2 + 14);
    v36[2] = _mm_loadu_si128(a2 + 9);
    v12 = _mm_loadu_si128(a2 + 15);
    v36[3] = _mm_loadu_si128(a2 + 10);
    v35 = v7.m128i_i32[2];
    dword_3E5E8 = v7.m128i_i32[2];
    v36[4] = v8;
    v36[5] = v9;
    v36[6] = v10;
    v36[7] = v11;
    v36[8] = v12;
    if ( v7.m128i_i32[2] )
    {
      dword_3E5E0 = a2[5].m128i_i64[1];
      goto LABEL_14;
    }
    v31 = sub_24E50(0, (unsigned int)dword_3E68C, a2[3].m128i_i64[1]);
    v32 = dcgettext(0, "WARNING: file %s appears to have mode 0000", 5);
    error(0, 0, v32, v31);
    v27 = 0;
  }
  dword_3E5E0 = a2[5].m128i_i64[1];
  if ( v27 )
  {
LABEL_14:
    if ( !(unsigned char)sub_E150(&v35, a2[3].m128i_i64[1], &a2[16], v36, 0) )
      return v37 - __readfsqword(0x28u);
    v13 = a2[6].m128i_u16[4];
    v14 = v13;
    if ( (v35 & 0xF000) == 0x4000 )
      goto LABEL_16;
    goto LABEL_36;
  }
  v13 = a2[6].m128i_u16[4];
LABEL_36:
  v14 = v13;
  if ( (unsigned short)(v13 - 1) > 1u && v13 != 6 )
  {
    v15 = dword_3E624;
    v16 = 0;
    if ( dword_3E624 < 0 )
    {
      v18 = qword_3E660;
      LODWORD(v17) = 0;
      goto LABEL_22;
    }
    goto LABEL_18;
  }
LABEL_16:
  if ( v13 == 11 )
  {
    sub_1A190(a1, a2, 1);
    return v37 - __readfsqword(0x28u);
  }
  v15 = dword_3E624;
  v16 = 1;
  LODWORD(v17) = 0;
  if ( dword_3E624 >= 0 )
  {
LABEL_18:
    if ( a2[5].m128i_i64[1] >= v15 )
    {
      sub_1A190(a1, a2, 4);
      v14 = a2[6].m128i_u16[4];
      v17 = a2[5].m128i_i64[1] > dword_3E624;
    }
    else
    {
      v14 = v13;
      LODWORD(v17) = 0;
    }
  }
  v18 = qword_3E660;
  if ( (short)v14 == 1 )
  {
    if ( !(char)word_3E620 )
    {
      if ( (qword_3E660 & 4) == 0 )
        return v37 - __readfsqword(0x28u);
      goto LABEL_41;
    }
  }
  else if ( (short)v14 == 6 && (char)word_3E620 )
  {
    if ( (qword_3E660 & 4) == 0 )
      goto LABEL_27;
    goto LABEL_41;
  }
LABEL_22:
  v19 = v18 & 4;
  if ( a2[5].m128i_i64[1] >= dword_3E628 )
  {
    if ( !v19 )
      goto LABEL_24;
    goto LABEL_42;
  }
  if ( !v19 )
    goto LABEL_26;
LABEL_41:
  LODWORD(v17) = 1;
LABEL_42:
  v28 = (unsigned char)word_3E5E4;
  v34 = HIBYTE(word_3E5E4);
  v29 = sub_7750(v14);
  v30 = (const char *)sub_24E50(0, (unsigned int)dword_3E68C, a2[3].m128i_i64[1]);
  __fprintf_chk(
    stderr,
    1,
    "consider_visiting (late): %s: fts_info=%-6s, isdir=%d ignore=%d have_stat=%d have_type=%d \n",
    v30,
    v29,
    v16,
    v17,
    v28,
    v34);
LABEL_24:
  if ( !v17 )
    sub_7880(a1, (long long)a2, (long long)v36);
LABEL_26:
  if ( a2[6].m128i_i16[4] == 6 )
LABEL_27:
    byte_3E600 = 0;
  return v37 - __readfsqword(0x28u);
}



// Function: find @ 0x7fe0
long long find(const char *a1)
{
  long long v1; // rsi
  unsigned int v2; // r13d
  void *v3; // rbp
  int *v4; // rbx
  long long v5; // rax
  unsigned int v6; // r14d
  int v7; // eax
  long long v8; // rax
  const __m128i *v9; // rsi
  int v10; // r8d
  long long v11; // rax
  char *v12; // rax
  long long v13; // r12
  char *v14; // rax
  long long result; // rax
  long long v16[9]; // [rsp+10h] [rbp-48h] BYREF

  v16[3] = __readfsqword(0x28u);
  dword_3E5FC = strlen(a1);
  sub_76A0(-100);
  v16[0] = a1;
  v16[1] = 0;
  v1 = unk_3E018;
  switch ( dword_3E668 )
  {
    case 1:
      v1 = unk_3E018 | 3u;
      unk_3E018 |= 3u;
      break;
    case 2:
      v1 = unk_3E018 | 0x11u;
      unk_3E018 |= 0x11u;
      break;
    case 0:
      v1 = unk_3E018 | 0x10u;
      unk_3E018 |= 0x10u;
      break;
  }
  if ( byte_3E62D )
  {
    v1 = (unsigned int)v1 | 0x40;
    unk_3E018 = v1;
  }
  v2 = 0x80000000;
  v3 = (void *)sub_195E0(v16, v1, 0);
  v4 = __errno_location();
  if ( !v3 )
  {
    v13 = sub_DFB0(0, a1);
    v14 = dcgettext(0, "cannot search %s", 5);
    error(0, *v4, v14, v13);
    status = 1;
    return 1;
  }
  while ( 1 )
  {
    *v4 = 0;
    v8 = sub_19B40(v3);
    v9 = (const __m128i *)v8;
    if ( !v8 )
      break;
    v5 = *(long long *)(v8 + 88);
    v6 = v9[5].m128i_i64[1];
    if ( byte_3E608 )
    {
      if ( v2 != (int)v5 )
      {
        sub_D940();
        v6 = v9[5].m128i_u32[2];
      }
    }
    v7 = v9[8].m128i_i32[2];
    v2 = v6;
    byte_3E618 = 0;
    LOBYTE(word_3E5E4) = 0;
    dword_3E5E8 = v7;
    HIBYTE(word_3E5E4) = v7 != 0;
    sub_7A00((long long)v3, v9);
  }
  if ( *v4 )
  {
    v11 = sub_DFB0(0, a1);
    error(0, *v4, "failed to read file names from file system at or below %s", v11);
LABEL_18:
    status = 1;
    return 0;
  }
  v10 = sub_19950(v3);
  result = 1;
  if ( v10 )
  {
    v12 = dcgettext(0, "failed to restore working directory after searching %s", 5);
    error(0, *v4, v12, a1);
    goto LABEL_18;
  }
  return result;
}



// Function: process_all_startpoints @ 0x8210
long long process_all_startpoints(int a1, long long *a2)
{
  char *v2; // rbp
  char *v3; // rsi
  const char *v4; // r13
  void *v5; // rbp
  unsigned int v6; // r12d
  const char *v7; // r14
  int v9; // eax
  unsigned int v10; // ebp
  long long v11; // r12
  char *v12; // rax
  const char *v13; // rax
  char *v14; // r12
  int *v15; // rax
  const char *v16; // rdx
  char *v17; // rax
  char *v18; // rax
  char *v19; // r12
  int *v20; // rax
  char *v21; // rax
  long long v22; // r12
  char *v23; // rax
  char *v24; // rax
  FILE *stream; // [rsp+8h] [rbp-180h]
  int v26; // [rsp+1Ch] [rbp-16Ch] BYREF
  struct stat buf; // [rsp+20h] [rbp-168h] BYREF
  struct stat v28; // [rsp+B0h] [rbp-D8h] BYREF
  short v29; // [rsp+146h] [rbp-42h] BYREF
  unsigned long long v30; // [rsp+148h] [rbp-40h]

  v30 = __readfsqword(0x28u);
  if ( a1 <= 0 || (unsigned char)sub_DCA0(*a2, 1) )
  {
    v2 = filename;
    if ( !filename )
    {
      v29 = 46;
      return (unsigned int)sub_7FE0((const char *)&v29);
    }
    if ( !strcmp(filename, "-") )
    {
      if ( byte_3E698 )
      {
        v21 = dcgettext(0, "option -files0-from reading from standard input cannot be combined with -ok, -okdir", 5);
        error(1, 0, "%s\n", v21);
        goto LABEL_47;
      }
      v3 = dcgettext(0, "(standard input)", 5);
      v4 = (const char *)sub_DFB0(0, v3);
      stream = stdin;
      goto LABEL_6;
    }
    v4 = (const char *)sub_DFB0(0, v2);
    stream = fopen(filename, "r");
    if ( stream )
    {
      v9 = fileno(stream);
      v10 = v9;
      if ( v9 >= 0 )
      {
        if ( byte_3E698 && !fstat(v9, &buf) && !fstat(0, &v28) && buf.st_ino == v28.st_ino && buf.st_dev == v28.st_dev )
        {
          v17 = dcgettext(
                  0,
                  "option -files0-from: standard input must not refer to the same file when combined with -ok, -okdir",
                  5);
          error(1, 0, "%s: %s\n", v17, v4);
          goto LABEL_43;
        }
        sub_174F0(v10, 1);
LABEL_6:
        v5 = (void *)sub_171E0(stream);
LABEL_7:
        if ( v5 )
        {
          v6 = 1;
          while ( 1 )
          {
            v7 = (const char *)sub_17220(v5, &v26);
            if ( !v7 )
              break;
            if ( *v7 )
            {
              if ( !filename && (unsigned char)sub_DCA0(v7, 1) )
                goto LABEL_14;
              dword_3E5FC = strlen(v7);
              if ( !(unsigned char)sub_7FE0(v7) )
              {
                v6 = 0;
                goto LABEL_14;
              }
            }
            else
            {
              if ( filename )
              {
                v11 = sub_172C0(v5);
                v12 = dcgettext(0, "invalid zero-length file name", 5);
                error(0, 0, "%s:%lu: %s", v4, v11, v12);
              }
              else
              {
                v13 = (const char *)sub_DFB0(0, v7);
                error(0, 2, "%s", v13);
              }
              status = 1;
              v6 = 0;
            }
          }
          if ( v26 != 3 )
          {
            if ( v26 == 4 )
            {
              v14 = dcgettext(0, "%s: read error", 5);
              v15 = __errno_location();
              v16 = v14;
              v6 = 0;
              error(0, *v15, v16, v4);
              status = 1;
              sub_172E0(v5);
              return v6;
            }
            if ( v26 != 2 )
              __assert_fail("!\"unexpected error code from argv_iter\"", "ftsfind.c", 0x2A3u, "process_all_startpoints");
LABEL_14:
            sub_172E0(v5);
            if ( !(char)v6 || !filename || !ferror(stream) && !(unsigned int)sub_17850(stream) )
              return v6;
LABEL_43:
            v18 = dcgettext(0, "error reading %s", 5);
            error(1, 0, v18, v4);
          }
        }
LABEL_47:
        sub_26D50();
      }
    }
    else
    {
      v19 = dcgettext(0, "cannot open %s for reading", 5);
      v20 = __errno_location();
      error(1, *v20, v19, v4);
    }
    __assert_fail("fd >= 0", "ftsfind.c", 0x262u, "process_all_startpoints");
  }
  stream = (FILE *)filename;
  if ( !filename )
  {
    v4 = 0;
    v5 = (void *)sub_171B0(a2);
    goto LABEL_7;
  }
  v22 = sub_DFB0(0, *a2);
  v23 = dcgettext(0, "extra operand %s", 5);
  error(0, 0, v23, v22);
  v24 = dcgettext(0, "file operands cannot be combined with -files0-from", 5);
  error(1, 0, "%s", v24);
  return sub_86D0();
}



// Function: is_fts_enabled @ 0x86d0
long long is_fts_enabled(int *a1)
{
  *a1 = unk_3E018;
  return 1;
}



// Function: pred_prune @ 0x86f0
long long pred_prune(long long a1, long long a2)
{
  if ( !(char)word_3E620 )
    return 1;
  if ( !(char)word_3E5E4 )
    __assert_fail("state.have_stat", "pred.c", 0x355u, "pred_prune");
  if ( !a2 || (*(int *)(a2 + 24) & 0xF000) != 0x4000 )
    return 1;
  byte_3E600 = 1;
  return 1;
}



// Function: ts_difference @ 0x8750
double ts_difference(time_t a1, int a2, time_t a3, int a4)
{
  return (double)(a2 - a4) * 0.000000001 + difftime(a1, a3);
}



// Function: compare_ts @ 0x8790
long long compare_ts(time_t a1, long long a2, time_t a3, long long a4)
{
  long long result; // rax
  double v5; // xmm0_8

  if ( a1 != a3 || (result = 0, a2 != a4) )
  {
    v5 = sub_8750(a1, a2, a3, a4);
    result = 0xFFFFFFFFLL;
    if ( v5 >= 0.0 )
      return 1;
  }
  return result;
}



// Function: pred_timewindow @ 0x87d0
char pred_timewindow(time_t a1, long long a2, long long a3, int a4)
{
  int v4; // eax
  int v5; // eax
  double v7; // xmm0_8

  v4 = *(int *)(a3 + 60);
  if ( v4 == 1 )
  {
    return (unsigned int)sub_8790(a1, a2, *(long long *)(a3 + 64), *(long long *)(a3 + 72)) >> 31;
  }
  else if ( v4 == 2 )
  {
    v7 = sub_8750(a1, a2, *(long long *)(a3 + 64), *(long long *)(a3 + 72));
    LOBYTE(v5) = 0;
    if ( v7 > 0.0 )
      LOBYTE(v5) = (double)a4 >= v7;
  }
  else
  {
    if ( v4 )
      __assert_fail("0", "pred.c", 0x8Au, "pred_timewindow");
    LOBYTE(v5) = (int)sub_8790(a1, a2, *(long long *)(a3 + 64), *(long long *)(a3 + 72)) > 0;
  }
  return v5;
}



// Function: pred_and @ 0x8870
long long pred_and(long long a1, long long a2, long long a3)
{
  long long v4; // rdx
  long long result; // rax

  v4 = *(long long *)(a3 + 272);
  if ( !v4 )
    return sub_E2A0(a1, a2, *(long long *)(a3 + 280));
  result = sub_E2A0(a1, a2, v4);
  if ( (char)result )
    return sub_E2A0(a1, a2, *(long long *)(a3 + 280));
  return result;
}



// Function: pred_or @ 0x88c0
long long pred_or(long long a1, long long a2, long long a3)
{
  long long v4; // rdx
  long long result; // rax

  v4 = *(long long *)(a3 + 272);
  if ( !v4 )
    return sub_E2A0(a1, a2, *(long long *)(a3 + 280));
  result = sub_E2A0(a1, a2, v4);
  if ( !(char)result )
    return sub_E2A0(a1, a2, *(long long *)(a3 + 280));
  return result;
}



// Function: perform_delete @ 0x8910
bool perform_delete(int flag)
{
  return unlinkat(dword_3E5F8, src, flag) == 0;
}



// Function: pred_name_common @ 0x8940
bool pred_name_common(long long a1, const char *a2, int a3)
{
  char *v4; // rbp
  int v5; // ebx

  v4 = (char *)sub_176F0();
  sub_17810(v4);
  v5 = fnmatch(a2, v4, a3);
  free(v4);
  return v5 == 0;
}



// Function: is_ok @ 0x8990
long long is_ok(long long a1, long long a2)
{
  char *v2; // rax
  char *v4; // r12
  int *v5; // rax

  sub_17BC0(stdout);
  v2 = dcgettext(0, "< %s ... %s > ? ", 5);
  if ( (int)__fprintf_chk(stderr, 1, v2, a1, a2) < 0 )
  {
    v4 = dcgettext(0, "Failed to write prompt for -ok", 5);
    v5 = __errno_location();
    error(1, *v5, v4);
    sub_E360();
    exit(status);
  }
  sub_17BC0(stderr);
  return sub_276E0();
}



// Function: blank_rtrim @ 0x8a40
char *blank_rtrim(char *src, char *s)
{
  int v2; // eax
  int v3; // ecx
  char *v4; // rdx
  char *v5; // r8

  if ( !src )
    return 0;
  strcpy(s, src);
  v2 = strlen(s);
  v3 = v2 - 1;
  if ( v2 - 1 < 0 )
  {
    v5 = &s[v2];
  }
  else
  {
    v4 = &s[v3];
    while ( 1 )
    {
      v5 = v4;
      if ( *v4 != 32 && *v4 != 9 )
        break;
      --v3;
      --v4;
      if ( v3 == -1 )
        goto LABEL_8;
    }
    v5 = &s[v3 + 1];
  }
LABEL_8:
  *v5 = 0;
  return s;
}



// Function: pred_amin @ 0x8b70
char pred_amin(long long a1, long long a2, long long a3)
{
  time_t v4; // rax
  long long v5; // rdx

  v4 = sub_25C40(a2);
  return sub_87D0(v4, v5, a3, 60);
}



// Function: pred_anewer @ 0x8ba0
bool pred_anewer(long long a1, long long a2, long long a3)
{
  time_t v4; // rax
  long long v5; // rdx

  if ( *(int *)(a3 + 60) )
    __assert_fail("COMP_GT == pred_ptr->args.reftime.kind", "pred.c", 0xA6u, "pred_anewer");
  v4 = sub_25C40(a2);
  return (int)sub_8790(v4, v5, *(long long *)(a3 + 64), *(long long *)(a3 + 72)) > 0;
}



// Function: pred_atime @ 0x8c00
char pred_atime(long long a1, long long a2, long long a3)
{
  time_t v4; // rax
  long long v5; // rdx

  v4 = sub_25C40(a2);
  return sub_87D0(v4, v5, a3, 86400);
}



// Function: pred_cmin @ 0x8c40
char pred_cmin(long long a1, long long a2, long long a3)
{
  time_t v4; // rax
  long long v5; // rdx

  v4 = sub_25C50(a2);
  return sub_87D0(v4, v5, a3, 60);
}



// Function: pred_cnewer @ 0x8c70
bool pred_cnewer(long long a1, long long a2, long long a3)
{
  time_t v4; // rax
  long long v5; // rdx

  if ( *(int *)(a3 + 60) )
    __assert_fail("COMP_GT == pred_ptr->args.reftime.kind", "pred.c", 0xC7u, "pred_cnewer");
  v4 = sub_25C50(a2);
  return (int)sub_8790(v4, v5, *(long long *)(a3 + 64), *(long long *)(a3 + 72)) > 0;
}



// Function: pred_comma @ 0x8cd0
long long pred_comma(long long a1, long long a2, long long a3)
{
  long long v4; // rdx

  v4 = *(long long *)(a3 + 272);
  if ( v4 )
    sub_E2A0(a1, a2, v4);
  return sub_E2A0(a1, a2, *(long long *)(a3 + 280));
}



// Function: pred_ctime @ 0x8d10
char pred_ctime(long long a1, long long a2, long long a3)
{
  time_t v4; // rax
  long long v5; // rdx

  v4 = sub_25C50(a2);
  return sub_87D0(v4, v5, a3, 86400);
}



// Function: pred_delete @ 0x8d40
long long pred_delete(long long a1, long long a2)
{
  int v2; // ebx
  unsigned int v3; // eax
  unsigned int v4; // r12d
  int *v5; // r13
  long long v8; // r14
  char *v9; // rax
  unsigned int v10; // eax

  if ( !strcmp(src, ".") )
    return 1;
  v2 = 0;
  if ( (char)word_3E5E4 )
    v2 = ((*(int *)(a2 + 24) & 0xF000) == 0x4000) << 9;
  LOBYTE(v3) = sub_8910(v2);
  v4 = v3;
  if ( (char)v3 )
    return 1;
  v5 = __errno_location();
  if ( *v5 == 2 )
  {
    v10 = (unsigned char)byte_3E62E;
    if ( byte_3E62E )
    {
      *v5 = 0;
      return v10;
    }
  }
  else if ( *v5 == 21 && !v2 && sub_8910(512) )
  {
    return 1;
  }
  v8 = sub_DFB0(0, a1);
  v9 = dcgettext(0, "cannot delete %s", 5);
  error(0, *v5, v9, v8);
  status = 1;
  return v4;
}



// Function: pred_empty @ 0x8e50
long long pred_empty(long long a1, long long a2)
{
  int v2; // eax
  unsigned int v3; // r12d
  int *v5; // rax
  int v6; // esi
  int *v7; // rbx
  int v8; // ecx
  int v9; // r8d
  int v10; // r9d
  int v11; // eax
  int v12; // r12d
  DIR *v13; // r13
  struct dirent *v14; // rax
  const char *v15; // rax
  const char *v16; // rax
  const char *v17; // rax
  int v18; // edi

  v2 = *(int *)(a2 + 24) & 0xF000;
  if ( v2 == 0x4000 )
  {
    v5 = __errno_location();
    v6 = (int)src;
    *v5 = 0;
    v7 = v5;
    v11 = sub_1E690(dword_3E5F8, v6, 592128, v8, v9, v10);
    v12 = v11;
    if ( v11 < 0 )
      goto LABEL_15;
    v13 = fdopendir(v11);
    if ( !v13 )
    {
      v17 = (const char *)sub_DFB0(0, a1);
      error(0, *v7, "%s", v17);
      v18 = v12;
      v3 = 0;
      status = 1;
      close(v18);
      return v3;
    }
    *v7 = 0;
    while ( 1 )
    {
      v14 = readdir(v13);
      if ( !v14 )
        break;
      if ( v14->d_name[0] != 46 || v14->d_name[1] && *(short *)&v14->d_name[1] != 46 )
      {
        v3 = 0;
        goto LABEL_13;
      }
    }
    v3 = 1;
LABEL_13:
    if ( *v7 )
    {
      v3 = 0;
      v16 = (const char *)sub_DFB0(0, a1);
      error(0, *v7, "%s", v16);
      status = 1;
      closedir(v13);
      return v3;
    }
    if ( closedir(v13) )
    {
LABEL_15:
      v3 = 0;
      v15 = (const char *)sub_DFB0(0, a1);
      error(0, *v7, "%s", v15);
      status = 1;
    }
  }
  else
  {
    v3 = 0;
    if ( v2 == 0x8000 )
      LOBYTE(v3) = *(long long *)(a2 + 48) == 0;
  }
  return v3;
}



// Function: pred_execdir @ 0x8ff0
long long pred_execdir()
{
  return sub_A390(src);
}



// Function: pred_false @ 0x9000
long long pred_false()
{
  return 0;
}



// Function: pred_fls @ 0x9010
long long pred_fls(int a1, int a2, long long a3)
{
  sub_16170(a1, dword_3E5F8, (int)src, a2, qword_3E638, dword_3E65C, *(unsigned char *)(a3 + 40), *(FILE **)(a3 + 64));
  return 1;
}



// Function: pred_fprint @ 0x9050
long long pred_fprint(const char *a1, long long a2, long long a3)
{
  sub_16960(*(long long *)(a3 + 64), *(long long *)(a3 + 88), *(unsigned char *)(a3 + 80), "%s\n", a1);
  return 1;
}



// Function: pred_fprint0 @ 0x9090
long long pred_fprint0(const char *a1, long long a2, long long a3)
{
  FILE *v3; // rbp

  v3 = *(FILE **)(a3 + 64);
  fputs(a1, v3);
  putc(0, v3);
  return 1;
}



// Function: pred_fstype @ 0x90c0
bool pred_fstype(long long a1, long long a2, long long a3)
{
  const char *v3; // rbp
  const char *v4; // rax

  v3 = *(const char **)(a3 + 56);
  v4 = (const char *)sub_104C0(a2, a1);
  return strcmp(v4, v3) == 0;
}



// Function: pred_gid @ 0x90f0
long long pred_gid(long long a1, long long a2, long long a3, long long a4, unsigned int a5)
{
  int v5; // eax
  unsigned int v6; // r8d

  v5 = *(int *)(a3 + 56);
  if ( v5 == 1 )
  {
    LOBYTE(a5) = (unsigned long long)*(unsigned int *)(a2 + 32) < *(long long *)(a3 + 64);
    return a5;
  }
  else if ( v5 == 2 )
  {
    LOBYTE(a5) = *(unsigned int *)(a2 + 32) == *(long long *)(a3 + 64);
    return a5;
  }
  else
  {
    v6 = 0;
    if ( !v5 )
      LOBYTE(v6) = (unsigned long long)*(unsigned int *)(a2 + 32) > *(long long *)(a3 + 64);
    return v6;
  }
}



// Function: pred_group @ 0x9140
bool pred_group(long long a1, long long a2, long long a3)
{
  return *(int *)(a3 + 56) == *(int *)(a2 + 32);
}



// Function: pred_ilname @ 0x9150
long long pred_ilname(long long a1, long long a2, long long a3)
{
  return sub_8AD0(a1, *(unsigned int *)(a2 + 24), a3, 1);
}



// Function: pred_iname @ 0x9170
bool pred_iname(long long a1, long long a2, long long a3)
{
  return sub_8940(a1, *(const char **)(a3 + 56), 16);
}



// Function: pred_inum @ 0x9190
long long pred_inum(long long a1, long long a2, long long a3, long long a4, unsigned int a5)
{
  int v5; // eax
  unsigned int v6; // r8d

  v5 = *(int *)(a3 + 56);
  if ( v5 == 1 )
  {
    LOBYTE(a5) = *(long long *)(a2 + 8) < *(long long *)(a3 + 64);
    return a5;
  }
  else if ( v5 == 2 )
  {
    LOBYTE(a5) = *(long long *)(a2 + 8) == *(long long *)(a3 + 64);
    return a5;
  }
  else
  {
    v6 = 0;
    if ( !v5 )
      LOBYTE(v6) = *(long long *)(a2 + 8) > *(long long *)(a3 + 64);
    return v6;
  }
}



// Function: pred_ipath @ 0x91e0
bool pred_ipath(char *name, long long a2, long long a3)
{
  return fnmatch(*(const char **)(a3 + 56), name, 16) == 0;
}



// Function: pred_links @ 0x9210
long long pred_links(long long a1, long long a2, long long a3, long long a4, unsigned int a5)
{
  int v5; // eax
  unsigned int v6; // r8d

  v5 = *(int *)(a3 + 56);
  if ( v5 == 1 )
  {
    LOBYTE(a5) = *(long long *)(a2 + 16) < *(long long *)(a3 + 64);
    return a5;
  }
  else if ( v5 == 2 )
  {
    LOBYTE(a5) = *(long long *)(a2 + 16) == *(long long *)(a3 + 64);
    return a5;
  }
  else
  {
    v6 = 0;
    if ( !v5 )
      LOBYTE(v6) = *(long long *)(a2 + 16) > *(long long *)(a3 + 64);
    return v6;
  }
}



// Function: pred_lname @ 0x9260
long long pred_lname(long long a1, long long a2, long long a3)
{
  return sub_8AD0(a1, *(unsigned int *)(a2 + 24), a3, 0);
}



// Function: pred_mmin @ 0x9280
char pred_mmin(long long a1, long long a2, long long a3)
{
  time_t v4; // rax
  long long v5; // rdx

  v4 = sub_25C60(a2);
  return sub_87D0(v4, v5, a3, 60);
}



// Function: pred_mtime @ 0x92b0
char pred_mtime(long long a1, long long a2, long long a3)
{
  time_t v4; // rax
  long long v5; // rdx

  v4 = sub_25C60(a2);
  return sub_87D0(v4, v5, a3, 86400);
}



// Function: pred_name @ 0x92e0
bool pred_name(long long a1, long long a2, long long a3)
{
  return sub_8940(a1, *(const char **)(a3 + 56), 0);
}



// Function: pred_negate @ 0x92f0
long long pred_negate(long long a1, long long a2, long long a3)
{
  return (unsigned int)sub_E2A0(a1, a2, *(long long *)(a3 + 280)) ^ 1;
}



// Function: pred_newer @ 0x9310
bool pred_newer(long long a1, long long a2, long long a3)
{
  time_t v4; // rax
  long long v5; // rdx

  if ( *(int *)(a3 + 60) )
    __assert_fail("COMP_GT == pred_ptr->args.reftime.kind", "pred.c", 0x26Du, "pred_newer");
  v4 = sub_25C60(a2);
  return (int)sub_8790(v4, v5, *(long long *)(a3 + 64), *(long long *)(a3 + 72)) > 0;
}



// Function: pred_newerXY @ 0x9370
bool pred_newerXY(long long a1, long long a2, long long a3)
{
  time_t v5; // rdi
  long long v6; // rdx
  long long v7; // rsi
  bool result; // al
  long long v9; // rdx
  long long v10; // rdx
  long long v11; // r12
  char *v12; // rax
  long long v13; // rdx

  if ( *(int *)(a3 + 60) )
    __assert_fail("COMP_GT == pred_ptr->args.reftime.kind", "pred.c", 0x277u, "pred_newerXY");
  switch ( *(int *)(a3 + 56) )
  {
    case 0:
      v5 = sub_25C40(a2);
      v7 = v13;
      goto LABEL_4;
    case 1:
      v5 = sub_25C70(a2);
      v7 = v10;
      if ( v10 >= 0 )
        goto LABEL_4;
      v11 = sub_DFB0(0, a1);
      v12 = dcgettext(0, "WARNING: cannot determine birth time of file %s", 5);
      error(0, 0, v12, v11);
      result = 0;
      break;
    case 2:
      v5 = sub_25C50(a2);
      v7 = v9;
      goto LABEL_4;
    case 3:
      v5 = sub_25C60(a2);
      v7 = v6;
LABEL_4:
      result = (int)sub_8790(v5, v7, *(long long *)(a3 + 64), *(long long *)(a3 + 72)) > 0;
      break;
    case 4:
      __assert_fail("pred_ptr->args.reftime.xval != XVAL_TIME", "pred.c", 0x27Cu, "pred_newerXY");
    default:
      __assert_fail("collected", "pred.c", 0x29Bu, "pred_newerXY");
  }
  return result;
}



// Function: pred_nogroup @ 0x94a0
bool pred_nogroup(long long a1, long long a2)
{
  return getgrgid(*(int *)(a2 + 32)) == 0;
}



// Function: pred_nouser @ 0x94c0
bool pred_nouser(long long a1, long long a2)
{
  return getpwuid(*(int *)(a2 + 28)) == 0;
}



// Function: pred_ok @ 0x94e0
long long pred_ok(char *s, long long a2, long long a3)
{
  if ( (unsigned char)sub_8990(**(long long **)(a3 + 232), (long long)s) )
    return sub_A390(s);
  else
    return 0;
}



// Function: pred_okdir @ 0x9530
long long pred_okdir(long long a1, long long a2, long long a3)
{
  if ( (unsigned char)sub_8990(**(long long **)(a3 + 232), a1) )
    return sub_A390(src);
  else
    return 0;
}



// Function: pred_path @ 0x95a0
bool pred_path(char *name, long long a2, long long a3)
{
  return fnmatch(*(const char **)(a3 + 56), name, 0) == 0;
}



// Function: pred_perm @ 0x95c0
long long pred_perm(long long a1, long long a2, long long a3, long long a4, unsigned int a5)
{
  int v5; // eax
  int v6; // ecx
  int v7; // edx
  unsigned int v8; // r8d

  v5 = *(int *)(a2 + 24);
  v6 = *(int *)(a3 + 4LL * ((v5 & 0xF000) == 0x4000) + 60);
  v7 = *(int *)(a3 + 56);
  if ( v7 == 1 )
  {
    v8 = 1;
    if ( v6 )
      LOBYTE(v8) = (v6 & v5) != 0;
    return v8;
  }
  else if ( v7 == 2 )
  {
    LOBYTE(a5) = (v5 & 0xFFF) == v6;
    return a5;
  }
  else
  {
    if ( v7 )
      abort();
    LOBYTE(a5) = (v6 & v5) == v6;
    return a5;
  }
}



// Function: pred_executable @ 0x9640
bool pred_executable()
{
  return faccessat(dword_3E5F8, src, 1, 0) == 0;
}



// Function: pred_readable @ 0x9670
bool pred_readable()
{
  return faccessat(dword_3E5F8, src, 4, 0) == 0;
}



// Function: pred_writable @ 0x96a0
bool pred_writable()
{
  return faccessat(dword_3E5F8, src, 2, 0) == 0;
}



// Function: pred_regex @ 0x96f0
bool pred_regex(char *string, long long a2, long long a3)
{
  int v4; // ebx

  v4 = strlen(string);
  return re_match(*(struct re_pattern_buffer **)(a3 + 56), string, v4, 0, 0) == v4;
}



// Function: pred_size @ 0x9730
long long pred_size(long long a1, long long a2, long long a3)
{
  unsigned int v3; // r8d
  long long v5; // rax
  bool v6; // cf
  int v7; // edx
  unsigned long long v8; // rax
  unsigned int v9; // r8d

  v3 = a2;
  v5 = *(long long *)(a2 + 48) / *(int *)(a3 + 60);
  v6 = *(long long *)(a2 + 48) % *(int *)(a3 + 60) == 0;
  v7 = *(int *)(a3 + 56);
  v8 = v5 - (v6 - 1LL);
  if ( v7 == 1 )
  {
    LOBYTE(v3) = *(long long *)(a3 + 64) > v8;
    return v3;
  }
  else if ( v7 == 2 )
  {
    LOBYTE(v3) = *(long long *)(a3 + 64) == v8;
    return v3;
  }
  else
  {
    v9 = 0;
    if ( !v7 )
      LOBYTE(v9) = *(long long *)(a3 + 64) < v8;
    return v9;
  }
}



// Function: pred_samefile @ 0x97a0
char pred_samefile(long long a1, long long *a2, long long a3)
{
  long long v4; // rax
  int v5; // eax

  v4 = a2[1];
  if ( v4 && v4 != *(long long *)(a3 + 56) || (v5 = sub_E080(a1, src, a2)) != 0 )
  {
    LOBYTE(v5) = 0;
  }
  else if ( a2[1] == *(long long *)(a3 + 56) )
  {
    LOBYTE(v5) = *a2 == *(long long *)(a3 + 64);
  }
  return v5;
}



// Function: pred_type @ 0x9810
long long pred_type(long long a1, long long a2, long long a3)
{
  int v3; // eax
  unsigned int v4; // r8d
  unsigned int v5; // eax
  long long v6; // rcx

  if ( !HIBYTE(word_3E5E4) )
    __assert_fail("state.have_type", "pred.c", 0x3D1u, "pred_type");
  LOWORD(v3) = dword_3E5E8;
  v4 = 0;
  if ( dword_3E5E8 )
  {
    if ( (char)word_3E5E4 )
      v3 = *(int *)(a2 + 24);
    v5 = v3 & 0xF000;
    if ( v5 == 24576 )
    {
      v6 = 0;
    }
    else if ( v5 > 0x6000 )
    {
      if ( v5 == 40960 )
      {
        v6 = 4;
      }
      else
      {
        v6 = 6;
        if ( v5 != 49152 )
        {
          if ( v5 != 0x8000 )
            return 0;
          v6 = 3;
        }
      }
    }
    else if ( v5 == 0x2000 )
    {
      v6 = 1;
    }
    else
    {
      v6 = 2;
      if ( v5 != 0x4000 )
      {
        v6 = 5;
        if ( v5 != 4096 )
          return 0;
      }
    }
    return *(unsigned char *)(a3 + v6 + 56);
  }
  return v4;
}



// Function: pred_uid @ 0x98f0
long long pred_uid(long long a1, long long a2, long long a3, long long a4, unsigned int a5)
{
  int v5; // eax
  unsigned int v6; // r8d

  v5 = *(int *)(a3 + 56);
  if ( v5 == 1 )
  {
    LOBYTE(a5) = (unsigned long long)*(unsigned int *)(a2 + 28) < *(long long *)(a3 + 64);
    return a5;
  }
  else if ( v5 == 2 )
  {
    LOBYTE(a5) = *(unsigned int *)(a2 + 28) == *(long long *)(a3 + 64);
    return a5;
  }
  else
  {
    v6 = 0;
    if ( !v5 )
      LOBYTE(v6) = (unsigned long long)*(unsigned int *)(a2 + 28) > *(long long *)(a3 + 64);
    return v6;
  }
}



// Function: pred_used @ 0x9940
char pred_used(long long a1, long long a2, long long a3)
{
  time_t v4; // r13
  long long v5; // rdx
  long long v6; // r12
  time_t v7; // rax
  long long v8; // rdx
  long long v9; // rbx
  time_t v10; // rbp
  time_t v11; // rdi
  long long v12; // rbx
  long long v13; // rsi

  v4 = sub_25C40(a2);
  v6 = v5;
  v7 = sub_25C50(a2);
  v9 = v8;
  v10 = v7;
  if ( (int)sub_8790(v4, v6, v7, v8) < 0 )
    return 0;
  v11 = v10 - v4;
  v12 = v9 - v6;
  v13 = v12;
  if ( v12 < 0 )
  {
    v13 = v12 + 1000000000;
    --v11;
  }
  return sub_87D0(v11, v13, a3, 86400);
}



// Function: pred_user @ 0x99d0
bool pred_user(long long a1, long long a2, long long a3)
{
  return *(int *)(a3 + 56) == *(int *)(a2 + 28);
}



// Function: pred_xtype @ 0x99e0
long long pred_xtype(long long a1, long long a2, long long a3)
{
  unsigned int (*v4)(char *, struct stat *); // rbx
  char v5; // bl
  int *v6; // rax
  int *v7; // r13
  const char *v8; // rax
  struct stat v10; // [rsp+0h] [rbp-C8h] BYREF
  unsigned long long v11; // [rsp+98h] [rbp-30h]

  v11 = __readfsqword(0x28u);
  if ( (unsigned char)sub_DC30() )
    v4 = (unsigned int (*)(char *, struct stat *))sub_DB20;
  else
    v4 = (unsigned int (*)(char *, struct stat *))sub_DAA0;
  sub_D860(&v10);
  if ( !v4(src, &v10) )
    return sub_9810(a1, (long long)&v10, a3);
  v5 = sub_DC30();
  v6 = __errno_location();
  v7 = v6;
  if ( v5 && *v6 == 2 )
    return sub_9810(a1, a2, a3);
  v8 = (const char *)sub_DFB0(0, a1);
  error(0, *v7, "%s", v8);
  status = 1;
  return 0;
}



// Function: pred_context @ 0x9af0
long long pred_context(long long a1, long long a2, long long a3)
{
  unsigned int v3; // r12d
  long long v6; // r13
  char *v7; // r12
  int *v8; // rax
  const char *v9; // rdx
  char *name[7]; // [rsp+0h] [rbp-38h] BYREF

  name[1] = (char *)__readfsqword(0x28u);
  if ( (int)qword_3E680((unsigned int)dword_3E5F8, src, name) < 0 )
  {
    v6 = sub_DFB0(0, a1);
    v7 = dcgettext(0, "getfilecon failed: %s", 5);
    v8 = __errno_location();
    v9 = v7;
    v3 = 0;
    error(0, *v8, v9, v6);
  }
  else
  {
    LOBYTE(v3) = fnmatch(*(const char **)(a3 + 56), name[0], 0) == 0;
    freecon(name[0]);
  }
  return v3;
}



// Function: print_list @ 0x9bc0
unsigned long long print_list(long long a1, long long a2)
{
  long long v2; // rbx
  char *v3; // rax
  char v5[264]; // [rsp+0h] [rbp-138h] BYREF
  unsigned long long v6; // [rsp+108h] [rbp-30h]

  v6 = __readfsqword(0x28u);
  if ( a2 )
  {
    v2 = a2;
    do
    {
      v3 = sub_8A40(*(char **)(v2 + 8), v5);
      __fprintf_chk(a1, 1, "[%s] ", v3);
      v2 = *(long long *)(v2 + 264);
    }
    while ( v2 );
  }
  __fprintf_chk(a1, 1, "\n");
  return v6 - __readfsqword(0x28u);
}



// Function: print_optlist @ 0x9c70
void print_optlist(long long a1, long long a2)
{
  const char *v2; // r9
  const char *v3; // r8
  const char *v4; // rcx
  long long v5; // rax
  long long v6; // rdx
  double v7; // xmm0_8
  double v8; // xmm1_8
  long long v9; // rcx
  long long v10; // rdx

  if ( a2 )
  {
    sub_9E10(a1, *(long long *)(a2 + 272));
    v2 = "[need inum] ";
    if ( !*(char *)(a2 + 28) )
      v2 = "";
    v3 = "[need type] ";
    if ( !*(char *)(a2 + 27) )
      v3 = "";
    v4 = "[call stat] ";
    if ( !*(char *)(a2 + 26) )
      v4 = "";
    __fprintf_chk(a1, 1, "%s%s%s", v4, v3, v2);
    sub_B5A0(a1, a2);
    __fprintf_chk(a1, 1, " [est success rate %.4g] ", *(float *)(a2 + 36));
    if ( (qword_3E660 & 0x40) != 0 )
    {
      __fprintf_chk(a1, 1, "[real success rate %lu/%lu", *(long long *)(a2 + 296), *(long long *)(a2 + 288));
      v5 = *(long long *)(a2 + 288);
      if ( v5 )
      {
        v6 = *(long long *)(a2 + 296);
        if ( v6 < 0 )
        {
          v9 = *(long long *)(a2 + 296) & 1LL | (*(long long *)(a2 + 296) >> 1);
          v7 = (double)(int)v9 + (double)(int)v9;
        }
        else
        {
          v7 = (double)(int)v6;
        }
        if ( v5 < 0 )
        {
          v10 = *(long long *)(a2 + 288) & 1LL | (*(long long *)(a2 + 288) >> 1);
          v8 = (double)(int)v10 + (double)(int)v10;
        }
        else
        {
          v8 = (double)(int)v5;
        }
        __fprintf_chk(a1, 1, "=%.4g] ", v7 / v8);
      }
      else
      {
        __fprintf_chk(a1, 1, "=_] ");
      }
    }
    sub_9E10(a1, *(long long *)(a2 + 280));
  }
}



// Function: print_parenthesised @ 0x9e10
void print_parenthesised(long long a1, long long a2)
{
  long long v2; // rbp
  long long v3; // rdx

  if ( a2 )
  {
    v2 = a2;
    while ( 1 )
    {
      v3 = *(long long *)(v2 + 272);
      if ( *(long long (**)(long long, long long, long long))v2 != sub_88C0
        && *(long long (**)(long long, long long, long long))v2 != sub_8870 )
      {
        break;
      }
      if ( v3 )
        goto LABEL_7;
      v2 = *(long long *)(v2 + 280);
      if ( !v2 )
        return;
    }
    if ( v3 || *(long long *)(v2 + 280) )
    {
LABEL_7:
      __fprintf_chk(a1, 1, "%s", " ( ");
      sub_9C70(a1, v2);
      __fprintf_chk(a1, 1, "%s", " ) ");
      return;
    }
    sub_9C70(a1, v2);
  }
}



// Function: show_success_rates @ 0x9ee0
long long show_success_rates(long long a1)
{
  long long result; // rax

  if ( (qword_3E660 & 0x40) != 0 )
  {
    __fprintf_chk(stderr, 1, "Predicate success rates after completion:\n");
    sub_9C70((long long)stderr, a1);
    return __fprintf_chk(stderr, 1, "\n");
  }
  return result;
}



// Function: pred_sanity_check @ 0x9f40
void pred_sanity_check(long long a1)
{
  long long (*v1)(long long, long long); // rdx
  long long v2; // rax
  long long (*v3)(long long, long long); // rcx

  for ( ; a1; a1 = *(long long *)(a1 + 264) )
  {
    v1 = *(long long (**)(long long, long long))a1;
    if ( !*(long long *)a1 )
      __assert_fail("p->pred_func != NULL", "pred.c", 0x51Eu, "pred_sanity_check");
    v2 = *(long long *)(a1 + 304);
    if ( !v2 )
      __assert_fail("p->parser_entry != NULL", "pred.c", 0x521u, "pred_sanity_check");
    v3 = *(long long (**)(long long, long long))(v2 + 24);
    if ( v1 != v3 && v3 )
      __assert_fail("p->parser_entry->pred_func == p->pred_func", "pred.c", 0x52Au, "pred_sanity_check");
    switch ( *(int *)v2 )
    {
      case 0:
      case 2:
        if ( *(int *)v2 )
          __assert_fail("p->parser_entry->type != ARG_POSITIONAL_OPTION", "pred.c", 0x53Bu, "pred_sanity_check");
        __assert_fail("p->parser_entry->type != ARG_OPTION", "pred.c", 0x53Au, "pred_sanity_check");
      case 1:
      case 3:
      case 4:
      case 5:
        if ( *(char *)(a1 + 25) )
          __assert_fail("!p->no_default_print", "pred.c", 0x553u, "pred_sanity_check");
        if ( *(char *)(a1 + 24) )
          __assert_fail("!p->side_effects", "pred.c", 0x554u, "pred_sanity_check");
        return;
      case 6:
        if ( !*(char *)(a1 + 24) )
          __assert_fail("p->side_effects", "pred.c", 0x53Fu, "pred_sanity_check");
        if ( v1 != sub_86F0 && (char *)v1 != (char *)&loc_8A20 && !*(char *)(a1 + 25) )
          __assert_fail("p->no_default_print", "pred.c", 0x545u, "pred_sanity_check");
        return;
      default:
        continue;
    }
  }
}



// Function: initialize_wd_for_exec @ 0xa120
long long initialize_wd_for_exec(long long a1, int a2, int a3)
{
  long long v4; // rax
  int v5; // ecx
  int v6; // r8d
  int v7; // r9d
  long long v8; // rdi
  long long result; // rax

  v4 = sub_26990(16);
  *(long long *)(a1 + 192) = v4;
  *(long long *)(v4 + 8) = 0;
  *(int *)v4 = sub_1E690(a2, a3, 0, v5, v6, v7);
  v8 = **(unsigned int **)(a1 + 192);
  result = 0;
  if ( (int)v8 >= 0 )
  {
    sub_174F0(v8, 1);
    return 1;
  }
  return result;
}



// Function: record_exec_dir @ 0xa190
long long record_exec_dir(long long a1)
{
  char *v1; // r12
  void *v2; // r13
  unsigned int v3; // r12d

  if ( *(int *)(a1 + 152) )
    return 1;
  v1 = src;
  if ( !strchr(src, 47) )
    return sub_A120(a1, dword_3E5F8, (int)".");
  v2 = (void *)sub_177A0(v1);
  v3 = sub_A120(a1, dword_3E5F8, (int)v2);
  free(v2);
  return v3;
}



// Function: prep_child_for_exec @ 0xa220
long long prep_child_for_exec(unsigned int a1, long long a2)
{
  unsigned int v2; // r12d
  const char *v3; // r12
  const char *v4; // r14
  char *v5; // r13
  int *v6; // rax
  const char *v7; // r8
  unsigned int v9; // r13d
  int v10; // edx
  int v11; // ecx
  int v12; // r8d
  int v13; // r9d
  char *v14; // r12
  int *v15; // rax
  const char *v16; // r12
  int *v17; // rax
  long long v18[7]; // [rsp+Eh] [rbp-3Ah] BYREF

  v2 = 1;
  *(long long *)((char *)&v18[1] + 2) = __readfsqword(0x28u);
  if ( (char)a1 )
  {
    v9 = a1;
    strcpy((char *)v18, "/dev/null");
    if ( close(0) < 0 )
    {
      v9 = 0;
      v14 = dcgettext(0, "Cannot close standard input", 5);
      v15 = __errno_location();
      error(0, *v15, v14);
    }
    else if ( (int)sub_17B20((unsigned int)v18, 0, v10, v11, v12, v13) < 0 )
    {
      v16 = (const char *)sub_DFB0(0, v18);
      v17 = __errno_location();
      error(0, *v17, "%s", v16);
    }
    v2 = v9;
  }
  if ( (unsigned int)sub_25250(a2) )
  {
    v3 = "";
    v4 = "";
    if ( *(int *)a2 < 0 )
    {
      v3 = *(const char **)(a2 + 8);
      v4 = ": ";
      if ( !v3 )
      {
        v3 = "";
        v4 = "";
      }
    }
    v5 = dcgettext(0, "Failed to change directory%s%s", 5);
    v6 = __errno_location();
    v7 = v3;
    v2 = 0;
    error(0, *v6, v5, v4, v7);
  }
  return v2;
}



// Function: impl_pred_exec @ 0xa390
long long impl_pred_exec(char *s, long long a2, long long a3)
{
  const char *v3; // r12
  long long v5; // r15
  long long v6; // rbx
  const char *v7; // rdx
  size_t v8; // rax
  long long v9; // rdx
  size_t v11; // rax
  const char *v12; // rax
  long long v13; // r13
  char *v14; // r12
  int *v15; // rax
  const char *v16; // [rsp+8h] [rbp-60h]
  size_t v17; // [rsp+10h] [rbp-58h]
  const char *v18; // [rsp+18h] [rbp-50h]
  unsigned char v19; // [rsp+27h] [rbp-41h]
  char *ptr; // [rsp+28h] [rbp-40h]

  v3 = s;
  v19 = sub_DF90(*(long long *)a3);
  if ( !v19 )
  {
    if ( *(void **)(a3 + 248) == qword_3E5C0 )
    {
      v18 = 0;
      v5 = 0;
      ptr = 0;
      goto LABEL_4;
    }
LABEL_23:
    __assert_fail("execp->wd_for_exec == initial_wd", "exec.c", 0x94u, "impl_pred_exec");
  }
  if ( !(unsigned char)sub_A190(a3 + 56) )
  {
    v13 = sub_DFB0(0, s);
    v14 = dcgettext(0, "Failed to save working directory in order to run a command on %s", 5);
    v15 = __errno_location();
    error(1, *v15, v14, v13);
    goto LABEL_23;
  }
  ptr = (char *)sub_176F0(src);
  v3 = ptr;
  v5 = 2LL * (*ptr != 47);
  v12 = 0;
  if ( *ptr != 47 )
    v12 = "./";
  v18 = v12;
LABEL_4:
  LODWORD(v6) = *(unsigned char *)(a3 + 56);
  if ( !(char)v6 )
  {
    v6 = 0;
    while ( *(int *)(a3 + 240) > (int)v6 )
    {
      v17 = strlen(v3);
      v7 = *(const char **)(*(long long *)(a3 + 232) + 8 * v6++);
      v16 = v7;
      v8 = strlen(v7);
      sub_15290(a3 + 64, a3 + 152, v16, v8, v18, v5, v3, v17, 0);
    }
    sub_14E90(a3 + 64, a3 + 152);
    v9 = *(int *)(a3 + 256) & 0x7F | (unsigned int)(unsigned char)BYTE1(*(int *)(a3 + 256));
    LOBYTE(v6) = (int)v9 == 0;
    if ( !v19 )
    {
      if ( !ptr )
        return (unsigned int)v6;
LABEL_17:
      __assert_fail("local", "exec.c", 0xD0u, "impl_pred_exec");
    }
    sub_25280(*(long long *)(a3 + 248), a3 + 152, v9);
    if ( !ptr )
      return (unsigned int)v6;
    v19 = v6;
LABEL_12:
    free(ptr);
    LODWORD(v6) = v19;
    return (unsigned int)v6;
  }
  v11 = strlen(v3);
  sub_15000(a3 + 64, a3 + 152, v3, v11 + 1, v18, v5, 0);
  if ( *(int *)(a3 + 208) )
    byte_3E608 = 1;
  if ( ptr )
  {
    if ( !v19 )
      goto LABEL_17;
    goto LABEL_12;
  }
  return (unsigned int)v6;
}



// Function: launch @ 0xa610
long long launch(long long a1, long long a2, long long a3, const char **a4)
{
  __pid_t v6; // eax
  __pid_t v7; // r13d
  long long v8; // rsi
  char v9; // bp
  int *v10; // rax
  int *v11; // rbx
  const char *v12; // rax
  int *v13; // rbx
  int v14; // ebx
  int v15; // ebx
  unsigned long long v17; // rbx
  long long v18; // rsi
  const char *v19; // rax
  long long v20; // r12
  char *v21; // rax
  long long v22; // r12
  char *v23; // rax
  char *v24; // r12
  int *v25; // rax

  if ( (qword_3E660 & 0x20) != 0 )
  {
    v17 = 0;
    __fprintf_chk(stderr, 1, "DebugExec: launching process (argc=%lu):", *(long long *)(a2 + 96) - 1LL);
    if ( *(long long *)(a2 + 96) != 1 )
    {
      do
      {
        v18 = *(long long *)(*(long long *)(a2 + 104) + 8 * v17++);
        v19 = (const char *)sub_DFB0(0, v18);
        __fprintf_chk(stderr, 1, " %s", v19);
      }
      while ( *(long long *)(a2 + 96) - 1LL > v17 );
    }
    __fprintf_chk(stderr, 1, "\n");
  }
  sub_17BC0(stdout);
  sub_17BC0(stderr);
  if ( dword_3E01C )
  {
    dword_3E01C = 0;
    signal(17, 0);
  }
  v6 = fork();
  v7 = v6;
  if ( v6 == -1 )
  {
    v24 = dcgettext(0, "cannot fork", 5);
    v25 = __errno_location();
    error(1, *v25, v24);
    goto LABEL_31;
  }
  if ( !v6 )
  {
    v8 = *(long long *)(a2 + 192);
    if ( v8 )
    {
      if ( (unsigned char)sub_A220(*(unsigned char *)(a2 + 188), v8) )
      {
        if ( (unsigned char)sub_15DD0() )
          sub_15DF0();
        v9 = sub_15470(a4);
        v10 = __errno_location();
        v11 = v10;
        if ( v9 )
          *v10 = 7;
        else
          execvp(*a4, (char *const *)a4);
        v12 = (const char *)sub_DFB0(0, *a4);
        error(0, *v11, "%s", v12);
      }
      _exit(1);
    }
LABEL_31:
    __assert_fail("NULL != execp->wd_for_exec", "exec.c", 0x141u, "launch");
  }
  do
  {
    if ( waitpid(v7, (int *)(a2 + 200), 0) != -1 )
    {
      v14 = *(int *)(a2 + 200);
      if ( __OFSUB__((*(char *)(a2 + 200) & 0x7F) + 1, 1) || (*(char *)(a2 + 200) & 0x7F) == 0 )
      {
        v15 = BYTE1(v14);
        if ( (qword_3E660 & 0x20) != 0 )
          __fprintf_chk(stderr, 1, "DebugExec: process (PID=%ld) terminated with exit status: %d\n", v7, v15);
        if ( !v15 )
          return 1;
      }
      else
      {
        v22 = sub_24E50(0, (unsigned int)dword_3E68C, *a4);
        v23 = dcgettext(0, "%s terminated by signal %d", 5);
        error(0, 0, v23, v22, v14 & 0x7F);
      }
      if ( *(char *)a2 )
        status = 1;
      return 1;
    }
    v13 = __errno_location();
  }
  while ( *v13 == 4 );
  v20 = sub_DFB0(0, *a4);
  v21 = dcgettext(0, "error waiting for %s", 5);
  error(0, *v13, v21, v20);
  status = 1;
  return 0;
}



// Function: predicate_is_cost_free @ 0xa950
bool predicate_is_cost_free(long long a1)
{
  long long (*v1)(long long, long long, long long); // rdx
  bool result; // al

  v1 = *(long long (**)(long long, long long, long long))a1;
  result = *(long long *)a1 == (long long)sub_91E0
        || *(long long *)a1 == (long long)sub_9170
        || *(long long *)a1 == (long long)sub_95A0
        || *(long long *)a1 == (long long)sub_92E0;
  if ( !result && v1 != sub_8CD0 && word_3E688 != 0 && v1 != sub_92F0 && v1 != sub_8870 && v1 != sub_88C0 )
    return *(int *)(a1 + 32) == 0;
  return result;
}



// Function: predlist_init @ 0xa9e0
void predlist_init(long long *a1)
{
  a1[1] = 0;
  *a1 = 0;
}



// Function: predlist_insert @ 0xa9f0
long long predlist_insert(long long *a1, long long a2, long long *a3)
{
  long long result; // rax
  bool v4; // zf

  *a3 = *(long long *)(a2 + 272);
  result = *a1;
  v4 = a1[1] == 0;
  *(long long *)(a2 + 272) = *a1;
  *a1 = a2;
  if ( v4 )
    a1[1] = a2;
  return result;
}



// Function: pred_cost_compare @ 0xaa20
long long pred_cost_compare(long long a1, long long a2, char a3)
{
  unsigned int v3; // eax
  float v4; // xmm0_4
  float v5; // xmm1_4
  long long result; // rax

  v3 = *(int *)(a2 + 32);
  if ( *(int *)(a1 + 32) != v3 )
    return *(int *)(a1 + 32) < v3 ? -1 : 1;
  v4 = *(float *)(a1 + 36);
  v5 = *(float *)(a2 + 36);
  result = 0;
  if ( v4 != v5 )
  {
    if ( a3 )
    {
      result = 0xFFFFFFFFLL;
      if ( v5 <= v4 )
        return 1;
    }
    else
    {
      return 2 * (unsigned int)(v5 > v4) - 1;
    }
  }
  return result;
}



// Function: subtree_has_side_effects @ 0xaa70
long long subtree_has_side_effects(long long a1)
{
  long long result; // rax

  while ( a1 )
  {
    result = *(unsigned char *)(a1 + 24);
    if ( (char)result )
      return result;
    result = subtree_has_side_effects(*(long long *)(a1 + 272));
    if ( (char)result )
      return result;
    a1 = *(long long *)(a1 + 280);
  }
  return 0;
}



// Function: worst_cost @ 0xaab0
long long worst_cost(long long a1)
{
  unsigned int v1; // ebx
  long long result; // rax
  unsigned int v3; // edx

  if ( !a1 )
    return 0;
  v1 = worst_cost(*(long long *)(a1 + 272));
  result = worst_cost(*(long long *)(a1 + 280));
  v3 = *(int *)(a1 + 32);
  if ( v1 >= (unsigned int)result )
    result = v1;
  if ( (unsigned int)result < v3 )
    return v3;
  return result;
}



// Function: perform_arm_swap @ 0xab00
long long perform_arm_swap(long long a1)
{
  long long result; // rax
  long long v2; // rdx

  result = *(long long *)(a1 + 272);
  v2 = *(long long *)(result + 280);
  *(long long *)(result + 280) = *(long long *)(a1 + 280);
  *(long long *)(a1 + 280) = v2;
  return result;
}



// Function: constrain_rate @ 0xab30
float constrain_rate(float a1)
{
  if ( a1 > 1.0 )
    return 1.0;
  else
    return fmaxf(0.0, a1);
}



// Function: merge_pred @ 0xab60
long long merge_pred(long long a1, long long a2, long long *a3)
{
  long long result; // rax

  result = *a3;
  *(long long *)(a2 + 272) = *a3;
  *a3 = a1;
  return result;
}



// Function: getrate @ 0xab70
float getrate(long long a1)
{
  float result; // xmm0_4

  result = 1.0;
  if ( a1 )
    return *(float *)(a1 + 36);
  return result;
}



// Function: cost_name @ 0xab90
const char *cost_name(int a1)
{
  long long v1; // rax
  int *i; // rdx

  v1 = 0;
  for ( i = &unk_3E020; *i != a1; i += 4 )
  {
    v1 = (unsigned int)(v1 + 1);
    if ( (int)v1 == 11 )
      return "unknown";
  }
  return (const char *)*((long long *)&unk_3E020 + 2 * v1 + 1);
}



// Function: type_name @ 0xabd0
const char *type_name(short a1)
{
  const char *result; // rax
  short v2; // dx
  int v3; // ecx
  short *i; // rax

  result = "no";
  if ( a1 )
  {
    v2 = 1;
    v3 = 0;
    for ( i = &unk_3C040; ; v2 = i[8] )
    {
      ++v3;
      i += 8;
      if ( a1 == v2 || v2 == -1 )
        break;
    }
    return (const char *)*((long long *)&unk_3C040 + 2 * v3 + 1);
  }
  return result;
}



// Function: prec_name @ 0xac30
const char *prec_name(short a1)
{
  const char *result; // rax
  short v2; // dx
  int v3; // ecx
  short *i; // rax

  result = "no";
  if ( a1 )
  {
    v2 = 1;
    v3 = 0;
    for ( i = &unk_3C0C0; ; v2 = i[8] )
    {
      ++v3;
      i += 8;
      if ( a1 == v2 || v2 == -1 )
        break;
    }
    return (const char *)*((long long *)&unk_3C0C0 + 2 * v3 + 1);
  }
  return result;
}



// Function: check_normalization @ 0xac90
long long check_normalization(long long a1, char a2)
{
  long long v2; // rbx
  long long v3; // rdi
  long long result; // rax

  while ( 1 )
  {
    v2 = a1;
    if ( a2 && *(int *)(a1 + 16) != 3 )
      __assert_fail("BI_OP == p->p_type", "tree.c", 0x4CEu, "check_normalization");
    v3 = *(long long *)(a1 + 272);
    if ( v3 )
    {
      if ( *(int *)(v3 + 16) != 3 )
        __assert_fail("BI_OP == p->pred_left->p_type", "tree.c", 0x4D3u, "check_normalization");
      result = check_normalization(v3, 0);
    }
    a1 = *(long long *)(v2 + 280);
    if ( !a1 )
      break;
    a2 = 0;
  }
  return result;
}



// Function: scan_rest @ 0xad10
long long scan_rest(long long *a1, long long a2, short a3)
{
  long long v4; // rdx
  unsigned int v6; // r13d
  long long v7; // rsi
  char *v9; // rax
  char *v10; // rax

  v4 = *a1;
  if ( *a1 && *(int *)(v4 + 16) != 5 )
  {
    while ( 1 )
    {
      if ( *(int *)(v4 + 20) <= a3 )
        return a2;
      v6 = *(int *)(v4 + 16);
      if ( v6 == 4 )
        goto LABEL_12;
      if ( v6 > 4 )
        break;
      if ( v6 != 3 )
      {
LABEL_12:
        v10 = dcgettext(0, "invalid expression", 5);
        error(1, 0, v10);
        return ((long long (*)(void))sub_AE10)();
      }
      *(long long *)(v4 + 272) = a2;
      a2 = *a1;
      v7 = (unsigned int)*(short *)(*a1 + 20);
      *a1 = *(long long *)(*a1 + 264);
      *(long long *)(a2 + 280) = sub_AE10(a1, v7);
      v4 = *a1;
      if ( !*a1 )
        return a2;
    }
    if ( v6 == 5 )
      return a2;
    v9 = dcgettext(0, "oops -- invalid expression type (%d)!", 5);
    error(1, 0, v9, v6);
  }
  return 0;
}



// Function: get_expr @ 0xae10
int get_expr(long long *a1, short a2, long long a3)
{
  long long v3; // r12
  long long *v4; // rbp
  long long v5; // rax
  bool v6; // zf
  char *v7; // rax
  long long v8; // rax
  short v9; // dx
  char v11; // cl
  char *v12; // rax
  long long v13; // rax
  char *v14; // rax
  char *v15; // rax
  char *v16; // rax
  char *v17; // rax
  char *v18; // rax
  char *v19; // rax
  char *v20; // rax
  char *v21; // rax
  long long v22; // r12
  char *v23; // rax
  char *v24; // rax
  long long v25; // r12
  char *v26; // rax

  v3 = *a1;
  if ( !*a1 )
    goto LABEL_28;
  v4 = a1;
  switch ( *(int *)(v3 + 16) )
  {
    case 0:
      goto LABEL_26;
    case 1:
      v8 = *(long long *)(v3 + 264);
      *a1 = v8;
      goto LABEL_9;
    case 2:
      goto LABEL_8;
    case 3:
      goto LABEL_25;
    case 4:
      v5 = *(long long *)(v3 + 264);
      if ( !v5 || *(char *)(v5 + 41) )
        goto LABEL_29;
      v6 = *(int *)(v5 + 16) == 5;
      *a1 = v5;
      if ( !v6 )
        goto LABEL_17;
      if ( !*(char *)(v3 + 41) )
        goto LABEL_24;
      v3 = *(long long *)(v5 + 8);
      v7 = dcgettext(0, "invalid expression: expected expression before closing parentheses '%s'.", 5);
      a1 = (long long *)(&dword_0 + 1);
      error(1, 0, v7, v3);
LABEL_8:
      *a1 = *(long long *)(v3 + 264);
      *(long long *)(v3 + 280) = get_expr(a1, 4);
      v8 = *v4;
      goto LABEL_9;
    case 5:
      if ( !a3 )
        goto LABEL_30;
      v11 = *(char *)(v3 + 41);
      if ( (unsigned int)(*(int *)(a3 + 16) - 2) <= 1 )
      {
        if ( !v11 )
          goto LABEL_32;
        goto LABEL_21;
      }
      if ( v11 )
      {
LABEL_21:
        v3 = *(long long *)(a3 + 8);
        v14 = dcgettext(0, "expected an expression after '%s'", 5);
        error(1, 0, v14, v3);
        goto LABEL_22;
      }
      v12 = dcgettext(0, "invalid expression; you have too many ')'", 5);
      a1 = (long long *)(&dword_0 + 1);
      error(1, 0, v12);
LABEL_17:
      v3 = get_expr(a1, 0);
      v13 = *v4;
      if ( !*v4 || *(int *)(v13 + 16) != 5 )
        goto LABEL_31;
      v8 = *(long long *)(v13 + 264);
      *v4 = v8;
LABEL_9:
      if ( !v8 )
        return v3;
      v9 = a2;
      if ( *(int *)(v8 + 20) <= a2 )
        return v3;
LABEL_22:
      v3 = sub_AD10(v4, v3, v9);
      if ( v3 )
        return v3;
      v15 = dcgettext(0, "invalid expression", 5);
      error(1, 0, v15);
LABEL_24:
      v16 = dcgettext(0, "invalid expression; empty parentheses are not allowed.", 5);
      error(1, 0, v16);
LABEL_25:
      v3 = *(long long *)(v3 + 8);
      v17 = dcgettext(0, "invalid expression; you have used a binary operator '%s' with nothing before it.", 5);
      error(1, 0, v17, v3);
LABEL_26:
      v18 = dcgettext(0, "invalid expression", 5);
      error(1, 0, v18);
LABEL_27:
      v19 = dcgettext(0, "oops -- invalid expression type!", 5);
      error(1, 0, v19);
LABEL_28:
      v20 = dcgettext(0, "invalid expression", 5);
      error(1, 0, v20);
LABEL_29:
      v3 = *(long long *)(v3 + 8);
      v21 = dcgettext(
              0,
              "invalid expression; expected to find a ')' but didn't see one. Perhaps you need an extra predicate after '%s'",
              5);
      error(1, 0, v21, v3);
LABEL_30:
      v22 = *(long long *)(v3 + 8);
      v23 = dcgettext(0, "invalid expression: expected expression before closing parentheses '%s'.", 5);
      error(1, 0, v23, v22);
LABEL_31:
      v24 = dcgettext(0, "invalid expression; I was expecting to find a ')' somewhere but did not see one.", 5);
      error(1, 0, v24);
LABEL_32:
      v25 = *(long long *)(a3 + 8);
      v26 = dcgettext(0, "expected an expression between '%s' and ')'", 5);
      error(1, 0, v26, v25);
      return compar((char *)&dword_0 + 1, 0);
    default:
      goto LABEL_27;
  }
}



// Function: cost_table_comparison @ 0xb160
int cost_table_comparison(long long *a1, long long *a2)
{
  long long s1; // [rsp+8h] [rbp-20h] BYREF
  long long s2[3]; // [rsp+10h] [rbp-18h] BYREF

  s2[1] = __readfsqword(0x28u);
  s1 = *a1;
  s2[0] = *a2;
  return memcmp(&s1, s2, 8u);
}



// Function: set_new_parent @ 0xb1c0
void set_new_parent(long long a1, int a2, long long *a3)
{
  long long v4; // rax
  float v5; // xmm0_4
  float v6; // xmm0_4
  long long v7; // rax
  float v8; // xmm0_4
  float v9; // xmm0_4
  long long v10; // rax

  v4 = sub_26C50(312);
  *(int *)(v4 + 16) = 3;
  *(int *)(v4 + 20) = a2;
  *(int *)(v4 + 32) = 0;
  switch ( a2 )
  {
    case 2:
      v5 = *(float *)(a1 + 36);
      *(long long *)(v4 + 8) = "-o";
      *(long long *)v4 = sub_88C0;
      v6 = sub_AB30(v5);
      *(long long *)(v7 + 280) = a1;
      *(float *)(v7 + 36) = v6;
      *a3 = v7;
      break;
    case 3:
      v8 = *(float *)(a1 + 36);
      *(long long *)(v4 + 8) = "-a";
      *(long long *)v4 = sub_8870;
      v9 = sub_AB30(v8);
      *(long long *)(v10 + 280) = a1;
      *(float *)(v10 + 36) = v9;
      *a3 = v10;
      break;
    case 1:
      *a3 = v4;
      *(long long *)(v4 + 8) = ",";
      *(long long *)v4 = sub_8CD0;
      *(int *)(v4 + 36) = 1065353216;
      *(long long *)(v4 + 280) = a1;
      break;
    default:
      *(long long *)(v4 + 280) = a1;
      *a3 = v4;
      break;
  }
}



// Function: get_pred_cost @ 0xb350
long long get_pred_cost(long long a1)
{
  unsigned int v1; // ebp
  long long (*v2)(); // rax
  unsigned int *v3; // rax
  long long result; // rax
  long long v5; // r12
  char *v6; // rax
  long long v7[7]; // [rsp+0h] [rbp-38h] BYREF

  v1 = 3;
  v7[3] = __readfsqword(0x28u);
  if ( !*(char *)(a1 + 26) )
  {
    v1 = 1;
    if ( !*(char *)(a1 + 28) )
      v1 = 2 * *(unsigned char *)(a1 + 27);
  }
  v2 = *(long long (**)())a1;
  if ( *(long long (**)(char *))a1 == sub_8FE0 || v2 == sub_8FF0 )
    return 8 - (unsigned int)*(unsigned char *)(a1 + 56);
  if ( v2 == sub_FAD0 )
  {
    result = *(unsigned int *)(a1 + 32);
  }
  else
  {
    if ( !dword_3E6B0 )
    {
      qsort(&off_3E0E0, 0x3Bu, 0x10u, (__compar_fn_t)compar);
      if ( !(unsigned char)sub_B2D0() )
      {
        error(1, 0, "failed to sort the costlookup array");
        return sub_B4D0();
      }
      dword_3E6B0 = 1;
      v2 = *(long long (**)())a1;
    }
    v7[0] = v2;
    v3 = (unsigned int *)bsearch(v7, &off_3E0E0, 0x3Bu, 0x10u, (__compar_fn_t)compar);
    if ( v3 )
    {
      result = v3[2];
    }
    else
    {
      v5 = *(long long *)(a1 + 8);
      v6 = dcgettext(
             0,
             "warning: there is no entry in the predicate evaluation cost table for predicate %s; please report this as a bug",
             5);
      error(0, 0, v6, v5);
      result = 10;
    }
  }
  if ( v1 >= (unsigned int)result )
    return v1;
  return result;
}



// Function: estimate_costs @ 0xb4d0
void estimate_costs(long long a1)
{
  if ( a1 )
  {
    estimate_costs(*(long long *)(a1 + 280));
    estimate_costs(*(long long *)(a1 + 272));
    *(int *)(a1 + 32) = sub_B350(a1);
  }
}



// Function: matches_start_point @ 0xb510
bool matches_start_point(char *pattern, unsigned char a2)
{
  int v2; // r12d
  unsigned long long v3; // rbx

  v2 = 16 * a2;
  if ( !qword_3E6B8 )
    return fnmatch(pattern, ".", v2) == 0;
  v3 = 0;
  do
  {
    if ( !fnmatch(pattern, *(const char **)(qword_3E6C0 + 8 * v3), v2) )
      return 1;
    ++v3;
  }
  while ( qword_3E6B8 > v3 );
  return 0;
}



// Function: print_predicate @ 0xb5a0
long long print_predicate(long long a1, long long a2)
{
  const char *v2; // r8
  const char *v3; // rcx

  v2 = *(const char **)(a2 + 48);
  v3 = *(const char **)(a2 + 8);
  if ( v2 )
    return __fprintf_chk(a1, 1, "%s %s", v3, v2);
  else
    return __fprintf_chk(a1, 1, "%s", v3);
}



// Function: get_eval_tree @ 0xb5e0
long long get_eval_tree()
{
  return qword_3E6D0;
}



// Function: calculate_derived_rates @ 0xb5f0
void calculate_derived_rates(long long a1)
{
  long long v1; // r12
  long long v2; // rbp
  unsigned int v3; // eax
  long long (*v4)(long long, long long, long long); // rax
  float v5; // xmm1_4
  float v6; // xmm0_4
  float v7; // xmm1_4

  if ( !a1 )
    __assert_fail("NULL != p", "tree.c", 0x484u, "calculate_derived_rates");
  v1 = *(long long *)(a1 + 280);
  if ( v1 )
    calculate_derived_rates(*(long long *)(a1 + 280));
  v2 = *(long long *)(a1 + 272);
  if ( v2 )
    calculate_derived_rates(*(long long *)(a1 + 272));
  v3 = *(int *)(a1 + 16);
  switch ( v3 )
  {
    case 5u:
      __assert_fail("p->p_type != CLOSE_PAREN", "tree.c", 0x48Bu, "calculate_derived_rates");
    case 4u:
      __assert_fail("p->p_type != OPEN_PAREN", "tree.c", 0x48Cu, "calculate_derived_rates");
    case 2u:
      if ( *(long long (**)(long long, long long, long long))a1 != sub_92F0 )
        __assert_fail("pred_is (p, pred_negate)", "tree.c", 0x49Cu, "calculate_derived_rates");
      if ( v2 )
        __assert_fail("NULL == p->pred_left", "tree.c", 0x49Du, "calculate_derived_rates");
      *(float *)(a1 + 36) = 1.0 - *(float *)(v1 + 36);
      break;
    default:
      if ( v3 > 2 )
      {
        if ( v3 != 3 )
          __assert_fail("0", "tree.c", 0x4C0u, "calculate_derived_rates");
        v4 = *(long long (**)(long long, long long, long long))a1;
        if ( *(long long (**)(long long, long long, long long))a1 == sub_8870 )
        {
          v7 = sub_AB70(v1);
          v6 = sub_AB70(v2) * v7;
        }
        else if ( v4 == sub_8CD0 )
        {
          v6 = 1.0;
        }
        else
        {
          if ( v4 != sub_88C0 )
            __assert_fail("0", "tree.c", 0x4B4u, "calculate_derived_rates");
          v5 = sub_AB70(v1);
          v6 = sub_AB70(v2) + v5;
        }
        *(float *)(a1 + 36) = sub_AB30(v6);
      }
      else if ( v3 )
      {
        if ( v1 )
          __assert_fail("NULL == p->pred_right", "tree.c", 0x496u, "calculate_derived_rates");
        if ( v2 )
          __assert_fail("NULL == p->pred_left", "tree.c", 0x497u, "calculate_derived_rates");
      }
      else
      {
        if ( v1 )
          __assert_fail("NULL == p->pred_right", "tree.c", 0x491u, "calculate_derived_rates");
        if ( v2 )
          __assert_fail("NULL == p->pred_left", "tree.c", 0x492u, "calculate_derived_rates");
      }
      break;
  }
}



// Function: get_new_pred @ 0xb8a0
long long get_new_pred(int *a1)
{
  long long v1; // rax
  char v2; // dl

  if ( !*a1 )
    __assert_fail("entry->type != ARG_OPTION", "tree.c", 0x5CDu, "get_new_pred");
  if ( *a1 == 2 )
    __assert_fail("entry->type != ARG_POSITIONAL_OPTION", "tree.c", 0x5CEu, "get_new_pred");
  v1 = sub_26C50(312);
  if ( ptr )
    *(long long *)(qword_3E6C8 + 264) = v1;
  else
    ptr = (void *)v1;
  *(short *)(v1 + 26) = 257;
  v2 = byte_3E62F;
  *(long long *)(v1 + 304) = a1;
  *(long long *)(v1 + 16) = 0;
  *(long long *)(v1 + 48) = "ThisShouldBeSetToSomethingElse";
  *(char *)(v1 + 40) = v2;
  *(long long *)(v1 + 32) = 0x3F8000000000000ALL;
  qword_3E6C8 = v1;
  return sub_B2B0(v1);
}



// Function: get_new_pred_noarg @ 0xb970
long long get_new_pred_noarg(int *a1)
{
  long long result; // rax

  result = sub_B8A0(a1);
  if ( result )
    *(long long *)(result + 48) = 0;
  return result;
}



// Function: get_new_pred_chk_op @ 0xb990
long long get_new_pred_chk_op(int *a1, long long a2)
{
  int *v3; // rdi
  int v4; // eax
  long long result; // rax
  long long v6; // rax
  char *v7; // rax

  v3 = (int *)qword_3E6A0;
  if ( !qword_3E6A0 )
  {
    qword_3E6A0 = sub_147F0("and");
    v3 = (int *)qword_3E6A0;
    if ( !qword_3E6A0 )
      __assert_fail("entry_and != NULL", "tree.c", 0x5F8u, "get_new_pred_chk_op");
  }
  if ( !qword_3E6C8 )
    goto LABEL_5;
  v4 = *(int *)(qword_3E6C8 + 16);
  if ( (v4 & 0xFFFFFFFB) == 1 )
  {
    v6 = sub_B970(v3);
    *(long long *)(v6 + 8) = "-a";
    *(long long *)v6 = sub_8870;
    *(long long *)(v6 + 48) = 0;
    *(long long *)(v6 + 56) = 0;
    *(long long *)(v6 + 16) = 0x300000003LL;
    *(int *)(v6 + 24) = 0;
    *(char *)(v6 + 28) = 0;
    goto LABEL_5;
  }
  if ( v4 )
  {
LABEL_5:
    result = sub_B8A0(a1);
    *(long long *)(result + 48) = a2;
    *(long long *)(result + 304) = a1;
    return result;
  }
  v7 = dcgettext(0, "oops -- invalid default insertion of and!", 5);
  error(1, 0, v7);
  return sub_BAA0();
}



// Function: print_tree @ 0xbaa0
void print_tree(long long a1, long long a2, int a3)
{
  long long v3; // r12
  int v5; // r14d
  const char *v6; // rax
  const char *v7; // r8
  const char *v8; // rax
  const char *v9; // r8
  int v10; // r14d
  int v11; // r14d
  const char *v12; // rcx
  const char *v13; // rcx

  if ( a2 )
  {
    v3 = a2;
    do
    {
      if ( a3 > 0 )
      {
        v5 = 0;
        do
        {
          __fprintf_chk(a1, 1, "    ");
          ++v5;
        }
        while ( a3 != v5 );
      }
      __fprintf_chk(a1, 1, "pred=[");
      sub_B5A0(a1, v3);
      sub_AC30(*(short *)(v3 + 20));
      v6 = sub_ABD0(*(short *)(v3 + 16));
      __fprintf_chk(a1, 1, "] type=%s prec=%s", v6, v7);
      v8 = sub_AB90(*(int *)(v3 + 32));
      __fprintf_chk(a1, 1, " cost=%s est_success_rate=%#.4g %sside effects ", v8, *(float *)(v3 + 36), v9);
      if ( (*(long long *)(v3 + 24) & 0xFFFFFF0000LL) != 0 )
      {
        __fprintf_chk(a1, 1, "Needs ");
        if ( *(char *)(v3 + 26) )
        {
          __fprintf_chk(a1, 1, "stat");
          if ( *(char *)(v3 + 28) )
          {
            v12 = ",";
LABEL_20:
            __fprintf_chk(a1, 1, "%sinode", v12);
            if ( !*(char *)(v3 + 27) )
              goto LABEL_7;
LABEL_21:
            v13 = ",";
LABEL_22:
            __fprintf_chk(a1, 1, "%stype", v13);
            goto LABEL_7;
          }
          if ( *(char *)(v3 + 27) )
            goto LABEL_21;
        }
        else
        {
          if ( *(char *)(v3 + 28) )
          {
            v12 = "";
            goto LABEL_20;
          }
          if ( *(char *)(v3 + 27) )
          {
            v13 = "";
            goto LABEL_22;
          }
        }
      }
LABEL_7:
      __fprintf_chk(a1, 1, "\n");
      if ( a3 > 0 )
      {
        v10 = 0;
        do
        {
          __fprintf_chk(a1, 1, "    ");
          ++v10;
        }
        while ( a3 != v10 );
      }
      if ( *(long long *)(v3 + 272) )
      {
        __fprintf_chk(a1, 1, "left:\n");
        print_tree(a1, *(long long *)(v3 + 272), (unsigned int)(a3 + 1));
      }
      else
      {
        if ( !*(long long *)(v3 + 280) )
        {
          __fprintf_chk(a1, 1, "no children.\n");
          return;
        }
        __fprintf_chk(a1, 1, "no left.\n");
      }
      v11 = 0;
      if ( a3 > 0 )
      {
        do
        {
          __fprintf_chk(a1, 1, "    ");
          ++v11;
        }
        while ( a3 != v11 );
      }
      if ( !*(long long *)(v3 + 280) )
      {
        __fprintf_chk(a1, 1, "no right.\n");
        return;
      }
      ++a3;
      __fprintf_chk(a1, 1, "right:\n");
      v3 = *(long long *)(v3 + 280);
    }
    while ( v3 );
  }
}



// Function: predlist_merge_sort @ 0xbdc0
void predlist_merge_sort(long long *a1, long long *a2)
{
  long long v2; // rbx
  long long v4; // rbp
  long long v5; // r8
  long long v6; // rsi
  long long v7; // rcx
  long long v8; // rcx
  long long v9; // rax
  bool v10; // zf
  long long v11; // [rsp+0h] [rbp-48h] BYREF
  long long v12; // [rsp+8h] [rbp-40h]
  unsigned long long v13; // [rsp+18h] [rbp-30h]

  v2 = *a1;
  v13 = __readfsqword(0x28u);
  if ( !v2 )
    return;
  if ( (qword_3E660 & 8) == 0 )
  {
    sub_B5F0(v2);
    sub_A9E0(&v11);
    goto LABEL_4;
  }
  __fprintf_chk(stderr, 1, "%s:\n", "predlist before merge sort");
  sub_BAA0((long long)stderr, *a1, 2);
  v2 = *a1;
  sub_B5F0(*a1);
  sub_A9E0(&v11);
  v4 = v11;
  if ( v2 )
  {
LABEL_4:
    v4 = v11;
    do
    {
      v5 = v2;
      v2 = *(long long *)(v2 + 272);
      *a1 = v2;
      *(long long *)(v5 + 272) = 0;
      if ( v4 )
      {
        v6 = *(long long *)(v5 + 280);
        v7 = v4;
        while ( (int)sub_AA20(*(long long *)(v7 + 280), v6, *(int *)(v7 + 20) != 2) < 0 )
        {
          v7 = *(long long *)(v8 + 272);
          if ( !v7 )
            goto LABEL_16;
        }
        v9 = *(long long *)(v8 + 272);
        *(long long *)(v5 + 272) = v9;
        if ( !v9 )
          v12 = v5;
        *(long long *)(v8 + 272) = v5;
      }
      else
      {
LABEL_16:
        v10 = v12 == 0;
        *(long long *)(v5 + 272) = v4;
        v11 = v5;
        if ( v10 )
          v12 = v5;
        v4 = v5;
      }
    }
    while ( v2 );
  }
  if ( (qword_3E660 & 8) != 0 )
  {
    __fprintf_chk(stderr, 1, "%s:\n", "predlist after merge sort");
    sub_BAA0((long long)stderr, v4, 2);
  }
  sub_B5F0(v4);
  sub_AB60(v4, v12, a2);
  sub_A9E0(a1);
}



// Function: opt_expr @ 0xc030
long long opt_expr(long long *a1)
{
  long long v1; // rdx
  long long *v2; // rbx
  long long *v3; // rax
  long long v4; // rdx
  long long *v5; // rcx
  long long v6; // r9
  long long v7; // rdi
  long long v8; // r14
  int v9; // eax
  unsigned int v10; // ebp
  unsigned int v11; // r13d
  long long v12; // r15
  unsigned int v13; // eax
  bool (*v14)(char *, long long, long long); // r8
  long long v15; // rax
  long long v17; // rax
  long long *v18; // r8
  long long v19; // rsi
  const char *v20; // rax
  char *v21; // rax
  long long v22; // [rsp+0h] [rbp-118h] BYREF
  long long v23; // [rsp+8h] [rbp-110h]
  long long v24; // [rsp+10h] [rbp-108h] BYREF
  long long v25; // [rsp+18h] [rbp-100h]
  long long v26[22]; // [rsp+20h] [rbp-F8h] BYREF
  unsigned long long v27; // [rsp+D8h] [rbp-40h]

  v1 = *a1;
  v27 = __readfsqword(0x28u);
  v22 = 0;
  v23 = 0;
  v24 = 0;
  v25 = 0;
  if ( !v1 )
    return 0;
  v2 = a1;
  v3 = v26;
  do
    sub_A9E0(v3);
  while ( v5 != v3 );
  v7 = *(long long *)(v4 + 272);
  if ( v7 )
  {
    while ( *(long long *)(v7 + 272) )
    {
      v4 = v7;
      v7 = *(long long *)(v7 + 272);
    }
    v19 = *(unsigned int *)(v4 + 20);
    v18 = (long long *)(v4 + 272);
  }
  else
  {
    v18 = v2;
    v7 = v4;
    v19 = 3;
  }
  if ( *(int *)(v7 + 16) == 3 )
  {
    if ( (qword_3E660 & 9) == 0 )
    {
      v8 = *v2;
      goto LABEL_12;
    }
    goto LABEL_42;
  }
  sub_B1C0(v7, v19, v18);
  if ( (qword_3E660 & 9) != 0 )
  {
LABEL_42:
    __fprintf_chk(stderr, 1, "Normalized Eval Tree:\n", v5, v18, v6, v22, v23, v24, v25);
    v19 = *v2;
    sub_BAA0((long long)stderr, *v2, 0);
  }
  v8 = *v2;
  if ( *v2 )
  {
LABEL_12:
    v9 = *(int *)(v8 + 16);
    v10 = 0;
    if ( v9 == 3 )
      v10 = *(int *)(v8 + 20);
    v11 = 0;
    if ( v9 == 3 )
      goto LABEL_38;
    while ( 1 )
    {
      v12 = *(long long *)(v8 + 280);
      v13 = *(int *)(v12 + 16);
      if ( v13 == 2 )
        break;
      if ( v13 > 2 )
      {
        if ( v13 != 3 )
        {
          v21 = dcgettext(0, "oops -- invalid expression type!", 5);
          error(1, 0, v21);
        }
        *(char *)(v12 + 24) = opt_expr(v8 + 280, v19);
        v12 = *(long long *)(v8 + 280);
LABEL_34:
        if ( *(char *)(v12 + 24) )
        {
LABEL_40:
          v19 = 11;
          v11 = 1;
          sub_BFD0(v26, 11, &v24, &v22, v2);
        }
LABEL_35:
        v2 = (long long *)(v8 + 272);
        goto LABEL_36;
      }
      if ( v10 == 1 )
        goto LABEL_34;
      if ( *(char *)(v12 + 24) )
        goto LABEL_40;
      if ( sub_A950(*(long long *)(v8 + 280)) )
      {
        if ( (qword_3E660 & 8) != 0 )
        {
          __fprintf_chk(stderr, 1, "-O%d: promoting cheap predicate ", (unsigned short)word_3E688);
          sub_B5A0((long long)stderr, *(long long *)(v8 + 280));
          __fprintf_chk(stderr, 1, " into name_list\n");
        }
        v19 = v8;
        sub_A9F0(&v24, v8, v2);
      }
      else if ( v14 == sub_96F0 )
      {
        v19 = v8;
        sub_A9F0(&v22, v8, v2);
      }
      else
      {
        if ( (unsigned short)word_3E688 <= 1u )
          goto LABEL_35;
        v15 = *(unsigned int *)(v12 + 32);
        v19 = (unsigned int)(v15 - 1);
        if ( ((unsigned int)v19 > 1 || *(char *)(v12 + 26)) && word_3E688 == 2 )
          goto LABEL_35;
        if ( (qword_3E660 & 8) != 0 )
        {
          __fprintf_chk(stderr, 1, "-O%d: categorising predicate ", (unsigned short)word_3E688);
          sub_B5A0((long long)stderr, *(long long *)(v8 + 280));
          v20 = sub_AB90(*(int *)(*(long long *)(v8 + 280) + 32LL));
          __fprintf_chk(stderr, 1, " by cost (%s)\n", v20);
          v15 = *(unsigned int *)(*(long long *)(v8 + 280) + 32LL);
        }
        v19 = v8;
        sub_A9F0(&v26[2 * v15], v8, v2);
      }
LABEL_36:
      v8 = *v2;
      if ( !*v2 )
        goto LABEL_30;
      if ( *(int *)(v8 + 16) == 3 )
      {
LABEL_38:
        if ( *(int *)(v8 + 20) != v10 )
        {
          v19 = v10;
          sub_B1C0(v8, v10, v2);
          v8 = v17;
        }
      }
    }
    *(char *)(v12 + 24) = opt_expr(v12 + 280, v19);
    v12 = *(long long *)(v8 + 280);
    goto LABEL_34;
  }
  v11 = 0;
LABEL_30:
  sub_BFD0(v26, 11, &v24, &v22, v2);
  return v11;
}



// Function: consider_arm_swap @ 0xc450
long long consider_arm_swap(long long a1)
{
  const char *v1; // r8
  long long v2; // rcx
  unsigned int v3; // r12d
  long long v5; // rax
  long long v6; // rsi
  long long v7; // r8
  long long v8; // r8
  int v9; // eax
  int v10; // r9d
  float v11; // xmm2_4
  float v12; // xmm3_4

  v1 = "Not a binary operation";
  v2 = qword_3E660 & 8;
  if ( *(int *)(a1 + 16) != 3 )
    goto LABEL_2;
  v5 = *(long long *)(a1 + 272);
  if ( !v5 || !*(long long *)(a1 + 280) )
  {
    v1 = "Doesn't have two arms";
    goto LABEL_2;
  }
  v6 = *(long long *)(v5 + 280);
  if ( !v6 )
  {
    v1 = "Left arm has no child on RHS";
    goto LABEL_2;
  }
  if ( (unsigned char)sub_AA70(*(long long *)(v5 + 280)) )
  {
    v1 = "Left subtree has side-effects";
    goto LABEL_2;
  }
  v3 = sub_AA70(v7);
  if ( !(char)v3 )
  {
    sub_AAB0(v6);
    v9 = sub_AAB0(v8);
    if ( v10 < v9 )
    {
      v1 = "efficient as-is";
      goto LABEL_2;
    }
    if ( v10 != v9 )
      goto LABEL_22;
    v11 = *(float *)(v6 + 36);
    v12 = *((float *)v1 + 9);
    if ( v2 )
    {
      __fprintf_chk(stderr, 1, "Success rates: l=%f, r=%f\n", v11, v12);
      v2 = qword_3E660 & 8;
      if ( *(long long (**)(long long, long long, long long))a1 == sub_88C0 )
      {
        v1 = "Operation is OR; right success rate >= left";
        if ( v11 <= v12 )
          goto LABEL_2;
LABEL_22:
        if ( v2 )
        {
          __fprintf_chk(stderr, 1, "Performing arm swap on:\n", v2, v1);
          sub_BAA0((long long)stderr, a1, 0);
        }
LABEL_24:
        sub_AB00(a1);
        return 1;
      }
      v1 = "Not 'AND' or 'OR'";
      if ( *(long long (**)(long long, long long, long long))a1 != sub_8870 )
        goto LABEL_2;
    }
    else
    {
      if ( *(long long (**)(long long, long long, long long))a1 == sub_88C0 )
      {
        if ( v11 <= v12 )
          return v3;
        goto LABEL_24;
      }
      if ( *(long long (**)(long long, long long, long long))a1 != sub_8870 )
        return v3;
    }
    if ( v12 <= v11 )
    {
      v1 = "Operation is AND; right success rate <= left";
      goto LABEL_2;
    }
    goto LABEL_22;
  }
  v1 = "Right subtree has side-effects";
LABEL_2:
  v3 = 0;
  if ( !v2 )
    return v3;
  __fprintf_chk(stderr, 1, "Not an arm swap candidate (%s):\n", v1);
  sub_BAA0((long long)stderr, a1, 0);
  return 0;
}



// Function: do_arm_swaps @ 0xc6b0
long long do_arm_swaps(long long a1)
{
  if ( a1 )
  {
    do
    {
      while ( (unsigned char)sub_C450(a1) )
        ;
    }
    while ( (unsigned char)do_arm_swaps(*(long long *)(a1 + 272)) || (unsigned char)do_arm_swaps(*(long long *)(a1 + 280)) );
  }
  return 0;
}



// Function: build_expression_tree @ 0xc6f0
long long build_expression_tree(int a1, long long a2, int a3)
{
  long long v3; // rbp
  long long v4; // rax
  int v5; // edx
  long long v6; // r12
  long long v7; // rax
  long long v8; // r13
  long long v9; // rsi
  char *v10; // rcx
  long long v11; // rax
  long long v12; // rsi
  long long v13; // rdi
  long long v14; // rax
  long long v15; // rdi
  char v16; // al
  long long v17; // r12
  long long v18; // rax
  int *v19; // r14
  int v20; // r15d
  char *v21; // rax
  long long v22; // rdi
  long long v23; // rdi
  long long v24; // rax
  long long v25; // r12
  char *v26; // rax
  void *v27; // rdi
  char *v28; // rax
  long long v29; // rbp
  long long v30; // r12
  void *v32; // rdi
  long long v33; // rdi
  char *v34; // rax
  char *v35; // rax
  char *v36; // rax
  long long v37; // r12
  char *v38; // rax
  char *v39; // rax
  char *v40; // rax
  long long v41; // [rsp+8h] [rbp-70h]
  int v42[4]; // [rsp+1Ch] [rbp-5Ch] BYREF
  int v43; // [rsp+2Ch] [rbp-4Ch] BYREF
  void *ptr[9]; // [rsp+30h] [rbp-48h] BYREF

  v3 = a2;
  v42[0] = a1;
  ptr[1] = (void *)__readfsqword(0x28u);
  v4 = a3;
  v43 = a3;
  ::ptr = 0;
  qword_3E6C0 = a2 + 8LL * a3;
  if ( a3 < a1 )
  {
    while ( !(unsigned char)sub_DCA0(*(long long *)(a2 + 8 * v4), 1) )
    {
      ++qword_3E6B8;
      v5 = v43 + 1;
      v43 = v5;
      if ( v5 >= v42[0] )
        break;
      v4 = v5;
    }
  }
  v6 = sub_147F0("(");
  v41 = sub_147F0((char *)")");
  v7 = sub_147F0("print");
  v8 = v7;
  if ( !v6 )
    __assert_fail("entry_open != NULL", "tree.c", 0x4F6u, "build_expression_tree");
  if ( !v41 )
    __assert_fail("entry_close != NULL", "tree.c", 0x4F7u, "build_expression_tree");
  if ( !v7 )
    __assert_fail("entry_print != NULL", "tree.c", 0x4F8u, "build_expression_tree");
  sub_13EC0(v6, a2, v42);
  v9 = v42[0];
  v10 = ::ptr;
  *(long long *)(qword_3E6C8 + 8) = "(";
  v10[41] = 1;
  sub_147D0(v3, v9);
  sub_9F40(qword_3E6C8);
  v11 = v43;
  v12 = v42[0];
  if ( v43 >= v42[0] )
  {
LABEL_23:
    sub_147E0(v3, v12, qword_3E6C8, ::ptr);
    if ( *((long long *)::ptr + 33) )
    {
      if ( !(unsigned char)sub_DC60() )
        goto LABEL_31;
      sub_13690(v41, v3, v42);
      v22 = qword_3E6C8;
      *(long long *)(qword_3E6C8 + 8) = ")";
      *(char *)(v22 + 41) = 1;
      sub_9F40(v22);
      sub_12910(v8, v3, v42);
      v23 = qword_3E6C8;
      *(long long *)(qword_3E6C8 + 8) = "-print";
      *(char *)(v23 + 41) = 1;
      sub_9F40(v23);
      sub_9F40((long long)::ptr);
    }
    else
    {
      v32 = ::ptr;
      ptr[0] = ::ptr;
      qword_3E6C8 = 0;
      ::ptr = 0;
      free(v32);
      sub_12910(v8, v3, v42);
      v33 = qword_3E6C8;
      *(long long *)(qword_3E6C8 + 8) = "-print";
      sub_9F40(v33);
      sub_9F40((long long)::ptr);
    }
    if ( (qword_3E660 & 9) != 0 )
      goto LABEL_32;
    while ( 1 )
    {
      sub_14670(::ptr);
      sub_9F40((long long)::ptr);
      ptr[0] = ::ptr;
      LODWORD(v24) = sub_AE10((long long *)ptr, 0, 0);
      qword_3E6D0 = v24;
      v3 = v24;
      sub_B5F0(v24);
      if ( !ptr[0] )
        goto LABEL_34;
      if ( *(long long (**)())ptr[0] == sub_8C30 )
      {
        v39 = dcgettext(0, "you have too many ')'", 5);
        error(1, 0, v39);
      }
      v25 = *((long long *)ptr[0] + 1);
      if ( !v25 )
      {
        v34 = dcgettext(0, "unexpected extra predicate", 5);
        error(1, 0, v34);
        goto LABEL_40;
      }
      v26 = dcgettext(0, "unexpected extra predicate '%s'", 5);
      error(1, 0, v26, v25);
LABEL_31:
      v27 = (void *)*((long long *)::ptr + 33);
      ptr[0] = ::ptr;
      ::ptr = v27;
      sub_9F40((long long)v27);
      free(ptr[0]);
      if ( (qword_3E660 & 9) != 0 )
      {
LABEL_32:
        __fprintf_chk(stderr, 1, "Predicate List:\n");
        sub_9BC0((long long)stderr, (long long)::ptr);
      }
    }
  }
  while ( 1 )
  {
    v15 = *(long long *)(v3 + 8 * v11);
    byte_3E618 = 0;
    v16 = sub_DCA0(v15, 0);
    v17 = *(long long *)(v3 + 8LL * v43);
    if ( !v16 )
      goto LABEL_42;
    v18 = sub_147F0(*(char **)(v3 + 8LL * v43));
    v19 = (int *)v18;
    if ( !v18 )
    {
      v35 = dcgettext(0, "unknown predicate `%s'", 5);
      error(1, 0, v35, v17);
LABEL_42:
      v36 = dcgettext(0, "paths must precede expression: `%s'", 5);
      error(0, 0, v36, v17);
      if ( !access(*(const char **)(v3 + 8LL * v43), 0) )
      {
        v37 = *(long long *)(qword_3E6C8 + 8);
        v38 = dcgettext(0, "possible unquoted pattern after predicate `%s'?", 5);
        error(0, 0, v38, v37);
      }
      exit(1);
    }
    v20 = v43;
    if ( *(int *)v18 != 4 )
      v20 = ++v43;
    if ( !(*(unsigned char (**)(long long, long long, int *))(v18 + 16))(v18, v3, &v43) )
      break;
    v13 = qword_3E6C8;
    v14 = 0;
    *(long long *)(qword_3E6C8 + 8) = v17;
    if ( v43 != v20 )
      v14 = *(long long *)(v3 + 8LL * v20);
    *(long long *)(v13 + 48) = v14;
    sub_9F40(v13);
    sub_9F40((long long)::ptr);
    v11 = v43;
    v12 = v42[0];
    if ( v43 >= v42[0] )
      goto LABEL_23;
  }
  v8 = *(long long *)(v3 + 8LL * v43);
  if ( !v8 )
  {
    v40 = dcgettext(0, "missing argument to `%s'", 5);
    error(1, 0, v40, v17);
    return sub_CDF0();
  }
  if ( *v19 != 4 || v43 != v20 )
  {
    v21 = dcgettext(0, "invalid argument `%s' to `%s'", 5);
    v12 = 0;
    error(1, 0, v21, v8, v17);
    goto LABEL_23;
  }
  v28 = dcgettext(0, "invalid predicate `%s'", 5);
  error(1, 0, v28, v17);
LABEL_34:
  if ( (qword_3E660 & 9) != 0 )
  {
LABEL_40:
    __fprintf_chk(stderr, 1, "Eval Tree:\n");
    sub_BAA0((long long)stderr, qword_3E6D0, 0);
    v3 = qword_3E6D0;
  }
  sub_B4D0(v3);
  sub_C030(&qword_3E6D0);
  v29 = qword_3E6D0;
  sub_AC90(qword_3E6D0, 1);
  sub_C6B0(v29);
  v30 = qword_3E6D0;
  sub_AC90(qword_3E6D0, 1);
  if ( (qword_3E660 & 9) != 0 )
  {
    __fprintf_chk(stderr, 1, "Optimized Eval Tree:\n");
    sub_BAA0((long long)stderr, qword_3E6D0, 0);
    __fprintf_chk(stderr, 1, "Optimized command line:\n");
    sub_9C70((long long)stderr, qword_3E6D0);
    __fprintf_chk(stderr, 1, "\n");
    return qword_3E6D0;
  }
  return v30;
}



// Function: traverse_tree @ 0xcdf0
long long traverse_tree(long long a1, long long (*a2)(long long))
{
  long long v3; // rdi
  long long result; // rax

  do
  {
    v3 = *(long long *)(a1 + 272);
    if ( v3 )
      traverse_tree(v3, a2);
    result = a2(a1);
    a1 = *(long long *)(a1 + 280);
  }
  while ( a1 );
  return result;
}



// Function: undangle_file_pointers @ 0xce30
long long undangle_file_pointers(
        long long (**a1)(const char *a1, long long a2, long long a3),
        long long a2,
        long long a3,
        int a4)
{
  unsigned int v4; // eax
  long long result; // rax

  LOBYTE(v4) = *a1 == sub_9050;
  LOBYTE(a4) = (char *)*a1 == (char *)sub_FAD0;
  result = a4 | v4;
  LOBYTE(result) = ((char *)*a1 == (char *)sub_9010) | result;
  if ( (char)result || *a1 == sub_9090 )
    a1[8] = 0;
  return result;
}



// Function: show_valid_debug_options @ 0xce70
long long show_valid_debug_options(int a1)
{
  FILE *v1; // rbp
  char *v2; // rax
  char **v3; // rbx
  const char *v4; // r8
  long long v5; // rax
  long long result; // rax
  long long v7; // rbp
  const char *v8; // rcx
  const char *v9; // r8

  v1 = stdout;
  v2 = dcgettext(0, "Valid arguments for -D:\n", 5);
  fputs(v2, v1);
  v3 = &off_3C158;
  if ( !a1 )
  {
    v4 = "exec";
    v5 = 0;
    while ( 1 )
    {
      v7 = v5 + 1;
      if ( v5 )
      {
        result = __fprintf_chk(stdout, 1, "%s%s", ", ", v4);
        if ( v7 == 9 )
          return result;
      }
      else
      {
        __fprintf_chk(stdout, 1, "%s%s", "", v4);
      }
      v4 = *v3;
      v5 = v7;
      v3 += 3;
    }
  }
  v8 = "exec";
  v9 = "Show diagnostic information relating to -exec, -execdir, -ok and -okdir";
  while ( 1 )
  {
    result = __fprintf_chk(stdout, 1, "%-10s %s\n", v8, v9);
    if ( algn_3C218 == (char *)v3 )
      break;
    v9 = v3[2];
    v8 = *v3;
    v3 += 3;
  }
  return result;
}



// Function: do_exec @ 0xcf70
void do_exec(long long a1)
{
  long long v2; // rdx
  void *v3; // rdi

  sub_154F0(*(long long *)(a1 + 192), sub_D060, a1);
  v3 = *(void **)(a1 + 192);
  if ( v3 != qword_3E5C0 )
  {
    sub_25280(v3, sub_D060, v2);
    free(*(void **)(a1 + 192));
    *(long long *)(a1 + 192) = 0;
  }
}



// Function: do_complete_pending_execdirs @ 0xcfc0
void do_complete_pending_execdirs(long long a1)
{
  long long v1; // rbx

  if ( a1 )
  {
    v1 = a1;
    do
    {
      while ( 1 )
      {
        if ( !byte_3E608 )
          __assert_fail("state.execdirs_outstanding", "util.c", 0x18Cu, "do_complete_pending_execdirs");
        do_complete_pending_execdirs(*(long long *)(v1 + 272));
        if ( (*(long long (**)(long long, long long, long long))v1 == sub_9530 || *(long long (**)())v1 == sub_8FF0)
          && *(char *)(v1 + 56)
          && *(int *)(v1 + 208) )
        {
          break;
        }
        v1 = *(long long *)(v1 + 280);
        if ( !v1 )
          return;
      }
      sub_CF70(v1 + 56);
      v1 = *(long long *)(v1 + 280);
    }
    while ( v1 );
  }
}



// Function: exec_cb @ 0xd060
long long exec_cb(long long a1)
{
  sub_14E90(a1 + 8, a1 + 96);
  return 0;
}



// Function: complete_pending_execs @ 0xd080
long long complete_pending_execs(long long a1)
{
  long long v1; // rbx
  long long result; // rax

  if ( a1 )
  {
    v1 = a1;
    do
    {
      result = complete_pending_execs(*(long long *)(v1 + 272));
      if ( *(long long (**)(char *))v1 == sub_8FE0 && *(char *)(v1 + 56) )
      {
        result = *(unsigned int *)(v1 + 208);
        if ( (int)result )
          result = sub_14E90(v1 + 64, v1 + 152);
      }
      v1 = *(long long *)(v1 + 280);
    }
    while ( v1 );
  }
  return result;
}



// Function: cleanup_initial_cwd @ 0xd0f0
void cleanup_initial_cwd(long long a1, long long a2)
{
  long long v2; // rdx
  const char *v3; // rbx
  const char *v4; // r13
  char *v5; // r12
  int *v6; // rax

  if ( (unsigned int)sub_25250(qword_3E5C0) )
  {
    v3 = "";
    v4 = "";
    if ( *(int *)qword_3E5C0 < 0 )
    {
      v3 = (const char *)*((long long *)qword_3E5C0 + 1);
      v4 = ": ";
      if ( !v3 )
      {
        v3 = "";
        v4 = "";
      }
    }
    v5 = dcgettext(0, "Failed to restore initial working directory%s%s", 5);
    v6 = __errno_location();
    error(0, *v6, v5, v4, v3);
    _exit(1);
  }
  sub_25280(qword_3E5C0, a2, v2);
  free(qword_3E5C0);
  qword_3E5C0 = 0;
}



// Function: fallback_stat @ 0xd1a0
int fallback_stat(char *file, struct stat *buf, int a3)
{
  int v4; // eax

  v4 = *__errno_location();
  if ( v4 != 2 && v4 != 20 )
    return a3;
  if ( (qword_3E660 & 2) != 0 )
    __fprintf_chk(stderr, 1, "fallback_stat(): stat(%s) failed; falling back on lstat()\n", file);
  return fstatat(dword_3E5F8, file, buf, 256);
}



// Function: process_optimisation_option @ 0xd220
unsigned long long process_optimisation_option(char *nptr)
{
  char *v1; // r12
  int *v2; // rbx
  int *v3; // rax
  int v4; // ebp
  unsigned long long v5; // r13
  char *v7; // rax
  char *v8; // rax
  char *v9; // rax
  char *v10; // rax
  char *v11; // rax
  char *v12; // rax
  char *v13; // [rsp+0h] [rbp-38h] BYREF
  unsigned long long v14; // [rsp+8h] [rbp-30h]

  v2 = (int *)(unsigned char)*nptr;
  v14 = __readfsqword(0x28u);
  if ( !(char)v2 )
    goto LABEL_15;
  v1 = nptr;
  if ( ((*__ctype_b_loc())[(long long)v2] & 0x800) == 0 )
  {
    v9 = dcgettext(0, "Please specify a decimal number immediately after -O", 5);
    error(1, 0, v9);
LABEL_15:
    v10 = dcgettext(0, "The -O option must be immediately followed by a decimal integer", 5);
    error(1, 0, v10);
LABEL_16:
    v11 = dcgettext(0, "Please specify a decimal number immediately after -O", 5);
    error(1, 0, v11);
    goto LABEL_17;
  }
  v3 = __errno_location();
  v4 = *v3;
  *v3 = 0;
  v2 = v3;
  v5 = strtoul(nptr, &v13, 10);
  if ( !v5 )
  {
    if ( v13 != nptr )
    {
      if ( !*v13 )
      {
LABEL_6:
        word_3E688 = v5;
        *v2 = v4;
        return v14 - __readfsqword(0x28u);
      }
LABEL_13:
      v8 = dcgettext(0, "Invalid optimisation level %s", 5);
      error(1, 0, v8, nptr);
    }
    goto LABEL_16;
  }
  if ( *v13 )
    goto LABEL_13;
  if ( v5 != -1 )
    goto LABEL_11;
  if ( !*v2 )
  {
    do
    {
      v7 = dcgettext(
             0,
             "Optimisation level %lu is too high.  If you want to find files very quickly, consider using GNU locate.",
             5);
      error(1, 0, v7, v5);
LABEL_11:
      ;
    }
    while ( v5 > 0xFFFF );
    goto LABEL_6;
  }
LABEL_17:
  v12 = dcgettext(0, "Invalid optimisation level %s", 5);
  error(1, *v2, v12, v1);
  return sub_D3D0();
}



// Function: now @ 0xd3d0
time_t now()
{
  time_t result; // rax
  struct timeval v1; // [rsp+0h] [rbp-28h] BYREF
  unsigned long long v2; // [rsp+18h] [rbp-10h]

  v2 = __readfsqword(0x28u);
  if ( !gettimeofday(&v1, 0) )
    return v1.tv_sec;
  result = time(0);
  if ( result == -1 )
    __assert_fail("t != (time_t)-1", "util.c", 0x3D2u, "now");
  return result;
}



// Function: insert_primary_withpred @ 0xd450
long long *insert_primary_withpred(long long a1, long long a2, long long a3)
{
  long long *result; // rax
  long long v4; // rdx

  result = (long long *)sub_B990((int *)a1, a3);
  v4 = *(long long *)(a1 + 8);
  *result = a2;
  result[1] = v4;
  result[7] = 0;
  result[2] = 1;
  return result;
}



// Function: insert_primary @ 0xd490
long long *insert_primary(long long a1, long long a2)
{
  long long v3; // rsi

  v3 = *(long long *)(a1 + 24);
  if ( !v3 )
    __assert_fail("entry->pred_func != NULL", "util.c", 0x7Bu, "insert_primary");
  return sub_D450(a1, v3, a2);
}



// Function: insert_primary_noarg @ 0xd4d0
long long *insert_primary_noarg(long long a1)
{
  return sub_D490(a1, 0);
}



// Function: usage @ 0xd4e0
void usage(int status)
{
  long long v1; // r12
  char *v2; // rax
  char *v3; // rax
  FILE *v4; // rbp
  char *v5; // rax
  FILE *v6; // rbp
  char *v7; // rax
  FILE *v8; // rbp
  char *v9; // rax
  FILE *v10; // rbp
  char *v11; // rax
  FILE *v12; // rbp
  char *v13; // rax
  FILE *v14; // rbp
  char *v15; // rax
  FILE *v16; // rbp
  char *v17; // rax
  FILE *v18; // rbp
  char *v19; // rax
  FILE *v20; // rbp
  char *v21; // rax
  FILE *v22; // rbp
  char *v23; // rax

  v1 = qword_3EED0;
  if ( status )
  {
    v2 = dcgettext(0, "Try '%s --help' for more information.\n", 5);
    __fprintf_chk(stderr, 1, v2, v1);
    exit(status);
  }
  v3 = dcgettext(0, "Usage: %s [-H] [-L] [-P] [-Olevel] [-D debugopts] [path...] [expression]\n", 5);
  __fprintf_chk(stdout, 1, v3, v1);
  v4 = stdout;
  v5 = dcgettext(
         0,
         "\n"
         "Default path is the current directory; default expression is -print.\n"
         "Expression may consist of: operators, options, tests, and actions.\n",
         5);
  fputs(v5, v4);
  v6 = stdout;
  v7 = dcgettext(
         0,
         "\n"
         "Operators (decreasing precedence; -and is implicit where no others are given):\n"
         "      ( EXPR )   ! EXPR   -not EXPR   EXPR1 -a EXPR2   EXPR1 -and EXPR2\n"
         "      EXPR1 -o EXPR2   EXPR1 -or EXPR2   EXPR1 , EXPR2\n",
         5);
  fputs(v7, v6);
  v8 = stdout;
  v9 = dcgettext(0, "\nPositional options (always true):\n      -daystart -follow -nowarn -regextype -warn\n", 5);
  fputs(v9, v8);
  v10 = stdout;
  v11 = dcgettext(
          0,
          "\n"
          "Normal options (always true, specified before other expressions):\n"
          "      -depth -files0-from FILE -maxdepth LEVELS -mindepth LEVELS\n"
          "       -mount -noleaf -xdev -ignore_readdir_race -noignore_readdir_race\n",
          5);
  fputs(v11, v10);
  v12 = stdout;
  v13 = dcgettext(
          0,
          "\n"
          "Tests (N can be +N or -N or N):\n"
          "      -amin N -anewer FILE -atime N -cmin N -cnewer FILE -context CONTEXT\n"
          "      -ctime N -empty -false -fstype TYPE -gid N -group NAME -ilname PATTERN\n"
          "      -iname PATTERN -inum N -iwholename PATTERN -iregex PATTERN\n"
          "      -links N -lname PATTERN -mmin N -mtime N -name PATTERN -newer FILE\n"
          "      -nouser -nogroup -path PATTERN -perm [-/]MODE -regex PATTERN\n"
          "      -readable -writable -executable\n"
          "      -wholename PATTERN -size N[bcwkMG] -true -type [bcdpflsD] -uid N\n"
          "      -used N -user NAME -xtype [bcdpfls]\n",
          5);
  fputs(v13, v12);
  v14 = stdout;
  v15 = dcgettext(
          0,
          "\n"
          "Actions:\n"
          "      -delete -print0 -printf FORMAT -fprintf FILE FORMAT -print \n"
          "      -fprint0 FILE -fprint FILE -ls -fls FILE -prune -quit\n"
          "      -exec COMMAND ; -exec COMMAND {} + -ok COMMAND ;\n"
          "      -execdir COMMAND ; -execdir COMMAND {} + -okdir COMMAND ;\n",
          5);
  fputs(v15, v14);
  v16 = stdout;
  v17 = dcgettext(0, "\nOther common options:\n", 5);
  fputs(v17, v16);
  v18 = stdout;
  v19 = dcgettext(0, "      --help                   display this help and exit\n", 5);
  fputs(v19, v18);
  v20 = stdout;
  v21 = dcgettext(0, "      --version                output version information and exit\n\n", 5);
  fputs(v21, v20);
  sub_CE70(0);
  v22 = stdout;
  v23 = dcgettext(0, "\nUse '-D help' for a description of the options, or see find(1)\n\n", 5);
  fputs(v23, v22);
  sub_17130(stdout, qword_3EED0);
  exit(0);
}



// Function: process_debug_options @ 0xd6f0
unsigned long long process_debug_options(char *a1)
{
  char *v1; // rax
  char *v2; // r14
  const char **v3; // r13
  const char *v4; // rdi
  long long v5; // r15
  long long v6; // r14
  char *v7; // rax
  char *v9; // rax
  char *save_ptr; // [rsp+18h] [rbp-50h] BYREF
  char delim[2]; // [rsp+26h] [rbp-42h] BYREF
  unsigned long long v13; // [rsp+28h] [rbp-40h]

  v13 = __readfsqword(0x28u);
  save_ptr = 0;
  strcpy(delim, ",");
  v1 = strtok_r(a1, delim, &save_ptr);
  if ( !v1 )
  {
    v9 = dcgettext(0, "Empty argument to the -D option.", 5);
    error(0, 0, v9);
    sub_D4E0(1);
  }
  v2 = v1;
  do
  {
    v3 = (const char **)&off_3C158;
    v4 = "exec";
    v5 = 0;
    while ( strcmp(v4, v2) )
    {
      if ( ++v5 == 9 )
      {
        v6 = sub_24E50(0, (unsigned int)dword_3E68C, a1);
        v7 = dcgettext(0, "Ignoring unrecognised debug flag %s", 5);
        error(0, 0, v7, v6);
        goto LABEL_8;
      }
      v4 = *v3;
      v3 += 3;
    }
    qword_3E660 = *((int *)&off_3C140 + 6 * v5 + 2) | (unsigned long long)qword_3E660;
LABEL_8:
    v2 = strtok_r(0, delim, &save_ptr);
  }
  while ( v2 );
  if ( (qword_3E660 & 0x10) != 0 )
  {
    sub_CE70(1);
    exit(0);
  }
  return v13 - __readfsqword(0x28u);
}



// Function: set_stat_placeholders @ 0xd860
void set_stat_placeholders()
{
  ;
}



// Function: check_nofollow @ 0xd870
long long check_nofollow()
{
  unsigned int v0; // r12d
  __m128i v1; // xmm1
  float v2; // ebx
  struct utsname _0; // [rsp+0h] [rbp+0h] BYREF
  unsigned long long vars188; // [rsp+188h] [rbp+188h]

  v0 = 1;
  vars188 = __readfsqword(0x28u);
  if ( !uname(&_0) )
  {
    v1 = 0;
    *(float *)v1.m128i_i32 = atof(_0.release);
    v2 = COERCE_FLOAT(_mm_cvtsi128_si32(v1));
    if ( !strcmp("Linux", _0.sysname) )
    {
      LOBYTE(v0) = v2 >= 2.2;
    }
    else if ( !strcmp("FreeBSD", _0.sysname) )
    {
      LOBYTE(v0) = v2 >= 3.0999999;
    }
  }
  return v0;
}



// Function: complete_pending_execdirs @ 0xd940
void complete_pending_execdirs()
{
  long long v0; // rax

  if ( byte_3E608 )
  {
    v0 = sub_B5E0();
    sub_CFC0(v0);
    byte_3E608 = 0;
  }
}



// Function: record_initial_cwd @ 0xd970
long long record_initial_cwd()
{
  long long result; // rax
  void *v1; // rax
  const char *v2; // rbx
  const char *v3; // r13
  char *v4; // r12
  int *v5; // rax

  qword_3E5C0 = (void *)sub_26990(16);
  result = sub_251F0(qword_3E5C0);
  if ( (int)result )
  {
    v1 = qword_3E5C0;
    v2 = "";
    v3 = "";
    if ( *(int *)qword_3E5C0 < 0 )
      goto LABEL_4;
    while ( 1 )
    {
      v4 = dcgettext(0, "Failed to save initial working directory%s%s", 5);
      v5 = __errno_location();
      error(1, *v5, v4, v3, v2);
LABEL_4:
      v2 = (const char *)*((long long *)v1 + 1);
      v3 = ": ";
      if ( !v2 )
      {
        v2 = "";
        v3 = "";
      }
    }
  }
  return result;
}



// Function: optionh_stat @ 0xda00
int optionh_stat(char *file, struct stat *buf)
{
  int v2; // eax

  if ( dword_3E5F8 != -100 && dword_3E5F8 < 0 )
    __assert_fail("state.cwd_dir_fd >= 0", "util.c", 0x25Du, "optionh_stat");
  if ( dword_3E5E0 )
    return fstatat(dword_3E5F8, file, buf, 256);
  v2 = fstatat(dword_3E5F8, file, buf, 0);
  if ( v2 )
    return sub_D1A0(file, buf, v2);
  else
    return 0;
}



// Function: optionl_stat @ 0xdaa0
int optionl_stat(char *file, struct stat *buf)
{
  int v2; // eax

  if ( dword_3E5F8 != -100 && dword_3E5F8 < 0 )
    __assert_fail("state.cwd_dir_fd >= 0", "util.c", 0x27Cu, "optionl_stat");
  v2 = fstatat(dword_3E5F8, file, buf, 0);
  if ( v2 )
    return sub_D1A0(file, buf, v2);
  else
    return 0;
}



// Function: optionp_stat @ 0xdb20
int optionp_stat(char *file, struct stat *buf)
{
  if ( dword_3E5F8 < 0 && dword_3E5F8 != -100 )
    __assert_fail("(state.cwd_dir_fd >= 0) || (state.cwd_dir_fd==AT_FDCWD)", "util.c", 0x28Du, "optionp_stat");
  return fstatat(dword_3E5F8, file, buf, 256);
}



// Function: debug_stat @ 0xdb70
int debug_stat(char *file, struct stat *buf)
{
  ++qword_3E6E0;
  __fprintf_chk(stderr, 1, "debug_stat (%s)\n", file);
  if ( dword_3E668 == 1 )
    return sub_DAA0(file, buf);
  if ( dword_3E668 == 2 )
    return sub_DA00(file, buf);
  if ( dword_3E668 )
    __assert_fail("0", "util.c", 0x2A5u, "debug_stat");
  return sub_DB20(file, buf);
}



// Function: following_links @ 0xdc30
bool following_links()
{
  bool result; // al

  result = 1;
  if ( dword_3E668 != 1 )
  {
    result = 0;
    if ( dword_3E668 == 2 )
      return dword_3E5E0 == 0;
  }
  return result;
}



// Function: default_prints @ 0xdc60
long long default_prints(long long a1)
{
  while ( 1 )
  {
    if ( !a1 )
      return 1;
    if ( *(char *)(a1 + 25) )
      break;
    a1 = *(long long *)(a1 + 264);
  }
  return 0;
}



// Function: looks_like_expression @ 0xdca0
char looks_like_expression(char *a1, char a2)
{
  char result; // al

  switch ( *a1 )
  {
    case '!':
    case '(':
      result = a1[1] == 0;
      break;
    case ')':
    case ',':
      result = 0;
      if ( !a1[1] )
        result = a2 ^ 1;
      break;
    case '-':
      result = a1[1] != 0;
      break;
    default:
      result = 0;
      break;
  }
  return result;
}



// Function: process_leading_options @ 0xdd00
long long process_leading_options(int a1, long long a2)
{
  int v2; // r12d
  long long v3; // rbx
  long long v4; // r13
  char *v6; // rax

  if ( a1 <= 1 )
    return 1;
  v2 = 1;
  while ( 1 )
  {
    while ( 1 )
    {
      v3 = *(long long *)(a2 + 8LL * v2);
      v4 = 8LL * v2;
      if ( strcmp("-H", (const char *)v3) )
        break;
      ++v2;
      sub_146F0(2);
      if ( a1 <= v2 )
        return (unsigned int)v2;
    }
    if ( !strcmp("-L", (const char *)v3) )
    {
      sub_146F0(1);
      goto LABEL_9;
    }
    if ( !strcmp("-P", (const char *)v3) )
    {
      sub_146F0(0);
      goto LABEL_9;
    }
    if ( !strcmp("--", (const char *)v3) )
      return (unsigned int)++v2;
    if ( !strcmp("-D", (const char *)v3) )
    {
      if ( ++v2 >= a1 )
      {
        v6 = dcgettext(0, "Missing argument after the -D option.", 5);
        error(0, 0, v6);
        sub_D4E0(1);
      }
      sub_D6F0(*(char **)(a2 + v4 + 8));
    }
    else
    {
      if ( strncmp("-O", (const char *)v3, 2u) )
        return (unsigned int)v2;
      sub_D220((char *)(v3 + 2));
    }
LABEL_9:
    if ( a1 <= ++v2 )
      return (unsigned int)v2;
  }
}



// Function: set_option_defaults @ 0xde90
long long set_option_defaults(long long a1)
{
  char v1; // al
  int v2; // eax
  time_t v3; // rax
  long long v4; // rdx
  bool v5; // cf
  long long result; // rax
  char *v7; // rax

  *(char *)(a1 + 17) = getenv("POSIXLY_CORRECT") != 0;
  v1 = sub_D870();
  *(int *)(a1 + 92) = 0;
  *(char *)(a1 + 88) = v1;
  v2 = isatty(0);
  *(char *)(a1 + 15) = 0;
  *(char *)(a1 + 16) = v2 != 0;
  if ( *(char *)(a1 + 17) )
    *(char *)(a1 + 16) = 0;
  *(long long *)(a1 + 4) = -1;
  *(short *)a1 = 1;
  v3 = sub_D3D0();
  *(char *)(a1 + 56) = 0;
  *(long long *)(a1 + 24) = v3;
  *(long long *)(a1 + 32) = v4;
  *(long long *)(a1 + 48) = v4;
  v5 = *(char *)(a1 + 17) == 0;
  *(long long *)(a1 + 40) = v3 - 86400;
  *(short *)(a1 + 13) = 0;
  *(long long *)(a1 + 64) = 0;
  *(int *)(a1 + 60) = v5 ? 1024 : 512;
  *(short *)(a1 + 104) = 2;
  if ( getenv("FIND_BLOCK_SIZE") )
  {
    v7 = dcgettext(
           0,
           "The environment variable FIND_BLOCK_SIZE is not supported, the only thing that affects the block size is the "
           "POSIXLY_CORRECT environment variable",
           5);
    error(1, 0, v7);
    return sub_DF90(1);
  }
  else
  {
    *(char *)(a1 + 12) = 0;
    result = sub_146F0(0);
    *(int *)(a1 + 108) = 8;
    *(long long *)(a1 + 112) = 0;
    *(char *)(a1 + 120) = 0;
  }
  return result;
}



// Function: is_exec_in_local_dir @ 0xdf90
long long is_exec_in_local_dir(long long (*a1)(long long a1, long long a2, long long a3), long long a2, int a3)
{
  unsigned int v3; // eax

  LOBYTE(v3) = a1 == sub_9530;
  LOBYTE(a3) = (char *)a1 == (char *)sub_8FF0;
  return a3 | v3;
}



// Function: safely_quote_err_filename @ 0xdfb0
long long safely_quote_err_filename(long long a1, long long a2)
{
  return sub_24E50(a1, (unsigned int)dword_3E68C, a2);
}



// Function: report_file_err @ 0xdfd0
void report_file_err(int status, int errnum, char a3, long long a4)
{
  const char *v4; // rax
  const char *v5; // rax

  if ( a3 )
  {
    if ( !byte_3E618 )
    {
      v5 = (const char *)sub_DFB0(0, a4);
      error(status, errnum, "%s", v5);
      ::status = 1;
    }
    byte_3E618 = 1;
  }
  else
  {
    v4 = (const char *)sub_DFB0(0, a4);
    error(status, errnum, "%s", v4);
    ::status = 1;
  }
}



// Function: nonfatal_target_file_error @ 0xe060
void nonfatal_target_file_error(int errnum, long long a2)
{
  sub_DFD0(0, errnum, 1, a2);
}



// Function: get_statinfo @ 0xe080
long long get_statinfo(long long a1, long long a2, long long a3)
{
  int v5; // eax
  long long v6; // r12
  char *v7; // rax
  long long result; // rax
  int v9; // edi

  if ( (char)word_3E5E4 )
  {
    v5 = *(int *)(a3 + 24);
LABEL_6:
    dword_3E5E8 = v5;
    result = 0;
    word_3E5E4 = 257;
    return result;
  }
  if ( !(unsigned int)qword_3E670(a2, a3) )
  {
    v5 = *(int *)(a3 + 24);
    if ( !v5 )
    {
      v6 = sub_24E50(0, (unsigned int)dword_3E68C, a2);
      v7 = dcgettext(0, "WARNING: file %s appears to have mode 0000", 5);
      error(0, 0, v7, v6);
      v5 = *(int *)(a3 + 24);
      status = 1;
    }
    goto LABEL_6;
  }
  v9 = *__errno_location();
  if ( !byte_3E62E || (result = 0xFFFFFFFFLL, v9 != 2) )
  {
    sub_E060(v9, a1);
    return 0xFFFFFFFFLL;
  }
  return result;
}



// Function: digest_mode @ 0xe150
long long digest_mode(int *a1, long long a2, long long a3, long long a4, unsigned int a5)
{
  int v9; // ecx
  unsigned int v11; // r12d
  unsigned int v12; // eax
  int v13; // eax

  v9 = *a1;
  if ( *a1 )
  {
    if ( (*a1 & 0xF000) != 0xA000 || (LOBYTE(v12) = sub_DC30(), v11 = v12, !(char)v12) )
    {
      *(int *)(a4 + 24) = v9;
      HIBYTE(word_3E5E4) = 1;
      dword_3E5E8 = v9;
      return 1;
    }
    if ( !(unsigned int)sub_E080(a2, a3, a4) )
    {
      dword_3E5E8 = *(int *)(a4 + 24);
      *a1 = dword_3E5E8;
      HIBYTE(word_3E5E4) = 1;
      return v11;
    }
    return 0;
  }
  if ( (char)a5 )
  {
    dword_3E5E8 = 0;
    v11 = a5;
    word_3E5E4 = 0;
    return v11;
  }
  if ( (unsigned int)sub_E080(a2, a3, a4) )
    return 0;
  v13 = *(int *)(a4 + 24);
  *a1 = v13;
  dword_3E5E8 = v13;
  HIBYTE(word_3E5E4) = 1;
  return 1;
}



// Function: get_info @ 0xe230
long long get_info(long long a1, long long a2, char *a3)
{
  long long result; // rax

  if ( a3[26] && !(char)word_3E5E4 )
    return (unsigned int)-((unsigned int)sub_E080(a1, (long long)src, a2) != 0);
  if ( a3[27] && !HIBYTE(word_3E5E4) )
    return (unsigned int)-((unsigned int)sub_E080(a1, (long long)src, a2) != 0);
  result = 0;
  if ( a3[28] )
  {
    if ( !*(long long *)(a2 + 8) || !HIBYTE(word_3E5E4) || (*(int *)(a2 + 24) & 0xF000) == 0x4000 )
      return (unsigned int)-((unsigned int)sub_E080(a1, (long long)src, a2) != 0);
  }
  return result;
}



// Function: apply_predicate @ 0xe2a0
long long apply_predicate(long long a1, long long a2, long long a3)
{
  long long result; // rax

  ++*(long long *)(a3 + 288);
  if ( (*(long long *)(a3 + 24) & 0xFFFFFF0000LL) != 0 && (unsigned int)sub_E230(a1, a2, (char *)a3) )
    return 0;
  result = (*(long long (**)(long long, long long, long long))a3)(a1, a2, a3);
  if ( !(char)result )
    return 0;
  ++*(long long *)(a3 + 296);
  return result;
}



// Function: fatal_target_file_error @ 0xe310
void fatal_target_file_error(int errnum, long long a2)
{
  sub_DFD0(1, errnum, 1, a2);
  abort();
}



// Function: nonfatal_nontarget_file_error @ 0xe340
void nonfatal_nontarget_file_error(int errnum, long long a2)
{
  sub_DFD0(0, errnum, 0, a2);
}



// Function: cleanup @ 0xe360
void cleanup(long long a1, long long a2)
{
  long long v2; // rax
  long long v3; // rbp
  long long v4; // rdi
  int *v5; // rax

  v2 = sub_B5E0();
  if ( v2 )
  {
    v3 = v2;
    sub_CDF0(v2, sub_D080);
    sub_D940();
    sub_E560(qword_3E610);
    sub_CDF0(v3, (long long (*)(long long))sub_CE30);
    sub_D0F0(v3, (long long)sub_CE30);
    if ( !(unsigned char)sub_15DD0() )
      goto LABEL_3;
  }
  else
  {
    v4 = qword_3E610;
    sub_E560(qword_3E610);
    sub_D0F0(v4, a2);
    if ( !(unsigned char)sub_15DD0() )
      goto LABEL_3;
  }
  sub_15DF0();
  sub_15DA0();
LABEL_3:
  if ( (unsigned int)sub_17BC0(stdout) == -1 )
  {
    v5 = __errno_location();
    sub_E340(*v5, (long long)"standard output");
  }
}



// Function: fatal_nontarget_file_error @ 0xe410
void fatal_nontarget_file_error(int errnum, long long a2)
{
  byte_3E618 = 0;
  sub_DFD0(1, errnum, 0, a2);
  abort();
}



// Function: entry_comparator @ 0xe440
bool entry_comparator(long long *a1, long long *a2)
{
  bool result; // al

  result = 0;
  if ( a1[1] == a2[1] )
    return *a1 == *a2;
  return result;
}



// Function: entry_hashfunc @ 0xe470
unsigned long long entry_hashfunc(long long *a1, unsigned long long a2)
{
  return (a1[1] ^ *a1) % a2;
}



// Function: entry_free @ 0xe490
void entry_free(long long *ptr)
{
  FILE *v2; // rdi
  int *v3; // rax

  v2 = (FILE *)ptr[3];
  if ( v2 && (unsigned int)sub_17850(v2) )
  {
    v3 = __errno_location();
    sub_E410(*v3, ptr[2]);
  }
  free((void *)ptr[2]);
  free(ptr);
}



// Function: sharefile_init @ 0xe4d0
void **sharefile_init(char *s)
{
  void **v1; // r12
  char *v2; // rax
  long long v3; // rax
  void **v5; // rdi
  void **v6; // rdi

  v1 = (void **)malloc(0x10u);
  if ( v1 )
  {
    v2 = strdup(s);
    *v1 = v2;
    if ( v2 )
    {
      v3 = sub_1AC70(11, 0, sub_E470, sub_E440, sub_E490);
      v1[1] = (void *)v3;
      if ( !v3 )
      {
        free(*v1);
        v6 = v1;
        v1 = 0;
        free(v6);
      }
    }
    else
    {
      v5 = v1;
      v1 = 0;
      free(v5);
    }
  }
  return v1;
}



// Function: sharefile_destroy @ 0xe560
long long sharefile_destroy(void **a1)
{
  free(*a1);
  return sub_1AE30(a1[1]);
}



// Function: sharefile_fopen @ 0xe580
long long sharefile_fopen(long long *a1, const char *a2)
{
  __dev_t *v3; // rax
  __dev_t *v4; // rbp
  char *v5; // rax
  long long v6; // r12
  FILE *v7; // rax
  int v8; // eax
  int v9; // r12d
  long long v10; // rdi
  long long v11; // r12
  int *v13; // rax
  int v14; // r13d
  int *v15; // rbx
  struct stat v16; // [rsp+0h] [rbp-C8h] BYREF
  unsigned long long v17; // [rsp+98h] [rbp-30h]

  v17 = __readfsqword(0x28u);
  v3 = (__dev_t *)malloc(0x20u);
  if ( !v3 )
    return 0;
  v4 = v3;
  v5 = strdup(a2);
  v4[2] = (__dev_t)v5;
  v6 = (long long)v5;
  if ( v5 )
  {
    v7 = (FILE *)sub_17DA0(a2, *a1);
    v4[3] = (__dev_t)v7;
    v6 = (long long)v7;
    if ( v7 )
    {
      v8 = fileno(v7);
      v9 = v8;
      if ( v8 < 0 )
        __assert_fail("fd >= 0", "sharefile.c", 0xA6u, "sharefile_fopen");
      sub_174F0((unsigned int)v8, 1);
      if ( fstat(v9, &v16) < 0 )
      {
        v6 = 0;
        sub_E490(v4);
      }
      else
      {
        v10 = a1[1];
        *v4 = v16.st_dev;
        v4[1] = v16.st_ino;
        v11 = sub_1AA10(v10, v4);
        if ( v11 )
        {
          sub_E490(v4);
          return *(long long *)(v11 + 24);
        }
        else
        {
          v6 = sub_1B290(a1[1], v4);
          if ( v6 )
          {
            return v4[3];
          }
          else
          {
            v13 = __errno_location();
            v14 = *v13;
            v15 = v13;
            sub_E490(v4);
            *v15 = v14;
          }
        }
      }
    }
    else
    {
      sub_E490(v4);
    }
  }
  else
  {
    free(v4);
  }
  return v6;
}



// Function: is_octal_char @ 0xe720
bool is_octal_char(char a1)
{
  return (unsigned char)(a1 - 48) <= 7u;
}



// Function: parse_escape_char @ 0xe730
long long parse_escape_char(char a1)
{
  unsigned char v1; // di
  long long result; // rax

  v1 = a1 - 92;
  result = 0;
  if ( v1 <= 0x1Au )
    return (unsigned int)byte_2DA10[v1];
  return result;
}



// Function: mode_to_filetype @ 0xe750
char *mode_to_filetype(int a1)
{
  char *result; // rax

  result = "f";
  if ( a1 != 0x8000 )
  {
    result = (char *)"d";
    if ( a1 != 0x4000 )
    {
      result = (char *)"l";
      if ( a1 != 40960 )
      {
        result = "s";
        if ( a1 != 49152 )
        {
          result = "b";
          if ( a1 != 24576 )
          {
            result = (char *)"c";
            if ( a1 != 0x2000 )
            {
              result = "U";
              if ( a1 == 4096 )
                return (char *)"p";
            }
          }
        }
      }
    }
  }
  return result;
}



// Function: get_format_specifer_length @ 0xe7d0
long long get_format_specifer_length(char a1)
{
  char *v1; // r8
  long long result; // rax

  v1 = strchr("abcdDfFgGhHiklmMnpPsStuUyYZ%", a1);
  result = 1;
  if ( !v1 )
    return 2LL * (strchr("ABCT", a1) != 0);
  return result;
}



// Function: get_format_flags_length @ 0xe820
long long get_format_flags_length(long long a1)
{
  long long v1; // r12
  unsigned char v2; // bl
  const unsigned short *v3; // rdx

  v1 = 0;
  do
  {
    ++v1;
    v2 = *(char *)(a1 + v1);
  }
  while ( v2 && strchr("-+ #", (char)v2) );
  v3 = *__ctype_b_loc();
  while ( (v3[v2] & 0x800) != 0 )
  {
    ++v1;
    v2 = *(char *)(a1 + v1);
  }
  if ( v2 == 46 )
  {
    do
      ++v1;
    while ( (v3[*(unsigned char *)(a1 + v1)] & 0x800) != 0 );
  }
  return v1;
}



// Function: checked_fprintf @ 0xe8c0
unsigned long long checked_fprintf(long long a1, long long a2, ...)
{
  int *v3; // rax
  gcc_va_list va; // [rsp+0h] [rbp-D8h] BYREF
  unsigned long long v5; // [rsp+18h] [rbp-C0h]

  va_start(va, a2);
  v5 = __readfsqword(0x28u);
  if ( (int)((long long (*)(long long, long long, long long, __va_list_tag *))__vfprintf_chk)(
              *(long long *)(a1 + 8),
              1,
              a2,
              va) < 0 )
  {
    v3 = __errno_location();
    sub_E340(*v3, *(long long *)(a1 + 16));
  }
  return v5 - __readfsqword(0x28u);
}



// Function: checked_fflush @ 0xe9a0
void checked_fflush(long long a1)
{
  int *v1; // rax

  if ( (unsigned int)sub_17BC0(*(FILE **)(a1 + 8)) )
  {
    v1 = __errno_location();
    sub_E340(*v1, *(long long *)(a1 + 16));
  }
}



// Function: checked_print_quoted @ 0xe9d0
void checked_print_quoted(long long a1, const char *a2, long long a3)
{
  int *v3; // rax

  if ( (int)sub_16960(*(long long *)(a1 + 8), *(long long *)(a1 + 32), *(unsigned char *)(a1 + 24), a2, a3) < 0 )
  {
    v3 = __errno_location();
    sub_E340(*v3, *(long long *)(a1 + 16));
  }
}



// Function: do_time_format @ 0xeb50
char *do_time_format(const char *src, const __m128i *tp, const void *a3, size_t a4)
{
  size_t v5; // rax
  char *v6; // rax
  char *v7; // r12
  size_t v8; // rax
  int v9; // edx
  __m128i v10; // xmm0
  __m128i v11; // xmm1
  int v12; // eax
  __m128i v13; // xmm2
  char *v14; // rbp
  size_t v15; // rax
  size_t v16; // r15
  long long v17; // r13
  char *v18; // rax
  char *v19; // rbp
  char *v20; // rbx
  const unsigned short **v21; // r10
  long long v22; // rax
  size_t v23; // r8
  size_t v24; // rax
  char *dest; // [rsp+8h] [rbp-90h]
  long long v28; // [rsp+10h] [rbp-88h] BYREF
  long long v29; // [rsp+18h] [rbp-80h] BYREF
  struct tm tpa; // [rsp+20h] [rbp-78h] BYREF
  unsigned long long v31; // [rsp+58h] [rbp-40h]

  v31 = __readfsqword(0x28u);
  v5 = strlen(src);
  v6 = (char *)sub_26990(v5 + 2);
  *v6 = 95;
  v7 = v6;
  v8 = strlen(src);
  memcpy(v7 + 1, src, v8 + 1);
  v9 = tp->m128i_i32[0];
  v10 = _mm_loadu_si128(tp);
  v11 = _mm_loadu_si128(tp + 1);
  tpa.tm_zone = (const char *)tp[3].m128i_i64[0];
  v12 = v9 + 11;
  v13 = _mm_loadu_si128(tp + 2);
  if ( v9 >= 11 )
    v12 = v9 - 11;
  *(__m128i *)&tpa.tm_sec = v10;
  v14 = qword_3EB08;
  *(__m128i *)&tpa.tm_mon = v11;
  tpa.tm_sec = v12;
  *(__m128i *)&tpa.tm_isdst = v13;
  if ( !qword_3EB08 )
  {
    maxsize = 1;
    qword_3EB08 = (char *)sub_26990(1);
    v14 = qword_3EB08;
  }
  while ( 1 )
  {
    v15 = strftime(v14, maxsize, v7, (const struct tm *)tp);
    v16 = v15;
    if ( v15 )
    {
      if ( maxsize > v15 )
        break;
    }
    qword_3EB08 = (char *)sub_26AD0(v14, &maxsize, 1);
    v14 = qword_3EB08;
  }
  v17 = v15 + a4 + 1;
  v28 = 0;
  v29 = 0;
  v18 = (char *)sub_269E0(v14, v17);
  maxsize = v17;
  qword_3EB08 = v18;
  v19 = (char *)sub_26990(v17);
  strftime(v19, maxsize, v7, &tpa);
  v20 = qword_3EB08;
  if ( (unsigned char)sub_EA60(qword_3EB08, v19, &v28, &v29) )
  {
    if ( v29 == 2 )
    {
      v21 = __ctype_b_loc();
      v22 = v28 + 2;
      if ( ((*v21)[(unsigned char)v20[v28 + 2]] & 0x800) == 0 )
      {
        v23 = v16 - v22;
        v24 = a4 + v22;
        if ( v24 + v23 + 1 != v17 )
          __assert_fail("end_of_seconds + ns_size + suffix_len == final_len", "print.c", 0x225u, "do_time_format");
        dest = &v20[v28 + 2];
        memmove(&v20[v24], dest, v23 + 1);
        memcpy(dest, a3, a4);
      }
    }
  }
  free(v7);
  free(v19);
  return v20 + 1;
}



// Function: format_date @ 0xeda0
char *format_date(time_t a1, long long a2, int a3)
{
  long long v3; // r9
  unsigned long long v4; // rax
  long long v6; // rdx
  int v7; // ebp
  int v8; // ebx
  time_t v9; // rdi
  char *v10; // r12
  size_t v11; // rax
  size_t v12; // r13
  struct tm *v14; // rsi
  time_t v15; // [rsp+0h] [rbp-78h] BYREF
  long long v16; // [rsp+8h] [rbp-70h]
  char src[12]; // [rsp+14h] [rbp-64h] BYREF
  char s[40]; // [rsp+20h] [rbp-58h] BYREF
  unsigned long long v19; // [rsp+48h] [rbp-30h]

  v3 = a2;
  v15 = a1;
  v16 = a2;
  v19 = __readfsqword(0x28u);
  if ( a3 == 43 )
  {
    strcpy(src, "%Y-%m-%d+%T");
    v7 = 1;
    v8 = __snprintf_chk(s, 32, 1, 32, ".%09ld0", a2);
    goto LABEL_17;
  }
  v4 = (unsigned int)(a3 - 64);
  src[0] = 37;
  *(short *)&src[1] = (unsigned char)a3;
  if ( (unsigned int)v4 <= 0x18 )
  {
    v6 = 18350081;
    if ( _bittest64(&v6, v4) )
    {
      v7 = 1;
      v8 = __snprintf_chk(s, 32, 1, 32, ".%09ld0", a2);
      if ( a3 == 64 )
        goto LABEL_5;
LABEL_17:
      v14 = localtime(&v15);
      if ( v14 )
      {
        v10 = sub_EB50(src, (const __m128i *)v14, s, v8);
        if ( v10 )
          return v10;
      }
      goto LABEL_5;
    }
  }
  s[0] = 0;
  v7 = 0;
  v8 = 0;
  if ( a3 != 64 )
    goto LABEL_17;
LABEL_5:
  v9 = -v15;
  if ( v15 > 0 )
    v9 = v15;
  v10 = (char *)sub_1B800(v9, &unk_3EB21, 0, 1, 1, v3);
  if ( v10 <= (char *)&unk_3EB20 )
    __assert_fail("p > buf", "print.c", 0x2B9u, "format_date");
  if ( v10 >= (char *)&unk_3EE37 )
    __assert_fail("p < (buf + (sizeof buf))", "print.c", 0x2BAu, "format_date");
  if ( v15 < 0 )
    *--v10 = 45;
  if ( v7 )
  {
    v11 = strlen(v10) + v10 - (char *)&unk_3EB20;
    if ( v11 > 0x316 )
      __assert_fail("sizeof buf > used", "print.c", 0x2C7u, "format_date");
    v12 = 790 - v11;
    if ( strlen(s) >= 790 - v11 )
    {
      error(0, 0, "charsprinted=%ld but remaining=%lu: ns_buf=%s", v8, v12, s);
      if ( strlen(s) >= v12 )
        __assert_fail("strlen (ns_buf) < remaining", "print.c", 0x2D0u, "format_date");
    }
    strcat(v10, s);
  }
  return v10;
}



// Function: ctime_format @ 0xf050
char *ctime_format(time_t a1, long long a2)
{
  struct tm *v2; // rax
  long long tm_wday; // rdx
  long long tm_mon; // rcx
  int tm_hour; // esi
  int tm_min; // r8d
  time_t v8; // [rsp+0h] [rbp-18h] BYREF
  long long v9; // [rsp+8h] [rbp-10h]

  v8 = a1;
  v9 = a2;
  v2 = localtime(&v8);
  if ( !v2 )
    return sub_EDA0(v8, v9, 64);
  tm_wday = v2->tm_wday;
  if ( (int)tm_wday < 0 )
    __assert_fail("ptm->tm_wday >= 0", "print.c", 0x2EDu, "ctime_format");
  if ( (int)tm_wday > 6 )
    __assert_fail("ptm->tm_wday < 7", "print.c", 0x2EEu, "ctime_format");
  tm_mon = v2->tm_mon;
  if ( (int)tm_mon < 0 )
    __assert_fail("ptm->tm_mon >= 0", "print.c", 0x2EFu, "ctime_format");
  if ( (int)tm_mon > 11 )
    __assert_fail("ptm->tm_mon < 12", "print.c", 0x2F0u, "ctime_format");
  tm_hour = v2->tm_hour;
  if ( tm_hour < 0 )
    __assert_fail("ptm->tm_hour >= 0", "print.c", 0x2F1u, "ctime_format");
  if ( tm_hour > 23 )
    __assert_fail("ptm->tm_hour < 24", "print.c", 0x2F2u, "ctime_format");
  tm_min = v2->tm_min;
  if ( tm_min > 59 )
    __assert_fail("ptm->tm_min < 60", "print.c", 0x2F3u, "ctime_format");
  if ( v2->tm_sec > 61 )
    __assert_fail("ptm->tm_sec <= 61", "print.c", 0x2F4u, "ctime_format");
  if ( (int)__snprintf_chk(
              &unk_3E700,
              1024,
              1,
              1024,
              "%3s %3s %2d %02d:%02d:%02d.%09ld0 %04d",
              off_3C280[tm_wday],
              off_3C220[tm_mon],
              v2->tm_mday,
              tm_hour,
              tm_min,
              v2->tm_sec,
              v9,
              v2->tm_year + 1900) > 1023 )
    __assert_fail("nout < TIME_BUF_LEN", "print.c", 0x302u, "ctime_format");
  return (char *)&unk_3E700;
}



// Function: do_fprintf @ 0xf310
void do_fprintf(long long a1, int *a2, char *a3, long long a4, long long a5, long long a6)
{
  int v7; // eax
  char *v8; // r12
  char *v10; // rax
  long long *v11; // rax
  const char *v12; // rsi
  int v13; // edi
  char *v14; // rdx
  long long v15; // rsi
  char *v16; // rax
  long long v17; // r14
  char *v18; // rbp
  time_t v19; // rax
  long long v20; // rdx
  char *v21; // rax
  char *v22; // rax
  int v23; // eax
  long long v24; // rsi
  int *v25; // rax
  long long v26; // rsi
  int *v27; // rbp
  int v28; // eax
  const char *v29; // rax
  char *v30; // rax
  long long v31; // r13
  char *v32; // r12
  int *v33; // rax
  int *v34; // rax
  long long v35; // [rsp+8h] [rbp-360h] BYREF
  struct stat buf; // [rsp+10h] [rbp-358h] BYREF
  char v37[664]; // [rsp+A0h] [rbp-2C8h] BYREF
  unsigned long long v38; // [rsp+338h] [rbp-30h]

  v38 = __readfsqword(0x28u);
  v7 = *a2;
  if ( *a2 == 1 )
  {
    sub_EA10(*((long long *)a2 + 1), a2[4], a1);
    sub_E9A0(a1);
  }
  else if ( v7 == 2 )
  {
    v8 = a3;
    switch ( *((char *)a2 + 4) )
    {
      case 0:
        v30 = dcgettext(0, "error: %s at end of format string", 5);
        v24 = 0;
        error(1, 0, v30, "%");
        goto LABEL_63;
      case 0x25:
        goto LABEL_5;
      case 0x44:
        v10 = (char *)sub_1B800(*(long long *)a4, v37, 0, 1, 1, a6);
        goto LABEL_9;
      case 0x46:
        a3 = (char *)sub_104C0(a4, a3);
        goto LABEL_13;
      case 0x47:
        goto LABEL_11;
      case 0x48:
        v16 = (char *)sub_26990(dword_3E5FC + 1);
        v17 = dword_3E5FC;
        v18 = v16;
        memcpy(v16, v8, dword_3E5FC);
        v18[v17] = 0;
        sub_E8C0(a1, *((long long *)a2 + 1), v18);
        goto LABEL_25;
      case 0x4D:
        sub_17D90(a4, v37);
        v15 = *((long long *)a2 + 1);
        v37[10] = 0;
        sub_E8C0(a1, v15, v37);
        return;
      case 0x50:
        a3 = (char *)"";
        if ( dword_3E5E0 > 0 )
          a3 = &v8[dword_3E5FC + (v8[dword_3E5FC] == 47)];
LABEL_13:
        v12 = (const char *)*((long long *)a2 + 1);
        goto LABEL_14;
      case 0x53:
        sub_F2C0(*(long long *)(a4 + 48), *(long long *)(a4 + 64));
        sub_E8C0(a1, *((long long *)a2 + 1));
        return;
      case 0x55:
        goto LABEL_8;
      case 0x59:
        v13 = *(int *)(a4 + 24) & 0xF000;
        if ( v13 != 40960 )
          goto LABEL_21;
        if ( !fstatat(dword_3E5F8, src, &buf, 0) )
        {
          v10 = sub_E750(buf.st_mode & 0xF000);
          goto LABEL_9;
        }
        v25 = __errno_location();
        v26 = *((long long *)a2 + 1);
        v27 = v25;
        v28 = *v25;
        if ( v28 == 2 || v28 == 20 )
        {
          sub_E8C0(a1, v26, "N");
        }
        else if ( v28 == 40 )
        {
          sub_E8C0(a1, v26, "L");
        }
        else
        {
          sub_E8C0(a1, v26, "?");
          v29 = (const char *)sub_DFB0(0, (long long)v8);
          error(0, *v27, "%s", v29);
        }
        return;
      case 0x5A:
        v23 = qword_3E680((unsigned int)dword_3E5F8, src, &v35);
        v24 = *((long long *)a2 + 1);
        if ( v23 < 0 )
        {
LABEL_63:
          sub_E8C0(a1, v24, "");
          v31 = sub_DFB0(0, (long long)v8);
          v32 = dcgettext(0, "getfilecon failed: %s", 5);
          v33 = __errno_location();
          error(0, *v33, v32, v31);
          status = 1;
        }
        else
        {
          sub_E8C0(a1, v24, v35);
          freecon(v35);
        }
        return;
      case 0x61:
        v19 = sub_25C40(a4);
        goto LABEL_29;
      case 0x62:
        v10 = (char *)sub_1B800(*(long long *)(a4 + 64), v37, 0, 512, 512, a6);
        goto LABEL_9;
      case 0x63:
        v19 = sub_25C50(a4);
        goto LABEL_29;
      case 0x64:
        sub_E8C0(a1, *((long long *)a2 + 1), (unsigned int)dword_3E5E0);
        return;
      case 0x66:
        v18 = (char *)sub_176F0(a3);
        goto LABEL_48;
      case 0x67:
        v11 = getgrgid(*(int *)(a4 + 32));
        if ( v11 )
          goto LABEL_27;
LABEL_11:
        v10 = (char *)sub_1B800(*(unsigned int *)(a4 + 32), v37, 0, 1, 1, a6);
        goto LABEL_9;
      case 0x68:
        v18 = (char *)sub_26D30(a3);
        v21 = &v18[strlen(v18) - 1];
        if ( v18 > v21 )
          goto LABEL_46;
        break;
      case 0x69:
        v10 = (char *)sub_1B800(*(long long *)(a4 + 8), v37, 0, 1, 1, a6);
        goto LABEL_9;
      case 0x6B:
        v10 = (char *)sub_1B800(*(long long *)(a4 + 64), v37, 0, 512, 1024, a6);
        goto LABEL_9;
      case 0x6C:
        if ( (*(int *)(a4 + 24) & 0xF000) != 0xA000 )
          goto LABEL_36;
        v18 = (char *)sub_17190((unsigned int)dword_3E5F8, src);
        if ( v18 )
          goto LABEL_48;
        v34 = __errno_location();
        sub_E060(*v34, (long long)v8);
        status = 1;
LABEL_36:
        v18 = 0;
        sub_E9D0(a1, *((const char **)a2 + 1), (long long)"");
        goto LABEL_25;
      case 0x6D:
        sub_E8C0(a1, *((long long *)a2 + 1), *(int *)(a4 + 24) & 0xFFF);
        return;
      case 0x6E:
        v10 = (char *)sub_1B800(*(long long *)(a4 + 16), v37, 0, 1, 1, a6);
        goto LABEL_9;
      case 0x70:
        v12 = (const char *)*((long long *)a2 + 1);
LABEL_14:
        sub_E9D0(a1, v12, (long long)a3);
        return;
      case 0x73:
        v10 = (char *)sub_1B800(*(long long *)(a4 + 48), v37, 0, 1, 1, a6);
        goto LABEL_9;
      case 0x74:
        v19 = sub_25C60(a4);
LABEL_29:
        v14 = sub_F050(v19, v20);
        goto LABEL_22;
      case 0x75:
        v11 = getpwuid(*(int *)(a4 + 28));
        if ( v11 )
        {
LABEL_27:
          *(char *)(*((long long *)a2 + 1) + a2[4]) = 115;
          sub_E8C0(a1, *((long long *)a2 + 1), *v11);
        }
        else
        {
LABEL_8:
          v10 = (char *)sub_1B800(*(unsigned int *)(a4 + 28), v37, 0, 1, 1, a6);
LABEL_9:
          sub_E8C0(a1, *((long long *)a2 + 1), v10);
        }
        return;
      case 0x79:
        v13 = *(int *)(a4 + 24) & 0xF000;
LABEL_21:
        v14 = sub_E750(v13);
LABEL_22:
        sub_E8C0(a1, *((long long *)a2 + 1), v14);
        return;
      default:
        return;
    }
    while ( *v21 == 47 )
    {
      if ( v18 > --v21 )
        goto LABEL_46;
    }
    if ( v18 < v21 && v21[1] == 47 )
      v21[1] = 0;
LABEL_46:
    v22 = strrchr(v18, 47);
    if ( v22 )
    {
      *v22 = 0;
LABEL_48:
      sub_E9D0(a1, *((const char **)a2 + 1), (long long)v18);
    }
    else
    {
      sub_E9D0(a1, *((const char **)a2 + 1), (long long)".");
    }
LABEL_25:
    free(v18);
  }
  else if ( !v7 )
  {
LABEL_5:
    sub_EA10(*((long long *)a2 + 1), a2[4], a1);
  }
}



// Function: pred_fprintf @ 0xfad0
long long pred_fprintf(char *a1, long long a2, long long a3, long long a4, long long a5, long long a6)
{
  long long v6; // rbx
  long long v9; // r13
  char v10; // bp
  char v11; // al
  time_t v12; // rdi
  long long v13; // rdx
  long long v14; // rsi
  char *v15; // rax
  long long v17; // rdx
  long long v18; // rdx
  long long v19; // rdx

  v6 = *(long long *)(a3 + 56);
  if ( v6 )
  {
    v9 = a3 + 56;
    do
    {
      while ( 1 )
      {
        if ( *(int *)v6 == 2 )
        {
          v10 = *(char *)(v6 + 5);
          if ( v10 )
            break;
        }
        sub_F310(v9, (int *)v6, a1, a2, a5, a6);
        v6 = *(long long *)(v6 + 24);
        if ( !v6 )
          return 1;
      }
      v11 = *(char *)(v6 + 4);
      if ( v11 == 67 )
      {
        v12 = sub_25C50(a2);
        v14 = v19;
      }
      else if ( v11 > 67 )
      {
        if ( v11 != 84 )
LABEL_20:
          __assert_fail("0", "print.c", 0x518u, "pred_fprintf");
        v12 = sub_25C60(a2);
        v14 = v18;
      }
      else if ( v11 == 65 )
      {
        v12 = sub_25C40(a2);
        v14 = v13;
      }
      else
      {
        if ( v11 != 66 )
          goto LABEL_20;
        v12 = sub_25C70(a2);
        v14 = v17;
        if ( v10 != 64 && v17 < 0 )
        {
          sub_E8C0(v9, *(long long *)(v6 + 8), "");
          goto LABEL_9;
        }
      }
      v15 = sub_EDA0(v12, v14, v10);
      sub_E8C0(v9, *(long long *)(v6 + 8), v15);
LABEL_9:
      v6 = *(long long *)(v6 + 24);
    }
    while ( v6 );
  }
  return 1;
}



// Function: make_segment @ 0xfc20
long long make_segment(long long *a1, const char *a2, int a3, unsigned int a4, char a5, char a6, long long a7)
{
  size_t v8; // r14
  long long v11; // rax
  long long v12; // r15
  char *v13; // rax
  char *v14; // rcx
  unsigned int v15; // eax
  long long result; // rax
  long long v17; // rax
  char *v18; // rax
  char v19; // [rsp+Fh] [rbp-39h]

  v19 = a5;
  switch ( a5 )
  {
    case '{':
      __assert_fail("format_char != '{'", "print.c", 0x4Eu, "make_segment");
    case '[':
      __assert_fail("format_char != '['", "print.c", 0x4Fu, "make_segment");
    case '(':
      __assert_fail("format_char != '('", "print.c", 0x50u, "make_segment");
  }
  v8 = a3;
  v11 = sub_26990(32);
  *a1 = v11;
  v12 = v11;
  *(int *)v11 = a4;
  *(char *)(v11 + 4) = a5;
  *(char *)(v11 + 5) = a6;
  *(long long *)(v11 + 24) = 0;
  *(int *)(v11 + 16) = v8;
  v13 = (char *)sub_26990(v8 + 2);
  *(long long *)(v12 + 8) = v13;
  v14 = &strncpy(v13, a2, v8)[v8];
  if ( a4 <= 1 )
  {
    if ( a5 )
      __assert_fail("0 == format_char", "print.c", 0x61u, "make_segment");
    if ( a6 )
      __assert_fail("0 == aux_format_char", "print.c", 0x62u, "make_segment");
    v17 = *a1;
    *v14 = 0;
    return v17 + 24;
  }
  else
  {
    if ( a4 != 2 )
      __assert_fail("kind == KIND_FORMAT", "print.c", 0x69u, "make_segment");
    switch ( a5 )
    {
      case '%':
      case 'd':
        v18 = v14 + 1;
        goto LABEL_16;
      case 'A':
      case 'B':
      case 'C':
      case 'D':
      case 'F':
      case 'G':
      case 'M':
      case 'T':
      case 'U':
      case 'a':
      case 'b':
      case 'c':
      case 'g':
      case 'k':
      case 'n':
      case 's':
      case 't':
      case 'u':
        *v14++ = 115;
        *(char *)(a7 + 26) = 1;
        v15 = 3;
        goto LABEL_8;
      case 'H':
      case 'P':
      case 'f':
      case 'h':
      case 'p':
        v19 = 115;
        v18 = v14 + 1;
LABEL_16:
        *v14 = v19;
        *v18 = 0;
        goto LABEL_10;
      case 'S':
        *v14++ = 103;
        *(char *)(a7 + 26) = 1;
        v15 = 3;
        goto LABEL_8;
      case 'Y':
        *v14++ = 115;
        *(char *)(a7 + 26) = 1;
        v15 = 2;
        goto LABEL_8;
      case 'Z':
        *v14 = 115;
        v15 = 5;
        ++v14;
        goto LABEL_8;
      case 'i':
        *v14++ = 115;
        *(char *)(a7 + 28) = 1;
        v15 = 1;
        goto LABEL_8;
      case 'l':
        *v14++ = 115;
        *(char *)(a7 + 26) = 1;
        v15 = 4;
        goto LABEL_8;
      case 'm':
        *v14++ = 111;
        *(char *)(a7 + 26) = 1;
        v15 = 3;
        goto LABEL_8;
      case 'y':
        *v14++ = 115;
        *(char *)(a7 + 27) = 1;
        v15 = 2;
        goto LABEL_8;
      default:
        v15 = 0;
LABEL_8:
        *v14 = 0;
        if ( *(int *)(a7 + 32) < v15 )
          *(int *)(a7 + 32) = v15;
LABEL_10:
        result = *a1 + 24;
        break;
    }
  }
  return result;
}



// Function: insert_fprintf @ 0xfef0
long long insert_fprintf(const __m128i *a1, long long a2, long long a3)
{
  unsigned long long v3; // r15
  long long *v4; // rbx
  long long v5; // rax
  char v6; // al
  long long *v7; // r13
  const char *v8; // r12
  char *v9; // rbp
  char v10; // cl
  unsigned int v11; // r14d
  char v12; // al
  long long v13; // r14
  const char *v14; // rsi
  int v15; // edx
  long long v17; // rax
  long long v18; // r14
  char v19; // r9
  int v20; // edx
  const char *v21; // rsi
  const char *v22; // rsi
  char *v23; // rax
  const char *v24; // rsi
  int v25; // edx
  char *v26; // rax
  char *v27; // rax
  char *v28; // rax
  char *v29; // rax
  long long v30[9]; // [rsp+10h] [rbp-48h] BYREF

  v3 = a3;
  v30[1] = __readfsqword(0x28u);
  v4 = sub_D450(a2, (long long)sub_FAD0, a3);
  *(__m128i *)(v4 + 7) = _mm_loadu_si128(a1);
  *(__m128i *)(v4 + 9) = _mm_loadu_si128(a1 + 1);
  v5 = a1[2].m128i_i64[0];
  *((int *)v4 + 6) = 257;
  v4[11] = v5;
  *((int *)v4 + 8) = 0;
  v4[7] = 0;
  v6 = *(char *)v3;
  if ( !*(char *)v3 )
    return 1;
  v7 = v4 + 7;
  v8 = (const char *)v3;
  while ( 1 )
  {
    while ( 1 )
    {
      v10 = *(char *)(v3 + 1);
      if ( v6 == 92 )
        break;
      if ( v6 == 37 )
      {
        if ( !v10 )
          goto LABEL_39;
        v9 = (char *)(v3 + 1);
        if ( v10 != 37 )
        {
          v9 = (char *)(v3 + sub_E820(v3));
          v10 = *v9;
        }
        v3 = (unsigned int)v10;
        v17 = sub_E7D0(v10);
        v18 = v17;
        if ( v17 )
        {
          if ( v9[v17 - 1] )
          {
            v19 = 0;
            if ( v17 == 2 )
              v19 = v9[1];
            v20 = (int)v9 - (int)v8;
            v21 = v8;
            v8 = &v9[v17];
            v9 = &v9[v17 - 1];
            v7 = (long long *)sub_FC20(v7, v21, v20, 2u, v3, v19, (long long)v4);
            goto LABEL_24;
          }
          if ( strchr("{[(", v3) )
          {
LABEL_38:
            v28 = dcgettext(0, "error: the format directive `%%%c' is reserved for future use", 5);
            error(1, 0, v28, (unsigned int)v3);
LABEL_39:
            v29 = dcgettext(0, "error: %s at end of format string", 5);
            error(1, 0, v29, v3);
          }
          if ( v18 == 2 )
          {
            v22 = "warning: format directive `%%%c' should be followed by another character";
            if ( !v9[1] )
            {
LABEL_27:
              v23 = dcgettext(0, v22, 5);
              error(0, 0, v23, (unsigned int)v3);
              v24 = v8;
              v25 = (int)v9 + 1 - (int)v8;
              v8 = v9 + 1;
              v7 = (long long *)sub_FC20(v7, v24, v25, 0, 0, 0, (long long)v4);
LABEL_24:
              v3 = (unsigned long long)v8;
              goto LABEL_5;
            }
          }
        }
        else if ( strchr("{[(", v3) )
        {
          goto LABEL_38;
        }
        v22 = "warning: unrecognized format directive `%%%c'";
        goto LABEL_27;
      }
      v9 = (char *)v3++;
LABEL_5:
      v6 = v9[1];
      if ( !v6 )
        goto LABEL_13;
    }
    if ( v10 == 99 )
      break;
    if ( !v10 )
    {
      v9 = (char *)v3;
      v13 = 1;
      v26 = dcgettext(0, "warning: escape `\\' followed by nothing at all", 5);
      error(0, 0, v26);
      goto LABEL_12;
    }
    v11 = v10;
    v9 = (char *)(v3 + 1);
    if ( sub_E720(v10) )
    {
      v30[0] = 0;
      *(char *)v3 = sub_F270(v3 + 1, v30);
      v13 = v30[0] + 2LL;
      v9 = (char *)(v3 + v30[0] + 1);
      goto LABEL_12;
    }
    v12 = sub_E730(v11);
    if ( !v12 )
    {
      v3 += 2LL;
      v27 = dcgettext(0, "warning: unrecognized escape `\\%c'", 5);
      error(0, 0, v27, v11);
      goto LABEL_5;
    }
    *(char *)v3 = v12;
    v13 = 2;
LABEL_12:
    v14 = v8;
    v15 = v3 - (int)v8 + 1;
    v8 = (const char *)(v3 + v13);
    v3 += v13;
    v7 = (long long *)sub_FC20(v7, v14, v15, 0, 0, 0, (long long)v4);
    v6 = v9[1];
    if ( !v6 )
    {
LABEL_13:
      if ( (unsigned long long)v8 < v3 )
        sub_FC20(v7, v8, v3 - (int)v8, 0, 0, 0, (long long)v4);
      return 1;
    }
  }
  sub_FC20(v7, v8, v3 - (int)v8, 1u, 0, 0, (long long)v4);
  if ( *((char *)v4 + 26) && *((int *)v4 + 8) <= 2u )
    *((int *)v4 + 8) = 3;
  return 1;
}



// Function: free_file_system_list @ 0x102e0
long long free_file_system_list(void *ptr)
{
  void *v1; // rbx
  void *v2; // rdi
  long long result; // rax

  if ( ptr )
  {
    v1 = ptr;
    do
    {
      v2 = v1;
      v1 = (void *)*((long long *)v1 + 6);
      result = sub_1DC40(v2);
    }
    while ( v1 );
  }
  return result;
}



// Function: get_file_system_list @ 0x10310
void *get_file_system_list(unsigned char a1)
{
  void *result; // rax

  result = qword_3EE40;
  if ( !qword_3EE40 )
    goto LABEL_6;
  if ( byte_3EE38 != 1 && a1 )
  {
    sub_102E0(qword_3EE40);
    qword_3EE40 = 0;
LABEL_6:
    result = (void *)sub_1DC90(a1);
    byte_3EE38 = a1;
    qword_3EE40 = result;
  }
  return result;
}



// Function: set_fstype_devno @ 0x10360
long long set_fstype_devno(long long a1)
{
  long long result; // rax
  long long v2[23]; // [rsp+0h] [rbp-B8h] BYREF

  v2[19] = __readfsqword(0x28u);
  result = 0;
  if ( *(long long *)(a1 + 32) == -1 )
  {
    sub_D860();
    result = qword_3E670(*(long long *)(a1 + 8), v2);
    if ( (int)result )
      return 0xFFFFFFFFLL;
    else
      *(long long *)(a1 + 32) = v2[0];
  }
  return result;
}



// Function: filesystem_type @ 0x104c0
void *filesystem_type(long long *a1)
{
  void *result; // rax

  result = qword_3EE58;
  if ( !qword_3EE58 )
    goto LABEL_5;
  if ( !byte_3EE50 || *a1 != qword_3EE48 )
  {
    free(qword_3EE58);
LABEL_5:
    qword_3EE48 = *a1;
    result = (void *)sub_103E0(a1);
    qword_3EE58 = result;
  }
  return result;
}



// Function: is_used_fs_type @ 0x10510
long long is_used_fs_type(char *s1)
{
  void *v2; // rbx

  if ( strcmp("afs", s1) )
  {
    v2 = sub_10310(0);
    if ( v2 )
    {
      while ( strcmp(s1, *((const char **)v2 + 3)) )
      {
        v2 = (void *)*((long long *)v2 + 6);
        if ( !v2 )
          return 0;
      }
    }
  }
  return 1;
}



// Function: get_mounted_devices @ 0x10580
void *get_mounted_devices(long long *a1)
{
  long long v1; // rax
  long long v2; // r14
  void *v3; // r15
  long long v4; // r12
  long long v5; // rbp
  void *v6; // rdi
  long long v8; // [rsp+8h] [rbp-60h]
  void *ptr; // [rsp+10h] [rbp-58h]

  v1 = sub_1DC90(0);
  ptr = (void *)v1;
  if ( v1 )
  {
    v2 = v1;
    v3 = 0;
    v4 = 0;
    do
    {
      while ( 1 )
      {
        v5 = 8 * (v4 + 1);
        v8 = sub_15710(v3);
        if ( v8 )
          break;
        v6 = v3;
        v3 = 0;
        free(v6);
        v2 = *(long long *)(v2 + 48);
        if ( !v2 )
          goto LABEL_8;
      }
      v3 = (void *)v8;
      if ( !(unsigned int)sub_10360(v2) )
      {
        ++v4;
        *(long long *)(v8 + v5 - 8) = *(long long *)(v2 + 32);
      }
      v2 = *(long long *)(v2 + 48);
    }
    while ( v2 );
LABEL_8:
    sub_102E0(ptr);
    if ( v3 )
      *a1 = v4;
  }
  else
  {
    v3 = 0;
    sub_102E0(0);
  }
  return v3;
}



// Function: get_noop @ 0x10690
char *get_noop()
{
  long long v0; // r8
  long long *v1; // rax
  int v2; // edx
  int *v4; // rcx

  v0 = qword_3EE68;
  v1 = &unk_3CD40;
  v2 = 0;
  if ( !qword_3EE68 )
  {
    while ( 1 )
    {
      v0 = v1[1];
      ++v2;
      v4 = v1;
      if ( !v0 )
        break;
      v1 += 4;
      if ( *v4 == 1 )
      {
        qword_3EE68 = (long long)&unk_3CD20 + 32 * v2;
        return (char *)&unk_3CD20 + 32 * v2;
      }
    }
  }
  return (char *)v0;
}



// Function: should_issue_warnings @ 0x106f0
long long should_issue_warnings()
{
  long long result; // rax

  result = 0;
  if ( !byte_3E631 )
    return (unsigned char)byte_3E630;
  return result;
}



// Function: estimate_file_age_success_rate @ 0x10710
float estimate_file_age_success_rate(float a1)
{
  float v1; // xmm1_4

  if ( a1 < 0.1 )
    return 0.0099999998;
  v1 = 0.30000001;
  if ( a1 >= 1.0 && a1 <= 100.0 )
    return 0.38999999;
  return v1;
}



// Function: estimate_timestamp_success_rate @ 0x10760
float estimate_timestamp_success_rate(long long a1)
{
  return sub_10710((float)(int)((timer - a1) / 86400));
}



// Function: collect_arg_nonconst @ 0x10790
long long collect_arg_nonconst(long long a1, int *a2, long long *a3)
{
  long long v3; // rax
  long long v4; // rcx

  if ( a1 && (v3 = *a2, (v4 = *(long long *)(a1 + 8 * v3)) != 0) )
  {
    *a3 = v4;
    *a2 = v3 + 1;
    return 1;
  }
  else
  {
    *a3 = 0;
    return 0;
  }
}



// Function: collect_arg @ 0x107d0
long long collect_arg(long long a1, int *a2)
{
  long long result; // rax
  long long *v3; // r8
  long long v4[3]; // [rsp+0h] [rbp-18h] BYREF

  v4[1] = __readfsqword(0x28u);
  result = sub_10790(a1, a2, v4);
  *v3 = v4[0];
  return result;
}



// Function: parse_files0_from @ 0x10810
long long parse_files0_from(long long a1, long long a2, int *a3)
{
  long long result; // rax
  char *v4; // [rsp+0h] [rbp-18h]

  result = sub_107D0(a2, a3);
  if ( (char)result )
    filename = v4;
  return result;
}



// Function: get_comp_type @ 0x10860
long long get_comp_type(char **a1, int *a2)
{
  char *v2; // rax
  char v3; // dl

  v2 = *a1;
  v3 = **a1;
  if ( v3 == 43 )
  {
    *a2 = 0;
    *a1 = v2 + 1;
    return 1;
  }
  else
  {
    if ( v3 == 45 )
    {
      *a2 = 1;
      *a1 = v2 + 1;
    }
    else
    {
      *a2 = 2;
    }
    return 1;
  }
}



// Function: found_parser @ 0x108b0
int *found_parser(long long a1, int *a2)
{
  int *v2; // r12
  int v3; // eax
  long long v4; // rbx
  char *v6; // rax

  v2 = a2;
  v3 = *a2;
  if ( *a2 != 2 )
  {
    if ( v3 == 1 )
    {
      return 0;
    }
    else
    {
      v4 = qword_3EE70;
      if ( v3 )
      {
        if ( !qword_3EE70 )
          qword_3EE70 = a1;
      }
      else if ( qword_3EE70 && (unsigned char)sub_106F0() )
      {
        v6 = dcgettext(
               0,
               "warning: you have specified the global option %s after the argument %s, but global options are not positi"
               "onal, i.e., %s affects tests specified before it as well as those specified after it.  Please specify glo"
               "bal options before other arguments.",
               5);
        error(0, 0, v6, a1, v4, a1);
      }
    }
  }
  return v2;
}



// Function: fallback_getfilecon @ 0x10950
long long fallback_getfilecon(int fd, const char *a2, long long a3, unsigned int a4)
{
  int v5; // eax

  v5 = *__errno_location();
  if ( v5 != 2 && v5 != 20 )
    return a4;
  if ( (qword_3E660 & 2) != 0 )
    __fprintf_chk(stderr, 1, "fallback_getfilecon(): getfilecon(%s) failed; falling back on lgetfilecon()\n", a2);
  return sub_254C0(fd);
}



// Function: optionh_getfilecon @ 0x109e0
long long optionh_getfilecon(int fd, const char *a2, long long a3)
{
  unsigned int v4; // eax

  if ( dword_3E5E0 )
    return sub_254C0(fd);
  v4 = sub_252A0(fd);
  if ( v4 )
    return sub_10950(fd, a2, a3, v4);
  else
    return 0;
}



// Function: optionl_getfilecon @ 0x10a40
long long optionl_getfilecon(int fd, const char *a2, long long a3)
{
  unsigned int v4; // eax

  v4 = sub_252A0(fd);
  if ( v4 )
    return sub_10950(fd, a2, a3, v4);
  else
    return 0;
}



// Function: parse_version @ 0x10a80
void parse_version()
{
  char *v0; // rax
  const char *v1; // rdx
  int v2; // [rsp+0h] [rbp-14h] BYREF
  unsigned long long v3; // [rsp+4h] [rbp-10h]

  v3 = __readfsqword(0x28u);
  sub_15E60("find");
  v0 = dcgettext(0, "Features enabled: ", 5);
  __printf_chk(1, v0);
  __printf_chk(1, "D_TYPE ");
  v1 = "enabled";
  if ( !byte_3E678 )
    v1 = "disabled";
  __printf_chk(1, "O_NOFOLLOW(%s) ", v1);
  __printf_chk(1, "LEAF_OPTIMISATION ");
  if ( (int)is_selinux_enabled() > 0 )
    __printf_chk(1, "SELINUX ");
  v2 = 0;
  if ( (unsigned char)sub_86D0(&v2) )
  {
    __printf_chk(1, "FTS(");
    if ( (v2 & 0x200) != 0 )
      __printf_chk(1, "FTS_CWDFD");
    __printf_chk(1, ") ");
  }
  __printf_chk(1, "CBO(level=%d) ", (unsigned short)word_3E688);
  __printf_chk(1, "\n");
  exit(0);
}



// Function: parse_true @ 0x10bf0
long long parse_true(long long a1)
{
  long long *v1; // rax

  v1 = sub_D4D0(a1);
  *((short *)v1 + 13) = 0;
  *((int *)v1 + 9) = 1065353216;
  return 1;
}



// Function: parse_quit @ 0x10c20
long long parse_quit(long long a1)
{
  long long *v1; // rax

  v1 = sub_D4D0(a1);
  *((int *)v1 + 6) = 1;
  *((int *)v1 + 9) = 1065353216;
  return 1;
}



// Function: parse_prune @ 0x10c50
long long parse_prune(long long a1)
{
  long long *v1; // rax

  v1 = sub_D4D0(a1);
  if ( !(char)word_3E620 )
    *((short *)v1 + 13) = 0;
  *((short *)v1 + 12) = 1;
  return 1;
}



// Function: parse_nouser @ 0x10c80
long long parse_nouser(long long a1)
{
  *((int *)sub_D4D0(a1) + 9) = 981668463;
  return 1;
}



// Function: parse_accesscheck @ 0x10ca0
long long parse_accesscheck(long long a1)
{
  long long *v1; // rax
  int v2; // xmm0_4

  v1 = sub_D4D0(a1);
  v2 = 1045220557;
  *((int *)v1 + 6) = 0;
  if ( (bool (*)())*v1 != sub_9640 )
    v2 = 1063675494;
  *((int *)v1 + 9) = v2;
  return 1;
}



// Function: parse_empty @ 0x10ce0
long long parse_empty(long long a1)
{
  *((int *)sub_D4D0(a1) + 9) = 1008981770;
  return 1;
}



// Function: parse_delete @ 0x10d00
long long parse_delete(long long a1)
{
  long long *v1; // rax

  v1 = sub_D4D0(a1);
  LOBYTE(word_3E620) = 0;
  *((int *)v1 + 6) = 257;
  *((int *)v1 + 9) = 1065353216;
  return 1;
}



// Function: insert_type @ 0x10d30
long long insert_type(
        long long a1,
        int *a2,
        long long a3,
        long long (*a4)(long long a1, long long a2, long long a3),
        double a5,
        double a6,
        double a7,
        double a8)
{
  const char *v10; // r12
  unsigned int v11; // r13d
  char *v12; // rbx
  long long *v13; // rdx
  char v14; // al
  char v15; // cl
  char v16; // al
  unsigned int v17; // r14d
  long long v18; // rax
  char v19; // al
  char *v21; // rax
  char *v22; // rax
  char *v23; // rax
  char *v24; // rax
  char *v25; // rax
  char *v26; // rax
  char *v27; // [rsp+0h] [rbp-48h]

  v10 = "-xtype";
  if ( a4 != sub_99E0 )
    v10 = "-type";
  v11 = sub_107D0(a1, a2);
  if ( (char)v11 )
  {
    v12 = v27;
    if ( !*v27 )
    {
LABEL_28:
      v24 = dcgettext(0, "Arguments to %s should contain at least one letter", 5);
      error(1, 0, v24, v10);
LABEL_29:
      v25 = dcgettext(0, "Last file type in list argument to %s is missing, i.e., list is ending on: ','", 5);
      error(1, 0, v25, v10, a5, a6, a7, a8);
LABEL_30:
      v26 = dcgettext(0, "Must separate multiple arguments to %s using: ','", 5);
      error(1, 0, v26, v10, a5, a6, a7, a8);
    }
    v13 = sub_D450(a3, (long long)a4, (long long)v27);
    v14 = 0;
    *(long long *)&a8 = 922581086;
    v15 = 0;
    if ( a4 == sub_99E0 )
      v14 = v11;
    *((int *)v13 + 9) = 0;
    *(long long *)&a7 = 1023329645;
    if ( a4 != sub_99E0 )
      v15 = v11;
    *(long long *)&a6 = 1063256064;
    *((char *)v13 + 26) = v14;
    *((char *)v13 + 27) = v15;
    v16 = *v27;
    while ( v16 )
    {
      v17 = v16;
      switch ( v16 )
      {
        case 'D':
          goto LABEL_26;
        case 'b':
          *(long long *)&a5 = 979945644;
          v18 = 0;
          goto LABEL_13;
        case 'c':
          *(long long *)&a5 = 971522676;
          v18 = 1;
          goto LABEL_13;
        case 'd':
          *(long long *)&a5 = 1035785051;
          v18 = 2;
          goto LABEL_13;
        case 'f':
          *(long long *)&a5 = 1063256064;
          v18 = 3;
          goto LABEL_13;
        case 'l':
          *(long long *)&a5 = 1023329645;
          v18 = 4;
          goto LABEL_13;
        case 'p':
          *(long long *)&a5 = 922581086;
          v18 = 5;
          goto LABEL_13;
        case 's':
          *(long long *)&a5 = 931487997;
          v18 = 6;
LABEL_13:
          if ( *((char *)v13 + v18 + 56) )
            goto LABEL_27;
          *(float *)&a5 = *(float *)&a5 + *((float *)v13 + 9);
          *((int *)v13 + 9) = LODWORD(a5);
          *((char *)v13 + v18 + 56) = 1;
          v19 = v12[1];
          if ( !v19 )
            return v11;
          if ( v19 != 44 )
            goto LABEL_30;
          v16 = v12[2];
          if ( !v16 )
            goto LABEL_29;
          v12 += 2;
          break;
        default:
          v21 = dcgettext(0, "Unknown argument to %s: %c", 5);
          error(1, 0, v21, v10, v17);
LABEL_26:
          v22 = dcgettext(
                  0,
                  "%s %c is not supported because Solaris doors are not supported on the platform find was compiled on.",
                  5);
          error(1, 0, v22, v10, 68, a5, COERCE_DOUBLE(1063256064), COERCE_DOUBLE(1023329645), COERCE_DOUBLE(922581086));
LABEL_27:
          v23 = dcgettext(0, "Duplicate file type '%c' in the argument list to %s.", 5);
          error(1, 0, v23, v17, v10, a5, COERCE_DOUBLE(1063256064), COERCE_DOUBLE(1023329645), COERCE_DOUBLE(922581086));
          goto LABEL_28;
      }
    }
  }
  return v11;
}



// Function: parse_xtype @ 0x10fe0
long long parse_xtype(long long a1, long long a2, int *a3, double a4, double a5, double a6, double a7)
{
  return sub_10D30(a2, a3, a1, sub_99E0, a4, a5, a6, a7);
}



// Function: fnmatch_sanitycheck @ 0x11020
long long fnmatch_sanitycheck()
{
  char *v1; // rax

  if ( byte_3EE60 )
    return 1;
  if ( fnmatch("foo", "foo", 0) || !fnmatch("Foo", "foo", 0) || fnmatch("Foo", "foo", 16) )
  {
    v1 = dcgettext(0, "sanity check of the fnmatch() library function failed.", 5);
    error(1, 0, v1);
    return sub_110C0((char *)&dword_0 + 1);
  }
  else
  {
    byte_3EE60 = 1;
    return 1;
  }
}



// Function: estimate_pattern_match_rate @ 0x110c0
float estimate_pattern_match_rate(char *s, int a2)
{
  float result; // xmm0_4

  result = 0.80000001;
  if ( !strpbrk(s, "*?[") )
  {
    result = 0.1;
    if ( a2 )
    {
      result = 0.1;
      if ( strpbrk(s, ".") )
        return 0.80000001;
    }
  }
  return result;
}



// Function: is_feasible_path_argument @ 0x11130
bool is_feasible_path_argument(char *pattern, unsigned char a2)
{
  char *v2; // rax

  v2 = strrchr(pattern, 47);
  return !v2 || v2[1] || sub_B510(pattern, a2);
}



// Function: insert_path_check @ 0x11180
long long insert_path_check(
        long long a1,
        long long a2,
        int *a3,
        long long a4,
        bool (*a5)(char *name, long long a2, long long a3))
{
  unsigned int v8; // r12d
  long long *v10; // rbp
  float v11; // xmm0_4
  bool v12; // zf
  char *v13; // rax
  char *s; // [rsp+0h] [rbp-48h]

  sub_11020();
  v8 = sub_107D0(a2, a3);
  if ( (char)v8 )
  {
    v10 = sub_D450(a1, (long long)a5, (long long)s);
    *((short *)v10 + 13) = 0;
    v10[7] = s;
    v11 = sub_110C0(s, 0);
    v12 = byte_3E631 == 0;
    *((float *)v10 + 9) = v11;
    if ( v12 && !sub_11130(s, a5 == sub_91E0) )
    {
      v13 = dcgettext(0, "warning: -%s %s will not match anything because it ends with /.", 5);
      error(0, 0, v13, a4);
      *((int *)v10 + 9) = 841731191;
    }
  }
  return v8;
}



// Function: parse_wholename @ 0x11280
long long parse_wholename(long long a1, long long a2, int *a3)
{
  return sub_11180(a1, a2, a3, (long long)"wholename", sub_95A0);
}



// Function: parse_user @ 0x11300
long long parse_user(long long a1, long long a2, int *a3)
{
  long long v3; // r12
  struct passwd *v5; // rbx
  unsigned int pw_uid; // ebx
  long long *v7; // rax
  int v8; // xmm0_4
  size_t v9; // rax
  char *v10; // rax
  char *v11; // rax
  char *name; // [rsp+0h] [rbp-38h]

  LODWORD(v3) = sub_107D0(a2, a3);
  if ( (char)v3 )
  {
    v5 = getpwnam(name);
    endpwent();
    if ( v5 )
    {
      pw_uid = v5->pw_uid;
    }
    else
    {
      v9 = strspn(name, "0123456789");
      if ( !v9 || name[v9] )
      {
        if ( *name )
        {
          v3 = sub_24E50(0, (unsigned int)dword_3E68C);
          v10 = dcgettext(0, "%s is not the name of a known user", 5);
          error(1, 0, v10, v3);
        }
        v11 = dcgettext(0, "The argument to -user should not be empty", 5);
        error(1, 0, v11);
      }
      pw_uid = sub_16F40(name);
    }
    v7 = sub_D490(a1, (long long)name);
    v8 = 1065185444;
    *((int *)v7 + 14) = pw_uid;
    if ( pw_uid > 0x63 )
      v8 = 1045220557;
    *((int *)v7 + 9) = v8;
  }
  return (unsigned int)v3;
}



// Function: parse_nogroup @ 0x11460
long long parse_nogroup(long long a1)
{
  *((int *)sub_D490(a1, 0) + 9) = 953267991;
  return 1;
}



// Function: parse_lname @ 0x11480
long long parse_lname(long long a1, long long a2, int *a3)
{
  unsigned int v4; // r12d
  float *v6; // rax
  char *s; // [rsp+0h] [rbp-38h]

  sub_11020();
  v4 = sub_107D0(a2, a3);
  if ( (char)v4 )
  {
    v6 = (float *)sub_D490(a1, (long long)s);
    *((long long *)v6 + 7) = s;
    v6[9] = sub_110C0(s, 0) * 0.1;
  }
  return v4;
}



// Function: parse_ilname @ 0x11520
long long parse_ilname(long long a1, long long a2, int *a3)
{
  unsigned int v3; // r12d
  float *v5; // rax
  char *s; // [rsp+0h] [rbp-28h]

  v3 = sub_107D0(a2, a3);
  if ( (char)v3 )
  {
    v5 = (float *)sub_D490(a1, (long long)s);
    *((long long *)v5 + 7) = s;
    v5[9] = sub_110C0(s, 0) * 0.1;
  }
  return v3;
}



// Function: parse_context @ 0x115b0
long long parse_context(long long a1, long long a2, int *a3)
{
  long long *v4; // rax
  long long v5; // rdx
  long long v6; // rdx
  char *v8; // rax

  if ( !a2 )
    return 0;
  if ( !*(long long *)(a2 + 8LL * *a3) )
    return 0;
  if ( (int)is_selinux_enabled() <= 0 )
  {
    v8 = dcgettext(0, "invalid predicate -context: SELinux is not enabled.", 5);
    error(1, 0, v8);
    return sub_11650();
  }
  else
  {
    v4 = sub_D490(a1, 0);
    v5 = *a3;
    *((int *)v4 + 9) = 1008981770;
    v6 = *(long long *)(a2 + 8 * v5);
    *((char *)v4 + 26) = 0;
    v4[7] = v6;
    ++*a3;
    return 1;
  }
}



// Function: get_relative_timestamp @ 0x11650
long long get_relative_timestamp(char *a1, long long a2, long long a3, double a4)
{
  int v5; // ecx
  const char *v6; // r8
  int v7; // eax
  int v8; // r12d
  const char *v9; // r13
  char *v10; // r14
  unsigned int v11; // r15d
  double v13; // xmm0_8
  double v14; // xmm2_8
  long long v15; // rax
  long long v16; // rdx
  char *v17; // [rsp+8h] [rbp-60h] BYREF
  double v18; // [rsp+18h] [rbp-50h] BYREF
  double iptr[9]; // [rsp+20h] [rbp-48h] BYREF

  v17 = a1;
  *(long long *)&iptr[1] = __readfsqword(0x28u);
  if ( !(unsigned char)sub_10860(&v17, (int *)(a2 + 4)) )
    return 0;
  v7 = *(int *)(a2 + 4);
  v8 = v5;
  v9 = v6;
  if ( v7 )
  {
    if ( v7 == 1 )
      *(int *)(a2 + 4) = 0;
  }
  else
  {
    *(int *)(a2 + 4) = 1;
  }
  v10 = v17;
  v11 = sub_26D90(v17, 0, &v18, &strtod);
  if ( !(char)v11 )
    return 0;
  v13 = modf(a4 * v18, iptr) * 1000000000.0;
  if ( v13 >= 1000000000.0 )
    __assert_fail("nanosec < nanosec_per_sec", "parser.c", 0xC4Du, "get_relative_timestamp");
  v14 = iptr[0];
  v15 = (unsigned int)(int)((double)(int)a3 - iptr[0]);
  *(long long *)(a2 + 8) = v15;
  if ( v15 > a3 == v14 < 0.0 )
  {
    v16 = (unsigned int)(int)((double)v8 - v13);
    if ( v13 <= (double)v8 )
    {
      *(long long *)(a2 + 16) = v16;
    }
    else
    {
      *(long long *)(a2 + 16) = v16 + 1000000000;
      *(long long *)(a2 + 8) = v15 - 1;
    }
    return v11;
  }
  error(1, 0, v9, v10);
  return sub_11800();
}



// Function: parse_used @ 0x11800
long long parse_used(long long a1, long long a2, int *a3)
{
  unsigned int v3; // r12d
  long long *v5; // rax
  long long v6; // rsi
  float v7; // xmm0_4
  long long v8; // rcx
  char *v9; // rax
  char *v10; // [rsp+8h] [rbp-40h]
  __m128i v11; // [rsp+10h] [rbp-38h] BYREF
  long long v12; // [rsp+20h] [rbp-28h]
  unsigned long long v13; // [rsp+28h] [rbp-20h]

  v13 = __readfsqword(0x28u);
  v3 = sub_107D0(a2, a3);
  if ( !(char)v3 )
    return v3;
  v3 = sub_11650(v10, (long long)&v11, 0, 86400.0);
  if ( (char)v3 )
  {
    v5 = sub_D490(a1, (long long)v10);
    v6 = v11.m128i_i64[1];
    *(__m128i *)(v5 + 7) = _mm_load_si128(&v11);
    v5[9] = v12;
    v7 = sub_10710((float)(int)(v6 / 86400));
    *(float *)(v8 + 36) = v7;
    return v3;
  }
  v9 = dcgettext(0, "Invalid argument %s to -used", 5);
  error(1, 0, v9);
  return sub_11920();
}



// Function: do_parse_xmin @ 0x11920
long long do_parse_xmin(long long a1, long long a2, int *a3, int a4)
{
  int v6; // r13d
  unsigned int v7; // r12d
  long long *v9; // rax
  long long v10; // rdi
  char *v11; // [rsp+8h] [rbp-50h]
  __m128i v12; // [rsp+10h] [rbp-48h] BYREF
  long long v13; // [rsp+20h] [rbp-38h]
  unsigned long long v14; // [rsp+28h] [rbp-30h]

  v6 = *a3;
  v14 = __readfsqword(0x28u);
  v7 = sub_107D0(a2, a3);
  if ( (char)v7 )
  {
    v12.m128i_i32[0] = a4;
    v7 = sub_11650(v11, (long long)&v12, timer + 86400, 60.0);
    if ( (char)v7 )
    {
      v9 = sub_D490(a1, (long long)v11);
      v10 = v12.m128i_i64[1];
      *(__m128i *)(v9 + 7) = _mm_load_si128(&v12);
      v9[9] = v13;
      *((float *)v9 + 9) = sub_10760(v10);
    }
    else
    {
      *a3 = v6;
    }
  }
  return v7;
}



// Function: parse_mmin @ 0x11a10
long long parse_mmin(long long a1, long long a2, int *a3)
{
  return sub_11920(a1, a2, a3, 3);
}



// Function: get_stat_Ytime @ 0x11a40
long long get_stat_Ytime(long long a1, char a2, long long *a3)
{
  long long v5; // rdx
  long long v6; // rdx
  long long v7; // rdx
  long long v8; // rdx

  if ( a2 == 99 )
  {
    *a3 = sub_25C50(a1);
    a3[1] = v7;
    return 1;
  }
  else if ( a2 > 99 )
  {
    if ( a2 != 109 )
      goto LABEL_10;
    *a3 = sub_25C60(a1);
    a3[1] = v6;
    return 1;
  }
  else
  {
    if ( a2 != 66 )
    {
      if ( a2 == 97 )
      {
        *a3 = sub_25C40(a1);
        a3[1] = v5;
        return 1;
      }
LABEL_10:
      __assert_fail("0", "parser.c", 0x1F2u, "get_stat_Ytime");
    }
    *a3 = sub_25C70(a1);
    a3[1] = v8;
    return a3[1] >= 0LL;
  }
}



// Function: get_num @ 0x11ae0
bool get_num(char *a1, long long a2, int *a3)
{
  char *nptr[4]; // [rsp+8h] [rbp-20h] BYREF

  nptr[0] = a1;
  nptr[2] = (char *)__readfsqword(0x28u);
  return a1 && (unsigned char)sub_10860(nptr, a3) && (unsigned int)sub_27300(nptr[0]) == 0;
}



// Function: insert_num @ 0x11b60
long long *insert_num(long long a1, int *a2, long long a3)
{
  long long *v3; // r12
  char *v5; // r13
  long long *v7; // rax
  int v8; // ebp
  const char *v9; // r8
  const char *v10; // rcx
  int v11; // [rsp+4h] [rbp-34h] BYREF
  char *v12; // [rsp+8h] [rbp-30h]
  long long v13[5]; // [rsp+10h] [rbp-28h] BYREF

  v3 = 0;
  v13[1] = __readfsqword(0x28u);
  if ( (unsigned char)sub_107D0(a1, a2) )
  {
    v5 = v12;
    if ( sub_11AE0(v12, (long long)v13, &v11) )
    {
      v7 = sub_D490(a3, (long long)v5);
      v8 = v11;
      v3 = v7;
      *((int *)v7 + 14) = v11;
      v7[8] = v13[0];
      if ( (qword_3E660 & 1) != 0 )
      {
        __fprintf_chk(stderr, 1, "inserting %s\n", (const char *)v7[1]);
        v9 = " >";
        v10 = "gt";
        if ( v8 )
        {
          v9 = " <";
          v10 = "lt";
          if ( v8 != 1 )
          {
            v9 = " ?";
            if ( v8 == 2 )
              v9 = " =";
            v10 = "?";
            if ( v8 == 2 )
              v10 = "eq";
          }
        }
        __fprintf_chk(stderr, 1, "    type: %s    %s  ", v10, v9);
        __fprintf_chk(stderr, 1, "%lu\n", v3[8]);
      }
    }
  }
  return v3;
}



// Function: parse_links @ 0x11cb0
long long parse_links(long long a1, long long a2, int *a3)
{
  long long *v4; // rax
  long long v5; // rdx

  v4 = sub_11B60(a2, a3, a1);
  if ( v4 )
  {
    v5 = v4[8];
    if ( v5 == 1 )
    {
      *((int *)v4 + 9) = 1065185444;
      return 1;
    }
    else
    {
      if ( v5 == 2 )
        *((int *)v4 + 9) = 1008981770;
      else
        *((int *)v4 + 9) = 981668463;
      return 1;
    }
  }
  else
  {
    --*a3;
    return 0;
  }
}



// Function: parse_inum @ 0x11d20
long long parse_inum(long long a1, long long a2, int *a3)
{
  long long *v4; // rax

  v4 = sub_11B60(a2, a3, a1);
  if ( v4 )
  {
    *((int *)v4 + 9) = 897988541;
    *((short *)v4 + 13) = 0;
    *((char *)v4 + 28) = 1;
    return 1;
  }
  else
  {
    --*a3;
    return 0;
  }
}



// Function: parse_size @ 0x11de0
long long parse_size(long long a1, long long a2, int *a3)
{
  char v3; // bl
  long long v4; // rax
  char *v6; // r14
  int v7; // eax
  char *v8; // rax
  long long v9; // r13
  unsigned int v10; // eax
  unsigned int v11; // r15d
  long long *v12; // rax
  int v13; // edx
  long long v14; // rcx
  int v15; // xmm0_4
  char *v17; // rax
  char *v18; // rax
  char *v19; // rax
  int v20; // [rsp+4h] [rbp-54h] BYREF
  long long v21; // [rsp+8h] [rbp-50h] BYREF
  char v22[2]; // [rsp+16h] [rbp-42h] BYREF
  unsigned long long v23; // [rsp+18h] [rbp-40h]

  v23 = __readfsqword(0x28u);
  if ( !a2 )
    return 0;
  v4 = *a3;
  v6 = *(char **)(a2 + 8 * v4);
  if ( !v6 )
    return 0;
  v7 = strlen(*(const char **)(a2 + 8 * v4));
  if ( !v7 )
  {
    v17 = dcgettext(0, "invalid null argument to -size", 5);
    error(1, 0, v17);
LABEL_31:
    v22[0] = v3;
    v22[1] = 0;
    v18 = dcgettext(0, "Invalid argument `%s%s' to -size", 5);
    error(1, 0, v18, v6, v22);
    goto LABEL_32;
  }
  v8 = &v6[v7 - 1];
  v3 = *v8;
  if ( *v8 == 98 )
  {
    *v8 = 0;
    v9 = 512;
    goto LABEL_10;
  }
  if ( v3 > 98 )
  {
    switch ( v3 )
    {
      case 'k':
        *v8 = 0;
        v9 = 1024;
        break;
      case 'w':
        *v8 = 0;
        v9 = 2;
        break;
      case 'c':
        *v8 = 0;
        v9 = 1;
        break;
      default:
        goto LABEL_32;
    }
LABEL_10:
    LOBYTE(v10) = sub_11AE0(v6, (long long)&v21, &v20);
    v11 = v10;
    if ( (char)v10 )
    {
      v12 = sub_D490(a1, (long long)v6);
      v13 = v20;
      v14 = v21;
      *((int *)v12 + 15) = v9;
      *((int *)v12 + 14) = v13;
      if ( v13 )
      {
        v15 = 1008981770;
        if ( v13 == 1 )
        {
          v15 = 1063675494;
          if ( (unsigned long long)(v14 * v9) <= 0x5000 )
            v15 = 1036831949;
        }
      }
      else
      {
        v15 = 1036831949;
        if ( (unsigned long long)(v14 * v9) <= 0x5000 )
          v15 = 1063675494;
      }
      v12[8] = v14;
      *((short *)v12 + 13) = 1;
      *((int *)v12 + 9) = v15;
      ++*a3;
      return v11;
    }
    goto LABEL_31;
  }
  if ( v3 == 71 )
  {
    *v8 = 0;
    v9 = 0x40000000;
    goto LABEL_10;
  }
  if ( v3 <= 71 )
  {
    if ( (unsigned char)(v3 - 48) > 9u )
      goto LABEL_32;
    v9 = 512;
    v3 = 0;
    goto LABEL_10;
  }
  if ( v3 == 77 )
  {
    *v8 = 0;
    v9 = 0x100000;
    goto LABEL_10;
  }
LABEL_32:
  v19 = dcgettext(0, "invalid -size type `%c'", 5);
  error(1, 0, v19, (unsigned int)v3);
  return sub_12080();
}



// Function: collect_arg_stat_info @ 0x12080
long long collect_arg_stat_info(long long a1, int *a2, long long a3, long long *a4)
{
  unsigned int v6; // r12d
  int *v8; // rax
  long long v9; // [rsp+0h] [rbp-38h]

  v6 = sub_107D0(a1, a2);
  if ( (char)v6 )
  {
    *a4 = v9;
    if ( (unsigned int)qword_3E670(v9, a3) )
    {
      v8 = __errno_location();
      sub_E310(*v8, v9);
    }
  }
  else
  {
    *a4 = 0;
  }
  return v6;
}



// Function: parse_newer @ 0x12110
long long parse_newer(long long a1, long long a2, int *a3)
{
  unsigned int v4; // r12d
  float *v6; // rbp
  long long v7; // rax
  long long v8; // rdi
  long long v9; // rdx
  long long v10; // [rsp+8h] [rbp-D0h] BYREF
  char v11[88]; // [rsp+10h] [rbp-C8h] BYREF
  long long v12; // [rsp+68h] [rbp-70h]
  unsigned long long v13; // [rsp+A8h] [rbp-30h]

  v13 = __readfsqword(0x28u);
  sub_D860();
  v4 = sub_12080(a2, a3, (long long)v11, &v10);
  if ( (char)v4 )
  {
    v6 = (float *)sub_D490(a1, v10);
    v7 = sub_25C60(v11);
    *((long long *)v6 + 7) = 3;
    v8 = v12;
    *((long long *)v6 + 8) = v7;
    *((long long *)v6 + 9) = v9;
    v6[9] = sub_10760(v8);
  }
  return v4;
}



// Function: parse_cnewer @ 0x121d0
long long parse_cnewer(long long a1, long long a2, int *a3)
{
  unsigned int v4; // r12d
  long long *v6; // rax
  long long *v7; // rbp
  long long v8; // rax
  long long v9; // rdi
  long long v10; // rdx
  long long v11; // [rsp+8h] [rbp-D0h] BYREF
  char v12[88]; // [rsp+10h] [rbp-C8h] BYREF
  long long v13; // [rsp+68h] [rbp-70h]
  unsigned long long v14; // [rsp+A8h] [rbp-30h]

  v14 = __readfsqword(0x28u);
  sub_D860();
  v4 = sub_12080(a2, a3, (long long)v12, &v11);
  if ( (char)v4 )
  {
    v6 = sub_D490(a1, v11);
    *((int *)v6 + 14) = 2;
    v7 = v6;
    v8 = sub_25C60(v12);
    *((int *)v7 + 15) = 0;
    v9 = v13;
    v7[8] = v8;
    v7[9] = v10;
    *((float *)v7 + 9) = sub_10760(v9);
  }
  return v4;
}



// Function: parse_anewer @ 0x122a0
long long parse_anewer(long long a1, long long a2, int *a3)
{
  unsigned int v4; // r12d
  long long *v6; // rax
  long long *v7; // rbp
  long long v8; // rax
  long long v9; // rdi
  long long v10; // rdx
  long long v11; // [rsp+8h] [rbp-D0h] BYREF
  char v12[88]; // [rsp+10h] [rbp-C8h] BYREF
  long long v13; // [rsp+68h] [rbp-70h]
  unsigned long long v14; // [rsp+A8h] [rbp-30h]

  v14 = __readfsqword(0x28u);
  sub_D860();
  v4 = sub_12080(a2, a3, (long long)v12, &v11);
  if ( (char)v4 )
  {
    v6 = sub_D490(a1, v11);
    *((int *)v6 + 14) = 0;
    v7 = v6;
    v8 = sub_25C60(v12);
    *((int *)v7 + 15) = 0;
    v9 = v13;
    v7[8] = v8;
    v7[9] = v10;
    *((float *)v7 + 9) = sub_10760(v9);
  }
  return v4;
}



// Function: parse_samefile @ 0x12370
long long parse_samefile(long long a1, long long a2, int *a3)
{
  unsigned int v5; // r12d
  int v7; // edx
  int v8; // ecx
  int v9; // r8d
  int v10; // r9d
  long long v11; // r15
  int v12; // ebp
  int v13; // eax
  long long *v14; // rax
  long long v15; // rdx
  long long v16; // rdx
  int v17; // edi
  __m128i si128; // xmm1
  __m128i v19; // xmm2
  __m128i v20; // xmm3
  __m128i v21; // xmm4
  __m128i v22; // xmm5
  __m128i v23; // xmm6
  __m128i v24; // xmm7
  __m128i v25; // xmm0
  int *v26; // rax
  long long v27; // [rsp+8h] [rbp-170h] BYREF
  __m128i v28; // [rsp+10h] [rbp-168h] BYREF
  __m128i v29; // [rsp+20h] [rbp-158h]
  __m128i v30; // [rsp+30h] [rbp-148h]
  __m128i v31; // [rsp+40h] [rbp-138h]
  __m128i v32; // [rsp+50h] [rbp-128h]
  __m128i v33; // [rsp+60h] [rbp-118h]
  __m128i v34; // [rsp+70h] [rbp-108h]
  __m128i v35; // [rsp+80h] [rbp-F8h]
  __m128i v36; // [rsp+90h] [rbp-E8h]
  struct stat buf; // [rsp+A0h] [rbp-D8h] BYREF
  unsigned long long v38; // [rsp+138h] [rbp-40h]

  v38 = __readfsqword(0x28u);
  sub_D860();
  v5 = sub_12080(a2, a3, (long long)&v28, &v27);
  if ( (char)v5 )
  {
    sub_D860();
    v11 = v27;
    if ( !dword_3E668 )
    {
      if ( byte_3E678 )
      {
        v13 = sub_15CC0(v27, 0x20000, v7, v8, v9, v10);
        v12 = v13;
        if ( v13 < 0 )
          goto LABEL_8;
        goto LABEL_10;
      }
      if ( (v29.m128i_i16[4] & 0xF000) == 0xA000 )
      {
        v12 = -2;
LABEL_8:
        v14 = sub_D490(a1, v11);
        v15 = v28.m128i_i64[1];
        *((int *)v14 + 18) = v12;
        v14[7] = v15;
        v16 = v28.m128i_i64[0];
        *((int *)v14 + 9) = 1008981770;
        v14[8] = v16;
        *((short *)v14 + 13) = 1;
        return v5;
      }
    }
    v13 = sub_15CC0(v27, 0, v7, v8, v9, v10);
    v12 = v13;
    if ( v13 < 0 )
      goto LABEL_8;
LABEL_10:
    if ( fstat(v13, &buf) || (unsigned int)qword_3E670(v11, &v28) )
    {
      v26 = __errno_location();
      sub_E310(*v26, v11);
    }
    if ( dword_3E668 || byte_3E678 )
    {
      si128 = _mm_load_si128((const __m128i *)&buf.st_nlink);
      v19 = _mm_load_si128((const __m128i *)&buf.st_gid);
      v20 = _mm_load_si128((const __m128i *)&buf.st_size);
      v21 = _mm_load_si128((const __m128i *)&buf.st_blocks);
      v28 = _mm_load_si128((const __m128i *)&buf);
      v22 = _mm_load_si128((const __m128i *)&buf.st_atim.tv_nsec);
      v23 = _mm_load_si128((const __m128i *)&buf.st_mtim.tv_nsec);
      v29 = si128;
      v24 = _mm_load_si128((const __m128i *)&buf.st_ctim.tv_nsec);
      v25 = _mm_load_si128((const __m128i *)&buf.__unused[1]);
      v30 = v19;
      v31 = v20;
      v32 = v21;
      v33 = v22;
      v34 = v23;
      v35 = v24;
      v36 = v25;
    }
    else if ( (v29.m128i_i16[4] & 0xF000) == 0xA000 || *(_OWORD *)&v28 != *(_OWORD *)&buf.st_dev )
    {
      v17 = v12;
      v12 = -1;
      close(v17);
    }
    goto LABEL_8;
  }
  return v5;
}



// Function: insert_regex @ 0x125d0
long long insert_regex(long long a1, int *a2, long long a3, int a4)
{
  long long v4; // r13
  reg_syntax_t v5; // rbx
  unsigned int v6; // r12d
  long long *v8; // rbp
  long long v9; // rax
  struct re_pattern_buffer *v10; // r13
  long long v11; // rax
  size_t v12; // rax
  const char *v13; // rbx
  char *v14; // rax
  int *v15; // rdx
  char *s; // [rsp+0h] [rbp-38h]

  while ( 1 )
  {
    v4 = a3;
    v5 = a4;
    v6 = sub_107D0(a1, a2);
    if ( !(char)v6 )
      break;
    v8 = sub_D450(v4, (long long)sub_96F0, (long long)s);
    *((short *)v8 + 13) = 0;
    v9 = sub_26990(64);
    v8[7] = v9;
    v10 = (struct re_pattern_buffer *)v9;
    *(long long *)(v9 + 8) = 100;
    v11 = sub_26990(100);
    v10->fastmap = 0;
    v10->buffer = (unsigned char *)v11;
    re_set_syntax(v5);
    v10->syntax = v5;
    v10->translate = 0;
    v12 = strlen(s);
    v13 = re_compile_pattern(s, v12, v10);
    if ( !v13 )
    {
      *((float *)v8 + 9) = sub_110C0(s, 1);
      return v6;
    }
    v14 = dcgettext(0, "failed to compile regular expression '%s': %s", 5);
    error(1, 0, v14, s, v13);
    a4 = dword_3E67C;
    a1 = 0;
    a2 = v15;
    a3 = 1;
  }
  return v6;
}



// Function: parse_iregex @ 0x12710
long long parse_iregex(long long a1, long long a2, int *a3)
{
  return sub_125D0(a2, a3, a1, dword_3E67C | 0x400000u);
}



// Function: stream_is_tty @ 0x12740
bool stream_is_tty(FILE *a1)
{
  int v1; // edi
  bool result; // al

  v1 = fileno(a1);
  result = 0;
  if ( v1 != -1 )
    return isatty(v1) != 0;
  return result;
}



// Function: open_output_file @ 0x12770
bool open_output_file(char *s1, long long a2)
{
  FILE *v2; // rax
  bool result; // al
  char *v4; // rax
  FILE *v5; // rdi
  char *v6; // rax
  FILE *v7; // rdi
  int *v8; // rax

  *(long long *)a2 = 0;
  *(long long *)(a2 + 32) = sub_24AC0(0);
  if ( !strcmp(s1, "/dev/stderr") )
  {
    *(long long *)(a2 + 8) = stderr;
    v6 = dcgettext(0, "standard error", 5);
    v7 = *(FILE **)(a2 + 8);
    *(long long *)(a2 + 16) = v6;
    result = sub_12740(v7);
    *(char *)(a2 + 24) = result;
  }
  else if ( !strcmp(s1, "/dev/stdout") )
  {
    *(long long *)(a2 + 8) = stdout;
    v4 = dcgettext(0, "standard output", 5);
    v5 = *(FILE **)(a2 + 8);
    *(long long *)(a2 + 16) = v4;
    result = sub_12740(v5);
    *(char *)(a2 + 24) = result;
  }
  else
  {
    v2 = (FILE *)sub_E580((long long *)qword_3E610, s1);
    *(long long *)(a2 + 16) = s1;
    *(long long *)(a2 + 8) = v2;
    if ( !v2 )
    {
      v8 = __errno_location();
      sub_E410(*v8, (long long)s1);
    }
    result = sub_12740(v2);
    *(char *)(a2 + 24) = result;
  }
  return result;
}



// Function: open_stdout @ 0x12870
bool open_stdout(long long a1)
{
  return sub_12770("/dev/stdout", a1);
}



// Function: parse_printf @ 0x12880
long long parse_printf(long long a1, long long a2, int *a3)
{
  int v4; // r12d
  long long result; // rax
  long long v6; // [rsp+8h] [rbp-60h] BYREF
  __m128i v7[2]; // [rsp+10h] [rbp-58h] BYREF
  unsigned long long v8; // [rsp+38h] [rbp-30h]

  v4 = *a3;
  v8 = __readfsqword(0x28u);
  result = sub_10790(a2, a3, &v6);
  if ( (char)result )
  {
    sub_12870((long long)v7);
    result = sub_FEF0(v7, a1, v6);
    if ( !(char)result )
      *a3 = v4;
  }
  return result;
}



// Function: parse_print @ 0x12910
long long parse_print(long long a1)
{
  long long *v1; // rax

  v1 = sub_D4D0(a1);
  *((int *)v1 + 6) = 257;
  sub_12870((long long)(v1 + 7));
  return 1;
}



// Function: insert_fprint @ 0x12940
long long insert_fprint(long long a1, char *a2)
{
  long long *v2; // rax
  long long *v3; // rbx

  v2 = sub_D490(a1, (long long)a2);
  v3 = v2;
  if ( a2 )
    sub_12770(a2, (long long)(v2 + 7));
  else
    sub_12870((long long)(v2 + 7));
  *((int *)v3 + 6) = 257;
  *((int *)v3 + 9) = 1065353216;
  return 1;
}



// Function: parse_print0 @ 0x12990
long long parse_print0(long long a1)
{
  return sub_12940(a1, 0);
}



// Function: parse_fprint0 @ 0x129a0
long long parse_fprint0(long long a1, long long a2, int *a3)
{
  long long result; // rax
  char *v5; // [rsp+0h] [rbp-28h]

  result = sub_107D0(a2, a3);
  if ( (char)result )
  {
    result = sub_12940(a1, v5);
    if ( !(char)result )
      --*a3;
  }
  return result;
}



// Function: insert_fls @ 0x12a10
long long insert_fls(long long a1, char *a2)
{
  long long *v2; // rax
  long long *v3; // rbx

  v2 = sub_D4D0(a1);
  v3 = v2;
  if ( a2 )
    sub_12770(a2, (long long)(v2 + 7));
  else
    sub_12870((long long)(v2 + 7));
  *((int *)v3 + 9) = 1065353216;
  *((short *)v3 + 12) = 257;
  return 1;
}



// Function: parse_ls @ 0x12a60
long long parse_ls(long long a1)
{
  return sub_12A10(a1, 0);
}



// Function: parse_fls @ 0x12a70
long long parse_fls(long long a1, long long a2, int *a3)
{
  long long result; // rax
  char *v5; // [rsp+0h] [rbp-28h]

  result = sub_107D0(a2, a3);
  if ( (char)result )
  {
    result = sub_12A10(a1, v5);
    if ( !(char)result )
      --*a3;
  }
  return result;
}



// Function: parse_fprintf @ 0x12ae0
long long parse_fprintf(long long a1, long long a2, int *a3)
{
  int v4; // r13d
  long long result; // rax
  char *s1; // [rsp+0h] [rbp-68h]
  long long v7; // [rsp+8h] [rbp-60h] BYREF
  __m128i v8[2]; // [rsp+10h] [rbp-58h] BYREF
  unsigned long long v9; // [rsp+38h] [rbp-30h]

  v4 = *a3;
  v9 = __readfsqword(0x28u);
  if ( !(unsigned char)sub_107D0(a2, a3)
    || !(unsigned char)sub_10790(a2, a3, &v7)
    || (sub_12770(s1, (long long)v8), v4 = *a3, result = sub_FEF0(v8, a1, v7), !(char)result) )
  {
    *a3 = v4;
    return 0;
  }
  return result;
}



// Function: parse_fprint @ 0x12b90
long long parse_fprint(long long a1, long long a2, int *a3)
{
  unsigned int v3; // r12d
  long long *v5; // rbp
  char *s1; // [rsp+0h] [rbp-28h]

  v3 = sub_107D0(a2, a3);
  if ( (char)v3 )
  {
    v5 = sub_D490(a1, (long long)s1);
    sub_12770(s1, (long long)(v5 + 7));
    *((int *)v5 + 6) = 257;
    *((int *)v5 + 9) = 1065353216;
  }
  return v3;
}



// Function: parse_perm @ 0x12c20
long long parse_perm(long long a1, long long a2, int *a3)
{
  unsigned int v4; // r12d
  char *v5; // rdi
  int v6; // r14d
  void *v7; // rbp
  unsigned int v8; // eax
  long long v9; // rbx
  long long v10; // rax
  long long v11; // rbx
  long long *v12; // rax
  char *v14; // rax
  long long v15; // r12
  char *v16; // rax
  int v17; // [rsp+4h] [rbp-54h]
  long long v18; // [rsp+8h] [rbp-50h]
  char *v19; // [rsp+10h] [rbp-48h]

  v4 = sub_107D0(a2, a3);
  if ( !(char)v4 )
    return v4;
  if ( *v19 == 45 )
  {
    v5 = v19 + 1;
    v6 = 0;
    v17 = 1045220557;
  }
  else if ( *v19 == 47 )
  {
    v5 = v19 + 1;
    v6 = 1;
    v17 = 1050253722;
  }
  else
  {
    v5 = v19;
    v6 = 2;
    v17 = 1008981770;
  }
  v7 = (void *)sub_1D600(v5);
  if ( v7 && (*v19 != 43 || (unsigned char)(v19[1] - 48) > 7u) )
  {
    v8 = sub_1D9D0(0, 0, 0, v7, 0);
    v9 = v8;
    HIDWORD(v18) = v8;
    v10 = sub_1D9D0(0, 1, 0, v7, 0);
    LODWORD(v18) = v10;
    v11 = (v10 << 32) | v9;
    free(v7);
    if ( *v19 == 47 && !v18 )
    {
      v6 = 0;
      v14 = dcgettext(
              0,
              "warning: you have specified a mode pattern %s (which is equivalent to /000). The meaning of -perm /000 has"
              " now been changed to be consistent with -perm -000; that is, while it used to match no files, it now matches all files.",
              5);
      error(0, 0, v14);
      v17 = 1065329728;
    }
    v12 = sub_D490(a1, (long long)v19);
    *((int *)v12 + 14) = v6;
    *(long long *)((char *)v12 + 60) = v11;
    *((int *)v12 + 9) = v17;
    return v4;
  }
  v15 = sub_24E50(0, (unsigned int)dword_3E68C);
  v16 = dcgettext(0, "invalid mode %s", 5);
  error(1, 0, v16, v15);
  return sub_12E20();
}



// Function: check_path_safety @ 0x12e20
unsigned long long check_path_safety(long long a1)
{
  char *v1; // rax
  char *v2; // rbx
  char v3; // al
  char *v4; // rax
  char *v6; // rsi
  long long v7; // r12
  char *v8; // rax
  long long v9; // [rsp+8h] [rbp-40h] BYREF
  size_t n; // [rsp+10h] [rbp-38h] BYREF
  unsigned long long v11; // [rsp+18h] [rbp-30h]

  v11 = __readfsqword(0x28u);
  v1 = getenv("PATH");
  if ( v1 )
  {
    v2 = v1;
    sub_170D0(v1, ":", 1, &v9, &n);
    while ( n )
    {
      v3 = v2[v9];
      if ( n == 1 && v3 == 46 )
        break;
      if ( v3 != 47 )
      {
        v6 = strndup(&v2[v9], n);
        if ( !v6 )
          v6 = &v2[v9];
        v7 = sub_DFB0(0, (long long)v6);
        v8 = dcgettext(
               0,
               "The relative path %s is included in the PATH environment variable, which is insecure in combination with "
               "the %s action of find.  Please remove that entry from $PATH",
               5);
        error(1, 0, v8, v7, a1);
      }
      if ( !(unsigned char)sub_170D0(v2, ":", 0, &v9, &n) )
        return v11 - __readfsqword(0x28u);
    }
    v4 = dcgettext(
           0,
           "The current directory is included in the PATH environment variable, which is insecure in combination with the"
           " %s action of find.  Please remove the current directory from your $PATH (that is, remove \".\", doubled colo"
           "ns, or leading or trailing colons)",
           5);
    error(1, 0, v4, a1);
  }
  return v11 - __readfsqword(0x28u);
}



// Function: insert_exec_ok @ 0x12f60
long long insert_exec_ok(long long a1, long long a2, long long a3, int *a4)
{
  long long (*v4)(); // rbx
  long long v5; // rax
  char *v6; // r14
  char *v7; // r15
  long long *v8; // rbx
  long long v9; // rbp
  int v10; // esi
  const char *v11; // r12
  char *v12; // r13
  int v13; // r15d
  const char *v14; // r12
  char *v15; // rax
  long long *v17; // rbp
  long long *v18; // r13
  int v19; // r14d
  bool v20; // zf
  long long *v21; // r12
  const char **v22; // r14
  long long v23; // r13
  const char *v24; // r15
  size_t v25; // rax
  long long v26; // rcx
  long long v27; // rsi
  long long i; // rax
  char *v29; // rax
  char *v30; // rax
  char *v31; // rax
  char *s; // [rsp+0h] [rbp-A8h]
  int sa; // [rsp+0h] [rbp-A8h]
  long long (*v34)(); // [rsp+8h] [rbp-A0h]
  long long *ptr; // [rsp+18h] [rbp-90h]
  bool v38; // [rsp+33h] [rbp-75h]
  int v39; // [rsp+34h] [rbp-74h]
  long long v40; // [rsp+40h] [rbp-68h]
  const char **v41; // [rsp+48h] [rbp-60h]
  char v42[24]; // [rsp+50h] [rbp-58h] BYREF
  unsigned long long v43; // [rsp+68h] [rbp-40h]

  v4 = *(long long (**)())(a2 + 24);
  v43 = __readfsqword(0x28u);
  v34 = v4;
  if ( !a3 || !*(long long *)(a3 + 8LL * *a4) )
    return 0;
  ptr = sub_D450(a2, (long long)v4, (long long)"(some -exec* arguments)");
  *((int *)ptr + 6) = 257;
  ptr[31] = 0;
  v38 = (char *)v4 != (char *)sub_9530 && (char *)v4 != (char *)sub_94E0;
  if ( v38 )
  {
    *((char *)ptr + 244) = 0;
    if ( v4 == sub_8FF0 )
    {
LABEL_6:
      byte_3E62E = 0;
      sub_12E20(a1);
      goto LABEL_10;
    }
  }
  else
  {
    byte_3E698 = 1;
    *((char *)ptr + 244) = 1;
    if ( v4 == sub_8FF0 || (char *)v4 == (char *)sub_9530 )
      goto LABEL_6;
  }
  if ( !qword_3E5C0 )
    goto LABEL_55;
  ptr[31] = qword_3E5C0;
LABEL_10:
  *((char *)ptr + 56) = 0;
  v39 = *a4;
  v40 = *a4;
  v5 = 8 * v40;
  if ( !*(long long *)(a3 + 8 * v40) )
  {
    v13 = *a4;
    goto LABEL_37;
  }
  v6 = (char *)(a3 + v5 + 8);
  v41 = (const char **)(a3 + v5);
  v7 = *(char **)(a3 + 8LL * *a4);
  s = 0;
  v8 = (long long *)(a3 + v5);
  LODWORD(v9) = 0;
  v10 = 0;
  v11 = "{}";
  LODWORD(v12) = *a4;
  while ( *v7 == 59 )
  {
    if ( !v7[1] )
    {
      v13 = (int)v12;
      goto LABEL_23;
    }
LABEL_14:
    if ( !sub_1CAF0(v7, v11) )
      goto LABEL_31;
    LODWORD(v9) = v9 + 1;
    if ( !(int)v12 && (v34 == sub_8FF0 || (char *)v34 == (char *)sub_9530) )
    {
      v29 = dcgettext(
              0,
              "You may not use {} within the utility name for -execdir and -okdir, because this is a potential security problem.",
              5);
      error(1, 0, v29);
      goto LABEL_54;
    }
    s = v7;
    v10 = 1;
LABEL_19:
    v7 = *(char **)v6;
    LODWORD(v12) = (int)v12 + 1;
    v6 += 8;
    ++v8;
    if ( !v7 )
    {
      v13 = (int)v12;
LABEL_37:
      *a4 = v13;
      free(ptr);
      return 0;
    }
  }
  if ( *v7 != 43 || !v38 || v7[1] || !v10 )
    goto LABEL_14;
  v13 = (int)v12;
  *((char *)ptr + 56) = 1;
LABEL_23:
  if ( v39 == v13 || !*v8 )
    goto LABEL_37;
  if ( *((char *)ptr + 56) )
  {
    v14 = "dir";
    if ( v34 != sub_8FF0 )
      v14 = "";
    if ( (int)v9 > 1 )
    {
      v31 = dcgettext(0, "Only one instance of {} is supported with -exec%s ... +", 5);
      error(1, 0, v31, v14);
    }
    v6 = s;
    if ( strlen(s) != 2 )
    {
      v12 = v42;
      __snprintf_chk(v42, 19, 1, 19, "-exec%s ... {} +", v14);
      v9 = sub_24E50(2, (unsigned int)dword_3E68C);
      v8 = (long long *)sub_24E50(1, (unsigned int)dword_3E68C);
      v11 = (const char *)sub_24E50(0, (unsigned int)dword_3E68C);
      v15 = dcgettext(0, "In %s the %s must appear by itself, but you specified %s", 5);
      error(1, 0, v15, v11, v8, v9);
LABEL_31:
      v10 = 0;
      goto LABEL_19;
    }
  }
  v17 = ptr + 8;
  v18 = ptr + 7;
  if ( (unsigned int)sub_14CA0(ptr + 8, 2048) - 1 <= 1 )
  {
LABEL_54:
    v30 = dcgettext(0, "The environment is too large for exec().", 5);
    error(1, 0, v30);
LABEL_55:
    __assert_fail("NULL != initial_wd", "parser.c", 0xB79u, "insert_exec_ok");
  }
  sub_14D80(v17);
  v19 = v13 - v39;
  v20 = *((char *)ptr + 56) == 0;
  v21 = ptr + 19;
  ptr[16] = sub_A610;
  if ( v20 )
  {
    *((int *)ptr + 60) = v19;
    ptr[14] = "{}";
    ptr[13] = strlen("{}");
    ptr[17] = 0;
    ptr[18] = 0;
    ptr[29] = sub_26990(8LL * v19);
    sub_14DB0(v17, v21, v18);
    v26 = *((int *)ptr + 60);
    if ( (int)v26 > 0 )
    {
      v27 = ptr[29];
      for ( i = 0; i != v26; ++i )
        *(long long *)(v27 + 8 * i) = *(long long *)(a3 + 8 * v40 + 8 * i);
    }
  }
  else
  {
    ptr[29] = 0;
    ptr[14] = 0;
    ptr[15] = v19 - 1;
    ptr[13] = 0;
    ptr[17] = 0;
    ptr[18] = 0;
    sub_14DB0(v17, v21, v18);
    if ( v39 < v13 - 1 )
    {
      sa = v13;
      v22 = v41;
      v23 = a3 + 8 * (v40 + (unsigned int)(v13 - v39 - 2)) + 8;
      do
      {
        v24 = *v22++;
        v25 = strlen(v24);
        sub_15000(v17, v21, v24, v25 + 1, 0, 0, 1);
      }
      while ( v22 != (const char **)v23 );
      v13 = sa;
    }
  }
  if ( *v8 )
    *a4 = v13 + 1;
  else
    *a4 = v13;
  return 1;
}



// Function: parse_okdir @ 0x13550
long long parse_okdir(long long a1, long long a2, int *a3)
{
  return sub_12F60((long long)"-okdir", a1, a2, a3);
}



// Function: parse_or @ 0x135d0
long long parse_or(int *a1)
{
  long long v1; // rax

  v1 = sub_B970(a1);
  *(long long *)(v1 + 16) = 0x200000003LL;
  *(long long *)v1 = sub_88C0;
  *(short *)(v1 + 26) = 0;
  return 1;
}



// Function: parse_and @ 0x13610
long long parse_and(int *a1)
{
  long long v1; // rax

  v1 = sub_B970(a1);
  *(long long *)(v1 + 16) = 0x300000003LL;
  *(long long *)v1 = sub_8870;
  *(short *)(v1 + 26) = 0;
  return 1;
}



// Function: parse_comma @ 0x13650
long long parse_comma(int *a1)
{
  long long v1; // rax

  v1 = sub_B970(a1);
  *(long long *)(v1 + 16) = 0x100000003LL;
  *(long long *)v1 = sub_8CD0;
  *(short *)(v1 + 26) = 0;
  *(int *)(v1 + 36) = 1065353216;
  return 1;
}



// Function: parse_closeparen @ 0x13690
long long parse_closeparen(int *a1)
{
  long long v1; // rax

  v1 = sub_B970(a1);
  *(long long *)(v1 + 16) = 5;
  *(long long *)v1 = sub_8C30;
  *(short *)(v1 + 26) = 0;
  return 1;
}



// Function: check_name_arg @ 0x136c0
long long check_name_arg(long long a1, long long a2)
{
  const char *v2; // rdx
  long long v3; // rbx
  long long v4; // rbp
  long long v5; // r12
  char *v6; // rax

  if ( (unsigned char)sub_106F0() && strchr(v2, 47) )
  {
    v3 = sub_DFB0(2, a2);
    v4 = sub_DFB0(1, (long long)"/");
    v5 = sub_DFB0(0, a1);
    v6 = dcgettext(
           0,
           "warning: %s matches against basenames only, but the given pattern contains a directory separator (%s), thus t"
           "he expression will evaluate to false all the time.  Did you mean %s?",
           5);
    error(0, 0, v6, v5, v4, v3);
  }
  return 1;
}



// Function: parse_name @ 0x13750
long long parse_name(long long a1, long long a2, int *a3)
{
  int v4; // r13d
  unsigned int v5; // r12d
  long long *v7; // rbx
  char *s; // [rsp+0h] [rbp-38h]

  v4 = *a3;
  v5 = sub_107D0(a2, a3);
  if ( (char)v5 )
  {
    sub_11020();
    v5 = sub_136C0((long long)"-name", (long long)"-wholename");
    if ( (char)v5 )
    {
      v7 = sub_D490(a1, (long long)s);
      *((short *)v7 + 13) = 0;
      v7[7] = s;
      *((float *)v7 + 9) = sub_110C0(s, 0);
    }
    else
    {
      *a3 = v4;
    }
  }
  return v5;
}



// Function: parse_iname @ 0x13810
long long parse_iname(long long a1, long long a2, int *a3)
{
  unsigned int v4; // r12d
  long long *v6; // r13
  char *s; // [rsp+0h] [rbp-28h]

  sub_11020();
  if ( (unsigned char)sub_107D0(a2, a3) && (v4 = sub_136C0((long long)"-iname", (long long)"-iwholename"), (char)v4) )
  {
    v6 = sub_D490(a1, (long long)s);
    *((short *)v6 + 13) = 0;
    v6[7] = s;
    *((float *)v6 + 9) = sub_110C0(s, 0);
  }
  else
  {
    return 0;
  }
  return v4;
}



// Function: parse_group @ 0x138d0
long long parse_group(long long a1, long long a2, int *a3)
{
  unsigned int v3; // r12d
  struct group *v5; // rbx
  __gid_t gr_gid; // ebx
  long long *v7; // rax
  int v8; // xmm0_4
  bool v9; // cc
  int v10; // eax
  long long v11; // r12
  char *v12; // rax
  char *v13; // rax
  long long v14; // rbx
  long long v15; // r12
  char *v16; // rax
  char *name; // [rsp+0h] [rbp-38h]

  v3 = sub_107D0(a2, a3);
  if ( !(char)v3 )
    return v3;
  v5 = getgrnam(name);
  endgrent();
  if ( v5 )
  {
    gr_gid = v5->gr_gid;
LABEL_5:
    v7 = sub_D490(a1, (long long)name);
    v8 = 1065185444;
    v9 = v7[8] <= 0x63u;
    *((int *)v7 + 14) = gr_gid;
    if ( !v9 )
      v8 = 1045220557;
    *((int *)v7 + 9) = v8;
    return v3;
  }
  v10 = strspn(name, "0123456789");
  if ( !v10 )
  {
    if ( *name )
    {
      v11 = sub_24E50(0, (unsigned int)dword_3E68C);
      v12 = dcgettext(0, "%s is not the name of an existing group", 5);
      error(1, 0, v12, v11);
    }
    v13 = dcgettext(0, "argument to -group is empty, but should be a group name", 5);
    error(1, 0, v13);
  }
  if ( !name[v10] )
  {
    gr_gid = sub_16F40(name);
    goto LABEL_5;
  }
  v14 = sub_24E50(1, (unsigned int)dword_3E68C);
  v15 = sub_24E50(0, (unsigned int)dword_3E68C);
  v16 = dcgettext(
          0,
          "%s is not the name of an existing group and it does not look like a numeric group ID because it has the unexpected suffix %s",
          5);
  error(1, 0, v16, v15, v14);
  return sub_13A70((char *)&dword_0 + 1);
}



// Function: estimate_fstype_success_rate @ 0x13a70
unsigned long long estimate_fstype_success_rate(char *s1)
{
  const char *v2; // rax
  struct stat _0; // [rsp+0h] [rbp+0h] BYREF
  unsigned long long vars98; // [rsp+98h] [rbp+98h]

  vars98 = __readfsqword(0x28u);
  if ( !stat("/", &_0) )
  {
    v2 = (const char *)sub_104C0((long long *)&_0);
    strcmp(s1, v2);
  }
  return vars98 - __readfsqword(0x28u);
}



// Function: parse_daystart @ 0x13b10
long long parse_daystart()
{
  struct tm *v1; // rax
  time_t v2; // rdx

  if ( byte_3E658 )
    return 1;
  timer += 86400LL;
  qword_3E650 = 0;
  v1 = localtime(&timer);
  if ( v1 )
    v2 = 3600 * v1->tm_hour + v1->tm_sec + 60 * v1->tm_min;
  else
    v2 = timer % 86400;
  byte_3E658 = 1;
  timer -= v2;
  return 1;
}



// Function: parse_time @ 0x13bc0
long long parse_time(long long a1, long long a2, int *a3)
{
  unsigned int v4; // r12d
  char *v6; // r14
  long long v7; // r9
  long long *v8; // rax
  long long v9; // rdi
  long long *v10; // rbx
  const char *v11; // r8
  char *v12; // rcx
  char *v13; // rax
  char *v14; // rax
  char *v15; // rax
  int v16; // [rsp+Ch] [rbp-7Ch]
  int v17; // [rsp+1Ch] [rbp-6Ch] BYREF
  char *v18; // [rsp+20h] [rbp-68h] BYREF
  time_t timer; // [rsp+28h] [rbp-60h] BYREF
  __m128i v20; // [rsp+30h] [rbp-58h] BYREF
  long long v21; // [rsp+40h] [rbp-48h]
  unsigned long long v22; // [rsp+48h] [rbp-40h]

  v22 = __readfsqword(0x28u);
  dcgettext(0, "arithmetic overflow while converting %s days to a number of seconds", 5);
  v16 = *a3;
  v4 = sub_107D0(a2, a3);
  if ( (char)v4 )
  {
    v6 = v18;
    if ( (unsigned char)sub_10860(&v18, &v17) && v17 == 1 )
      v7 += 86399;
    v18 = v6;
    v4 = sub_11650(v6, (long long)&v20, v7, 86400.0);
    if ( (char)v4 )
    {
      v8 = sub_D490(a1, (long long)v6);
      v9 = v20.m128i_i64[1];
      v10 = v8;
      *(__m128i *)(v8 + 7) = _mm_load_si128(&v20);
      v8[9] = v21;
      *((float *)v8 + 9) = sub_10760(v9);
      if ( (qword_3E660 & 1) != 0 )
      {
        __fprintf_chk(stderr, 1, "inserting %s\n", (const char *)v10[1]);
        if ( v20.m128i_i32[1] )
        {
          if ( v20.m128i_i32[1] == 1 )
          {
            v11 = " <";
            v12 = (char *)"lt";
          }
          else
          {
            v11 = " ?";
            v12 = "?";
            if ( v20.m128i_i32[1] == 2 )
            {
              __fprintf_chk(stderr, 1, "    type: %s    %s  ", "eq", ">=");
              timer = v10[8];
              v14 = ctime(&timer);
              __fprintf_chk(stderr, 1, "%lu %s", v10[8], v14);
              timer = v10[8] + 86400LL;
              v15 = ctime(&timer);
              __fprintf_chk(stderr, 1, "                 <  %lu %s", timer, v15);
              return v4;
            }
          }
          __fprintf_chk(stderr, 1, "    type: %s    %s  ", v12, v11);
        }
        else
        {
          __fprintf_chk(stderr, 1, "    type: %s    %s  ", "gt", " >");
        }
        timer = v10[8];
        v13 = ctime(&timer);
        __fprintf_chk(stderr, 1, "%lu %s", v10[8], v13);
      }
    }
    else
    {
      *a3 = v16;
    }
  }
  return v4;
}



// Function: parse_negate @ 0x13e80
long long parse_negate(int *a1)
{
  long long v1; // rax

  v1 = sub_B990(a1, 0);
  *(long long *)(v1 + 16) = 0x400000002LL;
  *(long long *)v1 = sub_92F0;
  *(short *)(v1 + 26) = 0;
  return 1;
}



// Function: parse_openparen @ 0x13ec0
long long parse_openparen(int *a1)
{
  long long v1; // rax

  v1 = sub_B990(a1, 0);
  *(long long *)(v1 + 16) = 4;
  *(long long *)v1 = sub_9590;
  *(short *)(v1 + 26) = 0;
  return 1;
}



// Function: parse_newerXY @ 0x13f00
long long parse_newerXY(long long a1, long long a2, int *a3)
{
  long long v3; // rax
  long long v6; // r14
  int v7; // r13d
  unsigned int v8; // r9d
  char v9; // r15
  char *v10; // rax
  char *v11; // rax
  int v12; // r13d
  long long v13; // rsi
  long long v14; // r12
  char *v16; // rax
  int v17; // eax
  float v18; // xmm0_4
  bool (*v19)(long long, long long, long long); // rax
  char *v20; // rax
  long long v21; // r12
  char *v22; // rax
  long long v23; // r12
  char *v24; // rax
  int *v25; // rax
  long long v26; // [rsp+0h] [rbp-F8h]
  char v27; // [rsp+Eh] [rbp-EAh]
  char v28[146]; // [rsp+20h] [rbp-D8h] BYREF
  char s[6]; // [rsp+B2h] [rbp-46h] BYREF
  unsigned long long v30; // [rsp+B8h] [rbp-40h]

  v30 = __readfsqword(0x28u);
  if ( !a2 )
    return 0;
  v3 = *a3;
  v6 = *(long long *)(a2 + 8 * v3);
  v7 = *a3;
  v26 = 8 * v3;
  if ( !v6 || strlen((const char *)v6) != 8 )
    return 0;
  strcpy(s, "aBcmt");
  if ( strncmp("-newer", (const char *)v6, 6u) )
    goto LABEL_32;
  v9 = *(char *)(v6 + 6);
  v27 = *(char *)(v6 + 7);
  LOBYTE(v8) = v27 == 66 || v9 == 66;
  if ( (char)v8 )
  {
    v16 = dcgettext(0, "This system does not provide a way to find the birth time of a file.", 5);
    error(0, 0, v16);
    return 0;
  }
  if ( v9 != 116 )
  {
    v10 = strchr(s, v9);
    v8 = 0;
    if ( v10 )
    {
      v11 = strchr(s, v27);
      v8 = 0;
      if ( v11 )
      {
        v12 = v7 + 1;
        v13 = *(long long *)(a2 + v26 + 8);
        if ( v13 )
        {
          *a3 = v12;
          v14 = (long long)sub_D490(a1, v13);
          switch ( v9 )
          {
            case 'c':
              goto LABEL_30;
            case 'm':
              v17 = 3;
              break;
            case 'a':
              v17 = 0;
              break;
            default:
              if ( strchr(s, v9) )
                __assert_fail("0", "parser.c", 0x64Cu, "parse_newerXY");
              __assert_fail("strchr (validchars, x)", "parser.c", 0x64Bu, "parse_newerXY");
          }
          while ( 1 )
          {
            *(int *)(v14 + 56) = v17;
            if ( v27 != 116 )
              break;
            if ( (unsigned char)sub_22C40(v14 + 64, *(long long *)(a2 + 8LL * *a3), &qword_3E638) )
              goto LABEL_23;
            v14 = sub_24E50(0, (unsigned int)dword_3E68C);
            v20 = dcgettext(0, "I cannot figure out how to interpret %s as a date or time", 5);
            error(1, 0, v20, v14);
LABEL_30:
            v17 = 2;
          }
          sub_D860();
          if ( !(unsigned int)qword_3E670(*(long long *)(a2 + 8LL * *a3), v28) )
          {
            if ( sub_11A40((long long)v28, v27, (long long *)(v14 + 64)) )
            {
LABEL_23:
              *(int *)(v14 + 60) = 0;
              v18 = sub_10760(*(long long *)(v14 + 64));
              v19 = *(bool (**)(long long, long long, long long))v14;
              *(float *)(v14 + 36) = v18;
              ++*a3;
              if ( !v19 )
                __assert_fail("our_pred->pred_func != NULL", "parser.c", 0x66Fu, "parse_newerXY");
              if ( v19 != sub_9370 )
                __assert_fail("our_pred->pred_func == pred_newerXY", "parser.c", 0x670u, "parse_newerXY");
              v8 = *(unsigned char *)(v14 + 26);
              if ( !(char)v8 )
                __assert_fail("our_pred->need_stat", "parser.c", 0x671u, "parse_newerXY");
              return v8;
            }
            v23 = sub_DFB0(0, *(long long *)(a2 + 8LL * *a3));
            v24 = dcgettext(0, "Cannot obtain birth time of file %s", 5);
            error(1, 0, v24, v23);
          }
          v25 = __errno_location();
          sub_E310(*v25, *(long long *)(a2 + 8LL * *a3));
        }
        v21 = sub_24E50(0, (unsigned int)dword_3E68C);
        v22 = dcgettext(0, "The %s test needs an argument", 5);
        error(1, 0, v22, v21);
LABEL_32:
        __assert_fail("0 == strncmp (\"-newer\", argv[*arg_ptr], 6)", "parser.c", 0x613u, "parse_newerXY");
      }
    }
  }
  return v8;
}



// Function: parse_regextype @ 0x14370
long long parse_regextype(long long a1, long long a2, int *a3)
{
  char *s2; // [rsp+0h] [rbp-28h]

  if ( !(unsigned char)sub_107D0(a2, a3) )
    return 0;
  dword_3E67C = sub_16D30(s2);
  return sub_14350(a2, a3);
}



// Function: insert_depthspec @ 0x14400
long long insert_depthspec(long long a1, long long a2, int *a3, int *a4)
{
  long long v7; // r13
  int v8; // eax
  long long v9; // rsi
  int v10; // eax
  long long v12; // rbx
  char *v13; // rax
  char *s; // [rsp+0h] [rbp-38h]

  v7 = *(long long *)(a2 + 8LL * *a3 - 8);
  if ( (unsigned char)sub_107D0(a2, a3) )
  {
    v8 = strspn(s, "0123456789");
    v9 = (unsigned int)dword_3E68C;
    if ( v8 > 0 && !s[v8] )
    {
      v10 = sub_16F40(s);
      *a4 = v10;
      if ( v10 >= 0 )
        return sub_14350(a2, a3);
      v9 = (unsigned int)dword_3E68C;
    }
    v12 = sub_24E50(0, v9);
    v13 = dcgettext(0, "Expected a positive decimal integer argument to %s, but got %s", 5);
    error(1, 0, v13, v7, v12);
  }
  return 0;
}



// Function: parse_mindepth @ 0x14510
long long parse_mindepth(long long a1, long long a2, int *a3)
{
  return sub_14400(a1, a2, a3, &dword_3E628);
}



// Function: parse_depth @ 0x14530
long long parse_depth(long long a1, long long a2, long long a3)
{
  word_3E620 = 256;
  return sub_14350(a2, a3);
}



// Function: parse_d @ 0x14550
long long parse_d(long long a1, long long a2, long long a3)
{
  char *v5; // rax

  if ( (unsigned char)sub_106F0() )
  {
    v5 = dcgettext(
           0,
           "warning: the -d option is deprecated; please use -depth instead, because the latter is a POSIX-compliant feature.",
           5);
    error(0, 0, v5);
  }
  return sub_14530(a1, a2, a3);
}



// Function: parse_ignore_race @ 0x145b0
long long parse_ignore_race(long long a1, long long a2, long long a3)
{
  byte_3E62E = 1;
  return sub_14350(a2, a3);
}



// Function: parse_xdev @ 0x145d0
long long parse_xdev(long long a1, long long a2, long long a3)
{
  byte_3E62D = 1;
  return sub_14350(a2, a3);
}



// Function: parse_noleaf @ 0x145f0
long long parse_noleaf(long long a1, long long a2, long long a3)
{
  byte_3E62C = 1;
  return sub_14350(a2, a3);
}



// Function: parse_noignore_race @ 0x14610
long long parse_noignore_race(long long a1, long long a2, long long a3)
{
  byte_3E62E = 0;
  return sub_14350(a2, a3);
}



// Function: parse_nowarn @ 0x14630
long long parse_nowarn(long long a1, long long a2, long long a3)
{
  byte_3E630 = 0;
  return sub_14350(a2, a3);
}



// Function: parse_warn @ 0x14650
long long parse_warn(long long a1, long long a2, long long a3)
{
  byte_3E630 = 1;
  return sub_14350(a2, a3);
}



// Function: check_option_combinations @ 0x14670
void check_option_combinations(long long a1)
{
  int v1; // eax
  long long (*v2)(long long, long long); // rdx
  char *v3; // rax

  if ( a1 )
  {
    v1 = 0;
    do
    {
      while ( 1 )
      {
        v2 = *(long long (**)(long long, long long))a1;
        if ( *(long long (**)(long long, long long))a1 != sub_8D40 )
          break;
        a1 = *(long long *)(a1 + 264);
        v1 |= 1u;
        if ( !a1 )
          goto LABEL_8;
      }
      a1 = *(long long *)(a1 + 264);
      if ( v2 == sub_86F0 )
        v1 |= 2u;
    }
    while ( a1 );
LABEL_8:
    if ( v1 == 3 && !HIBYTE(word_3E620) )
    {
      v3 = dcgettext(
             (const char *)a1,
             "The -delete action automatically turns on -depth, but -prune does nothing when -depth is in effect.  If you"
             " want to carry on anyway, just explicitly use the -depth option.",
             5);
      error(1, 0, v3);
      sub_146F0(1);
    }
  }
}



// Function: set_follow_state @ 0x146f0
long long (*set_follow_state(int a1))(int fd)
{
  long long (*result)(int); // rax

  switch ( a1 )
  {
    case 1:
      byte_3E62C = 1;
      qword_3E670 = (long long (*)(long long, long long))sub_DAA0;
      result = (long long (*)(int))sub_10A40;
      qword_3E680 = (long long (*)(long long, long long, long long))sub_10A40;
      break;
    case 2:
      byte_3E62C = 1;
      qword_3E670 = (long long (*)(long long, long long))sub_DA00;
      result = (long long (*)(int))sub_109E0;
      qword_3E680 = (long long (*)(long long, long long, long long))sub_109E0;
      break;
    case 0:
      qword_3E670 = (long long (*)(long long, long long))sub_DB20;
      result = sub_10940;
      qword_3E680 = (long long (*)(long long, long long, long long))sub_10940;
      break;
  }
  dword_3E668 = a1;
  if ( (qword_3E660 & 2) != 0 )
  {
    result = (long long (*)(int))sub_DB70;
    qword_3E670 = (long long (*)(long long, long long))sub_DB70;
  }
  return result;
}



// Function: parse_follow @ 0x147a0
long long parse_follow(long long a1, long long a2, long long a3)
{
  long long v4; // r8

  sub_146F0(1);
  return sub_14350(v4, a3);
}



// Function: parse_begin_user_args @ 0x147d0
void parse_begin_user_args()
{
  qword_3EE70 = 0;
}



// Function: parse_end_user_args @ 0x147e0
void parse_end_user_args()
{
  ;
}



// Function: find_parser @ 0x147f0
int *find_parser(char *s)
{
  int *v2; // rsi
  const char **v3; // rbx
  const char *v4; // rdi
  int v5; // ebp
  const char *v6; // r13

  if ( !strncmp("-newer", s, 6u) )
  {
    v2 = &unk_3D800;
    if ( strlen(s) == 8 )
      return sub_108B0((long long)s, v2);
  }
  v3 = (const char **)&off_3CD48;
  v4 = "!";
  v5 = 0;
  v6 = &s[*s == 45];
  do
  {
    if ( !strcmp(v4, v6) )
    {
      v2 = (int *)((char *)&unk_3CD20 + 32 * v5);
      return sub_108B0((long long)s, v2);
    }
    v4 = *v3;
    v3 += 4;
    ++v5;
  }
  while ( v4 );
  return 0;
}



// Function: insert_false @ 0x148a0
long long insert_false()
{
  int *v0; // rax
  long long *v1; // rax

  v0 = sub_147F0("false");
  v1 = sub_D4D0((long long)v0);
  *((int *)v1 + 6) = 0;
  *((int *)v1 + 9) = 0;
  return 1;
}



// Function: parse_false @ 0x148d0
long long parse_false()
{
  return sub_148A0();
}



// Function: parse_fstype @ 0x148e0
long long parse_fstype(long long a1, long long a2, int *a3, float a4)
{
  unsigned int v4; // r12d
  float *v6; // rax
  float *v7; // rbp
  char *s1; // [rsp+0h] [rbp-28h]

  v4 = sub_107D0(a2, a3);
  if ( !(char)v4 )
    return v4;
  if ( (unsigned short)word_3E688 <= 1u || (unsigned char)sub_10510(s1) )
  {
    v6 = (float *)sub_D490(a1, (long long)s1);
    *((long long *)v6 + 7) = s1;
    v7 = v6;
    sub_13A70(s1);
    v7[9] = a4;
    return v4;
  }
  if ( (qword_3E660 & 8) != 0 )
    __fprintf_chk(stderr, 1, "-fstype %s can never succeed, substituting -false\n", s1);
  return sub_148A0();
}


