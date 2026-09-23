// Function: main @ 0x3ac0
int main(int argc,char **argv,char **envp) // return-dupe
{
  char *v1;
  char **v10;
  unsigned long v11 [4]; // stack - 0xf8
  stat v12; // stack - 0xd8
  int v13 [3]; // stack - 0x104
  char v14 [8];
  void *v15; // stack - 0x118
  void *v16; // stack - 0x120
  unsigned int v17; // stack - 0x110
  int v18; // stack - 0x12c
  int v19; // stack - 0x10c
  char *v2;
  int v20; // stack - 0x108
  unsigned char v3; // al
  int v4;
  int v5; // eax
  long v6; // rax
  unsigned long v7; // rax
  unsigned long v8; // rax
  unsigned int v9;
  
  v9 = 0;
  dat_13020 = 2;
  sub_62e0(*argv);
  setlocale(6,"");
  bindtextdomain("diffutils","/usr/local/share/locale");
  textdomain("diffutils");
  sub_6220(NULL);
  sub_86a0();
  v18 = 0;
  while (v4 = getopt_long(argc,argv,"aeimvx3AEL:TX",(option *)0x12920,NULL), v4 != -1) {
    switch(v4) {
      case 0x33:
        dat_130ed = 1;
        v9 |= 1;
        break;
      default:
        sub_41b0(NULL,0); // no-return
      case 0x41:
        dat_130eb = 1;
        v9 |= 2;
        dat_130ef = 1;
        break;
      case 0x45:
        dat_130ef = 1;
        v9 |= 4;
        break;
      case 0x4c:
        if (2 < v18)
          sub_41b0("too many file label options",0); // no-return
        v11[v18] = optarg;
        v18 += 1;
        break;
      case 0x54:
        dat_130ee = 1;
        break;
      case 0x58:
        dat_130ec = 1;
        v9 |= 8;
        break;
      case 0x61:
        dat_130f2 = 1;
        break;
      case 0x65:
        v9 |= 0x10;
        break;
      case 0x69:
        dat_130ea = '\x01';
        break;
      case 0x6d:
        dat_130e9 = 1;
        break;
      case 0x76:
        sub_8010(stdout,"diff3","GNU diffutils",dat_13018,sub_65f0("Randy Smith"),0);
        sub_5c30();
        return 0;
      case 0x78:
        dat_130ec = 1;
        v9 |= 0x20;
        break;
      case 0x80:
        dat_13010 = optarg;
        break;
      case 0x81:
        sub_4240();
        sub_5c30();
        return 0;
      case 0x82:
        dat_130f1 = 1;
      
    }
  }
  dat_130f0 = (dat_130e9 ^ 1) & v9 != 0;
  v3 = v9 == 0 & dat_130e9;
  dat_130eb |= v3;
  dat_130ef = v3 | dat_130ef;
  if (((v9 - 1 & v9) || ((dat_130ea && (dat_130e9)))) || ((v18 && (!dat_130ef))))
    sub_41b0("incompatible options",0); // no-return
  if (argc - optind != 3) {
    if (3 <= argc - optind)
      sub_41b0("extra operand \'%s\'",argv[(long)optind + 3]); // no-return
    sub_41b0("missing operand after \'%s\'",argv[(long)argc + -1]); // no-return
  }
  argv = &argv[optind];
  for (v6 = (long)v18; (int)v6 <= 2; v6 = v6 + 1) {
    v11[v6] = argv[v6];
  }
  v9 = (unsigned int)(v9 != 0 | dat_130e9);
  v4 = 2 - v9;
  v6 = (long)v4;
  v1 = argv[v6];
  if (strcmp(v1,"-")) {
label_3def:
    v19 = 3 - v4;
    v17 = 0;
    v13[0] = 0;
    v13[v19] = 1;
    v13[v6] = 2;
    v10 = argv;
    v20 = v4;
    do {
      v1 = *v10;
      if (strcmp(v1,"-")) {
        if (stat(v1,&v12) < 0)
          sub_43a0(*v10); // no-return
        if ((v12._24_4_ & 0xf000) == 0x4000) {
          error(2,0x15,"%s",*v10);
          return v5;
        }
      }
      v10 = &v10[1];
      if (&argv[3] == v10) {
        signal(0x11,0);
        v1 = argv[v13[2]];
        v7 = sub_5d70(argv[v13[1]],v1,v14,&v15);
        v2 = argv[v13[0]];
        v8 = sub_4e40(sub_5d70(v2,v1,v14,&v16));
        if (dat_130f0) // branch-flip
          v9 = sub_50f0(stdout,v8,&v17,v13,v11[0],v11[1],v11[2],v7);
        else if (dat_130e9) { // branch-flip
          sub_8570(argv[v13[0]],"r",stdin);
          v9 = sub_57d0(stdin,stdout,v8,&v17,v13,v11[0]);
          if (ferror_unlocked(stdin))
            sub_4210("read failed"); // no-return
        }
        else {
          sub_5520(stdout,v8,&v17,v13);
          v9 = 0;
        }
        free(v16);
        free(v15);
        sub_5c30();
        exit(v9 & 0xff); // no-return
      }
    } while( true );
  }
  v1 = *argv;
  v4 = v9 + 1;
  if (strcmp(v1,"-")) {
    v6 = (long)v4;
    v1 = argv[v6];
    if (strcmp(v1,"-")) goto label_3def;
  }
  sub_4210("\'-\' specified for more than one input file"); // no-return
}


