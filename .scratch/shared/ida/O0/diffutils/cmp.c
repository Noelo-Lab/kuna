// Function: try_help @ 0x29e9
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
  v3 = qword_20268;
  v4 = gettext("Try '%s --help' for more information.");
  error(2, 0, v4, v3);
  return sub_2A5F();
}



// Function: specify_ignore_initial @ 0x2a5f
unsigned long long specify_ignore_initial(int a1, char **a2, char a3)
{
  int v5; // [rsp+14h] [rbp-1Ch]
  long long v6; // [rsp+18h] [rbp-18h] BYREF
  char *v7; // [rsp+20h] [rbp-10h]
  unsigned long long v8; // [rsp+28h] [rbp-8h]

  v8 = __readfsqword(0x28u);
  v7 = *a2;
  v5 = sub_7E2E(v7, a2, 0, &v6, "kKMGTPEZY0");
  if ( v5 && (v5 != 2 || a3 != **a2) || v6 < 0 )
    sub_29E9("invalid --ignore-initial value '%s'", (long long)v7);
  if ( qword_10200[a1] < v6 )
    qword_10200[a1] = v6;
  return v8 - __readfsqword(0x28u);
}



// Function: specify_comparison_type @ 0x2b46
long long specify_comparison_type(unsigned int a1)
{
  if ( dword_10210 && a1 != dword_10210 )
    sub_29E9("options -l and -s are incompatible", 0);
  dword_10210 = a1;
  return a1;
}



// Function: check_stdout @ 0x2b8a
void check_stdout()
{
  char *v0; // rax
  char *v1; // rbx
  int *v2; // rax

  if ( ferror_unlocked(stdout) )
  {
    v0 = gettext("write failed");
    error(2, 0, "%s", v0);
  }
  if ( fclose(stdout) )
  {
    v1 = gettext("standard output");
    v2 = __errno_location();
    error(2, *v2, "%s", v1);
  }
}



// Function: usage @ 0x2c28
long long usage()
{
  long long v0; // rbx
  char *v1; // rax
  char *v2; // rax
  char *v3; // rax
  FILE *v4; // rbx
  char *v5; // rax
  char *v6; // rax
  char *v7; // r12
  char *v8; // rbx
  char *v9; // rax
  const char **i; // [rsp+8h] [rbp-18h]

  v0 = qword_20268;
  v1 = gettext("Usage: %s [OPTION]... FILE1 [FILE2 [SKIP1 [SKIP2]]]\n");
  printf(v1, v0);
  v2 = gettext("Compare two files byte by byte.");
  printf("%s\n", v2);
  v3 = gettext(
         "The optional SKIP1 and SKIP2 specify the number of bytes to skip\n"
         "at the beginning of each file (zero by default).");
  printf("\n%s\n\n", v3);
  v4 = stdout;
  v5 = gettext("Mandatory arguments to long options are mandatory for short options too.\n");
  fputs_unlocked(v5, v4);
  for ( i = (const char **)off_FB40; *i; ++i )
  {
    v6 = gettext(*i);
    printf("  %s\n", v6);
  }
  v7 = gettext("Exit status is 0 if inputs are the same, 1 if different, 2 if trouble.");
  v8 = gettext("If a FILE is '-' or missing, read standard input.");
  v9 = gettext(
         "SKIP values may be followed by the following multiplicative suffixes:\n"
         "kB 1000, K 1024, MB 1,000,000, M 1,048,576,\n"
         "GB 1,000,000,000, G 1,073,741,824, and so on for T, P, E, Z, Y.");
  printf("\n%s\n\n%s\n%s\n", v9, v8, v7);
  return sub_74C0();
}



