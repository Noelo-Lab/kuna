// Function: main @ 0x26e0
long long main(int a1, char **a2, char **a3)
{
  const char *v3; // r15
  char *const *v4; // r12
  int v5; // ebp
  char *v6; // rax
  const char *v7; // r13
  int v8; // ecx
  char *v9; // rax
  int v10; // ebx
  char *v11; // rax
  int v12; // eax
  int v13; // ebx
  const char *v14; // r14
  const char *v15; // rsi
  FILE *v16; // rdi
  char *v18; // rax
  char *v19; // rax
  char *v20; // rbx
  int *v21; // rax
  const char *v22; // rdx
  char *v23; // [rsp+0h] [rbp-48h]
  unsigned int v24; // [rsp+0h] [rbp-48h]
  long long v25; // [rsp+8h] [rbp-40h]

  v3 = 0;
  v4 = a2;
  v5 = a1;
  sub_4350(*a2);
  setlocale(6, "");
  bindtextdomain("coreutils", "/usr/local/share/locale");
  textdomain("coreutils");
  sub_7C40(sub_40F0);
  byte_18130 = 0;
  byte_18131 = 0;
  byte_18132 = 0;
  byte_18133 = 0;
  dword_18120 = 75;
  s = (char *)"";
  dword_1811C = 0;
  dword_18118 = 0;
  dword_18114 = 0;
  if ( a1 > 1 )
  {
    v6 = a2[1];
    if ( *v6 == 45 && (unsigned int)(v6[1] - 48) <= 9 )
    {
      v3 = v6 + 1;
      v5 = a1 - 1;
      v4 = a2 + 1;
      a2[1] = *a2;
    }
  }
  v23 = 0;
  v7 = "0123456789cstuw:p:g:";
  while ( 2 )
  {
    v8 = getopt_long(v5, v4, "0123456789cstuw:p:g:", &longopts, 0);
    if ( v8 != -1 )
    {
      if ( v8 <= 119 )
      {
        if ( v8 > 98 )
        {
          switch ( v8 )
          {
            case 'c':
              byte_18133 = 1;
              continue;
            case 'g':
              v23 = optarg;
              continue;
            case 'p':
              sub_2FD0(optarg);
              continue;
            case 's':
              byte_18131 = 1;
              continue;
            case 't':
              byte_18132 = 1;
              continue;
            case 'u':
              byte_18130 = 1;
              continue;
            case 'w':
              v3 = optarg;
              continue;
            default:
              goto LABEL_38;
          }
        }
        if ( v8 == -131 )
        {
          sub_6DD0(
            (int)stdout,
            (unsigned int)"fmt",
            (unsigned int)"GNU coreutils",
            (int)off_D010,
            (unsigned int)"Ross Paterson",
            0,
            (char)v23);
          exit(0);
        }
        if ( v8 == -130 )
          sub_3DC0(0);
      }
LABEL_38:
      if ( (unsigned int)(v8 - 48) <= 9 )
      {
        v24 = v8;
        v19 = dcgettext(
                0,
                "invalid option -- %c; -WIDTH is recognized only when it is the first\noption; use -w N instead",
                5);
        error(0, 0, v19, v24);
      }
      sub_3DC0(1);
    }
    break;
  }
  if ( v3 )
  {
    v7 = "invalid width";
    v9 = dcgettext(0, "invalid width", 5);
    dword_18120 = sub_7450(v3, 0, 2500, "", v9, 0);
    v10 = dword_18120;
    if ( v23 )
    {
      v11 = dcgettext(0, "invalid width", 5);
      dword_18110 = sub_7450(v23, 0, v10, "", v11, 0);
      goto LABEL_21;
    }
    goto LABEL_32;
  }
  if ( !v23 )
  {
LABEL_32:
    dword_18110 = 187 * dword_18120 / 200;
    goto LABEL_21;
  }
  v18 = dcgettext(0, "invalid width", 5);
  dword_18110 = sub_7450(v23, 0, 75, "", v18, 0);
  dword_18120 = dword_18110 + 10;
LABEL_21:
  v12 = optind;
  if ( optind == v5 )
  {
    LOBYTE(v13) = sub_3A30(stdin);
    goto LABEL_34;
  }
  LOBYTE(v23) = 0;
  v13 = 1;
  v7 = "-";
  v3 = "r";
  if ( optind < v5 )
  {
    while ( 1 )
    {
      v14 = v4[v12];
      if ( !strcmp(v14, v7) )
      {
        LOBYTE(v23) = 1;
        v13 &= sub_3A30(stdin);
        goto LABEL_25;
      }
      v15 = v14;
      v16 = fopen(v14, v3);
      if ( v16 )
      {
        v13 &= sub_3A30(v16);
        goto LABEL_25;
      }
      while ( 1 )
      {
        v25 = sub_65C0(4, v15);
        v20 = dcgettext(0, "cannot open %s for reading", 5);
        v21 = __errno_location();
        v22 = v20;
        v13 = 0;
        error(0, *v21, v22, v25);
LABEL_25:
        v12 = optind + 1;
        optind = v12;
        if ( v12 < v5 )
          break;
        if ( !(char)v23 )
          return (unsigned char)v13 ^ 1u;
LABEL_34:
        if ( !(unsigned int)sub_41E0(stdin) )
          return (unsigned char)v13 ^ 1u;
        v4 = (char *const *)dcgettext(0, "closing standard input", 5);
        v15 = (const char *)(unsigned int)*__errno_location();
        error(1, (int)v15, "%s", (const char *)v4);
      }
    }
  }
  return (unsigned char)v13 ^ 1u;
}



