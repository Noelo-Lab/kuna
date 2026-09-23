// Function: free_diff_block @ 0x2b49
void free_diff_block()
{
  ;
}



// Function: next_to_n2 @ 0x2b58
void next_to_n2()
{
  ;
}



// Function: main @ 0x2b67
long long main(int a1, char **a2, char **a3)
{
  int v3; // eax
  int v5; // eax
  unsigned char v6; // [rsp+13h] [rbp-12Dh]
  int i; // [rsp+14h] [rbp-12Ch]
  int j; // [rsp+14h] [rbp-12Ch]
  int k; // [rsp+14h] [rbp-12Ch]
  int v10; // [rsp+18h] [rbp-128h]
  int v11; // [rsp+1Ch] [rbp-124h]
  int v12; // [rsp+20h] [rbp-120h]
  int v13; // [rsp+24h] [rbp-11Ch]
  char v14[8]; // [rsp+28h] [rbp-118h] BYREF
  void *ptr; // [rsp+30h] [rbp-110h] BYREF
  void *v16; // [rsp+38h] [rbp-108h] BYREF
  const char **v17; // [rsp+40h] [rbp-100h]
  const char *v18; // [rsp+48h] [rbp-F8h]
  long long v19; // [rsp+50h] [rbp-F0h]
  long long v20; // [rsp+58h] [rbp-E8h]
  long long v21; // [rsp+60h] [rbp-E0h]
  int v22[3]; // [rsp+68h] [rbp-D8h] BYREF
  int v23[3]; // [rsp+74h] [rbp-CCh] BYREF
  long long v24; // [rsp+80h] [rbp-C0h]
  long long v25; // [rsp+88h] [rbp-B8h]
  long long v26; // [rsp+90h] [rbp-B0h]
  struct stat buf; // [rsp+A0h] [rbp-A0h] BYREF
  unsigned long long v28; // [rsp+138h] [rbp-8h]

  v28 = __readfsqword(0x28u);
  v11 = 0;
  v12 = 0;
  status = 2;
  sub_637F(*a2, a2, a3);
  setlocale(6, &locale);
  bindtextdomain("diffutils", "/usr/local/share/locale");
  textdomain("diffutils");
  sub_62BB(0);
  sub_9533();
  while ( 1 )
  {
    v13 = getopt_long(a1, a2, "aeimvx3AEL:TX", &longopts, 0);
    if ( v13 == -1 )
      break;
    switch ( v13 )
    {
      case 51:
        byte_1410E = 1;
        v11 |= 1u;
        continue;
      case 65:
        byte_14110 = 1;
        byte_1410C = 1;
        v11 |= 2u;
        continue;
      case 69:
        byte_1410C = 1;
        v11 |= 4u;
        continue;
      case 76:
        if ( v12 > 2 )
        {
          sub_3489("too many file label options", 0);
LABEL_20:
          sub_3489(0, 0);
        }
        else
        {
          v5 = v12++;
          *(&v24 + v5) = optarg;
        }
        break;
      case 84:
        byte_1410D = 1;
        continue;
      case 88:
        byte_1410F = 1;
        v11 |= 8u;
        continue;
      case 97:
        byte_14109 = 1;
        continue;
      case 101:
        v11 |= 0x10u;
        continue;
      case 105:
        byte_14111 = 1;
        continue;
      case 109:
        byte_14112 = 1;
        continue;
      case 118:
        v3 = sub_6940("Randy Smith");
        sub_8BE7(
          (int)stdout,
          (unsigned int)"diff3",
          (unsigned int)"GNU diffutils",
          (int)off_14028,
          v3,
          0,
          (char)a2);
        sub_34FF();
        return 0;
      case 120:
        byte_1410F = 1;
        v11 |= 0x20u;
        continue;
      case 128:
        file = (char *)optarg;
        continue;
      case 129:
        sub_3556();
        sub_34FF();
        return 0;
      case 130:
        byte_1410A = 1;
        continue;
      default:
        goto LABEL_20;
    }
  }
  byte_1410B = ((v11 != 0) & ((unsigned char)byte_14112 ^ 1)) != 0;
  byte_14110 = ((unsigned char)byte_14110 | (v11 == 0) & (unsigned char)byte_14112) != 0;
  byte_1410C = ((unsigned char)byte_1410C | (v11 == 0) & (unsigned char)byte_14112) != 0;
  if ( (v11 & (v11 - 1)) != 0
    || ((unsigned char)byte_14111 & (unsigned char)byte_14112) != 0
    || v12 && byte_1410C != 1 )
  {
    sub_3489("incompatible options", 0);
  }
  if ( a1 - optind != 3 )
  {
    if ( a1 - optind <= 2 )
      sub_3489("missing operand after '%s'", a2[a1 - 1]);
    sub_3489("extra operand '%s'", a2[optind + 3]);
  }
  v17 = (const char **)&a2[optind];
  for ( i = v12; i <= 2; ++i )
    *(&v24 + i) = (long long)v17[i];
  v10 = 2 - (unsigned char)(byte_1410B | byte_14112);
  if ( !strcmp(v17[v10], "-") )
  {
    v10 = 3 - v10;
    if ( !strcmp(*v17, "-") || !strcmp(v17[v10], "-") )
      sub_5FCF("'-' specified for more than one input file");
  }
  v22[0] = 0;
  v22[1] = 3 - v10;
  v22[2] = v10;
  for ( j = 0; j <= 2; ++j )
    v23[v22[j]] = j;
  for ( k = 0; k <= 2; ++k )
  {
    if ( strcmp(v17[k], "-") )
    {
      if ( stat(v17[k], &buf) < 0 )
        sub_600C(v17[k]);
      if ( (buf.st_mode & 0xF000) == 0x4000 )
        error(2, 21, "%s", v17[k]);
    }
  }
  signal(17, 0);
  v18 = v17[v23[2]];
  v19 = sub_43E5(v17[v23[1]], v18, v14, &v16);
  v20 = sub_43E5(v17[v23[0]], v18, v14, &ptr);
  sub_2B58();
  sub_2B58();
  v21 = sub_3696(v20, v19);
  sub_2B49();
  sub_2B49();
  if ( byte_1410B )
  {
    v6 = sub_54DE((int)stdout, v21, (unsigned int)v22, (unsigned int)v23, v24, v25, v26);
  }
  else if ( byte_14112 )
  {
    sub_9414(v17[v23[0]], "r", stdin);
    v6 = sub_5A29((int)stdin, (int)stdout, v21, (unsigned int)v22, (unsigned int)v23, v24, v25, v26);
    if ( ferror_unlocked(stdin) )
      sub_5FCF("read failed");
  }
  else
  {
    sub_5012(stdout, v21, v22, v23);
    v6 = 0;
  }
  free(ptr);
  free(v16);
  sub_34FF();
  exit(v6);
}



