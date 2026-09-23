// Function: usage @ 0x2aa2
void usage(int a1)
{
  long long v1; // rbx
  char *v2; // rax
  char *v3; // rax
  FILE *v4; // rbx
  char *v5; // rdi
  FILE *v6; // rbx
  char *v7; // rax
  FILE *v8; // rbx
  char *v9; // rax
  FILE *v10; // rbx
  char *v11; // rax
  FILE *v12; // rbx
  char *v13; // rax

  v1 = qword_18158;
  if ( a1 )
  {
    v2 = gettext("Try '%s --help' for more information.\n");
    fprintf(stderr, v2, v1);
  }
  else
  {
    v3 = gettext("Usage: %s [-WIDTH] [OPTION]... [FILE]...\n");
    printf(v3, v1);
    v4 = stdout;
    v5 = gettext(
           "Reformat each paragraph in the FILE(s), writing to standard output.\n"
           "The option -WIDTH is an abbreviated form of --width=DIGITS.\n");
    fputs_unlocked(v5, v4);
    sub_27A9();
    sub_27DA();
    v6 = stdout;
    v7 = gettext(
           "  -c, --crown-margin        preserve indentation of first two lines\n"
           "  -p, --prefix=STRING       reformat only lines beginning with STRING,\n"
           "                              reattaching the prefix to reformatted lines\n"
           "  -s, --split-only          split long lines, but do not refill\n");
    fputs_unlocked(v7, v6);
    v8 = stdout;
    v9 = gettext(
           "  -t, --tagged-paragraph    indentation of first line different from second\n"
           "  -u, --uniform-spacing     one space between words, two after sentences\n"
           "  -w, --width=WIDTH         maximum line width (default of 75 columns)\n"
           "  -g, --goal=WIDTH          goal width (default of 93% of width)\n");
    fputs_unlocked(v9, v8);
    v10 = stdout;
    v11 = gettext("      --help        display this help and exit\n");
    fputs_unlocked(v11, v10);
    v12 = stdout;
    v13 = gettext("      --version     output version information and exit\n");
    fputs_unlocked(v13, v12);
    sub_280B("fmt");
  }
  exit(a1);
}



