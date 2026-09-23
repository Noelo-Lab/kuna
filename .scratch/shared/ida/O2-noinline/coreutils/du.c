// Function: main @ 0x3cd0
long long main(long long a1, char **a2, char **a3)
{
  int v3; // r15d
  int v4; // ebx
  long long v5; // rax
  char *v6; // rdi
  char *v7; // rax
  char *v8; // rax
  int v9; // eax
  char *v10; // rax
  unsigned int v11; // eax
  char *v12; // rax
  unsigned int v13; // eax
  long long v14; // r15
  int *v15; // rax
  const char *v16; // rcx
  const char *v17; // r12
  int v18; // eax
  FILE *v19; // rdi
  long long v20; // rax
  void *v21; // rbp
  bool v22; // al
  const char *v23; // rax
  const char *v24; // r14
  long long v25; // r15
  char *v26; // rax
  long long v27; // rcx
  long long v28; // r12
  char *v29; // rax
  long long v30; // r15
  char *v31; // rax
  long long v32; // r15
  const char *v33; // rax
  char *v34; // rax
  long long v35; // r13
  char *v36; // r12
  int *v37; // rax
  long long result; // rax
  char *v39; // rax
  char *v40; // rax
  char **v41; // rdi
  int v42; // eax
  char *v43; // rax
  long long v44; // r12
  char *v45; // rax
  char *v46; // rax
  char *v47; // rax
  long long v48; // r13
  char *v49; // r12
  int *v50; // rax
  long long v51; // r12
  char *v52; // rax
  long long v53; // [rsp-8h] [rbp-90h]
  char *s1; // [rsp+8h] [rbp-80h]
  int v55; // [rsp+10h] [rbp-78h]
  char *v56; // [rsp+10h] [rbp-78h]
  int v57; // [rsp+18h] [rbp-70h]
  unsigned int v58; // [rsp+18h] [rbp-70h]
  char v59; // [rsp+1Eh] [rbp-6Ah]
  char v60; // [rsp+1Fh] [rbp-69h]
  int longind; // [rsp+24h] [rbp-64h] BYREF
  long long v62; // [rsp+28h] [rbp-60h] BYREF
  long long v63[11]; // [rsp+30h] [rbp-58h] BYREF

  v3 = 1;
  v4 = a1;
  v63[3] = __readfsqword(0x28u);
  v5 = sub_5630(a1, a2, a3);
  v6 = *a2;
  v63[1] = 0;
  v63[0] = v5;
  sub_E310(v6);
  setlocale(6, "");
  bindtextdomain("coreutils", "/usr/local/share/locale");
  textdomain("coreutils");
  sub_137D0(sub_6020);
  qword_1D140 = sub_68C0();
  v7 = getenv("DU_BLOCK_SIZE");
  sub_D070(v7, &dword_1D170, &qword_1D148);
  v59 = 0;
  v55 = 16;
  v57 = 8;
  s1 = 0;
  v60 = 0;
  while ( 1 )
  {
    longind = -1;
    LODWORD(v8) = getopt_long(v4, a2, "0abd:chHklmst:xB:DLPSX:", &longopts, &longind);
    if ( (int)v8 == -1 )
      break;
    if ( (int)v8 > 135 )
      goto LABEL_8;
    if ( (int)v8 <= 47 )
    {
      if ( (int)v8 == -131 )
      {
        sub_11510(
          (int)stdout,
          (unsigned int)"du",
          (unsigned int)"GNU coreutils",
          (int)off_1D018,
          (unsigned int)"Torbjorn Granlund",
          (unsigned int)"David MacKenzie",
          "Paul Eggert",
          "Jim Meyering",
          0,
          a2);
        exit(0);
      }
      if ( (int)v8 == -130 )
        sub_58B0(0);
LABEL_8:
      v3 = 0;
    }
    else
    {
      switch ( (int)v8 )
      {
        case 48:
          byte_1D182 = 1;
          continue;
        case 66:
          goto LABEL_23;
        case 68:
        case 72:
          v55 = 17;
          continue;
        case 76:
          v55 = 2;
          continue;
        case 80:
          v55 = 16;
          continue;
        case 83:
          byte_1D180 = 1;
          continue;
        case 88:
          if ( (unsigned int)sub_6FE0(sub_6B50, qword_1D140, optarg, 0x10000000, 10) )
          {
            v14 = sub_10680(0, 3, optarg);
            v15 = __errno_location();
            v16 = (const char *)v14;
            v3 = 0;
            error(0, *v15, "%s", v16);
          }
          continue;
        case 97:
          byte_1D186 = 1;
          continue;
        case 98:
          byte_1D185 = 1;
          dword_1D170 = 0;
          qword_1D148 = 1;
          continue;
        case 99:
          byte_1D181 = 1;
          continue;
        case 100:
          if ( (unsigned int)sub_120A0(optarg) )
          {
            v25 = sub_10840(optarg);
            v26 = dcgettext(0, "invalid maximum depth %s", 5);
            v27 = v25;
            v3 = 0;
            error(0, 0, v26, v27);
          }
          else
          {
            v60 = 1;
            qword_1D010 = v62;
          }
          continue;
        case 104:
          dword_1D170 = 176;
          qword_1D148 = 1;
          continue;
        case 107:
          dword_1D170 = 0;
          qword_1D148 = 1024;
          continue;
        case 108:
          byte_1D184 = 1;
          continue;
        case 109:
          dword_1D170 = 0;
          qword_1D148 = 0x100000;
          continue;
        case 115:
          v59 = 1;
          continue;
        case 116:
          v11 = sub_11B80(optarg);
          if ( v11 )
            sub_12020(v11, (unsigned int)longind, 116, &longopts, optarg);
          if ( qword_1D178 || *optarg != 45 )
            continue;
          v12 = dcgettext(0, "invalid --threshold argument '-0'", 5);
          error(1, 0, v12);
LABEL_23:
          v13 = sub_D070(optarg, &dword_1D170, &qword_1D148);
          if ( v13 )
            sub_12020(v13, (unsigned int)longind, 66, &longopts, optarg);
          return result;
        case 120:
          v57 = 72;
          continue;
        case 128:
          byte_1D185 = 1;
          continue;
        case 129:
          sub_6B50(qword_1D140, optarg, 0x10000000);
          continue;
        case 130:
          s1 = optarg;
          continue;
        case 131:
          dword_1D170 = 144;
          qword_1D148 = 1;
          continue;
        case 133:
          byte_1D16C = 1;
          v9 = 0;
          if ( optarg )
            v9 = dword_15700[sub_5DC0("--time", optarg, off_1C6A0, dword_15700, 4, off_1D020, 1, v53)];
          dword_1D168 = v9;
          v10 = getenv("TZ");
          qword_1D150 = sub_109C0(v10);
          continue;
        case 134:
          ::s1 = optarg;
          continue;
        case 135:
          byte_1D16D = 1;
          continue;
        default:
          goto LABEL_8;
      }
    }
  }
  if ( !(char)v3 )
    goto LABEL_121;
  if ( byte_1D186 )
  {
    if ( !v59 )
      goto LABEL_45;
    v47 = dcgettext(0, "cannot both summarize and show all entries", 5);
    error(0, 0, v47);
LABEL_121:
    sub_58B0(1);
  }
  if ( v60 )
  {
    if ( v59 )
    {
      v28 = qword_1D010;
      if ( qword_1D010
        || (v39 = dcgettext(0, "warning: summarizing is the same as using --max-depth=0", 5),
            error(0, 0, v39),
            (v28 = qword_1D010) != 0) )
      {
        v29 = dcgettext(0, "warning: summarizing conflicts with --max-depth=%lu", 5);
        error(0, 0, v29, v28);
        sub_58B0(1);
      }
LABEL_95:
      qword_1D010 = 0;
    }
  }
  else if ( v59 )
  {
    goto LABEL_95;
  }
LABEL_45:
  if ( byte_1D16D )
  {
    if ( byte_1D185 )
    {
      v43 = dcgettext(0, "warning: options --apparent-size and -b are ineffective with --inodes", 5);
      error(0, 0, v43);
    }
    qword_1D148 = 1;
  }
  if ( byte_1D16C )
  {
    if ( ::s1 )
    {
      v17 = ::s1;
      goto LABEL_52;
    }
    v8 = getenv("TIME_STYLE");
    ::s1 = v8;
    v17 = v8;
    if ( !v8 || (LODWORD(v8) = strcmp(v8, "locale"), !(int)v8) )
    {
      v17 = "long-iso";
      ::s1 = "long-iso";
      goto LABEL_102;
    }
    if ( *v17 == 43 )
    {
      v8 = strchr(v17, 10);
      if ( v8 )
      {
        *v8 = 0;
        goto LABEL_52;
      }
      goto LABEL_53;
    }
    while ( 1 )
    {
      v17 = ::s1;
      LODWORD(v8) = strncmp(::s1, "posix-", 6u);
      if ( (int)v8 )
        break;
      ::s1 = (char *)(v17 + 6);
    }
LABEL_52:
    if ( *v17 == 43 )
    {
LABEL_53:
      qword_1D158 = (long long)(v17 + 1);
    }
    else
    {
LABEL_102:
      v42 = dword_156F0[sub_5DC0("time style", v17, off_1C680, dword_156F0, 4, off_1D020, 1, v8)];
      switch ( v42 )
      {
        case 1:
          qword_1D158 = (long long)"%Y-%m-%d %H:%M";
          break;
        case 2:
          qword_1D158 = (long long)"%Y-%m-%d";
          break;
        case 0:
          qword_1D158 = (long long)"%Y-%m-%d %H:%M:%S.%N %z";
          break;
      }
    }
  }
  if ( s1 )
  {
    if ( v4 > optind )
    {
      v44 = sub_10840(a2[optind]);
      v45 = dcgettext(0, "extra operand %s", 5);
      error(0, 0, v45, v44);
      v46 = dcgettext(0, "file operands cannot be combined with --files0-from", 5);
      __fprintf_chk(stderr, 1, "%s\n", v46);
      sub_58B0(1);
    }
    v18 = strcmp(s1, "-");
    v19 = stdin;
    if ( !v18 || (v20 = sub_8C10(s1, "r", stdin), v19 = stdin, v20) )
    {
      v21 = (void *)sub_5EE0(v19);
      v22 = v3;
      goto LABEL_59;
    }
    v48 = sub_10580(4, s1);
    v49 = dcgettext(0, "cannot open %s for reading", 5);
    v50 = __errno_location();
    error(1, *v50, v49, v48);
LABEL_123:
    sub_11A40();
  }
  v41 = (char **)v63;
  if ( v4 > optind )
    v41 = &a2[optind];
  v21 = (void *)sub_5EB0(v41);
  v22 = v55 == 2 || optind + 1 < v4;
LABEL_59:
  byte_1D183 = v22;
  if ( !v21 )
    goto LABEL_123;
  qword_1D198 = (void *)sub_6240();
  if ( !qword_1D198 )
    goto LABEL_123;
  if ( byte_1D184 || !byte_1D183 )
    v57 |= 0x100u;
  v58 = v55 | v57;
  while ( 1 )
  {
    v23 = (const char *)sub_5F20(v21, &v62);
    v24 = v23;
    if ( !v23 )
      break;
    if ( s1 )
    {
      if ( !strcmp(s1, "-") && !strcmp(v24, "-") )
      {
        v30 = sub_10580(4, v24);
        v31 = dcgettext(0, "when reading file names from stdin, no file name of %s allowed", 5);
        error(0, 0, v31, v30);
        if ( !*v24 )
          goto LABEL_77;
      }
      else
      {
        if ( *v24 )
          goto LABEL_70;
LABEL_77:
        v32 = sub_5FC0(v21);
        v56 = dcgettext(0, "invalid zero-length file name", 5);
        v33 = (const char *)sub_10680(0, 3, s1);
        error(0, 0, "%s:%lu: %s", v33, v32, v56);
      }
      v3 = 0;
      continue;
    }
    if ( *v23 )
    {
LABEL_70:
      qword_1D110 = (long long)v24;
      v3 &= sub_5540(v58);
      continue;
    }
    v3 = 0;
    v34 = dcgettext(0, "invalid zero-length file name", 5);
    error(0, 0, "%s", v34);
  }
  switch ( (int)v62 )
  {
    case 3:
      goto LABEL_123;
    case 4:
      LOBYTE(v3) = 0;
      v35 = sub_10680(0, 3, s1);
      v36 = dcgettext(0, "%s: read error", 5);
      v37 = __errno_location();
      error(0, *v37, v36, v35);
      break;
    case 2:
      break;
    default:
      __assert_fail("!\"unexpected error code from argv_iter\"", "src/du.c", 0x439u, "main");
  }
  sub_5FE0(v21);
  sub_62B0(qword_1D198);
  if ( qword_1D190 )
    sub_62B0(qword_1D190);
  if ( s1 && (ferror_unlocked(stdin) || (unsigned int)sub_70C0(stdin)) && (char)v3 )
  {
    v51 = sub_10580(4, s1);
    v52 = dcgettext(0, "error reading %s", 5);
    error(1, 0, v52, v51);
  }
  if ( byte_1D181 )
  {
    v40 = dcgettext(0, "total", 5);
    sub_4E30(&unk_1D120, v40);
  }
  return (unsigned char)v3 ^ 1u;
}



