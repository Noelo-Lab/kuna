// Function: duinfo_init @ 0x40da
long long *duinfo_init(long long *a1)
{
  *a1 = 0;
  a1[1] = 0;
  a1[2] = 0x8000000000000000LL;
  a1[3] = -1;
  return a1;
}



// Function: duinfo_set @ 0x411a
long long duinfo_set(long long *a1, long long a2, long long a3, long long a4)
{
  *a1 = a2;
  a1[1] = 1;
  a1[2] = a3;
  a1[3] = a4;
  return a3;
}



// Function: duinfo_add @ 0x415c
long long duinfo_add(long long *a1, long long *a2)
{
  long long v2; // rax
  long long result; // rax
  long long v4; // rdx

  if ( *a1 + *a2 < (unsigned long long)*a1 )
    v2 = -1;
  else
    v2 = *a1 + *a2;
  *a1 = v2;
  a1[1] += a2[1];
  result = sub_1489D(a1[2], a1[3], a2[2], a2[3]);
  if ( (int)result < 0 )
  {
    v4 = a2[3];
    result = a2[2];
    a1[2] = result;
    a1[3] = v4;
  }
  return result;
}



// Function: usage @ 0x41ff
void usage(int a1)
{
  long long v1; // rbx
  char *v2; // rax
  long long v3; // r12
  long long v4; // rbx
  char *v5; // rax
  FILE *v6; // rbx
  char *v7; // rdi
  FILE *v8; // rbx
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
  FILE *v24; // rbx
  char *v25; // rax

  if ( a1 )
  {
    v1 = qword_211D0;
    v2 = gettext("Try '%s --help' for more information.\n");
    fprintf(stderr, v2, v1);
  }
  else
  {
    v3 = qword_211D0;
    v4 = qword_211D0;
    v5 = gettext("Usage: %s [OPTION]... [FILE]...\n  or:  %s [OPTION]... --files0-from=F\n");
    printf(v5, v4, v3);
    v6 = stdout;
    v7 = gettext("Summarize device usage of the set of FILEs, recursively for directories.\n");
    fputs_unlocked(v7, v6);
    sub_3D79();
    v8 = stdout;
    v9 = gettext(
           "  -0, --null            end each output line with NUL, not newline\n"
           "  -a, --all             write counts for all files, not just directories\n"
           "      --apparent-size   print apparent sizes rather than device usage; although\n"
           "                          the apparent size is usually smaller, it may be\n"
           "                          larger due to holes in ('sparse') files, internal\n"
           "                          fragmentation, indirect blocks, and the like\n");
    fputs_unlocked(v9, v8);
    v10 = stdout;
    v11 = gettext(
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
            "                          --summarize\n");
    fputs_unlocked(v11, v10);
    v12 = stdout;
    v13 = gettext(
            "      --files0-from=F   summarize device usage of the\n"
            "                          NUL-terminated file names specified in file F;\n"
            "                          if F is -, then read names from standard input\n"
            "  -H                    equivalent to --dereference-args (-D)\n"
            "  -h, --human-readable  print sizes in human readable format (e.g., 1K 234M 2G)\n"
            "      --inodes          list inode usage information instead of block usage\n");
    fputs_unlocked(v13, v12);
    v14 = stdout;
    v15 = gettext(
            "  -k                    like --block-size=1K\n"
            "  -L, --dereference     dereference all symbolic links\n"
            "  -l, --count-links     count sizes many times if hard linked\n"
            "  -m                    like --block-size=1M\n");
    fputs_unlocked(v15, v14);
    v16 = stdout;
    v17 = gettext(
            "  -P, --no-dereference  don't follow any symbolic links (this is the default)\n"
            "  -S, --separate-dirs   for directories do not include size of subdirectories\n"
            "      --si              like -h, but use powers of 1000 not 1024\n"
            "  -s, --summarize       display only a total for each argument\n");
    fputs_unlocked(v17, v16);
    v18 = stdout;
    v19 = gettext(
            "  -t, --threshold=SIZE  exclude entries smaller than SIZE if positive,\n"
            "                          or entries greater than SIZE if negative\n"
            "      --time            show time of the last modification of any file in the\n"
            "                          directory, or any of its subdirectories\n"
            "      --time=WORD       show time as WORD instead of modification time:\n"
            "                          atime, access, use, ctime or status\n"
            "      --time-style=STYLE  show times using STYLE, which can be:\n"
            "                            full-iso, long-iso, iso, or +FORMAT;\n"
            "                            FORMAT is interpreted like in 'date'\n");
    fputs_unlocked(v19, v18);
    v20 = stdout;
    v21 = gettext(
            "  -X, --exclude-from=FILE  exclude files that match any pattern in FILE\n"
            "      --exclude=PATTERN    exclude files that match PATTERN\n"
            "  -x, --one-file-system    skip directories on different file systems\n");
    fputs_unlocked(v21, v20);
    v22 = stdout;
    v23 = gettext("      --help        display this help and exit\n");
    fputs_unlocked(v23, v22);
    v24 = stdout;
    v25 = gettext("      --version     output version information and exit\n");
    fputs_unlocked(v25, v24);
    sub_3DDB("DU");
    sub_3DAA();
    sub_3E10("du");
  }
  exit(a1);
}



