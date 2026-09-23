// Function: main @ 0x39a0
void main(int a1, char **a2, char **a3)
{
  char *v4; // rdi
  long long v5; // r12
  size_t v6; // rax
  const char *v7; // r12
  char **v8; // r15
  const char *v9; // rdi
  FILE *v10; // rdi
  long long v11; // rdi
  const char *v12; // rsi
  int v13; // eax
  int v14; // edx
  int v15; // r8d
  int v16; // r9d
  int v17; // ecx
  int v18; // eax
  const char **v19; // rbx
  int v20; // edx
  int v21; // ecx
  int v22; // r8d
  int v23; // r9d
  const char *v24; // rsi
  int v25; // edx
  int v26; // ecx
  int v27; // r8d
  int v28; // r9d
  int v29; // edx
  int v30; // ecx
  int v31; // r8d
  int v32; // r9d
  FILE *v33; // rdi
  const char *v34; // rdx
  char v35; // al
  char *v36; // rdx
  FILE *v37; // rdi
  const char *v38; // rdx
  size_t v39; // rax
  int v40; // eax
  int *v41; // r15
  int v42; // r14d
  __sigset_t *p_sa_mask; // rdi
  long long v44; // rcx
  sigset_t *v45; // rsi
  long long v46; // r15
  int v47; // r14d
  const char *v48; // r14
  int v49; // r14d
  FILE *v50; // rdi
  const char *v51; // rdx
  const char *v52; // rcx
  char v53; // [rsp+0h] [rbp-F8h]
  char v54; // [rsp+0h] [rbp-F8h]
  char v55; // [rsp+0h] [rbp-F8h]
  char v56; // [rsp+0h] [rbp-F8h]
  char v57; // [rsp+0h] [rbp-F8h]
  int v58; // [rsp+4h] [rbp-F4h]
  char *v59; // [rsp+8h] [rbp-F0h]
  int v60; // [rsp+8h] [rbp-F0h]
  int argc; // [rsp+10h] [rbp-E8h] BYREF
  int longind; // [rsp+14h] [rbp-E4h] BYREF
  char **argv; // [rsp+18h] [rbp-E0h] BYREF
  struct sigaction oact; // [rsp+20h] [rbp-D8h] BYREF
  unsigned long long v65; // [rsp+B8h] [rbp-40h]

  v58 = a1;
  v4 = *a2;
  v65 = __readfsqword(0x28u);
  qword_1A850 = sub_D290(v4);
  v5 = qword_1A850;
  v6 = strlen((const char *)qword_1A850);
  if ( v6 > 4 )
  {
    v7 = (const char *)(v5 + v6 - 4);
    if ( !strcmp(v7, ".exe") )
      *v7 = 0;
  }
  argv = a2;
  v8 = (char **)sub_D2B0(&argc, &argv, "GZIP");
  ptr = v8;
  if ( v8 )
    v8 = argv;
  src = ".gz";
  qword_DC9C0 = strlen(".gz");
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
    v17 = v13 + 131;
    if ( v13 >= 0 )
    {
      v18 = v13 + 82;
      goto LABEL_13;
    }
    if ( optind != argc )
    {
      v50 = stderr;
      sub_F6B0(stderr, v53);
      sub_69D0(v50, "%s: %s: non-option in GZIP environment variable\n", v51, v52);
    }
    if ( optind != 1 && !dword_1A85C )
      sub_F6B0(stderr, v53);
    free(v8);
    longind = -1;
    optind = 1;
  }
  while ( 2 )
  {
    v11 = (unsigned int)v58;
    v12 = (const char *)a2;
    v17 = getopt_long(v58, a2, "ab:cdfhH?klLmMnNqrS:tvVZ123456789", &longopts, &longind);
    if ( v17 >= 0 )
    {
      v18 = v17 - 49;
      v8 = 0;
      if ( (unsigned int)(v17 - 49) > 0xD3 )
      {
LABEL_16:
        if ( v17 <= 130 || v17 == 194 )
          sub_69D0(v11, v12);
LABEL_10:
        sub_F6B0(stderr, v53);
        sub_F6B0(stderr, v54);
        v10 = stderr;
        sub_F6B0(stderr, v55);
        sub_69D0(v10, "option not valid in GZIP environment variable\n");
      }
LABEL_13:
      switch ( v18 )
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
          v19 = (const char **)&off_176E0;
          sub_F830((unsigned int)"Usage: %s [OPTION]... [FILE]...\n", qword_1A850, v14, v17, v15, v16, v53);
          v24 = "Compress or uncompress FILEs (by default, compress FILES in-place).";
          do
          {
            ++v19;
            sub_F830((unsigned int)"%s\n", (int)v24, v20, v21, v22, v23, v56);
            v24 = *v19;
          }
          while ( *v19 );
          goto LABEL_29;
        case 27:
          sub_54D0();
LABEL_29:
          sub_69A0();
        case 28:
          dword_18028 = 0;
          longind = -1;
          if ( !v8 )
            continue;
          goto LABEL_8;
        case 29:
        case 160:
          dword_18028 = 0;
          dword_1802C = 0;
          goto LABEL_7;
        case 34:
          v59 = optarg;
          v39 = strlen(optarg);
          longind = -1;
          qword_DC9C0 = v39;
          src = v59;
          if ( !v8 )
            continue;
          goto LABEL_8;
        case 37:
          sub_54D0();
          sub_F830((unsigned int)"\n", (int)v12, v25, v26, v27, v28, v53);
          sub_F830((unsigned int)"Written by Jean-loup Gailly.\n", (int)v12, v29, v30, v31, v32, v57);
          sub_69A0();
        case 41:
          v33 = stderr;
          sub_F6B0(stderr, v53);
          sub_69D0(v33, "%s: -Z not supported in this version\n", v34);
        case 48:
          dword_DC9FC = 1;
          longind = -1;
          if ( !v8 )
            continue;
          goto LABEL_8;
        case 49:
          dword_18024 = strtol(optarg, 0, 10);
          v35 = *optarg;
          if ( *optarg )
          {
            v36 = optarg + 1;
            do
            {
              if ( (unsigned char)(v35 - 48) > 9u )
              {
                v37 = stderr;
                sub_F6B0(stderr, v53);
                sub_69D0(v37, "%s: -b operand is not an integer\n", v38);
              }
              optarg = v36;
              v35 = *v36++;
            }
            while ( v35 );
          }
          goto LABEL_7;
        case 50:
          dword_1A860 = 1;
          longind = -1;
          if ( !v8 )
            continue;
          goto LABEL_8;
        case 51:
          dword_DC9F8 = 1;
          longind = -1;
          if ( !v8 )
            continue;
          goto LABEL_8;
        case 53:
          ++dword_DC9F4;
          longind = -1;
          if ( !v8 )
            continue;
          goto LABEL_8;
        case 58:
          dword_DC9F0 = 1;
          longind = -1;
          if ( !v8 )
            continue;
          goto LABEL_8;
        case 59:
          dword_1A860 = 1;
          dword_1A858 = 1;
          dword_DC9F8 = 1;
          dword_DC9E8 = 1;
          longind = -1;
          if ( !v8 )
            continue;
          goto LABEL_8;
        case 60:
          dword_18028 = 1;
          longind = -1;
          if ( !v8 )
            continue;
          goto LABEL_8;
        case 61:
        case 192:
          dword_18028 = 1;
          dword_1802C = 1;
          goto LABEL_7;
        case 64:
        case 195:
          dword_1A85C = 1;
          dword_DC9E4 = 0;
          goto LABEL_7;
        case 65:
          dword_DC9EC = 1;
          longind = -1;
          if ( !v8 )
            continue;
          goto LABEL_8;
        case 67:
          dword_1A860 = 1;
          dword_DC9F8 = 1;
          dword_1A858 = 1;
          longind = -1;
          if ( !v8 )
            continue;
          goto LABEL_8;
        case 69:
        case 200:
          ++dword_DC9E4;
          dword_1A85C = 0;
          goto LABEL_7;
        case 79:
          byte_DCA01 = 1;
          longind = -1;
          if ( !v8 )
            continue;
          goto LABEL_8;
        case 80:
        case 211:
          dword_1A000 = 1;
          goto LABEL_7;
        case 81:
          byte_DCA00 = 1;
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
          v17 -= 131;
LABEL_25:
          dword_1801C = v17 - 48;
          goto LABEL_7;
        default:
          goto LABEL_16;
      }
    }
    break;
  }
  if ( dword_18028 < 0 )
    dword_18028 = dword_DC9F8;
  if ( dword_1802C < 0 )
    dword_1802C = dword_DC9F8;
  v60 = optind;
  if ( dword_DC9FC )
  {
    if ( dword_1A85C )
    {
      if ( (unsigned long long)(qword_DC9C0 - 1) <= 0x1D )
      {
LABEL_75:
        v40 = 13;
LABEL_76:
        dword_DC920 = v40;
        v41 = dword_13350;
        sigemptyset(&set);
        while ( 1 )
        {
          v42 = *v41;
          sigaction(*v41, 0, &oact);
          if ( oact.sa_handler != (__sighandler_t)((char *)&dword_0 + 1) )
            sigaddset(&set, v42);
          if ( &dword_13350[6] == ++v41 )
          {
            p_sa_mask = &oact.sa_mask;
            v44 = 32;
            v45 = &set;
            v46 = 0;
            while ( v44 )
            {
              LODWORD(p_sa_mask->__val[0]) = v45->__val[0];
              v45 = (sigset_t *)((char *)v45 + 4);
              p_sa_mask = (__sigset_t *)((char *)p_sa_mask + 4);
              --v44;
            }
            oact.sa_flags = 0;
            oact.sa_handler = (__sighandler_t)sub_5600;
            while ( 1 )
            {
              v47 = dword_13350[v46];
              if ( sigismember(&set, v47) )
              {
                if ( !v46 )
                  dword_DC9E0 = 1;
                sigaction(v47, &oact, 0);
              }
              if ( ++v46 == 6 )
              {
                if ( v58 == v60 )
                {
                  sub_6A80();
                }
                else
                {
                  while ( optind < v58 )
                  {
                    v48 = a2[optind++];
                    if ( !strcmp(v48, "-") )
                    {
                      v49 = dword_1A860;
                      sub_6A80();
                      dword_1A860 = v49;
                    }
                    else
                    {
                      sub_6E90(v48);
                    }
                  }
                }
                if ( byte_DC510 && close(0) )
                {
                  strcpy(dword_1A420, "stdin");
                  sub_D4E0();
                }
                if ( !dword_DC9E8 )
                  goto LABEL_110;
                if ( !dword_1A85C && v58 - v60 > 1 )
                  sub_65D0(0xFFFFFFFFLL);
                if ( !(unsigned int)sub_E3E0(stdout) )
                {
LABEL_110:
                  if ( !dword_1A860
                    || (!byte_DCA00 || !fdatasync(1) || *__errno_location() == 22) && !close(1)
                    || *__errno_location() == 9 )
                  {
                    sub_6960(status);
                  }
                }
                sub_D550();
              }
            }
          }
        }
      }
      goto LABEL_97;
    }
    sub_F6B0(stderr, v53);
  }
  if ( (unsigned long long)(qword_DC9C0 - 1) <= 0x1D )
  {
    v40 = dword_1A85C;
    if ( !dword_1A85C )
      goto LABEL_76;
    goto LABEL_75;
  }
LABEL_97:
  sub_F6B0(stderr, v53);
  sub_6960(1);
}



// Function: bi_init @ 0x4380
long long (*bi_init(int a1))(void *buf)
{
  long long (*result)(void *); // rax

  dword_19058 = 0;
  result = 0;
  word_1905C = 0;
  if ( a1 != -1 )
  {
    qword_19050 = (long long)sub_DF80;
    return sub_DF80;
  }
  return result;
}



// Function: send_bits @ 0x43b0
short send_bits(int a1, int a2)
{
  int v2; // edx
  int v4; // eax
  int v5; // ecx
  int v6; // esi
  long long v7; // rcx
  long long v8; // rsi

  v2 = dword_19058;
  LOWORD(v4) = word_1905C | (a1 << dword_19058);
  v5 = 16 - a2;
  v6 = dword_19058 + a2;
  if ( v5 < dword_19058 )
  {
    v7 = (unsigned int)n;
    word_1905C |= a1 << dword_19058;
    v8 = (unsigned int)(n + 1);
    if ( (unsigned int)n > (unsigned int)&unk_3FFFD )
    {
      LODWORD(n) = n + 1;
      *((char *)&word_5B000 + v7) = v4;
      if ( (int)v8 == (int)&unk_40000 )
      {
        sub_D600();
        BYTE1(v4) = HIBYTE(word_1905C);
        v8 = (unsigned int)n;
      }
      LODWORD(n) = v8 + 1;
      *((char *)&word_5B000 + v8) = BYTE1(v4);
      if ( (int)v8 + 1 == (int)&unk_40000 )
        sub_D600();
      v2 = dword_19058;
    }
    else
    {
      *((char *)&word_5B000 + (unsigned int)n) = v4;
      LODWORD(n) = v7 + 2;
      *((char *)&word_5B000 + v8) = BYTE1(v4);
    }
    v6 = v2 + a2 - 16;
    v4 = (int)(unsigned short)a1 >> (16 - v2);
  }
  word_1905C = v4;
  dword_19058 = v6;
  return v4;
}



// Function: bi_reverse @ 0x44a0
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



// Function: bi_windup @ 0x44d0
long long bi_windup()
{
  long long v0; // rax
  char v1; // ch
  long long v2; // rdx
  long long v4; // rax
  int v5; // edx

  if ( dword_19058 <= 8 )
  {
    if ( dword_19058 > 0 )
    {
      v4 = (unsigned int)n;
      LODWORD(n) = n + 1;
      v5 = n;
      *((char *)&word_5B000 + v4) = word_1905C;
      if ( v5 == (int)&unk_40000 )
        goto LABEL_8;
    }
  }
  else
  {
    v0 = (unsigned int)n;
    v1 = HIBYTE(word_1905C);
    v2 = (unsigned int)(n + 1);
    if ( (unsigned int)n <= (unsigned int)&unk_3FFFD )
    {
      *((char *)&word_5B000 + (unsigned int)n) = word_1905C;
      LODWORD(n) = v0 + 2;
      *((char *)&word_5B000 + v2) = v1;
      goto LABEL_4;
    }
    LODWORD(n) = n + 1;
    *((char *)&word_5B000 + v0) = word_1905C;
    if ( (int)v2 == (int)&unk_40000 )
    {
      sub_D600();
      v1 = HIBYTE(word_1905C);
      v2 = (unsigned int)n;
    }
    *((char *)&word_5B000 + v2) = v1;
    LODWORD(n) = v2 + 1;
    if ( (int)v2 + 1 == (int)&unk_40000 )
LABEL_8:
      sub_D600();
  }
LABEL_4:
  dword_19058 = 0;
  word_1905C = 0;
  return 0;
}



// Function: copy_block @ 0x45c0
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

  result = sub_44D0();
  if ( a3 )
  {
    v7 = (unsigned int)n;
    v8 = (unsigned int)(n + 1);
    if ( (unsigned int)n > (unsigned int)&unk_3FFFD )
    {
      LODWORD(n) = n + 1;
      *((char *)&word_5B000 + v7) = a2;
      if ( (int)v8 == (int)&unk_40000 )
      {
        sub_D600();
        v8 = (unsigned int)n;
      }
      v9 = (unsigned int)(v8 + 1);
      LODWORD(n) = v8 + 1;
      *((char *)&word_5B000 + v8) = BYTE1(a2);
      if ( (int)v9 == (int)&unk_40000 )
      {
        sub_D600();
        v9 = (unsigned int)n;
      }
    }
    else
    {
      v9 = (unsigned int)(n + 2);
      *((char *)&word_5B000 + (unsigned int)n) = a2;
      *((char *)&word_5B000 + v8) = BYTE1(a2);
    }
    v10 = (unsigned int)(v9 + 1);
    v11 = ~(char)a2;
    v12 = (unsigned short)~(short)a2 >> 8;
    if ( (unsigned int)v9 > (unsigned int)&unk_3FFFD )
    {
      LODWORD(n) = v9 + 1;
      *((char *)&word_5B000 + v9) = v11;
      if ( (int)v10 == (int)&unk_40000 )
      {
        sub_D600();
        v10 = (unsigned int)n;
        LOBYTE(v12) = (unsigned short)~(short)a2 >> 8;
      }
      result = (unsigned int)(v10 + 1);
      LODWORD(n) = v10 + 1;
      *((char *)&word_5B000 + v10) = v12;
      if ( (int)result == (int)&unk_40000 )
        result = sub_D600();
    }
    else
    {
      v13 = (unsigned int)v9;
      result = (unsigned int)(v9 + 2);
      *((char *)&word_5B000 + v13) = v11;
      LODWORD(n) = result;
      *((char *)&word_5B000 + v10) = v12;
    }
  }
  v14 = &a1[a2];
  if ( a2 )
  {
    do
    {
      while ( 1 )
      {
        result = (unsigned int)n;
        v15 = *a1++;
        LODWORD(n) = n + 1;
        v16 = n;
        *((char *)&word_5B000 + result) = v15;
        if ( v16 == (int)&unk_40000 )
          break;
        if ( a1 == v14 )
          return result;
      }
      result = sub_D600();
    }
    while ( a1 != v14 );
  }
  return result;
}



// Function: longest_match @ 0x4740
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

  v2 = dword_19070;
  v3 = dword_1906C;
  v4 = dword_19064;
  if ( (unsigned int)dword_1906C < 0x7EFA )
    v3 = 32506;
  v5 = dword_19070;
  v6 = v3 - 32506;
  v7 = &byte_3B000[dword_1906C];
  v8 = v7[dword_19070 - 1];
  v9 = v7[dword_19070];
  if ( dword_19070 >= (unsigned int)dword_19060 )
    v4 = (unsigned int)dword_19064 >> 2;
  v10 = 0;
  while ( 1 )
  {
    v11 = &byte_3B000[a1];
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
      if ( *v12 != *v13 || &byte_3B000[dword_1906C + 258] <= v12 )
        goto LABEL_24;
    }
    LODWORD(v12) = (int)v12 + 7;
LABEL_24:
    v14 = (int)v12 - (unsigned int)&byte_3B000[dword_1906C];
    if ( v2 >= v14 )
    {
LABEL_6:
      a1 = *((unsigned short *)&qword_1A880 + (a1 & 0x7FFF));
      if ( v6 >= a1 )
        break;
      goto LABEL_7;
    }
    if ( dword_19080 <= v14 )
    {
      dword_19068 = a1;
      return (unsigned int)v14;
    }
    v16 = a1;
    v2 = v14;
    a1 = *((unsigned short *)&qword_1A880 + (a1 & 0x7FFF));
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
  dword_19068 = v16;
  return (unsigned int)v2;
}



// Function: rsync_roll @ 0x4960
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
      v11 = qword_19090;
      v12 = qword_19088;
      v13 = 0;
      do
      {
        v11 += (unsigned char)byte_3B000[result]
             - (unsigned long long)(unsigned char)byte_3B000[(unsigned int)(result - 4096)];
        if ( v12 == 0xFFFFFFFFLL && (v11 & 0xFFF) == 0 )
        {
          v12 = result;
          v13 = 1;
        }
        ++result;
      }
      while ( v3 > (unsigned int)result );
      qword_19090 = v11;
      if ( v13 )
        qword_19088 = v12;
    }
  }
  else
  {
    v4 = qword_19090;
    v5 = &byte_3B000[(unsigned int)result];
    v6 = &v5[a2];
    v7 = &byte_3B001[(unsigned int)result];
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
        qword_19090 = v4;
        result = 4096;
        goto LABEL_6;
      }
    }
    if ( (char)result )
      qword_19090 = v4;
  }
  return result;
}



// Function: fill_window @ 0x4a60
void fill_window()
{
  int v0; // r12d
  long long v1; // rbx
  int v2; // eax
  char *v3; // rax
  short *v4; // rdx
  short v5; // cx
  short v6; // dx

  v0 = dword_1906C;
  v1 = (unsigned int)dword_1909C;
  if ( dword_1909C + dword_1906C != 65537 && (unsigned int)dword_1906C > 0xFEF9 )
  {
    memcpy((char *)&unk_43000 - 0x8000, &unk_43000, 0x8000u);
    dword_19068 -= 0x8000;
    dword_1906C = v0 - 0x8000;
    if ( qword_19088 != 0xFFFFFFFFLL )
      qword_19088 -= 0x8000;
    qword_19078 -= 0x8000;
    v3 = (char *)&unk_2A880 - 0x10000;
    v4 = (short *)&unk_2A880;
    do
    {
      v5 = *v4 + 0x8000;
      if ( *v4 >= 0 )
        v5 = 0;
      *v4++ = v5;
    }
    while ( (short *)((char *)&unk_20000 + (long long)&unk_2A880 - 0x10000) != v4 );
    do
    {
      v6 = *(short *)v3 + 0x8000;
      if ( *(short *)v3 >= 0 )
        v6 = 0;
      v3 += 2;
      *((short *)v3 - 1) = v6;
    }
    while ( v3 != (char *)&unk_2A880 );
  }
  if ( !dword_190A0 )
  {
    v2 = qword_19050(&byte_3B000[v1 + (unsigned int)dword_1906C]);
    if ( (unsigned int)(v2 - 1) > 0xFFFFFFFD )
    {
      dword_190A0 = 1;
      *(short *)&byte_3B000[(unsigned int)dword_1909C + (unsigned long long)(unsigned int)dword_1906C] = 0;
    }
    else
    {
      dword_1909C += v2;
    }
  }
}



// Function: deflate @ 0x4bc0
long long deflate(int a1)
{
  int v1; // ecx
  int v2; // ecx
  int v3; // eax
  unsigned int v4; // eax
  short v5; // si
  unsigned int v6; // edx
  unsigned int v7; // edi
  unsigned int v8; // r14d
  int v9; // r13d
  unsigned int v10; // r15d
  unsigned int v11; // r12d
  short v12; // ax
  int v13; // esi
  short *v14; // rax
  unsigned int v15; // r8d
  char *v16; // rdi
  int v17; // eax
  long long v18; // rsi
  unsigned int v19; // r15d
  long long v20; // rdx
  long long v21; // rsi
  char *v22; // rdi
  unsigned int v23; // r13d
  long long v24; // rsi
  char *v25; // rdi
  long long v26; // rdx
  int v28; // eax
  int v29; // r12d
  int v30; // r15d
  int v31; // eax
  int v32; // r14d
  int v33; // ecx
  int v34; // edi
  int v35; // eax
  int v36; // esi
  long long v37; // r10
  short *v38; // rsi
  short v39; // r11
  unsigned int v40; // r9d
  long long v41; // rsi
  long long v42; // rdx
  long long v43; // rsi
  char *v44; // rdi
  unsigned int v45; // r13d
  long long v46; // rdx
  short *v47; // rax
  unsigned int v48; // edi
  int v49; // eax
  int v50; // r15d
  int v51; // r13d
  int v52; // r14d
  long long v53; // rsi
  long long v54; // rdx
  long long v55; // rsi
  char *v56; // rdi
  int v57; // eax
  unsigned int v58; // eax
  int v59; // eax
  int v60; // edi
  long long v61; // r13
  int v62; // edx
  int v63; // eax
  int v64; // ecx
  long long v65; // r8
  short *v66; // rcx
  short v67; // r9
  unsigned int v68; // eax
  unsigned int v69; // edx
  int v70; // [rsp+4h] [rbp-44h]
  unsigned int v71; // [rsp+8h] [rbp-40h]
  int v72; // [rsp+8h] [rbp-40h]
  unsigned int v73; // [rsp+Ch] [rbp-3Ch]

  if ( (unsigned int)(a1 - 1) > 8 )
    sub_D3F0("bad pack level");
  memset(&unk_2A880, 0, 0x10000u);
  qword_19088 = 0xFFFFFFFFLL;
  qword_19090 = 0;
  v1 = word_12020[4 * a1 + 1];
  dword_1906C = 0;
  dword_19098 = v1;
  v2 = word_12020[4 * a1];
  qword_19078 = 0;
  dword_19060 = v2;
  v3 = word_12020[4 * a1 + 3];
  dword_19080 = word_12020[4 * a1 + 2];
  dword_19064 = v3;
  v4 = qword_19050(byte_3B000);
  dword_1909C = v4;
  if ( v4 - 1 > 0xFFFFFFFD )
  {
    dword_190A0 = 1;
    dword_1909C = 0;
    if ( a1 > 3 )
      goto LABEL_101;
    goto LABEL_59;
  }
  dword_190A0 = 0;
  if ( v4 <= 0x105 )
  {
    do
      sub_4A60();
    while ( (unsigned int)dword_1909C <= 0x105 && !dword_190A0 );
  }
  v5 = (unsigned char)byte_3B001[0] ^ (unsigned short)(32 * (unsigned char)byte_3B000[0]);
  dword_190A4 = (unsigned char)byte_3B001[0] ^ (32 * (unsigned char)byte_3B000[0]);
  if ( a1 <= 3 )
  {
LABEL_59:
    v45 = dword_1909C;
    dword_19070 = 2;
    if ( !dword_1909C )
    {
      v26 = 0xFFFFFFFFLL;
LABEL_78:
      v24 = (unsigned int)dword_1906C - qword_19078;
      v25 = &byte_3B000[(unsigned int)qword_19078];
      if ( qword_19078 < 0 )
        v25 = 0;
      return sub_ADD0(v25, v24, v26, 1);
    }
    while ( 1 )
    {
LABEL_60:
      v46 = (unsigned int)dword_1906C;
      dword_190A4 = ((unsigned char)byte_3B000[dword_1906C + 2] ^ (unsigned short)(32 * dword_190A4)) & 0x7FFF;
      v47 = (short *)&qword_1A880 + (unsigned int)(dword_190A4 + 0x8000);
      v48 = (unsigned short)*v47;
      *((short *)&qword_1A880 + (dword_1906C & 0x7FFF)) = v48;
      *v47 = v46;
      if ( !v48 || (unsigned int)v46 - v48 > 0x7EFA || (unsigned int)v46 > 0xFEFA )
        goto LABEL_63;
      v58 = sub_4740(v48);
      if ( v58 <= v45 )
        v45 = v58;
      if ( v45 <= 2 )
        break;
      v59 = sub_B160((unsigned int)(dword_1906C - dword_19068), v45 - 3);
      v50 = dword_1A000;
      dword_1909C -= v45;
      v60 = dword_1906C;
      v52 = v59;
      if ( dword_1A000 )
      {
        v70 = dword_1906C;
        sub_4960(dword_1906C, v45);
        v60 = v70;
        if ( v45 > dword_19098 )
        {
LABEL_88:
          v61 = v45 + v60;
          dword_1906C = v61;
          dword_190A4 = (unsigned char)byte_3B000[(unsigned int)(v61 + 1)] ^ (32 * (unsigned char)byte_3B000[v61]);
          goto LABEL_66;
        }
      }
      else if ( v45 > dword_19098 )
      {
        goto LABEL_88;
      }
      LOWORD(v62) = dword_190A4;
      v63 = v60;
      do
      {
        v64 = v63++;
        v65 = v63 & 0x7FFF;
        v62 = ((unsigned char)byte_3B000[v64 + 3] ^ (unsigned short)(32 * v62)) & 0x7FFF;
        v66 = (short *)&qword_1A880 + (unsigned int)(v62 + 0x8000);
        v67 = *v66;
        *((short *)&qword_1A880 + v65) = *v66;
        *v66 = v63;
      }
      while ( v63 != v45 + v60 - 1 );
      dword_190A4 = v62;
      *((short *)&qword_1A880 + v65) = v67;
      *v66 = v60 + v45 - 1;
      dword_1906C = v45 + v60;
LABEL_66:
      if ( v50 && (v53 = (unsigned int)dword_1906C, (unsigned int)dword_1906C > (unsigned long long)qword_19088) )
      {
        v54 = 1;
        v52 = 2;
        qword_19088 = 0xFFFFFFFFLL;
      }
      else
      {
        if ( !v52 )
          goto LABEL_72;
        v53 = (unsigned int)dword_1906C;
        v54 = (unsigned int)(v52 - 1);
      }
      v55 = v53 - qword_19078;
      v56 = &byte_3B000[(unsigned int)qword_19078];
      if ( qword_19078 < 0 )
        v56 = 0;
      sub_ADD0(v56, v55, v54, 0);
      qword_19078 = (unsigned int)dword_1906C;
LABEL_72:
      v57 = dword_1909C;
      v45 = dword_1909C;
      if ( (unsigned int)dword_1909C <= 0x105 )
      {
        while ( !dword_190A0 )
        {
          sub_4A60();
          v57 = dword_1909C;
          v45 = dword_1909C;
          if ( (unsigned int)dword_1909C > 0x105 )
            goto LABEL_60;
        }
        v45 = v57;
        if ( !v57 )
        {
          v26 = (unsigned int)(v52 - 1);
          goto LABEL_78;
        }
      }
    }
    v46 = (unsigned int)dword_1906C;
LABEL_63:
    v49 = sub_B160(0, (unsigned char)byte_3B000[v46]);
    v50 = dword_1A000;
    v51 = dword_1906C;
    v52 = v49;
    if ( dword_1A000 )
      sub_4960(dword_1906C, 1);
    --dword_1909C;
    dword_1906C = v51 + 1;
    goto LABEL_66;
  }
  v6 = dword_1909C;
  if ( !dword_1909C )
  {
LABEL_101:
    v23 = -1;
    goto LABEL_43;
  }
  v7 = 2;
  v8 = 0;
  v9 = 0;
  while ( 1 )
  {
    v10 = dword_1906C;
    v11 = 2;
    v12 = (32 * v5) ^ (unsigned char)byte_3B000[dword_1906C + 2];
    v13 = dword_19068;
    dword_190A4 = v12 & 0x7FFF;
    v14 = (short *)&qword_1A880 + (v12 & 0x7FFFu) + 0x8000;
    v15 = (unsigned short)*v14;
    *((short *)&qword_1A880 + (dword_1906C & 0x7FFF)) = v15;
    *v14 = v10;
    dword_19070 = v7;
    if ( v15 && dword_19098 > v7 )
    {
      if ( v10 - v15 <= 0x7EFA && v10 <= 0xFEFA )
      {
        v71 = v6;
        v68 = sub_4740(v15);
        v69 = v71;
        if ( v68 <= v71 )
          v69 = v68;
        v11 = v69;
        if ( v69 == 3 )
        {
          v10 = dword_1906C;
          v7 = dword_19070;
          if ( (unsigned int)(dword_1906C - dword_19068) >= 0x1001 )
            v11 = 2;
        }
        else
        {
          v7 = dword_19070;
          v10 = dword_1906C;
        }
      }
      else
      {
        v11 = 2;
      }
    }
    if ( v7 > 2 && v11 <= v7 )
    {
      v28 = sub_B160(v10 - v13 - 1, v7 - 3);
      v29 = dword_19070;
      v30 = dword_1A000;
      v9 = v28;
      v31 = dword_19070 - 2;
      v6 = dword_1909C + 1 - dword_19070;
      dword_19070 -= 2;
      dword_1909C = v6;
      v32 = dword_1906C;
      if ( dword_1A000 )
      {
        v73 = v6;
        v72 = v31;
        sub_4960(dword_1906C, v29 - 1);
        v6 = v73;
        v31 = v72;
      }
      LOWORD(v33) = dword_190A4;
      v34 = v31 + v32;
      v35 = v32;
      do
      {
        v36 = v35++;
        v37 = v35 & 0x7FFF;
        v33 = ((unsigned char)byte_3B000[v36 + 3] ^ (unsigned short)(32 * v33)) & 0x7FFF;
        v38 = (short *)&qword_1A880 + (unsigned int)(v33 + 0x8000);
        v39 = *v38;
        *((short *)&qword_1A880 + v37) = *v38;
        *v38 = v35;
      }
      while ( v35 != v34 );
      v40 = v32 + v29 - 1;
      dword_190A4 = v33;
      *((short *)&qword_1A880 + v37) = v39;
      *v38 = v32 + v29 - 2;
      dword_19070 = 0;
      dword_1906C = v40;
      if ( v30 && (v41 = v40, v40 > (unsigned long long)qword_19088) )
      {
        v42 = 1;
        v9 = 2;
        qword_19088 = 0xFFFFFFFFLL;
      }
      else
      {
        if ( !v9 )
        {
          v8 = 0;
          v11 = 2;
          goto LABEL_23;
        }
        v42 = (unsigned int)(v9 - 1);
        v41 = v40;
      }
      v11 = 2;
      v43 = v41 - qword_19078;
      v44 = &byte_3B000[(unsigned int)qword_19078];
      if ( qword_19078 < 0 )
        v44 = 0;
      v8 = 0;
      sub_ADD0(v44, v43, v42, 0);
      v6 = dword_1909C;
      qword_19078 = (unsigned int)dword_1906C;
      goto LABEL_23;
    }
    if ( !v8 )
    {
      if ( dword_1A000 )
      {
        if ( v10 <= (unsigned long long)qword_19088 )
          goto LABEL_36;
        v9 = 2;
        qword_19088 = 0xFFFFFFFFLL;
        v16 = &byte_3B000[(unsigned int)qword_19078];
        if ( qword_19078 < 0 )
          v16 = 0;
        sub_ADD0(v16, v10 - qword_19078, 1, 0);
        qword_19078 = (unsigned int)dword_1906C;
        v10 = dword_1906C;
        if ( dword_1A000 )
LABEL_36:
          sub_4960(v10, 1);
      }
      v8 = 1;
      dword_1906C = v10 + 1;
      v6 = --dword_1909C;
      goto LABEL_23;
    }
    v17 = sub_B160(0, (unsigned char)byte_3B000[v10 - 1]);
    v9 = v17;
    if ( dword_1A000 )
    {
      v18 = (unsigned int)dword_1906C;
      v19 = dword_1906C;
      if ( (unsigned int)dword_1906C <= (unsigned long long)qword_19088 )
      {
        v20 = (unsigned int)(v17 - 1);
        if ( !v17 )
        {
LABEL_40:
          sub_4960(v19, 1);
          goto LABEL_35;
        }
      }
      else
      {
        v20 = v8;
        v9 = 2;
        qword_19088 = 0xFFFFFFFFLL;
      }
LABEL_32:
      v21 = v18 - qword_19078;
      v22 = &byte_3B000[(unsigned int)qword_19078];
      if ( qword_19078 < 0 )
        v22 = 0;
      sub_ADD0(v22, v21, v20, 0);
      qword_19078 = (unsigned int)dword_1906C;
      v19 = dword_1906C;
      if ( !dword_1A000 )
        goto LABEL_35;
      goto LABEL_40;
    }
    v19 = dword_1906C;
    v20 = (unsigned int)(v17 - 1);
    v18 = (unsigned int)dword_1906C;
    if ( v17 )
      goto LABEL_32;
LABEL_35:
    dword_1906C = v19 + 1;
    v6 = --dword_1909C;
LABEL_23:
    if ( v6 <= 0x105 )
    {
      while ( !dword_190A0 )
      {
        sub_4A60();
        v6 = dword_1909C;
        if ( (unsigned int)dword_1909C > 0x105 )
          goto LABEL_28;
      }
      v6 = dword_1909C;
      if ( !dword_1909C )
        break;
    }
LABEL_28:
    v5 = dword_190A4;
    v7 = v11;
  }
  v23 = v9 - 1;
  if ( v8 )
    sub_B160(0, (unsigned char)byte_3B000[dword_1906C - 1]);
LABEL_43:
  v24 = (unsigned int)dword_1906C - qword_19078;
  v25 = &byte_3B000[(unsigned int)qword_19078];
  v26 = v23;
  if ( qword_19078 < 0 )
    v25 = 0;
  return sub_ADD0(v25, v24, v26, 1);
}



