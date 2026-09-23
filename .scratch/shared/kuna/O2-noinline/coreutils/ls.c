// Function: main @ 0x4dd0
int main(int argc,char **argv,char **envp)
{
  char **v1;
  unsigned long v10; // stack - 0x50
  void *v2;
  int v3;
  unsigned int v4; // eax
  void *v5;
  long v6;
  unsigned long v7; // stack - 0x58
  unsigned long *v8;
  char v9 [16];
  
  sub_13390(*argv);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  sub_8dd0();
  sub_18230(sub_df00);
  dat_27210 = 0;
  dat_272b8 = 1;
  dat_27380 = NULL;
  dat_27370 = 0x8000000000000000;
  dat_27378 = 0xffffffffffffffff;
  v3 = sub_bda0(argc,argv);
  v6 = (long)v3;
  if ((dat_27312) && (sub_6af0(), dat_27312)) {
    dat_272c0 = 0;
    if ((((dat_272f4) || (sub_6420(0xd))) || ((sub_6420(0xe) && (dat_27390)))) || ((sub_6420(0xc) && (!dat_27338)))) {
label_51c8:
      dat_272fd = 1;
    }
  }
  else if (dat_272f4) goto label_51c8;
  if (((!dat_272f8) && (dat_272f8 = 1, !dat_272f5)) && (dat_27314 != 3))
    dat_272f8 = (-(unsigned int)(dat_27338 == 0) & 0xfffffffe) + 3;
  if (dat_272f6) {
    dat_273c8 = sub_f380(0x1e,NULL,sub_56d0,sub_56e0,sub_61a0);
    if (!dat_273c8)
      sub_16da0(); // no-return
    _obstack_begin((obstack *)0x270e0,0,0,dat_25fd0,dat_25f98);
  }
  dat_272a8 = sub_15d80(getenv("TZ"));
  v2 = dat_25fd0;
  v5 = dat_25f98;
  if (((dat_27330 - 3U & 0xfffffffdU) && (dat_27338)) && ((!dat_27365 && (!dat_2732c)))) {
    dat_272a1 = 0;
    dat_272a0 = 1;
    if (((!dat_272f6) && (!dat_27312)) && (!dat_27314))
      dat_272a0 = dat_272f4;
  }
  else {
    dat_272a1 = 1;
    dat_272a0 = 0;
  }
  dat_272a0 &= 1;
  if (dat_27318) {
    _obstack_begin((obstack *)0x271a0,0,0,dat_25fd0,dat_25f98);
    _obstack_begin((obstack *)0x27140,0,0,v2,v5);
  }
  if (dat_27311) {
    sub_5670();
    dat_27388 = sub_16f20();
    if (!dat_27388)
      dat_27388 = 0x1abb9;
  }
  dat_273b8 = 100;
  dat_273c0 = sub_16b00(100,0xd0);
  dat_273b0 = 0;
  sub_6e20();
  if (1 <= argc - v3) { // branch-flip
    do {
      v1 = &argv[v6];
      v6 += 1;
      sub_a550(*v1,0,1,"");
    } while ((int)v6 < argc);
    if (dat_273b0) { // branch-flip
label_52b2:
      sub_89f0();
      if (!dat_272f5)
        sub_a1e0(NULL,1);
      if (!dat_273b0) goto label_4fe9;
      sub_9f20();
      if (!dat_27380) goto label_50a0;
      sub_7460(10);
    }
    else {
label_4fe9:
      if (argc - v3 <= 1) goto label_5292;
    }
    while (v8 = dat_27380, dat_27380) {
label_5038:
      while( true ) {
        v6 = dat_273c8;
        dat_27380 = (unsigned long *)v8[3];
        if ((dat_273c8) && (!(char *)*v8)) break;
        sub_b180((char *)*v8,(char *)v8[1],(unsigned char)*(char *)&v8[2]);
        sub_61b0(v8);
        dat_272b8 = 1;
        v8 = dat_27380;
        if (!dat_27380) goto label_50a0;
      }
      v9 = sub_6330();
      v10 = SUB168(v9,8);
      v7 = SUB168(v9,0);
      v5 = (void *)sub_fa00(v6,&v7);
      if (!v5)
        __assert_fail("found","src/ls.c",0x70d,"main"); // no-return
      sub_61a0(v5);
      sub_61b0(v8);
    }
  }
  else {
    if (dat_272f5) // branch-flip
      sub_a550(".",3,1,"");
    else {
      sub_6840(".",NULL,1);
    }
    if (dat_273b0) goto label_52b2;
label_5292:
    if (dat_27380) {
      v8 = dat_27380;
      if (dat_27380[3]) goto label_5038;
      dat_272b8 = 0;
      goto label_5038;
    }
  }
label_50a0:
  if ((dat_27312) && (dat_27310)) {
    if ((dat_26060 != 2) || (((memcmp(dat_26068,(void *)0x1add7,2) || (dat_26070 != 1)) || (*dat_26078 != 'm'))))
      sub_7cc0();
    fflush_unlocked(stdout);
    sub_7dd0();
    for (v3 = dat_27214; v3; v3 = v3 + -1) {
      raise(0x13);
    }
    if (dat_27218)
      raise(dat_27218);
  }
  if (dat_27318) {
    sub_8060("//DIRED//",(void *)0x271a0);
    sub_8060("//SUBDIRED//",(void *)0x27140);
    v4 = sub_151e0(dat_272d0);
    __printf_chk(1,"//DIRED-OPTIONS// --quoting-style=%s\n",*(char **)((unsigned long)v4 * 8 + 0x259e0));
  }
  v6 = dat_273c8;
  if (dat_273c8) {
    if (sub_ef20(dat_273c8))
      __assert_fail("hash_get_n_entries (active_dir_set) == 0","src/ls.c",0x741,"main"); // no-return
    sub_f540(v6);
  }
  return dat_27210;
}


// Function: sub_55e0 @ 0x55e0
unsigned int sub_55e0(void *a0)
{
  if ((dat_27390) && (*(char *)((long)a0 + 0xb9)))
    return *(unsigned int *)((long)a0 + 0xac);
  return *(unsigned int *)((long)a0 + 0x30);
}


// Function: sub_5620 @ 0x5620
char * sub_5620(char *a0) // return-dupe
{
  char v1;
  char v2;
  
  v2 = *a0;
  if (!v2)
    return NULL;
  do {
    v1 = a0[1];
    if (v2 != '%') { // branch-flip
label_5639:
      v2 = v1;
    }
    else {
      if (v1 != '%') {
        if (v1 == 'b')
          return a0;
        goto label_5639;
      }
      v2 = a0[2];
      a0 = &a0[1];
    }
    a0 = &a0[1];
    if (!v2)
      return NULL;
  } while( true );
}


// Function: sub_5670 @ 0x5670
void sub_5670(void)
{
  unsigned int v1; // ebx
  unsigned long v2; // rbx
  bool v3; // al
  
  v2 = 0;
  do {
    v1 = (int)v2;
    v3 = 1;
    if (!sub_d3f0(v2 & 0xffffffff)) {
      if ((2 <= (unsigned int)(v1 - 0x2dU)) && (v1 != 0x7e))
        v3 = v1 == 0x5f;
      else {
        v3 = 1;
      }
    }
    *(unsigned char *)(v2 + 0x26fe0) = *(unsigned char *)(v2 + 0x26fe0) | v3;
    v2 += 1;
  } while (v2 != 0x100);
}


// Function: sub_56d0 @ 0x56d0
unsigned long sub_56d0(unsigned long *a0,unsigned long a1)
{
  return *a0 % a1;
}


// Function: sub_56e0 @ 0x56e0
unsigned long sub_56e0(struct_0 *a0,struct_0 *a1)
{
  if (a0->field_0x0 != a1->field_0x0)
    return 0;
  return CONCAT71((undefined7)((unsigned long)a1->field_0x8 >> 8),a0->field_0x8 == a1->field_0x8);
}


// Function: sub_5700 @ 0x5700
void sub_5700(unsigned int a0) // return-dupe
{
  if (dat_27218)
    return;
  dat_27218 = a0;
}


// Function: sub_5720 @ 0x5720
void sub_5720(void) // return-dupe
{
  if (dat_27218)
    return;
  dat_27214 += 1;
}


// Function: sub_5740 @ 0x5740
unsigned int sub_5740(long *a0,long *a1,unsigned int a2,long *a3)
{
  unsigned char v1;
  long v10;
  long v11;
  char *v12;
  bool v13;
  unsigned int v2; // eax
  char *v3;
  char *v4;
  char v5;
  char v6;
  long v7;
  long v8;
  long v9; // rdi
  
  v3 = (char *)*a1;
  v11 = *a0;
  v9 = 1;
  v5 = *v3;
  v10 = 0;
  v8 = v11 + 1;
  v6 = v5 + '\xa4';
  v13 = 0;
  v4 = v3;
  v7 = v8;
  if (v5 == '\\') goto label_57a6;
label_577a:
  if (v13 || SBORROW1(v5,'\\') != v6 < '\0') {
    if (v5 != '=') { // branch-flip
      if ((v5 <= '=') && ((!v5 || (v5 == ':')))) {
        v2 = 1;
label_57de:
        *a0 = v11;
        *a1 = (long)v3;
        *a3 = v10;
        return v2;
      }
    }
    else {
      v2 = a2;
      if ((char)a2) goto label_57de;
    }
  }
  else if (v5 == '^') {
    v1 = v3[1];
    if ((unsigned char)(v1 - 0x40) <= 0x3e) {
      v3 = &v3[2];
      v9 += 1;
      v8 = v7 + 1;
      *(unsigned char *)(v7 + -1) = v1 & 0x1f;
      goto label_5796;
    }
    v3 = &v3[1];
    if (v1 == 0x3f) {
      v9 += 1;
      v8 = v7 + 1;
      *(char *)(v7 + -1) = 0x7f;
      goto label_5796;
    }
    v2 = 0;
    goto label_57de;
  }
  v3 = &v3[1];
  v8 = v7;
label_578b:
  *(char *)(v8 + -1) = v5;
  v9 += 1;
  v8 += 1;
label_5796:
  do {
    v5 = *v3;
    v11 = v8 + -1;
    v10 = v9 + -1;
    v6 = v5 + '\xa4';
    v13 = v6 == '\0';
    v4 = v3;
    v7 = v8;
    if (!v13) goto label_577a;
label_57a6:
    v6 = v4[1];
    v3 = &v4[2];
    if (!v6) {
      v2 = 0;
      goto label_57de;
    }
    v5 = v6 + '\xd0';
    switch(v5) {
      case 0:
      case 1:
      case 2:
      case 3:
      case 4:
      case 5:
      case 6:
      case 7:
        v6 = *v3;
        if (8 <= (unsigned char)(v6 - 0x30U)) goto label_578b;
        do {
          v3 = &v3[1];
          v5 = v6 + '\xd0' + v5 * '\b';
          v6 = *v3;
        } while ((unsigned char)(v6 - 0x30U) <= 7);
        *(char *)(v8 + -1) = v5;
        v9 += 1;
        v8 += 1;
        goto label_5796;
      default:
        goto label_5820;
      case 0xf:
        v6 = '\x7f';
        break;
      case 0x28:
      case 0x48:
        goto label_58b7;
      case 0x2f:
        v6 = ' ';
        break;
      case 0x31:
        v6 = '\a';
        goto label_5820;
      case 0x32:
        v6 = '\b';
        goto label_5820;
      case 0x35:
        v6 = '\x1b';
        break;
      case 0x36:
        v6 = '\f';
        break;
      case 0x3e:
        v6 = '\n';
        break;
      case 0x42:
        v6 = '\r';
        break;
      case 0x44:
        v6 = '\t';
        break;
      case 0x46:
        v6 = '\v';
label_5820:
      
    }
    *(char *)(v8 + -1) = v6;
    v9 += 1;
    v8 += 1;
  } while( true );
label_58b7:
  v6 = v4[2];
  v12 = &v4[3];
  v5 = '\0';
  v3 = &v4[2];
  if ('F' < v6) goto label_5902;
  do {
    if ('A' <= v6) { // branch-flip
      v5 = v6 + '\xc9' + v5 * '\x10';
      v3 = v12;
    }
    else {
      if (10 <= (unsigned char)(v6 - 0x30U)) goto label_578b;
      v5 = v6 + '\xd0' + v5 * '\x10';
      v3 = v12;
    }
    while( true ) {
      v6 = *v3;
      v12 = &v3[1];
      if (v6 <= 'F') break;
label_5902:
      if (6 <= (unsigned char)(v6 + 0x9fU)) goto label_578b;
      v5 = v6 + '\xa9' + v5 * '\x10';
      v3 = v12;
    }
  } while( true );
}


// Function: sub_59b0 @ 0x59b0
void sub_59b0(bool a0) // return-dupe
{
  if (a0) {
    dat_27210 = 2;
    return;
  }
  if (dat_27210)
    return;
  dat_27210 = 1;
}


// Function: sub_59e0 @ 0x59e0
unsigned long sub_59e0(unsigned long a0)
{
  return a0;
}


// Function: sub_59f0 @ 0x59f0
unsigned long sub_59f0(int a0)
{
  if (a0 - 0x16U & 0xffffffefU)
    return sub_55c0(a0); // tail-call
  return 1;
}


// Function: sub_5a10 @ 0x5a10
unsigned long sub_5a10(void *a0)
{
  int v1;
  unsigned long v2;
  bool v3; // al
  unsigned int v4; // eax
  unsigned long v5; // rax
  undefined7 v6; // rax
  
  v1 = *(int *)((long)a0 + 0xa8);
  v2 = v5 >> 8;
  v6 = (undefined7)v2;
  v3 = v1 == 3 || v1 == 9;
  if (v1 != 3 && v1 != 9) {
    v4 = *(unsigned int *)((long)a0 + 0xac) & 0xf000;
    v5 = (unsigned long)CONCAT31((undefined3)(v4 >> 8),v4 == 0x4000);
  }
  return v5;
}


// Function: sub_5a40 @ 0x5a40
void sub_5a40(void *a0,void *a1,void *a2)
{
  unsigned int v1; // eax
  unsigned int v2; // eax
  
  v1 = sub_5a10(a1);
  v2 = sub_5a10(a0);
  if ((v1 & 0xff) == (v2 & 0xff)) {
    (*a2)(); // jump-as-call
    return;
  }
}


// Function: sub_5a80 @ 0x5a80
void sub_5a80(unsigned long *a0,unsigned long *a1,void *a2)
{
  char v1 [16];
  char v2 [16];
  
  v1 = sub_15ca0(&a0[3]);
  v2 = sub_15ca0(&a1[3]);
  if (!sub_162a0(SUB168(v2,0),SUB168(v2,8),SUB168(v1,0),SUB168(v1,8))) {
    (*a2)(*a0,*a1); // jump-as-call
    return;
  }
}


// Function: sub_5af0 @ 0x5af0
void sub_5af0(unsigned long *a0,unsigned long *a1,void *a2)
{
  char v1 [16];
  char v2 [16];
  
  v1 = sub_15cb0(&a0[3]);
  v2 = sub_15cb0(&a1[3]);
  if (!sub_162a0(SUB168(v2,0),SUB168(v2,8),SUB168(v1,0),SUB168(v1,8))) {
    (*a2)(*a0,*a1); // jump-as-call
    return;
  }
}


// Function: sub_5b60 @ 0x5b60
void sub_5b60(unsigned long *a0,unsigned long *a1,void *a2)
{
  char v1 [16];
  char v2 [16];
  
  v1 = sub_15c90(&a0[3]);
  v2 = sub_15c90(&a1[3]);
  if (!sub_162a0(SUB168(v2,0),SUB168(v2,8),SUB168(v1,0),SUB168(v1,8))) {
    (*a2)(*a0,*a1); // jump-as-call
    return;
  }
}


