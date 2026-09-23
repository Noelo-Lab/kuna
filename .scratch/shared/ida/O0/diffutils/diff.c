// Function: discard_confusing_lines @ 0x6122
unsigned long long discard_confusing_lines(long long *a1)
{
  long long *v1; // rax
  char *v2; // rax
  char *v3; // r13
  long long kk; // rbx
  long long v5; // rbx
  long long v6; // r12
  long long v7; // rbx
  long long v8; // r12
  long long v9; // rbx
  long long v10; // r12
  long long v11; // rax
  int i; // [rsp+14h] [rbp-DCh]
  int m; // [rsp+14h] [rbp-DCh]
  int ii; // [rsp+14h] [rbp-DCh]
  int mm; // [rsp+14h] [rbp-DCh]
  long long j; // [rsp+18h] [rbp-D8h]
  long long k; // [rsp+18h] [rbp-D8h]
  unsigned long long n; // [rsp+18h] [rbp-D8h]
  long long jj; // [rsp+18h] [rbp-D8h]
  long long nn; // [rsp+18h] [rbp-D8h]
  long long v22; // [rsp+20h] [rbp-D0h]
  long long v23; // [rsp+20h] [rbp-D0h]
  unsigned long long v24; // [rsp+28h] [rbp-C8h]
  unsigned long long v25; // [rsp+30h] [rbp-C0h]
  long long v26; // [rsp+38h] [rbp-B8h]
  long long v27; // [rsp+40h] [rbp-B0h]
  long long v28; // [rsp+40h] [rbp-B0h]
  long long v29; // [rsp+48h] [rbp-A8h]
  long long v30; // [rsp+50h] [rbp-A0h]
  char *v31; // [rsp+58h] [rbp-98h]
  long long v32; // [rsp+60h] [rbp-90h]
  long long v33; // [rsp+68h] [rbp-88h]
  long long v34; // [rsp+70h] [rbp-80h]
  unsigned long long v35; // [rsp+78h] [rbp-78h]
  char *v36; // [rsp+80h] [rbp-70h]
  long long v37; // [rsp+88h] [rbp-68h]
  long long v38; // [rsp+90h] [rbp-60h]
  unsigned long long v39; // [rsp+98h] [rbp-58h]
  void *ptr[2]; // [rsp+A0h] [rbp-50h]
  void *v41; // [rsp+B0h] [rbp-40h]
  char *v42; // [rsp+B8h] [rbp-38h]
  unsigned long long v43; // [rsp+C8h] [rbp-28h]

  v43 = __readfsqword(0x28u);
  v22 = sub_1C945(16LL * (a1[25] + a1[63]));
  for ( i = 0; i <= 1; ++i )
  {
    a1[38 * i + 32] = v22;
    v23 = 8LL * a1[38 * i + 25] + v22;
    a1[38 * i + 33] = v23;
    v22 = 8LL * a1[38 * i + 25] + v23;
  }
  v41 = (void *)sub_13030(16LL * a1[37]);
  v42 = (char *)v41 + 8 * a1[37];
  for ( j = 0; j < a1[25]; ++j )
  {
    v1 = (char *)v41 + 8 * *(long long *)(a1[31] + 8 * j);
    ++*v1;
  }
  for ( k = 0; k < a1[63]; ++k )
  {
    v2 = &v42[8 * *(long long *)(a1[69] + 8 * k)];
    ++*(long long *)v2;
  }
  ptr[0] = (void *)sub_13030(a1[25] + a1[63]);
  ptr[1] = (char *)ptr[0] + a1[25];
  for ( m = 0; m <= 1; ++m )
  {
    v35 = a1[38 * m + 25];
    v36 = ptr[m];
    v37 = (long long)*(&v41 + 1 - m);
    v38 = a1[38 * m + 31];
    v24 = 5;
    v25 = v35 >> 6;
    while ( 1 )
    {
      v25 >>= 2;
      if ( !v25 )
        break;
      v24 *= 2LL;
    }
    for ( n = 0; v35 > n; ++n )
    {
      if ( *(long long *)(8 * n + v38) )
      {
        v39 = *(long long *)(8LL * *(long long *)(8 * n + v38) + v37);
        if ( v39 )
        {
          if ( v24 < v39 )
            v36[n] = 2;
        }
        else
        {
          v36[n] = 1;
        }
      }
    }
  }
  for ( ii = 0; ii <= 1; ++ii )
  {
    v33 = a1[38 * ii + 25];
    v3 = ptr[ii];
    for ( jj = 0; jj < v33; ++jj )
    {
      if ( v3[jj] == 2 )
      {
        v3[jj] = 0;
      }
      else if ( v3[jj] )
      {
        v26 = 0;
        for ( kk = jj; kk < v33 && v3[kk]; ++kk )
        {
          if ( v3[kk] == 2 )
            ++v26;
        }
        while ( kk > jj && v3[kk - 1] == 2 )
        {
          v3[--kk] = 0;
          --v26;
        }
        v34 = kk - jj;
        if ( kk - jj >= 4 * v26 )
        {
          v27 = 1;
          v29 = v34 >> 2;
          while ( 1 )
          {
            v29 >>= 2;
            if ( v29 <= 0 )
              break;
            v27 *= 2;
          }
          v28 = v27 + 1;
          v5 = 0;
          v6 = 0;
          while ( v5 < v34 )
          {
            if ( v3[v5 + jj] == 2 )
            {
              if ( ++v6 == v28 )
              {
                v5 -= v6;
              }
              else if ( v28 < v6 )
              {
                v3[v5 + jj] = 0;
              }
            }
            else
            {
              v6 = 0;
            }
            ++v5;
          }
          v7 = 0;
          v8 = 0;
          while ( v7 < v34 && (v7 <= 7 || v3[v7 + jj] != 1) )
          {
            if ( v3[v7 + jj] == 2 )
            {
              v8 = 0;
              v3[v7 + jj] = 0;
            }
            else if ( v3[v7 + jj] )
            {
              ++v8;
            }
            else
            {
              v8 = 0;
            }
            if ( v8 == 3 )
              break;
            ++v7;
          }
          jj += v34 - 1;
          v9 = 0;
          v10 = 0;
          while ( v9 < v34 && (v9 <= 7 || v3[jj - v9] != 1) )
          {
            if ( v3[jj - v9] == 2 )
            {
              v10 = 0;
              v3[jj - v9] = 0;
            }
            else if ( v3[jj - v9] )
            {
              ++v10;
            }
            else
            {
              v10 = 0;
            }
            if ( v10 == 3 )
              break;
            ++v9;
          }
        }
        else
        {
          while ( kk > jj )
          {
            if ( v3[--kk] == 2 )
              v3[kk] = 0;
          }
        }
      }
    }
  }
  for ( mm = 0; mm <= 1; ++mm )
  {
    v31 = ptr[mm];
    v32 = a1[38 * mm + 25];
    v30 = 0;
    for ( nn = 0; nn < v32; ++nn )
    {
      if ( !byte_2D348 && v31[nn] )
      {
        *(char *)(a1[38 * mm + 35] + nn) = 1;
      }
      else
      {
        *(long long *)(8 * v30 + a1[38 * mm + 32]) = *(long long *)(a1[38 * mm + 31] + 8 * nn);
        v11 = v30++;
        *(long long *)(a1[38 * mm + 33] + 8 * v11) = nn;
      }
    }
    a1[38 * mm + 34] = v30;
  }
  free(ptr[0]);
  free(v41);
  return v43 - __readfsqword(0x28u);
}



// Function: shift_boundaries @ 0x6b31
void shift_boundaries(long long a1)
{
  long long v1; // rax
  long long v2; // rax
  long long v3; // rax
  long long v4; // rax
  int i; // [rsp+Ch] [rbp-4Ch]
  long long v6; // [rsp+10h] [rbp-48h]
  long long v7; // [rsp+18h] [rbp-40h]
  long long v8; // [rsp+20h] [rbp-38h]
  long long v9; // [rsp+28h] [rbp-30h]
  long long v10; // [rsp+30h] [rbp-28h]
  long long v11; // [rsp+38h] [rbp-20h]
  long long v12; // [rsp+40h] [rbp-18h]
  long long v13; // [rsp+48h] [rbp-10h]
  long long v14; // [rsp+50h] [rbp-8h]

  for ( i = 0; i <= 1; ++i )
  {
    v10 = *(long long *)(304LL * i + a1 + 280);
    v11 = *(long long *)(304LL * (1 - i) + a1 + 280);
    v12 = *(long long *)(304LL * i + a1 + 248);
    v6 = 0;
    v7 = 0;
    v13 = *(long long *)(304LL * i + a1 + 200);
    while ( 1 )
    {
      while ( v6 < v13 && !*(char *)(v6 + v10) )
      {
        do
          v1 = v7++;
        while ( *(char *)(v1 + v11) );
        ++v6;
      }
      if ( v6 == v13 )
        break;
      v8 = v6;
      do
        ++v6;
      while ( *(char *)(v6 + v10) );
      while ( *(char *)(v7 + v11) )
        ++v7;
      do
      {
        v14 = v6 - v8;
        while ( v8 && *(long long *)(8 * v8 - 8 + v12) == *(long long *)(8 * v6 - 8 + v12) )
        {
          *(char *)(--v8 + v10) = 1;
          *(char *)(--v6 + v10) = 0;
          while ( *(char *)(v8 - 1 + v10) )
            --v8;
          do
            --v7;
          while ( *(char *)(v7 + v11) );
        }
        if ( *(char *)(v7 - 1 + v11) )
          v2 = v6;
        else
          v2 = v13;
        v9 = v2;
        while ( v6 != v13 && *(long long *)(8 * v8 + v12) == *(long long *)(8 * v6 + v12) )
        {
          v3 = v8++;
          *(char *)(v3 + v10) = 0;
          v4 = v6++;
          *(char *)(v4 + v10) = 1;
          while ( *(char *)(v6 + v10) )
            ++v6;
          while ( *(char *)(++v7 + v11) )
            v9 = v6;
        }
      }
      while ( v14 != v6 - v8 );
      while ( v9 < v6 )
      {
        *(char *)(--v8 + v10) = 1;
        *(char *)(--v6 + v10) = 0;
        do
          --v7;
        while ( *(char *)(v7 + v11) );
      }
    }
  }
}



// Function: add_change @ 0x6e92
long long *add_change(long long a1, long long a2, long long a3, long long a4, long long a5)
{
  long long *result; // rax

  result = (long long *)sub_1C945(48);
  result[3] = a1;
  result[4] = a2;
  result[1] = a4;
  result[2] = a3;
  *result = a5;
  return result;
}



// Function: build_reverse_script @ 0x6f01
long long *build_reverse_script(long long *a1)
{
  long long *v2; // [rsp+18h] [rbp-48h]
  long long v3; // [rsp+20h] [rbp-40h]
  long long i; // [rsp+28h] [rbp-38h]
  long long v5; // [rsp+30h] [rbp-30h]
  long long v6; // [rsp+38h] [rbp-28h]
  long long v7; // [rsp+40h] [rbp-20h]
  long long v8; // [rsp+48h] [rbp-18h]
  long long v9; // [rsp+50h] [rbp-10h]
  long long v10; // [rsp+58h] [rbp-8h]

  v2 = 0;
  v5 = a1[35];
  v6 = a1[73];
  v7 = a1[25];
  v8 = a1[63];
  v3 = 0;
  for ( i = 0; v3 < v7 || i < v8; ++i )
  {
    if ( *(char *)(v3 + v5) | *(char *)(i + v6) )
    {
      v9 = v3;
      v10 = i;
      while ( *(char *)(v3 + v5) )
        ++v3;
      while ( *(char *)(i + v6) )
        ++i;
      v2 = sub_6E92(v9, v10, v3 - v9, i - v10, (long long)v2);
    }
    ++v3;
  }
  return v2;
}



// Function: build_script @ 0x7037
long long *build_script(long long *a1)
{
  long long *v2; // [rsp+18h] [rbp-38h]
  long long v3; // [rsp+20h] [rbp-30h]
  long long i; // [rsp+28h] [rbp-28h]
  long long v5; // [rsp+30h] [rbp-20h]
  long long v6; // [rsp+38h] [rbp-18h]
  long long v7; // [rsp+40h] [rbp-10h]
  long long v8; // [rsp+48h] [rbp-8h]

  v2 = 0;
  v5 = a1[35];
  v6 = a1[73];
  v3 = a1[25];
  for ( i = a1[63]; v3 >= 0 || i >= 0; --i )
  {
    if ( *(char *)(v3 - 1 + v5) | *(char *)(i - 1 + v6) )
    {
      v7 = v3;
      v8 = i;
      while ( *(char *)(v3 - 1 + v5) )
        --v3;
      while ( *(char *)(i - 1 + v6) )
        --i;
      v2 = sub_6E92(v3, i, v7 - v3, v8 - i, (long long)v2);
    }
    --v3;
  }
  return v2;
}



// Function: briefly_report @ 0x7167
void briefly_report(int a1, long long a2)
{
  long long v2; // r12
  long long v3; // rbx
  char *v4; // rax

  if ( a1 )
  {
    if ( qword_2D208 )
      v2 = qword_2D208;
    else
      v2 = *(long long *)(a2 + 312);
    if ( qword_2D200 )
      v3 = qword_2D200;
    else
      v3 = *(long long *)(a2 + 8);
    if ( byte_2D2A0 )
      v4 = gettext("Files %s and %s differ\n");
    else
      v4 = gettext("Binary files %s and %s differ\n");
    sub_109A8(v4, v3, v2);
  }
}



// Function: diff_2_files @ 0x7206
long long diff_2_files(long long a1)
{
  long long v1; // rax
  long long v2; // rax
  long long v3; // rcx
  long long v4; // rax
  char *v5; // rcx
  const char *v6; // rax
  int i; // [rsp+18h] [rbp-C8h]
  int j; // [rsp+18h] [rbp-C8h]
  int k; // [rsp+18h] [rbp-C8h]
  int n; // [rsp+18h] [rbp-C8h]
  unsigned int v12; // [rsp+1Ch] [rbp-C4h]
  char v13[8]; // [rsp+20h] [rbp-C0h] BYREF
  char v14[8]; // [rsp+28h] [rbp-B8h] BYREF
  char v15[8]; // [rsp+30h] [rbp-B0h] BYREF
  char v16[8]; // [rsp+38h] [rbp-A8h] BYREF
  void *m; // [rsp+40h] [rbp-A0h]
  long long *v18; // [rsp+48h] [rbp-98h]
  long long v19; // [rsp+50h] [rbp-90h]
  long long v20; // [rsp+58h] [rbp-88h]
  long long *v21; // [rsp+60h] [rbp-80h]
  long long v22; // [rsp+68h] [rbp-78h]
  void *ptr; // [rsp+70h] [rbp-70h]
  long long *v24; // [rsp+78h] [rbp-68h]
  long long *v25; // [rsp+80h] [rbp-60h]
  void *v26; // [rsp+88h] [rbp-58h]
  long long v27; // [rsp+90h] [rbp-50h]
  long long v28; // [rsp+98h] [rbp-48h]
  long long v29[2]; // [rsp+A0h] [rbp-40h] BYREF
  long long v30; // [rsp+B0h] [rbp-30h]
  long long v31; // [rsp+B8h] [rbp-28h]
  char v32; // [rsp+C0h] [rbp-20h]
  long long v33; // [rsp+C8h] [rbp-18h]
  unsigned long long v34; // [rsp+D8h] [rbp-8h]

  v34 = __readfsqword(0x28u);
  if ( (unsigned char)sub_FA13(a1, (unsigned char)byte_2D1ED) )
  {
    if ( *(long long *)(a1 + 64) != *(long long *)(a1 + 368)
      && *(long long *)(a1 + 64) > 0
      && *(long long *)(a1 + 368) > 0
      && (*(int *)a1 < 0 || (*(int *)(a1 + 40) & 0xF000) == 0x8000)
      && (*(int *)(a1 + 304) < 0 || (*(int *)(a1 + 344) & 0xF000) == 0x8000) )
    {
LABEL_9:
      v12 = 1;
      sub_7167(1, a1);
      goto LABEL_78;
    }
    if ( *(int *)a1 != *(int *)(a1 + 304) )
    {
      v27 = 0x7FFFFFFFFFFFFFFELL;
      v1 = sub_1D2A2(*(long long *)(a1 + 72), *(long long *)(a1 + 376), 0x7FFFFFFFFFFFFFFELL);
      v28 = sub_1D2A2(8, v1, 0x7FFFFFFFFFFFFFFELL);
      for ( i = 0; i <= 1; ++i )
        *(long long *)(a1 + 304LL * i + 160) = sub_1C991(*(long long *)(a1 + 304LL * i + 160), v28);
      while ( 1 )
      {
        for ( j = 0; j <= 1; ++j )
        {
          if ( *(int *)(a1 + 304LL * j) >= 0 )
            sub_DE51(a1 + 304LL * j, v28 - *(long long *)(a1 + 304LL * j + 176));
        }
        if ( *(long long *)(a1 + 176) != *(long long *)(a1 + 480)
          || memcmp(*(const void **)(a1 + 160), *(const void **)(a1 + 464), *(long long *)(a1 + 176)) )
        {
          goto LABEL_9;
        }
        if ( v28 != *(long long *)(a1 + 176) )
          break;
        *(long long *)(a1 + 480) = 0;
        *(long long *)(a1 + 176) = *(long long *)(a1 + 480);
      }
    }
    v12 = 0;
    sub_7167(0, a1);
  }
  else
  {
    v22 = *(long long *)(a1 + 200) + *(long long *)(a1 + 504) + 4LL;
    ptr = (void *)sub_13030(v22);
    *(long long *)(a1 + 280) = (char *)ptr + 1;
    *(long long *)(a1 + 584) = (char *)ptr + *(long long *)(a1 + 200) + 3;
    sub_6122((long long *)a1);
    v29[0] = *(long long *)(a1 + 256);
    v29[1] = *(long long *)(a1 + 560);
    v19 = *(long long *)(a1 + 272) + *(long long *)(a1 + 576) + 3LL;
    v30 = sub_1C945(16 * v19);
    v31 = v30 + 8 * v19;
    v30 += 8 * (*(long long *)(a1 + 576) + 1LL);
    v31 += 8 * (*(long long *)(a1 + 576) + 1LL);
    v32 = byte_2D338;
    v20 = 1;
    while ( v19 )
    {
      v20 *= 2;
      v19 >>= 2;
    }
    v2 = v20;
    if ( v20 < 4096 )
      v2 = 4096;
    v33 = v2;
    qmemcpy(&unk_2D360, (const void *)a1, 0x130u);
    qmemcpy(&unk_2D490, (const void *)(a1 + 304), 0x130u);
    sub_5D78(0, *(long long *)(a1 + 272), 0, *(long long *)(a1 + 576), (unsigned char)byte_2D348, v29);
    free((void *)(v30 + -8 - 8LL * *(long long *)(a1 + 576)));
    sub_6B31(a1);
    if ( dword_2D1C0 == 4 )
      v18 = sub_6F01((long long *)a1);
    else
      v18 = sub_7037((long long *)a1);
    if ( byte_2D1EC || stru_2D260.fastmap )
    {
      v21 = v18;
      v12 = 0;
      while ( v21 && !v12 )
      {
        v24 = v21;
        v25 = (long long *)sub_12437(v21);
        v21 = (long long *)*v25;
        *v25 = 0;
        v12 = sub_12AF4(v24, v13, v14, v15, v16) != 0;
        *v25 = v21;
      }
    }
    else
    {
      v12 = v18 != 0;
    }
    if ( byte_2D2A0 )
    {
      sub_7167(v12, a1);
    }
    else if ( v12 || byte_2D1C8 != 1 )
    {
      if ( qword_2D208 )
        v3 = qword_2D208;
      else
        v3 = *(long long *)(a1 + 312);
      if ( qword_2D200[0] )
        v4 = qword_2D200[0];
      else
        v4 = *(long long *)(a1 + 8);
      sub_118AC(v4, v3, *(long long *)(a1 + 608) != 0);
      switch ( dword_2D1C0 )
      {
        case 1:
          sub_FCFD(v18);
          break;
        case 2:
          sub_7F2C(v18, 0);
          break;
        case 3:
          sub_7F2C(v18, 1);
          break;
        case 4:
          sub_CB85(v18);
          break;
        case 5:
          sub_CD6A(v18);
          break;
        case 6:
          sub_CEBA(v18);
          break;
        case 7:
          sub_D076(v18);
          break;
        case 8:
          sub_FF70(v18);
          break;
        default:
          abort();
      }
      sub_11E1B();
    }
    free(*(void **)(a1 + 256));
    free(ptr);
    for ( k = 0; k <= 1; ++k )
    {
      free(*(void **)(a1 + 304LL * k + 248));
      free((void *)(*(long long *)(a1 + 304LL * k + 184) + 8LL * *(long long *)(a1 + 304LL * k + 192)));
    }
    for ( m = v18; m; m = v26 )
    {
      v26 = *(void **)m;
      free(m);
    }
    if ( dword_2D1C0 == 4 || dword_2D1C0 == 5 )
    {
      for ( n = 0; n <= 1; ++n )
      {
        if ( *(char *)(a1 + 304LL * n + 288) )
        {
          v5 = gettext("No newline at end of file");
          if ( qword_2D200[n] )
            v6 = (const char *)qword_2D200[n];
          else
            v6 = *(const char **)(a1 + 304LL * n + 8);
          error(0, 0, "%s: %s\n", v6, v5);
          v12 = 2;
        }
      }
    }
  }
LABEL_78:
  if ( *(long long *)(a1 + 160) != *(long long *)(a1 + 464) )
    free(*(void **)(a1 + 160));
  free(*(void **)(a1 + 464));
  return v12;
}



// Function: print_context_label @ 0x7d13
unsigned long long print_context_label(const char *a1, long long a2, const char *a3, const char *a4)
{
  unsigned int v7; // [rsp+2Ch] [rbp-54h]
  struct tm *v8; // [rsp+30h] [rbp-50h]
  char s[56]; // [rsp+40h] [rbp-40h] BYREF
  unsigned long long v10; // [rsp+78h] [rbp-8h]

  v10 = __readfsqword(0x28u);
  sub_128C2(0);
  if ( a4 )
  {
    fprintf(stream, "%s %s", a1, a4);
  }
  else
  {
    v8 = localtime((const time_t *)(a2 + 104));
    v7 = sub_1BB4F(a2 + 16);
    if ( !v8 || !sub_1710B(s, 43, qword_2D350, v8, 0, v7) )
      sprintf(s, "%ld.%.9d", *(long long *)(a2 + 104), v7);
    fprintf(stream, "%s %s\t%s", a1, a3, s);
  }
  sub_128C2(3);
  putc_unlocked(10, stream);
  return v10 - __readfsqword(0x28u);
}



// Function: print_context_header @ 0x7e5c
unsigned long long print_context_header(long long a1, const char **a2, char a3)
{
  if ( a3 )
  {
    sub_7D13("---", a1, *a2, (const char *)qword_2D200[0]);
    return sub_7D13("+++", a1 + 304, a2[1], (const char *)qword_2D208);
  }
  else
  {
    sub_7D13("***", a1, *a2, (const char *)qword_2D200[0]);
    return sub_7D13("---", a1 + 304, a2[1], (const char *)qword_2D208);
  }
}