// Function: license @ 0x54d0
long long license(long long a1, long long a2, long long a3, int a4, int a5, int a6)
{
  const char **v6; // rbx
  int v7; // edx
  int v8; // ecx
  int v9; // r8d
  int v10; // r9d
  const char *v11; // rsi
  long long result; // rax

  v6 = (const char **)off_17B20;
  sub_F830((unsigned int)"%s %s\n", qword_1A850, (int)off_183F8, a4, a5, a6);
  v11 = "Copyright (C) 2018 Free Software Foundation, Inc.";
  do
  {
    ++v6;
    result = sub_F830((unsigned int)"%s\n", (int)v11, v7, v8, v9, v10);
    v11 = *v6;
  }
  while ( *v6 );
  return result;
}



// Function: remove_output_file @ 0x5530
unsigned long long remove_output_file(char a1)
{
  int v2; // edi
  char *v4; // rax
  char *i; // rdx
  char *v6; // rsi
  char v7; // al
  sigset_t v8; // [rsp+0h] [rbp-4A8h] BYREF
  char name[1032]; // [rsp+80h] [rbp-428h] BYREF
  unsigned long long v10; // [rsp+488h] [rbp-20h]

  v10 = __readfsqword(0x28u);
  if ( a1 )
  {
    v2 = fd;
    if ( fd < 0 )
      return v10 - __readfsqword(0x28u);
  }
  else
  {
    sigprocmask(0, &set, &v8);
    v2 = fd;
    if ( fd < 0 )
    {
LABEL_8:
      sigprocmask(2, &v8, 0);
      return v10 - __readfsqword(0x28u);
    }
  }
  fd = -1;
  close(v2);
  v4 = (char *)&unk_DC520;
  for ( i = name; ; ++i )
  {
    v6 = v4 + 1;
    v7 = *v4;
    *i = v7;
    if ( !v7 )
      break;
    v4 = v6;
  }
  j__unlink(name);
  if ( !a1 )
    goto LABEL_8;
  return v10 - __readfsqword(0x28u);
}



// Function: abort_gzip_signal @ 0x5600
int abort_gzip_signal(int sig)
{
  sub_5530(1);
  if ( dword_DC920 == sig )
    _exit(2);
  signal(sig, 0);
  return raise(sig);
}



// Function: discard_input_bytes @ 0x5690
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
      v3 = (unsigned int)dword_1A008;
      if ( dword_1A008 < (unsigned int)dword_1A00C )
        break;
      while ( 1 )
      {
        v7 = sub_D6C0(0);
        if ( (a2 & 2) != 0 )
LABEL_10:
          sub_D0D0(&v7, 1);
LABEL_4:
        if ( v2 != -1 )
          break;
        if ( !v7 )
          return v8 - __readfsqword(0x28u);
        v3 = (unsigned int)dword_1A008;
        if ( dword_1A008 < (unsigned int)dword_1A00C )
          goto LABEL_3;
      }
      if ( !--v2 )
        return v8 - __readfsqword(0x28u);
    }
LABEL_3:
    v4 = v3 + 1;
    v5 = byte_9C000[v3];
    dword_1A008 = v4;
    v7 = v5;
    if ( (a2 & 2) != 0 )
      goto LABEL_10;
    goto LABEL_4;
  }
  return v8 - __readfsqword(0x28u);
}



// Function: get_method @ 0x5750
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

  v1 = dword_1A00C;
  v63 = __readfsqword(0x28u);
  v2 = (unsigned int)dword_1A008;
  if ( dword_DC9F4 && dword_1A860 )
  {
    if ( dword_1A008 >= (unsigned int)dword_1A00C )
    {
      v11 = sub_D6C0(1);
      v10 = (unsigned int)dword_1A008;
      v1 = dword_1A00C;
      v7 = v11;
    }
    else
    {
      v10 = (unsigned int)(dword_1A008 + 1);
      v7 = (unsigned char)byte_9C000[dword_1A008++];
      LOBYTE(v11) = v7;
    }
    s1[0] = v11;
    if ( (unsigned int)v10 >= v1 )
    {
      v6 = sub_D6C0(1);
      v5 = v6;
    }
    else
    {
      dword_1A008 = v10 + 1;
      v5 = (unsigned char)byte_9C000[v10];
      LOBYTE(v6) = byte_9C000[v10];
    }
  }
  else
  {
    if ( dword_1A008 >= (unsigned int)dword_1A00C )
    {
      v4 = sub_D6C0(0);
      v3 = (unsigned int)dword_1A008;
      v1 = dword_1A00C;
    }
    else
    {
      v3 = (unsigned int)++dword_1A008;
      v4 = byte_9C000[v2];
    }
    s1[0] = v4;
    if ( v4 )
    {
      if ( v1 <= (unsigned int)v3 )
      {
        LOBYTE(v6) = sub_D6C0(0);
      }
      else
      {
        dword_1A008 = v3 + 1;
        LOBYTE(v6) = byte_9C000[v3];
      }
      v5 = 0;
      v7 = 0;
    }
    else
    {
      if ( v1 <= (unsigned int)v3 )
      {
        v6 = sub_D6C0(1);
        v5 = v6;
      }
      else
      {
        dword_1A008 = v3 + 1;
        v5 = (unsigned char)byte_9C000[v3];
        LOBYTE(v6) = byte_9C000[v3];
      }
      v7 = 0;
    }
  }
  s1[1] = v6;
  dword_18020 = -1;
  qword_DFEA8 = 0;
  v8 = ++dword_DC9D8;
  dword_DC9DC = 0;
  if ( !memcmp(s1, &unk_12C6D, 2u) || !memcmp(s1, &unk_12C70, 2u) )
  {
    v12 = (unsigned int)dword_1A008;
    if ( dword_1A008 >= (unsigned int)dword_1A00C )
    {
      v13 = sub_D6C0(0);
    }
    else
    {
      ++dword_1A008;
      v13 = (unsigned char)byte_9C000[v12];
    }
    dword_18020 = v13;
    if ( v13 != 8 )
    {
      sub_F6B0(stderr, v56);
      status = 1;
      return 0xFFFFFFFFLL;
    }
    off_180D0 = sub_CC20;
    v14 = (unsigned int)dword_1A008;
    if ( dword_1A008 >= (unsigned int)dword_1A00C )
    {
      v15 = sub_D6C0(0);
    }
    else
    {
      ++dword_1A008;
      v15 = byte_9C000[v14];
    }
    if ( (v15 & 0x20) != 0 )
      goto LABEL_142;
    if ( (v15 & 0xC0) != 0 )
    {
      sub_F6B0(stderr, v56);
      status = 1;
      if ( dword_DC9F4 <= 1 )
        return 0xFFFFFFFFLL;
      v16 = (unsigned int)dword_1A008;
      v17 = dword_1A00C;
      if ( dword_1A008 < (unsigned int)dword_1A00C )
      {
LABEL_32:
        v18 = (unsigned int)(v16 + 1);
        dword_1A008 = v18;
        v19 = (unsigned char)byte_9C000[v16];
        if ( v17 > (unsigned int)v18 )
          goto LABEL_33;
        goto LABEL_86;
      }
    }
    else
    {
      v16 = (unsigned int)dword_1A008;
      v17 = dword_1A00C;
      if ( dword_1A008 < (unsigned int)dword_1A00C )
        goto LABEL_32;
    }
    v43 = sub_D6C0(0);
    v18 = (unsigned int)dword_1A008;
    v17 = dword_1A00C;
    v19 = v43;
    if ( dword_1A00C > (unsigned int)dword_1A008 )
    {
LABEL_33:
      v20 = (unsigned int)(v18 + 1);
      v21 = (unsigned char)byte_9C000[v18];
      dword_1A008 = v20;
      v22 = (v21 << 8) | v19;
      if ( (unsigned int)v20 < v17 )
        goto LABEL_34;
      goto LABEL_87;
    }
LABEL_86:
    v44 = sub_D6C0(0);
    v20 = (unsigned int)dword_1A008;
    v17 = dword_1A00C;
    v22 = ((long long)v44 << 8) | v19;
    if ( dword_1A008 < (unsigned int)dword_1A00C )
    {
LABEL_34:
      v23 = (unsigned int)(v20 + 1);
      v24 = (unsigned char)byte_9C000[v20];
      dword_1A008 = v23;
      v25 = (v24 << 16) | v22;
      if ( v17 > (unsigned int)v23 )
      {
LABEL_35:
        dword_1A008 = v23 + 1;
        v26 = (unsigned long long)(unsigned char)byte_9C000[v23] << 24;
        goto LABEL_36;
      }
LABEL_88:
      v26 = (long long)(int)sub_D6C0(0) << 24;
LABEL_36:
      v27 = v26 | v25;
      if ( v27 && !dword_18028 )
      {
        if ( v27 < 0 )
        {
          if ( !dword_1A85C )
            sub_F6B0(stderr, v56);
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
      v28 = (unsigned int)dword_1A008;
      v29 = dword_1A00C;
      if ( dword_1A008 >= (unsigned int)dword_1A00C )
      {
        v31 = sub_D6C0(0);
        v30 = (unsigned int)dword_1A008;
        v29 = dword_1A00C;
      }
      else
      {
        v30 = (unsigned int)++dword_1A008;
        v31 = byte_9C000[v28];
      }
      v61 = v31;
      if ( v29 <= (unsigned int)v30 )
      {
        v32 = sub_D6C0(0);
      }
      else
      {
        dword_1A008 = v30 + 1;
        v32 = byte_9C000[v30];
      }
      v62 = v32;
      v33 = v15 & 2;
      if ( (v15 & 2) != 0 )
      {
        s1[2] = 8;
        s1[3] = v15;
        v60 = v27;
        sub_D0D0(0, 0);
        sub_D0D0(s1, 10);
      }
      if ( (v15 & 4) != 0 )
      {
        v34 = (unsigned int)dword_1A008;
        v35 = dword_1A00C;
        if ( dword_1A008 >= (unsigned int)dword_1A00C )
        {
          v37 = sub_D6C0(0);
          v36 = (unsigned int)dword_1A008;
          v35 = dword_1A00C;
        }
        else
        {
          v36 = (unsigned int)++dword_1A008;
          v37 = byte_9C000[v34];
        }
        v58[0] = v37;
        v38 = v37;
        if ( v35 <= (unsigned int)v36 )
        {
          v39 = sub_D6C0(0);
        }
        else
        {
          dword_1A008 = v36 + 1;
          v39 = byte_9C000[v36];
        }
        v58[1] = v39;
        v40 = v38 | (v39 << 8);
        if ( dword_DC9E4 )
          sub_F6B0(stderr, v56);
        if ( v33 )
          sub_D0D0(v58, 2);
        sub_5690(v40, v15);
      }
      if ( (v15 & 8) != 0 )
      {
        if ( dword_1802C || dword_1A860 && !dword_DC9E8 || dword_DC9D8 > 1 )
        {
          sub_5690(-1, v15);
        }
        else
        {
          dest = (char *)sub_D290(file);
          v41 = dest;
          while ( 1 )
          {
            if ( dword_1A008 < (unsigned int)dword_1A00C )
              v42 = byte_9C000[dword_1A008++];
            else
              v42 = sub_D6C0(0);
            *v41++ = v42;
            if ( !v42 )
              break;
            if ( v41 >= dword_1A420 )
              sub_D3F0("corrupted input -- file name too large");
          }
          if ( v33 )
            sub_D0D0(dest, (unsigned int)((int)v41 - (int)dest));
          v54 = (const char *)sub_D290(dest);
          v55 = strlen(v54);
          memmove(dest, v54, v55 + 1);
        }
      }
      if ( (v15 & 0x10) != 0 )
        sub_5690(-1, v15);
      if ( v33 )
      {
        v46 = sub_D0D0(s1, 0);
        v47 = dword_1A00C;
        v48 = v46;
        v49 = (unsigned int)dword_1A008;
        if ( dword_1A008 >= (unsigned int)dword_1A00C )
        {
          v53 = sub_D6C0(0);
          v50 = (unsigned int)dword_1A008;
          v47 = dword_1A00C;
          v51 = v53;
        }
        else
        {
          v50 = (unsigned int)++dword_1A008;
          v51 = (unsigned char)byte_9C000[v49];
        }
        if ( (unsigned int)v50 >= v47 )
        {
          v52 = (unsigned int)sub_D6C0(0) << 8;
        }
        else
        {
          dword_1A008 = v50 + 1;
          v52 = (unsigned char)byte_9C000[v50] << 8;
        }
        if ( v48 != (v51 | v52) )
        {
          sub_F6B0(stderr, v56);
          status = 1;
          if ( dword_DC9F4 <= 1 )
            return 0xFFFFFFFFLL;
        }
      }
      result = (unsigned int)dword_18020;
      if ( dword_DC9D8 != 1 )
      {
        if ( dword_18020 >= 0 )
          return result;
        goto LABEL_71;
      }
      qword_DFEA8 = (unsigned int)(dword_1A008 + 8);
      if ( dword_18020 < 0 )
        goto LABEL_142;
      return result;
    }
LABEL_87:
    v45 = sub_D6C0(0);
    v23 = (unsigned int)dword_1A008;
    v25 = ((long long)v45 << 16) | v22;
    if ( dword_1A00C > (unsigned int)dword_1A008 )
      goto LABEL_35;
    goto LABEL_88;
  }
  if ( !memcmp(s1, &unk_12C90, 2u) && dword_1A008 == 2 && !memcmp(byte_9C000, &unk_12C90, 4u) )
  {
    dword_1A008 = 0;
    off_180D0 = sub_CC20;
    if ( !(unsigned int)sub_CB20(a1) )
    {
      dword_DC9DC = 1;
      goto LABEL_139;
    }
    return 0xFFFFFFFFLL;
  }
  if ( !memcmp(s1, &unk_12C95, 2u) )
  {
    dword_18020 = 2;
    off_180D0 = sub_C590;
    return 2;
  }
  if ( !memcmp(s1, &unk_12C98, 2u) )
  {
    dword_18020 = 1;
    dword_DC9DC = 1;
    off_180D0 = sub_BE90;
    return 1;
  }
  if ( !memcmp(s1, &unk_12C9B, 2u) )
  {
    dword_18020 = 3;
    dword_DC9DC = 1;
    off_180D0 = sub_B920;
    return 3;
  }
  if ( !dword_DC9F4 || !dword_1A860 || (result = (unsigned int)dword_DC9E8, dword_DC9E8) )
  {
LABEL_141:
    if ( v8 == 1 )
    {
LABEL_142:
      sub_F6B0(stderr, v56);
      status = 1;
      return 0xFFFFFFFFLL;
    }
LABEL_71:
    if ( s1[0] )
      goto LABEL_74;
    while ( !v5 )
    {
      if ( dword_1A008 >= (unsigned int)dword_1A00C )
        v5 = sub_D6C0(1);
      else
        v5 = (unsigned char)byte_9C000[dword_1A008++];
    }
    if ( v5 == -1 )
    {
      if ( dword_DC9E4 )
      {
        if ( !dword_1A85C )
          sub_F6B0(stderr, v56);
        if ( !status )
          status = 2;
      }
      return 4294967293LL;
    }
    else
    {
LABEL_74:
      if ( !dword_1A85C )
        sub_F6B0(stderr, v56);
      result = 4294967294LL;
      if ( !status )
        status = 2;
    }
    return result;
  }
  dword_18020 = 0;
  off_180D0 = (long long (*)())sub_D770;
  if ( v5 != -1 )
    --dword_1A008;
  dword_DC9DC = 1;
  if ( v7 != -1 )
  {
    sub_D5A0(1, s1, 1u);
LABEL_139:
    result = (unsigned int)dword_18020;
    if ( dword_18020 >= 0 )
      return result;
    v8 = dword_DC9D8;
    goto LABEL_141;
  }
  return result;
}



// Function: get_suffix @ 0x63e0
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

  v1 = off_18048;
  v2 = src;
  v19 = __readfsqword(0x28u);
  if ( off_18048 )
  {
    v3 = qword_DC9C0;
    v4 = &off_18048;
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
    v9 = &off_18048;
    v6 = (char *)sub_F640(v2);
    sub_D230(v6);
    v8 = 8;
    v7 = &off_18048 - 1;
  }
  else
  {
LABEL_6:
    v6 = (char *)sub_F640(v2);
    sub_D230(v6);
    v7 = (char **)&unk_18040;
    v8 = 0;
    v9 = (char **)&unk_18040;
  }
  v7[v8] = v6;
  v17 = strlen(a1);
  if ( v17 > 32 )
    __strcpy_chk(v18, &a1[v17 - 32], 33);
  else
    __strcpy_chk(v18, a1, 33);
  sub_D230(v18);
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



// Function: do_list @ 0x65d0
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

  if ( dword_18010 )
  {
    if ( a1 >= 0 )
    {
      dword_18010 = 0;
      if ( dword_DC9E4 )
        sub_F830((unsigned int)"method  crc     date  time  ", a2, a3, a4, a5, a6);
      if ( !dword_1A85C )
        sub_F830((unsigned int)"%*.*s %*.*s  ratio uncompressed_name\n", 19, 19, (unsigned int)"compressed", 19, 19);
LABEL_7:
      v6 = -1;
      if ( a1 != 8 || dword_DC9DC )
      {
        if ( !dword_DC9E4 )
          goto LABEL_9;
      }
      else
      {
        v6 = qword_DFE90;
        if ( !dword_DC9E4 )
          goto LABEL_9;
      }
      v41 = localtime(&timer);
      v42 = off_17680[a1];
      sub_F830((unsigned int)"%5s %08lx ", (int)v42, v6, v43, v44, v45);
      if ( v41 )
        sub_F830(
          (unsigned int)"%s%3d %02d:%02d ",
          (unsigned int)&unk_132E0 + 4 * v41->tm_mon,
          v41->tm_mday,
          v41->tm_hour,
          v41->tm_min,
          v49);
      else
        sub_F830((unsigned int)"??? ?? ??:?? ", (int)v42, v46, v47, v48, v49);
LABEL_9:
      v7 = qword_1A828;
      sub_D8A0(stdout);
      sub_F830((unsigned int)" ", v7, v8, v9, v10, v11);
      v12 = qword_1A820;
      sub_D8A0(stdout);
      sub_F830((unsigned int)" ", v12, v13, v14, v15, v16);
      v17 = qword_1A828;
      if ( qword_1A828 == -1 )
      {
        qword_DC508 = -1;
        v18 = 0;
        v19 = 0;
        v17 = 0;
        qword_DFEA8 = 0;
        qword_1A820 = 0;
        qword_1A828 = 0;
      }
      else
      {
        if ( qword_DC508 >= 0 )
          qword_DC508 += qword_1A828;
        v18 = qword_1A820;
        v19 = qword_1A820;
        if ( qword_1A820 == -1 )
        {
          qword_DC500 = -1;
          v20 = 0;
          v18 = 0;
          qword_DFEA8 = 0;
          qword_1A820 = 0;
          qword_1A828 = 0;
LABEL_14:
          sub_D850(v20, v18, stdout);
          sub_F830((unsigned int)" %s\n", (unsigned int)file, v21, v22, v23, v24);
          return;
        }
      }
      if ( qword_DC500 >= 0 )
        qword_DC500 += v19;
      v20 = v18 - (v17 - qword_DFEA8);
      goto LABEL_14;
    }
  }
  else if ( a1 >= 0 )
  {
    goto LABEL_7;
  }
  v25 = qword_DC508;
  if ( qword_DC508 > 0 && qword_DC500 > 0 )
  {
    if ( dword_DC9E4
      && (sub_F830((unsigned int)"                            ", qword_DC508, a3, a4, dword_DC9E4, a6),
          v25 = qword_DC508,
          dword_DC9E4)
      || !dword_1A85C )
    {
      sub_D8A0(stdout);
      sub_F830((unsigned int)" ", v25, v26, v27, v28, v29);
      v30 = qword_DC500;
      sub_D8A0(stdout);
      sub_F830((unsigned int)" ", v30, v31, v32, v33, v34);
      v25 = qword_DC508;
    }
    v35 = qword_DC500 - (v25 - qword_DFEA8);
    v36 = qword_DC500;
    sub_D850(v35, qword_DC500, stdout);
    sub_F830((unsigned int)" (totals)\n", v36, v37, v38, v39, v40);
  }
}



// Function: do_exit @ 0x6960
void do_exit(int status)
{
  if ( !dword_DC040 )
  {
    dword_DC040 = 1;
    free(ptr);
    ptr = 0;
  }
  exit(status);
}



// Function: finish_out @ 0x69a0
void finish_out()
{
  if ( (unsigned int)sub_E060(stdout) )
    sub_D550();
  sub_6960(0);
}



// Function: try_help @ 0x69d0
void try_help(long long a1, const char *a2)
{
  char v2; // [rsp-8h] [rbp-8h]

  sub_F6B0(stderr, v2);
  sub_6960(1);
}



// Function: treat_stdin @ 0x6a80
void treat_stdin(long long a1, long long a2, long long a3, long long a4, long long a5, long long a6, char a7)
{
  int v7; // edx
  int v8; // ecx
  int v9; // r9d
  int v10; // r8d
  int v11; // eax
  char v12; // [rsp+0h] [rbp-8h]

  if ( !(dword_DC9E8 | dword_DC9F4) && (byte_DCA01 || isatty(dword_DC9F8 == 0)) )
  {
    if ( !dword_1A85C )
      sub_F6B0(stderr, v12);
LABEL_27:
    sub_6960(1);
  }
  strcpy(dword_1A420, "stdin");
  strcpy(file, "stdout");
  if ( fstat(0, &buf) )
  {
    sub_5640("standard input");
    goto LABEL_27;
  }
  sub_6A00();
  sub_D150();
  dword_1A860 = 1;
  dword_DC9D8 = 0;
  dword_1A014 = 0;
  byte_DC510 = 1;
  if ( dword_DC9F8 )
  {
    v11 = sub_5750(0);
    dword_18020 = v11;
    if ( v11 < 0 )
      sub_6960(status);
  }
  while ( !(unsigned int)off_180D0() )
  {
    v10 = dword_DC9F8;
    if ( !dword_DC9F8 || dword_DC9DC )
    {
LABEL_13:
      if ( dword_DC9E8 )
      {
        sub_65D0(dword_18020, dword_DC9E8, v7, v8, v10, v9);
      }
      else if ( dword_DC9E4 )
      {
        if ( dword_1A858 )
        {
          sub_F6B0(stderr, a7);
        }
        else if ( !dword_DC9F8 )
        {
          sub_D850(qword_DFEA8 + qword_1A828 - qword_1A820, qword_1A828, stderr);
          sub_F6B0(stderr, a7);
        }
      }
      return;
    }
    if ( dword_1A008 == dword_1A00C )
    {
      if ( dword_1A00C != (int)&unk_40000 || (unsigned int)sub_D6C0(1) == -1 )
        goto LABEL_13;
      dword_1A008 = 0;
    }
    dword_18020 = sub_5750(dword_1A014);
    if ( dword_18020 < 0 )
      return;
    qword_1A820 = 0;
  }
}



// Function: atdir_set @ 0x6d00
long long atdir_set(void *s2, size_t n)
{
  size_t v3; // rbx
  size_t v4; // r13
  int v5; // r8d
  long long result; // rax
  int v7; // edx
  int v8; // ecx
  int v9; // r8d
  int v10; // r9d

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
  v5 = memcmp(byte_DC100, s2, v4);
  result = (unsigned int)fildes;
  if ( v5 || byte_DC100[v3] )
  {
    if ( fildes >= 0 )
      close(fildes);
    __memcpy_chk(byte_DC100, s2, v4, 1024);
    byte_DC100[v3] = 0;
    result = sub_E360((unsigned int)byte_DC100, 0x10000, v7, v8, v9, v10);
    fildes = result;
  }
  return result;
}



// Function: abort_gzip @ 0x8440
void abort_gzip()
{
  sub_5530(0);
  sub_6960(1);
}



