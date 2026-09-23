// Function: main @ 0x39c0
void main(int a1, char **a2, char **a3)
{
  char *v4; // rdi
  long long v5; // r12
  size_t v6; // rax
  const char *v7; // r12
  char **v8; // r15
  const char *v9; // rdi
  FILE *v10; // rdi
  unsigned long long v11; // rdi
  const char *v12; // rsi
  int v13; // eax
  long long v14; // rcx
  int v15; // eax
  int v16; // eax
  long long v17; // rdx
  FILE *v18; // rdi
  const char *v19; // rdx
  char *v20; // rbp
  char v21; // al
  char *v22; // rdx
  FILE *v23; // rdi
  const char *v24; // rdx
  size_t v25; // rax
  int v26; // r13d
  int v27; // eax
  char *v28; // rdi
  FILE *v29; // rdi
  const char *v30; // rdx
  const char *v31; // rcx
  char v32; // [rsp+0h] [rbp-68h]
  char v33; // [rsp+0h] [rbp-68h]
  char v34; // [rsp+0h] [rbp-68h]
  int v35; // [rsp+4h] [rbp-64h]
  char *v36; // [rsp+8h] [rbp-60h]
  int argc; // [rsp+18h] [rbp-50h] BYREF
  int longind; // [rsp+1Ch] [rbp-4Ch] BYREF
  char **argv[9]; // [rsp+20h] [rbp-48h] BYREF

  v35 = a1;
  v4 = *a2;
  argv[1] = (char **)__readfsqword(0x28u);
  qword_1B850 = sub_D3B0(v4);
  v5 = qword_1B850;
  v6 = strlen((const char *)qword_1B850);
  if ( v6 > 4 )
  {
    v7 = (const char *)(v5 + v6 - 4);
    if ( !strcmp(v7, ".exe") )
      *v7 = 0;
  }
  argv[0] = a2;
  v8 = (char **)sub_D3D0(&argc, argv, "GZIP");
  ptr = v8;
  if ( v8 )
    v8 = argv[0];
  src = ".gz";
  qword_DD9C0 = strlen(".gz");
LABEL_7:
  longind = -1;
  if ( v8 )
  {
LABEL_8:
    v9 = v8[optind];
    if ( v9 && !strcmp(v9, "--") )
      goto LABEL_10;
    v11 = (unsigned int)argc;
    v12 = (const char *)v8;
    v13 = getopt_long(argc, v8, "ab:cdfhH?klLmMnNqrS:tvVZ123456789", &longopts, &longind);
    LODWORD(v14) = v13 + 131;
    if ( v13 >= 0 )
    {
      v15 = v13 + 82;
      goto LABEL_13;
    }
    if ( optind != argc )
    {
      v29 = stderr;
      sub_F630(stderr, v32);
      sub_7040(v29, "%s: %s: non-option in GZIP environment variable\n", v30, v31);
    }
    if ( optind != 1 && !dword_1B85C )
      sub_F630(stderr, v32);
    free(v8);
    longind = -1;
    optind = 1;
  }
  while ( 2 )
  {
    v11 = (unsigned int)v35;
    v12 = (const char *)a2;
    v16 = getopt_long(v35, a2, "ab:cdfhH?klLmMnNqrS:tvVZ123456789", &longopts, &longind);
    v14 = (unsigned int)v16;
    if ( v16 >= 0 )
    {
      v15 = v16 - 49;
      v8 = 0;
      if ( (unsigned int)(v14 - 49) > 0xD3 )
      {
LABEL_16:
        if ( (int)v14 <= 130 || (int)v14 == 194 )
          sub_7040(v11, v12);
LABEL_10:
        sub_F630(stderr, v32);
        sub_F630(stderr, v33);
        v10 = stderr;
        sub_F630(stderr, v34);
        sub_7040(v10, "option not valid in GZIP environment variable\n");
      }
LABEL_13:
      switch ( v15 )
      {
        case 0:
        case 1:
        case 2:
        case 3:
        case 4:
        case 5:
        case 6:
        case 7:
        case 8:
          goto LABEL_25;
        case 23:
        case 55:
          sub_5490();
          sub_7010();
        case 27:
          sub_54E0();
          sub_7010();
        case 28:
          dword_19028 = 0;
          longind = -1;
          if ( !v8 )
            continue;
          goto LABEL_8;
        case 29:
        case 160:
          dword_19028 = 0;
          dword_1902C = 0;
          goto LABEL_7;
        case 34:
          v36 = optarg;
          v25 = strlen(optarg);
          longind = -1;
          qword_DD9C0 = v25;
          src = v36;
          if ( !v8 )
            continue;
          goto LABEL_8;
        case 37:
          sub_5540();
          sub_7010();
        case 41:
          v18 = stderr;
          sub_F630(stderr, v32);
          sub_7040(v18, "%s: -Z not supported in this version\n", v19);
        case 48:
          dword_DD9FC = 1;
          longind = -1;
          if ( !v8 )
            continue;
          goto LABEL_8;
        case 49:
          v20 = optarg;
          dword_19024 = atoi(optarg);
          v21 = *v20;
          if ( *v20 )
          {
            v22 = v20 + 1;
            do
            {
              if ( (unsigned char)(v21 - 48) > 9u )
              {
                v23 = stderr;
                sub_F630(stderr, v32);
                sub_7040(v23, "%s: -b operand is not an integer\n", v24);
              }
              optarg = v22;
              v21 = *v22++;
            }
            while ( v21 );
          }
          goto LABEL_7;
        case 50:
          dword_1B860 = 1;
          longind = -1;
          if ( !v8 )
            continue;
          goto LABEL_8;
        case 51:
          dword_DD9F8 = 1;
          longind = -1;
          if ( !v8 )
            continue;
          goto LABEL_8;
        case 53:
          ++dword_DD9F4;
          longind = -1;
          if ( !v8 )
            continue;
          goto LABEL_8;
        case 58:
          dword_DD9F0 = 1;
          longind = -1;
          if ( !v8 )
            continue;
          goto LABEL_8;
        case 59:
          dword_1B860 = 1;
          dword_1B858 = 1;
          dword_DD9F8 = 1;
          dword_DD9E8 = 1;
          longind = -1;
          if ( !v8 )
            continue;
          goto LABEL_8;
        case 60:
          dword_19028 = 1;
          longind = -1;
          if ( !v8 )
            continue;
          goto LABEL_8;
        case 61:
        case 192:
          dword_19028 = 1;
          dword_1902C = 1;
          goto LABEL_7;
        case 64:
        case 195:
          dword_1B85C = 1;
          dword_DD9E4 = 0;
          goto LABEL_7;
        case 65:
          dword_DD9EC = 1;
          longind = -1;
          if ( !v8 )
            continue;
          goto LABEL_8;
        case 67:
          dword_1B860 = 1;
          dword_DD9F8 = 1;
          dword_1B858 = 1;
          longind = -1;
          if ( !v8 )
            continue;
          goto LABEL_8;
        case 69:
        case 200:
          ++dword_DD9E4;
          dword_1B85C = 0;
          goto LABEL_7;
        case 79:
          byte_DDA01 = 1;
          longind = -1;
          if ( !v8 )
            continue;
          goto LABEL_8;
        case 80:
        case 211:
          dword_1B000 = 1;
          goto LABEL_7;
        case 81:
          byte_DDA00 = 1;
          longind = -1;
          if ( !v8 )
            continue;
          goto LABEL_8;
        case 131:
        case 132:
        case 133:
        case 134:
        case 135:
        case 136:
        case 137:
        case 138:
        case 139:
          LODWORD(v14) = v14 - 131;
LABEL_25:
          dword_1901C = v14 - 48;
          goto LABEL_7;
        default:
          goto LABEL_16;
      }
    }
    break;
  }
  if ( dword_19028 < 0 )
    dword_19028 = dword_DD9F8;
  if ( dword_1902C < 0 )
    dword_1902C = dword_DD9F8;
  v26 = optind;
  if ( dword_DD9FC )
  {
    if ( dword_1B85C )
    {
      if ( (unsigned long long)(qword_DD9C0 - 1) <= 0x1D )
        goto LABEL_73;
      goto LABEL_94;
    }
    v11 = (unsigned long long)stderr;
    v12 = "%s: option --ascii ignored on this system\n";
    sub_F630(stderr, v32);
  }
  if ( (unsigned long long)(qword_DD9C0 - 1) <= 0x1D )
  {
    v27 = dword_1B85C;
    if ( !dword_1B85C )
    {
LABEL_74:
      dword_DD920 = v27;
      sub_5570(v11, v12, v17, v14);
      if ( v35 == v26 )
      {
        sub_7070();
      }
      else
      {
        while ( optind < v35 )
        {
          v28 = a2[optind++];
          sub_7B60(v28);
        }
      }
      if ( byte_DD510 && close(0) )
      {
        strcpy(dword_1B420, "stdin");
        sub_D600();
      }
      if ( !dword_DD9E8 )
        goto LABEL_88;
      if ( !dword_1B85C && v35 - v26 > 1 )
        sub_6C40(0xFFFFFFFFLL);
      if ( !(unsigned int)sub_E460(stdout) )
      {
LABEL_88:
        if ( !dword_1B860
          || (!byte_DDA00 || !fdatasync(1) || *__errno_location() == 22) && !close(1)
          || *__errno_location() == 9 )
        {
          sub_6FD0(status);
        }
      }
      sub_D670();
    }
LABEL_73:
    v27 = 13;
    goto LABEL_74;
  }
LABEL_94:
  sub_F630(stderr, v32);
  sub_6FD0(1);
}



// Function: bi_init @ 0x4260
long long (*bi_init(int a1))(void *buf)
{
  long long (*result)(void *); // rax

  dword_1A058 = 0;
  result = 0;
  word_1A05C = 0;
  if ( a1 != -1 )
  {
    qword_1A050 = (long long)sub_E010;
    return sub_E010;
  }
  return result;
}



// Function: send_bits @ 0x4290
short send_bits(int a1, int a2)
{
  int v2; // edx
  int v4; // eax
  int v5; // ecx
  int v6; // esi
  long long v7; // rcx
  long long v8; // rsi

  v2 = dword_1A058;
  LOWORD(v4) = word_1A05C | (a1 << dword_1A058);
  v5 = 16 - a2;
  v6 = dword_1A058 + a2;
  if ( v5 < dword_1A058 )
  {
    v7 = (unsigned int)dword_1B004;
    word_1A05C |= a1 << dword_1A058;
    v8 = (unsigned int)(dword_1B004 + 1);
    if ( dword_1B004 > (unsigned int)&unk_3FFFD )
    {
      ++dword_1B004;
      *((char *)&word_5C000 + v7) = v4;
      if ( (int)v8 == (int)&unk_40000 )
      {
        sub_D7A0();
        BYTE1(v4) = HIBYTE(word_1A05C);
        v8 = (unsigned int)dword_1B004;
      }
      dword_1B004 = v8 + 1;
      *((char *)&word_5C000 + v8) = BYTE1(v4);
      if ( (int)v8 + 1 == (int)&unk_40000 )
        sub_D7A0();
      v2 = dword_1A058;
    }
    else
    {
      *((char *)&word_5C000 + (unsigned int)dword_1B004) = v4;
      dword_1B004 = v7 + 2;
      *((char *)&word_5C000 + v8) = BYTE1(v4);
    }
    v6 = v2 + a2 - 16;
    v4 = (int)(unsigned short)a1 >> (16 - v2);
  }
  word_1A05C = v4;
  dword_1A058 = v6;
  return v4;
}



// Function: bi_reverse @ 0x4380
long long bi_reverse(unsigned int a1, int a2)
{
  int v2; // edx
  char v3; // al
  int v4; // eax

  v2 = 0;
  do
  {
    v3 = a1;
    --a2;
    a1 >>= 1;
    v4 = v2 | v3 & 1;
    v2 = 2 * v4;
  }
  while ( a2 > 0 );
  return v4 & 0x7FFFFFFF;
}



// Function: bi_windup @ 0x43b0
long long bi_windup()
{
  long long v0; // rax
  char v1; // ch
  long long v2; // rdx
  long long v4; // rax
  int v5; // edx

  if ( dword_1A058 <= 8 )
  {
    if ( dword_1A058 > 0 )
    {
      v4 = (unsigned int)dword_1B004++;
      v5 = dword_1B004;
      *((char *)&word_5C000 + v4) = word_1A05C;
      if ( v5 == (int)&unk_40000 )
        goto LABEL_8;
    }
  }
  else
  {
    v0 = (unsigned int)dword_1B004;
    v1 = HIBYTE(word_1A05C);
    v2 = (unsigned int)(dword_1B004 + 1);
    if ( dword_1B004 <= (unsigned int)&unk_3FFFD )
    {
      *((char *)&word_5C000 + (unsigned int)dword_1B004) = word_1A05C;
      dword_1B004 = v0 + 2;
      *((char *)&word_5C000 + v2) = v1;
      goto LABEL_4;
    }
    ++dword_1B004;
    *((char *)&word_5C000 + v0) = word_1A05C;
    if ( (int)v2 == (int)&unk_40000 )
    {
      sub_D7A0();
      v1 = HIBYTE(word_1A05C);
      v2 = (unsigned int)dword_1B004;
    }
    *((char *)&word_5C000 + v2) = v1;
    dword_1B004 = v2 + 1;
    if ( (int)v2 + 1 == (int)&unk_40000 )
LABEL_8:
      sub_D7A0();
  }
LABEL_4:
  dword_1A058 = 0;
  word_1A05C = 0;
  return 0;
}



// Function: copy_block @ 0x44a0
long long copy_block(char *a1, int a2, int a3)
{
  long long result; // rax
  long long v7; // rax
  long long v8; // rdx
  long long v9; // rax
  long long v10; // rdx
  char v11; // si
  short v12; // cx
  long long v13; // rdi
  char *v14; // r12
  char v15; // cl
  int v16; // edx

  result = sub_43B0();
  if ( a3 )
  {
    v7 = (unsigned int)dword_1B004;
    v8 = (unsigned int)(dword_1B004 + 1);
    if ( dword_1B004 > (unsigned int)&unk_3FFFD )
    {
      ++dword_1B004;
      *((char *)&word_5C000 + v7) = a2;
      if ( (int)v8 == (int)&unk_40000 )
      {
        sub_D7A0();
        v8 = (unsigned int)dword_1B004;
      }
      v9 = (unsigned int)(v8 + 1);
      dword_1B004 = v8 + 1;
      *((char *)&word_5C000 + v8) = BYTE1(a2);
      if ( (int)v9 == (int)&unk_40000 )
      {
        sub_D7A0();
        v9 = (unsigned int)dword_1B004;
      }
    }
    else
    {
      v9 = (unsigned int)(dword_1B004 + 2);
      *((char *)&word_5C000 + (unsigned int)dword_1B004) = a2;
      *((char *)&word_5C000 + v8) = BYTE1(a2);
    }
    v10 = (unsigned int)(v9 + 1);
    v11 = ~(char)a2;
    v12 = (unsigned short)~(short)a2 >> 8;
    if ( (unsigned int)v9 > (unsigned int)&unk_3FFFD )
    {
      dword_1B004 = v9 + 1;
      *((char *)&word_5C000 + v9) = v11;
      if ( (int)v10 == (int)&unk_40000 )
      {
        sub_D7A0();
        v10 = (unsigned int)dword_1B004;
        LOBYTE(v12) = (unsigned short)~(short)a2 >> 8;
      }
      result = (unsigned int)(v10 + 1);
      dword_1B004 = v10 + 1;
      *((char *)&word_5C000 + v10) = v12;
      if ( (int)result == (int)&unk_40000 )
        result = sub_D7A0();
    }
    else
    {
      v13 = (unsigned int)v9;
      result = (unsigned int)(v9 + 2);
      *((char *)&word_5C000 + v13) = v11;
      dword_1B004 = result;
      *((char *)&word_5C000 + v10) = v12;
    }
  }
  v14 = &a1[a2];
  if ( a2 )
  {
    do
    {
      while ( 1 )
      {
        result = (unsigned int)dword_1B004;
        v15 = *a1++;
        v16 = ++dword_1B004;
        *((char *)&word_5C000 + result) = v15;
        if ( v16 == (int)&unk_40000 )
          break;
        if ( a1 == v14 )
          return result;
      }
      result = sub_D7A0();
    }
    while ( a1 != v14 );
  }
  return result;
}



// Function: longest_match @ 0x4620
long long longest_match(unsigned int a1)
{
  int v2; // r8d
  int v3; // edi
  unsigned int v4; // ecx
  long long v5; // rsi
  unsigned int v6; // edi
  char *v7; // rbx
  char v8; // bp
  char v9; // r10
  char v10; // r13
  char *v11; // rax
  char *v12; // rsi
  char *v13; // rax
  int v14; // esi
  int v16; // [rsp+0h] [rbp-34h]

  v2 = dword_1A070;
  v3 = dword_1A06C;
  v4 = dword_1A064;
  if ( (unsigned int)dword_1A06C < 0x7EFA )
    v3 = 32506;
  v5 = dword_1A070;
  v6 = v3 - 32506;
  v7 = &byte_3C000[dword_1A06C];
  v8 = v7[dword_1A070 - 1];
  v9 = v7[dword_1A070];
  if ( dword_1A070 >= (unsigned int)dword_1A060 )
    v4 = (unsigned int)dword_1A064 >> 2;
  v10 = 0;
  while ( 1 )
  {
    v11 = &byte_3C000[a1];
    if ( v11[v5] != v9 || v11[v5 - 1] != v8 || *v11 != *v7 || v11[1] != v7[1] )
      goto LABEL_6;
    v12 = v7 + 2;
    v13 = v11 + 2;
    while ( 1 )
    {
      if ( v12[1] != v13[1] )
      {
        LODWORD(v12) = (int)v12 + 1;
        goto LABEL_24;
      }
      if ( v12[2] != v13[2] )
      {
        LODWORD(v12) = (int)v12 + 2;
        goto LABEL_24;
      }
      if ( v12[3] != v13[3] )
      {
        LODWORD(v12) = (int)v12 + 3;
        goto LABEL_24;
      }
      if ( v12[4] != v13[4] )
      {
        LODWORD(v12) = (int)v12 + 4;
        goto LABEL_24;
      }
      if ( v12[5] != v13[5] )
      {
        LODWORD(v12) = (int)v12 + 5;
        goto LABEL_24;
      }
      if ( v12[6] != v13[6] )
      {
        LODWORD(v12) = (int)v12 + 6;
        goto LABEL_24;
      }
      if ( v12[7] != v13[7] )
        break;
      v12 += 8;
      v13 += 8;
      if ( *v12 != *v13 || &byte_3C000[dword_1A06C + 258] <= v12 )
        goto LABEL_24;
    }
    LODWORD(v12) = (int)v12 + 7;
LABEL_24:
    v14 = (int)v12 - (unsigned int)&byte_3C000[dword_1A06C];
    if ( v2 >= v14 )
    {
LABEL_6:
      a1 = *((unsigned short *)&qword_1B880 + (a1 & 0x7FFF));
      if ( v6 >= a1 )
        break;
      goto LABEL_7;
    }
    if ( dword_1A080 <= v14 )
    {
      dword_1A068 = a1;
      return (unsigned int)v14;
    }
    v16 = a1;
    v2 = v14;
    a1 = *((unsigned short *)&qword_1B880 + (a1 & 0x7FFF));
    v8 = v7[v14 - 1];
    v10 = 1;
    v9 = v7[v14];
    if ( v6 >= a1 )
      break;
LABEL_7:
    if ( !--v4 )
      break;
    v5 = v2;
  }
  if ( !v10 )
    return (unsigned int)v2;
  dword_1A068 = v16;
  return (unsigned int)v2;
}



// Function: rsync_roll @ 0x4840
long long rsync_roll(unsigned int a1, int a2)
{
  long long result; // rax
  unsigned int v3; // r9d
  long long v4; // rdi
  char *v5; // rdx
  char *v6; // rcx
  char *v7; // rsi
  long long v8; // r8
  char *v9; // rsi
  long long v10; // rax
  long long v11; // rsi
  long long v12; // r8
  char v13; // r11

  result = a1;
  v3 = a1 + a2;
  if ( a1 > 0xFFF )
  {
LABEL_6:
    if ( (unsigned int)result < v3 )
    {
      v11 = qword_1A090;
      v12 = qword_1A088;
      v13 = 0;
      do
      {
        v11 += (unsigned char)byte_3C000[result]
             - (unsigned long long)(unsigned char)byte_3C000[(unsigned int)(result - 4096)];
        if ( v12 == 0xFFFFFFFFLL && (v11 & 0xFFF) == 0 )
        {
          v12 = result;
          v13 = 1;
        }
        ++result;
      }
      while ( v3 > (unsigned int)result );
      qword_1A090 = v11;
      if ( v13 )
        qword_1A088 = v12;
    }
  }
  else
  {
    v4 = qword_1A090;
    v5 = &byte_3C000[(unsigned int)result];
    v6 = &v5[a2];
    v7 = &byte_3C001[(unsigned int)result];
    v8 = (unsigned int)(4095 - result);
    result = 0;
    v9 = &v7[v8];
    while ( v6 != v5 )
    {
      v10 = (unsigned char)*v5++;
      v4 += v10;
      result = 1;
      if ( v9 == v5 )
      {
        qword_1A090 = v4;
        result = 4096;
        goto LABEL_6;
      }
    }
    if ( (char)result )
      qword_1A090 = v4;
  }
  return result;
}



// Function: fill_window @ 0x4940
void fill_window()
{
  int v0; // r12d
  long long v1; // rbx
  int v2; // eax
  char *v3; // rax
  short *v4; // rdx
  short v5; // cx
  short v6; // dx

  v0 = dword_1A06C;
  v1 = (unsigned int)dword_1A09C;
  if ( dword_1A09C + dword_1A06C != 65537 && (unsigned int)dword_1A06C > 0xFEF9 )
  {
    memcpy((char *)&unk_44000 - 0x8000, &unk_44000, 0x8000u);
    dword_1A068 -= 0x8000;
    dword_1A06C = v0 - 0x8000;
    if ( qword_1A088 != 0xFFFFFFFFLL )
      qword_1A088 -= 0x8000;
    qword_1A078 -= 0x8000;
    v3 = (char *)&unk_2B880 - 0x10000;
    v4 = (short *)&unk_2B880;
    do
    {
      v5 = *v4 + 0x8000;
      if ( *v4 >= 0 )
        v5 = 0;
      *v4++ = v5;
    }
    while ( (short *)((char *)&unk_20000 + (long long)&unk_2B880 - 0x10000) != v4 );
    do
    {
      v6 = *(short *)v3 + 0x8000;
      if ( *(short *)v3 >= 0 )
        v6 = 0;
      v3 += 2;
      *((short *)v3 - 1) = v6;
    }
    while ( v3 != (char *)&unk_2B880 );
  }
  if ( !dword_1A0A0 )
  {
    v2 = qword_1A050(&byte_3C000[v1 + (unsigned int)dword_1A06C]);
    if ( (unsigned int)(v2 - 1) > 0xFFFFFFFD )
    {
      dword_1A0A0 = 1;
      *(short *)&byte_3C000[(unsigned int)dword_1A09C + (unsigned long long)(unsigned int)dword_1A06C] = 0;
    }
    else
    {
      dword_1A09C += v2;
    }
  }
}



// Function: deflate_fast @ 0x4aa0
long long deflate_fast()
{
  unsigned int v0; // r13d
  long long v1; // rdx
  short *v2; // rax
  unsigned int v3; // edi
  int v4; // eax
  int v5; // r15d
  int v6; // r13d
  int v7; // r14d
  long long v8; // rsi
  long long v9; // rdx
  long long v10; // rsi
  char *v11; // rdi
  int v12; // eax
  long long v13; // rdx
  char *v14; // rdi
  unsigned int v16; // eax
  int v17; // eax
  int v18; // edi
  long long v19; // r13
  int v20; // edx
  int v21; // eax
  int v22; // ecx
  long long v23; // r8
  short *v24; // rcx
  short v25; // r9
  int v26; // [rsp+Ch] [rbp-3Ch]

  v0 = dword_1A09C;
  dword_1A070 = 2;
  if ( dword_1A09C )
  {
    while ( 1 )
    {
LABEL_2:
      v1 = (unsigned int)dword_1A06C;
      dword_1A0A4 = ((unsigned char)byte_3C000[dword_1A06C + 2] ^ (unsigned short)(32 * dword_1A0A4)) & 0x7FFF;
      v2 = (short *)&qword_1B880 + (unsigned int)(dword_1A0A4 + 0x8000);
      v3 = (unsigned short)*v2;
      *((short *)&qword_1B880 + (dword_1A06C & 0x7FFF)) = v3;
      *v2 = v1;
      if ( !v3 || (unsigned int)v1 - v3 > 0x7EFA || (unsigned int)v1 > 0xFEFA )
        goto LABEL_5;
      v16 = sub_4620(v3);
      if ( v16 <= v0 )
        v0 = v16;
      if ( v0 <= 2 )
        break;
      v17 = sub_B290((unsigned int)(dword_1A06C - dword_1A068), v0 - 3);
      v5 = dword_1B000;
      dword_1A09C -= v0;
      v18 = dword_1A06C;
      v7 = v17;
      if ( dword_1B000 )
      {
        v26 = dword_1A06C;
        sub_4840(dword_1A06C, v0);
        v18 = v26;
        if ( dword_1A098 < v0 )
        {
LABEL_30:
          v19 = v0 + v18;
          dword_1A06C = v19;
          dword_1A0A4 = (unsigned char)byte_3C000[(unsigned int)(v19 + 1)] ^ (32 * (unsigned char)byte_3C000[v19]);
          goto LABEL_8;
        }
      }
      else if ( dword_1A098 < v0 )
      {
        goto LABEL_30;
      }
      LOWORD(v20) = dword_1A0A4;
      v21 = v18;
      do
      {
        v22 = v21++;
        v23 = v21 & 0x7FFF;
        v20 = ((unsigned char)byte_3C000[v22 + 3] ^ (unsigned short)(32 * v20)) & 0x7FFF;
        v24 = (short *)&qword_1B880 + (unsigned int)(v20 + 0x8000);
        v25 = *v24;
        *((short *)&qword_1B880 + v23) = *v24;
        *v24 = v21;
      }
      while ( v21 != v0 + v18 - 1 );
      *((short *)&qword_1B880 + v23) = v25;
      dword_1A0A4 = v20;
      *v24 = v18 + v0 - 1;
      dword_1A06C = v0 + v18;
LABEL_8:
      if ( v5 && (v8 = (unsigned int)dword_1A06C, (unsigned int)dword_1A06C > (unsigned long long)qword_1A088) )
      {
        v9 = 1;
        v7 = 2;
        qword_1A088 = 0xFFFFFFFFLL;
      }
      else
      {
        if ( !v7 )
          goto LABEL_14;
        v8 = (unsigned int)dword_1A06C;
        v9 = (unsigned int)(v7 - 1);
      }
      v10 = v8 - qword_1A078;
      v11 = &byte_3C000[(unsigned int)qword_1A078];
      if ( qword_1A078 < 0 )
        v11 = 0;
      sub_B070(v11, v10, v9, 0);
      qword_1A078 = (unsigned int)dword_1A06C;
LABEL_14:
      v12 = dword_1A09C;
      v0 = dword_1A09C;
      if ( (unsigned int)dword_1A09C <= 0x105 )
      {
        while ( !dword_1A0A0 )
        {
          sub_4940();
          v12 = dword_1A09C;
          v0 = dword_1A09C;
          if ( (unsigned int)dword_1A09C > 0x105 )
            goto LABEL_2;
        }
        v0 = v12;
        if ( !v12 )
        {
          v13 = (unsigned int)(v7 - 1);
          goto LABEL_20;
        }
      }
    }
    v1 = (unsigned int)dword_1A06C;
LABEL_5:
    v4 = sub_B290(0, (unsigned char)byte_3C000[v1]);
    v5 = dword_1B000;
    v6 = dword_1A06C;
    v7 = v4;
    if ( dword_1B000 )
      sub_4840(dword_1A06C, 1);
    --dword_1A09C;
    dword_1A06C = v6 + 1;
    goto LABEL_8;
  }
  v13 = 0xFFFFFFFFLL;
LABEL_20:
  v14 = 0;
  if ( qword_1A078 >= 0 )
    v14 = &byte_3C000[(unsigned int)qword_1A078];
  return sub_B070(v14, (unsigned int)dword_1A06C - qword_1A078, v13, 1);
}



// Function: lm_init @ 0x4db0
long long lm_init(int a1)
{
  int v1; // edx
  int v2; // eax
  long long result; // rax

  if ( (unsigned int)(a1 - 1) > 8 )
    sub_D510("bad pack level");
  memset(&unk_2B880, 0, 0x10000u);
  qword_1A090 = 0;
  qword_1A088 = 0xFFFFFFFFLL;
  dword_1A06C = 0;
  v1 = word_12020[4 * a1 + 1];
  qword_1A078 = 0;
  dword_1A098 = v1;
  dword_1A060 = word_12020[4 * a1];
  v2 = word_12020[4 * a1 + 3];
  dword_1A080 = word_12020[4 * a1 + 2];
  dword_1A064 = v2;
  result = qword_1A050(byte_3C000);
  dword_1A09C = result;
  if ( (unsigned int)(result - 1) > 0xFFFFFFFD )
  {
    dword_1A0A0 = 1;
    dword_1A09C = 0;
  }
  else
  {
    dword_1A0A0 = 0;
    if ( (unsigned int)result <= 0x105 )
    {
      do
        sub_4940();
      while ( (unsigned int)dword_1A09C <= 0x105 && !dword_1A0A0 );
    }
    dword_1A0A4 = (unsigned char)byte_3C001[0] ^ (32 * (unsigned char)byte_3C000[0]);
    return (unsigned char)byte_3C001[0] ^ (32 * (unsigned int)(unsigned char)byte_3C000[0]);
  }
  return result;
}



// Function: deflate @ 0x4ed0
long long deflate(int a1)
{
  unsigned int v1; // edx
  unsigned int v2; // r13d
  unsigned int v3; // esi
  unsigned int v4; // r14d
  int v5; // r13d
  unsigned int v6; // r15d
  unsigned int v7; // ebp
  short *v8; // rax
  unsigned int v9; // r8d
  int v10; // r9d
  char *v11; // rdi
  char *v12; // rdi
  int v14; // eax
  long long v15; // rsi
  unsigned int v16; // r15d
  long long v17; // rdx
  long long v18; // rsi
  char *v19; // rdi
  int v20; // eax
  int v21; // ebp
  int v22; // r15d
  int v23; // eax
  int v24; // r14d
  int v25; // ecx
  int v26; // edi
  int v27; // eax
  int v28; // esi
  long long v29; // r10
  short *v30; // rsi
  short v31; // r11
  unsigned int v32; // r9d
  long long v33; // rsi
  long long v34; // rdx
  long long v35; // rsi
  char *v36; // rdi
  unsigned int v37; // eax
  unsigned int v38; // edx
  int v39; // [rsp+4h] [rbp-44h]
  unsigned int v40; // [rsp+8h] [rbp-40h]
  int v41; // [rsp+8h] [rbp-40h]
  unsigned int v42; // [rsp+Ch] [rbp-3Ch]

  sub_4DB0(a1);
  if ( a1 <= 3 )
    return sub_4AA0();
  v1 = dword_1A09C;
  v2 = -1;
  if ( !dword_1A09C )
    goto LABEL_26;
  v3 = 2;
  v4 = 0;
  v5 = 0;
  while ( 1 )
  {
    v6 = dword_1A06C;
    v7 = 2;
    dword_1A0A4 = ((unsigned char)byte_3C000[dword_1A06C + 2] ^ (unsigned short)(32 * dword_1A0A4)) & 0x7FFF;
    v8 = (short *)&qword_1B880 + (unsigned int)(dword_1A0A4 + 0x8000);
    v9 = (unsigned short)*v8;
    *((short *)&qword_1B880 + (dword_1A06C & 0x7FFF)) = v9;
    v10 = dword_1A068;
    *v8 = v6;
    dword_1A070 = v3;
    if ( v9 && dword_1A098 > v3 )
    {
      if ( v6 - v9 <= 0x7EFA && v6 <= 0xFEFA )
      {
        v40 = v1;
        v39 = v10;
        v37 = sub_4620(v9);
        v38 = v40;
        v10 = v39;
        if ( v37 <= v40 )
          v38 = v37;
        v7 = v38;
        if ( v38 == 3 )
        {
          v6 = dword_1A06C;
          v3 = dword_1A070;
          if ( (unsigned int)(dword_1A06C - dword_1A068) >= 0x1001 )
            v7 = 2;
        }
        else
        {
          v3 = dword_1A070;
          v6 = dword_1A06C;
        }
      }
      else
      {
        v7 = 2;
      }
    }
    if ( v3 > 2 && v7 <= v3 )
    {
      v20 = sub_B290(v6 - v10 - 1, v3 - 3);
      v21 = dword_1A070;
      v22 = dword_1B000;
      v5 = v20;
      v23 = dword_1A070 - 2;
      v1 = dword_1A09C + 1 - dword_1A070;
      dword_1A070 -= 2;
      dword_1A09C = v1;
      v24 = dword_1A06C;
      if ( dword_1B000 )
      {
        v42 = v1;
        v41 = v23;
        sub_4840(dword_1A06C, v21 - 1);
        v1 = v42;
        v23 = v41;
      }
      LOWORD(v25) = dword_1A0A4;
      v26 = v23 + v24;
      v27 = v24;
      do
      {
        v28 = v27++;
        v29 = v27 & 0x7FFF;
        v25 = ((unsigned char)byte_3C000[v28 + 3] ^ (unsigned short)(32 * v25)) & 0x7FFF;
        v30 = (short *)&qword_1B880 + (unsigned int)(v25 + 0x8000);
        v31 = *v30;
        *((short *)&qword_1B880 + v29) = *v30;
        *v30 = v27;
      }
      while ( v27 != v26 );
      v32 = v24 + v21 - 1;
      dword_1A0A4 = v25;
      *((short *)&qword_1B880 + v29) = v31;
      *v30 = v24 + v21 - 2;
      dword_1A070 = 0;
      dword_1A06C = v32;
      if ( v22 && (v33 = v32, v32 > (unsigned long long)qword_1A088) )
      {
        v34 = 1;
        v5 = 2;
        qword_1A088 = 0xFFFFFFFFLL;
      }
      else
      {
        if ( !v5 )
        {
          v4 = 0;
          v7 = 2;
          goto LABEL_18;
        }
        v34 = (unsigned int)(v5 - 1);
        v33 = v32;
      }
      v7 = 2;
      v35 = v33 - qword_1A078;
      v36 = &byte_3C000[(unsigned int)qword_1A078];
      if ( qword_1A078 < 0 )
        v36 = 0;
      v4 = 0;
      sub_B070(v36, v35, v34, 0);
      v1 = dword_1A09C;
      qword_1A078 = (unsigned int)dword_1A06C;
      goto LABEL_18;
    }
    if ( !v4 )
    {
      if ( dword_1B000 )
      {
        if ( v6 <= (unsigned long long)qword_1A088 )
          goto LABEL_36;
        v5 = 2;
        qword_1A088 = 0xFFFFFFFFLL;
        v11 = &byte_3C000[(unsigned int)qword_1A078];
        if ( qword_1A078 < 0 )
          v11 = 0;
        sub_B070(v11, v6 - qword_1A078, 1, 0);
        qword_1A078 = (unsigned int)dword_1A06C;
        v6 = dword_1A06C;
        if ( dword_1B000 )
LABEL_36:
          sub_4840(v6, 1);
      }
      v4 = 1;
      dword_1A06C = v6 + 1;
      v1 = --dword_1A09C;
      goto LABEL_18;
    }
    v14 = sub_B290(0, (unsigned char)byte_3C000[v6 - 1]);
    v5 = v14;
    if ( !dword_1B000 )
    {
      v16 = dword_1A06C;
      v17 = (unsigned int)(v14 - 1);
      v15 = (unsigned int)dword_1A06C;
      if ( !v14 )
        goto LABEL_35;
      goto LABEL_32;
    }
    v15 = (unsigned int)dword_1A06C;
    v16 = dword_1A06C;
    if ( (unsigned int)dword_1A06C > (unsigned long long)qword_1A088 )
    {
      v17 = v4;
      v5 = 2;
      qword_1A088 = 0xFFFFFFFFLL;
LABEL_32:
      v18 = v15 - qword_1A078;
      v19 = &byte_3C000[(unsigned int)qword_1A078];
      if ( qword_1A078 < 0 )
        v19 = 0;
      sub_B070(v19, v18, v17, 0);
      qword_1A078 = (unsigned int)dword_1A06C;
      v16 = dword_1A06C;
      if ( !dword_1B000 )
        goto LABEL_35;
      goto LABEL_40;
    }
    v17 = (unsigned int)(v14 - 1);
    if ( v14 )
      goto LABEL_32;
LABEL_40:
    sub_4840(v16, 1);
LABEL_35:
    dword_1A06C = v16 + 1;
    v1 = --dword_1A09C;
LABEL_18:
    if ( v1 <= 0x105 )
    {
      while ( !dword_1A0A0 )
      {
        sub_4940();
        v1 = dword_1A09C;
        if ( (unsigned int)dword_1A09C > 0x105 )
          goto LABEL_23;
      }
      v1 = dword_1A09C;
      if ( !dword_1A09C )
        break;
    }
LABEL_23:
    v3 = v7;
  }
  v2 = v5 - 1;
  if ( v4 )
    sub_B290(0, (unsigned char)byte_3C000[dword_1A06C - 1]);
LABEL_26:
  v12 = 0;
  if ( qword_1A078 >= 0 )
    v12 = &byte_3C000[(unsigned int)qword_1A078];
  return sub_B070(v12, (unsigned int)dword_1A06C - qword_1A078, v2, 1);
}