// Function: duinfo_init @ 0x4a40
unsigned long long duinfo_init(long long *a1)
{
  *a1 = 0;
  a1[1] = 0;
  a1[2] = 0x8000000000000000LL;
  a1[3] = -1;
  return 0x8000000000000000LL;
}



// Function: duinfo_set @ 0x4a70
void duinfo_set(long long *a1, long long a2, long long a3, long long a4)
{
  *a1 = a2;
  a1[1] = 1;
  a1[2] = a3;
  a1[3] = a4;
}



// Function: duinfo_add @ 0x4a90
long long duinfo_add(long long *a1, long long *a2)
{
  long long v2; // rax
  long long v3; // r12
  long long v4; // r13
  long long v5; // rdx
  long long result; // rax

  v2 = *a1 + *a2;
  v3 = a2[2];
  if ( __CFADD__(*a1, *a2) )
    v2 = -1;
  v4 = a2[3];
  v5 = a2[2];
  *a1 = v2;
  a1[1] += a2[1];
  result = sub_10EE0(a1[2], a1[3], v5, v4);
  if ( (int)result < 0 )
  {
    a1[2] = v3;
    a1[3] = v4;
  }
  return result;
}



// Function: hash_ins @ 0x4b50
bool hash_ins(long long a1, long long a2, long long a3)
{
  int v3; // eax

  v3 = sub_62E0(a1, a3, a2);
  if ( v3 < 0 )
    sub_11A40();
  return v3 != 0;
}