// Function: sub_5bd0 @ 0x5bd0
void sub_5bd0(unsigned long *a0,unsigned long *a1,void *a2)
{
  char v1 [16];
  char v2 [16];
  
  v1 = sub_5610(&a0[3]);
  v2 = sub_5610(&a1[3]);
  if (!sub_162a0(SUB168(v2,0),SUB168(v2,8),SUB168(v1,0),SUB168(v1,8))) {
    (*a2)(*a0,*a1); // jump-as-call
    return;
  }
}


// Function: sub_5c40 @ 0x5c40
unsigned long sub_5c40(long a0,long a1) // early-return
{
  if (a1 <= a0)
    return (unsigned long)(a1 < a0);
  return 0xffffffff;
}


// Function: sub_5c60 @ 0x5c60
void sub_5c60(unsigned long *a0,unsigned long *a1)
{
  sub_5a80(a0,a1,sub_72a0); // tail-call
}


// Function: sub_5c70 @ 0x5c70
void sub_5c70(unsigned long *a0,unsigned long *a1)
{
  sub_5a80(a0,a1,dat_25fc0); // tail-call
}


// Function: sub_5c80 @ 0x5c80
void sub_5c80(unsigned long *a0,unsigned long *a1)
{
  sub_5a80(a1,a0,sub_72a0); // tail-call
}


// Function: sub_5ca0 @ 0x5ca0
void sub_5ca0(unsigned long *a0,unsigned long *a1)
{
  sub_5a80(a1,a0,dat_25fc0); // tail-call
}


// Function: sub_5cc0 @ 0x5cc0
void sub_5cc0(void *a0,void *a1)
{
  sub_5a40(a0,a1,sub_5c60); // tail-call
}


// Function: sub_5cd0 @ 0x5cd0
void sub_5cd0(void *a0,void *a1)
{
  sub_5a40(a0,a1,sub_5c70); // tail-call
}


// Function: sub_5ce0 @ 0x5ce0
void sub_5ce0(void *a0,void *a1)
{
  sub_5a40(a0,a1,sub_5c80); // tail-call
}


// Function: sub_5cf0 @ 0x5cf0
void sub_5cf0(void *a0,void *a1)
{
  sub_5a40(a0,a1,sub_5ca0); // tail-call
}


// Function: sub_5da0 @ 0x5da0
void sub_5da0(unsigned long *a0,unsigned long *a1)
{
  sub_5b60(a0,a1,sub_72a0); // tail-call
}


// Function: sub_5db0 @ 0x5db0
void sub_5db0(unsigned long *a0,unsigned long *a1)
{
  sub_5b60(a0,a1,dat_25fc0); // tail-call
}


// Function: sub_5dc0 @ 0x5dc0
void sub_5dc0(unsigned long *a0,unsigned long *a1)
{
  sub_5b60(a1,a0,sub_72a0); // tail-call
}


// Function: sub_5de0 @ 0x5de0
void sub_5de0(unsigned long *a0,unsigned long *a1)
{
  sub_5b60(a1,a0,dat_25fc0); // tail-call
}


// Function: sub_5e00 @ 0x5e00
void sub_5e00(void *a0,void *a1)
{
  sub_5a40(a0,a1,sub_5da0); // tail-call
}


// Function: sub_5e10 @ 0x5e10
void sub_5e10(void *a0,void *a1)
{
  sub_5a40(a0,a1,sub_5db0); // tail-call
}


// Function: sub_5e20 @ 0x5e20
void sub_5e20(void *a0,void *a1)
{
  sub_5a40(a0,a1,sub_5dc0); // tail-call
}


// Function: sub_5e30 @ 0x5e30
void sub_5e30(void *a0,void *a1)
{
  sub_5a40(a0,a1,sub_5de0); // tail-call
}


// Function: sub_5e40 @ 0x5e40
void sub_5e40(unsigned long *a0,unsigned long *a1)
{
  sub_5bd0(a0,a1,sub_72a0); // tail-call
}


// Function: sub_5e50 @ 0x5e50
void sub_5e50(unsigned long *a0,unsigned long *a1)
{
  sub_5bd0(a0,a1,dat_25fc0); // tail-call
}


// Function: sub_5e60 @ 0x5e60
void sub_5e60(unsigned long *a0,unsigned long *a1)
{
  sub_5bd0(a1,a0,sub_72a0); // tail-call
}


// Function: sub_5e80 @ 0x5e80
void sub_5e80(unsigned long *a0,unsigned long *a1)
{
  sub_5bd0(a1,a0,dat_25fc0); // tail-call
}


// Function: sub_5ea0 @ 0x5ea0
void sub_5ea0(void *a0,void *a1)
{
  sub_5a40(a0,a1,sub_5e40); // tail-call
}


// Function: sub_5eb0 @ 0x5eb0
void sub_5eb0(void *a0,void *a1)
{
  sub_5a40(a0,a1,sub_5e50); // tail-call
}


// Function: sub_5ec0 @ 0x5ec0
void sub_5ec0(void *a0,void *a1)
{
  sub_5a40(a0,a1,sub_5e60); // tail-call
}


// Function: sub_5ed0 @ 0x5ed0
void sub_5ed0(void *a0,void *a1)
{
  sub_5a40(a0,a1,sub_5e80); // tail-call
}


// Function: sub_5ee0 @ 0x5ee0
void sub_5ee0(void *a0,void *a1)
{
  sub_5a40(a0,a1,sub_a1a0); // tail-call
}


// Function: sub_5ef0 @ 0x5ef0
void sub_5ef0(void *a0,void *a1)
{
  sub_5a40(a0,a1,sub_a170); // tail-call
}


// Function: sub_5f00 @ 0x5f00
void sub_5f00(void *a0,void *a1)
{
  sub_5a40(a0,a1,sub_a150); // tail-call
}


// Function: sub_5f10 @ 0x5f10
void sub_5f10(void *a0,void *a1)
{
  sub_5a40(a0,a1,sub_a130); // tail-call
}


// Function: sub_5f90 @ 0x5f90
void sub_5f90(void *a0,void *a1)
{
  sub_5a40(a0,a1,sub_7350); // tail-call
}


// Function: sub_5fa0 @ 0x5fa0
void sub_5fa0(void *a0,void *a1)
{
  sub_5a40(a0,a1,sub_8970); // tail-call
}


// Function: sub_5fb0 @ 0x5fb0
void sub_5fb0(void *a0,void *a1)
{
  sub_5a40(a0,a1,sub_8940); // tail-call
}


// Function: sub_5fc0 @ 0x5fc0
void sub_5fc0(void *a0,void *a1)
{
  sub_5a40(a0,a1,sub_8950); // tail-call
}


// Function: sub_5fd0 @ 0x5fd0
void sub_5fd0(void *a0,void *a1)
{
  sub_5a40(a0,a1,sub_8920); // tail-call
}


// Function: sub_5fe0 @ 0x5fe0
void sub_5fe0(void *a0,void *a1)
{
  sub_5a40(a0,a1,sub_a050); // tail-call
}


// Function: sub_5ff0 @ 0x5ff0
void sub_5ff0(void *a0,void *a1)
{
  sub_5a40(a0,a1,sub_a060); // tail-call
}


// Function: sub_6000 @ 0x6000
void sub_6000(void) // return-dupe
{
  long *v1;
  long v2;
  
  if (!dat_273b0)
    return;
  v1 = dat_273a0;
  v2 = dat_273c0;
  do {
    *v1 = v2;
    v1 = &v1[1];
    v2 += 0xd0;
  } while (v1 != &dat_273a0[dat_273b0]);
}


// Function: sub_6040 @ 0x6040
unsigned char sub_6040(bool a0,unsigned int a1,int a2)
{
  unsigned int v1; // ecx
  bool v2;
  
  if (a0) { // branch-flip
    v1 = a1 & 0xf000;
    if (v1 == 0x8000) {
      if (dat_27314 != 3)
        return 0;
      return -((a1 & 0x49) != 0) & 0x2a;
    }
    if (v1 == 0x4000)
      return 0x2f;
    v2 = v1 == 0xa000;
    if (dat_27314 == 1)
      return 0;
  }
  else {
    if (a2 == 5)
      return 0;
    if ((a2 == 3) || (a2 == 9))
      return 0x2f;
    v2 = a2 == 6;
    if (dat_27314 == 1)
      return 0;
  }
  if (v2)
    return 0x40;
  if (a0) { // branch-flip
    if ((a1 & 0xf000) == 0x1000)
      return 0x7c;
    v2 = (a1 & 0xf000) == 0xc000;
  }
  else {
    if (a2 == 1)
      return 0x7c;
    v2 = a2 == 7;
  }
  return -v2 & 0x3d;
}


// Function: sub_6140 @ 0x6140
void sub_6140(char *a0,char *a1,char *a2)
{
  char v1;
  char v2;
  char *v3; // rcx
  char *v4;
  char *v5; // rdi
  
  v2 = *a1;
  v1 = v2;
  if (v2 == '.')
    v1 = a1[1];
  if (!v1) goto label_618a;
  v4 = a1;
  do {
    v5 = a0;
    v3 = v4;
    v4 = &v3[1];
    a0 = &v5[1];
    *v5 = v2;
    v2 = *v4;
  } while (v2);
  if (v4 <= a1) goto label_618a;
  if (*v3 == '/') goto label_618a;
  *a0 = '/';
  a0 = &v5[2];
  v2 = *a2;
  while (v2) {
    *a0 = v2;
    a0 = &a0[1];
    a2 = &a2[1];
label_618a:
    v2 = *a2;
  }
  *a0 = '\0';
}


// Function: sub_61b0 @ 0x61b0
void sub_61b0(struct_1 *a0)
{
  free(a0->field_0x0);
  free(a0->field_0x8);
  free(a0); // tail-call
}


// Function: sub_61d0 @ 0x61d0
void sub_61d0(unsigned long a0) // return-dupe
{
  unsigned long *v1;
  char v2 [16];
  long v3; // rax
  unsigned long *v4;
  long v5;
  unsigned long v6;
  
  if (dat_262e0 < a0) {
    if ((dat_27200) && (dat_27200 >> 1 <= a0)) {
      dat_27208 = sub_16ad0(dat_27208,dat_27200,0x18);
      v6 = dat_27200;
    }
    else {
      dat_27208 = sub_16ad0(dat_27208,a0,0x30);
      v6 = a0 * 2;
    }
    v2 = ZEXT816(v6 - dat_262e0) * ZEXT816(dat_262e0 + 1 + v6);
    if ((CARRY8(dat_262e0 + 1,v6)) || (SUB168(v2,8)))
      sub_16da0(); // no-return
    v3 = sub_16b00(SUB168(v2,0) >> 1,8);
    if (dat_262e0 < v6) {
      v5 = dat_262e0 * 8 + 8;
      do {
        *(long *)(dat_27208 + -8 + v5 * 3) = v3;
        v3 += v5;
        v5 += 8;
      } while (v5 != v6 * 8 + 8);
    }
    dat_262e0 = v6;
  }
  if (!a0)
    return;
  v5 = 0;
  v3 = 3;
  do {
    v4 = *(unsigned long **)(dat_27208 + -8 + v3 * 8);
    *(char *)(dat_27208 + -0x18 + v3 * 8) = 1;
    v5 += 8;
    *(long *)(dat_27208 + -0x10 + v3 * 8) = v3;
    v1 = (unsigned long *)(v5 + (long)v4);
    do {
      *v4 = 3;
      v4 = &v4[1];
    } while (v1 != v4);
    v3 += 3;
  } while (a0 * 3 + 3 != v3);
}


// Function: sub_6330 @ 0x6330
undefined16 sub_6330(void)
{
  long v1;
  
  v1 = dat_270f8;
  if (0x10 <= (unsigned long)(dat_270f8 - dat_270f0)) {
    dat_270f8 -= 0x10;
    return *(char (*)[16])(v1 + -0x10);
  }
  __assert_fail("dev_ino_size <= obstack_object_size (&dev_ino_obstack)","src/ls.c",0x41d,"dev_ino_pop"); // no-return
}


// Function: sub_6380 @ 0x6380
void sub_6380(unsigned long a0)
{
  unsigned long *v1; // rax
  
  v1 = (unsigned long *)sub_169e0(0x10);
  *v1 = a0;
  v1[1] = dat_272e8;
  dat_272e8 = v1;
}


// Function: sub_63b0 @ 0x63b0
long sub_63b0(unsigned char *a0) // early-return
{
  int v1; // eax
  long v2; // stack - 0x18
  
  v1 = sub_17190(a0,NULL,0,&v2,"");
  if (v1) // branch-flip
    v2 = -(unsigned long)(v1 != 1);
  else if (v2 <= -1)
    return 0;
  return v2;
}


// Function: sub_6420 @ 0x6420
unsigned long sub_6420(unsigned int a0)
{
  long v1;
  char *v2;
  unsigned long v3;
  
  v3 = 0;
  v1 = *(long *)((unsigned long)a0 * 0x10 + 0x26060);
  if (!v1)
    return 0;
  v2 = *(char **)((unsigned long)a0 * 0x10 + 0x26068);
  if (v1 != 1) {
    v3 = 0;
    if (v1 != 2)
      return 1;
    return CONCAT71((undefined7)((unsigned long)v3 >> 8),strncmp(v2,"00",2) != 0) & 0xffffffff;
  }
  return CONCAT71((undefined7)((unsigned long)v3 >> 8),strncmp(v2,"0",1) != 0) & 0xffffffff;
}


// Function: sub_64b0 @ 0x64b0
unsigned int sub_64b0(void) // early-return x2
{
  int v1; // eax
  char *v2; // rax
  
  v2 = getenv("QUOTING_STYLE");
  if (!v2)
    return 0xffffffff;
  v1 = sub_cf70(v2,(long *)0x259e0,(void *)0x1d9c0,4);
  if (0 <= v1)
    return *(unsigned int *)((long)v1 * 4 + 0x1d9c0);
  v2 = (char *)sub_158c0(v2);
  error(0,0,dcgettext(NULL,"ignoring invalid value of environment variable QUOTING_STYLE: %s",5),v2);
  return 0xffffffff;
}


// Function: sub_6590 @ 0x6590
unsigned int sub_6590(void)
{
  unsigned int v1; // eax
  
  if ('\0' <= (char)dat_26018)
    return dat_26018 & 1;
  v1 = isatty(1);
  dat_26018 = (char)v1;
  return v1 & 1;
}


// Function: sub_65c0 @ 0x65c0
unsigned long sub_65c0(long a0)
{
  char *v1; // rax
  void *v2; // rax
  unsigned long v3;
  unsigned long v4; // stack - 0x48
  long v5;
  unsigned long v6;
  int v7; // r15d
  
  v6 = 0xc;
  do {
    v7 = 0x2000e;
    v3 = 0;
    v5 = a0;
    do {
      v4 = v6;
      v1 = nl_langinfo(v7);
      if (strchr(v1,0x25))
        return 0;
      v2 = __ctype_b_loc();
      sub_5570((int)*v1,v5,v2,&v4);
      if (0x80 <= (unsigned long)sub_10df0(v1))
        return 0;
      if (v3 < v4)
        v3 = v4;
      v7 += 1;
      v5 += 0x80;
    } while (v7 != 0x2001a);
    if (v6 <= v3)
      return 1;
    v6 = v3;
  } while( true );
}


