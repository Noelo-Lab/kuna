// Function: left_dir @ 0x73b9
int left_dir()
{
  int result; // eax

  result = dword_49010 & 0x200;
  if ( (dword_49010 & 0x200) != 0 )
  {
    result = fd;
    if ( fd >= 0 )
    {
      result = close(fd);
      fd = -1;
    }
  }
  return result;
}



// Function: inside_dir @ 0x73f4
long long inside_dir(int a1)
{
  long long result; // rax

  result = dword_49010 & 0x200;
  if ( (dword_49010 & 0x200) != 0 )
  {
    if ( a1 != -100 && a1 < 0 )
      __assert_fail("dir_fd == AT_FDCWD || dir_fd >= 0", "ftsfind.c", 0x6Bu, "inside_dir");
    dword_499D8 = a1;
    result = (unsigned int)fd;
    if ( fd < 0 )
    {
      if ( a1 == -100 )
      {
        fd = -100;
      }
      else
      {
        result = sub_1D639((unsigned int)a1);
        fd = result;
      }
    }
  }
  return result;
}



// Function: get_fts_info_name @ 0x74c2
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
      sprintf(s, "[%d]", a1);
      result = s;
      break;
  }
  return result;
}



// Function: visit @ 0x75b5
long long visit(long long a1, long long a2, long long a3)
{
  bool v3; // al
  long long result; // rax
  long long v6; // [rsp+28h] [rbp-8h]

  v3 = *(short *)(a2 + 104) != 10 && *(short *)(a2 + 104) != 11;
  byte_499C4 = v3;
  s1 = *(char **)(a2 + 48);
  dword_499D8 = *(int *)(a1 + 44);
  v6 = sub_D7FF();
  sub_10873(*(long long *)(a2 + 56), a3, v6);
  result = (unsigned char)byte_499E0;
  if ( byte_499E0 )
    return sub_1FC0F(a1, a2, 4);
  return result;
}



// Function: partial_quotearg_n @ 0x7664
long long partial_quotearg_n(unsigned int a1, long long a2, long long a3, unsigned int a4)
{
  long long result; // rax
  char v6; // [rsp+27h] [rbp-9h]

  if ( !a3 )
    return sub_2F861(a1, a4, locale);
  v6 = *(char *)(a2 + a3);
  *(char *)(a2 + a3) = 0;
  result = sub_2F861(a1, a4, a2);
  *(char *)(a3 + a2) = v6;
  return result;
}



// Function: issue_loop_warning @ 0x76ea
void issue_loop_warning(long long a1)
{
  long long v1; // rbx
  char *v2; // rax
  long long v3; // r12
  long long v4; // rbx
  char *v5; // rax

  if ( (*(int *)(a1 + 136) & 0xF000) == 0xA000 )
  {
    v1 = sub_1096A(0, *(long long *)(a1 + 56));
    v2 = gettext(
           "Symbolic link %s is part of a loop in the directory hierarchy; we have already visited the directory to which it points.");
    error(0, 0, v2, v1);
  }
  else
  {
    v3 = sub_7664(1u, *(long long *)(*(long long *)a1 + 56LL), *(long long *)(*(long long *)a1 + 72LL), dword_499AC);
    v4 = sub_1096A(0, *(long long *)(a1 + 56));
    v5 = gettext("File system loop detected; %s is part of the same file system loop as %s.");
    error(0, 0, v5, v4, v3);
  }
}



// Function: symlink_loop @ 0x77d0
long long symlink_loop(long long a1)
{
  char v3[152]; // [rsp+20h] [rbp-A0h] BYREF
  unsigned long long v4; // [rsp+B8h] [rbp-8h]

  v4 = __readfsqword(0x28u);
  return (unsigned int)qword_49990(a1, v3) && *__errno_location() == 40;
}



// Function: consider_visiting @ 0x7852
unsigned long long consider_visiting(long long a1, long long a2)
{
  const char *v2; // r12
  const char *v3; // rbx
  int v4; // r14d
  int v5; // r15d
  const char *v6; // r13
  const char *v7; // rax
  long long v8; // rbx
  long long v9; // rbx
  long long v10; // rbx
  long long v11; // rbx
  long long v12; // rbx
  long long v13; // rbx
  long long v14; // rbx
  long long v15; // rbx
  long long v16; // rdx
  long long v17; // rbx
  char *v18; // rax
  int v19; // eax
  int v20; // r12d
  int v21; // ebx
  const char *v22; // r13
  const char *v23; // rax
  int v25; // [rsp+14h] [rbp-DCh] BYREF
  int v26; // [rsp+18h] [rbp-D8h]
  int v27; // [rsp+1Ch] [rbp-D4h]
  long long v28; // [rsp+20h] [rbp-D0h] BYREF
  long long v29; // [rsp+28h] [rbp-C8h]
  long long v30; // [rsp+30h] [rbp-C0h]
  long long v31; // [rsp+38h] [rbp-B8h]
  long long v32; // [rsp+40h] [rbp-B0h]
  long long v33; // [rsp+48h] [rbp-A8h]
  long long v34; // [rsp+50h] [rbp-A0h]
  long long v35; // [rsp+58h] [rbp-98h]
  long long v36; // [rsp+60h] [rbp-90h]
  long long v37; // [rsp+68h] [rbp-88h]
  long long v38; // [rsp+70h] [rbp-80h]
  long long v39; // [rsp+78h] [rbp-78h]
  long long v40; // [rsp+80h] [rbp-70h]
  long long v41; // [rsp+88h] [rbp-68h]
  long long v42; // [rsp+90h] [rbp-60h]
  long long v43; // [rsp+98h] [rbp-58h]
  long long v44; // [rsp+A0h] [rbp-50h]
  long long v45; // [rsp+A8h] [rbp-48h]
  unsigned long long v46; // [rsp+B8h] [rbp-38h]

  v46 = __readfsqword(0x28u);
  if ( (qword_49980 & 4) != 0 )
  {
    v2 = (const char *)sub_2F861(2, (unsigned int)dword_499AC, *(long long *)(a2 + 48));
    v3 = (const char *)sub_2F861(1, (unsigned int)dword_499AC, *(long long *)(a2 + 56));
    v4 = dword_49014;
    v5 = *(long long *)(a2 + 88);
    v6 = sub_74C2(*(unsigned short *)(a2 + 104));
    v7 = (const char *)sub_2F861(0, (unsigned int)dword_499AC, *(long long *)(a2 + 56));
    fprintf(
      stderr,
      "consider_visiting (early): %s: fts_info=%-6s, fts_level=%2d, prev_depth=%d fts_path=%s, fts_accpath=%s\n",
      v7,
      v6,
      v5,
      v4,
      v3,
      v2);
  }
  if ( *(short *)(a2 + 104) == 6 || *(long long *)(a2 + 88) > dword_49014 || !*(long long *)(a2 + 88) )
    sub_73B9();
  sub_73F4(*(int *)(a1 + 44));
  dword_49014 = *(long long *)(a2 + 88);
  v29 = *(long long *)(a2 + 120);
  switch ( *(short *)(a2 + 104) )
  {
    case 7:
      goto LABEL_8;
    case 4:
      sub_10A13(*(unsigned int *)(a2 + 64), *(long long *)(a2 + 56));
      if ( byte_49940 )
        return v46 - __readfsqword(0x28u);
      break;
    case 2:
      sub_76EA(a2);
      status = 1;
      return v46 - __readfsqword(0x28u);
    case 0xD:
      if ( sub_77D0(*(long long *)(a2 + 48)) )
        goto LABEL_16;
      break;
    case 0xA:
      if ( !*(long long *)(a2 + 88) )
      {
LABEL_8:
        sub_10A13(*(unsigned int *)(a2 + 64), *(long long *)(a2 + 56));
        return v46 - __readfsqword(0x28u);
      }
      if ( sub_77D0(*(long long *)(a2 + 48)) )
      {
LABEL_16:
        sub_10A13(40, *(long long *)(a2 + 56));
        return v46 - __readfsqword(0x28u);
      }
      sub_10A13(*(unsigned int *)(a2 + 64), *(long long *)(a2 + 56));
      break;
  }
  if ( *(short *)(a2 + 104) == 11 || *(short *)(a2 + 104) == 10 )
  {
    if ( byte_499C4 == 1 )
      __assert_fail("!state.have_stat", "ftsfind.c", 0x17Cu, "consider_visiting");
    if ( *(short *)(a2 + 104) != 11 && dword_499C8 )
      __assert_fail("ent->fts_info == FTS_NSOK || state.type == 0", "ftsfind.c", 0x17Du, "consider_visiting");
    v25 = dword_499C8;
  }
  else
  {
    byte_499C4 = 1;
    byte_499C5 = 1;
    v8 = *(long long *)(a2 + 120);
    v28 = *(long long *)(a2 + 112);
    v29 = v8;
    v9 = *(long long *)(a2 + 136);
    v30 = *(long long *)(a2 + 128);
    v31 = v9;
    v10 = *(long long *)(a2 + 152);
    v32 = *(long long *)(a2 + 144);
    v33 = v10;
    v11 = *(long long *)(a2 + 168);
    v34 = *(long long *)(a2 + 160);
    v35 = v11;
    v12 = *(long long *)(a2 + 184);
    v36 = *(long long *)(a2 + 176);
    v37 = v12;
    v13 = *(long long *)(a2 + 200);
    v38 = *(long long *)(a2 + 192);
    v39 = v13;
    v14 = *(long long *)(a2 + 216);
    v40 = *(long long *)(a2 + 208);
    v41 = v14;
    v15 = *(long long *)(a2 + 232);
    v42 = *(long long *)(a2 + 224);
    v43 = v15;
    v16 = *(long long *)(a2 + 248);
    v44 = *(long long *)(a2 + 240);
    v45 = v16;
    v25 = v31;
    dword_499C8 = v31;
    if ( !(int)v31 )
    {
      v17 = sub_2F861(0, (unsigned int)dword_499AC, *(long long *)(a2 + 56));
      v18 = gettext("WARNING: file %s appears to have mode 0000");
      error(0, 0, v18, v17);
    }
  }
  dword_499C0 = *(long long *)(a2 + 88);
  if ( !v25 || (unsigned char)sub_FE38(&v25, *(long long *)(a2 + 56), a2 + 256, &v28, 0) == 1 )
  {
    v26 = 0;
    v19 = (v25 & 0xF000) == 0x4000
       || *(short *)(a2 + 104) == 1
       || *(short *)(a2 + 104) == 6
       || *(short *)(a2 + 104) == 2;
    v27 = v19;
    if ( v19 && *(short *)(a2 + 104) == 11 )
    {
      sub_1FC0F(a1, a2, 1);
    }
    else
    {
      if ( dword_49944 >= 0 && *(long long *)(a2 + 88) >= dword_49944 )
      {
        sub_1FC0F(a1, a2, 4);
        if ( *(long long *)(a2 + 88) > dword_49944 )
          v26 = 1;
      }
      if ( *(short *)(a2 + 104) == 1 && byte_49940 != 1 )
      {
        v26 = 1;
      }
      else if ( *(short *)(a2 + 104) == 6 && byte_49940 )
      {
        v26 = 1;
      }
      else if ( *(long long *)(a2 + 88) < dword_49948 )
      {
        v26 = 1;
      }
      if ( (qword_49980 & 4) != 0 )
      {
        v20 = (unsigned char)byte_499C5;
        v21 = (unsigned char)byte_499C4;
        v22 = sub_74C2(*(unsigned short *)(a2 + 104));
        v23 = (const char *)sub_2F861(0, (unsigned int)dword_499AC, *(long long *)(a2 + 56));
        fprintf(
          stderr,
          "consider_visiting (late): %s: fts_info=%-6s, isdir=%d ignore=%d have_stat=%d have_type=%d \n",
          v23,
          v22,
          v27,
          v26,
          v21,
          v20);
      }
      if ( !v26 )
        sub_75B5(a1, a2, (long long)&v28);
      if ( *(short *)(a2 + 104) == 6 )
        byte_499E0 = 0;
    }
  }
  return v46 - __readfsqword(0x28u);
}



// Function: find @ 0x803a
long long find(const char *a1)
{
  long long v1; // r12
  char *v2; // rbx
  int *v3; // rax
  int v4; // eax
  const char *v5; // rbx
  int *v6; // rax
  char *v8; // rbx
  int *v9; // rax
  unsigned int v10; // [rsp+1Ch] [rbp-44h]
  long long v11; // [rsp+20h] [rbp-40h]
  long long v12; // [rsp+28h] [rbp-38h]
  long long v13[6]; // [rsp+30h] [rbp-30h] BYREF

  v13[3] = __readfsqword(0x28u);
  dword_499DC = strlen(a1);
  sub_73F4(-100);
  v13[0] = a1;
  v13[1] = 0;
  if ( dword_49988 == 2 )
  {
    dword_49010 |= 0x11u;
  }
  else if ( (unsigned int)dword_49988 <= 2 )
  {
    if ( dword_49988 )
    {
      if ( dword_49988 == 1 )
        dword_49010 |= 3u;
    }
    else
    {
      dword_49010 |= 0x10u;
    }
  }
  if ( byte_4994D )
    dword_49010 |= 0x40u;
  v11 = sub_1EC55(v13, (unsigned int)dword_49010, 0);
  if ( v11 )
  {
    v10 = 0x80000000;
    while ( 1 )
    {
      *__errno_location() = 0;
      v12 = sub_1F599(v11);
      if ( !v12 )
        break;
      if ( byte_499E8 && v10 != (unsigned int)*(long long *)(v12 + 88) )
        sub_F6CB();
      v10 = *(long long *)(v12 + 88);
      byte_499F8 = 0;
      byte_499C4 = 0;
      byte_499C5 = *(int *)(v12 + 136) != 0;
      if ( byte_499C5 )
        v4 = *(int *)(v12 + 136);
      else
        v4 = 0;
      dword_499C8 = v4;
      sub_7852(v11, v12);
    }
    if ( *__errno_location() )
    {
      v5 = (const char *)sub_1096A(0, a1);
      v6 = __errno_location();
      error(0, *v6, "failed to read file names from file system at or below %s", v5);
      status = 1;
      return 0;
    }
    if ( (unsigned int)sub_1F0DF(v11) )
    {
      v8 = gettext("failed to restore working directory after searching %s");
      v9 = __errno_location();
      error(0, *v9, v8, a1);
      status = 1;
      return 0;
    }
  }
  else
  {
    v1 = sub_1096A(0, a1);
    v2 = gettext("cannot search %s");
    v3 = __errno_location();
    error(0, *v3, v2, v1);
    status = 1;
  }
  return 1;
}



// Function: process_all_startpoints @ 0x82e7
long long process_all_startpoints(int a1, long long *a2)
{
  bool v2; // al
  long long v3; // rbx
  char *v4; // rax
  char *v5; // rax
  char *v6; // rax
  char *v7; // rsi
  char *v8; // rbx
  int *v9; // rax
  char *v10; // rax
  char *v12; // rbx
  int *v13; // rax
  const char *v14; // rax
  char *v15; // rax
  char *v16; // rax
  unsigned char v17; // [rsp+1Eh] [rbp-172h]
  unsigned int v18; // [rsp+20h] [rbp-170h] BYREF
  int fd; // [rsp+24h] [rbp-16Ch]
  FILE *stream; // [rsp+28h] [rbp-168h]
  const char *v21; // [rsp+30h] [rbp-160h]
  long long v22; // [rsp+38h] [rbp-158h]
  char *s; // [rsp+40h] [rbp-150h]
  long long v24; // [rsp+48h] [rbp-148h]
  struct stat buf; // [rsp+50h] [rbp-140h] BYREF
  struct stat v26; // [rsp+E0h] [rbp-B0h] BYREF
  short v27; // [rsp+176h] [rbp-1Ah] BYREF
  unsigned long long v28; // [rsp+178h] [rbp-18h]

  v28 = __readfsqword(0x28u);
  v2 = a1 > 0 && (unsigned char)sub_FF96(*a2, 1) != 1;
  stream = 0;
  v21 = 0;
  if ( filename )
  {
    if ( v2 )
    {
      v3 = sub_1096A(0, *a2);
      v4 = gettext("extra operand %s");
      error(0, 0, v4, v3);
      v5 = gettext("file operands cannot be combined with -files0-from");
      error(1, 0, "%s", v5);
    }
    if ( !strcmp(filename, "-") )
    {
      if ( byte_499B8 )
      {
        v6 = gettext("option -files0-from reading from standard input cannot be combined with -ok, -okdir");
        error(1, 0, "%s\n", v6);
      }
      v7 = gettext("(standard input)");
      v21 = (const char *)sub_1096A(0, v7);
      stream = stdin;
    }
    else
    {
      v21 = (const char *)sub_1096A(0, filename);
      stream = fopen(filename, "r");
      if ( !stream )
      {
        v8 = gettext("cannot open %s for reading");
        v9 = __errno_location();
        error(1, *v9, v8, v21);
      }
      fd = fileno(stream);
      if ( fd < 0 )
        __assert_fail("fd >= 0", "ftsfind.c", 0x262u, "process_all_startpoints");
      if ( byte_499B8 && !fstat(fd, &buf) && !fstat(0, &v26) && buf.st_ino == v26.st_ino && buf.st_dev == v26.st_dev )
      {
        v10 = gettext("option -files0-from: standard input must not refer to the same file when combined with -ok, -okdir");
        error(1, 0, "%s: %s\n", v10, v21);
      }
      sub_1D5BA((unsigned int)fd, 1);
    }
    v22 = sub_1D0FF(stream);
  }
  else
  {
    if ( !v2 )
    {
      v27 = 46;
      return sub_803A((const char *)&v27);
    }
    v22 = sub_1D0AA(a2);
  }
  if ( !v22 )
    sub_321FA();
  v17 = 1;
  while ( 1 )
  {
    s = (char *)sub_1D16C(v22, &v18);
    if ( !s )
      break;
    if ( *s )
    {
      if ( !filename && (unsigned char)sub_FF96(s, 1) )
        goto LABEL_48;
      dword_499DC = strlen(s);
      if ( (unsigned char)sub_803A(s) != 1 )
      {
        v17 = 0;
        goto LABEL_48;
      }
    }
    else
    {
      if ( filename )
      {
        v24 = sub_1D257(v22);
        v15 = gettext("invalid zero-length file name");
        error(0, 0, "%s:%lu: %s", v21, v24, v15);
      }
      else
      {
        v14 = (const char *)sub_1096A(0, s);
        error(0, 2, "%s", v14);
      }
      status = 1;
      v17 = 0;
    }
  }
  if ( v18 == 4 )
  {
    v12 = gettext("%s: read error");
    v13 = __errno_location();
    error(0, *v13, v12, v21);
    status = 1;
    v17 = 0;
  }
  else
  {
    if ( v18 > 4 )
      goto LABEL_38;
    if ( v18 != 2 )
    {
      if ( v18 == 3 )
        sub_321FA();
LABEL_38:
      __assert_fail("!\"unexpected error code from argv_iter\"", "ftsfind.c", 0x2A3u, "process_all_startpoints");
    }
  }
LABEL_48:
  sub_1D295(v22);
  if ( v17 && filename && (ferror(stream) || (unsigned int)sub_1DAD5(stream)) )
  {
    v16 = gettext("error reading %s");
    error(1, 0, v16, v21);
  }
  return v17;
}



// Function: main @ 0x88ed
long long main(unsigned int a1, char **a2, char **a3)
{
  char *v3; // rbx
  int *v4; // rax
  char *v5; // rbx
  int *v6; // rax
  char *v7; // rax
  unsigned int v9; // [rsp+14h] [rbp-1Ch]
  long long v10; // [rsp+18h] [rbp-18h]

  if ( *a2 )
    sub_2DD35(*a2, a2, a3);
  else
    sub_2DD35("find", a2, a3);
  sub_F78B();
  byte_499F8 = 0;
  status = 0;
  byte_499E8 = 0;
  dword_499D8 = -100;
  if ( (unsigned char)sub_1B5F4() )
    sub_1B20A();
  qword_499F0 = sub_10BE9("w");
  if ( !qword_499F0 )
  {
    v3 = gettext("Failed to initialize shared-file hash table");
    v4 = __errno_location();
    error(1, *v4, v3);
  }
  sub_106BD(&byte_49940);
  setlocale(6, locale);
  bindtextdomain("findutils", "/usr/local/share/locale");
  textdomain("findutils");
  if ( (unsigned int)sub_37150(sub_1D697) )
  {
    v5 = gettext("The atexit library function failed");
    v6 = __errno_location();
    error(1, *v6, v5);
  }
  v9 = sub_103E8(a1, a2);
  if ( (qword_49980 & 2) != 0 )
    qword_49990 = (long long (*)(long long, long long))sub_FD39;
  if ( (qword_49980 & 0x80) != 0 )
  {
    v7 = ctime(&timer);
    fprintf(stderr, "cur_day_start = %s", v7);
  }
  v10 = sub_DD20(a1, a2, v9);
  if ( (unsigned char)sub_82E7(a1 - v9, &a2[v9]) )
  {
    sub_AE53(v10);
    sub_FA38();
  }
  return (unsigned int)status;
}



// Function: is_fts_enabled @ 0x8b12
long long is_fts_enabled(int *a1)
{
  *a1 = dword_49010;
  return 1;
}



// Function: ts_difference @ 0x8b31
double ts_difference(time_t a1, int a2, time_t a3, int a4)
{
  return difftime(a1, a3) + 0.000000001 * (double)(a2 - a4);
}



// Function: compare_ts @ 0x8ba9
long long compare_ts(time_t a1, long long a2, time_t a3, long long a4)
{
  if ( a1 == a3 && a2 == a4 )
    return 0;
  if ( sub_8B31(a1, a2, a3, a4) >= 0.0 )
    return 1;
  return 0xFFFFFFFFLL;
}



// Function: pred_timewindow @ 0x8c32
char pred_timewindow(time_t a1, long long a2, long long a3, int a4)
{
  unsigned int v4; // eax
  int v5; // eax
  bool v6; // al
  double v9; // [rsp+28h] [rbp-8h]

  v4 = *(int *)(a3 + 60);
  if ( v4 == 2 )
  {
    v9 = sub_8B31(a1, a2, *(long long *)(a3 + 64), *(long long *)(a3 + 72));
    v6 = v9 > 0.0 && (double)a4 >= v9;
    LOBYTE(v5) = v6;
  }
  else
  {
    if ( v4 > 2 )
      __assert_fail("0", "pred.c", 0x8Au, "pred_timewindow");
    if ( v4 )
      return (unsigned int)sub_8BA9(a1, a2, *(long long *)(a3 + 64), *(long long *)(a3 + 72)) >> 31;
    else
      LOBYTE(v5) = (int)sub_8BA9(a1, a2, *(long long *)(a3 + 64), *(long long *)(a3 + 72)) > 0;
  }
  return v5;
}



// Function: pred_amin @ 0x8d51
char pred_amin(long long a1, long long a2, long long a3)
{
  time_t v3; // rax
  long long v4; // rdx

  v3 = sub_30A4C(a2);
  return sub_8C32(v3, v4, a3, 60);
}



// Function: pred_and @ 0x8d9c
long long pred_and(long long a1, long long a2, long long a3)
{
  if ( *(long long *)(a3 + 272) && !(unsigned char)sub_10873(a1, a2, *(long long *)(a3 + 272)) )
    return 0;
  else
    return sub_10873(a1, a2, *(long long *)(a3 + 280));
}



// Function: pred_anewer @ 0x8e0d
bool pred_anewer(long long a1, long long a2, long long a3)
{
  time_t v3; // rax
  long long v4; // rdx

  if ( *(int *)(a3 + 60) )
    __assert_fail("COMP_GT == pred_ptr->args.reftime.kind", "pred.c", 0xA6u, "pred_anewer");
  v3 = sub_30A4C(a2);
  return (int)sub_8BA9(v3, v4, *(long long *)(a3 + 64), *(long long *)(a3 + 72)) > 0;
}



// Function: pred_atime @ 0x8e96
char pred_atime(long long a1, long long a2, long long a3)
{
  time_t v3; // rax
  long long v4; // rdx

  v3 = sub_30A4C(a2);
  return sub_8C32(v3, v4, a3, 86400);
}



// Function: pred_closeparen @ 0x8ee1
long long pred_closeparen()
{
  return 1;
}



// Function: pred_cmin @ 0x8efc
char pred_cmin(long long a1, long long a2, long long a3)
{
  time_t v3; // rax
  long long v4; // rdx

  v3 = sub_30A66(a2);
  return sub_8C32(v3, v4, a3, 60);
}



// Function: pred_cnewer @ 0x8f47
bool pred_cnewer(long long a1, long long a2, long long a3)
{
  time_t v3; // rax
  long long v4; // rdx

  if ( *(int *)(a3 + 60) )
    __assert_fail("COMP_GT == pred_ptr->args.reftime.kind", "pred.c", 0xC7u, "pred_cnewer");
  v3 = sub_30A66(a2);
  return (int)sub_8BA9(v3, v4, *(long long *)(a3 + 64), *(long long *)(a3 + 72)) > 0;
}



// Function: pred_comma @ 0x8fd0
long long pred_comma(long long a1, long long a2, long long a3)
{
  if ( *(long long *)(a3 + 272) )
    sub_10873(a1, a2, *(long long *)(a3 + 272));
  return sub_10873(a1, a2, *(long long *)(a3 + 280));
}



// Function: pred_ctime @ 0x9036
char pred_ctime(long long a1, long long a2, long long a3)
{
  time_t v3; // rax
  long long v4; // rdx

  v3 = sub_30A66(a2);
  return sub_8C32(v3, v4, a3, 86400);
}



// Function: perform_delete @ 0x9081
bool perform_delete(int a1)
{
  return unlinkat(dword_499D8, s1, a1) == 0;
}



// Function: pred_delete @ 0x90b1
long long pred_delete(long long a1, long long a2)
{
  long long v3; // r12
  char *v4; // rbx
  int *v5; // rax
  int v6; // [rsp+2Ch] [rbp-14h]

  if ( !strcmp(s1, ".") )
    return 1;
  v6 = 0;
  if ( byte_499C4 && (*(int *)(a2 + 24) & 0xF000) == 0x4000 )
    v6 = 512;
  if ( sub_9081(v6) )
    return 1;
  if ( *__errno_location() == 2 && byte_4994E )
  {
    *__errno_location() = 0;
    return 1;
  }
  else if ( *__errno_location() == 21 && (v6 & 0x200) == 0 && sub_9081(v6 | 0x200) )
  {
    return 1;
  }
  else
  {
    v3 = sub_1096A(0, a1);
    v4 = gettext("cannot delete %s");
    v5 = __errno_location();
    error(0, *v5, v4, v3);
    status = 1;
    return 0;
  }
}



// Function: pred_empty @ 0x91f4
bool pred_empty(long long a1, long long a2)
{
  int v2; // r8d
  int v3; // r9d
  const char *v4; // rbx
  int *v5; // rax
  const char *v7; // rbx
  int *v8; // rax
  const char *v9; // rbx
  int *v10; // rax
  char v11; // [rsp+2Bh] [rbp-25h]
  int fd; // [rsp+2Ch] [rbp-24h]
  struct dirent *i; // [rsp+30h] [rbp-20h]
  DIR *dirp; // [rsp+38h] [rbp-18h]

  if ( (*(int *)(a2 + 24) & 0xF000) != 0x4000 )
    return (*(int *)(a2 + 24) & 0xF000) == 0x8000 && *(long long *)(a2 + 48) == 0;
  v11 = 1;
  *__errno_location() = 0;
  fd = sub_2704E(dword_499D8, (int)s1, 592128, (int)s1, v2, v3);
  if ( fd < 0 )
  {
LABEL_3:
    v4 = (const char *)sub_1096A(0, a1);
    v5 = __errno_location();
    error(0, *v5, "%s", v4);
    status = 1;
    return 0;
  }
  dirp = fdopendir(fd);
  if ( dirp )
  {
    *__errno_location() = 0;
    for ( i = readdir(dirp); i; i = readdir(dirp) )
    {
      if ( i->d_name[0] != 46 || i->d_name[1] && (i->d_name[1] != 46 || i->d_name[2]) )
      {
        v11 = 0;
        break;
      }
    }
    if ( *__errno_location() )
    {
      v9 = (const char *)sub_1096A(0, a1);
      v10 = __errno_location();
      error(0, *v10, "%s", v9);
      status = 1;
      closedir(dirp);
      return 0;
    }
    else
    {
      if ( closedir(dirp) )
        goto LABEL_3;
      return v11;
    }
  }
  else
  {
    v7 = (const char *)sub_1096A(0, a1);
    v8 = __errno_location();
    error(0, *v8, "%s", v7);
    status = 1;
    close(fd);
    return 0;
  }
}



// Function: pred_exec @ 0x9466
long long pred_exec(long long a1, long long a2, long long a3)
{
  return sub_B300(a1, a2, a3);
}



// Function: pred_execdir @ 0x9497
long long pred_execdir(long long a1, long long a2, long long a3)
{
  return sub_B300(s1, a2, a3);
}



// Function: pred_false @ 0x94cb
long long pred_false()
{
  return 0;
}



// Function: pred_fls @ 0x94e6
long long pred_fls(int a1, int a2, long long a3)
{
  sub_1B7B3(
    a1,
    dword_499D8,
    (int)s1,
    a2,
    qword_49958,
    dword_4997C,
    *(unsigned char *)(a3 + 40),
    *(long long *)(a3 + 64));
  return 1;
}



// Function: pred_fprint @ 0x954f
long long pred_fprint(const char *a1, long long a2, long long a3)
{
  sub_1C643(*(long long *)(a3 + 64), *(long long *)(a3 + 88), *(unsigned char *)(a3 + 80), "%s\n", a1);
  return 1;
}



// Function: pred_fprint0 @ 0x959f
long long pred_fprint0(const char *a1, long long a2, long long a3)
{
  FILE *stream; // [rsp+28h] [rbp-8h]

  stream = *(FILE **)(a3 + 64);
  fputs(a1, stream);
  putc(0, stream);
  return 1;
}



// Function: pred_fstype @ 0x95ee
long long pred_fstype(long long a1, long long a2, long long a3)
{
  const char *v3; // rbx
  const char *v4; // rax

  v3 = *(const char **)(a3 + 56);
  v4 = (const char *)sub_13A18(a2, a1);
  return strcmp(v4, v3) == 0;
}



// Function: pred_gid @ 0x9643
long long pred_gid(long long a1, long long a2, long long a3)
{
  unsigned int v3; // eax

  v3 = *(int *)(a3 + 56);
  if ( v3 == 2 )
  {
    if ( *(int *)(a2 + 32) == *(long long *)(a3 + 64) )
      return 1;
  }
  else if ( v3 <= 2 )
  {
    if ( v3 )
    {
      if ( (unsigned long long)*(unsigned int *)(a2 + 32) < *(long long *)(a3 + 64) )
        return 1;
    }
    else if ( (unsigned long long)*(unsigned int *)(a2 + 32) > *(long long *)(a3 + 64) )
    {
      return 1;
    }
  }
  return 0;
}



// Function: pred_group @ 0x96d8
long long pred_group(long long a1, long long a2, long long a3)
{
  return *(int *)(a3 + 56) == *(int *)(a2 + 32);
}



// Function: pred_ilname @ 0x970c
long long pred_ilname(long long a1, long long a2, long long a3)
{
  return sub_9975(a1, a2, a3, 1);
}



// Function: pred_name_common @ 0x973f
long long pred_name_common(long long a1, const char *a2, int a3)
{
  bool v5; // [rsp+27h] [rbp-9h]
  char *name; // [rsp+28h] [rbp-8h]

  name = (char *)sub_1D85C(a1);
  sub_1DA6F(name);
  v5 = fnmatch(a2, name, a3) == 0;
  free(name);
  return v5;
}



// Function: pred_iname @ 0x97a2
long long pred_iname(long long a1, long long a2, long long a3)
{
  return sub_973F(a1, *(const char **)(a3 + 56), 16);
}



// Function: pred_inum @ 0x97d8
long long pred_inum(long long a1, long long a2, long long a3)
{
  unsigned int v3; // eax

  v3 = *(int *)(a3 + 56);
  if ( v3 == 2 )
  {
    if ( *(long long *)(a2 + 8) == *(long long *)(a3 + 64) )
      return 1;
  }
  else if ( v3 <= 2 )
  {
    if ( v3 )
    {
      if ( *(long long *)(a2 + 8) < *(long long *)(a3 + 64) )
        return 1;
    }
    else if ( *(long long *)(a2 + 8) > *(long long *)(a3 + 64) )
    {
      return 1;
    }
  }
  return 0;
}



// Function: pred_ipath @ 0x986a
long long pred_ipath(const char *a1, long long a2, long long a3)
{
  return fnmatch(*(const char **)(a3 + 56), a1, 16) == 0;
}



// Function: pred_links @ 0x98b0
long long pred_links(long long a1, long long a2, long long a3)
{
  unsigned int v3; // eax

  v3 = *(int *)(a3 + 56);
  if ( v3 == 2 )
  {
    if ( *(long long *)(a2 + 16) == *(long long *)(a3 + 64) )
      return 1;
  }
  else if ( v3 <= 2 )
  {
    if ( v3 )
    {
      if ( *(long long *)(a2 + 16) < *(long long *)(a3 + 64) )
        return 1;
    }
    else if ( *(long long *)(a2 + 16) > *(long long *)(a3 + 64) )
    {
      return 1;
    }
  }
  return 0;
}



// Function: pred_lname @ 0x9942
long long pred_lname(long long a1, long long a2, long long a3)
{
  return sub_9975(a1, a2, a3, 0);
}



