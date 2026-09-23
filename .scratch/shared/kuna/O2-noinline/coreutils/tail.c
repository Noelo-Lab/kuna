// Function: main @ 0x29a0
int main(int argc,char **argv,char **envp)
{
  char **v1;
  long *v10; // rax
  long *v11;
  long *v12; // rax
  int *v13; // rax
  char *v14;
  char **v15;
  unsigned int v16; // ebx
  long v17; // stack - 0xf0
  int v18; // stack - 0xf4
  unsigned long v19; // stack - 0xe8
  bool v2;
  stat v20; // stack - 0xd8
  unsigned long v21; // stack - 0xe0
  char **v22;
  unsigned long v23; // stack - 0x100
  unsigned char v3; // al
  unsigned int v4; // eax
  int v5; // eax
  int v6; // eax
  int v7; // eax
  int v8; // eax
  int v9; // eax
  
  v18 = 0;
  v17 = 10;
  v19 = dat_f4f0;
  sub_91a0(*argv);
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
  v3 = sub_3bc0(argc,argv,&v17);
  argc -= (unsigned int)v3;
  sub_7080(argc,&argv[v3],&v17,&v18,&v19);
  if ((dat_15116) && (v17))
    v17 -= 1;
  v15 = (char **)0x15028;
  v23 = 1;
  if (optind < argc) {
    v15 = &(&argv[v3])[optind];
    v23 = (unsigned long)(argc - optind);
  }
  v2 = 0;
  v1 = &v15[v23];
  v22 = v15;
  do {
    v14 = *v22;
    if (!strcmp(v14,"-"))
      v2 = 1;
    v22 = &v22[1];
  } while (v1 != v22);
  if (v2) {
    if (dat_15020 == 1) {
      v14 = (char *)sub_b410(4,"-");
      error(1,0,dcgettext(NULL,"cannot follow %s by name",5),v14);
      return v9;
    }
    if (dat_15118) {
      if (((((dat_15110) || (dat_15020 != 2)) || (v23 != 1)) || ((fstat(0,&v20) || ((v20._24_4_ & 0xf000) == 0x8000)))) && (isatty(0)))
        error(0,0,dcgettext(NULL,"warning: following standard input indefinitely is ineffective",5));
      goto label_2b55;
    }
    if (v17) goto label_2b6a;
  }
  else {
label_2b55:
    if ((v17) || (dat_15118)) goto label_2b6a;
  }
  if (!dat_15116)
    return 0;
label_2b6a:
  v10 = (long *)sub_c0c0(v23,0x60);
  v11 = v10;
  do {
    v14 = *v15;
    v15 = &v15[1];
    v12 = &v11[0xc];
    *v11 = (long)v14;
    v11 = v12;
  } while (v15 != v1);
  if ((v18 == 1) || ((v23 != 1 && (!v18))))
    dat_15115 = 1;
  v16 = 1;
  v11 = v10;
  do {
    v12 = &v11[0xc];
    v4 = sub_6ab0(v11,v17);
    v16 &= v4;
    v11 = v12;
  } while (&v10[v23 * 0xc] != v12);
  if ((dat_15118) && (sub_3a50(v10,v23))) {
    if (fstat(1,&v20) < 0) {
      v14 = dcgettext(NULL,"standard output",5);
      error(1,*__errno_location(),v14);
      return v7;
    }
    dat_15117 = (v20._24_4_ & 0xf000) == 0x1000;
    if ((((!dat_1510c) && (!sub_3ae0(v10,v23))) && (!sub_38d0(v10,v23))) && (((sub_3900(v10) && (!sub_4b40(v10))) && ((!sub_3930(v10,v23) && (((char)v16 || (dat_15020 != 2)))))))) {
      v5 = inotify_init();
      v13 = __errno_location();
      if (0 <= v5) {
        if (fflush_unlocked(stdout)) {
          v14 = dcgettext(NULL,"write error",5);
          error(1,*v13,v14);
          return v8;
        }
        sub_5880(v19,v5,v10,v23,&v21);
        sub_8a10(v21);
        close(v5);
        *v13 = 0;
      }
      v14 = dcgettext(NULL,"inotify cannot be used, reverting to polling",5);
      error(0,*v13,v14);
    }
    dat_1510c = '\x01';
    sub_6220(v19,v10,v23);
  }
  if ((dat_1510e) && (close(0) <= -1)) {
    error(1,*__errno_location(),"-");
    return v6;
  }
  return v16 ^ 1;
}


// Function: sub_3830 @ 0x3830
void sub_3830(void *a0,unsigned int a1,unsigned long a2,struct_3 *a3,unsigned int a4)
{
  unsigned int v1;
  unsigned long v2;
  char v3 [16];
  
  *(unsigned int *)((long)a0 + 0x38) = a1;
  *(unsigned long *)((long)a0 + 8) = a2;
  v3 = sub_b7b0(a3);
  *(long *)((long)a0 + 0x10) = SUB168(v3,0);
  v2 = a3->field_0x0;
  *(long *)((long)a0 + 0x18) = SUB168(v3,8);
  *(unsigned long *)((long)a0 + 0x20) = v2;
  *(unsigned long *)((long)a0 + 0x28) = a3->field_0x8;
  v1 = a3->field_0x18;
  *(unsigned int *)((long)a0 + 0x40) = a4;
  *(unsigned int *)((long)a0 + 0x30) = v1;
  *(unsigned long *)((long)a0 + 0x58) = 0;
  *(char *)((long)a0 + 0x34) = 0;
}