// Function: sub_4110 @ 0x4110
void sub_4110(char *a0)
{
  char v1;
  
  v1 = *a0;
  if (v1 != ' ') goto label_412d;
  do {
    do {
      v1 = a0[1];
      a0 = &a0[1];
    } while (v1 == ' ');
label_412d:
  } while (v1 == '\t');
}


// Function: sub_4140 @ 0x4140
unsigned char * sub_4140(unsigned char *a0,long *a1)
{
  unsigned char *v1;
  unsigned int v2; // edx
  
  v2 = (unsigned int)*a0;
  v1 = NULL;
  if ((unsigned int)*a0 - 0x30 <= 9) {
    do {
      a0 = &a0[1];
      v1 = (unsigned char *)((long)(int)(v2 - 0x30) + (long)v1 * 10);
      v2 = (unsigned int)*a0;
    } while (v2 - 0x30 <= 9);
    *a1 = (long)v1;
    return a0;
  }
  return v1;
}


// Function: sub_4180 @ 0x4180
void sub_4180(long a0) // return-dupe
{
  long v1; // rax
  long v2;
  
  if (!a0)
    return;
  v2 = 0;
  while( true ) {
    v1 = a0;
    a0 = *(long *)(v1 + 0x68);
    *(long *)(v1 + 0x68) = v2;
    if (!a0) break;
    v2 = v1;
  }
}


// Function: sub_41b0 @ 0x41b0
void sub_41b0(char *a0,unsigned long a1)
{
  char *v1;
  
  if (a0)
    error(0,0,dcgettext(NULL,a0,5),a1);
  v1 = dat_23140;
  error(2,0,dcgettext(NULL,"Try \'%s --help\' for more information.",5),v1);
}


// Function: sub_4210 @ 0x4210
void sub_4210(char *a0)
{
  error(2,0,"%s",dcgettext(NULL,a0,5));
}


// Function: sub_4240 @ 0x4240
void sub_4240(void)
{
  FILE *v1;
  char v2;
  long v3; // rbx
  char *v4;
  
  v4 = dat_23140;
  v3 = 0x12880;
  __printf_chk(1,dcgettext(NULL,"Usage: %s [OPTION]... MYFILE OLDFILE YOURFILE\n",5),v4);
  __printf_chk(1,"%s\n\n",dcgettext(NULL,"Compare three files line by line.",5));
  v1 = stdout;
  fputs_unlocked(dcgettext(NULL,"Mandatory arguments to long options are mandatory for short options too.\n",5),v1);
  v2 = '-';
  v4 = "-A, --show-all              output all changes, bracketing conflicts";
  while( true ) {
    if (v2) { // branch-flip
      __printf_chk(1,"  %s\n",dcgettext(NULL,v4,5));
      v4 = *(char **)(v3 + 8);
      v1 = stdout;
    }
    else {
      putchar_unlocked(10);
      v4 = *(char **)(v3 + 8);
      v1 = stdout;
    }
    stdout = v1;
    if (!v4) break;
    v3 += 8;
    v2 = *v4;
  }
  fputs_unlocked(dcgettext(NULL,"\nThe default output format is a somewhat human-readable representation of\nthe changes.\n\nThe -e, -E, -x, -X (and corresponding long) options cause an ed script\nto be output instead of the default.\n\nFinally, the -m (--merge) option causes diff3 to do the merge internally\nand output the actual merged file.  For unusual input, this is more\nrobust than using ed.\n",5),v1);
  v4 = dcgettext(NULL,"Exit status is 0 if successful, 1 if conflicts, 2 if trouble.",5);
  __printf_chk(1,"\n%s\n%s\n",dcgettext(NULL,"If a FILE is \'-\', read standard input.",5),v4);
  sub_80c0(); // tail-call
}


// Function: sub_43a0 @ 0x43a0
void sub_43a0(char *a0)
{
  error(2,*__errno_location(),"%s",a0);
}


// Function: sub_43d0 @ 0x43d0
long sub_43d0(char *a0,char *a1,unsigned long *a2)
{
  char *v1;
  long v10; // rbx
  char *v11 [9]; // stack - 0x120
  int v12; // stack - 0x130
  stat v13; // stack - 0xd8
  unsigned int v14; // stack - 0x134
  char *v15; // stack - 0x128
  char *v16; // rsi
  long v17; // r14
  int v18; // stack - 0x12c
  int v2; // eax
  char **v3;
  char **v4;
  unsigned long v5;
  void *v6; // rax
  long v7; // rax
  long v8; // rax
  unsigned int v9; // ebx
  
  v15 = dat_13010;
  v3 = v11;
  if (dat_130f2) {
    v11[0] = "-a";
    v3 = &v11[1];
  }
  v4 = v3;
  if (dat_130f1) {
    v4 = &v3[1];
    *v3 = "--strip-trailing-cr";
  }
  v4[2] = a0;
  *v4 = "--horizon-lines=100";
  v4[1] = "--";
  v4[3] = a1;
  v4[4] = NULL;
  if (pipe(&v12))
    sub_43a0("pipe"); // no-return
  v2 = fork();
  if (!v2) {
    close(v12);
    if (v18 != 1) {
      dup2(v18,1);
      close(v18);
    }
    execvp(dat_13010,&v15);
    _exit((*__errno_location() == 2) + 0x7e); // no-return
  }
  if (v2 != -1) {
    close(v18);
    if (fstat(v12,&v13))
      sub_43a0("fstat"); // no-return
    v5 = 1;
    if (1 <= (long)v13._56_8_)
      v5 = v13._56_8_;
    v10 = 0;
    v6 = (void *)sub_8180(v5);
    while( true ) {
      v17 = v5 - v10;
      v7 = sub_8770(v12,(void *)((long)v6 + v10),v17);
      v10 += v7;
      if (v17 != v7) break;
      if (0x3ffffffffffffffe < v5)
        sub_8530(); // no-return
      v5 *= 2;
      v6 = (void *)sub_81d0(v6,v5);
    }
    if (v7 == -1)
      sub_43a0(dcgettext(NULL,"read failed",5)); // no-return
    if ((v10) && (*(char *)((long)v6 + v10 + -1) != '\n'))
      sub_4210("invalid diff format; incomplete last line"); // no-return
    *a2 = v6;
    if (close(v12))
      sub_43a0("close"); // no-return
    v2 = waitpid(v2,(int *)&v14,0);
    v1 = dat_13010;
    if (v2 <= -1)
      sub_43a0("waitpid"); // no-return
    if (v14 & 0x7f) { // branch-flip
      v9 = 0x7fffffff;
      v16 = "subsidiary program \'%s\' failed";
    }
    else {
      if (!((int)v14 >> 8 & 0xfeU))
        return (long)v6 + v10;
      v9 = (int)v14 >> 8 & 0xff;
      if ((char)(v14 >> 8) != '~') { // branch-flip
        v16 = "subsidiary program \'%s\' failed (exit status %d)";
        if (v9 == 0x7f)
          v16 = "subsidiary program \'%s\' not found";
      }
      else {
        v16 = "subsidiary program \'%s\' could not be invoked";
      }
    }
    error(2,0,dcgettext(NULL,v16,5),v1,(unsigned long)v9);
    return v8;
  }
  sub_43a0("fork"); // no-return
}


