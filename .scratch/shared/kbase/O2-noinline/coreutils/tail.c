// Function: sub_29a0 @ 0x29a0
unsigned long sub_29a0(int a0,unsigned long *a1)
{
  bool v1;
  char *v10;
  unsigned long v11; // rax
  unsigned long v12; // rax
  unsigned long v13; // rax
  unsigned long v14;
  unsigned long v15; // rax
  unsigned long *v16;
  unsigned int v17; // ebx
  long v18; // stack - 0xf0
  int v19; // stack - 0xf4
  unsigned char v2; // al
  unsigned long v20; // stack - 0xe8
  char v21 [24];
  unsigned long v22; // stack - 0xe0
  unsigned long *v23;
  long v24; // stack - 0x100
  unsigned int v25; // stack - 0xc0
  unsigned int v3; // eax
  int v4; // eax
  int v5; // eax
  unsigned long *v6; // rax
  unsigned long *v7;
  unsigned long *v8; // rax
  int *v9; // rax
  
  v19 = 0;
  v18 = 10;
  v20 = dat_f4f0;
  sub_91a0(*a1);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  sub_d460(sub_7b00);
  dat_1510e = '\0';
  dat_15119 = 1;
  dat_15115 = 0;
  dat_15116 = '\0';
  dat_15118 = '\0';
  dat_15114 = 10;
  v2 = sub_3bc0(a0,a1,&v18);
  a0 -= (unsigned int)v2;
  sub_7080(a0,&a1[v2],&v18,&v19,&v20);
  if ((dat_15116) && (v18))
    v18 -= 1;
  v16 = (unsigned long *)0x15028;
  v24 = 1;
  if (optind < a0) {
    v16 = &(&a1[v2])[optind];
    v24 = (long)(a0 - optind);
  }
  v1 = 0;
  v23 = &v16[v24];
  v7 = v16;
  do {
    v10 = (char *)*v7;
    if (!strcmp(v10,"-"))
      v1 = 1;
    v7 = &v7[1];
  } while (v23 != v7);
  if (v1) {
    if (dat_15020 == 1) {
      v14 = sub_b410(4,"-");
      error(1,0,dcgettext(NULL,"cannot follow %s by name",5),v14);
      return v15;
    }
    if (dat_15118) {
      if (((((dat_15110) || (dat_15020 != 2)) || (v24 != 1)) || ((fstat(0,v21) || ((v25 & 0xf000) == 0x8000)))) && (isatty(0)))
        error(0,0,dcgettext(NULL,"warning: following standard input indefinitely is ineffective",5));
      goto label_2b55;
    }
    if (v18) goto label_2b6a;
  }
  else {
label_2b55:
    if ((v18) || (dat_15118)) goto label_2b6a;
  }
  if (!dat_15116)
    return 0;
label_2b6a:
  v6 = (unsigned long *)sub_c0c0(v24,0x60);
  v7 = v6;
  do {
    v14 = *v16;
    v16 = &v16[1];
    v8 = &v7[0xc];
    *v7 = v14;
    v7 = v8;
  } while (v16 != v23);
  if ((v19 == 1) || ((v24 != 1 && (!v19))))
    dat_15115 = 1;
  v17 = 1;
  v16 = v6;
  do {
    v23 = &v16[0xc];
    v3 = sub_6ab0(v16,v18);
    v17 &= v3;
    v16 = v23;
  } while (&v6[v24 * 0xc] != v23);
  if ((dat_15118) && (sub_3a50(v6,v24))) {
    if (fstat(1,v21) < 0) {
      v10 = dcgettext(NULL,"standard output",5);
      error(1,*__errno_location(),v10);
      return v12;
    }
    dat_15117 = (v25 & 0xf000) == 0x1000;
    if ((((!dat_1510c) && (!sub_3ae0(v6,v24))) && (!sub_38d0(v6,v24))) && (((sub_3900(v6) && (!sub_4b40(v6))) && ((!sub_3930(v6,v24) && (((char)v17 || (dat_15020 != 2)))))))) {
      v4 = inotify_init();
      v9 = __errno_location();
      if (0 <= v4) {
        v5 = fflush_unlocked(stdout);
        if (v5) {
          v10 = dcgettext(NULL,"write error",5);
          error(1,*v9,v10);
          return v13;
        }
        sub_5880(v20,v4,v6,v24,&v22);
        sub_8a10(v22);
        close(v4);
        *v9 = 0;
      }
      v10 = dcgettext(NULL,"inotify cannot be used, reverting to polling",5);
      error(0,*v9,v10);
    }
    dat_1510c = '\x01';
    sub_6220(v20,v6,v24);
  }
  if ((dat_1510e) && (close(0) <= -1)) {
    error(1,*__errno_location(),"-");
    return v11;
  }
  return (unsigned long)(v17 ^ 1);
}


// Function: sub_3830 @ 0x3830
void sub_3830(long a0,unsigned int a1,unsigned long a2,unsigned long *a3,unsigned int a4)
{
  unsigned int v1;
  unsigned long v2;
  char v3 [16];
  
  *(unsigned int *)(a0 + 0x38) = a1;
  *(unsigned long *)(a0 + 8) = a2;
  v3 = sub_b7b0(a3);
  *(long *)(a0 + 0x10) = SUB168(v3,0);
  v2 = *a3;
  *(long *)(a0 + 0x18) = SUB168(v3,8);
  *(unsigned long *)(a0 + 0x20) = v2;
  *(unsigned long *)(a0 + 0x28) = a3[1];
  v1 = *(unsigned int *)&a3[3];
  *(unsigned int *)(a0 + 0x40) = a4;
  *(unsigned int *)(a0 + 0x30) = v1;
  *(unsigned long *)(a0 + 0x58) = 0;
  *(char *)(a0 + 0x34) = 0;
}


// Function: sub_3880 @ 0x3880
unsigned char sub_3880(long a0,long a1) // early-return
{
  unsigned char v1; // al
  unsigned char *v2; // rcx
  unsigned char *v3; // rdi
  
  if ((dat_1511a) && (dat_15020 == 1))
    return dat_1511a;
  v3 = (unsigned char *)(a0 + 0x34);
  v2 = &v3[a1 * 0x60];
  while( true ) {
    if (0 <= *(int *)&v3[4])
      return 1;
    v1 = (*v3 ^ 1) & dat_1511a;
    if (v1) break;
    v3 = &v3[0x60];
    if (v3 == v2)
      return 0;
  }
  return v1;
}


// Function: sub_38d0 @ 0x38d0
char sub_38d0(long a0,long a1)
{
  char *v1; // rax
  char *v2; // rdi
  
  v2 = (char *)(a0 + 0x35);
  v1 = &v2[a1 * 0x60];
  while ((*(int *)&v2[3] <= -1 || (!*v2))) {
    v2 = &v2[0x60];
    if (v2 == v1)
      return '\0';
  }
  return *v2;
}


// Function: sub_3900 @ 0x3900
unsigned long sub_3900(long a0,long a1)
{
  char *v1; // rax
  char *v2; // rdi
  
  v2 = (char *)(a0 + 0x35);
  v1 = &v2[a1 * 0x60];
  while ((*(int *)&v2[3] < 0 || (*v2))) {
    v2 = &v2[0x60];
    if (v2 == v1)
      return 0;
  }
  return 1;
}


// Function: sub_3930 @ 0x3930
unsigned int sub_3930(long a0,long a1)
{
  unsigned int *v1; // rcx
  unsigned int v2; // edx
  unsigned int *v3; // rdi
  
  v3 = (unsigned int *)(a0 + 0x30);
  v1 = &v3[a1 * 0x18];
  while (((int)v3[2] <= -1 || (v2 = *v3 & 0xf000, v2 == 0x1000 || v2 == 0x8000))) {
    v3 = &v3[0x18];
    if (v1 == v3)
      return 0;
  }
  return CONCAT31((undefined3)(v3[2] >> 8),v2 != 0x1000 && v2 != 0x8000);
}


// Function: sub_3980 @ 0x3980
unsigned long sub_3980(long a0,unsigned long a1)
{
  return (unsigned long)(long)*(int *)(a0 + 0x44) % a1;
}


// Function: sub_39a0 @ 0x39a0
unsigned int sub_39a0(long a0,long a1)
{
  return CONCAT31((undefined3)((unsigned int)*(int *)(a1 + 0x44) >> 8),*(int *)(a0 + 0x44) == *(int *)(a1 + 0x44));
}


// Function: sub_3a10 @ 0x3a10
void sub_3a10(unsigned long a0)
{
  char *v1; // rdx
  
  v1 = "";
  if (!dat_15010)
    v1 = "\n";
  __printf_chk(1,"%s==> %s <==\n",v1,a0);
  dat_15010 = 0;
}


// Function: sub_3a50 @ 0x3a50
long sub_3a50(unsigned long *a0,long a1)
{
  char *v1;
  unsigned long *v2; // rbp
  long v3; // r12
  
  v3 = 0;
  v2 = &a0[a1 * 0xc];
  do {
    v1 = (char *)*a0;
    if (((strcmp(v1,"-")) || (*(char *)((long)a0 + 0x34))) || ((*(int *)&a0[7] <= -1 || ((*(unsigned int *)&a0[6] & 0xf000) != 0x1000))))
      v3 += 1;
    else {
      *(unsigned int *)&a0[7] = 0xffffffff;
      *(char *)((long)a0 + 0x34) = 1;
    }
    a0 = &a0[0xc];
  } while (v2 != a0);
  return v3;
}


// Function: sub_3ae0 @ 0x3ae0
unsigned long sub_3ae0(unsigned long *a0,long a1)
{
  char *v1;
  unsigned long *v2; // rbp
  
  v2 = &a0[a1 * 0xc];
  while ((*(char *)((long)a0 + 0x34) || (v1 = (char *)*a0, strcmp(v1,"-")))) {
    a0 = &a0[0xc];
    if (a0 == v2)
      return 0;
  }
  return 1;
}


