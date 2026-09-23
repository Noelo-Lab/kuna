// Function: dfaerror @ 0x513f
long long dfaerror(const char *a1)
{
  error(2, 0, "%s", a1);
  return sub_5174();
}



// Function: dfawarn @ 0x5174
void dfawarn(long long a1)
{
  char *v1; // rax

  v1 = gettext("warning: %s");
  error(0, 0, v1, a1);
}



// Function: kwsmusts @ 0x51b4
long long kwsmusts(long long *a1)
{
  long long result; // rax
  size_t v2; // rax
  long long v3; // [rsp+18h] [rbp-28h]
  size_t n; // [rsp+20h] [rbp-20h]
  size_t v5; // [rsp+28h] [rbp-18h]
  char *ptr; // [rsp+30h] [rbp-10h]
  char *dest; // [rsp+38h] [rbp-8h]

  result = sub_1AE2F(a1[1]);
  v3 = result;
  if ( result )
  {
    *a1 = sub_F7C8(0);
    if ( *(char *)v3 )
    {
      ++a1[7];
      n = strlen((const char *)(v3 + 3));
      v5 = n + *(unsigned char *)(v3 + 1) + *(unsigned char *)(v3 + 2);
      ptr = (char *)sub_26B10(v5);
      *ptr = byte_33183;
      dest = &ptr[*(unsigned char *)(v3 + 1)];
      *((char *)a1 + 64) = (unsigned char)(*((char *)a1 + 64) | *(char *)(v3 + 1)) != 0;
      memcpy(dest, (const void *)(v3 + 3), n);
      if ( *(char *)(v3 + 2) )
        dest[n] = byte_33183;
      sub_CACB(*a1, ptr, v5);
      free(ptr);
    }
    else
    {
      v2 = strlen((const char *)(v3 + 3));
      sub_CACB(*a1, v3 + 3, v2);
    }
    sub_D9B5(*a1);
    return sub_1BB28(v3);
  }
  return result;
}



// Function: possible_backrefs_in_pattern @ 0x5354
long long possible_backrefs_in_pattern(char *a1, long long a2, char a3)
{
  int v3; // eax
  char *i; // rax
  size_t v6; // [rsp+10h] [rbp-30h]
  int v7; // [rsp+2Ch] [rbp-14h]
  char *s; // [rsp+30h] [rbp-10h]
  char *v9; // [rsp+38h] [rbp-8h]

  if ( a3 )
    v3 = 92;
  else
    v3 = 128;
  v7 = v3;
  v6 = a2 - 1;
  if ( a2 - 1 >= 0 )
  {
    v9 = &a1[v6];
    for ( i = (char *)memchr(a1, 92, v6); ; i = (char *)memchr(s + 1, 92, v9 - (s + 1)) )
    {
      s = i;
      if ( !i )
        break;
      if ( i[1] > 48 && i[1] <= 57 )
        return 1;
      if ( v7 == i[1] )
      {
        s = i + 1;
        if ( i + 1 == v9 )
          return 0;
      }
    }
  }
  return 0;
}



// Function: regex_compile @ 0x5428
long long regex_compile(
        long long a1,
        const char *a2,
        size_t a3,
        long long a4,
        long long a5,
        reg_syntax_t a6,
        unsigned char a7)
{
  char *v7; // rax
  unsigned char **v8; // rax
  unsigned long long allocated; // rbx
  reg_syntax_t v10; // rbx
  unsigned char *translate; // rbx
  unsigned char *v12; // rbx
  const char *v14; // rax
  ptrdiff_t v19; // [rsp+48h] [rbp-88h] BYREF
  long long v20; // [rsp+50h] [rbp-80h]
  const char *v21; // [rsp+58h] [rbp-78h]
  const char *v22; // [rsp+60h] [rbp-70h]
  ptrdiff_t v23; // [rsp+68h] [rbp-68h]
  struct re_pattern_buffer buffer; // [rsp+70h] [rbp-60h] BYREF
  unsigned long long v25; // [rsp+B8h] [rbp-18h]

  v25 = __readfsqword(0x28u);
  buffer.buffer = 0;
  buffer.allocated = 0;
  v20 = 255;
  if ( a7 | (unsigned char)byte_33180 )
    v7 = 0;
  else
    v7 = (char *)sub_26B10(v20 + 1);
  buffer.fastmap = v7;
  buffer.translate = 0;
  if ( a7 )
    re_set_syntax(a6 | 0x2000000);
  else
    re_set_syntax(a6);
  v21 = re_compile_pattern(a2, a3, &buffer);
  if ( v21 )
  {
    free(buffer.fastmap);
    if ( a5 < 0 )
      v14 = (const char *)&unk_2901F;
    else
      v14 = (const char *)sub_6A2E(a5, &v19);
    v22 = v14;
    if ( *v14 )
    {
      v23 = v19;
      error(0, 0, "%s:%td: %s", v22, v19, v21);
    }
    else
    {
      error(0, 0, "%s", v21);
    }
    return 0;
  }
  else
  {
    if ( a7 )
    {
      regfree(&buffer);
    }
    else
    {
      v8 = (unsigned char **)(*(long long *)(a1 + 16) + (a4 << 6));
      allocated = buffer.allocated;
      *v8 = buffer.buffer;
      v8[1] = (unsigned char *)allocated;
      v10 = buffer.syntax;
      v8[2] = (unsigned char *)buffer.used;
      v8[3] = (unsigned char *)v10;
      translate = buffer.translate;
      v8[4] = (unsigned char *)buffer.fastmap;
      v8[5] = translate;
      v12 = (unsigned char *)*((long long *)&buffer + 7);
      v8[6] = (unsigned char *)buffer.re_nsub;
      v8[7] = v12;
    }
    return 1;
  }
}



// Function: GEAcompile @ 0x5661
long long *GEAcompile(char *a1, size_t a2, reg_syntax_t a3, char a4)
{
  int v4; // eax
  long long v5; // rax
  const char *v6; // rax
  const char *v7; // rax
  size_t v8; // rax
  char *v12; // [rsp+18h] [rbp-B8h]
  char v13; // [rsp+25h] [rbp-ABh]
  char v14; // [rsp+26h] [rbp-AAh]
  char v15; // [rsp+27h] [rbp-A9h]
  long long v16; // [rsp+30h] [rbp-A0h] BYREF
  long long v17; // [rsp+38h] [rbp-98h] BYREF
  void *v18; // [rsp+40h] [rbp-90h]
  void *s; // [rsp+48h] [rbp-88h]
  void *src; // [rsp+50h] [rbp-80h]
  void *ptr; // [rsp+58h] [rbp-78h]
  size_t v22; // [rsp+60h] [rbp-70h]
  long long v23; // [rsp+68h] [rbp-68h]
  long long *v24; // [rsp+70h] [rbp-60h]
  unsigned long long v25; // [rsp+78h] [rbp-58h]
  char *v26; // [rsp+80h] [rbp-50h]
  size_t v27; // [rsp+88h] [rbp-48h]
  size_t n; // [rsp+90h] [rbp-40h]
  signed long long v29; // [rsp+98h] [rbp-38h]
  long long v30; // [rsp+A0h] [rbp-30h]
  size_t v31; // [rsp+A8h] [rbp-28h]
  long long v32; // [rsp+B0h] [rbp-20h]
  char *dest; // [rsp+B8h] [rbp-18h]
  size_t v34; // [rsp+C0h] [rbp-10h]
  unsigned long long v35; // [rsp+C8h] [rbp-8h]

  v12 = a1;
  v35 = __readfsqword(0x28u);
  v24 = (long long *)sub_26FC8(1, 72);
  v24[1] = sub_1BB47();
  if ( byte_33180 )
    a3 |= 0x400000uLL;
  if ( byte_33183 )
    v4 = 0;
  else
    v4 = 2;
  sub_1BB5B(v24[1], &byte_331A0, a3, a3 & 0x10 | 0x2C | (unsigned int)v4);
  v14 = ((unsigned char)byte_331A0 ^ 1 | (unsigned char)byte_331A2) != 0;
  s = a1;
  v25 = (unsigned long long)&a1[a2];
  v13 = 0;
  v24[2] = sub_26AEA(64);
  v24[2] += 64LL;
  v24[3] = 0;
  v16 = 1;
  src = a1;
  ptr = 0;
  v22 = 0;
  v17 = 0;
  v23 = 0;
  do
  {
    v26 = (char *)rawmemchr(s, 10);
    v27 = v26 - (char *)s;
    v15 = sub_5354((char *)s, v26 - (char *)s, v14);
    if ( v15 && src < s )
    {
      n = (char *)s - (char *)src;
      v29 = v22 - v17 + (char *)s - (char *)src;
      if ( v29 > 0 )
        ptr = (void *)sub_26DC6(ptr, &v17, v29, -1, 1);
      memcpy((char *)ptr + v22, src, n);
      v22 += n;
    }
    v30 = v24[3] - v16 + 2;
    if ( v30 > 0 )
    {
      v5 = sub_26DC6(v24[2] - 64LL, &v16, v30, -1, 64);
      v24[2] = v5;
      v24[2] += 64LL;
    }
    re_set_syntax(a3);
    if ( (unsigned char)sub_5428((long long)v24, (const char *)s, v27, v24[3], v23, a3, v15 == 0) != 1 )
      v13 = 1;
    s = v26 + 1;
    ++v23;
    if ( v15 )
    {
      ++v24[3];
      src = s;
    }
  }
  while ( (unsigned long long)s <= v25 );
  if ( v13 )
    exit(2);
  if ( (unsigned long long)src <= v25 )
  {
    if ( a1 >= src )
    {
      ptr = a1;
      v22 = a2;
    }
    else
    {
      v31 = v25 - (long long)src;
      ptr = (void *)sub_26B9F(ptr, v25 - (long long)src + v22);
      memcpy((char *)ptr + v22, src, v31);
      v22 += v31;
    }
  }
  if ( byte_33181 || byte_33182 )
  {
    v32 = 45;
    dest = (char *)sub_26B10(a2 + 45);
    if ( byte_33182 )
    {
      if ( (a3 & 0x2000) != 0 )
        v6 = "^(";
      else
        v6 = "^\\(";
    }
    else if ( (a3 & 0x2000) != 0 )
    {
      v6 = "(^|[^[:alnum:]_])(";
    }
    else
    {
      v6 = "\\(^\\|[^[:alnum:]_]\\)\\(";
    }
    strcpy(dest, v6);
    v34 = strlen(dest);
    memcpy(&dest[v34], a1, a2);
    v34 += a2;
    if ( byte_33182 )
    {
      if ( (a3 & 0x2000) != 0 )
        v7 = ")$";
      else
        v7 = "\\)$";
    }
    else if ( (a3 & 0x2000) != 0 )
    {
      v7 = ")([^[:alnum:]_]|$)";
    }
    else
    {
      v7 = "\\)\\([^[:alnum:]_]\\|$\\)";
    }
    strcpy(&dest[v34], v7);
    v8 = strlen(&dest[v34]);
    v34 += v8;
    v18 = dest;
    v12 = dest;
    sub_14B84(dest, v34, v24[1]);
  }
  else
  {
    v18 = 0;
    sub_14B84(a1, a2, v24[1]);
  }
  sub_51B4(v24);
  sub_1A490(0, 0, v24[1], 1);
  if ( ptr )
  {
    if ( a4 || (unsigned char)sub_19E86(v24[1]) != 1 )
    {
      v24[2] -= 64LL;
      ++v24[3];
      if ( (unsigned char)sub_5428((long long)v24, (const char *)ptr, v22, 0, -1, a3, 0) != 1 )
        abort();
    }
    if ( ptr != v12 )
      free(ptr);
  }
  free(v18);
  return v24;
}



// Function: EGexecute @ 0x5d23
long long EGexecute(long long a1, char *a2, long long a3, long long *a4, char *a5)
{
  char *v6; // rax
  long long v7; // rdx
  long long v8; // rdx
  signed long long v9; // rax
  char v13; // [rsp+3Ch] [rbp-B4h] BYREF
  bool v14; // [rsp+3Dh] [rbp-B3h]
  char v15; // [rsp+3Eh] [rbp-B2h]
  char v16; // [rsp+3Fh] [rbp-B1h]
  int v17; // [rsp+40h] [rbp-B0h]
  int v18; // [rsp+44h] [rbp-ACh]
  char *v19; // [rsp+48h] [rbp-A8h] BYREF
  long long v20; // [rsp+50h] [rbp-A0h] BYREF
  char *string; // [rsp+58h] [rbp-98h]
  char *v22; // [rsp+60h] [rbp-90h]
  char *v23; // [rsp+68h] [rbp-88h]
  void *v24; // [rsp+70h] [rbp-80h]
  char *v25; // [rsp+78h] [rbp-78h]
  long long v26; // [rsp+80h] [rbp-70h]
  long long v27; // [rsp+88h] [rbp-68h]
  long long i; // [rsp+90h] [rbp-60h]
  char *v29; // [rsp+98h] [rbp-58h]
  long long v30; // [rsp+A0h] [rbp-50h]
  long long v31; // [rsp+A8h] [rbp-48h]
  char *v32; // [rsp+B0h] [rbp-40h]
  long long v33; // [rsp+B8h] [rbp-38h]
  char *v34; // [rsp+C0h] [rbp-30h]
  void *v35; // [rsp+C8h] [rbp-28h]
  long long v36[4]; // [rsp+D0h] [rbp-20h] BYREF

  v36[3] = __readfsqword(0x28u);
  v15 = byte_33183;
  v30 = a1;
  v31 = sub_19DA3(*(long long *)(a1 + 8));
  v16 = sub_19DBC(*(long long *)(a1 + 8));
  v19 = a2;
  v32 = &a2[a3];
  v22 = a2;
  for ( string = a2; ; string = v22 )
  {
    if ( v22 >= v32 )
      return -1;
    v22 = v32;
    if ( a5 )
    {
      v23 = a5;
      goto LABEL_42;
    }
    v29 = string;
    v20 = 0;
    v14 = 0;
    v13 = 0;
    if ( *(long long *)v30 )
    {
      v33 = sub_F585(
              *(long long *)v30,
              &string[-*(unsigned char *)(v30 + 64)],
              v32 - string + *(unsigned char *)(v30 + 64),
              v36,
              1);
      if ( v33 < 0 )
        return v33;
      v24 = &string[v33];
      v34 = string;
      string = (char *)memrchr(a2, v15, &string[v33] - a2);
      v6 = string ? string + 1 : a2;
      string = v6;
      v29 = v6;
      v14 = v36[0] < *(long long *)(v30 + 56);
      if ( v14 || v16 != 1 )
        goto LABEL_14;
      v7 = 16;
      if ( (char *)v24 - string >= 16 )
        v7 = (char *)v24 - string;
      if ( v7 < ((char *)v24 - v34) >> 2 )
      {
LABEL_14:
        v22 = (char *)rawmemchr(v24, v15) + 1;
      }
      else
      {
        v8 = 16;
        if ( (char *)v24 - string >= 16 )
          v8 = (char *)v24 - string;
        if ( v8 >= (v32 - v34) >> 2 )
        {
          v22 = v32;
        }
        else
        {
          v9 = (char *)v24 - string;
          if ( (char *)v24 - string < 16 )
            v9 = 16;
          v22 = (char *)rawmemchr(&v34[4 * v9], v15) + 1;
        }
      }
      if ( v14 )
      {
        if ( (unsigned char)byte_331A0 ^ 1 | (unsigned char)byte_331A2 )
          break;
        if ( string > v19 )
          v19 = string;
        if ( !sub_F84A(&v19, 0, v24, v32) )
          break;
        v29 = v19;
      }
    }
    if ( v31 && !v14 )
    {
      v35 = (void *)sub_19D4D(v31, v29, v22, 0, &v20, 0);
      if ( !v35 || v35 == v22 )
        continue;
      if ( v20 )
      {
        string = (char *)memrchr(a2, v15, (char *)v35 - a2) + 1;
        v29 = string;
      }
      v22 = (char *)rawmemchr(v35, v15) + 1;
      v20 = 0;
    }
    v35 = (void *)sub_19D4D(*(long long *)(v30 + 8), v29, v22, 0, &v20, &v13);
    if ( !v35 || v35 == v22 )
      continue;
    if ( v20 )
      string = (char *)memrchr(a2, v15, (char *)v35 - a2) + 1;
    v22 = (char *)rawmemchr(v35, v15) + 1;
    if ( v13 != 1 )
      break;
    v23 = string;
LABEL_42:
    if ( v22 - string > 0x80000000LL )
      sub_27122();
    v25 = v22;
    v27 = 0;
    for ( i = 0; i < *(long long *)(v30 + 24); ++i )
    {
      *(char *)(*(long long *)(v30 + 16) + (i << 6) + 56) &= ~0x40u;
      *(char *)(*(long long *)(v30 + 16) + (i << 6) + 56) = ((byte_33183 == 10) << 7)
                                                        | *(char *)(*(long long *)(v30 + 16) + (i << 6) + 56) & 0x7F;
      v18 = re_search(
              (struct re_pattern_buffer *)((i << 6) + *(long long *)(v30 + 16)),
              string,
              (int)v22 - (int)string - 1,
              (int)v23 - (int)string,
              (int)v22 - (int)v23 - 1,
              (struct re_registers *)(v30 + 32));
      if ( v18 < -1 )
        sub_27122();
      if ( v18 < 0 )
        continue;
      v26 = **(int **)(v30 + 48) - v18;
      v24 = &string[v18];
      if ( v24 > v25 )
        continue;
      if ( !a5 || byte_33181 == 1 )
      {
        if ( (byte_33182 == 1 || byte_33181 == 1) && (!byte_33182 || v26 != v22 - v23 - 1) )
        {
          if ( byte_33182 != 1 && byte_33181 )
          {
            while ( v24 <= v25 )
            {
              v17 = 0;
              if ( !sub_FBE0((char *)v24 + v26, v22 - 1) && !sub_FC0E(string, v24, v22 - 1) )
                goto LABEL_75;
              if ( v26 > 0 )
              {
                --v26;
                *(char *)(*(long long *)(v30 + 16) + (i << 6) + 56) |= 0x40u;
                v17 = re_match(
                        (struct re_pattern_buffer *)((i << 6) + *(long long *)(v30 + 16)),
                        string,
                        v26 + (int)v24 - (int)v23,
                        (int)v24 - (int)string,
                        (struct re_registers *)(v30 + 32));
                if ( v17 < -1 )
                  sub_27122();
              }
              if ( v17 <= 0 )
              {
                if ( v24 == v22 - 1 )
                  goto LABEL_80;
                v24 = (char *)v24 + 1;
                *(char *)(*(long long *)(v30 + 16) + (i << 6) + 56) &= ~0x40u;
                v18 = re_search(
                        (struct re_pattern_buffer *)((i << 6) + *(long long *)(v30 + 16)),
                        string,
                        (int)v22 - (int)string - 1,
                        (int)v24 - (int)string,
                        (int)v22 - (int)v24 - 1,
                        (struct re_registers *)(v30 + 32));
                if ( v18 < 0 )
                {
                  if ( v18 < -1 )
                    sub_27122();
                  goto LABEL_80;
                }
                v26 = **(int **)(v30 + 48) - v18;
                v24 = &string[v18];
              }
              else
              {
                v26 = v17;
              }
            }
          }
          continue;
        }
        v24 = v23;
        v26 = v22 - v23;
      }
LABEL_75:
      if ( !a5 )
        goto LABEL_87;
      if ( v24 < v25 || v24 == v25 && v26 > v27 )
      {
        v25 = (char *)v24;
        v27 = v26;
      }
LABEL_80:
      ;
    }
    if ( v25 < v22 )
    {
      string = v25;
      v26 = v27;
      goto LABEL_88;
    }
  }
LABEL_87:
  v26 = v22 - string;
LABEL_88:
  *a4 = v26;
  return string - a2;
}



// Function: hash_pattern @ 0x6760
unsigned long long hash_pattern(long long a1, unsigned long long a2)
{
  unsigned long long v3; // [rsp+10h] [rbp-30h]
  char *i; // [rsp+18h] [rbp-28h]

  v3 = 0x3938A0565D85CB03LL;
  for ( i = (char *)(qword_33780 + a1 - 1); *i != 10; ++i )
    v3 = (33 * v3) ^ (unsigned char)*i;
  return v3 % a2;
}