// Function: fill_mount_table @ 0x4b80
unsigned long long fill_mount_table()
{
  long long v0; // rax
  long long i; // rbp
  long long v2; // rbx
  struct stat v4; // [rsp+0h] [rbp-B8h] BYREF
  unsigned long long v5; // [rsp+98h] [rbp-20h]

  v5 = __readfsqword(0x28u);
  v0 = sub_D8B0(0);
  if ( v0 )
  {
    for ( i = v0; ; i = v2 )
    {
      if ( (*(char *)(i + 40) & 3) == 0 && !stat(*(const char **)(i + 8), &v4) )
        sub_4B50((long long)qword_1D190, v4.st_ino, v4.st_dev);
      v2 = *(long long *)(i + 48);
      sub_D860((void *)i);
      if ( !v2 )
        break;
    }
  }
  return v5 - __readfsqword(0x28u);
}



// Function: mount_point_in_fts_cycle @ 0x4c30
long long mount_point_in_fts_cycle(long long *a1)
{
  long long *v1; // rbx
  long long *v2; // rbp

  v1 = a1;
  v2 = (long long *)*a1;
  if ( !qword_1D190 )
  {
    qword_1D190 = (void *)sub_6240();
    if ( !qword_1D190 )
      sub_11A40();
    sub_4B80();
  }
  if ( a1 == v2 )
    return 0;
  while ( (int)sub_6330(qword_1D190, v1[14], v1[15]) <= 0 )
  {
    v1 = (long long *)v1[1];
    if ( !v1 || v2 == v1 )
      return 0;
  }
  return 1;
}



