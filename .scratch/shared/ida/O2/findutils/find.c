// Function: main @ 0x7240
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
  sub_21E40(v4, a2, a3);
  sub_DA20();
  byte_38618 = 0;
  status = 0;
  byte_38608 = 0;
  dword_385F8 = -100;
  if ( (unsigned char)sub_16220() )
    sub_15FD0();
  qword_38610 = sub_EA90("w");
  if ( !qword_38610 )
  {
LABEL_15:
    v11 = dcgettext(0, "Failed to initialize shared-file hash table", 5);
    v12 = __errno_location();
    error(1, *v12, v11);
    start();
  }
  sub_E630(&word_38620);
  setlocale(6, "");
  bindtextdomain("findutils", "/usr/local/share/locale");
  textdomain("findutils");
  if ( (unsigned int)sub_27A00(sub_17A40) )
  {
    v9 = dcgettext(0, "The atexit library function failed", 5);
    v10 = __errno_location();
    error(1, *v10, v9);
    goto LABEL_15;
  }
  v5 = sub_E180(a1, a2);
  if ( (qword_38660 & 2) != 0 )
    qword_38670 = (long long)sub_DEC0;
  if ( (qword_38660 & 0x80u) != 0LL )
  {
    v8 = ctime(&timer);
    __fprintf_chk(stderr, 1, "cur_day_start = %s", v8);
  }
  v6 = sub_C9F0(a1, a2, v5);
  if ( (unsigned char)sub_8070(a1 - v5, &a2[v5]) )
  {
    sub_9E00(v6);
    sub_DAB0();
  }
  return (unsigned int)status;
}



// Function: get_fts_info_name @ 0x7510
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
      __sprintf_chk(&unk_38598, 1, 14, "[%d]", a1);
      result = (const char *)&unk_38598;
      break;
  }
  return result;
}



// Function: consider_visiting @ 0x7670
unsigned long long consider_visiting(long long a1, const __m128i *a2)
{
  char v4; // ch
  short v5; // ax
  long long v6; // rsi
  long long v7; // rdx
  long long v8; // rdi
  long long v9; // rcx
  __m128i v10; // xmm0
  __m128i v11; // xmm1
  __m128i v12; // xmm4
  __m128i v13; // xmm5
  __m128i v14; // xmm6
  __m128i v15; // xmm7
  __m128i v16; // xmm0
  unsigned short v17; // ax
  long long v18; // rdx
  int v19; // r13d
  long long v20; // rbx
  char v21; // dl
  short v22; // ax
  bool v23; // cf
  bool v24; // zf
  char *v25; // rax
  long long v26; // rax
  const char *v28; // r15
  long long v29; // rax
  int v30; // r13d
  const char *v31; // r14
  const char *v32; // rbx
  const char *v33; // rax
  int v34; // edx
  int v35; // r15d
  const char *v36; // r14
  const char *v37; // rax
  long long v38; // r13
  char *v39; // rax
  long long v40; // rsi
  long long v41; // rax
  long long v42; // rdx
  char *v43; // rbx
  char v44; // r12
  long long v45; // rax
  long long v46; // r13
  long long v47; // r12
  char *v48; // rax
  long long v49; // r12
  char *v50; // rax
  long long v51; // [rsp+8h] [rbp-180h]
  int v52; // [rsp+8h] [rbp-180h]
  int v53; // [rsp+1Ch] [rbp-16Ch] BYREF
  _OWORD v54[9]; // [rsp+20h] [rbp-168h] BYREF
  char v55[152]; // [rsp+B0h] [rbp-D8h] BYREF
  unsigned long long v56; // [rsp+148h] [rbp-40h]

  v56 = __readfsqword(0x28u);
  if ( (qword_38660 & 4) != 0 )
  {
    v28 = (const char *)sub_239F0(2, (unsigned int)dword_3868C, a2[3].m128i_i64[0]);
    v29 = sub_239F0(1, (unsigned int)dword_3868C, a2[3].m128i_i64[1]);
    v30 = dword_38014;
    v31 = (const char *)v29;
    v51 = a2[5].m128i_i64[1];
    v32 = sub_7510(a2[6].m128i_u16[4]);
    v33 = (const char *)sub_239F0(0, (unsigned int)dword_3868C, a2[3].m128i_i64[1]);
    __fprintf_chk(
      stderr,
      1,
      "consider_visiting (early): %s: fts_info=%-6s, fts_level=%2d, prev_depth=%d fts_path=%s, fts_accpath=%s\n",
      v33,
      v32,
      v51,
      v30,
      v31,
      v28);
  }
  v4 = BYTE1(dword_38018);
  v5 = a2[6].m128i_i16[4];
  v6 = dword_38018 & 0x200;
  if ( v5 == 6 )
  {
    if ( (dword_38018 & 0x200) == 0 )
    {
      dword_38014 = a2[5].m128i_i64[1];
      goto LABEL_17;
    }
  }
  else
  {
    v7 = a2[5].m128i_i64[1];
    if ( v7 <= dword_38014 && v7 )
      goto LABEL_6;
    if ( (dword_38018 & 0x200) == 0 )
      goto LABEL_11;
  }
  if ( fd < 0 )
  {
    v8 = *(unsigned int *)(a1 + 44);
    goto LABEL_7;
  }
  close(fd);
  v4 = BYTE1(dword_38018);
  fd = -1;
LABEL_6:
  v8 = *(unsigned int *)(a1 + 44);
  if ( (v4 & 2) == 0 )
  {
LABEL_10:
    v7 = a2[5].m128i_i64[1];
    v5 = a2[6].m128i_i16[4];
    goto LABEL_11;
  }
LABEL_7:
  if ( (int)v8 != -100 && (int)v8 < 0 )
    sub_7640(v8, v6);
  dword_385F8 = v8;
  if ( fd >= 0 )
    goto LABEL_10;
  if ( (int)v8 != -100 )
  {
    fd = sub_17A00(v8, v6);
    goto LABEL_10;
  }
  fd = -100;
  v7 = a2[5].m128i_i64[1];
  v5 = a2[6].m128i_i16[4];
LABEL_11:
  v9 = a2[7].m128i_i64[1];
  dword_38014 = v7;
  *((long long *)&v54[0] + 1) = v9;
  if ( v5 == 7 )
    goto LABEL_66;
  if ( v5 == 4 )
  {
    sub_E8C0(a2[4].m128i_i32[0]);
    if ( (char)word_38620 )
      return v56 - __readfsqword(0x28u);
    v5 = a2[6].m128i_i16[4];
    goto LABEL_16;
  }
  if ( v5 != 2 )
  {
    if ( v5 == 13 )
    {
      if ( (unsigned int)qword_38670(a2[3].m128i_i64[0], v55) && *__errno_location() == 40 )
        goto LABEL_63;
      v5 = a2[6].m128i_i16[4];
      goto LABEL_16;
    }
    if ( v5 != 10 )
      goto LABEL_16;
    if ( v7 )
    {
      if ( (unsigned int)qword_38670(a2[3].m128i_i64[0], v55) && *__errno_location() == 40 )
      {
LABEL_63:
        sub_E8C0(40);
        return v56 - __readfsqword(0x28u);
      }
      sub_E8C0(a2[4].m128i_i32[0]);
      v5 = a2[6].m128i_i16[4];
LABEL_16:
      if ( (unsigned short)(v5 - 10) <= 1u )
      {
        if ( (char)word_385E4 )
          __assert_fail("!state.have_stat", "ftsfind.c", 0x17Cu, "consider_visiting");
        v34 = dword_385E8;
        if ( v5 != 11 && dword_385E8 )
          __assert_fail("ent->fts_info == FTS_NSOK || state.type == 0", "ftsfind.c", 0x17Du, "consider_visiting");
        v53 = dword_385E8;
        goto LABEL_45;
      }
LABEL_17:
      v10 = _mm_loadu_si128(a2 + 7);
      v11 = _mm_loadu_si128(a2 + 8);
      word_385E4 = 257;
      v12 = _mm_loadu_si128(a2 + 11);
      v54[0] = v10;
      v13 = _mm_loadu_si128(a2 + 12);
      v14 = _mm_loadu_si128(a2 + 13);
      v54[1] = v11;
      v15 = _mm_loadu_si128(a2 + 14);
      v54[2] = _mm_loadu_si128(a2 + 9);
      v16 = _mm_loadu_si128(a2 + 15);
      v54[3] = _mm_loadu_si128(a2 + 10);
      v53 = v11.m128i_i32[2];
      dword_385E8 = v11.m128i_i32[2];
      v54[4] = v12;
      v54[5] = v13;
      v54[6] = v14;
      v54[7] = v15;
      v54[8] = v16;
      if ( v11.m128i_i32[2] )
      {
        dword_385E0 = a2[5].m128i_i64[1];
        goto LABEL_19;
      }
      v38 = sub_239F0(0, (unsigned int)dword_3868C, a2[3].m128i_i64[1]);
      v39 = dcgettext(0, "WARNING: file %s appears to have mode 0000", 5);
      error(0, 0, v39, v38);
      v34 = 0;
LABEL_45:
      dword_385E0 = a2[5].m128i_i64[1];
      if ( !v34 )
      {
        v17 = a2[6].m128i_u16[4];
LABEL_47:
        if ( (unsigned short)(v17 - 1) > 1u && v17 != 6 )
        {
          v18 = dword_38624;
          v19 = 0;
          LODWORD(v20) = 0;
          if ( dword_38624 < 0 )
            goto LABEL_27;
LABEL_23:
          if ( a2[5].m128i_i64[1] >= v18 )
          {
            sub_1A750(a1, a2, 4);
            v20 = a2[5].m128i_i64[1] > dword_38624;
            v17 = a2[6].m128i_u16[4];
LABEL_25:
            if ( v17 == 1 )
            {
              if ( !(char)word_38620 )
              {
LABEL_36:
                v21 = qword_38660;
                goto LABEL_37;
              }
            }
            else if ( v17 == 6 && (char)word_38620 )
            {
              goto LABEL_36;
            }
LABEL_27:
            v21 = qword_38660;
            if ( a2[5].m128i_i64[1] >= dword_38628 )
            {
              if ( (qword_38660 & 4) == 0 )
              {
LABEL_29:
                if ( !v20 )
                {
                  v22 = v17 - 10;
                  v23 = v22 == 0;
                  v24 = v22 == 1;
                  v25 = (char *)a2[3].m128i_i64[0];
                  LOBYTE(word_385E4) = !v23 && !v24;
                  src = v25;
                  dword_385F8 = *(int *)(a1 + 44);
                  v26 = sub_B580();
                  sub_E7B0(a2[3].m128i_i64[1], v54, v26);
                  if ( byte_38600 )
                    sub_1A750(a1, a2, 4);
                }
                goto LABEL_38;
              }
LABEL_71:
              v35 = (unsigned char)word_385E4;
              v52 = HIBYTE(word_385E4);
              v36 = sub_7510(v17);
              v37 = (const char *)sub_239F0(0, (unsigned int)dword_3868C, a2[3].m128i_i64[1]);
              __fprintf_chk(
                stderr,
                1,
                "consider_visiting (late): %s: fts_info=%-6s, isdir=%d ignore=%d have_stat=%d have_type=%d \n",
                v37,
                v36,
                v19,
                v20,
                v35,
                v52);
              v17 = a2[6].m128i_u16[4];
              goto LABEL_29;
            }
LABEL_37:
            if ( (v21 & 4) == 0 )
            {
LABEL_38:
              if ( a2[6].m128i_i16[4] == 6 )
                byte_38600 = 0;
              return v56 - __readfsqword(0x28u);
            }
            LODWORD(v20) = 1;
            goto LABEL_71;
          }
LABEL_24:
          LODWORD(v20) = 0;
          goto LABEL_25;
        }
LABEL_21:
        if ( v17 == 11 )
        {
          sub_1A750(a1, a2, 1);
          return v56 - __readfsqword(0x28u);
        }
        v18 = dword_38624;
        v19 = 1;
        if ( dword_38624 < 0 )
          goto LABEL_24;
        goto LABEL_23;
      }
LABEL_19:
      if ( !(unsigned char)sub_DFE0(&v53, a2[3].m128i_i64[1], &a2[16], v54, 0) )
        return v56 - __readfsqword(0x28u);
      v17 = a2[6].m128i_u16[4];
      if ( (v53 & 0xF000) == 0x4000 )
        goto LABEL_21;
      goto LABEL_47;
    }
LABEL_66:
    sub_E8C0(a2[4].m128i_i32[0]);
    return v56 - __readfsqword(0x28u);
  }
  if ( (a2[8].m128i_i32[2] & 0xF000) == 0xA000 )
  {
    v49 = sub_E8A0(0, a2[3].m128i_i64[1]);
    v50 = dcgettext(
            0,
            "Symbolic link %s is part of a loop in the directory hierarchy; we have already visited the directory to which it points.",
            5);
    error(0, 0, v50, v49);
  }
  else
  {
    v40 = (unsigned int)dword_3868C;
    v41 = *(long long *)(a2->m128i_i64[0] + 72);
    v42 = *(long long *)(a2->m128i_i64[0] + 56);
    if ( v41 )
    {
      v43 = (char *)(v42 + v41);
      v44 = *(char *)(v42 + v41);
      *(char *)(v42 + v41) = 0;
      v45 = sub_239F0(1, v40, v42);
      *v43 = v44;
      v46 = v45;
    }
    else
    {
      v46 = sub_239F0(1, (unsigned int)dword_3868C, "");
    }
    v47 = sub_E8A0(0, a2[3].m128i_i64[1]);
    v48 = dcgettext(0, "File system loop detected; %s is part of the same file system loop as %s.", 5);
    error(0, 0, v48, v47, v46);
  }
  status = 1;
  return v56 - __readfsqword(0x28u);
}



// Function: find @ 0x7e10
long long find(const char *a1)
{
  int v1; // eax
  long long v2; // rsi
  unsigned int v3; // r13d
  void *v4; // rbp
  int *v5; // rbx
  long long v6; // rax
  unsigned int v7; // r14d
  int v8; // eax
  long long v9; // rax
  const __m128i *v10; // rsi
  int v11; // r8d
  long long v12; // rax
  char *v13; // rax
  long long v14; // r12
  char *v15; // rax
  long long result; // rax
  long long v17[9]; // [rsp+10h] [rbp-48h] BYREF

  v17[3] = __readfsqword(0x28u);
  v1 = strlen(a1);
  v2 = (unsigned int)dword_38018;
  dword_385FC = v1;
  if ( (dword_38018 & 0x200) != 0 )
  {
    dword_385F8 = -100;
    if ( fd < 0 )
      fd = -100;
  }
  v17[0] = a1;
  v17[1] = 0;
  switch ( dword_38668 )
  {
    case 1:
      v2 = dword_38018 | 3u;
      dword_38018 |= 3u;
      break;
    case 2:
      v2 = dword_38018 | 0x11u;
      dword_38018 |= 0x11u;
      break;
    case 0:
      v2 = dword_38018 | 0x10u;
      dword_38018 |= 0x10u;
      break;
  }
  if ( byte_3862D )
  {
    v2 = (unsigned int)v2 | 0x40;
    dword_38018 = v2;
  }
  v3 = 0x80000000;
  v4 = (void *)sub_199D0(v17, v2, 0);
  v5 = __errno_location();
  if ( !v4 )
  {
    v14 = sub_E8A0(0, a1);
    v15 = dcgettext(0, "cannot search %s", 5);
    error(0, *v5, v15, v14);
    status = 1;
    return 1;
  }
  while ( 1 )
  {
    *v5 = 0;
    v9 = sub_19FF0(v4);
    v10 = (const __m128i *)v9;
    if ( !v9 )
      break;
    v6 = *(long long *)(v9 + 88);
    v7 = v10[5].m128i_i64[1];
    if ( byte_38608 )
    {
      if ( v3 != (int)v6 )
      {
        sub_D9F0();
        v7 = v10[5].m128i_u32[2];
      }
    }
    v8 = v10[8].m128i_i32[2];
    v3 = v7;
    byte_38618 = 0;
    LOBYTE(word_385E4) = 0;
    dword_385E8 = v8;
    HIBYTE(word_385E4) = v8 != 0;
    sub_7670((long long)v4, v10);
  }
  if ( *v5 )
  {
    v12 = sub_E8A0(0, a1);
    error(0, *v5, "failed to read file names from file system at or below %s", v12);
LABEL_21:
    status = 1;
    return 0;
  }
  v11 = sub_19E00(v4);
  result = 1;
  if ( v11 )
  {
    v13 = dcgettext(0, "failed to restore working directory after searching %s", 5);
    error(0, *v5, v13, a1);
    goto LABEL_21;
  }
  return result;
}



