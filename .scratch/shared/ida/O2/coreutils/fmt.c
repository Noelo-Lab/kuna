// Function: main @ 0x26a0
long long main(int a1, char **a2, char **a3)
{
  const char *v3; // r15
  char *const *v4; // r12
  int v5; // ebp
  char *v6; // rax
  const char *v7; // r13
  int v8; // ecx
  int v9; // ecx
  const char *v10; // rax
  char *v11; // rax
  int v12; // ebx
  char *v13; // rax
  int v14; // eax
  int v15; // ebx
  const char *v16; // r14
  const char *v17; // rsi
  FILE *v18; // rdi
  char *v20; // rax
  char *v21; // rax
  char *v22; // rbx
  int *v23; // rax
  const char *v24; // rdx
  char *v25; // [rsp+0h] [rbp-48h]
  unsigned int v26; // [rsp+0h] [rbp-48h]
  char *v27; // [rsp+8h] [rbp-40h]
  long long v28; // [rsp+8h] [rbp-40h]

  v4 = a2;
  v5 = a1;
  sub_4350(*a2);
  setlocale(6, "");
  bindtextdomain("coreutils", "/usr/local/share/locale");
  textdomain("coreutils");
  sub_7B90(sub_4100);
  byte_17130 = 0;
  byte_17131 = 0;
  byte_17132 = 0;
  byte_17133 = 0;
  dword_17120 = 75;
  s = (char *)"";
  dword_1711C = 0;
  dword_17118 = 0;
  dword_17114 = 0;
  v27 = 0;
  if ( a1 > 1 )
  {
    v6 = a2[1];
    if ( *v6 == 45 && (unsigned int)(v6[1] - 48) <= 9 )
    {
      v4 = a2 + 1;
      v5 = a1 - 1;
      v27 = v6 + 1;
      a2[1] = *a2;
    }
  }
  v25 = 0;
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
              byte_17133 = 1;
              continue;
            case 'g':
              v25 = optarg;
              continue;
            case 'p':
              dword_17118 = 0;
              v3 = optarg;
              if ( *optarg == 32 )
              {
                do
                  v9 = 1 - (int)optarg + (int)v3++;
                while ( *v3 == 32 );
                dword_17118 = v9;
              }
              s = (char *)v3;
              dword_1711C = strlen(v3);
              v10 = &v3[dword_1711C];
              if ( v3 < v10 )
              {
                do
                {
                  if ( *(v10 - 1) != 32 )
                    break;
                  --v10;
                }
                while ( v3 != v10 );
              }
              *v10 = 0;
              dword_17114 = (int)v10 - (int)v3;
              continue;
            case 's':
              byte_17131 = 1;
              continue;
            case 't':
              byte_17132 = 1;
              continue;
            case 'u':
              byte_17130 = 1;
              continue;
            case 'w':
              v27 = optarg;
              continue;
            default:
              goto LABEL_45;
          }
        }
        if ( v8 == -131 )
        {
          sub_6C80(stdout, (char)v25);
          exit(0);
        }
        if ( v8 == -130 )
          sub_3D70(0);
      }
LABEL_45:
      if ( (unsigned int)(v8 - 48) <= 9 )
      {
        v26 = v8;
        v21 = dcgettext(
                0,
                "invalid option -- %c; -WIDTH is recognized only when it is the first\noption; use -w N instead",
                5);
        error(0, 0, v21, v26);
      }
      sub_3D70(1);
    }
    break;
  }
  if ( v27 )
  {
    v7 = "invalid width";
    v11 = dcgettext(0, "invalid width", 5);
    dword_17120 = sub_74A0(v27, 0, 2500, "", v11, 0);
    v12 = dword_17120;
    if ( v25 )
    {
      v13 = dcgettext(0, "invalid width", 5);
      dword_17110 = sub_74A0(v25, 0, v12, "", v13, 0);
      goto LABEL_28;
    }
    goto LABEL_39;
  }
  if ( !v25 )
  {
LABEL_39:
    dword_17110 = 187 * dword_17120 / 200;
    goto LABEL_28;
  }
  v20 = dcgettext(0, "invalid width", 5);
  dword_17110 = sub_74A0(v25, 0, 75, "", v20, 0);
  dword_17120 = dword_17110 + 10;