// Function: print_only_size @ 0x4cc0
unsigned long long print_only_size(long long a1)
{
  FILE *v1; // rbp
  char *v2; // rdi
  long long v4; // [rsp+0h] [rbp-2A8h] BYREF
  unsigned long long v5; // [rsp+298h] [rbp-10h]

  v1 = stdout;
  v5 = __readfsqword(0x28u);
  if ( a1 == -1 )
    v2 = dcgettext(0, "Infinity", 5);
  else
    v2 = (char *)sub_C800(a1, &v4, (unsigned int)dword_1D170, 1, qword_1D148);
  fputs_unlocked(v2, v1);
  return v5 - __readfsqword(0x28u);
}



// Function: show_date @ 0x4d60
unsigned long long show_date(long long a1, long long a2, long long a3, long long a4)
{
  const char *v6; // rbp
  long long v7; // r12
  char *v8; // rax
  long long v9; // [rsp+0h] [rbp-88h] BYREF
  long long v10; // [rsp+8h] [rbp-80h]
  char v11[64]; // [rsp+10h] [rbp-78h] BYREF
  char v12[24]; // [rsp+50h] [rbp-38h] BYREF
  unsigned long long v13; // [rsp+68h] [rbp-20h]

  v9 = a2;
  v10 = a3;
  v13 = __readfsqword(0x28u);
  if ( sub_10CE0(a4, &v9, v11) )
  {
    sub_8B90(stdout, a1, v11, a4, (unsigned int)v10);
  }
  else
  {
    v6 = (const char *)sub_4D50(v9, v12);
    v7 = sub_10840(v6);
    v8 = dcgettext(0, "time %s is out of range", 5);
    error(0, 0, v8, v7);
    fputs_unlocked(v6, stdout);
  }
  return v13 - __readfsqword(0x28u);
}



// Function: print_size @ 0x4e30
int print_size(long long *a1, const char *a2)
{
  long long v3; // rdi

  v3 = a1[1];
  if ( !byte_1D16D )
    v3 = *a1;
  sub_4CC0(v3);
  if ( byte_1D16C )
  {
    putchar_unlocked(9);
    sub_4D60(qword_1D158, a1[2], a1[3], qword_1D150);
  }
  __printf_chk(1, "\t%s%c", a2, byte_1D182 == 0 ? 0xA : 0);
  return fflush_unlocked(stdout);
}



