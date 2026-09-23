// Function: bi_init @ 0x3a29
long long (*bi_init(unsigned int a1))()
{
  long long (*result)(); // rax

  dword_1A058 = a1;
  word_1A05C = 0;
  dword_1A060 = 0;
  result = (long long (*)())a1;
  if ( a1 != -1 )
  {
    qword_1A050 = (long long)sub_101D7;
    return sub_101D7;
  }
  return result;
}



// Function: send_bits @ 0x3a6c
long long send_bits(int a1, int a2)
{
  unsigned int v2; // eax
  unsigned int v3; // eax
  unsigned int v4; // eax
  unsigned int v5; // eax
  long long result; // rax

  if ( 16 - a2 >= dword_1A060 )
  {
    word_1A05C |= a1 << dword_1A060;
    result = (unsigned int)(dword_1A060 + a2);
    dword_1A060 += a2;
  }
  else
  {
    word_1A05C |= a1 << dword_1A060;
    if ( dword_DD870 > (unsigned int)&unk_3FFFD )
    {
      v4 = dword_DD870++;
      byte_5C000[v4] = word_1A05C;
      if ( dword_DD870 == (int)&unk_40000 )
        sub_F28E();
      v5 = dword_DD870++;
      byte_5C000[v5] = HIBYTE(word_1A05C);
      if ( dword_DD870 == (int)&unk_40000 )
        sub_F28E();
    }
    else
    {
      v2 = dword_DD870++;
      byte_5C000[v2] = word_1A05C;
      v3 = dword_DD870++;
      byte_5C000[v3] = HIBYTE(word_1A05C);
    }
    word_1A05C = (int)(unsigned short)a1 >> (16 - dword_1A060);
    result = (unsigned int)(dword_1A060 + a2 - 16);
    dword_1A060 = dword_1A060 + a2 - 16;
  }
  return result;
}



// Function: bi_reverse @ 0x3bed
long long bi_reverse(unsigned int a1, int a2)
{
  unsigned int v2; // ebx
  int v3; // eax

  v2 = 0;
  do
  {
    v3 = a1 & 1;
    a1 >>= 1;
    v2 = 2 * (v3 | v2);
    --a2;
  }
  while ( a2 > 0 );
  return v2 >> 1;
}



// Function: bi_windup @ 0x3c22
long long bi_windup()
{
  int v0; // eax
  unsigned int v1; // eax
  long long v2; // rdx
  long long result; // rax
  int v4; // eax
  int v5; // eax
  int v6; // eax

  if ( dword_1A060 <= 8 )
  {
    result = (unsigned int)dword_1A060;
    if ( dword_1A060 > 0 )
    {
      v6 = dword_DD870++;
      byte_5C000[v6] = word_1A05C;
      result = (unsigned int)dword_DD870;
      if ( dword_DD870 == (int)&unk_40000 )
        goto LABEL_10;
    }
  }
  else
  {
    if ( dword_DD870 <= (unsigned int)&unk_3FFFD )
    {
      v0 = dword_DD870++;
      byte_5C000[v0] = word_1A05C;
      v1 = dword_DD870++;
      v2 = v1;
      result = (long long)byte_5C000;
      byte_5C000[v2] = HIBYTE(word_1A05C);
      goto LABEL_11;
    }
    v4 = dword_DD870++;
    byte_5C000[v4] = word_1A05C;
    if ( dword_DD870 == (int)&unk_40000 )
      sub_F28E();
    v5 = dword_DD870++;
    byte_5C000[v5] = HIBYTE(word_1A05C);
    result = (unsigned int)dword_DD870;
    if ( dword_DD870 == (int)&unk_40000 )
LABEL_10:
      result = sub_F28E();
  }
LABEL_11:
  word_1A05C = 0;
  dword_1A060 = 0;
  return result;
}



// Function: copy_block @ 0x3d59
long long copy_block(char *a1, unsigned int a2, int a3)
{
  int v3; // eax
  int v4; // eax
  int v5; // eax
  int v6; // eax
  int v7; // eax
  int v8; // eax
  int v9; // eax
  int v10; // eax
  char *v11; // rax
  char v12; // cl
  long long result; // rax
  unsigned int v15; // [rsp+4h] [rbp-Ch]

  v15 = a2;
  sub_3C22();
  if ( a3 )
  {
    if ( dword_DD870 > (unsigned int)&unk_3FFFD )
    {
      v5 = dword_DD870++;
      byte_5C000[v5] = a2;
      if ( dword_DD870 == (int)&unk_40000 )
        sub_F28E();
      v6 = dword_DD870++;
      byte_5C000[v6] = BYTE1(a2);
      if ( dword_DD870 == (int)&unk_40000 )
        sub_F28E();
    }
    else
    {
      v3 = dword_DD870++;
      byte_5C000[v3] = a2;
      v4 = dword_DD870++;
      byte_5C000[v4] = BYTE1(a2);
    }
    if ( dword_DD870 > (unsigned int)&unk_3FFFD )
    {
      v9 = dword_DD870++;
      byte_5C000[v9] = ~(char)a2;
      if ( dword_DD870 == (int)&unk_40000 )
        sub_F28E();
      v10 = dword_DD870++;
      byte_5C000[v10] = (unsigned short)~(short)a2 >> 8;
      if ( dword_DD870 == (int)&unk_40000 )
        goto LABEL_15;
    }
    else
    {
      v7 = dword_DD870++;
      byte_5C000[v7] = ~(char)a2;
      v8 = dword_DD870++;
      byte_5C000[v8] = (unsigned short)~(short)a2 >> 8;
    }
  }
  while ( 1 )
  {
    result = v15--;
    if ( !(int)result )
      break;
    v11 = a1++;
    v12 = *v11;
    LODWORD(v11) = dword_DD870++;
    byte_5C000[(unsigned int)v11] = v12;
    if ( dword_DD870 == (int)&unk_40000 )
LABEL_15:
      sub_F28E();
  }
  return result;
}



// Function: lm_init @ 0x3f52
long long lm_init(int a1)
{
  long long result; // rax
  unsigned int i; // ebx

  if ( a1 <= 0 || a1 > 9 )
    sub_F5EF("bad pack level");
  memset(&unk_CD000, 0, 0x10000u);
  qword_1A0A0 = 0xFFFFFFFFLL;
  qword_1A098 = 0;
  dword_1A090 = *((unsigned short *)&unk_19042 + 4 * a1);
  dword_1A080 = *((unsigned short *)&unk_19040 + 4 * a1);
  dword_1A0A8 = *((unsigned short *)&unk_19044 + 4 * a1);
  dword_1A07C = word_19046[4 * a1];
  dword_1A074 = 0;
  qword_1A068 = 0;
  dword_1A08C = qword_1A050(byte_AD000, 0x10000);
  result = (unsigned int)dword_1A08C;
  if ( dword_1A08C && (result = (unsigned int)dword_1A08C, dword_1A08C != -1) )
  {
    dword_1A088 = 0;
    while ( 1 )
    {
      result = (unsigned int)dword_1A08C;
      if ( (unsigned int)dword_1A08C > 0x105 )
        break;
      result = (unsigned int)dword_1A088;
      if ( dword_1A088 )
        break;
      sub_4350();
    }
    dword_1A084 = 0;
    for ( i = 0; i <= 1; ++i )
    {
      result = ((unsigned short)(32 * dword_1A084) ^ byte_AD000[i]) & 0x7FFF;
      dword_1A084 = ((unsigned short)(32 * dword_1A084) ^ byte_AD000[i]) & 0x7FFF;
    }
  }
  else
  {
    dword_1A088 = 1;
    dword_1A08C = 0;
  }
  return result;
}



// Function: longest_match @ 0x40fe
long long longest_match(unsigned int a1)
{
  unsigned char *v1; // rbx
  int v2; // eax
  unsigned char *v3; // r13
  unsigned char v4; // r14
  unsigned char v5; // r15
  unsigned char *v6; // r12
  char *v7; // r12
  unsigned char *v8; // rbx
  char *v9; // r12
  char *v10; // r12
  char *v11; // r12
  char *v12; // r12
  char *v13; // r12
  char *v14; // r12
  char *v15; // r12
  char *v16; // r12
  int v17; // r12d
  unsigned int v20; // [rsp+8h] [rbp-34h]
  int v21; // [rsp+Ch] [rbp-30h]

  v20 = dword_1A07C;
  v1 = &byte_AD000[dword_1A074];
  v21 = dword_1A070;
  v2 = dword_1A074;
  if ( (unsigned int)dword_1A074 < 0x7EFA )
    v2 = 32506;
  v3 = &byte_AD000[dword_1A074 + 258];
  v4 = v1[dword_1A070 - 1];
  v5 = v1[dword_1A070];
  if ( dword_1A070 >= (unsigned int)dword_1A080 )
    v20 = (unsigned int)dword_1A07C >> 2;
  do
  {
    v6 = &byte_AD000[a1];
    if ( v5 == v6[v21] && v4 == v6[v21 - 1] && *v6 == *v1 )
    {
      v7 = v6 + 1;
      if ( *v7 == v1[1] )
      {
        v8 = v1 + 2;
        v9 = v7 + 1;
        do
        {
          ++v8;
          v10 = v9 + 1;
          if ( *v8 != *v10 )
            break;
          ++v8;
          v11 = v10 + 1;
          if ( *v8 != *v11 )
            break;
          ++v8;
          v12 = v11 + 1;
          if ( *v8 != *v12 )
            break;
          ++v8;
          v13 = v12 + 1;
          if ( *v8 != *v13 )
            break;
          ++v8;
          v14 = v13 + 1;
          if ( *v8 != *v14 )
            break;
          ++v8;
          v15 = v14 + 1;
          if ( *v8 != *v15 )
            break;
          ++v8;
          v16 = v15 + 1;
          if ( *v8 != *v16 )
            break;
          ++v8;
          v9 = v16 + 1;
          if ( *v8 != *v9 )
            break;
        }
        while ( v8 < v3 );
        v17 = 258 - ((int)v3 - (int)v8);
        v1 = &byte_AD000[dword_1A074];
        if ( v17 > v21 )
        {
          dword_1A078 = a1;
          v21 = v17;
          if ( v17 >= dword_1A0A8 )
            return (unsigned int)v21;
          v4 = v1[v17 - 1];
          v5 = v1[v17];
        }
      }
    }
    a1 = word_BD000[a1 & 0x7FFF];
    if ( a1 <= v2 - 32506 )
      break;
    --v20;
  }
  while ( v20 );
  return (unsigned int)v21;
}



// Function: fill_window @ 0x4350
unsigned long long fill_window()
{
  unsigned int i; // ebx
  unsigned int v1; // r12d
  unsigned short v2; // dx
  unsigned int j; // ebx
  unsigned int v4; // r12d
  unsigned short v5; // cx
  unsigned long long result; // rax
  int v7; // ebx
  unsigned int v8; // [rsp+Ch] [rbp-14h]

  v8 = (int)off_19020 - dword_1A08C - dword_1A074;
  if ( v8 == -1 )
  {
    v8 = -2;
  }
  else if ( (unsigned int)dword_1A074 > 0xFEF9 )
  {
    memcpy(byte_AD000, &unk_B5000, sizeof(byte_AD000));
    dword_1A078 -= 0x8000;
    dword_1A074 -= 0x8000;
    if ( qword_1A0A0 != 0xFFFFFFFFLL )
      qword_1A0A0 -= 0x8000;
    qword_1A068 -= 0x8000;
    for ( i = 0; i <= 0x7FFF; ++i )
    {
      v1 = word_BD000[i + 0x8000];
      if ( v1 <= 0x7FFF )
        v2 = 0;
      else
        v2 = v1 + 0x8000;
      word_BD000[i + 0x8000] = v2;
    }
    for ( j = 0; j <= 0x7FFF; ++j )
    {
      v4 = word_BD000[j];
      if ( v4 <= 0x7FFF )
        v5 = 0;
      else
        v5 = v4 + 0x8000;
      word_BD000[j] = v5;
    }
    v8 += 0x8000;
  }
  result = (unsigned int)dword_1A088;
  if ( !dword_1A088 )
  {
    v7 = qword_1A050(&byte_AD000[(unsigned int)dword_1A08C + (unsigned long long)(unsigned int)dword_1A074], v8);
    if ( v7 && v7 != -1 )
    {
      result = (unsigned int)(v7 + dword_1A08C);
      dword_1A08C += v7;
    }
    else
    {
      dword_1A088 = 1;
      return (unsigned long long)memset(
                                 &byte_AD000[(unsigned int)dword_1A08C + (unsigned long long)(unsigned int)dword_1A074],
                                 0,
                                 2u);
    }
  }
  return result;
}



// Function: rsync_roll @ 0x4567
long long rsync_roll(unsigned int a1, int a2)
{
  long long result; // rax
  int v3; // [rsp+0h] [rbp-18h]
  int v4; // [rsp+4h] [rbp-14h]
  unsigned int i; // [rsp+14h] [rbp-4h]
  unsigned int j; // [rsp+14h] [rbp-4h]

  v4 = a1;
  v3 = a2;
  if ( a1 <= 0xFFF )
  {
    for ( i = a1; i <= 0xFFF; ++i )
    {
      result = a1 + a2;
      if ( i == (int)result )
        return result;
      qword_1A098 += byte_AD000[i];
    }
    v3 = a1 + a2 - 4096;
    v4 = 4096;
  }
  for ( j = v4; ; ++j )
  {
    result = (unsigned int)(v4 + v3);
    if ( j >= (unsigned int)result )
      break;
    qword_1A098 += byte_AD000[j];
    qword_1A098 -= byte_AD000[j - 4096];
    if ( qword_1A0A0 == 0xFFFFFFFFLL && (qword_1A098 & 0xFFF) == 0 )
      qword_1A0A0 = j;
  }
  return result;
}



// Function: deflate_fast @ 0x4677
long long deflate_fast()
{
  unsigned char *v0; // rax
  unsigned char *v1; // rax
  int v3; // [rsp+4h] [rbp-Ch]
  unsigned int v4; // [rsp+8h] [rbp-8h]
  unsigned int v5; // [rsp+Ch] [rbp-4h]

  v3 = 0;
  v4 = 0;
  dword_1A070 = 2;
  while ( dword_1A08C )
  {
    dword_1A084 = ((unsigned short)(32 * dword_1A084) ^ byte_AD000[dword_1A074 + 2]) & 0x7FFF;
    v5 = word_BD000[dword_1A084 + 0x8000];
    word_BD000[dword_1A074 & 0x7FFF] = v5;
    word_BD000[dword_1A084 + 0x8000] = dword_1A074;
    if ( v5 )
    {
      if ( dword_1A074 - v5 <= 0x7EFA && (unsigned int)dword_1A074 <= (unsigned long long)off_19020 - 262 )
      {
        v4 = sub_40FE(v5);
        if ( v4 > dword_1A08C )
          v4 = dword_1A08C;
      }
    }
    if ( v4 <= 2 )
    {
      v3 = sub_C45D(0, byte_AD000[dword_1A074]);
      if ( dword_DD874 )
        sub_4567(dword_1A074, 1);
      --dword_1A08C;
      ++dword_1A074;
    }
    else
    {
      v3 = sub_C45D((unsigned int)(dword_1A074 - dword_1A078), v4 - 3);
      dword_1A08C -= v4;
      if ( dword_DD874 )
        sub_4567(dword_1A074, v4);
      if ( v4 > dword_1A090 )
      {
        dword_1A074 += v4;
        v4 = 0;
        dword_1A084 = byte_AD000[dword_1A074];
        dword_1A084 = ((unsigned short)(32 * dword_1A084) ^ byte_AD000[dword_1A074 + 1]) & 0x7FFF;
      }
      else
      {
        --v4;
        do
        {
          dword_1A084 = ((unsigned short)(32 * dword_1A084) ^ byte_AD000[++dword_1A074 + 2]) & 0x7FFF;
          word_BD000[dword_1A074 & 0x7FFF] = word_BD000[dword_1A084 + 0x8000];
          word_BD000[dword_1A084 + 0x8000] = dword_1A074;
          --v4;
        }
        while ( v4 );
        ++dword_1A074;
      }
    }
    if ( dword_DD874 && (unsigned int)dword_1A074 > (unsigned long long)qword_1A0A0 )
    {
      qword_1A0A0 = 0xFFFFFFFFLL;
      v3 = 2;
    }
    if ( v3 )
    {
      if ( qword_1A068 < 0 )
        v0 = 0;
      else
        v0 = &byte_AD000[(unsigned int)qword_1A068];
      sub_C1EA(v0, (unsigned int)dword_1A074 - qword_1A068, (unsigned int)(v3 - 1), 0);
      qword_1A068 = (unsigned int)dword_1A074;
    }
    while ( (unsigned int)dword_1A08C <= 0x105 && !dword_1A088 )
      sub_4350();
  }
  if ( qword_1A068 < 0 )
    v1 = 0;
  else
    v1 = &byte_AD000[(unsigned int)qword_1A068];
  return sub_C1EA(v1, (unsigned int)dword_1A074 - qword_1A068, (unsigned int)(v3 - 1), 1);
}



// Function: deflate @ 0x4a9e
long long deflate(int a1)
{
  unsigned int v1; // ebx
  unsigned char *v3; // rax
  unsigned char *v4; // rax
  unsigned char *v5; // rax
  unsigned char *v6; // rax
  int v7; // [rsp+10h] [rbp-20h]
  int v8; // [rsp+14h] [rbp-1Ch]
  unsigned int v9; // [rsp+18h] [rbp-18h]
  int v10; // [rsp+1Ch] [rbp-14h]

  v7 = 0;
  v8 = 0;
  v1 = 2;
  sub_3F52(a1);
  if ( a1 <= 3 )
    return sub_4677();
  while ( dword_1A08C )
  {
    dword_1A084 = ((unsigned short)(32 * dword_1A084) ^ byte_AD000[dword_1A074 + 2]) & 0x7FFF;
    v9 = word_BD000[dword_1A084 + 0x8000];
    word_BD000[dword_1A074 & 0x7FFF] = v9;
    word_BD000[dword_1A084 + 0x8000] = dword_1A074;
    dword_1A070 = v1;
    v10 = dword_1A078;
    v1 = 2;
    if ( v9
      && dword_1A070 < (unsigned int)dword_1A090
      && dword_1A074 - v9 <= 0x7EFA
      && (unsigned int)dword_1A074 <= (unsigned long long)off_19020 - 262 )
    {
      v1 = sub_40FE(v9);
      if ( v1 > dword_1A08C )
        v1 = dword_1A08C;
      if ( v1 == 3 && (unsigned int)(dword_1A074 - dword_1A078) > 0x1000 )
        v1 = 2;
    }
    if ( (unsigned int)dword_1A070 <= 2 || v1 > dword_1A070 )
    {
      if ( v8 )
      {
        v7 = sub_C45D(0, byte_AD000[dword_1A074 - 1]);
        if ( dword_DD874 && (unsigned int)dword_1A074 > (unsigned long long)qword_1A0A0 )
        {
          qword_1A0A0 = 0xFFFFFFFFLL;
          v7 = 2;
        }
        if ( v7 )
        {
          if ( qword_1A068 < 0 )
            v4 = 0;
          else
            v4 = &byte_AD000[(unsigned int)qword_1A068];
          sub_C1EA(v4, (unsigned int)dword_1A074 - qword_1A068, (unsigned int)(v7 - 1), 0);
          qword_1A068 = (unsigned int)dword_1A074;
        }
        if ( dword_DD874 )
          sub_4567(dword_1A074, 1);
        ++dword_1A074;
        --dword_1A08C;
      }
      else
      {
        if ( dword_DD874 && (unsigned int)dword_1A074 > (unsigned long long)qword_1A0A0 )
        {
          qword_1A0A0 = 0xFFFFFFFFLL;
          v7 = 2;
          if ( qword_1A068 < 0 )
            v5 = 0;
          else
            v5 = &byte_AD000[(unsigned int)qword_1A068];
          sub_C1EA(v5, (unsigned int)dword_1A074 - qword_1A068, 1, 0);
          qword_1A068 = (unsigned int)dword_1A074;
        }
        v8 = 1;
        if ( dword_DD874 )
          sub_4567(dword_1A074, 1);
        ++dword_1A074;
        --dword_1A08C;
      }
    }
    else
    {
      v7 = sub_C45D((unsigned int)(dword_1A074 - v10 - 1), (unsigned int)(dword_1A070 - 3));
      dword_1A08C = dword_1A08C - dword_1A070 + 1;
      dword_1A070 -= 2;
      if ( dword_DD874 )
        sub_4567(dword_1A074, dword_1A070 + 1);
      do
      {
        dword_1A084 = ((unsigned short)(32 * dword_1A084) ^ byte_AD000[++dword_1A074 + 2]) & 0x7FFF;
        word_BD000[dword_1A074 & 0x7FFF] = word_BD000[dword_1A084 + 0x8000];
        word_BD000[dword_1A084 + 0x8000] = dword_1A074;
        --dword_1A070;
      }
      while ( dword_1A070 );
      v8 = 0;
      v1 = 2;
      ++dword_1A074;
      if ( dword_DD874 && (unsigned int)dword_1A074 > (unsigned long long)qword_1A0A0 )
      {
        qword_1A0A0 = 0xFFFFFFFFLL;
        v7 = 2;
      }
      if ( v7 )
      {
        if ( qword_1A068 < 0 )
          v3 = 0;
        else
          v3 = &byte_AD000[(unsigned int)qword_1A068];
        sub_C1EA(v3, (unsigned int)dword_1A074 - qword_1A068, (unsigned int)(v7 - 1), 0);
        qword_1A068 = (unsigned int)dword_1A074;
      }
    }
    while ( (unsigned int)dword_1A08C <= 0x105 && !dword_1A088 )
      sub_4350();
  }
  if ( v8 )
    sub_C45D(0, byte_AD000[dword_1A074 - 1]);
  if ( qword_1A068 < 0 )
    v6 = 0;
  else
    v6 = &byte_AD000[(unsigned int)qword_1A068];
  return sub_C1EA(v6, (unsigned int)dword_1A074 - qword_1A068, (unsigned int)(v7 - 1), 1);
}



// Function: try_help @ 0x5097
void try_help()
{
  fprintf(stderr, "Try `%s --help' for more information.\n", s);
  sub_8E40(1);
}



// Function: help @ 0x50ce
char *help()
{
  const char **v0; // rax
  char *result; // rax
  char **v2; // [rsp+8h] [rbp-8h]

  v2 = &off_18A40;
  printf("Usage: %s [OPTION]... [FILE]...\n", s);
  while ( 1 )
  {
    result = *v2;
    if ( !*v2 )
      break;
    v0 = (const char **)v2++;
    printf("%s\n", *v0);
  }
  return result;
}



// Function: license @ 0x513b
char *license()
{
  const char **v0; // rax
  char *result; // rax
  char **v2; // [rsp+8h] [rbp-8h]

  v2 = off_186A0;
  printf("%s %s\n", s, off_19558);
  while ( 1 )
  {
    result = *v2;
    if ( !*v2 )
      break;
    v0 = (const char **)v2++;
    printf("%s\n", *v0);
  }
  return result;
}



// Function: version @ 0x51af
int version()
{
  sub_513B();
  printf("\n");
  return printf("Written by Jean-loup Gailly.\n");
}



// Function: progerror @ 0x51e7
void progerror(const char *a1)
{
  int v1; // [rsp+1Ch] [rbp-4h]

  v1 = *__errno_location();
  fprintf(stderr, "%s: ", s);
  *__errno_location() = v1;
  perror(a1);
  dword_DD018 = 1;
}



// Function: main @ 0x5249
void main(int a1, char **a2, char **a3)
{
  char **v3; // rax
  int v4; // eax
  int v5; // eax
  int argc; // [rsp+10h] [rbp-30h] BYREF
  int longind; // [rsp+14h] [rbp-2Ch] BYREF
  int v8; // [rsp+18h] [rbp-28h]
  int v9; // [rsp+1Ch] [rbp-24h]
  char **v10; // [rsp+20h] [rbp-20h] BYREF
  char **argv; // [rsp+28h] [rbp-18h]
  size_t v12; // [rsp+30h] [rbp-10h]
  unsigned long long v13; // [rsp+38h] [rbp-8h]

  v13 = __readfsqword(0x28u);
  s = (char *)sub_F40A(*a2, a2, a3);
  v12 = strlen(s);
  if ( v12 > 4 && !strcmp(&s[v12 - 4], ".exe") )
    s[v12 - 4] = 0;
  v10 = a2;
  ptr = (void *)sub_F454(&argc, &v10, "GZIP");
  if ( ptr )
    v3 = v10;
  else
    v3 = 0;
  argv = v3;
  s1 = ".gz";
  qword_DD8B8 = strlen(".gz");
  while ( 1 )
  {
    longind = -1;
    if ( argv )
    {
      if ( argv[optind] && !strcmp(argv[optind], "--") )
      {
        v8 = 176;
      }
      else
      {
        v8 = getopt_long(argc, argv, "ab:cdfhH?klLmMnNqrS:tvVZ123456789", &longopts, &longind);
        if ( v8 < 0 )
        {
          if ( optind != argc )
          {
            fprintf(stderr, "%s: %s: non-option in GZIP environment variable\n", s, argv[optind]);
            sub_5097();
          }
          if ( argc != 1 && !dword_DD004 )
            fprintf(stderr, "%s: warning: GZIP environment variable is deprecated; use an alias or script\n", s);
          free(argv);
          argv = 0;
          optind = 1;
          longind = -1;
        }
        else
        {
          v8 += 131;
        }
      }
    }
    if ( !argv )
      v8 = getopt_long(a1, a2, "ab:cdfhH?klLmMnNqrS:tvVZ123456789", &longopts, &longind);
    if ( v8 < 0 )
    {
      if ( dword_19094 < 0 )
        dword_19094 = dword_DD880;
      if ( dword_19090 < 0 )
        dword_19090 = dword_DD880;
      v9 = a1 - optind;
      if ( dword_DD87C && !dword_DD004 )
        fprintf(stderr, "%s: option --ascii ignored on this system\n", s);
      if ( !qword_DD8B8 || (unsigned long long)qword_DD8B8 > 0x1E )
      {
        fprintf(stderr, "%s: invalid suffix '%s'\n", s, s1);
        sub_8E40(1);
      }
      if ( dword_DD004 )
        v4 = 13;
      else
        v4 = 0;
      dword_DD940 = v4;
      sub_8C07();
      if ( v9 )
      {
        while ( a1 > optind )
        {
          v5 = optind++;
          sub_5F11(a2[v5]);
        }
      }
      else
      {
        sub_5B3E();
      }
      if ( byte_DDD60 && close(0) )
      {
        strcpy(dest, "stdin");
        sub_F6C7();
      }
      if ( dword_DD890 )
      {
        if ( !dword_DD004 && v9 > 1 )
          sub_7FDC(0xFFFFFFFFLL);
        if ( (unsigned int)sub_109D7(stdout) )
          sub_F74D();
      }
      if ( dword_DD000
        && (byte_DD879 && fdatasync(1) && *__errno_location() != 22 || close(1))
        && *__errno_location() != 9 )
      {
        sub_F74D();
      }
      sub_8E40((unsigned int)dword_DD018);
    }
    switch ( v8 )
    {
      case 49:
      case 50:
      case 51:
      case 52:
      case 53:
      case 54:
      case 55:
      case 56:
      case 57:
        goto LABEL_56;
      case 72:
      case 104:
        sub_50CE();
        sub_8E91();
      case 76:
        sub_513B();
        sub_8E91();
      case 77:
        dword_19094 = 0;
        continue;
      case 78:
      case 209:
        dword_19094 = 0;
        dword_19090 = 0;
        continue;
      case 83:
        qword_DD8B8 = strlen(optarg);
        s1 = optarg;
        continue;
      case 86:
        sub_51AF();
        sub_8E91();
      case 90:
        fprintf(stderr, "%s: -Z not supported in this version\n", s);
        sub_5097();
      case 97:
        dword_DD87C = 1;
        continue;
      case 98:
        dword_19098 = atoi(optarg);
        while ( *optarg )
        {
          if ( *optarg <= 47 || *optarg > 57 )
          {
            fprintf(stderr, "%s: -b operand is not an integer\n", s);
            sub_5097();
          }
          ++optarg;
        }
        continue;
      case 99:
        dword_DD000 = 1;
        continue;
      case 100:
        dword_DD880 = 1;
        continue;
      case 102:
        ++dword_DD884;
        continue;
      case 107:
        dword_DD888 = 1;
        continue;
      case 108:
        dword_DD000 = 1;
        dword_DD008 = 1;
        dword_DD880 = 1;
        dword_DD890 = 1;
        continue;
      case 109:
        dword_19094 = 1;
        continue;
      case 110:
      case 241:
        dword_19094 = 1;
        dword_19090 = 1;
        continue;
      case 113:
      case 244:
        dword_DD004 = 1;
        dword_DD894 = 0;
        continue;
      case 114:
        dword_DD88C = 1;
        continue;
      case 116:
        dword_DD000 = 1;
        dword_DD880 = 1;
        dword_DD008 = 1;
        continue;
      case 118:
      case 249:
        ++dword_DD894;
        dword_DD004 = 0;
        continue;
      case 128:
        byte_DD878 = 1;
        continue;
      case 129:
      case 260:
        dword_DD874 = 1;
        continue;
      case 130:
        byte_DD879 = 1;
        continue;
      case 180:
      case 181:
      case 182:
      case 183:
      case 184:
      case 185:
      case 186:
      case 187:
      case 188:
        v8 -= 131;
LABEL_56:
        dword_190A0 = v8 - 48;
        break;
      default:
        if ( v8 > 130 && v8 != 194 )
        {
          fprintf(stderr, "%s: ", s);
          if ( longind >= 0 )
            fprintf(stderr, "--%s: ", *(&longopts.name + 4 * longind));
          else
            fprintf(stderr, "-%c: ", (unsigned int)(v8 - 131));
          fprintf(stderr, "option not valid in GZIP environment variable\n");
        }
        sub_5097();
    }
  }
}