// Function: main @ 0x2bde
long long main(int a1, char **a2, char **a3)
{
  char *v3; // rax
  char *v4; // rax
  char *v5; // rax
  long long v6; // r12
  char *v7; // rbx
  int *v8; // rax
  char *v9; // rbx
  int *v10; // rax
  char **argv; // [rsp+0h] [rbp-50h]
  int argc; // [rsp+Ch] [rbp-44h]
  char v14; // [rsp+1Ah] [rbp-36h]
  char v15; // [rsp+1Bh] [rbp-35h]
  int v16; // [rsp+1Ch] [rbp-34h]
  char *v17; // [rsp+20h] [rbp-30h]
  long long v18; // [rsp+28h] [rbp-28h]
  char *s1; // [rsp+30h] [rbp-20h]
  FILE *v20; // [rsp+38h] [rbp-18h]

  argc = a1;
  argv = a2;
  v14 = 1;
  v17 = 0;
  v18 = 0;
  sub_498F(*a2, a2, a3);
  setlocale(6, locale);
  bindtextdomain("coreutils", "/usr/local/share/locale");
  textdomain("coreutils");
  sub_8540(sub_4625);
  byte_D103 = 0;
  byte_D102 = 0;
  byte_D101 = 0;
  byte_D100 = 0;
  dword_D110 = 75;
  qword_D108 = (char *)locale;
  dword_D114 = 0;
  dword_D118 = 0;
  dword_D11C = 0;
  if ( a1 > 1 && *a2[1] == 45 && (unsigned int)(a2[1][1] - 48) <= 9 )
  {
    v17 = a2[1] + 1;
    a2[1] = *a2;
    argv = a2 + 1;
    argc = a1 - 1;
  }
  while ( 2 )
  {
    v16 = getopt_long(argc, argv, "0123456789cstuw:p:g:", &longopts, 0);
    if ( v16 != -1 )
    {
      if ( v16 <= 119 )
      {
        if ( v16 >= 99 )
        {
          switch ( v16 )
          {
            case 'c':
              byte_D100 = 1;
              continue;
            case 'g':
              v18 = optarg;
              continue;
            case 'p':
              sub_312D(optarg);
              continue;
            case 's':
              byte_D102 = 1;
              continue;
            case 't':
              byte_D101 = 1;
              continue;
            case 'u':
              byte_D103 = 1;
              continue;
            case 'w':
              v17 = (char *)optarg;
              continue;
            default:
              goto LABEL_11;
          }
        }
        if ( v16 == -131 )
        {
          sub_7168(
            (int)stdout,
            (unsigned int)"fmt",
            (unsigned int)"GNU coreutils",
            (int)off_D010,
            (unsigned int)"Ross Paterson",
            0,
            (char)argv);
          exit(0);
        }
        if ( v16 == -130 )
          sub_2AA2(0);
      }
LABEL_11:
      if ( (unsigned int)(v16 - 48) <= 9 )
      {
        v3 = gettext("invalid option -- %c; -WIDTH is recognized only when it is the first\noption; use -w N instead");
        error(0, 0, v3, (unsigned int)v16);
      }
      sub_2AA2(1);
    }
    break;
  }
  if ( v17 )
  {
    v4 = gettext("invalid width");
    dword_D110 = sub_7AFA(v17, 0, 2500, locale, v4, 0);
  }
  if ( v18 )
  {
    v5 = gettext("invalid width");
    dword_D120 = sub_7AFA(v18, 0, dword_D110, locale, v5, 0);
    if ( !v17 )
      dword_D110 = dword_D120 + 10;
  }
  else
  {
    dword_D120 = 187 * dword_D110 / 200;
  }
  v15 = 0;
  if ( argc == optind )
  {
    v15 = 1;
    v14 = sub_31D1(stdin, "-");
  }
  else
  {
    while ( argc > optind )
    {
      s1 = argv[optind];
      if ( !strcmp(s1, "-") )
      {
        v14 = (unsigned char)(v14 & sub_31D1(stdin, s1)) != 0;
        v15 = 1;
      }
      else
      {
        v20 = fopen(s1, "r");
        if ( v20 )
        {
          v14 = (unsigned char)(v14 & sub_31D1(v20, s1)) != 0;
        }
        else
        {
          v6 = sub_6589(4, s1);
          v7 = gettext("cannot open %s for reading");
          v8 = __errno_location();
          error(0, *v8, v7, v6);
          v14 = 0;
        }
      }
      ++optind;
    }
  }
  if ( v15 && (unsigned int)sub_4791(stdin) )
  {
    v9 = gettext("closing standard input");
    v10 = __errno_location();
    error(1, *v10, "%s", v9);
  }
  return (unsigned char)v14 ^ 1u;
}



// Function: set_prefix @ 0x312d
signed long long set_prefix(char *a1)
{
  char *i; // [rsp+18h] [rbp-8h]

  dword_D118 = 0;
  while ( *a1 == 32 )
  {
    ++dword_D118;
    ++a1;
  }
  qword_D108 = a1;
  dword_D114 = strlen(a1);
  for ( i = &a1[dword_D114]; i > a1 && *(i - 1) == 32; --i )
    ;
  *i = 0;
  dword_D11C = (int)i - (int)a1;
  return i - a1;
}



// Function: fmt @ 0x31d1
long long fmt(FILE *a1, long long a2)
{
  int v2; // eax
  long long v3; // rbx
  char *v4; // rax
  int errnum; // [rsp+1Ch] [rbp-14h]

  sub_4752(a1, 2);
  byte_18128 = 0;
  dword_18134 = 0;
  dword_18138 = sub_3978(a1);
  while ( (unsigned char)sub_33B7(a1) )
  {
    sub_3DB9();
    sub_411D(qword_18120);
  }
  if ( ferror_unlocked(a1) )
    v2 = 0;
  else
    v2 = -1;
  errnum = v2;
  if ( a1 == (FILE *)stdin )
  {
    clearerr_unlocked(a1);
  }
  else if ( (unsigned int)sub_4791(a1) && errnum < 0 )
  {
    errnum = *__errno_location();
  }
  if ( errnum >= 0 )
  {
    v3 = sub_671B(0, 3, a2);
    if ( errnum )
      v4 = "%s";
    else
      v4 = gettext("read error");
    error(0, errnum, v4, v3);
  }
  return (unsigned int)errnum >> 31;
}