// Function: print_context_script @ 0x7f2c
long long print_context_script(long long *a1, char a2)
{
  long long *i; // [rsp+18h] [rbp-8h]

  if ( byte_2D1EC || stru_2D260.fastmap )
  {
    sub_8C9D(a1);
  }
  else
  {
    for ( i = a1; i; i = (long long *)*i )
      *((char *)i + 40) = 0;
  }
  qword_2D1B0 = -qword_2D448;
  qword_2D1B8 = 0x7FFFFFFFFFFFFFFFLL;
  if ( a2 )
    return sub_1245B(a1, sub_8BBF, sub_8710);
  else
    return sub_1245B(a1, sub_8BBF, sub_818F);
}



// Function: print_context_number_range @ 0x7ff1
unsigned long long print_context_number_range(long long a1, long long a2, long long a3)
{
  long long v4; // [rsp+28h] [rbp-18h] BYREF
  long long v5; // [rsp+30h] [rbp-10h] BYREF
  unsigned long long v6; // [rsp+38h] [rbp-8h]

  v6 = __readfsqword(0x28u);
  sub_129D0(a1, a2, a3, &v4, &v5);
  if ( v5 > v4 )
    fprintf(stream, "%ld,%ld", v4, v5);
  else
    fprintf(stream, "%ld", v5);
  return v6 - __readfsqword(0x28u);
}



// Function: print_context_function @ 0x80a2
size_t print_context_function(FILE *a1, long long a2)
{
  int i; // [rsp+18h] [rbp-8h]
  int j; // [rsp+1Ch] [rbp-4h]

  putc_unlocked(32, a1);
  for ( i = 0; (unsigned char)sub_133C9(*(unsigned char *)(i + a2)) && *(char *)(i + a2) != 10; ++i )
    ;
  for ( j = i; j <= i + 39 && *(char *)(j + a2) != 10; ++j )
    ;
  while ( i < j && (unsigned char)sub_133C9(*(unsigned char *)(j - 1LL + a2)) )
    --j;
  return fwrite_unlocked((const void *)(i + a2), 1u, j - i, a1);
}



// Function: pr_context_hunk @ 0x818f
unsigned long long pr_context_hunk(long long *a1)
{
  long long v1; // rax
  long long v2; // rax
  const char *v3; // rax
  const char *v4; // rax
  int v6; // [rsp+14h] [rbp-5Ch]
  long long v7; // [rsp+18h] [rbp-58h] BYREF
  long long v8; // [rsp+20h] [rbp-50h] BYREF
  long long v9; // [rsp+28h] [rbp-48h] BYREF
  long long v10; // [rsp+30h] [rbp-40h] BYREF
  long long i; // [rsp+38h] [rbp-38h]
  const char *v12; // [rsp+40h] [rbp-30h]
  long long v13; // [rsp+48h] [rbp-28h]
  long long *v14; // [rsp+50h] [rbp-20h]
  long long *v15; // [rsp+58h] [rbp-18h]
  FILE *stream; // [rsp+60h] [rbp-10h]
  unsigned long long v17; // [rsp+68h] [rbp-8h]

  v17 = __readfsqword(0x28u);
  v6 = sub_12AF4(a1, &v7, &v8, &v9, &v10);
  if ( v6 )
  {
    i = -qword_2D448;
    v1 = -qword_2D448;
    if ( v7 - qword_2D1D0 >= -qword_2D448 )
      v1 = v7 - qword_2D1D0;
    v7 = v1;
    v2 = i;
    if ( v9 - qword_2D1D0 >= i )
      v2 = v9 - qword_2D1D0;
    v9 = v2;
    if ( qword_2D430 - qword_2D1D0 <= v8 )
      v8 = qword_2D430 - 1;
    else
      v8 += qword_2D1D0;
    if ( qword_2D560 - qword_2D1D0 <= v10 )
      v10 = qword_2D560 - 1;
    else
      v10 += qword_2D1D0;
    v13 = 0;
    if ( buffer.fastmap )
      v13 = sub_8D30(qword_2D418, v7);
    sub_11B50();
    stream = ::stream;
    fputs_unlocked("***************", ::stream);
    if ( v13 )
      sub_80A2(stream, v13);
    putc_unlocked(10, stream);
    sub_128C2(4);
    fputs_unlocked("*** ", stream);
    sub_7FF1((long long)&unk_2D360, v7, v8);
    fputs_unlocked(" ****", stream);
    sub_128C2(3);
    putc_unlocked(10, stream);
    if ( (v6 & 1) != 0 )
    {
      v14 = a1;
      for ( i = v7; i <= v8; ++i )
      {
        sub_128C2(2);
        while ( v14 && i >= v14[3] + v14[2] )
          v14 = (long long *)*v14;
        v12 = " ";
        if ( v14 && i >= v14[3] )
        {
          if ( (long long)v14[1] <= 0 )
            v3 = "-";
          else
            v3 = "!";
          v12 = v3;
        }
        sub_124FE(v12, qword_2D418 + 8 * i, 1);
        sub_128C2(3);
        if ( *(char *)(*(long long *)(qword_2D418 + 8 * (i + 1)) - 1LL) == 10 )
          putc_unlocked(10, stream);
      }
    }
    sub_128C2(4);
    fputs_unlocked("--- ", stream);
    sub_7FF1((long long)&unk_2D490, v9, v10);
    fputs_unlocked(" ----", stream);
    sub_128C2(3);
    putc_unlocked(10, stream);
    if ( (v6 & 2) != 0 )
    {
      v15 = a1;
      for ( i = v9; i <= v10; ++i )
      {
        sub_128C2(1);
        while ( v15 && i >= v15[4] + v15[1] )
          v15 = (long long *)*v15;
        v12 = " ";
        if ( v15 && i >= v15[4] )
        {
          if ( (long long)v15[2] <= 0 )
            v4 = "+";
          else
            v4 = "!";
          v12 = v4;
        }
        sub_124FE(v12, qword_2D548 + 8 * i, 1);
        sub_128C2(3);
        if ( *(char *)(*(long long *)(qword_2D548 + 8 * (i + 1)) - 1LL) == 10 )
          putc_unlocked(10, stream);
      }
    }
  }
  return v17 - __readfsqword(0x28u);
}



// Function: print_unidiff_number_range @ 0x863e
unsigned long long print_unidiff_number_range(long long a1, long long a2, long long a3)
{
  long long v4; // [rsp+28h] [rbp-18h] BYREF
  long long v5; // [rsp+30h] [rbp-10h] BYREF
  unsigned long long v6; // [rsp+38h] [rbp-8h]

  v6 = __readfsqword(0x28u);
  sub_129D0(a1, a2, a3, &v4, &v5);
  if ( v5 > v4 )
  {
    fprintf(stream, "%ld,%ld", v4, v5 - v4 + 1);
  }
  else if ( v5 >= v4 )
  {
    fprintf(stream, "%ld", v5);
  }
  else
  {
    fprintf(stream, "%ld,0", v5);
  }
  return v6 - __readfsqword(0x28u);
}



// Function: pr_unidiff_hunk @ 0x8710
unsigned long long pr_unidiff_hunk(long long *a1)
{
  long long v1; // rax
  long long v2; // rax
  long long v3; // rax
  int v4; // edx
  long long v5; // rax
  long long v7; // rax
  long long v10; // [rsp+10h] [rbp-70h] BYREF
  long long v11; // [rsp+18h] [rbp-68h] BYREF
  long long v12; // [rsp+20h] [rbp-60h] BYREF
  long long v13; // [rsp+28h] [rbp-58h] BYREF
  long long v14; // [rsp+30h] [rbp-50h]
  long long v15; // [rsp+38h] [rbp-48h]
  long long v16; // [rsp+40h] [rbp-40h]
  long long *v17; // [rsp+48h] [rbp-38h]
  long long v18; // [rsp+50h] [rbp-30h]
  FILE *stream; // [rsp+58h] [rbp-28h]
  long long v20; // [rsp+60h] [rbp-20h]
  long long v21; // [rsp+68h] [rbp-18h]
  char **v22; // [rsp+70h] [rbp-10h]
  unsigned long long v23; // [rsp+78h] [rbp-8h]

  v23 = __readfsqword(0x28u);
  if ( (unsigned int)sub_12AF4(a1, &v10, &v11, &v12, &v13) )
  {
    v14 = -qword_2D448;
    v1 = -qword_2D448;
    if ( v10 - qword_2D1D0 >= -qword_2D448 )
      v1 = v10 - qword_2D1D0;
    v10 = v1;
    v2 = v14;
    if ( v12 - qword_2D1D0 >= v14 )
      v2 = v12 - qword_2D1D0;
    v12 = v2;
    if ( qword_2D430 - qword_2D1D0 <= v11 )
      v11 = qword_2D430 - 1;
    else
      v11 += qword_2D1D0;
    if ( qword_2D560 - qword_2D1D0 <= v13 )
      v13 = qword_2D560 - 1;
    else
      v13 += qword_2D1D0;
    v18 = 0;
    if ( buffer.fastmap )
      v18 = sub_8D30(qword_2D418, v10);
    sub_11B50();
    stream = ::stream;
    sub_128C2(4);
    fputs_unlocked("@@ -", stream);
    sub_863E((long long)&unk_2D360, v10, v11);
    fputs_unlocked(" +", stream);
    sub_863E((long long)&unk_2D490, v12, v13);
    fputs_unlocked(" @@", stream);
    sub_128C2(3);
    if ( v18 )
      sub_80A2(stream, v18);
    putc_unlocked(10, stream);
    v17 = a1;
    v14 = v10;
    v15 = v12;
    while ( v14 <= v11 || v15 <= v13 )
    {
      if ( v17 && v14 >= v17[3] )
      {
        v16 = v17[2];
        while ( v16-- )
        {
          v5 = v14++;
          v21 = qword_2D418 + 8 * v5;
          sub_128C2(2);
          putc_unlocked(45, stream);
          if ( byte_2D2B0 && (byte_2D2B1 != 1 || **(char **)v21 != 10) )
            putc_unlocked(9, stream);
          sub_124FE(0, v21, 1);
          sub_128C2(3);
          if ( *(char *)(*(long long *)(v21 + 8) - 1LL) == 10 )
            putc_unlocked(10, stream);
        }
        v16 = v17[1];
        while ( v16-- )
        {
          v7 = v15++;
          v20 = qword_2D548 + 8 * v7;
          sub_128C2(1);
          putc_unlocked(43, stream);
          if ( byte_2D2B0 && (byte_2D2B1 != 1 || **(char **)v20 != 10) )
            putc_unlocked(9, stream);
          sub_124FE(0, v20, 1);
          sub_128C2(3);
          if ( *(char *)(*(long long *)(v20 + 8) - 1LL) == 10 )
            putc_unlocked(10, stream);
        }
        v17 = (long long *)*v17;
      }
      else
      {
        v3 = v14++;
        v22 = (char **)(qword_2D418 + 8 * v3);
        if ( byte_2D2B1 != 1 || **v22 != 10 )
        {
          if ( byte_2D2B0 )
            v4 = 9;
          else
            v4 = 32;
          putc_unlocked(v4, stream);
        }
        sub_124CF(0, v22);
        ++v15;
      }
    }
  }
  return v23 - __readfsqword(0x28u);
}



// Function: find_hunk @ 0x8bbf
long long *find_hunk(long long *a1)
{
  long long v1; // rax
  long long v4; // [rsp+20h] [rbp-20h]
  long long v5; // [rsp+28h] [rbp-18h]
  long long *v6; // [rsp+30h] [rbp-10h]

  do
  {
    v4 = a1[3] + a1[2];
    v5 = a1[4] + a1[1];
    v6 = a1;
    a1 = (long long *)*a1;
    if ( a1 && *((char *)a1 + 40) )
      v1 = qword_2D1D0;
    else
      v1 = 2 * qword_2D1D0 + 1;
    if ( a1 && a1[3] - v4 != a1[4] - v5 )
      abort();
  }
  while ( a1 && v1 > a1[3] - v4 );
  return v6;
}



// Function: mark_ignorable @ 0x8c9d
unsigned long long mark_ignorable(long long *a1)
{
  char v3[8]; // [rsp+10h] [rbp-30h] BYREF
  char v4[8]; // [rsp+18h] [rbp-28h] BYREF
  char v5[8]; // [rsp+20h] [rbp-20h] BYREF
  char v6[8]; // [rsp+28h] [rbp-18h] BYREF
  long long *v7; // [rsp+30h] [rbp-10h]
  unsigned long long v8; // [rsp+38h] [rbp-8h]

  v8 = __readfsqword(0x28u);
  while ( a1 )
  {
    v7 = (long long *)*a1;
    *a1 = 0;
    *((char *)a1 + 40) = (unsigned int)sub_12AF4(a1, v3, v4, v5, v6) == 0;
    *a1 = (long long)v7;
    a1 = v7;
  }
  return v8 - __readfsqword(0x28u);
}



// Function: find_function @ 0x8d30
char *find_function(long long a1, long long a2)
{
  unsigned long long v2; // rax
  long long v4; // [rsp+20h] [rbp-20h]
  long long v5; // [rsp+28h] [rbp-18h]
  char *string; // [rsp+30h] [rbp-10h]

  v4 = a2;
  v5 = qword_2D1B0;
  qword_2D1B0 = a2;
  while ( --v4 >= v5 )
  {
    string = *(char **)(8 * v4 + a1);
    v2 = *(long long *)(8 * (v4 + 1) + a1) - (long long)string - 1LL;
    if ( v2 > 0x7FFFFFFF )
      LODWORD(v2) = 0x7FFFFFFF;
    if ( re_search(&buffer, string, v2, 0, v2, 0) >= 0 )
    {
      qword_2D1B8 = v4;
      return string;
    }
  }
  if ( qword_2D1B8 == 0x7FFFFFFFFFFFFFFFLL )
    return 0;
  else
    return *(char **)(8 * qword_2D1B8 + a1);
}



// Function: option_list @ 0x8e44
long long option_list(long long a1, int a2)
{
  int i; // [rsp+14h] [rbp-1Ch]
  int j; // [rsp+14h] [rbp-1Ch]
  long long v5; // [rsp+18h] [rbp-18h]
  char *v6; // [rsp+20h] [rbp-10h]
  long long v7; // [rsp+28h] [rbp-8h]

  v5 = 1;
  for ( i = 0; i < a2; ++i )
    v5 += sub_1A4F6(*(long long *)(8LL * i + a1)) + 1;
  v7 = sub_1C945(v5);
  v6 = (char *)v7;
  for ( j = 0; j < a2; ++j )
  {
    *v6 = 32;
    v6 = (char *)sub_1A550(v6 + 1, *(long long *)(8LL * j + a1));
  }
  *v6 = 0;
  return v7;
}



// Function: exclude_options @ 0x8f0e
long long exclude_options()
{
  if ( byte_2D1EF )
    return 268435472;
  else
    return 0x10000000;
}



// Function: main @ 0x8f2f
long long main(int a1, char **a2, char **a3)
{
  long long v3; // rax
  unsigned int v4; // eax
  size_t v5; // rax
  size_t v6; // rax
  long long v7; // r13
  long long v8; // r12
  long long v9; // rbx
  int v10; // r14d
  int v11; // eax
  unsigned int v13; // eax
  unsigned int v14; // eax
  long long v15; // rax
  int v16; // eax
  unsigned long long v17; // rax
  long long v18; // rax
  long long v19; // rax
  const char *v20; // rax
  const char *v21; // rax
  bool v22; // al
  int v23; // eax
  char v24; // [rsp+16h] [rbp-AAh]
  char v25; // [rsp+17h] [rbp-A9h]
  int status; // [rsp+18h] [rbp-A8h]
  int v27; // [rsp+1Ch] [rbp-A4h]
  unsigned int i; // [rsp+20h] [rbp-A0h]
  unsigned int j; // [rsp+20h] [rbp-A0h]
  unsigned int k; // [rsp+20h] [rbp-A0h]
  int v31; // [rsp+24h] [rbp-9Ch]
  int v32; // [rsp+28h] [rbp-98h]
  int v33; // [rsp+2Ch] [rbp-94h]
  long long v34; // [rsp+30h] [rbp-90h] BYREF
  long long v35; // [rsp+38h] [rbp-88h] BYREF
  char *endptr; // [rsp+40h] [rbp-80h] BYREF
  long long v37; // [rsp+48h] [rbp-78h]
  unsigned long long v38; // [rsp+50h] [rbp-70h]
  intmax_t v39; // [rsp+58h] [rbp-68h]
  char *s; // [rsp+60h] [rbp-60h]
  char *s1; // [rsp+68h] [rbp-58h]
  unsigned long long v42; // [rsp+70h] [rbp-50h]
  unsigned long long v43; // [rsp+78h] [rbp-48h]
  unsigned long long v44; // [rsp+80h] [rbp-40h]
  unsigned long long v45; // [rsp+88h] [rbp-38h]
  unsigned long long v46; // [rsp+90h] [rbp-30h]
  unsigned long long v47; // [rsp+98h] [rbp-28h]

  v47 = __readfsqword(0x28u);
  status = 0;
  v31 = -1;
  v37 = -1;
  v24 = 0;
  v38 = 0;
  v25 = 0;
  v34 = 0;
  v35 = 0;
  ::status = 2;
  sub_19A53(*a2, a2, a3);
  setlocale(6, &locale);
  bindtextdomain("diffutils", "/usr/local/share/locale");
  textdomain("diffutils");
  sub_13721(0);
  qword_2D600 = (long long)&buffer;
  qword_2D640 = (long long)&stru_2D260;
  re_set_syntax(0x50A46u);
  qword_2D340 = sub_139C7();
  byte_2D5C8 = 0;
  sub_1CFB9();
  while ( 1 )
  {
    v16 = getopt_long(a1, a2, "0123456789abBcC:dD:eEfF:hHiI:lL:nNpPqrsS:tTuU:vwW:x:X:yZ", &longopts, 0);
    v27 = v16;
    if ( v16 == -1 )
      break;
    switch ( v16 )
    {
      case 0:
      case 104:
      case 128:
      case 133:
        break;
      case 48:
      case 49:
      case 50:
      case 51:
      case 52:
      case 53:
      case 54:
      case 55:
      case 56:
      case 57:
        if ( (unsigned int)(v31 - 48) <= 9 )
        {
          if ( v37 - (v16 <= 51) > 0x666666666666665LL )
            v3 = 0x3FFFFFFFFFFFFFFFLL;
          else
            v3 = 10 * v37 + v16 - 48;
        }
        else
        {
          v3 = v16 - 48;
        }
        v37 = v3;
        break;
      case 66:
        byte_2D1EC = 1;
        break;
      case 67:
      case 85:
        if ( optarg )
        {
          v39 = strtoimax(optarg, &endptr, 10);
          if ( *endptr || v39 < 0 )
            sub_A338("invalid context length '%s'", optarg);
          if ( v39 > 0x3FFFFFFFFFFFFFFFLL )
            v39 = 0x3FFFFFFFFFFFFFFFLL;
        }
        else
        {
          v39 = 3;
        }
        if ( v27 == 85 )
          v4 = 3;
        else
          v4 = 2;
        sub_A63E(v4);
        if ( v39 > qword_2D1D0 )
          qword_2D1D0 = v39;
        v24 = 1;
        break;
      case 68:
        sub_A63E(7);
        v5 = strlen(optarg);
        s = (char *)sub_1C945(7 * v5 + 97);
        sprintf(
          s,
          "%%=%c#ifndef %s\n"
          "%%<#endif /* ! %s */\n"
          "%c#ifdef %s\n"
          "%%>#endif /* %s */\n"
          "%c#ifndef %s\n"
          "%%<#else /* %s */\n"
          "%%>#endif /* %s */\n",
          0,
          optarg,
          optarg,
          0,
          optarg,
          optarg,
          0,
          optarg,
          optarg,
          optarg);
        for ( i = 0; i <= 3; ++i )
        {
          sub_A5AF(&(&::s1)[i], s, "-D");
          v6 = strlen(s);
          s += v6 + 1;
        }
        break;
      case 69:
        if ( (unsigned int)dword_2D1E8 <= 3 )
          dword_2D1E8 |= 1u;
        break;
      case 70:
        sub_A107(&unk_2D5E0, optarg);
        break;
      case 72:
        byte_2D338 = 1;
        break;
      case 73:
        sub_A107(&qword_2D620, optarg);
        break;
      case 76:
        if ( qword_2D200[0] )
        {
          if ( qword_2D208 )
          {
            sub_10966("too many file label options");
LABEL_51:
            sub_A63E(6);
          }
          else
          {
            qword_2D208 = (long long)optarg;
          }
        }
        else
        {
          qword_2D200[0] = (long long)optarg;
        }
        break;
      case 78:
        byte_2D648 = 1;
        break;
      case 80:
        byte_2D649 = 1;
        break;
      case 83:
        sub_A5AF(&qword_2D2B8, optarg, "-S");
        break;
      case 84:
        byte_2D2B0 = 1;
        break;
      case 87:
        v39 = strtoimax(optarg, &endptr, 10);
        if ( v39 <= 0 || *endptr )
          sub_A338("invalid width '%s'", optarg);
        if ( v38 != v39 )
        {
          if ( v38 )
            sub_10966("conflicting width options");
          v38 = v39;
        }
        break;
      case 88:
        v14 = sub_8F0E();
        if ( !(unsigned int)sub_148B2(sub_142E9, qword_2D340, optarg, v14, 10) )
          break;
        sub_10925(optarg);
LABEL_68:
        sub_A63E(8);
        break;
      case 90:
        if ( (unsigned int)dword_2D1E8 <= 3 )
          dword_2D1E8 |= 2u;
        break;
      case 97:
        byte_2D1D8 = 1;
        break;
      case 98:
        if ( (unsigned int)dword_2D1E8 <= 3 )
          dword_2D1E8 = 4;
        break;
      case 99:
        sub_A63E(2);
        if ( qword_2D1D0 <= 2 )
          qword_2D1D0 = 3;
        break;
      case 100:
        byte_2D348 = 1;
        break;
      case 101:
        sub_A63E(4);
        break;
      case 102:
        sub_A63E(5);
        break;
      case 105:
        byte_2D1EE = 1;
        break;
      case 108:
        if ( !path[0] )
          sub_A338("pagination not supported on this host", 0);
        byte_2D2C0 = 1;
        signal(17, 0);
        break;
      case 110:
        goto LABEL_51;
      case 112:
        v25 = 1;
        sub_A107(&unk_2D5E0, "^[[:alpha:]$_]");
        break;
      case 113:
        byte_2D2A0 = 1;
        break;
      case 114:
        byte_2D5C9 = 1;
        break;
      case 115:
        byte_2D64A = 1;
        break;
      case 116:
        byte_2D2A1 = 1;
        break;
      case 117:
        sub_A63E(3);
        if ( qword_2D1D0 <= 2 )
          qword_2D1D0 = 3;
        break;
      case 118:
        v7 = sub_1A014("Len Tower");
        v8 = sub_1A014("Richard Stallman");
        v9 = sub_1A014("David Hayes");
        v10 = sub_1A014("Mike Haertel");
        v11 = sub_1A014("Paul Eggert");
        sub_1C71C(
          (int)stdout,
          (unsigned int)"diff",
          (unsigned int)"GNU diffutils",
          (int)off_2D0C0,
          v11,
          v10,
          v9,
          v8,
          v7,
          0);
        sub_A3AE();
        return 0;
      case 119:
        dword_2D1E8 = 5;
        break;
      case 120:
        v13 = sub_8F0E();
        sub_142E9(qword_2D340, optarg, v13);
        break;
      case 121:
        goto LABEL_68;
      case 129:
        sub_A5AF(&v34, optarg, "--from-file");
        break;
      case 130:
        sub_A405();
        sub_A3AE();
        return 0;
      case 131:
        v39 = strtoimax(optarg, &endptr, 10);
        if ( *endptr || v39 < 0 )
          sub_A338("invalid horizon length '%s'", optarg);
        v15 = v39;
        if ( qword_2D1E0 >= v39 )
          v15 = qword_2D1E0;
        qword_2D1E0 = v15;
        break;
      case 132:
        byte_2D1EF = 1;
        break;
      case 134:
        byte_2D319 = 1;
        break;
      case 135:
        sub_A63E(7);
        for ( j = 0; j <= 2; ++j )
          sub_A5AF(&qword_2D300[j], optarg, "--line-format");
        break;
      case 136:
        byte_2D1F0 = 1;
        break;
      case 137:
        byte_2D1EF = 0;
        break;
      case 138:
        sub_A63E(1);
        break;
      case 139:
        sub_A63E(8);
        byte_2D318 = 1;
        break;
      case 140:
        byte_2D2B2 = 1;
        break;
      case 141:
        byte_2D2B1 = 1;
        break;
      case 142:
        byte_2D31A = 1;
        break;
      case 143:
        v39 = strtoimax(optarg, &endptr, 10);
        if ( v39 <= 0 || *endptr )
          sub_A338("invalid tabsize '%s'", optarg);
        if ( qword_2D2A8 != v39 )
        {
          if ( qword_2D2A8 )
            sub_10966("conflicting tabsize options");
          qword_2D2A8 = v39;
        }
        break;
      case 144:
        sub_A5AF(&v35, optarg, "--to-file");
        break;
      case 145:
      case 146:
      case 147:
        sub_A63E(7);
        v27 -= 145;
        sub_A5AF(&qword_2D300[v27], optarg, &aUnchangedLineF[24 * v27]);
        break;
      case 148:
      case 149:
      case 150:
      case 151:
        sub_A63E(7);
        v27 -= 148;
        sub_A5AF(&(&::s1)[v27], optarg, &aUnchangedGroup[25 * v27]);
        break;
      case 152:
        sub_A682(optarg);
        break;
      case 153:
        sub_1140F(optarg);
        break;
      case 154:
        byte_2D5C8 = 1;
        break;
      default:
        sub_A338(0, 0);
        break;
    }
    v31 = v27;
  }
  if ( dword_2D1C4 == 1 )
  {
    s1 = getenv("TERM");
    if ( s1 )
    {
      if ( !strcmp(s1, "dumb") )
        dword_2D1C4 = 0;
    }
  }
  if ( !dword_2D1C0 )
  {
    if ( v25 )
    {
      sub_A63E(2);
      if ( v37 < 0 )
        qword_2D1D0 = 3;
    }
    else
    {
      sub_A63E(1);
    }
  }
  if ( dword_2D1C0 == 2 && !(unsigned char)sub_14CDF(2) )
    qword_2D350 = (long long)"%a %b %e %T %Y";
  else
    qword_2D350 = (long long)"%Y-%m-%d %H:%M:%S.%N %z";
  if ( v37 >= 0 && (dword_2D1C0 == 2 || dword_2D1C0 == 3) && (v37 > qword_2D1D0 || v37 < qword_2D1D0 && v24 != 1) )
    qword_2D1D0 = v37;
  if ( !qword_2D2A8 )
    qword_2D2A8 = 8;
  if ( !v38 )
    v38 = 130;
  if ( byte_2D2A1 )
    v17 = 1;
  else
    v17 = qword_2D2A8;
  v42 = v17;
  v43 = v38;
  v44 = v17 + 3;
  v45 = ((v17 + 3) >> 1) + (v38 >> 1) + ((unsigned char)(v17 + 3) & (unsigned char)v38 & 1);
  v46 = v45 - v45 % v17;
  if ( v46 <= 3 || v43 <= v46 )
  {
    v18 = 0;
  }
  else
  {
    v18 = v43 - v46;
    if ( v46 - 3 <= v43 - v46 )
      v18 = v46 - 3;
  }
  qword_2D320 = v18;
  if ( v18 )
    v19 = v46;
  else
    v19 = v43;
  qword_2D328 = v19;
  if ( qword_2D1E0 < qword_2D1D0 )
    qword_2D1E0 = qword_2D1D0;
  sub_A296(&unk_2D5E0);
  sub_A296(&qword_2D620);
  if ( dword_2D1C0 == 7 )
  {
    for ( k = 0; k <= 2; ++k )
    {
      if ( !qword_2D300[k] )
        qword_2D300[k] = (long long)"%l\n";
    }
    if ( !qword_2D2E8 )
    {
      if ( qword_2D2F8 )
        v20 = (const char *)qword_2D2F8;
      else
        v20 = "%<";
      qword_2D2E8 = (long long)v20;
    }
    if ( !qword_2D2F0 )
    {
      if ( qword_2D2F8 )
        v21 = (const char *)qword_2D2F8;
      else
        v21 = "%>";
      qword_2D2F0 = (long long)v21;
    }
    if ( !::s1 )
      ::s1 = "%=";
    if ( !qword_2D2F8 )
      qword_2D2F8 = sub_12FA9(qword_2D2E8, qword_2D2F0, &locale);
  }
  if ( dword_2D1C0 == 7 )
    v22 = !*::s1 || !strcmp(::s1, "%=") && !*(char *)qword_2D300[0];
  else
    v22 = ((dword_2D1C0 != 8) | (unsigned char)byte_2D31A) != 0;
  byte_2D1C8 = v22;
  v23 = qword_2D620 || dword_2D1E8;
  byte_2D1ED = (byte_2D2A0 & 1 & ~((unsigned char)(byte_2D1EE | byte_2D1EC | byte_2D2B2) | v23)) != 0;
  qword_2D330 = sub_8E44((long long)(a2 + 1), optind - 1);
  if ( v34 )
  {
    if ( !v35 )
      goto LABEL_187;
    sub_10966("--from-file and --to-file both specified");
    do
    {
      v33 = sub_A746(0, v34, a2[optind]);
      if ( status < v33 )
        status = v33;
      ++optind;
LABEL_187:
      ;
    }
    while ( a1 > optind );
  }
  else if ( v35 )
  {
    while ( a1 > optind )
    {
      v32 = sub_A746(0, a2[optind], v35);
      if ( status < v32 )
        status = v32;
      ++optind;
    }
  }
  else
  {
    if ( a1 - optind != 2 )
    {
      if ( a1 - optind <= 1 )
        sub_A338("missing operand after '%s'", a2[a1 - 1]);
      sub_A338("extra operand '%s'", a2[optind + 2]);
    }
    status = sub_A746(0, a2[optind], a2[optind + 1]);
  }
  sub_10BCA();
  sub_A3AE();
  exit(status);
}



