// Function: sub_513f @ 0x513f
void sub_513f(char *a0)
{
  error(2,0,"%s",a0);
}


// Function: sub_5174 @ 0x5174
void sub_5174(char *a0)
{
  error(0,0,gettext("warning: %s"),a0);
}


// Function: sub_51b4 @ 0x51b4
void sub_51b4(struct_26 *a0) // return-dupe
{
  char v1;
  char *v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // rax
  unsigned char *v5; // rax
  
  v2 = (char *)sub_1ae2f(a0->field_0x8);
  if (!v2)
    return;
  a0->field_0x0 = (void *)sub_f7c8(0);
  if (*v2) { // branch-flip
    a0->field_0x38 = a0->field_0x38 + 1;
    v3 = strlen(&v2[3]);
    v4 = (unsigned long)(unsigned char)v2[2] + (unsigned char)v2[1] + v3;
    v5 = (unsigned char *)sub_26b10(v4);
    *v5 = dat_33183;
    v1 = v2[1];
    a0->field_0x40 = v2[1] || a0->field_0x40;
    memcpy(&v5[(unsigned char)v1],&v2[3],v3);
    if (v2[2])
      (&v5[(unsigned char)v1])[v3] = dat_33183;
    sub_cacb(a0->field_0x0,v5,v4);
    free(v5);
  }
  else {
    v3 = strlen(&v2[3]);
    sub_cacb(a0->field_0x0,&v2[3],v3);
  }
  sub_d9b5(a0->field_0x0);
  sub_1bb28(v2);
}


// Function: sub_5354 @ 0x5354
unsigned long sub_5354(void *a0,long a1,bool a2) // return-dupe, ternary
{
  int v1; // eax
  void *v2; // rax
  void *v3; // stack - 0x18
  
  v1 = (a2) ? 0x5c : 0x80; // branch-flip
  if (a1 + -1 <= -1)
    return 0;
  v2 = (void *)((long)a0 + a1 + -1);
  v3 = a0;
  while( true ) {
    v3 = memchr(v3,0x5c,(long)v2 - (long)v3);
    if (!v3)
      return 0;
    if (('1' <= *(char *)((long)v3 + 1)) && (*(char *)((long)v3 + 1) <= '9')) break;
    if ((v1 == *(char *)((long)v3 + 1)) && (v3 = (void *)((long)v3 + 1), v3 == v2))
      return 0;
    v3 = (void *)((long)v3 + 1);
  }
  return 1;
}


// Function: sub_5428 @ 0x5428
unsigned long sub_5428(long a0,char *a1,unsigned long a2,long a3,long a4,unsigned long a5,unsigned int a6) // return-dupe, return-dupe x2, ternary x2
{
  char v1; // al
  undefined8 v10; // stack - 0x30
  unsigned long *v2; // rax
  char v3 [40]; // stack - 0x68
  long v4 [2]; // stack - 0x90
  char *v5; // stack - 0x80
  char *v6; // stack - 0x78
  long v7; // stack - 0x70
  undefined8 v8; // stack - 0x40
  undefined8 v9; // stack - 0x38
  
  v1 = (char)a6;
  v3._0_8_ = 0;
  v3._8_8_ = 0;
  v4[1] = 0xff;
  v3._32_8_ = (dat_33180 || v1) ? NULL : (void *)sub_26b10(0x100); // branch-flip
  v8 = 0;
  if (v1) // branch-flip
    re_set_syntax(a5 | 0x2000000);
  else {
    re_set_syntax(a5);
  }
  v5 = re_compile_pattern(a1,a2,(re_pattern_buffer *)v3);
  if (v5) {
    free(v3._32_8_);
    v6 = (0 <= a4) ? (char *)sub_6a2e(a4,v4) : ""; // branch-flip
    if (!*v6) {
      error(0,0,"%s",v5);
      return 0;
    }
    v7 = v4[0];
    error(0,0,"%s:%td: %s",v6,v4[0],v5);
    return 0;
  }
  if (v1) {
    regfree((re_pattern_buffer *)v3);
    return 1;
  }
  v2 = (unsigned long *)(a3 * 0x40 + *(long *)(a0 + 0x10));
  *v2 = v3._0_8_;
  v2[1] = v3._8_8_;
  v2[2] = v3._16_8_;
  v2[3] = v3._24_8_;
  v2[4] = v3._32_8_;
  v2[5] = v8;
  v2[6] = v9;
  v2[7] = v10;
  return 1;
}


// Function: sub_5661 @ 0x5661
long sub_5661(char *a0,unsigned long a1,unsigned long a2,bool a3) // ternary x5
{
  long v1;
  long v10; // stack - 0xa8
  bool v11;
  unsigned long v12; // stack - 0xd0
  unsigned long v13; // stack - 0xc8
  char *v14; // stack - 0xc0
  char *v15; // stack - 0x98
  char *v16; // stack - 0x90
  char *v17; // stack - 0x88
  char *v18; // stack - 0x80
  unsigned long v19; // stack - 0x78
  bool v2;
  long v20; // stack - 0x70
  long v21; // stack - 0x68
  char *v22; // stack - 0x60
  void *v23; // stack - 0x58
  unsigned long v24; // stack - 0x50
  unsigned long v25; // stack - 0x48
  long v26; // stack - 0x40
  long v27; // stack - 0x38
  unsigned long v28; // stack - 0x30
  unsigned long v29; // stack - 0x28
  bool v3;
  char *v30; // stack - 0x20
  unsigned long v31; // stack - 0x18
  char v4; // al
  char v5; // al
  unsigned int v6; // eax
  unsigned long v7; // rax
  char *v8; // rax
  long v9; // stack - 0xa0
  
  v21 = sub_26fc8(1,0x48);
  v7 = sub_1bb47();
  *(unsigned long *)(v21 + 8) = v7;
  v12 = a2;
  if (dat_33180)
    v12 = a2 | 0x400000;
  v6 = (dat_33183) ? 0 : 2; // branch-flip
  sub_1bb5b(*(void **)(v21 + 8),(char *)0x331a0,v12,v6 | (unsigned int)v12 & 0x10 | 0x2c);
  v11 = dat_331a2 != '\0';
  v2 = dat_331a0 != '\x01';
  v22 = &a0[a1];
  v3 = 0;
  v16 = a0;
  v7 = sub_26aea(0x40);
  *(unsigned long *)(v21 + 0x10) = v7;
  *(long *)(v21 + 0x10) = *(long *)(v21 + 0x10) + 0x40;
  *(unsigned long *)(v21 + 0x18) = 0;
  v10 = 1;
  v18 = NULL;
  v19 = 0;
  v9 = 0;
  v20 = 0;
  v17 = a0;
  do {
    v23 = rawmemchr(v16,10);
    v24 = (long)v23 - (long)v16;
    v4 = sub_5354(v16,v24,v11 || v2);
    if ((v4) && (v17 < v16)) {
      v25 = (long)v16 - (long)v17;
      v26 = v25 + (v19 - v9);
      if (1 <= v26)
        v18 = (char *)sub_26dc6(v18,&v9,v26,-1,1);
      memcpy(&v18[v19],v17,v25);
      v19 += v25;
    }
    v27 = (*(long *)(v21 + 0x18) - v10) + 2;
    if (1 <= v27) {
      v7 = sub_26dc6((void *)(*(long *)(v21 + 0x10) + -0x40),&v10,v27,-1,0x40);
      *(unsigned long *)(v21 + 0x10) = v7;
      *(long *)(v21 + 0x10) = *(long *)(v21 + 0x10) + 0x40;
    }
    re_set_syntax(v12);
    v5 = sub_5428(v21,v16,v24,*(long *)(v21 + 0x18),v20,v12,v4 == '\0');
    if (v5 != '\x01')
      v3 = 1;
    v16 = (char *)((long)v23 + 1);
    v20 += 1;
    if (v4) {
      *(long *)(v21 + 0x18) = *(long *)(v21 + 0x18) + 1;
      v17 = v16;
    }
  } while (v16 <= v22);
  if (!v3) {
    if (v17 <= v22) {
      if (v17 <= a0) { // branch-flip
        v18 = a0;
        v19 = a1;
      }
      else {
        v28 = (long)v22 - (long)v17;
        v18 = (char *)sub_26b9f(v18,v19 + v28);
        memcpy(&v18[v19],v17,v28);
        v19 += v28;
      }
    }
    if ((dat_33181) || (dat_33182)) {
      v11 = (v12 & 0x2000) != 0;
      v29 = 0x2d;
      v30 = (char *)sub_26b10(a1 + 0x2d);
      if (dat_33182) { // branch-flip
        v8 = (v11) ? "^(" : "^\\(";
      }
      else {
        v8 = (v11) ? "(^|[^[:alnum:]_])(" : "\\(^\\|[^[:alnum:]_]\\)\\(";
      }
      strcpy(v30,v8);
      v31 = strlen(v30);
      memcpy(&v30[v31],a0,a1);
      v31 += a1;
      if (dat_33182) { // branch-flip
        v8 = (v11) ? ")$" : "\\)$";
      }
      else {
        v8 = (v11) ? ")([^[:alnum:]_]|$)" : "\\)\\([^[:alnum:]_]\\|$\\)";
      }
      strcpy(&v30[v31],v8);
      v8 = &v30[v31];
      v13 = strlen(v8) + v31;
      v15 = v30;
      v14 = v30;
      v31 = v13;
    }
    else {
      v15 = NULL;
      v13 = a1;
      v14 = a0;
    }
    sub_14b84(v14,v13,*(long *)(v21 + 8));
    sub_51b4(v21);
    sub_1a490(0,0,*(long *)(v21 + 8),1);
    if (v18) {
      if ((a3) || (v1 = *(long *)(v21 + 8), sub_19e86(v1) != '\x01')) {
        *(long *)(v21 + 0x10) = *(long *)(v21 + 0x10) + -0x40;
        *(long *)(v21 + 0x18) = *(long *)(v21 + 0x18) + 1;
        v4 = sub_5428(v21,v18,v19,0,-1,v12,0);
        if (v4 != '\x01')
          abort(); // no-return
      }
      if (v18 != v14)
        free(v18);
    }
    free(v15);
    return v21;
  }
  exit(2); // no-return
}


// Function: sub_5d23 @ 0x5d23
long sub_5d23(long *a0,char *a1,long a2,long *a3,char *a4)
{
  void *v1; // rax
  char v10; // stack - 0xbc
  char v11; // stack - 0xbb
  char v12; // stack - 0xba
  char v13; // stack - 0xb9
  int v14; // stack - 0xb8
  int v15; // stack - 0xb4
  char *v16; // stack - 0xa0
  char *v17; // stack - 0x98
  char *v18; // stack - 0x90
  char *v19; // stack - 0x88
  char *v2;
  char *v20; // stack - 0x80
  char *v21; // stack - 0x78
  char *v22; // stack - 0x70
  long v23; // stack - 0x68
  char *v24; // stack - 0x60
  long *v25; // stack - 0x58
  long v26; // stack - 0x50
  char *v27; // stack - 0x48
  long v28; // stack - 0x40
  char *v29; // stack - 0x38
  unsigned long v3; // rax
  char *v30; // stack - 0x30
  long v4;
  char *v5; // rax
  int v6; // ecx
  long v7 [3]; // stack - 0x28
  char *v8; // stack - 0xb0
  long v9; // stack - 0xa8
  
  v12 = dat_33183;
  v4 = a0[1];
  v25 = a0;
  v26 = sub_19da3(v4);
  v4 = v25[1];
  v13 = sub_19dbc(v4);
  v27 = &a1[a2];
  v8 = a1;
  v16 = a1;
  v17 = a1;
  do {
    if (v27 <= v17)
      return -1;
    v17 = v27;
    if (a4) { // branch-flip
      v18 = a4;
label_6255:
      if (0x80000001 <= (long)v17 - (long)v16)
        sub_27122(); // no-return
      v20 = v17;
      v22 = NULL;
      for (v23 = 0; v23 < v25[3]; v23 = v23 + 1) {
        v4 = v23 * 0x40 + v25[2];
        *(unsigned char *)(v4 + 0x38) = *(unsigned char *)(v4 + 0x38) & 0xbf;
        v4 = v23 * 0x40 + v25[2];
        *(unsigned char *)(v4 + 0x38) = *(unsigned char *)(v4 + 0x38) & 0x7f | (dat_33183 == '\n') << 7;
        v15 = re_search((re_pattern_buffer *)(v25[2] + v23 * 0x40),v16,((int)v17 - (int)v16) + -1,(int)v18 - (int)v16,((int)v17 - (int)v18) + -1,&v25[4]);
        if (v15 <= -2)
          sub_27122(); // no-return
        if (0 <= v15) {
          v21 = (char *)(long)(*(int *)v25[6] - v15);
          v19 = &v16[v15];
          if (v19 <= v20) {
            if ((!a4) || (dat_33181 == '\x01')) {
              if (((dat_33182 == '\x01') || (dat_33181 == '\x01')) && ((!dat_33182 || (v21 != &v17[0xffffffffffffffff - (long)v18])))) {
                if ((dat_33182 != '\x01') && (dat_33181)) {
                  while (v19 <= v20) {
                    v14 = 0;
                    v2 = &v17[-1];
                    v5 = &v19[(long)v21];
                    if ((!sub_fbe0(v5,v2)) && (v2 = &v17[-1], !sub_fc0e(v16,v19,v2))) goto label_6645;
                    if (1 <= (long)v21) {
                      v21 = &v21[-1];
                      v4 = v23 * 0x40 + v25[2];
                      *(unsigned char *)(v4 + 0x38) = *(unsigned char *)(v4 + 0x38) | 0x40;
                      v14 = re_match((re_pattern_buffer *)(v25[2] + v23 * 0x40),v16,((int)v19 + (int)v21) - (int)v18,(int)v19 - (int)v16,&v25[4]);
                      if (v14 <= -2)
                        sub_27122(); // no-return
                    }
                    if (1 <= v14) // branch-flip
                      v21 = (char *)(long)v14;
                    else {
                      if (v19 == &v17[-1]) break;
                      v19 = &v19[1];
                      v4 = v23 * 0x40 + v25[2];
                      *(unsigned char *)(v4 + 0x38) = *(unsigned char *)(v4 + 0x38) & 0xbf;
                      v15 = re_search((re_pattern_buffer *)(v25[2] + v23 * 0x40),v16,((int)v17 - (int)v16) + -1,(int)v19 - (int)v16,((int)v17 - (int)v19) + -1,&v25[4]);
                      if (v15 <= -1) {
                        if (v15 <= -2)
                          sub_27122(); // no-return
                        break;
                      }
                      v21 = (char *)(long)(*(int *)v25[6] - v15);
                      v19 = &v16[v15];
                    }
                  }
                }
                goto label_6687;
              }
              v19 = v18;
              v21 = &v17[-(long)v18];
            }
label_6645:
            if (!a4) goto label_66f6;
            if ((v19 < v20) || ((v19 == v20 && ((long)v22 < (long)v21)))) {
              v20 = v19;
              v22 = v21;
            }
          }
        }
label_6687:
      }
      if (v20 < v17) {
        v16 = v20;
        v21 = v22;
        goto label_6708;
      }
    }
    else {
      v24 = v16;
      v9 = 0;
      v11 = '\0';
      v10 = '\0';
      if (*v25) {
        v28 = sub_f585(*v25,(long)v16 - (unsigned long)*(unsigned char *)&v25[8],&v27[(unsigned long)*(unsigned char *)&v25[8] - (long)v16],v7,1);
        if (v28 <= -1)
          return v28;
        v19 = &v16[v28];
        v29 = v16;
        v1 = memrchr(a1,(int)v12,(long)v19 - (long)a1);
        if (v1) // branch-flip
          v16 = (char *)((long)v1 + 1);
        else {
          v16 = a1;
        }
        v11 = v7[0] < v25[7];
        v24 = v16;
        if (((bool)v11) || (v13 != '\x01')) {
label_5f74:
          v6 = (int)v12;
          v17 = (char *)((long)rawmemchr(v19,v6) + 1);
        }
        else {
          v4 = 0x10;
          if (0x10 <= (long)v19 - (long)v16)
            v4 = (long)v19 - (long)v16;
          if (v4 < (long)v19 - (long)v29 >> 2) goto label_5f74;
          v4 = 0x10;
          if (0x10 <= (long)v19 - (long)v16)
            v4 = (long)v19 - (long)v16;
          if ((long)v27 - (long)v29 >> 2 <= v4) // branch-flip
            v17 = v27;
          else {
            v6 = (int)v12;
            v4 = (long)v19 - (long)v16;
            if (v4 <= 0xf)
              v4 = 0x10;
            v2 = &v29[v4 * 4];
            v17 = (char *)((long)rawmemchr(v2,v6) + 1);
          }
        }
        if (!v11) goto label_6088;
        if (!dat_331a2 && dat_331a0 == '\x01') {
          if (v8 < v16)
            v8 = v16;
          if (sub_f84a(&v8,NULL,v19,v27)) {
            v24 = v8;
            goto label_6088;
          }
        }
label_66f6:
        v21 = &v17[-(long)v16];
label_6708:
        *a3 = (long)v21;
        return (long)v16 - (long)a1;
      }
label_6088:
      if ((v26) && (v11 != '\x01')) {
        v30 = (char *)sub_19d4d(v26,v24,v17,0,&v9,0);
        if ((v30) && (v30 != v17)) {
          if (v9) {
            v3 = (long)v30 - (long)a1;
            v6 = (int)v12;
            v16 = (char *)((long)memrchr(a1,v6,v3) + 1);
            v24 = v16;
          }
          v6 = (int)v12;
          v17 = (char *)((long)rawmemchr(v30,v6) + 1);
          v9 = 0;
          goto label_616c;
        }
      }
      else {
label_616c:
        v30 = (char *)sub_19d4d(v25[1],v24,v17,0,&v9,&v10);
        if ((v30) && (v30 != v17)) {
          if (v9) {
            v3 = (long)v30 - (long)a1;
            v6 = (int)v12;
            v16 = (char *)((long)memrchr(a1,v6,v3) + 1);
          }
          v6 = (int)v12;
          v17 = (char *)((long)rawmemchr(v30,v6) + 1);
          if (v10 == '\x01') {
            v18 = v16;
            goto label_6255;
          }
          goto label_66f6;
        }
      }
    }
    v16 = v17;
  } while( true );
}


// Function: sub_6760 @ 0x6760
unsigned long sub_6760(long a0,unsigned long a1)
{
  unsigned long v1; // stack - 0x38
  unsigned char *v2; // stack - 0x30
  
  v1 = 0x3938a0565d85cb03;
  for (v2 = (unsigned char *)(a0 + -1 + dat_33780); *v2 != 10; v2 = &v2[1]) {
    v1 = (unsigned long)*v2 ^ v1 * 0x21;
  }
  return v1 % a1;
}


// Function: sub_67f8 @ 0x67f8
unsigned long sub_67f8(long a0,long a1)
{
  char *v1; // stack - 0x28
  char *v2; // stack - 0x20
  
  v1 = (char *)(a0 + -1 + dat_33780);
  v2 = (char *)(a1 + -1 + dat_33780);
  while( true ) {
    if (*v1 != *v2)
      return 0;
    if (*v1 == '\n') break;
    v1 = &v1[1];
    v2 = &v2[1];
  }
  return 1;
}