// Function: set_other_indent @ 0x32fb
long long set_other_indent(char a1)
{
  long long result; // rax

  if ( byte_D102 )
  {
    dword_18134 = dword_18130;
    return (unsigned int)dword_18130;
  }
  else if ( byte_D100 )
  {
    if ( a1 )
      result = (unsigned int)dword_D124;
    else
      result = (unsigned int)dword_18130;
    dword_18134 = result;
  }
  else if ( byte_D101 )
  {
    if ( !a1 || dword_D124 == dword_18130 )
    {
      result = (unsigned int)dword_18130;
      if ( dword_18134 == dword_18130 )
      {
        if ( dword_18130 )
          result = 0;
        else
          result = 3;
        dword_18134 = result;
      }
    }
    else
    {
      dword_18134 = dword_D124;
      return (unsigned int)dword_D124;
    }
  }
  else
  {
    dword_18134 = dword_18130;
    return (unsigned int)dword_18130;
  }
  return result;
}



// Function: get_paragraph @ 0x33b7
long long get_paragraph(long long a1)
{
  char v2; // al
  long long v3; // rax
  unsigned int i; // [rsp+1Ch] [rbp-4h]
  unsigned int v5; // [rsp+1Ch] [rbp-4h]

  dword_18140 = 0;
  for ( i = dword_18138;
        i == 10 || i == -1 || dword_1813C < dword_D118 || dword_D114 + dword_1813C > dword_D124;
        i = sub_3978(a1) )
  {
    if ( (unsigned int)sub_3630(a1, i) == -1 )
    {
      dword_18138 = -1;
      return 0;
    }
    putchar_unlocked(10);
  }
  dword_1812C = dword_1813C;
  dword_18130 = dword_D124;
  qword_E4C8 = (long long)&unk_D140;
  qword_18120 = (long long)&unk_E4E0;
  v5 = sub_3793(a1, i);
  v2 = sub_3743(v5);
  sub_32FB(v2);
  if ( !byte_D102 )
  {
    if ( byte_D100 )
    {
      if ( (unsigned char)sub_3743(v5) )
      {
        do
          v5 = sub_3793(a1, v5);
        while ( (unsigned char)sub_3743(v5) && dword_D124 == dword_18134 );
      }
    }
    else if ( byte_D101 )
    {
      if ( (unsigned char)sub_3743(v5) && dword_D124 != dword_18130 )
      {
        do
          v5 = sub_3793(a1, v5);
        while ( (unsigned char)sub_3743(v5) && dword_D124 == dword_18134 );
      }
    }
    else
    {
      while ( (unsigned char)sub_3743(v5) && dword_D124 == dword_18134 )
        v5 = sub_3793(a1, v5);
    }
  }
  if ( qword_18120 <= (unsigned long long)&unk_E4E0 )
    __assert_fail("word < word_limit", "src/fmt.c", 0x270u, "get_paragraph");
  v3 = qword_18120 - 40;
  *(char *)(v3 + 16) = *(char *)(qword_18120 - 40 + 16) | 8;
  *(char *)(qword_18120 - 40 + 16) = (2 * ((*(char *)(v3 + 16) & 8) != 0)) | *(char *)(qword_18120 - 40 + 16) & 0xFD;
  dword_18138 = v5;
  return 1;
}



// Function: copy_rest @ 0x3630
long long copy_rest(FILE *a1, unsigned int a2)
{
  char *v2; // rax
  unsigned int c; // [rsp+4h] [rbp-1Ch]
  char *i; // [rsp+18h] [rbp-8h]

  c = a2;
  dword_D128 = 0;
  if ( dword_D124 > dword_1813C || a2 != 10 && a2 != -1 )
  {
    sub_42AA((unsigned int)dword_1813C);
    for ( i = qword_D108; dword_D128 != dword_D124 && *i; ++i )
    {
      v2 = i;
      putchar_unlocked(*v2);
      ++dword_D128;
    }
    if ( a2 != -1 && a2 != 10 )
      sub_42AA((unsigned int)(dword_D124 - dword_D128));
    if ( a2 == -1 && dword_D11C + dword_1813C <= dword_D124 )
      putchar_unlocked(10);
  }
  while ( c != 10 && c != -1 )
  {
    putchar_unlocked(c);
    c = getc_unlocked(a1);
  }
  return c;
}