// Function: set_other_indent @ 0x2c80
long long set_other_indent(char a1)
{
  long long result; // rax

  if ( byte_18131 )
  {
    dword_D0EC = dword_D0F0;
    return (unsigned int)dword_D0F0;
  }
  else if ( byte_18133 )
  {
    result = (unsigned int)dword_1810C;
    if ( !a1 )
      result = (unsigned int)dword_D0F0;
    dword_D0EC = result;
  }
  else
  {
    result = (unsigned int)dword_D0F0;
    if ( byte_18132 )
    {
      if ( !a1 || dword_1810C == dword_D0F0 )
      {
        if ( dword_D0EC == dword_D0F0 )
        {
          dword_D0EC = dword_D0F0 == 0 ? 3 : 0;
          return (unsigned int)dword_D0EC;
        }
      }
      else
      {
        dword_D0EC = dword_1810C;
      }
    }
    else
    {
      dword_D0EC = dword_D0F0;
    }
  }
  return result;
}



// Function: same_para @ 0x2d10
long long same_para(int a1)
{
  long long result; // rax
  int v2; // edx

  result = 0;
  if ( dword_D0E4 == dword_D0F4 )
  {
    v2 = dword_1811C + dword_D0E4;
    if ( dword_1811C + dword_D0E4 <= dword_1810C )
    {
      LOBYTE(result) = a1 != 10;
      LOBYTE(v2) = a1 != -1;
      return v2 & (unsigned int)result;
    }
  }
  return result;
}



// Function: base_cost @ 0x2d50
long long base_cost(unsigned long long a1)
{
  long long v1; // r8
  char v2; // al
  char v3; // al

  v1 = 4900;
  if ( a1 > (unsigned long long)&unk_D120 )
  {
    v2 = *(char *)(a1 - 24);
    if ( (v2 & 2) != 0 )
    {
      v1 = (v2 & 8) == 0 ? 364900LL : 2400LL;
    }
    else
    {
      v1 = 3300;
      if ( (v2 & 4) == 0 )
      {
        v1 = 4900;
        if ( a1 > (unsigned long long)&unk_D148 && (*(char *)(a1 - 64) & 8) != 0 )
          v1 = 40000LL / (*(int *)(a1 - 32) + 2) + 4900;
      }
    }
  }
  v3 = *(char *)(a1 + 16);
  if ( (v3 & 1) != 0 )
  {
    v1 -= 1600;
    return v1;
  }
  if ( (v3 & 8) == 0 )
    return v1;
  return 22500LL / (*(int *)(a1 + 8) + 2) + v1;
}



// Function: line_cost @ 0x2e00
long long line_cost(long long a1, int a2)
{
  long long result; // rax

  if ( qword_D100 == a1 )
    return 0;
  result = 10 * (dword_18110 - a2) * (long long)(10 * (dword_18110 - a2));
  if ( qword_D100 != *(long long *)(a1 + 32) )
    result += (10 * (a2 - *(int *)(a1 + 20)) * (long long)(10 * (a2 - *(int *)(a1 + 20)))) >> 1;
  return result;
}