// Function: process_all_startpoints @ 0x8070
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
  if ( a1 <= 0 || (unsigned char)sub_E120(*a2, 1) )
  {
    v2 = filename;
    if ( !filename )
    {
      v29 = 46;
      return (unsigned int)sub_7E10((const char *)&v29);
    }
    if ( !strcmp(filename, "-") )
    {
      if ( byte_38698 )
      {
        v21 = dcgettext(0, "option -files0-from reading from standard input cannot be combined with -ok, -okdir", 5);
        error(1, 0, "%s\n", v21);
        goto LABEL_47;
      }
      v3 = dcgettext(0, "(standard input)", 5);
      v4 = (const char *)sub_E8A0(0, v3);
      stream = stdin;
      goto LABEL_6;
    }
    v4 = (const char *)sub_E8A0(0, v2);
    stream = fopen(filename, "r");
    if ( stream )
    {
      v9 = fileno(stream);
      v10 = v9;
      if ( v9 >= 0 )
      {
        if ( byte_38698 && !fstat(v9, &buf) && !fstat(0, &v28) && buf.st_ino == v28.st_ino && buf.st_dev == v28.st_dev )
        {
          v17 = dcgettext(
                  0,
                  "option -files0-from: standard input must not refer to the same file when combined with -ok, -okdir",
                  5);
          error(1, 0, "%s: %s\n", v17, v4);
          goto LABEL_43;
        }
        sub_17990(v10, 1);
LABEL_6:
        v5 = (void *)sub_176A0(stream);
LABEL_7:
        if ( v5 )
        {
          v6 = 1;
          while ( 1 )
          {
            v7 = (const char *)sub_176E0(v5, &v26);
            if ( !v7 )
              break;
            if ( *v7 )
            {
              if ( !filename && (unsigned char)sub_E120(v7, 1) )
                goto LABEL_14;
              dword_385FC = strlen(v7);
              if ( !(unsigned char)sub_7E10(v7) )
              {
                v6 = 0;
                goto LABEL_14;
              }
            }
            else
            {
              if ( filename )
              {
                v11 = sub_17780(v5);
                v12 = dcgettext(0, "invalid zero-length file name", 5);
                error(0, 0, "%s:%lu: %s", v4, v11, v12);
              }
              else
              {
                v13 = (const char *)sub_E8A0(0, v7);
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
              sub_177A0(v5);
              return v6;
            }
            if ( v26 != 2 )
              __assert_fail("!\"unexpected error code from argv_iter\"", "ftsfind.c", 0x2A3u, "process_all_startpoints");
LABEL_14:
            sub_177A0(v5);
            if ( !(char)v6 || !filename || !ferror(stream) && !(unsigned int)sub_17D40(stream) )
              return v6;
LABEL_43:
            v18 = dcgettext(0, "error reading %s", 5);
            error(1, 0, v18, v4);
          }
        }
LABEL_47:
        sub_25F50();
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
    v5 = (void *)sub_17670(a2);
    goto LABEL_7;
  }
  v22 = sub_E8A0(0, *a2);
  v23 = dcgettext(0, "extra operand %s", 5);
  error(0, 0, v23, v22);
  v24 = dcgettext(0, "file operands cannot be combined with -files0-from", 5);
  error(1, 0, "%s", v24);
  return sub_8530();
}



// Function: is_fts_enabled @ 0x8530
long long is_fts_enabled(int *a1)
{
  *a1 = dword_38018;
  return 1;
}



// Function: is_ok @ 0x8550
long long is_ok(long long a1, long long a2)
{
  char *v2; // rax
  char *v4; // r12
  int *v5; // rax

  sub_180C0(stdout);
  v2 = dcgettext(0, "< %s ... %s > ? ", 5);
  if ( (int)__fprintf_chk(stderr, 1, v2, a1, a2) < 0 )
  {
    v4 = dcgettext(0, "Failed to write prompt for -ok", 5);
    v5 = __errno_location();
    error(1, *v5, v4);
    sub_DAB0();
    exit(status);
  }
  sub_180C0(stderr);
  return sub_268C0();
}



// Function: print_parenthesised @ 0x8870
void print_parenthesised(long long a1, long long a2)
{
  long long v2; // rbp
  long long v3; // rdx
  long long v4; // rdx
  long long v5; // rcx

  if ( a2 )
  {
    v2 = a2;
    while ( 1 )
    {
      v3 = *(long long *)(v2 + 272);
      if ( *(long long (**)())v2 != sub_8940 && *(long long (**)())v2 != sub_8990 )
        break;
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
      sub_86E0(a1, v2, v4, v5);
      __fprintf_chk(a1, 1, "%s", " ) ");
      return;
    }
    sub_86E0(a1, v2, 0, sub_8940);
  }
}



// Function: pred_timewindow @ 0x89e0
char pred_timewindow(time_t a1, long long a2, long long a3, int a4)
{
  int v5; // eax
  time_t v7; // rsi
  long long v8; // rbx
  time_t v9; // rsi
  long long v10; // rbx
  double v12; // xmm0_8
  double v13; // xmm0_8

  v5 = *(int *)(a3 + 60);
  if ( v5 == 1 )
  {
    v7 = *(long long *)(a3 + 64);
    v8 = *(long long *)(a3 + 72);
    if ( a1 != v7 || (LOBYTE(v5) = 0, v8 != a2) )
      LOBYTE(v5) = (double)((int)a2 - (int)v8) * 0.000000001 + difftime(a1, v7) < 0.0;
  }
  else if ( v5 == 2 )
  {
    v12 = difftime(a1, *(long long *)(a3 + 64));
    LOBYTE(v5) = 0;
    v13 = (double)((int)a2 - *(int *)(a3 + 72)) * 0.000000001 + v12;
    if ( v13 > 0.0 )
      LOBYTE(v5) = (double)a4 >= v13;
  }
  else
  {
    if ( v5 )
      __assert_fail("0", "pred.c", 0x8Au, "pred_timewindow");
    v9 = *(long long *)(a3 + 64);
    v10 = *(long long *)(a3 + 72);
    if ( a1 != v9 || a2 != v10 )
      LOBYTE(v5) = (double)((int)a2 - (int)v10) * 0.000000001 + difftime(a1, v9) >= 0.0;
  }
  return v5;
}



// Function: pred_amin @ 0x8b10
char pred_amin(long long a1, long long a2, long long a3)
{
  return sub_89E0(*(long long *)(a2 + 72), *(long long *)(a2 + 80), a3, 60);
}



// Function: pred_anewer @ 0x8b30
bool pred_anewer(long long a1, long long a2, long long a3)
{
  time_t v3; // rdi
  long long v4; // rbx
  time_t v5; // rsi
  long long v6; // rbp
  bool result; // al

  if ( *(int *)(a3 + 60) )
    __assert_fail("COMP_GT == pred_ptr->args.reftime.kind", "pred.c", 0xA6u, "pred_anewer");
  v3 = *(long long *)(a2 + 72);
  v4 = *(long long *)(a2 + 80);
  v5 = *(long long *)(a3 + 64);
  v6 = *(long long *)(a3 + 72);
  if ( v3 != v5 )
    return (double)((int)v4 - (int)v6) * 0.000000001 + difftime(v3, v5) >= 0.0;
  result = 0;
  if ( v4 != v6 )
    return (double)((int)v4 - (int)v6) * 0.000000001 + difftime(v3, v5) >= 0.0;
  return result;
}



// Function: pred_atime @ 0x8bc0
char pred_atime(long long a1, long long a2, long long a3)
{
  return sub_89E0(*(long long *)(a2 + 72), *(long long *)(a2 + 80), a3, 86400);
}



// Function: pred_cmin @ 0x8bf0
char pred_cmin(long long a1, long long a2, long long a3)
{
  return sub_89E0(*(long long *)(a2 + 104), *(long long *)(a2 + 112), a3, 60);
}



// Function: pred_cnewer @ 0x8c10
bool pred_cnewer(long long a1, long long a2, long long a3)
{
  time_t v3; // rdi
  long long v4; // rbx
  time_t v5; // rsi
  long long v6; // rbp
  bool result; // al

  if ( *(int *)(a3 + 60) )
    __assert_fail("COMP_GT == pred_ptr->args.reftime.kind", "pred.c", 0xC7u, "pred_cnewer");
  v3 = *(long long *)(a2 + 104);
  v4 = *(long long *)(a2 + 112);
  v5 = *(long long *)(a3 + 64);
  v6 = *(long long *)(a3 + 72);
  if ( v3 != v5 )
    return (double)((int)v4 - (int)v6) * 0.000000001 + difftime(v3, v5) >= 0.0;
  result = 0;
  if ( v4 != v6 )
    return (double)((int)v4 - (int)v6) * 0.000000001 + difftime(v3, v5) >= 0.0;
  return result;
}



// Function: pred_comma @ 0x8ca0
long long pred_comma(long long a1, long long a2, long long a3)
{
  long long v4; // rdx

  v4 = *(long long *)(a3 + 272);
  if ( v4 )
    sub_E7B0(a1, a2, v4);
  return sub_E7B0(a1, a2, *(long long *)(a3 + 280));
}



// Function: pred_ctime @ 0x8ce0
char pred_ctime(long long a1, long long a2, long long a3)
{
  return sub_89E0(*(long long *)(a2 + 104), *(long long *)(a2 + 112), a3, 86400);
}



// Function: pred_delete @ 0x8d00
long long pred_delete(long long a1, long long a2)
{
  const char *v2; // r12
  int v3; // r13d
  int *v4; // rbx
  long long result; // rax
  long long v6; // r12
  char *v7; // rax

  v2 = src;
  if ( !strcmp(src, ".") )
    return 1;
  v3 = 0;
  if ( (char)word_385E4 )
    v3 = ((*(int *)(a2 + 24) & 0xF000) == 0x4000) << 9;
  if ( !unlinkat(dword_385F8, v2, v3) )
    return 1;
  v4 = __errno_location();
  if ( *v4 != 2 )
  {
    if ( *v4 == 21 && !v3 && !unlinkat(dword_385F8, src, 512) )
      return 1;
    goto LABEL_10;
  }
  result = (unsigned char)byte_3862E;
  if ( !byte_3862E )
  {
LABEL_10:
    v6 = sub_E8A0(0, a1);
    v7 = dcgettext(0, "cannot delete %s", 5);
    error(0, *v4, v7, v6);
    status = 1;
    return 0;
  }
  *v4 = 0;
  return result;
}



// Function: pred_empty @ 0x8e20
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
    v11 = sub_1F080(dword_385F8, v6, 592128, v8, v9, v10);
    v12 = v11;
    if ( v11 < 0 )
      goto LABEL_15;
    v13 = fdopendir(v11);
    if ( !v13 )
    {
      v17 = (const char *)sub_E8A0(0, a1);
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
      v16 = (const char *)sub_E8A0(0, a1);
      error(0, *v7, "%s", v16);
      status = 1;
      closedir(v13);
      return v3;
    }
    if ( closedir(v13) )
    {
LABEL_15:
      v3 = 0;
      v15 = (const char *)sub_E8A0(0, a1);
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



// Function: pred_execdir @ 0x8fc0
long long pred_execdir()
{
  return sub_A040(src);
}



// Function: pred_false @ 0x8fd0
long long pred_false()
{
  return 0;
}



// Function: pred_fstype @ 0x9090
bool pred_fstype(long long a1, long long a2, long long a3)
{
  const char *v3; // rbp
  const char *v4; // rax

  v3 = *(const char **)(a3 + 56);
  v4 = (const char *)sub_108E0(a2, a1);
  return strcmp(v4, v3) == 0;
}



// Function: pred_gid @ 0x90c0
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



// Function: pred_group @ 0x9110
bool pred_group(long long a1, long long a2, long long a3)
{
  return *(int *)(a3 + 56) == *(int *)(a2 + 32);
}



// Function: pred_ilname @ 0x9120
long long pred_ilname(long long a1, long long a2, long long a3)
{
  if ( (*(int *)(a2 + 24) & 0xF000) == 0xA000 )
    return sub_8660(a1, a3, 1);
  else
    return 0;
}



// Function: pred_iname @ 0x9150
bool pred_iname(long long a1, long long a2, long long a3)
{
  const char *v3; // r12
  char *v4; // rbp
  int v5; // ebx

  v3 = *(const char **)(a3 + 56);
  v4 = (char *)sub_17B90();
  sub_17D00(v4);
  v5 = fnmatch(v3, v4, 16);
  free(v4);
  return v5 == 0;
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
  if ( (*(int *)(a2 + 24) & 0xF000) == 0xA000 )
    return sub_8660(a1, a3, 0);
  else
    return 0;
}



// Function: pred_ls @ 0x9290
long long pred_ls(int a1, int a2, long long a3)
{
  sub_165E0(a1, dword_385F8, (int)src, a2, qword_38638, dword_3865C, *(unsigned char *)(a3 + 40), *(FILE **)(a3 + 64));
  return 1;
}



// Function: pred_mmin @ 0x92d0
char pred_mmin(long long a1, long long a2, long long a3)
{
  return sub_89E0(*(long long *)(a2 + 88), *(long long *)(a2 + 96), a3, 60);
}



// Function: pred_mtime @ 0x92f0
char pred_mtime(long long a1, long long a2, long long a3)
{
  return sub_89E0(*(long long *)(a2 + 88), *(long long *)(a2 + 96), a3, 86400);
}



// Function: pred_name @ 0x9310
bool pred_name(long long a1, long long a2, long long a3)
{
  const char *v3; // r12
  char *v4; // rbp
  int v5; // ebx

  v3 = *(const char **)(a3 + 56);
  v4 = (char *)sub_17B90();
  sub_17D00(v4);
  v5 = fnmatch(v3, v4, 0);
  free(v4);
  return v5 == 0;
}



// Function: pred_negate @ 0x9350
long long pred_negate(long long a1, long long a2, long long a3)
{
  return (unsigned int)sub_E7B0(a1, a2, *(long long *)(a3 + 280)) ^ 1;
}



// Function: pred_newer @ 0x9370
bool pred_newer(long long a1, long long a2, long long a3)
{
  time_t v3; // rdi
  long long v4; // rbx
  time_t v5; // rsi
  long long v6; // rbp
  bool result; // al

  if ( *(int *)(a3 + 60) )
    __assert_fail("COMP_GT == pred_ptr->args.reftime.kind", "pred.c", 0x26Du, "pred_newer");
  v3 = *(long long *)(a2 + 88);
  v4 = *(long long *)(a2 + 96);
  v5 = *(long long *)(a3 + 64);
  v6 = *(long long *)(a3 + 72);
  if ( v3 != v5 )
    return (double)((int)v4 - (int)v6) * 0.000000001 + difftime(v3, v5) >= 0.0;
  result = 0;
  if ( v4 != v6 )
    return (double)((int)v4 - (int)v6) * 0.000000001 + difftime(v3, v5) >= 0.0;
  return result;
}



// Function: pred_newerXY @ 0x9400
bool pred_newerXY(long long a1, long long *a2, long long a3)
{
  time_t v3; // rdi
  long long v4; // rbx
  time_t v5; // rsi
  long long v6; // rbp
  bool result; // al
  long long v8; // r12
  char *v9; // rax

  if ( *(int *)(a3 + 60) )
    __assert_fail("COMP_GT == pred_ptr->args.reftime.kind", "pred.c", 0x277u, "pred_newerXY");
  switch ( *(int *)(a3 + 56) )
  {
    case 0:
      v3 = a2[9];
      v4 = a2[10];
      goto LABEL_4;
    case 1:
      v8 = sub_E8A0(0, a1);
      v9 = dcgettext(0, "WARNING: cannot determine birth time of file %s", 5);
      error(0, 0, v9, v8);
      return 0;
    case 2:
      v3 = a2[13];
      v4 = a2[14];
      goto LABEL_4;
    case 3:
      v3 = a2[11];
      v4 = a2[12];
LABEL_4:
      v5 = *(long long *)(a3 + 64);
      v6 = *(long long *)(a3 + 72);
      if ( v5 != v3 )
        return (double)((int)v4 - (int)v6) * 0.000000001 + difftime(v3, v5) >= 0.0;
      result = 0;
      if ( v4 != v6 )
        return (double)((int)v4 - (int)v6) * 0.000000001 + difftime(v3, v5) >= 0.0;
      return result;
    case 4:
      __assert_fail("pred_ptr->args.reftime.xval != XVAL_TIME", "pred.c", 0x27Cu, "pred_newerXY");
    default:
      __assert_fail("collected", "pred.c", 0x29Bu, "pred_newerXY");
  }
}



// Function: pred_nogroup @ 0x9540
bool pred_nogroup(long long a1, long long a2)
{
  return getgrgid(*(int *)(a2 + 32)) == 0;
}



// Function: pred_nouser @ 0x9560
bool pred_nouser(long long a1, long long a2)
{
  return getpwuid(*(int *)(a2 + 28)) == 0;
}



// Function: pred_ok @ 0x9580
long long pred_ok(char *s, long long a2, long long a3)
{
  if ( (unsigned char)sub_8550(**(long long **)(a3 + 232), (long long)s) )
    return sub_A040(s);
  else
    return 0;
}



// Function: pred_okdir @ 0x95d0
long long pred_okdir(long long a1, long long a2, long long a3)
{
  if ( (unsigned char)sub_8550(**(long long **)(a3 + 232), a1) )
    return sub_A040(src);
  else
    return 0;
}



// Function: pred_path @ 0x9640
bool pred_path(char *name, long long a2, long long a3)
{
  return fnmatch(*(const char **)(a3 + 56), name, 0) == 0;
}



// Function: pred_perm @ 0x9660
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



// Function: pred_executable @ 0x96e0
bool pred_executable()
{
  return faccessat(dword_385F8, src, 1, 0) == 0;
}



// Function: pred_readable @ 0x9710
bool pred_readable()
{
  return faccessat(dword_385F8, src, 4, 0) == 0;
}



// Function: pred_writable @ 0x9740
bool pred_writable()
{
  return faccessat(dword_385F8, src, 2, 0) == 0;
}



// Function: pred_regex @ 0x97e0
bool pred_regex(char *string, long long a2, long long a3)
{
  int v4; // ebx

  v4 = strlen(string);
  return re_match(*(struct re_pattern_buffer **)(a3 + 56), string, v4, 0, 0) == v4;
}



// Function: pred_size @ 0x9820
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



// Function: pred_samefile @ 0x9890
char pred_samefile(long long a1, long long *a2, long long a3)
{
  long long v4; // rax
  int v5; // eax

  v4 = a2[1];
  if ( v4 && v4 != *(long long *)(a3 + 56) || (v5 = sub_D820(a1, src, a2)) != 0 )
  {
    LOBYTE(v5) = 0;
  }
  else if ( a2[1] == *(long long *)(a3 + 56) )
  {
    LOBYTE(v5) = *a2 == *(long long *)(a3 + 64);
  }
  return v5;
}



// Function: pred_type @ 0x9900
long long pred_type(long long a1, long long a2, long long a3)
{
  int v3; // eax
  unsigned int v4; // r8d
  unsigned int v5; // eax
  long long v6; // rcx

  if ( !HIBYTE(word_385E4) )
    __assert_fail("state.have_type", "pred.c", 0x3D1u, "pred_type");
  LOWORD(v3) = dword_385E8;
  v4 = 0;
  if ( dword_385E8 )
  {
    if ( (char)word_385E4 )
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



// Function: pred_uid @ 0x99e0
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



// Function: pred_used @ 0x9a30
char pred_used(long long a1, long long *a2, long long a3)
{
  long long v4; // r12
  long long v5; // rbx
  time_t v6; // r14
  long long v7; // rbp
  time_t v8; // rdi
  long long v9; // rbx
  long long v10; // rsi

  v4 = a2[10];
  v5 = a2[14];
  v6 = a2[9];
  v7 = a2[13];
  if ( v6 == v7 && v4 == v5 )
  {
    v10 = 0;
    v8 = 0;
    return sub_89E0(v8, v10, a3, 86400);
  }
  if ( (double)((int)v4 - (int)v5) * 0.000000001 + difftime(v6, a2[13]) >= 0.0 )
  {
    v8 = v7 - v6;
    v9 = v5 - v4;
    v10 = v9;
    if ( v9 < 0 )
    {
      v10 = v9 + 1000000000;
      --v8;
    }
    return sub_89E0(v8, v10, a3, 86400);
  }
  return 0;
}



// Function: pred_user @ 0x9ae0
bool pred_user(long long a1, long long a2, long long a3)
{
  return *(int *)(a3 + 56) == *(int *)(a2 + 28);
}



// Function: pred_xtype @ 0x9af0
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
  if ( (unsigned char)sub_DFB0() )
    v4 = (unsigned int (*)(char *, struct stat *))sub_DE70;
  else
    v4 = (unsigned int (*)(char *, struct stat *))sub_DDB0;
  sub_D810(&v10);
  if ( !v4(src, &v10) )
    return sub_9900(a1, (long long)&v10, a3);
  v5 = sub_DFB0();
  v6 = __errno_location();
  v7 = v6;
  if ( v5 && *v6 == 2 )
    return sub_9900(a1, a2, a3);
  v8 = (const char *)sub_E8A0(0, a1);
  error(0, *v7, "%s", v8);
  status = 1;
  return 0;
}



// Function: pred_context @ 0x9c00
long long pred_context(long long a1, long long a2, long long a3)
{
  unsigned int v3; // r12d
  long long v6; // r13
  char *v7; // r12
  int *v8; // rax
  const char *v9; // rdx
  char *name[7]; // [rsp+0h] [rbp-38h] BYREF

  name[1] = (char *)__readfsqword(0x28u);
  if ( (int)qword_38680((unsigned int)dword_385F8, src, name) < 0 )
  {
    v6 = sub_E8A0(0, a1);
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



// Function: print_list @ 0x9cd0
unsigned long long print_list(long long a1, long long a2)
{
  long long v2; // r13
  const char *v3; // rcx
  int v4; // eax
  int v5; // edx
  char *v6; // rax
  char *v7; // rsi
  char v9[264]; // [rsp+0h] [rbp-138h] BYREF
  unsigned long long i; // [rsp+108h] [rbp-30h]

  v2 = a2;
  for ( i = __readfsqword(0x28u); v2; v2 = *(long long *)(v2 + 264) )
  {
    v3 = *(const char **)(v2 + 8);
    if ( v3 )
    {
      __strcpy_chk(v9, *(long long *)(v2 + 8), 256);
      v4 = strlen(v9);
      v5 = v4 - 1;
      if ( v4 - 1 < 0 )
      {
        v7 = &v9[v4];
      }
      else
      {
        v6 = &v9[v5];
        while ( 1 )
        {
          v7 = v6;
          if ( *v6 != 32 && *v6 != 9 )
            break;
          --v5;
          --v6;
          if ( v5 == -1 )
            goto LABEL_9;
        }
        v7 = &v9[v5 + 1];
      }
LABEL_9:
      *v7 = 0;
      v3 = v9;
    }
    __fprintf_chk(a1, 1, "[%s] ", v3);
  }
  __fprintf_chk(a1, 1, "\n");
  return i - __readfsqword(0x28u);
}



// Function: show_success_rates @ 0x9e00
long long show_success_rates(long long a1)
{
  long long v2; // rdx
  long long v3; // rcx
  FILE *v4; // rdi
  long long result; // rax

  if ( (qword_38660 & 0x40) != 0 )
  {
    __fprintf_chk(stderr, 1, "Predicate success rates after completion:\n");
    v4 = stderr;
    if ( a1 )
    {
      sub_86E0(stderr, a1, v2, v3);
      v4 = stderr;
    }
    return __fprintf_chk(v4, 1, "\n");
  }
  return result;
}



// Function: pred_sanity_check @ 0x9e60
void pred_sanity_check(long long a1)
{
  long long (*v1)(); // rdx
  long long v2; // rax
  long long (*v3)(); // rcx

  for ( ; a1; a1 = *(long long *)(a1 + 264) )
  {
    v1 = *(long long (**)())a1;
    if ( !*(long long *)a1 )
      __assert_fail("p->pred_func != NULL", "pred.c", 0x51Eu, "pred_sanity_check");
    v2 = *(long long *)(a1 + 304);
    if ( !v2 )
      __assert_fail("p->parser_entry != NULL", "pred.c", 0x521u, "pred_sanity_check");
    v3 = *(long long (**)())(v2 + 24);
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
        if ( v1 != sub_8600 && (char *)v1 != (char *)&loc_85E0 && !*(char *)(a1 + 25) )
          __assert_fail("p->no_default_print", "pred.c", 0x545u, "pred_sanity_check");
        return;
      default:
        continue;
    }
  }
}



// Function: impl_pred_exec @ 0xa040
long long impl_pred_exec(char *s, long long a2, long long a3)
{
  long long v5; // r15
  long long v6; // rbx
  const char *v7; // rdx
  size_t v8; // rax
  long long v9; // rdx
  size_t v11; // rax
  char *v12; // r13
  long long v13; // rax
  int v14; // r14d
  void *v15; // r13
  long long v16; // rax
  int v17; // ecx
  int v18; // r8d
  int v19; // r9d
  long long v20; // rdi
  void *v21; // rax
  int v22; // r13d
  long long v23; // rax
  int v24; // ecx
  int v25; // r8d
  int v26; // r9d
  long long v27; // rdi
  long long v28; // r13
  char *v29; // r12
  int *v30; // rax
  const char *v31; // [rsp+8h] [rbp-60h]
  size_t v32; // [rsp+10h] [rbp-58h]
  void *v33; // [rsp+18h] [rbp-50h]
  unsigned char v34; // [rsp+27h] [rbp-41h]
  char *ptr; // [rsp+28h] [rbp-40h]

  v34 = sub_E880(*(long long *)a3);
  if ( !v34 )
  {
    if ( *(void **)(a3 + 248) == qword_385C0 )
    {
      v33 = 0;
      v5 = 0;
      ptr = 0;
      goto LABEL_4;
    }
LABEL_29:
    __assert_fail("execp->wd_for_exec == initial_wd", "exec.c", 0x94u, "impl_pred_exec");
  }
  v12 = src;
  if ( !*(int *)(a3 + 208) )
  {
    if ( strchr(src, 47) )
    {
      v13 = sub_17C40(v12);
      v14 = dword_385F8;
      v15 = (void *)v13;
      v16 = sub_25A50(16);
      *(long long *)(a3 + 248) = v16;
      *(long long *)(v16 + 8) = 0;
      *(int *)v16 = sub_1F080(v14, (int)v15, 0, v17, v18, v19);
      v20 = **(unsigned int **)(a3 + 248);
      if ( (int)v20 >= 0 )
      {
        sub_17990(v20, 1);
        free(v15);
        v12 = src;
        goto LABEL_22;
      }
      free(v15);
    }
    else
    {
      v22 = dword_385F8;
      v23 = sub_25A50(16);
      *(long long *)(a3 + 248) = v23;
      *(long long *)(v23 + 8) = 0;
      *(int *)v23 = sub_1F080(v22, (unsigned int)".", 0, v24, v25, v26);
      v27 = **(unsigned int **)(a3 + 248);
      if ( (int)v27 >= 0 )
      {
        sub_17990(v27, 1);
        v12 = src;
        goto LABEL_22;
      }
    }
    v28 = sub_E8A0(0, s);
    v29 = dcgettext(0, "Failed to save working directory in order to run a command on %s", 5);
    v30 = __errno_location();
    error(1, *v30, v29, v28);
    goto LABEL_29;
  }
LABEL_22:
  ptr = (char *)sub_17B90(v12);
  s = ptr;
  v5 = 2LL * (*ptr != 47);
  v21 = 0;
  if ( *ptr != 47 )
    v21 = &unk_28960;
  v33 = v21;
LABEL_4:
  LODWORD(v6) = *(unsigned char *)(a3 + 56);
  if ( !(char)v6 )
  {
    v6 = 0;
    while ( *(int *)(a3 + 240) > (int)v6 )
    {
      v32 = strlen(s);
      v7 = *(const char **)(*(long long *)(a3 + 232) + 8 * v6++);
      v31 = v7;
      v8 = strlen(v7);
      sub_15270(a3 + 64, a3 + 152, v31, v8, v33, v5, s, v32, 0);
    }
    sub_15430(a3 + 64, a3 + 152);
    v9 = *(int *)(a3 + 256) & 0x7F | (unsigned int)(unsigned char)BYTE1(*(int *)(a3 + 256));
    LOBYTE(v6) = (int)v9 == 0;
    if ( !v34 )
    {
      if ( !ptr )
        return (unsigned int)v6;
LABEL_17:
      __assert_fail("local", "exec.c", 0xD0u, "impl_pred_exec");
    }
    sub_242C0(*(long long *)(a3 + 248), a3 + 152, v9);
    if ( !ptr )
      return (unsigned int)v6;
    v34 = v6;
LABEL_12:
    free(ptr);
    LODWORD(v6) = v34;
    return (unsigned int)v6;
  }
  v11 = strlen(s);
  sub_14FB0(a3 + 64, a3 + 152, s, v11 + 1, v33, v5, 0);
  if ( *(int *)(a3 + 208) )
    byte_38608 = 1;
  if ( ptr )
  {
    if ( !v34 )
      goto LABEL_17;
    goto LABEL_12;
  }
  return (unsigned int)v6;
}



// Function: launch @ 0xa3a0
long long launch(long long a1, long long a2, long long a3, const char **a4)
{
  __pid_t v6; // eax
  __pid_t v7; // r13d
  long long v8; // r13
  int *v9; // rbx
  const char *v10; // rax
  int *v11; // rbx
  int v12; // ebx
  int v13; // ebx
  unsigned long long v15; // rbx
  long long v16; // rsi
  const char *v17; // rax
  char *v18; // rax
  char *v19; // rbp
  char *v20; // r12
  char *v21; // rax
  long long v22; // r12
  char *v23; // rax
  long long v24; // r12
  char *v25; // rax
  int v26; // edx
  int v27; // ecx
  int v28; // r8d
  int v29; // r9d
  const char *v30; // rax
  char *v31; // r12
  int *v32; // rax
  long long v33[7]; // [rsp+Eh] [rbp-3Ah] BYREF

  *(long long *)((char *)&v33[1] + 2) = __readfsqword(0x28u);
  if ( (qword_38660 & 0x20) != 0 )
  {
    v15 = 0;
    __fprintf_chk(stderr, 1, "DebugExec: launching process (argc=%lu):", *(long long *)(a2 + 96) - 1LL);
    if ( *(long long *)(a2 + 96) != 1 )
    {
      do
      {
        v16 = *(long long *)(*(long long *)(a2 + 104) + 8 * v15++);
        v17 = (const char *)sub_E8A0(0, v16);
        __fprintf_chk(stderr, 1, " %s", v17);
      }
      while ( *(long long *)(a2 + 96) - 1LL > v15 );
    }
    __fprintf_chk(stderr, 1, "\n");
  }
  sub_180C0(stdout);
  sub_180C0(stderr);
  if ( dword_3801C )
  {
    dword_3801C = 0;
    signal(17, 0);
  }
  v6 = fork();
  v7 = v6;
  if ( v6 == -1 )
  {
    v31 = dcgettext(0, "cannot fork", 5);
    v32 = __errno_location();
    error(1, *v32, v31);
  }
  if ( !v6 )
  {
    v8 = *(long long *)(a2 + 192);
    if ( !v8 )
      __assert_fail("NULL != execp->wd_for_exec", "exec.c", 0x141u, "launch");
    v9 = __errno_location();
    if ( !*(char *)(a2 + 188) )
      goto LABEL_45;
    strcpy((char *)v33, "/dev/null");
    if ( close(0) < 0 )
    {
      v18 = dcgettext(0, "Cannot close standard input", 5);
      error(0, *v9, v18);
      if ( !(unsigned int)sub_24290(v8) )
        goto LABEL_14;
      goto LABEL_29;
    }
    if ( (int)sub_18040((unsigned int)v33, 0, v26, v27, v28, v29) < 0 )
    {
      v30 = (const char *)sub_E8A0(0, v33);
      error(0, *v9, "%s", v30);
      if ( !(unsigned int)sub_24290(v8) )
        goto LABEL_9;
    }
    else
    {
LABEL_45:
      if ( !(unsigned int)sub_24290(v8) )
      {
LABEL_9:
        if ( (unsigned char)sub_16220() )
          sub_16240();
        if ( (unsigned char)sub_15970(a4) )
          *v9 = 7;
        else
          execvp(*a4, (char *const *)a4);
        v10 = (const char *)sub_E8A0(0, *a4);
        error(0, *v9, "%s", v10);
        goto LABEL_14;
      }
    }
LABEL_29:
    if ( *(int *)v8 >= 0 || (v19 = *(char **)(v8 + 8), v20 = (char *)": ", !v19) )
    {
      v19 = "";
      v20 = "";
    }
    v21 = dcgettext(0, "Failed to change directory%s%s", 5);
    error(0, *v9, v21, v20, v19);
LABEL_14:
    _exit(1);
  }
  do
  {
    if ( waitpid(v7, (int *)(a2 + 200), 0) != -1 )
    {
      v12 = *(int *)(a2 + 200);
      if ( __OFSUB__((*(char *)(a2 + 200) & 0x7F) + 1, 1) || (*(char *)(a2 + 200) & 0x7F) == 0 )
      {
        v13 = BYTE1(v12);
        if ( (qword_38660 & 0x20) != 0 )
          __fprintf_chk(stderr, 1, "DebugExec: process (PID=%ld) terminated with exit status: %d\n", v7, v13);
        if ( !v13 )
          return 1;
      }
      else
      {
        v24 = sub_239F0(0, (unsigned int)dword_3868C, *a4);
        v25 = dcgettext(0, "%s terminated by signal %d", 5);
        error(0, 0, v25, v24, v12 & 0x7F);
      }
      if ( *(char *)a2 )
        status = 1;
      return 1;
    }
    v11 = __errno_location();
  }
  while ( *v11 == 4 );
  v22 = sub_E8A0(0, *a4);
  v23 = dcgettext(0, "error waiting for %s", 5);
  error(0, *v11, v23, v22);
  status = 1;
  return 0;
}



// Function: check_normalization @ 0xa840
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



// Function: get_expr @ 0xa8c0
int get_expr(long long *a1, int a2, long long a3)
{
  unsigned int v3; // r13d
  long long v4; // r12
  long long *v5; // rbx
  int v6; // ebp
  long long v7; // rax
  bool v8; // zf
  char *v9; // rax
  long long v10; // rdx
  char v12; // cl
  char *v13; // rax
  long long v14; // rax
  char *v15; // rax
  bool v16; // cc
  long long v17; // rsi
  char *v18; // rax
  char *v19; // rax
  char *v20; // rax
  char *v21; // rax
  char *v22; // rax
  char *v23; // rax
  char *v24; // rax
  char *v25; // rax
  long long v26; // r12
  char *v27; // rax
  char *v28; // rax
  char *v29; // rax
  long long v30; // r12
  char *v31; // rax

  v4 = *a1;
  if ( !*a1 )
    goto LABEL_37;
  v5 = a1;
  v6 = a2;
  switch ( *(int *)(v4 + 16) )
  {
    case 0:
      goto LABEL_35;
    case 1:
      v10 = *(long long *)(v4 + 264);
      *a1 = v10;
      goto LABEL_9;
    case 2:
      goto LABEL_8;
    case 3:
      goto LABEL_34;
    case 4:
      v7 = *(long long *)(v4 + 264);
      if ( !v7 || *(char *)(v7 + 41) )
        goto LABEL_38;
      v8 = *(int *)(v7 + 16) == 5;
      *a1 = v7;
      if ( !v8 )
        goto LABEL_17;
      if ( !*(char *)(v4 + 41) )
        goto LABEL_33;
      v4 = *(long long *)(v7 + 8);
      v9 = dcgettext(0, "invalid expression: expected expression before closing parentheses '%s'.", 5);
      a1 = (long long *)(&dword_0 + 1);
      error(1, 0, v9, v4);
LABEL_8:
      *a1 = *(long long *)(v4 + 264);
      *(long long *)(v4 + 280) = get_expr(a1, 4, v4);
      v10 = *v5;
      goto LABEL_9;
    case 5:
      if ( !a3 )
        goto LABEL_39;
      v12 = *(char *)(v4 + 41);
      if ( (unsigned int)(*(int *)(a3 + 16) - 2) <= 1 )
      {
        if ( !v12 )
          goto LABEL_42;
        goto LABEL_21;
      }
      if ( v12 )
      {
LABEL_21:
        v4 = *(long long *)(a3 + 8);
        v15 = dcgettext(0, "expected an expression after '%s'", 5);
        error(1, 0, v15, v4);
        goto LABEL_22;
      }
      v13 = dcgettext(0, "invalid expression; you have too many ')'", 5);
      a1 = (long long *)(&dword_0 + 1);
      error(1, 0, v13);
LABEL_17:
      v4 = get_expr(a1, 0, v4);
      v14 = *v5;
      if ( !*v5 || *(int *)(v14 + 16) != 5 )
        goto LABEL_41;
      v10 = *(long long *)(v14 + 264);
      *v5 = v10;
LABEL_9:
      if ( !v10 )
        return v4;
      v6 = (short)a2;
      if ( *(int *)(v10 + 20) <= (short)a2 )
        return v4;
LABEL_22:
      v3 = *(int *)(v10 + 16);
      if ( v3 != 5 )
      {
        v16 = v3 <= 4;
        if ( v3 == 4 )
        {
LABEL_29:
          v18 = dcgettext(0, "invalid expression", 5);
          error(1, 0, v18);
        }
        else
        {
          while ( v16 )
          {
            if ( v3 != 3 )
              goto LABEL_29;
            *(long long *)(v10 + 272) = v4;
            v4 = *v5;
            v17 = (unsigned int)*(short *)(*v5 + 20);
            *v5 = *(long long *)(*v5 + 264);
            *(long long *)(v4 + 280) = get_expr(v5, v17, v10);
            v10 = *v5;
            if ( !*v5 || v6 >= *(int *)(v10 + 20) )
              return v4;
            v3 = *(int *)(v10 + 16);
            v16 = v3 <= 4;
            if ( v3 == 4 )
              goto LABEL_29;
          }
        }
        if ( v3 != 5 )
          goto LABEL_40;
        if ( v4 )
          return v4;
      }
      v19 = dcgettext(0, "invalid expression", 5);
      error(1, 0, v19);
LABEL_33:
      v20 = dcgettext(0, "invalid expression; empty parentheses are not allowed.", 5);
      error(1, 0, v20);
LABEL_34:
      v4 = *(long long *)(v4 + 8);
      v21 = dcgettext(0, "invalid expression; you have used a binary operator '%s' with nothing before it.", 5);
      error(1, 0, v21, v4);
LABEL_35:
      v22 = dcgettext(0, "invalid expression", 5);
      error(1, 0, v22);
LABEL_36:
      v23 = dcgettext(0, "oops -- invalid expression type!", 5);
      error(1, 0, v23);
LABEL_37:
      v24 = dcgettext(0, "invalid expression", 5);
      error(1, 0, v24);
LABEL_38:
      v4 = *(long long *)(v4 + 8);
      v25 = dcgettext(
              0,
              "invalid expression; expected to find a ')' but didn't see one. Perhaps you need an extra predicate after '%s'",
              5);
      error(1, 0, v25, v4);
LABEL_39:
      v26 = *(long long *)(v4 + 8);
      v27 = dcgettext(0, "invalid expression: expected expression before closing parentheses '%s'.", 5);
      error(1, 0, v27, v26);
LABEL_40:
      v28 = dcgettext(0, "oops -- invalid expression type (%d)!", 5);
      error(1, 0, v28, v3);
LABEL_41:
      v29 = dcgettext(0, "invalid expression; I was expecting to find a ')' somewhere but did not see one.", 5);
      error(1, 0, v29);
LABEL_42:
      v30 = *(long long *)(a3 + 8);
      v31 = dcgettext(0, "expected an expression between '%s' and ')'", 5);
      error(1, 0, v31, v30);
      return compar((char *)&dword_0 + 1, 0);
    default:
      goto LABEL_36;
  }
}



// Function: set_new_parent @ 0xad30
long long set_new_parent(long long a1, int a2, long long *a3)
{
  long long result; // rax
  float v5; // xmm0_4
  float v6; // xmm0_4

  result = sub_25E50(312);
  *(int *)(result + 16) = 3;
  *(int *)(result + 20) = a2;
  *(int *)(result + 32) = 0;
  switch ( a2 )
  {
    case 2:
      v5 = *(float *)(a1 + 36);
      *(long long *)(result + 8) = "-o";
      *(long long *)result = sub_8940;
      if ( v5 > 1.0 )
      {
LABEL_9:
        v6 = 1.0;
        goto LABEL_7;
      }
LABEL_6:
      v6 = fmaxf(0.0, v5);
LABEL_7:
      *(long long *)(result + 280) = a1;
      *(float *)(result + 36) = v6;
      *a3 = result;
      return result;
    case 3:
      v5 = *(float *)(a1 + 36);
      *(long long *)(result + 8) = "-a";
      *(long long *)result = sub_8990;
      if ( v5 > 1.0 )
        goto LABEL_9;
      goto LABEL_6;
    case 1:
      *a3 = result;
      *(long long *)(result + 8) = ",";
      *(long long *)result = sub_8CA0;
      *(int *)(result + 36) = 1065353216;
      *(long long *)(result + 280) = a1;
      break;
    default:
      *(long long *)(result + 280) = a1;
      *a3 = result;
      break;
  }
  return result;
}



// Function: estimate_costs @ 0xae30
long long estimate_costs(long long a1)
{
  unsigned long long v1; // rbp
  unsigned long long v2; // r14
  long long (**v3)(); // r15
  int v4; // eax
  long long v5; // r12
  char *v6; // rax
  unsigned int v7; // eax
  long long (**v9)(); // r14
  long long (*v10)(); // r15
  long long (*v11)(); // rbp
  unsigned long long v12; // [rsp+8h] [rbp-70h]
  long long (*v13)(); // [rsp+10h] [rbp-68h]
  unsigned int v14; // [rsp+1Ch] [rbp-5Ch]
  long long (*s1)(); // [rsp+28h] [rbp-50h] BYREF
  long long (*s2)(); // [rsp+30h] [rbp-48h] BYREF
  unsigned long long v17; // [rsp+38h] [rbp-40h]

  v17 = __readfsqword(0x28u);
  if ( !a1 )
    return v17 - __readfsqword(0x28u);
  estimate_costs(*(long long *)(a1 + 280));
  estimate_costs(*(long long *)(a1 + 272));
  v14 = 3;
  if ( !*(char *)(a1 + 26) )
  {
    v14 = 1;
    if ( !*(char *)(a1 + 28) )
      v14 = 2 * *(unsigned char *)(a1 + 27);
  }
  v13 = *(long long (**)())a1;
  if ( *(long long (**)(char *))a1 == sub_8FB0 || *(long long (**)())a1 == sub_8FC0 )
  {
    v7 = 8 - *(unsigned char *)(a1 + 56);
LABEL_14:
    *(int *)(a1 + 32) = v7;
    return v17 - __readfsqword(0x28u);
  }
  if ( v13 == sub_FF90 )
  {
    v7 = *(int *)(a1 + 32);
LABEL_19:
    if ( v14 >= v7 )
      v7 = v14;
    goto LABEL_14;
  }
  if ( dword_386B0 )
  {
LABEL_7:
    v1 = 59;
    v2 = 0;
    while ( 1 )
    {
      while ( 1 )
      {
        v12 = (v2 + v1) >> 1;
        v3 = &off_380E0[2 * v12];
        s1 = v13;
        s2 = *v3;
        v4 = memcmp(&s1, &s2, 8u);
        if ( v4 >= 0 )
          break;
        v1 = (v2 + v1) >> 1;
        if ( v12 <= v2 )
        {
LABEL_12:
          v5 = *(long long *)(a1 + 8);
          v6 = dcgettext(
                 0,
                 "warning: there is no entry in the predicate evaluation cost table for predicate %s; please report this as a bug",
                 5);
          error(0, 0, v6, v5);
          v7 = 10;
          goto LABEL_19;
        }
      }
      if ( !v4 )
        break;
      v2 = v12 + 1;
      if ( v1 <= v12 + 1 )
        goto LABEL_12;
    }
    v7 = *((int *)v3 + 2);
    goto LABEL_19;
  }
  v9 = off_380E0;
  qsort(off_380E0, 0x3Bu, 0x10u, compar);
  while ( 1 )
  {
    v10 = v9[2];
    v11 = *v9;
    s1 = v10;
    s2 = v11;
    if ( memcmp(&s1, &s2, 8u) < 0 )
      break;
    s1 = v11;
    s2 = v10;
    if ( memcmp(&s1, &s2, 8u) > 0 )
      __assert_fail("result <= 0", "tree.c", 0x3F5u, "check_sorted");
    v9 += 2;
    if ( &off_38480 == v9 )
    {
      dword_386B0 = 1;
      v13 = *(long long (**)())a1;
      goto LABEL_7;
    }
  }
  error(1, 0, "failed to sort the costlookup array");
  return sub_B0C0();
}



// Function: matches_start_point @ 0xb4b0
bool matches_start_point(char *pattern, unsigned char a2)
{
  int v2; // r12d
  unsigned long long v3; // rbx

  v2 = 16 * a2;
  if ( !qword_386B8 )
    return fnmatch(pattern, ".", v2) == 0;
  v3 = 0;
  do
  {
    if ( !fnmatch(pattern, *(const char **)(qword_386C0 + 8 * v3), v2) )
      return 1;
    ++v3;
  }
  while ( qword_386B8 > v3 );
  return 0;
}



// Function: get_eval_tree @ 0xb580
long long get_eval_tree()
{
  return qword_386D0;
}



// Function: calculate_derived_rates @ 0xb590
long long (*calculate_derived_rates(long long a1))(long long a1, long long a2, long long a3)
{
  long long v1; // r12
  long long v2; // rbp
  long long (*result)(long long, long long, long long); // rax
  float v4; // xmm2_4
  float v5; // xmm1_4
  float v6; // xmm0_4
  float v7; // xmm0_4
  int v8; // xmm0_4

  if ( !a1 )
    __assert_fail("NULL != p", "tree.c", 0x484u, "calculate_derived_rates");
  v1 = *(long long *)(a1 + 280);
  if ( v1 )
    calculate_derived_rates(*(long long *)(a1 + 280));
  v2 = *(long long *)(a1 + 272);
  if ( v2 )
    calculate_derived_rates(*(long long *)(a1 + 272));
  result = (long long (*)(long long, long long, long long))*(unsigned int *)(a1 + 16);
  if ( (int)result == 5 )
    __assert_fail("p->p_type != CLOSE_PAREN", "tree.c", 0x48Bu, "calculate_derived_rates");
  if ( (int)result == 4 )
    __assert_fail("p->p_type != OPEN_PAREN", "tree.c", 0x48Cu, "calculate_derived_rates");
  if ( (int)result != 2 )
  {
    if ( (unsigned int)result <= 2 )
    {
      if ( (int)result )
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
      return result;
    }
    if ( (int)result != 3 )
      __assert_fail("0", "tree.c", 0x4C0u, "calculate_derived_rates");
    result = *(long long (**)(long long, long long, long long))a1;
    if ( *(long long (**)())a1 == sub_8990 )
    {
      if ( v1 )
      {
        v7 = *(float *)(v1 + 36);
        v5 = 1.0;
      }
      else
      {
        v5 = 1.0;
        v7 = 1.0;
      }
      if ( v2 )
        v7 = v7 * *(float *)(v2 + 36);
    }
    else
    {
      if ( result == sub_8CA0 )
      {
        v8 = 1065353216;
LABEL_26:
        *(int *)(a1 + 36) = v8;
        return result;
      }
      if ( (char *)result != (char *)sub_8940 )
        __assert_fail("0", "tree.c", 0x4B4u, "calculate_derived_rates");
      if ( v1 )
      {
        v4 = *(float *)(v1 + 36);
        v5 = 1.0;
      }
      else
      {
        v5 = 1.0;
        v4 = 1.0;
      }
      v6 = 1.0;
      if ( v2 )
        v6 = *(float *)(v2 + 36);
      v7 = v6 + v4;
    }
    if ( v7 <= 1.0 )
      v5 = fmaxf(0.0, v7);
    v8 = LODWORD(v5);
    goto LABEL_26;
  }
  result = sub_9350;
  if ( *(long long (**)(long long, long long, long long))a1 != sub_9350 )
    __assert_fail("pred_is (p, pred_negate)", "tree.c", 0x49Cu, "calculate_derived_rates");
  if ( v2 )
    __assert_fail("NULL == p->pred_left", "tree.c", 0x49Du, "calculate_derived_rates");
  *(float *)(a1 + 36) = 1.0 - *(float *)(v1 + 36);
  return result;
}



// Function: get_new_pred @ 0xb8a0
long long get_new_pred(int *a1)
{
  long long result; // rax
  char v2; // dl

  if ( !*a1 )
    __assert_fail("entry->type != ARG_OPTION", "tree.c", 0x5CDu, "get_new_pred");
  if ( *a1 == 2 )
    __assert_fail("entry->type != ARG_POSITIONAL_OPTION", "tree.c", 0x5CEu, "get_new_pred");
  result = sub_25E50(312);
  if ( ptr )
    *(long long *)(qword_386C8 + 264) = result;
  else
    ptr = (void *)result;
  *(long long *)(result + 304) = a1;
  *(short *)(result + 26) = 257;
  v2 = byte_3862F;
  *(long long *)(result + 16) = 0;
  *(long long *)(result + 48) = "ThisShouldBeSetToSomethingElse";
  *(char *)(result + 40) = v2;
  *(long long *)(result + 32) = 0x3F8000000000000ALL;
  *(long long *)(result + 296) = 0;
  *(long long *)(result + 288) = 0;
  qword_386C8 = result;
  return result;
}



// Function: get_new_pred_chk_op @ 0xb9b0
long long get_new_pred_chk_op(int *a1, long long a2)
{
  int *v3; // rdi
  int v4; // eax
  long long result; // rax
  long long v6; // rax
  char *v7; // rax

  v3 = (int *)qword_386A0;
  if ( !qword_386A0 )
  {
    qword_386A0 = sub_14BC0("and");
    v3 = (int *)qword_386A0;
    if ( !qword_386A0 )
      __assert_fail("entry_and != NULL", "tree.c", 0x5F8u, "get_new_pred_chk_op");
  }
  if ( !qword_386C8 )
    goto LABEL_5;
  v4 = *(int *)(qword_386C8 + 16);
  if ( (v4 & 0xFFFFFFFB) == 1 )
  {
    v6 = sub_B8A0(v3);
    *(long long *)(v6 + 8) = "-a";
    *(long long *)v6 = sub_8990;
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
  return sub_BAC0();
}



// Function: print_tree @ 0xbac0
void print_tree(long long a1, long long a2, int a3)
{
  long long v3; // r12
  int v5; // r14d
  const char *v6; // r8
  const char *v7; // r8
  short v8; // si
  short v9; // dx
  int v10; // ecx
  short *i; // rax
  const char *v12; // rcx
  short v13; // si
  short v14; // dx
  int v15; // ecx
  short *j; // rax
  const char *v17; // r8
  int *v18; // rdx
  long long v19; // rax
  const char *v20; // rcx
  int v21; // r14d
  int v22; // r14d
  const char *v23; // rcx
  const char *v24; // rcx

  if ( a2 )
  {
    v3 = a2;
    do
    {
      v5 = 0;
      if ( a3 > 0 )
      {
        do
        {
          __fprintf_chk(a1, 1, "    ");
          ++v5;
        }
        while ( v5 != a3 );
      }
      __fprintf_chk(a1, 1, "pred=[");
      v6 = *(const char **)(v3 + 48);
      if ( v6 )
        __fprintf_chk(a1, 1, "%s %s", *(const char **)(v3 + 8), v6);
      else
        __fprintf_chk(a1, 1, "%s", *(const char **)(v3 + 8));
      v7 = "no";
      v8 = *(int *)(v3 + 20);
      if ( v8 )
      {
        v9 = 1;
        v10 = 0;
        for ( i = &unk_36100; ; v9 = i[8] )
        {
          ++v10;
          i += 8;
          if ( v8 == v9 || v9 == -1 )
            break;
        }
        v7 = (const char *)*((long long *)&unk_36100 + 2 * v10 + 1);
      }
      v12 = "no";
      v13 = *(int *)(v3 + 16);
      if ( v13 )
      {
        v14 = 1;
        v15 = 0;
        for ( j = &unk_36080; ; v14 = j[8] )
        {
          ++v15;
          j += 8;
          if ( v13 == v14 || v14 == -1 )
            break;
        }
        v12 = (const char *)*((long long *)&unk_36080 + 2 * v15 + 1);
      }
      __fprintf_chk(a1, 1, "] type=%s prec=%s", v12, v7);
      v17 = "";
      v18 = &unk_38020;
      if ( !*(char *)(v3 + 24) )
        v17 = "no ";
      v19 = 0;
      while ( *(int *)(v3 + 32) != *v18 )
      {
        v19 = (unsigned int)(v19 + 1);
        v18 += 4;
        if ( (int)v19 == 11 )
        {
          v20 = "unknown";
          goto LABEL_25;
        }
      }
      v20 = (const char *)*((long long *)&unk_38020 + 2 * v19 + 1);
LABEL_25:
      __fprintf_chk(a1, 1, " cost=%s est_success_rate=%#.4g %sside effects ", v20, *(float *)(v3 + 36), v17);
      if ( (*(long long *)(v3 + 24) & 0xFFFFFF0000LL) != 0 )
      {
        __fprintf_chk(a1, 1, "Needs ");
        if ( *(char *)(v3 + 26) )
        {
          __fprintf_chk(a1, 1, "stat");
          if ( *(char *)(v3 + 28) )
          {
            v23 = ",";
LABEL_39:
            __fprintf_chk(a1, 1, "%sinode", v23);
            if ( !*(char *)(v3 + 27) )
              goto LABEL_26;
LABEL_40:
            v24 = ",";
LABEL_41:
            __fprintf_chk(a1, 1, "%stype", v24);
            goto LABEL_26;
          }
          if ( *(char *)(v3 + 27) )
            goto LABEL_40;
        }
        else
        {
          if ( *(char *)(v3 + 28) )
          {
            v23 = "";
            goto LABEL_39;
          }
          if ( *(char *)(v3 + 27) )
          {
            v24 = "";
            goto LABEL_41;
          }
        }
      }
LABEL_26:
      v21 = 0;
      __fprintf_chk(a1, 1, "\n");
      if ( a3 > 0 )
      {
        do
        {
          __fprintf_chk(a1, 1, "    ");
          ++v21;
        }
        while ( v21 != a3 );
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
      v22 = 0;
      if ( a3 > 0 )
      {
        do
        {
          __fprintf_chk(a1, 1, "    ");
          ++v22;
        }
        while ( v22 != a3 );
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



// Function: predlist_merge_sort @ 0xbef0
void predlist_merge_sort(long long *a1, long long *a2)
{
  long long v2; // r14
  long long v5; // rbp
  long long v6; // r12
  float v7; // xmm0_4
  float v8; // xmm1_4
  long long v9; // rdx
  long long v10; // r8
  long long v11; // rdi
  long long v12; // rax
  unsigned int v13; // esi
  long long v14; // rdx

  v2 = *a1;
  if ( *a1 )
  {
    if ( (qword_38660 & 8) == 0 )
    {
      sub_B590(*a1);
LABEL_4:
      v5 = 0;
      v6 = 0;
      while ( 1 )
      {
        while ( 1 )
        {
          v10 = v2;
          v2 = *(long long *)(v2 + 272);
          *a1 = v2;
          *(long long *)(v10 + 272) = 0;
          if ( v5 )
            break;
LABEL_16:
          *(long long *)(v10 + 272) = v5;
          v5 = v10;
          if ( !v6 )
            v6 = v10;
          if ( !v2 )
            goto LABEL_19;
        }
        v11 = *(long long *)(v10 + 280);
        v12 = v5;
        v13 = *(int *)(v11 + 32);
        while ( 1 )
        {
          v14 = *(long long *)(v12 + 280);
          if ( *(int *)(v14 + 32) == v13 )
            break;
          if ( *(int *)(v14 + 32) >= v13 )
            goto LABEL_8;
LABEL_15:
          v12 = *(long long *)(v12 + 272);
          if ( !v12 )
            goto LABEL_16;
        }
        v7 = *(float *)(v14 + 36);
        v8 = *(float *)(v11 + 36);
        if ( v7 == v8 )
          goto LABEL_8;
        if ( *(int *)(v12 + 20) != 2 )
          break;
        if ( v8 <= v7 )
          goto LABEL_15;
LABEL_8:
        v9 = *(long long *)(v12 + 272);
        *(long long *)(v10 + 272) = v9;
        if ( !v9 )
          v6 = v10;
        *(long long *)(v12 + 272) = v10;
        if ( !v2 )
          goto LABEL_19;
      }
      if ( v8 <= v7 )
        goto LABEL_8;
      goto LABEL_15;
    }
    __fprintf_chk(stderr, 1, "%s:\n", "predlist before merge sort");
    sub_BAC0((long long)stderr, *a1, 2);
    v2 = *a1;
    sub_B590(*a1);
    if ( v2 )
      goto LABEL_4;
    v5 = 0;
    v6 = 0;
LABEL_19:
    if ( (qword_38660 & 8) != 0 )
    {
      __fprintf_chk(stderr, 1, "%s:\n", "predlist after merge sort");
      sub_BAC0((long long)stderr, v5, 2);
    }
    sub_B590(v5);
    *(long long *)(v6 + 272) = *a2;
    *a2 = v5;
    a1[1] = 0;
    *a1 = 0;
  }
}



// Function: opt_expr @ 0xc110
long long opt_expr(long long *a1)
{
  long long v1; // r9
  long long *v2; // r15
  long long v3; // rcx
  char *v4; // rax
  long long v5; // rdi
  long long v6; // r13
  int v7; // eax
  unsigned int v8; // ebx
  unsigned int v9; // r12d
  long long v10; // r14
  unsigned int v11; // eax
  bool (*v12)(char *, long long, long long); // rcx
  long long v13; // rax
  long long v14; // rax
  bool v16; // zf
  long long v17; // rax
  long long *v18; // rdx
  long long v19; // rsi
  long long v20; // rax
  long long v21; // rax
  const char *v22; // r8
  const char *v23; // rcx
  long long v24; // rax
  const char *v25; // r8
  const char *v26; // rcx
  int *v27; // rdx
  long long v28; // rax
  const char *v29; // rcx
  char *v30; // rax
  long long v31; // [rsp+0h] [rbp-118h] BYREF
  long long v32; // [rsp+8h] [rbp-110h]
  long long v33; // [rsp+10h] [rbp-108h] BYREF
  long long v34; // [rsp+18h] [rbp-100h]
  char v35[176]; // [rsp+20h] [rbp-F8h] BYREF
  char v36; // [rsp+D0h] [rbp-48h] BYREF
  unsigned long long v37; // [rsp+D8h] [rbp-40h]

  v1 = *a1;
  v37 = __readfsqword(0x28u);
  v31 = 0;
  v32 = 0;
  v33 = 0;
  v34 = 0;
  if ( !v1 )
    return 0;
  v2 = a1;
  v3 = v1;
  v4 = v35;
  do
  {
    *((long long *)v4 + 1) = 0;
    v4 += 16;
    *((long long *)v4 - 2) = 0;
  }
  while ( v4 != &v36 );
  v5 = *(long long *)(v1 + 272);
  if ( v5 )
  {
    while ( *(long long *)(v5 + 272) )
    {
      v3 = v5;
      v5 = *(long long *)(v5 + 272);
    }
    v19 = *(unsigned int *)(v3 + 20);
    v18 = (long long *)(v3 + 272);
  }
  else
  {
    v18 = v2;
    v5 = v1;
    v19 = 3;
  }
  if ( *(int *)(v5 + 16) == 3 )
  {
    v6 = v1;
    if ( (qword_38660 & 9) == 0 )
      goto LABEL_11;
    goto LABEL_45;
  }
  sub_AD30(v5, v19, v18);
  if ( (qword_38660 & 9) != 0 )
  {
LABEL_45:
    __fprintf_chk(stderr, 1, "Normalized Eval Tree:\n");
    v19 = *v2;
    sub_BAC0((long long)stderr, *v2, 0);
  }
  v6 = *v2;
  if ( *v2 )
  {
LABEL_11:
    v7 = *(int *)(v6 + 16);
    v8 = 0;
    if ( v7 == 3 )
      v8 = *(int *)(v6 + 20);
    v9 = 0;
    if ( v7 == 3 )
      goto LABEL_42;
    while ( 1 )
    {
      v10 = *(long long *)(v6 + 280);
      v11 = *(int *)(v10 + 16);
      v12 = *(bool (**)(char *, long long, long long))v10;
      if ( v11 == 2 )
        break;
      if ( v11 > 2 )
      {
        if ( v11 != 3 )
        {
          v30 = dcgettext(0, "oops -- invalid expression type!", 5);
          error(1, 0, v30);
        }
        *(char *)(v10 + 24) = opt_expr(v6 + 280, v19, v18, v12);
        v10 = *(long long *)(v6 + 280);
LABEL_37:
        if ( *(char *)(v10 + 24) )
        {
LABEL_38:
          v19 = 11;
          v9 = 1;
          sub_C0B0(v35, 11, &v33, &v31, v2);
        }
LABEL_39:
        v2 = (long long *)(v6 + 272);
        goto LABEL_40;
      }
      if ( v8 == 1 )
        goto LABEL_37;
      if ( *(char *)(v10 + 24) )
        goto LABEL_38;
      LOBYTE(v19) = (char *)v12 == (char *)sub_9150;
      if ( (char *)v12 != (char *)sub_9150
        && v12 != sub_9640
        && (char *)v12 != (char *)sub_9310
        && v12 != sub_91E0
        && ((v19 = (unsigned short)word_38688,
             (char *)v12 == (char *)sub_8CA0
          || word_38688 == 0
          || (char *)v12 == (char *)sub_8990
          || (char *)v12 == (char *)sub_9350)
         || (char *)v12 == (char *)sub_8940
         || *(int *)(v10 + 32)) )
      {
        if ( v12 == sub_97E0 )
        {
          *v2 = *(long long *)(v6 + 272);
          v20 = v31;
          v16 = v32 == 0;
          v31 = v6;
          *(long long *)(v6 + 272) = v20;
          if ( v16 )
            v32 = v6;
        }
        else
        {
          if ( (unsigned short)word_38688 <= 1u )
            goto LABEL_39;
          v13 = *(unsigned int *)(v10 + 32);
          if ( ((unsigned int)(v13 - 1) > 1 || *(char *)(v10 + 26)) && word_38688 == 2 )
            goto LABEL_39;
          if ( (qword_38660 & 8) != 0 )
          {
            __fprintf_chk(stderr, 1, "-O%d: categorising predicate ", (unsigned short)word_38688);
            v24 = *(long long *)(v6 + 280);
            v25 = *(const char **)(v24 + 48);
            v26 = *(const char **)(v24 + 8);
            if ( v25 )
              __fprintf_chk(stderr, 1, "%s %s", v26, v25);
            else
              __fprintf_chk(stderr, 1, "%s", v26);
            v27 = &unk_38020;
            v28 = 0;
            while ( *(int *)(*(long long *)(v6 + 280) + 32LL) != *v27 )
            {
              v28 = (unsigned int)(v28 + 1);
              v27 += 4;
              if ( (int)v28 == 11 )
              {
                v29 = "unknown";
                goto LABEL_64;
              }
            }
            v29 = (const char *)*((long long *)&unk_38020 + 2 * v28 + 1);
LABEL_64:
            v19 = 1;
            __fprintf_chk(stderr, 1, " by cost (%s)\n", v29);
            v13 = *(unsigned int *)(*(long long *)(v6 + 280) + 32LL);
          }
          v14 = 16 * v13;
          v18 = (long long *)&v35[v14];
          *v2 = *(long long *)(v6 + 272);
          *(long long *)(v6 + 272) = *(long long *)&v35[v14];
          *(long long *)&v35[v14] = v6;
          if ( !*(long long *)&v35[v14 + 8] )
            *(long long *)&v35[v14 + 8] = v6;
        }
      }
      else
      {
        if ( (qword_38660 & 8) != 0 )
        {
          __fprintf_chk(stderr, 1, "-O%d: promoting cheap predicate ", (unsigned short)word_38688);
          v21 = *(long long *)(v6 + 280);
          v22 = *(const char **)(v21 + 48);
          v23 = *(const char **)(v21 + 8);
          if ( v22 )
            __fprintf_chk(stderr, 1, "%s %s", v23, v22);
          else
            __fprintf_chk(stderr, 1, "%s", v23);
          v19 = 1;
          __fprintf_chk(stderr, 1, " into name_list\n");
        }
        v16 = v34 == 0;
        *v2 = *(long long *)(v6 + 272);
        v17 = v33;
        v33 = v6;
        *(long long *)(v6 + 272) = v17;
        if ( v16 )
          v34 = v6;
      }
LABEL_40:
      v6 = *v2;
      if ( !*v2 )
        goto LABEL_33;
      if ( *(int *)(v6 + 16) == 3 )
      {
LABEL_42:
        if ( *(int *)(v6 + 20) != v8 )
        {
          v19 = v8;
          v6 = sub_AD30(v6, v8, v2);
        }
      }
    }
    *(char *)(v10 + 24) = opt_expr(v10 + 280, v19, v18, v12);
    v10 = *(long long *)(v6 + 280);
    goto LABEL_37;
  }
  v9 = 0;
LABEL_33:
  sub_C0B0(v35, 11, &v33, &v31, v2);
  return v9;
}



// Function: do_arm_swaps @ 0xc690
long long do_arm_swaps(long long a1, long long a2)
{
  char v2; // r8
  const char *v3; // rcx
  long long v4; // rdx
  long long v5; // rdx
  long long v6; // rcx
  long long v8; // rax
  long long v9; // r10
  long long v10; // r10
  long long v11; // r11
  long long v12; // r14
  long long v13; // r11
  long long v14; // r15
  long long v15; // rdx
  long long v16; // r10
  long long v17; // r11
  unsigned int v18; // r9d
  unsigned int v19; // eax
  unsigned int v20; // eax
  unsigned int v21; // edx
  bool v22; // zf
  long long v23; // rcx
  float v24; // xmm2_4
  float v25; // xmm3_4
  long long (*v26)(); // rax
  float v27; // [rsp+4h] [rbp-3Ch]

  if ( a1 )
  {
    while ( 1 )
    {
      v2 = qword_38660;
      v3 = "Not a binary operation";
      v4 = qword_38660 & 8;
      if ( *(int *)(a1 + 16) != 3 )
        goto LABEL_3;
      v8 = *(long long *)(a1 + 272);
      while ( 1 )
      {
        if ( !v8 || !*(long long *)(a1 + 280) )
        {
          v3 = "Doesn't have two arms";
          v4 = v2 & 8;
          goto LABEL_3;
        }
        v9 = *(long long *)(v8 + 280);
        if ( !v9 )
        {
          v3 = "Left arm has no child on RHS";
          v4 = v2 & 8;
          goto LABEL_3;
        }
        if ( *(char *)(v9 + 24)
          || (unsigned char)sub_B430(*(long long *)(v9 + 272))
          || (a2 = *(long long *)(v10 + 280), (unsigned char)sub_B430(a2)) )
        {
          v3 = "Left subtree has side-effects";
          v4 = v2 & 8;
          goto LABEL_3;
        }
        if ( *(char *)(v11 + 24)
          || (v12 = *(long long *)(v11 + 272), (unsigned char)sub_B430(v12))
          || (v14 = *(long long *)(v13 + 280), (unsigned char)sub_B430(v14)) )
        {
          v3 = "Right subtree has side-effects";
          v4 = v2 & 8;
          goto LABEL_3;
        }
        v18 = 0;
        if ( v15 )
          v18 = sub_B0C0(v15);
        v19 = 0;
        if ( a2 )
          v19 = sub_B0C0(a2);
        if ( v18 < *(int *)(v16 + 32) )
          v18 = *(int *)(v16 + 32);
        if ( v18 < v19 )
          v18 = v19;
        a2 = 0;
        if ( v12 )
          a2 = (unsigned int)sub_B0C0(v12);
        v20 = 0;
        if ( v14 )
          v20 = sub_B0C0(v14);
        v21 = *(int *)(v17 + 32);
        if ( (unsigned int)a2 < v21 )
          a2 = v21;
        if ( (unsigned int)a2 < v20 )
          a2 = v20;
        v4 = v2 & 8;
        if ( (int)v18 < (int)a2 )
        {
          v3 = "efficient as-is";
          goto LABEL_3;
        }
        if ( v18 != (int)a2 )
          goto LABEL_41;
        v24 = *(float *)(v16 + 36);
        v25 = *(float *)(v17 + 36);
        if ( (v2 & 8) != 0 )
          break;
        v26 = *(long long (**)())a1;
        if ( *(long long (**)())a1 != sub_8940 )
          goto LABEL_47;
LABEL_50:
        if ( v24 <= v25 )
        {
          v3 = "Operation is OR; right success rate >= left";
          goto LABEL_3;
        }
LABEL_41:
        if ( v4 )
        {
          __fprintf_chk(stderr, 1, "Performing arm swap on:\n");
          sub_BAC0((long long)stderr, a1, 0);
          v2 = qword_38660;
          v4 = qword_38660 & 8;
        }
        v8 = *(long long *)(a1 + 272);
        a2 = *(long long *)(a1 + 280);
        v22 = *(int *)(a1 + 16) == 3;
        v23 = *(long long *)(v8 + 280);
        *(long long *)(v8 + 280) = a2;
        *(long long *)(a1 + 280) = v23;
        if ( !v22 )
        {
          v3 = "Not a binary operation";
          goto LABEL_3;
        }
      }
      a2 = 1;
      v27 = *(float *)(v16 + 36);
      __fprintf_chk(stderr, 1, "Success rates: l=%f, r=%f\n", v24, v25);
      v2 = qword_38660;
      v26 = *(long long (**)())a1;
      v24 = v27;
      v4 = qword_38660 & 8;
      if ( *(long long (**)())a1 == sub_8940 )
        goto LABEL_50;
LABEL_47:
      if ( v26 != sub_8990 )
      {
        v3 = "Not 'AND' or 'OR'";
        goto LABEL_3;
      }
      if ( v25 > v24 )
        goto LABEL_41;
      v3 = "Operation is AND; right success rate <= left";
LABEL_3:
      if ( v4 )
      {
        __fprintf_chk(stderr, 1, "Not an arm swap candidate (%s):\n", v3);
        a2 = a1;
        sub_BAC0((long long)stderr, a1, 0);
      }
      if ( !(unsigned char)do_arm_swaps(*(long long *)(a1 + 272), a2, v4, v3)
        && !(unsigned char)do_arm_swaps(*(long long *)(a1 + 280), a2, v5, v6) )
      {
        return 0;
      }
    }
  }
  return 0;
}



// Function: build_expression_tree @ 0xc9f0
// write access to const memory has been detected, the output may be wrong!
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
  long long result; // rax
  void *v32; // rdi
  long long v33; // rdi
  char *v34; // rax
  char *v35; // rax
  char *v36; // rax
  long long v37; // r12
  char *v38; // rax
  char *v39; // rax
  char *v40; // rax
  unsigned int v41; // eax
  int v42; // ecx
  long long v43; // [rsp+8h] [rbp-70h]
  int v44[4]; // [rsp+1Ch] [rbp-5Ch] BYREF
  int v45; // [rsp+2Ch] [rbp-4Ch] BYREF
  void *ptr[9]; // [rsp+30h] [rbp-48h] BYREF

  v3 = a2;
  v44[0] = a1;
  ptr[1] = (void *)__readfsqword(0x28u);
  v4 = a3;
  v45 = a3;
  ::ptr = 0;
  qword_386C0 = a2 + 8LL * a3;
  if ( a3 < a1 )
  {
    while ( !(unsigned char)sub_E120(*(long long *)(a2 + 8 * v4), 1) )
    {
      ++qword_386B8;
      v5 = v45 + 1;
      v45 = v5;
      if ( v5 >= v44[0] )
        break;
      v4 = v5;
    }
  }
  v6 = sub_14BC0("(");
  v43 = sub_14BC0((char *)")");
  v7 = sub_14BC0("print");
  v8 = v7;
  if ( !v6 )
    __assert_fail("entry_open != NULL", "tree.c", 0x4F6u, "build_expression_tree");
  if ( !v43 )
    __assert_fail("entry_close != NULL", "tree.c", 0x4F7u, "build_expression_tree");
  if ( !v7 )
    __assert_fail("entry_print != NULL", "tree.c", 0x4F8u, "build_expression_tree");
  sub_12C60(v6, a2, v44);
  v9 = v44[0];
  v10 = ::ptr;
  *(long long *)(qword_386C8 + 8) = "(";
  v10[41] = 1;
  sub_14BA0(v3, v9);
  sub_9E60(qword_386C8);
  v11 = v45;
  v12 = v44[0];
  if ( v45 >= v44[0] )
  {
LABEL_23:
    sub_14BB0(v3, v12, qword_386C8, ::ptr);
    if ( *((long long *)::ptr + 33) )
    {
      if ( !(unsigned char)sub_E0E0() )
        goto LABEL_31;
      sub_12A60(v43, v3, v44);
      v22 = qword_386C8;
      *(long long *)(qword_386C8 + 8) = ")";
      *(char *)(v22 + 41) = 1;
      sub_9E60(v22);
      sub_11E20(v8, v3, v44);
      v23 = qword_386C8;
      *(long long *)(qword_386C8 + 8) = "-print";
      *(char *)(v23 + 41) = 1;
      sub_9E60(v23);
      sub_9E60((long long)::ptr);
    }
    else
    {
      v32 = ::ptr;
      ptr[0] = ::ptr;
      qword_386C8 = 0;
      ::ptr = 0;
      free(v32);
      sub_11E20(v8, v3, v44);
      v33 = qword_386C8;
      *(long long *)(qword_386C8 + 8) = "-print";
      sub_9E60(v33);
      sub_9E60((long long)::ptr);
    }
    if ( (qword_38660 & 9) != 0 )
      goto LABEL_32;
    while ( 1 )
    {
      sub_14A70(::ptr);
      sub_9E60((long long)::ptr);
      ptr[0] = ::ptr;
      LODWORD(v24) = sub_A8C0((long long *)ptr, 0, 0);
      qword_386D0 = v24;
      v3 = v24;
      sub_B590(v24);
      if ( !ptr[0] )
        goto LABEL_34;
      if ( *(long long (**)())ptr[0] == sub_8BE0 )
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
      sub_9E60((long long)v27);
      free(ptr[0]);
      if ( (qword_38660 & 9) != 0 )
      {
LABEL_32:
        __fprintf_chk(stderr, 1, "Predicate List:\n");
        sub_9CD0((long long)stderr, (long long)::ptr);
      }
    }
  }
  while ( 1 )
  {
    v15 = *(long long *)(v3 + 8 * v11);
    byte_38618 = 0;
    v16 = sub_E120(v15, 0);
    v17 = *(long long *)(v3 + 8LL * v45);
    if ( !v16 )
      goto LABEL_42;
    v18 = sub_14BC0(*(char **)(v3 + 8LL * v45));
    v19 = (int *)v18;
    if ( !v18 )
    {
      v35 = dcgettext(0, "unknown predicate `%s'", 5);
      error(1, 0, v35, v17);
LABEL_42:
      v36 = dcgettext(0, "paths must precede expression: `%s'", 5);
      error(0, 0, v36, v17);
      if ( !access(*(const char **)(v3 + 8LL * v45), 0) )
      {
        v37 = *(long long *)(qword_386C8 + 8);
        v38 = dcgettext(0, "possible unquoted pattern after predicate `%s'?", 5);
        error(0, 0, v38, v37);
      }
      exit(1);
    }
    v20 = v45;
    if ( *(int *)v18 != 4 )
      v20 = ++v45;
    if ( !(*(unsigned char (**)(long long, long long, int *))(v18 + 16))(v18, v3, &v45) )
      break;
    v13 = qword_386C8;
    v14 = 0;
    *(long long *)(qword_386C8 + 8) = v17;
    if ( v45 != v20 )
      v14 = *(long long *)(v3 + 8LL * v20);
    *(long long *)(v13 + 48) = v14;
    sub_9E60(v13);
    sub_9E60((long long)::ptr);
    v11 = v45;
    v12 = v44[0];
    if ( v45 >= v44[0] )
      goto LABEL_23;
  }
  v8 = *(long long *)(v3 + 8LL * v45);
  if ( !v8 )
  {
    v40 = dcgettext(0, "missing argument to `%s'", 5);
    error(1, 0, v40, v17);
    LOBYTE(v41) = sub_9020 == (long long (*)())0x10102464C45LL;
    LOBYTE(v42) = sub_FF90 == (long long (*)())0x10102464C45LL;
    result = v42 | v41;
    LOBYTE(result) = (&sub_8FE0 == (_UNKNOWN *)0x10102464C45LL) | result;
    if ( (char)result || sub_9060 == (long long (*)())0x10102464C45LL )
      *(long long *)((char *)&dword_40 + 1) = 0;
    return result;
  }
  if ( *v19 != 4 || v45 != v20 )
  {
    v21 = dcgettext(0, "invalid argument `%s' to `%s'", 5);
    v12 = 0;
    error(1, 0, v21, v8, v17);
    goto LABEL_23;
  }
  v28 = dcgettext(0, "invalid predicate `%s'", 5);
  error(1, 0, v28, v17);
LABEL_34:
  if ( (qword_38660 & 9) != 0 )
  {
LABEL_40:
    __fprintf_chk(stderr, 1, "Eval Tree:\n");
    sub_BAC0((long long)stderr, qword_386D0, 0);
    v3 = qword_386D0;
  }
  sub_AE30(v3);
  sub_C110(&qword_386D0);
  v29 = qword_386D0;
  sub_A840(qword_386D0, 1);
  sub_C690(v29, 1);
  v30 = qword_386D0;
  sub_A840(qword_386D0, 1);
  if ( (qword_38660 & 9) != 0 )
  {
    __fprintf_chk(stderr, 1, "Optimized Eval Tree:\n");
    sub_BAC0((long long)stderr, qword_386D0, 0);
    __fprintf_chk(stderr, 1, "Optimized command line:\n");
    sub_9DE0(stderr, qword_386D0);
    __fprintf_chk(stderr, 1, "\n");
    return qword_386D0;
  }
  return v30;
}



// Function: do_complete_pending_execdirs @ 0xd130
void do_complete_pending_execdirs(long long a1)
{
  long long v1; // rbx
  long long v2; // rdx
  void *v3; // rdi

  if ( a1 )
  {
    v1 = a1;
    do
    {
      while ( 1 )
      {
        if ( !byte_38608 )
          __assert_fail("state.execdirs_outstanding", "util.c", 0x18Cu, "do_complete_pending_execdirs");
        do_complete_pending_execdirs(*(long long *)(v1 + 272));
        if ( *(long long (**)())v1 == sub_8FC0 || *(long long (**)(long long, long long, long long))v1 == sub_95D0 )
        {
          if ( *(char *)(v1 + 56) )
          {
            if ( *(int *)(v1 + 208) )
            {
              sub_159F0(*(long long *)(v1 + 248), sub_D220, v1 + 56);
              v3 = *(void **)(v1 + 248);
              if ( v3 != qword_385C0 )
                break;
            }
          }
        }
        v1 = *(long long *)(v1 + 280);
        if ( !v1 )
          return;
      }
      sub_242C0(v3, sub_D220, v2);
      free(*(void **)(v1 + 248));
      *(long long *)(v1 + 248) = 0;
      v1 = *(long long *)(v1 + 280);
    }
    while ( v1 );
  }
}



// Function: exec_cb @ 0xd220
long long exec_cb(long long a1)
{
  sub_15430(a1 + 8, a1 + 96);
  return 0;
}



// Function: complete_pending_execs @ 0xd240
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
      if ( *(long long (**)(char *))v1 == sub_8FB0 && *(char *)(v1 + 56) )
      {
        result = *(unsigned int *)(v1 + 208);
        if ( (int)result )
          result = sub_15430(v1 + 64, v1 + 152);
      }
      v1 = *(long long *)(v1 + 280);
    }
    while ( v1 );
  }
  return result;
}



// Function: insert_primary_noarg @ 0xd520
long long *insert_primary_noarg(long long a1)
{
  long long v1; // rbp
  long long *result; // rax
  long long v3; // rdx

  v1 = *(long long *)(a1 + 24);
  if ( !v1 )
    sub_D2B0();
  result = (long long *)sub_B9B0((int *)a1, 0);
  v3 = *(long long *)(a1 + 8);
  *result = v1;
  result[1] = v3;
  result[7] = 0;
  result[2] = 1;
  return result;
}



// Function: usage @ 0xd570
void usage(int status)
{
  long long v1; // r12
  char *v2; // rax
  char *v3; // rax
  const char **v4; // rbx
  FILE *v5; // rbp
  char *v6; // rax
  FILE *v7; // rbp
  char *v8; // rax
  FILE *v9; // rbp
  char *v10; // rax
  FILE *v11; // rbp
  char *v12; // rax
  FILE *v13; // rbp
  char *v14; // rax
  FILE *v15; // rbp
  char *v16; // rax
  FILE *v17; // rbp
  char *v18; // rax
  FILE *v19; // rbp
  char *v20; // rax
  FILE *v21; // rbp
  char *v22; // rax
  FILE *v23; // rbp
  char *v24; // rax
  long long i; // rax
  const char *v26; // r8
  long long v27; // rbp
  FILE *v28; // rbp
  char *v29; // rax

  v1 = qword_38ED0;
  if ( status )
  {
    v2 = dcgettext(0, "Try '%s --help' for more information.\n", 5);
    __fprintf_chk(stderr, 1, v2, v1);
    exit(status);
  }
  v3 = dcgettext(0, "Usage: %s [-H] [-L] [-P] [-Olevel] [-D debugopts] [path...] [expression]\n", 5);
  v4 = (const char **)&off_36180;
  __fprintf_chk(stdout, 1, v3, v1);
  v5 = stdout;
  v6 = dcgettext(
         0,
         "\n"
         "Default path is the current directory; default expression is -print.\n"
         "Expression may consist of: operators, options, tests, and actions.\n",
         5);
  fputs(v6, v5);
  v7 = stdout;
  v8 = dcgettext(
         0,
         "\n"
         "Operators (decreasing precedence; -and is implicit where no others are given):\n"
         "      ( EXPR )   ! EXPR   -not EXPR   EXPR1 -a EXPR2   EXPR1 -and EXPR2\n"
         "      EXPR1 -o EXPR2   EXPR1 -or EXPR2   EXPR1 , EXPR2\n",
         5);
  fputs(v8, v7);
  v9 = stdout;
  v10 = dcgettext(0, "\nPositional options (always true):\n      -daystart -follow -nowarn -regextype -warn\n", 5);
  fputs(v10, v9);
  v11 = stdout;
  v12 = dcgettext(
          0,
          "\n"
          "Normal options (always true, specified before other expressions):\n"
          "      -depth -files0-from FILE -maxdepth LEVELS -mindepth LEVELS\n"
          "       -mount -noleaf -xdev -ignore_readdir_race -noignore_readdir_race\n",
          5);
  fputs(v12, v11);
  v13 = stdout;
  v14 = dcgettext(
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
  fputs(v14, v13);
  v15 = stdout;
  v16 = dcgettext(
          0,
          "\n"
          "Actions:\n"
          "      -delete -print0 -printf FORMAT -fprintf FILE FORMAT -print \n"
          "      -fprint0 FILE -fprint FILE -ls -fls FILE -prune -quit\n"
          "      -exec COMMAND ; -exec COMMAND {} + -ok COMMAND ;\n"
          "      -execdir COMMAND ; -execdir COMMAND {} + -okdir COMMAND ;\n",
          5);
  fputs(v16, v15);
  v17 = stdout;
  v18 = dcgettext(0, "\nOther common options:\n", 5);
  fputs(v18, v17);
  v19 = stdout;
  v20 = dcgettext(0, "      --help                   display this help and exit\n", 5);
  fputs(v20, v19);
  v21 = stdout;
  v22 = dcgettext(0, "      --version                output version information and exit\n\n", 5);
  fputs(v22, v21);
  v23 = stdout;
  v24 = dcgettext(0, "Valid arguments for -D:\n", 5);
  fputs(v24, v23);
  for ( i = 0; ; i = v27 )
  {
    v26 = *v4;
    v27 = i + 1;
    if ( i )
    {
      __fprintf_chk(stdout, 1, "%s%s", ", ", v26);
      if ( v27 == 9 )
      {
        v28 = stdout;
        v29 = dcgettext(0, "\nUse '-D help' for a description of the options, or see find(1)\n\n", 5);
        fputs(v29, v28);
        sub_175F0(stdout, qword_38ED0);
        exit(0);
      }
    }
    else
    {
      __fprintf_chk(stdout, 1, "%s%s", "", v26);
    }
    v4 += 3;
  }
}



// Function: get_statinfo @ 0xd820
long long get_statinfo(long long a1, long long a2, long long a3)
{
  int v4; // eax
  long long v5; // r12
  char *v6; // rax
  long long result; // rax
  int v8; // r12d
  const char *v9; // rax

  if ( (char)word_385E4 )
  {
    v4 = *(int *)(a3 + 24);
LABEL_6:
    dword_385E8 = v4;
    result = 0;
    word_385E4 = 257;
    return result;
  }
  if ( !(unsigned int)qword_38670(a2, a3) )
  {
    v4 = *(int *)(a3 + 24);
    if ( !v4 )
    {
      v5 = sub_239F0(0, (unsigned int)dword_3868C, a2);
      v6 = dcgettext(0, "WARNING: file %s appears to have mode 0000", 5);
      error(0, 0, v6, v5);
      v4 = *(int *)(a3 + 24);
      status = 1;
    }
    goto LABEL_6;
  }
  v8 = *__errno_location();
  if ( !byte_3862E || (result = 0xFFFFFFFFLL, v8 != 2) )
  {
    if ( !byte_38618 )
    {
      v9 = (const char *)sub_239F0(0, (unsigned int)dword_3868C, a1);
      error(0, v8, "%s", v9);
      status = 1;
    }
    byte_38618 = 1;
    return 0xFFFFFFFFLL;
  }
  return result;
}



// Function: check_nofollow @ 0xd920
bool check_nofollow()
{
  __m128i v0; // xmm1
  float v1; // ebx
  struct utsname _0; // [rsp+0h] [rbp+0h] BYREF
  unsigned long long vars188; // [rsp+188h] [rbp+188h]

  vars188 = __readfsqword(0x28u);
  if ( uname(&_0) )
    return 1;
  v0 = 0;
  *(float *)v0.m128i_i32 = strtod(_0.release, 0);
  v1 = COERCE_FLOAT(_mm_cvtsi128_si32(v0));
  if ( !strcmp("Linux", _0.sysname) )
    return v1 >= 2.2;
  if ( strcmp("FreeBSD", _0.sysname) )
    return 1;
  return v1 >= 3.0999999;
}



// Function: record_initial_cwd @ 0xda20
long long record_initial_cwd()
{
  long long result; // rax
  void *v1; // rax
  char *v2; // rbx
  char *v3; // r13
  char *v4; // r12
  int *v5; // rax

  qword_385C0 = (void *)sub_25A50(16);
  result = sub_24230(qword_385C0);
  if ( (int)result )
  {
    v1 = qword_385C0;
    v2 = "";
    v3 = "";
    if ( *(int *)qword_385C0 < 0 )
      goto LABEL_4;
    while ( 1 )
    {
      v4 = dcgettext(0, "Failed to save initial working directory%s%s", 5);
      v5 = __errno_location();
      error(1, *v5, v4, v3, v2);
LABEL_4:
      v2 = (char *)*((long long *)v1 + 1);
      v3 = (char *)": ";
      if ( !v2 )
      {
        v2 = "";
        v3 = "";
      }
    }
  }
  return result;
}



// Function: cleanup @ 0xdab0
void cleanup(long long a1, long long (*a2)(long long a1))
{
  long long v2; // rax
  long long (**v3)(); // rbx
  long long v4; // rbp
  long long v5; // rdi
  long long (*v6)(); // rdi
  long long v7; // rdx
  int v8; // ebp
  const char *v9; // rax
  long long v10; // rax
  char *v11; // rbx
  char *v12; // r13
  char *v13; // r12
  int *v14; // rax

  v2 = sub_B580();
  if ( v2 )
  {
    v3 = (long long (**)())v2;
    v4 = v2;
    do
    {
      v5 = *(long long *)(v4 + 272);
      if ( v5 )
      {
        a2 = sub_D240;
        sub_D2E0(v5, sub_D240);
      }
      sub_D240(v4);
      v4 = *(long long *)(v4 + 280);
    }
    while ( v4 );
    if ( byte_38608 )
    {
      v10 = sub_B580();
      sub_D130(v10);
      byte_38608 = 0;
    }
    sub_EB20(qword_38610);
    do
    {
      v6 = v3[34];
      if ( v6 )
      {
        a2 = (long long (*)(long long))&loc_D0F0;
        sub_D2E0(v6, &loc_D0F0);
      }
      if ( (char *)*v3 == (char *)&sub_8FE0 || *v3 == sub_9020 || *v3 == sub_FF90 || *v3 == sub_9060 )
        v3[8] = 0;
      v3 = (long long (**)())v3[35];
    }
    while ( v3 );
  }
  else
  {
    sub_EB20(qword_38610);
  }
  if ( (unsigned int)sub_24290(qword_385C0) )
  {
    v11 = "";
    v12 = "";
    if ( *(int *)qword_385C0 < 0 )
    {
      v11 = (char *)*((long long *)qword_385C0 + 1);
      v12 = (char *)": ";
      if ( !v11 )
      {
        v11 = "";
        v12 = "";
      }
    }
    v13 = dcgettext(0, "Failed to restore initial working directory%s%s", 5);
    v14 = __errno_location();
    error(0, *v14, v13, v12, v11);
    _exit(1);
  }
  sub_242C0(qword_385C0, a2, v7);
  free(qword_385C0);
  qword_385C0 = 0;
  if ( (unsigned char)sub_16220() )
  {
    sub_16240();
    sub_161F0();
  }
  if ( (unsigned int)sub_180C0(stdout) == -1 )
  {
    v8 = *__errno_location();
    v9 = (const char *)sub_239F0(0, (unsigned int)dword_3868C, "standard output");
    error(0, v8, "%s", v9);
    status = 1;
  }
}



// Function: optionh_stat @ 0xdcd0
int optionh_stat(char *file, struct stat *buf)
{
  int v2; // r13d
  int v4; // eax

  if ( dword_385F8 < 0 && dword_385F8 != -100 )
    __assert_fail("state.cwd_dir_fd >= 0", "util.c", 0x25Du, "optionh_stat");
  if ( !dword_385E0 )
  {
    v2 = fstatat(dword_385F8, file, buf, 0);
    if ( !v2 )
      return v2;
    v4 = *__errno_location();
    if ( v4 != 2 && v4 != 20 )
      return v2;
    if ( (qword_38660 & 2) != 0 )
      __fprintf_chk(stderr, 1, "fallback_stat(): stat(%s) failed; falling back on lstat()\n", file);
  }
  return fstatat(dword_385F8, file, buf, 256);
}



// Function: debug_stat @ 0xdec0
int debug_stat(char *file, struct stat *buf)
{
  ++qword_386E0;
  __fprintf_chk(stderr, 1, "debug_stat (%s)\n", file);
  if ( dword_38668 == 1 )
    return sub_DDB0(file, buf);
  if ( dword_38668 == 2 )
    return sub_DCD0(file, buf);
  if ( dword_38668 )
    __assert_fail("0", "util.c", 0x2A5u, "debug_stat");
  if ( dword_385F8 < 0 && dword_385F8 != -100 )
    __assert_fail("(state.cwd_dir_fd >= 0) || (state.cwd_dir_fd==AT_FDCWD)", "util.c", 0x28Du, "optionp_stat");
  return fstatat(dword_385F8, file, buf, 256);
}



// Function: digest_mode @ 0xdfe0
long long digest_mode(int *a1, long long a2, long long a3, long long a4, unsigned int a5)
{
  int v6; // eax
  int v8; // eax

  v6 = *a1;
  if ( *a1 )
  {
    if ( (*a1 & 0xF000) != 0xA000 || dword_38668 != 1 && (dword_38668 != 2 || dword_385E0) )
    {
      HIBYTE(word_385E4) = 1;
      dword_385E8 = v6;
      *(int *)(a4 + 24) = v6;
      return 1;
    }
    if ( !(unsigned int)sub_D820(a2, a3, a4) )
    {
      dword_385E8 = *(int *)(a4 + 24);
      *a1 = dword_385E8;
      HIBYTE(word_385E4) = 1;
      return 1;
    }
    return 0;
  }
  if ( (char)a5 )
  {
    dword_385E8 = 0;
    word_385E4 = 0;
    return a5;
  }
  if ( (unsigned int)sub_D820(a2, a3, a4) )
    return 0;
  v8 = *(int *)(a4 + 24);
  *a1 = v8;
  dword_385E8 = v8;
  HIBYTE(word_385E4) = 1;
  return 1;
}



// Function: default_prints @ 0xe0e0
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



// Function: looks_like_expression @ 0xe120
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



// Function: process_leading_options @ 0xe180
long long process_leading_options(int a1, long long a2)
{
  int v2; // r12d
  long long v3; // rbp
  const char *v4; // rbx
  int v5; // r12d
  char *v6; // rax
  char *v7; // r12
  const char *v8; // rdi
  long long v9; // rbx
  const char **v10; // r14
  long long v11; // r12
  char *v12; // rax
  const char *v14; // r13
  long long v15; // rbx
  int *v16; // rax
  int v17; // ebx
  int *v18; // rbp
  unsigned long long v19; // rax
  long long v20; // r14
  char *v21; // rax
  const char *v22; // rsi
  char *v23; // rax
  char *v24; // rax
  char *v25; // rax
  char *v26; // rax
  char *v27; // rax
  char *v28; // rax
  FILE *v29; // rbp
  char *v30; // rax
  char **v31; // r12
  const char *v32; // rcx
  const char *v33; // r8
  char *v34; // [rsp+10h] [rbp-68h]
  int v36; // [rsp+1Ch] [rbp-5Ch]
  char *save_ptr; // [rsp+28h] [rbp-50h] BYREF
  char delim[2]; // [rsp+36h] [rbp-42h] BYREF
  unsigned long long i; // [rsp+38h] [rbp-40h]

  v2 = 1;
  for ( i = __readfsqword(0x28u); a1 > v2; ++v2 )
  {
    v3 = 8LL * v2;
    v4 = *(const char **)(a2 + v3);
    if ( !strcmp("-H", v4) )
    {
      sub_14AF0(2);
    }
    else if ( !strcmp("-L", v4) )
    {
      sub_14AF0(1);
    }
    else if ( !strcmp("-P", v4) )
    {
      sub_14AF0(0);
    }
    else
    {
      if ( !strcmp("--", v4) )
        return (unsigned int)++v2;
      if ( !strcmp("-D", v4) )
      {
        v5 = v2 + 1;
        if ( v5 >= a1 )
        {
          v22 = "Missing argument after the -D option.";
LABEL_37:
          v23 = dcgettext(0, v22, 5);
          error(0, 0, v23);
          sub_D570(1);
        }
        save_ptr = 0;
        strcpy(delim, ",");
        v34 = *(char **)(a2 + v3 + 8);
        v6 = strtok_r(v34, delim, &save_ptr);
        if ( !v6 )
        {
          v22 = "Empty argument to the -D option.";
          goto LABEL_37;
        }
        v36 = v5;
        v7 = v6;
        do
        {
          v8 = "exec";
          v9 = 0;
          v10 = (const char **)&off_36198;
          while ( strcmp(v8, v7) )
          {
            if ( ++v9 == 9 )
            {
              v11 = sub_239F0(0, (unsigned int)dword_3868C, v34);
              v12 = dcgettext(0, "Ignoring unrecognised debug flag %s", 5);
              error(0, 0, v12, v11);
              goto LABEL_15;
            }
            v8 = *v10;
            v10 += 3;
          }
          qword_38660 = *((int *)&off_36180 + 6 * v9 + 2) | (unsigned long long)qword_38660;
LABEL_15:
          v7 = strtok_r(0, delim, &save_ptr);
        }
        while ( v7 );
        v2 = v36;
        if ( (qword_38660 & 0x10) != 0 )
        {
          v29 = stdout;
          v30 = dcgettext(0, "Valid arguments for -D:\n", 5);
          v31 = &off_36198;
          fputs(v30, v29);
          v32 = "exec";
          v33 = "Show diagnostic information relating to -exec, -execdir, -ok and -okdir";
          while ( 1 )
          {
            __fprintf_chk(stdout, 1, "%-10s %s\n", v32, v33);
            if ( &unk_36258 == (_UNKNOWN *)v31 )
              break;
            v33 = v31[2];
            v32 = *v31;
            v31 += 3;
          }
          exit(0);
        }
      }
      else
      {
        if ( strncmp("-O", v4, 2u) )
          return (unsigned int)v2;
        v14 = v4 + 2;
        v15 = *((unsigned char *)v4 + 2);
        if ( !(char)v15 )
          goto LABEL_44;
        if ( ((*__ctype_b_loc())[v15] & 0x800) == 0 )
          goto LABEL_43;
        v16 = __errno_location();
        v17 = *v16;
        *v16 = 0;
        v18 = v16;
        v19 = strtoul(v14, &save_ptr, 10);
        if ( v19 )
        {
          if ( *save_ptr )
            goto LABEL_41;
          if ( v19 == -1 )
          {
            v20 = -1;
            if ( !*v18 )
              goto LABEL_33;
            v24 = dcgettext(0, "Invalid optimisation level %s", 5);
            error(1, *v18, v24, v14);
LABEL_41:
            v25 = dcgettext(0, "Invalid optimisation level %s", 5);
            error(1, 0, v25, v14);
LABEL_42:
            v26 = dcgettext(0, "Please specify a decimal number immediately after -O", 5);
            error(1, 0, v26);
LABEL_43:
            v27 = dcgettext(0, "Please specify a decimal number immediately after -O", 5);
            error(1, 0, v27);
LABEL_44:
            v28 = dcgettext(0, "The -O option must be immediately followed by a decimal integer", 5);
            error(1, 0, v28);
          }
          while ( v19 > 0xFFFF )
          {
            v20 = v19;
LABEL_33:
            v21 = dcgettext(
                    0,
                    "Optimisation level %lu is too high.  If you want to find files very quickly, consider using GNU locate.",
                    5);
            error(1, 0, v21, v20);
          }
        }
        else
        {
          if ( v14 == save_ptr )
            goto LABEL_42;
          if ( *save_ptr )
            goto LABEL_41;
        }
        word_38688 = v19;
        *v18 = v17;
      }
    }
  }
  return (unsigned int)v2;
}



// Function: set_option_defaults @ 0xe630
unsigned long long set_option_defaults(long long a1)
{
  bool v1; // al
  int v2; // eax
  __suseconds_t v3; // rdx
  time_t tv_sec; // rax
  bool v5; // cf
  char *v7; // rax
  struct timeval v8; // [rsp+0h] [rbp-28h] BYREF
  unsigned long long v9; // [rsp+18h] [rbp-10h]

  v9 = __readfsqword(0x28u);
  *(char *)(a1 + 17) = getenv("POSIXLY_CORRECT") != 0;
  v1 = sub_D920();
  *(int *)(a1 + 92) = 0;
  *(char *)(a1 + 88) = v1;
  v2 = isatty(0);
  *(char *)(a1 + 15) = 0;
  *(char *)(a1 + 16) = v2 != 0;
  if ( *(char *)(a1 + 17) )
    *(char *)(a1 + 16) = 0;
  *(long long *)(a1 + 4) = -1;
  *(short *)a1 = 1;
  if ( gettimeofday(&v8, 0) )
  {
    tv_sec = time(0);
    if ( tv_sec == -1 )
      goto LABEL_10;
    v3 = 0;
  }
  else
  {
    v3 = 1000 * v8.tv_usec;
    tv_sec = v8.tv_sec;
  }
  *(long long *)(a1 + 24) = tv_sec;
  *(long long *)(a1 + 40) = tv_sec - 86400;
  v5 = *(char *)(a1 + 17) == 0;
  *(short *)(a1 + 13) = 0;
  *(long long *)(a1 + 32) = v3;
  *(long long *)(a1 + 48) = v3;
  *(char *)(a1 + 56) = 0;
  *(int *)(a1 + 60) = v5 ? 1024 : 512;
  *(long long *)(a1 + 64) = 0;
  *(short *)(a1 + 104) = 2;
  if ( getenv("FIND_BLOCK_SIZE") )
  {
    v7 = dcgettext(
           0,
           "The environment variable FIND_BLOCK_SIZE is not supported, the only thing that affects the block size is the "
           "POSIXLY_CORRECT environment variable",
           5);
    error(1, 0, v7);
LABEL_10:
    __assert_fail("t != (time_t)-1", "util.c", 0x3D2u, "now");
  }
  *(char *)(a1 + 12) = 0;
  sub_14AF0(0);
  *(int *)(a1 + 108) = 8;
  *(long long *)(a1 + 112) = 0;
  *(char *)(a1 + 120) = 0;
  return v9 - __readfsqword(0x28u);
}



// Function: apply_predicate @ 0xe7b0
long long apply_predicate(long long a1, long long a2, long long a3)
{
  long long result; // rax

  ++*(long long *)(a3 + 288);
  if ( (*(long long *)(a3 + 24) & 0xFFFFFF0000LL) != 0
    && (*(char *)(a3 + 26) && !(char)word_385E4
     || *(char *)(a3 + 27) && !HIBYTE(word_385E4)
     || *(char *)(a3 + 28)
     && (!*(long long *)(a2 + 8) || !HIBYTE(word_385E4) || (*(int *)(a2 + 24) & 0xF000) == 0x4000))
    && (unsigned int)sub_D820(a1, (long long)src, a2) )
  {
    return 0;
  }
  result = (*(long long (**)(long long, long long, long long))a3)(a1, a2, a3);
  if ( !(char)result )
    return 0;
  ++*(long long *)(a3 + 296);
  return result;
}



// Function: is_exec_in_local_dir @ 0xe880
long long is_exec_in_local_dir(long long (*a1)(long long a1, long long a2, long long a3), long long a2, int a3)
{
  unsigned int v3; // eax

  LOBYTE(v3) = a1 == sub_95D0;
  LOBYTE(a3) = (char *)a1 == (char *)sub_8FC0;
  return a3 | v3;
}



// Function: fatal_target_file_error @ 0xe920
void fatal_target_file_error(int errnum, long long a2)
{
  const char *v2; // rax

  if ( !byte_38618 )
  {
    v2 = (const char *)sub_239F0(0, (unsigned int)dword_3868C, a2);
    error(1, errnum, "%s", v2);
    status = 1;
  }
  byte_38618 = 1;
  abort();
}



// Function: fatal_nontarget_file_error @ 0xe9b0
void fatal_nontarget_file_error(int errnum, long long a2)
{
  const char *v2; // rax

  byte_38618 = 0;
  v2 = (const char *)sub_239F0(0, (unsigned int)dword_3868C, a2);
  error(1, errnum, "%s", v2);
  status = 1;
  abort();
}



// Function: entry_comparator @ 0xea00
bool entry_comparator(long long *a1, long long *a2)
{
  bool result; // al

  result = 0;
  if ( a1[1] == a2[1] )
    return *a1 == *a2;
  return result;
}



// Function: entry_hashfunc @ 0xea30
unsigned long long entry_hashfunc(long long *a1, unsigned long long a2)
{
  return (a1[1] ^ *a1) % a2;
}



// Function: sharefile_init @ 0xea90
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
      v3 = sub_1B2F0(11, 0, sub_EA30, sub_EA00, sub_EA50);
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



// Function: sharefile_destroy @ 0xeb20
long long sharefile_destroy(void **a1)
{
  free(*a1);
  return sub_1B4B0(a1[1]);
}



// Function: sharefile_fopen @ 0xeb40
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
    v7 = (FILE *)sub_182A0(a2, *a1);
    v4[3] = (__dev_t)v7;
    v6 = (long long)v7;
    if ( v7 )
    {
      v8 = fileno(v7);
      v9 = v8;
      if ( v8 < 0 )
        __assert_fail("fd >= 0", "sharefile.c", 0xA6u, "sharefile_fopen");
      sub_17990((unsigned int)v8, 1);
      if ( fstat(v9, &v16) < 0 )
      {
        v6 = 0;
        sub_EA50(v4);
      }
      else
      {
        v10 = a1[1];
        *v4 = v16.st_dev;
        v4[1] = v16.st_ino;
        v11 = sub_1B060(v10, v4);
        if ( v11 )
        {
          sub_EA50(v4);
          return *(long long *)(v11 + 24);
        }
        else
        {
          v6 = sub_1B950(a1[1], v4);
          if ( v6 )
          {
            return v4[3];
          }
          else
          {
            v13 = __errno_location();
            v14 = *v13;
            v15 = v13;
            sub_EA50(v4);
            *v15 = v14;
          }
        }
      }
    }
    else
    {
      sub_EA50(v4);
    }
  }
  else
  {
    free(v4);
  }
  return v6;
}



// Function: checked_fprintf @ 0xece0
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
    sub_E970(*v3);
  }
  return v5 - __readfsqword(0x28u);
}



// Function: format_date @ 0xedc0
char *format_date(time_t a1, long long a2, int a3)
{
  long long v3; // r9
  unsigned long long v4; // rax
  long long v6; // rdx
  int v7; // r13d
  int v8; // ebp
  time_t v9; // rdi
  char *v10; // r12
  size_t v11; // rax
  size_t v12; // rbx
  struct tm *v14; // rbx
  size_t v15; // rax
  char *v16; // rax
  char *v17; // rbp
  size_t v18; // rax
  char *v19; // r12
  int v20; // eax
  size_t v21; // rax
  size_t v22; // r15
  char *v23; // rax
  char *v24; // r12
  unsigned char v25; // r13
  long long v26; // r14
  unsigned char v27; // bl
  const unsigned short *v28; // rax
  char *v29; // r13
  unsigned long long v30; // rdi
  char *v31; // r13
  unsigned int v32; // edx
  long long v33; // rsi
  char *v34; // [rsp+0h] [rbp-F8h]
  long long v35; // [rsp+8h] [rbp-F0h]
  char v36; // [rsp+17h] [rbp-E1h]
  long long v37; // [rsp+18h] [rbp-E0h]
  long long v38; // [rsp+20h] [rbp-D8h]
  long long v39; // [rsp+28h] [rbp-D0h]
  time_t timer; // [rsp+30h] [rbp-C8h] BYREF
  long long v41; // [rsp+38h] [rbp-C0h]
  struct tm tp; // [rsp+40h] [rbp-B8h] BYREF
  char src[12]; // [rsp+84h] [rbp-74h] BYREF
  char s[40]; // [rsp+90h] [rbp-68h] BYREF
  unsigned long long v45; // [rsp+B8h] [rbp-40h]

  v3 = a2;
  timer = a1;
  v41 = a2;
  v45 = __readfsqword(0x28u);
  if ( a3 == 43 )
  {
    strcpy(src, "%Y-%m-%d+%T");
    v7 = 1;
    v8 = __snprintf_chk(s, 32, 1, 32, ".%09ld0", a2);
  }
  else
  {
    v4 = (unsigned int)(a3 - 64);
    src[0] = 37;
    src[1] = a3;
    src[2] = 0;
    if ( (unsigned int)v4 <= 0x18 && (v6 = 18350081, _bittest64(&v6, v4)) )
    {
      v7 = 1;
      v8 = __snprintf_chk(s, 32, 1, 32, ".%09ld0", a2);
      if ( a3 == 64 )
        goto LABEL_5;
    }
    else
    {
      s[0] = 0;
      v7 = 0;
      v8 = 0;
      if ( a3 == 64 )
      {
LABEL_5:
        v9 = -timer;
        if ( timer > 0 )
          v9 = timer;
        v10 = (char *)sub_1BB50(v9, &unk_38B21, 0, 1, 1, v3);
        if ( v10 <= (char *)&unk_38B20 )
          __assert_fail("p > buf", "print.c", 0x2B9u, "format_date");
        if ( v10 >= (char *)&unk_38E37 )
          __assert_fail("p < (buf + (sizeof buf))", "print.c", 0x2BAu, "format_date");
        if ( timer < 0 )
          *--v10 = 45;
        if ( v7 )
        {
          v11 = strlen(v10) + v10 - (char *)&unk_38B20;
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
    }
  }
  v14 = localtime(&timer);
  if ( !v14 )
    goto LABEL_5;
  v38 = v8;
  v15 = strlen(src);
  v16 = (char *)sub_25A50(v15 + 2);
  *v16 = 95;
  v17 = v16;
  v18 = strlen(src);
  memcpy(v17 + 1, src, v18 + 1);
  v19 = qword_38B08;
  *(__m128i *)&tp.tm_sec = _mm_loadu_si128((const __m128i *)v14);
  *(__m128i *)&tp.tm_mon = _mm_loadu_si128((const __m128i *)&v14->tm_mon);
  *(__m128i *)&tp.tm_isdst = _mm_loadu_si128((const __m128i *)&v14->tm_isdst);
  tp.tm_zone = v14->tm_zone;
  v20 = tp.tm_sec + 11;
  if ( tp.tm_sec >= 11 )
    v20 = tp.tm_sec - 11;
  tp.tm_sec = v20;
  if ( !qword_38B08 )
  {
    maxsize = 1;
    qword_38B08 = (char *)sub_25A50(1);
    v19 = qword_38B08;
  }
  while ( 1 )
  {
    v21 = strftime(v19, maxsize, v17, v14);
    v22 = v21;
    if ( v21 )
    {
      if ( v21 < maxsize )
        break;
    }
    qword_38B08 = (char *)sub_25CA0(v19, &maxsize, 1);
    v19 = qword_38B08;
  }
  v39 = v38 + v21 + 1;
  v23 = (char *)sub_25AA0(v19, v39);
  maxsize = v38 + v22 + 1;
  qword_38B08 = v23;
  v24 = (char *)sub_25A50(maxsize);
  strftime(v24, maxsize, v17, &tp);
  v25 = *qword_38B08;
  v34 = qword_38B08;
  if ( *qword_38B08 )
  {
    v35 = 0;
    v26 = 0;
    v37 = 0;
    v36 = 0;
    while ( 1 )
    {
      v27 = v24[v26];
      if ( !v27 )
        break;
      if ( v27 != v25 )
      {
        v28 = *__ctype_b_loc();
        if ( (v28[v25] & 0x800) == 0 || (v28[v27] & 0x800) == 0 )
          break;
        if ( v36 )
        {
          if ( v35 != v26 - v37 )
            break;
          ++v35;
        }
        else
        {
          v37 = v26;
          v35 = 1;
          v36 = 1;
        }
      }
      v25 = v34[++v26];
      if ( !v25 )
      {
        if ( !v24[v26] && v35 == 2 )
        {
          v29 = &v34[v37 + 2];
          if ( ((*__ctype_b_loc())[(unsigned char)*v29] & 0x800) == 0 )
          {
            if ( v39 != v38 + v22 + 1 )
              __assert_fail("end_of_seconds + ns_size + suffix_len == final_len", "print.c", 0x225u, "do_time_format");
            memmove(&v34[v38 + 2 + v37], &v34[v37 + 2], v22 + 1 - (v37 + 2));
            if ( (unsigned int)v38 >= 8 )
            {
              v30 = (unsigned long long)(v29 + 8) & 0xFFFFFFFFFFFFFFF8LL;
              *(long long *)v29 = *(long long *)s;
              *(long long *)&v29[(unsigned int)v38 - 8] = *(long long *)&src[(unsigned int)v38 + 4];
              v31 = &v29[-v30];
              if ( (((int)v31 + (int)v38) & 0xFFFFFFF8) >= 8 )
              {
                v32 = 0;
                do
                {
                  v33 = v32;
                  v32 += 8;
                  *(long long *)(v30 + v33) = *(long long *)(s - v31 + v33);
                }
                while ( v32 < (((int)v31 + (int)v38) & 0xFFFFFFF8) );
              }
            }
            else if ( (v38 & 4) != 0 )
            {
              *(int *)v29 = *(int *)s;
              *(int *)&v29[(unsigned int)v38 - 4] = *(int *)&src[(unsigned int)v38 + 8];
            }
            else if ( (int)v38 )
            {
              *v29 = s[0];
              if ( (v38 & 2) != 0 )
                *(short *)&v34[v37 + (unsigned int)v38] = *(short *)&src[(unsigned int)v38 + 10];
            }
          }
        }
        break;
      }
    }
  }
  free(v17);
  free(v24);
  return v34 + 1;
}



// Function: ctime_format @ 0xf400
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
    return sub_EDC0(v8, v9, 64);
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
              &unk_38700,
              1024,
              1,
              1024,
              "%3s %3s %2d %02d:%02d:%02d.%09ld0 %04d",
              off_362C0[tm_wday],
              off_36260[tm_mon],
              v2->tm_mday,
              tm_hour,
              tm_min,
              v2->tm_sec,
              v9,
              v2->tm_year + 1900) > 1023 )
    __assert_fail("nout < TIME_BUF_LEN", "print.c", 0x302u, "ctime_format");
  return (char *)&unk_38700;
}



// Function: do_fprintf @ 0xf620
void do_fprintf(long long a1, const void **a2, char *a3, long long *a4, long long a5, long long a6)
{
  int v7; // eax
  size_t v8; // rbp
  int *v9; // rax
  size_t v12; // rbp
  int *v13; // rax
  long long v14; // rdi
  long long v15; // r8
  long long v16; // rcx
  char *v17; // rdx
  long long *v18; // rax
  char *v19; // r8
  long long v20; // rax
  long long v21; // rdx
  double v22; // xmm0_8
  long long v23; // rsi
  char *v24; // rax
  long long v25; // r14
  char *v26; // rbp
  char *v27; // rdx
  int v28; // eax
  char *v29; // rax
  char *v30; // rax
  int *v31; // rax
  int v32; // eax
  long long v33; // rsi
  char *v34; // rax
  long long v35; // r13
  char *v36; // r12
  int *v37; // rax
  int *v38; // rax
  int *v39; // rax
  long long v40; // rsi
  int *v41; // rbp
  int v42; // eax
  const char *v43; // rax
  int v44; // eax
  long long v45; // [rsp+8h] [rbp-360h] BYREF
  struct stat buf; // [rsp+10h] [rbp-358h] BYREF
  char v47[664]; // [rsp+A0h] [rbp-2C8h] BYREF
  unsigned long long v48; // [rsp+338h] [rbp-30h]

  v48 = __readfsqword(0x28u);
  v7 = *(int *)a2;
  if ( *(int *)a2 == 1 )
  {
    v12 = *((int *)a2 + 4);
    if ( v12 > fwrite(a2[1], 1u, v12, *(FILE **)(a1 + 8)) )
    {
      v13 = __errno_location();
      sub_E970(*v13);
    }
    if ( (unsigned int)sub_180C0(*(FILE **)(a1 + 8)) )
      goto LABEL_6;
    return;
  }
  if ( v7 == 2 )
  {
    switch ( *((char *)a2 + 4) )
    {
      case 0:
        v34 = dcgettext(0, "error: %s at end of format string", 5);
        v33 = 0;
        error(1, 0, v34, "%");
        goto LABEL_77;
      case 0x25:
        goto LABEL_5;
      case 0x44:
        v14 = *a4;
        v15 = 1;
        v16 = 1;
        goto LABEL_13;
      case 0x46:
        v19 = (char *)sub_108E0(a4, a3);
        goto LABEL_18;
      case 0x47:
        goto LABEL_16;
      case 0x48:
        v24 = (char *)sub_25A50(dword_385FC + 1);
        v25 = dword_385FC;
        v26 = v24;
        memcpy(v24, a3, dword_385FC);
        v26[v25] = 0;
        sub_ECE0(a1, (long long)a2[1], v26);
        goto LABEL_30;
      case 0x4D:
        sub_18290(a4, v47);
        v23 = (long long)a2[1];
        v47[10] = 0;
        sub_ECE0(a1, v23, v47);
        return;
      case 0x50:
        v19 = "";
        if ( dword_385E0 > 0 )
          v19 = &a3[dword_385FC + (a3[dword_385FC] == 47)];
LABEL_18:
        if ( (int)sub_16E80(*(long long *)(a1 + 8), *(long long *)(a1 + 32), *(unsigned char *)(a1 + 24), a2[1], v19) < 0 )
          goto LABEL_6;
        return;
      case 0x53:
        v20 = a4[6];
        v21 = a4[8];
        if ( v20 )
        {
          v22 = (double)(int)v21 * 512.0 / (double)(int)v20;
        }
        else
        {
          v22 = 1.0;
          if ( v21 )
          {
            v22 = -INFINITY;
            if ( v21 >= 0 )
              v22 = INFINITY;
          }
        }
        sub_ECE0(a1, (long long)a2[1], v22);
        return;
      case 0x55:
        goto LABEL_12;
      case 0x59:
        v28 = a4[3] & 0xF000;
        if ( v28 != 40960 )
        {
          v27 = "f";
          if ( v28 != 0x8000 )
          {
            v27 = (char *)"d";
            if ( v28 != 0x4000 )
            {
LABEL_34:
              v27 = "s";
              if ( v28 != 49152 )
              {
                v27 = "b";
                if ( v28 != 24576 )
                {
                  v27 = (char *)"c";
                  if ( v28 != 0x2000 )
                  {
                    v27 = "U";
                    if ( v28 == 4096 )
                      v27 = (char *)"p";
                  }
                }
              }
            }
          }
LABEL_39:
          sub_ECE0(a1, (long long)a2[1], v27);
          return;
        }
        if ( fstatat(dword_385F8, src, &buf, 0) )
        {
          v39 = __errno_location();
          v40 = (long long)a2[1];
          v41 = v39;
          v42 = *v39;
          if ( v42 == 2 || v42 == 20 )
          {
            sub_ECE0(a1, v40, "N");
          }
          else if ( v42 == 40 )
          {
            sub_ECE0(a1, v40, "L");
          }
          else
          {
            sub_ECE0(a1, v40, "?");
            v43 = (const char *)sub_E8A0(0, a3);
            error(0, *v41, "%s", v43);
          }
          return;
        }
        v17 = "f";
        v44 = buf.st_mode & 0xF000;
        if ( v44 != 0x8000 )
        {
          v17 = (char *)"d";
          if ( v44 != 0x4000 )
          {
            v17 = (char *)"l";
            if ( v44 != 40960 )
            {
              v17 = "s";
              if ( v44 != 49152 )
              {
                v17 = "b";
                if ( v44 != 24576 )
                {
                  v17 = (char *)"c";
                  if ( v44 != 0x2000 )
                  {
                    v17 = "U";
                    if ( v44 == 4096 )
                      v17 = (char *)"p";
                  }
                }
              }
            }
          }
        }
        goto LABEL_14;
      case 0x5A:
        v32 = qword_38680((unsigned int)dword_385F8, src, &v45);
        v33 = (long long)a2[1];
        if ( v32 < 0 )
        {
LABEL_77:
          sub_ECE0(a1, v33, "");
          v35 = sub_E8A0(0, a3);
          v36 = dcgettext(0, "getfilecon failed: %s", 5);
          v37 = __errno_location();
          error(0, *v37, v36, v35);
          status = 1;
        }
        else
        {
          sub_ECE0(a1, v33, v45);
          freecon(v45);
        }
        return;
      case 0x61:
        v27 = sub_F400(a4[9], a4[10]);
        goto LABEL_39;
      case 0x62:
        v14 = a4[8];
        v15 = 512;
        v16 = 512;
        goto LABEL_13;
      case 0x63:
        v27 = sub_F400(a4[13], a4[14]);
        goto LABEL_39;
      case 0x64:
        sub_ECE0(a1, (long long)a2[1], (unsigned int)dword_385E0);
        return;
      case 0x66:
        v26 = (char *)sub_17B90(a3);
        goto LABEL_63;
      case 0x67:
        v18 = getgrgid(*((int *)a4 + 8));
        if ( v18 )
          goto LABEL_41;
LABEL_16:
        v14 = *((unsigned int *)a4 + 8);
        v15 = 1;
        v16 = 1;
        goto LABEL_13;
      case 0x68:
        v26 = (char *)sub_25F30(a3);
        v29 = &v26[strlen(v26) - 1];
        if ( v26 > v29 )
          goto LABEL_61;
        break;
      case 0x69:
        v14 = a4[1];
        v15 = 1;
        v16 = 1;
        goto LABEL_13;
      case 0x6B:
        v14 = a4[8];
        v15 = 1024;
        v16 = 512;
        goto LABEL_13;
      case 0x6C:
        if ( (a4[3] & 0xF000) != 0xA000 )
          goto LABEL_50;
        v26 = (char *)sub_17650((unsigned int)dword_385F8, src);
        if ( v26 )
          goto LABEL_63;
        v38 = __errno_location();
        sub_E8C0(*v38);
        status = 1;
LABEL_50:
        v26 = 0;
        if ( (int)sub_16E80(*(long long *)(a1 + 8), *(long long *)(a1 + 32), *(unsigned char *)(a1 + 24), a2[1], "") >= 0 )
          goto LABEL_30;
        goto LABEL_64;
      case 0x6D:
        sub_ECE0(a1, (long long)a2[1], a4[3] & 0xFFF);
        return;
      case 0x6E:
        v14 = a4[2];
        v15 = 1;
        v16 = 1;
        goto LABEL_13;
      case 0x70:
        if ( (int)sub_16E80(*(long long *)(a1 + 8), *(long long *)(a1 + 32), *(unsigned char *)(a1 + 24), a2[1], a3) < 0 )
          goto LABEL_6;
        return;
      case 0x73:
        v14 = a4[6];
        v15 = 1;
        v16 = 1;
        goto LABEL_13;
      case 0x74:
        v27 = sub_F400(a4[11], a4[12]);
        goto LABEL_39;
      case 0x75:
        v18 = getpwuid(*((int *)a4 + 7));
        if ( v18 )
        {
LABEL_41:
          *((char *)a2[1] + *((int *)a2 + 4)) = 115;
          sub_ECE0(a1, (long long)a2[1], *v18);
        }
        else
        {
LABEL_12:
          v14 = *((unsigned int *)a4 + 7);
          v15 = 1;
          v16 = 1;
LABEL_13:
          v17 = (char *)sub_1BB50(v14, v47, 0, v16, v15, a6);
LABEL_14:
          sub_ECE0(a1, (long long)a2[1], v17);
        }
        return;
      case 0x79:
        v27 = "f";
        v28 = a4[3] & 0xF000;
        if ( v28 != 0x8000 )
        {
          v27 = (char *)"d";
          if ( v28 != 0x4000 )
          {
            v27 = (char *)"l";
            if ( v28 != 40960 )
              goto LABEL_34;
          }
        }
        goto LABEL_39;
      default:
        return;
    }
    while ( *v29 == 47 )
    {
      if ( v26 > --v29 )
        goto LABEL_61;
    }
    if ( v26 < v29 && v29[1] == 47 )
      v29[1] = 0;
LABEL_61:
    v30 = strrchr(v26, 47);
    if ( v30 )
    {
      *v30 = 0;
LABEL_63:
      if ( (int)sub_16E80(*(long long *)(a1 + 8), *(long long *)(a1 + 32), *(unsigned char *)(a1 + 24), a2[1], v26) >= 0 )
        goto LABEL_30;
    }
    else if ( (int)sub_16E80(*(long long *)(a1 + 8), *(long long *)(a1 + 32), *(unsigned char *)(a1 + 24), a2[1], ".") >= 0 )
    {
      goto LABEL_30;
    }
LABEL_64:
    v31 = __errno_location();
    sub_E970(*v31);
LABEL_30:
    free(v26);
    return;
  }
  if ( v7 )
    return;
LABEL_5:
  v8 = *((int *)a2 + 4);
  if ( v8 > fwrite(a2[1], 1u, v8, *(FILE **)(a1 + 8)) )
  {
LABEL_6:
    v9 = __errno_location();
    sub_E970(*v9);
  }
}



// Function: pred_fprintf @ 0xff90
long long pred_fprintf(char *a1, long long *a2, long long a3, long long a4, long long a5, long long a6)
{
  long long v6; // rbx
  long long v9; // r12
  int v10; // edx
  char v11; // al
  long long v12; // rsi
  time_t v13; // rdi
  char *v14; // rax

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
          if ( *(char *)(v6 + 5) )
            break;
        }
        sub_F620(v9, (const void **)v6, a1, a2, a5, a6);
        v6 = *(long long *)(v6 + 24);
        if ( !v6 )
          return 1;
      }
      v11 = *(char *)(v6 + 4);
      if ( v11 == 67 )
      {
        v12 = a2[14];
        v13 = a2[13];
      }
      else if ( v11 > 67 )
      {
        if ( v11 != 84 )
LABEL_19:
          __assert_fail("0", "print.c", 0x518u, "pred_fprintf");
        v12 = a2[12];
        v13 = a2[11];
      }
      else if ( v11 == 65 )
      {
        v12 = a2[10];
        v13 = a2[9];
      }
      else
      {
        if ( v11 != 66 )
          goto LABEL_19;
        v13 = -1;
        v12 = -1;
        if ( (char)v10 != 64 )
        {
          sub_ECE0(v9, *(long long *)(v6 + 8), "");
          goto LABEL_9;
        }
      }
      v14 = sub_EDC0(v13, v12, v10);
      sub_ECE0(v9, *(long long *)(v6 + 8), v14);
LABEL_9:
      v6 = *(long long *)(v6 + 24);
    }
    while ( v6 );
  }
  return 1;
}