// Function: hash_ins @ 0x43fb
bool hash_ins(long long a1, long long a2, long long a3)
{
  int v4; // [rsp+2Ch] [rbp-4h]

  v4 = sub_6BCF(a1, a3, a2);
  if ( v4 < 0 )
    sub_15992();
  return v4 != 0;
}



// Function: show_date @ 0x4441
unsigned long long show_date(long long a1, long long a2, long long a3, long long a4)
{
  long long v4; // rbx
  char *v5; // rax
  long long v7; // [rsp+0h] [rbp-A0h] BYREF
  long long v8; // [rsp+8h] [rbp-98h]
  long long v9; // [rsp+10h] [rbp-90h]
  long long v10; // [rsp+18h] [rbp-88h]
  char *s; // [rsp+28h] [rbp-78h]
  char v12[64]; // [rsp+30h] [rbp-70h] BYREF
  char v13[24]; // [rsp+70h] [rbp-30h] BYREF
  unsigned long long v14; // [rsp+88h] [rbp-18h]

  v10 = a1;
  v7 = a2;
  v8 = a3;
  v9 = a4;
  v14 = __readfsqword(0x28u);
  if ( sub_14623(a4, &v7, v12) )
  {
    sub_80DA(stdout, v10, v12, v9, (unsigned int)v8);
  }
  else
  {
    s = (char *)sub_40A7(v7, v13);
    v4 = sub_1401B(s);
    v5 = gettext("time %s is out of range");
    error(0, 0, v5, v4);
    fputs_unlocked(s, stdout);
  }
  return v14 - __readfsqword(0x28u);
}



// Function: print_only_size @ 0x4559
unsigned long long print_only_size(long long a1)
{
  FILE *v1; // rbx
  char *v2; // rax
  char v4[664]; // [rsp+10h] [rbp-2B0h] BYREF
  unsigned long long v5; // [rsp+2A8h] [rbp-18h]

  v5 = __readfsqword(0x28u);
  v1 = stdout;
  if ( a1 == -1 )
    v2 = gettext("Infinity");
  else
    v2 = (char *)sub_F661(a1, v4, (unsigned int)dword_21148, 1, qword_21170);
  fputs_unlocked(v2, v1);
  return v5 - __readfsqword(0x28u);
}



// Function: print_size @ 0x45f2
int print_size(long long *a1, const char *a2)
{
  long long v2; // rax
  long long v3; // rdx

  if ( byte_2114C )
    v2 = a1[1];
  else
    v2 = *a1;
  sub_4559(v2);
  if ( byte_2114D )
  {
    putchar_unlocked(9);
    sub_4441(qword_21160, a1[2], a1[3], qword_21168);
  }
  if ( byte_2113C )
    v3 = 0;
  else
    v3 = 10;
  printf("\t%s%c", a2, v3);
  return fflush_unlocked(stdout);
}



// Function: fill_mount_table @ 0x46a8
unsigned long long fill_mount_table()
{
  long long v1; // [rsp+0h] [rbp-B0h]
  long long v2; // [rsp+8h] [rbp-A8h]
  struct stat buf; // [rsp+10h] [rbp-A0h] BYREF
  unsigned long long v4; // [rsp+A8h] [rbp-8h]

  v4 = __readfsqword(0x28u);
  v1 = sub_11107(0);
  while ( v1 )
  {
    if ( (*(char *)(v1 + 40) & 2) == 0 && (*(char *)(v1 + 40) & 1) == 0 && !stat(*(const char **)(v1 + 8), &buf) )
      sub_43FB(qword_21128, buf.st_ino, buf.st_dev);
    v2 = v1;
    v1 = *(long long *)(v1 + 48);
    sub_11D9D(v2);
  }
  return v4 - __readfsqword(0x28u);
}



// Function: mount_point_in_fts_cycle @ 0x4795
long long mount_point_in_fts_cycle(long long *a1)
{
  long long *v2; // [rsp+8h] [rbp-18h]
  long long *v3; // [rsp+18h] [rbp-8h]

  v2 = a1;
  v3 = (long long *)*a1;
  if ( !qword_21128 )
  {
    qword_21128 = sub_6936();
    if ( !qword_21128 )
      sub_15992();
    sub_46A8();
  }
  while ( v2 && v2 != v3 )
  {
    if ( (int)sub_6C4B(qword_21128, v2[14], v2[15]) > 0 )
      return 1;
    v2 = (long long *)v2[1];
  }
  return 0;
}