// Function: fmt_paragraph @ 0x2e50
long long fmt_paragraph()
{
  void *v0; // r10
  long long v1; // r11
  int v2; // ebx
  unsigned int v3; // eax
  unsigned long long v4; // r15
  int v5; // r14d
  int v6; // ebp
  int v7; // r13d
  int v8; // r8d
  unsigned long long v9; // rcx
  int v10; // r8d
  long long v11; // rax
  int v12; // r8d
  long long v13; // r9
  long long v14; // r11
  long long v15; // rax
  long long v16; // r10
  unsigned long long v17; // rdi
  long long v18; // rax
  long long v19; // r9
  unsigned int v21; // [rsp+4h] [rbp-34h]

  v0 = &unk_D120;
  v1 = qword_D100;
  v2 = dword_18120;
  v3 = *(int *)(qword_D100 + 8);
  v4 = qword_D100 - 40;
  *(long long *)(qword_D100 + 24) = 0;
  *(int *)(v1 + 8) = v2;
  v21 = v3;
  if ( v4 >= (unsigned long long)&unk_D120 )
  {
    v5 = dword_D0EC;
    v6 = dword_D0E0;
    v7 = dword_D0F0;
    do
    {
      v8 = v5;
      v9 = v4;
      if ( (void *)v4 == v0 )
        v8 = v7;
      v10 = *(int *)(v4 + 8) + v8;
      do
      {
        v11 = sub_2E00(v9 + 40, v10);
        v15 = *(long long *)(v9 + 24) + v11;
        if ( v4 == v16 && v6 > 0 )
          v15 += (10 * (v12 - v6) * (long long)(10 * (v12 - v6))) >> 1;
        if ( v15 < v13 )
        {
          *(long long *)(v4 + 32) = v9;
          *(int *)(v4 + 20) = v12;
        }
        if ( v14 == v9 )
          break;
        v10 = *(int *)(v9 - 28) + *(int *)(v9 + 8) + v12;
      }
      while ( v2 > v10 );
      v17 = v4;
      v4 -= 40LL;
      v18 = sub_2D50(v17);
      *(long long *)(v4 + 64) = v19 + v18;
    }
    while ( v4 >= (unsigned long long)v0 );
  }
  *(int *)(v1 + 8) = v21;
  return v21;
}



// Function: set_prefix @ 0x2fd0
long long set_prefix(char *s)
{
  bool v1; // zf
  char *v2; // rbx
  int v3; // edx
  char *v4; // rax
  long long result; // rax

  v1 = *s == 32;
  v2 = s;
  dword_18118 = 0;
  if ( v1 )
  {
    do
      v3 = 1 - (int)s + (int)v2++;
    while ( *v2 == 32 );
    dword_18118 = v3;
  }
  ::s = v2;
  dword_1811C = strlen(v2);
  v4 = &v2[dword_1811C];
  if ( v4 > v2 )
  {
    do
    {
      if ( *(v4 - 1) != 32 )
        break;
      --v4;
    }
    while ( v4 != v2 );
  }
  *v4 = 0;
  result = v4 - v2;
  dword_18114 = result;
  return result;
}



// Function: get_space @ 0x3050
int get_space(FILE *stream, int a2)
{
  int result; // eax
  int v3; // eax

  for ( result = a2; result == 32; result = getc_unlocked(stream) )
  {
    v3 = dword_1810C + 1;
LABEL_4:
    dword_1810C = v3;
  }
  if ( result == 9 )
  {
    byte_D0F8 = 1;
    v3 = 8 * (dword_1810C / 8) + 8;
    goto LABEL_4;
  }
  return result;
}



// Function: get_prefix @ 0x30b0
int get_prefix(FILE *stream)
{
  int v1; // eax
  int result; // eax
  int v3; // edx
  char *v4; // rbx
  int v5; // edx

  dword_1810C = 0;
  v1 = getc_unlocked(stream);
  result = sub_3050(stream, v1);
  v3 = dword_1810C;
  if ( dword_18114 )
  {
    v4 = s;
    dword_D0E4 = dword_1810C;
    v5 = (unsigned char)*s;
    if ( (char)v5 )
    {
      while ( v5 == result )
      {
        ++v4;
        ++dword_1810C;
        result = getc_unlocked(stream);
        v5 = (unsigned char)*v4;
        if ( !(char)v5 )
          return sub_3050(stream, result);
      }
    }
    else
    {
      return sub_3050(stream, result);
    }
  }
  else
  {
    if ( dword_1810C > dword_18118 )
      v3 = dword_18118;
    dword_D0E4 = v3;
  }
  return result;
}