// Function: compare_patterns @ 0x67f8
long long compare_patterns(long long a1, long long a2)
{
  char *v3; // [rsp+10h] [rbp-20h]
  char *i; // [rsp+18h] [rbp-18h]

  v3 = (char *)(qword_33780 + a1 - 1);
  for ( i = (char *)(qword_33780 + a2 - 1); *v3 == *i; ++i )
  {
    if ( *v3 == 10 )
      return 1;
    ++v3;
  }
  return 0;
}



// Function: update_patterns @ 0x687b
char *update_patterns(long long a1, long long a2, long long a3, long long a4)
{
  long long v4; // rax
  long long *v5; // rax
  int v8; // [rsp+20h] [rbp-60h]
  int v9; // [rsp+24h] [rbp-5Ch]
  char *dest; // [rsp+28h] [rbp-58h]
  long long v11; // [rsp+30h] [rbp-50h]
  void *s; // [rsp+38h] [rbp-48h]
  unsigned long long v13; // [rsp+40h] [rbp-40h]
  char *v14; // [rsp+48h] [rbp-38h]

  dest = (char *)(a2 + a1);
  v11 = 1;
  v8 = 0;
  v13 = a3 + a1;
  for ( s = (void *)(a2 + a1); (unsigned long long)s < v13; s = v14 + 1 )
  {
    v14 = (char *)rawmemchr(s, 10);
    memmove(dest, s, v14 + 1 - (char *)s);
    v9 = sub_212A6(qword_33790, &dest[-a1 + 1], 0);
    if ( v9 )
    {
      if ( v9 < 0 )
        sub_27122();
      dest += v14 + 1 - (char *)s;
      if ( !v8 )
      {
        if ( qword_33778 == qword_33770 )
          qword_33768 = sub_26DC6(qword_33768, &qword_33770, 1, -1, 24);
        v4 = qword_33778++;
        v5 = (long long *)(qword_33768 + 24 * v4);
        *v5 = qword_33788;
        v5[1] = a4;
        v5[2] = v11;
      }
      ++qword_33788;
    }
    v8 = v9;
    ++v11;
  }
  return &dest[-a1];
}



// Function: pattern_file_name @ 0x6a2e
long long pattern_file_name(long long a1, long long *a2)
{
  long long i; // [rsp+18h] [rbp-8h]

  for ( i = 1; i < qword_33778 && a1 >= *(long long *)(qword_33768 + 24 * i); ++i )
    ;
  *a2 = a1 - *(long long *)(qword_33768 + 24 * i - 24) + *(long long *)(qword_33768 + 24 * i - 24 + 16);
  return *(long long *)(qword_33768 + 24 * i - 24 + 8);
}



// Function: clear_asan_poison @ 0x6b00
void clear_asan_poison()
{
  ;
}



// Function: asan_poison @ 0x6b0b
void asan_poison()
{
  ;
}



// Function: pr_sgr_start @ 0x6b1e
long long pr_sgr_start(unsigned char *a1)
{
  long long result; // rax

  result = *a1;
  if ( (char)result )
    return sub_276E4(off_33068[0], a1);
  return result;
}



// Function: pr_sgr_end @ 0x6b52
long long pr_sgr_end(unsigned char *a1)
{
  long long result; // rax

  result = *a1;
  if ( (char)result )
    return sub_27713(off_33070);
  return result;
}



// Function: pr_sgr_start_if @ 0x6b7f
long long pr_sgr_start_if(unsigned char *a1)
{
  long long result; // rax

  result = (unsigned int)dword_33758;
  if ( dword_33758 )
    return sub_6B1E(a1);
  return result;
}



// Function: pr_sgr_end_if @ 0x6ba8
long long pr_sgr_end_if(unsigned char *a1)
{
  long long result; // rax

  result = (unsigned int)dword_33758;
  if ( dword_33758 )
    return sub_6B52(a1);
  return result;
}



// Function: color_cap_mt_fct @ 0x6bd1
char *color_cap_mt_fct()
{
  off_33030[0] = off_33028[0];
  return off_33028[0];
}



// Function: color_cap_rv_fct @ 0x6bea
void color_cap_rv_fct()
{
  dword_33758 = -1;
}



// Function: color_cap_ne_fct @ 0x6bff
const char *color_cap_ne_fct()
{
  off_33068[0] = "\x1B[%sm";
  off_33070 = "\x1B[m";
  return "\x1B[m";
}



// Function: putchar_errno @ 0x6c26
int putchar_errno(int a1)
{
  int result; // eax

  result = putchar_unlocked(a1);
  if ( result < 0 )
  {
    result = *__errno_location();
    errnum = result;
  }
  return result;
}



// Function: fputs_errno @ 0x6c53
int fputs_errno(const char *a1)
{
  int result; // eax

  result = fputs_unlocked(a1, stdout);
  if ( result < 0 )
  {
    result = *__errno_location();
    errnum = result;
  }
  return result;
}



// Function: printf_errno @ 0x6c8d
unsigned long long printf_errno(const char *a1, long long a2, long long a3, long long a4, long long a5, long long a6, ...)
{
  gcc_va_list arg; // [rsp+10h] [rbp-D0h] BYREF
  unsigned long long v8; // [rsp+28h] [rbp-B8h]
  long long v9; // [rsp+38h] [rbp-A8h]
  long long v10; // [rsp+40h] [rbp-A0h]
  long long v11; // [rsp+48h] [rbp-98h]
  long long v12; // [rsp+50h] [rbp-90h]
  long long v13; // [rsp+58h] [rbp-88h]

  va_start(arg, a6);
  v9 = a2;
  v10 = a3;
  v11 = a4;
  v12 = a5;
  v13 = a6;
  v8 = __readfsqword(0x28u);
  arg[0].gp_offset = 8;
  if ( vfprintf(stdout, a1, arg) < 0 )
    errnum = *__errno_location();
  return v8 - __readfsqword(0x28u);
}



// Function: fwrite_errno @ 0x6d74
size_t fwrite_errno(const void *a1, size_t a2, size_t a3)
{
  size_t result; // rax

  result = fwrite_unlocked(a1, a2, a3, stdout);
  if ( result != a3 )
  {
    result = (unsigned int)*__errno_location();
    errnum = result;
  }
  return result;
}



// Function: fflush_errno @ 0x6dc0
int fflush_errno()
{
  int result; // eax

  result = fflush_unlocked(stdout);
  if ( result )
  {
    result = *__errno_location();
    errnum = result;
  }
  return result;
}



// Function: is_device_mode @ 0x6deb
long long is_device_mode(short a1)
{
  return (a1 & 0xF000) == 0x2000 || (a1 & 0xF000) == 0x6000 || (a1 & 0xF000) == 0xC000 || (a1 & 0xF000) == 0x1000;
}



// Function: skip_devices @ 0x6e43
long long skip_devices(unsigned char a1)
{
  return dword_337CC == 2 || ((dword_337CC == 0) & (a1 ^ 1)) != 0;
}



// Function: usable_st_size @ 0x6e8a
bool usable_st_size(long long a1)
{
  return (*(int *)(a1 + 24) & 0xF000) == 0x8000;
}



// Function: input_filename @ 0x6eac
long long input_filename()
{
  if ( !qword_337C0 )
    qword_337C0 = (long long)gettext("(standard input)");
  return qword_337C0;
}



// Function: suppressible_error @ 0x6edf
void suppressible_error(int a1)
{
  const char *v1; // rax

  if ( byte_33755 != 1 )
  {
    v1 = (const char *)sub_6EAC();
    error(0, a1, "%s", v1);
  }
  byte_337C9 = 1;
}



// Function: clean_up_stdout @ 0x6f2c
long long clean_up_stdout()
{
  long long result; // rax

  result = (unsigned int)errnum;
  if ( !errnum )
    return sub_1143E();
  return result;
}



// Function: initialize_unibyte_mask @ 0x6f46
long long initialize_unibyte_mask()
{
  long long result; // rax
  unsigned char v1; // [rsp+1h] [rbp-9h]
  int v2; // [rsp+2h] [rbp-8h]
  int i; // [rsp+6h] [rbp-4h]

  v1 = 0;
  v2 = 1;
  for ( i = 1; i <= 255; ++i )
  {
    if ( *((char *)&unk_331A3 + i) != 1 && (unsigned char)(i & v1) == 0 )
    {
      while ( i >= 2 * v2 )
        v2 *= 2;
      v1 |= v2;
    }
  }
  result = 0x101010101010101LL * v1;
  qword_337E8 = result;
  return result;
}



// Function: skip_easy_bytes @ 0x6fdd
char *skip_easy_bytes(char *a1)
{
  char *j; // [rsp+18h] [rbp-18h]
  char *i; // [rsp+20h] [rbp-10h]

  while ( ((unsigned char)a1 & 7) != 0 )
  {
    if ( ((unsigned char)sub_F64D((unsigned int)*a1) & (unsigned char)qword_337E8) != 0 )
      return a1;
    ++a1;
  }
  for ( i = a1; (*(long long *)i & qword_337E8) == 0; i += 8 )
    ;
  for ( j = i; ((unsigned char)sub_F64D((unsigned int)*j) & (unsigned char)qword_337E8) == 0; ++j )
    ;
  return j;
}



// Function: buf_has_encoding_errors @ 0x7098
long long buf_has_encoding_errors(char *a1, long long a2)
{
  char *i; // rax
  char *v4; // [rsp+10h] [rbp-20h]
  long long v5; // [rsp+18h] [rbp-18h]
  long long v6[2]; // [rsp+20h] [rbp-10h] BYREF

  v6[1] = __readfsqword(0x28u);
  if ( !qword_337E8 )
    return 0;
  v6[0] = 0;
  a1[a2] = -1;
  for ( i = sub_6FDD(a1); ; i = sub_6FDD(&v4[v5]) )
  {
    v4 = i;
    if ( i >= &a1[a2] )
      break;
    v5 = sub_F686(i, &a1[a2] - i, v6);
    if ( v5 < 0 )
      return 1;
  }
  return 0;
}



// Function: buf_has_nulls @ 0x716c
bool buf_has_nulls(const char *a1, long long a2)
{
  a1[a2] = 0;
  return strlen(a1) != a2;
}



// Function: file_must_have_nulls @ 0x71a6
long long file_must_have_nulls(long long a1, int a2, long long a3)
{
  int *v4; // rax
  long long offset; // [rsp+20h] [rbp-10h]
  __off_t v7; // [rsp+28h] [rbp-8h]

  if ( byte_337D0 == 1 || !sub_6E8A(a3) || a1 >= *(long long *)(a3 + 48) )
    return 0;
  offset = a1;
  if ( !a2 )
  {
    offset = lseek(0, 0, 1);
    if ( offset < 0 )
      return 0;
  }
  v7 = lseek(a2, offset, 4);
  if ( v7 < 0 )
    return 0;
  if ( lseek(a2, offset, 0) < 0 )
  {
    v4 = __errno_location();
    sub_6EDF(*v4);
  }
  return v7 < *(long long *)(a3 + 48);
}



// Function: context_length_arg @ 0x728f
void context_length_arg(const char *a1, long long *a2)
{
  char *v2; // rax

  if ( (unsigned int)sub_27281(a1, 0, 10, a2, &locale) > 1 || *a2 < 0 )
  {
    v2 = gettext("invalid context length argument");
    error(2, 0, "%s: %s", a1, v2);
  }
}



// Function: exclude_options @ 0x7316
long long exclude_options(char a1)
{
  if ( a1 )
    return 0x10000000;
  else
    return 1342177280;
}



// Function: skipped_file @ 0x7337
long long skipped_file(long long a1, unsigned char a2, char a3)
{
  long long *v5; // [rsp+18h] [rbp-8h]

  if ( a3 != 1 )
  {
    v5 = &unk_337A0;
  }
  else
  {
    if ( dword_33010 == 4 )
      return 1;
    if ( a2 && byte_337C8 )
      return 0;
    v5 = &unk_337B0;
  }
  return v5[a2] && (unsigned char)sub_1CBB6(v5[a2], a1);
}



// Function: add_count @ 0x73f5
long long add_count(long long a1, long long a2)
{
  char *v2; // rax

  if ( __OFADD__(a1, a2) )
  {
    v2 = gettext("input is too large to count");
    error(2, 0, v2);
  }
  return a1 + a2;
}



// Function: all_zeros @ 0x747d
long long all_zeros(char *a1, long long a2)
{
  char *i; // [rsp+18h] [rbp-8h]

  for ( i = a1; i < &a1[a2]; ++i )
  {
    if ( *i )
      return 0;
  }
  return 1;
}



// Function: reset @ 0x74c6
long long reset(int a1)
{
  char *v1; // rax
  __off_t v2; // rax
  int *v3; // rax

  if ( ((unsigned long long)ptr + 1) % qword_33818 )
    v1 = (char *)ptr + qword_33818 - ((unsigned long long)ptr + 1) % qword_33818 + 1;
  else
    v1 = (char *)ptr + 1;
  s = v1;
  qword_33808 = (long long)v1;
  *(v1 - 1) = byte_33183;
  fd = a1;
  if ( a1 )
    v2 = 0;
  else
    v2 = lseek(0, 0, 1);
  offset = v2;
  byte_337D0 = v2 < 0;
  byte_337D1 = v2 < 0;
  if ( v2 < 0 )
  {
    if ( *__errno_location() != 29 )
    {
      v3 = __errno_location();
      sub_6EDF(*v3);
      return 0;
    }
    offset = 0;
  }
  return 1;
}



// Function: fillbuf @ 0x7608
long long fillbuf(long long a1, long long a2)
{
  long long v2; // rax
  char *v3; // rax
  unsigned char v5; // [rsp+17h] [rbp-69h]
  char *v6; // [rsp+20h] [rbp-60h]
  char *v7; // [rsp+28h] [rbp-58h]
  long long v8; // [rsp+30h] [rbp-50h]
  long long v9; // [rsp+38h] [rbp-48h]
  __off_t v10; // [rsp+40h] [rbp-40h]
  long long v11; // [rsp+48h] [rbp-38h]
  long long v12; // [rsp+58h] [rbp-28h]
  __off_t v13; // [rsp+60h] [rbp-20h]
  char *v14; // [rsp+70h] [rbp-10h]

  v11 = qword_33818 + 8;
  if ( qword_33818 + 8 > (char *)ptr + qword_337F8 - (char *)s )
  {
    v12 = qword_33818 + a1 - qword_337F8 + v11;
    if ( v12 > 0 )
    {
      v8 = -1;
      if ( sub_6E8A(a2) )
      {
        v13 = *(long long *)(a2 + 48) - offset;
        if ( v13 >= 0 && !__OFADD__(v11 + a1, v13) )
        {
          v2 = v11 + a1 + v13;
          if ( v12 + qword_337F8 >= v2 )
            v2 = v12 + qword_337F8;
          v8 = v2;
        }
      }
      v7 = (char *)sub_26DC6(0, &qword_337F8, v12, v8, 1);
    }
    else
    {
      v7 = (char *)ptr;
    }
    if ( (unsigned long long)&v7[a1 + 1] % qword_33818 )
      v3 = &v7[qword_33818 - (unsigned long long)&v7[a1 + 1] % qword_33818 + 1 + a1];
    else
      v3 = &v7[a1 + 1];
    v6 = v3;
    memmove(&v3[-a1 - 1], (char *)s - a1 - 1, a1 + 1);
    if ( v12 > 0 )
    {
      free(ptr);
      ptr = v7;
    }
  }
  else
  {
    v6 = (char *)s;
  }
  qword_33808 = (long long)&v6[-a1];
  sub_6B00();
  v14 = (char *)((char *)ptr + qword_337F8 - 8 - v6 - ((char *)ptr + qword_337F8 - 8 - v6) % qword_33818);
  v5 = 1;
  while ( 1 )
  {
    v9 = sub_24C3E((unsigned int)fd, v6, v14);
    if ( v9 == -1 )
    {
      v9 = 0;
      v5 = 0;
    }
    offset += v9;
    if ( (v9 == 0) | (unsigned char)byte_33830 ^ 1 || (unsigned char)sub_747D(v6, v9) != 1 )
      break;
    qword_33838 = sub_73F5(qword_33838, v9);
    if ( byte_337D1 != 1 )
    {
      v10 = lseek(fd, offset, 3);
      if ( v10 < 0 && *__errno_location() == 6 && sub_6E8A(a2) && *(long long *)(a2 + 48) > offset )
        v10 = lseek(fd, 0, 2);
      if ( v10 >= 0 )
      {
        qword_33838 = sub_73F5(qword_33838, v10 - offset);
        offset = v10;
      }
      else
      {
        byte_337D1 = 1;
      }
    }
  }
  s = &v6[v9];
  memset(&v6[v9], 0, 8u);
  sub_6B0B();
  return v5;
}



// Function: nlscan @ 0x7a8d
unsigned long long nlscan(unsigned long long a1)
{
  long long v2; // [rsp+10h] [rbp-10h]
  void *s; // [rsp+18h] [rbp-8h]
  char *sa; // [rsp+18h] [rbp-8h]

  v2 = 0;
  for ( s = (void *)qword_33890; (unsigned long long)s < a1; s = sa + 1 )
  {
    sa = (char *)memchr(s, byte_33183, a1 - (long long)s);
    if ( !sa )
      break;
    ++v2;
  }
  qword_33838 = sub_73F5(qword_33838, v2);
  qword_33890 = a1;
  return a1;
}



// Function: print_filename @ 0x7b22
long long print_filename()
{
  const char *v0; // rax

  sub_6B7F((unsigned char *)off_33038[0]);
  v0 = (const char *)sub_6EAC();
  sub_6C53(v0);
  return sub_6BA8((unsigned char *)off_33038[0]);
}



// Function: print_sep @ 0x7b58
long long print_sep(char a1)
{
  sub_6B7F((unsigned char *)off_33050);
  sub_6C26(a1);
  return sub_6BA8((unsigned char *)off_33050);
}



// Function: print_offset @ 0x7b95
long long print_offset(long long a1, unsigned char *a2)
{
  long long v2; // rcx
  long long v3; // r8
  long long v4; // r9

  sub_6B7F(a2);
  sub_6C8D("%*ld", (unsigned int)dword_33760, a1, v2, v3, v4);
  return sub_6BA8(a2);
}



// Function: print_line_head @ 0x7be4
long long print_line_head(char *a1, long long a2, unsigned long long a3, char a4)
{
  char v7; // [rsp+26h] [rbp-Ah]
  char v8; // [rsp+27h] [rbp-9h]
  long long v9; // [rsp+28h] [rbp-8h]

  if ( dword_33840 == 1 || (v7 = a1[a2], v8 = sub_7098(a1, a2), a1[a2] = v7, !v8) )
  {
    if ( dword_33848 )
    {
      sub_7B22();
      if ( dword_3384C )
        sub_7B58(a4);
      else
        sub_6C26(0);
    }
    if ( byte_33852 )
    {
      if ( a3 > qword_33890 )
      {
        sub_7A8D((unsigned long long)a1);
        qword_33838 = sub_73F5(qword_33838, 1);
        qword_33890 = a3;
      }
      sub_7B95(qword_33838, (unsigned char *)off_33040[0]);
      sub_7B58(a4);
    }
    if ( byte_33853 )
    {
      v9 = sub_73F5(qword_33888, (long long)&a1[-qword_33808]);
      sub_7B95(v9, (unsigned char *)off_33048[0]);
      sub_7B58(a4);
    }
    if ( byte_3375D && dword_33848 | (unsigned char)byte_33852 | (unsigned char)byte_33853 )
    {
      if ( a2 )
        sub_6C26(9);
    }
    return 1;
  }
  else
  {
    byte_337CA = 1;
    return 0;
  }
}



// Function: print_line_middle @ 0x7d91
char *print_line_middle(char *a1, unsigned long long a2, unsigned char *a3, unsigned char *a4)
{
  char v4; // al
  size_t v8; // [rsp+30h] [rbp-30h] BYREF
  char *i; // [rsp+38h] [rbp-28h]
  char *v10; // [rsp+40h] [rbp-20h]
  long long v11; // [rsp+48h] [rbp-18h]
  char *v12; // [rsp+50h] [rbp-10h]
  unsigned long long v13; // [rsp+58h] [rbp-8h]

  v13 = __readfsqword(0x28u);
  v10 = 0;
  for ( i = a1; (unsigned long long)i < a2; i = &v12[v8] )
  {
    v11 = qword_337D8(qword_337E0, a1, a2 - (long long)a1, &v8, i);
    if ( v11 < 0 )
      break;
    v12 = &a1[v11];
    if ( &a1[v11] == (char *)a2 )
      break;
    if ( v8 )
    {
      if ( byte_3375C )
      {
        if ( byte_33851 )
          v4 = 45;
        else
          v4 = 58;
        if ( (unsigned char)sub_7BE4(v12, v8, a2, v4) != 1 )
          return 0;
      }
      else
      {
        sub_6B1E(a3);
        if ( v10 )
        {
          i = v10;
          v10 = 0;
        }
        sub_6D74(i, 1u, v12 - i);
      }
      sub_6B7F(a4);
      sub_6D74(v12, 1u, v8);
      sub_6BA8(a4);
      if ( byte_3375C )
        sub_6C26(byte_33183);
    }
    else
    {
      v8 = 1;
      if ( !v10 )
        v10 = i;
    }
  }
  if ( byte_3375C )
    return (char *)a2;
  if ( v10 )
    return v10;
  return i;
}



