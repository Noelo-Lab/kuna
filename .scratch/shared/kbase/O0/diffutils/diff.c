// Function: sub_6122 @ 0x6122
void sub_6122(long a0) // ternary x2
{
  long v1;
  void *v10 [3]; // stack - 0x48
  long v11; // r12
  int v12; // stack - 0xe4
  unsigned long v13; // stack - 0xe0
  long v14; // stack - 0xd8
  unsigned long v15; // stack - 0xd0
  unsigned long v16; // stack - 0xc8
  long v17; // stack - 0xc0
  long v18; // stack - 0xb8
  long v19; // stack - 0xb0
  void *v2;
  long v20; // stack - 0xa8
  long v21; // stack - 0x50
  unsigned long v3;
  void *v4;
  long *v5; // rax
  long v6;
  unsigned long v7;
  long v8; // rbx
  void *v9; // stack - 0x58
  
  v1 = *(long *)(a0 + 200);
  v6 = *(long *)(a0 + 0x1f8);
  v14 = sub_1c945((v6 + v1) * 0x10);
  for (v12 = 0; v12 <= 1; v12 = v12 + 1) {
    *(long *)((long)v12 * 0x130 + a0 + 0x100) = v14;
    v14 += *(long *)(a0 + (long)v12 * 0x130 + 200) * 8;
    *(long *)((long)v12 * 0x130 + a0 + 0x108) = v14;
    v14 += *(long *)(a0 + (long)v12 * 0x130 + 200) * 8;
  }
  v10[0] = (void *)sub_13030(*(long *)(a0 + 0x128) << 4);
  v10[1] = (void *)((long)v10[0] + *(long *)(a0 + 0x128) * 8);
  for (v13 = 0; (long)v13 < *(long *)(a0 + 200); v13 = v13 + 1) {
    v5 = (long *)(*(long *)(v13 * 8 + *(long *)(a0 + 0xf8)) * 8 + (long)v10[0]);
    *v5 = *v5 + 1;
  }
  for (v13 = 0; (long)v13 < *(long *)(a0 + 0x1f8); v13 = v13 + 1) {
    v5 = (long *)(*(long *)(v13 * 8 + *(long *)(a0 + 0x228)) * 8 + (long)v10[1]);
    *v5 = *v5 + 1;
  }
  v9 = (void *)sub_13030(*(long *)(a0 + 0x1f8) + *(long *)(a0 + 200));
  v21 = *(long *)(a0 + 200) + (long)v9;
  for (v12 = 0; v12 <= 1; v12 = v12 + 1) {
    v7 = *(unsigned long *)(a0 + (long)v12 * 0x130 + 200);
    v4 = (&v9)[v12];
    v2 = v10[1 - v12];
    v1 = *(long *)(a0 + (long)v12 * 0x130 + 0xf8);
    v15 = 5;
    v16 = v7 >> 6;
    while (v16 = v16 >> 2, v16) {
      v15 <<= 1;
    }
    for (v13 = 0; v13 < v7; v13 = v13 + 1) {
      if (*(long *)(v1 + v13 * 8)) {
        v3 = *(unsigned long *)((long)v2 + *(long *)(v1 + v13 * 8) * 8);
        if (v3) { // branch-flip
          if (v15 < v3)
            *(char *)((long)v4 + v13) = 2;
        }
        else {
          *(char *)((long)v4 + v13) = 1;
        }
      }
    }
  }
  for (v12 = 0; v12 <= 1; v12 = v12 + 1) {
    v1 = *(long *)(a0 + (long)v12 * 0x130 + 200);
    v4 = (&v9)[v12];
    for (v13 = 0; (long)v13 < v1; v13 = v13 + 1) {
      if (*(char *)(v13 + (long)v4) != '\x02') { // branch-flip
        if (*(char *)(v13 + (long)v4)) {
          v17 = 0;
          for (v7 = v13; ((long)v7 < v1 && (*(char *)(v7 + (long)v4))); v7 = v7 + 1) {
            if (*(char *)(v7 + (long)v4) == '\x02')
              v17 += 1;
          }
          while (((long)v13 < (long)v7 && (*(char *)(v7 + -1 + (long)v4) == '\x02'))) {
            v7 -= 1;
            *(char *)(v7 + (long)v4) = 0;
            v17 -= 1;
          }
          v6 = v7 - v13;
          if (v17 * 4 <= v6) { // branch-flip
            v18 = 1;
            v19 = v6 >> 2;
            while (v19 = v19 >> 2, 0 < v19) {
              v18 <<= 1;
            }
            v11 = 0;
            for (v8 = 0; v8 < v6; v8 = v8 + 1) {
              if (*(char *)(v13 + v8 + (long)v4) != '\x02') // branch-flip
                v11 = 0;
              else {
                v11 += 1;
                if (v11 != v18 + 1) { // branch-flip
                  if (v18 + 1 < v11)
                    *(char *)(v13 + v8 + (long)v4) = 0;
                }
                else {
                  v8 -= v11;
                }
              }
            }
            v11 = 0;
            for (v8 = 0; (v8 < v6 && ((v8 <= 7 || (*(char *)(v13 + v8 + (long)v4) != '\x01')))); v8 = v8 + 1) {
              if (*(char *)(v13 + v8 + (long)v4) != '\x02') { // branch-flip
                v11 = (*(char *)(v13 + v8 + (long)v4)) ? v11 + 1 : 0; // branch-flip
              }
              else {
                v11 = 0;
                *(char *)(v13 + v8 + (long)v4) = 0;
              }
              if (v11 == 3) break;
            }
            v13 += v6 + -1;
            v11 = 0;
            for (v8 = 0; (v8 < v6 && ((v8 <= 7 || (*(char *)((v13 - v8) + (long)v4) != '\x01')))); v8 = v8 + 1) {
              if (*(char *)((v13 - v8) + (long)v4) != '\x02') { // branch-flip
                v11 = (*(char *)((v13 - v8) + (long)v4)) ? v11 + 1 : 0; // branch-flip
              }
              else {
                v11 = 0;
                *(char *)((v13 - v8) + (long)v4) = 0;
              }
              if (v11 == 3) break;
            }
          }
          else {
            while ((long)v13 < (long)v7) {
              v7 -= 1;
              if (*(char *)(v7 + (long)v4) == '\x02')
                *(char *)(v7 + (long)v4) = 0;
            }
          }
        }
      }
      else {
        *(char *)(v13 + (long)v4) = 0;
      }
    }
  }
  for (v12 = 0; v12 <= 1; v12 = v12 + 1) {
    v4 = (&v9)[v12];
    v1 = *(long *)(a0 + (long)v12 * 0x130 + 200);
    v20 = 0;
    for (v13 = 0; (long)v13 < v1; v13 = v13 + 1) {
      if ((dat_2d348) || (!*(char *)((long)v4 + v13))) {
        *(unsigned long *)(*(long *)(a0 + (long)v12 * 0x130 + 0x100) + v20 * 8) = *(unsigned long *)(*(long *)(a0 + (long)v12 * 0x130 + 0xf8) + v13 * 8);
        v6 = v20 + 1;
        *(unsigned long *)(*(long *)(a0 + (long)v12 * 0x130 + 0x108) + v20 * 8) = v13;
        v20 = v6;
      }
      else {
        *(char *)(v13 + *(long *)(a0 + (long)v12 * 0x130 + 0x118)) = 1;
      }
    }
    *(long *)((long)v12 * 0x130 + a0 + 0x110) = v20;
  }
  free(v9);
  free(v10[0]);
}


// Function: sub_6b31 @ 0x6b31
void sub_6b31(long a0)
{
  long v1;
  long v10; // stack - 0x48
  long v11; // stack - 0x40
  long v12; // stack - 0x38
  long v2;
  long v3;
  long v4;
  long v5;
  char *v6; // rax
  long v7;
  int v8; // stack - 0x54
  long v9; // stack - 0x50
  
  for (v8 = 0; v8 <= 1; v8 = v8 + 1) {
    v2 = *(long *)(a0 + (long)v8 * 0x130 + 0x118);
    v3 = *(long *)(a0 + (long)(1 - v8) * 0x130 + 0x118);
    v4 = *(long *)(a0 + (long)v8 * 0x130 + 0xf8);
    v9 = 0;
    v10 = 0;
    v5 = *(long *)(a0 + (long)v8 * 0x130 + 200);
    while( true ) {
      for (; (v9 < v5 && (!*(char *)(v2 + v9))); v9 = v9 + 1) {
        do {
          v7 = v10 + 1;
          v6 = (char *)(v3 + v10);
          v10 = v7;
        } while (*v6);
      }
      if (v9 == v5) break;
      v11 = v9;
      do {
        v9 += 1;
      } while (*(char *)(v2 + v9));
      for (; *(char *)(v3 + v10); v10 = v10 + 1) {
      }
      do {
        v7 = v9 - v11;
        while ((v11 && (*(long *)(v4 + v11 * 8 + -8) == *(long *)(v4 + v9 * 8 + -8)))) {
          v11 -= 1;
          *(char *)(v2 + v11) = 1;
          v9 -= 1;
          *(char *)(v2 + v9) = 0;
          for (; *(char *)(v2 + v11 + -1); v11 = v11 + -1) {
          }
          do {
            v10 -= 1;
          } while (*(char *)(v3 + v10));
        }
        if (*(char *)(v3 + v10 + -1)) // branch-flip
          v12 = v9;
        else {
          v12 = v5;
        }
        while ((v9 != v5 && (*(long *)(v4 + v11 * 8) == *(long *)(v4 + v9 * 8)))) {
          v1 = v11 + 1;
          *(char *)(v2 + v11) = 0;
          *(char *)(v2 + v9) = 1;
          do {
            v9 += 1;
          } while (*(char *)(v2 + v9));
          while (v10 = v10 + 1, v11 = v1, *(char *)(v3 + v10)) {
            v12 = v9;
          }
        }
      } while (v7 != v9 - v11);
      while (v12 < v9) {
        v11 -= 1;
        *(char *)(v2 + v11) = 1;
        v9 -= 1;
        *(char *)(v2 + v9) = 0;
        do {
          v10 -= 1;
        } while (*(char *)(v3 + v10));
      }
    }
  }
}


// Function: sub_6e92 @ 0x6e92
unsigned long * sub_6e92(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4)
{
  unsigned long *v1; // rax
  
  v1 = (unsigned long *)sub_1c945(0x30);
  v1[3] = a0;
  v1[4] = a1;
  v1[1] = a3;
  v1[2] = a2;
  *v1 = a4;
  return v1;
}


// Function: sub_6f01 @ 0x6f01
unsigned long sub_6f01(long a0)
{
  long v1;
  long v2;
  long v3;
  long v4;
  long v5;
  long v6;
  unsigned long v7; // stack - 0x50
  long v8; // stack - 0x48
  long v9; // stack - 0x40
  
  v7 = 0;
  v1 = *(long *)(a0 + 0x118);
  v2 = *(long *)(a0 + 0x248);
  v3 = *(long *)(a0 + 200);
  v4 = *(long *)(a0 + 0x1f8);
  v8 = 0;
  for (v9 = 0; (v6 = v9, v5 = v8, v8 < v3 || (v9 < v4)); v9 = v9 + 1) {
    if (*(char *)(v2 + v9) || *(char *)(v1 + v8)) {
      for (; *(char *)(v1 + v8); v8 = v8 + 1) {
      }
      for (; *(char *)(v2 + v9); v9 = v9 + 1) {
      }
      v7 = sub_6e92(v5,v6,v8 - v5,v9 - v6,v7);
    }
    v8 += 1;
  }
  return v7;
}


// Function: sub_7037 @ 0x7037
unsigned long sub_7037(long a0)
{
  long v1;
  long v2;
  long v3;
  long v4;
  unsigned long v5; // stack - 0x40
  long v6; // stack - 0x38
  long v7; // stack - 0x30
  
  v5 = 0;
  v1 = *(long *)(a0 + 0x118);
  v2 = *(long *)(a0 + 0x248);
  v6 = *(long *)(a0 + 200);
  for (v7 = *(long *)(a0 + 0x1f8); (v4 = v7, v3 = v6, 0 <= v6 || (0 <= v7)); v7 = v7 + -1) {
    if (*(char *)(v2 + v7 + -1) || *(char *)(v1 + v6 + -1)) {
      for (; *(char *)(v1 + v6 + -1); v6 = v6 + -1) {
      }
      for (; *(char *)(v2 + v7 + -1); v7 = v7 + -1) {
      }
      v5 = sub_6e92(v6,v7,v3 - v6,v4 - v7,v5);
    }
    v6 -= 1;
  }
  return v5;
}


// Function: sub_7167 @ 0x7167
void sub_7167(int a0,long a1) // return-dupe
{
  char *v1; // rax
  long v2;
  long v3;
  
  if (!a0)
    return;
  if (dat_2d208) // branch-flip
    v3 = dat_2d208;
  else {
    v3 = *(long *)(a1 + 0x138);
  }
  if (dat_2d200) // branch-flip
    v2 = dat_2d200;
  else {
    v2 = *(long *)(a1 + 8);
  }
  if (dat_2d2a0) // branch-flip
    v1 = gettext("Files %s and %s differ\n");
  else {
    v1 = gettext("Binary files %s and %s differ\n");
  }
  sub_109a8(v1,v2,v3);
}


// Function: sub_7206 @ 0x7206
bool sub_7206(int *a0) // ternary x2
{
  int *v1;
  long v10;
  long v11;
  char *v12; // rax
  unsigned long v13; // stack - 0x48
  char v14 [8];
  char v15 [8];
  char v16 [8];
  char v17 [8];
  int *v18;
  unsigned long *v19;
  unsigned long *v2;
  bool v20;
  int v21; // stack - 0xd0
  unsigned long *v22; // stack - 0xa8
  unsigned long *v23; // stack - 0xa0
  long v24; // stack - 0x98
  long v25; // stack - 0x90
  unsigned long *v26; // stack - 0x88
  long v27; // stack - 0x80
  void *v28; // stack - 0x78
  unsigned long *v29; // stack - 0x70
  unsigned long v3;
  unsigned long *v30; // stack - 0x68
  unsigned long *v31; // stack - 0x60
  unsigned long v32; // stack - 0x58
  long v33; // stack - 0x50
  unsigned long v34; // stack - 0x40
  long v35; // stack - 0x38
  long v36; // stack - 0x30
  char v37; // stack - 0x28
  long v38; // stack - 0x20
  unsigned long v4;
  void *v5;
  void *v6;
  char v7; // al
  int v8; // eax
  unsigned long v9;
  
  v7 = sub_fa13(a0,dat_2d1ed);
  if (v7) { // branch-flip
    if (((((*(long *)&a0[0x10] != *(long *)&a0[0x5c]) && (1 <= *(long *)&a0[0x10])) && (1 <= *(long *)&a0[0x5c])) && ((*a0 <= -1 || ((a0[10] & 0xf000U) == 0x8000)))) && ((a0[0x4c] <= -1 || ((a0[0x56] & 0xf000U) == 0x8000))))
      v20 = 1;
    else if (*a0 != a0[0x4c]) { // branch-flip
      v32 = 0x7ffffffffffffffe;
      v9 = *(unsigned long *)&a0[0x5e];
      v3 = *(unsigned long *)&a0[0x12];
      v33 = sub_1d2a2(8,sub_1d2a2(v3,v9,0x7ffffffffffffffe),v32);
      for (v21 = 0; v21 <= 1; v21 = v21 + 1) {
        v9 = sub_1c991(*(unsigned long *)&a0[(long)v21 * 0x4c + 0x28],v33);
        *(unsigned long *)&a0[(long)v21 * 0x4c + 0x28] = v9;
      }
      while( true ) {
        for (v21 = 0; v21 <= 1; v21 = v21 + 1) {
          if (0 <= a0[(long)v21 * 0x4c])
            sub_de51(&a0[(long)v21 * 0x4c],v33 - *(long *)&a0[(long)v21 * 0x4c + 0x2c]);
        }
        if ((*(long *)&a0[0x2c] != *(long *)&a0[0x78]) || (v4 = *(unsigned long *)&a0[0x2c], v5 = *(void **)&a0[0x74], v6 = *(void **)&a0[0x28], memcmp(v6,v5,v4))) {
          v20 = 1;
          goto label_7572;
        }
        if (v33 != *(long *)&a0[0x2c]) break;
        a0[0x78] = 0;
        a0[0x79] = 0;
        *(unsigned long *)&a0[0x2c] = *(unsigned long *)&a0[0x78];
      }
      v20 = 0;
    }
    else {
      v20 = 0;
    }
label_7572:
    sub_7167(v20,a0);
  }
  else {
    v27 = *(long *)&a0[0x7e] + *(long *)&a0[0x32] + 4;
    v28 = (void *)sub_13030(v27);
    *(long *)&a0[0x46] = (long)v28 + 1;
    *(long *)&a0[0x92] = *(long *)&a0[0x32] + 3 + (long)v28;
    sub_6122(a0);
    v13 = *(unsigned long *)&a0[0x40];
    v34 = *(unsigned long *)&a0[0x8c];
    v24 = *(long *)&a0[0x90] + *(long *)&a0[0x44] + 3;
    v10 = sub_1c945(v24 * 0x10);
    v35 = (*(long *)&a0[0x90] + 1) * 8 + v10;
    v36 = (*(long *)&a0[0x90] + 1) * 8 + v24 * 8 + v10;
    v37 = dat_2d338;
    v25 = 1;
    for (; v24; v24 = v24 >> 2) {
      v25 <<= 1;
    }
    v38 = v25;
    if (v25 <= 0xfff)
      v38 = 0x1000;
    v10 = 0x26;
    v18 = a0;
    v19 = (unsigned long *)0x2d360;
    while (v10) {
      v2 = &v19[1];
      v1 = &v18[2];
      *v19 = *(unsigned long *)v18;
      v10 -= 1;
      v18 = v1;
      v19 = v2;
    }
    v10 = 0x26;
    v18 = &a0[0x4c];
    v19 = (unsigned long *)0x2d490;
    while (v10) {
      v2 = &v19[1];
      v1 = &v18[2];
      *v19 = *(unsigned long *)v18;
      v10 -= 1;
      v18 = v1;
      v19 = v2;
    }
    sub_5d78(0,*(unsigned long *)&a0[0x44],0,*(unsigned long *)&a0[0x90],dat_2d348,&v13);
    free((void *)(*(long *)&a0[0x90] * -8 + -8 + v35));
    sub_6b31(a0);
    v23 = (dat_2d1c0 != 4) ? (unsigned long *)sub_7037(a0) : (unsigned long *)sub_6f01(a0); // branch-flip
    if ((dat_2d1ec) || (dat_2d280)) {
      v26 = v23;
      v20 = 0;
      while ((v26 && (!v20))) {
        v29 = v26;
        v30 = (unsigned long *)sub_12437(v26);
        v26 = (unsigned long *)*v30;
        *v30 = 0;
        v8 = sub_12af4(v29,v17,v16,v15,v14);
        if (v8)
          v20 = 1;
        *v30 = v26;
      }
    }
    else {
      v20 = v23 != NULL;
    }
    if (dat_2d2a0) // branch-flip
      sub_7167(v20,a0);
    else if ((v20) || (dat_2d1c8 != '\x01')) {
      if (dat_2d208) // branch-flip
        v10 = dat_2d208;
      else {
        v10 = *(long *)&a0[0x4e];
      }
      if (dat_2d200) // branch-flip
        v11 = dat_2d200;
      else {
        v11 = *(long *)&a0[2];
      }
      sub_118ac(v11,v10,*(long *)&a0[0x98] != 0);
      switch(dat_2d1c0) {
        default:
          abort(); // no-return
        case 1:
          sub_fcfd(v23);
          break;
        case 2:
          sub_7f2c(v23,0);
          break;
        case 3:
          sub_7f2c(v23,1);
          break;
        case 4:
          sub_cb85(v23);
          break;
        case 5:
          sub_cd6a(v23);
          break;
        case 6:
          sub_ceba(v23);
          break;
        case 7:
          sub_d076(v23);
          break;
        case 8:
          sub_ff70(v23);
        
      }
      sub_11e1b();
    }
    free(*(void **)&a0[0x40]);
    free(v28);
    for (v21 = 0; v21 <= 1; v21 = v21 + 1) {
      free(*(void **)&a0[(long)v21 * 0x4c + 0x3e]);
      free((void *)(*(long *)&a0[(long)v21 * 0x4c + 0x30] * 8 + *(long *)&a0[(long)v21 * 0x4c + 0x2e]));
    }
    v22 = v23;
    while (v22) {
      v31 = (unsigned long *)*v22;
      free(v22);
      v22 = v31;
    }
    if ((dat_2d1c0 == 4) || (dat_2d1c0 == 5)) {
      for (v21 = 0; v21 <= 1; v21 = v21 + 1) {
        if ((char)a0[(long)v21 * 0x4c + 0x48]) {
          v12 = gettext("No newline at end of file");
          v9 = (*(long *)((long)v21 * 8 + 0x2d200)) ? *(unsigned long *)((long)v21 * 8 + 0x2d200) : *(unsigned long *)&a0[(long)v21 * 0x4c + 2]; // branch-flip
          error(0,0,"%s: %s\n",v9,v12); // no-return
        }
      }
    }
  }
  if (*(long *)&a0[0x28] != *(long *)&a0[0x74])
    free(*(void **)&a0[0x28]);
  free(*(void **)&a0[0x74]);
  return v20;
}