// Function: make_segment @ 0x100b0
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
  v11 = sub_25A50(32);
  *a1 = v11;
  v12 = v11;
  *(int *)v11 = a4;
  *(char *)(v11 + 4) = a5;
  *(char *)(v11 + 5) = a6;
  *(long long *)(v11 + 24) = 0;
  *(int *)(v11 + 16) = v8;
  v13 = (char *)sub_25A50(v8 + 2);
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



// Function: insert_fprintf @ 0x10380
long long insert_fprintf(const __m128i *a1, long long a2, char *a3)
{
  unsigned char *v3; // r15
  long long v5; // rax
  unsigned char v6; // al
  const char *v7; // r13
  unsigned char *v8; // r14
  long long v9; // rbp
  char v10; // al
  char *v11; // r14
  const char *v12; // rsi
  int v13; // edx
  long long i; // r15
  const unsigned short *v16; // rcx
  const char *v17; // r12
  const char *v18; // rsi
  char *v19; // rax
  const char *v20; // rsi
  int v21; // edx
  char *v22; // rax
  char v23; // r9
  const char *v24; // rsi
  int v25; // edx
  char *v26; // rax
  char *v27; // rax
  long long v28; // rsi
  char v29; // dl
  unsigned char *v30; // rax
  long long v31; // rdx
  char *v32; // rax
  char *v33; // rax
  long long *v34; // [rsp+0h] [rbp-48h]
  long long v35; // [rsp+8h] [rbp-40h]

  v35 = sub_D490(a2, sub_FF90);
  *(__m128i *)(v35 + 56) = _mm_loadu_si128(a1);
  *(__m128i *)(v35 + 72) = _mm_loadu_si128(a1 + 1);
  v5 = a1[2].m128i_i64[0];
  *(int *)(v35 + 24) = 257;
  *(long long *)(v35 + 88) = v5;
  *(int *)(v35 + 32) = 0;
  *(long long *)(v35 + 56) = 0;
  v6 = *a3;
  v34 = (long long *)(v35 + 56);
  if ( *a3 )
  {
    v7 = a3;
    while ( 1 )
    {
      while ( 1 )
      {
        LOBYTE(v9) = a3[1];
        if ( v6 != 92 )
          break;
        if ( (char)v9 == 99 )
        {
          sub_100B0(v34, v7, (int)a3 - (int)v7, 1u, 0, 0, v35);
          if ( *(char *)(v35 + 26) && *(int *)(v35 + 32) <= 2u )
            *(int *)(v35 + 32) = 3;
          return 1;
        }
        v3 = (unsigned char *)(a3 + 1);
        if ( !(char)v9 )
        {
          v11 = a3;
          v22 = dcgettext(0, "warning: escape `\\' followed by nothing at all", 5);
          error(0, 0, v22);
          goto LABEL_13;
        }
        if ( (unsigned char)(v9 - 48) <= 7u )
        {
          v27 = a3 + 1;
          v28 = 0;
          v29 = 0;
          while ( 1 )
          {
            v11 = v27;
            if ( (unsigned char)(v9 - 48) > 7u )
              break;
            ++v27;
            v29 = v9 + 8 * v29 - 48;
            v3 = (unsigned char *)v27;
            if ( v28 == 2 )
              goto LABEL_48;
            LOBYTE(v9) = *v27;
            ++v28;
          }
          v3 = (unsigned char *)v27;
          v11 = &a3[v28];
LABEL_48:
          *a3 = v29;
          goto LABEL_13;
        }
        if ( (unsigned char)(v9 - 92) > 0x1Au || (v10 = byte_2AA00[(unsigned char)(v9 - 92)]) == 0 )
        {
          a3 += 2;
          v8 = v3;
          v26 = dcgettext(0, "warning: unrecognized escape `\\%c'", 5);
          error(0, 0, v26, (unsigned int)(char)v9);
          goto LABEL_5;
        }
        *a3 = v10;
        v11 = a3 + 1;
        v3 = (unsigned char *)(a3 + 2);
LABEL_13:
        v12 = v7;
        v13 = (int)a3 - (int)v7 + 1;
        a3 = (char *)v3;
        v7 = (const char *)v3;
        v34 = (long long *)sub_100B0(v34, v12, v13, 0, 0, 0, v35);
        v6 = v11[1];
        if ( !v6 )
        {
LABEL_14:
          if ( v7 < a3 )
            sub_100B0(v34, v7, (int)a3 - (int)v7, 0, 0, 0, v35);
          return 1;
        }
      }
      if ( v6 == 37 )
        break;
      v8 = (unsigned char *)a3++;
LABEL_5:
      v6 = v8[1];
      if ( !v6 )
        goto LABEL_14;
    }
    if ( !(char)v9 )
    {
      v32 = dcgettext(0, "error: %s at end of format string", 5);
      error(1, 0, v32, a3);
LABEL_58:
      v33 = dcgettext(0, "error: the format directive `%%%c' is reserved for future use", 5);
      error(1, 0, v33, (unsigned int)v3);
      return sub_10870();
    }
    if ( (char)v9 == 37 )
    {
      v8 = (unsigned char *)(a3 + 1);
      LODWORD(v3) = 37;
      if ( strchr("abcdDfFgGhHiklmMnpPsStuUyYZ%", 37) )
      {
        v17 = a3 + 1;
        LODWORD(v3) = 37;
        v23 = 0;
        goto LABEL_41;
      }
    }
    else
    {
      for ( i = 0; ; LOBYTE(v9) = a3[i + 1] )
      {
        ++i;
        if ( !(char)v9 || !strchr("-+ #", (char)v9) )
          break;
      }
      v16 = *__ctype_b_loc();
      while ( (v16[(unsigned char)v9] & 0x800) != 0 )
        LOBYTE(v9) = a3[++i];
      v8 = (unsigned char *)&a3[i];
      if ( (char)v9 == 46 )
      {
        v8 = (unsigned char *)&a3[i + 1];
        v9 = *v8;
        if ( (v16[v9] & 0x800) != 0 )
        {
          v30 = (unsigned char *)&a3[i + 2];
          do
          {
            v31 = *v30;
            v8 = v30++;
            LOBYTE(v9) = v31;
          }
          while ( (v16[v31] & 0x800) != 0 );
        }
      }
      LODWORD(v3) = (char)v9;
      if ( strchr("abcdDfFgGhHiklmMnpPsStuUyYZ%", (char)v9) )
      {
        if ( !(char)v9 )
        {
          LODWORD(v3) = 0;
LABEL_30:
          if ( strchr("{[(", (int)v3) )
            goto LABEL_58;
          v17 = (const char *)(v8 + 1);
          v18 = "warning: unrecognized format directive `%%%c'";
LABEL_32:
          v19 = dcgettext(0, v18, 5);
          error(0, 0, v19, (unsigned int)v3);
          v20 = v7;
          v21 = (int)v17 - (int)v7;
          v7 = v17;
          v34 = (long long *)sub_100B0(v34, v20, v21, 0, 0, 0, v35);
LABEL_33:
          a3 = (char *)v7;
          goto LABEL_5;
        }
        v17 = (const char *)v8;
        v23 = 0;
LABEL_41:
        v24 = v7;
        v25 = (int)v8 - (int)v7;
        v8 = (unsigned char *)v17;
        v7 = v17 + 1;
        v34 = (long long *)sub_100B0(v34, v24, v25, 2u, (char)v3, v23, v35);
        goto LABEL_33;
      }
    }
    if ( !strchr("ABCT", (int)v3) )
      goto LABEL_30;
    v23 = v8[1];
    v17 = (const char *)(v8 + 1);
    if ( !v23 )
    {
      if ( strchr("{[(", (int)v3) )
        goto LABEL_58;
      v18 = "warning: format directive `%%%c' should be followed by another character";
      goto LABEL_32;
    }
    goto LABEL_41;
  }
  return 1;
}