// Function: main @ 0x2d6d
long long main(int a1, char **a2, char **a3)
{
  char *v3; // rax
  int v4; // ebx
  int v5; // eax
  long long result; // rax
  int v7; // eax
  int v8; // eax
  char *v9; // rax
  int v10; // eax
  const char *v11; // rbx
  int *v12; // rax
  int v13; // eax
  long long v14; // rbx
  int v15; // eax
  long long v16; // rbx
  long long v17; // rbx
  long long v18; // rdx
  const char *v19; // rbx
  int *v20; // rax
  int i; // [rsp+1Ch] [rbp-174h]
  int j; // [rsp+20h] [rbp-170h]
  int k; // [rsp+24h] [rbp-16Ch]
  int v24; // [rsp+28h] [rbp-168h]
  int status; // [rsp+2Ch] [rbp-164h]
  char *v26; // [rsp+30h] [rbp-160h] BYREF
  long long v27; // [rsp+38h] [rbp-158h]
  long long v28; // [rsp+40h] [rbp-150h]
  unsigned long long v29; // [rsp+48h] [rbp-148h]
  struct stat buf; // [rsp+50h] [rbp-140h] BYREF
  struct stat v31; // [rsp+E0h] [rbp-B0h] BYREF
  unsigned long long v32; // [rsp+178h] [rbp-18h]

  v32 = __readfsqword(0x28u);
  ::status = 2;
  sub_4A4B(*a2, a2, a3);
  setlocale(6, &locale);
  bindtextdomain("diffutils", "/usr/local/share/locale");
  textdomain("diffutils");
  sub_479F(0);
  sub_7C09();
  while ( 2 )
  {
    v24 = getopt_long(a1, a2, "bci:ln:sv", &longopts, 0);
    if ( v24 == -1 )
    {
      if ( a1 == optind )
        sub_29E9("missing operand after '%s'", (long long)a2[a1 - 1]);
      v7 = optind++;
      s1 = a2[v7];
      if ( a1 <= optind )
      {
        v9 = "-";
      }
      else
      {
        v8 = optind++;
        v9 = a2[v8];
      }
      s2 = v9;
      for ( i = 0; i <= 1 && a1 > optind; ++i )
      {
        v10 = optind++;
        v26 = a2[v10];
        sub_2A5F(i, &v26, 0);
      }
      if ( a1 > optind )
        sub_29E9("extra operand '%s'", (long long)a2[optind]);
      for ( j = 0; j <= 1; ++j )
      {
        if ( j && qword_10200[0] == qword_10208 && !strcmp(s1, s2) )
          return 0;
        if ( !strcmp((&s1)[j], "-") )
          dword_100B0[j] = 0;
        else
          dword_100B0[j] = open((&s1)[j], 0, 0);
        if ( dword_100B0[j] < 0 || fstat(dword_100B0[j], (struct stat *)&qword_100C0[18 * j]) )
        {
          if ( dword_100B0[j] < 0 && dword_10210 == 3 )
            exit(2);
          v11 = (&s1)[j];
          v12 = __errno_location();
          error(2, *v12, "%s", v11);
        }
      }
      v13 = qword_100C8 == qword_10158 && qword_100C0[0] == qword_10150
         || ((dword_100D8 & 0xF000) == 0x6000 && (dword_10168 & 0xF000) == 0x6000
          || (dword_100D8 & 0xF000) == 0x2000 && (dword_10168 & 0xF000) == 0x2000)
         && qword_100E8 == qword_10178;
      if ( v13 <= 0
        || dword_100D8 != dword_10168
        || qword_100D0 != qword_10160
        || dword_100DC != dword_1016C
        || dword_100E0 != dword_10170
        || qword_100F0 != qword_10180
        || qword_10118 != qword_101A8
        || qword_10128 != qword_101B8
        || (v14 = sub_447F(0), v14 != sub_447F(1)) )
      {
        if ( dword_10210 != 3 && !fstat(1, &buf) && !stat("/dev/null", &v31) )
        {
          v15 = buf.st_ino == v31.st_ino && buf.st_dev == v31.st_dev
             || ((buf.st_mode & 0xF000) == 0x6000 && (v31.st_mode & 0xF000) == 0x6000
              || (buf.st_mode & 0xF000) == 0x2000 && (v31.st_mode & 0xF000) == 0x2000)
             && buf.st_rdev == v31.st_rdev;
          if ( v15 > 0 )
            dword_10210 = 2;
        }
        if ( dword_10210 == 3 && (dword_100D8 & 0xF000) == 0x8000 && (dword_10168 & 0xF000) == 0x8000 )
        {
          v16 = qword_100F0;
          v27 = v16 - sub_447F(0);
          v17 = qword_10180;
          v28 = v17 - sub_447F(1);
          if ( v27 < 0 )
            v27 = 0;
          if ( v28 < 0 )
            v28 = 0;
          if ( v27 != v28 )
          {
            if ( qword_10010 < 0 )
              goto LABEL_92;
            v18 = v27;
            if ( v27 > v28 )
              v18 = v28;
            if ( v18 < qword_10010 )
LABEL_92:
              exit(1);
          }
        }
        qword_101F0 = sub_8307(qword_100F8, qword_10188, 0x7FFFFFFFFFFFFFF7LL);
        v29 = (unsigned long long)(qword_101F0 + 15) >> 3;
        qword_101E0 = sub_7605(16 * v29);
        qword_101E8 = qword_101E0 + 8 * v29;
        status = sub_37D2();
        for ( k = 0; k <= 1; ++k )
        {
          if ( close(dword_100B0[k]) )
          {
            v19 = (&s1)[k];
            v20 = __errno_location();
            error(2, *v20, "%s", v19);
          }
        }
        if ( status )
        {
          if ( (unsigned int)dword_10210 <= 1 )
            sub_2B8A();
        }
        exit(status);
      }
      return 0;
    }
    else
    {
      switch ( v24 )
      {
        case 98:
        case 99:
          byte_10214 = 1;
          continue;
        case 105:
          sub_2A5F(0, (char **)&optarg, 58);
          v3 = (char *)optarg++;
          if ( *v3 == 58 )
          {
            sub_2A5F(1, (char **)&optarg, 0);
          }
          else if ( qword_10208 < qword_10200[0] )
          {
            qword_10208 = qword_10200[0];
          }
          continue;
        case 108:
          sub_2B46(1u);
          continue;
        case 110:
          if ( (unsigned int)sub_7E2E(optarg, 0, 0, &v26, "kKMGTPEZY0") || (long long)v26 < 0 )
            sub_29E9("invalid --bytes value '%s'", optarg);
          if ( qword_10010 < 0 || qword_10010 >= (long long)v26 )
            qword_10010 = (long long)v26;
          continue;
        case 115:
          sub_2B46(3u);
          continue;
        case 118:
          v4 = sub_500C("David MacKenzie");
          v5 = sub_50B2("Torbjorn Granlund", &unk_C60C);
          sub_73DC((int)stdout, (unsigned int)"cmp", (unsigned int)"GNU diffutils", (int)off_10018, v5, v4, 0);
          sub_2B8A();
          result = 0;
          break;
        case 128:
          sub_2C28();
          sub_2B8A();
          result = 0;
          break;
        default:
          sub_29E9(0, 0);
          continue;
      }
    }
    return result;
  }
}