// Function: sub_7d13 @ 0x7d13
void sub_7d13(unsigned long a0,long a1,unsigned long a2,long a3)
{
  unsigned int v1; // eax
  void *v2; // rax
  long v3; // rax
  char v4 [56];
  
  sub_128c2(0);
  if (a3) // branch-flip
    fprintf(dat_2d5c0,"%s %s",a0,a3);
  else {
    v2 = localtime((void *)(a1 + 0x68));
    v1 = sub_1bb4f(a1 + 0x10);
    if ((!v2) || (v3 = sub_1710b(v4,0x2b,dat_2d350,v2,0,v1), !v3))
      sprintf(v4,"%ld.%.9d",*(unsigned long *)(a1 + 0x68),(unsigned long)v1);
    fprintf(dat_2d5c0,"%s %s\t%s",a0,a2,v4);
  }
  sub_128c2(3);
  putc_unlocked(10,dat_2d5c0);
}


// Function: sub_7e5c @ 0x7e5c
void sub_7e5c(long a0,unsigned long *a1,char a2) // return-dupe
{
  if (a2) {
    sub_7d13("---",a0,*a1,dat_2d200);
    sub_7d13("+++",a0 + 0x130,a1[1],dat_2d208);
    return;
  }
  sub_7d13("***",a0,*a1,dat_2d200);
  sub_7d13("---",a0 + 0x130,a1[1],dat_2d208);
}


// Function: sub_7f2c @ 0x7f2c
void sub_7f2c(unsigned long *a0,char a1) // return-dupe
{
  unsigned long *v1; // stack - 0x10
  
  if ((dat_2d1ec) || (dat_2d280))
    sub_8c9d(a0);
  else {
    for (v1 = a0; v1; v1 = (unsigned long *)*v1) {
      *(char *)&v1[5] = 0;
    }
  }
  dat_2d1b0 = -dat_2d448;
  dat_2d1b8 = 0x7fffffffffffffff;
  if (a1) {
    sub_1245b(a0,sub_8bbf,sub_8710);
    return;
  }
  sub_1245b(a0,sub_8bbf,sub_818f);
}


// Function: sub_7ff1 @ 0x7ff1
void sub_7ff1(unsigned long a0,unsigned long a1,unsigned long a2) // return-dupe
{
  long v1; // stack - 0x18
  long v2; // stack - 0x20
  
  sub_129d0(a0,a1,a2,&v2,&v1);
  if (v1 <= v2) {
    fprintf(dat_2d5c0,"%ld",v1);
    return;
  }
  fprintf(dat_2d5c0,"%ld,%ld",v2,v1);
}


// Function: sub_80a2 @ 0x80a2
void sub_80a2(void *a0,long a1)
{
  char v1;
  int v2; // stack - 0x10
  int v3; // stack - 0xc
  
  putc_unlocked(0x20,a0);
  for (v2 = 0; (v1 = *(char *)(a1 + v2), sub_133c9(v1) && (*(char *)(a1 + v2) != '\n')); v2 = v2 + 1) {
  }
  for (v3 = v2; (v3 <= v2 + 0x27 && (*(char *)(a1 + v3) != '\n')); v3 = v3 + 1) {
  }
  while ((v2 < v3 && (v1 = *(char *)(a1 + (long)v3 + -1), sub_133c9(v1)))) {
    v3 -= 1;
  }
  fwrite_unlocked((void *)(v2 + a1),1,(long)(v3 - v2),a0);
}


// Function: sub_818f @ 0x818f
void sub_818f(unsigned long *a0) // return-dupe x2, ternary x4
{
  unsigned int v1; // eax
  unsigned long *v10; // stack - 0x28
  unsigned long *v11; // stack - 0x20
  void *v12; // stack - 0x18
  long v2; // rax
  long v3; // stack - 0x48
  long v4; // stack - 0x50
  long v5; // stack - 0x58
  long v6; // stack - 0x60
  long v7; // stack - 0x40
  char *v8; // stack - 0x38
  long v9; // stack - 0x30
  
  v1 = sub_12af4(a0,&v6,&v5,&v4,&v3);
  if (!v1)
    return;
  v7 = -dat_2d448;
  v2 = v6 - dat_2d1d0;
  v6 = v7;
  if (v7 <= v2)
    v6 = v2;
  v2 = v4 - dat_2d1d0;
  v4 = v7;
  if (v7 <= v2)
    v4 = v2;
  v5 = (dat_2d430 - dat_2d1d0 <= v5) ? dat_2d430 + -1 : dat_2d1d0 + v5; // branch-flip
  v3 = (dat_2d560 - dat_2d1d0 <= v3) ? dat_2d560 + -1 : dat_2d1d0 + v3; // branch-flip
  v9 = 0;
  if (dat_2d240)
    v9 = sub_8d30(dat_2d418,v6);
  sub_11b50();
  v12 = dat_2d5c0;
  fputs_unlocked("***************",dat_2d5c0);
  if (v9)
    sub_80a2(v12,v9);
  putc_unlocked(10,v12);
  sub_128c2(4);
  fputs_unlocked("*** ",v12);
  sub_7ff1(0x2d360,v6,v5);
  fputs_unlocked(" ****",v12);
  sub_128c2(3);
  putc_unlocked(10,v12);
  if (v1 & 1) {
    v10 = a0;
    for (v7 = v6; v7 <= v5; v7 = v7 + 1) {
      sub_128c2(2);
      for (; (v10 && ((long)(v10[2] + v10[3]) <= v7)); v10 = (unsigned long *)*v10) {
      }
      v8 = " ";
      if ((v10) && ((long)v10[3] <= v7)) {
        v8 = (1 <= (long)v10[1]) ? "!" : "-"; // branch-flip
      }
      sub_124fe(v8,dat_2d418 + v7 * 8,1);
      sub_128c2(3);
      if (*(char *)(*(long *)((v7 + 1) * 8 + dat_2d418) + -1) == '\n')
        putc_unlocked(10,v12);
    }
  }
  sub_128c2(4);
  fputs_unlocked("--- ",v12);
  sub_7ff1(0x2d490,v4,v3);
  fputs_unlocked(" ----",v12);
  sub_128c2(3);
  putc_unlocked(10,v12);
  if (v1 & 2) {
    v11 = a0;
    for (v7 = v4; v7 <= v3; v7 = v7 + 1) {
      sub_128c2(1);
      for (; (v11 && ((long)(v11[1] + v11[4]) <= v7)); v11 = (unsigned long *)*v11) {
      }
      v8 = " ";
      if ((v11) && ((long)v11[4] <= v7)) {
        v8 = (1 <= (long)v11[2]) ? "!" : "+"; // branch-flip
      }
      sub_124fe(v8,dat_2d548 + v7 * 8,1);
      sub_128c2(3);
      if (*(char *)(*(long *)((v7 + 1) * 8 + dat_2d548) + -1) == '\n')
        putc_unlocked(10,v12);
    }
    return;
  }
}


// Function: sub_863e @ 0x863e
void sub_863e(unsigned long a0,unsigned long a1,unsigned long a2) // return-dupe, ternary
{
  char *v1; // rax
  long v2; // stack - 0x18
  long v3; // stack - 0x20
  
  sub_129d0(a0,a1,a2,&v3,&v2);
  if (v2 <= v3) {
    v1 = (v3 <= v2) ? "%ld" : "%ld,0"; // branch-flip
    fprintf(dat_2d5c0,v1,v2);
    return;
  }
  fprintf(dat_2d5c0,"%ld,%ld",v3,(v2 - v3) + 1);
}


// Function: sub_8710 @ 0x8710
void sub_8710(unsigned long *a0) // return-dupe, ternary x3
{
  long v1;
  long v10; // stack - 0x48
  unsigned long *v11; // stack - 0x40
  long v12; // stack - 0x38
  void *v13; // stack - 0x30
  unsigned long *v14; // stack - 0x28
  unsigned long *v15; // stack - 0x20
  unsigned long *v16; // stack - 0x18
  long v2;
  int v3; // edx
  long v4; // stack - 0x60
  long v5; // stack - 0x68
  long v6; // stack - 0x70
  long v7; // stack - 0x78
  long v8; // stack - 0x58
  long v9; // stack - 0x50
  
  if (!sub_12af4(a0,&v7,&v6,&v5,&v4))
    return;
  v8 = -dat_2d448;
  v2 = v7 - dat_2d1d0;
  v7 = v8;
  if (v8 <= v2)
    v7 = v2;
  v2 = v5 - dat_2d1d0;
  v5 = v8;
  if (v8 <= v2)
    v5 = v2;
  v6 = (dat_2d430 - dat_2d1d0 <= v6) ? dat_2d430 + -1 : dat_2d1d0 + v6; // branch-flip
  v4 = (dat_2d560 - dat_2d1d0 <= v4) ? dat_2d560 + -1 : dat_2d1d0 + v4; // branch-flip
  v12 = 0;
  if (dat_2d240)
    v12 = sub_8d30(dat_2d418,v7);
  sub_11b50();
  v13 = dat_2d5c0;
  sub_128c2(4);
  fputs_unlocked("@@ -",v13);
  sub_863e(0x2d360,v7,v6);
  fputs_unlocked(" +",v13);
  sub_863e(0x2d490,v5,v4);
  fputs_unlocked(" @@",v13);
  sub_128c2(3);
  if (v12)
    sub_80a2(v13,v12);
  putc_unlocked(10,v13);
  v8 = v7;
  v9 = v5;
  v11 = a0;
  while ((v8 <= v6 || (v9 <= v4))) {
    if ((v11) && ((long)v11[3] <= v8)) {
      v10 = v11[2];
      while (v2 = v10 + -1, v10) {
        v1 = v8 + 1;
        v15 = (unsigned long *)(v8 * 8 + dat_2d418);
        v8 = v1;
        v10 = v2;
        sub_128c2(2);
        putc_unlocked(0x2d,v13);
        if ((dat_2d2b0) && ((dat_2d2b1 != '\x01' || (*(char *)*v15 != '\n'))))
          putc_unlocked(9,v13);
        sub_124fe(0,v15,1);
        sub_128c2(3);
        if (*(char *)(v15[1] + -1) == '\n')
          putc_unlocked(10,v13);
      }
      v10 = v11[1];
      while (v2 = v10 + -1, v10) {
        v1 = v9 + 1;
        v14 = (unsigned long *)(v9 * 8 + dat_2d548);
        v9 = v1;
        v10 = v2;
        sub_128c2(1);
        putc_unlocked(0x2b,v13);
        if ((dat_2d2b0) && ((dat_2d2b1 != '\x01' || (*(char *)*v14 != '\n'))))
          putc_unlocked(9,v13);
        sub_124fe(0,v14,1);
        sub_128c2(3);
        if (*(char *)(v14[1] + -1) == '\n')
          putc_unlocked(10,v13);
      }
      v11 = (unsigned long *)*v11;
      v10 = 0xffffffffffffffff;
    }
    else {
      v2 = v8 + 1;
      v16 = (unsigned long *)(v8 * 8 + dat_2d418);
      v8 = v2;
      if ((dat_2d2b1 != '\x01') || (*(char *)*v16 != '\n')) {
        v3 = (dat_2d2b0) ? 9 : 0x20; // branch-flip
        putc_unlocked(v3,v13);
      }
      sub_124cf(0,v16);
      v9 += 1;
    }
  }
}


// Function: sub_8bbf @ 0x8bbf
long * sub_8bbf(long *a0)
{
  long *v1;
  long v2;
  long *v3; // stack - 0x40
  
  v1 = a0;
  while( true ) {
    v3 = v1;
    v1 = (long *)*v3;
    if ((v1) && (*(char *)&v1[5]))
      v2 = dat_2d1d0;
    else {
      v2 = dat_2d1d0 * 2 + 1;
    }
    if ((v1) && (v1[3] - (v3[2] + v3[3]) != v1[4] - (v3[1] + v3[4]))) break;
    if ((!v1) || (v2 <= v1[3] - (v3[2] + v3[3])))
      return v3;
  }
  abort(); // no-return
}


// Function: sub_8c9d @ 0x8c9d
void sub_8c9d(unsigned long *a0)
{
  char v1 [8];
  char v2 [8];
  char v3 [8];
  char v4 [8];
  unsigned long *v5; // stack - 0x40
  unsigned long *v6; // stack - 0x18
  
  v5 = a0;
  while (v5) {
    v6 = (unsigned long *)*v5;
    *v5 = 0;
    *(bool *)&v5[5] = sub_12af4(v5,v4,v3,v2,v1) == 0;
    *v5 = v6;
    v5 = v6;
  }
}


// Function: sub_8d30 @ 0x8d30
long sub_8d30(long a0,long a1)
{
  long v1;
  long v2;
  long v3;
  unsigned long v4; // rax
  long v5; // stack - 0x28
  
  v3 = dat_2d1b0;
  dat_2d1b0 = a1;
  v5 = a1;
  do {
    v1 = v5 + -1;
    if (v1 < v3) {
      if (dat_2d1b8 != 0x7fffffffffffffff)
        return *(long *)(a0 + dat_2d1b8 * 8);
      return 0;
    }
    v2 = *(long *)(a0 + v1 * 8);
    v4 = (*(long *)(a0 + v5 * 8) - v2) - 1;
    if (0x80000000 <= v4)
      v4 = 0x7fffffff;
    v5 = v1;
  } while ((int)re_search(0x2d220,v2,v4 & 0xffffffff,0,v4 & 0xffffffff,0) < 0);
  dat_2d1b8 = v1;
  return v2;
}


// Function: sub_8e44 @ 0x8e44
char * sub_8e44(long a0,int a1)
{
  unsigned long v1;
  char *v2; // rax
  int v3; // stack - 0x24
  long v4; // stack - 0x20
  char *v5; // stack - 0x18
  
  v4 = 1;
  for (v3 = 0; v3 < a1; v3 = v3 + 1) {
    v1 = *(unsigned long *)(a0 + (long)v3 * 8);
    v4 += sub_1a4f6(v1) + 1;
  }
  v2 = (char *)sub_1c945(v4);
  v5 = v2;
  for (v3 = 0; v3 < a1; v3 = v3 + 1) {
    *v5 = 0x20;
    v1 = *(unsigned long *)(a0 + (long)v3 * 8);
    v5 = (char *)sub_1a550(&v5[1],v1);
  }
  *v5 = 0;
  return v2;
}


// Function: sub_8f0e @ 0x8f0e
unsigned long sub_8f0e(void) // return-dupe
{
  if (dat_2d1ef)
    return 0x10000010;
  return 0x10000000;
}