// Function: sub_687b @ 0x687b
long sub_687b(long a0,long a1,long a2,long a3)
{
  long v1;
  int v2; // eax
  unsigned long v3; // rax
  long *v4; // rax
  int v5; // stack - 0x68
  void *v6; // stack - 0x60
  long v7; // stack - 0x58
  void *v8; // stack - 0x50
  
  v6 = (void *)(a0 + a1);
  v7 = 1;
  v5 = 0;
  v8 = (void *)(a0 + a1);
  do {
    if ((void *)(a0 + a2) <= v8)
      return (long)v6 - a0;
    v3 = (long)rawmemchr(v8,10) + (1U - (long)v8);
    memmove(v6,v8,v3);
    v2 = sub_212a6(dat_33790,(long)v6 + (1U - a0),NULL);
    if (v2) {
      if (v2 <= -1)
        sub_27122(); // no-return
      v6 = (void *)((long)v6 + v3);
      if (!v5) {
        if (dat_33778 == dat_33770)
          dat_33768 = sub_26dc6(dat_33768,(long *)0x33770,1,-1,0x18);
        v1 = dat_33778 + 1;
        v4 = (long *)(dat_33778 * 0x18 + dat_33768);
        *v4 = dat_33788;
        v4[1] = a3;
        v4[2] = v7;
        dat_33778 = v1;
      }
      dat_33788 += 1;
    }
    v5 = v2;
    v7 += 1;
    v8 = (void *)((long)v8 + v3);
  } while( true );
}


// Function: sub_6a2e @ 0x6a2e
unsigned long sub_6a2e(long a0,long *a1)
{
  long v1; // stack - 0x10
  
  for (v1 = 1; (v1 < dat_33778 && (*(long *)(v1 * 0x18 + dat_33768) <= a0)); v1 = v1 + 1) {
  }
  *a1 = (a0 - *(long *)(v1 * 0x18 + -0x18 + dat_33768)) + *(long *)(v1 * 0x18 + dat_33768 + -8);
  return *(unsigned long *)(v1 * 0x18 + dat_33768 + -0x10);
}


// Function: sub_6b00 @ 0x6b00
void sub_6b00(void)
{
  return;
}


// Function: sub_6b0b @ 0x6b0b
void sub_6b0b(void)
{
  return;
}


// Function: sub_6b1e @ 0x6b1e
void sub_6b1e(char *a0) // return-dupe
{
  if (!*a0)
    return;
  sub_276e4(dat_33068,a0);
}


// Function: sub_6b52 @ 0x6b52
void sub_6b52(char *a0) // return-dupe
{
  if (!*a0)
    return;
  sub_27713(dat_33070);
}


// Function: sub_6b7f @ 0x6b7f
void sub_6b7f(char *a0) // return-dupe
{
  if (!dat_33758)
    return;
  sub_6b1e(a0);
}


// Function: sub_6ba8 @ 0x6ba8
void sub_6ba8(char *a0) // return-dupe
{
  if (!dat_33758)
    return;
  sub_6b52(a0);
}


// Function: sub_6bd1 @ 0x6bd1
void sub_6bd1(void)
{
  dat_33030 = dat_33028;
}


// Function: sub_6bea @ 0x6bea
void sub_6bea(void)
{
  dat_33758 = 0xffffffff;
}


// Function: sub_6bff @ 0x6bff
void sub_6bff(void)
{
  dat_33068 = 0x290e3;
  dat_33070 = 0x290e9;
}


// Function: sub_6c26 @ 0x6c26
void sub_6c26(int a0) // return-dupe
{
  if (0 <= putchar_unlocked(a0))
    return;
  dat_33798 = *__errno_location();
}


// Function: sub_6c53 @ 0x6c53
void sub_6c53(char *a0) // return-dupe
{
  if (0 <= fputs_unlocked(a0,stdout))
    return;
  dat_33798 = *__errno_location();
}


// Function: sub_6c8d @ 0x6c8d
void sub_6c8d(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,unsigned long a6,unsigned long a7,char *a8,unsigned long a9,unsigned long a10,unsigned long a11,unsigned long a12,unsigned long a13) // return-dupe
{
  char v1; // al
  unsigned long v10; // stack - 0x98
  unsigned long v11; // stack - 0x90
  unsigned long v12; // stack - 0x88
  unsigned long v13; // stack - 0x78
  unsigned long v14; // stack - 0x68
  unsigned long v15; // stack - 0x58
  unsigned long v16; // stack - 0x48
  unsigned long v17; // stack - 0x38
  unsigned long v18; // stack - 0x28
  unsigned long v19; // stack - 0x18
  char v2 [8];
  unsigned int v3; // stack - 0xd8
  unsigned int v4; // stack - 0xd4
  char *v5; // stack - 0xd0
  char *v6; // stack - 0xc8
  unsigned long v7; // stack - 0xb0
  unsigned long v8; // stack - 0xa8
  unsigned long v9; // stack - 0xa0
  
  if (v1) {
    v12 = a0;
    v13 = a1;
    v14 = a2;
    v15 = a3;
    v16 = a4;
    v17 = a5;
    v18 = a6;
    v19 = a7;
  }
  v3 = 8;
  v4 = 0x30;
  v5 = &Stack0000000000000008;
  v6 = v2;
  v7 = a9;
  v8 = a10;
  v9 = a11;
  v10 = a12;
  v11 = a13;
  if (0 <= vfprintf(stdout,a8,&v3))
    return;
  dat_33798 = *__errno_location();
}


// Function: sub_6d74 @ 0x6d74
void sub_6d74(void *a0,unsigned long a1,unsigned long a2) // return-dupe
{
  if (fwrite_unlocked(a0,a1,a2,stdout) == a2)
    return;
  dat_33798 = *__errno_location();
}


// Function: sub_6dc0 @ 0x6dc0
void sub_6dc0(void) // return-dupe
{
  if (!fflush_unlocked(stdout))
    return;
  dat_33798 = *__errno_location();
}


// Function: sub_6deb @ 0x6deb
unsigned int sub_6deb(unsigned int a0) // return-dupe
{
  if (((((a0 & 0xf000) != 0x2000) && ((a0 & 0xf000) != 0x6000)) && ((a0 & 0xf000) != 0xc000)) && ((a0 & 0xf000) != 0x1000))
    return 0;
  return 1;
}


// Function: sub_6e43 @ 0x6e43
unsigned int sub_6e43(unsigned char a0) // return-dupe
{
  if ((dat_337cc != 2) && (!((a0 ^ 1) & dat_337cc == 0)))
    return 0;
  return 1;
}


// Function: sub_6e8a @ 0x6e8a
unsigned int sub_6e8a(long a0)
{
  unsigned int v1; // eax
  
  v1 = *(unsigned int *)(a0 + 0x18) & 0xf000;
  return CONCAT31((undefined3)(v1 >> 8),v1 == 0x8000);
}


// Function: sub_6eac @ 0x6eac
char * sub_6eac(void)
{
  if (!dat_337c0)
    dat_337c0 = gettext("(standard input)");
  return dat_337c0;
}


// Function: sub_6edf @ 0x6edf
void sub_6edf(int a0) // return-dupe
{
  if (dat_33755 == '\x01') {
    dat_337c9 = 1;
    return;
  }
  error(0,a0,"%s",(char *)sub_6eac());
  dat_337c9 = 1;
}


// Function: sub_6f2c @ 0x6f2c
void sub_6f2c(void) // return-dupe
{
  if (dat_33798)
    return;
  sub_1143e();
}


// Function: sub_6f46 @ 0x6f46
void sub_6f46(void)
{
  unsigned char v1; // stack - 0x11
  int v2; // stack - 0x10
  unsigned int v3; // stack - 0xc
  
  v1 = 0;
  v2 = 1;
  for (v3 = 1; (int)v3 <= 0xff; v3 = v3 + 1) {
    if (!(v1 & v3) && *(char *)((long)(int)v3 + 0x331a3) != '\x01') {
      for (; v2 * 2 <= (int)v3; v2 = v2 << 1) {
      }
      v1 |= (unsigned char)v2;
    }
  }
  dat_337e8 = (unsigned long)v1 * (SUB168(ZEXT816(0xffffffffffffffff) * ZEXT816(0x8080808080808081),8) >> 7);
}


// Function: sub_6fdd @ 0x6fdd
unsigned long * sub_6fdd(unsigned long *a0)
{
  unsigned long v1;
  unsigned long *v2; // stack - 0x20
  unsigned long *v3; // stack - 0x18
  
  v2 = a0;
  while( true ) {
    if (!((unsigned long)v2 & 7)) {
      for (v3 = v2; !(dat_337e8 & *v3); v3 = &v3[1]) {
      }
      v2 = v3;
      while (v1 = *v2, !(dat_337e8 & (unsigned char)sub_f64d((int)(char)v1))) {
        v2 = (unsigned long *)((long)v2 + 1);
      }
      return v2;
    }
    v1 = *v2;
    if (dat_337e8 & (unsigned char)sub_f64d((int)(char)v1)) break;
    v2 = (unsigned long *)((long)v2 + 1);
  }
  return v2;
}


// Function: sub_7098 @ 0x7098
unsigned long sub_7098(unsigned long *a0,long a1) // return-dupe
{
  long v1; // rax
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x18
  unsigned long *v4; // stack - 0x28
  
  if (!dat_337e8)
    return 0;
  v3 = 0;
  *(char *)((long)a0 + a1) = 0xff;
  v4 = a0;
  while( true ) {
    v2 = sub_6fdd(v4);
    if ((unsigned long)((long)a0 + a1) <= v2)
      return 0;
    v1 = sub_f686(v2,(long)a0 + (a1 - v2),&v3);
    if (v1 <= -1) break;
    v4 = (unsigned long *)(v2 + v1);
  }
  return 1;
}


// Function: sub_716c @ 0x716c
unsigned long sub_716c(char *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  a0[a1] = '\0';
  v1 = strlen(a0);
  return CONCAT71((undefined7)(v1 >> 8),v1 != a1);
}


// Function: sub_71a6 @ 0x71a6
unsigned long sub_71a6(long a0,int a1,long a2)
{
  long v1; // rax
  long v2; // stack - 0x18
  
  if (((dat_337d0 != '\x01') && (sub_6e8a(a2))) && (a0 < *(long *)(a2 + 0x30))) {
    v2 = a0;
    if (!a1) {
      v2 = lseek(0,0,1);
      if (v2 <= -1)
        return 0;
    }
    v1 = lseek(a1,v2,4);
    if (0 <= v1) {
      if ((long)lseek(a1,v2,0) <= -1)
        sub_6edf(*__errno_location());
      if (v1 < *(long *)(a2 + 0x30))
        return 1;
    }
  }
  return 0;
}


// Function: sub_728f @ 0x728f
void sub_728f(char *a0,long *a1)
{
  if (((unsigned int)sub_27281(a0,NULL,10,a1,(char *)0x290d2) <= 1) && (0 <= *a1))
    return;
  error(2,0,"%s: %s",a0,gettext("invalid context length argument"));
}


// Function: sub_7316 @ 0x7316
unsigned long sub_7316(bool a0) // return-dupe
{
  if (a0)
    return 0x10000000;
  return 0x50000000;
}


// Function: sub_7337 @ 0x7337
unsigned int sub_7337(char *a0,unsigned char a1,char a2) // return-dupe
{
  long *v1;
  long v2; // stack - 0x10
  
  if (a2 != '\x01') // branch-flip
    v2 = 0x337a0;
  else {
    if (dat_33010 == 4)
      return 1;
    if ((a1) && (dat_337c8))
      return 0;
    v2 = 0x337b0;
  }
  if ((*(long *)(v2 + (unsigned long)a1 * 8)) && (v1 = *(long **)(v2 + (unsigned long)a1 * 8), sub_1cbb6(v1,a0)))
    return 1;
  return 0;
}


// Function: sub_73f5 @ 0x73f5
long sub_73f5(long a0,long a1)
{
  long v1; // rax
  
  if (!SCARRY8(a1,a0))
    return a1 + a0;
  error(2,0,gettext("input is too large to count"));
  return v1;
}


// Function: sub_747d @ 0x747d
unsigned long sub_747d(char *a0,long a1)
{
  char *v1; // stack - 0x10
  
  v1 = a0;
  while( true ) {
    if (&a0[a1] <= v1)
      return 1;
    if (*v1) break;
    v1 = &v1[1];
  }
  return 0;
}


// Function: sub_74c6 @ 0x74c6
unsigned long sub_74c6(int a0) // return-dupe, ternary
{
  dat_33808 = ((dat_337f0 + 1U) % dat_33818) ? (dat_33818 - (dat_337f0 + 1U) % dat_33818) + 1 + dat_337f0 : dat_337f0 + 1; // branch-flip
  *(char *)(dat_33808 + -1) = dat_33183;
  dat_33800 = a0;
  dat_33810 = dat_33808;
  if (a0) // branch-flip
    dat_33820 = 0;
  else {
    dat_33820 = lseek(0,0,1);
  }
  dat_337d0 = -(char)((long)dat_33820 >> 0x3f);
  if (!dat_337d0) {
    dat_337d1 = dat_337d0;
    return 1;
  }
  dat_337d1 = dat_337d0;
  if (*__errno_location() == 0x1d) {
    dat_33820 = 0;
    return 1;
  }
  sub_6edf(*__errno_location());
  return 0;
}


// Function: sub_7608 @ 0x7608
char sub_7608(long a0,long a1)
{
  long v1;
  long v10; // stack - 0x50
  long v11; // stack - 0x48
  long v2; // rax
  long v3; // rax
  unsigned long v4; // rax
  long v5; // rax
  char v6; // stack - 0x71
  long v7; // stack - 0x68
  void *v8; // stack - 0x60
  long v9; // stack - 0x58
  
  v1 = dat_33818 + 8;
  if (v1 <= (long)dat_337f0 + (dat_337f8 - (long)dat_33810)) // branch-flip
    v7 = (long)dat_33810;
  else {
    v5 = v1 + ((a0 + dat_33818) - dat_337f8);
    if (1 <= v5) { // branch-flip
      v9 = -1;
      if (((sub_6e8a(a1)) && (v2 = *(long *)(a1 + 0x30) - dat_33820, 0 <= v2)) && (v3 = v2 + a0 + v1, !SCARRY8(v2,a0 + v1))) {
        v9 = v3;
        if (v3 <= dat_337f8 + v5)
          v9 = dat_337f8 + v5;
      }
      v8 = (void *)sub_26dc6(NULL,(long *)0x337f8,v5,v9,1);
    }
    else {
      v8 = dat_337f0;
    }
    v1 = a0;
    if ((unsigned long)((long)v8 + a0 + 1) % dat_33818)
      v1 = a0 + (dat_33818 - (unsigned long)((long)v8 + a0 + 1) % dat_33818);
    v7 = (long)v8 + v1 + 1;
    v4 = a0 + 1;
    memmove((void *)(v7 - v4),(void *)((long)dat_33810 - v4),v4);
    if (1 <= v5) {
      free(dat_337f0);
      dat_337f0 = v8;
    }
  }
  dat_33808 = v7 - a0;
  sub_6b00();
  v1 = (long)dat_337f0 + ((dat_337f8 + -8) - v7);
  v5 = v1 % (long)dat_33818;
  v6 = 1;
  while( true ) {
    v10 = sub_24c3e(dat_33800,v7,v1 - v5);
    if (v10 == -1) {
      v10 = 0;
      v6 = 0;
    }
    dat_33820 = v10 + dat_33820;
    if ((dat_33830 != '\x01' || !v10) || (sub_747d(v7,v10) != '\x01')) break;
    dat_33838 = sub_73f5(dat_33838,v10);
    if (dat_337d1 != '\x01') {
      v11 = lseek(dat_33800,dat_33820,3);
      if (((v11 <= -1) && (*__errno_location() == 6)) && ((sub_6e8a(a1) && (dat_33820 < *(long *)(a1 + 0x30)))))
        v11 = lseek(dat_33800,0,2);
      if (0 <= v11) { // branch-flip
        dat_33838 = sub_73f5(dat_33838,v11 - dat_33820);
        dat_33820 = v11;
      }
      else {
        dat_337d1 = '\x01';
      }
    }
  }
  dat_33810 = (void *)(v7 + v10);
  memset(dat_33810,0,8);
  sub_6b0b((long)dat_33810 + 8,(dat_337f8 - ((long)dat_33810 - (long)dat_337f0)) + -8);
  return v6;
}


// Function: sub_7a8d @ 0x7a8d
void sub_7a8d(void *a0)
{
  long v1; // stack - 0x18
  void *v2; // stack - 0x10
  
  v1 = 0;
  v2 = dat_33890;
  while ((v2 < a0 && (v2 = memchr(v2,(int)dat_33183,(long)a0 - (long)v2), v2))) {
    v1 += 1;
    v2 = (void *)((long)v2 + 1);
  }
  dat_33838 = sub_73f5(dat_33838,v1);
  dat_33890 = a0;
}


// Function: sub_7b22 @ 0x7b22
void sub_7b22(void)
{
  sub_6b7f(dat_33038);
  sub_6c53((char *)sub_6eac());
  sub_6ba8(dat_33038);
}


// Function: sub_7b58 @ 0x7b58
void sub_7b58(char a0)
{
  sub_6b7f(dat_33050);
  sub_6c26((int)a0);
  sub_6ba8(dat_33050);
}


// Function: sub_7b95 @ 0x7b95
void sub_7b95(unsigned long a0,char *a1)
{
  sub_6b7f(a1);
  sub_6c8d("%*ld",dat_33760,a0);
  sub_6ba8(a1);
}


// Function: sub_7be4 @ 0x7be4
unsigned long sub_7be4(unsigned long *a0,long a1,unsigned long a2,char a3)
{
  char v1;
  char v2; // al
  unsigned long v3; // rax
  
  if (dat_33840 != 1) {
    v1 = *(char *)((long)a0 + a1);
    v2 = sub_7098(a0,a1);
    *(char *)(a1 + (long)a0) = v1;
    if (v2) {
      dat_337ca = 1;
      return 0;
    }
  }
  if (dat_33848) {
    sub_7b22();
    if (dat_3384c) // branch-flip
      sub_7b58((int)a3);
    else {
      sub_6c26(0);
    }
  }
  if (dat_33852) {
    if (dat_33890 < a2) {
      sub_7a8d(a0);
      dat_33838 = sub_73f5(dat_33838,1);
      dat_33890 = a2;
    }
    sub_7b95(dat_33838,dat_33040);
    sub_7b58((int)a3);
  }
  if (dat_33853) {
    v3 = sub_73f5(dat_33888,(long)a0 - dat_33808);
    sub_7b95(v3,dat_33048);
    sub_7b58((int)a3);
  }
  if (((dat_3375d) && (dat_33853 || (dat_33852 || dat_33848))) && (a1))
    sub_6c26(9);
  return 1;
}