// Function: input_eof @ 0x5a65
long long input_eof()
{
  if ( !dword_DD880 || dword_DD89C )
    return 1;
  if ( dword_DD86C == dword_DD868 )
  {
    if ( dword_DD868 != (int)&unk_40000 || (unsigned int)sub_F0BC(1) == -1 )
      return 1;
    dword_DD86C = 0;
  }
  return 0;
}



// Function: get_input_size_and_time @ 0x5acc
long long get_input_size_and_time()
{
  long long result; // rax
  long long v1; // rdx

  qword_DD020 = -1;
  qword_DD038 = -1;
  result = buf.st_mode & 0xF000;
  if ( (int)result == 0x8000 )
  {
    qword_DD020 = buf.st_size;
    if ( !dword_19094 || (result = (unsigned int)dword_DD890, dword_DD890) )
    {
      result = sub_1108F(&buf);
      timer = result;
      qword_DD038 = v1;
    }
  }
  return result;
}



// Function: treat_stdin @ 0x5b3e
int treat_stdin()
{
  const char *v0; // rdx
  const char *v1; // rax
  int result; // eax

  if ( !dword_DD884 && !dword_DD890 && (byte_DD878 || isatty(dword_DD880 == 0)) )
  {
    if ( !dword_DD004 )
    {
      if ( dword_DD880 )
        v0 = (const char *)&unk_1378C;
      else
        v0 = (const char *)&unk_1378F;
      if ( dword_DD880 )
        v1 = "read from";
      else
        v1 = "written to";
      fprintf(
        stderr,
        "%s: compressed data not %s a terminal. Use -f to force %scompression.\nFor help, type: %s -h\n",
        s,
        v1,
        v0,
        s);
    }
    sub_8E40(1);
  }
  strcpy(dest, "stdin");
  strcpy(byte_DD460, "stdout");
  if ( fstat(0, &buf) )
  {
    sub_51E7("standard input");
    sub_8E40(1);
  }
  sub_5ACC();
  sub_F078();
  dword_DD000 = 1;
  dword_DD8A0 = 0;
  dword_DD860 = 0;
  byte_DDD60 = 1;
  if ( dword_DD880 )
  {
    dword_1909C = sub_72F1((unsigned int)dword_DD860);
    if ( dword_1909C < 0 )
      sub_8E40((unsigned int)dword_DD018);
  }
  while ( 1 )
  {
    result = off_190D0();
    if ( result )
      break;
    if ( (unsigned int)sub_5A65(0, 1) )
    {
      if ( dword_DD890 )
        return sub_7FDC((unsigned int)dword_1909C);
      result = dword_DD894;
      if ( dword_DD894 )
      {
        if ( dword_DD008 )
        {
          return fprintf(stderr, " OK\n");
        }
        else
        {
          result = dword_DD880;
          if ( !dword_DD880 )
          {
            sub_F7A6(qword_DD040 - (qword_DD048 - qword_E1698), qword_DD040, stderr);
            return fprintf(stderr, "\n");
          }
        }
      }
      return result;
    }
    dword_1909C = sub_72F1((unsigned int)dword_DD860);
    result = dword_1909C;
    if ( dword_1909C < 0 )
      return result;
    qword_DD048 = 0;
  }
  return result;
}



// Function: atdir_eq @ 0x5dee
long long atdir_eq(const char *a1, size_t a2)
{
  size_t n; // [rsp+0h] [rbp-10h]

  n = a2;
  if ( !a2 )
  {
    a1 = ".-";
    n = 1;
  }
  return !memcmp(byte_DDD80, a1, n) && !byte_DDD80[n];
}



// Function: atdir_set @ 0x5e60
long long atdir_set(const char *a1, size_t a2)
{
  int v2; // ecx
  int v3; // r8d
  int v4; // r9d
  size_t n; // [rsp+0h] [rbp-10h]
  const char *src; // [rsp+8h] [rbp-8h]

  src = a1;
  n = a2;
  if ( !sub_5DEE(a1, a2) )
  {
    if ( fildes >= 0 )
      close(fildes);
    if ( !a2 )
    {
      src = ".-";
      n = 1;
    }
    memcpy(byte_DDD80, src, n);
    byte_DDD80[n] = 0;
    fildes = sub_10863((unsigned int)byte_DDD80, 0x10000, (unsigned int)byte_DDD80, v2, v3, v4, n);
  }
  return (unsigned int)fildes;
}



// Function: treat_file @ 0x5f11
unsigned long long treat_file(const char *a1)
{
  const char *v1; // rax
  sigset_t *p_oset; // rsi
  long long v3; // rdi
  long long v4; // rdi
  int v5; // eax
  int v6; // eax
  int v7; // eax
  const char *v8; // rax
  int fd; // [rsp+18h] [rbp-A8h]
  int v11; // [rsp+20h] [rbp-A0h]
  int v12; // [rsp+24h] [rbp-9Ch]
  char *name; // [rsp+28h] [rbp-98h]
  sigset_t oset; // [rsp+30h] [rbp-90h] BYREF
  unsigned long long v15; // [rsp+B8h] [rbp-8h]

  v15 = __readfsqword(0x28u);
  if ( !strcmp(a1, "-") )
  {
    v12 = dword_DD000;
    sub_5B3E();
    dword_DD000 = v12;
    return v15 - __readfsqword(0x28u);
  }
  dword_DD860 = sub_6D5A(a1, &buf);
  if ( dword_DD860 < 0 )
    return v15 - __readfsqword(0x28u);
  if ( (buf.st_mode & 0xF000) == 0x4000 )
  {
    if ( dword_DD88C )
    {
      sub_897B((unsigned int)dword_DD860, a1);
    }
    else
    {
      close(dword_DD860);
      if ( !dword_DD004 )
        fprintf(stderr, "%s: %s is a directory -- ignored\n", s, dest);
      if ( !dword_DD018 )
        dword_DD018 = 2;
    }
    return v15 - __readfsqword(0x28u);
  }
  if ( !dword_DD000 )
  {
    if ( (buf.st_mode & 0xF000) != 0x8000 )
    {
      if ( !dword_DD004 )
        fprintf(stderr, "%s: %s is not a directory or a regular file - ignored\n", s, dest);
      if ( !dword_DD018 )
        dword_DD018 = 2;
      goto LABEL_52;
    }
    if ( (buf.st_mode & 0x800) != 0 )
    {
      if ( !dword_DD004 )
        fprintf(stderr, "%s: %s is set-user-ID on execution - ignored\n", s, dest);
      if ( !dword_DD018 )
        dword_DD018 = 2;
      goto LABEL_52;
    }
    if ( (buf.st_mode & 0x400) != 0 )
    {
      if ( !dword_DD004 )
        fprintf(stderr, "%s: %s is set-group-ID on execution - ignored\n", s, dest);
      if ( !dword_DD018 )
        dword_DD018 = 2;
      goto LABEL_52;
    }
    if ( !dword_DD884 )
    {
      if ( (buf.st_mode & 0x200) != 0 )
      {
        if ( !dword_DD004 )
          fprintf(stderr, "%s: %s has the sticky bit set - file ignored\n", s, dest);
        if ( !dword_DD018 )
          dword_DD018 = 2;
        goto LABEL_52;
      }
      if ( buf.st_nlink > 1 )
      {
        if ( !dword_DD004 )
        {
          if ( buf.st_nlink == 2 )
            v1 = (const char *)&unk_1378F;
          else
            v1 = "s";
          fprintf(stderr, "%s: %s has %lu other link%s -- file ignored\n", s, dest, buf.st_nlink - 1, v1);
        }
        if ( !dword_DD018 )
          dword_DD018 = 2;
        goto LABEL_52;
      }
    }
  }
  sub_5ACC();
  if ( !dword_DD000 || dword_DD008 )
  {
    if ( (unsigned int)sub_6FF8() )
    {
LABEL_52:
      close(dword_DD860);
      return v15 - __readfsqword(0x28u);
    }
  }
  else
  {
    strcpy(byte_DD460, "stdout");
  }
  sub_F078();
  dword_DD8A0 = 0;
  if ( dword_DD880 )
  {
    dword_1909C = sub_72F1((unsigned int)dword_DD860);
    if ( dword_1909C < 0 )
      goto LABEL_52;
  }
  if ( dword_DD000 )
  {
    ::fd = 1;
  }
  else
  {
    if ( (unsigned int)sub_6814() )
      return v15 - __readfsqword(0x28u);
    if ( !dword_DD880 && dword_DD01C && !dword_DD894 && !dword_DD004 )
      fprintf(stderr, "%s: %s compressed to %s\n", s, dest, byte_DD460);
  }
  if ( !dword_DD01C )
    dword_DD01C = dword_19090 == 0;
  if ( dword_DD894 && !dword_DD890 )
    fprintf(stderr, "%s:\t", dest);
  while ( 1 )
  {
    p_oset = (sigset_t *)(unsigned int)::fd;
    v3 = (unsigned int)dword_DD860;
    if ( (unsigned int)off_190D0() )
      break;
    if ( (unsigned int)sub_5A65(v3, p_oset) )
      goto LABEL_71;
    dword_1909C = sub_72F1((unsigned int)dword_DD860);
    if ( dword_1909C < 0 )
      goto LABEL_71;
    qword_DD048 = 0;
  }
  dword_1909C = -1;
LABEL_71:
  v4 = (unsigned int)dword_DD860;
  if ( close(dword_DD860) )
    sub_F6C7(v4, p_oset);
  if ( dword_DD890 )
  {
    sub_7FDC((unsigned int)dword_1909C);
  }
  else
  {
    if ( !dword_DD000 )
    {
      sub_870D(&buf, p_oset);
      if ( byte_DD879
        && (fildes >= 0 && fdatasync(fildes) && *__errno_location() != 22 || fsync(::fd) && *__errno_location() != 22)
        || close(::fd) )
      {
        sub_F74D();
      }
      if ( !dword_DD888 )
      {
        name = (char *)sub_10244(dest);
        v5 = sub_5DEE(dest, name - dest) ? fildes : -1;
        fd = v5;
        sigprocmask(0, &set, &oset);
        dword_190A4 = -1;
        v6 = fd >= 0 ? unlinkat(fd, name, 0) : sub_F430(dest);
        v7 = v6 ? *__errno_location() : 0;
        v11 = v7;
        p_oset = &oset;
        sigprocmask(2, &oset, 0);
        if ( v11 )
        {
          if ( !dword_DD004 )
          {
            p_oset = (sigset_t *)"%s: ";
            fprintf(stderr, "%s: ", s);
          }
          if ( !dword_DD018 )
            dword_DD018 = 2;
          if ( !dword_DD004 )
          {
            *__errno_location() = v11;
            perror(dest);
          }
        }
      }
    }
    if ( dword_1909C == -1 )
    {
      if ( !dword_DD000 )
        sub_8EBB(0, p_oset);
    }
    else if ( dword_DD894 )
    {
      if ( dword_DD008 )
      {
        fprintf(stderr, " OK");
      }
      else if ( dword_DD880 )
      {
        sub_F7A6(qword_DD048 - (qword_DD040 - qword_E1698), qword_DD048, stderr);
      }
      else
      {
        sub_F7A6(qword_DD040 - (qword_DD048 - qword_E1698), qword_DD040, stderr);
      }
      if ( !dword_DD008 )
      {
        if ( dword_DD888 )
          v8 = "created";
        else
          v8 = "replaced with";
        fprintf(stderr, " -- %s %s", v8, byte_DD460);
      }
      fprintf(stderr, "\n");
    }
  }
  return v15 - __readfsqword(0x28u);
}



// Function: volatile_strcpy @ 0x67dc
long long volatile_strcpy(char *a1, unsigned char *a2)
{
  unsigned char *v2; // rax
  char *v3; // rdx
  long long result; // rax

  do
  {
    v2 = a2++;
    v3 = a1++;
    result = *v2;
    *v3 = result;
  }
  while ( (char)result );
  return result;
}



// Function: create_outfile @ 0x6814
long long create_outfile()
{
  int v0; // r8d
  int v1; // r9d
  char v3; // [rsp+0h] [rbp-C0h]
  int v4; // [rsp+Ch] [rbp-B4h]
  int v5; // [rsp+10h] [rbp-B0h]
  int v6; // [rsp+18h] [rbp-A8h]
  int v7; // [rsp+1Ch] [rbp-A4h]
  char *v8; // [rsp+20h] [rbp-A0h]
  long long v9; // [rsp+28h] [rbp-98h]
  sigset_t oset; // [rsp+30h] [rbp-90h] BYREF
  unsigned long long v11; // [rsp+B8h] [rbp-8h]

  v11 = __readfsqword(0x28u);
  v4 = 0;
  v8 = byte_DD460;
  v5 = -100;
  if ( !dword_DD888 )
  {
    v9 = sub_10244(byte_DD460);
    v6 = sub_5E60(byte_DD460, v9 - (long long)byte_DD460);
    if ( v6 >= 0 )
    {
      LODWORD(v8) = v9;
      v5 = v6;
    }
  }
  while ( 1 )
  {
    sub_67DC(byte_DD960, (unsigned char *)byte_DD460);
    sigprocmask(0, &set, &oset);
    fd = sub_10AE4(v5, (int)v8, 193, 384, v0, v1, v3);
    dword_190A4 = fd;
    v7 = *__errno_location();
    sigprocmask(2, &oset, 0);
    if ( fd >= 0 )
      break;
    if ( v7 == 17 )
    {
      if ( (unsigned int)sub_85C8() )
      {
        close(dword_DD860);
        return 1;
      }
    }
    else
    {
      if ( v7 != 36 )
      {
        sub_51E7(byte_DD460);
        close(dword_DD860);
        return 1;
      }
      sub_83EA(byte_DD460);
      v4 = 1;
    }
  }
  if ( v4 && dword_DD880 )
  {
    if ( !dword_DD004 )
      fprintf(stderr, "%s: %s: warning, name truncated\n", s, byte_DD460);
    if ( !dword_DD018 )
      dword_DD018 = 2;
  }
  return 0;
}



// Function: get_suffix @ 0x6a54
const char *get_suffix(const char *a1)
{
  long long v1; // rax
  unsigned char v3; // [rsp+13h] [rbp-5Dh]
  int v4; // [rsp+14h] [rbp-5Ch]
  int v5; // [rsp+18h] [rbp-58h]
  int v6; // [rsp+1Ch] [rbp-54h]
  const char **i; // [rsp+20h] [rbp-50h]
  const char **v8; // [rsp+20h] [rbp-50h]
  const char *v9; // [rsp+28h] [rbp-48h]
  size_t v10; // [rsp+30h] [rbp-40h]
  void *ptr; // [rsp+38h] [rbp-38h]
  char dest[40]; // [rsp+40h] [rbp-30h] BYREF
  unsigned long long v13; // [rsp+68h] [rbp-8h]

  v13 = __readfsqword(0x28u);
  v3 = 0;
  for ( i = (const char **)off_190E8; *i; ++i )
  {
    v10 = strlen(*i);
    if ( v10 > qword_DD8B8 && !strcmp(s1, &(*i)[v10 - qword_DD8B8]) )
    {
      v3 = 1;
      break;
    }
  }
  ptr = (void *)sub_12377(s1);
  sub_F393(ptr);
  if ( v3 )
    v1 = 8;
  else
    v1 = 0;
  qword_190E0[v1] = ptr;
  v8 = (const char **)&qword_190E0[v3];
  v4 = strlen(a1);
  if ( v4 > 32 )
    strcpy(dest, &a1[v4 - 32]);
  else
    strcpy(dest, a1);
  sub_F393(dest);
  v5 = strlen(dest);
  v9 = 0;
  while ( 1 )
  {
    v6 = strlen(*v8);
    if ( v5 > v6 && dest[v5 - v6 - 1] != 47 && !strcmp(&dest[v5 - (long long)v6], *v8) )
      break;
    if ( !*++v8 )
      goto LABEL_19;
  }
  v9 = &a1[v4 - (long long)v6];
LABEL_19:
  free(ptr);
  return v9;
}



// Function: open_and_stat @ 0x6c6d
long long open_and_stat(const char *a1, int a2, struct stat *a3, long long a4, int a5, int a6)
{
  int v8; // [rsp+14h] [rbp-2Ch]
  int v9; // [rsp+20h] [rbp-20h]
  int v10; // [rsp+24h] [rbp-1Ch]
  int fd; // [rsp+28h] [rbp-18h]
  int v12; // [rsp+2Ch] [rbp-14h]
  int v13; // [rsp+30h] [rbp-10h]
  long long v14; // [rsp+38h] [rbp-8h]

  v8 = a2;
  v9 = -100;
  v13 = (int)a1;
  if ( !dword_DD000 && !dword_DD884 )
    v8 = a2 | 0x20000;
  if ( !dword_DD888 )
  {
    v14 = sub_10244(a1);
    v10 = sub_5E60(a1, v14 - (long long)a1);
    if ( v10 >= 0 )
    {
      v13 = v14;
      v9 = v10;
    }
  }
  fd = sub_10AE4(v9, v13, v8, v13, a5, a6);
  if ( fd < 0 || !fstat(fd, a3) )
    return (unsigned int)fd;
  v12 = *__errno_location();
  close(fd);
  *__errno_location() = v12;
  return 0xFFFFFFFFLL;
}



// Function: open_input_file @ 0x6d5a
long long open_input_file(const char *a1, struct stat *a2)
{
  long long v2; // rcx
  int v3; // r8d
  int v4; // r9d
  long long v6; // rcx
  int v7; // r8d
  int v8; // r9d
  int v9; // [rsp+18h] [rbp-38h]
  int v10; // [rsp+20h] [rbp-30h]
  int v11; // [rsp+20h] [rbp-30h]
  int v12; // [rsp+24h] [rbp-2Ch]
  char **v13; // [rsp+28h] [rbp-28h]
  char *src; // [rsp+30h] [rbp-20h]
  char *s1; // [rsp+38h] [rbp-18h]

  v9 = 0;
  v13 = (char **)&qword_19140;
  qword_19140 = ::s1;
  if ( strlen(a1) <= 0x3FE )
  {
    strcpy(dest, a1);
    v10 = sub_6C6D(dest, 2304, a2, v2, v3, v4);
    if ( v10 >= 0 )
      return (unsigned int)v10;
    if ( !dword_DD880 || *__errno_location() != 2 || sub_6A54(dest) )
    {
LABEL_14:
      sub_51E7(dest);
      return 0xFFFFFFFFLL;
    }
    v12 = strlen(dest);
    if ( !strcmp(::s1, ".gz") )
      v13 = off_19148;
    while ( 1 )
    {
      src = *v13;
      s1 = *v13;
      strcpy(dest, a1);
      if ( v12 + strlen(src) > 0x3FF )
        break;
      strcat(dest, src);
      v11 = sub_6C6D(dest, 2304, a2, v6, v7, v8);
      if ( v11 >= 0 )
        return (unsigned int)v11;
      if ( *__errno_location() != 2 )
        goto LABEL_14;
      if ( !strcmp(s1, ::s1) )
        v9 = *__errno_location();
      if ( !*++v13 )
      {
        strcpy(dest, a1);
        strcat(dest, ::s1);
        *__errno_location() = v9;
        sub_51E7(dest);
        return 0xFFFFFFFFLL;
      }
    }
  }
  fprintf(stderr, "%s: %s: file name too long\n", s, a1);
  dword_DD018 = 1;
  return 0xFFFFFFFFLL;
}



// Function: make_ofname @ 0x6ff8
long long make_ofname()
{
  char *s1; // [rsp+8h] [rbp-8h]

  strcpy(byte_DD460, dest);
  s1 = (char *)sub_6A54(byte_DD460);
  if ( dword_DD880 )
  {
    if ( !s1 )
    {
      if ( !dword_DD88C && dword_DD008 )
        return 0;
      if ( dword_DD894 || !dword_DD88C && !dword_DD004 )
      {
        if ( !dword_DD004 )
          fprintf(stderr, "%s: %s: unknown suffix -- ignored\n", s, dest);
        if ( !dword_DD018 )
          dword_DD018 = 2;
      }
      return 2;
    }
    sub_F393(s1);
    if ( !strcmp(s1, ".tgz") || !strcmp(s1, ".taz") )
      strcpy(s1, ".tar");
    else
      *s1 = 0;
    return 0;
  }
  if ( !s1 || dword_DD884 )
  {
    dword_DD01C = 0;
    if ( qword_DD8B8 + strlen(byte_DD460) <= 0x3FF )
    {
      strcat(byte_DD460, ::s1);
      return 0;
    }
    if ( !dword_DD004 )
      fprintf(stderr, "%s: %s: file name too long\n", s, dest);
    if ( !dword_DD018 )
      dword_DD018 = 2;
    return 2;
  }
  else
  {
    if ( dword_DD894 || !dword_DD88C && !dword_DD004 )
      fprintf(stderr, "%s: %s already has %s suffix -- unchanged\n", s, dest, s1);
    return 2;
  }
}



// Function: discard_input_bytes @ 0x7240
unsigned long long discard_input_bytes(long long a1, char a2)
{
  int v2; // eax
  char v3; // al
  char v6; // [rsp+17h] [rbp-9h] BYREF
  unsigned long long v7; // [rsp+18h] [rbp-8h]

  v7 = __readfsqword(0x28u);
  while ( a1 )
  {
    if ( dword_DD86C >= (unsigned int)dword_DD868 )
    {
      v3 = sub_F0BC(0);
    }
    else
    {
      v2 = dword_DD86C++;
      v3 = byte_1B000[v2];
    }
    v6 = v3;
    if ( (a2 & 2) != 0 )
      sub_EFDA(&v6, 1);
    if ( a1 == -1 )
    {
      if ( !v6 )
        return v7 - __readfsqword(0x28u);
    }
    else
    {
      --a1;
    }
  }
  return v7 - __readfsqword(0x28u);
}