// Function: sub_46d0 @ 0x46d0
void sub_46d0(FILE *a0,bool a1,long a2,long a3)
{
  fputs_unlocked(".\n",a0);
  if (!a1)
    return;
  if (a3 != 1) {
    __fprintf_chk(a0,1,"%ld,%lds/^\\.//\n",a2,a2 + -1 + a3); // tail-call
    return;
  }
  __fprintf_chk(a0,1,"%lds/^\\.//\n",a2); // tail-call
}


// Function: sub_4770 @ 0x4770
char * sub_4770(char *a0,unsigned long *a1,long *a2,char *a3,char a4)
{
  char v1;
  char *v2;
  char *v3;
  
  if ((*a0 == a4) && (a0[1] == ' ')) {
    a0 = &a0[2];
    *a1 = (unsigned long)a0;
    v2 = a0;
    while (v3 = &v2[1], *v2 != '\n') {
      v2 = v3;
    }
    *a2 = (long)v3 - (long)a0;
    if ((v3 < a3) && (*v3 == '\\')) {
      if (dat_130f0) // branch-flip
        __fprintf_chk(stderr,1,"%s:",dat_23140);
      else {
        *a2 = ((long)v3 - (long)a0) + -1;
      }
      v2 = &v2[2];
      v1 = dat_130f0;
      while( true ) {
        if (v1)
          putc_unlocked((int)*v2,stderr);
        v3 = &v2[1];
        if (*v2 == '\n') break;
        v2 = v3;
        v1 = dat_130f0;
      }
    }
    return v3;
  }
  sub_4210("invalid diff format; incorrect leading line chars"); // no-return
}


// Function: sub_4850 @ 0x4850
unsigned int * sub_4850(long a0,long a1,long a2,long a3,unsigned long a4,unsigned long a5)
{
  unsigned int *v1; // rax
  unsigned long v2; // rax
  unsigned long v3;
  
  v1 = (unsigned int *)sub_8180(0x70);
  *(long *)&v1[4] = a1;
  *v1 = 0;
  *(unsigned long *)&v1[0x1a] = 0;
  *(long *)&v1[2] = a0;
  *(long *)&v1[6] = a2;
  *(long *)&v1[8] = a3;
  *(unsigned long *)&v1[10] = a4;
  *(unsigned long *)&v1[0xc] = a5;
  v3 = (a1 - a0) + 1;
  if (v3) { // branch-flip
    *(unsigned long *)&v1[0xe] = sub_8410(v3,8);
    v2 = sub_8410(v3,8);
    *(unsigned long *)&v1[0x14] = v2;
    a3 = *(long *)&v1[8] - *(long *)&v1[6];
  }
  else {
    a3 -= a2;
    *(unsigned long *)&v1[0xe] = 0;
    *(unsigned long *)&v1[0x14] = 0;
  }
  v3 = a3 + 1;
  if (v3) { // branch-flip
    *(unsigned long *)&v1[0x10] = sub_8410(v3,8);
    v2 = sub_8410(v3,8);
  }
  else {
    *(unsigned long *)&v1[0x10] = 0;
    v2 = 0;
  }
  *(unsigned long *)&v1[0x16] = v2;
  v3 = (*(long *)&v1[0xc] - *(long *)&v1[10]) + 1;
  if (v3) { // branch-flip
    *(unsigned long *)&v1[0x12] = sub_8410(v3,8);
    v2 = sub_8410(v3,8);
  }
  else {
    *(unsigned long *)&v1[0x12] = 0;
    v2 = 0;
  }
  *(unsigned long *)&v1[0x18] = v2;
  return v1;
}