// Function: add_regexp @ 0xa107
void add_regexp(long long a1, const char *a2)
{
  long long v2; // rax
  unsigned char v3; // [rsp+1Fh] [rbp-31h]
  long long v4; // [rsp+20h] [rbp-30h]
  long long v5; // [rsp+28h] [rbp-28h]
  unsigned long long v6; // [rsp+30h] [rbp-20h]
  size_t length; // [rsp+38h] [rbp-18h]
  const char *v8; // [rsp+40h] [rbp-10h]
  unsigned long long v9; // [rsp+48h] [rbp-8h]

  length = strlen(a2);
  v8 = re_compile_pattern(a2, length, *(struct re_pattern_buffer **)(a1 + 32));
  if ( v8 )
  {
    error(2, 0, "%s: %s", a2, v8);
  }
  else
  {
    v4 = *(long long *)a1;
    v5 = *(long long *)(a1 + 8);
    *(char *)(a1 + 24) = *(long long *)a1 != 0;
    v3 = *(char *)(a1 + 24);
    *(long long *)(a1 + 8) = length + v5 + 2 * v3;
    v9 = *(long long *)(a1 + 8);
    v6 = *(long long *)(a1 + 16);
    if ( v6 <= v9 )
    {
      if ( !v6 )
        v6 = 1;
      do
        v6 *= 2LL;
      while ( v6 <= v9 );
      *(long long *)(a1 + 16) = v6;
      v4 = sub_1C991(v4, v6);
      *(long long *)a1 = v4;
    }
    if ( v3 )
    {
      *(char *)(v4 + v5) = 92;
      v2 = v5 + 1;
      v5 += 2;
      *(char *)(v4 + v2) = 124;
    }
    memcpy((void *)(v5 + v4), a2, length + 1);
  }
}



// Function: summarize_regexp_list @ 0xa296
void summarize_regexp_list(const char **a1)
{
  const char *v1; // rbx
  const char *v2; // [rsp+18h] [rbp-18h]

  if ( *a1 )
  {
    v1 = a1[4];
    *((long long *)v1 + 4) = sub_1C945(256);
    if ( *((char *)a1 + 24) )
    {
      v2 = re_compile_pattern(*a1, (size_t)a1[1], (struct re_pattern_buffer *)a1[4]);
      if ( v2 )
        error(2, 0, "%s: %s", *a1, v2);
    }
  }
}



// Function: try_help @ 0xa338
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
  v3 = qword_3D8A8;
  v4 = gettext("Try '%s --help' for more information.");
  error(2, 0, v4, v3);
  return sub_A3AE();
}



// Function: check_stdout @ 0xa3ae
int check_stdout()
{
  int result; // eax
  char *v1; // rax

  if ( ferror_unlocked(stdout) )
    sub_10966("write failed");
  result = fclose(stdout);
  if ( result )
  {
    v1 = gettext("standard output");
    return sub_10925(v1);
  }
  return result;
}



// Function: usage @ 0xa405
long long usage()
{
  long long v0; // rbx
  char *v1; // rax
  char *v2; // rax
  FILE *v3; // rsi
  char *v4; // rdi
  char *j; // rax
  int v7; // [rsp+4h] [rbp-2Ch]
  const char **i; // [rsp+8h] [rbp-28h]
  const char *s; // [rsp+10h] [rbp-20h]
  char *v10; // [rsp+18h] [rbp-18h]

  v0 = qword_3D8A8;
  v1 = gettext("Usage: %s [OPTION]... FILES\n");
  printf(v1, v0);
  v2 = gettext("Compare FILES line by line.");
  printf("%s\n\n", v2);
  v3 = stdout;
  v4 = gettext("Mandatory arguments to long options are mandatory for short options too.\n");
  fputs_unlocked(v4, v3);
  for ( i = (const char **)off_2C6E0; *i; ++i )
  {
    if ( **i )
    {
      s = gettext(*i);
      for ( j = strchr(s, 10); ; j = strchr(v10 + 1, 10) )
      {
        v10 = j;
        if ( !j )
          break;
        v7 = (int)j + 1 - (int)s;
        if ( v7 > 4095 )
          __assert_fail("msglen < 4096", "diff.c", 0x3F8u, "usage");
        printf("  %.*s", v7, s);
        s = v10 + 1;
      }
      if ( *s == 32 || *s == 45 )
        printf("  %s\n", s);
      else
        printf("%s\n", s);
    }
    else
    {
      putchar_unlocked(10);
    }
  }
  return sub_1C800();
}



// Function: specify_value @ 0xa5af
const char **specify_value(const char **a1, const char *a2, long long a3)
{
  char *v3; // rax

  if ( *a1 && strcmp(*a1, a2) )
  {
    v3 = gettext("conflicting %s option value '%s'");
    error(0, 0, v3, a3, a2);
    sub_A338(0, 0);
  }
  *a1 = a2;
  return a1;
}



// Function: specify_style @ 0xa63e
long long specify_style(unsigned int a1)
{
  long long result; // rax

  result = (unsigned int)dword_2D1C0;
  if ( a1 != dword_2D1C0 )
  {
    if ( dword_2D1C0 )
      sub_A338("conflicting output style options", 0);
    dword_2D1C0 = a1;
    return a1;
  }
  return result;
}



// Function: specify_colors_style @ 0xa682
void specify_colors_style(const char *a1)
{
  if ( a1 && strcmp(a1, "auto") )
  {
    if ( !strcmp(a1, "always") )
    {
      dword_2D1C4 = 2;
    }
    else if ( !strcmp(a1, "never") )
    {
      dword_2D1C4 = 0;
    }
    else
    {
      sub_A338("invalid color '%s'", (long long)a1);
    }
  }
  else
  {
    dword_2D1C4 = 1;
  }
}



// Function: set_mtime_to_now @ 0xa723
long long set_mtime_to_now(long long a1)
{
  return sub_14C76(a1 + 88);
}



// Function: compare_files @ 0xa746
long long compare_files(long long a1, char *a2, char *a3)
{
  const char *v3; // rax
  int v5; // eax
  int v6; // eax
  int i; // ebx
  long long *v8; // rax
  long long v9; // rdi
  long long v10; // rdi
  long long v11; // rdi
  long long v12; // rdi
  long long v13; // rdi
  long long v14; // rdi
  long long v15; // rdi
  long long v16; // rdi
  long long v17; // rdi
  __off_t v18; // rcx
  bool v19; // al
  int j; // ebx
  bool v21; // al
  int k; // ebx
  int m; // ebx
  long long v24; // rax
  bool v25; // al
  int v26; // eax
  bool v27; // al
  const char *v28; // r12
  char *v29; // rbx
  const char *v30; // rdx
  char *v31; // rax
  int n; // ebx
  int ii; // ebx
  char *v34; // rdx
  char *v35; // rax
  char *v36; // rdx
  char *v37; // rax
  char *v38; // rax
  char *v39; // [rsp+8h] [rbp-308h]
  char *v40; // [rsp+10h] [rbp-300h]
  bool v42; // [rsp+23h] [rbp-2EDh]
  unsigned int v43; // [rsp+24h] [rbp-2ECh]
  int v44; // [rsp+2Ch] [rbp-2E4h]
  int v45; // [rsp+34h] [rbp-2DCh]
  char *ptr; // [rsp+38h] [rbp-2D8h]
  char *v47; // [rsp+40h] [rbp-2D0h]
  char *s1; // [rsp+58h] [rbp-2B8h]
  long long v49; // [rsp+60h] [rbp-2B0h]
  char *file; // [rsp+68h] [rbp-2A8h]
  __off_t v51; // [rsp+78h] [rbp-298h]
  char *v52; // [rsp+80h] [rbp-290h]
  char *v53; // [rsp+88h] [rbp-288h]
  int s[2]; // [rsp+90h] [rbp-280h] BYREF
  char *s2; // [rsp+98h] [rbp-278h]
  long long v56; // [rsp+A0h] [rbp-270h] BYREF
  long long v57; // [rsp+A8h] [rbp-268h]
  long long v58; // [rsp+B0h] [rbp-260h] BYREF
  __int128 v59; // [rsp+B8h] [rbp-258h]
  long long v60; // [rsp+C8h] [rbp-248h]
  long long v61; // [rsp+D0h] [rbp-240h]
  long long v62; // [rsp+D8h] [rbp-238h]
  long long v63[3]; // [rsp+E0h] [rbp-230h]
  long long v64; // [rsp+F8h] [rbp-218h]
  long long v65; // [rsp+100h] [rbp-210h]
  long long v66; // [rsp+108h] [rbp-208h]
  long long v67; // [rsp+110h] [rbp-200h]
  long long v68; // [rsp+118h] [rbp-1F8h]
  long long v69; // [rsp+120h] [rbp-1F0h]
  long long v70; // [rsp+128h] [rbp-1E8h]
  int fd; // [rsp+1C0h] [rbp-150h]
  char *v72; // [rsp+1C8h] [rbp-148h]
  long long v73[3]; // [rsp+1D0h] [rbp-140h] BYREF
  int v74; // [rsp+1E8h] [rbp-128h]
  long long v75; // [rsp+1ECh] [rbp-124h]
  long long v76; // [rsp+1F8h] [rbp-118h]
  long long v77; // [rsp+200h] [rbp-110h]
  long long v78; // [rsp+228h] [rbp-E8h]
  long long v79; // [rsp+238h] [rbp-D8h]
  long long v80; // [rsp+2F0h] [rbp-20h]
  unsigned long long v81; // [rsp+2F8h] [rbp-18h]

  v40 = a2;
  v39 = a3;
  v81 = __readfsqword(0x28u);
  v43 = 0;
  if ( (!a2 || !a3) && (byte_2D649 != 1 || !a3) && byte_2D648 != 1 )
  {
    if ( a2 )
      v3 = a2;
    else
      v3 = a3;
    sub_109A8("Only in %s: %s\n", *(const char **)(a1 + 304LL * (a2 == 0) + 8), v3);
    return 1;
  }
  memset(s, 0, 0x260u);
  v80 = a1;
  if ( a2 )
    v5 = -2;
  else
    v5 = -1;
  s[0] = v5;
  if ( v39 )
    v6 = -2;
  else
    v6 = -1;
  fd = v6;
  if ( !a2 )
    v40 = v39;
  if ( !v39 )
    v39 = v40;
  if ( a1 )
  {
    ptr = (char *)sub_14ADE(*(long long *)(a1 + 8), v40, 0);
    s2 = ptr;
    v47 = (char *)sub_14ADE(*(long long *)(a1 + 312), v39, 0);
    v72 = v47;
  }
  else
  {
    ptr = 0;
    v47 = 0;
    s2 = v40;
    v72 = v39;
  }
  for ( i = 0; i <= 1; ++i )
  {
    if ( *((int *)&v56 + 76 * i - 4) == -1 )
      continue;
    if ( i && !strcmp((const char *)*(&v57 + 38 * i - 2), s2) )
    {
      *((int *)&v56 + 76 * i - 4) = s[0];
      v8 = &v58 + 38 * i - 2;
      v9 = v57;
      *v8 = v56;
      v8[1] = v9;
      v10 = v59;
      v8[2] = v58;
      v8[3] = v10;
      v11 = v60;
      v8[4] = *((long long *)&v59 + 1);
      v8[5] = v11;
      v12 = v62;
      v8[6] = v61;
      v8[7] = v12;
      v13 = v63[1];
      v8[8] = v63[0];
      v8[9] = v13;
      v14 = v64;
      v8[10] = v63[2];
      v8[11] = v14;
      v15 = v66;
      v8[12] = v65;
      v8[13] = v15;
      v16 = v68;
      v8[14] = v67;
      v8[15] = v16;
      v17 = v70;
      v8[16] = v69;
      v8[17] = v17;
      continue;
    }
    if ( !strcmp((const char *)*(&v57 + 38 * i - 2), "-") )
    {
      *((int *)&v56 + 76 * i - 4) = 0;
      if ( !isatty(0) )
        sub_131E6(0, 0);
      if ( fstat(0, (struct stat *)&s[76 * i + 4]) )
      {
LABEL_43:
        *((int *)&v56 + 76 * i - 4) = -3 - *__errno_location();
        continue;
      }
      if ( (*(int *)(&v60 + 38 * i - 2) & 0xF000) == 0x8000 )
      {
        v51 = lseek(0, 0, 1);
        if ( v51 >= 0 )
        {
          v18 = 0;
          if ( v63[38 * i - 2] - v51 >= 0 )
            v18 = v63[38 * i - 2] - v51;
          v63[38 * i - 2] = v18;
        }
        else
        {
          *((int *)&v56 + 76 * i - 4) = -3 - *__errno_location();
        }
      }
      sub_A723((long long)&s[76 * i + 4]);
    }
    else
    {
      if ( byte_2D1F0 )
        v19 = lstat((const char *)*(&v57 + 38 * i - 2), (struct stat *)&s[76 * i + 4]) != 0;
      else
        v19 = stat((const char *)*(&v57 + 38 * i - 2), (struct stat *)&s[76 * i + 4]) != 0;
      if ( v19 )
        goto LABEL_43;
    }
  }
  for ( j = 0; j <= 1; ++j )
  {
    if ( byte_2D648 || !j && byte_2D649 )
    {
      if ( *((int *)&v56 + 76 * j - 4) == -2 )
        v21 = (*(int *)(&v60 + 38 * j - 2) & 0xF000) == 0x8000
           && (*(int *)(&v60 + 38 * j - 2) & 0x1FF) == 0
           && !v63[38 * j - 2];
      else
        v21 = (*((int *)&v56 + 76 * j - 4) == -5 || *((int *)&v56 + 76 * j - 4) == -12)
           && !a1
           && (*((int *)&v56 + 76 * (1 - j) - 4) == -2 || !*((int *)&v56 + 76 * (1 - j) - 4));
      if ( v21 )
        *((int *)&v56 + 76 * j - 4) = -1;
    }
  }
  for ( k = 0; k <= 1; ++k )
  {
    if ( *((int *)&v56 + 76 * k - 4) == -1 )
    {
      memset(&s[76 * k + 4], 0, 0x90u);
      *((int *)&v60 + 76 * k - 4) = *((int *)&v60 + 76 * (1 - k) - 4);
    }
  }
  for ( m = 0; m <= 1; ++m )
  {
    v45 = -3 - *((int *)&v56 + 76 * m - 4);
    if ( v45 >= 0 )
    {
      *__errno_location() = v45;
      sub_108EC(*(&v57 + 38 * m - 2));
      v43 = 2;
    }
  }
  if ( !v43 && !a1 && ((v59 & 0xF000) == 0x4000) != ((v74 & 0xF000) == 0x4000) )
  {
    v44 = (v59 & 0xF000) != 0x4000;
    s1 = (char *)*(&v57 + 38 * ((v59 & 0xF000) == 0x4000) - 2);
    v49 = *(&v57 + 38 * ((v59 & 0xF000) != 0x4000) - 2);
    v24 = sub_13111(s1);
    ptr = (char *)sub_C9BD(v49, v24);
    *(&v57 + 38 * v44 - 2) = (long long)ptr;
    file = (char *)*(&v57 + 38 * v44 - 2);
    if ( !strcmp(s1, "-") )
      sub_10966("cannot compare '-' to a directory");
    if ( byte_2D1F0 )
      v25 = lstat(file, (struct stat *)&s[76 * v44 + 4]) != 0;
    else
      v25 = stat(file, (struct stat *)&s[76 * v44 + 4]) != 0;
    if ( v25 )
    {
      sub_108EC(file);
      v43 = 2;
    }
  }
  if ( v43 || s[0] == -1 && fd == -1 )
    goto LABEL_197;
  v27 = 0;
  if ( s[0] != -1 && fd != -1 )
  {
    v26 = v57 == v73[1] && v56 == v73[0]
       || ((v59 & 0xF000) == 0x6000 && (v74 & 0xF000) == 0x6000 || (v59 & 0xF000) == 0x2000 && (v74 & 0xF000) == 0x2000)
       && v60 == v76;
    if ( v26 > 0
      && (int)v59 == v74
      && v58 == v73[2]
      && *(long long *)((char *)&v59 + 4) == v75
      && v61 == v77
      && v64 == v78
      && v66 == v79 )
    {
      v27 = 1;
    }
  }
  v42 = v27;
  if ( v27 )
  {
    if ( byte_2D1C8 )
      goto LABEL_197;
  }
  if ( (v59 & 0xF000) == 0x4000 && (v74 & 0xF000) == 0x4000 )
  {
    if ( dword_2D1C0 == 7 )
      sub_10966("-D option not supported with directories");
    if ( a1 && byte_2D5C9 != 1 )
    {
      sub_109A8("Common subdirectories: %s and %s\n", s2, v72);
      goto LABEL_197;
    }
LABEL_132:
    v43 = sub_C21B(s, compare_files);
    goto LABEL_197;
  }
  if ( (v59 & 0xF000) != 0x4000
    && (v74 & 0xF000) != 0x4000
    && (!a1
     || ((v59 & 0xF000) == 0x8000 || (v59 & 0xF000) == 0xA000) && ((v74 & 0xF000) == 0x8000 || (v74 & 0xF000) == 0xA000)) )
  {
    if ( (v59 & 0xF000) != 0xA000 && (v74 & 0xF000) != 0xA000 )
    {
      if ( byte_2D1ED && (v59 & 0xF000) == 0x8000 && (v74 & 0xF000) == 0x8000 && v61 != v77 && v61 > 0 && v77 > 0 )
      {
        if ( qword_2D208 )
          v34 = (char *)qword_2D208;
        else
          v34 = v72;
        if ( qword_2D200[0] )
          v35 = (char *)qword_2D200[0];
        else
          v35 = s2;
        sub_109A8("Files %s and %s differ\n", v35, v34);
        v43 = 1;
      }
      else
      {
        if ( s[0] == -2 )
        {
          s[0] = open(s2, 0, 0);
          if ( s[0] < 0 )
          {
            sub_108EC(s2);
            v43 = 2;
          }
        }
        if ( fd == -2 )
        {
          if ( v42 )
          {
            fd = s[0];
          }
          else
          {
            fd = open(v72, 0, 0);
            if ( fd < 0 )
            {
              sub_108EC(v72);
              v43 = 2;
            }
          }
        }
        if ( !v43 )
          v43 = sub_7206((long long)s);
        if ( s[0] >= 0 && close(s[0]) )
        {
          sub_108EC(s2);
          v43 = 2;
        }
        if ( fd >= 0 && s[0] != fd && close(fd) )
        {
          sub_108EC(v72);
          v43 = 2;
        }
      }
      goto LABEL_197;
    }
    if ( !byte_2D1F0 )
      __assert_fail("no_dereference_symlinks", "diff.c", 0x549u, "compare_files");
    if ( (v59 & 0xF000) == 0xA000 && (v74 & 0xF000) == 0xA000 )
    {
      v52 = 0;
      v53 = 0;
      for ( n = 0; n <= 1; ++n )
      {
        (&v52)[n] = (char *)sub_1CF7B(*(&v57 + 38 * n - 2));
        if ( !(&v52)[n] )
        {
          sub_108EC(*(&v57 + 38 * n - 2));
          v43 = 2;
          break;
        }
      }
      if ( !v43 && strcmp(v52, v53) )
      {
        sub_109A8("Symbolic links %s and %s differ\n", s2, v72);
        v43 = 1;
      }
      for ( ii = 0; ii <= 1; ++ii )
        free((&v52)[ii]);
      goto LABEL_197;
    }
    v28 = (const char *)sub_14997(v73);
    if ( qword_2D208 )
      v29 = (char *)qword_2D208;
    else
      v29 = v72;
    v30 = (const char *)sub_14997(&v56);
    if ( qword_2D200[0] )
      v31 = (char *)qword_2D200[0];
    else
      v31 = s2;
LABEL_142:
    sub_109E2("File %s is a %s while file %s is a %s\n", v31, v30, v29, v28);
    v43 = 1;
    goto LABEL_197;
  }
  if ( s[0] != -1 && fd != -1 )
  {
    v28 = (const char *)sub_14997(v73);
    if ( qword_2D208 )
      v29 = (char *)qword_2D208;
    else
      v29 = v72;
    v30 = (const char *)sub_14997(&v56);
    if ( qword_2D200[0] )
      v31 = (char *)qword_2D200[0];
    else
      v31 = s2;
    goto LABEL_142;
  }
  if ( ((v59 & 0xF000) == 0x4000 || (v74 & 0xF000) == 0x4000) && byte_2D5C9 && (byte_2D648 || byte_2D649 && s[0] == -1) )
    goto LABEL_132;
  if ( !a1 )
    __assert_fail("parent", "diff.c", 0x52Du, "compare_files");
  sub_109A8("Only in %s: %s\n", *(const char **)(a1 + 304LL * (s[0] == -1) + 8), v40);
  v43 = 1;
LABEL_197:
  if ( v43 )
  {
    if ( fflush_unlocked(stdout) )
    {
      v38 = gettext("standard output");
      sub_10925(v38);
    }
  }
  else if ( byte_2D64A && (v59 & 0xF000) != 0x4000 )
  {
    if ( qword_2D208 )
      v36 = (char *)qword_2D208;
    else
      v36 = v72;
    if ( qword_2D200[0] )
      v37 = (char *)qword_2D200[0];
    else
      v37 = s2;
    sub_109A8("Files %s and %s are identical\n", v37, v36);
  }
  free(ptr);
  free(v47);
  return v43;
}