// Function: inflate_codes @ 0x8460
long long inflate_codes(long long a1, long long a2, unsigned int a3, unsigned int a4)
{
  unsigned long long v4; // r14
  unsigned int v5; // r13d
  unsigned int v6; // r12d
  unsigned char *v7; // rbx
  unsigned int v8; // r8d
  long long v9; // rax
  long long v10; // rax
  char v11; // cl
  long long v12; // r15
  long long v13; // rax
  char v14; // cl
  unsigned int v15; // r15d
  long long v16; // rax
  char v17; // cl
  unsigned int v18; // r15d
  unsigned long long v19; // rbx
  long long v20; // rax
  char v21; // cl
  unsigned char *v22; // r13
  unsigned int v23; // r8d
  unsigned int v24; // r14d
  long long v25; // rax
  char v26; // cl
  unsigned char v27; // cl
  unsigned int v28; // edx
  unsigned long long v29; // rsi
  unsigned long long v30; // r15
  unsigned int v31; // r13d
  unsigned int v32; // ebx
  long long v33; // rax
  char v34; // cl
  unsigned char v35; // al
  char v36; // cl
  int v37; // eax
  unsigned int v38; // r9d
  unsigned int v39; // r13d
  char *v40; // rdi
  char *v41; // rsi
  unsigned int v42; // ebx
  unsigned int v43; // r15d
  unsigned int v44; // eax
  unsigned int v45; // eax
  long long v46; // rcx
  long long v47; // rdx
  int v51; // [rsp+18h] [rbp-60h]
  unsigned short v52; // [rsp+1Ch] [rbp-5Ch]
  unsigned int v53; // [rsp+1Ch] [rbp-5Ch]
  unsigned short i; // [rsp+20h] [rbp-58h]
  unsigned long long v55; // [rsp+20h] [rbp-58h]
  int v57; // [rsp+30h] [rbp-48h]
  short v59; // [rsp+38h] [rbp-40h]
  unsigned char *v60; // [rsp+38h] [rbp-40h]

  v4 = qword_DCA10;
  v5 = dword_DCA0C;
  v6 = n;
  v51 = word_13380[a3];
  v57 = word_13380[a4];
LABEL_2:
  while ( v5 < a3 )
  {
LABEL_7:
    if ( dword_1A008 >= (unsigned int)dword_1A00C )
    {
      LODWORD(n) = v6;
      v10 = (unsigned char)sub_D6C0(0);
    }
    else
    {
      v10 = (unsigned char)byte_9C000[dword_1A008++];
    }
    v11 = v5;
    v5 += 8;
    v4 |= v10 << v11;
  }
  while ( 1 )
  {
    v7 = (unsigned char *)(a1 + 16LL * ((unsigned int)v4 & v51));
    v8 = *v7;
    if ( v8 > 0x10 )
    {
      while ( v8 != 99 )
      {
        v12 = v8 - 16;
        v5 -= v7[1];
        for ( v4 >>= v7[1]; (unsigned int)v12 > v5; v4 |= v13 << v14 )
        {
          if ( dword_1A008 < (unsigned int)dword_1A00C )
          {
            v13 = (unsigned char)byte_9C000[dword_1A008++];
          }
          else
          {
            LODWORD(n) = v6;
            v13 = (unsigned char)sub_D6C0(0);
          }
          v14 = v5;
          v5 += 8;
        }
        v8 = *(unsigned char *)(*((long long *)v7 + 1) + 16 * (v4 & word_13380[v12]));
        v7 = (unsigned char *)(*((long long *)v7 + 1) + 16 * (v4 & word_13380[v12]));
        if ( v8 <= 0x10 )
          goto LABEL_4;
      }
      return 1;
    }
LABEL_4:
    v5 -= v7[1];
    v4 >>= v7[1];
    if ( v8 != 16 )
      break;
    v9 = v6++;
    byte_3B000[v9] = *((short *)v7 + 4);
    if ( v6 != 0x8000 )
      goto LABEL_2;
    LODWORD(n) = 0x8000;
    v6 = 0;
    sub_D640();
    if ( v5 < a3 )
      goto LABEL_7;
  }
  if ( v8 != 15 )
  {
    if ( v8 > v5 )
    {
      v15 = v8;
      do
      {
        if ( dword_1A008 < (unsigned int)dword_1A00C )
        {
          v16 = (unsigned char)byte_9C000[dword_1A008++];
        }
        else
        {
          LODWORD(n) = v6;
          v16 = (unsigned char)sub_D6C0(0);
        }
        v17 = v5;
        v5 += 8;
        v4 |= v16 << v17;
      }
      while ( v15 > v5 );
      v8 = v15;
    }
    v18 = v5 - v8;
    v52 = *((short *)v7 + 4);
    v19 = v4 >> v8;
    for ( i = word_13380[v8]; a4 > v18; v19 |= v20 << v21 )
    {
      if ( dword_1A008 < (unsigned int)dword_1A00C )
      {
        v20 = (unsigned char)byte_9C000[dword_1A008++];
      }
      else
      {
        LODWORD(n) = v6;
        v20 = (unsigned char)sub_D6C0(0);
      }
      v21 = v18;
      v18 += 8;
    }
    v22 = (unsigned char *)(a2 + 16LL * ((unsigned int)v19 & v57));
    v23 = *v22;
    if ( v23 > 0x10 )
    {
      v59 = v4;
      do
      {
        if ( v23 == 99 )
          return 1;
        v24 = v23 - 16;
        v18 -= v22[1];
        v19 >>= v22[1];
        if ( v18 < v23 - 16 )
        {
          do
          {
            if ( dword_1A008 < (unsigned int)dword_1A00C )
            {
              v25 = (unsigned char)byte_9C000[dword_1A008++];
            }
            else
            {
              LODWORD(n) = v6;
              v25 = (unsigned char)sub_D6C0(0);
            }
            v26 = v18;
            v18 += 8;
            v19 |= v25 << v26;
          }
          while ( v24 > v18 );
        }
        v23 = *(unsigned char *)(*((long long *)v22 + 1) + 16 * (v19 & word_13380[v24]));
        v22 = (unsigned char *)(*((long long *)v22 + 1) + 16 * (v19 & word_13380[v24]));
      }
      while ( v23 > 0x10 );
      LOWORD(v4) = v59;
    }
    v27 = v22[1];
    v28 = v18 - v27;
    v29 = v19 >> v27;
    if ( v23 > v28 )
    {
      v60 = v22;
      v30 = v19 >> v27;
      v31 = v23;
      v32 = v28;
      do
      {
        while ( dword_1A008 < (unsigned int)dword_1A00C )
        {
          v33 = (unsigned char)byte_9C000[dword_1A008];
          v34 = v32;
          v32 += 8;
          ++dword_1A008;
          v30 |= v33 << v34;
          if ( v31 <= v32 )
            goto LABEL_48;
        }
        LODWORD(n) = v6;
        v35 = sub_D6C0(0);
        v36 = v32;
        v32 += 8;
        v30 |= (unsigned long long)v35 << v36;
      }
      while ( v31 > v32 );
LABEL_48:
      v23 = v31;
      v22 = v60;
      v29 = v30;
      v28 = v32;
    }
    v37 = v52;
    v53 = v28 - v23;
    v38 = (v4 & i) + v37;
    LOWORD(v37) = *((short *)v22 + 4);
    v39 = v38;
    v55 = v29 >> v23;
    LOWORD(v4) = v6 - v37 - (v29 & word_13380[v23]);
    while ( 1 )
    {
      v42 = v4 & 0x7FFF;
      v43 = 0x8000 - v6;
      if ( v6 < v42 )
        v43 = 0x8000 - v42;
      if ( v39 <= v43 )
        v43 = v39;
      v44 = v42 - v6;
      v39 -= v43;
      if ( v6 > v42 )
        v44 = v6 - v42;
      if ( v44 >= v43 )
      {
        v40 = &byte_3B000[v6];
        v41 = &byte_3B000[v4 & 0x7FFF];
        LOWORD(v4) = v42 + v43;
        v6 += v43;
        memcpy(v40, v41, v43);
        if ( v6 != 0x8000 )
          goto LABEL_51;
      }
      else
      {
        LODWORD(v4) = v42 + v43;
        v45 = v42;
        do
        {
          v46 = v45;
          v47 = v45 + v6 - v42;
          ++v45;
          byte_3B000[v47] = byte_3B000[v46];
        }
        while ( v45 != (int)v4 );
        v6 += v43;
        if ( v6 != 0x8000 )
        {
LABEL_51:
          if ( !v39 )
            goto LABEL_63;
          continue;
        }
      }
      LODWORD(n) = 0x8000;
      v6 = 0;
      sub_D640();
      if ( !v39 )
      {
LABEL_63:
        v5 = v53;
        v4 = v55;
        goto LABEL_2;
      }
    }
  }
  LODWORD(n) = v6;
  qword_DCA10 = v4;
  dword_DCA0C = v5;
  return 0;
}



// Function: huft_build @ 0x89b0
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
  unsigned int v36; // r13d
  int v37; // r12d
  long long *v38; // rsi
  char *v39; // rax
  int v40; // r10d
  unsigned int v41; // edx
  unsigned int v42; // r10d
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
  char *v61; // r8
  long long v62; // rax
  long long v63; // rdx
  char *v64; // rdx
  unsigned int i; // edx
  int v66; // ecx
  long long v67; // rdx
  int v68; // r9d
  unsigned int v69; // edx
  char *v70; // rax
  short *v71; // rax
  long long v72; // rbx
  void *v73; // rdi
  unsigned int v74; // [rsp+4h] [rbp-65Ch]
  long long v75; // [rsp+10h] [rbp-650h]
  char v76; // [rsp+18h] [rbp-648h]
  unsigned int v77; // [rsp+1Ch] [rbp-644h]
  int v78; // [rsp+20h] [rbp-640h]
  int v79; // [rsp+24h] [rbp-63Ch]
  char v80; // [rsp+28h] [rbp-638h]
  int v81; // [rsp+2Ch] [rbp-634h]
  char *v82; // [rsp+30h] [rbp-630h]
  unsigned int *v83; // [rsp+38h] [rbp-628h]
  unsigned int *v84; // [rsp+40h] [rbp-620h]
  unsigned int v85; // [rsp+48h] [rbp-618h]
  signed int v86; // [rsp+4Ch] [rbp-614h]
  unsigned int *v87; // [rsp+50h] [rbp-610h]
  int v91; // [rsp+70h] [rbp-5F0h]
  _OWORD v92[4]; // [rsp+78h] [rbp-5E8h] BYREF
  int v93; // [rsp+B8h] [rbp-5A8h] BYREF
  int v94; // [rsp+C8h] [rbp-598h] BYREF
  int v95; // [rsp+CCh] [rbp-594h]
  char v96[72]; // [rsp+D0h] [rbp-590h] BYREF
  long long v97[16]; // [rsp+118h] [rbp-548h]
  int v98[290]; // [rsp+198h] [rbp-4C8h] BYREF
  unsigned long long v99; // [rsp+620h] [rbp-40h]

  v8 = a2;
  v11 = &a1[a2];
  v99 = __readfsqword(0x28u);
  memset(v92, 0, sizeof(v92));
  v12 = a1;
  v93 = 0;
  do
  {
    v13 = *v12++;
    ++*((int *)v92 + v13);
  }
  while ( v11 != v12 );
  v14 = v92[0];
  if ( LODWORD(v92[0]) != a2 )
  {
    v15 = *a7;
    v16 = 1;
    while ( 1 )
    {
      v17 = v16;
      if ( *((int *)v92 + v16) )
        break;
      if ( ++v16 == 17 )
      {
        v78 = 17;
        v16 = (long long)&unk_20000;
        v17 = 17;
LABEL_8:
        if ( v15 < v17 )
          v15 = v78;
        v18 = &v93;
        v19 = 16;
        while ( !*v18 )
        {
          --v18;
          if ( !--v19 )
          {
            if ( !v15 )
            {
              v20 = (int)v16 - LODWORD(v92[0]) < 0;
              v21 = v16 - LODWORD(v92[0]);
              *a7 = 0;
              v91 = v21;
              if ( !v20 )
              {
                LODWORD(v92[0]) = v16;
                v95 = 0;
                v86 = 0;
LABEL_28:
                v26 = v96;
                v27 = 0;
                v28 = (char *)v92 + 4 * v19;
                v29 = (char *)v92 + 4;
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
                    v32 = (unsigned int)*(&v94 + v31);
                    v98[v32] = v30;
                    *(&v94 + v31) = v32 + 1;
                  }
                  ++v30;
                }
                while ( v8 != v30 );
                v97[0] = 0;
                v33 = (unsigned int)*(&v94 + v86);
                v94 = 0;
                if ( v86 < v78 )
                  return (v91 != 0) & (unsigned char)(v86 != 1);
                v34 = v7;
                v35 = 0;
                v84 = v98;
                v36 = 0;
                v37 = -v15;
                v87 = &v98[v33];
                v38 = a6;
                v79 = -1;
                v83 = (unsigned int *)v92 + v78;
                v39 = 0;
                while ( 1 )
                {
                  v77 = *v83;
                  if ( *v83 )
                    break;
LABEL_70:
                  ++v78;
                  ++v83;
                  if ( v78 == v86 + 1 )
                    return (v91 != 0) & (unsigned char)(v86 != 1);
                }
                v40 = v37;
                while ( 1 )
                {
                  v41 = v15 + v40;
                  if ( (int)(v15 + v40) < v78 )
                    break;
                  v80 = v78 - v40;
LABEL_54:
                  v57 = 99;
                  if ( v87 > v84 )
                  {
                    v58 = *v84++;
                    if ( v58 >= a3 )
                    {
                      v69 = v58 - a3;
                      v59 = v82;
                      LOWORD(v59) = *(short *)(a4 + 2LL * v69);
                      v57 = *(char *)(a5 + 2LL * v69);
                    }
                    else
                    {
                      v59 = v82;
                      LOWORD(v59) = v58;
                      v57 = (v58 != 256) + 15;
                    }
                    v82 = v59;
                  }
                  v60 = v36 >> v40;
                  if ( v36 >> v40 < v35 )
                  {
                    v61 = v39;
                    v62 = v34;
                    do
                    {
                      v63 = v60;
                      LOBYTE(v62) = v57;
                      v60 += 1 << v80;
                      BYTE1(v62) = v78 - v40;
                      v64 = &v61[16 * v63];
                      *(long long *)v64 = v62;
                      *((long long *)v64 + 1) = v82;
                    }
                    while ( v60 < v35 );
                    v34 = v62;
                    v39 = v61;
                  }
                  for ( i = 1 << (v78 - 1); (i & v36) != 0; i >>= 1 )
                    v36 ^= i;
                  v36 ^= i;
                  if ( *(&v94 + v79) != (v36 & ((1 << v40) - 1)) )
                  {
                    v66 = v40;
                    v67 = v79 - 1;
                    do
                    {
                      v66 -= v15;
                      v68 = v67--;
                    }
                    while ( (v36 & ((1 << v66) - 1)) != *(&v94 + v67 + 1) );
                    v79 = v68;
                    v40 = v66;
                  }
                  if ( !--v77 )
                  {
                    v37 = v40;
                    goto LABEL_70;
                  }
                }
                v85 = v36;
                v42 = v15 + v40;
                v43 = v86 - v41;
                v44 = v78 - v41;
                v45 = v79 + 1;
                v46 = v34;
                v47 = v43;
                while ( 1 )
                {
                  v81 = v42;
                  v79 = v45;
                  v35 = 1 << v44;
                  LOBYTE(v51) = v44;
                  v76 = v42 - v15;
                  v80 = v44;
                  if ( 1 << v44 > v77 )
                  {
                    v52 = v15;
                    if ( v47 <= v15 )
                      v52 = v47;
                    if ( v52 > v44 )
                    {
                      v51 = v44 + 1;
                      v53 = v83;
                      for ( j = v35 - v77; v51 < v52; j = v56 - v55 )
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
                  v75 = v46;
                  v74 = v42;
                  v48 = (char *)malloc(16LL * (v35 + 1));
                  v46 = v75;
                  v49 = v48;
                  if ( !v48 )
                    break;
                  dword_DCA08 += v35 + 1;
                  v39 = v48 + 16;
                  *v38 = v49 + 16;
                  v38 = v49 + 8;
                  *((long long *)v49 + 1) = 0;
                  v97[v45] = v49 + 16;
                  if ( (int)v45 )
                  {
                    v82 = v49 + 16;
                    LOBYTE(v46) = v51 + 16;
                    *(&v94 + v45) = v85;
                    v50 = (long long *)(16LL * (v85 >> v76) + *(long long *)&v96[8 * v45 + 64]);
                    BYTE1(v46) = v15;
                    v50[1] = v39;
                    *v50 = v46;
                  }
                  ++v45;
                  v42 = v15 + v74;
                  v44 -= v15;
                  v47 -= v15;
                  if ( (int)(v15 + v74) >= v78 )
                  {
                    v36 = v85;
                    v40 = v81;
                    v34 = v46;
                    goto LABEL_54;
                  }
                }
                if ( (int)v45 )
                {
                  v72 = v97[0];
                  while ( v72 )
                  {
                    v73 = (void *)(v72 - 16);
                    v72 = *(long long *)(v72 - 8);
                    free(v73);
                  }
                }
                return 3;
              }
              return 2;
            }
            *a7 = 0;
            v15 = 0;
            v86 = 0;
            goto LABEL_26;
          }
        }
        v86 = v19;
        if ( v15 > v19 )
          v15 = v19;
        *a7 = v15;
        if ( v17 >= v19 )
        {
          v91 = v16 - *((int *)v92 + v19);
          if ( v91 >= 0 )
          {
            *((int *)v92 + v19) = v16;
            v95 = 0;
            if ( v19 != 1 )
              goto LABEL_28;
            goto LABEL_30;
          }
          return 2;
        }
        v23 = v16 - *((int *)v92 + v17);
        if ( v23 < 0 )
          return 2;
        v24 = (char *)v92 + 4 * v17;
        v25 = (char *)v92 + 4 * v17 + 4 * (unsigned long long)(v19 - v17 - 1);
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
        v14 = *((int *)v92 + v19);
LABEL_26:
        v91 = v16 - v14;
        if ( (int)v16 - v14 >= 0 )
        {
          v95 = 0;
          *((int *)v92 + v19) = v16;
          goto LABEL_28;
        }
        return 2;
      }
    }
    v78 = v16;
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
  dword_DCA08 += 3;
  *a6 = v71;
  *a7 = 1;
  return 0;
}



// Function: inflate_fixed @ 0x90c0
long long inflate_fixed()
{
  unsigned int *v0; // rbx
  unsigned int *v1; // rax
  long long *v2; // rax
  int v3; // r12d
  long long v5; // rbx
  void *v6; // rdi
  long long v7; // rbx
  void *v8; // rdi
  long long v9; // rbx
  void *v10; // rdi
  unsigned int v11; // [rsp+8h] [rbp-4C0h] BYREF
  unsigned int v12; // [rsp+Ch] [rbp-4BCh] BYREF
  long long v13; // [rsp+10h] [rbp-4B8h] BYREF
  long long v14; // [rsp+18h] [rbp-4B0h] BYREF
  unsigned int v15[30]; // [rsp+20h] [rbp-4A8h] BYREF
  long long v16; // [rsp+98h] [rbp-430h] BYREF
  char v17; // [rsp+260h] [rbp-268h] BYREF
  char v18[96]; // [rsp+420h] [rbp-A8h] BYREF
  long long v19[9]; // [rsp+480h] [rbp-48h] BYREF

  v19[5] = __readfsqword(0x28u);
  v0 = v15;
  v1 = v15;
  do
    *v1++ = 8;
  while ( v1 != (unsigned int *)&v17 );
  do
    *v1++ = 9;
  while ( v1 != (unsigned int *)v18 );
  v2 = v18;
  do
  {
    *(int *)v2 = 7;
    v2 = (long long *)((char *)v2 + 4);
  }
  while ( v19 != v2 );
  v11 = 7;
  v19[0] = 0x800000008LL;
  v19[1] = 0x800000008LL;
  v19[2] = 0x800000008LL;
  v19[3] = 0x800000008LL;
  v3 = sub_89B0(v15, 0x120u, 0x101u, (long long)&unk_181A0, (long long)&unk_18160, &v13, &v11);
  if ( !v3 )
  {
    do
      *v0++ = 5;
    while ( v0 != (unsigned int *)&v16 );
    v12 = 5;
    v3 = sub_89B0(v15, 0x1Eu, 0, (long long)&unk_18120, (long long)&unk_180E0, &v14, &v12);
    if ( v3 > 1 )
    {
      v5 = v13;
      while ( v5 )
      {
        v6 = (void *)(v5 - 16);
        v5 = *(long long *)(v5 - 8);
        free(v6);
      }
    }
    else
    {
      v3 = sub_8460(v13, v14, v11, v12);
      if ( v3 )
      {
        return 1;
      }
      else
      {
        v7 = v13;
        while ( v7 )
        {
          v8 = (void *)(v7 - 16);
          v7 = *(long long *)(v7 - 8);
          free(v8);
        }
        v9 = v14;
        while ( v9 )
        {
          v10 = (void *)(v9 - 16);
          v9 = *(long long *)(v9 - 8);
          free(v10);
        }
      }
    }
  }
  return (unsigned int)v3;
}



// Function: inflate_dynamic @ 0x92b0
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
  unsigned int v29; // eax
  unsigned int v30; // r12d
  unsigned int v31; // ebx
  unsigned int v32; // eax
  unsigned long long v33; // r14
  unsigned int v34; // r13d
  long long v35; // rax
  char v36; // cl
  long long v38; // rax
  int v39; // ecx
  unsigned int v40; // eax
  long long v41; // rcx
  long long v42; // rbx
  unsigned int v43; // eax
  long long v44; // r13
  int v45; // r14d
  void *v46; // rdi
  unsigned int v47; // eax
  long long v48; // rbx
  void *v49; // rdi
  long long v50; // rax
  char v51; // cl
  char v52; // si
  unsigned int v53; // esi
  unsigned int *v54; // rax
  long long v55; // rax
  char v56; // cl
  char v57; // si
  unsigned int v58; // esi
  unsigned int *v59; // rax
  long long v60; // rax
  char v61; // cl
  char v62; // si
  unsigned int v63; // esi
  unsigned int *v64; // rax
  long long v65; // rbx
  void *v66; // rdi
  long long v67; // rbx
  void *v68; // rdi
  long long v69; // rbx
  void *v70; // rdi
  int v71; // eax
  long long v72; // rbx
  void *v73; // rdi
  long long v74; // rbx
  void *v75; // rdi
  long long v76; // rbx
  void *v77; // rdi
  unsigned int v78; // [rsp+0h] [rbp-588h]
  unsigned int v79; // [rsp+0h] [rbp-588h]
  unsigned int v80; // [rsp+4h] [rbp-584h]
  int v81; // [rsp+8h] [rbp-580h]
  unsigned int v82; // [rsp+Ch] [rbp-57Ch]
  unsigned int v83; // [rsp+18h] [rbp-570h]
  unsigned int v84; // [rsp+1Ch] [rbp-56Ch]
  unsigned int v85; // [rsp+38h] [rbp-550h] BYREF
  unsigned int v86; // [rsp+3Ch] [rbp-54Ch] BYREF
  long long v87; // [rsp+40h] [rbp-548h] BYREF
  long long v88; // [rsp+48h] [rbp-540h] BYREF
  unsigned int v89[318]; // [rsp+50h] [rbp-538h] BYREF
  unsigned long long v90; // [rsp+548h] [rbp-40h]

  v0 = dword_DCA0C;
  v1 = qword_DCA10;
  v90 = __readfsqword(0x28u);
  v2 = n;
  if ( (unsigned int)dword_DCA0C <= 4 )
  {
    v3 = (unsigned int)dword_1A008;
    if ( dword_1A008 >= (unsigned int)dword_1A00C )
    {
      v4 = (unsigned char)sub_D6C0(0);
    }
    else
    {
      ++dword_1A008;
      v4 = (unsigned char)byte_9C000[v3];
    }
    v5 = v0;
    v0 += 8;
    v1 |= v4 << v5;
  }
  v6 = v1;
  v7 = v0 - 5;
  v8 = v1 >> 5;
  v83 = (v6 & 0x1F) + 257;
  if ( (unsigned int)(v0 - 5) <= 4 )
  {
    v9 = (unsigned int)dword_1A008;
    if ( dword_1A008 >= (unsigned int)dword_1A00C )
    {
      LODWORD(n) = v2;
      v10 = (unsigned char)sub_D6C0(0);
    }
    else
    {
      ++dword_1A008;
      v10 = (unsigned char)byte_9C000[v9];
    }
    v7 = v0 + 3;
    v8 |= v10 << ((unsigned char)v0 - 5);
  }
  v11 = v8;
  v12 = v7 - 5;
  v13 = v8 >> 5;
  v14 = v13;
  v84 = (v11 & 0x1F) + 1;
  if ( (unsigned int)(v7 - 5) <= 3 )
  {
    v15 = (unsigned int)dword_1A008;
    if ( dword_1A008 >= (unsigned int)dword_1A00C )
    {
      LODWORD(n) = v2;
      v16 = (unsigned char)sub_D6C0(0);
    }
    else
    {
      ++dword_1A008;
      v16 = (unsigned char)byte_9C000[v15];
    }
    v12 = v7 + 3;
    v14 = (v16 << ((unsigned char)v7 - 5)) | v13;
  }
  if ( v83 > 0x11E || v84 > 0x1E )
    return 1;
  v17 = v14;
  v18 = v12 - 4;
  v19 = v14 >> 4;
  v20 = (char *)dword_133C0;
  v21 = v17 & 0xF;
  v78 = v21 + 4;
  v22 = (char *)&dword_133C0[v21 + 4];
  do
  {
    if ( v18 <= 2 )
    {
      if ( dword_1A008 < (unsigned int)dword_1A00C )
      {
        v23 = (unsigned char)byte_9C000[dword_1A008++];
      }
      else
      {
        LODWORD(n) = v2;
        v23 = (unsigned char)sub_D6C0(0);
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
    v89[v25] = v26 & 7;
  }
  while ( v22 != v20 );
  for ( i = v78; i != 19; v89[dword_133C0[v28]] = 0 )
    v28 = i++;
  v85 = 7;
  v29 = sub_89B0(v89, 0x13u, 0x13u, 0, 0, &v87, &v85);
  v30 = v29;
  if ( !v29 )
  {
    if ( !v87 )
      return 2;
    v82 = 0;
    v79 = 0;
    v80 = v84 + v83;
    v31 = v85;
    v81 = word_13380[v85];
    v32 = v18;
    v33 = v19;
    v34 = v32;
    do
    {
      while ( v31 > v34 )
      {
        if ( dword_1A008 < (unsigned int)dword_1A00C )
        {
          v35 = (unsigned char)byte_9C000[dword_1A008++];
        }
        else
        {
          LODWORD(n) = v2;
          v35 = (unsigned char)sub_D6C0(0);
        }
        v36 = v34;
        v34 += 8;
        v33 |= v35 << v36;
      }
      v38 = v87 + 16LL * ((unsigned int)v33 & v81);
      v88 = v38;
      v39 = *(unsigned char *)(v38 + 1);
      v33 >>= v39;
      v34 -= v39;
      if ( *(char *)v38 == 99 )
      {
        if ( v87 )
        {
          v65 = v87;
          do
          {
            v66 = (void *)(v65 - 16);
            v65 = *(long long *)(v65 - 8);
            free(v66);
          }
          while ( v65 );
        }
        return 2;
      }
      v40 = *(unsigned short *)(v38 + 8);
      if ( v40 > 0xF )
      {
        if ( v40 == 16 )
        {
          if ( v34 <= 1 )
          {
            if ( dword_1A008 >= (unsigned int)dword_1A00C )
            {
              LODWORD(n) = v2;
              v60 = (unsigned char)sub_D6C0(0);
            }
            else
            {
              v60 = (unsigned char)byte_9C000[dword_1A008++];
            }
            v61 = v34;
            v34 += 8;
            v33 |= v60 << v61;
          }
          v62 = v33;
          v33 >>= 2;
          v34 -= 2;
          v63 = v62 & 3;
          if ( v63 + v79 + 3 > v80 )
            return 1;
          v64 = &v89[v30];
          do
            *v64++ = v82;
          while ( &v89[(int)v30 + 3 + (unsigned long long)v63] != v64 );
          v30 += v63 + 3;
        }
        else if ( v40 == 17 )
        {
          if ( v34 <= 2 )
          {
            if ( dword_1A008 >= (unsigned int)dword_1A00C )
            {
              LODWORD(n) = v2;
              v55 = (unsigned char)sub_D6C0(0);
            }
            else
            {
              v55 = (unsigned char)byte_9C000[dword_1A008++];
            }
            v56 = v34;
            v34 += 8;
            v33 |= v55 << v56;
          }
          v57 = v33;
          v33 >>= 3;
          v34 -= 3;
          v58 = v57 & 7;
          if ( v58 + v79 + 3 > v80 )
            return 1;
          v59 = &v89[v30];
          do
            *v59++ = 0;
          while ( &v89[(int)v30 + 3 + (unsigned long long)v58] != v59 );
          v82 = 0;
          v30 += v58 + 3;
        }
        else
        {
          if ( v34 <= 6 )
          {
            if ( dword_1A008 >= (unsigned int)dword_1A00C )
            {
              LODWORD(n) = v2;
              v50 = (unsigned char)sub_D6C0(0);
            }
            else
            {
              v50 = (unsigned char)byte_9C000[dword_1A008++];
            }
            v51 = v34;
            v34 += 8;
            v33 |= v50 << v51;
          }
          v52 = v33;
          v33 >>= 7;
          v34 -= 7;
          v53 = v52 & 0x7F;
          if ( v53 + v79 + 11 > v80 )
            return 1;
          v54 = &v89[v30];
          do
            *v54++ = 0;
          while ( &v89[(int)v30 + 11 + (unsigned long long)v53] != v54 );
          v82 = 0;
          v30 += v53 + 11;
        }
      }
      else
      {
        v41 = (int)v30;
        v82 = v40;
        ++v30;
        v89[v41] = v40;
      }
      v79 = v30;
    }
    while ( v30 < v80 );
    v42 = v87;
    v43 = v34;
    v44 = v33;
    v45 = v43;
    while ( v42 )
    {
      v46 = (void *)(v42 - 16);
      v42 = *(long long *)(v42 - 8);
      free(v46);
    }
    v85 = 9;
    qword_DCA10 = v44;
    dword_DCA0C = v45;
    v47 = sub_89B0(v89, v83, 0x101u, (long long)&unk_181A0, (long long)&unk_18160, &v87, &v85);
    v30 = v47;
    if ( !v47 )
    {
      v86 = 6;
      v30 = sub_89B0(&v89[v83], v84, 0, (long long)&unk_18120, (long long)&unk_180E0, &v88, &v86);
      if ( v30 )
      {
        if ( v30 == 1 )
        {
          v76 = v88;
          while ( v76 )
          {
            v77 = (void *)(v76 - 16);
            v76 = *(long long *)(v76 - 8);
            free(v77);
          }
        }
        v67 = v87;
        while ( v67 )
        {
          v68 = (void *)(v67 - 16);
          v67 = *(long long *)(v67 - 8);
          free(v68);
        }
      }
      else
      {
        v71 = sub_8460(v87, v88, v85, v86);
        v72 = v87;
        v30 = v71 != 0;
        while ( v72 )
        {
          v73 = (void *)(v72 - 16);
          v72 = *(long long *)(v72 - 8);
          free(v73);
        }
        v74 = v88;
        while ( v74 )
        {
          v75 = (void *)(v74 - 16);
          v74 = *(long long *)(v74 - 8);
          free(v75);
        }
      }
      return v30;
    }
    if ( v47 != 1 )
      return v30;
    v48 = v87;
    while ( v48 )
    {
      v49 = (void *)(v48 - 16);
      v48 = *(long long *)(v48 - 8);
      free(v49);
    }
    return 1;
  }
  if ( v29 == 1 )
  {
    v69 = v87;
    while ( v69 )
    {
      v70 = (void *)(v69 - 16);
      v69 = *(long long *)(v69 - 8);
      free(v70);
    }
    return 1;
  }
  return v30;
}



// Function: inflate @ 0x9a50
long long inflate()
{
  unsigned int v0; // r14d
  unsigned long long v1; // rbx
  long long v2; // rax
  unsigned long long v3; // rax
  int v4; // r15d
  int v5; // r12d
  unsigned long long v6; // rbx
  char v7; // al
  unsigned long long v8; // rbx
  int v9; // eax
  long long result; // rax
  char v11; // r12
  long long v12; // rax
  unsigned int v13; // r13d
  unsigned long long v14; // r14
  int v15; // ebx
  unsigned int v16; // ecx
  long long v17; // rax
  long long v18; // rax
  unsigned char v19; // al
  int v20; // eax
  unsigned long long v21; // r15
  unsigned int v22; // r14d
  long long v23; // rax
  char v24; // cl
  unsigned long long v25; // r15
  int v26; // ebx
  long long v27; // rax
  long long v28; // rax
  bool v29; // cf
  unsigned int v30; // [rsp+8h] [rbp-40h]
  int v31; // [rsp+8h] [rbp-40h]
  int v32; // [rsp+Ch] [rbp-3Ch]

  v0 = 0;
  v1 = 0;
  LODWORD(n) = 0;
  dword_DCA0C = 0;
  for ( qword_DCA10 = 0; ; v1 = qword_DCA10 )
  {
    dword_DCA08 = 0;
    if ( v0 )
    {
      v11 = v1;
      v6 = v1 >> 1;
      v5 = v11 & 1;
      if ( v0 - 1 > 1 )
      {
        v4 = v0 - 3;
      }
      else
      {
        if ( dword_1A008 >= (unsigned int)dword_1A00C )
          v12 = (unsigned char)sub_D6C0(0);
        else
          v12 = (unsigned char)byte_9C000[dword_1A008++];
        v4 = v0 + 5;
        v6 |= v12 << ((unsigned char)v0 - 1);
      }
    }
    else
    {
      if ( dword_1A008 >= (unsigned int)dword_1A00C )
        v2 = (unsigned char)sub_D6C0(0);
      else
        v2 = (unsigned char)byte_9C000[dword_1A008++];
      v3 = v1 | v2;
      v4 = 5;
      v5 = v3 & 1;
      v6 = v3 >> 1;
    }
    v7 = v6;
    v8 = v6 >> 2;
    dword_DCA0C = v4;
    v9 = v7 & 3;
    qword_DCA10 = v8;
    if ( v9 == 2 )
    {
      result = sub_92B0();
      goto LABEL_46;
    }
    if ( !v9 )
      break;
    if ( v9 != 1 )
      return 2;
    result = sub_90C0();
LABEL_46:
    if ( (int)result )
      return result;
    v0 = dword_DCA0C;
    if ( v5 )
      goto LABEL_42;
LABEL_48:
    ;
  }
  v13 = n;
  v32 = v4 & 7;
  v14 = v8 >> (v4 & 7);
  v15 = v4 - v32;
  if ( (unsigned int)(v4 - v32) <= 0xF )
  {
    v16 = v4 - v32;
    do
    {
      while ( dword_1A008 < (unsigned int)dword_1A00C )
      {
        v17 = (unsigned char)byte_9C000[dword_1A008++];
        v18 = v17 << v16;
        v16 += 8;
        v14 |= v18;
        if ( v16 > 0xF )
          goto LABEL_22;
      }
      v30 = v16;
      LODWORD(n) = v13;
      v19 = sub_D6C0(0);
      v16 = v30 + 8;
      v14 |= (unsigned long long)v19 << v30;
    }
    while ( v30 + 8 <= 0xF );
LABEL_22:
    v15 += ((v32 - v4 + 15) & 0xFFFFFFF8) + 8;
  }
  v20 = (unsigned short)v14;
  v21 = v14 >> 16;
  v22 = v15 - 16;
  v31 = v20;
  if ( (unsigned int)(v15 - 16) <= 0xF )
  {
    do
    {
      if ( dword_1A008 < (unsigned int)dword_1A00C )
      {
        v23 = (unsigned char)byte_9C000[dword_1A008++];
      }
      else
      {
        LODWORD(n) = v13;
        v23 = (unsigned char)sub_D6C0(0);
      }
      v24 = v22;
      v22 += 8;
      v21 |= v23 << v24;
    }
    while ( v22 <= 0xF );
    v22 = v15 + ((31 - v15) & 0xFFFFFFF8) - 8;
  }
  if ( v31 != (unsigned short)~(short)v21 )
    return 1;
  v25 = v21 >> 16;
  v0 = v22 - 16;
  v26 = v31 - 1;
  if ( v31 )
  {
    do
    {
      while ( 1 )
      {
        if ( v0 > 7 )
        {
          v0 -= 8;
        }
        else
        {
          if ( dword_1A008 < (unsigned int)dword_1A00C )
          {
            v27 = (unsigned char)byte_9C000[dword_1A008++];
          }
          else
          {
            LODWORD(n) = v13;
            v27 = (unsigned char)sub_D6C0(0);
          }
          v25 |= v27 << v0;
        }
        v28 = v13++;
        byte_3B000[v28] = v25;
        if ( v13 == 0x8000 )
          break;
        v25 >>= 8;
        v29 = v26-- == 0;
        if ( v29 )
          goto LABEL_41;
      }
      LODWORD(n) = 0x8000;
      v13 = 0;
      v25 >>= 8;
      sub_D640();
      v29 = v26-- == 0;
    }
    while ( !v29 );
  }
LABEL_41:
  LODWORD(n) = v13;
  qword_DCA10 = v25;
  dword_DCA0C = v0;
  if ( !v5 )
    goto LABEL_48;
LABEL_42:
  if ( v0 > 7 )
  {
    dword_1A008 += ~((v0 - 8) >> 3);
    dword_DCA0C = v0 & 7;
  }
  sub_D640();
  return 0;
}



// Function: init_block @ 0x9df0
long long init_block()
{
  short *v0; // rax
  short *v1; // rax
  char *v2; // rax

  v0 = &unk_DF080;
  do
  {
    v0 += 2;
    *(v0 - 2) = 0;
  }
  while ( v0 != (short *)&unk_DF4F8 );
  v1 = &unk_DEF80;
  do
  {
    v1 += 2;
    *(v1 - 2) = 0;
  }
  while ( v1 != (short *)&unk_DEFF8 );
  v2 = &unk_DE9E0;
  do
  {
    v2 += 4;
    *((short *)v2 - 2) = 0;
  }
  while ( v2 != algn_DEA2C );
  qword_DCA38 = 0;
  word_DF480 = 1;
  qword_DCA40 = 0;
  dword_DCA4C = 0;
  dword_DCA50 = 0;
  dword_DCA54 = 0;
  byte_DCA49 = 0;
  byte_DCA48 = 1;
  return 1;
}



// Function: pqdownheap @ 0x9ea0
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
  v3 = dword_DE0A4;
  v4 = 2 * a2;
  v5 = *((int *)&unk_DE0C0 + a2);
  v15 = v5;
  if ( 2 * a2 <= dword_DE0A4 )
  {
    v6 = (int)v5;
    v7 = *(short *)(a1 + 4LL * (int)v5);
    do
    {
      v8 = *((int *)&unk_DE0C0 + v4);
      v9 = v4;
      v10 = *(short *)(a1 + 4 * v8);
      v11 = *((int *)&unk_DE0C0 + v4);
      if ( v3 > v4 )
      {
        v9 = v4 + 1;
        v12 = *((int *)&unk_DE0C0 + v4 + 1);
        v13 = *(short *)(a1 + 4 * v12);
        if ( v13 < v10 )
        {
          v10 = *(short *)(a1 + 4 * v12);
          v11 = *((int *)&unk_DE0C0 + v4 + 1);
        }
        else if ( v13 == v10 )
        {
          if ( byte_DDE60[v12] > byte_DDE60[v8] )
            v9 = v4;
          else
            v11 = *((int *)&unk_DE0C0 + v4 + 1);
        }
        else
        {
          v9 = v4;
        }
      }
      if ( v7 < v10 || v7 == v10 && byte_DDE60[v6] <= byte_DDE60[v11] )
        break;
      v4 = 2 * v9;
      *((int *)&unk_DE0C0 + v2) = v11;
      v2 = v9;
    }
    while ( 2 * v9 <= v3 );
  }
  *((int *)&unk_DE0C0 + v2) = v15;
  return v15;
}