// Function: same_para @ 0x3743
long long same_para(int a1)
{
  return dword_1813C == dword_1812C && dword_D114 + dword_1813C <= dword_D124 && a1 != 10 && a1 != -1;
}



// Function: get_line @ 0x3793
long long get_line(FILE *a1, char a2)
{
  char *v2; // rax
  long long v3; // rax
  bool v4; // al
  int v5; // edx
  unsigned int v7; // [rsp+4h] [rbp-2Ch]
  int v8; // [rsp+1Ch] [rbp-14h]

  LOBYTE(v7) = a2;
  do
  {
    *(long long *)qword_18120 = qword_E4C8;
    do
    {
      if ( &qword_E4C8 == (long long *)qword_E4C8 )
      {
        sub_32FB(1);
        sub_3BD2();
      }
      v2 = (char *)qword_E4C8++;
      *v2 = v7;
      v7 = getc_unlocked(a1);
    }
    while ( v7 != -1 && (unsigned char)sub_4512(v7) != 1 );
    v3 = qword_18120;
    *(int *)(qword_18120 + 8) = qword_E4C8 - *(long long *)qword_18120;
    dword_D124 += *(int *)(v3 + 8);
    sub_3ABA(qword_18120);
    v8 = dword_D124;
    v7 = sub_3A4C(a1, v7);
    *(int *)(qword_18120 + 12) = dword_D124 - v8;
    v4 = v7 == -1 || (*(char *)(qword_18120 + 16) & 2) != 0 && (v7 == 10 || *(int *)(qword_18120 + 12) > 1);
    *(char *)(qword_18120 + 16) = (8 * v4) | *(char *)(qword_18120 + 16) & 0xF7;
    if ( v7 == 10 || v7 == -1 || byte_D103 )
    {
      if ( (*(char *)(qword_18120 + 16) & 8) != 0 )
        v5 = 2;
      else
        v5 = 1;
      *(int *)(qword_18120 + 12) = v5;
    }
    if ( &unk_180D0 == (_UNKNOWN *)qword_18120 )
    {
      sub_32FB(1);
      sub_3BD2();
    }
    qword_18120 += 40;
  }
  while ( v7 != 10 && v7 != -1 );
  return sub_3978(a1);
}



// Function: get_prefix @ 0x3978
long long get_prefix(FILE *a1)
{
  unsigned int v1; // eax
  int v2; // eax
  unsigned int v4; // [rsp+14h] [rbp-Ch]
  char *i; // [rsp+18h] [rbp-8h]

  dword_D124 = 0;
  v1 = getc_unlocked(a1);
  v4 = sub_3A4C(a1, v1);
  if ( dword_D11C )
  {
    dword_1813C = dword_D124;
    for ( i = qword_D108; *i; ++i )
    {
      if ( v4 != (unsigned char)*i )
        return v4;
      ++dword_D124;
      v4 = getc_unlocked(a1);
    }
    return (unsigned int)sub_3A4C(a1, v4);
  }
  else
  {
    v2 = dword_D118;
    if ( dword_D124 <= dword_D118 )
      v2 = dword_D124;
    dword_1813C = v2;
  }
  return v4;
}



// Function: get_space @ 0x3a4c
long long get_space(FILE *a1, unsigned int a2)
{
  while ( 1 )
  {
    if ( a2 == 32 )
    {
      ++dword_D124;
      goto LABEL_6;
    }
    if ( a2 != 9 )
      return a2;
    byte_18128 = 1;
    dword_D124 = 8 * (dword_D124 / 8 + 1);
LABEL_6:
    a2 = getc_unlocked(a1);
  }
}



// Function: check_punctuation @ 0x3aba
char **check_punctuation(unsigned long long *a1)
{
  bool v1; // dl
  unsigned char v3; // [rsp+1Fh] [rbp-11h]
  char *v4; // [rsp+20h] [rbp-10h]
  unsigned long long v5; // [rsp+28h] [rbp-8h]

  v5 = *a1;
  v4 = (char *)(*((int *)a1 + 2) - 1LL + *a1);
  v3 = *v4;
  *((char *)a1 + 16) = (strchr("(['`\"", *(char *)*a1) != 0) | a1[2] & 0xFE;
  *((char *)a1 + 16) = (4 * (((*__ctype_b_loc())[v3] & 4) != 0)) | a1[2] & 0xFB;
  while ( v5 < (unsigned long long)v4 && strchr(")]'\"", *v4) )
    --v4;
  v1 = strchr(".?!", *v4) != 0;
  *((char *)a1 + 16) = (2 * v1) | a1[2] & 0xFD;
  return (char **)a1;
}