// Function: put_space @ 0x3150
void put_space(int a1)
{
  int v1; // edx
  int v2; // ebx
  int v3; // eax
  int v4; // ebp
  signed int v5; // eax
  int v6; // ebp
  int v7; // eax

  v1 = dword_18108;
  v2 = dword_18108 + a1;
  if ( byte_D0F8 )
  {
    v3 = v2 + 7;
    if ( v2 >= 0 )
      v3 = dword_18108 + a1;
    v4 = v3;
    v5 = v3 & 0xFFFFFFF8;
    v6 = v4 >> 3;
    if ( dword_18108 + 1 < v5 && dword_18108 < v5 )
    {
      do
      {
        putchar_unlocked(9);
        v7 = dword_18108 / 8 + 1;
        v1 = 8 * v7;
        dword_18108 = 8 * v7;
      }
      while ( v6 > v7 );
    }
  }
  if ( v2 > v1 )
  {
    do
    {
      putchar_unlocked(32);
      ++dword_18108;
    }
    while ( dword_18108 < v2 );
  }
}



// Function: put_word @ 0x31f0
int put_word(char **a1)
{
  long long v1; // rbp
  char *v2; // rbx
  char *v4; // rbp
  int v5; // edi
  int result; // eax

  v1 = *((unsigned int *)a1 + 2);
  v2 = *a1;
  if ( (int)v1 )
  {
    v4 = &v2[v1];
    do
    {
      v5 = *v2++;
      result = putchar_unlocked(v5);
    }
    while ( v4 != v2 );
    LODWORD(v1) = *((int *)a1 + 2);
  }
  dword_18108 += v1;
  return result;
}



// Function: put_line @ 0x3230
int put_line(long long a1, int a2)
{
  char **v2; // rbp
  int v3; // ebx
  long long v4; // rbx
  char **v5; // rdi

  v2 = (char **)a1;
  dword_18108 = 0;
  sub_3150(dword_D0F4);
  fputs_unlocked(s, stdout);
  v3 = a2 - (dword_18108 + dword_18114);
  dword_18108 += dword_18114;
  sub_3150(v3);
  v4 = *(long long *)(a1 + 32) - 40LL;
  if ( a1 != v4 )
  {
    do
    {
      v5 = v2;
      v2 += 5;
      sub_31F0(v5);
      sub_3150(*((int *)v2 - 7));
    }
    while ( (char **)v4 != v2 );
  }
  sub_31F0(v2);
  dword_D0E0 = dword_18108;
  return putchar_unlocked(10);
}



// Function: put_paragraph @ 0x32d0
int put_paragraph(long long a1)
{
  int result; // eax
  long long i; // rbx

  result = sub_3230((long long)&unk_D120, dword_D0F0);
  for ( i = qword_D140; a1 != i; i = *(long long *)(i + 32) )
    result = sub_3230(i, dword_D0EC);
  return result;
}



// Function: copy_rest @ 0x3320
long long copy_rest(FILE *stream, unsigned int c)
{
  unsigned int v2; // ebp
  bool v4; // r12
  int v5; // eax
  int v6; // edx
  int v7; // edi
  char *v8; // r13
  int v10; // eax

  v2 = c;
  dword_18108 = 0;
  v4 = c != -1 && c != 10;
  if ( dword_1810C > dword_D0E4 )
  {
    sub_3150(dword_D0E4);
    v5 = dword_18108;
    v6 = dword_1810C;
    v8 = s;
    if ( dword_18108 == dword_1810C )
    {
LABEL_7:
      if ( !v4 )
      {
        if ( c == -1 && dword_D0E4 + dword_18114 <= dword_1810C )
          putchar_unlocked(10);
        return v2;
      }
      v7 = v6 - v5;
      goto LABEL_13;
    }
    do
    {
LABEL_6:
      if ( !*v8 )
        break;
      putchar_unlocked(*v8);
      v6 = dword_1810C;
      ++v8;
      v5 = dword_18108 + 1;
      dword_18108 = v5;
    }
    while ( v5 != dword_1810C );
    goto LABEL_7;
  }
  if ( !v4 )
    return v2;
  sub_3150(dword_D0E4);
  v5 = dword_18108;
  v6 = dword_1810C;
  v7 = 0;
  v8 = s;
  if ( dword_18108 != dword_1810C )
    goto LABEL_6;
LABEL_13:
  sub_3150(v7);
  while ( 1 )
  {
    putchar_unlocked(v2);
    v10 = getc_unlocked(stream);
    v2 = v10;
    if ( v10 == 10 )
      break;
    if ( v10 == -1 )
      return 0xFFFFFFFFLL;
  }
  return v2;
}