// Function: get_file_system_list @ 0x10870
void *get_file_system_list(unsigned char a1)
{
  void *v2; // r12
  void *result; // rax
  void *v4; // rdi

  v2 = qword_38E40;
  if ( qword_38E40 )
  {
    if ( byte_38E38 == 1 || !a1 )
      return qword_38E40;
    do
    {
      v4 = v2;
      v2 = (void *)*((long long *)v2 + 6);
      sub_1EFB0(v4);
    }
    while ( v2 );
    qword_38E40 = 0;
  }
  result = (void *)sub_1E550(a1);
  byte_38E38 = a1;
  qword_38E40 = result;
  return result;
}



// Function: filesystem_type @ 0x108e0
void *filesystem_type(long long *a1)
{
  void *result; // rax
  const char **v2; // rbx
  const char **v3; // rbp
  const char *v4; // rax
  char *v5; // rax
  char *v6; // rax
  long long v7[25]; // [rsp+0h] [rbp-C8h] BYREF

  v7[19] = __readfsqword(0x28u);
  result = qword_38E58;
  if ( qword_38E58 )
  {
    if ( byte_38E50 && *a1 == qword_38E48 )
      return result;
    free(qword_38E58);
  }
  qword_38E48 = *a1;
  v2 = (const char **)sub_10870(1u);
  if ( !v2 )
  {
    v6 = dcgettext(0, "Cannot read mounted file system list", 5);
    error(1, 0, v6);
    return (void *)sub_10A50((char *)&dword_0 + 1);
  }
  v3 = 0;
  do
  {
    if ( !strcmp(v2[3], "ignore") )
      goto LABEL_9;
    v4 = v2[4];
    if ( v4 == (const char *)-1LL )
    {
      sub_D810(v7);
      if ( (unsigned int)qword_38670(v2[1], v7) )
        goto LABEL_9;
      v4 = (const char *)v7[0];
      v2[4] = (const char *)v7[0];
    }
    if ( (const char *)*a1 == v4 )
      v3 = v2;
LABEL_9:
    v2 = (const char **)v2[6];
  }
  while ( v2 );
  if ( v3 )
  {
    result = (void *)sub_25F30((void *)v3[3]);
    byte_38E50 = 1;
  }
  else
  {
    byte_38E50 = 0;
    v5 = dcgettext(0, "unknown", 5);
    result = (void *)sub_25F30(v5);
  }
  qword_38E58 = result;
  return result;
}