// Function: sub_3b30 @ 0x3b30
void sub_3b30(unsigned int a0,unsigned long a1)
{
  unsigned long v1; // rax
  char *v2; // rax
  
  if (a0 + 1 <= 1)
    return;
  if (!close(a0))
    return;
  v1 = sub_b410(4,a1);
  v2 = dcgettext(NULL,"closing %s (fd=%d)",5);
  error(0,*__errno_location(),v2,v1,(unsigned long)a0); // tail-call
}


// Function: sub_3bc0 @ 0x3bc0
unsigned long sub_3bc0(int a0,long a1,unsigned long *a2) // return-dupe
{
  char v1;
  bool v10; // zf
  int v2; // eax
  char *v3;
  char *v4;
  unsigned long v5; // rax
  unsigned long v6; // rax
  char *v7;
  char v8; // bpl
  char v9; // r13b
  
  if (a0 != 2) {
    if (a0 != 3) { // branch-flip
      if (a0 != 4)
        return 0;
      v4 = *(char **)(a1 + 0x10);
    }
    else {
      v4 = *(char **)(a1 + 0x10);
      if ((*v4 != '-') || (!v4[1])) goto label_3bf5;
    }
    if (strcmp(v4,"--"))
      return 0;
  }
label_3bf5:
  v2 = sub_9110();
  v7 = *(char **)(a1 + 8);
  v4 = &v7[1];
  if (*v7 != '+') { // branch-flip
    if (*v7 != '-')
      return 0;
    if ((0x30db0 <= v2) && (!v7[(unsigned long)(v7[1] == 'c') + 1]))
      return 0;
    v8 = 0;
  }
  else {
    v8 = 1;
    if ((unsigned int)(v2 - 0x30db0U) <= 0x2b8)
      return 0;
  }
  v1 = v7[1];
  v7 = v4;
  while ((unsigned int)((int)v1 - 0x30U) <= 9) {
    v3 = &v7[1];
    v7 = &v7[1];
    v1 = *v3;
  }
  if (v1 != 'c') { // branch-flip
    if (v1 != 'l') { // branch-flip
      if (v1 != 'b') {
        v9 = 1;
        v6 = 10;
        v3 = v7;
        goto label_3c8b;
      }
      v9 = 0;
      v6 = 0x1400;
    }
    else {
      v9 = 1;
      v6 = 10;
    }
  }
  else {
    v9 = 0;
    v6 = 10;
  }
  v3 = &v7[1];
label_3c8b:
  v10 = *v3 == 'f';
  if (v10)
    v3 = &v3[1];
  if (*v3)
    return 0;
  if (v4 == v7) {
    *a2 = v6;
    dat_15116 = v8;
    dat_15118 = v10;
    dat_15119 = v9;
    return 1;
  }
  if (!(sub_c6c0(v4,0,10,a2,"b") & 0xfffffffd)) {
    dat_15116 = v8;
    dat_15118 = v10;
    dat_15119 = v9;
    return 1;
  }
  v6 = sub_b6d0(*(unsigned long *)(a1 + 8));
  v4 = dcgettext(NULL,"invalid number",5);
  error(1,*__errno_location(),"%s: %s",v4,v6);
  return v5;
}


// Function: sub_3da0 @ 0x3da0
void sub_3da0(void *a0,unsigned long a1)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  char *v3; // rax
  
  if (!a1)
    return;
  v1 = fwrite_unlocked(a0,1,a1,stdout);
  if (a1 <= v1)
    return;
  clearerr_unlocked(stdout);
  v2 = sub_b410(4,"standard output");
  v3 = dcgettext(NULL,"error writing %s",5);
  error(1,*__errno_location(),v3,v2);
}


// Function: sub_3e30 @ 0x3e30
unsigned long sub_3e30(unsigned long a0,unsigned int a1,long a2,long *a3) // return-dupe
{
  char *v1;
  long v2; // rax
  void *v3; // rax
  unsigned long v4; // rax
  char *v5; // rax
  char v6 [8200];
  char *v7;
  int v8; // r12d
  
  if (!a2)
    return 0;
  while( true ) {
    v2 = sub_b6e0(a1,v6,_DT_INIT);
    if (!v2)
      return 0xffffffff;
    if (v2 == -1) break;
    *a3 = *a3 + v2;
    v8 = (int)dat_15114;
    v1 = &v6[v2];
    v7 = v6;
    while (v3 = memchr(v7,v8,(long)v1 - (long)v7), v3) {
      v7 = (char *)((long)v3 + 1);
      a2 -= 1;
      if (!a2) {
        if (v1 <= v7)
          return 0;
        sub_3da0(v7,(long)v1 - (long)v7);
        return 0;
      }
    }
  }
  v4 = sub_b410(4,a0);
  v5 = dcgettext(NULL,"error reading %s",5);
  error(0,*__errno_location(),v5,v4);
  return 1;
}


// Function: sub_3f80 @ 0x3f80
int * sub_3f80(char a0,unsigned long a1,unsigned int a2,void *a3)
{
  int *v1;
  unsigned long v2;
  int *v3; // rax
  void *v4;
  void *v5;
  char v6 [8200];
  long v7; // r12
  
  v7 = 0;
  v5 = a3;
  do {
    v4 = _DT_INIT;
    if (v5 <= (void *)0x2000)
      v4 = v5;
    v3 = (long)sub_b6e0(a2,v6,v4);
    if (v3 == (int *)0xffffffffffffffff) {
      v3 = __errno_location();
      v1 = v3;
      if (*v3 == 0xb) {
        v3 = (int *)v7;
        return v3;
      }
      v3 = (unsigned long)sub_b410(4,a1);
      v2 = v3;
      v3 = (char *)dcgettext(NULL,"error reading %s",5);
      error(1,*v1,(char *)v3,v2);
      return v3;
    }
    if (!v3) {
      v3 = (int *)v7;
      return v3;
    }
    if (a0)
      sub_3a10(a1);
    v7 += (long)v3;
    sub_3da0(v6,v3);
    if (a3 != (void *)0xffffffffffffffff) {
      v5 = &v5[-(long)v3];
      if (!v5) {
        v3 = (int *)v7;
        return v3;
      }
      if (a3 == (void *)0xfffffffffffffffe) {
        v3 = (int *)v7;
        return v3;
      }
    }
    a0 = '\0';
  } while( true );
}


// Function: sub_40c0 @ 0x40c0
unsigned long sub_40c0(unsigned long a0,unsigned int a1,unsigned long a2,long *a3) // return-dupe
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  char *v3; // rax
  char v4 [8200];
  
  if (!a2)
    return 0;
  do {
    v1 = sub_b6e0(a1,v4,_DT_INIT);
    if (!v1)
      return 0xffffffff;
    if (v1 == 0xffffffffffffffff) {
      v2 = sub_b410(4,a0);
      v3 = dcgettext(NULL,"error reading %s",5);
      error(0,*__errno_location(),v3,v2);
      return 1;
    }
    *a3 = *a3 + v1;
    if (a2 < v1) {
      sub_3da0(&v4[a2],v1 - a2);
      return 0;
    }
    a2 -= v1;
  } while (a2);
  return 0;
}


// Function: sub_41c0 @ 0x41c0
void sub_41c0(unsigned long a0,unsigned long a1,int a2,unsigned long a3)
{
  unsigned long v1; // rax
  int *v2; // rax
  unsigned long v3; // rax
  char v4 [24];
  char *v5; // rax
  
  if (0 <= (long)lseek())
    return;
  v1 = sub_9070(a1,v4);
  v2 = __errno_location();
  if (a2 != 1) { // branch-flip
    v3 = sub_b510(0,3,a3);
    v5 = "%s: cannot seek to offset %s";
  }
  else {
    v3 = sub_b510(0,3,a3);
    v5 = "%s: cannot seek to relative offset %s";
  }
  v5 = dcgettext(NULL,v5,5);
  error(0,*v2,v5,v3,v1);
  exit(1); // no-return
}


// Function: sub_4290 @ 0x4290
unsigned long sub_4290(unsigned long a0,unsigned int a1,long a2,long a3,long a4,long *a5) // return-dupe
{
  long v1;
  long v10; // rsi
  long v11; // stack - 0x2050
  long v2; // rax
  long v3; // rax
  unsigned long v4; // rax
  char *v5; // rax
  long v6;
  int v7; // ebx
  void *v8; // rbx
  char v9 [8200];
  
  if (!a2)
    return 1;
  v8 = (void *)((a4 - a3) % 0x2000);
  if (!v8)
    v8 = _DT_INIT;
  v10 = a4 - (long)v8;
  v11 = a4;
  sub_41c0(a1,v10,0,a0);
  v2 = sub_b6e0(a1,v9,v8);
  if (v2 != -1) {
    v7 = (int)dat_15114;
    *a5 = v10 + v2;
    if ((v2) && (v9[v2 + -1] != dat_15114))
      a2 -= 1;
    while( true ) {
      v6 = v2;
      while ((v6 && (v3 = memrchr(v9,v7), v3))) {
        v1 = a2 + -1;
        v6 = v3 - (long)v9;
        if (!a2) {
          sub_3da0(v3 + 1,(v2 + -1) - v6);
          v2 = sub_3f80(0,a0,a1,(v11 - v2) - v10);
          *a5 = *a5 + v2;
          return 1;
        }
        a2 = v1;
      }
      if (v10 == a3) {
        sub_41c0(a1,v10,0,a0);
        *a5 = v10 + sub_3f80(0,a0,a1,v11);
        return 1;
      }
      v10 -= 0x2000;
      sub_41c0(a1,v10,0,a0);
      v2 = sub_b6e0(a1,v9,_DT_INIT);
      if (v2 == -1) break;
      *a5 = v10 + v2;
      if (!v2)
        return 1;
      v7 = (int)dat_15114;
    }
  }
  v4 = sub_b410(4,a0);
  v5 = dcgettext(NULL,"error reading %s",5);
  error(0,*__errno_location(),v5,v4);
  return 0;
}