// Function: process_file @ 0x4831
long long process_file(long long a1, long long a2)
{
  long long v2; // rbx
  char *v3; // rax
  long long v4; // rbx
  char *v5; // rax
  const char *v7; // rax
  long long v8; // rbx
  char *v9; // rax
  long long v10; // rax
  long long v11; // rdx
  long long v12; // rcx
  long long v13; // rsi
  unsigned long long v14; // rax
  bool v15; // al
  unsigned char v17; // [rsp+12h] [rbp-9Eh]
  char v18; // [rsp+13h] [rbp-9Dh]
  int v19; // [rsp+14h] [rbp-9Ch]
  unsigned long long i; // [rsp+18h] [rbp-98h]
  const char *v21; // [rsp+20h] [rbp-90h]
  long long *v22; // [rsp+28h] [rbp-88h]
  unsigned long long v23; // [rsp+40h] [rbp-70h]
  long long v24[4]; // [rsp+50h] [rbp-60h] BYREF
  unsigned long long v25; // [rsp+70h] [rbp-40h] BYREF
  unsigned long long v26; // [rsp+78h] [rbp-38h]
  long long v27; // [rsp+80h] [rbp-30h]
  long long v28; // [rsp+88h] [rbp-28h]
  unsigned long long v29; // [rsp+98h] [rbp-18h]

  v29 = __readfsqword(0x28u);
  v17 = 1;
  v21 = *(const char **)(a2 + 56);
  v22 = (long long *)(a2 + 112);
  v19 = *(unsigned short *)(a2 + 104);
  if ( v19 == 4 )
  {
    v2 = sub_13BD4(4, v21);
    v3 = gettext("cannot read directory %s");
    error(0, *(int *)(a2 + 64), v3, v2);
    v17 = 0;
LABEL_37:
    if ( dword_21150 )
    {
      if ( dword_21150 == 2 )
        v10 = sub_14093(v22);
      else
        v10 = sub_140AD(v22);
    }
    else
    {
      v10 = sub_140C7(v22);
    }
    if ( byte_21139 )
    {
      v12 = *(long long *)(a2 + 160);
      if ( v12 < 0 )
        v12 = 0;
      v13 = v12;
    }
    else
    {
      v13 = *(long long *)(a2 + 176) << 9;
    }
    sub_411A(v24, v13, v10, v11);
    v23 = *(long long *)(a2 + 88);
    v25 = v24[0];
    v26 = v24[1];
    v27 = v24[2];
    v28 = v24[3];
    if ( qword_211A0 )
    {
      if ( v23 != qword_21130 )
      {
        if ( v23 <= qword_21130 )
        {
          if ( v23 != qword_21130 - 1 )
            __assert_fail("level == prev_level - 1", "src/du.c", 0x27Eu, "process_file");
          sub_415C((long long *)&v25, (long long *)(qword_211A8 + (qword_21130 << 6)));
          if ( byte_2113E != 1 )
            sub_415C((long long *)&v25, (long long *)(qword_211A8 + (qword_21130 << 6) + 32));
          sub_415C((long long *)(qword_211A8 + (v23 << 6) + 32), (long long *)(qword_211A8 + (qword_21130 << 6)));
          sub_415C((long long *)(qword_211A8 + (v23 << 6) + 32), (long long *)(qword_211A8 + (qword_21130 << 6) + 32));
        }
        else
        {
          if ( v23 >= qword_211A0 )
          {
            qword_211A8 = sub_15307(qword_211A8, v23, 128);
            qword_211A0 = 2 * v23;
          }
          for ( i = qword_21130 + 1; i <= v23; ++i )
          {
            sub_40DA((long long *)(qword_211A8 + (i << 6)));
            sub_40DA((long long *)(qword_211A8 + (i << 6) + 32));
          }
        }
      }
    }
    else
    {
      qword_211A0 = v23 + 10;
      qword_211A8 = sub_15838(v23 + 10, 64);
    }
    qword_21130 = v23;
    if ( byte_2113E != 1 || v19 != 6 && v19 != 4 )
      sub_415C((long long *)(qword_211A8 + (v23 << 6)), v24);
    sub_415C(qword_21180, v24);
    if ( (v19 == 6 || v19 == 4) && v23 <= qword_21010 || byte_21138 && v23 <= qword_21010 || !v23 )
    {
      if ( byte_2114C )
        v14 = v26;
      else
        v14 = v25;
      if ( qword_21140 >= 0 )
        v15 = v14 >= qword_21140;
      else
        v15 = v14 <= -qword_21140;
      if ( v15 )
        sub_45F2((long long *)&v25, v21);
    }
    return v17;
  }
  if ( v19 == 6 )
    goto LABEL_37;
  v18 = sub_76F0(qword_21178, v21);
  if ( v18 != 1 )
  {
    if ( v19 == 11 )
    {
      sub_C3CB(a1, a2, 1);
      if ( sub_BD55(a1) != a2 )
        __assert_fail("e == ent", "src/du.c", 0x20Fu, "process_file");
      v19 = *(unsigned short *)(a2 + 104);
    }
    if ( v19 == 10 || v19 == 13 )
    {
      v4 = sub_13BD4(4, v21);
      v5 = gettext("cannot access %s");
      error(0, *(int *)(a2 + 64), v5, v4);
      return 0;
    }
    if ( (*(int *)(a1 + 72) & 0x40) != 0 && *(long long *)(a2 + 88) > 0 && *(long long *)(a1 + 24) != *v22 )
      v18 = 1;
  }
  if ( !v18
    && (byte_2113A == 1
     || !byte_2113B && ((*(int *)(a2 + 136) & 0xF000) == 0x4000 || *(long long *)(a2 + 128) <= 1u)
     || sub_43FB(qword_21120, *(long long *)(a2 + 120), *(long long *)(a2 + 112))) )
  {
    if ( v19 == 7 )
    {
      v7 = (const char *)sub_13D66(0, 3, v21);
      error(0, *(int *)(a2 + 64), "%s", v7);
      v17 = 0;
    }
    else if ( v19 <= 7 )
    {
      if ( v19 == 1 )
        return 1;
      if ( v19 == 2 )
      {
        if ( !(unsigned char)sub_15A4D(a1, a2) || (unsigned char)sub_4795((long long *)a2) == 1 )
          return 1;
        v8 = sub_13D66(0, 3, v21);
        v9 = gettext(
               "WARNING: Circular directory structure.\n"
               "This almost certainly means that you have a corrupted file system.\n"
               "NOTIFY YOUR SYSTEM MANAGER.\n"
               "The following directory is part of the cycle:\n"
               "  %s\n");
        error(0, 0, v9, v8);
        return 0;
      }
    }
    goto LABEL_37;
  }
  if ( v19 == 1 )
  {
    sub_C3CB(a1, a2, 4);
    if ( sub_BD55(a1) != a2 )
      __assert_fail("e == ent", "src/du.c", 0x230u, "process_file");
  }
  return 1;
}