// Function: scan_tree @ 0x9f80
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
          word_DE9E0[2 * v9] += result;
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
          ++word_DE9E0[2 * v9];
        ++word_DEA20;
      }
      else if ( (int)result > 10 )
      {
        ++word_DEA28;
      }
      else
      {
        ++word_DEA24;
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



// Function: build_tree @ 0xa090
unsigned long long build_tree(long long a1)
{
  long long v1; // rbx
  long long v3; // rdi
  int v4; // eax
  long long v5; // r15
  char v6; // dl
  long long v7; // rax
  int v8; // esi
  int v9; // ecx
  int v10; // edi
  long long v11; // rdx
  char v12; // r9
  long long v13; // r10
  long long v14; // r8
  short *v15; // r14
  long long v16; // rcx
  long long v17; // rax
  int v18; // r13d
  long long v19; // r13
  long long v20; // rax
  long long v21; // r15
  long long v22; // r14
  long long v23; // rbx
  long long v24; // rax
  short *v25; // r10
  short *v26; // rdi
  short v27; // si
  unsigned char v28; // si
  unsigned char v29; // dl
  char v30; // al
  short v31; // ax
  long long v32; // rsi
  short *v33; // rbx
  int v34; // eax
  int v35; // r9d
  int v36; // r8d
  short *v37; // rdx
  int v38; // r14d
  char v39; // si
  int *v40; // r10
  long long v41; // r12
  long long v42; // r15
  long long v43; // rdx
  unsigned short *v44; // r11
  int v45; // eax
  long long v46; // rsi
  int v47; // ebp
  long long v48; // rax
  int v49; // r12d
  int v50; // r11d
  long long v51; // r10
  short v52; // cx
  char *v53; // rdx
  int v54; // eax
  long long v55; // rsi
  long long v56; // rdx
  int v57; // eax
  long long v58; // rbp
  int v59; // esi
  unsigned int v60; // edi
  long long v62; // r13
  char v63; // r14
  int v64; // edx
  long long v65; // rax
  long long v66; // rcx
  unsigned short *v67; // r12
  long long v68; // r15
  long long v69; // rcx
  short v70; // [rsp+0h] [rbp-98h]
  int v71; // [rsp+0h] [rbp-98h]
  int v72; // [rsp+0h] [rbp-98h]
  int *v73; // [rsp+8h] [rbp-90h]
  short v74; // [rsp+12h] [rbp-86h]
  char v75; // [rsp+12h] [rbp-86h]
  int v76; // [rsp+14h] [rbp-84h]
  int v77; // [rsp+18h] [rbp-80h]
  long long v78; // [rsp+18h] [rbp-80h]
  long long v80; // [rsp+28h] [rbp-70h]
  short v81[20]; // [rsp+30h] [rbp-68h]
  unsigned long long v82; // [rsp+58h] [rbp-40h]

  v1 = *(long long *)a1;
  v82 = __readfsqword(0x28u);
  v3 = *(long long *)(a1 + 8);
  dword_DE0A4 = 0;
  dword_DE0A0 = 573;
  v80 = v3;
  v4 = *(int *)(a1 + 28);
  v70 = v4;
  if ( v4 <= 0 )
  {
    v76 = -1;
    v5 = v4;
    v8 = 0;
  }
  else
  {
    v5 = v4;
    v6 = 0;
    v7 = 0;
    v8 = 0;
    v9 = -1;
    do
    {
      while ( *(short *)(v1 + 4 * v7) )
      {
        ++v8;
        byte_DDE60[v7] = 0;
        v9 = v7;
        dword_DE0C0[v8] = v7++;
        v6 = 1;
        if ( v7 == v5 )
          goto LABEL_6;
      }
      *(short *)(v1 + 4 * v7++ + 2) = 0;
    }
    while ( v7 != v5 );
LABEL_6:
    v76 = v9;
    if ( v6 )
    {
      dword_DE0A4 = v8;
      if ( v8 > 1 )
      {
        v77 = v8;
        goto LABEL_17;
      }
    }
    else
    {
      v8 = 0;
    }
  }
  v10 = v76;
  v11 = v8;
  v12 = 0;
  v13 = qword_DCA40;
  v14 = qword_DCA38;
  do
  {
    if ( v10 > 1 )
    {
      v15 = (short *)v1;
      v16 = 0;
      v17 = 0;
    }
    else
    {
      v17 = ++v10;
      v16 = 4LL * v10;
      v15 = (short *)(v1 + v16);
    }
    dword_DE0C4[v11] = v17;
    *v15 = 1;
    byte_DDE60[v17] = 0;
    if ( v80 )
    {
      v12 = 1;
      v14 -= *(unsigned short *)(v80 + v16 + 2);
    }
    ++v11;
  }
  while ( (int)v11 <= 1 );
  v76 = v10;
  v77 = 2;
  dword_DE0A4 = 2;
  qword_DCA40 = v13 - (unsigned int)(1 - v8) - 1;
  if ( v12 )
    qword_DCA38 = v14;
LABEL_17:
  *(int *)(a1 + 36) = v76;
  v18 = v77 >> 1;
  do
    sub_9EA0(v1, v18--);
  while ( v18 );
  v19 = v77;
  v20 = v5;
  v21 = 571;
  v22 = v1;
  v23 = v20;
  v74 = v77 + v70;
  do
  {
    dword_DE0A4 = v19 - 1;
    v71 = dword_DE0C4[0];
    dword_DE0C4[0] = dword_DE0C0[v19];
    sub_9EA0(v22, 1);
    v24 = dword_DE0C4[0];
    dword_DE0A0 = v21;
    v25 = (short *)(v22 + 4LL * dword_DE0C4[0]);
    dword_DE0C4[v21] = v71;
    v26 = (short *)(v22 + 4LL * v71);
    v27 = *v26 + *v25;
    dword_DE0C0[v21] = v24;
    *(short *)(v22 + 4 * v23) = v27;
    v28 = byte_DDE60[v71];
    v29 = byte_DDE60[v24];
    v30 = v29 + 1;
    if ( v28 >= v29 )
      v30 = v28 + 1;
    v21 -= 2;
    byte_DDE60[v23] = v30;
    v31 = v74 - v19--;
    v25[1] = v31;
    v26[1] = v31;
    dword_DE0C4[0] = v23++;
    sub_9EA0(v22, 1);
  }
  while ( (int)v19 > 1 );
  v32 = dword_DE0C4[0];
  v33 = (short *)v22;
  v34 = -2 * v77 + 575;
  dword_DE0A0 = -2 * v77 + 574;
  dword_DE0C0[dword_DE0A0] = dword_DE0C4[0];
  v35 = *(int *)(a1 + 36);
  v36 = *(int *)(a1 + 32);
  v78 = *(long long *)(a1 + 16);
  v72 = *(int *)(a1 + 24);
  v37 = word_DE9C0;
  do
    *v37++ = 0;
  while ( v37 != word_DE9E0 );
  *(short *)(v22 + 4 * v32 + 2) = 0;
  if ( v34 <= 572 )
  {
    v75 = 0;
    v38 = 0;
    v39 = 0;
    v40 = &dword_DE0C0[v34];
    v41 = qword_DCA40;
    v42 = qword_DCA38;
    v73 = &dword_DE0C4[v34 + (unsigned long long)(unsigned int)(572 - v34)];
    do
    {
      v43 = *v40;
      v44 = &v33[2 * v43];
      v45 = (unsigned short)v33[2 * v44[1] + 1] + 1;
      if ( v36 < v45 )
      {
        ++v38;
        v45 = v36;
      }
      v44[1] = v45;
      if ( v35 >= (int)v43 )
      {
        v46 = v45;
        v47 = 0;
        ++word_DE9C0[v45];
        if ( v72 <= (int)v43 )
        {
          v47 = *(int *)(v78 + 4LL * ((int)v43 - v72));
          v46 = v47 + v45;
        }
        v48 = *v44;
        v41 += v48 * v46;
        v39 = 1;
        if ( v80 )
        {
          v75 = 1;
          v42 += v48 * (v47 + *(unsigned short *)(v80 + 4 * v43 + 2));
        }
      }
      ++v40;
    }
    while ( v73 != v40 );
    if ( v75 )
      qword_DCA38 = v42;
    if ( v39 )
      qword_DCA40 = v41;
    if ( v38 )
    {
      v49 = v38 - 2;
      v50 = v36 - 1;
      v51 = v36;
      v52 = word_DE9C0[v36 - 1];
      if ( v52 )
        goto LABEL_45;
LABEL_41:
      v53 = (char *)&word_DE9C0[v36];
      v54 = v36 - 1;
      do
      {
        v52 = *((short *)v53 - 2);
        v55 = v54;
        v53 -= 2;
        --v54;
      }
      while ( !v52 );
      while ( 1 )
      {
        word_DE9C0[v54] = v52 - 1;
        word_DE9C0[v55] += 2;
        LOWORD(v55) = word_DE9C0[v36] - 1;
        word_DE9C0[v36] = v55;
        if ( v49 == v38 - 2 - ((v38 - 1) & 0xFFFFFFFE) )
          break;
        v52 = word_DE9C0[v50];
        v49 -= 2;
        if ( !v52 )
          goto LABEL_41;
LABEL_45:
        v55 = v36;
        v54 = v36 - 1;
      }
      if ( v36 )
      {
        v62 = qword_DCA40;
        v63 = 0;
        v64 = 573;
        while ( 1 )
        {
          LODWORD(v55) = (unsigned short)v55;
          if ( (short)v55 )
          {
            v65 = v64 - 1;
            do
            {
              while ( 1 )
              {
                v66 = dword_DE0C0[v65];
                v64 = v65;
                if ( v35 >= (int)v66 )
                  break;
                --v65;
              }
              v67 = &v33[2 * v66];
              v68 = v67[1];
              if ( (int)v68 != v36 )
              {
                v69 = *v67;
                v67[1] = v36;
                v63 = 1;
                v62 += (v51 - v68) * v69;
              }
              --v65;
              LODWORD(v55) = v55 - 1;
            }
            while ( (int)v55 );
          }
          v36 = v50;
          --v51;
          if ( !v50 )
            break;
          LOWORD(v55) = word_DE9C0[v51];
          --v50;
        }
        if ( v63 )
          qword_DCA40 = v62;
      }
    }
  }
  v56 = 0;
  v57 = 0;
  do
  {
    LOWORD(v57) = word_DE9C0[v56] + v57;
    v57 *= 2;
    v81[++v56] = v57;
  }
  while ( v56 != 15 );
  if ( v76 != -1 )
  {
    v58 = (long long)&v33[2 * v76 + 2];
    do
    {
      v59 = (unsigned short)v33[1];
      if ( v33[1] )
      {
        v60 = (unsigned short)v81[(unsigned short)v33[1]];
        v81[(unsigned short)v33[1]] = v60 + 1;
        *v33 = sub_44A0(v60, v59);
      }
      v33 += 2;
    }
    while ( (short *)v58 != v33 );
  }
  return v82 - __readfsqword(0x28u);
}



// Function: send_tree @ 0xa6b0
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
            sub_43B0((unsigned short)word_DE9E0[2 * v10], (unsigned short)word_DE9E0[2 * v10 + 1]);
            v9 = v14;
          }
          sub_43B0((unsigned short)word_DEA20, (unsigned short)word_DEA22);
          LOWORD(v2) = sub_43B0(v9 - 3, 2);
        }
        else if ( v9 > 10 )
        {
          sub_43B0((unsigned short)word_DEA28, (unsigned short)word_DEA2A);
          LOWORD(v2) = sub_43B0(v14 - 10, 7);
        }
        else
        {
          sub_43B0((unsigned short)word_DEA24, (unsigned short)word_DEA26);
          LOWORD(v2) = sub_43B0(v14 - 2, 3);
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
          LOWORD(v2) = sub_43B0((unsigned short)word_DE9E0[2 * v10], (unsigned short)word_DE9E0[2 * v10 + 1]);
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



// Function: compress_block @ 0xa870
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

  if ( dword_DCA54 )
  {
    v2 = 0;
    v3 = 0;
    v4 = 0;
    for ( i = 0; ; i = v7 )
    {
      if ( (i & 7) == 0 )
      {
        v6 = v3++;
        v2 = byte_DCA60[v6];
      }
      v7 = i + 1;
      v8 = (unsigned char)byte_9C000[i];
      if ( (v2 & 1) == 0 )
        break;
      v14 = (unsigned char)v8;
      v16 = byte_DDD60[v8];
      sub_43B0(
        *(unsigned short *)(a1 + 4LL * (unsigned int)(v16 + 257)),
        *(unsigned short *)(a1 + 4LL * (unsigned int)(v16 + 257) + 2));
      v9 = dword_182C0[v16];
      if ( v9 )
        sub_43B0(v14 - dword_DDAE0[v16], v9);
      v10 = word_4B000[v4];
      if ( v10 > 0xFF )
        v11 = byte_DDB60[(v10 >> 7) + 256];
      else
        v11 = byte_DDB60[word_4B000[v4]];
      v17 = word_4B000[v4];
      v15 = v11;
      sub_43B0(*(unsigned short *)(a2 + 4 * v11), *(unsigned short *)(a2 + 4 * v11 + 2));
      v4 = (unsigned int)(v4 + 1);
      v12 = dword_18240[v15];
      if ( !v12 )
        goto LABEL_4;
      v2 >>= 1;
      sub_43B0(v17 - dword_DDA60[v15], v12);
      if ( dword_DCA54 <= v7 )
        return sub_43B0(*(unsigned short *)(a1 + 1024), *(unsigned short *)(a1 + 1026));
LABEL_5:
      ;
    }
    sub_43B0(*(unsigned short *)(a1 + 4 * v8), *(unsigned short *)(a1 + 4 * v8 + 2));
LABEL_4:
    v2 >>= 1;
    if ( dword_DCA54 <= v7 )
      return sub_43B0(*(unsigned short *)(a1 + 1024), *(unsigned short *)(a1 + 1026));
    goto LABEL_5;
  }
  return sub_43B0(*(unsigned short *)(a1 + 1024), *(unsigned short *)(a1 + 1026));
}



// Function: ct_init @ 0xaa20
long long ct_init(long long a1)
{
  int v1; // esi
  long long v2; // rdi
  int v3; // r8d
  int v4; // ecx
  int v5; // r9d
  unsigned char *v6; // rax
  long long v7; // rdi
  int v8; // r10d
  int v9; // ecx
  int v10; // r11d
  unsigned char *v11; // rax
  int v12; // r10d
  int v13; // r12d
  unsigned char *v14; // rax
  short *v15; // rax
  short *v16; // rax
  short v17; // di
  char *v18; // rbx
  short *v19; // rax
  short v20; // r8
  short v21; // ax
  short v22; // r8
  short *v23; // rax
  long long i; // rax
  int v25; // esi
  unsigned int v26; // edi
  long long j; // rbx
  short v29[20]; // [rsp+0h] [rbp-58h]
  unsigned long long v30; // [rsp+28h] [rbp-30h]

  v1 = (unsigned short)word_DEA82[0];
  v30 = __readfsqword(0x28u);
  qword_DCA20 = a1;
  qword_DCA28 = 0;
  qword_DCA30 = 0;
  if ( word_DEA82[0] )
    return v30 - __readfsqword(0x28u);
  v2 = 0;
  v3 = 0;
  do
  {
    v4 = dword_182C0[v2];
    dword_DDAE0[v2] = v3;
    v5 = 1 << v4;
    if ( 1 << v4 > 0 )
    {
      v6 = &byte_DDD60[v3];
      do
        *v6++ = v2;
      while ( v6 != &byte_DDD60[v3 + 1 + v5 - 1] );
      v3 += v5;
    }
    ++v2;
  }
  while ( v2 != 28 );
  v7 = 0;
  v8 = 0;
  byte_DDD60[v3 - 1] = 28;
  do
  {
    v9 = dword_18240[v7];
    dword_DDA60[v7] = v8;
    v10 = 1 << v9;
    if ( 1 << v9 > 0 )
    {
      v11 = &byte_DDB60[v8];
      do
        *v11++ = v7;
      while ( v11 != &byte_DDB60[v8 + 1 + v10 - 1] );
      v8 += v10;
    }
    ++v7;
  }
  while ( v7 != 16 );
  v12 = v8 >> 7;
  do
  {
    dword_DDA60[v7] = v12 << 7;
    v13 = 1 << (dword_18240[v7] - 7);
    if ( v13 > 0 )
    {
      v14 = &byte_DDB60[v12];
      do
      {
        v14[256] = v7;
        ++v14;
      }
      while ( v14 != &byte_DDB60[v12 + 1 + v13 - 1] );
      v12 += v13;
    }
    ++v7;
  }
  while ( v7 != 30 );
  v15 = word_DE9C0;
  do
    *v15++ = 0;
  while ( v15 != word_DE9E0 );
  v16 = &unk_DEB02;
  v17 = word_DE9D0;
  v18 = (char *)&unk_DEB02 - 2;
  do
  {
    v16 += 2;
    *(v16 - 2) = 8;
  }
  while ( v16 != (short *)&unk_DED42 );
  v19 = &unk_DED42;
  v20 = word_DE9D2;
  do
  {
    v19 += 2;
    *(v19 - 2) = 9;
  }
  while ( v19 != (short *)&unk_DEF02 );
  v21 = v20 + 112;
  v22 = word_DE9CE;
  word_DE9D2 = v21;
  v23 = (short *)&unk_DEF02;
  do
  {
    v23 += 2;
    *(v23 - 2) = 7;
  }
  while ( v23 != &word_DEF62 );
  word_DEF66 = 8;
  word_DEF6A = 8;
  word_DEF6E = 8;
  word_DEF72 = 8;
  word_DEF76 = 8;
  word_DEF7A = 8;
  word_DEF7E = 8;
  word_DE9CE = v22 + 24;
  word_DEF62 = 8;
  word_DE9D0 = v17 + 152;
  for ( i = 0; i != 15; ++i )
  {
    LOWORD(v1) = word_DE9C0[i] + v1;
    v1 *= 2;
    v29[i + 1] = v1;
  }
  do
  {
    v25 = *((unsigned short *)v18 + 1);
    if ( *((short *)v18 + 1) )
    {
      v26 = (unsigned short)v29[*((unsigned short *)v18 + 1)];
      v29[*((unsigned short *)v18 + 1)] = v26 + 1;
      *(short *)v18 = sub_44A0(v26, v25);
    }
    v18 += 4;
  }
  while ( &unk_DEF80 != (_UNKNOWN *)v18 );
  for ( j = 0; j != 30; ++j )
  {
    word_DEA82[2 * j] = 5;
    word_DEA82[2 * j - 1] = sub_44A0(j, 5);
  }
  return sub_9DF0();
}



// Function: flush_block @ 0xadd0
long long flush_block(char *a1, long long a2, int a3, int a4)
{
  short *v6; // r8
  bool v7; // zf
  long long v8; // rax
  long long i; // rdx
  long long v10; // rbx
  long long v11; // rax
  unsigned long long v12; // rax
  unsigned long long v13; // rdx
  unsigned long long v14; // rsi
  int v15; // r15d
  int v16; // edi
  unsigned char *v17; // r15
  int v18; // edi
  char *v19; // rbx
  long long j; // rax
  long long v21; // rax
  unsigned short *v23; // rax
  unsigned int v24; // edx
  int v25; // ecx
  unsigned short *v26; // rax
  unsigned int v27; // ecx
  int v28; // esi
  short *v29; // rax
  int v30; // esi
  int v32; // [rsp+10h] [rbp-48h]
  int v34; // [rsp+1Ch] [rbp-3Ch]

  v6 = (short *)qword_DCA20;
  v7 = *(short *)qword_DCA20 == 0xFFFF;
  byte_DCA60[dword_DCA4C] = byte_DCA49;
  if ( v7 )
  {
    v23 = (unsigned short *)&unk_DF080;
    v24 = 0;
    do
    {
      v25 = *v23;
      v23 += 2;
      v24 += v25;
    }
    while ( &unk_DF09C != (_UNKNOWN *)v23 );
    v26 = (unsigned short *)&unk_DF09C;
    v27 = 0;
    do
    {
      v28 = *v26;
      v26 += 2;
      v27 += v28;
    }
    while ( &unk_DF280 != (_UNKNOWN *)v26 );
    v29 = (short *)&unk_DF280;
    do
    {
      v30 = (unsigned short)*v29;
      v29 += 2;
      v24 += v30;
    }
    while ( &word_DF480 != v29 );
    *v6 = v24 <= v27 >> 2;
  }
  sub_A090((long long)&off_183C0);
  sub_A090((long long)&off_18380);
  sub_9F80((long long)&unk_DF080, dword_183E4);
  sub_9F80((long long)&unk_DEF80, dword_183A4);
  sub_A090((long long)&off_18340);
  v8 = 18;
  for ( i = 15; ; i = byte_13410[v8] )
  {
    v10 = (int)v8;
    if ( word_DE9E0[2 * i + 1] )
      break;
    if ( --v8 == 2 )
    {
      v11 = 23;
      v10 = 2;
      goto LABEL_7;
    }
  }
  v11 = 3 * (int)v8 + 17;
LABEL_7:
  qword_DCA40 += v11;
  v12 = (unsigned long long)(qword_DCA40 + 10) >> 3;
  v13 = (unsigned long long)(qword_DCA38 + 10) >> 3;
  qword_DCA28 += a2;
  v14 = v13;
  if ( v12 <= v13 )
    v14 = (unsigned long long)(qword_DCA40 + 10) >> 3;
  if ( a2 + 4 > v14 || !a1 )
  {
    if ( v12 >= v13 )
    {
      sub_43B0(a4 + 2, 3);
      sub_A870((long long)&unk_DEB00, (long long)&unk_DEA80);
      qword_DCA30 += qword_DCA38 + 3;
    }
    else
    {
      sub_43B0(a4 + 4, 3);
      v15 = dword_183A4;
      v34 = dword_183E4;
      v32 = dword_183A4;
      sub_43B0(dword_183E4 - 256, 5);
      v16 = v15;
      v17 = byte_13410;
      sub_43B0(v16, 5);
      v18 = v10 - 3;
      v19 = (char *)&byte_13410[v10];
      sub_43B0(v18, 4);
      for ( j = 16; ; j = *++v17 )
      {
        sub_43B0((unsigned short)word_DE9E0[2 * j + 1], 3);
        if ( v19 == (char *)v17 )
          break;
      }
      sub_A6B0((long long)&unk_DF080, v34);
      sub_A6B0((long long)&unk_DEF80, v32);
      sub_A870((long long)&unk_DF080, (long long)&unk_DEF80);
      qword_DCA30 += qword_DCA40 + 3;
    }
    sub_9DF0();
    if ( !a4 )
      goto LABEL_17;
LABEL_22:
    sub_44D0();
    v21 = qword_DCA30 + 7;
    qword_DCA30 += 7;
    return v21 >> 3;
  }
  sub_43B0(a4, 3);
  qword_DCA30 = ((qword_DCA30 + 10) & 0xFFFFFFFFFFFFFFF8LL) + 8 * (a2 + 4);
  sub_45C0(a1, a2, 1);
  sub_9DF0();
  if ( a4 )
    goto LABEL_22;
LABEL_17:
  v21 = qword_DCA30;
  if ( a3 && (qword_DCA30 & 7) != 0 )
  {
    sub_43B0(0, 3);
    qword_DCA30 = (qword_DCA30 + 10) & 0xFFFFFFFFFFFFFFF8LL;
    sub_45C0(a1, 0, 1);
    v21 = qword_DCA30;
  }
  return v21 >> 3;
}