// Function: sub_66c0 @ 0x66c0
void sub_66c0(void) // return-dupe x2
{
  char *v1;
  long v2;
  unsigned int v3; // eax
  long v4; // rax
  char *v5;
  char v6 [1536];
  char v7 [8];
  long v8 [2]; // stack - 0x658
  char *v9;
  
  v4 = sub_5620(dat_26040);
  v8[0] = v4;
  v8[1] = sub_5620(dat_26048);
  if (!v4 && !v8[1])
    return;
  if (!sub_65c0(v6))
    return;
  v4 = 0;
  do {
    v1 = *(char **)(v4 * 8 + 0x26040);
    v2 = v8[v4];
    v9 = (char *)(v4 * 0x600 + 0x263e0);
    v5 = v6;
    do {
      if (v2) { // branch-flip
        if (0x80 < v2 - (long)v1)
          return;
        v3 = __snprintf_chk(v9,0x80,1,0xffffffffffffffff,"%.*s%s%s",(int)(v2 - (long)v1),v1,v5,(char *)(v2 + 2));
      }
      else {
        v3 = snprintf(v9,0x80,"%s",v1);
      }
      if (0x7f < v3)
        return;
      v5 = &v5[0x80];
      v9 = &v9[0x80];
    } while (v5 != v7);
    if (v4 == 1) {
      dat_263c8 = 1;
      return;
    }
    v4 = 1;
  } while( true );
}


// Function: sub_6840 @ 0x6840
void sub_6840(char *a0,char *a1,char a2)
{
  unsigned long *v1; // rax
  unsigned long v2;
  
  v1 = (unsigned long *)sub_169e0(0x20);
  v2 = 0;
  if (a1)
    v2 = sub_16d80(a1);
  v1[1] = v2;
  v2 = 0;
  if (a0)
    v2 = sub_16d80(a0);
  *v1 = v2;
  *(char *)&v1[2] = a2;
  v1[3] = dat_27380;
  dat_27380 = v1;
}


// Function: sub_68b0 @ 0x68b0
unsigned int sub_68b0(void)
{
  if (dat_27334 <= 3)
    return *(unsigned int *)&"@"[(unsigned long)dat_27334 * 4];
  abort(); // no-return
}


// Function: sub_68d0 @ 0x68d0
unsigned int sub_68d0(void)
{
  unsigned int v1; // r12d
  
  v1 = (-(unsigned int)(dat_272fc == '\0') & 0xffffff00) + 0x102;
  if (dat_2732c)
    v1 |= 0x400;
  if (!dat_27338) {
    v1 |= sub_68b0();
    if ((dat_26029) || (dat_2732e))
      v1 |= 0x20c;
    else {
      v1 |= 0x204;
    }
    if (dat_26028)
      v1 |= 0x10;
  }
  switch(dat_27330) {
    case 0:
    case 1:
    case 2:
    case 4:
    case 6:
      break;
    case 3:
      return v1 | 0x200;
    case 5:
      v1 |= sub_68b0();
      break;
    default:
      abort(); // no-return
    
  }
  return v1;
}


// Function: sub_6990 @ 0x6990
unsigned long sub_6990(unsigned long *a0,char *a1)
{
  char *v1;
  
  if (!a0)
    return 0;
  do {
    v1 = (char *)*a0;
    if (!fnmatch(v1,a1,4))
      return 1;
    a0 = (unsigned long *)a0[1];
  } while (a0);
  return 0;
}


// Function: sub_69f0 @ 0x69f0
unsigned long sub_69f0(char *a0)
{
  char v1; // al
  unsigned long v2; // rax
  
  if (dat_272f0 != 2) {
    if (*a0 != '.') { // branch-flip
      if ((!dat_272f0) && (v1 = sub_6990(dat_272e0,a0), v1))
        return 1;
    }
    else if ((!dat_272f0) || (!a0[(unsigned long)(a0[1] == '.') + 1]))
      return 1;
  }
  v2 = sub_6990(dat_272e8,a0); // tail-call
  return v2;
}


// Function: sub_6a50 @ 0x6a50
unsigned long sub_6a50(void)
{
  char *v1; // rax
  char *v2;
  
  v1 = getenv("TERM");
  if ((v1) && (*v1)) {
    v2 = "# Configuration file for dircolors, a utility to help you set the";
    do {
      if ((!strncmp(v2,"TERM ",5)) && (!fnmatch(&v2[5],v1,0)))
        return 1;
      v2 = &v2[strlen(v2) + 1];
    } while (&v2[-0x19780] <= (char *)0x12c7);
  }
  return 0;
}


// Function: sub_6af0 @ 0x6af0
void sub_6af0(void)
{
  char *v1;
  char v10; // stack - 0x41
  char v2;
  void *v3;
  char *v4;
  void *v5;
  long v6; // rcx
  unsigned short v7; // stack - 0x43
  char *v8; // stack - 0x58
  void *v9; // stack - 0x50
  
  v8 = getenv("LS_COLORS");
  if ((v8) && (*v8)) {
    v7 = 0x3f3f;
    v10 = 0;
    dat_27300 = (void *)sub_16d80(v8);
    v9 = dat_27300;
    do {
      while( true ) {
        while (v2 = *v8, v2 == '*') {
          v4 = &v8[1];
          v5 = (void *)sub_169e0(0x28);
          *(void **)((long)v5 + 0x20) = dat_27308;
          *(void **)((long)v5 + 8) = v9;
          dat_27308 = v5;
          v8 = v4;
          v4 = v8;
          if ((!sub_5740(&v9,&v8,1,v5)) || (v4 = &v8[1], *v8 != '=')) goto label_6c65;
          *(void **)((long)v5 + 0x18) = v9;
          v8 = v4;
          v4 = v8;
          if (!sub_5740(&v9,&v8,0,(long *)((long)v5 + 0x10))) goto label_6c65;
        }
        if (v2 != ':') break;
        v8 = &v8[1];
      }
      if (!v2) goto label_6cb8;
      v7 = CONCAT11(v7._1_1_,v2);
      v4 = &v8[1];
      if (!v8[1]) goto label_6c65;
      v7 = CONCAT11(v8[1],v2);
      v1 = &v8[2];
      v8 = &v8[3];
      v4 = v8;
      if (*v1 != '=') goto label_6c65;
      v6 = 0;
      v4 = "lc";
      while (strcmp((char *)&v7,v4)) {
        v6 += 1;
        v4 = *(char **)(v6 * 8 + 0x25860);
        if (!v4) goto label_6c36;
      }
      v6 = (long)(int)v6 * 0x10;
      *(void **)(v6 + 0x26068) = v9;
    } while (sub_5740(&v9,&v8,0,(long *)(v6 + 0x26060)));
label_6c36:
    v4 = (char *)sub_158c0(&v7);
    error(0,0,dcgettext(NULL,"unrecognized prefix: %s",5),v4);
    v4 = v8;
label_6c65:
    v8 = v4;
    error(0,0,dcgettext(NULL,"unparsable value for LS_COLORS environment variable",5));
    free(dat_27300);
    v5 = dat_27308;
    while (v5) {
      v3 = *(void **)((long)v5 + 0x20);
      free(v5);
      v5 = v3;
    }
    dat_27312 = 0;
label_6cb8:
    if ((dat_260d0 == 6) && (!strncmp(dat_260d8,"target",6)))
      dat_27390 = 1;
  }
  else {
    v4 = getenv("COLORTERM");
    if (((!v4) || (!*v4)) && (!sub_6a50()))
      dat_27312 = 0;
  }
}


// Function: sub_6dc0 @ 0x6dc0
void sub_6dc0(struct_3 *a0)
{
  char *v1;
  
  free(a0->field_0x0);
  free(a0->field_0x8);
  free(a0->field_0x10);
  v1 = a0->field_0xb0;
  if (v1 == (char *)0x2602a)
    return;
  if (sub_55d0()) {
    free(v1); // tail-call
    return;
  }
  freecon(v1); // tail-call
}


// Function: sub_6e20 @ 0x6e20
void sub_6e20(void)
{
  long v1;
  unsigned long v2; // rbx
  
  if (dat_273b0) {
    v2 = 0;
    do {
      v1 = v2 * 8;
      v2 += 1;
      sub_6dc0(*(struct_3 **)(dat_273a0 + v1));
    } while (v2 < dat_273b0);
  }
  dat_273b0 = 0;
  dat_273a9 = 0;
  dat_27364 = 0;
  dat_27360 = 0;
  dat_2735c = 0;
  dat_27358 = 0;
  dat_27350 = 0;
  dat_2734c = 0;
  dat_27348 = 0;
  dat_27354 = 0;
  dat_27344 = 0;
  dat_27340 = 0;
  dat_2733c = 0;
}


// Function: sub_6ed0 @ 0x6ed0
unsigned long sub_6ed0(char *a0,unsigned long a1) // early-return
{
  unsigned int v1; // eax
  
  if (!a0)
    return __snprintf_chk(NULL,0,1,0xffffffffffffffff,"%lu",a1); // tail-call
  v1 = sub_113a0(a0,0);
  if (0 <= (int)v1)
    return (unsigned long)v1;
  return 0;
}


// Function: sub_6f10 @ 0x6f10
char * sub_6f10(void *a0,unsigned long a1,void *a2)
{
  unsigned long v1;
  
  if (!*(char *)((long)a2 + 0xb8))
    return "?";
  v1 = *(unsigned long *)((long)a2 + 0x20);
  if (!v1)
    return "?";
  return (char *)sub_10c20(v1,a0); // tail-call
}


// Function: sub_6f50 @ 0x6f50
void sub_6f50(char a0,char *a1,char *a2)
{
  unsigned long v1; // rax
  
  v1 = sub_15600(4,a2);
  error(0,*__errno_location(),a1,v1);
  sub_59b0((unsigned char)a0); // tail-call
}


// Function: sub_6f90 @ 0x6f90
int sub_6f90(char *a0,struct_10 *a1,bool a2)
{
  int v1;
  int v2;
  
  if (a1->field_0x18 != dat_262f0) { // branch-flip
    if (a2) // branch-flip
      v2 = sub_15b10(a0,&a1[5].field_0x0[0x10]);
    else {
      v2 = sub_15b30();
    }
    if (v2 <= -1) {
      v1 = *__errno_location();
      if (sub_59f0(v1)) {
        dat_262f0 = a1->field_0x18;
        return v2;
      }
    }
  }
  else {
    v2 = -1;
    *__errno_location() = 0x5f;
  }
  return v2;
}


// Function: sub_7020 @ 0x7020
unsigned long sub_7020(char *a0,struct_10 *a1)
{
  int v1;
  unsigned int v2; // eax
  int *v3; // rax
  unsigned long v4; // r8
  
  v3 = __errno_location();
  if (a1->field_0x18 == dat_262e8) {
    *v3 = 0x5f;
    return 0;
  }
  *v3 = 0;
  v2 = sub_e060(a0,&a1->field_0x18);
  v4 = (unsigned long)v2;
  if ((int)v2 <= 0) {
    v1 = *v3;
    if (sub_59f0(v1)) {
      dat_262e8 = a1->field_0x18;
      return v4 & 0xffffffff;
    }
  }
  return v4 & 0xffffffff;
}


// Function: sub_70b0 @ 0x70b0
void sub_70b0(char *a0,void *a1,char a2)
{
  long v1; // rax
  
  v1 = sub_cdd0(a0,*(unsigned long *)((long)a1 + 0x48));
  *(long *)((long)a1 + 8) = v1;
  if (v1)
    return;
  sub_6f50((unsigned char)a2,dcgettext(NULL,"cannot read symbolic link %s",5),a0); // tail-call
}


// Function: sub_7100 @ 0x7100
char * sub_7100(char *a0,char *a1)
{
  unsigned long v1; // rax
  char *v2; // rax
  
  if (a1) { // branch-flip
    if ((*a1 == '/') || (v1 = sub_dfb0(a0), !v1))
      return (char *)sub_16d80(a1); // tail-call
    v2 = (char *)sub_169e0(v1 + 2 + strlen(a1));
    if (a0[v1 - 1] != '/')
      v1 += 1;
    strcpy(stpncpy(v2,a0,v1),a1);
  }
  else {
    v2 = NULL;
  }
  return v2;
}


// Function: sub_71a0 @ 0x71a0
unsigned long sub_71a0(char *a0) // early-return
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  char v3 [2]; // stack - 0x22
  
  v1 = sub_152c0(v3,2,a0,0xffffffffffffffff,dat_272d0);
  if (*a0 != v3[0])
    return 1;
  v2 = strlen(a0);
  return CONCAT71((undefined7)(v2 >> 8),v2 != v1);
}


// Function: sub_7220 @ 0x7220
void sub_7220(unsigned long a0)
{
  if (dat_2732d) {
    sub_6ed0(NULL,a0 & 0xffffffff); // tail-call
    return;
  }
  sub_6ed0((char *)sub_109e0(a0),a0 & 0xffffffff); // tail-call
}


// Function: sub_7260 @ 0x7260
void sub_7260(unsigned long a0)
{
  if (dat_2732d) {
    sub_6ed0(NULL,a0 & 0xffffffff); // tail-call
    return;
  }
  sub_6ed0((char *)sub_10840(a0),a0 & 0xffffffff); // tail-call
}


// Function: sub_72a0 @ 0x72a0
void sub_72a0(char *a0,char *a1)
{
  *__errno_location() = 0;
  strcoll(a0,a1); // tail-call
}


// Function: sub_72d0 @ 0x72d0
void sub_72d0(unsigned long *a0,unsigned long *a1,void *a2)
{
  char *v1; // rax
  char *v2; // rax
  
  v1 = strrchr((char *)*a0,0x2e);
  v2 = strrchr((char *)*a1,0x2e);
  if (!v2)
    v2 = "";
  if (!v1)
    v1 = "";
  if (!(*a2)(v1,v2)) {
    (*a2)(*a0,*a1); // jump-as-call
    return;
  }
}


// Function: sub_7350 @ 0x7350
void sub_7350(unsigned long *a0,unsigned long *a1)
{
  sub_72d0(a1,a0,dat_25fc0); // tail-call
}


// Function: sub_73b0 @ 0x73b0
void sub_73b0(char *a0)
{
  sub_5580((char *)sub_d360(a0)); // tail-call
}


// Function: sub_73d0 @ 0x73d0
unsigned long sub_73d0(unsigned long a0,unsigned long a1)
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3;
  
  if (a1 <= a0)
    return v2;
  do {
    while ((v3 = a0 + 1, !dat_272c0 || (a1 / dat_272c0 <= v3 / dat_272c0))) {
      v1 = putchar_unlocked(0x20);
      a0 = v3;
      if (a1 <= v3)
        return CONCAT44(dat_4,v1);
    }
    putchar_unlocked(9);
    v3 = a0 / dat_272c0;
    a0 = (dat_272c0 + a0) - a0 % dat_272c0;
  } while (a0 < a1);
  return v3;
}


// Function: sub_7460 @ 0x7460
void sub_7460(char a0)
{
  dat_271f8 += 1;
  putchar_unlocked((int)a0); // tail-call
}