// Function: cmp @ 0x37d2
long long cmp()
{
  long long v0; // rax
  long long v1; // rbx
  long long v2; // rax
  const char *v3; // rbx
  int *v4; // rax
  const char *v5; // rbx
  int *v6; // rax
  const char *v7; // rbx
  int *v8; // rax
  size_t v9; // rax
  char *v11; // r12
  char *v12; // rbx
  char *v13; // rax
  char *v15; // rax
  char *v16; // rax
  char *v17; // rax
  bool v18; // [rsp+Eh] [rbp-142h]
  unsigned char v19; // [rsp+Fh] [rbp-141h]
  unsigned char v20; // [rsp+10h] [rbp-140h]
  unsigned char v21; // [rsp+11h] [rbp-13Fh]
  unsigned char v22; // [rsp+12h] [rbp-13Eh]
  int v23; // [rsp+14h] [rbp-13Ch]
  int i; // [rsp+18h] [rbp-138h]
  int k; // [rsp+18h] [rbp-138h]
  int j; // [rsp+1Ch] [rbp-134h]
  long long v27; // [rsp+20h] [rbp-130h]
  long long v28; // [rsp+28h] [rbp-128h]
  long long v29; // [rsp+30h] [rbp-120h]
  unsigned long long v30; // [rsp+38h] [rbp-118h]
  long long v31; // [rsp+40h] [rbp-110h]
  unsigned long long v32; // [rsp+48h] [rbp-108h]
  long long v33; // [rsp+50h] [rbp-100h]
  long long v34; // [rsp+58h] [rbp-F8h]
  long long v35; // [rsp+60h] [rbp-F0h]
  char *s1; // [rsp+68h] [rbp-E8h]
  char *s2; // [rsp+70h] [rbp-E0h]
  long long v38; // [rsp+78h] [rbp-D8h]
  unsigned long long v39; // [rsp+80h] [rbp-D0h]
  unsigned long long v40; // [rsp+88h] [rbp-C8h]
  size_t n; // [rsp+90h] [rbp-C0h]
  const char *v42; // [rsp+98h] [rbp-B8h]
  long long v43; // [rsp+A0h] [rbp-B0h]
  long long v44; // [rsp+A8h] [rbp-A8h]
  const char *format; // [rsp+B0h] [rbp-A0h]
  char *v46; // [rsp+B8h] [rbp-98h]
  long long v47; // [rsp+C0h] [rbp-90h]
  long long v48; // [rsp+C8h] [rbp-88h]
  long long v49; // [rsp+D0h] [rbp-80h]
  long long v50; // [rsp+D8h] [rbp-78h]
  char v51[5]; // [rsp+E6h] [rbp-6Ah] BYREF
  char v52[5]; // [rsp+EBh] [rbp-65h] BYREF
  char v53[32]; // [rsp+F0h] [rbp-60h] BYREF
  char v54[24]; // [rsp+110h] [rbp-40h] BYREF
  unsigned long long v55; // [rsp+128h] [rbp-28h]

  v55 = __readfsqword(0x28u);
  v18 = 1;
  v27 = 1;
  v28 = 1;
  v29 = qword_10010;
  v34 = qword_101E0;
  v35 = qword_101E8;
  s1 = (char *)qword_101E0;
  s2 = (char *)qword_101E8;
  v23 = 0;
  if ( dword_10210 == 1 )
  {
    if ( qword_10010 < 0 )
      v0 = 0x7FFFFFFFFFFFFFFFLL;
    else
      v0 = qword_10010;
    v31 = v0;
    for ( i = 0; i <= 1; ++i )
    {
      if ( (dword_100D8[36 * i] & 0xF000) == 0x8000 )
      {
        v1 = qword_100F0[18 * i];
        v38 = v1 - sub_447F((unsigned int)i);
        if ( v38 < v31 )
          v31 = v38;
      }
    }
    for ( j = 1; ; ++j )
    {
      v31 /= 10;
      if ( !v31 )
        break;
    }
  }
  for ( k = 0; k <= 1; ++k )
  {
    v32 = qword_10200[k];
    if ( v32 && sub_447F((unsigned int)k) == -1 )
    {
      do
      {
        v2 = qword_101F0;
        if ( v32 <= qword_101F0 )
          v2 = v32;
        v49 = v2;
        v50 = sub_8238((unsigned int)dword_100B0[k], s1, v2);
        if ( v50 != v49 )
        {
          if ( v50 != -1 )
            break;
          v3 = (&::s1)[k];
          v4 = __errno_location();
          error(2, *v4, "%s", v3);
        }
        v32 -= v50;
      }
      while ( v32 );
    }
  }
  while ( 1 )
  {
    v33 = qword_101F0;
    if ( v29 >= 0 )
    {
      if ( qword_101F0 > (unsigned long long)v29 )
        v33 = v29;
      v29 -= v33;
    }
    v39 = sub_8238((unsigned int)dword_100B0[0], s1, v33);
    if ( v39 == -1 )
    {
      v5 = ::s1;
      v6 = __errno_location();
      error(2, *v6, "%s", v5);
    }
    v40 = sub_8238((unsigned int)dword_100B4, s2, v33);
    if ( v40 == -1 )
    {
      v7 = ::s2;
      v8 = __errno_location();
      error(2, *v8, "%s", v7);
    }
    v9 = v40;
    if ( v39 <= v40 )
      v9 = v39;
    n = v9;
    if ( !memcmp(s1, s2, v9) )
    {
      v30 = n;
    }
    else
    {
      if ( v39 >= v40 )
        s2[v39] = 85;
      if ( v40 >= v39 )
        s1[v40] = 121;
      s1[v39] = ~s2[v39];
      s2[v40] = ~s1[v40];
      memset(&s1[v39 + 1], 0, ~(char)v39 & 7);
      memset(&s2[v40 + 1], 0, ~(char)v40 & 7);
      v30 = sub_42D7(v34, v35);
    }
    v28 += v30;
    if ( !dword_10210 && v30 )
    {
      v27 += sub_4350(s1, v30);
      v18 = s1[v30 - 1] == 10;
    }
    if ( v30 < n )
    {
      if ( dword_10210 == 3 )
        return 1;
      if ( (unsigned int)dword_10210 <= 3 )
      {
        if ( dword_10210 == 2 )
        {
          v23 = 1;
        }
        else if ( (unsigned int)dword_10210 <= 2 )
        {
          if ( !dword_10210 )
          {
            v43 = sub_490D(v28, v53);
            v44 = sub_490D(v27, v54);
            if ( byte_10214 != 1 )
            {
              format = gettext("%s %s differ: byte %s, line %s\n");
              if ( format != "%s %s differ: byte %s, line %s\n" || (unsigned char)sub_4863(5) )
                printf(format, ::s1, ::s2, v43, v44);
              else
                printf("%s %s differ: char %s, line %s\n", ::s1, ::s2, v43, v44);
            }
            else
            {
              v21 = s1[v30];
              v22 = s2[v30];
              sub_43CF(v51, v21);
              sub_43CF(v52, v22);
              v11 = ::s2;
              v12 = ::s1;
              v13 = gettext("%s %s differ: byte %s, line %s is %3o %s %3o %s\n");
              printf(v13, v12, v11, v43, v44, v21, v51, v22, v52);
            }
            return 1;
          }
          do
          {
            v19 = s1[v30];
            v20 = s2[v30];
            if ( v19 != v20 )
            {
              v42 = (const char *)sub_490D(v28, v54);
              if ( byte_10214 != 1 )
              {
                printf("%*s %3o %3o\n", j, v42, v19, v20);
              }
              else
              {
                sub_43CF(v52, v19);
                sub_43CF(v53, v20);
                printf("%*s %3o %-4s %3o %s\n", j, v42, v19, v52, v20, v53);
              }
            }
            ++v28;
            ++v30;
          }
          while ( v30 < n );
          v23 = -1;
        }
      }
    }
    if ( v39 != v40 )
      break;
    if ( v23 > 0 || v39 != qword_101F0 )
      return v23 != 0;
  }
  if ( v23 <= 0 && dword_10210 != 3 )
  {
    v46 = (&::s1)[v40 < v39];
    if ( v28 == 1 )
    {
      v15 = gettext("cmp: EOF on %s which is empty\n");
      fprintf(stderr, v15, v46);
    }
    else
    {
      v47 = sub_490D(v28 - 1, v53);
      if ( dword_10210 )
      {
        v17 = gettext("cmp: EOF on %s after byte %s\n");
        fprintf(stderr, v17, v46, v47);
      }
      else
      {
        v48 = sub_490D(v27 - v18, v54);
        if ( v18 )
          v16 = gettext("cmp: EOF on %s after byte %s, line %s\n");
        else
          v16 = gettext("cmp: EOF on %s after byte %s, in line %s\n");
        fprintf(stderr, v16, v46, v47, v48);
      }
    }
  }
  return 1;
}