// Function: try_help @ 0x3489
long long try_help(const char *a1, long long a2)
{
  char *v2; // rax
  long long v3; // rbx
  char *v4; // rax

  if ( a1 )
  {
    v2 = gettext(a1);
    error(0, 0, v2, a2);
  }
  v3 = qword_24148;
  v4 = gettext("Try '%s --help' for more information.");
  error(2, 0, v4, v3);
  return sub_34FF();
}



// Function: check_stdout @ 0x34ff
int check_stdout()
{
  int result; // eax
  char *v1; // rax

  if ( ferror_unlocked(stdout) )
    sub_5FCF("write failed");
  result = fclose(stdout);
  if ( result )
  {
    v1 = gettext("standard output");
    sub_600C(v1);
  }
  return result;
}



// Function: usage @ 0x3556
long long usage()
{
  long long v0; // rbx
  char *v1; // rax
  char *v2; // rax
  FILE *v3; // rbx
  char *v4; // rax
  char *v5; // rax
  FILE *v6; // rbx
  char *v7; // rax
  char *v8; // rbx
  char *v9; // rax
  const char **i; // [rsp+8h] [rbp-18h]

  v0 = qword_24148;
  v1 = gettext("Usage: %s [OPTION]... MYFILE OLDFILE YOURFILE\n");
  printf(v1, v0);
  v2 = gettext("Compare three files line by line.");
  printf("%s\n\n", v2);
  v3 = stdout;
  v4 = gettext("Mandatory arguments to long options are mandatory for short options too.\n");
  fputs_unlocked(v4, v3);
  for ( i = (const char **)&off_13A40; *i; ++i )
  {
    if ( **i )
    {
      v5 = gettext(*i);
      printf("  %s\n", v5);
    }
    else
    {
      putchar_unlocked(10);
    }
  }
  v6 = stdout;
  v7 = gettext(
         "\n"
         "The default output format is a somewhat human-readable representation of\n"
         "the changes.\n"
         "\n"
         "The -e, -E, -x, -X (and corresponding long) options cause an ed script\n"
         "to be output instead of the default.\n"
         "\n"
         "Finally, the -m (--merge) option causes diff3 to do the merge internally\n"
         "and output the actual merged file.  For unusual input, this is more\n"
         "robust than using ed.\n");
  fputs_unlocked(v7, v6);
  v8 = gettext("Exit status is 0 if successful, 1 if conflicts, 2 if trouble.");
  v9 = gettext("If a FILE is '-', read standard input.");
  printf("\n%s\n%s\n", v9, v8);
  return sub_8CCB();
}



// Function: make_3way_diff @ 0x3696
long long make_3way_diff(long long a1, long long a2, long long a3, long long a4, long long a5, long long a6)
{
  long long v7; // [rsp+0h] [rbp-A0h]
  long long v8; // [rsp+8h] [rbp-98h]
  unsigned int v9; // [rsp+1Ch] [rbp-84h]
  int v10; // [rsp+20h] [rbp-80h]
  int v11; // [rsp+24h] [rbp-7Ch]
  long long v12; // [rsp+28h] [rbp-78h] BYREF
  long long v13; // [rsp+30h] [rbp-70h]
  long long *i; // [rsp+38h] [rbp-68h]
  long long *v15; // [rsp+40h] [rbp-60h]
  void *v16; // [rsp+48h] [rbp-58h]
  long long v17; // [rsp+50h] [rbp-50h]
  long long v18; // [rsp+58h] [rbp-48h]
  long long v19[2]; // [rsp+60h] [rbp-40h] BYREF
  long long v20[2]; // [rsp+70h] [rbp-30h] BYREF
  long long v21; // [rsp+80h] [rbp-20h]
  long long v22; // [rsp+88h] [rbp-18h]
  unsigned long long v23; // [rsp+98h] [rbp-8h]

  v8 = a1;
  v7 = a2;
  v23 = __readfsqword(0x28u);
  v12 = 0;
  v15 = &v12;
  v21 = a1;
  v22 = a2;
  v16 = &unk_FD80;
  while ( v21 || v22 )
  {
    v20[1] = 0;
    v20[0] = 0;
    v19[1] = 0;
    v19[0] = 0;
    if ( v21 )
    {
      if ( v22 )
        v10 = *(long long *)(v21 + 16) > *(long long *)(v22 + 16);
      else
        v10 = 0;
    }
    else
    {
      v10 = 1;
    }
    v9 = v10;
    v17 = *(&v21 + v10);
    v13 = *(long long *)(v17 + 24);
    v20[v10] = v17;
    v19[v10] = v20[v10];
    *(&v21 + v10) = *(long long *)(v17 + 64);
    *(long long *)(v20[v10] + 64LL) = 0;
    v11 = !v10;
    for ( i = (long long *)*(&v21 + !v10); i && i[2] <= v13 + 1; i = (long long *)*(&v21 + (int)(v9 ^ 1)) )
    {
      if ( v19[v11] )
        *(long long *)(v20[v11] + 64LL) = i;
      else
        v19[v11] = i;
      v20[v11] = i;
      *(&v21 + v11) = *(long long *)(*(&v21 + v11) + 64);
      i[8] = 0;
      if ( v13 < i[3] )
      {
        v9 ^= 1u;
        v13 = i[3];
      }
      v11 = v9 ^ 1;
    }
    v18 = sub_393D(v19, v20, v10, v9, v16, a6, v7, v8);
    if ( !v18 )
      sub_5FCF("internal error: screwup in format of diff blocks");
    *v15 = v18;
    v15 = (long long *)(v18 + 104);
    v16 = (void *)v18;
  }
  return v12;
}