LABEL_28:
  v14 = optind;
  if ( optind == v5 )
  {
    LOBYTE(v15) = sub_3700(stdin);
    goto LABEL_41;
  }
  LOBYTE(v25) = 0;
  v15 = 1;
  v7 = "-";
  v3 = "r";
  if ( optind < v5 )
  {
    while ( 1 )
    {
      v16 = v4[v14];
      if ( !strcmp(v16, v7) )
      {
        LOBYTE(v25) = 1;
        v15 &= sub_3700(stdin);
        goto LABEL_32;
      }
      v17 = v16;
      v18 = fopen(v16, v3);
      if ( v18 )
      {
        v15 &= sub_3700(v18);
        goto LABEL_32;
      }
      while ( 1 )
      {
        v28 = sub_6030(4, v17);
        v22 = dcgettext(0, "cannot open %s for reading", 5);
        v23 = __errno_location();
        v24 = v22;
        v15 = 0;
        error(0, *v23, v24, v28);
LABEL_32:
        v14 = optind + 1;
        optind = v14;
        if ( v14 < v5 )
          break;
        if ( !(char)v25 )
          return (unsigned char)v15 ^ 1u;
LABEL_41:
        if ( !(unsigned int)sub_41F0(stdin) )
          return (unsigned char)v15 ^ 1u;
        v4 = (char *const *)dcgettext(0, "closing standard input", 5);
        v17 = (const char *)(unsigned int)*__errno_location();
        error(1, (int)v17, "%s", (const char *)v4);
      }
    }
  }
  return (unsigned char)v15 ^ 1u;
}



// Function: fmt_paragraph @ 0x2cb0
long long fmt_paragraph()
{
  long long v0; // rsi
  int v1; // r9d
  unsigned long long v2; // rcx
  unsigned int v3; // eax
  int v4; // r13d
  int v5; // r12d
  int v6; // r10d
  int v7; // r11d
  int v8; // r15d
  int v9; // ebx
  unsigned long long v10; // rdx
  long long v11; // rdi
  int v12; // r15d
  long long v13; // rax
  long long v14; // r14
  long long v15; // rax
  long long v16; // r14
  char v17; // al
  char v18; // al
  unsigned int v20; // [rsp+0h] [rbp-34h]

  v0 = qword_C100;
  v1 = dword_17120;
  v2 = qword_C100 - 40;
  v3 = *(int *)(qword_C100 + 8);
  *(long long *)(qword_C100 + 24) = 0;
  *(int *)(v0 + 8) = v1;
  v20 = v3;
  if ( v2 >= (unsigned long long)&unk_C120 )
  {
    v4 = dword_C0EC;
    v5 = dword_C0F0;
    v6 = dword_17110;
    v7 = dword_C0E0;
    do
    {
      v8 = v4;
      v9 = *(int *)(v2 + 8);
      v10 = v2;
      if ( (_UNKNOWN *)v2 == &unk_C120 )
        v8 = v5;
      v11 = 0x7FFFFFFFFFFFFFFFLL;
      v12 = v9 + v8;
      do
      {
        v10 += 40LL;
        if ( v0 == v10 )
        {
          v13 = 0;
        }
        else
        {
          v13 = 10 * (v6 - v12) * (long long)(10 * (v6 - v12));
          if ( v0 != *(long long *)(v10 + 32) )
          {
            v14 = 10 * (v12 - *(int *)(v10 + 20));
            v13 += (v14 * v14) >> 1;
          }
        }
        v15 = *(long long *)(v10 + 24) + v13;
        if ( (_UNKNOWN *)v2 == &unk_C120 && v7 > 0 )
          v15 += (10 * (v12 - v7) * (long long)(10 * (v12 - v7))) >> 1;
        if ( v15 < v11 )
        {
          *(long long *)(v2 + 32) = v10;
          v11 = v15;
          *(int *)(v2 + 20) = v12;
        }
        if ( v0 == v10 )
          break;
        v12 += *(int *)(v10 - 28) + *(int *)(v10 + 8);
      }
      while ( v1 > v12 );
      v16 = 4900;
      if ( v2 > (unsigned long long)&unk_C120 )
      {
        v17 = *(char *)(v2 - 24);
        if ( (v17 & 2) != 0 )
        {
          v16 = (v17 & 8) == 0 ? 364900LL : 2400LL;
        }
        else
        {
          v16 = 3300;
          if ( (v17 & 4) == 0 )
          {
            v16 = 4900;
            if ( v2 > (unsigned long long)&unk_C148 && (*(char *)(v2 - 64) & 8) != 0 )
              v16 = 40000LL / (*(int *)(v2 - 32) + 2) + 4900;
          }
        }
      }
      v18 = *(char *)(v2 + 16);
      if ( (v18 & 1) != 0 )
      {
        v16 -= 1600;
      }
      else if ( (v18 & 8) != 0 )
      {
        v16 += 22500LL / (v9 + 2);
      }
      v2 -= 40LL;
      *(long long *)(v2 + 64) = v16 + v11;
    }
    while ( v2 >= (unsigned long long)&unk_C120 );
  }
  *(int *)(v0 + 8) = v20;
  return v20;
}