// Function: block_compare @ 0x42d7
char *block_compare(long long *a1, long long *a2)
{
  long long *i; // [rsp+10h] [rbp-20h]
  char *v5; // [rsp+20h] [rbp-10h]
  char *j; // [rsp+28h] [rbp-8h]

  for ( i = a1; *i == *a2; ++i )
    ++a2;
  v5 = i;
  for ( j = a2; *v5 == *j; ++j )
    ++v5;
  return (char *)(v5 - (char *)a1);
}



// Function: count_newlines @ 0x4350
long long count_newlines(char *a1, long long a2)
{
  char *i; // rax
  char v4[9]; // [rsp+17h] [rbp-19h]
  char *v5; // [rsp+28h] [rbp-8h]

  v4[8] = 0;
  v5 = &a1[a2];
  *(long long *)v4 = (unsigned char)a1[a2];
  a1[a2] = 10;
  for ( i = (char *)rawmemchr(a1, 10); i != v5; i = (char *)rawmemchr(i + 1, 10) )
    ++*(long long *)&v4[1];
  *v5 = v4[0];
  return *(long long *)&v4[1];
}



// Function: sprintc @ 0x43cf
char *sprintc(char *a1, char a2)
{
  char *v2; // rax
  char *v3; // rax
  char v5; // [rsp+4h] [rbp-Ch]
  char *v6; // [rsp+8h] [rbp-8h]

  v6 = a1;
  v5 = a2;
  if ( ((*__ctype_b_loc())[(unsigned char)a2] & 0x4000) == 0 )
  {
    if ( a2 < 0 )
    {
      *a1 = 77;
      v6 = a1 + 2;
      a1[1] = 45;
      v5 = a2 + 0x80;
    }
    if ( (unsigned char)v5 > 0x1Fu )
    {
      if ( v5 == 127 )
      {
        v3 = v6++;
        *v3 = 94;
        v5 = 63;
      }
    }
    else
    {
      v2 = v6++;
      *v2 = 94;
      v5 += 64;
    }
  }
  *v6 = v5;
  v6[1] = 0;
  return v6 + 1;
}



// Function: file_position @ 0x447f
long long file_position(int a1)
{
  if ( byte_10215[a1] != 1 )
  {
    byte_10215[a1] = 1;
    qword_10220[a1] = lseek(dword_100B0[a1], qword_10200[a1], 1);
  }
  return qword_10220[a1];
}