// Function: get_input_size_and_time @ 0x53f0
long long get_input_size_and_time()
{
  __int128 v0; // rax

  qword_1B840 = -1;
  *(&timer + 1) = -1;
  *(long long *)&v0 = buf.st_mode & 0xF000;
  if ( (int)v0 == 0x8000 )
  {
    qword_1B840 = buf.st_size;
    if ( !dword_19028 || (*(long long *)&v0 = (unsigned int)dword_DD9E8, dword_DD9E8) )
    {
      *(long long *)&v0 = sub_E940(&buf);
      *(_OWORD *)&timer = v0;
    }
  }
  return v0;
}



// Function: volatile_strcpy @ 0x5470
long long volatile_strcpy(char *a1, unsigned char *a2)
{
  long long result; // rax

  while ( 1 )
  {
    result = *a2;
    *a1 = result;
    if ( !(char)result )
      break;
    ++a2;
    ++a1;
  }
  return result;
}



// Function: help @ 0x5490
long long help(long long a1, long long a2, int a3, int a4, int a5, int a6)
{
  const char **v6; // rbx
  int v7; // edx
  int v8; // ecx
  int v9; // r8d
  int v10; // r9d
  const char *v11; // rsi
  long long result; // rax

  v6 = (const char **)&off_186E0;
  sub_F7B0((unsigned int)"Usage: %s [OPTION]... [FILE]...\n", qword_1B850, a3, a4, a5, a6);
  v11 = "Compress or uncompress FILEs (by default, compress FILES in-place).";
  do
  {
    ++v6;
    result = sub_F7B0((unsigned int)"%s\n", (int)v11, v7, v8, v9, v10);
    v11 = *v6;
  }
  while ( *v6 );
  return result;
}



// Function: license @ 0x54e0
long long license(long long a1, long long a2, long long a3, int a4, int a5, int a6)
{
  const char **v6; // rbx
  int v7; // edx
  int v8; // ecx
  int v9; // r8d
  int v10; // r9d
  const char *v11; // rsi
  long long result; // rax

  v6 = (const char **)off_18B20;
  sub_F7B0((unsigned int)"%s %s\n", qword_1B850, (int)off_193F8, a4, a5, a6);
  v11 = "Copyright (C) 2018 Free Software Foundation, Inc.";
  do
  {
    ++v6;
    result = sub_F7B0((unsigned int)"%s\n", (int)v11, v7, v8, v9, v10);
    v11 = *v6;
  }
  while ( *v6 );
  return result;
}



// Function: version @ 0x5540
long long version(long long a1, long long a2, long long a3, int a4, int a5, int a6)
{
  int v6; // edx
  int v7; // ecx
  int v8; // r8d
  int v9; // r9d
  int v10; // edx
  int v11; // ecx
  int v12; // r8d
  int v13; // r9d

  sub_54E0(a1, a2, a3, a4, a5, a6);
  sub_F7B0((unsigned int)"\n", a2, v6, v7, v8, v9);
  return sub_F7B0((unsigned int)"Written by Jean-loup Gailly.\n", a2, v10, v11, v12, v13);
}



// Function: install_signal_handlers @ 0x5570
unsigned long long install_signal_handlers()
{
  int v0; // ebp
  int *v1; // rbx
  long long v2; // rbx
  int v3; // ebp
  struct sigaction v5; // [rsp+0h] [rbp-D8h] BYREF
  unsigned long long v6; // [rsp+98h] [rbp-40h]

  v0 = 2;
  v1 = dword_13350;
  v6 = __readfsqword(0x28u);
  sigemptyset(&set);
  while ( 1 )
  {
    sigaction(v0, 0, &v5);
    if ( v5.sa_handler != (__sighandler_t)((char *)&dword_0 + 1) )
      sigaddset(&set, v0);
    if ( ++v1 == &dword_13350[6] )
      break;
    v0 = *v1;
  }
  v5.sa_flags = 0;
  v5.sa_handler = (__sighandler_t)sub_57E0;
  v2 = 0;
  *(__m128i *)v5.sa_mask.__val = _mm_load_si128((const __m128i *)&set);
  v3 = 2;
  *(__m128i *)&v5.sa_mask.__val[2] = _mm_load_si128((const __m128i *)&set.__val[2]);
  *(__m128i *)&v5.sa_mask.__val[4] = _mm_load_si128((const __m128i *)&set.__val[4]);
  *(__m128i *)&v5.sa_mask.__val[6] = _mm_load_si128((const __m128i *)&set.__val[6]);
  *(__m128i *)&v5.sa_mask.__val[8] = _mm_load_si128((const __m128i *)&set.__val[8]);
  *(__m128i *)&v5.sa_mask.__val[10] = _mm_load_si128((const __m128i *)&set.__val[10]);
  *(__m128i *)&v5.sa_mask.__val[12] = _mm_load_si128((const __m128i *)&set.__val[12]);
  *(__m128i *)&v5.sa_mask.__val[14] = _mm_load_si128((const __m128i *)&set.__val[14]);
  while ( 1 )
  {
    if ( sigismember(&set, v3) )
    {
      if ( !v2 )
        dword_DD9E0 = 1;
      sigaction(v3, &v5, 0);
    }
    if ( ++v2 == 6 )
      break;
    v3 = dword_13350[v2];
  }
  return v6 - __readfsqword(0x28u);
}



// Function: remove_output_file @ 0x56e0
unsigned long long remove_output_file(char a1)
{
  int v1; // edi
  const char *v3; // r8
  int v4; // edi
  const char *v5; // r8
  sigset_t v6; // [rsp+0h] [rbp-498h] BYREF
  char v7[1032]; // [rsp+80h] [rbp-418h] BYREF
  unsigned long long v8; // [rsp+488h] [rbp-10h]

  v8 = __readfsqword(0x28u);
  if ( a1 )
  {
    v1 = fd;
    if ( fd >= 0 )
    {
      fd = -1;
      close(v1);
      sub_5470(v7, byte_DD520);
      j__unlink(v3);
    }
  }
  else
  {
    sigprocmask(0, &set, &v6);
    v4 = fd;
    if ( fd >= 0 )
    {
      fd = -1;
      close(v4);
      sub_5470(v7, byte_DD520);
      j__unlink(v5);
    }
    sigprocmask(2, &v6, 0);
  }
  return v8 - __readfsqword(0x28u);
}



// Function: abort_gzip_signal @ 0x57e0
int abort_gzip_signal(int sig)
{
  sub_56E0(1);
  if ( dword_DD920 == sig )
    _exit(2);
  signal(sig, 0);
  return raise(sig);
}



// Function: progerror @ 0x5820
void progerror(char *s)
{
  int *v1; // rax
  int v2; // r12d
  int *v3; // rbx
  char v4; // [rsp+0h] [rbp-18h]

  v1 = __errno_location();
  v2 = *v1;
  v3 = v1;
  sub_F630(stderr, v4);
  *v3 = v2;
  perror(s);
  status = 1;
}



// Function: check_ofname @ 0x5870
int check_ofname()
{
  FILE *v0; // rdi
  int result; // eax
  char v2; // [rsp+0h] [rbp-8h]
  char v3; // [rsp+0h] [rbp-8h]

  if ( !dword_DD9F4 )
  {
    if ( (sub_F630(stderr, v2), !dword_DD9E0)
      || !byte_DDA01 && !isatty(0)
      || (sub_F630(stderr, v3),
          v0 = stderr,
          sub_E460(stderr),
          !(unsigned char)sub_F5F0(v0, " do you wish to overwrite (y or n)? ")) )
    {
      sub_F630(stderr, v3);
      if ( !status )
      {
        status = 2;
        return 1;
      }
      return 1;
    }
  }
  result = j__unlink(file);
  if ( result )
  {
    sub_5820(file);
    return 1;
  }
  return result;
}



// Function: input_eof @ 0x5940
long long input_eof()
{
  unsigned int v0; // r12d

  if ( !dword_DD9F8 )
    return 1;
  v0 = dword_DD9DC;
  if ( dword_DD9DC )
    return 1;
  if ( dword_1B008 == dword_1B00C )
  {
    if ( dword_1B00C == (int)&unk_40000 && (unsigned int)sub_D830(1) != -1 )
    {
      dword_1B008 = 0;
      return v0;
    }
    return 1;
  }
  return v0;
}



// Function: atdir_eq @ 0x59a0
bool atdir_eq(void *s2, size_t n)
{
  size_t v3; // rbx
  size_t v4; // rdx
  int v5; // r8d
  bool result; // al

  if ( n )
  {
    v3 = n;
    v4 = n;
  }
  else
  {
    v4 = 1;
    v3 = 1;
    s2 = &unk_13310;
  }
  v5 = memcmp(byte_DD100, s2, v4);
  result = 0;
  if ( !v5 )
    return byte_DD100[v3] == 0;
  return result;
}



// Function: atdir_set @ 0x5a00
long long atdir_set(void *a1, size_t a2)
{
  void *v2; // rbp
  long long v3; // rbx
  bool v4; // r8
  long long result; // rax
  long long v6; // rdx
  long long v7; // rax
  int v8; // edx
  int v9; // ecx
  int v10; // r8d
  int v11; // r9d

  v2 = a1;
  v3 = a2;
  v4 = sub_59A0(a1, a2);
  result = (unsigned int)fildes;
  if ( !v4 )
  {
    if ( fildes >= 0 )
    {
      close(fildes);
      v6 = a2;
      if ( a2 )
        goto LABEL_4;
    }
    else
    {
      v6 = a2;
      if ( a2 )
      {
LABEL_4:
        v7 = __memcpy_chk(byte_DD100, v2, v6, 1024);
        *(char *)(v7 + v3) = 0;
        result = sub_E3C0(v7, 0x10000, v8, v9, v10, v11);
        fildes = result;
        return result;
      }
    }
    v6 = 1;
    v3 = 1;
    v2 = &unk_13310;
    goto LABEL_4;
  }
  return result;
}



// Function: discard_input_bytes @ 0x5a90
unsigned long long discard_input_bytes(long long a1, char a2)
{
  long long v2; // rbx
  long long v3; // rax
  int v4; // edx
  char v5; // al
  char v7; // [rsp+7h] [rbp-21h] BYREF
  unsigned long long v8; // [rsp+8h] [rbp-20h]

  v2 = a1;
  v8 = __readfsqword(0x28u);
  if ( a1 )
  {
    while ( 1 )
    {
      v3 = (unsigned int)dword_1B008;
      if ( dword_1B008 < (unsigned int)dword_1B00C )
        break;
      while ( 1 )
      {
        v7 = sub_D830(0);
        if ( (a2 & 2) != 0 )
LABEL_10:
          sub_D1F0(&v7, 1);
LABEL_4:
        if ( v2 != -1 )
          break;
        if ( !v7 )
          return v8 - __readfsqword(0x28u);
        v3 = (unsigned int)dword_1B008;
        if ( dword_1B008 < (unsigned int)dword_1B00C )
          goto LABEL_3;
      }
      if ( !--v2 )
        return v8 - __readfsqword(0x28u);
    }
LABEL_3:
    v4 = v3 + 1;
    v5 = byte_9D000[v3];
    dword_1B008 = v4;
    v7 = v5;
    if ( (a2 & 2) != 0 )
      goto LABEL_10;
    goto LABEL_4;
  }
  return v8 - __readfsqword(0x28u);
}



// Function: get_method @ 0x5b50
long long get_method(unsigned int a1)
{
  unsigned int v1; // ecx
  long long v2; // rax
  long long v3; // rdx
  char v4; // al
  int v5; // r12d
  int v6; // eax
  int v7; // r14d
  int v8; // ebx
  long long result; // rax
  long long v10; // rdx
  int v11; // eax
  long long v12; // rax
  int v13; // eax
  long long v14; // rax
  char v15; // bl
  long long v16; // rax
  unsigned int v17; // edx
  long long v18; // rcx
  long long v19; // r14
  long long v20; // rsi
  long long v21; // rax
  long long v22; // r14
  long long v23; // rcx
  long long v24; // r13
  long long v25; // r14
  long long v26; // rax
  long long v27; // r14
  long long v28; // rax
  unsigned int v29; // ecx
  long long v30; // rdx
  char v31; // al
  char v32; // al
  char v33; // r13
  long long v34; // rax
  unsigned int v35; // ecx
  long long v36; // rdx
  unsigned char v37; // al
  int v38; // r15d
  unsigned char v39; // al
  unsigned int v40; // r14d
  char *v41; // r15
  char v42; // al
  int v43; // eax
  int v44; // eax
  int v45; // eax
  unsigned short v46; // ax
  unsigned int v47; // ecx
  int v48; // ebp
  long long v49; // rax
  long long v50; // rdx
  int v51; // ebx
  int v52; // eax
  int v53; // eax
  const char *v54; // r15
  size_t v55; // rax
  char v56; // [rsp+0h] [rbp-68h]
  char *dest; // [rsp+8h] [rbp-60h]
  char v58[2]; // [rsp+1Ch] [rbp-4Ch] BYREF
  char s1[4]; // [rsp+1Eh] [rbp-4Ah] BYREF
  int v60; // [rsp+22h] [rbp-46h]
  char v61; // [rsp+26h] [rbp-42h]
  char v62; // [rsp+27h] [rbp-41h]
  unsigned long long v63; // [rsp+28h] [rbp-40h]

  v1 = dword_1B00C;
  v63 = __readfsqword(0x28u);
  v2 = (unsigned int)dword_1B008;
  if ( dword_DD9F4 && dword_1B860 )
  {
    if ( dword_1B008 >= (unsigned int)dword_1B00C )
    {
      v11 = sub_D830(1);
      v10 = (unsigned int)dword_1B008;
      v1 = dword_1B00C;
      v7 = v11;
    }
    else
    {
      v10 = (unsigned int)(dword_1B008 + 1);
      v7 = (unsigned char)byte_9D000[dword_1B008++];
      LOBYTE(v11) = v7;
    }
    s1[0] = v11;
    if ( (unsigned int)v10 >= v1 )
    {
      v6 = sub_D830(1);
      v5 = v6;
    }
    else
    {
      dword_1B008 = v10 + 1;
      v5 = (unsigned char)byte_9D000[v10];
      LOBYTE(v6) = byte_9D000[v10];
    }
  }
  else
  {
    if ( dword_1B008 >= (unsigned int)dword_1B00C )
    {
      v4 = sub_D830(0);
      v3 = (unsigned int)dword_1B008;
      v1 = dword_1B00C;
    }
    else
    {
      v3 = (unsigned int)++dword_1B008;
      v4 = byte_9D000[v2];
    }
    s1[0] = v4;
    if ( v4 )
    {
      if ( v1 <= (unsigned int)v3 )
      {
        LOBYTE(v6) = sub_D830(0);
      }
      else
      {
        dword_1B008 = v3 + 1;
        LOBYTE(v6) = byte_9D000[v3];
      }
      v5 = 0;
      v7 = 0;
    }
    else
    {
      if ( v1 <= (unsigned int)v3 )
      {
        v6 = sub_D830(1);
        v5 = v6;
      }
      else
      {
        dword_1B008 = v3 + 1;
        v5 = (unsigned char)byte_9D000[v3];
        LOBYTE(v6) = byte_9D000[v3];
      }
      v7 = 0;
    }
  }
  s1[1] = v6;
  dword_19020 = -1;
  qword_E0EA8 = 0;
  v8 = ++dword_DD9D8;
  dword_DD9DC = 0;
  if ( !memcmp(s1, &unk_12CB4, 2u) || !memcmp(s1, &unk_12CB7, 2u) )
  {
    v12 = (unsigned int)dword_1B008;
    if ( dword_1B008 >= (unsigned int)dword_1B00C )
    {
      v13 = sub_D830(0);
    }
    else
    {
      ++dword_1B008;
      v13 = (unsigned char)byte_9D000[v12];
    }
    dword_19020 = v13;
    if ( v13 != 8 )
    {
      sub_F630(stderr, v56);
      status = 1;
      return 0xFFFFFFFFLL;
    }
    off_190D0 = sub_CD20;
    v14 = (unsigned int)dword_1B008;
    if ( dword_1B008 >= (unsigned int)dword_1B00C )
    {
      v15 = sub_D830(0);
    }
    else
    {
      ++dword_1B008;
      v15 = byte_9D000[v14];
    }
    if ( (v15 & 0x20) != 0 )
      goto LABEL_142;
    if ( (v15 & 0xC0) != 0 )
    {
      sub_F630(stderr, v56);
      status = 1;
      if ( dword_DD9F4 <= 1 )
        return 0xFFFFFFFFLL;
      v16 = (unsigned int)dword_1B008;
      v17 = dword_1B00C;
      if ( dword_1B008 < (unsigned int)dword_1B00C )
      {
LABEL_32:
        v18 = (unsigned int)(v16 + 1);
        dword_1B008 = v18;
        v19 = (unsigned char)byte_9D000[v16];
        if ( v17 > (unsigned int)v18 )
          goto LABEL_33;
        goto LABEL_86;
      }
    }
    else
    {
      v16 = (unsigned int)dword_1B008;
      v17 = dword_1B00C;
      if ( dword_1B008 < (unsigned int)dword_1B00C )
        goto LABEL_32;
    }
    v43 = sub_D830(0);
    v18 = (unsigned int)dword_1B008;
    v17 = dword_1B00C;
    v19 = v43;
    if ( dword_1B00C > (unsigned int)dword_1B008 )
    {
LABEL_33:
      v20 = (unsigned int)(v18 + 1);
      v21 = (unsigned char)byte_9D000[v18];
      dword_1B008 = v20;
      v22 = (v21 << 8) | v19;
      if ( (unsigned int)v20 < v17 )
        goto LABEL_34;
      goto LABEL_87;
    }
LABEL_86:
    v44 = sub_D830(0);
    v20 = (unsigned int)dword_1B008;
    v17 = dword_1B00C;
    v22 = ((long long)v44 << 8) | v19;
    if ( dword_1B008 < (unsigned int)dword_1B00C )
    {
LABEL_34:
      v23 = (unsigned int)(v20 + 1);
      v24 = (unsigned char)byte_9D000[v20];
      dword_1B008 = v23;
      v25 = (v24 << 16) | v22;
      if ( v17 > (unsigned int)v23 )
      {
LABEL_35:
        dword_1B008 = v23 + 1;
        v26 = (unsigned long long)(unsigned char)byte_9D000[v23] << 24;
        goto LABEL_36;
      }
LABEL_88:
      v26 = (long long)(int)sub_D830(0) << 24;
LABEL_36:
      v27 = v26 | v25;
      if ( v27 && !dword_19028 )
      {
        if ( v27 < 0 )
        {
          if ( !dword_1B85C )
            sub_F630(stderr, v56);
          if ( !status )
            status = 2;
          *(&timer + 1) = 999999999;
          timer = 0x7FFFFFFFFFFFFFFFLL;
        }
        else
        {
          *(_OWORD *)&timer = (unsigned long long)v27;
        }
      }
      v28 = (unsigned int)dword_1B008;
      v29 = dword_1B00C;
      if ( dword_1B008 >= (unsigned int)dword_1B00C )
      {
        v31 = sub_D830(0);
        v30 = (unsigned int)dword_1B008;
        v29 = dword_1B00C;
      }
      else
      {
        v30 = (unsigned int)++dword_1B008;
        v31 = byte_9D000[v28];
      }
      v61 = v31;
      if ( v29 <= (unsigned int)v30 )
      {
        v32 = sub_D830(0);
      }
      else
      {
        dword_1B008 = v30 + 1;
        v32 = byte_9D000[v30];
      }
      v62 = v32;
      v33 = v15 & 2;
      if ( (v15 & 2) != 0 )
      {
        s1[2] = 8;
        s1[3] = v15;
        v60 = v27;
        sub_D1F0(0, 0);
        sub_D1F0(s1, 10);
      }
      if ( (v15 & 4) != 0 )
      {
        v34 = (unsigned int)dword_1B008;
        v35 = dword_1B00C;
        if ( dword_1B008 >= (unsigned int)dword_1B00C )
        {
          v37 = sub_D830(0);
          v36 = (unsigned int)dword_1B008;
          v35 = dword_1B00C;
        }
        else
        {
          v36 = (unsigned int)++dword_1B008;
          v37 = byte_9D000[v34];
        }
        v58[0] = v37;
        v38 = v37;
        if ( v35 <= (unsigned int)v36 )
        {
          v39 = sub_D830(0);
        }
        else
        {
          dword_1B008 = v36 + 1;
          v39 = byte_9D000[v36];
        }
        v58[1] = v39;
        v40 = v38 | (v39 << 8);
        if ( dword_DD9E4 )
          sub_F630(stderr, v56);
        if ( v33 )
          sub_D1F0(v58, 2);
        sub_5A90(v40, v15);
      }
      if ( (v15 & 8) != 0 )
      {
        if ( dword_1902C || dword_1B860 && !dword_DD9E8 || dword_DD9D8 > 1 )
        {
          sub_5A90(-1, v15);
        }
        else
        {
          dest = (char *)sub_D3B0(file);
          v41 = dest;
          while ( 1 )
          {
            if ( dword_1B008 < (unsigned int)dword_1B00C )
              v42 = byte_9D000[dword_1B008++];
            else
              v42 = sub_D830(0);
            *v41++ = v42;
            if ( !v42 )
              break;
            if ( v41 >= dword_1B420 )
              sub_D510("corrupted input -- file name too large");
          }
          if ( v33 )
            sub_D1F0(dest, (unsigned int)((int)v41 - (int)dest));
          v54 = (const char *)sub_D3B0(dest);
          v55 = strlen(v54);
          memmove(dest, v54, v55 + 1);
        }
      }
      if ( (v15 & 0x10) != 0 )
        sub_5A90(-1, v15);
      if ( v33 )
      {
        v46 = sub_D1F0(s1, 0);
        v47 = dword_1B00C;
        v48 = v46;
        v49 = (unsigned int)dword_1B008;
        if ( dword_1B008 >= (unsigned int)dword_1B00C )
        {
          v53 = sub_D830(0);
          v50 = (unsigned int)dword_1B008;
          v47 = dword_1B00C;
          v51 = v53;
        }
        else
        {
          v50 = (unsigned int)++dword_1B008;
          v51 = (unsigned char)byte_9D000[v49];
        }
        if ( (unsigned int)v50 >= v47 )
        {
          v52 = (unsigned int)sub_D830(0) << 8;
        }
        else
        {
          dword_1B008 = v50 + 1;
          v52 = (unsigned char)byte_9D000[v50] << 8;
        }
        if ( v48 != (v51 | v52) )
        {
          sub_F630(stderr, v56);
          status = 1;
          if ( dword_DD9F4 <= 1 )
            return 0xFFFFFFFFLL;
        }
      }
      result = (unsigned int)dword_19020;
      if ( dword_DD9D8 != 1 )
      {
        if ( dword_19020 >= 0 )
          return result;
        goto LABEL_71;
      }
      qword_E0EA8 = (unsigned int)(dword_1B008 + 8);
      if ( dword_19020 < 0 )
        goto LABEL_142;
      return result;
    }
LABEL_87:
    v45 = sub_D830(0);
    v23 = (unsigned int)dword_1B008;
    v25 = ((long long)v45 << 16) | v22;
    if ( dword_1B00C > (unsigned int)dword_1B008 )
      goto LABEL_35;
    goto LABEL_88;
  }
  if ( !memcmp(s1, &unk_12CD7, 2u) && dword_1B008 == 2 && !memcmp(byte_9D000, &unk_12CD7, 4u) )
  {
    dword_1B008 = 0;
    off_190D0 = sub_CD20;
    if ( !(unsigned int)sub_CC20(a1) )
    {
      dword_DD9DC = 1;
      goto LABEL_139;
    }
    return 0xFFFFFFFFLL;
  }
  if ( !memcmp(s1, &unk_12CDC, 2u) )
  {
    dword_19020 = 2;
    off_190D0 = sub_C9F0;
    return 2;
  }
  if ( !memcmp(s1, &unk_12CDF, 2u) )
  {
    dword_19020 = 1;
    dword_DD9DC = 1;
    off_190D0 = sub_C040;
    return 1;
  }
  if ( !memcmp(s1, &unk_12CE2, 2u) )
  {
    dword_19020 = 3;
    dword_DD9DC = 1;
    off_190D0 = sub_BFD0;
    return 3;
  }
  if ( !dword_DD9F4 || !dword_1B860 || (result = (unsigned int)dword_DD9E8, dword_DD9E8) )
  {
LABEL_141:
    if ( v8 == 1 )
    {
LABEL_142:
      sub_F630(stderr, v56);
      status = 1;
      return 0xFFFFFFFFLL;
    }
LABEL_71:
    if ( s1[0] )
      goto LABEL_74;
    while ( !v5 )
    {
      if ( dword_1B008 >= (unsigned int)dword_1B00C )
        v5 = sub_D830(1);
      else
        v5 = (unsigned char)byte_9D000[dword_1B008++];
    }
    if ( v5 == -1 )
    {
      if ( dword_DD9E4 )
      {
        if ( !dword_1B85C )
          sub_F630(stderr, v56);
        if ( !status )
          status = 2;
      }
      return 4294967293LL;
    }
    else
    {
LABEL_74:
      if ( !dword_1B85C )
        sub_F630(stderr, v56);
      result = 4294967294LL;
      if ( !status )
        status = 2;
    }
    return result;
  }
  dword_19020 = 0;
  off_190D0 = (long long (*)())sub_D720;
  if ( v5 != -1 )
    --dword_1B008;
  dword_DD9DC = 1;
  if ( v7 != -1 )
  {
    sub_D6C0(1, s1, 1);
LABEL_139:
    result = (unsigned int)dword_19020;
    if ( dword_19020 >= 0 )
      return result;
    v8 = dword_DD9D8;
    goto LABEL_141;
  }
  return result;
}



// Function: get_suffix @ 0x67e0
const char *get_suffix(const char *a1)
{
  const char *v1; // r13
  char *v2; // r12
  unsigned long long v3; // rbp
  char **v4; // rbx
  size_t v5; // rax
  char *v6; // r14
  char **v7; // rax
  long long v8; // rdx
  char **v9; // rbx
  int v10; // eax
  const char *v11; // r15
  int v12; // ebp
  int v13; // eax
  long long v14; // r13
  long long v16; // [rsp+8h] [rbp-80h]
  int v17; // [rsp+18h] [rbp-70h]
  char v18[40]; // [rsp+20h] [rbp-68h] BYREF
  unsigned long long v19; // [rsp+48h] [rbp-40h]

  v1 = off_19048;
  v2 = src;
  v19 = __readfsqword(0x28u);
  if ( off_19048 )
  {
    v3 = qword_DD9C0;
    v4 = &off_19048;
    while ( 1 )
    {
      v5 = strlen(v1);
      if ( v3 < v5 && !strcmp(v2, &v1[v5 - v3]) )
        break;
      v1 = v4[1];
      ++v4;
      if ( !v1 )
        goto LABEL_6;
    }
    v9 = &off_19048;
    v6 = (char *)sub_F5D0(v2);
    sub_D350(v6);
    v8 = 8;
    v7 = &off_19048 - 1;
  }
  else
  {
LABEL_6:
    v6 = (char *)sub_F5D0(v2);
    sub_D350(v6);
    v7 = (char **)&unk_19040;
    v8 = 0;
    v9 = (char **)&unk_19040;
  }
  v7[v8] = v6;
  v17 = strlen(a1);
  if ( v17 > 32 )
    __strcpy_chk(v18, &a1[v17 - 32], 33);
  else
    __strcpy_chk(v18, a1, 33);
  sub_D350(v18);
  v10 = strlen(v18);
  v11 = *v9;
  v12 = v10;
  v16 = v10;
  while ( 1 )
  {
    v13 = strlen(v11);
    if ( v12 > v13 && v18[v12 - v13 - 1] != 47 )
    {
      v14 = v13;
      if ( !strcmp(&v18[v16 - v13], v11) )
        break;
    }
    v11 = v9[1];
    ++v9;
    if ( !v11 )
      goto LABEL_14;
  }
  v11 = &a1[v17 - v14];
LABEL_14:
  free(v6);
  return v11;
}



// Function: make_ofname @ 0x69d0
long long make_ofname()
{
  const char *v0; // rax
  unsigned int v1; // r12d
  char *v2; // rbx
  unsigned int v3; // r12d
  char v5; // [rsp+0h] [rbp-28h]

  __strcpy_chk(file, dword_1B420, 1024);
  v0 = sub_67E0(file);
  v1 = dword_DD9F8;
  v2 = (char *)v0;
  if ( !dword_DD9F8 )
  {
    if ( v0 && !dword_DD9F4 )
    {
      if ( dword_DD9E4 || !(dword_1B85C | dword_DD9EC) )
        sub_F630(stderr, v5);
      return 2;
    }
    dword_1B848 = 0;
    if ( qword_DD9C0 + strlen(file) <= 0x3FF )
    {
      __strcat_chk(file, src, 1024);
      return v1;
    }
    if ( dword_1B85C )
      goto LABEL_10;
    goto LABEL_17;
  }
  if ( !v0 )
  {
    v3 = dword_DD9EC;
    if ( dword_DD9EC )
    {
      if ( !dword_DD9E4 )
        return 2;
LABEL_9:
      if ( dword_1B85C )
        goto LABEL_10;
      goto LABEL_17;
    }
    if ( dword_1B858 )
      return v3;
    if ( dword_DD9E4 )
      goto LABEL_9;
    if ( dword_1B85C )
      return 2;
LABEL_17:
    sub_F630(stderr, v5);
LABEL_10:
    if ( !status )
    {
      status = 2;
      return 2;
    }
    return 2;
  }
  sub_D350(v0);
  if ( strcmp(v2, ".tgz") && strcmp(v2, ".taz") )
  {
    *v2 = 0;
    return 0;
  }
  strcpy(v2, ".tar");
  return 0;
}



// Function: do_list @ 0x6c40
void do_list(int a1, int a2, int a3, int a4, int a5, int a6)
{
  int v6; // r12d
  int v7; // esi
  int v8; // edx
  int v9; // ecx
  int v10; // r8d
  int v11; // r9d
  int v12; // esi
  int v13; // edx
  int v14; // ecx
  int v15; // r8d
  int v16; // r9d
  long long v17; // rax
  long long v18; // rsi
  long long v19; // rcx
  long long v20; // rdi
  int v21; // edx
  int v22; // ecx
  int v23; // r8d
  int v24; // r9d
  long long v25; // rsi
  int v26; // edx
  int v27; // ecx
  int v28; // r8d
  int v29; // r9d
  int v30; // esi
  int v31; // edx
  int v32; // ecx
  int v33; // r8d
  int v34; // r9d
  long long v35; // rdi
  int v36; // esi
  int v37; // edx
  int v38; // ecx
  int v39; // r8d
  int v40; // r9d
  struct tm *v41; // rbp
  char *v42; // rsi
  int v43; // ecx
  int v44; // r8d
  int v45; // r9d
  int v46; // edx
  int v47; // ecx
  int v48; // r8d
  int v49; // r9d

  if ( dword_19010 )
  {
    if ( a1 >= 0 )
    {
      dword_19010 = 0;
      if ( dword_DD9E4 )
        sub_F7B0((unsigned int)"method  crc     date  time  ", a2, a3, a4, a5, a6);
      if ( !dword_1B85C )
        sub_F7B0((unsigned int)"%*.*s %*.*s  ratio uncompressed_name\n", 19, 19, (unsigned int)"compressed", 19, 19);
LABEL_7:
      v6 = -1;
      if ( a1 != 8 || dword_DD9DC )
      {
        if ( !dword_DD9E4 )
          goto LABEL_9;
      }
      else
      {
        v6 = qword_E0E90;
        if ( !dword_DD9E4 )
          goto LABEL_9;
      }
      v41 = localtime(&timer);
      v42 = off_18680[a1];
      sub_F7B0((unsigned int)"%5s %08lx ", (int)v42, v6, v43, v44, v45);
      if ( v41 )
        sub_F7B0(
          (unsigned int)"%s%3d %02d:%02d ",
          (unsigned int)&unk_132E0 + 4 * v41->tm_mon,
          v41->tm_mday,
          v41->tm_hour,
          v41->tm_min,
          v49);
      else
        sub_F7B0((unsigned int)"??? ?? ??:?? ", (int)v42, v46, v47, v48, v49);
LABEL_9:
      v7 = qword_1B828;
      sub_D930(stdout);
      sub_F7B0((unsigned int)" ", v7, v8, v9, v10, v11);
      v12 = qword_1B820;
      sub_D930(stdout);
      sub_F7B0((unsigned int)" ", v12, v13, v14, v15, v16);
      v17 = qword_1B828;
      if ( qword_1B828 == -1 )
      {
        qword_DD508 = -1;
        v18 = 0;
        v19 = 0;
        v17 = 0;
        qword_E0EA8 = 0;
        qword_1B820 = 0;
        qword_1B828 = 0;
      }
      else
      {
        if ( qword_DD508 >= 0 )
          qword_DD508 += qword_1B828;
        v18 = qword_1B820;
        v19 = qword_1B820;
        if ( qword_1B820 == -1 )
        {
          qword_DD500 = -1;
          v20 = 0;
          v18 = 0;
          qword_E0EA8 = 0;
          qword_1B820 = 0;
          qword_1B828 = 0;
LABEL_14:
          sub_D8E0(v20, v18, stdout);
          sub_F7B0((unsigned int)" %s\n", (unsigned int)file, v21, v22, v23, v24);
          return;
        }
      }
      if ( qword_DD500 >= 0 )
        qword_DD500 += v19;
      v20 = v18 - (v17 - qword_E0EA8);
      goto LABEL_14;
    }
  }
  else if ( a1 >= 0 )
  {
    goto LABEL_7;
  }
  v25 = qword_DD508;
  if ( qword_DD508 > 0 && qword_DD500 > 0 )
  {
    if ( dword_DD9E4
      && (sub_F7B0((unsigned int)"                            ", qword_DD508, a3, a4, dword_DD9E4, a6),
          v25 = qword_DD508,
          dword_DD9E4)
      || !dword_1B85C )
    {
      sub_D930(stdout);
      sub_F7B0((unsigned int)" ", v25, v26, v27, v28, v29);
      v30 = qword_DD500;
      sub_D930(stdout);
      sub_F7B0((unsigned int)" ", v30, v31, v32, v33, v34);
      v25 = qword_DD508;
    }
    v35 = qword_DD500 - (v25 - qword_E0EA8);
    v36 = qword_DD500;
    sub_D8E0(v35, qword_DD500, stdout);
    sub_F7B0((unsigned int)" (totals)\n", v36, v37, v38, v39, v40);
  }
}



// Function: do_exit @ 0x6fd0
void do_exit(int status)
{
  if ( !dword_DD040 )
  {
    dword_DD040 = 1;
    free(ptr);
    ptr = 0;
  }
  exit(status);
}



// Function: finish_out @ 0x7010
void finish_out()
{
  if ( (unsigned int)sub_E0F0(stdout) )
    sub_D670();
  sub_6FD0(0);
}



// Function: try_help @ 0x7040
void try_help(long long a1, const char *a2)
{
  char v2; // [rsp-8h] [rbp-8h]

  sub_F630(stderr, v2);
  sub_6FD0(1);
}



// Function: treat_stdin @ 0x7070
void treat_stdin(long long a1, long long a2, long long a3, long long a4, long long a5, long long a6, char a7)
{
  int v7; // edx
  int v8; // ecx
  int v9; // r8d
  int v10; // r9d
  int v11; // eax
  char v12; // [rsp+0h] [rbp-8h]

  if ( !(dword_DD9E8 | dword_DD9F4) && (byte_DDA01 || isatty(dword_DD9F8 == 0)) )
  {
    if ( !dword_1B85C )
      sub_F630(stderr, v12);
LABEL_23:
    sub_6FD0(1);
  }
  strcpy(dword_1B420, "stdin");
  strcpy(file, "stdout");
  if ( fstat(0, &buf) )
  {
    sub_5820("standard input");
    goto LABEL_23;
  }
  sub_53F0();
  sub_D270();
  dword_1B860 = 1;
  dword_DD9D8 = 0;
  dword_1B014 = 0;
  byte_DD510 = 1;
  if ( dword_DD9F8 )
  {
    v11 = sub_5B50(0);
    dword_19020 = v11;
    if ( v11 < 0 )
      sub_6FD0(status);
  }
  while ( !(unsigned int)off_190D0() )
  {
    if ( (unsigned int)sub_5940() )
    {
      if ( dword_DD9E8 )
      {
        sub_6C40(dword_19020, dword_DD9E8, v7, v8, v9, v10);
      }
      else if ( dword_DD9E4 )
      {
        if ( dword_1B858 )
        {
          sub_F630(stderr, a7);
        }
        else if ( !dword_DD9F8 )
        {
          sub_D8E0(qword_E0EA8 + qword_1B828 - qword_1B820, qword_1B828, stderr);
          sub_F630(stderr, a7);
        }
      }
      return;
    }
    dword_19020 = sub_5B50(dword_1B014);
    if ( dword_19020 < 0 )
      return;
    qword_1B820 = 0;
  }
}