// Function: get_space @ 0x2ea0
int get_space(_IO_FILE *a1, int a2)
{
  int result; // eax
  char *IO_read_ptr; // rax

  for ( result = a2; ; result = (unsigned char)*IO_read_ptr )
  {
    while ( result == 32 )
    {
      ++dword_1710C;
      IO_read_ptr = a1->_IO_read_ptr;
      if ( IO_read_ptr < a1->_IO_read_end )
        goto LABEL_4;
LABEL_7:
      result = __uflow(a1);
    }
    if ( result != 9 )
      break;
    byte_C0F8 = 1;
    dword_1710C = 8 * (dword_1710C / 8) + 8;
    IO_read_ptr = a1->_IO_read_ptr;
    if ( IO_read_ptr >= a1->_IO_read_end )
      goto LABEL_7;
LABEL_4:
    a1->_IO_read_ptr = IO_read_ptr + 1;
  }
  return result;
}



// Function: get_prefix @ 0x2f30
int get_prefix(_IO_FILE *a1)
{
  char *IO_read_ptr; // rax
  int v2; // esi
  int result; // eax
  int v4; // edx
  char *v5; // rbx
  int v6; // edx
  char *v7; // rax

  IO_read_ptr = a1->_IO_read_ptr;
  dword_1710C = 0;
  if ( IO_read_ptr >= a1->_IO_read_end )
  {
    v2 = __uflow(a1);
  }
  else
  {
    a1->_IO_read_ptr = IO_read_ptr + 1;
    v2 = (unsigned char)*IO_read_ptr;
  }
  result = sub_2EA0(a1, v2);
  v4 = dword_1710C;
  if ( dword_17114 )
  {
    v5 = s;
    dword_C0E4 = dword_1710C;
    v6 = (unsigned char)*s;
    if ( (char)v6 )
    {
      while ( v6 == result )
      {
        ++dword_1710C;
        v7 = a1->_IO_read_ptr;
        if ( v7 < a1->_IO_read_end )
        {
          a1->_IO_read_ptr = v7 + 1;
          result = (unsigned char)*v7;
        }
        else
        {
          result = __uflow(a1);
        }
        v6 = (unsigned char)*++v5;
        if ( !(char)v6 )
          return sub_2EA0(a1, result);
      }
    }
    else
    {
      return sub_2EA0(a1, result);
    }
  }
  else
  {
    if ( dword_1710C > dword_17118 )
      v4 = dword_17118;
    dword_C0E4 = v4;
  }
  return result;
}



// Function: put_word @ 0x3000
void put_word(unsigned char **a1)
{
  long long v1; // rbp
  unsigned char *v2; // rbx
  unsigned char *v3; // rbp
  int v4; // esi
  char *IO_write_ptr; // rax

  v1 = *((unsigned int *)a1 + 2);
  v2 = *a1;
  if ( (int)v1 )
  {
    v3 = &v2[v1];
    do
    {
      v4 = *v2++;
      IO_write_ptr = stdout->_IO_write_ptr;
      if ( IO_write_ptr >= stdout->_IO_write_end )
      {
        __overflow(stdout, v4);
      }
      else
      {
        stdout->_IO_write_ptr = IO_write_ptr + 1;
        *IO_write_ptr = v4;
      }
    }
    while ( v2 != v3 );
    LODWORD(v1) = *((int *)a1 + 2);
  }
  dword_17108 += v1;
}