// Function: match_lname @ 0x9975
long long match_lname(long long a1, long long a2, long long a3, char a4)
{
  int v4; // edx
  int *v5; // rax
  unsigned char v9; // [rsp+27h] [rbp-9h]
  char *name; // [rsp+28h] [rbp-8h]

  v9 = 0;
  if ( (*(int *)(a2 + 24) & 0xF000) == 0xA000 )
  {
    name = (char *)sub_1D06D((unsigned int)dword_499D8, s1);
    if ( name )
    {
      if ( a4 )
        v4 = 16;
      else
        v4 = 0;
      if ( !fnmatch(*(const char **)(a3 + 56), name, v4) )
        v9 = 1;
    }
    else
    {
      v5 = __errno_location();
      sub_10A13((unsigned int)*v5, a1);
      status = 1;
    }
    free(name);
  }
  return v9;
}



// Function: pred_ls @ 0x9a33
long long pred_ls(int a1, int a2, long long a3)
{
  return sub_94E6(a1, a2, a3);
}



// Function: pred_mmin @ 0x9a64
char pred_mmin(long long a1, long long a2, long long a3)
{
  time_t v3; // rax
  long long v4; // rdx

  v3 = sub_30A80(a2);
  return sub_8C32(v3, v4, a3, 60);
}



// Function: pred_mtime @ 0x9aaf
char pred_mtime(long long a1, long long a2, long long a3)
{
  time_t v3; // rax
  long long v4; // rdx

  v3 = sub_30A80(a2);
  return sub_8C32(v3, v4, a3, 86400);
}



// Function: pred_name @ 0x9afa
long long pred_name(long long a1, long long a2, long long a3)
{
  return sub_973F(a1, *(const char **)(a3 + 56), 0);
}



// Function: pred_negate @ 0x9b30
long long pred_negate(long long a1, long long a2, long long a3)
{
  return (unsigned char)sub_10873(a1, a2, *(long long *)(a3 + 280)) == 0;
}



// Function: pred_newer @ 0x9b79
bool pred_newer(long long a1, long long a2, long long a3)
{
  time_t v3; // rax
  long long v4; // rdx

  if ( *(int *)(a3 + 60) )
    __assert_fail("COMP_GT == pred_ptr->args.reftime.kind", "pred.c", 0x26Du, "pred_newer");
  v3 = sub_30A80(a2);
  return (int)sub_8BA9(v3, v4, *(long long *)(a3 + 64), *(long long *)(a3 + 72)) > 0;
}



// Function: pred_newerXY @ 0x9c02
bool pred_newerXY(long long a1, long long a2, long long a3)
{
  bool result; // al
  long long v4; // rdx
  long long v5; // rdx
  long long v6; // rbx
  char *v7; // rax
  long long v8; // rdx
  long long v9; // rdx
  char v11; // [rsp+2Fh] [rbp-21h]
  time_t v12; // [rsp+30h] [rbp-20h]
  long long v13; // [rsp+38h] [rbp-18h]

  v11 = 0;
  if ( *(int *)(a3 + 60) )
    __assert_fail("COMP_GT == pred_ptr->args.reftime.kind", "pred.c", 0x277u, "pred_newerXY");
  switch ( *(int *)(a3 + 56) )
  {
    case 0:
      v12 = sub_30A4C(a2);
      v13 = v4;
      v11 = 1;
      goto LABEL_12;
    case 1:
      v12 = sub_30A9A(a2);
      v13 = v5;
      v11 = 1;
      if ( v5 >= 0 )
        goto LABEL_12;
      v6 = sub_1096A(0, a1);
      v7 = gettext("WARNING: cannot determine birth time of file %s");
      error(0, 0, v7, v6);
      result = 0;
      break;
    case 2:
      v12 = sub_30A66(a2);
      v13 = v8;
      v11 = 1;
      goto LABEL_12;
    case 3:
      v12 = sub_30A80(a2);
      v13 = v9;
      v11 = 1;
      goto LABEL_12;
    case 4:
      if ( *(int *)(a3 + 56) == 4 )
        __assert_fail("pred_ptr->args.reftime.xval != XVAL_TIME", "pred.c", 0x27Cu, "pred_newerXY");
      return 0;
    default:
LABEL_12:
      if ( !v11 )
        __assert_fail("collected", "pred.c", 0x29Bu, "pred_newerXY");
      result = (int)sub_8BA9(v12, v13, *(long long *)(a3 + 64), *(long long *)(a3 + 72)) > 0;
      break;
  }
  return result;
}



// Function: pred_nogroup @ 0x9dd5
bool pred_nogroup(long long a1, long long a2)
{
  return getgrgid(*(int *)(a2 + 32)) == 0;
}



// Function: pred_nouser @ 0x9e03
bool pred_nouser(long long a1, long long a2)
{
  return getpwuid(*(int *)(a2 + 28)) == 0;
}



// Function: is_ok @ 0x9e31
long long is_ok(long long a1, long long a2)
{
  char *v2; // rax
  char *v3; // rbx
  int *v4; // rax

  sub_1E1A8(stdout);
  v2 = gettext("< %s ... %s > ? ");
  if ( fprintf(stderr, v2, a1, a2) < 0 )
  {
    v3 = gettext("Failed to write prompt for -ok");
    v4 = __errno_location();
    error(1, *v4, v3);
  }
  sub_1E1A8(stderr);
  return sub_32D7C();
}



// Function: pred_ok @ 0x9ece
long long pred_ok(long long a1, long long a2, long long a3)
{
  if ( (unsigned char)sub_9E31(**(long long **)(a3 + 232), a1) )
    return sub_B300(a1, a2, a3);
  else
    return 0;
}



// Function: pred_okdir @ 0x9f27
long long pred_okdir(long long a1, long long a2, long long a3)
{
  if ( (unsigned char)sub_9E31(**(long long **)(a3 + 232), a1) )
    return sub_B300(s1, a2, a3);
  else
    return 0;
}



// Function: pred_openparen @ 0x9f83
long long pred_openparen()
{
  return 1;
}



// Function: pred_or @ 0x9f9e
long long pred_or(long long a1, long long a2, long long a3)
{
  if ( *(long long *)(a3 + 272) && (unsigned char)sub_10873(a1, a2, *(long long *)(a3 + 272)) == 1 )
    return 1;
  else
    return sub_10873(a1, a2, *(long long *)(a3 + 280));
}



// Function: pred_path @ 0xa012
long long pred_path(const char *a1, long long a2, long long a3)
{
  return fnmatch(*(const char **)(a3 + 56), a1, 0) == 0;
}



// Function: pred_perm @ 0xa058
bool pred_perm(long long a1, long long a2, long long a3)
{
  unsigned int v3; // eax
  int v4; // eax
  int v6; // [rsp+28h] [rbp-8h]
  int v7; // [rsp+2Ch] [rbp-4h]

  v6 = *(int *)(a2 + 24);
  v7 = *(int *)(a3 + 4 * (((v6 & 0xF000) == 0x4000) + 12LL) + 12);
  v3 = *(int *)(a3 + 56);
  if ( v3 == 2 )
  {
    LOBYTE(v4) = v7 == (v6 & 0xFFF);
  }
  else
  {
    if ( v3 > 2 )
      abort();
    if ( v3 )
    {
      if ( v7 )
        LOBYTE(v4) = (v7 & v6) != 0;
      else
        LOBYTE(v4) = 1;
    }
    else
    {
      return v7 == (v7 & v6);
    }
  }
  return v4;
}



// Function: pred_executable @ 0xa0fa
bool pred_executable()
{
  return faccessat(dword_499D8, s1, 1, 0) == 0;
}



// Function: pred_readable @ 0xa137
bool pred_readable()
{
  return faccessat(dword_499D8, s1, 4, 0) == 0;
}



// Function: pred_writable @ 0xa174
bool pred_writable()
{
  return faccessat(dword_499D8, s1, 2, 0) == 0;
}



// Function: pred_print @ 0xa1b1
long long pred_print(const char *a1, long long a2, long long a3)
{
  sub_1C643(*(long long *)(a3 + 64), *(long long *)(a3 + 88), *(unsigned char *)(a3 + 80), "%s\n", a1);
  return 1;
}



// Function: pred_print0 @ 0xa201
long long pred_print0(const char *a1, long long a2, long long a3)
{
  return sub_959F(a1, a2, a3);
}



// Function: pred_prune @ 0xa232
long long pred_prune(long long a1, long long a2)
{
  if ( byte_49940 )
  {
    if ( !byte_499C4 )
      __assert_fail("state.have_stat", "pred.c", 0x355u, "pred_prune");
    if ( a2 && (*(int *)(a2 + 24) & 0xF000) == 0x4000 )
      byte_499E0 = 1;
  }
  return 1;
}



// Function: pred_quit @ 0xa2b0
void pred_quit()
{
  sub_FA38();
  exit(status);
}



// Function: pred_regex @ 0xa2da
long long pred_regex(const char *a1, long long a2, long long a3)
{
  int length; // [rsp+2Ch] [rbp-4h]

  length = strlen(a1);
  return length == re_match(*(struct re_pattern_buffer **)(a3 + 56), a1, length, 0, 0);
}



// Function: pred_size @ 0xa336
long long pred_size(long long a1, long long a2, long long a3)
{
  unsigned int v3; // eax
  unsigned long long v5; // [rsp+20h] [rbp-8h]

  v5 = *(long long *)(a2 + 48) / *(int *)(a3 + 60) + (*(long long *)(a2 + 48) % *(int *)(a3 + 60) != 0);
  v3 = *(int *)(a3 + 56);
  if ( v3 == 2 )
  {
    if ( v5 == *(long long *)(a3 + 64) )
      return 1;
  }
  else if ( v3 <= 2 )
  {
    if ( v3 )
    {
      if ( v5 < *(long long *)(a3 + 64) )
        return 1;
    }
    else if ( v5 > *(long long *)(a3 + 64) )
    {
      return 1;
    }
  }
  return 0;
}



// Function: pred_samefile @ 0xa3f7
long long pred_samefile(long long a1, long long *a2, long long a3)
{
  if ( a2[1] && a2[1] != *(long long *)(a3 + 56) )
    return 0;
  if ( (unsigned int)sub_F27D(a1, s1, a2) )
    return 0;
  return a2[1] == *(long long *)(a3 + 56) && *a2 == *(long long *)(a3 + 64);
}



// Function: pred_true @ 0xa497
long long pred_true()
{
  return 1;
}



// Function: pred_type @ 0xa4b2
long long pred_type(long long a1, long long a2, long long a3)
{
  unsigned int v4; // eax
  int v5; // [rsp+28h] [rbp-8h]
  unsigned int v6; // [rsp+2Ch] [rbp-4h]

  v6 = 7;
  if ( !byte_499C5 )
    __assert_fail("state.have_type", "pred.c", 0x3D1u, "pred_type");
  if ( !dword_499C8 )
    return 0;
  if ( byte_499C4 )
    v5 = *(int *)(a2 + 24);
  else
    LOWORD(v5) = dword_499C8;
  v4 = v5 & 0xF000;
  if ( v4 == 49152 )
  {
    v6 = 6;
  }
  else if ( v4 <= 0xC000 )
  {
    if ( v4 == 40960 )
    {
      v6 = 4;
    }
    else if ( v4 <= 0xA000 )
    {
      if ( v4 == 0x8000 )
      {
        v6 = 3;
      }
      else if ( v4 <= 0x8000 )
      {
        if ( v4 == 24576 )
        {
          v6 = 0;
        }
        else if ( v4 <= 0x6000 )
        {
          if ( v4 == 0x4000 )
          {
            v6 = 2;
          }
          else if ( v4 <= 0x4000 )
          {
            if ( v4 == 4096 )
            {
              v6 = 5;
            }
            else if ( v4 == 0x2000 )
            {
              v6 = 1;
            }
          }
        }
      }
    }
  }
  return v6 != 7 && *(char *)(a3 + v6 + 56);
}



// Function: pred_uid @ 0xa600
long long pred_uid(long long a1, long long a2, long long a3)
{
  unsigned int v3; // eax

  v3 = *(int *)(a3 + 56);
  if ( v3 == 2 )
  {
    if ( *(int *)(a2 + 28) == *(long long *)(a3 + 64) )
      return 1;
  }
  else if ( v3 <= 2 )
  {
    if ( v3 )
    {
      if ( (unsigned long long)*(unsigned int *)(a2 + 28) < *(long long *)(a3 + 64) )
        return 1;
    }
    else if ( (unsigned long long)*(unsigned int *)(a2 + 28) > *(long long *)(a3 + 64) )
    {
      return 1;
    }
  }
  return 0;
}



// Function: pred_used @ 0xa695
char pred_used(long long a1, long long a2, long long a3)
{
  long long v3; // rdx
  long long v4; // rdx
  time_t v7; // [rsp+20h] [rbp-30h]
  long long v8; // [rsp+28h] [rbp-28h]
  time_t v9; // [rsp+30h] [rbp-20h]
  long long v10; // [rsp+38h] [rbp-18h]
  time_t v11; // [rsp+40h] [rbp-10h]
  long long v12; // [rsp+48h] [rbp-8h]

  v9 = sub_30A4C(a2);
  v10 = v3;
  v11 = sub_30A66(a2);
  v12 = v4;
  if ( (int)sub_8BA9(v9, v10, v11, v4) < 0 )
    return 0;
  v7 = v11 - v9;
  v8 = v12 - v10;
  if ( v12 - v10 < 0 )
  {
    v8 += 1000000000;
    --v7;
  }
  return sub_8C32(v7, v8, a3, 86400);
}



// Function: pred_user @ 0xa75a
long long pred_user(long long a1, long long a2, long long a3)
{
  return *(int *)(a3 + 56) == *(int *)(a2 + 28);
}



// Function: pred_xtype @ 0xa78e
long long pred_xtype(long long a1, long long a2, long long a3)
{
  const char *v4; // rbx
  int *v5; // rax
  long long (*v7)(); // [rsp+28h] [rbp-B8h]
  char v8[152]; // [rsp+30h] [rbp-B0h] BYREF
  unsigned long long v9; // [rsp+C8h] [rbp-18h]

  v9 = __readfsqword(0x28u);
  if ( (unsigned char)sub_FE03() )
    v7 = sub_FCC0;
  else
    v7 = sub_FC21;
  sub_F26E(v8);
  if ( !((unsigned int (*)(char *, char *))v7)(s1, v8) )
    return sub_A4B2(a1, (long long)v8, a3);
  if ( (unsigned char)sub_FE03() && *__errno_location() == 2 )
    return sub_A4B2(a1, a2, a3);
  v4 = (const char *)sub_1096A(0, a1);
  v5 = __errno_location();
  error(0, *v5, "%s", v4);
  status = 1;
  return 0;
}



// Function: pred_context @ 0xa8d8
bool pred_context(long long a1, long long a2, long long a3)
{
  long long v3; // r12
  char *v4; // rbx
  int *v5; // rax
  int v8; // [rsp+2Ch] [rbp-24h]
  char *name[4]; // [rsp+30h] [rbp-20h] BYREF

  name[1] = (char *)__readfsqword(0x28u);
  if ( (int)qword_499A0((unsigned int)dword_499D8, s1, name) >= 0 )
  {
    v8 = fnmatch(*(const char **)(a3 + 56), name[0], 0) == 0;
    freecon(name[0]);
    return v8;
  }
  else
  {
    v3 = sub_1096A(0, a1);
    v4 = gettext("getfilecon failed: %s");
    v5 = __errno_location();
    error(0, *v5, v4, v3);
    return 0;
  }
}



// Function: blank_rtrim @ 0xa9cd
char *blank_rtrim(const char *a1, char *a2)
{
  int i; // [rsp+1Ch] [rbp-4h]

  if ( !a1 )
    return 0;
  strcpy(a2, a1);
  for ( i = strlen(a2) - 1; i >= 0 && (a2[i] == 32 || a2[i] == 9); --i )
    ;
  a2[i + 1] = 0;
  return a2;
}



// Function: print_list @ 0xaa62
unsigned long long print_list(FILE *a1, long long a2)
{
  char *v2; // rax
  char v5[264]; // [rsp+20h] [rbp-110h] BYREF
  unsigned long long v6; // [rsp+128h] [rbp-8h]

  v6 = __readfsqword(0x28u);
  while ( a2 )
  {
    v2 = sub_A9CD(*(const char **)(a2 + 8), v5);
    fprintf(a1, "[%s] ", v2);
    a2 = *(long long *)(a2 + 264);
  }
  fprintf(a1, "\n");
  return v6 - __readfsqword(0x28u);
}



// Function: print_parenthesised @ 0xab30
int print_parenthesised(FILE *a1, long long a2)
{
  int result; // eax
  int v3; // [rsp+1Ch] [rbp-4h]

  v3 = 0;
  if ( a2 )
  {
    if ( (*(long long (**)(long long, long long, long long))a2 == sub_9F9E
       || *(long long (**)(long long, long long, long long))a2 == sub_8D9C)
      && !*(long long *)(a2 + 272) )
    {
      return print_parenthesised(a1, *(long long *)(a2 + 280));
    }
    else
    {
      if ( *(long long *)(a2 + 272) || *(long long *)(a2 + 280) )
        v3 = 1;
      if ( v3 )
        fprintf(a1, "%s", " ( ");
      result = sub_AC38(a1, a2);
      if ( v3 )
        return fprintf(a1, "%s", " ) ");
    }
  }
  return result;
}



// Function: print_optlist @ 0xac38
void print_optlist(FILE *a1, long long a2)
{
  const char *v2; // rcx
  const char *v3; // rdx
  const char *v4; // rax
  long long v5; // rax
  double v6; // xmm0_8
  long long v7; // rdx
  long long v8; // rax
  double v9; // xmm1_8
  long long v10; // rdx

  if ( a2 )
  {
    sub_AB30(a1, *(long long *)(a2 + 272));
    if ( *(char *)(a2 + 28) )
      v2 = "[need inum] ";
    else
      v2 = (const char *)&unk_387B2;
    if ( *(char *)(a2 + 27) )
      v3 = "[need type] ";
    else
      v3 = (const char *)&unk_387B2;
    if ( *(char *)(a2 + 26) )
      v4 = "[call stat] ";
    else
      v4 = (const char *)&unk_387B2;
    fprintf(a1, "%s%s%s", v4, v3, v2);
    sub_C2DC(a1, a2);
    fprintf(a1, " [est success rate %.4g] ", *(float *)(a2 + 36));
    if ( (qword_49980 & 0x40) != 0 )
    {
      fprintf(a1, "[real success rate %lu/%lu", *(long long *)(a2 + 296), *(long long *)(a2 + 288));
      if ( *(long long *)(a2 + 288) )
      {
        v5 = *(long long *)(a2 + 296);
        if ( v5 < 0 )
        {
          v7 = *(long long *)(a2 + 296) & 1LL | ((unsigned long long)v5 >> 1);
          v6 = (double)(int)v7 + (double)(int)v7;
        }
        else
        {
          v6 = (double)(int)v5;
        }
        v8 = *(long long *)(a2 + 288);
        if ( v8 < 0 )
        {
          v10 = *(long long *)(a2 + 288) & 1LL | ((unsigned long long)v8 >> 1);
          v9 = (double)(int)v10 + (double)(int)v10;
        }
        else
        {
          v9 = (double)(int)v8;
        }
        fprintf(a1, "=%.4g] ", v6 / v9);
      }
      else
      {
        fprintf(a1, "=_] ");
      }
    }
    sub_AB30(a1, *(long long *)(a2 + 280));
  }
}



// Function: show_success_rates @ 0xae53
int show_success_rates(long long a1)
{
  int result; // eax

  result = qword_49980 & 0x40;
  if ( (qword_49980 & 0x40) != 0 )
  {
    fprintf(stderr, "Predicate success rates after completion:\n");
    sub_AC38(stderr, a1);
    return fprintf(stderr, "\n");
  }
  return result;
}



// Function: pred_sanity_check @ 0xaec7
long long pred_sanity_check(long long a1)
{
  long long result; // rax
  long long i; // [rsp+18h] [rbp-8h]

  result = a1;
  for ( i = a1; i; i = result )
  {
    if ( !*(long long *)i )
      __assert_fail("p->pred_func != NULL", "pred.c", 0x51Eu, "pred_sanity_check");
    if ( !*(long long *)(i + 304) )
      __assert_fail("p->parser_entry != NULL", "pred.c", 0x521u, "pred_sanity_check");
    if ( *(long long *)(*(long long *)(i + 304) + 24LL) && *(long long *)(*(long long *)(i + 304) + 24LL) != *(long long *)i )
      __assert_fail("p->parser_entry->pred_func == p->pred_func", "pred.c", 0x52Au, "pred_sanity_check");
    switch ( **(int **)(i + 304) )
    {
      case 0:
      case 2:
        if ( !**(int **)(i + 304) )
          __assert_fail("p->parser_entry->type != ARG_OPTION", "pred.c", 0x53Au, "pred_sanity_check");
        if ( **(int **)(i + 304) == 2 )
          __assert_fail("p->parser_entry->type != ARG_POSITIONAL_OPTION", "pred.c", 0x53Bu, "pred_sanity_check");
        return result;
      case 1:
      case 3:
      case 4:
      case 5:
        if ( *(char *)(i + 25) == 1 )
          __assert_fail("!p->no_default_print", "pred.c", 0x553u, "pred_sanity_check");
        if ( *(char *)(i + 24) == 1 )
          __assert_fail("!p->side_effects", "pred.c", 0x554u, "pred_sanity_check");
        return result;
      case 6:
        if ( !*(char *)(i + 24) )
          __assert_fail("p->side_effects", "pred.c", 0x53Fu, "pred_sanity_check");
        if ( *(long long (**)(long long, long long))i != sub_A232
          && *(void (**)())i != sub_A2B0
          && !*(char *)(i + 25) )
        {
          __assert_fail("p->no_default_print", "pred.c", 0x545u, "pred_sanity_check");
        }
        return result;
      default:
        break;
    }
    result = *(long long *)(i + 264);
  }
  return result;
}



// Function: initialize_wd_for_exec @ 0xb184
long long initialize_wd_for_exec(long long a1, int a2, int a3)
{
  int *v3; // rbx
  int v4; // r8d
  int v5; // r9d

  *(long long *)(a1 + 192) = sub_31BC2(16);
  *(long long *)(*(long long *)(a1 + 192) + 8LL) = 0;
  v3 = *(int **)(a1 + 192);
  *v3 = sub_2704E(a2, a3, 0, a3, v4, v5);
  if ( **(int **)(a1 + 192) < 0 )
    return 0;
  sub_1D5BA(**(unsigned int **)(a1 + 192), 1);
  return 1;
}



// Function: record_exec_dir @ 0xb22b
long long record_exec_dir(long long a1)
{
  unsigned char v2; // [rsp+17h] [rbp-9h]
  void *ptr; // [rsp+18h] [rbp-8h]

  if ( *(int *)(a1 + 152) )
    return 1;
  if ( !strchr(s1, 47) )
    return sub_B184(a1, dword_499D8, (int)".");
  ptr = (void *)sub_1D9D4(s1);
  v2 = sub_B184(a1, dword_499D8, (int)ptr);
  free(ptr);
  return v2;
}



// Function: impl_pred_exec @ 0xb300
long long impl_pred_exec(char *a1, long long a2, long long *a3)
{
  long long v3; // r12
  char *v4; // rbx
  int *v5; // rax
  int v6; // eax
  size_t v7; // rbx
  int v8; // eax
  bool v10; // [rsp+22h] [rbp-3Eh]
  char v11; // [rsp+23h] [rbp-3Dh]
  int i; // [rsp+24h] [rbp-3Ch]
  char *ptr; // [rsp+28h] [rbp-38h]
  char *s; // [rsp+30h] [rbp-30h]
  const char *v15; // [rsp+38h] [rbp-28h]
  int v16; // [rsp+40h] [rbp-20h]
  long long v17; // [rsp+48h] [rbp-18h]

  v17 = (long long)(a3 + 7);
  ptr = 0;
  v11 = sub_10933(*a3);
  if ( v11 )
  {
    if ( (unsigned char)sub_B22B(v17) != 1 )
    {
      v3 = sub_1096A(0, a1);
      v4 = gettext("Failed to save working directory in order to run a command on %s");
      v5 = __errno_location();
      error(1, *v5, v4, v3);
    }
    ptr = (char *)sub_1D85C(s1);
    s = ptr;
    if ( *ptr == 47 )
    {
      LODWORD(v15) = 0;
      v16 = 0;
    }
    else
    {
      v15 = "./";
      v16 = 2;
    }
  }
  else
  {
    if ( *(void **)(v17 + 192) != ::ptr )
      __assert_fail("execp->wd_for_exec == initial_wd", "exec.c", 0x94u, "impl_pred_exec");
    s = a1;
    LODWORD(v15) = 0;
    v16 = 0;
  }
  if ( *(char *)v17 )
  {
    v6 = strlen(s);
    sub_1A1B8(v17 + 8, v17 + 96, (int)s, v6 + 1, (int)v15, v16, 0);
    if ( *(int *)(v17 + 152) )
      byte_499E8 = 1;
    v10 = 1;
  }
  else
  {
    for ( i = 0; i < *(int *)(v17 + 184); ++i )
    {
      v7 = strlen(s);
      v8 = strlen(*(const char **)(*(long long *)(v17 + 176) + 8LL * i));
      sub_19AF7(v17 + 8, v17 + 96, *(long long *)(*(long long *)(v17 + 176) + 8LL * i), v8, (int)v15, v16, s, v7, 0);
    }
    sub_19F5A(v17 + 8, v17 + 96);
    v10 = (*(int *)(v17 + 200) & 0x7F) == 0 && BYTE1(*(int *)(v17 + 200)) == 0;
    if ( v11 )
      sub_2FE52(*(long long *)(v17 + 192));
  }
  if ( ptr )
  {
    if ( !v11 )
      __assert_fail("local", "exec.c", 0xD0u, "impl_pred_exec");
    free(ptr);
  }
  return v10;
}



// Function: prep_child_for_exec @ 0xb615
long long prep_child_for_exec(char a1, long long a2)
{
  int v2; // edx
  int v3; // ecx
  int v4; // r8d
  int v5; // r9d
  char *v6; // rbx
  int *v7; // rax
  const char *v8; // rbx
  int *v9; // rax
  void *v10; // r12
  const char *v11; // rbx
  char *v12; // r13
  int *v13; // rax
  long long v15; // [rsp+0h] [rbp-50h]
  unsigned char v16; // [rsp+1Dh] [rbp-33h]
  long long v17[6]; // [rsp+1Eh] [rbp-32h] BYREF

  v15 = a2;
  *(long long *)((char *)&v17[1] + 2) = __readfsqword(0x28u);
  v16 = 1;
  if ( a1 )
  {
    strcpy((char *)v17, "/dev/null");
    if ( close(0) >= 0 )
    {
      if ( (int)sub_1E034((unsigned int)v17, 0, v2, v3, v4, v5, a2) < 0 )
      {
        v8 = (const char *)sub_1096A(0, v17);
        v9 = __errno_location();
        error(0, *v9, "%s", v8);
      }
    }
    else
    {
      v6 = gettext("Cannot close standard input");
      v7 = __errno_location();
      error(0, *v7, v6);
      v16 = 0;
    }
  }
  if ( (unsigned int)sub_2FE17(v15) )
  {
    if ( *(int *)v15 < 0 && *(long long *)(v15 + 8) )
      v10 = *(void **)(v15 + 8);
    else
      v10 = &unk_38A9E;
    if ( *(int *)v15 < 0 && *(long long *)(v15 + 8) )
      v11 = ": ";
    else
      v11 = (const char *)&unk_38A9E;
    v12 = gettext("Failed to change directory%s%s");
    v13 = __errno_location();
    error(0, *v13, v12, v11, v10);
    return 0;
  }
  return v16;
}



// Function: launch @ 0xb7ae
long long launch(long long a1, long long a2, long long a3, const char **a4)
{
  const char *v4; // rax
  char *v5; // rbx
  int *v6; // rax
  const char *v7; // rbx
  int *v8; // rax
  long long v9; // r12
  char *v10; // rbx
  int *v11; // rax
  unsigned int v13; // r12d
  long long v14; // rbx
  char *v15; // rax
  int i; // [rsp+2Ch] [rbp-24h]
  __pid_t pid; // [rsp+30h] [rbp-20h]
  int v19; // [rsp+34h] [rbp-1Ch]

  if ( (qword_49980 & 0x20) != 0 )
  {
    fprintf(stderr, "DebugExec: launching process (argc=%lu):", *(long long *)(a2 + 96) - 1LL);
    for ( i = 0; i < (unsigned long long)(*(long long *)(a2 + 96) - 1LL); ++i )
    {
      v4 = (const char *)sub_1096A(0, *(long long *)(*(long long *)(a2 + 104) + 8LL * i));
      fprintf(stderr, " %s", v4);
    }
    fprintf(stderr, "\n");
  }
  sub_1E1A8(stdout);
  sub_1E1A8(stderr);
  if ( dword_4901C )
  {
    dword_4901C = 0;
    signal(17, 0);
  }
  pid = fork();
  if ( pid == -1 )
  {
    v5 = gettext("cannot fork");
    v6 = __errno_location();
    error(1, *v6, v5);
  }
  if ( !pid )
  {
    if ( !*(long long *)(a2 + 192) )
      __assert_fail("NULL != execp->wd_for_exec", "exec.c", 0x141u, "launch");
    if ( (unsigned char)sub_B615(*(char *)(a2 + 188), *(long long *)(a2 + 192)) != 1 )
      _exit(1);
    if ( (unsigned char)sub_1B5F4() )
      sub_1B61E();
    if ( (unsigned char)sub_1A94D(a4) )
      *__errno_location() = 7;
    else
      execvp(*a4, (char *const *)a4);
    v7 = (const char *)sub_1096A(0, *a4);
    v8 = __errno_location();
    error(0, *v8, "%s", v7);
    _exit(1);
  }
  while ( waitpid(pid, (int *)(a2 + 200), 0) == -1 )
  {
    if ( *__errno_location() != 4 )
    {
      v9 = sub_1096A(0, *a4);
      v10 = gettext("error waiting for %s");
      v11 = __errno_location();
      error(0, *v11, v10, v9);
      status = 1;
      return 0;
    }
  }
  if ( (char)((*(char *)(a2 + 200) & 0x7F) + 1) >> 1 <= 0 )
  {
    v19 = (unsigned char)BYTE1(*(int *)(a2 + 200));
    if ( (qword_49980 & 0x20) != 0 )
      fprintf(stderr, "DebugExec: process (PID=%ld) terminated with exit status: %d\n", pid, v19);
    if ( v19 )
    {
      if ( *(char *)a2 )
        status = 1;
      return 1;
    }
    else
    {
      return 1;
    }
  }
  else
  {
    v13 = *(int *)(a2 + 200) & 0x7F;
    v14 = sub_2F861(0, (unsigned int)dword_499AC, *a4);
    v15 = gettext("%s terminated by signal %d");
    error(0, 0, v15, v14, v13);
    if ( *(char *)a2 )
      status = 1;
    return 1;
  }
}



// Function: matches_start_point @ 0xbb9d
bool matches_start_point(const char *a1, char a2)
{
  int flags; // [rsp+14h] [rbp-Ch]
  unsigned long long i; // [rsp+18h] [rbp-8h]

  flags = 0;
  if ( a2 )
    flags = 16;
  if ( !qword_49A28 )
    return fnmatch(a1, ".", flags) == 0;
  for ( i = 0; i < qword_49A28; ++i )
  {
    if ( !fnmatch(a1, *(const char **)(qword_49A20 + 8 * i), flags) )
      return 1;
  }
  return 0;
}



// Function: get_expr @ 0xbc44
long long get_expr(long long *a1, short a2, long long a3)
{
  char *v3; // rax
  char *v4; // rax
  long long v5; // rbx
  char *v6; // rax
  long long v7; // rbx
  char *v8; // rax
  long long v9; // rbx
  char *v10; // rax
  long long v11; // rbx
  char *v12; // rax
  char *v13; // rax
  long long v14; // rbx
  char *v15; // rax
  long long v16; // rbx
  char *v17; // rax
  char *v18; // rax
  char *v19; // rax
  char *v20; // rax
  char *v22; // rax
  long long v24; // [rsp+8h] [rbp-38h]
  long long v25; // [rsp+20h] [rbp-20h]
  long long v26; // [rsp+28h] [rbp-18h]

  v25 = 0;
  v26 = *a1;
  if ( !*a1 )
  {
    v3 = gettext("invalid expression");
    error(1, 0, v3);
  }
  switch ( *(int *)(*a1 + 16) )
  {
    case 0:
      v4 = gettext("invalid expression");
      error(1, 0, v4);
      goto LABEL_5;
    case 1:
      goto LABEL_15;
    case 2:
      v25 = *a1;
      *a1 = *(long long *)(*a1 + 264);
      *(long long *)(v25 + 280) = get_expr(a1, 4, v25);
      break;
    case 3:
LABEL_5:
      v5 = *(long long *)(v26 + 8);
      v6 = gettext("invalid expression; you have used a binary operator '%s' with nothing before it.");
      error(1, 0, v6, v5);
      goto LABEL_6;
    case 4:
      if ( !*(long long *)(*a1 + 264) || *(char *)(*(long long *)(*a1 + 264) + 41LL) )
      {
        v14 = *(long long *)(v26 + 8);
        v15 = gettext(
                "invalid expression; expected to find a ')' but didn't see one. Perhaps you need an extra predicate after '%s'");
        error(1, 0, v15, v14);
      }
      v24 = *a1;
      *a1 = *(long long *)(*a1 + 264);
      if ( *(int *)(*a1 + 16) == 5 )
      {
        if ( *(char *)(v24 + 41) )
        {
          v16 = *(long long *)(*a1 + 8);
          v17 = gettext("invalid expression: expected expression before closing parentheses '%s'.");
          error(1, 0, v17, v16);
        }
        v18 = gettext("invalid expression; empty parentheses are not allowed.");
        error(1, 0, v18);
      }
      v25 = get_expr(a1, 0, v24);
      if ( !*a1 || *(int *)(*a1 + 16) != 5 )
      {
        v19 = gettext("invalid expression; I was expecting to find a ')' somewhere but did not see one.");
        error(1, 0, v19);
      }
      *a1 = *(long long *)(*a1 + 264);
      break;
    case 5:
LABEL_6:
      if ( !a3 )
      {
        v7 = *(long long *)(v26 + 8);
        v8 = gettext("invalid expression: expected expression before closing parentheses '%s'.");
        error(1, 0, v8, v7);
      }
      if ( (*(int *)(a3 + 16) == 2 || *(int *)(a3 + 16) == 3) && *(char *)(v26 + 41) != 1 )
      {
        v9 = *(long long *)(a3 + 8);
        v10 = gettext("expected an expression between '%s' and ')'");
        error(1, 0, v10, v9);
      }
      if ( *(char *)(*a1 + 41) )
      {
        v11 = *(long long *)(a3 + 8);
        v12 = gettext("expected an expression after '%s'");
        error(1, 0, v12, v11);
      }
      v13 = gettext("invalid expression; you have too many ')'");
      error(1, 0, v13);
LABEL_15:
      v25 = *a1;
      *a1 = *(long long *)(*a1 + 264);
      break;
    default:
      v20 = gettext("oops -- invalid expression type!");
      error(1, 0, v20);
      break;
  }
  if ( !*a1 )
    return v25;
  if ( *(int *)(*a1 + 20) > a2 )
  {
    v25 = sub_C08F(a1, v25, (unsigned int)a2);
    if ( !v25 )
    {
      v22 = gettext("invalid expression");
      error(1, 0, v22);
    }
  }
  return v25;
}



