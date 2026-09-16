// Function: sub_1000 @ 0x1000
void sub_1000(void) // return-dupe
{
  if (!dat_7fe8)
    return;
  (*dat_7fe8)();
}

// Function: __cxa_finalize @ 0x10b0
void __cxa_finalize(void)
{
  (*dat_7ff8)(); // jump-as-call
}

// Function: puts @ 0x10c0
int puts(char *a0)
{
  int v1; // eax
  
  v1 = (*dat_7f98)(); // jump-as-call
  return v1;
}

// Function: write @ 0x10d0
long write(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_7fa0)(); // jump-as-call
  return v1;
}

// Function: __stack_chk_fail @ 0x10e0
void __stack_chk_fail(void)
{
  (*dat_7fa8)(); // jump-as-call
}

// Function: alarm @ 0x10f0
unsigned int alarm(unsigned int a0)
{
  unsigned int v1; // eax
  
  v1 = (*dat_7fb0)(); // jump-as-call
  return v1;
}

// Function: read @ 0x1100
long read(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_7fb8)(); // jump-as-call
  return v1;
}

// Function: signal @ 0x1110
void signal(void)
{
  (*dat_7fc0)(); // jump-as-call
}

// Function: memcpy @ 0x1120
void * memcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_7fc8)(); // jump-as-call
  return v1;
}

// Function: getc @ 0x1130
int getc(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_7fd0)(); // jump-as-call
  return v1;
}

// Function: sub_1140 @ 0x1140
void sub_1140(void)
{
  signal(0xe,0x12b0);
  alarm(0x14); // tail-call
}

// Function: sub_1170 @ 0x1170
unsigned long sub_1170(void) // return-dupe
{
  int v1; // eax
  
  if (sub_12d0()) {
    puts((char *)0x6b30);
    return 0;
  }
  puts((char *)0x6b22);
  do {
    v1 = getc(dat_8010);
    if (v1 == -1)
      return 0;
  } while (v1 != 10);
  return 0;
}

// Function: sub_11c0 @ 0x11c0
void sub_11c0(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_7fe0)(sub_1170,v2,&Stack0000000000000008,0x2250,0x22c0,a2,v1);
  do {
  } while( true );
}