// Function: sub_3880 @ 0x3880
unsigned char sub_3880(void *a0,long a1) // early-return
{
  unsigned char v1; // al
  unsigned char *v2; // rcx
  unsigned char *v3; // rdi
  
  if ((dat_1511a) && (dat_15020 == 1))
    return dat_1511a;
  v3 = (unsigned char *)((long)a0 + 0x34);
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
char sub_38d0(void *a0,long a1)
{
  char *v1; // rax
  char *v2; // rdi
  
  v2 = (char *)((long)a0 + 0x35);
  v1 = &v2[a1 * 0x60];
  while ((*(int *)&v2[3] <= -1 || (!*v2))) {
    v2 = &v2[0x60];
    if (v2 == v1)
      return '\0';
  }
  return *v2;
}


// Function: sub_3900 @ 0x3900
unsigned long sub_3900(void *a0,long a1)
{
  char *v1; // rax
  char *v2; // rdi
  
  v2 = (char *)((long)a0 + 0x35);
  v1 = &v2[a1 * 0x60];
  while ((*(int *)&v2[3] < 0 || (*v2))) {
    v2 = &v2[0x60];
    if (v2 == v1)
      return 0;
  }
  return 1;
}


// Function: sub_3930 @ 0x3930
unsigned int sub_3930(void *a0,long a1)
{
  unsigned int *v1; // rcx
  unsigned int v2; // edx
  unsigned int *v3; // rdi
  
  v3 = (unsigned int *)((long)a0 + 0x30);
  v1 = &v3[a1 * 0x18];
  while (((int)v3[2] <= -1 || (v2 = *v3 & 0xf000, v2 == 0x1000 || v2 == 0x8000))) {
    v3 = &v3[0x18];
    if (v1 == v3)
      return 0;
  }
  return CONCAT31((undefined3)(v3[2] >> 8),v2 != 0x1000 && v2 != 0x8000);
}


// Function: sub_3980 @ 0x3980
unsigned long sub_3980(void *a0,unsigned long a1)
{
  return (unsigned long)(long)*(int *)((long)a0 + 0x44) % a1;
}


// Function: sub_39a0 @ 0x39a0
unsigned int sub_39a0(void *a0,void *a1)
{
  return CONCAT31((undefined3)((unsigned int)*(int *)((long)a1 + 0x44) >> 8),*(int *)((long)a0 + 0x44) == *(int *)((long)a1 + 0x44));
}


// Function: sub_3a10 @ 0x3a10
void sub_3a10(char *a0)
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
void sub_3b30(unsigned int a0,char *a1)
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
unsigned long sub_3bc0(int a0,void *a1,unsigned long *a2) // return-dupe
{
  unsigned char *v1;
  char v10; // bpl
  char v11; // r13b
  bool v12; // zf
  unsigned char v2;
  int v3; // eax
  unsigned char *v4;
  char *v5;
  char *v6; // rax
  unsigned long v7; // rax
  unsigned long v8; // rcx
  unsigned char *v9;
  
  if (a0 != 2) {
    if (a0 != 3) { // branch-flip
      if (a0 != 4)
        return 0;
      v5 = *(char **)((long)a1 + 0x10);
    }
    else {
      v5 = *(char **)((long)a1 + 0x10);
      if ((*v5 != '-') || (!v5[1])) goto label_3bf5;
    }
    if (strcmp(v5,"--"))
      return 0;
  }
label_3bf5:
  v3 = sub_9110();
  v5 = *(char **)((long)a1 + 8);
  v1 = (unsigned char *)&v5[1];
  if (*v5 != '+') { // branch-flip
    if (*v5 != '-')
      return 0;
    if ((0x30db0 <= v3) && (!v5[(unsigned long)(v5[1] == 'c') + 1]))
      return 0;
    v10 = 0;
  }
  else {
    v10 = 1;
    if ((unsigned int)(v3 - 0x30db0U) <= 0x2b8)
      return 0;
  }
  v2 = v5[1];
  v9 = v1;
  while ((unsigned int)((int)(char)v2 - 0x30U) <= 9) {
    v4 = &v9[1];
    v9 = &v9[1];
    v2 = *v4;
  }
  if (v2 != 99) { // branch-flip
    if (v2 != 0x6c) { // branch-flip
      if (v2 != 0x62) {
        v11 = 1;
        v8 = 10;
        v4 = v9;
        goto label_3c8b;
      }
      v11 = 0;
      v8 = 0x1400;
    }
    else {
      v11 = 1;
      v8 = 10;
    }
  }
  else {
    v11 = 0;
    v8 = 10;
  }
  v4 = &v9[1];
label_3c8b:
  v12 = *v4 == 0x66;
  if (v12)
    v4 = &v4[1];
  if (*v4)
    return 0;
  if (v1 == v9) {
    *a2 = v8;
    dat_15116 = v10;
    dat_15118 = v12;
    dat_15119 = v11;
    return 1;
  }
  if (!(sub_c6c0(v1,NULL,10,a2,"b") & 0xfffffffd)) {
    dat_15116 = v10;
    dat_15118 = v12;
    dat_15119 = v11;
    return 1;
  }
  v5 = (char *)sub_b6d0(*(char **)((long)a1 + 8));
  v6 = dcgettext(NULL,"invalid number",5);
  error(1,*__errno_location(),"%s: %s",v6,v5);
  return v7;
}


// Function: sub_3da0 @ 0x3da0
void sub_3da0(void *a0,unsigned long a1)
{
  unsigned long v1; // rax
  char *v2; // rax
  
  if (!a1)
    return;
  if (a1 <= fwrite_unlocked(a0,1,a1,stdout))
    return;
  clearerr_unlocked(stdout);
  v1 = sub_b410(4,"standard output");
  v2 = dcgettext(NULL,"error writing %s",5);
  error(1,*__errno_location(),v2,v1);
}


// Function: sub_3e30 @ 0x3e30
unsigned long sub_3e30(char *a0,unsigned int a1,long a2,long *a3) // return-dupe
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
int * sub_3f80(char a0,char *a1,unsigned int a2,unsigned long a3)
{
  int *v1;
  char *v2;
  int *v3; // rax
  unsigned long v4;
  unsigned long v5;
  char v6 [8200];
  long v7; // r12
  
  v7 = 0;
  v5 = a3;
  do {
    v4 = 0x2000;
    if (v5 <= 0x2000)
      v4 = v5;
    v3 = (unsigned long)sub_b6e0(a2,v6,v4);
    if (v3 == (int *)0xffffffffffffffff) {
      v3 = __errno_location();
      v1 = v3;
      if (*v3 == 0xb) {
        v3 = (int *)v7;
        return v3;
      }
      v3 = (char *)sub_b410(4,a1);
      v2 = (char *)v3;
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
    if (a3 != 0xffffffffffffffff) {
      v5 -= (long)v3;
      if (!v5) {
        v3 = (int *)v7;
        return v3;
      }
      if (a3 == 0xfffffffffffffffe) {
        v3 = (int *)v7;
        return v3;
      }
    }
    a0 = 0;
  } while( true );
}


// Function: sub_40c0 @ 0x40c0
unsigned long sub_40c0(char *a0,unsigned int a1,unsigned long a2,long *a3) // return-dupe
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
void sub_41c0(unsigned long a0,unsigned long a1,int a2,char *a3)
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
unsigned long sub_4290(char *a0,unsigned int a1,long a2,unsigned long a3,unsigned long a4,long *a5) // return-dupe
{
  void *v1; // rax
  unsigned long v10; // stack - 0x2050
  long v2;
  unsigned long v3; // rax
  char *v4; // rax
  unsigned long v5;
  int v6; // ebx
  unsigned long v7; // rax
  char v8 [8200];
  unsigned long v9; // rsi
  
  if (!a2)
    return 1;
  v7 = (long)(a4 - a3) % 0x2000;
  if (!v7)
    v7 = 0x2000;
  v9 = a4 - v7;
  v10 = a4;
  sub_41c0(a1,v9,0,a0);
  v7 = sub_b6e0(a1,v8,v7);
  if (v7 != 0xffffffffffffffff) {
    v6 = (int)dat_15114;
    *a5 = v9 + v7;
    if ((v7) && (v8[v7 - 1] != dat_15114))
      a2 -= 1;
    while( true ) {
      v5 = v7;
      while ((v5 && (v1 = memrchr(v8,v6,v5), v1))) {
        v2 = a2 + -1;
        v5 = (long)v1 - (long)v8;
        if (!a2) {
          sub_3da0((void *)((long)v1 + 1),(v7 - 1) - v5);
          v2 = sub_3f80(0,a0,a1,(v10 - v7) - v9);
          *a5 = *a5 + v2;
          return 1;
        }
        a2 = v2;
      }
      if (v9 == a3) {
        sub_41c0(a1,v9,0,a0);
        *a5 = v9 + sub_3f80(0,a0,a1,v10);
        return 1;
      }
      v9 -= 0x2000;
      sub_41c0(a1,v9,0,a0);
      v7 = sub_b6e0(a1,v8,0x2000);
      if (v7 == 0xffffffffffffffff) break;
      *a5 = v9 + v7;
      if (!v7)
        return 1;
      v6 = (int)dat_15114;
    }
  }
  v3 = sub_b410(4,a0);
  v4 = dcgettext(NULL,"error reading %s",5);
  error(0,*__errno_location(),v4,v3);
  return 0;
}


// Function: sub_4510 @ 0x4510
unsigned long sub_4510(char *a0,unsigned int a1,unsigned long a2,long *a3)
{
  long v1;
  unsigned long v10; // r12
  void *v11; // r13
  void *v12;
  unsigned long v13;
  unsigned long v2;
  unsigned long v3;
  void *v4;
  void *v5;
  unsigned long v6;
  unsigned long v7; // rax
  char *v8; // rax
  void *v9;
  
  v4 = (void *)sub_bfa0(0x2010);
  *(unsigned long *)((long)v4 + 0x2000) = 0;
  *(unsigned long *)((long)v4 + 0x2008) = 0;
  v5 = (void *)sub_bfa0(0x2010);
  v9 = v5;
  v12 = v4;
  v13 = 0;
  while (v11 = v9, v9 = v4, v6 = sub_b6e0(a1,v5,0x2000), v6 - 1 <= 0xfffffffffffffffd) {
    v13 += v6;
    *a3 = *a3 + v6;
    *(unsigned long *)((long)v11 + 0x2000) = v6;
    v1 = *(long *)((long)v12 + 0x2000);
    *(unsigned long *)((long)v11 + 0x2008) = 0;
    v4 = v9; // branch-flip
    if (0x2000 <= v1 + v6) {
      *(void **)((long)v12 + 0x2008) = v11;
      v6 = v13 - *(long *)((long)v9 + 0x2000);
      v12 = v11; // branch-flip
      if (v6 <= a2) {
        v5 = (void *)sub_bfa0(0x2010);
        v9 = v5;
      }
      else {
        v4 = *(void **)((long)v9 + 0x2008);
        v5 = v9;
        v13 = v6;
      }
    }
    else {
      memcpy((void *)(v1 + (long)v12),v5,v6);
      *(long *)((long)v12 + 0x2000) = *(long *)((long)v12 + 0x2000) + *(long *)((long)v11 + 0x2000);
      v9 = v11;
    }
  }
  free(v11);
  if (v6 != 0xffffffffffffffff) { // branch-flip
    v6 = *(unsigned long *)((long)v9 + 0x2000);
    v3 = v13 - v6;
    v4 = v9;
    while (v2 = v3, a2 < v2) {
      v4 = *(void **)((long)v4 + 0x2008);
      v6 = *(unsigned long *)((long)v4 + 0x2000);
      v3 = v2 - v6;
      v13 = v2;
    }
    v5 = v4;
    if (a2 < v13) {
      v6 += a2 - v13;
      v5 = (void *)((long)v4 + (v13 - a2));
    }
    sub_3da0(v5,v6);
    for (v1 = *(long *)((long)v4 + 0x2008); v1; v1 = *(long *)(v1 + 0x2008)) {
      sub_3da0(v1,*(unsigned long *)(v1 + 0x2000));
    }
    v10 = 1;
  }
  else {
    v7 = sub_b410(4,a0);
    v8 = dcgettext(NULL,"error reading %s",5);
    v10 = 0;
    error(0,*__errno_location(),v8,v7);
    if (!v9)
      return 0;
  }
  do {
    v4 = *(void **)((long)v9 + 0x2008);
    free(v9);
    v9 = v4;
  } while (v4);
  return v10;
}


// Function: sub_4760 @ 0x4760
unsigned long sub_4760(char *a0,unsigned int a1,unsigned long a2,long *a3)
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
  while (v10 = v2, v3 = sub_b6e0(a1,v13,0x2000), (unsigned long)(v3 - 1U) <= 0xfffffffffffffffd) {
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
    if (0x2000 <= *(unsigned long *)((long)v13 + 0x2000) + *(long *)((long)v9 + 0x2000)) {
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
        sub_3da0(v3,*(unsigned long *)(v3 + 0x2000));
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
unsigned long sub_4a40(int a0,char *a1) // return-dupe x3
{
  int v1; // eax
  int *v2; // rax
  char *v3; // rax
  char *v4; // rax
  statfs v5; // stack - 0xa8
  
  if (!fstatfs(a0,&v5)) {
    v1 = sub_2fb0(v5._0_8_);
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
  v3 = (char *)sub_b410(4,a1);
  v4 = dcgettext(NULL,"cannot determine location of %s. reverting to polling",5);
  error(0,*v2,v4,v3);
  return 1;
}


// Function: sub_4b40 @ 0x4b40
unsigned long sub_4b40(unsigned long *a0,long a1)
{
  char *v1;
  stat v2; // stack - 0xb8
  unsigned long *v3; // rbp
  
  v3 = &a0[a1 * 0xc];
  while ((v1 = (char *)*a0, lstat(v1,&v2) || ((v2._24_4_ & 0xf000) != 0xa000))) {
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
void sub_5090(struct_2 *a0,long *a1) // return-dupe x3
{
  int v1;
  long v2;
  char *v3; // rax
  long v4;
  int *v5; // rax
  char *v6; // rax
  stat v7; // stack - 0xc8
  bool v8;
  char v9 [16];
  
  if (a0->field_0x38 == -1)
    return;
  v3 = (char *)sub_5050(a0->field_0x0);
  v1 = a0->field_0x38;
  if (fstat(v1,&v7)) {
    v5 = __errno_location();
    a0->field_0x3c = *v5;
    sub_3b30(a0->field_0x38,v3);
    a0->field_0x38 = 0xffffffff;
    return;
  }
  if ((a0->field_0x30 & 0xf000) == 0x8000) {
    if ((long)a0->field_0x8 <= (long)v7._48_8_) { // branch-flip
      if (v7._48_8_ == a0->field_0x8) {
        v9 = sub_b7b0(&v7);
        v4 = a0->field_0x10;
        v2 = a0->field_0x18;
        if (!sub_b800(v4,v2,SUB168(v9,0),SUB168(v9,8)))
          return;
      }
    }
    else {
      v6 = (char *)sub_b510(0,3,v3);
      error(0,0,dcgettext(NULL,"%s: file truncated",5),v6);
      sub_41c0(a0->field_0x38,0,0,v3);
      a0->field_0x8 = 0;
    }
  }
  v8 = 0;
  if (dat_15115)
    v8 = (struct_2 *)*a1 != a0;
  v4 = sub_3f80(v8,v3,a0->field_0x38,0xffffffffffffffff);
  a0->field_0x8 = a0->field_0x8 + v4;
  if (!v4)
    return;
  *a1 = (long)a0;
  if (!fflush_unlocked(stdout))
    return;
  v3 = dcgettext(NULL,"write error",5);
  error(1,*__errno_location(),v3);
}


// Function: sub_5270 @ 0x5270
void sub_5270(struct_4 *a0,unsigned char a1)
{
  unsigned int v1;
  unsigned long v10; // rax
  char *v11;
  stat v12; // stack - 0xd8
  unsigned long v13; // r8
  unsigned int v2;
  char v3;
  char v4; // al
  char v5;
  int v6; // eax
  int v7; // eax
  unsigned int *v8; // rax
  char *v9;
  
  v9 = a0->field_0x0;
  v6 = strcmp(v9,"-");
  v1 = a0->field_0x3c;
  v5 = a0->field_0x36;
  if (v6) { // branch-flip
    v7 = sub_7ea0(v9,(unsigned int)(a1 ^ 1) << 0xb);
    v2 = a0->field_0x3c;
  }
  else {
    v7 = 0;
    v2 = v1;
  }
  v4 = sub_5260(a0->field_0x38,v2);
  v3 = dat_1511a;
  if (!v4)
    __assert_fail("valid_file_spec (f)","src/tail.c",0x3e2,"recheck"); // no-return
  if (dat_1511a) { // branch-flip
    if (v7 != -1) { // branch-flip
      a0->field_0x36 = 1;
      if ((dat_1510c) || ((v9 = a0->field_0x0, lstat(v9,&v12) || ((v12._24_4_ & 0xf000) != 0xa000)))) goto label_53ca;
    }
    else {
      a0->field_0x36 = 0;
      v9 = a0->field_0x0;
      if (dat_1510c) {
        a0->field_0x3c = *__errno_location();
        goto label_54d9;
      }
      if ((lstat(v9,&v12)) || ((v12._24_4_ & 0xf000) != 0xa000)) goto label_5321;
    }
  }
  else {
    a0->field_0x36 = 1;
    if ((dat_1510c) || ((v9 = a0->field_0x0, lstat(v9,&v12) || ((v12._24_4_ & 0xf000) != 0xa000)))) {
      if (v7 != -1) {
label_53ca:
        if (0 <= fstat(v7,&v12)) {
          if ((!((v12._24_4_ & 0xf000) - 0x1000 & 0xffffe000)) || ((v12._24_4_ & 0xb000) == 0x8000)) {
            v9 = a0->field_0x0;
            v5 = sub_4a40(v7,(char *)sub_5050(v9));
            *(char *)((long)&a0->field_0x34 + 1) = v5;
            if ((v5) && (!dat_1510c)) {
              a0->field_0x3c = 0xffffffff;
              v9 = a0->field_0x0;
              v9 = (char *)sub_b410(4,(char *)sub_5050(v9));
              error(0,0,dcgettext(NULL,"%s has been replaced with an untailable remote file",5),v9);
              a0->field_0x34 = 0x101;
              goto label_5348;
            }
            a0->field_0x3c = 0;
            if (v1 & 0xfffffffd) { // branch-flip
              if (a0->field_0x38 != -1)
                __assert_fail("f->fd == -1","src/tail.c",0x42f,"recheck"); // no-return
              v9 = a0->field_0x0;
              v10 = sub_b410(4,(char *)sub_5050(v9));
              v9 = "%s has become accessible";
            }
            else {
              v9 = a0->field_0x0;
              if (a0->field_0x38 != -1) {
                if ((a0->field_0x28 == v12._8_8_) && (a0->field_0x20 == v12._0_8_)) {
                  sub_3b30(v7,(char *)sub_5050(v9));
                  return;
                }
                v9 = (char *)sub_b410(4,(char *)sub_5050(v9));
                error(0,0,dcgettext(NULL,"%s has been replaced;  following new file",5),v9);
                v9 = (char *)sub_5050(a0->field_0x0);
                sub_3b30(a0->field_0x38,v9);
                goto label_5484;
              }
              v10 = sub_b410(4,(char *)sub_5050(v9));
              v9 = "%s has appeared;  following new file";
            }
            error(0,0,dcgettext(NULL,v9,5),v10);
label_5484:
            v13 = (unsigned long)a1;
            if (!v6)
              v13 = 0xffffffff;
            sub_3830(a0,v7,0,&v12,v13);
            v9 = a0->field_0x0;
            sub_41c0(v7,0,0,(char *)sub_5050(v9));
            return;
          }
          a0->field_0x3c = 0xffffffff;
          a0->field_0x36 = 0;
          if ((v3) && (dat_15020 == 1)) {
            *(char *)&a0->field_0x34 = 0;
            if (v5) // branch-flip
              v9 = "";
            else {
              v9 = "";
              if (v1 == 0xffffffff) goto label_5348;
            }
          }
          else {
            *(char *)&a0->field_0x34 = 1;
            if ((!v5) && (v1 == 0xffffffff)) goto label_5348;
            v9 = dcgettext(NULL,"; giving up on this name",5);
          }
          v11 = a0->field_0x0;
          v11 = (char *)sub_b410(4,(char *)sub_5050(v11));
          error(0,0,dcgettext(NULL,"%s has been replaced with an untailable file%s",5),v11,v9);
          goto label_5348;
        }
      }
label_5321:
      v8 = (unsigned int *)__errno_location();
      v9 = a0->field_0x0;
      v2 = *v8;
      a0->field_0x3c = v2;
      if (a0->field_0x36) {
        if (v2 != v1) {
          v9 = (char *)sub_b510(0,3,(char *)sub_5050(v9));
          error(0,*v8,"%s",v9);
        }
        goto label_5348;
      }
label_54d9:
      if (v5) {
        v9 = (char *)sub_b410(4,(char *)sub_5050(v9));
        v11 = dcgettext(NULL,"%s has become inaccessible",5);
        error(0,a0->field_0x3c,v11,v9);
      }
      goto label_5348;
    }
  }
  a0->field_0x3c = 0xffffffff;
  v9 = a0->field_0x0;
  *(char *)&a0->field_0x34 = 1;
  v9 = (char *)sub_b410(4,(char *)sub_5050(v9));
  error(0,0,dcgettext(NULL,"%s has been replaced with an untailable symbolic link",5),v9);
label_5348:
  v9 = a0->field_0x0;
  sub_3b30(v7,(char *)sub_5050(v9));
  v9 = (char *)sub_5050(a0->field_0x0);
  sub_3b30(a0->field_0x38,v9);
  a0->field_0x38 = 0xffffffff;
  return;
}


// Function: sub_5880 @ 0x5880
void sub_5880(double a0,unsigned int a1,long *a2,unsigned long a3,unsigned long *a4) // return-dupe
{
  char v1;
  struct_7 *v10; // rax
  unsigned long v11; // rax
  long v12;
  unsigned int *v13; // rax
  struct_14 *v14;
  char *v15;
  struct_14 *v16; // rax
  long *v17;
  unsigned long v18; // rax
  int *v19;
  unsigned int v2;
  long *v20; // stack - 0x140
  stat v21; // stack - 0xd8
  char v22 [68];
  char *v23;
  unsigned long v24; // stack - 0x1a8
  unsigned long v25; // stack - 0x198
  int *v26; // stack - 0x180
  unsigned long v27; // stack - 0x168
  int v28; // stack - 0x15c
  int v29; // stack - 0xf4
  unsigned int v3;
  int v4;
  long v5;
  bool v6;
  bool v7;
  unsigned int v8; // eax
  int v9;
  
  v10 = (struct_7 *)sub_8850(a3,NULL,sub_3980,sub_39a0,0);
  if (!v10)
    sub_c360(); // no-return, return-dupe
  v7 = 0;
  v6 = 0;
  *a4 = v10;
  v24._0_4_ = dat_15020;
  v8 = 2;
  if (dat_15020 == 1)
    v8 = 0xc06;
  v27 = 0;
  v17 = a2;
  do {
    if (!*(char *)((long)v17 + 0x34)) {
      v15 = (char *)*v17;
      v11 = strlen(v15);
      *(unsigned int *)((long)v17 + 0x44) = 0xffffffff;
      if (v27 < v11)
        v27 = v11;
      if ((int)v24 == 1) {
        v12 = sub_7bb0(v15);
        v1 = v15[v12];
        v23 = ".";
        v17[10] = sub_7990(v15) - (long)v15;
        v15[v12] = '\0';
        if (v12)
          v23 = (char *)*v17;
        *(unsigned int *)&v17[9] = inotify_add_watch(a1,v23,0x784);
        *(char *)(*v17 + v12) = v1;
        if ((int)v17[9] < 0) {
          v19 = __errno_location();
          if (*v19 != 0x1c) {
            v15 = (char *)sub_b410(4,(char *)*v17);
            v23 = dcgettext(NULL,"cannot watch parent directory of %s",5);
            error(0,*v19,v23,v15); // tail-call
            return;
          }
label_5b10:
          v15 = "inotify resources exhausted";
          error(0,0,dcgettext(NULL,v15,5)); // return-dupe, tail-call
          return;
        }
        v15 = (char *)*v17;
      }
      v9 = inotify_add_watch(a1,v15,v8);
      *(int *)((long)v17 + 0x44) = v9;
      if (0 <= v9) { // branch-flip
        if (!sub_8e70(v10,v17)) {
          sub_c360();
        }
        v6 = 1;
        v24._0_4_ = dat_15020;
      }
      else {
        if ((int)v17[7] != -1)
          v7 = 1;
        v13 = (unsigned int *)__errno_location();
        if ((*v13 & 0xffffffef) == 0xc) goto label_5b10;
        if (*(unsigned int *)((long)v17 + 0x3c) != *v13) {
          v15 = (char *)sub_b410(4,(char *)*v17);
          v23 = dcgettext(NULL,"cannot watch %s",5);
          error(0,*v13,v23,v15);
          v24._0_4_ = dat_15020;
        }
      }
    }
    v17 = &v17[0xc];
  } while (v17 != &a2[a3 * 0xc]);
  if ((int)v24 == 2) {
    if (v7)
      return;
    if (!v6)
      exit(1); // no-return
  }
  v20 = &a2[a3 * 0xc + -0xc];
  v17 = a2;
  do {
    if (!*(char *)((long)v17 + 0x34)) {
      if (dat_15020 != 1) { // branch-flip
        if ((((int)v17[7] != -1) && (v15 = (char *)*v17, !stat(v15,&v21))) && ((v17[4] != CONCAT26(v21._6_2_,CONCAT24(v21._4_2_,v21._0_4_)) || (v17[5] != v21._8_8_)))) {
          v15 = (char *)*v17;
          v18 = sub_b410(4,(char *)sub_5050(v15));
          v15 = dcgettext(NULL,"%s was replaced",5);
          error(0,*__errno_location(),v15,v18);
          return;
        }
      }
      else {
        sub_5270(v17,0);
      }
      sub_5090(v17,&v20);
    }
    v17 = &v17[0xc];
  } while (v17 != &a2[a3 * 0xc]);
  v27 += 0x11;
  v6 = 0;
  v26 = (int *)sub_bfa0(v27);
  v25 = 0;
  v24 = 0;
  v28 = 3;
label_5bf7:
  do {
    if (((dat_15020 == 1) && (!dat_1511a)) && (!sub_83f0(v10))) {
      error(1,0,dcgettext(NULL,"no files remaining",5));
      return;
    }
    if (v25 <= v24) {
      do {
        if (dat_15110) { // branch-flip
          if (v6)
            exit(0); // no-return
          if ((kill(dat_15110,0)) && (*__errno_location() != 1)) {
            v9 = 0;
            v6 = 1;
          }
          else {
            v9 = 0;
            if (0.0 < a0) {
              if (dat_f4e0 <= a0) goto label_5e10;
              v9 = (int)(dat_f4e8 * a0);
              v9 = (unsigned int)((double)v9 < dat_f4e8 * a0) + v9;
            }
          }
        }
        else {
label_5e10:
          v9 = -1;
        }
        v21._4_2_ = 1;
        v21._8_8_ = 1;
        v21._0_4_ = a1;
        v9 = poll(&v21,(unsigned long)dat_15117 + 1,v9);
      } while (!v9);
      if (v9 < 0) {
        v15 = dcgettext(NULL,"error waiting for inotify and output events",5);
        error(1,*__errno_location(),v15);
        return;
      }
      if (v21._14_2_)
        sub_4bd0(); // no-return
      v25 = sub_b6e0(a1,v26,v27);
      if (v25) {
        v12 = 0x10;
        v19 = v26;
        if (v25 != 0xffffffffffffffff) goto label_5c1f;
        if (*__errno_location() != 0x16) {
label_5ed4:
          v15 = dcgettext(NULL,"error reading inotify event",5);
          error(1,*__errno_location(),v15);
          return;
        }
      }
      if (!v28) goto label_5ed4;
      v28 -= 1;
      v27 *= 2;
      v26 = (int *)sub_bff0(v26,v27);
      v25 = 0;
      v24 = 0;
      goto label_5bf7;
    }
    v12 = v24 + 0x10;
    v19 = (int *)((long)v26 + v24);
label_5c1f:
    v2 = v19[3];
    v9 = *v19;
    v24 = v12 + (unsigned long)v2;
    v3 = v19[1];
    if (v3 & 0x400) { // branch-flip
      if (v2) goto label_5c55;
      v17 = &a2[9];
      do {
        if ((int)*v17 == v9) {
          v15 = "directory containing watched file was removed";
          error(0,0,dcgettext(NULL,v15,5));
          return;
        }
        v17 = &v17[0xc];
      } while (&(&a2[9])[a3 * 0xc] != v17);
label_5e28:
      v29 = v9;
      v14 = (struct_14 *)sub_85f0(v10,v22);
      if (v14) {
        v2 = v19[1];
label_5cde:
        if (v2 & 0xe04) { // branch-flip
          if (v2 & 0x400) {
            inotify_rm_watch(a1,v14->field_0x44);
            sub_8ed0(v10,v14);
          }
          sub_5270(v14,0);
        }
        else {
          sub_5090(v14,&v20);
        }
      }
    }
    else {
      if (!v2) goto label_5e28;
label_5c55:
      v11 = 0;
      v17 = a2;
      do {
        if (((int)v17[9] == v9) && (v12 = v17[10], v5 = *v17, !strcmp((char *)&v19[4],(char *)(v12 + v5)))) {
          if (a3 == v11) goto label_5bf7;
          v14 = (struct_14 *)&a2[v11 * 0xc];
          if (!(v3 & 0x200)) {
            v9 = inotify_add_watch(a1,v14->field_0x0,v8);
            if (v9 <= -1) {
              v13 = (unsigned int *)__errno_location();
              if ((*v13 & 0xffffffef) == 0xc) goto label_5b10;
              v15 = (char *)sub_b410(4,v14->field_0x0);
              v23 = dcgettext(NULL,"cannot watch %s",5);
              error(0,*v13,v23,v15);
            }
            v4 = v14->field_0x44;
            if (v4 != v9) { // branch-flip
              if (0 <= v4) {
                inotify_rm_watch(a1);
                sub_8ed0(v10,v14);
              }
            }
            else if (0 <= v4) goto label_5cc7;
            v14->field_0x44 = v9;
            if (v9 == -1) goto label_5bf7;
            v16 = (struct_14 *)sub_8ed0(v10,v14);
            if ((v16) && (v14 != v16)) {
              if (dat_15020 == 1)
                sub_5270(v16,0);
              v16->field_0x44 = 0xffffffff;
              v15 = (char *)sub_5050(v16->field_0x0);
              sub_3b30(v16->field_0x38,v15);
            }
            if (!sub_8e70(v10,v14)) {
              sub_c360();
            }
          }
label_5cc7:
          if (dat_15020 == 1)
            sub_5270(v14,0);
          v2 = v19[1];
          goto label_5cde;
        }
        v11 += 1;
        v17 = &v17[0xc];
      } while (a3 != v11);
    }
  } while( true );
}


// Function: sub_6220 @ 0x6220
void sub_6220(double a0,unsigned long *a1,long a2)
{
  unsigned int v1;
  int v10; // eax
  long v11;
  char *v12; // rax
  char *v13; // rax
  int *v14; // rax
  unsigned long v15;
  unsigned long *v16;
  unsigned long *v17; // rbx
  stat v18; // stack - 0xd8
  long v19; // rbp
  int v2;
  unsigned int v20;
  bool v21;
  char v22 [16];
  long v23; // stack - 0x100
  long v3;
  bool v4;
  bool v5;
  bool v6;
  undefined4 v7;
  int v8; // eax
  unsigned int v9;
  
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
        v2 = *(int *)&v16[7];
        if (v2 <= -1) break;
        v12 = (char *)sub_5050((char *)*v16);
        v1 = *(unsigned int *)&v16[6];
        v9 = v20;
        if (*(unsigned int *)&v16[8] != v20) {
          v8 = sub_7d50(v2,3);
          if (v20) { // branch-flip
            if (0 <= v8) goto label_6309;
          }
          else if ((0 <= v8) && ((v8 == (v8 | 0x800) || (sub_7d50(v2,4) != -1)))) {
label_6309:
            *(unsigned int *)&v16[8] = v20;
            goto label_6310;
          }
          v14 = __errno_location();
          if (((*(unsigned int *)&v16[6] & 0xf000) != 0x8000) || (*v14 != 1)) {
            v12 = (char *)sub_b510(0,3,v12);
            v13 = dcgettext(NULL,"%s: cannot change nonblocking mode",5);
            error(1,*v14,v13,v12);
            return;
          }
          v9 = *(unsigned int *)&v16[8];
        }
label_6310:
        v15 = 0xfffffffffffffffe;
        if (v9) { // branch-flip
label_631f:
          v11 = sub_3f80(0,v12,v2,v15);
          v16[1] = v16[1] + v11;
          v6 = (bool)(v6 | v11 != 0);
        }
        else {
          v10 = fstat(v2,&v18);
          v7 = v18._24_4_;
          if (!v10) {
            if ((*(unsigned int *)&v16[6] == v18._24_4_) && (((v18._24_4_ & 0xf000) != 0x8000 || (v16[1] == v18._48_8_)))) {
              v22 = sub_b7b0(&v18);
              v11 = v16[2];
              v3 = v16[3];
              if (!sub_b800(v11,v3,SUB168(v22,0),SUB168(v22,8))) {
                v15 = v16[0xb];
                v16[0xb] = v15 + 1;
                if ((dat_15018 <= v15) && (dat_15020 == 1)) {
                  sub_5270(v16,*(int *)&v16[8] != 0);
                  v16[0xb] = 0;
                }
                goto label_633a;
              }
            }
            v22 = sub_b7b0(&v18);
            v16[0xb] = 0;
            v16[2] = SUB168(v22,0);
            v16[3] = SUB168(v22,8);
            *(unsigned int *)&v16[6] = v7;
            if (((v1 & 0xf000) == 0x8000) && ((long)v18._48_8_ < (long)v16[1])) {
              v13 = (char *)sub_b510(0,3,v12);
              error(0,0,dcgettext(NULL,"%s: file truncated",5),v13);
              sub_41c0(v2,0,0,v12);
              v16[1] = 0;
            }
            if ((v23 != v19) && (dat_15115))
              sub_3a10(v12);
            v23 = v19;
            if (*(int *)&v16[8]) // branch-flip
              v15 = 0xfffffffffffffffe;
            else {
              v15 = 0xffffffffffffffff;
              if (((v1 & 0xf000) == 0x8000) && (*(char *)((long)v16 + 0x35)))
                v15 = v18._48_8_ - v16[1];
            }
            goto label_631f;
          }
          *(unsigned int *)&v16[7] = 0xffffffff;
          v14 = __errno_location();
          *(int *)((long)v16 + 0x3c) = *v14;
          v12 = (char *)sub_b510(0,3,v12);
          error(0,*v14,"%s",v12);
          close(v2);
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
      if (fflush_unlocked(stdout)) {
        v12 = dcgettext(NULL,"write error",5);
        error(1,*__errno_location(),v12);
        return;
      }
      sub_4bf0();
      if (!v6) {
        if (v5)
          return;
        if (((dat_15110) && (kill(dat_15110,0))) && (*__errno_location() != 1))
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
void sub_6a90(char *a0,int a1,unsigned long a2,unsigned long *a3)
{
  *a3 = 0;
  if (dat_15119) {
    sub_4c60(a0,a1,a2,a3); // tail-call
    return;
  }
  sub_6820(a0,a1,a2,a3); // tail-call
}


// Function: sub_6ab0 @ 0x6ab0
unsigned int sub_6ab0(struct_16 *a0,unsigned long a1)
{
  unsigned char v1;
  stat v10; // stack - 0xd8
  int v2;
  unsigned int v3; // eax
  int v4; // eax
  int *v5; // rax
  unsigned long v6; // rax
  char *v7;
  char *v8;
  unsigned long v9; // stack - 0xe0
  
  v7 = a0->field_0x0;
  v2 = strcmp(v7,"-");
  if (v2) { // branch-flip
    v4 = sub_7ea0(v7,0);
    v1 = dat_1511a;
    if (!dat_1511a) {
      a0->field_0x36 = 1;
      v7 = a0->field_0x0;
      if (v4 == -1) {
label_6c50:
        v5 = __errno_location();
        if (dat_15118) {
          a0->field_0x38 = 0xffffffff;
          v2 = *v5;
          a0->field_0x34 = v1 ^ 1;
          a0->field_0x3c = v2;
          a0->field_0x28 = 0;
          a0->field_0x20 = 0;
        }
        v7 = a0->field_0x0;
        v7 = (char *)sub_b410(4,(char *)sub_5050(v7));
        v8 = dcgettext(NULL,"cannot open %s for reading",5);
        error(0,*v5,v8,v7);
        return 0;
      }
      goto label_6b04;
    }
    v7 = a0->field_0x0;
    if (v4 == -1) {
      a0->field_0x36 = 0;
      goto label_6c50;
    }
  }
  else {
    dat_1510e = 1;
    v4 = 0;
  }
  a0->field_0x36 = 1;
label_6b04:
  if (dat_15115) {
    sub_3a10((char *)sub_5050(v7));
    v7 = a0->field_0x0;
  }
  v3 = sub_6a90((char *)sub_5050(v7),v4,a1,&v9);
  if (dat_15118) { // branch-flip
    a0->field_0x3c = (v3 & 0xff) - 1;
    if (0 <= fstat(v4,&v10)) { // branch-flip
      if (((v10._24_4_ & 0xf000) - 0x1000 & 0xffffe000) && ((v10._24_4_ & 0xb000) != 0x8000)) {
        a0->field_0x3c = 0xffffffff;
        v7 = "";
        a0->field_0x36 = 0;
        a0->field_0x34 = dat_1511a ^ 1;
        if (dat_1511a ^ 1)
          v7 = dcgettext(NULL,"; giving up on this name",5);
        v8 = a0->field_0x0;
        v8 = (char *)sub_b510(0,3,(char *)sub_5050(v8));
        error(0,0,dcgettext(NULL,"%s: cannot follow end of this type of file%s",5),v8,v7);
      }
      else if ((char)v3) {
        sub_3830(a0,v4,v9,&v10,-(unsigned int)(v2 == 0) | 1);
        v7 = (char *)sub_5050(a0->field_0x0);
        a0->field_0x35 = sub_4a40(v4,v7);
        return v3;
      }
    }
    else {
      v5 = __errno_location();
      v7 = a0->field_0x0;
      a0->field_0x3c = *v5;
      v7 = (char *)sub_b410(4,(char *)sub_5050(v7));
      v8 = dcgettext(NULL,"error reading %s",5);
      error(0,*v5,v8,v7);
    }
    v7 = a0->field_0x0;
    v3 = 0;
    a0->field_0x34 = dat_1511a ^ 1;
    sub_3b30(v4,(char *)sub_5050(v7));
    a0->field_0x38 = 0xffffffff;
  }
  else if ((v2) && (close(v4))) {
    v7 = a0->field_0x0;
    v6 = sub_b410(4,(char *)sub_5050(v7));
    v7 = dcgettext(NULL,"error reading %s",5);
    v3 = 0;
    error(0,*__errno_location(),v7,v6);
  }
  return v3;
}


// Function: sub_6e60 @ 0x6e60
void sub_6e60(int a0)
{
  FILE *v1;
  char *v2;
  
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
  unsigned long v1; // rax
  unsigned long v2;
  unsigned long v3; // rdx
  unsigned long v4; // rsi
  long v5; // r8
  
  v1 = SUB168(ZEXT816(a0) * ZEXT816(0xaaaaaaaaaaaaaaab),0);
  v3 = a0 % 3;
  if ((10 <= a0) && (v3)) {
    v5 = 0x10;
    v4 = 9;
    v2 = 3;
    do {
      v2 += 2;
      v4 += v5;
      v1 = a0 / v2;
      v3 = a0 % v2;
      if (a0 <= v4) break;
      v5 += 8;
    } while (v3);
  }
  v2 = v1 >> 8;
  v1 = CONCAT71((undefined7)v2,v3 != 0);
  return v1;
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
unsigned long sub_8000(void *a0) // return-dupe
{
  float v1;
  float *v2;
  float v3; // xmm1_da
  
  v2 = *(float **)((long)a0 + 0x28);
  if (v2 == (float *)0xf5d0)
    return 1;
  v1 = v2[2];
  if ((((dat_f5e4 < v1) && (v1 < dat_f5e8)) && (dat_f5ec < v2[3])) && (dat_f600 <= *v2)) {
    v3 = *v2 + dat_f5e4;
    if (((v3 < v2[1]) && (v2[1] <= dat_f5f0)) && (v3 < v1))
      return 1;
  }
  *(unsigned long *)((long)a0 + 0x28) = 0xf5d0;
  return 0;
}


// Function: sub_8090 @ 0x8090
void sub_8090(struct_9 *a0,struct_8 *a1)
{
  unsigned long v1;
  
  v1 = a0->field_0x48;
  a1->field_0x0 = 0;
  a1->field_0x8 = v1;
  a0->field_0x48 = a1;
}


// Function: sub_80b0 @ 0x80b0
long sub_80b0(struct_6 *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  v1 = (*a0->field_0x30)(a1,a0->field_0x10);
  if (v1 < (unsigned long)a0->field_0x10)
    return v1 * 0x10 + a0->field_0x0;
  abort(); // no-return
}


// Function: sub_80e0 @ 0x80e0
long sub_80e0(struct_7 *a0,long a1,unsigned long *a2,bool a3) // early-return
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
    v1 = a0->field_0x38;
    if (!(*v1)(a1)) {
      while( true ) {
        if (!(long *)v4[1])
          return 0;
        if ((*(long *)v4[1] == a1) || (v1 = a0->field_0x38, (*v1)(a1))) break;
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
void sub_81e0(struct_9 *a0)
{
  if (a0->field_0x48) {
    a0->field_0x48 = *(unsigned long *)(a0->field_0x48 + 8);
    return;
  }
  malloc(0x10); // tail-call
}


// Function: sub_8210 @ 0x8210
unsigned long sub_8210(struct_12 *a0,struct_21 *a1,bool a2) // return-dupe x2
{
  long *v1;
  long v2;
  struct_11 *v3; // rax
  struct_11 *v4; // rax
  long *v5;
  long *v6;
  long v7;
  
  v5 = a1->field_0x0;
  if (a1->field_0x8 <= v5)
    return 1;
  do {
    while (v7 = *v5, !v7) {
label_8238:
      v5 = &v5[2];
      if (a1->field_0x8 <= v5)
        return 1;
    }
    v6 = (long *)v5[1];
    if ((long *)v5[1]) {
      do {
        while( true ) {
          v7 = *v6;
          v3 = (struct_11 *)sub_80b0(a0,v7);
          v1 = (long *)v6[1];
          if (v3->field_0x0) break;
          v3->field_0x0 = v7;
          a0->field_0x18 = a0->field_0x18 + 1;
          sub_8090(a0);
          v6 = v1;
          if (!v1) goto label_82a8;
        }
        v6[1] = v3->field_0x8;
        v3->field_0x8 = (long)v6;
        v6 = v1;
      } while (v1);
label_82a8:
      v7 = *v5;
    }
    v5[1] = 0;
    if (a2) goto label_8238;
    v3 = (struct_11 *)sub_80b0(a0,v7);
    if (v3->field_0x0) { // branch-flip
      v4 = (struct_11 *)sub_81e0(a0);
      if (!v4)
        return 0;
      v2 = v3->field_0x8;
      v4->field_0x0 = v7;
      v4->field_0x8 = v2;
      v3->field_0x8 = (long)v4;
    }
    else {
      v3->field_0x0 = v7;
      a0->field_0x18 = a0->field_0x18 + 1;
    }
    *v5 = 0;
    v5 = &v5[2];
    a1->field_0x18 = a1->field_0x18 + -1;
    if (a1->field_0x8 <= v5)
      return 1;
  } while( true );
}


// Function: sub_83d0 @ 0x83d0
unsigned long sub_83d0(struct_19 *a0)
{
  return a0->field_0x10;
}


// Function: sub_83e0 @ 0x83e0
unsigned long sub_83e0(struct_20 *a0)
{
  return a0->field_0x18;
}


// Function: sub_83f0 @ 0x83f0
unsigned long sub_83f0(struct_5 *a0)
{
  return a0->field_0x20;
}


// Function: sub_8400 @ 0x8400
unsigned long sub_8400(struct_21 *a0)
{
  long v1;
  long *v2;
  unsigned long v3;
  unsigned long v4;
  
  v2 = a0->field_0x0;
  v4 = 0;
  do {
    if (a0->field_0x8 <= v2)
      return v4;
    while (*v2) {
      v3 = 1;
      for (v1 = v2[1]; v1; v1 = *(long *)(v1 + 8)) {
        v3 += 1;
      }
      if (v4 < v3)
        v4 = v3;
      v2 = &v2[2];
      if (a0->field_0x8 <= v2)
        return v4;
    }
    v2 = &v2[2];
  } while( true );
}


// Function: sub_84d0 @ 0x84d0
void sub_84d0(struct_5 *a0,FILE *a1)
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
  __fprintf_chk(a1,1,"# buckets used:    %lu (%.2f%%)\n",v3,((double)v3 * dat_f600) / (double)v2);
  __fprintf_chk(a1,1,"max bucket length: %lu\n",v4); // tail-call
}


// Function: sub_85f0 @ 0x85f0
long sub_85f0(struct_7 *a0,long a1)
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
    v1 = a0->field_0x38;
    if ((*v1)(a1)) break;
    v2 = (long *)v2[1];
    if (!v2)
      return 0;
    v3 = *v2;
  }
  return *v2;
}


// Function: sub_86a0 @ 0x86a0
long sub_86a0(void *a0,long a1)
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
    if (*(long **)((long)a0 + 8) <= v2)
      return 0;
  } while (!*v2);
  return *v2;
}


// Function: sub_8760 @ 0x8760
long sub_8760(struct_21 *a0,void *a1,unsigned long a2)
{
  long *v1;
  long v2;
  long v3; // r12
  long *v4;
  
  v4 = a0->field_0x0;
  if (a0->field_0x8 <= v4) // branch-flip
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
    } while (v4 < a0->field_0x8);
  }
  return v3;
}


// Function: sub_8850 @ 0x8850
unsigned long * sub_8850(unsigned long a0,void *a1,void *a2,void *a3,unsigned long a4) // return-dupe
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
    a1 = (void *)0xf5d0;
  v1[5] = a1;
  if (sub_8000(v1)) {
    v2 = sub_8340(*(float *)((long)a1 + 8),a0,(unsigned char)*(char *)((long)a1 + 0x10));
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
void sub_8960(struct_23 *a0)
{
  unsigned long *v1;
  unsigned long v2;
  void *v3;
  unsigned long *v4;
  long *v5;
  long *v6; // r12
  
  v5 = a0->field_0x0;
  if (a0->field_0x0 < a0->field_0x8) {
    do {
      while (!*v5) {
        v5 = &v5[2];
        if (a0->field_0x8 <= v5) goto label_89fb;
      }
      v3 = a0->field_0x40;
      v4 = (unsigned long *)v5[1];
      if ((unsigned long *)v5[1]) {
        while( true ) {
          if (v3) {
            (*v3)(*v4);
            v3 = a0->field_0x40;
          }
          v1 = (unsigned long *)v4[1];
          v2 = a0->field_0x48;
          *v4 = 0;
          v4[1] = v2;
          a0->field_0x48 = v4;
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
    } while (v6 < a0->field_0x8);
  }
label_89fb:
  a0->field_0x18 = 0;
  a0->field_0x20 = 0;
  return;
}


// Function: sub_8a10 @ 0x8a10
void sub_8a10(struct_18 *a0)
{
  void *v1;
  void *v2;
  long *v3;
  long *v4;
  long v5;
  
  v4 = a0->field_0x0;
  v3 = a0->field_0x8;
  if ((a0->field_0x40) && (a0->field_0x20)) {
    if (v3 <= v4) goto label_8abc;
    do {
      while (v5 = *v4, !v5) {
        v4 = &v4[2];
        if (v3 <= v4) goto label_8a7f;
      }
      v3 = v4;
      while( true ) {
        (*(void *)a0->field_0x40)(v5);
        v3 = (long *)v3[1];
        if (!v3) break;
        v5 = *v3;
      }
      v3 = a0->field_0x8;
      v4 = &v4[2];
    } while (v4 < v3);
label_8a7f:
    v4 = a0->field_0x0;
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
    } while (v4 < a0->field_0x8);
  }
label_8abc:
  v2 = a0->field_0x48;
  while (v2) {
    v1 = *(void **)((long)v2 + 8);
    free(v2);
    v2 = v1;
  }
  free(a0->field_0x0);
  free(a0); // tail-call
}


// Function: sub_8b00 @ 0x8b00
unsigned int sub_8b00(struct_13 *a0,unsigned long a1) // return-dupe
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
  
  v4 = sub_8340(*(float *)(a0->field_0x28 + 8),a1,(unsigned char)*(char *)(a0->field_0x28 + 0x10));
  if (!v4)
    return 0;
  if (a0->field_0x10 == v4)
    return 1;
  v2 = calloc(v4,0x10);
  if (!v2)
    return 0;
  v3 = (void *)((long)v2 + v4 * 0x10);
  v5 = 0;
  v7 = a0->field_0x28;
  v6 = 0;
  v8 = a0->field_0x30;
  v9 = a0->field_0x38;
  v10 = a0->field_0x40;
  v11 = a0->field_0x48;
  v1 = sub_8210(&v2,a0,0);
  if (!(char)v1) {
    a0->field_0x48 = v11;
    if ((sub_8210(a0,&v2,1)) && (sub_8210(a0,&v2,0))) {
      free(v2);
      return v1;
    }
    abort(); // no-return
  }
  free(a0->field_0x0);
  a0->field_0x0 = v2;
  a0->field_0x8 = v3;
  a0->field_0x10 = v4;
  a0->field_0x18 = v5;
  a0->field_0x48 = v11;
  return v1;
}


// Function: sub_8c80 @ 0x8c80
unsigned long sub_8c80(void *a0,long a1,long *a2) // ternary
{
  long v1;
  struct_11 *v2; // rax
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
    v6 = (float)*(unsigned long *)((long)a0 + 0x18);
    v5 = (float)*(unsigned long *)((long)a0 + 0x10);
    if (*(float *)(*(long *)((long)a0 + 0x28) + 8) * v5 < v6) {
      sub_8000(a0);
      v1 = *(long *)((long)a0 + 0x28);
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
      *(long *)((long)a0 + 0x20) = *(long *)((long)a0 + 0x20) + 1;
      *(long *)((long)a0 + 0x18) = *(long *)((long)a0 + 0x18) + 1;
      return 1;
    }
    v2 = (struct_11 *)sub_81e0(a0);
    if (!v2)
      return 0xffffffff;
    v2->field_0x0 = a1;
    v2->field_0x8 = v3[1];
    v3[1] = (long)v2;
    *(long *)((long)a0 + 0x20) = *(long *)((long)a0 + 0x20) + 1;
    return 1;
  }
  abort();
}


// Function: sub_8e70 @ 0x8e70
unsigned long sub_8e70(struct_7 *a0,unsigned long a1) // early-return
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
long sub_8ed0(void *a0,long a1) // ternary x2
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
  *(long *)((long)a0 + 0x20) = *(long *)((long)a0 + 0x20) + -1;
  if (!*v7) {
    v6 = *(long *)((long)a0 + 0x18) - 1;
    *(unsigned long *)((long)a0 + 0x18) = v6;
    v1 = (0 <= (long)v6) ? *(unsigned long *)((long)a0 + 0x10) : *(unsigned long *)((long)a0 + 0x10); // branch-flip
    v9 = (float)v6;
    v8 = (float)v1;
    if (v9 < **(float **)((long)a0 + 0x28) * v8) {
      sub_8000(a0);
      v2 = *(float **)((long)a0 + 0x28);
      if (v9 < *v2 * v8) {
        v8 = v8 * v2[1];
        if (!*(char *)&v2[4])
          v8 = v8 * v2[2];
        v6 = (dat_f5f8 <= v8) ? (long)(v8 - dat_f5f8) ^ 0x8000000000000000 : (unsigned long)v8;
        if (!sub_8b00(a0,v6)) {
          v4 = *(void **)((long)a0 + 0x48);
          while (v4) {
            v3 = *(void **)((long)v4 + 8);
            free(v4);
            v4 = v3;
          }
          *(unsigned long *)((long)a0 + 0x48) = 0;
        }
      }
    }
    return v5;
  }
  return v5;
}