// Function: ct_tally @ 0xb160
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

  v2 = (unsigned int)dword_DCA54;
  v3 = ++dword_DCA54;
  byte_9C000[v2] = a2;
  v4 = byte_DCA48;
  if ( a1 )
  {
    v12 = a1 - 1;
    ++*((short *)&unk_DF080 + 2 * byte_DDD60[a2] + 514);
    if ( a1 - 1 <= 255 )
      v13 = byte_DDB60[v12];
    else
      v13 = byte_DDB60[(v12 >> 7) + 256];
    ++word_DEF80[2 * v13];
    v14 = (unsigned int)dword_DCA50++;
    word_4B000[v14] = v12;
    byte_DCA49 |= v4;
  }
  else
  {
    ++*((short *)&unk_DF080 + 2 * a2);
  }
  byte_DCA48 = 2 * v4;
  if ( (v3 & 7) == 0 )
  {
    v5 = (unsigned int)dword_DCA4C;
    v6 = byte_DCA49;
    byte_DCA48 = 1;
    byte_DCA49 = 0;
    ++dword_DCA4C;
    byte_DCA60[v5] = v6;
  }
  if ( dword_1801C <= 2 )
    goto LABEL_10;
  if ( (v3 & 0xFFF) != 0 )
    goto LABEL_10;
  v7 = 0;
  v8 = 8LL * v3;
  do
  {
    v9 = (int)dword_18240[v7 / 2];
    v10 = (unsigned short)word_DEF80[v7];
    v7 += 2LL;
    v8 += v10 * (v9 + 5);
  }
  while ( v7 != 60 );
  if ( v3 >> 1 <= dword_DCA50
    || (result = 1, ((unsigned long long)(unsigned int)dword_1906C - qword_19078) >> 1 <= v8 >> 3) )
  {
LABEL_10:
    result = 1;
    if ( v3 != 0x7FFF )
      return dword_DCA50 == 0x8000;
  }
  return result;
}



// Function: fillbuf @ 0xb320
long long fillbuf(int a1)
{
  int v1; // ebx
  unsigned int v2; // eax
  int v3; // edx
  int v4; // ecx
  int v5; // ecx
  long long result; // rax

  v1 = a1;
  v2 = dword_DF990;
  v3 = (unsigned short)word_DF994 << a1;
  v4 = dword_DF98C;
  if ( dword_DF98C < a1 )
  {
    do
    {
      v1 -= v4;
      v3 |= v2 << v1;
      word_DF994 = v3;
      if ( dword_1A008 < (unsigned int)dword_1A00C )
      {
        v2 = (unsigned char)byte_9C000[dword_1A008++];
        dword_DF990 = v2;
      }
      else
      {
        v2 = sub_D6C0(1);
        dword_DF990 = v2;
        if ( v2 == -1 )
        {
          dword_DF990 = 0;
          v3 = (unsigned short)word_DF994;
          v2 = 0;
        }
        else
        {
          v3 = (unsigned short)word_DF994;
        }
      }
      dword_DF98C = 8;
      v4 = 8;
    }
    while ( v1 > 8 );
  }
  v5 = v4 - v1;
  result = v2 >> v5;
  dword_DF98C = v5;
  word_DF994 = result | v3;
  return result;
}



// Function: make_table @ 0xb3f0
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
    sub_D3F0("Bad table\n");
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
                *((short *)&qword_1A880 + v8) = 0;
                *((short *)&qword_1A880 + v8 + 0x8000) = 0;
                v32 = (unsigned short)v8;
                *v30 = v8++;
              }
              if ( (v29 & (1 << (15 - a3))) == 0 )
                break;
              v30 = (short *)&qword_1A880 + v32 + 0x8000;
              v29 *= 2;
              if ( !--v31 )
                goto LABEL_31;
            }
            v30 = (short *)&qword_1A880 + v32;
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



// Function: read_pt_len @ 0xb700
int *read_pt_len(signed int a1, int a2, int a3)
{
  int v4; // ebx
  int v5; // r15d
  char *v6; // r14
  char *v7; // rbp
  int v8; // edi
  short v9; // r12
  unsigned int v10; // edx
  int v11; // eax
  int v12; // edi
  char *v13; // rax
  int *result; // rax
  int v15; // ebp
  long long v16; // rax
  int v17; // ebx
  char *v18; // rax
  int v20; // [rsp+Ch] [rbp-3Ch]

  v4 = (int)(unsigned short)word_DF994 >> (16 - a2);
  sub_B320(a2);
  if ( v4 )
  {
    v5 = 0;
    v20 = a3 - 1;
LABEL_3:
    v6 = (char *)&unk_DFBC0 + v5;
    v7 = (char *)&unk_DFBC1 + v5 + (unsigned int)(v20 - v5);
    while ( v5 < v4 )
    {
      v8 = 3;
      v9 = (unsigned short)word_DF994 >> 13;
      if ( (unsigned short)word_DF994 >> 13 == 7 )
      {
        if ( (word_DF994 & 0x1000) != 0 )
        {
          v10 = 4096;
          v11 = 7;
          do
          {
            v10 >>= 1;
            v12 = v11++;
          }
          while ( ((unsigned short)v10 & (unsigned short)word_DF994) != 0 );
          if ( v11 > 16 )
            sub_D3F0("Bad table\n");
          v8 = v12 - 2;
          LOBYTE(v9) = v11;
        }
        else
        {
          LOBYTE(v9) = 7;
          v8 = 4;
        }
      }
      sub_B320(v8);
      *v6++ = v9;
      ++v5;
      if ( v6 == v7 )
      {
        v15 = (int)(unsigned short)word_DF994 >> 14;
        sub_B320(2);
        if ( v15 )
        {
          v16 = v5;
          do
            *((char *)&unk_DFBC0 + v16++) = 0;
          while ( v16 != v5 + (long long)v15 );
          v5 += v15;
        }
        goto LABEL_3;
      }
    }
    if ( v5 < a1 )
    {
      v13 = (char *)&unk_DFBC0 + v5;
      do
        *v13++ = 0;
      while ( v13 != (char *)&unk_DFBC1 + v5 + (unsigned int)(a1 - 1 - v5) );
    }
    return (int *)sub_B3F0(a1, (unsigned char *)&unk_DFBC0, 8u, (long long)&unk_DF9A0);
  }
  else
  {
    v17 = (int)(unsigned short)word_DF994 >> (16 - a2);
    sub_B320(a2);
    v18 = &unk_DFBC0;
    do
      *v18++ = 0;
    while ( v18 != (char *)&unk_DFBC1 + (unsigned int)(a1 - 1) );
    result = (int *)&unk_DF9A0;
    do
    {
      *(short *)result = v17;
      result = (int *)((char *)result + 2);
    }
    while ( result != &dword_DFBA0 );
  }
  return result;
}



// Function: unlzh @ 0xb920
long long unlzh(int a1, int a2)
{
  int v2; // esi
  unsigned int v3; // edx
  long long v4; // rcx
  char v5; // di
  long long v6; // rdi
  unsigned int v7; // r13d
  int v8; // edx
  int v9; // ecx
  long long v10; // rbp
  unsigned int v11; // edx
  long long v12; // rdx
  int v14; // ebp
  int v15; // ebp
  int v16; // r15d
  long long v17; // r15
  int v18; // edx
  long long v19; // rcx
  long long v20; // rdx
  unsigned int v21; // ecx
  int v22; // r15d
  short *v23; // rcx
  short *v24; // rdx
  long long v25; // rbp
  unsigned int v26; // edx
  char v27; // si
  int v28; // edx
  int v29; // ecx
  char v30; // di
  long long v31; // rsi
  int v32; // r9d
  int v33; // ebp
  short *v34; // rdx
  int v35; // ebp
  unsigned short *v36; // rdx
  int v37; // r12d
  int v38; // [rsp+4h] [rbp-54h]
  int v39; // [rsp+18h] [rbp-40h]

  dword_1A014 = a1;
  dword_1A010 = a2;
  word_DF994 = 0;
  dword_DF990 = 0;
  dword_DF98C = 0;
  sub_B320(16);
  dword_DFBA0 = 0;
  dword_DF988 = 0;
  dword_DF984 = 0;
  do
  {
    v2 = dword_DF988;
    v3 = dword_DF980;
    v4 = 0;
    v5 = 0;
    while ( 1 )
    {
      v7 = v4;
      if ( --v2 < 0 )
        break;
      v6 = v3;
      v3 = ((short)v3 + 1) & 0x1FFF;
      byte_3B000[v4++] = byte_3B000[v6];
      v5 = 1;
      if ( v4 == 0x2000 )
      {
        dword_DF988 = v2;
        v7 = 0x2000;
        dword_DF980 = v3;
        goto LABEL_15;
      }
    }
    dword_DF988 = v2;
    if ( v5 )
      dword_DF980 = v3;
    while ( 1 )
    {
      v8 = dword_DFBA0;
      v9 = (unsigned short)word_DF994;
      if ( !dword_DFBA0 )
        break;
LABEL_8:
      dword_DFBA0 = v8 - 1;
      v10 = word_4B000[(unsigned short)v9 >> 4];
      if ( (unsigned int)v10 > 0x1FD )
      {
        v11 = 8;
        do
        {
          while ( (v9 & v11) != 0 )
          {
            v11 >>= 1;
            v10 = *((unsigned short *)&qword_1A880 + (unsigned int)(v10 + 0x8000));
            if ( (unsigned int)v10 <= 0x1FD )
              goto LABEL_13;
          }
          LODWORD(v10) = *((unsigned short *)&qword_1A880 + v10);
          v11 >>= 1;
        }
        while ( (unsigned int)v10 > 0x1FD );
      }
LABEL_13:
      sub_B320(*((unsigned char *)&word_5B000 + (unsigned int)v10));
      if ( (unsigned int)v10 > 0xFF )
      {
        dword_DF988 = v10 - 253;
        v25 = word_DF9A0[HIBYTE(word_DF994)];
        if ( (unsigned int)v25 > 0xD )
        {
          v26 = 128;
          do
          {
            while ( ((unsigned short)word_DF994 & (unsigned short)v26) != 0 )
            {
              v26 >>= 1;
              v25 = *((unsigned short *)&qword_1A880 + (unsigned int)(v25 + 0x8000));
              if ( (unsigned int)v25 <= 0xD )
                goto LABEL_47;
            }
            LODWORD(v25) = *((unsigned short *)&qword_1A880 + v25);
            v26 >>= 1;
          }
          while ( (unsigned int)v25 > 0xD );
        }
LABEL_47:
        sub_B320(*((unsigned char *)&unk_DFBC0 + (unsigned int)v25));
        if ( (int)v25 )
        {
          v37 = v25 - 1;
          LODWORD(v25) = (int)(unsigned short)word_DF994 >> (16 - (v25 - 1));
          sub_B320(v37);
          LOWORD(v25) = (1 << v37) + v25;
        }
        v27 = 0;
        v28 = ((short)v7 - 1 - (short)v25) & 0x1FFF;
        v29 = dword_DF988 - 1;
        dword_DF980 = v28;
        while ( 1 )
        {
          v32 = v29;
          if ( v29 < 0 )
            break;
          --v29;
          v30 = byte_3B000[v28];
          v31 = v7++;
          v28 = ((short)v28 + 1) & 0x1FFF;
          byte_3B000[v31] = v30;
          v27 = 1;
          if ( v7 == 0x2000 )
          {
            dword_DF988 = v32;
            dword_DF980 = v28;
            goto LABEL_15;
          }
        }
        dword_DF988 = v29;
        if ( v27 )
          dword_DF980 = v28;
      }
      else
      {
        v12 = v7++;
        byte_3B000[v12] = v10;
        if ( v7 == 0x2000 )
          goto LABEL_15;
      }
    }
    v14 = (unsigned short)word_DF994;
    sub_B320(16);
    dword_DFBA0 = v14;
    if ( v14 )
    {
      sub_B700(19, 5, 3);
      v15 = 0;
      v16 = (int)(unsigned short)word_DF994 >> 7;
      v39 = v16;
      sub_B320(9);
      if ( v16 )
      {
        do
        {
          while ( 1 )
          {
            v20 = word_DF9A0[HIBYTE(word_DF994)];
            if ( word_DF9A0[HIBYTE(word_DF994)] > 0x12u )
            {
              v21 = 128;
              do
              {
                if ( ((unsigned short)word_DF994 & (unsigned short)v21) != 0 )
                  v20 = *((unsigned short *)&qword_1A880 + (int)v20 + 0x8000);
                else
                  LODWORD(v20) = *((unsigned short *)&qword_1A880 + v20);
                v21 >>= 1;
              }
              while ( (int)v20 > 18 );
            }
            v17 = (int)v20;
            v38 = v20;
            sub_B320(*((unsigned char *)&unk_DFBC0 + (int)v20));
            v18 = v38;
            if ( v38 <= 2 )
              break;
            v19 = v15++;
            *((char *)&word_5B000 + v19) = v38 - 2;
            if ( v39 <= v15 )
              goto LABEL_35;
          }
          if ( v38 )
          {
            v22 = (unsigned short)word_DF994;
            if ( v38 == 1 )
            {
              sub_B320(4);
              v18 = (v22 >> 12) + 2;
            }
            else
            {
              sub_B320(9);
              v18 = (v22 >> 7) + 19;
            }
            v17 = v18;
          }
          v23 = (short *)((char *)&word_5B000 + v15);
          do
          {
            *(char *)v23 = 0;
            v23 = (short *)((char *)v23 + 1);
          }
          while ( (short *)((char *)&word_5B000 + v15 + v17 + 1) != v23 );
          v15 += v18 + 1;
        }
        while ( v39 > v15 );
LABEL_35:
        if ( v15 <= 509 )
        {
          v24 = (short *)((char *)&word_5B000 + v15);
          do
          {
            *(char *)v24 = 0;
            v24 = (short *)((char *)v24 + 1);
          }
          while ( v24 != (short *)((char *)&word_5B000 + v15 + (unsigned int)(509 - v15) + 1) );
        }
        sub_B3F0(0x1FEu, (unsigned char *)&word_5B000, 0xCu, (long long)word_4B000);
      }
      else
      {
        v33 = (unsigned short)word_DF994;
        sub_B320(9);
        v34 = &word_5B000;
        v35 = v33 >> 7;
        do
        {
          *(char *)v34 = 0;
          v34 = (short *)((char *)v34 + 1);
        }
        while ( &unk_5B1FE != (_UNKNOWN *)v34 );
        v36 = word_4B000;
        do
          *v36++ = v35;
        while ( &word_4B000[4096] != v36 );
      }
      sub_B700(14, 4, -1);
      v8 = dword_DFBA0;
      v9 = (unsigned short)word_DF994;
      goto LABEL_8;
    }
    dword_DF984 = 1;
    if ( !v7 )
      return 0;
LABEL_15:
    sub_D5A0(a2, byte_3B000, v7);
  }
  while ( !dword_DF984 );
  return 0;
}



// Function: unlzw @ 0xbe90
long long unlzw(unsigned int a1, int a2)
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
  long long v13; // rdx
  long long v14; // rcx
  signed long long v15; // rdi
  long long v16; // rdx
  char v17; // cl
  long long v18; // rax
  char *v19; // rdx
  signed long long v20; // rax
  char v21; // si
  char *v22; // r14
  char *v23; // rdx
  int v24; // eax
  long long v25; // r13
  int v26; // edi
  long long v27; // rcx
  long long v28; // r13
  char *v29; // rax
  char v30; // dl
  long long v31; // rax
  char *v32; // r13
  int v33; // r15d
  int v34; // r12d
  char *v35; // rdi
  size_t v36; // rdx
  unsigned int v37; // r12d
  int v39; // ecx
  const char *v40; // rdi
  char v41; // [rsp+0h] [rbp-A8h]
  long long v42; // [rsp+8h] [rbp-A0h]
  unsigned int n; // [rsp+10h] [rbp-98h]
  size_t na; // [rsp+10h] [rbp-98h]
  char v45; // [rsp+1Ch] [rbp-8Ch]
  long long v46; // [rsp+20h] [rbp-88h]
  long long v47; // [rsp+38h] [rbp-70h]
  int v49; // [rsp+48h] [rbp-60h]
  size_t v50; // [rsp+48h] [rbp-60h]
  long long v51; // [rsp+50h] [rbp-58h]
  signed long long v52; // [rsp+58h] [rbp-50h]
  int v53; // [rsp+60h] [rbp-48h]
  int v54; // [rsp+64h] [rbp-44h]
  unsigned int v55; // [rsp+68h] [rbp-40h]

  v2 = (unsigned int)dword_1A008;
  if ( dword_1A008 >= (unsigned int)dword_1A00C )
  {
    v3 = sub_D6C0(0);
  }
  else
  {
    ++dword_1A008;
    v3 = (unsigned char)byte_9C000[v2];
  }
  dword_18024 = v3;
  dword_183E8 = v3 & 0x80;
  if ( (v3 & 0x60) == 0 )
    goto LABEL_8;
  if ( !dword_1A85C )
  {
    sub_F6B0(stderr, v41);
    if ( status )
      goto LABEL_7;
    goto LABEL_6;
  }
  if ( !status )
LABEL_6:
    status = 2;
LABEL_7:
  LOBYTE(v3) = dword_18024;
LABEL_8:
  dword_18024 = v3 & 0x1F;
  v51 = 1LL << (v3 & 0x1F);
  if ( (v3 & 0x1Fu) > 0x10 )
  {
    v37 = 1;
    sub_F6B0(stderr, v41);
    status = 1;
    return v37;
  }
  qword_1A880 = 0;
  LODWORD(v4) = dword_1A00C;
  qword_1A978 = 0;
  v5 = (unsigned int)(8 * dword_1A008);
  v54 = dword_1A00C;
  v42 = 256 - ((dword_183E8 == 0) - 1LL);
  memset(
    (void *)((unsigned long long)&unk_1A888 & 0xFFFFFFFFFFFFFFF8LL),
    0,
    8LL
  * (((unsigned int)&qword_1A880 - (unsigned int)((unsigned long long)&unk_1A888 & 0xFFFFFFFFFFFFFFF8LL) + 256) >> 3));
  v6 = 255;
  do
    byte_3B000[v6] = v6;
  while ( v6-- != 0 );
  v53 = 9;
  v46 = 511;
  v8 = 0;
  v9 = v6;
  v55 = 511;
  v45 = 0;
LABEL_12:
  v10 = v5 >> 3;
  if ( (unsigned int)v10 > (unsigned int)v4 )
  {
LABEL_13:
    dword_1A00C = 0;
    v11 = byte_9C000;
LABEL_14:
    n = v8;
    v12 = sub_D190(a1, v11);
    v8 = n;
    v54 = v12;
    if ( v12 == -1 )
      sub_D4E0(a1, v11, v13, v14, n);
    qword_1A828 += v12;
    LODWORD(v4) = v12 + dword_1A00C;
    dword_1A00C += v12;
    goto LABEL_16;
  }
  while ( 1 )
  {
    v4 = (unsigned int)(v4 - v10);
    if ( (int)v4 > 0 )
    {
      v29 = byte_9C000;
      do
      {
        v30 = v29[(int)v10];
        *v29++ = v30;
      }
      while ( (char *)&unk_9C001 + (unsigned int)(v4 - 1) != v29 );
    }
    dword_1A00C = v4;
    if ( (unsigned int)v4 <= 0x3F )
    {
      v11 = &byte_9C000[v4];
      goto LABEL_14;
    }
LABEL_16:
    if ( !v54 )
      break;
    v5 = 0;
    v47 = 8 * ((unsigned int)v4 - (unsigned long long)((unsigned int)v4 % v53));
    if ( v47 <= 0 )
      goto LABEL_12;
LABEL_18:
    if ( v42 <= v46 )
    {
      v5 = 0;
      while ( 1 )
      {
        v15 = v9;
        v16 = v5 >> 3;
        na = v9;
        v17 = v5 & 7;
        v5 += v53;
        v18 = (unsigned char)byte_9C000[v16];
        v9 = v55
           & (unsigned long long)((long long)(v18
                                        | ((unsigned long long)(unsigned char)byte_9C000[v16 + 2] << 16)
                                        | ((unsigned long long)(unsigned char)byte_9C000[v16 + 1] << 8)) >> v17);
        if ( v15 == -1 )
        {
          if ( v9 > 255 )
            sub_D3F0("corrupt input.");
          v31 = v8;
          v45 = v9;
          ++v8;
          *((char *)&word_5B000 + v31) = v9;
          if ( v5 >= v47 )
            goto LABEL_44;
        }
        else
        {
          if ( v9 != 256 )
          {
            if ( v9 < v42 )
            {
              v20 = v55
                  & (unsigned long long)((long long)(v18
                                               | ((unsigned long long)(unsigned char)byte_9C000[v16 + 2] << 16)
                                               | ((unsigned long long)(unsigned char)byte_9C000[v16 + 1] << 8)) >> v17);
              v19 = (char *)&unk_5AFFE;
              goto LABEL_26;
            }
            if ( v9 > v42 )
            {
              if ( v8 > 0 )
                sub_D5A0(a2, &word_5B000, (unsigned int)v8);
              v40 = "corrupt input.";
              if ( !dword_1A860 )
                v40 = "corrupt input. Use zcat to recover some data.";
              sub_D3F0(v40);
            }
LABEL_24:
            v19 = &byte_5AFFD;
            byte_5AFFD = v45;
            v20 = na;
            goto LABEL_26;
          }
          if ( dword_183E8 )
          {
            qword_1A880 = 0;
            qword_1A978 = 0;
            memset(
              (void *)((unsigned long long)&unk_1A888 & 0xFFFFFFFFFFFFFFF8LL),
              0,
              8LL
            * (((unsigned int)&qword_1A880 - (unsigned int)((unsigned long long)&unk_1A888 & 0xFFFFFFFFFFFFFFF8LL) + 256) >> 3));
            v55 = 511;
            v42 = 256;
            v39 = v53;
            LODWORD(v4) = dword_1A00C;
            v53 = 9;
            v46 = 511;
            v9 = na;
            v5 = 8 * v39 - (v5 - 1 + 8 * v39) % (8 * v39) + v5 - 1;
            goto LABEL_12;
          }
          v20 = 256;
          v19 = (char *)&unk_5AFFE;
          if ( v42 == 256 )
            goto LABEL_24;
          do
          {
            *--v19 = byte_3B000[v20];
            v20 = *((unsigned short *)&qword_1A880 + v20);
LABEL_26:
            ;
          }
          while ( v20 > 255 );
          v21 = byte_3B000[v20];
          v22 = v19 - 1;
          *(v19 - 1) = v21;
          v23 = (char *)(&unk_5AFFE - (_UNKNOWN *)(v19 - 1));
          v45 = v21;
          v24 = (int)v23;
          if ( v8 + (int)v23 > (int)&unk_3FFFF )
          {
            v50 = v5;
            v32 = v22;
            v52 = v9;
            v33 = v8;
            do
            {
              v34 = (int)&unk_40000 - v33;
              if ( (int)&unk_40000 - v33 > v24 )
                v34 = v24;
              if ( v34 > 0 )
              {
                v35 = (char *)&word_5B000 + v33;
                v33 += v34;
                memcpy(v35, v32, v34);
              }
              if ( v33 > (int)&unk_3FFFF )
              {
                v36 = (unsigned int)v33;
                v33 = 0;
                sub_D5A0(a2, &word_5B000, v36);
              }
              v32 += v34;
              v24 = &unk_5AFFE - (_UNKNOWN *)v32;
            }
            while ( v24 > 0 );
            v8 = v33;
            v5 = v50;
            v9 = v52;
          }
          else
          {
            v49 = v8 + (int)v23;
            memcpy((char *)&word_5B000 + v8, v22, (int)v23);
            v8 = v49;
          }
          if ( v51 > v42 )
          {
            *((short *)&qword_1A880 + v42) = na;
            byte_3B000[v42++] = v21;
          }
          if ( v5 >= v47 )
          {
LABEL_44:
            if ( !v54 )
              goto LABEL_60;
            LODWORD(v4) = dword_1A00C;
            goto LABEL_12;
          }
        }
        if ( v46 < v42 )
        {
          LODWORD(v4) = dword_1A00C;
          v25 = v5 - 1;
          goto LABEL_34;
        }
      }
    }
    v25 = -1;
LABEL_34:
    v26 = v53 + 1;
    v27 = 8 * v53;
    v53 = v26;
    v46 = v51;
    v28 = v27 - (v27 + v25) % v27 + v25;
    if ( dword_18024 != v26 )
      v46 = (1LL << v26) - 1;
    v10 = v28 >> 3;
    v55 = (1 << v26) - 1;
    if ( (unsigned int)v10 > (unsigned int)v4 )
      goto LABEL_13;
  }
  v47 = 8LL * (unsigned int)v4 - (v53 - 1);
  if ( v47 > 0 )
    goto LABEL_18;
LABEL_60:
  v37 = 0;
  if ( v8 > 0 )
    sub_D5A0(a2, &word_5B000, (unsigned int)v8);
  return v37;
}



// Function: unpack @ 0xc590
long long unpack(int a1, int a2)
{
  long long v2; // rax
  int v3; // ebp
  long long v4; // rdx
  unsigned int v5; // esi
  long long v6; // rbx
  int v7; // eax
  int v8; // eax
  long long v9; // r12
  int v10; // r13d
  int v11; // ebp
  int v12; // eax
  int v13; // edi
  long long v14; // rax
  int v15; // edx
  long long v16; // r15
  long long v17; // r13
  int v18; // eax
  long long v19; // r12
  long long v20; // r14
  long long v21; // rax
  int v22; // eax
  long long v23; // rax
  int v24; // ecx
  int v25; // edx
  int v26; // edx
  long long v27; // r8
  long long v28; // rsi
  short *v29; // rax
  int v30; // edx
  short *v31; // rdx
  int v32; // r8d
  unsigned long long v33; // rax
  unsigned int v34; // esi
  int v35; // r12d
  long long v36; // rdx
  int v37; // ecx
  char v38; // al
  unsigned long long v39; // rbx
  long long v40; // rax
  int i; // r13d
  long long v43; // rbx
  long long v44; // rax
  unsigned long long v45; // r14
  unsigned int v46; // edx
  int v47; // [rsp+8h] [rbp-40h]
  int v48; // [rsp+Ch] [rbp-3Ch]

  v2 = 0;
  v3 = 4;
  dword_1A014 = a1;
  v4 = (unsigned int)dword_1A008;
  qword_DFE88 = 0;
  dword_1A010 = a2;
  v5 = dword_1A00C;
  do
  {
    v6 = v2 << 8;
    if ( (unsigned int)v4 >= v5 )
    {
      v7 = sub_D6C0(0);
      if ( v7 < 0 )
        goto LABEL_64;
      v4 = (unsigned int)dword_1A008;
      v5 = dword_1A00C;
    }
    else
    {
      dword_1A008 = v4 + 1;
      LOBYTE(v7) = byte_9C000[v4];
      v4 = (unsigned int)(v4 + 1);
    }
    v2 = v6 | (unsigned char)v7;
    qword_DFE88 = v2;
    --v3;
  }
  while ( v3 );
  if ( (unsigned int)v4 >= v5 )
  {
    v8 = sub_D6C0(0);
    if ( v8 < 0 )
LABEL_64:
      sub_D3F0("invalid compressed data -- unexpected end of file");
    v8 = (unsigned char)v8;
  }
  else
  {
    dword_1A008 = v4 + 1;
    v8 = (unsigned char)byte_9C000[v4];
  }
  dword_DFE80 = v8;
  if ( (unsigned int)(v8 - 1) > 0x18 )
    sub_D3F0("invalid compressed data -- Huffman code bit length out of range");
  v9 = 1;
  v10 = 0;
  v11 = 1;
  do
  {
    v14 = (unsigned int)dword_1A008;
    if ( dword_1A008 < (unsigned int)dword_1A00C )
    {
      ++dword_1A008;
      v12 = (unsigned char)byte_9C000[v14];
    }
    else
    {
      v12 = sub_D6C0(0);
      if ( v12 < 0 )
        goto LABEL_64;
      v12 = (unsigned char)v12;
    }
    v13 = dword_DFE80;
    dword_DFC80[v9] = v12;
    if ( v11 - (v13 == (int)v9) < v12 )
      goto LABEL_76;
    ++v9;
    v10 += v12;
    v11 = 2 * (v11 - v12) + 1;
  }
  while ( v13 >= (int)v9 );
  if ( v10 > 255 )
LABEL_76:
    sub_D3F0("too many leaves in Huffman tree");
  v15 = dword_DFC80[v13];
  dword_DFC80[v13] = v15 + 1;
  if ( v13 <= 0 )
  {
    dword_DFC80[v13] = v15 + 2;
    goto LABEL_73;
  }
  v16 = 1;
  v17 = 0;
  do
  {
    dword_DFD00[v16] = v17;
    v18 = dword_DFC80[v16];
    if ( v18 > 0 )
    {
      v19 = (int)v17 + 1;
      v20 = v17 + (unsigned int)(v18 - 1) + 2;
      do
      {
        while ( 1 )
        {
          v21 = (unsigned int)dword_1A008;
          v17 = (int)v19;
          if ( dword_1A008 >= (unsigned int)dword_1A00C )
            break;
          ++v19;
          ++dword_1A008;
          *((char *)&unk_DFD7F + v19 - 1) = byte_9C000[v21];
          if ( v20 == v19 )
            goto LABEL_26;
        }
        v22 = sub_D6C0(0);
        if ( v22 < 0 )
          goto LABEL_64;
        *((char *)&unk_DFD7F + v19++) = v22;
      }
      while ( v20 != v19 );
LABEL_26:
      v13 = dword_DFE80;
    }
    ++v16;
  }
  while ( v13 >= (int)v16 );
  v23 = v13;
  v48 = dword_DFC80[v13];
  v24 = v48 + 1;
  dword_DFC80[v13] = v48 + 1;
  if ( v13 <= 0 )
    goto LABEL_73;
  v25 = 0;
  while ( 1 )
  {
    v26 = v25 >> 1;
    dword_DFC00[v23] = v26;
    dword_DFD00[v23--] -= v26;
    v25 = v24 + v26;
    if ( (int)v23 <= 0 )
      break;
    v24 = dword_DFC80[v23];
  }
  if ( v25 >> 1 != 1 )
LABEL_73:
    sub_D3F0("too few leaves in Huffman tree");
  v27 = 1;
  if ( v13 > 12 )
    v13 = 12;
  dword_DFBF0 = v13;
  v28 = 1 << v13;
  v29 = (short *)((char *)&word_5B000 + v28);
  do
  {
    v30 = dword_DFC80[v27] << (v13 - v27);
    if ( v30 )
    {
      v31 = (short *)((char *)v29 + ~(unsigned long long)(unsigned int)(v30 - 1));
      do
      {
        v29 = (short *)((char *)v29 - 1);
        *(char *)v29 = v27;
      }
      while ( v29 != v31 );
    }
    ++v27;
  }
  while ( v13 >= (int)v27 );
  if ( v29 > &word_5B000 )
  {
    do
    {
      v29 = (short *)((char *)v29 - 1);
      *(char *)v29 = 0;
    }
    while ( v29 != &word_5B000 );
  }
  dword_DFBE0 = 0;
  v32 = 0;
  qword_DFBE8 = 0;
  v47 = v28 - 1;
  v33 = 0;
  while ( v32 < v13 )
  {
LABEL_50:
    v39 = v33 << 8;
    v40 = (unsigned int)dword_1A008;
    if ( dword_1A008 >= (unsigned int)dword_1A00C )
    {
      LODWORD(v40) = sub_D6C0(0);
      if ( (int)v40 < 0 )
        goto LABEL_64;
      v32 = dword_DFBE0;
      v13 = dword_DFBF0;
    }
    else
    {
      ++dword_1A008;
      LOBYTE(v40) = byte_9C000[v40];
    }
    v32 += 8;
    v33 = v39 | (unsigned char)v40;
    dword_DFBE0 = v32;
    qword_DFBE8 = v33;
  }
  while ( 1 )
  {
    v34 = (v33 >> ((unsigned char)v32 - (unsigned char)v13)) & v47;
    v35 = *((unsigned char *)&word_5B000 + v34);
    if ( *((char *)&word_5B000 + v34) )
    {
      v34 >>= v13 - v35;
    }
    else
    {
      i = v47;
      if ( v34 >= dword_DFC00[v13] )
      {
        v35 = v13;
      }
      else
      {
        v43 = v13 + 1;
        while ( 1 )
        {
          v35 = v43;
          for ( i = 2 * i + 1; v32 < (int)v43; dword_DFBE0 = v32 )
          {
            v45 = v33 << 8;
            v44 = (unsigned int)dword_1A008;
            if ( dword_1A008 < (unsigned int)dword_1A00C )
            {
              ++dword_1A008;
              LOBYTE(v44) = byte_9C000[v44];
            }
            else
            {
              LODWORD(v44) = sub_D6C0(0);
              if ( (int)v44 < 0 )
                goto LABEL_64;
            }
            v33 = v45 | (unsigned char)v44;
            v32 = dword_DFBE0 + 8;
            qword_DFBE8 = v33;
          }
          v46 = dword_DFC00[v43++];
          v34 = i & (v33 >> ((unsigned char)v32 - (unsigned char)v35));
          if ( v46 <= v34 )
            break;
          v32 = dword_DFBE0;
        }
      }
    }
    if ( v34 == v48 && dword_DFE80 == v35 )
      break;
    v36 = (unsigned int)n;
    v38 = byte_DFD80[dword_DFD00[v35] + v34];
    LODWORD(n) = n + 1;
    v37 = n;
    byte_3B000[v36] = v38;
    if ( v37 == 0x8000 )
      sub_D640();
    v13 = dword_DFBF0;
    v33 = qword_DFBE8;
    v32 = dword_DFBE0 - v35;
    dword_DFBE0 = v32;
    if ( v32 < dword_DFBF0 )
      goto LABEL_50;
  }
  sub_D640();
  if ( (unsigned int)qword_1A820 != qword_DFE88 )
    sub_D3F0("invalid compressed data--length error");
  return 0;
}