// Function: is_used_fs_type @ 0x10a50
long long is_used_fs_type(char *s1)
{
  void *v2; // rbx

  if ( strcmp("afs", s1) )
  {
    v2 = sub_10870(0);
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



// Function: get_mounted_devices @ 0x10ac0
void *get_mounted_devices(long long *a1)
{
  long long v1; // rax
  long long *v2; // rbp
  long long *v3; // r14
  void *v4; // r15
  long long v5; // rax
  long long v6; // rbx
  void *v7; // rdi
  void *v8; // rdi
  long long v10; // [rsp+8h] [rbp-100h]
  long long v12[27]; // [rsp+30h] [rbp-D8h] BYREF

  v12[19] = __readfsqword(0x28u);
  v1 = sub_1E550(0);
  if ( !v1 )
    return 0;
  v2 = (long long *)v1;
  v3 = (long long *)v1;
  v4 = 0;
  v10 = 0;
  do
  {
    while ( 1 )
    {
      v6 = sub_15BE0(v4);
      if ( !v6 )
        break;
      v5 = v3[4];
      if ( v5 == -1 )
      {
        sub_D810(v12);
        if ( (unsigned int)qword_38670(v3[1], v12) )
        {
          v4 = (void *)v6;
          goto LABEL_5;
        }
        v5 = v12[0];
        v3[4] = v12[0];
      }
      *(long long *)(v6 + 8 * (v10 + 1) - 8) = v5;
      v4 = (void *)v6;
      ++v10;
LABEL_5:
      v3 = (long long *)v3[6];
      if ( !v3 )
        goto LABEL_8;
    }
    v7 = v4;
    v4 = 0;
    free(v7);
    v3 = (long long *)v3[6];
  }
  while ( v3 );
  do
  {
LABEL_8:
    v8 = v2;
    v2 = (long long *)v2[6];
    sub_1EFB0(v8);
  }
  while ( v2 );
  if ( v4 )
    *a1 = v10;
  return v4;
}



// Function: parse_files0_from @ 0x10c30
long long parse_files0_from(long long a1, long long a2, int *a3)
{
  unsigned int v3; // r8d
  long long v4; // rax
  char *v5; // rcx

  if ( !a2 )
    return 0;
  v3 = 0;
  v4 = *a3;
  v5 = *(char **)(a2 + 8 * v4);
  if ( v5 )
  {
    v3 = 1;
    *a3 = v4 + 1;
    filename = v5;
  }
  return v3;
}



// Function: parse_version @ 0x10c80
void parse_version()
{
  char *v0; // rax
  const char *v1; // rdx
  int v2; // [rsp+0h] [rbp-14h] BYREF
  unsigned long long v3; // [rsp+4h] [rbp-10h]

  v3 = __readfsqword(0x28u);
  sub_16370("find");
  v0 = dcgettext(0, "Features enabled: ", 5);
  __printf_chk(1, v0);
  __printf_chk(1, "D_TYPE ");
  v1 = "enabled";
  if ( !byte_38678 )
    v1 = "disabled";
  __printf_chk(1, "O_NOFOLLOW(%s) ", v1);
  __printf_chk(1, "LEAF_OPTIMISATION ");
  if ( (int)is_selinux_enabled() > 0 )
    __printf_chk(1, "SELINUX ");
  v2 = 0;
  if ( (unsigned char)sub_8530(&v2) )
  {
    __printf_chk(1, "FTS(");
    if ( (v2 & 0x200) != 0 )
      __printf_chk(1, "FTS_CWDFD");
    __printf_chk(1, ") ");
  }
  __printf_chk(1, "CBO(level=%d) ", (unsigned short)word_38688);
  __printf_chk(1, "\n");
  exit(0);
}



// Function: parse_quit @ 0x10e20
long long parse_quit(long long a1)
{
  long long *v1; // rax

  v1 = sub_D520(a1);
  *((int *)v1 + 6) = 1;
  *((int *)v1 + 9) = 1065353216;
  return 1;
}



// Function: parse_prune @ 0x10e50
long long parse_prune(long long a1)
{
  long long *v1; // rax

  v1 = sub_D520(a1);
  if ( !(char)word_38620 )
    *((short *)v1 + 13) = 0;
  *((short *)v1 + 12) = 1;
  return 1;
}



// Function: parse_nouser @ 0x10e80
long long parse_nouser(long long a1)
{
  *((int *)sub_D520(a1) + 9) = 981668463;
  return 1;
}



// Function: parse_accesscheck @ 0x10ea0
long long parse_accesscheck(long long a1)
{
  long long *v1; // rax
  int v2; // xmm0_4

  v1 = sub_D520(a1);
  v2 = 1045220557;
  *((int *)v1 + 6) = 0;
  if ( (bool (*)())*v1 != sub_96E0 )
    v2 = 1063675494;
  *((int *)v1 + 9) = v2;
  return 1;
}



// Function: parse_empty @ 0x10ee0
long long parse_empty(long long a1)
{
  *((int *)sub_D520(a1) + 9) = 1008981770;
  return 1;
}



// Function: parse_delete @ 0x10f00
long long parse_delete(long long a1)
{
  long long *v1; // rax

  v1 = sub_D520(a1);
  LOBYTE(word_38620) = 0;
  *((int *)v1 + 6) = 257;
  *((int *)v1 + 9) = 1065353216;
  return 1;
}



// Function: insert_type @ 0x10f30
long long insert_type(
        long long a1,
        int *a2,
        long long a3,
        long long (*a4)(long long a1, long long a2, long long a3),
        double a5)
{
  const char *v5; // r12
  char *v6; // rbx
  long long (*v7)(long long, long long, long long); // r13
  long long v8; // rax
  long long v9; // rdx
  char v10; // al
  unsigned int v11; // r13d
  unsigned char v12; // al
  long long v13; // rax
  char v14; // al
  char v16; // al
  char *v17; // rax
  char *v18; // rax
  char *v19; // rax
  char *v20; // rax
  char *v21; // rax
  char *v22; // rax
  int *v23; // rdx

  while ( 1 )
  {
    v5 = "-xtype";
    if ( a4 != sub_9AF0 )
      v5 = "-type";
    if ( !a1 )
      return 0;
    v6 = *(char **)(a1 + 8LL * *a2);
    if ( !v6 )
      return 0;
    ++*a2;
    if ( !*v6 )
      goto LABEL_28;
    v7 = a4;
    v8 = sub_D490(a3, a4);
    *(int *)(v8 + 36) = 0;
    v9 = v8;
    *(char *)(v8 + 26) = v7 == sub_9AF0;
    *(char *)(v8 + 27) = v7 != sub_9AF0;
    v10 = *v6;
    if ( !*v6 )
      return 1;
    v11 = v10;
    v12 = v10 - 68;
    while ( 2 )
    {
      switch ( v12 )
      {
        case 0u:
          v18 = dcgettext(
                  0,
                  "%s %c is not supported because Solaris doors are not supported on the platform find was compiled on.",
                  5);
          error(1, 0, v18, v5, 68, a5, COERCE_DOUBLE(1063256064), COERCE_DOUBLE(1023329645), COERCE_DOUBLE(922581086));
          goto LABEL_25;
        case 0x1Eu:
          *(long long *)&a5 = 979945644;
          v13 = 0;
          goto LABEL_10;
        case 0x1Fu:
          *(long long *)&a5 = 971522676;
          v13 = 1;
          goto LABEL_10;
        case 0x20u:
          *(long long *)&a5 = 1035785051;
          v13 = 2;
          goto LABEL_10;
        case 0x22u:
          *(long long *)&a5 = 1063256064;
          v13 = 3;
          goto LABEL_10;
        case 0x28u:
          *(long long *)&a5 = 1023329645;
          v13 = 4;
          goto LABEL_10;
        case 0x2Cu:
          *(long long *)&a5 = 922581086;
          v13 = 5;
          goto LABEL_10;
        case 0x2Fu:
          *(long long *)&a5 = 931487997;
          v13 = 6;
LABEL_10:
          if ( !*(char *)(v9 + v13 + 56) )
          {
            *(float *)&a5 = *(float *)&a5 + *(float *)(v9 + 36);
            *(int *)(v9 + 36) = LODWORD(a5);
            *(char *)(v9 + v13 + 56) = 1;
            v14 = v6[1];
            if ( !v14 )
              return 1;
            if ( v14 != 44 )
              goto LABEL_26;
            v16 = v6[2];
            if ( !v16 )
              goto LABEL_27;
            v11 = v16;
            v12 = v16 - 68;
            v6 += 2;
            if ( v12 > 0x2Fu )
            {
LABEL_22:
              v17 = dcgettext(0, "Unknown argument to %s: %c", 5);
              error(
                1,
                0,
                v17,
                v5,
                v11,
                a5,
                COERCE_DOUBLE(1063256064),
                COERCE_DOUBLE(1023329645),
                COERCE_DOUBLE(922581086));
              return 0;
            }
            continue;
          }
LABEL_25:
          v19 = dcgettext(0, "Duplicate file type '%c' in the argument list to %s.", 5);
          error(1, 0, v19, v11, v5, a5, COERCE_DOUBLE(1063256064), COERCE_DOUBLE(1023329645), COERCE_DOUBLE(922581086));
LABEL_26:
          v20 = dcgettext(0, "Must separate multiple arguments to %s using: ','", 5);
          error(1, 0, v20, v5, a5, COERCE_DOUBLE(1063256064), COERCE_DOUBLE(1023329645), COERCE_DOUBLE(922581086));
LABEL_27:
          v21 = dcgettext(0, "Last file type in list argument to %s is missing, i.e., list is ending on: ','", 5);
          error(1, 0, v21, v5, a5, COERCE_DOUBLE(1063256064), COERCE_DOUBLE(1023329645), COERCE_DOUBLE(922581086));
LABEL_28:
          v22 = dcgettext(0, "Arguments to %s should contain at least one letter", 5);
          error(1, 0, v22, v5);
          a4 = sub_9AF0;
          a1 = 0;
          a2 = v23;
          a3 = 1;
          break;
        default:
          goto LABEL_22;
      }
      break;
    }
  }
}



// Function: parse_type @ 0x11200
long long parse_type(long long a1, long long a2, int *a3, double a4)
{
  return sub_10F30(a2, a3, a1, sub_9900, a4);
}



// Function: fnmatch_sanitycheck @ 0x11220
long long fnmatch_sanitycheck()
{
  char *v1; // rax

  if ( byte_38E60 )
    return 1;
  if ( fnmatch("foo", "foo", 0) || !fnmatch("Foo", "foo", 0) || fnmatch("Foo", "foo", 16) )
  {
    v1 = dcgettext(0, "sanity check of the fnmatch() library function failed.", 5);
    error(1, 0, v1);
    return sub_112C0();
  }
  else
  {
    byte_38E60 = 1;
    return 1;
  }
}



// Function: insert_path_check @ 0x112c0
long long insert_path_check(
        long long a1,
        long long a2,
        int *a3,
        long long a4,
        bool (*a5)(char *name, long long a2, long long a3))
{
  unsigned int v8; // r8d
  char *v9; // rbx
  long long v10; // rbp
  bool v11; // zf
  char *v13; // rax
  char *v14; // rax

  sub_11220();
  if ( a2 )
  {
    v8 = 0;
    v9 = *(char **)(a2 + 8LL * *a3);
    if ( !v9 )
      return v8;
    ++*a3;
    v10 = sub_D490(a1, a5);
    *(short *)(v10 + 26) = 0;
    *(long long *)(v10 + 56) = v9;
    if ( strpbrk(v9, "*?[") )
    {
      v11 = byte_38631 == 0;
      *(int *)(v10 + 36) = 1061997773;
      if ( !v11 )
        return 1;
    }
    else
    {
      v11 = byte_38631 == 0;
      *(int *)(v10 + 36) = 1036831949;
      if ( !v11 )
        return 1;
    }
    v13 = strrchr(v9, 47);
    if ( v13 && !v13[1] && !sub_B4B0(v9, a5 == sub_91E0) )
    {
      v14 = dcgettext(0, "warning: -%s %s will not match anything because it ends with /.", 5);
      error(0, 0, v14, a4, v9);
      *(int *)(v10 + 36) = 841731191;
    }
    return 1;
  }
  return 0;
}



// Function: parse_wholename @ 0x113f0
long long parse_wholename(long long a1, long long a2, int *a3)
{
  return sub_112C0(a1, a2, a3, (long long)"wholename", sub_9640);
}



// Function: parse_nogroup @ 0x115c0
long long parse_nogroup(long long a1)
{
  *(int *)(sub_D4D0(a1, 0) + 36) = 953267991;
  return 1;
}



// Function: parse_lname @ 0x115e0
long long parse_lname(long long a1, long long a2, int *a3)
{
  long long v4; // rdx
  unsigned int v5; // r8d
  const char *v6; // r13
  long long v7; // rsi
  long long v8; // rax
  long long v9; // rbx
  int v10; // xmm0_4

  sub_11220();
  if ( !a2 )
    return 0;
  v4 = *a3;
  v5 = 0;
  v6 = *(const char **)(a2 + 8 * v4);
  if ( v6 )
  {
    v7 = *(long long *)(a2 + 8 * v4);
    ++*a3;
    v8 = sub_D4D0(a1, v7);
    *(long long *)(v8 + 56) = v6;
    v9 = v8;
    v10 = 1034147595;
    if ( !strpbrk(v6, "*?[") )
      v10 = 1008981771;
    *(int *)(v9 + 36) = v10;
    return 1;
  }
  return v5;
}



// Function: parse_ilname @ 0x11690
long long parse_ilname(long long a1, long long a2, int *a3)
{
  unsigned int v3; // r8d
  long long v4; // rcx
  const char *v5; // rbp
  long long v6; // rsi
  long long v7; // rax
  long long v8; // rbx
  int v9; // xmm0_4

  if ( !a2 )
    return 0;
  v3 = 0;
  v4 = *a3;
  v5 = *(const char **)(a2 + 8 * v4);
  if ( v5 )
  {
    v6 = *(long long *)(a2 + 8 * v4);
    ++*a3;
    v7 = sub_D4D0(a1, v6);
    *(long long *)(v7 + 56) = v5;
    v8 = v7;
    v9 = 1034147595;
    if ( !strpbrk(v5, "*?[") )
      v9 = 1008981771;
    *(int *)(v8 + 36) = v9;
    return 1;
  }
  return v3;
}



// Function: parse_size @ 0x11720
long long parse_size(long long a1, long long a2, int *a3)
{
  char v3; // bl
  long long v4; // rax
  char *v6; // r13
  int v7; // eax
  char *v8; // rax
  long long v9; // r12
  char v10; // al
  long long v11; // rax
  long long v12; // rdx
  int v13; // xmm0_4
  char *v15; // rax
  char *v16; // rax
  char *v17; // rax
  long long v18; // [rsp+0h] [rbp-48h]
  char v19[2]; // [rsp+16h] [rbp-32h] BYREF
  unsigned long long v20; // [rsp+18h] [rbp-30h]

  v20 = __readfsqword(0x28u);
  if ( !a2 )
    return 0;
  v4 = *a3;
  v6 = *(char **)(a2 + 8 * v4);
  if ( !v6 )
    return 0;
  v7 = strlen(*(const char **)(a2 + 8 * v4));
  if ( !v7 )
    goto LABEL_34;
  v8 = &v6[v7 - 1];
  v3 = *v8;
  if ( *v8 == 98 )
  {
    *v8 = 0;
    v9 = 512;
    goto LABEL_10;
  }
  if ( v3 <= 98 )
  {
    if ( v3 != 71 )
    {
      if ( v3 <= 71 )
      {
        if ( (unsigned char)(v3 - 48) <= 9u )
        {
          v10 = *v6;
          v9 = 512;
          v3 = 0;
          if ( *v6 != 43 )
          {
LABEL_11:
            if ( v10 == 45 )
            {
              if ( !(unsigned int)sub_26490(v6 + 1) )
              {
                v11 = sub_D4D0(a1, v6);
                v12 = v18;
                v13 = 1063675494;
                *(int *)(v11 + 60) = v9;
                *(int *)(v11 + 56) = 1;
                if ( (unsigned long long)(v18 * v9) <= 0x5000 )
                  v13 = 1036831949;
                goto LABEL_14;
              }
            }
            else if ( !(unsigned int)sub_26490(v6) )
            {
              v11 = sub_D4D0(a1, v6);
              v12 = v18;
              v13 = 1008981770;
              *(int *)(v11 + 56) = 2;
              *(int *)(v11 + 60) = v9;
LABEL_14:
              *(long long *)(v11 + 64) = v12;
              *(short *)(v11 + 26) = 1;
              *(int *)(v11 + 36) = v13;
              ++*a3;
              return 1;
            }
            goto LABEL_35;
          }
          goto LABEL_27;
        }
      }
      else if ( v3 == 77 )
      {
        *v8 = 0;
        v9 = 0x100000;
        goto LABEL_10;
      }
      goto LABEL_33;
    }
    *v8 = 0;
    v9 = 0x40000000;
LABEL_10:
    v10 = *v6;
    if ( *v6 != 43 )
      goto LABEL_11;
LABEL_27:
    if ( !(unsigned int)sub_26490(v6 + 1) )
    {
      v11 = sub_D4D0(a1, v6);
      v12 = v18;
      v13 = 1036831949;
      *(int *)(v11 + 60) = v9;
      *(int *)(v11 + 56) = 0;
      if ( (unsigned long long)(v18 * v9) <= 0x5000 )
        v13 = 1063675494;
      goto LABEL_14;
    }
    goto LABEL_35;
  }
  switch ( v3 )
  {
    case 'k':
      *v8 = 0;
      v9 = 1024;
      goto LABEL_10;
    case 'w':
      *v8 = 0;
      v9 = 2;
      goto LABEL_10;
    case 'c':
      *v8 = 0;
      v9 = 1;
      goto LABEL_10;
  }
LABEL_33:
  v15 = dcgettext(0, "invalid -size type `%c'", 5);
  error(1, 0, v15, (unsigned int)v3);
LABEL_34:
  v16 = dcgettext(0, "invalid null argument to -size", 5);
  error(1, 0, v16);
LABEL_35:
  v19[0] = v3;
  v19[1] = 0;
  v17 = dcgettext(0, "Invalid argument `%s%s' to -size", 5);
  error(1, 0, v17, v6, v19);
  return sub_11A40();
}



// Function: parse_samefile @ 0x11a40
long long parse_samefile(long long a1, long long a2, int *a3)
{
  long long v5; // rdx
  long long v6; // r14
  long long v7; // rdi
  int v8; // ebp
  int v9; // edx
  int v10; // ecx
  int v11; // r8d
  int v12; // r9d
  int v13; // ebp
  int v14; // eax
  long long v15; // rax
  long long v16; // rdx
  long long v17; // rdx
  int v19; // edi
  __m128i si128; // xmm1
  __m128i v21; // xmm2
  __m128i v22; // xmm3
  __m128i v23; // xmm4
  __m128i v24; // xmm5
  __m128i v25; // xmm6
  __m128i v26; // xmm7
  __m128i v27; // xmm0
  int *v28; // rax
  __m128i v29; // [rsp+0h] [rbp-168h] BYREF
  __m128i v30; // [rsp+10h] [rbp-158h]
  __m128i v31; // [rsp+20h] [rbp-148h]
  __m128i v32; // [rsp+30h] [rbp-138h]
  __m128i v33; // [rsp+40h] [rbp-128h]
  __m128i v34; // [rsp+50h] [rbp-118h]
  __m128i v35; // [rsp+60h] [rbp-108h]
  __m128i v36; // [rsp+70h] [rbp-F8h]
  __m128i v37; // [rsp+80h] [rbp-E8h]
  struct stat buf; // [rsp+90h] [rbp-D8h] BYREF
  unsigned long long v39; // [rsp+128h] [rbp-40h]

  v39 = __readfsqword(0x28u);
  sub_D810(&v29);
  if ( a2 )
  {
    v5 = *a3;
    v6 = *(long long *)(a2 + 8 * v5);
    if ( v6 )
    {
      v7 = *(long long *)(a2 + 8 * v5);
      ++*a3;
      v8 = qword_38670(v7, &v29);
      if ( !v8 )
      {
        sub_D810(&buf);
        if ( !dword_38668 )
        {
          if ( byte_38678 )
          {
            v8 = 0x20000;
          }
          else if ( (v30.m128i_i16[4] & 0xF000) == 0xA000 )
          {
            v13 = -2;
LABEL_10:
            v15 = sub_D4D0(a1, v6);
            v16 = v29.m128i_i64[1];
            *(int *)(v15 + 72) = v13;
            *(long long *)(v15 + 56) = v16;
            v17 = v29.m128i_i64[0];
            *(int *)(v15 + 36) = 1008981770;
            *(long long *)(v15 + 64) = v17;
            *(short *)(v15 + 26) = 1;
            return 1;
          }
        }
        v14 = sub_160B0(v6, v8, v9, v10, v11, v12, v29.m128i_i8[0]);
        v13 = v14;
        if ( v14 < 0 )
          goto LABEL_10;
        if ( !fstat(v14, &buf) && !(unsigned int)qword_38670(v6, &v29) )
        {
          if ( dword_38668 || byte_38678 )
          {
            si128 = _mm_load_si128((const __m128i *)&buf.st_nlink);
            v21 = _mm_load_si128((const __m128i *)&buf.st_gid);
            v22 = _mm_load_si128((const __m128i *)&buf.st_size);
            v23 = _mm_load_si128((const __m128i *)&buf.st_blocks);
            v29 = _mm_load_si128((const __m128i *)&buf);
            v24 = _mm_load_si128((const __m128i *)&buf.st_atim.tv_nsec);
            v25 = _mm_load_si128((const __m128i *)&buf.st_mtim.tv_nsec);
            v30 = si128;
            v26 = _mm_load_si128((const __m128i *)&buf.st_ctim.tv_nsec);
            v27 = _mm_load_si128((const __m128i *)&buf.__unused[1]);
            v31 = v21;
            v32 = v22;
            v33 = v23;
            v34 = v24;
            v35 = v25;
            v36 = v26;
            v37 = v27;
          }
          else if ( (v30.m128i_i16[4] & 0xF000) == 0xA000 || *(_OWORD *)&v29 != *(_OWORD *)&buf.st_dev )
          {
            v19 = v13;
            v13 = -1;
            close(v19);
          }
          goto LABEL_10;
        }
      }
      v28 = __errno_location();
      sub_E920(*v28, v6);
    }
  }
  return 0;
}



// Function: open_output_file @ 0x11c90
bool open_output_file(char *s1, long long a2)
{
  long long v3; // rax
  FILE *v4; // rdi
  int v5; // edi
  bool result; // al
  char *v7; // rax
  char *v8; // rax
  int *v9; // rax

  *(long long *)a2 = 0;
  *(long long *)(a2 + 32) = sub_23640(0);
  if ( !strcmp(s1, "/dev/stderr") )
  {
    *(long long *)(a2 + 8) = stderr;
    v8 = dcgettext(0, "standard error", 5);
    v4 = *(FILE **)(a2 + 8);
    *(long long *)(a2 + 16) = v8;
  }
  else if ( !strcmp(s1, "/dev/stdout") )
  {
    *(long long *)(a2 + 8) = stdout;
    v7 = dcgettext(0, "standard output", 5);
    v4 = *(FILE **)(a2 + 8);
    *(long long *)(a2 + 16) = v7;
  }
  else
  {
    v3 = sub_EB40((long long *)qword_38610, s1);
    *(long long *)(a2 + 16) = s1;
    *(long long *)(a2 + 8) = v3;
    v4 = (FILE *)v3;
    if ( !v3 )
    {
      v9 = __errno_location();
      sub_E9B0(*v9, (long long)s1);
    }
  }
  v5 = fileno(v4);
  result = 0;
  if ( v5 != -1 )
    result = isatty(v5) != 0;
  *(char *)(a2 + 24) = result;
  return result;
}



// Function: parse_printf @ 0x11d80
long long parse_printf(long long a1, long long a2, int *a3)
{
  char *v4; // r12
  int v5; // ebp
  long long result; // rax
  __m128i v7[2]; // [rsp+0h] [rbp-58h] BYREF
  unsigned long long v8; // [rsp+28h] [rbp-30h]

  v8 = __readfsqword(0x28u);
  if ( !a2 )
    return 0;
  v4 = *(char **)(a2 + 8LL * *a3);
  v5 = *a3;
  result = 0;
  if ( v4 )
  {
    *a3 = v5 + 1;
    sub_11C90("/dev/stdout", (long long)v7);
    result = sub_10380(v7, a1, v4);
    if ( !(char)result )
      *a3 = v5;
  }
  return result;
}



// Function: parse_print @ 0x11e20
long long parse_print(long long a1)
{
  long long *v1; // rax

  v1 = sub_D520(a1);
  *((int *)v1 + 6) = 257;
  sub_11C90("/dev/stdout", (long long)(v1 + 7));
  return 1;
}



// Function: parse_fprint0 @ 0x11e50
long long parse_fprint0(long long a1, long long a2, int *a3)
{
  unsigned int v3; // r8d
  long long v4; // rcx
  char *v5; // rbp
  long long v6; // rsi
  long long v7; // rbx

  if ( !a2 )
    return 0;
  v3 = 0;
  v4 = *a3;
  v5 = *(char **)(a2 + 8 * v4);
  if ( v5 )
  {
    v6 = *(long long *)(a2 + 8 * v4);
    ++*a3;
    v7 = sub_D4D0(a1, v6);
    sub_11C90(v5, v7 + 56);
    *(int *)(v7 + 24) = 257;
    v3 = 1;
    *(int *)(v7 + 36) = 1065353216;
  }
  return v3;
}



// Function: parse_fls @ 0x11ec0
long long parse_fls(long long a1, long long a2, int *a3)
{
  unsigned int v3; // r8d
  char *v4; // rbp
  long long *v5; // rbx

  if ( !a2 )
    return 0;
  v3 = 0;
  v4 = *(char **)(a2 + 8LL * *a3);
  if ( v4 )
  {
    ++*a3;
    v5 = sub_D520(a1);
    sub_11C90(v4, (long long)(v5 + 7));
    *((int *)v5 + 9) = 1065353216;
    v3 = 1;
    *((short *)v5 + 12) = 257;
  }
  return v3;
}



// Function: parse_fprintf @ 0x11f30
long long parse_fprintf(long long a1, long long a2, int *a3)
{
  int v4; // ebp
  char *v6; // rdi
  char *v7; // r13
  long long result; // rax
  __m128i v9[2]; // [rsp+0h] [rbp-58h] BYREF
  unsigned long long v10; // [rsp+28h] [rbp-30h]

  v4 = *a3;
  v10 = __readfsqword(0x28u);
  if ( !a2
    || (v6 = *(char **)(a2 + 8LL * v4)) == 0
    || (v7 = *(char **)(a2 + 8LL * v4 + 8)) == 0
    || (*a3 = v4 + 2, sub_11C90(v6, (long long)v9), v4 = *a3, result = sub_10380(v9, a1, v7), !(char)result) )
  {
    *a3 = v4;
    return 0;
  }
  return result;
}



// Function: parse_fprint @ 0x11fd0
long long parse_fprint(long long a1, long long a2, int *a3)
{
  unsigned int v3; // r8d
  long long v4; // rcx
  char *v5; // rbp
  long long v6; // rsi
  long long v7; // rbx

  if ( !a2 )
    return 0;
  v3 = 0;
  v4 = *a3;
  v5 = *(char **)(a2 + 8 * v4);
  if ( v5 )
  {
    v6 = *(long long *)(a2 + 8 * v4);
    ++*a3;
    v7 = sub_D4D0(a1, v6);
    sub_11C90(v5, v7 + 56);
    *(int *)(v7 + 24) = 257;
    v3 = 1;
    *(int *)(v7 + 36) = 1065353216;
  }
  return v3;
}



// Function: parse_perm @ 0x12040
long long parse_perm(long long a1, long long a2, int *a3)
{
  char *v3; // rbp
  char *v5; // rdi
  int v6; // r14d
  void *v7; // r12
  unsigned int v8; // eax
  long long v9; // rbx
  unsigned int v10; // r15d
  unsigned long long v11; // rbx
  long long v12; // rax
  char *v14; // rax
  long long v15; // r12
  char *v16; // rax
  int v17; // [rsp+0h] [rbp-40h]
  unsigned int v18; // [rsp+4h] [rbp-3Ch]

  if ( !a2 )
    return 0;
  v3 = *(char **)(a2 + 8LL * *a3);
  if ( !v3 )
    return 0;
  ++*a3;
  if ( *v3 == 45 )
  {
    v5 = v3 + 1;
    v6 = 0;
    v17 = 1045220557;
  }
  else if ( *v3 == 47 )
  {
    v5 = v3 + 1;
    v6 = 1;
    v17 = 1050253722;
  }
  else
  {
    v5 = v3;
    v6 = 2;
    v17 = 1008981770;
  }
  v7 = (void *)sub_1DEF0(v5);
  if ( !v7 || *v3 == 43 && (unsigned char)(v3[1] - 48) <= 7u )
  {
    v15 = sub_239F0(0, (unsigned int)dword_3868C, v3);
    v16 = dcgettext(0, "invalid mode %s", 5);
    error(1, 0, v16, v15);
    return sub_12230();
  }
  else
  {
    v8 = sub_1E310(0, 0, 0, v7, 0);
    v9 = v8;
    v18 = v8;
    v10 = sub_1E310(0, 1, 0, v7, 0);
    v11 = ((unsigned long long)v10 << 32) | v9;
    free(v7);
    if ( *v3 == 47 && !(v18 | v10) )
    {
      v6 = 0;
      v14 = dcgettext(
              0,
              "warning: you have specified a mode pattern %s (which is equivalent to /000). The meaning of -perm /000 has"
              " now been changed to be consistent with -perm -000; that is, while it used to match no files, it now matches all files.",
              5);
      error(0, 0, v14, v3);
      v17 = 1065329728;
    }
    v12 = sub_D4D0(a1, v3);
    *(int *)(v12 + 56) = v6;
    *(long long *)(v12 + 60) = v11;
    *(int *)(v12 + 36) = v17;
    return 1;
  }
}



// Function: insert_exec_ok @ 0x12230
long long insert_exec_ok(const char *a1, long long a2, long long a3, signed int *a4)
{
  long long (*v4)(); // rbx
  int *v6; // rcx
  char *v7; // rax
  unsigned long long v8; // r13
  size_t v9; // rsi
  const char *v10; // rdi
  char v11; // al
  char *v12; // rax
  long long v13; // rax
  char *v14; // r14
  char *v15; // r15
  long long *v16; // rbx
  long long v17; // rbp
  int v18; // esi
  signed int v19; // r15d
  char *v20; // rax
  char *v22; // rbp
  char *v23; // r13
  int v24; // r14d
  bool v25; // zf
  char *v26; // r12
  const char **v27; // r14
  long long v28; // r13
  const char *v29; // r15
  size_t v30; // rax
  long long v31; // rcx
  long long v32; // rsi
  long long i; // rax
  char *v34; // rax
  char *v35; // rsi
  long long v36; // r13
  char *v37; // rax
  char *v38; // rax
  char *v39; // rax
  char *s; // [rsp+0h] [rbp-B8h]
  int sa; // [rsp+0h] [rbp-B8h]
  long long (*v42)(); // [rsp+8h] [rbp-B0h]
  int *ptr; // [rsp+18h] [rbp-A0h]
  bool v46; // [rsp+33h] [rbp-85h]
  signed int v47; // [rsp+34h] [rbp-84h]
  long long v48; // [rsp+38h] [rbp-80h]
  const char **v49; // [rsp+48h] [rbp-70h]
  long long v50; // [rsp+50h] [rbp-68h] BYREF
  size_t v51; // [rsp+58h] [rbp-60h] BYREF
  char v52[24]; // [rsp+60h] [rbp-58h] BYREF
  unsigned long long v53; // [rsp+78h] [rbp-40h]

  v4 = *(long long (**)())(a2 + 24);
  v53 = __readfsqword(0x28u);
  v42 = v4;
  if ( !a3 || !*(long long *)(a3 + 8LL * *a4) )
    return 0;
  ptr = (int *)sub_D490(a2, v4);
  v6 = ptr;
  ptr[6] = 257;
  *((long long *)ptr + 31) = 0;
  v46 = (char *)v4 != (char *)sub_95D0 && (char *)v4 != (char *)sub_9580;
  if ( v46 )
    goto LABEL_14;
  byte_38698 = 1;
  *((char *)ptr + 244) = 1;
  if ( v4 == sub_8FC0 || (char *)v4 == (char *)sub_95D0 )
  {
    while ( 1 )
    {
      byte_3862E = 0;
      v7 = getenv("PATH");
      v8 = (unsigned long long)v7;
      if ( !v7 )
        break;
      v4 = (long long (*)())":";
      sub_17570(v7, ":", 1, &v50, &v51);
      while ( 1 )
      {
        v9 = v51;
        if ( !v51 )
          break;
        v10 = (const char *)(v8 + v50);
        v11 = *(char *)(v8 + v50);
        if ( v51 == 1 && v11 == 46 )
          break;
        if ( v11 != 47 )
          goto LABEL_61;
        if ( !(unsigned char)sub_17570(v8, ":", 0, &v50, &v51) )
          goto LABEL_17;
      }
      v12 = dcgettext(
              0,
              "The current directory is included in the PATH environment variable, which is insecure in combination with "
              "the %s action of find.  Please remove the current directory from your $PATH (that is, remove \".\", double"
              "d colons, or leading or trailing colons)",
              5);
      error(1, 0, v12, a1);
LABEL_14:
      *((char *)v6 + 244) = 0;
      if ( v4 != sub_8FC0 )
        goto LABEL_15;
    }
  }
  else
  {
LABEL_15:
    if ( !qword_385C0 )
      goto LABEL_64;
    *((long long *)ptr + 31) = qword_385C0;
  }
LABEL_17:
  *((char *)ptr + 56) = 0;
  v47 = *a4;
  v48 = *a4;
  v13 = 8 * v48;
  if ( !*(long long *)(a3 + 8 * v48) )
  {
    v19 = *a4;
    goto LABEL_44;
  }
  v14 = (char *)(a3 + v13 + 8);
  v49 = (const char **)(a3 + v13);
  v15 = *(char **)(a3 + 8LL * *a4);
  s = 0;
  v16 = (long long *)(a3 + v13);
  LODWORD(v17) = 0;
  v18 = 0;
  a1 = "{}";
  v8 = (unsigned int)*a4;
  while ( *v15 == 59 )
  {
    if ( !v15[1] )
    {
      v19 = v8;
      goto LABEL_30;
    }
LABEL_21:
    if ( sub_1D5D0(v15, a1) )
    {
      LODWORD(v17) = v17 + 1;
      if ( !(int)v8 && (v42 == sub_8FC0 || (char *)v42 == (char *)sub_95D0) )
      {
        v34 = dcgettext(
                0,
                "You may not use {} within the utility name for -execdir and -okdir, because this is a potential security problem.",
                5);
        v9 = 0;
        v10 = (char *)&dword_0 + 1;
        error((int)&dword_0 + 1, 0, v34);
LABEL_61:
        v35 = strndup(v10, v9);
        if ( !v35 )
          goto LABEL_66;
        goto LABEL_62;
      }
      s = v15;
      v18 = 1;
      goto LABEL_26;
    }
LABEL_38:
    v18 = 0;
LABEL_26:
    v15 = *(char **)v14;
    v8 = (unsigned int)(v8 + 1);
    v14 += 8;
    ++v16;
    if ( !v15 )
    {
      v19 = v8;
LABEL_44:
      *a4 = v19;
      free(ptr);
      return 0;
    }
  }
  if ( *v15 != 43 || !v46 || v15[1] || !v18 )
    goto LABEL_21;
  v19 = v8;
  *((char *)ptr + 56) = 1;
LABEL_30:
  if ( v47 == v19 || !*v16 )
    goto LABEL_44;
  if ( !*((char *)ptr + 56) )
    goto LABEL_45;
  a1 = "dir";
  if ( v42 != sub_8FC0 )
    a1 = "";
  if ( (int)v17 > 1 )
  {
    v39 = dcgettext(0, "Only one instance of {} is supported with -exec%s ... +", 5);
    error(1, 0, v39, a1);
LABEL_66:
    v35 = (char *)(v8 + v50);
LABEL_62:
    v36 = sub_E8A0(0, v35);
    v37 = dcgettext(
            0,
            "The relative path %s is included in the PATH environment variable, which is insecure in combination with the"
            " %s action of find.  Please remove that entry from $PATH",
            5);
    error(1, 0, v37, v36, a1);
  }
  v14 = s;
  if ( strlen(s) != 2 )
  {
    v8 = (unsigned long long)v52;
    __snprintf_chk(v52, 19, 1, 19, "-exec%s ... {} +", a1);
    v17 = sub_239F0(2, (unsigned int)dword_3868C, s);
    v16 = (long long *)sub_239F0(1, (unsigned int)dword_3868C, "{}");
    a1 = (const char *)sub_239F0(0, (unsigned int)dword_3868C, v52);
    v20 = dcgettext(0, "In %s the %s must appear by itself, but you specified %s", 5);
    error(1, 0, v20, a1, v16, v17);
    goto LABEL_38;
  }
LABEL_45:
  v22 = (char *)(ptr + 16);
  v23 = (char *)(ptr + 14);
  if ( (unsigned int)sub_156E0(ptr + 16, 2048) - 1 <= 1 )
  {
    v38 = dcgettext(0, "The environment is too large for exec().", 5);
    error(1, 0, v38);
LABEL_64:
    __assert_fail("NULL != initial_wd", "parser.c", 0xB79u, "insert_exec_ok");
  }
  sub_15860(v22);
  v24 = v19 - v47;
  v25 = *((char *)ptr + 56) == 0;
  v26 = (char *)(ptr + 38);
  *((long long *)ptr + 16) = sub_A3A0;
  if ( v25 )
  {
    ptr[60] = v24;
    *((long long *)ptr + 14) = "{}";
    *((long long *)ptr + 13) = strlen("{}");
    *((long long *)ptr + 17) = 0;
    *((long long *)ptr + 18) = 0;
    *((long long *)ptr + 29) = sub_25A50(8LL * v24);
    sub_15890(v22, v26, v23);
    v31 = (int)ptr[60];
    if ( (int)v31 > 0 )
    {
      v32 = *((long long *)ptr + 29);
      for ( i = 0; i != v31; ++i )
        *(long long *)(v32 + 8 * i) = *(long long *)(a3 + 8 * v48 + 8 * i);
    }
  }
  else
  {
    *((long long *)ptr + 29) = 0;
    *((long long *)ptr + 14) = 0;
    *((long long *)ptr + 15) = v24 - 1;
    *((long long *)ptr + 13) = 0;
    *((long long *)ptr + 17) = 0;
    *((long long *)ptr + 18) = 0;
    sub_15890(v22, v26, v23);
    if ( v19 - 1 > v47 )
    {
      sa = v19;
      v27 = v49;
      v28 = a3 + 8 * (v48 + (unsigned int)(v19 - v47 - 2)) + 8;
      do
      {
        v29 = *v27++;
        v30 = strlen(v29);
        sub_14FB0(v22, v26, v29, v30 + 1, 0, 0, 1);
      }
      while ( v27 != (const char **)v28 );
      v19 = sa;
    }
  }
  if ( *v16 )
    *a4 = v19 + 1;
  else
    *a4 = v19;
  return 1;
}



// Function: parse_okdir @ 0x12920
long long parse_okdir(long long a1, long long a2, signed int *a3)
{
  return sub_12230("-okdir", a1, a2, a3);
}



// Function: parse_or @ 0x129a0
long long parse_or()
{
  long long v0; // rax

  v0 = sub_B990();
  *(long long *)(v0 + 16) = 0x200000003LL;
  *(long long *)v0 = sub_8940;
  *(short *)(v0 + 26) = 0;
  return 1;
}



// Function: parse_and @ 0x129e0
long long parse_and()
{
  long long v0; // rax

  v0 = sub_B990();
  *(long long *)(v0 + 16) = 0x300000003LL;
  *(long long *)v0 = sub_8990;
  *(short *)(v0 + 26) = 0;
  return 1;
}



// Function: parse_comma @ 0x12a20
long long parse_comma()
{
  long long v0; // rax

  v0 = sub_B990();
  *(long long *)(v0 + 16) = 0x100000003LL;
  *(long long *)v0 = sub_8CA0;
  *(short *)(v0 + 26) = 0;
  *(int *)(v0 + 36) = 1065353216;
  return 1;
}



// Function: parse_closeparen @ 0x12a60
long long parse_closeparen()
{
  long long v0; // rax

  v0 = sub_B990();
  *(long long *)(v0 + 16) = 5;
  *(long long *)v0 = sub_8BE0;
  *(short *)(v0 + 26) = 0;
  return 1;
}



// Function: parse_group @ 0x12a90
long long parse_group(long long a1, long long a2, int *a3)
{
  int v3; // eax
  char *v4; // r12
  struct group *v5; // rbx
  __gid_t gr_gid; // ebx
  long long v7; // rax
  int v8; // xmm0_4
  bool v9; // cc
  int v11; // eax
  long long v12; // rsi
  char *v13; // rax
  long long v14; // r12
  char *v15; // rax
  char *v16; // rax
  long long v17; // rbx
  long long v18; // r12
  char *v19; // rax

  v3 = *a3;
  if ( !a2 )
    return 0;
  v4 = *(char **)(a2 + 8LL * v3);
  if ( !v4 )
    return 0;
  *a3 = v3 + 1;
  v5 = getgrnam(v4);
  endgrent();
  if ( !v5 )
  {
    v11 = strspn(v4, "0123456789");
    if ( v11 )
    {
      v12 = (unsigned int)dword_3868C;
      v13 = &v4[v11];
      if ( !*v13 )
      {
        gr_gid = sub_17410(v4);
        goto LABEL_5;
      }
LABEL_16:
      v17 = sub_239F0(1, v12, v13);
      v18 = sub_239F0(0, (unsigned int)dword_3868C, v4);
      v19 = dcgettext(
              0,
              "%s is not the name of an existing group and it does not look like a numeric group ID because it has the un"
              "expected suffix %s",
              5);
      error(1, 0, v19, v18, v17);
      return sub_12C20();
    }
    if ( !*v4 )
    {
      v16 = dcgettext(0, "argument to -group is empty, but should be a group name", 5);
      v12 = 0;
      error(1, 0, v16);
      goto LABEL_16;
    }
    v14 = sub_239F0(0, (unsigned int)dword_3868C, v4);
    v15 = dcgettext(0, "%s is not the name of an existing group", 5);
    error(1, 0, v15, v14);
    return 0;
  }
  gr_gid = v5->gr_gid;
LABEL_5:
  v7 = sub_D4D0(a1, v4);
  v8 = 1065185444;
  v9 = *(long long *)(v7 + 64) <= 0x63u;
  *(int *)(v7 + 56) = gr_gid;
  if ( !v9 )
    v8 = 1045220557;
  *(int *)(v7 + 36) = v8;
  return 1;
}



// Function: parse_negate @ 0x12c20
long long parse_negate(int *a1)
{
  long long v1; // rax

  v1 = sub_B9B0(a1, 0);
  *(long long *)(v1 + 16) = 0x400000002LL;
  *(long long *)v1 = sub_9350;
  *(short *)(v1 + 26) = 0;
  return 1;
}



// Function: parse_openparen @ 0x12c60
long long parse_openparen(int *a1)
{
  long long v1; // rax

  v1 = sub_B9B0(a1, 0);
  *(long long *)(v1 + 16) = 4;
  *(long long *)v1 = sub_9630;
  *(short *)(v1 + 26) = 0;
  return 1;
}



// Function: optionl_getfilecon @ 0x12dc0
long long optionl_getfilecon(int fd)
{
  if ( (unsigned int)sub_242E0(fd) )
    return sub_12D30(fd);
  else
    return 0;
}



// Function: parse_name @ 0x12fc0
long long parse_name(long long a1, long long a2, int *a3)
{
  long long v3; // rax
  unsigned int v5; // r12d
  const char *v6; // r13
  int v7; // ebp
  long long v9; // rbx
  int v10; // xmm0_4

  if ( !a2 )
    return 0;
  v3 = *a3;
  v5 = 0;
  v6 = *(const char **)(a2 + 8 * v3);
  v7 = *a3;
  if ( !v6 )
    return v5;
  *a3 = v3 + 1;
  sub_11220();
  v5 = sub_12F00("-name", "-wholename", v6);
  if ( !(char)v5 )
  {
    *a3 = v7;
    return v5;
  }
  v9 = sub_D4D0(a1, v6);
  *(short *)(v9 + 26) = 0;
  *(long long *)(v9 + 56) = v6;
  v10 = 1061997773;
  if ( !strpbrk(v6, "*?[") )
    v10 = 1036831949;
  *(int *)(v9 + 36) = v10;
  return v5;
}



// Function: parse_iname @ 0x13090
long long parse_iname(long long a1, long long a2, int *a3)
{
  long long v4; // rdx
  const char *v5; // r14
  long long v6; // rdx
  unsigned int v7; // r12d
  long long v9; // rbx
  int v10; // xmm0_4

  sub_11220();
  if ( !a2 )
    return 0;
  v4 = *a3;
  v5 = *(const char **)(a2 + 8 * v4);
  if ( !v5 )
    return 0;
  v6 = *(long long *)(a2 + 8 * v4);
  ++*a3;
  v7 = sub_12F00("-iname", "-iwholename", v6);
  if ( !(char)v7 )
    return 0;
  v9 = sub_D4D0(a1, v5);
  *(short *)(v9 + 26) = 0;
  *(long long *)(v9 + 56) = v5;
  v10 = 1061997773;
  if ( !strpbrk(v5, "*?[") )
    v10 = 1036831949;
  *(int *)(v9 + 36) = v10;
  return v7;
}



// Function: parse_regextype @ 0x13420
long long parse_regextype(long long a1, long long a2, int *a3)
{
  char *v3; // rdi

  if ( !a2 )
    return 0;
  v3 = *(char **)(a2 + 8LL * *a3);
  if ( !v3 )
    return 0;
  ++*a3;
  dword_3867C = sub_17200(v3);
  return sub_133B0();
}



// Function: insert_depthspec @ 0x13460
long long insert_depthspec(long long a1, long long a2, int *a3, int *a4)
{
  int *v4; // rbx
  long long v5; // rcx
  long long v6; // r12
  char *v7; // r13
  int v8; // eax
  long long v9; // rsi
  int v10; // eax
  long long v12; // rbx
  char *v13; // rax

  while ( 1 )
  {
    v4 = a4;
    v5 = *a3;
    v6 = *(long long *)(a2 + 8 * v5 - 8);
    if ( !a2 )
      break;
    v7 = *(char **)(a2 + 8 * v5);
    if ( !v7 )
      break;
    *a3 = v5 + 1;
    v8 = strspn(v7, "0123456789");
    v9 = (unsigned int)dword_3868C;
    if ( v8 > 0 && !v7[v8] )
    {
      v10 = sub_17410(v7);
      *v4 = v10;
      if ( v10 >= 0 )
        return sub_133B0(v7, v9);
      v9 = (unsigned int)dword_3868C;
    }
    v12 = sub_239F0(0, v9, v7);
    v13 = dcgettext(0, "Expected a positive decimal integer argument to %s, but got %s", 5);
    a2 = 0;
    error(1, 0, v13, v6, v12);
    a4 = &dword_38628;
  }
  return 0;
}



// Function: parse_maxdepth @ 0x13520
long long parse_maxdepth(long long a1, long long a2, int *a3)
{
  return sub_13460(a1, a2, a3, &dword_38624);
}



// Function: parse_newerXY @ 0x13620
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
  long long v18; // rax
  long long v19; // rdx
  time_t v20; // rcx
  int v21; // xmm1_4
  float v22; // xmm0_4
  bool (*v23)(long long, long long *, long long); // rax
  long long v24; // r12
  char *v25; // rax
  long long v26; // r12
  char *v27; // rax
  int *v28; // rax
  long long v29; // [rsp+8h] [rbp-F0h]
  char v30; // [rsp+1Ah] [rbp-DEh]
  char v31[72]; // [rsp+20h] [rbp-D8h] BYREF
  long long v32; // [rsp+68h] [rbp-90h]
  long long v33; // [rsp+70h] [rbp-88h]
  long long v34; // [rsp+78h] [rbp-80h]
  long long v35; // [rsp+80h] [rbp-78h]
  long long v36; // [rsp+88h] [rbp-70h]
  long long v37; // [rsp+90h] [rbp-68h]
  char s[6]; // [rsp+B2h] [rbp-46h] BYREF
  unsigned long long v39; // [rsp+B8h] [rbp-40h]

  v39 = __readfsqword(0x28u);
  if ( !a2 )
    return 0;
  v3 = *a3;
  v6 = *(long long *)(a2 + 8 * v3);
  v7 = *a3;
  v29 = 8 * v3;
  if ( !v6 || strlen((const char *)v6) != 8 )
    return 0;
  strcpy(s, "aBcmt");
  if ( strncmp("-newer", (const char *)v6, 6u) )
    goto LABEL_43;
  v9 = *(char *)(v6 + 6);
  v30 = *(char *)(v6 + 7);
  LOBYTE(v8) = v30 == 66 || v9 == 66;
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
      v11 = strchr(s, v30);
      v8 = 0;
      if ( v11 )
      {
        v12 = v7 + 1;
        v13 = *(long long *)(a2 + v29 + 8);
        if ( v13 )
        {
          *a3 = v12;
          v14 = sub_D4D0(a1, v13);
          switch ( v9 )
          {
            case 'c':
              v17 = 2;
              break;
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
          *(int *)(v14 + 56) = v17;
          if ( v30 == 116 )
          {
            if ( (unsigned char)sub_21DE0(v14 + 64, *(long long *)(a2 + 8LL * *a3), &qword_38638) )
            {
              v18 = *(long long *)(v14 + 64);
LABEL_27:
              v20 = timer;
              *(int *)(v14 + 60) = 0;
              v21 = 1008981770;
              v22 = (float)(int)((v20 - v18) / 86400);
              if ( v22 >= 0.1 )
              {
                v21 = 1050253722;
                if ( v22 >= 1.0 && v22 <= 100.0 )
                  v21 = 1053273620;
              }
              v23 = *(bool (**)(long long, long long *, long long))v14;
              *(int *)(v14 + 36) = v21;
              ++*a3;
              if ( !v23 )
                __assert_fail("our_pred->pred_func != NULL", "parser.c", 0x66Fu, "parse_newerXY");
              if ( v23 != sub_9400 )
                __assert_fail("our_pred->pred_func == pred_newerXY", "parser.c", 0x670u, "parse_newerXY");
              v8 = *(unsigned char *)(v14 + 26);
              if ( !(char)v8 )
                __assert_fail("our_pred->need_stat", "parser.c", 0x671u, "parse_newerXY");
              return v8;
            }
            v26 = sub_239F0(0, (unsigned int)dword_3868C, *(long long *)(a2 + 8LL * *a3));
            v27 = dcgettext(0, "I cannot figure out how to interpret %s as a date or time", 5);
            error(1, 0, v27, v26);
          }
          else
          {
            sub_D810(v31);
            if ( !(unsigned int)qword_38670(*(long long *)(a2 + 8LL * *a3), v31) )
            {
              if ( v30 == 99 )
              {
                v18 = v36;
                v19 = v37;
                goto LABEL_26;
              }
              if ( v30 > 99 )
              {
                if ( v30 == 109 )
                {
                  v18 = v34;
                  v19 = v35;
                  goto LABEL_26;
                }
              }
              else if ( v30 == 97 )
              {
                v18 = v32;
                v19 = v33;
LABEL_26:
                *(long long *)(v14 + 64) = v18;
                *(long long *)(v14 + 72) = v19;
                goto LABEL_27;
              }
              __assert_fail("0", "parser.c", 0x1F2u, "get_stat_Ytime");
            }
          }
          v28 = __errno_location();
          sub_E920(*v28, *(long long *)(a2 + 8LL * *a3));
        }
        v24 = sub_239F0(0, (unsigned int)dword_3868C, v6);
        v25 = dcgettext(0, "The %s test needs an argument", 5);
        error(1, 0, v25, v24);
LABEL_43:
        __assert_fail("0 == strncmp (\"-newer\", argv[*arg_ptr], 6)", "parser.c", 0x613u, "parse_newerXY");
      }
    }
  }
  return v8;
}