// Function: sub_8f2f @ 0x8f2f
unsigned long sub_8f2f(int a0,char **a1) // ternary x2, warn: iteboolean: re-rolled 2 0/1 select diamond(s) to a boolean assignment in sub_8f2f
{
  char *v1;
  unsigned long v10; // rax
  unsigned long v11; // rax
  unsigned long v12; // rax
  char *v13; // stack - 0x88
  long v14; // stack - 0x98
  long v15; // stack - 0x90
  int v16; // stack - 0xb0
  int v17; // stack - 0xac
  unsigned int v18; // stack - 0xa8
  int v19; // stack - 0xa4
  char *v2;
  unsigned long v20; // stack - 0x80
  unsigned long v21; // stack - 0x78
  unsigned long v22; // stack - 0x70
  char *v23; // stack - 0x68
  char *v24; // stack - 0x60
  unsigned long v25; // stack - 0x58
  unsigned long v26; // stack - 0x50
  unsigned long v27; // stack - 0x48
  unsigned long v28; // stack - 0x40
  unsigned long v29; // stack - 0x38
  bool v3;
  bool v4;
  unsigned char v5; // al
  unsigned int v6; // eax
  int v7; // eax
  unsigned long v8;
  unsigned long v9; // rax
  
  v16 = 0;
  v19 = -1;
  v20 = 0xffffffffffffffff;
  v4 = 0;
  v21 = 0;
  v3 = 0;
  v14 = 0;
  v15 = 0;
  dat_2d0c8 = 2;
  sub_19a53(*a1);
  setlocale(6,"");
  bindtextdomain("diffutils","/usr/local/share/locale");
  textdomain("diffutils");
  sub_13721(0);
  dat_2d600 = 0x2d220;
  dat_2d640 = 0x2d260;
  re_set_syntax(0x50a46);
  dat_2d340 = sub_139c7();
  dat_2d5c8 = 0;
  sub_1cfb9();
  do {
    v17 = getopt_long(a0,a1,"0123456789abBcC:dD:eEfF:hHiI:lL:nNpPqrsS:tTuU:vwW:x:X:yZ",(void *)0x2bf40,NULL);
    if (v17 == -1) {
      if (dat_2d1c4 == 1) {
        v24 = getenv("TERM");
        if ((v24) && (v7 = strcmp(v24,"dumb"), !v7))
          dat_2d1c4 = 0;
      }
      if (!dat_2d1c0) {
        if (v3) {
          sub_a63e(2);
          if ((long)v20 <= -1)
            dat_2d1d0 = 3;
        }
        else {
          sub_a63e(1);
        }
      }
      if ((dat_2d1c0 != 2) || (sub_14cdf(2)))
        dat_2d350 = "%Y-%m-%d %H:%M:%S.%N %z";
      else {
        dat_2d350 = "%a %b %e %T %Y";
      }
      if (((0 <= (long)v20) && ((dat_2d1c0 == 2 || (dat_2d1c0 == 3)))) && (((long)dat_2d1d0 < (long)v20 || (((long)v20 < (long)dat_2d1d0 && (!v4))))))
        dat_2d1d0 = v20;
      if (!dat_2d2a8)
        dat_2d2a8 = 8;
      if (!v21)
        v21 = 0x82;
      if (dat_2d2a1) // branch-flip
        v25 = 1;
      else {
        v25 = dat_2d2a8;
      }
      v26 = v21;
      v27 = v25 + 3;
      v28 = (unsigned long)((unsigned int)v21 & (unsigned int)v27 & 1) + (v21 >> 1) + (v27 >> 1);
      v29 = v28 - v28 % v25;
      if ((4 <= v29) && (v29 < v21)) {
        dat_2d320 = v21 - v29;
        if (v29 - 3 <= v21 - v29)
          dat_2d320 = v29 - 3;
      }
      else {
        dat_2d320 = 0;
      }
      if (dat_2d320) // branch-flip
        dat_2d328 = v29;
      else {
        dat_2d328 = v21;
      }
      if ((long)dat_2d1e0 < (long)dat_2d1d0)
        dat_2d1e0 = dat_2d1d0;
      sub_a296(0x2d5e0);
      sub_a296(0x2d620);
      if (dat_2d1c0 == 7) {
        for (v18 = 0; v18 <= 2; v18 = v18 + 1) {
          if (!*(long *)((long)(int)v18 * 8 + 0x2d300))
            *(char **)((long)(int)v18 * 8 + 0x2d300) = "%l\n";
        }
        if (!dat_2d2e8) {
          if (dat_2d2f8) // branch-flip
            dat_2d2e8 = dat_2d2f8;
          else {
            dat_2d2e8 = "%<";
          }
        }
        if (!dat_2d2f0) {
          if (dat_2d2f8) // branch-flip
            dat_2d2f0 = dat_2d2f8;
          else {
            dat_2d2f0 = "%>";
          }
        }
        if (!dat_2d2e0)
          dat_2d2e0 = "%=";
        if (!dat_2d2f8)
          dat_2d2f8 = (char *)sub_12fa9(dat_2d2e8,dat_2d2f0,0x2353e);
      }
      if (dat_2d1c0 != 7) // branch-flip
        dat_2d1c8 = dat_2d31a || dat_2d1c0 != 8;
      else {
        dat_2d1c8 = !((*dat_2d2e0) && ((v7 = strcmp(dat_2d2e0,"%="), v7 || (*dat_2d300))));
      }
      v5 = ((dat_2d620) || (dat_2d1e8));
      dat_2d1ed = (~(v5 | dat_2d2b2 | dat_2d1ec | dat_2d1ee) & dat_2d2a0 & 1) != 0;
      dat_2d330 = sub_8e44(&a1[1],optind + -1);
      if (v14) { // branch-flip
        if (v15)
          sub_10966("--from-file and --to-file both specified"); // no-return
        for (; optind < a0; optind = optind + 1) {
          v7 = sub_a746(0,v14,a1[optind]);
          if (v16 < v7)
            v16 = v7;
        }
      }
      else if (v15) { // branch-flip
        for (; optind < a0; optind = optind + 1) {
          v7 = sub_a746(0,a1[optind],v15);
          if (v16 < v7)
            v16 = v7;
        }
      }
      else {
        if (a0 - optind != 2) {
          if (2 > a0 - optind)
            sub_a338("missing operand after \'%s\'",a1[(long)a0 + -1]); // no-return
          sub_a338("extra operand \'%s\'",a1[(long)optind + 2]); // no-return
        }
        v1 = a1[(long)optind + 1];
        v2 = a1[optind];
        v16 = sub_a746(0,v2,v1);
      }
      sub_10bca();
      sub_a3ae();
      exit(v16); // no-return
    }
    switch(v17) {
      case 0:
      case 0x68:
      case 0x80:
      case 0x85:
        break;
      default:
        sub_a338(0,0); // no-return
      case 0x30:
      case 0x31:
      case 0x32:
      case 0x33:
      case 0x34:
      case 0x35:
      case 0x36:
      case 0x37:
      case 0x38:
      case 0x39:
        if (10 <= (unsigned int)(v19 - 0x30U)) // branch-flip
          v20 = (unsigned long)(v17 + -0x30);
        else {
          v20 = (0x666666666666666 <= (long)(v20 - (v17 <= 0x33))) ? 0x3fffffffffffffff : (long)(v17 + -0x30) + v20 * 10; // branch-flip
        }
        break;
      case 0x42:
        dat_2d1ec = 1;
        break;
      case 0x43:
      case 0x55:
        if (optarg) { // branch-flip
          v22 = strtoimax(optarg,&v13,10);
          if ((*v13) || ((long)v22 <= -1))
            sub_a338("invalid context length \'%s\'",optarg); // no-return
          if (0x4000000000000000 <= (long)v22)
            v22 = 0x3fffffffffffffff;
        }
        else {
          v22 = 3;
        }
        v6 = (v17 != 0x55) ? 2 : 3; // branch-flip
        sub_a63e(v6);
        if ((long)dat_2d1d0 < (long)v22)
          dat_2d1d0 = v22;
        v4 = 1;
        break;
      case 0x44:
        sub_a63e(7);
        v8 = strlen(optarg);
        v23 = (char *)sub_1c945(v8 * 7 + 0x61);
        sprintf(v23,"%%=%c#ifndef %s\n%%<#endif /* ! %s */\n%c#ifdef %s\n%%>#endif /* %s */\n%c#ifndef %s\n%%<#else /* %s */\n%%>#endif /* %s */\n",0,optarg,optarg,0,optarg,optarg,0,optarg,optarg,optarg);
        for (v18 = 0; v18 <= 3; v18 = v18 + 1) {
          sub_a5af((long)(int)v18 * 8 + 0x2d2e0,v23,"-D");
          v8 = strlen(v23);
          v23 = &v23[v8 + 1];
        }
        break;
      case 0x45:
        if (dat_2d1e8 <= 3)
          dat_2d1e8 |= 1;
        break;
      case 0x46:
        sub_a107(0x2d5e0,optarg);
        break;
      case 0x48:
        dat_2d338 = 1;
        break;
      case 0x49:
        sub_a107(0x2d620,optarg);
        break;
      case 0x4c:
        if (dat_2d200) { // branch-flip
          if (dat_2d208)
            sub_10966("too many file label options"); // no-return
          dat_2d208 = optarg;
        }
        else {
          dat_2d200 = optarg;
        }
        break;
      case 0x4e:
        dat_2d648 = 1;
        break;
      case 0x50:
        dat_2d649 = 1;
        break;
      case 0x53:
        sub_a5af(0x2d2b8,optarg,"-S");
        break;
      case 0x54:
        dat_2d2b0 = 1;
        break;
      case 0x57:
        v22 = strtoimax(optarg,&v13,10);
        if (((long)v22 <= 0) || (*v13))
          sub_a338("invalid width \'%s\'",optarg); // no-return
        if (v21 != v22) {
          if (v21)
            sub_10966("conflicting width options"); // no-return
          v21 = v22;
        }
        break;
      case 0x58:
        v7 = sub_148b2(sub_142e9,dat_2d340,optarg,sub_8f0e(),10);
        if (v7)
          sub_10925(optarg); // no-return
        break;
      case 0x5a:
        if (dat_2d1e8 <= 3)
          dat_2d1e8 |= 2;
        break;
      case 0x61:
        dat_2d1d8 = 1;
        break;
      case 0x62:
        if (dat_2d1e8 <= 3)
          dat_2d1e8 = 4;
        break;
      case 99:
        sub_a63e(2);
        if ((long)dat_2d1d0 <= 2)
          dat_2d1d0 = 3;
        break;
      case 100:
        dat_2d348 = 1;
        break;
      case 0x65:
        sub_a63e(4);
        break;
      case 0x66:
        sub_a63e(5);
        break;
      case 0x69:
        dat_2d1ee = 1;
        break;
      case 0x6c:
        if (!s_25400[0])
          sub_a338("pagination not supported on this host",0); // no-return
        dat_2d2c0 = 1;
        signal(0x11,0);
        break;
      case 0x6e:
        sub_a63e(6);
        break;
      case 0x70:
        v3 = 1;
        sub_a107(0x2d5e0,"^[[:alpha:]$_]");
        break;
      case 0x71:
        dat_2d2a0 = 1;
        break;
      case 0x72:
        dat_2d5c9 = 1;
        break;
      case 0x73:
        dat_2d64a = 1;
        break;
      case 0x74:
        dat_2d2a1 = '\x01';
        break;
      case 0x75:
        sub_a63e(3);
        if ((long)dat_2d1d0 <= 2)
          dat_2d1d0 = 3;
        break;
      case 0x76:
        v9 = sub_1a014("Len Tower");
        v10 = sub_1a014("Richard Stallman");
        v11 = sub_1a014("David Hayes");
        v12 = sub_1a014("Mike Haertel");
        sub_1c71c(stdout,"diff","GNU diffutils",dat_2d0c0,sub_1a014("Paul Eggert"),v12,v11,v10,v9,0);
        sub_a3ae();
        return 0;
      case 0x77:
        dat_2d1e8 = 5;
        break;
      case 0x78:
        sub_142e9(dat_2d340,optarg,sub_8f0e());
        break;
      case 0x79:
        sub_a63e(8);
        break;
      case 0x81:
        sub_a5af(&v14,optarg,"--from-file");
        break;
      case 0x82:
        sub_a405();
        sub_a3ae();
        return 0;
      case 0x83:
        v22 = strtoimax(optarg,&v13,10);
        if ((*v13) || ((long)v22 <= -1))
          sub_a338("invalid horizon length \'%s\'",optarg); // no-return
        v8 = v22;
        if ((long)v22 <= (long)dat_2d1e0)
          v8 = dat_2d1e0;
        dat_2d1e0 = v8;
        break;
      case 0x84:
        dat_2d1ef = 1;
        break;
      case 0x86:
        dat_2d319 = 1;
        break;
      case 0x87:
        sub_a63e(7);
        for (v18 = 0; v18 <= 2; v18 = v18 + 1) {
          sub_a5af((long)(int)v18 * 8 + 0x2d300,optarg,"--line-format");
        }
        break;
      case 0x88:
        dat_2d1f0 = 1;
        break;
      case 0x89:
        dat_2d1ef = 0;
        break;
      case 0x8a:
        sub_a63e(1);
        break;
      case 0x8b:
        sub_a63e(8);
        dat_2d318 = 1;
        break;
      case 0x8c:
        dat_2d2b2 = 1;
        break;
      case 0x8d:
        dat_2d2b1 = 1;
        break;
      case 0x8e:
        dat_2d31a = '\x01';
        break;
      case 0x8f:
        v22 = strtoimax(optarg,&v13,10);
        if ((((long)v22 <= 0) || (0xfffffffffffffffd <= v22)) || (*v13))
          sub_a338("invalid tabsize \'%s\'",optarg); // no-return
        if (dat_2d2a8 != v22) {
          if (dat_2d2a8)
            sub_10966("conflicting tabsize options"); // no-return
          dat_2d2a8 = v22;
        }
        break;
      case 0x90:
        sub_a5af(&v15,optarg,"--to-file");
        break;
      case 0x91:
      case 0x92:
      case 0x93:
        sub_a63e(7);
        v17 -= 0x91;
        sub_a5af((long)v17 * 8 + 0x2d300,optarg,&"--unchanged-line-format"[(long)v17 * 0x18]);
        break;
      case 0x94:
      case 0x95:
      case 0x96:
      case 0x97:
        sub_a63e(7);
        v17 -= 0x94;
        sub_a5af((long)v17 * 8 + 0x2d2e0,optarg,&"--unchanged-group-format"[(long)v17 * 0x19]);
        break;
      case 0x98:
        sub_a682(optarg);
        break;
      case 0x99:
        sub_1140f(optarg);
        break;
      case 0x9a:
        dat_2d5c8 = 1;
      
    }
    v19 = v17;
  } while( true );
}


// Function: sub_a107 @ 0xa107
void sub_a107(long *a0,char *a1)
{
  unsigned char v1;
  unsigned long v2; // rax
  long v3;
  long v4; // stack - 0x38
  long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  
  v2 = strlen(a1);
  v3 = re_compile_pattern(a1,v2,a0[4]);
  if (v3)
    error(2,0,"%s: %s",a1,v3); // no-return
  v4 = *a0;
  v5 = a0[1];
  *(bool *)&a0[3] = v4 != 0;
  v1 = *(unsigned char *)&a0[3];
  a0[1] = (int)((unsigned int)v1 * 2) + v5 + v2;
  v6 = a0[2];
  if (v6 <= (unsigned long)a0[1]) {
    if (!v6)
      v6 = 1;
    do {
      v6 <<= 1;
    } while (v6 <= (unsigned long)a0[1]);
    a0[2] = v6;
    v4 = sub_1c991(v4,v6);
    *a0 = v4;
  }
  if (v1) {
    v3 = v5 + 1;
    *(char *)(v5 + v4) = 0x5c;
    v5 += 2;
    *(char *)(v3 + v4) = 0x7c;
  }
  memcpy((void *)(v4 + v5),a1,v2 + 1);
}


// Function: sub_a296 @ 0xa296
void sub_a296(long *a0)
{
  long v1;
  
  if (*a0) {
    v1 = a0[4];
    *(unsigned long *)(v1 + 0x20) = sub_1c945(0x100);
    if ((char)a0[3]) {
      v1 = re_compile_pattern(*a0,a0[1],a0[4]);
      if (v1)
        error(2,0,"%s: %s",*a0,v1); // no-return
    }
  }
}


// Function: sub_a338 @ 0xa338
void sub_a338(char *a0,unsigned long a1)
{
  unsigned long v1;
  
  v1 = dat_3d8a8;
  if (a0)
    error(0,0,gettext(a0),a1); // no-return
  error(2,0,gettext("Try \'%s --help\' for more information."),v1); // no-return
}


// Function: sub_a3ae @ 0xa3ae
void sub_a3ae(void)
{
  int v1; // eax
  
  v1 = ferror_unlocked(stdout);
  if (v1)
    sub_10966("write failed"); // no-return
  v1 = fclose(stdout);
  if (v1)
    sub_10925(gettext("standard output")); // no-return
}


// Function: sub_a405 @ 0xa405
void sub_a405(void) // warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_a405
{
  void *v1;
  unsigned long v2;
  unsigned int v3; // eax
  int v4; // eax
  char *v5;
  long *v6; // stack - 0x30
  char *v7; // stack - 0x28
  
  v2 = dat_3d8a8;
  printf(gettext("Usage: %s [OPTION]... FILES\n"),v2);
  printf("%s\n\n",gettext("Compare FILES line by line."));
  v1 = stdout;
  fputs_unlocked(gettext("Mandatory arguments to long options are mandatory for short options too.\n"),v1);
  v6 = (long *)0x2c6e0;
  do {
    if (!*v6) {
      sub_1c800();
      return;
    }
    if (*(char *)*v6) { // branch-flip
      v5 = (char *)*v6;
      v7 = gettext(v5);
      while( true ) {
        v5 = strchr(v7,10);
        if (!v5) break;
        v3 = ((int)v5 + 1) - (int)v7;
        if (0x1000 <= (int)v3)
          __assert_fail("msglen < 4096","diff.c",0x3f8,"usage"); // no-return
        printf("  %.*s",(unsigned long)v3,v7);
        v7 = &v5[1];
      }
      v4 = ((*v7 != ' ') && (*v7 != '-'));
      printf(&"  %s\n"[v4 * 2],v7);
    }
    else {
      putchar_unlocked(10);
    }
    v6 = &v6[1];
  } while( true );
}


// Function: sub_a5af @ 0xa5af
void sub_a5af(long *a0,char *a1,unsigned long a2)
{
  char *v1;
  
  if (*a0) {
    v1 = (char *)*a0;
    if (strcmp(v1,a1))
      error(0,0,gettext("conflicting %s option value \'%s\'"),a2,a1); // no-return
  }
  *a0 = (long)a1;
}


// Function: sub_a63e @ 0xa63e
void sub_a63e(int a0) // return-dupe
{
  if (a0 == dat_2d1c0)
    return;
  if (!dat_2d1c0) {
    dat_2d1c0 = a0;
    return;
  }
  sub_a338("conflicting output style options",0); // no-return
}


// Function: sub_a682 @ 0xa682
void sub_a682(char *a0)
{
  if ((!a0) || (!strcmp(a0,"auto"))) {
    dat_2d1c4 = 1;
    return;
  }
  if (!strcmp(a0,"always")) {
    dat_2d1c4 = 2;
    return;
  }
  if (!strcmp(a0,"never")) {
    dat_2d1c4 = 0;
    return;
  }
  sub_a338("invalid color \'%s\'",a0); // no-return
}


// Function: sub_a723 @ 0xa723
void sub_a723(long a0)
{
  sub_14c76(a0 + 0x58);
}


// Function: sub_a746 @ 0xa746
int sub_a746(long a0,char *a1,char *a2) // ternary x2
{
  bool v1;
  char *v10;
  int v11 [2]; // stack - 0x288
  long v12 [3]; // stack - 0x278
  long v13; // stack - 0x148
  char *v14 [2]; // stack - 0x298
  unsigned int v15 [2]; // stack - 0x260
  long v16 [31]; // stack - 0x250
  char *v17; // stack - 0x310
  char *v18; // stack - 0x308
  int v19; // stack - 0x2f4
  int v2;
  char *v20; // stack - 0x2e0
  char *v21; // stack - 0x2d8
  char *v22; // stack - 0x280
  int v23; // stack - 0x158
  char *v24; // stack - 0x150
  long v25; // stack - 0x140
  long v26; // stack - 0x138
  unsigned int v27; // stack - 0x130
  int v28; // stack - 0x12c
  int v29; // stack - 0x128
  unsigned int v3; // eax
  long v30; // stack - 0x120
  long v31; // stack - 0x118
  long v32; // stack - 0xf0
  long v33; // stack - 0xe0
  long v34; // stack - 0x28
  unsigned long v35; // stack - 0x258
  int v4;
  unsigned long v5; // rax
  unsigned long v6; // rax
  char *v7;
  long v8;
  long v9;
  
  v19 = 0;
  if ((((!a1) || (!a2)) && ((dat_2d649 != '\x01' || (!a2)))) && (dat_2d648 != '\x01')) {
    if (a1)
      a2 = a1;
    sub_109a8("Only in %s: %s\n",*(unsigned long *)((long)(int)(unsigned int)(a1 == NULL) * 0x130 + a0 + 8),a2);
    return 1;
  }
  memset(v11,0,0x260);
  v34 = a0;
  v11[0] = (a1) ? -2 : -1; // branch-flip
  v23 = (a2) ? -2 : -1; // branch-flip
  v18 = a1;
  if (!a1)
    v18 = a2;
  v17 = a2;
  if (!a2)
    v17 = v18;
  if (a0) { // branch-flip
    v20 = (char *)sub_14ade(*(unsigned long *)(a0 + 8),v18,0);
    v22 = v20;
    v21 = (char *)sub_14ade(*(unsigned long *)(a0 + 0x138),v17,0);
    v24 = v21;
  }
  else {
    v20 = NULL;
    v21 = NULL;
    v22 = v18;
    v24 = v17;
  }
  for (v4 = 0; v4 <= 1; v4 = v4 + 1) {
    if (v11[(long)v4 * 0x4c] != -1) {
      if ((v4) && (v2 = strcmp((char *)v12[(long)v4 * 0x26 + -1],v22), !v2)) {
        v11[(long)v4 * 0x4c] = v11[0];
        v9 = (long)v4;
        v12[v9 * 0x26] = v12[0];
        v12[v9 * 0x26 + 1] = v12[1];
        v12[v9 * 0x26 + 2] = v12[2];
        *(unsigned long *)&v15[v9 * 0x4c] = CONCAT44(v15[1],v15[0]);
        v16[v9 * 0x26 + -1] = CONCAT44(v35._4_4_,(int)v35);
        v16[v9 * 0x26] = v16[0];
        v16[v9 * 0x26 + 1] = v16[1];
        v16[v9 * 0x26 + 2] = v16[2];
        v16[v9 * 0x26 + 3] = v16[3];
        v16[v9 * 0x26 + 4] = v16[4];
        v16[v9 * 0x26 + 5] = v16[5];
        v16[v9 * 0x26 + 6] = v16[6];
        v16[v9 * 0x26 + 7] = v16[7];
        v16[v9 * 0x26 + 8] = v16[8];
        v16[v9 * 0x26 + 9] = v16[9];
        v16[v9 * 0x26 + 10] = v16[10];
        v16[v9 * 0x26 + 0xb] = v16[0xb];
        v16[v9 * 0x26 + 0xc] = v16[0xc];
      }
      else {
        v10 = (char *)v12[(long)v4 * 0x26 + -1];
        if (strcmp(v10,"-")) { // branch-flip
          if (dat_2d1f0) // branch-flip
            v2 = lstat((char *)v12[(long)v4 * 0x26 + -1],&v12[(long)v4 * 0x26]);
          else {
            v2 = stat((char *)v12[(long)v4 * 0x26 + -1],&v12[(long)v4 * 0x26]);
          }
          if (v2)
            v11[(long)v4 * 0x4c] = 0xfffffffd - *__errno_location();
        }
        else {
          v11[(long)v4 * 0x4c] = 0;
          if (!isatty(0))
            sub_131e6(0,0);
          if (fstat(0,&v12[(long)v4 * 0x26])) // branch-flip
            v11[(long)v4 * 0x4c] = 0xfffffffd - *__errno_location();
          else {
            if ((v15[(long)v4 * 0x4c] & 0xf000) == 0x8000) {
              v9 = lseek(0,0,1);
              if (0 <= v9) { // branch-flip
                v8 = 0;
                if (0 <= v16[(long)v4 * 0x26 + 1] - v9)
                  v8 = v16[(long)v4 * 0x26 + 1] - v9;
                v16[(long)v4 * 0x26 + 1] = v8;
              }
              else {
                v11[(long)v4 * 0x4c] = 0xfffffffd - *__errno_location();
              }
            }
            sub_a723(&v12[(long)v4 * 0x26]);
          }
        }
      }
    }
  }
  for (v4 = 0; v4 <= 1; v4 = v4 + 1) {
    if ((dat_2d648) || ((!v4 && (dat_2d649)))) {
      if (v11[(long)v4 * 0x4c] != -2) { // branch-flip
        if (((v11[(long)v4 * 0x4c] != -5) && (v11[(long)v4 * 0x4c] != -0xc)) || ((a0 || ((v11[(long)(1 - v4) * 0x4c] != -2 && (v11[(long)(1 - v4) * 0x4c]))))))
          v1 = 0;
        else {
          v1 = 1;
        }
      }
      else if ((((v15[(long)v4 * 0x4c] & 0xf000) != 0x8000) || (v15[(long)v4 * 0x4c] & 0x1ff)) || (v16[(long)v4 * 0x26 + 1]))
        v1 = 0;
      else {
        v1 = 1;
      }
      if (v1)
        v11[(long)v4 * 0x4c] = -1;
    }
  }
  for (v4 = 0; v4 <= 1; v4 = v4 + 1) {
    if (v11[(long)v4 * 0x4c] == -1) {
      memset(&v12[(long)v4 * 0x26],0,0x90);
      v15[(long)v4 * 0x4c] = v15[(long)(1 - v4) * 0x4c];
    }
  }
  for (v4 = 0; v4 <= 1; v4 = v4 + 1) {
    v2 = v11[(long)v4 * 0x4c];
    if (0 <= 0xfffffffdU - v2) {
      *__errno_location() = 0xfffffffdU - v2;
      sub_108ec(v12[(long)v4 * 0x26 + -1]);
      v19 = 2;
    }
  }
  if (((!v19) && (!a0)) && (((v27 & 0xf000) == 0x4000) != ((v15[0] & 0xf000) == 0x4000))) {
    v3 = (unsigned int)((v15[0] & 0xf000) == 0x4000);
    v4 = 1 - v3;
    v10 = (char *)v12[(long)(int)v3 * 0x26 + -1];
    v9 = v12[(long)v4 * 0x26 + -1];
    v20 = (char *)sub_c9bd(v9,sub_13111(v10));
    v12[(long)v4 * 0x26 + -1] = (long)v20;
    v7 = (char *)v12[(long)v4 * 0x26 + -1];
    if (!strcmp(v10,"-"))
      sub_10966("cannot compare \'-\' to a directory"); // no-return
    if (dat_2d1f0) // branch-flip
      v4 = lstat(v7,&v12[(long)v4 * 0x26]);
    else {
      v4 = stat(v7,&v12[(long)v4 * 0x26]);
    }
    if (v4) {
      sub_108ec(v7);
      v19 = 2;
    }
  }
  if ((v19) || ((v11[0] == -1 && (v23 == -1)))) goto label_bcfe;
  if ((v11[0] != -1) && (v23 != -1)) {
    if (((v12[1] != v25) || (v12[0] != v13)) && (((((v15[0] & 0xf000) != 0x6000 || ((v27 & 0xf000) != 0x6000)) && (((v15[0] & 0xf000) != 0x2000 || ((v27 & 0xf000) != 0x2000)))) || (v16[0] != v30))))
      v1 = 0;
    else {
      v1 = 1;
    }
    if ((((((!v1) || (v15[0] != v27)) || (v12[2] != v26)) || ((v15[1] != v28 || ((int)v35 != v29)))) || (v16[1] != v31)) || ((v16[6] != v32 || (v16[8] != v33)))) goto label_b57a;
    v1 = 1;
  }
  else {
label_b57a:
    v1 = 0;
  }
  if ((!v1) || (!dat_2d1c8)) {
    if ((v27 & 0xf000) != 0x4000 || (v15[0] & 0xf000) != 0x4000) { // branch-flip
      if (((v27 & 0xf000) != 0x4000 && (v15[0] & 0xf000) != 0x4000) && ((!a0 || ((((v15[0] & 0xf000) == 0x8000 || ((v15[0] & 0xf000) == 0xa000)) && (((v27 & 0xf000) == 0x8000 || ((v27 & 0xf000) == 0xa000)))))))) {
        if (((v15[0] & 0xf000) != 0xa000) && ((v27 & 0xf000) != 0xa000)) {
          if ((((dat_2d1ed) && ((v15[0] & 0xf000) == 0x8000)) && ((v27 & 0xf000) == 0x8000)) && (((v16[1] != v31 && (1 <= v16[1])) && (1 <= v31)))) {
            if (dat_2d208) // branch-flip
              v10 = dat_2d208;
            else {
              v10 = v24;
            }
            if (dat_2d200) // branch-flip
              v7 = dat_2d200;
            else {
              v7 = v22;
            }
            sub_109a8("Files %s and %s differ\n",v7,v10);
            v19 = 1;
          }
          else {
            if (v11[0] == -2) {
              v11[0] = open(v22,0,0);
              if (v11[0] <= -1) {
                sub_108ec(v22);
                v19 = 2;
              }
            }
            if (v23 == -2) {
              if (v1)
                v23 = v11[0];
              else {
                v23 = open(v24,0,0);
                if (v23 <= -1) {
                  sub_108ec(v24);
                  v19 = 2;
                }
              }
            }
            if (!v19)
              v19 = sub_7206(v11);
            if ((0 <= v11[0]) && (v4 = close(v11[0]), v4)) {
              sub_108ec(v22);
              v19 = 2;
            }
            if (((0 <= v23) && (v11[0] != v23)) && (v4 = close(v23), v4)) {
              sub_108ec(v24);
              v19 = 2;
            }
          }
        }
        else {
          if (!dat_2d1f0)
            __assert_fail("no_dereference_symlinks","diff.c",0x549,"compare_files"); // no-return
          if (((v15[0] & 0xf000) != 0xa000) || ((v27 & 0xf000) != 0xa000)) {
            v5 = sub_14997(&v13);
            if (dat_2d208) // branch-flip
              v10 = dat_2d208;
            else {
              v10 = v24;
            }
            v6 = sub_14997(v12);
            if (dat_2d200) // branch-flip
              v7 = dat_2d200;
            else {
              v7 = v22;
            }
            sub_109e2("File %s is a %s while file %s is a %s\n",v7,v6,v10,v5);
            v19 = 1;
          }
          else {
            v14[0] = NULL;
            v14[1] = NULL;
            for (v4 = 0; v4 <= 1; v4 = v4 + 1) {
              v9 = v12[(long)v4 * 0x26 + -1];
              v14[v4] = (char *)sub_1cf7b(v9);
              if (!v14[v4]) {
                sub_108ec(v12[(long)v4 * 0x26 + -1]);
                v19 = 2;
                break;
              }
            }
            if ((!v19) && (v4 = strcmp(v14[0],v14[1]), v4)) {
              sub_109a8("Symbolic links %s and %s differ\n",v22,v24);
              v19 = 1;
            }
            for (v4 = 0; v4 <= 1; v4 = v4 + 1) {
              free(v14[v4]);
            }
          }
        }
      }
      else if ((v11[0] != -1) && (v23 != -1)) {
        v5 = sub_14997(&v13);
        if (dat_2d208) // branch-flip
          v10 = dat_2d208;
        else {
          v10 = v24;
        }
        v6 = sub_14997(v12);
        if (dat_2d200) // branch-flip
          v7 = dat_2d200;
        else {
          v7 = v22;
        }
        sub_109e2("File %s is a %s while file %s is a %s\n",v7,v6,v10,v5);
        v19 = 1;
      }
      else if (((v27 & 0xf000) != 0x4000 && (v15[0] & 0xf000) != 0x4000) || ((!dat_2d5c9 || ((!dat_2d648 && ((!dat_2d649 || (v11[0] != -1)))))))) {
        if (!a0)
          __assert_fail("parent","diff.c",0x52d,"compare_files"); // no-return
        sub_109a8("Only in %s: %s\n",*(unsigned long *)((long)(int)(unsigned int)(v11[0] == -1) * 0x130 + a0 + 8),v18);
        v19 = 1;
      }
      else {
        v19 = sub_c21b(v11,sub_a746);
      }
    }
    else {
      if (dat_2d1c0 == 7)
        sub_10966("-D option not supported with directories"); // no-return
      if ((a0) && (dat_2d5c9 != '\x01'))
        sub_109a8("Common subdirectories: %s and %s\n",v22,v24);
      else {
        v19 = sub_c21b(v11,sub_a746);
      }
    }
  }
label_bcfe:
  if (v19) { // branch-flip
    v4 = fflush_unlocked(stdout);
    if (v4)
      sub_10925(gettext("standard output")); // no-return
  }
  else if ((dat_2d64a) && ((v15[0] & 0xf000) != 0x4000)) {
    if (dat_2d208) // branch-flip
      v10 = dat_2d208;
    else {
      v10 = v24;
    }
    if (dat_2d200) // branch-flip
      v7 = dat_2d200;
    else {
      v7 = v22;
    }
    sub_109a8("Files %s and %s are identical\n",v7,v10);
  }
  free(v20);
  free(v21);
  return v19;
}