// Function: check_zipfile @ 0xcb20
long long check_zipfile(int a1)
{
  char *v1; // rax
  unsigned char v3; // dl
  char v4; // al
  char v5; // [rsp+0h] [rbp-8h]

  dword_1A014 = a1;
  v1 = &byte_9C000[dword_1A008];
  dword_1A008 += *((unsigned short *)v1 + 13) + *((unsigned short *)v1 + 14) + 30;
  if ( dword_1A008 > (unsigned int)dword_1A00C
    || *(int *)v1 != 67324752
    || (v3 = v1[8], dword_18020 = v3, (v3 & 0xF7) != 0)
    || (v4 = v1[6], dword_DFEA0 = v4 & 1, (v4 & 1) != 0) )
  {
    sub_F6B0(stderr, v5);
    status = 1;
    return 1;
  }
  else
  {
    dword_DFE9C = 1;
    dword_DFE98 = (v4 & 8) != 0;
    return 0;
  }
}



// Function: unzip @ 0xcc20
long long unzip(int a1, int a2)
{
  long long v2; // rbp
  long long v3; // r12
  long long v4; // r13
  long long v5; // rax
  char v6; // al
  int v7; // ecx
  long long v8; // rax
  int v9; // eax
  unsigned int *v10; // rbx
  long long v11; // rax
  char *v12; // rsi
  unsigned int v13; // r13d
  unsigned long long v14; // rdi
  unsigned int *v16; // rbx
  long long v17; // rax
  unsigned int v18; // [rsp+0h] [rbp-48h] BYREF
  unsigned int v19; // [rsp+4h] [rbp-44h]
  char v20[4]; // [rsp+8h] [rbp-40h] BYREF
  unsigned int v21; // [rsp+Ch] [rbp-3Ch]
  char v22; // [rsp+10h] [rbp-38h] BYREF
  unsigned long long v23; // [rsp+18h] [rbp-30h]

  v23 = __readfsqword(0x28u);
  dword_1A014 = a1;
  dword_1A010 = a2;
  sub_D0D0(0, 0);
  if ( dword_DFE9C )
  {
    if ( dword_DFE98 )
    {
      v3 = 0;
      v2 = 0;
    }
    else
    {
      v2 = (unsigned int)dword_9C00E;
      v3 = (unsigned int)dword_9C016;
    }
    if ( dword_18020 != 8 )
    {
      if ( !dword_18020 )
      {
        v4 = (unsigned int)dword_9C016;
        v5 = (unsigned int)dword_9C012;
        if ( dword_DFEA0 )
          v5 = (unsigned int)dword_9C012 - 12LL;
        if ( dword_9C016 != v5 )
        {
          sub_F6B0(stderr, v18);
          sub_D3F0("invalid compressed data--length mismatch");
        }
        if ( dword_9C016 )
        {
          do
          {
            while ( 1 )
            {
              v8 = (unsigned int)dword_1A008;
              if ( dword_1A008 < (unsigned int)dword_1A00C )
              {
                ++dword_1A008;
                v6 = byte_9C000[v8];
              }
              else
              {
                v6 = sub_D6C0(0);
              }
              v7 = n + 1;
              byte_3B000[(unsigned int)n] = v6;
              LODWORD(n) = v7;
              if ( v7 == 0x8000 )
                break;
              if ( !--v4 )
                goto LABEL_39;
            }
            sub_D640();
            --v4;
          }
          while ( v4 );
        }
LABEL_39:
        sub_D640();
        if ( dword_DFE9C )
          goto LABEL_40;
        goto LABEL_21;
      }
LABEL_55:
      sub_D3F0("internal error, invalid method");
    }
  }
  else
  {
    if ( dword_18020 != 8 )
      goto LABEL_55;
    v3 = 0;
    v2 = 0;
  }
  v9 = sub_9A50();
  if ( v9 == 3 )
    sub_D430();
  if ( v9 )
    sub_D3F0("invalid compressed data--format violated");
  if ( dword_DFE9C )
  {
LABEL_40:
    if ( dword_DFE98 )
    {
      v16 = &v18;
      do
      {
        while ( 1 )
        {
          v17 = (unsigned int)dword_1A008;
          if ( dword_1A008 >= (unsigned int)dword_1A00C )
            break;
          v16 = (unsigned int *)((char *)v16 + 1);
          ++dword_1A008;
          *((char *)v16 - 1) = byte_9C000[v17];
          if ( v16 == (unsigned int *)&v22 )
            goto LABEL_45;
        }
        v16 = (unsigned int *)((char *)v16 + 1);
        *((char *)v16 - 1) = sub_D6C0(0);
      }
      while ( v16 != (unsigned int *)&v22 );
LABEL_45:
      v2 = v19;
      v3 = v21;
    }
    goto LABEL_26;
  }
LABEL_21:
  v10 = &v18;
  do
  {
    while ( 1 )
    {
      v11 = (unsigned int)dword_1A008;
      if ( dword_1A008 >= (unsigned int)dword_1A00C )
        break;
      v10 = (unsigned int *)((char *)v10 + 1);
      ++dword_1A008;
      *((char *)v10 - 1) = byte_9C000[v11];
      if ( v10 == (unsigned int *)v20 )
        goto LABEL_25;
    }
    v10 = (unsigned int *)((char *)v10 + 1);
    *((char *)v10 - 1) = sub_D6C0(0);
  }
  while ( v10 != (unsigned int *)v20 );
LABEL_25:
  v2 = v18;
  v3 = v19;
LABEL_26:
  v12 = 0;
  v13 = 0;
  if ( sub_D0D0(&word_5B000, 0) != v2 )
  {
    v12 = "\n%s: %s: invalid compressed data--crc error\n";
    v13 = 1;
    sub_F6B0(stderr, v18);
  }
  if ( (unsigned int)qword_1A820 == v3 )
  {
    v14 = (unsigned int)dword_DFE9C;
    if ( !dword_DFE9C )
      goto LABEL_33;
  }
  else
  {
    v14 = (unsigned long long)stderr;
    v12 = "\n%s: %s: invalid compressed data--length error\n";
    sub_F6B0(stderr, v18);
    if ( !dword_DFE9C )
      goto LABEL_49;
    v13 = 1;
  }
  if ( dword_1A008 + 4 >= (unsigned int)dword_1A00C )
    goto LABEL_33;
  v12 = byte_9C000;
  if ( ((*((unsigned char *)&unk_9C001 + (unsigned int)dword_1A008) << 8)
      | (unsigned int)(unsigned char)byte_9C000[dword_1A008]
      | (unsigned long long)((long long)(*((unsigned char *)&unk_9C002 + (unsigned int)dword_1A008)
                                   | (byte_9C003[dword_1A008] << 8)) << 16)) != 0x4034B50 )
    goto LABEL_33;
  v12 = (char *)(unsigned int)dword_1A860;
  if ( dword_1A860 )
  {
    if ( !dword_1A85C )
    {
      v14 = (unsigned long long)stderr;
      v12 = "%s: %s has more than one entry--rest ignored\n";
      sub_F6B0(stderr, v18);
    }
    if ( !status )
      status = 2;
LABEL_33:
    dword_DFE9C = 0;
    dword_DFE98 = 0;
    qword_DFE90 = v2;
    if ( !v13 )
      return v13;
    goto LABEL_34;
  }
  v14 = (unsigned long long)stderr;
  v12 = "%s: %s has more than one entry -- unchanged\n";
  sub_F6B0(stderr, v18);
  dword_DFE9C = 0;
LABEL_49:
  dword_DFE98 = 0;
  qword_DFE90 = v2;
LABEL_34:
  status = 1;
  if ( !dword_1A858 )
    sub_8440(v14, v12);
  return 1;
}



// Function: getcrc @ 0xd130
long long getcrc()
{
  return qword_183F0 ^ 0xFFFFFFFFLL;
}



// Function: clear_bufs @ 0xd150
void clear_bufs()
{
  qword_1A004 = 0;
  dword_1A00C = 0;
  qword_1A820 = 0;
  qword_1A828 = 0;
}



// Function: read_buffer @ 0xd190
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
      v12 = sub_E0F0(fd, 3, v7, v8, v10, v11, v16);
      if ( v12 >= 0 )
      {
        if ( (v12 & 0x800) != 0 )
        {
          BYTE1(v12) &= ~8u;
          if ( (unsigned int)sub_E0F0(fd, 4, v12, v13, v14, v15, v17) != -1 )
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



// Function: strlwr @ 0xd230
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



// Function: add_envopt @ 0xd2b0
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
    v4 = (char *)sub_F640(v3);
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
      v10 = (long long *)sub_F540(v5 + 2, 8);
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



// Function: gzip_error @ 0xd3f0
void gzip_error()
{
  FILE *v0; // rdi
  const char *v1; // rdx
  const char *v2; // rcx
  const char *v3; // r8
  char v4; // [rsp-8h] [rbp-8h]

  v0 = stderr;
  sub_F6B0(stderr, v4);
  sub_8440(v0, "\n%s: %s: %s\n", v1, v2, v3);
}



// Function: xalloc_die @ 0xd430
void xalloc_die()
{
  FILE *v0; // rdi
  const char *v1; // rdx
  char v2; // [rsp-8h] [rbp-8h]

  v0 = stderr;
  sub_F6B0(stderr, v2);
  sub_8440(v0, "\n%s: memory_exhausted\n", v1);
}



// Function: warning @ 0xd460
long long warning()
{
  long long result; // rax
  char v1; // [rsp-8h] [rbp-8h]

  if ( dword_1A85C )
  {
    result = (unsigned int)status;
    if ( !status )
      status = 2;
  }
  else
  {
    result = sub_F6B0(stderr, v1);
    if ( !status )
      status = 2;
  }
  return result;
}



// Function: read_error @ 0xd4e0
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
  sub_F6B0(stderr, v4);
  if ( v1 )
  {
    *v2 = v1;
    perror(dword_1A420);
    sub_8440(dword_1A420, "\n%s: ");
  }
  v3 = stderr;
  sub_F6B0(stderr, v5);
  sub_8440(v3, "%s: unexpected end of file\n");
}



// Function: write_error @ 0xd550
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
  sub_F6B0(stderr, v4);
  *v2 = v1;
  perror(file);
  sub_8440(file, "\n%s: ", v3);
}



// Function: flush_outbuf @ 0xd600
long long flush_outbuf()
{
  long long result; // rax

  if ( (int)qword_1A004 )
  {
    result = sub_D5A0(dword_1A010, &word_5B000, (unsigned int)qword_1A004);
    LODWORD(qword_1A004) = 0;
  }
  return result;
}



// Function: flush_window @ 0xd640
void flush_window()
{
  unsigned long long v0; // rax
  char *v1; // rdx
  char v2; // cl

  if ( (int)qword_1A004 )
  {
    v0 = qword_183F0;
    v1 = byte_3B000;
    do
    {
      v2 = *v1++;
      v0 = qword_13800[(unsigned char)(v0 ^ v2)] ^ (v0 >> 8);
    }
    while ( v1 != &byte_3B000[(unsigned int)qword_1A004] );
    qword_183F0 = v0;
    sub_D5A0(dword_1A010, byte_3B000, (unsigned int)qword_1A004);
    LODWORD(qword_1A004) = 0;
  }
}



// Function: fill_inbuf @ 0xd6c0
long long fill_inbuf(int a1)
{
  long long v1; // rax
  int v2; // eax

  v1 = 0;
  dword_1A00C = 0;
  while ( 1 )
  {
    v2 = sub_D190(dword_1A014, &byte_9C000[v1], (unsigned int)&unk_40000 - (unsigned int)v1);
    if ( !v2 )
      break;
    if ( v2 == -1 )
      goto LABEL_10;
    v1 = (unsigned int)(dword_1A00C + v2);
    dword_1A00C = v1;
    if ( (unsigned int)v1 > (unsigned int)&unk_3FFFF )
      goto LABEL_8;
  }
  v1 = (unsigned int)dword_1A00C;
  if ( dword_1A00C )
  {
LABEL_8:
    HIDWORD(qword_1A004) = 1;
    qword_1A828 += v1;
    return (unsigned char)byte_9C000[0];
  }
  if ( !a1 )
  {
    sub_D640();
    *__errno_location() = 0;
LABEL_10:
    sub_D4E0();
  }
  return 0xFFFFFFFFLL;
}



// Function: copy @ 0xd770
long long copy(int fd, int a2)
{
  int v2; // r13d
  long long v3; // r14
  size_t v4; // r13
  char *v5; // r14
  size_t v6; // rdx
  unsigned int v7; // eax
  int v8; // eax

  *__errno_location() = 0;
  v2 = dword_1A00C;
  v3 = HIDWORD(qword_1A004);
  if ( HIDWORD(qword_1A004) < dword_1A00C )
  {
    do
    {
      v4 = (unsigned int)(v2 - v3);
      v5 = &byte_9C000[v3];
      qword_1A820 += (unsigned int)v4;
      if ( !dword_1A858 )
      {
        while ( 1 )
        {
          v6 = 0x7FFFFFFF;
          if ( (unsigned int)v4 <= 0x7FFFFFFF )
            v6 = v4;
          v7 = write(a2, v5, v6);
          if ( v7 == (int)v4 )
            break;
          if ( v7 == -1 )
            sub_D550();
          v4 = (unsigned int)v4 - v7;
          v5 += v7;
        }
      }
      v8 = sub_D190(fd, byte_9C000, (unsigned int)&unk_40000);
      v2 = v8;
      if ( v8 == -1 )
        sub_D4E0();
      dword_1A00C = v8;
      v3 = 0;
      qword_1A828 += v8;
      HIDWORD(qword_1A004) = 0;
    }
    while ( v8 );
  }
  return 0;
}



// Function: display_ratio @ 0xd850
long long display_ratio(long long a1, long long a2, FILE *a3, long long a4, long long a5, long long a6, char a7)
{
  return sub_F6B0(a3, a7);
}



// Function: fprint_off @ 0xd8a0
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



// Function: zip @ 0xd9d0
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
  dword_1A010 = a2;
  v39 = 0;
  dword_1A014 = a1;
  word_5B000 = -29921;
  dword_18020 = 8;
  byte_5B002 = 8;
  LODWORD(qword_1A004) = 4;
  byte_5B003 = 8 * (dword_1A848 != 0);
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
    byte_5B004 = v3;
    v7 = 7;
    byte_5B005 = v4;
    LODWORD(v8) = 6;
LABEL_5:
    *((char *)&word_5B000 + (unsigned int)v8) = v5;
    LODWORD(qword_1A004) = v8 + 2;
    *((char *)&word_5B000 + v7) = v6;
    goto LABEL_6;
  }
  sub_D460("file timestamp out of range for gzip format");
  v34 = qword_1A004;
  if ( (unsigned int)qword_1A004 <= (unsigned int)&unk_3FFFD )
  {
    v8 = (unsigned int)(qword_1A004 + 2);
    v7 = (unsigned int)(qword_1A004 + 3);
    *((char *)&word_5B000 + (unsigned int)qword_1A004) = 0;
    *((char *)&word_5B000 + (unsigned int)(v34 + 1)) = 0;
  }
  else
  {
    v35 = qword_1A004 + 1;
    *((char *)&word_5B000 + (unsigned int)qword_1A004) = 0;
    LODWORD(qword_1A004) = v35;
    if ( v35 == (int)&unk_40000 )
    {
      sub_D600();
      v35 = qword_1A004;
    }
    v8 = v35 + 1;
    LODWORD(qword_1A004) = v35 + 1;
    *((char *)&word_5B000 + v35) = 0;
    if ( (int)v8 == (int)&unk_40000 )
    {
      sub_D600();
      v8 = (unsigned int)qword_1A004;
      v7 = (unsigned int)(qword_1A004 + 1);
      if ( (unsigned int)qword_1A004 > (unsigned int)&unk_3FFFD )
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
  LODWORD(qword_1A004) = v7;
  *((char *)&word_5B000 + v8) = 0;
  if ( (int)v7 == (int)&unk_40000 )
  {
    sub_D600();
    v7 = (unsigned int)qword_1A004;
  }
  *((char *)&word_5B000 + v7) = 0;
  LODWORD(qword_1A004) = v7 + 1;
  if ( (int)v7 + 1 == (int)&unk_40000 )
    sub_D600();
LABEL_6:
  sub_D0D0(0, 0);
  sub_4380(a2);
  v9 = &dword_18020;
  sub_AA20((long long)&v39);
  v10 = 4;
  if ( dword_1801C != 1 )
    v10 = 2 * (dword_1801C == 9);
  v11 = (unsigned int)(qword_1A004 + 1);
  *((char *)&word_5B000 + (unsigned int)qword_1A004) = v10;
  LODWORD(qword_1A004) = v11;
  if ( (int)v11 == (int)&unk_40000 )
  {
    sub_D600();
    v11 = (unsigned int)qword_1A004;
  }
  v12 = (unsigned int)(v11 + 1);
  *((char *)&word_5B000 + v11) = 3;
  LODWORD(qword_1A004) = v12;
  if ( (int)v12 == (int)&unk_40000 )
  {
    sub_D600();
    v12 = (unsigned int)qword_1A004;
  }
  if ( dword_1A848 )
  {
    v13 = (char *)sub_D290(dword_1A420);
    do
    {
      while ( 1 )
      {
        v14 = *v13;
        LODWORD(qword_1A004) = v12 + 1;
        *((char *)&word_5B000 + v12) = v14;
        v12 = (unsigned int)(v12 + 1);
        if ( (int)v12 == (int)&unk_40000 )
          break;
        if ( !*v13++ )
          goto LABEL_17;
      }
      sub_D600();
      ++v13;
      v12 = (unsigned int)qword_1A004;
    }
    while ( *(v13 - 1) );
  }
LABEL_17:
  v15 = (unsigned int)dword_1801C;
  qword_DFEA8 = (unsigned int)v12;
  sub_4BC0(dword_1801C);
  if ( qword_1A840 != -1 && qword_1A840 != qword_1A828 )
  {
    v15 = (unsigned long long)stderr;
    v9 = (int *)"%s: %s: file size changed while zipping\n";
    sub_F6B0(stderr, v38);
  }
  v16 = sub_D130(v15, v9);
  v17 = (unsigned int)qword_1A004;
  v18 = (unsigned int)(qword_1A004 + 1);
  LODWORD(qword_1A004) = qword_1A004 + 1;
  if ( (unsigned int)v17 > (unsigned int)&unk_3FFFD )
  {
    *((char *)&word_5B000 + v17) = v16;
    if ( (int)v18 == (int)&unk_40000 )
    {
      sub_D600();
      v18 = (unsigned int)qword_1A004;
    }
    v36 = sub_D130(v15, v9);
    v20 = (unsigned int)(v18 + 1);
    LODWORD(qword_1A004) = v18 + 1;
    *((char *)&word_5B000 + v18) = BYTE1(v36);
    if ( (int)v20 == (int)&unk_40000 )
    {
      sub_D600();
      v20 = (unsigned int)qword_1A004;
    }
  }
  else
  {
    v19 = (unsigned int)v17;
    v20 = (unsigned int)(v17 + 2);
    *((char *)&word_5B000 + v19) = v16;
    v21 = sub_D130(v15, v9);
    LODWORD(qword_1A004) = v20;
    *((char *)&word_5B000 + v18) = BYTE1(v21);
  }
  v22 = sub_D130(v15, v9);
  v23 = (unsigned int)(v20 + 1);
  LODWORD(qword_1A004) = v20 + 1;
  v24 = v22 >> 16;
  if ( (unsigned int)v20 > (unsigned int)&unk_3FFFD )
  {
    *((char *)&word_5B000 + v20) = v24;
    if ( (int)v23 == (int)&unk_40000 )
    {
      sub_D600();
      v23 = (unsigned int)qword_1A004;
    }
    v37 = sub_D130(v15, v9);
    v26 = (unsigned int)(v23 + 1);
    LODWORD(qword_1A004) = v23 + 1;
    *((char *)&word_5B000 + v23) = BYTE3(v37);
    if ( (int)v26 == (int)&unk_40000 )
    {
      sub_D600();
      v26 = (unsigned int)qword_1A004;
    }
  }
  else
  {
    v25 = (unsigned int)v20;
    v26 = (unsigned int)(v20 + 2);
    *((char *)&word_5B000 + v25) = v24;
    *((char *)&word_5B000 + v23) = (unsigned int)sub_D130(v15, v9) >> 24;
  }
  v27 = qword_1A828;
  v28 = (unsigned int)(v26 + 1);
  if ( (unsigned int)v26 > (unsigned int)&unk_3FFFD )
  {
    LODWORD(qword_1A004) = v26 + 1;
    *((char *)&word_5B000 + v26) = qword_1A828;
    if ( (int)v28 == (int)&unk_40000 )
    {
      sub_D600();
      v27 = qword_1A828;
      v28 = (unsigned int)qword_1A004;
    }
    v30 = (unsigned int)(v28 + 1);
    *((char *)&word_5B000 + v28) = BYTE1(v27);
    LODWORD(qword_1A004) = v30;
    if ( (int)v30 == (int)&unk_40000 )
    {
      sub_D600();
      v30 = (unsigned int)qword_1A004;
      v27 = qword_1A828;
    }
  }
  else
  {
    v29 = (unsigned int)v26;
    v30 = (unsigned int)(v26 + 2);
    *((char *)&word_5B000 + v29) = qword_1A828;
    *((char *)&word_5B000 + v28) = BYTE1(v27);
  }
  v31 = v27 >> 16;
  v32 = (unsigned int)(v30 + 1);
  if ( (unsigned int)v30 > (unsigned int)&unk_3FFFD )
  {
    LODWORD(qword_1A004) = v30 + 1;
    *((char *)&word_5B000 + v30) = v31;
    if ( (int)v32 == (int)&unk_40000 )
    {
      sub_D600();
      v32 = (unsigned int)qword_1A004;
      v31 = (unsigned long long)qword_1A828 >> 16;
    }
    *((char *)&word_5B000 + v32) = BYTE1(v31);
    LODWORD(qword_1A004) = v32 + 1;
    if ( (int)v32 + 1 == (int)&unk_40000 )
      sub_D600();
  }
  else
  {
    *((char *)&word_5B000 + (unsigned int)v30) = v31;
    LODWORD(qword_1A004) = v30 + 2;
    *((char *)&word_5B000 + v32) = BYTE1(v31);
  }
  qword_DFEA8 += 8;
  sub_D600();
  return 0;
}



// Function: file_read @ 0xdf80
long long file_read(void *buf, unsigned int a2)
{
  unsigned int v2; // eax
  unsigned int v3; // r12d

  v2 = sub_D190(dword_1A014, buf, a2);
  v3 = v2;
  if ( v2 )
  {
    if ( v2 == -1 )
      sub_D4E0();
    sub_D0D0(buf, v2);
    qword_1A828 += v3;
  }
  return v3;
}



// Function: last_component @ 0xdfd0
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



// Function: rpl_fcntl @ 0xe0f0
long long rpl_fcntl(int a1, int a2, long long a3, long long a4, long long a5, long long a6)
{
  int v6; // r12d
  int v8; // eax
  int *v9; // rax
  int v10; // edi
  int v11; // r13d
  int *v12; // rbp
  long long v13; // [rsp+40h] [rbp-38h]
  long long v14; // [rsp+48h] [rbp-30h]
  long long v15; // [rsp+50h] [rbp-28h]
  long long v16; // [rsp+58h] [rbp-20h]

  v13 = a3;
  v14 = a4;
  v15 = a5;
  v16 = a6;
  if ( !a2 )
    return (unsigned int)fcntl(a1, 0, (unsigned int)v13);
  if ( a2 != 1030 )
  {
    if ( a2 > 11 )
    {
      if ( (unsigned int)(a2 - 1024) > 0xA )
        return (unsigned int)fcntl(a1, a2, v13);
      if ( ((1LL << a2) & 0x2C5) == 0 )
      {
        if ( ((1LL << a2) & 0x502) == 0 )
          return (unsigned int)fcntl(a1, a2, v13);
        return (unsigned int)fcntl(a1, a2);
      }
    }
    else
    {
      if ( a2 <= 0 )
        return (unsigned int)fcntl(a1, a2, v13);
      if ( ((1LL << a2) & 0x514) == 0 )
      {
        if ( ((1LL << a2) & 0xA0A) == 0 )
          return (unsigned int)fcntl(a1, a2, v13);
        return (unsigned int)fcntl(a1, a2);
      }
    }
    return (unsigned int)fcntl(a1, a2, (unsigned int)v13);
  }
  if ( dword_DFEB0 < 0 )
  {
    v6 = fcntl(a1, 0, (unsigned int)v13);
    if ( v6 < 0 || dword_DFEB0 != -1 )
      return (unsigned int)v6;
  }
  else
  {
    v6 = fcntl(a1, 1030, (unsigned int)v13);
    if ( v6 >= 0 || *__errno_location() != 22 )
    {
      dword_DFEB0 = 1;
      return (unsigned int)v6;
    }
    v6 = fcntl(a1, 0, (unsigned int)v13);
    if ( v6 < 0 )
      return (unsigned int)v6;
    dword_DFEB0 = -1;
  }
  v8 = fcntl(v6, 1);
  if ( v8 < 0 || fcntl(v6, 2, v8 | 1u) == -1 )
  {
    v9 = __errno_location();
    v10 = v6;
    v6 = -1;
    v11 = *v9;
    v12 = v9;
    close(v10);
    *v12 = v11;
  }
  return (unsigned int)v6;
}



// Function: open_safer @ 0xe360
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
  return sub_E870(v8);
}



// Function: rpl_fseeko @ 0xe430
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



// Function: openat_safer @ 0xe4b0
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
  return sub_E870(v8);
}



// Function: direntry_cmp_name @ 0xe530
int direntry_cmp_name(const char **a1, const char **a2)
{
  return strcmp(*a1, *a2);
}