// Function: du_files @ 0x505b
long long du_files(long long *a1, unsigned int a2)
{
  long long v2; // r12
  char *v3; // rbx
  int *v4; // rax
  char *v5; // rbx
  int *v6; // rax
  bool v8; // [rsp+1Fh] [rbp-21h]
  long long i; // [rsp+20h] [rbp-20h]
  long long v10; // [rsp+28h] [rbp-18h]

  v8 = 1;
  if ( *a1 )
  {
    for ( i = sub_159D2(a1, a2, 0); ; v8 = (v8 & (unsigned char)sub_4831(i, v10)) != 0 )
    {
      v10 = sub_BD55(i);
      if ( !v10 )
        break;
    }
    if ( *__errno_location() )
    {
      v2 = sub_13D66(0, 3, *(long long *)(i + 32));
      v3 = gettext("fts_read failed: %s");
      v4 = __errno_location();
      error(0, *v4, v3, v2);
      v8 = 0;
    }
    qword_21130 = 0;
    if ( (unsigned int)sub_B89B(i) )
    {
      v5 = gettext("fts_close failed");
      v6 = __errno_location();
      error(0, *v6, v5);
      return 0;
    }
  }
  return v8;
}



// Function: main @ 0x5197
long long main(int a1, char **a2, char **a3)
{
  char *v3; // rax
  long long v4; // rbx
  char *v5; // rax
  char *v6; // rax
  const char *v7; // rbx
  int *v8; // rax
  int v9; // eax
  char *v10; // rax
  char *v11; // rax
  char *v12; // rax
  char *v13; // rax
  char *v14; // rax
  unsigned int v15; // eax
  long long v16; // rbx
  char *v17; // rax
  char *v18; // rax
  long long v19; // r12
  char *v20; // rbx
  int *v21; // rax
  char **v22; // rax
  bool v23; // al
  long long v24; // r12
  char *v25; // rbx
  int *v26; // rax
  long long v27; // rbx
  char *v28; // rax
  char *v29; // rax
  char *v30; // rbx
  const char *v31; // rax
  long long v32; // rbx
  char *v33; // rax
  char *v34; // rax
  char v36; // [rsp+14h] [rbp-8Ch]
  char v37; // [rsp+15h] [rbp-8Bh]
  char v38; // [rsp+16h] [rbp-8Ah]
  char v39; // [rsp+17h] [rbp-89h]
  int longind; // [rsp+18h] [rbp-88h] BYREF
  long long v41; // [rsp+1Ch] [rbp-84h]
  int v42; // [rsp+24h] [rbp-7Ch]
  unsigned int v43; // [rsp+28h] [rbp-78h]
  unsigned int v44; // [rsp+2Ch] [rbp-74h]
  long long v45; // [rsp+30h] [rbp-70h] BYREF
  char *s1; // [rsp+38h] [rbp-68h]
  long long v47; // [rsp+40h] [rbp-60h]
  long long v48; // [rsp+48h] [rbp-58h]
  char *v49; // [rsp+50h] [rbp-50h]
  char **v50; // [rsp+58h] [rbp-48h]
  char *v51; // [rsp+60h] [rbp-40h]
  long long v52; // [rsp+68h] [rbp-38h]
  long long v53[6]; // [rsp+70h] [rbp-30h] BYREF

  v53[3] = __readfsqword(0x28u);
  v36 = 0;
  v37 = 1;
  s1 = 0;
  v41 = 0x1000000008LL;
  v38 = 0;
  v53[0] = sub_40CC(".", a2, a3);
  v53[1] = 0;
  sub_11FDA(*a2);
  setlocale(6, locale);
  bindtextdomain("coreutils", "/usr/local/share/locale");
  textdomain("coreutils");
  sub_186C0(sub_6762);
  qword_21178 = sub_6EBE();
  v3 = getenv("DU_BLOCK_SIZE");
  sub_102CD(v3, &dword_21148, &qword_21170);
  while ( 1 )
  {
    longind = -1;
    v42 = getopt_long(a1, a2, "0abd:chHklmst:xB:DLPSX:", &longopts, &longind);
    if ( v42 == -1 )
      break;
    if ( v42 > 135 )
      goto LABEL_49;
    if ( v42 >= 48 )
    {
      switch ( v42 )
      {
        case 48:
          byte_2113C = 1;
          continue;
        case 66:
          v44 = sub_102CD(optarg, &dword_21148, &qword_21170);
          if ( v44 )
            sub_160C2(v44, (unsigned int)longind, (unsigned int)(char)v42, &longopts, optarg);
          continue;
        case 68:
        case 72:
          HIDWORD(v41) = 17;
          continue;
        case 76:
          HIDWORD(v41) = 2;
          continue;
        case 80:
          HIDWORD(v41) = 16;
          continue;
        case 83:
          byte_2113E = 1;
          continue;
        case 88:
          if ( (unsigned int)sub_7DAC(sub_77E3, qword_21178, optarg, 0x10000000, 10) )
          {
            v7 = (const char *)sub_13D66(0, 3, optarg);
            v8 = __errno_location();
            error(0, *v8, "%s", v7);
            v37 = 0;
          }
          continue;
        case 97:
          byte_21138 = 1;
          continue;
        case 98:
          byte_21139 = 1;
          dword_21148 = 0;
          qword_21170 = 1;
          continue;
        case 99:
          byte_2113D = 1;
          continue;
        case 100:
          if ( (unsigned int)sub_161E3(optarg, 0, 0, &v45, locale) )
          {
            v4 = sub_1401B(optarg);
            v5 = gettext("invalid maximum depth %s");
            error(0, 0, v5, v4);
            v37 = 0;
          }
          else
          {
            v36 = 1;
            qword_21010 = v45;
          }
          continue;
        case 104:
          dword_21148 = 176;
          qword_21170 = 1;
          continue;
        case 107:
          dword_21148 = 0;
          qword_21170 = 1024;
          continue;
        case 108:
          byte_2113A = 1;
          continue;
        case 109:
          dword_21148 = 0;
          qword_21170 = 0x100000;
          continue;
        case 115:
          v38 = 1;
          continue;
        case 116:
          v43 = sub_15B9A(optarg, 0, 0, &qword_21140, "kKmMGTPEZY0");
          if ( v43 )
            sub_160C2(v43, (unsigned int)longind, (unsigned int)(char)v42, &longopts, optarg);
          if ( qword_21140 || *(char *)optarg != 45 )
            continue;
          v6 = gettext("invalid --threshold argument '-0'");
          error(1, 0, v6);
LABEL_30:
          LODWORD(v41) = v41 | 0x40;
          break;
        case 120:
          goto LABEL_30;
        case 128:
          byte_21139 = 1;
          continue;
        case 129:
          sub_77E3(qword_21178, optarg, 0x10000000);
          continue;
        case 130:
          s1 = (char *)optarg;
          continue;
        case 131:
          dword_21148 = 144;
          qword_21170 = 1;
          continue;
        case 133:
          byte_2114D = 1;
          if ( optarg )
            v9 = dword_194A0[sub_63C9(
                               (unsigned int)"--time",
                               optarg,
                               (unsigned int)off_209C0,
                               (unsigned int)dword_194A0,
                               4,
                               (int)off_21020,
                               1)];
          else
            v9 = 0;
          dword_21150 = v9;
          v10 = getenv("TZ");
          qword_21168 = sub_141A3(v10);
          continue;
        case 134:
          ::s1 = (char *)optarg;
          continue;
        case 135:
          byte_2114C = 1;
          continue;
        default:
          goto LABEL_49;
      }
    }
    else
    {
      if ( v42 == -131 )
      {
        sub_1517A(
          (int)stdout,
          (unsigned int)"du",
          (unsigned int)"GNU coreutils",
          (int)off_21018,
          (unsigned int)"Torbjorn Granlund",
          (unsigned int)"David MacKenzie",
          "Paul Eggert",
          "Jim Meyering",
          0);
        exit(0);
      }
      if ( v42 == -130 )
        sub_41FF(0);
LABEL_49:
      v37 = 0;
    }
  }
  if ( v37 != 1 )
    sub_41FF(1);
  if ( byte_21138 && v38 )
  {
    v11 = gettext("cannot both summarize and show all entries");
    error(0, 0, v11);
    sub_41FF(1);
  }
  if ( v38 && v36 && !qword_21010 )
  {
    v12 = gettext("warning: summarizing is the same as using --max-depth=0");
    error(0, 0, v12);
  }
  if ( v38 && v36 && qword_21010 )
  {
    v48 = qword_21010;
    v13 = gettext("warning: summarizing conflicts with --max-depth=%lu");
    error(0, 0, v13, v48);
    sub_41FF(1);
  }
  if ( v38 )
    qword_21010 = 0;
  if ( byte_2114C )
  {
    if ( byte_21139 )
    {
      v14 = gettext("warning: options --apparent-size and -b are ineffective with --inodes");
      error(0, 0, v14);
    }
    qword_21170 = 1;
  }
  if ( byte_2114D )
  {
    if ( !::s1 )
    {
      ::s1 = getenv("TIME_STYLE");
      if ( ::s1 && strcmp(::s1, "locale") )
      {
        if ( *::s1 == 43 )
        {
          v49 = strchr(::s1, 10);
          if ( v49 )
            *v49 = 0;
        }
        else
        {
          while ( !strncmp(::s1, "posix-", 6u) )
            ::s1 += 6;
        }
      }
      else
      {
        ::s1 = "long-iso";
      }
    }
    if ( *::s1 == 43 )
    {
      qword_21160 = (long long)(::s1 + 1);
    }
    else
    {
      v15 = dword_194D0[sub_63C9(
                          (unsigned int)"time style",
                          (int)::s1,
                          (unsigned int)off_20A00,
                          (unsigned int)dword_194D0,
                          4,
                          (int)off_21020,
                          1)];
      if ( v15 == 2 )
      {
        qword_21160 = (long long)"%Y-%m-%d";
      }
      else if ( v15 <= 2 )
      {
        if ( v15 )
          qword_21160 = (long long)"%Y-%m-%d %H:%M";
        else
          qword_21160 = (long long)"%Y-%m-%d %H:%M:%S.%N %z";
      }
    }
  }
  if ( s1 )
  {
    if ( a1 > optind )
    {
      v16 = sub_1401B(a2[optind]);
      v17 = gettext("extra operand %s");
      error(0, 0, v17, v16);
      v18 = gettext("file operands cannot be combined with --files0-from");
      fprintf(stderr, "%s\n", v18);
      sub_41FF(1);
    }
    if ( strcmp(s1, "-") && !sub_ABEA(s1, "r", stdin) )
    {
      v19 = sub_13BD4(4, s1);
      v20 = gettext("cannot open %s for reading");
      v21 = __errno_location();
      error(1, *v21, v20, v19);
    }
    v47 = sub_655D(stdin);
    byte_2113B = 1;
  }
  else
  {
    if ( a1 <= optind )
      v22 = (char **)v53;
    else
      v22 = &a2[optind];
    v50 = v22;
    v47 = sub_6508(v22);
    v23 = a1 > optind + 1 || HIDWORD(v41) == 2;
    byte_2113B = v23;
  }
  if ( !v47 )
    sub_15992();
  qword_21120 = sub_6936();
  if ( !qword_21120 )
    sub_15992();
  if ( byte_2113A || byte_2113B != 1 )
    LODWORD(v41) = v41 | 0x100;
  LODWORD(v41) = HIDWORD(v41) | v41;
  while ( 1 )
  {
    v39 = 0;
    v51 = (char *)sub_65CA(v47, &v45);
    if ( !v51 )
      break;
    if ( s1 && !strcmp(s1, "-") && !strcmp(v51, "-") )
    {
      v27 = sub_13BD4(4, v51);
      v28 = gettext("when reading file names from stdin, no file name of %s allowed");
      error(0, 0, v28, v27);
      v39 = 1;
    }
    if ( !*v51 )
    {
      if ( s1 )
      {
        v52 = sub_66B5(v47);
        v30 = gettext("invalid zero-length file name");
        v31 = (const char *)sub_13D66(0, 3, s1);
        error(0, 0, "%s:%lu: %s", v31, v52, v30);
      }
      else
      {
        v29 = gettext("invalid zero-length file name");
        error(0, 0, "%s", v29);
      }
      v39 = 1;
    }
    if ( v39 )
    {
      v37 = 0;
    }
    else
    {
      qword_211B0 = (long long)v51;
      v37 &= sub_505B(&qword_211B0, v41);
    }
  }
  if ( (int)v45 == 4 )
  {
    v24 = sub_13D66(0, 3, s1);
    v25 = gettext("%s: read error");
    v26 = __errno_location();
    error(0, *v26, v25, v24);
    v37 = 0;
  }
  else
  {
    if ( (unsigned int)v45 > 4 )
      goto LABEL_120;
    if ( (int)v45 != 2 )
    {
      if ( (int)v45 == 3 )
        sub_15992();
LABEL_120:
      __assert_fail("!\"unexpected error code from argv_iter\"", "src/du.c", 0x439u, "main");
    }
  }
  sub_66F3(v47);
  sub_69C8(qword_21120);
  if ( qword_21128 )
    sub_69C8(qword_21128);
  if ( s1 && (ferror_unlocked(stdin) || (unsigned int)sub_7EA0(stdin)) && v37 )
  {
    v32 = sub_13BD4(4, s1);
    v33 = gettext("error reading %s");
    error(1, 0, v33, v32);
  }
  if ( byte_2113D )
  {
    v34 = gettext("total");
    sub_45F2(qword_21180, v34);
  }
  return (unsigned char)v37 ^ 1u;
}