// Function: sub_7d91 @ 0x7d91
unsigned long * sub_7d91(unsigned long *a0,unsigned long *a1,char *a2,char *a3) // ternary
{
  char v1; // al
  char v2; // al
  long v3; // stack - 0x38
  unsigned long *v4; // stack - 0x30
  unsigned long *v5; // stack - 0x28
  long v6; // stack - 0x20
  unsigned long *v7; // stack - 0x18
  
  v5 = NULL;
  v4 = a0;
  do {
    if (((a1 <= v4) || (v6 = (*dat_337d8)(dat_337e0,a0,(long)a1 - (long)a0,&v3,v4), v6 <= -1)) || (v7 = (unsigned long *)((long)a0 + v6), v7 == a1)) {
      if (dat_3375c) // branch-flip
        v4 = a1;
      else if (v5)
        v4 = v5;
      return v4;
    }
    if (v3) { // branch-flip
      if (dat_3375c) { // branch-flip
        v1 = (dat_33851) ? 0x2d : 0x3a; // branch-flip
        v2 = sub_7be4(v7,v3,a1,(unsigned char)v1);
        if (v2 != '\x01')
          return NULL;
      }
      else {
        sub_6b1e(a2);
        if (v5) {
          v4 = v5;
          v5 = NULL;
        }
        sub_6d74(v4,1,(long)v7 - (long)v4);
      }
      sub_6b7f(a3);
      sub_6d74(v7,1,v3);
      sub_6ba8(a3);
      if (dat_3375c)
        sub_6c26((int)dat_33183);
    }
    else {
      v3 = 1;
      if (!v5)
        v5 = v4;
    }
    v4 = (unsigned long *)((long)v7 + v3);
  } while( true );
}


// Function: sub_7f87 @ 0x7f87
unsigned long sub_7f87(unsigned long a0,unsigned long a1,char *a2) // warn: iteboolean: re-rolled 2 0/1 select diamond(s) to a boolean assignment in sub_7f87
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x20
  
  v1 = !((a1 <= a0) || (*(char *)(a1 - 1) != dat_33183));
  v2 = (unsigned long)v1;
  v1 = !((a1 - v2 <= a0) || (*(char *)(a1 + ~v2) != '\r'));
  v2 = (a1 - (v2 + (long)v1)) - a0;
  v3 = a0;
  if (1 <= (long)v2) {
    sub_6b1e(a2);
    sub_6d74(a0,1,v2);
    v3 = a0 + v2;
    sub_6b52(a2);
  }
  return v3;
}


// Function: sub_806d @ 0x806d
void sub_806d(unsigned long a0,unsigned long a1,char a2) // return-dupe x2
{
  unsigned char v1;
  bool v2; // zf
  unsigned long v3; // stack - 0x30
  char *v4; // stack - 0x18
  char *v5; // stack - 0x10
  
  if ((dat_3375c != '\x01') && (sub_7be4(a0,(a1 - a0) + -1,a1,(int)a2) != '\x01'))
    return;
  v2 = (bool)dat_33851 != (a2 == ':');
  if (dat_33758) { // branch-flip
    if ((dat_33851) && (dat_33758 <= -1))
      v1 = 1;
    else {
      v1 = 0;
    }
    if ((bool)(v1 ^ a2 == ':'))
      v4 = dat_33058;
    else {
      v4 = dat_33060;
    }
    if (a2 != ':') // branch-flip
      v5 = dat_33030;
    else {
      v5 = dat_33028;
    }
  }
  else {
    v5 = NULL;
    v4 = NULL;
  }
  v3 = a0;
  if (((dat_3375c) && (v2)) || ((dat_33758 && ((*v4 || (*v5)))))) {
    if ((v2) && ((dat_3375c || (*v5)))) {
      v3 = sub_7d91(a0,a1,v4,v5);
      if (!v3)
        return;
    }
    if ((dat_3375c != '\x01') && (*v4))
      v3 = sub_7f87(v3,a1,v4);
  }
  if ((dat_3375c != '\x01') && (v3 < a1))
    sub_6d74(v3,1,a1 - v3);
  if (dat_33878)
    sub_6dc0();
  if (dat_33798) {
    error(2,dat_33798,gettext("write error"));
    return;
  }
  dat_33898 = a1;
}


// Function: sub_828e @ 0x828e
void sub_828e(void *a0)
{
  void *v1; // rax
  
  if (!dat_33898)
    dat_33898 = dat_33808;
  while ((1 <= dat_338a8 && (dat_33898 < a0))) {
    v1 = rawmemchr(dat_33898,(int)dat_33183);
    sub_806d(dat_33898,(long)v1 + 1,0x2d);
    dat_338a8 -= 1;
  }
}


// Function: sub_8326 @ 0x8326
void sub_8326(void *a0,void *a1)
{
  void *v1;
  char v2;
  void *v3;
  char *v4; // rax
  void *v5; // stack - 0x38
  long v6; // stack - 0x30
  long v7; // stack - 0x28
  
  v2 = dat_33183;
  if ((dat_33850 != '\x01') && (1 <= dat_338a8))
    sub_828e(a0);
  v5 = a0;
  if (dat_33850 != '\x01') {
    if (dat_33898) // branch-flip
      v3 = dat_33898;
    else {
      v3 = dat_33808;
    }
    for (v6 = 0; v6 < dat_33858; v6 = v6 + 1) {
      if (v3 < v5) {
        do {
          v1 = (void *)((long)v5 - 1);
          v4 = (char *)((long)v5 - 2);
          v5 = v1;
        } while (v2 != *v4);
      }
    }
    if ((((0 <= dat_33858) || (0 <= dat_33860)) && (dat_339c0)) && ((v5 != dat_33898 && (dat_33020)))) {
      sub_6b7f(dat_33050);
      sub_6c53(dat_33020);
      sub_6ba8(dat_33050);
      sub_6c26(10);
    }
    while (v5 < a0) {
      v3 = (void *)((long)rawmemchr(v5,(int)v2) + 1);
      sub_806d(v5,v3,0x2d);
      v5 = v3;
    }
  }
  if (dat_33851) { // branch-flip
    v7 = 0;
    while ((v5 < a1 && (v7 < dat_338a0))) {
      v3 = (void *)((long)rawmemchr(v5,(int)v2) + 1);
      if (dat_33850 != '\x01')
        sub_806d(v5,v3,0x3a);
      v7 += 1;
      v5 = v3;
    }
  }
  else {
    if (dat_33850 != '\x01')
      sub_806d(a0,a1,0x3a);
    v7 = 1;
    v5 = a1;
  }
  dat_33828 = dat_33820 - (dat_33810 - (long)v5);
  if (dat_33850) // branch-flip
    dat_338a8 = 0;
  else {
    dat_338a8 = dat_33860;
    if (dat_33860 <= -1)
      dat_338a8 = 0;
  }
  dat_339c0 = 1;
  dat_338a0 -= v7;
}


// Function: sub_85c1 @ 0x85c1
void sub_85c1(char *a0,char *a1,char a2) // return-dupe
{
  char *v1;
  char *v2; // stack - 0x10
  
  v2 = a0;
  if (!a2)
    return;
  while( true ) {
    *a1 = '\0';
    v2 = &v2[strlen(v2)];
    *a1 = a2;
    if (v2 == a1) break;
    do {
      v1 = &v2[1];
      *v2 = a2;
      v2 = v1;
    } while (!*v1);
  }
}


// Function: sub_862e @ 0x862e
long sub_862e(unsigned long a0,unsigned long a1)
{
  int v1;
  long v2; // stack - 0x50
  unsigned long v3; // stack - 0x48
  long v4; // stack - 0x40
  long v5; // stack - 0x38
  unsigned long v6; // stack - 0x30
  unsigned long v7; // stack - 0x28
  unsigned long v8; // stack - 0x20
  unsigned long v9; // stack - 0x18
  
  v5 = dat_338a0;
  v3 = a0;
  do {
    if (a1 <= v3)
      return v5 - dat_338a0; // return-dupe
    v4 = (*dat_337d8)(dat_337e0,v3,a1 - v3,&v2,0);
    if (v4 <= -1) {
      if (dat_33851 != '\x01') {
        return v5 - dat_338a0;
      }
      v4 = a1 - v3;
      v2 = 0;
    }
    v6 = v3 + v4;
    v7 = v6 + v2;
    if ((dat_33851 != '\x01') && (v6 == a1)) {
      return v5 - dat_338a0;
    }
    if ((dat_33851 != '\x01') || (v3 < v6)) {
      if (dat_33851) { // branch-flip
        v9 = v6;
        v8 = v3;
      }
      else {
        v9 = v7;
        v8 = v6;
      }
      sub_8326(v8,v9);
      if ((!dat_338a0) || (dat_338b0)) {
        if (dat_338b1) {
          if (dat_337c9) // branch-flip
            v1 = dat_33080;
          else {
            v1 = 0;
          }
          exit(v1); // no-return
        }
        return v5 - dat_338a0;
      }
    }
    v3 = v7;
  } while( true );
}


// Function: sub_87e9 @ 0x87e9
long sub_87e9(unsigned int a0,long a1,char *a2) // early-return x3, warn: iteboolean: re-rolled 2 0/1 select diamond(s) to a boolean assignment in sub_87e9
{
  bool v1;
  char v10; // stack - 0x5e
  long v11; // stack - 0x58
  long v12; // stack - 0x50
  long v13; // stack - 0x48
  char *v14; // stack - 0x40
  char *v15; // stack - 0x38
  char *v16; // stack - 0x30
  long v17; // stack - 0x28
  long v18; // stack - 0x20
  char v2;
  char v3;
  char v4;
  char v5;
  int v6; // eax
  long v7;
  char *v8;
  char *v9; // rax
  
  v4 = dat_338b0;
  v3 = dat_33850;
  v2 = dat_33183;
  v10 = '\0';
  v17 = -1;
  if (sub_74c6(a0,a1) != '\x01')
    return 0;
  dat_33888 = 0;
  dat_33898 = NULL;
  dat_33838 = 0;
  dat_338a0 = dat_33870;
  dat_33828 = 0;
  dat_338a8 = 0;
  dat_33830 = ((dat_33831) && (!v2));
  dat_337ca = '\0';
  v11 = 0;
  v13 = 0;
  v14 = NULL;
  if (sub_7608(0,a1) != '\x01') {
    sub_6edf(*__errno_location());
    return 0;
  }
  dat_33760 = 0;
  if (dat_3375d) {
    if (sub_6e8a(a1)) // branch-flip
      v18 = *(long *)(a1 + 0x30);
    else {
      v18 = 0x7fffffffffffffff;
    }
    v6 = ((dat_33852) && (v18 != 0x7fffffffffffffff));
    v18 += v6;
    do {
      dat_33760 += 1;
      v18 /= 10;
    } while (v18);
  }
  v1 = 1;
  while( true ) {
    if ((((v17 <= -1) && (v2)) && (dat_33840 != 1)) && ((v5 = sub_716c(dat_33808,(long)dat_33810 - (long)dat_33808), v5 || ((v1 && (v7 = (long)dat_33810 - (long)dat_33808, sub_71a6(v7,a0,a1))))))) {
      if (dat_33840 == 2)
        return 0;
      if (dat_33868 != '\x01') {
        dat_33850 = '\x01';
        dat_338b0 = '\x01';
      }
      v17 = v11;
      v10 = v2;
      dat_33830 = dat_33831;
    }
    dat_33890 = dat_33808;
    if (dat_33898)
      dat_33898 = dat_33808;
    v8 = &dat_33808[(long)v14];
    if (v8 == dat_33810) {
      *a2 = 1;
      if (v13) {
        v8 = &dat_33810[1];
        *dat_33810 = v2;
        dat_33810 = v8;
        if (dat_338a0) {
          v9 = &dat_33808[(long)v14 - v13];
          v11 += sub_862e(v9,v8);
        }
        if (dat_338a8)
          sub_828e(dat_33810);
      }
      goto label_8d91;
    }
    sub_85c1(v8,dat_33810,(int)v10);
    v5 = v8[-1];
    v8[-1] = v2;
    v9 = &dat_33810[1 - (long)v8];
    v16 = (char *)((long)memrchr(&v8[-1],(int)v2,(unsigned long)v9) + 1);
    v8[-1] = v5;
    if (v16 == v8)
      v16 = &v8[-v13];
    v7 = -v13;
    v13 = (long)dat_33810 - (long)v16;
    if (&v8[v7] < v16) {
      if (dat_338a0)
        v11 += sub_862e(&v8[v7],v16);
      if (dat_338a8)
        sub_828e(v16);
      if ((!dat_338a0) && (!dat_338a8)) goto label_8d91;
      if (dat_338b0) {
        v7 = v17;
        if (v17 <= -1)
          v7 = 0;
        if (v7 < v11) goto label_8d91;
      }
    }
    v12 = 0;
    v15 = v16;
    while (((v12 < dat_33858 && (dat_33808 < v15)) && (v15 != dat_33898))) {
      v12 += 1;
      do {
        v8 = &v15[-1];
        v9 = &v15[-2];
        v15 = v8;
      } while (v2 != *v9);
    }
    if (v15 != dat_33898)
      dat_33898 = NULL;
    v14 = &v16[v13 - (long)v15];
    if (dat_33853)
      dat_33888 = sub_73f5(dat_33888,&dat_33810[-(long)v14 - (long)dat_33808]);
    if (dat_33852)
      sub_7a8d(v15);
    if (sub_7608(v14,a1) != '\x01') break;
    v1 = 0;
  }
  sub_6edf(*__errno_location());
label_8d91:
  dat_33850 = v3;
  dat_338b0 = v4;
  if (((!dat_33840) && (v3 != '\x01')) && ((dat_337ca || ((0 <= v17 && (v17 < v11)))))) {
    v8 = (char *)sub_6eac();
    error(0,0,gettext("%s: binary file matches"),v8);
  }
  return v11;
}


// Function: sub_8e14 @ 0x8e14
unsigned long sub_8e14(long a0,long a1,unsigned char a2) // return-dupe, ternary, warn: iteboolean: re-rolled 2 0/1 select diamond(s) to a boolean assignment in sub_8e14
{
  int v1;
  stat *v10; // stack - 0xc0
  char *v2;
  long v3;
  bool v4; // al
  unsigned int v5;
  int v6; // eax
  unsigned long v7; // rax
  stat v8;
  bool v9; // zf
  
  v9 = (*(long *)(a1 + 0x58) == 0 & a2) != 0;
  if (*(short *)(a1 + 0x68) == 6)
    return 1;
  if (!v9) {
    v5 = !(((*(short *)(a1 + 0x68) != 1) && (*(short *)(a1 + 0x68) != 2)) && (*(short *)(a1 + 0x68) != 4));
    if (sub_7337((char *)(a1 + 0x100),0,v5)) {
      sub_1e92f(a0,a1,4);
      return 1;
    }
  }
  dat_337c0 = *(long *)(a1 + 0x38);
  if ((dat_337c8) && (*(char *)(dat_337c0 + 1)))
    dat_337c0 += 2;
  v3 = dat_337c0;
  v4 = ((*(unsigned int *)(a0 + 0x48) & 2) || ((*(unsigned int *)(a0 + 0x48) & 1 && (v9))));
  switch(*(unsigned short *)(a1 + 0x68)) {
    default:
      abort(); // no-return
    case 1:
      if (dat_33010 == 3)
        return 1;
      sub_1e92f(a0,a1,4);
      break;
    case 2:
      if (dat_33755 == '\x01')
        return 1;
      error(0,0,gettext("%s: warning: recursive directory loop"),v3);
      return 1;
    case 3:
    case 0xb:
      if (sub_6e43(v9)) {
        v10 = (stat *)(a1 + 0x70);
        if (!*(int *)(a1 + 0x88)) {
          v6 = (v4) ? 0 : 0x100;
          v2 = *(char **)(a1 + 0x30);
          v1 = *(int *)(a0 + 0x2c);
          if (fstatat(v1,v2,&v8,v6)) {
            sub_6edf(*__errno_location());
            return 1;
          }
          v10 = &v8;
        }
        v5 = *(unsigned int *)&v10->field_0x18;
        if (sub_6deb(v5))
          return 1;
      }
      break;
    case 4:
    case 7:
    case 10:
      sub_6edf(*(unsigned int *)(a1 + 0x40));
      return 1;
    case 8:
    case 0xd:
      break;
    case 0xc:
    case 0xe:
      return 1;
    
  }
  v7 = sub_91bc(*(unsigned int *)(a0 + 0x2c),*(unsigned long *)(a1 + 0x30),v4,v9);
  return v7;
}


// Function: sub_9197 @ 0x9197
unsigned long sub_9197(int a0) // return-dupe
{
  if ((a0 != 0x28) && (a0 != 0x1f))
    return 0;
  return 1;
}


// Function: sub_91bc @ 0x91bc
unsigned long sub_91bc(unsigned int a0,char *a1,bool a2,char a3) // ternary x2
{
  unsigned int v1; // eax
  int v2;
  unsigned int v3; // ebx
  
  v3 = (a2) ? 0x100 : 0x20100; // branch-flip
  v1 = (sub_6e43((unsigned char)a3)) ? 0x800 : 0; // branch-flip
  v2 = sub_22af5(a0,a1,v1 | v3);
  if (v2 <= -1) {
    if (!a2) {
      v2 = *__errno_location();
      if (sub_9197(v2) == '\x01')
        return 1;
    }
    sub_6edf(*__errno_location());
    return 1;
  }
  return sub_9456(v2,(unsigned char)a3);
}


// Function: sub_9272 @ 0x9272
unsigned long sub_9272(int a0,long a1) // early-return x2
{
  unsigned int v1;
  unsigned long v2; // rax
  long v3; // rax
  unsigned long v4; // stack - 0x10
  
  if (((*(unsigned int *)(a1 + 0x18) & 0xf000) == 0x1000) && (dat_338b2)) {
    v2 = splice(a0,NULL,1,NULL,0x18000,1);
    v4 = v2;
    if ((long)v2 <= -1) {
      v1 = *__errno_location();
      v2 = (unsigned long)v1;
      if (v1 == 0x16) goto label_9331;
    }
    while (0 < (long)v4) {
      v2 = splice(a0,NULL,1,NULL,0x18000,1);
      v4 = v2;
    }
    return CONCAT71((undefined7)(v2 >> 8),v4 == 0);
  }
label_9331:
  do {
    v3 = sub_24c3e(a0,dat_337f0,dat_337f8);
    if (!v3)
      return 1;
  } while (v3 != -1);
  return 0;
}


// Function: sub_9361 @ 0x9361
void sub_9361(int a0,long a1,char a2) // return-dupe
{
  bool v1;
  
  if (a0)
    return;
  if (dat_338a0) { // branch-flip
    if ((a2 != '\x01') && (((dat_337d0 || (((long)lseek(0,0,2) <= -1 && (*__errno_location() != 0x16)))) && (sub_9272(0,a1) != '\x01'))))
      v1 = 1;
    else {
      v1 = 0;
    }
  }
  else if (((dat_33820 != dat_33828) && (dat_337d0 != '\x01')) && ((long)lseek(0,dat_33828,0) <= -1))
    v1 = 1;
  else {
    v1 = 0;
  }
  if (v1) {
    sub_6edf(*__errno_location());
    return;
  }
}


