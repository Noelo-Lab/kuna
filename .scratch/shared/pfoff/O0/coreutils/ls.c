// Function: sub_53e7 @ 0x53e7
unsigned int sub_53e7(long a0)
{
  unsigned int v1;
  
  if ((dat_2b358) && (*(char *)(a0 + 0xb9)))
    v1 = *(unsigned int *)(a0 + 0xac);
  else {
    v1 = *(unsigned int *)(a0 + 0x30);
  }
  return v1;
}


// Function: sub_5422 @ 0x5422
void sub_5422(char a0)
{
  dat_2b500 += 1;
  putchar_unlocked((int)a0);
}


// Function: sub_5453 @ 0x5453
void sub_5453(char *a0,unsigned long a1)
{
  dat_2b500 = a1 + dat_2b500;
  fwrite_unlocked(a0,1,a1,stdout);
}


// Function: sub_549e @ 0x549e
void sub_549e(char *a0)
{
  sub_5453(a0,strlen(a0));
}


// Function: sub_54cf @ 0x54cf
void sub_54cf(void) // return-dupe
{
  if (!dat_2b3cc)
    return;
  sub_549e("  ");
}


// Function: sub_54f4 @ 0x54f4
void sub_54f4(obstack *a0) // return-dupe
{
  if (!dat_2b3cc)
    return;
  if ((unsigned long)(*(long *)&a0->field_0x20 - *(long *)&a0->field_0x18) < 8)
    _obstack_newchunk(a0,8);
  memcpy(*(void **)&a0->field_0x18,(void *)0x2b500,8);
  *(long *)&a0->field_0x18 = *(long *)&a0->field_0x18 + 8;
}


// Function: sub_558e @ 0x558e
void sub_558e(unsigned long a0,unsigned long a1)
{
  if ((unsigned long)(dat_2b600 - dat_2b5f8) < 0x10)
    _obstack_newchunk((obstack *)0x2b5e0,0x10);
  *(unsigned long *)(dat_2b5f8 + -8) = a0;
  *(unsigned long *)(dat_2b5f8 + -0x10) = a1;
}


// Function: sub_563d @ 0x563d
undefined16 sub_563d(void)
{
  if ((unsigned long)((long)dat_2b5f8 - dat_2b5f0) < 0x10)
    __assert_fail("dev_ino_size <= obstack_object_size (&dev_ino_obstack)","src/ls.c",0x41d,"dev_ino_pop"); // no-return
  dat_2b5f8 = (char (*)[16])((long)dat_2b5f8 + -0x10);
  return *dat_2b5f8;
}


// Function: sub_56dc @ 0x56dc
void sub_56dc(char *a0,long a1,long a2)
{
  stat v1; // stack - 0xa8
  
  if (!a0)
    __assert_fail("name","src/ls.c",0x428,"assert_matching_dev_ino"); // no-return
  if (stat(a0,&v1) <= -1)
    __assert_fail("0 <= stat (name, &sb)","src/ls.c",0x429,"assert_matching_dev_ino"); // no-return
  if (v1._0_8_ != a2)
    __assert_fail("sb.st_dev == di.st_dev","src/ls.c",0x42a,"assert_matching_dev_ino"); // no-return
  if (v1._8_8_ == a1)
    return;
  __assert_fail("sb.st_ino == di.st_ino","src/ls.c",0x42b,"assert_matching_dev_ino"); // no-return
}


// Function: sub_581f @ 0x581f
void sub_581f(char *a0,long a1) // return-dupe
{
  long v1;
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x40
  
  v2 = (unsigned long)(*(long *)(a1 + 0x18) - *(long *)(a1 + 0x10)) >> 3;
  if (!v2)
    return;
  v1 = *(long *)(a1 + 0x10);
  if (v1 == *(long *)(a1 + 0x18))
    *(unsigned char *)(a1 + 0x50) = *(unsigned char *)(a1 + 0x50) | 2;
  *(unsigned long *)(a1 + 0x18) = ~*(unsigned long *)(a1 + 0x30) & *(long *)(a1 + 0x18) + *(long *)(a1 + 0x30);
  if ((unsigned long)(*(long *)(a1 + 0x20) - *(long *)(a1 + 8)) < (unsigned long)(*(long *)(a1 + 0x18) - *(long *)(a1 + 8)))
    *(unsigned long *)(a1 + 0x18) = *(unsigned long *)(a1 + 0x20);
  *(unsigned long *)(a1 + 0x10) = *(unsigned long *)(a1 + 0x18);
  fputs_unlocked(a0,stdout);
  for (v3 = 0; v3 < v2; v3 = v3 + 1) {
    printf(" %ld",*(long *)(v1 + v3 * 8));
  }
  putchar_unlocked(10);
}


// Function: sub_5990 @ 0x5990
undefined16 sub_5990(long a0)
{
  return sub_1b491(a0);
}


// Function: sub_59be @ 0x59be
unsigned long sub_59be(void) // return-dupe x3
{
  switch(dat_2b3b0) {
    case 0:
      return 0x40;
    default:
      abort(); // no-return
    case 1:
      return 0x80;
    case 2:
      return 0x20;
    case 3:
      return 0x800;
    
  }
}


// Function: sub_5a0c @ 0x5a0c
unsigned int sub_5a0c(void)
{
  unsigned int v1; // eax
  unsigned int v2; // stack - 0xc
  
  v2 = 2;
  if (dat_2b3e9)
    v2 = 0x102;
  if (dat_2b3bb)
    v2 |= 0x400;
  if (!dat_2b3ac) {
    v1 = sub_59be() | v2;
    v2 = v1 | 0x204;
    if ((dat_2b022) || (dat_2b3b9))
      v2 = v1 | 0x20c;
    if (dat_2b023)
      v2 |= 0x10;
  }
  switch(dat_2b3b4) {
    case 0:
    case 1:
    case 2:
    case 4:
    case 6:
      break;
    case 3:
      v2 |= 0x200;
      break;
    case 5:
      v2 |= sub_59be();
      break;
    default:
      abort(); // no-return
    
  }
  return v2;
}


// Function: sub_5ad2 @ 0x5ad2
int sub_5ad2(int a0,char *a1,struct_4 *a2,unsigned int a3,unsigned int a4)
{
  int v1; // eax
  unsigned int v2 [20]; // stack - 0x128
  char v3 [16];
  unsigned long v4; // stack - 0xd8
  unsigned long v5; // stack - 0xd0
  
  v1 = statx(a0,a1,a3 | 0x800,a4,v2);
  if ((0 <= v1) && (sub_5297(v2,a2), a4 & 0x800)) {
    if (v2[0] & 0x800) { // branch-flip
      v3 = sub_5267(v4,v5);
      a2->field_0x58 = SUB168(v3,0);
      a2->field_0x60 = SUB168(v3,8);
    }
    else {
      a2->field_0x60 = 0xffffffffffffffff;
      a2->field_0x58 = a2->field_0x60;
    }
  }
  return v1;
}


// Function: sub_5c0e @ 0x5c0e
void sub_5c0e(char *a0,struct_4 *a1)
{
  sub_5ad2(0xffffff9c,a0,a1,0,sub_5a0c());
}


// Function: sub_5c48 @ 0x5c48
void sub_5c48(char *a0,struct_4 *a1)
{
  sub_5ad2(0xffffff9c,a0,a1,0x100,sub_5a0c());
}


// Function: sub_5c82 @ 0x5c82
void sub_5c82(char *a0,struct_4 *a1)
{
  sub_5ad2(0xffffff9c,a0,a1,0,2);
}


// Function: sub_5cb8 @ 0x5cb8
void sub_5cb8(char *a0,struct_4 *a1)
{
  sub_5ad2(0xffffff9c,a0,a1,0,0x100);
}


// Function: sub_5cee @ 0x5cee
void sub_5cee(unsigned int a0,struct_4 *a1)
{
  sub_5ad2(a0,"",a1,0x1000,0x100);
}


// Function: sub_5d23 @ 0x5d23
char * sub_5d23(char *a0)
{
  char *v1; // stack - 0x10
  
  v1 = a0;
  do {
    if (!*v1)
      return NULL;
    if (*v1 == '%') {
      if (v1[1] != '%') { // branch-flip
        if (v1[1] == 'b')
          return v1;
      }
      else {
        v1 = &v1[1];
      }
    }
    v1 = &v1[1];
  } while( true );
}


// Function: sub_5d77 @ 0x5d77
void sub_5d77(void) // warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_5d77
{
  unsigned char v1; // cl
  int v2; // stack - 0xc
  
  for (v2 = 0; v2 <= 0xff; v2 = v2 + 1) {
    v1 = ((((sub_10016(v2)) || (v2 == 0x7e)) || ((v2 == 0x2d || (v2 == 0x2e)))) || (v2 == 0x5f));
    *(unsigned char *)((long)v2 + 0x2b640) = *(unsigned char *)((long)v2 + 0x2b640) | v1;
  }
}


// Function: sub_5df4 @ 0x5df4
unsigned long sub_5df4(long a0)
{
  char v1;
  long v2;
  char *v3; // rax
  unsigned long v4; // stack - 0x48
  int v5; // stack - 0x50
  unsigned long v6; // stack - 0x40
  unsigned long v7; // stack - 0x38
  char *v8; // stack - 0x30
  unsigned long v9; // stack - 0x28
  
  v6 = 0xc;
  do {
    v7 = v6;
    v6 = 0;
    v5 = 0;
    while (v5 <= 0xb) {
      v4 = v7;
      v3 = nl_langinfo(v5 + 0x2000e);
      v8 = v3;
      if (strchr(v3,0x25))
        return 0;
      v2 = *(long *)__ctype_b_loc();
      v1 = *v8;
      v9 = sub_15291(v8,(char *)((long)v5 * 0x80 + a0),0x80,&v4,(*(unsigned short *)((unsigned long)(unsigned char)sub_4eb2((int)v1) * 2 + v2) & 0x800) != 0,0);
      if (0x80 <= v9)
        return 0;
      if (v4 <= v6)
        v4 = v6;
      v5 += 1;
      v6 = v4;
    }
    if (v7 <= v6)
      return 1;
  } while( true );
}


// Function: sub_5f43 @ 0x5f43
void sub_5f43(void)
{
  char *v1;
  int v10; // stack - 0x640
  long v2;
  long v3;
  char *v4; // rax
  long v5 [2]; // stack - 0x628
  char v6 [1544];
  int v7; // stack - 0x64c
  int v8; // stack - 0x648
  int v9; // stack - 0x644
  
  for (v7 = 0; v7 <= 1; v7 = v7 + 1) {
    v1 = *(char **)((long)v7 * 8 + 0x2b200);
    v5[v7] = sub_5d23(v1);
  }
  if (((v5[0]) || (v5[1])) && (sub_5df4(v6) == '\x01')) {
    for (v8 = 0; v8 <= 1; v8 = v8 + 1) {
      v1 = *(char **)((long)v8 * 8 + 0x2b200);
      for (v9 = 0; v9 <= 0xb; v9 = v9 + 1) {
        v4 = (char *)(((long)v8 * 0xc + (long)v9) * 0x80 + 0x2b740);
        if (v5[v8]) { // branch-flip
          if (0x80 < v5[v8] - (long)v1)
            return;
          v2 = v5[v8];
          v3 = v5[v8];
          v10 = snprintf(v4,0x80,"%.*s%s%s",(int)v2 - (int)v1,v1,&v6[(long)v9 * 0x80],(char *)(v3 + 2));
        }
        else {
          v10 = snprintf(v4,0x80,"%s",v1);
        }
        if (v10 < 0)
          return;
        if (0x7f < v10)
          return;
      }
    }
    dat_2c340 = 1;
  }
}


// Function: sub_61bd @ 0x61bd
unsigned long sub_61bd(unsigned long *a0,unsigned long a1)
{
  return *a0 % a1;
}


// Function: sub_61ea @ 0x61ea
unsigned int sub_61ea(struct_0 *a0,struct_0 *a1) // return-dupe
{
  if ((a0->field_0x0 == a1->field_0x0) && (a0->field_0x8 == a1->field_0x8))
    return 1;
  return 0;
}


// Function: sub_6243 @ 0x6243
void sub_6243(void *a0)
{
  free(a0);
}


// Function: sub_6262 @ 0x6262
bool sub_6262(unsigned long a0,unsigned long a1)
{
  struct_6 *v1; // rax
  struct_6 *v2; // rax
  
  v1 = (struct_6 *)sub_1c68b(0x10);
  v1->field_0x0 = a1;
  v1->field_0x8 = a0;
  v2 = (struct_6 *)sub_136d0(dat_2b320,v1);
  if (!v2)
    sub_1ccc3(); // no-return
  if (v2 != v1)
    free(v1);
  return v2 != v1;
}


// Function: sub_62e7 @ 0x62e7
void sub_62e7(struct_1 *a0)
{
  free(a0->field_0x0);
  free(a0->field_0x8);
  free(a0);
}


// Function: sub_6325 @ 0x6325
unsigned int sub_6325(unsigned int a0)
{
  long v1;
  char *v2;
  unsigned int v3; // eax
  
  v1 = *(long *)((unsigned long)a0 * 0x10 + 0x2b080);
  v2 = *(char **)((unsigned long)a0 * 0x10 + 0x2b088);
  if (v1) { // branch-flip
    if (v1 == 1) {
      if (!strncmp(v2,"0",1)) {
        v3 = 0;
        return v3;
      }
    }
    if (v1 == 2) {
      if (!strncmp(v2,"00",2)) {
        v3 = 0;
        return v3;
      }
    }
    v3 = 1;
  }
  else {
    v3 = 0;
  }
  return v3;
}


// Function: sub_63ca @ 0x63ca
void sub_63ca(void)
{
  sub_e4ca((struct_2 *)0x2b080);
  sub_e4ca((struct_2 *)0x2b090);
}


// Function: sub_63f3 @ 0x63f3
void sub_63f3(void)
{
  if (dat_2b3d4) {
    if (sub_6325(4)) {
      sub_e4ca((struct_2 *)0x2b080);
      sub_e4ca((struct_2 *)0x2b0c0);
      sub_e4ca((struct_2 *)0x2b090);
    }
  }
}


// Function: sub_6444 @ 0x6444
void sub_6444(unsigned int a0) // return-dupe
{
  if (dat_2b4e0)
    return;
  dat_2b4e0 = a0;
}


// Function: sub_6465 @ 0x6465
void sub_6465(void) // return-dupe
{
  if (dat_2b4e0)
    return;
  dat_2b4e4 += 1;
}


// Function: sub_648c @ 0x648c
void sub_648c(void)
{
  sigset_t v1;
  int v2; // stack - 0xa0
  
  while ((dat_2b4e0 || (dat_2b4e4))) {
    if (dat_2b3d6)
      sub_63ca();
    fflush_unlocked(stdout);
    sigprocmask(0,(sigset_t *)0x2b460,&v1);
    v2 = dat_2b4e0;
    if (dat_2b4e4) { // branch-flip
      dat_2b4e4 -= 1;
      v2 = 0x13;
    }
    else {
      signal(dat_2b4e0,0);
    }
    raise(v2);
    sigprocmask(2,&v1,NULL);
  }
}


// Function: sub_6593 @ 0x6593
void sub_6593(bool a0) // return-dupe, ternary
{
  int v1;
  void *v2 [17]; // stack - 0xa8
  int v3; // stack - 0xac
  undefined4 v4; // stack - 0x20
  
  if (a0) {
    sigemptyset((sigset_t *)0x2b460);
    for (v3 = 0; v3 <= 0xb; v3 = v3 + 1) {
      sigaction(*(int *)((long)v3 * 4 + 0x23380),NULL,(sigaction *)v2);
      if (v2[0] != (void *)0x1)
        sigaddset((sigset_t *)0x2b460,*(int *)((long)v3 * 4 + 0x23380));
    }
    v2[1] = (void *)dat_2b460;
    v2[2] = (void *)dat_2b468;
    v2[3] = (void *)dat_2b470;
    v2[4] = (void *)dat_2b478;
    v2[5] = (void *)dat_2b480;
    v2[6] = (void *)dat_2b488;
    v2[7] = (void *)dat_2b490;
    v2[8] = (void *)dat_2b498;
    v2[9] = (void *)dat_2b4a0;
    v2[10] = (void *)dat_2b4a8;
    v2[0xb] = (void *)dat_2b4b0;
    v2[0xc] = (void *)dat_2b4b8;
    v2[0xd] = (void *)dat_2b4c0;
    v2[0xe] = (void *)dat_2b4c8;
    v2[0xf] = (void *)dat_2b4d0;
    v2[0x10] = (void *)dat_2b4d8;
    v4 = 0x10000000;
    for (v3 = 0; v3 <= 0xb; v3 = v3 + 1) {
      v1 = *(int *)((long)v3 * 4 + 0x23380);
      if (sigismember((void *)0x2b460,v1)) {
        v2[0] = (*(int *)((long)v3 * 4 + 0x23380) != 0x14) ? sub_6444 : sub_6465; // branch-flip
        sigaction(*(int *)((long)v3 * 4 + 0x23380),(sigaction *)v2,NULL);
      }
    }
    return;
  }
  for (v3 = 0; v3 <= 0xb; v3 = v3 + 1) {
    v1 = *(int *)((long)v3 * 4 + 0x23380);
    if (sigismember((void *)0x2b460,v1))
      signal(*(unsigned int *)((long)v3 * 4 + 0x23380),0);
  }
}


// Function: sub_6858 @ 0x6858
void sub_6858(void)
{
  sub_6593(1);
}


// Function: sub_686d @ 0x686d
void sub_686d(void)
{
  sub_6593(0);
}