// Function: sub_4510 @ 0x4510
unsigned long sub_4510(unsigned long a0,unsigned int a1,unsigned long a2,long *a3)
{
  unsigned long v1;
  void *v10; // r13
  void *v11;
  unsigned long v12;
  void *v2;
  void *v3;
  unsigned long v4;
  unsigned long v5; // rax
  char *v6; // rax
  void *v7;
  long v8;
  unsigned long v9; // r12
  
  v2 = (void *)sub_bfa0(0x2010);
  *(unsigned long *)((long)v2 + 0x2000) = 0;
  *(unsigned long *)((long)v2 + 0x2008) = 0;
  v3 = (void *)sub_bfa0(0x2010);
  v7 = v3;
  v11 = v2;
  v12 = 0;
  while (v10 = v7, v7 = v2, v4 = sub_b6e0(a1,v3,_DT_INIT), v4 - 1 <= 0xfffffffffffffffd) {
    v12 += v4;
    *a3 = *a3 + v4;
    *(unsigned long *)((long)v10 + 0x2000) = v4;
    v8 = *(long *)((long)v11 + 0x2000);
    *(unsigned long *)((long)v10 + 0x2008) = 0;
    v2 = v7; // branch-flip
    if (_DT_INIT <= (void *)(v8 + v4)) {
      *(void **)((long)v11 + 0x2008) = v10;
      v4 = v12 - *(long *)((long)v7 + 0x2000);
      v11 = v10; // branch-flip
      if (v4 <= a2) {
        v3 = (void *)sub_bfa0(0x2010);
        v7 = v3;
      }
      else {
        v2 = *(void **)((long)v7 + 0x2008);
        v3 = v7;
        v12 = v4;
      }
    }
    else {
      memcpy((void *)(v8 + (long)v11),v3,v4);
      *(long *)((long)v11 + 0x2000) = *(long *)((long)v11 + 0x2000) + *(long *)((long)v10 + 0x2000);
      v7 = v10;
    }
  }
  free(v10);
  if (v4 != 0xffffffffffffffff) { // branch-flip
    v8 = *(long *)((long)v7 + 0x2000);
    v4 = v12 - v8;
    v2 = v7;
    while (v1 = v4, a2 < v1) {
      v2 = *(void **)((long)v2 + 0x2008);
      v8 = *(long *)((long)v2 + 0x2000);
      v4 = v1 - v8;
      v12 = v1;
    }
    v3 = v2;
    if (a2 < v12) {
      v8 += a2 - v12;
      v3 = (void *)((long)v2 + (v12 - a2));
    }
    sub_3da0(v3,v8);
    for (v8 = *(long *)((long)v2 + 0x2008); v8; v8 = *(long *)(v8 + 0x2008)) {
      sub_3da0(v8,*(unsigned long *)&_DT_INIT[v8]);
    }
    v9 = 1;
  }
  else {
    v5 = sub_b410(4,a0);
    v6 = dcgettext(NULL,"error reading %s",5);
    v9 = 0;
    error(0,*__errno_location(),v6,v5);
    if (!v7)
      return 0;
  }
  do {
    v2 = *(void **)((long)v7 + 0x2008);
    free(v7);
    v7 = v2;
  } while (v2);
  return v9;
}


// Function: sub_4760 @ 0x4760
unsigned long sub_4760(unsigned long a0,unsigned int a1,unsigned long a2,long *a3)
{
  unsigned long v1;
  void *v10;
  unsigned long v11;
  long v12; // r14
  void *v13;
  void *v2;
  long v3;
  unsigned long v4;
  unsigned long v5; // rax
  char *v6; // rax
  unsigned long v7; // rdx
  int v8; // ebp
  void *v9;
  
  v2 = (void *)sub_bfa0(0x2018);
  *(unsigned long *)((long)v2 + 0x2008) = 0;
  *(unsigned long *)((long)v2 + 0x2000) = 0;
  *(unsigned long *)((long)v2 + 0x2010) = 0;
  v9 = v2;
  v11 = 0;
  v13 = (void *)sub_bfa0(0x2018);
  while (v10 = v2, v3 = sub_b6e0(a1,v13,_DT_INIT), (unsigned long)(v3 - 1U) <= 0xfffffffffffffffd) {
    *(long *)((long)v13 + 0x2000) = v3;
    v8 = (int)dat_15114;
    *a3 = *a3 + v3;
    *(unsigned long *)((long)v13 + 0x2008) = 0;
    *(unsigned long *)((long)v13 + 0x2010) = 0;
    v2 = v13;
    while (v2 = memchr(v2,v8,(long)v13 + (v3 - (long)v2)), v2) {
      *(long *)((long)v13 + 0x2008) = *(long *)((long)v13 + 0x2008) + 1;
      v2 = (void *)((long)v2 + 1);
    }
    v11 += *(long *)((long)v13 + 0x2008);
    v2 = v10; // branch-flip
    if (_DT_INIT <= (void *)(*(unsigned long *)((long)v13 + 0x2000) + *(long *)((long)v9 + 0x2000))) {
      *(void **)((long)v9 + 0x2010) = v13;
      v4 = v11 - *(long *)((long)v10 + 0x2008);
      v9 = v13; // branch-flip
      if (v4 <= a2)
        v13 = (void *)sub_bfa0(0x2018);
      else {
        v2 = *(void **)((long)v10 + 0x2010);
        v11 = v4;
        v13 = v10;
      }
    }
    else {
      memcpy((void *)(*(long *)((long)v9 + 0x2000) + (long)v9),v13,*(unsigned long *)((long)v13 + 0x2000));
      *(long *)((long)v9 + 0x2000) = *(long *)((long)v9 + 0x2000) + *(long *)((long)v13 + 0x2000);
      *(long *)((long)v9 + 0x2008) = *(long *)((long)v9 + 0x2008) + *(long *)((long)v13 + 0x2008);
    }
  }
  free(v13);
  if (v3 != -1) { // branch-flip
    v3 = *(long *)((long)v9 + 0x2000);
    v4 = CONCAT71((undefined7)((unsigned long)v7 >> 8),!v3 || !a2) & 0xffffffff;
    if (v3 && a2) {
      if (*(char *)((long)v9 + v3 + -1) != dat_15114) {
        *(long *)((long)v9 + 0x2008) = *(long *)((long)v9 + 0x2008) + 1;
        v11 += 1;
      }
      v4 = v11 - *(long *)((long)v10 + 0x2008);
      v2 = v10;
      while (v1 = v4, a2 < v1) {
        v2 = *(void **)((long)v2 + 0x2010);
        v4 = v1 - *(long *)((long)v2 + 0x2008);
        v11 = v1;
      }
      v3 = *(long *)((long)v2 + 0x2000);
      v9 = v2;
      if (a2 < v11) {
        v12 = v11 - a2;
        v8 = (int)dat_15114;
        do {
          v9 = (void *)((long)rawmemchr(v9,v8) + 1);
          v12 -= 1;
        } while (v12);
      }
      sub_3da0(v9,(long)v2 + (v3 - (long)v9));
      for (v3 = *(long *)((long)v2 + 0x2010); v3; v3 = *(long *)(v3 + 0x2010)) {
        sub_3da0(v3,*(unsigned long *)&_DT_INIT[v3]);
      }
      v4 = 1;
      goto label_4910;
    }
  }
  else {
    v4 = 0;
    v5 = sub_b410(4,a0);
    v6 = dcgettext(NULL,"error reading %s",5);
    error(0,*__errno_location(),v6,v5);
  }
  if (!v10)
    return v4;
label_4910:
  do {
    v2 = *(void **)((long)v10 + 0x2010);
    free(v10);
    v10 = v2;
  } while (v2);
  return v4;
}


// Function: sub_4a40 @ 0x4a40
unsigned long sub_4a40(unsigned long a0,unsigned long a1) // return-dupe x3
{
  int v1; // eax
  int *v2; // rax
  unsigned long v3; // rax
  char *v4; // rax
  unsigned long v5 [15]; // stack - 0xa8
  
  if (!fstatfs(a0,v5)) {
    v1 = sub_2fb0(v5[0]);
    if (1 <= v1) { // branch-flip
      if (v1 == 1)
        return 0;
    }
    else if (-1 <= v1)
      return 1;
    __assert_fail("!\"unexpected return value from is_local_fs_type\"","src/tail.c",0x3cc,"fremote"); // no-return
  }
  v2 = __errno_location();
  if (*v2 == 0x26)
    return 1;
  v3 = sub_b410(4,a1);
  v4 = dcgettext(NULL,"cannot determine location of %s. reverting to polling",5);
  error(0,*v2,v4,v3);
  return 1;
}


// Function: sub_4b40 @ 0x4b40
unsigned long sub_4b40(unsigned long *a0,long a1)
{
  char *v1;
  char v2 [24];
  unsigned long *v3; // rbp
  unsigned int v4; // stack - 0xa0
  
  v3 = &a0[a1 * 0xc];
  while ((v1 = (char *)*a0, lstat(v1,v2) || ((v4 & 0xf000) != 0xa000))) {
    a0 = &a0[0xc];
    if (a0 == v3)
      return 0;
  }
  return 1;
}


// Function: sub_4bd0 @ 0x4bd0
void sub_4bd0(void)
{
  raise(0xd);
  exit(1); // no-return
}


// Function: sub_4bf0 @ 0x4bf0
void sub_4bf0(void)
{
  unsigned long v1; // stack - 0x18
  
  if (dat_15117) {
    v1 = 1;
    if ((0 <= (int)poll(&v1,1,0)) && (v1 & 0x18000000000000))
      sub_4bd0(); // no-return
  }
}