// Function: using_to_diff3_block @ 0x393d
long long using_to_diff3_block(long long *a1, long long a2, int a3, int a4, long long a5)
{
  long long v5; // rdx
  long long v7; // rax
  long long v8; // rax
  int i; // [rsp+24h] [rbp-9Ch]
  long long v10; // [rsp+24h] [rbp-9Ch]
  char v11[12]; // [rsp+24h] [rbp-9Ch]
  long long *j; // [rsp+28h] [rbp-98h]
  long long k; // [rsp+30h] [rbp-90h]
  long long m; // [rsp+30h] [rbp-90h]
  long long v15; // [rsp+38h] [rbp-88h]
  long long v16; // [rsp+40h] [rbp-80h]
  long long v17; // [rsp+48h] [rbp-78h]
  long long v18; // [rsp+50h] [rbp-70h]
  long long v19; // [rsp+58h] [rbp-68h]
  long long *v20; // [rsp+68h] [rbp-58h]
  long long v21; // [rsp+70h] [rbp-50h]
  long long v22; // [rsp+78h] [rbp-48h]
  long long v23[2]; // [rsp+90h] [rbp-30h]
  long long v24[4]; // [rsp+A0h] [rbp-20h]

  v24[3] = __readfsqword(0x28u);
  v16 = *(long long *)(a1[a3] + 16LL);
  v17 = *(long long *)(*(long long *)(8LL * a4 + a2) + 24LL);
  for ( i = 0; i <= 1; ++i )
  {
    if ( a1[i] )
    {
      v23[i] = v16 - *(long long *)(a1[i] + 16LL) + *(long long *)a1[i];
      v5 = v17 - *(long long *)(*(long long *)(8LL * i + a2) + 24LL) + *(long long *)(*(long long *)(8LL * i + a2) + 8LL);
    }
    else
    {
      v23[i] = v16 - *(long long *)(a5 + 48) + *(long long *)(a5 + 16LL * i + 16);
      v5 = v17 - *(long long *)(a5 + 48) + *(long long *)(a5 + 16LL * i + 16);
    }
    v24[i] = v5;
  }
  v18 = sub_4116(v23[0], v24[0], v23[1], v24[1], v16, v17);
  LODWORD(v10) = 0;
  while ( (int)v10 <= 1 )
  {
    for ( j = (long long *)a1[(int)v10]; j; j = (long long *)j[8] )
    {
      if ( (unsigned char)sub_4067(
                              j[5],
                              j[7],
                              8 * (j[2] - v16) + *(long long *)(v18 + 72),
                              *(long long *)(v18 + 96) + 8 * (j[2] - v16),
                              j[3] - j[2] + 1LL) != 1 )
        return 0;
    }
    v10 = (unsigned int)(v10 + 1);
  }
  *(int *)v11 = 0;
  while ( *(int *)v11 <= 1 )
  {
    v20 = (long long *)a1[*(int *)v11];
    v21 = v23[*(int *)v11];
    v22 = v24[*(int *)v11];
    for ( k = 0; ; ++k )
    {
      v7 = v20 ? *v20 : v22 + 1;
      if ( v7 <= v21 + k )
        break;
      *(long long *)(8 * k + *(long long *)(v18 + 8 * (*(int *)v11 + 6LL) + 8)) = *(long long *)(*(long long *)(v18 + 72) + 8 * k);
      *(long long *)(8 * k + *(long long *)(v18 + 8 * (*(int *)v11 + 10LL))) = *(long long *)(*(long long *)(v18 + 96) + 8 * k);
    }
    for ( *(long long *)&v11[4] = v20; *(long long *)&v11[4]; *(long long *)&v11[4] = *(long long *)(*(long long *)&v11[4] + 64LL) )
    {
      if ( (unsigned char)sub_4067(
                              *(long long *)(*(long long *)&v11[4] + 32LL),
                              *(long long *)(*(long long *)&v11[4] + 48LL),
                              8 * (**(long long **)&v11[4] - v21) + *(long long *)(v18 + 8 * (*(int *)v11 + 6LL) + 8),
                              *(long long *)(v18 + 8 * (*(int *)v11 + 10LL)) + 8 * (**(long long **)&v11[4] - v21),
                              *(long long *)(*(long long *)&v11[4] + 8LL) - **(long long **)&v11[4] + 1LL) != 1 )
        return 0;
      v15 = *(long long *)(*(long long *)&v11[4] + 24LL) + 1LL - v16;
      for ( m = *(long long *)(*(long long *)&v11[4] + 8LL) + 1LL - v21; ; ++m )
      {
        v8 = *(long long *)(*(long long *)&v11[4] + 64LL) ? **(long long **)(*(long long *)&v11[4] + 64LL) : v22 + 1;
        if ( m >= v8 - v21 )
          break;
        *(long long *)(8 * m + *(long long *)(v18 + 8 * (*(int *)v11 + 6LL) + 8)) = *(long long *)(*(long long *)(v18 + 72) + 8 * v15);
        *(long long *)(8 * m + *(long long *)(v18 + 8 * (*(int *)v11 + 10LL))) = *(long long *)(*(long long *)(v18 + 96) + 8 * v15++);
      }
    }
    *(long long *)v11 = (unsigned int)(*(int *)v11 + 1);
  }
  if ( *a1 )
  {
    if ( a1[1] )
    {
      v19 = *(long long *)(v18 + 16) - *(long long *)(v18 + 8) + 1LL;
      if ( v19 == *(long long *)(v18 + 32) - *(long long *)(v18 + 24) + 1LL
        && (unsigned char)sub_4311(
                              *(long long *)(v18 + 56),
                              *(long long *)(v18 + 80),
                              *(long long *)(v18 + 64),
                              *(long long *)(v18 + 88),
                              v19) == 1 )
      {
        *(int *)v18 = 7;
      }
      else
      {
        *(int *)v18 = 4;
      }
    }
    else
    {
      *(int *)v18 = 5;
    }
  }
  else
  {
    *(int *)v18 = 6;
  }
  return v18;
}