// Function: process_file @ 0x4ec0
long long process_file(long long a1, long long a2)
{
  int v3; // ebx
  const char *v4; // r14
  unsigned int v5; // r12d
  int v6; // eax
  long long v7; // rdi
  long long v8; // rax
  long long v9; // rdx
  long long v10; // rsi
  __m128i v11; // xmm1
  unsigned long long v12; // rbp
  long long v13; // rdx
  char v14; // r15
  unsigned long long v15; // rdx
  long long *v16; // rcx
  long long v17; // rcx
  unsigned long long v18; // rdx
  unsigned int v19; // ebx
  unsigned long long v20; // rax
  long long v22; // rax
  long long v23; // r12
  char *v24; // rax
  long long v25; // rcx
  long long *v26; // r8
  long long v27; // rax
  long long *v28; // r9
  long long v29; // rax
  long long *v30; // rsi
  long long v31; // r13
  char *v32; // rax
  const char *v33; // rax
  long long v34; // r13
  char *v35; // rax
  long long *v36; // [rsp+8h] [rbp-A0h]
  long long *v37; // [rsp+8h] [rbp-A0h]
  long long *v38; // [rsp+10h] [rbp-98h]
  long long *v39; // [rsp+10h] [rbp-98h]
  long long v40; // [rsp+10h] [rbp-98h]
  long long v41; // [rsp+18h] [rbp-90h]
  long long *v42; // [rsp+18h] [rbp-90h]
  __m128i v43; // [rsp+20h] [rbp-88h] BYREF
  __m128i v44; // [rsp+30h] [rbp-78h] BYREF
  _OWORD v45[2]; // [rsp+40h] [rbp-68h] BYREF
  unsigned long long v46; // [rsp+68h] [rbp-40h]

  v3 = *(unsigned short *)(a2 + 104);
  v4 = *(const char **)(a2 + 56);
  v46 = __readfsqword(0x28u);
  if ( v3 != 4 )
  {
    v5 = 1;
    if ( v3 == 6 )
      goto LABEL_3;
    v5 = sub_6A90(qword_1D140, v4);
    if ( (char)v5 )
      goto LABEL_74;
    if ( v3 == 11 )
    {
      sub_B190(a1, a2, 1);
      if ( a2 != sub_AB50(a1) )
        __assert_fail("e == ent", "src/du.c", 0x20Fu, "process_file");
      v3 = *(unsigned short *)(a2 + 104);
    }
    if ( v3 == 13 || v3 == 10 )
    {
      v31 = sub_10580(4, v4);
      v32 = dcgettext(0, "cannot access %s", 5);
      error(0, *(int *)(a2 + 64), v32, v31);
      return v5;
    }
    if ( ((*(char *)(a1 + 72) & 0x40) == 0
       || *(long long *)(a2 + 88) <= 0
       || *(long long *)(a1 + 24) == *(long long *)(a2 + 112))
      && (byte_1D184
       || !byte_1D183 && ((*(int *)(a2 + 136) & 0xF000) == 0x4000 || *(long long *)(a2 + 128) <= 1u)
       || sub_4B50((long long)qword_1D198, *(long long *)(a2 + 120), *(long long *)(a2 + 112))) )
    {
      if ( v3 != 2 )
      {
        if ( v3 == 7 )
        {
          v5 = 0;
          v33 = (const char *)sub_10680(0, 3, v4);
          error(0, *(int *)(a2 + 64), "%s", v33);
        }
        else
        {
          v5 = 1;
          if ( v3 == 1 )
            return v5;
        }
LABEL_3:
        v6 = dword_1D168;
        v7 = a2 + 112;
        if ( dword_1D168 )
          goto LABEL_4;
LABEL_37:
        v8 = sub_108B0(v7);
        if ( byte_1D185 )
          goto LABEL_7;
LABEL_38:
        v10 = *(long long *)(a2 + 176) << 9;
        goto LABEL_9;
      }
      if ( (unsigned char)sub_11AD0(a1, a2) )
      {
        v5 = sub_4C30((long long *)a2);
        if ( !(char)v5 )
        {
          v34 = sub_10680(0, 3, v4);
          v35 = dcgettext(
                  0,
                  "WARNING: Circular directory structure.\n"
                  "This almost certainly means that you have a corrupted file system.\n"
                  "NOTIFY YOUR SYSTEM MANAGER.\n"
                  "The following directory is part of the cycle:\n"
                  "  %s\n",
                  5);
          error(0, 0, v35, v34);
          return v5;
        }
      }
    }
    else
    {
LABEL_74:
      if ( v3 == 1 )
      {
        sub_B190(a1, a2, 4);
        if ( a2 != sub_AB50(a1) )
          __assert_fail("e == ent", "src/du.c", 0x230u, "process_file");
      }
    }
    return 1;
  }
  v23 = sub_10580(4, v4);
  v24 = dcgettext(0, "cannot read directory %s", 5);
  v25 = v23;
  v5 = 0;
  error(0, *(int *)(a2 + 64), v24, v25);
  v6 = dword_1D168;
  v7 = a2 + 112;
  if ( !dword_1D168 )
    goto LABEL_37;
LABEL_4:
  if ( v6 == 2 )
    v8 = sub_10890(v7);
  else
    v8 = sub_108A0(v7);
  if ( !byte_1D185 )
    goto LABEL_38;
LABEL_7:
  v10 = *(long long *)(a2 + 160);
  if ( v10 < 0 )
    v10 = 0;
LABEL_9:
  sub_4A70(&v43, v10, v8, v9);
  v11 = _mm_load_si128(&v44);
  v12 = *(long long *)(a2 + 88);
  v45[0] = _mm_load_si128(&v43);
  v45[1] = v11;
  if ( qword_1D108 )
  {
    v13 = qword_1D188;
    v14 = byte_1D180;
    if ( qword_1D188 != v12 )
    {
      if ( qword_1D188 >= v12 )
      {
        if ( qword_1D188 - 1 != v12 )
          __assert_fail("level == prev_level - 1", "src/du.c", 0x27Eu, "process_file");
        v41 = qword_1D100;
        v38 = (long long *)(qword_1D100 + (qword_1D188 << 6));
        sub_4A90((long long *)v45, v38);
        v26 = v38;
        v14 = byte_1D180;
        v27 = v41;
        v28 = v38 + 4;
        if ( !byte_1D180 )
        {
          v30 = v38 + 4;
          v42 = v38;
          v40 = v27;
          v37 = v28;
          sub_4A90((long long *)v45, v30);
          v26 = v42;
          v27 = v40;
          v28 = v37;
        }
        v39 = v28;
        v36 = (long long *)(v27 + (v12 << 6) + 32);
        sub_4A90(v36, v26);
        sub_4A90(v36, v39);
      }
      else
      {
        if ( qword_1D108 <= v12 )
        {
          v29 = sub_11770(qword_1D100, v12, 128);
          v13 = qword_1D188;
          qword_1D100 = v29;
          qword_1D108 = 2 * v12;
        }
        v15 = v13 + 1;
        v14 = byte_1D180;
        if ( v12 >= v15 )
        {
          v16 = (long long *)(qword_1D100 + (v15 << 6));
          do
          {
            sub_4A40(v16);
            sub_4A40((long long *)(v17 + 32));
          }
          while ( v12 >= v18 );
        }
      }
    }
  }
  else
  {
    qword_1D108 = v12 + 10;
    v22 = sub_11920(v12 + 10, 64);
    v14 = byte_1D180;
    qword_1D100 = v22;
  }
  qword_1D188 = v12;
  v19 = v3 & 0xFFFFFFFD;
  if ( v14 )
  {
    if ( v19 == 4 )
    {
      sub_4A90(qword_1D120, &v43);
      goto LABEL_21;
    }
    sub_4A90((long long *)(qword_1D100 + (v12 << 6)), &v43);
    sub_4A90(qword_1D120, &v43);
  }
  else
  {
    sub_4A90((long long *)(qword_1D100 + (v12 << 6)), &v43);
    sub_4A90(qword_1D120, &v43);
    if ( v19 == 4 )
      goto LABEL_21;
  }
  if ( !byte_1D186 )
    goto LABEL_22;
LABEL_21:
  if ( qword_1D010 < v12 )
  {
LABEL_22:
    if ( v12 )
      return v5;
  }
  v20 = *((long long *)&v45[0] + 1);
  if ( !byte_1D16D )
    v20 = *(long long *)&v45[0];
  if ( qword_1D178 < 0 )
  {
    if ( -qword_1D178 < v20 )
      return v5;
    goto LABEL_27;
  }
  if ( qword_1D178 <= v20 )
LABEL_27:
    sub_4E30((long long *)v45, v4);
  return v5;
}