// Function: sub_5090 @ 0x5090
void sub_5090(unsigned long *a0,unsigned long *a1) // return-dupe x3
{
  unsigned long v1;
  char v10 [16];
  long v11; // stack - 0x98
  int v2;
  unsigned long v3; // rax
  long v4; // rax
  char *v5; // rax
  int *v6; // rax
  unsigned long v7;
  char v8 [48];
  bool v9;
  
  if (*(int *)&a0[7] == -1)
    return;
  v3 = sub_5050(*a0);
  v2 = *(int *)&a0[7];
  if (fstat(v2,v8)) {
    v6 = __errno_location();
    *(int *)((long)a0 + 0x3c) = *v6;
    sub_3b30(*(unsigned int *)&a0[7],v3);
    *(unsigned int *)&a0[7] = 0xffffffff;
    return;
  }
  if ((*(unsigned int *)&a0[6] & 0xf000) == 0x8000) {
    if ((long)a0[1] <= v11) { // branch-flip
      if (v11 == a0[1]) {
        v10 = sub_b7b0(v8);
        v7 = a0[2];
        v1 = a0[3];
        if (!sub_b800(v7,v1,SUB168(v10,0),SUB168(v10,8)))
          return;
      }
    }
    else {
      v7 = sub_b510(0,3,v3);
      error(0,0,dcgettext(NULL,"%s: file truncated",5),v7);
      sub_41c0(*(unsigned int *)&a0[7],0,0,v3);
      a0[1] = 0;
    }
  }
  v9 = 0;
  if (dat_15115)
    v9 = (unsigned long *)*a1 != a0;
  v4 = sub_3f80(v9,v3,*(unsigned int *)&a0[7],0xffffffffffffffff);
  a0[1] = a0[1] + v4;
  if (!v4)
    return;
  *a1 = a0;
  v2 = fflush_unlocked(stdout);
  if (!v2)
    return;
  v5 = dcgettext(NULL,"write error",5);
  error(1,*__errno_location(),v5);
}


// Function: sub_5270 @ 0x5270
void sub_5270(unsigned long *a0,unsigned char a1)
{
  unsigned int v1;
  char *v10;
  long v11; // stack - 0xd8
  unsigned long v12; // r8
  long v13; // stack - 0xd0
  unsigned int v14; // stack - 0xc0
  unsigned int v2;
  char v3;
  char v4; // al
  char v5;
  int v6; // eax
  int v7; // eax
  unsigned int *v8; // rax
  unsigned long v9;
  
  v10 = (char *)*a0;
  v6 = strcmp(v10,"-");
  v1 = *(unsigned int *)((long)a0 + 0x3c);
  v5 = *(char *)((long)a0 + 0x36);
  if (v6) { // branch-flip
    v7 = sub_7ea0(v10,(unsigned int)(a1 ^ 1) << 0xb);
    v2 = *(unsigned int *)((long)a0 + 0x3c);
  }
  else {
    v7 = 0;
    v2 = v1;
  }
  v4 = sub_5260(*(unsigned int *)&a0[7],v2);
  v3 = dat_1511a;
  if (!v4)
    __assert_fail("valid_file_spec (f)","src/tail.c",0x3e2,"recheck"); // no-return
  if (dat_1511a) { // branch-flip
    if (v7 != -1) { // branch-flip
      *(char *)((long)a0 + 0x36) = 1;
      if ((dat_1510c) || ((v10 = (char *)*a0, lstat(v10,&v11) || ((v14 & 0xf000) != 0xa000)))) goto label_53ca;
    }
    else {
      *(char *)((long)a0 + 0x36) = 0;
      v10 = (char *)*a0;
      if (dat_1510c) {
        *(int *)((long)a0 + 0x3c) = *__errno_location();
        goto label_54d9;
      }
      if ((lstat(v10,&v11)) || ((v14 & 0xf000) != 0xa000)) goto label_5321;
    }
  }
  else {
    *(char *)((long)a0 + 0x36) = 1;
    if ((dat_1510c) || ((v10 = (char *)*a0, lstat(v10,&v11) || ((v14 & 0xf000) != 0xa000)))) {
      if (v7 != -1) {
label_53ca:
        if (0 <= fstat(v7,&v11)) {
          if ((!((v14 & 0xf000) - 0x1000 & 0xffffe000)) || ((v14 & 0xb000) == 0x8000)) {
            v9 = *a0;
            v5 = sub_4a40(v7,sub_5050(v9));
            *(char *)((long)a0 + 0x35) = v5;
            if ((v5) && (!dat_1510c)) {
              *(unsigned int *)((long)a0 + 0x3c) = 0xffffffff;
              v9 = *a0;
              v9 = sub_b410(4,sub_5050(v9));
              error(0,0,dcgettext(NULL,"%s has been replaced with an untailable remote file",5),v9);
              *(unsigned short *)((long)a0 + 0x34) = 0x101;
              goto label_5348;
            }
            *(unsigned int *)((long)a0 + 0x3c) = 0;
            if (v1 & 0xfffffffd) { // branch-flip
              if (*(int *)&a0[7] != -1)
                __assert_fail("f->fd == -1","src/tail.c",0x42f,"recheck"); // no-return
              v9 = *a0;
              v9 = sub_b410(4,sub_5050(v9));
              v10 = "%s has become accessible";
            }
            else {
              v9 = *a0;
              if (*(int *)&a0[7] != -1) {
                if ((a0[5] == v13) && (a0[4] == v11)) {
                  sub_3b30(v7,sub_5050(v9));
                  return;
                }
                v9 = sub_b410(4,sub_5050(v9));
                error(0,0,dcgettext(NULL,"%s has been replaced;  following new file",5),v9);
                v9 = sub_5050(*a0);
                sub_3b30(*(unsigned int *)&a0[7],v9);
                goto label_5484;
              }
              v9 = sub_b410(4,sub_5050(v9));
              v10 = "%s has appeared;  following new file";
            }
            error(0,0,dcgettext(NULL,v10,5),v9);
label_5484:
            v12 = (unsigned long)a1;
            if (!v6)
              v12 = 0xffffffff;
            sub_3830(a0,v7,0,&v11,v12);
            v9 = *a0;
            sub_41c0(v7,0,0,sub_5050(v9));
            return;
          }
          *(unsigned int *)((long)a0 + 0x3c) = 0xffffffff;
          *(char *)((long)a0 + 0x36) = 0;
          if ((v3) && (dat_15020 == 1)) {
            *(char *)((long)a0 + 0x34) = 0;
            if (v5) // branch-flip
              v10 = "";
            else {
              v10 = "";
              if (v1 == 0xffffffff) goto label_5348;
            }
          }
          else {
            *(char *)((long)a0 + 0x34) = 1;
            if ((!v5) && (v1 == 0xffffffff)) goto label_5348;
            v10 = dcgettext(NULL,"; giving up on this name",5);
          }
          v9 = *a0;
          v9 = sub_b410(4,sub_5050(v9));
          error(0,0,dcgettext(NULL,"%s has been replaced with an untailable file%s",5),v9,v10);
          goto label_5348;
        }
      }
label_5321:
      v8 = (unsigned int *)__errno_location();
      v10 = (char *)*a0;
      v2 = *v8;
      *(unsigned int *)((long)a0 + 0x3c) = v2;
      if (*(char *)((long)a0 + 0x36)) {
        if (v2 != v1) {
          v9 = sub_b510(0,3,sub_5050(v10));
          error(0,*v8,"%s",v9);
        }
        goto label_5348;
      }
label_54d9:
      if (v5) {
        v9 = sub_b410(4,sub_5050(v10));
        v10 = dcgettext(NULL,"%s has become inaccessible",5);
        error(0,*(int *)((long)a0 + 0x3c),v10,v9);
      }
      goto label_5348;
    }
  }
  *(unsigned int *)((long)a0 + 0x3c) = 0xffffffff;
  v9 = *a0;
  *(char *)((long)a0 + 0x34) = 1;
  v9 = sub_b410(4,sub_5050(v9));
  error(0,0,dcgettext(NULL,"%s has been replaced with an untailable symbolic link",5),v9);
label_5348:
  v9 = *a0;
  sub_3b30(v7,sub_5050(v9));
  v9 = sub_5050(*a0);
  sub_3b30(*(unsigned int *)&a0[7],v9);
  *(unsigned int *)&a0[7] = 0xffffffff;
  return;
}