// Function: check_punctuation @ 0x3460
long long check_punctuation(unsigned char **a1)
{
  unsigned char *v1; // r12
  char *v2; // rbx
  char *v3; // rbp
  int v4; // edx
  long long result; // rax
  int v6; // ebp

  v1 = *a1;
  v2 = (char *)&(*a1)[*((int *)a1 + 2) - 1];
  v3 = strchr("(['`\"", (char)**a1);
  *((char *)a1 + 16) = (char)a1[2] & 0xFA | (v3 != 0) | (*__ctype_b_loc())[(unsigned char)*v2] & 4;
  if ( v1 < (unsigned char *)v2 )
  {
    while ( 1 )
    {
      v6 = *v2;
      if ( !strchr(")]'\"", v6) )
        break;
      if ( v1 == (unsigned char *)(v2 - 1) )
      {
        v6 = *(v2 - 1);
        break;
      }
      --v2;
    }
  }
  else
  {
    v6 = *v2;
  }
  LOBYTE(v4) = strchr(".?!", v6) != 0;
  v4 *= 2;
  result = v4 | (char)a1[2] & 0xFDu;
  *((char *)a1 + 16) = v4 | (char)a1[2] & 0xFD;
  return result;
}



// Function: flush_paragraph @ 0x3530
size_t flush_paragraph()
{
  long long v0; // r12
  long long v1; // rdx
  long long v2; // rsi
  long long *v3; // rbp
  long long v4; // rax
  long long *v5; // rdi
  long long v6; // rcx
  long long v7; // rcx
  long long v8; // r12
  int v9; // eax
  long long v10; // rdx
  long long v11; // rax
  unsigned long long v12; // r12
  long long v13; // rcx
  long long *i; // rax
  size_t result; // rax

  v0 = qword_D100;
  if ( (_UNKNOWN *)qword_D100 == &unk_D120 )
  {
    result = fwrite_unlocked(&unk_16D80, 1u, qword_16D60 - (long long)&unk_16D80, stdout);
    qword_16D60 = (long long)&unk_16D80;
  }
  else
  {
    sub_2E50();
    v1 = qword_D140;
    if ( v0 == qword_D140 )
    {
      v3 = (long long *)v0;
    }
    else
    {
      v2 = *(long long *)(qword_D140 + 24);
      v3 = (long long *)v0;
      v4 = 0x7FFFFFFFFFFFFFFFLL;
      do
      {
        v5 = (long long *)v1;
        v1 = *(long long *)(v1 + 32);
        v6 = v2;
        v2 = *(long long *)(v1 + 24);
        v7 = v6 - v2;
        if ( v7 < v4 )
        {
          v4 = v7;
          v3 = v5;
        }
        if ( v4 <= 0x7FFFFFFFFFFFFFF6LL )
          v4 += 9;
      }
      while ( v0 != v1 );
    }
    sub_32D0((long long)v3);
    v8 = qword_16D60;
    v9 = __memmove_chk(&unk_16D80, *v3, qword_16D60 - *v3, 5000);
    v10 = *v3;
    v11 = (int)(*v3 - v9);
    qword_16D60 = v8 - v11;
    v12 = qword_D100;
    v13 = -v11;
    if ( qword_D100 >= (unsigned long long)v3 )
    {
      for ( i = v3; ; v10 = *i )
      {
        i += 5;
        *(i - 5) = v13 + v10;
        if ( (unsigned long long)i > v12 )
          break;
      }
    }
    result = __memmove_chk(&unk_D120, v3, v12 - (long long)v3 + 40, 40000);
    qword_D100 = v12 - ((char *)v3 - (char *)&unk_D120);
  }
  return result;
}