// Function: sub_9456 @ 0x9456
unsigned char sub_9456(int a0,unsigned char a1) // ternary
{
  char v1; // al
  long v10; // stack - 0xe0
  long v11; // stack - 0xd8
  long v12; // stack - 0xd0
  unsigned long v13; // stack - 0xc0
  unsigned char v2; // al
  int v3; // eax
  char *v4; // rax
  stat v5; // stack - 0xb8
  unsigned long v6; // stack - 0xc8
  char v7; // stack - 0xe6
  unsigned char v8; // stack - 0xe5
  unsigned int v9; // stack - 0xe4
  
  v8 = 1;
  v7 = 0;
  if (fstat(a0,&v5)) // branch-flip
    sub_6edf(*__errno_location());
  else if ((((!a0) || (!sub_6e43(a1))) || (v1 = sub_6deb(v5._24_4_), !v1)) && (((!a0 || (!a1)) || (v1 = sub_7337(dat_337c0,1,(v5._24_4_ & 0xf000) == 0x4000), !v1)))) {
    if ((int)dat_33848 <= -1)
      dat_33848 = (unsigned int)((v5._24_4_ & 0xf000) == 0x4000);
    if (((a0) && (dat_33010 == 3)) && ((v5._24_4_ & 0xf000) == 0x4000)) {
      v9 = dat_33014 & ~(unsigned int)(a1 ^ 1);
      if (close(a0))
        sub_6edf(*__errno_location());
      v6 = dat_337c0;
      v13 = 0;
      v10 = sub_1d975(&v6,v9,0);
      if (v10) {
        while( true ) {
          v11 = sub_1e2b9(v10);
          if (!v11) break;
          v2 = sub_8e14(v10,v11,a1);
          v8 = (v2 & v8) != 0;
        }
        if (*__errno_location())
          sub_6edf(*__errno_location());
        v3 = sub_1ddff(v10);
        if (!v3)
          return v8;
        sub_6edf(*__errno_location());
        return v8;
      }
      sub_27122(); // no-return
    }
    if ((!a0) || (((dat_33010 != 4 || ((v5._24_4_ & 0xf000) != 0x4000)) && (((dat_337cc != 2 && ((dat_337cc || (a1 == 1)))) || (v1 = sub_6deb(v5._24_4_), !v1)))))) {
      if (((((dat_33850 != '\x01') && (!dat_33844)) && (2 <= dat_33870)) && (((v5._24_4_ & 0xf000) == 0x8000 && (v5._8_8_ == dat_336c8)))) && (v5._0_8_ == dat_336c0)) {
        if (dat_33755 != '\x01') {
          v4 = (char *)sub_6eac();
          error(0,0,gettext("%s: input file is also the output"),v4);
        }
        dat_337c9 = 1;
      }
      else {
        v12 = sub_87e9(a0,&v5,&v7);
        if (dat_33868) {
          if (dat_33848) {
            sub_7b22();
            if (dat_3384c) // branch-flip
              sub_7b58(0x3a);
            else {
              sub_6c26(0);
            }
          }
          sub_6c8d("%ld\n",v12);
          if (dat_33878)
            sub_6dc0();
        }
        v8 = v12 == 0;
        if (dat_33844) { // branch-flip
          v3 = ((bool)v8) ? 2 : 1;
          if (v3 == dat_33844) {
            sub_7b22();
            sub_6c26(dat_3384c & 10);
            if (dat_33878)
              sub_6dc0();
          }
        }
        else {
          sub_9361(a0,&v5,(unsigned char)v7);
        }
      }
    }
  }
  if ((a0) && (close(a0)))
    sub_6edf(*__errno_location());
  return v8;
}


// Function: sub_992b @ 0x992b
void sub_992b(char *a0) // return-dupe
{
  if (!strcmp(a0,"-")) {
    dat_337c0 = (char *)dat_33880;
    if (dat_338b3)
      sub_2716d(0,0);
    sub_9456(0,1);
    return;
  }
  dat_337c0 = a0;
  sub_91bc(0xffffff9c,a0,1,1);
}


// Function: sub_99b6 @ 0x99b6
void sub_99b6(int a0)
{
  char *v1; // rax
  
  if (a0) { // branch-flip
    v1 = (char *)sub_1ff0c();
    fprintf(stderr,gettext("Usage: %s [OPTION]... PATTERNS [FILE]...\n"),v1);
    v1 = (char *)sub_1ff0c();
    fprintf(stderr,gettext("Try \'%s --help\' for more information.\n"),v1);
  }
  else {
    v1 = (char *)sub_1ff0c();
    printf(gettext("Usage: %s [OPTION]... PATTERNS [FILE]...\n"),v1);
    printf(gettext("Search for PATTERNS in each FILE.\n"));
    v1 = (char *)sub_1ff0c();
    printf(gettext("Example: %s -i \'hello world\' menu.h main.c\nPATTERNS can contain multiple patterns separated by newlines.\n\nPattern selection and interpretation:\n"),v1);
    printf(gettext("  -E, --extended-regexp     PATTERNS are extended regular expressions\n  -F, --fixed-strings       PATTERNS are strings\n  -G, --basic-regexp        PATTERNS are basic regular expressions\n  -P, --perl-regexp         PATTERNS are Perl regular expressions\n"));
    printf(gettext("  -e, --regexp=PATTERNS     use PATTERNS for matching\n  -f, --file=FILE           take PATTERNS from FILE\n  -i, --ignore-case         ignore case distinctions in patterns and data\n      --no-ignore-case      do not ignore case distinctions (default)\n  -w, --word-regexp         match only whole words\n  -x, --line-regexp         match only whole lines\n  -z, --null-data           a data line ends in 0 byte, not newline\n"));
    printf(gettext("\nMiscellaneous:\n  -s, --no-messages         suppress error messages\n  -v, --invert-match        select non-matching lines\n  -V, --version             display version information and exit\n      --help                display this help text and exit\n"));
    printf(gettext("\nOutput control:\n  -m, --max-count=NUM       stop after NUM selected lines\n  -b, --byte-offset         print the byte offset with output lines\n  -n, --line-number         print line number with output lines\n      --line-buffered       flush output on every line\n  -H, --with-filename       print file name with output lines\n  -h, --no-filename         suppress the file name prefix on output\n      --label=LABEL         use LABEL as the standard input file name prefix\n"));
    printf(gettext("  -o, --only-matching       show only nonempty parts of lines that match\n  -q, --quiet, --silent     suppress all normal output\n      --binary-files=TYPE   assume that binary files are TYPE;\n                            TYPE is \'binary\', \'text\', or \'without-match\'\n  -a, --text                equivalent to --binary-files=text\n"));
    printf(gettext("  -I                        equivalent to --binary-files=without-match\n  -d, --directories=ACTION  how to handle directories;\n                            ACTION is \'read\', \'recurse\', or \'skip\'\n  -D, --devices=ACTION      how to handle devices, FIFOs and sockets;\n                            ACTION is \'read\' or \'skip\'\n  -r, --recursive           like --directories=recurse\n  -R, --dereference-recursive  likewise, but follow all symlinks\n"));
    printf(gettext("      --include=GLOB        search only files that match GLOB (a file pattern)\n      --exclude=GLOB        skip files that match GLOB\n      --exclude-from=FILE   skip files that match any file pattern from FILE\n      --exclude-dir=GLOB    skip directories that match GLOB\n"));
    printf(gettext("  -L, --files-without-match  print only names of FILEs with no selected lines\n  -l, --files-with-matches  print only names of FILEs with selected lines\n  -c, --count               print only a count of selected lines per FILE\n  -T, --initial-tab         make tabs line up (if needed)\n  -Z, --null                print 0 byte after FILE name\n"));
    printf(gettext("\nContext control:\n  -B, --before-context=NUM  print NUM lines of leading context\n  -A, --after-context=NUM   print NUM lines of trailing context\n  -C, --context=NUM         print NUM lines of output context\n"));
    printf(gettext("  -NUM                      same as --context=NUM\n      --group-separator=SEP  print SEP on line between matches with context\n      --no-group-separator  do not print separator for matches with context\n      --color[=WHEN],\n      --colour[=WHEN]       use markers to highlight the matching strings;\n                            WHEN is \'always\', \'never\', or \'auto\'\n  -U, --binary              do not strip CR characters at EOL (MSDOS/Windows)\n\n"));
    printf(gettext("When FILE is \'-\', read standard input.  With no FILE, read \'.\' if\nrecursive, \'-\' otherwise.  With fewer than two FILEs, assume -h.\nExit status is 0 if any line is selected, 1 otherwise;\nif any error occurs and -q is not given, the exit status is 2.\n"));
    sub_269ee();
  }
  exit(a0); // no-return
}


// Function: sub_9be6 @ 0x9be6
unsigned long sub_9be6(char *a0,unsigned int a1)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  unsigned int v3; // stack - 0xc
  
  v3 = 0;
  while( true ) {
    if (7 <= v3) {
      error(2,0,gettext("invalid matcher %s"),a0);
      return v2;
    }
    if (!strcmp(a0,(char *)((long)(int)v3 * 0x20 + 0x327a0))) break;
    v3 += 1;
  }
  if ((0 <= (int)a1) && (a1 != v3)) {
    error(2,0,gettext("conflicting matchers specified"));
    return v1;
  }
  return (unsigned long)v3;
}


// Function: sub_9c9f @ 0x9c9f
int sub_9c9f(int a0,char **a1,long *a2)
{
  char *v1;
  bool v2;
  int v3;
  int v4; // eax
  char v5 [21]; // stack - 0x38
  char v6 [19];
  char *v7; // stack - 0x40
  
  v2 = 0;
  v7 = v5;
  do {
    v3 = optind;
    v4 = getopt_long(a0,a1,"0123456789A:B:C:D:EFGHIPTUVX:abcd:e:f:hiLlm:noqRrsuvwxyZz",(option *)0x320a0,NULL);
    if (sub_10eab(v4) != '\x01') {
label_9da2:
      if (v7 != v5) {
        *v7 = '\0';
        sub_728f(v5,a2);
      }
      return v4;
    }
    if ((v3 == dat_33018) && (v2))
      v7 = &v7[-(unsigned long)(v5[0] == '0')];
    else {
      v7 = v5;
    }
    if (v7 == v6) {
      strcpy(v7,"...");
      v7 = &v7[3];
      goto label_9da2;
    }
    v1 = &v7[1];
    *v7 = (char)v4;
    v2 = 1;
    dat_33018 = v3;
    v7 = v1;
  } while( true );
}


// Function: sub_9ddf @ 0x9ddf
void sub_9ddf(void)
{
  char v1;
  char *v2;
  char *v3; // stack - 0x30
  char *v4; // stack - 0x28
  char *v5; // stack - 0x20
  long *v6; // stack - 0x18
  
  v2 = getenv("GREP_COLORS");
  if ((!v2) || (!*v2))
    return;
  v3 = (char *)sub_270f1(v2);
  v4 = v3;
  v5 = NULL;
label_9e38:
  do {
    for (; (*v3 == ':' || (!*v3)); v3 = &v3[1]) {
      v1 = *v3;
      *v3 = '\0';
      v6 = (long *)0x31f80;
      while ((*v6 && (v2 = (char *)*v6, strcmp(v2,v4)))) {
        v6 = &v6[3];
      }
      if ((v6[1]) && (v5))
        *(char **)v6[1] = v5;
      if (v6[2])
        (*(void *)v6[2])();
      if (!v1)
        return;
      v5 = NULL;
      v4 = &v3[1];
label_9f6a:
    }
    if (*v3 != '=') {
      if (v5) {
        if ((*v3 != ';') && (v1 = *v3, !sub_10eab((int)v1)))
          return;
        goto label_9f6a;
      }
      v3 = &v3[1];
      goto label_9e38;
    }
    if (v3 == v4)
      return;
    if (v5)
      return;
    v5 = &v3[1];
    *v3 = '\0';
    v3 = v5;
  } while( true );
}


// Function: sub_9f7b @ 0x9f7b
unsigned long sub_9f7b(char *a0,long a1)
{
  long v1; // rax
  unsigned long v2; // stack - 0x18
  long v3; // stack - 0x28
  
  v2 = 0;
  v3 = 0;
  while( true ) {
    if (a1 <= v3)
      return 0;
    v1 = sub_f6da(&a0[v3],a1 - v3,&v2);
    if (v1 <= -1) break;
    v3 += v1;
  }
  return 1;
}


// Function: sub_a011 @ 0xa011
void sub_a011(void)
{
  int v1;
  int v2 [32];
  mbstate_t v3; // stack - 0xb0
  char v4 [24];
  int v5; // stack - 0xc0
  char v6; // stack - 0xbc
  int v7; // stack - 0xb8
  
  v5 = 0;
  do {
    if (0x100 <= v5)
      return;
    v1 = *(int *)(((long)v5 + 0x40) * 4 + 0x331a4);
    if (v1 != -1) {
      v6 = 1;
      v7 = sub_1c01a(v1,v2);
      do {
        v7 -= 1;
        if (v7 <= -1) goto label_a0e9;
        v3 = 0;
        v1 = v2[v7];
      } while (wcrtomb(v4,v1,&v3) == 1);
      v6 = 0xff;
label_a0e9:
      *(char *)((long)v5 + 0x338c0) = v6;
    }
    v5 += 1;
  } while( true );
}


// Function: sub_a134 @ 0xa134
unsigned long sub_a134(unsigned char *a0,unsigned long a1,mbstate_t *a2) // early-return x3
{
  unsigned char v1;
  unsigned int v2; // stack - 0xac
  char v3 [136];
  unsigned long v4; // stack - 0xa8
  unsigned long v5; // stack - 0xa0
  
  if (*(int *)(((long)(int)(unsigned int)*a0 + 0x40) * 4 + 0x331a4) != -1)
    return (long)*(char *)((long)(int)(unsigned int)*a0 + 0x338c0);
  v5 = sub_21b80(&v2,a0,a1,a2);
  if (0x11 <= v5)
    return 0xffffffffffffffff;
  if (sub_1c01a(v2,v3))
    return 0xffffffffffffffff;
  v4 = v5;
  do {
    v4 -= 1;
    if ((long)v4 <= 0)
      return v5;
    v1 = a0[v4];
  } while (toupper((unsigned int)v1) == (unsigned int)v1);
  return 0xffffffffffffffff;
}


// Function: sub_a29a @ 0xa29a
unsigned long sub_a29a(char *a0,long a1)
{
  int v1; // eax
  unsigned long v2; // stack - 0x18
  long v3; // stack - 0x20
  
  v2 = 0;
  v3 = 0;
  while( true ) {
    if (a1 <= v3)
      return 1;
    v1 = sub_a134(&a0[v3],a1 - v3,&v2);
    if (v1 <= -1) break;
    v3 += v1;
  }
  return 0;
}


// Function: sub_a32f @ 0xa32f
void sub_a32f(unsigned long *a0,unsigned long *a1)
{
  char *v1;
  char *v2; // rax
  unsigned long v3; // stack - 0x18
  unsigned long v4; // stack - 0x40
  char *v5; // stack - 0x38
  char *v6; // stack - 0x30
  unsigned long v7; // stack - 0x28
  
  v4 = *a1;
  v5 = (char *)*a0;
  v3 = 0;
  v2 = (char *)sub_26c63(v4 + 1,2);
  v6 = v2;
  do {
    if (!v4) {
      *v6 = '\n';
      free((void *)*a0);
      *a0 = v2;
      *a1 = (long)v6 - (long)v2;
      return;
    }
    v7 = sub_f6da(v5,v4,&v3);
    if (v7 != 1) { // branch-flip
      if ((long)v7 <= 1) {
        if (v7 != 0xfffffffffffffffe) { // branch-flip
          if (v7 == 0xffffffffffffffff) {
            memset(&v3,0,8);
            v7 = 1;
            goto label_a412;
          }
        }
        else {
          v7 = v4;
        }
      }
      v6 = mempcpy(v6,v5,v7);
    }
    else {
label_a412:
      if (((unsigned int)((int)*v5 - 0x24U) <= 0x3a) && (0x580000000000441U >> ((unsigned char)((int)*v5 - 0x24U) & 0x3f) & 1)) {
        v1 = &v6[1];
        *v6 = '\\';
        v6 = v1;
      }
      v1 = &v6[1];
      *v6 = *v5;
      v6 = v1;
    }
    v5 = &v5[v7];
    v4 -= v7;
  } while( true );
}


// Function: sub_a4d3 @ 0xa4d3
int sub_a4d3(int a0,unsigned char *a1,unsigned long *a2)
{
  unsigned char v1;
  void *v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // stack - 0x18
  int v5; // stack - 0x4c
  unsigned long v6; // stack - 0x48
  void *v7; // stack - 0x40
  unsigned char *v8; // stack - 0x38
  
  v6 = *a2;
  v2 = (void *)sub_26b10(v6 + 1);
  v7 = v2;
  v4 = 0;
  v8 = a1;
  for (; v6; v6 = v6 - v3) {
    v1 = *v8;
    v5 = a0;
    if ((char)v1 <= '|') {
      if ('{' <= (char)v1) goto label_a5e5;
      if (v1 == 0x5e) {
        free(v2); // return-dupe
        return v5;
      }
      if ((char)v1 <= '^') {
        if (v1 != 0x5c) { // branch-flip
          if ((char)v1 <= '\\') {
            if (v1 == 0x5b) {
              free(v2);
              return v5;
            }
            if ((char)v1 <= '[') {
              if ('/' <= (char)v1) { // branch-flip
                if (v1 == 0x3f) {
label_a5e5:
                  if (a0) {
                    free(v2);
                    return v5;
                  }
                }
              }
              else if ('$' <= (char)v1) {
                v3 = 1L << (v1 & 0x3f);
                if (v3 & 0x441000000000) {
                  free(v2);
                  return v5;
                }
                if (v3 & 0x90000000000) goto label_a5e5;
              }
            }
          }
        }
        else if (2 <= (long)v6) {
          v1 = v8[1];
          if ('=' <= (char)v1) { // branch-flip
            if (((char)v1 <= '|') && ('>' <= (char)v1)) {
              v3 = 1L << (v1 - 0x3e & 0x3f);
              if (!(v3 & 0x220001402200011)) {
                v3 &= 0x6000000000000002;
                goto label_a65d;
              }
              free(v2);
              return v5;
            }
          }
          else if ('\n' <= (char)v1) {
            v3 = 1L << (v1 & 0x3f);
            if (v3 & 0x13fe008000000400) {
              free(v2);
              return v5;
            }
            v3 &= 0xb0000000000;
label_a65d:
            if ((v3) && (!a0)) {
              free(v2);
              return v5;
            }
          }
          v8 = &v8[1];
          v6 -= 1;
        }
      }
    }
    if (dat_33180) // branch-flip
      v3 = sub_a134(v8,v6,&v4);
    else {
      v3 = sub_f6da(v8,v6,&v4);
    }
    if ((long)v3 < 0) {
      free(v2);
      return v5;
    }
    v7 = mempcpy(v7,v8,v3);
    v8 = &v8[v3];
  }
  if (*a2 != (long)v7 - (long)v2) {
    *a2 = (long)v7 - (long)v2;
    *(char *)mempcpy(a1,v2,(long)v7 - (long)v2) = 10;
  }
  v5 = 2;
  free(v2);
  return v5;
}