// Function: print_line_tail @ 0x7f87
char *print_line_tail(char *a1, unsigned long long a2, unsigned char *a3)
{
  int v3; // eax
  int v4; // eax
  char *v7; // [rsp+18h] [rbp-18h]
  long long v8; // [rsp+20h] [rbp-10h]
  signed long long v9; // [rsp+28h] [rbp-8h]

  v7 = a1;
  v3 = a2 > (unsigned long long)a1 && *(char *)(a2 - 1) == byte_33183;
  v8 = v3;
  v4 = (unsigned long long)a1 < a2 - v3 && *(char *)(~(long long)v3 + a2) == 13;
  v9 = a2 - (v4 + v8) - (long long)a1;
  if ( v9 > 0 )
  {
    sub_6B1E(a3);
    sub_6D74(a1, 1u, v9);
    v7 = &a1[v9];
    sub_6B52(a3);
  }
  return v7;
}



// Function: prline @ 0x806d
char *prline(char *a1, char *a2, char a3)
{
  char *result; // rax
  bool v4; // al
  unsigned char *v5; // rax
  char *v6; // rax
  char *v7; // rax
  char *v9; // [rsp+18h] [rbp-28h]
  unsigned char *v10; // [rsp+30h] [rbp-10h]
  unsigned char *v11; // [rsp+38h] [rbp-8h]

  v9 = a1;
  if ( byte_3375C != 1 )
  {
    result = (char *)((unsigned int)sub_7BE4(a1, a2 - a1 - 1, (unsigned long long)a2, a3) ^ 1);
    if ( (char)result )
      return result;
  }
  if ( dword_33758 )
  {
    v4 = byte_33851 && dword_33758 < 0;
    if ( (a3 == 58) != v4 )
      v5 = (unsigned char *)off_33058;
    else
      v5 = (unsigned char *)off_33060;
    v10 = v5;
    if ( a3 == 58 )
      v6 = off_33028[0];
    else
      v6 = off_33030[0];
    v11 = (unsigned char *)v6;
  }
  else
  {
    v11 = 0;
    v10 = 0;
  }
  if ( byte_3375C && (a3 == 58) != byte_33851 || dword_33758 && (*v10 || *v11) )
  {
    if ( (a3 == 58) != byte_33851 && (byte_3375C || *v11) )
    {
      result = sub_7D91(a1, (unsigned long long)a2, v10, v11);
      v9 = result;
      if ( !result )
        return result;
    }
    if ( byte_3375C != 1 && *v10 )
      v9 = sub_7F87(v9, (unsigned long long)a2, v10);
  }
  if ( byte_3375C != 1 && a2 > v9 )
    sub_6D74(v9, 1u, a2 - v9);
  if ( byte_33878 )
    sub_6DC0();
  if ( errnum )
  {
    v7 = gettext("write error");
    error(2, errnum, v7);
  }
  qword_33898 = a2;
  return a2;
}



// Function: prpending @ 0x828e
void *prpending(unsigned long long a1)
{
  void *result; // rax
  char *v2; // [rsp+18h] [rbp-8h]

  if ( !qword_33898 )
    qword_33898 = (void *)qword_33808;
  while ( 1 )
  {
    result = (void *)qword_338A8;
    if ( qword_338A8 <= 0 )
      break;
    result = qword_33898;
    if ( a1 <= (unsigned long long)qword_33898 )
      break;
    v2 = (char *)rawmemchr(qword_33898, byte_33183);
    sub_806D((char *)qword_33898, v2 + 1, 45);
    --qword_338A8;
  }
  return result;
}



// Function: prtext @ 0x8326
long long prtext(char *a1, char *a2)
{
  char *v2; // rax
  long long v3; // rax
  long long result; // rax
  char v5; // [rsp+1Fh] [rbp-31h]
  char *s; // [rsp+20h] [rbp-30h]
  long long i; // [rsp+28h] [rbp-28h]
  long long j; // [rsp+30h] [rbp-20h]
  char *v9; // [rsp+40h] [rbp-10h]
  char *v10; // [rsp+48h] [rbp-8h]

  v5 = byte_33183;
  if ( byte_33850 != 1 && qword_338A8 > 0 )
    sub_828E((unsigned long long)a1);
  s = a1;
  if ( byte_33850 != 1 )
  {
    if ( qword_33898 )
      v2 = (char *)qword_33898;
    else
      v2 = (char *)qword_33808;
    for ( i = 0; i < qword_33858; ++i )
    {
      if ( s > v2 )
      {
        do
          --s;
        while ( v5 != *(s - 1) );
      }
    }
    if ( (qword_33858 >= 0 || qword_33860 >= 0) && byte_339C0 && s != qword_33898 && off_33020[0] )
    {
      sub_6B7F((unsigned char *)off_33050);
      sub_6C53(off_33020[0]);
      sub_6BA8((unsigned char *)off_33050);
      sub_6C26(10);
    }
    while ( s < a1 )
    {
      v9 = (char *)rawmemchr(s, v5) + 1;
      sub_806D(s, v9, 45);
      s = v9;
    }
  }
  if ( byte_33851 )
  {
    for ( j = 0; s < a2 && j < qword_338A0; ++j )
    {
      v10 = (char *)rawmemchr(s, v5) + 1;
      if ( byte_33850 != 1 )
        sub_806D(s, v10, 58);
      s = v10;
    }
  }
  else
  {
    if ( byte_33850 != 1 )
      sub_806D(a1, a2, 58);
    j = 1;
    s = a2;
  }
  qword_33828 = offset - ((char *)::s - s);
  if ( byte_33850 )
  {
    v3 = 0;
  }
  else
  {
    v3 = qword_33860;
    if ( qword_33860 < 0 )
      v3 = 0;
  }
  qword_338A8 = v3;
  byte_339C0 = 1;
  result = qword_338A0 - j;
  qword_338A0 -= j;
  return result;
}



// Function: zap_nuls @ 0x85c1
const char *zap_nuls(const char *a1, char *a2, unsigned int a3)
{
  const char *result; // rax
  char *v4; // rax
  char v5; // [rsp+Ch] [rbp-14h]

  result = (const char *)a3;
  v5 = a3;
  if ( (char)a3 )
  {
    while ( 1 )
    {
      *a2 = 0;
      a1 += strlen(a1);
      *a2 = v5;
      result = a1;
      if ( a1 == a2 )
        break;
      do
      {
        v4 = (char *)a1++;
        *v4 = v5;
      }
      while ( !*a1 );
    }
  }
  return result;
}



// Function: grepbuf @ 0x862e
long long grepbuf(unsigned long long a1, unsigned long long a2)
{
  char *v2; // rax
  char *v3; // rax
  int v4; // eax
  long long v6; // [rsp+18h] [rbp-48h] BYREF
  unsigned long long i; // [rsp+20h] [rbp-40h]
  long long v8; // [rsp+28h] [rbp-38h]
  long long v9; // [rsp+30h] [rbp-30h]
  unsigned long long v10; // [rsp+38h] [rbp-28h]
  unsigned long long v11; // [rsp+40h] [rbp-20h]
  char *v12; // [rsp+48h] [rbp-18h]
  char *v13; // [rsp+50h] [rbp-10h]
  unsigned long long v14; // [rsp+58h] [rbp-8h]

  v14 = __readfsqword(0x28u);
  v9 = qword_338A0;
  for ( i = a1; i < a2; i = v11 )
  {
    v8 = qword_337D8(qword_337E0, i, a2 - i, &v6, 0);
    if ( v8 < 0 )
    {
      if ( byte_33851 != 1 )
        return v9 - qword_338A0;
      v8 = a2 - i;
      v6 = 0;
    }
    v10 = v8 + i;
    v11 = v6 + v8 + i;
    if ( byte_33851 != 1 && v10 == a2 )
      break;
    if ( byte_33851 != 1 || i < v10 )
    {
      v2 = (char *)(byte_33851 ? i : v10);
      v12 = v2;
      v3 = (char *)(byte_33851 ? v10 : v11);
      v13 = v3;
      sub_8326(v12, v3);
      if ( !qword_338A0 || byte_338B0 )
      {
        if ( byte_338B1 )
        {
          if ( byte_337C9 )
            v4 = status;
          else
            v4 = 0;
          exit(v4);
        }
        return v9 - qword_338A0;
      }
    }
  }
  return v9 - qword_338A0;
}



// Function: grep @ 0x87e9
long long grep(int a1, long long a2, char *a3)
{
  bool v4; // al
  int *v5; // rax
  long long v6; // rax
  int v7; // eax
  long long v8; // rax
  int *v9; // rax
  char *v10; // rax
  long long v11; // rbx
  char *v12; // rax
  char v14; // [rsp+2Ah] [rbp-56h]
  char i; // [rsp+2Bh] [rbp-55h]
  char v16; // [rsp+2Ch] [rbp-54h]
  char v17; // [rsp+2Dh] [rbp-53h]
  char v18; // [rsp+2Eh] [rbp-52h]
  char v19; // [rsp+2Fh] [rbp-51h]
  long long v20; // [rsp+30h] [rbp-50h]
  long long v21; // [rsp+38h] [rbp-48h]
  signed long long v22; // [rsp+40h] [rbp-40h]
  long long v23; // [rsp+48h] [rbp-38h]
  const char *v24; // [rsp+50h] [rbp-30h]
  unsigned long long v25; // [rsp+50h] [rbp-30h]
  char *v26; // [rsp+50h] [rbp-30h]
  const char *v27; // [rsp+58h] [rbp-28h]
  long long v28; // [rsp+60h] [rbp-20h]
  long long v29; // [rsp+68h] [rbp-18h]
  long long v30; // [rsp+68h] [rbp-18h]

  v16 = byte_33183;
  v14 = 0;
  v17 = byte_338B0;
  v18 = byte_33850;
  v28 = -1;
  if ( (unsigned char)sub_74C6(a1) != 1 )
    return 0;
  qword_33888 = 0;
  qword_33898 = 0;
  qword_33838 = 0;
  qword_338A0 = qword_33870;
  qword_33828 = 0;
  qword_338A8 = 0;
  v4 = byte_33831 && !v16;
  byte_33830 = v4;
  byte_337CA = 0;
  v20 = 0;
  v22 = 0;
  v23 = 0;
  if ( (unsigned char)sub_7608(0, a2) != 1 )
  {
    v5 = __errno_location();
    sub_6EDF(*v5);
    return 0;
  }
  else
  {
    dword_33760 = 0;
    if ( byte_3375D )
    {
      if ( sub_6E8A(a2) )
        v6 = *(long long *)(a2 + 48);
      else
        v6 = 0x7FFFFFFFFFFFFFFFLL;
      v29 = v6;
      v7 = byte_33852 && v6 != 0x7FFFFFFFFFFFFFFFLL;
      v30 = v7 + v29;
      do
      {
        ++dword_33760;
        v30 /= 10;
      }
      while ( v30 );
    }
    for ( i = 1; ; i = 0 )
    {
      if ( v28 < 0
        && v16
        && dword_33840 != 1
        && (sub_716C((const char *)qword_33808, (long long)s - qword_33808)
         || i && sub_71A6((long long)s - qword_33808, a1, a2)) )
      {
        if ( dword_33840 == 2 )
          return 0;
        if ( byte_33868 != 1 )
        {
          byte_33850 = 1;
          byte_338B0 = 1;
        }
        v28 = v20;
        v14 = v16;
        byte_33830 = byte_33831;
      }
      qword_33890 = qword_33808;
      if ( qword_33898 )
        qword_33898 = (void *)qword_33808;
      v24 = (const char *)(qword_33808 + v23);
      if ( (void *)(qword_33808 + v23) == s )
        break;
      sub_85C1(v24, (char *)s, v14);
      v19 = *(v24 - 1);
      *((char *)v24 - 1) = v16;
      v27 = (char *)memrchr(v24 - 1, v16, (char *)s - v24 + 1) + 1;
      *((char *)v24 - 1) = v19;
      if ( v27 == v24 )
        v27 = &v24[-v22];
      v25 = (unsigned long long)&v24[-v22];
      v22 = (char *)s - v27;
      if ( v25 < (unsigned long long)v27 )
      {
        if ( qword_338A0 )
          v20 += sub_862E(v25, (unsigned long long)v27);
        if ( qword_338A8 )
          sub_828E((unsigned long long)v27);
        if ( !qword_338A0 && !qword_338A8 )
          goto LABEL_68;
        if ( byte_338B0 )
        {
          v8 = v28;
          if ( v28 < 0 )
            v8 = 0;
          if ( v20 > v8 )
            goto LABEL_68;
        }
      }
      v21 = 0;
      v26 = (char *)v27;
      while ( v21 < qword_33858 && (unsigned long long)v26 > qword_33808 && v26 != qword_33898 )
      {
        ++v21;
        do
          --v26;
        while ( v16 != *(v26 - 1) );
      }
      if ( v26 != qword_33898 )
        qword_33898 = 0;
      v23 = &v27[v22] - v26;
      if ( byte_33853 )
        qword_33888 = sub_73F5(qword_33888, (long long)s - qword_33808 - v23);
      if ( byte_33852 )
        sub_7A8D((unsigned long long)v26);
      if ( (unsigned char)sub_7608(v23, a2) != 1 )
      {
        v9 = __errno_location();
        sub_6EDF(*v9);
        goto LABEL_68;
      }
    }
    *a3 = 1;
    if ( v22 )
    {
      v10 = (char *)s;
      s = (char *)s + 1;
      *v10 = v16;
      if ( qword_338A0 )
        v20 += sub_862E(qword_33808 + v23 - v22, (unsigned long long)s);
      if ( qword_338A8 )
        sub_828E((unsigned long long)s);
    }
LABEL_68:
    byte_338B0 = v17;
    byte_33850 = v18;
    if ( !dword_33840 && byte_33850 != 1 && (byte_337CA || v28 >= 0 && v28 < v20) )
    {
      v11 = sub_6EAC();
      v12 = gettext("%s: binary file matches");
      error(0, 0, v12, v11);
    }
    return v20;
  }
}



// Function: grepdirent @ 0x8e14
long long grepdirent(long long a1, long long a2, unsigned char a3)
{
  long long result; // rax
  char v4; // al
  bool v5; // al
  long long v6; // rbx
  char *v7; // rax
  int v8; // eax
  int *v9; // rax
  unsigned char v10; // [rsp+Ch] [rbp-D4h]
  bool v11; // [rsp+23h] [rbp-BDh]
  struct stat *p_buf; // [rsp+28h] [rbp-B8h]
  struct stat buf; // [rsp+30h] [rbp-B0h] BYREF
  unsigned long long v14; // [rsp+C8h] [rbp-18h]

  v14 = __readfsqword(0x28u);
  v10 = (a3 & (*(long long *)(a2 + 88) == 0)) != 0;
  if ( *(short *)(a2 + 104) == 6 )
    return 1;
  if ( (a3 & (*(long long *)(a2 + 88) == 0)) != 0
    || (*(short *)(a2 + 104) != 1 && *(short *)(a2 + 104) != 2 && *(short *)(a2 + 104) != 4 ? (v4 = 0) : (v4 = 1),
        !(unsigned char)sub_7337(a2 + 256, 0, v4 & 1)) )
  {
    qword_337C0 = *(long long *)(a2 + 56);
    if ( byte_337C8 && *(char *)(qword_337C0 + 1) )
      qword_337C0 += 2;
    v5 = (*(int *)(a1 + 72) & 2) != 0 || (*(int *)(a1 + 72) & 1) != 0 && v10;
    v11 = v5;
    switch ( *(short *)(a2 + 104) )
    {
      case 1:
        if ( dword_33010 == 3 )
          return 1;
        sub_1E92F(a1, a2, 4);
        goto LABEL_39;
      case 2:
        if ( byte_33755 != 1 )
        {
          v6 = qword_337C0;
          v7 = gettext("%s: warning: recursive directory loop");
          error(0, 0, v7, v6);
        }
        return 1;
      case 3:
      case 0xB:
        if ( !sub_6E43(v10) )
          goto LABEL_39;
        p_buf = (struct stat *)(a2 + 112);
        if ( *(int *)(a2 + 136) )
          goto LABEL_35;
        if ( v11 )
          v8 = 0;
        else
          v8 = 256;
        if ( fstatat(*(int *)(a1 + 44), *(const char **)(a2 + 48), &buf, v8) )
        {
          v9 = __errno_location();
          sub_6EDF(*v9);
          result = 1;
        }
        else
        {
          p_buf = &buf;
LABEL_35:
          if ( sub_6DEB(p_buf->st_mode) )
            result = 1;
          else
LABEL_39:
            result = sub_91BC(*(unsigned int *)(a1 + 44), *(long long *)(a2 + 48), v11, v10);
        }
        break;
      case 4:
      case 7:
      case 0xA:
        sub_6EDF(*(int *)(a2 + 64));
        return 1;
      case 8:
      case 0xD:
        goto LABEL_39;
      case 0xC:
      case 0xE:
        return 1;
      default:
        abort();
    }
  }
  else
  {
    sub_1E92F(a1, a2, 4);
    return 1;
  }
  return result;
}



// Function: open_symlink_nofollow_error @ 0x9197
long long open_symlink_nofollow_error(int a1)
{
  return a1 == 40 || a1 == 31;
}



// Function: grepfile @ 0x91bc
long long grepfile(int a1, int a2, char a3, unsigned char a4)
{
  int v4; // ebx
  int v5; // r8d
  int v6; // r9d
  int v7; // eax
  int *v8; // rax
  int *v9; // rax
  int v13; // [rsp+2Ch] [rbp-14h]

  if ( a3 )
    v4 = 256;
  else
    v4 = 131328;
  if ( sub_6E43(a4) )
    v7 = 2048;
  else
    v7 = 0;
  v13 = sub_22AF5(a1, a2, v4 | (unsigned int)v7, a2, v5, v6);
  if ( v13 >= 0 )
    return sub_9456((unsigned int)v13, a4);
  if ( a3 || (v8 = __errno_location(), !sub_9197(*v8)) )
  {
    v9 = __errno_location();
    sub_6EDF(*v9);
  }
  return 1;
}



// Function: drain_input @ 0x9272
bool drain_input(unsigned int a1, long long a2)
{
  ssize_t v3; // [rsp+18h] [rbp-8h]
  long long v4; // [rsp+18h] [rbp-8h]

  if ( (*(int *)(a2 + 24) & 0xF000) == 0x1000
    && byte_338B2
    && ((v3 = splice(a1, 0, 1, 0, 0x18000u, 1u), v3 >= 0) || *__errno_location() != 22) )
  {
    while ( v3 > 0 )
      v3 = splice(a1, 0, 1, 0, 0x18000u, 1u);
    return v3 == 0;
  }
  else
  {
    while ( 1 )
    {
      v4 = sub_24C3E(a1, ptr, qword_337F8);
      if ( !v4 )
        break;
      if ( v4 == -1 )
        return 0;
    }
    return 1;
  }
}



// Function: finalize_input @ 0x9361
void finalize_input(int a1, long long a2, char a3)
{
  bool v3; // al
  bool v4; // al
  bool v5; // al
  int *v6; // rax

  if ( !a1 )
  {
    if ( qword_338A0 )
    {
      v3 = a3 != 1 && (byte_337D0 || lseek(0, 0, 2) < 0 && *__errno_location() != 22) && !sub_9272(0, a2);
      v4 = v3;
    }
    else
    {
      v5 = offset != qword_33828 && byte_337D0 != 1 && lseek(0, qword_33828, 0) < 0;
      v4 = v5;
    }
    if ( v4 )
    {
      v6 = __errno_location();
      sub_6EDF(*v6);
    }
  }
}