// Function: streamsavedir @ 0xe540
char *streamsavedir(DIR *dirp, unsigned int a2)
{
  long long v2; // rbx
  size_t v3; // r15
  char *v4; // r13
  char *v5; // r14
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

  compar = (int (*)(const void *, const void *))qword_17B50[a2];
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
          if ( !v3 )
          {
            v33 = src;
            srca = (void *)(v9 + 1);
            v23 = sub_F440(v4);
            v10 = v33;
            v11 = (signed long long)srca;
            v4 = (char *)v23;
          }
          v12 = (long long *)&v4[8 * v3];
          v32 = v11;
          ++v3;
          srcb = v12;
          v13 = sub_F640(v10);
          v14 = v32;
          *srcb = v13;
        }
        else
        {
          if ( -v2 <= v11 )
          {
            v34 = src;
            srcc = (void *)(v9 + 1);
            v24 = sub_F440(v5);
            v10 = v34;
            v11 = (signed long long)srcc;
            v5 = (char *)v24;
          }
          srcd = (void *)v11;
          memcpy(&v5[v2], v10, v11);
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
    v25 = v5;
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
        v5 = (char *)sub_F180(v18);
        do
        {
          v20 = stpcpy(&v5[v17], *(const char **)&v4[8 * v19]);
          v21 = *(void **)&v4[8 * v19++];
          v17 = v20 - v5 + 1;
          free(v21);
        }
        while ( v3 != v19 );
        v16 = &v5[v17];
      }
      else
      {
        v5 = (char *)sub_F180(v2 + 1);
        v16 = v5;
      }
      free(v4);
    }
    else
    {
      if ( !v2 )
        v5 = (char *)sub_F1F0(v5, 1);
      v16 = &v5[v2];
    }
    *v16 = 0;
  }
  return v5;
}



// Function: savedir @ 0xe800
char *savedir(long long a1, unsigned int a2)
{
  DIR *v2; // rax
  DIR *v3; // rbp
  char *v4; // r12

  v2 = (DIR *)sub_10AF0();
  if ( !v2 )
    return 0;
  v3 = v2;
  v4 = sub_E540(v2, a2);
  if ( !closedir(v3) )
    return v4;
  free(v4);
  return 0;
}



// Function: fd_safer @ 0xe870
long long fd_safer(unsigned int fd)
{
  unsigned int v2; // ebp
  int *v3; // rax
  int v4; // r13d
  int *v5; // rbx

  if ( fd > 2 )
    return fd;
  v2 = sub_10C00();
  v3 = __errno_location();
  v4 = *v3;
  v5 = v3;
  close(fd);
  *v5 = v4;
  return v2;
}



// Function: validate_timespec @ 0xe8d0
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



// Function: update_timespec @ 0xe980
long long update_timespec(long long *a1, struct timespec **a2)
{
  struct timespec *v2; // rbx
  __syscall_slong_t tv_nsec; // rdx
  __syscall_slong_t v4; // rax
  __syscall_slong_t v5; // rax
  unsigned int v6; // r12d
  __syscall_slong_t v8; // rdx

  v2 = *a2;
  tv_nsec = (*a2)->tv_nsec;
  v4 = (*a2)[1].tv_nsec;
  if ( tv_nsec == 1073741822 )
  {
    v6 = 1;
    if ( v4 == 1073741822 )
      return v6;
    v8 = a1[10];
    v2->tv_sec = a1[9];
    v2->tv_nsec = v8;
  }
  else if ( tv_nsec == 0x3FFFFFFF )
  {
    if ( v4 == 0x3FFFFFFF )
    {
      *a2 = 0;
      return 0;
    }
    sub_10BA0(*a2);
    v4 = v2[1].tv_nsec;
    if ( v4 == 1073741822 )
      goto LABEL_4;
  }
  else if ( v4 == 1073741822 )
  {
LABEL_4:
    v5 = a1[12];
    v6 = 0;
    v2[1].tv_sec = a1[11];
    v2[1].tv_nsec = v5;
    return v6;
  }
  v6 = 0;
  if ( v4 != 0x3FFFFFFF )
    return v6;
  sub_10BA0(v2 + 1);
  return 0;
}



// Function: fdutimens @ 0xea40
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
  bool v16; // bl
  bool v17; // r13
  __m128i si128; // xmm3
  __time_t v19; // rax
  bool v20; // dl
  bool v21; // dl
  struct timeval *tvp; // [rsp+8h] [rbp-130h] BYREF
  struct timespec times; // [rsp+10h] [rbp-128h] BYREF
  __m128i v24; // [rsp+20h] [rbp-118h]
  long long v25[4]; // [rsp+30h] [rbp-108h] BYREF
  struct timeval v26; // [rsp+50h] [rbp-E8h] BYREF
  __m128i v27; // [rsp+60h] [rbp-D8h]
  struct stat buf; // [rsp+70h] [rbp-C8h] BYREF
  unsigned long long v29; // [rsp+108h] [rbp-30h]

  p_times = (struct timespec *)a3;
  v29 = __readfsqword(0x28u);
  if ( a3 )
  {
    v5 = _mm_loadu_si128(a3 + 1);
    tvp = (struct timeval *)&times;
    times = (struct timespec)_mm_loadu_si128(a3);
    v24 = v5;
    v6 = sub_E8D0(&times);
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
  if ( dword_DFEB8 < 0 )
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
      *p_times = buf.st_atim;
    }
    else
    {
      v6 = 3;
      if ( p_times[1].tv_nsec == 1073741822 )
        p_times[1] = buf.st_mtim;
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
    dword_DFEB8 = 1;
    return (unsigned int)v7;
  }
LABEL_11:
  dword_DFEB8 = -1;
  dword_DFEB4 = -1;
  if ( !v6 )
    goto LABEL_18;
  if ( v6 != 3 && (fd < 0 ? stat(file, &buf) != 0 : fstat(fd, &buf) != 0) )
    return (unsigned int)-1;
  if ( p_times )
  {
    v7 = 0;
    if ( (unsigned char)sub_E980(&buf, (struct timespec **)&tvp) )
      return (unsigned int)v7;
  }
LABEL_18:
  v10 = tvp;
  if ( tvp )
  {
    tv_usec = tvp->tv_usec;
    v25[0] = tvp->tv_sec;
    v12 = tv_usec;
    v25[2] = tvp[1].tv_sec;
    v13 = tvp[1].tv_usec;
    v10 = (struct timeval *)v25;
    v25[1] = v12 / 1000;
    v25[3] = v13 / 1000;
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
    v16 = v10->tv_usec > (long long)&unk_7A11F;
    v17 = v10[1].tv_usec > (long long)&unk_7A11F;
    if ( (v10->tv_usec > (long long)&unk_7A11F || v10[1].tv_usec > (long long)&unk_7A11F) && !fstat(fd, &buf) )
    {
      si128 = _mm_load_si128((const __m128i *)&v10[1]);
      v19 = buf.st_atim.tv_sec - v10->tv_sec;
      v20 = buf.st_mtim.tv_sec - v10[1].tv_sec == 1;
      v26 = (struct timeval)_mm_load_si128((const __m128i *)v10);
      v27 = si128;
      v21 = v17 && v20;
      if ( v19 == 1 && v16 && !buf.st_atim.tv_nsec )
      {
        v26.tv_usec = 0;
        if ( !v21 || buf.st_mtim.tv_nsec )
          goto LABEL_43;
      }
      else if ( !v21 || buf.st_mtim.tv_nsec )
      {
        return 0;
      }
      v27.m128i_i64[1] = 0;
LABEL_43:
      futimesat(fd, 0, &v26);
      return 0;
    }
  }
  return (unsigned int)v7;
}