// Function: sub_5880 @ 0x5880
void sub_5880(double a0,unsigned int a1,long *a2,long a3,long *a4) // return-dupe
{
  char v1;
  int v10;
  long v11;
  unsigned long v12; // rax
  long v13; // rax
  unsigned int *v14; // rax
  long *v15;
  unsigned long v16; // rax
  char *v17;
  long *v18; // rax
  int *v19;
  unsigned int v2;
  long *v20; // stack - 0x140
  unsigned int v21; // stack - 0xd8
  char v22 [68];
  char *v23;
  unsigned long v24;
  unsigned long v25; // stack - 0x1a8
  unsigned long v26; // stack - 0x198
  int *v27; // stack - 0x180
  long v28; // stack - 0x168
  int v29; // stack - 0x15c
  unsigned int v3;
  int v30; // stack - 0xf4
  unsigned short v31; // stack - 0xd4
  unsigned short v32; // stack - 0xd2
  long v33; // stack - 0xd0
  int v4;
  long v5;
  long v6;
  bool v7;
  bool v8;
  unsigned int v9; // eax
  
  v11 = sub_8850(a3,0,sub_3980,sub_39a0,0);
  if (!v11)
    sub_c360(); // no-return, return-dupe
  v8 = 0;
  v7 = 0;
  *a4 = v11;
  v25._0_4_ = dat_15020;
  v9 = 2;
  if (dat_15020 == 1)
    v9 = 0xc06;
  v24 = 0;
  v15 = a2;
  do {
    if (!*(char *)((long)v15 + 0x34)) {
      v17 = (char *)*v15;
      v12 = strlen(v17);
      *(unsigned int *)((long)v15 + 0x44) = 0xffffffff;
      if (v24 < v12)
        v24 = v12;
      if ((int)v25 == 1) {
        v13 = sub_7bb0(v17);
        v1 = v17[v13];
        v23 = ".";
        v15[10] = sub_7990(v17) - (long)v17;
        v17[v13] = '\0';
        if (v13)
          v23 = (char *)*v15;
        *(unsigned int *)&v15[9] = inotify_add_watch(a1,v23,0x784);
        *(char *)(*v15 + v13) = v1;
        if ((int)v15[9] < 0) {
          v19 = __errno_location();
          if (*v19 != 0x1c) {
            v16 = sub_b410(4,*v15);
            v17 = dcgettext(NULL,"cannot watch parent directory of %s",5);
            error(0,*v19,v17,v16); // tail-call
            return;
          }
label_5b10:
          v17 = "inotify resources exhausted";
          error(0,0,dcgettext(NULL,v17,5)); // return-dupe, tail-call
          return;
        }
        v17 = (char *)*v15;
      }
      v10 = inotify_add_watch(a1,v17,v9);
      *(int *)((long)v15 + 0x44) = v10;
      if (0 <= v10) { // branch-flip
        if (!sub_8e70(v11,v15)) {
          sub_c360();
        }
        v7 = 1;
        v25._0_4_ = dat_15020;
      }
      else {
        if ((int)v15[7] != -1)
          v8 = 1;
        v14 = (unsigned int *)__errno_location();
        if ((*v14 & 0xffffffef) == 0xc) goto label_5b10;
        if (*(unsigned int *)((long)v15 + 0x3c) != *v14) {
          v16 = sub_b410(4,*v15);
          v17 = dcgettext(NULL,"cannot watch %s",5);
          error(0,*v14,v17,v16);
          v25._0_4_ = dat_15020;
        }
      }
    }
    v15 = &v15[0xc];
  } while (v15 != &a2[a3 * 0xc]);
  if ((int)v25 == 2) {
    if (v8)
      return;
    if (!v7)
      exit(1); // no-return
  }
  v20 = &a2[a3 * 0xc + -0xc];
  v15 = a2;
  do {
    if (!*(char *)((long)v15 + 0x34)) {
      if (dat_15020 != 1) { // branch-flip
        if ((((int)v15[7] != -1) && (v17 = (char *)*v15, !stat(v17,&v21))) && ((v15[4] != CONCAT26(v32,CONCAT24(v31,v21)) || (v15[5] != v33)))) {
          v11 = *v15;
          v16 = sub_b410(4,sub_5050(v11));
          v17 = dcgettext(NULL,"%s was replaced",5);
          error(0,*__errno_location(),v17,v16);
          return;
        }
      }
      else {
        sub_5270(v15,0);
      }
      sub_5090(v15,&v20);
    }
    v15 = &v15[0xc];
  } while (v15 != &a2[a3 * 0xc]);
  v28 = v24 + 0x11;
  v7 = 0;
  v26 = 0;
  v27 = (int *)sub_bfa0(v28);
  v25 = 0;
  v29 = 3;
label_5bf7:
  do {
    if (((dat_15020 == 1) && (!dat_1511a)) && (!sub_83f0(v11))) {
      error(1,0,dcgettext(NULL,"no files remaining",5));
      return;
    }
    if (v26 <= v25) {
      do {
        if (dat_15110) { // branch-flip
          if (v7)
            exit(0); // no-return
          v10 = kill(dat_15110,0);
          if ((v10) && (*__errno_location() != 1)) {
            v10 = 0;
            v7 = 1;
          }
          else {
            v10 = 0;
            if (0.0 < a0) {
              if (dat_f4e0 <= a0) goto label_5e10;
              v10 = (int)(dat_f4e8 * a0);
              v10 = (unsigned int)((double)v10 < dat_f4e8 * a0) + v10;
            }
          }
        }
        else {
label_5e10:
          v10 = -1;
        }
        v31 = 1;
        v33 = 1;
        v21 = a1;
        v10 = poll(&v21,(unsigned long)dat_15117 + 1,v10);
      } while (!v10);
      if (v10 < 0) {
        v17 = dcgettext(NULL,"error waiting for inotify and output events",5);
        error(1,*__errno_location(),v17);
        return;
      }
      if (v33._6_2_)
        sub_4bd0(); // no-return
      v26 = sub_b6e0(a1,v27,v28);
      if (v26) {
        v13 = 0x10;
        v19 = v27;
        if (v26 != 0xffffffffffffffff) goto label_5c1f;
        if (*__errno_location() != 0x16) {
label_5ed4:
          v17 = dcgettext(NULL,"error reading inotify event",5);
          error(1,*__errno_location(),v17);
          return;
        }
      }
      if (!v29) goto label_5ed4;
      v29 -= 1;
      v28 *= 2;
      v26 = 0;
      v27 = (int *)sub_bff0(v27,v28);
      v25 = 0;
      goto label_5bf7;
    }
    v13 = v25 + 0x10;
    v19 = (int *)((long)v27 + v25);
label_5c1f:
    v2 = v19[3];
    v10 = *v19;
    v25 = v13 + (unsigned long)v2;
    v3 = v19[1];
    if (v3 & 0x400) { // branch-flip
      if (v2) goto label_5c55;
      v15 = &a2[9];
      do {
        if ((int)*v15 == v10) {
          v17 = "directory containing watched file was removed";
          error(0,0,dcgettext(NULL,v17,5));
          return;
        }
        v15 = &v15[0xc];
      } while (&(&a2[9])[a3 * 0xc] != v15);
label_5e28:
      v30 = v10;
      v15 = (long *)sub_85f0(v11,v22);
      if (v15) {
        v2 = v19[1];
label_5cde:
        if (v2 & 0xe04) { // branch-flip
          if (v2 & 0x400) {
            inotify_rm_watch(a1,*(unsigned int *)((long)v15 + 0x44));
            sub_8ed0(v11,v15);
          }
          sub_5270(v15,0);
        }
        else {
          sub_5090(v15,&v20);
        }
      }
    }
    else {
      if (!v2) goto label_5e28;
label_5c55:
      v13 = 0;
      v15 = a2;
      do {
        if (((int)v15[9] == v10) && (v5 = v15[10], v6 = *v15, !strcmp((char *)&v19[4],(char *)(v5 + v6)))) {
          if (a3 == v13) goto label_5bf7;
          v15 = &a2[v13 * 0xc];
          if (!(v3 & 0x200)) {
            v10 = inotify_add_watch(a1,*v15,v9);
            if (v10 <= -1) {
              v14 = (unsigned int *)__errno_location();
              if ((*v14 & 0xffffffef) == 0xc) goto label_5b10;
              v16 = sub_b410(4,*v15);
              v17 = dcgettext(NULL,"cannot watch %s",5);
              error(0,*v14,v17,v16);
            }
            v4 = *(int *)((long)v15 + 0x44);
            if (v4 != v10) { // branch-flip
              if (0 <= v4) {
                inotify_rm_watch(a1);
                sub_8ed0(v11,v15);
              }
            }
            else if (0 <= v4) goto label_5cc7;
            *(int *)((long)v15 + 0x44) = v10;
            if (v10 == -1) goto label_5bf7;
            v18 = (long *)sub_8ed0(v11,v15);
            if ((v18) && (v15 != v18)) {
              if (dat_15020 == 1)
                sub_5270(v18,0);
              *(unsigned int *)((long)v18 + 0x44) = 0xffffffff;
              v16 = sub_5050(*v18);
              sub_3b30((int)v18[7],v16);
            }
            if (!sub_8e70(v11,v15)) {
              sub_c360();
            }
          }
label_5cc7:
          if (dat_15020 == 1)
            sub_5270(v15,0);
          v2 = v19[1];
          goto label_5cde;
        }
        v13 += 1;
        v15 = &v15[0xc];
      } while (a3 != v13);
    }
  } while( true );
}