// Function: scan_rest @ 0xc08f
long long scan_rest(long long *a1, long long a2, short a3)
{
  unsigned int v4; // eax
  char *v5; // rax
  unsigned int v6; // ebx
  char *v7; // rax
  long long v10; // [rsp+28h] [rbp-18h]

  if ( !*a1 || *(int *)(*a1 + 16) == 5 )
    return 0;
  while ( *a1 && *(int *)(*a1 + 20) > a3 )
  {
    v4 = *(int *)(*a1 + 16);
    if ( v4 == 5 )
      return a2;
    if ( v4 > 5 )
    {
      v6 = *(int *)(*a1 + 16);
      v7 = gettext("oops -- invalid expression type (%d)!");
      error(1, 0, v7, v6);
    }
    else
    {
      if ( v4 == 4 || v4 <= 2 )
      {
        v5 = gettext("invalid expression");
        error(1, 0, v5);
      }
      v10 = *a1;
      *(long long *)(*a1 + 272) = a2;
      a2 = *a1;
      *a1 = *(long long *)(*a1 + 264);
      *(long long *)(a2 + 280) = sub_BC44(a1, *(int *)(a2 + 20), v10);
    }
  }
  return a2;
}



// Function: predicate_is_cost_free @ 0xc209
bool predicate_is_cost_free(long long a1)
{
  if ( *(long long (**)(long long, long long, long long))a1 == sub_9AFA
    || *(long long (**)(const char *, long long, long long))a1 == sub_A012
    || *(long long (**)(long long, long long, long long))a1 == sub_97A2
    || *(long long (**)(const char *, long long, long long))a1 == sub_986A )
  {
    return 1;
  }
  if ( !word_499A8 )
    return 0;
  if ( *(long long (**)(long long, long long, long long))a1 == sub_8D9C
    || *(long long (**)(long long, long long, long long))a1 == sub_9B30
    || *(long long (**)(long long, long long, long long))a1 == sub_8FD0
    || *(long long (**)(long long, long long, long long))a1 == sub_9F9E )
  {
    return 0;
  }
  return *(int *)(a1 + 32) == 0;
}



// Function: print_predicate @ 0xc2dc
int print_predicate(FILE *a1, long long a2)
{
  if ( *(long long *)(a2 + 48) )
    return fprintf(a1, "%s %s", *(const char **)(a2 + 8), *(const char **)(a2 + 48));
  else
    return fprintf(a1, "%s", *(const char **)(a2 + 8));
}



// Function: predlist_init @ 0xc34d
long long *predlist_init(long long *a1)
{
  a1[1] = 0;
  *a1 = a1[1];
  return a1;
}



// Function: predlist_insert @ 0xc377
long long *predlist_insert(long long *a1, long long a2, long long *a3)
{
  long long *result; // rax

  *a3 = *(long long *)(a2 + 272);
  *(long long *)(a2 + 272) = *a1;
  *a1 = a2;
  result = (long long *)a1[1];
  if ( !result )
  {
    a1[1] = *a1;
    return a1;
  }
  return result;
}



// Function: pred_cost_compare @ 0xc3e1
long long pred_cost_compare(long long a1, long long a2, char a3)
{
  if ( *(int *)(a1 + 32) == *(int *)(a2 + 32) )
  {
    if ( *(float *)(a1 + 36) == *(float *)(a2 + 36) )
    {
      return 0;
    }
    else if ( a3 )
    {
      if ( *(float *)(a2 + 36) <= *(float *)(a1 + 36) )
        return 1;
      else
        return 0xFFFFFFFFLL;
    }
    else if ( *(float *)(a2 + 36) <= *(float *)(a1 + 36) )
    {
      return 0xFFFFFFFFLL;
    }
    else
    {
      return 1;
    }
  }
  else if ( *(int *)(a1 + 32) >= *(int *)(a2 + 32) )
  {
    return 1;
  }
  else
  {
    return 0xFFFFFFFFLL;
  }
}



// Function: predlist_merge_sort @ 0xc49b
unsigned long long predlist_merge_sort(long long *a1, long long a2)
{
  long long i; // [rsp+20h] [rbp-30h]
  long long v4; // [rsp+28h] [rbp-28h]
  long long v5; // [rsp+30h] [rbp-20h] BYREF
  long long v6; // [rsp+38h] [rbp-18h]
  unsigned long long v7; // [rsp+48h] [rbp-8h]

  v7 = __readfsqword(0x28u);
  if ( *a1 )
  {
    if ( (qword_49980 & 8) != 0 )
    {
      fprintf(stderr, "%s:\n", "predlist before merge sort");
      sub_EA9F(stderr, *a1, 2);
    }
    sub_D838(*a1);
    sub_C34D(&v5);
    while ( *a1 )
    {
      v4 = *a1;
      *a1 = *(long long *)(*a1 + 272LL);
      *(long long *)(v4 + 272) = 0;
      for ( i = v5;
            i && (int)sub_C3E1(*(long long *)(i + 280), *(long long *)(v4 + 280), *(int *)(i + 20) != 2) < 0;
            i = *(long long *)(i + 272) )
      {
        ;
      }
      if ( i )
      {
        *(long long *)(v4 + 272) = *(long long *)(i + 272);
        if ( !*(long long *)(v4 + 272) )
          v6 = v4;
        *(long long *)(i + 272) = v4;
      }
      else
      {
        *(long long *)(v4 + 272) = v5;
        v5 = v4;
        if ( !v6 )
          v6 = v4;
      }
    }
    if ( (qword_49980 & 8) != 0 )
    {
      fprintf(stderr, "%s:\n", "predlist after merge sort");
      sub_EA9F(stderr, v5, 2);
    }
    sub_D838(v5);
    sub_D3FD(v5, v6, a2);
    sub_C34D(a1);
  }
  return v7 - __readfsqword(0x28u);
}



// Function: merge_lists @ 0xc6e0
long long merge_lists(long long a1, int a2, long long *a3, long long a4, long long a5)
{
  long long result; // rax
  unsigned int i; // [rsp+3Ch] [rbp-4h]

  qword_49A38 = (long long (*)(long long, long long))sub_C49B;
  sub_C49B(a3, a5);
  qword_49A38(a4, a5);
  for ( i = 0; ; ++i )
  {
    result = i;
    if ( (int)i >= a2 )
      break;
    qword_49A38(a1 + 16LL * (int)i, a5);
  }
  return result;
}



// Function: subtree_has_side_effects @ 0xc77a
long long subtree_has_side_effects(long long a1)
{
  if ( !a1 )
    return 0;
  return *(char *)(a1 + 24)
      || (unsigned char)subtree_has_side_effects(*(long long *)(a1 + 272))
      || (unsigned char)subtree_has_side_effects(*(long long *)(a1 + 280));
}



// Function: worst_cost @ 0xc7e3
long long worst_cost(long long a1)
{
  unsigned int v1; // edx
  unsigned int v2; // eax
  unsigned int v4; // [rsp+14h] [rbp-Ch]
  unsigned int v5; // [rsp+18h] [rbp-8h]

  if ( !a1 )
    return 0;
  v5 = worst_cost(*(long long *)(a1 + 272));
  v1 = worst_cost(*(long long *)(a1 + 280));
  v2 = v5;
  if ( v1 >= v5 )
    v2 = v1;
  v4 = v2;
  if ( v2 < *(int *)(a1 + 32) )
    return *(unsigned int *)(a1 + 32);
  return v4;
}



// Function: perform_arm_swap @ 0xc856
long long perform_arm_swap(long long a1)
{
  long long v2; // [rsp+10h] [rbp-8h]

  v2 = *(long long *)(*(long long *)(a1 + 272) + 280LL);
  *(long long *)(*(long long *)(a1 + 272) + 280LL) = *(long long *)(a1 + 280);
  *(long long *)(a1 + 280) = v2;
  return a1;
}



// Function: consider_arm_swap @ 0xc8a7
long long consider_arm_swap(long long a1)
{
  bool v2; // [rsp+17h] [rbp-29h]
  int v3; // [rsp+18h] [rbp-28h]
  int v4; // [rsp+1Ch] [rbp-24h]
  float v5; // [rsp+20h] [rbp-20h]
  float v6; // [rsp+24h] [rbp-1Ch]
  const char *v7; // [rsp+28h] [rbp-18h]
  long long *v8; // [rsp+30h] [rbp-10h]
  long long *v9; // [rsp+38h] [rbp-8h]

  v7 = 0;
  if ( *(int *)(a1 + 16) != 3 )
    v7 = "Not a binary operation";
  if ( !v7 && (!*(long long *)(a1 + 272) || !*(long long *)(a1 + 280)) )
    v7 = "Doesn't have two arms";
  if ( !v7 && !*(long long *)(*(long long *)(a1 + 272) + 280LL) )
    v7 = "Left arm has no child on RHS";
  v8 = (long long *)(a1 + 280);
  v9 = (long long *)(*(long long *)(a1 + 272) + 280LL);
  if ( !v7 && sub_C77A(*v9) )
    v7 = "Left subtree has side-effects";
  if ( !v7 && sub_C77A(*v8) )
    v7 = "Right subtree has side-effects";
  if ( !v7 )
  {
    v3 = sub_C7E3(*v9);
    v4 = sub_C7E3(*v8);
    if ( v3 < v4 )
      v7 = "efficient as-is";
  }
  if ( v7 )
    goto LABEL_35;
  if ( v3 == v4 )
  {
    v5 = *(float *)(*v9 + 36);
    v6 = *(float *)(*v8 + 36);
    if ( (qword_49980 & 8) != 0 )
      fprintf(stderr, "Success rates: l=%f, r=%f\n", v5, v6);
    if ( *(long long (**)(long long, long long, long long))a1 == sub_9F9E )
    {
      v2 = v5 > v6;
      if ( v5 <= v6 )
        v7 = "Operation is OR; right success rate >= left";
    }
    else if ( *(long long (**)(long long, long long, long long))a1 == sub_8D9C )
    {
      v2 = v6 > v5;
      if ( v6 <= v5 )
        v7 = "Operation is AND; right success rate <= left";
    }
    else
    {
      v2 = 0;
      v7 = "Not 'AND' or 'OR'";
    }
  }
  else
  {
    v2 = 1;
  }
  if ( !v2 )
  {
LABEL_35:
    if ( (qword_49980 & 8) != 0 )
    {
      fprintf(stderr, "Not an arm swap candidate (%s):\n", v7);
      sub_EA9F(stderr, a1, 0);
    }
    return 0;
  }
  else
  {
    if ( (qword_49980 & 8) != 0 )
    {
      fprintf(stderr, "Performing arm swap on:\n");
      sub_EA9F(stderr, a1, 0);
    }
    sub_C856(a1);
    return 1;
  }
}



// Function: do_arm_swaps @ 0xcb9f
long long do_arm_swaps(long long a1)
{
  char v2; // [rsp+1Fh] [rbp-1h]

  if ( !a1 )
    return 0;
  do
  {
    v2 = 0;
    if ( (unsigned char)sub_C8A7(a1)
      || (unsigned char)do_arm_swaps(*(long long *)(a1 + 272))
      || (unsigned char)do_arm_swaps(*(long long *)(a1 + 280)) )
    {
      v2 = 1;
    }
  }
  while ( v2 );
  return 0;
}



// Function: opt_expr @ 0xcc0f
long long opt_expr(long long *a1)
{
  const char *v3; // rax
  long long v4; // rbx
  long long v5; // rbx
  char *v6; // rax
  unsigned char v7; // [rsp+1Eh] [rbp-122h]
  int i; // [rsp+20h] [rbp-120h]
  unsigned int v9; // [rsp+24h] [rbp-11Ch]
  unsigned int v10; // [rsp+28h] [rbp-118h]
  unsigned int v11; // [rsp+2Ch] [rbp-114h]
  long long j; // [rsp+30h] [rbp-110h]
  long long v13; // [rsp+30h] [rbp-110h]
  long long *v14; // [rsp+38h] [rbp-108h]
  long long *v15; // [rsp+38h] [rbp-108h]
  long long (*v16)(const char *, long long, long long); // [rsp+48h] [rbp-F8h]
  long long v17[2]; // [rsp+50h] [rbp-F0h] BYREF
  long long v18[2]; // [rsp+60h] [rbp-E0h] BYREF
  long long v19[23]; // [rsp+70h] [rbp-D0h] BYREF
  unsigned long long v20; // [rsp+128h] [rbp-18h]

  v20 = __readfsqword(0x28u);
  v17[0] = 0;
  v17[1] = 0;
  v18[0] = 0;
  v18[1] = 0;
  v7 = 0;
  if ( !a1 || !*a1 )
    return 0;
  for ( i = 0; i <= 10; ++i )
    sub_C34D(&v19[2 * i]);
  v14 = a1;
  v9 = 3;
  for ( j = *a1; *(long long *)(j + 272); j = *(long long *)(j + 272) )
  {
    v14 = (long long *)(j + 272);
    v9 = *(int *)(j + 20);
  }
  if ( *(int *)(j + 16) != 3 )
    sub_D2D0(j, v9, v14);
  if ( (qword_49980 & 9) != 0 )
  {
    fprintf(stderr, "Normalized Eval Tree:\n");
    sub_EA9F(stderr, *a1, 0);
  }
  v15 = a1;
  v10 = 0;
  if ( *a1 && *(int *)(*a1 + 16LL) == 3 )
    v10 = *(int *)(*a1 + 20LL);
  while ( 1 )
  {
    v13 = *v15;
    if ( !*v15 )
      break;
    if ( *(int *)(v13 + 16) == 3 && v10 != *(int *)(v13 + 20) )
      v13 = sub_D2D0(v13, v10, v15);
    v11 = *(int *)(*(long long *)(v13 + 280) + 16LL);
    v16 = **(long long (***)(const char *, long long, long long))(v13 + 280);
    if ( v11 == 3 )
    {
      v5 = *(long long *)(v13 + 280);
      *(char *)(v5 + 24) = opt_expr(v13 + 280);
      goto LABEL_45;
    }
    if ( v11 > 3 )
    {
      v6 = gettext("oops -- invalid expression type!");
      error(1, 0, v6);
      goto LABEL_45;
    }
    if ( v11 > 1 )
    {
      v4 = *(long long *)(v13 + 280);
      *(char *)(v4 + 24) = opt_expr(v4 + 280);
      goto LABEL_45;
    }
    if ( v10 == 1 || *(char *)(*(long long *)(v13 + 280) + 24LL) == 1 )
      goto LABEL_45;
    if ( sub_C209(*(long long *)(v13 + 280)) )
    {
      if ( (qword_49980 & 8) != 0 )
      {
        fprintf(stderr, "-O%d: promoting cheap predicate ", (unsigned short)word_499A8);
        sub_C2DC(stderr, *(long long *)(v13 + 280));
        fprintf(stderr, " into name_list\n");
      }
      sub_C377(v18, v13, v15);
    }
    else if ( v16 == sub_A2DA )
    {
      sub_C377(v17, v13, v15);
    }
    else if ( (unsigned short)word_499A8 > 1u
           && (*(int *)(*(long long *)(v13 + 280) + 32LL) == 2 || *(int *)(*(long long *)(v13 + 280) + 32LL) == 1)
           && *(char *)(*(long long *)(v13 + 280) + 26LL) != 1
           || (unsigned short)word_499A8 > 2u )
    {
      if ( (qword_49980 & 8) != 0 )
      {
        fprintf(stderr, "-O%d: categorising predicate ", (unsigned short)word_499A8);
        sub_C2DC(stderr, *(long long *)(v13 + 280));
        v3 = (const char *)sub_E95D(*(unsigned int *)(*(long long *)(v13 + 280) + 32LL));
        fprintf(stderr, " by cost (%s)\n", v3);
      }
      sub_C377(&v19[2 * *(unsigned int *)(*(long long *)(v13 + 280) + 32LL)], v13, v15);
    }
    else
    {
LABEL_45:
      if ( *(char *)(*(long long *)(v13 + 280) + 24LL) )
      {
        sub_C6E0((long long)v19, 11, v18, (long long)v17, (long long)v15);
        v7 = 1;
      }
      v15 = (long long *)(v13 + 272);
    }
  }
  sub_C6E0((long long)v19, 11, v18, (long long)v17, (long long)v15);
  return v7;
}



// Function: constrain_rate @ 0xd290
float constrain_rate(float result)
{
  if ( result > 1.0 )
    return 1.0;
  if ( result < 0.0 )
    return 0.0;
  return result;
}



// Function: set_new_parent @ 0xd2d0
long long set_new_parent(long long a1, unsigned int a2, long long *a3)
{
  __m128i v3; // xmm0
  __m128i v4; // xmm0
  long long v7; // [rsp+28h] [rbp-8h]

  v7 = sub_3205A(312);
  *(int *)(v7 + 16) = 3;
  *(int *)(v7 + 20) = a2;
  *(int *)(v7 + 32) = 0;
  if ( a2 == 3 )
  {
    *(long long *)v7 = sub_8D9C;
    *(long long *)(v7 + 8) = "-a";
    v4 = _mm_cvtsi32_si128(*(int *)(a1 + 36));
    *(float *)v4.m128i_i32 = sub_D290(*(float *)v4.m128i_i32);
    *(int *)(v7 + 36) = _mm_cvtsi128_si32(v4);
  }
  else if ( a2 <= 3 )
  {
    if ( a2 == 1 )
    {
      *(long long *)v7 = sub_8FD0;
      *(long long *)(v7 + 8) = ",";
      *(int *)(v7 + 36) = 1065353216;
    }
    else if ( a2 == 2 )
    {
      *(long long *)v7 = sub_9F9E;
      *(long long *)(v7 + 8) = "-o";
      v3 = _mm_cvtsi32_si128(*(int *)(a1 + 36));
      *(float *)v3.m128i_i32 = sub_D290(*(float *)v3.m128i_i32);
      *(int *)(v7 + 36) = _mm_cvtsi128_si32(v3);
    }
  }
  *(long long *)(v7 + 280) = a1;
  *a3 = v7;
  return v7;
}



// Function: merge_pred @ 0xd3fd
long long *merge_pred(long long a1, long long a2, long long *a3)
{
  *(long long *)(a2 + 272) = *a3;
  *a3 = a1;
  return a3;
}



// Function: check_sorted @ 0xd431
long long check_sorted(
        long long a1,
        unsigned long long a2,
        long long a3,
        int (*a4)(unsigned long long, unsigned long long))
{
  unsigned long long i; // [rsp+30h] [rbp-10h]

  for ( i = 1; i < a2; ++i )
  {
    if ( a4(a1 + a3 * i, a3 * (i - 1) + a1) < 0 )
      return 0;
    if ( a4(a1 + a3 * (i - 1), a3 * i + a1) > 0 )
      __assert_fail("result <= 0", "tree.c", 0x3F5u, "check_sorted");
  }
  return 1;
}



// Function: cost_table_comparison @ 0xd52e
int cost_table_comparison(long long *a1, long long *a2)
{
  long long s1; // [rsp+28h] [rbp-18h] BYREF
  long long s2[2]; // [rsp+30h] [rbp-10h] BYREF

  s2[1] = __readfsqword(0x28u);
  s1 = *a1;
  s2[0] = *a2;
  return memcmp(&s1, s2, 8u);
}



// Function: get_pred_cost @ 0xd5a5
long long get_pred_cost(long long a1)
{
  long long v1; // rbx
  char *v2; // rax
  unsigned int v4; // [rsp+10h] [rbp-40h]
  unsigned int v5; // [rsp+14h] [rbp-3Ch]
  int *v6; // [rsp+18h] [rbp-38h]
  long long key[5]; // [rsp+20h] [rbp-30h] BYREF

  key[3] = __readfsqword(0x28u);
  if ( *(char *)(a1 + 26) )
  {
    v4 = 3;
  }
  else if ( *(char *)(a1 + 28) )
  {
    v4 = 1;
  }
  else if ( *(char *)(a1 + 27) )
  {
    v4 = 2;
  }
  else
  {
    v4 = 0;
  }
  if ( *(long long (**)(long long, long long, long long))a1 == sub_9466
    || *(long long (**)(long long, long long, long long))a1 == sub_9497 )
  {
    if ( *(char *)(a1 + 56) )
      v5 = 7;
    else
      v5 = 8;
  }
  else if ( *(long long (**)())a1 == sub_1376D )
  {
    v5 = *(int *)(a1 + 32);
  }
  else
  {
    if ( !dword_49A30 )
    {
      qsort(&off_49020, 0x3Bu, 0x10u, (__compar_fn_t)compar);
      if ( (unsigned char)sub_D431(
                              (long long)&off_49020,
                              0x3Bu,
                              16,
                              (int (*)(unsigned long long, unsigned long long))compar) != 1 )
        error(1, 0, "failed to sort the costlookup array");
      dword_49A30 = 1;
    }
    key[0] = *(long long *)a1;
    v6 = bsearch(key, &off_49020, 0x3Bu, 0x10u, (__compar_fn_t)compar);
    if ( v6 )
    {
      v5 = v6[2];
    }
    else
    {
      v1 = *(long long *)(a1 + 8);
      v2 = gettext(
             "warning: there is no entry in the predicate evaluation cost table for predicate %s; please report this as a bug");
      error(0, 0, v2, v1);
      v5 = 10;
    }
  }
  if ( v5 <= v4 )
    return v4;
  else
    return v5;
}



// Function: estimate_costs @ 0xd7ac
void estimate_costs(long long a1)
{
  if ( a1 )
  {
    estimate_costs(*(long long *)(a1 + 280));
    estimate_costs(*(long long *)(a1 + 272));
    *(int *)(a1 + 32) = sub_D5A5(a1);
  }
}



// Function: get_eval_tree @ 0xd7ff
long long get_eval_tree()
{
  return qword_49A10;
}



// Function: getrate @ 0xd810
float getrate(long long a1)
{
  if ( a1 )
    return *(float *)(a1 + 36);
  else
    return 1.0;
}



// Function: calculate_derived_rates @ 0xd838
float calculate_derived_rates(long long a1)
{
  float result; // xmm0_4
  __m128i v2; // xmm0
  float v3; // [rsp+4h] [rbp-1Ch]
  float v4; // [rsp+4h] [rbp-1Ch]
  float v5; // [rsp+1Ch] [rbp-4h]

  if ( !a1 )
    __assert_fail("NULL != p", "tree.c", 0x484u, "calculate_derived_rates");
  if ( *(long long *)(a1 + 280) )
    calculate_derived_rates(*(long long *)(a1 + 280));
  if ( *(long long *)(a1 + 272) )
    calculate_derived_rates(*(long long *)(a1 + 272));
  if ( *(int *)(a1 + 16) == 5 )
    __assert_fail("p->p_type != CLOSE_PAREN", "tree.c", 0x48Bu, "calculate_derived_rates");
  if ( *(int *)(a1 + 16) == 4 )
    __assert_fail("p->p_type != OPEN_PAREN", "tree.c", 0x48Cu, "calculate_derived_rates");
  switch ( *(int *)(a1 + 16) )
  {
    case 0:
      if ( *(long long *)(a1 + 280) )
        __assert_fail("NULL == p->pred_right", "tree.c", 0x491u, "calculate_derived_rates");
      if ( *(long long *)(a1 + 272) )
        __assert_fail("NULL == p->pred_left", "tree.c", 0x492u, "calculate_derived_rates");
      result = *(float *)(a1 + 36);
      break;
    case 1:
      if ( *(long long *)(a1 + 280) )
        __assert_fail("NULL == p->pred_right", "tree.c", 0x496u, "calculate_derived_rates");
      if ( *(long long *)(a1 + 272) )
        __assert_fail("NULL == p->pred_left", "tree.c", 0x497u, "calculate_derived_rates");
      result = *(float *)(a1 + 36);
      break;
    case 2:
      if ( *(long long (**)(long long, long long, long long))a1 != sub_9B30 )
        __assert_fail("pred_is (p, pred_negate)", "tree.c", 0x49Cu, "calculate_derived_rates");
      if ( *(long long *)(a1 + 272) )
        __assert_fail("NULL == p->pred_left", "tree.c", 0x49Du, "calculate_derived_rates");
      *(float *)(a1 + 36) = 1.0 - *(float *)(*(long long *)(a1 + 280) + 36LL);
      result = *(float *)(a1 + 36);
      break;
    case 3:
      if ( *(long long (**)(long long, long long, long long))a1 == sub_8D9C )
      {
        v3 = sub_D810(*(long long *)(a1 + 280));
        v5 = sub_D810(*(long long *)(a1 + 272)) * v3;
      }
      else if ( *(long long (**)(long long, long long, long long))a1 == sub_8FD0 )
      {
        v5 = 1.0;
      }
      else
      {
        if ( *(long long (**)(long long, long long, long long))a1 != sub_9F9E )
          __assert_fail("0", "tree.c", 0x4B4u, "calculate_derived_rates");
        v4 = sub_D810(*(long long *)(a1 + 280));
        v5 = sub_D810(*(long long *)(a1 + 272)) + v4;
      }
      v2 = _mm_cvtsi32_si128(LODWORD(v5));
      *(float *)v2.m128i_i32 = sub_D290(*(float *)v2.m128i_i32);
      *(int *)(a1 + 36) = _mm_cvtsi128_si32(v2);
      result = *(float *)(a1 + 36);
      break;
    case 4:
    case 5:
      *(int *)(a1 + 36) = 1065353216;
      result = *(float *)(a1 + 36);
      break;
    default:
      __assert_fail("0", "tree.c", 0x4C0u, "calculate_derived_rates");
  }
  return result;
}



// Function: check_normalization @ 0xdc43
long long check_normalization(long long a1, char a2)
{
  long long result; // rax

  if ( a2 && *(int *)(a1 + 16) != 3 )
    __assert_fail("BI_OP == p->p_type", "tree.c", 0x4CEu, "check_normalization");
  if ( *(long long *)(a1 + 272) )
  {
    if ( *(int *)(*(long long *)(a1 + 272) + 16LL) != 3 )
      __assert_fail("BI_OP == p->pred_left->p_type", "tree.c", 0x4D3u, "check_normalization");
    check_normalization(*(long long *)(a1 + 272), 0);
  }
  result = *(long long *)(a1 + 280);
  if ( result )
    return check_normalization(*(long long *)(a1 + 280), 0);
  return result;
}



// Function: build_expression_tree @ 0xdd20
long long build_expression_tree(unsigned int a1, long long a2, int a3, double a4)
{
  long long v4; // rbx
  char *v5; // rax
  long long v6; // rbx
  char *v7; // rax
  char *v8; // rax
  char *v9; // rax
  long long v10; // rbx
  char *v11; // rax
  char *v12; // rax
  char *v13; // rax
  long long v14; // rbx
  char *v15; // rax
  char *v16; // rax
  unsigned int v18; // [rsp+Ch] [rbp-54h] BYREF
  int i; // [rsp+10h] [rbp-50h] BYREF
  int v20; // [rsp+14h] [rbp-4Ch]
  void *ptr; // [rsp+18h] [rbp-48h] BYREF
  long long v22; // [rsp+20h] [rbp-40h]
  long long v23; // [rsp+28h] [rbp-38h]
  long long v24; // [rsp+30h] [rbp-30h]
  long long v25; // [rsp+38h] [rbp-28h]
  long long v26; // [rsp+40h] [rbp-20h]
  unsigned long long v27; // [rsp+48h] [rbp-18h]

  v18 = a1;
  v27 = __readfsqword(0x28u);
  qword_49A08 = 0;
  qword_49A20 = 8LL * a3 + a2;
  for ( i = a3; i < (int)v18 && (unsigned char)sub_FF96(*(long long *)(8LL * i + a2), 1) != 1; ++i )
    ++qword_49A28;
  v22 = sub_1437A("(");
  v23 = sub_1437A(")");
  v24 = sub_1437A("print");
  if ( !v22 )
    __assert_fail("entry_open != NULL", "tree.c", 0x4F6u, "build_expression_tree");
  if ( !v23 )
    __assert_fail("entry_close != NULL", "tree.c", 0x4F7u, "build_expression_tree");
  if ( !v24 )
    __assert_fail("entry_print != NULL", "tree.c", 0x4F8u, "build_expression_tree");
  sub_168B2(v22, a2, &v18);
  *(long long *)(qword_49A18 + 8) = "(";
  *(char *)(qword_49A08 + 41) = 1;
  sub_14262(a2, v18, qword_49A18, qword_49A08);
  sub_AEC7(qword_49A18);
  while ( i < (int)v18 )
  {
    byte_499F8 = 0;
    if ( (unsigned char)sub_FF96(*(long long *)(8LL * i + a2), 0) != 1 )
    {
      v4 = *(long long *)(8LL * i + a2);
      v5 = gettext("paths must precede expression: `%s'");
      error(0, 0, v5, v4);
      if ( !access(*(const char **)(8LL * i + a2), 0) )
      {
        v6 = *(long long *)(qword_49A18 + 8);
        v7 = gettext("possible unquoted pattern after predicate `%s'?");
        error(0, 0, v7, v6);
      }
      exit(1);
    }
    v25 = *(long long *)(8LL * i + a2);
    v26 = sub_1437A(v25);
    if ( !v26 )
    {
      v8 = gettext("unknown predicate `%s'");
      error(1, 0, v8, v25);
    }
    if ( *(int *)v26 != 4 )
      ++i;
    v20 = i;
    if ( (*(unsigned char (**)(long long, long long, int *))(v26 + 16))(v26, a2, &i) != 1 )
    {
      if ( *(long long *)(8LL * i + a2) )
      {
        if ( *(int *)v26 == 4 && v20 == i )
        {
          v9 = gettext("invalid predicate `%s'");
          error(1, 0, v9, v25);
        }
        v10 = *(long long *)(8LL * i + a2);
        v11 = gettext("invalid argument `%s' to `%s'");
        error(1, 0, v11, v10, v25);
      }
      v12 = gettext("missing argument to `%s'");
      error(1, 0, v12, v25);
    }
    *(long long *)(qword_49A18 + 8) = v25;
    if ( v20 == i )
      *(long long *)(qword_49A18 + 48) = 0;
    else
      *(long long *)(qword_49A18 + 48) = *(long long *)(a2 + 8LL * v20);
    sub_AEC7(qword_49A18);
    sub_AEC7(qword_49A08);
  }
  sub_14287(a2, v18, qword_49A18, qword_49A08);
  if ( *(long long *)(qword_49A08 + 264) )
  {
    if ( (unsigned char)sub_FF58(*(long long *)(qword_49A08 + 264)) != 1 )
    {
      ptr = (void *)qword_49A08;
      qword_49A08 = *(long long *)(qword_49A08 + 264);
      sub_AEC7(qword_49A08);
      free(ptr);
    }
    else
    {
      sub_14833(v23, a2, &v18);
      *(long long *)(qword_49A18 + 8) = ")";
      *(char *)(qword_49A18 + 41) = 1;
      sub_AEC7(qword_49A18);
      sub_16EB3(v24, a2, &v18);
      *(long long *)(qword_49A18 + 8) = "-print";
      *(char *)(qword_49A18 + 41) = 1;
      sub_AEC7(qword_49A18);
      sub_AEC7(qword_49A08);
    }
  }
  else
  {
    ptr = (void *)qword_49A08;
    qword_49A18 = *(long long *)(qword_49A08 + 264);
    qword_49A08 = qword_49A18;
    free(ptr);
    sub_16EB3(v24, a2, &v18);
    *(long long *)(qword_49A18 + 8) = "-print";
    sub_AEC7(qword_49A18);
    sub_AEC7(qword_49A08);
  }
  if ( (qword_49980 & 9) != 0 )
  {
    fprintf(stderr, "Predicate List:\n");
    sub_AA62(stderr, qword_49A08);
  }
  sub_13F89(qword_49A08);
  sub_AEC7(qword_49A08);
  ptr = (void *)qword_49A08;
  qword_49A10 = sub_BC44((long long *)&ptr, 0, 0);
  *(float *)&a4 = sub_D838(qword_49A10);
  if ( ptr )
  {
    if ( *(long long (**)())ptr == sub_8EE1 )
    {
      v13 = gettext("you have too many ')'");
      error(1, 0, v13, a4);
    }
    if ( *((long long *)ptr + 1) )
    {
      v14 = *((long long *)ptr + 1);
      v15 = gettext("unexpected extra predicate '%s'");
      error(1, 0, v15, v14, a4);
    }
    v16 = gettext("unexpected extra predicate");
    error(1, 0, v16, a4);
  }
  if ( (qword_49980 & 9) != 0 )
  {
    fprintf(stderr, "Eval Tree:\n");
    sub_EA9F(stderr, qword_49A10, 0);
  }
  sub_D7AC(qword_49A10);
  sub_CC0F(&qword_49A10);
  sub_DC43(qword_49A10, 1);
  sub_CB9F(qword_49A10);
  sub_DC43(qword_49A10, 1);
  if ( (qword_49980 & 9) != 0 )
  {
    fprintf(stderr, "Optimized Eval Tree:\n");
    sub_EA9F(stderr, qword_49A10, 0);
    fprintf(stderr, "Optimized command line:\n");
    sub_AC38(stderr, qword_49A10);
    fprintf(stderr, "\n");
  }
  return qword_49A10;
}