// Function: sub_49b0 @ 0x49b0
unsigned long sub_49b0(long a0,long a1,long a2,long a3,long a4)
{
  void *v1;
  void *v2;
  unsigned long v3;
  long v4; // rbx
  
  if (!a4)
    return 1;
  v4 = 0;
  while( true ) {
    while( true ) {
      v2 = *(void **)(a2 + v4 * 8);
      v3 = *(unsigned long *)(a1 + v4 * 8);
      if (v2) break;
      *(unsigned long *)(a2 + v4 * 8) = *(unsigned long *)(a0 + v4 * 8);
      *(unsigned long *)(a3 + v4 * 8) = v3;
      v4 += 1;
      if (v4 == a4)
        return 1;
    }
    if (*(unsigned long *)(a3 + v4 * 8) != v3)
      return 0;
    v1 = *(void **)(a0 + v4 * 8);
    if (memcmp(v1,v2,v3)) break;
    v4 += 1;
    if (v4 == a4)
      return 1;
  }
  return 0;
}


// Function: sub_4a50 @ 0x4a50
unsigned long sub_4a50(long a0,long a1,long a2,long a3,long a4)
{
  void *v1;
  void *v2;
  unsigned long v3;
  long v4; // rbx
  
  v4 = 0;
  while( true ) {
    if (a4 == v4)
      return 1;
    v1 = *(void **)(a0 + v4 * 8);
    if (!v1)
      return 0;
    v2 = *(void **)(a2 + v4 * 8);
    if (!v2)
      return 0;
    v3 = *(unsigned long *)(a1 + v4 * 8);
    if (v3 != *(unsigned long *)(a3 + v4 * 8)) break;
    v4 += 1;
    if (memcmp(v1,v2,v3))
      return 0;
  }
  return 0;
}


// Function: sub_4ae0 @ 0x4ae0
unsigned int * sub_4ae0(long *a0,long a1,int a2,int a3,long a4)
{
  long *v1;
  long v10;
  long v11; // rdx
  long v12;
  long *v13;
  long v14 [5]; // stack - 0x68
  long v15;
  long v16; // stack - 0x90
  long *v2;
  long v3;
  long v4;
  long v5;
  unsigned int *v6; // rax
  long v7;
  long v8;
  long v9;
  
  v3 = *(long *)(a0[a2] + 0x10);
  v15 = *(long *)(*(long *)(a1 + (long)a3 * 8) + 0x18);
  v5 = 0;
  do {
    v13 = *(long **)((long)a0 + v5);
    if (v13) { // branch-flip
      *(long *)((long)v14 + v5) = (v3 - v13[2]) + *v13;
      v9 = (v15 - *(long *)(*(long *)(a1 + v5) + 0x18)) + *(long *)(*(long *)(a1 + v5) + 8);
    }
    else {
      *(long *)((long)v14 + v5) = (v3 - *(long *)(a4 + 0x30)) + *(long *)(a4 + 0x10 + v5 * 2);
      v9 = (v15 - *(long *)(a4 + 0x30)) + *(long *)(a4 + 0x10 + v5 * 2);
    }
    *(long *)((long)v14 + v5 + 0x10) = v9;
    v5 += 8;
  } while (v5 != 0x10);
  v6 = (unsigned int *)sub_4850(v14[0],v14[2],v14[1],v14[3],v3);
  v16 = 0;
  while( true ) {
    v15 = a0[v16];
    if (v15) {
      v5 = *(long *)&v6[0x18];
      v9 = *(long *)&v6[0x12];
      do {
        v7 = *(long *)(v15 + 0x10);
        v12 = *(long *)(v15 + 0x18);
        v8 = *(long *)(v15 + 0x38);
        v4 = *(long *)(v15 + 0x28);
        v10 = (v7 - v3) * 8;
        if (!sub_49b0(v4,v8,v10 + v9,v5 + v10,(v12 - v7) + 1))
          return NULL;
        v15 = *(long *)(v15 + 0x40);
      } while (v15);
    }
    if (v16 == 1) break;
    v16 = 1;
  }
  v15 = v14[0];
  v5 = 0;
  while( true ) {
    v13 = *(long **)((long)a0 + v5);
    v7 = 0;
    v9 = v15;
    while( true ) {
      v12 = v14[2] + 1;
      if (v13)
        v12 = *v13;
      if (v12 <= v9) break;
      v9 += 1;
      *(unsigned long *)(*(long *)((long)v6 + v5 + 0x38) + v7) = *(unsigned long *)(*(long *)&v6[0x12] + v7);
      *(unsigned long *)(*(long *)((long)v6 + v5 + 0x50) + v7) = *(unsigned long *)(*(long *)&v6[0x18] + v7);
      v7 += 8;
    }
    if (v13) break;
label_4d8b:
    if (v5 + 8 == 0x10) {
      if (*a0) { // branch-flip
        if (a0[1]) { // branch-flip
          v3 = *(long *)&v6[4];
          v15 = *(long *)&v6[2];
          if ((v3 - v15 != *(long *)&v6[8] - *(long *)&v6[6]) || (v5 = *(long *)&v6[0x16], v9 = *(long *)&v6[0x10], v7 = *(long *)&v6[0x14], v12 = *(long *)&v6[0xe], !sub_4a50(v12,v7,v9,v5,(v3 - v15) + 1)))
            *v6 = 4;
          else {
            *v6 = 7;
          }
        }
        else {
          *v6 = 5;
        }
      }
      else {
        *v6 = 6;
      }
      return v6;
    }
    v15 = *(long *)((long)v14 + v5 + 8);
    v14[2] = *(long *)((long)v14 + v5 + 0x18);
    v5 += 8;
  }
  v9 = *(long *)((long)v6 + v5 + 0x38);
  v7 = *(long *)((long)v6 + v5 + 0x50);
label_4cf0:
  v12 = *v13;
  v8 = v13[1];
  v4 = v13[6];
  v10 = v13[4];
  v11 = (v12 - v15) * 8;
  if (!sub_49b0(v10,v4,v11 + v9,v7 + v11,(v8 - v12) + 1))
    return NULL;
  v1 = &v13[1];
  v2 = &v13[3];
  v13 = (long *)v13[8];
  v12 = (*v2 - v3) * 8;
  v8 = (*v1 + 1) - v15;
  do {
    v12 += 8;
    if (v13) { // branch-flip
      if (*v13 - v15 <= v8) goto label_4cf0;
    }
    else if ((v14[2] + 1) - v15 <= v8) goto label_4d8b;
    *(unsigned long *)(v9 + v8 * 8) = *(unsigned long *)(*(long *)&v6[0x12] + v12);
    *(unsigned long *)(v7 + v8 * 8) = *(unsigned long *)(*(long *)&v6[0x18] + v12);
    v8 += 1;
  } while( true );
}