// Function: sub_7480 @ 0x7480
unsigned long * sub_7480(unsigned long *a0,bool a1) // early-return x2
{
  unsigned long v1;
  unsigned long *v10;
  char v11;
  char *v2;
  unsigned long *v3;
  unsigned int v4;
  long v5; // rax
  unsigned long v6; // rax
  unsigned int v7;
  char *v8;
  unsigned int v9; // esi
  
  if (a1) { // branch-flip
    v8 = (char *)a0[1];
    v4 = *(unsigned int *)((long)a0 + 0xac);
    if (!*(char *)((long)a0 + 0xb9)) {
      v9 = 0xffffffff;
      if (sub_6420(0xc)) {
        v5 = 0xc0;
        goto label_750f;
      }
      goto label_7542;
    }
    v11 = *(char *)&a0[0x17];
    v9 = 0;
    v7 = 0;
    if (v11) goto label_7551;
label_74b9:
    v9 = v7;
    v4 = *(unsigned int *)&"\r"[(unsigned long)*(unsigned int *)&a0[0x15] * 4];
    v6 = (unsigned long)v4;
    v11 = v4 == 7;
    if (v4 == 5) {
label_7660:
      v6 = strlen(v8);
      for (v3 = dat_27308; v3; v3 = (unsigned long *)v3[4]) {
        v1 = *v3;
        if ((v1 <= v6) && (v2 = (char *)v3[1], !sub_d5c0(&v8[v6 - v1],v2))) {
          if (!v3[3])
            return NULL;
          return &v3[2];
        }
      }
      v5 = 0x50;
      goto label_750f;
    }
  }
  else {
    v10 = a0;
    v4 = sub_55e0(a0);
    v8 = (char *)*v10;
    v9 = (unsigned int)*(unsigned char *)((long)v10 + 0xb9);
label_7542:
    v11 = *(char *)&a0[0x17];
    v7 = v9;
    if (!v11) goto label_74b9;
label_7551:
    v7 = v4 & 0xf000;
    if (v7 == 0x8000) {
      if ((v4 & 0x800) && (sub_6420(0x10))) {
        v5 = 0x100;
        goto label_750f;
      }
      if ((v4 & 0x400) && (sub_6420(0x11))) {
        v5 = 0x110;
        goto label_750f;
      }
      if ((sub_6420(0x15)) && (*(char *)&a0[0x18])) {
        v5 = 0x150;
        goto label_750f;
      }
      if ((v4 & 0x49) && (sub_6420(0xe))) {
        v5 = 0xe0;
        goto label_750f;
      }
      if ((2 <= (unsigned long)a0[5]) && (sub_6420(0x16))) {
        v5 = 0x160;
        goto label_750f;
      }
      goto label_7660;
    }
    if (v7 == 0x4000) {
      if ((v4 & 0x202) == 0x202) {
        v5 = 0x140;
        if (sub_6420(0x14)) goto label_750f;
      }
      if (v4 & 2) {
        v5 = 0x130;
        if (sub_6420(0x13)) goto label_750f;
      }
      v5 = 0x60;
      if (v4 & 0x200)
        v5 = (-(unsigned long)(sub_6420(0x12) == '\0') & 0xffffffffffffff40) + 0x120;
      goto label_750f;
    }
    if (v7 != 0xa000) {
      v5 = 0x80;
      if ((((v7 != 0x1000) && (v5 = 0x90, v7 != 0xc000)) && (v5 = 0xa0, v7 != 0x6000)) && (v5 = 0xd0, v7 == 0x2000))
        v5 = 0xb0;
      goto label_750f;
    }
    v6 = 7;
  }
  if ((v9) || (!v11))
    v5 = v6 << 4;
  else {
    v5 = 0xd0;
    if (!dat_27390)
      v5 = (-(unsigned long)(sub_6420(0xd) == '\0') & 0xffffffffffffffa0) + 0xd0;
  }
label_750f:
  if (!*(long *)(v5 + 0x26068))
    return NULL;
  return (unsigned long *)(v5 + 0x26060);
}


// Function: sub_77d0 @ 0x77d0
void sub_77d0(char *a0,unsigned long a1)
{
  dat_271f8 += a1;
  fwrite_unlocked(a0,1,a1,stdout); // tail-call
}


// Function: sub_77f0 @ 0x77f0
void sub_77f0(char *a0)
{
  sub_77d0(a0,strlen(a0)); // tail-call
}


// Function: sub_7810 @ 0x7810
void sub_7810(void)
{
  if (!dat_27318)
    return;
  sub_77f0("  "); // tail-call
}


// Function: sub_7830 @ 0x7830
char * sub_7830(char *a0,bool a1)
{
  char v1;
  unsigned char v2; // al
  char *v3; // rax
  char v4; // dl
  char *v5;
  char *v6; // r15
  
  v3 = (char *)sub_16b00(3,strlen(a0) + 1);
  v1 = *a0;
  v5 = v3;
  do {
    while( true ) {
      if (!v1) {
        *v5 = '\0';
        return v3;
      }
      a0 = &a0[1];
      if ((v1 != '/') || (!a1)) break;
      *v5 = '/';
label_787f:
      v5 = &v5[1];
      v1 = *a0;
    }
    v2 = sub_5570((int)v1);
    if (*(char *)((unsigned long)v2 + 0x26fe0)) {
      *v5 = v4;
      goto label_787f;
    }
    v6 = &v5[3];
    __sprintf_chk(v5,1,0xffffffffffffffff,"%%%02x",(unsigned long)v2);
    v1 = *a0;
    v5 = v6;
  } while( true );
}


// Function: sub_7900 @ 0x7900
void sub_7900(obstack *a0)
{
  if (!dat_27318)
    return;
  if (8 <= (unsigned long)(*(long *)&a0->field_0x20 - (long)*(unsigned long **)&a0->field_0x18)) {
    **(unsigned long **)&a0->field_0x18 = dat_271f8;
    *(long *)&a0->field_0x18 = *(long *)&a0->field_0x18 + 8;
    return;
  }
  _obstack_newchunk(a0,8);
  **(unsigned long **)&a0->field_0x18 = dat_271f8;
  *(long *)&a0->field_0x18 = *(long *)&a0->field_0x18 + 8;
}


// Function: sub_7970 @ 0x7970
void sub_7970(unsigned long a0,unsigned long a1)
{
  unsigned long *v1;
  
  v1 = dat_270f8;
  if ((unsigned long)(dat_27100 - (long)dat_270f8) <= 0xf) {
    _obstack_newchunk((obstack *)0x270e0,0x10);
    v1 = dat_270f8;
  }
  dat_270f8 = &v1[2];
  v1[1] = a0;
  *v1 = a1;
}


// Function: sub_79d0 @ 0x79d0
void sub_79d0(bool a0) // return-dupe
{
  int *v1; // rbx
  void *v2 [17]; // stack - 0xd8
  int v3;
  int *v4; // rbp
  undefined4 v5; // stack - 0x50
  
  if (a0) {
    v1 = (int *)0x19644;
    v3 = 0x14;
    v4 = (int *)0x19644;
    sigemptyset((sigset_t *)0x27220);
    while( true ) {
      sigaction(v3,NULL,(sigaction *)v2);
      if (v2[0] != (void *)0x1)
        sigaddset((sigset_t *)0x27220,v3);
      if (v4 == (int *)0x19670) break;
      v3 = *v4;
      v4 = &v4[1];
    }
    v5 = 0x10000000;
    v3 = 0x14;
    v2[1] = (void *)dat_27220;
    v2[2] = (void *)dat_27228;
    v2[3] = (void *)dat_27230;
    v2[4] = (void *)dat_27238;
    v2[5] = (void *)dat_27240;
    v2[6] = (void *)dat_27248;
    v2[7] = (void *)dat_27250;
    v2[8] = (void *)dat_27258;
    v2[9] = (void *)dat_27260;
    v2[10] = (void *)dat_27268;
    v2[0xb] = (void *)dat_27270;
    v2[0xc] = (void *)dat_27278;
    v2[0xd] = (void *)dat_27280;
    v2[0xe] = (void *)dat_27288;
    v2[0xf] = (void *)dat_27290;
    v2[0x10] = (void *)dat_27298;
    while( true ) {
      if (sigismember((void *)0x27220,v3)) {
        v2[0] = sub_5700;
        if (v3 == 0x14)
          v2[0] = sub_5720;
        sigaction(v3,(sigaction *)v2,NULL);
      }
      if (v1 == (int *)0x19670) break;
      v3 = *v1;
      v1 = &v1[1];
    }
    return;
  }
  v1 = (int *)0x19644;
  v3 = 0x14;
  while( true ) {
    if (sigismember((void *)0x27220,v3))
      signal(v3,0);
    if (v1 == (int *)0x19670) break;
    v3 = *v1;
    v1 = &v1[1];
  }
}


// Function: sub_7ba0 @ 0x7ba0
void sub_7ba0(void)
{
  sub_79d0(1); // tail-call
}


// Function: sub_7bb0 @ 0x7bb0
void sub_7bb0(struct_4 *a0)
{
  if (!dat_27310) {
    dat_27310 = '\x01';
    if (0 <= tcgetpgrp(1)) { // branch-flip
      sub_7ba0();
      sub_7c10();
    }
    else {
      sub_7c10();
    }
  }
  fwrite_unlocked(a0->field_0x8,a0->field_0x0,1,stdout); // tail-call
}


// Function: sub_7c10 @ 0x7c10
void sub_7c10(void)
{
  if (dat_26088) {
    sub_7bb0(0x26080); // tail-call
    return;
  }
  sub_7bb0(0x26060);
  sub_7bb0(0x26090);
  sub_7bb0(0x26070); // tail-call
}


// Function: sub_7c60 @ 0x7c60
void sub_7c60(void)
{
  if (!dat_27312)
    return;
  if (!sub_6420(4))
    return;
  sub_7bb0((struct_4 *)0x26060);
  sub_7bb0((struct_4 *)0x260a0);
  sub_7bb0((struct_4 *)0x26070); // tail-call
}


// Function: sub_7cc0 @ 0x7cc0
void sub_7cc0(void)
{
  sub_7bb0((struct_4 *)0x26060);
  sub_7bb0((struct_4 *)0x26070); // tail-call
}


// Function: sub_7ce0 @ 0x7ce0
void sub_7ce0(void)
{
  sigset_t v1;
  int v2;
  
  while ((dat_27218 || (dat_27214))) {
    if (dat_27310)
      sub_7cc0();
    fflush_unlocked(stdout);
    sigprocmask(0,(sigset_t *)0x27220,&v1);
    v2 = dat_27218;
    if (dat_27214) { // branch-flip
      dat_27214 -= 1;
      v2 = 0x13;
    }
    else {
      signal(dat_27218,0);
    }
    raise(v2);
    sigprocmask(2,&v1,NULL);
  }
}


// Function: sub_7dd0 @ 0x7dd0
void sub_7dd0(void)
{
  sub_79d0(0); // tail-call
}


// Function: sub_7df0 @ 0x7df0
unsigned long sub_7df0(unsigned long a0,unsigned long a1)
{
  struct_19 *v1; // rax
  struct_19 *v2; // rax
  
  v1 = (struct_19 *)sub_169e0(0x10);
  v1->field_0x0 = a1;
  v1->field_0x8 = a0;
  v2 = (struct_19 *)sub_f9a0(dat_273c8,v1);
  if (v2) {
    if (v1 != v2)
      free(v1);
    return CONCAT71((undefined7)((unsigned long)a1 >> 8),v1 != v2) & 0xffffffff;
  }
  sub_16da0(); // no-return
}


// Function: sub_8060 @ 0x8060
void sub_8060(char *a0,void *a1)
{
  unsigned long v1;
  unsigned long v2; // rax
  unsigned long v3;
  unsigned long *v4;
  unsigned long *v5;
  
  v5 = *(unsigned long **)((long)a1 + 0x18);
  v4 = *(unsigned long **)((long)a1 + 0x10);
  if (8 <= (unsigned long)((long)v5 - (long)v4)) {
    if (v5 == v4)
      *(unsigned char *)((long)a1 + 0x50) = *(unsigned char *)((long)a1 + 0x50) | 2;
    v2 = (long)v5 + *(unsigned long *)((long)a1 + 0x30) & ~*(unsigned long *)((long)a1 + 0x30);
    v3 = *(unsigned long *)((long)a1 + 0x20);
    if (v2 - *(long *)((long)a1 + 8) <= *(unsigned long *)((long)a1 + 0x20) - *(long *)((long)a1 + 8))
      v3 = v2;
    v5 = (unsigned long *)(((long)v5 - (long)v4 & 0xfffffffffffffff8U) + (long)v4);
    *(unsigned long *)((long)a1 + 0x18) = v3;
    *(unsigned long *)((long)a1 + 0x10) = v3;
    fputs_unlocked(a0,stdout);
    do {
      v1 = *v4;
      v4 = &v4[1];
      __printf_chk(1," %ld",v1);
    } while (v5 != v4);
    putchar_unlocked(10); // tail-call
    return;
  }
}


// Function: sub_8100 @ 0x8100
void sub_8100(char *a0,unsigned long a1,int a2)
{
  bool v1; // cf
  
  if (a0) {
    a2 -= sub_113a0(a0,0);
    if (a2 <= -1)
      a2 = 0;
    sub_77f0(a0);
    do {
      sub_7460(0x20);
      v1 = a2 != 0;
      a2 -= 1;
    } while (v1);
    return;
  }
  dat_271f8 += __printf_chk(1,"%*lu ",a2,a1);
}


// Function: sub_8180 @ 0x8180
void sub_8180(unsigned long a0,unsigned int a1,bool a2)
{
  char *v1; // rax
  
  v1 = "?";
  if ((a2) && (v1 = NULL, !dat_2732d))
    v1 = (char *)sub_109e0(a0);
  sub_8100(v1,a0 & 0xffffffff,a1); // tail-call
}


// Function: sub_81d0 @ 0x81d0
void sub_81d0(unsigned long a0,unsigned int a1,bool a2)
{
  char *v1; // rax
  
  v1 = "?";
  if ((a2) && (v1 = NULL, !dat_2732d))
    v1 = (char *)sub_10840(a0);
  sub_8100(v1,a0 & 0xffffffff,a1); // tail-call
}


// Function: sub_82b0 @ 0x82b0
unsigned long sub_82b0(void) // early-return
{
  long v1; // rax
  unsigned long v2; // rax
  char v3 [64];
  unsigned long v4; // stack - 0x460
  char v5 [1016];
  
  if (0 <= (int)dat_26014)
    return (unsigned long)dat_26014;
  v4 = 0;
  if (sub_160a0(dat_272a8,&v4,v3)) {
    v1 = sub_8240(v5,0,v3,dat_272a8,0);
    if (v1) {
      v2 = sub_11180(v5,v1,0);
      dat_26014 = (unsigned int)v2;
      goto label_834e;
    }
  }
  v2 = (unsigned long)dat_26014;
label_834e:
  if (0 <= (int)v2)
    return v2;
  dat_26014 = 0;
  return 0;
}


// Function: sub_87f0 @ 0x87f0
long sub_87f0(char *a0,unsigned int *a1,unsigned int a2)
{
  char v1 [8200];
  char *v2; // stack - 0x2038
  unsigned char v3; // stack - 0x2039
  long v4; // stack - 0x2030
  
  v2 = v1;
  sub_8390(&v2,a0,a1,a2,&v4,&v3);
  if ((v2 != v1) && (v2 != a0))
    free(v2);
  return (unsigned long)v3 + v4;
}


// Function: sub_8890 @ 0x8890
void sub_8890(struct_5 *a0)
{
  if (a0->field_0xc8)
    return;
  sub_87f0(a0->field_0x0,dat_272d0,a0->field_0xc4); // tail-call
}


// Function: sub_88c0 @ 0x88c0
void sub_88c0(struct_5 *a0,struct_5 *a1,void *a2)
{
  int v1; // eax
  
  v1 = sub_8890(a0);
  if (v1 == sub_8890(a1)) {
    (*a2)(a0->field_0x0,a1->field_0x0); // jump-as-call
    return;
  }
}


// Function: sub_8920 @ 0x8920
void sub_8920(struct_5 *a0,struct_5 *a1)
{
  sub_88c0(a1,a0,dat_25fc0); // tail-call
}


// Function: sub_8940 @ 0x8940
void sub_8940(struct_5 *a0,struct_5 *a1)
{
  sub_88c0(a0,a1,dat_25fc0); // tail-call
}


// Function: sub_8950 @ 0x8950
void sub_8950(struct_5 *a0,struct_5 *a1)
{
  sub_88c0(a1,a0,sub_72a0); // tail-call
}


// Function: sub_8970 @ 0x8970
void sub_8970(struct_5 *a0,struct_5 *a1)
{
  sub_88c0(a0,a1,sub_72a0); // tail-call
}


// Function: sub_8980 @ 0x8980
void sub_8980(void)
{
  struct_5 *v1;
  unsigned long v2; // rbx
  
  if ((dat_27330 != 2) && ((!dat_272b0 || (2 <= (unsigned int)(dat_27338 - 2U)))))
    return;
  v2 = 0;
  if (dat_273b0) {
    do {
      v1 = *(struct_5 **)(dat_273a0 + v2 * 8);
      v2 += 1;
      v1->field_0xc8 = sub_8890(v1);
    } while (v2 < dat_273b0);
  }
}