// Function: main @ 0x6882
int main(int argc,char **argv,char **envp) // warn: iteboolean: re-rolled 2 0/1 select diamond(s) to a boolean assignment in main
{
  int v1;
  char v10 [16];
  int v11; // stack - 0x44
  int v12; // stack - 0x40
  unsigned long v13; // stack - 0x20
  long *v2;
  long *v3;
  char *v4;
  long *v5;
  int v6; // eax
  unsigned int v7; // eax
  void *v8; // rax
  unsigned long v9; // stack - 0x28
  
  sub_18e89(*argv);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  sub_4e99(2);
  sub_1ec10(sub_110ae);
  dat_2b4e8 = 0;
  dat_2b428 = 1;
  dat_2b368 = NULL;
  dat_2b370 = 0x8000000000000000;
  dat_2b378 = 0xffffffffffffffff;
  v6 = sub_70af(argc,argv);
  if (dat_2b3d4)
    sub_88ab();
  if (dat_2b3d4)
    dat_2b420 = 0;
  if (dat_2b3f2) // branch-flip
    dat_2b3e8 = 1;
  else if ((dat_2b3d4) && (((sub_6325(0xd) || ((sub_6325(0xe) && (dat_2b358)))) || ((sub_6325(0xc) && (!dat_2b3ac))))))
    dat_2b3e8 = 1;
  if (!dat_2b3ec) {
    if (((dat_2b3f1) || (dat_2b3d0 == 3)) || (!dat_2b3ac))
      dat_2b3ec = 1;
    else {
      dat_2b3ec = 3;
    }
  }
  if (dat_2b3f0) {
    dat_2b320 = sub_1298f(0x1e,0,sub_61bd,sub_61ea,sub_6243);
    if (!dat_2b320)
      sub_1ccc3(); // no-return
    _obstack_begin((obstack *)0x2b5e0,0,0,dat_2afd0,dat_2af98);
  }
  dat_2b438 = sub_1b525(getenv("TZ"));
  dat_2b440 = !(((dat_2b3b4 != 5) && (dat_2b3b4 != 3)) && ((dat_2b3ac && ((!dat_2b380 && (!dat_2b3bb))))));
  dat_2b441 = ((dat_2b440 != '\x01') && ((((dat_2b3f0 || (dat_2b3d4)) || (dat_2b3d0)) || (dat_2b3f2))));
  if (dat_2b3cc) {
    _obstack_begin((obstack *)0x2b520,0,0,dat_2afd0,dat_2af98);
    _obstack_begin((obstack *)0x2b580,0,0,dat_2afd0,dat_2af98);
  }
  if (dat_2b3d5) {
    sub_5d77();
    dat_2b360 = sub_1ce96();
    if (!dat_2b360)
      dat_2b360 = 0x1f27b;
  }
  dat_2b330 = 100;
  dat_2b328 = sub_1c804(100,0xd0);
  dat_2b338 = 0;
  sub_96f3();
  v11 = v6; // branch-flip
  if (1 <= argc - v6) {
    do {
      v1 = v11 + 1;
      sub_9a40(argv[v11],0,0,1,"");
      v11 = v1;
    } while (v1 < argc);
  }
  else if (dat_2b3f1) // branch-flip
    sub_9a40(".",3,0,1,"");
  else {
    sub_8e46(".",NULL,1);
  }
  if ((dat_2b338) && (sub_bfc5(), dat_2b3f1 != '\x01'))
    sub_aa65(NULL,1);
  if (dat_2b338) { // branch-flip
    sub_c12d();
    if (dat_2b368)
      sub_5422(10);
  }
  else if (((argc - v6 <= 1) && (dat_2b368)) && (!dat_2b368[3]))
    dat_2b428 = 0;
  while (v5 = dat_2b368, dat_2b368) {
    if ((dat_2b320) && (!*dat_2b368)) {
      dat_2b368 = (long *)dat_2b368[3];
      v10 = sub_563d();
      v13 = SUB168(v10,8);
      v9 = SUB168(v10,0);
      v8 = (void *)sub_13740(dat_2b320,&v9);
      if (!v8)
        __assert_fail("found","src/ls.c",0x70d,"main"); // no-return
      sub_6243(v8);
      sub_62e7(v5);
    }
    else {
      v2 = &dat_2b368[2];
      v3 = &dat_2b368[1];
      v4 = (char *)*dat_2b368;
      dat_2b368 = (long *)dat_2b368[3];
      sub_8ede(v4,(char *)*v3,(unsigned char)*(char *)v2);
      sub_62e7(v5);
      dat_2b428 = 1;
    }
  }
  if ((dat_2b3d4) && (dat_2b3d6)) {
    if ((dat_2b080 != 2) || (((memcmp(dat_2b088,(void *)0x20638,2) || (dat_2b090 != 1)) || (*dat_2b098 != 'm'))))
      sub_63ca();
    fflush_unlocked(stdout);
    sub_686d();
    for (v12 = dat_2b4e4; v12; v12 = v12 + -1) {
      raise(0x13);
    }
    if (dat_2b4e0)
      raise(dat_2b4e0);
  }
  if (dat_2b3cc) {
    sub_581f("//DIRED//",0x2b520);
    sub_581f("//SUBDIRED//",0x2b580);
    v7 = sub_18fc3(dat_2b410);
    printf("//DIRED-OPTIONS// --quoting-style=%s\n",*(char **)((unsigned long)v7 * 8 + 0x2a9e0));
  }
  if (dat_2b320) {
    if (sub_12002(dat_2b320))
      __assert_fail("hash_get_n_entries (active_dir_set) == 0","src/ls.c",0x741,"main"); // no-return
    sub_12c0f(dat_2b320);
  }
  return dat_2b4e8;
}


// Function: sub_6fe6 @ 0x6fe6
long sub_6fe6(unsigned char *a0) // early-return x3
{
  int v1; // eax
  long v2; // stack - 0x18
  
  v1 = sub_1d251(a0,NULL,0,&v2,"");
  if (!v1) {
    if (0 <= v2)
      return v2;
    return 0;
  }
  if (v1 != 1)
    return -1;
  return 0;
}


// Function: sub_706a @ 0x706a
bool sub_706a(void)
{
  if (dat_2b031 <= '\xff')
    dat_2b031 = (char)isatty(1);
  return dat_2b031 != '\0';
}


// Function: sub_70af @ 0x70af
unsigned long sub_70af(int a0,char **a1) // ternary x4, warn: iteboolean: re-rolled 4 0/1 select diamond(s) to a boolean assignment in sub_70af
{
  bool v1;
  long v10; // rax
  unsigned short v11; // stack - 0x28
  int v12; // stack - 0xc4
  int v13; // stack - 0xc0
  int v14; // stack - 0xbc
  int v15; // stack - 0xb8
  int v16; // stack - 0xb4
  int v17; // stack - 0xb0
  int v18; // stack - 0xac
  int v19; // stack - 0xa8
  FILE *v2;
  int v20; // stack - 0xa4
  char *v21; // stack - 0x98
  long v22; // stack - 0x90
  unsigned long v23; // stack - 0x88
  unsigned long v24; // stack - 0x80
  char *v25; // stack - 0x78
  char *v26; // stack - 0x70
  char *v27; // stack - 0x68
  long *v28; // stack - 0x60
  unsigned short v29; // stack - 0x26
  int v3; // eax
  unsigned int v30; // stack - 0x24
  unsigned long v4; // rax
  long *v5;
  unsigned char *v6; // rax
  char *v7;
  unsigned long v8; // rax
  unsigned long v9; // rax
  
  v21 = NULL;
  v1 = 0;
  v12 = -1;
  v13 = -1;
  v14 = -1;
  v15 = -1;
  v22 = -1;
  v23 = 0xffffffffffffffff;
label_7128:
  v29 = 0xffff;
  v11 = 0xffff;
  v3 = getopt_long(a0,a1,"abcdfghiklmnopqrstuvw:xABCDFGHI:LNQRST:UXZ1",(option *)0x2a0c0,(int *)&v11);
  if (v3 != -1) {
    switch(v3) {
      case 0x31:
        if (v12)
          v12 = 1;
        break;
      case 0x41:
        dat_2b3f4 = 1;
        break;
      case 0x42:
        sub_94e9("*~");
        sub_94e9(".*~");
        break;
      case 0x43:
        v12 = 2;
        break;
      case 0x44:
        dat_2b3cc = 1;
        break;
      case 0x46:
        if (optarg) // branch-flip
          v16 = *(int *)(sub_fe17("--classify",optarg,0x2a720,0x20980,4,dat_2b220,1) * 4 + 0x20980);
        else {
          v16 = 1;
        }
        if (v16 != 1) {
          if (v16 != 2) break;
          if (!sub_706a()) break;
        }
        dat_2b3d0 = 3;
        break;
      case 0x47:
        dat_2b023 = 0;
        break;
      case 0x48:
        dat_2b3ec = 2;
        break;
      case 0x49:
        sub_94e9(optarg);
        break;
      case 0x4c:
        dat_2b3ec = 4;
        break;
      case 0x4e:
        v14 = 0;
        break;
      case 0x51:
        v14 = 5;
        break;
      case 0x52:
        dat_2b3f0 = 1;
        break;
      case 0x53:
        v15 = 3;
        break;
      case 0x54:
        v7 = gettext("invalid tab size");
        v22 = sub_1cd03(optarg,0,0,0x7fffffffffffffff,0x1f27b,v7,2);
        break;
      case 0x55:
        v15 = 6;
        break;
      case 0x58:
        v15 = 1;
        break;
      case 0x5a:
        dat_2b380 = 1;
        break;
      case 0x61:
        dat_2b3f4 = 2;
        break;
      case 0x62:
        v14 = 7;
        break;
      case 99:
        dat_2b3b0 = 1;
        break;
      case 100:
        dat_2b3f1 = 1;
        break;
      case 0x66:
        dat_2b3f4 = 2;
        v15 = 6;
        if (!v12)
          v12 = -1;
        dat_2b3d4 = '\0';
        dat_2b3d5 = 0;
        dat_2b3bb = 0;
        break;
      case 0x67:
        v12 = 0;
        dat_2b022 = 0;
        break;
      case 0x68:
        dat_2b3bc = 0xb0;
        dat_2b3c8 = 0xb0;
        dat_2b3c0 = 1;
        dat_2b028 = 1;
        break;
      case 0x69:
        dat_2b3e9 = 1;
        break;
      case 0x6b:
        v1 = 1;
        break;
      case 0x6c:
        v12 = 0;
        break;
      case 0x6d:
        v12 = 4;
        break;
      case 0x6e:
        dat_2b3ba = 1;
        v12 = 0;
        break;
      case 0x6f:
        v12 = 0;
        dat_2b023 = 0;
        break;
      case 0x70:
        dat_2b3d0 = 1;
        break;
      case 0x71:
        v13 = 1;
        break;
      case 0x72:
        dat_2b3b8 = 1;
        break;
      case 0x73:
        dat_2b3bb = 1;
        break;
      case 0x74:
        v15 = 5;
        break;
      case 0x75:
        dat_2b3b0 = 2;
        break;
      case 0x76:
        v15 = 4;
        break;
      case 0x77:
        v23 = sub_6fe6(optarg);
        if (0 <= (long)v23) break;
        v4 = sub_1aeca(optarg);
        error(2,0,"%s: %s",gettext("invalid line width"),v4);
        goto label_73b3;
      case 0x78:
label_73b3:
        v12 = 3;
        break;
      case 0x80:
        dat_2b3b9 = 1;
        break;
      case 0x81:
        v3 = sub_149fd(optarg,(unsigned int *)0x2b3bc,(long *)0x2b3c0);
        if (v3)
          sub_1d130(v3,CONCAT22(v29,v11),0,0x2a0c0,optarg); // no-return
        dat_2b3c8 = dat_2b3bc;
        dat_2b028 = dat_2b3c0;
        break;
      case 0x82:
        if (optarg) // branch-flip
          v17 = *(int *)(sub_fe17("--color",optarg,0x2a720,0x20980,4,dat_2b220,1) * 4 + 0x20980);
        else {
          v17 = 1;
        }
        dat_2b3d4 = !((v17 != 1) && ((v17 != 2 || (!sub_706a()))));
        break;
      case 0x83:
        dat_2b3ec = 3;
        break;
      case 0x84:
        dat_2b3d0 = 2;
        break;
      case 0x85:
        v12 = *(int *)(sub_fe17("--format",optarg,0x2a660,0x208a0,4,dat_2b220,1) * 4 + 0x208a0);
        break;
      case 0x86:
        v12 = 0;
        v21 = (char *)sub_523e("full-iso");
        break;
      case 0x87:
        dat_2b3f2 = 1;
        break;
      case 0x88:
        v5 = (long *)sub_1c68b(0x10);
        *v5 = (long)optarg;
        v5[1] = (long)dat_2b400;
        dat_2b400 = v5;
        break;
      case 0x89:
        if (optarg) // branch-flip
          v18 = *(int *)(sub_fe17("--hyperlink",optarg,0x2a720,0x20980,4,dat_2b220,1) * 4 + 0x20980);
        else {
          v18 = 1;
        }
        dat_2b3d5 = !((v18 != 1) && ((v18 != 2 || (!sub_706a()))));
        break;
      case 0x8a:
        dat_2b3d0 = *(unsigned int *)(sub_fe17("--indicator-style",optarg,0x29fa0,0x205e0,4,dat_2b220,1) * 4 + 0x205e0);
        break;
      case 0x8b:
        v14 = *(int *)(sub_fe17("--quoting-style",optarg,0x2a9e0,0x238c0,4,dat_2b220,1) * 4 + 0x238c0);
        break;
      case 0x8c:
        v13 = 0;
        break;
      case 0x8d:
        dat_2b3bc = 0x90;
        dat_2b3c8 = 0x90;
        dat_2b3c0 = 1;
        dat_2b028 = 1;
        break;
      case 0x8e:
        v15 = *(int *)(sub_fe17("--sort",optarg,0x2a6a0,0x208d0,4,dat_2b220,1) * 4 + 0x208d0);
        break;
      case 0x8f:
        dat_2b3b0 = *(int *)(sub_fe17("--time",optarg,0x2a6e0,0x20920,4,dat_2b220,1) * 4 + 0x20920);
        break;
      case 0x90:
        v21 = optarg;
        break;
      case 0x91:
        goto label_7a55;
      case 0xffffff7d:
        if (dat_2b210 != 1) { // branch-flip
          v7 = (dat_2b210 != 2) ? "vdir" : "dir"; // branch-flip
        }
        else {
          v7 = "ls";
        }
        sub_1c4ab(stdout,v7,"GNU coreutils",dat_2b218,"Richard M. Stallman","David MacKenzie",0);
        exit(0); // no-return
      case 0xffffff7e:
        sub_f135(0); // no-return
      default:
        sub_f135(2); // no-return
      
    }
    goto label_7128;
  }
  if (!dat_2b3c0) {
    v6 = (unsigned char *)getenv("LS_BLOCK_SIZE");
    sub_149fd(v6,(unsigned int *)0x2b3bc,(long *)0x2b3c0);
    if ((v6) || (getenv("BLOCK_SIZE"))) {
      dat_2b3c8 = dat_2b3bc;
      dat_2b028 = dat_2b3c0;
    }
    if (v1) {
      dat_2b3bc = 0;
      dat_2b3c0 = 0x400;
    }
  }
  if (v12 <= -1) {
    if (dat_2b210 != 1) { // branch-flip
      v12 = (dat_2b210 != 2) ? 0 : 2; // branch-flip
    }
    else {
      v12 = (sub_706a()) ? 2 : 1; // branch-flip
    }
  }
  v24 = v23;
  dat_2b3ac = v12;
  if ((((v12 == 2) || (v12 == 3)) || (v12 == 4)) || (dat_2b3d4)) {
    if ((((long)v23 <= -1) && (sub_706a())) && ((0 <= ioctl(1,0x5413,&v11) && (v29))))
      v24 = (unsigned long)v29;
    if ((((long)v24 <= -1) && (v6 = (unsigned char *)getenv("COLUMNS"), v6)) && (*v6)) {
      v24 = sub_6fe6(v6);
      if ((long)v24 <= -1) {
        v7 = (char *)sub_1aeca(v6);
        error(0,0,gettext("ignoring invalid width in environment variable COLUMNS: %s"),v7);
      }
    }
  }
  if ((long)v24 < 0)
    v24 = 0x50;
  dat_2b4f8 = v24 / 3 + (unsigned long)(v24 % 3 != 0);
  dat_2b430 = v24;
  if (((dat_2b3ac == 2) || (dat_2b3ac == 3)) || (dat_2b3ac == 4)) {
    if (0 <= v22) // branch-flip
      dat_2b420 = v22;
    else {
      dat_2b420 = 8;
      v6 = (unsigned char *)getenv("TABSIZE");
      if (v6) {
        if (sub_1d251(v6,NULL,0,&v11,"")) { // branch-flip
          v7 = (char *)sub_1aeca(v6);
          error(0,0,gettext("ignoring invalid tab size in environment variable TABSIZE: %s"),v7);
        }
        else {
          dat_2b420 = CONCAT44(v30,CONCAT22(v29,v11));
        }
      }
    }
  }
  if (0 <= v13) // branch-flip
    dat_2b408 = v13 != 0;
  else {
    dat_2b408 = !((dat_2b210 != 1) || (!sub_706a()));
  }
  v19 = v14;
  if (v14 <= -1)
    v19 = sub_8cff();
  if (v19 <= -1) {
    if (dat_2b210 != 1) // branch-flip
      v19 = 7;
    else {
      v19 = (sub_706a()) ? 3 : -1; // branch-flip
    }
  }
  if (0 <= v19)
    sub_18fe7(NULL,v19);
  v3 = sub_18fc3(NULL);
  dat_2b341 = !(((dat_2b3ac) && (((dat_2b3ac != 2 && (dat_2b3ac != 3)) || (!dat_2b430)))) || (((v3 != 1 && (v3 != 3)) && (v3 != 6))));
  dat_2b410 = sub_18f74(NULL);
  if (v3 == 7)
    sub_19012(dat_2b410,0x20,1);
  if (2 <= dat_2b3d0) {
    for (v25 = &"*=>@|"[dat_2b3d0 - 2]; *v25; v25 = &v25[1]) {
      sub_19012(dat_2b410,(int)*v25,1);
    }
  }
  dat_2b418 = sub_18f74(NULL);
  sub_19012(dat_2b418,0x3a,1);
  dat_2b3cc = ((dat_2b3d5 ^ 1) & dat_2b3ac == 0 & dat_2b3cc) != 0;
  if ((int)dat_2b030 < (int)(unsigned int)dat_2b3cc) {
    error(2,0,gettext("--dired and --zero are incompatible"));
    return v8;
  }
  if (v15 <= -1) {
    if ((dat_2b3ac) && (((dat_2b3b0 == 1 || (dat_2b3b0 == 2)) || (dat_2b3b0 == 3))))
      v15 = 5;
    else {
      v15 = 0;
    }
  }
  dat_2b3b4 = v15;
  if (!dat_2b3ac) {
    v26 = v21;
    if (!v21) {
      v26 = getenv("TIME_STYLE");
      if (!v26)
        v26 = (char *)sub_523e("locale");
    }
    while (!strncmp(v26,"posix-",6)) {
      if (sub_11f2c(2) != '\x01') {
        return (unsigned long)optind; // return-dupe
      }
      v26 = &v26[6];
    }
    if (*v26 != '+') { // branch-flip
      v10 = sub_fa7e(v26,0x29f60,0x205b0,4);
      if (v10 <= -1) {
        sub_fc27("time style",v26,v10);
        v2 = stderr;
        fputs_unlocked(gettext("Valid arguments are:\n"),v2);
        v28 = (long *)0x29f60;
        while (v2 = stderr, *v28) {
          v5 = &v28[1];
          fprintf(stderr,"  - [posix-]%s\n",(char *)*v28);
          v28 = v5;
        }
        fputs_unlocked(gettext("  - +FORMAT (e.g., +%H:%M) for a \'date\'-style format\n"),v2);
        sub_f135(2); // no-return
      }
      switch(v10) { // branch-flip
        case 0:
          dat_2b208 = "%Y-%m-%d %H:%M:%S.%N %z";
          dat_2b200 = "%Y-%m-%d %H:%M:%S.%N %z";
          break;
        case 1:
          dat_2b208 = "%Y-%m-%d %H:%M";
          dat_2b200 = "%Y-%m-%d %H:%M";
          break;
        case 2:
          dat_2b200 = "%Y-%m-%d ";
          dat_2b208 = "%m-%d %H:%M";
          break;
        case 3:
          if (sub_11f2c(2)) {
            for (v20 = 0; v20 <= 1; v20 = v20 + 1) {
              v7 = *(char **)((long)v20 * 8 + 0x2b200);
              *(char **)((long)v20 * 8 + 0x2b200) = dcgettext(NULL,v7,2);
            }
          }
        
      }
    }
    else {
      v26 = &v26[1];
      v7 = strchr(v26,10);
      if (v7) { // branch-flip
        if (strchr(&v7[1],10)) {
          v7 = (char *)sub_1aeca(v26);
          error(2,0,gettext("invalid time style format %s"),v7);
          return v9;
        }
        v27 = &v7[1];
        *v7 = '\0';
      }
      else {
        v27 = v26;
      }
      dat_2b208 = v27;
      dat_2b200 = v26;
    }
    sub_5f43();
  }
  return (unsigned long)optind;
label_7a55:
  dat_2b030 = '\0';
  v13 = 0;
  if (v12)
    v12 = 1;
  dat_2b3d4 = '\0';
  v14 = 0;
  goto label_7128;
}


// Function: sub_842f @ 0x842f
unsigned long sub_842f(void)
{
  unsigned long v1; // rax
  
  v1 = (*(void *)((long)dat_211d8 + 0x211d8))(); // jump-as-call
  return v1;
}


// Function: sub_87ec @ 0x87ec
unsigned long sub_87ec(void) // return-dupe
{
  char *v1; // rax
  char *v2; // stack - 0x18
  
  v1 = getenv("TERM");
  if ((v1) && (*v1)) {
    v2 = "# Configuration file for dircolors, a utility to help you set the";
    while( true ) {
      if ((char *)0x12c8 <= &v2[-0x1f2c0])
        return 0;
      if ((!strncmp(v2,"TERM ",5)) && (!fnmatch(&v2[5],v1,0))) break;
      v2 = &v2[strlen(v2) + 1];
    }
    return 1;
  }
  return 0;
}


// Function: sub_88ab @ 0x88ab
void sub_88ab(void)
{
  char *v1; // rax
  char v2 [3];
  
  v1 = getenv("LS_COLORS");
  if ((v1) && (*v1)) {
    strcpy(v2,"??");
    dat_2b3e0 = sub_1cc92(v1);
    (*(void *)((long)dat_21390 + 0x2138c))(); // jump-as-call
    return;
  }
  v1 = getenv("COLORTERM");
  if (((!v1) || (!*v1)) && (sub_87ec() != '\x01'))
    dat_2b3d4 = 0;
}