// Function: sub_bde2 @ 0xbde2
unsigned long sub_bde2(int *a0,unsigned long *a1) // return-dupe x3
{
  char *v1;
  unsigned long v10; // stack - 0x40
  long v11; // stack - 0x38
  int v2;
  char v3; // al
  void *v4; // rax
  void *v5; // rax
  unsigned long v6; // rax
  unsigned long v7; // rbx
  unsigned long v8; // stack - 0x50
  char *v9; // stack - 0x48
  
  a1[1] = 0;
  a1[2] = 0;
  v8 = 0;
  v9 = NULL;
  if (*a0 != -1) {
    v4 = opendir(*(char **)&a0[2]);
    if (!v4)
      return 0;
    v10 = 0x200;
    v11 = 0;
    v9 = (char *)sub_1c945(0x200);
    a1[2] = (unsigned long)v9;
    while( true ) {
      *__errno_location() = 0;
      v5 = readdir(v4);
      if (!v5) break;
      v1 = (char *)((long)v5 + 0x13);
      v6 = strlen((char *)((long)v5 + 0x13)) + 1;
      if (((*v1 != '.') || ((*(char *)((long)v5 + 0x14) && ((*(char *)((long)v5 + 0x14) != '.' || (*(char *)((long)v5 + 0x15))))))) && (v3 = sub_141f6(dat_2d340,v1), !v3)) {
        while (v10 < v6 + v11) {
          if (0x3fffffffffffffff <= v10)
            sub_1cf09(); // no-return
          v10 <<= 1;
          v9 = (char *)sub_1c991(v9,v10);
          a1[2] = (unsigned long)v9;
        }
        memcpy(&v9[v11],v1,v6);
        v11 += v6;
        v8 += 1;
      }
    }
    if (*__errno_location()) {
      v2 = *__errno_location();
      closedir(v4);
      *__errno_location() = v2;
      return 0;
    }
    if (closedir(v4))
      return 0;
  }
  if (v8 <= 0xffffffffffffffd) {
    v6 = sub_1c945((v8 + 1) * 8);
    a1[1] = v6;
    *a1 = v8;
    for (v7 = 0; v7 < v8; v7 = v7 + 1) {
      *(char **)(v7 * 8 + v6) = v9;
      v9 = &v9[strlen(v9) + 1];
    }
    *(unsigned long *)(v6 + v8 * 8) = 0;
    return 1;
  }
  sub_1cf09(); // no-return
}


// Function: sub_c084 @ 0xc084
int sub_c084(char *a0,char *a1) // ternary
{
  char *v1; // rax
  int v2; // stack - 0x1c
  
  *__errno_location() = 0;
  v2 = (dat_2d1ef) ? strcasecmp(a0,a1) : strcoll(a0,a1); // branch-flip
  if (*__errno_location()) {
    v1 = gettext("cannot compare file names \'%s\' and \'%s\'");
    error(0,*__errno_location(),v1,a0,a1); // no-return
  }
  return v2;
}


// Function: sub_c140 @ 0xc140
unsigned long sub_c140(char *a0,char *a1)
{
  unsigned int v1; // eax
  
  if ((dat_2d660) && ((v1 = sub_c084(a0,a1), v1 || (dat_2d1ef))))
    return (unsigned long)v1;
  return CONCAT44(dat_4,strcmp(a0,a1));
}


// Function: sub_c1a0 @ 0xc1a0
unsigned long sub_c1a0(unsigned long *a0,unsigned long *a1)
{
  char *v1;
  char *v2;
  unsigned int v3; // eax
  unsigned long v4; // rax
  
  v1 = (char *)*a0;
  v2 = (char *)*a1;
  if (dat_2d660) { // branch-flip
    v3 = sub_c084(v1,v2);
    if (!v3) {
      v3 = strcmp(v1,v2); // return-dupe
      return v4;
    }
    v4 = (unsigned long)v3;
  }
  else {
    v3 = strcmp(v1,v2);
  }
  return v4;
}


// Function: sub_c21b @ 0xc21b
unsigned int sub_c21b(int *a0,void *a1)
{
  unsigned long v1;
  long v10;
  long *v11 [9]; // stack - 0x68
  int v12; // stack - 0xa0
  int v13; // stack - 0x9c
  long *v14; // stack - 0x80
  long *v2;
  char *v3;
  int v4;
  int v5;
  int v6;
  char v7; // al
  unsigned int v8; // eax
  long **v9;
  
  v12 = 0;
  if (((*a0 == -1) || (v7 = sub_c796(a0,0), v7)) && ((a0[0x4c] == -1 || (v7 = sub_c796(a0,1), v7)))) {
    v8 = (unsigned int)(*a0 == -1);
    v9 = (int *)&a0[(long)(int)v8 * 0x4c + 2];
    v1 = *v9;
    v9 = (char *)gettext("%s: recursive directory loop");
    error(0,0,(char *)v9,v1); // no-return
  }
  for (v13 = 0; v13 <= 1; v13 = v13 + 1) {
    v9 = (int *)&a0[(long)v13 * 0x4c];
    v7 = sub_bde2(v9,&v11[(long)v13 * 3 + 2]);
    if (v7 != '\x01') {
      v9 = (int *)&a0[(long)v13 * 0x4c + 2];
      sub_108ec(*v9);
      v12 = 2;
    }
  }
  if (!v12) {
    v11[0] = v11[3];
    v11[1] = v11[6];
    dat_2d660 = 1;
    v8 = _setjmp(0x2d680);
    if (v8)
      dat_2d660 = 0;
    for (v13 = 0; v13 <= 1; v13 = v13 + 1) {
      v9 = &v11[(long)v13 * 3 + 2];
      v2 = *v9;
      v9 = (long)(long)v13;
      qsort(v11[(long)v9],v2,8,sub_c1a0);
    }
    if ((dat_2d2b8) && (!*(long *)&a0[0x98])) {
      while ((*v11[0] && (v8 = sub_c140(*v11[0],dat_2d2b8), (int)v8 <= -1))) {
        v9 = (long *)&v11[0][1];
        v11[0] = (long *)v9;
      }
      while ((*v11[1] && (v8 = sub_c140(*v11[1],dat_2d2b8), (int)v8 <= -1))) {
        v9 = (long *)&v11[1][1];
        v11[1] = (long *)v9;
      }
    }
    while ((*v11[0] || (*v11[1]))) {
      if (*v11[0]) { // branch-flip
        if (*v11[1]) // branch-flip
          v8 = sub_c140(*v11[0],*v11[1]);
        else {
          v8 = -1;
        }
      }
      else {
        v8 = 1;
      }
      v5 = v8;
      if (((!v8) && (dat_2d1ef)) && (v8 = strcmp((char *)*v11[0],(char *)*v11[1]), v8)) {
        v4 = (int)v8 >> 0x1f;
        v8 = -v4;
        v6 = v8;
        v8 = v4 + 1;
        v9 = (long)(long)(int)v8;
        v2 = v11[(long)v9];
        v9 = (long)(long)v6;
        v3 = (char *)*v11[(long)v9];
        v14 = v2;
        do {
          v14 = &v14[1];
          if ((!*v14) || (v8 = sub_c140(*v14,v3), v8)) goto label_c676;
          v8 = strcmp((char *)*v14,v3);
        } while ((int)v8 <= -1);
        if (!v8) {
          v9 = (unsigned long)((long)v14 - (long)v2);
          memmove(&v2[1],v2,(unsigned long)v9);
          *v2 = (long)v3;
        }
      }
label_c676:
      if (0 <= v5) { // branch-flip
        v2 = &v11[1][1];
        v10 = *v11[1];
        v11[1] = v2;
      }
      else {
        v10 = 0;
      }
      if (1 <= v5) // branch-flip
        v9 = NULL;
      else {
        v2 = &v11[0][1];
        v9 = (long)*v11[0];
        v11[0] = v2;
      }
      v8 = (*a1)(a0,v9,v10);
      if (v12 < (int)v8)
        v12 = v8;
    }
  }
  for (v13 = 0; v13 <= 1; v13 = v13 + 1) {
    v9 = &v11[(long)v13 * 3 + 3];
    free(*v9);
    v9 = &v11[(long)v13 * 3 + 4];
    free(*v9);
  }
  v8 = v12;
  return v8;
}


// Function: sub_c796 @ 0xc796
unsigned long sub_c796(long a0,int a1)
{
  bool v1;
  long v2; // stack - 0x10
  
  v2 = a0;
  do {
    v2 = *(long *)(v2 + 0x260);
    if (!v2)
      return 0;
    if (((*(long *)((long)a1 * 0x130 + v2 + 0x18) != *(long *)((long)a1 * 0x130 + a0 + 0x18)) || (*(long *)((long)a1 * 0x130 + v2 + 0x10) != *(long *)((long)a1 * 0x130 + a0 + 0x10))) && (((((*(unsigned int *)((long)a1 * 0x130 + v2 + 0x28) & 0xf000) != 0x6000 || ((*(unsigned int *)((long)a1 * 0x130 + a0 + 0x28) & 0xf000) != 0x6000)) && (((*(unsigned int *)((long)a1 * 0x130 + v2 + 0x28) & 0xf000) != 0x2000 || ((*(unsigned int *)((long)a1 * 0x130 + a0 + 0x28) & 0xf000) != 0x2000)))) || (*(long *)((long)a1 * 0x130 + v2 + 0x38) != *(long *)((long)a1 * 0x130 + a0 + 0x38)))))
      v1 = 0;
    else {
      v1 = 1;
    }
  } while (!v1);
  return 1;
}


// Function: sub_c9bd @ 0xc9bd
unsigned long sub_c9bd(unsigned long a0,char *a1)
{
  long v1;
  unsigned long v10; // stack - 0x140
  char *v2;
  unsigned long v3; // rax
  char v4 [8];
  unsigned int v5 [2]; // stack - 0x148
  char *v6; // stack - 0x180
  long *v7; // stack - 0x178
  long *v8; // stack - 0x160
  void *v9; // stack - 0x158
  
  v8 = NULL;
  v9 = NULL;
  v6 = a1;
  if (dat_2d1ef) {
    v5[0] = 0;
    v10 = a0;
    if (sub_bde2(v5,v4)) {
      dat_2d660 = 1;
      if (!_setjmp(0x2d680)) {
        for (v7 = v8; *v7; v7 = &v7[1]) {
          v1 = *v7;
          if (!sub_c140(v1,a1)) {
            v2 = (char *)*v7;
            if (!strcmp(v2,a1)) {
              v6 = (char *)*v7;
              break;
            }
            if (v6 == a1)
              v6 = (char *)*v7;
          }
        }
      }
    }
  }
  v3 = sub_14ade(a0,v6,0);
  free(v8);
  free(v9);
  return v3;
}


// Function: sub_cb85 @ 0xcb85
void sub_cb85(unsigned long a0)
{
  sub_1245b(a0,sub_12449,sub_cbb5);
}


// Function: sub_cbb5 @ 0xcbb5
void sub_cbb5(unsigned long a0) // return-dupe x3
{
  bool v1;
  unsigned int v2; // eax
  long v3; // stack - 0x20
  long v4; // stack - 0x28
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x38
  long v7; // stack - 0x18
  
  v2 = sub_12af4(a0,&v6,&v5,&v4,&v3);
  if (!v2)
    return;
  sub_11b50();
  sub_12a37(0x2c,0x2d360,v6,v5);
  fputc_unlocked((int)*(char *)((unsigned long)v2 + 0x25780),dat_2d5c0);
  fputc_unlocked(10,dat_2d5c0);
  if (v2 == 1)
    return;
  v1 = 1;
  for (v7 = v4; v7 <= v3; v7 = v7 + 1) {
    if (!v1) {
      fputs_unlocked("a\n",dat_2d5c0);
      v1 = 1;
    }
    if ((**(char **)(v7 * 8 + dat_2d548) != '.') || (*(char *)(*(long *)(v7 * 8 + dat_2d548) + 1) != '\n'))
      sub_124cf(0x25164,v7 * 8 + dat_2d548);
    else {
      fputs_unlocked("..\n.\ns/.//\n",dat_2d5c0);
      v1 = 0;
    }
  }
  if (!v1)
    return;
  fputs_unlocked(".\n",dat_2d5c0);
}


// Function: sub_cd6a @ 0xcd6a
void sub_cd6a(unsigned long a0)
{
  sub_1245b(a0,sub_12437,sub_cd9a);
}


// Function: sub_cd9a @ 0xcd9a
void sub_cd9a(unsigned long a0) // return-dupe x2
{
  unsigned int v1; // eax
  long v2; // stack - 0x20
  long v3; // stack - 0x28
  unsigned long v4; // stack - 0x30
  unsigned long v5; // stack - 0x38
  long v6; // stack - 0x18
  
  v1 = sub_12af4(a0,&v5,&v4,&v3,&v2);
  if (!v1)
    return;
  sub_11b50();
  fputc_unlocked((int)*(char *)((unsigned long)v1 + 0x25780),dat_2d5c0);
  sub_12a37(0x20,0x2d360,v5,v4);
  fputc_unlocked(10,dat_2d5c0);
  if (v1 == 1)
    return;
  for (v6 = v3; v6 <= v2; v6 = v6 + 1) {
    sub_124cf(0x25164,v6 * 8 + dat_2d548);
  }
  fputs_unlocked(".\n",dat_2d5c0);
}


// Function: sub_ceba @ 0xceba
void sub_ceba(unsigned long a0)
{
  sub_1245b(a0,sub_12437,sub_ceea);
}