// Function: copy_stringlist @ 0x4067
long long copy_stringlist(const void **a1, size_t *a2, const void **a3, size_t *a4, long long a5)
{
  while ( a5-- )
  {
    if ( *a3 )
    {
      if ( *a2 != *a4 || memcmp(*a1, *a3, *a2) )
        return 0;
    }
    else
    {
      *a3 = *a1;
      *a4 = *a2;
    }
    ++a3;
    ++a1;
    ++a4;
    ++a2;
  }
  return 1;
}



// Function: create_diff3_block @ 0x4116
long long create_diff3_block(long long a1, long long a2, long long a3, long long a4, long long a5, long long a6)
{
  long long v6; // rcx
  long long v7; // rcx
  long long v8; // rcx
  long long v14; // [rsp+30h] [rbp-10h]
  long long v15; // [rsp+38h] [rbp-8h]
  long long v16; // [rsp+38h] [rbp-8h]
  long long v17; // [rsp+38h] [rbp-8h]

  v14 = sub_8E10(112);
  *(int *)v14 = 0;
  *(long long *)(v14 + 104) = 0;
  *(long long *)(v14 + 8) = a1;
  *(long long *)(v14 + 16) = a2;
  *(long long *)(v14 + 24) = a3;
  *(long long *)(v14 + 32) = a4;
  *(long long *)(v14 + 40) = a5;
  *(long long *)(v14 + 48) = a6;
  v6 = *(long long *)(v14 + 8);
  v15 = *(long long *)(v14 + 16) - v6 + 1;
  if ( *(long long *)(v14 + 16) - v6 == -1 )
  {
    *(long long *)(v14 + 56) = 0;
    *(long long *)(v14 + 80) = 0;
  }
  else
  {
    *(long long *)(v14 + 56) = sub_927A(v15, 8);
    *(long long *)(v14 + 80) = sub_927A(v15, 8);
  }
  v7 = *(long long *)(v14 + 24);
  v16 = *(long long *)(v14 + 32) - v7 + 1;
  if ( *(long long *)(v14 + 32) - v7 == -1 )
  {
    *(long long *)(v14 + 64) = 0;
    *(long long *)(v14 + 88) = 0;
  }
  else
  {
    *(long long *)(v14 + 64) = sub_927A(v16, 8);
    *(long long *)(v14 + 88) = sub_927A(v16, 8);
  }
  v8 = *(long long *)(v14 + 40);
  v17 = *(long long *)(v14 + 48) - v8 + 1;
  if ( *(long long *)(v14 + 48) - v8 == -1 )
  {
    *(long long *)(v14 + 72) = 0;
    *(long long *)(v14 + 96) = 0;
  }
  else
  {
    *(long long *)(v14 + 72) = sub_927A(v17, 8);
    *(long long *)(v14 + 96) = sub_927A(v17, 8);
  }
  return v14;
}



// Function: compare_line_list @ 0x4311
long long compare_line_list(const void **a1, size_t *a2, const void **a3, long long *a4, long long a5)
{
  long long *v5; // rax
  size_t *v6; // rax
  size_t v7; // rdx
  const void **v8; // rax
  const void *v9; // rcx
  const void **v10; // rax

  while ( a5-- )
  {
    if ( *a1 )
    {
      if ( *a3 )
      {
        v5 = a4++;
        if ( *a2 == *v5 )
        {
          v6 = a2++;
          v7 = *v6;
          v8 = a3++;
          v9 = *v8;
          v10 = a1++;
          if ( !memcmp(*v10, v9, v7) )
            continue;
        }
      }
    }
    return 0;
  }
  return 1;
}



// Function: process_diff @ 0x43e5
long long process_diff(long long a1, long long a2, long long *a3, char **a4)
{
  long long v4; // rbx
  char *v5; // rax
  char *v6; // rax
  unsigned int v10; // [rsp+24h] [rbp-6Ch]
  char *v11; // [rsp+28h] [rbp-68h] BYREF
  char *s1; // [rsp+30h] [rbp-60h] BYREF
  long long v13; // [rsp+38h] [rbp-58h] BYREF
  long long i; // [rsp+40h] [rbp-50h]
  long long *v15; // [rsp+48h] [rbp-48h]
  long long *v16; // [rsp+50h] [rbp-40h]
  unsigned long long v17; // [rsp+58h] [rbp-38h]
  unsigned long long v18; // [rsp+60h] [rbp-30h]
  long long v19; // [rsp+68h] [rbp-28h]
  long long v20; // [rsp+70h] [rbp-20h]
  unsigned long long v21; // [rsp+78h] [rbp-18h]

  v21 = __readfsqword(0x28u);
  v15 = &v13;
  v17 = 0xFFFFFFFFFFFFFFFLL;
  v18 = sub_4A3A(a1, a2, &v11);
  *a4 = v11;
  s1 = v11;
  while ( v18 > (unsigned long long)s1 )
  {
    v16 = (long long *)sub_8E10(72);
    v16[5] = 0;
    v16[4] = v16[5];
    v16[7] = 0;
    v16[6] = v16[7];
    v10 = sub_4887(&s1, v16);
    if ( !v10 || *s1 != 10 )
    {
      v4 = qword_24148;
      v5 = gettext("%s: diff failed: ");
      fprintf(stderr, v5, v4);
      do
      {
        putc_unlocked(*s1, stderr);
        v6 = s1++;
      }
      while ( *v6 != 10 );
      exit(2);
    }
    ++s1;
    if ( v10 == 3 )
    {
      ++v16[2];
    }
    else
    {
      if ( v10 > 3 )
        sub_5FCF("internal error: invalid diff type in process_diff");
      if ( v10 == 1 )
        ++*v16;
    }
    if ( v10 != 1 )
    {
      v19 = v16[1] - *v16 + 1LL;
      if ( v17 <= v19 )
        sub_93D4();
      v16[4] = sub_8E10(8 * v19);
      v16[6] = sub_8E10(8 * v19);
      for ( i = 0; i < v19; ++i )
        s1 = (char *)sub_4EDC(s1, v16[4] + 8 * i, v16[6] + 8 * i, v18, 60);
    }
    if ( v10 == 2 )
    {
      if ( strncmp(s1, "---\n", 4u) )
        sub_5FCF("invalid diff format; invalid change separator");
      s1 += 4;
    }
    if ( v10 != 3 )
    {
      v20 = v16[3] - v16[2] + 1LL;
      if ( v17 <= v20 )
        sub_93D4();
      v16[5] = sub_8E10(8 * v20);
      v16[7] = sub_8E10(8 * v20);
      for ( i = 0; i < v20; ++i )
        s1 = (char *)sub_4EDC(s1, v16[5] + 8 * i, v16[7] + 8 * i, v18, 62);
    }
    *v15 = (long long)v16;
    v15 = v16 + 8;
  }
  *v15 = 0;
  *a3 = v16;
  return v13;
}