// Function: sub_89f0 @ 0x89f0
void sub_89f0(void) // return-dupe
{
  int v1; // eax
  int v2;
  unsigned long v3;
  
  v3 = dat_273b0;
  if (dat_27398 < (dat_273b0 >> 1) + dat_273b0) {
    free(dat_273a0);
    dat_273a0 = (void *)sub_16b00(v3,0x18);
    dat_27398 = dat_273b0 * 3;
  }
  sub_6000();
  sub_8980();
  if (dat_27330 == 6)
    return;
  v1 = _setjmp((void *)0x26300);
  v3 = (unsigned long)dat_27330;
  v2 = 0;
  if (v1) {
    if (dat_27330 == 4)
      __assert_fail("sort_type != sort_version","src/ls.c",0x1008,"sort_files"); // no-return
    sub_6000();
    v2 = 1;
  }
  if ((int)v3 == 5)
    v3 = (unsigned long)(dat_27334 + 5);
  sub_11610(dat_273a0,dat_273b0,*(void **)(((unsigned long)dat_272f4 + ((unsigned long)dat_2732f + ((long)v2 + v3 * 2) * 2) * 2) * 8 + 0x24f60));
}


// Function: sub_8af0 @ 0x8af0
long sub_8af0(void *a0)
{
  char v1;
  unsigned int v2;
  unsigned int v3;
  unsigned long v4; // rax
  char *v5; // rax
  long v6;
  char v7 [664];
  
  if (dat_272fc) { // branch-flip
    if (dat_27338 == 4) {
      v5 = (char *)sub_10c20(*(unsigned long *)((long)a0 + 0x20),v7);
      v6 = strlen(v5) + 1;
      goto label_8bba;
    }
    v6 = (long)dat_27360 + 1;
    if (dat_2732c) {
label_8b41:
      v4 = (unsigned long)dat_2735c;
      goto label_8b4c;
    }
    if (!dat_27365) goto label_8b5c;
label_8bf9:
    v4 = (unsigned long)dat_27354;
  }
  else {
    v6 = 0;
label_8bba:
    if (dat_2732c) {
      if (dat_27338 != 4) goto label_8b41;
      v5 = "?";
      if (*(char *)((long)a0 + 0xb8))
        v5 = (char *)sub_ffa0(*(unsigned long *)((long)a0 + 0x58),v7,dat_27328,0x200,dat_27320);
      v4 = strlen(v5);
label_8b4c:
      v6 += v4 + 1;
    }
    if (!dat_27365) goto label_8b5c;
    if (dat_27338 != 4) goto label_8bf9;
    v4 = strlen(*(char **)((long)a0 + 0xb0));
  }
  v6 += v4 + 1;
label_8b5c:
  v6 += sub_8890(a0);
  if (dat_27314) {
    v2 = *(unsigned int *)((long)a0 + 0xa8);
    v3 = *(unsigned int *)((long)a0 + 0x30);
    v1 = *(char *)((long)a0 + 0xb8);
    v6 = (v6 + 1) - (unsigned long)(sub_6040((unsigned char)v1,v3,v2) == '\0');
  }
  return v6;
}


// Function: sub_8c90 @ 0x8c90
unsigned long sub_8c90(bool a0)
{
  unsigned long *v1;
  long v2; // rax
  unsigned long v3;
  unsigned long v4; // rcx
  unsigned long v5;
  unsigned long v6;
  char *v7;
  unsigned long v8;
  
  v6 = dat_27200;
  if (dat_273b0 <= dat_27200)
    v6 = dat_273b0;
  v8 = dat_273b0;
  if (dat_27200)
    v8 = v6;
  v6 = 0;
  sub_61d0(v8);
  if (dat_273b0) {
    do {
      v2 = sub_8af0(*(void **)(dat_273a0 + v6 * 8));
      if (v8) {
        v3 = 0;
        v7 = dat_27208;
        do {
          v4 = v3 + 1;
          if (*v7) {
            if (a0) // branch-flip
              v5 = v6 / (((dat_273b0 - 1) + v4) / v4);
            else {
              v5 = v6 % v4;
            }
            v1 = (unsigned long *)(*(long *)&v7[0x10] + v5 * 8);
            v3 = v2 + (unsigned long)(v5 != v3) * 2;
            v5 = *v1;
            if (v5 < v3) {
              *(unsigned long *)&v7[8] = *(long *)&v7[8] + (v3 - v5);
              *v1 = v3;
              *v7 = *(unsigned long *)&v7[8] < dat_272b0;
            }
          }
          v7 = &v7[0x18];
          v3 = v4;
        } while (v8 != v4);
      }
      v6 += 1;
    } while (v6 < dat_273b0);
  }
  if (2 <= v8) {
    v7 = &dat_27208[v8 * 0x18 + -0x18];
    v6 = v8;
    do {
      if (*v7)
        return v6;
      v6 -= 1;
      v7 = &v7[-0x18];
      v8 = 1;
    } while (v6 != 1);
  }
  return v8;
}


// Function: sub_8eb0 @ 0x8eb0
long sub_8eb0(char *a0,unsigned int *a1,unsigned int a2,struct_4 *a3,bool a4,obstack *a5,char *a6)
{
  char *v1;
  char v10;
  char v11;
  long v12; // stack - 0x2078
  unsigned long v2; // rax
  char *v3; // rax
  char *v4; // rax
  char *v5; // rcx
  char v6 [8200];
  char *v7; // stack - 0x2050
  unsigned char v8; // stack - 0x2051
  unsigned long v9;
  
  v1 = a6;
  v7 = v6;
  v2 = sub_8390(&v7,a0,a1,a2,NULL,&v8);
  if ((v8) && (a4))
    sub_7460(0x20);
  if (a3)
    sub_8e50(a3);
  v11 = dat_273a9;
  v9 = v2;
  if (v1) { // branch-flip
    v10 = '\0';
    if ((dat_273a8) && (v10 = dat_273a9, dat_273a9)) {
      if (v8) { // branch-flip
        v12 = 0;
        v11 = 0;
      }
      else {
        putchar_unlocked((int)*v7);
        v12 = 1;
        v9 = v2 - 2;
      }
    }
    else {
      v12 = 0;
      v11 = v10;
    }
    v3 = (char *)sub_7830(dat_27388,0);
    v4 = (char *)sub_7830(v1,1);
    v5 = "";
    if (*v4 != '/')
      v5 = "/";
    __printf_chk(1,"\x1b]8;;file://%s%s%s\a",v3,v5,v4);
    free(v3);
    free(v4);
  }
  else {
    v12 = 0;
    v11 = 0;
  }
  if (a5) { // branch-flip
    sub_7900(a5);
    fwrite_unlocked(&v7[v12],1,v9,stdout);
    dat_271f8 += v2;
    sub_7900(a5);
  }
  else {
    fwrite_unlocked(&v7[v12],1,v9,stdout);
    dat_271f8 += v2;
  }
  if ((v1) && (fputs_unlocked("\x1b]8;;\a",stdout), v11))
    putchar_unlocked((int)v7[v2 - 1]);
  if ((v7 != v6) && (v7 != a0))
    free(v7);
  return v8 + v2;
}


// Function: sub_9160 @ 0x9160
long sub_9160(struct_13 *a0,unsigned char a1,obstack *a2,unsigned long a3)
{
  unsigned long v1;
  long v2; // rax
  struct_4 *v3; // rax
  char *v4;
  
  v4 = a0->field_0x8;
  if (!a1)
    v4 = a0->field_0x0;
  if ((dat_27312) && ((v3 = (struct_4 *)sub_7480(a0,a1), v3 || (sub_6420(4))))) {
    v2 = sub_8eb0(v4,dat_272d0,a0->field_0xc4,v3,a1 ^ 1,a2,a0->field_0x10);
    sub_7ce0();
    sub_7c10();
    if (dat_272b0) {
      v1 = v2 + -1 + a3;
      if (a3 / dat_272b0 != v1 / dat_272b0)
        sub_7bb0((struct_4 *)0x261d0,a3 / dat_272b0,v1 % dat_272b0);
    }
  }
  else {
    v2 = sub_8eb0(v4,dat_272d0,a0->field_0xc4,NULL,a1 ^ 1,a2,a0->field_0x10);
    sub_7ce0();
  }
  return v2;
}


// Function: sub_92b0 @ 0x92b0
void sub_92b0(void *a0) // return-dupe x4
{
  char v1;
  unsigned long v10; // rdx
  unsigned long v11; // rdx
  unsigned long v12; // rdx
  unsigned long v13; // rdx
  char *v14; // rbx
  char *v15;
  char *v16;
  long v17; // rbx
  char v18; // stack - 0x1144
  char v19 [3656];
  unsigned long v2;
  char v20 [656];
  char v21 [32];
  char v22 [68];
  unsigned long v23; // stack - 0x1198
  int v24; // ebp
  int v25;
  unsigned long v26;
  unsigned long v27;
  char *v28; // rax
  bool v29;
  int v3;
  char v30 [16];
  unsigned long v31; // stack - 0x1190
  unsigned long v32; // stack - 0x1143
  unsigned short v33; // stack - 0x113b
  char v34; // stack - 0x1139
  unsigned int v4; // eax
  char *v5;
  long v6; // rax
  unsigned long v7; // rdx
  unsigned long v8; // rdx
  unsigned long v9;
  
  if (*(char *)((long)a0 + 0xb8)) // branch-flip
    sub_e2f0((struct_14 *)((long)a0 + 0x18),&v18);
  else {
    v34 = 0;
    v33 = 0x3f3f;
    v18 = "?pcdb-lswd"[*(unsigned int *)((long)a0 + 0xa8)];
    v32 = 0x3f3f3f3f3f3f3f3f;
  }
  if (dat_27364) { // branch-flip
    if (*(int *)((long)a0 + 0xbc) == 1) {
      v33 = CONCAT11(0x2e,(char)v33);
      goto label_9310;
    }
    if (*(int *)((long)a0 + 0xbc) != 2) goto label_9310;
    v33 = CONCAT11(0x2b,(char)v33);
    if (dat_27334 == 2) goto label_9600;
label_931f:
    if (3 <= dat_27334) { // branch-flip
      if (dat_27334 != 3)
        abort(); // no-return
      v30 = sub_5610((void *)((long)a0 + 0x18));
      v31 = SUB168(v30,8);
      v23 = SUB168(v30,0);
      v29 = (v23 & v31) != 0xffffffffffffffff;
    }
    else if (dat_27334) { // branch-flip
      v29 = 1;
      v30 = sub_15ca0((void *)((long)a0 + 0x18));
      v31 = SUB168(v30,8);
      v23 = SUB168(v30,0);
    }
    else {
      v29 = 1;
      v30 = sub_15cb0((void *)((long)a0 + 0x18));
      v31 = SUB168(v30,8);
      v23 = SUB168(v30,0);
    }
  }
  else {
    v33 &= 0xff;
label_9310:
    if (dat_27334 != 2) goto label_931f;
label_9600:
    v29 = 1;
    v30 = sub_15c90((void *)((long)a0 + 0x18));
    v31 = SUB168(v30,8);
    v23 = SUB168(v30,0);
  }
  v15 = v19;
  if (dat_272fc) {
    v15 = (char *)sub_6f10(v20,0x15,a0);
    v15 = &v19[__sprintf_chk(v19,1,0xe3b,"%*s ",dat_27360,v15)];
  }
  if (dat_2732c) {
    v28 = "?";
    if (*(char *)((long)a0 + 0xb8))
      v28 = (char *)sub_ffa0(*(unsigned long *)((long)a0 + 0x58),v20,dat_27328,0x200,dat_27320);
    v3 = dat_2735c;
    v3 -= sub_113a0(v28,0);
    if (v3 <= 0) goto label_9818;
    v5 = &v15[v3];
    do {
      v14 = &v15[1];
      *v15 = ' ';
      v15 = v14;
      v16 = v28;
    } while (v5 != v14);
    while( true ) {
      v28 = &v16[1];
      v15 = &v5[1];
      v1 = *v16;
      *v5 = v1;
      if (!v1) break;
label_9818:
      v5 = v15;
      v16 = v28;
    }
    *v5 = ' ';
  }
  v28 = "?";
  if (*(char *)((long)a0 + 0xb8))
    v28 = (char *)sub_10c20(*(unsigned long *)((long)a0 + 0x28),v20);
  v15 = &v15[__sprintf_chk(v15,1,0xffffffffffffffff,"%s %*s ",&v18,dat_27358,v28)];
  sub_7810();
  if ((((dat_26029) || (dat_26028)) || (dat_2732e)) || (v9 = v7, dat_27365)) {
    sub_77d0(v19,(long)v15 - (long)v19);
    v9 = v8;
    if (dat_26029) {
      sub_81d0(*(unsigned int *)((long)a0 + 0x34),dat_27350,(unsigned char)*(char *)((long)a0 + 0xb8));
      v9 = v11;
    }
    if (dat_26028) {
      sub_8180(*(unsigned int *)((long)a0 + 0x38),dat_2734c,(unsigned char)*(char *)((long)a0 + 0xb8));
      v9 = v12;
    }
    if (dat_2732e) {
      sub_81d0(*(unsigned int *)((long)a0 + 0x34),dat_27348,(unsigned char)*(char *)((long)a0 + 0xb8));
      v9 = v13;
    }
    v15 = v19;
    if (dat_27365) {
      sub_8100(*(char **)((long)a0 + 0xb0),0,dat_27354);
      v9 = v10;
    }
  }
  if (*(char *)((long)a0 + 0xb8)) { // branch-flip
    if ((*(unsigned int *)((long)a0 + 0x30) & 0xb000) != 0x2000) {
      v2 = *(unsigned long *)((long)a0 + 0x48);
      v28 = (char *)sub_ffa0(sub_59e0(v2,v20,v9,1));
      goto label_9493;
    }
    v9 = *(unsigned long *)((long)a0 + 0x40);
    v24 = dat_2733c - (dat_27344 + 2 + dat_27340);
    v28 = (char *)sub_10c20(gnu_dev_minor(v9),v20);
    v3 = dat_27340;
    v9 = *(unsigned long *)((long)a0 + 0x40);
    v5 = (char *)sub_10c20(gnu_dev_major(v9),v21);
    v25 = 0;
    if (0 <= v24)
      v25 = v24;
    v25 += dat_27344;
    v5 = &v15[__sprintf_chk(v15,1,0xffffffffffffffff,"%*s, %*s ",v25,v5,v3,v28)];
  }
  else {
    v28 = "?";
label_9493:
    v3 = dat_2733c;
    v3 -= sub_113a0(v28,0);
    if (1 <= v3) { // branch-flip
      v16 = v15;
      do {
        v14 = &v16[1];
        *v16 = ' ';
        v16 = v14;
        v5 = &v15[v3];
      } while (&v15[v3] != v14);
    }
    else {
      v5 = v15;
    }
    do {
      v15 = v5;
      v1 = *v28;
      v28 = &v28[1];
      v5 = &v15[1];
      *v15 = v1;
    } while (v1);
    *v15 = ' ';
  }
  *v5 = '\x01';
  if ((!*(char *)((long)a0 + 0xb8)) || (!v29)) {
label_94f4:
    v15 = "?";
label_94fb:
    v3 = sub_82b0();
    v5 = &v5[__sprintf_chk(v5,1,0xffffffffffffffff,"%*s ",v3,v15)];
  }
  else {
    v6 = sub_160a0(dat_272a8,&v23,v22);
    v27 = v31;
    v26 = v23;
    if (v6) { // branch-flip
      v3 = sub_162a0(dat_27370,dat_27378,v23,v31);
      if (v3 < 0) {
        sub_e960((timespec *)0x27370);
        v26 = v23;
        v27 = v31;
      }
      v3 = sub_162a0(dat_27370 + -0xf0c2ac,dat_27378,v26,v27);
      v4 = 0;
      if (v3 <= -1) {
        v4 = sub_162a0(v23,v31,dat_27370,dat_27378);
        v4 >>= 0x1f;
      }
      v6 = sub_8240(v5,v4,v22,dat_272a8,v31 & 0xffffffff);
      if (!v6) goto label_9a20;
      v5 = &v5[v6];
    }
    else {
label_9a20:
      if (*v5) {
        if (!*(char *)((long)a0 + 0xb8)) goto label_94f4;
        v15 = (char *)sub_7de0(v23,v20);
        goto label_94fb;
      }
    }
    *v5 = ' ';
    v5 = &v5[1];
  }
  v17 = (long)v5 - (long)v19;
  sub_77d0(v19,v17);
  v6 = sub_9160(a0,0,(obstack *)0x271a0,v17);
  if (*(int *)((long)a0 + 0xa8) != 6) {
    if (!dat_27314)
      return;
    sub_9280((unsigned char)*(char *)((long)a0 + 0xb8),*(unsigned int *)((long)a0 + 0x30));
    return;
  }
  if (!*(long *)((long)a0 + 8))
    return;
  sub_77f0(" -> ");
  sub_9160(a0,1,NULL,v17 + 4 + v6);
  if (!dat_27314)
    return;
  sub_9280(1,*(unsigned int *)((long)a0 + 0xac),0);
}