// Function: flush_paragraph @ 0x3bd2
char *flush_paragraph()
{
  char *result; // rax
  signed int v1; // [rsp+4h] [rbp-1Ch]
  const void **src; // [rsp+8h] [rbp-18h]
  long long i; // [rsp+10h] [rbp-10h]
  const void **j; // [rsp+10h] [rbp-10h]
  long long v5; // [rsp+18h] [rbp-8h]

  if ( (_UNKNOWN *)qword_18120 == &unk_E4E0 )
  {
    fwrite_unlocked(&unk_D140, 1u, qword_E4C8 - (long long)&unk_D140, stdout);
    qword_E4C8 = (long long)&unk_D140;
    return (char *)&unk_D140;
  }
  else
  {
    sub_3DB9();
    src = (const void **)qword_18120;
    v5 = 0x7FFFFFFFFFFFFFFFLL;
    for ( i = qword_E500; i != qword_18120; i = *(long long *)(i + 32) )
    {
      if ( v5 > *(long long *)(i + 24) - *(long long *)(*(long long *)(i + 32) + 24LL) )
      {
        src = (const void **)i;
        v5 = *(long long *)(i + 24) - *(long long *)(*(long long *)(i + 32) + 24LL);
      }
      if ( v5 <= 0x7FFFFFFFFFFFFFF6LL )
        v5 += 9;
    }
    sub_411D(src);
    memmove(&unk_D140, *src, qword_E4C8 - (long long)*src);
    v1 = (unsigned int)*src - (unsigned int)&unk_D140;
    qword_E4C8 -= v1;
    for ( j = src; (unsigned long long)j <= qword_18120; j += 5 )
      *j = (char *)*j - v1;
    memmove(&unk_E4E0, src, qword_18120 - (long long)src + 40);
    result = (char *)(qword_18120 + &unk_E4E0 - (_UNKNOWN *)src);
    qword_18120 = (long long)result;
  }
  return result;
}



// Function: fmt_paragraph @ 0x3db9
long long fmt_paragraph()
{
  int v0; // eax
  long long result; // rax
  int v2; // [rsp+8h] [rbp-28h]
  int v3; // [rsp+Ch] [rbp-24h]
  unsigned long long i; // [rsp+10h] [rbp-20h]
  unsigned long long v5; // [rsp+18h] [rbp-18h]
  long long v6; // [rsp+20h] [rbp-10h]
  long long v7; // [rsp+28h] [rbp-8h]

  *(long long *)(qword_18120 + 24) = 0;
  v3 = *(int *)(qword_18120 + 8);
  *(int *)(qword_18120 + 8) = dword_D110;
  for ( i = qword_18120 - 40; i >= (unsigned long long)&unk_E4E0; i -= 40LL )
  {
    v7 = 0x7FFFFFFFFFFFFFFFLL;
    if ( (_UNKNOWN *)i == &unk_E4E0 )
      v0 = dword_18130;
    else
      v0 = dword_18134;
    v5 = i;
    v2 = *(int *)(i + 8) + v0;
    do
    {
      v5 += 40LL;
      v6 = sub_406B(v5, (unsigned int)v2) + *(long long *)(v5 + 24);
      if ( (_UNKNOWN *)i == &unk_E4E0 && dword_18140 > 0 )
        v6 += 10 * (v2 - dword_18140) * (long long)(10 * (v2 - dword_18140)) / 2;
      if ( v6 < v7 )
      {
        v7 = v6;
        *(long long *)(i + 32) = v5;
        *(int *)(i + 20) = v2;
      }
      if ( v5 == qword_18120 )
        break;
      v2 += *(int *)(v5 - 40 + 12) + *(int *)(v5 + 8);
    }
    while ( v2 < dword_D110 );
    *(long long *)(i + 24) = sub_3F67(i) + v7;
  }
  result = qword_18120;
  *(int *)(qword_18120 + 8) = v3;
  return result;
}