// Function: sub_6220 @ 0x6220
void sub_6220(unsigned long a0,unsigned long *a1,long a2)
{
  unsigned int v1;
  int v10; // eax
  unsigned long v11; // rax
  char *v12; // rax
  unsigned long v13;
  int *v14; // rax
  long v15; // rax
  unsigned long *v16;
  unsigned long *v17; // rbx
  char v18 [24];
  long v19; // rbp
  unsigned long v2;
  unsigned int v20;
  bool v21;
  char v22 [16];
  long v23; // stack - 0x100
  unsigned int v24; // stack - 0xc0
  long v25; // stack - 0xa8
  unsigned long v3;
  bool v4;
  bool v5;
  bool v6;
  unsigned int v7; // eax
  unsigned int v8;
  int v9;
  
  if (dat_15110) { // branch-flip
    v21 = 0;
    v20 = 0;
  }
  else {
    v20 = 0;
    v21 = 0;
    if (dat_15020 == 2 && a2 == 1) {
      if (*(int *)&a1[7] != -1) { // branch-flip
        v21 = (*(unsigned int *)&a1[6] & 0xf000) != 0x8000;
        v20 = (unsigned int)v21;
      }
      else {
        v21 = 0;
      }
    }
  }
  v23 = a2 + -1;
  v5 = 0;
  do {
    v6 = 0;
    v19 = 0;
    v16 = a1;
    do {
      while( true ) {
        if (*(char *)((long)v16 + 0x34)) goto label_633a;
        v9 = *(int *)&v16[7];
        if (v9 <= -1) break;
        v11 = sub_5050(*v16);
        v1 = *(unsigned int *)&v16[6];
        v8 = v20;
        if (*(unsigned int *)&v16[8] != v20) {
          v7 = sub_7d50(v9,3);
          if (v20) { // branch-flip
            if (0 <= (int)v7) goto label_6309;
          }
          else if ((0 <= (int)v7) && ((v7 == (v7 | 0x800) || (sub_7d50(v9,4) != -1)))) {
label_6309:
            *(unsigned int *)&v16[8] = v20;
            goto label_6310;
          }
          v14 = __errno_location();
          if (((*(unsigned int *)&v16[6] & 0xf000) != 0x8000) || (*v14 != 1)) {
            v11 = sub_b510(0,3,v11);
            v12 = dcgettext(NULL,"%s: cannot change nonblocking mode",5);
            error(1,*v14,v12,v11);
            return;
          }
          v8 = *(unsigned int *)&v16[8];
        }
label_6310:
        v15 = -2;
        if (v8) { // branch-flip
label_631f:
          v15 = sub_3f80(0,v11,v9,v15);
          v16[1] = v16[1] + v15;
          v6 = (bool)(v6 | v15 != 0);
        }
        else {
          v10 = fstat(v9,v18);
          v8 = v24;
          if (!v10) {
            if ((*(unsigned int *)&v16[6] == v24) && (((v24 & 0xf000) != 0x8000 || (v16[1] == v25)))) {
              v22 = sub_b7b0(v18);
              v13 = v16[2];
              v2 = v16[3];
              if (!sub_b800(v13,v2,SUB168(v22,0),SUB168(v22,8))) {
                v3 = v16[0xb];
                v16[0xb] = v3 + 1;
                if ((dat_15018 <= v3) && (dat_15020 == 1)) {
                  sub_5270(v16,*(int *)&v16[8] != 0);
                  v16[0xb] = 0;
                }
                goto label_633a;
              }
            }
            v22 = sub_b7b0(v18);
            v16[0xb] = 0;
            v16[2] = SUB168(v22,0);
            v16[3] = SUB168(v22,8);
            *(unsigned int *)&v16[6] = v8;
            if (((v1 & 0xf000) == 0x8000) && (v25 < (long)v16[1])) {
              v13 = sub_b510(0,3,v11);
              error(0,0,dcgettext(NULL,"%s: file truncated",5),v13);
              sub_41c0(v9,0,0,v11);
              v16[1] = 0;
            }
            if ((v23 != v19) && (dat_15115))
              sub_3a10(v11);
            v23 = v19;
            if (*(int *)&v16[8]) // branch-flip
              v15 = -2;
            else {
              v15 = -1;
              if (((v1 & 0xf000) == 0x8000) && (*(char *)((long)v16 + 0x35)))
                v15 = v25 - v16[1];
            }
            goto label_631f;
          }
          *(unsigned int *)&v16[7] = 0xffffffff;
          v14 = __errno_location();
          *(int *)((long)v16 + 0x3c) = *v14;
          v11 = sub_b510(0,3,v11);
          error(0,*v14,"%s",v11);
          close(v9);
        }
label_633a:
        v19 += 1;
        v16 = &v16[0xc];
        if (a2 == v19) goto label_6380;
      }
      v19 += 1;
      v17 = &v16[0xc];
      sub_5270(v16,v20);
      v16 = v17;
    } while (a2 != v19);
label_6380:
    if (!sub_3880(a1,a2)) {
      error(0,0,dcgettext(NULL,"no files remaining",5)); // tail-call
      return;
    }
    v4 = (bool)(v6 ^ 1U | v21);
    if (v4) {
      v9 = fflush_unlocked(stdout);
      if (v9) {
        v12 = dcgettext(NULL,"write error",5);
        error(1,*__errno_location(),v12);
        return;
      }
      sub_4bf0();
      if (!v6) {
        if (v5)
          return;
        if (((dat_15110) && (v9 = kill(dat_15110,0), v9)) && (*__errno_location() != 1))
          v5 = v4;
        else if (sub_c4e0(a0)) {
          v12 = dcgettext(NULL,"cannot read realtime clock",5);
          error(1,*__errno_location(),v12);
          return;
        }
      }
    }
    else {
      sub_4bf0();
    }
  } while( true );
}


// Function: sub_6a90 @ 0x6a90
void sub_6a90(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long *a3)
{
  *a3 = 0;
  if (dat_15119) {
    sub_4c60(); // tail-call
    return;
  }
  sub_6820(); // tail-call
}


// Function: sub_6ab0 @ 0x6ab0
unsigned int sub_6ab0(unsigned long *a0,unsigned long a1)
{
  unsigned char v1;
  unsigned int v10; // stack - 0xc0
  int v2;
  unsigned int v3; // eax
  int v4; // eax
  int *v5; // rax
  unsigned long v6;
  char *v7;
  unsigned long v8; // stack - 0xe0
  char v9 [24];
  
  v7 = (char *)*a0;
  v2 = strcmp(v7,"-");
  if (v2) { // branch-flip
    v4 = sub_7ea0(v7,0);
    v1 = dat_1511a;
    if (!dat_1511a) {
      *(char *)((long)a0 + 0x36) = 1;
      v7 = (char *)*a0;
      if (v4 == -1) {
label_6c50:
        v5 = __errno_location();
        if (dat_15118) {
          *(unsigned int *)&a0[7] = 0xffffffff;
          v2 = *v5;
          *(unsigned char *)((long)a0 + 0x34) = v1 ^ 1;
          *(int *)((long)a0 + 0x3c) = v2;
          a0[5] = 0;
          a0[4] = 0;
        }
        v6 = *a0;
        v6 = sub_b410(4,sub_5050(v6));
        v7 = dcgettext(NULL,"cannot open %s for reading",5);
        error(0,*v5,v7,v6);
        return 0;
      }
      goto label_6b04;
    }
    v7 = (char *)*a0;
    if (v4 == -1) {
      *(char *)((long)a0 + 0x36) = 0;
      goto label_6c50;
    }
  }
  else {
    dat_1510e = 1;
    v4 = 0;
  }
  *(char *)((long)a0 + 0x36) = 1;
label_6b04:
  if (dat_15115) {
    sub_3a10(sub_5050(v7));
    v7 = (char *)*a0;
  }
  v3 = sub_6a90(sub_5050(v7),v4,a1,&v8);
  if (dat_15118) { // branch-flip
    *(unsigned int *)((long)a0 + 0x3c) = (v3 & 0xff) - 1;
    if (0 <= fstat(v4,v9)) { // branch-flip
      if (((v10 & 0xf000) - 0x1000 & 0xffffe000) && ((v10 & 0xb000) != 0x8000)) {
        *(unsigned int *)((long)a0 + 0x3c) = 0xffffffff;
        v7 = "";
        *(char *)((long)a0 + 0x36) = 0;
        *(unsigned char *)((long)a0 + 0x34) = dat_1511a ^ 1;
        if (dat_1511a ^ 1)
          v7 = dcgettext(NULL,"; giving up on this name",5);
        v6 = *a0;
        v6 = sub_b510(0,3,sub_5050(v6));
        error(0,0,dcgettext(NULL,"%s: cannot follow end of this type of file%s",5),v6,v7);
      }
      else if ((char)v3) {
        sub_3830(a0,v4,v8,v9,-(unsigned int)(v2 == 0) | 1);
        v6 = sub_5050(*a0);
        *(char *)((long)a0 + 0x35) = sub_4a40(v4,v6);
        return v3;
      }
    }
    else {
      v5 = __errno_location();
      v6 = *a0;
      *(int *)((long)a0 + 0x3c) = *v5;
      v6 = sub_b410(4,sub_5050(v6));
      v7 = dcgettext(NULL,"error reading %s",5);
      error(0,*v5,v7,v6);
    }
    v6 = *a0;
    v3 = 0;
    *(unsigned char *)((long)a0 + 0x34) = dat_1511a ^ 1;
    sub_3b30(v4,sub_5050(v6));
    *(unsigned int *)&a0[7] = 0xffffffff;
  }
  else if ((v2) && (close(v4))) {
    v6 = *a0;
    v6 = sub_b410(4,sub_5050(v6));
    v7 = dcgettext(NULL,"error reading %s",5);
    v3 = 0;
    error(0,*__errno_location(),v7,v6);
  }
  return v3;
}


// Function: sub_6e60 @ 0x6e60
void sub_6e60(int a0)
{
  void *v1;
  unsigned long v2;
  
  v2 = dat_15138;
  if (a0) // branch-flip
    __fprintf_chk(stderr,1,dcgettext(NULL,"Try \'%s --help\' for more information.\n",5),v2);
  else {
    __printf_chk(1,dcgettext(NULL,"Usage: %s [OPTION]... [FILE]...\n",5),v2);
    __printf_chk(1,dcgettext(NULL,"Print the last %d lines of each FILE to standard output.\nWith more than one FILE, precede each with a header giving the file name.\n",5),10);
    sub_39b0();
    sub_39e0();
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -c, --bytes=[+]NUM       output the last NUM bytes; or use -c +NUM to\n                             output starting with byte NUM of each file\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -f, --follow[={name|descriptor}]\n                           output appended data as the file grows;\n                             an absent option argument means \'descriptor\'\n  -F                       same as --follow=name --retry\n",5),v1);
    __printf_chk(1,dcgettext(NULL,"  -n, --lines=[+]NUM       output the last NUM lines, instead of the last %d;\n                             or use -n +NUM to output starting with line NUM\n      --max-unchanged-stats=N\n                           with --follow=name, reopen a FILE which has not\n                             changed size after N (default %d) iterations\n                             to see if it has been unlinked or renamed\n                             (this is the usual case of rotated log files);\n                             with inotify, this option is rarely useful\n",5),10,5);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --pid=PID            with -f, terminate after process ID, PID dies\n  -q, --quiet, --silent    never output headers giving file names\n      --retry              keep trying to open a file if it is inaccessible\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -s, --sleep-interval=N   with -f, sleep for approximately N seconds\n                             (default 1.0) between iterations;\n                             with inotify and --pid=P, check process P at\n                             least once every N seconds\n  -v, --verbose            always output headers giving file names\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -z, --zero-terminated    line delimiter is NUL, not newline\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --help        display this help and exit\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --version     output version information and exit\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nNUM may have a multiplier suffix:\nb 512, kB 1000, K 1024, MB 1000*1000, M 1024*1024,\nGB 1000*1000*1000, G 1024*1024*1024, and so on for T, P, E, Z, Y.\nBinary prefixes can be used, too: KiB=K, MiB=M, and so on.\n\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"With --follow (-f), tail defaults to following the file descriptor, which\nmeans that even if a tail\'ed file is renamed, tail will continue to track\nits end.  This default behavior is not desirable when you really want to\ntrack the actual name of the file, not the file descriptor (e.g., log\nrotation).  Use --follow=name in that case.  That causes tail to track the\nnamed file in a way that accommodates renaming, removal and creation.\n",5),v1);
    sub_4e20();
  }
  exit(a0); // no-return
}