// Function: init_pred_perf @ 0xe60c
long long init_pred_perf(long long a1)
{
  *(long long *)(a1 + 296) = 0;
  *(long long *)(a1 + 288) = *(long long *)(a1 + 296);
  return a1 + 288;
}



// Function: get_new_pred_noarg @ 0xe644
long long get_new_pred_noarg(long long a1)
{
  long long v2; // [rsp+18h] [rbp-8h]

  v2 = sub_E67D(a1);
  if ( v2 )
    *(long long *)(v2 + 48) = 0;
  return v2;
}



// Function: get_new_pred @ 0xe67d
long long get_new_pred(int *a1)
{
  long long v1; // rbx

  if ( !*a1 )
    __assert_fail("entry->type != ARG_OPTION", "tree.c", 0x5CDu, "get_new_pred");
  if ( *a1 == 2 )
    __assert_fail("entry->type != ARG_POSITIONAL_OPTION", "tree.c", 0x5CEu, "get_new_pred");
  v1 = sub_3205A(312);
  if ( qword_49A08 )
    *(long long *)(qword_49A18 + 264) = v1;
  else
    qword_49A08 = v1;
  qword_49A18 = v1;
  *(long long *)(v1 + 304) = a1;
  *(int *)(qword_49A18 + 16) = 0;
  *(int *)(qword_49A18 + 20) = 0;
  *(char *)(qword_49A18 + 26) = 1;
  *(char *)(qword_49A18 + 27) = 1;
  *(int *)(qword_49A18 + 32) = 10;
  *(long long *)(qword_49A18 + 48) = "ThisShouldBeSetToSomethingElse";
  *(char *)(qword_49A18 + 40) = byte_4994F;
  *(int *)(qword_49A18 + 36) = 1065353216;
  sub_E60C(qword_49A18);
  return qword_49A18;
}



// Function: get_new_pred_chk_op @ 0xe7dd
long long get_new_pred_chk_op(int *a1, long long a2)
{
  unsigned int v2; // eax
  char *v3; // rax
  long long result; // rax
  long long v5; // [rsp+18h] [rbp-8h]

  if ( !qword_49A40 )
    qword_49A40 = sub_1437A("and");
  if ( !qword_49A40 )
    __assert_fail("entry_and != NULL", "tree.c", 0x5F8u, "get_new_pred_chk_op");
  if ( qword_49A18 )
  {
    v2 = *(int *)(qword_49A18 + 16);
    if ( v2 == 5 )
    {
LABEL_12:
      v5 = sub_E644(qword_49A40);
      *(long long *)v5 = sub_8D9C;
      *(long long *)(v5 + 8) = "-a";
      *(int *)(v5 + 16) = 3;
      *(int *)(v5 + 20) = 3;
      *(char *)(v5 + 26) = 0;
      *(char *)(v5 + 27) = 0;
      *(char *)(v5 + 28) = 0;
      *(long long *)(v5 + 48) = 0;
      *(long long *)(v5 + 56) = 0;
      *(char *)(v5 + 24) = 0;
      *(char *)(v5 + 25) = 0;
      goto LABEL_13;
    }
    if ( v2 <= 5 )
    {
      if ( v2 )
      {
        if ( v2 != 1 )
          goto LABEL_13;
      }
      else
      {
        v3 = gettext("oops -- invalid default insertion of and!");
        error(1, 0, v3);
      }
      goto LABEL_12;
    }
  }
LABEL_13:
  result = sub_E67D(a1);
  *(long long *)(result + 48) = a2;
  *(long long *)(result + 304) = a1;
  return result;
}



// Function: cost_name @ 0xe95d
const char *cost_name(int a1)
{
  unsigned int i; // [rsp+Ch] [rbp-8h]

  for ( i = 0; i < 0xB; ++i )
  {
    if ( a1 == *((int *)&unk_493E0 + 4 * i) )
      return (&off_493E8)[2 * i];
  }
  return "unknown";
}



// Function: type_name @ 0xe9bd
char *type_name(short a1)
{
  int i; // [rsp+10h] [rbp-4h]

  for ( i = 0; *((short *)&unk_49520 + 8 * i) != 0xFFFF && a1 != *((short *)&unk_49520 + 8 * i); ++i )
    ;
  return (&off_49528)[2 * i];
}



// Function: prec_name @ 0xea2e
char *prec_name(short a1)
{
  int i; // [rsp+10h] [rbp-4h]

  for ( i = 0; *((short *)&unk_494A0 + 8 * i) != 0xFFFF && a1 != *((short *)&unk_494A0 + 8 * i); ++i )
    ;
  return (&off_494A8)[2 * i];
}



// Function: print_tree @ 0xea9f
void print_tree(FILE *a1, long long a2, int a3)
{
  char *v3; // rbx
  char *v4; // rax
  const char *v5; // rbx
  double v6; // r12
  const char *v7; // rax
  const char *v8; // rax
  const char *v9; // rax
  int i; // [rsp+28h] [rbp-18h]
  int j; // [rsp+28h] [rbp-18h]
  int k; // [rsp+28h] [rbp-18h]
  int v14; // [rsp+2Ch] [rbp-14h]

  if ( a2 )
  {
    for ( i = 0; i < a3; ++i )
      fprintf(a1, "    ");
    fprintf(a1, "pred=[");
    sub_C2DC(a1, a2);
    v3 = sub_EA2E(*(int *)(a2 + 20));
    v4 = sub_E9BD(*(int *)(a2 + 16));
    fprintf(a1, "] type=%s prec=%s", v4, v3);
    if ( *(char *)(a2 + 24) )
      v5 = (const char *)&unk_3955C;
    else
      v5 = "no ";
    v6 = *(float *)(a2 + 36);
    v7 = sub_E95D(*(int *)(a2 + 32));
    fprintf(a1, " cost=%s est_success_rate=%#.4g %sside effects ", v7, v6, v5);
    if ( *(char *)(a2 + 26) || *(char *)(a2 + 27) || *(char *)(a2 + 28) )
    {
      v14 = 0;
      fprintf(a1, "Needs ");
      if ( *(char *)(a2 + 26) )
      {
        fprintf(a1, "stat");
        v14 = 1;
      }
      if ( *(char *)(a2 + 28) )
      {
        if ( v14 )
          v8 = ",";
        else
          v8 = (const char *)&unk_3955C;
        fprintf(a1, "%sinode", v8);
        v14 = 1;
      }
      if ( *(char *)(a2 + 27) )
      {
        if ( v14 )
          v9 = ",";
        else
          v9 = (const char *)&unk_3955C;
        fprintf(a1, "%stype", v9);
      }
    }
    fprintf(a1, "\n");
    for ( j = 0; j < a3; ++j )
      fprintf(a1, "    ");
    if ( *(long long *)(a2 + 272) || *(long long *)(a2 + 280) )
    {
      if ( *(long long *)(a2 + 272) )
      {
        fprintf(a1, "left:\n");
        print_tree(a1, *(long long *)(a2 + 272), (unsigned int)(a3 + 1));
      }
      else
      {
        fprintf(a1, "no left.\n");
      }
      for ( k = 0; k < a3; ++k )
        fprintf(a1, "    ");
      if ( *(long long *)(a2 + 280) )
      {
        fprintf(a1, "right:\n");
        print_tree(a1, *(long long *)(a2 + 280), (unsigned int)(a3 + 1));
      }
      else
      {
        fprintf(a1, "no right.\n");
      }
    }
    else
    {
      fprintf(a1, "no children.\n");
    }
  }
}



// Function: insert_primary_withpred @ 0xee5a
long long insert_primary_withpred(long long a1, long long a2, long long a3)
{
  long long result; // rax

  result = sub_E7DD((int *)a1, a3);
  *(long long *)result = a2;
  *(long long *)(result + 8) = *(long long *)(a1 + 8);
  *(long long *)(result + 56) = 0;
  *(int *)(result + 16) = 1;
  *(int *)(result + 20) = 0;
  return result;
}



// Function: insert_primary @ 0xeecc
long long insert_primary(long long a1, long long a2)
{
  if ( !*(long long *)(a1 + 24) )
    __assert_fail("entry->pred_func != NULL", "util.c", 0x7Bu, "insert_primary");
  return sub_EE5A(a1, *(long long *)(a1 + 24), a2);
}



// Function: insert_primary_noarg @ 0xef32
long long insert_primary_noarg(long long a1)
{
  return sub_EECC(a1, 0);
}



// Function: show_valid_debug_options @ 0xef55
int show_valid_debug_options(int a1)
{
  FILE *v1; // rbx
  char *v2; // rax
  int result; // eax
  const char *v4; // rax
  unsigned long long i; // [rsp+18h] [rbp-18h]
  unsigned long long j; // [rsp+18h] [rbp-18h]

  v1 = stdout;
  v2 = gettext("Valid arguments for -D:\n");
  result = fputs(v2, v1);
  if ( a1 )
  {
    for ( i = 0; i <= 8; ++i )
      result = fprintf(stdout, "%-10s %s\n", (&off_495A0)[3 * i], off_495B0[3 * i]);
  }
  else
  {
    for ( j = 0; j <= 8; ++j )
    {
      if ( j )
        v4 = ", ";
      else
        v4 = (const char *)&unk_398CD;
      result = fprintf(stdout, "%s%s", v4, (&off_495A0)[3 * j]);
    }
  }
  return result;
}



// Function: usage @ 0xf071
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

  if ( a1 )
  {
    v1 = qword_4A230;
    v2 = gettext("Try '%s --help' for more information.\n");
    fprintf(stderr, v2, v1);
    exit(a1);
  }
  v3 = qword_4A230;
  v4 = gettext("Usage: %s [-H] [-L] [-P] [-Olevel] [-D debugopts] [path...] [expression]\n");
  fprintf(stdout, v4, v3);
  v5 = stdout;
  v6 = gettext(
         "\n"
         "Default path is the current directory; default expression is -print.\n"
         "Expression may consist of: operators, options, tests, and actions.\n");
  fputs(v6, v5);
  v7 = stdout;
  v8 = gettext(
         "\n"
         "Operators (decreasing precedence; -and is implicit where no others are given):\n"
         "      ( EXPR )   ! EXPR   -not EXPR   EXPR1 -a EXPR2   EXPR1 -and EXPR2\n"
         "      EXPR1 -o EXPR2   EXPR1 -or EXPR2   EXPR1 , EXPR2\n");
  fputs(v8, v7);
  v9 = stdout;
  v10 = gettext("\nPositional options (always true):\n      -daystart -follow -nowarn -regextype -warn\n");
  fputs(v10, v9);
  v11 = stdout;
  v12 = gettext(
          "\n"
          "Normal options (always true, specified before other expressions):\n"
          "      -depth -files0-from FILE -maxdepth LEVELS -mindepth LEVELS\n"
          "       -mount -noleaf -xdev -ignore_readdir_race -noignore_readdir_race\n");
  fputs(v12, v11);
  v13 = stdout;
  v14 = gettext(
          "\n"
          "Tests (N can be +N or -N or N):\n"
          "      -amin N -anewer FILE -atime N -cmin N -cnewer FILE -context CONTEXT\n"
          "      -ctime N -empty -false -fstype TYPE -gid N -group NAME -ilname PATTERN\n"
          "      -iname PATTERN -inum N -iwholename PATTERN -iregex PATTERN\n"
          "      -links N -lname PATTERN -mmin N -mtime N -name PATTERN -newer FILE\n"
          "      -nouser -nogroup -path PATTERN -perm [-/]MODE -regex PATTERN\n"
          "      -readable -writable -executable\n"
          "      -wholename PATTERN -size N[bcwkMG] -true -type [bcdpflsD] -uid N\n"
          "      -used N -user NAME -xtype [bcdpfls]\n");
  fputs(v14, v13);
  v15 = stdout;
  v16 = gettext(
          "\n"
          "Actions:\n"
          "      -delete -print0 -printf FORMAT -fprintf FILE FORMAT -print \n"
          "      -fprint0 FILE -fprint FILE -ls -fls FILE -prune -quit\n"
          "      -exec COMMAND ; -exec COMMAND {} + -ok COMMAND ;\n"
          "      -execdir COMMAND ; -execdir COMMAND {} + -okdir COMMAND ;\n");
  fputs(v16, v15);
  v17 = stdout;
  v18 = gettext("\nOther common options:\n");
  fputs(v18, v17);
  v19 = stdout;
  v20 = gettext("      --help                   display this help and exit\n");
  fputs(v20, v19);
  v21 = stdout;
  v22 = gettext("      --version                output version information and exit\n\n");
  fputs(v22, v21);
  sub_EF55(0);
  v23 = stdout;
  v24 = gettext("\nUse '-D help' for a description of the options, or see find(1)\n\n");
  fputs(v24, v23);
  sub_1D008(stdout, qword_4A230);
  exit(0);
}



// Function: set_stat_placeholders @ 0xf26e
void set_stat_placeholders()
{
  ;
}



// Function: get_statinfo @ 0xf27d
long long get_statinfo(long long a1, long long a2, long long a3)
{
  long long v3; // rbx
  char *v4; // rax
  int *v5; // rax

  if ( byte_499C4 == 1 )
    goto LABEL_9;
  sub_F26E();
  if ( !(unsigned int)qword_49990(a2, a3) )
  {
    if ( !*(int *)(a3 + 24) )
    {
      v3 = sub_2F861(0, (unsigned int)dword_499AC, a2);
      v4 = gettext("WARNING: file %s appears to have mode 0000");
      error(0, 0, v4, v3);
      status = 1;
    }
LABEL_9:
    byte_499C4 = 1;
    byte_499C5 = 1;
    dword_499C8 = *(int *)(a3 + 24);
    return 0;
  }
  if ( byte_4994E != 1 || *__errno_location() != 2 )
  {
    v5 = __errno_location();
    sub_10A13((unsigned int)*v5, a1);
  }
  return 0xFFFFFFFFLL;
}



// Function: get_info @ 0xf388
long long get_info(long long a1, long long a2, char *a3)
{
  char v4; // [rsp+2Fh] [rbp-1h]

  v4 = 0;
  if ( a3[26] && byte_499C4 != 1 )
  {
    v4 = 1;
  }
  else if ( a3[27] && byte_499C5 != 1 )
  {
    v4 = 1;
  }
  else if ( a3[28] )
  {
    if ( *(long long *)(a2 + 8) )
    {
      if ( byte_499C5 != 1 || (*(int *)(a2 + 24) & 0xF000) == 0x4000 )
        v4 = 1;
    }
    else
    {
      v4 = 1;
    }
  }
  if ( v4 && (unsigned int)sub_F27D(a1, (long long)s1, a2) )
    return 0xFFFFFFFFLL;
  else
    return 0;
}



// Function: check_nofollow @ 0xf45a
char check_nofollow()
{
  float v0; // xmm0_4
  struct utsname name; // [rsp+10h] [rbp-190h] BYREF
  unsigned long long v3; // [rsp+198h] [rbp-8h]

  v3 = __readfsqword(0x28u);
  if ( uname(&name) )
    return 1;
  v0 = atof(name.release);
  if ( !strcmp("Linux", name.sysname) )
    return v0 >= 2.2;
  return strcmp("FreeBSD", name.sysname) || v0 >= 3.0999999;
}



// Function: exec_cb @ 0xf540
long long exec_cb(long long a1)
{
  sub_19F5A(a1 + 8, a1 + 96);
  return 0;
}



// Function: do_exec @ 0xf57a
void *do_exec(long long a1)
{
  void *result; // rax

  sub_1A9DD(*(long long *)(a1 + 192), sub_F540, a1);
  result = ptr;
  if ( *(void **)(a1 + 192) != ptr )
  {
    sub_2FE52(*(long long *)(a1 + 192));
    free(*(void **)(a1 + 192));
    *(long long *)(a1 + 192) = 0;
    return (void *)a1;
  }
  return result;
}



// Function: do_complete_pending_execdirs @ 0xf5fa
long long do_complete_pending_execdirs(long long a1)
{
  long long result; // rax

  if ( a1 )
  {
    if ( !byte_499E8 )
      __assert_fail("state.execdirs_outstanding", "util.c", 0x18Cu, "do_complete_pending_execdirs");
    do_complete_pending_execdirs(*(long long *)(a1 + 272));
    if ( *(long long (**)(long long, long long, long long))a1 == sub_9497
      || *(long long (**)(long long, long long, long long))a1 == sub_9F27 )
    {
      if ( *(char *)(a1 + 56) )
      {
        if ( *(int *)(a1 + 208) )
          sub_F57A(a1 + 56);
      }
    }
    return do_complete_pending_execdirs(*(long long *)(a1 + 280));
  }
  return result;
}



// Function: complete_pending_execdirs @ 0xf6cb
long long complete_pending_execdirs()
{
  long long result; // rax
  long long v1; // rax

  result = (unsigned char)byte_499E8;
  if ( byte_499E8 )
  {
    v1 = sub_D7FF();
    result = sub_F5FA(v1);
    byte_499E8 = 0;
  }
  return result;
}



// Function: complete_pending_execs @ 0xf6f5
long long complete_pending_execs(long long a1)
{
  long long result; // rax

  if ( a1 )
  {
    complete_pending_execs(*(long long *)(a1 + 272));
    if ( *(long long (**)(long long, long long, long long))a1 == sub_9466 && *(char *)(a1 + 56) )
    {
      if ( *(int *)(a1 + 208) )
        sub_19F5A(a1 + 64, a1 + 152);
    }
    return complete_pending_execs(*(long long *)(a1 + 280));
  }
  return result;
}



// Function: record_initial_cwd @ 0xf78b
void record_initial_cwd()
{
  void *v0; // r12
  const char *v1; // rbx
  char *v2; // r13
  int *v3; // rax

  ptr = (void *)sub_31BC2(16);
  if ( (unsigned int)sub_2FD99(ptr) )
  {
    if ( *(int *)ptr < 0 && *((long long *)ptr + 1) )
      v0 = (void *)*((long long *)ptr + 1);
    else
      v0 = &unk_398CD;
    if ( *(int *)ptr < 0 && *((long long *)ptr + 1) )
      v1 = ": ";
    else
      v1 = (const char *)&unk_398CD;
    v2 = gettext("Failed to save initial working directory%s%s");
    v3 = __errno_location();
    error(1, *v3, v2, v1, v0);
  }
}



// Function: cleanup_initial_cwd @ 0xf861
void cleanup_initial_cwd()
{
  void *v0; // r12
  const char *v1; // rbx
  char *v2; // r13
  int *v3; // rax

  if ( (unsigned int)sub_2FE17(ptr) )
  {
    if ( *(int *)ptr < 0 && *((long long *)ptr + 1) )
      v0 = (void *)*((long long *)ptr + 1);
    else
      v0 = &unk_398CD;
    if ( *(int *)ptr < 0 && *((long long *)ptr + 1) )
      v1 = ": ";
    else
      v1 = (const char *)&unk_398CD;
    v2 = gettext("Failed to restore initial working directory%s%s");
    v3 = __errno_location();
    error(0, *v3, v2, v1, v0);
    _exit(1);
  }
  sub_2FE52(ptr);
  free(ptr);
  ptr = 0;
}



// Function: traverse_tree @ 0xf959
long long traverse_tree(long long a1, void (*a2)(long long))
{
  long long result; // rax

  if ( *(long long *)(a1 + 272) )
    traverse_tree(*(long long *)(a1 + 272), a2);
  a2(a1);
  result = *(long long *)(a1 + 280);
  if ( result )
    return traverse_tree(*(long long *)(a1 + 280), a2);
  return result;
}



// Function: undangle_file_pointers @ 0xf9d1
long long (*undangle_file_pointers(long long a1))(const char *a1, long long a2, long long a3)
{
  long long (*result)(const char *, long long, long long); // rax

  if ( *(long long (**)(const char *, long long, long long))a1 == sub_954F
    || *(long long (**)())a1 == sub_1376D
    || *(long long (**)(int, int, long long))a1 == sub_94E6
    || (result = *(long long (**)(const char *, long long, long long))a1,
        *(long long (**)(const char *, long long, long long))a1 == sub_959F) )
  {
    *(long long *)(a1 + 64) = 0;
    return (long long (*)(const char *, long long, long long))a1;
  }
  return result;
}



// Function: cleanup @ 0xfa38
long long cleanup()
{
  long long result; // rax
  int *v1; // rax
  long long v2; // [rsp+8h] [rbp-8h]

  v2 = sub_D7FF();
  if ( v2 )
  {
    sub_F959(v2, (void (*)(long long))sub_F6F5);
    sub_F6CB();
  }
  sub_10CAC(qword_499F0);
  if ( v2 )
    sub_F959(v2, (void (*)(long long))sub_F9D1);
  sub_F861();
  if ( (unsigned char)sub_1B5F4() )
  {
    sub_1B61E();
    sub_1B5C4();
  }
  result = sub_1E1A8(stdout);
  if ( (int)result == -1 )
  {
    v1 = __errno_location();
    return sub_10A77((unsigned int)*v1, "standard output");
  }
  return result;
}



// Function: fallback_stat @ 0xfae2
int fallback_stat(const char *a1, struct stat *a2, int a3)
{
  int v3; // eax

  v3 = *__errno_location();
  if ( v3 != 2 && v3 != 20 )
    return a3;
  if ( (qword_49980 & 2) != 0 )
    fprintf(stderr, "fallback_stat(): stat(%s) failed; falling back on lstat()\n", a1);
  return fstatat(dword_499D8, a1, a2, 256);
}



// Function: optionh_stat @ 0xfb5c
int optionh_stat(const char *a1, struct stat *a2)
{
  int v3; // [rsp+1Ch] [rbp-4h]

  if ( dword_499D8 != -100 && dword_499D8 < 0 )
    __assert_fail("state.cwd_dir_fd >= 0", "util.c", 0x25Du, "optionh_stat");
  sub_F26E();
  if ( dword_499C0 )
    return fstatat(dword_499D8, a1, a2, 256);
  v3 = fstatat(dword_499D8, a1, a2, 0);
  if ( v3 )
    return sub_FAE2(a1, a2, v3);
  else
    return 0;
}



// Function: optionl_stat @ 0xfc21
int optionl_stat(const char *a1, struct stat *a2)
{
  int v3; // [rsp+1Ch] [rbp-4h]

  if ( dword_499D8 != -100 && dword_499D8 < 0 )
    __assert_fail("state.cwd_dir_fd >= 0", "util.c", 0x27Cu, "optionl_stat");
  sub_F26E();
  v3 = fstatat(dword_499D8, a1, a2, 0);
  if ( v3 )
    return sub_FAE2(a1, a2, v3);
  else
    return 0;
}



// Function: optionp_stat @ 0xfcc0
int optionp_stat(const char *a1, struct stat *a2)
{
  if ( dword_499D8 < 0 && dword_499D8 != -100 )
    __assert_fail("(state.cwd_dir_fd >= 0) || (state.cwd_dir_fd==AT_FDCWD)", "util.c", 0x28Du, "optionp_stat");
  sub_F26E();
  return fstatat(dword_499D8, a1, a2, 256);
}



// Function: debug_stat @ 0xfd39
int debug_stat(const char *a1, struct stat *a2)
{
  ++qword_49A48;
  fprintf(stderr, "debug_stat (%s)\n", a1);
  if ( dword_49988 == 2 )
    return sub_FB5C(a1, a2);
  if ( (unsigned int)dword_49988 > 2 )
    goto LABEL_8;
  if ( dword_49988 )
  {
    if ( dword_49988 == 1 )
      return sub_FC21(a1, a2);
LABEL_8:
    __assert_fail("0", "util.c", 0x2A5u, "debug_stat");
  }
  return sub_FCC0(a1, a2);
}



// Function: following_links @ 0xfe03
bool following_links()
{
  if ( dword_49988 == 1 )
    return 1;
  if ( dword_49988 == 2 )
    return dword_499C0 == 0;
  return 0;
}



// Function: digest_mode @ 0xfe38
long long digest_mode(int *a1, long long a2, long long a3, long long a4, char a5)
{
  if ( *a1 )
  {
    if ( (*a1 & 0xF000) == 0xA000 && sub_FE03() )
    {
      if ( (unsigned int)sub_F27D(a2, a3, a4) )
        return 0;
      dword_499C8 = *(int *)(a4 + 24);
      *a1 = dword_499C8;
      byte_499C5 = 1;
    }
    else
    {
      byte_499C5 = 1;
      dword_499C8 = *a1;
      *(int *)(a4 + 24) = dword_499C8;
    }
  }
  else if ( a5 )
  {
    byte_499C4 = 0;
    byte_499C5 = 0;
    dword_499C8 = 0;
  }
  else
  {
    if ( (unsigned int)sub_F27D(a2, a3, a4) )
      return 0;
    *a1 = *(int *)(a4 + 24);
    dword_499C8 = *a1;
    byte_499C5 = 1;
  }
  return 1;
}



// Function: default_prints @ 0xff58
long long default_prints(long long a1)
{
  while ( a1 )
  {
    if ( *(char *)(a1 + 25) )
      return 0;
    a1 = *(long long *)(a1 + 264);
  }
  return 1;
}