// Function: sub_ceea @ 0xceea
void sub_ceea(unsigned long a0) // return-dupe x2
{
  unsigned int v1; // eax
  long v10; // stack - 0x28
  long v11; // stack - 0x18
  long v2;
  long v3; // stack - 0x40
  long v4; // stack - 0x48
  unsigned long v5; // stack - 0x50
  unsigned long v6; // stack - 0x58
  long v7; // stack - 0x30
  long v8; // stack - 0x38
  long v9; // stack - 0x20
  
  v1 = sub_12af4(a0,&v6,&v5,&v4,&v3);
  if (!v1)
    return;
  sub_11b50();
  sub_129d0(0x2d360,v6,v5,&v8,&v7);
  if (v1 & 1) {
    if (v8 <= v7) // branch-flip
      v2 = (v7 - v8) + 1;
    else {
      v2 = 1;
    }
    fprintf(dat_2d5c0,"d%ld %ld\n",v8,v2);
  }
  if (!(v1 & 2))
    return;
  sub_129d0(0x2d490,v4,v3,&v10,&v9);
  if (v10 <= v9) // branch-flip
    v2 = (v9 - v10) + 1;
  else {
    v2 = 1;
  }
  fprintf(dat_2d5c0,"a%ld %ld\n",v7,v2);
  for (v11 = v4; v11 <= v3; v11 = v11 + 1) {
    sub_124cf(0x25164,v11 * 8 + dat_2d548);
  }
}


// Function: sub_d076 @ 0xd076
void sub_d076(unsigned long a0) // return-dupe
{
  dat_2d748 = -dat_2d448;
  dat_2d750 = dat_2d748;
  sub_1245b(a0,sub_12437,sub_d11e);
  if ((dat_2d430 <= dat_2d748) && (dat_2d560 <= dat_2d750))
    return;
  sub_11b50();
  sub_d221(dat_2d2e0,dat_2d748,dat_2d430,dat_2d750,dat_2d560);
}


// Function: sub_d11e @ 0xd11e
void sub_d11e(unsigned long a0) // return-dupe
{
  unsigned int v1; // eax
  long v2; // stack - 0x18
  long v3; // stack - 0x20
  long v4; // stack - 0x28
  long v5; // stack - 0x30
  
  v1 = sub_12af4(a0,&v5,&v4,&v3,&v2);
  if (!v1)
    return;
  sub_11b50();
  if ((dat_2d748 < v5) || (dat_2d750 < v3))
    sub_d221(dat_2d2e0,dat_2d748,v5,dat_2d750,v3);
  dat_2d748 = v4 + 1;
  dat_2d750 = v2 + 1;
  sub_d221(*(unsigned long *)((unsigned long)v1 * 8 + 0x2d2e0),v5,dat_2d748,v3,dat_2d750);
}


// Function: sub_d221 @ 0xd221
void sub_d221(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4)
{
  unsigned long v1; // stack - 0x48
  unsigned long v2; // stack - 0x40
  unsigned long v3; // stack - 0x38
  unsigned long v4; // stack - 0x30
  unsigned long v5; // stack - 0x28
  unsigned long v6; // stack - 0x20
  
  v1 = 0x2d360;
  v4 = 0x2d490;
  v2 = a1;
  v3 = a2;
  v5 = a3;
  v6 = a4;
  sub_d2bc(dat_2d5c0,a0,0,&v1);
}


// Function: sub_d2bc @ 0xd2bc
char * sub_d2bc(void *a0,char *a1,char a2,long a3)
{
  char *v1;
  char *v2;
  char *v3; // stack - 0x68
  long v4 [3]; // stack - 0x48
  char v5;
  int v6; // stack - 0x6c
  void *v7; // stack - 0x60
  void *v8; // stack - 0x58
  char *v9; // stack - 0x50
  
label_d542:
  do {
    v5 = *a1;
    if ((v5 == a2) || (!v5))
      return a1;
    v9 = &a1[1];
    v2 = v9;
    if (v5 != '%') goto label_d52f;
    v2 = &a1[2];
    v5 = *v9;
    switch(v5) {
      case 0x25:
label_d52f:
        a1 = v2;
        if (a0)
          putc_unlocked((int)v5,a0);
        goto label_d542;
      default:
        a1 = (char *)sub_da03(a0,a1,0,0,a3);
        if (a1) goto label_d542;
label_d522:
        v5 = '%';
        v2 = v9;
        goto label_d52f;
      case 0x28:
        for (v6 = 0; v6 <= 1; v6 = v6 + 1) {
          if (10 <= (unsigned int)((int)*v2 - 0x30U)) { // branch-flip
            v5 = *v2;
            v4[v6] = sub_d577(a3,(int)v5);
            if (v4[v6] < 0) goto label_d522;
            v1 = &v2[1];
          }
          else {
            *__errno_location() = 0;
            v4[v6] = strtoimax(v2,&v3,10);
            if (*__errno_location()) goto label_d522;
            v1 = v3;
          }
          v2 = &v1[1];
          if (*v1 != "=?"[v6]) goto label_d522;
        }
        if (v4[0] != v4[1]) { // branch-flip
          v7 = NULL;
          v8 = a0;
        }
        else {
          v8 = NULL;
          v7 = a0;
        }
        a1 = (char *)sub_d2bc(v7,v2,0x3a,a3);
        if ((*a1) && (a1 = (char *)sub_d2bc(v8,&a1[1],0x29,a3), *a1))
          a1 = &a1[1];
        break;
      case 0x3c:
        sub_d6bd(a0,dat_2d308,a3);
        a1 = v2;
        break;
      case 0x3d:
        sub_d6bd(a0,dat_2d300,a3);
        a1 = v2;
        break;
      case 0x3e:
        goto label_d4cc;
      
    }
  } while( true );
label_d4cc:
  sub_d6bd(a0,dat_2d310,a3 + 0x18);
  a1 = v2;
  goto label_d542;
}


// Function: sub_d577 @ 0xd577
long sub_d577(unsigned long *a0,char a1) // early-return
{
  unsigned long v1;
  unsigned long v2;
  long v3; // rax
  char v4; // stack - 0x14
  unsigned long *v5; // stack - 0x10
  
  v4 = a1;
  v5 = a0;
  switch(a1) {
    case 0x45:
      v4 = 0x65;
      v5 = &a0[3];
      break;
    case 0x46:
      v4 = 0x66;
      v5 = &a0[3];
      break;
    case 0x4c:
      v4 = 0x6c;
      v5 = &a0[3];
      break;
    case 0x4d:
      v4 = 0x6d;
      v5 = &a0[3];
      break;
    case 0x4e:
      v4 = 0x6e;
      v5 = &a0[3];
    
  }
  switch(v4) {
    case 0x65:
      v1 = v5[1];
      v2 = *v5;
      v3 = sub_129a8(v2,v1) + -1;
      break;
    case 0x66:
      v3 = sub_129a8(*v5,v5[1]);
      break;
    default:
      return -1;
    case 0x6c:
      v1 = v5[2];
      v2 = *v5;
      v3 = sub_129a8(v2,v1) + -1;
      break;
    case 0x6d:
      v3 = sub_129a8(*v5,v5[2]);
      break;
    case 0x6e:
      v3 = v5[2] - v5[1];
    
  }
  return v3;
}


// Function: sub_d6bd @ 0xd6bd
void sub_d6bd(void *a0,char *a1,long *a2) // return-dupe x3
{
  char *v1;
  long v2;
  long v3;
  long v4;
  char *v5;
  char *v6;
  char v7;
  long v8; // stack - 0x50
  
  v2 = *a2;
  v3 = *(long *)(v2 + 0xb8);
  v8 = a2[1];
  v4 = a2[2];
  if (!a0)
    return;
  if ((dat_2d2a1 != '\x01') && (*a1 == '%')) {
    if ((a1[1] == 'l') && (((a1[2] == '\n' && (!a1[3])) && (v8 < v4)))) {
      fwrite_unlocked(*(void **)(v3 + v8 * 8),1,(*(long *)(v3 + v4 * 8) + (unsigned long)(*(char *)(*(long *)(v3 + v4 * 8) + -1) != '\n')) - *(long *)(v3 + v8 * 8),a0);
      return;
    }
    if ((a1[1] == 'L') && (!a1[2])) {
      fwrite_unlocked(*(void **)(v3 + v8 * 8),1,*(long *)(v3 + v4 * 8) - *(long *)(v3 + v8 * 8),a0);
      return;
    }
  }
  do {
    if (v4 <= v8)
      return;
    v5 = a1;
    while( true ) {
      v1 = &v5[1];
      v7 = *v5;
      if (!v7) break;
      v6 = v1;
      if (v7 != '%') { // branch-flip
label_d9c0:
        putc_unlocked((int)v7,a0); // crossjump-dupe
        v5 = v6;
      }
      else {
        v6 = &v5[2];
        v7 = *v1;
        if (v7 != 'l') { // branch-flip
          if ('m' <= v7) { // branch-flip
label_d989:
            v5 = (char *)sub_da03(a0,v5,v2,v8,0);
            if (!v5) {
              v7 = '%';
              v6 = v1;
              putc_unlocked((int)v7,a0);
              v5 = v6;
            }
          }
          else {
            if (v7 == '%') goto label_d9c0;
            if (v7 != 'L') goto label_d989;
            sub_1267d(*(unsigned long *)(v3 + v8 * 8),*(unsigned long *)(v3 + (v8 + 1) * 8),0,0);
            v5 = v6;
          }
        }
        else {
          sub_1267d(*(unsigned long *)(v3 + v8 * 8),*(long *)(v3 + (v8 + 1) * 8) - (unsigned long)(*(char *)(*(long *)(v3 + (v8 + 1) * 8) + -1) == '\n'),0,0);
          v5 = v6;
        }
      }
    }
    v8 += 1;
  } while( true );
}


// Function: sub_da03 @ 0xda03
char * sub_da03(void *a0,void *a1,long a2,unsigned long a3,unsigned long a4) // early-return x5
{
  long v1;
  long v10; // stack - 0x70
  void *v11; // stack - 0x68
  void *v12; // stack - 0x60
  char v13; // stack - 0x4a
  char v14; // stack - 0x49
  char *v15; // stack - 0x48
  long v16; // stack - 0x40
  long v17; // stack - 0x38
  char *v18; // stack - 0x30
  unsigned long v19; // stack - 0x28
  char *v2;
  char *v20; // stack - 0x20
  char *v21; // stack - 0x18
  unsigned long v3; // rax
  char v4 [8];
  char v5; // stack - 0x4b
  char *v6; // rsp
  char *v7; // rsp
  unsigned long v8; // stack - 0x80
  unsigned long v9; // stack - 0x78
  
  v6 = v4;
  v12 = a0;
  v11 = a1;
  v10 = a2;
  v9 = a3;
  v8 = a4;
  v15 = (char *)((long)a1 + 1);
  do {
    do {
      v2 = &v15[1];
      v13 = *v15;
      v15 = v2;
    } while (v13 == '-');
  } while ((v13 == '\'') || (v13 == '0'));
  while ((unsigned int)((int)v13 - 0x30U) <= 9) {
    v2 = &v15[1];
    v13 = *v15;
    v15 = v2;
  }
  if (v13 == '.') {
    do {
      v2 = &v15[1];
      v13 = *v15;
      v15 = v2;
    } while ((unsigned int)((int)v13 - 0x30U) <= 9);
  }
  v2 = v15;
  v15 = &v15[1];
  v14 = *v2;
  if (0x21 <= (unsigned int)((int)v13 - 0x58U))
    return NULL;
  v3 = 1L << ((unsigned char)((int)v13 - 0x58U) & 0x3f);
  if (v3 & 0x100801001) { // branch-flip
    if (a2) { // branch-flip
      if (v14 != 'n')
        return NULL;
      v16 = sub_129a8(a2,a3);
    }
    else {
      v16 = sub_d577(a4,(int)v14);
      if (v16 <= -1)
        return NULL;
    }
    if (v12) {
      v17 = v16;
      v18 = &v15[0xfffffffffffffffe - (long)v11];
      v19 = 1;
      if (0xfa1 <= &v15[1 - (long)v11]) { // branch-flip
        v2 = (char *)sub_1cf49(&v15[1 - (long)v11]);
        v7 = v4;
      }
      else {
        v3 = ((unsigned long)&v15[0x37 - (long)v11] / 0x10) * 0x10;
        for (; v6 != &v4[-(v3 & 0xfffffffffffff000)]; v6 = &v6[-0x1000]) {
          *(unsigned long *)&v6[-8] = *(unsigned long *)&v6[-8];
        }
        v1 = -(unsigned long)((unsigned int)v3 & 0xfff);
        v7 = &v6[v1];
        if (v3 & 0xfff)
          *(unsigned long *)&v6[((unsigned long)((unsigned int)v3 & 0xfff) - 8) + v1] = *(unsigned long *)&v6[((unsigned long)((unsigned int)v3 & 0xfff) - 8) + v1];
        v2 = (char *)(((unsigned long)&v6[v1 + 0xf] & 0xfffffffffffffff0) + 0x1f & 0xffffffffffffffe0);
      }
      v20 = v2;
      v21 = &v2[(long)&v18[v19]];
      *(unsigned long *)&v7[-8] = 0xdce7;
      memcpy(v2,v11,(unsigned long)v18,v7[-8]);
      *(unsigned long *)&v7[-8] = 0xdd0c;
      memcpy(&v20[(long)v18],"l",v19,v7[-8]);
      v2 = v21;
      v21 = &v21[1];
      *v2 = v13;
      *v21 = '\0';
      *(unsigned long *)&v7[-8] = 0xdd41;
      fprintf(v12,v20,v17);
      *(unsigned long *)&v7[-8] = 0xdd4d;
      sub_168f3(v20);
    }
  }
  else {
    if (!(v3 & 0x800))
      return NULL;
    if (v14 != '\'')
      return NULL;
    v15 = (char *)sub_dd74(v15,&v5);
    if (!v15)
      return NULL;
    if (v12)
      putc_unlocked((int)v5,v12);
  }
  return v15;
}


// Function: sub_dd74 @ 0xdd74
char * sub_dd74(char *a0,char *a1) // early-return x4
{
  unsigned int v1; // eax
  char *v2;
  char *v3;
  char v4; // stack - 0x1e
  
  v4 = *a0;
  if (v4 != '\\') { // branch-flip
    if ((v4 <= '\\') && ((!v4 || (v4 == '\''))))
      return NULL;
    v3 = &a0[2];
    if (a0[1] != '\'')
      return NULL;
  }
  else {
    v4 = '\0';
    v2 = &a0[1];
    while( true ) {
      v3 = &v2[1];
      if (*v2 == '\'') break;
      v1 = (int)*v2 - 0x30;
      if (8 <= v1)
        return NULL;
      v4 = (char)v1 + v4 * '\b';
      v2 = v3;
    }
    if (((long)&v3[0xfffffffffffffffe - (long)a0] <= 0) || (4 <= (long)&v3[0xfffffffffffffffe - (long)a0]))
      return NULL;
  }
  *a1 = v4;
  return v3;
}


// Function: sub_de51 @ 0xde51
void sub_de51(unsigned int *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  if ((a1) && (*(char *)((long)a0 + 0x121) != '\x01')) {
    v1 = sub_1d1d3(*a0,*(long *)&a0[0x28] + *(long *)&a0[0x2c],a1);
    if (v1 == 0xffffffffffffffff)
      sub_10925(*(unsigned long *)&a0[2]); // no-return
    *(unsigned long *)&a0[0x2c] = *(long *)&a0[0x2c] + v1;
    *(bool *)((long)a0 + 0x121) = v1 < a1;
  }
}


// Function: sub_df00 @ 0xdf00
unsigned long sub_df00(int *a0,char a1)
{
  int v1;
  unsigned long v2;
  unsigned long v3;
  int v4; // eax
  void *v5; // rax
  
  if (0 <= *a0) { // branch-flip
    v2 = *(unsigned long *)&a0[0x12];
    *(unsigned long *)&a0[0x2a] = sub_1d2a2(8,v2,0x7fffffffffffffef);
    v2 = *(unsigned long *)&a0[0x2a];
    *(unsigned long *)&a0[0x28] = sub_1c945(v2);
    if (a1 != '\x01') {
      v4 = sub_131e6(*a0,0);
      sub_de51(a0,*(unsigned long *)&a0[0x2a]);
      v3 = *(unsigned long *)&a0[0x2c];
      if (v4) {
        v1 = *a0;
        if ((long)lseek(v1,-v3,1) <= -1)
          sub_10925(*(unsigned long *)&a0[2]); // no-return
        sub_131e6(*a0,v4);
        a0[0x2c] = 0;
        a0[0x2d] = 0;
        *(char *)((long)a0 + 0x121) = 0;
      }
      v5 = memchr(*(void **)&a0[0x28],0,v3);
      return CONCAT71((undefined7)((unsigned long)v5 >> 8),v5 != NULL);
    }
  }
  else {
    a0[0x2a] = 8;
    a0[0x2b] = 0;
    v2 = *(unsigned long *)&a0[0x2a];
    *(unsigned long *)&a0[0x28] = sub_1c945(v2);
  }
  a0[0x2c] = 0;
  a0[0x2d] = 0;
  *(char *)((long)a0 + 0x121) = 0;
  return 0;
}


// Function: sub_e09d @ 0xe09d
void sub_e09d(int *a0) // return-dupe x3
{
  unsigned long v1;
  unsigned long v2;
  unsigned long v3;
  unsigned long v4; // rax
  
  if (*a0 < 0)
    return;
  if ((a0[10] & 0xf000U) == 0x8000) {
    v1 = *(unsigned long *)&a0[0x10];
    v4 = (v1 & 0xfffffffffffffff8) + 0x10;
    if (((*(unsigned long *)&a0[0x10] != v1) || (v4 < v1)) || (0x7fffffffffffffff <= v4))
      sub_1cf09(); // no-return
    if (*(unsigned long *)&a0[0x2a] < v4) {
      *(unsigned long *)&a0[0x2a] = v4;
      v2 = *(unsigned long *)&a0[0x28];
      *(unsigned long *)&a0[0x28] = sub_1c991(v2,v4);
    }
    if ((*(unsigned long *)&a0[0x2c] <= v1) && (sub_de51(a0,(v1 - *(long *)&a0[0x2c]) + 1), *(unsigned long *)&a0[0x2c] <= v1))
      return;
  }
  sub_de51(a0,*(long *)&a0[0x2a] - *(long *)&a0[0x2c]);
  if (!*(long *)&a0[0x2c])
    return;
  while( true ) {
    if (*(long *)&a0[0x2c] != *(long *)&a0[0x2a]) {
      *(unsigned long *)&a0[0x2a] = *(long *)&a0[0x2c] + 0x10U & 0xfffffffffffffff8;
      v2 = *(unsigned long *)&a0[0x2a];
      v3 = *(unsigned long *)&a0[0x28];
      *(unsigned long *)&a0[0x28] = sub_1c991(v3,v2);
      return;
    }
    if (0x3ffffffffffffff8 <= *(unsigned long *)&a0[0x2a]) break;
    *(long *)&a0[0x2a] = *(long *)&a0[0x2a] * 2;
    v2 = *(unsigned long *)&a0[0x2a];
    v3 = *(unsigned long *)&a0[0x28];
    *(unsigned long *)&a0[0x28] = sub_1c991(v3,v2);
    sub_de51(a0,*(long *)&a0[0x2a] - *(long *)&a0[0x2c]);
  }
  sub_1cf09(); // no-return
}