// Function: get_method @ 0x72f1
long long get_method(unsigned int a1)
{
  int v1; // eax
  int v2; // eax
  int v3; // eax
  int v4; // eax
  int v5; // eax
  char v6; // al
  int v7; // eax
  char v8; // al
  int v9; // eax
  int v10; // eax
  unsigned long long v11; // rsi
  int v12; // eax
  int v13; // eax
  int v15; // eax
  unsigned char v16; // al
  int v17; // eax
  long long v18; // rax
  int v19; // eax
  long long v20; // rax
  int v21; // eax
  long long v22; // rax
  int v23; // eax
  long long v24; // rax
  int v25; // eax
  char v26; // al
  unsigned int v27; // eax
  long long v28; // rdx
  char v29; // al
  int v30; // eax
  unsigned char v31; // al
  int v32; // eax
  unsigned char v33; // al
  int v34; // eax
  char v35; // dl
  char *v36; // rax
  char *v37; // rdx
  size_t v38; // rax
  int v39; // eax
  int v40; // eax
  int v41; // eax
  int v42; // eax
  int v43; // eax
  int v44; // eax
  unsigned char v45; // [rsp+17h] [rbp-49h]
  int v46; // [rsp+18h] [rbp-48h]
  int v47; // [rsp+1Ch] [rbp-44h]
  int i; // [rsp+20h] [rbp-40h]
  int v49; // [rsp+24h] [rbp-3Ch]
  unsigned int v50; // [rsp+24h] [rbp-3Ch]
  int v51; // [rsp+28h] [rbp-38h]
  int v52; // [rsp+2Ch] [rbp-34h]
  int v53; // [rsp+2Ch] [rbp-34h]
  char *s; // [rsp+30h] [rbp-30h]
  char *sa; // [rsp+30h] [rbp-30h]
  long long v56; // [rsp+38h] [rbp-28h]
  long long v57; // [rsp+38h] [rbp-28h]
  long long v58; // [rsp+38h] [rbp-28h]
  long long v59; // [rsp+38h] [rbp-28h]
  char *dest; // [rsp+40h] [rbp-20h]
  char v61[2]; // [rsp+4Ch] [rbp-14h] BYREF
  char s1; // [rsp+4Eh] [rbp-12h] BYREF
  char v63; // [rsp+4Fh] [rbp-11h]
  char v64; // [rsp+50h] [rbp-10h]
  unsigned char v65; // [rsp+51h] [rbp-Fh]
  int v66; // [rsp+52h] [rbp-Eh]
  char v67; // [rsp+56h] [rbp-Ah]
  char v68; // [rsp+57h] [rbp-9h]
  unsigned long long v69; // [rsp+58h] [rbp-8h]

  v69 = __readfsqword(0x28u);
  if ( dword_DD884 && dword_DD000 )
  {
    if ( dword_DD86C >= (unsigned int)dword_DD868 )
    {
      v2 = sub_F0BC(1);
    }
    else
    {
      v1 = dword_DD86C++;
      v2 = (unsigned char)byte_1B000[v1];
    }
    v46 = v2;
    s1 = v2;
    if ( dword_DD86C >= (unsigned int)dword_DD868 )
    {
      v4 = sub_F0BC(1);
    }
    else
    {
      v3 = dword_DD86C++;
      v4 = (unsigned char)byte_1B000[v3];
    }
    v47 = v4;
    v63 = v4;
  }
  else
  {
    if ( dword_DD86C >= (unsigned int)dword_DD868 )
    {
      v6 = sub_F0BC(0);
    }
    else
    {
      v5 = dword_DD86C++;
      v6 = byte_1B000[v5];
    }
    s1 = v6;
    v46 = 0;
    if ( v6 )
    {
      if ( dword_DD86C >= (unsigned int)dword_DD868 )
      {
        v8 = sub_F0BC(0);
      }
      else
      {
        v7 = dword_DD86C++;
        v8 = byte_1B000[v7];
      }
      v63 = v8;
      v47 = 0;
    }
    else
    {
      if ( dword_DD86C >= (unsigned int)dword_DD868 )
      {
        v10 = sub_F0BC(1);
      }
      else
      {
        v9 = dword_DD86C++;
        v10 = (unsigned char)byte_1B000[v9];
      }
      v47 = v10;
      v63 = v10;
    }
  }
  dword_1909C = -1;
  ++dword_DD8A0;
  qword_E1698 = 0;
  dword_DD89C = 0;
  v11 = (unsigned long long)&unk_13A23;
  if ( !memcmp(&s1, &unk_13A23, 2u) || (v11 = (unsigned long long)&off_13A26, !memcmp(&s1, &off_13A26, 2u)) )
  {
    if ( dword_DD86C >= (unsigned int)dword_DD868 )
    {
      v13 = sub_F0BC(0);
    }
    else
    {
      v12 = dword_DD86C++;
      v13 = (unsigned char)byte_1B000[v12];
    }
    dword_1909C = v13;
    if ( v13 != 8 )
    {
      fprintf(stderr, "%s: %s: unknown method %d -- not supported\n", ::s, ::dest, dword_1909C);
      dword_DD018 = 1;
      return 0xFFFFFFFFLL;
    }
    off_190D0 = sub_E86A;
    if ( dword_DD86C >= (unsigned int)dword_DD868 )
    {
      v16 = sub_F0BC(0);
    }
    else
    {
      v15 = dword_DD86C++;
      v16 = byte_1B000[v15];
    }
    v45 = v16;
    if ( (v16 & 0x20) != 0 )
    {
      fprintf(stderr, "%s: %s is encrypted -- not supported\n", ::s, ::dest);
      dword_DD018 = 1;
      return 0xFFFFFFFFLL;
    }
    if ( (v16 & 0xC0) != 0 )
    {
      v11 = (unsigned long long)"%s: %s has flags 0x%x -- not supported\n";
      fprintf(stderr, "%s: %s has flags 0x%x -- not supported\n", ::s, ::dest, v16);
      dword_DD018 = 1;
      if ( dword_DD884 <= 1 )
        return 0xFFFFFFFFLL;
    }
    if ( dword_DD86C >= (unsigned int)dword_DD868 )
    {
      v18 = (int)sub_F0BC(0);
    }
    else
    {
      v17 = dword_DD86C++;
      v18 = (unsigned char)byte_1B000[v17];
    }
    v56 = v18;
    if ( dword_DD86C >= (unsigned int)dword_DD868 )
    {
      v20 = (long long)(int)sub_F0BC(0) << 8;
    }
    else
    {
      v19 = dword_DD86C++;
      v20 = (unsigned long long)(unsigned char)byte_1B000[v19] << 8;
    }
    v57 = v20 | v56;
    if ( dword_DD86C >= (unsigned int)dword_DD868 )
    {
      v22 = (long long)(int)sub_F0BC(0) << 16;
    }
    else
    {
      v21 = dword_DD86C++;
      v22 = (unsigned long long)(unsigned char)byte_1B000[v21] << 16;
    }
    v58 = v22 | v57;
    if ( dword_DD86C >= (unsigned int)dword_DD868 )
    {
      v24 = (long long)(int)sub_F0BC(0) << 24;
    }
    else
    {
      v23 = dword_DD86C++;
      v24 = (unsigned long long)(unsigned char)byte_1B000[v23] << 24;
    }
    v59 = v24 | v58;
    if ( v59 && !dword_19094 )
    {
      if ( v59 < 0 )
      {
        if ( !dword_DD004 )
        {
          v11 = (unsigned long long)"%s: %s: MTIME %lu out of range for this platform\n";
          fprintf(stderr, "%s: %s: MTIME %lu out of range for this platform\n", ::s, ::dest, v59);
        }
        if ( !dword_DD018 )
          dword_DD018 = 2;
        timer = 0x7FFFFFFFFFFFFFFFLL;
        qword_DD038 = 999999999;
      }
      else
      {
        timer = v59;
        qword_DD038 = 0;
      }
    }
    if ( dword_DD86C >= (unsigned int)dword_DD868 )
    {
      v26 = sub_F0BC(0);
    }
    else
    {
      v25 = dword_DD86C++;
      v26 = byte_1B000[v25];
    }
    v67 = v26;
    if ( dword_DD86C >= (unsigned int)dword_DD868 )
    {
      v29 = sub_F0BC(0);
    }
    else
    {
      v27 = dword_DD86C++;
      v28 = v27;
      v29 = byte_1B000[v27];
    }
    v68 = v29;
    if ( (v45 & 2) != 0 )
    {
      v64 = 8;
      v65 = v45;
      v66 = v59;
      sub_EFDA(0, 0);
      v11 = 10;
      sub_EFDA(&s1, 10);
    }
    if ( (v45 & 4) != 0 )
    {
      if ( dword_DD86C >= (unsigned int)dword_DD868 )
      {
        v31 = sub_F0BC(0);
      }
      else
      {
        v30 = dword_DD86C++;
        v31 = byte_1B000[v30];
      }
      v61[0] = v31;
      v49 = v31;
      if ( dword_DD86C >= (unsigned int)dword_DD868 )
      {
        v33 = sub_F0BC(0);
      }
      else
      {
        v32 = dword_DD86C++;
        v33 = byte_1B000[v32];
      }
      v61[1] = v33;
      v50 = (v33 << 8) | v49;
      if ( dword_DD894 )
        fprintf(stderr, "%s: %s: extra field of %u bytes ignored\n", ::s, ::dest, v50);
      if ( (v45 & 2) != 0 )
        sub_EFDA(v61, 2);
      v11 = v45;
      sub_7240(v50, v45);
    }
    if ( (v45 & 8) != 0 )
    {
      if ( dword_19090 || dword_DD000 && !dword_DD890 || dword_DD8A0 > 1 )
      {
        sub_7240(-1, v45);
      }
      else
      {
        s = (char *)sub_F40A(byte_DD460, v11, v28);
        dest = s;
        while ( 1 )
        {
          if ( dword_DD86C >= (unsigned int)dword_DD868 )
          {
            v35 = sub_F0BC(0);
          }
          else
          {
            v34 = dword_DD86C++;
            v35 = byte_1B000[v34];
          }
          *s = v35;
          v36 = s;
          v37 = ++s;
          if ( !*v36 )
            break;
          if ( s >= (char *)&dword_DD860 )
            sub_F5EF("corrupted input -- file name too large");
        }
        if ( (v45 & 2) != 0 )
        {
          v11 = (unsigned int)((int)s - (int)dest);
          sub_EFDA(dest, v11);
        }
        sa = (char *)sub_F40A(dest, v11, v37);
        v38 = strlen(sa);
        memmove(dest, sa, v38 + 1);
        if ( !dword_DD890 && dest )
          dword_DD890 = 0;
      }
    }
    if ( (v45 & 0x10) != 0 )
      sub_7240(-1, v45);
    if ( (v45 & 2) != 0 )
    {
      v51 = (unsigned short)sub_EFDA(&s1, 0);
      if ( dword_DD86C >= (unsigned int)dword_DD868 )
      {
        v40 = sub_F0BC(0);
      }
      else
      {
        v39 = dword_DD86C++;
        v40 = (unsigned char)byte_1B000[v39];
      }
      v52 = v40;
      if ( dword_DD86C >= (unsigned int)dword_DD868 )
      {
        v42 = (unsigned int)sub_F0BC(0) << 8;
      }
      else
      {
        v41 = dword_DD86C++;
        v42 = (unsigned char)byte_1B000[v41] << 8;
      }
      v53 = v42 | v52;
      if ( v53 != v51 )
      {
        fprintf(stderr, "%s: %s: header checksum 0x%04x != computed checksum 0x%04x\n", ::s, ::dest, v53, v51);
        dword_DD018 = 1;
        if ( dword_DD884 <= 1 )
          return 0xFFFFFFFFLL;
      }
    }
    if ( dword_DD8A0 == 1 )
      qword_E1698 = (unsigned int)(dword_DD86C + 8);
  }
  else if ( !memcmp(&s1, &unk_13B7C, 2u) && dword_DD86C == 2 && !memcmp(byte_1B000, &unk_13B7C, 4u) )
  {
    dword_DD86C = 0;
    off_190D0 = sub_E86A;
    if ( (unsigned int)sub_E655(a1) )
      return 0xFFFFFFFFLL;
    dword_DD89C = 1;
  }
  else if ( !memcmp(&s1, &unk_13B81, 2u) )
  {
    off_190D0 = sub_E3FD;
    dword_1909C = 2;
  }
  else if ( !memcmp(&s1, &unk_13B84, 2u) )
  {
    off_190D0 = sub_D846;
    dword_1909C = 1;
    dword_DD89C = 1;
  }
  else if ( !memcmp(&s1, &unk_13B87, 2u) )
  {
    off_190D0 = sub_D7D6;
    dword_1909C = 3;
    dword_DD89C = 1;
  }
  else if ( dword_DD884 && dword_DD000 && !dword_DD890 )
  {
    dword_1909C = 0;
    off_190D0 = sub_EF1D;
    if ( v47 != -1 )
      --dword_DD86C;
    dword_DD89C = 1;
    if ( v46 != -1 )
      sub_F321(1, &s1, 1);
  }
  if ( dword_1909C >= 0 )
    return (unsigned int)dword_1909C;
  if ( dword_DD8A0 == 1 )
  {
    fprintf(stderr, "\n%s: %s: not in gzip format\n", ::s, ::dest);
    dword_DD018 = 1;
    return 0xFFFFFFFFLL;
  }
  else
  {
    if ( s1 )
      goto LABEL_147;
    for ( i = v47; !i; i = v44 )
    {
      if ( dword_DD86C >= (unsigned int)dword_DD868 )
      {
        v44 = sub_F0BC(1);
      }
      else
      {
        v43 = dword_DD86C++;
        v44 = (unsigned char)byte_1B000[v43];
      }
    }
    if ( i != -1 )
    {
LABEL_147:
      if ( !dword_DD004 )
        fprintf(stderr, "\n%s: %s: decompression OK, trailing garbage ignored\n", ::s, ::dest);
      if ( !dword_DD018 )
        dword_DD018 = 2;
      return 4294967294LL;
    }
    else
    {
      if ( dword_DD894 )
      {
        if ( !dword_DD004 )
          fprintf(stderr, "\n%s: %s: decompression OK, trailing zero bytes ignored\n", ::s, ::dest);
        if ( !dword_DD018 )
          dword_DD018 = 2;
      }
      return 4294967293LL;
    }
  }
}



// Function: do_list @ 0x7fdc
int do_list(int a1)
{
  int result; // eax
  long long v2; // [rsp+20h] [rbp-10h]
  struct tm *v3; // [rsp+28h] [rbp-8h]

  if ( dword_190C8 && a1 >= 0 )
  {
    dword_190C8 = 0;
    if ( dword_DD894 )
      printf("method  crc     date  time  ");
    if ( !dword_DD004 )
      printf("%*.*s %*.*s  ratio uncompressed_name\n", 19, 19, "compressed", 19, 19, "uncompressed");
    goto LABEL_16;
  }
  if ( a1 >= 0 )
  {
LABEL_16:
    v2 = -1;
    if ( a1 == 8 && !dword_DD89C )
      v2 = qword_E1680;
    if ( dword_DD894 )
    {
      v3 = localtime(&timer);
      printf("%5s %08lx ", off_18B20[a1], v2);
      if ( v3 )
        printf("%s%3d %02d:%02d ", &aJan[4 * v3->tm_mon], v3->tm_mday, v3->tm_hour, v3->tm_min);
      else
        printf("??? ?? ??:?? ");
    }
    sub_F81A(stdout, qword_DD040, 19);
    printf(" ");
    sub_F81A(stdout, qword_DD048, 19);
    printf(" ");
    if ( qword_DD040 == -1 )
    {
      qword_DDD68 = -1;
      qword_E1698 = 0;
      qword_DD048 = 0;
      qword_DD040 = 0;
    }
    else if ( qword_DDD68 >= 0 )
    {
      qword_DDD68 += qword_DD040;
    }
    if ( qword_DD048 == -1 )
    {
      qword_DDD70 = -1;
      qword_E1698 = 0;
      qword_DD048 = 0;
      qword_DD040 = 0;
    }
    else if ( qword_DDD70 >= 0 )
    {
      qword_DDD70 += qword_DD048;
    }
    sub_F7A6(qword_DD048 - (qword_DD040 - qword_E1698), qword_DD048, stdout);
    return printf(" %s\n", byte_DD460);
  }
  result = qword_DDD68;
  if ( qword_DDD68 > 0 )
  {
    result = qword_DDD70;
    if ( qword_DDD70 > 0 )
    {
      if ( dword_DD894 )
        printf("                            ");
      if ( dword_DD894 || !dword_DD004 )
      {
        sub_F81A(stdout, qword_DDD68, 19);
        printf(" ");
        sub_F81A(stdout, qword_DDD70, 19);
        printf(" ");
      }
      sub_F7A6(qword_DDD70 - (qword_DDD68 - qword_E1698), qword_DDD70, stdout);
      return printf(" (totals)\n");
    }
  }
  return result;
}



// Function: shorten_name @ 0x83ea
char *shorten_name(const char *a1)
{
  char *result; // rax
  char *v2; // rax
  int v3; // [rsp+14h] [rbp-1Ch]
  int v4; // [rsp+18h] [rbp-18h]
  int v5; // [rsp+1Ch] [rbp-14h]
  char *dest; // [rsp+20h] [rbp-10h]
  char *desta; // [rsp+20h] [rbp-10h]
  char *v8; // [rsp+28h] [rbp-8h]
  char *v9; // [rsp+28h] [rbp-8h]

  dest = 0;
  v3 = 3;
  v4 = strlen(a1);
  if ( dword_DD880 )
  {
    if ( v4 <= 1 )
      sub_F5EF("name too short");
    result = (char *)&a1[v4 - 1];
    *result = 0;
  }
  else
  {
    v8 = (char *)sub_6A54(a1);
    if ( !v8 )
      sub_F5EF("can't recover suffix\n");
    *v8 = 0;
    dword_DD01C = 1;
    if ( v4 > 4 && !strcmp(v8 - 4, ".tar") )
    {
      return strcpy(v8 - 4, ".tgz");
    }
    else
    {
      do
      {
        v9 = (char *)sub_10244(a1);
        while ( *v9 )
        {
          v5 = strcspn(v9, ".");
          v9 += v5;
          if ( v5 > v3 )
            dest = v9 - 1;
          if ( *v9 )
            ++v9;
        }
        if ( dest )
          break;
        --v3;
      }
      while ( v3 );
      if ( dest )
      {
        do
        {
          *dest = dest[1];
          v2 = dest++;
        }
        while ( *v2 );
        desta = dest - 1;
      }
      else
      {
        desta = strrchr(a1, 46);
        if ( !desta )
          sub_F5EF("internal error in shorten_name");
        if ( !desta[1] )
          --desta;
      }
      return strcpy(desta, s1);
    }
  }
  return result;
}



// Function: check_ofname @ 0x85c8
long long check_ofname()
{
  int v1; // [rsp+Ch] [rbp-4h]

  if ( dword_DD884 )
    goto LABEL_15;
  v1 = 0;
  fprintf(stderr, "%s: %s already exists;", s, byte_DD460);
  if ( dword_DD898 && (byte_DD878 || isatty(0)) )
  {
    fprintf(stderr, " do you wish to overwrite (y or n)? ");
    sub_109D7(stderr);
    v1 = (unsigned char)sub_123A8();
  }
  if ( v1 )
  {
LABEL_15:
    if ( (unsigned int)sub_F430(byte_DD460) )
    {
      sub_51E7(byte_DD460);
      return 1;
    }
    else
    {
      return 0;
    }
  }
  else
  {
    fprintf(stderr, "\tnot overwritten\n");
    if ( !dword_DD018 )
      dword_DD018 = 2;
    return 1;
  }
}



// Function: do_chown @ 0x86dc
int do_chown(int a1, long long a2, __uid_t a3, __gid_t a4)
{
  return fchown(a1, a3, a4);
}



// Function: copy_stat @ 0x870d
unsigned long long copy_stat(int *a1)
{
  long long v1; // rdx
  long long v2; // rdx
  bool v3; // al
  bool v5; // [rsp+1Fh] [rbp-41h]
  int mode; // [rsp+20h] [rbp-40h]
  int v7; // [rsp+24h] [rbp-3Ch]
  int v8; // [rsp+2Ch] [rbp-34h]
  long long v9[2]; // [rsp+30h] [rbp-30h] BYREF
  long long v10; // [rsp+40h] [rbp-20h]
  long long v11; // [rsp+48h] [rbp-18h]
  unsigned long long v12; // [rsp+58h] [rbp-8h]

  v12 = __readfsqword(0x28u);
  mode = a1[6] & 0x1FF;
  v9[0] = sub_1105B(a1);
  v9[1] = v1;
  v10 = sub_1108F(a1);
  v11 = v2;
  v3 = dword_DD880 && qword_DD038 >= 0 && (v10 != timer || v11 != qword_DD038);
  v5 = v3;
  if ( v3 )
  {
    v10 = timer;
    v11 = qword_DD038;
  }
  if ( (unsigned int)sub_113CA((unsigned int)fd, byte_DD460, v9) )
  {
    v7 = *__errno_location();
    if ( !dword_DD004 )
      fprintf(stderr, "%s: ", s);
    if ( !dword_DD018 )
      dword_DD018 = 2;
    if ( !dword_DD004 )
    {
      *__errno_location() = v7;
      perror(byte_DD460);
    }
  }
  else if ( v5 && dword_DD894 > 1 )
  {
    fprintf(stderr, "%s: timestamp restored\n", byte_DD460);
  }
  sub_86DC(fd, (long long)byte_DD460, 0xFFFFFFFF, a1[8]);
  if ( fchmod(fd, mode) )
  {
    v8 = *__errno_location();
    if ( !dword_DD004 )
      fprintf(stderr, "%s: ", s);
    if ( !dword_DD018 )
      dword_DD018 = 2;
    if ( !dword_DD004 )
    {
      *__errno_location() = v8;
      perror(byte_DD460);
    }
  }
  sub_86DC(fd, (long long)byte_DD460, a1[7], 0xFFFFFFFF);
  return v12 - __readfsqword(0x28u);
}



// Function: treat_dir @ 0x897b
unsigned long long treat_dir(int a1, const char *a2)
{
  size_t v2; // rax
  char *s1; // [rsp+18h] [rbp-438h]
  size_t v5; // [rsp+20h] [rbp-430h]
  DIR *dirp; // [rsp+28h] [rbp-428h]
  char *ptr; // [rsp+30h] [rbp-420h]
  size_t v8; // [rsp+38h] [rbp-418h]
  char dest[1032]; // [rsp+40h] [rbp-410h] BYREF
  unsigned long long v10; // [rsp+448h] [rbp-8h]

  v10 = __readfsqword(0x28u);
  dirp = fdopendir(a1);
  if ( dirp )
  {
    ptr = (char *)sub_10C61(dirp, 0);
    if ( !ptr )
      sub_51E7(a2);
    if ( closedir(dirp) )
      sub_51E7(a2);
    if ( ptr )
    {
      for ( s1 = ptr; *s1; s1 += v8 + 1 )
      {
        v5 = strlen(a2);
        v8 = strlen(s1);
        if ( strcmp(s1, ".") && strcmp(s1, "..") )
        {
          if ( v5 + v8 > 0x3FD )
          {
            fprintf(stderr, "%s: %s/%s: pathname too long\n", s, a2, s1);
            dword_DD018 = 1;
          }
          else
          {
            strcpy(dest, a2);
            if ( *(char *)sub_10244(dest) && dest[v5 - 1] != 47 )
            {
              v2 = v5++;
              dest[v2] = 47;
            }
            strcpy(&dest[v5], s1);
            sub_5F11(dest);
          }
        }
      }
      free(ptr);
    }
  }
  else
  {
    sub_51E7(a2);
    close(a1);
  }
  return v10 - __readfsqword(0x28u);
}



// Function: install_signal_handlers @ 0x8c07
unsigned long long install_signal_handlers()
{
  int i; // [rsp+8h] [rbp-A8h]
  int j; // [rsp+8h] [rbp-A8h]
  struct sigaction oact; // [rsp+10h] [rbp-A0h] BYREF
  unsigned long long v4; // [rsp+A8h] [rbp-8h]

  v4 = __readfsqword(0x28u);
  sigemptyset(&set);
  for ( i = 0; i < 6; ++i )
  {
    sigaction(dword_190B0[i], 0, &oact);
    if ( oact.sa_handler != (__sighandler_t)((char *)&dword_0 + 1) )
      sigaddset(&set, dword_190B0[i]);
  }
  oact.sa_handler = (__sighandler_t)sub_8FBB;
  oact.sa_mask = set;
  oact.sa_flags = 0;
  for ( j = 0; j < 6; ++j )
  {
    if ( sigismember(&set, dword_190B0[j]) )
    {
      if ( !j )
        dword_DD898 = 1;
      sigaction(dword_190B0[j], &oact, 0);
    }
  }
  return v4 - __readfsqword(0x28u);
}



// Function: do_exit @ 0x8e40
void do_exit(int a1)
{
  if ( dword_DE210 )
    exit(a1);
  dword_DE210 = 1;
  free(ptr);
  ptr = 0;
  exit(a1);
}



// Function: finish_out @ 0x8e91
void finish_out()
{
  if ( (unsigned int)sub_10304(stdout) )
    sub_F74D();
  sub_8E40(0);
}



// Function: remove_output_file @ 0x8ebb
unsigned long long remove_output_file(char a1)
{
  int fd; // [rsp+1Ch] [rbp-494h]
  sigset_t oset; // [rsp+20h] [rbp-490h] BYREF
  char v4[1032]; // [rsp+A0h] [rbp-410h] BYREF
  unsigned long long v5; // [rsp+4A8h] [rbp-8h]

  v5 = __readfsqword(0x28u);
  if ( a1 != 1 )
    sigprocmask(0, &set, &oset);
  fd = dword_190A4;
  if ( dword_190A4 >= 0 )
  {
    dword_190A4 = -1;
    close(fd);
    sub_67DC(v4, byte_DD960);
    sub_F430(v4);
  }
  if ( a1 != 1 )
    sigprocmask(2, &oset, 0);
  return v5 - __readfsqword(0x28u);
}



// Function: abort_gzip @ 0x8f9f
void abort_gzip()
{
  sub_8EBB(0);
  sub_8E40(1);
}



// Function: abort_gzip_signal @ 0x8fbb
int abort_gzip_signal(int a1)
{
  sub_8EBB(1);
  if ( a1 == dword_DD940 )
    _exit(2);
  signal(a1, 0);
  return raise(a1);
}