// Function: usage @ 0x58b0
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
  FILE *v23; // rsi
  char *v24; // rdi

  v2 = qword_1D1B0;
  if ( status )
  {
    v3 = dcgettext(0, "Try '%s --help' for more information.\n", 5);
    __fprintf_chk(stderr, 1, v3, v2);
  }
  else
  {
    v4 = dcgettext(0, "Usage: %s [OPTION]... [FILE]...\n  or:  %s [OPTION]... --files0-from=F\n", 5);
    __printf_chk(1, v4, v2, v2);
    v5 = stdout;
    v6 = dcgettext(0, "Summarize device usage of the set of FILEs, recursively for directories.\n", 5);
    fputs_unlocked(v6, v5);
    sub_4AF0();
    v7 = stdout;
    v8 = dcgettext(
           0,
           "  -0, --null            end each output line with NUL, not newline\n"
           "  -a, --all             write counts for all files, not just directories\n"
           "      --apparent-size   print apparent sizes rather than device usage; although\n"
           "                          the apparent size is usually smaller, it may be\n"
           "                          larger due to holes in ('sparse') files, internal\n"
           "                          fragmentation, indirect blocks, and the like\n",
           5);
    fputs_unlocked(v8, v7);
    v9 = stdout;
    v10 = dcgettext(
            0,
            "  -B, --block-size=SIZE  scale sizes by SIZE before printing them; e.g.,\n"
            "                           '-BM' prints sizes in units of 1,048,576 bytes;\n"
            "                           see SIZE format below\n"
            "  -b, --bytes           equivalent to '--apparent-size --block-size=1'\n"
            "  -c, --total           produce a grand total\n"
            "  -D, --dereference-args  dereference only symlinks that are listed on the\n"
            "                          command line\n"
            "  -d, --max-depth=N     print the total for a directory (or file, with --all)\n"
            "                          only if it is N or fewer levels below the command\n"
            "                          line argument;  --max-depth=0 is the same as\n"
            "                          --summarize\n",
            5);
    fputs_unlocked(v10, v9);
    v11 = stdout;
    v12 = dcgettext(
            0,
            "      --files0-from=F   summarize device usage of the\n"
            "                          NUL-terminated file names specified in file F;\n"
            "                          if F is -, then read names from standard input\n"
            "  -H                    equivalent to --dereference-args (-D)\n"
            "  -h, --human-readable  print sizes in human readable format (e.g., 1K 234M 2G)\n"
            "      --inodes          list inode usage information instead of block usage\n",
            5);
    fputs_unlocked(v12, v11);
    v13 = stdout;
    v14 = dcgettext(
            0,
            "  -k                    like --block-size=1K\n"
            "  -L, --dereference     dereference all symbolic links\n"
            "  -l, --count-links     count sizes many times if hard linked\n"
            "  -m                    like --block-size=1M\n",
            5);
    fputs_unlocked(v14, v13);
    v15 = stdout;
    v16 = dcgettext(
            0,
            "  -P, --no-dereference  don't follow any symbolic links (this is the default)\n"
            "  -S, --separate-dirs   for directories do not include size of subdirectories\n"
            "      --si              like -h, but use powers of 1000 not 1024\n"
            "  -s, --summarize       display only a total for each argument\n",
            5);
    fputs_unlocked(v16, v15);
    v17 = stdout;
    v18 = dcgettext(
            0,
            "  -t, --threshold=SIZE  exclude entries smaller than SIZE if positive,\n"
            "                          or entries greater than SIZE if negative\n"
            "      --time            show time of the last modification of any file in the\n"
            "                          directory, or any of its subdirectories\n"
            "      --time=WORD       show time as WORD instead of modification time:\n"
            "                          atime, access, use, ctime or status\n"
            "      --time-style=STYLE  show times using STYLE, which can be:\n"
            "                            full-iso, long-iso, iso, or +FORMAT;\n"
            "                            FORMAT is interpreted like in 'date'\n",
            5);
    fputs_unlocked(v18, v17);
    v19 = stdout;
    v20 = dcgettext(
            0,
            "  -X, --exclude-from=FILE  exclude files that match any pattern in FILE\n"
            "      --exclude=PATTERN    exclude files that match PATTERN\n"
            "  -x, --one-file-system    skip directories on different file systems\n",
            5);
    fputs_unlocked(v20, v19);
    v21 = stdout;
    v22 = dcgettext(0, "      --help        display this help and exit\n", 5);
    fputs_unlocked(v22, v21);
    v23 = stdout;
    v24 = dcgettext(0, "      --version     output version information and exit\n", 5);
    fputs_unlocked(v24, v23);
    sub_5870();
    sub_4B20();
    sub_5640();
  }
  exit(status);
}