// Function: dir_read @ 0xbde2
long long dir_read(long long a1, unsigned long long *a2)
{
  DIR *v2; // rbx
  struct dirent *v4; // rax
  unsigned long long i; // rbx
  int v6; // [rsp+14h] [rbp-4Ch]
  unsigned long long v7; // [rsp+18h] [rbp-48h]
  char *s; // [rsp+20h] [rbp-40h]
  unsigned long long v9; // [rsp+28h] [rbp-38h]
  long long v10; // [rsp+30h] [rbp-30h]
  char *src; // [rsp+38h] [rbp-28h]
  size_t n; // [rsp+40h] [rbp-20h]
  long long v13; // [rsp+48h] [rbp-18h]

  a2[1] = 0;
  a2[2] = 0;
  v7 = 0;
  s = 0;
  if ( *(int *)a1 == -1 )
    goto LABEL_21;
  v2 = opendir(*(const char **)(a1 + 8));
  if ( !v2 )
    return 0;
  v9 = 512;
  v10 = 0;
  s = (char *)sub_1C945(512);
  a2[2] = (unsigned long long)s;
  while ( 1 )
  {
    *__errno_location() = 0;
    v4 = readdir(v2);
    if ( !v4 )
      break;
    src = v4->d_name;
    n = strlen(v4->d_name) + 1;
    if ( (*src != 46 || src[1] && (src[1] != 46 || src[2])) && !(unsigned char)sub_141F6(qword_2D340, src) )
    {
      while ( v9 < v10 + n )
      {
        if ( v9 > 0x3FFFFFFFFFFFFFFELL )
          sub_1CF09();
        v9 *= 2LL;
        s = (char *)sub_1C991(s, v9);
        a2[2] = (unsigned long long)s;
      }
      memcpy(&s[v10], src, n);
      v10 += n;
      ++v7;
    }
  }
  if ( *__errno_location() )
  {
    v6 = *__errno_location();
    closedir(v2);
    *__errno_location() = v6;
    return 0;
  }
  if ( closedir(v2) )
    return 0;
LABEL_21:
  if ( v7 > 0xFFFFFFFFFFFFFFDLL )
    sub_1CF09();
  v13 = sub_1C945(8 * (v7 + 1));
  a2[1] = v13;
  *a2 = v7;
  for ( i = 0; i < v7; ++i )
  {
    *(long long *)(v13 + 8 * i) = s;
    s += strlen(s) + 1;
  }
  *(long long *)(8 * v7 + v13) = 0;
  return 1;
}



// Function: compare_collated @ 0xc084
long long compare_collated(const char *a1, const char *a2)
{
  char *v2; // rbx
  int *v3; // rax
  unsigned int v5; // [rsp+1Ch] [rbp-14h]

  *__errno_location() = 0;
  if ( byte_2D1EF )
    v5 = strcasecmp(a1, a2);
  else
    v5 = strcoll(a1, a2);
  if ( *__errno_location() )
  {
    v2 = gettext("cannot compare file names '%s' and '%s'");
    v3 = __errno_location();
    error(0, *v3, v2, a1, a2);
    longjmp(env, 1);
  }
  return v5;
}



// Function: compare_names @ 0xc140
int compare_names(const char *a1, const char *a2)
{
  int v3; // [rsp+1Ch] [rbp-4h]

  if ( byte_2D660 && ((v3 = sub_C084(a1, a2)) != 0 || byte_2D1EF) )
    return v3;
  else
    return strcmp(a1, a2);
}



// Function: compare_names_for_qsort @ 0xc1a0
int compare_names_for_qsort(char **a1, char **a2)
{
  int v3; // [rsp+1Ch] [rbp-24h]
  char *s1; // [rsp+30h] [rbp-10h]
  char *s2; // [rsp+38h] [rbp-8h]

  s1 = *a1;
  s2 = *a2;
  if ( byte_2D660 && (v3 = sub_C084(s1, s2)) != 0 )
    return v3;
  else
    return strcmp(s1, s2);
}



// Function: diff_dirs @ 0xc21b
long long diff_dirs(long long a1, long long (*a2)(long long, long long, const char *))
{
  long long v2; // rbx
  char *v3; // rax
  int v5; // eax
  const char **v6; // rax
  const char *v7; // rdx
  long long *v8; // rax
  long long v9; // rax
  int v10; // [rsp+18h] [rbp-98h]
  int i; // [rsp+1Ch] [rbp-94h]
  int j; // [rsp+1Ch] [rbp-94h]
  int m; // [rsp+1Ch] [rbp-94h]
  int v14; // [rsp+20h] [rbp-90h]
  int v15; // [rsp+24h] [rbp-8Ch]
  int v16; // [rsp+30h] [rbp-80h]
  int v17; // [rsp+34h] [rbp-7Ch]
  const char **k; // [rsp+38h] [rbp-78h]
  const char **src; // [rsp+40h] [rbp-70h]
  char *s2; // [rsp+48h] [rbp-68h]
  void *base; // [rsp+50h] [rbp-60h]
  const char **v22; // [rsp+58h] [rbp-58h]
  char v23[8]; // [rsp+60h] [rbp-50h] BYREF
  void *v24; // [rsp+68h] [rbp-48h]
  char v25[16]; // [rsp+70h] [rbp-40h]
  long long v26[5]; // [rsp+80h] [rbp-30h]

  v26[3] = __readfsqword(0x28u);
  v10 = 0;
  if ( (*(int *)a1 == -1 || (unsigned char)sub_C796(a1, 0))
    && (*(int *)(a1 + 304) == -1 || (unsigned char)sub_C796(a1, 1)) )
  {
    v2 = *(long long *)(a1 + 304LL * (*(int *)a1 == -1) + 8);
    v3 = gettext("%s: recursive directory loop");
    error(0, 0, v3, v2);
    return 2;
  }
  else
  {
    for ( i = 0; i <= 1; ++i )
    {
      if ( (unsigned char)sub_BDE2(a1 + 304LL * i, (unsigned long long *)&v23[24 * i]) != 1 )
      {
        sub_108EC(*(long long *)(a1 + 304LL * i + 8));
        v10 = 2;
      }
    }
    if ( !v10 )
    {
      base = v24;
      v22 = (const char **)v26[0];
      byte_2D660 = _setjmp(env) == 0;
      for ( j = 0; j <= 1; ++j )
        qsort(*(&base + j), *(long long *)&v25[24 * j - 16], 8u, (__compar_fn_t)compar);
      if ( qword_2D2B8 && !*(long long *)(a1 + 608) )
      {
        while ( *(long long *)base && sub_C140(*(const char **)base, (const char *)qword_2D2B8) < 0 )
          base = (char *)base + 8;
        while ( *v22 && sub_C140(*v22, (const char *)qword_2D2B8) < 0 )
          ++v22;
      }
      while ( *(long long *)base || *v22 )
      {
        if ( *(long long *)base )
        {
          if ( *v22 )
            v5 = sub_C140(*(const char **)base, *v22);
          else
            v5 = -1;
        }
        else
        {
          v5 = 1;
        }
        v14 = v5;
        if ( !v5 )
        {
          if ( byte_2D1EF )
          {
            v15 = strcmp(*(const char **)base, *v22);
            if ( v15 )
            {
              src = (const char **)*(&base + (v15 >= 0));
              s2 = *(char **)*(&base + (v15 < 0));
              for ( k = src + 1; *k && !sub_C140(*k, s2); ++k )
              {
                v16 = strcmp(*k, s2);
                if ( v16 >= 0 )
                {
                  if ( !v16 )
                  {
                    memmove(src + 1, src, (char *)k - (char *)src);
                    *src = s2;
                  }
                  break;
                }
              }
            }
          }
        }
        if ( v14 < 0 )
        {
          v7 = 0;
        }
        else
        {
          v6 = v22++;
          v7 = *v6;
        }
        if ( v14 > 0 )
        {
          v9 = 0;
        }
        else
        {
          v8 = (long long *)base;
          base = (char *)base + 8;
          v9 = *v8;
        }
        v17 = a2(a1, v9, v7);
        if ( v17 > v10 )
          v10 = v17;
      }
    }
    for ( m = 0; m <= 1; ++m )
    {
      free(*(void **)&v25[24 * m - 8]);
      free((void *)v26[3 * m - 2]);
    }
    return (unsigned int)v10;
  }
}



// Function: dir_loop @ 0xc796
long long dir_loop(long long a1, int a2)
{
  int v2; // eax
  long long v4; // [rsp+14h] [rbp-8h]

  v4 = a1;
  while ( 1 )
  {
    v4 = *(long long *)(v4 + 608);
    if ( !v4 )
      break;
    v2 = *(long long *)(v4 + 304LL * a2 + 24) == *(long long *)(a1 + 304LL * a2 + 24)
      && *(long long *)(v4 + 304LL * a2 + 16) == *(long long *)(a1 + 304LL * a2 + 16)
      || ((*(int *)(v4 + 304LL * a2 + 40) & 0xF000) == 0x6000
       && (*(int *)(a1 + 304LL * a2 + 40) & 0xF000) == 0x6000
       || (*(int *)(v4 + 304LL * a2 + 40) & 0xF000) == 0x2000
       && (*(int *)(a1 + 304LL * a2 + 40) & 0xF000) == 0x2000)
      && *(long long *)(v4 + 304LL * a2 + 56) == *(long long *)(a1 + 304LL * a2 + 56);
    if ( v2 > 0 )
      return 1;
  }
  return 0;
}



// Function: find_dir_file_pathname @ 0xc9bd
long long find_dir_file_pathname(long long a1, const char *a2)
{
  long long v2; // rax
  const char *v4; // [rsp+18h] [rbp-178h]
  const char **i; // [rsp+20h] [rbp-170h]
  long long v6; // [rsp+28h] [rbp-168h]
  unsigned long long v7; // [rsp+30h] [rbp-160h] BYREF
  void *ptr; // [rsp+38h] [rbp-158h]
  void *v9; // [rsp+40h] [rbp-150h]
  int v10; // [rsp+50h] [rbp-140h] BYREF
  long long v11; // [rsp+58h] [rbp-138h]
  unsigned long long v12; // [rsp+188h] [rbp-8h]

  v12 = __readfsqword(0x28u);
  v4 = a2;
  ptr = 0;
  v9 = 0;
  if ( !byte_2D1EF || (v11 = a1, v10 = 0, !(unsigned char)sub_BDE2((long long)&v10, &v7)) )
  {
LABEL_13:
    v2 = sub_14ADE(a1, v4, 0);
    goto LABEL_14;
  }
  byte_2D660 = 1;
  if ( !_setjmp(env) )
  {
    for ( i = (const char **)ptr; *i; ++i )
    {
      if ( !sub_C140(*i, a2) )
      {
        if ( !strcmp(*i, a2) )
        {
          v2 = sub_14ADE(a1, *i, 0);
          goto LABEL_14;
        }
        if ( v4 == a2 )
          v4 = *i;
      }
    }
    goto LABEL_13;
  }
  v2 = sub_14ADE(a1, a2, 0);
LABEL_14:
  v6 = v2;
  free(ptr);
  free(v9);
  return v6;
}



// Function: print_ed_script @ 0xcb85
long long print_ed_script(long long a1)
{
  return sub_1245B(a1, sub_12449, sub_CBB5);
}



// Function: print_ed_hunk @ 0xcbb5
unsigned long long print_ed_hunk(long long a1)
{
  char v2; // [rsp+1Bh] [rbp-35h]
  int v3; // [rsp+1Ch] [rbp-34h]
  long long v4; // [rsp+20h] [rbp-30h] BYREF
  long long v5; // [rsp+28h] [rbp-28h] BYREF
  long long v6; // [rsp+30h] [rbp-20h] BYREF
  long long v7; // [rsp+38h] [rbp-18h] BYREF
  long long i; // [rsp+40h] [rbp-10h]
  unsigned long long v9; // [rsp+48h] [rbp-8h]

  v9 = __readfsqword(0x28u);
  v3 = sub_12AF4(a1, &v4, &v5, &v6, &v7);
  if ( v3 )
  {
    sub_11B50();
    sub_12A37(44, &unk_2D360, v4, v5);
    fputc_unlocked(byte_25780[v3], stream);
    fputc_unlocked(10, stream);
    if ( v3 != 1 )
    {
      v2 = 1;
      for ( i = v6; i <= v7; ++i )
      {
        if ( v2 != 1 )
        {
          fputs_unlocked("a\n", stream);
          v2 = 1;
        }
        if ( **(char **)(qword_2D548 + 8 * i) == 46 && *(char *)(*(long long *)(qword_2D548 + 8 * i) + 1LL) == 10 )
        {
          fputs_unlocked("..\n.\ns/.//\n", stream);
          v2 = 0;
        }
        else
        {
          sub_124CF(&unk_25164, qword_2D548 + 8 * i);
        }
      }
      if ( v2 )
        fputs_unlocked(".\n", stream);
    }
  }
  return v9 - __readfsqword(0x28u);
}



// Function: pr_forward_ed_script @ 0xcd6a
long long pr_forward_ed_script(long long a1)
{
  return sub_1245B(a1, sub_12437, sub_CD9A);
}



// Function: pr_forward_ed_hunk @ 0xcd9a
unsigned long long pr_forward_ed_hunk(long long a1)
{
  int v2; // [rsp+1Ch] [rbp-34h]
  long long v3; // [rsp+20h] [rbp-30h] BYREF
  long long v4; // [rsp+28h] [rbp-28h] BYREF
  long long v5; // [rsp+30h] [rbp-20h] BYREF
  long long v6; // [rsp+38h] [rbp-18h] BYREF
  long long i; // [rsp+40h] [rbp-10h]
  unsigned long long v8; // [rsp+48h] [rbp-8h]

  v8 = __readfsqword(0x28u);
  v2 = sub_12AF4(a1, &v3, &v4, &v5, &v6);
  if ( v2 )
  {
    sub_11B50();
    fputc_unlocked(byte_25780[v2], stream);
    sub_12A37(32, &unk_2D360, v3, v4);
    fputc_unlocked(10, stream);
    if ( v2 != 1 )
    {
      for ( i = v5; i <= v6; ++i )
        sub_124CF(&unk_25164, qword_2D548 + 8 * i);
      fputs_unlocked(".\n", stream);
    }
  }
  return v8 - __readfsqword(0x28u);
}



// Function: print_rcs_script @ 0xceba
long long print_rcs_script(long long a1)
{
  return sub_1245B(a1, sub_12437, sub_CEEA);
}



// Function: print_rcs_hunk @ 0xceea
unsigned long long print_rcs_hunk(long long a1)
{
  long long v1; // rcx
  long long v2; // rcx
  int v4; // [rsp+1Ch] [rbp-54h]
  long long v5; // [rsp+20h] [rbp-50h] BYREF
  long long v6; // [rsp+28h] [rbp-48h] BYREF
  long long v7; // [rsp+30h] [rbp-40h] BYREF
  long long v8; // [rsp+38h] [rbp-38h] BYREF
  long long v9; // [rsp+40h] [rbp-30h] BYREF
  long long v10; // [rsp+48h] [rbp-28h] BYREF
  long long v11; // [rsp+50h] [rbp-20h] BYREF
  long long v12; // [rsp+58h] [rbp-18h] BYREF
  long long i; // [rsp+60h] [rbp-10h]
  unsigned long long v14; // [rsp+68h] [rbp-8h]

  v14 = __readfsqword(0x28u);
  v4 = sub_12AF4(a1, &v5, &v6, &v7, &v8);
  if ( v4 )
  {
    sub_11B50();
    sub_129D0(&unk_2D360, v5, v6, &v9, &v10);
    if ( (v4 & 1) != 0 )
    {
      if ( v9 > v10 )
        v1 = 1;
      else
        v1 = v10 - v9 + 1;
      fprintf(stream, "d%ld %ld\n", v9, v1);
    }
    if ( (v4 & 2) != 0 )
    {
      sub_129D0(&unk_2D490, v7, v8, &v11, &v12);
      if ( v11 > v12 )
        v2 = 1;
      else
        v2 = v12 - v11 + 1;
      fprintf(stream, "a%ld %ld\n", v10, v2);
      for ( i = v7; i <= v8; ++i )
        sub_124CF(&unk_25164, qword_2D548 + 8 * i);
    }
  }
  return v14 - __readfsqword(0x28u);
}



// Function: print_ifdef_script @ 0xd076
long long print_ifdef_script(long long a1)
{
  long long result; // rax

  qword_2D750 = -qword_2D448;
  qword_2D748 = -qword_2D448;
  sub_1245B(a1, sub_12437, sub_D11E);
  if ( qword_2D430 > qword_2D748 || (result = qword_2D750, qword_2D560 > qword_2D750) )
  {
    sub_11B50();
    return sub_D221(s1, qword_2D748, qword_2D430, qword_2D750, qword_2D560);
  }
  return result;
}



// Function: print_ifdef_hunk @ 0xd11e
unsigned long long print_ifdef_hunk(long long a1)
{
  int v2; // [rsp+14h] [rbp-2Ch]
  long long v3; // [rsp+18h] [rbp-28h] BYREF
  long long v4; // [rsp+20h] [rbp-20h] BYREF
  long long v5; // [rsp+28h] [rbp-18h] BYREF
  long long v6; // [rsp+30h] [rbp-10h] BYREF
  unsigned long long v7; // [rsp+38h] [rbp-8h]

  v7 = __readfsqword(0x28u);
  v2 = sub_12AF4(a1, &v3, &v4, &v5, &v6);
  if ( v2 )
  {
    sub_11B50();
    if ( qword_2D748 < v3 || qword_2D750 < v5 )
      sub_D221(s1, qword_2D748, v3, qword_2D750, v5);
    qword_2D748 = v4 + 1;
    qword_2D750 = v6 + 1;
    sub_D221((&s1)[v2], v3, v4 + 1, v5, v6 + 1);
  }
  return v7 - __readfsqword(0x28u);
}