// Function: skipwhite @ 0x47d5
char *skipwhite(char *a1)
{
  while ( *a1 == 32 || *a1 == 9 )
    ++a1;
  return a1;
}



// Function: readnum @ 0x4804
unsigned char *readnum(unsigned char *a1, long long *a2)
{
  unsigned char *v3; // [rsp+8h] [rbp-18h]
  unsigned char v4; // [rsp+17h] [rbp-9h]
  long long v5; // [rsp+18h] [rbp-8h]

  v3 = a1;
  v4 = *a1;
  v5 = 0;
  if ( (unsigned int)*a1 - 48 > 9 )
    return 0;
  do
  {
    v5 = v4 - 48 + 10 * v5;
    v4 = *++v3;
  }
  while ( (unsigned int)*v3 - 48 <= 9 );
  *a2 = v5;
  return v3;
}



// Function: process_diff_control @ 0x4887
long long process_diff_control(char **a1, long long *a2)
{
  unsigned char *v2; // rax
  int v4; // eax
  unsigned char *v5; // rax
  unsigned int v6; // [rsp+14h] [rbp-14h]
  unsigned char *v7; // [rsp+18h] [rbp-10h]
  char *v8; // [rsp+18h] [rbp-10h]
  char *v9; // [rsp+18h] [rbp-10h]
  unsigned char *v10; // [rsp+18h] [rbp-10h]
  char *v11; // [rsp+18h] [rbp-10h]
  unsigned char *v12; // [rsp+18h] [rbp-10h]

  v2 = sub_47D5(*a1);
  v7 = sub_4804(v2, a2);
  if ( !v7 )
    return 0;
  v8 = sub_47D5(v7);
  if ( *v8 == 44 )
  {
    v8 = sub_4804(v8 + 1, a2 + 1);
    if ( !v8 )
      return 0;
  }
  else
  {
    a2[1] = *a2;
  }
  v9 = sub_47D5(v8);
  v4 = (char)*v9;
  if ( v4 == 100 )
  {
    v6 = 3;
  }
  else
  {
    if ( v4 > 100 )
      return 0;
    if ( v4 == 97 )
    {
      v6 = 1;
    }
    else
    {
      if ( v4 != 99 )
        return 0;
      v6 = 2;
    }
  }
  v5 = sub_47D5(v9 + 1);
  v10 = sub_4804(v5, a2 + 2);
  if ( !v10 )
    return 0;
  v11 = sub_47D5(v10);
  if ( *v11 == 44 )
  {
    v12 = sub_4804(v11 + 1, a2 + 3);
    if ( !v12 )
      return 0;
    v11 = sub_47D5(v12);
  }
  else
  {
    a2[3] = a2[2];
  }
  *a1 = v11;
  return v6;
}