// Function: sub_9c00 @ 0x9c00
void sub_9c00(char a0)
{
  unsigned long v1;
  void *v2;
  unsigned long v3;
  unsigned long v4; // rbx
  unsigned long v5;
  int v6; // r15d
  
  if (!dat_273b0) {
    putchar_unlocked((int)dat_26019); // tail-call
    return;
  }
  v4 = 0;
  v5 = 0;
  do {
    v2 = *(void **)(dat_273a0 + v4 * 8);
    if (dat_272b0) { // branch-flip
      v3 = sub_8af0(v2);
      if (v4) { // branch-flip
        v1 = v3 + v5 + 2;
        if ((!dat_272b0) || ((v1 < dat_272b0 && (v5 <= -v3 - 3)))) {
          v3 = v1;
          goto label_9cb0;
        }
        v6 = (int)dat_26019;
        v5 = 0;
label_9cb6:
        putchar_unlocked((int)a0);
        putchar_unlocked(v6);
      }
      else {
        v3 = v5 + v3;
      }
    }
    else {
      v3 = v5;
      if (v4) {
        v3 = v5 + 2;
label_9cb0:
        v5 += 2;
        v6 = 0x20;
        goto label_9cb6;
      }
    }
    v4 += 1;
    sub_9a80(v2,v5);
    v5 = v3;
    if (dat_273b0 <= v4) {
      putchar_unlocked((int)dat_26019); // tail-call
      return;
    }
  } while( true );
}


// Function: sub_9d30 @ 0x9d30
void sub_9d30(void)
{
  long v1;
  void *v2;
  unsigned long v3; // rax
  long v4; // rax
  unsigned long v5; // rdx
  unsigned long v6; // rbx
  long v7;
  unsigned long v8; // rbp
  unsigned long v9;
  
  v3 = sub_8c90(0);
  v1 = dat_27208 + -0x18 + v3 * 0x18;
  v2 = (void *)*dat_273a0;
  v4 = sub_8af0(v2);
  v7 = **(long **)(v1 + 0x10);
  sub_9a80(v2,0);
  if (2 <= dat_273b0) {
    v6 = 1;
    v9 = 0;
    do {
      v5 = v6 % v3;
      if (v5) { // branch-flip
        v8 = v7 + v9;
        sub_73d0(v9 + v4,v8);
      }
      else {
        v8 = 0;
        putchar_unlocked((int)dat_26019);
      }
      v2 = (void *)dat_273a0[v6];
      v6 += 1;
      sub_9a80(v2,v8);
      v4 = sub_8af0(v2);
      v7 = *(long *)(*(long *)(v1 + 0x10) + v5 * 8);
      v9 = v8;
    } while (v6 < dat_273b0);
  }
  putchar_unlocked((int)dat_26019); // tail-call
}


// Function: sub_9e30 @ 0x9e30
void sub_9e30(void) // return-dupe
{
  long v1;
  unsigned long v10; // stack - 0x40
  void *v2;
  long v3;
  unsigned long v4; // rax
  long v5; // rax
  unsigned long v6; // rbx
  unsigned long v7;
  long v8; // r13
  unsigned long v9;
  
  v4 = sub_8c90(1);
  v10 = 0;
  v1 = dat_27208 + v4 * 0x18;
  v4 = (dat_273b0 / v4 + 1) - (unsigned long)(dat_273b0 % v4 == 0);
  if (!v4)
    return;
  do {
    v8 = 0;
    v7 = v10;
    v9 = 0;
    while( true ) {
      v2 = *(void **)(dat_273a0 + v7 * 8);
      v5 = sub_8af0(v2);
      v3 = *(long *)(*(long *)(v1 + -8) + v8);
      v8 += 8;
      sub_9a80(v2,v9);
      v7 += v4;
      if (dat_273b0 <= v7) break;
      v6 = v3 + v9;
      sub_73d0(v9 + v5,v6);
      v9 = v6;
    }
    putchar_unlocked((int)dat_26019);
    v10 += 1;
  } while (v4 != v10);
}


// Function: sub_9f20 @ 0x9f20
void sub_9f20(void) // return-dupe x2
{
  long v1;
  unsigned long v2; // rbx
  
  switch(dat_27338) {
    case 0:
      v2 = 0;
      if (!dat_273b0)
        return;
      do {
        sub_7c60();
        v1 = v2 * 8;
        v2 += 1;
        sub_92b0(*(void **)(dat_273a0 + v1));
        sub_7460((int)dat_26019);
      } while (v2 < dat_273b0);
      return;
    case 1:
      v2 = 0;
      if (!dat_273b0)
        return;
      do {
        v1 = v2 * 8;
        v2 += 1;
        sub_9a80(*(void **)(dat_273a0 + v1),0);
        putchar_unlocked((int)dat_26019);
      } while (v2 < dat_273b0);
      return;
    case 2:
      if (dat_272b0) {
        sub_9e30(); // tail-call
        return;
      }
      break;
    case 3:
      if (dat_272b0) {
        sub_9d30(); // tail-call
        return;
      }
      break;
    case 4:
      sub_9c00(0x2c); // tail-call
      return;
    default:
      return;
    
  }
  sub_9c00(0x20); // tail-call
}


// Function: sub_a050 @ 0xa050
void sub_a050(unsigned long *a0,unsigned long *a1)
{
  sub_a010((char *)*a0,(char *)*a1); // tail-call
}


// Function: sub_a060 @ 0xa060
void sub_a060(unsigned long *a0,unsigned long *a1)
{
  sub_a010((char *)*a1,(char *)*a0); // tail-call
}


// Function: sub_a130 @ 0xa130
void sub_a130(struct_29 *a0,struct_29 *a1)
{
  sub_a100(a1->field_0x0,a1->field_0x48,a0->field_0x0,a0->field_0x48,dat_25fc0); // tail-call
}


// Function: sub_a150 @ 0xa150
void sub_a150(struct_29 *a0,struct_29 *a1)
{
  sub_a100(a1->field_0x0,a1->field_0x48,a0->field_0x0,a0->field_0x48,sub_72a0); // tail-call
}


// Function: sub_a170 @ 0xa170
void sub_a170(struct_29 *a0,struct_29 *a1)
{
  sub_a100(a0->field_0x0,a0->field_0x48,a1->field_0x0,a1->field_0x48,dat_25fc0); // tail-call
}


// Function: sub_a1a0 @ 0xa1a0
void sub_a1a0(struct_29 *a0,struct_29 *a1)
{
  sub_a100(a0->field_0x0,a0->field_0x48,a1->field_0x0,a1->field_0x48,sub_72a0); // tail-call
}


// Function: sub_a1e0 @ 0xa1e0
void sub_a1e0(char *a0,char a1) // return-dupe
{
  unsigned int v1;
  int v2;
  unsigned long *v3;
  char *v4;
  long *v5;
  long v6;
  
  if ((a0) && (dat_273c8))
    sub_6840(NULL,a0,0);
  v6 = dat_273b0;
  if (!dat_273b0) {
    dat_273b0 = 0;
    return;
  }
  do {
    v6 -= 1;
    v3 = (unsigned long *)dat_273a0[v6];
    v1 = *(unsigned int *)&v3[0x15];
    if (sub_a1d0(v1)) {
      v4 = (char *)*v3;
      if (a0) { // branch-flip
        if (sub_73b0(v4)) goto label_a26c;
        if (*v4 == '/') goto label_a291;
        v4 = (char *)sub_e300(a0,v4,NULL);
        sub_6840(v4,(char *)v3[1],(unsigned char)a1);
        free(v4);
        v2 = *(int *)&v3[0x15];
      }
      else {
label_a291:
        sub_6840(v4,(char *)v3[1],(unsigned char)a1);
        v2 = *(int *)&v3[0x15];
      }
      if (v2 == 9)
        sub_6dc0(v3);
    }
label_a26c:
    if (!v6) {
      if (dat_273b0) {
        v6 = 0;
        v5 = dat_273a0;
        do {
          v2 = *(int *)(*v5 + 0xa8);
          dat_273a0[v6] = *v5;
          v5 = &v5[1];
          v6 += (unsigned long)(v2 != 9);
        } while (&dat_273a0[dat_273b0] != v5);
        dat_273b0 = v6;
        return;
      }
      dat_273b0 = 0;
      return;
    }
  } while( true );
}


// Function: sub_a400 @ 0xa400
int sub_a400(int a0,char *a1,struct_16 *a2,unsigned int a3,unsigned int a4)
{
  int v1; // eax
  long v2;
  char v3 [16];
  unsigned char v4; // stack - 0x137
  unsigned long v5; // stack - 0xe8
  unsigned int v6; // stack - 0xe0
  
  v1 = statx(a0,a1,a3 | 0x800,a4,&v2);
  if ((0 <= v1) && (sub_a340(&v2,a2), a4 & 0x800)) {
    if (v4 & 8) { // branch-flip
      v3 = sub_a330(v5,v6);
      a2->field_0x58 = SUB168(v3,0);
      a2->field_0x60 = SUB168(v3,8);
    }
    else {
      a2->field_0x60 = 0xffffffffffffffff;
      a2->field_0x58 = 0xffffffffffffffff;
    }
  }
  return v1;
}


// Function: sub_a4c0 @ 0xa4c0
void sub_a4c0(char *a0,struct_16 *a1)
{
  sub_a400(0xffffff9c,a0,a1,0,sub_68d0()); // tail-call
}


// Function: sub_a4f0 @ 0xa4f0
void sub_a4f0(char *a0,struct_16 *a1)
{
  sub_a400(0xffffff9c,a0,a1,0x100,sub_68d0()); // tail-call
}


// Function: sub_a530 @ 0xa530
int sub_a530(char *a0,struct_16 *a1)
{
  return sub_a400(0xffffff9c,a0,a1,0,2); // tail-call
}


// Function: sub_b140 @ 0xb140
int sub_b140(unsigned long a0,struct_16 *a1)
{
  return sub_a400(a0,"",a1,0x1000,0x100); // tail-call
}


// Function: sub_b160 @ 0xb160
int sub_b160(char *a0,struct_16 *a1)
{
  return sub_a400(0xffffff9c,a0,a1,0,0x100); // tail-call
}


// Function: sub_b180 @ 0xb180
void sub_b180(char *a0,char *a1,char a2)
{
  unsigned long v1;
  unsigned long v10; // stack - 0x368
  char v11 [663];
  unsigned int v12;
  unsigned long v13; // stack - 0x378
  unsigned long v14; // stack - 0x360
  unsigned long v2;
  unsigned char v3; // al
  int v4; // eax
  int *v5; // rax
  DIR *v6; // rax
  dirent *v7; // rax
  char *v8; // rax
  unsigned long v9; // rax
  
  v5 = __errno_location();
  *v5 = 0;
  v6 = opendir(a0);
  if (!v6) {
    sub_6f50((unsigned char)a2,dcgettext(NULL,"cannot open directory %s",5),a0); // tail-call
    return;
  }
  if (dat_273c8) {
    v4 = dirfd(v6);
    if (0 <= v4) { // branch-flip
      v4 = sub_b140(v4);
      v1 = v10;
      v2 = v14;
    }
    else {
      v4 = sub_b160(a0,&v10);
      v1 = v10;
      v2 = v14;
    }
    if (v4 <= -1) {
      sub_6f50((unsigned char)a2,dcgettext(NULL,"cannot determine device and inode of %s",5),a0);
      closedir(v6); // tail-call
      return;
    }
    if (sub_7df0(v1)) {
      v8 = (char *)sub_15700(0,3,a0);
      error(0,0,dcgettext(NULL,"%s: not listing already-listed directory",5),v8);
      closedir(v6);
      sub_59b0(1); // tail-call
      return;
    }
    sub_7970(v1,v2);
  }
  sub_6e20();
  if ((dat_272f6) || (dat_272b8)) {
    if (!dat_26010)
      sub_7460(10);
    dat_26010 = '\0';
    v8 = NULL;
    sub_7810();
    if ((dat_27311) && (v8 = (char *)sub_de90(a0,2), !v8))
      sub_6f50((unsigned char)a2,dcgettext(NULL,"error canonicalizing %s",5),a0);
    if (!a1)
      a1 = a0;
    sub_8eb0(a1,dat_272c8,0xffffffff,NULL,1,(obstack *)0x27140,v8);
    free(v8);
    sub_77f0(":\n");
  }
  v13 = 0;
  do {
    *v5 = 0;
    v7 = readdir(v6);
    if (v7) { // branch-flip
      if (!sub_69f0(&v7->field_0x13)) {
        v12 = 0;
        v3 = v7->field_0x12 - 1;
        if (v3 <= 0xd)
          v12 = *(unsigned int *)((unsigned long)v3 * 4 + 0x195e0);
        v13 += sub_a550(&v7->field_0x13,v12,0,a0);
        if ((((dat_27338 == 1) && (dat_27330 == 6)) && (!dat_2732c)) && (!dat_272f6)) {
          sub_89f0();
          sub_9f20();
          sub_6e20();
        }
      }
    }
    else {
      if (!*v5) {
label_b357:
        if (closedir(v6))
          sub_6f50((unsigned char)a2,dcgettext(NULL,"closing directory %s",5),a0);
        sub_89f0();
        if (dat_272f6)
          sub_a1e0(a0,0);
        if ((!dat_27338) || (dat_2732c)) {
          v8 = (char *)sub_ffa0(v13,v11,dat_27328,0x200,dat_27320);
          v9 = strlen(v8);
          v8[-1] = ' ';
          v8[v9] = dat_26019;
          sub_7810();
          sub_77f0(dcgettext(NULL,"total",5));
          sub_77d0(&v8[-1],&(&v8[v9])[1 - (long)&v8[-1]]);
        }
        if (dat_273b0) {
          sub_9f20(); // tail-call
          return;
        }
        return;
      }
      sub_6f50((unsigned char)a2,dcgettext(NULL,"reading directory %s",5),a0);
      if (*v5 != 0x4b) goto label_b357;
    }
    sub_7ce0();
  } while( true );
}