// Function: format_ifdef @ 0xd221
unsigned long long format_ifdef(long long a1, long long a2, long long a3, long long a4, long long a5)
{
  long long v6[7]; // [rsp+30h] [rbp-40h] BYREF
  unsigned long long v7; // [rsp+68h] [rbp-8h]

  v7 = __readfsqword(0x28u);
  v6[0] = &unk_2D360;
  v6[1] = a2;
  v6[2] = a3;
  v6[3] = &unk_2D490;
  v6[4] = a4;
  v6[5] = a5;
  sub_D2BC(stream, a1, 0, v6);
  return v7 - __readfsqword(0x28u);
}



// Function: format_group @ 0xd2bc
const char *format_group(FILE *a1, const char *a2, char a3, long long a4)
{
  char *v5; // rax
  char v6; // r13
  char *v7; // rbx
  char *v8; // rax
  int v12; // [rsp+2Ch] [rbp-64h]
  char *endptr; // [rsp+30h] [rbp-60h] BYREF
  FILE *stream; // [rsp+38h] [rbp-58h]
  FILE *v15; // [rsp+40h] [rbp-50h]
  const char *v16; // [rsp+48h] [rbp-48h]
  long long v17[8]; // [rsp+50h] [rbp-40h]

  v17[3] = __readfsqword(0x28u);
  while ( 1 )
  {
    v6 = *a2;
    if ( *a2 == a3 || !v6 )
      return a2;
    v16 = ++a2;
    if ( v6 == 37 )
    {
      v5 = (char *)a2++;
      v6 = *v5;
      switch ( *v5 )
      {
        case '%':
          goto LABEL_22;
        case '(':
          v12 = 0;
          break;
        case '<':
        case '=':
        case '>':
          sub_D6BD(a1);
          continue;
        default:
          a2 = (const char *)sub_DA03(a1, a2 - 2, 0, 0, a4);
          if ( !a2 )
            goto LABEL_21;
          continue;
      }
      while ( v12 <= 1 )
      {
        if ( (unsigned int)(*a2 - 48) > 9 )
        {
          v17[v12] = sub_D577(a4, (unsigned int)*a2);
          if ( (long long)v17[v12] < 0 )
            goto LABEL_21;
          v7 = (char *)(a2 + 1);
        }
        else
        {
          *__errno_location() = 0;
          v17[v12] = strtoimax(a2, &endptr, 10);
          if ( *__errno_location() )
            goto LABEL_21;
          v7 = endptr;
        }
        v8 = v7;
        a2 = v7 + 1;
        if ( *v8 != asc_2517C[v12] )
        {
LABEL_21:
          v6 = 37;
          a2 = v16;
          goto LABEL_22;
        }
        ++v12;
      }
      if ( v17[0] == v17[1] )
      {
        stream = a1;
        v15 = 0;
      }
      else
      {
        stream = 0;
        v15 = a1;
      }
      a2 = (const char *)format_group(stream, a2, 58, a4);
      if ( *a2 )
      {
        a2 = (const char *)format_group(v15, a2 + 1, 41, a4);
        if ( *a2 )
          ++a2;
      }
    }
    else
    {
LABEL_22:
      if ( a1 )
        putc_unlocked(v6, a1);
    }
  }
}



// Function: groups_letter_value @ 0xd577
long long groups_letter_value(long long *a1, char a2)
{
  long long result; // rax
  char v3; // [rsp+4h] [rbp-Ch]
  long long *v4; // [rsp+8h] [rbp-8h]

  v4 = a1;
  v3 = a2;
  switch ( a2 )
  {
    case 'E':
      v3 = 101;
      v4 = a1 + 3;
      break;
    case 'F':
      v3 = 102;
      v4 = a1 + 3;
      break;
    case 'L':
      v3 = 108;
      v4 = a1 + 3;
      break;
    case 'M':
      v3 = 109;
      v4 = a1 + 3;
      break;
    case 'N':
      v3 = 110;
      v4 = a1 + 3;
      break;
    default:
      break;
  }
  switch ( v3 )
  {
    case 'e':
      result = sub_129A8(*v4, v4[1]) - 1;
      break;
    case 'f':
      result = sub_129A8(*v4, v4[1]);
      break;
    case 'l':
      result = sub_129A8(*v4, v4[2]) - 1;
      break;
    case 'm':
      result = sub_129A8(*v4, v4[2]);
      break;
    case 'n':
      result = v4[2] - v4[1];
      break;
    default:
      result = -1;
      break;
  }
  return result;
}



// Function: print_ifdef_lines @ 0xd6bd
long long print_ifdef_lines(FILE *stream, char *a2, long long *a3)
{
  long long result; // rax
  long long v4; // rbx
  char *v5; // rax
  char v6; // r12
  char *v7; // rax
  long long v8; // [rsp+18h] [rbp-48h]
  long long v9; // [rsp+20h] [rbp-40h]
  long long v10; // [rsp+28h] [rbp-38h]
  long long v11; // [rsp+30h] [rbp-30h]
  long long v12; // [rsp+38h] [rbp-28h]

  v9 = *a3;
  v10 = *(long long *)(*a3 + 184LL);
  v8 = a3[1];
  result = a3[2];
  v11 = result;
  if ( stream )
  {
    if ( byte_2D2A1 != 1 && *a2 == 37 )
    {
      if ( a2[1] == 108 && a2[2] == 10 && !a2[3] && v8 < result )
        return fwrite_unlocked(
                 *(const void **)(8 * v8 + v10),
                 1u,
                 (*(char *)(*(long long *)(8 * result + v10) - 1LL) != 10)
               + *(long long *)(8 * result + v10)
               - *(long long *)(8 * v8 + v10),
                 stream);
      if ( a2[1] == 76 && !a2[2] )
        return fwrite_unlocked(
                 *(const void **)(8 * v8 + v10),
                 1u,
                 *(long long *)(8 * result + v10) - *(long long *)(8 * v8 + v10),
                 stream);
    }
LABEL_26:
    result = v8;
    if ( v8 < v11 )
    {
      v4 = (long long)a2;
      while ( 1 )
      {
        while ( 1 )
        {
          v7 = (char *)v4++;
          v6 = *v7;
          if ( !*v7 )
          {
            ++v8;
            goto LABEL_26;
          }
          v12 = v4;
          if ( v6 == 37 )
            break;
LABEL_23:
          putc_unlocked(v6, stream);
        }
        v5 = (char *)v4++;
        v6 = *v5;
        if ( *v5 == 108 )
        {
          sub_1267D(
            *(long long *)(8 * v8 + v10),
            *(long long *)(8 * (v8 + 1) + v10) - (*(char *)(*(long long *)(8 * (v8 + 1) + v10) - 1LL) == 10),
            0,
            0);
        }
        else
        {
          if ( v6 > 108 )
            goto LABEL_21;
          if ( v6 == 37 )
            goto LABEL_23;
          if ( v6 == 76 )
          {
            sub_1267D(*(long long *)(8 * v8 + v10), *(long long *)(8 * (v8 + 1) + v10), 0, 0);
          }
          else
          {
LABEL_21:
            v4 = sub_DA03(stream, v4 - 2, v9, v8, 0);
            if ( !v4 )
            {
              v6 = 37;
              v4 = v12;
              goto LABEL_23;
            }
          }
        }
      }
    }
  }
  return result;
}



// Function: do_printf_spec @ 0xda03
long long do_printf_spec(FILE *a1, void *a2, long long a3, long long a4, long long *a5)
{
  char *v5; // rax
  char *v6; // rax
  char *v7; // rax
  char *v8; // rax
  unsigned int v9; // eax
  long long v10; // rax
  size_t v12; // rax
  void *v13; // rsp
  char *v14; // rax
  char *v15; // rax
  char v16[8]; // [rsp+8h] [rbp-80h] BYREF
  long long *v17; // [rsp+10h] [rbp-78h]
  long long v18; // [rsp+18h] [rbp-70h]
  long long v19; // [rsp+20h] [rbp-68h] BYREF
  void *src; // [rsp+28h] [rbp-60h]
  FILE *stream; // [rsp+30h] [rbp-58h]
  char v22; // [rsp+45h] [rbp-43h] BYREF
  char v23; // [rsp+46h] [rbp-42h]
  char v24; // [rsp+47h] [rbp-41h]
  long long v25; // [rsp+48h] [rbp-40h]
  long long v26; // [rsp+50h] [rbp-38h]
  long long v27; // [rsp+58h] [rbp-30h]
  size_t n; // [rsp+60h] [rbp-28h]
  size_t v29; // [rsp+68h] [rbp-20h]
  void *dest; // [rsp+70h] [rbp-18h]
  char *v31; // [rsp+78h] [rbp-10h]
  unsigned long long v32; // [rsp+80h] [rbp-8h]

  stream = a1;
  src = a2;
  v19 = a3;
  v18 = a4;
  v17 = a5;
  v32 = __readfsqword(0x28u);
  v25 = (long long)a2 + 1;
  do
  {
    do
    {
      v5 = (char *)v25++;
      v23 = *v5;
    }
    while ( v23 == 45 );
  }
  while ( v23 == 39 || v23 == 48 );
  while ( (unsigned int)(v23 - 48) <= 9 )
  {
    v6 = (char *)v25++;
    v23 = *v6;
  }
  if ( v23 == 46 )
  {
    do
    {
      v7 = (char *)v25++;
      v23 = *v7;
    }
    while ( (unsigned int)(v23 - 48) <= 9 );
  }
  v8 = (char *)v25++;
  v24 = *v8;
  v9 = v23 - 88;
  if ( v9 > 0x20 )
    return 0;
  v10 = 1LL << v9;
  if ( (v10 & 0x100801001LL) != 0 )
  {
    if ( v19 )
    {
      if ( v24 != 110 )
        return 0;
      v26 = sub_129A8(v19, v18);
    }
    else
    {
      v26 = sub_D577(v17, v24);
      if ( v26 < 0 )
        return 0;
    }
    if ( stream )
    {
      v27 = v26;
      n = v25 - (long long)src - 2;
      v29 = 1;
      if ( n + 3 > 0xFA0 )
      {
        v14 = (char *)sub_1CF49(n + v29 + 2);
      }
      else
      {
        v12 = 16 * ((n + v29 + 56) / 0x10);
        while ( v16 != &v16[-(v12 & 0xFFFFFFFFFFFFF000LL)] )
          ;
        v13 = alloca(v12 & 0xFFF);
        if ( (v12 & 0xFFF) != 0 )
          *(long long *)&v16[(v12 & 0xFFF) - 8] = *(long long *)&v16[(v12 & 0xFFF) - 8];
        v14 = (char *)(((unsigned long long)&v19 + 7) & 0xFFFFFFFFFFFFFFE0LL);
      }
      dest = v14;
      v31 = &v14[v29 + n];
      memcpy(v14, src, n);
      memcpy((char *)dest + n, &unk_25238, v29);
      v15 = v31++;
      *v15 = v23;
      *v31 = 0;
      fprintf(stream, (const char *)dest, v27);
      sub_168F3(dest);
    }
    return v25;
  }
  if ( (v10 & 0x800) == 0 )
    return 0;
  if ( v24 != 39 )
    return 0;
  v25 = sub_DD74(v25, &v22);
  if ( !v25 )
    return 0;
  if ( stream )
    putc_unlocked(v22, stream);
  return v25;
}



// Function: scan_char_literal @ 0xdd74
char *scan_char_literal(char *a1, char *a2)
{
  char *v2; // rbx
  int v3; // eax
  char *v5; // rax
  char i; // [rsp+12h] [rbp-16h]
  unsigned int v7; // [rsp+14h] [rbp-14h]

  v2 = a1 + 1;
  v3 = *a1;
  if ( v3 == 92 )
  {
    for ( i = 0; ; i = 8 * i + v7 )
    {
      v5 = v2++;
      if ( *v5 == 39 )
        break;
      v7 = *v5 - 48;
      if ( v7 > 7 )
        return 0;
    }
    if ( v2 - a1 - 2 <= 0 || v2 - a1 - 2 > 3 )
      return 0;
  }
  else
  {
    if ( v3 <= 92 && (!*a1 || v3 == 39) )
      return 0;
    i = *a1;
    v2 = a1 + 2;
    if ( a1[1] != 39 )
      return 0;
  }
  *a2 = i;
  return v2;
}



// Function: file_block_read @ 0xde51
void file_block_read(unsigned int *a1, unsigned long long a2)
{
  unsigned long long v2; // [rsp+18h] [rbp-8h]

  if ( a2 && *((char *)a1 + 289) != 1 )
  {
    v2 = sub_1D1D3(*a1, *((long long *)a1 + 20) + *((long long *)a1 + 22), a2);
    if ( v2 == -1 )
      sub_10925(*((long long *)a1 + 1));
    *((long long *)a1 + 22) += v2;
    *((char *)a1 + 289) = v2 < a2;
  }
}



// Function: sip @ 0xdf00
bool sip(unsigned int *a1, char a2)
{
  unsigned int v3; // [rsp+14h] [rbp-Ch]
  size_t n; // [rsp+18h] [rbp-8h]

  if ( (*a1 & 0x80000000) != 0 )
  {
    *((long long *)a1 + 21) = 8;
    *((long long *)a1 + 20) = sub_1C945(*((long long *)a1 + 21));
LABEL_9:
    *((long long *)a1 + 22) = 0;
    *((char *)a1 + 289) = 0;
    return 0;
  }
  *((long long *)a1 + 21) = sub_1D2A2(8, *((long long *)a1 + 9), 0x7FFFFFFFFFFFFFEFLL);
  *((long long *)a1 + 20) = sub_1C945(*((long long *)a1 + 21));
  if ( a2 == 1 )
    goto LABEL_9;
  v3 = sub_131E6(*a1, 0);
  sub_DE51(a1, *((long long *)a1 + 21));
  n = *((long long *)a1 + 22);
  if ( v3 )
  {
    if ( lseek(*a1, -(long long)n, 1) < 0 )
      sub_10925(*((long long *)a1 + 1));
    sub_131E6(*a1, v3);
    *((long long *)a1 + 22) = 0;
    *((char *)a1 + 289) = 0;
  }
  return memchr(*((const void **)a1 + 20), 0, n) != 0;
}



// Function: slurp @ 0xe09d
unsigned long long slurp(unsigned int *a1)
{
  unsigned long long result; // rax
  unsigned long long v2; // [rsp+10h] [rbp-10h]
  unsigned long long v3; // [rsp+18h] [rbp-8h]

  result = *a1;
  if ( (result & 0x80000000) == 0LL )
  {
    if ( (a1[10] & 0xF000) != 0x8000 )
      goto LABEL_10;
    v2 = *((long long *)a1 + 8);
    v3 = (v2 & 0xFFFFFFFFFFFFFFF8LL) + 16;
    if ( v3 < v2 || v3 > 0x7FFFFFFFFFFFFFFELL )
      sub_1CF09();
    if ( v3 > *((long long *)a1 + 21) )
    {
      *((long long *)a1 + 21) = v3;
      *((long long *)a1 + 20) = sub_1C991(*((long long *)a1 + 20), v3);
    }
    if ( v2 < *((long long *)a1 + 22)
      || (sub_DE51(a1, v2 - *((long long *)a1 + 22) + 1), result = *((long long *)a1 + 22), v2 < result) )
    {
LABEL_10:
      sub_DE51(a1, *((long long *)a1 + 21) - *((long long *)a1 + 22));
      result = *((long long *)a1 + 22);
      if ( result )
      {
        while ( *((long long *)a1 + 22) == *((long long *)a1 + 21) )
        {
          if ( *((long long *)a1 + 21) > 0x3FFFFFFFFFFFFFF7uLL )
            sub_1CF09();
          *((long long *)a1 + 21) *= 2LL;
          *((long long *)a1 + 20) = sub_1C991(*((long long *)a1 + 20), *((long long *)a1 + 21));
          sub_DE51(a1, *((long long *)a1 + 21) - *((long long *)a1 + 22));
        }
        *((long long *)a1 + 21) = (*((long long *)a1 + 22) + 16LL) & 0xFFFFFFFFFFFFFFF8LL;
        result = sub_1C991(*((long long *)a1 + 20), *((long long *)a1 + 21));
        *((long long *)a1 + 20) = result;
      }
    }
  }
  return result;
}



// Function: find_and_hash_each_line @ 0xe2f8
long long find_and_hash_each_line(long long a1)
{
  long long v1; // rax
  unsigned char *v2; // rax
  unsigned char *v3; // rax
  long long v4; // rax
  unsigned char *v5; // rax
  unsigned char *v6; // rax
  unsigned long long v7; // rax
  unsigned char *v8; // rax
  unsigned char *v9; // rax
  unsigned char *v10; // rax
  long long v11; // rax
  unsigned char *v12; // rax
  unsigned char v14; // [rsp+17h] [rbp-A9h]
  unsigned char v15; // [rsp+17h] [rbp-A9h]
  unsigned char v16; // [rsp+17h] [rbp-A9h]
  char v17; // [rsp+18h] [rbp-A8h]
  bool v18; // [rsp+19h] [rbp-A7h]
  bool v19; // [rsp+1Ah] [rbp-A6h]
  unsigned char v20; // [rsp+1Bh] [rbp-A5h]
  unsigned int v21; // [rsp+1Ch] [rbp-A4h]
  unsigned char *v22; // [rsp+20h] [rbp-A0h]
  long long i; // [rsp+28h] [rbp-98h]
  long long j; // [rsp+28h] [rbp-98h]
  long long *v25; // [rsp+30h] [rbp-90h]
  long long v26; // [rsp+38h] [rbp-88h]
  long long v27; // [rsp+40h] [rbp-80h]
  long long v28; // [rsp+48h] [rbp-78h]
  long long v29; // [rsp+50h] [rbp-70h]
  long long *v30; // [rsp+58h] [rbp-68h]
  long long v31; // [rsp+60h] [rbp-60h]
  unsigned long long v32; // [rsp+68h] [rbp-58h]
  unsigned long long v33; // [rsp+70h] [rbp-50h]
  unsigned long long v34; // [rsp+78h] [rbp-48h]
  unsigned char *v35; // [rsp+80h] [rbp-40h]
  unsigned long long v36; // [rsp+88h] [rbp-38h]
  long long v37; // [rsp+90h] [rbp-30h]
  unsigned long long v38; // [rsp+98h] [rbp-28h]
  unsigned char *v39; // [rsp+A0h] [rbp-20h]
  unsigned char *s2; // [rsp+A8h] [rbp-18h]
  long long n; // [rsp+B0h] [rbp-10h]
  void *s1; // [rsp+B8h] [rbp-8h]

  v22 = *(unsigned char **)(a1 + 224);
  v26 = *(long long *)(a1 + 184);
  v27 = *(long long *)(a1 + 216);
  v28 = 0;
  v37 = *(long long *)(a1 + 192);
  v29 = sub_1C945(8 * v27);
  v30 = ptr;
  v31 = qword_2D770;
  v32 = qword_2D778;
  v38 = *(long long *)(a1 + 240);
  v39 = (unsigned char *)(*(long long *)(a1 + 160) + *(long long *)(a1 + 176));
  v17 = byte_2D1EE;
  v21 = dword_2D1E8;
  v18 = dword_2D1E8 != 0;
  v19 = (unsigned char)(byte_2D1EE | (dword_2D1E8 != 0)) != 0;
  while ( (unsigned long long)v22 < v38 )
  {
    s2 = v22;
    v33 = 0;
    if ( v21 == 5 )
    {
      while ( 1 )
      {
        v2 = v22++;
        v14 = *v2;
        if ( *v2 == 10 )
          break;
        if ( ((*__ctype_b_loc())[v14] & 0x2000) == 0 )
        {
          if ( v17 )
            v1 = tolower(v14);
          else
            v1 = v14;
          v33 = __ROL8__(v33, 7) + v1;
        }
      }
      goto LABEL_57;
    }
    if ( v21 <= 5 )
    {
      if ( v21 > 3 )
      {
        while ( 1 )
        {
          v5 = v22++;
          v15 = *v5;
          if ( *v5 == 10 )
            goto LABEL_57;
          if ( ((*__ctype_b_loc())[v15] & 0x2000) != 0 )
          {
            while ( 1 )
            {
              v3 = v22++;
              v15 = *v3;
              if ( *v3 == 10 )
                goto LABEL_57;
              if ( ((*__ctype_b_loc())[v15] & 0x2000) == 0 )
              {
                v33 = __ROL8__(v33, 7) + 32LL;
                break;
              }
            }
          }
          if ( v17 )
            v4 = tolower(v15);
          else
            v4 = v15;
          v33 = __ROL8__(v33, 7) + v4;
        }
      }
      if ( v21 )
      {
        v34 = 0;
        while ( 1 )
        {
          v8 = v22++;
          v16 = *v8;
          if ( *v8 == 10 )
            goto LABEL_57;
          if ( (v21 & 2) != 0 && ((*__ctype_b_loc())[v16] & 0x2000) != 0 )
          {
            v35 = v22;
            while ( 1 )
            {
              v6 = v35++;
              if ( *v6 == 10 )
                break;
              v20 = *v6;
              if ( ((*__ctype_b_loc())[v20] & 0x2000) == 0 )
                goto LABEL_32;
            }
            v22 = v35;
            goto LABEL_57;
          }
LABEL_32:
          v36 = 1;
          if ( (v21 & 1) != 0 )
          {
            if ( v16 == 13 )
            {
              v34 = 0;
            }
            else
            {
              if ( v16 > 0xDu )
                goto LABEL_44;
              if ( v16 == 8 )
              {
                v34 -= v34 != 0;
                goto LABEL_45;
              }
              if ( v16 == 9 )
              {
                v16 = 32;
                v36 = qword_2D2A8 - v34 % qword_2D2A8;
                if ( v34 > v34 + v36 )
                  v7 = 0;
                else
                  v7 = v34 + v36;
                v34 = v7;
              }
              else
              {
LABEL_44:
                ++v34;
              }
            }
          }
LABEL_45:
          if ( v17 )
            v16 = tolower(v16);
          do
          {
            v33 = v16 + __ROL8__(v33, 7);
            --v36;
          }
          while ( v36 );
        }
      }
    }
    if ( v17 )
    {
      while ( 1 )
      {
        v9 = v22++;
        if ( *v9 == 10 )
          break;
        v33 = tolower(*v9) + __ROL8__(v33, 7);
      }
    }
    else
    {
      while ( 1 )
      {
        v10 = v22++;
        if ( *v10 == 10 )
          break;
        v33 = *v10 + __ROL8__(v33, 7);
      }
    }
LABEL_57:
    v25 = (long long *)(qword_2D758 + 8 * (v33 % qword_2D760));
    n = v22 - s2 - 1;
    if ( v22 == v39 && *(char *)(a1 + 288) && dword_2D1C0 != 4 && dword_2D1C0 != 5 && v21 <= 1 )
      v25 = (long long *)(qword_2D758 - 8);
    for ( i = *v25; i; i = v30[4 * i] )
    {
      if ( v33 != v30[4 * i + 1] )
        continue;
      s1 = (void *)v30[4 * i + 2];
      if ( n == v30[4 * i + 3] )
      {
        if ( !memcmp(s1, s2, n) )
          goto LABEL_78;
        if ( !v19 )
          continue;
      }
      else if ( !v18 )
      {
        continue;
      }
      if ( (unsigned char)sub_11F90(s1, s2) != 1 )
        goto LABEL_78;
    }
    v11 = v31++;
    i = v11;
    if ( v11 == v32 )
    {
      if ( v32 > 0x1FFFFFFFFFFFFFELL )
        sub_1CF09();
      v32 *= 2LL;
      v30 = (long long *)sub_1C991(v30, 32 * v32);
    }
    v30[4 * i] = *v25;
    v30[4 * i + 1] = v33;
    v30[4 * i + 2] = s2;
    v30[4 * i + 3] = n;
    *v25 = i;
LABEL_78:
    if ( v28 == v27 )
    {
      if ( v27 > 0x2AAAAAAAAAAAAAA9LL
        || (unsigned long long)(2 * v27 - v37) > 0xFFFFFFFFFFFFFFELL
        || (unsigned long long)(v27 - v37) > 0xFFFFFFFFFFFFFFELL )
      {
        sub_1CF09();
      }
      v27 = 2 * v27 - v37;
      v29 = sub_1C991(v29, 8 * v27);
      v26 = -8 * v37 + sub_1C991(8 * v37 + v26, 8 * (v27 - v37));
    }
    *(long long *)(v26 + 8 * v28) = s2;
    *(long long *)(v29 + 8 * v28++) = i;
  }
  *(long long *)(a1 + 200) = v28;
  for ( j = 0; ; ++j )
  {
    if ( v28 == v27 )
    {
      if ( v27 > 0x2AAAAAAAAAAAAAA9LL
        || (unsigned long long)(2 * v27 - v37) > 0xFFFFFFFFFFFFFFELL
        || (unsigned long long)(v27 - v37) > 0xFFFFFFFFFFFFFFELL )
      {
        sub_1CF09();
      }
      v27 = 2 * v27 - v37;
      v26 = -8 * v37 + sub_1C991(8 * v37 + v26, 8 * (v27 - v37));
    }
    *(long long *)(v26 + 8 * v28) = v22;
    if ( v22 == v39 )
      break;
    if ( j >= qword_2D1D0 && byte_2D1C8 )
      goto LABEL_103;
    ++v28;
    do
      v12 = v22++;
    while ( *v12 != 10 );
  }
  if ( *(char *)(a1 + 288) && dword_2D1C0 != 4 && dword_2D1C0 != 5 )
    --*(long long *)(8 * v28 + v26);
LABEL_103:
  *(long long *)(a1 + 184) = v26;
  *(long long *)(a1 + 208) = v28;
  *(long long *)(a1 + 216) = v27;
  *(long long *)(a1 + 248) = v29;
  ptr = v30;
  qword_2D778 = v32;
  qword_2D770 = v31;
  return v31;
}