// Function: sub_4e40 @ 0x4e40
unsigned long sub_4e40(long a0,long a1)
{
  long v1;
  unsigned long v10; // rsi
  long v11;
  unsigned int v12;
  unsigned int v13; // r10d
  long v14; // stack - 0x20
  long v2;
  unsigned int v3;
  unsigned int v4;
  int v5; // eax
  unsigned long v6; // rax
  long v7;
  long *v8;
  long v9 [6]; // stack - 0x50
  
  v11 = 0xdd60;
  v8 = v9;
  v9[5] = a0;
  v9[0] = 0;
  v14 = a1;
  v1 = a1;
  if (!a0) goto label_4fab;
  do {
    v6 = v1;
    v9[4] = 0;
    v9[3] = 0;
    v9[2] = 0;
    v9[1] = 0;
    if (v6) { // branch-flip
      v1 = *(long *)(v6 + 0x10);
      v7 = *(long *)(a0 + 0x10);
      v13 = (unsigned int)(v1 < v7);
      v10 = (unsigned long)(v1 < v7);
      v6 = (unsigned long)(v7 <= v1);
      a0 = v9[v10 + 5];
    }
    else {
      v6 = 1;
      v13 = 0;
      v10 = 0;
    }
    while( true ) {
      v9[4] = 0;
      v9[3] = 0;
      v9[2] = 0;
      v9[1] = 0;
      v1 = *(long *)(a0 + 0x40);
      v7 = *(long *)(a0 + 0x18);
      v9[v10 + 3] = a0;
      v9[v10 + 1] = a0;
      v9[v10 + 5] = v1;
      *(unsigned long *)(a0 + 0x40) = 0;
      v1 = v9[v6 + 5];
      v3 = v13;
      while ((v1 && (*(long *)(v1 + 0x10) <= v7 + 1))) {
        if (v9[v6 + 1]) // branch-flip
          *(long *)(v9[v6 + 3] + 0x40) = v1;
        else {
          v9[v6 + 1] = v1;
        }
        v9[v6 + 3] = v1;
        v2 = *(long *)(v9[v6 + 5] + 0x40);
        *(unsigned long *)(v1 + 0x40) = 0;
        v1 = *(long *)(v1 + 0x18);
        v9[v6 + 5] = v2;
        v5 = v3 ^ 1;
        v4 = v5;
        v12 = v3;
        if (v7 < v1) {
          v7 = v1;
          v4 = v3;
          v12 = v5;
        }
        v5 = v4;
        v6 = (unsigned long)v5;
        v1 = v9[v6 + 5];
        v3 = v12;
      }
      v6 = sub_4ae0(&v9[1],&v9[3],v13,v3,v11);
      if (!v6)
        sub_4210("internal error: screwup in format of diff blocks"); // no-return
      *v8 = v6;
      v8 = (long *)(v6 + 0x68);
      v1 = v14;
      a0 = v9[5];
      v11 = v6;
      if (v9[5]) break;
label_4fab:
      v6 = v1;
      a0 = v6;
      if (!v6) {
        v6 = v9[0];
        return v6;
      }
      v10 = 1;
      v6 = 0;
      v9[4] = 0;
      v9[3] = 0;
      v13 = 1;
      v9[2] = 0;
      v9[1] = 0;
    }
  } while( true );
}


// Function: sub_5030 @ 0x5030
char sub_5030(FILE *a0,long a1,int a2)
{
  long v1;
  char *v2;
  long v3; // rbx
  long v4; // r13
  long v5; // r14
  char v6; // stack - 0x41
  
  v4 = (long)a2;
  v5 = v4 * 0x10 + a1;
  if (*(long *)(v5 + 8) <= *(long *)(v5 + 0x10)) { // branch-flip
    v6 = 0;
    v3 = 0;
    do {
      v1 = v3 * 8;
      v2 = *(char **)(*(long *)(a1 + 0x38 + v4 * 8) + v3 * 8);
      if (*v2 == '.') {
        fputc_unlocked(0x2e,a0);
        v6 = 1;
      }
      v3 += 1;
      fwrite_unlocked(v2,1,*(unsigned long *)(*(long *)(a1 + 0x50 + v4 * 8) + v1),a0);
    } while (v3 <= *(long *)(v5 + 0x10) - *(long *)(v5 + 8));
  }
  else {
    v6 = 0;
  }
  return v6;
}