// Function: sub_7f20 @ 0x7f20
unsigned long sub_7f20(unsigned long a0)
{
  char v1 [16];
  unsigned long v2; // rax
  unsigned long v3;
  unsigned long v4; // rdx
  unsigned long v5; // rsi
  long v6; // r8
  
  v1._8_8_ = 0;
  v1._0_8_ = a0;
  v2 = SUB168(v1._0_16_ * ZEXT816(0xaaaaaaaaaaaaaaab),0);
  v4 = a0 % 3;
  if ((10 <= a0) && (v4)) {
    v6 = 0x10;
    v5 = 9;
    v3 = 3;
    do {
      v3 += 2;
      v5 += v6;
      v2 = a0 / v3;
      v4 = a0 % v3;
      if (a0 <= v5) break;
      v6 += 8;
    } while (v4);
  }
  v3 = v2 >> 8;
  v2 = CONCAT71((undefined7)v3,v4 != 0);
  return v2;
}


// Function: sub_7f90 @ 0x7f90
unsigned long sub_7f90(unsigned long a0)
{
  char v1; // al
  unsigned long v2; // rax
  unsigned long v3;
  
  v2 = 10;
  if (10 <= a0)
    v2 = a0;
  v3 = v2 | 1;
  while ((v3 != 0xffffffffffffffff && (v1 = sub_7f20(v3), !v1))) {
    v3 += 2;
  }
  v2 = v3;
  return v2;
}


// Function: sub_7fd0 @ 0x7fd0
unsigned long sub_7fd0(unsigned long a0,unsigned long a1)
{
  return (unsigned long)sub_caf0(a0,3) % a1;
}


// Function: sub_7ff0 @ 0x7ff0
bool sub_7ff0(long a0,long a1)
{
  return a1 == a0;
}


// Function: sub_8000 @ 0x8000
unsigned long sub_8000(long a0) // return-dupe
{
  float v1;
  float *v2;
  float v3; // xmm1_da
  
  v2 = *(float **)(a0 + 0x28);
  if (v2 == (float *)0xf5d0)
    return 1;
  v1 = v2[2];
  if ((((dat_f5e4 < v1) && (v1 < dat_f5e8)) && (dat_f5ec < v2[3])) && (dat_f600 <= *v2)) {
    v3 = *v2 + dat_f5e4;
    if (((v3 < v2[1]) && (v2[1] <= dat_f5f0)) && (v3 < v1))
      return 1;
  }
  *(unsigned long *)(a0 + 0x28) = 0xf5d0;
  return 0;
}


// Function: sub_8090 @ 0x8090
void sub_8090(long a0,unsigned long *a1)
{
  unsigned long v1;
  
  v1 = *(unsigned long *)(a0 + 0x48);
  *a1 = 0;
  a1[1] = v1;
  *(unsigned long **)(a0 + 0x48) = a1;
}


// Function: sub_80b0 @ 0x80b0
long sub_80b0(long *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  v1 = (*(void *)a0[6])(a1,a0[2]);
  if (v1 < (unsigned long)a0[2])
    return v1 * 0x10 + *a0;
  abort(); // no-return
}


// Function: sub_80e0 @ 0x80e0
long sub_80e0(long a0,long a1,unsigned long *a2,char a3) // early-return
{
  void *v1;
  long *v2;
  long v3;
  long *v4;
  long v5;
  
  v4 = (long *)sub_80b0(a0,a1);
  *a2 = v4;
  v5 = *v4;
  if (!v5)
    return 0;
  if (v5 != a1) {
    v1 = *(void **)(a0 + 0x38);
    if (!(*v1)(a1)) {
      while( true ) {
        if (!(long *)v4[1])
          return 0;
        if ((*(long *)v4[1] == a1) || (v1 = *(void **)(a0 + 0x38), (*v1)(a1))) break;
        v4 = (long *)v4[1];
      }
      v5 = *(long *)v4[1];
      if (!a3)
        return v5;
      v4[1] = ((long *)v4[1])[1];
      sub_8090(a0);
      return v5;
    }
    v5 = *v4;
  }
  if (a3) {
    v2 = (long *)v4[1];
    if (v2) {
      v3 = v2[1];
      *v4 = *v2;
      v4[1] = v3;
      sub_8090(a0);
      return v5;
    }
    *v4 = 0;
  }
  return v5;
}


// Function: sub_81e0 @ 0x81e0
void sub_81e0(long a0)
{
  if (*(long *)(a0 + 0x48)) {
    *(unsigned long *)(a0 + 0x48) = *(unsigned long *)(*(long *)(a0 + 0x48) + 8);
    return;
  }
  malloc(0x10); // tail-call
}