// Function: hash_get_n_buckets @ 0xd9a8
long long hash_get_n_buckets(long long a1)
{
  return *(long long *)(a1 + 16);
}



// Function: hash_get_n_buckets_used @ 0xd9be
long long hash_get_n_buckets_used(long long a1)
{
  return *(long long *)(a1 + 24);
}



// Function: hash_get_n_entries @ 0xd9d4
long long hash_get_n_entries(long long a1)
{
  return *(long long *)(a1 + 32);
}



// Function: hash_get_max_bucket_length @ 0xd9ea
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



// Function: hash_table_ok @ 0xda6c
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



// Function: hash_print_statistics @ 0xdb0a
int hash_print_statistics(long long a1, FILE *a2)
{
  double v2; // xmm0_8
  double v3; // xmm1_8
  double v4; // xmm0_8
  long long v6; // [rsp+10h] [rbp-20h]
  long long v7; // [rsp+18h] [rbp-18h]
  long long v8; // [rsp+20h] [rbp-10h]
  unsigned long long v9; // [rsp+28h] [rbp-8h]

  v6 = sub_D9D4(a1);
  v7 = sub_D9A8(a1);
  v8 = sub_D9BE(a1);
  v9 = sub_D9EA(a1);
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



// Function: safe_hasher @ 0xdc51
long long safe_hasher(long long a1, long long a2)
{
  unsigned long long v3; // [rsp+18h] [rbp-8h]

  v3 = (*(long long (**)(long long, long long))(a1 + 48))(a2, *(long long *)(a1 + 16));
  if ( v3 >= *(long long *)(a1 + 16) )
    abort();
  return *(long long *)a1 + 16 * v3;
}



// Function: hash_lookup @ 0xdcac
long long hash_lookup(long long a1, long long a2)
{
  long long *i; // [rsp+10h] [rbp-10h]
  long long *v4; // [rsp+18h] [rbp-8h]

  v4 = (long long *)sub_DC51(a1, a2);
  if ( !*v4 )
    return 0;
  for ( i = v4; i; i = (long long *)i[1] )
  {
    if ( a2 == *i || (*(unsigned char (**)(long long, long long))(a1 + 56))(a2, *i) )
      return *i;
  }
  return 0;
}



// Function: hash_get_first @ 0xdd43
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



// Function: hash_get_next @ 0xdda3
long long hash_get_next(long long a1, long long a2)
{
  unsigned long long v3; // [rsp+10h] [rbp-10h]
  unsigned long long v4; // [rsp+18h] [rbp-8h]

  v3 = sub_DC51(a1, a2);
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



// Function: hash_get_entries @ 0xde41
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



// Function: hash_do_for_each @ 0xdee1
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



// Function: hash_string @ 0xdf79
unsigned long long hash_string(char *a1, unsigned long long a2)
{
  unsigned long long v4; // [rsp+18h] [rbp-8h]

  v4 = 0;
  while ( *a1 )
    v4 = (31 * v4 + (unsigned char)*a1++) % a2;
  return v4;
}



// Function: is_prime @ 0xdfd3
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



// Function: next_prime @ 0xe042
unsigned long long next_prime(unsigned long long a1)
{
  long long v2; // [rsp+0h] [rbp-8h]
  unsigned long long i; // [rsp+0h] [rbp-8h]

  v2 = a1;
  if ( a1 <= 9 )
    v2 = 10;
  for ( i = v2 | 1; i != -1 && !sub_DFD3(i); i += 2LL )
    ;
  return i;
}



// Function: hash_reset_tuning @ 0xe08d
long long hash_reset_tuning(long long a1)
{
  *(int *)a1 = 0;
  *(int *)(a1 + 4) = 1065353216;
  *(int *)(a1 + 8) = 1061997773;
  *(int *)(a1 + 12) = 1068826100;
  *(char *)(a1 + 16) = 0;
  return a1;
}



// Function: raw_hasher @ 0xe0e3
unsigned long long raw_hasher(long long a1, unsigned long long a2)
{
  return sub_175F6(a1, 3) % a2;
}



// Function: raw_comparator @ 0xe11e
bool raw_comparator(long long a1, long long a2)
{
  return a1 == a2;
}



// Function: check_tuning @ 0xe13b
long long check_tuning(long long a1)
{
  long long v2; // [rsp+10h] [rbp-8h]

  v2 = *(long long *)(a1 + 40);
  if ( (_UNKNOWN *)v2 == &unk_1AA40 )
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
  *(long long *)(a1 + 40) = &unk_1AA40;
  return 0;
}



// Function: compute_bucket_size @ 0xe248
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
  v6 = sub_E042(v5);
  v4 = v6 >> 61 != 0;
  if ( (v6 & 0x1000000000000000LL) != 0 )
    v4 = 1;
  if ( v4 )
    return 0;
  else
    return v6;
}



// Function: hash_initialize @ 0xe361
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
    v8 = sub_E0E3;
  if ( !a4 )
    v7 = sub_E11E;
  ptr = (size_t *)malloc(0x50u);
  if ( !ptr )
    return 0;
  if ( !a2 )
    v9 = &unk_1AA40;
  ptr[5] = (size_t)v9;
  if ( (unsigned char)sub_E13B((long long)ptr) == 1
    && (ptr[2] = sub_E248(a1, (long long)v9)) != 0
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



// Function: hash_clear @ 0xe4d6
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



// Function: hash_free @ 0xe5e1
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



// Function: allocate_entry @ 0xe710
void *allocate_entry(long long a1)
{
  long long v2; // [rsp+18h] [rbp-8h]

  if ( !*(long long *)(a1 + 72) )
    return malloc(0x10u);
  v2 = *(long long *)(a1 + 72);
  *(long long *)(a1 + 72) = *(long long *)(v2 + 8);
  return (void *)v2;
}



// Function: free_entry @ 0xe75f
long long free_entry(long long a1, long long *a2)
{
  *a2 = 0;
  a2[1] = *(long long *)(a1 + 72);
  *(long long *)(a1 + 72) = a2;
  return a1;
}



// Function: hash_find_entry @ 0xe799
long long hash_find_entry(long long a1, long long a2, long long *a3, char a4)
{
  long long v5; // rdx
  long long *i; // [rsp+20h] [rbp-30h]
  long long *v9; // [rsp+28h] [rbp-28h]
  long long v10; // [rsp+30h] [rbp-20h]
  long long *v11; // [rsp+38h] [rbp-18h]
  long long v12; // [rsp+40h] [rbp-10h]
  long long *v13; // [rsp+48h] [rbp-8h]

  v9 = (long long *)sub_DC51(a1, a2);
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
        sub_E75F(a1, v13);
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
      sub_E75F(a1, v11);
    }
    return v10;
  }
}