// Function: sub_50f0 @ 0x50f0
unsigned long sub_50f0(FILE *a0,long a1,int *a2,long a3,char *a4,char *a5,char *a6)
{
  long v1;
  int *v2;
  long v3; // rax
  long v4;
  long v5;
  unsigned int v6; // eax
  int v7;
  unsigned long v8; // r14
  
  v2 = (int *)sub_4180(a1);
  if (v2) { // branch-flip
    v8 = 0;
label_515f:
    do {
      if (*v2 != 4) { // branch-flip
        v7 = *(int *)(a3 + (unsigned long)(unsigned int)(*v2 - 5) * 4);
        if (v7 != 1) { // branch-flip
          if (v7 != 2) { // branch-flip
            if (v7 == -1) goto label_5167;
          }
          else if (!dat_130ec) {
            v5 = *(long *)&v2[(long)*a2 * 4 + 2];
            v4 = *(long *)&v2[(long)*a2 * 4 + 4];
            goto label_518f;
          }
        }
        else if (dat_130eb) {
          v7 = 6;
          v4 = *(long *)&v2[(long)*a2 * 4 + 4];
          v5 = *(long *)&v2[(long)*a2 * 4 + 2];
          __fprintf_chk(a0,1,"%lda\n",v4);
          v6 = 0;
          goto label_52b9;
        }
      }
      else {
label_5167:
        if (!dat_130ed) {
          v5 = *(long *)&v2[(long)*a2 * 4 + 2];
          v4 = *(long *)&v2[(long)*a2 * 4 + 4];
          if (dat_130ef) { // branch-flip
            __fprintf_chk(a0,1,"%lda\n",v4);
            v6 = (unsigned int)dat_130eb;
            if (dat_130eb) {
              __fprintf_chk(a0,1,"||||||| %s\n",a5);
              v6 = sub_5030(a0,v2,a2[1]);
            }
            fputs_unlocked("=======\n",a0);
            v7 = a2[2];
            v6 = (v6 | sub_5030(a0,v2,v7)) & 0xff;
            v7 = 4;
label_52b9:
            __fprintf_chk(a0,1,">>>>>>> %s\n",a6);
            sub_46d0(a0,v6,v4 + 2,(*(long *)&v2[(long)a2[1] * 4 + 4] - *(long *)&v2[(long)a2[1] * 4 + 2]) + 3 + (*(long *)&v2[(long)a2[2] * 4 + 4] - *(long *)&v2[(long)a2[2] * 4 + 2]));
            if (v7 != 4) { // branch-flip
              __fprintf_chk(a0,1,"%lda\n<<<<<<< %s\n",v5 + -1,a5);
              v6 = 0;
              if (v7 == 6) {
                v6 = sub_5030(a0,v2,a2[1]);
                fputs_unlocked("=======\n",a0);
                v6 &= 0xff;
              }
            }
            else {
              __fprintf_chk(a0,1,"%lda\n<<<<<<< %s\n",v5 + -1,a4);
              v6 = 0;
            }
            v8 = 1;
            sub_46d0(a0,v6,v5 + 1,(*(long *)&v2[(long)a2[1] * 4 + 4] - *(long *)&v2[(long)a2[1] * 4 + 2]) + 1);
          }
          else {
label_518f:
            if (*(long *)&v2[(long)a2[2] * 4 + 4] - *(long *)&v2[(long)a2[2] * 4 + 2] != -1) {
              if (v4 - v5 != -1) { // branch-flip
                if (v4 != v5) // branch-flip
                  __fprintf_chk(a0,1,"%ld,%ldc\n",v5,v4);
                else {
                  __fprintf_chk(a0,1,"%ldc\n",v4);
                }
              }
              else {
                __fprintf_chk(a0,1,"%lda\n",v4);
              }
              v3 = (long)a2[2];
              v4 = *(long *)&v2[v3 * 4 + 4];
              v1 = *(long *)&v2[v3 * 4 + 2];
              sub_46d0(a0,(unsigned char)sub_5030(a0,v2,v3),v5,(v4 - v1) + 1);
              v2 = *(int **)&v2[0x1a];
              if (!v2) break;
              goto label_515f;
            }
            if (v5 != v4) // branch-flip
              __fprintf_chk(a0,1,"%ld,%ldd\n",v5,v4);
            else {
              __fprintf_chk(a0,1,"%ldd\n",v5);
            }
          }
        }
      }
      v2 = *(int **)&v2[0x1a];
    } while (v2);
  }
  else {
    v8 = 0;
  }
  if (dat_130ea)
    fputs_unlocked("w\nq\n",a0);
  return v8;
}


// Function: sub_5520 @ 0x5520
void sub_5520(FILE *a0,int *a1,long a2,long a3) // return-dupe
{
  long v1;
  long v10; // r12
  int v11; // stack - 0x58
  unsigned int v12; // stack - 0x54
  long v2;
  void *v3;
  unsigned long v4;
  int v5;
  unsigned int v6;
  char *v7; // rax
  char v8 [2]; // stack - 0x42
  long v9; // rbp
  
  v7 = "  ";
  if (dat_130ee)
    v7 = "\t";
  if (a1) {
    do {
      if (*a1 != 4) { // branch-flip
        v6 = *a1 - 5;
        if (3 <= v6)
          sub_4210("internal error: invalid diff type passed to output"); // no-return
        v8[1] = 0;
        v11 = *(int *)(a3 + (unsigned long)v6 * 4);
        v8[0] = (char)v11 + '1';
        v12 = (unsigned int)(v11 == 0);
      }
      else {
        v8[0] = '\0';
        v12 = 3;
        v11 = 3;
      }
      __fprintf_chk(a0,1,"====%s\n",v8);
      v6 = 0;
      do {
        v9 = (long)*(int *)(a2 + (long)(int)v6 * 4);
        v1 = *(long *)&a1[v9 * 4 + 2];
        v2 = *(long *)&a1[v9 * 4 + 4];
        v5 = v6 + 1;
        __fprintf_chk(a0,1,"%d:",v5);
        if (v1 != v2) { // branch-flip
          if (v1 - v2 != 1) // branch-flip
            __fprintf_chk(a0,1,"%ld,%ldc\n",v1,v2);
          else {
            __fprintf_chk(a0,1,"%lda\n",v1 + -1);
          }
        }
        else {
          __fprintf_chk(a0,1,"%ldc\n",v1);
        }
        if ((v1 <= v2) && (v12 != v6)) {
          v10 = 0;
          do {
            fputs_unlocked(v7,a0);
            v3 = *(void **)(*(long *)&a1[(v9 + 6) * 2 + 2] + v10 * 8);
            v4 = *(unsigned long *)(*(long *)&a1[v9 * 2 + 0x14] + v10 * 8);
            v10 += 1;
            fwrite_unlocked(v3,1,v4,a0);
          } while (v10 <= v2 - v1);
          if (*(char *)((long)v3 + (v4 - 1)) != '\n')
            __fprintf_chk(a0,1,"\n\\ %s\n",dcgettext(NULL,"No newline at end of file",5));
        }
        if (v11 == 1)
          v5 = *(unsigned int *)((long)(int)v6 * 4 + 0xdd40);
        v6 = v5;
      } while (v5 <= 2);
      a1 = *(int **)&a1[0x1a];
    } while (a1);
    return;
  }
}