// Function: prepare_text @ 0xedad
char *prepare_text(long long a1)
{
  char *result; // rax
  int v2; // eax
  char *v3; // rax
  signed long long v4; // rax
  signed long long v5; // [rsp+18h] [rbp-28h]
  char *v6; // [rsp+20h] [rbp-20h]
  char *i; // [rsp+28h] [rbp-18h]
  char *v8; // [rsp+28h] [rbp-18h]
  char *s; // [rsp+30h] [rbp-10h]

  v5 = *(long long *)(a1 + 176);
  result = *(char **)(a1 + 160);
  s = result;
  if ( result )
  {
    if ( byte_2D2B2 )
    {
      result[v5] = 13;
      v6 = (char *)rawmemchr(result, 13);
      for ( i = v6; i != &s[v5]; i = v8 + 1 )
      {
        v2 = *i == 13 && i[1] == 10;
        v8 = &i[v2];
        v3 = v6++;
        *v3 = *v8;
      }
      v5 = v6 - s;
    }
    if ( v5 )
    {
      if ( s[v5 - 1] != 10 )
      {
        v4 = v5++;
        s[v4] = 10;
        *(char *)(a1 + 288) = 1;
      }
    }
    memset(&s[v5], 0, 8u);
    *(long long *)(a1 + 176) = v5;
    return (char *)a1;
  }
  return result;
}



// Function: guess_lines @ 0xeef5
unsigned long long guess_lines(long long a1, unsigned long long a2, unsigned long long a3)
{
  unsigned long long v3; // rax
  unsigned long long v4; // rax

  if ( a1 <= 9 )
    v3 = 32;
  else
    v3 = a2 / (a1 - 1);
  v4 = a3 / v3;
  if ( !v4 )
    v4 = 1;
  if ( v4 > 0x787878787878782LL )
    v4 = 0x787878787878782LL;
  return v4 + 5;
}



// Function: find_identical_ends @ 0xef6a
long long find_identical_ends(long long a1)
{
  unsigned long long v2; // rax
  int v3; // eax
  char *v4; // rax
  long long v5; // rax
  long long v6; // rax
  bool v7; // al
  long long v8; // rax
  char *v9; // rax
  long long v10; // rax
  signed long long v11; // rax
  unsigned long long v12; // rcx
  long long *v14; // [rsp+20h] [rbp-B0h]
  long long *v15; // [rsp+28h] [rbp-A8h]
  char *v16; // [rsp+30h] [rbp-A0h]
  char *v17; // [rsp+30h] [rbp-A0h]
  char *v18; // [rsp+30h] [rbp-A0h]
  char *i; // [rsp+38h] [rbp-98h]
  long long *v20; // [rsp+38h] [rbp-98h]
  char *v21; // [rsp+40h] [rbp-90h]
  long long v22; // [rsp+48h] [rbp-88h]
  long long v23; // [rsp+50h] [rbp-80h]
  long long v24; // [rsp+50h] [rbp-80h]
  long long k; // [rsp+50h] [rbp-80h]
  long long m; // [rsp+50h] [rbp-80h]
  long long n; // [rsp+50h] [rbp-80h]
  long long v28; // [rsp+58h] [rbp-78h]
  unsigned long long v29; // [rsp+60h] [rbp-70h]
  long long j; // [rsp+68h] [rbp-68h]
  char *v31; // [rsp+70h] [rbp-60h]
  long long *v32; // [rsp+78h] [rbp-58h]
  unsigned long long v33; // [rsp+80h] [rbp-50h]
  unsigned long long v34; // [rsp+88h] [rbp-48h]
  char *v35; // [rsp+90h] [rbp-40h]
  unsigned long long v36; // [rsp+98h] [rbp-38h]
  unsigned long long v37; // [rsp+98h] [rbp-38h]
  signed long long v38; // [rsp+A0h] [rbp-30h]
  long long v39; // [rsp+A8h] [rbp-28h]
  long long v40; // [rsp+B0h] [rbp-20h]
  long long v41; // [rsp+B8h] [rbp-18h]
  unsigned long long v42; // [rsp+C0h] [rbp-10h]
  long long v43; // [rsp+C8h] [rbp-8h]

  sub_E09D((unsigned int *)a1);
  sub_EDAD(a1);
  if ( *(int *)a1 == *(int *)(a1 + 304) )
  {
    *(long long *)(a1 + 464) = *(long long *)(a1 + 160);
    *(long long *)(a1 + 472) = *(long long *)(a1 + 168);
    *(long long *)(a1 + 480) = *(long long *)(a1 + 176);
    *(char *)(a1 + 592) = *(char *)(a1 + 288);
  }
  else
  {
    sub_E09D((unsigned int *)(a1 + 304));
    sub_EDAD(a1 + 304);
  }
  v14 = *(long long **)(a1 + 160);
  v15 = *(long long **)(a1 + 464);
  v31 = v14;
  v32 = v15;
  v33 = *(long long *)(a1 + 176);
  v34 = *(long long *)(a1 + 480);
  if ( v14 == v15 )
  {
    i = (char *)v15 + v34;
    v16 = (char *)v15 + v34;
  }
  else
  {
    if ( v33 >= v34 )
      *((char *)v15 + v34) = ~*((char *)v14 + v34);
    else
      *((char *)v14 + v33) = ~*((char *)v15 + v33);
    while ( *v14 == *v15 )
    {
      ++v14;
      ++v15;
    }
    v16 = v14;
    for ( i = v15; *v16 == *i; ++i )
      ++v16;
    if ( dword_2D1C0 != 4
      && dword_2D1C0 != 5
      && v16 > &v31[v33 - *(unsigned char *)(a1 + 288)] != i > (char *)v32 + v34 - *(unsigned char *)(a1 + 592) )
    {
      --v16;
      --i;
    }
  }
  v23 = qword_2D1E0;
  while ( v16 != v31 )
  {
    if ( *(v16 - 1) == 10 && !v23-- )
      break;
    --v16;
    --i;
  }
  *(long long *)(a1 + 224) = v16;
  *(long long *)(a1 + 528) = i;
  v17 = &v31[v33];
  v20 = (long long *)((char *)v32 + v34);
  if ( dword_2D1C0 == 4 || dword_2D1C0 == 5 || *(char *)(a1 + 288) == *(char *)(a1 + 592) )
  {
    if ( v33 < v34 )
      v2 = 0;
    else
      v2 = v33 - v34;
    v21 = (char *)(*(long long *)(a1 + 224) + v2);
    while ( v17 != v21 )
    {
      --v17;
      v20 = (long long *)((char *)v20 - 1);
      if ( *v17 != *(char *)v20 )
      {
        ++v17;
        v20 = (long long *)((char *)v20 + 1);
        v21 = v17;
        break;
      }
    }
    v3 = v31 != v17 && *(v17 - 1) != 10 || v32 != v20 && *((char *)v20 - 1) != 10;
    v24 = v3 + qword_2D1E0;
    while ( 1 )
    {
      v5 = v24--;
      if ( !v5 || v17 == &v31[v33] )
        break;
      do
        v4 = v17++;
      while ( *v4 != 10 );
    }
    v20 = (long long *)((char *)v20 + v17 - v21);
  }
  *(long long *)(a1 + 240) = v17;
  *(long long *)(a1 + 544) = v20;
  if ( !byte_2D1C8 || buffer.fastmap || qword_2D1D0 > 0x1FFFFFFFFFFFFFFELL || v33 <= qword_2D1D0 )
  {
    j = 0;
    v29 = sub_EEF5(0, 0, v33);
  }
  else
  {
    v36 = sub_EEF5(0, 0, (unsigned long long)&v17[-*(long long *)(a1 + 224)]);
    v38 = sub_EEF5(0, 0, &v31[v33] - v17);
    for ( j = 1; j <= qword_2D1D0; j *= 2 )
      ;
    v6 = v38;
    if ( qword_2D1D0 <= v38 )
      v6 = qword_2D1D0;
    v29 = j + v36 + v6;
  }
  v39 = j - 1;
  v28 = 0;
  v22 = sub_1C945(8 * v29);
  v7 = byte_2D1C8 != 1 || v17 != *(char **)(a1 + 224) || v20 != *(long long **)(a1 + 528);
  v18 = v31;
  if ( v7 )
  {
    v35 = *(char **)(a1 + 224);
    while ( v18 != v35 )
    {
      v8 = v28++;
      v40 = v39 & v8;
      if ( (v39 & v8) == v29 )
      {
        if ( v29 > 0x7FFFFFFFFFFFFFELL )
          sub_1CF09();
        v29 *= 2LL;
        v22 = sub_1C991(v22, 8 * v29);
      }
      *(long long *)(v22 + 8 * v40) = v18;
      do
        v9 = v18++;
      while ( *v9 != 10 );
    }
  }
  if ( j && v28 > qword_2D1D0 )
    v10 = qword_2D1D0;
  else
    v10 = v28;
  v41 = v10;
  v37 = sub_EEF5(v28, v18 - v31, (unsigned long long)v20 - *(long long *)(a1 + 528));
  v11 = sub_EEF5(v28, v18 - v31, (char *)v32 + v34 - (char *)v20);
  v12 = v41 + v37;
  if ( qword_2D1D0 <= v11 )
    v11 = qword_2D1D0;
  v42 = v12 + v11;
  if ( (long long)(v12 + v11) < v41 || v42 > 0xFFFFFFFFFFFFFFELL )
    sub_1CF09();
  v43 = sub_1C945(8 * v42);
  if ( v41 != v28 )
  {
    for ( k = 0; k < v41; ++k )
      *(long long *)(8 * k + v43) = *(long long *)(8 * (v39 & (v28 - qword_2D1D0 + k)) + v22);
    for ( m = 0; m < v41; ++m )
      *(long long *)(8 * m + v22) = *(long long *)(8 * m + v43);
  }
  for ( n = 0; n < v41; ++n )
    *(long long *)(8 * n + v43) = (char *)v32 + *(long long *)(8 * n + v22) - (long long)v31;
  *(long long *)(a1 + 184) = v22 + 8 * v41;
  *(long long *)(a1 + 488) = 8 * v41 + v43;
  *(long long *)(a1 + 496) = -v41;
  *(long long *)(a1 + 192) = *(long long *)(a1 + 496);
  *(long long *)(a1 + 216) = v29 - v41;
  *(long long *)(a1 + 520) = v42 - v41;
  *(long long *)(a1 + 536) = v28;
  *(long long *)(a1 + 232) = *(long long *)(a1 + 536);
  return a1;
}



// Function: read_files @ 0xfa13
long long read_files(unsigned int *a1, char a2)
{
  bool v3; // [rsp+1Ah] [rbp-6h]
  char v4; // [rsp+1Bh] [rbp-5h]
  int i; // [rsp+1Ch] [rbp-4h]
  int j; // [rsp+1Ch] [rbp-4h]

  v4 = (unsigned char)(a2 | byte_2D1D8) != 0;
  v3 = (unsigned char)(a2 | sub_DF00(a1, v4)) != 0;
  if ( *a1 == a1[76] )
  {
    *((long long *)a1 + 58) = *((long long *)a1 + 20);
    *((long long *)a1 + 59) = *((long long *)a1 + 21);
    *((long long *)a1 + 60) = *((long long *)a1 + 22);
  }
  else
  {
    v3 = (v3 | sub_DF00(a1 + 76, v3 | v4)) != 0;
  }
  if ( v3 )
  {
    sub_131E6(*a1, 0);
    sub_131E6(a1[76], 0);
    return 1;
  }
  else
  {
    sub_EF6A((long long)a1);
    qword_2D778 = *((long long *)a1 + 27) + *((long long *)a1 + 65) + 1LL;
    if ( (unsigned long long)qword_2D778 > 0x3FFFFFFFFFFFFFELL )
      sub_1CF09();
    ptr = (void *)sub_1C945(32 * qword_2D778);
    qword_2D770 = 1;
    for ( i = 9; 1LL << i < (unsigned long long)(qword_2D778 / 3); ++i )
      ;
    qword_2D760 = (1LL << i) - byte_25240[i];
    if ( (unsigned long long)qword_2D760 > 0xFFFFFFFFFFFFFFELL )
      sub_1CF09();
    qword_2D758 = sub_13030(8 * (qword_2D760 + 1)) + 8;
    for ( j = 0; j <= 1; ++j )
      sub_E2F8((long long)&a1[76 * j]);
    *((long long *)a1 + 75) = qword_2D770;
    *((long long *)a1 + 37) = *((long long *)a1 + 75);
    free(ptr);
    free((void *)(qword_2D758 - 8));
    return 0;
  }
}



// Function: print_normal_script @ 0xfcfd
long long print_normal_script(long long a1)
{
  return sub_1245B(a1, sub_12437, sub_FD2D);
}



// Function: print_normal_hunk @ 0xfd2d
unsigned long long print_normal_hunk(long long a1)
{
  long long i; // rbx
  long long j; // rbx
  int v4; // [rsp+14h] [rbp-3Ch]
  long long v5; // [rsp+18h] [rbp-38h] BYREF
  long long v6; // [rsp+20h] [rbp-30h] BYREF
  long long v7; // [rsp+28h] [rbp-28h] BYREF
  long long v8; // [rsp+30h] [rbp-20h] BYREF
  unsigned long long v9; // [rsp+38h] [rbp-18h]

  v9 = __readfsqword(0x28u);
  v4 = sub_12AF4(a1, &v5, &v6, &v7, &v8);
  if ( v4 )
  {
    sub_11B50();
    sub_128C2(4);
    sub_12A37(44, &unk_2D360, v5, v6);
    fputc_unlocked(byte_25780[v4], stream);
    sub_12A37(44, &unk_2D490, v7, v8);
    sub_128C2(3);
    fputc_unlocked(10, stream);
    if ( (v4 & 1) != 0 )
    {
      for ( i = v5; i <= v6; ++i )
      {
        sub_128C2(2);
        sub_124FE(&unk_25280, 8 * i + qword_2D418, 1);
        sub_128C2(3);
        if ( *(char *)(*(long long *)(qword_2D418 + 8 * (i + 1)) - 1LL) == 10 )
          putc_unlocked(10, stream);
      }
    }
    if ( v4 == 3 )
      fputs_unlocked("---\n", stream);
    if ( (v4 & 2) != 0 )
    {
      for ( j = v7; j <= v8; ++j )
      {
        sub_128C2(1);
        sub_124FE(">", 8 * j + qword_2D548, 1);
        sub_128C2(3);
        if ( *(char *)(*(long long *)(qword_2D548 + 8 * (j + 1)) - 1LL) == 10 )
          putc_unlocked(10, stream);
      }
    }
  }
  return v9 - __readfsqword(0x28u);
}



// Function: print_sdiff_script @ 0xff70
long long print_sdiff_script(long long a1)
{
  sub_11B50();
  qword_2D788 = -qword_2D448;
  qword_2D780 = -qword_2D448;
  sub_1245B(a1, sub_12437, sub_106E7);
  return sub_1055A(qword_2D430, qword_2D560);
}



// Function: tab_from_to @ 0xffdd
unsigned long long tab_from_to(unsigned long long a1, unsigned long long a2)
{
  unsigned long long v2; // rax
  unsigned long long v4; // [rsp+8h] [rbp-28h]
  unsigned long long i; // [rsp+18h] [rbp-18h]
  FILE *stream; // [rsp+20h] [rbp-10h]
  long long v7; // [rsp+28h] [rbp-8h]

  v4 = a1;
  stream = ::stream;
  v7 = qword_2D2A8;
  if ( byte_2D2A1 != 1 )
  {
    for ( i = a1 + qword_2D2A8 - a1 % qword_2D2A8; i <= a2; i += v7 )
    {
      putc_unlocked(9, stream);
      v4 = i;
    }
  }
  while ( 1 )
  {
    v2 = v4++;
    if ( a2 <= v2 )
      break;
    putc_unlocked(32, stream);
  }
  return a2;
}



// Function: print_half_line @ 0x10090
unsigned long long print_half_line(char **a1, unsigned long long a2, unsigned long long a3)
{
  unsigned long long v3; // rbx
  unsigned long long v4; // r12
  char *v5; // r14
  unsigned long long v6; // r15
  char *v7; // rax
  char v8; // r13
  unsigned long long v9; // rax
  wchar_t c; // [rsp+20h] [rbp-70h] BYREF
  int v13; // [rsp+24h] [rbp-6Ch]
  unsigned long long v14; // [rsp+28h] [rbp-68h]
  FILE *stream; // [rsp+30h] [rbp-60h]
  void *ptr; // [rsp+38h] [rbp-58h]
  unsigned long long v17; // [rsp+40h] [rbp-50h]
  size_t n; // [rsp+48h] [rbp-48h]
  long long v19[8]; // [rsp+50h] [rbp-40h] BYREF

  v19[1] = __readfsqword(0x28u);
  stream = ::stream;
  v3 = 0;
  v4 = 0;
  v5 = *a1;
  v6 = (unsigned long long)a1[1];
  v19[0] = 0;
  while ( (unsigned long long)v5 < v6 )
  {
    ptr = v5;
    v7 = v5++;
    v8 = *v7;
    if ( *v7 > 95 )
    {
      if ( (unsigned int)(v8 - 97) > 0x1D )
      {
LABEL_26:
        n = sub_1D369(&c, ptr, v6 - (long long)ptr, v19);
        if ( n && n <= 0xFFFFFFFFFFFFFFFDLL )
        {
          v13 = wcwidth(c);
          if ( v13 > 0 )
            v3 += v13;
          if ( v3 <= a3 )
          {
            v4 = v3;
            fwrite_unlocked(ptr, 1u, n, stdout);
          }
          v5 = (char *)ptr + n;
        }
        else
        {
LABEL_33:
          if ( v3 < a3 )
            putc_unlocked(v8, stream);
        }
      }
      else
      {
LABEL_35:
        v9 = v3++;
        if ( a3 > v9 )
        {
LABEL_36:
          v4 = v3;
          putc_unlocked(v8, stream);
        }
      }
    }
    else
    {
      if ( v8 < 8 )
        goto LABEL_26;
      switch ( v8 )
      {
        case 8:
          if ( !v3 )
            continue;
          if ( --v3 >= a3 )
            continue;
          if ( v4 > v3 )
            goto LABEL_36;
          while ( v4 < v3 )
          {
            putc_unlocked(32, stream);
            ++v4;
          }
          break;
        case 9:
          v17 = qword_2D2A8 - v3 % qword_2D2A8;
          if ( v3 == v4 )
          {
            v14 = v4 + v17;
            if ( byte_2D2A1 )
            {
              if ( a3 < v14 )
                v14 = a3;
              while ( v4 < v14 )
              {
                putc_unlocked(32, stream);
                ++v4;
              }
            }
            else if ( v14 < a3 )
            {
              v4 = v14;
              putc_unlocked(v8, stream);
            }
          }
          v3 += v17;
          continue;
        case 10:
          return v4;
        case 11:
        case 12:
          goto LABEL_33;
        case 13:
          putc_unlocked(v8, stream);
          sub_FFDD(0, a2);
          v4 = 0;
          v3 = 0;
          continue;
        case 32:
        case 33:
        case 34:
        case 35:
        case 37:
        case 38:
        case 39:
        case 40:
        case 41:
        case 42:
        case 43:
        case 44:
        case 45:
        case 46:
        case 47:
        case 48:
        case 49:
        case 50:
        case 51:
        case 52:
        case 53:
        case 54:
        case 55:
        case 56:
        case 57:
        case 58:
        case 59:
        case 60:
        case 61:
        case 62:
        case 63:
        case 65:
        case 66:
        case 67:
        case 68:
        case 69:
        case 70:
        case 71:
        case 72:
        case 73:
        case 74:
        case 75:
        case 76:
        case 77:
        case 78:
        case 79:
        case 80:
        case 81:
        case 82:
        case 83:
        case 84:
        case 85:
        case 86:
        case 87:
        case 88:
        case 89:
        case 90:
        case 91:
        case 92:
        case 93:
        case 94:
        case 95:
          goto LABEL_35;
        default:
          goto LABEL_26;
      }
    }
  }
  return v4;
}