// Function: read_diff @ 0x4a3a
long long read_diff(long long a1, long long a2, long long *a3)
{
  long long *v3; // rax
  long long *v4; // rax
  long long *v5; // rax
  long long *v6; // rax
  long long *v7; // rax
  long long *v8; // rax
  int v9; // eax
  __blksize_t st_blksize; // rax
  char *v11; // rax
  int v12; // eax
  char *v13; // rbx
  const char *v14; // rax
  char *v15; // rax
  int stat_loc; // [rsp+24h] [rbp-14Ch] BYREF
  int errnum; // [rsp+28h] [rbp-148h]
  __pid_t pid; // [rsp+2Ch] [rbp-144h]
  int fd; // [rsp+30h] [rbp-140h]
  unsigned int v22; // [rsp+34h] [rbp-13Ch]
  long long v23; // [rsp+38h] [rbp-138h]
  unsigned long long v24; // [rsp+40h] [rbp-130h]
  long long v25; // [rsp+48h] [rbp-128h]
  long long *v26; // [rsp+50h] [rbp-120h]
  long long v27; // [rsp+58h] [rbp-118h]
  long long v28; // [rsp+60h] [rbp-110h]
  int pipedes[2]; // [rsp+68h] [rbp-108h] BYREF
  char *argv; // [rsp+70h] [rbp-100h] BYREF
  long long v31; // [rsp+78h] [rbp-F8h] BYREF
  struct stat buf; // [rsp+C0h] [rbp-B0h] BYREF
  unsigned long long v33; // [rsp+158h] [rbp-18h]

  v33 = __readfsqword(0x28u);
  errnum = 0;
  v26 = &v31;
  argv = file;
  if ( byte_14109 )
  {
    v3 = v26++;
    *v3 = (long long)"-a";
  }
  if ( byte_1410A )
  {
    v4 = v26++;
    *v4 = (long long)"--strip-trailing-cr";
  }
  v5 = v26++;
  *v5 = (long long)"--horizon-lines=100";
  v6 = v26++;
  *v6 = (long long)"--";
  v7 = v26++;
  *v7 = a1;
  v8 = v26++;
  *v8 = a2;
  *v26 = 0;
  if ( pipe(pipedes) )
    sub_600C("pipe");
  pid = fork();
  if ( !pid )
  {
    close(pipedes[0]);
    if ( pipedes[1] != 1 )
    {
      dup2(pipedes[1], 1);
      close(pipedes[1]);
    }
    execvp(file, &argv);
    if ( *__errno_location() == 2 )
      v9 = 127;
    else
      v9 = 126;
    _exit(v9);
  }
  if ( pid == -1 )
    sub_600C("fork");
  close(pipedes[1]);
  fd = pipedes[0];
  if ( fstat(pipedes[0], &buf) )
    sub_600C("fstat");
  st_blksize = buf.st_blksize;
  if ( buf.st_blksize <= 0 )
    st_blksize = 1;
  v24 = st_blksize;
  v23 = sub_8E10(st_blksize);
  v25 = 0;
  while ( 1 )
  {
    v27 = v24 - v25;
    v28 = sub_9671((unsigned int)fd, v23 + v25, v24 - v25);
    v25 += v28;
    if ( v28 != v27 )
      break;
    if ( v24 > 0x3FFFFFFFFFFFFFFELL )
      sub_93D4();
    v24 *= 2LL;
    v23 = sub_8E5C(v23, v24);
  }
  if ( v28 == -1 )
  {
    v11 = gettext("read failed");
    sub_600C(v11);
  }
  if ( v25 && *(char *)(v25 - 1 + v23) != 10 )
    sub_5FCF("invalid diff format; incomplete last line");
  *a3 = v23;
  if ( close(fd) )
    sub_600C("close");
  if ( waitpid(pid, &stat_loc, 0) < 0 )
    sub_600C("waitpid");
  if ( errnum || (stat_loc & 0x7F) != 0 )
    v12 = 0x7FFFFFFF;
  else
    v12 = BYTE1(stat_loc);
  v22 = v12;
  if ( v12 > 1 )
  {
    v13 = file;
    switch ( v22 )
    {
      case 0x7Eu:
        v14 = "subsidiary program '%s' could not be invoked";
        break;
      case 0x7Fu:
        v14 = "subsidiary program '%s' not found";
        break;
      case 0x7FFFFFFFu:
        v14 = "subsidiary program '%s' failed";
        break;
      default:
        v14 = "subsidiary program '%s' failed (exit status %d)";
        break;
    }
    v15 = gettext(v14);
    error(2, errnum, v15, v13, v22);
  }
  return v23 + v25;
}



// Function: scan_diff_line @ 0x4edc
char *scan_diff_line(char *a1, long long *a2, long long *a3, unsigned long long a4, char a5)
{
  char *v5; // rax
  char *v6; // rax
  char *v8; // [rsp+38h] [rbp-8h]

  if ( a5 != *a1 || a1[1] != 32 )
    sub_5FCF("invalid diff format; incorrect leading line chars");
  v8 = a1 + 2;
  *a2 = a1 + 2;
  do
    v5 = v8++;
  while ( *v5 != 10 );
  *a3 = &v8[-*a2];
  if ( (unsigned long long)v8 < a4 && *v8 == 92 )
  {
    if ( byte_1410B )
      fprintf(stderr, "%s:", (const char *)qword_24148);
    else
      --*a3;
    ++v8;
    do
    {
      if ( byte_1410B )
        putc_unlocked((char)*v8, stderr);
      v6 = v8++;
    }
    while ( *v6 != 10 );
  }
  return v8;
}



// Function: output_diff3 @ 0x5012
unsigned long long output_diff3(FILE *a1, long long a2, long long a3, long long a4)
{
  const char *v4; // rax
  unsigned int v5; // eax
  char *v6; // rax
  int v7; // eax
  int i; // [rsp+28h] [rbp-68h]
  int v12; // [rsp+2Ch] [rbp-64h]
  int v13; // [rsp+30h] [rbp-60h]
  int v14; // [rsp+34h] [rbp-5Ch]
  long long v16; // [rsp+40h] [rbp-50h]
  char *s; // [rsp+48h] [rbp-48h]
  long long v18; // [rsp+50h] [rbp-40h]
  long long v19; // [rsp+58h] [rbp-38h]
  void *ptr; // [rsp+70h] [rbp-20h]
  size_t n; // [rsp+78h] [rbp-18h]
  char v22[2]; // [rsp+86h] [rbp-Ah] BYREF
  unsigned long long v23; // [rsp+88h] [rbp-8h]

  v23 = __readfsqword(0x28u);
  if ( byte_1410D )
    v4 = "\t";
  else
    v4 = (const char *)&off_FC70;
  s = (char *)v4;
  while ( a2 )
  {
    v5 = *(int *)a2;
    if ( *(int *)a2 == 4 )
    {
      v22[0] = 0;
      v13 = 3;
      v12 = 3;
    }
    else
    {
      if ( v5 < 4 || v5 - 5 > 2 )
        sub_5FCF("internal error: invalid diff type passed to output");
      v12 = *(int *)(4LL * (unsigned int)(*(int *)a2 - 5) + a4);
      v22[0] = v12 + 49;
      v22[1] = 0;
      v13 = v12 == 0;
    }
    fprintf(a1, "====%s\n", v22);
    for ( i = 0; i <= 2; i = v7 )
    {
      v14 = *(int *)(4LL * i + a3);
      v18 = *(long long *)(a2 + 16LL * v14 + 8);
      v19 = *(long long *)(a2 + 16LL * v14 + 16);
      fprintf(a1, "%d:", i + 1);
      if ( v18 == v19 )
      {
        fprintf(a1, "%ldc\n", v18);
      }
      else if ( v18 - v19 == 1 )
      {
        fprintf(a1, "%lda\n", v18 - 1);
      }
      else
      {
        fprintf(a1, "%ld,%ldc\n", v18, v19);
      }
      if ( i != v13 && v18 <= v19 )
      {
        v16 = 0;
        do
        {
          fputs_unlocked(s, a1);
          ptr = *(void **)(*(long long *)(a2 + 8 * (v14 + 6LL) + 8) + 8 * v16);
          n = *(long long *)(*(long long *)(a2 + 8 * (v14 + 10LL)) + 8 * v16);
          fwrite_unlocked(ptr, 1u, n, a1);
          ++v16;
        }
        while ( v16 <= v19 - v18 );
        if ( *((char *)ptr + n - 1) != 10 )
        {
          v6 = gettext("No newline at end of file");
          fprintf(a1, "\n\\ %s\n", v6);
        }
      }
      if ( v12 == 1 )
        v7 = dword_14010[i];
      else
        v7 = i + 1;
    }
    a2 = *(long long *)(a2 + 104);
  }
  return v23 - __readfsqword(0x28u);
}