// Function: put_space @ 0x3060
void put_space(int a1)
{
  int v1; // edx
  int v2; // ebx
  int v3; // eax
  int v4; // ebp
  signed int v5; // eax
  int v6; // ebp
  char *IO_write_ptr; // rax
  int v8; // eax
  char *v9; // rax

  v1 = dword_17108;
  v2 = dword_17108 + a1;
  if ( byte_C0F8 )
  {
    v3 = v2 + 7;
    if ( v2 >= 0 )
      v3 = dword_17108 + a1;
    v4 = v3;
    v5 = v3 & 0xFFFFFFF8;
    v6 = v4 >> 3;
    if ( dword_17108 + 1 < v5 && dword_17108 < v5 )
    {
      do
      {
        IO_write_ptr = stdout->_IO_write_ptr;
        if ( IO_write_ptr >= stdout->_IO_write_end )
        {
          __overflow(stdout, 9);
        }
        else
        {
          stdout->_IO_write_ptr = IO_write_ptr + 1;
          *IO_write_ptr = 9;
        }
        v8 = dword_17108 / 8 + 1;
        v1 = 8 * v8;
        dword_17108 = 8 * v8;
      }
      while ( v6 > v8 );
    }
  }
  if ( v2 > v1 )
  {
    do
    {
      v9 = stdout->_IO_write_ptr;
      if ( v9 >= stdout->_IO_write_end )
      {
        __overflow(stdout, 32);
      }
      else
      {
        stdout->_IO_write_ptr = v9 + 1;
        *v9 = 32;
      }
      ++dword_17108;
    }
    while ( dword_17108 < v2 );
  }
}



// Function: put_line @ 0x3140
int put_line(long long a1, int a2)
{
  unsigned char **v2; // rbp
  int v3; // ebx
  long long v4; // rbx
  unsigned char **v5; // rdi
  char *IO_write_ptr; // rax

  v2 = (unsigned char **)a1;
  dword_17108 = 0;
  sub_3060(dword_C0F4);
  fputs_unlocked(s, stdout);
  v3 = a2 - (dword_17108 + dword_17114);
  dword_17108 += dword_17114;
  sub_3060(v3);
  v4 = *(long long *)(a1 + 32) - 40LL;
  if ( a1 != v4 )
  {
    do
    {
      v5 = v2;
      v2 += 5;
      sub_3000(v5);
      sub_3060(*((int *)v2 - 7));
    }
    while ( (unsigned char **)v4 != v2 );
  }
  sub_3000(v2);
  dword_C0E0 = dword_17108;
  IO_write_ptr = stdout->_IO_write_ptr;
  if ( IO_write_ptr >= stdout->_IO_write_end )
  {
    LODWORD(IO_write_ptr) = __overflow(stdout, 10);
  }
  else
  {
    stdout->_IO_write_ptr = IO_write_ptr + 1;
    *IO_write_ptr = 10;
  }
  return (int)IO_write_ptr;
}