// Function: parse_ls @ 0x13b00
long long parse_ls(long long a1)
{
  long long *v1; // rbx

  v1 = sub_D520(a1);
  sub_11C90("/dev/stdout", (long long)(v1 + 7));
  *((int *)v1 + 9) = 1065353216;
  *((short *)v1 + 12) = 257;
  return 1;
}



// Function: parse_print0 @ 0x13b40
long long parse_print0(long long a1)
{
  long long v1; // rbx

  v1 = sub_D4D0(a1, 0);
  sub_11C90("/dev/stdout", v1 + 56);
  *(int *)(v1 + 24) = 257;
  *(int *)(v1 + 36) = 1065353216;
  return 1;
}



// Function: parse_follow @ 0x13b80
long long parse_follow(long long a1, long long a2)
{
  byte_3862C = 1;
  qword_38680 = (long long (*)(long long, long long, long long))sub_12DC0;
  dword_38668 = 1;
  if ( (qword_38660 & 2) != 0 )
    qword_38670 = (long long (*)(long long, long long))sub_DEC0;
  else
    qword_38670 = (long long (*)(long long, long long))sub_DDB0;
  return sub_133B0(a1, a2);
}



// Function: get_relative_timestamp @ 0x13be0
long long get_relative_timestamp(char *a1, long long a2, long long a3, int a4, const char *a5, double a6)
{
  char *v8; // r12
  unsigned int v10; // r15d
  double v12; // xmm0_8
  double v13; // xmm2_8
  long long v14; // rax
  long long v15; // rdx
  double v16; // [rsp+18h] [rbp-50h] BYREF
  double iptr[9]; // [rsp+20h] [rbp-48h] BYREF

  v8 = a1;
  *(long long *)&iptr[1] = __readfsqword(0x28u);
  if ( *a1 == 43 )
  {
    *(int *)(a2 + 4) = 1;
    v8 = a1 + 1;
  }
  else if ( *a1 == 45 )
  {
    *(int *)(a2 + 4) = 0;
    v8 = a1 + 1;
  }
  else
  {
    *(int *)(a2 + 4) = 2;
  }
  v10 = sub_25F90(v8, 0, &v16, &strtod);
  if ( !(char)v10 )
    return v10;
  v12 = modf(a6 * v16, iptr) * 1000000000.0;
  if ( v12 >= 1000000000.0 )
    __assert_fail("nanosec < nanosec_per_sec", "parser.c", 0xC4Du, "get_relative_timestamp");
  v13 = iptr[0];
  v14 = (unsigned int)(int)((double)(int)a3 - iptr[0]);
  *(long long *)(a2 + 8) = v14;
  if ( v14 > a3 == v13 < 0.0 )
  {
    v15 = (unsigned int)(int)((double)a4 - v12);
    if ( v12 <= (double)a4 )
    {
      *(long long *)(a2 + 16) = v15;
    }
    else
    {
      *(long long *)(a2 + 16) = v15 + 1000000000;
      *(long long *)(a2 + 8) = v14 - 1;
    }
    return v10;
  }
  error(1, 0, a5, v8);
  return sub_13D80();
}