// Function: sub_12d0 @ 0x12d0
unsigned int sub_12d0(void)
{
  long v1;
  unsigned int v10;
  undefined4 v11;
  char v12 [16];
  undefined4 v13;
  undefined4 v14;
  undefined4 v15;
  unsigned char v16;
  unsigned int *v17; // rax
  long v18;
  unsigned char **v19;
  unsigned int v2;
  unsigned int *v20;
  unsigned char **v21;
  unsigned char v22;
  unsigned int *v23; // rcx
  unsigned long v24; // rcx
  unsigned int *v25;
  unsigned char *v26;
  unsigned char *v27;
  char v28 [16]; // stack - 0x3b78
  char v29 [15144];
  unsigned int v3;
  long v30;
  char v31 [16]; // stack - 0x3b68
  unsigned char *v32 [2]; // stack - 0x3b58
  unsigned long v33;
  unsigned int *v34;
  unsigned long v35;
  unsigned char **v36; // rcx
  unsigned char **v37;
  unsigned int v38;
  bool v39;
  int v4;
  unsigned long v40; // stack - 0x3b80
  unsigned int v5;
  unsigned int v6;
  unsigned int v7;
  unsigned int v8;
  unsigned int v9;
  
  v38 = 0;
  v40 = 0x1335;
  v28._0_16_ = 0;
  v31._0_16_ = 0;
  v27 = memcpy(v29,(void *)0x3008,0x3b1a);
label_1345:
  do {
    v16 = *v27;
    v39 = v16 == 0xaa;
    v26 = v27;
    if (!v39) {
label_1359:
      v12 = v28._0_16_;
      if (0xaa <= v16 && !v39) {
        if (v16 != 0xd5) { // branch-flip
          if (0xd6 <= v16) { // branch-flip
            if (v16 != 0xef) { // branch-flip
              if (v16 != 0xf6)
                return v38;
              v32[0] = &v26[2];
              v16 = v26[1];
              if (v16 & 3) { // branch-flip
                if ((v16 & 3) != 2)
                  return v38;
                *(unsigned int *)&v28[(unsigned long)(v16 >> 4 & 3) * 4] = *(unsigned int *)&v26[2];
                v27 = &v26[6];
              }
              else {
                *(unsigned int *)&v28[(unsigned long)(v16 >> 4 & 3) * 4] = *(unsigned int *)&v28[(unsigned long)(v16 >> 6) * 4];
                v27 = &v26[5];
              }
            }
            else {
              v27 = &v26[2];
              v32[0] = v27;
              v16 = v26[1];
              v22 = v16 & 3;
              if (v22 != 2) { // branch-flip
                if (v22 != 3) { // branch-flip
                  if (v22 != 1) { // branch-flip
                    v22 = v28[(unsigned long)(v16 >> 4 & 3) * 4];
                    v40 = 0x1add;
                    if (write((unsigned int)v22,&v28[(unsigned long)(v16 >> 6) * 4],1) != 1) { // branch-flip
                      *v32[0] = 0;
                      v27 = v32[0];
                    }
                    else {
label_1af8:
                      v27 = v32[0];
                    }
                  }
                  else {
                    v22 = v26[2];
                    v40 = 0x2198;
                    if (write((unsigned int)v22,&v28[(unsigned long)(v16 >> 6) * 4],1) != 1)
                      v32[0][2] = 0;
                    v27 = &v32[0][2];
                  }
                }
                else {
                  v16 = v26[3];
                  v40 = 0x1cae;
                  if (write((unsigned int)v16,v27,1) != 1)
                    v32[0][3] = 0;
                  v27 = &v32[0][3];
                }
              }
              else {
                v16 = v28[(unsigned long)(v16 >> 4 & 3) * 4];
                v40 = 0x1f14;
                if (write((unsigned int)v16,v27,1) != 1)
                  v32[0][1] = 0;
                v27 = &v32[0][1];
              }
            }
          }
          else if (v16 != 0xb1) { // branch-flip
            if (v16 != 0xc3)
              return v38;
            v32[0] = &v26[2];
            v16 = v26[1];
            if (v16 & 3) { // branch-flip
              if ((v16 & 3) != 2)
                return v38;
              if (*(int *)&v28[(unsigned long)(v16 >> 4 & 3) * 4] != *(int *)&v26[2])
                v38 = 1;
              v27 = &v26[7];
            }
            else {
              if (*(int *)&v28[(unsigned long)(v16 >> 4 & 3) * 4] != *(int *)&v28[(unsigned long)(v16 >> 6) * 4])
                v38 = 1;
              v27 = &v26[4];
            }
          }
          else {
            v27 = &v26[2];
            v32[0] = v27;
            v16 = v26[1];
            if (v16 & 3) { // branch-flip
              if ((v16 & 3) != 2)
                return v38;
              v27 = &v26[6];
              *(unsigned int *)&v28[(unsigned long)(v16 >> 4 & 3) * 4] = *(unsigned int *)&v28[(unsigned long)(v16 >> 4 & 3) * 4] >> (v26[2] & 0x1f);
            }
            else {
              *(unsigned int *)&v28[(unsigned long)(v16 >> 4 & 3) * 4] = *(unsigned int *)&v28[(unsigned long)(v16 >> 4 & 3) * 4] >> (v28[(unsigned long)(v16 >> 6) * 4] & 0x1f);
            }
          }
        }
        else {
          v32[0] = &v26[2];
          v16 = v26[1] & 3;
          v11 = v28._0_4_;
          v13 = v28._4_4_;
          v14 = v28._8_4_;
          v15 = v28._12_4_;
          if (v16 != 2) { // branch-flip
            if (v16 != 3) { // branch-flip
              v4 = *(int *)&v28[(unsigned long)(v26[1] >> 6) * 4];
              v35 = (unsigned long)(unsigned int)(7 - v4);
              v18 = v35 * 4;
              v34 = (unsigned int *)&v28[v18];
              if (v16 != 1) { // branch-flip
                if (v28 < v34) {
                  v24 = (unsigned long)(v18 - 1U) >> 3;
                  v33 = v24 + 1;
                  v1 = v18 + v24 * -4;
                  if ((v33 * 4 - v1 && v1 <= (long)(v33 * 4)) || ((unsigned long)(v18 - 1U) <= 0x17)) {
                    v20 = (unsigned int *)v28;
                    do {
                      v5 = *v20;
                      v25 = &v20[1];
                      v17 = &v34[-1];
                      *v20 = *v34;
                      *v34 = v5;
                      v20 = v25;
                      v34 = v17;
                    } while (v25 < v17);
                  }
                  else {
                    v20 = &(&v30)[v35];
                    v25 = (unsigned int *)v28;
                    do {
                      v5 = *v25;
                      v2 = v25[1];
                      v3 = v25[2];
                      v7 = v25[3];
                      v8 = *v20;
                      v9 = v20[1];
                      v10 = v20[2];
                      v17 = &v20[-4];
                      v23 = &v25[4];
                      *v25 = v20[3];
                      v25[1] = v10;
                      v25[2] = v9;
                      v25[3] = v8;
                      *v20 = v7;
                      v20[1] = v3;
                      v20[2] = v2;
                      v20[3] = v5;
                      v20 = v17;
                      v25 = v23;
                    } while (v17 != &(&(&v30)[v35])[(v33 >> 2) * -4]);
                    v35 = v33 & 0xfffffffffffffffc;
                    v18 = v35 * 4;
                    v34 = &v34[-v35];
                    if (v33 != v35) {
                      v5 = *(unsigned int *)&v28[v18];
                      *(unsigned int *)&v28[v18] = *v34;
                      *v34 = v5;
                      if (&v28[v18 + 4] < &v34[-1]) {
                        v5 = *(unsigned int *)&v28[v18 + 4];
                        *(unsigned int *)&v28[v18 + 4] = v34[-1];
                        v34[-1] = v5;
                        if (&v28[v18 + 8] < &v34[-2]) {
                          v5 = *(unsigned int *)&v28[v18 + 8];
                          *(unsigned int *)&v28[v18 + 8] = v34[-2];
                          v34[-2] = v5;
                        }
                      }
                    }
                  }
                }
                v21 = (unsigned char **)&v31[0xc];
                v37 = (unsigned char **)&v28[(unsigned long)(unsigned int)(8 - v4) * 4];
                if (v37 < v21) {
                  v35 = (unsigned long)&v31[0xb - (long)v37] >> 3;
                  if (((long)(((unsigned long)(unsigned int)(8 - v4) + 1 + v35) * 4) <= (long)(v35 * -4 + 0x1c) || v32 <= v37) && (0x18 <= &v31[0xb - (long)v37])) {
                    v35 += 1;
                    v19 = v37;
                    v34 = (unsigned int *)v31;
                    do {
                      v7 = *(unsigned int *)v19;
                      v8 = *(unsigned int *)((long)v19 + 4);
                      v9 = *(unsigned int *)&v19[1];
                      v10 = *(unsigned int *)((long)v19 + 0xc);
                      v5 = *v34;
                      v2 = v34[1];
                      v3 = v34[2];
                      v36 = &v19[2];
                      *(unsigned int *)v19 = v34[3];
                      *(unsigned int *)((long)v19 + 4) = v3;
                      *(unsigned int *)&v19[1] = v2;
                      *(unsigned int *)((long)v19 + 0xc) = v5;
                      v20 = &v34[-4];
                      *v34 = v10;
                      v34[1] = v9;
                      v34[2] = v8;
                      v34[3] = v7;
                      v19 = v36;
                      v34 = v20;
                    } while (v36 != &v37[(v35 >> 2) * 2]);
                    v33 = v35 & 0xfffffffffffffffc;
                    v34 = (unsigned int *)((long)v37 + v33 * 4);
                    v18 = v33 * -4;
                    if (v35 != v33) {
                      v5 = *v34;
                      *v34 = *(unsigned int *)((long)v21 + v18);
                      *(unsigned int *)((long)v21 + v18) = v5;
                      if (&v34[1] < &v31[v18 + 8]) {
                        v5 = v34[1];
                        v34[1] = *(unsigned int *)&v31[v18 + 8];
                        *(unsigned int *)&v31[v18 + 8] = v5;
                        if (&v34[2] < &v31[v18 + 4]) {
                          v5 = v34[2];
                          v34[2] = *(unsigned int *)&v31[v18 + 4];
                          *(unsigned int *)&v31[v18 + 4] = v5;
                        }
                      }
                    }
                  }
                  else {
                    do {
                      v5 = *(unsigned int *)v37;
                      v36 = (unsigned char **)((long)v37 + 4);
                      v19 = (unsigned char **)((long)v21 - 4);
                      *(unsigned int *)v37 = *(unsigned int *)v21;
                      *(unsigned int *)v21 = v5;
                      v21 = v19;
                      v37 = v36;
                    } while (v36 < v19);
                  }
                }
                v27 = &v26[4];
                v28._4_4_ = v31._8_4_;
                v28._0_4_ = v31._12_4_;
                v28._8_4_ = v31._4_4_;
                v28._12_4_ = v31._0_4_;
                v31._4_4_ = v14;
                v31._0_4_ = v15;
                v31._8_4_ = v13;
                v31._12_4_ = v11;
              }
              else {
                if (v28 < v34) {
                  v24 = (unsigned long)(v18 - 1U) >> 3;
                  v33 = v24 + 1;
                  v1 = v18 + v24 * -4;
                  if ((v33 * 4 - v1 && v1 <= (long)(v33 * 4)) || ((unsigned long)(v18 - 1U) <= 0x17)) {
                    v20 = (unsigned int *)v28;
                    do {
                      v5 = *v20;
                      v25 = &v20[1];
                      v17 = &v34[-1];
                      *v20 = *v34;
                      *v34 = v5;
                      v20 = v25;
                      v34 = v17;
                    } while (v25 < v17);
                  }
                  else {
                    v20 = &(&v30)[v35];
                    v25 = (unsigned int *)v28;
                    do {
                      v5 = *v25;
                      v2 = v25[1];
                      v3 = v25[2];
                      v7 = v25[3];
                      v8 = *v20;
                      v9 = v20[1];
                      v10 = v20[2];
                      v17 = &v20[-4];
                      v23 = &v25[4];
                      *v25 = v20[3];
                      v25[1] = v10;
                      v25[2] = v9;
                      v25[3] = v8;
                      *v20 = v7;
                      v20[1] = v3;
                      v20[2] = v2;
                      v20[3] = v5;
                      v20 = v17;
                      v25 = v23;
                    } while (v17 != &(&(&v30)[v35])[(v33 >> 2) * -4]);
                    v35 = v33 & 0xfffffffffffffffc;
                    v18 = v35 * 4;
                    v34 = &v34[-v35];
                    if (v33 != v35) {
                      v5 = *(unsigned int *)&v28[v18];
                      *(unsigned int *)&v28[v18] = *v34;
                      *v34 = v5;
                      if (&v28[v18 + 4] < &v34[-1]) {
                        v5 = *(unsigned int *)&v28[v18 + 4];
                        *(unsigned int *)&v28[v18 + 4] = v34[-1];
                        v34[-1] = v5;
                        if (&v28[v18 + 8] < &v34[-2]) {
                          v5 = *(unsigned int *)&v28[v18 + 8];
                          *(unsigned int *)&v28[v18 + 8] = v34[-2];
                          v34[-2] = v5;
                        }
                      }
                    }
                  }
                }
                v21 = (unsigned char **)&v31[0xc];
                v37 = (unsigned char **)&v28[(unsigned long)(unsigned int)(8 - v4) * 4];
                if (v37 < v21) {
                  v35 = (unsigned long)&v31[0xb - (long)v37] >> 3;
                  if (((long)(((unsigned long)(unsigned int)(8 - v4) + 1 + v35) * 4) <= (long)(v35 * -4 + 0x1c) || v32 <= v37) && (0x18 <= &v31[0xb - (long)v37])) {
                    v35 += 1;
                    v19 = v37;
                    v34 = (unsigned int *)v31;
                    do {
                      v7 = *(unsigned int *)v19;
                      v8 = *(unsigned int *)((long)v19 + 4);
                      v9 = *(unsigned int *)&v19[1];
                      v10 = *(unsigned int *)((long)v19 + 0xc);
                      v5 = *v34;
                      v2 = v34[1];
                      v3 = v34[2];
                      v36 = &v19[2];
                      *(unsigned int *)v19 = v34[3];
                      *(unsigned int *)((long)v19 + 4) = v3;
                      *(unsigned int *)&v19[1] = v2;
                      *(unsigned int *)((long)v19 + 0xc) = v5;
                      v20 = &v34[-4];
                      *v34 = v10;
                      v34[1] = v9;
                      v34[2] = v8;
                      v34[3] = v7;
                      v19 = v36;
                      v34 = v20;
                    } while (&v37[(v35 >> 2) * 2] != v36);
                    v33 = v35 & 0xfffffffffffffffc;
                    v34 = (unsigned int *)((long)v37 + v33 * 4);
                    v18 = v33 * -4;
                    if (v35 != v33) {
                      v5 = *v34;
                      *v34 = *(unsigned int *)((long)v21 + v18);
                      *(unsigned int *)((long)v21 + v18) = v5;
                      if (&v34[1] < &v31[v18 + 8]) {
                        v5 = v34[1];
                        v34[1] = *(unsigned int *)&v31[v18 + 8];
                        *(unsigned int *)&v31[v18 + 8] = v5;
                        if (&v34[2] < &v31[v18 + 4]) {
                          v5 = v34[2];
                          v34[2] = *(unsigned int *)&v31[v18 + 4];
                          *(unsigned int *)&v31[v18 + 4] = v5;
                        }
                      }
                    }
                  }
                  else {
                    do {
                      v5 = *(unsigned int *)v37;
                      v36 = (unsigned char **)((long)v37 + 4);
                      v19 = (unsigned char **)((long)v21 - 4);
                      *(unsigned int *)v37 = *(unsigned int *)v21;
                      *(unsigned int *)v21 = v5;
                      v21 = v19;
                      v37 = v36;
                    } while (v36 < v19);
                  }
                }
                v27 = &v26[3];
                v28._4_4_ = v31._8_4_;
                v28._0_4_ = v31._12_4_;
                v28._8_4_ = v31._4_4_;
                v28._12_4_ = v31._0_4_;
                v31._4_4_ = v14;
                v31._0_4_ = v15;
                v31._8_4_ = v13;
                v31._12_4_ = v11;
              }
            }
            else {
              v6 = *(unsigned int *)&v26[2];
              v35 = (unsigned long)(~v6 & 7);
              v18 = v35 * 4;
              v34 = (unsigned int *)&v28[v18];
              if (v28 < v34) {
                v24 = (unsigned long)(v18 - 1U) >> 3;
                v33 = v24 + 1;
                v1 = v18 + v24 * -4;
                if ((v33 * 4 - v1 && v1 <= (long)(v33 * 4)) || ((unsigned long)(v18 - 1U) <= 0x17)) {
                  v20 = (unsigned int *)v28;
                  do {
                    v5 = *v20;
                    v25 = &v20[1];
                    v17 = &v34[-1];
                    *v20 = *v34;
                    *v34 = v5;
                    v20 = v25;
                    v34 = v17;
                  } while (v25 < v17);
                }
                else {
                  v20 = &(&v30)[v35];
                  v25 = (unsigned int *)v28;
                  do {
                    v5 = *v25;
                    v2 = v25[1];
                    v3 = v25[2];
                    v7 = v25[3];
                    v8 = *v20;
                    v9 = v20[1];
                    v10 = v20[2];
                    v17 = &v20[-4];
                    v23 = &v25[4];
                    *v25 = v20[3];
                    v25[1] = v10;
                    v25[2] = v9;
                    v25[3] = v8;
                    *v20 = v7;
                    v20[1] = v3;
                    v20[2] = v2;
                    v20[3] = v5;
                    v20 = v17;
                    v25 = v23;
                  } while (&(&(&v30)[v35])[(v33 >> 2) * -4] != v17);
                  v35 = v33 & 0xfffffffffffffffc;
                  v18 = v35 * 4;
                  v34 = &v34[-v35];
                  if (v33 != v35) {
                    v5 = *(unsigned int *)&v28[v18];
                    *(unsigned int *)&v28[v18] = *v34;
                    *v34 = v5;
                    if (&v28[v18 + 4] < &v34[-1]) {
                      v5 = *(unsigned int *)&v28[v18 + 4];
                      *(unsigned int *)&v28[v18 + 4] = v34[-1];
                      v34[-1] = v5;
                      if (&v28[v18 + 8] < &v34[-2]) {
                        v5 = *(unsigned int *)&v28[v18 + 8];
                        *(unsigned int *)&v28[v18 + 8] = v34[-2];
                        v34[-2] = v5;
                      }
                    }
                  }
                }
              }
              v21 = (unsigned char **)&v31[0xc];
              v35 = (unsigned long)(8 - (v6 & 7));
              v37 = (unsigned char **)&v28[v35 * 4];
              if (v37 < v21) {
                v33 = (unsigned long)&v31[0xb - (long)v37] >> 3;
                if (((long)((v35 + 1 + v33) * 4) <= (long)(v33 * -4 + 0x1c) || v32 <= v37) && (0x18 <= &v31[0xb - (long)v37])) {
                  v33 += 1;
                  v19 = v37;
                  v34 = (unsigned int *)v31;
                  do {
                    v7 = *(unsigned int *)v19;
                    v8 = *(unsigned int *)((long)v19 + 4);
                    v9 = *(unsigned int *)&v19[1];
                    v10 = *(unsigned int *)((long)v19 + 0xc);
                    v5 = *v34;
                    v2 = v34[1];
                    v3 = v34[2];
                    v36 = &v19[2];
                    *(unsigned int *)v19 = v34[3];
                    *(unsigned int *)((long)v19 + 4) = v3;
                    *(unsigned int *)&v19[1] = v2;
                    *(unsigned int *)((long)v19 + 0xc) = v5;
                    v20 = &v34[-4];
                    *v34 = v10;
                    v34[1] = v9;
                    v34[2] = v8;
                    v34[3] = v7;
                    v19 = v36;
                    v34 = v20;
                  } while (v36 != &v37[(v33 >> 2) * 2]);
                  v35 = v33 & 0xfffffffffffffffc;
                  v34 = (unsigned int *)((long)v37 + v35 * 4);
                  v18 = v35 * -4;
                  if (v33 != v35) {
                    v5 = *v34;
                    *v34 = *(unsigned int *)((long)v21 + v18);
                    *(unsigned int *)((long)v21 + v18) = v5;
                    if (&v34[1] < &v31[v18 + 8]) {
                      v5 = v34[1];
                      v34[1] = *(unsigned int *)&v31[v18 + 8];
                      *(unsigned int *)&v31[v18 + 8] = v5;
                      if (&v34[2] < &v31[v18 + 4]) {
                        v5 = v34[2];
                        v34[2] = *(unsigned int *)&v31[v18 + 4];
                        *(unsigned int *)&v31[v18 + 4] = v5;
                      }
                    }
                  }
                }
                else {
                  do {
                    v5 = *(unsigned int *)v37;
                    v36 = (unsigned char **)((long)v37 + 4);
                    v19 = (unsigned char **)((long)v21 - 4);
                    *(unsigned int *)v37 = *(unsigned int *)v21;
                    *(unsigned int *)v21 = v5;
                    v21 = v19;
                    v37 = v36;
                  } while (v36 < v19);
                }
              }
              v28._4_4_ = v31._8_4_;
              v28._0_4_ = v31._12_4_;
              v28._8_4_ = v31._4_4_;
              v28._12_4_ = v31._0_4_;
              v31._4_4_ = v14;
              v31._0_4_ = v15;
              v31._8_4_ = v13;
              v31._12_4_ = v11;
label_1801:
              v27 = &v26[9];
            }
          }
          else {
            v6 = *(unsigned int *)&v26[2];
            v35 = (unsigned long)(~v6 & 7);
            v18 = v35 * 4;
            v34 = (unsigned int *)&v28[v18];
            if (v28 < v34) {
              v24 = (unsigned long)(v18 - 1U) >> 3;
              v33 = v24 + 1;
              v1 = v18 + v24 * -4;
              if ((v33 * 4 - v1 && v1 <= (long)(v33 * 4)) || ((unsigned long)(v18 - 1U) <= 0x17)) {
                v20 = (unsigned int *)v28;
                do {
                  v5 = *v20;
                  v25 = &v20[1];
                  v17 = &v34[-1];
                  *v20 = *v34;
                  *v34 = v5;
                  v20 = v25;
                  v34 = v17;
                } while (v25 < v17);
              }
              else {
                v20 = &(&v30)[v35];
                v25 = (unsigned int *)v28;
                do {
                  v5 = *v25;
                  v2 = v25[1];
                  v3 = v25[2];
                  v7 = v25[3];
                  v8 = *v20;
                  v9 = v20[1];
                  v10 = v20[2];
                  v17 = &v20[-4];
                  v23 = &v25[4];
                  *v25 = v20[3];
                  v25[1] = v10;
                  v25[2] = v9;
                  v25[3] = v8;
                  *v20 = v7;
                  v20[1] = v3;
                  v20[2] = v2;
                  v20[3] = v5;
                  v20 = v17;
                  v25 = v23;
                } while (v17 != &(&(&v30)[v35])[(v33 >> 2) * -4]);
                v35 = v33 & 0xfffffffffffffffc;
                v18 = v35 * 4;
                v34 = &v34[-v35];
                if (v33 != v35) {
                  v5 = *(unsigned int *)&v28[v18];
                  *(unsigned int *)&v28[v18] = *v34;
                  *v34 = v5;
                  if (&v28[v18 + 4] < &v34[-1]) {
                    v5 = *(unsigned int *)&v28[v18 + 4];
                    *(unsigned int *)&v28[v18 + 4] = v34[-1];
                    v34[-1] = v5;
                    if (&v28[v18 + 8] < &v34[-2]) {
                      v5 = *(unsigned int *)&v28[v18 + 8];
                      *(unsigned int *)&v28[v18 + 8] = v34[-2];
                      v34[-2] = v5;
                    }
                  }
                }
              }
            }
            v21 = (unsigned char **)&v31[0xc];
            v35 = (unsigned long)(8 - (v6 & 7));
            v37 = (unsigned char **)&v28[v35 * 4];
            if (v37 < v21) {
              v33 = (unsigned long)&v31[0xb - (long)v37] >> 3;
              if (((long)((v35 + 1 + v33) * 4) <= (long)(v33 * -4 + 0x1c) || v32 <= v37) && (0x18 <= &v31[0xb - (long)v37])) {
                v33 += 1;
                v19 = v37;
                v34 = (unsigned int *)v31;
                do {
                  v7 = *(unsigned int *)v19;
                  v8 = *(unsigned int *)((long)v19 + 4);
                  v9 = *(unsigned int *)&v19[1];
                  v10 = *(unsigned int *)((long)v19 + 0xc);
                  v5 = *v34;
                  v2 = v34[1];
                  v3 = v34[2];
                  v36 = &v19[2];
                  *(unsigned int *)v19 = v34[3];
                  *(unsigned int *)((long)v19 + 4) = v3;
                  *(unsigned int *)&v19[1] = v2;
                  *(unsigned int *)((long)v19 + 0xc) = v5;
                  v20 = &v34[-4];
                  *v34 = v10;
                  v34[1] = v9;
                  v34[2] = v8;
                  v34[3] = v7;
                  v19 = v36;
                  v34 = v20;
                } while (&v37[(v33 >> 2) * 2] != v36);
                v35 = v33 & 0xfffffffffffffffc;
                v34 = (unsigned int *)((long)v37 + v35 * 4);
                v18 = v35 * -4;
                if (v33 != v35) {
                  v5 = *v34;
                  *v34 = *(unsigned int *)((long)v21 + v18);
                  *(unsigned int *)((long)v21 + v18) = v5;
                  if (&v34[1] < &v31[v18 + 8]) {
                    v5 = v34[1];
                    v34[1] = *(unsigned int *)&v31[v18 + 8];
                    *(unsigned int *)&v31[v18 + 8] = v5;
                    if (&v34[2] < &v31[v18 + 4]) {
                      v5 = v34[2];
                      v34[2] = *(unsigned int *)&v31[v18 + 4];
                      *(unsigned int *)&v31[v18 + 4] = v5;
                    }
                  }
                }
              }
              else {
                do {
                  v5 = *(unsigned int *)v37;
                  v36 = (unsigned char **)((long)v37 + 4);
                  v19 = (unsigned char **)((long)v21 - 4);
                  *(unsigned int *)v37 = *(unsigned int *)v21;
                  *(unsigned int *)v21 = v5;
                  v21 = v19;
                  v37 = v36;
                } while (v36 < v19);
              }
            }
            v27 = &v26[5];
            v28._4_4_ = v31._8_4_;
            v28._0_4_ = v31._12_4_;
            v28._8_4_ = v31._4_4_;
            v28._12_4_ = v31._0_4_;
            v31._4_4_ = v14;
            v31._0_4_ = v15;
            v31._8_4_ = v13;
            v31._12_4_ = v11;
          }
        }
        goto label_1345;
      }
      switch(v16) {
        default:
          goto label_1540;
        case 0x1e:
          goto label_1410;
        case 0x3e:
          v32[0] = &v26[2];
          v16 = v26[1];
          if (v16 & 3) { // branch-flip
            if ((v16 & 3) != 2)
              return v38;
            v35 = (unsigned long)(v16 >> 4 & 3);
            *(unsigned int *)&v28[v35 * 4] = *(unsigned int *)&v28[v35 * 4] | *(unsigned int *)&v26[2];
            v27 = &v26[7];
          }
          else {
            *(unsigned int *)&v28[(unsigned long)(v16 >> 4 & 3) * 4] = *(unsigned int *)&v28[(unsigned long)(v16 >> 4 & 3) * 4] | *(unsigned int *)&v28[(unsigned long)(v16 >> 6) * 4];
            v27 = &v26[4];
          }
          goto label_1345;
        case 0x4a:
          v32[0] = &v26[2];
          v16 = v26[1];
          if (v16 & 3) { // branch-flip
            if ((v16 & 3) != 2)
              return v38;
            *(int *)&v28[(unsigned long)(v16 >> 4 & 3) * 4] = *(int *)&v28[(unsigned long)(v16 >> 4 & 3) * 4] + *(int *)&v26[2];
            v27 = &v26[7];
          }
          else {
            *(int *)&v28[(unsigned long)(v16 >> 4 & 3) * 4] = *(int *)&v28[(unsigned long)(v16 >> 4 & 3) * 4] + *(int *)&v28[(unsigned long)(v16 >> 6) * 4];
            v27 = &v26[2];
          }
          goto label_1345;
        case 0x5d:
          v16 = v26[1];
          if (v16 & 3) { // branch-flip
            if ((v16 & 3) != 2)
              return v38;
            v6 = *(unsigned int *)&v26[2];
            v32[0] = &v26[8];
          }
          else {
            v32[0] = &v26[3];
            v6 = *(unsigned int *)&v28[(unsigned long)(v16 >> 6) * 4];
          }
          v40 = 0x1b81;
          if ((unsigned long)v6 == read(0,&v28[(unsigned long)(v16 >> 4 & 3) * 4],(unsigned long)v6)) goto label_1af8;
          v38 = 1;
          *v32[0] = 0;
          v27 = v32[0];
          goto label_1345;
        case 0x7c:
          v32[0] = &v26[2];
          v16 = v26[1];
          if (v16 & 3) { // branch-flip
            if ((v16 & 3) != 2)
              return v38;
            *(int *)&v28[(unsigned long)(v16 >> 4 & 3) * 4] = *(int *)&v28[(unsigned long)(v16 >> 4 & 3) * 4] << (v26[2] & 0x1f);
            v27 = &v26[7];
          }
          else {
            *(int *)&v28[(unsigned long)(v16 >> 4 & 3) * 4] = *(int *)&v28[(unsigned long)(v16 >> 4 & 3) * 4] << (v28[(unsigned long)(v16 >> 6) * 4] & 0x1f);
            v27 = &v26[4];
          }
          goto label_1345;
        case 0x8b:
          v32[0] = &v26[2];
          v16 = v26[1];
          if (v16 & 3) { // branch-flip
            if ((v16 & 3) != 2)
              return v38;
            v35 = (unsigned long)(v16 >> 4 & 3);
            *(unsigned int *)&v28[v35 * 4] = *(unsigned int *)&v28[v35 * 4] ^ *(unsigned int *)&v26[2];
            v27 = &v26[6];
          }
          else {
            *(unsigned int *)&v28[(unsigned long)(v16 >> 4 & 3) * 4] = *(unsigned int *)&v28[(unsigned long)(v16 >> 4 & 3) * 4] ^ *(unsigned int *)&v28[(unsigned long)(v16 >> 6) * 4];
            v27 = &v26[3];
          }
          goto label_1345;
        
      }
    }
label_1470:
    v32[0] = &v26[2];
    v16 = v26[1];
    if (v16 & 3) { // branch-flip
      if ((v16 & 3) != 2) {
label_1540:
        return v38;
      }
      v35 = (unsigned long)(v16 >> 4 & 3);
      *(int *)&v28[v35 * 4] = *(int *)&v28[v35 * 4] - *(int *)&v26[2];
      v27 = &v26[8];
    }
    else {
      v35 = (unsigned long)(v16 >> 4 & 3);
      *(int *)&v28[v35 * 4] = *(int *)&v28[v35 * 4] - *(int *)&v28[(unsigned long)(v16 >> 6) * 4];
      v27 = &v26[3];
    }
  } while( true );
label_1410:
  v28._0_16_ = v31._0_16_;
  v31._0_16_ = v12;
  v16 = v26[1] & 3;
  if (v16 == 2) goto label_1801;
  if (v16 == 3) {
    v27 = &v26[10];
    goto label_1345;
  }
  v27 = &v26[3];
  v26 = &v26[8];
  if (v16 != 1)
    v26 = v27;
  v16 = *v26;
  v39 = 0;
  if (v16 == 0xaa) goto label_1470;
  goto label_1359;
}