// Function: print_1sdiff_line @ 0x10388
int print_1sdiff_line(long long a1, char a2, char **a3)
{
  unsigned long long v3; // rax
  char v4; // al
  char v7; // [rsp+14h] [rbp-3Ch]
  bool v8; // [rsp+2Eh] [rbp-22h]
  char v9; // [rsp+2Fh] [rbp-21h]
  unsigned long long v10; // [rsp+30h] [rbp-20h]
  unsigned long long v11; // [rsp+30h] [rbp-20h]
  FILE *stream; // [rsp+38h] [rbp-18h]
  unsigned long long v13; // [rsp+40h] [rbp-10h]
  unsigned long long v14; // [rsp+48h] [rbp-8h]

  v7 = a2;
  stream = ::stream;
  v13 = qword_2D320;
  LODWORD(v3) = qword_2D328;
  v14 = qword_2D328;
  v10 = 0;
  v8 = 0;
  v9 = 0;
  if ( a2 == 60 )
  {
    LODWORD(v3) = sub_128C2(2);
    v9 = 1;
  }
  else if ( a2 == 62 )
  {
    LODWORD(v3) = sub_128C2(1);
    v9 = 1;
  }
  if ( a1 )
  {
    v8 = *(char *)(*(long long *)(a1 + 8) - 1LL) == 10;
    v3 = sub_10090((char **)a1, 0, v13);
    v10 = v3;
  }
  if ( a2 != 32 )
  {
    v10 = sub_FFDD(v10, (v13 + v14 - 1) >> 1) + 1;
    if ( a2 == 124 && v8 != (*(a3[1] - 1) == 10) )
    {
      if ( v8 )
        v4 = 47;
      else
        v4 = 92;
      v7 = v4;
    }
    LODWORD(v3) = putc_unlocked(v7, stream);
  }
  if ( a3 )
  {
    v8 = v8 || *(a3[1] - 1) == 10;
    LODWORD(v3) = (unsigned char)**a3;
    if ( (char)v3 != 10 )
    {
      v11 = sub_FFDD(v10, v14);
      LODWORD(v3) = sub_10090(a3, v11, v13);
    }
  }
  if ( v8 )
    LODWORD(v3) = putc_unlocked(10, stream);
  if ( v9 )
    LODWORD(v3) = sub_128C2(3);
  return v3;
}



// Function: print_sdiff_common_lines @ 0x1055a
long long print_sdiff_common_lines(long long a1, long long a2)
{
  long long v2; // rax
  char **v3; // rdx
  long long v4; // rax
  long long v5; // rax
  long long v6; // rax
  long long v8; // [rsp+10h] [rbp-20h]
  long long v9; // [rsp+18h] [rbp-18h]

  v8 = qword_2D780;
  v9 = qword_2D788;
  if ( byte_2D31A != 1 && (qword_2D780 != a1 || qword_2D788 != a2) )
  {
    if ( byte_2D318 )
      fprintf(stream, "i%ld,%ld\n", a1 - qword_2D780, a2 - qword_2D788);
    if ( byte_2D319 != 1 )
    {
      while ( v8 != a1 && v9 != a2 )
      {
        v2 = v9++;
        v3 = (char **)(qword_2D548 + 8 * v2);
        v4 = v8++;
        sub_10388(qword_2D418 + 8 * v4, 32, v3);
      }
      while ( v9 != a2 )
      {
        v5 = v9++;
        sub_10388(0, 41, (char **)(qword_2D548 + 8 * v5));
      }
    }
    while ( v8 != a1 )
    {
      v6 = v8++;
      sub_10388(qword_2D418 + 8 * v6, 40, 0);
    }
  }
  qword_2D780 = a1;
  qword_2D788 = a2;
  return a2;
}



// Function: print_sdiff_hunk @ 0x106e7
unsigned long long print_sdiff_hunk(long long a1)
{
  long long v1; // rbx
  long long i; // r12
  char v3; // al
  long long j; // r12
  long long k; // rbx
  int v7; // [rsp+14h] [rbp-4Ch]
  long long v8; // [rsp+18h] [rbp-48h] BYREF
  long long v9; // [rsp+20h] [rbp-40h] BYREF
  long long v10; // [rsp+28h] [rbp-38h] BYREF
  long long v11; // [rsp+30h] [rbp-30h] BYREF
  long long v12; // [rsp+38h] [rbp-28h]
  long long v13; // [rsp+40h] [rbp-20h]
  unsigned long long v14; // [rsp+48h] [rbp-18h]

  v14 = __readfsqword(0x28u);
  v7 = sub_12AF4(a1, &v8, &v9, &v10, &v11);
  if ( v7 )
  {
    sub_1055A(v8, v10);
    if ( byte_2D318 )
    {
      v12 = v9 - v8 + 1;
      v13 = v11 - v10 + 1;
      fprintf(stream, "c%ld,%ld\n", v12, v13);
    }
    if ( v7 == 3 )
    {
      v1 = v8;
      for ( i = v10; v1 <= v9 && i <= v11; ++i )
        sub_10388(8 * v1++ + qword_2D418, 124, (char **)(qword_2D548 + 8 * i));
      if ( i > v11 )
        v3 = 0;
      else
        v3 = 2;
      LOBYTE(v7) = (v1 <= v9) + v3;
      v8 = v1;
      qword_2D780 = v1;
      v10 = i;
      qword_2D788 = i;
    }
    if ( (v7 & 2) != 0 )
    {
      for ( j = v10; j <= v11; ++j )
        sub_10388(0, 62, (char **)(8 * j + qword_2D548));
      qword_2D788 = j;
    }
    if ( (v7 & 1) != 0 )
    {
      for ( k = v8; k <= v9; ++k )
        sub_10388(8 * k + qword_2D418, 60, 0);
      qword_2D780 = k;
    }
  }
  return v14 - __readfsqword(0x28u);
}



// Function: perror_with_name @ 0x108ec
void perror_with_name(const char *a1)
{
  int *v1; // rax

  v1 = __errno_location();
  error(0, *v1, "%s", a1);
}



// Function: pfatal_with_name @ 0x10925
long long pfatal_with_name(const char *a1)
{
  int errnum; // [rsp+1Ch] [rbp-4h]

  errnum = *__errno_location();
  sub_10BCA();
  error(2, errnum, "%s", a1);
  return sub_10966(2);
}



// Function: fatal @ 0x10966
long long fatal(const char *a1)
{
  char *v1; // rax

  sub_10BCA();
  v1 = gettext(a1);
  error(2, 0, "%s", v1);
  return sub_109A8((char *)&dword_0 + 2);
}



// Function: message @ 0x109a8
long long message(const char *a1, long long a2, long long a3)
{
  return sub_109E2(a1, a2, a3, 0, 0);
}



// Function: message5 @ 0x109e2
unsigned long long message5(char *a1, char *a2, char *a3, char *a4, char *a5)
{
  char *v5; // rax
  char *v6; // rax
  int v7; // eax
  char *v8; // rax
  int i; // [rsp+34h] [rbp-7Ch]
  int v14; // [rsp+34h] [rbp-7Ch]
  char *dest; // [rsp+38h] [rbp-78h]
  long long v16; // [rsp+40h] [rbp-70h]
  long long *v17; // [rsp+48h] [rbp-68h]
  char *s[11]; // [rsp+50h] [rbp-60h]
  unsigned long long v19; // [rsp+A8h] [rbp-8h]

  v19 = __readfsqword(0x28u);
  if ( byte_2D2C0 )
  {
    v16 = 8;
    s[0] = a1;
    s[1] = a2;
    s[2] = a3;
    if ( a4 )
      v5 = a4;
    else
      v5 = (char *)&unk_2540F;
    s[3] = v5;
    if ( a5 )
      v6 = a5;
    else
      v6 = (char *)&unk_2540F;
    s[4] = v6;
    for ( i = 0; i <= 4; ++i )
    {
      s[i + 6] = (char *)(strlen(s[i]) + 1);
      v16 += (long long)s[i + 6];
    }
    v17 = (long long *)sub_1C945(v16);
    v14 = 0;
    dest = (char *)(v17 + 1);
    while ( v14 <= 4 )
    {
      memcpy(dest, s[v14], (size_t)s[v14 + 6]);
      v7 = v14++;
      dest = &dest[(long long)s[v7 + 6]];
    }
    *off_2D020 = (long long)v17;
    *v17 = 0;
    off_2D020 = v17;
  }
  else
  {
    if ( byte_2D318 )
      putchar_unlocked(32);
    v8 = gettext(a1);
    printf(v8, a2, a3, a4, a5);
  }
  return v19 - __readfsqword(0x28u);
}



// Function: print_message_queue @ 0x10bca
unsigned long long print_message_queue()
{
  const char *v0; // rbx
  char *v1; // r14
  char *v2; // r13
  char *v3; // r12
  char *v4; // rbx
  char *v5; // rax
  int i; // [rsp+Ch] [rbp-64h]
  char *ptr; // [rsp+10h] [rbp-60h]
  char *v9; // [rsp+18h] [rbp-58h]
  char *s[5]; // [rsp+20h] [rbp-50h]
  unsigned long long v11; // [rsp+48h] [rbp-28h]

  v11 = __readfsqword(0x28u);
  for ( ptr = (char *)qword_2D7A0; ptr; ptr = v9 )
  {
    v9 = *(char **)ptr;
    s[0] = ptr + 8;
    for ( i = 0; i <= 3; ++i )
    {
      v0 = s[i];
      s[i + 1] = (char *)&v0[strlen(v0) + 1];
    }
    v1 = s[4];
    v2 = s[3];
    v3 = s[2];
    v4 = s[1];
    v5 = gettext(s[0]);
    printf(v5, v4, v3, v2, v1);
    free(ptr);
  }
  return v11 - __readfsqword(0x28u);
}



// Function: sighandler @ 0x10ccc
long long sighandler(unsigned int a1)
{
  long long result; // rax

  result = (unsigned int)dword_2D840;
  if ( !dword_2D840 )
  {
    dword_2D840 = a1;
    return a1;
  }
  return result;
}



// Function: stophandler @ 0x10ced
long long stophandler()
{
  long long result; // rax

  result = (unsigned int)dword_2D840;
  if ( !dword_2D840 )
    return (unsigned int)++dword_2D844;
  return result;
}



// Function: process_signals @ 0x10d14
unsigned long long process_signals()
{
  int sig; // [rsp+8h] [rbp-98h]
  sigset_t oset; // [rsp+10h] [rbp-90h] BYREF
  unsigned long long v3; // [rsp+98h] [rbp-8h]

  v3 = __readfsqword(0x28u);
  while ( dword_2D840 || dword_2D844 )
  {
    sub_128C2(3);
    fflush_unlocked(stdout);
    sigprocmask(0, &set, &oset);
    sig = dword_2D840;
    if ( dword_2D844 )
    {
      --dword_2D844;
      raise(19);
    }
    else
    {
      signal(dword_2D840, 0);
      raise(sig);
    }
    sigprocmask(2, &oset, 0);
  }
  return v3 - __readfsqword(0x28u);
}



// Function: install_signal_handlers @ 0x10e15
unsigned long long install_signal_handlers()
{
  void (*v0)(int); // rax
  int i; // [rsp+Ch] [rbp-A4h]
  int j; // [rsp+Ch] [rbp-A4h]
  struct sigaction oact; // [rsp+10h] [rbp-A0h] BYREF
  unsigned long long v5; // [rsp+A8h] [rbp-8h]

  v5 = __readfsqword(0x28u);
  sigemptyset(&set);
  for ( i = 0; i <= 11; ++i )
  {
    sigaction(dword_257C0[i], 0, &oact);
    if ( oact.sa_handler != (__sighandler_t)((char *)&dword_0 + 1) )
      sigaddset(&set, dword_257C0[i]);
  }
  oact.sa_mask = set;
  oact.sa_flags = 0x10000000;
  for ( j = 0; j <= 11; ++j )
  {
    if ( sigismember(&set, dword_257C0[j]) )
    {
      if ( dword_257C0[j] == 20 )
        v0 = (void (*)(int))sub_10CED;
      else
        v0 = (void (*)(int))sub_10CCC;
      oact.sa_handler = v0;
      sigaction(dword_257C0[j], &oact, 0);
    }
  }
  return v5 - __readfsqword(0x28u);
}



// Function: get_funky_string @ 0x11052
bool get_funky_string(char **a1, char **a2, char a3, long long *a4)
{
  int v4; // eax
  char *v5; // rdx
  char *v6; // rax
  int v7; // eax
  char *v8; // rax
  char *v9; // rax
  char *v10; // rax
  int v11; // eax
  char *v12; // rax
  char *v13; // rax
  char *v14; // rax
  char *v15; // rax
  char *v16; // rax
  char v17; // cl
  char *v18; // rax
  char *v19; // rax
  char v23; // [rsp+23h] [rbp-1Dh]
  int v24; // [rsp+24h] [rbp-1Ch]
  long long v25; // [rsp+28h] [rbp-18h]
  char *v26; // [rsp+30h] [rbp-10h]
  char *v27; // [rsp+38h] [rbp-8h]

  v26 = *a2;
  v27 = *a1;
  v25 = 0;
  v23 = 0;
  v24 = 0;
  while ( 1 )
  {
    switch ( v24 )
    {
      case 0:
        v4 = *v26;
        if ( v4 == 94 )
        {
          v24 = 4;
          ++v26;
          continue;
        }
        if ( v4 > 94 )
          goto LABEL_16;
        if ( v4 == 92 )
        {
          v24 = 1;
          ++v26;
          continue;
        }
        if ( v4 > 92 )
          goto LABEL_16;
        if ( v4 == 61 )
        {
          if ( a3 )
          {
            v24 = 5;
            continue;
          }
        }
        else if ( v4 <= 61 && (!*v26 || v4 == 58) )
        {
          v24 = 5;
          continue;
        }
LABEL_16:
        v5 = v26++;
        v6 = v27++;
        *v6 = *v5;
        ++v25;
        continue;
      case 1:
        v7 = *v26;
        if ( *v26 )
        {
          if ( (unsigned int)v7 > 0x78 || v7 < 48 )
          {
LABEL_34:
            v23 = *v26;
          }
          else
          {
            switch ( *v26 )
            {
              case '0':
              case '1':
              case '2':
              case '3':
              case '4':
              case '5':
              case '6':
              case '7':
                v24 = 2;
                v23 = *v26 - 48;
                break;
              case '?':
                v23 = 127;
                break;
              case 'X':
              case 'x':
                v24 = 3;
                v23 = 0;
                break;
              case '_':
                v23 = 32;
                break;
              case 'a':
                v23 = 7;
                break;
              case 'b':
                v23 = 8;
                break;
              case 'e':
                v23 = 27;
                break;
              case 'f':
                v23 = 12;
                break;
              case 'n':
                v23 = 10;
                break;
              case 'r':
                v23 = 13;
                break;
              case 't':
                v23 = 9;
                break;
              case 'v':
                v23 = 11;
                break;
              default:
                goto LABEL_34;
            }
          }
        }
        else
        {
          v24 = 6;
        }
        if ( v24 == 1 )
        {
          v8 = v27++;
          *v8 = v23;
          ++v25;
          v24 = 0;
        }
        ++v26;
        continue;
      case 2:
        if ( *v26 > 47 && *v26 <= 55 )
        {
          v10 = v26++;
          v23 = 8 * v23 + *v10 - 48;
        }
        else
        {
          v9 = v27++;
          *v9 = v23;
          ++v25;
          v24 = 0;
        }
        continue;
      case 3:
        v11 = *v26;
        if ( v11 > 102 )
          goto LABEL_51;
        if ( v11 >= 97 )
        {
          v13 = v26++;
          v23 = 16 * v23 + *v13 - 87;
        }
        else
        {
          if ( v11 > 57 )
          {
            if ( (unsigned int)(v11 - 65) <= 5 )
            {
              v14 = v26++;
              v23 = 16 * v23 + *v14 - 55;
              continue;
            }
          }
          else if ( v11 >= 48 )
          {
            v12 = v26++;
            v23 = 16 * v23 + *v12 - 48;
            continue;
          }
LABEL_51:
          v15 = v27++;
          *v15 = v23;
          ++v25;
          v24 = 0;
        }
        break;
      case 4:
        v24 = 0;
        if ( *v26 <= 63 || *v26 == 127 )
        {
          if ( *v26 == 63 )
          {
            v19 = v27++;
            *v19 = 127;
            ++v25;
          }
          else
          {
            v24 = 6;
          }
        }
        else
        {
          v16 = v26++;
          v17 = *v16;
          v18 = v27++;
          *v18 = v17 & 0x1F;
          ++v25;
        }
        continue;
      default:
        *a1 = v27;
        *a2 = v26;
        *a4 = v25;
        return v24 != 6;
    }
  }
}



// Function: set_color_palette @ 0x1140f
long long set_color_palette(long long a1)
{
  qword_2D868 = a1;
  return a1;
}



// Function: parse_diff_color @ 0x11429
unsigned long long parse_diff_color()
{
  int v0; // eax
  int v1; // eax
  char *v2; // rax
  char *v3; // rax
  char *v4; // rax
  int v5; // eax
  char *v6; // rax
  char *v7; // rax
  int v8; // eax
  char *v9; // rax
  int v11; // [rsp+8h] [rbp-48h]
  int v12; // [rsp+Ch] [rbp-44h]
  char *v13; // [rsp+10h] [rbp-40h] BYREF
  char *v14; // [rsp+18h] [rbp-38h] BYREF
  long long *v15; // [rsp+20h] [rbp-30h]
  long long *v16; // [rsp+28h] [rbp-28h]
  void *ptr; // [rsp+30h] [rbp-20h]
  void *v18; // [rsp+38h] [rbp-18h]
  char dest[3]; // [rsp+45h] [rbp-Bh] BYREF
  unsigned long long v20; // [rsp+48h] [rbp-8h]

  v20 = __readfsqword(0x28u);
  v13 = (char *)qword_2D868;
  if ( !qword_2D868 || !*v13 )
    return v20 - __readfsqword(0x28u);
  v15 = 0;
  strcpy(dest, "??");
  ptr = (void *)sub_1CED8(v13);
  v14 = (char *)ptr;
  v12 = 1;
  while ( 2 )
  {
    switch ( v12 )
    {
      case 1:
        v0 = *v13;
        if ( v0 == 58 )
        {
          ++v13;
          continue;
        }
        if ( v0 > 58 )
          goto LABEL_16;
        if ( *v13 )
        {
          if ( v0 == 42 )
          {
            v15 = (long long *)sub_1C945(40);
            v15[4] = qword_2D860;
            qword_2D860 = (long long)v15;
            ++v13;
            v15[1] = v14;
            if ( sub_11052(&v14, &v13, 1, v15) )
              v1 = 4;
            else
              v1 = 6;
            v12 = v1;
          }
          else
          {
LABEL_16:
            v2 = v13++;
            dest[0] = *v2;
            v12 = 2;
          }
          continue;
        }
        v12 = 5;
LABEL_38:
        if ( v12 == 6 )
        {
          v9 = gettext("unparsable value for --palette");
          error(0, 0, v9);
          free(ptr);
          v16 = (long long *)qword_2D860;
          while ( v16 )
          {
            v18 = v16;
            v16 = (long long *)v16[4];
            free(v18);
          }
          byte_2D859 = 0;
        }
        return v20 - __readfsqword(0x28u);
      case 2:
        if ( *v13 )
        {
          v3 = v13++;
          dest[1] = *v3;
          v12 = 3;
        }
        else
        {
          v12 = 6;
        }
        continue;
      case 3:
        v12 = 6;
        v4 = v13++;
        if ( *v4 != 61 )
          continue;
        v11 = 0;
        while ( 2 )
        {
          if ( *(&off_2C920 + v11) )
          {
            if ( strcmp(dest, (const char *)*(&off_2C920 + v11)) )
            {
              ++v11;
              continue;
            }
            *(&off_2D048 + 2 * v11) = v14;
            if ( sub_11052(&v14, &v13, 0, (long long *)&unk_2D040 + 2 * v11) )
              v5 = 1;
            else
              v5 = 6;
            v12 = v5;
          }
          break;
        }
        if ( v12 == 6 )
        {
          v6 = gettext("unrecognized prefix: %s");
          error(0, 0, v6, dest);
        }
        continue;
      case 4:
        v7 = v13++;
        if ( *v7 == 61 )
        {
          v15[3] = v14;
          if ( sub_11052(&v14, &v13, 0, v15 + 2) )
            v8 = 1;
          else
            v8 = 6;
          v12 = v8;
        }
        else
        {
          v12 = 6;
        }
        continue;
      case 6:
        goto LABEL_38;
    }
  }
}



// Function: check_color_output @ 0x117e9
unsigned long long check_color_output(char a1)
{
  unsigned long long result; // rax
  int v2; // eax
  bool v3; // al
  bool v4; // al
  bool v5; // [rsp+1Fh] [rbp-1h]

  result = (unsigned long long)stream;
  if ( stream )
  {
    result = (unsigned int)dword_2D1C4;
    if ( dword_2D1C4 )
    {
      v3 = 1;
      if ( !byte_2D5C8 )
      {
        if ( a1 == 1 || (v2 = fileno(stream), !isatty(v2)) )
          v3 = 0;
      }
      v5 = v3;
      v4 = dword_2D1C4 == 2 || dword_2D1C4 == 1 && v3;
      byte_2D859 = v4;
      result = v4;
      if ( (char)result )
        result = sub_11429();
      if ( v5 )
        return sub_10E15();
    }
  }
  return result;
}



// Function: setup_output @ 0x118ac
long long setup_output(long long a1, long long a2, unsigned char a3)
{
  qword_2D848 = a1;
  qword_2D850 = a2;
  byte_2D858 = a3;
  stream = 0;
  return a3;
}



// Function: c_escape_char @ 0x118ef
char c_escape_char(char a1)
{
  char result; // al

  if ( a1 > 34 )
  {
    if ( a1 == 92 )
      return 92;
    return a1 <= 31;
  }
  if ( a1 < 7 )
    return a1 <= 31;
  switch ( a1 )
  {
    case 7:
      result = 97;
      break;
    case 8:
      result = 98;
      break;
    case 9:
      result = 116;
      break;
    case 10:
      result = 110;
      break;
    case 11:
      result = 118;
      break;
    case 12:
      result = 102;
      break;
    case 13:
      result = 114;
      break;
    case 34:
      result = 34;
      break;
    default:
      return a1 <= 31;
  }
  return result;
}



// Function: c_escape @ 0x11984
char *c_escape(char *a1)
{
  int v1; // eax
  char *v2; // rax
  char *v3; // rax
  char *v4; // rax
  char v6; // [rsp+14h] [rbp-2Ch]
  char v7; // [rsp+15h] [rbp-2Bh]
  char v8; // [rsp+16h] [rbp-2Ah]
  char *i; // [rsp+18h] [rbp-28h]
  char *j; // [rsp+18h] [rbp-28h]
  long long v11; // [rsp+20h] [rbp-20h]
  char *v12; // [rsp+28h] [rbp-18h]
  char *v13; // [rsp+38h] [rbp-8h]

  v11 = 0;
  v6 = 0;
  for ( i = a1; *i; ++i )
  {
    if ( *i == 32 )
    {
      v6 = 1;
    }
    else
    {
      v1 = sub_118EF(*i);
      if ( (char)v1 )
      {
        if ( v1 == 1 )
          v11 += 3;
        else
          ++v11;
      }
    }
  }
  if ( !v6 && !v11 )
    return a1;
  v13 = (char *)sub_1C945(i - a1 + v11 + 3);
  v12 = v13 + 1;
  *v13 = 34;
  for ( j = a1; *j; ++j )
  {
    v7 = *j;
    v8 = sub_118EF(*j);
    if ( v8 )
    {
      *v12 = 92;
      if ( v8 == 1 )
      {
        v12[1] = ((unsigned char)v7 >> 6) + 48;
        v12[2] = ((v7 >> 3) & 7) + 48;
        v3 = v12 + 3;
        v12 += 4;
        *v3 = (v7 & 7) + 48;
      }
      else
      {
        v4 = v12 + 1;
        v12 += 2;
        *v4 = v8;
      }
    }
    else
    {
      v2 = v12++;
      *v2 = v7;
    }
  }
  *v12 = 34;
  v12[1] = 0;
  return v13;
}