// Function: parse_used @ 0x13d80
long long parse_used(long long a1, long long a2, int *a3)
{
  char *v3; // rbp
  unsigned int v4; // r12d
  long long v5; // rax
  long long v6; // rsi
  int v7; // xmm1_4
  float v8; // xmm0_4
  char *v10; // rax
  __m128i v11; // [rsp+0h] [rbp-38h] BYREF
  long long v12; // [rsp+10h] [rbp-28h]
  unsigned long long v13; // [rsp+18h] [rbp-20h]

  v13 = __readfsqword(0x28u);
  if ( !a2 )
    return 0;
  v3 = *(char **)(a2 + 8LL * *a3);
  if ( !v3 )
    return 0;
  ++*a3;
  v4 = sub_13BE0(
         v3,
         (long long)&v11,
         0,
         0,
         "arithmetic overflow while converting %s days to a number of seconds",
         86400.0);
  if ( (char)v4 )
  {
    v5 = sub_D4D0(a1, v3);
    v6 = v11.m128i_i64[1];
    v7 = 1008981770;
    *(__m128i *)(v5 + 56) = _mm_load_si128(&v11);
    *(long long *)(v5 + 72) = v12;
    v8 = (float)(int)(v6 / 86400);
    if ( v8 >= 0.1 )
    {
      v7 = 1050253722;
      if ( v8 >= 1.0 && v8 <= 100.0 )
        v7 = 1053273620;
    }
    *(int *)(v5 + 36) = v7;
    return v4;
  }
  v10 = dcgettext(0, "Invalid argument %s to -used", 5);
  error(1, 0, v10, v3);
  return sub_13ED0();
}