// Function: grepdesc @ 0x9456
long long grepdesc(int a1, unsigned char a2)
{
  int *v2; // rax
  int *v3; // rax
  int *v4; // rax
  int *v5; // rax
  long long v7; // rbx
  char *v8; // rax
  long long v9; // rdx
  long long v10; // rcx
  long long v11; // r8
  long long v12; // r9
  int v13; // edx
  int *v14; // rax
  char v15; // [rsp+12h] [rbp-DEh] BYREF
  bool v16; // [rsp+13h] [rbp-DDh]
  unsigned int v17; // [rsp+14h] [rbp-DCh]
  long long v18; // [rsp+18h] [rbp-D8h]
  long long v19; // [rsp+20h] [rbp-D0h]
  long long v20; // [rsp+28h] [rbp-C8h]
  long long v21[2]; // [rsp+30h] [rbp-C0h] BYREF
  struct stat buf; // [rsp+40h] [rbp-B0h] BYREF
  unsigned long long v23; // [rsp+D8h] [rbp-18h]

  v23 = __readfsqword(0x28u);
  v16 = 1;
  v15 = 0;
  if ( fstat(a1, &buf) )
  {
    v2 = __errno_location();
    sub_6EDF(*v2);
    goto LABEL_56;
  }
  if ( a1 && sub_6E43(a2) && sub_6DEB(buf.st_mode)
    || a1 && a2 && (unsigned char)sub_7337(qword_337C0, 1u, (buf.st_mode & 0xF000) == 0x4000) )
  {
LABEL_56:
    if ( a1 )
    {
      if ( close(a1) )
      {
        v14 = __errno_location();
        sub_6EDF(*v14);
      }
    }
    return v16;
  }
  if ( dword_33848 < 0 )
    dword_33848 = (buf.st_mode & 0xF000) == 0x4000;
  if ( !a1 || dword_33010 != 3 || (buf.st_mode & 0xF000) != 0x4000 )
  {
    if ( !a1
      || (dword_33010 != 4 || (buf.st_mode & 0xF000) != 0x4000)
      && (dword_337CC != 2 && (dword_337CC || a2 == 1) || !sub_6DEB(buf.st_mode)) )
    {
      if ( byte_33850 != 1
        && !dword_33844
        && qword_33870 > 1
        && (buf.st_mode & 0xF000) == 0x8000
        && buf.st_ino == qword_336C8
        && buf.st_dev == qword_336C0 )
      {
        if ( byte_33755 != 1 )
        {
          v7 = sub_6EAC();
          v8 = gettext("%s: input file is also the output");
          error(0, 0, v8, v7);
        }
        byte_337C9 = 1;
      }
      else
      {
        v20 = sub_87E9(a1, (long long)&buf, &v15);
        if ( byte_33868 )
        {
          if ( dword_33848 )
          {
            sub_7B22();
            if ( dword_3384C )
              sub_7B58(58);
            else
              sub_6C26(0);
          }
          sub_6C8D("%ld\n", v20, v9, v10, v11, v12);
          if ( byte_33878 )
            sub_6DC0();
        }
        v16 = v20 == 0;
        if ( dword_33844 )
        {
          if ( v16 )
            v13 = 2;
          else
            v13 = 1;
          if ( v13 == dword_33844 )
          {
            sub_7B22();
            sub_6C26(dword_3384C & 0xA);
            if ( byte_33878 )
              sub_6DC0();
          }
        }
        else
        {
          sub_9361(a1, (long long)&buf, v15);
        }
      }
    }
    goto LABEL_56;
  }
  v17 = ~(a2 ^ 1) & dword_33014;
  if ( close(a1) )
  {
    v3 = __errno_location();
    sub_6EDF(*v3);
  }
  v21[0] = qword_337C0;
  v21[1] = 0;
  v18 = sub_1D975(v21, v17, 0);
  if ( !v18 )
    sub_27122();
  while ( 1 )
  {
    v19 = sub_1E2B9(v18);
    if ( !v19 )
      break;
    v16 = (v16 & (unsigned char)sub_8E14(v18, v19, a2)) != 0;
  }
  if ( *__errno_location() )
  {
    v4 = __errno_location();
    sub_6EDF(*v4);
  }
  if ( (unsigned int)sub_1DDFF(v18) )
  {
    v5 = __errno_location();
    sub_6EDF(*v5);
  }
  return v16;
}



// Function: grep_command_line_arg @ 0x992b
long long grep_command_line_arg(const char *a1)
{
  if ( !strcmp(a1, "-") )
  {
    qword_337C0 = qword_33880;
    if ( byte_338B3 )
      sub_2716D(0, 0);
    return sub_9456(0, 1u);
  }
  else
  {
    qword_337C0 = (long long)a1;
    return sub_91BC(-100, (int)a1, 1, 1u);
  }
}



// Function: usage @ 0x99b6
void usage(int a1)
{
  long long v1; // rbx
  char *v2; // rax
  long long v3; // rbx
  char *v4; // rax
  char *v5; // rax
  long long v6; // rbx
  char *v7; // rax
  char *v8; // rax
  char *v9; // rax
  char *v10; // rax
  char *v11; // rax
  char *v12; // rax
  char *v13; // rax
  char *v14; // rax
  char *v15; // rax
  char *v16; // rax
  char *v17; // rax
  char *v18; // rax

  v1 = sub_1FF0C();
  v2 = gettext("Usage: %s [OPTION]... PATTERNS [FILE]...\n");
  if ( a1 )
  {
    fprintf(stderr, v2, v1);
    v3 = sub_1FF0C();
    v4 = gettext("Try '%s --help' for more information.\n");
    fprintf(stderr, v4, v3);
  }
  else
  {
    printf(v2, v1);
    v5 = gettext("Search for PATTERNS in each FILE.\n");
    printf(v5);
    v6 = sub_1FF0C();
    v7 = gettext(
           "Example: %s -i 'hello world' menu.h main.c\n"
           "PATTERNS can contain multiple patterns separated by newlines.\n"
           "\n"
           "Pattern selection and interpretation:\n");
    printf(v7, v6);
    v8 = gettext(
           "  -E, --extended-regexp     PATTERNS are extended regular expressions\n"
           "  -F, --fixed-strings       PATTERNS are strings\n"
           "  -G, --basic-regexp        PATTERNS are basic regular expressions\n"
           "  -P, --perl-regexp         PATTERNS are Perl regular expressions\n");
    printf(v8);
    v9 = gettext(
           "  -e, --regexp=PATTERNS     use PATTERNS for matching\n"
           "  -f, --file=FILE           take PATTERNS from FILE\n"
           "  -i, --ignore-case         ignore case distinctions in patterns and data\n"
           "      --no-ignore-case      do not ignore case distinctions (default)\n"
           "  -w, --word-regexp         match only whole words\n"
           "  -x, --line-regexp         match only whole lines\n"
           "  -z, --null-data           a data line ends in 0 byte, not newline\n");
    printf(v9);
    v10 = gettext(
            "\n"
            "Miscellaneous:\n"
            "  -s, --no-messages         suppress error messages\n"
            "  -v, --invert-match        select non-matching lines\n"
            "  -V, --version             display version information and exit\n"
            "      --help                display this help text and exit\n");
    printf(v10);
    v11 = gettext(
            "\n"
            "Output control:\n"
            "  -m, --max-count=NUM       stop after NUM selected lines\n"
            "  -b, --byte-offset         print the byte offset with output lines\n"
            "  -n, --line-number         print line number with output lines\n"
            "      --line-buffered       flush output on every line\n"
            "  -H, --with-filename       print file name with output lines\n"
            "  -h, --no-filename         suppress the file name prefix on output\n"
            "      --label=LABEL         use LABEL as the standard input file name prefix\n");
    printf(v11);
    v12 = gettext(
            "  -o, --only-matching       show only nonempty parts of lines that match\n"
            "  -q, --quiet, --silent     suppress all normal output\n"
            "      --binary-files=TYPE   assume that binary files are TYPE;\n"
            "                            TYPE is 'binary', 'text', or 'without-match'\n"
            "  -a, --text                equivalent to --binary-files=text\n");
    printf(v12);
    v13 = gettext(
            "  -I                        equivalent to --binary-files=without-match\n"
            "  -d, --directories=ACTION  how to handle directories;\n"
            "                            ACTION is 'read', 'recurse', or 'skip'\n"
            "  -D, --devices=ACTION      how to handle devices, FIFOs and sockets;\n"
            "                            ACTION is 'read' or 'skip'\n"
            "  -r, --recursive           like --directories=recurse\n"
            "  -R, --dereference-recursive  likewise, but follow all symlinks\n");
    printf(v13);
    v14 = gettext(
            "      --include=GLOB        search only files that match GLOB (a file pattern)\n"
            "      --exclude=GLOB        skip files that match GLOB\n"
            "      --exclude-from=FILE   skip files that match any file pattern from FILE\n"
            "      --exclude-dir=GLOB    skip directories that match GLOB\n");
    printf(v14);
    v15 = gettext(
            "  -L, --files-without-match  print only names of FILEs with no selected lines\n"
            "  -l, --files-with-matches  print only names of FILEs with selected lines\n"
            "  -c, --count               print only a count of selected lines per FILE\n"
            "  -T, --initial-tab         make tabs line up (if needed)\n"
            "  -Z, --null                print 0 byte after FILE name\n");
    printf(v15);
    v16 = gettext(
            "\n"
            "Context control:\n"
            "  -B, --before-context=NUM  print NUM lines of leading context\n"
            "  -A, --after-context=NUM   print NUM lines of trailing context\n"
            "  -C, --context=NUM         print NUM lines of output context\n");
    printf(v16);
    v17 = gettext(
            "  -NUM                      same as --context=NUM\n"
            "      --group-separator=SEP  print SEP on line between matches with context\n"
            "      --no-group-separator  do not print separator for matches with context\n"
            "      --color[=WHEN],\n"
            "      --colour[=WHEN]       use markers to highlight the matching strings;\n"
            "                            WHEN is 'always', 'never', or 'auto'\n"
            "  -U, --binary              do not strip CR characters at EOL (MSDOS/Windows)\n"
            "\n");
    printf(v17);
    v18 = gettext(
            "When FILE is '-', read standard input.  With no FILE, read '.' if\n"
            "recursive, '-' otherwise.  With fewer than two FILEs, assume -h.\n"
            "Exit status is 0 if any line is selected, 1 otherwise;\n"
            "if any error occurs and -q is not given, the exit status is 2.\n");
    printf(v18);
    sub_269EE();
  }
  exit(a1);
}



// Function: setmatcher @ 0x9be6
void setmatcher(const char *a1, int a2)
{
  char *v2; // rax
  char *v3; // rax
  unsigned int i; // [rsp+1Ch] [rbp-4h]

  for ( i = 0; ; ++i )
  {
    if ( i > 6 )
    {
      v3 = gettext("invalid matcher %s");
      error(2, 0, v3, a1);
      return;
    }
    if ( !strcmp(a1, &aGrep[32 * i]) )
      break;
  }
  if ( a2 >= 0 && a2 != i )
  {
    v2 = gettext("conflicting matchers specified");
    error(2, 0, v2);
  }
}



// Function: get_nondigit_option @ 0x9c9f
long long get_nondigit_option(int a1, char *const *a2, long long *a3)
{
  char *v3; // rax
  char v6; // [rsp+2Fh] [rbp-41h]
  int i; // [rsp+30h] [rbp-40h]
  unsigned int v8; // [rsp+34h] [rbp-3Ch]
  char *dest; // [rsp+38h] [rbp-38h]
  char *desta; // [rsp+38h] [rbp-38h]
  char v11[21]; // [rsp+40h] [rbp-30h] BYREF
  char v12[27]; // [rsp+55h] [rbp-1Bh] BYREF

  *(long long *)&v12[19] = __readfsqword(0x28u);
  dest = v11;
  v6 = 0;
  for ( i = optind; ; i = optind )
  {
    v8 = getopt_long(a1, a2, "0123456789A:B:C:D:EFGHIPTUVX:abcd:e:f:hiLlm:noqRrsuvwxyZz", &longopts, 0);
    if ( (unsigned char)sub_10EAB(v8) != 1 )
      break;
    if ( i == dword_33018 && v6 == 1 )
      desta = &dest[-(v11[0] == 48)];
    else
      desta = v11;
    if ( desta == v12 )
    {
      strcpy(desta, "...");
      dest = desta + 3;
      break;
    }
    v3 = desta;
    dest = desta + 1;
    *v3 = v8;
    v6 = 1;
    dword_33018 = i;
  }
  if ( dest != v11 )
  {
    *dest = 0;
    sub_728F(v11, a3);
  }
  return v8;
}



// Function: parse_grep_colors @ 0x9ddf
long long parse_grep_colors()
{
  long long result; // rax
  char *v1; // rax
  char *v2; // rax
  char v3; // [rsp+7h] [rbp-29h]
  char *v4; // [rsp+8h] [rbp-28h]
  char *s2; // [rsp+10h] [rbp-20h]
  char *i; // [rsp+18h] [rbp-18h]
  char **j; // [rsp+20h] [rbp-10h]
  long long v8; // [rsp+28h] [rbp-8h]

  result = (long long)getenv("GREP_COLORS");
  v8 = result;
  if ( result )
  {
    result = *(unsigned char *)result;
    if ( (char)result )
    {
      v4 = (char *)sub_270F1(v8);
      s2 = v4;
      for ( i = 0; ; i = 0 )
      {
        while ( *v4 != 58 && *v4 )
        {
          if ( *v4 == 61 )
          {
            result = (long long)v4;
            if ( v4 == s2 || i )
              return result;
            v2 = v4++;
            *v2 = 0;
            i = v4;
          }
          else if ( i )
          {
            if ( *v4 != 59 )
            {
              result = sub_10EAB((unsigned int)*v4);
              if ( !(char)result )
                return result;
            }
            ++v4;
          }
          else
          {
            ++v4;
          }
        }
        v3 = *v4;
        v1 = v4++;
        *v1 = 0;
        for ( j = off_31F80; *j && strcmp(*j, s2); j += 3 )
          ;
        if ( j[1] && i )
          *(long long *)j[1] = i;
        result = (long long)j[2];
        if ( result )
          result = ((long long (*)(void))j[2])();
        if ( !v3 )
          break;
        s2 = v4;
      }
    }
  }
  return result;
}



// Function: contains_encoding_error @ 0x9f7b
long long contains_encoding_error(long long a1, long long a2)
{
  long long i; // [rsp+10h] [rbp-20h]
  long long v4; // [rsp+18h] [rbp-18h]
  long long v5[2]; // [rsp+20h] [rbp-10h] BYREF

  v5[1] = __readfsqword(0x28u);
  v5[0] = 0;
  for ( i = 0; i < a2; i += v4 )
  {
    v4 = sub_F6DA(a1 + i, a2 - i, v5);
    if ( v4 < 0 )
      return 1;
  }
  return 0;
}



// Function: setup_ok_fold @ 0xa011
unsigned long long setup_ok_fold()
{
  int i; // [rsp+8h] [rbp-B8h]
  char v2; // [rsp+Ch] [rbp-B4h]
  int v3; // [rsp+10h] [rbp-B0h]
  unsigned int v4; // [rsp+14h] [rbp-ACh]
  mbstate_t ps; // [rsp+18h] [rbp-A8h] BYREF
  wchar_t wc[32]; // [rsp+20h] [rbp-A0h] BYREF
  char s[24]; // [rsp+A0h] [rbp-20h] BYREF
  unsigned long long v8; // [rsp+B8h] [rbp-8h]

  v8 = __readfsqword(0x28u);
  for ( i = 0; i <= 255; ++i )
  {
    v4 = dword_331A4[i + 64];
    if ( v4 != -1 )
    {
      v2 = 1;
      v3 = sub_1C01A(v4, wc);
      while ( --v3 >= 0 )
      {
        ps = 0;
        if ( wcrtomb(s, wc[v3], &ps) != 1 )
        {
          v2 = -1;
          break;
        }
      }
      byte_338C0[i] = v2;
    }
  }
  return v8 - __readfsqword(0x28u);
}



// Function: fgrep_icase_charlen @ 0xa134
long long fgrep_icase_charlen(unsigned char *a1, long long a2, long long a3)
{
  unsigned char v4; // [rsp+2Bh] [rbp-A5h]
  unsigned int v5; // [rsp+2Ch] [rbp-A4h] BYREF
  unsigned long long v6; // [rsp+30h] [rbp-A0h]
  unsigned long long v7; // [rsp+38h] [rbp-98h]
  char v8[136]; // [rsp+40h] [rbp-90h] BYREF
  unsigned long long v9; // [rsp+C8h] [rbp-8h]

  v9 = __readfsqword(0x28u);
  if ( dword_331A4[*a1 + 64] != -1 )
    return (char)byte_338C0[*a1];
  v7 = sub_21B80(&v5, a1, a2, a3);
  if ( v7 > 0x10 )
    return -1;
  if ( (unsigned int)sub_1C01A(v5, v8) )
    return -1;
  v6 = v7;
  while ( (long long)--v6 > 0 )
  {
    v4 = a1[v6];
    if ( toupper(v4) != v4 )
      return -1;
  }
  return v7;
}



// Function: fgrep_icase_available @ 0xa29a
long long fgrep_icase_available(long long a1, long long a2)
{
  int v3; // [rsp+14h] [rbp-1Ch]
  long long i; // [rsp+18h] [rbp-18h]
  long long v5[2]; // [rsp+20h] [rbp-10h] BYREF

  v5[1] = __readfsqword(0x28u);
  v5[0] = 0;
  for ( i = 0; i < a2; i += v3 )
  {
    v3 = sub_A134((unsigned char *)(a1 + i), a2 - i, (long long)v5);
    if ( v3 < 0 )
      return 0;
  }
  return 1;
}



// Function: fgrep_to_grep_pattern @ 0xa32f
unsigned long long fgrep_to_grep_pattern(void **a1, long long *a2)
{
  unsigned int v2; // eax
  char *v3; // rax
  char *v4; // rax
  long long v6; // [rsp+18h] [rbp-38h]
  char *src; // [rsp+20h] [rbp-30h]
  char *dest; // [rsp+28h] [rbp-28h]
  long long n; // [rsp+30h] [rbp-20h]
  long long v10; // [rsp+38h] [rbp-18h]
  long long s; // [rsp+40h] [rbp-10h] BYREF
  unsigned long long v12; // [rsp+48h] [rbp-8h]

  v12 = __readfsqword(0x28u);
  v6 = *a2;
  src = *a1;
  s = 0;
  v10 = sub_26C63(v6 + 1, 2);
  dest = (char *)v10;
  while ( v6 )
  {
    n = sub_F6DA(src, v6, &s);
    if ( n != 1 )
    {
      if ( n > 1 )
        goto LABEL_8;
      if ( n == -2 )
      {
        n = v6;
LABEL_8:
        dest = mempcpy(dest, src, n);
        goto LABEL_14;
      }
      if ( n != -1 )
        goto LABEL_8;
      memset(&s, 0, sizeof(s));
      n = 1;
    }
    v2 = (char)*src - 36;
    if ( v2 <= 0x3A && ((0x580000000000441uLL >> v2) & 1) != 0 )
    {
      v3 = dest++;
      *v3 = 92;
    }
    v4 = dest++;
    *v4 = *src;
LABEL_14:
    src += n;
    v6 -= n;
  }
  *dest = 10;
  free(*a1);
  *a1 = (void *)v10;
  *a2 = (long long)&dest[-v10];
  return v12 - __readfsqword(0x28u);
}