// Function: base_cost @ 0x3f67
long long base_cost(unsigned long long a1)
{
  long long v2; // [rsp+10h] [rbp-8h]

  v2 = 4900;
  if ( a1 > (unsigned long long)&unk_E4E0 )
  {
    if ( (*(char *)(a1 - 40 + 16) & 2) != 0 )
    {
      if ( (*(char *)(a1 - 40 + 16) & 8) != 0 )
        v2 = 2400;
      else
        v2 = 364900;
    }
    else if ( (*(char *)(a1 - 40 + 16) & 4) != 0 )
    {
      v2 = 3300;
    }
    else if ( a1 > (unsigned long long)&unk_E508 && (*(char *)(a1 - 80 + 16) & 8) != 0 )
    {
      v2 = 40000LL / (*(int *)(a1 - 40 + 8) + 2) + 4900;
    }
  }
  if ( (*(char *)(a1 + 16) & 1) != 0 )
  {
    v2 -= 1600;
  }
  else if ( (*(char *)(a1 + 16) & 8) != 0 )
  {
    v2 += 22500LL / (*(int *)(a1 + 8) + 2);
  }
  return v2;
}



// Function: line_cost @ 0x406b
long long line_cost(long long a1, int a2)
{
  long long v3; // rcx
  long long v4; // [rsp+14h] [rbp-8h]

  if ( a1 == qword_18120 )
    return 0;
  v4 = 10 * (dword_D120 - a2) * (long long)(10 * (dword_D120 - a2));
  if ( *(long long *)(a1 + 32) != qword_18120 )
  {
    v3 = 10 * (a2 - *(int *)(a1 + 20));
    v4 += v3 * v3 / 2;
  }
  return v4;
}



// Function: put_paragraph @ 0x411d
long long put_paragraph(long long a1)
{
  long long result; // rax
  long long i; // [rsp+18h] [rbp-8h]

  sub_417F(&unk_E4E0, (unsigned int)dword_18130);
  for ( i = qword_E500; ; i = *(long long *)(i + 32) )
  {
    result = i;
    if ( i == a1 )
      break;
    sub_417F(i, (unsigned int)dword_18134);
  }
  return result;
}



// Function: put_line @ 0x417f
int put_line(long long a1, int a2)
{
  long long v3; // [rsp+8h] [rbp-18h]
  long long v4; // [rsp+18h] [rbp-8h]

  v3 = a1;
  dword_D128 = 0;
  sub_42AA((unsigned int)dword_1812C);
  fputs_unlocked(qword_D108, stdout);
  dword_D128 += dword_D11C;
  sub_42AA((unsigned int)(a2 - dword_D128));
  v4 = *(long long *)(a1 + 32) - 40LL;
  while ( v3 != v4 )
  {
    sub_4248(v3);
    sub_42AA(*(unsigned int *)(v3 + 12));
    v3 += 40;
  }
  sub_4248(v3);
  dword_18140 = dword_D128;
  return putchar_unlocked(10);
}



// Function: put_word @ 0x4248
long long put_word(char **a1)
{
  char *v1; // rax
  int v2; // edx
  long long result; // rax
  int i; // [rsp+14h] [rbp-Ch]
  char *v5; // [rsp+18h] [rbp-8h]

  v5 = *a1;
  for ( i = *((int *)a1 + 2); i; --i )
  {
    v1 = v5++;
    putchar_unlocked(*v1);
  }
  v2 = *((int *)a1 + 2);
  result = (unsigned int)(v2 + dword_D128);
  dword_D128 += v2;
  return result;
}



// Function: put_space @ 0x42aa
long long put_space(int a1)
{
  int v1; // eax
  long long result; // rax
  int v3; // [rsp+18h] [rbp-8h]
  int v4; // [rsp+1Ch] [rbp-4h]

  v3 = dword_D128 + a1;
  if ( byte_18128 )
  {
    v1 = dword_D128 + a1;
    if ( v3 < 0 )
      v1 = v3 + 7;
    v4 = 8 * (v1 >> 3);
    if ( v4 > dword_D128 + 1 )
    {
      while ( v4 > dword_D128 )
      {
        putchar_unlocked(9);
        dword_D128 = 8 * (dword_D128 / 8 + 1);
      }
    }
  }
  while ( 1 )
  {
    result = (unsigned int)dword_D128;
    if ( v3 <= dword_D128 )
      break;
    putchar_unlocked(32);
    ++dword_D128;
  }
  return result;
}