// Function: main @ 0xa7d1
int main(int argc,char **argv,char **envp) // ternary x3, warn: iteboolean: re-rolled 2 0/1 select diamond(s) to a boolean assignment in main
{
  char **v1;
  int v10; // eax
  char *v11;
  unsigned long v12; // rax
  long v13;
  long v14; // stack - 0x1c8
  long v15; // stack - 0x1d0
  stat v16; // stack - 0x148
  stat v17; // stack - 0xb8
  long v18; // stack - 0x1d8
  char v19; // stack - 0x22
  char v2 [16];
  char v20 [8];
  bool v21; // stack - 0x202
  int v22; // stack - 0x200
  int v23; // stack - 0x1fc
  unsigned int v24; // stack - 0x1f8
  int v25; // stack - 0x1f4
  int v26; // stack - 0x1f0
  int v27; // stack - 0x1ec
  int v28; // stack - 0x1e8
  char *v29; // stack - 0x1b8
  char v3 [16];
  FILE *v30; // stack - 0x1b0
  long v31; // stack - 0x1a8
  char *v32; // stack - 0x1a0
  char **v33; // stack - 0x198
  char *v34; // stack - 0x190
  unsigned long v35; // stack - 0x188
  char *v36; // stack - 0x180
  unsigned long v37; // stack - 0x178
  long v38; // stack - 0x170
  unsigned long v39; // stack - 0x168
  bool v4;
  unsigned long v40; // stack - 0x160
  long v41; // stack - 0x158
  char *v42; // stack - 0x150
  char v43; // stack - 0x23
  char v44; // stack - 0x21
  unsigned char v5; // al
  char v6;
  unsigned int v7; // eax
  unsigned int v8; // eax
  int v9;
  
  v29 = NULL;
  v18 = 0;
  v15 = 0;
  v22 = -1;
  dat_33080 = 2;
  v24 = 0;
  dat_33183 = 10;
  dat_3384c = 0xffffffff;
  dat_33870 = 0x7fffffffffffffff;
  dat_33858 = -1;
  dat_33860 = -1;
  v14 = -1;
  dat_3375c = '\0';
  setlocale(6,(char *)0x290d2);
  bindtextdomain("grep","/usr/local/share/locale");
  textdomain("grep");
  sub_1bf18((char *)0x331a0);
  sub_285f0(sub_6f2c);
  sub_112dd(NULL);
  v23 = 0;
  dat_33790 = sub_208d6(0,0,sub_6760,sub_67f8,0);
  if (!dat_33790)
    sub_27122(); // no-return
label_b57f:
  do {
    v9 = optind;
    v10 = sub_9c9f(argc,argv,&v14);
    if (v10 == -1) {
      if (dat_33754) {
        sub_2690a(stdout,sub_1ff0c(),"GNU grep","3.8",0);
        puts(gettext("Written by Mike Haertel and others; see\n<https://git.sv.gnu.org/cgit/grep.git/tree/AUTHORS>."));
        return 0;
      }
      if (dat_33750)
        sub_99b6(0); // no-return
      if (v29) { // branch-flip
        if (!v18) {
          dat_33851 = dat_33851 != '\x01';
          dat_33181 = '\0';
          dat_33182 = '\0';
          v18 = 1;
          *v29 = '\n';
        }
      }
      else {
        if (argc <= optind)
          sub_99b6(2); // no-return
        v9 = optind + 1;
        v34 = argv[optind];
        v5 = (((v22 != 2) && (*v34 == '\\')) && (v34[1] == '-'));
        optind = v9;
        dat_33780 = (char *)sub_270f1(&v34[v5]);
        v29 = dat_33780;
        v12 = strlen(dat_33780);
        v35 = v12;
        v29[v12] = '\n';
        v18 = sub_687b(v29,0,v12 + 1,0x290d2);
      }
      v18 -= 1;
      sub_20b56(dat_33790);
      v4 = 0;
      if ((dat_338b1 != '\x01') && (!fstat(1,&v16))) {
        if ((v16._24_4_ & 0xf000) != 0x8000) { // branch-flip
          if ((v16._24_4_ & 0xf000) == 0x2000) {
            if (((stat("/dev/null",&v17)) || (v16._8_8_ != v17._8_8_)) || (v16._0_8_ != v17._0_8_))
              v4 = 1;
            else {
              dat_338b2 = '\x01';
            }
          }
        }
        else {
          dat_336c0 = v16._0_8_;
          dat_336c8 = v16._8_8_;
          dat_336d8 = CONCAT44(v16._28_4_,v16._24_4_);
          dat_336d0 = v16._16_8_;
          dat_336e0 = v16._32_8_;
          dat_336e8 = v16._40_8_;
          dat_336f0 = v16._48_8_;
          dat_336f8 = v16._56_8_;
          dat_33700 = v16._64_8_;
          dat_33708 = v16._72_8_;
          dat_33710 = v16._80_8_;
          dat_33718 = v16._88_8_;
          dat_33720 = v16._96_8_;
          dat_33728 = v16._104_8_;
          dat_33730 = v16._112_8_;
          dat_33738 = v16._120_8_;
          dat_33740 = v16._128_8_;
          dat_33748 = v16._136_8_;
        }
      }
      if (dat_338b2 || dat_338b1) { // branch-flip
        dat_33844 = 0;
label_b9b9:
        dat_33868 = '\0';
        dat_338b0 = '\x01';
      }
      else if (dat_33844) goto label_b9b9;
      dat_33850 = dat_338b0 || dat_33868;
      if (dat_33860 <= -1)
        dat_33860 = v14;
      if (dat_33858 <= -1)
        dat_33858 = v14;
      if (((!dat_33870) || ((((!v18 && (dat_33851)) && (dat_33182 != '\x01')) && (dat_33181 != '\x01')))) && (dat_33844 != 2))
        return 1;
      if (dat_33758 == 2) {
        dat_33758 = !(((!v4) || (!sub_2768b())) || (!isatty(1)));
      }
      sub_276d9();
      if (!dat_33758) goto label_bbb8;
      v36 = getenv("GREP_COLOR");
      if ((!v36) || (!*v36)) goto label_bb59;
      v32 = v36;
      goto label_bb33;
    }
    switch(v10) {
      case 0:
      case 0x55:
        goto label_b57f;
      default:
        sub_99b6(2); // no-return
      case 0x41:
        sub_728f(optarg,(long *)0x33860);
        goto label_b57f;
      case 0x42:
        sub_728f(optarg,(long *)0x33858);
        goto label_b57f;
      case 0x43:
        sub_728f(optarg,&v14);
        goto label_b57f;
      case 0x44:
        if (!strcmp(optarg,"read")) {
          dat_337cc = 1;
          goto label_b57f;
        }
        if (!strcmp(optarg,"skip")) {
          dat_337cc = 2;
          goto label_b57f;
        }
        error(2,0,gettext("unknown devices method"));
        goto label_aa4b;
      case 0x45:
label_aa4b:
        v22 = sub_9be6("egrep",v22);
        goto label_b57f;
      case 0x46:
        v22 = sub_9be6("fgrep",v22);
        break;
      case 0x47:
        v22 = sub_9be6("grep",v22);
        break;
      case 0x48:
        v24 = 1;
        break;
      case 0x49:
        dat_33840 = 2;
        break;
      case 0x4c:
        dat_33844 = 2;
        break;
      case 0x50:
        v22 = sub_9be6("perl",v22);
        break;
      case 0x52:
        dat_33014 = 0x30a;
        goto label_b073;
      case 0x71:
        dat_338b1 = '\x01';
        dat_33080 = 0;
        break;
      case 0x54:
        dat_3375d = 1;
        break;
      case 0x56:
        dat_33754 = '\x01';
        break;
      case 0x58:
        v22 = sub_9be6(optarg,v22);
        break;
      case 0x5a:
        dat_3384c = 0;
        break;
      case 0x61:
        dat_33840 = 1;
        break;
      case 0x62:
        dat_33853 = 1;
        break;
      case 99:
        dat_33868 = '\x01';
        break;
      case 100:
        dat_33010 = *(int *)(sub_10c86("--directories",optarg,0x32700,0x293a8,4,dat_33078,1) * 4 + 0x293a8);
        if (dat_33010 == 3)
          v23 = v9;
        break;
      case 0x65:
        v40 = strlen(optarg);
        v41 = v40 + (v18 - v15) + 1;
        if (1 <= v41) {
          dat_33780 = (char *)sub_26dc6(v29,&v15,v41,-1,1);
          v29 = dat_33780;
        }
        v42 = mempcpy(&v29[v18],optarg,v40);
        *v42 = 10;
        v13 = v40 + v18;
        v18 = sub_687b(v29,v18,v13 + 1,0x290d2);
        break;
      case 0x66:
        if (strcmp(optarg,"-")) { // branch-flip
          v11 = (dat_338b3) ? "rb" : "r"; // branch-flip
          v30 = fopen(optarg,v11);
          v11 = optarg;
          if (!v30)
            error(2,*__errno_location(),"%s",v11);
        }
        else {
          if (dat_338b3)
            sub_2716d(0,0);
          v30 = stdin;
        }
        v31 = v18;
        while( true ) {
          v38 = (v31 - v15) + 2;
          if (1 <= v38) {
            dat_33780 = (char *)sub_26dc6(v29,&v15,v38,-1,1);
            v29 = dat_33780;
          }
          v39 = fread_unlocked(&v29[v31],1,v15 - (v31 + 1),v30);
          if (!v39) break;
          v31 += v39;
        }
        v25 = *__errno_location();
        if (!ferror_unlocked(v30)) {
          v25 = 0;
          if (v30 != stdin) { // branch-flip
            if (fclose(v30))
              v25 = *__errno_location();
          }
          else {
            clearerr_unlocked(v30);
          }
        }
        if (v25)
          error(2,v25,"%s",optarg);
        if ((v31 != v18) && (v29[v31 + -1] != '\n')) {
          v13 = v31 + 1;
          v29[v31] = '\n';
          v31 = v13;
        }
        v18 = sub_687b(v29,v18,v31,optarg);
        break;
      case 0x68:
        v24 = 0xffffffff;
        break;
      case 0x69:
      case 0x79:
        dat_33180 = '\x01';
        break;
      case 0x6c:
        dat_33844 = 1;
        break;
      case 0x6d:
        v7 = sub_27281(optarg,NULL,10,(unsigned long *)0x33870,(char *)0x290d2);
        if (2 <= v7)
          error(2,0,gettext("invalid max count"));
        break;
      case 0x6e:
        dat_33852 = 1;
        break;
      case 0x6f:
        dat_3375c = '\x01';
        break;
      case 0x72:
label_b073:
        dat_33010 = 3;
        v23 = v9;
        break;
      case 0x73:
        dat_33755 = 1;
        break;
      case 0x75:
        error(0,0,gettext("warning: --unix-byte-offsets (-u) is obsolete"));
        break;
      case 0x76:
        dat_33851 = '\x01';
        break;
      case 0x77:
        sub_f770();
        dat_33181 = '\x01';
        break;
      case 0x78:
        dat_33182 = '\x01';
        break;
      case 0x7a:
        dat_33183 = 0;
        break;
      case 0x80:
        if (strcmp(optarg,"binary")) { // branch-flip
          if (strcmp(optarg,"text")) { // branch-flip
            if (strcmp(optarg,"without-match")) {
              error(2,0,gettext("unknown binary-files type"));
              goto label_b188;
            }
            dat_33840 = 2;
          }
          else {
            dat_33840 = 1;
          }
        }
        else {
          dat_33840 = 0;
        }
        break;
      case 0x81:
label_b188:
        if (optarg) { // branch-flip
          v9 = sub_11390(optarg,"always");
          if (((v9) && (v9 = sub_11390(optarg,"yes"), v9)) && (v9 = sub_11390(optarg,"force"), v9)) {
            v9 = sub_11390(optarg,"never");
            if (((v9) && (v9 = sub_11390(optarg,"no"), v9)) && (v9 = sub_11390(optarg,"none"), v9)) {
              v9 = sub_11390(optarg,"auto");
              if (((v9) && (v9 = sub_11390(optarg,"tty"), v9)) && (v9 = sub_11390(optarg,"if-tty"), v9))
                dat_33750 = 1;
              else {
                dat_33758 = 2;
              }
            }
            else {
              dat_33758 = 0;
            }
          }
          else {
            dat_33758 = 1;
          }
        }
        else {
          dat_33758 = 2;
        }
        break;
      case 0x82:
        sub_1c12b(optarg);
        for (v28 = 0; v28 <= 1; v28 = v28 + 1) {
          if (!*(long *)((long)v28 * 8 + 0x337b0))
            *(unsigned long *)((long)v28 * 8 + 0x337b0) = sub_1c384();
          v8 = sub_7316(v28 != 0);
          sub_1cca9(*(unsigned long *)((long)v28 * 8 + 0x337b0),optarg,v8);
        }
        break;
      case 0x83:
      case 0x86:
        for (v26 = 0; v26 <= 1; v26 = v26 + 1) {
          if (!*(long *)((long)v26 * 8 + 0x337a0))
            *(unsigned long *)((long)v26 * 8 + 0x337a0) = sub_1c384();
          v7 = (v10 != 0x86) ? 0 : 0x20000000; // branch-flip
          sub_1cca9(*(unsigned long *)((long)v26 * 8 + 0x337a0),optarg,v7 | sub_7316(v26 != 0));
        }
        break;
      case 0x84:
        for (v27 = 0; v27 <= 1; v27 = v27 + 1) {
          if (!*(long *)((long)v27 * 8 + 0x337a0))
            *(unsigned long *)((long)v27 * 8 + 0x337a0) = sub_1c384();
          v8 = sub_7316(v27 != 0);
          v9 = sub_1d272(sub_1cca9,*(unsigned long *)((long)v27 * 8 + 0x337a0),optarg,v8,10);
          v11 = optarg;
          if (v9)
            error(2,*__errno_location(),"%s",v11);
        }
        break;
      case 0x85:
        dat_33020 = optarg;
        break;
      case 0x87:
        dat_33878 = 1;
        break;
      case 0x88:
        dat_33880 = optarg;
        break;
      case 0x89:
        dat_33180 = '\0';
      
    }
  } while( true );
label_bb33:
  if ((*v32 != ';') && (v6 = *v32, !sub_10eab((int)v6))) goto label_bb59;
  if (!v32[1]) {
    dat_33030 = v36;
    dat_33028 = v36;
label_bb59:
    sub_9ddf();
    if ((v36 == dat_33028) || (v36 == dat_33030))
      error(0,0,gettext("warning: GREP_COLOR=\'%s\' is deprecated; use GREP_COLORS=\'mt=%s\'"),v36,v36);
label_bbb8:
    sub_6f46();
    if (v22 <= -1)
      v22 = 0;
    if (((v22 == 2) || (v22 == 1)) || (!v22)) {
      if (dat_33180)
        sub_a011();
      if (v22 != 2) { // branch-flip
        if (2 <= dat_33788)
          v22 = sub_a4d3(v22,v29,&v18);
      }
      else {
        if (dat_331a0 != '\x01') { // branch-flip
          if ((dat_33788 != 1) || (!dat_33181))
            v4 = 0;
          else {
            v4 = 1;
          }
        }
        else {
          v6 = sub_9f7b(v29,v18);
          if ((v6) || ((dat_33180 && (v6 = sub_a29a(v29,v18), v6 != '\x01'))))
            v4 = 1;
          else {
            v4 = 0;
          }
        }
        if (v4) {
          sub_a32f((unsigned long *)0x33780,&v18);
          v29 = dat_33780;
          v22 = 0;
        }
      }
    }
    dat_337d8 = *(void **)((long)v22 * 0x20 + 0x327b8);
    dat_337e0 = (**(void **)((long)v22 * 0x20 + 0x327b0))(v29,v18,(long)*(int *)((long)v22 * 0x20 + 0x327ac),dat_33758 || dat_3375c);
    v43 = 0;
    v19 = dat_33183;
    v44 = 0;
    v13 = (*dat_337d8)(dat_337e0,&v19,1,v20,0);
    dat_33831 = (v13 == 0) == (bool)dat_33851;
    argc -= optind;
    if ((v24) || (2 <= argc))
      dat_33848 = ~v24 >> 0x1f;
    else {
      dat_33848 = -(unsigned int)(dat_33010 == 3);
    }
    if (dat_338b3)
      sub_2716d(1,0);
    v37 = sysconf(0x1e);
    if ((1 <= (long)v37) && ((long)v37 <= 0x3ffffffffffffffb)) {
      v2._8_8_ = 0;
      v2._0_8_ = v37;
      if (SUB168((ZEXT816(0) << 0x40 | ZEXT816(0x18000)) % v2._0_16_,0)) { // branch-flip
        v3._8_8_ = 0;
        v3._0_8_ = v37;
        v13 = (v37 - SUB168((ZEXT816(0) << 0x40 | ZEXT816(0x18000)) % v3._0_16_,0)) + 0x18000;
      }
      else {
        v13 = 0x18000;
      }
      dat_337f8 = v37 + v13 + 8;
      dat_33818 = v37;
      dat_337f0 = sub_26b10(dat_337f8);
      if ((dat_33014 & 2) && (!dat_337cc))
        dat_337cc = 1;
      if (1 <= argc) // branch-flip
        v33 = &argv[optind];
      else if ((dat_33010 != 3) || (v23 <= 0))
        v33 = (char **)0x32730;
      else {
        v33 = (char **)0x32720;
        dat_337c8 = 1;
      }
      v21 = 1;
      do {
        v1 = &v33[1];
        v11 = *v33;
        v33 = v1;
        v21 = (sub_992b(v11) & v21) != 0;
      } while (*v33);
      v7 = (dat_337c9) ? 2 : (unsigned int)v21; // branch-flip
      return v7;
    }
    abort(); // no-return
  }
  v32 = &v32[1];
  goto label_bb33;
}


// Function: sub_c03f @ 0xc03f
unsigned long * sub_c03f(char *a0,long a1)
{
  unsigned long *v1; // rax
  unsigned long v2; // stack - 0x50
  char *v3; // stack - 0x48
  char *v4; // stack - 0x40
  unsigned long v5; // stack - 0x38
  unsigned long v6; // stack - 0x30
  void *v7; // stack - 0x28
  unsigned long v8; // stack - 0x20
  
  v3 = NULL;
  v2 = 0;
  v6 = sub_f7c8(1);
  v4 = a0;
  do {
    v7 = rawmemchr(v4,10);
    v5 = (long)v7 - (long)v4;
    if (dat_33182) {
      if ((dat_33183 != '\n') || (v4 <= a0)) {
        if ((long)v2 <= (long)(v5 + 1)) {
          free(v3);
          v2 = v5;
          v3 = (char *)sub_26dc6(NULL,&v2,2,-1,1);
          *v3 = dat_33183;
        }
        memcpy(&v3[1],v4,v5);
        v3[v5 + 1] = dat_33183;
        v4 = v3;
      }
      else {
        v4 = &v4[-1];
      }
      v5 += 2;
    }
    sub_cacb(v6,v4,v5);
    v4 = (char *)((long)v7 + 1);
  } while (v4 <= &a0[a1]);
  free(v3);
  v8 = sub_d633(v6);
  sub_d9b5(v6);
  v1 = (unsigned long *)sub_26aea(0x28);
  *v1 = v6;
  v1[1] = v8;
  v1[2] = a0;
  v1[3] = a1;
  v1[4] = 0;
  return v1;
}