// Function: sub_e2f8 @ 0xe2f8
void sub_e2f8(long a0) // ternary x3
{
  unsigned char v1;
  unsigned char *v10; // rax
  unsigned long v11;
  unsigned char *v12;
  long *v13; // rax
  bool v14; // zf
  unsigned char v15; // stack - 0xb1
  unsigned char *v16; // stack - 0xa8
  unsigned long v17; // stack - 0xa0
  unsigned long *v18; // stack - 0x98
  long v19; // stack - 0x90
  long v2;
  long v20; // stack - 0x88
  long v21; // stack - 0x80
  long v22; // stack - 0x78
  long v23; // stack - 0x70
  unsigned long v24; // stack - 0x68
  unsigned long v25; // stack - 0x60
  unsigned long v26; // stack - 0x58
  unsigned long v27; // stack - 0x50
  unsigned char *v28; // stack - 0x48
  long v29; // stack - 0x40
  unsigned char *v3;
  unsigned long v4;
  void *v5;
  bool v6;
  unsigned int v7;
  char v8;
  unsigned char *v9;
  
  v16 = *(unsigned char **)(a0 + 0xe0);
  v19 = *(long *)(a0 + 0xb8);
  v20 = *(long *)(a0 + 0xd8);
  v21 = 0;
  v2 = *(long *)(a0 + 0xc0);
  v22 = sub_1c945(v20 << 3);
  v8 = dat_2d1ee;
  v7 = dat_2d1e8;
  v23 = dat_2d768;
  v24 = dat_2d770;
  v25 = dat_2d778;
  v3 = *(unsigned char **)(a0 + 0xf0);
  v10 = (unsigned char *)(*(long *)(a0 + 0xb0) + *(long *)(a0 + 0xa0));
  v14 = dat_2d1e8 != 0;
  v6 = dat_2d1ee != '\0';
  while (v9 = v16, v16 < v3) {
    v26 = 0;
    if (v7 == 5) {
      while( true ) {
        v12 = &v16[1];
        v1 = *v16;
        v16 = v12;
        if (v1 == 10) break;
        if (!(*(unsigned short *)((unsigned long)v1 * 2 + *(long *)__ctype_b_loc()) & 0x2000)) {
          v11 = (v8) ? (unsigned long)tolower((unsigned int)v1) : (unsigned long)v1; // branch-flip
          v26 = v11 + (v26 << 7 | v26 >> 0x39);
        }
      }
      goto label_e811;
    }
    if (6 <= v7) { // branch-flip
label_e783:
      if (v8) { // branch-flip
        while( true ) {
          v12 = &v16[1];
          v1 = *v16;
          v16 = v12;
          if (v1 == 10) break;
          v26 = (v26 << 7 | v26 >> 0x39) + (long)tolower((unsigned int)v1);
        }
      }
      else {
        while( true ) {
          v12 = &v16[1];
          v1 = *v16;
          v16 = v12;
          if (v1 == 10) break;
          v26 = (v26 << 7 | v26 >> 0x39) + (unsigned long)v1;
        }
      }
    }
    else if (4 <= v7) { // branch-flip
      if (v7 != 4) goto label_e783;
      while( true ) {
        v12 = &v16[1];
        v15 = *v16;
        v16 = v12;
        if (v15 == 10) break;
        if (*(unsigned short *)((unsigned long)v15 * 2 + *(long *)__ctype_b_loc()) & 0x2000) {
          do {
            v12 = &v16[1];
            v15 = *v16;
            v16 = v12;
            if (v15 == 10) goto label_e811;
          } while (*(unsigned short *)((unsigned long)v15 * 2 + *(long *)__ctype_b_loc()) & 0x2000);
          v26 = (v26 << 7 | v26 >> 0x39) + 0x20;
        }
        v11 = (v8) ? (unsigned long)tolower((unsigned int)v15) : (unsigned long)v15; // branch-flip
        v26 = v11 + (v26 << 7 | v26 >> 0x39);
      }
    }
    else {
      if (!v7) goto label_e783;
      v27 = 0;
      while( true ) {
        v12 = &v16[1];
        v15 = *v16;
        v16 = v12;
        if (v15 == 10) break;
        if ((v7 & 2) && (*(unsigned short *)((unsigned long)v15 * 2 + *(long *)__ctype_b_loc()) & 0x2000)) {
          v28 = v12;
          do {
            v16 = &v28[1];
            v1 = *v28;
            if (v1 == 10) {
              goto label_e811;
            }
            v28 = v16;
          } while (*(unsigned short *)((unsigned long)v1 * 2 + *(long *)__ctype_b_loc()) & 0x2000);
        }
        v29 = 1;
        if (v7 & 1) {
          if (v15 != 0xd) { // branch-flip
            if (0xe <= v15) { // branch-flip
label_e711:
              v27 += 1;
            }
            else if (v15 != 8) { // branch-flip
              if (v15 != 9) goto label_e711;
              v15 = 0x20;
              v29 = dat_2d2a8 - v27 % dat_2d2a8;
              v27 = (v27 <= v29 + v27) ? v29 + v27 : 0; // branch-flip
            }
            else {
              v27 -= v27 != 0;
            }
          }
          else {
            v27 = 0;
          }
        }
        if (v8)
          v15 = (unsigned char)tolower((unsigned int)v15);
        do {
          v26 = (v26 << 7 | v26 >> 0x39) + (unsigned long)v15;
          v29 -= 1;
          v16 = v12;
        } while (v29);
      }
    }
label_e811:
    v18 = (unsigned long *)((v26 % dat_2d760) * 8 + dat_2d758);
    v12 = &v16[0xffffffffffffffff - (long)v9];
    if ((((v16 == v10) && (*(char *)(a0 + 0x120))) && (dat_2d1c0 != 4)) && ((dat_2d1c0 != 5 && (v7 <= 1))))
      v18 = (unsigned long *)(dat_2d758 + -8);
    for (v17 = *v18; v17; v17 = *(unsigned long *)(v23 + v17 * 0x20)) {
      if (v26 == *(unsigned long *)(v23 + v17 * 0x20 + 8)) {
        v5 = *(void **)(v23 + v17 * 0x20 + 0x10);
        if (v12 != *(unsigned char **)(v23 + v17 * 0x20 + 0x18)) { // branch-flip
          if (v14) {
label_ea3f:
            if (sub_11f90(v5,v9) != '\x01') goto label_ea84;
          }
        }
        else {
          if (!memcmp(v5,v9,(unsigned long)v12)) goto label_ea84;
          if (v14 || v6) goto label_ea3f;
        }
      }
    }
    v11 = v24 + 1;
    v17 = v24;
    if (v24 == v25) {
      if (0x1ffffffffffffff <= v25)
        sub_1cf09(); // no-return
      v4 = v25 << 1;
      v23 = sub_1c991(v23,v25 << 6);
      v25 = v4;
    }
    *(unsigned long *)(v24 * 0x20 + v23) = *v18;
    *(unsigned long *)(v24 * 0x20 + v23 + 8) = v26;
    *(unsigned char **)(v24 * 0x20 + v23 + 0x10) = v9;
    *(unsigned char **)(v24 * 0x20 + v23 + 0x18) = v12;
    *v18 = v24;
    v24 = v11;
label_ea84:
    if (v21 == v20) {
      if (((0x2aaaaaaaaaaaaaa9 < v20) || (0xffffffffffffffe < (unsigned long)(v20 * 2 - v2))) || (0xfffffffffffffff <= (unsigned long)(v20 - v2)))
        sub_1cf09(); // no-return
      v20 = v20 * 2 - v2;
      v22 = sub_1c991(v22,v20 * 8);
      v19 = sub_1c991(v19 + v2 * 8,(v20 - v2) * 8) + v2 * -8;
    }
    *(unsigned char **)(v21 * 8 + v19) = v9;
    *(unsigned long *)(v21 * 8 + v22) = v17;
    v21 += 1;
  }
  *(long *)(a0 + 200) = v21;
  v17 = 0;
  while( true ) {
    if (v21 == v20) {
      if (((0x2aaaaaaaaaaaaaa9 < v20) || (0xffffffffffffffe < (unsigned long)(v20 * 2 - v2))) || (0xfffffffffffffff <= (unsigned long)(v20 - v2)))
        sub_1cf09(); // no-return
      v20 = v20 * 2 - v2;
      v19 = sub_1c991(v19 + v2 * 8,(v20 - v2) * 8) + v2 * -8;
    }
    *(unsigned char **)(v21 * 8 + v19) = v16;
    if (v16 == v10) break;
    if ((dat_2d1d0 <= (long)v17) && (dat_2d1c8)) goto label_ed3e;
    v21 += 1;
    do {
      v3 = &v16[1];
      v1 = *v16;
      v16 = v3;
    } while (v1 != 10);
    v17 += 1;
  }
  if (((*(char *)(a0 + 0x120)) && (dat_2d1c0 != 4)) && (dat_2d1c0 != 5)) {
    v13 = (long *)(v19 + v21 * 8);
    *v13 = *v13 + -1;
  }
label_ed3e:
  *(long *)(a0 + 0xb8) = v19;
  *(long *)(a0 + 0xd0) = v21;
  *(long *)(a0 + 0xd8) = v20;
  *(long *)(a0 + 0xf8) = v22;
  dat_2d768 = v23;
  dat_2d778 = v25;
  dat_2d770 = v24;
  return;
}


// Function: sub_edad @ 0xedad
void sub_edad(long a0) // return-dupe, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_edad
{
  char *v1;
  long v2;
  void *v3;
  int v4; // eax
  char *v5; // rax
  long v6; // stack - 0x30
  char *v7; // stack - 0x28
  char *v8; // stack - 0x20
  
  v6 = *(long *)(a0 + 0xb0);
  v3 = *(void **)(a0 + 0xa0);
  if (!v3)
    return;
  if (dat_2d2b2) {
    v5 = (char *)(v6 + (long)v3);
    *v5 = '\r';
    v7 = rawmemchr(v3,0xd);
    v8 = v7;
    while (v8 != v5) {
      v4 = !((*v8 != '\r') || (v8[1] != '\n'));
      v1 = &v7[1];
      *v7 = v8[v4];
      v8 = &(&v8[v4])[1];
      v7 = v1;
    }
    v6 -= (long)v5 - (long)v7;
  }
  if ((v6) && (*(char *)((long)v3 + v6 + -1) != '\n')) {
    v2 = v6 + 1;
    *(char *)(v6 + (long)v3) = 10;
    *(char *)(a0 + 0x120) = 1;
    v6 = v2;
  }
  memset((void *)(v6 + (long)v3),0,8);
  *(long *)(a0 + 0xb0) = v6;
}


// Function: sub_eef5 @ 0xeef5
long sub_eef5(long a0,unsigned long a1,unsigned long a2) // ternary
{
  a1 = (10 <= a0) ? a1 / (unsigned long)(a0 - 1U) : 0x20; // branch-flip
  a2 /= a1;
  if (!a2)
    a2 = 1;
  if (0x787878787878783 <= a2)
    a2 = 0x787878787878782;
  return a2 + 5;
}


// Function: sub_ef6a @ 0xef6a
void sub_ef6a(int *a0) // ternary, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_ef6a
{
  long *v1;
  unsigned long v10;
  unsigned long v11;
  bool v12; // al
  long *v13; // stack - 0xb8
  long *v14; // stack - 0xb0
  long *v15; // stack - 0xa8
  long *v16; // stack - 0xa0
  long *v17; // stack - 0x98
  long v18; // stack - 0x90
  long v19; // stack - 0x88
  long *v2;
  unsigned long v20; // stack - 0x80
  unsigned long v21; // stack - 0x78
  long v22; // stack - 0x70
  long *v3;
  unsigned long v4;
  long v5;
  long *v6;
  int v7; // eax
  long *v8;
  long v9;
  
  sub_e09d(a0);
  sub_edad(a0);
  if (*a0 != a0[0x4c]) { // branch-flip
    sub_e09d(&a0[0x4c]);
    sub_edad(&a0[0x4c]);
  }
  else {
    *(unsigned long *)&a0[0x74] = *(unsigned long *)&a0[0x28];
    *(unsigned long *)&a0[0x76] = *(unsigned long *)&a0[0x2a];
    *(unsigned long *)&a0[0x78] = *(unsigned long *)&a0[0x2c];
    *(char *)&a0[0x94] = (char)a0[0x48];
  }
  v2 = *(long **)&a0[0x28];
  v3 = *(long **)&a0[0x74];
  v10 = *(unsigned long *)&a0[0x2c];
  v11 = *(unsigned long *)&a0[0x78];
  if (v2 != v3) { // branch-flip
    v13 = v2;
    v14 = v3;
    if (v11 <= v10) // branch-flip
      *(unsigned char *)(v11 + (long)v3) = ~*(unsigned char *)(v11 + (long)v2);
    else {
      *(unsigned char *)(v10 + (long)v2) = ~*(unsigned char *)(v10 + (long)v3);
    }
    while (*v13 == *v14) {
      v14 = &v14[1];
      v13 = &v13[1];
    }
    v16 = v14;
    for (v15 = v13; (char)*v15 == (char)*v16; v15 = (long *)((long)v15 + 1)) {
      v16 = (long *)((long)v16 + 1);
    }
    if (((dat_2d1c0 != 4) && (dat_2d1c0 != 5)) && ((long *)((long)v3 + (v11 - *(unsigned char *)&a0[0x94])) < v16 != (long *)((long)v2 + (v10 - *(unsigned char *)&a0[0x48])) < v15)) {
      v15 = (long *)((long)v15 + -1);
      v16 = (long *)((long)v16 + -1);
    }
  }
  else {
    v15 = (long *)((long)v3 + v11);
    v16 = v15;
  }
  v19 = dat_2d1e0;
  while ((v15 != v2 && ((*(char *)((long)v15 + -1) != '\n' || (v9 = v19 + -1, v12 = v19 != 0, v19 = v9, v12))))) {
    v15 = (long *)((long)v15 + -1);
    v16 = (long *)((long)v16 + -1);
  }
  *(long **)&a0[0x38] = v15;
  *(long **)&a0[0x84] = v16;
  v8 = (long *)(v10 + (long)v2);
  v16 = (long *)(v11 + (long)v3);
  if (((dat_2d1c0 == 4) || (dat_2d1c0 == 5)) || (v15 = v8, (char)a0[0x48] == (char)a0[0x94])) {
    v9 = (v11 <= v10) ? v10 - v11 : 0; // branch-flip
    v17 = (long *)(v9 + *(long *)&a0[0x38]);
    v1 = v8;
    v6 = v16;
    do {
      v16 = v6;
      v15 = v1;
      if (v15 == v17) goto label_f3ed;
      v1 = (long *)((long)v15 + -1);
      v6 = (long *)((long)v16 + -1);
    } while (*(char *)(long *)((long)v15 + -1) == *(char *)(long *)((long)v16 + -1));
    v17 = v15;
label_f3ed:
    if (((v2 != v15) && (*(char *)((long)v15 + -1) != '\n')) || ((v3 != v16 && (*(char *)((long)v16 + -1) != '\n'))))
      v7 = 1;
    else {
      v7 = 0;
    }
    v19 = dat_2d1e0 + v7;
    while ((v9 = v19 + -1, v19 && (v15 != v8))) {
      do {
        v1 = (long *)((long)v15 + 1);
        v5 = *v15;
        v15 = v1;
        v19 = v9;
      } while ((char)v5 != '\n');
    }
    v16 = (long *)((long)v16 + ((long)v15 - (long)v17));
  }
  *(long **)&a0[0x3c] = v15;
  *(long **)&a0[0x88] = v16;
  if ((((dat_2d1c8) && (!dat_2d240)) && ((long)dat_2d1d0 <= 0x1ffffffffffffffe)) && (dat_2d1d0 < v10)) {
    v9 = sub_eef5(0,0,(long)v15 - *(long *)&a0[0x38]);
    v21 = sub_eef5(0,0,(char *)((long)v2 + (v10 - (long)v15)));
    for (v22 = 1; v22 <= (long)dat_2d1d0; v22 = v22 << 1) {
    }
    if ((long)dat_2d1d0 <= (long)v21)
      v21 = dat_2d1d0;
    v21 += v22 + v9;
  }
  else {
    v22 = 0;
    v21 = sub_eef5(0,0,v10);
  }
  v20 = 0;
  v18 = sub_1c945(v21 << 3);
  v12 = (((dat_2d1c8 != '\x01') || (v15 != *(long **)&a0[0x38])) || (v16 != *(long **)&a0[0x84]));
  v15 = v2;
  if (v12) {
    v8 = *(long **)&a0[0x38];
    while (v15 != v8) {
      v10 = v20 + 1;
      v20 &= v22 - 1U;
      if (v20 == v21) {
        if (0x7ffffffffffffff <= v21)
          sub_1cf09(); // no-return
        v4 = v21 * 2;
        v18 = sub_1c991(v18,v21 << 4);
        v21 = v4;
      }
      *(long **)(v20 * 8 + v18) = v15;
      do {
        v1 = (long *)((long)v15 + 1);
        v9 = *v15;
        v15 = v1;
        v20 = v10;
      } while ((char)v9 != '\n');
    }
  }
  if ((v22) && ((long)dat_2d1d0 < (long)v20))
    v10 = dat_2d1d0;
  else {
    v10 = v20;
  }
  v9 = sub_eef5(v20,(long)v15 - (long)v2,(long)v16 - *(long *)&a0[0x84]);
  v11 = sub_eef5(v20,(long)v15 - (long)v2,(char *)((long)v3 + (v11 - (long)v16)));
  if ((long)dat_2d1d0 <= (long)v11)
    v11 = dat_2d1d0;
  v11 += v10 + v9;
  if (((long)v10 <= (long)v11) && (v11 <= 0xffffffffffffffe)) {
    v9 = sub_1c945(v11 * 8);
    if (v10 != v20) {
      for (v19 = 0; v19 < (long)v10; v19 = v19 + 1) {
        *(unsigned long *)(v9 + v19 * 8) = *(unsigned long *)(v18 + (v19 + (v20 - dat_2d1d0) & v22 - 1U) * 8);
      }
      for (v19 = 0; v19 < (long)v10; v19 = v19 + 1) {
        *(unsigned long *)(v18 + v19 * 8) = *(unsigned long *)(v9 + v19 * 8);
      }
    }
    for (v19 = 0; v19 < (long)v10; v19 = v19 + 1) {
      *(char **)(v9 + v19 * 8) = (char *)((long)v3 + (*(long *)(v18 + v19 * 8) - (long)v2));
    }
    *(unsigned long *)&a0[0x2e] = v10 * 8 + v18;
    *(unsigned long *)&a0[0x7a] = v9 + v10 * 8;
    *(unsigned long *)&a0[0x7c] = -v10;
    *(unsigned long *)&a0[0x30] = *(unsigned long *)&a0[0x7c];
    *(unsigned long *)&a0[0x36] = v21 - v10;
    *(unsigned long *)&a0[0x82] = v11 - v10;
    *(unsigned long *)&a0[0x86] = v20;
    *(unsigned long *)&a0[0x3a] = *(unsigned long *)&a0[0x86];
    return;
  }
  sub_1cf09(); // no-return
}


// Function: sub_fa13 @ 0xfa13
unsigned long sub_fa13(int *a0,char a1) // return-dupe
{
  bool v1;
  bool v2;
  long v3; // rax
  bool v4;
  int v5; // stack - 0xc
  
  v1 = dat_2d1d8 != '\0';
  v2 = sub_df00(a0,v1 || a1) != '\0';
  v4 = v2 || a1;
  if (*a0 != a0[0x4c]) // branch-flip
    v4 = sub_df00(&a0[0x4c],a1 || (v1 || v2)) || v4;
  else {
    *(unsigned long *)&a0[0x74] = *(unsigned long *)&a0[0x28];
    *(unsigned long *)&a0[0x76] = *(unsigned long *)&a0[0x2a];
    *(unsigned long *)&a0[0x78] = *(unsigned long *)&a0[0x2c];
  }
  if (v4) {
    sub_131e6(*a0,0);
    sub_131e6(a0[0x4c],0);
    return 1;
  }
  sub_ef6a(a0);
  dat_2d778 = *(long *)&a0[0x82] + *(long *)&a0[0x36] + 1;
  if (0x3ffffffffffffff <= dat_2d778)
    sub_1cf09(); // no-return
  dat_2d768 = (void *)sub_1c945(dat_2d778 * 0x20);
  dat_2d770 = 1;
  for (v5 = 9; (unsigned long)(1L << ((unsigned char)v5 & 0x3f)) < (unsigned long)((long)dat_2d778 / 3); v5 = v5 + 1) {
  }
  dat_2d760 = (1L << ((unsigned char)v5 & 0x3f)) - (unsigned long)*(unsigned char *)((long)v5 + 0x25240);
  if (dat_2d760 <= 0xffffffffffffffe) {
    v3 = dat_2d760 + 1;
    dat_2d758 = sub_13030(v3 * 8) + 8;
    for (v5 = 0; v5 <= 1; v5 = v5 + 1) {
      sub_e2f8(&a0[(long)v5 * 0x4c]);
    }
    *(unsigned long *)&a0[0x96] = dat_2d770;
    *(unsigned long *)&a0[0x4a] = *(unsigned long *)&a0[0x96];
    free(dat_2d768);
    free((void *)(dat_2d758 + -8));
    return 0;
  }
  sub_1cf09(); // no-return
}


// Function: sub_fcfd @ 0xfcfd
void sub_fcfd(unsigned long a0)
{
  sub_1245b(a0,sub_12437,sub_fd2d);
}


// Function: sub_fd2d @ 0xfd2d
void sub_fd2d(unsigned long a0) // return-dupe x2
{
  unsigned int v1; // eax
  long v2;
  long v3; // stack - 0x28
  long v4; // stack - 0x30
  long v5; // stack - 0x38
  long v6; // stack - 0x40
  
  v1 = sub_12af4(a0,&v6,&v5,&v4,&v3);
  if (!v1)
    return;
  sub_11b50();
  sub_128c2(4);
  sub_12a37(0x2c,0x2d360,v6,v5);
  fputc_unlocked((int)*(char *)((unsigned long)v1 + 0x25780),dat_2d5c0);
  sub_12a37(0x2c,0x2d490,v4,v3);
  sub_128c2(3);
  fputc_unlocked(10,dat_2d5c0);
  if (v1 & 1) {
    for (v2 = v6; v2 <= v5; v2 = v2 + 1) {
      sub_128c2(2);
      sub_124fe("<",dat_2d418 + v2 * 8,1);
      sub_128c2(3);
      if (*(char *)(*(long *)((v2 + 1) * 8 + dat_2d418) + -1) == '\n')
        putc_unlocked(10,dat_2d5c0);
    }
  }
  if (v1 == 3)
    fputs_unlocked("---\n",dat_2d5c0);
  if (!(v1 & 2))
    return;
  for (v2 = v4; v2 <= v3; v2 = v2 + 1) {
    sub_128c2(1);
    sub_124fe(">",dat_2d548 + v2 * 8,1);
    sub_128c2(3);
    if (*(char *)(*(long *)((v2 + 1) * 8 + dat_2d548) + -1) == '\n')
      putc_unlocked(10,dat_2d5c0);
  }
}