// Function: flush_paragraph @ 0x3200
size_t flush_paragraph()
{
  long long v0; // r12
  long long v1; // rdx
  long long v2; // rsi
  long long *v3; // rbx
  long long v4; // rax
  long long *v5; // rdi
  long long v6; // rcx
  long long v7; // rcx
  long long i; // r12
  long long v9; // r12
  int v10; // eax
  long long v11; // rdx
  long long v12; // rax
  long long v13; // r12
  long long v14; // rcx
  long long *v15; // rax
  unsigned long long v16; // r12
  long long v17; // rcx
  size_t result; // rax

  v0 = qword_C100;
  if ( (_UNKNOWN *)qword_C100 == &unk_C120 )
  {
    result = fwrite_unlocked(&unk_15D80, 1u, qword_15D60 - (long long)&unk_15D80, stdout);
    qword_15D60 = (long long)&unk_15D80;
  }
  else
  {
    sub_2CB0();
    v1 = qword_C140;
    if ( v0 == qword_C140 )
    {
      v3 = (long long *)v0;
    }
    else
    {
      v2 = *(long long *)(qword_C140 + 24);
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
    sub_3140((long long)&unk_C120, dword_C0F0);
    for ( i = qword_C140; v3 != (long long *)i; i = *(long long *)(i + 32) )
      sub_3140(i, dword_C0EC);
    v9 = qword_15D60;
    v10 = __memmove_chk(&unk_15D80, *v3, qword_15D60 - *v3, 5000);
    v11 = *v3;
    v12 = (int)(*v3 - v10);
    v13 = v9 - v12;
    v14 = v12;
    v15 = v3;
    qword_15D60 = v13;
    v16 = qword_C100;
    v17 = -v14;
    if ( qword_C100 >= (unsigned long long)v3 )
    {
      while ( 1 )
      {
        v15 += 5;
        *(v15 - 5) = v17 + v11;
        if ( v16 < (unsigned long long)v15 )
          break;
        v11 = *v15;
      }
    }
    result = __memmove_chk(&unk_C120, v3, v16 - (long long)v3 + 40, 40000);
    qword_C100 = v16 - ((char *)v3 - (char *)&unk_C120);
  }
  return result;
}



// Function: get_line @ 0x3420
int get_line(_IO_FILE *a1, char a2)
{
  int v2; // ebp
  const unsigned short **v3; // rax
  long long *v4; // rdx
  int *v5; // rax
  long long v6; // rdx
  char *IO_read_ptr; // rax
  long long v8; // r13
  int v9; // ecx
  unsigned char *v10; // rbx
  int v11; // eax
  int v12; // esi
  char *v13; // r14
  int v14; // eax
  long long v15; // rdx
  int v16; // eax
  char v17; // cl
  bool v18; // al
  int v20; // r15d
  int v21; // [rsp+14h] [rbp-44h]
  const unsigned short **v22; // [rsp+18h] [rbp-40h]

  LOBYTE(v2) = a2;
  v3 = __ctype_b_loc();
  v4 = (long long *)qword_C100;
  v22 = v3;
  do
  {
    while ( 2 )
    {
      v5 = (int *)qword_15D60;
      *v4 = qword_15D60;
LABEL_3:
      if ( v5 == &dword_17108 )
        goto LABEL_22;
      while ( 1 )
      {
        v6 = (long long)v5 + 1;
        *(char *)v5 = v2;
        IO_read_ptr = a1->_IO_read_ptr;
        qword_15D60 = v6;
        if ( IO_read_ptr < a1->_IO_read_end )
        {
          a1->_IO_read_ptr = IO_read_ptr + 1;
          v2 = (unsigned char)*IO_read_ptr;
          v5 = (int *)v6;
          goto LABEL_6;
        }
        v2 = __uflow(a1);
        v5 = (int *)qword_15D60;
        if ( v2 == -1 )
          break;
LABEL_6:
        if ( v2 <= 13 )
        {
          if ( v2 > 8 )
            break;
          goto LABEL_3;
        }
        if ( v2 == 32 )
          break;
        if ( v5 == &dword_17108 )
        {
LABEL_22:
          sub_3390(1);
          sub_3200();
          v5 = (int *)qword_15D60;
        }
      }
      v8 = qword_C100;
      v9 = dword_1710C;
      v10 = *(unsigned char **)qword_C100;
      v11 = (int)v5 - *(long long *)qword_C100;
      *(int *)(qword_C100 + 8) = v11;
      v12 = (char)*v10;
      v21 = v11 + v9;
      v13 = (char *)&v10[v11 - 1];
      dword_1710C = v11 + v9;
      *(char *)(v8 + 16) = *(char *)(v8 + 16) & 0xFA
                          | (strchr("(['`\"", v12) != 0)
                          | (*v22)[(unsigned char)*v13] & 4;
      if ( v10 < (unsigned char *)v13 )
      {
        while ( 1 )
        {
          v20 = *v13;
          if ( !strchr(")]'\"", v20) )
            break;
          if ( v10 == (unsigned char *)(v13 - 1) )
          {
            v20 = *(v13 - 1);
            break;
          }
          --v13;
        }
      }
      else
      {
        v20 = *v13;
      }
      *(char *)(v8 + 16) = (2 * (strchr(".?!", v20) != 0)) | *(char *)(v8 + 16) & 0xFD;
      v14 = sub_2EA0(a1, v2);
      v15 = qword_C100;
      v2 = v14;
      v16 = dword_1710C - v21;
      *(int *)(qword_C100 + 12) = dword_1710C - v21;
      if ( v2 == -1 )
      {
        *(char *)(v15 + 16) |= 8u;
LABEL_24:
        *(int *)(v15 + 12) = ((*(char *)(v15 + 16) & 8) != 0) + 1;
        if ( (_UNKNOWN *)v15 == &unk_15D10 )
          break;
        goto LABEL_25;
      }
      v17 = *(char *)(v15 + 16);
      if ( (v17 & 2) != 0 )
      {
        v18 = v2 == 10 || v16 > 1;
        if ( v18 )
          goto LABEL_16;
        *(char *)(v15 + 16) = v17 & 0xF7;
        goto LABEL_17;
      }
      v18 = 0;
LABEL_16:
      *(char *)(v15 + 16) = (8 * v18) | *(char *)(v15 + 16) & 0xF7;
      if ( v2 == 10 )
        goto LABEL_24;
LABEL_17:
      if ( byte_17130 )
        goto LABEL_24;
      if ( (_UNKNOWN *)v15 != &unk_15D10 )
      {
        v4 = (long long *)(v15 + 40);
        qword_C100 = (long long)v4;
        continue;
      }
      break;
    }
    sub_3390(1);
    sub_3200();
    v15 = qword_C100;
LABEL_25:
    v4 = (long long *)(v15 + 40);
    qword_C100 = (long long)v4;
  }
  while ( v2 != 10 && v2 != -1 );
  return sub_2F30(a1);
}



// Function: fmt @ 0x3700
long long fmt(FILE *stream, long long a2)
{
  int v4; // ebp
  bool v5; // r13
  int v6; // eax
  int v7; // r8d
  char *v8; // r15
  int v9; // esi
  char *IO_write_ptr; // rax
  long long v11; // r12
  char *v12; // rdx
  long long result; // rax
  char *v14; // rax
  bool v15; // al
  char *v16; // rax
  char *IO_read_ptr; // rax
  char *v18; // rax
  long long v19; // rdi
  int v20; // esi
  int v21; // ecx
  int v22; // r8d
  long long v23; // r15
  long long i; // rbp
  int v25; // eax
  int v26; // ebp

  sub_41C0(stream, 2);
  byte_C0F8 = 0;
  dword_C0EC = 0;
  dword_C0E8 = sub_2F30(stream);
  v4 = dword_C0E8;
LABEL_2:
  dword_C0E0 = 0;
  while ( v4 != 10 && v4 != -1 )
  {
    if ( dword_17118 <= dword_C0E4 && dword_C0E4 + dword_1711C <= dword_1710C )
    {
      dword_C0F4 = dword_C0E4;
      dword_C0F0 = dword_1710C;
      qword_15D60 = (long long)&unk_15D80;
      qword_C100 = (long long)&unk_C120;
      v19 = 0;
      v20 = sub_3420(stream, v4);
      if ( dword_C0E4 == dword_C0F4 && dword_C0E4 + dword_1711C <= dword_1710C )
        v19 = (v20 != 10) & (unsigned char)(v20 != -1);
      sub_3390(v19);
      if ( !byte_17131 )
      {
        if ( byte_17133 )
        {
          if ( v21 == v22 && dword_1711C + v21 <= dword_1710C && v20 != 10 && v20 != -1 )
          {
            do
              v20 = sub_3420(stream, v20);
            while ( dword_C0E4 == dword_C0F4
                 && dword_1711C + dword_C0E4 <= dword_1710C
                 && v20 != 10
                 && v20 != -1
                 && dword_C0EC == dword_1710C );
          }
        }
        else if ( byte_17132 )
        {
          if ( v21 == v22 && dword_1711C + v21 <= dword_1710C && v20 != 10 && v20 != -1 && dword_C0F0 != dword_1710C )
          {
            do
              v20 = sub_3420(stream, v20);
            while ( dword_C0E4 == dword_C0F4
                 && dword_1711C + dword_C0E4 <= dword_1710C
                 && v20 != 10
                 && v20 != -1
                 && dword_C0EC == dword_1710C );
          }
        }
        else if ( v21 == v22 )
        {
          do
          {
            if ( dword_1711C + v21 > dword_1710C )
              break;
            if ( v20 == 10 )
              break;
            if ( v20 == -1 )
              break;
            if ( dword_C0EC != dword_1710C )
              break;
            v25 = sub_3420(stream, v20);
            v21 = dword_C0E4;
            v20 = v25;
          }
          while ( dword_C0E4 == dword_C0F4 );
        }
      }
      v23 = qword_C100;
      if ( qword_C100 <= (unsigned long long)&unk_C120 )
        __assert_fail("word < word_limit", "src/fmt.c", 0x270u, "get_paragraph");
      *(char *)(qword_C100 - 24) |= 0xAu;
      dword_C0E8 = v20;
      sub_2CB0();
      sub_3140((long long)&unk_C120, dword_C0F0);
      for ( i = qword_C140; i != v23; i = *(long long *)(i + 32) )
        sub_3140(i, dword_C0EC);
      v4 = dword_C0E8;
      goto LABEL_2;
    }
    dword_17108 = 0;
    if ( dword_1710C > dword_C0E4 )
    {
      v5 = 1;
LABEL_9:
      sub_3060(dword_C0E4);
      v6 = dword_17108;
      v7 = dword_1710C;
      v8 = s;
      if ( dword_1710C == dword_17108 )
        goto LABEL_16;
      goto LABEL_13;
    }
    sub_3060(dword_C0E4);
    v6 = dword_17108;
    v7 = dword_1710C;
    v8 = s;
    if ( dword_17108 == dword_1710C )
    {
      sub_3060(0);
      goto LABEL_39;
    }
LABEL_33:
    v5 = 1;
    do
    {
LABEL_13:
      v9 = (unsigned char)*v8;
      if ( !(char)v9 )
        break;
      ++v8;
      IO_write_ptr = stdout->_IO_write_ptr;
      if ( IO_write_ptr < stdout->_IO_write_end )
      {
        stdout->_IO_write_ptr = IO_write_ptr + 1;
        *IO_write_ptr = v9;
      }
      else
      {
        __overflow(stdout, v9);
        v7 = dword_1710C;
      }
      v6 = dword_17108 + 1;
      dword_17108 = v6;
    }
    while ( v6 != v7 );
LABEL_16:
    if ( v5 )
    {
      sub_3060(v7 - v6);
      if ( v4 == -1 )
        goto LABEL_18;
      goto LABEL_39;
    }
    if ( v4 == -1 )
      goto LABEL_18;
LABEL_29:
    v14 = stdout->_IO_write_ptr;
    if ( v14 >= stdout->_IO_write_end )
    {
      __overflow(stdout, 10);
    }
    else
    {
      stdout->_IO_write_ptr = v14 + 1;
      *v14 = 10;
    }
    v4 = sub_2F30(stream);
  }
  dword_17108 = 0;
  v5 = v4 != -1 && v4 != 10;
  if ( dword_1710C > dword_C0E4 )
    goto LABEL_9;
  if ( v4 == -1 || v4 == 10 )
    goto LABEL_28;
  sub_3060(dword_C0E4);
  v6 = dword_17108;
  v7 = dword_1710C;
  v8 = s;
  if ( dword_17108 != dword_1710C )
    goto LABEL_33;
  sub_3060(0);
  if ( v4 != -1 )
  {
    do
    {
LABEL_39:
      v16 = stdout->_IO_write_ptr;
      if ( v16 >= stdout->_IO_write_end )
      {
        __overflow(stdout, (unsigned char)v4);
      }
      else
      {
        stdout->_IO_write_ptr = v16 + 1;
        *v16 = v4;
      }
      IO_read_ptr = stream->_IO_read_ptr;
      if ( IO_read_ptr < stream->_IO_read_end )
      {
        stream->_IO_read_ptr = IO_read_ptr + 1;
        v4 = (unsigned char)*IO_read_ptr;
        v15 = 1;
      }
      else
      {
        v4 = __uflow(stream);
        v15 = v4 != -1;
      }
    }
    while ( v4 != 10 && v15 );
LABEL_28:
    if ( v4 == -1 )
      goto LABEL_19;
    goto LABEL_29;
  }
LABEL_18:
  if ( dword_C0E4 + dword_17114 <= dword_1710C )
  {
    v18 = stdout->_IO_write_ptr;
    if ( v18 >= stdout->_IO_write_end )
    {
      __overflow(stdout, 10);
    }
    else
    {
      stdout->_IO_write_ptr = v18 + 1;
      *v18 = 10;
    }
  }
LABEL_19:
  dword_C0E8 = -1;
  if ( (stream->_flags & 0x20) != 0 )
  {
    if ( stream == stdin )
      clearerr_unlocked(stream);
    else
      sub_41F0(stream);
    v11 = sub_63B0(0, 3, a2);
LABEL_23:
    v12 = dcgettext(0, "read error", 5);
    error(0, 0, v12, v11);
    return 0;
  }
  if ( stream == stdin )
  {
    clearerr_unlocked(stream);
    return 1;
  }
  else if ( (unsigned int)sub_41F0(stream) )
  {
    v26 = *__errno_location();
    result = 1;
    if ( v26 >= 0 )
    {
      v11 = sub_63B0(0, 3, a2);
      if ( !v26 )
        goto LABEL_23;
      error(0, v26, "%s", v11);
      return 0;
    }
  }
  else
  {
    return 1;
  }
  return result;
}



// Function: usage @ 0x3d70
void usage(int status)
{
  long long v2; // r13
  char *v3; // rax
  long long *v4; // rbx
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
  const char *v20; // rsi
  const char *v21; // r14
  char *v22; // rax
  char *v23; // rdi
  char *v24; // rax
  char *v25; // rax
  FILE *v26; // rbx
  char *v27; // rax
  long long v28[23]; // [rsp+0h] [rbp-B8h] BYREF

  v2 = qword_17148;
  v28[15] = __readfsqword(0x28u);
  if ( status )
  {
    v3 = dcgettext(0, "Try '%s --help' for more information.\n", 5);
    __fprintf_chk(stderr, 1, v3, v2);
  }
  else
  {
    v4 = v28;
    v5 = dcgettext(0, "Usage: %s [-WIDTH] [OPTION]... [FILE]...\n", 5);
    __printf_chk(1, v5, v2);
    v6 = stdout;
    v7 = dcgettext(
           0,
           "Reformat each paragraph in the FILE(s), writing to standard output.\n"
           "The option -WIDTH is an abbreviated form of --width=DIGITS.\n",
           5);
    fputs_unlocked(v7, v6);
    v8 = stdout;
    v9 = dcgettext(0, "\nWith no FILE, or when FILE is -, read standard input.\n", 5);
    fputs_unlocked(v9, v8);
    v10 = stdout;
    v11 = dcgettext(0, "\nMandatory arguments to long options are mandatory for short options too.\n", 5);
    fputs_unlocked(v11, v10);
    v12 = stdout;
    v13 = dcgettext(
            0,
            "  -c, --crown-margin        preserve indentation of first two lines\n"
            "  -p, --prefix=STRING       reformat only lines beginning with STRING,\n"
            "                              reattaching the prefix to reformatted lines\n"
            "  -s, --split-only          split long lines, but do not refill\n",
            5);
    fputs_unlocked(v13, v12);
    v14 = stdout;
    v15 = dcgettext(
            0,
            "  -t, --tagged-paragraph    indentation of first line different from second\n"
            "  -u, --uniform-spacing     one space between words, two after sentences\n"
            "  -w, --width=WIDTH         maximum line width (default of 75 columns)\n"
            "  -g, --goal=WIDTH          goal width (default of 93% of width)\n",
            5);
    fputs_unlocked(v15, v14);
    v16 = stdout;
    v17 = dcgettext(0, "      --help        display this help and exit\n", 5);
    fputs_unlocked(v17, v16);
    v18 = stdout;
    v19 = dcgettext(0, "      --version     output version information and exit\n", 5);
    fputs_unlocked(v19, v18);
    v28[0] = "[";
    v20 = "[";
    v28[1] = "test invocation";
    v28[2] = "coreutils";
    v28[3] = "Multi-call invocation";
    v28[6] = "sha256sum";
    v28[4] = "sha224sum";
    v28[8] = "sha384sum";
    v28[5] = "sha2 utilities";
    v28[7] = "sha2 utilities";
    v28[9] = "sha2 utilities";
    v28[10] = "sha512sum";
    v28[11] = "sha2 utilities";
    v28[12] = 0;
    v28[13] = 0;
    do
    {
      if ( !strcmp("fmt", v20) )
        break;
      v20 = (const char *)v4[2];
      v4 += 2;
    }
    while ( v20 );
    v21 = (const char *)v4[1];
    if ( !v21 )
      v21 = "fmt";
    v22 = dcgettext(0, "\n%s online help: <%s>\n", 5);
    __printf_chk(1, v22, "GNU coreutils");
    v23 = setlocale(5, 0);
    if ( v23 )
    {
      if ( strncmp(v23, "en_", 3u) )
      {
        v26 = stdout;
        v27 = dcgettext(0, "Report any translation bugs to <https://translationproject.org/team/>\n", 5);
        fputs_unlocked(v27, v26);
      }
    }
    strcmp("fmt", "[");
    v24 = dcgettext(0, "Full documentation <%s%s>\n", 5);
    __printf_chk(1, v24, "https://www.gnu.org/software/coreutils/");
    v25 = dcgettext(0, "or available locally via: info '(coreutils) %s%s'\n", 5);
    __printf_chk(1, v25, v21);
  }
  exit(status);
}