// Function: sub_c23c @ 0xc23c
long sub_c23c(long *a0,void *a1,long a2,long *a3,void *a4)
{
  long *v1;
  void *v10; // stack - 0x90
  char v11 [16];
  bool v12; // zf
  void *v13; // stack - 0x80
  long v14; // stack - 0x78
  long v15; // stack - 0x70
  long *v16; // stack - 0x68
  long v17; // stack - 0x60
  long v18; // stack - 0x58
  void *v19; // stack - 0x50
  char v2;
  long v20; // stack - 0x38
  long v21; // stack - 0x18
  unsigned char v3; // al
  long v4; // rax
  unsigned char v5; // dl
  char *v6; // rdx
  void *v7; // rax
  char v8 [16];
  long v9; // stack - 0x88
  
  v2 = dat_33183;
  v17 = *a0;
  v12 = (unsigned char)((dat_33182 ^ 1) & dat_331a0 & (dat_331a2 ^ 1)) != 0;
  v5 = (v12 || a4) | dat_33181;
  v3 = dat_33182 ^ 1;
  if (a4) // branch-flip
    v10 = a4;
  else {
    v10 = a1;
  }
  v13 = v10;
  v16 = a0;
  do {
    if (((void *)((long)a1 + a2) < v13) || (v18 = sub_f585(v17,(long)v13 - (unsigned long)dat_33182,(long)a1 + (unsigned long)dat_33182 + (a2 - (long)v13),v8,(v3 & v5) != 0), v18 < 0))
      return -1;
    v15 = v20 - (int)((unsigned int)dat_33182 * 2);
    v9 = 0;
    if ((v12) && (v6 = (char *)(v18 + (long)v13), sub_f84a(&v10,&v9,v6,a2 + (long)a1)))
      v13 = (void *)((long)v10 + -1);
    else {
      v13 = (void *)((long)v13 + v18);
      if ((dat_33181 ^ 1) & a4 != NULL) {
label_c7f0:
        *a3 = v15;
        return (long)v13 - (long)a1;
      }
      if (dat_33182) {
        v15 += (unsigned long)(a4 == NULL);
        goto label_c7f0;
      }
      if (dat_33181 != 1) {
label_c73e:
        if ((unsigned long)((long)a1 + a2) <= (unsigned long)(v15 + (long)v13)) // branch-flip
          v14 = (long)a1 + a2;
        else {
          v7 = (void *)((long)v13 + v15);
          v14 = (long)rawmemchr(v7,(int)v2) + 1;
        }
label_c79b:
        v7 = memrchr(a1,(int)v2,(long)v13 - (long)a1);
        if (v7) // branch-flip
          v13 = (void *)((long)v7 + 1);
        else {
          v13 = a1;
        }
        v15 = v14 - (long)v13;
        goto label_c7f0;
      }
      if (!v9) {
        v19 = memrchr(v10,(int)v2,(long)v13 - (long)v10);
        if (v19)
          v10 = (void *)((long)v19 + 1);
      }
      if (1 <= v9) // branch-flip
        v4 = sub_fbe0((long)v13 - v9,a2 + (long)a1);
      else {
        v4 = sub_fc0e(v10,v13,a2 + (long)a1);
      }
      if (!v4) {
        while( true ) {
          v4 = (long)v13 + v15;
          if (!sub_fbe0(v4,a2 + (long)a1)) {
            if (!a4) goto label_c73e;
            goto label_c7f0;
          }
          if ((!a4) && (dat_331a0 != 1)) break;
          if (!v15) goto label_c6dc;
          v15 -= 1;
          if (sub_f585(v17,v13,v15,v11,1)) goto label_c6dc;
          v15 = v21;
        }
        if (!v16[4]) {
          sub_a32f(&v16[2],&v16[3]);
          v4 = sub_5661((char *)v16[2],v16[3],0x10a46,0);
          v16[4] = v4;
        }
        if ((unsigned long)((long)a1 + a2) <= (unsigned long)(v15 + (long)v13)) // branch-flip
          v14 = (long)a1 + a2;
        else {
          v7 = (void *)((long)v13 + v15);
          v14 = (long)rawmemchr(v7,(int)v2) + 1;
        }
        v4 = v14 - (long)v13;
        v1 = (long *)v16[4];
        if (0 <= (long)sub_5d23(v1,v13,v4,a3,NULL)) goto label_c79b;
        v13 = (void *)(v14 + -1);
      }
label_c6dc:
      v4 = sub_fbb2(v13,a2 + (long)a1);
      v10 = (void *)((long)v13 + v4);
      v13 = v10;
    }
    v13 = (void *)((long)v13 + 1);
  } while( true );
}


// Function: sub_c845 @ 0xc845
void sub_c845(char a0)
{
  sub_f64d((int)a0);
}


// Function: sub_c863 @ 0xc863
char sub_c863(long a0,char a1)
{
  if (a0)
    a1 = *(char *)(a0 + (unsigned long)(unsigned char)sub_c845((int)a1));
  return a1;
}


// Function: sub_c89b @ 0xc89b
void * sub_c89b(unsigned long a0)
{
  long v1;
  void *v2; // rax
  
  v2 = (void *)sub_26aea(0x9a0);
  _obstack_begin(v2,0,0,sub_26aea,dat_32fc8);
  *(unsigned long *)((long)v2 + 0x58) = 0;
  if ((unsigned long)(*(long *)((long)v2 + 0x20) - *(long *)((long)v2 + 0x18)) < 0x40)
    _obstack_newchunk(v2,0x40);
  *(long *)((long)v2 + 0x18) = *(long *)((long)v2 + 0x18) + 0x40;
  v1 = *(long *)((long)v2 + 0x10);
  if (v1 == *(long *)((long)v2 + 0x18))
    *(unsigned char *)((long)v2 + 0x50) = *(unsigned char *)((long)v2 + 0x50) | 2;
  *(unsigned long *)((long)v2 + 0x18) = ~*(unsigned long *)((long)v2 + 0x30) & *(long *)((long)v2 + 0x18) + *(long *)((long)v2 + 0x30);
  if ((unsigned long)(*(long *)((long)v2 + 0x20) - *(long *)((long)v2 + 8)) < (unsigned long)(*(long *)((long)v2 + 0x18) - *(long *)((long)v2 + 8)))
    *(unsigned long *)((long)v2 + 0x18) = *(unsigned long *)((long)v2 + 0x20);
  *(unsigned long *)((long)v2 + 0x10) = *(unsigned long *)((long)v2 + 0x18);
  *(long *)((long)v2 + 0x60) = v1;
  **(unsigned long **)((long)v2 + 0x60) = 0;
  *(unsigned long *)(*(long *)((long)v2 + 0x60) + 8) = 0;
  *(unsigned long *)(*(long *)((long)v2 + 0x60) + 0x10) = 0;
  *(unsigned long *)(*(long *)((long)v2 + 0x60) + 0x18) = 0;
  *(unsigned long *)(*(long *)((long)v2 + 0x60) + 0x20) = 0;
  *(unsigned long *)(*(long *)((long)v2 + 0x60) + 0x28) = 0;
  *(unsigned long *)(*(long *)((long)v2 + 0x60) + 0x30) = 0;
  *(unsigned long *)((long)v2 + 0x68) = 0x7fffffffffffffff;
  *(unsigned long *)((long)v2 + 0x970) = 0;
  *(unsigned long *)((long)v2 + 0x980) = a0;
  *(void **)((long)v2 + 0x998) = sub_f50c;
  return v2;
}


// Function: sub_cacb @ 0xcacb
void sub_cacb(void *a0,unsigned char *a1,long a2) // return-dupe, ternary x2
{
  unsigned char *v1;
  char v10; // dl
  long *v11 [13]; // stack - 0x78
  int v12 [12]; // stack - 0xa8
  bool v13; // zf
  long v14; // stack - 0x170
  unsigned char *v15; // stack - 0x168
  long *v16; // stack - 0x150
  long *v17; // stack - 0x148
  long v18; // stack - 0x140
  long *v19; // stack - 0x138
  long v2;
  long v3;
  long v4;
  long *v5;
  long v6;
  long *v7;
  long v8;
  unsigned char v9;
  
  v16 = *(long **)((long)a0 + 0x60);
  v3 = *(long *)((long)a0 + 0x980);
  v13 = *(void **)((long)a0 + 0x998) == sub_f061;
  v14 = a2;
  v15 = a1;
  if (v13) {
    v15 = &a1[a2];
    v14 = a2;
  }
  do {
    v2 = v14 + -1;
    if (!v14) {
      if (!*v16)
        *v16 = *(long *)((long)a0 + 0x58) * 2 + 1;
      *(long *)((long)a0 + 0x58) = *(long *)((long)a0 + 0x58) + 1;
      if (*(long *)((long)a0 + 0x68) > v16[5]) {
        *(long *)((long)a0 + 0x68) = v16[5];
        return;
      }
      return;
    }
    if (v13) {
      v15 = &v15[-1];
      v9 = *v15;
    }
    else {
      v1 = &v15[1];
      v9 = *v15;
      v15 = v1;
    }
    if (v3)
      v9 = *(unsigned char *)(v3 + (unsigned long)v9);
    v17 = (long *)v16[1];
    v11[0] = &v16[1];
    v12[0] = 0;
    v18 = 1;
    while ((v17 && (v9 != *(unsigned char *)&v17[3]))) {
      v11[v18] = v17;
      if (*(unsigned char *)&v17[3] <= v9) { // branch-flip
        v4 = v18 + 1;
        v12[v18] = 1;
        v17 = (long *)v17[1];
        v18 = v4;
      }
      else {
        v4 = v18 + 1;
        v12[v18] = 0;
        v17 = (long *)*v17;
        v18 = v4;
      }
    }
    if (!v17) {
      if ((unsigned long)(*(long *)((long)a0 + 0x20) - *(long *)((long)a0 + 0x18)) < 0x20)
        _obstack_newchunk(a0,0x20);
      *(long *)((long)a0 + 0x18) = *(long *)((long)a0 + 0x18) + 0x20;
      v17 = *(long **)((long)a0 + 0x10);
      if (v17 == *(long **)((long)a0 + 0x18))
        *(unsigned char *)((long)a0 + 0x50) = *(unsigned char *)((long)a0 + 0x50) | 2;
      *(unsigned long *)((long)a0 + 0x18) = ~*(unsigned long *)((long)a0 + 0x30) & *(long *)((long)a0 + 0x18) + *(long *)((long)a0 + 0x30);
      if ((unsigned long)(*(long *)((long)a0 + 0x20) - *(long *)((long)a0 + 8)) < (unsigned long)(*(long *)((long)a0 + 0x18) - *(long *)((long)a0 + 8)))
        *(unsigned long *)((long)a0 + 0x18) = *(unsigned long *)((long)a0 + 0x20);
      *(unsigned long *)((long)a0 + 0x10) = *(unsigned long *)((long)a0 + 0x18);
      *v17 = 0;
      v17[1] = 0;
      if ((unsigned long)(*(long *)((long)a0 + 0x20) - *(long *)((long)a0 + 0x18)) < 0x40)
        _obstack_newchunk(a0,0x40);
      *(long *)((long)a0 + 0x18) = *(long *)((long)a0 + 0x18) + 0x40;
      v4 = *(long *)((long)a0 + 0x10);
      if (v4 == *(long *)((long)a0 + 0x18))
        *(unsigned char *)((long)a0 + 0x50) = *(unsigned char *)((long)a0 + 0x50) | 2;
      *(unsigned long *)((long)a0 + 0x18) = ~*(unsigned long *)((long)a0 + 0x30) & *(long *)((long)a0 + 0x18) + *(long *)((long)a0 + 0x30);
      if ((unsigned long)(*(long *)((long)a0 + 0x20) - *(long *)((long)a0 + 8)) < (unsigned long)(*(long *)((long)a0 + 0x18) - *(long *)((long)a0 + 8)))
        *(unsigned long *)((long)a0 + 0x18) = *(unsigned long *)((long)a0 + 0x20);
      *(unsigned long *)((long)a0 + 0x10) = *(unsigned long *)((long)a0 + 0x18);
      v17[2] = v4;
      *(unsigned long *)v17[2] = 0;
      *(unsigned long *)(v17[2] + 8) = 0;
      *(long **)(v17[2] + 0x10) = v16;
      *(unsigned long *)(v17[2] + 0x18) = 0;
      *(unsigned long *)(v17[2] + 0x20) = 0;
      *(long *)(v17[2] + 0x28) = v16[5] + 1;
      *(unsigned long *)(v17[2] + 0x30) = 0;
      *(unsigned char *)&v17[3] = v9;
      *(char *)((long)v17 + 0x19) = 0;
      v18 -= 1;
      if (v12[v18]) // branch-flip
        v11[v18][1] = (long)v17;
      else {
        *v11[v18] = (long)v17;
      }
      while ((v18 && (!*(char *)((long)v11[v18] + 0x19)))) {
        if (v12[v18]) // branch-flip
          *(char *)((long)v11[v18] + 0x19) = *(char *)((long)v11[v18] + 0x19) + '\x01';
        else {
          *(char *)((long)v11[v18] + 0x19) = *(char *)((long)v11[v18] + 0x19) + '\xff';
        }
        v18 -= 1;
      }
      if (v18) {
        if (v12[v18]) { // branch-flip
label_d19a:
          if (v12[v18] == 1) {
            v5 = v11[v18];
            *(char *)((long)v5 + 0x19) = *(char *)((long)v5 + 0x19) + '\x01';
            if (*(char *)((long)v5 + 0x19)) goto label_d1d3;
          }
        }
        else {
          v5 = v11[v18];
          *(char *)((long)v5 + 0x19) = *(char *)((long)v5 + 0x19) + '\xff';
          if (!*(char *)((long)v5 + 0x19)) goto label_d19a;
label_d1d3:
          if (*(char *)((long)v11[v18] + 0x19) != '\xfe') { // branch-flip
            if (*(char *)((long)v11[v18] + 0x19) != '\x02')
              abort(); // no-return
            if (v12[v18 + 1]) { // branch-flip
              if (v12[v18 + 1] != 1)
                abort(); // no-return
              v5 = v11[v18];
              v19 = (long *)v5[1];
              v4 = *v19;
              *v19 = (long)v5;
              v5[1] = v4;
              *(char *)((long)v5 + 0x19) = 0;
              *(char *)((long)v19 + 0x19) = *(char *)((long)v5 + 0x19);
            }
            else {
              v5 = v11[v18];
              v7 = (long *)v5[1];
              v19 = (long *)*v7;
              v4 = *v19;
              v8 = v19[1];
              *v19 = (long)v5;
              v5[1] = v4;
              v19[1] = (long)v7;
              *v7 = v8;
              v10 = (*(char *)((long)v19 + 0x19) != '\x01') ? 0 : 0xff; // branch-flip
              *(char *)((long)v5 + 0x19) = v10;
              *(bool *)((long)v7 + 0x19) = *(char *)((long)v19 + 0x19) == '\xff';
              *(char *)((long)v19 + 0x19) = 0;
            }
          }
          else if (v12[v18 + 1]) { // branch-flip
            if (v12[v18 + 1] != 1)
              abort(); // no-return
            v5 = v11[v18];
            v4 = *v5;
            v19 = *(long **)(v4 + 8);
            v8 = v19[1];
            v6 = *v19;
            *v19 = v4;
            *(long *)(v4 + 8) = v6;
            v19[1] = (long)v5;
            *v5 = v8;
            v10 = (*(char *)((long)v19 + 0x19) != '\x01') ? 0 : 0xff; // branch-flip
            *(char *)(v4 + 0x19) = v10;
            *(bool *)((long)v5 + 0x19) = *(char *)((long)v19 + 0x19) == '\xff';
            *(char *)((long)v19 + 0x19) = 0;
          }
          else {
            v5 = v11[v18];
            v19 = (long *)*v5;
            v4 = v19[1];
            v19[1] = (long)v5;
            *v5 = v4;
            *(char *)((long)v5 + 0x19) = 0;
            *(char *)((long)v19 + 0x19) = *(char *)((long)v5 + 0x19);
          }
          if (v12[v18 + -1]) // branch-flip
            v11[v18 + -1][1] = (long)v19;
          else {
            *v11[v18 + -1] = (long)v19;
          }
        }
      }
    }
    v16 = (long *)v17[2];
    v14 = v2;
  } while( true );
}


// Function: sub_d633 @ 0xd633
unsigned long sub_d633(long a0)
{
  return *(unsigned long *)(a0 + 0x58);
}


// Function: sub_d649 @ 0xd649
void sub_d649(unsigned long *a0,long *a1) // return-dupe
{
  long v1;
  
  if (!a0)
    return;
  sub_d649(*a0,a1);
  sub_d649(a0[1],a1);
  v1 = *a1;
  *(unsigned long *)(v1 + 0x18) = a0[2];
  *a1 = *(long *)(v1 + 0x18);
}


// Function: sub_d6b4 @ 0xd6b4
void sub_d6b4(struct_21 *a0,long a1,unsigned long a2,char a3) // return-dupe, ternary
{
  long v1; // stack - 0x28
  unsigned long *v2; // stack - 0x10
  
  if (!a0)
    return;
  sub_d6b4(a0->field_0x0,a1,a2,(unsigned char)a3);
  sub_d6b4(a0->field_0x8,a1,a2,(unsigned char)a3);
  v1 = a1;
  while( true ) {
    if (!v1) {
      *(unsigned long *)(a0->field_0x10 + 0x20) = a2;
      return;
    }
    v2 = *(unsigned long **)(v1 + 8);
    while ((v2 && (a0->field_0x18 != *(char *)&v2[3]))) {
      v2 = (*(unsigned char *)&v2[3] <= (unsigned char)a0->field_0x18) ? (unsigned long *)v2[1] : (unsigned long *)*v2; // branch-flip
    }
    if (v2) break;
    v1 = *(long *)(v1 + 0x20);
  }
  *(unsigned long *)(a0->field_0x10 + 0x20) = v2[2];
  if (a3 == '\x01')
    return;
  if (!*(long *)v2[2])
    return;
  if (*(long *)a0->field_0x10)
    return;
  *(unsigned long *)a0->field_0x10 = 0xffffffffffffffff;
}


// Function: sub_d7f4 @ 0xd7f4
void sub_d7f4(struct_23 *a0,long a1,long a2) // return-dupe x2
{
  if (!a0)
    return;
  sub_d7f4(a0->field_0x0,a1,a2);
  sub_d7f4(a0->field_0x8,a1,a2);
  if ((long)(unsigned long)*(unsigned char *)(a2 + (unsigned long)a0->field_0x18) <= a1)
    return;
  *(char *)(a2 + (unsigned long)a0->field_0x18) = (char)a1;
}