// Function: sub_8cff @ 0x8cff
unsigned int sub_8cff(void) // early-return x2
{
  int v1; // eax
  char *v2; // rax
  
  v2 = getenv("QUOTING_STYLE");
  if (!v2)
    return 0xffffffff;
  v1 = sub_fa7e(v2,0x2a9e0,0x238c0,4);
  if (v1 <= -1) {
    v2 = (char *)sub_1aeca(v2);
    error(0,0,gettext("ignoring invalid value of environment variable QUOTING_STYLE: %s"),v2);
    return 0xffffffff;
  }
  return *(unsigned int *)((long)v1 * 4 + 0x238c0);
}


// Function: sub_8db1 @ 0x8db1
void sub_8db1(bool a0) // return-dupe
{
  if (a0) {
    dat_2b4e8 = 2;
    return;
  }
  if (!dat_2b4e8) {
    dat_2b4e8 = 1;
    return;
  }
}


// Function: sub_8de7 @ 0x8de7
void sub_8de7(char a0,char *a1,char *a2)
{
  unsigned long v1; // rax
  
  v1 = sub_1aa83(4,a2);
  error(0,*__errno_location(),a1,v1);
  sub_8db1((unsigned char)a0);
}


// Function: sub_8e46 @ 0x8e46
void sub_8e46(char *a0,char *a1,char a2)
{
  unsigned long *v1; // rax
  unsigned long v2; // rax
  
  v1 = (unsigned long *)sub_1c68b(0x20);
  if (a1) // branch-flip
    v2 = sub_1cc92(a1);
  else {
    v2 = 0;
  }
  v1[1] = v2;
  if (a0) // branch-flip
    v2 = sub_1cc92(a0);
  else {
    v2 = 0;
  }
  *v1 = v2;
  *(char *)&v1[2] = a2;
  v1[3] = dat_2b368;
  dat_2b368 = v1;
}


// Function: sub_8ede @ 0x8ede
void sub_8ede(char *a0,char *a1,char a2) // return-dupe
{
  char v1;
  unsigned long v10; // stack - 0x378
  char *v11; // stack - 0x370
  DIR *v12; // stack - 0x368
  unsigned long v13; // stack - 0x340
  int v2; // eax
  char *v3;
  dirent *v4; // rax
  char *v5; // rax
  unsigned long v6; // rax
  unsigned long v7; // stack - 0x348
  char v8 [663];
  unsigned int v9; // stack - 0x380
  
  v10 = 0;
  *__errno_location() = 0;
  v12 = opendir(a0);
  if (!v12) {
    sub_8de7((unsigned char)a2,gettext("cannot open directory %s"),a0);
    return;
  }
  if (dat_2b320) {
    v2 = dirfd(v12);
    if (0 <= v2) { // branch-flip
      sub_5cee(v2,&v7);
      v1 = dat_3;
    }
    else {
      sub_5cb8(a0,&v7);
      v1 = dat_3;
    }
    if (v1 <= '\xff') {
      sub_8de7((unsigned char)a2,gettext("cannot determine device and inode of %s"),a0);
      closedir(v12);
      return;
    }
    v1 = sub_6262(v7,v13);
    if (v1) {
      v3 = (char *)sub_1ac15(0,3,a0);
      error(0,0,gettext("%s: not listing already-listed directory"),v3);
      closedir(v12);
      sub_8db1(1);
      return;
    }
    sub_558e(v7,v13);
  }
  sub_96f3();
  if ((dat_2b3f0) || (dat_2b428)) {
    if (dat_2b032 != '\x01')
      sub_5422(10);
    dat_2b032 = '\0';
    sub_54cf();
    v11 = NULL;
    if (dat_2b3d5) {
      v11 = (char *)sub_1101b(a0,2);
      if (!v11)
        sub_8de7((unsigned char)a2,gettext("error canonicalizing %s"),a0);
    }
    if (a1) { // branch-flip
    }
    else {
      a1 = a0;
    }
    sub_d87f(a1,dat_2b418,0xffffffff,NULL,1,(obstack *)0x2b580,v11);
    free(v11);
    sub_549e(":\n");
  }
  do {
    *__errno_location() = 0;
    v4 = readdir(v12);
    if (v4) { // branch-flip
      if (sub_958d(&v4->field_0x13) != '\x01') {
        v9 = 0;
        switch(v4->field_0x12) {
          case 1:
            v9 = 1;
            break;
          case 2:
            v9 = 2;
            break;
          case 4:
            v9 = 3;
            break;
          case 6:
            v9 = 4;
            break;
          case 8:
            v9 = 5;
            break;
          case 10:
            v9 = 6;
            break;
          case 0xc:
            v9 = 7;
            break;
          case 0xe:
            v9 = 8;
          
        }
        v10 += sub_9a40(&v4->field_0x13,v9,0,0,a0);
        if ((((dat_2b3ac == 1) && (dat_2b3b4 == 6)) && (dat_2b3bb != '\x01')) && (dat_2b3f0 != '\x01')) {
          sub_bfc5();
          sub_c12d();
          sub_96f3();
        }
      }
    }
    else {
      if (!*__errno_location()) {
label_9382:
        if (closedir(v12))
          sub_8de7((unsigned char)a2,gettext("closing directory %s"),a0);
        sub_bfc5();
        if (dat_2b3f0)
          sub_aa65(a0,0);
        if ((!dat_2b3ac) || (dat_2b3bb)) {
          v5 = (char *)sub_13d91(v10,v8,dat_2b3bc,0x200,dat_2b3c0);
          v6 = strlen(v5);
          v3 = &v5[-1];
          *v3 = ' ';
          v5[v6] = dat_2b030;
          sub_54cf();
          sub_549e(gettext("total"));
          sub_5453(v3,&(&v5[v6])[1 - (long)v3]);
        }
        if (!dat_2b338)
          return;
        sub_c12d();
        return;
      }
      sub_8de7((unsigned char)a2,gettext("reading directory %s"),a0);
      if (*__errno_location() != 0x4b) goto label_9382;
    }
    sub_648c();
  } while( true );
}


// Function: sub_94e9 @ 0x94e9
void sub_94e9(unsigned long a0)
{
  unsigned long *v1; // rax
  
  v1 = (unsigned long *)sub_1c68b(0x10);
  *v1 = a0;
  v1[1] = dat_2b3f8;
  dat_2b3f8 = v1;
}


// Function: sub_952f @ 0x952f
unsigned long sub_952f(unsigned long *a0,char *a1)
{
  char *v1;
  unsigned long *v2; // stack - 0x10
  
  v2 = a0;
  while( true ) {
    if (!v2)
      return 0;
    v1 = (char *)*v2;
    if (!fnmatch(v1,a1,4)) break;
    v2 = (unsigned long *)v2[1];
  }
  return 1;
}


// Function: sub_958d @ 0x958d
unsigned int sub_958d(char *a0) // ternary
{
  char v1; // al
  long v2; // rdx
  
  if ((dat_2b3f4 != 2) && (*a0 == '.')) {
    if (!dat_2b3f4)
      return 1;
    v2 = (a0[1] != '.') ? 1 : 2; // branch-flip
    if (!a0[v2])
      return 1;
  }
  if (((dat_2b3f4) || (v1 = sub_952f(dat_2b400,a0), !v1)) && (v1 = sub_952f(dat_2b3f8,a0), !v1))
    return 0;
  return 1;
}


// Function: sub_9635 @ 0x9635
unsigned long sub_9635(unsigned long a0)
{
  return a0;
}


// Function: sub_9647 @ 0x9647
unsigned long sub_9647(void)
{
  *__errno_location() = 0x5f;
  return 0;
}


// Function: sub_9669 @ 0x9669
void sub_9669(struct_9 *a0) // return-dupe x2
{
  free(a0->field_0x0);
  free(a0->field_0x8);
  free(a0->field_0x10);
  if (a0->field_0xb0 == 0x2b020)
    return;
  if (sub_525c()) {
    free((void *)a0->field_0xb0);
    return;
  }
  freecon((char *)a0->field_0xb0);
}


// Function: sub_96f3 @ 0x96f3
void sub_96f3(void)
{
  unsigned long v1; // stack - 0x18
  
  for (v1 = 0; v1 < dat_2b338; v1 = v1 + 1) {
    sub_9669(*(struct_9 **)(v1 * 8 + dat_2b348));
  }
  dat_2b338 = 0;
  dat_2b340 = 0;
  dat_2b381 = 0;
  dat_2b384 = 0;
  dat_2b388 = 0;
  dat_2b38c = 0;
  dat_2b394 = 0;
  dat_2b398 = 0;
  dat_2b39c = 0;
  dat_2b390 = 0;
  dat_2b3a0 = 0;
  dat_2b3a4 = 0;
  dat_2b3a8 = 0;
}


// Function: sub_97c0 @ 0x97c0
unsigned int sub_97c0(int a0)
{
  if (((a0 != 0x16) && (a0 != 0x26)) && (!sub_524c(a0)))
    return 0;
  return 1;
}


// Function: sub_97fa @ 0x97fa
int sub_97fa(char *a0,long a1,bool a2) // early-return
{
  int v1;
  int v2; // eax
  
  if (*(long *)(a1 + 0x18) == dat_2c428) {
    *__errno_location() = 0x5f;
    return -1;
  }
  if (a2) // branch-flip
    v2 = sub_1b22b(a0,(char **)(a1 + 0xb0));
  else {
    v2 = sub_1b268(a0,(char **)(a1 + 0xb0));
  }
  if (v2 <= -1) {
    v1 = *__errno_location();
    if (sub_97c0(v1))
      dat_2c428 = *(long *)(a1 + 0x18);
  }
  return v2;
}


// Function: sub_98ab @ 0x98ab
int sub_98ab(char *a0,long a1) // early-return
{
  int v1;
  int v2; // eax
  
  if (*(long *)(a1 + 0x18) == dat_2c430) {
    *__errno_location() = 0x5f;
    return 0;
  }
  *__errno_location() = 0;
  v2 = sub_112c6(a0,a1 + 0x18);
  if (v2 <= 0) {
    v1 = *__errno_location();
    if (sub_97c0(v1))
      dat_2c430 = *(long *)(a1 + 0x18);
  }
  return v2;
}


// Function: sub_9936 @ 0x9936
char sub_9936(char *a0,long a1) // early-return
{
  int v1;
  char v2; // al
  
  if (*(long *)(a1 + 0x18) == dat_2c438) {
    *__errno_location() = 0x5f;
    return '\0';
  }
  v2 = sub_9647(a0);
  if (v2 != '\x01') {
    v1 = *__errno_location();
    if (sub_97c0(v1))
      dat_2c438 = *(long *)(a1 + 0x18);
  }
  return v2;
}


// Function: sub_99b1 @ 0x99b1
unsigned int sub_99b1(char *a0) // return-dupe
{
  unsigned long v1; // rax
  char v2 [2]; // stack - 0x12
  
  v1 = sub_1a389(v2,2,a0,0xffffffffffffffff,dat_2b410);
  if ((*a0 == v2[0]) && (v1 == strlen(a0)))
    return 0;
  return 1;
}


// Function: sub_9a40 @ 0x9a40
unsigned long sub_9a40(char *a0,int a1,long a2,char a3,char *a4)
{
  long v1;
  char *v10; // stack - 0x3e8
  char v11 [24];
  char v12 [664];
  char v13 [32];
  char **v14; // rsp
  char **v15;
  long v16; // stack - 0x3e0
  char v17; // stack - 0x3d8
  int v18; // stack - 0x3d4
  char *v19; // stack - 0x3d0
  unsigned char v2; // al
  char v20; // stack - 0x3c4
  char v21; // stack - 0x3c3
  char v22; // stack - 0x3c2
  char v23; // stack - 0x3c1
  unsigned int v24; // stack - 0x3c0
  int v25; // stack - 0x3bc
  unsigned int v26; // stack - 0x3b8
  int v27; // stack - 0x3b4
  int v28; // stack - 0x3b0
  int v29; // stack - 0x3ac
  char v3; // al
  int v30; // stack - 0x3a8
  int v31; // stack - 0x3a4
  int v32; // stack - 0x3a0
  int v33; // stack - 0x39c
  int v34; // stack - 0x398
  int v35; // stack - 0x394
  unsigned long v36; // stack - 0x390
  char *v37; // stack - 0x388
  unsigned long *v38; // stack - 0x380
  void *v39; // stack - 0x378
  int v4;
  unsigned long v40; // stack - 0x370
  unsigned int v41; // stack - 0x350
  unsigned int v5; // eax
  unsigned int v6;
  unsigned long v7;
  unsigned long v8;
  char *v9;
  
  v14 = &v10;
  v36 = 0;
  v10 = a4;
  v16 = a2;
  v17 = a3;
  v18 = a1;
  v19 = a0;
  if ((a3 == '\x01') && (a2))
    __assert_fail("! command_line_arg || inode == NOT_AN_INODE_NUMBER","src/ls.c",0xd07,"gobble_file"); // no-return
  if (dat_2b338 == dat_2b330) {
    dat_2b328 = sub_1c638(dat_2b328,dat_2b330,0x1a0);
    dat_2b330 *= 2;
  }
  v38 = (unsigned long *)(dat_2b338 * 0xd0 + dat_2b328);
  memset(v38,0,0xd0);
  v38[4] = v16;
  *(int *)&v38[0x15] = v18;
  *(unsigned int *)((long)v38 + 0xc4) = 0xffffffff;
  if ((dat_2b340 != '\x01') && (dat_2b341)) {
    v2 = sub_99b1(v19);
    *(unsigned int *)((long)v38 + 0xc4) = (unsigned int)v2;
    if (*(int *)((long)v38 + 0xc4))
      dat_2b340 = '\x01';
  }
  if (((((((!v17) && (!dat_2b3d5)) && (!dat_2b440)) && (((v18 != 3 || (!dat_2b3d4)) || ((!sub_6325(0x13) && ((!sub_6325(0x12) && (!sub_6325(0x14))))))))) && (((!dat_2b3e9 && (!dat_2b441)) || (((v18 != 6 && (v18)) || ((dat_2b3ec != 4 && ((!dat_2b358 && (!dat_2b3e8)))))))))) && ((!dat_2b3e9 || (v16)))) && ((v15 = &v10, !dat_2b441 || (((v18 && (!v17)) && ((v15 = &v10, v18 != 5 || ((dat_2b3d0 != 3 && ((v15 = &v10, !dat_2b3d4 || ((((!sub_6325(0xe) && (!sub_6325(0x10))) && (!sub_6325(0x11))) && (v15 = &v10, !sub_6325(0x15))))))))))))))) goto label_a7ca;
  if ((*v19 != '/') && (*v10)) {
    v7 = strlen(v19);
    v7 = ((strlen(v10) + v7 + 0x19) / 0x10) * 0x10;
    for (; v14 != (char **)((long)&v10 - (v7 & 0xfffffffffffff000)); v14 = (char *)((long)v14 + -0x1000)) {
      *(unsigned long *)((long)v14 + -8) = *(unsigned long *)((long)v14 + -8);
    }
    v1 = -(unsigned long)((unsigned int)v7 & 0xfff);
    v15 = (char *)((long)v14 + v1);
    if (v7 & 0xfff)
      *(unsigned long *)((long)v14 + ((unsigned long)((unsigned int)v7 & 0xfff) - 8) + v1) = *(unsigned long *)((long)v14 + ((unsigned long)((unsigned int)v7 & 0xfff) - 8) + v1);
    *(unsigned long *)((long)v14 + v1 + -8) = 0x9e47;
    v37 = (char *)((unsigned long)((long)v14 + v1 + 0xf) & 0xfffffffffffffff0);
    sub_eb94((char *)((unsigned long)((long)v14 + v1 + 0xf) & 0xfffffffffffffff0),v10,v19);
  }
  else {
    v37 = v19;
    v15 = &v10;
  }
  if (dat_2b3d5) {
    *(unsigned long *)((long)v15 + -8) = 0x9e66;
    v8 = sub_1101b(v37,2);
    v38[2] = v8;
    if (!v38[2]) {
      *(unsigned long *)((long)v15 + -8) = 0x9e93;
      v9 = gettext("error canonicalizing %s");
      *(unsigned long *)((long)v15 + -8) = 0x9eae;
      sub_8de7((unsigned char)v17,v9,v37);
    }
  }
  if (4 <= dat_2b3ec) { // branch-flip
    if (dat_2b3ec == 4) {
      *(unsigned long *)((long)v15 + -8) = 0x9ee9;
      v24 = sub_5c0e(v37,(struct_4 *)&v38[3]);
      v20 = 1;
      goto label_9fac;
    }
label_9f7c:
    *(unsigned long *)((long)v15 + -8) = 0x9f99;
    v24 = sub_5c48(v37,(struct_4 *)&v38[3]);
    v20 = 0;
  }
  else {
    if ((dat_2b3ec <= 1) || (!v17)) goto label_9f7c;
    *(unsigned long *)((long)v15 + -8) = 0x9f21;
    v24 = sub_5c0e(v37,(struct_4 *)&v38[3]);
    v20 = 1;
    if (dat_2b3ec != 2) {
      if (0 <= (int)v24) // branch-flip
        v23 = (*(unsigned int *)&v38[6] & 0xf000) != 0x4000;
      else {
        *(unsigned long *)((long)v15 + -8) = 0x9f47;
        v23 = *__errno_location() == 2;
      }
      if ((bool)v23 == 1) goto label_9f7c;
    }
  }
label_9fac:
  if (v24) {
    *(unsigned long *)((long)v15 + -8) = 0x9fc8;
    v9 = gettext("cannot access %s");
    *(unsigned long *)((long)v15 + -8) = 0x9fe3;
    sub_8de7((unsigned char)v17,v9,v37);
    v38[0x16] = 0x2b020;
    if (v17)
      return 0;
    *(unsigned long *)((long)v15 + -8) = 0xa01a;
    *v38 = sub_1cc92(v19);
    dat_2b338 += 1;
    return 0;
  }
  *(char *)&v38[0x17] = 1;
  if (((v18 == 5) || ((*(unsigned int *)&v38[6] & 0xf000) == 0x8000)) && (dat_2b3d4)) {
    *(unsigned long *)((long)v15 + -8) = 0xa085;
    if (sub_6325(0x15)) {
      *(unsigned long *)((long)v15 + -8) = 0xa0a2;
      v3 = sub_9936(v37,v38);
      *(char *)&v38[0x18] = v3;
    }
  }
  if ((!dat_2b3ac) || (dat_2b380)) {
    v21 = '\0';
    v22 = '\0';
    *(unsigned long *)((long)v15 + -8) = 0xa0f6;
    v4 = sub_97fa(v37,v38,(unsigned char)v20);
    v25 = v4;
    v5 = -(v4 >> 0x1f);
    v24 = v5;
    if (v5) { // branch-flip
      v38[0x16] = 0x2b020;
      *(unsigned long *)((long)v15 + -8) = 0xa194;
      v4 = *__errno_location();
      *(unsigned long *)((long)v15 + -8) = 0xa19d;
      if (!sub_524c(v4)) {
        *(unsigned long *)((long)v15 + -8) = 0xa1a6;
        if (*__errno_location() != 0x3d) goto label_a1b7;
      }
      v24 = 0;
    }
    else {
      *(unsigned long *)((long)v15 + -8) = 0xa11c;
      if (sub_525c()) { // branch-flip
        v9 = (char *)v38[0x16];
        *(unsigned long *)((long)v15 + -8) = 0xa140;
        v21 = strcmp("_",v9) != 0;
      }
      else {
        v9 = (char *)v38[0x16];
        *(unsigned long *)((long)v15 + -8) = 0xa16d;
        v21 = strcmp("unlabeled",v9) != 0;
      }
    }
label_a1b7:
    if ((!v24) && (!dat_2b3ac)) {
      *(unsigned long *)((long)v15 + -8) = 0xa1e3;
      v26 = sub_98ab(v37,v38);
      v24 = v26 >> 0x1f;
      v22 = 0 < (int)v26;
    }
    if ((v21) || (v22)) {
      if ((v21) && (v22 != '\x01'))
        v6 = 1;
      else {
        v6 = 2;
      }
    }
    else {
      v6 = 0;
    }
    *(unsigned int *)((long)v38 + 0xbc) = v6;
    dat_2b381 = *(int *)((long)v38 + 0xbc) || dat_2b381;
    if (v24) {
      *(unsigned long *)((long)v15 + -8) = 0xa2a2;
      v9 = (char *)sub_1ac15(0,3,v37);
      *(unsigned long *)((long)v15 + -8) = 0xa2aa;
      v4 = *__errno_location();
      *(unsigned long *)((long)v15 + -8) = 0xa2c7;
      error(0,v4,"%s",v9);
    }
  }
  if (((*(unsigned int *)&v38[6] & 0xf000) == 0xa000) && ((!dat_2b3ac || (dat_2b3e8)))) {
    *(unsigned long *)((long)v15 + -8) = 0xa31a;
    sub_a8f5(v37,v38,(unsigned char)v17);
    v9 = (char *)v38[1];
    *(unsigned long *)((long)v15 + -8) = 0xa337;
    v39 = (void *)sub_a964(v37,v9);
    if ((v39) && (!*(int *)((long)v38 + 0xc4))) {
      v9 = (char *)v38[1];
      *(unsigned long *)((long)v15 + -8) = 0xa36c;
      if (sub_99b1(v9))
        *(unsigned int *)((long)v38 + 0xc4) = 0xffffffff;
    }
    if ((v39) && ((2 <= dat_2b3d0 || (dat_2b3e8)))) {
      *(unsigned long *)((long)v15 + -8) = 0xa3ba;
      v4 = sub_5c82(v39,v11);
      if (!v4) {
        *(char *)((long)v38 + 0xb9) = 1;
        *(unsigned int *)((long)v38 + 0xac) = v41;
      }
    }
    *(unsigned long *)((long)v15 + -8) = 0xa3ee;
    free(v39);
  }
  if ((*(unsigned int *)&v38[6] & 0xf000) != 0xa000) { // branch-flip
    if ((*(unsigned int *)&v38[6] & 0xf000) != 0x4000) // branch-flip
      *(unsigned int *)&v38[0x15] = 5;
    else if ((v17) && (dat_2b3f1 != '\x01'))
      *(unsigned int *)&v38[0x15] = 9;
    else {
      *(unsigned int *)&v38[0x15] = 3;
    }
  }
  else {
    *(unsigned int *)&v38[0x15] = 6;
  }
  v8 = v38[0xb];
  v36 = v8;
  if ((!dat_2b3ac) || (dat_2b3bb)) {
    *(unsigned long *)((long)v15 + -8) = 0xa4cd;
    v9 = (char *)sub_13d91(v8,v12,dat_2b3bc,0x200,dat_2b3c0);
    *(unsigned long *)((long)v15 + -8) = 0xa4da;
    v27 = sub_1574e(v9,0);
    if (dat_2b388 < v27)
      dat_2b388 = v27;
  }
  if (!dat_2b3ac) {
    if (dat_2b022) {
      v6 = *(unsigned int *)((long)v38 + 0x34);
      *(unsigned long *)((long)v15 + -8) = 0xa524;
      v28 = sub_c5c3(v6);
      if (dat_2b394 < v28)
        dat_2b394 = v28;
    }
    if (dat_2b023) {
      v6 = *(unsigned int *)&v38[7];
      *(unsigned long *)((long)v15 + -8) = 0xa560;
      v29 = sub_c603(v6);
      if (dat_2b398 < v29)
        dat_2b398 = v29;
    }
    if (dat_2b3b9) {
      v6 = *(unsigned int *)((long)v38 + 0x34);
      *(unsigned long *)((long)v15 + -8) = 0xa59c;
      v30 = sub_c5c3(v6);
      if (dat_2b39c < v30)
        dat_2b39c = v30;
    }
  }
  if (dat_2b380) {
    v9 = (char *)v38[0x16];
    *(unsigned long *)((long)v15 + -8) = 0xa5dd;
    v31 = (int)strlen(v9);
    if (dat_2b390 < v31)
      dat_2b390 = v31;
  }
  if (!dat_2b3ac) {
    v7 = v38[5];
    *(unsigned long *)((long)v15 + -8) = 0xa628;
    v9 = (char *)sub_15035(v7,v13);
    *(unsigned long *)((long)v15 + -8) = 0xa630;
    v4 = (int)strlen(v9);
    if (dat_2b38c < v4)
      dat_2b38c = v4;
    v32 = v4;
    if (((*(unsigned int *)&v38[6] & 0xf000) != 0x2000) && ((*(unsigned int *)&v38[6] & 0xf000) != 0x6000)) {
      v8 = v38[9];
      *(unsigned long *)((long)v15 + -8) = 0xa768;
      v8 = sub_9635(v8);
      v40 = v8;
      *(unsigned long *)((long)v15 + -8) = 0xa79a;
      v9 = (char *)sub_13d91(v8,v12,dat_2b3c8,1,dat_2b028);
      *(unsigned long *)((long)v15 + -8) = 0xa7a7;
      v33 = sub_1574e(v9,0);
      if (dat_2b3a8 < v33)
        dat_2b3a8 = v33;
    }
    else {
      v8 = v38[8];
      *(unsigned long *)((long)v15 + -8) = 0xa693;
      v6 = gnu_dev_major(v8);
      *(unsigned long *)((long)v15 + -8) = 0xa6a7;
      v9 = (char *)sub_15035(v6,v12);
      *(unsigned long *)((long)v15 + -8) = 0xa6af;
      v34 = (int)strlen(v9);
      if (dat_2b3a0 < v34)
        dat_2b3a0 = v34;
      v8 = v38[8];
      *(unsigned long *)((long)v15 + -8) = 0xa6e2;
      v6 = gnu_dev_minor(v8);
      *(unsigned long *)((long)v15 + -8) = 0xa6f6;
      v9 = (char *)sub_15035(v6,v12);
      *(unsigned long *)((long)v15 + -8) = 0xa6fe;
      v4 = (int)strlen(v9);
      if (dat_2b3a4 < v4)
        dat_2b3a4 = v4;
      v34 = dat_2b3a4 + dat_2b3a0 + 2;
      if (dat_2b3a8 < v34)
        dat_2b3a8 = v34;
    }
  }
label_a7ca:
  if (dat_2b3e9) {
    v7 = v38[4];
    *(unsigned long *)((long)v15 + -8) = 0xa7f2;
    v9 = (char *)sub_15035(v7,v12);
    *(unsigned long *)((long)v15 + -8) = 0xa7fa;
    v35 = (int)strlen(v9);
    if (dat_2b384 < v35)
      dat_2b384 = v35;
  }
  *(unsigned long *)((long)v15 + -8) = 0xa829;
  *v38 = sub_1cc92(v19);
  dat_2b338 += 1;
  return v36;
}