// Function: sub_57d0 @ 0x57d0
char sub_57d0(FILE *a0,FILE *a1,int *a2,int *a3,long a4,char *a5,unsigned long a6,char *a7)
{
  long v1;
  long v10; // stack - 0x58
  int v11; // stack - 0x50
  bool v12; // stack - 0x4a
  char v13; // stack - 0x49
  long v2;
  unsigned int v3;
  int v4; // eax
  int v5; // eax
  long v6;
  long v7; // rdx
  bool v8; // cf
  char *v9; // stack - 0x60
  
  if (a2) {
    v6 = 0;
    v13 = 0;
    v3 = *a2;
    if (v3 == 4) goto label_584a;
    do {
      v3 = *(int *)(a4 + (unsigned long)(unsigned int)(v3 - 5) * 4);
      if (v3 != 1) { // branch-flip
        if (v3 != 2) { // branch-flip
          if (v3 == -1) goto label_584a;
        }
        else {
          v12 = dat_130ec;
          if (!dat_130ec) {
            v11 = 7;
            v10 = *(long *)&a2[2] + -1;
            v6 = (*(long *)&a2[2] - v6) + -2;
            v9 = "<<<<<<< %s\n";
            if (0 <= v6) goto label_5898;
            goto label_58fa;
          }
        }
      }
      else {
        v12 = dat_130eb;
        if (dat_130eb) {
          v11 = 6;
          v10 = *(long *)&a2[2] + -1;
          v6 = (*(long *)&a2[2] - v6) + -2;
          v9 = "<<<<<<< %s\n";
          if (0 <= v6) goto label_5898;
          goto label_5aef;
        }
      }
      while( true ) {
        a2 = *(int **)&a2[0x1a];
        if (!a2) goto label_59ac;
        v3 = *a2;
        if (v3 != 4) break;
label_584a:
        if (!dat_130ed) {
          v12 = dat_130ef;
          v10 = *(long *)&a2[2] + -1;
          v6 = (*(long *)&a2[2] - v6) + -2;
          if (0 <= v6) { // branch-flip
            v11 = 4;
            v9 = "||||||| %s\n";
label_5898:
            do {
              while (v3 = getc_unlocked(a0), v3 == -1) {
                if (ferror_unlocked(a0)) {
                  sub_43a0(dcgettext(NULL,"read failed",5)); // return-dupe, no-return
                }
                if (feof_unlocked(a0)) {
                  sub_4210("input file shrank"); // return-dupe, no-return
                }
                putc_unlocked(-1,a1);
              }
              putc_unlocked(v3,a1);
            } while ((v3 != 10) || (v8 = v6 != 0, v6 = v6 + -1, v8));
            if (v12) {
              if (v11 == 4) goto label_5b9f;
label_58d4:
              if (dat_130eb) {
label_5aef:
                __fprintf_chk(a1,1,v9,a6);
                v6 = (long)a3[1];
                if (*(long *)&a2[v6 * 4 + 2] <= *(long *)&a2[v6 * 4 + 4]) {
                  v7 = 0;
                  do {
                    v1 = v7 * 8;
                    v2 = v7 * 8;
                    v7 += 1;
                    fwrite_unlocked(*(void **)(*(long *)&a2[v6 * 2 + 0xe] + v2),1,*(unsigned long *)(*(long *)&a2[v6 * 2 + 0x14] + v1),a1);
                    v6 = (long)a3[1];
                  } while (v7 <= *(long *)&a2[v6 * 4 + 4] - *(long *)&a2[v6 * 4 + 2]);
                }
              }
              fputs_unlocked("=======\n",a1);
              v12 = 1;
              v13 = 1;
            }
          }
          else if (dat_130ef) {
            v9 = "||||||| %s\n";
label_5b9f:
            __fprintf_chk(a1,1,"<<<<<<< %s\n",a5);
            for (v6 = 0; v7 = (long)*a3, v6 <= *(long *)&a2[v7 * 4 + 4] - *(long *)&a2[v7 * 4 + 2]; v6 = v6 + 1) {
              fwrite_unlocked(*(void **)(*(long *)&a2[v7 * 2 + 0xe] + v6 * 8),1,*(unsigned long *)(*(long *)&a2[v7 * 2 + 0x14] + v6 * 8),a1);
            }
            goto label_58d4;
          }
label_58fa:
          v6 = (long)a3[2];
          if (*(long *)&a2[v6 * 4 + 2] <= *(long *)&a2[v6 * 4 + 4]) {
            v7 = 0;
            do {
              v1 = v7 * 8;
              v2 = v7 * 8;
              v7 += 1;
              fwrite_unlocked(*(void **)(*(long *)&a2[v6 * 2 + 0xe] + v2),1,*(unsigned long *)(*(long *)&a2[v6 * 2 + 0x14] + v1),a1);
              v6 = (long)a3[2];
            } while (v7 <= *(long *)&a2[v6 * 4 + 4] - *(long *)&a2[v6 * 4 + 2]);
          }
          if (v12)
            __fprintf_chk(a1,1,">>>>>>> %s\n",a7);
          v7 = *(long *)&a2[4] - *(long *)&a2[2];
          v6 = v10 + 1 + v7;
          if (0 <= v7) {
            do {
              while (v3 = getc_unlocked(a0), v3 != 10) {
                if (v3 == -1) {
                  if (ferror_unlocked(a0))
                    sub_43a0(dcgettext(NULL,"read failed",5));
                  if (feof_unlocked(a0)) {
                    if ((!v7) && (!*(long *)&a2[0x1a]))
                      return v13;
                    sub_4210("input file shrank");
                  }
                }
              }
              v8 = v7 != 0;
              v7 -= 1;
            } while (v8);
          }
        }
      }
    } while( true );
  }
  v13 = 0;
label_59ac:
  do {
    v3 = getc_unlocked(a0);
    if (v3 == -1) {
      v4 = ferror_unlocked(a0);
      v5 = feof_unlocked(a0);
      if (v4 || v5)
        return v13;
    }
    putc_unlocked(v3,a1);
  } while( true );
}