// Function: try_fgrep_pattern @ 0xa4d3
long long try_fgrep_pattern(unsigned int a1, unsigned char *a2, long long *a3)
{
  int v3; // eax
  int v4; // eax
  long long v5; // rdx
  signed long long v6; // rax
  unsigned int v9; // [rsp+2Ch] [rbp-44h]
  long long v10; // [rsp+30h] [rbp-40h]
  void *dest; // [rsp+38h] [rbp-38h]
  unsigned char *src; // [rsp+40h] [rbp-30h]
  void *ptr; // [rsp+48h] [rbp-28h]
  size_t n; // [rsp+58h] [rbp-18h]
  long long v15[2]; // [rsp+60h] [rbp-10h] BYREF

  v15[1] = __readfsqword(0x28u);
  v9 = a1;
  v10 = *a3;
  ptr = (void *)sub_26B10(*a3 + 1);
  dest = ptr;
  src = a2;
  v15[0] = 0;
  while ( v10 )
  {
    v3 = (char)*src;
    if ( v3 > 124 )
      goto LABEL_32;
    if ( v3 < 123 )
    {
      if ( v3 == 94 )
        goto LABEL_41;
      if ( v3 > 94 )
        goto LABEL_32;
      if ( v3 == 92 )
      {
        if ( v10 <= 1 )
          goto LABEL_32;
        v4 = (char)src[1];
        if ( v4 > 60 )
        {
          if ( v4 > 124 || v4 < 62 )
            goto LABEL_31;
          v5 = 1LL << ((unsigned char)v4 - 62);
          if ( (v5 & 0x220001402200011LL) != 0 )
            goto LABEL_41;
          if ( (v5 & 0x6000000000000002LL) == 0 )
          {
LABEL_31:
            ++src;
            --v10;
            goto LABEL_32;
          }
        }
        else
        {
          if ( v4 < 10 )
            goto LABEL_31;
          if ( ((1LL << v4) & 0x13FE008000000400LL) != 0 )
            goto LABEL_41;
          if ( ((1LL << v4) & 0xB0000000000LL) == 0 )
            goto LABEL_31;
        }
        if ( !a1 )
          goto LABEL_41;
        goto LABEL_31;
      }
      if ( v3 > 92 )
        goto LABEL_32;
      if ( v3 == 91 )
        goto LABEL_41;
      if ( v3 > 46 )
      {
        if ( v3 != 63 )
          goto LABEL_32;
      }
      else
      {
        if ( v3 < 36 )
          goto LABEL_32;
        if ( ((1LL << v3) & 0x441000000000LL) != 0 )
          goto LABEL_41;
        if ( ((1LL << v3) & 0x90000000000LL) == 0 )
          goto LABEL_32;
      }
    }
    if ( a1 )
      goto LABEL_41;
LABEL_32:
    if ( byte_33180 )
      v6 = sub_A134(src, v10, (long long)v15);
    else
      v6 = sub_F6DA(src, v10, v15);
    n = v6;
    if ( v6 < 0 )
      goto LABEL_41;
    dest = mempcpy(dest, src, v6);
    src += n;
    v10 -= n;
  }
  if ( *a3 != (char *)dest - (char *)ptr )
  {
    *a3 = (char *)dest - (char *)ptr;
    *(char *)mempcpy(a2, ptr, (char *)dest - (char *)ptr) = 10;
  }
  v9 = 2;
LABEL_41:
  free(ptr);
  return v9;
}



// Function: main @ 0xa7d1
long long main(int a1, char **a2, char **a3)
{
  char *v3; // rax
  int v4; // eax
  int v5; // eax
  int v6; // eax
  int v7; // eax
  int v8; // eax
  char *v9; // rax
  const char *v10; // rax
  const char *v11; // rbx
  int *v12; // rax
  long long v13; // rax
  char *v14; // rax
  char *v15; // rax
  int v16; // ebx
  int v17; // eax
  unsigned int v18; // eax
  const char *v19; // rbx
  int *v20; // rax
  unsigned int v21; // eax
  int v22; // eax
  int v23; // eax
  int v24; // r9d
  char *v25; // rax
  int v27; // eax
  bool v28; // al
  int v29; // eax
  char *v30; // rax
  bool v31; // al
  int v32; // eax
  unsigned long long v33; // rdx
  const char **v34; // rax
  char v35; // [rsp+15h] [rbp-1FBh]
  bool v36; // [rsp+16h] [rbp-1FAh]
  int v37; // [rsp+18h] [rbp-1F8h]
  int v38; // [rsp+1Ch] [rbp-1F4h]
  int v39; // [rsp+20h] [rbp-1F0h]
  int errnum; // [rsp+24h] [rbp-1ECh]
  int k; // [rsp+28h] [rbp-1E8h]
  int m; // [rsp+2Ch] [rbp-1E4h]
  int j; // [rsp+30h] [rbp-1E0h]
  int v44; // [rsp+34h] [rbp-1DCh]
  int v45; // [rsp+38h] [rbp-1D8h]
  int v46; // [rsp+3Ch] [rbp-1D4h]
  long long v47; // [rsp+40h] [rbp-1D0h] BYREF
  long long v48; // [rsp+48h] [rbp-1C8h] BYREF
  long long v49; // [rsp+50h] [rbp-1C0h] BYREF
  char v50[8]; // [rsp+58h] [rbp-1B8h] BYREF
  char *s; // [rsp+60h] [rbp-1B0h]
  FILE *stream; // [rsp+68h] [rbp-1A8h]
  long long i; // [rsp+70h] [rbp-1A0h]
  char *ii; // [rsp+78h] [rbp-198h]
  char **v55; // [rsp+80h] [rbp-190h]
  char *v56; // [rsp+88h] [rbp-188h]
  size_t v57; // [rsp+90h] [rbp-180h]
  char *v58; // [rsp+98h] [rbp-178h]
  long long v59; // [rsp+A0h] [rbp-170h]
  long long v60; // [rsp+A8h] [rbp-168h]
  size_t v61; // [rsp+B0h] [rbp-160h]
  size_t n; // [rsp+B8h] [rbp-158h]
  signed long long v63; // [rsp+C0h] [rbp-150h]
  char *v64; // [rsp+C8h] [rbp-148h]
  struct stat buf; // [rsp+D0h] [rbp-140h] BYREF
  struct stat v66; // [rsp+160h] [rbp-B0h] BYREF
  char v67; // [rsp+1F5h] [rbp-1Bh]
  char v68[2]; // [rsp+1F6h] [rbp-1Ah] BYREF
  unsigned long long v69; // [rsp+1F8h] [rbp-18h]

  v69 = __readfsqword(0x28u);
  s = 0;
  v47 = 0;
  v48 = 0;
  v37 = -1;
  status = 2;
  v39 = 0;
  byte_33183 = 10;
  dword_3384C = -1;
  qword_33870 = 0x7FFFFFFFFFFFFFFFLL;
  qword_33858 = -1;
  qword_33860 = -1;
  v49 = -1;
  byte_3375C = 0;
  setlocale(6, &locale);
  bindtextdomain("grep", "/usr/local/share/locale");
  textdomain("grep");
  sub_1BF18(&byte_331A0);
  sub_285F0(sub_6F2C);
  sub_112DD(0);
  v38 = 0;
  qword_33790 = sub_208D6(0, 0, sub_6760, sub_67F8, 0);
  if ( !qword_33790 )
    sub_27122();
  while ( 1 )
  {
    v44 = optind;
    v22 = sub_9C9F(a1, a2, &v49);
    v45 = v22;
    if ( v22 == -1 )
      break;
    switch ( v22 )
    {
      case 0:
      case 85:
        continue;
      case 65:
        sub_728F(optarg, &qword_33860);
        continue;
      case 66:
        sub_728F(optarg, &qword_33858);
        continue;
      case 67:
        sub_728F(optarg, &v49);
        continue;
      case 68:
        if ( !strcmp(optarg, "read") )
        {
          dword_337CC = 1;
        }
        else if ( !strcmp(optarg, "skip") )
        {
          dword_337CC = 2;
        }
        else
        {
          v3 = gettext("unknown devices method");
          error(2, 0, v3);
LABEL_12:
          sub_9BE6("egrep", v37);
          v37 = v4;
        }
        continue;
      case 69:
        goto LABEL_12;
      case 70:
        sub_9BE6("fgrep", v37);
        v37 = v5;
        continue;
      case 71:
        sub_9BE6("grep", v37);
        v37 = v7;
        continue;
      case 72:
        v39 = 1;
        continue;
      case 73:
        dword_33840 = 2;
        continue;
      case 76:
        dword_33844 = 2;
        continue;
      case 80:
        sub_9BE6("perl", v37);
        v37 = v6;
        continue;
      case 82:
        dword_33014 = 778;
        goto LABEL_67;
      case 84:
        byte_3375D = 1;
        continue;
      case 86:
        byte_33754 = 1;
        continue;
      case 88:
        sub_9BE6(optarg, v37);
        v37 = v8;
        continue;
      case 90:
        dword_3384C = 0;
        continue;
      case 97:
        dword_33840 = 1;
        continue;
      case 98:
        byte_33853 = 1;
        continue;
      case 99:
        byte_33868 = 1;
        continue;
      case 100:
        dword_33010 = dword_293A8[sub_10C86(
                                    (unsigned int)"--directories",
                                    (int)optarg,
                                    (unsigned int)off_32700,
                                    (unsigned int)dword_293A8,
                                    4,
                                    (int)off_33078,
                                    1)];
        if ( dword_33010 == 3 )
          v38 = v44;
        continue;
      case 101:
        n = strlen(optarg);
        v63 = v47 - v48 + n + 1;
        if ( v63 > 0 )
        {
          s = (char *)sub_26DC6(s, &v48, v63, -1, 1);
          qword_33780 = (long long)s;
        }
        v64 = mempcpy(&s[v47], optarg, n);
        *v64 = 10;
        v47 = (long long)sub_687B((long long)s, v47, v47 + n + 1, (long long)&locale);
        continue;
      case 102:
        if ( !strcmp(optarg, "-") )
        {
          if ( byte_338B3 )
            sub_2716D(0, 0);
          stream = (FILE *)stdin;
        }
        else
        {
          if ( byte_338B3 )
            v10 = "rb";
          else
            v10 = "r";
          stream = fopen(optarg, v10);
          if ( !stream )
          {
            v11 = optarg;
            v12 = __errno_location();
            error(2, *v12, "%s", v11);
          }
        }
        for ( i = v47; ; i += v61 )
        {
          v60 = i - v48 + 2;
          if ( v60 > 0 )
          {
            s = (char *)sub_26DC6(s, &v48, v60, -1, 1);
            qword_33780 = (long long)s;
          }
          v61 = fread_unlocked(&s[i], 1u, v48 - (i + 1), stream);
          if ( !v61 )
            break;
        }
        errnum = *__errno_location();
        if ( !ferror_unlocked(stream) )
        {
          errnum = 0;
          if ( stream == (FILE *)stdin )
          {
            clearerr_unlocked(stream);
          }
          else if ( fclose(stream) )
          {
            errnum = *__errno_location();
          }
        }
        if ( errnum )
          error(2, errnum, "%s", optarg);
        if ( i != v47 && s[i - 1] != 10 )
        {
          v13 = i++;
          s[v13] = 10;
        }
        v47 = (long long)sub_687B((long long)s, v47, i, (long long)optarg);
        continue;
      case 104:
        v39 = -1;
        continue;
      case 105:
      case 121:
        byte_33180 = 1;
        continue;
      case 108:
        dword_33844 = 1;
        continue;
      case 109:
        if ( (unsigned int)sub_27281(optarg, 0, 10, &qword_33870, &locale) > 1 )
        {
          v14 = gettext("invalid max count");
          error(2, 0, v14);
        }
        continue;
      case 110:
        byte_33852 = 1;
        continue;
      case 111:
        byte_3375C = 1;
        continue;
      case 113:
        LOBYTE(word_338B1) = 1;
        status = 0;
        continue;
      case 114:
LABEL_67:
        dword_33010 = 3;
        v38 = v44;
        continue;
      case 115:
        byte_33755 = 1;
        continue;
      case 117:
        v9 = gettext("warning: --unix-byte-offsets (-u) is obsolete");
        error(0, 0, v9);
        continue;
      case 118:
        byte_33851 = 1;
        continue;
      case 119:
        sub_F770();
        byte_33181 = 1;
        continue;
      case 120:
        byte_33182 = 1;
        continue;
      case 122:
        byte_33183 = 0;
        continue;
      case 128:
        if ( !strcmp(optarg, "binary") )
        {
          dword_33840 = 0;
        }
        else if ( !strcmp(optarg, "text") )
        {
          dword_33840 = 1;
        }
        else if ( !strcmp(optarg, "without-match") )
        {
          dword_33840 = 2;
        }
        else
        {
          v15 = gettext("unknown binary-files type");
          error(2, 0, v15);
LABEL_81:
          if ( optarg )
          {
            if ( (unsigned int)sub_11390(optarg, "always")
              && (unsigned int)sub_11390(optarg, "yes")
              && (unsigned int)sub_11390(optarg, "force") )
            {
              if ( (unsigned int)sub_11390(optarg, "never")
                && (unsigned int)sub_11390(optarg, "no")
                && (unsigned int)sub_11390(optarg, "none") )
              {
                if ( (unsigned int)sub_11390(optarg, "auto")
                  && (unsigned int)sub_11390(optarg, "tty")
                  && (unsigned int)sub_11390(optarg, "if-tty") )
                {
                  dword_33750 = 1;
                }
                else
                {
                  dword_33758 = 2;
                }
              }
              else
              {
                dword_33758 = 0;
              }
            }
            else
            {
              dword_33758 = 1;
            }
          }
          else
          {
            dword_33758 = 2;
          }
        }
        break;
      case 129:
        goto LABEL_81;
      case 130:
        sub_1C12B(optarg);
        for ( j = 0; j <= 1; ++j )
        {
          if ( !qword_337B0[j] )
            qword_337B0[j] = sub_1C384();
          v21 = sub_7316(j != 0);
          sub_1CCA9(qword_337B0[j], optarg, v21);
        }
        continue;
      case 131:
      case 134:
        for ( k = 0; k <= 1; ++k )
        {
          if ( !qword_337A0[k] )
            qword_337A0[k] = sub_1C384();
          if ( v45 == 134 )
            v16 = 0x20000000;
          else
            v16 = 0;
          v17 = sub_7316(k != 0);
          sub_1CCA9(qword_337A0[k], optarg, v17 | (unsigned int)v16);
        }
        continue;
      case 132:
        for ( m = 0; m <= 1; ++m )
        {
          if ( !qword_337A0[m] )
            qword_337A0[m] = sub_1C384();
          v18 = sub_7316(m != 0);
          if ( (unsigned int)sub_1D272(sub_1CCA9, qword_337A0[m], optarg, v18, 10) )
          {
            v19 = optarg;
            v20 = __errno_location();
            error(2, *v20, "%s", v19);
          }
        }
        continue;
      case 133:
        off_33020[0] = optarg;
        continue;
      case 135:
        byte_33878 = 1;
        continue;
      case 136:
        qword_33880 = (long long)optarg;
        continue;
      case 137:
        byte_33180 = 0;
        continue;
      default:
        sub_99B6(2);
    }
  }
  if ( byte_33754 )
  {
    v23 = sub_1FF0C();
    sub_2690A((int)stdout, v23, (unsigned int)"GNU grep", (unsigned int)"3.8", 0, v24, (char)a2);
    v25 = gettext("Written by Mike Haertel and others; see\n<https://git.sv.gnu.org/cgit/grep.git/tree/AUTHORS>.");
    puts(v25);
    return 0;
  }
  if ( dword_33750 )
    sub_99B6(0);
  if ( s )
  {
    if ( !v47 )
    {
      byte_33851 = byte_33851 != 1;
      byte_33181 = 0;
      byte_33182 = 0;
      v47 = 1;
      *s = 10;
    }
  }
  else
  {
    if ( a1 <= optind )
      sub_99B6(2);
    v27 = optind++;
    v56 = a2[v27];
    v28 = v37 != 2 && *v56 == 92 && v56[1] == 45;
    s = (char *)sub_270F1(&v56[v28]);
    qword_33780 = (long long)s;
    v57 = strlen(s);
    s[v57] = 10;
    v47 = (long long)sub_687B((long long)s, 0, v57 + 1, (long long)&locale);
  }
  --v47;
  sub_20B56(qword_33790);
  v35 = 0;
  if ( (unsigned char)word_338B1 != 1 && !fstat(1, &buf) )
  {
    if ( (buf.st_mode & 0xF000) == 0x8000 )
    {
      *(struct stat *)byte_336C0 = buf;
    }
    else if ( (buf.st_mode & 0xF000) == 0x2000 )
    {
      if ( !stat("/dev/null", &v66) && buf.st_ino == v66.st_ino && buf.st_dev == v66.st_dev )
        HIBYTE(word_338B1) = 1;
      else
        v35 = 1;
    }
  }
  if ( word_338B1 )
  {
    dword_33844 = 0;
  }
  else if ( !dword_33844 )
  {
    goto LABEL_153;
  }
  byte_33868 = 0;
  byte_338B0 = 1;
LABEL_153:
  byte_33850 = (unsigned char)(byte_33868 | byte_338B0) != 0;
  if ( qword_33860 < 0 )
    qword_33860 = v49;
  if ( qword_33858 < 0 )
    qword_33858 = v49;
  if ( (!qword_33870 || !v47 && byte_33851 && byte_33182 != 1 && byte_33181 != 1) && dword_33844 != 2 )
    return 1;
  if ( dword_33758 == 2 )
  {
    v29 = v35 && (unsigned int)sub_2768B() && isatty(1);
    dword_33758 = v29;
  }
  sub_276D9();
  if ( dword_33758 )
  {
    v58 = getenv("GREP_COLOR");
    if ( v58 && *v58 )
    {
      for ( ii = v58; *ii == 59 || (unsigned char)sub_10EAB((unsigned int)*ii); ++ii )
      {
        if ( !ii[1] )
        {
          off_33030[0] = v58;
          off_33028[0] = v58;
          break;
        }
      }
    }
    sub_9DDF();
    if ( v58 == off_33028[0] || v58 == off_33030[0] )
    {
      v30 = gettext("warning: GREP_COLOR='%s' is deprecated; use GREP_COLORS='mt=%s'");
      error(0, 0, v30, v58, v58);
    }
  }
  sub_6F46();
  if ( v37 < 0 )
    v37 = 0;
  if ( (unsigned int)v37 <= 2 )
  {
    if ( byte_33180 )
      sub_A011();
    if ( v37 == 2 )
    {
      if ( byte_331A0 != 1 )
        v31 = qword_33788 == 1 && byte_33181;
      else
        v31 = (unsigned char)sub_9F7B((long long)s, v47)
           || byte_33180 && (unsigned char)sub_A29A((long long)s, v47) != 1;
      if ( v31 )
      {
        sub_A32F((void **)&qword_33780, &v47);
        s = (char *)qword_33780;
        v37 = 0;
      }
    }
    else if ( qword_33788 > 1 )
    {
      v37 = sub_A4D3(v37, (unsigned char *)s, &v47);
    }
  }
  qword_337D8 = (long long (*)(long long, long long, long long, long long, long long))*(&off_327B8 + 4 * v37);
  qword_337E0 = ((long long (*)(char *, long long, long long, bool))off_327B0[4 * v37])(
                  s,
                  v47,
                  *((int *)&unk_327AC + 8 * v37),
                  ((unsigned char)byte_3375C | dword_33758) != 0);
  v67 = 0;
  v68[0] = byte_33183;
  v68[1] = 0;
  byte_33831 = (qword_337D8(qword_337E0, v68, 1, v50, 0) == 0) == byte_33851;
  v46 = a1 - optind;
  if ( v39 || v46 > 1 )
    v32 = v39 >= 0;
  else
    v32 = -(dword_33010 == 3);
  dword_33848 = v32;
  if ( byte_338B3 )
    sub_2716D(1, 0);
  v59 = sysconf(30);
  if ( v59 <= 0 || v59 > 0x3FFFFFFFFFFFFFFBLL )
    abort();
  qword_33818 = v59;
  if ( 0x18000uLL % v59 )
    v33 = qword_33818 - 0x18000uLL % qword_33818 + 98304;
  else
    v33 = 98304;
  qword_337F8 = v33 + qword_33818 + 8;
  ptr = (void *)sub_26B10(v33 + qword_33818 + 8);
  if ( (dword_33014 & 2) != 0 && !dword_337CC )
    dword_337CC = 1;
  if ( v46 <= 0 )
  {
    if ( dword_33010 == 3 && v38 > 0 )
    {
      v55 = (char **)&off_32720;
      byte_337C8 = 1;
    }
    else
    {
      v55 = &off_32730;
    }
  }
  else
  {
    v55 = &a2[optind];
  }
  v36 = 1;
  do
  {
    v34 = (const char **)v55++;
    v36 = (v36 & (unsigned char)sub_992B(*v34)) != 0;
  }
  while ( *v55 );
  if ( byte_337C9 )
    return 2;
  else
    return v36;
}