// Function: create_outfile @ 0x76e0
long long create_outfile()
{
  int v0; // r13d
  char *v1; // r12
  int v2; // r8d
  int v3; // r9d
  long long result; // rax
  int *v5; // [rsp+0h] [rbp-D8h]
  int v6; // [rsp+8h] [rbp-D0h]
  int v7; // [rsp+Ch] [rbp-CCh]
  sigset_t oset; // [rsp+10h] [rbp-C8h] BYREF
  unsigned long long v9; // [rsp+98h] [rbp-40h]

  v9 = __readfsqword(0x28u);
  if ( dword_DD9F0 || (v1 = (char *)sub_E060(file), v0 = sub_5A00(file, v1 - file), v0 < 0) )
  {
    v0 = -100;
    v1 = file;
  }
  v7 = 0;
  LOBYTE(v5) = (unsigned char)__errno_location();
  while ( 1 )
  {
    sub_5470(byte_DD520, (unsigned char *)file);
    sigprocmask(0, &set, &oset);
    dword_1B010 = sub_E520(v0, (int)v1, 193, 384, v2, v3, (char)v5);
    fd = dword_1B010;
    v6 = *v5;
    sigprocmask(2, &oset, 0);
    if ( dword_1B010 >= 0 )
      break;
    if ( v6 == 17 )
    {
      if ( sub_5870() )
        goto LABEL_9;
    }
    else
    {
      if ( v6 != 36 )
      {
        sub_5820(file);
LABEL_9:
        close(dword_1B014);
        return 1;
      }
      sub_7550();
      v7 = 1;
    }
  }
  if ( !v7 || !dword_DD9F8 )
    return 0;
  if ( !dword_1B85C )
    sub_F630(stderr, (char)v5);
  result = (unsigned int)status;
  if ( status )
    return 0;
  status = 2;
  return result;
}



// Function: treat_file @ 0x7b60
void treat_file(char *s, long long a2, long long a3, long long a4, long long a5, long long a6, char a7)
{
  long long v8; // rdx
  long long v9; // rcx
  long long v10; // r8
  long long v11; // r9
  int v12; // edi
  int v13; // eax
  int v14; // ebp
  int v15; // eax
  long long v16; // rsi
  long long v17; // rdi
  int v18; // edx
  int v19; // ecx
  int v20; // r8d
  int v21; // r9d
  const char *v22; // r12
  int v23; // eax
  int *v24; // rax
  int v25; // r14d
  int *v26; // r12
  int v27; // eax
  int v28; // r14d
  sigset_t v29; // [rsp+0h] [rbp-B8h] BYREF
  unsigned long long v30; // [rsp+88h] [rbp-30h]

  v30 = __readfsqword(0x28u);
  if ( !strcmp(s, "-") )
  {
    v14 = dword_1B860;
    sub_7070((long long)s, (long long)"-", v8, v9, v10, v11, v29.__val[0]);
    dword_1B860 = v14;
    return;
  }
  dword_1B014 = sub_7380(s);
  v12 = dword_1B014;
  if ( dword_1B014 < 0 )
    return;
  v13 = buf.st_mode & 0xF000;
  if ( v13 == 0x4000 )
  {
    if ( dword_DD9EC )
    {
      sub_8400(dword_1B014, s);
    }
    else
    {
      close(dword_1B014);
      if ( !dword_1B85C )
        sub_F630(stderr, v29.__val[0]);
      if ( !status )
        status = 2;
    }
    return;
  }
  if ( dword_1B860 )
  {
    sub_53F0((unsigned int)dword_1B014);
    if ( !dword_1B858 )
    {
      strcpy(file, "stdout");
      goto LABEL_23;
    }
  }
  else
  {
    if ( v13 != 0x8000 )
    {
      if ( dword_1B85C )
      {
LABEL_8:
        if ( !status )
          status = 2;
        close(v12);
        return;
      }
LABEL_7:
      sub_F630(stderr, v29.__val[0]);
      v12 = dword_1B014;
      goto LABEL_8;
    }
    if ( (buf.st_mode & 0x800) != 0 )
    {
      if ( dword_1B85C )
        goto LABEL_8;
      goto LABEL_7;
    }
    if ( (buf.st_mode & 0x400) != 0 )
    {
      if ( dword_1B85C )
        goto LABEL_8;
      goto LABEL_7;
    }
    if ( !dword_DD9F4 )
    {
      if ( (buf.st_mode & 0x200) != 0 )
      {
        if ( dword_1B85C )
          goto LABEL_8;
        goto LABEL_7;
      }
      if ( buf.st_nlink > 1 )
      {
        if ( dword_1B85C )
          goto LABEL_8;
        goto LABEL_7;
      }
    }
    sub_53F0((unsigned int)dword_1B014);
  }
  if ( (unsigned int)sub_69D0() )
  {
LABEL_54:
    close(dword_1B014);
    return;
  }
LABEL_23:
  sub_D270();
  dword_DD9D8 = 0;
  if ( dword_DD9F8 )
  {
    dword_19020 = sub_5B50(dword_1B014);
    if ( dword_19020 < 0 )
      goto LABEL_54;
  }
  if ( dword_1B860 )
  {
    dword_1B010 = 1;
    v15 = dword_DD9E4;
  }
  else
  {
    if ( (unsigned int)sub_76E0() )
      return;
    v15 = dword_DD9E4;
    if ( !dword_DD9F8 )
    {
      if ( !dword_1B848 )
        goto LABEL_50;
      if ( dword_1B85C | dword_DD9E4 )
        goto LABEL_27;
      sub_F630(stderr, v29.__val[0]);
      v15 = dword_DD9E4;
    }
  }
  if ( !dword_1B848 )
LABEL_50:
    dword_1B848 = dword_1902C == 0;
LABEL_27:
  if ( v15 && !dword_DD9E8 )
    sub_F630(stderr, v29.__val[0]);
  while ( 1 )
  {
    v16 = (unsigned int)dword_1B010;
    if ( (unsigned int)off_190D0() )
      break;
    if ( (unsigned int)sub_5940() )
      goto LABEL_35;
    dword_19020 = sub_5B50(dword_1B014);
    if ( dword_19020 < 0 )
      goto LABEL_35;
    qword_1B820 = 0;
  }
  dword_19020 = -1;
LABEL_35:
  v17 = (unsigned int)dword_1B014;
  if ( close(dword_1B014) )
    sub_D600(v17, v16);
  if ( dword_DD9E8 )
  {
    sub_6C40(dword_19020, v16, v18, v19, v20, v21);
    return;
  }
  if ( dword_1B860 )
  {
    if ( dword_19020 == -1 )
      return;
    goto LABEL_39;
  }
  sub_78E0(v17, v16);
  if ( byte_DDA00
    && (fildes >= 0 && fdatasync(fildes) && *__errno_location() != 22 || fsync(dword_1B010) && *__errno_location() != 22)
    || close(dword_1B010) )
  {
    sub_D670();
  }
  if ( dword_DD9F0 )
    goto LABEL_76;
  v22 = (const char *)sub_E060(dword_1B420);
  if ( !sub_59A0(dword_1B420, v22 - dword_1B420) )
  {
    sigprocmask(0, &set, &v29);
    fd = -1;
LABEL_73:
    v23 = j__unlink(dword_1B420);
    goto LABEL_74;
  }
  v28 = fildes;
  sigprocmask(0, &set, &v29);
  fd = -1;
  if ( v28 < 0 )
    goto LABEL_73;
  v23 = unlinkat(v28, v22, 0);
LABEL_74:
  if ( v23 )
  {
    v24 = __errno_location();
    v25 = *v24;
    v26 = v24;
    sigprocmask(2, &v29, 0);
    if ( !v25 )
      goto LABEL_76;
    v27 = dword_1B85C;
    if ( dword_1B85C )
    {
      if ( status )
        goto LABEL_76;
    }
    else
    {
      sub_F630(stderr, v29.__val[0]);
      v27 = dword_1B85C;
      if ( status )
      {
LABEL_90:
        if ( !v27 )
        {
          *v26 = v25;
          perror(dword_1B420);
        }
        goto LABEL_76;
      }
    }
    status = 2;
    goto LABEL_90;
  }
  sigprocmask(2, &v29, 0);
LABEL_76:
  if ( dword_19020 != -1 )
  {
LABEL_39:
    if ( dword_DD9E4 )
    {
      if ( dword_1B858 )
      {
        sub_F630(stderr, v29.__val[0]);
      }
      else if ( dword_DD9F8 )
      {
        sub_D8E0(qword_1B820 - (qword_1B828 - qword_E0EA8), qword_1B820, stderr);
      }
      else
      {
        sub_D8E0(qword_1B828 - (qword_1B820 - qword_E0EA8), qword_1B828, stderr);
      }
      if ( !dword_1B858 )
        sub_F630(stderr, v29.__val[0]);
      sub_F630(stderr, a7);
    }
    return;
  }
  if ( !dword_1B860 )
    sub_56E0(0);
}



// Function: treat_dir @ 0x8400
void treat_dir(int fd, char *s)
{
  DIR *v2; // rax
  DIR *v3; // r12
  char *v4; // r14
  char *v5; // rax
  long long v6; // rcx
  long long v7; // rdx
  long long v8; // rcx
  long long v9; // r8
  long long v10; // r9
  size_t v11; // rbx
  bool v12; // zf
  char v13[8]; // [rsp+0h] [rbp-458h]
  char *src; // [rsp+8h] [rbp-450h]
  char v15[1032]; // [rsp+10h] [rbp-448h] BYREF
  unsigned long long v16; // [rsp+418h] [rbp-40h]

  v16 = __readfsqword(0x28u);
  v2 = fdopendir(fd);
  if ( !v2 )
  {
    sub_5820(s);
    close(fd);
    return;
  }
  v3 = v2;
  src = (char *)sub_E5B0(v2);
  if ( src )
  {
    if ( closedir(v3) )
      sub_5820(s);
    v4 = src;
    if ( !*src )
    {
LABEL_16:
      free(src);
      return;
    }
    while ( 1 )
    {
      while ( 1 )
      {
        v11 = strlen(v4);
        if ( strcmp(v4, ".") )
        {
          if ( strcmp(v4, "..") )
            break;
        }
LABEL_11:
        v4 += v11 + 1;
        if ( !*v4 )
          goto LABEL_16;
      }
      *(long long *)v13 = strlen(s);
      if ( *(long long *)v13 + v11 <= 0x3FD )
      {
        __strcpy_chk(v15, s, 1024);
        v5 = (char *)sub_E060(v15);
        v6 = *(long long *)v13;
        if ( *v5 )
        {
          if ( v15[*(long long *)v13 - 1] != 47 )
          {
            v15[*(long long *)v13] = 47;
            v6 = *(long long *)v13 + 1LL;
          }
        }
        strcpy(&v15[v6], v4);
        sub_7B60(v15, (long long)v4, v7, v8, v9, v10, v13[0]);
        goto LABEL_11;
      }
      v4 += v11 + 1;
      sub_F630(stderr, v13[0]);
      v12 = *v4 == 0;
      status = 1;
      if ( v12 )
        goto LABEL_16;
    }
  }
  sub_5820(s);
  if ( closedir(v3) )
    sub_5820(s);
}



// Function: abort_gzip @ 0x8620
void abort_gzip()
{
  sub_56E0(0);
  sub_6FD0(1);
}



// Function: inflate_stored @ 0x8640
long long inflate_stored(long long a1, long long a2)
{
  unsigned int v2; // r12d
  int v3; // ebp
  long long v4; // rcx
  unsigned long long v5; // r13
  int v6; // ebx
  unsigned int v7; // r14d
  long long v8; // rax
  long long v9; // rax
  unsigned int v10; // eax
  unsigned int v11; // r14d
  unsigned long long v12; // r13
  unsigned int v13; // r15d
  long long v14; // rax
  long long v15; // rax
  long long v16; // r8
  long long v17; // rdx
  unsigned int v18; // ebp
  unsigned long long v19; // r13
  unsigned int v20; // r14d
  long long v22; // rax
  long long v23; // rax
  long long v25; // rax
  int v26; // [rsp+Ch] [rbp-3Ch]

  v2 = dword_1B004;
  v26 = dword_DDA0C;
  v3 = dword_DDA0C & 7;
  v4 = dword_DDA0C & 7;
  v5 = (unsigned long long)qword_DDA10 >> v4;
  v6 = dword_DDA0C - v3;
  if ( (unsigned int)(dword_DDA0C - v3) <= 0xF )
  {
    v7 = dword_DDA0C - v3;
    do
    {
      v9 = (unsigned int)dword_1B008;
      if ( dword_1B008 < (unsigned int)dword_1B00C )
      {
        ++dword_1B008;
        v8 = (unsigned char)byte_9D000[v9];
      }
      else
      {
        a1 = 0;
        dword_1B004 = v2;
        v8 = (unsigned char)sub_D830(0);
      }
      v4 = v7;
      v7 += 8;
      v5 |= v8 << v4;
    }
    while ( v7 <= 0xF );
    v6 += ((v3 - v26 + 15) & 0xFFFFFFF8) + 8;
  }
  v10 = (unsigned short)v5;
  v11 = v6 - 16;
  v12 = v5 >> 16;
  v13 = v10;
  if ( (unsigned int)(v6 - 16) <= 0xF )
  {
    do
    {
      v15 = (unsigned int)dword_1B008;
      if ( dword_1B008 < (unsigned int)dword_1B00C )
      {
        ++dword_1B008;
        v14 = (unsigned char)byte_9D000[v15];
      }
      else
      {
        a1 = 0;
        dword_1B004 = v2;
        v14 = (unsigned char)sub_D830(0);
      }
      v4 = v11;
      v11 += 8;
      v12 |= v14 << v4;
    }
    while ( v11 <= 0xF );
    v11 = v6 + ((31 - v6) & 0xFFFFFFF8) - 8;
  }
  v16 = 1;
  v17 = v13;
  if ( (unsigned short)~(short)v12 == v13 )
  {
    v18 = v13 - 1;
    v19 = v12 >> 16;
    v20 = v11 - 16;
    if ( v13 )
    {
      do
      {
        if ( v20 > 7 )
        {
          v20 -= 8;
        }
        else
        {
          v25 = (unsigned int)dword_1B008;
          if ( dword_1B008 < (unsigned int)dword_1B00C )
          {
            ++dword_1B008;
            v22 = (unsigned char)byte_9D000[v25];
          }
          else
          {
            a1 = 0;
            dword_1B004 = v2;
            v22 = (unsigned char)sub_D830(0);
          }
          v4 = v20;
          v19 |= v22 << v20;
        }
        v23 = v2++;
        byte_3C000[v23] = v19;
        if ( v2 == 0x8000 )
        {
          dword_1B004 = 0x8000;
          v2 = 0;
          sub_D7E0(a1, a2, v17, v4, v16);
        }
        v19 >>= 8;
      }
      while ( v18-- != 0 );
    }
    dword_1B004 = v2;
    LODWORD(v16) = 0;
    qword_DDA10 = v19;
    dword_DDA0C = v20;
  }
  return (unsigned int)v16;
}



// Function: inflate_codes @ 0x8840
long long inflate_codes(char *a1, long long a2, unsigned int a3, unsigned int a4)
{
  unsigned long long v4; // r14
  unsigned int v5; // r13d
  unsigned int v6; // r12d
  char *v7; // rbx
  long long v8; // r8
  long long v9; // rcx
  long long v10; // rdx
  long long v11; // rax
  long long v12; // rax
  char v13; // cl
  long long v14; // r15
  long long v15; // rax
  char v16; // cl
  unsigned int v17; // r15d
  long long v18; // rax
  char v19; // cl
  unsigned int v20; // r15d
  unsigned long long v21; // rbx
  long long v22; // rax
  char v23; // cl
  unsigned char *v24; // r13
  long long v25; // r8
  unsigned int v26; // r14d
  long long v27; // rax
  char v28; // cl
  unsigned char v29; // cl
  unsigned int v30; // edx
  unsigned long long v31; // rsi
  unsigned long long v32; // r15
  unsigned int v33; // r13d
  unsigned int v34; // ebx
  long long v35; // rax
  char v36; // cl
  unsigned char v37; // al
  char v38; // cl
  int v39; // eax
  unsigned int v40; // r9d
  unsigned int v41; // r13d
  char *v42; // rsi
  long long v43; // rdx
  long long v44; // rcx
  unsigned int v45; // ebx
  unsigned int v46; // r15d
  unsigned int v47; // eax
  unsigned int v48; // eax
  long long v49; // rcx
  char *v52; // [rsp+10h] [rbp-68h]
  int v53; // [rsp+18h] [rbp-60h]
  unsigned short v54; // [rsp+1Ch] [rbp-5Ch]
  unsigned int v55; // [rsp+1Ch] [rbp-5Ch]
  unsigned short i; // [rsp+20h] [rbp-58h]
  unsigned long long v57; // [rsp+20h] [rbp-58h]
  int v59; // [rsp+30h] [rbp-48h]
  short v61; // [rsp+38h] [rbp-40h]
  unsigned char *v62; // [rsp+38h] [rbp-40h]

  v4 = qword_DDA10;
  v5 = dword_DDA0C;
  v6 = dword_1B004;
  v52 = a1;
  v53 = word_13380[a3];
  v59 = word_13380[a4];
LABEL_2:
  while ( v5 < a3 )
  {
LABEL_7:
    if ( dword_1B008 >= (unsigned int)dword_1B00C )
    {
      a1 = 0;
      dword_1B004 = v6;
      v12 = (unsigned char)sub_D830(0);
    }
    else
    {
      v12 = (unsigned char)byte_9D000[dword_1B008++];
    }
    v13 = v5;
    v5 += 8;
    v4 |= v12 << v13;
  }
  while ( 1 )
  {
    v7 = &v52[16 * ((unsigned int)v4 & v53)];
    v8 = (unsigned char)*v7;
    if ( (unsigned int)v8 > 0x10 )
    {
      while ( (int)v8 != 99 )
      {
        v14 = (unsigned int)(v8 - 16);
        v5 -= (unsigned char)v7[1];
        for ( v4 >>= v7[1]; (unsigned int)v14 > v5; v4 |= v15 << v16 )
        {
          if ( dword_1B008 < (unsigned int)dword_1B00C )
          {
            v15 = (unsigned char)byte_9D000[dword_1B008++];
          }
          else
          {
            a1 = 0;
            dword_1B004 = v6;
            v15 = (unsigned char)sub_D830(0);
          }
          v16 = v5;
          v5 += 8;
        }
        v8 = *(unsigned char *)(*((long long *)v7 + 1) + 16 * (v4 & word_13380[v14]));
        v7 = (char *)(*((long long *)v7 + 1) + 16 * (v4 & word_13380[v14]));
        if ( (unsigned int)v8 <= 0x10 )
          goto LABEL_4;
      }
      return 1;
    }
LABEL_4:
    v9 = (unsigned char)v7[1];
    v5 -= (unsigned char)v7[1];
    v4 >>= v9;
    if ( (int)v8 != 16 )
      break;
    v10 = *((unsigned short *)v7 + 4);
    v11 = v6++;
    byte_3C000[v11] = v10;
    if ( v6 != 0x8000 )
      goto LABEL_2;
    dword_1B004 = 0x8000;
    v6 = 0;
    sub_D7E0(a1, byte_3C000, v10, v9, v8);
    if ( v5 < a3 )
      goto LABEL_7;
  }
  if ( (int)v8 != 15 )
  {
    if ( (unsigned int)v8 > v5 )
    {
      v17 = v8;
      do
      {
        if ( dword_1B008 < (unsigned int)dword_1B00C )
        {
          v18 = (unsigned char)byte_9D000[dword_1B008++];
        }
        else
        {
          dword_1B004 = v6;
          v18 = (unsigned char)sub_D830(0);
        }
        v19 = v5;
        v5 += 8;
        v4 |= v18 << v19;
      }
      while ( v17 > v5 );
      LODWORD(v8) = v17;
    }
    v20 = v5 - v8;
    v54 = *((short *)v7 + 4);
    v21 = v4 >> v8;
    for ( i = word_13380[(unsigned int)v8]; a4 > v20; v21 |= v22 << v23 )
    {
      if ( dword_1B008 < (unsigned int)dword_1B00C )
      {
        v22 = (unsigned char)byte_9D000[dword_1B008++];
      }
      else
      {
        dword_1B004 = v6;
        v22 = (unsigned char)sub_D830(0);
      }
      v23 = v20;
      v20 += 8;
    }
    v24 = (unsigned char *)(a2 + 16LL * ((unsigned int)v21 & v59));
    v25 = *v24;
    if ( (unsigned int)v25 > 0x10 )
    {
      v61 = v4;
      do
      {
        if ( (int)v25 == 99 )
          return 1;
        v26 = v25 - 16;
        v20 -= v24[1];
        v21 >>= v24[1];
        if ( v20 < (int)v25 - 16 )
        {
          do
          {
            if ( dword_1B008 < (unsigned int)dword_1B00C )
            {
              v27 = (unsigned char)byte_9D000[dword_1B008++];
            }
            else
            {
              dword_1B004 = v6;
              v27 = (unsigned char)sub_D830(0);
            }
            v28 = v20;
            v20 += 8;
            v21 |= v27 << v28;
          }
          while ( v26 > v20 );
        }
        v25 = *(unsigned char *)(*((long long *)v24 + 1) + 16 * (v21 & word_13380[v26]));
        v24 = (unsigned char *)(*((long long *)v24 + 1) + 16 * (v21 & word_13380[v26]));
      }
      while ( (unsigned int)v25 > 0x10 );
      LOWORD(v4) = v61;
    }
    v29 = v24[1];
    v30 = v20 - v29;
    v31 = v21 >> v29;
    if ( (unsigned int)v25 > v30 )
    {
      v62 = v24;
      v32 = v21 >> v29;
      v33 = v25;
      v34 = v30;
      do
      {
        while ( dword_1B008 < (unsigned int)dword_1B00C )
        {
          v35 = (unsigned char)byte_9D000[dword_1B008];
          v36 = v34;
          v34 += 8;
          ++dword_1B008;
          v32 |= v35 << v36;
          if ( v33 <= v34 )
            goto LABEL_48;
        }
        dword_1B004 = v6;
        v37 = sub_D830(0);
        v38 = v34;
        v34 += 8;
        v32 |= (unsigned long long)v37 << v38;
      }
      while ( v33 > v34 );
LABEL_48:
      v25 = v33;
      v24 = v62;
      v31 = v32;
      v30 = v34;
    }
    v39 = v54;
    v55 = v30 - v25;
    v40 = (v4 & i) + v39;
    LOWORD(v39) = *((short *)v24 + 4);
    v41 = v40;
    v57 = v31 >> v25;
    LOWORD(v4) = v6 - v39 - (v31 & word_13380[(unsigned int)v25]);
    while ( 1 )
    {
      v45 = v4 & 0x7FFF;
      v46 = 0x8000 - v6;
      if ( v6 < v45 )
        v46 = 0x8000 - v45;
      if ( v41 <= v46 )
        v46 = v41;
      v42 = (char *)(v6 - v45);
      v47 = v45 - v6;
      v41 -= v46;
      if ( v6 > v45 )
        v47 = v6 - v45;
      if ( v47 >= v46 )
      {
        a1 = &byte_3C000[v6];
        v42 = &byte_3C000[v4 & 0x7FFF];
        LOWORD(v4) = v45 + v46;
        v6 += v46;
        memcpy(a1, v42, v46);
        if ( v6 != 0x8000 )
          goto LABEL_51;
      }
      else
      {
        LODWORD(v4) = v45 + v46;
        v48 = v45;
        do
        {
          v49 = v48;
          a1 = byte_3C000;
          v43 = v48 + (unsigned int)v42;
          ++v48;
          v44 = (unsigned char)byte_3C000[v49];
          byte_3C000[v43] = v44;
        }
        while ( v48 != (int)v4 );
        v6 += v46;
        if ( v6 != 0x8000 )
        {
LABEL_51:
          if ( !v41 )
            goto LABEL_63;
          continue;
        }
      }
      dword_1B004 = 0x8000;
      v6 = 0;
      sub_D7E0(a1, v42, v43, v44, v25);
      if ( !v41 )
      {
LABEL_63:
        v5 = v55;
        v4 = v57;
        goto LABEL_2;
      }
    }
  }
  dword_1B004 = v6;
  qword_DDA10 = v4;
  dword_DDA0C = v5;
  return 0;
}



// Function: huft_build @ 0x8dc0
long long huft_build(
        unsigned int *a1,
        unsigned int a2,
        unsigned int a3,
        long long a4,
        long long a5,
        long long *a6,
        unsigned int *a7)
{
  long long v7; // r13
  long long v8; // r10
  unsigned int *v11; // rsi
  unsigned int *v12; // rdx
  long long v13; // rcx
  int v14; // edx
  unsigned int v15; // r15d
  long long v16; // rax
  unsigned int v17; // ecx
  int *v18; // r9
  unsigned int v19; // r8d
  bool v20; // sf
  int v21; // esi
  int v23; // eax
  char *v24; // rdx
  char *v25; // rcx
  char *v26; // rcx
  int v27; // edx
  char *v28; // r8
  char *v29; // rax
  long long v30; // rax
  long long v31; // rdx
  long long v32; // rcx
  long long v33; // rax
  long long v34; // r11
  unsigned int v35; // ebx
  int v36; // r13d
  unsigned int v37; // r12d
  long long *v38; // rsi
  char *v39; // rax
  unsigned int v40; // r8d
  unsigned int v41; // edx
  int v42; // r10d
  unsigned int v43; // r9d
  unsigned int v44; // r13d
  long long v45; // r14
  long long v46; // rdx
  unsigned int v47; // r12d
  char *v48; // rax
  char *v49; // rcx
  long long *v50; // rdi
  unsigned int v51; // ebp
  unsigned int v52; // ecx
  unsigned int *v53; // rax
  unsigned int j; // ebx
  unsigned int v55; // edi
  unsigned int v56; // ebx
  char v57; // di
  unsigned int v58; // edx
  char *v59; // rcx
  unsigned int v60; // ecx
  char *v61; // r9
  long long v62; // rax
  long long v63; // rdx
  char *v64; // rdx
  unsigned int i; // edx
  int v66; // ecx
  long long v67; // rdx
  int v68; // r10d
  unsigned int v69; // edx
  char *v70; // rax
  short *v71; // rax
  int v72; // [rsp+4h] [rbp-65Ch]
  long long v73; // [rsp+10h] [rbp-650h]
  char v74; // [rsp+18h] [rbp-648h]
  unsigned int v75; // [rsp+1Ch] [rbp-644h]
  int v76; // [rsp+20h] [rbp-640h]
  int v77; // [rsp+24h] [rbp-63Ch]
  char v78; // [rsp+28h] [rbp-638h]
  int v79; // [rsp+2Ch] [rbp-634h]
  char *v80; // [rsp+30h] [rbp-630h]
  unsigned int *v81; // [rsp+38h] [rbp-628h]
  unsigned int *v82; // [rsp+40h] [rbp-620h]
  unsigned int v83; // [rsp+48h] [rbp-618h]
  signed int v84; // [rsp+4Ch] [rbp-614h]
  unsigned int *v85; // [rsp+50h] [rbp-610h]
  int v89; // [rsp+70h] [rbp-5F0h]
  _OWORD v90[4]; // [rsp+78h] [rbp-5E8h] BYREF
  int v91; // [rsp+B8h] [rbp-5A8h] BYREF
  int v92; // [rsp+C8h] [rbp-598h] BYREF
  int v93; // [rsp+CCh] [rbp-594h]
  char v94[72]; // [rsp+D0h] [rbp-590h] BYREF
  long long v95[16]; // [rsp+118h] [rbp-548h]
  int v96[290]; // [rsp+198h] [rbp-4C8h] BYREF
  unsigned long long v97; // [rsp+620h] [rbp-40h]

  v8 = a2;
  v11 = &a1[a2];
  v97 = __readfsqword(0x28u);
  memset(v90, 0, sizeof(v90));
  v12 = a1;
  v91 = 0;
  do
  {
    v13 = *v12++;
    ++*((int *)v90 + v13);
  }
  while ( v11 != v12 );
  v14 = v90[0];
  if ( LODWORD(v90[0]) != a2 )
  {
    v15 = *a7;
    v16 = 1;
    while ( 1 )
    {
      v17 = v16;
      if ( *((int *)v90 + v16) )
        break;
      if ( ++v16 == 17 )
      {
        v76 = 17;
        v16 = (long long)&unk_20000;
        v17 = 17;
LABEL_8:
        if ( v15 < v17 )
          v15 = v76;
        v18 = &v91;
        v19 = 16;
        while ( !*v18 )
        {
          --v18;
          if ( !--v19 )
          {
            if ( !v15 )
            {
              v20 = (int)v16 - LODWORD(v90[0]) < 0;
              v21 = v16 - LODWORD(v90[0]);
              *a7 = 0;
              v89 = v21;
              if ( !v20 )
              {
                LODWORD(v90[0]) = v16;
                v93 = 0;
                v84 = 0;
LABEL_28:
                v26 = v94;
                v27 = 0;
                v28 = (char *)v90 + 4 * v19;
                v29 = (char *)v90 + 4;
                do
                {
                  v29 += 4;
                  v26 += 4;
                  v27 += *((int *)v29 - 1);
                  *((int *)v26 - 1) = v27;
                }
                while ( v28 != v29 );
LABEL_30:
                v30 = 0;
                do
                {
                  v31 = a1[v30];
                  if ( (int)v31 )
                  {
                    v32 = (unsigned int)*(&v92 + v31);
                    v96[v32] = v30;
                    *(&v92 + v31) = v32 + 1;
                  }
                  ++v30;
                }
                while ( v8 != v30 );
                v95[0] = 0;
                v33 = (unsigned int)*(&v92 + v84);
                v92 = 0;
                if ( v84 < v76 )
                  return (v89 != 0) & (unsigned char)(v84 != 1);
                v34 = v7;
                v35 = 0;
                v36 = -v15;
                v82 = v96;
                v37 = 0;
                v38 = a6;
                v85 = &v96[v33];
                v77 = -1;
                v81 = (unsigned int *)v90 + v76;
                v39 = 0;
                while ( 1 )
                {
                  v75 = *v81;
                  if ( *v81 )
                    break;
LABEL_70:
                  ++v76;
                  ++v81;
                  if ( v76 == v84 + 1 )
                    return (v89 != 0) & (unsigned char)(v84 != 1);
                }
                v40 = v37;
                while ( 1 )
                {
                  v41 = v15 + v36;
                  if ( (int)(v15 + v36) < v76 )
                    break;
                  v78 = v76 - v36;
LABEL_54:
                  v57 = 99;
                  if ( v85 > v82 )
                  {
                    v58 = *v82++;
                    if ( v58 >= a3 )
                    {
                      v69 = v58 - a3;
                      v59 = v80;
                      LOWORD(v59) = *(short *)(a4 + 2LL * v69);
                      v57 = *(char *)(a5 + 2LL * v69);
                    }
                    else
                    {
                      v59 = v80;
                      LOWORD(v59) = v58;
                      v57 = (v58 != 256) + 15;
                    }
                    v80 = v59;
                  }
                  v60 = v40 >> v36;
                  if ( v40 >> v36 < v35 )
                  {
                    v61 = v39;
                    v62 = v34;
                    do
                    {
                      v63 = v60;
                      LOBYTE(v62) = v57;
                      v60 += 1 << v78;
                      BYTE1(v62) = v76 - v36;
                      v64 = &v61[16 * v63];
                      *(long long *)v64 = v62;
                      *((long long *)v64 + 1) = v80;
                    }
                    while ( v60 < v35 );
                    v34 = v62;
                    v39 = v61;
                  }
                  for ( i = 1 << (v76 - 1); (i & v40) != 0; i >>= 1 )
                    v40 ^= i;
                  v40 ^= i;
                  if ( (v40 & ((1 << v36) - 1)) != *(&v92 + v77) )
                  {
                    v66 = v36;
                    v67 = v77 - 1;
                    do
                    {
                      v66 -= v15;
                      v68 = v67--;
                    }
                    while ( (v40 & ((1 << v66) - 1)) != *(&v92 + v67 + 1) );
                    v77 = v68;
                    v36 = v66;
                  }
                  if ( !--v75 )
                  {
                    v37 = v40;
                    goto LABEL_70;
                  }
                }
                v42 = v15 + v36;
                v83 = v40;
                v43 = v84 - v41;
                v44 = v76 - v41;
                v45 = v77 + 1;
                v46 = v34;
                v47 = v43;
                while ( 1 )
                {
                  v79 = v42;
                  v77 = v45;
                  v35 = 1 << v44;
                  LOBYTE(v51) = v44;
                  v74 = v42 - v15;
                  v78 = v44;
                  if ( 1 << v44 > v75 )
                  {
                    v52 = v15;
                    if ( v47 <= v15 )
                      v52 = v47;
                    if ( v52 > v44 )
                    {
                      v51 = v44 + 1;
                      v53 = v81;
                      for ( j = v35 - v75; v51 < v52; j = v56 - v55 )
                      {
                        v55 = v53[1];
                        v56 = 2 * j;
                        ++v53;
                        if ( v55 >= v56 )
                          break;
                        ++v51;
                      }
                      v35 = 1 << v51;
                    }
                  }
                  v73 = v46;
                  v72 = v42;
                  v48 = (char *)malloc(16LL * (v35 + 1));
                  v46 = v73;
                  v49 = v48;
                  if ( !v48 )
                    break;
                  dword_DDA08 += v35 + 1;
                  v39 = v48 + 16;
                  *v38 = v49 + 16;
                  v38 = v49 + 8;
                  *((long long *)v49 + 1) = 0;
                  v95[v45] = v49 + 16;
                  if ( (int)v45 )
                  {
                    v80 = v49 + 16;
                    LOBYTE(v46) = v51 + 16;
                    *(&v92 + v45) = v83;
                    v50 = (long long *)(16LL * (v83 >> v74) + *(long long *)&v94[8 * v45 + 64]);
                    BYTE1(v46) = v15;
                    v50[1] = v39;
                    *v50 = v46;
                  }
                  ++v45;
                  v42 = v15 + v72;
                  v44 -= v15;
                  v47 -= v15;
                  if ( v76 <= (int)(v15 + v72) )
                  {
                    v36 = v79;
                    v40 = v83;
                    v34 = v46;
                    goto LABEL_54;
                  }
                }
                if ( (int)v45 )
                  sub_8D90(v95[0], v38, v73, 0);
                return 3;
              }
              return 2;
            }
            *a7 = 0;
            v15 = 0;
            v84 = 0;
            goto LABEL_26;
          }
        }
        v84 = v19;
        if ( v15 > v19 )
          v15 = v19;
        *a7 = v15;
        if ( v17 >= v19 )
        {
          v89 = v16 - *((int *)v90 + v19);
          if ( v89 >= 0 )
          {
            *((int *)v90 + v19) = v16;
            v93 = 0;
            if ( v19 != 1 )
              goto LABEL_28;
            goto LABEL_30;
          }
          return 2;
        }
        v23 = v16 - *((int *)v90 + v17);
        if ( v23 < 0 )
          return 2;
        v24 = (char *)v90 + 4 * v17;
        v25 = (char *)v90 + 4 * v17 + 4 * (unsigned long long)(v19 - v17 - 1);
        while ( 1 )
        {
          LODWORD(v16) = 2 * v23;
          if ( v25 == v24 )
            break;
          v23 = v16 - *((int *)v24 + 1);
          v24 += 4;
          if ( v23 < 0 )
            return 2;
        }
        v14 = *((int *)v90 + v19);
LABEL_26:
        v89 = v16 - v14;
        if ( (int)v16 - v14 >= 0 )
        {
          v93 = 0;
          *((int *)v90 + v19) = v16;
          goto LABEL_28;
        }
        return 2;
      }
    }
    v76 = v16;
    LODWORD(v16) = 1 << v16;
    goto LABEL_8;
  }
  v70 = (char *)malloc(0x30u);
  if ( !v70 )
    return 3;
  v71 = v70 + 16;
  *((long long *)v71 - 1) = 0;
  *v71 = 355;
  v71[8] = 355;
  dword_DDA08 += 3;
  *a6 = v71;
  *a7 = 1;
  return 0;
}