// Function: huft_build @ 0x9005
long long huft_build(
        unsigned int *a1,
        unsigned int a2,
        unsigned int a3,
        long long a4,
        long long a5,
        long long *a6,
        unsigned int *a7)
{
  unsigned int *v7; // r13
  unsigned int v8; // r12d
  long long *v9; // rax
  unsigned int i; // ebx
  signed int v12; // r15d
  unsigned int j; // r12d
  int v14; // ebx
  int *v15; // r13
  int *v16; // rax
  long long *v17; // rax
  unsigned int *v18; // r13
  unsigned int v19; // r12d
  int *v20; // rax
  int v21; // ebx
  int v22; // eax
  unsigned int v23; // r12d
  unsigned int *v24; // r13
  char *v25; // r14
  unsigned int v26; // eax
  unsigned int v27; // ebx
  long long *v28; // rax
  long long *v29; // rcx
  char v30; // al
  unsigned int *v31; // rax
  unsigned int m; // ebx
  char *v33; // rcx
  unsigned int n; // ebx
  int v36; // [rsp+Ch] [rbp-654h]
  int v41; // [rsp+34h] [rbp-62Ch]
  int v42; // [rsp+4Ch] [rbp-614h]
  unsigned int v43; // [rsp+50h] [rbp-610h]
  int v44; // [rsp+50h] [rbp-610h]
  unsigned int v45; // [rsp+50h] [rbp-610h]
  int v46; // [rsp+54h] [rbp-60Ch]
  unsigned int v47; // [rsp+58h] [rbp-608h]
  int v48; // [rsp+5Ch] [rbp-604h]
  int v49; // [rsp+5Ch] [rbp-604h]
  int v50; // [rsp+5Ch] [rbp-604h]
  unsigned int v51; // [rsp+60h] [rbp-600h]
  signed int v52; // [rsp+64h] [rbp-5FCh]
  long long *k; // [rsp+68h] [rbp-5F8h]
  int *v54; // [rsp+68h] [rbp-5F8h]
  long long v55; // [rsp+70h] [rbp-5F0h]
  char *v56; // [rsp+78h] [rbp-5E8h]
  int s[20]; // [rsp+80h] [rbp-5E0h] BYREF
  int v58; // [rsp+84h] [rbp-5DCh] BYREF
  int v59[2]; // [rsp+D0h] [rbp-590h]
  long long v60; // [rsp+D8h] [rbp-588h] BYREF
  long long v61[16]; // [rsp+120h] [rbp-540h]
  int v62[290]; // [rsp+1A0h] [rbp-4C0h] BYREF
  unsigned long long v63; // [rsp+628h] [rbp-38h]

  v63 = __readfsqword(0x28u);
  memset(s, 0, 0x44u);
  v7 = a1;
  v8 = a2;
  do
  {
    ++s[*v7++];
    --v8;
  }
  while ( v8 );
  if ( a2 == s[0] )
  {
    v9 = malloc(0x30u);
    if ( v9 )
    {
      dword_DE224 += 3;
      v9[1] = 0;
      *((char *)v9 + 16) = 99;
      *((char *)v9 + 17) = 1;
      *((char *)v9 + 32) = 99;
      *((char *)v9 + 33) = 1;
      *a6 = v9 + 2;
      *a7 = 1;
      return 0;
    }
    else
    {
      return 3;
    }
  }
  else
  {
    v47 = *a7;
    for ( i = 1; i <= 0x10 && !s[i]; ++i )
      ;
    v12 = i;
    if ( i > v47 )
      v47 = i;
    for ( j = 16; j && !s[j]; --j )
      ;
    v52 = j;
    if ( j < v47 )
      v47 = j;
    *a7 = v47;
    v48 = 1 << i;
    while ( i < j )
    {
      v49 = v48 - s[i];
      if ( v49 < 0 )
        return 2;
      ++i;
      v48 = 2 * v49;
    }
    v50 = v48 - s[j];
    if ( v50 >= 0 )
    {
      s[j] += v50;
      v14 = 0;
      v59[1] = 0;
      v15 = &v58;
      for ( k = &v60; --j; k = (long long *)((char *)k + 4) )
      {
        v16 = v15++;
        v14 += *v16;
        v17 = k;
        *(int *)v17 = v14;
      }
      v18 = a1;
      v19 = 0;
      do
      {
        v20 = (int *)v18++;
        v21 = *v20;
        if ( *v20 )
        {
          v22 = v59[v21];
          v59[v21] = v22 + 1;
          v62[v22] = v19;
        }
        ++v19;
      }
      while ( v19 < a2 );
      v41 = v59[v52];
      v23 = 0;
      v59[0] = 0;
      v24 = v62;
      v46 = -1;
      v36 = -v47;
      v61[0] = 0;
      v25 = 0;
      v51 = 0;
      while ( 1 )
      {
        if ( v12 > v52 )
          return v50 && v52 != 1;
        v42 = s[v12];
LABEL_68:
        if ( v42-- )
          break;
        ++v12;
      }
      while ( 1 )
      {
        if ( v12 <= (int)(v36 + v47) )
        {
          if ( v24 < &v62[v41] )
          {
            if ( a3 <= *v24 )
            {
              LOBYTE(v55) = *(short *)(2LL * (*v24 - a3) + a5);
              v31 = v24++;
              LOWORD(v56) = *(short *)(2LL * (*v31 - a3) + a4);
            }
            else
            {
              if ( *v24 > 0xFF )
                v30 = 15;
              else
                v30 = 16;
              LOBYTE(v55) = v30;
              LOWORD(v56) = *v24++;
            }
          }
          else
          {
            LOBYTE(v55) = 99;
          }
          for ( m = v23 >> v36; m < v51; m += 1 << (v12 - v36) )
          {
            v33 = &v25[16 * m];
            BYTE1(v55) = v12 - v36;
            *(long long *)v33 = v55;
            *((long long *)v33 + 1) = v56;
          }
          for ( n = 1 << (v12 - 1); (n & v23) != 0; n >>= 1 )
            v23 ^= n;
          v23 ^= n;
          while ( (v23 & ((1 << v36) - 1)) != v59[v46] )
          {
            --v46;
            v36 -= v47;
          }
          goto LABEL_68;
        }
        ++v46;
        v36 += v47;
        if ( v52 - v36 <= v47 )
          v26 = v52 - v36;
        else
          v26 = v47;
        v27 = v12 - v36;
        v43 = 1 << (v12 - v36);
        if ( v43 > v42 + 1 )
        {
          v44 = v43 - v42 - 1;
          v54 = &s[v12];
          if ( v27 < v26 )
          {
            while ( ++v27 < v26 )
            {
              v45 = 2 * v44;
              if ( v45 <= *++v54 )
                break;
              v44 = v45 - *v54;
            }
          }
        }
        v51 = 1 << v27;
        v28 = malloc(16LL * (unsigned int)((1 << v27) + 1));
        if ( !v28 )
          break;
        dword_DE224 += v51 + 1;
        *a6 = v28 + 2;
        a6 = v28 + 1;
        v28[1] = 0;
        v25 = (char *)(v28 + 2);
        v61[v46] = v28 + 2;
        if ( v46 )
        {
          v59[v46] = v23;
          BYTE1(v55) = v47;
          LOBYTE(v55) = v27 + 16;
          v56 = (char *)(v28 + 2);
          v29 = (long long *)(v61[v46 - 1] + 16LL * (v23 >> (v36 - v47)));
          *v29 = v55;
          v29[1] = v25;
        }
      }
      if ( v46 )
        sub_975B(v61[0]);
      return 3;
    }
    else
    {
      return 2;
    }
  }
}



// Function: huft_free @ 0x975b
long long huft_free(long long a1)
{
  long long *v2; // rbx
  long long v3; // r12

  while ( a1 )
  {
    v2 = (long long *)(a1 - 16);
    v3 = v2[1];
    free(v2);
    a1 = v3;
  }
  return 0;
}



// Function: inflate_codes @ 0x979a
long long inflate_codes(long long a1, long long a2, unsigned int a3, unsigned int a4)
{
  unsigned long long v4; // r13
  unsigned int v5; // ebx
  int v6; // eax
  long long v7; // rax
  unsigned int v8; // r12d
  unsigned int v10; // r12d
  int v11; // eax
  long long v12; // rax
  unsigned int v13; // eax
  int v14; // eax
  long long v15; // rax
  unsigned long long v16; // r13
  unsigned int i; // ebx
  int v18; // eax
  long long v19; // rax
  unsigned int v20; // r12d
  unsigned int v21; // r12d
  int v22; // eax
  long long v23; // rax
  unsigned long long v24; // r13
  unsigned int j; // ebx
  int v26; // eax
  long long v27; // rax
  unsigned int v28; // eax
  unsigned int v29; // r12d
  unsigned int v30; // eax
  unsigned int v31; // eax
  unsigned int v32; // edx
  unsigned int n; // [rsp+24h] [rbp-3Ch]
  unsigned int n_4; // [rsp+28h] [rbp-38h]
  unsigned int v37; // [rsp+2Ch] [rbp-34h]
  int v38; // [rsp+30h] [rbp-30h]
  int v39; // [rsp+34h] [rbp-2Ch]
  unsigned char *v40; // [rsp+38h] [rbp-28h]
  unsigned char *v41; // [rsp+38h] [rbp-28h]

  v4 = qword_DE218;
  v5 = dword_DE220;
  v37 = dword_DD870;
  v38 = word_192E0[a3];
  v39 = word_192E0[a4];
  while ( 1 )
  {
    while ( v5 < a3 )
    {
      if ( dword_DD86C >= (unsigned int)dword_DD868 )
      {
        dword_DD870 = v37;
        v7 = (unsigned char)sub_F0BC(0);
      }
      else
      {
        v6 = dword_DD86C++;
        v7 = (unsigned char)byte_1B000[v6];
      }
      v4 |= v7 << v5;
      v5 += 8;
    }
    v40 = (unsigned char *)(16LL * (v38 & (unsigned int)v4) + a1);
    v8 = *v40;
    if ( v8 > 0x10 )
      break;
LABEL_17:
    v4 >>= v40[1];
    v5 -= v40[1];
    if ( v8 == 16 )
    {
      v13 = v37++;
      byte_AD000[v13] = *((short *)v40 + 4);
      if ( v37 == 0x8000 )
      {
        dword_DD870 = 0x8000;
        sub_F2CC();
        v37 = 0;
      }
    }
    else
    {
      if ( v8 == 15 )
      {
        dword_DD870 = v37;
        qword_DE218 = v4;
        dword_DE220 = v5;
        return 0;
      }
      while ( v5 < v8 )
      {
        if ( dword_DD86C >= (unsigned int)dword_DD868 )
        {
          dword_DD870 = v37;
          v15 = (unsigned char)sub_F0BC(0);
        }
        else
        {
          v14 = dword_DD86C++;
          v15 = (unsigned char)byte_1B000[v14];
        }
        v4 |= v15 << v5;
        v5 += 8;
      }
      n = *((unsigned short *)v40 + 4) + (v4 & word_192E0[v8]);
      v16 = v4 >> v8;
      for ( i = v5 - v8; i < a4; i += 8 )
      {
        if ( dword_DD86C >= (unsigned int)dword_DD868 )
        {
          dword_DD870 = v37;
          v19 = (unsigned char)sub_F0BC(0);
        }
        else
        {
          v18 = dword_DD86C++;
          v19 = (unsigned char)byte_1B000[v18];
        }
        v16 |= v19 << i;
      }
      v41 = (unsigned char *)(16LL * (v39 & (unsigned int)v16) + a2);
      v20 = *v41;
      if ( v20 > 0x10 )
      {
        while ( v20 != 99 )
        {
          v16 >>= v41[1];
          i -= v41[1];
          v21 = v20 - 16;
          while ( i < v21 )
          {
            if ( dword_DD86C >= (unsigned int)dword_DD868 )
            {
              dword_DD870 = v37;
              v23 = (unsigned char)sub_F0BC(0);
            }
            else
            {
              v22 = dword_DD86C++;
              v23 = (unsigned char)byte_1B000[v22];
            }
            v16 |= v23 << i;
            i += 8;
          }
          v41 = (unsigned char *)(*((long long *)v41 + 1) + 16 * (v16 & word_192E0[v21]));
          v20 = *v41;
          if ( v20 <= 0x10 )
            goto LABEL_43;
        }
        return 1;
      }
LABEL_43:
      v24 = v16 >> v41[1];
      for ( j = i - v41[1]; j < v20; j += 8 )
      {
        if ( dword_DD86C >= (unsigned int)dword_DD868 )
        {
          dword_DD870 = v37;
          v27 = (unsigned char)sub_F0BC(0);
        }
        else
        {
          v26 = dword_DD86C++;
          v27 = (unsigned char)byte_1B000[v26];
        }
        v24 |= v27 << j;
      }
      LOWORD(n_4) = v37 - *((short *)v41 + 4) - (word_192E0[v20] & v24);
      v4 = v24 >> v20;
      v5 = j - v20;
      do
      {
        n_4 &= 0x7FFFu;
        if ( n_4 <= v37 )
          v28 = 0x8000 - v37;
        else
          v28 = 0x8000 - n_4;
        if ( v28 > n )
          v28 = n;
        v29 = v28;
        n -= v28;
        if ( n_4 >= v37 )
          v30 = n_4 - v37;
        else
          v30 = v37 - n_4;
        if ( v30 < v29 )
        {
          do
          {
            v31 = n_4++;
            v32 = v37++;
            byte_AD000[v32] = byte_AD000[v31];
            --v29;
          }
          while ( v29 );
        }
        else
        {
          memcpy(&byte_AD000[v37], &byte_AD000[n_4], v29);
          v37 += v29;
          LOWORD(n_4) = v29 + n_4;
        }
        if ( v37 == 0x8000 )
        {
          dword_DD870 = 0x8000;
          sub_F2CC();
          v37 = 0;
        }
      }
      while ( n );
    }
  }
  while ( v8 != 99 )
  {
    v4 >>= v40[1];
    v5 -= v40[1];
    v10 = v8 - 16;
    while ( v5 < v10 )
    {
      if ( dword_DD86C >= (unsigned int)dword_DD868 )
      {
        dword_DD870 = v37;
        v12 = (unsigned char)sub_F0BC(0);
      }
      else
      {
        v11 = dword_DD86C++;
        v12 = (unsigned char)byte_1B000[v11];
      }
      v4 |= v12 << v5;
      v5 += 8;
    }
    v40 = (unsigned char *)(*((long long *)v40 + 1) + 16 * (v4 & word_192E0[v10]));
    v8 = *v40;
    if ( v8 <= 0x10 )
      goto LABEL_17;
  }
  return 1;
}



// Function: inflate_stored @ 0x9d74
long long inflate_stored()
{
  unsigned long long v0; // r12
  unsigned int i; // ebx
  int v2; // eax
  long long v3; // rax
  unsigned long long v4; // r12
  unsigned int j; // ebx
  int v6; // eax
  long long v7; // rax
  unsigned long long v9; // r12
  unsigned int k; // ebx
  int v11; // eax
  long long v12; // rax
  int v13; // eax
  int v15; // [rsp+8h] [rbp-18h]
  int v16; // [rsp+Ch] [rbp-14h]

  v16 = dword_DD870;
  v0 = (unsigned long long)qword_DE218 >> (dword_DE220 & 7);
  for ( i = dword_DE220 - (dword_DE220 & 7); i <= 0xF; i += 8 )
  {
    if ( dword_DD86C >= (unsigned int)dword_DD868 )
    {
      dword_DD870 = v16;
      v3 = (unsigned char)sub_F0BC(0);
    }
    else
    {
      v2 = dword_DD86C++;
      v3 = (unsigned char)byte_1B000[v2];
    }
    v0 |= v3 << i;
  }
  v15 = (unsigned short)v0;
  v4 = v0 >> 16;
  for ( j = i - 16; j <= 0xF; j += 8 )
  {
    if ( dword_DD86C >= (unsigned int)dword_DD868 )
    {
      dword_DD870 = v16;
      v7 = (unsigned char)sub_F0BC(0);
    }
    else
    {
      v6 = dword_DD86C++;
      v7 = (unsigned char)byte_1B000[v6];
    }
    v4 |= v7 << j;
  }
  if ( v15 != (unsigned short)~(short)v4 )
    return 1;
  v9 = v4 >> 16;
  for ( k = j - 16; v15--; k -= 8 )
  {
    while ( k <= 7 )
    {
      if ( dword_DD86C >= (unsigned int)dword_DD868 )
      {
        dword_DD870 = v16;
        v12 = (unsigned char)sub_F0BC(0);
      }
      else
      {
        v11 = dword_DD86C++;
        v12 = (unsigned char)byte_1B000[v11];
      }
      v9 |= v12 << k;
      k += 8;
    }
    v13 = v16++;
    byte_AD000[v13] = v9;
    if ( v16 == 0x8000 )
    {
      dword_DD870 = 0x8000;
      sub_F2CC();
      v16 = 0;
    }
    v9 >>= 8;
  }
  dword_DD870 = v16;
  qword_DE218 = v9;
  dword_DE220 = k;
  return 0;
}



// Function: inflate_fixed @ 0x9f58
long long inflate_fixed()
{
  unsigned int v1; // [rsp+4h] [rbp-4ACh] BYREF
  unsigned int v2; // [rsp+8h] [rbp-4A8h] BYREF
  int i; // [rsp+Ch] [rbp-4A4h]
  long long v4; // [rsp+10h] [rbp-4A0h] BYREF
  long long v5; // [rsp+18h] [rbp-498h] BYREF
  unsigned int v6[290]; // [rsp+20h] [rbp-490h] BYREF
  unsigned long long v7; // [rsp+4A8h] [rbp-8h]

  v7 = __readfsqword(0x28u);
  for ( i = 0; i <= 143; ++i )
    v6[i] = 8;
  while ( i <= 255 )
    v6[i++] = 9;
  while ( i <= 279 )
    v6[i++] = 7;
  while ( i <= 287 )
    v6[i++] = 8;
  v1 = 7;
  i = sub_9005(v6, 0x120u, 0x101u, (long long)&unk_191E0, (long long)&unk_19220, &v4, &v1);
  if ( i )
    return (unsigned int)i;
  for ( i = 0; i <= 29; ++i )
    v6[i] = 5;
  v2 = 5;
  i = sub_9005(v6, 0x1Eu, 0, (long long)&unk_19260, (long long)&unk_192A0, &v5, &v2);
  if ( i <= 1 )
  {
    if ( (unsigned int)sub_979A(v4, v5, v1, v2) )
    {
      return 1;
    }
    else
    {
      sub_975B(v4);
      sub_975B(v5);
      return 0;
    }
  }
  else
  {
    sub_975B(v4);
    return (unsigned int)i;
  }
}



// Function: inflate_dynamic @ 0xa191
long long inflate_dynamic()
{
  unsigned long long v0; // r12
  unsigned int v1; // ebx
  int v2; // eax
  long long v3; // rax
  unsigned long long v4; // r12
  unsigned int i; // ebx
  int v6; // eax
  long long v7; // rax
  unsigned long long v8; // r12
  unsigned int j; // ebx
  int v10; // eax
  long long v11; // rax
  unsigned long long v12; // r12
  unsigned int v13; // ebx
  int v15; // eax
  long long v16; // rax
  int v17; // eax
  long long v18; // rax
  unsigned int v19; // eax
  int v20; // eax
  long long v21; // rax
  unsigned int v22; // eax
  int v24; // eax
  long long v25; // rax
  unsigned int v26; // eax
  int v28; // eax
  long long v29; // rax
  unsigned int v30; // eax
  unsigned int v32; // [rsp+0h] [rbp-550h] BYREF
  unsigned int v33; // [rsp+4h] [rbp-54Ch] BYREF
  unsigned int v34; // [rsp+8h] [rbp-548h]
  unsigned int k; // [rsp+Ch] [rbp-544h]
  unsigned int v36; // [rsp+10h] [rbp-540h]
  int v37; // [rsp+14h] [rbp-53Ch]
  unsigned int v38; // [rsp+18h] [rbp-538h]
  unsigned int v39; // [rsp+1Ch] [rbp-534h]
  unsigned int v40; // [rsp+20h] [rbp-530h]
  unsigned int v41; // [rsp+24h] [rbp-52Ch]
  int v42; // [rsp+28h] [rbp-528h]
  int v43; // [rsp+2Ch] [rbp-524h]
  long long v44; // [rsp+30h] [rbp-520h] BYREF
  long long v45; // [rsp+38h] [rbp-518h] BYREF
  unsigned int v46[318]; // [rsp+40h] [rbp-510h] BYREF
  unsigned long long v47; // [rsp+538h] [rbp-18h]

  v47 = __readfsqword(0x28u);
  v0 = qword_DE218;
  v1 = dword_DE220;
  v37 = dword_DD870;
  while ( v1 <= 4 )
  {
    if ( dword_DD86C >= (unsigned int)dword_DD868 )
    {
      dword_DD870 = v37;
      v3 = (unsigned char)sub_F0BC(0);
    }
    else
    {
      v2 = dword_DD86C++;
      v3 = (unsigned char)byte_1B000[v2];
    }
    v0 |= v3 << v1;
    v1 += 8;
  }
  v38 = (v0 & 0x1F) + 257;
  v4 = v0 >> 5;
  for ( i = v1 - 5; i <= 4; i += 8 )
  {
    if ( dword_DD86C >= (unsigned int)dword_DD868 )
    {
      dword_DD870 = v37;
      v7 = (unsigned char)sub_F0BC(0);
    }
    else
    {
      v6 = dword_DD86C++;
      v7 = (unsigned char)byte_1B000[v6];
    }
    v4 |= v7 << i;
  }
  v39 = (v4 & 0x1F) + 1;
  v8 = v4 >> 5;
  for ( j = i - 5; j <= 3; j += 8 )
  {
    if ( dword_DD86C >= (unsigned int)dword_DD868 )
    {
      dword_DD870 = v37;
      v11 = (unsigned char)sub_F0BC(0);
    }
    else
    {
      v10 = dword_DD86C++;
      v11 = (unsigned char)byte_1B000[v10];
    }
    v8 |= v11 << j;
  }
  v40 = (v8 & 0xF) + 4;
  v12 = v8 >> 4;
  v13 = j - 4;
  if ( v38 > 0x11E || v39 > 0x1E )
    return 1;
  for ( k = 0; k < v40; ++k )
  {
    while ( v13 <= 2 )
    {
      if ( dword_DD86C >= (unsigned int)dword_DD868 )
      {
        dword_DD870 = v37;
        v16 = (unsigned char)sub_F0BC(0);
      }
      else
      {
        v15 = dword_DD86C++;
        v16 = (unsigned char)byte_1B000[v15];
      }
      v12 |= v16 << v13;
      v13 += 8;
    }
    v46[dword_19180[k]] = v12 & 7;
    v12 >>= 3;
    v13 -= 3;
  }
  while ( k <= 0x12 )
    v46[dword_19180[k++]] = 0;
  v32 = 7;
  v34 = sub_9005(v46, 0x13u, 0x13u, 0, 0, &v44, &v32);
  if ( v34 )
  {
    if ( v34 == 1 )
      sub_975B(v44);
    return v34;
  }
  else if ( v44 )
  {
    v41 = v38 + v39;
    v42 = word_192E0[v32];
    v36 = 0;
    v34 = 0;
    while ( v41 > v34 )
    {
      while ( v13 < v32 )
      {
        if ( dword_DD86C >= (unsigned int)dword_DD868 )
        {
          dword_DD870 = v37;
          v18 = (unsigned char)sub_F0BC(0);
        }
        else
        {
          v17 = dword_DD86C++;
          v18 = (unsigned char)byte_1B000[v17];
        }
        v12 |= v18 << v13;
        v13 += 8;
      }
      v45 = 16LL * (v42 & (unsigned int)v12) + v44;
      k = *(unsigned char *)(v45 + 1);
      v12 >>= k;
      v13 -= k;
      if ( *(char *)v45 == 99 )
      {
        sub_975B(v44);
        return 2;
      }
      k = *(unsigned short *)(v45 + 8);
      if ( k > 0xF )
      {
        if ( k == 16 )
        {
          while ( v13 <= 1 )
          {
            if ( dword_DD86C >= (unsigned int)dword_DD868 )
            {
              dword_DD870 = v37;
              v21 = (unsigned char)sub_F0BC(0);
            }
            else
            {
              v20 = dword_DD86C++;
              v21 = (unsigned char)byte_1B000[v20];
            }
            v12 |= v21 << v13;
            v13 += 8;
          }
          k = (v12 & 3) + 3;
          v12 >>= 2;
          v13 -= 2;
          if ( v41 < v34 + k )
            return 1;
          while ( k-- )
          {
            v22 = v34++;
            v46[v22] = v36;
          }
        }
        else if ( k == 17 )
        {
          while ( v13 <= 2 )
          {
            if ( dword_DD86C >= (unsigned int)dword_DD868 )
            {
              dword_DD870 = v37;
              v25 = (unsigned char)sub_F0BC(0);
            }
            else
            {
              v24 = dword_DD86C++;
              v25 = (unsigned char)byte_1B000[v24];
            }
            v12 |= v25 << v13;
            v13 += 8;
          }
          k = (v12 & 7) + 3;
          v12 >>= 3;
          v13 -= 3;
          if ( v41 < v34 + k )
            return 1;
          while ( k-- )
          {
            v26 = v34++;
            v46[v26] = 0;
          }
          v36 = 0;
        }
        else
        {
          while ( v13 <= 6 )
          {
            if ( dword_DD86C >= (unsigned int)dword_DD868 )
            {
              dword_DD870 = v37;
              v29 = (unsigned char)sub_F0BC(0);
            }
            else
            {
              v28 = dword_DD86C++;
              v29 = (unsigned char)byte_1B000[v28];
            }
            v12 |= v29 << v13;
            v13 += 8;
          }
          k = (v12 & 0x7F) + 11;
          v12 >>= 7;
          v13 -= 7;
          if ( v41 < v34 + k )
            return 1;
          while ( k-- )
          {
            v30 = v34++;
            v46[v30] = 0;
          }
          v36 = 0;
        }
      }
      else
      {
        v36 = k;
        v19 = v34++;
        v46[v19] = k;
      }
    }
    sub_975B(v44);
    qword_DE218 = v12;
    dword_DE220 = v13;
    v32 = dword_19304;
    v34 = sub_9005(v46, v38, 0x101u, (long long)&unk_191E0, (long long)&unk_19220, &v44, &v32);
    if ( v34 )
    {
      if ( v34 == 1 )
        sub_975B(v44);
      return v34;
    }
    else
    {
      v33 = dword_19308;
      v34 = sub_9005(&v46[v38], v39, 0, (long long)&unk_19260, (long long)&unk_192A0, &v45, &v33);
      if ( v34 )
      {
        if ( v34 == 1 )
          sub_975B(v45);
        sub_975B(v44);
        return v34;
      }
      else
      {
        v43 = sub_979A(v44, v45, v32, v33) != 0;
        sub_975B(v44);
        sub_975B(v45);
        return v43;
      }
    }
  }
  else
  {
    return 2;
  }
}



// Function: inflate_block @ 0xaa58
long long inflate_block(int *a1)
{
  unsigned long long v1; // r12
  int v2; // ebx
  int v3; // eax
  long long v4; // rax
  unsigned long long v5; // r12
  unsigned int i; // ebx
  int v7; // eax
  long long v8; // rax
  int v10; // [rsp+18h] [rbp-18h]
  int v11; // [rsp+1Ch] [rbp-14h]

  v1 = qword_DE218;
  v2 = dword_DE220;
  v10 = dword_DD870;
  while ( !v2 )
  {
    if ( dword_DD86C >= (unsigned int)dword_DD868 )
    {
      dword_DD870 = v10;
      v4 = (unsigned char)sub_F0BC(0);
    }
    else
    {
      v3 = dword_DD86C++;
      v4 = (unsigned char)byte_1B000[v3];
    }
    v1 |= v4;
    v2 = 8;
  }
  *a1 = v1 & 1;
  v5 = v1 >> 1;
  for ( i = v2 - 1; i <= 1; i += 8 )
  {
    if ( dword_DD86C >= (unsigned int)dword_DD868 )
    {
      dword_DD870 = v10;
      v8 = (unsigned char)sub_F0BC(0);
    }
    else
    {
      v7 = dword_DD86C++;
      v8 = (unsigned char)byte_1B000[v7];
    }
    v5 |= v8 << i;
  }
  v11 = v5 & 3;
  qword_DE218 = v5 >> 2;
  dword_DE220 = i - 2;
  if ( v11 == 2 )
    return sub_A191();
  if ( (v5 & 3) == 0 )
    return sub_9D74();
  if ( v11 == 1 )
    return sub_9F58();
  return 2;
}



// Function: inflate @ 0xab98
long long inflate()
{
  int v1; // [rsp+Ch] [rbp-14h] BYREF
  unsigned int v2; // [rsp+10h] [rbp-10h]
  unsigned int v3; // [rsp+14h] [rbp-Ch]
  unsigned long long v4; // [rsp+18h] [rbp-8h]

  v4 = __readfsqword(0x28u);
  dword_DD870 = 0;
  dword_DE220 = 0;
  qword_DE218 = 0;
  v2 = 0;
  do
  {
    dword_DE224 = 0;
    v3 = sub_AA58(&v1);
    if ( v3 )
      return v3;
    if ( v2 < dword_DE224 )
      v2 = dword_DE224;
  }
  while ( !v1 );
  while ( (unsigned int)dword_DE220 > 7 )
  {
    dword_DE220 -= 8;
    --dword_DD86C;
  }
  sub_F2CC();
  return 0;
}