// Function: Fcompile @ 0xc03f
long long *Fcompile(char *a1, long long a2)
{
  signed long long v3; // [rsp+28h] [rbp-48h] BYREF
  void *ptr; // [rsp+30h] [rbp-40h]
  void *s; // [rsp+38h] [rbp-38h]
  size_t n; // [rsp+40h] [rbp-30h]
  long long v7; // [rsp+48h] [rbp-28h]
  char *v8; // [rsp+50h] [rbp-20h]
  long long v9; // [rsp+58h] [rbp-18h]
  long long *v10; // [rsp+60h] [rbp-10h]
  unsigned long long v11; // [rsp+68h] [rbp-8h]

  v11 = __readfsqword(0x28u);
  ptr = 0;
  v3 = 0;
  v7 = sub_F7C8(1);
  s = a1;
  do
  {
    v8 = (char *)rawmemchr(s, 10);
    n = v8 - (char *)s;
    if ( byte_33182 )
    {
      if ( byte_33183 == 10 && a1 < s )
      {
        s = (char *)s - 1;
      }
      else
      {
        if ( (long long)(n + 1) >= v3 )
        {
          free(ptr);
          v3 = n;
          ptr = (void *)sub_26DC6(0, &v3, 2, -1, 1);
          *(char *)ptr = byte_33183;
        }
        memcpy((char *)ptr + 1, s, n);
        *((char *)ptr + n + 1) = byte_33183;
        s = ptr;
      }
      n += 2LL;
    }
    sub_CACB(v7, s, n);
    s = v8 + 1;
  }
  while ( v8 + 1 <= &a1[a2] );
  free(ptr);
  v9 = sub_D633(v7);
  sub_D9B5(v7);
  v10 = (long long *)sub_26AEA(40);
  *v10 = v7;
  v10[1] = v9;
  v10[2] = (long long)a1;
  v10[3] = a2;
  v10[4] = 0;
  return v10;
}



// Function: Fexecute @ 0xc23c
long long Fexecute(long long *a1, char *a2, long long a3, long long *a4, char *a5)
{
  char *v5; // rax
  long long *v7; // rax
  long long v8; // rax
  char *v10; // rax
  char v14; // [rsp+35h] [rbp-8Bh]
  bool v15; // [rsp+36h] [rbp-8Ah]
  bool v16; // [rsp+37h] [rbp-89h]
  void *s; // [rsp+38h] [rbp-88h] BYREF
  long long v18; // [rsp+40h] [rbp-80h] BYREF
  char *v19; // [rsp+48h] [rbp-78h]
  char *v20; // [rsp+50h] [rbp-70h]
  long long v21; // [rsp+58h] [rbp-68h]
  long long *v22; // [rsp+60h] [rbp-60h]
  long long v23; // [rsp+68h] [rbp-58h]
  long long v24; // [rsp+70h] [rbp-50h]
  char *v25; // [rsp+78h] [rbp-48h]
  char v26[16]; // [rsp+80h] [rbp-40h] BYREF
  long long v27; // [rsp+90h] [rbp-30h]
  char v28[16]; // [rsp+A0h] [rbp-20h] BYREF
  long long v29; // [rsp+B0h] [rbp-10h]
  unsigned long long v30; // [rsp+B8h] [rbp-8h]

  v30 = __readfsqword(0x28u);
  v14 = byte_33183;
  v22 = a1;
  v23 = *a1;
  v15 = (((unsigned char)byte_331A2 ^ 1) & (unsigned char)byte_331A0 & ((unsigned char)byte_33182 ^ 1)) != 0;
  v16 = ((byte_33181 | (a5 != 0 || v15)) & ((unsigned char)byte_33182 ^ 1)) != 0;
  if ( a5 )
    v5 = a5;
  else
    v5 = a2;
  v19 = v5;
  s = v5;
  while ( 1 )
  {
    if ( v19 > &a2[a3] )
      return -1;
    v24 = sub_F585(v23, &v19[-(unsigned char)byte_33182], &a2[a3] - v19 + (unsigned char)byte_33182, v26, v16);
    if ( v24 < 0 )
      return -1;
    v21 = v27 - 2 * (unsigned char)byte_33182;
    v18 = 0;
    if ( v15 && sub_F84A(&s, &v18, &v19[v24], &a2[a3]) )
    {
      v19 = (char *)s - 1;
      goto LABEL_36;
    }
    v19 += v24;
    if ( ((a5 != 0) & ((unsigned char)byte_33181 ^ 1)) != 0 )
      goto LABEL_46;
    if ( byte_33182 )
    {
      v21 += a5 == 0;
      goto LABEL_46;
    }
    if ( byte_33181 != 1 )
    {
LABEL_39:
      if ( &v19[v21] >= &a2[a3] )
        v20 = &a2[a3];
      else
        v20 = (char *)rawmemchr(&v19[v21], v14) + 1;
      goto LABEL_42;
    }
    if ( !v18 )
    {
      v25 = (char *)memrchr(s, v14, v19 - (char *)s);
      if ( v25 )
        s = v25 + 1;
    }
    if ( v18 <= 0 ? sub_FC0E(s, v19, &a2[a3]) == 0 : sub_FBE0(&v19[-v18], &a2[a3]) == 0 )
      break;
LABEL_35:
    v8 = sub_FBB2(v19, &a2[a3]);
    v19 += v8;
    s = v19;
LABEL_36:
    ++v19;
  }
  while ( 1 )
  {
    if ( !sub_FBE0(&v19[v21], &a2[a3]) )
    {
      if ( !a5 )
        goto LABEL_39;
      goto LABEL_46;
    }
    if ( !a5 && byte_331A0 != 1 )
      break;
    if ( !v21 )
      goto LABEL_35;
    if ( sub_F585(v23, v19, --v21, v28, 1) )
      goto LABEL_35;
    v21 = v29;
  }
  if ( !v22[4] )
  {
    sub_A32F((void **)v22 + 2, v22 + 3);
    v7 = sub_5661((char *)v22[2], v22[3], 0x10A46u, 0);
    v22[4] = v7;
  }
  if ( &v19[v21] >= &a2[a3] )
    v20 = &a2[a3];
  else
    v20 = (char *)rawmemchr(&v19[v21], v14) + 1;
  if ( sub_5D23(v22[4], v19, v20 - v19, a4, 0) < 0 )
  {
    v19 = v20 - 1;
    goto LABEL_35;
  }
LABEL_42:
  v19 = (char *)memrchr(a2, v14, v19 - a2);
  if ( v19 )
    v10 = v19 + 1;
  else
    v10 = a2;
  v19 = v10;
  v21 = v20 - v10;
LABEL_46:
  *a4 = v21;
  return v19 - a2;
}



// Function: U @ 0xc845
long long U(char a1)
{
  return sub_F64D((unsigned int)a1);
}



// Function: tr @ 0xc863
long long tr(long long a1, unsigned char a2)
{
  if ( a1 )
    return *(unsigned char *)((unsigned char)sub_C845(a2) + a1);
  else
    return a2;
}



// Function: kwsalloc @ 0xc89b
long long kwsalloc(long long a1)
{
  long long v2; // [rsp+18h] [rbp-38h]
  long long v3; // [rsp+48h] [rbp-8h]

  v2 = sub_26AEA(2464);
  obstack_begin(v2, 0, 0, sub_26AEA, &free);
  *(long long *)(v2 + 88) = 0;
  if ( *(long long *)(v2 + 32) - *(long long *)(v2 + 24) < 0x40u )
    obstack_newchunk(v2, 64);
  *(long long *)(v2 + 24) += 64LL;
  v3 = *(long long *)(v2 + 16);
  if ( v3 == *(long long *)(v2 + 24) )
    *(char *)(v2 + 80) |= 2u;
  *(long long *)(v2 + 24) = (*(long long *)(v2 + 48) + *(long long *)(v2 + 24)) & ~*(long long *)(v2 + 48);
  if ( *(long long *)(v2 + 24) - *(long long *)(v2 + 8) > *(long long *)(v2 + 32) - *(long long *)(v2 + 8) )
    *(long long *)(v2 + 24) = *(long long *)(v2 + 32);
  *(long long *)(v2 + 16) = *(long long *)(v2 + 24);
  *(long long *)(v2 + 96) = v3;
  **(long long **)(v2 + 96) = 0;
  *(long long *)(*(long long *)(v2 + 96) + 8LL) = 0;
  *(long long *)(*(long long *)(v2 + 96) + 16LL) = 0;
  *(long long *)(*(long long *)(v2 + 96) + 24LL) = 0;
  *(long long *)(*(long long *)(v2 + 96) + 32LL) = 0;
  *(long long *)(*(long long *)(v2 + 96) + 40LL) = 0;
  *(long long *)(*(long long *)(v2 + 96) + 48LL) = 0;
  *(long long *)(v2 + 104) = 0x7FFFFFFFFFFFFFFFLL;
  *(long long *)(v2 + 2416) = 0;
  *(long long *)(v2 + 2432) = a1;
  *(long long *)(v2 + 2456) = sub_F50C;
  return v2;
}



// Function: kwsincr @ 0xcacb
unsigned long long kwsincr(long long a1, unsigned char *a2, long long a3)
{
  unsigned char v3; // al
  unsigned char *v4; // rax
  long long v5; // rax
  long long v6; // rax
  char v7; // dl
  long long v8; // rax
  long long v9; // rax
  int v10; // eax
  int v11; // eax
  char v12; // dl
  int v13; // eax
  char v14; // dl
  long long v17; // [rsp+8h] [rbp-168h]
  unsigned char *v18; // [rsp+10h] [rbp-160h]
  bool v19; // [rsp+25h] [rbp-14Bh]
  unsigned char v20; // [rsp+27h] [rbp-149h]
  long long *v21; // [rsp+28h] [rbp-148h]
  long long *v22; // [rsp+30h] [rbp-140h]
  long long v23; // [rsp+38h] [rbp-138h]
  long long v24; // [rsp+38h] [rbp-138h]
  long long *v25; // [rsp+40h] [rbp-130h]
  long long v26; // [rsp+48h] [rbp-128h]
  long long *v27; // [rsp+78h] [rbp-F8h]
  long long v28; // [rsp+A8h] [rbp-C8h]
  long long v29; // [rsp+B0h] [rbp-C0h]
  long long v30; // [rsp+B0h] [rbp-C0h]
  long long v31; // [rsp+B0h] [rbp-C0h]
  long long v32; // [rsp+B8h] [rbp-B8h]
  long long v33; // [rsp+B8h] [rbp-B8h]
  long long v34; // [rsp+B8h] [rbp-B8h]
  char *v35; // [rsp+C0h] [rbp-B0h]
  char *v36; // [rsp+C0h] [rbp-B0h]
  char *v37; // [rsp+C0h] [rbp-B0h]
  long long v38; // [rsp+C8h] [rbp-A8h]
  long long v39; // [rsp+C8h] [rbp-A8h]
  long long v40; // [rsp+C8h] [rbp-A8h]
  int v41[12]; // [rsp+D0h] [rbp-A0h]
  long long v42[13]; // [rsp+100h] [rbp-70h]
  unsigned long long v43; // [rsp+168h] [rbp-8h]

  v18 = a2;
  v17 = a3;
  v43 = __readfsqword(0x28u);
  v21 = *(long long **)(a1 + 96);
  v26 = *(long long *)(a1 + 2432);
  v19 = *(long long *)(a1 + 2456) == (long long)sub_F061;
  if ( *(long long (**)())(a1 + 2456) == sub_F061 )
    v18 = &a2[a3];
  while ( v17-- )
  {
    if ( v19 )
    {
      v3 = *--v18;
    }
    else
    {
      v4 = v18++;
      v3 = *v4;
    }
    if ( v26 )
      v3 = *(char *)(v3 + v26);
    v20 = v3;
    v22 = (long long *)v21[1];
    v42[0] = v21 + 1;
    v41[0] = 0;
    v23 = 1;
    while ( v22 && v20 != *((char *)v22 + 24) )
    {
      v42[v23] = v22;
      v5 = v23++;
      if ( v20 >= *((char *)v22 + 24) )
      {
        v41[v5] = 1;
        v22 = (long long *)v22[1];
      }
      else
      {
        v41[v5] = 0;
        v22 = (long long *)*v22;
      }
    }
    if ( !v22 )
    {
      if ( *(long long *)(a1 + 32) - *(long long *)(a1 + 24) < 0x20u )
        obstack_newchunk(a1, 32);
      *(long long *)(a1 + 24) += 32LL;
      v27 = *(long long **)(a1 + 16);
      if ( v27 == *(long long **)(a1 + 24) )
        *(char *)(a1 + 80) |= 2u;
      *(long long *)(a1 + 24) = (*(long long *)(a1 + 48) + *(long long *)(a1 + 24)) & ~*(long long *)(a1 + 48);
      if ( *(long long *)(a1 + 24) - *(long long *)(a1 + 8) > *(long long *)(a1 + 32) - *(long long *)(a1 + 8) )
        *(long long *)(a1 + 24) = *(long long *)(a1 + 32);
      *(long long *)(a1 + 16) = *(long long *)(a1 + 24);
      v22 = v27;
      *v27 = 0;
      v27[1] = 0;
      if ( *(long long *)(a1 + 32) - *(long long *)(a1 + 24) < 0x40u )
        obstack_newchunk(a1, 64);
      *(long long *)(a1 + 24) += 64LL;
      v28 = *(long long *)(a1 + 16);
      if ( v28 == *(long long *)(a1 + 24) )
        *(char *)(a1 + 80) |= 2u;
      *(long long *)(a1 + 24) = (*(long long *)(a1 + 48) + *(long long *)(a1 + 24)) & ~*(long long *)(a1 + 48);
      if ( *(long long *)(a1 + 24) - *(long long *)(a1 + 8) > *(long long *)(a1 + 32) - *(long long *)(a1 + 8) )
        *(long long *)(a1 + 24) = *(long long *)(a1 + 32);
      *(long long *)(a1 + 16) = *(long long *)(a1 + 24);
      v27[2] = v28;
      *(long long *)v27[2] = 0;
      *(long long *)(v27[2] + 8) = 0;
      *(long long *)(v27[2] + 16) = v21;
      *(long long *)(v27[2] + 24) = 0;
      *(long long *)(v27[2] + 32) = 0;
      *(long long *)(v27[2] + 40) = v21[5] + 1LL;
      *(long long *)(v27[2] + 48) = 0;
      *((char *)v27 + 24) = v20;
      *((char *)v27 + 25) = 0;
      v24 = v23 - 1;
      if ( v41[v24] )
        *(long long *)(v42[v24] + 8LL) = v27;
      else
        *(long long *)v42[v24] = v27;
      while ( v24 && !*(char *)(v42[v24] + 25LL) )
      {
        v6 = v42[v24];
        if ( v41[v24] )
          v7 = *(char *)(v6 + 25) + 1;
        else
          v7 = *(char *)(v6 + 25) - 1;
        *(char *)(v6 + 25) = v7;
        --v24;
      }
      if ( v24 )
      {
        if ( !v41[v24] && (v8 = v42[v24], --*(char *)(v8 + 25), *(char *)(v8 + 25))
          || v41[v24] == 1 && (v9 = v42[v24], ++*(char *)(v9 + 25), *(char *)(v9 + 25)) )
        {
          v10 = *(char *)(v42[v24] + 25LL);
          if ( v10 == -2 )
          {
            v11 = v41[v24 + 1];
            if ( v11 )
            {
              if ( v11 != 1 )
                abort();
              v36 = (char *)v42[v24];
              v29 = *(long long *)v36;
              v25 = *(long long **)(*(long long *)v36 + 8LL);
              v39 = v25[1];
              v32 = *v25;
              *v25 = *(long long *)v36;
              *(long long *)(v29 + 8) = v32;
              v25[1] = v36;
              *(long long *)v36 = v39;
              if ( *((char *)v25 + 25) == 1 )
                v12 = -1;
              else
                v12 = 0;
              *(char *)(v29 + 25) = v12;
              v36[25] = *((char *)v25 + 25) == 0xFF;
              *((char *)v25 + 25) = 0;
            }
            else
            {
              v35 = (char *)v42[v24];
              v25 = *(long long **)v35;
              v38 = *(long long *)(*(long long *)v35 + 8LL);
              v25[1] = v35;
              *(long long *)v35 = v38;
              v35[25] = 0;
              *((char *)v25 + 25) = v35[25];
            }
          }
          else
          {
            if ( v10 != 2 )
              abort();
            v13 = v41[v24 + 1];
            if ( v13 )
            {
              if ( v13 != 1 )
                abort();
              v30 = v42[v24];
              v25 = *(long long **)(v30 + 8);
              v33 = *v25;
              *v25 = v30;
              *(long long *)(v30 + 8) = v33;
              *(char *)(v30 + 25) = 0;
              *((char *)v25 + 25) = *(char *)(v30 + 25);
            }
            else
            {
              v31 = v42[v24];
              v37 = *(char **)(v31 + 8);
              v25 = *(long long **)v37;
              v34 = **(long long **)v37;
              v40 = *(long long *)(*(long long *)v37 + 8LL);
              *v25 = v31;
              *(long long *)(v31 + 8) = v34;
              v25[1] = v37;
              *(long long *)v37 = v40;
              if ( *((char *)v25 + 25) == 1 )
                v14 = -1;
              else
                v14 = 0;
              *(char *)(v31 + 25) = v14;
              v37[25] = *((char *)v25 + 25) == 0xFF;
              *((char *)v25 + 25) = 0;
            }
          }
          if ( v41[v24 - 1] )
            *(long long *)(v42[v24 - 1] + 8LL) = v25;
          else
            *(long long *)v42[v24 - 1] = v25;
        }
      }
    }
    v21 = (long long *)v22[2];
  }
  if ( !*v21 )
    *v21 = 2LL * *(long long *)(a1 + 88) + 1;
  ++*(long long *)(a1 + 88);
  if ( v21[5] < *(long long *)(a1 + 104) )
    *(long long *)(a1 + 104) = v21[5];
  return v43 - __readfsqword(0x28u);
}



// Function: kwswords @ 0xd633
long long kwswords(long long a1)
{
  return *(long long *)(a1 + 88);
}



// Function: enqueue @ 0xd649
long long *enqueue(long long *a1, long long *a2)
{
  long long v2; // rax
  long long *result; // rax

  if ( a1 )
  {
    enqueue(*a1, a2);
    enqueue(a1[1], a2);
    v2 = *a2;
    *(long long *)(v2 + 24) = a1[2];
    *a2 = *(long long *)(v2 + 24);
    return a2;
  }
  return result;
}



// Function: treefails @ 0xd6b4
unsigned long long treefails(long long a1, long long a2, long long a3, unsigned int a4)
{
  unsigned long long result; // rax
  unsigned char v5; // [rsp+4h] [rbp-2Ch]
  long long v7; // [rsp+10h] [rbp-20h]
  long long *v8; // [rsp+28h] [rbp-8h]

  v7 = a2;
  result = a4;
  v5 = a4;
  if ( a1 )
  {
    treefails(*(long long *)a1, a2, a3, (unsigned char)a4);
    treefails(*(long long *)(a1 + 8), a2, a3, v5);
    while ( 1 )
    {
      if ( !v7 )
      {
        result = *(long long *)(a1 + 16);
        *(long long *)(result + 32) = a3;
        return result;
      }
      v8 = *(long long **)(v7 + 8);
      while ( v8 && *(char *)(a1 + 24) != *((char *)v8 + 24) )
      {
        if ( *(char *)(a1 + 24) >= *((char *)v8 + 24) )
          v8 = (long long *)v8[1];
        else
          v8 = (long long *)*v8;
      }
      if ( v8 )
        break;
      v7 = *(long long *)(v7 + 32);
    }
    *(long long *)(*(long long *)(a1 + 16) + 32LL) = v8[2];
    result = v5 ^ 1u;
    if ( v5 != 1 )
    {
      result = *(long long *)v8[2];
      if ( result )
      {
        result = **(long long **)(a1 + 16);
        if ( !result )
        {
          result = *(long long *)(a1 + 16);
          *(long long *)result = -1;
        }
      }
    }
  }
  return result;
}



// Function: treedelta @ 0xd7f4
char *treedelta(long long a1, long long a2, long long a3)
{
  char *result; // rax

  if ( a1 )
  {
    treedelta(*(long long *)a1, a2, a3);
    treedelta(*(long long *)(a1 + 8), a2, a3);
    result = (char *)*(unsigned char *)(*(unsigned char *)(a1 + 24) + a3);
    if ( a2 < (long long)result )
    {
      result = (char *)(*(unsigned char *)(a1 + 24) + a3);
      *result = a2;
    }
  }
  return result;
}