// Function: dotlines @ 0x5369
long long dotlines(FILE *a1, long long a2, int a3)
{
  unsigned char v5; // [rsp+2Fh] [rbp-11h]
  long long i; // [rsp+30h] [rbp-10h]
  char *ptr; // [rsp+38h] [rbp-8h]

  v5 = 0;
  for ( i = 0; i <= *(long long *)(a2 + 16LL * a3 + 16) - *(long long *)(a2 + 16LL * a3 + 8); ++i )
  {
    ptr = *(char **)(*(long long *)(a2 + 8 * (a3 + 6LL) + 8) + 8 * i);
    if ( *ptr == 46 )
    {
      v5 = 1;
      fputc_unlocked(46, a1);
    }
    fwrite_unlocked(ptr, 1u, *(long long *)(*(long long *)(a2 + 8 * (a3 + 10LL)) + 8 * i), a1);
  }
  return v5;
}



// Function: undotlines @ 0x544f
int undotlines(FILE *a1, char a2, long long a3, long long a4)
{
  int result; // eax

  result = fputs_unlocked(".\n", a1);
  if ( a2 )
  {
    if ( a4 == 1 )
      return fprintf(a1, "%lds/^\\.//\n", a3);
    else
      return fprintf(a1, "%ld,%lds/^\\.//\n", a3, a3 + a4 - 1);
  }
  return result;
}



// Function: output_diff3_edscript @ 0x54de
long long output_diff3_edscript(FILE *a1, long long a2, int *a3, long long a4, const char *a5, const char *a6, const char *a7)
{
  unsigned int v7; // eax
  const char *v8; // rax
  long long v9; // rbx
  char v10; // al
  char v16; // [rsp+31h] [rbp-2Fh]
  char v17; // [rsp+31h] [rbp-2Fh]
  unsigned char v18; // [rsp+32h] [rbp-2Eh]
  char v19; // [rsp+33h] [rbp-2Dh]
  unsigned int v20; // [rsp+34h] [rbp-2Ch]
  long long i; // [rsp+38h] [rbp-28h]
  long long v22; // [rsp+40h] [rbp-20h]
  long long v23; // [rsp+48h] [rbp-18h]

  v18 = 0;
  for ( i = sub_5F95(a2); i; i = *(long long *)(i + 104) )
  {
    if ( *(int *)i == 4 )
      v7 = 4;
    else
      v7 = *(int *)(4LL * (unsigned int)(*(int *)i - 5) + a4) + 5;
    v20 = v7;
    if ( v7 == 7 )
    {
      if ( byte_1410F )
        continue;
      v19 = 0;
    }
    else
    {
      if ( v7 > 7 )
        continue;
      if ( v7 == 4 )
      {
        if ( byte_1410E )
          continue;
        v19 = byte_1410C;
      }
      else
      {
        if ( v7 != 6 || byte_14110 != 1 )
          continue;
        v19 = 1;
      }
    }
    v22 = *(long long *)(i + 16LL * *a3 + 8);
    v23 = *(long long *)(i + 16LL * *a3 + 16);
    if ( v19 )
    {
      v18 = 1;
      fprintf(a1, "%lda\n", v23);
      v16 = 0;
      if ( v20 == 4 )
      {
        if ( byte_14110 )
        {
          fprintf(a1, "||||||| %s\n", a6);
          v16 = sub_5369(a1, i, a3[1]);
        }
        fputs_unlocked("=======\n", a1);
        v16 = (unsigned char)(v16 | sub_5369(a1, i, a3[2])) != 0;
      }
      fprintf(a1, ">>>>>>> %s\n", a7);
      sub_544F(
        a1,
        v16,
        v23 + 2,
        *(long long *)(i + 16LL * a3[1] + 16)
      - *(long long *)(i + 16LL * a3[1] + 8)
      + 1LL
      + *(long long *)(i + 16LL * a3[2] + 16)
      - *(long long *)(i + 16LL * a3[2] + 8)
      + 1LL
      + 1);
      if ( v20 == 4 )
        v8 = a5;
      else
        v8 = a6;
      fprintf(a1, "%lda\n<<<<<<< %s\n", v22 - 1, v8);
      v17 = 0;
      if ( v20 == 6 )
      {
        v17 = sub_5369(a1, i, a3[1]);
        fputs_unlocked("=======\n", a1);
      }
      sub_544F(a1, v17, v22 + 1, *(long long *)(i + 16LL * a3[1] + 16) - *(long long *)(i + 16LL * a3[1] + 8) + 1LL);
    }
    else if ( *(long long *)(i + 16LL * a3[2] + 16) - *(long long *)(i + 16LL * a3[2] + 8) == -1 )
    {
      if ( v22 == v23 )
        fprintf(a1, "%ldd\n", v22);
      else
        fprintf(a1, "%ld,%ldd\n", v22, v23);
    }
    else
    {
      if ( v23 - v22 == -1 )
      {
        fprintf(a1, "%lda\n", v23);
      }
      else if ( v23 == v22 )
      {
        fprintf(a1, "%ldc\n", v23);
      }
      else
      {
        fprintf(a1, "%ld,%ldc\n", v22, v23);
      }
      v9 = *(long long *)(i + 16LL * a3[2] + 16) - *(long long *)(i + 16LL * a3[2] + 8) + 1LL;
      v10 = sub_5369(a1, i, a3[2]);
      sub_544F(a1, v10, v22, v9);
    }
  }
  if ( byte_14111 )
    fputs_unlocked("w\nq\n", a1);
  return v18;
}