// Function: sub_8210 @ 0x8210
unsigned long sub_8210(long a0,unsigned long *a1,char a2) // return-dupe x2
{
  long v1;
  long *v2; // rax
  long *v3;
  long *v4;
  long *v5;
  long v6;
  
  v5 = (long *)*a1;
  if ((long *)a1[1] <= v5)
    return 1;
  do {
    while (v6 = *v5, !v6) {
label_8238:
      v5 = &v5[2];
      if ((long *)a1[1] <= v5)
        return 1;
    }
    v3 = (long *)v5[1];
    if ((long *)v5[1]) {
      do {
        while( true ) {
          v6 = *v3;
          v2 = (long *)sub_80b0(a0,v6);
          v4 = (long *)v3[1];
          if (*v2) break;
          *v2 = v6;
          *(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + 1;
          sub_8090(a0);
          v3 = v4;
          if (!v4) goto label_82a8;
        }
        v3[1] = v2[1];
        v2[1] = (long)v3;
        v3 = v4;
      } while (v4);
label_82a8:
      v6 = *v5;
    }
    v5[1] = 0;
    if (a2) goto label_8238;
    v3 = (long *)sub_80b0(a0,v6);
    if (*v3) { // branch-flip
      v4 = (long *)sub_81e0(a0);
      if (!v4)
        return 0;
      v1 = v3[1];
      *v4 = v6;
      v4[1] = v1;
      v3[1] = (long)v4;
    }
    else {
      *v3 = v6;
      *(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + 1;
    }
    *v5 = 0;
    v5 = &v5[2];
    a1[3] = a1[3] + -1;
    if ((long *)a1[1] <= v5)
      return 1;
  } while( true );
}


// Function: sub_83d0 @ 0x83d0
unsigned long sub_83d0(long a0)
{
  return *(unsigned long *)(a0 + 0x10);
}


// Function: sub_83e0 @ 0x83e0
unsigned long sub_83e0(long a0)
{
  return *(unsigned long *)(a0 + 0x18);
}


// Function: sub_83f0 @ 0x83f0
unsigned long sub_83f0(long a0)
{
  return *(unsigned long *)(a0 + 0x20);
}


// Function: sub_8400 @ 0x8400
unsigned long sub_8400(unsigned long *a0)
{
  long v1;
  long *v2;
  unsigned long v3;
  unsigned long v4;
  
  v2 = (long *)*a0;
  v4 = 0;
  do {
    if ((long *)a0[1] <= v2)
      return v4;
    while (*v2) {
      v3 = 1;
      for (v1 = v2[1]; v1; v1 = *(long *)(v1 + 8)) {
        v3 += 1;
      }
      if (v4 < v3)
        v4 = v3;
      v2 = &v2[2];
      if ((long *)a0[1] <= v2)
        return v4;
    }
    v2 = &v2[2];
  } while( true );
}


// Function: sub_84d0 @ 0x84d0
void sub_84d0(unsigned long a0,void *a1)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // rax
  
  v1 = sub_83f0(a0);
  v2 = sub_83d0();
  v3 = sub_83e0();
  v4 = sub_8400();
  __fprintf_chk(a1,1,"# entries:         %lu\n",v1);
  __fprintf_chk(a1,1,"# buckets:         %lu\n",v2);
  __fprintf_chk(a1,1,"# buckets used:    %lu (%.2f%%)\n",v3);
  __fprintf_chk(a1,1,"max bucket length: %lu\n",v4); // tail-call
}


// Function: sub_85f0 @ 0x85f0
long sub_85f0(long a0,long a1)
{
  void *v1;
  long *v2;
  long v3;
  
  v2 = (long *)sub_80b0(a0,a1);
  v3 = *v2;
  if (!v3)
    return 0;
  while( true ) {
    if (v3 == a1)
      return a1;
    v1 = *(void **)(a0 + 0x38);
    if ((*v1)(a1)) break;
    v2 = (long *)v2[1];
    if (!v2)
      return 0;
    v3 = *v2;
  }
  return *v2;
}


// Function: sub_86a0 @ 0x86a0
long sub_86a0(long a0,long a1)
{
  long v1;
  long *v2; // rax
  long *v3;
  
  v2 = (long *)sub_80b0(a0,a1);
  v3 = v2;
  do {
    v1 = *v3;
    v3 = (long *)v3[1];
    if (v1 == a1) {
      if (v3)
        return *v3;
      break;
    }
  } while (v3);
  do {
    v2 = &v2[2];
    if (*(long **)(a0 + 8) <= v2)
      return 0;
  } while (!*v2);
  return *v2;
}


// Function: sub_8760 @ 0x8760
long sub_8760(unsigned long *a0,void *a1,unsigned long a2)
{
  long *v1;
  long v2;
  long v3; // r12
  long *v4;
  
  v4 = (long *)*a0;
  if ((long *)a0[1] <= v4) // branch-flip
    v3 = 0;
  else {
    v3 = 0;
    do {
      v2 = *v4;
      if (v2) {
        v1 = v4;
        while( true ) {
          if (!(*a1)(v2,a2))
            return v3;
          v1 = (long *)v1[1];
          v3 += 1;
          if (!v1) break;
          v2 = *v1;
        }
      }
      v4 = &v4[2];
    } while (v4 < (long *)a0[1]);
  }
  return v3;
}


// Function: sub_8850 @ 0x8850
unsigned long * sub_8850(unsigned long a0,long a1,void *a2,void *a3,unsigned long a4) // return-dupe
{
  unsigned long *v1; // rax
  unsigned long v2; // rax
  void *v3; // rax
  
  if (!a2)
    a2 = sub_7fd0;
  if (!a3)
    a3 = sub_7ff0;
  v1 = malloc(0x50);
  if (!v1)
    return NULL;
  if (!a1)
    a1 = 0xf5d0;
  v1[5] = a1;
  if (sub_8000(v1)) {
    v2 = sub_8340(*(unsigned int *)(a1 + 8),a0,*(char *)(a1 + 0x10));
    v1[2] = v2;
    if (v2) {
      v3 = calloc(v2,0x10);
      *v1 = v3;
      if (v3) {
        v1[6] = a2;
        v1[3] = 0;
        v1[7] = a3;
        v1[8] = a4;
        v1[1] = (void *)((long)v3 + v1[2] * 0x10);
        v1[4] = 0;
        v1[9] = 0;
        return v1;
      }
    }
  }
  free(v1);
  return NULL;
}


// Function: sub_8960 @ 0x8960
void sub_8960(unsigned long *a0)
{
  unsigned long *v1;
  unsigned long v2;
  void *v3;
  unsigned long *v4;
  long *v5;
  long *v6; // r12
  
  v5 = (long *)*a0;
  if ((long *)*a0 < (long *)a0[1]) {
    do {
      while (!*v5) {
        v5 = &v5[2];
        if ((long *)a0[1] <= v5) goto label_89fb;
      }
      v3 = (void *)a0[8];
      v4 = (unsigned long *)v5[1];
      if ((unsigned long *)v5[1]) {
        while( true ) {
          if (v3) {
            (*v3)(*v4);
            v3 = (void *)a0[8];
          }
          v1 = (unsigned long *)v4[1];
          v2 = a0[9];
          *v4 = 0;
          v4[1] = v2;
          a0[9] = v4;
          if (!v1) break;
          v4 = v1;
        }
      }
      if (v3)
        (*v3)(*v5);
      *v5 = 0;
      v6 = &v5[2];
      v5[1] = 0;
      v5 = v6;
    } while (v6 < (long *)a0[1]);
  }
label_89fb:
  a0[3] = 0;
  a0[4] = 0;
  return;
}


// Function: sub_8a10 @ 0x8a10
void sub_8a10(unsigned long *a0)
{
  void *v1;
  void *v2;
  long *v3;
  long *v4;
  long v5;
  
  v4 = (long *)*a0;
  v3 = (long *)a0[1];
  if ((a0[8]) && (a0[4])) {
    if (v3 <= v4) goto label_8abc;
    do {
      while (v5 = *v4, !v5) {
        v4 = &v4[2];
        if (v3 <= v4) goto label_8a7f;
      }
      v3 = v4;
      while( true ) {
        (*(void *)a0[8])(v5);
        v3 = (long *)v3[1];
        if (!v3) break;
        v5 = *v3;
      }
      v3 = (long *)a0[1];
      v4 = &v4[2];
    } while (v4 < v3);
label_8a7f:
    v4 = (long *)*a0;
  }
  if (v4 < v3) {
    do {
      v2 = (void *)v4[1];
      while (v2) {
        v1 = *(void **)((long)v2 + 8);
        free(v2);
        v2 = v1;
      }
      v4 = &v4[2];
    } while (v4 < (long *)a0[1]);
  }
label_8abc:
  v2 = (void *)a0[9];
  while (v2) {
    v1 = *(void **)((long)v2 + 8);
    free(v2);
    v2 = v1;
  }
  free((void *)*a0);
  free(a0); // tail-call
}


// Function: sub_8b00 @ 0x8b00
unsigned int sub_8b00(unsigned long *a0,unsigned long a1) // return-dupe
{
  unsigned int v1; // eax
  unsigned long v10; // stack - 0x48
  unsigned long v11; // stack - 0x40
  void *v2; // stack - 0x88
  void *v3; // stack - 0x80
  unsigned long v4; // stack - 0x78
  unsigned long v5; // stack - 0x70
  unsigned long v6; // stack - 0x68
  unsigned long v7; // stack - 0x60
  unsigned long v8; // stack - 0x58
  unsigned long v9; // stack - 0x50
  
  v4 = sub_8340(*(unsigned int *)(a0[5] + 8),a1,*(char *)(a0[5] + 0x10));
  if (!v4)
    return 0;
  if (a0[2] == v4)
    return 1;
  v2 = calloc(v4,0x10);
  if (!v2)
    return 0;
  v3 = (void *)((long)v2 + v4 * 0x10);
  v5 = 0;
  v7 = a0[5];
  v6 = 0;
  v8 = a0[6];
  v9 = a0[7];
  v10 = a0[8];
  v11 = a0[9];
  v1 = sub_8210(&v2,a0,0);
  if (!(char)v1) {
    a0[9] = v11;
    if ((sub_8210(a0,&v2,1)) && (sub_8210(a0,&v2,0))) {
      free(v2);
      return v1;
    }
    abort(); // no-return
  }
  free((void *)*a0);
  *a0 = v2;
  a0[1] = v3;
  a0[2] = v4;
  a0[3] = v5;
  a0[9] = v11;
  return v1;
}


// Function: sub_8c80 @ 0x8c80
unsigned long sub_8c80(long a0,long a1,long *a2) // ternary
{
  long v1;
  long *v2; // rax
  long *v3; // stack - 0x38
  unsigned long v4; // rsi
  float v5; // xmm4_da
  float v6; // xmm5_da
  
  if (a1) {
    v1 = sub_80e0(a0,a1,&v3,0);
    if (v1) {
      if (a2) {
        *a2 = v1;
        return 0;
      }
      return 0;
    }
    v6 = (float)*(unsigned long *)(a0 + 0x18);
    v5 = (float)*(unsigned long *)(a0 + 0x10);
    if (*(float *)(*(long *)(a0 + 0x28) + 8) * v5 < v6) {
      sub_8000(a0);
      v1 = *(long *)(a0 + 0x28);
      if (v5 * *(float *)(v1 + 8) < v6) {
        v5 = v5 * *(float *)(v1 + 0xc);
        if (!*(char *)(v1 + 0x10))
          v5 = v5 * *(float *)(v1 + 8);
        if (dat_f5f4 <= v5)
          return 0xffffffff;
        v4 = (dat_f5f8 <= v5) ? (long)(v5 - dat_f5f8) ^ 0x8000000000000000 : (unsigned long)v5;
        if (!sub_8b00(a0,v4))
          return 0xffffffff;
        if (sub_80e0(a0,a1,&v3,0)) {
          abort(); // no-return, return-dupe
        }
      }
    }
    if (!*v3) {
      *v3 = a1;
      *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + 1;
      *(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + 1;
      return 1;
    }
    v2 = (long *)sub_81e0(a0);
    if (!v2)
      return 0xffffffff;
    *v2 = a1;
    v2[1] = v3[1];
    v3[1] = (long)v2;
    *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + 1;
    return 1;
  }
  abort();
}


// Function: sub_8e70 @ 0x8e70
unsigned long sub_8e70(unsigned long a0,unsigned long a1) // early-return
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x18
  bool v4; // zf
  
  v1 = sub_8c80(a0,a1,&v3);
  if (v1 != -1) {
    v4 = v1 == 0;
    v2 = a1;
    if (v4)
      v2 = v3;
    return v2;
  }
  v2 = 0;
  return v2;
}


// Function: sub_8ed0 @ 0x8ed0
long sub_8ed0(long a0,unsigned long a1) // ternary x2
{
  unsigned long v1;
  float *v2;
  void *v3;
  void *v4;
  long v5; // rax
  unsigned long v6; // rax
  long *v7; // stack - 0x28
  float v8; // xmm4_da
  float v9;
  
  v5 = sub_80e0(a0,a1,&v7,1);
  if (!v5)
    return 0;
  *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + -1;
  if (!*v7) {
    v6 = *(long *)(a0 + 0x18) - 1;
    *(unsigned long *)(a0 + 0x18) = v6;
    v1 = (0 <= (long)v6) ? *(unsigned long *)(a0 + 0x10) : *(unsigned long *)(a0 + 0x10); // branch-flip
    v9 = (float)v6;
    v8 = (float)v1;
    if (v9 < **(float **)(a0 + 0x28) * v8) {
      sub_8000(a0);
      v2 = *(float **)(a0 + 0x28);
      if (v9 < *v2 * v8) {
        v8 = v8 * v2[1];
        if (!*(char *)&v2[4])
          v8 = v8 * v2[2];
        v6 = (dat_f5f8 <= v8) ? (long)(v8 - dat_f5f8) ^ 0x8000000000000000 : (unsigned long)v8;
        if (!sub_8b00(a0,v6)) {
          v4 = *(void **)(a0 + 0x48);
          while (v4) {
            v3 = *(void **)((long)v4 + 8);
            free(v4);
            v4 = v3;
          }
          *(unsigned long *)(a0 + 0x48) = 0;
        }
      }
    }
    return v5;
  }
  return v5;
}