// Function: hasevery @ 0xd883
bool hasevery(long long *a1, long long a2)
{
  long long *v3; // [rsp+8h] [rbp-8h]

  v3 = a1;
  if ( !a2 )
    return 1;
  if ( (unsigned char)hasevery(a1, *(long long *)a2) != 1 )
    return 0;
  if ( (unsigned char)hasevery(a1, *(long long *)(a2 + 8)) != 1 )
    return 0;
  while ( v3 && *(char *)(a2 + 24) != *((char *)v3 + 24) )
  {
    if ( *(char *)(a2 + 24) >= *((char *)v3 + 24) )
      v3 = (long long *)v3[1];
    else
      v3 = (long long *)*v3;
  }
  return v3 != 0;
}



// Function: treenext @ 0xd943
long long treenext(long long a1, long long a2)
{
  long long result; // rax

  if ( a1 )
  {
    treenext(*(long long *)a1, a2);
    treenext(*(long long *)(a1 + 8), a2);
    result = *(long long *)(a1 + 16);
    *(long long *)(a2 + 8LL * *(unsigned char *)(a1 + 24)) = result;
  }
  return result;
}



// Function: kwsprep @ 0xd9b5
unsigned long long kwsprep(long long a1)
{
  char *v1; // rax
  long long v2; // rax
  char *v3; // rax
  unsigned char v4; // al
  int v5; // eax
  unsigned char v6; // al
  bool v8; // [rsp+1Bh] [rbp-A15h]
  int v9; // [rsp+1Ch] [rbp-A14h]
  int v10; // [rsp+20h] [rbp-A10h]
  int m; // [rsp+24h] [rbp-A0Ch]
  int v12; // [rsp+28h] [rbp-A08h]
  int jj; // [rsp+2Ch] [rbp-A04h]
  long long *v14; // [rsp+30h] [rbp-A00h] BYREF
  long long *i; // [rsp+38h] [rbp-9F8h]
  long long j; // [rsp+40h] [rbp-9F0h]
  long long k; // [rsp+48h] [rbp-9E8h]
  long long n; // [rsp+50h] [rbp-9E0h]
  long long ii; // [rsp+58h] [rbp-9D8h]
  long long v20; // [rsp+60h] [rbp-9D0h]
  void *s; // [rsp+68h] [rbp-9C8h]
  long long v22; // [rsp+70h] [rbp-9C0h]
  long long v23; // [rsp+78h] [rbp-9B8h]
  unsigned long long v24; // [rsp+80h] [rbp-9B0h]
  long long v25; // [rsp+88h] [rbp-9A8h]
  long long v26; // [rsp+90h] [rbp-9A0h]
  long long v27; // [rsp+98h] [rbp-998h]
  void *ptr; // [rsp+A0h] [rbp-990h]
  struct obstack *obstack; // [rsp+A8h] [rbp-988h]
  void *block; // [rsp+B0h] [rbp-980h]
  void *v31; // [rsp+B8h] [rbp-978h]
  long long v32; // [rsp+C0h] [rbp-970h]
  long long v33; // [rsp+C8h] [rbp-968h]
  unsigned long long v34; // [rsp+D0h] [rbp-960h]
  long long v35; // [rsp+D8h] [rbp-958h]
  long long v36; // [rsp+E0h] [rbp-950h]
  long long v37; // [rsp+E8h] [rbp-948h]
  long long v38; // [rsp+F0h] [rbp-940h]
  long long v39; // [rsp+F8h] [rbp-938h]
  unsigned long long v40; // [rsp+100h] [rbp-930h]
  long long v41; // [rsp+108h] [rbp-928h]
  long long v42; // [rsp+110h] [rbp-920h]
  long long v43; // [rsp+118h] [rbp-918h]
  char v44; // [rsp+120h] [rbp-910h] BYREF
  char v45; // [rsp+920h] [rbp-110h] BYREF
  unsigned long long v46; // [rsp+A28h] [rbp-8h]

  v46 = __readfsqword(0x28u);
  v20 = *(long long *)(a1 + 2432);
  if ( v20 )
    v1 = &v45;
  else
    v1 = (char *)(a1 + 112);
  s = v1;
  v8 = *(long long *)(a1 + 88) == 1;
  if ( *(long long *)(a1 + 88) == 1 )
  {
    v14 = *(long long **)(a1 + 96);
    for ( i = v14; i; i = (long long *)i[3] )
      sub_D649((long long *)i[1], (long long *)&v14);
    v22 = a1;
    v23 = a1;
    v24 = *(long long *)(a1 + 104);
    v25 = a1;
    if ( *(long long *)(a1 + 32) - *(long long *)(a1 + 24) < v24 )
      obstack_newchunk(v23, v24);
    *(long long *)(v23 + 24) += v24;
    v26 = v22;
    v27 = *(long long *)(v22 + 16);
    if ( v27 == *(long long *)(v22 + 24) )
      *(char *)(v26 + 80) |= 2u;
    *(long long *)(v26 + 24) = (*(long long *)(v26 + 48) + *(long long *)(v26 + 24)) & ~*(long long *)(v26 + 48);
    if ( *(long long *)(v26 + 24) - *(long long *)(v26 + 8) > *(long long *)(v26 + 32) - *(long long *)(v26 + 8) )
      *(long long *)(v26 + 24) = *(long long *)(v26 + 32);
    *(long long *)(v26 + 16) = *(long long *)(v26 + 24);
    *(long long *)(a1 + 2416) = v27;
    i = *(long long **)(a1 + 96);
    for ( j = 0; j < *(long long *)(a1 + 104); ++j )
    {
      *(char *)(*(long long *)(a1 + 2416) + j) = *(char *)(i[1] + 24LL);
      i = (long long *)i[3];
    }
    ptr = (void *)sub_C89B(*(long long *)(a1 + 2432));
    *((long long *)ptr + 307) = sub_F061;
    sub_CACB((long long)ptr, *(unsigned char **)(a1 + 2416), *(long long *)(a1 + 104));
    obstack = (struct obstack *)a1;
    block = 0;
    obstack_free((struct obstack *)a1, 0);
    qmemcpy((void *)a1, ptr, 0x9A0u);
    free(ptr);
  }
  v2 = *(long long *)(a1 + 104);
  if ( v2 > 255 )
    LODWORD(v2) = 255;
  memset(s, v2, 0x100u);
  v14 = *(long long **)(a1 + 96);
  for ( i = v14; i; i = (long long *)i[3] )
  {
    sub_D649((long long *)i[1], (long long *)&v14);
    sub_D7F4(i[1], i[5], (long long)s);
    sub_D6B4(i[1], i[4], *(long long *)(a1 + 96), v8);
    if ( v8 )
    {
      i[6] = *(long long *)(a1 + 104);
      i[7] = *(long long *)(a1 + 104);
      for ( k = i[4]; k; k = *(long long *)(k + 32) )
      {
        if ( !sub_D883(*(long long **)(k + 8), i[1]) && i[5] - *(long long *)(k + 40) < *(long long *)(k + 48) )
          *(long long *)(k + 48) = i[5] - *(long long *)(k + 40);
        if ( *i && *(long long *)(k + 56) > i[5] - *(long long *)(k + 40) )
          *(long long *)(k + 56) = i[5] - *(long long *)(k + 40);
      }
    }
  }
  if ( v8 )
  {
    for ( i = *(long long **)(*(long long *)(a1 + 96) + 24LL); i; i = (long long *)i[3] )
    {
      if ( i[7] > *(long long *)(i[2] + 56LL) )
        i[7] = *(long long *)(i[2] + 56LL);
      if ( i[6] > i[7] )
        i[6] = i[7];
    }
  }
  if ( v20 )
    v3 = &v44;
  else
    v3 = (char *)(a1 + 368);
  v31 = v3;
  memset(v3, 0, 0x800u);
  sub_D943(*(long long *)(*(long long *)(a1 + 96) + 8LL), (long long)v31);
  v9 = -2;
  v10 = -1;
  for ( m = 0; m <= 255; ++m )
  {
    v12 = m;
    if ( v20 )
    {
      v4 = sub_C845(*(char *)(m + v20));
      v12 = v4;
      *(long long *)(a1 + 8 * (m + 46LL)) = *((long long *)v31 + v4);
    }
    if ( *(long long *)(a1 + 8 * (m + 46LL)) )
    {
      if ( v9 >= -1 )
      {
        if ( v9 == v12 )
        {
          if ( v10 == v12 )
            v5 = m;
          else
            v5 = -1;
          v10 = v5;
        }
        else if ( m == v12 && v9 == v10 )
        {
          v10 = m;
        }
        else
        {
          v9 = -1;
        }
      }
      else
      {
        v9 = v12;
        v10 = m;
      }
    }
  }
  *(int *)(a1 + 2440) = v9;
  *(int *)(a1 + 2444) = v10;
  if ( v8 )
  {
    v32 = a1;
    v33 = a1;
    v34 = *(long long *)(a1 + 104);
    v35 = a1;
    if ( *(long long *)(a1 + 32) - *(long long *)(a1 + 24) < v34 )
      obstack_newchunk(v33, v34);
    *(long long *)(v33 + 24) += v34;
    v36 = v32;
    v37 = *(long long *)(v32 + 16);
    if ( v37 == *(long long *)(v32 + 24) )
      *(char *)(v36 + 80) |= 2u;
    *(long long *)(v36 + 24) = (*(long long *)(v36 + 48) + *(long long *)(v36 + 24)) & ~*(long long *)(v36 + 48);
    if ( *(long long *)(v36 + 24) - *(long long *)(v36 + 8) > *(long long *)(v36 + 32) - *(long long *)(v36 + 8) )
      *(long long *)(v36 + 24) = *(long long *)(v36 + 32);
    *(long long *)(v36 + 16) = *(long long *)(v36 + 24);
    *(long long *)(a1 + 2416) = v37;
    i = *(long long **)(a1 + 96);
    for ( n = *(long long *)(a1 + 104); n > 0; --n )
    {
      *(char *)(*(long long *)(a1 + 2416) + n - 1) = *(char *)(i[1] + 24LL);
      i = (long long *)i[3];
    }
    if ( *(long long *)(a1 + 104) > 1 )
    {
      v38 = a1;
      v39 = a1;
      v40 = 8 * (*(long long *)(a1 + 104) - 1LL);
      v41 = a1;
      if ( *(long long *)(a1 + 32) - *(long long *)(a1 + 24) < v40 )
        obstack_newchunk(v39, v40);
      *(long long *)(v39 + 24) += v40;
      v42 = v38;
      v43 = *(long long *)(v38 + 16);
      if ( v43 == *(long long *)(v38 + 24) )
        *(char *)(v42 + 80) |= 2u;
      *(long long *)(v42 + 24) = (*(long long *)(v42 + 48) + *(long long *)(v42 + 24)) & ~*(long long *)(v42 + 48);
      if ( *(long long *)(v42 + 24) - *(long long *)(v42 + 8) > *(long long *)(v42 + 32) - *(long long *)(v42 + 8) )
        *(long long *)(v42 + 24) = *(long long *)(v42 + 32);
      *(long long *)(v42 + 16) = *(long long *)(v42 + 24);
      *(long long *)(a1 + 2424) = v43;
      i = *(long long **)(*(long long *)(a1 + 96) + 24LL);
      for ( ii = 0; ii < *(long long *)(a1 + 104) - 1LL; ++ii )
      {
        *(long long *)(8 * ii + *(long long *)(a1 + 2424)) = i[6];
        i = (long long *)i[3];
      }
      *(char *)(a1 + 2448) = sub_C863(v20, *(char *)(*(long long *)(a1 + 2416) + *(long long *)(a1 + 104) - 2LL));
    }
  }
  if ( v20 )
  {
    for ( jj = 0; jj <= 255; ++jj )
    {
      v6 = sub_C845(*(char *)(jj + v20));
      *(char *)(a1 + jj + 112) = *((char *)s + v6);
    }
  }
  return v46 - __readfsqword(0x28u);
}



// Function: bm_delta2_search @ 0xe78b
long long bm_delta2_search(
        unsigned long long *a1,
        unsigned long long a2,
        long long a3,
        long long a4,
        long long a5,
        char a6,
        char a7,
        long long a8,
        long long a9)
{
  char v9; // bl
  char v10; // bl
  unsigned long long v16; // [rsp+30h] [rbp-30h]
  long long v17; // [rsp+38h] [rbp-28h]
  long long i; // [rsp+40h] [rbp-20h]
  long long j; // [rsp+48h] [rbp-18h]

  v16 = *a1;
  v17 = a4;
  for ( i = 0; ; i = j - 1 )
  {
    j = 2;
    if ( a7 == (unsigned char)sub_C863(a5, *(char *)(v16 - 2)) )
    {
      do
      {
        if ( ++j > v17 )
          break;
        v9 = sub_C863(a5, *(char *)(v16 - j));
      }
      while ( v9 == (unsigned char)sub_C863(a5, *(char *)(a3 - j)) );
      if ( j > v17 )
      {
        for ( j = v17 + i + 1; j <= a4; ++j )
        {
          v10 = sub_C863(a5, *(char *)(v16 - j));
          if ( v10 != (unsigned char)sub_C863(a5, *(char *)(a3 - j)) )
            break;
        }
        if ( j > a4 )
        {
          *a1 = v16 - a4;
          return 1;
        }
      }
    }
    v17 = *(long long *)(*(long long *)(a9 + 2424) + 8 * j - 16);
    v16 += v17;
    if ( v16 > a2 )
      break;
    if ( a6 != (unsigned char)sub_C863(a5, *(char *)(v16 - 1)) )
    {
      if ( a8 )
        v16 += *(unsigned char *)((unsigned char)sub_C845(*(char *)(v16 - 1)) + a8);
      break;
    }
  }
  *a1 = v16;
  return 0;
}



// Function: memchr_kwset @ 0xe9c2
long long memchr_kwset(char *a1, long long a2, long long a3)
{
  char *v5; // [rsp+18h] [rbp-28h]
  unsigned long long v6; // [rsp+30h] [rbp-10h]

  v5 = a1;
  v6 = (unsigned long long)&a1[a2];
  if ( *(int *)(a3 + 2444) >= 0 )
  {
    while ( (unsigned long long)v5 < v6 )
    {
      if ( *(long long *)(a3 + 8 * ((unsigned char)sub_C845(*v5) + 46LL)) )
        return (long long)v5;
      if ( !((unsigned long long)++v5 % 0x10) )
        return sub_222E7(v5, *(unsigned int *)(a3 + 2440), *(unsigned int *)(a3 + 2444), v6 - (long long)v5);
    }
  }
  else
  {
    while ( (unsigned long long)v5 < v6 )
    {
      if ( *(long long *)(a3 + 8 * ((unsigned char)sub_C845(*v5) + 46LL)) )
        return (long long)v5;
      ++v5;
    }
  }
  return 0;
}



// Function: bmexec_trans @ 0xeae1
long long bmexec_trans(long long a1, char *a2, long long a3)
{
  unsigned char v4; // al
  unsigned char v5; // al
  unsigned char v6; // al
  unsigned char v7; // al
  unsigned char v8; // al
  unsigned char v9; // al
  unsigned char v10; // al
  unsigned char v11; // al
  unsigned char v12; // al
  unsigned char v13; // al
  unsigned char v14; // al
  unsigned char v15; // al
  char v17; // [rsp+2Eh] [rbp-52h]
  char v18; // [rsp+2Fh] [rbp-51h]
  int v19; // [rsp+30h] [rbp-50h]
  int v20; // [rsp+30h] [rbp-50h]
  int v21; // [rsp+30h] [rbp-50h]
  int v22; // [rsp+30h] [rbp-50h]
  char *v23; // [rsp+38h] [rbp-48h] BYREF
  long long v24; // [rsp+40h] [rbp-40h]
  long long v25; // [rsp+48h] [rbp-38h]
  long long v26; // [rsp+50h] [rbp-30h]
  long long v27; // [rsp+58h] [rbp-28h]
  long long v28; // [rsp+60h] [rbp-20h]
  char *v29; // [rsp+68h] [rbp-18h]
  char *v30; // [rsp+70h] [rbp-10h]
  unsigned long long v31; // [rsp+78h] [rbp-8h]

  v31 = __readfsqword(0x28u);
  v25 = *(long long *)(a1 + 104);
  v26 = *(long long *)(a1 + 2432);
  if ( !v25 )
    return 0;
  if ( v25 > a3 )
    return -1;
  if ( v25 == 1 )
  {
    v23 = (char *)sub_E9C2(a2, a3, a1);
    if ( v23 )
      return v23 - a2;
    else
      return -1;
  }
  v27 = a1 + 112;
  v28 = *(long long *)(a1 + 2416) + v25;
  v23 = &a2[v25];
  v17 = *(int *)(a1 + 2440);
  v18 = *(char *)(a1 + 2448);
  v24 = 12 * v25;
  if ( is_mul_ok(0xCu, v25) && a3 > v24 )
  {
    v29 = &a2[a3 - 11 * v25];
    while ( 1 )
    {
      while ( 1 )
      {
        if ( v29 < v23 )
          goto LABEL_22;
        v30 = v23;
        v4 = sub_C845(*(v23 - 1));
        v23 += *(unsigned char *)(v4 + v27);
        v5 = sub_C845(*(v23 - 1));
        v19 = *(unsigned char *)(v5 + v27);
        v23 += *(unsigned char *)(v5 + v27);
        if ( v19 )
        {
          v6 = sub_C845(*(v23 - 1));
          v23 += *(unsigned char *)(v6 + v27);
          v7 = sub_C845(*(v23 - 1));
          v23 += *(unsigned char *)(v7 + v27);
          v8 = sub_C845(*(v23 - 1));
          v20 = *(unsigned char *)(v8 + v27);
          v23 += *(unsigned char *)(v8 + v27);
          if ( v20 )
          {
            v9 = sub_C845(*(v23 - 1));
            v23 += *(unsigned char *)(v9 + v27);
            v10 = sub_C845(*(v23 - 1));
            v23 += *(unsigned char *)(v10 + v27);
            v11 = sub_C845(*(v23 - 1));
            v21 = *(unsigned char *)(v11 + v27);
            v23 += *(unsigned char *)(v11 + v27);
            if ( v21 )
              break;
          }
        }
LABEL_19:
        if ( (unsigned char)sub_E78B(
                                (unsigned long long *)&v23,
                                (unsigned long long)v29,
                                v28,
                                v25,
                                v26,
                                v17,
                                v18,
                                v27,
                                a1) )
          return v23 - a2;
      }
      v12 = sub_C845(*(v23 - 1));
      v23 += *(unsigned char *)(v12 + v27);
      v13 = sub_C845(*(v23 - 1));
      v23 += *(unsigned char *)(v13 + v27);
      if ( v23 - v30 < 128 )
      {
        --v23;
        v23 = (char *)sub_E9C2(v23, &a2[a3] - v23, a1);
        if ( !v23 )
          return -1;
        if ( v29 <= ++v23 )
          break;
        goto LABEL_19;
      }
    }
  }
LABEL_22:
  v29 = &a2[a3];
  v14 = sub_C845(*(v23 - 1));
  v22 = *(unsigned char *)(v14 + v27);
  while ( v22 <= v29 - v23 )
  {
    v23 += v22;
    v15 = sub_C845(*(v23 - 1));
    v22 = *(unsigned char *)(v15 + v27);
    if ( !*(char *)(v15 + v27)
      && (unsigned char)sub_E78B((unsigned long long *)&v23, (unsigned long long)v29, v28, v25, v26, v17, v18, 0, a1) )
    {
      return v23 - a2;
    }
  }
  return -1;
}



// Function: bmexec @ 0xf061
long long bmexec(long long a1, char *a2, long long a3, long long *a4)
{
  long long result; // rax

  result = sub_EAE1(a1, a2, a3);
  *a4 = 0;
  a4[1] = result;
  a4[2] = *(long long *)(a1 + 104);
  return result;
}