// Function: lutimens @ 0xee90
long long lutimens(char *file, const __m128i *a2)
{
  struct timespec *v2; // rbx
  __m128i v3; // xmm1
  int v4; // r13d
  int v5; // eax
  unsigned int v6; // r12d
  struct timespec *p_times; // [rsp+8h] [rbp-F0h] BYREF
  struct timespec times; // [rsp+10h] [rbp-E8h] BYREF
  struct timespec st_mtim; // [rsp+20h] [rbp-D8h]
  struct stat buf; // [rsp+30h] [rbp-C8h] BYREF
  unsigned long long v12; // [rsp+C8h] [rbp-30h]

  v2 = (struct timespec *)a2;
  v12 = __readfsqword(0x28u);
  if ( a2 )
  {
    v3 = _mm_loadu_si128(a2 + 1);
    p_times = &times;
    times = (struct timespec)_mm_loadu_si128(a2);
    st_mtim = (struct timespec)v3;
    v4 = sub_E8D0(&times);
    if ( v4 < 0 )
      return (unsigned int)-1;
    if ( dword_DFEB4 < 0 )
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
        times = buf.st_atim;
      }
      else
      {
        v2 = &times;
        v4 = 3;
        if ( st_mtim.tv_nsec == 1073741822 )
          st_mtim = buf.st_mtim;
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
    if ( dword_DFEB4 < 0 )
    {
      dword_DFEB4 = -1;
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
      dword_DFEB8 = 1;
      dword_DFEB4 = 1;
      return v6;
    }
  }
  else
  {
    *__errno_location() = 38;
  }
LABEL_12:
  dword_DFEB4 = -1;
  if ( !v4 )
    goto LABEL_21;
  if ( v4 == 3 )
  {
    if ( !v2 )
    {
LABEL_16:
      if ( (buf.st_mode & 0xF000) != 0xA000 )
        return (unsigned int)sub_EA40(-1, file, (const __m128i *)p_times);
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
  if ( !(unsigned char)sub_E980(&buf, &p_times) )
    goto LABEL_16;
  return v6;
}



// Function: xmalloc @ 0xf160
void *xmalloc(size_t a1)
{
  void *result; // rax

  result = malloc(a1);
  if ( !result )
    sub_D430();
  return result;
}



// Function: ximalloc @ 0xf180
void *ximalloc(size_t a1)
{
  void *result; // rax

  result = malloc(a1);
  if ( !result )
    sub_D430();
  return result;
}



// Function: xirealloc @ 0xf1f0
void *xirealloc(void *a1, unsigned long long a2)
{
  void *result; // rax

  result = realloc(a1, (a2 == 0) | a2);
  if ( !result )
    sub_D430();
  return result;
}



// Function: xnmalloc @ 0xf2a0
long long xnmalloc(long long a1, long long a2)
{
  long long result; // rax

  result = reallocarray(0, a1, a2);
  if ( !result )
    sub_D430();
  return result;
}



// Function: xinmalloc @ 0xf2d0
long long xinmalloc(long long a1, long long a2)
{
  long long v2; // rdx
  long long v3; // rsi
  long long result; // rax

  if ( a1 && a2 )
  {
    v2 = a2;
    v3 = a1;
  }
  else
  {
    v3 = 1;
    v2 = 1;
  }
  result = reallocarray(0, v3, v2);
  if ( !result )
    sub_D430();
  return result;
}



// Function: x2realloc @ 0xf320
long long x2realloc(long long a1, unsigned long long *a2)
{
  unsigned long long v2; // rbx
  long long v3; // rax
  bool v4; // cf
  unsigned long long v5; // rbx
  long long result; // rax

  v2 = *a2;
  if ( a1 )
  {
    v3 = (v2 >> 1) + 1;
    v4 = __CFADD__(v3, v2);
    v5 = v3 + v2;
    if ( v4 )
      goto LABEL_6;
    result = reallocarray(a1, v5, 1);
    if ( !result )
    {
      if ( v5 )
        goto LABEL_6;
    }
    *a2 = v5;
  }
  else
  {
    if ( !v2 )
      v2 = 128;
    result = reallocarray(0, v2, 1);
    if ( !result )
LABEL_6:
      sub_D430();
    *a2 = v2;
  }
  return result;
}



// Function: xpalloc @ 0xf440
void *xpalloc(void *ptr, long long *a2, signed long long a3, long long a4, signed long long a5)
{
  long long v7; // rcx
  long long v8; // rbx
  bool v9; // of
  signed long long v10; // rbx
  signed long long v11; // rax
  signed long long v12; // rbp
  signed long long v13; // rcx
  void *result; // rax
  long long v15; // rbp

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
LABEL_18:
    v10 = v15 / a5;
    v12 = v15 - v15 % a5;
    if ( ptr )
      goto LABEL_10;
    goto LABEL_19;
  }
  if ( v12 <= 127 )
  {
    v15 = 128;
    goto LABEL_18;
  }
  if ( ptr )
    goto LABEL_10;
LABEL_19:
  *a2 = 0;
LABEL_10:
  if ( v10 - v7 < a3
    && ((v9 = __OFADD__(a3, v7), v13 = a3 + v7, v10 = v13, v9)
     || v13 > a4 && a4 >= 0
     || (v12 = a5 * v13, !is_mul_ok(a5, v13)))
    || (result = realloc(ptr, v12)) == 0 && (!ptr || v12) )
  {
    sub_D430();
  }
  *a2 = v10;
  return result;
}



// Function: xcalloc @ 0xf540
void *xcalloc(size_t a1, size_t a2)
{
  void *result; // rax

  result = calloc(a1, a2);
  if ( !result )
    sub_D430();
  return result;
}



// Function: xicalloc @ 0xf570
void *xicalloc(size_t a1, size_t a2)
{
  void *result; // rax

  result = calloc(a1, a2);
  if ( !result )
    sub_D430();
  return result;
}



// Function: xmemdup @ 0xf5a0
void *xmemdup(void *src, size_t n)
{
  void *v2; // rax

  v2 = sub_F160(n);
  return memcpy(v2, src, n);
}



// Function: ximemdup @ 0xf5d0
void *ximemdup(void *src, size_t n)
{
  void *v2; // rax

  v2 = sub_F180(n);
  return memcpy(v2, src, n);
}



// Function: ximemdup0 @ 0xf600
void *ximemdup0(void *src, size_t n)
{
  char *v2; // rax

  v2 = sub_F180(n + 1);
  v2[n] = 0;
  return memcpy(v2, src, n);
}



// Function: xstrdup @ 0xf640
void *xstrdup(char *src)
{
  size_t v1; // rax

  v1 = strlen(src);
  return sub_F5A0(src, v1 + 1);
}



// Function: yesno @ 0xf660
long long yesno()
{
  unsigned int v0; // r12d
  int v1; // eax

  v1 = getc(stdin);
  for ( LOBYTE(v0) = (v1 & 0xFFFFFFDF) == 89; v1 != 10; v1 = getc(stdin) )
  {
    if ( v1 == -1 )
      break;
  }
  return v0;
}



// Function: rpl_fprintf @ 0xf6b0
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
  v2 = (char *)sub_F900(v8, &n, a2, va);
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
    sub_10B90(s);
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



// Function: __printf__ @ 0xf830
long long __printf__()
{
  return sub_10A00(stdout);
}



// Function: vasnprintf @ 0xf900
char *vasnprintf(void *a1, size_t *a2, void *a3, long long a4, double a5)
{
  int v6; // eax
  char *v7; // r8
  long long v8; // rax
  unsigned long long v9; // rax
  size_t v10; // rdi
  size_t v11; // r12
  long long *v12; // rax
  size_t v13; // r13
  char *v14; // r15
  char *v15; // r9
  char *v16; // r14
  size_t v17; // r14
  size_t v18; // rbx
  int *v19; // rax
  void *v20; // r14
  char *v22; // rax
  void *v23; // r9
  char v24; // dl
  long long v25; // rax
  char *v26; // r14
  char *v27; // rax
  int v28; // edi
  int v29; // eax
  char *v30; // r12
  long long *v31; // rdi
  char *v32; // rsi
  char *v33; // rax
  char *v34; // rsi
  char *v35; // rax
  char v36; // al
  long long v37; // rax
  char *v38; // rax
  long long v39; // rax
  char *v40; // rcx
  int v41; // eax
  unsigned long long v42; // rax
  char *v43; // rax
  char *v44; // r14
  int *v45; // rax
  char *v46; // r12
  unsigned long long v47; // rsi
  bool v48; // r14
  char *v49; // rax
  char *v50; // rax
  char *v51; // rax
  char *v52; // rax
  int *v53; // rax
  char *v54; // rdi
  long long v55; // r9
  int v56; // eax
  unsigned long long v57; // r10
  int v58; // edx
  long long v59; // rax
  bool v60; // cf
  size_t v61; // rax
  unsigned long long v62; // rdx
  bool v63; // r14
  char *v64; // rax
  int v65; // eax
  long long v66; // rax
  char *v67; // rdi
  unsigned __int128 v68; // fst7
  long long v69; // rax
  char *v70; // rdi
  signed long long v71; // rdx
  void *v72; // rsp
  char *v73; // rax
  char *v74; // rax
  char *v75; // rax
  int *v76; // rax
  char *v77; // r14
  unsigned long long v78; // r12
  bool v79; // r14
  char *v80; // rax
  char *v81; // rax
  char *v82; // rax
  long long v83; // [rsp-8h] [rbp-448h]
  unsigned __int128 v84; // [rsp+10h] [rbp-430h]
  long long v85; // [rsp+20h] [rbp-420h] BYREF
  unsigned long long v86; // [rsp+28h] [rbp-418h]
  int v87; // [rsp+34h] [rbp-40Ch]
  void *ptr; // [rsp+38h] [rbp-408h]
  size_t *v89; // [rsp+40h] [rbp-400h]
  void *v90; // [rsp+48h] [rbp-3F8h]
  long long v91; // [rsp+50h] [rbp-3F0h]
  long long *v92; // [rsp+58h] [rbp-3E8h]
  char *v93; // [rsp+60h] [rbp-3E0h]
  void *v94; // [rsp+68h] [rbp-3D8h]
  void *src; // [rsp+70h] [rbp-3D0h]
  void *v96; // [rsp+78h] [rbp-3C8h]
  int v97; // [rsp+84h] [rbp-3BCh] BYREF
  unsigned int v98; // [rsp+88h] [rbp-3B8h]
  unsigned int v99; // [rsp+8Ch] [rbp-3B4h]
  char v100[8]; // [rsp+90h] [rbp-3B0h] BYREF
  void *v101; // [rsp+98h] [rbp-3A8h]
  char v102[224]; // [rsp+A0h] [rbp-3A0h] BYREF
  long long v103; // [rsp+180h] [rbp-2C0h] BYREF
  void *v104; // [rsp+188h] [rbp-2B8h]
  unsigned long long v105; // [rsp+190h] [rbp-2B0h]
  long long v106; // [rsp+198h] [rbp-2A8h]
  char v107[616]; // [rsp+1A0h] [rbp-2A0h] BYREF
  unsigned long long v108; // [rsp+408h] [rbp-38h]

  v96 = a1;
  v89 = a2;
  v108 = __readfsqword(0x28u);
  src = a3;
  if ( (int)sub_10E50(a3, &v103, v100) < 0 )
    return 0;
  v6 = sub_10C20(a4, v100);
  v7 = (char *)src;
  if ( v6 < 0 )
  {
    if ( v104 != v107 )
      free(v104);
    if ( v101 != v102 )
      free(v101);
    v76 = __errno_location();
    v15 = 0;
    *v76 = 22;
    return v15;
  }
  v8 = v105 + 7;
  if ( v105 >= 0xFFFFFFFFFFFFFFF9LL )
    v8 = -1;
  v60 = __CFADD__(v106, v8);
  v9 = v106 + v8;
  if ( v60 )
    goto LABEL_109;
  v10 = v9 + 6;
  v11 = __CFADD__(v9, 6);
  if ( v9 >= 0xFFFFFFFFFFFFFFFALL )
    goto LABEL_109;
  if ( v10 > 0xF9F )
  {
    if ( v9 != -7 )
    {
      v12 = (long long *)malloc(v10);
      v92 = v12;
      if ( v12 )
      {
        ptr = v12;
        v7 = (char *)src;
        goto LABEL_11;
      }
    }
LABEL_109:
    src = __errno_location();
    goto LABEL_27;
  }
  while ( &v85 != (long long *)((char *)&v85 - ((v9 + 29) & 0xFFFFFFFFFFFFF000LL)) )
    ;
  v71 = ((short)v9 + 29) & 0xFF0;
  v72 = alloca(v71);
  if ( (((short)v9 + 29) & 0xFF0) != 0 )
    *(long long *)((char *)&v84 + v71 + 8) = *(long long *)((char *)&v84 + v71 + 8);
  ptr = 0;
  v92 = &v85;
LABEL_11:
  v13 = 0;
  if ( v96 )
    v13 = *v89;
  v14 = (char *)v104;
  v15 = (char *)v96;
  v91 = 0;
  v16 = *(char **)v104;
  if ( *(char **)v104 == v7 )
    goto LABEL_103;
LABEL_14:
  v17 = v16 - v7;
  v18 = v17 + v11;
  if ( __CFADD__(v17, v11) )
    v18 = -1;
  if ( v13 < v18 )
  {
    if ( v13 )
    {
      if ( (v13 & 0x8000000000000000LL) != 0LL )
        goto LABEL_21;
      v13 *= 2LL;
    }
    else
    {
      v13 = 12;
    }
    if ( v13 < v18 )
      v13 = v18;
    if ( v13 != -1 )
    {
      if ( !v15 || v15 == v96 )
      {
        v93 = v7;
        v94 = v15;
        LOBYTE(src) = v15 == v96;
        v50 = (char *)malloc(v13);
        v7 = v93;
        if ( !v50 )
          goto LABEL_22;
        if ( v11 && (char)src )
        {
          src = v93;
          v51 = (char *)memcpy(v50, v94, v11);
          v7 = (char *)src;
          v15 = v51;
        }
        else
        {
          v15 = v50;
        }
      }
      else
      {
        src = v15;
        v94 = v7;
        v22 = (char *)realloc(v15, v13);
        v23 = src;
        if ( !v22 )
          goto LABEL_110;
        v7 = (char *)v94;
        v15 = v22;
      }
      goto LABEL_44;
    }
LABEL_21:
    v94 = v15;
LABEL_22:
    v19 = __errno_location();
    v20 = v94;
    src = v19;
LABEL_23:
    if ( v20 != v96 && v20 )
      goto LABEL_111;
    goto LABEL_25;
  }
LABEL_44:
  src = v15;
  memcpy(&v15[v11], v7, v17);
  v15 = (char *)src;
  while ( 1 )
  {
    if ( v103 == v91 )
    {
      v78 = v18 + 1;
      if ( v18 == -1 )
      {
        if ( v13 != -1 )
          goto LABEL_21;
        goto LABEL_239;
      }
      if ( v13 >= v78 )
      {
LABEL_239:
        v15[v18] = 0;
        if ( v13 > v78 && v15 != v96 )
        {
          v96 = v15;
          v81 = (char *)realloc(v15, v18 + 1);
          v15 = (char *)v96;
          if ( v81 )
            v15 = v81;
        }
        if ( ptr )
        {
          v96 = v15;
          free(ptr);
          v15 = (char *)v96;
        }
        if ( v104 != v107 )
        {
          v96 = v15;
          free(v104);
          v15 = (char *)v96;
        }
        if ( v101 != v102 )
        {
          v96 = v15;
          free(v101);
          v15 = (char *)v96;
        }
        *v89 = v18;
        return v15;
      }
      if ( v13 )
      {
        if ( (v13 & 0x8000000000000000LL) != 0LL )
          goto LABEL_21;
        v13 *= 2LL;
        if ( v13 >= v78 )
          goto LABEL_235;
      }
      else if ( v78 <= 0xC )
      {
        v13 = 12;
LABEL_235:
        v79 = v15 == v96;
        if ( !v15 || v15 == v96 )
        {
          src = v15;
          v82 = (char *)malloc(v13);
          v15 = (char *)src;
          if ( !v82 )
            goto LABEL_21;
          if ( v18 && v79 )
            v15 = (char *)memcpy(v82, src, v18);
          else
            v15 = v82;
        }
        else
        {
          src = v15;
          v80 = (char *)realloc(v15, v13);
          v23 = src;
          if ( !v80 )
            goto LABEL_110;
          v15 = v80;
        }
        goto LABEL_239;
      }
      if ( v18 == -2 )
        goto LABEL_21;
      v13 = v18 + 1;
      goto LABEL_235;
    }
    v24 = v14[72];
    v25 = *((long long *)v14 + 10);
    if ( v24 == 37 )
      break;
    if ( v25 == -1 )
      goto LABEL_258;
    v26 = (char *)v101;
    v27 = (char *)v101 + 32 * v25;
    v28 = *(int *)v27;
    LODWORD(v94) = *(int *)v27;
    if ( v24 == 110 )
    {
      v52 = (char *)*((long long *)v27 + 2);
      switch ( v28 )
      {
        case 18:
          *v52 = v18;
          v11 = v18;
          goto LABEL_102;
        case 19:
          *(short *)v52 = v18;
          v11 = v18;
          goto LABEL_102;
        case 20:
          *(int *)v52 = v18;
          v11 = v18;
          goto LABEL_102;
        case 21:
        case 22:
          *(long long *)v52 = v18;
          v11 = v18;
          goto LABEL_102;
        default:
          goto LABEL_258;
      }
    }
    v29 = *((int *)v14 + 4);
    v30 = (char *)v92 + 1;
    *(char *)v92 = 37;
    if ( (v29 & 1) != 0 )
    {
      v31 = v92;
      *((char *)v92 + 1) = 39;
      v30 = (char *)v31 + 2;
    }
    if ( (v29 & 2) != 0 )
      *v30++ = 45;
    if ( (v29 & 4) != 0 )
      *v30++ = 43;
    if ( (v29 & 8) != 0 )
      *v30++ = 32;
    if ( (v29 & 0x10) != 0 )
      *v30++ = 35;
    if ( (v29 & 0x40) != 0 )
      *v30++ = 73;
    if ( (v29 & 0x20) != 0 )
      *v30++ = 48;
    v32 = (char *)*((long long *)v14 + 3);
    v33 = (char *)*((long long *)v14 + 4);
    if ( v32 != v33 )
    {
      v93 = v15;
      src = (void *)(v33 - v32);
      memcpy(v30, v32, v33 - v32);
      v15 = v93;
      v30 = &v30[(long long)src];
    }
    v34 = (char *)*((long long *)v14 + 6);
    v35 = (char *)*((long long *)v14 + 7);
    if ( v34 != v35 )
    {
      v93 = v15;
      src = (void *)(v35 - v34);
      memcpy(v30, v34, v35 - v34);
      v15 = v93;
      v30 = &v30[(long long)src];
    }
    switch ( (int)v94 )
    {
      case 7:
      case 8:
      case 14:
      case 16:
        goto LABEL_69;
      case 9:
      case 10:
        *v30++ = 108;
LABEL_69:
        *v30++ = 108;
        break;
      case 12:
        *v30++ = 76;
        break;
      default:
        break;
    }
    v36 = v14[72];
    v30[1] = 0;
    *v30 = v36;
    v37 = *((long long *)v14 + 5);
    if ( v37 == -1 )
    {
      LODWORD(v93) = 0;
    }
    else
    {
      v38 = &v26[32 * v37];
      if ( *(int *)v38 != 5 )
        goto LABEL_258;
      LODWORD(v93) = 1;
      v98 = *((int *)v38 + 4);
    }
    v39 = *((long long *)v14 + 8);
    if ( v39 != -1 )
    {
      v40 = &v26[32 * v39];
      if ( *(int *)v40 != 5 )
        goto LABEL_258;
      v41 = (int)v93;
      *(&v98 + (unsigned int)v93) = *((int *)v40 + 4);
      LODWORD(v93) = v41 + 1;
    }
    v42 = v18 + 2;
    if ( v18 >= 0xFFFFFFFFFFFFFFFELL )
    {
      if ( v13 != -1 )
        goto LABEL_21;
LABEL_202:
      v44 = v15;
      goto LABEL_84;
    }
    if ( v13 >= v42 )
      goto LABEL_202;
    if ( v13 )
    {
      if ( (v13 & 0x8000000000000000LL) != 0LL )
        goto LABEL_21;
      v13 *= 2LL;
      if ( v13 >= v42 )
        goto LABEL_81;
    }
    else if ( v42 <= 0xC )
    {
      v13 = 12;
      goto LABEL_81;
    }
    if ( v18 == -3 )
      goto LABEL_21;
    v13 = v18 + 2;
LABEL_81:
    if ( !v15 || v15 == v96 )
    {
      v90 = v15;
      LOBYTE(src) = v15 == v96;
      v74 = (char *)malloc(v13);
      v15 = (char *)v90;
      v44 = v74;
      if ( !v74 )
        goto LABEL_21;
      if ( v18 && (char)src )
        memcpy(v74, v90, v18);
    }
    else
    {
      src = v15;
      v43 = (char *)realloc(v15, v13);
      v15 = (char *)src;
      v44 = v43;
      if ( !v43 )
        goto LABEL_21;
    }
LABEL_84:
    v44[v18] = 0;
    v45 = __errno_location();
    v90 = v30;
    v46 = v44;
    src = v45;
    v87 = *v45;
    while ( 2 )
    {
      v47 = 0x7FFFFFFF;
      v97 = -1;
      *(int *)src = 0;
      if ( v13 - v18 <= 0x7FFFFFFF )
        v47 = v13 - v18;
      switch ( (int)v94 )
      {
        case 1:
          v55 = (unsigned int)*((char *)v101 + 32 * *((long long *)v14 + 10) + 16);
          goto LABEL_138;
        case 2:
          v55 = *((unsigned char *)v101 + 32 * *((long long *)v14 + 10) + 16);
          goto LABEL_138;
        case 3:
          v55 = (unsigned int)*((short *)v101 + 16 * *((long long *)v14 + 10) + 8);
          goto LABEL_138;
        case 4:
          v55 = *((unsigned short *)v101 + 16 * *((long long *)v14 + 10) + 8);
          goto LABEL_138;
        case 5:
        case 6:
        case 13:
        case 14:
          v55 = *((unsigned int *)v101 + 8 * *((long long *)v14 + 10) + 4);
LABEL_138:
          v86 = v13 - v18;
          v54 = &v46[v18];
          if ( (int)v93 != 1 )
          {
            if ( (int)v93 != 2 )
            {
              *(long long *)&v84 = &v97;
              goto LABEL_141;
            }
            goto LABEL_189;
          }
          *((long long *)&v84 + 1) = &v97;
          *(long long *)&v84 = v55;
          goto LABEL_192;
        case 7:
        case 8:
        case 9:
        case 10:
        case 15:
        case 16:
        case 17:
          v54 = &v46[v18];
          v55 = *((long long *)v101 + 4 * *((long long *)v14 + 10) + 2);
          v86 = v13 - v18;
          if ( (int)v93 == 1 )
          {
            *((long long *)&v84 + 1) = &v97;
            *(long long *)&v84 = v55;
LABEL_192:
            v55 = v98;
LABEL_141:
            v56 = __snprintf_chk(v54, v47, 1, -1, v92, v55, a5, v84, *((long long *)&v84 + 1));
            v58 = v97;
            v57 = v86;
            if ( v97 >= 0 )
              goto LABEL_117;
LABEL_142:
            if ( !*((char *)v90 + 1) )
            {
              if ( v56 < 0 )
              {
                if ( !*(int *)src )
                {
                  v65 = 22;
                  if ( (v14[72] & 0xEF) == 0x63 )
                    v65 = 84;
                  *(int *)src = v65;
                }
                if ( v46 != v96 && v46 )
                  free(v46);
                if ( ptr )
                  free(ptr);
                if ( v104 != v107 )
                  free(v104);
                if ( v101 != v102 )
                  free(v101);
                return 0;
              }
LABEL_120:
              v97 = v56;
              v58 = v56;
              goto LABEL_121;
            }
            *((char *)v90 + 1) = 0;
            continue;
          }
          if ( (int)v93 != 2 )
          {
            v56 = __snprintf_chk(v54, v47, 1, -1, v92, v55, a5, &v97, *((long long *)&v84 + 1));
LABEL_115:
            v57 = v86;
            goto LABEL_116;
          }
LABEL_189:
          v56 = __snprintf_chk(v54, v47, 1, -1, v92, v98, a5, v99, v55);
LABEL_190:
          v57 = v86;
LABEL_116:
          v58 = v97;
          if ( v97 < 0 )
            goto LABEL_142;
LABEL_117:
          if ( v58 < v47 && v46[v58 + v18] )
            goto LABEL_258;
          if ( v58 < v56 )
            goto LABEL_120;
LABEL_121:
          if ( (unsigned int)(v58 + 1) >= v47 )
          {
            if ( v57 > 0x7FFFFFFE )
            {
              if ( v46 != v96 && v46 )
                free(v46);
              if ( ptr )
                free(ptr);
              if ( v104 != v107 )
                free(v104);
              if ( v101 != v102 )
                free(v101);
              v15 = 0;
              *(int *)src = 75;
              return v15;
            }
            v59 = (unsigned int)(v58 + 2);
            v60 = __CFADD__(v18, v59);
            v61 = v18 + v59;
            if ( (v13 & 0x8000000000000000LL) == 0LL )
            {
              v62 = 2 * v13;
              if ( v60 )
                goto LABEL_184;
              if ( v62 >= v61 )
                v61 = 2 * v13;
              goto LABEL_127;
            }
            if ( v60 )
            {
              if ( v13 != -1 )
                goto LABEL_184;
            }
            else
            {
              v61 = -1;
              v62 = -1;
LABEL_127:
              if ( v13 < v61 )
              {
                if ( !v13 )
                  v62 = 12;
                if ( v62 >= v61 )
                  v61 = v62;
                v13 = v61;
                if ( v61 == -1 )
                {
LABEL_184:
                  v20 = v46;
                  goto LABEL_23;
                }
                v63 = v46 == v96;
                if ( !v46 || v46 == v96 )
                {
                  v73 = (char *)malloc(v61);
                  if ( !v73 )
                    goto LABEL_184;
                  if ( v18 && v63 )
                    v46 = (char *)memcpy(v73, v46, v18);
                  else
                    v46 = v73;
                }
                else
                {
                  v64 = (char *)realloc(v46, v61);
                  if ( !v64 )
                    goto LABEL_184;
                  v46 = v64;
                }
              }
            }
            continue;
          }
          v77 = v46;
          v11 = v58 + v18;
          v15 = v77;
          *(int *)src = v87;
LABEL_102:
          v7 = (char *)*((long long *)v14 + 1);
          v16 = (char *)*((long long *)v14 + 11);
          v14 += 88;
          ++v91;
          if ( v16 != v7 )
            goto LABEL_14;
LABEL_103:
          v18 = v11;
          break;
        case 11:
          v69 = *((long long *)v14 + 10);
          v86 = v13 - v18;
          v70 = &v46[v18];
          a5 = *((double *)v101 + 4 * v69 + 2);
          if ( (int)v93 == 1 )
          {
            v56 = __snprintf_chk(v70, v47, 1, -1, v92, v98, a5, &v97, *((long long *)&v84 + 1));
          }
          else
          {
            if ( (int)v93 != 2 )
            {
              v56 = __snprintf_chk(v70, v47, 1, -1, v92, &v97, a5, v85, v86);
              v57 = v86;
              goto LABEL_116;
            }
            v56 = __snprintf_chk(v70, v47, 1, -1, v92, v98, a5, v99, &v97);
          }
          v57 = v86;
          goto LABEL_116;
        case 12:
          v66 = *((long long *)v14 + 10);
          v86 = v13 - v18;
          v67 = &v46[v18];
          v68 = *((unsigned __int128 *)v101 + 2 * v66 + 1);
          if ( (int)v93 == 1 )
          {
            v56 = __snprintf_chk(v67, v47, 1, -1, v92, v98, a5, v68, v68 >> 64);
            goto LABEL_190;
          }
          if ( (int)v93 != 2 )
          {
            v84 = v68;
            v56 = __snprintf_chk(v67, v47, 1, -1, v92, &v97, a5, v84, *((long long *)&v84 + 1));
            goto LABEL_115;
          }
          v56 = __snprintf_chk(v67, v47, 1, -1, v92, v98, a5, v99, v83);
          v57 = v86;
          goto LABEL_116;
        default:
          goto LABEL_258;
      }
      break;
    }
  }
  if ( v25 != -1 )
LABEL_258:
    abort();
  v11 = v18 + 1;
  if ( v18 == -1 )
    v11 = -1;
  if ( v13 >= v11 )
  {
LABEL_101:
    v15[v18] = 37;
    goto LABEL_102;
  }
  if ( !v13 )
  {
    v13 = 12;
    goto LABEL_94;
  }
  if ( (v13 & 0x8000000000000000LL) != 0LL )
    goto LABEL_21;
  v13 *= 2LL;
LABEL_94:
  if ( v13 < v11 )
    v13 = v11;
  if ( v13 == -1 )
    goto LABEL_21;
  v48 = v15 == v96;
  if ( !v15 || v15 == v96 )
  {
    src = v15;
    v75 = (char *)malloc(v13);
    v15 = (char *)src;
    if ( v75 )
    {
      if ( v18 && v48 )
        v15 = (char *)memcpy(v75, src, v18);
      else
        v15 = v75;
      goto LABEL_101;
    }
    goto LABEL_21;
  }
  src = v15;
  v49 = (char *)realloc(v15, v13);
  v23 = src;
  if ( v49 )
  {
    v15 = v49;
    goto LABEL_101;
  }
LABEL_110:
  v96 = v23;
  v53 = __errno_location();
  v20 = v96;
  src = v53;
LABEL_111:
  free(v20);
LABEL_25:
  if ( ptr )
    free(ptr);
LABEL_27:
  if ( v104 != v107 )
    free(v104);
  if ( v101 != v102 )
    free(v101);
  *(int *)src = 12;
  return 0;
}



// Function: rpl_vfprintf @ 0x10a00
long long rpl_vfprintf(FILE *s, void *a2, long long a3, double a4)
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
  v4 = sub_F900(v9, &n, a2, a3, a4);
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
    sub_10B90(s);
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



// Function: opendir_safer @ 0x10af0
DIR *opendir_safer(const char *a1)
{
  DIR *v1; // rax
  DIR *v2; // r12
  unsigned int v3; // eax
  long long v4; // rcx
  long long v5; // r8
  long long v6; // r9
  int v8; // ebp
  int *v9; // rax
  int *v10; // rbx
  int v11; // r14d
  DIR *v12; // r13
  DIR *v13; // rax

  v1 = opendir(a1);
  v2 = v1;
  if ( !v1 )
    return v2;
  v3 = dirfd(v1);
  if ( v3 > 2 )
    return v2;
  v8 = sub_E0F0(v3, 1030, 3, v4, v5, v6);
  v9 = __errno_location();
  v10 = v9;
  if ( v8 >= 0 )
  {
    v13 = fdopendir(v8);
    v11 = *v10;
    v12 = v13;
    if ( !v13 )
      close(v8);
  }
  else
  {
    v11 = *v9;
    v12 = 0;
  }
  closedir(v2);
  *v10 = v11;
  return v12;
}



// Function: fseterr @ 0x10b90
void fseterr(int *a1)
{
  *a1 |= 0x20u;
}



// Function: current_timespec @ 0x10bb0
__time_t current_timespec()
{
  struct timespec v1; // [rsp+0h] [rbp-28h] BYREF
  unsigned long long v2; // [rsp+18h] [rbp-10h]

  v2 = __readfsqword(0x28u);
  clock_gettime(0, &v1);
  return v1.tv_sec;
}



// Function: dup_safer @ 0x10c00
long long dup_safer(int a1, long long a2, long long a3, long long a4, long long a5, long long a6)
{
  return sub_E0F0(a1, 0, 3, a4, a5, a6);
}



// Function: printf_fetchargs @ 0x10c20
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



// Function: printf_parse @ 0x10e50
long long printf_parse(char *a1, long long *a2, unsigned long long *a3)
{
  void *v3; // r9
  char *v4; // r10
  long long *v8; // rcx
  unsigned long long v9; // r8
  long long v10; // r11
  long long v11; // rsi
  char *v12; // rbx
  unsigned long long v14; // r13
  char **v15; // r12
  char v16; // bp
  char *j; // rdx
  int v18; // edx
  int v19; // eax
  long long v20; // rax
  char v21; // cl
  unsigned long long v22; // rbp
  char *v23; // rdi
  char *v24; // rax
  unsigned long long v25; // r8
  char *v26; // rax
  unsigned long long v27; // rdx
  char *v28; // rax
  char *v29; // rcx
  char *v30; // rbp
  long long v31; // rax
  char *v32; // rcx
  char v33; // si
  unsigned long long v34; // rbx
  unsigned long long v35; // rdx
  char *v36; // rax
  char *v37; // rsi
  char *v38; // rax
  int v39; // ecx
  unsigned long long v40; // rdx
  char *v41; // rax
  char *v42; // rsi
  int *v43; // r13
  long long v44; // rax
  long long v45; // rbp
  void *v46; // rdi
  long long *v47; // rax
  char *v48; // rdx
  char v49; // cl
  char *v50; // rax
  unsigned long long v51; // rdi
  char *v52; // rcx
  long long v53; // rdx
  long long i; // rsi
  bool v55; // cf
  unsigned long long v56; // rsi
  void *v57; // rdi
  char *v58; // rdx
  char v59; // al
  unsigned long long v60; // rax
  char *v61; // rax
  char v62; // si
  char *v63; // rbx
  unsigned long long v64; // rsi
  char *v65; // rdx
  long long v66; // rax
  long long k; // rcx
  long long v68; // rbp
  void *v69; // rdi
  unsigned long long v70; // r8
  char *v71; // rax
  char *v72; // rdx
  char v73; // al
  unsigned long long v74; // rax
  char *v75; // rax
  unsigned long long v76; // r8
  char *v77; // rax
  char *v78; // rax
  long long *v79; // rax
  char *v80; // rax
  char v81; // dl
  char *v82; // rbx
  unsigned long long v83; // rdi
  char *v84; // rcx
  long long v85; // rax
  long long m; // rdx
  unsigned long long v87; // rdx
  char *v88; // rax
  char *v89; // rax
  char *v90; // rax
  unsigned long long v91; // [rsp+0h] [rbp-78h]
  void *v92; // [rsp+0h] [rbp-78h]
  void *v93; // [rsp+0h] [rbp-78h]
  unsigned long long v94; // [rsp+8h] [rbp-70h]
  unsigned long long v95; // [rsp+10h] [rbp-68h]
  void *v96; // [rsp+18h] [rbp-60h]
  void *v97; // [rsp+18h] [rbp-60h]
  int v98; // [rsp+18h] [rbp-60h]
  char *v99; // [rsp+18h] [rbp-60h]
  char *v100; // [rsp+18h] [rbp-60h]
  void *v101; // [rsp+18h] [rbp-60h]
  char *v102; // [rsp+18h] [rbp-60h]
  char *v103; // [rsp+18h] [rbp-60h]
  char *v104; // [rsp+20h] [rbp-58h]
  char *v105; // [rsp+20h] [rbp-58h]
  void *v106; // [rsp+20h] [rbp-58h]
  void *v107; // [rsp+20h] [rbp-58h]
  int v108; // [rsp+20h] [rbp-58h]
  char *v109; // [rsp+20h] [rbp-58h]
  void *v110; // [rsp+20h] [rbp-58h]
  char *v111; // [rsp+20h] [rbp-58h]
  void *src; // [rsp+28h] [rbp-50h]
  void *srca; // [rsp+28h] [rbp-50h]
  char *srcb; // [rsp+28h] [rbp-50h]
  char *srcc; // [rsp+28h] [rbp-50h]
  void *srcf; // [rsp+28h] [rbp-50h]
  void *srce; // [rsp+28h] [rbp-50h]
  void *srcd; // [rsp+28h] [rbp-50h]
  void *srcg; // [rsp+28h] [rbp-50h]
  unsigned long long v120; // [rsp+30h] [rbp-48h]
  long long v121; // [rsp+30h] [rbp-48h]
  long long v122; // [rsp+30h] [rbp-48h]
  long long v123; // [rsp+30h] [rbp-48h]
  unsigned long long v124; // [rsp+30h] [rbp-48h]
  long long v125; // [rsp+30h] [rbp-48h]
  unsigned long long v126; // [rsp+38h] [rbp-40h]
  unsigned long long v127; // [rsp+38h] [rbp-40h]
  unsigned long long v128; // [rsp+38h] [rbp-40h]
  unsigned long long v129; // [rsp+38h] [rbp-40h]

  v3 = a2 + 4;
  v4 = (char *)(a3 + 2);
  v8 = a2 + 4;
  v9 = 7;
  v10 = 7;
  *a2 = 0;
  a2[1] = (long long)(a2 + 4);
  v11 = 0;
  *a3 = 0;
  a3[1] = (unsigned long long)(a3 + 2);
  v94 = 0;
  v91 = 0;
  v95 = 0;
  while ( 1 )
  {
    if ( !*a1 )
    {
      v8[11 * v11] = a1;
      a2[2] = v91;
      a2[3] = v94;
      return 0;
    }
    v12 = a1 + 1;
    if ( *a1 == 37 )
      break;
LABEL_3:
    a1 = v12;
  }
  v14 = -1;
  v15 = (char **)&v8[11 * v11];
  *v15 = a1;
  *((int *)v15 + 4) = 0;
  v15[3] = 0;
  v15[4] = 0;
  v15[5] = (char *)-1LL;
  v15[6] = 0;
  v15[7] = 0;
  v15[8] = (char *)-1LL;
  v15[10] = (char *)-1LL;
  v16 = a1[1];
  if ( (unsigned char)(v16 - 48) <= 9u )
  {
    v48 = a1 + 1;
    do
      v49 = *++v48;
    while ( (unsigned char)(v49 - 48) <= 9u );
    v14 = -1;
    if ( v49 == 36 )
    {
      v50 = a1 + 2;
      v51 = 0;
      while ( 1 )
      {
        v52 = v50 - 1;
        v53 = (char)(v16 - 48);
        for ( i = v51 > 0x1999999999999999LL ? -1LL : 10 * v51; ; i = -1 )
        {
          v16 = *v50;
          v55 = __CFADD__(v53, i);
          v56 = v53 + i;
          v51 = v56;
          LOBYTE(v53) = *v50 - 48;
          if ( !v55 )
            break;
          if ( (unsigned char)v53 > 9u )
            goto LABEL_88;
          v52 = v50;
          v53 = (char)v53;
          ++v50;
        }
        if ( (unsigned char)v53 > 9u )
          break;
        ++v50;
      }
      v14 = v56 - 1;
      if ( v56 - 1 > 0xFFFFFFFFFFFFFFFDLL )
      {
LABEL_88:
        v23 = (char *)a3[1];
LABEL_89:
        if ( v4 != v23 )
        {
          v92 = v3;
          free(v23);
          v3 = v92;
        }
        v57 = (void *)a2[1];
        if ( v3 != v57 )
          free(v57);
        *__errno_location() = 22;
        return 0xFFFFFFFFLL;
      }
      v16 = v52[2];
      v12 = v52 + 2;
    }
  }
  for ( j = v12 + 1; v16 == 39; ++j )
  {
    v19 = *((int *)v15 + 4) | 1;
LABEL_15:
    *((int *)v15 + 4) = v19;
    v16 = *j;
    v12 = j;
  }
  switch ( v16 )
  {
    case ' ':
      v19 = *((int *)v15 + 4) | 8;
      goto LABEL_15;
    case '#':
      v19 = *((int *)v15 + 4) | 0x10;
      goto LABEL_15;
    case '+':
      v19 = *((int *)v15 + 4) | 4;
      goto LABEL_15;
    case '-':
      v19 = *((int *)v15 + 4) | 2;
      goto LABEL_15;
    case '0':
      v19 = *((int *)v15 + 4) | 0x20;
      goto LABEL_15;
    case 'I':
      v19 = *((int *)v15 + 4) | 0x40;
      goto LABEL_15;
    default:
      if ( v16 != 42 )
      {
        if ( (unsigned char)(v16 - 48) <= 9u )
        {
          v15[3] = v12;
          if ( (unsigned char)(*v12 - 48) <= 9u )
          {
            v72 = v12;
            do
              v73 = *++v72;
            while ( (unsigned char)(v73 - 48) <= 9u );
            v74 = v72 - v12;
            v12 = v72;
            if ( v91 >= v74 )
              v74 = v91;
            v91 = v74;
          }
          v15[4] = v12;
          goto LABEL_157;
        }
        goto LABEL_11;
      }
      v20 = 1;
      v15[3] = v12;
      v15[4] = j;
      v21 = v12[1];
      if ( v91 )
        v20 = v91;
      v91 = v20;
      if ( (unsigned char)(v21 - 48) > 9u )
        goto LABEL_29;
      v61 = j;
      do
        v62 = *++v61;
      while ( (unsigned char)(v62 - 48) <= 9u );
      if ( v62 == 36 )
      {
        v63 = v12 + 2;
        v64 = 0;
        while ( 1 )
        {
          v65 = v63 - 1;
          v66 = (char)(v21 - 48);
          for ( k = v64 > 0x1999999999999999LL ? -1LL : 10 * v64; ; k = -1 )
          {
            v55 = __CFADD__(v66, k);
            v68 = v66 + k;
            v64 = v66 + k;
            v21 = *v63;
            LOBYTE(v66) = *v63 - 48;
            if ( !v55 )
              break;
            if ( (unsigned char)v66 > 9u )
              goto LABEL_88;
            v65 = v63;
            v66 = (char)v66;
            ++v63;
          }
          if ( (unsigned char)v66 > 9u )
            break;
          ++v63;
        }
        v22 = v68 - 1;
        if ( v22 > 0xFFFFFFFFFFFFFFFDLL )
          goto LABEL_88;
        v15[5] = (char *)v22;
        v12 = v65 + 2;
      }
      else
      {
LABEL_29:
        v15[5] = (char *)v95;
        if ( v95 == -1 )
          goto LABEL_88;
        v22 = v95++;
        v12 = j;
      }
      v23 = (char *)a3[1];
      v24 = v23;
      if ( v9 <= v22 )
      {
        v25 = 2 * v9;
        if ( v25 <= v22 )
          v25 = v22 + 1;
        if ( v25 >> 59 )
          goto LABEL_132;
        v120 = v25;
        src = (void *)v10;
        v104 = v4;
        v96 = v3;
        if ( v4 == v23 )
        {
          v26 = (char *)malloc(32 * v25);
          v9 = v120;
          v10 = (long long)src;
          v4 = v104;
          v3 = v96;
        }
        else
        {
          v26 = (char *)realloc(v23, 32 * v25);
          v3 = v96;
          v4 = v104;
          v10 = (long long)src;
          v9 = v120;
        }
        v23 = v26;
        v24 = (char *)a3[1];
        if ( !v23 )
          goto LABEL_132;
        if ( v4 == v24 )
        {
          v124 = v9;
          srcd = (void *)v10;
          v110 = v3;
          v102 = v4;
          v88 = (char *)memcpy(v23, v4, 32 * *a3);
          v9 = v124;
          v10 = (long long)srcd;
          v3 = v110;
          v4 = v102;
          v23 = v88;
        }
        a3[1] = (unsigned long long)v23;
      }
      v27 = *a3;
      v28 = &v23[32 * *a3];
      if ( *a3 <= v22 )
      {
        do
        {
          ++v27;
          *(int *)v28 = 0;
          v29 = v28;
          v28 += 32;
        }
        while ( v27 <= v22 );
        *a3 = v27;
        *(int *)v29 = 0;
      }
      v30 = &v23[32 * v22];
      if ( *(int *)v30 )
      {
        if ( *(int *)v30 != 5 )
          goto LABEL_89;
LABEL_157:
        v16 = *v12;
LABEL_11:
        if ( v16 != 46 )
          goto LABEL_12;
        goto LABEL_46;
      }
      *(int *)v30 = 5;
      v16 = *v12;
      if ( *v12 != 46 )
        goto LABEL_12;
LABEL_46:
      if ( v12[1] == 42 )
      {
        v31 = 2;
        v32 = v12 + 2;
        v15[6] = v12;
        v15[7] = v12 + 2;
        v33 = v12[2];
        if ( v94 >= 2 )
          v31 = v94;
        v94 = v31;
        if ( (unsigned char)(v33 - 48) > 9u )
          goto LABEL_50;
        v80 = v12 + 2;
        do
          v81 = *++v80;
        while ( (unsigned char)(v81 - 48) <= 9u );
        if ( v81 == 36 )
        {
          v82 = v12 + 3;
          v83 = 0;
          while ( 1 )
          {
            v84 = v82 - 1;
            v85 = (char)(v33 - 48);
            for ( m = v83 > 0x1999999999999999LL ? -1LL : 10 * v83; ; m = -1 )
            {
              v33 = *v82;
              v55 = __CFADD__(v85, m);
              v87 = v85 + m;
              v83 = v87;
              LOBYTE(v85) = *v82 - 48;
              if ( !v55 )
                break;
              if ( (unsigned char)v85 > 9u )
                goto LABEL_88;
              v84 = v82;
              v85 = (char)v85;
              ++v82;
            }
            if ( (unsigned char)v85 > 9u )
              break;
            ++v82;
          }
          v34 = v87 - 1;
          if ( v87 - 1 > 0xFFFFFFFFFFFFFFFDLL )
            goto LABEL_88;
          v15[8] = (char *)v34;
          v32 = v84 + 2;
        }
        else
        {
LABEL_50:
          v34 = (unsigned long long)v15[8];
          if ( v34 == -1 )
          {
            v15[8] = (char *)v95;
            if ( v95 == -1 )
              goto LABEL_88;
            v34 = v95++;
          }
        }
        v23 = (char *)a3[1];
        v24 = v23;
        if ( v9 <= v34 )
        {
          v76 = 2 * v9;
          if ( v76 <= v34 )
            v76 = v34 + 1;
          if ( v76 >> 59 )
            goto LABEL_132;
          v127 = v76;
          v122 = v10;
          srcc = v4;
          v107 = v3;
          v99 = v32;
          if ( v4 == v23 )
          {
            v89 = (char *)malloc(32 * v76);
            v9 = v127;
            v10 = v122;
            v4 = srcc;
            v3 = v107;
            v23 = v89;
            v32 = v99;
          }
          else
          {
            v77 = (char *)realloc(v23, 32 * v76);
            v32 = v99;
            v3 = v107;
            v4 = srcc;
            v10 = v122;
            v23 = v77;
            v9 = v127;
          }
          v24 = (char *)a3[1];
          if ( !v23 )
            goto LABEL_132;
          if ( v4 == v24 )
          {
            v129 = v9;
            v125 = v10;
            srcg = v3;
            v111 = v32;
            v103 = v4;
            v90 = (char *)memcpy(v23, v4, 32 * *a3);
            v9 = v129;
            v10 = v125;
            v3 = srcg;
            v32 = v111;
            v23 = v90;
            v4 = v103;
          }
          a3[1] = (unsigned long long)v23;
        }
        v35 = *a3;
        v36 = &v23[32 * *a3];
        if ( *a3 <= v34 )
        {
          do
          {
            ++v35;
            *(int *)v36 = 0;
            v37 = v36;
            v36 += 32;
          }
          while ( v35 <= v34 );
          *a3 = v35;
          *(int *)v37 = 0;
        }
        v38 = &v23[32 * v34];
        if ( *(int *)v38 )
        {
          if ( *(int *)v38 != 5 )
            goto LABEL_89;
          v16 = *v32;
          v12 = v32;
        }
        else
        {
          *(int *)v38 = 5;
          v12 = v32;
          v16 = *v32;
        }
      }
      else
      {
        v15[6] = v12;
        v58 = v12 + 1;
        if ( (unsigned char)(v12[1] - 48) > 9u )
        {
          ++v12;
          v60 = 1;
        }
        else
        {
          do
            v59 = *++v58;
          while ( (unsigned char)(v59 - 48) <= 9u );
          v60 = v58 - v12;
          v12 = v58;
        }
        v15[7] = v58;
        v16 = *v58;
        if ( v94 >= v60 )
          v60 = v94;
        v94 = v60;
      }
LABEL_12:
      v18 = 0;
      ++v12;
      for ( ; v16 == 104; ++v12 )
      {
        v18 |= 1 << (v18 & 1);
LABEL_24:
        v16 = *v12;
      }
      break;
  }
  switch ( v16 )
  {
    case 'L':
      v18 |= 4u;
      goto LABEL_24;
    case 'Z':
    case 'j':
    case 'l':
    case 't':
    case 'z':
      v18 += 8;
      goto LABEL_24;
    default:
      switch ( v16 )
      {
        case '%':
          goto LABEL_67;
        case 'A':
        case 'E':
        case 'F':
        case 'G':
        case 'a':
        case 'e':
        case 'f':
        case 'g':
          v39 = 12;
          if ( v18 <= 15 )
            v39 = ((v18 & 4) != 0) + 11;
          goto LABEL_59;
        case 'C':
          v39 = 14;
          v16 = 99;
          goto LABEL_59;
        case 'S':
          v39 = 16;
          v16 = 115;
          goto LABEL_59;
        case 'X':
        case 'o':
        case 'u':
        case 'x':
          v39 = 10;
          if ( v18 <= 15 && (v18 & 4) == 0 )
          {
            v39 = 8;
            if ( v18 <= 7 )
            {
              v39 = 2;
              if ( (v18 & 2) == 0 )
                v39 = (v18 & 1) == 0 ? 6 : 4;
            }
          }
          goto LABEL_59;
        case 'c':
          v39 = (v18 > 7) + 13;
          goto LABEL_59;
        case 'd':
        case 'i':
          v39 = 9;
          if ( v18 <= 15 && (v18 & 4) == 0 )
          {
            v39 = 7;
            if ( v18 <= 7 )
            {
              v39 = 1;
              if ( (v18 & 2) == 0 )
                v39 = (v18 & 1) == 0 ? 5 : 3;
            }
          }
          goto LABEL_59;
        case 'n':
          v39 = 22;
          if ( v18 <= 15 && (v18 & 4) == 0 )
          {
            v39 = 21;
            if ( v18 <= 7 )
            {
              v39 = 18;
              if ( (v18 & 2) == 0 )
                v39 = 20 - (v18 & 1);
            }
          }
          goto LABEL_59;
        case 'p':
          v39 = 17;
          goto LABEL_59;
        case 's':
          v39 = (v18 > 7) + 15;
LABEL_59:
          if ( v14 == -1 )
          {
            v15[10] = (char *)v95;
            if ( v95 == -1 )
              goto LABEL_88;
            v14 = v95++;
          }
          else
          {
            v15[10] = (char *)v14;
          }
          v23 = (char *)a3[1];
          v24 = v23;
          if ( v9 > v14 )
            goto LABEL_62;
          v70 = 2 * v9;
          if ( v70 <= v14 )
            v70 = v14 + 1;
          if ( v70 >> 59 )
            goto LABEL_132;
          v126 = v70;
          v121 = v10;
          srcb = v4;
          v106 = v3;
          v98 = v39;
          if ( v4 == v23 )
          {
            v75 = (char *)malloc(32 * v70);
            v9 = v126;
            v10 = v121;
            v4 = srcb;
            v3 = v106;
            v23 = v75;
            v39 = v98;
          }
          else
          {
            v71 = (char *)realloc(v23, 32 * v70);
            v39 = v98;
            v3 = v106;
            v4 = srcb;
            v10 = v121;
            v23 = v71;
            v9 = v126;
          }
          v24 = (char *)a3[1];
          if ( !v23 )
            goto LABEL_132;
          if ( v4 == v24 )
          {
            v128 = v9;
            v123 = v10;
            srcf = v3;
            v108 = v39;
            v100 = v4;
            v78 = (char *)memcpy(v23, v4, 32 * *a3);
            v9 = v128;
            v10 = v123;
            v3 = srcf;
            v39 = v108;
            v23 = v78;
            v4 = v100;
          }
          a3[1] = (unsigned long long)v23;
LABEL_62:
          v40 = *a3;
          v41 = &v23[32 * *a3];
          if ( *a3 <= v14 )
          {
            do
            {
              ++v40;
              *(int *)v41 = 0;
              v42 = v41;
              v41 += 32;
            }
            while ( v40 <= v14 );
            *a3 = v40;
            *(int *)v42 = 0;
          }
          v43 = (int *)&v23[32 * v14];
          if ( *v43 )
          {
            if ( *v43 != v39 )
              goto LABEL_89;
          }
          else
          {
            *v43 = v39;
          }
LABEL_67:
          *((char *)v15 + 72) = v16;
          v44 = *a2;
          v15[1] = v12;
          v11 = v44 + 1;
          *a2 = v44 + 1;
          if ( v10 > (unsigned long long)(v44 + 1) )
          {
            v8 = (long long *)a2[1];
            goto LABEL_3;
          }
          if ( v10 >= 0 )
          {
            v45 = 2 * v10;
            if ( (unsigned long long)(2 * v10) <= 0x2E8BA2E8BA2E8BALL )
            {
              v46 = (void *)a2[1];
              srca = (void *)v9;
              v105 = v4;
              v97 = v3;
              if ( v3 == v46 )
              {
                v47 = malloc(176 * v10);
                v9 = (unsigned long long)srca;
                v4 = v105;
                v3 = v97;
              }
              else
              {
                v47 = realloc(v46, 176 * v10);
                v3 = v97;
                v4 = v105;
                v9 = (unsigned long long)srca;
              }
              v8 = v47;
              if ( v47 )
              {
                if ( v3 == (void *)a2[1] )
                {
                  srce = (void *)v9;
                  v109 = v4;
                  v101 = v3;
                  v79 = memcpy(v47, v3, 88 * *a2);
                  v9 = (unsigned long long)srce;
                  v4 = v109;
                  v3 = v101;
                  v8 = v79;
                }
                a2[1] = (long long)v8;
                v11 = *a2;
                v10 = v45;
                goto LABEL_3;
              }
            }
          }
          v24 = (char *)a3[1];
LABEL_132:
          if ( v4 != v24 )
          {
            v93 = v3;
            free(v24);
            v3 = v93;
          }
          v69 = (void *)a2[1];
          if ( v3 != v69 )
            free(v69);
          *__errno_location() = 12;
          return 0xFFFFFFFFLL;
        default:
          goto LABEL_88;
      }
  }
}