// Function: sub_a869 @ 0xa869
unsigned int sub_a869(long a0) // return-dupe
{
  if ((*(int *)(a0 + 0xa8) != 3) && (*(int *)(a0 + 0xa8) != 9))
    return 0;
  return 1;
}


// Function: sub_a8a4 @ 0xa8a4
unsigned int sub_a8a4(long a0) // return-dupe
{
  if (((*(int *)(a0 + 0xa8) != 3) && (*(int *)(a0 + 0xa8) != 9)) && ((*(unsigned int *)(a0 + 0xac) & 0xf000) != 0x4000))
    return 0;
  return 1;
}


// Function: sub_a8f5 @ 0xa8f5
void sub_a8f5(char *a0,long a1,char a2) // return-dupe
{
  unsigned long v1;
  
  v1 = *(unsigned long *)(a1 + 0x48);
  *(unsigned long *)(a1 + 8) = sub_f7a0(a0,v1);
  if (*(long *)(a1 + 8))
    return;
  sub_8de7((unsigned char)a2,gettext("cannot read symbolic link %s"),a0);
}


// Function: sub_a964 @ 0xa964
char * sub_a964(char *a0,char *a1) // early-return
{
  char *v1; // rax
  unsigned long v2; // stack - 0x18
  
  if (!a1)
    return NULL;
  if (*a1 != '/') { // branch-flip
    v2 = sub_111a6(a0);
    if (v2) { // branch-flip
      v1 = (char *)sub_1c68b(strlen(a1) + v2 + 2);
      if (a0[v2 - 1] != '/')
        v2 += 1;
      stpcpy(stpncpy(v1,a0,v2),a1);
    }
    else {
      v1 = (char *)sub_1cc92(a1);
    }
  }
  else {
    v1 = (char *)sub_1cc92(a1);
  }
  return v1;
}


// Function: sub_aa37 @ 0xaa37
void sub_aa37(char *a0)
{
  sub_4ec1((char *)sub_ff56(a0));
}


// Function: sub_aa65 @ 0xaa65
void sub_aa65(char *a0,char a1)
{
  unsigned long v1;
  unsigned long *v2;
  long v3;
  char *v4;
  unsigned long v5; // stack - 0x30
  long v6; // stack - 0x28
  
  if ((a0) && (dat_2b320))
    sub_8e46(NULL,a0,0);
  v5 = dat_2b338;
label_aba6:
  do {
    v1 = v5 - 1;
    if (!v5) {
      v6 = 0;
      for (v5 = 0; v5 < dat_2b338; v5 = v5 + 1) {
        v3 = *(long *)(v5 * 8 + dat_2b348);
        *(long *)(dat_2b348 + v6 * 8) = v3;
        v6 += (unsigned long)(*(int *)(v3 + 0xa8) != 9);
      }
      dat_2b338 = v6;
      return;
    }
    v2 = *(unsigned long **)(v1 * 8 + dat_2b348);
    v5 = v1;
  } while (!sub_a869(v2));
  if (a0) goto label_aaf6;
  goto label_ab10;
label_aaf6:
  v4 = (char *)*v2;
  if (sub_aa37(v4) != '\x01') {
label_ab10:
    if ((a0) && (*(char *)*v2 != '/')) {
      v4 = (char *)sub_116a7(a0,(char *)*v2,NULL);
      sub_8e46(v4,(char *)v2[1],(unsigned char)a1);
      free(v4);
    }
    else {
      sub_8e46((char *)*v2,(char *)v2[1],(unsigned char)a1);
    }
    if (*(int *)&v2[0x15] == 9)
      sub_9669(v2);
  }
  goto label_aba6;
}


// Function: sub_ac36 @ 0xac36
int sub_ac36(char *a0,char *a1)
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // rax
  char *v4; // rax
  
  *__errno_location() = 0;
  v1 = strcoll(a0,a1);
  if (*__errno_location()) {
    v2 = sub_1ae9d(1,a1);
    v3 = sub_1ae9d(0,a0);
    v4 = gettext("cannot compare file names %s and %s");
    error(0,*__errno_location(),v4,v3,v2);
    sub_8db1(0);
    longjmp((void *)0x2c360,1); // no-return
  }
  return v1;
}


// Function: sub_ad02 @ 0xad02
unsigned long sub_ad02(long a0,long a1,void *a2)
{
  unsigned char v1; // al
  unsigned long v2; // rax
  unsigned int v3; // edx
  unsigned int v4; // ebx
  
  v1 = sub_a8a4(a1);
  v4 = (unsigned int)v1;
  v1 = sub_a8a4(a0);
  v3 = (unsigned int)v1;
  if (v4 - v3) // branch-flip
    v2 = (unsigned long)(v4 - v3);
  else {
    v2 = (*a2)(a0,a1);
  }
  return v2;
}


// Function: sub_ad65 @ 0xad65
unsigned long * sub_ad65(unsigned long *a0,unsigned long *a1,void *a2) // ternary
{
  long v1;
  unsigned int v2; // eax
  unsigned long *v3; // rax
  char v4 [16];
  char v5 [16];
  
  v3 = &a0[3];
  v4 = sub_1b477(v3);
  v3 = SUB168(v4,0);
  v1 = (long)v3;
  v3 = &a1[3];
  v5 = sub_1b477(v3);
  v3 = SUB168(v5,0);
  v2 = sub_1bc1f(v3,SUB168(v5,8),v1,SUB168(v4,8));
  v3 = (v2) ? (unsigned long)(unsigned long)v2 : (unsigned long)(*a2)(*a0,*a1); // branch-flip
  return v3;
}


// Function: sub_adf8 @ 0xadf8
unsigned long * sub_adf8(unsigned long *a0,unsigned long *a1,void *a2) // ternary
{
  long v1;
  unsigned int v2; // eax
  unsigned long *v3; // rax
  char v4 [16];
  char v5 [16];
  
  v3 = &a0[3];
  v4 = sub_1b491(v3);
  v3 = SUB168(v4,0);
  v1 = (long)v3;
  v3 = &a1[3];
  v5 = sub_1b491(v3);
  v3 = SUB168(v5,0);
  v2 = sub_1bc1f(v3,SUB168(v5,8),v1,SUB168(v4,8));
  v3 = (v2) ? (unsigned long)(unsigned long)v2 : (unsigned long)(*a2)(*a0,*a1); // branch-flip
  return v3;
}


// Function: sub_ae8b @ 0xae8b
unsigned long * sub_ae8b(unsigned long *a0,unsigned long *a1,void *a2) // ternary
{
  long v1;
  unsigned int v2; // eax
  unsigned long *v3; // rax
  char v4 [16];
  char v5 [16];
  
  v3 = &a0[3];
  v4 = sub_1b45d(v3);
  v3 = SUB168(v4,0);
  v1 = (long)v3;
  v3 = &a1[3];
  v5 = sub_1b45d(v3);
  v3 = SUB168(v5,0);
  v2 = sub_1bc1f(v3,SUB168(v5,8),v1,SUB168(v4,8));
  v3 = (v2) ? (unsigned long)(unsigned long)v2 : (unsigned long)(*a2)(*a0,*a1); // branch-flip
  return v3;
}


// Function: sub_af1e @ 0xaf1e
unsigned long * sub_af1e(unsigned long *a0,unsigned long *a1,void *a2) // ternary
{
  long v1;
  unsigned int v2; // eax
  unsigned long *v3; // rax
  char v4 [16];
  char v5 [16];
  
  v3 = &a0[3];
  v4 = sub_5990(v3);
  v3 = SUB168(v4,0);
  v1 = (long)v3;
  v3 = &a1[3];
  v5 = sub_5990(v3);
  v3 = SUB168(v5,0);
  v2 = sub_1bc1f(v3,SUB168(v5,8),v1,SUB168(v4,8));
  v3 = (v2) ? (unsigned long)(unsigned long)v2 : (unsigned long)(*a2)(*a0,*a1); // branch-flip
  return v3;
}


// Function: sub_afb1 @ 0xafb1
unsigned long sub_afb1(long a0,long a1) // early-return
{
  if (a1 <= a0)
    return (unsigned long)(a1 < a0);
  return 0xffffffff;
}


// Function: sub_afe2 @ 0xafe2
unsigned long sub_afe2(struct_15 *a0,struct_15 *a1,void *a2)
{
  unsigned int v1; // eax
  unsigned long v2; // rax
  
  v1 = sub_afb1(a1->field_0x48,a0->field_0x48);
  if (v1) // branch-flip
    v2 = (unsigned long)v1;
  else {
    v2 = (*a2)(a0->field_0x0,a1->field_0x0);
  }
  return v2;
}


// Function: sub_b03f @ 0xb03f
void sub_b03f(unsigned long *a0,unsigned long *a1,void *a2)
{
  (*a2)(*a0,*a1);
}


// Function: sub_b073 @ 0xb073
char * sub_b073(unsigned long *a0,unsigned long *a1,void *a2) // ternary
{
  char *v1;
  unsigned int v2; // eax
  char *v3; // rax
  char *v4;
  
  v3 = strrchr((char *)*a0,0x2e);
  v1 = v3;
  v3 = strrchr((char *)*a1,0x2e);
  if (v3) // branch-flip
    v4 = v3;
  else {
    v4 = "";
  }
  if (v1) // branch-flip
    v3 = v1;
  else {
    v3 = "";
  }
  v2 = (*a2)(v3,v4);
  v3 = (v2) ? (unsigned long)(unsigned long)v2 : (unsigned long)(*a2)(*a0,*a1); // branch-flip
  return v3;
}


// Function: sub_b119 @ 0xb119
unsigned long sub_b119(struct_13 *a0)
{
  unsigned long v1; // rax
  
  if (a0->field_0xc8) // branch-flip
    v1 = a0->field_0xc8;
  else {
    v1 = sub_d671(a0->field_0x0,dat_2b410,a0->field_0xc4);
  }
  return v1;
}


// Function: sub_b16b @ 0xb16b
unsigned long sub_b16b(struct_13 *a0,struct_13 *a1,void *a2)
{
  int v1;
  unsigned int v2; // eax
  unsigned long v3; // rax
  
  v2 = sub_b119(a0);
  v1 = v2;
  v2 = sub_b119(a1);
  v2 = v1 - v2;
  if (v2) // branch-flip
    v3 = (unsigned long)v2;
  else {
    v3 = (*a2)(a0->field_0x0,a1->field_0x0);
  }
  return v3;
}


// Function: sub_b1d2 @ 0xb1d2
void sub_b1d2(unsigned long *a0,unsigned long *a1)
{
  sub_ad65(a0,a1,sub_ac36);
}


// Function: sub_b202 @ 0xb202
void sub_b202(unsigned long *a0,unsigned long *a1)
{
  sub_ad65(a0,a1,dat_2afc0);
}


// Function: sub_b232 @ 0xb232
void sub_b232(unsigned long *a0,unsigned long *a1)
{
  sub_ad65(a1,a0,sub_ac36);
}


// Function: sub_b262 @ 0xb262
void sub_b262(unsigned long *a0,unsigned long *a1)
{
  sub_ad65(a1,a0,dat_2afc0);
}


// Function: sub_b292 @ 0xb292
void sub_b292(long a0,long a1)
{
  sub_ad02(a0,a1,sub_b1d2);
}


// Function: sub_b2c2 @ 0xb2c2
void sub_b2c2(long a0,long a1)
{
  sub_ad02(a0,a1,sub_b202);
}


// Function: sub_b2f2 @ 0xb2f2
void sub_b2f2(long a0,long a1)
{
  sub_ad02(a0,a1,sub_b232);
}


// Function: sub_b322 @ 0xb322
void sub_b322(long a0,long a1)
{
  sub_ad02(a0,a1,sub_b262);
}


// Function: sub_b352 @ 0xb352
void sub_b352(unsigned long *a0,unsigned long *a1)
{
  sub_adf8(a0,a1,sub_ac36);
}


// Function: sub_b382 @ 0xb382
void sub_b382(unsigned long *a0,unsigned long *a1)
{
  sub_adf8(a0,a1,dat_2afc0);
}


// Function: sub_b3b2 @ 0xb3b2
void sub_b3b2(unsigned long *a0,unsigned long *a1)
{
  sub_adf8(a1,a0,sub_ac36);
}


// Function: sub_b3e2 @ 0xb3e2
void sub_b3e2(unsigned long *a0,unsigned long *a1)
{
  sub_adf8(a1,a0,dat_2afc0);
}


// Function: sub_b412 @ 0xb412
void sub_b412(long a0,long a1)
{
  sub_ad02(a0,a1,sub_b352);
}


// Function: sub_b442 @ 0xb442
void sub_b442(long a0,long a1)
{
  sub_ad02(a0,a1,sub_b382);
}


// Function: sub_b472 @ 0xb472
void sub_b472(long a0,long a1)
{
  sub_ad02(a0,a1,sub_b3b2);
}


// Function: sub_b4a2 @ 0xb4a2
void sub_b4a2(long a0,long a1)
{
  sub_ad02(a0,a1,sub_b3e2);
}


// Function: sub_b4d2 @ 0xb4d2
void sub_b4d2(unsigned long *a0,unsigned long *a1)
{
  sub_ae8b(a0,a1,sub_ac36);
}


// Function: sub_b502 @ 0xb502
void sub_b502(unsigned long *a0,unsigned long *a1)
{
  sub_ae8b(a0,a1,dat_2afc0);
}


// Function: sub_b532 @ 0xb532
void sub_b532(unsigned long *a0,unsigned long *a1)
{
  sub_ae8b(a1,a0,sub_ac36);
}


// Function: sub_b562 @ 0xb562
void sub_b562(unsigned long *a0,unsigned long *a1)
{
  sub_ae8b(a1,a0,dat_2afc0);
}


// Function: sub_b592 @ 0xb592
void sub_b592(long a0,long a1)
{
  sub_ad02(a0,a1,sub_b4d2);
}