// Function: sub_b690 @ 0xb690
void sub_b690(int a0)
{
  FILE *v1;
  char *v2;
  
  v2 = dat_27400;
  if (a0) // branch-flip
    __fprintf_chk(stderr,1,dcgettext(NULL,"Try \'%s --help\' for more information.\n",5),v2);
  else {
    __printf_chk(1,dcgettext(NULL,"Usage: %s [OPTION]... [FILE]...\n",5),v2);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"List information about the FILEs (the current directory by default).\nSort entries alphabetically if none of -cftuvSUX nor --sort is specified.\n",5),v1);
    sub_6530();
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -a, --all                  do not ignore entries starting with .\n  -A, --almost-all           do not list implied . and ..\n      --author               with -l, print the author of each file\n  -b, --escape               print C-style escapes for nongraphic characters\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --block-size=SIZE      with -l, scale sizes by SIZE when printing them;\n                             e.g., \'--block-size=M\'; see SIZE format below\n\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -B, --ignore-backups       do not list implied entries ending with ~\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -c                         with -lt: sort by, and show, ctime (time of last\n                             modification of file status information);\n                             with -l: show ctime and sort by name;\n                             otherwise: sort by ctime, newest first\n\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -C                         list entries by columns\n      --color[=WHEN]         color the output WHEN; more info below\n  -d, --directory            list directories themselves, not their contents\n  -D, --dired                generate output designed for Emacs\' dired mode\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -f                         list all entries in directory order\n  -F, --classify[=WHEN]      append indicator (one of */=>@|) to entries WHEN\n      --file-type            likewise, except do not append \'*\'\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --format=WORD          across -x, commas -m, horizontal -x, long -l,\n                             single-column -1, verbose -l, vertical -C\n\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --full-time            like -l --time-style=full-iso\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -g                         like -l, but do not list owner\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --group-directories-first\n                             group directories before files;\n                             can be augmented with a --sort option, but any\n                             use of --sort=none (-U) disables grouping\n\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -G, --no-group             in a long listing, don\'t print group names\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -h, --human-readable       with -l and -s, print sizes like 1K 234M 2G etc.\n      --si                   likewise, but use powers of 1000 not 1024\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -H, --dereference-command-line\n                             follow symbolic links listed on the command line\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --dereference-command-line-symlink-to-dir\n                             follow each command line symbolic link\n                             that points to a directory\n\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --hide=PATTERN         do not list implied entries matching shell PATTERN\n                             (overridden by -a or -A)\n\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --hyperlink[=WHEN]     hyperlink file names WHEN\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --indicator-style=WORD\n                             append indicator with style WORD to entry names:\n                             none (default), slash (-p),\n                             file-type (--file-type), classify (-F)\n\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -i, --inode                print the index number of each file\n  -I, --ignore=PATTERN       do not list implied entries matching shell PATTERN\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -k, --kibibytes            default to 1024-byte blocks for file system usage;\n                             used only with -s and per directory totals\n\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -l                         use a long listing format\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -L, --dereference          when showing file information for a symbolic\n                             link, show information for the file the link\n                             references rather than for the link itself\n\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -m                         fill width with a comma separated list of entries\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -n, --numeric-uid-gid      like -l, but list numeric user and group IDs\n  -N, --literal              print entry names without quoting\n  -o                         like -l, but do not list group information\n  -p, --indicator-style=slash\n                             append / indicator to directories\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -q, --hide-control-chars   print ? instead of nongraphic characters\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --show-control-chars   show nongraphic characters as-is (the default,\n                             unless program is \'ls\' and output is a terminal)\n\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -Q, --quote-name           enclose entry names in double quotes\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --quoting-style=WORD   use quoting style WORD for entry names:\n                             literal, locale, shell, shell-always,\n                             shell-escape, shell-escape-always, c, escape\n                             (overrides QUOTING_STYLE environment variable)\n\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -r, --reverse              reverse order while sorting\n  -R, --recursive            list subdirectories recursively\n  -s, --size                 print the allocated size of each file, in blocks\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -S                         sort by file size, largest first\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --sort=WORD            sort by WORD instead of name: none (-U), size (-S),\n                             time (-t), version (-v), extension (-X), width\n\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --time=WORD            change the default of using modification times;\n                               access time (-u): atime, access, use;\n                               change time (-c): ctime, status;\n                               birth time: birth, creation;\n                             with -l, WORD determines which time to show;\n                             with --sort=time, sort by WORD (newest first)\n\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --time-style=TIME_STYLE\n                             time/date format with -l; see TIME_STYLE below\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -t                         sort by time, newest first; see --time\n  -T, --tabsize=COLS         assume tab stops at each COLS instead of 8\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -u                         with -lt: sort by, and show, access time;\n                             with -l: show access time and sort by name;\n                             otherwise: sort by access time, newest first\n\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -U                         do not sort; list entries in directory order\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -v                         natural sort of (version) numbers within text\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"  -w, --width=COLS           set output width to COLS.  0 means no limit\n  -x                         list entries by lines instead of by columns\n  -X                         sort alphabetically by entry extension\n  -Z, --context              print any security context of each file\n      --zero                 end each output line with NUL, not newline\n  -1                         list one file per line\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --help        display this help and exit\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"      --version     output version information and exit\n",5),v1);
    sub_6560();
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nThe TIME_STYLE argument can be full-iso, long-iso, iso, locale, or +FORMAT.\nFORMAT is interpreted like in date(1).  If FORMAT is FORMAT1<newline>FORMAT2,\nthen FORMAT1 applies to non-recent files and FORMAT2 to recent files.\nTIME_STYLE prefixed with \'posix-\' takes effect only outside the POSIX locale.\nAlso the TIME_STYLE environment variable sets the default style to use.\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nThe WHEN argument defaults to \'always\' and can also be \'auto\' or \'never\'.\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nUsing color to distinguish file types is disabled both by default and\nwith --color=never.  With --color=auto, ls emits color codes only when\nstandard output is connected to a terminal.  The LS_COLORS environment\nvariable can change the settings.  Use the dircolors(1) command to set it.\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nExit status:\n 0  if OK,\n 1  if minor problems (e.g., cannot access subdirectory),\n 2  if serious trouble (e.g., cannot access command-line argument).\n",5),v1);
    v2 = "ls";
    if ((dat_261e0 != 1) && (v2 = "dir", dat_261e0 != 2))
      v2 = "vdir";
    sub_7e40(v2);
  }
  exit(a0); // no-return
}


// Function: sub_bda0 @ 0xbda0
unsigned long sub_bda0(unsigned int a0,char **a1)
{
  char v1;
  unsigned long v10; // rax
  char *v11;
  long *v12; // rax
  unsigned short v13; // stack - 0x48
  unsigned long v14;
  unsigned int v15; // stack - 0x74
  int v16; // stack - 0x70
  int v17; // stack - 0x6c
  unsigned long v18; // stack - 0x68
  char *v19; // stack - 0x60
  bool v2;
  long v20; // stack - 0x58
  int v21; // stack - 0x50
  unsigned short v22; // stack - 0x46
  unsigned int v23; // stack - 0x44
  FILE *v3;
  unsigned char v4;
  int v5;
  unsigned int v6;
  unsigned long v7; // rax
  long v8; // rax
  unsigned long v9; // rax
  
  v17 = -1;
  v18 = 0xffffffffffffffff;
  v20 = -1;
  v16 = -1;
  v21 = -1;
  v15 = 0xffffffff;
  v2 = 0;
  v19 = NULL;
label_be20:
  v22 = 0xffff;
  v13 = 0xffff;
  v14 = (unsigned long)a0;
  v5 = getopt_long(a0,a1,"abcdfghiklmnopqrstuvw:xABCDFGHI:LNQRST:UXZ1",(option *)0x252c0,(int *)&v13);
  if (v5 != -1) {
    switch(v5 + 0x83) {
      case 0:
        v11 = "ls";
        if ((dat_261e0 != 1) && (v11 = "dir", dat_261e0 != 2))
          v11 = "vdir";
        sub_16870(stdout,v11,"GNU coreutils",dat_261e8,"Richard M. Stallman","David MacKenzie",0,v14);
        exit(0); // no-return
      case 1:
        sub_b690(0); // no-return
      default:
        goto label_cc62;
      case 0xb4:
        v15 = (unsigned int)(v15 != 0);
        break;
      case 0xc4:
        dat_272f0 = 1;
        break;
      case 0xc5:
        sub_6380("*~");
        sub_6380(".*~");
        break;
      case 0xc6:
        v15 = 2;
        break;
      case 199:
        dat_27318 = 1;
        break;
      case 0xc9:
        if (optarg) {
          v5 = *(int *)(sub_d270("--classify",optarg,0x251a0,0x196c0,4,dat_261f0,1) * 4 + 0x196c0);
          if ((v5 != 1) && ((v5 != 2 || (!sub_6590())))) break;
        }
        dat_27314 = 3;
        break;
      case 0xca:
        dat_26028 = 0;
        break;
      case 0xcb:
        dat_272f8 = 2;
        break;
      case 0xcc:
        sub_6380(optarg);
        break;
      case 0xcf:
        dat_272f8 = 4;
        break;
      case 0xd1:
        v16 = 0;
        break;
      case 0xd4:
        v16 = 5;
        break;
      case 0xd5:
        dat_272f6 = 1;
        break;
      case 0xd6:
        v17 = 3;
        break;
      case 0xd7:
        v11 = dcgettext(NULL,"invalid tab size",5);
        v20 = sub_16de0(optarg,0,0,0x7fffffffffffffff,0x1abb9,v11,2);
        break;
      case 0xd8:
        v17 = 6;
        break;
      case 0xdb:
        v17 = 1;
        break;
      case 0xdd:
        dat_27365 = 1;
        break;
      case 0xe4:
        dat_272f0 = 2;
        break;
      case 0xe5:
        v16 = 7;
        break;
      case 0xe6:
        dat_27334 = 1;
        break;
      case 0xe7:
        dat_272f5 = 1;
        break;
      case 0xe9:
        dat_272f0 = 2;
        dat_27312 = 0;
        dat_27311 = 0;
        v6 = 0xffffffff;
        if (v15)
          v6 = v15;
        dat_2732c = 0;
        v17 = 6;
        v15 = v6;
        break;
      case 0xea:
        dat_26029 = 0;
        v15 = 0;
        break;
      case 0xeb:
        dat_27328 = 0xb0;
        dat_2731c = 0xb0;
        dat_27320 = 1;
        dat_26020 = 1;
        break;
      case 0xec:
        dat_272fc = 1;
        break;
      case 0xee:
        v2 = 1;
        break;
      case 0xef:
        v15 = 0;
        break;
      case 0xf0:
        v15 = 4;
        break;
      case 0xf1:
        dat_2732d = 1;
        v15 = 0;
        break;
      case 0xf2:
        dat_26028 = 0;
        v15 = 0;
        break;
      case 0xf3:
        dat_27314 = 1;
        break;
      case 0xf4:
        v21 = 1;
        break;
      case 0xf5:
        dat_2732f = 1;
        break;
      case 0xf6:
        dat_2732c = 1;
        break;
      case 0xf7:
        v17 = 5;
        break;
      case 0xf8:
        dat_27334 = 2;
        break;
      case 0xf9:
label_c180:
        v17 = 4;
        break;
      case 0xfa:
        v18 = sub_63b0(optarg);
        if (0 <= (long)v18) break;
        v7 = sub_158c0(optarg);
        error(2,0,"%s: %s",dcgettext(NULL,"invalid line width",5),v7);
        goto label_c180;
      case 0xfb:
        v15 = 3;
        break;
      case 0x103:
        dat_2732e = 1;
        break;
      case 0x104:
        v5 = sub_10810(optarg,0x27328,0x27320);
        if (v5)
          sub_17110(v5,CONCAT22(v22,v13),0,0x252c0,optarg); // no-return
        dat_2731c = dat_27328;
        dat_26020 = dat_27320;
        break;
      case 0x105:
        if (optarg) { // branch-flip
          v5 = *(int *)(sub_d270("--color",optarg,0x251a0,0x196c0,4,dat_261f0) * 4 + 0x196c0);
          if (v5 == 1) goto label_c0ae;
          v4 = 0;
          if (v5 == 2)
            v4 = sub_6590();
        }
        else {
label_c0ae:
          v4 = 1;
        }
        dat_27312 = v4 & 1;
        break;
      case 0x106:
        dat_272f8 = 3;
        break;
      case 0x107:
        dat_27314 = 2;
        break;
      case 0x108:
        v15 = *(unsigned int *)(sub_d270("--format",optarg,0x25280,0x19730,4,dat_261f0) * 4 + 0x19730);
        break;
      case 0x109:
        v19 = (char *)sub_55b0("full-iso");
        v15 = 0;
        break;
      case 0x10a:
        dat_272f4 = 1;
        break;
      case 0x10b:
        v12 = (long *)sub_169e0(0x10);
        *v12 = (long)optarg;
        v12[1] = (long)dat_272e0;
        dat_272e0 = v12;
        break;
      case 0x10c:
        if (optarg) { // branch-flip
          v5 = *(int *)(sub_d270("--hyperlink",optarg,0x251a0,0x196c0,4,dat_261f0,1) * 4 + 0x196c0);
          if (v5 == 1) goto label_bf7f;
          v4 = 0;
          if (v5 == 2)
            v4 = sub_6590();
        }
        else {
label_bf7f:
          v4 = 1;
        }
        dat_27311 = v4 & 1;
        break;
      case 0x10d:
        dat_27314 = *(unsigned int *)(sub_d270("--indicator-style",optarg,0x25940,0x19750,4,dat_261f0,1) * 4 + 0x19750);
        break;
      case 0x10e:
        v16 = *(int *)(sub_d270("--quoting-style",optarg,0x259e0,0x1d9c0,4,dat_261f0,1) * 4 + 0x1d9c0);
        break;
      case 0x10f:
        goto label_be86;
      case 0x110:
        dat_27328 = 0x90;
        dat_2731c = 0x90;
        dat_27320 = 1;
        dat_26020 = 1;
        break;
      case 0x111:
        v17 = *(int *)(sub_d270("--sort",optarg,0x25240,0x19710,4,dat_261f0,1) * 4 + 0x19710);
        break;
      case 0x112:
        dat_27334 = *(int *)(sub_d270("--time",optarg,0x25200,0x196f0,4,dat_261f0,1) * 4 + 0x196f0);
        break;
      case 0x113:
        goto label_c47c;
      case 0x114:
        dat_26019 = '\0';
        dat_27312 = 0;
        v16 = 0;
        v15 = (unsigned int)(v15 != 0);
label_be86:
        v21 = 0;
      
    }
    goto label_be20;
  }
  if (!dat_27320) {
    v11 = getenv("LS_BLOCK_SIZE");
    sub_10810(v11,0x27328,0x27320);
    if ((v11) || (getenv("BLOCK_SIZE"))) {
      dat_2731c = dat_27328;
      dat_26020 = dat_27320;
    }
    if (v2) {
      dat_27328 = 0;
      dat_27320 = 0x400;
    }
  }
  if (0 <= (int)v15) { // branch-flip
    dat_27338 = v15;
    if (2 < v15 - 2) goto label_c89f;
label_c580:
    if ((v18 == 0xffffffffffffffff) && (((!sub_6590() || (ioctl(1,0x5413,&v13) <= -1)) || (v18 = (unsigned long)v22, !v22)))) {
      v11 = getenv("COLUMNS");
      if ((v11) && (*v11)) {
        v18 = sub_63b0(v11);
        if ((long)v18 <= -1) {
          v11 = (char *)sub_158c0(v11);
          error(0,0,dcgettext(NULL,"ignoring invalid width in environment variable COLUMNS: %s",5),v11);
          v18 = 0x50;
        }
      }
      else {
label_c8b8:
        v18 = 0x50;
      }
    }
  }
  else {
    if (dat_261e0 != 1) { // branch-flip
      if (dat_261e0 == 2) {
label_cae0:
        dat_27338 = 2;
        goto label_c580;
      }
      dat_27338 = 0;
    }
    else {
      if (sub_6590()) goto label_cae0;
      dat_27338 = 1;
    }
label_c89f:
    if (dat_27312) goto label_c580;
    if (v18 == 0xffffffffffffffff) goto label_c8b8;
  }
  dat_272b0 = v18;
  dat_27200 = (unsigned long)(v18 != (SUB168(ZEXT816(v18) * ZEXT816(0xaaaaaaaaaaaaaaab),8) & 0xfffffffffffffffe) + v18 / 3) + v18 / 3;
  if (dat_27338 - 2 <= 2) {
    if (0 <= v20) { // branch-flip
label_c7a4:
      dat_272c0 = v20;
    }
    else {
      dat_272c0 = 8;
      v11 = getenv("TABSIZE");
      if (v11) {
        if (!sub_17190(v11,0,0,&v13,0x1abb9)) {
          v20 = CONCAT44(v23,CONCAT22(v22,v13));
          goto label_c7a4;
        }
        v11 = (char *)sub_158c0(v11);
        error(0,0,dcgettext(NULL,"ignoring invalid tab size in environment variable TABSIZE: %s",5),v11);
      }
    }
  }
  v4 = (unsigned char)v21;
  if ((v21 == -1) && (v4 = 0, dat_261e0 == 1))
    v4 = sub_6590();
  dat_272d8 = v4 & 1;
  if (0 <= v16) { // branch-flip
label_c5fd:
    sub_15200(0,v16); // crossjump-dupe
  }
  else {
    v16 = sub_64b0();
    if ((0 <= v16) || (v16 = 7, dat_261e0 != 1)) goto label_c5fd;
    if (sub_6590()) {
      v16 = 3;
      sub_15200(0,v16);
    }
  }
  v5 = sub_151e0(NULL);
  if (((dat_27338) && ((2 <= dat_27338 - 2 || (!dat_272b0)))) || ((v5 != 3 && ((v5 != 6 && (v5 != 1)))))) {
    dat_273a8 = 0;
    dat_272d0 = sub_151a0(0);
    if (v5 == 7)
      sub_15220(dat_272d0,0x20,1);
  }
  else {
    dat_273a8 = 1;
    dat_272d0 = sub_151a0(0);
  }
  if (2 <= dat_27314) {
    v11 = &"*=>@|"[dat_27314 - 2];
    v1 = "*=>@|"[dat_27314 - 2];
    while (v1) {
      v11 = &v11[1];
      sub_15220(dat_272d0,(int)v1,1);
      v1 = *v11;
    }
  }
  dat_272c8 = sub_151a0(0);
  sub_15220(dat_272c8,0x3a,1);
  dat_27318 = (dat_27311 ^ 1) & dat_27338 == 0 & dat_27318;
  if ((int)dat_26019 < (int)(unsigned int)dat_27318) {
    error(2,0,dcgettext(NULL,"--dired and --zero are incompatible",5));
    return v9;
  }
  if (0 <= v17) { // branch-flip
label_c700:
    dat_27330 = v17;
    if (dat_27338) {
      return (unsigned long)optind; // return-dupe
    }
  }
  else {
    if (dat_27338) {
      if ((unsigned int)(dat_27334 - 1U) <= 2) {
        dat_27330 = 5;
        return (unsigned long)optind;
      }
      v17 = 0;
      goto label_c700;
    }
    dat_27330 = 0;
  }
  if ((!v19) && (v19 = getenv("TIME_STYLE"), !v19))
    v19 = (char *)sub_55b0("locale");
  while (!strncmp(v19,"posix-",6)) {
    if (!sub_e9c0(2)) {
      return (unsigned long)optind;
    }
    v19 = &v19[6];
  }
  if (*v19 != '+') { // branch-flip
    v12 = (long *)0x25980;
    v8 = sub_cf70(v19,0x25980,0x19760,4);
    if (v8 < 0) {
      sub_d0e0("time style",v19,v8);
      v3 = stderr;
      fputs_unlocked(dcgettext(NULL,"Valid arguments are:\n",5),v3);
      v11 = "full-iso";
      do {
        v12 = &v12[1];
        __fprintf_chk(stderr,1,"  - [posix-]%s\n",v11);
        v3 = stderr;
        v11 = (char *)*v12;
      } while (v11);
      fputs_unlocked(dcgettext(NULL,"  - +FORMAT (e.g., +%H:%M) for a \'date\'-style format\n",5),v3);
label_cc62:
      sub_b690(2); // no-return
    }
    switch(v8) { // branch-flip
      case 0:
        dat_26048 = "%Y-%m-%d %H:%M:%S.%N %z";
        dat_26040 = "%Y-%m-%d %H:%M:%S.%N %z";
        break;
      case 1:
        dat_26048 = "%Y-%m-%d %H:%M";
        dat_26040 = "%Y-%m-%d %H:%M";
        break;
      case 2:
        dat_26040 = "%Y-%m-%d ";
        dat_26048 = "%m-%d %H:%M";
        break;
      case 3:
        if (sub_e9c0(2)) {
          dat_26040 = dcgettext(NULL,dat_26040,2);
          dat_26048 = dcgettext(NULL,dat_26048,2);
        }
      
    }
  }
  else {
    v19 = &v19[1];
    v11 = strchr(v19,10);
    if (v11) { // branch-flip
      if (strchr(&v11[1],10)) {
        v11 = (char *)sub_158c0(v19);
        error(2,0,dcgettext(NULL,"invalid time style format %s",5),v11);
        return v10;
      }
      *v11 = '\0';
      dat_26048 = &v11[1];
    }
    else {
      dat_26048 = v19;
    }
    dat_26040 = v19;
  }
  sub_66c0();
  return (unsigned long)optind;
label_c47c:
  v19 = optarg;
  goto label_be20;
}