// Function: inflate_dynamic @ 0x94b0
long long inflate_dynamic()
{
  int v0; // r13d
  unsigned long long v1; // rbx
  int v2; // ebp
  long long v3; // rax
  long long v4; // rax
  char v5; // cl
  char v6; // al
  int v7; // r12d
  unsigned long long v8; // rbx
  long long v9; // rax
  long long v10; // rax
  char v11; // al
  int v12; // r14d
  unsigned long long v13; // rbx
  unsigned long long v14; // r13
  long long v15; // rax
  long long v16; // rax
  char v17; // al
  unsigned int v18; // r14d
  unsigned long long v19; // r13
  char *v20; // r12
  unsigned int v21; // eax
  char *v22; // rbx
  long long v23; // rax
  char v24; // cl
  long long v25; // rax
  char v26; // cl
  unsigned int i; // ebx
  long long v28; // rax
  long long v29; // rsi
  long long v30; // rdx
  unsigned int v31; // r12d
  long long v32; // rdx
  unsigned int v33; // ebx
  long long v34; // rax
  char v35; // cl
  char *v37; // rax
  int v38; // ecx
  unsigned int v39; // eax
  long long v40; // rcx
  long long v41; // rax
  char v42; // cl
  char v43; // di
  unsigned int v44; // edi
  unsigned int *v45; // rax
  long long v46; // rax
  char v47; // cl
  char v48; // di
  unsigned int v49; // edi
  unsigned int *v50; // rax
  long long v51; // rax
  char v52; // cl
  char v53; // di
  unsigned int v54; // edi
  unsigned int *v55; // rax
  long long v56; // rsi
  long long v57; // rdx
  long long v58; // rdx
  long long v59; // [rsp-8h] [rbp-590h]
  unsigned int v60; // [rsp+0h] [rbp-588h]
  unsigned int v61; // [rsp+0h] [rbp-588h]
  unsigned int v62; // [rsp+4h] [rbp-584h]
  int v63; // [rsp+8h] [rbp-580h]
  unsigned int v64; // [rsp+Ch] [rbp-57Ch]
  unsigned int v65; // [rsp+18h] [rbp-570h]
  unsigned int v66; // [rsp+1Ch] [rbp-56Ch]
  signed int v67; // [rsp+38h] [rbp-550h] BYREF
  unsigned int v68; // [rsp+3Ch] [rbp-54Ch] BYREF
  char *v69; // [rsp+40h] [rbp-548h] BYREF
  char *v70; // [rsp+48h] [rbp-540h] BYREF
  unsigned int v71[318]; // [rsp+50h] [rbp-538h] BYREF
  unsigned long long v72; // [rsp+548h] [rbp-40h]

  v0 = dword_DDA0C;
  v1 = qword_DDA10;
  v72 = __readfsqword(0x28u);
  v2 = dword_1B004;
  if ( (unsigned int)dword_DDA0C <= 4 )
  {
    v3 = (unsigned int)dword_1B008;
    if ( dword_1B008 >= (unsigned int)dword_1B00C )
    {
      v4 = (unsigned char)sub_D830(0);
    }
    else
    {
      ++dword_1B008;
      v4 = (unsigned char)byte_9D000[v3];
    }
    v5 = v0;
    v0 += 8;
    v1 |= v4 << v5;
  }
  v6 = v1;
  v7 = v0 - 5;
  v8 = v1 >> 5;
  v65 = (v6 & 0x1F) + 257;
  if ( (unsigned int)(v0 - 5) <= 4 )
  {
    v9 = (unsigned int)dword_1B008;
    if ( dword_1B008 >= (unsigned int)dword_1B00C )
    {
      dword_1B004 = v2;
      v10 = (unsigned char)sub_D830(0);
    }
    else
    {
      ++dword_1B008;
      v10 = (unsigned char)byte_9D000[v9];
    }
    v7 = v0 + 3;
    v8 |= v10 << ((unsigned char)v0 - 5);
  }
  v11 = v8;
  v12 = v7 - 5;
  v13 = v8 >> 5;
  v14 = v13;
  v66 = (v11 & 0x1F) + 1;
  if ( (unsigned int)(v7 - 5) <= 3 )
  {
    v15 = (unsigned int)dword_1B008;
    if ( dword_1B008 >= (unsigned int)dword_1B00C )
    {
      dword_1B004 = v2;
      v16 = (unsigned char)sub_D830(0);
    }
    else
    {
      ++dword_1B008;
      v16 = (unsigned char)byte_9D000[v15];
    }
    v12 = v7 + 3;
    v14 = (v16 << ((unsigned char)v7 - 5)) | v13;
  }
  if ( v65 > 0x11E || v66 > 0x1E )
    return 1;
  v17 = v14;
  v18 = v12 - 4;
  v19 = v14 >> 4;
  v20 = (char *)dword_133C0;
  v21 = v17 & 0xF;
  v60 = v21 + 4;
  v22 = (char *)&dword_133C0[v21 + 4];
  do
  {
    if ( v18 <= 2 )
    {
      if ( dword_1B008 < (unsigned int)dword_1B00C )
      {
        v23 = (unsigned char)byte_9D000[dword_1B008++];
      }
      else
      {
        dword_1B004 = v2;
        v23 = (unsigned char)sub_D830(0);
      }
      v24 = v18;
      v18 += 8;
      v19 |= v23 << v24;
    }
    v25 = *(unsigned int *)v20;
    v26 = v19;
    v20 += 4;
    v19 >>= 3;
    v18 -= 3;
    v71[v25] = v26 & 7;
  }
  while ( v22 != v20 );
  for ( i = v60; i != 19; v71[dword_133C0[v28]] = 0 )
    v28 = i++;
  v67 = 7;
  v29 = 19;
  v31 = sub_8DC0(v71, 0x13u, 0x13u, 0, 0, &v69, (unsigned int *)&v67);
  if ( !v31 )
  {
    if ( !v69 )
      return 2;
    v32 = v67;
    v64 = 0;
    v61 = 0;
    v62 = v66 + v65;
    v33 = v67;
    v63 = word_13380[v67];
    do
    {
      while ( v18 < v33 )
      {
        if ( dword_1B008 < (unsigned int)dword_1B00C )
        {
          v32 = (unsigned int)(dword_1B008 + 1);
          v34 = (unsigned char)byte_9D000[dword_1B008++];
        }
        else
        {
          dword_1B004 = v2;
          v34 = (unsigned char)sub_D830(0);
        }
        v35 = v18;
        v18 += 8;
        v19 |= v34 << v35;
      }
      v37 = &v69[16 * ((unsigned int)v19 & v63)];
      v70 = v37;
      v38 = (unsigned char)v37[1];
      v19 >>= v38;
      v18 -= v38;
      if ( *v37 == 99 )
      {
        sub_8D90(v69, 19, v32);
        return 2;
      }
      v39 = *((unsigned short *)v37 + 4);
      if ( v39 > 0xF )
      {
        if ( v39 == 16 )
        {
          if ( v18 <= 1 )
          {
            if ( dword_1B008 >= (unsigned int)dword_1B00C )
            {
              dword_1B004 = v2;
              v51 = (unsigned char)sub_D830(0);
            }
            else
            {
              v51 = (unsigned char)byte_9D000[dword_1B008++];
            }
            v52 = v18;
            v18 += 8;
            v19 |= v51 << v52;
          }
          v53 = v19;
          v19 >>= 2;
          v18 -= 2;
          v54 = v53 & 3;
          if ( v54 + v61 + 3 > v62 )
            return 1;
          v32 = v64;
          v55 = &v71[v31];
          do
            *v55++ = v64;
          while ( &v71[(int)v31 + 3 + (unsigned long long)v54] != v55 );
          v31 += v54 + 3;
        }
        else if ( v39 == 17 )
        {
          if ( v18 <= 2 )
          {
            if ( dword_1B008 >= (unsigned int)dword_1B00C )
            {
              dword_1B004 = v2;
              v46 = (unsigned char)sub_D830(0);
            }
            else
            {
              v46 = (unsigned char)byte_9D000[dword_1B008++];
            }
            v47 = v18;
            v18 += 8;
            v19 |= v46 << v47;
          }
          v48 = v19;
          v19 >>= 3;
          v18 -= 3;
          v49 = v48 & 7;
          if ( v49 + v61 + 3 > v62 )
            return 1;
          v50 = &v71[v31];
          do
            *v50++ = 0;
          while ( &v71[(int)v31 + 3 + (unsigned long long)v49] != v50 );
          v64 = 0;
          v31 += v49 + 3;
        }
        else
        {
          if ( v18 <= 6 )
          {
            if ( dword_1B008 >= (unsigned int)dword_1B00C )
            {
              dword_1B004 = v2;
              v41 = (unsigned char)sub_D830(0);
            }
            else
            {
              v41 = (unsigned char)byte_9D000[dword_1B008++];
            }
            v42 = v18;
            v18 += 8;
            v19 |= v41 << v42;
          }
          v43 = v19;
          v19 >>= 7;
          v18 -= 7;
          v44 = v43 & 0x7F;
          if ( v44 + v61 + 11 > v62 )
            return 1;
          v45 = &v71[v31];
          do
            *v45++ = 0;
          while ( &v71[(int)v31 + 11 + (unsigned long long)v44] != v45 );
          v64 = 0;
          v31 += v44 + 11;
        }
      }
      else
      {
        v40 = (int)v31;
        v64 = v39;
        ++v31;
        v71[v40] = v39;
      }
      v61 = v31;
    }
    while ( v31 < v62 );
    sub_8D90(v69, 19, v32);
    v67 = 9;
    qword_DDA10 = v19;
    dword_DDA0C = v18;
    v29 = v59;
    v31 = sub_8DC0(v71, v65, 0x101u, (long long)&unk_191A0, (long long)&unk_19160, &v69, (unsigned int *)&v67);
    if ( v31 )
      goto LABEL_35;
    v68 = 6;
    v29 = v66;
    v31 = sub_8DC0(&v71[v65], v66, 0, (long long)&unk_19120, (long long)&unk_190E0, &v70, &v68);
    v30 = v59;
    if ( !v31 )
    {
      v56 = (long long)v70;
      v31 = sub_8840(v69, (long long)v70, v67, v68) != 0;
      sub_8D90(v69, v56, v57);
      sub_8D90(v70, v56, v58);
      return v31;
    }
    if ( v31 == 1 )
      sub_8D90(v70, v66, v59);
    goto LABEL_45;
  }
LABEL_35:
  if ( v31 == 1 )
LABEL_45:
    sub_8D90(v69, v29, v30);
  return v31;
}



// Function: inflate_fixed @ 0x9b70
long long inflate_fixed()
{
  unsigned int *v0; // rbx
  unsigned int *v1; // rax
  long long *v2; // rax
  long long result; // rax
  long long v4; // rsi
  long long v5; // rdx
  long long v6; // rdx
  long long v7; // [rsp-10h] [rbp-4E8h]
  int v8; // [rsp+Ch] [rbp-4CCh]
  unsigned int v9; // [rsp+18h] [rbp-4C0h] BYREF
  unsigned int v10; // [rsp+1Ch] [rbp-4BCh] BYREF
  char *v11; // [rsp+20h] [rbp-4B8h] BYREF
  long long v12; // [rsp+28h] [rbp-4B0h] BYREF
  unsigned int v13[30]; // [rsp+30h] [rbp-4A8h] BYREF
  long long v14; // [rsp+A8h] [rbp-430h] BYREF
  char v15; // [rsp+270h] [rbp-268h] BYREF
  char v16[96]; // [rsp+430h] [rbp-A8h] BYREF
  long long v17[9]; // [rsp+490h] [rbp-48h] BYREF

  v17[5] = __readfsqword(0x28u);
  v0 = v13;
  v1 = v13;
  do
    *v1++ = 8;
  while ( v1 != (unsigned int *)&v15 );
  do
    *v1++ = 9;
  while ( v16 != (char *)v1 );
  v2 = v16;
  do
  {
    *(int *)v2 = 7;
    v2 = (long long *)((char *)v2 + 4);
  }
  while ( v17 != v2 );
  v9 = 7;
  v17[0] = 0x800000008LL;
  v17[1] = 0x800000008LL;
  v17[2] = 0x800000008LL;
  v17[3] = 0x800000008LL;
  result = sub_8DC0(v13, 0x120u, 0x101u, (long long)&unk_191A0, (long long)&unk_19160, &v11, &v9);
  if ( !(int)result )
  {
    do
      *v0++ = 5;
    while ( v0 != (unsigned int *)&v14 );
    v10 = 5;
    v8 = sub_8DC0(v13, 0x1Eu, 0, (long long)&unk_19120, (long long)&unk_190E0, &v12, &v10);
    if ( v8 > 1 )
    {
      sub_8D90(v11, 30, v7);
      return (unsigned int)v8;
    }
    else
    {
      v4 = v12;
      if ( (unsigned int)sub_8840(v11, v12, v9, v10) )
      {
        return 1;
      }
      else
      {
        sub_8D90(v11, v4, v5);
        sub_8D90(v12, v4, v6);
        return 0;
      }
    }
  }
  return result;
}



// Function: inflate_block @ 0x9d20
long long inflate_block(int *a1, long long a2)
{
  int *v2; // r12
  int v3; // ebp
  int v4; // edx
  long long v5; // r13
  long long v6; // rax
  long long v7; // rbx
  unsigned long long v8; // rbx
  char v9; // al
  unsigned long long v10; // rbx
  int v11; // eax
  int v12; // edx
  char v14; // r12
  long long v15; // rax
  long long v16; // rax

  v2 = a1;
  v3 = dword_DDA0C;
  v4 = dword_1B004;
  v5 = qword_DDA10;
  if ( dword_DDA0C )
  {
    v14 = dword_DDA0C - 1;
    v10 = (unsigned long long)qword_DDA10 >> 1;
    *a1 = qword_DDA10 & 1;
    if ( (unsigned int)(v3 - 1) > 1 )
    {
      v11 = v3 - 3;
    }
    else
    {
      v15 = (unsigned int)dword_1B008;
      if ( dword_1B008 >= (unsigned int)dword_1B00C )
      {
        a1 = 0;
        dword_1B004 = v4;
        v16 = (unsigned char)sub_D830(0);
      }
      else
      {
        ++dword_1B008;
        v16 = (unsigned char)byte_9D000[v15];
      }
      v10 |= v16 << v14;
      v11 = v3 + 5;
    }
  }
  else
  {
    v6 = (unsigned int)dword_1B008;
    if ( dword_1B008 >= (unsigned int)dword_1B00C )
    {
      a1 = 0;
      v7 = (unsigned char)sub_D830(0);
    }
    else
    {
      ++dword_1B008;
      v7 = (unsigned char)byte_9D000[v6];
    }
    v8 = v5 | v7;
    v9 = v8;
    v10 = v8 >> 1;
    *v2 = v9 & 1;
    v11 = 5;
  }
  dword_DDA0C = v11;
  v12 = v10 & 3;
  qword_DDA10 = v10 >> 2;
  if ( v12 == 2 )
    return sub_94B0();
  if ( (v10 & 3) == 0 )
    return sub_8640((long long)a1, a2);
  if ( v12 == 1 )
    return sub_9B70();
  return 2;
}



// Function: inflate @ 0x9e80
long long inflate(long long a1, long long a2)
{
  long long result; // rax
  long long v3; // rcx
  long long v4; // r8
  long long v5; // rdx
  unsigned int v6; // ecx
  int v7; // [rsp+14h] [rbp-14h] BYREF
  unsigned long long v8; // [rsp+18h] [rbp-10h]

  v8 = __readfsqword(0x28u);
  dword_1B004 = 0;
  dword_DDA0C = 0;
  qword_DDA10 = 0;
  while ( 1 )
  {
    dword_DDA08 = 0;
    result = sub_9D20(&v7, a2);
    if ( (int)result )
      break;
    if ( v7 )
    {
      v5 = (unsigned int)dword_DDA0C;
      if ( (unsigned int)dword_DDA0C > 7 )
      {
        v6 = dword_DDA0C - 8;
        v5 = dword_DDA0C & 7;
        dword_DDA0C &= 7u;
        v3 = ~(v6 >> 3);
        dword_1B008 += v3;
      }
      sub_D7E0(&v7, a2, v5, v3, v4);
      return 0;
    }
  }
  return result;
}



// Function: init_block @ 0x9f30
long long init_block()
{
  short *v0; // rax
  short *v1; // rax
  char *v2; // rax

  v0 = &unk_E0080;
  do
  {
    v0 += 2;
    *(v0 - 2) = 0;
  }
  while ( v0 != (short *)&unk_E04F8 );
  v1 = &unk_DFF80;
  do
  {
    v1 += 2;
    *(v1 - 2) = 0;
  }
  while ( v1 != (short *)&unk_DFFF8 );
  v2 = &unk_DF9E0;
  do
  {
    v2 += 4;
    *((short *)v2 - 2) = 0;
  }
  while ( v2 != algn_DFA2C );
  qword_DDA38 = 0;
  word_E0480 = 1;
  qword_DDA40 = 0;
  dword_DDA4C = 0;
  dword_DDA50 = 0;
  dword_DDA54 = 0;
  byte_DDA49 = 0;
  byte_DDA48 = 1;
  return 1;
}



// Function: pqdownheap @ 0x9fe0
long long pqdownheap(long long a1, int a2)
{
  long long v2; // r8
  int v3; // r9d
  int v4; // edx
  unsigned int v5; // eax
  long long v6; // r15
  unsigned short v7; // r12
  long long v8; // r10
  int v9; // eax
  unsigned short v10; // si
  int v11; // ecx
  long long v12; // r13
  unsigned short v13; // bp
  unsigned int v15; // [rsp+0h] [rbp-34h]

  v2 = a2;
  v3 = dword_DF0A4;
  v4 = 2 * a2;
  v5 = *((int *)&unk_DF0C0 + a2);
  v15 = v5;
  if ( 2 * a2 <= dword_DF0A4 )
  {
    v6 = (int)v5;
    v7 = *(short *)(a1 + 4LL * (int)v5);
    do
    {
      v8 = *((int *)&unk_DF0C0 + v4);
      v9 = v4;
      v10 = *(short *)(a1 + 4 * v8);
      v11 = *((int *)&unk_DF0C0 + v4);
      if ( v3 > v4 )
      {
        v9 = v4 + 1;
        v12 = *((int *)&unk_DF0C0 + v4 + 1);
        v13 = *(short *)(a1 + 4 * v12);
        if ( v13 < v10 )
        {
          v10 = *(short *)(a1 + 4 * v12);
          v11 = *((int *)&unk_DF0C0 + v4 + 1);
        }
        else if ( v13 == v10 )
        {
          if ( byte_DEE60[v12] > byte_DEE60[v8] )
            v9 = v4;
          else
            v11 = *((int *)&unk_DF0C0 + v4 + 1);
        }
        else
        {
          v9 = v4;
        }
      }
      if ( v7 < v10 || v7 == v10 && byte_DEE60[v6] <= byte_DEE60[v11] )
        break;
      v4 = 2 * v9;
      *((int *)&unk_DF0C0 + v2) = v11;
      v2 = v9;
    }
    while ( 2 * v9 <= v3 );
  }
  *((int *)&unk_DF0C0 + v2) = v15;
  return v15;
}



// Function: gen_bitlen @ 0xa0c0
long long gen_bitlen(long long *a1)
{
  long long v1; // r9
  int v2; // r15d
  int v3; // r8d
  long long v4; // r14
  unsigned short *v5; // rax
  long long v6; // rcx
  long long result; // rax
  char v8; // si
  int v9; // r12d
  long long v10; // r13
  int *v11; // r11
  long long v12; // rdx
  unsigned short *v13; // rbx
  long long v14; // rsi
  int v15; // ebp
  int v16; // ebp
  int v17; // ebx
  unsigned short v18; // cx
  long long v19; // r11
  char *v20; // rdx
  int v21; // eax
  long long v22; // rsi
  long long v23; // rsi
  char v24; // bp
  int v25; // edx
  int v26; // r12d
  long long v27; // rcx
  unsigned short *v28; // r13
  long long v29; // r14
  long long v30; // rcx
  int v31; // [rsp+0h] [rbp-58h]
  char v32; // [rsp+17h] [rbp-41h]
  long long v33; // [rsp+18h] [rbp-40h]
  long long v34; // [rsp+20h] [rbp-38h]

  v1 = *a1;
  v2 = *((int *)a1 + 9);
  v3 = *((int *)a1 + 8);
  v4 = a1[1];
  v34 = a1[2];
  v31 = *((int *)a1 + 6);
  v5 = word_DF9C0;
  do
    *v5++ = 0;
  while ( v5 != (unsigned short *)&unk_DF9E0 );
  v6 = dword_DF0A0;
  result = dword_DF0C0[dword_DF0A0];
  *(short *)(v1 + 4 * result + 2) = 0;
  if ( (int)v6 <= 571 )
  {
    v8 = 0;
    v9 = 0;
    v32 = 0;
    v10 = qword_DDA40;
    v11 = &dword_DF0C0[v6];
    v33 = qword_DDA38;
    do
    {
      v12 = v11[1];
      v13 = (unsigned short *)(v1 + 4 * v12);
      result = (unsigned int)*(unsigned short *)(v1 + 4LL * v13[1] + 2) + 1;
      if ( v3 < (int)result )
      {
        ++v9;
        result = (unsigned int)v3;
      }
      v13[1] = result;
      if ( v2 >= (int)v12 )
      {
        v14 = (int)result;
        v15 = 0;
        ++word_DF9C0[(int)result];
        if ( v31 <= (int)v12 )
        {
          v15 = *(int *)(v34 + 4LL * ((int)v12 - v31));
          v14 = v15 + (int)result;
        }
        result = *v13;
        v10 += result * v14;
        v8 = 1;
        if ( v4 )
        {
          v32 = 1;
          v33 += result * (v15 + *(unsigned short *)(v4 + 4 * v12 + 2));
        }
      }
      ++v11;
    }
    while ( v11 != &dword_DF0C4[v6 + (unsigned int)(571 - v6)] );
    if ( v32 )
    {
      result = v33;
      qword_DDA38 = v33;
    }
    if ( v8 )
      qword_DDA40 = v10;
    if ( v9 )
    {
      v16 = v9 - 2;
      v17 = v3 - 1;
      v18 = word_DF9C0[v3 - 1];
      v19 = v3;
      if ( v18 )
        goto LABEL_23;
LABEL_19:
      v20 = (char *)&word_DF9C0[v3];
      v21 = v3 - 1;
      do
      {
        v18 = *((short *)v20 - 2);
        v22 = v21;
        v20 -= 2;
        --v21;
      }
      while ( !v18 );
      while ( 1 )
      {
        word_DF9C0[v21] = v18 - 1;
        word_DF9C0[v22] += 2;
        result = (unsigned int)word_DF9C0[v3]-- - 1;
        if ( v16 == v9 - 2 - ((v9 - 1) & 0xFFFFFFFE) )
          break;
        v18 = word_DF9C0[v17];
        v16 -= 2;
        if ( !v18 )
          goto LABEL_19;
LABEL_23:
        v22 = v3;
        v21 = v3 - 1;
      }
      if ( v3 )
      {
        v23 = qword_DDA40;
        v24 = 0;
        v25 = 573;
        v26 = (unsigned short)result;
        while ( 1 )
        {
          if ( v26 )
          {
            result = v25 - 1;
            do
            {
              while ( 1 )
              {
                v27 = dword_DF0C0[result];
                v25 = result;
                if ( v2 >= (int)v27 )
                  break;
                --result;
              }
              v28 = (unsigned short *)(v1 + 4 * v27);
              v29 = v28[1];
              if ( (int)v29 != v3 )
              {
                v30 = *v28;
                v28[1] = v3;
                v24 = 1;
                v23 += (v19 - v29) * v30;
              }
              --result;
              --v26;
            }
            while ( v26 );
          }
          v3 = v17;
          --v19;
          if ( !v17 )
            break;
          v26 = word_DF9C0[v19];
          --v17;
        }
        if ( v24 )
          qword_DDA40 = v23;
      }
    }
  }
  return result;
}



// Function: scan_tree @ 0xa370
long long scan_tree(long long a1, int a2)
{
  int v2; // edx
  long long result; // rax
  int v4; // r9d
  int v5; // r8d
  long long v6; // r11
  unsigned short *v7; // rcx
  int v8; // edi
  int v9; // esi

  v2 = *(unsigned short *)(a1 + 2);
  result = a2;
  *(short *)(a1 + 4LL * a2 + 6) = -1;
  v4 = 4 - (v2 == 0);
  v5 = v2 == 0 ? 138 : 7;
  if ( a2 >= 0 )
  {
    v6 = a1 + 4LL * a2 + 10;
    v7 = (unsigned short *)(a1 + 6);
    LODWORD(result) = 0;
    v8 = -1;
    while ( 1 )
    {
      result = (unsigned int)(result + 1);
      v9 = v2;
      v2 = *v7;
      if ( (int)result < v5 )
      {
        if ( v2 == v9 )
          goto LABEL_10;
        if ( (int)result < v4 )
        {
          word_DF9E0[2 * v9] += result;
          if ( v2 )
            goto LABEL_15;
LABEL_17:
          v8 = v9;
          v4 = 3;
          v5 = 138;
          result = 0;
          goto LABEL_10;
        }
      }
      if ( v9 )
      {
        if ( v9 != v8 )
          ++word_DF9E0[2 * v9];
        ++word_DFA20;
      }
      else if ( (int)result > 10 )
      {
        ++word_DFA28;
      }
      else
      {
        ++word_DFA24;
      }
      if ( !v2 )
        goto LABEL_17;
      if ( v2 == v9 )
      {
        v8 = v2;
        v4 = 3;
        v5 = 6;
        result = 0;
LABEL_10:
        v7 += 2;
        if ( (unsigned short *)v6 == v7 )
          return result;
      }
      else
      {
LABEL_15:
        v7 += 2;
        v8 = v9;
        v4 = 4;
        result = 0;
        v5 = 7;
        if ( (unsigned short *)v6 == v7 )
          return result;
      }
    }
  }
  return result;
}



// Function: set_file_type @ 0xa480
long long set_file_type()
{
  unsigned int v0; // edx
  unsigned short *v1; // rax
  int v2; // ecx
  unsigned short *v3; // rax
  unsigned int v4; // ecx
  int v5; // esi
  short *v6; // rax
  int v7; // esi
  long long result; // rax

  v0 = 0;
  v1 = (unsigned short *)((char *)&unk_E009C - 28);
  do
  {
    v2 = *v1;
    v1 += 2;
    v0 += v2;
  }
  while ( v1 != (unsigned short *)&unk_E009C );
  v3 = (unsigned short *)&unk_E009C;
  v4 = 0;
  do
  {
    v5 = *v3;
    v3 += 2;
    v4 += v5;
  }
  while ( &unk_E0280 != (_UNKNOWN *)v3 );
  v6 = (short *)&unk_E0280;
  do
  {
    v7 = (unsigned short)*v6;
    v6 += 2;
    v0 += v7;
  }
  while ( &word_E0480 != v6 );
  result = qword_DDA20;
  *(short *)qword_DDA20 = v4 >> 2 >= v0;
  return result;
}



// Function: gen_codes @ 0xa500
unsigned long long gen_codes(short *a1, int a2)
{
  long long v2; // rdx
  int v3; // eax
  short *v4; // rbx
  long long v5; // rbp
  int v6; // esi
  unsigned int v7; // edi
  short v9[20]; // [rsp+0h] [rbp-48h]
  unsigned long long v10; // [rsp+28h] [rbp-20h]

  v2 = 0;
  v10 = __readfsqword(0x28u);
  v3 = 0;
  do
  {
    LOWORD(v3) = word_DF9C0[v2] + v3;
    v3 *= 2;
    v9[++v2] = v3;
  }
  while ( v2 != 15 );
  if ( a2 >= 0 )
  {
    v4 = a1;
    v5 = (long long)&a1[2 * a2 + 2];
    do
    {
      v6 = (unsigned short)v4[1];
      if ( v4[1] )
      {
        v7 = (unsigned short)v9[(unsigned short)v4[1]];
        v9[(unsigned short)v4[1]] = v7 + 1;
        *v4 = sub_4380(v7, v6);
      }
      v4 += 2;
    }
    while ( v4 != (short *)v5 );
  }
  return v10 - __readfsqword(0x28u);
}



// Function: build_tree @ 0xa5a0
unsigned long long build_tree(long long a1)
{
  int v1; // eax
  long long v2; // rbp
  long long v3; // r8
  long long v4; // r12
  char v5; // dl
  long long v6; // rax
  int v7; // esi
  int v8; // ecx
  int v9; // edi
  long long v10; // rdx
  char v11; // r10
  long long v12; // r11
  long long v13; // r9
  short *v14; // r15
  long long v15; // rcx
  long long v16; // rax
  int v17; // r14d
  long long v18; // r14
  long long v19; // rax
  long long v20; // r12
  short *v21; // r15
  long long v22; // rbp
  long long v23; // rax
  short *v24; // r9
  short *v25; // r10
  short v26; // si
  unsigned char v27; // si
  unsigned char v28; // dl
  char v29; // al
  short v30; // ax
  int v31; // eax
  short v33; // [rsp+8h] [rbp-50h]
  int v34; // [rsp+8h] [rbp-50h]
  short v35; // [rsp+Eh] [rbp-4Ah]
  int v36; // [rsp+10h] [rbp-48h]
  int v37; // [rsp+14h] [rbp-44h]

  v1 = *(int *)(a1 + 28);
  v2 = *(long long *)a1;
  dword_DF0A4 = 0;
  v3 = *(long long *)(a1 + 8);
  v33 = v1;
  dword_DF0A0 = 573;
  if ( v1 <= 0 )
  {
    v36 = -1;
    v4 = v1;
    v7 = 0;
  }
  else
  {
    v4 = v1;
    v5 = 0;
    v6 = 0;
    v7 = 0;
    v8 = -1;
    do
    {
      while ( *(short *)(v2 + 4 * v6) )
      {
        ++v7;
        byte_DEE60[v6] = 0;
        v8 = v6;
        dword_DF0C0[v7] = v6++;
        v5 = 1;
        if ( v6 == v4 )
          goto LABEL_6;
      }
      *(short *)(v2 + 4 * v6++ + 2) = 0;
    }
    while ( v6 != v4 );
LABEL_6:
    v36 = v8;
    if ( v5 )
    {
      dword_DF0A4 = v7;
      if ( v7 > 1 )
      {
        v37 = v7;
        goto LABEL_17;
      }
    }
    else
    {
      v7 = 0;
    }
  }
  v9 = v36;
  v10 = v7;
  v11 = 0;
  v12 = qword_DDA40;
  v13 = qword_DDA38;
  do
  {
    if ( v9 > 1 )
    {
      v14 = (short *)v2;
      v15 = 0;
      v16 = 0;
    }
    else
    {
      v16 = ++v9;
      v15 = 4LL * v9;
      v14 = (short *)(v2 + v15);
    }
    dword_DF0C4[v10] = v16;
    *v14 = 1;
    byte_DEE60[v16] = 0;
    if ( v3 )
    {
      v11 = 1;
      v13 -= *(unsigned short *)(v3 + v15 + 2);
    }
    ++v10;
  }
  while ( (int)v10 <= 1 );
  v36 = v9;
  v37 = 2;
  dword_DF0A4 = 2;
  qword_DDA40 = v12 - (unsigned int)(1 - v7) - 1;
  if ( v11 )
    qword_DDA38 = v13;
LABEL_17:
  *(int *)(a1 + 36) = v36;
  v17 = v37 >> 1;
  do
    sub_9FE0(v2, v17--);
  while ( v17 );
  v18 = v37;
  v19 = v4;
  v20 = 571;
  v21 = (short *)v2;
  v22 = v19;
  v35 = v37 + v33;
  do
  {
    dword_DF0A4 = v18 - 1;
    v34 = dword_DF0C4[0];
    dword_DF0C4[0] = dword_DF0C0[v18];
    sub_9FE0((long long)v21, 1);
    v23 = dword_DF0C4[0];
    dword_DF0A0 = v20;
    v24 = &v21[2 * v34];
    v25 = &v21[2 * dword_DF0C4[0]];
    v26 = *v24 + *v25;
    dword_DF0C4[v20] = v34;
    dword_DF0C0[v20] = v23;
    v21[2 * v22] = v26;
    v27 = byte_DEE60[v34];
    v28 = byte_DEE60[v23];
    v29 = v28 + 1;
    if ( v27 >= v28 )
      v29 = v27 + 1;
    v20 -= 2;
    byte_DEE60[v22] = v29;
    v30 = v35 - v18--;
    v25[1] = v30;
    v24[1] = v30;
    dword_DF0C4[0] = v22++;
    sub_9FE0((long long)v21, 1);
  }
  while ( (int)v18 > 1 );
  v31 = -2 * v37 + 577;
  if ( v37 <= 1 )
    v31 = 573;
  dword_DF0A0 = v31 - 3;
  dword_DF0C0[dword_DF0A0] = dword_DF0C4[0];
  sub_A0C0((long long *)a1);
  return sub_A500(v21, v36);
}



// Function: build_bl_tree @ 0xa8c0
long long build_bl_tree()
{
  long long result; // rax
  long long i; // rdx

  sub_A370((long long)&unk_E0080, dword_193E4);
  sub_A370((long long)&unk_DFF80, dword_193A4);
  sub_A5A0((long long)&off_19340);
  result = 18;
  for ( i = 15; !word_DF9E0[2 * i + 1]; i = *((unsigned char *)&unk_13410 + result) )
  {
    if ( --result == 2 )
    {
      qword_DDA40 += 23;
      return 2;
    }
  }
  qword_DDA40 += 3 * (int)result + 17;
  return (unsigned int)result;
}



// Function: send_tree @ 0xa960
short send_tree(long long a1, int a2)
{
  int v2; // eax
  int v3; // ebp
  int v4; // esi
  int v5; // ecx
  long long v6; // r12
  int v7; // edx
  int v8; // edi
  int v9; // r14d
  int v10; // r13d
  int v14; // [rsp+Ch] [rbp-3Ch]

  v2 = a2;
  v3 = *(unsigned short *)(a1 + 2);
  v4 = 4 - (v3 == 0);
  v5 = v3 == 0 ? 138 : 7;
  if ( v2 >= 0 )
  {
    v6 = 0;
    v7 = 0;
    v8 = -1;
    do
    {
      LOWORD(v2) = a1;
      v9 = v7 + 1;
      v10 = v3;
      v3 = *(unsigned short *)(a1 + 4 * v6 + 6);
      if ( v7 + 1 >= v5 )
        goto LABEL_11;
      if ( v3 == v10 )
      {
        ++v7;
        goto LABEL_9;
      }
      if ( v9 >= v4 )
      {
LABEL_11:
        v14 = v7;
        if ( v10 )
        {
          if ( v8 != v10 )
          {
            sub_4290((unsigned short)word_DF9E0[2 * v10], (unsigned short)word_DF9E0[2 * v10 + 1]);
            v9 = v14;
          }
          sub_4290((unsigned short)word_DFA20, (unsigned short)word_DFA22);
          LOWORD(v2) = sub_4290(v9 - 3, 2);
        }
        else if ( v9 > 10 )
        {
          sub_4290((unsigned short)word_DFA28, (unsigned short)word_DFA2A);
          LOWORD(v2) = sub_4290(v14 - 10, 7);
        }
        else
        {
          sub_4290((unsigned short)word_DFA24, (unsigned short)word_DFA26);
          LOWORD(v2) = sub_4290(v14 - 2, 3);
        }
        if ( !v3 )
        {
LABEL_18:
          v8 = v10;
          v4 = 3;
          v5 = 138;
          v7 = 0;
          goto LABEL_9;
        }
        if ( v3 == v10 )
        {
          v8 = v3;
          v4 = 3;
          v5 = 6;
          v7 = 0;
          goto LABEL_9;
        }
      }
      else
      {
        do
        {
          LOWORD(v2) = sub_4290((unsigned short)word_DF9E0[2 * v10], (unsigned short)word_DF9E0[2 * v10 + 1]);
          --v9;
        }
        while ( v9 );
        if ( !v3 )
          goto LABEL_18;
      }
      v8 = v10;
      v4 = 4;
      v5 = 7;
      v7 = 0;
LABEL_9:
      ++v6;
    }
    while ( a2 >= (int)v6 );
  }
  return v2;
}



// Function: send_all_trees @ 0xab20
short send_all_trees(int a1, int a2, int a3)
{
  char *v4; // r14
  long long v5; // rax
  char *v6; // rbx

  sub_4290(a1 - 257, 5);
  sub_4290(a2 - 1, 5);
  sub_4290(a3 - 4, 4);
  if ( a3 > 0 )
  {
    v4 = (char *)&unk_13411;
    v5 = 16;
    v6 = (char *)&unk_13411 + (unsigned int)(a3 - 1);
    while ( 1 )
    {
      sub_4290((unsigned short)word_DF9E0[2 * v5 + 1], 3);
      if ( v4 == v6 )
        break;
      v5 = (unsigned char)*v4++;
    }
  }
  sub_A960((long long)&unk_E0080, a1 - 1);
  return sub_A960((long long)&unk_DFF80, a2 - 1);
}



// Function: compress_block @ 0xabd0
short compress_block(long long a1, long long a2)
{
  unsigned char v2; // bl
  unsigned int v3; // r14d
  long long v4; // r12
  long long i; // rax
  long long v6; // rdx
  unsigned int v7; // ebp
  long long v8; // rax
  int v9; // esi
  unsigned int v10; // edx
  long long v11; // rax
  int v12; // esi
  int v14; // [rsp+8h] [rbp-50h]
  long long v15; // [rsp+8h] [rbp-50h]
  int v16; // [rsp+10h] [rbp-48h]
  int v17; // [rsp+14h] [rbp-44h]

  if ( dword_DDA54 )
  {
    v2 = 0;
    v3 = 0;
    v4 = 0;
    for ( i = 0; ; i = v7 )
    {
      if ( (i & 7) == 0 )
      {
        v6 = v3++;
        v2 = byte_DDA60[v6];
      }
      v7 = i + 1;
      v8 = (unsigned char)byte_9D000[i];
      if ( (v2 & 1) == 0 )
        break;
      v14 = (unsigned char)v8;
      v16 = byte_DED60[v8];
      sub_4290(
        *(unsigned short *)(a1 + 4LL * (unsigned int)(v16 + 257)),
        *(unsigned short *)(a1 + 4LL * (unsigned int)(v16 + 257) + 2));
      v9 = dword_192C0[v16];
      if ( v9 )
        sub_4290(v14 - dword_DEAE0[v16], v9);
      v10 = word_4C000[v4];
      if ( v10 > 0xFF )
        v11 = byte_DEB60[(v10 >> 7) + 256];
      else
        v11 = byte_DEB60[word_4C000[v4]];
      v17 = word_4C000[v4];
      v15 = v11;
      sub_4290(*(unsigned short *)(a2 + 4 * v11), *(unsigned short *)(a2 + 4 * v11 + 2));
      v4 = (unsigned int)(v4 + 1);
      v12 = dword_19240[v15];
      if ( !v12 )
        goto LABEL_4;
      v2 >>= 1;
      sub_4290(v17 - dword_DEA60[v15], v12);
      if ( dword_DDA54 <= v7 )
        return sub_4290(*(unsigned short *)(a1 + 1024), *(unsigned short *)(a1 + 1026));
LABEL_5:
      ;
    }
    sub_4290(*(unsigned short *)(a1 + 4 * v8), *(unsigned short *)(a1 + 4 * v8 + 2));
LABEL_4:
    v2 >>= 1;
    if ( dword_DDA54 <= v7 )
      return sub_4290(*(unsigned short *)(a1 + 1024), *(unsigned short *)(a1 + 1026));
    goto LABEL_5;
  }
  return sub_4290(*(unsigned short *)(a1 + 1024), *(unsigned short *)(a1 + 1026));
}