// Function: sub_d883 @ 0xd883
unsigned long sub_d883(unsigned long *a0,struct_21 *a1) // early-return x3
{
  unsigned long v1;
  unsigned int v2; // eax
  unsigned long *v3;
  unsigned long *v4; // stack - 0x10
  
  if (!a1)
    return 1;
  v1 = a1->field_0x0;
  if (sub_d883(a0,v1) != '\x01')
    return 0;
  v1 = a1->field_0x8;
  v2 = sub_d883(a0,v1) ^ 1;
  v3 = (unsigned long *)(unsigned long)v2;
  v4 = a0;
  if ((char)v2)
    return 0;
  while ((v4 && (v3 = NULL, a1->field_0x18 != *(char *)&v4[3]))) {
    if (*(unsigned char *)&v4[3] <= (unsigned char)a1->field_0x18) { // branch-flip
      v3 = (unsigned long *)v4[1];
      v4 = v3;
    }
    else {
      v3 = (unsigned long *)*v4;
      v4 = v3;
    }
  }
  return CONCAT71((undefined7)((unsigned long)v3 >> 8),v4 != NULL);
}


// Function: sub_d943 @ 0xd943
void sub_d943(struct_23 *a0,long a1) // return-dupe
{
  if (!a0)
    return;
  sub_d943(a0->field_0x0,a1);
  sub_d943(a0->field_0x8,a1);
  *(unsigned long *)((unsigned long)a0->field_0x18 * 8 + a1) = a0->field_0x10;
}


// Function: sub_d9b5 @ 0xd9b5
void sub_d9b5(unsigned long *a0) // return-dupe
{
  unsigned long *v1;
  unsigned long *v10;
  unsigned long *v11;
  bool v12; // zf
  unsigned int v13; // stack - 0xa1c
  unsigned int v14; // stack - 0xa18
  unsigned int v15; // stack - 0xa14
  unsigned int v16; // stack - 0xa10
  int v17; // stack - 0xa0c
  long *v18; // stack - 0xa00
  long v19; // stack - 0x9f8
  unsigned long *v2;
  long v20; // stack - 0x9f0
  long v21; // stack - 0x9e8
  long v22; // stack - 0x9e0
  long v23; // stack - 0x9d8
  unsigned long *v24; // stack - 0x9d0
  unsigned long *v25; // stack - 0x9c8
  unsigned long *v26; // stack - 0x9c0
  unsigned long v27; // stack - 0x9b8
  unsigned long *v28; // stack - 0x9b0
  unsigned long *v29; // stack - 0x9a8
  char v3;
  long v30; // stack - 0x9a0
  unsigned long *v31; // stack - 0x998
  unsigned long *v32; // stack - 0x990
  unsigned long v33; // stack - 0x988
  unsigned long *v34; // stack - 0x980
  unsigned long *v35; // stack - 0x978
  unsigned long *v36; // stack - 0x970
  unsigned long v37; // stack - 0x968
  unsigned long *v38; // stack - 0x960
  unsigned long *v39; // stack - 0x958
  struct_21 *v4;
  long v40; // stack - 0x950
  unsigned long *v41; // stack - 0x948
  unsigned long *v42; // stack - 0x940
  unsigned long v43; // stack - 0x938
  unsigned long *v44; // stack - 0x930
  unsigned long *v45; // stack - 0x928
  long v46; // stack - 0x920
  char v5; // al
  long v6;
  unsigned long v7 [33];
  long *v8; // stack - 0xa08
  unsigned long v9 [256];
  
  v23 = a0[0x130];
  if (v23) // branch-flip
    v24 = v7;
  else {
    v24 = &a0[0xe];
  }
  v12 = a0[0xb] == 1;
  if (v12) {
    v8 = (long *)a0[0xc];
    for (v18 = v8; v18; v18 = (long *)v18[3]) {
      sub_d649((unsigned long *)v18[1],&v8);
    }
    v27 = a0[0xd];
    v25 = a0;
    v26 = a0;
    v28 = a0;
    if ((unsigned long)(a0[4] - a0[3]) < v27)
      _obstack_newchunk((obstack *)a0,v27);
    v26[3] = v26[3] + v27;
    v29 = v25;
    v30 = v25[2];
    if (v30 == v25[3])
      *(unsigned char *)&v25[10] = *(unsigned char *)&v25[10] | 2;
    v25[3] = ~v25[6] & v25[3] + v25[6];
    if ((unsigned long)(v25[4] - v25[1]) < (unsigned long)(v25[3] - v25[1]))
      v25[3] = v25[4];
    v25[2] = v25[3];
    a0[0x12e] = v30;
    v18 = (long)a0[0xc];
    for (v19 = 0; v19 < (long)a0[0xd]; v19 = v19 + 1) {
      *(char *)(v19 + a0[0x12e]) = *(char *)(*(long *)((long)v18 + 8) + 0x18);
      v18 = *(long **)((long)v18 + 0x18);
    }
    v31 = (unsigned long *)sub_c89b(a0[0x130]);
    v31[0x133] = sub_f061;
    sub_cacb(v31,(unsigned char *)a0[0x12e],a0[0xd]);
    v33 = 0;
    v32 = a0;
    _obstack_free((obstack *)a0,NULL);
    v6 = 0x134;
    v10 = v31;
    v11 = a0;
    while (v6) {
      v2 = &v11[1];
      v1 = &v10[1];
      *v11 = *v10;
      v6 -= 1;
      v10 = v1;
      v11 = v2;
    }
    free(v31);
  }
  v6 = a0[0xd];
  if (0x100 <= v6)
    v6 = 0xff;
  memset(v24,(int)v6,0x100);
  v8 = (long *)a0[0xc];
  for (v18 = v8; v18; v18 = (long *)v18[3]) {
    sub_d649((unsigned long *)v18[1],&v8);
    sub_d7f4((struct_23 *)v18[1],v18[5],v24);
    sub_d6b4((struct_21 *)v18[1],v18[4],a0[0xc],v12);
    if (v12) {
      v18[6] = a0[0xd];
      v18[7] = a0[0xd];
      for (v20 = v18[4]; v20; v20 = *(long *)(v20 + 0x20)) {
        v4 = (struct_21 *)v18[1];
        v10 = *(unsigned long **)(v20 + 8);
        if ((sub_d883(v10,v4) != '\x01') && (v18[5] - *(long *)(v20 + 0x28) < *(long *)(v20 + 0x30)))
          *(long *)(v20 + 0x30) = v18[5] - *(long *)(v20 + 0x28);
        if ((*v18) && (v18[5] - *(long *)(v20 + 0x28) < *(long *)(v20 + 0x38)))
          *(long *)(v20 + 0x38) = v18[5] - *(long *)(v20 + 0x28);
      }
    }
  }
  if (v12) {
    for (v18 = *(long **)(a0[0xc] + 0x18); v18; v18 = (long *)v18[3]) {
      if (*(long *)(v18[2] + 0x38) < v18[7])
        v18[7] = *(long *)(v18[2] + 0x38);
      if (v18[7] < v18[6])
        v18[6] = v18[7];
    }
  }
  if (v23) // branch-flip
    v34 = v9;
  else {
    v34 = &a0[0x2e];
  }
  memset(v34,0,0x800);
  sub_d943(*(struct_23 **)(a0[0xc] + 8),v34);
  v13 = 0xfffffffe;
  v14 = 0xffffffff;
  for (v15 = 0; (int)v15 <= 0xff; v15 = v15 + 1) {
    v16 = v15;
    if (v23) {
      v3 = *(char *)(v23 + (int)v15);
      v16 = (unsigned int)(unsigned char)sub_c845((int)v3);
      a0[(long)(int)v15 + 0x2e] = v34[(int)v16];
    }
    if (a0[(long)(int)v15 + 0x2e]) {
      if (-1 <= (int)v13) { // branch-flip
        if (v13 != v16) { // branch-flip
          if ((v15 != v16) || (v13 != v14))
            v13 = 0xffffffff;
          else {
            v14 = v15;
          }
        }
        else if (v14 != v16) // branch-flip
          v14 = 0xffffffff;
        else {
          v14 = v15;
        }
      }
      else {
        v13 = v16;
        v14 = v15;
      }
    }
  }
  *(unsigned int *)&a0[0x131] = v13;
  *(unsigned int *)((long)a0 + 0x98c) = v14;
  if (v12) {
    v37 = a0[0xd];
    v35 = a0;
    v36 = a0;
    v38 = a0;
    if ((unsigned long)(a0[4] - a0[3]) < v37)
      _obstack_newchunk((obstack *)a0,v37);
    v36[3] = v36[3] + v37;
    v39 = v35;
    v40 = v35[2];
    if (v40 == v35[3])
      *(unsigned char *)&v35[10] = *(unsigned char *)&v35[10] | 2;
    v35[3] = ~v35[6] & v35[3] + v35[6];
    if ((unsigned long)(v35[4] - v35[1]) < (unsigned long)(v35[3] - v35[1]))
      v35[3] = v35[4];
    v35[2] = v35[3];
    a0[0x12e] = v40;
    v18 = (long *)a0[0xc];
    for (v21 = a0[0xd]; 0 < v21; v21 = v21 + -1) {
      *(char *)(v21 + -1 + a0[0x12e]) = *(char *)(v18[1] + 0x18);
      v18 = (long *)v18[3];
    }
    if (2 <= (long)a0[0xd]) {
      v43 = (a0[0xd] + -1) * 8;
      v41 = a0;
      v42 = a0;
      v44 = a0;
      if ((unsigned long)(a0[4] - a0[3]) < v43)
        _obstack_newchunk((obstack *)a0,v43);
      v42[3] = v42[3] + v43;
      v45 = v41;
      v46 = v41[2];
      if (v46 == v41[3])
        *(unsigned char *)&v41[10] = *(unsigned char *)&v41[10] | 2;
      v41[3] = ~v41[6] & v41[3] + v41[6];
      if ((unsigned long)(v41[4] - v41[1]) < (unsigned long)(v41[3] - v41[1]))
        v41[3] = v41[4];
      v41[2] = v41[3];
      a0[0x12f] = v46;
      v18 = *(long **)(a0[0xc] + 0x18);
      for (v22 = 0; v22 < a0[0xd] + -1; v22 = v22 + 1) {
        *(long *)(a0[0x12f] + v22 * 8) = v18[6];
        v18 = (long *)v18[3];
      }
      v5 = sub_c863(v23,(int)*(char *)(a0[0xd] + -2 + a0[0x12e]));
      *(char *)&a0[0x132] = v5;
    }
  }
  if (v23) {
    for (v17 = 0; v17 <= 0xff; v17 = v17 + 1) {
      v3 = *(char *)(v23 + v17);
      *(char *)((long)a0 + (long)v17 + 0x70) = *(char *)((long)v24 + (unsigned long)(unsigned char)sub_c845((int)v3));
    }
    return;
  }
}


// Function: sub_e78b @ 0xe78b
unsigned long sub_e78b(unsigned long *a0,unsigned long a1,long a2,long a3,long a4,char a5,unsigned int a6,long a7,long a8) // return-dupe
{
  char v1;
  char v2;
  char v3; // al
  unsigned long v4; // stack - 0x38
  long v5; // stack - 0x30
  long v6; // stack - 0x28
  long v7; // stack - 0x20
  
  v2 = (char)a6;
  v4 = *a0;
  v6 = 0;
  v5 = a3;
  do {
    v7 = 2;
    v1 = *(char *)(v4 - 2);
    if (v2 == sub_c863(a4,(int)v1)) {
      do {
        v7 += 1;
        if (v5 < v7) break;
        v3 = sub_c863(a4,(int)*(char *)(v4 - v7));
        v1 = *(char *)(a2 - v7);
      } while (v3 == sub_c863(a4,(int)v1));
      if (v5 < v7) {
        v7 = v6 + v5;
        do {
          v7 += 1;
          if (a3 < v7) break;
          v3 = sub_c863(a4,(int)*(char *)(v4 - v7));
          v1 = *(char *)(a2 - v7);
        } while (v3 == sub_c863(a4,(int)v1));
        if (a3 < v7) {
          *a0 = v4 - a3;
          return 1;
        }
      }
    }
    v5 = *(long *)(v7 * 8 + -0x10 + *(long *)(a8 + 0x978));
    v4 += v5;
    if (a1 < v4) goto label_e9ac;
    v1 = *(char *)(v4 - 1);
    if (a5 != sub_c863(a4,(int)v1)) {
      if (a7) {
        v2 = *(char *)(v4 - 1);
        v4 += *(unsigned char *)(a7 + (unsigned long)(unsigned char)sub_c845((int)v2));
      }
label_e9ac:
      *a0 = v4;
      return 0;
    }
    v6 = v7 + -1;
  } while( true );
}


// Function: sub_e9c2 @ 0xe9c2
char * sub_e9c2(char *a0,long a1,long a2) // early-return
{
  char v1;
  unsigned int v2;
  unsigned int v3;
  char *v4; // rax
  char *v5; // stack - 0x30
  
  v4 = &a0[a1];
  v5 = a0;
  if (0 <= *(int *)(a2 + 0x98c)) {
    while( true ) {
      if (v4 <= v5)
        return NULL;
      v1 = *v5;
      if (*(long *)(a2 + ((long)(int)(unsigned int)(unsigned char)sub_c845((int)v1) + 0x2e) * 8)) break;
      v5 = &v5[1];
      if (!((unsigned long)v5 % 0x10)) {
        v2 = *(unsigned int *)(a2 + 0x98c);
        v3 = *(unsigned int *)(a2 + 0x988);
        return (char *)sub_222e7(v5,v3,v2,(long)v4 - (long)v5);
      }
    }
    return v5;
  }
  while( true ) {
    if (v4 <= v5)
      return NULL;
    v1 = *v5;
    if (*(long *)(a2 + ((long)(int)(unsigned int)(unsigned char)sub_c845((int)v1) + 0x2e) * 8)) break;
    v5 = &v5[1];
  }
  return v5;
}


// Function: sub_eae1 @ 0xeae1
long sub_eae1(long a0,long a1,long a2) // early-return x3
{
  char v1;
  long v10; // stack - 0x38
  long v11; // stack - 0x30
  long v12; // stack - 0x28
  unsigned long v13; // stack - 0x20
  unsigned long v14; // stack - 0x18
  char v2; // al
  char v3;
  int v4; // eax
  long v5; // rax
  unsigned long v6; // stack - 0x50
  unsigned int v7; // stack - 0x58
  long v8; // stack - 0x48
  long v9; // stack - 0x40
  
  v9 = *(long *)(a0 + 0x68);
  v10 = *(long *)(a0 + 0x980);
  if (!v9)
    return 0;
  if (a2 < v9)
    return -1;
  if (v9 != 1) { // branch-flip
    v11 = a0 + 0x70;
    v12 = v9 + *(long *)(a0 + 0x970);
    v6 = a1 + v9;
    v2 = (char)*(unsigned int *)(a0 + 0x988);
    v1 = *(char *)(a0 + 0x990);
    v8 = v9 * 0xc;
    if ((SEXT816(v8) != SEXT816(v9) * SEXT816(0xc)) || (a2 <= v8)) {
label_ef6e:
      v13 = a1 + a2;
      v3 = *(char *)(v6 - 1);
      v7 = (unsigned int)*(unsigned char *)(v11 + (unsigned long)(unsigned char)sub_c845((int)v3));
      do {
        if ((long)(v13 - v6) < (long)(int)v7)
          return -1;
        v6 = (long)(int)v7 + v6;
        v3 = *(char *)(v6 - 1);
        v7 = (unsigned int)*(unsigned char *)(v11 + (unsigned long)(unsigned char)sub_c845((int)v3));
      } while ((v7) || (v3 = sub_e78b(&v6,v13,v12,v9,v10,(int)v2,(int)v1,0,a0), !v3));
      v5 = v6 - a1;
    }
    else {
      v13 = a1 + a2 + v9 * -0xb;
label_ef5d:
      do {
        if (v13 < v6) goto label_ef6e;
        v14 = v6;
        v3 = *(char *)(v6 - 1);
        v6 = (long)(int)(unsigned int)*(unsigned char *)(v11 + (unsigned long)(unsigned char)sub_c845((int)v3)) + v6;
        v3 = *(char *)(v6 - 1);
        v4 = (unsigned int)*(unsigned char *)(v11 + (unsigned long)(unsigned char)sub_c845((int)v3));
        v6 = (long)v4 + v6;
        if (v4) {
          v3 = *(char *)(v6 - 1);
          v6 = (long)(int)(unsigned int)*(unsigned char *)(v11 + (unsigned long)(unsigned char)sub_c845((int)v3)) + v6;
          v3 = *(char *)(v6 - 1);
          v6 = (long)(int)(unsigned int)*(unsigned char *)(v11 + (unsigned long)(unsigned char)sub_c845((int)v3)) + v6;
          v3 = *(char *)(v6 - 1);
          v4 = (unsigned int)*(unsigned char *)(v11 + (unsigned long)(unsigned char)sub_c845((int)v3));
          v6 = (long)v4 + v6;
          if (v4) {
            v3 = *(char *)(v6 - 1);
            v6 = (long)(int)(unsigned int)*(unsigned char *)(v11 + (unsigned long)(unsigned char)sub_c845((int)v3)) + v6;
            v3 = *(char *)(v6 - 1);
            v6 = (long)(int)(unsigned int)*(unsigned char *)(v11 + (unsigned long)(unsigned char)sub_c845((int)v3)) + v6;
            v3 = *(char *)(v6 - 1);
            v4 = (unsigned int)*(unsigned char *)(v11 + (unsigned long)(unsigned char)sub_c845((int)v3));
            v6 = (long)v4 + v6;
            if (v4) {
              v3 = *(char *)(v6 - 1);
              v6 = (long)(int)(unsigned int)*(unsigned char *)(v11 + (unsigned long)(unsigned char)sub_c845((int)v3)) + v6;
              v3 = *(char *)(v6 - 1);
              v6 = (long)(int)(unsigned int)*(unsigned char *)(v11 + (unsigned long)(unsigned char)sub_c845((int)v3)) + v6;
              if (0x80 <= (long)(v6 - v14)) goto label_ef5d;
              v6 -= 1;
              v5 = sub_e9c2(v6,(a1 + a2) - v6,a0);
              if (!v5)
                return -1;
              v6 = v5 + 1;
              if (v13 <= v6) goto label_ef6e;
            }
          }
        }
        v3 = sub_e78b(&v6,v13,v12,v9,v10,(int)v2,(int)v1,v11,a0);
      } while (!v3);
      v5 = v6 - a1;
    }
  }
  else {
    v5 = sub_e9c2(a1,a2,a0);
    if (!v5)
      return -1;
    v5 -= a1;
  }
  return v5;
}