// Function: ct_init @ 0xac6f
long long ct_init(long long a1, long long a2)
{
  long long result; // rax
  int v3; // eax
  int v4; // eax
  int v5; // eax
  int v6; // eax
  int v7; // eax
  int v8; // eax
  int v9; // eax
  long long v10; // rsi
  unsigned long long v11; // rdi
  int j; // [rsp+1Ch] [rbp-14h]
  int m; // [rsp+1Ch] [rbp-14h]
  int n; // [rsp+1Ch] [rbp-14h]
  int jj; // [rsp+1Ch] [rbp-14h]
  signed int kk; // [rsp+1Ch] [rbp-14h]
  int ii; // [rsp+20h] [rbp-10h]
  int v18; // [rsp+24h] [rbp-Ch]
  int i; // [rsp+28h] [rbp-8h]
  int k; // [rsp+28h] [rbp-8h]
  int v21; // [rsp+2Ch] [rbp-4h]
  int v22; // [rsp+2Ch] [rbp-4h]

  qword_E1170 = a1;
  qword_E1178 = a2;
  qword_E1168 = 0;
  qword_E1160 = 0;
  result = word_DF0C2[0];
  if ( !word_DF0C2[0] )
  {
    v18 = 0;
    for ( i = 0; i <= 27; ++i )
    {
      dword_E0040[i] = v18;
      for ( j = 0; j < 1 << dword_19320[i]; ++j )
      {
        v3 = v18++;
        byte_DFD40[v3] = i;
      }
    }
    byte_DFD40[v18 - 1] = i;
    v21 = 0;
    for ( k = 0; k <= 15; ++k )
    {
      dword_E00C0[k] = v21;
      for ( m = 0; m < 1 << dword_193A0[k]; ++m )
      {
        v4 = v21++;
        byte_DFE40[v4] = k;
      }
    }
    v22 = v21 >> 7;
    while ( k <= 29 )
    {
      dword_E00C0[k] = v22 << 7;
      for ( n = 0; n < 1 << (LOBYTE(dword_193A0[k]) - 7); ++n )
      {
        v5 = v22++;
        byte_DFE40[v5 + 256] = k;
      }
      ++k;
    }
    for ( ii = 0; ii <= 15; ++ii )
      word_DF1E0[ii] = 0;
    for ( jj = 0; jj <= 143; ++jj )
    {
      v6 = jj;
      word_DEC42[2 * v6] = 8;
      ++word_DF1F0;
    }
    while ( jj <= 255 )
    {
      v7 = jj++;
      word_DEC42[2 * v7] = 9;
      ++word_DF1F2;
    }
    while ( jj <= 279 )
    {
      v8 = jj++;
      word_DEC42[2 * v8] = 7;
      ++word_DF1EE;
    }
    while ( jj <= 287 )
    {
      v9 = jj++;
      word_DEC42[2 * v9] = 8;
      ++word_DF1F0;
    }
    v10 = 287;
    v11 = (unsigned long long)&unk_DEC40;
    sub_B76A(&unk_DEC40, 287);
    for ( kk = 0; kk <= 29; ++kk )
    {
      word_DF0C2[2 * kk] = 5;
      v10 = 5;
      v11 = (unsigned int)kk;
      *((short *)&unk_DF0C0 + 2 * kk) = sub_3BED(kk, 5);
    }
    return sub_AFF6(v11, v10);
  }
  return result;
}



// Function: init_block @ 0xaff6
long long init_block()
{
  int i; // [rsp+0h] [rbp-4h]
  int j; // [rsp+0h] [rbp-4h]
  int k; // [rsp+0h] [rbp-4h]

  for ( i = 0; i <= 285; ++i )
    word_DE240[2 * i] = 0;
  for ( j = 0; j <= 29; ++j )
    word_DEB40[2 * j] = 0;
  for ( k = 0; k <= 18; ++k )
    *((short *)&unk_DF140 + 2 * k) = 0;
  word_DE640 = 1;
  qword_E1158 = 0;
  qword_E1150 = 0;
  dword_E1148 = 0;
  dword_E1144 = 0;
  dword_E1140 = 0;
  byte_E114C = 0;
  byte_E114D = 1;
  return 0;
}



// Function: pqdownheap @ 0xb0dd
long long pqdownheap(long long a1, int a2)
{
  int v3; // [rsp+0h] [rbp-1Ch]
  int i; // [rsp+14h] [rbp-8h]
  unsigned int v5; // [rsp+18h] [rbp-4h]

  v3 = a2;
  v5 = *((int *)&unk_DF200 + a2);
  for ( i = 2 * a2; i <= dword_DFAF4; i *= 2 )
  {
    if ( i < dword_DFAF4
      && (*(short *)(4LL * *((int *)&unk_DF200 + i + 1) + a1) < *(short *)(4LL * *((int *)&unk_DF200 + i) + a1)
       || *(short *)(4LL * *((int *)&unk_DF200 + i + 1) + a1) == *(short *)(4LL * *((int *)&unk_DF200 + i) + a1)
       && byte_DFB00[*((int *)&unk_DF200 + i + 1)] <= byte_DFB00[*((int *)&unk_DF200 + i)]) )
    {
      ++i;
    }
    if ( *(short *)(4LL * (int)v5 + a1) < *(short *)(4LL * *((int *)&unk_DF200 + i) + a1)
      || *(short *)(4LL * (int)v5 + a1) == *(short *)(4LL * *((int *)&unk_DF200 + i) + a1)
      && byte_DFB00[v5] <= byte_DFB00[*((int *)&unk_DF200 + i)] )
    {
      break;
    }
    *((int *)&unk_DF200 + v3) = *((int *)&unk_DF200 + i);
    v3 = i;
  }
  *((int *)&unk_DF200 + v3) = v5;
  return v5;
}



// Function: gen_bitlen @ 0xb363
unsigned long long gen_bitlen(long long *a1)
{
  unsigned long long result; // rax
  long long v2; // rdx
  unsigned short v3; // [rsp+Ah] [rbp-3Eh]
  int j; // [rsp+Ch] [rbp-3Ch]
  signed int v5; // [rsp+10h] [rbp-38h]
  int v6; // [rsp+10h] [rbp-38h]
  int i; // [rsp+14h] [rbp-34h]
  int v8; // [rsp+14h] [rbp-34h]
  int k; // [rsp+14h] [rbp-34h]
  int m; // [rsp+14h] [rbp-34h]
  int v11; // [rsp+18h] [rbp-30h]
  int v12; // [rsp+1Ch] [rbp-2Ch]
  int v13; // [rsp+20h] [rbp-28h]
  int v14; // [rsp+24h] [rbp-24h]
  int v15; // [rsp+28h] [rbp-20h]
  unsigned int v16; // [rsp+2Ch] [rbp-1Ch]
  long long v17; // [rsp+30h] [rbp-18h]
  long long v18; // [rsp+38h] [rbp-10h]
  long long v19; // [rsp+40h] [rbp-8h]

  v17 = *a1;
  v18 = a1[2];
  v13 = *((int *)a1 + 6);
  v14 = *((int *)a1 + 9);
  v15 = *((int *)a1 + 8);
  v19 = a1[1];
  v12 = 0;
  for ( i = 0; i <= 15; ++i )
    word_DF1E0[i] = 0;
  *(short *)(4LL * dword_DF200[dword_DFAF8] + v17 + 2) = 0;
  result = (unsigned int)(dword_DFAF8 + 1);
  for ( j = dword_DFAF8 + 1; j <= 572; ++j )
  {
    v5 = dword_DF200[j];
    v8 = *(unsigned short *)(4LL * *(unsigned short *)(4LL * v5 + v17 + 2) + v17 + 2) + 1;
    if ( v8 > v15 )
    {
      v8 = v15;
      ++v12;
    }
    *(short *)(4LL * v5 + v17 + 2) = v8;
    result = (unsigned int)v5;
    if ( v5 <= v14 )
    {
      ++word_DF1E0[v8];
      v11 = 0;
      if ( v5 >= v13 )
        v11 = *(int *)(4LL * (v5 - v13) + v18);
      v3 = *(short *)(4LL * v5 + v17);
      result = (v8 + v11) * (unsigned long long)v3 + qword_E1150;
      qword_E1150 = result;
      if ( v19 )
      {
        result = (*(unsigned short *)(4LL * v5 + v19 + 2) + v11) * (unsigned long long)v3 + qword_E1158;
        qword_E1158 = result;
      }
    }
  }
  if ( v12 )
  {
    do
    {
      for ( k = v15 - 1; !word_DF1E0[k]; --k )
        ;
      --word_DF1E0[k];
      word_DF1E0[k + 1] += 2;
      --word_DF1E0[v15];
      v12 -= 2;
    }
    while ( v12 > 0 );
    result = (unsigned int)v15;
    for ( m = v15; m; --m )
    {
      v2 = m;
      result = (unsigned short)word_DF1E0[v2];
      v6 = (unsigned short)word_DF1E0[v2];
      while ( v6 )
      {
        v16 = dword_DF200[--j];
        result = v16;
        if ( (int)v16 <= v14 )
        {
          result = (unsigned int)m;
          if ( *(unsigned short *)(4LL * (int)v16 + v17 + 2) != m )
          {
            qword_E1150 += (m - (unsigned long long)*(unsigned short *)(4LL * (int)v16 + v17 + 2))
                         * *(unsigned short *)(4LL * (int)v16 + v17);
            result = 4LL * (int)v16 + v17;
            *(short *)(result + 2) = m;
          }
          --v6;
        }
      }
    }
  }
  return result;
}



// Function: gen_codes @ 0xb76a
unsigned long long gen_codes(long long a1, int a2)
{
  long long v2; // rdx
  unsigned short v3; // ax
  short v5; // [rsp+12h] [rbp-3Eh]
  int i; // [rsp+14h] [rbp-3Ch]
  int j; // [rsp+18h] [rbp-38h]
  int v8; // [rsp+1Ch] [rbp-34h]
  short v9[20]; // [rsp+20h] [rbp-30h]
  unsigned long long v10; // [rsp+48h] [rbp-8h]

  v10 = __readfsqword(0x28u);
  v5 = 0;
  for ( i = 1; i <= 15; ++i )
  {
    v5 = 2 * (word_DF1E0[i - 1] + v5);
    v9[i] = v5;
  }
  for ( j = 0; j <= a2; ++j )
  {
    v2 = 4LL * j;
    v8 = *(unsigned short *)(v2 + a1 + 2);
    if ( *(short *)(v2 + a1 + 2) )
    {
      v3 = v9[*(unsigned short *)(v2 + a1 + 2)];
      v9[*(unsigned short *)(v2 + a1 + 2)] = v3 + 1;
      *(short *)(4LL * j + a1) = sub_3BED(v3, v8);
    }
  }
  return v10 - __readfsqword(0x28u);
}



// Function: build_tree @ 0xb868
unsigned long long build_tree(long long a1)
{
  int v1; // edx
  int v2; // eax
  int v3; // eax
  char v4; // al
  long long v5; // rax
  int i; // [rsp+18h] [rbp-28h]
  int j; // [rsp+18h] [rbp-28h]
  int v9; // [rsp+18h] [rbp-28h]
  int v10; // [rsp+1Ch] [rbp-24h]
  int v11; // [rsp+20h] [rbp-20h]
  int v12; // [rsp+24h] [rbp-1Ch]
  int v13; // [rsp+28h] [rbp-18h]
  int v14; // [rsp+2Ch] [rbp-14h]
  long long v15; // [rsp+30h] [rbp-10h]
  long long v16; // [rsp+38h] [rbp-8h]

  v15 = *(long long *)a1;
  v16 = *(long long *)(a1 + 8);
  v12 = *(int *)(a1 + 28);
  v10 = -1;
  v11 = v12;
  dword_DFAF4 = 0;
  dword_DFAF8 = 573;
  for ( i = 0; i < v12; ++i )
  {
    if ( *(short *)(4LL * i + v15) )
    {
      v10 = i;
      dword_DF200[++dword_DFAF4] = i;
      byte_DFB00[i] = 0;
    }
    else
    {
      *(short *)(4LL * i + v15 + 2) = 0;
    }
  }
  while ( dword_DFAF4 <= 1 )
  {
    if ( v10 > 1 )
      v1 = 0;
    else
      v1 = ++v10;
    v2 = ++dword_DFAF4;
    dword_DF200[dword_DFAF4] = v1;
    v14 = dword_DF200[v2];
    *(short *)(4LL * v14 + v15) = 1;
    byte_DFB00[v14] = 0;
    --qword_E1150;
    if ( v16 )
      qword_E1158 -= *(unsigned short *)(4LL * v14 + v16 + 2);
  }
  *(int *)(a1 + 36) = v10;
  for ( j = dword_DFAF4 / 2; j > 0; --j )
    sub_B0DD(v15, j);
  do
  {
    v9 = dword_DF204;
    v3 = dword_DFAF4--;
    dword_DF204 = dword_DF200[v3];
    sub_B0DD(v15, 1);
    v13 = dword_DF204;
    dword_DF200[--dword_DFAF8] = v9;
    dword_DF200[--dword_DFAF8] = v13;
    *(short *)(4LL * v11 + v15) = *(short *)(4LL * v9 + v15) + *(short *)(4LL * v13 + v15);
    if ( byte_DFB00[v9] < byte_DFB00[v13] )
      v4 = byte_DFB00[v13];
    else
      v4 = byte_DFB00[v9];
    byte_DFB00[v11] = v4 + 1;
    v5 = 4LL * v13 + v15;
    *(short *)(v5 + 2) = v11;
    *(short *)(4LL * v9 + v15 + 2) = *(short *)(v5 + 2);
    LODWORD(v5) = v11++;
    dword_DF204 = v5;
    sub_B0DD(v15, 1);
  }
  while ( dword_DFAF4 > 1 );
  dword_DF200[--dword_DFAF8] = dword_DF204;
  sub_B363((long long *)a1);
  return sub_B76A(v15, v10);
}



// Function: scan_tree @ 0xbc5c
long long scan_tree(long long a1, int a2)
{
  long long result; // rax
  unsigned int i; // [rsp+10h] [rbp-1Ch]
  int v4; // [rsp+14h] [rbp-18h]
  int v5; // [rsp+18h] [rbp-14h]
  int v6; // [rsp+1Ch] [rbp-10h]
  int v7; // [rsp+20h] [rbp-Ch]
  int v8; // [rsp+24h] [rbp-8h]
  int v9; // [rsp+28h] [rbp-4h]

  v4 = -1;
  v5 = *(unsigned short *)(a1 + 2);
  v6 = 0;
  v7 = 7;
  v8 = 4;
  if ( !*(short *)(a1 + 2) )
  {
    v7 = 138;
    v8 = 3;
  }
  *(short *)(4 * (a2 + 1LL) + a1 + 2) = -1;
  for ( i = 0; ; ++i )
  {
    result = i;
    if ( (int)i > a2 )
      break;
    v9 = v5;
    v5 = *(unsigned short *)(4 * ((int)i + 1LL) + a1 + 2);
    if ( ++v6 >= v7 || v9 != v5 )
    {
      if ( v6 >= v8 )
      {
        if ( v9 )
        {
          if ( v9 != v4 )
            ++*((short *)&unk_DF140 + 2 * v9);
          ++word_DF180;
        }
        else if ( v6 > 10 )
        {
          ++word_DF188;
        }
        else
        {
          ++word_DF184;
        }
      }
      else
      {
        *((short *)&unk_DF140 + 2 * v9) += v6;
      }
      v6 = 0;
      v4 = v9;
      if ( v5 )
      {
        if ( v9 == v5 )
        {
          v7 = 6;
          v8 = 3;
        }
        else
        {
          v7 = 7;
          v8 = 4;
        }
      }
      else
      {
        v7 = 138;
        v8 = 3;
      }
    }
  }
  return result;
}



// Function: send_tree @ 0xbe31
long long send_tree(long long a1, int a2)
{
  long long result; // rax
  unsigned int i; // [rsp+14h] [rbp-1Ch]
  int v4; // [rsp+18h] [rbp-18h]
  int v5; // [rsp+1Ch] [rbp-14h]
  int v6; // [rsp+20h] [rbp-10h]
  int v7; // [rsp+24h] [rbp-Ch]
  int v8; // [rsp+28h] [rbp-8h]
  int v9; // [rsp+2Ch] [rbp-4h]

  v4 = -1;
  v5 = *(unsigned short *)(a1 + 2);
  v6 = 0;
  v7 = 7;
  v8 = 4;
  if ( !*(short *)(a1 + 2) )
  {
    v7 = 138;
    v8 = 3;
  }
  for ( i = 0; ; ++i )
  {
    result = i;
    if ( (int)i > a2 )
      break;
    v9 = v5;
    v5 = *(unsigned short *)(4 * ((int)i + 1LL) + a1 + 2);
    if ( ++v6 >= v7 || v9 != v5 )
    {
      if ( v6 >= v8 )
      {
        if ( v9 )
        {
          if ( v9 != v4 )
          {
            sub_3A6C(*((unsigned short *)&unk_DF140 + 2 * v9), word_DF142[2 * v9]);
            --v6;
          }
          sub_3A6C((unsigned short)word_DF180, (unsigned short)word_DF182);
          sub_3A6C(v6 - 3, 2);
        }
        else if ( v6 > 10 )
        {
          sub_3A6C((unsigned short)word_DF188, (unsigned short)word_DF18A);
          sub_3A6C(v6 - 11, 7);
        }
        else
        {
          sub_3A6C((unsigned short)word_DF184, (unsigned short)word_DF186);
          sub_3A6C(v6 - 3, 3);
        }
      }
      else
      {
        do
        {
          sub_3A6C(*((unsigned short *)&unk_DF140 + 2 * v9), word_DF142[2 * v9]);
          --v6;
        }
        while ( v6 );
      }
      v6 = 0;
      v4 = v9;
      if ( v5 )
      {
        if ( v9 == v5 )
        {
          v7 = 6;
          v8 = 3;
        }
        else
        {
          v7 = 7;
          v8 = 4;
        }
      }
      else
      {
        v7 = 138;
        v8 = 3;
      }
    }
  }
  return result;
}



// Function: build_bl_tree @ 0xc06c
long long build_bl_tree()
{
  int i; // [rsp+Ch] [rbp-4h]

  sub_BC5C((long long)word_DE240, dword_194C4);
  sub_BC5C((long long)word_DEB40, dword_19504);
  sub_B868((long long)&off_19520);
  for ( i = 18; i > 2 && !word_DF142[2 * byte_19470[i]]; --i )
    ;
  qword_E1150 += 3 * (i + 1) + 14;
  return (unsigned int)i;
}



// Function: send_all_trees @ 0xc120
long long send_all_trees(int a1, int a2, int a3)
{
  int i; // [rsp+1Ch] [rbp-4h]

  sub_3A6C(a1 - 257, 5);
  sub_3A6C(a2 - 1, 5);
  sub_3A6C(a3 - 4, 4);
  for ( i = 0; i < a3; ++i )
    sub_3A6C(word_DF142[2 * byte_19470[i]], 3);
  sub_BE31((long long)word_DE240, a1 - 1);
  return sub_BE31((long long)word_DEB40, a2 - 1);
}



// Function: flush_block @ 0xc1ea
long long flush_block(char *a1, long long a2, int a3, int a4)
{
  int v7; // [rsp+2Ch] [rbp-14h]
  unsigned long long v8; // [rsp+30h] [rbp-10h]

  byte_E0140[dword_E1148] = byte_E114C;
  if ( *(short *)qword_E1170 == 0xFFFF )
    sub_C97C();
  sub_B868((long long)&off_194A0);
  sub_B868((long long)&off_194E0);
  v7 = sub_C06C();
  v8 = (unsigned long long)(qword_E1150 + 10) >> 3;
  qword_E1168 += a2;
  if ( (unsigned long long)(qword_E1158 + 10) >> 3 <= v8 )
    v8 = (unsigned long long)(qword_E1158 + 10) >> 3;
  if ( v8 >= a2 + 4 && a1 )
  {
    sub_3A6C(a4, 3);
    qword_E1160 = (qword_E1160 + 10) & 0xFFFFFFFFFFFFFFF8LL;
    qword_E1160 += 8 * (a2 + 4);
    sub_3D59(a1, a2, 1);
  }
  else if ( (unsigned long long)(qword_E1158 + 10) >> 3 == v8 )
  {
    sub_3A6C(a4 + 2, 3);
    sub_C6F9(&unk_DEC40, &unk_DF0C0);
    qword_E1160 += qword_E1158 + 3;
  }
  else
  {
    sub_3A6C(a4 + 4, 3);
    sub_C120(dword_194C4 + 1, dword_19504 + 1, v7 + 1);
    sub_C6F9(word_DE240, word_DEB40);
    qword_E1160 += qword_E1150 + 3;
  }
  sub_AFF6();
  if ( a4 )
  {
    sub_3C22();
    qword_E1160 += 7;
  }
  else if ( a3 && (qword_E1160 & 7) != 0 )
  {
    sub_3A6C(0, 3);
    qword_E1160 = (qword_E1160 + 10) & 0xFFFFFFFFFFFFFFF8LL;
    sub_3D59(a1, 0, 1);
  }
  return qword_E1160 >> 3;
}



// Function: ct_tally @ 0xc45d
long long ct_tally(int a1, int a2)
{
  int v2; // eax
  int v3; // eax
  int v4; // eax
  unsigned int v5; // eax
  int v6; // edx
  int v8; // [rsp+4h] [rbp-24h]
  int i; // [rsp+14h] [rbp-14h]
  unsigned long long v10; // [rsp+18h] [rbp-10h]
  unsigned long long v11; // [rsp+18h] [rbp-10h]

  v2 = dword_E1140++;
  byte_1B000[v2] = a2;
  if ( a1 )
  {
    v8 = a1 - 1;
    v3 = (unsigned char)byte_DFD40[a2] + 257;
    ++word_DE240[2 * v3];
    if ( a1 - 1 > 255 )
      v4 = (unsigned char)byte_DFE40[(v8 >> 7) + 256];
    else
      v4 = (unsigned char)byte_DFE40[v8];
    ++word_DEB40[2 * v4];
    v5 = dword_E1144++;
    word_9C800[v5] = v8;
    byte_E114C |= byte_E114D;
  }
  else
  {
    ++word_DE240[2 * a2];
  }
  byte_E114D *= 2;
  if ( (dword_E1140 & 7) == 0 )
  {
    v6 = dword_E1148++;
    byte_E0140[v6] = byte_E114C;
    byte_E114C = 0;
    byte_E114D = 1;
  }
  if ( dword_190A0 > 2 && (dword_E1140 & 0xFFF) == 0 )
  {
    v10 = 8LL * (unsigned int)dword_E1140;
    for ( i = 0; i <= 29; ++i )
      v10 += ((int)dword_193A0[i] + 5LL) * (unsigned short)word_DEB40[2 * i];
    v11 = v10 >> 3;
    if ( (unsigned int)dword_E1140 >> 1 > dword_E1144
      && v11 < ((unsigned long long)(unsigned int)dword_1A074 - qword_1A068) >> 1 )
    {
      return 1;
    }
  }
  return dword_E1140 == 0x7FFF || dword_E1144 == 0x8000;
}



// Function: compress_block @ 0xc6f9
long long compress_block(long long a1, long long a2)
{
  int v2; // eax
  unsigned int v3; // eax
  int v4; // eax
  unsigned int v5; // eax
  unsigned char i; // [rsp+13h] [rbp-1Dh]
  unsigned int v8; // [rsp+14h] [rbp-1Ch]
  int v9; // [rsp+18h] [rbp-18h]
  int v10; // [rsp+1Ch] [rbp-14h]
  int v11; // [rsp+20h] [rbp-10h]
  int v12; // [rsp+24h] [rbp-Ch]
  unsigned int v13; // [rsp+24h] [rbp-Ch]
  int v14; // [rsp+28h] [rbp-8h]
  int v15; // [rsp+28h] [rbp-8h]
  unsigned int v16; // [rsp+2Ch] [rbp-4h]

  v8 = 0;
  v9 = 0;
  v10 = 0;
  for ( i = 0; v8 < dword_E1140; i >>= 1 )
  {
    if ( (v8 & 7) == 0 )
    {
      v2 = v10++;
      i = byte_E0140[v2];
    }
    v3 = v8++;
    v11 = (unsigned char)byte_1B000[v3];
    if ( (i & 1) != 0 )
    {
      v12 = (unsigned char)byte_DFD40[(unsigned char)byte_1B000[v3]];
      sub_3A6C(
        *(unsigned short *)(4LL * (unsigned int)(v12 + 257) + a1),
        *(unsigned short *)(4LL * (unsigned int)(v12 + 257) + a1 + 2));
      v14 = dword_19320[v12];
      if ( v14 )
        sub_3A6C(v11 - dword_E0040[v12], v14);
      v4 = v9++;
      v16 = (unsigned short)word_9C800[v4];
      if ( v16 > 0xFF )
        v5 = (unsigned char)byte_DFE40[(v16 >> 7) + 256];
      else
        v5 = (unsigned char)byte_DFE40[(unsigned short)word_9C800[v4]];
      v13 = v5;
      sub_3A6C(*(unsigned short *)(4LL * v5 + a2), *(unsigned short *)(4LL * v5 + a2 + 2));
      v15 = dword_193A0[v13];
      if ( v15 )
        sub_3A6C(v16 - dword_E00C0[v13], v15);
    }
    else
    {
      sub_3A6C(
        *(unsigned short *)(4LL * (unsigned char)byte_1B000[v3] + a1),
        *(unsigned short *)(4LL * (unsigned char)byte_1B000[v3] + a1 + 2));
    }
  }
  return sub_3A6C(*(unsigned short *)(a1 + 1024), *(unsigned short *)(a1 + 1026));
}



// Function: set_file_type @ 0xc97c
long long set_file_type()
{
  int v0; // eax
  int v1; // eax
  int v2; // eax
  long long result; // rax
  int v4; // [rsp+0h] [rbp-Ch]
  unsigned int v5; // [rsp+4h] [rbp-8h]
  unsigned int v6; // [rsp+8h] [rbp-4h]

  v4 = 0;
  v5 = 0;
  v6 = 0;
  while ( v4 <= 6 )
  {
    v0 = v4++;
    v6 += (unsigned short)word_DE240[2 * v0];
  }
  while ( v4 <= 127 )
  {
    v1 = v4++;
    v5 += (unsigned short)word_DE240[2 * v1];
  }
  while ( v4 <= 255 )
  {
    v2 = v4++;
    v6 += (unsigned short)word_DE240[2 * v2];
  }
  result = qword_E1170;
  *(short *)qword_E1170 = v6 <= v5 >> 2;
  return result;
}