// Function: ct_init @ 0xad80
void ct_init(long long a1)
{
  long long v1; // rsi
  int v2; // edi
  int v3; // ecx
  int v4; // r8d
  unsigned char *v5; // rax
  int v6; // eax
  long long v7; // rsi
  int v8; // edi
  int v9; // ecx
  int v10; // r10d
  unsigned char *v11; // rax
  int v12; // edi
  int v13; // r10d
  unsigned char *v14; // rax
  unsigned short *v15; // rax
  short *v16; // rax
  short v17; // cx
  short *v18; // rax
  short v19; // si
  short v20; // ax
  short v21; // si
  short *v22; // rax
  long long v23; // rbx

  qword_DDA20 = a1;
  qword_DDA28 = 0;
  qword_DDA30 = 0;
  if ( !word_DFA82[0] )
  {
    v1 = 0;
    v2 = 0;
    do
    {
      v3 = dword_192C0[v1];
      dword_DEAE0[v1] = v2;
      v4 = 1 << v3;
      if ( 1 << v3 > 0 )
      {
        v5 = &byte_DED60[v2];
        do
          *v5++ = v1;
        while ( v5 != &byte_DED60[v2 + 1 + v4 - 1] );
        v2 += v4;
      }
      ++v1;
    }
    while ( v1 != 28 );
    v6 = v2 - 1;
    v7 = 0;
    v8 = 0;
    byte_DED60[v6] = 28;
    do
    {
      v9 = dword_19240[v7];
      dword_DEA60[v7] = v8;
      v10 = 1 << v9;
      if ( 1 << v9 > 0 )
      {
        v11 = &byte_DEB60[v8];
        do
          *v11++ = v7;
        while ( v11 != &byte_DEB60[v8 + 1 + v10 - 1] );
        v8 += v10;
      }
      ++v7;
    }
    while ( v7 != 16 );
    v12 = v8 >> 7;
    do
    {
      dword_DEA60[v7] = v12 << 7;
      v13 = 1 << (dword_19240[v7] - 7);
      if ( v13 > 0 )
      {
        v14 = &byte_DEB60[v12];
        do
        {
          v14[256] = v7;
          ++v14;
        }
        while ( v14 != &byte_DEB60[v12 + 1 + v13 - 1] );
        v12 += v13;
      }
      ++v7;
    }
    while ( v7 != 30 );
    v15 = word_DF9C0;
    do
      *v15++ = 0;
    while ( v15 != word_DF9E0 );
    v16 = &unk_DFB02;
    v17 = word_DF9D0;
    do
    {
      v16 += 2;
      *(v16 - 2) = 8;
    }
    while ( &unk_DFD42 != (_UNKNOWN *)v16 );
    v18 = &unk_DFD42;
    v19 = word_DF9D2;
    do
    {
      v18 += 2;
      *(v18 - 2) = 9;
    }
    while ( &unk_DFF02 != (_UNKNOWN *)v18 );
    v20 = v19 + 112;
    v21 = word_DF9CE;
    word_DF9D2 = v20;
    v22 = (short *)&unk_DFF02;
    do
    {
      v22 += 2;
      *(v22 - 2) = 7;
    }
    while ( &word_DFF62 != v22 );
    word_DF9CE = v21 + 24;
    word_DFF66 = 8;
    word_DFF6A = 8;
    word_DFF7E = 8;
    v23 = 0;
    word_DFF62 = 8;
    word_DFF6E = 8;
    word_DFF72 = 8;
    word_DFF76 = 8;
    word_DFF7A = 8;
    word_DF9D0 = v17 + 152;
    sub_A500(&word_DFB00, 287);
    do
    {
      word_DFA82[2 * v23] = 5;
      word_DFA82[2 * v23 - 1] = sub_4380(v23, 5);
      ++v23;
    }
    while ( v23 != 30 );
    sub_9F30();
  }
}



// Function: flush_block @ 0xb070
long long flush_block(char *a1, long long a2, int a3, int a4)
{
  unsigned long long v7; // r15
  int v8; // eax
  int v9; // r14d
  unsigned long long v10; // rdi
  unsigned long long v11; // rcx
  unsigned long long v12; // rax
  long long v13; // rax

  byte_DDA60[dword_DDA4C] = byte_DDA49;
  if ( *(short *)qword_DDA20 == 0xFFFF )
    sub_A480();
  v7 = a2 + 4;
  sub_A5A0((long long)&off_193C0);
  sub_A5A0((long long)&off_19380);
  v8 = sub_A8C0();
  qword_DDA28 += a2;
  v9 = v8;
  v10 = (unsigned long long)(qword_DDA40 + 10) >> 3;
  v11 = (unsigned long long)(qword_DDA38 + 10) >> 3;
  v12 = v11;
  if ( v10 <= v11 )
    v12 = (unsigned long long)(qword_DDA40 + 10) >> 3;
  if ( v7 > v12 || !a1 )
  {
    if ( v10 >= v11 )
    {
      sub_4290(a4 + 2, 3);
      sub_ABD0((long long)&word_DFB00, (long long)&unk_DFA80);
      qword_DDA30 += qword_DDA38 + 3;
    }
    else
    {
      sub_4290(a4 + 4, 3);
      sub_AB20(dword_193E4 + 1, dword_193A4 + 1, v9 + 1);
      sub_ABD0((long long)&unk_E0080, (long long)&unk_DFF80);
      qword_DDA30 += qword_DDA40 + 3;
    }
    sub_9F30();
    if ( !a4 )
      goto LABEL_10;
LABEL_15:
    sub_43B0();
    v13 = qword_DDA30 + 7;
    qword_DDA30 += 7;
    return v13 >> 3;
  }
  sub_4290(a4, 3);
  qword_DDA30 = ((qword_DDA30 + 10) & 0xFFFFFFFFFFFFFFF8LL) + 8 * v7;
  sub_44A0(a1, a2, 1);
  sub_9F30();
  if ( a4 )
    goto LABEL_15;
LABEL_10:
  v13 = qword_DDA30;
  if ( a3 && (qword_DDA30 & 7) != 0 )
  {
    sub_4290(0, 3);
    qword_DDA30 = (qword_DDA30 + 10) & 0xFFFFFFFFFFFFFFF8LL;
    sub_44A0(a1, 0, 1);
    v13 = qword_DDA30;
  }
  return v13 >> 3;
}



// Function: ct_tally @ 0xb290
long long ct_tally(int a1, int a2)
{
  long long v2; // rax
  unsigned int v3; // r9d
  char v4; // al
  long long v5; // rax
  char v6; // cl
  unsigned long long v7; // rdx
  unsigned long long v8; // rcx
  long long v9; // rax
  long long v10; // rsi
  long long result; // rax
  int v12; // edx
  long long v13; // rcx
  long long v14; // rcx

  v2 = (unsigned int)dword_DDA54;
  v3 = ++dword_DDA54;
  byte_9D000[v2] = a2;
  v4 = byte_DDA48;
  if ( a1 )
  {
    v12 = a1 - 1;
    ++*((short *)&unk_E0080 + 2 * byte_DED60[a2] + 514);
    if ( a1 - 1 <= 255 )
      v13 = byte_DEB60[v12];
    else
      v13 = byte_DEB60[(v12 >> 7) + 256];
    ++word_DFF80[2 * v13];
    v14 = (unsigned int)dword_DDA50++;
    word_4C000[v14] = v12;
    byte_DDA49 |= v4;
  }
  else
  {
    ++*((short *)&unk_E0080 + 2 * a2);
  }
  byte_DDA48 = 2 * v4;
  if ( (v3 & 7) == 0 )
  {
    v5 = (unsigned int)dword_DDA4C;
    v6 = byte_DDA49;
    byte_DDA48 = 1;
    byte_DDA49 = 0;
    ++dword_DDA4C;
    byte_DDA60[v5] = v6;
  }
  if ( dword_1901C <= 2 )
    goto LABEL_10;
  if ( (v3 & 0xFFF) != 0 )
    goto LABEL_10;
  v7 = 0;
  v8 = 8LL * v3;
  do
  {
    v9 = (int)dword_19240[v7 / 2];
    v10 = (unsigned short)word_DFF80[v7];
    v7 += 2LL;
    v8 += v10 * (v9 + 5);
  }
  while ( v7 != 60 );
  if ( v3 >> 1 <= dword_DDA50
    || (result = 1, ((unsigned long long)(unsigned int)dword_1A06C - qword_1A078) >> 1 <= v8 >> 3) )
  {
LABEL_10:
    result = 1;
    if ( v3 != 0x7FFF )
      return dword_DDA50 == 0x8000;
  }
  return result;
}



// Function: fillbuf @ 0xb450
long long fillbuf(int a1)
{
  int v1; // ebx
  unsigned int v2; // eax
  int v3; // edx
  int v4; // ecx
  int v5; // ecx
  long long result; // rax

  v1 = a1;
  v2 = dword_E0990;
  v3 = (unsigned short)word_E0994 << a1;
  v4 = dword_E098C;
  if ( dword_E098C < a1 )
  {
    do
    {
      v1 -= v4;
      v3 |= v2 << v1;
      word_E0994 = v3;
      if ( dword_1B008 < (unsigned int)dword_1B00C )
      {
        v2 = (unsigned char)byte_9D000[dword_1B008++];
        dword_E0990 = v2;
      }
      else
      {
        v2 = sub_D830(1);
        dword_E0990 = v2;
        if ( v2 == -1 )
        {
          dword_E0990 = 0;
          v3 = (unsigned short)word_E0994;
          v2 = 0;
        }
        else
        {
          v3 = (unsigned short)word_E0994;
        }
      }
      dword_E098C = 8;
      v4 = 8;
    }
    while ( v1 > 8 );
  }
  v5 = v4 - v1;
  result = v2 >> v5;
  dword_E098C = v5;
  word_E0994 = result | v3;
  return result;
}



// Function: init_getbits @ 0xb520
long long init_getbits()
{
  dword_E0990 = 0;
  word_E0994 = 0;
  dword_E098C = 0;
  return sub_B450(16);
}



// Function: huf_decode_start @ 0xb550
long long huf_decode_start()
{
  long long result; // rax

  result = sub_B520();
  dword_E0BA0 = 0;
  return result;
}



// Function: decode_start @ 0xb570
long long decode_start()
{
  long long result; // rax

  result = sub_B550();
  dword_E0988 = 0;
  dword_E0984 = 0;
  return result;
}



// Function: getbits @ 0xb5a0
long long getbits(int a1)
{
  unsigned int v1; // r12d

  v1 = (int)(unsigned short)word_E0994 >> (16 - a1);
  sub_B450(a1);
  return v1;
}



// Function: decode_p @ 0xb5c0
long long decode_p()
{
  long long v0; // r12
  unsigned int v1; // eax

  v0 = word_E09A0[HIBYTE(word_E0994)];
  if ( (unsigned int)v0 > 0xD )
  {
    v1 = 128;
    do
    {
      while ( ((unsigned short)word_E0994 & (unsigned short)v1) != 0 )
      {
        v1 >>= 1;
        v0 = *((unsigned short *)&qword_1B880 + (unsigned int)(v0 + 0x8000));
        if ( (unsigned int)v0 <= 0xD )
          goto LABEL_6;
      }
      LODWORD(v0) = *((unsigned short *)&qword_1B880 + v0);
      v1 >>= 1;
    }
    while ( (unsigned int)v0 > 0xD );
  }
LABEL_6:
  sub_B450(*((unsigned char *)&unk_E0BC0 + (unsigned int)v0));
  if ( (int)v0 )
    return (1 << (v0 - 1)) + (unsigned int)sub_B5A0((int)v0 - 1);
  else
    return 0;
}



// Function: make_table @ 0xb670
unsigned long long make_table(unsigned int a1, unsigned char *a2, unsigned int a3, long long a4)
{
  char *v7; // rax
  unsigned int v8; // edx
  unsigned char *v9; // rax
  long long v10; // rsi
  long long v11; // rcx
  int v12; // r9d
  long long i; // rax
  unsigned short v14; // cx
  long long v15; // rax
  unsigned int v16; // eax
  long long v17; // rsi
  char v18; // cl
  unsigned int v19; // eax
  long long v20; // r15
  long long v21; // rdi
  unsigned int v22; // eax
  long long v23; // r11
  unsigned short v24; // r8
  unsigned short v25; // bp
  unsigned int v26; // ebx
  short *v27; // rax
  int v29; // ebx
  short *v30; // r12
  unsigned int v31; // ecx
  long long v32; // rax
  long long v33; // rcx
  char v34; // [rsp+10h] [rbp-D8h]
  short v35; // [rsp+20h] [rbp-C8h]
  char v36; // [rsp+22h] [rbp-C6h] BYREF
  char v37[64]; // [rsp+42h] [rbp-A6h] BYREF
  short v38; // [rsp+82h] [rbp-66h]
  unsigned short v39; // [rsp+A2h] [rbp-46h]
  unsigned long long v40; // [rsp+A8h] [rbp-40h]

  v40 = __readfsqword(0x28u);
  v7 = &v36;
  do
  {
    v7 += 2;
    *((short *)v7 - 1) = 0;
  }
  while ( v7 != v37 );
  v8 = a1;
  v9 = a2;
  v10 = (long long)&a2[a1];
  do
  {
    v11 = *v9++;
    ++*(&v35 + v11);
  }
  while ( v9 != (unsigned char *)v10 );
  v12 = 0;
  v38 = 0;
  for ( i = 1; i != 17; ++i )
  {
    v12 += (unsigned short)*(&v35 + i) << (16 - i);
    *(short *)&v37[2 * i + 64] = v12;
  }
  v14 = v39;
  if ( v39 )
LABEL_35:
    sub_D510("Bad table\n");
  v15 = 1;
  v34 = 16 - a3;
  while ( 1 )
  {
    *(short *)&v37[2 * v15 + 62] = (int)v14 >> (16 - a3);
    *(short *)&v37[2 * v15 + 14] = 1 << (a3 - v15);
    if ( a3 < (unsigned int)++v15 )
      break;
    v14 = *(short *)&v37[2 * v15 + 62];
  }
  v16 = a3 + 1;
  do
  {
    v17 = v16;
    v18 = 16 - v16++;
    *(short *)&v37[2 * v17 + 14] = 1 << v18;
  }
  while ( v16 != 17 );
  v19 = (int)*(unsigned short *)&v37[2 * (a3 + 1) + 62] >> v34;
  if ( v19 && v19 != 1 << a3 )
  {
    do
    {
      v33 = v19++;
      *(short *)(a4 + 2 * v33) = 0;
    }
    while ( 1 << a3 != v19 );
  }
  v20 = a1;
  v21 = 0;
  do
  {
    v22 = a2[v21];
    if ( a2[v21] )
    {
      v23 = a2[v21];
      v24 = *(short *)&v37[2 * v23 + 62];
      v25 = *(short *)&v37[2 * v23 + 14];
      if ( a3 < v22 )
      {
        v29 = v24;
        v30 = (short *)(a4 + 2LL * (v24 >> v34));
        v31 = v22 - a3;
        if ( v22 != a3 )
        {
          do
          {
            while ( 1 )
            {
              v32 = (unsigned short)*v30;
              if ( !(short)v32 )
              {
                *((short *)&qword_1B880 + v8) = 0;
                *((short *)&qword_1B880 + v8 + 0x8000) = 0;
                v32 = (unsigned short)v8;
                *v30 = v8++;
              }
              if ( (v29 & (1 << (15 - a3))) == 0 )
                break;
              v30 = (short *)&qword_1B880 + v32 + 0x8000;
              v29 *= 2;
              if ( !--v31 )
                goto LABEL_31;
            }
            v30 = (short *)&qword_1B880 + v32;
            v29 *= 2;
            --v31;
          }
          while ( v31 );
        }
LABEL_31:
        *v30 = v21;
      }
      else
      {
        v26 = v24 + v25;
        if ( 1 << a3 < v26 )
          goto LABEL_35;
        if ( v26 > v24 )
        {
          v27 = (short *)(a4 + 2LL * v24);
          do
            *v27++ = v21;
          while ( (short *)(a4 + 2 + 2 * (v24 + (unsigned long long)((unsigned int)v25 - 1))) != v27 );
        }
      }
      *(short *)&v37[2 * v23 + 62] = v25 + v24;
    }
    ++v21;
  }
  while ( v20 != v21 );
  return v40 - __readfsqword(0x28u);
}



// Function: read_c_len @ 0xb980
unsigned long long read_c_len()
{
  int v0; // eax
  int v1; // ebp
  long long v2; // r14
  long long v3; // rax
  long long v4; // rbx
  unsigned int v5; // eax
  int v6; // ebx
  char *v7; // rax
  int v8; // edx
  unsigned long long result; // rax
  short *v10; // rdx
  unsigned short *v11; // rdx

  v0 = sub_B5A0(9);
  if ( !v0 )
  {
    result = sub_B5A0(9);
    v10 = &word_5C000;
    do
    {
      *(char *)v10 = 0;
      v10 = (short *)((char *)v10 + 1);
    }
    while ( v10 != (short *)&unk_5C1FE );
    v11 = word_4C000;
    do
      *v11++ = result;
    while ( v11 != &word_4C000[4096] );
    return result;
  }
  v1 = v0;
  LODWORD(v2) = 0;
  if ( v0 <= 0 )
    goto LABEL_21;
  do
  {
    while ( 1 )
    {
      v4 = word_E09A0[HIBYTE(word_E0994)];
      if ( word_E09A0[HIBYTE(word_E0994)] > 0x12u )
      {
        v5 = 128;
        do
        {
          if ( ((unsigned short)word_E0994 & (unsigned short)v5) != 0 )
            v4 = *((unsigned short *)&qword_1B880 + (int)v4 + 0x8000);
          else
            LODWORD(v4) = *((unsigned short *)&qword_1B880 + v4);
          v5 >>= 1;
        }
        while ( (int)v4 > 18 );
      }
      sub_B450(*((unsigned char *)&unk_E0BC0 + (int)v4));
      if ( (int)v4 <= 2 )
        break;
      v3 = (int)v2;
      LODWORD(v2) = v2 + 1;
      *((char *)&word_5C000 + v3) = v4 - 2;
LABEL_6:
      if ( (int)v2 >= v1 )
        goto LABEL_20;
    }
    if ( (int)v4 )
    {
      v6 = (int)v4 == 1 ? sub_B5A0(4) + 3 : (unsigned int)sub_B5A0(9) + 20;
      LODWORD(v4) = v6 - 1;
      if ( (int)v4 < 0 )
        goto LABEL_6;
    }
    v7 = (char *)&word_5C000 + (int)v2;
    v8 = v4 + (int)v7;
    do
      *v7++ = 0;
    while ( v8 - (int)v7 >= 0 );
    LODWORD(v2) = v2 + v4 + 1;
  }
  while ( (int)v2 < v1 );
LABEL_20:
  if ( (int)v2 <= 509 )
  {
LABEL_21:
    v2 = (int)v2;
    do
      *((char *)&word_5C000 + v2++) = 0;
    while ( (int)v2 <= 509 );
  }
  return sub_B670(0x1FEu, (unsigned char *)&word_5C000, 0xCu, (long long)word_4C000);
}



// Function: read_pt_len @ 0xbb40
int *read_pt_len(signed int a1, int a2, int a3)
{
  signed int v4; // eax
  signed int v5; // ebx
  signed int v6; // r15d
  char *v7; // r13
  char *v8; // rbp
  int v9; // edi
  short v10; // r12
  unsigned int v11; // edx
  int v12; // eax
  int v13; // edi
  char *v14; // rax
  int *result; // rax
  int v16; // eax
  char *v17; // rdx
  short v18; // dx
  char *v19; // rax
  int v21; // [rsp+Ch] [rbp-3Ch]

  v4 = sub_B5A0(a2);
  if ( v4 )
  {
    v5 = v4;
    v6 = 0;
    v21 = a3 - 1;
LABEL_3:
    v7 = (char *)&unk_E0BC0 + v6;
    v8 = (char *)&unk_E0BC1 + v6 + (unsigned int)(v21 - v6);
    while ( v6 < v5 )
    {
      v9 = 3;
      v10 = (unsigned short)word_E0994 >> 13;
      if ( (unsigned short)word_E0994 >> 13 == 7 )
      {
        if ( (word_E0994 & 0x1000) != 0 )
        {
          v11 = 4096;
          v12 = 7;
          do
          {
            v11 >>= 1;
            v13 = v12++;
          }
          while ( ((unsigned short)v11 & (unsigned short)word_E0994) != 0 );
          if ( v12 > 16 )
            sub_D510("Bad table\n");
          v9 = v13 - 2;
          LOBYTE(v10) = v12;
        }
        else
        {
          LOBYTE(v10) = 7;
          v9 = 4;
        }
      }
      sub_B450(v9);
      *v7++ = v10;
      ++v6;
      if ( v7 == v8 )
      {
        v16 = sub_B5A0(2);
        if ( v16 > 0 )
        {
          v17 = (char *)&unk_E0BC0 + v6;
          do
            *v17++ = 0;
          while ( v17 != (char *)&unk_E0BC1 + v6 + (unsigned int)(v16 - 1) );
          v6 += v16;
        }
        goto LABEL_3;
      }
    }
    if ( v6 < a1 )
    {
      v14 = (char *)&unk_E0BC0 + v6;
      do
        *v14++ = 0;
      while ( v14 != (char *)&unk_E0BC1 + v6 + (unsigned int)(a1 - 1 - v6) );
    }
    return (int *)sub_B670(a1, (unsigned char *)&unk_E0BC0, 8u, (long long)word_E09A0);
  }
  else
  {
    v18 = sub_B5A0(a2);
    v19 = &unk_E0BC0;
    do
      *v19++ = 0;
    while ( v19 != (char *)&unk_E0BC1 + (unsigned int)(a1 - 1) );
    result = (int *)word_E09A0;
    do
    {
      *(short *)result = v18;
      result = (int *)((char *)result + 2);
    }
    while ( result != &dword_E0BA0 );
  }
  return result;
}



// Function: decode_c @ 0xbd50
long long decode_c()
{
  int v0; // eax
  long long v1; // r12
  unsigned int v2; // eax

  v0 = dword_E0BA0;
  if ( !dword_E0BA0 )
  {
    LODWORD(v1) = 510;
    dword_E0BA0 = sub_B5A0(16);
    if ( !dword_E0BA0 )
      return (unsigned int)v1;
    sub_BB40(19, 5, 3);
    sub_B980();
    sub_BB40(14, 4, -1);
    v0 = dword_E0BA0;
  }
  dword_E0BA0 = v0 - 1;
  v1 = word_4C000[(unsigned short)word_E0994 >> 4];
  if ( (unsigned int)v1 > 0x1FD )
  {
    v2 = 8;
    do
    {
      while ( ((unsigned short)word_E0994 & (unsigned short)v2) != 0 )
      {
        v2 >>= 1;
        v1 = *((unsigned short *)&qword_1B880 + (unsigned int)(v1 + 0x8000));
        if ( (unsigned int)v1 <= 0x1FD )
          goto LABEL_7;
      }
      LODWORD(v1) = *((unsigned short *)&qword_1B880 + v1);
      v2 >>= 1;
    }
    while ( (unsigned int)v1 > 0x1FD );
  }
LABEL_7:
  sub_B450(*((unsigned char *)&word_5C000 + (unsigned int)v1));
  return (unsigned int)v1;
}



// Function: unlzh @ 0xbfd0
long long unlzh(int a1, unsigned int a2)
{
  unsigned int v2; // eax

  dword_1B014 = a1;
  dword_1B010 = a2;
  sub_B570();
  while ( !dword_E0984 )
  {
    v2 = sub_BE40(0x2000, byte_3C000);
    if ( v2 )
      sub_D6C0(a2, byte_3C000, v2);
  }
  return 0;
}



// Function: unlzw @ 0xc040
long long unlzw(unsigned int a1, unsigned int a2)
{
  long long v2; // rax
  int v3; // eax
  long long v4; // rsi
  long long v5; // r13
  long long v6; // rax
  int v8; // r8d
  signed long long v9; // r15
  long long v10; // r13
  char *v11; // rsi
  int v12; // eax
  signed long long v13; // rdi
  long long v14; // rdx
  char v15; // cl
  long long v16; // rax
  char *v17; // rdx
  signed long long v18; // rax
  char v19; // si
  char *v20; // r14
  char *v21; // rdx
  int v22; // eax
  long long v23; // r13
  int v24; // edi
  long long v25; // rcx
  long long v26; // r13
  char *v27; // rax
  char v28; // dl
  long long v29; // rax
  char *v30; // r13
  int v31; // r15d
  int v32; // r12d
  char *v33; // rdi
  long long v34; // rdx
  unsigned int v35; // r12d
  int v37; // ecx
  const char *v38; // rdi
  char v39; // [rsp+0h] [rbp-A8h]
  long long v40; // [rsp+8h] [rbp-A0h]
  int v41; // [rsp+10h] [rbp-98h]
  signed long long v42; // [rsp+10h] [rbp-98h]
  char v43; // [rsp+1Ch] [rbp-8Ch]
  long long v44; // [rsp+20h] [rbp-88h]
  long long v45; // [rsp+38h] [rbp-70h]
  int v47; // [rsp+48h] [rbp-60h]
  long long v48; // [rsp+48h] [rbp-60h]
  long long v49; // [rsp+50h] [rbp-58h]
  signed long long v50; // [rsp+58h] [rbp-50h]
  int v51; // [rsp+60h] [rbp-48h]
  int v52; // [rsp+64h] [rbp-44h]
  unsigned int v53; // [rsp+68h] [rbp-40h]

  v2 = (unsigned int)dword_1B008;
  if ( dword_1B008 >= (unsigned int)dword_1B00C )
  {
    v3 = sub_D830(0);
  }
  else
  {
    ++dword_1B008;
    v3 = (unsigned char)byte_9D000[v2];
  }
  dword_19024 = v3;
  dword_193E8 = v3 & 0x80;
  if ( (v3 & 0x60) == 0 )
    goto LABEL_8;
  if ( !dword_1B85C )
  {
    sub_F630(stderr, v39);
    if ( status )
      goto LABEL_7;
    goto LABEL_6;
  }
  if ( !status )
LABEL_6:
    status = 2;
LABEL_7:
  LOBYTE(v3) = dword_19024;
LABEL_8:
  dword_19024 = v3 & 0x1F;
  v49 = 1LL << (v3 & 0x1F);
  if ( (v3 & 0x1Fu) > 0x10 )
  {
    v35 = 1;
    sub_F630(stderr, v39);
    status = 1;
    return v35;
  }
  qword_1B880 = 0;
  LODWORD(v4) = dword_1B00C;
  qword_1B978 = 0;
  v5 = (unsigned int)(8 * dword_1B008);
  v52 = dword_1B00C;
  v40 = 256 - ((dword_193E8 == 0) - 1LL);
  memset(
    (void *)((unsigned long long)&unk_1B888 & 0xFFFFFFFFFFFFFFF8LL),
    0,
    8LL
  * (((unsigned int)&qword_1B880 - (unsigned int)((unsigned long long)&unk_1B888 & 0xFFFFFFFFFFFFFFF8LL) + 256) >> 3));
  v6 = 255;
  do
    byte_3C000[v6] = v6;
  while ( v6-- != 0 );
  v51 = 9;
  v44 = 511;
  v8 = 0;
  v9 = v6;
  v53 = 511;
  v43 = 0;
LABEL_12:
  v10 = v5 >> 3;
  if ( (unsigned int)v10 > (unsigned int)v4 )
  {
LABEL_13:
    dword_1B00C = 0;
    v11 = byte_9D000;
LABEL_14:
    v41 = v8;
    v12 = sub_D2B0(a1, v11);
    v8 = v41;
    v52 = v12;
    if ( v12 == -1 )
      sub_D600(a1, v11);
    qword_1B828 += v12;
    LODWORD(v4) = v12 + dword_1B00C;
    dword_1B00C += v12;
    goto LABEL_16;
  }
  while ( 1 )
  {
    v4 = (unsigned int)(v4 - v10);
    if ( (int)v4 > 0 )
    {
      v27 = byte_9D000;
      do
      {
        v28 = v27[(int)v10];
        *v27++ = v28;
      }
      while ( (char *)&unk_9D001 + (unsigned int)(v4 - 1) != v27 );
    }
    dword_1B00C = v4;
    if ( (unsigned int)v4 <= 0x3F )
    {
      v11 = &byte_9D000[v4];
      goto LABEL_14;
    }
LABEL_16:
    if ( !v52 )
      break;
    v5 = 0;
    v45 = 8 * ((unsigned int)v4 - (unsigned long long)((unsigned int)v4 % v51));
    if ( v45 <= 0 )
      goto LABEL_12;
LABEL_18:
    if ( v40 <= v44 )
    {
      v5 = 0;
      while ( 1 )
      {
        v13 = v9;
        v14 = v5 >> 3;
        v42 = v9;
        v15 = v5 & 7;
        v5 += v51;
        v16 = (unsigned char)byte_9D000[v14];
        v9 = v53
           & (unsigned long long)((long long)(v16
                                        | ((unsigned long long)(unsigned char)byte_9D000[v14 + 2] << 16)
                                        | ((unsigned long long)(unsigned char)byte_9D000[v14 + 1] << 8)) >> v15);
        if ( v13 == -1 )
        {
          if ( v9 > 255 )
            sub_D510("corrupt input.");
          v29 = v8;
          v43 = v9;
          ++v8;
          *((char *)&word_5C000 + v29) = v9;
          if ( v5 >= v45 )
            goto LABEL_44;
        }
        else
        {
          if ( v9 != 256 )
          {
            if ( v9 < v40 )
            {
              v18 = v53
                  & (unsigned long long)((long long)(v16
                                               | ((unsigned long long)(unsigned char)byte_9D000[v14 + 2] << 16)
                                               | ((unsigned long long)(unsigned char)byte_9D000[v14 + 1] << 8)) >> v15);
              v17 = (char *)&unk_5BFFE;
              goto LABEL_26;
            }
            if ( v9 > v40 )
            {
              if ( v8 > 0 )
                sub_D6C0(a2, &word_5C000, (unsigned int)v8);
              v38 = "corrupt input.";
              if ( !dword_1B860 )
                v38 = "corrupt input. Use zcat to recover some data.";
              sub_D510(v38);
            }
LABEL_24:
            v17 = &byte_5BFFD;
            byte_5BFFD = v43;
            v18 = v42;
            goto LABEL_26;
          }
          if ( dword_193E8 )
          {
            qword_1B880 = 0;
            qword_1B978 = 0;
            memset(
              (void *)((unsigned long long)&unk_1B888 & 0xFFFFFFFFFFFFFFF8LL),
              0,
              8LL
            * (((unsigned int)&qword_1B880 - (unsigned int)((unsigned long long)&unk_1B888 & 0xFFFFFFFFFFFFFFF8LL) + 256) >> 3));
            v53 = 511;
            v40 = 256;
            v37 = v51;
            LODWORD(v4) = dword_1B00C;
            v51 = 9;
            v44 = 511;
            v9 = v42;
            v5 = 8 * v37 - (v5 - 1 + 8 * v37) % (8 * v37) + v5 - 1;
            goto LABEL_12;
          }
          v18 = 256;
          v17 = (char *)&unk_5BFFE;
          if ( v40 == 256 )
            goto LABEL_24;
          do
          {
            *--v17 = byte_3C000[v18];
            v18 = *((unsigned short *)&qword_1B880 + v18);
LABEL_26:
            ;
          }
          while ( v18 > 255 );
          v19 = byte_3C000[v18];
          v20 = v17 - 1;
          *(v17 - 1) = v19;
          v21 = (char *)(&unk_5BFFE - (_UNKNOWN *)(v17 - 1));
          v43 = v19;
          v22 = (int)v21;
          if ( v8 + (int)v21 > (int)&unk_3FFFF )
          {
            v48 = v5;
            v30 = v20;
            v50 = v9;
            v31 = v8;
            do
            {
              v32 = (int)&unk_40000 - v31;
              if ( (int)&unk_40000 - v31 > v22 )
                v32 = v22;
              if ( v32 > 0 )
              {
                v33 = (char *)&word_5C000 + v31;
                v31 += v32;
                memcpy(v33, v30, v32);
              }
              if ( v31 > (int)&unk_3FFFF )
              {
                v34 = (unsigned int)v31;
                v31 = 0;
                sub_D6C0(a2, &word_5C000, v34);
              }
              v30 += v32;
              v22 = &unk_5BFFE - (_UNKNOWN *)v30;
            }
            while ( v22 > 0 );
            v8 = v31;
            v5 = v48;
            v9 = v50;
          }
          else
          {
            v47 = v8 + (int)v21;
            memcpy((char *)&word_5C000 + v8, v20, (int)v21);
            v8 = v47;
          }
          if ( v49 > v40 )
          {
            *((short *)&qword_1B880 + v40) = v42;
            byte_3C000[v40++] = v19;
          }
          if ( v5 >= v45 )
          {
LABEL_44:
            if ( !v52 )
              goto LABEL_60;
            LODWORD(v4) = dword_1B00C;
            goto LABEL_12;
          }
        }
        if ( v44 < v40 )
        {
          LODWORD(v4) = dword_1B00C;
          v23 = v5 - 1;
          goto LABEL_34;
        }
      }
    }
    v23 = -1;
LABEL_34:
    v24 = v51 + 1;
    v25 = 8 * v51;
    v51 = v24;
    v44 = v49;
    v26 = v25 - (v25 + v23) % v25 + v23;
    if ( dword_19024 != v24 )
      v44 = (1LL << v24) - 1;
    v10 = v26 >> 3;
    v53 = (1 << v24) - 1;
    if ( (unsigned int)v10 > (unsigned int)v4 )
      goto LABEL_13;
  }
  v45 = 8LL * (unsigned int)v4 - (v51 - 1);
  if ( v45 > 0 )
    goto LABEL_18;
LABEL_60:
  v35 = 0;
  if ( v8 > 0 )
    sub_D6C0(a2, &word_5C000, (unsigned int)v8);
  return v35;
}



// Function: read_byte @ 0xc820
long long read_byte()
{
  long long v0; // rax
  long long result; // rax

  v0 = (unsigned int)dword_1B008;
  if ( dword_1B008 >= (unsigned int)dword_1B00C )
  {
    result = sub_D830(0);
    if ( (int)result < 0 )
      sub_D510("invalid compressed data -- unexpected end of file");
  }
  else
  {
    ++dword_1B008;
    return (unsigned char)byte_9D000[v0];
  }
  return result;
}



// Function: read_tree @ 0xc870
long long read_tree()
{
  int v0; // ebp
  long long v1; // rbx
  unsigned char v2; // al
  long long v3; // r12
  int v4; // ebx
  int v5; // ebp
  unsigned char v6; // al
  int v7; // edx
  bool v8; // zf
  long long result; // rax
  long long v10; // r12
  int v11; // r13d
  int v12; // ebp
  char *v13; // r15
  long long v14; // rbp

  v0 = 4;
  v1 = 0;
  qword_E0E88 = 0;
  do
  {
    v1 = (unsigned char)sub_C820() | (unsigned long long)(v1 << 8);
    qword_E0E88 = v1;
    --v0;
  }
  while ( v0 );
  v2 = sub_C820();
  dword_E0E80 = v2;
  if ( (unsigned int)v2 - 1 > 0x18 )
    sub_D510("invalid compressed data -- Huffman code bit length out of range");
  if ( v2 )
  {
    v3 = 1;
    v4 = 1;
    v5 = 0;
    do
    {
      v6 = sub_C820();
      v7 = dword_E0E80;
      v8 = dword_E0E80 == (int)v3;
      dword_E0C80[v3] = v6;
      if ( v6 > v4 - v8 )
        goto LABEL_19;
      ++v3;
      v5 += v6;
      v4 = 2 * (v4 - v6) + 1;
    }
    while ( v7 >= (int)v3 );
    if ( v5 > 255 )
LABEL_19:
      sub_D510("too many leaves in Huffman tree");
    result = v7;
    ++dword_E0C80[v7];
    if ( v7 > 0 )
    {
      v10 = 1;
      v11 = 0;
      do
      {
        v12 = dword_E0C80[v10];
        dword_E0D00[v10] = v11;
        if ( v12 > 0 )
        {
          v13 = (char *)&unk_E0D80 + v11;
          v14 = (unsigned int)(v12 - 1);
          do
            *v13++ = sub_C820();
          while ( v13 != (char *)&unk_E0D81 + v11 + v14 );
          v7 = dword_E0E80;
          v11 += v14 + 1;
        }
        ++v10;
      }
      while ( v7 >= (int)v10 );
      result = v7;
    }
  }
  else
  {
    ++dword_E0C80[0];
    result = 0;
  }
  ++dword_E0C80[result];
  return result;
}