// Function: looks_like_expression @ 0xff96
long long looks_like_expression(char *a1, char a2)
{
  long long result; // rax

  switch ( *a1 )
  {
    case '!':
    case '(':
      result = a1[1] == 0;
      break;
    case ')':
    case ',':
      if ( a1[1] )
        result = 0;
      else
        result = a2 == 0;
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



// Function: process_debug_options @ 0x1004b
unsigned long long process_debug_options(char *a1)
{
  long long v1; // rbx
  char *v2; // rax
  char *v3; // rax
  char v5; // [rsp+17h] [rbp-39h]
  char *save_ptr; // [rsp+18h] [rbp-38h] BYREF
  char *s2; // [rsp+20h] [rbp-30h]
  unsigned long long i; // [rsp+28h] [rbp-28h]
  char delim[2]; // [rsp+36h] [rbp-1Ah] BYREF
  unsigned long long v10; // [rsp+38h] [rbp-18h]

  v10 = __readfsqword(0x28u);
  save_ptr = 0;
  strcpy(delim, ",");
  v5 = 1;
  for ( s2 = strtok_r(a1, delim, &save_ptr); s2; s2 = strtok_r(0, delim, &save_ptr) )
  {
    v5 = 0;
    for ( i = 0; i <= 8; ++i )
    {
      if ( !strcmp((&off_495A0)[3 * i], s2) )
      {
        qword_49980 |= *((int *)&unk_495A8 + 6 * i);
        break;
      }
    }
    if ( i > 8 )
    {
      v1 = sub_2F861(0, (unsigned int)dword_499AC, a1);
      v2 = gettext("Ignoring unrecognised debug flag %s");
      error(0, 0, v2, v1);
    }
  }
  if ( v5 )
  {
    v3 = gettext("Empty argument to the -D option.");
    error(0, 0, v3);
    sub_F071(1);
  }
  if ( (qword_49980 & 0x10) != 0 )
  {
    sub_EF55(1);
    exit(0);
  }
  return v10 - __readfsqword(0x28u);
}



// Function: process_optimisation_option @ 0x101fd
unsigned long long process_optimisation_option(const char *a1)
{
  char *v1; // rax
  char *v2; // rax
  char *v3; // rax
  char *v4; // rax
  char *v5; // rbx
  int *v6; // rax
  char *v7; // rax
  int v9; // [rsp+14h] [rbp-2Ch]
  char *endptr; // [rsp+18h] [rbp-28h] BYREF
  unsigned long long v11; // [rsp+20h] [rbp-20h]
  unsigned long long v12; // [rsp+28h] [rbp-18h]

  v12 = __readfsqword(0x28u);
  if ( !*a1 )
  {
    v1 = gettext("The -O option must be immediately followed by a decimal integer");
    error(1, 0, v1);
  }
  if ( ((*__ctype_b_loc())[*(unsigned char *)a1] & 0x800) == 0 )
  {
    v2 = gettext("Please specify a decimal number immediately after -O");
    error(1, 0, v2);
  }
  v9 = *__errno_location();
  *__errno_location() = 0;
  v11 = strtoul(a1, &endptr, 10);
  if ( !v11 && a1 == endptr )
  {
    v3 = gettext("Please specify a decimal number immediately after -O");
    error(1, 0, v3);
  }
  if ( *endptr )
  {
    v4 = gettext("Invalid optimisation level %s");
    error(1, 0, v4, a1);
  }
  if ( v11 == -1 && *__errno_location() )
  {
    v5 = gettext("Invalid optimisation level %s");
    v6 = __errno_location();
    error(1, *v6, v5, a1);
  }
  if ( v11 > 0xFFFF )
  {
    v7 = gettext("Optimisation level %lu is too high.  If you want to find files very quickly, consider using GNU locate.");
    error(1, 0, v7, v11);
  }
  word_499A8 = v11;
  *__errno_location() = v9;
  return v12 - __readfsqword(0x28u);
}



// Function: process_leading_options @ 0x103e8
long long process_leading_options(int a1, long long a2)
{
  char *v2; // rax
  int i; // [rsp+18h] [rbp-8h]
  unsigned int v5; // [rsp+1Ch] [rbp-4h]

  for ( i = 1; ; ++i )
  {
    v5 = i;
    if ( i >= a1 )
      break;
    if ( !strcmp("-H", *(const char **)(8LL * i + a2)) )
    {
      sub_141B0(2);
    }
    else if ( !strcmp("-L", *(const char **)(8LL * i + a2)) )
    {
      sub_141B0(1);
    }
    else if ( !strcmp("-P", *(const char **)(8LL * i + a2)) )
    {
      sub_141B0(0);
    }
    else
    {
      if ( !strcmp("--", *(const char **)(8LL * i + a2)) )
        return (unsigned int)(i + 1);
      if ( !strcmp("-D", *(const char **)(8LL * i + a2)) )
      {
        if ( a1 <= i + 1 )
        {
          v2 = gettext("Missing argument after the -D option.");
          error(0, 0, v2);
          sub_F071(1);
        }
        sub_1004B(*(char **)(8 * ++i + a2));
      }
      else
      {
        if ( strncmp("-O", *(const char **)(8LL * i + a2), 2u) )
          return (unsigned int)i;
        sub_101FD((const char *)(*(long long *)(8LL * i + a2) + 2LL));
      }
    }
  }
  return v5;
}



// Function: now @ 0x10601
__time_t now()
{
  time_t v1; // [rsp+8h] [rbp-38h]
  struct timeval tv; // [rsp+20h] [rbp-20h] BYREF
  unsigned long long v3; // [rsp+38h] [rbp-8h]

  v3 = __readfsqword(0x28u);
  if ( !gettimeofday(&tv, 0) )
    return tv.tv_sec;
  v1 = time(0);
  if ( v1 == -1 )
    __assert_fail("t != (time_t)-1", "util.c", 0x3D2u, "now");
  return v1;
}



// Function: set_option_defaults @ 0x106bd
long long set_option_defaults(long long a1)
{
  long long v1; // rdx
  char *v2; // rax

  if ( getenv("POSIXLY_CORRECT") )
    *(char *)(a1 + 17) = 1;
  else
    *(char *)(a1 + 17) = 0;
  *(char *)(a1 + 88) = sub_F45A();
  *(int *)(a1 + 92) = 0;
  if ( isatty(0) )
    *(char *)(a1 + 16) = 1;
  else
    *(char *)(a1 + 16) = 0;
  *(char *)(a1 + 15) = 0;
  if ( *(char *)(a1 + 17) )
    *(char *)(a1 + 16) = 0;
  *(char *)a1 = 1;
  *(char *)(a1 + 1) = 0;
  *(int *)(a1 + 8) = -1;
  *(int *)(a1 + 4) = *(int *)(a1 + 8);
  *(long long *)(a1 + 24) = sub_10601();
  *(long long *)(a1 + 32) = v1;
  *(long long *)(a1 + 40) = *(long long *)(a1 + 24) - 86400LL;
  *(long long *)(a1 + 48) = *(long long *)(a1 + 32);
  *(char *)(a1 + 56) = 0;
  *(char *)(a1 + 13) = 0;
  *(char *)(a1 + 14) = 0;
  if ( *(char *)(a1 + 17) )
    *(int *)(a1 + 60) = 512;
  else
    *(int *)(a1 + 60) = 1024;
  *(long long *)(a1 + 64) = 0;
  *(short *)(a1 + 104) = 2;
  if ( getenv("FIND_BLOCK_SIZE") )
  {
    v2 = gettext(
           "The environment variable FIND_BLOCK_SIZE is not supported, the only thing that affects the block size is the "
           "POSIXLY_CORRECT environment variable");
    error(1, 0, v2);
  }
  *(char *)(a1 + 12) = 0;
  sub_141B0(0);
  *(int *)(a1 + 108) = 8;
  *(long long *)(a1 + 112) = 0;
  *(char *)(a1 + 120) = 0;
  return a1;
}



// Function: apply_predicate @ 0x10873
long long apply_predicate(long long a1, long long a2, long long a3)
{
  ++*(long long *)(a3 + 288);
  if ( (*(char *)(a3 + 26) || *(char *)(a3 + 27) || *(char *)(a3 + 28))
    && (unsigned int)sub_F388(a1, a2, (char *)a3) )
  {
    return 0;
  }
  if ( !(*(unsigned char (**)(long long, long long, long long))a3)(a1, a2, a3) )
    return 0;
  ++*(long long *)(a3 + 296);
  return 1;
}



// Function: is_exec_in_local_dir @ 0x10933
long long is_exec_in_local_dir(long long (*a1)(long long a1, long long a2, long long a3))
{
  return a1 == sub_9497 || a1 == sub_9F27;
}



// Function: safely_quote_err_filename @ 0x1096a
long long safely_quote_err_filename(unsigned int a1, long long a2)
{
  return sub_2F861(a1, (unsigned int)dword_499AC, a2);
}



// Function: report_file_err @ 0x10995
void report_file_err(int a1, int a2, char a3, long long a4)
{
  const char *v4; // rax

  if ( a3 != 1 || byte_499F8 != 1 )
  {
    v4 = (const char *)sub_1096A(0, a4);
    error(a1, a2, "%s", v4);
    status = 1;
  }
  if ( a3 )
    byte_499F8 = 1;
}



// Function: nonfatal_target_file_error @ 0x10a13
void nonfatal_target_file_error(int a1, long long a2)
{
  sub_10995(0, a1, 1, a2);
}



// Function: fatal_target_file_error @ 0x10a44
void fatal_target_file_error(int a1, long long a2)
{
  sub_10995(1, a1, 1, a2);
  abort();
}



// Function: nonfatal_nontarget_file_error @ 0x10a77
void nonfatal_nontarget_file_error(int a1, long long a2)
{
  sub_10995(0, a1, 0, a2);
}



// Function: fatal_nontarget_file_error @ 0x10aa8
void fatal_nontarget_file_error(int a1, long long a2)
{
  byte_499F8 = 0;
  sub_10995(1, a1, 0, a2);
  abort();
}



// Function: entry_comparator @ 0x10ae2
long long entry_comparator(long long *a1, long long *a2)
{
  return a1[1] == a2[1] && *a1 == *a2;
}



// Function: entry_free @ 0x10b3b
void entry_free(long long *a1)
{
  long long v1; // rbx
  int *v2; // rax

  if ( a1[3] && (unsigned int)sub_1DAD5(a1[3]) )
  {
    v1 = a1[2];
    v2 = __errno_location();
    sub_10AA8(*v2, v1);
  }
  free((void *)a1[2]);
  free(a1);
}



// Function: entry_hashfunc @ 0x10bb1
unsigned long long entry_hashfunc(long long *a1, unsigned long long a2)
{
  return (*a1 ^ a1[1]) % a2;
}



// Function: sharefile_init @ 0x10be9
void **sharefile_init(const char *a1)
{
  void **ptr; // [rsp+18h] [rbp-8h]

  ptr = (void **)malloc(0x10u);
  if ( ptr )
  {
    *ptr = strdup(a1);
    if ( *ptr )
    {
      ptr[1] = (void *)sub_21BB1(11, 0, sub_10BB1, sub_10AE2, sub_10B3B);
      if ( ptr[1] )
        return ptr;
      free(*ptr);
      free(ptr);
    }
    else
    {
      free(ptr);
    }
  }
  return 0;
}



// Function: sharefile_destroy @ 0x10cac
long long sharefile_destroy(long long a1)
{
  free(*(void **)a1);
  return sub_21E31(*(long long *)(a1 + 8));
}



// Function: sharefile_fopen @ 0x10ce6
long long sharefile_fopen(long long *a1, const char *a2)
{
  int fd; // [rsp+10h] [rbp-C0h]
  int v4; // [rsp+14h] [rbp-BCh]
  void *ptr; // [rsp+20h] [rbp-B0h]
  long long v6; // [rsp+28h] [rbp-A8h]
  struct stat buf; // [rsp+30h] [rbp-A0h] BYREF
  unsigned long long v8; // [rsp+C8h] [rbp-8h]

  v8 = __readfsqword(0x28u);
  ptr = malloc(0x20u);
  if ( !ptr )
    return 0;
  *((long long *)ptr + 2) = strdup(a2);
  if ( *((long long *)ptr + 2) )
  {
    *((long long *)ptr + 3) = sub_1E49E(a2, *a1);
    if ( *((long long *)ptr + 3) )
    {
      fd = fileno(*((FILE **)ptr + 3));
      if ( fd < 0 )
        __assert_fail("fd >= 0", "sharefile.c", 0xA6u, "sharefile_fopen");
      sub_1D5BA((unsigned int)fd, 1);
      if ( fstat(fd, &buf) >= 0 )
      {
        *(long long *)ptr = buf.st_dev;
        *((long long *)ptr + 1) = buf.st_ino;
        v6 = sub_214FC(a1[1], ptr);
        if ( v6 )
        {
          sub_10B3B((long long *)ptr);
          return *(long long *)(v6 + 24);
        }
        else if ( sub_228F2(a1[1], ptr) )
        {
          return *((long long *)ptr + 3);
        }
        else
        {
          v4 = *__errno_location();
          sub_10B3B((long long *)ptr);
          *__errno_location() = v4;
          return 0;
        }
      }
      else
      {
        sub_10B3B((long long *)ptr);
        return 0;
      }
    }
    else
    {
      sub_10B3B((long long *)ptr);
      return 0;
    }
  }
  else
  {
    free(ptr);
    return 0;
  }
}



// Function: make_segment @ 0x10f4b
long long make_segment(long long *a1, const char *a2, int a3, unsigned int a4, char a5, char a6, long long a7)
{
  long long v7; // rbx
  char *v9; // rax
  char *v10; // rax
  char *v11; // rax
  char *v12; // rax
  char *v13; // rax
  char *v14; // rax
  char *v15; // rax
  char *v16; // rax
  char *v17; // rax
  char *v18; // rax
  char *v19; // rax
  char *v20; // rax
  char *v21; // rax
  unsigned int v26; // [rsp+24h] [rbp-1Ch]
  char *desta; // [rsp+28h] [rbp-18h]
  char *dest; // [rsp+28h] [rbp-18h]

  v26 = 0;
  switch ( a5 )
  {
    case '{':
      __assert_fail("format_char != '{'", "print.c", 0x4Eu, "make_segment");
    case '[':
      __assert_fail("format_char != '['", "print.c", 0x4Fu, "make_segment");
    case '(':
      __assert_fail("format_char != '('", "print.c", 0x50u, "make_segment");
  }
  *a1 = sub_31BC2(32);
  *(int *)*a1 = a4;
  *(char *)(*a1 + 4) = a5;
  *(char *)(*a1 + 5) = a6;
  *(long long *)(*a1 + 24) = 0;
  *(int *)(*a1 + 16) = a3;
  v7 = *a1;
  *(long long *)(v7 + 8) = sub_31BC2(a3 + 2LL);
  desta = *(char **)(v7 + 8);
  strncpy(desta, a2, a3);
  dest = &desta[a3];
  if ( a4 > 1 )
  {
    if ( a4 != 2 )
      __assert_fail("kind == KIND_FORMAT", "print.c", 0x69u, "make_segment");
    switch ( a5 )
    {
      case '%':
        v9 = dest++;
        *v9 = 37;
        break;
      case 'A':
      case 'B':
      case 'C':
      case 'F':
      case 'M':
      case 'T':
      case 'a':
      case 'c':
      case 'g':
      case 's':
      case 't':
      case 'u':
        *(char *)(a7 + 26) = 1;
        v26 = 3;
        v13 = dest++;
        *v13 = 115;
        break;
      case 'D':
      case 'G':
      case 'U':
      case 'b':
      case 'k':
      case 'n':
        *(char *)(a7 + 26) = 1;
        v26 = 3;
        v19 = dest++;
        *v19 = 115;
        break;
      case 'H':
        v18 = dest++;
        *v18 = 115;
        break;
      case 'P':
      case 'f':
      case 'h':
      case 'p':
        v16 = dest++;
        *v16 = 115;
        break;
      case 'S':
        *(char *)(a7 + 26) = 1;
        v26 = 3;
        v14 = dest++;
        *v14 = 103;
        break;
      case 'Y':
        *(char *)(a7 + 26) = 1;
        v26 = 2;
        v15 = dest++;
        *v15 = 115;
        break;
      case 'Z':
        v26 = 5;
        v17 = dest++;
        *v17 = 115;
        break;
      case 'd':
        v20 = dest++;
        *v20 = 100;
        break;
      case 'i':
        *(char *)(a7 + 28) = 1;
        v26 = 1;
        v12 = dest++;
        *v12 = 115;
        break;
      case 'l':
        *(char *)(a7 + 26) = 1;
        v26 = 4;
        v10 = dest++;
        *v10 = 115;
        break;
      case 'm':
        v21 = dest++;
        *v21 = 111;
        *(char *)(a7 + 26) = 1;
        v26 = 3;
        break;
      case 'y':
        *(char *)(a7 + 27) = 1;
        v26 = 2;
        v11 = dest++;
        *v11 = 115;
        break;
      default:
        break;
    }
    *dest = 0;
    if ( v26 > *(int *)(a7 + 32) )
      *(int *)(a7 + 32) = v26;
    return *a1 + 24;
  }
  else
  {
    if ( a5 )
      __assert_fail("0 == format_char", "print.c", 0x61u, "make_segment");
    if ( a6 )
      __assert_fail("0 == aux_format_char", "print.c", 0x62u, "make_segment");
    *dest = 0;
    return *a1 + 24;
  }
}



// Function: is_octal_char @ 0x1133c
long long is_octal_char(char a1)
{
  return a1 > 47 && a1 <= 55;
}



// Function: parse_octal_escape @ 0x11366
long long parse_octal_escape(long long a1, long long *a2)
{
  unsigned int v2; // ebx
  int i; // r12d
  long long v5; // [rsp+18h] [rbp-18h]

  v5 = 0;
  v2 = 0;
  for ( i = 0; i <= 2 && sub_1133C(*(char *)(a1 + v5)); ++i )
    v2 = 8 * v2 + *(char *)(a1 + v5++) - 48;
  *a2 = v5 - 1;
  return v2;
}



// Function: parse_escape_char @ 0x113f2
long long parse_escape_char(char a1)
{
  char v2; // [rsp+13h] [rbp-1h]

  v2 = 0;
  switch ( a1 )
  {
    case '\\':
      v2 = 92;
      break;
    case 'a':
      v2 = 7;
      break;
    case 'b':
      v2 = 8;
      break;
    case 'f':
      v2 = 12;
      break;
    case 'n':
      v2 = 10;
      break;
    case 'r':
      v2 = 13;
      break;
    case 't':
      v2 = 9;
      break;
    case 'v':
      v2 = 11;
      break;
    default:
      return (unsigned int)v2;
  }
  return (unsigned int)v2;
}



// Function: get_format_flags_length @ 0x11467
long long get_format_flags_length(long long a1)
{
  long long v2; // [rsp+18h] [rbp-8h]

  v2 = 0;
  do
    ++v2;
  while ( *(char *)(a1 + v2) && strchr("-+ #", *(char *)(a1 + v2)) );
  while ( ((*__ctype_b_loc())[*(unsigned char *)(a1 + v2)] & 0x800) != 0 )
    ++v2;
  if ( *(char *)(a1 + v2) == 46 )
  {
    ++v2;
    while ( ((*__ctype_b_loc())[*(unsigned char *)(a1 + v2)] & 0x800) != 0 )
      ++v2;
  }
  return v2;
}



// Function: get_format_specifer_length @ 0x11545
long long get_format_specifer_length(char a1)
{
  if ( strchr("abcdDfFgGhHiklmMnpPsStuUyYZ%", a1) )
    return 1;
  if ( strchr("ABCT", a1) )
    return 2;
  return 0;
}



// Function: insert_fprintf @ 0x1159f
long long insert_fprintf(long long *a1, long long a2, const char *a3)
{
  long long *v3; // rax
  long long v4; // rbx
  long long v5; // rbx
  char *v7; // rax
  char v8; // al
  unsigned int v9; // ebx
  char *v10; // rax
  char *v11; // rax
  char v12; // al
  unsigned int v13; // ebx
  char *v14; // rax
  unsigned int v15; // ebx
  char *v16; // rax
  unsigned int v17; // ebx
  char *v18; // rax
  char v19; // [rsp+2Fh] [rbp-51h]
  long long v20; // [rsp+30h] [rbp-50h] BYREF
  char *v21; // [rsp+38h] [rbp-48h]
  char *i; // [rsp+40h] [rbp-40h]
  long long *v23; // [rsp+48h] [rbp-38h]
  long long v24; // [rsp+50h] [rbp-30h]
  long long v25; // [rsp+58h] [rbp-28h]
  long long v26; // [rsp+60h] [rbp-20h]
  unsigned long long v27; // [rsp+68h] [rbp-18h]

  v27 = __readfsqword(0x28u);
  v21 = (char *)a3;
  v26 = sub_EE5A(a2, (long long)sub_1376D, (long long)a3);
  *(char *)(v26 + 25) = 1;
  *(char *)(v26 + 24) = *(char *)(v26 + 25);
  v3 = (long long *)v26;
  v4 = a1[1];
  *(long long *)(v26 + 56) = *a1;
  v3[8] = v4;
  v5 = a1[3];
  v3[9] = a1[2];
  v3[10] = v5;
  v3[11] = a1[4];
  *(char *)(v26 + 27) = 0;
  *(char *)(v26 + 26) = 0;
  *(int *)(v26 + 32) = 0;
  v23 = (long long *)(v26 + 56);
  *(long long *)(v26 + 56) = 0;
  for ( i = v21; ; ++i )
  {
    if ( !*i )
    {
      if ( i > v21 )
        sub_10F4B(v23, v21, (int)i - (int)v21, 0, 0, 0, v26);
      return 1;
    }
    if ( *i == 92 && i[1] == 99 )
      break;
    if ( *i == 92 )
    {
      v24 = 1;
      if ( i[1] )
      {
        if ( sub_1133C(i[v24]) )
        {
          v20 = 0;
          v8 = sub_11366((long long)&i[v24], &v20);
          *i = v8;
          v24 += v20;
        }
        else
        {
          v19 = sub_113F2(i[v24]);
          if ( !v19 )
          {
            v9 = i[v24];
            v10 = gettext("warning: unrecognized escape `\\%c'");
            error(0, 0, v10, v9);
            i += v24;
            continue;
          }
          *i = v19;
        }
      }
      else
      {
        v7 = gettext("warning: escape `\\' followed by nothing at all");
        error(0, 0, v7);
        --v24;
      }
      v23 = (long long *)sub_10F4B(v23, v21, (int)i - (int)v21 + 1, 0, 0, 0, v26);
      v21 = &i[v24 + 1];
      i += v24;
    }
    else if ( *i == 37 )
    {
      if ( !i[1] )
      {
        v11 = gettext("error: %s at end of format string");
        error(1, 0, v11, i);
      }
      if ( i[1] == 37 )
        v25 = 1;
      else
        v25 = sub_11467((long long)i);
      i += v25;
      v25 = sub_11545(*i);
      if ( v25 && i[v25 - 1] )
      {
        if ( v25 == 2 )
          v12 = i[1];
        else
          v12 = 0;
        v23 = (long long *)sub_10F4B(v23, v21, (int)i - (int)v21, 2u, *i, v12, v26);
        i += v25 - 1;
      }
      else
      {
        if ( strchr("{[(", *i) )
        {
          v13 = *i;
          v14 = gettext("error: the format directive `%%%c' is reserved for future use");
          error(1, 0, v14, v13);
        }
        if ( v25 != 2 || i[1] )
        {
          v17 = *i;
          v18 = gettext("warning: unrecognized format directive `%%%c'");
          error(0, 0, v18, v17);
        }
        else
        {
          v15 = *i;
          v16 = gettext("warning: format directive `%%%c' should be followed by another character");
          error(0, 0, v16, v15);
        }
        v23 = (long long *)sub_10F4B(v23, v21, (int)i + 1 - (int)v21, 0, 0, 0, v26);
      }
      v21 = i + 1;
    }
  }
  sub_10F4B(v23, v21, (int)i - (int)v21, 1u, 0, 0, v26);
  if ( *(char *)(v26 + 26) && *(int *)(v26 + 32) <= 2u )
    *(int *)(v26 + 32) = 3;
  return 1;
}



// Function: scan_for_digit_differences @ 0x11b14
long long scan_for_digit_differences(long long a1, long long a2, long long *a3, long long *a4)
{
  char v7; // [rsp+27h] [rbp-9h]
  long long i; // [rsp+28h] [rbp-8h]

  v7 = 0;
  for ( i = 0; *(char *)(a1 + i) && *(char *)(a2 + i); ++i )
  {
    if ( *(char *)(a1 + i) != *(char *)(a2 + i) )
    {
      if ( ((*__ctype_b_loc())[*(unsigned char *)(a1 + i)] & 0x800) == 0
        || ((*__ctype_b_loc())[*(unsigned char *)(a2 + i)] & 0x800) == 0 )
      {
        return 0;
      }
      if ( v7 != 1 )
      {
        *a3 = i;
        *a4 = 1;
        v7 = 1;
      }
      else
      {
        if ( i - *a3 != *a4 )
          return 0;
        ++*a4;
      }
    }
  }
  return !*(char *)(a1 + i) && !*(char *)(a2 + i);
}



// Function: do_time_format @ 0x11c89
char *do_time_format(const char *a1, long long a2, const void *a3, size_t a4)
{
  size_t v4; // rax
  size_t v5; // rax
  long long v6; // rbx
  long long v7; // rbx
  long long v8; // rbx
  long long v12; // [rsp+20h] [rbp-90h] BYREF
  long long v13; // [rsp+28h] [rbp-88h] BYREF
  char *format; // [rsp+30h] [rbp-80h]
  size_t v15; // [rsp+38h] [rbp-78h]
  long long v16; // [rsp+40h] [rbp-70h]
  char *v17; // [rsp+48h] [rbp-68h]
  long long v18; // [rsp+50h] [rbp-60h]
  size_t n; // [rsp+58h] [rbp-58h]
  struct tm v20; // [rsp+60h] [rbp-50h] BYREF
  unsigned long long v21; // [rsp+98h] [rbp-18h]

  v21 = __readfsqword(0x28u);
  v4 = strlen(a1);
  format = (char *)sub_31BC2(v4 + 2);
  *format = 95;
  v5 = strlen(a1);
  memcpy(format + 1, a1, v5 + 1);
  v6 = *(long long *)(a2 + 8);
  *(long long *)&v20.tm_sec = *(long long *)a2;
  *(long long *)&v20.tm_hour = v6;
  v7 = *(long long *)(a2 + 24);
  *(long long *)&v20.tm_mon = *(long long *)(a2 + 16);
  *(long long *)&v20.tm_wday = v7;
  v8 = *(long long *)(a2 + 40);
  *(long long *)&v20.tm_isdst = *(long long *)(a2 + 32);
  v20.tm_gmtoff = v8;
  v20.tm_zone = *(const char **)(a2 + 48);
  if ( v20.tm_sec <= 10 )
    v20.tm_sec += 11;
  else
    v20.tm_sec -= 11;
  if ( !qword_49A60 )
  {
    maxsize = 1;
    qword_49A60 = (char *)sub_31BC2(1);
  }
  while ( 1 )
  {
    v15 = strftime(qword_49A60, maxsize, format, (const struct tm *)a2);
    if ( v15 )
    {
      if ( v15 < maxsize )
        break;
    }
    qword_49A60 = (char *)sub_31DBF(qword_49A60, &maxsize, 1);
  }
  v12 = 0;
  v13 = 0;
  v16 = v15 + a4 + 1;
  qword_49A60 = (char *)sub_31C2C(qword_49A60, v16);
  maxsize = v16;
  v17 = (char *)sub_31BC2(v16);
  strftime(v17, maxsize, format, &v20);
  if ( sub_11B14((long long)qword_49A60, (long long)v17, &v12, &v13)
    && v13 == 2
    && ((*__ctype_b_loc())[(unsigned char)qword_49A60[v12 + 2]] & 0x800) == 0 )
  {
    v18 = v12 + v13;
    n = v15 - (v12 + v13) + 1;
    if ( v16 != a4 + v15 + 1 )
      __assert_fail("end_of_seconds + ns_size + suffix_len == final_len", "print.c", 0x225u, "do_time_format");
    memmove(&qword_49A60[v18 + a4], &qword_49A60[v18], n);
    memcpy(&qword_49A60[v12 + v13], a3, a4);
  }
  free(format);
  free(v17);
  return qword_49A60 + 1;
}



// Function: format_date @ 0x1201f
char *format_date(time_t a1, long long a2, int a3)
{
  time_t v4; // rax
  size_t v5; // rax
  size_t v6; // rax
  time_t timer; // [rsp+10h] [rbp-A0h] BYREF
  long long v9; // [rsp+18h] [rbp-98h]
  int v10; // [rsp+20h] [rbp-90h]
  int v11; // [rsp+24h] [rbp-8Ch]
  char *v12; // [rsp+28h] [rbp-88h]
  struct tm *v13; // [rsp+30h] [rbp-80h]
  char *v14; // [rsp+38h] [rbp-78h]
  time_t v15; // [rsp+40h] [rbp-70h]
  size_t v16; // [rsp+48h] [rbp-68h]
  unsigned long long v17; // [rsp+50h] [rbp-60h]
  size_t v18; // [rsp+58h] [rbp-58h]
  char dest[12]; // [rsp+64h] [rbp-4Ch] BYREF
  char s[40]; // [rsp+70h] [rbp-40h] BYREF
  unsigned long long v21; // [rsp+98h] [rbp-18h]

  timer = a1;
  v9 = a2;
  v21 = __readfsqword(0x28u);
  v10 = 0;
  v11 = 0;
  if ( a3 == 43 )
  {
    strcpy(dest, "%Y-%m-%d+%T");
    v11 = 1;
  }
  else
  {
    dest[0] = 37;
    dest[1] = a3;
    dest[2] = 0;
    v11 = (unsigned int)(a3 - 64) <= 0x18 && ((0x1180001uLL >> ((unsigned char)a3 - 64)) & 1) != 0;
  }
  if ( v11 )
  {
    v10 = snprintf(s, 0x20u, ".%09ld0", v9);
    if ( v10 > 31 )
      __assert_fail("charsprinted < NS_BUF_LEN", "print.c", 0x299u, "format_date");
  }
  else
  {
    v10 = 0;
    s[0] = 0;
  }
  if ( a3 != 64 )
  {
    v13 = localtime(&timer);
    if ( v13 )
    {
      v14 = sub_11C89(dest, (long long)v13, s, v10);
      if ( v14 )
        return v14;
    }
  }
  v15 = timer;
  if ( timer >= 0 )
    v4 = v15;
  else
    v4 = -v15;
  v12 = (char *)sub_22EB1(v4, &unk_49A81, 0, 1, 1);
  if ( v12 <= (char *)&unk_49A80 )
    __assert_fail("p > buf", "print.c", 0x2B9u, "format_date");
  if ( v12 >= byte_49D97 )
    __assert_fail("p < (buf + (sizeof buf))", "print.c", 0x2BAu, "format_date");
  if ( timer < 0 )
    *--v12 = 45;
  if ( v11 )
  {
    v16 = strlen(v12);
    v17 = v12 - (char *)&unk_49A80 + v16;
    if ( v17 > 0x316 )
      __assert_fail("sizeof buf > used", "print.c", 0x2C7u, "format_date");
    v18 = 790 - v17;
    v5 = strlen(s);
    if ( v18 <= v5 )
      error(0, 0, "charsprinted=%ld but remaining=%lu: ns_buf=%s", v10, v18, s);
    v6 = strlen(s);
    if ( v18 <= v6 )
      __assert_fail("strlen (ns_buf) < remaining", "print.c", 0x2D0u, "format_date");
    strcat(v12, s);
  }
  return v12;
}



// Function: ctime_format @ 0x123c8
char *ctime_format(time_t a1, long long a2)
{
  time_t timer; // [rsp+0h] [rbp-20h] BYREF
  long long v4; // [rsp+8h] [rbp-18h]
  int v5; // [rsp+14h] [rbp-Ch]
  struct tm *v6; // [rsp+18h] [rbp-8h]

  timer = a1;
  v4 = a2;
  v6 = localtime(&timer);
  if ( !v6 )
    return sub_1201F(timer, v4, 64);
  if ( v6->tm_wday < 0 )
    __assert_fail("ptm->tm_wday >= 0", "print.c", 0x2EDu, "ctime_format");
  if ( v6->tm_wday > 6 )
    __assert_fail("ptm->tm_wday < 7", "print.c", 0x2EEu, "ctime_format");
  if ( v6->tm_mon < 0 )
    __assert_fail("ptm->tm_mon >= 0", "print.c", 0x2EFu, "ctime_format");
  if ( v6->tm_mon > 11 )
    __assert_fail("ptm->tm_mon < 12", "print.c", 0x2F0u, "ctime_format");
  if ( v6->tm_hour < 0 )
    __assert_fail("ptm->tm_hour >= 0", "print.c", 0x2F1u, "ctime_format");
  if ( v6->tm_hour > 23 )
    __assert_fail("ptm->tm_hour < 24", "print.c", 0x2F2u, "ctime_format");
  if ( v6->tm_min > 59 )
    __assert_fail("ptm->tm_min < 60", "print.c", 0x2F3u, "ctime_format");
  if ( v6->tm_sec > 61 )
    __assert_fail("ptm->tm_sec <= 61", "print.c", 0x2F4u, "ctime_format");
  v5 = snprintf(
         byte_49DA0,
         0x400u,
         "%3s %3s %2d %02d:%02d:%02d.%09ld0 %04d",
         off_49680[v6->tm_wday],
         off_496C0[v6->tm_mon],
         v6->tm_mday,
         v6->tm_hour,
         v6->tm_min,
         v6->tm_sec,
         v4,
         v6->tm_year + 1900);
  if ( v5 > 1023 )
    __assert_fail("nout < TIME_BUF_LEN", "print.c", 0x302u, "ctime_format");
  return byte_49DA0;
}



// Function: file_sparseness @ 0x12693
long long file_sparseness(long long a1)
{
  double v1; // xmm0_8

  if ( *(long long *)(a1 + 48) )
  {
    v1 = 512.0 * (double)(int)*(long long *)(a1 + 64) / (double)(int)*(long long *)(a1 + 48);
  }
  else if ( *(long long *)(a1 + 64) )
  {
    if ( *(long long *)(a1 + 64) >= 0 )
      v1 = INFINITY;
    else
      v1 = -INFINITY;
  }
  else
  {
    v1 = 1.0;
  }
  return *(long long *)&v1;
}



// Function: checked_fprintf @ 0x1272c
unsigned long long checked_fprintf(long long a1, const char *a2, long long a3, long long a4, long long a5, long long a6, ...)
{
  long long v6; // rbx
  int *v7; // rax
  gcc_va_list arg; // [rsp+20h] [rbp-E0h] BYREF
  unsigned long long v10; // [rsp+38h] [rbp-C8h]
  long long v11; // [rsp+50h] [rbp-B0h]
  long long v12; // [rsp+58h] [rbp-A8h]
  long long v13; // [rsp+60h] [rbp-A0h]
  long long v14; // [rsp+68h] [rbp-98h]

  va_start(arg, a6);
  v11 = a3;
  v12 = a4;
  v13 = a5;
  v14 = a6;
  v10 = __readfsqword(0x28u);
  arg[0].gp_offset = 16;
  if ( vfprintf(*(FILE **)(a1 + 8), a2, arg) < 0 )
  {
    v6 = *(long long *)(a1 + 16);
    v7 = __errno_location();
    sub_10A77(*v7, v6);
  }
  return v10 - __readfsqword(0x28u);
}



// Function: checked_print_quoted @ 0x12839
void checked_print_quoted(long long a1, const char *a2, long long a3)
{
  long long v3; // rbx
  int *v4; // rax

  if ( (int)sub_1C643(*(long long *)(a1 + 8), *(long long *)(a1 + 32), *(unsigned char *)(a1 + 24), a2, a3) < 0 )
  {
    v3 = *(long long *)(a1 + 16);
    v4 = __errno_location();
    sub_10A77(*v4, v3);
  }
}



// Function: checked_fwrite @ 0x128a9
void checked_fwrite(const void *a1, size_t a2, size_t a3, long long a4)
{
  long long v4; // rbx
  int *v5; // rax

  if ( fwrite(a1, a2, a3, *(FILE **)(a4 + 8)) < a3 )
  {
    v4 = *(long long *)(a4 + 16);
    v5 = __errno_location();
    sub_10A77(*v5, v4);
  }
}



// Function: checked_fflush @ 0x12910
void checked_fflush(long long a1)
{
  long long v1; // rbx
  int *v2; // rax

  if ( (unsigned int)sub_1E1A8(*(long long *)(a1 + 8)) )
  {
    v1 = *(long long *)(a1 + 16);
    v2 = __errno_location();
    sub_10A77(*v2, v1);
  }
}



// Function: mode_to_filetype @ 0x12955
const char *mode_to_filetype(int a1)
{
  switch ( a1 )
  {
    case 32768:
      return "f";
    case 16384:
      return "d";
    case 40960:
      return "l";
    case 49152:
      return "s";
    case 24576:
      return "b";
    case 8192:
      return "c";
    case 4096:
      return "p";
  }
  return "U";
}



// Function: do_fprintf @ 0x129e7
unsigned long long do_fprintf(long long a1, long long a2, char *a3, long long a4, long long a5, long long a6, double a7)
{
  unsigned int v7; // eax
  time_t v8; // rax
  long long v9; // rdx
  char *v10; // rax
  long long v11; // r8
  long long v12; // r9
  long long v13; // rax
  long long v14; // r8
  long long v15; // r9
  time_t v16; // rax
  long long v17; // rdx
  char *v18; // rax
  long long v19; // r8
  long long v20; // r9
  long long v21; // rax
  long long v22; // r8
  long long v23; // r9
  long long v24; // rax
  long long v25; // r8
  long long v26; // r9
  long long v27; // rax
  long long v28; // r8
  long long v29; // r9
  long long v30; // r8
  long long v31; // r9
  long long v32; // rax
  long long v33; // r8
  long long v34; // r9
  long long v35; // rax
  long long v36; // r8
  long long v37; // r9
  int *v38; // rax
  long long v39; // r8
  long long v40; // r9
  long long v41; // rax
  long long v42; // r8
  long long v43; // r9
  long long v44; // rax
  long long v45; // r8
  long long v46; // r9
  long long v47; // r8
  long long v48; // r9
  time_t v49; // rax
  long long v50; // rdx
  char *v51; // rax
  long long v52; // r8
  long long v53; // r9
  long long v54; // r8
  long long v55; // r9
  long long v56; // rax
  long long v57; // r8
  long long v58; // r9
  long long v59; // r8
  long long v60; // r9
  long long v61; // r8
  long long v62; // r9
  const char *v63; // rbx
  int *v64; // rax
  const char *v65; // rax
  long long v66; // r8
  long long v67; // r9
  const char *v68; // rax
  long long v69; // r8
  long long v70; // r9
  long long v71; // r8
  long long v72; // r9
  long long v73; // r12
  char *v74; // rbx
  int *v75; // rax
  char *v76; // rax
  long long v80; // [rsp+38h] [rbp-388h] BYREF
  char *v81; // [rsp+40h] [rbp-380h]
  char *i; // [rsp+48h] [rbp-378h]
  void *v83; // [rsp+50h] [rbp-370h]
  long long *v84; // [rsp+58h] [rbp-368h]
  char *s; // [rsp+60h] [rbp-360h]
  long long *v86; // [rsp+68h] [rbp-358h]
  void *ptr; // [rsp+70h] [rbp-350h]
  void *dest; // [rsp+78h] [rbp-348h]
  struct stat buf; // [rsp+80h] [rbp-340h] BYREF
  char v90[664]; // [rsp+110h] [rbp-2B0h] BYREF
  unsigned long long v91; // [rsp+3A8h] [rbp-18h]

  v91 = __readfsqword(0x28u);
  v7 = *(int *)a2;
  if ( *(int *)a2 == 2 )
  {
    switch ( *(char *)(a2 + 4) )
    {
      case 0:
        v76 = gettext("error: %s at end of format string");
        error(1, 0, v76, "%");
        return v91 - __readfsqword(0x28u);
      case 0x25:
        goto LABEL_6;
      case 0x44:
        v21 = sub_22EB1(*(long long *)a4, v90, 0, 1, 1);
        sub_1272C(a1, *(const char **)(a2 + 8), v21, *(long long *)(a2 + 8), v22, v23);
        return v91 - __readfsqword(0x28u);
      case 0x46:
        v24 = sub_13A18(a4, a3);
        sub_12839(a1, *(const char **)(a2 + 8), v24);
        return v91 - __readfsqword(0x28u);
      case 0x47:
        goto LABEL_18;
      case 0x48:
        dest = (void *)sub_31BC2(dword_499DC + 1);
        memcpy(dest, a3, dword_499DC);
        *((char *)dest + dword_499DC) = 0;
        sub_1272C(a1, *(const char **)(a2 + 8), (long long)dest, *(long long *)(a2 + 8), v30, v31);
        free(dest);
        return v91 - __readfsqword(0x28u);
      case 0x4D:
        sub_1E472(a4, v90);
        v90[10] = 0;
        sub_1272C(a1, *(const char **)(a2 + 8), (long long)v90, *(long long *)(a2 + 8), v39, v40);
        return v91 - __readfsqword(0x28u);
      case 0x50:
        if ( dword_499C0 <= 0 )
        {
          v81 = (char *)&unk_3AA45;
        }
        else
        {
          v81 = &a3[dword_499DC];
          if ( *v81 == 47 )
            ++v81;
        }
        sub_12839(a1, *(const char **)(a2 + 8), (long long)v81);
        return v91 - __readfsqword(0x28u);
      case 0x53:
        sub_12693(a4);
        sub_1272C(a1, *(const char **)(a2 + 8), a1, *(long long *)(a2 + 8), v47, v48, a7);
        return v91 - __readfsqword(0x28u);
      case 0x55:
        goto LABEL_54;
      case 0x59:
        if ( (*(int *)(a4 + 24) & 0xF000) != 0xA000 )
          goto LABEL_64;
        if ( fstatat(dword_499D8, s1, &buf, 0) )
        {
          if ( *__errno_location() == 2 || *__errno_location() == 20 )
          {
            sub_1272C(a1, *(const char **)(a2 + 8), (long long)"N", *(long long *)(a2 + 8), v59, v60, a4);
          }
          else if ( *__errno_location() == 40 )
          {
            sub_1272C(a1, *(const char **)(a2 + 8), (long long)"L", *(long long *)(a2 + 8), v61, v62);
          }
          else
          {
            sub_1272C(a1, *(const char **)(a2 + 8), (long long)"?", *(long long *)(a2 + 8), v61, v62);
            v63 = (const char *)sub_1096A(0, (long long)a3);
            v64 = __errno_location();
            error(0, *v64, "%s", v63);
          }
        }
        else
        {
          v65 = sub_12955(buf.st_mode & 0xF000);
          sub_1272C(a1, *(const char **)(a2 + 8), (long long)v65, *(long long *)(a2 + 8), v66, v67);
        }
        break;
      case 0x5A:
        if ( (int)qword_499A0((unsigned int)dword_499D8, s1, &v80) >= 0 )
        {
          sub_1272C(a1, *(const char **)(a2 + 8), v80, *(long long *)(a2 + 8), v71, v72);
          freecon(v80);
        }
        else
        {
          sub_1272C(a1, *(const char **)(a2 + 8), (long long)&unk_3AA45, *(long long *)(a2 + 8), v71, v72);
          v73 = sub_1096A(0, (long long)a3);
          v74 = gettext("getfilecon failed: %s");
          v75 = __errno_location();
          error(0, *v75, v74, v73);
          status = 1;
        }
        return v91 - __readfsqword(0x28u);
      case 0x61:
        v8 = sub_30A4C(a4);
        v10 = sub_123C8(v8, v9);
        sub_1272C(a1, *(const char **)(a2 + 8), (long long)v10, *(long long *)(a2 + 8), v11, v12);
        return v91 - __readfsqword(0x28u);
      case 0x62:
        v13 = sub_22EB1(*(long long *)(a4 + 64), v90, 0, 512, 512);
        sub_1272C(a1, *(const char **)(a2 + 8), v13, *(long long *)(a2 + 8), v14, v15);
        return v91 - __readfsqword(0x28u);
      case 0x63:
        v16 = sub_30A66(a4);
        v18 = sub_123C8(v16, v17);
        sub_1272C(a1, *(const char **)(a2 + 8), (long long)v18, *(long long *)(a2 + 8), v19, v20);
        return v91 - __readfsqword(0x28u);
      case 0x64:
        sub_1272C(a1, *(const char **)(a2 + 8), (unsigned int)dword_499C0, *(long long *)(a2 + 8), a5, a6);
        return v91 - __readfsqword(0x28u);
      case 0x66:
        ptr = (void *)sub_1D85C(a3);
        sub_12839(a1, *(const char **)(a2 + 8), (long long)ptr);
        free(ptr);
        return v91 - __readfsqword(0x28u);
      case 0x67:
        v86 = (long long *)getgrgid(*(int *)(a4 + 32));
        if ( v86 )
        {
          *(char *)(*(long long *)(a2 + 8) + *(int *)(a2 + 16)) = 115;
          sub_1272C(a1, *(const char **)(a2 + 8), *v86, *(long long *)(a2 + 8), v25, v26);
        }
        else
        {
LABEL_18:
          v27 = sub_22EB1(*(unsigned int *)(a4 + 32), v90, 0, 1, 1);
          sub_1272C(a1, *(const char **)(a2 + 8), v27, *(long long *)(a2 + 8), v28, v29);
        }
        break;
      case 0x68:
        s = (char *)sub_321C9(a3);
        for ( i = &s[strlen(s) - 1]; s <= i && *i == 47; --i )
          ;
        if ( s < i && i[1] == 47 )
          i[1] = 0;
        i = strrchr(s, 47);
        if ( i )
        {
          *i = 0;
          sub_12839(a1, *(const char **)(a2 + 8), (long long)s);
        }
        else
        {
          sub_12839(a1, *(const char **)(a2 + 8), (long long)".");
        }
        free(s);
        break;
      case 0x69:
        v32 = sub_22EB1(*(long long *)(a4 + 8), v90, 0, 1, 1);
        sub_1272C(a1, *(const char **)(a2 + 8), v32, *(long long *)(a2 + 8), v33, v34);
        break;
      case 0x6B:
        v35 = sub_22EB1(*(long long *)(a4 + 64), v90, 0, 512, 1024);
        sub_1272C(a1, *(const char **)(a2 + 8), v35, *(long long *)(a2 + 8), v36, v37);
        break;
      case 0x6C:
        v83 = 0;
        if ( (*(int *)(a4 + 24) & 0xF000) == 0xA000 )
        {
          v83 = (void *)sub_1D06D((unsigned int)dword_499D8, s1);
          if ( !v83 )
          {
            v38 = __errno_location();
            sub_10A13(*v38, (long long)a3);
            status = 1;
          }
        }
        if ( v83 )
          sub_12839(a1, *(const char **)(a2 + 8), (long long)v83);
        else
          sub_12839(a1, *(const char **)(a2 + 8), (long long)&unk_3AA45);
        free(v83);
        break;
      case 0x6D:
        sub_1272C(a1, *(const char **)(a2 + 8), *(int *)(a4 + 24) & 0xFFF, *(long long *)(a2 + 8), a5, a6);
        break;
      case 0x6E:
        v41 = sub_22EB1(*(long long *)(a4 + 16), v90, 0, 1, 1);
        sub_1272C(a1, *(const char **)(a2 + 8), v41, *(long long *)(a2 + 8), v42, v43);
        break;
      case 0x70:
        sub_12839(a1, *(const char **)(a2 + 8), (long long)a3);
        break;
      case 0x73:
        v44 = sub_22EB1(*(long long *)(a4 + 48), v90, 0, 1, 1);
        sub_1272C(a1, *(const char **)(a2 + 8), v44, *(long long *)(a2 + 8), v45, v46);
        break;
      case 0x74:
        v49 = sub_30A80(a4);
        v51 = sub_123C8(v49, v50);
        sub_1272C(a1, *(const char **)(a2 + 8), (long long)v51, *(long long *)(a2 + 8), v52, v53);
        break;
      case 0x75:
        v84 = (long long *)getpwuid(*(int *)(a4 + 28));
        if ( v84 )
        {
          *(char *)(*(long long *)(a2 + 8) + *(int *)(a2 + 16)) = 115;
          sub_1272C(a1, *(const char **)(a2 + 8), *v84, *(long long *)(a2 + 8), v54, v55);
        }
        else
        {
LABEL_54:
          v56 = sub_22EB1(*(unsigned int *)(a4 + 28), v90, 0, 1, 1);
          sub_1272C(a1, *(const char **)(a2 + 8), v56, *(long long *)(a2 + 8), v57, v58);
        }
        break;
      case 0x79:
LABEL_64:
        v68 = sub_12955(*(int *)(a4 + 24) & 0xF000);
        sub_1272C(a1, *(const char **)(a2 + 8), (long long)v68, *(long long *)(a2 + 8), v69, v70);
        break;
      default:
        return v91 - __readfsqword(0x28u);
    }
  }
  else if ( v7 <= 2 )
  {
    if ( v7 )
    {
      if ( v7 == 1 )
      {
        sub_128A9(*(const void **)(a2 + 8), 1u, *(int *)(a2 + 16), a1);
        sub_12910(a1);
      }
    }
    else
    {
LABEL_6:
      sub_128A9(*(const void **)(a2 + 8), 1u, *(int *)(a2 + 16), a1);
    }
  }
  return v91 - __readfsqword(0x28u);
}



// Function: pred_fprintf @ 0x1376d
long long pred_fprintf(char *a1, long long a2, long long a3, double a4, long long a5, long long a6, long long a7)
{
  int v7; // eax
  long long v8; // r8
  long long v9; // r9
  long long v10; // rdx
  long long v11; // rdx
  long long v12; // rdx
  long long v13; // rdx
  char *v14; // rax
  long long v15; // r8
  long long v16; // r9
  int v18; // [rsp+2Ch] [rbp-24h]
  long long i; // [rsp+30h] [rbp-20h]
  long long v20; // [rsp+38h] [rbp-18h]
  time_t v21; // [rsp+40h] [rbp-10h]
  long long v22; // [rsp+48h] [rbp-8h]

  v20 = a3 + 56;
  for ( i = *(long long *)(a3 + 56); i; i = *(long long *)(i + 24) )
  {
    if ( *(int *)i == 2 && *(char *)(i + 5) )
    {
      v7 = *(char *)(i + 4);
      if ( v7 == 84 )
      {
        v21 = sub_30A80(a2);
        v22 = v13;
        v18 = 1;
      }
      else
      {
        if ( v7 > 84 )
          goto LABEL_17;
        if ( v7 == 67 )
        {
          v21 = sub_30A66(a2);
          v22 = v12;
          v18 = 1;
        }
        else
        {
          if ( v7 > 67 )
            goto LABEL_17;
          if ( v7 == 65 )
          {
            v21 = sub_30A4C(a2);
            v22 = v10;
            v18 = 1;
          }
          else
          {
            if ( v7 != 66 )
LABEL_17:
              __assert_fail("0", "print.c", 0x518u, "pred_fprintf");
            v21 = sub_30A9A(a2);
            v22 = v11;
            v18 = *(char *)(i + 5) == 64 || v11 >= 0;
          }
        }
      }
      if ( v18 )
      {
        v14 = sub_1201F(v21, v22, *(char *)(i + 5));
        sub_1272C(v20, *(const char **)(i + 8), (long long)v14, *(long long *)(i + 8), v15, v16);
      }
      else
      {
        sub_1272C(v20, *(const char **)(i + 8), (long long)&unk_3AA45, *(long long *)(i + 8), v8, v9);
      }
    }
    else
    {
      sub_129E7(v20, i, a1, a2, a6, a7, a4);
    }
  }
  return 1;
}



// Function: free_file_system_list @ 0x1395f
long long free_file_system_list(long long a1)
{
  long long result; // rax
  long long v3; // [rsp+18h] [rbp-8h]

  while ( a1 )
  {
    v3 = *(long long *)(a1 + 48);
    sub_26F67(a1);
    result = v3;
    a1 = v3;
  }
  return result;
}



// Function: get_file_system_list @ 0x1399c
long long get_file_system_list(unsigned char a1)
{
  if ( qword_4A1A0 && byte_4A1A8 != 1 && a1 )
  {
    sub_1395F(qword_4A1A0);
    qword_4A1A0 = 0;
  }
  if ( !qword_4A1A0 )
  {
    qword_4A1A0 = sub_262D1(a1);
    byte_4A1A8 = a1;
  }
  return qword_4A1A0;
}



// Function: filesystem_type @ 0x13a18
void *filesystem_type(long long *a1, long long a2)
{
  if ( qword_4A1B0 )
  {
    if ( byte_4A1B8 && *a1 == qword_4A1C0 )
      return qword_4A1B0;
    free(qword_4A1B0);
  }
  qword_4A1C0 = *a1;
  qword_4A1B0 = (void *)sub_13BE0(a1, a2, &byte_4A1B8);
  return qword_4A1B0;
}



// Function: is_used_fs_type @ 0x13aa6
long long is_used_fs_type(const char *a1)
{
  long long i; // [rsp+10h] [rbp-10h]
  long long v3; // [rsp+18h] [rbp-8h]

  if ( !strcmp("afs", a1) )
    return 1;
  v3 = sub_1399C(0);
  if ( !v3 )
    return 1;
  for ( i = v3; i; i = *(long long *)(i + 48) )
  {
    if ( !strcmp(a1, *(const char **)(i + 24)) )
      return 1;
  }
  return 0;
}



// Function: set_fstype_devno @ 0x13b3b
long long set_fstype_devno(long long a1)
{
  long long v2[20]; // [rsp+10h] [rbp-A0h] BYREF

  v2[19] = __readfsqword(0x28u);
  if ( *(long long *)(a1 + 32) != -1 )
    return 0;
  sub_F26E();
  if ( (unsigned int)qword_49990(*(long long *)(a1 + 8), v2) )
    return 0xFFFFFFFFLL;
  *(long long *)(a1 + 32) = v2[0];
  return 0;
}



// Function: file_system_type_uncached @ 0x13be0
long long file_system_type_uncached(long long *a1, long long a2, bool *a3)
{
  char *v3; // rax
  char *v4; // rax
  long long i; // [rsp+20h] [rbp-20h]
  long long v8; // [rsp+28h] [rbp-18h]
  long long v9; // [rsp+30h] [rbp-10h]
  long long v10; // [rsp+38h] [rbp-8h]

  v8 = 0;
  v10 = sub_1399C(1u);
  if ( !v10 )
  {
    v3 = gettext("Cannot read mounted file system list");
    error(1, 0, v3);
  }
  v9 = 0;
  for ( i = v10; i; i = *(long long *)(i + 48) )
  {
    if ( strcmp(*(const char **)(i + 24), "ignore") && !(unsigned int)sub_13B3B(i) && *(long long *)(i + 32) == *a1 )
      v8 = i;
  }
  if ( v8 )
    v9 = sub_321C9(*(long long *)(v8 + 24));
  *a3 = v9 != 0;
  if ( v9 )
    return v9;
  v4 = gettext("unknown");
  return sub_321C9(v4);
}



// Function: get_mounted_devices @ 0x13cfc
void *get_mounted_devices(long long *a1)
{
  long long v2; // [rsp+18h] [rbp-38h] BYREF
  long long v3; // [rsp+20h] [rbp-30h]
  long long i; // [rsp+28h] [rbp-28h]
  void *ptr; // [rsp+30h] [rbp-20h]
  long long v6; // [rsp+38h] [rbp-18h]
  void *v7; // [rsp+40h] [rbp-10h]
  unsigned long long v8; // [rsp+48h] [rbp-8h]

  v8 = __readfsqword(0x28u);
  v2 = 0;
  v3 = 0;
  ptr = 0;
  v6 = sub_262D1(0);
  for ( i = v6; i; i = *(long long *)(i + 48) )
  {
    v7 = (void *)sub_1AC29(ptr, 8 * (v3 + 1), &v2);
    if ( v7 )
    {
      ptr = v7;
      if ( !(unsigned int)sub_13B3B(i) )
        *((long long *)ptr + v3++) = *(long long *)(i + 32);
    }
    else
    {
      free(ptr);
      ptr = 0;
    }
  }
  sub_1395F(v6);
  if ( ptr )
    *a1 = v3;
  return ptr;
}



// Function: fallback_getfilecon @ 0x13e1c
long long fallback_getfilecon(unsigned int a1, const char *a2, long long a3, unsigned int a4)
{
  int v4; // eax

  v4 = *__errno_location();
  if ( v4 != 2 && v4 != 20 )
    return a4;
  if ( (qword_49980 & 2) != 0 )
    fprintf(stderr, "fallback_getfilecon(): getfilecon(%s) failed; falling back on lgetfilecon()\n", a2);
  return sub_3011A(a1, a2, a3);
}



// Function: optionh_getfilecon @ 0x13e94
long long optionh_getfilecon(unsigned int a1, const char *a2, long long a3)
{
  unsigned int v5; // [rsp+2Ch] [rbp-4h]

  if ( dword_499C0 )
    return sub_3011A(a1, a2, a3);
  v5 = sub_2FE8C(a1, a2, a3);
  if ( v5 )
    return sub_13E1C(a1, a2, a3, v5);
  else
    return 0;
}



// Function: optionl_getfilecon @ 0x13f08
long long optionl_getfilecon(unsigned int a1, const char *a2, long long a3)
{
  unsigned int v5; // [rsp+2Ch] [rbp-4h]

  v5 = sub_2FE8C(a1, a2, a3);
  if ( v5 )
    return sub_13E1C(a1, a2, a3, v5);
  else
    return 0;
}



// Function: optionp_getfilecon @ 0x13f5b
long long optionp_getfilecon(unsigned int a1, long long a2, long long a3)
{
  return sub_3011A(a1, a2, a3);
}



// Function: check_option_combinations @ 0x13f89
void check_option_combinations(long long a1)
{
  char *v1; // rax
  int v3; // [rsp+1Ch] [rbp-4h]

  v3 = 0;
  while ( a1 )
  {
    if ( *(long long (**)(long long, long long))a1 == sub_90B1 )
    {
      v3 |= 1u;
    }
    else if ( *(long long (**)(long long, long long))a1 == sub_A232 )
    {
      v3 |= 2u;
    }
    a1 = *(long long *)(a1 + 264);
  }
  if ( (v3 & 2) != 0 && (v3 & 1) != 0 && byte_49941 != 1 )
  {
    v1 = gettext(
           "The -delete action automatically turns on -depth, but -prune does nothing when -depth is in effect.  If you w"
           "ant to carry on anyway, just explicitly use the -depth option.");
    error(1, 0, v1);
  }
}



// Function: get_noop @ 0x14033
long long get_noop()
{
  int i; // [rsp+0h] [rbp-4h]

  if ( !qword_4A1D0 )
  {
    for ( i = 0; (&off_47DE8)[4 * i]; ++i )
    {
      if ( *((int *)&unk_47DE0 + 8 * i) == 1 )
      {
        qword_4A1D0 = (long long)&unk_47DE0 + 32 * i;
        return qword_4A1D0;
      }
    }
  }
  return qword_4A1D0;
}



// Function: get_stat_Ytime @ 0x140b3
long long get_stat_Ytime(long long a1, char a2, long long *a3)
{
  long long v3; // rdx
  long long v5; // rdx
  long long v6; // rdx
  long long v7; // rdx

  if ( a2 == 109 )
  {
    *a3 = sub_30A80(a1);
    a3[1] = v7;
    return 1;
  }
  else
  {
    if ( a2 > 109 )
      goto LABEL_11;
    if ( a2 == 99 )
    {
      *a3 = sub_30A66(a1);
      a3[1] = v6;
      return 1;
    }
    else
    {
      if ( a2 > 99 )
        goto LABEL_11;
      if ( a2 != 66 )
      {
        if ( a2 == 97 )
        {
          *a3 = sub_30A4C(a1);
          a3[1] = v3;
          return 1;
        }
LABEL_11:
        __assert_fail("0", "parser.c", 0x1F2u, "get_stat_Ytime");
      }
      *a3 = sub_30A9A(a1);
      a3[1] = v5;
      return a3[1] >= 0LL;
    }
  }
}



// Function: set_follow_state @ 0x141b0
int (*set_follow_state(unsigned int a1))(const char *a1, struct stat *a2)
{
  int (*result)(const char *, struct stat *); // rax

  if ( a1 == 2 )
  {
    qword_49990 = (long long (*)(long long, long long))sub_FB5C;
    qword_499A0 = (long long (*)(long long, long long, long long))sub_13E94;
    byte_4994C = 1;
  }
  else if ( a1 <= 2 )
  {
    if ( a1 )
    {
      qword_49990 = (long long (*)(long long, long long))sub_FC21;
      qword_499A0 = (long long (*)(long long, long long, long long))sub_13F08;
      byte_4994C = 1;
    }
    else
    {
      qword_49990 = (long long (*)(long long, long long))sub_FCC0;
      qword_499A0 = (long long (*)(long long, long long, long long))sub_13F5B;
    }
  }
  dword_49988 = a1;
  result = (int (*)(const char *, struct stat *))(qword_49980 & 2);
  if ( (qword_49980 & 2) != 0 )
  {
    qword_49990 = (long long (*)(long long, long long))sub_FD39;
    return sub_FD39;
  }
  return result;
}



// Function: parse_begin_user_args @ 0x14262
void parse_begin_user_args()
{
  qword_4A1C8 = 0;
}



// Function: parse_end_user_args @ 0x14287
void parse_end_user_args()
{
  ;
}



// Function: should_issue_warnings @ 0x142a1
long long should_issue_warnings()
{
  if ( byte_49951 )
    return 0;
  else
    return (unsigned char)byte_49950;
}



// Function: found_parser @ 0x142c4
int *found_parser(long long a1, int *a2)
{
  long long v3; // rbx
  char *v4; // rax

  if ( *a2 != 2 )
  {
    if ( *a2 == 1 )
      return 0;
    if ( *a2 )
    {
      if ( !qword_4A1C8 )
        qword_4A1C8 = a1;
    }
    else if ( qword_4A1C8 )
    {
      if ( (unsigned char)sub_142A1() )
      {
        v3 = qword_4A1C8;
        v4 = gettext(
               "warning: you have specified the global option %s after the argument %s, but global options are not positi"
               "onal, i.e., %s affects tests specified before it as well as those specified after it.  Please specify glo"
               "bal options before other arguments.");
        error(0, 0, v4, a1, v3, a1);
      }
    }
  }
  return a2;
}



// Function: find_parser @ 0x1437a
int *find_parser(const char *a1)
{
  char *s2; // [rsp+8h] [rbp-18h]
  int i; // [rsp+14h] [rbp-Ch]

  s2 = (char *)a1;
  if ( !strncmp("-newer", a1, 6u) && strlen(a1) == 8 )
    return sub_142C4((long long)a1, dword_47DC0);
  if ( *a1 == 45 )
    s2 = (char *)(a1 + 1);
  for ( i = 0; (&off_47DE8)[4 * i]; ++i )
  {
    if ( !strcmp((&off_47DE8)[4 * i], s2) )
      return sub_142C4((long long)a1, (int *)&unk_47DE0 + 8 * i);
  }
  return 0;
}



// Function: estimate_file_age_success_rate @ 0x1446f
float estimate_file_age_success_rate(float a1)
{
  if ( a1 < 0.1 )
    return 0.0099999998;
  if ( a1 < 1.0 )
    return 0.30000001;
  if ( a1 <= 100.0 )
    return 0.38999999;
  return 0.30000001;
}



// Function: estimate_timestamp_success_rate @ 0x144ce
float estimate_timestamp_success_rate(long long a1)
{
  __m128i v1; // xmm1

  v1 = 0;
  *(float *)v1.m128i_i32 = (float)(int)((timer - a1) / 86400);
  return sub_1446F(COERCE_FLOAT(_mm_cvtsi128_si32(v1)));
}



// Function: collect_arg_nonconst @ 0x14528
long long collect_arg_nonconst(long long a1, int *a2, long long *a3)
{
  if ( a1 && *(long long *)(8LL * *a2 + a1) )
  {
    *a3 = *(long long *)(8LL * (*a2)++ + a1);
    return 1;
  }
  else
  {
    *a3 = 0;
    return 0;
  }
}



// Function: collect_arg @ 0x145ab
long long collect_arg(long long a1, int *a2, long long *a3)
{
  long long result; // rax
  long long v5[2]; // [rsp+30h] [rbp-10h] BYREF

  v5[1] = __readfsqword(0x28u);
  LOBYTE(result) = sub_14528(a1, a2, v5);
  *a3 = v5[0];
  return (unsigned char)result;
}



// Function: collect_arg_stat_info @ 0x14611
long long collect_arg_stat_info(long long a1, int *a2, long long a3, long long *a4)
{
  long long v4; // rbx
  int *v5; // rax
  long long v9[3]; // [rsp+20h] [rbp-20h] BYREF

  v9[1] = __readfsqword(0x28u);
  if ( (unsigned char)sub_145AB(a1, a2, v9) )
  {
    *a4 = v9[0];
    if ( (unsigned int)qword_49990(v9[0], a3) )
    {
      v4 = v9[0];
      v5 = __errno_location();
      sub_10A44(*v5, v4);
    }
    return 1;
  }
  else
  {
    *a4 = 0;
    return 0;
  }
}



// Function: parse_and @ 0x146c4
long long parse_and(long long a1)
{
  long long v1; // rax

  v1 = sub_E644(a1);
  *(long long *)v1 = sub_8D9C;
  *(int *)(v1 + 16) = 3;
  *(int *)(v1 + 20) = 3;
  *(char *)(v1 + 27) = 0;
  *(char *)(v1 + 26) = *(char *)(v1 + 27);
  return 1;
}



// Function: parse_anewer @ 0x1472e
long long parse_anewer(long long a1, long long a2, int *a3, __m128i a4)
{
  int *v4; // rbx
  long long v5; // rdx
  long long v8; // [rsp+20h] [rbp-C0h] BYREF
  int *v9; // [rsp+28h] [rbp-B8h]
  char v10[88]; // [rsp+30h] [rbp-B0h] BYREF
  long long v11; // [rsp+88h] [rbp-58h]
  unsigned long long v12; // [rsp+C8h] [rbp-18h]

  v12 = __readfsqword(0x28u);
  sub_F26E();
  if ( !(unsigned char)sub_14611(a2, a3, (long long)v10, &v8) )
    return 0;
  v9 = (int *)sub_EECC(a1, v8);
  v9[14] = 0;
  v4 = v9;
  *((long long *)v4 + 8) = sub_30A80(v10);
  *((long long *)v4 + 9) = v5;
  v9[15] = 0;
  *(float *)a4.m128i_i32 = sub_144CE(v11);
  v9[9] = _mm_cvtsi128_si32(a4);
  return 1;
}



// Function: parse_closeparen @ 0x14833
long long parse_closeparen(long long a1)
{
  long long v1; // rax

  v1 = sub_E644(a1);
  *(long long *)v1 = sub_8EE1;
  *(int *)(v1 + 16) = 5;
  *(int *)(v1 + 20) = 0;
  *(char *)(v1 + 27) = 0;
  *(char *)(v1 + 26) = *(char *)(v1 + 27);
  return 1;
}



// Function: parse_cnewer @ 0x1489d
long long parse_cnewer(long long a1, long long a2, int *a3, __m128i a4)
{
  int *v4; // rbx
  long long v5; // rdx
  long long v8; // [rsp+20h] [rbp-C0h] BYREF
  int *v9; // [rsp+28h] [rbp-B8h]
  char v10[88]; // [rsp+30h] [rbp-B0h] BYREF
  long long v11; // [rsp+88h] [rbp-58h]
  unsigned long long v12; // [rsp+C8h] [rbp-18h]

  v12 = __readfsqword(0x28u);
  sub_F26E();
  if ( !(unsigned char)sub_14611(a2, a3, (long long)v10, &v8) )
    return 0;
  v9 = (int *)sub_EECC(a1, v8);
  v9[14] = 2;
  v4 = v9;
  *((long long *)v4 + 8) = sub_30A80(v10);
  *((long long *)v4 + 9) = v5;
  v9[15] = 0;
  *(float *)a4.m128i_i32 = sub_144CE(v11);
  v9[9] = _mm_cvtsi128_si32(a4);
  return 1;
}



// Function: parse_comma @ 0x149a2
long long parse_comma(long long a1)
{
  long long v1; // rax

  v1 = sub_E644(a1);
  *(long long *)v1 = sub_8FD0;
  *(int *)(v1 + 16) = 3;
  *(int *)(v1 + 20) = 1;
  *(char *)(v1 + 27) = 0;
  *(char *)(v1 + 26) = *(char *)(v1 + 27);
  *(int *)(v1 + 36) = 1065353216;
  return 1;
}



// Function: parse_daystart @ 0x14a1d
long long parse_daystart()
{
  time_t v0; // rax
  struct tm *v2; // [rsp+28h] [rbp-8h]

  if ( byte_49978 != 1 )
  {
    timer += 86400LL;
    qword_49970 = 0;
    v2 = localtime(&timer);
    if ( v2 )
      v0 = 60 * v2->tm_min + v2->tm_sec + 3600 * v2->tm_hour;
    else
      v0 = timer % 86400;
    timer -= v0;
    byte_49978 = 1;
  }
  return 1;
}



// Function: parse_delete @ 0x14afc
long long parse_delete(long long a1)
{
  long long v1; // rax

  v1 = sub_EF32(a1);
  *(char *)(v1 + 25) = 1;
  *(char *)(v1 + 24) = *(char *)(v1 + 25);
  byte_49940 = 0;
  *(char *)(v1 + 27) = 0;
  *(char *)(v1 + 26) = *(char *)(v1 + 27);
  *(int *)(v1 + 36) = 1065353216;
  return 1;
}



// Function: parse_depth @ 0x14b71
long long parse_depth(long long a1, long long a2, long long a3)
{
  byte_49940 = 0;
  byte_49941 = 1;
  return sub_17BAE(a1, a2, a3);
}



// Function: parse_d @ 0x14bb0
long long parse_d(long long a1, long long a2, long long a3)
{
  char *v3; // rax

  if ( (unsigned char)sub_142A1() )
  {
    v3 = gettext(
           "warning: the -d option is deprecated; please use -depth instead, because the latter is a POSIX-compliant feature.");
    error(0, 0, v3);
  }
  return sub_14B71(a1, a2, a3);
}



// Function: parse_empty @ 0x14c10
long long parse_empty(long long a1)
{
  *(int *)(sub_EF32(a1) + 36) = 1008981770;
  return 1;
}



// Function: parse_exec @ 0x14c50
long long parse_exec(long long a1, long long a2, long long a3)
{
  return sub_1893A("-exec", a1, a2, a3);
}



// Function: parse_execdir @ 0x14c88
long long parse_execdir(long long a1, long long a2, long long a3)
{
  return sub_1893A("-execdir", a1, a2, a3);
}



// Function: insert_false @ 0x14cc0
long long insert_false()
{
  long long v0; // rax
  int *v2; // [rsp+0h] [rbp-10h]

  v2 = sub_1437A("false");
  v0 = sub_EF32((long long)v2);
  *(char *)(v0 + 27) = 0;
  *(char *)(v0 + 26) = *(char *)(v0 + 27);
  *(char *)(v0 + 25) = 0;
  *(char *)(v0 + 24) = *(char *)(v0 + 25);
  *(int *)(v0 + 36) = 0;
  return 1;
}



// Function: parse_false @ 0x14d31
long long parse_false()
{
  return sub_14CC0();
}



// Function: parse_files0_from @ 0x14d50
long long parse_files0_from(long long a1, long long a2, int *a3)
{
  long long v4[2]; // [rsp+20h] [rbp-10h] BYREF

  v4[1] = __readfsqword(0x28u);
  if ( !(unsigned char)sub_145AB(a2, a3, v4) )
    return 0;
  filename = (char *)v4[0];
  return 1;
}



// Function: insert_fls @ 0x14dbf
long long insert_fls(long long a1, long long a2)
{
  long long v3; // [rsp+18h] [rbp-8h]

  v3 = sub_EF32(a1);
  if ( a2 )
    sub_19964(a2, v3 + 56);
  else
    sub_19A85(v3 + 56);
  *(char *)(v3 + 25) = 1;
  *(char *)(v3 + 24) = *(char *)(v3 + 25);
  *(int *)(v3 + 36) = 1065353216;
  return 1;
}



// Function: parse_fls @ 0x14e42
long long parse_fls(long long a1, long long a2, int *a3)
{
  long long v5[2]; // [rsp+20h] [rbp-10h] BYREF

  v5[1] = __readfsqword(0x28u);
  if ( (unsigned char)sub_145AB(a2, a3, v5) )
  {
    if ( (unsigned char)sub_14DBF(a1, v5[0]) )
      return 1;
    --*a3;
  }
  return 0;
}



// Function: parse_follow @ 0x14ecc
long long parse_follow(long long a1, long long a2, long long a3)
{
  sub_141B0(1u);
  return sub_17BAE(a1, a2, a3);
}



// Function: parse_fprint @ 0x14f07
long long parse_fprint(long long a1, long long a2, int *a3)
{
  long long v4; // [rsp+28h] [rbp-18h] BYREF
  long long v5; // [rsp+30h] [rbp-10h]
  unsigned long long v6; // [rsp+38h] [rbp-8h]

  v6 = __readfsqword(0x28u);
  if ( !(unsigned char)sub_145AB(a2, a3, &v4) )
    return 0;
  v5 = sub_EECC(a1, v4);
  sub_19964(v4, v5 + 56);
  *(char *)(v5 + 25) = 1;
  *(char *)(v5 + 24) = *(char *)(v5 + 25);
  *(char *)(v5 + 27) = 0;
  *(char *)(v5 + 26) = *(char *)(v5 + 27);
  *(int *)(v5 + 36) = 1065353216;
  return 1;
}



// Function: insert_fprint @ 0x14fd8
long long insert_fprint(long long a1, long long a2)
{
  long long v3; // [rsp+18h] [rbp-8h]

  v3 = sub_EECC(a1, a2);
  if ( a2 )
    sub_19964(a2, v3 + 56);
  else
    sub_19A85(v3 + 56);
  *(char *)(v3 + 25) = 1;
  *(char *)(v3 + 24) = *(char *)(v3 + 25);
  *(char *)(v3 + 27) = 0;
  *(char *)(v3 + 26) = *(char *)(v3 + 27);
  *(int *)(v3 + 36) = 1065353216;
  return 1;
}



// Function: parse_fprint0 @ 0x15079
long long parse_fprint0(long long a1, long long a2, int *a3)
{
  long long v5[2]; // [rsp+20h] [rbp-10h] BYREF

  v5[1] = __readfsqword(0x28u);
  if ( (unsigned char)sub_145AB(a2, a3, v5) )
  {
    if ( (unsigned char)sub_14FD8(a1, v5[0]) )
      return 1;
    --*a3;
  }
  return 0;
}



// Function: estimate_fstype_success_rate @ 0x15103
unsigned long long estimate_fstype_success_rate(const char *a1)
{
  char *s2; // [rsp+18h] [rbp-A8h]
  struct stat buf; // [rsp+20h] [rbp-A0h] BYREF
  unsigned long long v4; // [rsp+B8h] [rbp-8h]

  v4 = __readfsqword(0x28u);
  if ( !stat("/", &buf) )
  {
    s2 = (char *)sub_13A18((long long *)&buf, (long long)"/");
    strcmp(a1, s2);
  }
  return v4 - __readfsqword(0x28u);
}



// Function: parse_fstype @ 0x151c2
long long parse_fstype(long long a1, long long a2, int *a3, __m128i a4)
{
  const char *v5; // [rsp+28h] [rbp-18h] BYREF
  long long v6; // [rsp+30h] [rbp-10h]
  unsigned long long v7; // [rsp+38h] [rbp-8h]

  v7 = __readfsqword(0x28u);
  if ( !(unsigned char)sub_145AB(a2, a3, &v5) )
    return 0;
  if ( (unsigned short)word_499A8 <= 1u || (unsigned char)sub_13AA6(v5) )
  {
    v6 = sub_EECC(a1, (long long)v5);
    *(long long *)(v6 + 56) = v5;
    sub_15103(v5);
    *(int *)(v6 + 36) = _mm_cvtsi128_si32(a4);
    return 1;
  }
  else
  {
    if ( (qword_49980 & 8) != 0 )
      fprintf(stderr, "-fstype %s can never succeed, substituting -false\n", v5);
    return sub_14CC0();
  }
}



// Function: parse_gid @ 0x152b9
long long parse_gid(long long a1, long long a2, int *a3)
{
  int v3; // xmm0_4
  long long v6; // [rsp+28h] [rbp-8h]

  v6 = sub_197BF(a2, a3, a1);
  if ( v6 )
  {
    if ( *(long long *)(v6 + 64) > 0x63u )
      v3 = 1045220557;
    else
      v3 = 1065185444;
    *(int *)(v6 + 36) = v3;
    return 1;
  }
  else
  {
    --*a3;
    return 0;
  }
}



// Function: parse_group @ 0x15339
long long parse_group(long long a1, long long a2, int *a3)
{
  long long v3; // r12
  long long v4; // rbx
  char *v5; // rax
  long long v6; // rbx
  char *v7; // rax
  char *v8; // rax
  int v9; // xmm0_4
  __gid_t gr_gid; // [rsp+24h] [rbp-3Ch]
  int v12; // [rsp+2Ch] [rbp-34h]
  char *name; // [rsp+30h] [rbp-30h] BYREF
  struct group *v14; // [rsp+38h] [rbp-28h]
  long long v15; // [rsp+40h] [rbp-20h]
  unsigned long long v16; // [rsp+48h] [rbp-18h]

  v16 = __readfsqword(0x28u);
  if ( !(unsigned char)sub_145AB(a2, a3, &name) )
    return 0;
  v14 = getgrnam(name);
  endgrent();
  if ( !v14 )
  {
    v12 = strspn(name, "0123456789");
    if ( v12 )
    {
      if ( !name[v12] )
      {
        gr_gid = sub_1CD12(name, (unsigned int)dword_499AC);
        goto LABEL_11;
      }
      v3 = sub_2F861(1, (unsigned int)dword_499AC, &name[v12]);
      v4 = sub_2F861(0, (unsigned int)dword_499AC, name);
      v5 = gettext(
             "%s is not the name of an existing group and it does not look like a numeric group ID because it has the une"
             "xpected suffix %s");
      error(1, 0, v5, v4, v3);
    }
    if ( *name )
    {
      v6 = sub_2F861(0, (unsigned int)dword_499AC, name);
      v7 = gettext("%s is not the name of an existing group");
      error(1, 0, v7, v6);
    }
    v8 = gettext("argument to -group is empty, but should be a group name");
    error(1, 0, v8);
    goto LABEL_11;
  }
  gr_gid = v14->gr_gid;
LABEL_11:
  v15 = sub_EECC(a1, (long long)name);
  *(int *)(v15 + 56) = gr_gid;
  if ( *(long long *)(v15 + 64) > 0x63u )
    v9 = 1045220557;
  else
    v9 = 1065185444;
  *(int *)(v15 + 36) = v9;
  return 1;
}



// Function: parse_help @ 0x15554
void parse_help()
{
  sub_F071(0);
}



// Function: estimate_pattern_match_rate @ 0x15576
float estimate_pattern_match_rate(const char *a1, int a2)
{
  if ( strpbrk(a1, "*?[") || a2 && strpbrk(a1, ".") )
    return 0.80000001;
  else
    return 0.1;
}



// Function: parse_ilname @ 0x155d9
long long parse_ilname(long long a1, long long a2, int *a3)
{
  float v3; // xmm0_4
  const char *v5; // [rsp+28h] [rbp-18h] BYREF
  long long v6; // [rsp+30h] [rbp-10h]
  unsigned long long v7; // [rsp+38h] [rbp-8h]

  v7 = __readfsqword(0x28u);
  if ( !(unsigned char)sub_145AB(a2, a3, &v5) )
    return 0;
  v6 = sub_EECC(a1, (long long)v5);
  *(long long *)(v6 + 56) = v5;
  v3 = sub_15576(v5, 0);
  *(float *)(v6 + 36) = v3 * 0.1;
  return 1;
}



// Function: fnmatch_sanitycheck @ 0x15686
long long fnmatch_sanitycheck()
{
  char *v0; // rax

  if ( byte_4A1D8 != 1 )
  {
    if ( fnmatch("foo", "foo", 0) || !fnmatch("Foo", "foo", 0) || fnmatch("Foo", "foo", 16) )
    {
      v0 = gettext("sanity check of the fnmatch() library function failed.");
      error(1, 0, v0);
    }
    byte_4A1D8 = 1;
  }
  return (unsigned char)byte_4A1D8;
}



// Function: check_name_arg @ 0x1573c
long long check_name_arg(long long a1, long long a2, const char *a3)
{
  long long v3; // r13
  long long v4; // r12
  long long v5; // rbx
  char *v6; // rax

  if ( (unsigned char)sub_142A1() && strchr(a3, 47) )
  {
    v3 = sub_1096A(2u, a2);
    v4 = sub_1096A(1u, (long long)"/");
    v5 = sub_1096A(0, a1);
    v6 = gettext(
           "warning: %s matches against basenames only, but the given pattern contains a directory separator (%s), thus t"
           "he expression will evaluate to false all the time.  Did you mean %s?");
    error(0, 0, v6, v5, v4, v3);
  }
  return 1;
}



// Function: parse_iname @ 0x157fa
long long parse_iname(long long a1, long long a2, int *a3, __m128i a4)
{
  const char *v6; // [rsp+28h] [rbp-18h] BYREF
  long long v7; // [rsp+30h] [rbp-10h]
  unsigned long long v8; // [rsp+38h] [rbp-8h]

  v8 = __readfsqword(0x28u);
  sub_15686();
  if ( !(unsigned char)sub_145AB(a2, a3, &v6)
    || !(unsigned char)sub_1573C((long long)"-iname", (long long)"-iwholename", v6) )
  {
    return 0;
  }
  v7 = sub_EECC(a1, (long long)v6);
  *(char *)(v7 + 27) = 0;
  *(char *)(v7 + 26) = *(char *)(v7 + 27);
  *(long long *)(v7 + 56) = v6;
  *(float *)a4.m128i_i32 = sub_15576(v6, 0);
  *(int *)(v7 + 36) = _mm_cvtsi128_si32(a4);
  return 1;
}



// Function: parse_inum @ 0x158e1
long long parse_inum(long long a1, long long a2, int *a3)
{
  long long v5; // [rsp+28h] [rbp-8h]

  v5 = sub_197BF(a2, a3, a1);
  if ( v5 )
  {
    *(int *)(v5 + 36) = 897988541;
    *(char *)(v5 + 28) = 1;
    *(char *)(v5 + 26) = 0;
    *(char *)(v5 + 27) = 0;
    return 1;
  }
  else
  {
    --*a3;
    return 0;
  }
}



// Function: parse_iregex @ 0x15961
long long parse_iregex(long long a1, long long a2, long long a3)
{
  return sub_1723E(a2, a3, a1, dword_4999C | 0x400000u);
}



// Function: parse_links @ 0x1599c
long long parse_links(long long a1, long long a2, int *a3)
{
  long long v5; // [rsp+28h] [rbp-8h]

  v5 = sub_197BF(a2, a3, a1);
  if ( v5 )
  {
    if ( *(long long *)(v5 + 64) == 1 )
    {
      *(int *)(v5 + 36) = 1065185444;
    }
    else if ( *(long long *)(v5 + 64) == 2 )
    {
      *(int *)(v5 + 36) = 1008981770;
    }
    else
    {
      *(int *)(v5 + 36) = 981668463;
    }
    return 1;
  }
  else
  {
    --*a3;
    return 0;
  }
}



// Function: parse_lname @ 0x15a46
long long parse_lname(long long a1, long long a2, int *a3)
{
  float v3; // xmm0_4
  const char *v6; // [rsp+28h] [rbp-18h] BYREF
  long long v7; // [rsp+30h] [rbp-10h]
  unsigned long long v8; // [rsp+38h] [rbp-8h]

  v8 = __readfsqword(0x28u);
  sub_15686();
  if ( !(unsigned char)sub_145AB(a2, a3, &v6) )
    return 0;
  v7 = sub_EECC(a1, (long long)v6);
  *(long long *)(v7 + 56) = v6;
  v3 = sub_15576(v6, 0);
  *(float *)(v7 + 36) = v3 * 0.1;
  return 1;
}



// Function: parse_ls @ 0x15af8
long long parse_ls(long long a1)
{
  return sub_14DBF(a1, 0);
}



// Function: insert_depthspec @ 0x15b23
long long insert_depthspec(long long a1, long long a2, int *a3, int *a4)
{
  long long v5; // rbx
  char *v6; // rax
  int v9; // [rsp+24h] [rbp-2Ch]
  char *s; // [rsp+28h] [rbp-28h] BYREF
  long long v11; // [rsp+30h] [rbp-20h]
  unsigned long long v12; // [rsp+38h] [rbp-18h]

  v12 = __readfsqword(0x28u);
  v11 = *(long long *)(8LL * *a3 - 8 + a2);
  if ( (unsigned char)sub_145AB(a2, a3, &s) )
  {
    v9 = strspn(s, "0123456789");
    if ( v9 > 0 && !s[v9] )
    {
      *a4 = sub_1CD12(s, (unsigned int)dword_499AC);
      if ( *a4 >= 0 )
        return sub_17BAE(a1, a2, a3);
    }
    v5 = sub_2F861(0, (unsigned int)dword_499AC, s);
    v6 = gettext("Expected a positive decimal integer argument to %s, but got %s");
    error(1, 0, v6, v11, v5);
  }
  return 0;
}



// Function: parse_maxdepth @ 0x15c63
long long parse_maxdepth(long long a1, long long a2, int *a3)
{
  return sub_15B23(a1, a2, a3, &dword_49944);
}



// Function: parse_mindepth @ 0x15c98
long long parse_mindepth(long long a1, long long a2, int *a3)
{
  return sub_15B23(a1, a2, a3, &dword_49948);
}



// Function: do_parse_xmin @ 0x15ccd
long long do_parse_xmin(long long a1, long long a2, int *a3, int a4)
{
  const char *v4; // r9
  __m128i v5; // xmm0
  long long *v6; // rax
  long long v7; // rdx
  int v11; // [rsp+2Ch] [rbp-44h]
  long long v12; // [rsp+30h] [rbp-40h] BYREF
  long long *v13; // [rsp+38h] [rbp-38h]
  time_t v14; // [rsp+40h] [rbp-30h]
  long long v15; // [rsp+48h] [rbp-28h]
  long long v16; // [rsp+50h] [rbp-20h] BYREF
  long long v17; // [rsp+58h] [rbp-18h]
  long long v18; // [rsp+60h] [rbp-10h]
  unsigned long long v19; // [rsp+68h] [rbp-8h]

  v19 = __readfsqword(0x28u);
  v11 = *a3;
  if ( (unsigned char)sub_145AB(a2, a3, &v12) )
  {
    v15 = qword_49970;
    LODWORD(v16) = a4;
    v14 = timer + 86400;
    v5 = (__m128i)0x404E000000000000uLL;
    if ( (unsigned char)sub_1916A(
                            v12,
                            &v16,
                            timer + 86400,
                            qword_49970,
                            "arithmetic overflow while converting %s minutes to a number of seconds",
                            v4) )
    {
      v6 = (long long *)sub_EECC(a1, v12);
      v13 = v6;
      v7 = v17;
      v6[7] = v16;
      v6[8] = v7;
      v6[9] = v18;
      *(float *)v5.m128i_i32 = sub_144CE(v17);
      *((int *)v13 + 9) = _mm_cvtsi128_si32(v5);
      return 1;
    }
    *a3 = v11;
  }
  return 0;
}



// Function: parse_amin @ 0x15df0
long long parse_amin(long long a1, long long a2, int *a3)
{
  return sub_15CCD(a1, a2, a3, 0);
}



// Function: parse_cmin @ 0x15e23
long long parse_cmin(long long a1, long long a2, int *a3)
{
  return sub_15CCD(a1, a2, a3, 2);
}



// Function: parse_mmin @ 0x15e56
long long parse_mmin(long long a1, long long a2, int *a3)
{
  return sub_15CCD(a1, a2, a3, 3);
}



// Function: parse_name @ 0x15e89
long long parse_name(long long a1, long long a2, int *a3, __m128i a4)
{
  int v6; // [rsp+24h] [rbp-1Ch]
  const char *v7; // [rsp+28h] [rbp-18h] BYREF
  long long v8; // [rsp+30h] [rbp-10h]
  unsigned long long v9; // [rsp+38h] [rbp-8h]

  v9 = __readfsqword(0x28u);
  v6 = *a3;
  if ( (unsigned char)sub_145AB(a2, a3, &v7) )
  {
    sub_15686();
    if ( (unsigned char)sub_1573C((long long)"-name", (long long)"-wholename", v7) )
    {
      v8 = sub_EECC(a1, (long long)v7);
      *(char *)(v8 + 27) = 0;
      *(char *)(v8 + 26) = *(char *)(v8 + 27);
      *(long long *)(v8 + 56) = v7;
      *(float *)a4.m128i_i32 = sub_15576(v7, 0);
      *(int *)(v8 + 36) = _mm_cvtsi128_si32(a4);
      return 1;
    }
    *a3 = v6;
  }
  return 0;
}



// Function: parse_negate @ 0x15f82
long long parse_negate(int *a1)
{
  long long v1; // rax

  v1 = sub_E7DD(a1, 0);
  *(long long *)v1 = sub_9B30;
  *(int *)(v1 + 16) = 2;
  *(int *)(v1 + 20) = 4;
  *(char *)(v1 + 27) = 0;
  *(char *)(v1 + 26) = *(char *)(v1 + 27);
  return 1;
}



// Function: parse_newer @ 0x15ff1
long long parse_newer(long long a1, long long a2, int *a3, __m128i a4)
{
  int *v4; // rbx
  long long v5; // rdx
  long long v8; // [rsp+20h] [rbp-C0h] BYREF
  int *v9; // [rsp+28h] [rbp-B8h]
  char v10[88]; // [rsp+30h] [rbp-B0h] BYREF
  long long v11; // [rsp+88h] [rbp-58h]
  unsigned long long v12; // [rsp+C8h] [rbp-18h]

  v12 = __readfsqword(0x28u);
  sub_F26E();
  if ( !(unsigned char)sub_14611(a2, a3, (long long)v10, &v8) )
    return 0;
  v9 = (int *)sub_EECC(a1, v8);
  v4 = v9;
  *((long long *)v4 + 8) = sub_30A80(v10);
  *((long long *)v4 + 9) = v5;
  v9[14] = 3;
  v9[15] = 0;
  *(float *)a4.m128i_i32 = sub_144CE(v11);
  v9[9] = _mm_cvtsi128_si32(a4);
  return 1;
}



// Function: parse_newerXY @ 0x160f6
long long parse_newerXY(long long a1, long long a2, int *a3, __m128i a4)
{
  char *v5; // rax
  long long v6; // rbx
  char *v7; // rax
  long long v8; // rbx
  char *v9; // rax
  long long v10; // rbx
  int *v11; // rax
  long long v12; // rbx
  char *v13; // rax
  char v15; // [rsp+26h] [rbp-BAh]
  char v16; // [rsp+27h] [rbp-B9h]
  long long v17; // [rsp+28h] [rbp-B8h]
  char v18[146]; // [rsp+30h] [rbp-B0h] BYREF
  char s[6]; // [rsp+C2h] [rbp-1Eh] BYREF
  unsigned long long v20; // [rsp+C8h] [rbp-18h]

  v20 = __readfsqword(0x28u);
  if ( !a2 || !*(long long *)(8LL * *a3 + a2) )
    return 0;
  if ( strlen(*(const char **)(8LL * *a3 + a2)) != 8 )
    return 0;
  strcpy(s, "aBcmt");
  if ( strncmp("-newer", *(const char **)(8LL * *a3 + a2), 6u) )
    __assert_fail("0 == strncmp (\"-newer\", argv[*arg_ptr], 6)", "parser.c", 0x613u, "parse_newerXY");
  v15 = *(char *)(*(long long *)(8LL * *a3 + a2) + 6LL);
  v16 = *(char *)(*(long long *)(8LL * *a3 + a2) + 7LL);
  if ( v15 == 66 || v16 == 66 )
  {
    v5 = gettext("This system does not provide a way to find the birth time of a file.");
    error(0, 0, v5);
    return 0;
  }
  if ( v15 == 116 || !strchr(s, v15) || !strchr(s, v16) )
    return 0;
  if ( !*(long long *)(8LL * (*a3 + 1) + a2) )
  {
    v6 = sub_2F861(0, (unsigned int)dword_499AC, *(long long *)(8LL * *a3 + a2));
    v7 = gettext("The %s test needs an argument");
    error(1, 0, v7, v6);
  }
  v17 = sub_EECC(a1, *(long long *)(8LL * ++*a3 + a2));
  if ( v15 == 109 )
  {
    *(int *)(v17 + 56) = 3;
  }
  else
  {
    if ( v15 > 109 )
      goto LABEL_27;
    if ( v15 == 99 )
    {
      *(int *)(v17 + 56) = 2;
    }
    else
    {
      if ( v15 > 99 )
        goto LABEL_27;
      if ( v15 != 66 )
      {
        if ( v15 == 97 )
        {
          *(int *)(v17 + 56) = 0;
          goto LABEL_30;
        }
LABEL_27:
        if ( !strchr(s, v15) )
          __assert_fail("strchr (validchars, x)", "parser.c", 0x64Bu, "parse_newerXY");
        __assert_fail("0", "parser.c", 0x64Cu, "parse_newerXY");
      }
      *(int *)(v17 + 56) = 1;
    }
  }
LABEL_30:
  if ( v16 != 116 )
  {
LABEL_33:
    sub_F26E();
    if ( (unsigned int)qword_49990(*(long long *)(8LL * *a3 + a2), v18) )
    {
      v10 = *(long long *)(8LL * *a3 + a2);
      v11 = __errno_location();
      sub_10A44(*v11, v10);
    }
    if ( !sub_140B3((long long)v18, v16, (long long *)(v17 + 64)) )
    {
      v12 = sub_1096A(0, *(long long *)(8LL * *a3 + a2));
      v13 = gettext("Cannot obtain birth time of file %s");
      error(1, 0, v13, v12);
    }
    goto LABEL_37;
  }
  if ( (unsigned char)sub_2DCB0(v17 + 64, *(long long *)(8LL * *a3 + a2), &qword_49958) != 1 )
  {
    v8 = sub_2F861(0, (unsigned int)dword_499AC, *(long long *)(8LL * *a3 + a2));
    v9 = gettext("I cannot figure out how to interpret %s as a date or time");
    error(1, 0, v9, v8);
    goto LABEL_33;
  }
LABEL_37:
  *(int *)(v17 + 60) = 0;
  *(float *)a4.m128i_i32 = sub_144CE(*(long long *)(v17 + 64));
  *(int *)(v17 + 36) = _mm_cvtsi128_si32(a4);
  ++*a3;
  if ( !*(long long *)v17 )
    __assert_fail("our_pred->pred_func != NULL", "parser.c", 0x66Fu, "parse_newerXY");
  if ( *(bool (**)(long long, long long, long long))v17 != sub_9C02 )
    __assert_fail("our_pred->pred_func == pred_newerXY", "parser.c", 0x670u, "parse_newerXY");
  if ( !*(char *)(v17 + 26) )
    __assert_fail("our_pred->need_stat", "parser.c", 0x671u, "parse_newerXY");
  return 1;
}



// Function: parse_noleaf @ 0x1674d
long long parse_noleaf(long long a1, long long a2, long long a3)
{
  byte_4994C = 1;
  return sub_17BAE(a1, a2, a3);
}



// Function: parse_nogroup @ 0x16785
long long parse_nogroup(long long a1)
{
  *(int *)(sub_EECC(a1, 0) + 36) = 953267991;
  return 1;
}



// Function: parse_nouser @ 0x167ca
long long parse_nouser(long long a1)
{
  *(int *)(sub_EF32(a1) + 36) = 981668463;
  return 1;
}



// Function: parse_nowarn @ 0x1680a
long long parse_nowarn(long long a1, long long a2, long long a3)
{
  byte_49950 = 0;
  return sub_17BAE(a1, a2, a3);
}



// Function: parse_ok @ 0x16842
long long parse_ok(long long a1, long long a2, long long a3)
{
  return sub_1893A("-ok", a1, a2, a3);
}



// Function: parse_okdir @ 0x1687a
long long parse_okdir(long long a1, long long a2, long long a3)
{
  return sub_1893A("-okdir", a1, a2, a3);
}



// Function: parse_openparen @ 0x168b2
long long parse_openparen(int *a1)
{
  long long v1; // rax

  v1 = sub_E7DD(a1, 0);
  *(long long *)v1 = sub_9F83;
  *(int *)(v1 + 16) = 4;
  *(int *)(v1 + 20) = 0;
  *(char *)(v1 + 27) = 0;
  *(char *)(v1 + 26) = *(char *)(v1 + 27);
  return 1;
}



// Function: parse_or @ 0x16921
long long parse_or(long long a1)
{
  long long v1; // rax

  v1 = sub_E644(a1);
  *(long long *)v1 = sub_9F9E;
  *(int *)(v1 + 16) = 3;
  *(int *)(v1 + 20) = 2;
  *(char *)(v1 + 27) = 0;
  *(char *)(v1 + 26) = *(char *)(v1 + 27);
  return 1;
}



// Function: is_feasible_path_argument @ 0x1698b
long long is_feasible_path_argument(const char *a1, char a2)
{
  char *v3; // [rsp+18h] [rbp-8h]

  v3 = strrchr(a1, 47);
  return !v3 || v3[1] || sub_BB9D(a1, a2);
}



// Function: insert_path_check @ 0x169f6
long long insert_path_check(
        long long a1,
        long long a2,
        int *a3,
        long long a4,
        long long (*a5)(const char *a1, long long a2, long long a3),
        __m128i a6)
{
  const char *v6; // rbx
  char *v7; // rax
  char v12; // [rsp+37h] [rbp-29h]
  const char *v13; // [rsp+38h] [rbp-28h] BYREF
  long long v14; // [rsp+40h] [rbp-20h]
  unsigned long long v15; // [rsp+48h] [rbp-18h]

  v15 = __readfsqword(0x28u);
  v12 = a5 == sub_986A;
  sub_15686();
  if ( !(unsigned char)sub_145AB(a2, a3, &v13) )
    return 0;
  v14 = sub_EE5A(a1, (long long)a5, (long long)v13);
  *(char *)(v14 + 27) = 0;
  *(char *)(v14 + 26) = *(char *)(v14 + 27);
  *(long long *)(v14 + 56) = v13;
  *(float *)a6.m128i_i32 = sub_15576(v13, 0);
  *(int *)(v14 + 36) = _mm_cvtsi128_si32(a6);
  if ( byte_49951 != 1 && !sub_1698B(v13, v12) )
  {
    v6 = v13;
    v7 = gettext("warning: -%s %s will not match anything because it ends with /.");
    error(0, 0, v7, a4, v6);
    *(int *)(v14 + 36) = 841731191;
  }
  return 1;
}



// Function: parse_path @ 0x16b4b
long long parse_path(long long a1, long long a2, int *a3, __m128i a4)
{
  return sub_169F6(a1, a2, a3, (long long)"path", sub_A012, a4);
}



// Function: parse_wholename @ 0x16b8a
long long parse_wholename(long long a1, long long a2, int *a3, __m128i a4)
{
  return sub_169F6(a1, a2, a3, (long long)"wholename", sub_A012, a4);
}



// Function: parse_ipath @ 0x16bc9
long long parse_ipath(long long a1, long long a2, int *a3, __m128i a4)
{
  return sub_169F6(a1, a2, a3, (long long)"ipath", sub_986A, a4);
}



// Function: parse_iwholename @ 0x16c08
long long parse_iwholename(long long a1, long long a2, int *a3, __m128i a4)
{
  return sub_169F6(a1, a2, a3, (long long)"iwholename", sub_986A, a4);
}



// Function: parse_perm @ 0x16c47
long long parse_perm(long long a1, long long a2, int *a3)
{
  int v4; // eax
  long long v5; // rbx
  char *v6; // rax
  char *v7; // rbx
  char *v8; // rax
  int v9; // [rsp+2Ch] [rbp-44h]
  int v10; // [rsp+30h] [rbp-40h]
  int v11; // [rsp+34h] [rbp-3Ch]
  char *v12; // [rsp+38h] [rbp-38h] BYREF
  void *ptr; // [rsp+40h] [rbp-30h]
  int *v14; // [rsp+48h] [rbp-28h]
  int src; // [rsp+50h] [rbp-20h] BYREF
  int v16; // [rsp+54h] [rbp-1Ch]
  unsigned long long v17; // [rsp+58h] [rbp-18h]

  v17 = __readfsqword(0x28u);
  if ( (unsigned char)sub_145AB(a2, a3, &v12) != 1 )
    return 0;
  v4 = *v12;
  if ( v4 == 45 )
  {
    v10 = 1;
    v11 = 0;
    v9 = 1045220557;
  }
  else if ( v4 == 47 )
  {
    v10 = 1;
    v11 = 1;
    v9 = 1050253722;
  }
  else
  {
    v10 = 0;
    v11 = 2;
    v9 = 1008981770;
  }
  ptr = (void *)sub_25A35(&v12[v10]);
  if ( !ptr || *v12 == 43 && v12[1] > 47 && v12[1] <= 55 )
  {
    v5 = sub_2F861(0, (unsigned int)dword_499AC, v12);
    v6 = gettext("invalid mode %s");
    error(1, 0, v6, v5);
  }
  src = sub_25F2E(0, 0, 0, ptr, 0);
  v16 = sub_25F2E(0, 1, 0, ptr, 0);
  free(ptr);
  if ( *v12 == 47 && !src && !v16 )
  {
    v7 = v12;
    v8 = gettext(
           "warning: you have specified a mode pattern %s (which is equivalent to /000). The meaning of -perm /000 has no"
           "w been changed to be consistent with -perm -000; that is, while it used to match no files, it now matches all files.");
    error(0, 0, v8, v7);
    v11 = 0;
    v9 = 1065329728;
  }
  v14 = (int *)sub_EECC(a1, (long long)v12);
  v14[9] = v9;
  v14[14] = v11;
  memcpy(v14 + 15, &src, 8u);
  return 1;
}



// Function: parse_print @ 0x16eb3
long long parse_print(long long a1)
{
  char *v2; // [rsp+28h] [rbp-8h]

  v2 = (char *)sub_EF32(a1);
  v2[25] = 1;
  v2[24] = v2[25];
  v2[27] = 0;
  v2[26] = v2[27];
  sub_19A85(v2 + 56);
  return 1;
}



// Function: parse_print0 @ 0x16f20
long long parse_print0(long long a1)
{
  return sub_14FD8(a1, 0);
}



// Function: parse_printf @ 0x16f4b
long long parse_printf(long long a1, long long a2, int *a3)
{
  int v5; // [rsp+24h] [rbp-3Ch]
  const char *v6; // [rsp+28h] [rbp-38h] BYREF
  long long v7[6]; // [rsp+30h] [rbp-30h] BYREF

  v7[5] = __readfsqword(0x28u);
  v5 = *a3;
  if ( !(unsigned char)sub_14528(a2, a3, &v6) )
    return 0;
  sub_19A85(v7);
  if ( (unsigned char)sub_1159F(v7, a1, v6) )
    return 1;
  *a3 = v5;
  return 0;
}



// Function: parse_fprintf @ 0x16fef
long long parse_fprintf(long long a1, long long a2, int *a3)
{
  int v5; // [rsp+2Ch] [rbp-44h]
  long long v6; // [rsp+30h] [rbp-40h] BYREF
  const char *v7; // [rsp+38h] [rbp-38h] BYREF
  long long v8[6]; // [rsp+40h] [rbp-30h] BYREF

  v8[5] = __readfsqword(0x28u);
  v5 = *a3;
  if ( (unsigned char)sub_145AB(a2, a3, &v6) )
  {
    if ( (unsigned char)sub_14528(a2, a3, &v7) )
    {
      sub_19964(v6, v8);
      v5 = *a3;
      if ( (unsigned char)sub_1159F(v8, a1, v7) )
        return 1;
    }
  }
  *a3 = v5;
  return 0;
}



// Function: parse_prune @ 0x170b7
long long parse_prune(long long a1)
{
  char *v2; // [rsp+28h] [rbp-8h]

  v2 = (char *)sub_EF32(a1);
  if ( byte_49940 != 1 )
  {
    v2[27] = 0;
    v2[26] = v2[27];
  }
  v2[24] = 1;
  v2[25] = 0;
  return 1;
}



// Function: parse_quit @ 0x1711b
long long parse_quit(long long a1)
{
  long long v1; // rax

  v1 = sub_EF32(a1);
  *(char *)(v1 + 27) = 0;
  *(char *)(v1 + 26) = *(char *)(v1 + 27);
  *(char *)(v1 + 24) = 1;
  *(char *)(v1 + 25) = 0;
  *(int *)(v1 + 36) = 1065353216;
  return 1;
}



// Function: parse_regextype @ 0x17182
long long parse_regextype(long long a1, long long a2, int *a3)
{
  long long v5[2]; // [rsp+20h] [rbp-10h] BYREF

  v5[1] = __readfsqword(0x28u);
  if ( !(unsigned char)sub_145AB(a2, a3, v5) )
    return 0;
  dword_4999C = sub_1CA8F(v5[0]);
  return sub_17BAE(a1, a2, a3);
}



// Function: parse_regex @ 0x1720a
long long parse_regex(long long a1, long long a2, long long a3)
{
  return sub_1723E(a2, a3, a1, (unsigned int)dword_4999C);
}



// Function: insert_regex @ 0x1723e
long long insert_regex(long long a1, int *a2, long long a3, int a4, __m128i a5)
{
  unsigned char *v5; // rax
  size_t v6; // rax
  char *v7; // rbx
  char *v8; // rax
  char *s; // [rsp+28h] [rbp-38h] BYREF
  long long v13; // [rsp+30h] [rbp-30h]
  struct re_pattern_buffer *buffer; // [rsp+38h] [rbp-28h]
  const char *v15; // [rsp+40h] [rbp-20h]
  unsigned long long v16; // [rsp+48h] [rbp-18h]

  v16 = __readfsqword(0x28u);
  if ( !(unsigned char)sub_145AB(a1, a2, &s) )
    return 0;
  v13 = sub_EE5A(a3, (long long)sub_A2DA, (long long)s);
  *(char *)(v13 + 27) = 0;
  *(char *)(v13 + 26) = *(char *)(v13 + 27);
  buffer = (struct re_pattern_buffer *)sub_31BC2(64);
  *(long long *)(v13 + 56) = buffer;
  buffer->allocated = 100;
  v5 = (unsigned char *)sub_31BC2(buffer->allocated);
  buffer->buffer = v5;
  buffer->fastmap = 0;
  re_set_syntax(a4);
  buffer->syntax = a4;
  buffer->translate = 0;
  v6 = strlen(s);
  v15 = re_compile_pattern(s, v6, buffer);
  if ( v15 )
  {
    v7 = s;
    v8 = gettext("failed to compile regular expression '%s': %s");
    error(1, 0, v8, v7, v15);
  }
  *(float *)a5.m128i_i32 = sub_15576(s, 1);
  *(int *)(v13 + 36) = _mm_cvtsi128_si32(a5);
  return 1;
}



// Function: parse_size @ 0x173d3
long long parse_size(long long a1, long long a2, int *a3)
{
  char *v4; // rax
  unsigned int v5; // ebx
  char *v6; // rax
  char *v7; // rax
  int v8; // xmm0_4
  int v9; // xmm0_4
  char v11; // [rsp+2Bh] [rbp-45h]
  int v12; // [rsp+2Ch] [rbp-44h] BYREF
  int v13; // [rsp+30h] [rbp-40h]
  int v14; // [rsp+34h] [rbp-3Ch]
  long long v15; // [rsp+38h] [rbp-38h] BYREF
  char *s; // [rsp+40h] [rbp-30h]
  long long v17; // [rsp+48h] [rbp-28h]
  char v18[2]; // [rsp+56h] [rbp-1Ah] BYREF
  unsigned long long v19; // [rsp+58h] [rbp-18h]

  v19 = __readfsqword(0x28u);
  v13 = 512;
  if ( !a2 || !*(long long *)(8LL * *a3 + a2) )
    return 0;
  s = *(char **)(8LL * *a3 + a2);
  v14 = strlen(s);
  if ( !v14 )
  {
    v4 = gettext("invalid null argument to -size");
    error(1, 0, v4);
  }
  v11 = s[v14 - 1];
  if ( v11 == 119 )
  {
    v13 = 2;
    s[v14 - 1] = 0;
    goto LABEL_28;
  }
  if ( v11 > 119 )
    goto LABEL_27;
  if ( v11 == 107 )
  {
    v13 = 1024;
    s[v14 - 1] = 0;
    goto LABEL_28;
  }
  if ( v11 > 107 )
    goto LABEL_27;
  if ( v11 == 99 )
  {
    v13 = 1;
    s[v14 - 1] = 0;
    goto LABEL_28;
  }
  if ( v11 > 99 )
    goto LABEL_27;
  if ( v11 == 98 )
  {
    v13 = 512;
    s[v14 - 1] = 0;
    goto LABEL_28;
  }
  if ( v11 == 77 )
  {
    v13 = 0x100000;
    s[v14 - 1] = 0;
    goto LABEL_28;
  }
  if ( v11 > 77 )
  {
LABEL_27:
    v5 = *(char *)(*(long long *)(8LL * *a3 + a2) + v14 - 1LL);
    v6 = gettext("invalid -size type `%c'");
    error(1, 0, v6, v5);
    goto LABEL_28;
  }
  if ( v11 > 57 )
  {
    if ( v11 != 71 )
      goto LABEL_27;
    v13 = 0x40000000;
    s[v14 - 1] = 0;
  }
  else
  {
    if ( v11 < 48 )
      goto LABEL_27;
    v11 = 0;
  }
LABEL_28:
  if ( (unsigned char)sub_19722(s, &v15, &v12) != 1 )
  {
    v18[0] = v11;
    v18[1] = 0;
    v7 = gettext("Invalid argument `%s%s' to -size");
    error(1, 0, v7, s, v18);
  }
  v17 = sub_EECC(a1, (long long)s);
  *(int *)(v17 + 56) = v12;
  *(int *)(v17 + 60) = v13;
  *(long long *)(v17 + 64) = v15;
  *(char *)(v17 + 26) = 1;
  *(char *)(v17 + 27) = 0;
  if ( v12 )
  {
    if ( v12 == 1 )
    {
      if ( (unsigned long long)(v13 * v15) <= 0x5000 )
        v9 = 1036831949;
      else
        v9 = 1063675494;
      *(int *)(v17 + 36) = v9;
    }
    else
    {
      *(int *)(v17 + 36) = 1008981770;
    }
  }
  else
  {
    if ( (unsigned long long)(v13 * v15) <= 0x5000 )
      v8 = 1063675494;
    else
      v8 = 1036831949;
    *(int *)(v17 + 36) = v8;
  }
  ++*a3;
  return 1;
}



// Function: parse_samefile @ 0x1777e
long long parse_samefile(long long a1, long long a2, int *a3)
{
  int v4; // ecx
  int v5; // r8d
  int v6; // r9d
  long long v7; // rbx
  int *v8; // rax
  long long v9; // rbx
  int *v10; // rax
  int fd; // [rsp+28h] [rbp-158h]
  int v13; // [rsp+2Ch] [rbp-154h]
  long long v14; // [rsp+30h] [rbp-150h] BYREF
  long long v15; // [rsp+38h] [rbp-148h]
  struct stat v16; // [rsp+40h] [rbp-140h] BYREF
  struct stat buf; // [rsp+D0h] [rbp-B0h] BYREF
  unsigned long long v18; // [rsp+168h] [rbp-18h]

  v18 = __readfsqword(0x28u);
  sub_F26E();
  if ( (unsigned char)sub_14611(a2, a3, (long long)&v16, &v14) != 1 )
    return 0;
  sub_F26E();
  v13 = 0;
  if ( dword_49988 )
  {
    fd = -1;
  }
  else if ( byte_49998 )
  {
    v13 = 0x20000;
    fd = -1;
  }
  else if ( (v16.st_mode & 0xF000) == 0xA000 )
  {
    fd = -2;
  }
  else
  {
    fd = -1;
  }
  if ( fd == -1 )
  {
    fd = sub_1B433(v14, v13, v13, v4, v5, v6);
    if ( fd >= 0 )
    {
      if ( fstat(fd, &buf) )
      {
        v7 = v14;
        v8 = __errno_location();
        sub_10A44(*v8, v7);
      }
      if ( (unsigned int)qword_49990(v14, &v16) )
      {
        v9 = v14;
        v10 = __errno_location();
        sub_10A44(*v10, v9);
      }
      if ( dword_49988 || byte_49998 == 1 )
      {
        v16 = buf;
      }
      else if ( (v16.st_mode & 0xF000) == 0xA000 || v16.st_dev != buf.st_dev || v16.st_ino != buf.st_ino )
      {
        close(fd);
        fd = -1;
      }
    }
  }
  v15 = sub_EECC(a1, v14);
  *(long long *)(v15 + 56) = v16.st_ino;
  *(long long *)(v15 + 64) = v16.st_dev;
  *(int *)(v15 + 72) = fd;
  *(char *)(v15 + 27) = 0;
  *(char *)(v15 + 26) = 1;
  *(int *)(v15 + 36) = 1008981770;
  return 1;
}



// Function: parse_true @ 0x17b57
long long parse_true(long long a1)
{
  long long v1; // rax

  v1 = sub_EF32(a1);
  *(char *)(v1 + 27) = 0;
  *(char *)(v1 + 26) = *(char *)(v1 + 27);
  *(int *)(v1 + 36) = 1065353216;
  return 1;
}



// Function: parse_noop @ 0x17bae
long long parse_noop()
{
  long long v0; // rax

  v0 = sub_14033();
  return sub_17B57(v0);
}



// Function: parse_accesscheck @ 0x17be3
long long parse_accesscheck(long long a1)
{
  long long v2; // [rsp+28h] [rbp-8h]

  v2 = sub_EF32(a1);
  *(char *)(v2 + 27) = 0;
  *(char *)(v2 + 26) = *(char *)(v2 + 27);
  *(char *)(v2 + 25) = 0;
  *(char *)(v2 + 24) = *(char *)(v2 + 25);
  if ( *(bool (**)())v2 == sub_A0FA )
    *(int *)(v2 + 36) = 1045220557;
  else
    *(int *)(v2 + 36) = 1063675494;
  return 1;
}



// Function: parse_type @ 0x17c77
long long parse_type(long long a1, long long a2, long long a3)
{
  return sub_183BF(a2, a3, a1, sub_A4B2);
}



// Function: parse_uid @ 0x17cac
long long parse_uid(long long a1, long long a2, int *a3)
{
  int v3; // xmm0_4
  long long v6; // [rsp+28h] [rbp-8h]

  v6 = sub_197BF(a2, a3, a1);
  if ( v6 )
  {
    if ( *(long long *)(v6 + 64) > 0x63u )
      v3 = 1045220557;
    else
      v3 = 1065185444;
    *(int *)(v6 + 36) = v3;
    return 1;
  }
  else
  {
    --*a3;
    return 0;
  }
}



// Function: parse_used @ 0x17d2c
long long parse_used(long long a1, long long a2, int *a3)
{
  long long *v3; // rax
  long long v4; // rdx
  __m128i v5; // xmm1
  __m128i v6; // xmm0
  long long v8; // rbx
  char *v9; // rax
  long long v10; // [rsp+28h] [rbp-58h] BYREF
  const char *v11; // [rsp+30h] [rbp-50h]
  long long *v12; // [rsp+38h] [rbp-48h]
  long long v13; // [rsp+40h] [rbp-40h]
  long long v14; // [rsp+48h] [rbp-38h]
  long long v15; // [rsp+50h] [rbp-30h] BYREF
  long long v16; // [rsp+58h] [rbp-28h]
  long long v17; // [rsp+60h] [rbp-20h]
  unsigned long long v18; // [rsp+68h] [rbp-18h]

  v18 = __readfsqword(0x28u);
  v11 = "arithmetic overflow while converting %s days to a number of seconds";
  if ( (unsigned char)sub_145AB(a2, a3, &v10) )
  {
    v13 = 0;
    v14 = 0;
    if ( (unsigned char)sub_1916A(v10, &v15, 0, 0, v11, 86400.0) )
    {
      v3 = (long long *)sub_EECC(a1, v10);
      v12 = v3;
      v4 = v16;
      v3[7] = v15;
      v3[8] = v4;
      v3[9] = v17;
      v5 = 0;
      *(float *)v5.m128i_i32 = (float)(int)(v16 / 86400);
      v6 = _mm_cvtsi32_si128(_mm_cvtsi128_si32(v5));
      *(float *)v6.m128i_i32 = sub_1446F(*(float *)v6.m128i_i32);
      *((int *)v12 + 9) = _mm_cvtsi128_si32(v6);
      return 1;
    }
    v8 = v10;
    v9 = gettext("Invalid argument %s to -used");
    error(1, 0, v9, v8);
  }
  return 0;
}



// Function: parse_user @ 0x17e8d
long long parse_user(long long a1, long long a2, int *a3)
{
  long long v3; // rbx
  char *v4; // rax
  char *v5; // rax
  int v6; // xmm0_4
  __uid_t pw_uid; // [rsp+24h] [rbp-3Ch]
  char *name; // [rsp+28h] [rbp-38h] BYREF
  struct passwd *v10; // [rsp+30h] [rbp-30h]
  size_t v11; // [rsp+38h] [rbp-28h]
  long long v12; // [rsp+40h] [rbp-20h]
  unsigned long long v13; // [rsp+48h] [rbp-18h]

  v13 = __readfsqword(0x28u);
  if ( !(unsigned char)sub_145AB(a2, a3, &name) )
    return 0;
  v10 = getpwnam(name);
  endpwent();
  if ( v10 )
  {
    pw_uid = v10->pw_uid;
  }
  else
  {
    v11 = strspn(name, "0123456789");
    if ( !v11 || name[v11] )
    {
      if ( *name )
      {
        v3 = sub_2F861(0, (unsigned int)dword_499AC, name);
        v4 = gettext("%s is not the name of a known user");
        error(1, 0, v4, v3);
      }
      v5 = gettext("The argument to -user should not be empty");
      error(1, 0, v5);
    }
    else
    {
      pw_uid = sub_1CD12(name, (unsigned int)dword_499AC);
    }
  }
  v12 = sub_EECC(a1, (long long)name);
  *(int *)(v12 + 56) = pw_uid;
  if ( *(int *)(v12 + 56) > 0x63u )
    v6 = 1045220557;
  else
    v6 = 1065185444;
  *(int *)(v12 + 36) = v6;
  return 1;
}



// Function: parse_version @ 0x1802c
void parse_version()
{
  char *v0; // rax
  const char *v1; // rax
  int v2; // [rsp+24h] [rbp-Ch] BYREF
  unsigned long long v3; // [rsp+28h] [rbp-8h]

  v3 = __readfsqword(0x28u);
  sub_1B6B6("find");
  v0 = gettext("Features enabled: ");
  printf(v0);
  printf("D_TYPE ");
  if ( byte_49998 )
    v1 = "enabled";
  else
    v1 = "disabled";
  printf("O_NOFOLLOW(%s) ", v1);
  printf("LEAF_OPTIMISATION ");
  if ( (int)is_selinux_enabled() > 0 )
    printf("SELINUX ");
  v2 = 0;
  if ( (unsigned char)sub_8B12(&v2) )
  {
    printf("FTS(");
    if ( (v2 & 0x200) != 0 )
      printf("FTS_CWDFD");
    printf(") ");
  }
  printf("CBO(level=%d) ", (unsigned short)word_499A8);
  printf("\n");
  exit(0);
}



// Function: parse_context @ 0x181cd
long long parse_context(long long a1, long long a2, int *a3)
{
  char *v4; // rax
  long long v5; // rax

  if ( !a2 || !*(long long *)(8LL * *a3 + a2) )
    return 0;
  if ( (int)is_selinux_enabled() <= 0 )
  {
    v4 = gettext("invalid predicate -context: SELinux is not enabled.");
    error(1, 0, v4);
  }
  v5 = sub_EECC(a1, 0);
  *(int *)(v5 + 36) = 1008981770;
  *(char *)(v5 + 26) = 0;
  *(long long *)(v5 + 56) = *(long long *)(8LL * (*a3)++ + a2);
  return 1;
}



// Function: parse_xdev @ 0x182aa
long long parse_xdev()
{
  byte_4994D = 1;
  return sub_17BAE();
}



// Function: parse_ignore_race @ 0x182e2
long long parse_ignore_race()
{
  byte_4994E = 1;
  return sub_17BAE();
}



// Function: parse_noignore_race @ 0x1831a
long long parse_noignore_race()
{
  byte_4994E = 0;
  return sub_17BAE();
}



// Function: parse_warn @ 0x18352
long long parse_warn()
{
  byte_49950 = 1;
  return sub_17BAE();
}



// Function: parse_xtype @ 0x1838a
long long parse_xtype(long long a1, long long a2, long long a3)
{
  return sub_183BF(a2, a3, a1, sub_A78E);
}



// Function: insert_type @ 0x183bf
long long insert_type(
        long long a1,
        int *a2,
        long long a3,
        long long (*a4)(long long a1, long long a2, long long a3))
{
  const char *v4; // rax
  char *v6; // rax
  unsigned int v7; // ebx
  char *v8; // rax
  unsigned int v9; // ebx
  char *v10; // rax
  unsigned int v11; // ebx
  char *v12; // rax
  char *v13; // rax
  char *v14; // rax
  unsigned int v17; // [rsp+28h] [rbp-38h]
  float v18; // [rsp+2Ch] [rbp-34h]
  char *v19; // [rsp+30h] [rbp-30h] BYREF
  const char *v20; // [rsp+38h] [rbp-28h]
  long long v21; // [rsp+40h] [rbp-20h]
  unsigned long long v22; // [rsp+48h] [rbp-18h]

  v22 = __readfsqword(0x28u);
  if ( a4 == sub_A78E )
    v4 = "-xtype";
  else
    v4 = "-type";
  v20 = v4;
  if ( (unsigned char)sub_145AB(a1, a2, &v19) != 1 )
    return 0;
  if ( !*v19 )
  {
    v6 = gettext("Arguments to %s should contain at least one letter");
    error(1, 0, v6, v20);
  }
  v21 = sub_EE5A(a3, (long long)a4, (long long)v19);
  *(int *)(v21 + 36) = 0;
  if ( a4 == sub_A78E )
  {
    *(char *)(v21 + 26) = 1;
    *(char *)(v21 + 27) = 0;
  }
  else
  {
    *(char *)(v21 + 26) = 0;
    *(char *)(v21 + 27) = 1;
  }
  while ( *v19 )
  {
    v18 = 0.0099999998;
    switch ( *v19 )
    {
      case 'D':
        v7 = (char)*v19;
        v8 = gettext("%s %c is not supported because Solaris doors are not supported on the platform find was compiled on.");
        error(1, 0, v8, v20, v7);
        goto LABEL_20;
      case 'b':
        v17 = 0;
        v18 = 0.00088800001;
        break;
      case 'c':
        v17 = 1;
        v18 = 0.000443;
        break;
      case 'd':
        v17 = 2;
        v18 = 0.092200004;
        break;
      case 'f':
        v17 = 3;
        v18 = 0.875;
        break;
      case 'l':
        v17 = 4;
        v18 = 0.031099999;
        break;
      case 'p':
        v17 = 5;
        v18 = 0.0000075540001;
        break;
      case 's':
        v17 = 6;
        v18 = 0.000015899999;
        break;
      default:
LABEL_20:
        v17 = 0;
        v9 = (char)*v19;
        v10 = gettext("Unknown argument to %s: %c");
        error(1, 0, v10, v20, v9);
        break;
    }
    if ( *(char *)(v21 + v17 + 56) )
    {
      v11 = (char)*v19;
      v12 = gettext("Duplicate file type '%c' in the argument list to %s.");
      error(1, 0, v12, v11, v20);
    }
    *(float *)(v21 + 36) = *(float *)(v21 + 36) + v18;
    *(char *)(v21 + v17 + 56) = 1;
    if ( *++v19 )
    {
      if ( *v19 != 44 )
      {
        v13 = gettext("Must separate multiple arguments to %s using: ','");
        error(1, 0, v13, v20);
      }
      if ( !*++v19 )
      {
        v14 = gettext("Last file type in list argument to %s is missing, i.e., list is ending on: ','");
        error(1, 0, v14, v20);
      }
    }
  }
  return 1;
}



// Function: stream_is_tty @ 0x1876f
bool stream_is_tty(FILE *a1)
{
  int fd; // [rsp+1Ch] [rbp-4h]

  fd = fileno(a1);
  return fd != -1 && isatty(fd) != 0;
}



// Function: check_path_safety @ 0x187ac
unsigned long long check_path_safety(long long a1)
{
  char *v1; // rax
  long long v2; // rax
  long long v3; // rbx
  char *v4; // rax
  long long v6; // [rsp+10h] [rbp-40h] BYREF
  size_t n; // [rsp+18h] [rbp-38h] BYREF
  char *v8; // [rsp+20h] [rbp-30h]
  const char *v9; // [rsp+28h] [rbp-28h]
  char *v10; // [rsp+30h] [rbp-20h]
  unsigned long long v11; // [rsp+38h] [rbp-18h]

  v11 = __readfsqword(0x28u);
  v8 = getenv("PATH");
  v9 = ":";
  if ( v8 )
  {
    sub_1CF56(v8, v9, 1, &v6, &n);
    do
    {
      if ( !n || n == 1 && v8[v6] == 46 )
      {
        v1 = gettext(
               "The current directory is included in the PATH environment variable, which is insecure in combination with"
               " the %s action of find.  Please remove the current directory from your $PATH (that is, remove \".\", doub"
               "led colons, or leading or trailing colons)");
        error(1, 0, v1, a1);
      }
      if ( v8[v6] != 47 )
      {
        v10 = strndup(&v8[v6], n);
        if ( v10 )
          v2 = (long long)v10;
        else
          v2 = (long long)&v8[v6];
        v3 = sub_1096A(0, v2);
        v4 = gettext(
               "The relative path %s is included in the PATH environment variable, which is insecure in combination with "
               "the %s action of find.  Please remove that entry from $PATH");
        error(1, 0, v4, v3, a1);
      }
    }
    while ( (unsigned char)sub_1CF56(v8, v9, 0, &v6, &n) );
  }
  return v11 - __readfsqword(0x28u);
}



// Function: insert_exec_ok @ 0x1893a
long long insert_exec_ok(long long a1, long long a2, long long a3, int *a4)
{
  char *v5; // rax
  char *v6; // rax
  long long v7; // r13
  long long v8; // r12
  long long v9; // rbx
  char *v10; // rax
  char *v11; // rax
  int v12; // eax
  char v15; // [rsp+27h] [rbp-89h]
  int v16; // [rsp+28h] [rbp-88h]
  int i; // [rsp+2Ch] [rbp-84h]
  int j; // [rsp+2Ch] [rbp-84h]
  int v19; // [rsp+30h] [rbp-80h]
  int v20; // [rsp+34h] [rbp-7Ch]
  int v21; // [rsp+38h] [rbp-78h]
  int v22; // [rsp+3Ch] [rbp-74h]
  char *s; // [rsp+40h] [rbp-70h]
  const char *v24; // [rsp+48h] [rbp-68h]
  long long (*v25)(long long, long long, long long); // [rsp+50h] [rbp-60h]
  char *ptr; // [rsp+58h] [rbp-58h]
  char v27[24]; // [rsp+70h] [rbp-40h] BYREF
  unsigned long long v28; // [rsp+88h] [rbp-28h]

  v28 = __readfsqword(0x28u);
  v25 = *(long long (**)(long long, long long, long long))(a2 + 24);
  if ( !a3 || !*(long long *)(8LL * *a4 + a3) )
    return 0;
  ptr = (char *)sub_EE5A(a2, (long long)v25, (long long)"(some -exec* arguments)");
  ptr[25] = 1;
  ptr[24] = ptr[25];
  ptr[26] = 0;
  ptr[27] = ptr[26];
  *((long long *)ptr + 31) = 0;
  if ( v25 == sub_9F27 || v25 == sub_9ECE )
  {
    v15 = 0;
    byte_499B8 = 1;
    ptr[244] = 1;
  }
  else
  {
    v15 = 1;
    ptr[244] = 0;
  }
  if ( v25 == sub_9497 || v25 == sub_9F27 )
  {
    *((long long *)ptr + 31) = 0;
    byte_4994E = 0;
    sub_187AC(a1);
  }
  else
  {
    if ( !::ptr )
      __assert_fail("NULL != initial_wd", "parser.c", 0xB79u, "insert_exec_ok");
    *((long long *)ptr + 31) = ::ptr;
  }
  ptr[56] = 0;
  v21 = *a4;
  v16 = *a4;
  v19 = 0;
  v20 = 0;
  s = 0;
  while ( *(long long *)(8LL * v16 + a3)
       && (**(char **)(8LL * v16 + a3) != 59 || *(char *)(*(long long *)(8LL * v16 + a3) + 1LL)) )
  {
    if ( v15 && **(char **)(8LL * v16 + a3) == 43 && !*(char *)(*(long long *)(8LL * v16 + a3) + 1LL) && v19 )
    {
      ptr[56] = 1;
      break;
    }
    v19 = 0;
    if ( sub_24CC3(*(long long *)(8LL * v16 + a3), "{}") )
    {
      v19 = 1;
      s = *(char **)(8LL * v16 + a3);
      ++v20;
      if ( !v16 && (v25 == sub_9497 || v25 == sub_9F27) )
      {
        v5 = gettext(
               "You may not use {} within the utility name for -execdir and -okdir, because this is a potential security problem.");
        error(1, 0, v5);
      }
    }
    ++v16;
  }
  if ( v16 != v21 && *(long long *)(8LL * v16 + a3) )
  {
    if ( ptr[56] )
    {
      if ( v25 == sub_9497 )
        v24 = "dir";
      else
        v24 = (const char *)&unk_3BDF6;
      if ( v20 > 1 )
      {
        v6 = gettext("Only one instance of {} is supported with -exec%s ... +");
        error(1, 0, v6, v24);
      }
      if ( strlen(s) != 2 )
      {
        if ( (unsigned long long)snprintf(v27, 0x13u, "-exec%s ... {} +", v24) > 0x13 )
          __assert_fail("needed <= MsgBufSize", "parser.c", 0xBC6u, "insert_exec_ok");
        v7 = sub_2F861(2, (unsigned int)dword_499AC, s);
        v8 = sub_2F861(1, (unsigned int)dword_499AC, "{}");
        v9 = sub_2F861(0, (unsigned int)dword_499AC, v27);
        v10 = gettext("In %s the %s must appear by itself, but you specified %s");
        error(1, 0, v10, v9, v8, v7);
      }
    }
    v22 = sub_1A58C(ptr + 64, 2048);
    if ( v22 && (unsigned int)(v22 - 1) <= 1 )
    {
      v11 = gettext("The environment is too large for exec().");
      error(1, 0, v11);
    }
    sub_1A6E3(ptr + 64);
    *((long long *)ptr + 16) = sub_B7AE;
    if ( ptr[56] )
    {
      *((long long *)ptr + 29) = 0;
      *((long long *)ptr + 14) = 0;
      *((long long *)ptr + 13) = 0;
      *((long long *)ptr + 17) = 0;
      *((long long *)ptr + 18) = 0;
      *((long long *)ptr + 15) = v16 - v21 - 1;
      sub_1A742(ptr + 64, ptr + 152, ptr + 56);
      for ( i = v21; i < v16 - 1; ++i )
      {
        v12 = strlen(*(const char **)(8LL * i + a3));
        sub_1A1B8((int)ptr + 64, (int)ptr + 152, *(long long *)(8LL * i + a3), v12 + 1, 0, 0, 1);
      }
    }
    else
    {
      *((int *)ptr + 60) = v16 - v21;
      *((long long *)ptr + 14) = "{}";
      *((long long *)ptr + 13) = strlen(*((const char **)ptr + 14));
      *((long long *)ptr + 17) = 0;
      *((long long *)ptr + 18) = 0;
      *((long long *)ptr + 29) = sub_31BC2(8LL * *((int *)ptr + 60));
      sub_1A742(ptr + 64, ptr + 152, ptr + 56);
      for ( j = 0; j < *((int *)ptr + 60); ++j )
        *(long long *)(8LL * j + *((long long *)ptr + 29)) = *(long long *)(8LL * (j + v21) + a3);
    }
    if ( *(long long *)(8LL * v16 + a3) )
      *a4 = v16 + 1;
    else
      *a4 = v16;
    return 1;
  }
  else
  {
    *a4 = v16;
    free(ptr);
    return 0;
  }
}



// Function: get_relative_timestamp @ 0x1916a
long long get_relative_timestamp(long long a1, long long a2, long long a3, int a4, const char *a5, double a6)
{
  unsigned int v6; // eax
  long long v11; // [rsp+28h] [rbp-28h] BYREF
  double v12; // [rsp+30h] [rbp-20h] BYREF
  double iptr; // [rsp+38h] [rbp-18h] BYREF
  double v14; // [rsp+40h] [rbp-10h]
  unsigned long long v15; // [rsp+48h] [rbp-8h]

  v11 = a1;
  v15 = __readfsqword(0x28u);
  if ( !(unsigned char)sub_196AB(&v11, a2 + 4) )
    return 0;
  v6 = *(int *)(a2 + 4);
  if ( v6 <= 1 )
  {
    if ( v6 )
      *(int *)(a2 + 4) = 0;
    else
      *(int *)(a2 + 4) = 1;
  }
  if ( !(unsigned char)sub_3223A(v11, 0, &v12, &strtod) )
    return 0;
  v14 = 1000000000.0 * modf(v12 * a6, &iptr);
  if ( (double)1000000000 <= v14 )
    __assert_fail("nanosec < nanosec_per_sec", "parser.c", 0xC4Du, "get_relative_timestamp");
  *(long long *)(a2 + 8) = (unsigned int)(int)((double)(int)a3 - iptr);
  if ( a3 < *(long long *)(a2 + 8) != iptr < 0.0 )
    error(1, 0, a5, v11);
  *(long long *)(a2 + 16) = (unsigned int)(int)((double)a4 - v14);
  if ( v14 > (double)a4 )
  {
    *(long long *)(a2 + 16) += 1000000000LL;
    --*(long long *)(a2 + 8);
  }
  return 1;
}



// Function: parse_time @ 0x1938f
long long parse_time(long long a1, long long a2, int *a3)
{
  __m128i v4; // xmm0
  long long *v5; // rax
  long long v6; // rdx
  const char *v7; // rdx
  const char *v8; // rax
  char *v9; // rax
  char *v10; // rax
  int v12; // [rsp+28h] [rbp-68h] BYREF
  int v13; // [rsp+2Ch] [rbp-64h]
  long long v14; // [rsp+30h] [rbp-60h] BYREF
  time_t timer; // [rsp+38h] [rbp-58h] BYREF
  char *v16; // [rsp+40h] [rbp-50h]
  long long v17; // [rsp+48h] [rbp-48h]
  long long v18; // [rsp+50h] [rbp-40h]
  long long *v19; // [rsp+58h] [rbp-38h]
  long long v20; // [rsp+60h] [rbp-30h]
  long long v21; // [rsp+68h] [rbp-28h]
  long long v22; // [rsp+70h] [rbp-20h] BYREF
  long long v23; // [rsp+78h] [rbp-18h]
  long long v24; // [rsp+80h] [rbp-10h]
  unsigned long long v25; // [rsp+88h] [rbp-8h]

  v25 = __readfsqword(0x28u);
  v16 = gettext("arithmetic overflow while converting %s days to a number of seconds");
  v13 = *a3;
  if ( (unsigned char)sub_145AB(a2, a3, &v14) != 1 )
    return 0;
  v17 = v14;
  v20 = ::timer;
  v21 = qword_49970;
  if ( (unsigned char)sub_196AB(&v14, &v12) && v12 == 1 )
  {
    v20 += 86399;
    v18 = v20;
  }
  v14 = v17;
  v4 = (__m128i)0x40F5180000000000uLL;
  if ( (unsigned char)sub_1916A(v17, (long long)&v22, v20, v21, v16, 86400.0) != 1 )
  {
    *a3 = v13;
    return 0;
  }
  else
  {
    v5 = (long long *)sub_EECC(a1, v17);
    v19 = v5;
    v6 = v23;
    v5[7] = v22;
    v5[8] = v6;
    v5[9] = v24;
    *(float *)v4.m128i_i32 = sub_144CE(v23);
    *((int *)v19 + 9) = _mm_cvtsi128_si32(v4);
    if ( (qword_49980 & 1) != 0 )
    {
      fprintf(stderr, "inserting %s\n", (const char *)v19[1]);
      if ( HIDWORD(v22) )
      {
        if ( HIDWORD(v22) == 1 )
        {
          v7 = " <";
        }
        else if ( HIDWORD(v22) == 2 )
        {
          v7 = ">=";
        }
        else
        {
          v7 = " ?";
        }
      }
      else
      {
        v7 = " >";
      }
      if ( HIDWORD(v22) )
      {
        if ( HIDWORD(v22) == 1 )
        {
          v8 = "lt";
        }
        else if ( HIDWORD(v22) == 2 )
        {
          v8 = "eq";
        }
        else
        {
          v8 = "?";
        }
      }
      else
      {
        v8 = "gt";
      }
      fprintf(stderr, "    type: %s    %s  ", v8, v7);
      timer = v19[8];
      v9 = ctime(&timer);
      fprintf(stderr, "%lu %s", v19[8], v9);
      if ( HIDWORD(v22) == 2 )
      {
        timer = v19[8] + 86400LL;
        v10 = ctime(&timer);
        fprintf(stderr, "                 <  %lu %s", timer, v10);
      }
    }
    return 1;
  }
}



// Function: get_comp_type @ 0x196ab
long long get_comp_type(long long *a1, int *a2)
{
  int v2; // eax

  v2 = *(char *)*a1;
  if ( v2 == 43 )
  {
    *a2 = 0;
    ++*a1;
  }
  else if ( v2 == 45 )
  {
    *a2 = 1;
    ++*a1;
  }
  else
  {
    *a2 = 2;
  }
  return 1;
}



// Function: get_num @ 0x19722
bool get_num(long long a1, long long a2, int *a3)
{
  long long v4; // [rsp+18h] [rbp-18h] BYREF
  char v5[8]; // [rsp+20h] [rbp-10h] BYREF
  unsigned long long v6; // [rsp+28h] [rbp-8h]

  v4 = a1;
  v6 = __readfsqword(0x28u);
  if ( !a1 )
    return 0;
  if ( a3 && (unsigned char)sub_196AB(&v4, a3) != 1 )
    return 0;
  return (unsigned int)sub_3291E(v4, v5, 10, a2, &unk_3BDF6) == 0;
}



// Function: insert_num @ 0x197bf
long long insert_num(long long a1, int *a2, long long a3)
{
  const char *v3; // rdx
  const char *v4; // rax
  int v7; // [rsp+2Ch] [rbp-24h] BYREF
  long long v8; // [rsp+30h] [rbp-20h] BYREF
  long long v9; // [rsp+38h] [rbp-18h] BYREF
  long long v10; // [rsp+40h] [rbp-10h]
  unsigned long long v11; // [rsp+48h] [rbp-8h]

  v11 = __readfsqword(0x28u);
  if ( !(unsigned char)sub_145AB(a1, a2, &v8) || !sub_19722(v8, (long long)&v9, &v7) )
    return 0;
  v10 = sub_EECC(a3, v8);
  *(int *)(v10 + 56) = v7;
  *(long long *)(v10 + 64) = v9;
  if ( (qword_49980 & 1) != 0 )
  {
    fprintf(stderr, "inserting %s\n", *(const char **)(v10 + 8));
    if ( v7 )
    {
      if ( v7 == 1 )
      {
        v3 = " <";
      }
      else if ( v7 == 2 )
      {
        v3 = " =";
      }
      else
      {
        v3 = " ?";
      }
    }
    else
    {
      v3 = " >";
    }
    if ( v7 )
    {
      if ( v7 == 1 )
      {
        v4 = "lt";
      }
      else if ( v7 == 2 )
      {
        v4 = "eq";
      }
      else
      {
        v4 = "?";
      }
    }
    else
    {
      v4 = "gt";
    }
    fprintf(stderr, "    type: %s    %s  ", v4, v3);
    fprintf(stderr, "%lu\n", *(long long *)(v10 + 64));
  }
  return v10;
}



// Function: open_output_file @ 0x19964
bool open_output_file(const char *a1, long long a2)
{
  int *v2; // rax
  bool result; // al

  *(long long *)a2 = 0;
  *(long long *)(a2 + 32) = sub_2DE20(0);
  if ( !strcmp(a1, "/dev/stderr") )
  {
    *(long long *)(a2 + 8) = stderr;
    *(long long *)(a2 + 16) = gettext("standard error");
  }
  else if ( !strcmp(a1, "/dev/stdout") )
  {
    *(long long *)(a2 + 8) = stdout;
    *(long long *)(a2 + 16) = gettext("standard output");
  }
  else
  {
    *(long long *)(a2 + 8) = sub_10CE6((long long *)qword_499F0, a1);
    *(long long *)(a2 + 16) = a1;
    if ( !*(long long *)(a2 + 8) )
    {
      v2 = __errno_location();
      sub_10AA8(*v2, (long long)a1);
    }
  }
  result = sub_1876F(*(FILE **)(a2 + 8));
  *(char *)(a2 + 24) = result;
  return result;
}



// Function: open_stdout @ 0x19a85
bool open_stdout(long long a1)
{
  return sub_19964("/dev/stdout", a1);
}