// Function: sub_b5c2 @ 0xb5c2
void sub_b5c2(long a0,long a1)
{
  sub_ad02(a0,a1,sub_b502);
}


// Function: sub_b5f2 @ 0xb5f2
void sub_b5f2(long a0,long a1)
{
  sub_ad02(a0,a1,sub_b532);
}


// Function: sub_b622 @ 0xb622
void sub_b622(long a0,long a1)
{
  sub_ad02(a0,a1,sub_b562);
}


// Function: sub_b652 @ 0xb652
void sub_b652(unsigned long *a0,unsigned long *a1)
{
  sub_af1e(a0,a1,sub_ac36);
}


// Function: sub_b682 @ 0xb682
void sub_b682(unsigned long *a0,unsigned long *a1)
{
  sub_af1e(a0,a1,dat_2afc0);
}


// Function: sub_b6b2 @ 0xb6b2
void sub_b6b2(unsigned long *a0,unsigned long *a1)
{
  sub_af1e(a1,a0,sub_ac36);
}


// Function: sub_b6e2 @ 0xb6e2
void sub_b6e2(unsigned long *a0,unsigned long *a1)
{
  sub_af1e(a1,a0,dat_2afc0);
}


// Function: sub_b712 @ 0xb712
void sub_b712(long a0,long a1)
{
  sub_ad02(a0,a1,sub_b652);
}


// Function: sub_b742 @ 0xb742
void sub_b742(long a0,long a1)
{
  sub_ad02(a0,a1,sub_b682);
}


// Function: sub_b772 @ 0xb772
void sub_b772(long a0,long a1)
{
  sub_ad02(a0,a1,sub_b6b2);
}


// Function: sub_b7a2 @ 0xb7a2
void sub_b7a2(long a0,long a1)
{
  sub_ad02(a0,a1,sub_b6e2);
}


// Function: sub_b7d2 @ 0xb7d2
void sub_b7d2(struct_15 *a0,struct_15 *a1)
{
  sub_afe2(a0,a1,sub_ac36);
}


// Function: sub_b802 @ 0xb802
void sub_b802(struct_15 *a0,struct_15 *a1)
{
  sub_afe2(a0,a1,dat_2afc0);
}


// Function: sub_b832 @ 0xb832
void sub_b832(struct_15 *a0,struct_15 *a1)
{
  sub_afe2(a1,a0,sub_ac36);
}


// Function: sub_b862 @ 0xb862
void sub_b862(struct_15 *a0,struct_15 *a1)
{
  sub_afe2(a1,a0,dat_2afc0);
}


// Function: sub_b892 @ 0xb892
void sub_b892(long a0,long a1)
{
  sub_ad02(a0,a1,sub_b7d2);
}


// Function: sub_b8c2 @ 0xb8c2
void sub_b8c2(long a0,long a1)
{
  sub_ad02(a0,a1,sub_b802);
}


// Function: sub_b8f2 @ 0xb8f2
void sub_b8f2(long a0,long a1)
{
  sub_ad02(a0,a1,sub_b832);
}


// Function: sub_b922 @ 0xb922
void sub_b922(long a0,long a1)
{
  sub_ad02(a0,a1,sub_b862);
}


// Function: sub_b952 @ 0xb952
void sub_b952(unsigned long *a0,unsigned long *a1)
{
  sub_b03f(a0,a1,sub_ac36);
}


// Function: sub_b982 @ 0xb982
void sub_b982(unsigned long *a0,unsigned long *a1)
{
  sub_b03f(a0,a1,dat_2afc0);
}


// Function: sub_b9b2 @ 0xb9b2
void sub_b9b2(unsigned long *a0,unsigned long *a1)
{
  sub_b03f(a1,a0,sub_ac36);
}


// Function: sub_b9e2 @ 0xb9e2
void sub_b9e2(unsigned long *a0,unsigned long *a1)
{
  sub_b03f(a1,a0,dat_2afc0);
}


// Function: sub_ba12 @ 0xba12
void sub_ba12(long a0,long a1)
{
  sub_ad02(a0,a1,sub_b952);
}


// Function: sub_ba42 @ 0xba42
void sub_ba42(long a0,long a1)
{
  sub_ad02(a0,a1,sub_b982);
}


// Function: sub_ba72 @ 0xba72
void sub_ba72(long a0,long a1)
{
  sub_ad02(a0,a1,sub_b9b2);
}


// Function: sub_baa2 @ 0xbaa2
void sub_baa2(long a0,long a1)
{
  sub_ad02(a0,a1,sub_b9e2);
}


// Function: sub_bad2 @ 0xbad2
void sub_bad2(unsigned long *a0,unsigned long *a1)
{
  sub_b073(a0,a1,sub_ac36);
}


// Function: sub_bb02 @ 0xbb02
void sub_bb02(unsigned long *a0,unsigned long *a1)
{
  sub_b073(a0,a1,dat_2afc0);
}


// Function: sub_bb32 @ 0xbb32
void sub_bb32(unsigned long *a0,unsigned long *a1)
{
  sub_b073(a1,a0,sub_ac36);
}


// Function: sub_bb62 @ 0xbb62
void sub_bb62(unsigned long *a0,unsigned long *a1)
{
  sub_b073(a1,a0,dat_2afc0);
}


// Function: sub_bb92 @ 0xbb92
void sub_bb92(long a0,long a1)
{
  sub_ad02(a0,a1,sub_bad2);
}


// Function: sub_bbc2 @ 0xbbc2
void sub_bbc2(long a0,long a1)
{
  sub_ad02(a0,a1,sub_bb02);
}


// Function: sub_bbf2 @ 0xbbf2
void sub_bbf2(long a0,long a1)
{
  sub_ad02(a0,a1,sub_bb32);
}


// Function: sub_bc22 @ 0xbc22
void sub_bc22(long a0,long a1)
{
  sub_ad02(a0,a1,sub_bb62);
}


// Function: sub_bc52 @ 0xbc52
void sub_bc52(struct_13 *a0,struct_13 *a1)
{
  sub_b16b(a0,a1,sub_ac36);
}


// Function: sub_bc82 @ 0xbc82
void sub_bc82(struct_13 *a0,struct_13 *a1)
{
  sub_b16b(a0,a1,dat_2afc0);
}


// Function: sub_bcb2 @ 0xbcb2
void sub_bcb2(struct_13 *a0,struct_13 *a1)
{
  sub_b16b(a1,a0,sub_ac36);
}


// Function: sub_bce2 @ 0xbce2
void sub_bce2(struct_13 *a0,struct_13 *a1)
{
  sub_b16b(a1,a0,dat_2afc0);
}


// Function: sub_bd12 @ 0xbd12
void sub_bd12(long a0,long a1)
{
  sub_ad02(a0,a1,sub_bc52);
}


// Function: sub_bd42 @ 0xbd42
void sub_bd42(long a0,long a1)
{
  sub_ad02(a0,a1,sub_bc82);
}


// Function: sub_bd72 @ 0xbd72
void sub_bd72(long a0,long a1)
{
  sub_ad02(a0,a1,sub_bcb2);
}


// Function: sub_bda2 @ 0xbda2
void sub_bda2(long a0,long a1)
{
  sub_ad02(a0,a1,sub_bce2);
}


// Function: sub_bdd2 @ 0xbdd2
unsigned long sub_bdd2(unsigned long *a0,unsigned long *a1)
{
  unsigned int v1; // eax
  unsigned long v2; // rax
  
  v1 = sub_11c04((char *)*a0,(char *)*a1);
  if (v1) // branch-flip
    v2 = (unsigned long)v1;
  else {
    v1 = strcmp((char *)*a0,(char *)*a1);
  }
  return v2;
}


// Function: sub_be28 @ 0xbe28
void sub_be28(unsigned long *a0,unsigned long *a1)
{
  sub_bdd2(a0,a1);
}


// Function: sub_be51 @ 0xbe51
void sub_be51(unsigned long *a0,unsigned long *a1)
{
  sub_bdd2(a1,a0);
}


// Function: sub_be7a @ 0xbe7a
void sub_be7a(long a0,long a1)
{
  sub_ad02(a0,a1,sub_be28);
}


// Function: sub_beaa @ 0xbeaa
void sub_beaa(long a0,long a1)
{
  sub_ad02(a0,a1,sub_be51);
}


// Function: sub_beda @ 0xbeda
void sub_beda(void)
{
  unsigned long v1; // stack - 0x10
  
  for (v1 = 0; v1 < dat_2b338; v1 = v1 + 1) {
    *(unsigned long *)(v1 * 8 + dat_2b348) = dat_2b328 + v1 * 0xd0;
  }
}


// Function: sub_bf3d @ 0xbf3d
void sub_bf3d(void)
{
  struct_13 *v1;
  unsigned long v2; // stack - 0x18
  
  if ((dat_2b3b4 == 2) || ((dat_2b430 && ((dat_2b3ac == 2 || (dat_2b3ac == 3)))))) {
    for (v2 = 0; v2 < dat_2b338; v2 = v2 + 1) {
      v1 = *(struct_13 **)(v2 * 8 + dat_2b348);
      v1->field_0xc8 = sub_b119(v1);
    }
  }
}


// Function: sub_bfc5 @ 0xbfc5
void sub_bfc5(void) // return-dupe
{
  unsigned int v1;
  unsigned char v2; // stack - 0x9
  
  if (dat_2b350 < (dat_2b338 >> 1) + dat_2b338) {
    free(dat_2b348);
    dat_2b348 = (void *)sub_1c804(dat_2b338,0x18);
    dat_2b350 = dat_2b338 * 3;
  }
  sub_beda();
  sub_bf3d();
  if (dat_2b3b4 == 6)
    return;
  if (_setjmp((void *)0x2c360)) { // branch-flip
    v2 = 1;
    if (dat_2b3b4 == 4)
      __assert_fail("sort_type != sort_version","src/ls.c",0x1008,"sort_files"); // no-return
    sub_beda();
  }
  else {
    v2 = 0;
  }
  if (dat_2b3b4 != 5) // branch-flip
    v1 = 0;
  else {
    v1 = dat_2b3b0;
  }
  sub_15dfc(dat_2b348,dat_2b338,*(void **)((((long)(int)(unsigned int)dat_2b3b8 + ((long)(int)(unsigned int)v2 + (unsigned long)(unsigned int)(v1 + dat_2b3b4) * 2) * 2) * 2 + (long)(int)(unsigned int)dat_2b3f2) * 8 + 0x2a780));
}


// Function: sub_c12d @ 0xc12d
void sub_c12d(void) // return-dupe x7
{
  unsigned long v1; // stack - 0x10
  
  switch(dat_2b3ac) {
    case 0:
      for (v1 = 0; v1 < dat_2b338; v1 = v1 + 1) {
        sub_63f3();
        sub_c6c8(*(unsigned long *)(v1 * 8 + dat_2b348));
        sub_5422((int)dat_2b030);
      }
      return;
    case 1:
      for (v1 = 0; v1 < dat_2b338; v1 = v1 + 1) {
        sub_dd38(*(unsigned long *)(v1 * 8 + dat_2b348),0);
        putchar_unlocked((int)dat_2b030);
      }
      return;
    case 2:
      goto label_c1bf;
    case 3:
      if (!dat_2b430) {
        sub_e9d6(0x20);
        return;
      }
      sub_e86a();
      return;
    case 4:
      sub_e9d6(0x2c);
      return;
    default:
      return;
    
  }
label_c1bf:
  if (!dat_2b430) {
    sub_e9d6(0x20);
    return;
  }
  sub_e708();
}


// Function: sub_c25f @ 0xc25f
void sub_c25f(char *a0,long a1,unsigned char a2,tm *a3,unsigned long a4,unsigned int a5)
{
  char *v1;
  
  if (dat_2c340) // branch-flip
    v1 = (char *)(((long)(int)(unsigned int)a2 * 0xc + (long)*(int *)&a3->field_0x10) * 0x80 + 0x2b740);
  else {
    v1 = *(char **)((long)(int)(unsigned int)a2 * 8 + 0x2b200);
  }
  sub_15f45(a0,a1,v1,a3,a4,a5);
}


// Function: sub_c308 @ 0xc308
int sub_c308(void)
{
  char v1 [64];
  unsigned long v2; // stack - 0x458
  char v3 [1016];
  long v4; // stack - 0x450
  
  if (dat_2b034 <= -1) {
    v2 = 0;
    if ((sub_1b9a5(dat_2b438,&v2,v1)) && (v4 = sub_c25f(v3,0x3e9,0,v1,dat_2b438,0), v4))
      dat_2b034 = sub_15784(v3,v4,0);
    if (dat_2b034 <= -1)
      dat_2b034 = 0;
  }
  return dat_2b034;
}


// Function: sub_c3fc @ 0xc3fc
void sub_c3fc(char *a0,unsigned long a1,unsigned int a2) // return-dupe
{
  int v1;
  bool v2; // zf
  int v3; // stack - 0x10
  
  if (a0) {
    v3 = a2 - sub_1574e(a0,0);
    if (v3 <= -1)
      v3 = 0;
    sub_549e(a0);
    do {
      sub_5422(0x20);
      v1 = v3 + -1;
      v2 = v3 != 0;
      v3 = v1;
    } while (v2);
    return;
  }
  dat_2b500 += printf("%*lu ",(unsigned long)a2,a1);
}


// Function: sub_c49e @ 0xc49e
void sub_c49e(unsigned int a0,unsigned int a1,char a2) // ternary
{
  char *v1; // rax
  
  if (a2 != '\x01') // branch-flip
    v1 = "?";
  else {
    v1 = (dat_2b3ba) ? NULL : (char *)sub_14a53(a0); // branch-flip
  }
  sub_c3fc(v1,a0,a1);
}


// Function: sub_c4fe @ 0xc4fe
void sub_c4fe(unsigned int a0,unsigned int a1,char a2) // ternary
{
  char *v1; // rax
  
  if (a2 != '\x01') // branch-flip
    v1 = "?";
  else {
    v1 = (dat_2b3ba) ? NULL : (char *)sub_14ca5(a0); // branch-flip
  }
  sub_c3fc(v1,a0,a1);
}


// Function: sub_c55e @ 0xc55e
unsigned long sub_c55e(char *a0,unsigned long a1) // early-return
{
  unsigned int v1; // eax
  unsigned long v2; // rax
  bool v3; // sf
  
  if (a0) { // branch-flip
    v1 = sub_1574e(a0,0);
    v3 = (int)v1 <= -1;
    v2 = (unsigned long)v1;
    if (v3) {
      v2 = 0;
      return v2;
    }
  }
  else {
    v1 = snprintf(NULL,0,"%lu",a1);
  }
  return v2;
}


// Function: sub_c5c3 @ 0xc5c3
void sub_c5c3(unsigned int a0) // ternary
{
  char *v1; // rax
  
  v1 = (dat_2b3ba) ? NULL : (char *)sub_14a53(a0); // branch-flip
  sub_c55e(v1,a0);
}


// Function: sub_c603 @ 0xc603
void sub_c603(unsigned int a0) // ternary
{
  char *v1; // rax
  
  v1 = (dat_2b3ba) ? NULL : (char *)sub_14ca5(a0); // branch-flip
  sub_c55e(v1,a0);
}


// Function: sub_c643 @ 0xc643
char * sub_c643(long a0,unsigned long a1,long a2) // early-return
{
  unsigned long v1;
  
  if (a1 <= 0x14)
    __assert_fail("INT_BUFSIZE_BOUND (uintmax_t) <= buflen","src/ls.c",0x10be,"format_inode"); // no-return
  if ((*(char *)(a2 + 0xb8)) && (*(long *)(a2 + 0x20))) {
    v1 = *(unsigned long *)(a2 + 0x20);
    return (char *)sub_15035(v1,a0);
  }
  return "?";
}


// Function: sub_c6c8 @ 0xc6c8
void sub_c6c8(long a0) // return-dupe x4, ternary x2, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_c6c8
{
  unsigned long v1;
  long v10; // rax
  int v11;
  char v12; // stack - 0x1124
  char v13 [9];
  char v14 [3656];
  char v15 [656];
  char v16 [32];
  char v17 [68];
  long v18; // stack - 0x1188
  unsigned long v19; // r9
  unsigned long v2;
  char v20 [16];
  int v21; // stack - 0x11bc
  int v22; // stack - 0x11b8
  long v23; // stack - 0x11b0
  char *v24; // stack - 0x11a8
  char *v25; // stack - 0x11a0
  char *v26; // stack - 0x1198
  unsigned long v27; // stack - 0x1180
  long v28; // stack - 0x1178
  unsigned long v29; // stack - 0x1170
  bool v3;
  char v30; // stack - 0x111a
  char v31; // stack - 0x1119
  unsigned int v4;
  char v5; // al
  int v6; // eax
  int v7;
  char *v8;
  char *v9;
  
  v3 = 1;
  if (*(char *)(a0 + 0xb8)) // branch-flip
    sub_1167b(a0 + 0x18,&v12);
  else {
    v12 = "?pcdb-lswd"[*(unsigned int *)(a0 + 0xa8)];
    memset(v13,0x3f,10);
    v31 = 0;
  }
  if (dat_2b381 != '\x01') // branch-flip
    v30 = 0;
  else if (*(int *)(a0 + 0xbc) != 1) { // branch-flip
    if (*(int *)(a0 + 0xbc) == 2)
      v30 = 0x2b;
  }
  else {
    v30 = 0x2e;
  }
  switch(dat_2b3b0) { // branch-flip
    case 0:
      v20 = sub_1b491(a0 + 0x18);
      v27 = SUB168(v20,8);
      v18 = SUB168(v20,0);
      break;
    default:
      abort(); // no-return, return-dupe
    case 1:
      v20 = sub_1b477(a0 + 0x18);
      v27 = SUB168(v20,8);
      v18 = SUB168(v20,0);
      break;
    case 2:
      v20 = sub_1b45d(a0 + 0x18);
      v27 = SUB168(v20,8);
      v18 = SUB168(v20,0);
      break;
    case 3:
      v20 = sub_5990(a0 + 0x18);
      v27 = SUB168(v20,8);
      v18 = SUB168(v20,0);
      if ((v18 == -1) && (v27 == 0xffffffffffffffff))
        v3 = 0;
    
  }
  v24 = v14;
  if (dat_2b3e9) {
    v8 = (char *)sub_c643(v15,0x15,a0);
    v24 = &v24[sprintf(v24,"%*s ",dat_2b384,v8)];
  }
  if (dat_2b3bb) {
    v25 = (*(char *)(a0 + 0xb8) != '\x01') ? "?" : (char *)sub_13d91(*(unsigned long *)(a0 + 0x58),v15,dat_2b3bc,0x200,dat_2b3c0); // branch-flip
    v21 = dat_2b388;
    v21 -= sub_1574e(v25,0);
    while (v8 = v24, 0 < v21) {
      v8 = &v24[1];
      *v24 = ' ';
      v21 -= 1;
      v24 = v8;
    }
    do {
      v24 = v8;
      v9 = &v25[1];
      v8 = &v24[1];
      *v24 = *v25;
      v25 = v9;
    } while (*v24);
    *v24 = ' ';
    v24 = v8;
  }
  v8 = (*(char *)(a0 + 0xb8) != '\x01') ? "?" : (char *)sub_15035(*(unsigned long *)(a0 + 0x28),v15); // branch-flip
  v24 = &v24[sprintf(v24,"%s %*s ",&v12,dat_2b38c,v8)];
  sub_54cf();
  if ((((dat_2b022) || (dat_2b023)) || (dat_2b3b9)) || (dat_2b380)) {
    sub_5453(v14,(long)v24 - (long)v14);
    if (dat_2b022)
      sub_c49e(*(unsigned int *)(a0 + 0x34),dat_2b394,(unsigned char)*(char *)(a0 + 0xb8));
    if (dat_2b023)
      sub_c4fe(*(unsigned int *)(a0 + 0x38),dat_2b398,(unsigned char)*(char *)(a0 + 0xb8));
    if (dat_2b3b9)
      sub_c49e(*(unsigned int *)(a0 + 0x34),dat_2b39c,(unsigned char)*(char *)(a0 + 0xb8));
    if (dat_2b380)
      sub_c3fc(*(char **)(a0 + 0xb0),0,dat_2b390);
    v24 = v14;
  }
  v4 = dat_2b3c8;
  v1 = dat_2b028;
  if ((*(char *)(a0 + 0xb8)) && (((*(unsigned int *)(a0 + 0x30) & 0xf000) == 0x2000 || ((*(unsigned int *)(a0 + 0x30) & 0xf000) == 0x6000)))) {
    v6 = dat_2b3a8 - (dat_2b3a4 + dat_2b3a0 + 2);
    v1 = *(unsigned long *)(a0 + 0x40);
    v8 = (char *)sub_15035(gnu_dev_minor(v1),v15);
    v7 = dat_2b3a4;
    v1 = *(unsigned long *)(a0 + 0x40);
    v9 = (char *)sub_15035(gnu_dev_major(v1),v16);
    v11 = 0;
    if (0 <= v6)
      v11 = v6;
    v11 += dat_2b3a0;
    v24 = &v24[sprintf(v24,"%*s, %*s ",v11,v9,v7,v8)];
  }
  else {
    if (*(char *)(a0 + 0xb8) != '\x01') // branch-flip
      v26 = "?";
    else {
      v2 = *(unsigned long *)(a0 + 0x48);
      v26 = (char *)sub_13d91(sub_9635(v2),v15,v4,1,v1);
    }
    v22 = dat_2b3a8;
    v22 -= sub_1574e(v26,0);
    while (v8 = v24, 0 < v22) {
      v8 = &v24[1];
      *v24 = ' ';
      v22 -= 1;
      v24 = v8;
    }
    do {
      v24 = v8;
      v9 = &v26[1];
      v8 = &v24[1];
      *v24 = *v26;
      v26 = v9;
    } while (*v24);
    *v24 = ' ';
    v24 = v8;
  }
  v23 = 0;
  *v24 = '\x01';
  if (((*(char *)(a0 + 0xb8)) && (v3)) && (sub_1b9a5(dat_2b438,&v18,v17))) {
    v7 = sub_1bc1f(dat_2b370,dat_2b378,v18,v27);
    if (v7 <= -1)
      sub_11ec3((timespec *)0x2b370);
    v28 = dat_2b370 + -0xf0c2ac;
    v29 = dat_2b378;
    v7 = sub_1bc1f(v28,dat_2b378,v18,v27);
    v5 = !((0 <= v7) || (v7 = sub_1bc1f(v18,v27,dat_2b370,dat_2b378), 0 <= v7));
    v19 = v27 & 0xffffffff;
    v23 = sub_c25f(v24,0x3e9,(unsigned char)v5,v17,dat_2b438,v19);
  }
  if ((v23) || (!*v24)) {
    v8 = &v24[v23];
    v24 = &v8[1];
    *v8 = ' ';
  }
  else {
    if ((!*(char *)(a0 + 0xb8)) || (!v3))
      v8 = "?";
    else {
      v8 = (char *)sub_5219(v18,v15);
    }
    v7 = sub_c308();
    v24 = &v24[sprintf(v24,"%*s ",v7,v8)];
  }
  sub_5453(v14,(long)v24 - (long)v14);
  v10 = sub_db96(a0,0,(obstack *)0x2b520,(long)v24 - (long)v14);
  if (*(int *)(a0 + 0xa8) != 6) {
    if (!dat_2b3d0)
      return;
    sub_e05c((unsigned char)*(char *)(a0 + 0xb8),*(unsigned int *)(a0 + 0x30),*(unsigned int *)(a0 + 0xa8));
    return;
  }
  if (!*(long *)(a0 + 8))
    return;
  sub_549e(" -> ");
  sub_db96(a0,1,NULL,&v24[(v10 - (long)v14) + 4]);
  if (!dat_2b3d0)
    return;
  sub_e05c(1,*(unsigned int *)(a0 + 0xac),0);
}