// Function: sub_ff70 @ 0xff70
void sub_ff70(unsigned long a0)
{
  sub_11b50();
  dat_2d780 = -dat_2d448;
  dat_2d788 = dat_2d780;
  sub_1245b(a0,sub_12437,sub_106e7);
  sub_1055a(dat_2d430,dat_2d560);
}


// Function: sub_ffdd @ 0xffdd
unsigned long sub_ffdd(unsigned long a0,unsigned long a1)
{
  unsigned long v1;
  void *v2;
  unsigned long v3; // stack - 0x30
  unsigned long v4; // stack - 0x20
  
  v2 = dat_2d5c0;
  v1 = dat_2d2a8;
  v3 = a0;
  if (dat_2d2a1 != '\x01') {
    v3 = a0;
    for (v4 = (a0 + dat_2d2a8) - a0 % dat_2d2a8; v4 <= a1; v4 = v4 + v1) {
      putc_unlocked(9,v2);
      v3 = v4;
    }
  }
  while (v1 = v3 + 1, v3 < a1) {
    putc_unlocked(0x20,v2);
    v3 = v1;
  }
  return a1;
}


// Function: sub_10090 @ 0x10090
unsigned long sub_10090(unsigned long *a0,unsigned long a1,unsigned long a2)
{
  char *v1;
  unsigned long v10;
  int v11; // stack - 0x74
  unsigned long v12; // stack - 0x70
  void *v13; // stack - 0x68
  char *v14; // stack - 0x60
  long v15; // stack - 0x58
  unsigned long v16; // stack - 0x50
  char v2;
  char *v3;
  bool v4;
  int v5; // eax
  unsigned long v6;
  unsigned long v7;
  unsigned long v8; // stack - 0x48
  int v9; // stack - 0x78
  
  v13 = dat_2d5c0;
  v3 = (char *)a0[1];
  v8 = 0;
  v7 = 0;
  v10 = 0;
  v1 = (char *)*a0;
  do {
    while( true ) {
      v14 = v1;
      if (v3 <= v14)
        return v10;
      v1 = &v14[1];
      v2 = *v14;
      v5 = (int)v2;
      if (v5 <= 0x5f) break;
      if (0x1e <= (unsigned int)(v5 - 0x61U)) { // branch-flip
label_10288:
        v16 = sub_1d369(&v9,v14,(long)v3 - (long)v14,&v8);
        if ((v16) && (v16 <= 0xfffffffffffffffd)) {
          v11 = wcwidth(v9);
          if (1 <= v11)
            v7 += (long)v11;
          if (v7 <= a2) {
            fwrite_unlocked(v14,1,v16,stdout);
            v10 = v7;
          }
          v1 = &v14[v16];
        }
        else {
label_10310:
          if (v7 < a2)
            putc_unlocked((int)v2,v13);
        }
      }
      else {
label_1032d:
        v6 = v7 + 1;
        v4 = v7 < a2;
        v7 = v6;
        if (v4) {
          putc_unlocked((int)v2,v13);
          v10 = v6;
        }
      }
    }
    if (v5 <= 7) goto label_10288;
    switch(v5) {
      case 8:
        if ((v7) && (v7 = v7 - 1, v7 < a2)) {
          if (v10 <= v7) { // branch-flip
            for (; v10 < v7; v10 = v10 + 1) {
              putc_unlocked(0x20,v13);
            }
          }
          else {
            putc_unlocked((int)v2,v13);
            v10 = v7;
          }
        }
        break;
      case 9:
        v15 = dat_2d2a8 - v7 % dat_2d2a8;
        if (v7 == v10) {
          v6 = v15 + v10;
          v12 = v6;
          if (dat_2d2a1) { // branch-flip
            if (a2 < v6)
              v12 = a2;
            for (; v10 < v12; v10 = v10 + 1) {
              putc_unlocked(0x20,v13);
            }
          }
          else if (v6 < a2) {
            putc_unlocked((int)v2,v13);
            v10 = v6;
          }
        }
        v7 = v15 + v7;
        break;
      case 10:
        return v10;
      case 0xb:
      case 0xc:
        goto label_10310;
      case 0xd:
        putc_unlocked((int)v2,v13);
        sub_ffdd(0,a1);
        v7 = 0;
        v10 = 0;
        break;
      default:
        goto label_10288;
      case 0x20:
      case 0x21:
      case 0x22:
      case 0x23:
      case 0x25:
      case 0x26:
      case 0x27:
      case 0x28:
      case 0x29:
      case 0x2a:
      case 0x2b:
      case 0x2c:
      case 0x2d:
      case 0x2e:
      case 0x2f:
      case 0x30:
      case 0x31:
      case 0x32:
      case 0x33:
      case 0x34:
      case 0x35:
      case 0x36:
      case 0x37:
      case 0x38:
      case 0x39:
      case 0x3a:
      case 0x3b:
      case 0x3c:
      case 0x3d:
      case 0x3e:
      case 0x3f:
      case 0x41:
      case 0x42:
      case 0x43:
      case 0x44:
      case 0x45:
      case 0x46:
      case 0x47:
      case 0x48:
      case 0x49:
      case 0x4a:
      case 0x4b:
      case 0x4c:
      case 0x4d:
      case 0x4e:
      case 0x4f:
      case 0x50:
      case 0x51:
      case 0x52:
      case 0x53:
      case 0x54:
      case 0x55:
      case 0x56:
      case 0x57:
      case 0x58:
      case 0x59:
      case 0x5a:
      case 0x5b:
      case 0x5c:
      case 0x5d:
      case 0x5e:
      case 0x5f:
        goto label_1032d;
      
    }
  } while( true );
}


// Function: sub_10388 @ 0x10388
void sub_10388(long a0,char a1,unsigned long *a2) // return-dupe, ternary
{
  bool v1;
  long v2;
  long v3;
  void *v4;
  char v5; // stack - 0x44
  bool v6; // stack - 0x2a
  long v7; // stack - 0x28
  
  v4 = dat_2d5c0;
  v3 = dat_2d328;
  v2 = dat_2d320;
  v7 = 0;
  v6 = 0;
  v1 = 0;
  if (a1 != '<') { // branch-flip
    if (a1 == '>') {
      sub_128c2(1);
      v1 = 1;
    }
  }
  else {
    sub_128c2(2);
    v1 = 1;
  }
  if (a0) {
    v6 = *(char *)(*(long *)(a0 + 8) + -1) == '\n';
    v7 = sub_10090(a0,0,v2);
  }
  if (a1 != ' ') {
    v7 = sub_ffdd(v7,(unsigned long)((v3 + v2) - 1U) >> 1) + 1;
    v5 = a1;
    if ((a1 == '|') && (v6 != (*(char *)(a2[1] + -1) == '\n'))) {
      v5 = (v6) ? '/' : '\\'; // branch-flip
    }
    putc_unlocked((int)v5,v4);
  }
  if ((a2) && (v6 = *(char *)(a2[1] + -1) == '\n' || v6, *(char *)*a2 != '\n'))
    sub_10090(a2,sub_ffdd(v7,v3),v2);
  if (v6)
    putc_unlocked(10,v4);
  if (v1) {
    sub_128c2(3);
    return;
  }
}


// Function: sub_1055a @ 0x1055a
void sub_1055a(long a0,long a1)
{
  long v1;
  long v2;
  long v3; // stack - 0x28
  long v4; // stack - 0x20
  
  v3 = dat_2d780;
  v4 = dat_2d788;
  if ((dat_2d31a != '\x01') && ((dat_2d780 != a0 || (dat_2d788 != a1)))) {
    if (dat_2d318)
      fprintf(dat_2d5c0,"i%ld,%ld\n",a0 - dat_2d780,a1 - dat_2d788);
    if (dat_2d319 != '\x01') {
      while ((v3 != a0 && (v4 != a1))) {
        v2 = v4 + 1;
        v1 = v3 + 1;
        sub_10388(v3 * 8 + dat_2d418,0x20,dat_2d548 + v4 * 8);
        v3 = v1;
        v4 = v2;
      }
      while (v4 != a1) {
        v2 = v4 + 1;
        sub_10388(0,0x29,v4 * 8 + dat_2d548);
        v4 = v2;
      }
    }
    while (v3 != a0) {
      v2 = v3 + 1;
      sub_10388(v3 * 8 + dat_2d418,0x28,0);
      v3 = v2;
    }
  }
  dat_2d780 = a0;
  dat_2d788 = a1;
}


// Function: sub_106e7 @ 0x106e7
void sub_106e7(unsigned long a0) // return-dupe x2, ternary
{
  int v1; // eax
  long v10; // stack - 0x28
  long v2;
  long v3; // stack - 0x38
  long v4; // stack - 0x40
  long v5; // stack - 0x48
  long v6; // stack - 0x50
  long v7;
  unsigned int v8; // stack - 0x54
  long v9; // stack - 0x30
  
  v8 = sub_12af4(a0,&v6,&v5,&v4,&v3);
  if (!v8)
    return;
  sub_1055a(v6,v4);
  if (dat_2d318) {
    v9 = (v5 - v6) + 1;
    v10 = (v3 - v4) + 1;
    fprintf(dat_2d5c0,"c%ld,%ld\n",v9,v10);
  }
  if (v8 == 3) {
    v2 = v6;
    for (v7 = v4; (v2 <= v5 && (v7 <= v3)); v7 = v7 + 1) {
      sub_10388(dat_2d418 + v2 * 8,0x7c,v7 * 8 + dat_2d548);
      v2 += 1;
    }
    v1 = (v7 <= v3) ? 2 : 0; // branch-flip
    v8 = v1 + (unsigned int)(v2 <= v5);
    dat_2d780 = v2;
    dat_2d788 = v7;
    v6 = v2;
    v4 = v7;
  }
  if (v8 & 2) {
    for (v2 = v4; v2 <= v3; v2 = v2 + 1) {
      sub_10388(0,0x3e,dat_2d548 + v2 * 8);
    }
    dat_2d788 = v2;
  }
  if (!(v8 & 1))
    return;
  for (v2 = v6; v2 <= v5; v2 = v2 + 1) {
    sub_10388(dat_2d418 + v2 * 8,0x3c,0);
  }
  dat_2d780 = v2;
}


// Function: sub_108ec @ 0x108ec
void sub_108ec(unsigned long a0)
{
  error(0,*__errno_location(),"%s",a0); // no-return
}


// Function: sub_10925 @ 0x10925
void sub_10925(unsigned long a0)
{
  int v1;
  
  v1 = *__errno_location();
  sub_10bca();
  error(2,v1,"%s",a0); // no-return
}


// Function: sub_10966 @ 0x10966
void sub_10966(char *a0)
{
  sub_10bca();
  error(2,0,"%s",gettext(a0)); // no-return
}


// Function: sub_109a8 @ 0x109a8
void sub_109a8(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_109e2(a0,a1,a2,0,0);
}


// Function: sub_109e2 @ 0x109e2
void sub_109e2(char *a0,unsigned long a1,unsigned long a2,long a3,long a4) // return-dupe
{
  char *v1;
  unsigned long *v2; // rax
  char *v3 [3]; // stack - 0x68
  unsigned long v4 [5];
  int v5; // stack - 0x84
  unsigned long *v6; // stack - 0x80
  long v7; // stack - 0x78
  long v8; // stack - 0x50
  long v9; // stack - 0x48
  
  if (!dat_2d2c0) {
    if (dat_2d318)
      putchar_unlocked(0x20);
    printf(gettext(a0),a1,a2,a3,a4);
    return;
  }
  v7 = 8;
  v3[0] = a0;
  v3[1] = (char *)a1;
  v3[2] = (char *)a2;
  if (!a3)
    a3 = 0x2540f;
  v8 = a3;
  if (!a4)
    a4 = 0x2540f;
  v9 = a4;
  for (v5 = 0; v5 <= 4; v5 = v5 + 1) {
    v1 = v3[v5];
    v4[v5] = strlen(v1) + 1;
    v7 += v4[v5];
  }
  v2 = (unsigned long *)sub_1c945(v7);
  v6 = &v2[1];
  for (v5 = 0; v5 <= 4; v5 = v5 + 1) {
    memcpy(v6,v3[v5],v4[v5]);
    v6 = (unsigned long *)((long)v6 + v4[v5]);
  }
  *dat_2d020 = v2;
  *v2 = 0;
  dat_2d020 = v2;
}


// Function: sub_10bca @ 0x10bca
void sub_10bca(void)
{
  unsigned long *v1;
  long v2;
  char *v3;
  long v4;
  long v5;
  long v6;
  long v7 [5]; // stack - 0x58
  int v8; // stack - 0x6c
  unsigned long *v9; // stack - 0x68
  
  v9 = dat_2d7a0;
  while (v9) {
    v1 = (unsigned long *)*v9;
    for (v8 = 0; v6 = v7[4], v5 = v7[3], v4 = v7[2], v2 = v7[1], v8 <= 3; v8 = v8 + 1) {
      v2 = v7[v8];
      v3 = (char *)v7[v8];
      v7[v8 + 1] = strlen(v3) + 1 + v2;
    }
    printf(gettext((char *)&v9[1]),v2,v4,v5,v6);
    free(v9);
    v9 = v1;
  }
}


// Function: sub_10ccc @ 0x10ccc
void sub_10ccc(unsigned int a0) // return-dupe
{
  if (dat_2d840)
    return;
  dat_2d840 = a0;
}


// Function: sub_10ced @ 0x10ced
void sub_10ced(void) // return-dupe
{
  if (dat_2d840)
    return;
  dat_2d844 += 1;
}


// Function: sub_10d14 @ 0x10d14
void sub_10d14(void)
{
  char v1 [136];
  int v2; // stack - 0xa0
  
  while ((dat_2d840 || (dat_2d844))) {
    sub_128c2(3);
    fflush_unlocked(stdout);
    sigprocmask(0,(void *)0x2d7c0,v1);
    v2 = dat_2d840;
    if (dat_2d844) { // branch-flip
      dat_2d844 -= 1;
      v2 = 0x13;
    }
    else {
      signal(dat_2d840,0);
    }
    raise(v2);
    sigprocmask(2,v1,NULL);
  }
}


// Function: sub_10e15 @ 0x10e15
void sub_10e15(void) // ternary
{
  unsigned int v1;
  unsigned long v10; // stack - 0x70
  unsigned long v11; // stack - 0x68
  unsigned long v12; // stack - 0x60
  unsigned long v13; // stack - 0x58
  unsigned long v14; // stack - 0x50
  unsigned long v15; // stack - 0x48
  unsigned long v16; // stack - 0x40
  unsigned long v17; // stack - 0x38
  unsigned long v18; // stack - 0x30
  unsigned long v19; // stack - 0x28
  void *v2; // stack - 0xa8
  unsigned int v20; // stack - 0x20
  int v3; // stack - 0xac
  unsigned long v4; // stack - 0xa0
  unsigned long v5; // stack - 0x98
  unsigned long v6; // stack - 0x90
  unsigned long v7; // stack - 0x88
  unsigned long v8; // stack - 0x80
  unsigned long v9; // stack - 0x78
  
  sigemptyset((void *)0x2d7c0);
  for (v3 = 0; v3 <= 0xb; v3 = v3 + 1) {
    sigaction(*(int *)((long)v3 * 4 + 0x257c0),NULL,&v2);
    if (v2 != (void *)0x1)
      sigaddset((void *)0x2d7c0,*(int *)((long)v3 * 4 + 0x257c0));
  }
  v4 = dat_2d7c0;
  v5 = dat_2d7c8;
  v6 = dat_2d7d0;
  v7 = dat_2d7d8;
  v8 = dat_2d7e0;
  v9 = dat_2d7e8;
  v10 = dat_2d7f0;
  v11 = dat_2d7f8;
  v12 = dat_2d800;
  v13 = dat_2d808;
  v14 = dat_2d810;
  v15 = dat_2d818;
  v16 = dat_2d820;
  v17 = dat_2d828;
  v18 = dat_2d830;
  v19 = dat_2d838;
  v20 = 0x10000000;
  for (v3 = 0; v3 <= 0xb; v3 = v3 + 1) {
    v1 = *(unsigned int *)((long)v3 * 4 + 0x257c0);
    if (sigismember(0x2d7c0,v1)) {
      v2 = (*(int *)((long)v3 * 4 + 0x257c0) != 0x14) ? sub_10ccc : sub_10ced; // branch-flip
      sigaction(*(int *)((long)v3 * 4 + 0x257c0),&v2,NULL);
    }
  }
}


// Function: sub_11052 @ 0x11052
unsigned long sub_11052(void)
{
  unsigned long v1; // rax
  
  v1 = (*(void *)((long)dat_25410 + 0x25410))(); // jump-as-call
  return v1;
}


// Function: sub_1140f @ 0x1140f
void sub_1140f(unsigned long a0)
{
  dat_2d868 = a0;
}


// Function: sub_11429 @ 0x11429
void sub_11429(void)
{
  char *v1;
  char v2 [3];
  long v3; // fs_offset
  unsigned long v4; // stack - 0x10
  
  v1 = dat_2d868;
  v4 = *(unsigned long *)(v3 + 0x28);
  if ((dat_2d868) && (*dat_2d868)) {
    strcpy(v2,"??");
    sub_1ced8(v1);
    (*(void *)((long)dat_255b4 + 0x255b0))(); // jump-as-call
    return;
  }
}


// Function: sub_117e9 @ 0x117e9
void sub_117e9(char a0) // return-dupe, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_117e9
{
  bool v1; // al
  int v2; // eax
  
  if (!dat_2d5c0)
    return;
  if (!dat_2d1c4)
    return;
  if (dat_2d5c8) { // branch-flip
label_11848:
    v1 = 1;
  }
  else {
    if (a0 != '\x01') {
      v2 = fileno(dat_2d5c0);
      if (isatty(v2)) goto label_11848;
    }
    v1 = 0;
  }
  dat_2d859 = ((dat_2d1c4 == 2) || ((dat_2d1c4 == 1 && (v1))));
  if (dat_2d859)
    sub_11429();
  if (!v1)
    return;
  sub_10e15();
}


// Function: sub_118ac @ 0x118ac
void sub_118ac(unsigned long a0,unsigned long a1,char a2)
{
  dat_2d848 = a0;
  dat_2d850 = a1;
  dat_2d858 = a2;
  dat_2d5c0 = 0;
}


// Function: sub_118ef @ 0x118ef
unsigned long sub_118ef(char a0) // early-return x9
{
  unsigned int v1; // eax
  unsigned long v2; // rax
  
  v1 = (unsigned int)a0;
  v2 = (unsigned long)v1;
  if (0x23 <= (int)v1) { // branch-flip
    if (v1 == 0x5c)
      return 0x5c;
  }
  else if (7 <= (int)v1) {
    v1 -= 7;
    v2 = (unsigned long)v1;
    if (v1 <= 0x1b) {
      v2 = (long)*(int *)((unsigned long)v1 * 4 + 0x255cc) + 0x255cc;
      switch(v1) {
        case 0:
          return 0x61;
        case 1:
          return 0x62;
        case 2:
          return 0x74;
        case 3:
          return 0x6e;
        case 4:
          return 0x76;
        case 5:
          return 0x66;
        case 6:
          return 0x72;
        case 0x1b:
          return 0x22;
        
      }
    }
  }
  return CONCAT71((undefined7)(v2 >> 8),a0 <= '\x1f');
}


// Function: sub_11984 @ 0x11984
unsigned char * sub_11984(unsigned char *a0) // ternary
{
  unsigned char *v1;
  unsigned char v2;
  bool v3;
  char v4; // al
  unsigned char v5; // al
  unsigned char *v6; // rax
  unsigned char *v7; // stack - 0x30
  long v8; // stack - 0x28
  unsigned char *v9; // stack - 0x20
  
  v8 = 0;
  v3 = 0;
  for (v7 = a0; *v7; v7 = &v7[1]) {
    if (*v7 != 0x20) { // branch-flip
      v4 = sub_118ef((int)(char)*v7);
      if (!v4) goto label_119ec;
      v8 = (v4 != '\x01') ? v8 + 1 : v8 + 3; // branch-flip
    }
    else {
      v3 = 1;
    }
label_119ec:
  }
  if ((v3) || (v8)) {
    v6 = (unsigned char *)sub_1c945(&v7[(v8 - (long)a0) + 3]);
    v9 = &v6[1];
    *v6 = 0x22;
    for (v7 = a0; *v7; v7 = &v7[1]) {
      v2 = *v7;
      v5 = sub_118ef((int)(char)v2);
      if (v5) { // branch-flip
        if (v5 != 1) { // branch-flip
          *v9 = 0x5c;
          v1 = &v9[2];
          v9[1] = v5;
          v9 = v1;
        }
        else {
          *v9 = 0x5c;
          v9[1] = (v2 >> 6) + 0x30;
          v9[2] = ((char)v2 >> 3 & 7U) + 0x30;
          v1 = &v9[4];
          v9[3] = (v2 & 7) + 0x30;
          v9 = v1;
        }
      }
      else {
        v1 = &v9[1];
        *v9 = v2;
        v9 = v1;
      }
    }
    *v9 = 0x22;
    v9[1] = 0;
    a0 = v6;
  }
  return a0;
}