// Function: is_prime @ 0xb340
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



// Function: next_prime @ 0xb3b0
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
      if ( sub_B340(v2) )
        break;
      v2 += 2LL;
    }
    while ( v2 != -1 );
  }
  return v2;
}



// Function: raw_hasher @ 0xb3f0
unsigned long long raw_hasher(long long a1, unsigned long long a2)
{
  return sub_12DC0(a1, 3) % a2;
}



// Function: raw_comparator @ 0xb410
bool raw_comparator(long long a1, long long a2)
{
  return a2 == a1;
}



// Function: check_tuning @ 0xb420
long long check_tuning(long long a1)
{
  long long v1; // rax
  float v2; // xmm0_4
  float v3; // xmm1_4
  float v4; // xmm2_4
  long long result; // rax

  v1 = *(long long *)(a1 + 40);
  if ( (_UNKNOWN *)v1 == &unk_15A40 )
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
    *(long long *)(a1 + 40) = &unk_15A40;
    return 0;
  }
  return result;
}



// Function: free_entry @ 0xb4b0
long long free_entry(long long a1, long long *a2)
{
  long long result; // rax

  result = *(long long *)(a1 + 72);
  *a2 = 0;
  a2[1] = result;
  *(long long *)(a1 + 72) = a2;
  return result;
}



// Function: safe_hasher @ 0xb4d0
long long safe_hasher(long long a1, long long a2)
{
  unsigned long long v2; // rax

  v2 = (*(long long (**)(long long, long long))(a1 + 48))(a2, *(long long *)(a1 + 16));
  if ( *(long long *)(a1 + 16) <= v2 )
    abort();
  return *(long long *)a1 + 16 * v2;
}



// Function: hash_find_entry @ 0xb500
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

  v6 = (__m128i *)sub_B4D0(a1, a2);
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
          sub_B4B0(a1, v11);
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
      sub_B4B0(a1, v9);
      return v12;
    }
    else
    {
      v7->m128i_i64[0] = 0;
    }
  }
  return result;
}