// Function: sub_d10c @ 0xd10c
unsigned long sub_d10c(unsigned long *a0,unsigned long a1,char *a2,unsigned int *a3,int a4,unsigned long *a5,char *a6) // warn: iteboolean: re-rolled 3 0/1 select diamond(s) to a boolean assignment in sub_d10c
{
  char *v1;
  unsigned long v10; // rax
  mbstate_t v11; // stack - 0x28
  int v12; // stack - 0x8c
  int v13; // stack - 0x88
  int v14; // stack - 0x84
  char *v15; // stack - 0x80
  unsigned long v16; // stack - 0x78
  unsigned long v17; // stack - 0x70
  char *v18; // stack - 0x68
  char *v19; // stack - 0x60
  char *v2;
  long v20; // stack - 0x58
  char *v21; // stack - 0x50
  char *v22; // stack - 0x48
  char *v23; // stack - 0x40
  char *v24; // stack - 0x38
  char *v25; // stack - 0x30
  char v3;
  long v4;
  char *v5;
  bool v6; // al
  bool v7;
  char v8; // al
  int v9; // eax
  
  v5 = a6;
  v15 = (char *)*a0;
  v17 = 0;
  v13 = sub_18fc3(a3);
  v6 = ((dat_2b408) && (((v13 == 1 || (v13 == 2)) || (!v13))));
  if (a4) { // branch-flip
    v17 = sub_1a389(v15,a1,a2,0xffffffffffffffff,a3);
    if (a1 <= v17) {
      v15 = (char *)sub_1c68b(v17 + 1);
      sub_1a389(v15,v17 + 1,a2,0xffffffffffffffff,a3);
    }
    v7 = ((*a2 != *v15) || (v17 != strlen(a2)));
  }
  else if (v6) {
    v17 = strlen(a2);
    if (a1 <= v17) {
      v10 = v17 + 1;
      v15 = (char *)sub_1c68b(v10);
    }
    memcpy(v15,a2,v17 + 1);
    v7 = 0;
  }
  else {
    v17 = strlen(a2);
    v7 = 0;
    v15 = a2;
  }
  if (v6) {
    if (2 <= __ctype_get_mb_cur_max()) { // branch-flip
      v18 = v15;
      v25 = &v15[v17];
      v19 = v15;
      v16 = 0;
label_d4c6:
      if (v18 < v25) {
        v9 = (int)*v18;
        if (0x7f <= v9) { // branch-flip
label_d3b9:
          v11 = 0;
          do {
            v20 = sub_150be(&v12,v18,(long)v25 - (long)v18,&v11);
            if (v20 == -1) {
              v18 = &v18[1];
              v1 = &v19[1];
              *v19 = '?';
              v16 += 1;
              v19 = v1;
              break;
            }
            if (v20 == -2) {
              v18 = v25;
              v1 = &v19[1];
              *v19 = '?';
              v16 += 1;
              v19 = v1;
              break;
            }
            if (!v20)
              v20 = 1;
            v14 = wcwidth(v12);
            if (0 <= v14) { // branch-flip
              while (v20) {
                v1 = &v18[1];
                v2 = &v19[1];
                *v19 = *v18;
                v20 -= 1;
                v18 = v1;
                v19 = v2;
              }
              v16 += (long)v14;
            }
            else {
              v18 = &v18[v20];
              v1 = &v19[1];
              *v19 = '?';
              v16 += 1;
              v19 = v1;
            }
          } while (!mbsinit(&v11));
        }
        else {
          if (v9 <= 0x60) {
            if (0x60 > v9) {
              if (v9 > 0x40) goto label_d392;
              if (0x24 <= v9) { // branch-flip
                if (0x1b > (unsigned int)(v9 - 0x25U)) goto label_d392;
              }
              else if (v9 > 0x1f) goto label_d392;
            }
            goto label_d3b9;
          }
label_d392:
          v1 = &v18[1];
          v2 = &v19[1];
          *v19 = *v18;
          v16 += 1;
          v18 = v1;
          v19 = v2;
        }
        goto label_d4c6;
      }
      v17 = (long)v19 - (long)v15;
    }
    else {
      v24 = &v15[v17];
      for (v21 = v15; v21 < v24; v21 = &v21[1]) {
        v4 = *(long *)__ctype_b_loc();
        v3 = *v21;
        if (!(*(unsigned short *)((unsigned long)(unsigned char)sub_4eb2((int)v3) * 2 + v4) & 0x4000))
          *v21 = '?';
      }
      v16 = v17;
    }
  }
  else if (a5) {
    if (2 <= __ctype_get_mb_cur_max()) { // branch-flip
      v9 = sub_15784(v15,v17,0);
      v16 = (unsigned long)v9;
    }
    else {
      v23 = &v15[v17];
      v16 = 0;
      for (v22 = v15; v22 < v23; v22 = &v22[1]) {
        v4 = *(long *)__ctype_b_loc();
        v3 = *v22;
        if (*(unsigned short *)((unsigned long)(unsigned char)sub_4eb2((int)v3) * 2 + v4) & 0x4000)
          v16 += 1;
      }
    }
  }
  v8 = !(((!dat_2b341) || (!dat_2b340)) || (v7));
  *v5 = v8;
  if (a5)
    *a5 = v16;
  *a0 = v15;
  return v17;
}


// Function: sub_d671 @ 0xd671
long sub_d671(char *a0,unsigned int *a1,unsigned int a2)
{
  char v1 [8200];
  long v2; // stack - 0x2020
  char *v3; // stack - 0x2028
  unsigned char v4; // stack - 0x2029
  
  v3 = v1;
  sub_d10c(&v3,0x2000,a0,a1,a2,&v2,&v4);
  if ((v3 != v1) && (a0 != v3))
    free(v3);
  return v2 + (unsigned long)v4;
}


// Function: sub_d775 @ 0xd775
char * sub_d775(char *a0,bool a1)
{
  char *v1;
  char *v2;
  char v3;
  unsigned char v4; // al
  char *v5; // rax
  char *v6; // stack - 0x20
  char *v7; // stack - 0x18
  
  v5 = (char *)sub_1c804(3,strlen(a0) + 1);
  v7 = v5;
  v6 = a0;
  while (*v6) {
    if ((a1) && (*v6 == '/')) {
      v1 = &v7[1];
      *v7 = '/';
      v6 = &v6[1];
      v7 = v1;
    }
    else {
      v3 = *v6;
      if (*(char *)((long)(int)(unsigned int)(unsigned char)sub_4eb2((int)v3) + 0x2b640)) { // branch-flip
        v1 = &v6[1];
        v2 = &v7[1];
        *v7 = *v6;
        v6 = v1;
        v7 = v2;
      }
      else {
        v1 = &v6[1];
        v4 = sub_4eb2((int)*v6);
        v7 = &v7[sprintf(v7,"%%%02x",(unsigned int)v4)];
        v6 = v1;
      }
    }
  }
  *v7 = '\0';
  return v5;
}


// Function: sub_d87f @ 0xd87f
long sub_d87f(char *a0,unsigned int *a1,unsigned int a2,struct_2 *a3,bool a4,obstack *a5,char *a6) // ternary
{
  char *v1;
  char *v2; // rax
  char v3 [8200];
  char *v4; // stack - 0x2038
  unsigned char v5; // stack - 0x203a
  unsigned char v6; // stack - 0x2039
  long v7; // stack - 0x2030
  char *v8; // stack - 0x2028
  char *v9; // stack - 0x2020
  
  v1 = a6;
  v4 = v3;
  v7 = sub_d10c(&v4,0x2000,a0,a1,a2,NULL,&v5);
  if ((v5) && (a4))
    sub_5422(0x20);
  if (a3)
    sub_e0a3(a3);
  v6 = 0;
  if (v1) {
    if (((dat_2b341) && (dat_2b340)) && (v5 != 1)) {
      v6 = 1;
      putchar_unlocked((int)*v4);
    }
    v8 = (char *)sub_d775(dat_2b360,0);
    v9 = (char *)sub_d775(v1,1);
    v2 = (*v9 != '/') ? "/" : ""; // branch-flip
    printf("\x1b]8;;file://%s%s%s\a",v8,v2,v9);
    free(v8);
    free(v9);
  }
  if (a5)
    sub_54f4(a5);
  fwrite_unlocked(&v4[v6],1,v7 - (int)((unsigned int)v6 * 2),stdout);
  dat_2b500 = v7 + dat_2b500;
  if (a5)
    sub_54f4(a5);
  if ((v1) && (fputs_unlocked("\x1b]8;;\a",stdout), v6))
    putchar_unlocked((int)v4[v7 + -1]);
  if ((v4 != v3) && (a0 != v4))
    free(v4);
  return v7 + (unsigned long)v5;
}


// Function: sub_db96 @ 0xdb96
long sub_db96(struct_18 *a0,char a1,obstack *a2,unsigned long a3) // ternary x2
{
  bool v1; // al
  char *v2;
  struct_2 *v3; // rax
  long v4; // rax
  unsigned long v5; // rax
  
  v2 = (a1) ? a0->field_0x8 : a0->field_0x0; // branch-flip
  v3 = (dat_2b3d4) ? (struct_2 *)sub_e101(a0,(unsigned char)a1) : NULL; // branch-flip
  if (dat_2b3d4) { // branch-flip
    if (!v3) {
      if (!sub_6325(4)) goto label_dc1d;
    }
    v1 = 1;
  }
  else {
label_dc1d:
    v1 = 0;
  }
  v4 = sub_d87f(v2,dat_2b410,a0->field_0xc4,v3,a1 == '\0',a2,a0->field_0x10);
  sub_648c();
  if (v1) {
    sub_dce3();
    if (dat_2b430) {
      v5 = (v4 + a3) - 1;
      if (a3 / dat_2b430 != v5 / dat_2b430)
        sub_e4ca((struct_2 *)0x2b1f0,dat_2b430,v5 % dat_2b430);
    }
  }
  return v4;
}


// Function: sub_dce3 @ 0xdce3
void sub_dce3(void) // return-dupe
{
  if (dat_2b0a8) {
    sub_e4ca((struct_2 *)0x2b0a0);
    return;
  }
  sub_e4ca((struct_2 *)0x2b080);
  sub_e4ca((struct_2 *)0x2b0b0);
  sub_e4ca((struct_2 *)0x2b090);
}


// Function: sub_dd38 @ 0xdd38
char * sub_dd38(long a0,unsigned long a1) // ternary
{
  char v1;
  unsigned int v2;
  unsigned int v3;
  char *v4;
  int v5;
  char *v6; // rax
  int v7;
  char v8 [664];
  long v9; // stack - 0x2b0
  
  sub_63f3();
  if (dat_2b3e9) {
    v6 = (char *)sub_c643(v8,0x28c,a0);
    v4 = v6;
    if (dat_2b3ac != 4) // branch-flip
      v5 = dat_2b384;
    else {
      v5 = 0;
    }
    printf("%*s ",v5,v4);
  }
  if (dat_2b3bb) {
    v6 = (*(char *)(a0 + 0xb8) != '\x01') ? "?" : (char *)sub_13d91(*(unsigned long *)(a0 + 0x58),v8,dat_2b3bc,0x200,dat_2b3c0); // branch-flip
    if (dat_2b3ac != 4) // branch-flip
      v7 = dat_2b388;
    else {
      v7 = 0;
    }
    printf("%*s ",v7,v6);
  }
  if (dat_2b380) {
    if (dat_2b3ac != 4) // branch-flip
      v5 = dat_2b390;
    else {
      v5 = 0;
    }
    printf("%*s ",v5,*(char **)(a0 + 0xb0));
  }
  v6 = (long)sub_db96(a0,0,NULL,a1);
  v9 = (long)v6;
  if (dat_2b3d0) {
    v2 = *(unsigned int *)(a0 + 0xa8);
    v3 = *(unsigned int *)(a0 + 0x30);
    v1 = *(char *)(a0 + 0xb8);
    v6 = (unsigned long)(unsigned long)(unsigned char)sub_e05c((unsigned char)v1,v3,v2);
    v9 += (long)v6;
  }
  v6 = (char *)v9;
  return v6;
}


// Function: sub_df23 @ 0xdf23
char sub_df23(bool a0,unsigned int a1,int a2) // return-dupe x7, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_df23
{
  char v1; // al
  unsigned int v2; // eax
  bool v3;
  
  if (a0) { // branch-flip
    v2 = a1 & 0xf000;
    v3 = v2 == 0x8000;
  }
  else {
    v3 = a2 == 5;
  }
  if (v3) {
    if (((a0) && (dat_2b3d0 == 3)) && (v2 = a1 & 0x49, v2)) {
      v1 = 0x2a;
      return v1;
    }
    v1 = 0;
    return v1;
  }
  if (a0) { // branch-flip
    v2 = a1 & 0xf000;
    v1 = v2 == 0x4000;
  }
  else {
    v1 = !((a2 != 3) && (a2 != 9));
  }
  if ((bool)v1) {
    v1 = 0x2f;
    return v1;
  }
  if (dat_2b3d0 == 1) {
    v1 = 0;
    return v1;
  }
  if (a0) { // branch-flip
    v2 = a1 & 0xf000;
    v3 = v2 == 0xa000;
  }
  else {
    v3 = a2 == 6;
  }
  if (v3) {
    v1 = 0x40;
    return v1;
  }
  if (a0) { // branch-flip
    v2 = a1 & 0xf000;
    v3 = v2 == 0x1000;
  }
  else {
    v3 = a2 == 1;
  }
  if (v3) {
    v1 = 0x7c;
    return v1;
  }
  if (a0) { // branch-flip
    v2 = a1 & 0xf000;
    v3 = v2 == 0xc000;
  }
  else {
    v3 = a2 == 7;
  }
  if (!v3) {
    v1 = 0;
    return v1;
  }
  v1 = 0x3d;
  return v1;
}


// Function: sub_e05c @ 0xe05c
unsigned long sub_e05c(char a0,unsigned int a1,unsigned int a2)
{
  unsigned long v1;
  char v2;
  char v3; // al
  unsigned long v4; // rax
  
  v4 = sub_df23((unsigned char)a0,a1,a2);
  v2 = v3;
  if (v3)
    v4 = sub_5422((int)v3);
  v1 = (unsigned long)v4 >> 8;
  v4 = CONCAT71((undefined7)v1,v2 != '\0');
  return v4;
}


// Function: sub_e0a3 @ 0xe0a3
unsigned long sub_e0a3(struct_2 *a0)
{
  unsigned long v1;
  unsigned long v2; // rax
  
  if (a0) {
    if (sub_6325(4))
      sub_63ca();
    sub_e4ca((struct_2 *)0x2b080);
    sub_e4ca(a0);
    v2 = sub_e4ca((struct_2 *)0x2b090);
  }
  v1 = (unsigned long)v2 >> 8;
  v2 = CONCAT71((undefined7)v1,a0 != NULL);
  return v2;
}


// Function: sub_e101 @ 0xe101
unsigned long * sub_e101(struct_17 *a0,bool a1) // early-return, ternary x3
{
  unsigned long v1;
  char *v2;
  unsigned long v3;
  unsigned long v4; // rax
  unsigned int v5; // stack - 0x34
  unsigned int v6; // stack - 0x30
  unsigned int v7; // stack - 0x2c
  unsigned long *v8; // stack - 0x28
  char *v9; // stack - 0x20
  
  if (a1) { // branch-flip
    v9 = a0->field_0x8;
    v6 = a0->field_0xac;
    v7 = (a0->field_0xb9) ? 0 : 0xffffffff; // branch-flip
  }
  else {
    v9 = a0->field_0x0;
    v6 = sub_53e7(a0);
    v7 = (unsigned int)(unsigned char)a0->field_0xb9;
  }
  if ((v7 != 0xffffffff) || (!sub_6325(0xc))) {
    if (a0->field_0xb8 != '\x01') // branch-flip
      v5 = *(unsigned int *)((unsigned long)a0->field_0xa8 * 4 + 0x2b040);
    else if ((v6 & 0xf000) != 0x8000) { // branch-flip
      if ((v6 & 0xf000) != 0x4000) { // branch-flip
        if ((v6 & 0xf000) != 0xa000) { // branch-flip
          if ((v6 & 0xf000) != 0x1000) { // branch-flip
            if ((v6 & 0xf000) != 0xc000) { // branch-flip
              if ((v6 & 0xf000) != 0x6000) { // branch-flip
                v5 = ((v6 & 0xf000) != 0x2000) ? 0xd : 0xb; // branch-flip
              }
              else {
                v5 = 10;
              }
            }
            else {
              v5 = 9;
            }
          }
          else {
            v5 = 8;
          }
        }
        else {
          v5 = 7;
        }
      }
      else {
        v5 = 6;
        if (((v6 & 0x200) && (v6 & 2)) && (sub_6325(0x14)))
          v5 = 0x14;
        else if ((v6 & 2) && (sub_6325(0x13)))
          v5 = 0x13;
        else if ((v6 & 0x200) && (sub_6325(0x12)))
          v5 = 0x12;
      }
    }
    else {
      v5 = 5;
      if ((v6 & 0x800) && (sub_6325(0x10)))
        v5 = 0x10;
      else if ((v6 & 0x400) && (sub_6325(0x11)))
        v5 = 0x11;
      else if ((sub_6325(0x15)) && (a0->field_0xc0))
        v5 = 0x15;
      else if ((v6 & 0x49) && (sub_6325(0xe)))
        v5 = 0xe;
      else if ((2 <= a0->field_0x28) && (sub_6325(0x16)))
        v5 = 0x16;
    }
  }
  else {
    v5 = 0xc;
  }
  v8 = NULL;
  if (v5 == 5) {
    v4 = strlen(v9);
    v8 = dat_2b3d8;
    while ((v8 && ((v4 < *v8 || (v1 = *v8, v2 = (char *)v8[1], v3 = *v8, sub_102b5(&v9[v4 - v3],v2,v1)))))) {
      v8 = (unsigned long *)v8[4];
    }
  }
  if (((v5 == 7) && (!v7)) && ((dat_2b358 || (sub_6325(0xd)))))
    v5 = 0xd;
  v8 = (v8) ? &v8[2] : (unsigned long *)((unsigned long)v5 * 0x10 + 0x2b080); // branch-flip
  if (!v8[1])
    return NULL;
  return v8;
}