// Function: insert_num @ 0x13ed0
long long insert_num(long long a1, int *a2, long long a3)
{
  long long v4; // r12
  char *v5; // rdi
  int v6; // ebx
  long long v7; // rax
  const char *v9; // r8
  const char *v10; // rcx
  long long v11; // [rsp+8h] [rbp-30h]

  if ( !a1 )
    return 0;
  v4 = *(long long *)(a1 + 8LL * *a2);
  if ( !v4 )
    return v4;
  ++*a2;
  if ( *(char *)v4 == 43 )
  {
    v5 = (char *)(v4 + 1);
    v6 = 0;
  }
  else if ( *(char *)v4 == 45 )
  {
    v5 = (char *)(v4 + 1);
    v6 = 1;
  }
  else
  {
    v5 = (char *)v4;
    v6 = 2;
  }
  if ( (unsigned int)sub_26490(v5) )
    return 0;
  v7 = sub_D4D0(a3, v4);
  *(int *)(v7 + 56) = v6;
  v4 = v7;
  *(long long *)(v7 + 64) = v11;
  if ( (qword_38660 & 1) != 0 )
  {
    __fprintf_chk(stderr, 1, "inserting %s\n", *(const char **)(v7 + 8));
    v9 = " >";
    v10 = "gt";
    if ( v6 )
    {
      v9 = " =";
      if ( v6 == 1 )
        v9 = " <";
      v10 = "eq";
      if ( v6 == 1 )
        v10 = "lt";
    }
    __fprintf_chk(stderr, 1, "    type: %s    %s  ", v10, v9);
    __fprintf_chk(stderr, 1, "%lu\n", *(long long *)(v4 + 64));
  }
  return v4;
}



// Function: parse_links @ 0x14060
long long parse_links(long long a1, long long a2, int *a3)
{
  long long v4; // rax
  long long v5; // rdx

  v4 = sub_13ED0(a2, a3, a1);
  if ( v4 )
  {
    v5 = *(long long *)(v4 + 64);
    if ( v5 == 1 )
    {
      *(int *)(v4 + 36) = 1065185444;
      return 1;
    }
    else
    {
      if ( v5 == 2 )
        *(int *)(v4 + 36) = 1008981770;
      else
        *(int *)(v4 + 36) = 981668463;
      return 1;
    }
  }
  else
  {
    --*a3;
    return 0;
  }
}



// Function: parse_inum @ 0x140d0
long long parse_inum(long long a1, long long a2, int *a3)
{
  long long v4; // rax

  v4 = sub_13ED0(a2, a3, a1);
  if ( v4 )
  {
    *(int *)(v4 + 36) = 897988541;
    *(short *)(v4 + 26) = 0;
    *(char *)(v4 + 28) = 1;
    return 1;
  }
  else
  {
    --*a3;
    return 0;
  }
}



// Function: parse_newer @ 0x141e0
long long parse_newer(long long a1, long long a2, int *a3)
{
  long long v5; // rdx
  long long v6; // r13
  long long v7; // rdi
  long long v8; // rax
  time_t v9; // rcx
  long long v10; // rsi
  long long v11; // rax
  int v12; // xmm2_4
  float v13; // xmm0_4
  int *v15; // rax
  long long v16[25]; // [rsp+0h] [rbp-C8h] BYREF

  v16[19] = __readfsqword(0x28u);
  sub_D810(v16);
  if ( !a2 )
    return 0;
  v5 = *a3;
  v6 = *(long long *)(a2 + 8 * v5);
  if ( !v6 )
    return 0;
  v7 = *(long long *)(a2 + 8 * v5);
  ++*a3;
  if ( (unsigned int)qword_38670(v7, v16) )
  {
    v15 = __errno_location();
    sub_E920(*v15, v6);
  }
  v8 = sub_D4D0(a1, v6);
  v9 = timer;
  v10 = v8;
  v11 = v16[11];
  *(long long *)(v10 + 72) = v16[12];
  v12 = 1008981770;
  *(long long *)(v10 + 64) = v11;
  *(long long *)(v10 + 56) = 3;
  v13 = (float)(int)((v9 - v11) / 86400);
  if ( v13 >= 0.1 )
  {
    v12 = 1050253722;
    if ( v13 >= 1.0 && v13 <= 100.0 )
      v12 = 1053273620;
  }
  *(int *)(v10 + 36) = v12;
  return 1;
}



// Function: parse_anewer @ 0x14340
long long parse_anewer(long long a1, long long a2, int *a3)
{
  long long v5; // rdx
  long long v6; // r13
  long long v7; // rdi
  long long v8; // rax
  time_t v9; // rcx
  long long v10; // rsi
  long long v11; // rax
  int v12; // xmm2_4
  float v13; // xmm0_4
  int *v15; // rax
  long long v16[25]; // [rsp+0h] [rbp-C8h] BYREF

  v16[19] = __readfsqword(0x28u);
  sub_D810(v16);
  if ( !a2 )
    return 0;
  v5 = *a3;
  v6 = *(long long *)(a2 + 8 * v5);
  if ( !v6 )
    return 0;
  v7 = *(long long *)(a2 + 8 * v5);
  ++*a3;
  if ( (unsigned int)qword_38670(v7, v16) )
  {
    v15 = __errno_location();
    sub_E920(*v15, v6);
  }
  v8 = sub_D4D0(a1, v6);
  v9 = timer;
  v10 = v8;
  v11 = v16[11];
  *(long long *)(v10 + 72) = v16[12];
  v12 = 1008981770;
  *(long long *)(v10 + 64) = v11;
  *(long long *)(v10 + 56) = 0;
  v13 = (float)(int)((v9 - v11) / 86400);
  if ( v13 >= 0.1 )
  {
    v12 = 1050253722;
    if ( v13 >= 1.0 && v13 <= 100.0 )
      v12 = 1053273620;
  }
  *(int *)(v10 + 36) = v12;
  return 1;
}



// Function: parse_cnewer @ 0x144a0
long long parse_cnewer(long long a1, long long a2, int *a3)
{
  long long v5; // rdx
  long long v6; // r13
  long long v7; // rdi
  long long v8; // rax
  time_t v9; // rcx
  long long v10; // rsi
  long long v11; // rax
  int v12; // xmm2_4
  float v13; // xmm0_4
  int *v15; // rax
  long long v16[25]; // [rsp+0h] [rbp-C8h] BYREF

  v16[19] = __readfsqword(0x28u);
  sub_D810(v16);
  if ( !a2 )
    return 0;
  v5 = *a3;
  v6 = *(long long *)(a2 + 8 * v5);
  if ( !v6 )
    return 0;
  v7 = *(long long *)(a2 + 8 * v5);
  ++*a3;
  if ( (unsigned int)qword_38670(v7, v16) )
  {
    v15 = __errno_location();
    sub_E920(*v15, v6);
  }
  v8 = sub_D4D0(a1, v6);
  v9 = timer;
  v10 = v8;
  v11 = v16[11];
  *(long long *)(v10 + 72) = v16[12];
  v12 = 1008981770;
  *(long long *)(v10 + 64) = v11;
  *(long long *)(v10 + 56) = 2;
  v13 = (float)(int)((v9 - v11) / 86400);
  if ( v13 >= 0.1 )
  {
    v12 = 1050253722;
    if ( v13 >= 1.0 && v13 <= 100.0 )
      v12 = 1053273620;
  }
  *(int *)(v10 + 36) = v12;
  return 1;
}



// Function: do_parse_xmin @ 0x14600
long long do_parse_xmin(long long a1, long long a2, int *a3, int a4)
{
  long long v4; // rax
  unsigned int v6; // r12d
  char *v7; // r14
  int v8; // r13d
  int v9; // ecx
  long long v11; // rsi
  time_t v12; // rcx
  int v13; // xmm1_4
  float v14; // xmm0_4
  __m128i v15; // [rsp+0h] [rbp-48h] BYREF
  long long v16; // [rsp+10h] [rbp-38h]
  unsigned long long v17; // [rsp+18h] [rbp-30h]

  v17 = __readfsqword(0x28u);
  if ( a2 )
  {
    v4 = *a3;
    v6 = 0;
    v7 = *(char **)(a2 + 8 * v4);
    v8 = *a3;
    if ( v7 )
    {
      v15.m128i_i32[0] = a4;
      v9 = qword_38650;
      *a3 = v4 + 1;
      v6 = sub_13BE0(
             v7,
             (long long)&v15,
             timer + 86400,
             v9,
             "arithmetic overflow while converting %s minutes to a number of seconds",
             60.0);
      if ( (char)v6 )
      {
        v11 = sub_D4D0(a1, v7);
        v12 = timer - v15.m128i_i64[1];
        *(__m128i *)(v11 + 56) = _mm_load_si128(&v15);
        v13 = 1008981770;
        *(long long *)(v11 + 72) = v16;
        v14 = (float)(int)(v12 / 86400);
        if ( v14 >= 0.1 )
        {
          v13 = 1050253722;
          if ( v14 >= 1.0 && v14 <= 100.0 )
            v13 = 1053273620;
        }
        *(int *)(v11 + 36) = v13;
      }
      else
      {
        *a3 = v8;
      }
    }
  }
  else
  {
    return 0;
  }
  return v6;
}



// Function: parse_mmin @ 0x14750
long long parse_mmin(long long a1, long long a2, int *a3)
{
  return sub_14600(a1, a2, a3, 3);
}



// Function: parse_time @ 0x14780
long long parse_time(long long a1, long long a2, int *a3)
{
  char *v5; // rax
  const char *v6; // r8
  long long v7; // rax
  unsigned int v8; // r12d
  char *v9; // r14
  int v10; // r15d
  long long v11; // rdx
  char *v12; // rdi
  long long v14; // rbx
  time_t v15; // rcx
  int v16; // xmm1_4
  float v17; // xmm0_4
  const char *v18; // r8
  char *v19; // rcx
  char *v20; // rax
  char *v21; // rax
  char *v22; // rax
  time_t timer; // [rsp+8h] [rbp-60h] BYREF
  __m128i v24; // [rsp+10h] [rbp-58h] BYREF
  long long v25; // [rsp+20h] [rbp-48h]
  unsigned long long v26; // [rsp+28h] [rbp-40h]

  v26 = __readfsqword(0x28u);
  v5 = dcgettext(0, "arithmetic overflow while converting %s days to a number of seconds", 5);
  if ( a2 )
  {
    v6 = v5;
    v7 = *a3;
    v8 = 0;
    v9 = *(char **)(a2 + 8 * v7);
    v10 = *a3;
    if ( v9 )
    {
      v11 = ::timer;
      v12 = *(char **)(a2 + 8 * v7);
      *a3 = v7 + 1;
      if ( *v9 == 45 )
        v11 += (long long)&loc_1517F;
      v8 = sub_13BE0(v12, (long long)&v24, v11, qword_38650, v6, 86400.0);
      if ( (char)v8 )
      {
        v14 = sub_D4D0(a1, v9);
        v15 = ::timer - v24.m128i_i64[1];
        *(__m128i *)(v14 + 56) = _mm_load_si128(&v24);
        v16 = 1008981770;
        *(long long *)(v14 + 72) = v25;
        v17 = (float)(int)(v15 / 86400);
        if ( v17 >= 0.1 )
        {
          v16 = 1050253722;
          if ( v17 >= 1.0 && v17 <= 100.0 )
            v16 = 1053273620;
        }
        *(int *)(v14 + 36) = v16;
        if ( (qword_38660 & 1) != 0 )
        {
          __fprintf_chk(stderr, 1, "inserting %s\n", *(const char **)(v14 + 8));
          if ( v24.m128i_i32[1] )
          {
            if ( v24.m128i_i32[1] == 1 )
            {
              v18 = " <";
              v19 = (char *)"lt";
            }
            else
            {
              v18 = " ?";
              v19 = "?";
              if ( v24.m128i_i32[1] == 2 )
              {
                __fprintf_chk(stderr, 1, "    type: %s    %s  ", "eq", ">=");
                timer = *(long long *)(v14 + 64);
                v21 = ctime(&timer);
                __fprintf_chk(stderr, 1, "%lu %s", *(long long *)(v14 + 64), v21);
                timer = *(long long *)(v14 + 64) + 86400LL;
                v22 = ctime(&timer);
                __fprintf_chk(stderr, 1, "                 <  %lu %s", timer, v22);
                return v8;
              }
            }
            __fprintf_chk(stderr, 1, "    type: %s    %s  ", v19, v18);
          }
          else
          {
            __fprintf_chk(stderr, 1, "    type: %s    %s  ", "gt", " >");
          }
          timer = *(long long *)(v14 + 64);
          v20 = ctime(&timer);
          __fprintf_chk(stderr, 1, "%lu %s", *(long long *)(v14 + 64), v20);
        }
      }
      else
      {
        *a3 = v10;
      }
    }
  }
  else
  {
    return 0;
  }
  return v8;
}



// Function: check_option_combinations @ 0x14a70
void check_option_combinations(long long a1)
{
  int v1; // eax
  long long (*v2)(); // rdx
  char *v3; // rax

  if ( a1 )
  {
    v1 = 0;
    do
    {
      while ( 1 )
      {
        v2 = *(long long (**)())a1;
        if ( *(long long (**)(long long, long long))a1 != sub_8D00 )
          break;
        a1 = *(long long *)(a1 + 264);
        v1 |= 1u;
        if ( !a1 )
          goto LABEL_8;
      }
      a1 = *(long long *)(a1 + 264);
      if ( v2 == sub_8600 )
        v1 |= 2u;
    }
    while ( a1 );
LABEL_8:
    if ( v1 == 3 && !HIBYTE(word_38620) )
    {
      v3 = dcgettext(
             (const char *)a1,
             "The -delete action automatically turns on -depth, but -prune does nothing when -depth is in effect.  If you"
             " want to carry on anyway, just explicitly use the -depth option.",
             5);
      error(1, 0, v3);
      sub_14AF0(1);
    }
  }
}



// Function: parse_begin_user_args @ 0x14ba0
void parse_begin_user_args()
{
  qword_38E70 = 0;
}



// Function: parse_end_user_args @ 0x14bb0
void parse_end_user_args()
{
  ;
}



// Function: find_parser @ 0x14bc0
long long find_parser(char *s)
{
  char *v2; // rsi
  const char **v3; // rbx
  const char *v4; // rdi
  int v5; // ebp
  const char *v6; // r13

  if ( !strncmp("-newer", s, 6u) )
  {
    v2 = (char *)&unk_37840;
    if ( strlen(s) == 8 )
      return sub_12CA0(s, v2);
  }
  v3 = (const char **)&off_36D88;
  v4 = "!";
  v5 = 0;
  v6 = &s[*s == 45];
  do
  {
    if ( !strcmp(v4, v6) )
    {
      v2 = (char *)&unk_36D60 + 32 * v5;
      return sub_12CA0(s, v2);
    }
    v4 = *v3;
    v3 += 4;
    ++v5;
  }
  while ( v4 );
  return 0;
}



// Function: parse_fstype @ 0x14c70
long long parse_fstype(long long a1, long long a2, int *a3)
{
  unsigned int v3; // r8d
  char *v4; // rbp
  bool v5; // cc
  long long v6; // rax
  long long *v7; // rax
  long long v9; // rax
  long long v10; // r12
  int v11; // xmm0_4
  const char *v12; // rax
  struct stat v13; // [rsp+0h] [rbp-C8h] BYREF
  unsigned long long v14; // [rsp+98h] [rbp-30h]

  v14 = __readfsqword(0x28u);
  if ( a2 )
  {
    v3 = 0;
    v4 = *(char **)(a2 + 8LL * *a3);
    if ( v4 )
    {
      v5 = (unsigned short)word_38688 <= 1u;
      ++*a3;
      if ( v5 || (unsigned char)sub_10A50(v4) )
      {
        v9 = sub_D4D0(a1, v4);
        *(long long *)(v9 + 56) = v4;
        v10 = v9;
        v11 = 1065353216;
        if ( !stat("/", &v13) )
        {
          v12 = (const char *)sub_108E0((long long *)&v13);
          v11 = 1060320051;
          if ( strcmp(v4, v12) )
            v11 = 1050253722;
        }
        *(int *)(v10 + 36) = v11;
        return 1;
      }
      else
      {
        if ( (qword_38660 & 8) != 0 )
          __fprintf_chk(stderr, 1, "-fstype %s can never succeed, substituting -false\n", v4);
        v6 = sub_14BC0("false");
        v7 = sub_D520(v6);
        v3 = 1;
        *((int *)v7 + 6) = 0;
        *((int *)v7 + 9) = 0;
      }
    }
  }
  else
  {
    return 0;
  }
  return v3;
}



// Function: parse_false @ 0x14df0
long long parse_false()
{
  long long v0; // rax
  long long *v1; // rax

  v0 = sub_14BC0("false");
  v1 = sub_D520(v0);
  *((int *)v1 + 6) = 0;
  *((int *)v1 + 9) = 0;
  return 1;
}