// Function: fillbuf @ 0xca3c
long long fillbuf(int a1)
{
  int v1; // eax
  int v2; // eax
  long long result; // rax
  int v4; // [rsp+Ch] [rbp-4h]

  v4 = a1;
  word_E13C0 = (unsigned short)word_E13C0 << a1;
  while ( v4 > dword_E13C8 )
  {
    v4 -= dword_E13C8;
    word_E13C0 |= dword_E13C4 << v4;
    if ( dword_DD86C >= (unsigned int)dword_DD868 )
    {
      v2 = sub_F0BC(1);
    }
    else
    {
      v1 = dword_DD86C++;
      v2 = (unsigned char)byte_1B000[v1];
    }
    dword_E13C4 = v2;
    if ( v2 == -1 )
      dword_E13C4 = 0;
    dword_E13C8 = 8;
  }
  dword_E13C8 -= v4;
  result = ((unsigned int)dword_E13C4 >> dword_E13C8) | (unsigned short)word_E13C0;
  word_E13C0 |= (unsigned int)dword_E13C4 >> dword_E13C8;
  return result;
}



// Function: getbits @ 0xcb39
long long getbits(int a1)
{
  unsigned int v2; // [rsp+1Ch] [rbp-4h]

  v2 = (int)(unsigned short)word_E13C0 >> (16 - a1);
  sub_CA3C(a1);
  return v2;
}



// Function: init_getbits @ 0xcb74
long long init_getbits()
{
  word_E13C0 = 0;
  dword_E13C4 = 0;
  dword_E13C8 = 0;
  return sub_CA3C(16);
}



// Function: make_table @ 0xcba6
unsigned long long make_table(unsigned int a1, long long a2, unsigned int a3, long long a4)
{
  unsigned int v4; // eax
  unsigned short v5; // ax
  long long v6; // rdx
  unsigned int i; // [rsp+28h] [rbp-B8h]
  unsigned int j; // [rsp+28h] [rbp-B8h]
  unsigned int k; // [rsp+28h] [rbp-B8h]
  unsigned int m; // [rsp+28h] [rbp-B8h]
  int v13; // [rsp+28h] [rbp-B8h]
  unsigned int jj; // [rsp+28h] [rbp-B8h]
  int ii; // [rsp+28h] [rbp-B8h]
  unsigned int v16; // [rsp+2Ch] [rbp-B4h]
  unsigned int n; // [rsp+30h] [rbp-B0h]
  unsigned int v18; // [rsp+34h] [rbp-ACh]
  char v19; // [rsp+38h] [rbp-A8h]
  int v20; // [rsp+3Ch] [rbp-A4h]
  unsigned int v21; // [rsp+40h] [rbp-A0h]
  unsigned int v22; // [rsp+44h] [rbp-9Ch]
  unsigned short *v23; // [rsp+48h] [rbp-98h]
  short v24[68]; // [rsp+50h] [rbp-90h]
  unsigned long long v25; // [rsp+D8h] [rbp-8h]

  v25 = __readfsqword(0x28u);
  for ( i = 1; i <= 0x10; ++i )
    v24[i] = 0;
  for ( j = 0; j < a1; ++j )
    ++v24[*(unsigned char *)(j + a2)];
  v24[49] = 0;
  for ( k = 1; k <= 0x10; ++k )
    v24[k + 49] = v24[k + 48] + ((unsigned short)v24[k] << (16 - k));
  if ( v24[65] )
    sub_F5EF("Bad table\n");
  v19 = 16 - a3;
  for ( m = 1; m <= a3; ++m )
  {
    v24[m + 48] = (int)(unsigned short)v24[m + 48] >> v19;
    v24[m + 24] = 1 << (a3 - m);
  }
  while ( m <= 0x10 )
  {
    v24[m + 24] = 1 << (16 - m);
    ++m;
  }
  v13 = (int)(unsigned short)v24[a3 + 1 + 48] >> v19;
  if ( v13 )
  {
    while ( v13 != 1 << a3 )
    {
      v4 = v13++;
      *(short *)(2LL * v4 + a4) = 0;
    }
  }
  v18 = a1;
  v20 = 1 << (15 - a3);
  for ( n = 0; n < a1; ++n )
  {
    v21 = *(unsigned char *)(n + a2);
    if ( *(char *)(n + a2) )
    {
      v22 = (unsigned short)v24[*(unsigned char *)(n + a2) + 48]
          + (unsigned short)v24[*(unsigned char *)(n + a2) + 24];
      if ( v21 > a3 )
      {
        v16 = (unsigned short)v24[*(unsigned char *)(n + a2) + 48];
        v23 = (unsigned short *)(2LL * (v16 >> v19) + a4);
        for ( ii = v21 - a3; ii; --ii )
        {
          if ( !*v23 )
          {
            word_BD000[v18] = 0;
            word_BD000[v18 + 0x8000] = word_BD000[v18];
            v5 = v18++;
            *v23 = v5;
          }
          if ( (v20 & v16) != 0 )
            v6 = *v23 + 0x8000LL;
          else
            v6 = *v23;
          v23 = &word_BD000[v6];
          v16 *= 2;
        }
        *v23 = n;
      }
      else
      {
        if ( v22 > 1 << a3 )
          sub_F5EF("Bad table\n");
        for ( jj = (unsigned short)v24[*(unsigned char *)(n + a2) + 48]; jj < v22; ++jj )
          *(short *)(2LL * jj + a4) = n;
      }
      v24[v21 + 48] = v22;
    }
  }
  return v25 - __readfsqword(0x28u);
}



// Function: read_pt_len @ 0xd0a0
unsigned long long read_pt_len(signed int a1, int a2, int a3)
{
  unsigned long long result; // rax
  int v4; // eax
  signed int v5; // eax
  signed int v6; // eax
  signed int v7; // eax
  signed int j; // [rsp+10h] [rbp-10h]
  int k; // [rsp+10h] [rbp-10h]
  signed int v11; // [rsp+10h] [rbp-10h]
  short v12; // [rsp+14h] [rbp-Ch]
  int v13; // [rsp+14h] [rbp-Ch]
  int v14; // [rsp+14h] [rbp-Ch]
  unsigned int i; // [rsp+18h] [rbp-8h]
  signed int v16; // [rsp+1Ch] [rbp-4h]

  v16 = sub_CB39(a2);
  if ( v16 )
  {
    v11 = 0;
    while ( v11 < v16 )
    {
      v13 = (unsigned short)word_E13C0 >> 13;
      if ( v13 == 7 )
      {
        for ( i = 4096; ((unsigned short)i & (unsigned short)word_E13C0) != 0; i >>= 1 )
          ++v13;
        if ( v13 > 16 )
          sub_F5EF("Bad table\n");
      }
      v4 = v13;
      if ( v13 < 6 )
        v4 = 6;
      sub_CA3C(v4 - 3);
      v5 = v11++;
      byte_E1180[v5] = v13;
      if ( v11 == a3 )
      {
        v14 = sub_CB39(2);
        while ( --v14 >= 0 )
        {
          v6 = v11++;
          byte_E1180[v6] = 0;
        }
      }
    }
    while ( v11 < a1 )
    {
      v7 = v11++;
      byte_E1180[v7] = 0;
    }
    return sub_CBA6(a1, (long long)byte_E1180, 8u, (long long)word_E11C0);
  }
  else
  {
    v12 = sub_CB39(a2);
    for ( j = 0; ; ++j )
    {
      result = (unsigned int)j;
      if ( j >= a1 )
        break;
      byte_E1180[j] = 0;
    }
    for ( k = 0; k <= 255; ++k )
    {
      result = (unsigned long long)word_E11C0;
      word_E11C0[k] = v12;
    }
  }
  return result;
}



// Function: read_c_len @ 0xd23e
unsigned long long read_c_len()
{
  unsigned long long result; // rax
  long long v1; // rdx
  int v2; // eax
  int v3; // eax
  int v4; // eax
  int i; // [rsp+0h] [rbp-10h]
  int j; // [rsp+0h] [rbp-10h]
  int v7; // [rsp+0h] [rbp-10h]
  short v8; // [rsp+4h] [rbp-Ch]
  int v9; // [rsp+4h] [rbp-Ch]
  int v10; // [rsp+4h] [rbp-Ch]
  unsigned int v11; // [rsp+8h] [rbp-8h]
  int v12; // [rsp+Ch] [rbp-4h]

  v12 = sub_CB39(9);
  if ( v12 )
  {
    v7 = 0;
    while ( v7 < v12 )
    {
      v9 = (unsigned short)word_E11C0[HIBYTE(word_E13C0)];
      if ( (unsigned int)v9 > 0x12 )
      {
        v11 = 128;
        do
        {
          if ( ((unsigned short)v11 & (unsigned short)word_E13C0) != 0 )
            v1 = v9 + 0x8000LL;
          else
            v1 = v9;
          v9 = word_BD000[v1];
          v11 >>= 1;
        }
        while ( (unsigned int)v9 > 0x12 );
      }
      sub_CA3C((unsigned char)byte_E1180[v9]);
      if ( v9 > 2 )
      {
        v3 = v7++;
        byte_5C000[v3] = v9 - 2;
      }
      else
      {
        if ( v9 )
        {
          if ( v9 == 1 )
            v10 = sub_CB39(4) + 3;
          else
            v10 = sub_CB39(9) + 20;
        }
        else
        {
          v10 = 1;
        }
        while ( --v10 >= 0 )
        {
          v2 = v7++;
          byte_5C000[v2] = 0;
        }
      }
    }
    while ( v7 <= 509 )
    {
      v4 = v7++;
      byte_5C000[v4] = 0;
    }
    return sub_CBA6(0x1FEu, (long long)byte_5C000, 0xCu, (long long)word_9C800);
  }
  else
  {
    result = sub_CB39(9);
    v8 = result;
    for ( i = 0; i <= 509; ++i )
    {
      result = i;
      byte_5C000[i] = 0;
    }
    for ( j = 0; j <= 4095; ++j )
    {
      result = (unsigned long long)word_9C800;
      word_9C800[j] = v8;
    }
  }
  return result;
}



// Function: decode_c @ 0xd445
long long decode_c()
{
  long long v1; // rdx
  unsigned int v2; // [rsp+8h] [rbp-8h]
  unsigned int v3; // [rsp+Ch] [rbp-4h]

  if ( !dword_E11A0 )
  {
    dword_E11A0 = sub_CB39(16);
    if ( !dword_E11A0 )
      return 510;
    sub_D0A0(19, 5, 3);
    sub_D23E();
    sub_D0A0(14, 4, -1);
  }
  --dword_E11A0;
  v2 = (unsigned short)word_9C800[(unsigned short)word_E13C0 >> 4];
  if ( v2 > 0x1FD )
  {
    v3 = 8;
    do
    {
      if ( ((unsigned short)v3 & (unsigned short)word_E13C0) != 0 )
        v1 = v2 + 0x8000LL;
      else
        v1 = v2;
      v2 = word_BD000[v1];
      v3 >>= 1;
    }
    while ( v2 > 0x1FD );
  }
  sub_CA3C((unsigned char)byte_5C000[v2]);
  return v2;
}



// Function: decode_p @ 0xd564
long long decode_p()
{
  long long v0; // rdx
  unsigned int v2; // [rsp+8h] [rbp-18h]
  unsigned int v3; // [rsp+Ch] [rbp-14h]

  v2 = (unsigned short)word_E11C0[HIBYTE(word_E13C0)];
  if ( v2 > 0xD )
  {
    v3 = 128;
    do
    {
      if ( ((unsigned short)v3 & (unsigned short)word_E13C0) != 0 )
        v0 = v2 + 0x8000LL;
      else
        v0 = v2;
      v2 = word_BD000[v0];
      v3 >>= 1;
    }
    while ( v2 > 0xD );
  }
  sub_CA3C((unsigned char)byte_E1180[v2]);
  if ( v2 )
    return (1 << (v2 - 1)) + (unsigned int)sub_CB39(v2 - 1);
  return v2;
}



// Function: huf_decode_start @ 0xd641
long long huf_decode_start()
{
  long long result; // rax

  result = sub_CB74();
  dword_E11A0 = 0;
  return result;
}



// Function: decode_start @ 0xd65b
long long decode_start()
{
  long long result; // rax

  result = sub_D641();
  dword_E13CC = 0;
  dword_E13D0 = 0;
  return result;
}



// Function: decode @ 0xd67f
long long decode(int a1, long long a2)
{
  unsigned int v3; // [rsp+18h] [rbp-8h]
  unsigned int v4; // [rsp+1Ch] [rbp-4h]

  v3 = 0;
  while ( --dword_E13CC >= 0 )
  {
    *(char *)(v3 + a2) = *(char *)((unsigned int)dword_E13D4 + a2);
    dword_E13D4 = ((short)dword_E13D4 + 1) & 0x1FFF;
    if ( ++v3 == a1 )
      return v3;
  }
  do
  {
    while ( 1 )
    {
      v4 = sub_D445();
      if ( v4 == 510 )
      {
        dword_E13D0 = 1;
        return v3;
      }
      if ( v4 <= 0xFF )
        break;
      dword_E13CC = v4 - 253;
      dword_E13D4 = ((short)v3 - (unsigned short)sub_D564() - 1) & 0x1FFF;
      while ( --dword_E13CC >= 0 )
      {
        *(char *)(v3 + a2) = *(char *)((unsigned int)dword_E13D4 + a2);
        dword_E13D4 = ((short)dword_E13D4 + 1) & 0x1FFF;
        if ( ++v3 == a1 )
          return v3;
      }
    }
    *(char *)(v3++ + a2) = v4;
  }
  while ( v3 != a1 );
  return v3;
}



// Function: unlzh @ 0xd7d6
long long unlzh(int a1, unsigned int a2)
{
  unsigned int v3; // [rsp+1Ch] [rbp-4h]

  dword_DD860 = a1;
  fd = a2;
  sub_D65B();
  while ( !dword_E13D0 )
  {
    v3 = sub_D67F(0x2000, (long long)byte_AD000);
    if ( v3 )
      sub_F321(a2, byte_AD000, v3);
  }
  return 0;
}



// Function: unlzw @ 0xd846
long long unlzw(unsigned int a1, unsigned int a2)
{
  int v2; // eax
  int v3; // eax
  long long v5; // rax
  unsigned int v6; // eax
  char *v7; // rsi
  long long v8; // rax
  int v9; // eax
  const char *v10; // rax
  unsigned char v12; // [rsp+1Ch] [rbp-74h]
  int v13; // [rsp+20h] [rbp-70h]
  unsigned int v14; // [rsp+24h] [rbp-6Ch]
  int v15; // [rsp+28h] [rbp-68h]
  int v16; // [rsp+2Ch] [rbp-64h]
  signed int j; // [rsp+30h] [rbp-60h]
  int v18; // [rsp+34h] [rbp-5Ch]
  unsigned int v19; // [rsp+38h] [rbp-58h]
  char *src; // [rsp+40h] [rbp-50h]
  unsigned char *srca; // [rsp+40h] [rbp-50h]
  long long i; // [rsp+48h] [rbp-48h]
  long long v23; // [rsp+48h] [rbp-48h]
  long long v24; // [rsp+50h] [rbp-40h]
  long long v25; // [rsp+58h] [rbp-38h]
  long long v26; // [rsp+60h] [rbp-30h]
  long long v27; // [rsp+68h] [rbp-28h]
  long long v28; // [rsp+70h] [rbp-20h]
  long long v29; // [rsp+78h] [rbp-18h]
  long long v30; // [rsp+88h] [rbp-8h]

  if ( dword_DD86C >= (unsigned int)dword_DD868 )
  {
    v3 = sub_F0BC(0);
  }
  else
  {
    v2 = dword_DD86C++;
    v3 = (unsigned char)byte_1B000[v2];
  }
  dword_19098 = v3;
  dword_19548 = v3 & 0x80;
  if ( (v3 & 0x60) != 0 )
  {
    if ( !dword_DD004 )
      fprintf(stderr, "\n%s: %s: warning, unknown flags 0x%x\n", s, dest, dword_19098 & 0x60);
    if ( !dword_DD018 )
      dword_DD018 = 2;
  }
  dword_19098 &= 0x1Fu;
  v28 = 1LL << dword_19098;
  if ( dword_19098 <= 16 )
  {
    v16 = dword_DD868;
    v15 = 9;
    v27 = 511;
    v14 = 511;
    v24 = -1;
    v12 = 0;
    v13 = 0;
    v25 = (unsigned int)(8 * dword_DD86C);
    if ( dword_19548 )
      v5 = 257;
    else
      v5 = 256;
    v26 = v5;
    memset(word_BD000, 0, 0x100u);
    for ( i = 255; i >= 0; --i )
      byte_AD000[i] = i;
    do
    {
LABEL_17:
      v19 = v25 >> 3;
      if ( v19 > dword_DD868 )
        v6 = 0;
      else
        v6 = dword_DD868 - v19;
      for ( j = 0; j < (int)v6; ++j )
        byte_1B000[j] = byte_1B000[j + v19];
      dword_DD868 = v6;
      v25 = 0;
      if ( v6 <= 0x3F )
      {
        v7 = &byte_1B000[dword_DD868];
        v16 = sub_F194(a1, v7, &unk_40000);
        if ( v16 == -1 )
          sub_F6C7(a1, v7);
        dword_DD868 += v16;
        qword_DD040 += v16;
      }
      if ( v16 )
        v8 = 8 * ((unsigned int)dword_DD868 - (unsigned long long)(dword_DD868 % (unsigned int)v15));
      else
        v8 = 8LL * (unsigned int)dword_DD868 - (v15 - 1);
      v29 = v8;
      while ( v29 > v25 )
      {
        if ( v26 > v27 )
        {
          v25 = v25 - 1 + 8 * v15 - (v25 - 1 + 8 * v15) % (8 * v15);
          if ( ++v15 == dword_19098 )
            v27 = v28;
          else
            v27 = (1LL << v15) - 1;
          v14 = (1 << v15) - 1;
          goto LABEL_17;
        }
        v23 = ((long long)(((unsigned long long)(unsigned char)byte_1B000[(v25 >> 3) + 2] << 16)
                       | *(unsigned short *)&byte_1B000[v25 >> 3]) >> (v25 & 7))
            & v14;
        v25 += v15;
        if ( v24 == -1 )
        {
          if ( v23 > 255 )
            sub_F5EF("corrupt input.");
          v24 = v23;
          v12 = v23;
          v9 = v13++;
          byte_5C000[v9] = v23;
        }
        else
        {
          if ( v23 == 256 && dword_19548 )
          {
            memset(word_BD000, 0, 0x100u);
            v26 = 256;
            v25 = v25 - 1 + 8 * v15 - (v25 - 1 + 8 * v15) % (8 * v15);
            v15 = 9;
            v27 = 511;
            v14 = 511;
            goto LABEL_17;
          }
          v30 = v23;
          src = byte_AC7FE;
          if ( v23 >= v26 )
          {
            if ( v23 > v26 )
            {
              if ( v13 > 0 )
                sub_F321(a2, byte_5C000, (unsigned int)v13);
              if ( dword_DD000 )
                v10 = "corrupt input.";
              else
                v10 = "corrupt input. Use zcat to recover some data.";
              sub_F5EF(v10);
            }
            src = &byte_AC7FE[-1];
            byte_AC7FE[-1] = v12;
            v23 = v24;
          }
          while ( (unsigned long long)v23 > 0xFF )
          {
            *--src = byte_AD000[v23];
            v23 = word_BD000[v23];
          }
          v12 = byte_AD000[v23];
          srca = (unsigned char *)(src - 1);
          *srca = v12;
          v18 = (unsigned int)byte_AC7FE - (int)srca;
          if ( v18 + v13 <= (int)&unk_3FFFF )
          {
            memcpy(&byte_5C000[v13], srca, v18);
            v13 += v18;
          }
          else
          {
            do
            {
              if ( v18 > (int)&unk_40000 - v13 )
                v18 = (int)&unk_40000 - v13;
              if ( v18 > 0 )
              {
                memcpy(&byte_5C000[v13], srca, v18);
                v13 += v18;
              }
              if ( v13 > (int)&unk_3FFFF )
              {
                sub_F321(a2, byte_5C000, (unsigned int)v13);
                v13 = 0;
              }
              srca += v18;
              v18 = byte_AC7FE - srca;
            }
            while ( v18 > 0 );
          }
          if ( v26 < v28 )
          {
            word_BD000[v26] = v24;
            byte_AD000[v26++] = v12;
          }
          v24 = v30;
        }
      }
    }
    while ( v16 );
    if ( v13 > 0 )
      sub_F321(a2, byte_5C000, (unsigned int)v13);
    return 0;
  }
  else
  {
    fprintf(stderr, "\n%s: %s: compressed with %d bits, can only handle %d bits\n", s, dest, dword_19098, 16);
    dword_DD018 = 1;
    return 1;
  }
}



// Function: read_byte @ 0xdff1
long long read_byte()
{
  int v0; // eax
  long long result; // rax

  if ( dword_DD86C >= (unsigned int)dword_DD868 )
  {
    LODWORD(result) = sub_F0BC(0);
  }
  else
  {
    v0 = dword_DD86C++;
    LODWORD(result) = (unsigned char)byte_1B000[v0];
  }
  if ( (int)result < 0 )
    sub_F5EF("invalid compressed data -- unexpected end of file");
  return (unsigned int)result;
}



// Function: read_tree @ 0xe055
int *read_tree()
{
  long long v0; // rbx
  unsigned char v1; // al
  int v2; // ebx
  int j; // [rsp+0h] [rbp-20h]
  int k; // [rsp+0h] [rbp-20h]
  int v6; // [rsp+4h] [rbp-1Ch]
  int i; // [rsp+8h] [rbp-18h]
  int v8; // [rsp+8h] [rbp-18h]
  int m; // [rsp+8h] [rbp-18h]
  int v10; // [rsp+Ch] [rbp-14h]

  v10 = 1;
  qword_E13E0 = 0;
  for ( i = 1; i <= 4; ++i )
  {
    v0 = qword_E13E0 << 8;
    qword_E13E0 = v0 | (unsigned char)sub_DFF1();
  }
  v1 = sub_DFF1();
  dword_E13E8 = v1;
  if ( !v1 || dword_E13E8 > 25 )
    sub_F5EF("invalid compressed data -- Huffman code bit length out of range");
  v8 = 0;
  for ( j = 1; j <= dword_E13E8; ++j )
  {
    dword_E1580[j] = (unsigned char)sub_DFF1();
    if ( v10 - (j == dword_E13E8) < dword_E1580[j] )
      sub_F5EF("too many leaves in Huffman tree");
    v10 = 2 * (v10 - dword_E1580[j] + 1) - 1;
    v8 += dword_E1580[j];
  }
  if ( v8 > 255 )
    sub_F5EF("too many leaves in Huffman tree");
  ++dword_E1580[dword_E13E8];
  v6 = 0;
  for ( k = 1; k <= dword_E13E8; ++k )
  {
    dword_E1500[k] = v6;
    for ( m = dword_E1580[k]; m > 0; --m )
    {
      v2 = v6++;
      byte_E1400[v2] = sub_DFF1();
    }
  }
  ++dword_E1580[dword_E13E8];
  return dword_E1580;
}



// Function: unpack @ 0xe3fd
long long unpack(int a1, int a2)
{
  long long v2; // rbx
  unsigned int v3; // ebx
  long long v4; // rbx
  unsigned int v5; // ecx
  int v6; // edx
  int i; // [rsp+1Ch] [rbp-24h]
  int v9; // [rsp+20h] [rbp-20h]
  int v10; // [rsp+24h] [rbp-1Ch]
  int v11; // [rsp+28h] [rbp-18h]

  dword_DD860 = a1;
  fd = a2;
  sub_E055();
  sub_E29D();
  dword_E1678 = 0;
  qword_E1670 = 0;
  v9 = (1 << dword_E1668) - 1;
  v10 = dword_E1580[dword_E13E8] - 1;
  while ( 1 )
  {
    while ( dword_E1678 < dword_E1668 )
    {
      v2 = qword_E1670 << 8;
      qword_E1670 = v2 | (unsigned char)sub_DFF1();
      dword_E1678 += 8;
    }
    v3 = v9 & ((unsigned long long)qword_E1670 >> ((unsigned char)dword_E1678 - (unsigned char)dword_E1668));
    i = (unsigned char)byte_5C000[v3];
    if ( byte_5C000[v3] )
    {
      v3 >>= dword_E1668 - i;
    }
    else
    {
      v11 = v9;
      for ( i = dword_E1668;
            v3 < dword_E1600[i];
            v3 = v11 & ((unsigned long long)qword_E1670 >> ((unsigned char)dword_E1678 - (unsigned char)i)) )
      {
        ++i;
        v11 = 2 * v11 + 1;
        while ( i > dword_E1678 )
        {
          v4 = qword_E1670 << 8;
          qword_E1670 = v4 | (unsigned char)sub_DFF1();
          dword_E1678 += 8;
        }
      }
    }
    if ( v3 == v10 && i == dword_E13E8 )
      break;
    v5 = v3 + dword_E1500[i];
    v6 = dword_DD870++;
    byte_AD000[v6] = byte_E1400[v5];
    if ( dword_DD870 == 0x8000 )
      sub_F2CC();
    dword_E1678 -= i;
  }
  sub_F2CC();
  if ( (unsigned int)qword_DD048 != qword_E13E0 )
    sub_F5EF("invalid compressed data--length error");
  return 0;
}



// Function: check_zipfile @ 0xe655
long long check_zipfile(int a1)
{
  char *v2; // [rsp+18h] [rbp-8h]

  v2 = &byte_1B000[dword_DD86C];
  dword_DD860 = a1;
  dword_DD86C += *((unsigned short *)v2 + 13) + 30 + *((unsigned short *)v2 + 14);
  if ( dword_DD86C <= (unsigned int)dword_DD868 && *(int *)v2 == 67324752 )
  {
    dword_1909C = (unsigned char)v2[8];
    if ( !dword_1909C || dword_1909C == 8 )
    {
      dword_E1688 = v2[6] & 1;
      if ( dword_E1688 )
      {
        fprintf(stderr, "\n%s: %s: encrypted file -- use unzip\n", s, dest);
        dword_DD018 = 1;
        return 1;
      }
      else
      {
        dword_E1690 = (v2[6] & 8) != 0;
        dword_E168C = 1;
        return 0;
      }
    }
    else
    {
      fprintf(stderr, "\n%s: %s: first entry not deflated or stored -- use unzip\n", s, dest);
      dword_DD018 = 1;
      return 1;
    }
  }
  else
  {
    fprintf(stderr, "\n%s: %s: not a valid zip file\n", s, dest);
    dword_DD018 = 1;
    return 1;
  }
}