// Function: sub_e4ca @ 0xe4ca
void sub_e4ca(struct_2 *a0)
{
  if (dat_2b3d6 != '\x01') {
    dat_2b3d6 = '\x01';
    if (0 <= tcgetpgrp(1))
      sub_6858();
    sub_dce3();
  }
  fwrite_unlocked(a0->field_0x8,a0->field_0x0,1,stdout);
}


// Function: sub_e530 @ 0xe530
long sub_e530(long a0) // ternary
{
  char v1;
  unsigned int v2;
  unsigned int v3;
  unsigned long v4;
  char *v5; // rax
  char v6 [664];
  long v7; // stack - 0x2b0
  
  v7 = 0;
  if (dat_2b3e9) {
    if (dat_2b3ac != 4) // branch-flip
      v4 = (unsigned long)dat_2b384;
    else {
      v4 = *(unsigned long *)(a0 + 0x20);
      v4 = strlen((char *)sub_15035(v4,v6));
    }
    v7 = v4 + 1;
  }
  if (dat_2b3bb) {
    if (dat_2b3ac != 4) // branch-flip
      v4 = (unsigned long)dat_2b388;
    else {
      v5 = (*(char *)(a0 + 0xb8) != '\x01') ? "?" : (char *)sub_13d91(*(unsigned long *)(a0 + 0x58),v6,dat_2b3bc,0x200,dat_2b3c0); // branch-flip
      v4 = strlen(v5);
    }
    v7 += v4 + 1;
  }
  if (dat_2b380) {
    if (dat_2b3ac != 4) // branch-flip
      v4 = (unsigned long)dat_2b390;
    else {
      v4 = strlen(*(char **)(a0 + 0xb0));
    }
    v7 += v4 + 1;
  }
  v7 += sub_b119(a0);
  if (dat_2b3d0) {
    v2 = *(unsigned int *)(a0 + 0xa8);
    v3 = *(unsigned int *)(a0 + 0x30);
    v1 = *(char *)(a0 + 0xb8);
    v7 += (unsigned long)(sub_df23((unsigned char)v1,v3,v2) != '\0');
  }
  return v7;
}


// Function: sub_e708 @ 0xe708
void sub_e708(void)
{
  long v1;
  unsigned long v10; // stack - 0x40
  long v2;
  long v3;
  unsigned long v4; // rax
  long v5; // rax
  long v6; // rax
  unsigned long v7; // stack - 0x58
  long v8; // stack - 0x50
  unsigned long v9; // stack - 0x48
  
  v4 = sub_ee73(1);
  v5 = v4 * 0x18 + dat_2b4f0;
  v4 = (unsigned long)(dat_2b338 % v4 != 0) + dat_2b338 / v4;
  for (v7 = 0; v7 < v4; v7 = v7 + 1) {
    v8 = 0;
    v9 = v7;
    v10 = 0;
    while( true ) {
      v2 = *(long *)(v9 * 8 + dat_2b348);
      v6 = sub_e530(v2);
      v1 = v8 + 1;
      v3 = *(long *)(v8 * 8 + *(long *)(v5 + -8));
      sub_dd38(v2,v10);
      v9 += v4;
      if (dat_2b338 <= v9) break;
      sub_eaef(v6 + v10,v10 + v3);
      v10 += v3;
      v8 = v1;
    }
    putchar_unlocked((int)dat_2b030);
  }
}


// Function: sub_e86a @ 0xe86a
void sub_e86a(void)
{
  long v1;
  unsigned long v2; // rax
  long v3; // rax
  unsigned long v4; // stack - 0x48
  unsigned long v5; // stack - 0x40
  long v6; // stack - 0x38
  long v7; // stack - 0x30
  
  v5 = 0;
  v2 = sub_ee73(0);
  v3 = v2 * 0x18 + -0x18 + dat_2b4f0;
  v1 = *dat_2b348;
  v6 = sub_e530(v1);
  v7 = **(long **)(v3 + 0x10);
  sub_dd38(v1,0);
  for (v4 = 1; v4 < dat_2b338; v4 = v4 + 1) {
    if (v4 % v2) { // branch-flip
      sub_eaef(v6 + v5,v5 + v7);
      v5 += v7;
    }
    else {
      putchar_unlocked((int)dat_2b030);
      v5 = 0;
    }
    v1 = dat_2b348[v4];
    sub_dd38(v1,v5);
    v6 = sub_e530(v1);
    v7 = *(long *)((v4 % v2) * 8 + *(long *)(v3 + 0x10));
  }
  putchar_unlocked((int)dat_2b030);
}


// Function: sub_e9d6 @ 0xe9d6
void sub_e9d6(char a0)
{
  long v1;
  unsigned long v2; // rax
  char v3; // stack - 0x29
  unsigned long v4; // stack - 0x28
  unsigned long v5; // stack - 0x20
  
  v5 = 0;
  for (v4 = 0; v4 < dat_2b338; v4 = v4 + 1) {
    v1 = *(long *)(v4 * 8 + dat_2b348);
    if (dat_2b430) // branch-flip
      v2 = sub_e530(v1);
    else {
      v2 = 0;
    }
    if (v4) {
      if ((dat_2b430) && ((dat_2b430 <= v2 + v5 + 2 || ((unsigned long)(-v2 - 3U) < v5)))) {
        v5 = 0;
        v3 = dat_2b030;
      }
      else {
        v5 += 2;
        v3 = ' ';
      }
      putchar_unlocked((int)a0);
      putchar_unlocked((int)v3);
    }
    sub_dd38(v1,v5);
    v5 += v2;
  }
  putchar_unlocked((int)dat_2b030);
}


// Function: sub_eaef @ 0xeaef
void sub_eaef(unsigned long a0,unsigned long a1)
{
  unsigned long v1; // stack - 0x10
  
  v1 = a0;
  while (v1 < a1) {
    if ((dat_2b420) && ((v1 + 1) / dat_2b420 < a1 / dat_2b420)) {
      putchar_unlocked(9);
      v1 += dat_2b420 - v1 % dat_2b420;
    }
    else {
      putchar_unlocked(0x20);
      v1 += 1;
    }
  }
}


// Function: sub_eb94 @ 0xeb94
void sub_eb94(char *a0,char *a1,char *a2)
{
  char *v1;
  char *v2;
  char *v3; // stack - 0x30
  char *v4; // stack - 0x20
  char *v5; // stack - 0x10
  
  v4 = a0;
  v5 = a1;
  if ((*a1 != '.') || (v3 = a2, v4 = a0, v5 = a1, a1[1])) {
    while (*v5) {
      v1 = &v5[1];
      v2 = &v4[1];
      *v4 = *v5;
      v4 = v2;
      v5 = v1;
    }
    v3 = a2;
    if ((v5 <= a1) || (v3 = a2, v5[-1] == '/')) goto label_ec3b;
    v1 = &v4[1];
    *v4 = '/';
    v3 = a2;
    v4 = v1;
  }
label_ec3b:
  while (*v3) {
    v1 = &v3[1];
    v2 = &v4[1];
    *v4 = *v3;
    v3 = v1;
    v4 = v2;
  }
  *v4 = '\0';
}


// Function: sub_ec50 @ 0xec50
void sub_ec50(unsigned long a0)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // stack - 0x40
  unsigned long v5; // stack - 0x38
  long v6; // stack - 0x30
  unsigned long v7; // stack - 0x28
  
  if (dat_2c440 < a0) {
    if ((dat_2b4f8) && (dat_2b4f8 >> 1 <= a0)) {
      dat_2b4f0 = sub_1c638(dat_2b4f0,dat_2b4f8,0x18);
      v5 = dat_2b4f8;
    }
    else {
      dat_2b4f0 = sub_1c638(dat_2b4f0,a0,0x30);
      v5 = a0 * 2;
    }
    v1 = v5 - dat_2c440;
    v2 = v5 + dat_2c440 + 1;
    v3 = v2 * v1;
    if ((v2 < v5) || (v2 != v3 / v1))
      sub_1ccc3(); // no-return
    v6 = sub_1c804(v3 >> 1,8,v3 % v1);
    for (v4 = dat_2c440; v4 < v5; v4 = v4 + 1) {
      *(long *)(dat_2b4f0 + v4 * 0x18 + 0x10) = v6;
      v6 += (v4 + 1) * 8;
    }
    dat_2c440 = v5;
  }
  for (v4 = 0; v4 < a0; v4 = v4 + 1) {
    *(char *)(v4 * 0x18 + dat_2b4f0) = 1;
    *(unsigned long *)(dat_2b4f0 + v4 * 0x18 + 8) = (v4 + 1) * 3;
    for (v7 = 0; v7 <= v4; v7 = v7 + 1) {
      *(unsigned long *)(v7 * 8 + *(long *)(v4 * 0x18 + dat_2b4f0 + 0x10)) = 3;
    }
  }
}


// Function: sub_ee73 @ 0xee73
unsigned long sub_ee73(bool a0) // ternary
{
  long v1; // rax
  unsigned long v2; // rax
  unsigned long v3; // rax
  long v4; // rdx
  unsigned long v5; // stack - 0x48
  unsigned long v6; // stack - 0x40
  unsigned long v7; // stack - 0x38
  
  if ((dat_2b4f8) && (dat_2b4f8 < dat_2b338))
    v6 = dat_2b4f8;
  else {
    v6 = dat_2b338;
  }
  sub_ec50(v6);
  for (v5 = 0; v5 < dat_2b338; v5 = v5 + 1) {
    v1 = sub_e530(*(long *)(v5 * 8 + dat_2b348));
    for (v7 = 0; v7 < v6; v7 = v7 + 1) {
      if (*(char *)(v7 * 0x18 + dat_2b4f0)) {
        if (a0) // branch-flip
          v2 = v5 / ((v7 + dat_2b338) / (v7 + 1));
        else {
          v2 = v5 % (v7 + 1);
        }
        v4 = (v2 != v7) ? 2 : 0; // branch-flip
        v3 = v1 + v4;
        if (*(unsigned long *)(v2 * 8 + *(long *)(v7 * 0x18 + dat_2b4f0 + 0x10)) < v3) {
          *(unsigned long *)(v7 * 0x18 + dat_2b4f0 + 8) = *(long *)(v7 * 0x18 + dat_2b4f0 + 8) + (v3 - *(long *)(v2 * 8 + *(long *)(v7 * 0x18 + dat_2b4f0 + 0x10)));
          *(unsigned long *)(*(long *)(v7 * 0x18 + dat_2b4f0 + 0x10) + v2 * 8) = v3;
          *(bool *)(v7 * 0x18 + dat_2b4f0) = *(unsigned long *)(v7 * 0x18 + dat_2b4f0 + 8) < dat_2b430;
        }
      }
    }
  }
  while ((2 <= v6 && (!*(char *)(v6 * 0x18 + -0x18 + dat_2b4f0)))) {
    v6 -= 1;
  }
  return v6;
}