// Function: transfer_entries @ 0xe932
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
        v9 = (long long *)sub_DC51(a1, *j);
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
          sub_E75F(a1, j);
        }
      }
      v8 = *i;
      i[1] = 0;
      if ( !a3 )
      {
        v10 = (long long *)sub_DC51(a1, v8);
        if ( *v10 )
        {
          v11 = sub_E710(a1);
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



// Function: hash_rehash @ 0xeb06
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
  nmemb = sub_E248(a2, *(long long *)(a1 + 40));
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
  if ( (unsigned char)sub_E932((long long)&v4, a1, 0) )
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
    if ( (unsigned char)sub_E932(a1, (long long)&v4, 1) != 1 || (unsigned char)sub_E932(a1, (long long)&v4, 0) != 1 )
      abort();
    free(v4);
    return 0;
  }
}



// Function: hash_insert_if_absent @ 0xed31
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
  v26 = sub_E799((long long)a1, a2, &v25, 0);
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
      sub_E13B((long long)a1);
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
        if ( (unsigned char)sub_EB06((long long)a1, v23) != 1 )
          return 0xFFFFFFFFLL;
        if ( sub_E799((long long)a1, a2, &v25, 0) )
          abort();
      }
    }
    if ( *v25 )
    {
      v28 = sub_E710((long long)a1);
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



// Function: hash_insert @ 0xf0a2
long long hash_insert(long long *a1, long long a2)
{
  int v3; // [rsp+1Ch] [rbp-14h]
  long long v4[2]; // [rsp+20h] [rbp-10h] BYREF

  v4[1] = __readfsqword(0x28u);
  v3 = sub_ED31(a1, a2, v4);
  if ( v3 == -1 )
    return 0;
  if ( v3 )
    return a2;
  return v4[0];
}



// Function: hash_remove @ 0xf112
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
  v26 = sub_E799(a1, a2, &v24, 1);
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
      sub_E13B(a1);
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
        if ( (unsigned char)sub_EB06(a1, v19) != 1 )
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



// Function: hash_delete @ 0xf418
long long hash_delete(long long a1, long long a2)
{
  return sub_F112(a1, a2);
}