// Function: unpack @ 0xc9f0
long long unpack(int a1, int a2)
{
  unsigned long long v2; // r15
  long long v3; // r8
  long long v4; // rsi
  long long v5; // rdx
  unsigned int v6; // eax
  int v7; // r12d
  long long v8; // rcx
  long long v9; // rdx
  char v10; // al
  long long v11; // rsi
  int i; // r13d
  long long v14; // rbx
  int v15; // [rsp+8h] [rbp-40h]
  int v16; // [rsp+Ch] [rbp-3Ch]

  v2 = 0;
  dword_1B010 = a2;
  dword_1B014 = a1;
  sub_C870();
  sub_C740();
  v4 = (unsigned int)dword_E0BF0;
  dword_E0BE0 = 0;
  qword_E0BE8 = 0;
  v15 = (1 << dword_E0BF0) - 1;
  v5 = 0;
  v16 = dword_E0C80[dword_E0E80] - 1;
  while ( 1 )
  {
    if ( (int)v5 < (int)v4 )
    {
      qword_E0BE8 = (unsigned char)sub_C820() | (v2 << 8);
      v2 = qword_E0BE8;
      v5 = (unsigned int)(dword_E0BE0 + 8);
      dword_E0BE0 += 8;
      goto LABEL_8;
    }
    v6 = v15 & (v2 >> ((unsigned char)v5 - (unsigned char)v4));
    v7 = *((unsigned char *)&word_5C000 + v6);
    if ( *((char *)&word_5C000 + v6) )
    {
      v4 = (unsigned int)(v4 - v7);
      v8 = (unsigned int)v4;
      v6 >>= v4;
    }
    else
    {
      v8 = (int)v4;
      i = v15;
      if ( v6 >= dword_E0C00[(int)v4] )
      {
        v7 = v4;
      }
      else
      {
        v4 = (unsigned int)(v4 + 1);
        v14 = (int)v4;
        while ( 1 )
        {
          v7 = v14;
          for ( i = 2 * i + 1; (int)v5 < (int)v14; dword_E0BE0 = v5 )
          {
            qword_E0BE8 = (unsigned char)sub_C820() | (v2 << 8);
            v2 = qword_E0BE8;
            v5 = (unsigned int)(dword_E0BE0 + 8);
          }
          v8 = (unsigned int)dword_E0C00[v14++];
          v6 = i & (v2 >> ((unsigned char)v5 - (unsigned char)v7));
          if ( (unsigned int)v8 <= v6 )
            break;
          v5 = (unsigned int)dword_E0BE0;
        }
      }
    }
    if ( v6 == v16 && dword_E0E80 == v7 )
      break;
    v9 = (unsigned int)dword_1B004;
    v10 = *((char *)&unk_E0D80 + dword_E0D00[v7] + v6);
    v11 = (unsigned int)++dword_1B004;
    byte_3C000[v9] = v10;
    if ( (int)v11 == 0x8000 )
      sub_D7E0(byte_3C000, v11, v9, v7, v3);
    v2 = qword_E0BE8;
    v5 = (unsigned int)(dword_E0BE0 - v7);
    dword_E0BE0 -= v7;
LABEL_8:
    v4 = (unsigned int)dword_E0BF0;
  }
  sub_D7E0(&word_5C000, v4, v5, v8, v3);
  if ( (unsigned int)qword_1B820 != qword_E0E88 )
    sub_D510("invalid compressed data--length error");
  return 0;
}



// Function: check_zipfile @ 0xcc20
long long check_zipfile(int a1)
{
  char *v1; // rax
  unsigned char v3; // dl
  char v4; // al
  char v5; // [rsp+0h] [rbp-8h]

  dword_1B014 = a1;
  v1 = &byte_9D000[dword_1B008];
  dword_1B008 += *((unsigned short *)v1 + 13) + *((unsigned short *)v1 + 14) + 30;
  if ( dword_1B008 > (unsigned int)dword_1B00C
    || *(int *)v1 != 67324752
    || (v3 = v1[8], dword_19020 = v3, (v3 & 0xF7) != 0)
    || (v4 = v1[6], dword_E0EA0 = v4 & 1, (v4 & 1) != 0) )
  {
    sub_F630(stderr, v5);
    status = 1;
    return 1;
  }
  else
  {
    dword_E0E9C = 1;
    dword_E0E98 = (v4 & 8) != 0;
    return 0;
  }
}



// Function: unzip @ 0xcd20
long long unzip(int a1, int a2)
{
  long long v2; // rdx
  long long v3; // r8
  long long v4; // rbp
  long long v5; // r12
  long long v6; // rcx
  long long v7; // r13
  long long v8; // rax
  char v9; // al
  long long v10; // rax
  int v11; // eax
  unsigned int *v12; // rbx
  long long v13; // rax
  char *v14; // rsi
  unsigned int v15; // r13d
  unsigned long long v16; // rdi
  unsigned int *v18; // rbx
  long long v19; // rax
  unsigned int v20; // [rsp+0h] [rbp-48h] BYREF
  unsigned int v21; // [rsp+4h] [rbp-44h]
  char v22[4]; // [rsp+8h] [rbp-40h] BYREF
  unsigned int v23; // [rsp+Ch] [rbp-3Ch]
  char v24; // [rsp+10h] [rbp-38h] BYREF
  unsigned long long v25; // [rsp+18h] [rbp-30h]

  v25 = __readfsqword(0x28u);
  dword_1B014 = a1;
  dword_1B010 = a2;
  sub_D1F0(0, 0);
  if ( dword_E0E9C )
  {
    if ( dword_E0E98 )
    {
      v5 = 0;
      v4 = 0;
    }
    else
    {
      v4 = (unsigned int)dword_9D00E;
      v5 = (unsigned int)dword_9D016;
    }
    if ( dword_19020 != 8 )
    {
      if ( !dword_19020 )
      {
        v6 = (unsigned int)dword_9D012;
        v7 = (unsigned int)dword_9D016;
        v8 = (unsigned int)dword_9D012;
        if ( dword_E0EA0 )
          v8 = (unsigned int)dword_9D012 - 12LL;
        if ( dword_9D016 != v8 )
        {
          sub_F630(stderr, v20);
          sub_D510("invalid compressed data--length mismatch");
        }
        if ( dword_9D016 )
        {
          do
          {
            while ( 1 )
            {
              v10 = (unsigned int)dword_1B008;
              if ( dword_1B008 < (unsigned int)dword_1B00C )
              {
                ++dword_1B008;
                v9 = byte_9D000[v10];
              }
              else
              {
                v9 = sub_D830(0);
              }
              v2 = (unsigned int)dword_1B004;
              v6 = (unsigned int)(dword_1B004 + 1);
              byte_3C000[dword_1B004] = v9;
              dword_1B004 = v6;
              if ( (int)v6 == 0x8000 )
                break;
              if ( !--v7 )
                goto LABEL_39;
            }
            sub_D7E0(0, 0, v2, v6, v3);
            --v7;
          }
          while ( v7 );
        }
LABEL_39:
        sub_D7E0(0, 0, v2, v6, v3);
        if ( dword_E0E9C )
          goto LABEL_40;
        goto LABEL_21;
      }
LABEL_55:
      sub_D510("internal error, invalid method");
    }
  }
  else
  {
    if ( dword_19020 != 8 )
      goto LABEL_55;
    v5 = 0;
    v4 = 0;
  }
  v11 = sub_9E80(0, 0);
  if ( v11 == 3 )
    sub_D550();
  if ( v11 )
    sub_D510("invalid compressed data--format violated");
  if ( dword_E0E9C )
  {
LABEL_40:
    if ( dword_E0E98 )
    {
      v18 = &v20;
      do
      {
        while ( 1 )
        {
          v19 = (unsigned int)dword_1B008;
          if ( dword_1B008 >= (unsigned int)dword_1B00C )
            break;
          v18 = (unsigned int *)((char *)v18 + 1);
          ++dword_1B008;
          *((char *)v18 - 1) = byte_9D000[v19];
          if ( v18 == (unsigned int *)&v24 )
            goto LABEL_45;
        }
        v18 = (unsigned int *)((char *)v18 + 1);
        *((char *)v18 - 1) = sub_D830(0);
      }
      while ( v18 != (unsigned int *)&v24 );
LABEL_45:
      v4 = v21;
      v5 = v23;
    }
    goto LABEL_26;
  }
LABEL_21:
  v12 = &v20;
  do
  {
    while ( 1 )
    {
      v13 = (unsigned int)dword_1B008;
      if ( dword_1B008 >= (unsigned int)dword_1B00C )
        break;
      v12 = (unsigned int *)((char *)v12 + 1);
      ++dword_1B008;
      *((char *)v12 - 1) = byte_9D000[v13];
      if ( v12 == (unsigned int *)v22 )
        goto LABEL_25;
    }
    v12 = (unsigned int *)((char *)v12 + 1);
    *((char *)v12 - 1) = sub_D830(0);
  }
  while ( v12 != (unsigned int *)v22 );
LABEL_25:
  v4 = v20;
  v5 = v21;
LABEL_26:
  v14 = 0;
  v15 = 0;
  if ( sub_D1F0(&word_5C000, 0) != v4 )
  {
    v14 = "\n%s: %s: invalid compressed data--crc error\n";
    v15 = 1;
    sub_F630(stderr, v20);
  }
  if ( (unsigned int)qword_1B820 == v5 )
  {
    v16 = (unsigned int)dword_E0E9C;
    if ( !dword_E0E9C )
      goto LABEL_33;
  }
  else
  {
    v16 = (unsigned long long)stderr;
    v14 = "\n%s: %s: invalid compressed data--length error\n";
    sub_F630(stderr, v20);
    if ( !dword_E0E9C )
      goto LABEL_49;
    v15 = 1;
  }
  if ( dword_1B008 + 4 >= (unsigned int)dword_1B00C )
    goto LABEL_33;
  v14 = byte_9D000;
  if ( ((*((unsigned char *)&unk_9D001 + (unsigned int)dword_1B008) << 8)
      | (unsigned int)(unsigned char)byte_9D000[dword_1B008]
      | (unsigned long long)((long long)(*((unsigned char *)&unk_9D002 + (unsigned int)dword_1B008)
                                   | (byte_9D003[dword_1B008] << 8)) << 16)) != 0x4034B50 )
    goto LABEL_33;
  v14 = (char *)(unsigned int)dword_1B860;
  if ( dword_1B860 )
  {
    if ( !dword_1B85C )
    {
      v16 = (unsigned long long)stderr;
      v14 = "%s: %s has more than one entry--rest ignored\n";
      sub_F630(stderr, v20);
    }
    if ( !status )
      status = 2;
LABEL_33:
    dword_E0E9C = 0;
    dword_E0E98 = 0;
    qword_E0E90 = v4;
    if ( !v15 )
      return v15;
    goto LABEL_34;
  }
  v16 = (unsigned long long)stderr;
  v14 = "%s: %s has more than one entry -- unchanged\n";
  sub_F630(stderr, v20);
  dword_E0E9C = 0;
LABEL_49:
  dword_E0E98 = 0;
  qword_E0E90 = v4;
LABEL_34:
  status = 1;
  if ( !dword_1B858 )
    sub_8620(v16, v14);
  return 1;
}



// Function: write_buffer @ 0xd1d0
ssize_t write_buffer(int a1, const void *a2, unsigned int a3)
{
  if ( a3 > 0x7FFFFFFF )
    a3 = 0x7FFFFFFF;
  return write(a1, a2, a3);
}



// Function: updcrc @ 0xd1f0
long long updcrc(char *a1, int a2)
{
  long long result; // rax
  unsigned long long v3; // rdx
  char *v4; // rcx
  char v5; // al

  result = 0;
  v3 = 0xFFFFFFFFLL;
  if ( a1 )
  {
    v3 = qword_193F0;
    if ( a2 )
    {
      v4 = &a1[a2];
      do
      {
        v5 = *a1++;
        v3 = qword_13800[(unsigned char)(v3 ^ v5)] ^ (v3 >> 8);
      }
      while ( a1 != v4 );
    }
    result = v3 ^ 0xFFFFFFFF;
  }
  qword_193F0 = v3;
  return result;
}



// Function: getcrc @ 0xd250
long long getcrc()
{
  return qword_193F0 ^ 0xFFFFFFFFLL;
}



// Function: clear_bufs @ 0xd270
void clear_bufs()
{
  dword_1B004 = 0;
  dword_1B008 = 0;
  dword_1B00C = 0;
  qword_1B820 = 0;
  qword_1B828 = 0;
}



// Function: read_buffer @ 0xd2b0
long long read_buffer(int fd, void *buf, unsigned int a3)
{
  char v3; // bl
  size_t v4; // r12
  int v5; // r14d
  int v7; // edx
  int v8; // ecx
  int *v9; // rbx
  int v10; // r8d
  int v11; // r9d
  int v12; // eax
  int v13; // ecx
  int v14; // r8d
  int v15; // r9d
  char v16; // [rsp-18h] [rbp-28h]
  char v17; // [rsp-18h] [rbp-28h]

  if ( a3 > 0x7FFFFFFF )
    a3 = 0x7FFFFFFF;
  v16 = v3;
  v4 = a3;
  v5 = read(fd, buf, a3);
  if ( v5 < 0 )
  {
    v9 = __errno_location();
    if ( *v9 == 11 )
    {
      v12 = sub_E270(fd, 3, v7, v8, v10, v11, v16);
      if ( v12 >= 0 )
      {
        if ( (v12 & 0x800) != 0 )
        {
          BYTE1(v12) &= ~8u;
          if ( (unsigned int)sub_E270(fd, 4, v12, v13, v14, v15, v17) != -1 )
            return (unsigned int)read(fd, buf, v4);
        }
        else
        {
          *v9 = 11;
        }
      }
    }
  }
  return (unsigned int)v5;
}



// Function: strlwr @ 0xd350
unsigned char *strlwr(unsigned char *a1)
{
  unsigned char v1; // bl
  unsigned char *v2; // rbp
  const unsigned short **v3; // r13

  v1 = *a1;
  if ( *a1 )
  {
    v2 = a1;
    v3 = __ctype_b_loc();
    do
    {
      if ( ((*v3)[v1] & 0x100) != 0 )
        v1 = (*__ctype_tolower_loc())[v1];
      *v2 = v1;
      v1 = *++v2;
    }
    while ( v1 );
  }
  return a1;
}



// Function: add_envopt @ 0xd3d0
char *add_envopt(int *a1, long long **a2, const char *a3)
{
  char *v3; // rax
  char *v4; // r13
  int v5; // r12d
  const char *v6; // rbx
  const char *v7; // rbx
  const char *v8; // rax
  long long v9; // rbx
  long long *v10; // rax
  long long *v11; // rdx
  long long *v12; // r12
  long long *v13; // r14
  const char *v14; // rbx
  bool v16; // zf
  char *v17; // rdi

  v3 = getenv(a3);
  v4 = v3;
  if ( v3 )
  {
    v5 = 0;
    v4 = (char *)sub_F5D0(v3);
    v6 = v4;
    if ( *v4 )
    {
      while ( 1 )
      {
        v7 = &v6[strspn(v6, " \t")];
        if ( !*v7 )
          break;
        ++v5;
        v8 = &v7[strcspn(v7, " \t")];
        if ( *v8 )
        {
          v16 = v8[1] == 0;
          *v8 = 0;
          v6 = v8 + 1;
          if ( !v16 )
            continue;
        }
        goto LABEL_5;
      }
      if ( !v5 )
        goto LABEL_14;
LABEL_5:
      v9 = (unsigned int)(v5 + 1);
      *a1 = v9;
      v10 = (long long *)sub_F4D0(v5 + 2, 8);
      v11 = *a2;
      v12 = v10 + 1;
      *a2 = v10;
      v13 = &v10[v9];
      v14 = v4;
      *v10 = *v11;
      do
      {
        ++v12;
        v14 += strspn(v14, " \t");
        *(v12 - 1) = v14;
        do
          ++v14;
        while ( *(v14 - 1) );
      }
      while ( v13 != v12 );
      *v13 = 0;
    }
    else
    {
LABEL_14:
      v17 = v4;
      v4 = 0;
      free(v17);
    }
  }
  return v4;
}



// Function: gzip_error @ 0xd510
void gzip_error()
{
  FILE *v0; // rdi
  const char *v1; // rdx
  const char *v2; // rcx
  const char *v3; // r8
  char v4; // [rsp-8h] [rbp-8h]

  v0 = stderr;
  sub_F630(stderr, v4);
  sub_8620(v0, "\n%s: %s: %s\n", v1, v2, v3);
}



// Function: xalloc_die @ 0xd550
void xalloc_die()
{
  FILE *v0; // rdi
  const char *v1; // rdx
  char v2; // [rsp-8h] [rbp-8h]

  v0 = stderr;
  sub_F630(stderr, v2);
  sub_8620(v0, "\n%s: memory_exhausted\n", v1);
}



// Function: warning @ 0xd580
long long warning()
{
  long long result; // rax
  char v1; // [rsp-8h] [rbp-8h]

  if ( dword_1B85C )
  {
    result = (unsigned int)status;
    if ( !status )
      status = 2;
  }
  else
  {
    result = sub_F630(stderr, v1);
    if ( !status )
      status = 2;
  }
  return result;
}



// Function: read_error @ 0xd600
void read_error()
{
  int *v0; // rax
  int v1; // ebp
  int *v2; // rbx
  FILE *v3; // rdi
  char v4; // [rsp+0h] [rbp-18h]
  char v5; // [rsp+0h] [rbp-18h]

  v0 = __errno_location();
  v1 = *v0;
  v2 = v0;
  sub_F630(stderr, v4);
  if ( v1 )
  {
    *v2 = v1;
    perror(dword_1B420);
    sub_8620(dword_1B420, "\n%s: ");
  }
  v3 = stderr;
  sub_F630(stderr, v5);
  sub_8620(v3, "%s: unexpected end of file\n");
}



// Function: write_error @ 0xd670
void write_error()
{
  int *v0; // rax
  int v1; // ebp
  int *v2; // rbx
  const char *v3; // rdx
  char v4; // [rsp+0h] [rbp-18h]

  v0 = __errno_location();
  v1 = *v0;
  v2 = v0;
  sub_F630(stderr, v4);
  *v2 = v1;
  perror(file);
  sub_8620(file, "\n%s: ", v3);
}



// Function: write_buf @ 0xd6c0
ssize_t write_buf(int a1, char *a2, unsigned int a3)
{
  ssize_t result; // rax
  unsigned int v5; // ebx

  result = a3;
  qword_1B820 += a3;
  if ( !dword_1B858 )
  {
    v5 = a3;
    while ( 1 )
    {
      result = sub_D1D0(a1, a2, v5);
      if ( v5 == (int)result )
        break;
      if ( (int)result == -1 )
        sub_D670();
      v5 -= result;
      a2 += (unsigned int)result;
    }
  }
  return result;
}



// Function: copy @ 0xd720
long long copy(int fd, int a2)
{
  int v3; // edx
  long long v4; // rsi
  int v5; // eax

  *__errno_location() = 0;
  v3 = dword_1B00C;
  v4 = (unsigned int)dword_1B008;
  if ( dword_1B008 < (unsigned int)dword_1B00C )
  {
    do
    {
      sub_D6C0(a2, &byte_9D000[v4], v3 - v4);
      v5 = sub_D2B0(fd, byte_9D000, (unsigned int)&unk_40000);
      v3 = v5;
      if ( v5 == -1 )
        sub_D600();
      dword_1B00C = v5;
      v4 = 0;
      qword_1B828 += v5;
      dword_1B008 = 0;
    }
    while ( v5 );
  }
  return 0;
}



// Function: flush_outbuf @ 0xd7a0
ssize_t flush_outbuf()
{
  ssize_t result; // rax

  if ( dword_1B004 )
  {
    result = sub_D6C0(dword_1B010, (char *)&word_5C000, dword_1B004);
    dword_1B004 = 0;
  }
  return result;
}



// Function: flush_window @ 0xd7e0
ssize_t flush_window()
{
  unsigned int v0; // r8d
  char *v1; // r9
  ssize_t result; // rax

  if ( dword_1B004 )
  {
    sub_D1F0(byte_3C000, dword_1B004);
    result = sub_D6C0(dword_1B010, v1, v0);
    dword_1B004 = 0;
  }
  return result;
}



// Function: fill_inbuf @ 0xd830
long long fill_inbuf(int a1)
{
  long long v1; // rax
  int v2; // eax

  v1 = 0;
  dword_1B00C = 0;
  while ( 1 )
  {
    v2 = sub_D2B0(dword_1B014, &byte_9D000[v1], (unsigned int)&unk_40000 - (unsigned int)v1);
    if ( !v2 )
      break;
    if ( v2 == -1 )
      goto LABEL_10;
    v1 = (unsigned int)(dword_1B00C + v2);
    dword_1B00C = v1;
    if ( (unsigned int)v1 > (unsigned int)&unk_3FFFF )
      goto LABEL_8;
  }
  v1 = (unsigned int)dword_1B00C;
  if ( dword_1B00C )
  {
LABEL_8:
    dword_1B008 = 1;
    qword_1B828 += v1;
    return (unsigned char)byte_9D000[0];
  }
  if ( !a1 )
  {
    sub_D7E0();
    *__errno_location() = 0;
LABEL_10:
    sub_D600();
  }
  return 0xFFFFFFFFLL;
}



// Function: display_ratio @ 0xd8e0
long long display_ratio(long long a1, long long a2, FILE *a3, long long a4, long long a5, long long a6, char a7)
{
  return sub_F630(a3, a7);
}



// Function: fprint_off @ 0xd930
unsigned long long fprint_off(FILE *stream, long long a2, int a3)
{
  long long v3; // rcx
  char *v5; // rbx
  int v6; // esi
  int v7; // r13d
  int v9; // edi
  char *v11; // rdi
  char *v12; // rbx
  int v13; // eax
  char v14; // [rsp+40h] [rbp-38h] BYREF
  unsigned long long v15; // [rsp+48h] [rbp-30h]

  v3 = a2;
  v15 = __readfsqword(0x28u);
  v5 = &v14;
  if ( a2 < 0 )
  {
    v11 = &v14;
    do
    {
      v12 = v11--;
      v13 = 10 * (v3 / 10) + 48 - v3;
      v3 /= 10;
      *v11 = v13;
    }
    while ( v3 );
    *(v11 - 1) = 45;
    v5 = v12 - 2;
  }
  else
  {
    do
    {
      *--v5 = v3 % 0xAuLL + 48;
      v3 /= 0xAuLL;
    }
    while ( v3 );
  }
  v6 = a3 - ((unsigned int)&v14 - (int)v5);
  v7 = v6 - 1;
  if ( v6 > 0 )
  {
    do
      putc(32, stream);
    while ( v7-- != 0 );
  }
  if ( v5 < &v14 )
  {
    do
    {
      v9 = *v5++;
      putc(v9, stream);
    }
    while ( v5 != &v14 );
  }
  return v15 - __readfsqword(0x28u);
}



// Function: zip @ 0xda60
long long zip(int a1, int a2)
{
  char v3; // di
  char v4; // dl
  char v5; // si
  char v6; // cl
  long long v7; // rax
  long long v8; // rdx
  int *v9; // rsi
  char v10; // cl
  long long v11; // rax
  long long v12; // rbx
  char *v13; // r12
  char v14; // dl
  unsigned long long v15; // rdi
  char v16; // al
  long long v17; // rbx
  long long v18; // r12
  long long v19; // rdx
  long long v20; // rbx
  long long v21; // rax
  unsigned long long v22; // rax
  long long v23; // r12
  unsigned long long v24; // rax
  long long v25; // rdx
  long long v26; // rbx
  unsigned long long v27; // rax
  long long v28; // rdx
  long long v29; // rcx
  long long v30; // rbx
  unsigned long long v31; // rax
  long long v32; // rdx
  int v34; // ecx
  unsigned int v35; // eax
  long long v36; // rax
  long long v37; // rax
  char v38; // [rsp+0h] [rbp-28h]
  short v39; // [rsp+6h] [rbp-22h] BYREF
  unsigned long long v40; // [rsp+8h] [rbp-20h]

  v40 = __readfsqword(0x28u);
  dword_1B010 = a2;
  v39 = 0;
  dword_1B014 = a1;
  word_5C000 = -29921;
  dword_19020 = 8;
  byte_5C002 = 8;
  dword_1B004 = 4;
  byte_5C003 = 8 * (dword_1B848 != 0);
  if ( *(&timer + 1) < 0 )
  {
    v6 = 0;
    v5 = 0;
    v4 = 0;
    v3 = 0;
    goto LABEL_4;
  }
  if ( (unsigned long long)(timer - 1) <= 0xFFFFFFFE )
  {
    v3 = timer;
    v4 = BYTE1(timer);
    v5 = BYTE2(timer);
    v6 = BYTE3(timer);
LABEL_4:
    byte_5C004 = v3;
    v7 = 7;
    byte_5C005 = v4;
    LODWORD(v8) = 6;
LABEL_5:
    *((char *)&word_5C000 + (unsigned int)v8) = v5;
    dword_1B004 = v8 + 2;
    *((char *)&word_5C000 + v7) = v6;
    goto LABEL_6;
  }
  sub_D580("file timestamp out of range for gzip format");
  v34 = dword_1B004;
  if ( dword_1B004 <= (unsigned int)&unk_3FFFD )
  {
    v8 = (unsigned int)(dword_1B004 + 2);
    v7 = (unsigned int)(dword_1B004 + 3);
    *((char *)&word_5C000 + (unsigned int)dword_1B004) = 0;
    *((char *)&word_5C000 + (unsigned int)(v34 + 1)) = 0;
  }
  else
  {
    v35 = dword_1B004 + 1;
    *((char *)&word_5C000 + (unsigned int)dword_1B004) = 0;
    dword_1B004 = v35;
    if ( v35 == (int)&unk_40000 )
    {
      sub_D7A0();
      v35 = dword_1B004;
    }
    v8 = v35 + 1;
    dword_1B004 = v35 + 1;
    *((char *)&word_5C000 + v35) = 0;
    if ( (int)v8 == (int)&unk_40000 )
    {
      sub_D7A0();
      v8 = (unsigned int)dword_1B004;
      v7 = (unsigned int)(dword_1B004 + 1);
      if ( dword_1B004 > (unsigned int)&unk_3FFFD )
        goto LABEL_35;
LABEL_57:
      v6 = 0;
      v5 = 0;
      goto LABEL_5;
    }
    v7 = v35 + 2;
  }
  if ( (unsigned int)v8 <= (unsigned int)&unk_3FFFD )
    goto LABEL_57;
LABEL_35:
  dword_1B004 = v7;
  *((char *)&word_5C000 + v8) = 0;
  if ( (int)v7 == (int)&unk_40000 )
  {
    sub_D7A0();
    v7 = (unsigned int)dword_1B004;
  }
  *((char *)&word_5C000 + v7) = 0;
  dword_1B004 = v7 + 1;
  if ( (int)v7 + 1 == (int)&unk_40000 )
    sub_D7A0();
LABEL_6:
  sub_D1F0(0, 0);
  sub_4260(a2);
  v9 = &dword_19020;
  sub_AD80((long long)&v39);
  v10 = 4;
  if ( dword_1901C != 1 )
    v10 = 2 * (dword_1901C == 9);
  v11 = (unsigned int)(dword_1B004 + 1);
  *((char *)&word_5C000 + (unsigned int)dword_1B004) = v10;
  dword_1B004 = v11;
  if ( (int)v11 == (int)&unk_40000 )
  {
    sub_D7A0();
    v11 = (unsigned int)dword_1B004;
  }
  v12 = (unsigned int)(v11 + 1);
  *((char *)&word_5C000 + v11) = 3;
  dword_1B004 = v12;
  if ( (int)v12 == (int)&unk_40000 )
  {
    sub_D7A0();
    v12 = (unsigned int)dword_1B004;
  }
  if ( dword_1B848 )
  {
    v13 = (char *)sub_D3B0(dword_1B420);
    do
    {
      while ( 1 )
      {
        v14 = *v13;
        dword_1B004 = v12 + 1;
        *((char *)&word_5C000 + v12) = v14;
        v12 = (unsigned int)(v12 + 1);
        if ( (int)v12 == (int)&unk_40000 )
          break;
        if ( !*v13++ )
          goto LABEL_17;
      }
      sub_D7A0();
      ++v13;
      v12 = (unsigned int)dword_1B004;
    }
    while ( *(v13 - 1) );
  }
LABEL_17:
  v15 = (unsigned int)dword_1901C;
  qword_E0EA8 = (unsigned int)v12;
  sub_4ED0(dword_1901C);
  if ( qword_1B840 != -1 && qword_1B840 != qword_1B828 )
  {
    v15 = (unsigned long long)stderr;
    v9 = (int *)"%s: %s: file size changed while zipping\n";
    sub_F630(stderr, v38);
  }
  v16 = sub_D250(v15, v9);
  v17 = (unsigned int)dword_1B004;
  v18 = (unsigned int)++dword_1B004;
  if ( (unsigned int)v17 > (unsigned int)&unk_3FFFD )
  {
    *((char *)&word_5C000 + v17) = v16;
    if ( (int)v18 == (int)&unk_40000 )
    {
      sub_D7A0();
      v18 = (unsigned int)dword_1B004;
    }
    v36 = sub_D250(v15, v9);
    v20 = (unsigned int)(v18 + 1);
    dword_1B004 = v18 + 1;
    *((char *)&word_5C000 + v18) = BYTE1(v36);
    if ( (int)v20 == (int)&unk_40000 )
    {
      sub_D7A0();
      v20 = (unsigned int)dword_1B004;
    }
  }
  else
  {
    v19 = (unsigned int)v17;
    v20 = (unsigned int)(v17 + 2);
    *((char *)&word_5C000 + v19) = v16;
    v21 = sub_D250(v15, v9);
    dword_1B004 = v20;
    *((char *)&word_5C000 + v18) = BYTE1(v21);
  }
  v22 = sub_D250(v15, v9);
  v23 = (unsigned int)(v20 + 1);
  dword_1B004 = v20 + 1;
  v24 = v22 >> 16;
  if ( (unsigned int)v20 > (unsigned int)&unk_3FFFD )
  {
    *((char *)&word_5C000 + v20) = v24;
    if ( (int)v23 == (int)&unk_40000 )
    {
      sub_D7A0();
      v23 = (unsigned int)dword_1B004;
    }
    v37 = sub_D250(v15, v9);
    v26 = (unsigned int)(v23 + 1);
    dword_1B004 = v23 + 1;
    *((char *)&word_5C000 + v23) = BYTE3(v37);
    if ( (int)v26 == (int)&unk_40000 )
    {
      sub_D7A0();
      v26 = (unsigned int)dword_1B004;
    }
  }
  else
  {
    v25 = (unsigned int)v20;
    v26 = (unsigned int)(v20 + 2);
    *((char *)&word_5C000 + v25) = v24;
    *((char *)&word_5C000 + v23) = (unsigned int)sub_D250(v15, v9) >> 24;
  }
  v27 = qword_1B828;
  v28 = (unsigned int)(v26 + 1);
  if ( (unsigned int)v26 > (unsigned int)&unk_3FFFD )
  {
    dword_1B004 = v26 + 1;
    *((char *)&word_5C000 + v26) = qword_1B828;
    if ( (int)v28 == (int)&unk_40000 )
    {
      sub_D7A0();
      v27 = qword_1B828;
      v28 = (unsigned int)dword_1B004;
    }
    v30 = (unsigned int)(v28 + 1);
    *((char *)&word_5C000 + v28) = BYTE1(v27);
    dword_1B004 = v30;
    if ( (int)v30 == (int)&unk_40000 )
    {
      sub_D7A0();
      v30 = (unsigned int)dword_1B004;
      v27 = qword_1B828;
    }
  }
  else
  {
    v29 = (unsigned int)v26;
    v30 = (unsigned int)(v26 + 2);
    *((char *)&word_5C000 + v29) = qword_1B828;
    *((char *)&word_5C000 + v28) = BYTE1(v27);
  }
  v31 = v27 >> 16;
  v32 = (unsigned int)(v30 + 1);
  if ( (unsigned int)v30 > (unsigned int)&unk_3FFFD )
  {
    dword_1B004 = v30 + 1;
    *((char *)&word_5C000 + v30) = v31;
    if ( (int)v32 == (int)&unk_40000 )
    {
      sub_D7A0();
      v32 = (unsigned int)dword_1B004;
      v31 = (unsigned long long)qword_1B828 >> 16;
    }
    *((char *)&word_5C000 + v32) = BYTE1(v31);
    dword_1B004 = v32 + 1;
    if ( (int)v32 + 1 == (int)&unk_40000 )
      sub_D7A0();
  }
  else
  {
    *((char *)&word_5C000 + (unsigned int)v30) = v31;
    dword_1B004 = v30 + 2;
    *((char *)&word_5C000 + v32) = BYTE1(v31);
  }
  qword_E0EA8 += 8;
  sub_D7A0();
  return 0;
}



// Function: file_read @ 0xe010
long long file_read(void *buf, unsigned int a2)
{
  int v2; // eax
  unsigned int v3; // r12d

  v2 = sub_D2B0(dword_1B014, buf, a2);
  v3 = v2;
  if ( v2 )
  {
    if ( v2 == -1 )
      sub_D600();
    sub_D1F0((char *)buf, v2);
    qword_1B828 += v3;
  }
  return v3;
}



// Function: last_component @ 0xe060
char *last_component(char *a1)
{
  char v1; // dl
  char *result; // rax
  char *v3; // rcx
  char v4; // si

  v1 = *a1;
  for ( result = a1; v1 == 47; ++result )
    v1 = result[1];
  if ( v1 )
  {
    v3 = result;
    v4 = 0;
    do
    {
      while ( v1 == 47 )
      {
        v1 = *++v3;
        v4 = 1;
        if ( !v1 )
          return result;
      }
      if ( v4 )
      {
        result = v3;
        v4 = 0;
      }
      v1 = *++v3;
    }
    while ( v1 );
  }
  return result;
}



// Function: rpl_fcntl_DUPFD @ 0xe180
int rpl_fcntl_DUPFD(int a1, unsigned int a2)
{
  return fcntl(a1, 0, a2);
}



// Function: rpl_fcntl_DUPFD_CLOEXEC @ 0xe190
long long rpl_fcntl_DUPFD_CLOEXEC(int a1, unsigned int a2)
{
  int v2; // r12d
  int v4; // eax
  int *v5; // rax
  int v6; // edi
  int v7; // r13d
  int *v8; // rbp

  if ( dword_E0EB0 >= 0 )
  {
    v2 = fcntl(a1, 1030, a2);
    if ( v2 < 0 && *__errno_location() == 22 )
    {
      v2 = sub_E180(a1, a2);
      if ( v2 >= 0 )
      {
        dword_E0EB0 = -1;
LABEL_8:
        v4 = fcntl(v2, 1);
        if ( v4 < 0 || fcntl(v2, 2, v4 | 1u) == -1 )
        {
          v5 = __errno_location();
          v6 = v2;
          v2 = -1;
          v7 = *v5;
          v8 = v5;
          close(v6);
          *v8 = v7;
        }
      }
    }
    else
    {
      dword_E0EB0 = 1;
    }
    return (unsigned int)v2;
  }
  v2 = sub_E180(a1, a2);
  if ( v2 < 0 )
    return (unsigned int)v2;
  if ( dword_E0EB0 == -1 )
    goto LABEL_8;
  return (unsigned int)v2;
}



// Function: rpl_fcntl @ 0xe270
int rpl_fcntl(int a1, int a2, long long a3)
{
  if ( !a2 )
    return sub_E180(a1, a3);
  if ( a2 == 1030 )
    return sub_E190(a1, a3);
  if ( a2 > 11 )
  {
    if ( (unsigned int)(a2 - 1024) > 0xA )
      return fcntl(a1, a2, a3);
    if ( ((1LL << a2) & 0x2C5) == 0 )
    {
      if ( ((1LL << a2) & 0x502) == 0 )
        return fcntl(a1, a2, a3);
      return fcntl(a1, a2);
    }
  }
  else
  {
    if ( a2 <= 0 )
      return fcntl(a1, a2, a3);
    if ( ((1LL << a2) & 0x514) == 0 )
    {
      if ( ((1LL << a2) & 0xA0A) == 0 )
        return fcntl(a1, a2, a3);
      return fcntl(a1, a2);
    }
  }
  return fcntl(a1, a2, (unsigned int)a3);
}



// Function: open_safer @ 0xe3c0
long long open_safer(const char *a1, char a2, long long a3, long long a4, long long a5, long long a6, char a7)
{
  long long v7; // rdx
  int v8; // eax
  int v10; // [rsp+0h] [rbp-58h]
  char *v11; // [rsp+8h] [rbp-50h]
  char *v12; // [rsp+10h] [rbp-48h]
  unsigned long long v13; // [rsp+18h] [rbp-40h]
  char v14; // [rsp+20h] [rbp-38h] BYREF
  long long v15; // [rsp+30h] [rbp-28h]

  v15 = a3;
  v7 = 0;
  v13 = __readfsqword(0x28u);
  if ( (a2 & 0x40) != 0 )
  {
    v10 = 16;
    v7 = (unsigned int)v15;
    v11 = &a7;
    v12 = &v14;
  }
  v8 = open(a1, a2, v7, a4, a5, a6, v10, v11, v12, v13);
  return sub_E980(v8);
}



// Function: clear_ungetc_buffer_preserving_position @ 0xe440
long long clear_ungetc_buffer_preserving_position(FILE *a1)
{
  long long result; // rax

  if ( (a1->_flags & 0x100) != 0 )
    return sub_E4A0(a1);
  return result;
}