// Function: get_line @ 0x3690
int get_line(FILE *stream, char a2)
{
  long long *v2; // rdx
  int v4; // ebp
  int *v5; // rax
  unsigned int v6; // eax
  char v7; // r8
  long long v8; // rdx
  unsigned char **v9; // rdi
  int v10; // r14d
  long long v11; // rdx
  int v12; // r14d
  long long v13; // rax
  int v14; // edx
  char v15; // cl
  bool v16; // dl

  v2 = (long long *)qword_D100;
  LOBYTE(v4) = a2;
  do
  {
    while ( 1 )
    {
      v5 = (int *)qword_16D60;
      *v2 = qword_16D60;
      while ( 1 )
      {
        if ( v5 == &dword_18108 )
        {
          sub_2C80(1);
          sub_3530();
          v5 = (int *)qword_16D60;
        }
        *(char *)v5 = v4;
        qword_16D60 = (long long)v5 + 1;
        v6 = getc_unlocked(stream);
        v4 = v6;
        if ( v6 == -1 )
          break;
        v7 = sub_4030(v6);
        v5 = (int *)qword_16D60;
        v8 = qword_16D60;
        if ( v7 )
          goto LABEL_8;
      }
      v8 = qword_16D60;
LABEL_8:
      v9 = (unsigned char **)qword_D100;
      v10 = dword_1810C;
      v11 = v8 - *(long long *)qword_D100;
      *(int *)(qword_D100 + 8) = v11;
      v12 = v11 + v10;
      dword_1810C = v12;
      sub_3460(v9);
      v4 = sub_3050(stream, v4);
      v13 = qword_D100;
      v14 = dword_1810C - v12;
      *(int *)(qword_D100 + 12) = dword_1810C - v12;
      if ( v4 == -1 )
        break;
      v15 = *(char *)(v13 + 16);
      if ( (v15 & 2) != 0 )
      {
        v16 = v4 == 10 || v14 > 1;
        if ( !v16 )
        {
          *(char *)(v13 + 16) = v15 & 0xF7;
          goto LABEL_12;
        }
      }
      else
      {
        v16 = 0;
      }
      *(char *)(v13 + 16) = (8 * v16) | *(char *)(v13 + 16) & 0xF7;
      if ( v4 == 10 )
        goto LABEL_16;
LABEL_12:
      if ( byte_18130 )
        goto LABEL_16;
      if ( (_UNKNOWN *)v13 == &unk_16D10 )
        goto LABEL_21;
      v2 = (long long *)(v13 + 40);
      qword_D100 = v13 + 40;
    }
    *(char *)(v13 + 16) |= 8u;
LABEL_16:
    *(int *)(v13 + 12) = ((*(char *)(v13 + 16) & 8) != 0) + 1;
    if ( (_UNKNOWN *)v13 == &unk_16D10 )
    {
LABEL_21:
      sub_2C80(1);
      sub_3530();
      v13 = qword_D100;
    }
    v2 = (long long *)(v13 + 40);
    qword_D100 = v13 + 40;
  }
  while ( v4 != 10 && v4 != -1 );
  return sub_30B0(stream);
}



// Function: get_paragraph @ 0x3850
long long get_paragraph(FILE *stream)
{
  int v1; // esi
  int v3; // esi
  char v4; // al
  char v5; // al

  v1 = dword_D0E8;
  dword_D0E0 = 0;
  while ( v1 == 10 || v1 == -1 || dword_D0E4 < dword_18118 || dword_D0E4 + dword_1811C > dword_1810C )
  {
    if ( (unsigned int)sub_3320(stream, v1) == -1 )
    {
      dword_D0E8 = -1;
      return 0;
    }
    putchar_unlocked(10);
    v1 = sub_30B0(stream);
  }
  dword_D0F4 = dword_D0E4;
  dword_D0F0 = dword_1810C;
  qword_16D60 = (long long)&unk_16D80;
  qword_D100 = (long long)&unk_D120;
  v3 = sub_3690(stream, v1);
  v4 = sub_2D10(v3);
  sub_2C80(v4);
  if ( !byte_18131 )
  {
    v5 = sub_2D10(v3);
    if ( byte_18133 )
    {
      if ( v5 )
      {
        do
          v3 = sub_3690(stream, v3);
        while ( (unsigned char)sub_2D10(v3) && dword_1810C == dword_D0EC );
      }
    }
    else if ( byte_18132 )
    {
      if ( v5 && dword_1810C != dword_D0F0 )
      {
        do
          v3 = sub_3690(stream, v3);
        while ( (unsigned char)sub_2D10(v3) && dword_1810C == dword_D0EC );
      }
    }
    else
    {
      while ( (unsigned char)sub_2D10(v3) && dword_1810C == dword_D0EC )
        v3 = sub_3690(stream, v3);
    }
  }
  if ( qword_D100 <= (unsigned long long)&unk_D120 )
    __assert_fail("word < word_limit", "src/fmt.c", 0x270u, "get_paragraph");
  *(char *)(qword_D100 - 24) |= 0xAu;
  dword_D0E8 = v3;
  return 1;
}