// Function: sub_f135 @ 0xf135
void sub_f135(int a0) // ternary
{
  FILE *v1;
  char *v2;
  
  v2 = dat_2c478;
  if (a0) // branch-flip
    fprintf(stderr,gettext("Try \'%s --help\' for more information.\n"),v2);
  else {
    printf(gettext("Usage: %s [OPTION]... [FILE]...\n"),v2);
    v1 = stdout;
    fputs_unlocked(gettext("List information about the FILEs (the current directory by default).\nSort entries alphabetically if none of -cftuvSUX nor --sort is specified.\n"),v1);
    sub_4f20();
    v1 = stdout;
    fputs_unlocked(gettext("  -a, --all                  do not ignore entries starting with .\n  -A, --almost-all           do not list implied . and ..\n      --author               with -l, print the author of each file\n  -b, --escape               print C-style escapes for nongraphic characters\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --block-size=SIZE      with -l, scale sizes by SIZE when printing them;\n                             e.g., \'--block-size=M\'; see SIZE format below\n\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -B, --ignore-backups       do not list implied entries ending with ~\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -c                         with -lt: sort by, and show, ctime (time of last\n                             modification of file status information);\n                             with -l: show ctime and sort by name;\n                             otherwise: sort by ctime, newest first\n\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -C                         list entries by columns\n      --color[=WHEN]         color the output WHEN; more info below\n  -d, --directory            list directories themselves, not their contents\n  -D, --dired                generate output designed for Emacs\' dired mode\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -f                         list all entries in directory order\n  -F, --classify[=WHEN]      append indicator (one of */=>@|) to entries WHEN\n      --file-type            likewise, except do not append \'*\'\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --format=WORD          across -x, commas -m, horizontal -x, long -l,\n                             single-column -1, verbose -l, vertical -C\n\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --full-time            like -l --time-style=full-iso\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -g                         like -l, but do not list owner\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --group-directories-first\n                             group directories before files;\n                             can be augmented with a --sort option, but any\n                             use of --sort=none (-U) disables grouping\n\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -G, --no-group             in a long listing, don\'t print group names\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -h, --human-readable       with -l and -s, print sizes like 1K 234M 2G etc.\n      --si                   likewise, but use powers of 1000 not 1024\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -H, --dereference-command-line\n                             follow symbolic links listed on the command line\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --dereference-command-line-symlink-to-dir\n                             follow each command line symbolic link\n                             that points to a directory\n\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --hide=PATTERN         do not list implied entries matching shell PATTERN\n                             (overridden by -a or -A)\n\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --hyperlink[=WHEN]     hyperlink file names WHEN\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --indicator-style=WORD\n                             append indicator with style WORD to entry names:\n                             none (default), slash (-p),\n                             file-type (--file-type), classify (-F)\n\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -i, --inode                print the index number of each file\n  -I, --ignore=PATTERN       do not list implied entries matching shell PATTERN\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -k, --kibibytes            default to 1024-byte blocks for file system usage;\n                             used only with -s and per directory totals\n\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -l                         use a long listing format\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -L, --dereference          when showing file information for a symbolic\n                             link, show information for the file the link\n                             references rather than for the link itself\n\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -m                         fill width with a comma separated list of entries\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -n, --numeric-uid-gid      like -l, but list numeric user and group IDs\n  -N, --literal              print entry names without quoting\n  -o                         like -l, but do not list group information\n  -p, --indicator-style=slash\n                             append / indicator to directories\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -q, --hide-control-chars   print ? instead of nongraphic characters\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --show-control-chars   show nongraphic characters as-is (the default,\n                             unless program is \'ls\' and output is a terminal)\n\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -Q, --quote-name           enclose entry names in double quotes\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --quoting-style=WORD   use quoting style WORD for entry names:\n                             literal, locale, shell, shell-always,\n                             shell-escape, shell-escape-always, c, escape\n                             (overrides QUOTING_STYLE environment variable)\n\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -r, --reverse              reverse order while sorting\n  -R, --recursive            list subdirectories recursively\n  -s, --size                 print the allocated size of each file, in blocks\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -S                         sort by file size, largest first\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --sort=WORD            sort by WORD instead of name: none (-U), size (-S),\n                             time (-t), version (-v), extension (-X), width\n\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --time=WORD            change the default of using modification times;\n                               access time (-u): atime, access, use;\n                               change time (-c): ctime, status;\n                               birth time: birth, creation;\n                             with -l, WORD determines which time to show;\n                             with --sort=time, sort by WORD (newest first)\n\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --time-style=TIME_STYLE\n                             time/date format with -l; see TIME_STYLE below\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -t                         sort by time, newest first; see --time\n  -T, --tabsize=COLS         assume tab stops at each COLS instead of 8\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -u                         with -lt: sort by, and show, access time;\n                             with -l: show access time and sort by name;\n                             otherwise: sort by access time, newest first\n\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -U                         do not sort; list entries in directory order\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -v                         natural sort of (version) numbers within text\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -w, --width=COLS           set output width to COLS.  0 means no limit\n  -x                         list entries by lines instead of by columns\n  -X                         sort alphabetically by entry extension\n  -Z, --context              print any security context of each file\n      --zero                 end each output line with NUL, not newline\n  -1                         list one file per line\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --help        display this help and exit\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --version     output version information and exit\n"),v1);
    sub_4f51();
    v1 = stdout;
    fputs_unlocked(gettext("\nThe TIME_STYLE argument can be full-iso, long-iso, iso, locale, or +FORMAT.\nFORMAT is interpreted like in date(1).  If FORMAT is FORMAT1<newline>FORMAT2,\nthen FORMAT1 applies to non-recent files and FORMAT2 to recent files.\nTIME_STYLE prefixed with \'posix-\' takes effect only outside the POSIX locale.\nAlso the TIME_STYLE environment variable sets the default style to use.\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("\nThe WHEN argument defaults to \'always\' and can also be \'auto\' or \'never\'.\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("\nUsing color to distinguish file types is disabled both by default and\nwith --color=never.  With --color=auto, ls emits color codes only when\nstandard output is connected to a terminal.  The LS_COLORS environment\nvariable can change the settings.  Use the dircolors(1) command to set it.\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("\nExit status:\n 0  if OK,\n 1  if minor problems (e.g., cannot access subdirectory),\n 2  if serious trouble (e.g., cannot access command-line argument).\n"),v1);
    if (dat_2b210 != 1) { // branch-flip
      v2 = (dat_2b210 != 2) ? "vdir" : "dir"; // branch-flip
    }
    else {
      v2 = "ls";
    }
    sub_4f82(v2);
  }
  exit(a0); // no-return
}


// Function: sub_11fd6 @ 0x11fd6
unsigned long sub_11fd6(long a0)
{
  return *(unsigned long *)(a0 + 0x10);
}


// Function: sub_11fec @ 0x11fec
unsigned long sub_11fec(long a0)
{
  return *(unsigned long *)(a0 + 0x18);
}


// Function: sub_12002 @ 0x12002
unsigned long sub_12002(long a0)
{
  return *(unsigned long *)(a0 + 0x20);
}


// Function: sub_12018 @ 0x12018
unsigned long sub_12018(struct_19 *a0)
{
  long *v1; // stack - 0x28
  unsigned long v2; // stack - 0x20
  long *v3; // stack - 0x18
  unsigned long v4; // stack - 0x10
  
  v2 = 0;
  for (v1 = a0->field_0x0; v1 < a0->field_0x8; v1 = &v1[2]) {
    if (*v1) {
      v3 = v1;
      v4 = 1;
      while (v3 = (long *)v3[1], v3) {
        v4 += 1;
      }
      if (v2 < v4)
        v2 = v4;
    }
  }
  return v2;
}


// Function: sub_1209a @ 0x1209a
unsigned long sub_1209a(struct_19 *a0) // return-dupe
{
  long *v1; // stack - 0x28
  long v2; // stack - 0x20
  long v3; // stack - 0x18
  long *v4; // stack - 0x10
  
  v2 = 0;
  v3 = 0;
  for (v1 = a0->field_0x0; v1 < a0->field_0x8; v1 = &v1[2]) {
    if (*v1) {
      v4 = v1;
      v2 += 1;
      do {
        v3 += 1;
        v4 = (long *)v4[1];
      } while (v4);
    }
  }
  if ((v2 == a0->field_0x18) && (v3 == a0->field_0x20))
    return 1;
  return 0;
}


// Function: sub_12138 @ 0x12138
void sub_12138(unsigned long a0,FILE *a1)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // rax
  
  v1 = sub_12002(a0);
  v2 = sub_11fd6(a0);
  v3 = sub_11fec(a0);
  v4 = sub_12018(a0);
  fprintf(a1,"# entries:         %lu\n",v1);
  fprintf(a1,"# buckets:         %lu\n",v2);
  fprintf(a1,"# buckets used:    %lu (%.2f%%)\n",v3,(dat_234f8 * (double)v3) / (double)v2);
  fprintf(a1,"max bucket length: %lu\n",v4);
}


// Function: sub_1227f @ 0x1227f
long sub_1227f(struct_5 *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  v1 = (*a0->field_0x30)(a1,a0->field_0x10);
  if ((unsigned long)a0->field_0x10 <= v1)
    abort(); // no-return
  return v1 * 0x10 + a0->field_0x0;
}


// Function: sub_122da @ 0x122da
long sub_122da(long a0,long a1) // return-dupe
{
  void *v1;
  long v2;
  long *v3; // stack - 0x18
  
  v3 = (long *)sub_1227f(a0,a1);
  if (!*v3)
    return 0;
  while( true ) {
    if (!v3)
      return 0;
    if ((a1 == *v3) || (v1 = *(void **)(a0 + 0x38), v2 = *v3, (*v1)(a1,v2))) break;
    v3 = (long *)v3[1];
  }
  return *v3;
}


// Function: sub_12371 @ 0x12371
long sub_12371(struct_19 *a0) // early-return
{
  long *v1; // stack - 0x10
  
  if (!a0->field_0x20)
    return 0;
  v1 = a0->field_0x0;
  while( true ) {
    if (a0->field_0x8 <= v1)
      abort(); // no-return
    if (*v1) break;
    v1 = &v1[2];
  }
  return *v1;
}


// Function: sub_123d1 @ 0x123d1
long sub_123d1(long a0,long a1) // early-return
{
  long *v1; // stack - 0x18
  long *v2; // stack - 0x10
  
  v1 = (long *)sub_1227f(a0,a1);
  v2 = v1;
  while ((a1 != *v2 || (!v2[1]))) {
    v2 = (long *)v2[1];
    if (!v2) {
      do {
        v1 = &v1[2];
        if (*(long **)(a0 + 8) <= v1)
          return 0;
      } while (!*v1);
      return *v1;
    }
  }
  return *(long *)v2[1];
}


// Function: sub_1246f @ 0x1246f
unsigned long sub_1246f(struct_19 *a0,long a1,unsigned long a2)
{
  unsigned long v1;
  unsigned long v2; // stack - 0x20
  long *v3; // stack - 0x18
  long *v4; // stack - 0x10
  
  v2 = 0;
  v3 = a0->field_0x0;
  do {
    if (a0->field_0x8 <= v3)
      return v2;
    if (*v3) {
      v4 = v3;
      while (v4) {
        if (a2 <= v2)
          return v2;
        v1 = v2 + 1;
        *(long *)(v2 * 8 + a1) = *v4;
        v4 = (long *)v4[1];
        v2 = v1;
      }
    }
    v3 = &v3[2];
  } while( true );
}


// Function: sub_1250f @ 0x1250f
long sub_1250f(struct_19 *a0,void *a1,unsigned long a2)
{
  long v1;
  long v2; // stack - 0x20
  long *v3; // stack - 0x18
  long *v4; // stack - 0x10
  
  v2 = 0;
  v3 = a0->field_0x0;
  do {
    if (a0->field_0x8 <= v3)
      return v2;
    if (*v3) {
      for (v4 = v3; v4; v4 = (long *)v4[1]) {
        v1 = *v4;
        if ((*a1)(v1,a2) != '\x01')
          return v2;
        v2 += 1;
      }
    }
    v3 = &v3[2];
  } while( true );
}


// Function: sub_125a7 @ 0x125a7
unsigned long sub_125a7(unsigned char *a0,unsigned long a1)
{
  unsigned char *v1; // stack - 0x20
  unsigned long v2; // stack - 0x10
  
  v2 = 0;
  for (v1 = a0; *v1; v1 = &v1[1]) {
    v2 = ((unsigned long)*v1 + v2 * 0x1f) % a1;
  }
  return v2;
}


// Function: sub_12601 @ 0x12601
unsigned long sub_12601(unsigned long a0)
{
  unsigned long v1; // stack - 0x18
  unsigned long v2; // stack - 0x10
  
  v2 = 9;
  for (v1 = 3; (v2 < a0 && (a0 % v1)); v1 = v1 + 2) {
    v2 += (v1 + 1) * 4;
  }
  return CONCAT71((undefined7)(a0 % v1 >> 8),a0 % v1 != 0);
}


// Function: sub_12670 @ 0x12670
unsigned long sub_12670(unsigned long a0)
{
  unsigned long v1; // stack - 0x10
  
  v1 = a0;
  if (a0 <= 9)
    v1 = 10;
  v1 |= 1;
  while( true ) {
    if (v1 == 0xffffffffffffffff)
      return 0xffffffffffffffff;
    if (sub_12601(v1) == '\x01') break;
    v1 += 2;
  }
  return v1;
}


// Function: sub_126bb @ 0x126bb
void sub_126bb(struct_20 *a0)
{
  a0->field_0x0 = 0;
  a0->field_0x4 = dat_23500;
  a0->field_0x8 = dat_23504;
  a0->field_0xc = dat_23508;
  a0->field_0x10 = 0;
}


// Function: sub_12711 @ 0x12711
unsigned long sub_12711(unsigned long a0,unsigned long a1)
{
  return (unsigned long)sub_1e664(a0,3) % a1;
}


// Function: sub_1274c @ 0x1274c
unsigned long sub_1274c(long a0,long a1)
{
  return CONCAT71((undefined7)((unsigned long)a0 >> 8),a0 == a1);
}


// Function: sub_12769 @ 0x12769
unsigned long sub_12769(long a0) // return-dupe x2
{
  float *v1;
  
  v1 = *(float **)(a0 + 0x28);
  if (v1 == (float *)0x23470)
    return 1;
  if ((((dat_2350c < v1[2]) && (v1[2] < dat_23500 - dat_2350c)) && (dat_2350c + dat_23500 < v1[3])) && (((0.0 <= *v1 && (*v1 + dat_2350c < v1[1])) && ((v1[1] <= dat_23500 && (*v1 + dat_2350c < v1[2]))))))
    return 1;
  *(unsigned long *)(a0 + 0x28) = 0x23470;
  return 0;
}


// Function: sub_12876 @ 0x12876
unsigned long sub_12876(unsigned long a0,long a1) // early-return x2
{
  unsigned long v1; // rax
  float v2; // xmm0_da
  unsigned long v3; // stack - 0x20
  
  v3 = a0;
  if (*(char *)(a1 + 0x10) != '\x01') {
    v2 = (float)a0 / *(float *)(a1 + 8);
    if (dat_23510 <= v2)
      return 0;
    if (dat_23514 <= v2)
      v3 = (long)(v2 - dat_23514) ^ 0x8000000000000000;
    else {
      v3 = (unsigned long)v2;
    }
  }
  v1 = sub_12670(v3);
  if (!((long)(v1 << 3) <= -1 || v1 >> 0x3d))
    return v1;
  return 0;
}


// Function: sub_1298f @ 0x1298f
long * sub_1298f(unsigned long a0,long a1,void *a2,void *a3,long a4) // early-return x2
{
  unsigned long v1;
  long *v2; // rax
  void *v3; // stack - 0x38
  void *v4; // stack - 0x30
  long v5; // stack - 0x28
  
  v4 = a2;
  if (!a2)
    v4 = sub_12711;
  v3 = a3;
  if (!a3)
    v3 = sub_1274c;
  v2 = malloc(0x50);
  if (!v2)
    return NULL;
  v5 = a1;
  if (!a1)
    v5 = 0x23470;
  v2[5] = v5;
  if (sub_12769(v2) == '\x01') {
    v2[2] = sub_12876(a0,v5);
    if (v2[2]) {
      v1 = v2[2];
      *v2 = (long)calloc(v1,0x10);
      if (*v2) {
        v2[1] = *v2 + v2[2] * 0x10;
        v2[3] = 0;
        v2[4] = 0;
        v2[6] = (long)v4;
        v2[7] = (long)v3;
        v2[8] = a4;
        v2[9] = 0;
        return v2;
      }
    }
  }
  free(v2);
  return NULL;
}


// Function: sub_12b04 @ 0x12b04
void sub_12b04(struct_21 *a0)
{
  unsigned long *v1;
  long *v2; // stack - 0x20
  unsigned long *v3; // stack - 0x18
  
  for (v2 = a0->field_0x0; v2 < a0->field_0x8; v2 = &v2[2]) {
    if (*v2) {
      v3 = (unsigned long *)v2[1];
      while (v3) {
        if (a0->field_0x40)
          (*(void *)a0->field_0x40)(*v3);
        *v3 = 0;
        v1 = (unsigned long *)v3[1];
        v3[1] = a0->field_0x48;
        a0->field_0x48 = v3;
        v3 = v1;
      }
      if (a0->field_0x40)
        (*(void *)a0->field_0x40)(*v2);
      *v2 = 0;
      v2[1] = 0;
    }
  }
  a0->field_0x18 = 0;
  a0->field_0x20 = 0;
}


// Function: sub_12c0f @ 0x12c0f
void sub_12c0f(struct_12 *a0)
{
  void *v1;
  long *v2; // stack - 0x20
  long *v3; // stack - 0x18
  
  if ((a0->field_0x40) && (a0->field_0x20)) {
    for (v2 = a0->field_0x0; v2 < a0->field_0x8; v2 = &v2[2]) {
      if (*v2) {
        for (v3 = v2; v3; v3 = (long *)v3[1]) {
          (*(void *)a0->field_0x40)(*v3);
        }
      }
    }
  }
  for (v2 = a0->field_0x0; v2 < a0->field_0x8; v2 = &v2[2]) {
    v3 = (void *)v2[1];
    while (v3) {
      v1 = *(void **)((long)v3 + 8);
      free(v3);
      v3 = v1;
    }
  }
  v3 = a0->field_0x48;
  while (v3) {
    v1 = *(void **)((long)v3 + 8);
    free(v3);
    v3 = v1;
  }
  free(a0->field_0x0);
  free(a0);
}


// Function: sub_12d3e @ 0x12d3e
void * sub_12d3e(long a0)
{
  void *v1; // stack - 0x10
  
  if (*(long *)(a0 + 0x48)) { // branch-flip
    v1 = *(void **)(a0 + 0x48);
    *(unsigned long *)(a0 + 0x48) = *(unsigned long *)((long)v1 + 8);
  }
  else {
    v1 = malloc(0x10);
  }
  return v1;
}


// Function: sub_12d8d @ 0x12d8d
void sub_12d8d(long a0,struct_6 *a1)
{
  a1->field_0x0 = 0;
  a1->field_0x8 = *(unsigned long *)(a0 + 0x48);
  *(struct_6 **)(a0 + 0x48) = a1;
}


// Function: sub_12dc7 @ 0x12dc7
long sub_12dc7(long a0,long a1,unsigned long *a2,bool a3) // early-return
{
  void *v1;
  long v2;
  struct_6 *v3;
  long v4;
  unsigned long v5;
  long *v6; // stack - 0x38
  
  v6 = (long *)sub_1227f(a0,a1);
  *a2 = v6;
  if (!*v6)
    return 0;
  if ((a1 != *v6) && (v1 = *(void **)(a0 + 0x38), v2 = *v6, !(*v1)(a1,v2))) {
    while( true ) {
      if (!v6[1])
        return 0;
      if ((a1 == *(long *)v6[1]) || (v1 = *(void **)(a0 + 0x38), v5 = *(unsigned long *)v6[1], (*v1)(a1,v5))) break;
      v6 = (long *)v6[1];
    }
    v2 = *(long *)v6[1];
    if (a3) {
      v3 = (struct_6 *)v6[1];
      v6[1] = v3->field_0x8;
      sub_12d8d(a0,v3);
    }
    return v2;
  }
  v2 = *v6;
  if (a3) {
    if (v6[1]) { // branch-flip
      v3 = (struct_6 *)v6[1];
      v4 = v3->field_0x8;
      *v6 = v3->field_0x0;
      v6[1] = v4;
      sub_12d8d(a0,v3);
    }
    else {
      *v6 = 0;
    }
  }
  return v2;
}


// Function: sub_12f60 @ 0x12f60
unsigned long sub_12f60(long a0,struct_19 *a1,bool a2)
{
  long *v1;
  long v2;
  struct_0 *v3; // rax
  struct_0 *v4; // rax
  long *v5; // stack - 0x38
  long *v6; // stack - 0x30
  
  v5 = a1->field_0x0;
  do {
    if (a1->field_0x8 <= v5)
      return 1;
    if (*v5) {
      v6 = (long *)v5[1];
      while (v6) {
        v2 = *v6;
        v3 = (struct_0 *)sub_1227f(a0,v2);
        v1 = (long *)v6[1];
        if (v3->field_0x0) { // branch-flip
          v6[1] = v3->field_0x8;
          v3->field_0x8 = (long)v6;
        }
        else {
          v3->field_0x0 = v2;
          *(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + 1;
          sub_12d8d(a0,v6);
        }
        v6 = v1;
      }
      v2 = *v5;
      v5[1] = 0;
      if (!a2) {
        v3 = (struct_0 *)sub_1227f(a0,v2);
        if (v3->field_0x0) { // branch-flip
          v4 = (struct_0 *)sub_12d3e(a0);
          if (!v4)
            return 0;
          v4->field_0x0 = v2;
          v4->field_0x8 = v3->field_0x8;
          v3->field_0x8 = (long)v4;
        }
        else {
          v3->field_0x0 = v2;
          *(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + 1;
        }
        *v5 = 0;
        a1->field_0x18 = a1->field_0x18 + -1;
      }
    }
    v5 = &v5[2];
  } while( true );
}


// Function: sub_13134 @ 0x13134
unsigned long sub_13134(struct_8 *a0,unsigned long a1) // return-dupe x4
{
  void *v1; // stack - 0x68
  unsigned long v10; // stack - 0x20
  void *v2; // stack - 0x60
  unsigned long v3; // stack - 0x58
  unsigned long v4; // stack - 0x50
  unsigned long v5; // stack - 0x48
  unsigned long v6; // stack - 0x40
  unsigned long v7; // stack - 0x38
  unsigned long v8; // stack - 0x30
  unsigned long v9; // stack - 0x28
  
  v3 = sub_12876(a1,a0->field_0x28);
  if (!v3)
    return 0;
  if (v3 == a0->field_0x10)
    return 1;
  v1 = calloc(v3,0x10);
  if (!v1)
    return 0;
  v2 = (void *)((long)v1 + v3 * 0x10);
  v4 = 0;
  v5 = 0;
  v6 = a0->field_0x28;
  v7 = a0->field_0x30;
  v8 = a0->field_0x38;
  v9 = a0->field_0x40;
  v10 = a0->field_0x48;
  if (sub_12f60(&v1,a0,0)) {
    free(a0->field_0x0);
    a0->field_0x0 = v1;
    a0->field_0x8 = v2;
    a0->field_0x10 = v3;
    a0->field_0x18 = v4;
    a0->field_0x48 = v10;
    return 1;
  }
  a0->field_0x48 = v10;
  if ((sub_12f60(a0,&v1,1) == '\x01') && (sub_12f60(a0,&v1,0) == '\x01')) {
    free(v1);
    return 0;
  }
  abort(); // no-return
}


// Function: sub_1335f @ 0x1335f
unsigned long sub_1335f(long a0,long a1,long *a2) // return-dupe x2, return-dupe, ternary x2
{
  unsigned long v1; // rax
  long *v2; // rax
  long *v3; // stack - 0x30
  float v4; // xmm0_da
  long v5; // stack - 0x28
  long v6; // stack - 0x20
  
  if (!a1)
    abort(); // no-return
  v5 = sub_12dc7(a0,a1,&v3,0);
  if (v5) {
    if (!a2)
      return 0;
    *a2 = v5;
    return 0;
  }
  if (((float)*(unsigned long *)(a0 + 0x10) * *(float *)(*(long *)(a0 + 0x28) + 8) < (float)*(unsigned long *)(a0 + 0x18)) && (sub_12769(a0), (float)*(unsigned long *)(a0 + 0x10) * *(float *)(*(long *)(a0 + 0x28) + 8) < (float)*(unsigned long *)(a0 + 0x18))) {
    v6 = *(long *)(a0 + 0x28);
    v4 = (*(char *)(v6 + 0x10)) ? (float)*(unsigned long *)(a0 + 0x10) * *(float *)(v6 + 0xc) : *(float *)(v6 + 8) * (*(float *)(v6 + 0xc) * (float)*(unsigned long *)(a0 + 0x10)); // branch-flip
    if (dat_23510 <= v4)
      return 0xffffffff;
    v1 = (dat_23514 <= v4) ? (long)(v4 - dat_23514) ^ 0x8000000000000000 : (unsigned long)v4;
    if (sub_13134(a0,v1) != '\x01')
      return 0xffffffff;
    if (sub_12dc7(a0,a1,&v3,0))
      abort(); // no-return
  }
  if (!*v3) {
    *v3 = a1;
    *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + 1;
    *(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + 1;
    return 1;
  }
  v2 = (long *)sub_12d3e(a0);
  if (v2) {
    *v2 = a1;
    v2[1] = v3[1];
    v3[1] = (long)v2;
    *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + 1;
    return 1;
  }
  return 0xffffffff;
}


// Function: sub_136d0 @ 0x136d0
unsigned long sub_136d0(long a0,unsigned long a1) // early-return
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x18
  
  v1 = sub_1335f(a0,a1,&v3);
  if (v1 != -1) {
    if (v1) // branch-flip
      v2 = a1;
    else {
      v2 = v3;
    }
    return v2;
  }
  v2 = 0;
  return v2;
}


// Function: sub_13740 @ 0x13740
long sub_13740(long a0,long a1) // early-return, ternary x2
{
  char v1; // al
  long *v2; // stack - 0x40
  float v3; // xmm0_da
  void *v4; // stack - 0x38
  long v5; // stack - 0x30
  long v6; // stack - 0x28
  unsigned long v7; // stack - 0x20
  void *v8; // stack - 0x18
  
  v5 = sub_12dc7(a0,a1,&v2,1);
  if (!v5)
    return 0;
  *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + -1;
  if (((!*v2) && (*(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + -1, (float)*(unsigned long *)(a0 + 0x18) < (float)*(unsigned long *)(a0 + 0x10) * **(float **)(a0 + 0x28))) && (sub_12769(a0), (float)*(unsigned long *)(a0 + 0x18) < (float)*(unsigned long *)(a0 + 0x10) * **(float **)(a0 + 0x28))) {
    v6 = *(long *)(a0 + 0x28);
    if (*(char *)(v6 + 0x10)) { // branch-flip
      v3 = (float)*(unsigned long *)(a0 + 0x10) * *(float *)(v6 + 4);
      v7 = (dat_23514 <= v3) ? (long)(v3 - dat_23514) ^ 0x8000000000000000 : (unsigned long)v3;
    }
    else {
      v3 = *(float *)(v6 + 8) * (*(float *)(v6 + 4) * (float)*(unsigned long *)(a0 + 0x10));
      v7 = (dat_23514 <= v3) ? (long)(v3 - dat_23514) ^ 0x8000000000000000 : (unsigned long)v3;
    }
    v1 = sub_13134(a0,v7);
    if (v1 != '\x01') {
      v4 = *(void **)(a0 + 0x48);
      while (v4) {
        v8 = *(void **)((long)v4 + 8);
        free(v4);
        v4 = v8;
      }
      *(unsigned long *)(a0 + 0x48) = 0;
    }
  }
  return v5;
}


// Function: sub_13a46 @ 0x13a46
void sub_13a46(long a0,long a1)
{
  sub_13740(a0,a1);
}