// Function: sub_ea50 @ 0xea50
unsigned long sub_ea50(unsigned long a0)
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


// Function: sub_eac0 @ 0xeac0
unsigned long sub_eac0(unsigned long a0)
{
  char v1; // al
  unsigned long v2; // rax
  unsigned long v3;
  
  v2 = 10;
  if (10 <= a0)
    v2 = a0;
  v3 = v2 | 1;
  while ((v3 != 0xffffffffffffffff && (v1 = sub_ea50(v3), !v1))) {
    v3 += 2;
  }
  v2 = v3;
  return v2;
}


// Function: sub_eb00 @ 0xeb00
unsigned long sub_eb00(unsigned long a0,unsigned long a1)
{
  return (unsigned long)sub_17eb0(a0,3) % a1;
}


// Function: sub_eb20 @ 0xeb20
bool sub_eb20(long a0,long a1)
{
  return a1 == a0;
}


// Function: sub_eb30 @ 0xeb30
unsigned long sub_eb30(struct_18 *a0) // return-dupe
{
  float v1;
  float *v2;
  float v3; // xmm1_da
  
  v2 = a0->field_0x28;
  if (v2 == (float *)0x1d2f0)
    return 1;
  v1 = v2[2];
  if ((((dat_1d304 < v1) && (v1 < dat_1d308)) && (dat_1d30c < v2[3])) && (dat_1d320 <= *v2)) {
    v3 = *v2 + dat_1d304;
    if (((v3 < v2[1]) && (v2[1] <= dat_1d310)) && (v3 < v1))
      return 1;
  }
  a0->field_0x28 = (float *)0x1d2f0;
  return 0;
}


// Function: sub_ebc0 @ 0xebc0
void sub_ebc0(struct_20 *a0,struct_19 *a1)
{
  unsigned long v1;
  
  v1 = a0->field_0x48;
  a1->field_0x0 = 0;
  a1->field_0x8 = v1;
  a0->field_0x48 = a1;
}


// Function: sub_ebe0 @ 0xebe0
long sub_ebe0(struct_21 *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  v1 = (*a0->field_0x30)(a1,a0->field_0x10);
  if (v1 < (unsigned long)a0->field_0x10)
    return v1 * 0x10 + a0->field_0x0;
  abort(); // no-return
}


// Function: sub_ec10 @ 0xec10
long sub_ec10(struct_22 *a0,long a1,unsigned long *a2,bool a3) // early-return
{
  void *v1;
  long *v2;
  long v3;
  long *v4;
  long v5;
  
  v4 = (long *)sub_ebe0(a0,a1);
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
      sub_ebc0(a0);
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
      sub_ebc0(a0);
      return v5;
    }
    *v4 = 0;
  }
  return v5;
}


// Function: sub_ed10 @ 0xed10
void sub_ed10(struct_20 *a0)
{
  if (a0->field_0x48) {
    a0->field_0x48 = *(unsigned long *)(a0->field_0x48 + 8);
    return;
  }
  malloc(0x10); // tail-call
}


// Function: sub_ed40 @ 0xed40
unsigned long sub_ed40(struct_24 *a0,struct_30 *a1,bool a2) // return-dupe x2
{
  long *v1;
  long v2;
  struct_0 *v3; // rax
  struct_0 *v4; // rax
  long *v5;
  long *v6;
  long v7;
  
  v5 = a1->field_0x0;
  if (a1->field_0x8 <= v5)
    return 1;
  do {
    while (v7 = *v5, !v7) {
label_ed68:
      v5 = &v5[2];
      if (a1->field_0x8 <= v5)
        return 1;
    }
    v6 = (long *)v5[1];
    if ((long *)v5[1]) {
      do {
        while( true ) {
          v7 = *v6;
          v3 = (struct_0 *)sub_ebe0(a0,v7);
          v1 = (long *)v6[1];
          if (v3->field_0x0) break;
          v3->field_0x0 = v7;
          a0->field_0x18 = a0->field_0x18 + 1;
          sub_ebc0(a0);
          v6 = v1;
          if (!v1) goto label_edd8;
        }
        v6[1] = v3->field_0x8;
        v3->field_0x8 = (long)v6;
        v6 = v1;
      } while (v1);
label_edd8:
      v7 = *v5;
    }
    v5[1] = 0;
    if (a2) goto label_ed68;
    v3 = (struct_0 *)sub_ebe0(a0,v7);
    if (v3->field_0x0) { // branch-flip
      v4 = (struct_0 *)sub_ed10(a0);
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


// Function: sub_ef00 @ 0xef00
unsigned long sub_ef00(struct_9 *a0)
{
  return a0->field_0x10;
}


// Function: sub_ef10 @ 0xef10
unsigned long sub_ef10(struct_10 *a0)
{
  return a0->field_0x18;
}


// Function: sub_ef20 @ 0xef20
unsigned long sub_ef20(struct_11 *a0)
{
  return a0->field_0x20;
}


// Function: sub_ef30 @ 0xef30
unsigned long sub_ef30(struct_30 *a0)
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


// Function: sub_f000 @ 0xf000
void sub_f000(struct_11 *a0,FILE *a1)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // rax
  
  v1 = sub_ef20(a0);
  v2 = sub_ef00();
  v3 = sub_ef10();
  v4 = sub_ef30();
  __fprintf_chk(a1,1,"# entries:         %lu\n",v1);
  __fprintf_chk(a1,1,"# buckets:         %lu\n",v2);
  __fprintf_chk(a1,1,"# buckets used:    %lu (%.2f%%)\n",v3,((double)v3 * dat_1d320) / (double)v2);
  __fprintf_chk(a1,1,"max bucket length: %lu\n",v4); // tail-call
}


// Function: sub_f120 @ 0xf120
long sub_f120(struct_22 *a0,long a1)
{
  void *v1;
  long *v2;
  long v3;
  
  v2 = (long *)sub_ebe0(a0,a1);
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


// Function: sub_f1d0 @ 0xf1d0
long sub_f1d0(void *a0,long a1)
{
  long v1;
  long *v2; // rax
  long *v3;
  
  v2 = (long *)sub_ebe0(a0,a1);
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


// Function: sub_f290 @ 0xf290
long sub_f290(struct_30 *a0,void *a1,unsigned long a2)
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


// Function: sub_f380 @ 0xf380
unsigned long * sub_f380(unsigned long a0,void *a1,void *a2,void *a3,unsigned long a4) // return-dupe
{
  unsigned long *v1; // rax
  unsigned long v2; // rax
  void *v3; // rax
  
  if (!a2)
    a2 = sub_eb00;
  if (!a3)
    a3 = sub_eb20;
  v1 = malloc(0x50);
  if (!v1)
    return NULL;
  if (!a1)
    a1 = (void *)0x1d2f0;
  v1[5] = a1;
  if (sub_eb30(v1)) {
    v2 = sub_ee70(*(float *)((long)a1 + 8),a0,(unsigned char)*(char *)((long)a1 + 0x10));
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


// Function: sub_f490 @ 0xf490
void sub_f490(struct_32 *a0)
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
        if (a0->field_0x8 <= v5) goto label_f52b;
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
label_f52b:
  a0->field_0x18 = 0;
  a0->field_0x20 = 0;
  return;
}


// Function: sub_f540 @ 0xf540
void sub_f540(struct_27 *a0)
{
  void *v1;
  void *v2;
  long *v3;
  long *v4;
  long v5;
  
  v4 = a0->field_0x0;
  v3 = a0->field_0x8;
  if ((a0->field_0x40) && (a0->field_0x20)) {
    if (v3 <= v4) goto label_f5ec;
    do {
      while (v5 = *v4, !v5) {
        v4 = &v4[2];
        if (v3 <= v4) goto label_f5af;
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
label_f5af:
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
label_f5ec:
  v2 = a0->field_0x48;
  while (v2) {
    v1 = *(void **)((long)v2 + 8);
    free(v2);
    v2 = v1;
  }
  free(a0->field_0x0);
  free(a0); // tail-call
}


// Function: sub_f630 @ 0xf630
unsigned int sub_f630(struct_25 *a0,unsigned long a1) // return-dupe
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
  
  v4 = sub_ee70(*(float *)(a0->field_0x28 + 8),a1,(unsigned char)*(char *)(a0->field_0x28 + 0x10));
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
  v1 = sub_ed40(&v2,a0,0);
  if (!(char)v1) {
    a0->field_0x48 = v11;
    if ((sub_ed40(a0,&v2,1)) && (sub_ed40(a0,&v2,0))) {
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


// Function: sub_f7b0 @ 0xf7b0
unsigned long sub_f7b0(void *a0,long a1,long *a2) // ternary
{
  long v1;
  struct_0 *v2; // rax
  long *v3; // stack - 0x38
  unsigned long v4; // rsi
  float v5; // xmm4_da
  float v6; // xmm5_da
  
  if (a1) {
    v1 = sub_ec10(a0,a1,&v3,0);
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
      sub_eb30(a0);
      v1 = *(long *)((long)a0 + 0x28);
      if (v5 * *(float *)(v1 + 8) < v6) {
        v5 = v5 * *(float *)(v1 + 0xc);
        if (!*(char *)(v1 + 0x10))
          v5 = v5 * *(float *)(v1 + 8);
        if (dat_1d314 <= v5)
          return 0xffffffff;
        v4 = (dat_1d318 <= v5) ? (long)(v5 - dat_1d318) ^ 0x8000000000000000 : (unsigned long)v5;
        if (!sub_f630(a0,v4))
          return 0xffffffff;
        if (sub_ec10(a0,a1,&v3,0)) {
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
    v2 = (struct_0 *)sub_ed10(a0);
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


// Function: sub_f9a0 @ 0xf9a0
unsigned long sub_f9a0(void *a0,unsigned long a1) // early-return
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x18
  bool v4; // zf
  
  v1 = sub_f7b0(a0,a1,&v3);
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


// Function: sub_fa00 @ 0xfa00
long sub_fa00(void *a0,long a1) // ternary x2
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
  
  v5 = sub_ec10(a0,a1,&v7,1);
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
      sub_eb30(a0);
      v2 = *(float **)((long)a0 + 0x28);
      if (v9 < *v2 * v8) {
        v8 = v8 * v2[1];
        if (!*(char *)&v2[4])
          v8 = v8 * v2[2];
        v6 = (dat_1d318 <= v8) ? (long)(v8 - dat_1d318) ^ 0x8000000000000000 : (unsigned long)v8;
        if (!sub_f630(a0,v6)) {
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