// Function: sub_f061 @ 0xf061
unsigned long sub_f061(long a0,long a1,long a2,unsigned long *a3)
{
  unsigned long v1; // rax
  
  if (*(long *)(a0 + 0x980)) // branch-flip
    v1 = sub_eae1(a0,a1,a2);
  else {
    v1 = sub_eae1(a0,a1,a2);
  }
  *a3 = 0;
  a3[1] = v1;
  a3[2] = *(unsigned long *)(a0 + 0x68);
  return v1;
}


// Function: sub_f0f4 @ 0xf0f4
long sub_f0f4(long a0,char *a1,long a2,long *a3,bool a4) // early-return x5, ternary x2
{
  char v1;
  unsigned long v10; // stack - 0x38
  unsigned long *v11; // stack - 0x30
  long *v12; // stack - 0x28
  int v2;
  long v3;
  unsigned char v4; // al
  char *v5; // rax
  char *v6;
  long *v7; // stack - 0x50
  long *v8; // stack - 0x48
  char *v9; // stack - 0x40
  
  if (a2 < *(long *)(a0 + 0x68))
    return -1;
  v3 = *(long *)(a0 + 0x980);
  v7 = *(long **)(a0 + 0x60);
  v5 = &a1[a2];
  v9 = a1;
  if (!*v7) {
    v2 = *(int *)(a0 + 0x988);
    v9 = a1;
label_f17b:
    if (0 <= v2) { // branch-flip
      v6 = (char *)sub_e9c2(v9,(long)v5 - (long)v9,a0);
      if (!v6)
        return -1;
      v1 = *v6;
      v7 = *(long **)(a0 + ((long)(int)(unsigned int)(unsigned char)sub_c863(v3,(int)v1) + 0x2e) * 8);
      v9 = &v6[1];
    }
    else {
      while( true ) {
        v6 = &v9[1];
        v1 = *v9;
        v7 = *(long **)(a0 + ((long)(int)(unsigned int)(unsigned char)sub_c863(v3,(int)v1) + 0x2e) * 8);
        v9 = v6;
        if (v7) break;
        if (v5 <= v6)
          return -1;
      }
    }
label_f24b:
    do {
      if (*v7) break;
      if (v5 <= v9)
        return -1;
      v6 = &v9[1];
      v4 = sub_c863(v3,(int)*v9);
      v11 = (unsigned long *)v7[1];
      while (v4 != *(unsigned char *)&v11[3]) {
        v11 = (*(unsigned char *)&v11[3] <= v4) ? (unsigned long *)v11[1] : (unsigned long *)*v11; // branch-flip
        if (!v11) {
          v7 = (long *)v7[4];
          if (!v7) {
            v7 = *(long **)(a0 + ((long)(int)(unsigned int)v4 + 0x2e) * 8);
            v9 = v6;
            if (v7) goto label_f24b;
            if (v5 <= v6)
              return -1;
            goto label_f17b;
          }
          if (*v7) goto label_f366;
          v11 = (unsigned long *)v7[1];
        }
      }
      v7 = (long *)v11[2];
      v9 = v6;
    } while( true );
  }
label_f366:
  for (v8 = v7; *v8 < 0; v8 = (long *)v8[4]) {
  }
  v10 = (long)v9 - v8[5];
  if (a4) {
    while (v9 < v5) {
      v6 = &v9[1];
      v4 = sub_c863(v3,(int)*v9);
      do {
        v11 = (unsigned long *)v7[1];
        while ((v11 && (v4 != *(unsigned char *)&v11[3]))) {
          v11 = (*(unsigned char *)&v11[3] <= v4) ? (unsigned long *)v11[1] : (unsigned long *)*v11; // branch-flip
        }
      } while (((!v11) && (v7 = (long *)v7[4], v7)) && (v8[5] <= v7[5]));
      if (!v11) break;
      v7 = (long *)v11[2];
      v9 = v6;
      if (*v7) {
        for (v12 = v7; *v12 < 0; v12 = (long *)v12[4]) {
        }
        if ((unsigned long)((long)v6 - v12[5]) <= v10) {
          v8 = v12;
          v10 = (long)v6 - v12[5];
        }
      }
    }
  }
  *a3 = *v8 >> 1;
  a3[1] = v10 - (long)a1;
  a3[2] = v8[5];
  return v10 - (long)a1;
}


// Function: sub_f50c @ 0xf50c
void sub_f50c(long a0,char *a1,long a2,long *a3,char a4) // return-dupe
{
  if (*(long *)(a0 + 0x980)) {
    sub_f0f4(a0,a1,a2,a3,(unsigned char)a4);
    return;
  }
  sub_f0f4(a0,a1,a2,a3,(unsigned char)a4);
}


// Function: sub_f585 @ 0xf585
void sub_f585(long a0,unsigned long a1,unsigned long a2,unsigned long a3,char a4)
{
  (**(void **)(a0 + 0x998))(a0,a1,a2,a3,(unsigned char)a4);
}


// Function: sub_f5d1 @ 0xf5d1
void sub_f5d1(obstack *a0)
{
  _obstack_free(a0,NULL);
  free(a0);
}


// Function: sub_f73c @ 0xf73c
unsigned int sub_f73c(unsigned int a0)
{
  if ((a0 != 0x5f) && (!iswalnum(a0)))
    return 0;
  return 1;
}


// Function: sub_f770 @ 0xf770
void sub_f770(void)
{
  unsigned int v1;
  int v2; // stack - 0xc
  
  for (v2 = 0; v2 <= 0xff; v2 = v2 + 1) {
    v1 = *(unsigned int *)(((long)v2 + 0x40) * 4 + 0x331a4);
    *(char *)((long)v2 + 0x339e0) = sub_f73c(v1);
  }
}


// Function: sub_f7c8 @ 0xf7c8
void sub_f7c8(bool a0)
{
  int v1; // stack - 0x14
  long v2; // stack - 0x10
  
  v2 = 0;
  if (dat_33180) {
    if ((__ctype_get_mb_cur_max() == 1) || (a0)) {
      v2 = sub_26b10(0x100);
      for (v1 = 0; v1 <= 0xff; v1 = v1 + 1) {
        *(char *)(v2 + v1) = (char)toupper(v1);
      }
    }
  }
  sub_c89b(v2);
}


// Function: sub_f84a @ 0xf84a
long sub_f84a(unsigned long *a0,long *a1,char *a2,long a3) // early-return
{
  long v1; // rax
  unsigned long v2; // stack - 0x18
  int v3; // stack - 0x3c
  char *v4; // stack - 0x38
  char *v5; // stack - 0x30
  long v6; // stack - 0x28
  
  v5 = (char *)*a0;
  if (v5 < a2) {
    if (dat_331a2) { // branch-flip
      v4 = a2;
      if (((int)*a2 & 0xc0U) == 0x80) {
        for (v3 = 1; v3 <= 3; v3 = v3 + 1) {
          if (((int)a2[-v3] & 0xc0U) != 0x80) {
            if (!((int)(~(int)a2[-v3] & 0xffU) >> (7U - (char)v3 & 0x1fU))) {
              v2 = 0;
              v1 = sub_f686(&a2[-(long)v3],a3 - ((long)a2 - (long)v3),&v2);
              if (0 <= v1) {
                v5 = &a2[-(long)v3];
                v4 = &v5[v1];
              }
            }
            break;
          }
        }
      }
    }
    else {
      v2 = 0;
      v4 = v5;
      do {
        v6 = sub_f6da(v4,a3 - (long)v4,&v2);
        if (v6 <= -1) {
          v6 = 1;
          memset(&v2,0,8);
        }
        v5 = v4;
        v4 = &v4[v6];
      } while (v4 < a2);
      if (a1)
        *a1 = v6;
    }
    *a0 = (unsigned long)v4;
    if (v4 == a2)
      return 0;
  }
  return (long)a2 - (long)v5;
}


// Function: sub_fa82 @ 0xfa82
long sub_fa82(long a0,long a1,char a2)
{
  char *v1;
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x18
  unsigned int v4; // stack - 0x2c
  long v5; // stack - 0x28
  long v6; // stack - 0x20
  
  v5 = 0;
  v3 = 0;
  do {
    if (a1 - a0 <= v5)
      return v5;
    if (*(char *)((long)(int)(unsigned int)*(unsigned char *)(a0 + v5) + 0x339e0)) // branch-flip
      v5 += 1;
    else {
      if (*(char *)((long)(int)(unsigned int)*(unsigned char *)(a0 + v5) + 0x331a3) != '\xfe')
        return v5;
      v4 = 0;
      v2 = (a1 - a0) - v5;
      v1 = (char *)(v5 + a0);
      v6 = sub_21b80(&v4,v1,v2,&v3);
      if (sub_f73c(v4) != '\x01')
        return v5;
      v5 += (unsigned long)(v6 == 0) + v6;
    }
    if (a2 != '\x01')
      return v5;
  } while( true );
}


// Function: sub_fbb2 @ 0xfbb2
void sub_fbb2(long a0,long a1)
{
  sub_fa82(a0,a1,1);
}


// Function: sub_fbe0 @ 0xfbe0
void sub_fbe0(long a0,long a1)
{
  sub_fa82(a0,a1,0);
}


// Function: sub_fc0e @ 0xfc0e
unsigned long sub_fc0e(long a0,long a1,long a2) // early-return
{
  unsigned char *v1;
  unsigned char v2; // al
  unsigned long v3; // rax
  long v4; // stack - 0x18
  
  if (a0 == a1) {
    v3 = 0;
    return v3;
  }
  v1 = (unsigned char *)(a1 + -1);
  if ((dat_331a0 != '\x01') || (v2 = *v1 >> 7, v2 = ~v2, v2 = v2 & dat_331a2, v2)) {
    v3 = (long)(int)(unsigned int)*v1;
    v3 = (unsigned long)*(unsigned char *)(v3 + 0x339e0);
  }
  else {
    v4 = a0;
    v3 = sub_f84a(&v4,NULL,v1,a2);
    v3 = sub_fbe0((long)v1 - v3,a2);
  }
  return v3;
}


// Function: sub_fd0f @ 0xfd0f
void sub_fd0f(unsigned long a0)
{
  if ((long)a0 <= -1)
    sub_27122(); // no-return
  sub_26b10(a0);
}


// Function: sub_fd3f @ 0xfd3f
void sub_fd3f(void *a0)
{
  free(a0);
}


// Function: sub_fd62 @ 0xfd62
int sub_fd62(unsigned long *a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned int a4)
{
  unsigned long v1;
  int v2;
  int v3 [3]; // stack - 0x34
  long v4; // stack - 0x28
  long v5; // stack - 0x20
  unsigned long v6; // stack - 0x18
  
  while( true ) {
    while( true ) {
      v3[1] = 0x2000;
      v4 = 0x7fffffffffffffff;
      v3[2] = pcre2_match_8(a0[1],a1,a2,a3,a4,a0[3],a0[2]);
      if ((v3[2] != -0x2e) || (v4 / 2 < (long)a0[5])) break;
      v5 = a0[5];
      a0[5] = v5 * 2;
      v6 = a0[5];
      pcre2_jit_stack_free_8(a0[4]);
      v1 = *a0;
      a0[4] = pcre2_jit_stack_create_8(v5,v6,v1);
      if (!a0[4])
        sub_27122(); // no-return
      if (!a0[2]) {
        v1 = *a0;
        a0[2] = pcre2_match_context_create_8(v1);
      }
      pcre2_jit_stack_assign_8(a0[2],0,a0[4]);
    }
    if (v3[2] != -0x35)
      return v3[2];
    pcre2_config_8(7,v3);
    v2 = v3[0] * 2;
    if (v3[0] <= -1) break;
    v3[0] = v2;
    if (!a0[2]) {
      v1 = *a0;
      a0[2] = pcre2_match_context_create_8(v1);
    }
    pcre2_set_depth_limit_8(a0[2],v3[0]);
  }
  return v3[2];
}


// Function: sub_ff6e @ 0xff6e
unsigned long sub_ff6e(void)
{
  return 0;
}


// Function: sub_ff80 @ 0xff80
unsigned long * sub_ff80(void *a0,unsigned long a1) // ternary
{
  int v1; // eax
  char v10 [264];
  unsigned long v11; // stack - 0x188
  void *v12; // stack - 0x180
  unsigned int v13; // stack - 0x174
  void *v14; // stack - 0x168
  void *v15; // stack - 0x160
  unsigned long *v16; // stack - 0x158
  unsigned long v17; // stack - 0x150
  unsigned long v18; // stack - 0x148
  unsigned long v19; // stack - 0x140
  unsigned int v2; // eax
  void *v20; // stack - 0x138
  void *v21; // stack - 0x130
  unsigned long *v3; // rax
  unsigned long *v4; // rax
  unsigned long v5; // rax
  unsigned long *v6; // rax
  unsigned long *v7; // rax
  char v8 [8];
  int v9; // stack - 0x178
  
  v13 = (dat_33180) ? 0x18 : 0x10; // branch-flip
  v15 = (void *)((long)a0 + a1);
  v16 = (unsigned long *)sub_26b10(0x38);
  *v16 = pcre2_general_context_create_8(sub_fd0f,sub_fd3f,0);
  v17 = *v16;
  v18 = pcre2_compile_context_create_8(v17);
  if (dat_331a0) {
    if (dat_331a2 != '\x01') {
      error(2,0,gettext("-P supports only unibyte and UTF-8 locales"));
      return v3;
    }
    v13 |= 0x4080000;
  }
  if (v15 == rawmemchr(a0,10)) {
    v14 = NULL;
    v11 = a1;
    v12 = a0;
    if (dat_33182) // branch-flip
      pcre2_set_compile_extra_options_8(v18,8);
    else if (dat_33181) {
      v19 = a1 + 0x11;
      v14 = (void *)sub_26b10(v19);
      v20 = v14;
      v21 = mempcpy(v14,"(?<!\\w)(?:)(?!\\w)invalid argument %s for %s",10);
      v21 = mempcpy(v21,a0,a1);
      memcpy(v21,")(?!\\w)invalid argument %s for %s",7);
      v12 = v20;
      v11 = v19;
    }
    v5 = pcre2_maketables_8(v17);
    pcre2_set_character_tables_8(v18,v5);
    v5 = pcre2_compile_8(v12,v11,v13,&v9,v8,v18);
    v16[1] = v5;
    if (v16[1]) {
      free(v14);
      pcre2_compile_context_free_8(v18);
      v16[2] = 0;
      v5 = pcre2_match_data_create_from_pattern_8(v16[1],v17);
      v16[3] = v5;
      v1 = pcre2_jit_compile_8(v16[1],1);
      v9 = v1;
      if (((v1) && (v1 != -0x2d)) && (v1 != -0x30)) {
        error(2,0,gettext("JIT internal error: %d"),v1);
        return v7;
      }
      v16[4] = 0;
      v16[5] = 0x8000;
      v2 = sub_fd62(v16,0x2a907,0,0,1);
      *(unsigned int *)&v16[6] = v2;
      v2 = sub_fd62(v16,0x2a907,0,0,0);
      *(unsigned int *)((long)v16 + 0x34) = v2;
      return v16;
    }
    pcre2_get_error_message_8(v9,v10,0x100);
    error(2,0,"%s",v10);
    return v6;
  }
  error(2,0,gettext("the -P option only supports a single pattern"));
  return v4;
}


// Function: sub_10409 @ 0x10409
long sub_10409(unsigned long *a0,char *a1,long a2,long *a3,char *a4) // early-return
{
  char v1;
  unsigned int v10; // stack - 0x7c
  char *v11; // stack - 0x78
  char *v12; // stack - 0x70
  char *v13; // stack - 0x68
  char *v14; // stack - 0x60
  char *v15; // stack - 0x58
  long *v2; // rax
  char *v3; // rax
  long v4; // rax
  long v5; // rax
  unsigned long v6; // rax
  long v7; // rax
  bool v8; // stack - 0x81
  int v9; // stack - 0x80
  
  if (a4) // branch-flip
    v11 = a4;
  else {
    v11 = a1;
  }
  v8 = v11[-1] == dat_33183;
  v2 = (long *)pcre2_get_ovector_pointer_8(a0[3]);
  v12 = a1;
  v13 = a1;
  do {
    v3 = rawmemchr(v11,(int)dat_33183);
    while( true ) {
      while (v1 = *v11, *(char *)((long)(int)(unsigned int)(unsigned char)sub_f64d((int)v1) + 0x331a3) == '\xff') {
        v11 = &v11[1];
        v8 = 0;
        v13 = v11;
      }
      v4 = (long)v11 - (long)v13;
      if (v11 == v3) break;
      v10 = (unsigned int)(v8 != 1);
      v9 = sub_fd62(a0,v13,(long)v3 - (long)v13,v4,v10);
      if (sub_ff6e(v9) != '\x01') goto label_10655;
      v5 = pcre2_get_startchar_8(a0[3]);
      if (v4 <= v5) {
        if (v5) // branch-flip
          v9 = sub_fd62(a0,v13,v5,v4,v10 | 0x40000002);
        else {
          *v2 = 0;
          v2[1] = 0;
          v9 = *(int *)((long)a0 + ((long)(int)(unsigned int)v8 + 0xc) * 4);
        }
        if (v9 != -1) goto label_10655;
        v11 = &v13[v5 + 1];
        v8 = 0;
      }
      v13 = &v13[v5 + 1];
    }
    v2[1] = v4;
    *v2 = v2[1];
    v9 = *(int *)((long)a0 + ((long)(int)(unsigned int)v8 + 0xc) * 4);
label_10655:
    if (v9 != -1) break;
    v8 = 1;
    v11 = &v3[1];
    v12 = v11;
    v13 = v11;
  } while (v11 < &a1[a2]);
  if (1 <= v9) {
    v14 = &v13[*v2];
    v15 = &v13[v2[1]];
    if (!a4) {
      v14 = v12;
      v15 = &v3[1];
    }
    *a3 = (long)v15 - (long)v14;
    return (long)v14 - (long)a1;
  }
  if (-0x2d <= v9) { // branch-flip
    if (v9 == -1)
      return -1;
  }
  else if (-0x3f <= v9) {
    switch(v9) {
      case 0xffffffcb:
label_10787:
        v6 = sub_6eac();
        error(2,0,gettext("%s: exceeded PCRE\'s nested backtracking limit"),v6);
        goto label_107b8;
      case 0xffffffcc:
label_107b8:
        v6 = sub_6eac();
        error(2,0,gettext("%s: PCRE detected recurse loop"),v6);
        goto label_107e9;
      case 0xffffffc1:
label_107e9:
        v6 = sub_6eac();
        error(2,0,gettext("%s: exceeded PCRE\'s heap limit"),v6);
        break;
      case 0xffffffd0:
        v6 = sub_6eac();
        error(2,0,gettext("%s: memory exhausted"),v6);
        goto label_10725;
      case 0xffffffd1:
label_10756:
        v6 = sub_6eac();
        error(2,0,gettext("%s: exceeded PCRE\'s backtracking limit"),v6);
        goto label_10787;
      case 0xffffffd2:
label_10725:
        v6 = sub_6eac();
        error(2,0,gettext("%s: exhausted PCRE JIT stack"),v6);
        goto label_10756;
      
    }
  }
  v3 = (char *)sub_6eac();
  error(2,0,gettext("%s: internal PCRE error: %d"),v3,v9);
  return v7;
}