// Function: rpl_fseeko @ 0xe4a0
int rpl_fseeko(FILE *stream, __off_t a2, int a3)
{
  int v4; // eax
  __off_t v5; // rax

  if ( stream->_IO_read_end != stream->_IO_read_ptr
    || stream->_IO_write_ptr != stream->_IO_write_base
    || stream->_IO_save_base )
  {
    return fseeko(stream, a2, a3);
  }
  v4 = fileno(stream);
  v5 = lseek(v4, a2, a3);
  if ( v5 == -1 )
    return -1;
  stream->_flags &= ~0x10u;
  stream->_offset = v5;
  return 0;
}



// Function: openat_safer @ 0xe520
long long openat_safer(int a1, const char *a2, char a3, long long a4, long long a5, long long a6, char a7)
{
  long long v7; // rcx
  int v8; // eax
  int v10; // [rsp+0h] [rbp-58h]
  char *v11; // [rsp+8h] [rbp-50h]
  char *v12; // [rsp+10h] [rbp-48h]
  unsigned long long v13; // [rsp+18h] [rbp-40h]
  char v14; // [rsp+20h] [rbp-38h] BYREF
  long long v15; // [rsp+38h] [rbp-20h]

  v15 = a4;
  v7 = 0;
  v13 = __readfsqword(0x28u);
  if ( (a3 & 0x40) != 0 )
  {
    v10 = 24;
    v7 = (unsigned int)v15;
    v11 = &a7;
    v12 = &v14;
  }
  v8 = openat(a1, a2, a3, v7, a5, a6, v10, v11, v12, v13);
  return sub_E980(v8);
}



// Function: direntry_cmp_name @ 0xe5a0
int direntry_cmp_name(const char **a1, const char **a2)
{
  return strcmp(*a1, *a2);
}



// Function: streamsavedir @ 0xe5b0
long long streamsavedir(DIR *dirp, unsigned int a2)
{
  long long v2; // rbx
  size_t v3; // r15
  char *v4; // r13
  long long v5; // r14
  int *v6; // r12
  struct dirent *v7; // rax
  char v8; // dl
  size_t v9; // rax
  char *v10; // r9
  signed long long v11; // r10
  long long *v12; // rdx
  long long v13; // rax
  signed long long v14; // r10
  long long v15; // rbp
  char *v16; // rbx
  long long v17; // rbx
  long long v18; // rdi
  long long v19; // rbp
  char *v20; // rax
  void *v21; // rdi
  long long v23; // rax
  long long v24; // rax
  void *v25; // rdi
  int (*compar)(const void *, const void *); // [rsp+8h] [rbp-70h]
  char *src; // [rsp+10h] [rbp-68h]
  long long *srcb; // [rsp+10h] [rbp-68h]
  void *srcd; // [rsp+10h] [rbp-68h]
  void *srca; // [rsp+10h] [rbp-68h]
  void *srcc; // [rsp+10h] [rbp-68h]
  signed long long v32; // [rsp+18h] [rbp-60h]
  char *v33; // [rsp+18h] [rbp-60h]
  char *v34; // [rsp+18h] [rbp-60h]
  long long v35; // [rsp+28h] [rbp-50h] BYREF
  long long v36[9]; // [rsp+30h] [rbp-48h] BYREF

  v36[1] = __readfsqword(0x28u);
  v35 = 0;
  v36[0] = 0;
  compar = (int (*)(const void *, const void *))qword_18B50[a2];
  if ( !dirp )
    return 0;
  v2 = 0;
  v3 = 0;
  v4 = 0;
  v5 = 0;
  v6 = __errno_location();
  while ( 1 )
  {
    *v6 = 0;
    v7 = readdir(dirp);
    if ( !v7 )
      break;
    v8 = v7->d_name[0];
    if ( v8 == 46 && (v8 = v7->d_name[1], v8 == 46) )
    {
      if ( v7->d_name[2] )
      {
LABEL_8:
        src = v7->d_name;
        v9 = strlen(v7->d_name);
        v10 = src;
        v11 = v9 + 1;
        if ( compar )
        {
          if ( v36[0] == v3 )
          {
            v33 = src;
            srca = (void *)(v9 + 1);
            v23 = sub_F3F0(v4, v36, 1, -1, 8);
            v10 = v33;
            v11 = (signed long long)srca;
            v4 = (char *)v23;
          }
          v12 = (long long *)&v4[8 * v3];
          v32 = v11;
          ++v3;
          srcb = v12;
          v13 = sub_F5D0(v10);
          v14 = v32;
          *srcb = v13;
        }
        else
        {
          if ( v35 - v2 <= v11 )
          {
            v34 = src;
            srcc = (void *)(v9 + 1);
            v24 = sub_F3F0(v5, &v35, v11 - (v35 - v2), 0x7FFFFFFFFFFFFFFELL, 1);
            v10 = v34;
            v11 = (signed long long)srcc;
            v5 = v24;
          }
          srcd = (void *)v11;
          memcpy((void *)(v5 + v2), v10, v11);
          v14 = (signed long long)srcd;
        }
        v2 += v14;
      }
    }
    else if ( v8 )
    {
      goto LABEL_8;
    }
  }
  if ( *v6 )
  {
    free(v4);
    v25 = (void *)v5;
    v5 = 0;
    free(v25);
  }
  else
  {
    if ( compar )
    {
      v15 = v2 + 1;
      if ( v3 )
      {
        v17 = 0;
        qsort(v4, v3, 8u, compar);
        v18 = v15;
        v19 = 0;
        v5 = sub_F250(v18);
        do
        {
          v20 = stpcpy((char *)(v5 + v17), *(const char **)&v4[8 * v19]);
          v21 = *(void **)&v4[8 * v19++];
          v17 = (long long)&v20[-v5 + 1];
          free(v21);
        }
        while ( v3 != v19 );
        v16 = (char *)(v5 + v17);
      }
      else
      {
        v5 = sub_F250(v2 + 1);
        v16 = (char *)v5;
      }
      free(v4);
    }
    else
    {
      if ( v2 == v35 )
        v5 = sub_F2C0(v5, v2 + 1);
      v16 = (char *)(v5 + v2);
    }
    *v16 = 0;
  }
  return v5;
}



// Function: savedir @ 0xe870
void *savedir(long long a1, unsigned int a2)
{
  DIR *v2; // rax
  DIR *v3; // rbp
  void *v4; // r12

  v2 = (DIR *)sub_10990();
  if ( !v2 )
    return 0;
  v3 = v2;
  v4 = (void *)sub_E5B0(v2, a2);
  if ( !closedir(v3) )
    return v4;
  free(v4);
  return 0;
}



// Function: fd_safer @ 0xe980
long long fd_safer(unsigned int fd)
{
  unsigned int v2; // ebp
  int *v3; // rax
  int v4; // r13d
  int *v5; // rbx

  if ( fd > 2 )
    return fd;
  v2 = sub_10B10();
  v3 = __errno_location();
  v4 = *v3;
  v5 = v3;
  close(fd);
  *v5 = v4;
  return v2;
}



// Function: validate_timespec @ 0xe9e0
long long validate_timespec(long long *a1)
{
  unsigned long long v1; // r8
  unsigned long long v2; // rax
  int v3; // edx
  int v4; // r9d

  v1 = a1[1];
  if ( v1 - 1073741822 > 1 && v1 > 0x3B9AC9FF || (v2 = a1[3], v2 - 1073741822 > 1) && v2 > 0x3B9AC9FF )
  {
    *__errno_location() = 22;
    return 0xFFFFFFFFLL;
  }
  else
  {
    v3 = 0;
    v4 = 0;
    if ( v1 - 1073741822 <= 1 )
    {
      *a1 = 0;
      v4 = 1;
      v3 = v1 == 1073741822;
    }
    if ( v2 - 1073741822 <= 1 )
    {
      a1[2] = 0;
      v4 = 1;
      v3 += v2 == 1073741822;
    }
    return v4 + (unsigned int)(v3 == 1);
  }
}



// Function: update_timespec @ 0xea90
long long update_timespec(long long a1, struct timespec **a2)
{
  struct timespec *v2; // rbx
  __syscall_slong_t tv_nsec; // rax
  __syscall_slong_t v4; // r12
  long long v5; // rax
  __syscall_slong_t v6; // rdx
  long long result; // rax
  __syscall_slong_t v8; // rdx

  v2 = *a2;
  tv_nsec = (*a2)->tv_nsec;
  v4 = (*a2)[1].tv_nsec;
  if ( tv_nsec == 1073741822 )
  {
    result = 1;
    if ( v4 == 1073741822 )
      return result;
    v2->tv_sec = sub_E920();
    v2->tv_nsec = v8;
  }
  else if ( tv_nsec == 0x3FFFFFFF )
  {
    if ( v4 == 0x3FFFFFFF )
    {
      *a2 = 0;
      return 0;
    }
    sub_10A40(*a2);
    v4 = v2[1].tv_nsec;
    if ( v4 == 1073741822 )
      goto LABEL_4;
  }
  else if ( v4 == 1073741822 )
  {
LABEL_4:
    v5 = sub_E940(a1);
    v2[1].tv_nsec = v6;
    v2[1].tv_sec = v5;
    return 0;
  }
  result = 0;
  if ( v4 == 0x3FFFFFFF )
  {
    sub_10A40(v2 + 1);
    return 0;
  }
  return result;
}



// Function: fdutimens @ 0xeb50
long long fdutimens(int fd, char *file, const __m128i *a3)
{
  struct timespec *p_times; // rbx
  __m128i v5; // xmm1
  int v6; // r14d
  int v7; // r12d
  bool v8; // zf
  struct timeval *v10; // r14
  __suseconds_t tv_usec; // rsi
  __suseconds_t v12; // rax
  __suseconds_t v13; // rsi
  bool v15; // al
  __syscall_slong_t v16; // rdx
  bool v17; // bl
  bool v18; // r13
  __m128i si128; // xmm3
  __time_t v20; // rax
  bool v21; // dl
  bool v22; // r13
  __syscall_slong_t v23; // rdx
  struct timeval *tvp; // [rsp+8h] [rbp-140h] BYREF
  struct timespec times; // [rsp+10h] [rbp-138h] BYREF
  __m128i v26; // [rsp+20h] [rbp-128h]
  long long v27[4]; // [rsp+30h] [rbp-118h] BYREF
  struct timeval v28; // [rsp+50h] [rbp-F8h] BYREF
  __m128i v29; // [rsp+60h] [rbp-E8h]
  struct stat buf; // [rsp+70h] [rbp-D8h] BYREF
  unsigned long long v31; // [rsp+108h] [rbp-40h]

  p_times = (struct timespec *)a3;
  v31 = __readfsqword(0x28u);
  if ( a3 )
  {
    v5 = _mm_loadu_si128(a3 + 1);
    tvp = (struct timeval *)&times;
    times = (struct timespec)_mm_loadu_si128(a3);
    v26 = v5;
    v6 = sub_E9E0(&times);
    if ( v6 < 0 )
      return (unsigned int)-1;
    p_times = &times;
  }
  else
  {
    tvp = 0;
    v6 = 0;
  }
  if ( fd < 0 && !file )
  {
    *__errno_location() = 9;
    return (unsigned int)-1;
  }
  if ( dword_E0EB8 < 0 )
    goto LABEL_11;
  if ( v6 == 2 )
  {
    if ( fd < 0 )
      v15 = stat(file, &buf) != 0;
    else
      v15 = fstat(fd, &buf) != 0;
    if ( v15 )
      return (unsigned int)-1;
    if ( p_times->tv_nsec == 1073741822 )
    {
      v6 = 3;
      p_times->tv_sec = sub_E920(&buf);
      p_times->tv_nsec = v23;
    }
    else
    {
      v6 = 3;
      if ( p_times[1].tv_nsec == 1073741822 )
      {
        p_times[1].tv_sec = sub_E940(&buf);
        p_times[1].tv_nsec = v16;
      }
    }
  }
  if ( fd < 0 )
  {
    v7 = utimensat(-100, file, p_times, 0);
    v8 = v7 == 0;
    if ( v7 > 0 )
      goto LABEL_10;
  }
  else
  {
    v7 = futimens(fd, p_times);
    v8 = v7 == 0;
    if ( v7 > 0 )
    {
LABEL_10:
      *__errno_location() = 38;
      goto LABEL_11;
    }
  }
  if ( v8 || *__errno_location() != 38 )
  {
    dword_E0EB8 = 1;
    return (unsigned int)v7;
  }
LABEL_11:
  dword_E0EB8 = -1;
  dword_E0EB4 = -1;
  if ( !v6 )
    goto LABEL_18;
  if ( v6 != 3 && (fd < 0 ? stat(file, &buf) != 0 : fstat(fd, &buf) != 0) )
    return (unsigned int)-1;
  if ( p_times )
  {
    v7 = 0;
    if ( (unsigned char)sub_EA90((long long)&buf, (struct timespec **)&tvp) )
      return (unsigned int)v7;
  }
LABEL_18:
  v10 = tvp;
  if ( tvp )
  {
    tv_usec = tvp->tv_usec;
    v27[0] = tvp->tv_sec;
    v12 = tv_usec;
    v27[2] = tvp[1].tv_sec;
    v13 = tvp[1].tv_usec;
    v10 = (struct timeval *)v27;
    v27[1] = v12 / 1000;
    v27[3] = v13 / 1000;
  }
  if ( fd < 0 )
    return (unsigned int)futimesat(-100, file, v10);
  v7 = futimesat(fd, 0, v10);
  if ( v7 )
  {
    v7 = -1;
    if ( file )
      return (unsigned int)utimes(file, v10);
    return (unsigned int)v7;
  }
  if ( v10 )
  {
    v17 = v10->tv_usec > (long long)&unk_7A11F;
    v18 = v10[1].tv_usec > (long long)&unk_7A11F;
    if ( (v10->tv_usec > (long long)&unk_7A11F || v10[1].tv_usec > (long long)&unk_7A11F) && !fstat(fd, &buf) )
    {
      si128 = _mm_load_si128((const __m128i *)&v10[1]);
      v20 = buf.st_atim.tv_sec - v10->tv_sec;
      v21 = buf.st_mtim.tv_sec - v10[1].tv_sec == 1;
      v28 = (struct timeval)_mm_load_si128((const __m128i *)v10);
      v29 = si128;
      v22 = v21 && v18;
      if ( v20 == 1 && v17 && !sub_E8E0(&buf) )
      {
        v28.tv_usec = 0;
        if ( !v22 || sub_E900(&buf) )
          goto LABEL_43;
      }
      else if ( !v22 || sub_E900(&buf) )
      {
        return 0;
      }
      v29.m128i_i64[1] = 0;
LABEL_43:
      futimesat(fd, 0, &v28);
      return 0;
    }
  }
  return (unsigned int)v7;
}



// Function: lutimens @ 0xef90
long long lutimens(char *file, const __m128i *a2)
{
  struct timespec *v2; // rbx
  __m128i v3; // xmm1
  int v4; // r13d
  int v5; // eax
  unsigned int v6; // r12d
  __syscall_slong_t v8; // rdx
  __m128i v9; // rax
  struct timespec *p_times; // [rsp+8h] [rbp-F0h] BYREF
  struct timespec times; // [rsp+10h] [rbp-E8h] BYREF
  __m128i v12; // [rsp+20h] [rbp-D8h]
  struct stat buf; // [rsp+30h] [rbp-C8h] BYREF
  unsigned long long v14; // [rsp+C8h] [rbp-30h]

  v2 = (struct timespec *)a2;
  v14 = __readfsqword(0x28u);
  if ( a2 )
  {
    v3 = _mm_loadu_si128(a2 + 1);
    p_times = &times;
    times = (struct timespec)_mm_loadu_si128(a2);
    v12 = v3;
    v4 = sub_E9E0(&times);
    if ( v4 < 0 )
      return (unsigned int)-1;
    if ( dword_E0EB4 < 0 )
    {
      v2 = &times;
      goto LABEL_12;
    }
    if ( v4 == 2 )
    {
      if ( lstat(file, &buf) )
        return (unsigned int)-1;
      if ( times.tv_nsec == 1073741822 )
      {
        v2 = &times;
        v4 = 3;
        times.tv_sec = sub_E920(&buf);
        times.tv_nsec = v8;
      }
      else
      {
        v2 = &times;
        v4 = 3;
        if ( v12.m128i_i64[1] == 1073741822 )
        {
          v9.m128i_i64[0] = sub_E940(&buf);
          v12 = v9;
        }
      }
    }
    else
    {
      v2 = &times;
    }
  }
  else
  {
    v4 = 0;
    p_times = 0;
    if ( dword_E0EB4 < 0 )
    {
      dword_E0EB4 = -1;
LABEL_21:
      if ( lstat(file, &buf) )
        return (unsigned int)-1;
      goto LABEL_16;
    }
  }
  v5 = utimensat(-100, file, v2, 256);
  v6 = v5;
  if ( v5 <= 0 )
  {
    if ( !v5 || *__errno_location() != 38 )
    {
      dword_E0EB8 = 1;
      dword_E0EB4 = 1;
      return v6;
    }
  }
  else
  {
    *__errno_location() = 38;
  }
LABEL_12:
  dword_E0EB4 = -1;
  if ( !v4 )
    goto LABEL_21;
  if ( v4 == 3 )
  {
    if ( !v2 )
    {
LABEL_16:
      if ( (buf.st_mode & 0xF000) != 0xA000 )
        return (unsigned int)sub_EB50(-1, file, (const __m128i *)p_times);
      *__errno_location() = 38;
      return (unsigned int)-1;
    }
  }
  else
  {
    if ( lstat(file, &buf) )
      return (unsigned int)-1;
    if ( !v2 )
      goto LABEL_16;
  }
  v6 = 0;
  if ( !(unsigned char)sub_EA90((long long)&buf, &p_times) )
    goto LABEL_16;
  return v6;
}



// Function: xmalloc @ 0xf230
void *xmalloc(size_t a1)
{
  void *result; // rax

  result = malloc(a1);
  if ( !result )
    sub_D550();
  return result;
}



// Function: ximalloc @ 0xf250
void *ximalloc(size_t a1)
{
  void *result; // rax

  result = j__malloc(a1);
  if ( !result )
    sub_D550();
  return result;
}



// Function: xrealloc @ 0xf280
void *xrealloc(void *a1, size_t a2)
{
  void *result; // rax

  result = realloc(a1, a2);
  if ( !result && (!a1 || a2) )
    sub_D550();
  return result;
}



// Function: xirealloc @ 0xf2c0
long long xirealloc()
{
  long long result; // rax

  result = sub_10AB0();
  if ( !result )
    sub_D550();
  return result;
}



// Function: xreallocarray @ 0xf2e0
long long xreallocarray(long long a1, long long a2, long long a3)
{
  long long result; // rax

  result = reallocarray();
  if ( !result && (!a1 || a2 && a3) )
    sub_D550();
  return result;
}



// Function: xireallocarray @ 0xf330
long long xireallocarray()
{
  long long result; // rax

  result = sub_10AE0();
  if ( !result )
    sub_D550();
  return result;
}



// Function: x2nrealloc @ 0xf370
long long x2nrealloc(long long a1, unsigned long long *a2, unsigned long long a3)
{
  unsigned long long v3; // rbx
  long long v4; // rax
  bool v5; // cf
  long long result; // rax

  v3 = *a2;
  if ( a1 )
  {
    v4 = (v3 >> 1) + 1;
    v5 = __CFADD__(v4, v3);
    v3 += v4;
    if ( v5 )
      sub_D550();
  }
  else if ( !v3 )
  {
    v3 = 0x80 / a3 + (a3 > 0x80);
  }
  result = sub_F2E0(a1, v3, a3);
  *a2 = v3;
  return result;
}



// Function: xpalloc @ 0xf3f0
void *xpalloc(void *a1, long long *a2, signed long long a3, long long a4, signed long long a5)
{
  long long v7; // rcx
  long long v8; // rbx
  bool v9; // of
  signed long long v10; // rbx
  signed long long v11; // rax
  signed long long v12; // rsi
  signed long long v13; // rcx
  void *result; // rax
  long long v15; // rsi

  v7 = *a2;
  v8 = *a2 >> 1;
  v9 = __OFADD__(*a2, v8);
  v10 = *a2 + v8;
  if ( v9 )
    v10 = 0x7FFFFFFFFFFFFFFFLL;
  v11 = a4;
  if ( v10 <= a4 )
    v11 = v10;
  if ( a4 >= 0 )
    v10 = v11;
  v12 = a5 * v10;
  if ( !is_mul_ok(a5, v10) )
  {
    v15 = 0x7FFFFFFFFFFFFFFFLL;
LABEL_17:
    v10 = v15 / a5;
    v12 = v15 - v15 % a5;
    if ( a1 )
      goto LABEL_10;
LABEL_18:
    *a2 = 0;
    goto LABEL_10;
  }
  if ( v12 <= 127 )
  {
    v15 = 128;
    goto LABEL_17;
  }
  if ( !a1 )
    goto LABEL_18;
LABEL_10:
  if ( v10 - v7 < a3 )
  {
    if ( (v9 = __OFADD__(a3, v7), v13 = a3 + v7, v10 = v13, v9)
      || v13 > a4 && a4 >= 0
      || (v12 = a5 * v13, !is_mul_ok(a5, v13)) )
    {
      sub_D550();
    }
  }
  result = sub_F280(a1, v12);
  *a2 = v10;
  return result;
}



// Function: xcalloc @ 0xf4d0
void *xcalloc(size_t a1, size_t a2)
{
  void *result; // rax

  result = calloc(a1, a2);
  if ( !result )
    sub_D550();
  return result;
}



// Function: xicalloc @ 0xf500
void *xicalloc(size_t a1, size_t a2)
{
  void *result; // rax

  result = j__calloc(a1, a2);
  if ( !result )
    sub_D550();
  return result;
}



// Function: xmemdup @ 0xf530
void *xmemdup(void *src, size_t n)
{
  void *v2; // rax

  v2 = sub_F230(n);
  return memcpy(v2, src, n);
}



// Function: ximemdup @ 0xf560
void *ximemdup(void *src, size_t n)
{
  void *v2; // rax

  v2 = sub_F250(n);
  return memcpy(v2, src, n);
}



// Function: ximemdup0 @ 0xf590
void *ximemdup0(void *src, size_t n)
{
  char *v2; // rax

  v2 = sub_F250(n + 1);
  v2[n] = 0;
  return memcpy(v2, src, n);
}



// Function: xstrdup @ 0xf5d0
void *xstrdup(char *src)
{
  size_t v1; // rax

  v1 = strlen(src);
  return sub_F530(src, v1 + 1);
}



// Function: yesno @ 0xf5f0
long long yesno()
{
  unsigned int v0; // r12d
  int v1; // eax

  v1 = getchar();
  for ( LOBYTE(v0) = (v1 & 0xFFFFFFDF) == 89; v1 != 10; v1 = getchar() )
  {
    if ( v1 == -1 )
      break;
  }
  return v0;
}



// Function: rpl_fprintf @ 0xf630
long long rpl_fprintf(FILE *s, long long a2, ...)
{
  char *v2; // rax
  size_t v3; // rbx
  char *v4; // rbp
  long long result; // rax
  size_t n; // [rsp+10h] [rbp-8D8h] BYREF
  gcc_va_list va; // [rsp+18h] [rbp-8D0h] BYREF
  char v8[2008]; // [rsp+30h] [rbp-8B8h] BYREF
  unsigned long long v9; // [rsp+808h] [rbp-E0h]

  va_start(va, a2);
  v9 = __readfsqword(0x28u);
  n = 2000;
  v2 = (char *)sub_F880(v8, &n, a2, va);
  v3 = n;
  v4 = v2;
  if ( !v2 )
    goto LABEL_11;
  if ( fwrite(v2, 1u, n, s) >= v3 )
  {
    if ( v4 != v8 )
      free(v4);
    if ( v3 <= 0x7FFFFFFF )
      return (unsigned int)v3;
    *__errno_location() = 75;
LABEL_11:
    sub_10A30(s);
    return 0xFFFFFFFFLL;
  }
  result = 0xFFFFFFFFLL;
  if ( v4 != v8 )
  {
    free(v4);
    return 0xFFFFFFFFLL;
  }
  return result;
}



// Function: __printf__ @ 0xf7b0
long long __printf__()
{
  return sub_108A0(stdout);
}



// Function: vasnprintf @ 0xf880
char *vasnprintf(void *a1, signed long long *a2, char *a3, long long a4, double a5)
{
  size_t v7; // rax
  char *v8; // rax
  signed long long v9; // rbx
  char *v10; // r14
  char *v11; // rax
  size_t v12; // r12
  char *v13; // r8
  char *v14; // r15
  char *v15; // rax
  unsigned long long v16; // rax
  size_t v17; // r9
  void *v18; // r8
  size_t v19; // r13
  char *v20; // rax
  char v21; // dl
  long long v22; // rax
  char *v23; // rcx
  char *v24; // rax
  int v25; // edi
  int v26; // eax
  char *v27; // r12
  char *v28; // rdx
  char *v29; // rsi
  char *v30; // rax
  char *v31; // rsi
  char *v32; // rax
  long long *v34; // rdx
  short v35; // ax
  signed long long v36; // rax
  void *v37; // rsp
  char v38; // al
  long long v39; // rax
  char *v40; // rax
  long long v41; // rax
  char *v42; // rcx
  int v43; // eax
  unsigned long long v44; // rax
  char *v45; // rax
  int *v46; // rax
  long long v47; // rsi
  char *v48; // rdi
  int *v49; // r9
  int v50; // eax
  unsigned long long v51; // rsi
  int v52; // edx
  unsigned long long v53; // r12
  long long v54; // rax
  size_t v55; // rax
  bool v56; // r12
  char *v57; // rax
  long long v58; // r9
  int v59; // eax
  int v60; // eax
  long double v61; // fst7
  char *v62; // rax
  unsigned long long v63; // rax
  char *v64; // rax
  char *v65; // rax
  char *v66; // rax
  char *v67; // rax
  char *v68; // rax
  char *v69; // rax
  unsigned long long v70; // rax
  unsigned long long v71; // rdx
  bool v72; // r12
  char *v73; // rax
  char *v74; // rax
  char *v75; // rax
  long long v76; // [rsp+0h] [rbp-1428h]
  long long v77; // [rsp+8h] [rbp-1420h] BYREF
  long long v78[507]; // [rsp+10h] [rbp-1418h] BYREF
  long double v79; // [rsp+FE8h] [rbp-440h]
  long double v80; // [rsp+FF8h] [rbp-430h]
  long long v81; // [rsp+1008h] [rbp-420h] BYREF
  unsigned long long v82; // [rsp+1010h] [rbp-418h]
  int v83; // [rsp+101Ch] [rbp-40Ch]
  void *v84; // [rsp+1020h] [rbp-408h]
  signed long long *v85; // [rsp+1028h] [rbp-400h]
  char *v86; // [rsp+1030h] [rbp-3F8h]
  long long v87; // [rsp+1038h] [rbp-3F0h]
  char *v88; // [rsp+1040h] [rbp-3E8h]
  char *v89; // [rsp+1048h] [rbp-3E0h]
  void *src; // [rsp+1050h] [rbp-3D8h]
  size_t n; // [rsp+1058h] [rbp-3D0h]
  void *ptr; // [rsp+1060h] [rbp-3C8h]
  int v93; // [rsp+106Ch] [rbp-3BCh] BYREF
  unsigned int v94; // [rsp+1070h] [rbp-3B8h]
  unsigned int v95; // [rsp+1074h] [rbp-3B4h]
  char v96[8]; // [rsp+1078h] [rbp-3B0h] BYREF
  void *v97; // [rsp+1080h] [rbp-3A8h]
  char v98[224]; // [rsp+1088h] [rbp-3A0h] BYREF
  long long v99; // [rsp+1168h] [rbp-2C0h] BYREF
  void *v100; // [rsp+1170h] [rbp-2B8h]
  long long v101; // [rsp+1178h] [rbp-2B0h]
  long long v102; // [rsp+1180h] [rbp-2A8h]
  char v103[616]; // [rsp+1188h] [rbp-2A0h] BYREF
  unsigned long long v104; // [rsp+13F0h] [rbp-38h]

  ptr = a1;
  v85 = a2;
  v104 = __readfsqword(0x28u);
  if ( (int)sub_10DD0(a3, &v99, v96) < 0 )
    return 0;
  if ( (int)sub_10BA0(a4, v96) < 0 )
  {
    if ( v100 != v103 )
      free(v100);
    if ( v97 != v98 )
      free(v97);
    v14 = 0;
    *__errno_location() = 22;
    return v14;
  }
  v7 = sub_10B70(7, v101, v102, 6);
  if ( v7 <= 0xF9F )
  {
    v34 = (long long *)((char *)&v81 - ((v7 + 23) & 0xFFFFFFFFFFFFF000LL));
    v35 = (v7 + 23) & 0xFFF0;
    if ( &v81 != v34 )
    {
      while ( &v77 != v34 )
        ;
    }
    v36 = v35 & 0xFFF;
    v37 = alloca(v36);
    if ( v36 )
      *(long long *)((char *)&v76 + v36) = *(long long *)((char *)&v76 + v36);
    v84 = 0;
    v88 = v78;
  }
  else
  {
    if ( v7 == -1 || (v8 = malloc(v7), (v88 = v8) == 0) )
    {
      src = __errno_location();
      goto LABEL_50;
    }
    v84 = v8;
  }
  v9 = 0;
  if ( ptr )
    v9 = *v85;
  v10 = (char *)v100;
  v11 = a3;
  v87 = 0;
  v12 = 0;
  v13 = v11;
  v14 = (char *)ptr;
  v15 = *(char **)v100;
  if ( *(char **)v100 == v13 )
    goto LABEL_164;
LABEL_10:
  src = v13;
  n = v15 - v13;
  v16 = sub_10B30(v12, v15 - v13);
  v17 = n;
  v18 = src;
  v19 = v16;
  if ( v9 < v16 )
  {
    if ( v9 )
    {
      if ( v9 < 0 )
        goto LABEL_45;
      v9 *= 2LL;
    }
    else
    {
      v9 = 12;
    }
    if ( v9 < v16 )
      v9 = v16;
    if ( v9 == -1 )
      goto LABEL_45;
    if ( !v14 || v14 == ptr )
    {
      v89 = (char *)src;
      LOBYTE(src) = v14 == ptr;
      v65 = (char *)malloc(v9);
      v17 = n;
      v18 = v89;
      if ( !v65 )
        goto LABEL_45;
      if ( v12 && (char)src )
      {
        src = v89;
        v66 = (char *)memcpy(v65, v14, v12);
        v17 = n;
        v18 = src;
        v14 = v66;
      }
      else
      {
        v14 = v65;
      }
    }
    else
    {
      v20 = (char *)realloc(v14, v9);
      if ( !v20 )
        goto LABEL_175;
      v17 = n;
      v18 = src;
      v14 = v20;
    }
  }
  memcpy(&v14[v12], v18, v17);
  while ( 1 )
  {
    if ( v99 == v87 )
    {
      v70 = sub_10B30(v19, 1);
      if ( v70 > v9 )
      {
        if ( v9 )
        {
          v71 = 2 * v9;
          if ( v9 < 0 )
            goto LABEL_45;
        }
        else
        {
          v71 = 12;
        }
        v9 = v71;
        if ( v70 >= v71 )
          v9 = v70;
        if ( v9 == -1 )
        {
LABEL_45:
          src = __errno_location();
LABEL_46:
          if ( v14 != ptr && v14 )
            goto LABEL_176;
          goto LABEL_48;
        }
        v72 = v14 == ptr;
        if ( !v14 || v14 == ptr )
        {
          v74 = (char *)malloc(v9);
          if ( !v74 )
            goto LABEL_45;
          if ( v19 && v72 )
            v14 = (char *)memcpy(v74, v14, v19);
          else
            v14 = v74;
        }
        else
        {
          v73 = (char *)realloc(v14, v9);
          if ( !v73 )
            goto LABEL_175;
          v14 = v73;
        }
      }
      v14[v19] = 0;
      if ( v14 != ptr && v19 + 1 < v9 )
      {
        v75 = (char *)realloc(v14, v19 + 1);
        if ( v75 )
          v14 = v75;
      }
      if ( v84 )
        free(v84);
      if ( v100 != v103 )
        free(v100);
      if ( v97 != v98 )
        free(v97);
      *v85 = v19;
      return v14;
    }
    v21 = v10[72];
    v22 = *((long long *)v10 + 10);
    if ( v21 == 37 )
      break;
    if ( v22 == -1 )
      goto LABEL_243;
    v23 = (char *)v97;
    v24 = (char *)v97 + 32 * v22;
    v25 = *(int *)v24;
    LODWORD(n) = *(int *)v24;
    if ( v21 == 110 )
    {
      v67 = (char *)*((long long *)v24 + 2);
      switch ( v25 )
      {
        case 18:
          *v67 = v19;
          v12 = v19;
          goto LABEL_163;
        case 19:
          *(short *)v67 = v19;
          v12 = v19;
          goto LABEL_163;
        case 20:
          *(int *)v67 = v19;
          v12 = v19;
          goto LABEL_163;
        case 21:
        case 22:
          *(long long *)v67 = v19;
          v12 = v19;
          goto LABEL_163;
        default:
          goto LABEL_243;
      }
    }
    v26 = *((int *)v10 + 4);
    v27 = v88 + 1;
    *v88 = 37;
    if ( (v26 & 1) != 0 )
    {
      v28 = v88;
      v88[1] = 39;
      v27 = v28 + 2;
    }
    if ( (v26 & 2) != 0 )
      *v27++ = 45;
    if ( (v26 & 4) != 0 )
      *v27++ = 43;
    if ( (v26 & 8) != 0 )
      *v27++ = 32;
    if ( (v26 & 0x10) != 0 )
      *v27++ = 35;
    if ( (v26 & 0x40) != 0 )
      *v27++ = 73;
    if ( (v26 & 0x20) != 0 )
      *v27++ = 48;
    v29 = (char *)*((long long *)v10 + 3);
    v30 = (char *)*((long long *)v10 + 4);
    if ( v29 != v30 )
    {
      v89 = v23;
      src = (void *)(v30 - v29);
      memcpy(v27, v29, v30 - v29);
      v23 = v89;
      v27 = &v27[(long long)src];
    }
    v31 = (char *)*((long long *)v10 + 6);
    v32 = (char *)*((long long *)v10 + 7);
    if ( v31 != v32 )
    {
      v89 = v23;
      src = (void *)(v32 - v31);
      memcpy(v27, v31, v32 - v31);
      v23 = v89;
      v27 = &v27[(long long)src];
    }
    switch ( (int)n )
    {
      case 7:
      case 8:
      case 14:
      case 16:
        goto LABEL_63;
      case 9:
      case 10:
        *v27++ = 108;
LABEL_63:
        *v27++ = 108;
        break;
      case 12:
        *v27++ = 76;
        break;
      default:
        break;
    }
    v38 = v10[72];
    v27[1] = 0;
    *v27 = v38;
    v39 = *((long long *)v10 + 5);
    if ( v39 == -1 )
    {
      LODWORD(v89) = 0;
    }
    else
    {
      v40 = &v23[32 * v39];
      if ( *(int *)v40 != 5 )
        goto LABEL_243;
      LODWORD(v89) = 1;
      v94 = *((int *)v40 + 4);
    }
    v41 = *((long long *)v10 + 8);
    if ( v41 != -1 )
    {
      v42 = &v23[32 * v41];
      if ( *(int *)v42 != 5 )
        goto LABEL_243;
      v43 = (int)v89;
      *(&v94 + (unsigned int)v89) = *((int *)v42 + 4);
      LODWORD(v89) = v43 + 1;
    }
    v44 = sub_10B30(v19, 2);
    if ( v44 > v9 )
    {
      if ( v9 )
      {
        if ( v9 < 0 )
          goto LABEL_45;
        v9 *= 2LL;
      }
      else
      {
        v9 = 12;
      }
      if ( v44 >= v9 )
        v9 = v44;
      if ( v9 == -1 )
        goto LABEL_45;
      if ( !v14 || v14 == ptr )
      {
        LOBYTE(src) = v14 == ptr;
        v68 = (char *)malloc(v9);
        if ( !v68 )
          goto LABEL_45;
        if ( v19 && (char)src )
          v14 = (char *)memcpy(v68, v14, v19);
        else
          v14 = v68;
      }
      else
      {
        v45 = (char *)realloc(v14, v9);
        if ( !v45 )
          goto LABEL_45;
        v14 = v45;
      }
    }
    v14[v19] = 0;
    v46 = __errno_location();
    v86 = v27;
    src = v46;
    v83 = *v46;
    while ( 2 )
    {
      v47 = 0x7FFFFFFF;
      v93 = -1;
      *(int *)src = 0;
      if ( v9 - v19 <= 0x7FFFFFFF )
        v47 = v9 - v19;
      switch ( (int)n )
      {
        case 1:
          v48 = &v14[v19];
          v58 = (unsigned int)*((char *)v97 + 32 * *((long long *)v10 + 10) + 16);
          v59 = (int)v89;
          if ( (int)v89 == 1 )
            goto LABEL_134;
          goto LABEL_108;
        case 2:
          v48 = &v14[v19];
          v58 = *((unsigned char *)v97 + 32 * *((long long *)v10 + 10) + 16);
          v59 = (int)v89;
          if ( (int)v89 == 1 )
            goto LABEL_134;
          goto LABEL_108;
        case 3:
          v48 = &v14[v19];
          v58 = (unsigned int)*((short *)v97 + 16 * *((long long *)v10 + 10) + 8);
          v59 = (int)v89;
          if ( (int)v89 == 1 )
            goto LABEL_134;
          goto LABEL_108;
        case 4:
          v48 = &v14[v19];
          v58 = *((unsigned short *)v97 + 16 * *((long long *)v10 + 10) + 8);
          if ( (int)v89 == 1 )
            goto LABEL_134;
          if ( (int)v89 == 2 )
            goto LABEL_138;
          goto LABEL_109;
        case 5:
        case 6:
        case 13:
        case 14:
          v48 = &v14[v19];
          v58 = *((unsigned int *)v97 + 8 * *((long long *)v10 + 10) + 4);
          v59 = (int)v89;
          if ( (int)v89 != 1 )
          {
LABEL_108:
            if ( v59 != 2 )
            {
LABEL_109:
              *(long long *)&v80 = &v93;
              goto LABEL_110;
            }
LABEL_138:
            *(long long *)&v80 = &v93;
            *((long long *)&v79 + 1) = v58;
            goto LABEL_139;
          }
LABEL_134:
          *((long long *)&v80 + 1) = &v93;
          *(long long *)&v80 = v58;
          goto LABEL_135;
        case 7:
        case 8:
        case 9:
        case 10:
        case 15:
        case 16:
        case 17:
          v48 = &v14[v19];
          v49 = (int *)*((long long *)v97 + 4 * *((long long *)v10 + 10) + 2);
          if ( (int)v89 == 1 )
          {
            *((long long *)&v80 + 1) = &v93;
            *(long long *)&v80 = v49;
LABEL_135:
            v58 = v94;
LABEL_110:
            v82 = v47;
            v50 = __snprintf_chk(v48, v47, 1, -1, v88, v58, a5);
            v52 = v93;
            v51 = v82;
            if ( v93 >= 0 )
              goto LABEL_89;
LABEL_111:
            if ( !v86[1] )
            {
              if ( v50 < 0 )
              {
                if ( !*(int *)src )
                {
                  v60 = 22;
                  if ( (v10[72] & 0xEF) == 0x63 )
                    v60 = 84;
                  *(int *)src = v60;
                }
                if ( v14 != ptr && v14 )
                  free(v14);
                if ( v84 )
                  free(v84);
                if ( v100 != v103 )
                  free(v100);
                if ( v97 != v98 )
                  free(v97);
                return 0;
              }
LABEL_92:
              v93 = v50;
              v52 = v50;
              goto LABEL_93;
            }
            v86[1] = 0;
            continue;
          }
          if ( (int)v89 != 2 )
          {
            *(long long *)&v80 = &v93;
LABEL_87:
            v82 = v47;
            v50 = __snprintf_chk(v48, v47, 1, -1, v88, v49, a5);
            v51 = v82;
            goto LABEL_88;
          }
          *(long long *)&v80 = &v93;
          *((long long *)&v79 + 1) = v49;
LABEL_139:
          *(long long *)&v79 = v95;
LABEL_132:
          v82 = v47;
          v50 = __snprintf_chk(v48, v47, 1, -1, v88, v94, a5);
          v51 = v82;
LABEL_88:
          v52 = v93;
          if ( v93 < 0 )
            goto LABEL_111;
LABEL_89:
          if ( v52 < v51 && v14[v52 + v19] )
            goto LABEL_243;
          if ( v52 < v50 )
            goto LABEL_92;
LABEL_93:
          if ( (unsigned int)(v52 + 1) >= v51 )
          {
            if ( v9 - v19 > 0x7FFFFFFE )
            {
              if ( v14 != ptr && v14 )
                free(v14);
              if ( v84 )
                free(v84);
              if ( v100 != v103 )
                free(v100);
              if ( v97 != v98 )
                free(v97);
              v14 = 0;
              *(int *)src = 75;
              return v14;
            }
            v53 = 2 * v9;
            if ( v9 < 0 )
              v53 = -1;
            v54 = sub_10B30(v19, (unsigned int)(v52 + 2));
            v55 = sub_10B90(v54, v53);
            if ( v9 < v55 )
            {
              if ( !v9 )
                v53 = 12;
              if ( v53 >= v55 )
                v55 = v53;
              v9 = v55;
              if ( v55 == -1 )
                goto LABEL_46;
              v56 = v14 == ptr;
              if ( !v14 || v14 == ptr )
              {
                v62 = (char *)malloc(v55);
                if ( !v62 )
                  goto LABEL_46;
                if ( v19 && v56 )
                  v14 = (char *)memcpy(v62, v14, v19);
                else
                  v14 = v62;
              }
              else
              {
                v57 = (char *)realloc(v14, v55);
                if ( !v57 )
                  goto LABEL_46;
                v14 = v57;
              }
            }
            continue;
          }
          v12 = v52 + v19;
          *(int *)src = v83;
LABEL_163:
          v13 = (char *)*((long long *)v10 + 1);
          v15 = (char *)*((long long *)v10 + 11);
          v10 += 88;
          ++v87;
          if ( v15 != v13 )
            goto LABEL_10;
LABEL_164:
          v19 = v12;
          break;
        case 11:
          v48 = &v14[v19];
          a5 = *((double *)v97 + 4 * *((long long *)v10 + 10) + 2);
          if ( (int)v89 == 1 )
          {
            *(long long *)&v80 = &v93;
            goto LABEL_132;
          }
          if ( (int)v89 == 2 )
          {
            *((long long *)&v80 + 1) = &v93;
            *(long long *)&v80 = v95;
            goto LABEL_132;
          }
          v82 = v47;
          v50 = __snprintf_chk(v48, v47, 1, -1, v88, &v93, a5);
          v51 = v82;
          goto LABEL_88;
        case 12:
          v48 = &v14[v19];
          v61 = *((long double *)v97 + 2 * *((long long *)v10 + 10) + 1);
          if ( (int)v89 == 1 )
          {
            *(long long *)&v80 = &v93;
            v79 = v61;
            goto LABEL_132;
          }
          if ( (int)v89 != 2 )
          {
            v49 = &v93;
            v80 = v61;
            goto LABEL_87;
          }
          *(long long *)&v80 = &v93;
          v82 = v47;
          v79 = v61;
          v78[505] = v95;
          v50 = __snprintf_chk(v48, v47, 1, -1, v88, v94, a5);
          v51 = v82;
          goto LABEL_88;
        default:
          goto LABEL_243;
      }
      break;
    }
  }
  if ( v22 != -1 )
LABEL_243:
    abort();
  v63 = sub_10B30(v19, 1);
  v12 = v63;
  if ( v9 >= v63 )
  {
LABEL_162:
    v14[v19] = 37;
    goto LABEL_163;
  }
  if ( !v9 )
  {
    v9 = 12;
    goto LABEL_155;
  }
  if ( v9 < 0 )
    goto LABEL_45;
  v9 *= 2LL;
LABEL_155:
  if ( v9 < v63 )
    v9 = v63;
  if ( v9 == -1 )
    goto LABEL_45;
  if ( !v14 || v14 == ptr )
  {
    LOBYTE(n) = v14 == ptr;
    v69 = (char *)malloc(v9);
    if ( v69 )
    {
      if ( v19 && (char)n )
        v14 = (char *)memcpy(v69, v14, v19);
      else
        v14 = v69;
      goto LABEL_162;
    }
    goto LABEL_45;
  }
  v64 = (char *)realloc(v14, v9);
  if ( v64 )
  {
    v14 = v64;
    goto LABEL_162;
  }
LABEL_175:
  src = __errno_location();
LABEL_176:
  free(v14);
LABEL_48:
  if ( v84 )
    free(v84);
LABEL_50:
  if ( v100 != v103 )
    free(v100);
  if ( v97 != v98 )
    free(v97);
  *(int *)src = 12;
  return 0;
}