// Function: sub_11b50 @ 0x11b50
void sub_11b50(void) // return-dupe x2, ternary
{
  int v1; // eax
  unsigned long v10; // stack - 0x20
  void *v2; // rax
  int v3; // stack - 0x50
  char *v4; // stack - 0x38
  void *v5; // stack - 0x48
  int v6; // stack - 0x4c
  void *v7; // stack - 0x40
  char *v8; // stack - 0x30
  void *v9; // stack - 0x28
  
  if (dat_2d5c0)
    return;
  v5 = (void *)sub_11984(dat_2d848);
  v7 = (void *)sub_11984(dat_2d850);
  v2 = (void *)sub_1d0f7("diff%s %s %s",dat_2d330,v5,v7);
  if (dat_2d2c0) { // branch-flip
    v1 = fflush_unlocked(stdout);
    if (v1)
      sub_10925(gettext("write failed")); // no-return
    v4 = "/usr/bin/pr";
    v8 = "-h";
    v9 = v2;
    v10 = 0;
    if (pipe(&v3))
      sub_10925("pipe"); // no-return
    dat_2d870 = fork();
    if (dat_2d870 <= -1)
      sub_10925("fork"); // no-return
    if (!dat_2d870) {
      close(v6);
      if (v3) {
        v1 = dup2(v3,0);
        if (v1 <= -1)
          sub_10925("dup2"); // no-return
        close(v3);
      }
      execv("/usr/bin/pr",&v4);
      v1 = (*__errno_location() != 2) ? 0x7e : 0x7f; // branch-flip
      _exit(v1); // no-return
    }
    close(v3);
    dat_2d5c0 = fdopen(v6,"w");
    if (!dat_2d5c0)
      sub_10925("fdopen"); // no-return
    sub_117e9(1);
  }
  else {
    dat_2d5c0 = stdout;
    sub_117e9(0);
    if (dat_2d858)
      printf("%s\n",v2);
  }
  free(v2);
  if (dat_2d1c0 != 2) { // branch-flip
    if (dat_2d1c0 == 3)
      sub_7e5c(0x2d360,&v5,1);
  }
  else {
    sub_7e5c(0x2d360,&v5,0);
  }
  if (v5 != dat_2d848)
    free(v5);
  if (v7 == dat_2d850)
    return;
  free(v7);
}


// Function: sub_11e1b @ 0x11e1b
void sub_11e1b(void) // ternary
{
  int v1; // eax
  char *v2; // rax
  unsigned int v3; // stack - 0x1c
  int v4; // stack - 0x18
  unsigned int v5; // stack - 0x14
  
  if ((dat_2d5c0) && (dat_2d5c0 != stdout)) {
    v4 = 0;
    v1 = ferror_unlocked(dat_2d5c0);
    if (v1)
      sub_10966("write failed"); // no-return
    v1 = fclose(dat_2d5c0);
    if (v1)
      sub_10925(gettext("write failed")); // no-return
    v1 = waitpid(dat_2d870,(int *)&v3,0);
    if (v1 <= -1)
      sub_10925("waitpid"); // no-return
    if ((v4) || (v3 & 0x7f))
      v5 = 0x7fffffff;
    else {
      v5 = (int)v3 >> 8 & 0xff;
    }
    if (v5) {
      if (v5 != 0x7e) { // branch-flip
        if (v5 != 0x7f) { // branch-flip
          v2 = (v5 != 0x7fffffff) ? "subsidiary program \'%s\' failed (exit status %d)" : "subsidiary program \'%s\' failed"; // branch-flip
        }
        else {
          v2 = "subsidiary program \'%s\' not found";
        }
      }
      else {
        v2 = "subsidiary program \'%s\' could not be invoked";
      }
      v2 = gettext(v2);
      error(2,v4,v2,"/usr/bin/pr",(unsigned long)v5); // no-return
    }
  }
  dat_2d5c0 = NULL;
}


// Function: sub_11f90 @ 0x11f90
unsigned long sub_11f90(unsigned char *a0,unsigned char *a1) // return-dupe x3, ternary
{
  unsigned int v1; // eax
  unsigned char *v10;
  unsigned char *v11;
  unsigned long v12; // stack - 0x48
  unsigned char *v13; // stack - 0x40
  unsigned char *v14; // stack - 0x38
  unsigned long v15; // stack - 0x30
  unsigned int v2; // eax
  long v3; // rax
  char v4; // bl
  unsigned char v5;
  char v6; // r12b
  unsigned char v7;
  unsigned char *v8;
  unsigned char *v9;
  
  v12 = 0;
  v8 = a0;
  v10 = a1;
label_11fbb:
  v9 = &v8[1];
  v5 = *v8;
  v1 = (unsigned int)v5;
  v11 = &v10[1];
  v7 = *v10;
  v2 = (unsigned int)v7;
  if (v5 == v7) goto label_123e3;
  switch(dat_2d1e8) {
    case 0:
      goto label_123b9;
    case 1:
label_122e2:
      if (((v5 == 0x20) && (v7 == 9)) || ((v5 == 9 && (v7 == 0x20)))) {
        v15 = v12;
        do {
          if ((char)v1 != ' ') { // branch-flip
            if ((char)v1 != '\t') goto label_1234b;
            v12 += dat_2d2a8 - v12 % dat_2d2a8;
          }
          else {
            v12 += 1;
          }
          v8 = &v9[1];
          v1 = (unsigned int)*v9;
          v9 = v8;
        } while( true );
      }
      break;
    case 2:
    case 3:
      if ((*(unsigned short *)((unsigned long)v1 * 2 + *(long *)__ctype_b_loc()) & 0x2000) && (*(unsigned short *)((unsigned long)v2 * 2 + *(long *)__ctype_b_loc()) & 0x2000)) {
        if (v5 != 10) {
          v13 = v9;
          while ((v5 = *v13, v5 != 10 && (*(unsigned short *)((unsigned long)v5 * 2 + *(long *)__ctype_b_loc()) & 0x2000))) {
            v13 = &v13[1];
          }
          if (v5 != 10) goto label_123b9;
        }
        if (v7 == 10)
          return 0;
        v14 = v11;
        while ((v5 = *v14, v5 != 10 && (*(unsigned short *)((unsigned long)v5 * 2 + *(long *)__ctype_b_loc()) & 0x2000))) {
          v14 = &v14[1];
        }
        if (v5 == 10)
          return 0;
      }
      else if (dat_2d1e8 != 2) goto label_122e2;
      break;
    case 4:
      v8 = v9;
      if (*(unsigned short *)((unsigned long)v5 * 2 + *(long *)__ctype_b_loc()) & 0x2000) {
        do {
          v9 = v8;
          if ((char)v1 == '\n') goto label_120dd;
          v1 = (unsigned int)*v9;
          v8 = &v9[1];
        } while (*(unsigned short *)((unsigned long)v1 * 2 + *(long *)__ctype_b_loc()) & 0x2000);
        v1 = 0x20;
      }
label_120dd:
      v8 = v11;
      if (*(unsigned short *)((unsigned long)v7 * 2 + *(long *)__ctype_b_loc()) & 0x2000) {
        do {
          v11 = v8;
          if ((char)v2 == '\n') goto label_12140;
          v2 = (unsigned int)*v11;
          v8 = &v11[1];
        } while (*(unsigned short *)((unsigned long)v2 * 2 + *(long *)__ctype_b_loc()) & 0x2000);
        v2 = 0x20;
      }
label_12140:
      v4 = (char)v1;
      v6 = (char)v2;
      if (v4 == v6) goto label_123b9;
      if ((((v6 != ' ') || (v4 == '\n')) || (v9 <= &a0[1])) || (!(*(unsigned short *)((unsigned long)v9[-2] * 2 + *(long *)__ctype_b_loc()) & 0x2000))) {
        if (((v4 != ' ') || (v6 == '\n')) || ((v11 <= &a1[1] || (!(*(unsigned short *)((unsigned long)v11[-2] * 2 + *(long *)__ctype_b_loc()) & 0x2000))))) goto label_123b9;
        v10 = &v11[-1];
        v8 = v9;
      }
      else {
        v8 = &v9[-1];
        v10 = v11;
      }
      goto label_11fbb;
    case 5:
      while ((*(unsigned short *)((unsigned long)v1 * 2 + *(long *)__ctype_b_loc()) & 0x2000 && ((char)v1 != '\n'))) {
        v8 = &v9[1];
        v1 = (unsigned int)*v9;
        v9 = v8;
      }
      while ((*(unsigned short *)((unsigned long)v2 * 2 + *(long *)__ctype_b_loc()) & 0x2000 && ((char)v2 != '\n'))) {
        v8 = &v11[1];
        v2 = (unsigned int)*v11;
        v11 = v8;
      }
    
  }
label_123b9:
  v5 = (unsigned char)v1;
  v7 = (unsigned char)v2;
  if (dat_2d1ee) {
    v5 = (unsigned char)tolower(v1);
    v7 = (unsigned char)tolower(v2);
  }
  if (v5 != v7)
    return 1;
label_123e3:
  if (v5 == 10)
    return 0;
  v3 = (v5 != 9) ? 1 : dat_2d2a8 - v12 % dat_2d2a8; // branch-flip
  v12 += v3;
  v8 = v9;
  v10 = v11;
  goto label_11fbb;
label_1234b:
  if ((char)v2 != ' ') { // branch-flip
    if ((char)v2 != '\t') goto label_12392;
    v15 += dat_2d2a8 - v15 % dat_2d2a8;
  }
  else {
    v15 += 1;
  }
  v8 = &v11[1];
  v2 = (unsigned int)*v11;
  v11 = v8;
  goto label_1234b;
label_12392:
  if (v12 != v15)
    return 1;
  goto label_123b9;
}


// Function: sub_12437 @ 0x12437
unsigned long sub_12437(unsigned long a0)
{
  return a0;
}


// Function: sub_12449 @ 0x12449
unsigned long sub_12449(unsigned long a0)
{
  return a0;
}


// Function: sub_1245b @ 0x1245b
void sub_1245b(long a0,void *a1,void *a2)
{
  long v1;
  long *v2; // rax
  long v3; // stack - 0x20
  
  v3 = a0;
  while (v3) {
    v2 = (long *)(*a1)(v3);
    v1 = *v2;
    *v2 = 0;
    (*a2)(v3);
    *v2 = v1;
    v3 = v1;
  }
}


// Function: sub_124cf @ 0x124cf
void sub_124cf(unsigned long a0,unsigned long a1)
{
  sub_124fe(a0,a1,0);
}


// Function: sub_124fe @ 0x124fe
void sub_124fe(char *a0,unsigned long *a1,char a2) // ternary, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_124fe
{
  unsigned long v1;
  long v2;
  void *v3;
  int v4; // eax
  char *v5; // stack - 0x38
  char *v6; // stack - 0x30
  char *v7; // stack - 0x28
  
  v3 = dat_2d5c0;
  v1 = *a1;
  v2 = a1[1];
  v5 = NULL;
  if ((a0) && (*a0)) {
    v5 = (dat_2d2b0) ? "%s\t" : "%s "; // branch-flip
    v6 = v5;
    v7 = a0;
    if ((dat_2d2b1) && (*(char *)*a1 == '\n')) {
      v6 = "%s";
      v7 = &a0[*a0 == ' '];
    }
    fprintf(dat_2d5c0,v6,v7);
  }
  v4 = ((a2) && (*(char *)(v2 + -1) == '\n'));
  sub_1267d(v1,v2 - v4,v5,a0);
  if (((!a0) || (*a0)) && (*(char *)(v2 + -1) != '\n')) {
    sub_128c2(3);
    fprintf(v3,"\n\\ %s\n",gettext("No newline at end of file"));
  }
}


// Function: sub_1267d @ 0x1267d
void sub_1267d(unsigned char *a0,unsigned char *a1,char *a2,unsigned long a3) // return-dupe x2
{
  unsigned char *v1;
  unsigned long v10; // stack - 0x48
  unsigned char v2;
  void *v3;
  unsigned long v4;
  unsigned long v5; // rax
  unsigned char *v6; // stack - 0x70
  unsigned long v7; // stack - 0x60
  long v8; // stack - 0x58
  long v9; // stack - 0x50
  
  v3 = dat_2d5c0;
  v4 = dat_2d2a8;
  v10 = 0x400;
  if (dat_2d2a1 != '\x01') {
    v7 = (long)a1 - (long)a0;
    v6 = a0;
    while( true ) {
      if (!v7)
        return;
      v4 = v10;
      if (v7 <= 0x400)
        v4 = v7;
      v5 = fwrite_unlocked(v6,1,v4,dat_2d5c0);
      if (v5 < v4) break;
      v6 = &v6[v5];
      v7 -= v5;
      sub_10d14();
    }
    return;
  }
  v5 = 0;
  v8 = 0;
label_1286f:
  do {
    while( true ) {
      while( true ) {
        if (a1 <= a0)
          return;
        v8 += 1;
        if (v8 == 0x400) {
          sub_10d14();
          v8 = 0;
        }
        v1 = &a0[1];
        v2 = *a0;
        a0 = v1;
        if (v2 != 0xd) break;
        putc_unlocked(0xd,v3);
        if (((a2) && (v1 < a1)) && (*v1 != 10))
          fprintf(v3,a2,a3);
        v5 = 0;
      }
      if (v2 <= 0xd) break;
label_12838:
      v5 += (*(unsigned short *)((unsigned long)v2 * 2 + *(long *)__ctype_b_loc()) & 0x4000) != 0;
      putc_unlocked((unsigned int)v2,v3);
    }
    if (v2 != 8) {
      if (v2 != 9) goto label_12838;
      v9 = v4 - v5 % v4;
      v5 = v9 + v5;
      do {
        putc_unlocked(0x20,v3);
        v9 -= 1;
      } while (v9);
      goto label_1286f;
    }
    if (v5) {
      v5 -= 1;
      putc_unlocked(8,v3);
    }
  } while( true );
}


// Function: sub_12889 @ 0x12889
void sub_12889(unsigned long *a0)
{
  fwrite_unlocked((void *)a0[1],*a0,1,dat_2d5c0);
}


// Function: sub_128c2 @ 0x128c2
void sub_128c2(int a0)
{
  if (a0 != 3)
    sub_10d14();
  if ((dat_2d859) && (a0 != dat_2d010)) {
    sub_12889(0x2d040);
    switch(a0) {
      case 0:
        sub_12889(0x2d080);
        break;
      case 1:
        sub_12889(0x2d090);
        break;
      case 2:
        sub_12889(0x2d0a0);
        break;
      case 3:
        sub_12889(0x2d070);
        break;
      case 4:
        sub_12889(0x2d0b0);
        break;
      default:
        abort(); // no-return
      
    }
    sub_12889(0x2d050);
    dat_2d010 = a0;
  }
}


// Function: sub_129a8 @ 0x129a8
long sub_129a8(long a0,long a1)
{
  return a1 + *(long *)(a0 + 0xe8) + 1;
}


// Function: sub_129d0 @ 0x129d0
void sub_129d0(unsigned long a0,long a1,long a2,long *a3,long *a4)
{
  *a3 = sub_129a8(a0,a1 + -1) + 1;
  *a4 = sub_129a8(a0,a2 + 1) + -1;
}


// Function: sub_12a37 @ 0x12a37
void sub_12a37(char a0,unsigned long a1,unsigned long a2,unsigned long a3) // return-dupe
{
  long v1; // stack - 0x18
  long v2; // stack - 0x20
  
  sub_129d0(a1,a2,a3,&v2,&v1);
  if (v2 < v1) {
    fprintf(dat_2d5c0,"%ld%c%ld",v2,(unsigned long)(unsigned int)(int)a0,v1);
    return;
  }
  fprintf(dat_2d5c0,"%ld",v1);
}


// Function: sub_12af4 @ 0x12af4
unsigned char sub_12af4(long *a0,long *a1,long *a2,long *a3,long *a4) // early-return, ternary, warn: iteboolean: re-rolled 2 0/1 select diamond(s) to a boolean assignment in sub_12af4
{
  char v1;
  long v10; // rax
  char *v11; // rax
  unsigned char v12; // dl
  long *v13; // stack - 0xa0
  long v14; // stack - 0x98
  long v15; // stack - 0x90
  long v16; // stack - 0x88
  unsigned char *v17; // stack - 0x80
  unsigned char *v18; // stack - 0x78
  unsigned char *v2;
  bool v3;
  long v4;
  long v5;
  bool v6; // al
  bool v7; // al
  unsigned int v8; // eax
  long v9; // rax
  
  v5 = dat_2d548;
  v4 = dat_2d418;
  if ((dat_2d1ec) || (dat_2d280))
    v3 = 1;
  else {
    v3 = 0;
  }
  v8 = (unsigned int)dat_2d1ec;
  v6 = ((dat_2d1ec) && (2 <= dat_2d1e8));
  v7 = ((v6) && (4 <= dat_2d1e8));
  v15 = 0;
  v14 = 0;
  *a1 = a0[3];
  *a3 = a0[4];
  v13 = a0;
  do {
    v9 = v13[2] + v13[3] + -1;
    v10 = v13[1] + v13[4] + -1;
    v14 += v13[2];
    v15 += v13[1];
    v16 = v13[3];
    while ((v16 <= v9 && (v3))) {
      v2 = *(unsigned char **)(v4 + v16 * 8);
      v11 = (char *)(*(long *)(v4 + (v16 + 1) * 8) + -1);
      v1 = *v11;
      v17 = v2;
      if (v6) {
        for (; *v17 != 10; v17 = &v17[1]) {
          if (!(*(unsigned short *)((unsigned long)*v17 * 2 + *(long *)__ctype_b_loc()) & 0x2000)) {
            if (!v7)
              v17 = v2;
            break;
          }
        }
      }
      if ((&v11[(unsigned long)(v1 != '\n') - (long)v17] != (char *)(long)(int)(v8 - 1)) && ((!dat_2d280 || ((int)re_search(0x2d260,v2,(unsigned long)&v11[(unsigned long)(v1 != '\n') - (long)v2] & 0xffffffff,0,(unsigned long)&v11[(unsigned long)(v1 != '\n') - (long)v2] & 0xffffffff,0) <= -1))))
        v3 = 0;
      v16 += 1;
    }
    v16 = v13[4];
    while ((v16 <= v10 && (v3))) {
      v2 = *(unsigned char **)(v5 + v16 * 8);
      v11 = (char *)(*(long *)(v5 + (v16 + 1) * 8) + -1);
      v1 = *v11;
      v18 = v2;
      if (v6) {
        for (; *v18 != 10; v18 = &v18[1]) {
          if (!(*(unsigned short *)((unsigned long)*v18 * 2 + *(long *)__ctype_b_loc()) & 0x2000)) {
            if (!v7)
              v18 = v2;
            break;
          }
        }
      }
      if ((&v11[(unsigned long)(v1 != '\n') - (long)v18] != (char *)(long)(int)(v8 - 1)) && ((!dat_2d280 || ((int)re_search(0x2d260,v2,(unsigned long)&v11[(unsigned long)(v1 != '\n') - (long)v2] & 0xffffffff,0,(unsigned long)&v11[(unsigned long)(v1 != '\n') - (long)v2] & 0xffffffff,0) <= -1))))
        v3 = 0;
      v16 += 1;
    }
    v13 = (long *)*v13;
    if (!v13) {
      *a2 = v9;
      *a4 = v10;
      if (v3)
        return 0;
      v12 = (v15) ? 2 : 0; // branch-flip
      return v14 != 0 | v12;
    }
  } while( true );
}


// Function: sub_12fa9 @ 0x12fa9
char * sub_12fa9(char *a0,char *a1,char *a2)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  char *v3; // rax
  
  v1 = strlen(a0);
  v2 = strlen(a1);
  v3 = (char *)sub_1c945(strlen(a2) + v1 + v2 + 1);
  sprintf(v3,"%s%s%s",a0,a1,a2);
  return v3;
}


// Function: sub_13030 @ 0x13030
void * sub_13030(unsigned long a0)
{
  void *v1; // rax
  
  v1 = (void *)sub_1c945(a0);
  memset(v1,0,a0);
  return v1;
}


// Function: sub_1306b @ 0x1306b
void sub_1306b(unsigned long *a0)
{
  unsigned long *v1; // stack - 0x30
  
  fflush_unlocked(stdout);
  for (v1 = a0; v1; v1 = (unsigned long *)*v1) {
    fprintf(stderr,"%3ld %3ld delete %ld insert %ld\n",v1[3],v1[4],v1[2],v1[1]);
  }
  fflush_unlocked(stderr);
}