// Function: fmt @ 0x3a30
long long fmt(FILE *stream, long long a2)
{
  unsigned int v2; // r12d
  long long v3; // r13
  char *v4; // rax
  int v6; // ebp
  const char *v7; // rax

  sub_41B0(stream, 2);
  byte_D0F8 = 0;
  dword_D0EC = 0;
  dword_D0E8 = sub_30B0(stream);
  while ( 1 )
  {
    v2 = sub_3850(stream);
    if ( !(char)v2 )
      break;
    sub_2E50();
    sub_32D0(qword_D100);
  }
  if ( ferror_unlocked(stream) )
  {
    if ( stream == stdin )
      clearerr_unlocked(stream);
    else
      sub_41E0(stream);
    v3 = sub_66C0(0, 3, a2);
    goto LABEL_8;
  }
  if ( stream == stdin )
  {
    clearerr_unlocked(stream);
    return 1;
  }
  else
  {
    if ( (unsigned int)sub_41E0(stream) )
    {
      v6 = *__errno_location();
      if ( v6 >= 0 )
      {
        v7 = (const char *)sub_66C0(0, 3, a2);
        v3 = (long long)v7;
        if ( v6 )
        {
          error(0, v6, "%s", v7);
          return v2;
        }
LABEL_8:
        v4 = dcgettext(0, "read error", 5);
        error(0, 0, v4, v3);
        return v2;
      }
    }
    return 1;
  }
}



// Function: usage @ 0x3dc0
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
  FILE *v13; // rsi
  char *v14; // rdi

  v2 = qword_18148;
  if ( status )
  {
    v3 = dcgettext(0, "Try '%s --help' for more information.\n", 5);
    __fprintf_chk(stderr, 1, v3, v2);
  }
  else
  {
    v4 = dcgettext(0, "Usage: %s [-WIDTH] [OPTION]... [FILE]...\n", 5);
    __printf_chk(1, v4, v2);
    v5 = stdout;
    v6 = dcgettext(
           0,
           "Reformat each paragraph in the FILE(s), writing to standard output.\n"
           "The option -WIDTH is an abbreviated form of --width=DIGITS.\n",
           5);
    fputs_unlocked(v6, v5);
    sub_2F70();
    sub_2FA0();
    v7 = stdout;
    v8 = dcgettext(
           0,
           "  -c, --crown-margin        preserve indentation of first two lines\n"
           "  -p, --prefix=STRING       reformat only lines beginning with STRING,\n"
           "                              reattaching the prefix to reformatted lines\n"
           "  -s, --split-only          split long lines, but do not refill\n",
           5);
    fputs_unlocked(v8, v7);
    v9 = stdout;
    v10 = dcgettext(
            0,
            "  -t, --tagged-paragraph    indentation of first line different from second\n"
            "  -u, --uniform-spacing     one space between words, two after sentences\n"
            "  -w, --width=WIDTH         maximum line width (default of 75 columns)\n"
            "  -g, --goal=WIDTH          goal width (default of 93% of width)\n",
            5);
    fputs_unlocked(v10, v9);
    v11 = stdout;
    v12 = dcgettext(0, "      --help        display this help and exit\n", 5);
    fputs_unlocked(v12, v11);
    v13 = stdout;
    v14 = dcgettext(0, "      --version     output version information and exit\n", 5);
    fputs_unlocked(v14, v13);
    sub_3B90();
  }
  exit(status);
}