// Function: acexec_trans @ 0xf0f4
long long acexec_trans(long long a1, char *a2, long long a3, long long *a4, char a5)
{
  unsigned char *v6; // rax
  unsigned char *v7; // rax
  unsigned char *v8; // rax
  long long v9; // rax
  unsigned char *v10; // rax
  long long v11; // rax
  unsigned char v14; // [rsp+32h] [rbp-4Eh]
  unsigned char v15; // [rsp+33h] [rbp-4Dh]
  int v16; // [rsp+34h] [rbp-4Ch]
  long long *v17; // [rsp+38h] [rbp-48h]
  long long *i; // [rsp+40h] [rbp-40h]
  char *v19; // [rsp+48h] [rbp-38h]
  long long v20; // [rsp+48h] [rbp-38h]
  char *v21; // [rsp+50h] [rbp-30h]
  long long m; // [rsp+58h] [rbp-28h]
  long long j; // [rsp+58h] [rbp-28h]
  long long *k; // [rsp+60h] [rbp-20h]
  long long v25; // [rsp+68h] [rbp-18h]
  unsigned long long v26; // [rsp+70h] [rbp-10h]

  if ( a3 < *(long long *)(a1 + 104) )
    return -1;
  v25 = *(long long *)(a1 + 2432);
  v17 = *(long long **)(a1 + 96);
  v26 = (unsigned long long)&a2[a3];
  v19 = a2;
  if ( *v17 )
  {
LABEL_33:
    for ( i = v17; *i < 0; i = (long long *)i[4] )
      ;
    v21 = &v19[-i[5]];
    if ( a5 )
    {
      while ( (unsigned long long)v19 < v26 )
      {
        v10 = (unsigned char *)v19++;
        v15 = sub_C863(v25, *v10);
        do
        {
          for ( j = v17[1]; j && v15 != *(char *)(j + 24); j = v11 )
          {
            if ( v15 >= *(char *)(j + 24) )
              v11 = *(long long *)(j + 8);
            else
              v11 = *(long long *)j;
          }
          if ( j )
            break;
          v17 = (long long *)v17[4];
          if ( !v17 )
            break;
        }
        while ( i[5] <= v17[5] );
        if ( !j )
          break;
        v17 = *(long long **)(j + 16);
        if ( *v17 )
        {
          for ( k = *(long long **)(j + 16); *k < 0; k = (long long *)k[4] )
            ;
          if ( &v19[-k[5]] <= v21 )
          {
            v21 = &v19[-k[5]];
            i = k;
          }
        }
      }
    }
    *a4 = *i >> 1;
    a4[1] = v21 - a2;
    a4[2] = i[5];
    return v21 - a2;
  }
  else
  {
    v16 = *(int *)(a1 + 2440);
    while ( 1 )
    {
      if ( v16 >= 0 )
      {
        v20 = sub_E9C2(v19, v26 - (long long)v19, a1);
        if ( !v20 )
          return -1;
        v7 = (unsigned char *)v20;
        v19 = (char *)(v20 + 1);
        v17 = *(long long **)(a1 + 8 * ((unsigned char)sub_C863(v25, *v7) + 46LL));
      }
      else
      {
        while ( 1 )
        {
          v6 = (unsigned char *)v19++;
          v17 = *(long long **)(a1 + 8 * ((unsigned char)sub_C863(v25, *v6) + 46LL));
          if ( v17 )
            break;
          if ( (unsigned long long)v19 >= v26 )
            return -1;
        }
      }
LABEL_14:
      while ( 2 )
      {
        if ( *v17 )
          goto LABEL_33;
        if ( (unsigned long long)v19 >= v26 )
          return -1;
        v8 = (unsigned char *)v19++;
        v14 = sub_C863(v25, *v8);
        for ( m = v17[1]; ; m = v17[1] )
        {
          do
          {
            if ( v14 == *(char *)(m + 24) )
            {
              v17 = *(long long **)(m + 16);
              goto LABEL_14;
            }
            if ( v14 >= *(char *)(m + 24) )
              v9 = *(long long *)(m + 8);
            else
              v9 = *(long long *)m;
            m = v9;
          }
          while ( v9 );
          v17 = (long long *)v17[4];
          if ( !v17 )
            break;
          if ( *v17 )
          {
            --v19;
            goto LABEL_33;
          }
        }
        v17 = *(long long **)(a1 + 8 * (v14 + 46LL));
        if ( v17 )
          continue;
        break;
      }
      if ( (unsigned long long)v19 >= v26 )
        return -1;
    }
  }
}



// Function: acexec @ 0xf50c
long long acexec(long long a1, char *a2, long long a3, long long *a4, char a5)
{
  return sub_F0F4(a1, a2, a3, a4, a5);
}



// Function: kwsexec @ 0xf585
long long kwsexec(long long a1, long long a2, long long a3, long long a4, unsigned char a5)
{
  return (*(long long (**)(long long, long long, long long, long long, long long))(a1 + 2456))(a1, a2, a3, a4, a5);
}



// Function: kwsfree @ 0xf5d1
void kwsfree(struct obstack *a1)
{
  obstack_free(a1, 0);
  free(a1);
}



// Function: wordchar @ 0xf73c
long long wordchar(wint_t a1)
{
  return a1 == 95 || iswalnum(a1);
}



// Function: wordinit @ 0xf770
void wordinit()
{
  int i; // [rsp+Ch] [rbp-4h]

  for ( i = 0; i <= 255; ++i )
    byte_339E0[i] = sub_F73C(dword_331A4[i + 64]);
}



// Function: kwsinit @ 0xf7c8
long long kwsinit(char a1)
{
  int c; // [rsp+14h] [rbp-Ch]
  long long v3; // [rsp+18h] [rbp-8h]

  v3 = 0;
  if ( byte_33180 && (__ctype_get_mb_cur_max() == 1 || a1) )
  {
    v3 = sub_26B10(256);
    for ( c = 0; c <= 255; ++c )
      *(char *)(c + v3) = toupper(c);
  }
  return sub_C89B(v3);
}



// Function: mb_goback @ 0xf84a
char *mb_goback(unsigned long long *a1, long long *a2, char *a3, long long a4)
{
  int i; // [rsp+2Ch] [rbp-34h]
  unsigned long long v8; // [rsp+30h] [rbp-30h]
  char *v9; // [rsp+38h] [rbp-28h]
  long long v10; // [rsp+40h] [rbp-20h]
  long long v11; // [rsp+48h] [rbp-18h]
  long long s[2]; // [rsp+50h] [rbp-10h] BYREF

  s[1] = __readfsqword(0x28u);
  v8 = *a1;
  v9 = (char *)*a1;
  if ( (unsigned long long)a3 <= *a1 )
    return &a3[-v8];
  if ( byte_331A2 )
  {
    v8 = (unsigned long long)a3;
    if ( (*a3 & 0xC0) == 0x80 )
    {
      for ( i = 1; i <= 3; ++i )
      {
        if ( (a3[-i] & 0xC0) != 0x80 )
        {
          if ( !((int)(unsigned char)~a3[-i] >> (7 - i)) )
          {
            s[0] = 0;
            v11 = sub_F686(&a3[-i], a4 - (long long)&a3[-i], s);
            if ( v11 >= 0 )
            {
              v9 = &a3[-i];
              v8 = (unsigned long long)&v9[v11];
            }
          }
          break;
        }
      }
    }
  }
  else
  {
    s[0] = 0;
    do
    {
      v10 = sub_F6DA(v8, a4 - v8, s);
      if ( v10 < 0 )
      {
        v10 = 1;
        memset(s, 0, 8u);
      }
      v9 = (char *)v8;
      v8 += v10;
    }
    while ( v8 < (unsigned long long)a3 );
    if ( a2 )
      *a2 = v10;
  }
  *a1 = v8;
  if ( (char *)v8 == a3 )
    return 0;
  else
    return (char *)(a3 - v9);
}



// Function: wordchars_count @ 0xfa82
long long wordchars_count(long long a1, long long a2, char a3)
{
  unsigned char v5; // [rsp+2Bh] [rbp-25h]
  wint_t v6; // [rsp+2Ch] [rbp-24h] BYREF
  long long v7; // [rsp+30h] [rbp-20h]
  long long v8; // [rsp+38h] [rbp-18h]
  long long v9[2]; // [rsp+40h] [rbp-10h] BYREF

  v9[1] = __readfsqword(0x28u);
  v7 = 0;
  v9[0] = 0;
  while ( v7 < a2 - a1 )
  {
    v5 = *(char *)(v7 + a1);
    if ( byte_339E0[v5] )
    {
      ++v7;
    }
    else
    {
      if ( *((char *)&unk_331A3 + v5) != 0xFE )
        return v7;
      v6 = 0;
      v8 = sub_21B80(&v6, v7 + a1, a2 - a1 - v7, v9);
      if ( !sub_F73C(v6) )
        return v7;
      v7 += v8 + (v8 == 0);
    }
    if ( a3 != 1 )
      return v7;
  }
  return v7;
}



// Function: wordchars_size @ 0xfbb2
long long wordchars_size(long long a1, long long a2)
{
  return sub_FA82(a1, a2, 1);
}



// Function: wordchar_next @ 0xfbe0
long long wordchar_next(long long a1, long long a2)
{
  return sub_FA82(a1, a2, 0);
}



// Function: wordchar_prev @ 0xfc0e
long long wordchar_prev(unsigned long long a1, long long a2, long long a3)
{
  char *v4; // rax
  char *v6; // [rsp+10h] [rbp-30h]
  unsigned char v7; // [rsp+2Fh] [rbp-11h]
  unsigned long long v8[2]; // [rsp+30h] [rbp-10h] BYREF

  v8[1] = __readfsqword(0x28u);
  if ( a1 == a2 )
    return 0;
  v6 = (char *)(a2 - 1);
  v7 = *(char *)(a2 - 1);
  if ( byte_331A0 != 1 || ((unsigned char)byte_331A2 & ~(v7 >> 7)) != 0 )
    return (unsigned char)byte_339E0[v7];
  v8[0] = a1;
  v4 = sub_F84A(v8, 0, v6, a3);
  return sub_FBE0(v6 - v4, a3);
}



// Function: private_malloc @ 0xfd0f
long long private_malloc(long long a1)
{
  if ( a1 < 0 )
    sub_27122(a1);
  return sub_26B10(a1);
}



// Function: private_free @ 0xfd3f
void private_free(void *a1)
{
  free(a1);
}



// Function: jit_exec @ 0xfd62
long long jit_exec(long long *a1, long long a2, long long a3, long long a4, unsigned int a5)
{
  long long v5; // rdi
  bool v6; // cl
  int v12[2]; // [rsp+34h] [rbp-2Ch] BYREF
  unsigned int matched; // [rsp+3Ch] [rbp-24h]
  long long v14; // [rsp+40h] [rbp-20h]
  long long v15; // [rsp+48h] [rbp-18h]
  long long v16; // [rsp+50h] [rbp-10h]
  unsigned long long v17; // [rsp+58h] [rbp-8h]

  v17 = __readfsqword(0x28u);
  while ( 1 )
  {
    while ( 1 )
    {
      v12[1] = 0x2000;
      v14 = 0x7FFFFFFFFFFFFFFFLL;
      matched = pcre2_match_8(a1[1], a2, a3, a4, a5, a1[3], a1[2]);
      if ( matched != -46 || a1[5] > v14 / 2 )
        break;
      v15 = a1[5];
      a1[5] = 2 * v15;
      v16 = a1[5];
      pcre2_jit_stack_free_8(a1[4]);
      v5 = v15;
      a1[4] = pcre2_jit_stack_create_8(v15, v16, *a1);
      if ( !a1[4] )
        sub_27122(v5);
      if ( !a1[2] )
        a1[2] = pcre2_match_context_create_8(*a1);
      pcre2_jit_stack_assign_8(a1[2], 0, a1[4]);
    }
    if ( matched != -53 )
      break;
    pcre2_config_8(7, v12);
    v6 = v12[0] < 0;
    v12[0] *= 2;
    if ( v6 )
      return matched;
    if ( !a1[2] )
      a1[2] = pcre2_match_context_create_8(*a1);
    pcre2_set_depth_limit_8(a1[2], v12[0]);
  }
  return matched;
}



// Function: bad_utf8_from_pcre2 @ 0xff6e
long long bad_utf8_from_pcre2()
{
  return 0;
}



// Function: Pcompile @ 0xff80
long long *Pcompile(char *a1, size_t a2)
{
  int v2; // eax
  char *v3; // rax
  char *v4; // rax
  void *v5; // rax
  long long v6; // rax
  long long v7; // rax
  long long v8; // rax
  unsigned int v9; // ebx
  char *v10; // rax
  int v11; // eax
  int v12; // eax
  size_t n; // [rsp+10h] [rbp-180h]
  void *s; // [rsp+18h] [rbp-178h]
  unsigned int v16; // [rsp+20h] [rbp-170h] BYREF
  unsigned int v17; // [rsp+24h] [rbp-16Ch]
  char v18[8]; // [rsp+28h] [rbp-168h] BYREF
  void *ptr; // [rsp+30h] [rbp-160h]
  char *v20; // [rsp+38h] [rbp-158h]
  long long *v21; // [rsp+40h] [rbp-150h]
  long long v22; // [rsp+48h] [rbp-148h]
  long long v23; // [rsp+50h] [rbp-140h]
  size_t v24; // [rsp+58h] [rbp-138h]
  void *dest; // [rsp+60h] [rbp-130h]
  void *v26; // [rsp+68h] [rbp-128h]
  char v27[264]; // [rsp+70h] [rbp-120h] BYREF
  unsigned long long v28; // [rsp+178h] [rbp-18h]

  s = a1;
  n = a2;
  v28 = __readfsqword(0x28u);
  if ( byte_33180 )
    v2 = 24;
  else
    v2 = 16;
  v17 = v2;
  v20 = &a1[a2];
  v21 = (long long *)sub_26B10(56);
  *v21 = pcre2_general_context_create_8(sub_FD0F, sub_FD3F, 0);
  v22 = *v21;
  v23 = pcre2_compile_context_create_8(v22);
  if ( byte_331A0 )
  {
    if ( byte_331A2 != 1 )
    {
      v3 = gettext("-P supports only unibyte and UTF-8 locales");
      error(2, 0, v3);
    }
    v17 |= 0x4080000u;
  }
  if ( v20 != rawmemchr(a1, 10) )
  {
    v4 = gettext("the -P option only supports a single pattern");
    error(2, 0, v4);
  }
  ptr = 0;
  if ( byte_33182 )
  {
    pcre2_set_compile_extra_options_8(v23, 8);
  }
  else if ( byte_33181 )
  {
    v24 = a2 + 17;
    ptr = (void *)sub_26B10(a2 + 17);
    dest = ptr;
    v5 = mempcpy(ptr, "(?<!\\w)(?:)(?!\\w)invalid argument %s for %s", 0xAu);
    v26 = mempcpy(v5, a1, a2);
    memcpy(v26, ")(?!\\w)invalid argument %s for %s", 7u);
    s = ptr;
    n = a2 + 17;
  }
  v6 = pcre2_maketables_8(v22);
  pcre2_set_character_tables_8(v23, v6);
  v7 = pcre2_compile_8(s, n, v17, &v16, v18, v23);
  v21[1] = v7;
  if ( !v21[1] )
  {
    pcre2_get_error_message_8(v16, v27, 256);
    error(2, 0, "%s", v27);
  }
  free(ptr);
  pcre2_compile_context_free_8(v23);
  v21[2] = 0;
  v8 = pcre2_match_data_create_from_pattern_8(v21[1], v22);
  v21[3] = v8;
  v16 = pcre2_jit_compile_8(v21[1], 1);
  if ( v16 && v16 != -45 && v16 != -48 )
  {
    v9 = v16;
    v10 = gettext("JIT internal error: %d");
    error(2, 0, v10, v9);
  }
  v21[4] = 0;
  v21[5] = 0x8000;
  v11 = sub_FD62(v21, (long long)&unk_2A907, 0, 0, 1u);
  *((int *)v21 + 12) = v11;
  v12 = sub_FD62(v21, (long long)&unk_2A907, 0, 0, 0);
  *((int *)v21 + 13) = v12;
  return v21;
}



// Function: Pexecute @ 0x10409
long long Pexecute(long long *a1, long long a2, long long a3, long long *a4, char *a5)
{
  char *v5; // rax
  long long v6; // rbx
  char *v7; // rax
  long long v8; // rbx
  char *v9; // rax
  long long v10; // rbx
  char *v11; // rax
  long long v12; // rbx
  char *v13; // rax
  long long v14; // rbx
  char *v15; // rax
  long long v16; // rbx
  char *v17; // rax
  long long v18; // rbx
  char *v19; // rax
  bool v24; // [rsp+37h] [rbp-79h]
  int v25; // [rsp+38h] [rbp-78h]
  unsigned int v26; // [rsp+3Ch] [rbp-74h]
  char *s; // [rsp+40h] [rbp-70h]
  char *v28; // [rsp+48h] [rbp-68h]
  long long v29; // [rsp+50h] [rbp-60h]
  long long v30; // [rsp+58h] [rbp-58h]
  char *v31; // [rsp+60h] [rbp-50h]
  long long *ovector_pointer_8; // [rsp+70h] [rbp-40h]
  char *v33; // [rsp+78h] [rbp-38h]
  long long v34; // [rsp+80h] [rbp-30h]
  long long startchar_8; // [rsp+88h] [rbp-28h]

  if ( a5 )
    v5 = a5;
  else
    v5 = (char *)a2;
  s = v5;
  v24 = *(v5 - 1) == byte_33183;
  v28 = (char *)a2;
  ovector_pointer_8 = (long long *)pcre2_get_ovector_pointer_8(a1[3]);
  v29 = a2;
  do
  {
    v33 = (char *)rawmemchr(s, byte_33183);
    while ( 1 )
    {
      while ( *((char *)&unk_331A3 + (unsigned char)sub_F64D((unsigned int)*s)) == 0xFF )
      {
        v29 = (long long)++s;
        v24 = 0;
      }
      v34 = (long long)&s[-v29];
      if ( s == v33 )
        break;
      v26 = !v24;
      v25 = sub_FD62(a1, v29, (long long)&v33[-v29], v34, v26);
      if ( (unsigned char)sub_FF6E((unsigned int)v25) != 1 )
        goto LABEL_18;
      startchar_8 = pcre2_get_startchar_8(a1[3]);
      if ( v34 <= startchar_8 )
      {
        if ( startchar_8 )
        {
          v25 = sub_FD62(a1, v29, startchar_8, v34, v26 | 0x40000002);
        }
        else
        {
          *ovector_pointer_8 = 0;
          ovector_pointer_8[1] = 0;
          v25 = *((int *)a1 + v24 + 12);
        }
        if ( v25 != -1 )
          goto LABEL_18;
        s = (char *)(startchar_8 + 1 + v29);
        v24 = 0;
      }
      v29 += startchar_8 + 1;
    }
    ovector_pointer_8[1] = v34;
    *ovector_pointer_8 = ovector_pointer_8[1];
    v25 = *((int *)a1 + v24 + 12);
LABEL_18:
    if ( v25 != -1 )
      break;
    v24 = 1;
    v28 = v33 + 1;
    v29 = (long long)(v33 + 1);
    s = v33 + 1;
  }
  while ( (unsigned long long)(v33 + 1) < a3 + a2 );
  if ( v25 <= 0 )
  {
    if ( v25 > -46 )
    {
      if ( v25 == -1 )
        return -1;
    }
    else if ( v25 >= -63 )
    {
      switch ( v25 )
      {
        case -63:
          goto LABEL_31;
        case -53:
          goto LABEL_29;
        case -52:
          goto LABEL_30;
        case -48:
          v6 = sub_6EAC();
          v7 = gettext("%s: memory exhausted");
          error(2, 0, v7, v6);
          goto LABEL_27;
        case -47:
          goto LABEL_28;
        case -46:
LABEL_27:
          v8 = sub_6EAC();
          v9 = gettext("%s: exhausted PCRE JIT stack");
          error(2, 0, v9, v8);
LABEL_28:
          v10 = sub_6EAC();
          v11 = gettext("%s: exceeded PCRE's backtracking limit");
          error(2, 0, v11, v10);
LABEL_29:
          v12 = sub_6EAC();
          v13 = gettext("%s: exceeded PCRE's nested backtracking limit");
          error(2, 0, v13, v12);
LABEL_30:
          v14 = sub_6EAC();
          v15 = gettext("%s: PCRE detected recurse loop");
          error(2, 0, v15, v14);
LABEL_31:
          v16 = sub_6EAC();
          v17 = gettext("%s: exceeded PCRE's heap limit");
          error(2, 0, v17, v16);
          break;
        default:
          break;
      }
    }
    v18 = sub_6EAC();
    v19 = gettext("%s: internal PCRE error: %d");
    error(2, 0, v19, v18, (unsigned int)v25);
    return -1;
  }
  if ( a5 )
  {
    v30 = *ovector_pointer_8 + v29;
    v31 = (char *)(ovector_pointer_8[1] + v29);
  }
  else
  {
    v30 = (long long)v28;
    v31 = v33 + 1;
  }
  *a4 = &v31[-v30];
  return v30 - a2;
}