// Function: output_diff3_merge @ 0x5a29
long long output_diff3_merge(
        FILE *a1,
        FILE *a2,
        long long a3,
        int *a4,
        long long a5,
        const char *a6,
        long long a7,
        const char *a8)
{
  unsigned int v8; // eax
  char *v9; // rax
  char *v10; // rax
  int v12; // ebx
  unsigned char v16; // [rsp+36h] [rbp-3Ah]
  char v17; // [rsp+37h] [rbp-39h]
  int c; // [rsp+38h] [rbp-38h]
  int ca; // [rsp+38h] [rbp-38h]
  int cb; // [rsp+38h] [rbp-38h]
  unsigned int v21; // [rsp+3Ch] [rbp-34h]
  long long v22; // [rsp+40h] [rbp-30h]
  long long i; // [rsp+40h] [rbp-30h]
  long long j; // [rsp+40h] [rbp-30h]
  long long k; // [rsp+40h] [rbp-30h]
  long long v26; // [rsp+40h] [rbp-30h]
  long long v28; // [rsp+50h] [rbp-20h]
  long long v29; // [rsp+50h] [rbp-20h]
  char *format; // [rsp+58h] [rbp-18h]

  v16 = 0;
  v28 = 0;
  while ( a3 )
  {
    if ( *(int *)a3 == 4 )
      v8 = 4;
    else
      v8 = *(int *)(4LL * (unsigned int)(*(int *)a3 - 5) + a5) + 5;
    v21 = v8;
    format = "<<<<<<< %s\n";
    if ( v8 == 7 )
    {
      if ( byte_1410F )
        goto LABEL_49;
      v17 = 0;
    }
    else
    {
      if ( v8 > 7 )
        goto LABEL_49;
      if ( v8 == 4 )
      {
        if ( byte_1410E )
          goto LABEL_49;
        v17 = byte_1410C;
        format = "||||||| %s\n";
      }
      else
      {
        if ( v8 != 6 || byte_14110 != 1 )
          goto LABEL_49;
        v17 = 1;
      }
    }
    v22 = *(long long *)(a3 + 8) - v28 - 1;
    v29 = *(long long *)(a3 + 8) - 1LL;
    while ( --v22 >= 0 )
    {
      do
      {
        c = getc_unlocked(a1);
        if ( c == -1 )
        {
          if ( ferror_unlocked(a1) )
          {
            v9 = gettext("read failed");
            sub_600C(v9);
          }
          if ( feof_unlocked(a1) )
            sub_5FCF("input file shrank");
        }
        putc_unlocked(c, a2);
      }
      while ( c != 10 );
    }
    if ( v17 )
    {
      v16 = 1;
      if ( v21 == 4 )
      {
        fprintf(a2, "<<<<<<< %s\n", a6);
        for ( i = 0; i <= *(long long *)(a3 + 16LL * *a4 + 16) - *(long long *)(a3 + 16LL * *a4 + 8); ++i )
          fwrite_unlocked(
            *(const void **)(*(long long *)(a3 + 8 * (*a4 + 6LL) + 8) + 8 * i),
            1u,
            *(long long *)(*(long long *)(a3 + 8 * (*a4 + 10LL)) + 8 * i),
            a2);
      }
      if ( byte_14110 )
      {
        fprintf(a2, format, a7);
        for ( j = 0; j <= *(long long *)(a3 + 16LL * a4[1] + 16) - *(long long *)(a3 + 16LL * a4[1] + 8); ++j )
          fwrite_unlocked(
            *(const void **)(*(long long *)(a3 + 8 * (a4[1] + 6LL) + 8) + 8 * j),
            1u,
            *(long long *)(*(long long *)(a3 + 8 * (a4[1] + 10LL)) + 8 * j),
            a2);
      }
      fputs_unlocked("=======\n", a2);
    }
    for ( k = 0; k <= *(long long *)(a3 + 16LL * a4[2] + 16) - *(long long *)(a3 + 16LL * a4[2] + 8); ++k )
      fwrite_unlocked(
        *(const void **)(*(long long *)(a3 + 8 * (a4[2] + 6LL) + 8) + 8 * k),
        1u,
        *(long long *)(*(long long *)(a3 + 8 * (a4[2] + 10LL)) + 8 * k),
        a2);
    if ( v17 )
      fprintf(a2, ">>>>>>> %s\n", a8);
    v26 = *(long long *)(a3 + 16) - *(long long *)(a3 + 8) + 1LL;
    v28 = v26 + v29;
    while ( --v26 >= 0 )
    {
      while ( 1 )
      {
        ca = getc_unlocked(a1);
        if ( ca == 10 )
          break;
        if ( ca == -1 )
        {
          if ( ferror_unlocked(a1) )
          {
            v10 = gettext("read failed");
            sub_600C(v10);
          }
          if ( feof_unlocked(a1) )
          {
            if ( v26 || *(long long *)(a3 + 104) )
              sub_5FCF("input file shrank");
            return v16;
          }
        }
      }
    }
LABEL_49:
    a3 = *(long long *)(a3 + 104);
  }
  while ( 1 )
  {
    cb = getc_unlocked(a1);
    if ( cb == -1 )
    {
      v12 = ferror_unlocked(a1);
      if ( v12 | feof_unlocked(a1) )
        break;
    }
    putc_unlocked(cb, a2);
  }
  return v16;
}



// Function: reverse_diff3_blocklist @ 0x5f95
long long reverse_diff3_blocklist(long long a1)
{
  long long v2; // r12
  long long v3; // r13

  v2 = 0;
  while ( a1 )
  {
    v3 = *(long long *)(a1 + 104);
    *(long long *)(a1 + 104) = v2;
    v2 = a1;
    a1 = v3;
  }
  return v2;
}



// Function: fatal @ 0x5fcf
void fatal(const char *a1)
{
  char *v1; // rax

  v1 = gettext(a1);
  error(2, 0, "%s", v1);
  sub_600C(2);
}



// Function: perror_with_exit @ 0x600c
void perror_with_exit(const char *a1)
{
  int *v1; // rax

  v1 = __errno_location();
  error(2, *v1, "%s", a1);
  sub_6042();
}