// Function: rpl_vfprintf @ 0x108a0
long long rpl_vfprintf(FILE *s, char *a2, long long a3, double a4)
{
  char *v4; // rax
  size_t v5; // rbx
  char *v6; // rbp
  long long result; // rax
  size_t n; // [rsp+18h] [rbp-810h] BYREF
  char v9[2008]; // [rsp+20h] [rbp-808h] BYREF
  unsigned long long v10; // [rsp+7F8h] [rbp-30h]

  v10 = __readfsqword(0x28u);
  n = 2000;
  v4 = sub_F880(v9, (signed long long *)&n, a2, a3, a4);
  v5 = n;
  if ( !v4 )
    goto LABEL_11;
  v6 = v4;
  if ( fwrite(v4, 1u, n, s) >= v5 )
  {
    if ( v6 != v9 )
      free(v6);
    if ( v5 <= 0x7FFFFFFF )
      return (unsigned int)v5;
    *__errno_location() = 75;
LABEL_11:
    sub_10A30(s);
    return 0xFFFFFFFFLL;
  }
  result = 0xFFFFFFFFLL;
  if ( v6 != v9 )
  {
    free(v6);
    return 0xFFFFFFFFLL;
  }
  return result;
}



// Function: opendir_safer @ 0x10990
DIR *opendir_safer(const char *a1)
{
  DIR *v1; // rax
  DIR *v2; // r12
  unsigned int v3; // eax
  int v5; // ebp
  int *v6; // rax
  int *v7; // rbx
  int v8; // r14d
  DIR *v9; // r13
  DIR *v10; // rax

  v1 = opendir(a1);
  v2 = v1;
  if ( !v1 )
    return v2;
  v3 = dirfd(v1);
  if ( v3 > 2 )
    return v2;
  v5 = sub_E270(v3, 1030, 3);
  v6 = __errno_location();
  v7 = v6;
  if ( v5 >= 0 )
  {
    v10 = fdopendir(v5);
    v8 = *v7;
    v9 = v10;
    if ( !v10 )
      close(v5);
  }
  else
  {
    v8 = *v6;
    v9 = 0;
  }
  closedir(v2);
  *v7 = v8;
  return v9;
}



// Function: fseterr @ 0x10a30
void fseterr(int *a1)
{
  *a1 |= 0x20u;
}



// Function: gettime @ 0x10a40
int gettime(struct timespec *tp)
{
  return clock_gettime(0, tp);
}



// Function: current_timespec @ 0x10a50
__time_t current_timespec()
{
  struct timespec v1; // [rsp+0h] [rbp-28h] BYREF
  unsigned long long v2; // [rsp+18h] [rbp-10h]

  v2 = __readfsqword(0x28u);
  sub_10A40(&v1);
  return v1.tv_sec;
}



// Function: dup_safer @ 0x10b10
int dup_safer(int a1)
{
  return sub_E270(a1, 0, 3);
}



// Function: printf_fetchargs @ 0x10ba0
long long printf_fetchargs(int *a1, long long *a2)
{
  long long v2; // r8
  long long v3; // rax
  long long v4; // rsi
  unsigned int v5; // edx
  long long *v6; // r11
  unsigned int v8; // edx
  int *v9; // r11
  unsigned int v10; // edx
  int *v11; // r11
  unsigned int v12; // edx
  int *v13; // r11
  unsigned int v14; // edx
  long long *v15; // r11
  long double *v16; // rdx
  unsigned int v17; // edx
  void **v18; // r11
  void *v19; // rdx
  unsigned int v20; // edx
  const char **v21; // r11
  const char *v22; // rdx

  v2 = *a2;
  v3 = a2[1];
  if ( *a2 )
  {
    v4 = 0;
    while ( 2 )
    {
      switch ( *(int *)v3 )
      {
        case 1:
        case 2:
          v12 = *a1;
          if ( (unsigned int)*a1 > 0x2F )
          {
            v13 = (int *)*((long long *)a1 + 1);
            *((long long *)a1 + 1) = v13 + 2;
          }
          else
          {
            v13 = (int *)(*((long long *)a1 + 2) + v12);
            *a1 = v12 + 8;
          }
          *(char *)(v3 + 16) = *v13;
          goto LABEL_7;
        case 3:
        case 4:
          v10 = *a1;
          if ( (unsigned int)*a1 > 0x2F )
          {
            v11 = (int *)*((long long *)a1 + 1);
            *((long long *)a1 + 1) = v11 + 2;
          }
          else
          {
            v11 = (int *)(*((long long *)a1 + 2) + v10);
            *a1 = v10 + 8;
          }
          *(short *)(v3 + 16) = *v11;
          goto LABEL_7;
        case 5:
        case 6:
        case 0xD:
        case 0xE:
          v8 = *a1;
          if ( (unsigned int)*a1 > 0x2F )
          {
            v9 = (int *)*((long long *)a1 + 1);
            *((long long *)a1 + 1) = v9 + 2;
          }
          else
          {
            v9 = (int *)(*((long long *)a1 + 2) + v8);
            *a1 = v8 + 8;
          }
          *(int *)(v3 + 16) = *v9;
          goto LABEL_7;
        case 7:
        case 8:
        case 9:
        case 0xA:
        case 0x11:
        case 0x12:
        case 0x13:
        case 0x14:
        case 0x15:
        case 0x16:
          v5 = *a1;
          if ( (unsigned int)*a1 > 0x2F )
          {
            v6 = (long long *)*((long long *)a1 + 1);
            *((long long *)a1 + 1) = v6 + 1;
          }
          else
          {
            v6 = (long long *)(*((long long *)a1 + 2) + v5);
            *a1 = v5 + 8;
          }
          *(long long *)(v3 + 16) = *v6;
          goto LABEL_7;
        case 0xB:
          v14 = a1[1];
          if ( v14 > 0xAF )
          {
            v15 = (long long *)*((long long *)a1 + 1);
            *((long long *)a1 + 1) = v15 + 1;
          }
          else
          {
            v15 = (long long *)(*((long long *)a1 + 2) + v14);
            a1[1] = v14 + 16;
          }
          *(long long *)(v3 + 16) = *v15;
          goto LABEL_7;
        case 0xC:
          v16 = (long double *)((*((long long *)a1 + 1) + 15LL) & 0xFFFFFFFFFFFFFFF0LL);
          *((long long *)a1 + 1) = v16 + 1;
          *(long double *)(v3 + 16) = *v16;
          goto LABEL_7;
        case 0xF:
          v20 = *a1;
          if ( (unsigned int)*a1 > 0x2F )
          {
            v21 = (const char **)*((long long *)a1 + 1);
            *((long long *)a1 + 1) = v21 + 1;
          }
          else
          {
            v21 = (const char **)(*((long long *)a1 + 2) + v20);
            *a1 = v20 + 8;
          }
          v22 = *v21;
          if ( !*v21 )
            v22 = "(NULL)";
          *(long long *)(v3 + 16) = v22;
          goto LABEL_7;
        case 0x10:
          v17 = *a1;
          if ( (unsigned int)*a1 > 0x2F )
          {
            v18 = (void **)*((long long *)a1 + 1);
            *((long long *)a1 + 1) = v18 + 1;
          }
          else
          {
            v18 = (void **)(*((long long *)a1 + 2) + v17);
            *a1 = v17 + 8;
          }
          v19 = *v18;
          if ( !*v18 )
            v19 = &unk_14160;
          *(long long *)(v3 + 16) = v19;
LABEL_7:
          ++v4;
          v3 += 32;
          if ( v4 == v2 )
            return 0;
          continue;
        default:
          return 0xFFFFFFFFLL;
      }
    }
  }
  return 0;
}



// Function: printf_parse @ 0x10dd0
long long printf_parse(char *a1, long long *a2, unsigned long long *a3)
{
  void *v3; // r10
  long long *v7; // rcx
  unsigned long long v8; // r9
  long long v9; // r11
  long long v10; // rsi
  char *v11; // rbx
  unsigned long long v13; // r13
  char **v14; // r12
  char v15; // bp
  char *i; // rdx
  int v17; // edx
  int v18; // eax
  int v19; // ecx
  char *v20; // rdi
  unsigned long long v21; // rdx
  char *v22; // rax
  char *v23; // rsi
  int *v24; // r13
  long long v25; // rax
  long long v26; // rbp
  void *v27; // rdi
  long long *v28; // rax
  long long v29; // rax
  char v30; // si
  unsigned long long v31; // rbp
  unsigned long long v32; // r9
  char *v33; // rax
  char *v34; // rax
  unsigned long long v35; // rdx
  char *v36; // rax
  char *v37; // rcx
  char *v38; // rbp
  long long v39; // rax
  char *v40; // rcx
  char v41; // di
  unsigned long long v42; // rbx
  unsigned long long v43; // rdx
  char *v44; // rax
  char *v45; // rsi
  char *v46; // rax
  char *v47; // rax
  char v48; // dl
  char v49; // dl
  void *v50; // rbp
  unsigned long long v51; // rax
  char **v52; // r13
  unsigned long long v53; // r12
  long long v54; // rdi
  void *v55; // rdi
  char *v56; // rdx
  char v57; // al
  unsigned long long v58; // rax
  char *v59; // rdx
  char v60; // al
  unsigned long long v61; // rax
  long long v62; // rax
  unsigned long long v63; // r9
  long long v64; // rax
  char *v65; // rax
  void *v66; // rdi
  char *v67; // rax
  char v68; // cl
  char *v69; // rbx
  unsigned long long v70; // rax
  unsigned long long v71; // rbp
  long long v72; // rdi
  long long v73; // rsi
  long long *v74; // rax
  char *v75; // rax
  unsigned long long v76; // r9
  long long v77; // rax
  char *v78; // rax
  char *v79; // rax
  char v80; // dl
  long long v81; // rbp
  unsigned long long v82; // rax
  char *j; // rbx
  char v84; // si
  long long v85; // rdi
  char *v86; // rcx
  char *v87; // rax
  char *v88; // rax
  unsigned long long v89; // [rsp+0h] [rbp-78h]
  void *v90; // [rsp+0h] [rbp-78h]
  void *v91; // [rsp+0h] [rbp-78h]
  unsigned long long v92; // [rsp+8h] [rbp-70h]
  char *v93; // [rsp+10h] [rbp-68h]
  unsigned long long v94; // [rsp+18h] [rbp-60h]
  void *src; // [rsp+20h] [rbp-58h]
  void *srca; // [rsp+20h] [rbp-58h]
  void *srcb; // [rsp+20h] [rbp-58h]
  void *srch; // [rsp+20h] [rbp-58h]
  int srcj; // [rsp+20h] [rbp-58h]
  int srcc; // [rsp+20h] [rbp-58h]
  void *srcd; // [rsp+20h] [rbp-58h]
  void *srcg; // [rsp+20h] [rbp-58h]
  int srck; // [rsp+20h] [rbp-58h]
  char *srcl; // [rsp+20h] [rbp-58h]
  char *srce; // [rsp+20h] [rbp-58h]
  void *srcf; // [rsp+20h] [rbp-58h]
  void *srci; // [rsp+20h] [rbp-58h]
  char *srcm; // [rsp+20h] [rbp-58h]
  unsigned long long v109; // [rsp+28h] [rbp-50h]
  long long v110; // [rsp+28h] [rbp-50h]
  long long v111; // [rsp+28h] [rbp-50h]
  void *v112; // [rsp+28h] [rbp-50h]
  void *v113; // [rsp+28h] [rbp-50h]
  long long v114; // [rsp+28h] [rbp-50h]
  unsigned long long v115; // [rsp+28h] [rbp-50h]
  void *v116; // [rsp+28h] [rbp-50h]
  void *v117; // [rsp+28h] [rbp-50h]
  void *v118; // [rsp+28h] [rbp-50h]
  unsigned long long v119; // [rsp+28h] [rbp-50h]
  long long v120; // [rsp+28h] [rbp-50h]
  void *v121; // [rsp+28h] [rbp-50h]
  unsigned long long v122; // [rsp+30h] [rbp-48h]
  long long v123; // [rsp+30h] [rbp-48h]
  long long v124; // [rsp+30h] [rbp-48h]
  long long v125; // [rsp+30h] [rbp-48h]
  long long v126; // [rsp+30h] [rbp-48h]
  long long v127; // [rsp+30h] [rbp-48h]
  unsigned long long v128; // [rsp+30h] [rbp-48h]
  long long v129; // [rsp+30h] [rbp-48h]
  unsigned long long v130; // [rsp+38h] [rbp-40h]
  unsigned long long v131; // [rsp+38h] [rbp-40h]
  unsigned long long v132; // [rsp+38h] [rbp-40h]
  unsigned long long v133; // [rsp+38h] [rbp-40h]

  v3 = a2 + 4;
  v7 = a2 + 4;
  v8 = 7;
  v9 = 7;
  *a2 = 0;
  a2[1] = (long long)(a2 + 4);
  v10 = 0;
  *a3 = 0;
  v93 = (char *)(a3 + 2);
  a3[1] = (unsigned long long)(a3 + 2);
  v92 = 0;
  v89 = 0;
  v94 = 0;
  while ( 1 )
  {
    if ( !*a1 )
    {
      v7[11 * v10] = a1;
      a2[2] = v89;
      a2[3] = v92;
      return 0;
    }
    v11 = a1 + 1;
    if ( *a1 == 37 )
      break;
LABEL_3:
    a1 = v11;
  }
  v13 = -1;
  v14 = (char **)&v7[11 * v10];
  *v14 = a1;
  *((int *)v14 + 4) = 0;
  v14[3] = 0;
  v14[4] = 0;
  v14[5] = (char *)-1LL;
  v14[6] = 0;
  v14[7] = 0;
  v14[8] = (char *)-1LL;
  v14[10] = (char *)-1LL;
  v15 = a1[1];
  if ( (unsigned char)(v15 - 48) <= 9u )
  {
    v47 = a1 + 1;
    do
      v48 = *++v47;
    while ( (unsigned char)(v48 - 48) <= 9u );
    v13 = -1;
    if ( v48 == 36 )
    {
      v49 = v15;
      srcb = (void *)v9;
      v50 = v3;
      v51 = 0;
      v52 = (char **)&v7[11 * v10];
      v53 = v8;
      while ( 1 )
      {
        v54 = -1;
        if ( v51 <= 0x1999999999999999LL )
          v54 = 10 * v51;
        v51 = sub_10B30(v54, (char)(v49 - 48));
        v49 = v11[1];
        if ( (unsigned char)(v49 - 48) > 9u )
          break;
        ++v11;
      }
      v8 = v53;
      v14 = v52;
      v9 = (long long)srcb;
      v13 = v51 - 1;
      v3 = v50;
      if ( v51 - 1 > 0xFFFFFFFFFFFFFFFDLL )
      {
LABEL_88:
        v20 = (char *)a3[1];
LABEL_89:
        if ( v93 != v20 )
        {
          v90 = v3;
          free(v20);
          v3 = v90;
        }
        v55 = (void *)a2[1];
        if ( v3 != v55 )
          free(v55);
        *__errno_location() = 22;
        return 0xFFFFFFFFLL;
      }
      v15 = v11[2];
      v11 += 2;
    }
  }
  for ( i = v11 + 1; v15 == 39; ++i )
  {
    v18 = *((int *)v14 + 4) | 1;
LABEL_19:
    *((int *)v14 + 4) = v18;
    v15 = *i;
    v11 = i;
  }
  switch ( v15 )
  {
    case ' ':
      v18 = *((int *)v14 + 4) | 8;
      goto LABEL_19;
    case '#':
      v18 = *((int *)v14 + 4) | 0x10;
      goto LABEL_19;
    case '+':
      v18 = *((int *)v14 + 4) | 4;
      goto LABEL_19;
    case '-':
      v18 = *((int *)v14 + 4) | 2;
      goto LABEL_19;
    case '0':
      v18 = *((int *)v14 + 4) | 0x20;
      goto LABEL_19;
    case 'I':
      v18 = *((int *)v14 + 4) | 0x40;
      goto LABEL_19;
    default:
      if ( v15 != 42 )
      {
        if ( (unsigned char)(v15 - 48) <= 9u )
        {
          v14[3] = v11;
          if ( (unsigned char)(*v11 - 48) <= 9u )
          {
            v56 = v11;
            do
              v57 = *++v56;
            while ( (unsigned char)(v57 - 48) <= 9u );
            v58 = v56 - v11;
            v11 = v56;
            if ( v89 >= v58 )
              v58 = v89;
            v89 = v58;
          }
          v14[4] = v11;
          goto LABEL_101;
        }
        goto LABEL_11;
      }
      v29 = 1;
      v14[3] = v11;
      v14[4] = i;
      v30 = v11[1];
      if ( v89 )
        v29 = v89;
      v89 = v29;
      if ( (unsigned char)(v30 - 48) > 9u )
        goto LABEL_49;
      v67 = i;
      do
        v68 = *++v67;
      while ( (unsigned char)(v68 - 48) <= 9u );
      if ( v68 == 36 )
      {
        srcd = v3;
        v69 = i;
        v70 = 0;
        v114 = v9;
        v71 = v8;
        while ( 1 )
        {
          v72 = -1;
          v73 = (char)(v30 - 48);
          if ( v70 <= 0x1999999999999999LL )
            v72 = 10 * v70;
          v70 = sub_10B30(v72, v73);
          v30 = v69[1];
          if ( (unsigned char)(v30 - 48) > 9u )
            break;
          ++v69;
        }
        v8 = v71;
        v3 = srcd;
        v9 = v114;
        v31 = v70 - 1;
        if ( v70 - 1 > 0xFFFFFFFFFFFFFFFDLL )
          goto LABEL_88;
        v14[5] = (char *)v31;
        v11 = v69 + 2;
      }
      else
      {
LABEL_49:
        v14[5] = (char *)v94;
        if ( v94 == -1 )
          goto LABEL_88;
        v31 = v94++;
        v11 = i;
      }
      if ( v8 > v31 )
      {
        v20 = (char *)a3[1];
      }
      else
      {
        v32 = 2 * v8;
        if ( v32 <= v31 )
        {
          v111 = v9;
          srch = v3;
          v62 = sub_10B30(v31, 1);
          v9 = v111;
          v3 = srch;
          v32 = v62;
        }
        v33 = (char *)a3[1];
        if ( v32 > 0x7FFFFFFFFFFFFFFLL )
          goto LABEL_141;
        v122 = v32;
        v110 = v9;
        srca = v3;
        if ( v93 == v33 )
        {
          v34 = (char *)malloc(32 * v32);
          v8 = v122;
          v9 = v110;
          v3 = srca;
        }
        else
        {
          v34 = (char *)realloc(v33, 32 * v32);
          v3 = srca;
          v9 = v110;
          v8 = v122;
        }
        v20 = v34;
        v33 = (char *)a3[1];
        if ( !v20 )
          goto LABEL_141;
        if ( v93 == v33 )
        {
          v128 = v8;
          v120 = v9;
          srci = v3;
          v87 = (char *)memcpy(v20, v93, 32 * *a3);
          v8 = v128;
          v9 = v120;
          v3 = srci;
          v20 = v87;
        }
        a3[1] = (unsigned long long)v20;
      }
      v35 = *a3;
      v36 = &v20[32 * *a3];
      if ( *a3 <= v31 )
      {
        do
        {
          ++v35;
          *(int *)v36 = 0;
          v37 = v36;
          v36 += 32;
        }
        while ( v35 <= v31 );
        *a3 = v35;
        *(int *)v37 = 0;
      }
      v38 = &v20[32 * v31];
      if ( *(int *)v38 )
      {
        if ( *(int *)v38 != 5 )
          goto LABEL_89;
LABEL_101:
        v15 = *v11;
LABEL_11:
        if ( v15 != 46 )
          goto LABEL_12;
        goto LABEL_66;
      }
      *(int *)v38 = 5;
      v15 = *v11;
      if ( *v11 != 46 )
        goto LABEL_12;
LABEL_66:
      if ( v11[1] == 42 )
      {
        v39 = 2;
        v40 = v11 + 2;
        v14[6] = v11;
        v14[7] = v11 + 2;
        if ( v92 >= 2 )
          v39 = v92;
        v41 = v11[2];
        v92 = v39;
        if ( (unsigned char)(v41 - 48) > 9u )
          goto LABEL_70;
        v79 = v11 + 2;
        do
          v80 = *++v79;
        while ( (unsigned char)(v80 - 48) <= 9u );
        if ( v80 == 36 )
        {
          srcf = v3;
          v81 = v9;
          v82 = 0;
          v119 = v8;
          for ( j = v11 + 2; ; ++j )
          {
            v84 = v41 - 48;
            v85 = -1;
            if ( v82 <= 0x1999999999999999LL )
              v85 = 10 * v82;
            v82 = sub_10B30(v85, v84);
            v41 = j[1];
            if ( (unsigned char)(v41 - 48) > 9u )
              break;
          }
          v86 = j;
          v3 = srcf;
          v8 = v119;
          v42 = v82 - 1;
          v9 = v81;
          if ( v82 - 1 > 0xFFFFFFFFFFFFFFFDLL )
            goto LABEL_88;
          v14[8] = (char *)v42;
          v40 = v86 + 2;
        }
        else
        {
LABEL_70:
          v42 = (unsigned long long)v14[8];
          if ( v42 == -1 )
          {
            v14[8] = (char *)v94;
            if ( v94 == -1 )
              goto LABEL_88;
            v42 = v94++;
          }
        }
        if ( v8 <= v42 )
        {
          v76 = 2 * v8;
          if ( v76 <= v42 )
          {
            v126 = v9;
            v117 = v3;
            srcl = v40;
            v77 = sub_10B30(v42, 1);
            v9 = v126;
            v3 = v117;
            v40 = srcl;
            v76 = v77;
          }
          v33 = (char *)a3[1];
          if ( v76 > 0x7FFFFFFFFFFFFFFLL )
            goto LABEL_141;
          v132 = v76;
          v127 = v9;
          v118 = v3;
          srce = v40;
          if ( v93 == v33 )
          {
            v78 = (char *)malloc(32 * v76);
            v8 = v132;
            v9 = v127;
            v3 = v118;
            v40 = srce;
          }
          else
          {
            v78 = (char *)realloc(v33, 32 * v76);
            v40 = srce;
            v3 = v118;
            v9 = v127;
            v8 = v132;
          }
          v20 = v78;
          v33 = (char *)a3[1];
          if ( !v20 )
            goto LABEL_141;
          if ( v93 == v33 )
          {
            v133 = v8;
            v129 = v9;
            v121 = v3;
            srcm = v40;
            v88 = (char *)memcpy(v20, v93, 32 * *a3);
            v8 = v133;
            v9 = v129;
            v3 = v121;
            v40 = srcm;
            v20 = v88;
          }
          a3[1] = (unsigned long long)v20;
        }
        else
        {
          v20 = (char *)a3[1];
        }
        v43 = *a3;
        v44 = &v20[32 * *a3];
        if ( *a3 <= v42 )
        {
          do
          {
            ++v43;
            *(int *)v44 = 0;
            v45 = v44;
            v44 += 32;
          }
          while ( v43 <= v42 );
          *a3 = v43;
          *(int *)v45 = 0;
        }
        v46 = &v20[32 * v42];
        if ( *(int *)v46 )
        {
          if ( *(int *)v46 != 5 )
            goto LABEL_89;
          v15 = *v40;
          v11 = v40;
        }
        else
        {
          *(int *)v46 = 5;
          v11 = v40;
          v15 = *v40;
        }
      }
      else
      {
        v14[6] = v11;
        v59 = v11 + 1;
        if ( (unsigned char)(v11[1] - 48) > 9u )
        {
          ++v11;
          v61 = 1;
        }
        else
        {
          do
            v60 = *++v59;
          while ( (unsigned char)(v60 - 48) <= 9u );
          v61 = v59 - v11;
          v11 = v59;
        }
        v14[7] = v59;
        v15 = *v59;
        if ( v92 >= v61 )
          v61 = v92;
        v92 = v61;
      }
LABEL_12:
      v17 = 0;
      ++v11;
      for ( ; v15 == 104; ++v11 )
      {
        v17 |= 1 << (v17 & 1);
LABEL_16:
        v15 = *v11;
      }
      break;
  }
  switch ( v15 )
  {
    case 'L':
      v17 |= 4u;
      goto LABEL_16;
    case 'Z':
    case 'j':
    case 'l':
    case 't':
    case 'z':
      v17 += 8;
      goto LABEL_16;
    default:
      switch ( v15 )
      {
        case '%':
          goto LABEL_36;
        case 'A':
        case 'E':
        case 'F':
        case 'G':
        case 'a':
        case 'e':
        case 'f':
        case 'g':
          v19 = 12;
          if ( v17 <= 15 )
            v19 = ((v17 & 4) != 0) + 11;
          goto LABEL_28;
        case 'C':
          v19 = 14;
          v15 = 99;
          goto LABEL_28;
        case 'S':
          v19 = 16;
          v15 = 115;
          goto LABEL_28;
        case 'X':
        case 'o':
        case 'u':
        case 'x':
          v19 = 10;
          if ( v17 <= 15 && (v17 & 4) == 0 )
          {
            v19 = 8;
            if ( v17 <= 7 )
            {
              v19 = 2;
              if ( (v17 & 2) == 0 )
                v19 = (v17 & 1) == 0 ? 6 : 4;
            }
          }
          goto LABEL_28;
        case 'c':
          v19 = (v17 > 7) + 13;
          goto LABEL_28;
        case 'd':
        case 'i':
          v19 = 9;
          if ( v17 <= 15 && (v17 & 4) == 0 )
          {
            v19 = 7;
            if ( v17 <= 7 )
            {
              v19 = 1;
              if ( (v17 & 2) == 0 )
                v19 = (v17 & 1) == 0 ? 5 : 3;
            }
          }
          goto LABEL_28;
        case 'n':
          v19 = 22;
          if ( v17 <= 15 && (v17 & 4) == 0 )
          {
            v19 = 21;
            if ( v17 <= 7 )
            {
              v19 = 18;
              if ( (v17 & 2) == 0 )
                v19 = 20 - (v17 & 1);
            }
          }
          goto LABEL_28;
        case 'p':
          v19 = 17;
          goto LABEL_28;
        case 's':
          v19 = (v17 > 7) + 15;
LABEL_28:
          if ( v13 == -1 )
          {
            v14[10] = (char *)v94;
            if ( v94 == -1 )
              goto LABEL_88;
            v13 = v94++;
            if ( v8 > v13 )
            {
LABEL_30:
              v20 = (char *)a3[1];
              goto LABEL_31;
            }
          }
          else
          {
            v14[10] = (char *)v13;
            if ( v8 > v13 )
              goto LABEL_30;
          }
          v63 = 2 * v8;
          if ( v63 <= v13 )
          {
            v123 = v9;
            v112 = v3;
            srcj = v19;
            v64 = sub_10B30(v13, 1);
            v9 = v123;
            v3 = v112;
            v19 = srcj;
            v63 = v64;
          }
          v33 = (char *)a3[1];
          if ( v63 > 0x7FFFFFFFFFFFFFFLL )
            goto LABEL_141;
          v130 = v63;
          v124 = v9;
          v113 = v3;
          srcc = v19;
          if ( v93 == v33 )
          {
            v65 = (char *)malloc(32 * v63);
            v8 = v130;
            v9 = v124;
            v3 = v113;
            v19 = srcc;
          }
          else
          {
            v65 = (char *)realloc(v33, 32 * v63);
            v19 = srcc;
            v3 = v113;
            v9 = v124;
            v8 = v130;
          }
          v20 = v65;
          v33 = (char *)a3[1];
          if ( !v20 )
            goto LABEL_141;
          if ( v93 == v33 )
          {
            v131 = v8;
            v125 = v9;
            v116 = v3;
            srck = v19;
            v75 = (char *)memcpy(v20, v93, 32 * *a3);
            v8 = v131;
            v9 = v125;
            v3 = v116;
            v19 = srck;
            v20 = v75;
          }
          a3[1] = (unsigned long long)v20;
LABEL_31:
          v21 = *a3;
          v22 = &v20[32 * *a3];
          if ( *a3 <= v13 )
          {
            do
            {
              ++v21;
              *(int *)v22 = 0;
              v23 = v22;
              v22 += 32;
            }
            while ( v21 <= v13 );
            *a3 = v21;
            *(int *)v23 = 0;
          }
          v24 = (int *)&v20[32 * v13];
          if ( *v24 )
          {
            if ( *v24 != v19 )
              goto LABEL_89;
          }
          else
          {
            *v24 = v19;
          }
LABEL_36:
          *((char *)v14 + 72) = v15;
          v25 = *a2;
          v14[1] = v11;
          v10 = v25 + 1;
          *a2 = v25 + 1;
          if ( v9 > (unsigned long long)(v25 + 1) )
          {
            v7 = (long long *)a2[1];
            goto LABEL_3;
          }
          if ( v9 >= 0 )
          {
            v26 = 2 * v9;
            if ( (unsigned long long)(2 * v9) <= 0x2E8BA2E8BA2E8BALL )
            {
              v27 = (void *)a2[1];
              v109 = v8;
              src = v3;
              if ( v3 == v27 )
              {
                v28 = malloc(176 * v9);
                v8 = v109;
                v3 = src;
              }
              else
              {
                v28 = realloc(v27, 176 * v9);
                v3 = src;
                v8 = v109;
              }
              v7 = v28;
              if ( v28 )
              {
                if ( v3 == (void *)a2[1] )
                {
                  v115 = v8;
                  srcg = v3;
                  v74 = memcpy(v28, v3, 88 * *a2);
                  v8 = v115;
                  v3 = srcg;
                  v7 = v74;
                }
                a2[1] = (long long)v7;
                v10 = *a2;
                v9 = v26;
                goto LABEL_3;
              }
            }
          }
          v33 = (char *)a3[1];
LABEL_141:
          if ( v93 != v33 )
          {
            v91 = v3;
            free(v33);
            v3 = v91;
          }
          v66 = (void *)a2[1];
          if ( v3 != v66 )
            free(v66);
          *__errno_location() = 12;
          return 0xFFFFFFFFLL;
        default:
          goto LABEL_88;
      }
  }
}