// Function: unzip @ 0xe86a
long long unzip(int a1, int a2)
{
  long long v2; // rbx
  unsigned long long v3; // rdx
  long long v4; // rcx
  int v5; // eax
  char v6; // al
  int v7; // eax
  int v9; // eax
  char v10; // al
  int v11; // eax
  char v12; // al
  unsigned char v14; // [rsp+13h] [rbp-4Dh]
  int j; // [rsp+14h] [rbp-4Ch]
  int i; // [rsp+14h] [rbp-4Ch]
  unsigned int v17; // [rsp+18h] [rbp-48h]
  int v18; // [rsp+1Ch] [rbp-44h]
  long long v19; // [rsp+20h] [rbp-40h]
  long long v20; // [rsp+28h] [rbp-38h]
  unsigned int v21; // [rsp+30h] [rbp-30h]
  unsigned int v22; // [rsp+34h] [rbp-2Ch]
  unsigned int v23; // [rsp+3Ch] [rbp-24h]
  unsigned long long v24; // [rsp+48h] [rbp-18h]

  v24 = __readfsqword(0x28u);
  v19 = 0;
  v20 = 0;
  v17 = 0;
  dword_DD860 = a1;
  fd = a2;
  sub_EFDA(0, 0);
  if ( dword_E168C && !dword_E1690 )
  {
    v19 = (unsigned int)dword_1B00E;
    v20 = (unsigned int)dword_1B016;
  }
  if ( dword_1909C == 8 )
  {
    v18 = sub_AB98();
    if ( v18 == 3 )
      sub_F634();
    if ( v18 )
      sub_F5EF("invalid compressed data--format violated");
  }
  else
  {
    if ( !dword_E168C || dword_1909C )
      sub_F5EF("internal error, invalid method");
    v2 = (unsigned int)dword_1B016;
    v3 = (unsigned int)dword_1B012;
    if ( dword_E1688 )
      v4 = 12;
    else
      v4 = 0;
    if ( dword_1B016 != (unsigned int)dword_1B012 - v4 )
    {
      fprintf(stderr, "len %lu, siz %lu\n", (unsigned int)dword_1B016, (unsigned int)dword_1B012);
      sub_F5EF("invalid compressed data--length mismatch");
    }
    while ( v2-- )
    {
      if ( dword_DD86C >= (unsigned int)dword_DD868 )
      {
        v6 = sub_F0BC(0);
      }
      else
      {
        v5 = dword_DD86C++;
        v6 = byte_1B000[v5];
      }
      v14 = v6;
      v7 = dword_DD870++;
      v3 = (unsigned long long)byte_AD000;
      byte_AD000[v7] = v14;
      if ( dword_DD870 == 0x8000 )
        ((void (*)(void))sub_F2CC)();
    }
    sub_F2CC(0, 0, v3);
  }
  if ( dword_E168C )
  {
    if ( dword_E1690 )
    {
      for ( i = 0; i <= 15; ++i )
      {
        if ( dword_DD86C >= (unsigned int)dword_DD868 )
        {
          v12 = sub_F0BC(0);
        }
        else
        {
          v11 = dword_DD86C++;
          v12 = byte_1B000[v11];
        }
        *((char *)&v21 + i) = v12;
      }
      v19 = v22;
      v20 = v23;
    }
  }
  else
  {
    for ( j = 0; j <= 7; ++j )
    {
      if ( dword_DD86C >= (unsigned int)dword_DD868 )
      {
        v10 = sub_F0BC(0);
      }
      else
      {
        v9 = dword_DD86C++;
        v10 = byte_1B000[v9];
      }
      *((char *)&v21 + j) = v10;
    }
    v19 = v21;
    v20 = v22;
  }
  if ( v19 != sub_EFDA(byte_5C000, 0) )
  {
    fprintf(stderr, "\n%s: %s: invalid compressed data--crc error\n", s, dest);
    v17 = 1;
  }
  if ( v20 != (unsigned int)qword_DD048 )
  {
    fprintf(stderr, "\n%s: %s: invalid compressed data--length error\n", s, dest);
    v17 = 1;
  }
  if ( dword_E168C
    && dword_DD86C + 4 < (unsigned int)dword_DD868
    && (*(unsigned short *)&byte_1B000[dword_DD86C]
      | (unsigned long long)((long long)((unsigned char)byte_1B000[dword_DD86C + 2]
                                   | ((unsigned char)byte_1B000[dword_DD86C + 3] << 8)) << 16)) == 0x4034B50 )
  {
    if ( dword_DD000 )
    {
      if ( !dword_DD004 )
        fprintf(stderr, "%s: %s has more than one entry--rest ignored\n", s, dest);
      if ( !dword_DD018 )
        dword_DD018 = 2;
    }
    else
    {
      fprintf(stderr, "%s: %s has more than one entry -- unchanged\n", s, dest);
      v17 = 1;
    }
  }
  dword_E168C = 0;
  dword_E1690 = 0;
  qword_E1680 = v19;
  if ( !v17 )
    return 0;
  dword_DD018 = 1;
  if ( !dword_DD008 )
    sub_8F9F();
  return v17;
}



// Function: copy @ 0xef1d
long long copy(unsigned int a1, unsigned int a2)
{
  int v3; // [rsp+1Ch] [rbp-4h]

  *__errno_location() = 0;
  while ( dword_DD868 > (unsigned int)dword_DD86C )
  {
    sub_F321(a2, &byte_1B000[dword_DD86C], (unsigned int)(dword_DD868 - dword_DD86C));
    v3 = sub_F194(a1, byte_1B000, &unk_40000);
    if ( v3 == -1 )
      sub_F6C7(a1, byte_1B000);
    qword_DD040 += v3;
    dword_DD868 = v3;
    dword_DD86C = 0;
  }
  return 0;
}



// Function: updcrc @ 0xefda
long long updcrc(char *a1, int a2)
{
  unsigned long long v2; // rbx
  char *v3; // rax
  int v5; // [rsp+0h] [rbp-14h]
  char *v6; // [rsp+4h] [rbp-10h]

  v6 = a1;
  v5 = a2;
  if ( a1 )
  {
    v2 = qword_19550;
    if ( a2 )
    {
      do
      {
        v3 = v6++;
        v2 = (v2 >> 8) ^ qword_14680[(unsigned char)(v2 ^ *v3)];
        --v5;
      }
      while ( v5 );
    }
  }
  else
  {
    v2 = 0xFFFFFFFFLL;
  }
  qword_19550 = v2;
  return v2 ^ 0xFFFFFFFF;
}



// Function: getcrc @ 0xf05f
long long getcrc()
{
  return qword_19550 ^ 0xFFFFFFFFLL;
}



// Function: clear_bufs @ 0xf078
long long clear_bufs()
{
  dword_DD870 = 0;
  dword_DD86C = 0;
  dword_DD868 = 0;
  qword_DD048 = 0;
  qword_DD040 = 0;
  return 0;
}



// Function: fill_inbuf @ 0xf0bc
long long fill_inbuf(int a1)
{
  char *v1; // rsi
  long long v2; // rdi
  long long v3; // rdx
  int v6; // [rsp+1Ch] [rbp-4h]

  dword_DD868 = 0;
  do
  {
    v1 = &byte_1B000[dword_DD868];
    v2 = (unsigned int)dword_DD860;
    v6 = sub_F194((unsigned int)dword_DD860, v1, (unsigned int)((int)&unk_40000 - dword_DD868));
    if ( !v6 )
      break;
    if ( v6 == -1 )
      sub_F6C7(v2, v1);
    v3 = (unsigned int)dword_DD868;
    dword_DD868 += v6;
  }
  while ( dword_DD868 <= (unsigned int)&unk_3FFFF );
  if ( dword_DD868 )
  {
    qword_DD040 += (unsigned int)dword_DD868;
    dword_DD86C = 1;
    return (unsigned char)byte_1B000[0];
  }
  else
  {
    if ( !a1 )
    {
      sub_F2CC(v2, v1, v3);
      *__errno_location() = 0;
      sub_F6C7(v2, v1);
    }
    return 0xFFFFFFFFLL;
  }
}



// Function: read_buffer @ 0xf194
long long read_buffer(int a1, void *a2, int a3)
{
  int v3; // edx
  int v4; // ecx
  int v5; // r8d
  int v6; // r9d
  int v7; // ecx
  int v8; // r8d
  int v9; // r9d
  int v10; // eax
  char buf; // [rsp+0h] [rbp-20h]
  void *bufa; // [rsp+0h] [rbp-20h]
  unsigned int nbytes; // [rsp+8h] [rbp-18h]
  int v15; // [rsp+18h] [rbp-8h]
  int v16; // [rsp+1Ch] [rbp-4h]

  nbytes = a3;
  if ( a3 < 0 )
    nbytes = 0x7FFFFFFF;
  v15 = read(a1, a2, nbytes);
  if ( v15 < 0 && *__errno_location() == 11 )
  {
    v16 = sub_103BD(a1, 3, v3, v4, v5, v6, (char)a2);
    if ( v16 >= 0 )
    {
      if ( (v16 & 0x800) != 0 )
      {
        v10 = v16;
        BYTE1(v10) = BYTE1(v16) & 0xF7;
        if ( (unsigned int)sub_103BD(a1, 4, v10, v7, v8, v9, buf) != -1 )
          return (unsigned int)read(a1, bufa, nbytes);
      }
      else
      {
        *__errno_location() = 11;
      }
    }
  }
  return (unsigned int)v15;
}



// Function: write_buffer @ 0xf254
ssize_t write_buffer(int a1, const void *a2, int a3)
{
  unsigned int n; // [rsp+8h] [rbp-8h]

  n = a3;
  if ( a3 < 0 )
    n = 0x7FFFFFFF;
  return write(a1, a2, n);
}



// Function: flush_outbuf @ 0xf28e
long long flush_outbuf()
{
  long long result; // rax

  result = (unsigned int)dword_DD870;
  if ( dword_DD870 )
  {
    result = sub_F321((unsigned int)fd, byte_5C000, (unsigned int)dword_DD870);
    dword_DD870 = 0;
  }
  return result;
}



// Function: flush_window @ 0xf2cc
long long flush_window()
{
  long long result; // rax

  result = (unsigned int)dword_DD870;
  if ( dword_DD870 )
  {
    sub_EFDA(byte_AD000, dword_DD870);
    result = sub_F321((unsigned int)fd, byte_AD000, (unsigned int)dword_DD870);
    dword_DD870 = 0;
  }
  return result;
}



// Function: write_buf @ 0xf321
long long write_buf(int a1, char *a2, unsigned int a3)
{
  long long result; // rax
  int v5; // [rsp+8h] [rbp-18h]
  unsigned int v6; // [rsp+1Ch] [rbp-4h]

  v5 = a3;
  qword_DD048 += a3;
  result = (unsigned int)dword_DD008;
  if ( !dword_DD008 )
  {
    while ( 1 )
    {
      v6 = sub_F254(a1, a2, v5);
      result = v6;
      if ( v6 == v5 )
        break;
      if ( v6 == -1 )
        sub_F74D();
      v5 -= v6;
      a2 += v6;
    }
  }
  return result;
}



// Function: strlwr @ 0xf393
unsigned char *strlwr(unsigned char *a1)
{
  unsigned char v1; // al
  unsigned char *i; // [rsp+18h] [rbp-8h]

  for ( i = a1; *i; ++i )
  {
    if ( ((*__ctype_b_loc())[*i] & 0x100) != 0 )
      v1 = tolower(*i);
    else
      v1 = *i;
    *i = v1;
  }
  return a1;
}



// Function: gzip_base_name @ 0xf40a
long long gzip_base_name(long long a1)
{
  return sub_10244(a1);
}



// Function: xunlink @ 0xf430
long long xunlink(const char *a1)
{
  return (unsigned int)unlink(a1);
}



// Function: add_envopt @ 0xf454
char *add_envopt(int *a1, long long **a2, const char *a3)
{
  char *v4; // rax
  long long *v5; // rax
  char **v6; // rax
  char *v7; // rax
  int v8; // [rsp+2Ch] [rbp-24h]
  char *s; // [rsp+30h] [rbp-20h]
  char *sa; // [rsp+30h] [rbp-20h]
  char *sb; // [rsp+30h] [rbp-20h]
  char **v12; // [rsp+38h] [rbp-18h]
  char *ptr; // [rsp+40h] [rbp-10h]
  char *ptra; // [rsp+40h] [rbp-10h]
  long long *v15; // [rsp+48h] [rbp-8h]

  v8 = 0;
  ptr = getenv(a3);
  if ( !ptr )
    return 0;
  ptra = (char *)sub_12377(ptr);
  s = ptra;
  while ( *s )
  {
    sa = &s[strspn(s, " \t")];
    if ( !*sa )
      break;
    s = &sa[strcspn(sa, " \t")];
    if ( *s )
    {
      v4 = s++;
      *v4 = 0;
    }
    ++v8;
  }
  if ( v8 )
  {
    *a1 = v8 + 1;
    v5 = (long long *)sub_1224E(*a1 + 1, 8);
    v15 = *a2;
    *a2 = v5;
    v12 = (char **)(v5 + 1);
    *v5 = *v15;
    sb = ptra;
    while ( v8 > 0 )
    {
      sb += strspn(sb, " \t");
      v6 = v12++;
      *v6 = sb;
      do
        v7 = sb++;
      while ( *v7 );
      --v8;
    }
    *v12 = 0;
    return ptra;
  }
  else
  {
    free(ptra);
    return 0;
  }
}



// Function: gzip_error @ 0xf5ef
void gzip_error(const char *a1)
{
  fprintf(stderr, "\n%s: %s: %s\n", s, dest, a1);
  sub_8F9F();
}



// Function: xalloc_die @ 0xf634
void xalloc_die()
{
  fprintf(stderr, "\n%s: memory_exhausted\n", s);
  sub_8F9F();
}



// Function: warning @ 0xf666
long long warning(const char *a1)
{
  long long result; // rax

  if ( !dword_DD004 )
    fprintf(stderr, "%s: %s: warning: %s\n", s, dest, a1);
  result = (unsigned int)dword_DD018;
  if ( !dword_DD018 )
    dword_DD018 = 2;
  return result;
}



// Function: read_error @ 0xf6c7
void read_error()
{
  int v0; // [rsp+Ch] [rbp-4h]

  v0 = *__errno_location();
  fprintf(stderr, "\n%s: ", s);
  if ( v0 )
  {
    *__errno_location() = v0;
    perror(dest);
  }
  else
  {
    fprintf(stderr, "%s: unexpected end of file\n", dest);
  }
  sub_8F9F();
}



// Function: write_error @ 0xf74d
void write_error()
{
  int v0; // [rsp+Ch] [rbp-4h]

  v0 = *__errno_location();
  fprintf(stderr, "\n%s: ", s);
  *__errno_location() = v0;
  perror(byte_DD460);
  sub_8F9F();
}



// Function: display_ratio @ 0xf7a6
int display_ratio(int a1, long long a2, FILE *a3)
{
  double v3; // rax

  if ( a2 )
    v3 = 100.0 * (double)a1 / (double)(int)a2;
  else
    v3 = 0.0;
  return fprintf(a3, "%5.1f%%", v3);
}



// Function: fprint_off @ 0xf81a
unsigned long long fprint_off(FILE *a1, long long a2, int a3)
{
  int v3; // eax
  int v5; // [rsp+Ch] [rbp-74h]
  long long v6; // [rsp+10h] [rbp-70h]
  char *v7; // [rsp+28h] [rbp-58h]
  long long v8; // [rsp+70h] [rbp-10h] BYREF
  unsigned long long v9; // [rsp+78h] [rbp-8h]
  long long savedregs; // [rsp+80h] [rbp+0h] BYREF

  v6 = a2;
  v9 = __readfsqword(0x28u);
  v7 = (char *)&v8;
  if ( a2 >= 0 )
  {
    do
    {
      *--v7 = (char)v6 % 10 + 48;
      v6 /= 10;
    }
    while ( v6 );
  }
  else
  {
    do
    {
      *--v7 = 48 - (char)v6 % 10;
      v6 /= 10;
    }
    while ( v6 );
    *--v7 = 45;
  }
  v5 = a3 - ((unsigned int)&savedregs - 80 + 64 - (int)v7);
  while ( 1 )
  {
    v3 = v5--;
    if ( v3 <= 0 )
      break;
    putc(32, a1);
  }
  while ( v7 < (char *)&v8 )
    putc(*v7++, a1);
  return v9 - __readfsqword(0x28u);
}



// Function: zip @ 0xf9dd
long long zip(int a1, unsigned int a2)
{
  int v2; // eax
  int v3; // eax
  int v4; // eax
  int v5; // eax
  int v6; // eax
  int v7; // eax
  int v8; // eax
  int v9; // eax
  int v10; // eax
  int v11; // eax
  int v12; // eax
  int v13; // eax
  int v14; // eax
  char v15; // cl
  int v16; // eax
  char *v17; // rax
  char v18; // cl
  int v19; // eax
  char v20; // cl
  int v21; // eax
  char v22; // cl
  int v23; // eax
  char v24; // cl
  int v25; // eax
  char v26; // cl
  int v27; // eax
  char v28; // cl
  int v29; // eax
  unsigned long long v30; // rcx
  int v31; // eax
  char v32; // cl
  int v33; // eax
  int v34; // eax
  int v35; // eax
  int v36; // eax
  int v37; // eax
  int v38; // eax
  int v39; // eax
  int v40; // eax
  int v41; // eax
  char v43; // [rsp+13h] [rbp-1Dh]
  short v44; // [rsp+14h] [rbp-1Ch] BYREF
  short v45; // [rsp+16h] [rbp-1Ah]
  time_t v46; // [rsp+18h] [rbp-18h]
  char *v47; // [rsp+20h] [rbp-10h]
  unsigned long long v48; // [rsp+28h] [rbp-8h]

  v48 = __readfsqword(0x28u);
  v43 = 0;
  v44 = 0;
  v45 = 0;
  dword_DD860 = a1;
  fd = a2;
  dword_1909C = 8;
  dword_DD870 = 1;
  byte_5C000[0] = 31;
  if ( (int)&unk_40000 == 1 )
    sub_F28E();
  v2 = dword_DD870++;
  byte_5C000[v2] = -117;
  if ( dword_DD870 == (int)&unk_40000 )
    sub_F28E();
  v3 = dword_DD870++;
  byte_5C000[v3] = 8;
  if ( dword_DD870 == (int)&unk_40000 )
    sub_F28E();
  if ( dword_DD01C )
    v43 = 8;
  v4 = dword_DD870++;
  byte_5C000[v4] = v43;
  if ( dword_DD870 == (int)&unk_40000 )
    sub_F28E();
  if ( qword_DD038 >= 0 )
  {
    if ( timer <= 0 || timer > 0xFFFFFFFFLL )
    {
      sub_F666("file timestamp out of range for gzip format");
      v46 = 0;
    }
    else
    {
      v46 = timer;
    }
  }
  else
  {
    v46 = 0;
  }
  if ( dword_DD870 > (unsigned int)&unk_3FFFD )
  {
    v7 = dword_DD870++;
    byte_5C000[v7] = v46;
    if ( dword_DD870 == (int)&unk_40000 )
      sub_F28E();
    v8 = dword_DD870++;
    byte_5C000[v8] = BYTE1(v46);
    if ( dword_DD870 == (int)&unk_40000 )
      sub_F28E();
  }
  else
  {
    v5 = dword_DD870++;
    byte_5C000[v5] = v46;
    v6 = dword_DD870++;
    byte_5C000[v6] = BYTE1(v46);
  }
  if ( dword_DD870 > (unsigned int)&unk_3FFFD )
  {
    v11 = dword_DD870++;
    byte_5C000[v11] = BYTE2(v46);
    if ( dword_DD870 == (int)&unk_40000 )
      sub_F28E();
    v12 = dword_DD870++;
    byte_5C000[v12] = BYTE3(v46);
    if ( dword_DD870 == (int)&unk_40000 )
      sub_F28E();
  }
  else
  {
    v9 = dword_DD870++;
    byte_5C000[v9] = BYTE2(v46);
    v10 = dword_DD870++;
    byte_5C000[v10] = BYTE3(v46);
  }
  sub_EFDA(0, 0);
  sub_3A29(a2);
  sub_AC6F((long long)&v44, (long long)&dword_1909C);
  if ( dword_190A0 == 1 )
  {
    v45 |= 4u;
  }
  else if ( dword_190A0 == 9 )
  {
    v45 |= 2u;
  }
  v13 = dword_DD870++;
  byte_5C000[v13] = v45;
  if ( dword_DD870 == (int)&unk_40000 )
    sub_F28E();
  v14 = dword_DD870++;
  byte_5C000[v14] = 3;
  if ( dword_DD870 == (int)&unk_40000 )
    sub_F28E();
  if ( dword_DD01C )
  {
    v47 = (char *)sub_F40A((long long)dest);
    do
    {
      v15 = *v47;
      v16 = dword_DD870++;
      byte_5C000[v16] = v15;
      if ( dword_DD870 == (int)&unk_40000 )
        sub_F28E();
      v17 = v47++;
    }
    while ( *v17 );
  }
  qword_E1698 = (unsigned int)dword_DD870;
  sub_4A9E(dword_190A0);
  if ( qword_DD020 != -1 && qword_DD040 != qword_DD020 )
    fprintf(stderr, "%s: %s: file size changed while zipping\n", s, dest);
  if ( dword_DD870 > (unsigned int)&unk_3FFFD )
  {
    v22 = sub_F05F();
    v23 = dword_DD870++;
    byte_5C000[v23] = v22;
    if ( dword_DD870 == (int)&unk_40000 )
      sub_F28E();
    v24 = (unsigned short)sub_F05F() >> 8;
    v25 = dword_DD870++;
    byte_5C000[v25] = v24;
    if ( dword_DD870 == (int)&unk_40000 )
      sub_F28E();
  }
  else
  {
    v18 = sub_F05F();
    v19 = dword_DD870++;
    byte_5C000[v19] = v18;
    v20 = (unsigned short)sub_F05F() >> 8;
    v21 = dword_DD870++;
    byte_5C000[v21] = v20;
  }
  if ( dword_DD870 > (unsigned int)&unk_3FFFD )
  {
    v30 = (unsigned long long)sub_F05F() >> 16;
    v31 = dword_DD870++;
    byte_5C000[v31] = v30;
    if ( dword_DD870 == (int)&unk_40000 )
      sub_F28E();
    v32 = (unsigned int)sub_F05F() >> 24;
    v33 = dword_DD870++;
    byte_5C000[v33] = v32;
    if ( dword_DD870 == (int)&unk_40000 )
      sub_F28E();
  }
  else
  {
    v26 = (unsigned int)sub_F05F() >> 16;
    v27 = dword_DD870++;
    byte_5C000[v27] = v26;
    v28 = (unsigned int)sub_F05F() >> 24;
    v29 = dword_DD870++;
    byte_5C000[v29] = v28;
  }
  if ( dword_DD870 > (unsigned int)&unk_3FFFD )
  {
    v36 = dword_DD870++;
    byte_5C000[v36] = qword_DD040;
    if ( dword_DD870 == (int)&unk_40000 )
      sub_F28E();
    v37 = dword_DD870++;
    byte_5C000[v37] = BYTE1(qword_DD040);
    if ( dword_DD870 == (int)&unk_40000 )
      sub_F28E();
  }
  else
  {
    v34 = dword_DD870++;
    byte_5C000[v34] = qword_DD040;
    v35 = dword_DD870++;
    byte_5C000[v35] = BYTE1(qword_DD040);
  }
  if ( dword_DD870 > (unsigned int)&unk_3FFFD )
  {
    v40 = dword_DD870++;
    byte_5C000[v40] = BYTE2(qword_DD040);
    if ( dword_DD870 == (int)&unk_40000 )
      sub_F28E();
    v41 = dword_DD870++;
    byte_5C000[v41] = BYTE3(qword_DD040);
    if ( dword_DD870 == (int)&unk_40000 )
      sub_F28E();
  }
  else
  {
    v38 = dword_DD870++;
    byte_5C000[v38] = BYTE2(qword_DD040);
    v39 = dword_DD870++;
    byte_5C000[v39] = BYTE3(qword_DD040);
  }
  qword_E1698 += 8;
  sub_F28E();
  return 0;
}



// Function: file_read @ 0x101d7
long long file_read(char *a1, int a2)
{
  unsigned int v3; // [rsp+1Ch] [rbp-4h]

  v3 = sub_F194(dword_DD860, a1, a2);
  if ( !v3 )
    return 0;
  if ( v3 == -1 )
    sub_F6C7();
  sub_EFDA(a1, v3);
  qword_DD040 += v3;
  return v3;
}



// Function: last_component @ 0x10244
char *last_component(char *a1)
{
  char v2; // [rsp+17h] [rbp-11h]
  char *i; // [rsp+20h] [rbp-8h]

  v2 = 0;
  while ( *a1 == 47 )
    ++a1;
  for ( i = a1; *i; ++i )
  {
    if ( *i == 47 )
    {
      v2 = 1;
    }
    else if ( v2 )
    {
      a1 = i;
      v2 = 0;
    }
  }
  return a1;
}



// Function: base_len @ 0x102b1
size_t base_len(const char *a1)
{
  size_t i; // [rsp+10h] [rbp-10h]

  for ( i = strlen(a1); i > 1 && a1[i - 1] == 47; --i )
    ;
  return i;
}



// Function: rpl_fcntl @ 0x103bd
long long rpl_fcntl(
        unsigned int a1,
        int a2,
        long long a3,
        long long a4,
        long long a5,
        long long a6,
        __m128 a7,
        __m128 a8,
        __m128 a9,
        __m128 a10,
        __m128 a11,
        __m128 a12,
        __m128 a13,
        __m128 a14)
{
  long long v16; // [rsp+60h] [rbp-A0h]
  long long v17; // [rsp+68h] [rbp-98h]
  long long v18; // [rsp+70h] [rbp-90h]
  long long v19; // [rsp+78h] [rbp-88h]
  __m128 v20; // [rsp+80h] [rbp-80h]
  __m128 v21; // [rsp+90h] [rbp-70h]
  __m128 v22; // [rsp+A0h] [rbp-60h]
  __m128 v23; // [rsp+B0h] [rbp-50h]
  __m128 v24; // [rsp+C0h] [rbp-40h]
  __m128 v25; // [rsp+D0h] [rbp-30h]
  __m128 v26; // [rsp+E0h] [rbp-20h]
  __m128 v27; // [rsp+F0h] [rbp-10h]

  v16 = a3;
  v17 = a4;
  v18 = a5;
  v19 = a6;
  v20 = a7;
  v21 = a8;
  v22 = a9;
  v23 = a10;
  v24 = a11;
  v25 = a12;
  v26 = a13;
  v27 = a14;
  if ( a2 )
  {
    if ( a2 != 1030 )
    {
      if ( a2 > 11 )
      {
        if ( a2 > 1034 || a2 < 1024 )
          return (unsigned int)fcntl(a1, a2, v16);
        if ( ((1LL << a2) & 0x2C5) == 0 )
        {
          if ( ((1LL << a2) & 0x502) == 0 )
            return (unsigned int)fcntl(a1, a2, v16);
          return (unsigned int)fcntl(a1, a2);
        }
      }
      else
      {
        if ( a2 < 0 )
          return (unsigned int)fcntl(a1, a2, v16);
        if ( ((1LL << a2) & 0x515) == 0 )
        {
          if ( ((1LL << a2) & 0xA0A) == 0 )
            return (unsigned int)fcntl(a1, a2, v16);
          return (unsigned int)fcntl(a1, a2);
        }
      }
      return (unsigned int)fcntl(a1, a2, (unsigned int)v16);
    }
    return (unsigned int)sub_10760(a1, (unsigned int)v16);
  }
  else
  {
    return (unsigned int)sub_1072F(a1, (unsigned int)v16);
  }
}