// Function: allocate_entry @ 0xb600
long long *allocate_entry(long long a1)
{
  long long *result; // rax

  result = *(long long **)(a1 + 72);
  if ( !result )
    return malloc(0x10u);
  *(long long *)(a1 + 72) = result[1];
  return result;
}



// Function: transfer_entries @ 0xb630
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
              v9 = (long long *)sub_B4D0(a1, *v7);
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
            sub_B4B0(a1, v10);
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
    v11 = (long long *)sub_B4D0(a1, v6);
    v12 = v11;
    if ( !*v11 )
    {
      *v11 = v6;
      ++*(long long *)(a1 + 24);
      goto LABEL_15;
    }
    v13 = sub_B600(a1);
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



// Function: hash_get_n_buckets @ 0xb7f0
long long hash_get_n_buckets(long long a1)
{
  return *(long long *)(a1 + 16);
}



// Function: hash_get_n_buckets_used @ 0xb800
long long hash_get_n_buckets_used(long long a1)
{
  return *(long long *)(a1 + 24);
}



// Function: hash_get_n_entries @ 0xb810
long long hash_get_n_entries(long long a1)
{
  return *(long long *)(a1 + 32);
}



// Function: hash_get_max_bucket_length @ 0xb820
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



// Function: hash_print_statistics @ 0xb8f0
long long hash_print_statistics(long long a1, long long a2)
{
  long long v2; // rbx
  long long v3; // r13
  unsigned long long v4; // r12
  long long v5; // r9
  double v6; // xmm0_8
  double v7; // xmm1_8

  sub_B810(a1);
  v2 = sub_B7F0(a1);
  v3 = sub_B800(a1);
  v4 = sub_B820(a1);
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



// Function: hash_lookup @ 0xba10
long long hash_lookup(long long a1, long long a2)
{
  long long v2; // rbp
  long long *v3; // rax
  long long v4; // rsi
  long long *v5; // rbx

  v2 = a2;
  v3 = (long long *)sub_B4D0(a1, a2);
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



// Function: hash_get_next @ 0xbac0
long long hash_get_next(long long a1, long long a2)
{
  long long *v2; // rax
  long long *v3; // rdx
  long long v4; // rcx
  long long v5; // r8

  v2 = (long long *)sub_B4D0(a1, a2);
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



// Function: hash_do_for_each @ 0xbb80
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



// Function: hash_string @ 0xbc00
unsigned long long hash_string(unsigned char *a1, unsigned long long a2)
{
  long long v2; // rcx
  unsigned long long i; // rdx
  unsigned long long v4; // rax

  v2 = *a1;
  for ( i = 0; (char)v2; i = v4 % a2 )
  {
    ++a1;
    v4 = v2 + 31 * i;
    v2 = *a1;
  }
  return i;
}



// Function: hash_initialize @ 0xbc70
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
    v7 = sub_B3F0;
  if ( !a4 )
    v8 = sub_B410;
  v9 = malloc(0x50u);
  if ( v9 )
  {
    if ( !a2 )
      v5 = (float *)&unk_15A40;
    v9[5] = v5;
    if ( (unsigned char)sub_B420((long long)v9)
      && (v10 = sub_B760(a1, *((unsigned char *)v5 + 16), v5[2]), (v9[2] = v10) != 0)
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



// Function: hash_clear @ 0xbd80
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



// Function: hash_free @ 0xbe30
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



// Function: hash_rehash @ 0xbf20
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
  v2 = sub_B760(a2, *(unsigned char *)(*(long long *)(a1 + 40) + 16LL), *(float *)(*(long long *)(a1 + 40) + 8LL));
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
  v6 = sub_B630((long long)&ptr, a1, 0);
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
    if ( !(unsigned char)sub_B630(a1, (long long)&ptr, 1) || !(unsigned char)sub_B630(a1, (long long)&ptr, 0) )
      abort();
    free(ptr);
  }
  return v6;
}



// Function: hash_insert_if_absent @ 0xc0a0
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
  v5 = sub_B500((long long)a1, a2, v22, 0);
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
      sub_B420((long long)a1);
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
        if ( !(unsigned char)sub_BF20((long long)a1, v21) )
          return (unsigned int)-1;
        if ( sub_B500((long long)a1, a2, v22, 0) )
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
    v12 = sub_B600((long long)a1);
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



// Function: hash_insert @ 0xc290
long long hash_insert(long long *a1, long long a2)
{
  int v2; // eax
  bool v3; // zf
  long long result; // rax
  long long v5[3]; // [rsp+0h] [rbp-18h] BYREF

  v5[1] = __readfsqword(0x28u);
  v2 = sub_C0A0(a1, a2, v5);
  if ( v2 == -1 )
    return 0;
  v3 = v2 == 0;
  result = a2;
  if ( v3 )
    return v5[0];
  return result;
}



// Function: hash_remove @ 0xc2f0
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
  v3 = sub_B500((long long)a1, a2, v18, 1);
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
    sub_B420((long long)a1);
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
      if ( !(unsigned char)sub_BF20((long long)a1, v14) )
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