// Function: begin_output @ 0x11b50
unsigned long long begin_output()
{
  int v0; // r8d
  int v1; // r9d
  char *v2; // rax
  int v3; // eax
  char *ptr; // [rsp+0h] [rbp-50h]
  int pipedes[2]; // [rsp+8h] [rbp-48h] BYREF
  void *v7; // [rsp+10h] [rbp-40h] BYREF
  void *v8; // [rsp+18h] [rbp-38h]
  char *argv[5]; // [rsp+20h] [rbp-30h] BYREF
  unsigned long long v10; // [rsp+48h] [rbp-8h]

  v10 = __readfsqword(0x28u);
  if ( !stream )
  {
    v7 = sub_11984((char *)qword_2D848);
    v8 = sub_11984((char *)qword_2D850);
    ptr = (char *)sub_1D0F7((unsigned int)"diff%s %s %s", qword_2D330, (int)v7, (int)v8, v0, v1);
    if ( byte_2D2C0 )
    {
      if ( fflush_unlocked(stdout) )
      {
        v2 = gettext("write failed");
        sub_10925(v2);
      }
      argv[0] = "/usr/bin/pr";
      argv[1] = "-h";
      argv[2] = ptr;
      argv[3] = 0;
      if ( pipe(pipedes) )
        sub_10925("pipe");
      pid = fork();
      if ( pid < 0 )
        sub_10925("fork");
      if ( !pid )
      {
        close(pipedes[1]);
        if ( pipedes[0] )
        {
          if ( dup2(pipedes[0], 0) < 0 )
            sub_10925("dup2");
          close(pipedes[0]);
        }
        execv("/usr/bin/pr", argv);
        if ( *__errno_location() == 2 )
          v3 = 127;
        else
          v3 = 126;
        _exit(v3);
      }
      close(pipedes[0]);
      stream = fdopen(pipedes[1], "w");
      if ( !stream )
        sub_10925("fdopen");
      sub_117E9(1);
    }
    else
    {
      stream = stdout;
      sub_117E9(0);
      if ( byte_2D858 )
        printf("%s\n", ptr);
    }
    free(ptr);
    if ( dword_2D1C0 == 2 )
    {
      sub_7E5C((long long)&unk_2D360, (const char **)&v7, 0);
    }
    else if ( dword_2D1C0 == 3 )
    {
      sub_7E5C((long long)&unk_2D360, (const char **)&v7, 1);
    }
    if ( v7 != (void *)qword_2D848 )
      free(v7);
    if ( v8 != (void *)qword_2D850 )
      free(v8);
  }
  return v10 - __readfsqword(0x28u);
}



// Function: finish_output @ 0x11e1b
unsigned long long finish_output()
{
  char *v0; // rax
  int v1; // eax
  const char *v2; // rax
  char *v3; // rax
  int stat_loc; // [rsp+Ch] [rbp-14h] BYREF
  int errnum; // [rsp+10h] [rbp-10h]
  unsigned int v7; // [rsp+14h] [rbp-Ch]
  unsigned long long v8; // [rsp+18h] [rbp-8h]

  v8 = __readfsqword(0x28u);
  if ( stream && stream != stdout )
  {
    errnum = 0;
    if ( ferror_unlocked(stream) )
      sub_10966("write failed");
    if ( fclose(stream) )
    {
      v0 = gettext("write failed");
      sub_10925(v0);
    }
    if ( waitpid(pid, &stat_loc, 0) < 0 )
      sub_10925("waitpid");
    if ( errnum || (stat_loc & 0x7F) != 0 )
      v1 = 0x7FFFFFFF;
    else
      v1 = BYTE1(stat_loc);
    v7 = v1;
    if ( v1 )
    {
      switch ( v7 )
      {
        case 0x7Eu:
          v2 = "subsidiary program '%s' could not be invoked";
          break;
        case 0x7Fu:
          v2 = "subsidiary program '%s' not found";
          break;
        case 0x7FFFFFFFu:
          v2 = "subsidiary program '%s' failed";
          break;
        default:
          v2 = "subsidiary program '%s' failed (exit status %d)";
          break;
      }
      v3 = gettext(v2);
      error(2, errnum, v3, "/usr/bin/pr", v7);
    }
  }
  stream = 0;
  return v8 - __readfsqword(0x28u);
}



// Function: lines_differ @ 0x11f90
long long lines_differ(unsigned long long a1, unsigned long long a2)
{
  unsigned long long v2; // r13
  unsigned long long v3; // r14
  unsigned char *v4; // rax
  unsigned char v5; // bl
  unsigned char *v6; // rax
  unsigned char v7; // r12
  unsigned char *v8; // rax
  unsigned char *v9; // rax
  unsigned char *v10; // rax
  unsigned char *v11; // rax
  unsigned char *v13; // rax
  unsigned char *v14; // rax
  unsigned long long v15; // rax
  unsigned char v16; // [rsp+1Fh] [rbp-41h]
  unsigned char v17; // [rsp+1Fh] [rbp-41h]
  unsigned long long v18; // [rsp+20h] [rbp-40h]
  unsigned char *i; // [rsp+28h] [rbp-38h]
  unsigned char *j; // [rsp+30h] [rbp-30h]
  unsigned long long v21; // [rsp+38h] [rbp-28h]

  v2 = a1;
  v3 = a2;
  v18 = 0;
  while ( 2 )
  {
    while ( 2 )
    {
      v4 = (unsigned char *)v2++;
      v5 = *v4;
      v6 = (unsigned char *)v3++;
      v7 = *v6;
      if ( v5 == *v6 )
        goto LABEL_69;
      switch ( dword_2D1E8 )
      {
        case 1:
          goto LABEL_49;
        case 2:
        case 3:
          if ( ((*__ctype_b_loc())[v5] & 0x2000) != 0 && ((*__ctype_b_loc())[v7] & 0x2000) != 0 )
          {
            if ( v5 == 10 )
              goto LABEL_46;
            for ( i = (unsigned char *)v2; ; ++i )
            {
              v16 = *i;
              if ( *i == 10 || ((*__ctype_b_loc())[v16] & 0x2000) == 0 )
                break;
            }
            if ( v16 == 10 )
            {
LABEL_46:
              if ( v7 == 10 )
                return 0;
              for ( j = (unsigned char *)v3; ; ++j )
              {
                v17 = *j;
                if ( *j == 10 || ((*__ctype_b_loc())[v17] & 0x2000) == 0 )
                  break;
              }
              if ( v17 == 10 )
                return 0;
            }
            goto LABEL_66;
          }
          if ( dword_2D1E8 == 2 )
            goto LABEL_66;
LABEL_49:
          if ( (v5 != 32 || v7 != 9) && (v5 != 9 || v7 != 32) )
            goto LABEL_66;
          v21 = v18;
          while ( 2 )
          {
            if ( v5 == 32 )
            {
              ++v18;
LABEL_58:
              v13 = (unsigned char *)v2++;
              v5 = *v13;
              continue;
            }
            break;
          }
          if ( v5 == 9 )
          {
            v18 += qword_2D2A8 - v18 % qword_2D2A8;
            goto LABEL_58;
          }
          break;
        case 4:
          if ( ((*__ctype_b_loc())[v5] & 0x2000) == 0 )
            goto LABEL_17;
          do
          {
            if ( v5 == 10 )
              goto LABEL_17;
            v10 = (unsigned char *)v2++;
            v5 = *v10;
          }
          while ( ((*__ctype_b_loc())[v5] & 0x2000) != 0 );
          --v2;
          v5 = 32;
LABEL_17:
          if ( ((*__ctype_b_loc())[v7] & 0x2000) != 0 )
          {
            do
            {
              if ( v7 == 10 )
                goto LABEL_22;
              v11 = (unsigned char *)v3++;
              v7 = *v11;
            }
            while ( ((*__ctype_b_loc())[v7] & 0x2000) != 0 );
            --v3;
            v7 = 32;
          }
LABEL_22:
          if ( v5 == v7 )
            goto LABEL_66;
          if ( v7 == 32 && v5 != 10 && v2 > a1 + 1 && ((*__ctype_b_loc())[*(unsigned char *)(v2 - 2)] & 0x2000) != 0 )
          {
            --v2;
          }
          else
          {
            if ( v5 != 32
              || v7 == 10
              || v3 <= a2 + 1
              || ((*__ctype_b_loc())[*(unsigned char *)(v3 - 2)] & 0x2000) == 0 )
            {
              goto LABEL_66;
            }
            --v3;
          }
          continue;
        case 5:
          while ( ((*__ctype_b_loc())[v5] & 0x2000) != 0 && v5 != 10 )
          {
            v8 = (unsigned char *)v2++;
            v5 = *v8;
          }
          while ( ((*__ctype_b_loc())[v7] & 0x2000) != 0 && v7 != 10 )
          {
            v9 = (unsigned char *)v3++;
            v7 = *v9;
          }
          goto LABEL_66;
        default:
          goto LABEL_66;
      }
      break;
    }
    while ( 1 )
    {
      if ( v7 == 32 )
      {
        ++v21;
        goto LABEL_63;
      }
      if ( v7 != 9 )
        break;
      v21 += qword_2D2A8 - v21 % qword_2D2A8;
LABEL_63:
      v14 = (unsigned char *)v3++;
      v7 = *v14;
    }
    if ( v18 != v21 )
      return 1;
LABEL_66:
    if ( byte_2D1EE )
    {
      v5 = tolower(v5);
      v7 = tolower(v7);
    }
    if ( v5 == v7 )
    {
LABEL_69:
      if ( v5 == 10 )
        return 0;
      if ( v5 == 9 )
        v15 = qword_2D2A8 - v18 % qword_2D2A8;
      else
        v15 = 1;
      v18 += v15;
      continue;
    }
    return 1;
  }
}



// Function: find_change @ 0x12437
long long find_change(long long a1)
{
  return a1;
}



// Function: find_reverse_change @ 0x12449
long long find_reverse_change(long long a1)
{
  return a1;
}



// Function: print_script @ 0x1245b
long long *print_script(long long *a1, long long (*a2)(long long *), void (*a3)(long long *))
{
  long long *result; // rax
  long long *i; // [rsp+28h] [rbp-18h]
  long long *v6; // [rsp+30h] [rbp-10h]
  long long *v7; // [rsp+38h] [rbp-8h]

  result = a1;
  for ( i = a1; i; *v7 = i )
  {
    v6 = i;
    v7 = (long long *)a2(i);
    i = (long long *)*v7;
    *v7 = 0;
    a3(v6);
    result = v7;
  }
  return result;
}



// Function: print_1_line @ 0x124cf
long long print_1_line(long long a1, long long a2)
{
  return sub_124FE(a1, a2, 0);
}



// Function: print_1_line_nl @ 0x124fe
int print_1_line_nl(char *a1, long long *a2, char a3)
{
  char *v3; // rax
  int v4; // eax
  int result; // eax
  char *v6; // rax
  char *v8; // [rsp+20h] [rbp-30h]
  char *format; // [rsp+28h] [rbp-28h]
  char *v10; // [rsp+30h] [rbp-20h]
  char *v11; // [rsp+38h] [rbp-18h]
  long long v12; // [rsp+40h] [rbp-10h]
  FILE *stream; // [rsp+48h] [rbp-8h]

  v11 = (char *)*a2;
  v12 = a2[1];
  stream = ::stream;
  v8 = 0;
  if ( a1 && *a1 )
  {
    if ( byte_2D2B0 )
      v3 = (char *)&unk_25740;
    else
      v3 = (char *)&unk_25744;
    v8 = v3;
    format = v3;
    v10 = a1;
    if ( byte_2D2B1 && *(char *)*a2 == 10 )
    {
      format = "%s";
      v10 = &a1[*a1 == 32];
    }
    fprintf(::stream, format, v10);
  }
  v4 = a3 && *(char *)(v12 - 1) == 10;
  sub_1267D(v11, v12 - v4, v8, a1);
  if ( !a1 || (result = (unsigned char)*a1, (char)result) )
  {
    result = *(unsigned char *)(v12 - 1);
    if ( (char)result != 10 )
    {
      sub_128C2(3);
      v6 = gettext("No newline at end of file");
      return fprintf(stream, "\n\\ %s\n", v6);
    }
  }
  return result;
}



// Function: output_1_line @ 0x1267d
int output_1_line(char *a1, unsigned long long a2, const char *a3, long long a4)
{
  size_t v4; // rax
  size_t v5; // rax
  FILE *v6; // r13
  char *v7; // r12
  unsigned long long v8; // rbx
  unsigned char v9; // r14
  char *ptr; // [rsp+18h] [rbp-68h]
  unsigned long long v14; // [rsp+28h] [rbp-58h]
  long long v15; // [rsp+30h] [rbp-50h]
  unsigned long long v16; // [rsp+38h] [rbp-48h]
  unsigned long long v17; // [rsp+48h] [rbp-38h]
  size_t n; // [rsp+50h] [rbp-30h]

  ptr = a1;
  if ( byte_2D2A1 == 1 )
  {
    v6 = stream;
    v7 = a1;
    v8 = 0;
    LODWORD(v4) = qword_2D2A8;
    v17 = qword_2D2A8;
    v15 = 0;
    while ( 1 )
    {
      while ( 1 )
      {
        while ( 1 )
        {
          if ( (unsigned long long)v7 >= a2 )
            return v4;
          if ( ++v15 == 1024 )
          {
            sub_10D14();
            v15 = 0;
          }
          v4 = (size_t)v7++;
          v9 = *(char *)v4;
          LODWORD(v4) = v9;
          if ( v9 != 13 )
            break;
          LODWORD(v4) = putc_unlocked(v9, v6);
          if ( a3 )
          {
            if ( (unsigned long long)v7 < a2 )
            {
              LODWORD(v4) = (unsigned char)*v7;
              if ( (char)v4 != 10 )
                LODWORD(v4) = fprintf(v6, a3, a4);
            }
          }
          v8 = 0;
        }
        if ( v9 <= 0xDu )
          break;
LABEL_26:
        v8 += ((*__ctype_b_loc())[v9] & 0x4000) != 0;
        LODWORD(v4) = putc_unlocked(v9, v6);
      }
      if ( v9 == 8 )
      {
        if ( v8 )
        {
          --v8;
          LODWORD(v4) = putc_unlocked(v9, v6);
        }
      }
      else
      {
        if ( v9 != 9 )
          goto LABEL_26;
        v16 = v17 - v8 % v17;
        v8 += v16;
        do
        {
          LODWORD(v4) = putc_unlocked(32, v6);
          --v16;
        }
        while ( v16 );
      }
    }
  }
  LODWORD(v4) = a2 - (int)a1;
  v14 = a2 - (long long)a1;
  while ( v14 )
  {
    v5 = 1024;
    if ( v14 <= 0x400 )
      v5 = v14;
    n = v5;
    v4 = fwrite_unlocked(ptr, 1u, v5, stream);
    if ( v4 < n )
      break;
    ptr += v4;
    v14 -= v4;
    LODWORD(v4) = sub_10D14();
  }
  return v4;
}



// Function: put_indicator @ 0x12889
size_t put_indicator(long long a1)
{
  return fwrite_unlocked(*(const void **)(a1 + 8), *(long long *)a1, 1u, stream);
}



// Function: set_color_context @ 0x128c2
long long set_color_context(unsigned int a1)
{
  long long result; // rax

  if ( a1 != 3 )
    sub_10D14();
  result = (unsigned char)byte_2D859;
  if ( byte_2D859 )
  {
    result = (unsigned int)dword_2D010;
    if ( a1 != dword_2D010 )
    {
      sub_12889((long long)&unk_2D040);
      switch ( a1 )
      {
        case 0u:
          sub_12889((long long)&unk_2D080);
          break;
        case 1u:
          sub_12889((long long)&unk_2D090);
          break;
        case 2u:
          sub_12889((long long)&unk_2D0A0);
          break;
        case 3u:
          sub_12889((long long)&unk_2D070);
          break;
        case 4u:
          sub_12889((long long)&unk_2D0B0);
          break;
        default:
          abort();
      }
      sub_12889((long long)&unk_2D050);
      dword_2D010 = a1;
      return a1;
    }
  }
  return result;
}



// Function: translate_line_number @ 0x129a8
long long translate_line_number(long long a1, long long a2)
{
  return *(long long *)(a1 + 232) + a2 + 1;
}



// Function: translate_range @ 0x129d0
long long *translate_range(long long a1, long long a2, long long a3, long long *a4, long long *a5)
{
  long long v5; // rdx

  *a4 = sub_129A8(a1, a2 - 1) + 1;
  v5 = sub_129A8(a1, a3 + 1) - 1;
  *a5 = v5;
  return a5;
}



// Function: print_number_range @ 0x12a37
unsigned long long print_number_range(char a1, long long a2, long long a3, long long a4)
{
  long long v5; // [rsp+28h] [rbp-18h] BYREF
  long long v6; // [rsp+30h] [rbp-10h] BYREF
  unsigned long long v7; // [rsp+38h] [rbp-8h]

  v7 = __readfsqword(0x28u);
  sub_129D0(a2, a3, a4, &v5, &v6);
  if ( v6 <= v5 )
    fprintf(stream, "%ld", v6);
  else
    fprintf(stream, "%ld%c%ld", v5, (unsigned int)a1, v6);
  return v7 - __readfsqword(0x28u);
}



// Function: analyze_hunk @ 0x12af4
long long analyze_hunk(long long *a1, long long *a2, long long *a3, long long *a4, long long *a5)
{
  bool v5; // al
  bool v6; // al
  bool v7; // al
  int v9; // edx
  bool v12; // [rsp+35h] [rbp-9Bh]
  bool v13; // [rsp+36h] [rbp-9Ah]
  bool v14; // [rsp+37h] [rbp-99h]
  long long *v15; // [rsp+38h] [rbp-98h]
  long long v16; // [rsp+40h] [rbp-90h]
  long long v17; // [rsp+48h] [rbp-88h]
  long long i; // [rsp+50h] [rbp-80h]
  long long j; // [rsp+50h] [rbp-80h]
  char *v20; // [rsp+58h] [rbp-78h]
  char *v21; // [rsp+60h] [rbp-70h]
  long long v22; // [rsp+68h] [rbp-68h]
  long long v23; // [rsp+70h] [rbp-60h]
  long long v24; // [rsp+78h] [rbp-58h]
  long long v25; // [rsp+80h] [rbp-50h]
  long long v26; // [rsp+88h] [rbp-48h]
  char *string; // [rsp+90h] [rbp-40h]
  char *v28; // [rsp+98h] [rbp-38h]
  char *v29; // [rsp+A0h] [rbp-30h]
  char *v30; // [rsp+B0h] [rbp-20h]
  char *v31; // [rsp+B8h] [rbp-18h]
  char *v32; // [rsp+C0h] [rbp-10h]

  v5 = byte_2D1EC || stru_2D260.fastmap;
  v12 = v5;
  v22 = (unsigned char)byte_2D1EC - 1;
  v6 = byte_2D1EC && (unsigned int)dword_2D1E8 > 1;
  v13 = v6;
  v7 = v6 && (unsigned int)dword_2D1E8 > 3;
  v14 = v7;
  v23 = qword_2D418;
  v24 = qword_2D548;
  v17 = 0;
  v16 = 0;
  *a2 = a1[3];
  *a4 = a1[4];
  v15 = a1;
  do
  {
    v25 = v15[3] + v15[2] - 1LL;
    v26 = v15[4] + v15[1] - 1LL;
    v16 += v15[2];
    v17 += v15[1];
    for ( i = v15[3]; i <= v25 && v12; ++i )
    {
      string = *(char **)(8 * i + v23);
      v28 = (char *)(*(long long *)(8 * (i + 1) + v23) - 1LL);
      v29 = &v28[*v28 != 10];
      v20 = string;
      if ( v13 )
      {
        while ( *v20 != 10 )
        {
          if ( ((*__ctype_b_loc())[(unsigned char)*v20] & 0x2000) == 0 )
          {
            if ( !v14 )
              v20 = string;
            break;
          }
          ++v20;
        }
      }
      if ( v29 - v20 != v22
        && (!stru_2D260.fastmap
         || re_search(&stru_2D260, string, (int)v29 - (int)string, 0, (int)v29 - (int)string, 0) < 0) )
      {
        v12 = 0;
      }
    }
    for ( j = v15[4]; j <= v26 && v12; ++j )
    {
      v30 = *(char **)(8 * j + v24);
      v31 = (char *)(*(long long *)(8 * (j + 1) + v24) - 1LL);
      v32 = &v31[*v31 != 10];
      v21 = v30;
      if ( v13 )
      {
        while ( *v21 != 10 )
        {
          if ( ((*__ctype_b_loc())[(unsigned char)*v21] & 0x2000) == 0 )
          {
            if ( !v14 )
              v21 = v30;
            break;
          }
          ++v21;
        }
      }
      if ( v32 - v21 != v22
        && (!stru_2D260.fastmap
         || re_search(&stru_2D260, v30, (int)v32 - (int)v30, 0, (int)v32 - (int)v30, 0) < 0) )
      {
        v12 = 0;
      }
    }
    v15 = (long long *)*v15;
  }
  while ( v15 );
  *a3 = v25;
  *a5 = v26;
  if ( v12 )
    return 0;
  if ( v17 )
    v9 = 2;
  else
    v9 = 0;
  return v9 | (unsigned int)(v16 != 0);
}



// Function: concat @ 0x12fa9
char *concat(const char *a1, const char *a2, const char *a3)
{
  size_t v3; // rbx
  size_t v4; // rbx
  size_t v5; // rax
  char *v8; // [rsp+28h] [rbp-18h]

  v3 = strlen(a1);
  v4 = strlen(a2) + v3;
  v5 = strlen(a3);
  v8 = (char *)sub_1C945(v4 + v5 + 1);
  sprintf(v8, "%s%s%s", a1, a2, a3);
  return v8;
}



// Function: zalloc @ 0x13030
void *zalloc(size_t a1)
{
  void *s; // [rsp+18h] [rbp-8h]

  s = (void *)sub_1C945(a1);
  memset(s, 0, a1);
  return s;
}



// Function: debug_script @ 0x1306b
int debug_script(long long *a1)
{
  fflush_unlocked(stdout);
  while ( a1 )
  {
    fprintf(stderr, "%3ld %3ld delete %ld insert %ld\n", a1[3], a1[4], a1[2], a1[1]);
    a1 = (long long *)*a1;
  }
  return fflush_unlocked(stderr);
}