// Function: rpl_fcntl_DUPFD @ 0x1072f
long long rpl_fcntl_DUPFD(int a1, unsigned int a2)
{
  return (unsigned int)fcntl(a1, 0, a2);
}



// Function: rpl_fcntl_DUPFD_CLOEXEC @ 0x10760
long long rpl_fcntl_DUPFD_CLOEXEC(int a1, unsigned int a2)
{
  int v3; // [rsp+14h] [rbp-Ch]
  int v4; // [rsp+18h] [rbp-8h]
  int v5; // [rsp+1Ch] [rbp-4h]

  if ( dword_E16A0 < 0 )
  {
    v3 = sub_1072F(a1, a2);
  }
  else
  {
    v3 = fcntl(a1, 1030, a2);
    if ( v3 < 0 && *__errno_location() == 22 )
    {
      v3 = sub_1072F(a1, a2);
      if ( v3 >= 0 )
        dword_E16A0 = -1;
    }
    else
    {
      dword_E16A0 = 1;
    }
  }
  if ( v3 >= 0 && dword_E16A0 == -1 )
  {
    v4 = fcntl(v3, 1);
    if ( v4 < 0 || fcntl(v3, 2, v4 | 1u) == -1 )
    {
      v5 = *__errno_location();
      close(v3);
      *__errno_location() = v5;
      return (unsigned int)-1;
    }
  }
  return (unsigned int)v3;
}



// Function: open_safer @ 0x10863
long long open_safer(const char *a1, char a2, unsigned int a3)
{
  unsigned int v3; // eax
  unsigned int v5; // [rsp+1Ch] [rbp-D4h]

  v5 = 0;
  if ( (a2 & 0x40) != 0 )
    v5 = a3;
  v3 = open(a1, a2, v5);
  return sub_110E3(v3);
}



// Function: clear_ungetc_buffer_preserving_position @ 0x1099f
long long clear_ungetc_buffer_preserving_position(int *a1)
{
  long long result; // rax

  result = *a1 & 0x100;
  if ( (int)result )
    return sub_10A26(a1, 0, 1);
  return result;
}



// Function: rpl_fseeko @ 0x10a26
int rpl_fseeko(FILE *a1, __off_t a2, int a3)
{
  int v3; // eax
  __off_t v6; // [rsp+28h] [rbp-8h]

  if ( a1->_IO_read_end != a1->_IO_read_ptr || a1->_IO_write_ptr != a1->_IO_write_base || a1->_IO_save_base )
    return fseeko(a1, a2, a3);
  v3 = fileno(a1);
  v6 = lseek(v3, a2, a3);
  if ( v6 == -1 )
    return -1;
  a1->_flags &= ~0x10u;
  a1->_offset = v6;
  return 0;
}



// Function: openat_safer @ 0x10ae4
long long openat_safer(int a1, const char *a2, char a3, unsigned int a4)
{
  unsigned int v4; // eax
  unsigned int v6; // [rsp+1Ch] [rbp-D4h]

  v6 = 0;
  if ( (a3 & 0x40) != 0 )
    v6 = a4;
  v4 = openat(a1, a2, a3, v6);
  return sub_110E3(v4);
}



// Function: direntry_cmp_name @ 0x10c22
int direntry_cmp_name(const char **a1, const char **a2)
{
  return strcmp(*a1, *a2);
}



// Function: streamsavedir @ 0x10c61
void *streamsavedir(DIR *a1, int a2)
{
  long long v3; // rdx
  long long *v4; // rbx
  char *v5; // rax
  long long v6; // [rsp+18h] [rbp-78h] BYREF
  long long v7; // [rsp+20h] [rbp-70h] BYREF
  void *v8; // [rsp+28h] [rbp-68h]
  void *ptr; // [rsp+30h] [rbp-60h]
  size_t nmemb; // [rsp+38h] [rbp-58h]
  long long v11; // [rsp+40h] [rbp-50h]
  signed long long i; // [rsp+48h] [rbp-48h]
  __compar_fn_t compar; // [rsp+50h] [rbp-40h]
  struct dirent *v14; // [rsp+58h] [rbp-38h]
  void *src; // [rsp+60h] [rbp-30h]
  size_t n; // [rsp+68h] [rbp-28h]
  char *dest; // [rsp+70h] [rbp-20h]
  unsigned long long v18; // [rsp+78h] [rbp-18h]

  v18 = __readfsqword(0x28u);
  v8 = 0;
  v6 = 0;
  ptr = 0;
  v7 = 0;
  nmemb = 0;
  v11 = 0;
  compar = (__compar_fn_t)qword_18B70[a2];
  if ( !a1 )
    return 0;
  while ( 1 )
  {
    *__errno_location() = 0;
    v14 = readdir(a1);
    if ( !v14 )
      break;
    src = v14->d_name;
    if ( v14->d_name[0] == 46 )
    {
      if ( *((char *)src + 1) == 46 )
        v3 = 2;
      else
        v3 = 1;
    }
    else
    {
      v3 = 0;
    }
    if ( *((char *)src + v3) )
    {
      n = strlen(v14->d_name) + 1;
      if ( compar )
      {
        if ( nmemb == v7 )
          ptr = (void *)sub_1204C(ptr, &v7, 1, -1, 8);
        v4 = (char *)ptr + 8 * nmemb;
        *v4 = sub_12377(src);
        ++nmemb;
      }
      else
      {
        if ( (long long)n >= v6 - v11 )
          v8 = (void *)sub_1204C(v8, &v6, n - (v6 - v11), 0x7FFFFFFFFFFFFFFELL, 1);
        memcpy((char *)v8 + v11, src, n);
      }
      v11 += n;
    }
  }
  if ( *__errno_location() )
  {
    free(ptr);
    free(v8);
    return 0;
  }
  else
  {
    if ( compar )
    {
      if ( nmemb )
        qsort(ptr, nmemb, 8u, compar);
      v8 = (void *)sub_11D96(v11 + 1);
      v11 = 0;
      for ( i = 0; i < (long long)nmemb; ++i )
      {
        dest = (char *)v8 + v11;
        v5 = stpcpy((char *)v8 + v11, *((const char **)ptr + i));
        v11 += v5 - dest + 1;
        free(*((void **)ptr + i));
      }
      free(ptr);
    }
    else if ( v11 == v6 )
    {
      v8 = (void *)sub_11E25(v8, v11 + 1);
    }
    *((char *)v8 + v11) = 0;
    return v8;
  }
}



// Function: savedir @ 0x10f97
void *savedir(long long a1, int a2)
{
  DIR *dirp; // [rsp+10h] [rbp-10h]
  void *ptr; // [rsp+18h] [rbp-8h]

  dirp = (DIR *)sub_123F7(a1);
  if ( !dirp )
    return 0;
  ptr = sub_10C61(dirp, a2);
  if ( !closedir(dirp) )
    return ptr;
  free(ptr);
  return 0;
}



// Function: fd_safer @ 0x110e3
long long fd_safer(unsigned int a1)
{
  unsigned int fd; // [rsp+Ch] [rbp-14h]
  unsigned int v3; // [rsp+18h] [rbp-8h]
  int v4; // [rsp+1Ch] [rbp-4h]

  fd = a1;
  if ( a1 <= 2 )
  {
    v3 = sub_12614(a1);
    v4 = *__errno_location();
    close(a1);
    *__errno_location() = v4;
    return v3;
  }
  return fd;
}



// Function: validate_timespec @ 0x11134
long long validate_timespec(long long *a1)
{
  int v2; // [rsp+18h] [rbp-8h]
  int v3; // [rsp+1Ch] [rbp-4h]

  v2 = 0;
  v3 = 0;
  if ( (a1[1] == 0x3FFFFFFF || a1[1] == 1073741822 || a1[1] < 0x3B9ACA00u)
    && (a1[3] == 0x3FFFFFFF || a1[3] == 1073741822 || a1[3] < 0x3B9ACA00u) )
  {
    if ( a1[1] == 0x3FFFFFFF || a1[1] == 1073741822 )
    {
      *a1 = 0;
      v2 = 1;
      if ( a1[1] == 1073741822 )
        v3 = 1;
    }
    if ( a1[3] == 0x3FFFFFFF || a1[3] == 1073741822 )
    {
      a1[2] = 0;
      v2 = 1;
      if ( a1[3] == 1073741822 )
        ++v3;
    }
    return (unsigned int)(v3 == 1) + v2;
  }
  else
  {
    *__errno_location() = 22;
    return 0xFFFFFFFFLL;
  }
}



// Function: update_timespec @ 0x1129e
long long update_timespec(long long a1, long long a2)
{
  long long v3; // rdx
  long long v4; // rdx
  long long *v5; // [rsp+18h] [rbp-18h]

  v5 = *(long long **)a2;
  if ( *(long long *)(*(long long *)a2 + 8LL) == 1073741822 && v5[3] == 1073741822 )
    return 1;
  if ( v5[1] == 0x3FFFFFFF && v5[3] == 0x3FFFFFFF )
  {
    *(long long *)a2 = 0;
    return 0;
  }
  else
  {
    if ( v5[1] == 1073741822 )
    {
      *v5 = sub_1105B(a1);
      v5[1] = v3;
    }
    else if ( v5[1] == 0x3FFFFFFF )
    {
      sub_124C4(v5);
    }
    if ( v5[3] == 1073741822 )
    {
      v5[2] = sub_1108F(a1);
      v5[3] = v4;
    }
    else if ( v5[3] == 0x3FFFFFFF )
    {
      sub_124C4(v5 + 2);
    }
    return 0;
  }
}



// Function: fdutimens @ 0x113ca
int fdutimens(int a1, const char *a2, long long *a3)
{
  struct timespec *v3; // rax
  long long v4; // rdx
  long long v5; // rdx
  bool v7; // al
  struct timespec *v8; // rbx
  __syscall_slong_t v9; // rdx
  struct timespec *v10; // rbx
  __syscall_slong_t v11; // rdx
  __suseconds_t tv_usec; // rdx
  __suseconds_t v14; // rdx
  short v16; // [rsp+2Eh] [rbp-142h]
  int v17; // [rsp+30h] [rbp-140h]
  int v18; // [rsp+34h] [rbp-13Ch]
  int v19; // [rsp+34h] [rbp-13Ch]
  struct timespec *times; // [rsp+38h] [rbp-138h] BYREF
  struct timeval *tvp; // [rsp+40h] [rbp-130h]
  struct timeval *p_tv_sec; // [rsp+48h] [rbp-128h]
  __time_t v23; // [rsp+50h] [rbp-120h]
  __time_t v24; // [rsp+58h] [rbp-118h]
  long long v25[4]; // [rsp+60h] [rbp-110h] BYREF
  long long v26[4]; // [rsp+80h] [rbp-F0h] BYREF
  __time_t tv_sec; // [rsp+A0h] [rbp-D0h] BYREF
  __suseconds_t v28; // [rsp+A8h] [rbp-C8h]
  __time_t v29; // [rsp+B0h] [rbp-C0h]
  __suseconds_t v30; // [rsp+B8h] [rbp-B8h]
  struct stat buf; // [rsp+C0h] [rbp-B0h] BYREF
  unsigned long long v32; // [rsp+158h] [rbp-18h]

  v32 = __readfsqword(0x28u);
  if ( a3 )
    v3 = (struct timespec *)v25;
  else
    v3 = 0;
  times = v3;
  v17 = 0;
  if ( v3 )
  {
    v4 = a3[1];
    v25[0] = *a3;
    v25[1] = v4;
    v5 = a3[3];
    v25[2] = a3[2];
    v25[3] = v5;
    v17 = sub_11134(times);
  }
  if ( v17 < 0 )
    return -1;
  if ( a1 < 0 && !a2 )
  {
    *__errno_location() = 9;
    return -1;
  }
  if ( dword_E16A4 >= 0 )
  {
    if ( v17 == 2 )
    {
      if ( a1 >= 0 )
        v7 = fstat(a1, &buf) != 0;
      else
        v7 = stat(a2, &buf) != 0;
      if ( v7 )
        return -1;
      if ( times->tv_nsec == 1073741822 )
      {
        v8 = times;
        times->tv_sec = sub_1105B(&buf);
        v8->tv_nsec = v9;
      }
      else if ( times[1].tv_nsec == 1073741822 )
      {
        v10 = times + 1;
        times[1].tv_sec = sub_1108F(&buf);
        v10->tv_nsec = v11;
      }
      v17 = 3;
    }
    if ( a1 < 0 )
    {
      v18 = utimensat(-100, a2, times, 0);
      if ( v18 > 0 )
        *__errno_location() = 38;
      if ( !v18 || *__errno_location() != 38 )
      {
        dword_E16A4 = 1;
        return v18;
      }
    }
    if ( a1 >= 0 )
    {
      v19 = futimens(a1, times);
      if ( v19 > 0 )
        *__errno_location() = 38;
      if ( !v19 || *__errno_location() != 38 )
      {
        dword_E16A4 = 1;
        return v19;
      }
    }
  }
  dword_E16A4 = -1;
  dword_E16A8 = -1;
  if ( !v17 )
    goto LABEL_45;
  if ( v17 != 3 && (a1 >= 0 ? fstat(a1, &buf) != 0 : stat(a2, &buf) != 0) )
    return -1;
  if ( times && (unsigned char)sub_1129E((long long)&buf, (long long)&times) )
    return 0;
LABEL_45:
  if ( times )
  {
    v26[0] = times->tv_sec;
    v26[1] = times->tv_nsec / 1000;
    v26[2] = times[1].tv_sec;
    v26[3] = times[1].tv_nsec / 1000;
    tvp = (struct timeval *)v26;
  }
  else
  {
    tvp = 0;
  }
  if ( a1 < 0 )
    return futimesat(-100, a2, tvp);
  if ( futimesat(a1, 0, tvp) )
  {
    if ( a2 )
      return utimes(a2, tvp);
    else
      return -1;
  }
  else
  {
    if ( tvp )
    {
      LOBYTE(v16) = tvp->tv_usec > (long long)&unk_7A11F;
      HIBYTE(v16) = tvp[1].tv_usec > (long long)&unk_7A11F;
      if ( v16 )
      {
        if ( !fstat(a1, &buf) )
        {
          v23 = buf.st_atim.tv_sec - tvp->tv_sec;
          v24 = buf.st_mtim.tv_sec - tvp[1].tv_sec;
          p_tv_sec = 0;
          tv_usec = tvp->tv_usec;
          tv_sec = tvp->tv_sec;
          v28 = tv_usec;
          v14 = tvp[1].tv_usec;
          v29 = tvp[1].tv_sec;
          v30 = v14;
          if ( (char)v16 && v23 == 1 && !sub_11006(&buf) )
          {
            p_tv_sec = (struct timeval *)&tv_sec;
            v28 = 0;
          }
          if ( HIBYTE(v16) && v24 == 1 && !sub_11032(&buf) )
          {
            p_tv_sec = (struct timeval *)&tv_sec;
            v30 = 0;
          }
          if ( p_tv_sec )
            futimesat(a1, 0, p_tv_sec);
        }
      }
    }
    return 0;
  }
}



// Function: utimens @ 0x11a12
int utimens(const char *a1, long long *a2)
{
  return sub_113CA(-1, a1, a2);
}



// Function: lutimens @ 0x11a3d
int lutimens(const char *a1, long long *a2)
{
  struct timespec *v2; // rax
  long long v3; // rdx
  long long v4; // rdx
  struct timespec *v6; // rbx
  __syscall_slong_t v7; // rdx
  struct timespec *v8; // rbx
  __syscall_slong_t v9; // rdx
  int v10; // [rsp+10h] [rbp-E0h]
  int v11; // [rsp+14h] [rbp-DCh]
  struct timespec *times; // [rsp+18h] [rbp-D8h] BYREF
  long long v13[4]; // [rsp+20h] [rbp-D0h] BYREF
  struct stat buf; // [rsp+40h] [rbp-B0h] BYREF
  unsigned long long v15; // [rsp+D8h] [rbp-18h]

  v15 = __readfsqword(0x28u);
  if ( a2 )
    v2 = (struct timespec *)v13;
  else
    v2 = 0;
  times = v2;
  v10 = 0;
  if ( v2 )
  {
    v3 = a2[1];
    v13[0] = *a2;
    v13[1] = v3;
    v4 = a2[3];
    v13[2] = a2[2];
    v13[3] = v4;
    v10 = sub_11134(times);
  }
  if ( v10 < 0 )
    return -1;
  if ( dword_E16A8 >= 0 )
  {
    if ( v10 == 2 )
    {
      if ( lstat(a1, &buf) )
        return -1;
      if ( times->tv_nsec == 1073741822 )
      {
        v6 = times;
        times->tv_sec = sub_1105B(&buf);
        v6->tv_nsec = v7;
      }
      else if ( times[1].tv_nsec == 1073741822 )
      {
        v8 = times + 1;
        times[1].tv_sec = sub_1108F(&buf);
        v8->tv_nsec = v9;
      }
      v10 = 3;
    }
    v11 = utimensat(-100, a1, times, 256);
    if ( v11 > 0 )
      *__errno_location() = 38;
    if ( !v11 || *__errno_location() != 38 )
    {
      dword_E16A4 = 1;
      dword_E16A8 = 1;
      return v11;
    }
  }
  dword_E16A8 = -1;
  if ( !v10 )
    goto LABEL_37;
  if ( v10 != 3 && lstat(a1, &buf) )
    return -1;
  if ( times && (unsigned char)sub_1129E((long long)&buf, (long long)&times) )
    return 0;
LABEL_37:
  if ( !v10 && lstat(a1, &buf) )
    return -1;
  if ( (buf.st_mode & 0xF000) != 0xA000 )
    return sub_113CA(-1, a1, times);
  *__errno_location() = 38;
  return -1;
}



// Function: nonnull @ 0x11d4e
long long nonnull(long long a1)
{
  if ( !a1 )
    sub_F634();
  return a1;
}



// Function: xmalloc @ 0x11d70
long long xmalloc(size_t a1)
{
  void *v1; // rax

  v1 = malloc(a1);
  return sub_11D4E((long long)v1);
}



// Function: ximalloc @ 0x11d96
long long ximalloc(long long a1)
{
  long long v1; // rax

  v1 = sub_12547(a1);
  return sub_11D4E(v1);
}



// Function: xcharalloc @ 0x11dbc
long long xcharalloc(size_t a1)
{
  return sub_11D70(a1);
}



// Function: xrealloc @ 0x11dda
void *xrealloc(void *a1, size_t a2)
{
  void *v3; // [rsp+18h] [rbp-8h]

  v3 = realloc(a1, a2);
  if ( !v3 && (!a1 || a2) )
    sub_F634();
  return v3;
}



// Function: xirealloc @ 0x11e25
long long xirealloc(long long a1, long long a2)
{
  long long v2; // rax

  v2 = sub_12565(a1, a2);
  return sub_11D4E(v2);
}



// Function: xreallocarray @ 0x11e56
long long xreallocarray(long long a1, long long a2, long long a3)
{
  long long v5; // [rsp+28h] [rbp-8h]

  v5 = reallocarray(a1, a2, a3);
  if ( !v5 && (!a1 || a2 && a3) )
    sub_F634();
  return v5;
}



// Function: xireallocarray @ 0x11eb0
long long xireallocarray(long long a1, long long a2, long long a3)
{
  long long v3; // rax

  v3 = sub_125C5(a1, a2, a3);
  return sub_11D4E(v3);
}



// Function: xnmalloc @ 0x11ee9
long long xnmalloc(long long a1, long long a2)
{
  return sub_11E56(0, a1, a2);
}



// Function: xinmalloc @ 0x11f14
long long xinmalloc(long long a1, long long a2)
{
  return sub_11EB0(0, a1, a2);
}



// Function: x2realloc @ 0x11f3f
long long x2realloc(long long a1, long long a2)
{
  return sub_11F6D(a1, a2, 1);
}



// Function: x2nrealloc @ 0x11f6d
long long x2nrealloc(long long a1, unsigned long long *a2, unsigned long long a3)
{
  long long v3; // rax
  bool v4; // cl
  long long result; // rax
  unsigned long long v6; // [rsp+20h] [rbp-10h]

  v6 = *a2;
  if ( a1 )
  {
    v3 = (v6 >> 1) + 1;
    v4 = __CFADD__(v6, v3);
    v6 += v3;
    if ( v4 )
      sub_F634();
  }
  else if ( !v6 )
  {
    v6 = (0x80 / a3 == 0) + 0x80 / a3;
  }
  result = sub_11E56(a1, v6, a3);
  *a2 = v6;
  return result;
}



// Function: xpalloc @ 0x1204c
void *xpalloc(void *a1, long long *a2, signed long long a3, long long a4, signed long long a5)
{
  long long v5; // rax
  void *result; // rax
  signed long long v7; // [rsp+38h] [rbp-28h]
  signed long long v8; // [rsp+40h] [rbp-20h]
  long long v9; // [rsp+48h] [rbp-18h]

  v9 = *a2;
  v7 = (v9 >> 1) + v9;
  if ( __OFADD__(v9 >> 1, v9) )
    v7 = 0x7FFFFFFFFFFFFFFFLL;
  if ( a4 >= 0 && a4 < v7 )
    v7 = a4;
  v8 = a5 * v7;
  if ( is_mul_ok(a5, v7) )
  {
    if ( v8 > 127 )
      v5 = 0;
    else
      v5 = 128;
  }
  else
  {
    v5 = 0x7FFFFFFFFFFFFFFFLL;
  }
  if ( v5 )
  {
    v7 = v5 / a5;
    v8 = v5 - v5 % a5;
  }
  if ( !a1 )
    *a2 = 0;
  if ( a3 > v7 - v9 )
  {
    if ( (v7 = v9 + a3, __OFADD__(v9, a3)) || a4 >= 0 && a4 < v7 || (v8 = a5 * v7, !is_mul_ok(a5, v7)) )
      sub_F634();
  }
  result = sub_11DDA(a1, v8);
  *a2 = v7;
  return result;
}



// Function: xzalloc @ 0x12208
long long xzalloc(long long a1)
{
  return sub_1224E(a1, 1);
}



// Function: xizalloc @ 0x1222b
long long xizalloc(long long a1)
{
  return sub_1227F(a1, 1);
}



// Function: xcalloc @ 0x1224e
long long xcalloc(size_t a1, size_t a2)
{
  void *v2; // rax

  v2 = calloc(a1, a2);
  return sub_11D4E((long long)v2);
}



// Function: xicalloc @ 0x1227f
long long xicalloc(long long a1, long long a2)
{
  long long v2; // rax

  v2 = sub_1259C(a1, a2);
  return sub_11D4E(v2);
}



// Function: xmemdup @ 0x122b0
void *xmemdup(const void *a1, size_t a2)
{
  void *v2; // rax

  v2 = (void *)sub_11D70(a2);
  return memcpy(v2, a1, a2);
}



// Function: ximemdup @ 0x122e8
void *ximemdup(const void *a1, size_t a2)
{
  void *v2; // rax

  v2 = (void *)sub_11D96(a2);
  return memcpy(v2, a1, a2);
}



// Function: ximemdup0 @ 0x12328
void *ximemdup0(const void *a1, size_t a2)
{
  char *dest; // [rsp+18h] [rbp-8h]

  dest = (char *)sub_11D96(a2 + 1);
  dest[a2] = 0;
  return memcpy(dest, a1, a2);
}



// Function: xstrdup @ 0x12377
void *xstrdup(const char *a1)
{
  size_t v1; // rax

  v1 = strlen(a1);
  return sub_122B0(a1, v1 + 1);
}



// Function: yesno @ 0x123a8
long long yesno()
{
  bool v0; // al
  bool v2; // [rsp+Bh] [rbp-5h]
  int v3; // [rsp+Ch] [rbp-4h]

  v3 = getchar();
  v0 = v3 == 121 || v3 == 89;
  v2 = v0;
  while ( v3 != 10 && v3 != -1 )
    v3 = getchar();
  return v2;
}



// Function: opendir_safer @ 0x123f7
DIR *opendir_safer(
        const char *a1,
        __m128 a2,
        __m128 a3,
        __m128 a4,
        __m128 a5,
        double a6,
        double a7,
        __m128 a8,
        __m128 a9)
{
  long long v9; // rcx
  long long v10; // r8
  long long v11; // r9
  __m128 v12; // xmm4
  __m128 v13; // xmm5
  int v15; // [rsp+14h] [rbp-1Ch]
  unsigned int v16; // [rsp+18h] [rbp-18h]
  int fd; // [rsp+1Ch] [rbp-14h]
  DIR *dirp; // [rsp+20h] [rbp-10h]
  DIR *v19; // [rsp+28h] [rbp-8h]

  dirp = opendir(a1);
  if ( dirp )
  {
    v16 = dirfd(dirp);
    if ( v16 <= 2 )
    {
      fd = sub_103BD(v16, 1030, 3, v9, v10, v11, a2, a3, a4, a5, v12, v13, a8, a9);
      if ( fd >= 0 )
      {
        v19 = fdopendir(fd);
        v15 = *__errno_location();
        if ( !v19 )
          close(fd);
      }
      else
      {
        v15 = *__errno_location();
        v19 = 0;
      }
      closedir(dirp);
      *__errno_location() = v15;
      return v19;
    }
  }
  return dirp;
}



// Function: gettime @ 0x124c4
int gettime(struct timespec *a1)
{
  return clock_gettime(0, a1);
}



// Function: current_timespec @ 0x124e8
__time_t current_timespec()
{
  struct timespec v1; // [rsp+0h] [rbp-20h] BYREF
  unsigned long long v2; // [rsp+18h] [rbp-8h]

  v2 = __readfsqword(0x28u);
  sub_124C4(&v1);
  return v1.tv_sec;
}



// Function: dup_safer @ 0x12614
long long dup_safer(
        unsigned int a1,
        __m128 a2,
        __m128 a3,
        __m128 a4,
        __m128 a5,
        __m128 a6,
        __m128 a7,
        __m128 a8,
        __m128 a9,
        long long a10,
        long long a11,
        long long a12,
        long long a13,
        long long a14)
{
  return sub_103BD(a1, 0, 3, a12, a13, a14, a2, a3, a4, a5, a6, a7, a8, a9);
}