// Function: sub_5c30 @ 0x5c30
void sub_5c30(void)
{
  if (ferror_unlocked(stdout))
    sub_4210("write failed"); // no-return
  if (!fclose(stdout))
    return;
  sub_43a0(dcgettext(NULL,"standard output",5)); // no-return
}


// Function: sub_5d70 @ 0x5d70
long sub_5d70(char *a0,char *a1,unsigned long *a2,unsigned long *a3)
{
  long v1;
  char *v10; // stack - 0x58
  long v11; // stack - 0x48
  char *v12; // stack - 0x50
  long v13;
  long *v14;
  long *v15; // rax
  unsigned long v2;
  long v3;
  long v4;
  int v5; // eax
  char *v6;
  long v7;
  char *v8;
  char *v9;
  
  v14 = &v11;
  v6 = (char *)sub_43d0(a0,a1,&v10);
  *a3 = v10;
  v12 = v10;
  if (v10 < v6) {
label_5dd0:
    do {
      v15 = (long *)sub_8180(0x48);
      v15[5] = 0;
      v15[4] = 0;
      v15[7] = 0;
      v15[6] = 0;
      v5 = sub_5c80(&v12,v15);
      v9 = dat_23140;
      if ((!v5) || (v8 = &v12[1], *v12 != '\n')) {
        __fprintf_chk(stderr,1,dcgettext(NULL,"%s: diff failed: ",5),v9);
        do {
          v6 = v12;
          putc_unlocked((int)*v12,stderr);
          v12 = &v6[1];
        } while (*v6 != '\n');
        exit(2); // no-return
      }
      v12 = v8;
      if (v5 != 2) { // branch-flip
        if (v5 == 3) {
          v15[2] = v15[2] + 1;
          goto label_5e3d;
        }
        if (v5 != 1)
          sub_4210("internal error: invalid diff type in process_diff"); // no-return
        *v15 = *v15 + 1;
      }
      else {
label_5e3d:
        v3 = v15[1];
        v4 = *v15;
        v2 = (v3 - v4) + 1;
        if (0xffffffffffffffe < v2) {
          sub_8530(); // no-return, return-dupe
        }
        v15[4] = sub_8180(v2 * 8);
        v7 = sub_8180(v2 * 8);
        v15[6] = v7;
        if (v2) {
          v13 = 0;
          while( true ) {
            v8 = (char *)sub_4770(v8,(unsigned long *)(v13 * 8 + v15[4]),(long *)(v7 + v13 * 8),v6,0x3c);
            v12 = v8;
            v1 = v13 + 1;
            if (v3 - v4 == v13) break;
            v7 = v15[6];
            v13 = v1;
          }
        }
        v9 = v12;
        if (v5 != 2) { // branch-flip
          if (v5 == 3) {
            *v14 = (long)v15;
            v14 = &v15[8];
            if (v6 <= v12) break;
            goto label_5dd0;
          }
        }
        else {
          if (strncmp(v12,"---\n",4))
            sub_4210("invalid diff format; invalid change separator"); // no-return
          v12 = &v9[4];
        }
      }
      v3 = v15[3];
      v4 = v15[2];
      v2 = (v3 - v4) + 1;
      if (0xffffffffffffffe < v2)
        sub_8530();
      v15[5] = sub_8180(v2 * 8);
      v7 = sub_8180(v2 * 8);
      v15[7] = v7;
      v9 = v12;
      if (v2) {
        v13 = 0;
        while( true ) {
          v9 = (char *)sub_4770(v9,(unsigned long *)(v13 * 8 + v15[5]),(long *)(v7 + v13 * 8),v6,0x3e);
          v12 = v9;
          v1 = v13 + 1;
          if (v3 - v4 == v13) break;
          v7 = v15[7];
          v13 = v1;
        }
      }
      *v14 = (long)v15;
      v14 = &v15[8];
    } while (v9 < v6);
  }
  *v14 = 0;
  *a2 = v15;
  return v11;
}

