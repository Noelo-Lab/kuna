// Function: sub_3c09 @ 0x3c09
void sub_3c09(char *a0,unsigned long a1)
{
  unsigned long v1;
  
  if (a0)
    error(0,0,gettext(a0),a1);
  v1 = dat_225e8;
  error(2,0,gettext("Try \'%s --help\' for more information."),v1);
}


// Function: sub_3c7f @ 0x3c7f
void sub_3c7f(void)
{
  int v1; // eax
  
  v1 = ferror_unlocked(stdout);
  if (v1)
    sub_3e69("write failed"); // no-return
  v1 = fclose(stdout);
  if (v1)
    sub_3eab(gettext("standard output")); // no-return
}


// Function: sub_3cd6 @ 0x3cd6
void sub_3cd6(void)
{
  void *v1;
  unsigned long v2;
  char *v3;
  long *v4; // stack - 0x20
  
  v2 = dat_225e8;
  printf(gettext("Usage: %s [OPTION]... FILE1 FILE2\n"),v2);
  printf("%s\n\n",gettext("Side-by-side merge of differences between FILE1 and FILE2."));
  v1 = stdout;
  fputs_unlocked(gettext("Mandatory arguments to long options are mandatory for short options too.\n"),v1);
  for (v4 = (long *)0x11a40; *v4; v4 = &v4[1]) {
    if (*(char *)*v4) { // branch-flip
      v3 = (char *)*v4;
      printf("  %s\n",gettext(v3));
    }
    else {
      putchar_unlocked(10);
    }
  }
  v3 = gettext("Exit status is 0 if inputs are the same, 1 if different, 2 if trouble.");
  printf("\n%s\n%s\n",gettext("If a FILE is \'-\', read standard input."),v3);
  sub_8c7b();
}


// Function: sub_3df5 @ 0x3df5
void sub_3df5(void) // return-dupe
{
  if (1 <= dat_120b8)
    kill(dat_120b8,0xd);
  if (!dat_120a8)
    return;
  unlink(dat_120a8);
}


// Function: sub_3e3e @ 0x3e3e
void sub_3e3e(void)
{
  sub_3df5(0);
  sub_510d(0);
  sub_51e3();
  exit(2); // no-return
}


// Function: sub_3e69 @ 0x3e69
void sub_3e69(char *a0)
{
  error(0,0,"%s",gettext(a0));
  sub_3e3e(); // no-return
}


// Function: sub_3eab @ 0x3eab
void sub_3eab(unsigned long a0)
{
  int v1;
  
  v1 = *__errno_location();
  sub_51e3();
  error(0,v1,"%s",a0);
  sub_3e3e(); // no-return
}


// Function: sub_3ef1 @ 0x3ef1
void sub_3ef1(int a0,unsigned int a1,int a2,unsigned long a3) // ternary
{
  unsigned int v1; // eax
  char *v2; // rax
  
  if ((a0) || (a1 & 0x7f))
    v1 = 0x7fffffff;
  else {
    v1 = (int)a1 >> 8 & 0xff;
  }
  if (a2 < (int)v1) {
    if (v1 != 0x7e) { // branch-flip
      if (v1 != 0x7f) { // branch-flip
        v2 = (v1 != 0x7fffffff) ? "subsidiary program \'%s\' failed (exit status %d)" : "subsidiary program \'%s\' failed"; // branch-flip
      }
      else {
        v2 = "subsidiary program \'%s\' not found";
      }
    }
    else {
      v2 = "subsidiary program \'%s\' could not be invoked";
    }
    error(0,a0,gettext(v2),a3,(unsigned long)v1);
    sub_3e3e(); // no-return
  }
}


// Function: sub_3fa3 @ 0x3fa3
void * sub_3fa3(char *a0,char *a1)
{
  void *v1; // rax
  
  v1 = fopen(a0,a1);
  if (!v1)
    sub_3eab(a0); // no-return
  return v1;
}


// Function: sub_3fe7 @ 0x3fe7
void sub_3fe7(void *a0)
{
  if (fclose(a0))
    sub_3eab("fclose"); // no-return
}


// Function: sub_4019 @ 0x4019
long sub_4019(unsigned long a0,unsigned long a1,void *a2)
{
  long v1; // rax
  
  v1 = fread_unlocked(a0,1,a1,a2);
  if (!v1) {
    if (ferror_unlocked(a2))
      sub_3eab(gettext("read failed")); // no-return
  }
  return v1;
}


// Function: sub_4082 @ 0x4082
void sub_4082(void *a0,unsigned long a1,void *a2)
{
  if (a1 != fwrite_unlocked(a0,1,a1,a2))
    sub_3eab(gettext("write failed")); // no-return
}


// Function: sub_40d3 @ 0x40d3
void sub_40d3(void *a0)
{
  if (fflush_unlocked(a0))
    sub_3eab(gettext("write failed")); // no-return
}


// Function: sub_410d @ 0x410d
char * sub_410d(char *a0,char a1,unsigned long a2) // warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_410d
{
  unsigned char v1; // al
  void *v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // rax
  char *v5;
  
  if (!strcmp(a0,"-"))
    sub_3e69("cannot interactively merge standard input"); // no-return
  if (a1 != '\x01') // branch-flip
    v5 = a0;
  else {
    v2 = (void *)sub_5f32(a2);
    v3 = strlen(a0);
    v4 = sub_5f9f(v2);
    v1 = ((*(char *)sub_5f32(a0)) && (a0[v3 - 1] != '/'));
    v5 = (char *)sub_8dc0(v4 + v1 + v3 + 1);
    memcpy(v5,a0,v3);
    v5[v3] = '/';
    memcpy(&v5[v1 + v3],v2,v4);
    v5[v1 + v3 + v4] = '\0';
  }
  return v5;
}


// Function: sub_425e @ 0x425e
void sub_425e(unsigned long *a0,unsigned long a1)
{
  *a0 = a1;
  a0[3] = sub_8dc0(0x10001);
  a0[2] = a0[3];
  a0[1] = a0[2];
  *(char *)a0[3] = 10;
}


// Function: sub_42c0 @ 0x42c0
long sub_42c0(unsigned long *a0)
{
  long v1; // rax
  
  v1 = sub_4019(a0[2],0x10000,*a0);
  a0[1] = a0[2];
  a0[3] = a0[2] + v1;
  *(char *)a0[3] = 10;
  sub_51e3();
  return v1;
}


// Function: sub_432d @ 0x432d
void sub_432d(long a0,long a1,unsigned long a2)
{
  void *v1;
  long v2; // stack - 0x28
  long v3; // stack - 0x10
  
  v3 = *(long *)(a0 + 8);
  v2 = a1;
  while( true ) {
    while( true ) {
      if (!v2) {
        sub_4082(v3,*(long *)(a0 + 8) - v3,a2);
        return;
      }
      v1 = *(void **)(a0 + 8);
      *(void **)(a0 + 8) = rawmemchr(v1,10);
      if (*(long *)(a0 + 8) == *(long *)(a0 + 0x18)) break;
      v2 -= 1;
      *(long *)(a0 + 8) = *(long *)(a0 + 8) + 1;
    }
    sub_4082(v3,*(long *)(a0 + 0x18) - v3,a2);
    if (!sub_42c0(a0)) break;
    v3 = *(long *)(a0 + 8);
  }
}


// Function: sub_4414 @ 0x4414
void sub_4414(long a0,long a1) // return-dupe
{
  void *v1;
  long v2; // stack - 0x18
  
  v2 = a1;
  do {
    while( true ) {
      if (!v2)
        return;
      v1 = *(void **)(a0 + 8);
      *(void **)(a0 + 8) = rawmemchr(v1,10);
      if (*(long *)(a0 + 8) == *(long *)(a0 + 0x18)) break;
      v2 -= 1;
      *(long *)(a0 + 8) = *(long *)(a0 + 8) + 1;
    }
  } while (sub_42c0(a0));
}


// Function: sub_4493 @ 0x4493
unsigned long sub_4493(long a0,void *a1,unsigned long a2)
{
  void *v1;
  void *v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // stack - 0x40
  void *v5; // stack - 0x38
  
  v4 = a2;
  v5 = a1;
  while( true ) {
    v1 = *(void **)(a0 + 8);
    v2 = rawmemchr(v1,10);
    v3 = (long)v2 - (long)v1;
    if (v4 <= v3)
      return 0;
    memcpy(v5,v1,v3);
    if (v2 < *(void **)(a0 + 0x18)) break;
    if (!sub_42c0(a0)) {
      if (v3)
        return 0;
      return 0xffffffff;
    }
    v5 = (void *)((long)v5 + v3);
    v4 -= v3;
  }
  *(char *)(v3 + (long)v5) = 0;
  *(long *)(a0 + 8) = (long)v2 + 1;
  return 1;
}


// Function: sub_4573 @ 0x4573
unsigned long sub_4573(int a0,char **a1) // return-dupe, ternary
{
  unsigned char v1; // al
  int v10; // stack - 0xbc
  int v11; // stack - 0xb8
  unsigned int v12; // stack - 0xb4
  char *v13; // stack - 0xb0
  unsigned long v14; // stack - 0xa8
  unsigned long v15; // stack - 0xa0
  unsigned long v16; // stack - 0x98
  unsigned long v17; // stack - 0x90
  unsigned long v18; // stack - 0x88
  void *v19; // stack - 0x80
  unsigned char v2; // al
  int v20; // stack - 0x14
  char v3; // al
  int v4; // eax
  unsigned long v5; // rax
  int v6; // stack - 0x18
  char v7 [32];
  char v8 [32];
  char v9 [32];
  
  dat_12020 = 2;
  sub_632f(*a1);
  setlocale(6,"");
  bindtextdomain("diffutils","/usr/local/share/locale");
  textdomain("diffutils");
  sub_626b(sub_3df5);
  sub_93c4();
  v13 = getenv("EDITOR");
  if (v13)
    dat_12010 = v13;
  sub_4ebd("diff");
  while( true ) {
    v11 = getopt_long(a0,a1,"abBdEHiI:lo:stvw:WZ",(void *)0x117a0,NULL);
    if (v11 == -1) break;
    switch(v11) {
      case 0x42:
        sub_4ebd("-B");
        break;
      default:
        sub_3c09(0,0); // no-return
      case 0x45:
        sub_4ebd("-E");
        break;
      case 0x48:
        sub_4ebd("-H");
        break;
      case 0x49:
        sub_4ebd("-I");
        sub_4ebd(optarg);
        break;
      case 0x57:
        sub_4ebd("-w");
        break;
      case 0x5a:
        sub_4ebd("-Z");
        break;
      case 0x61:
        sub_4ebd("-a");
        break;
      case 0x62:
        sub_4ebd("-b");
        break;
      case 100:
        sub_4ebd("-d");
        break;
      case 0x69:
        sub_4ebd("-i");
        break;
      case 0x6c:
        sub_4ebd("--left-column");
        break;
      case 0x6f:
        dat_124e8 = optarg;
        break;
      case 0x73:
        dat_124f0 = '\x01';
        break;
      case 0x74:
        sub_4ebd("-t");
        break;
      case 0x76:
        sub_8b97(stdout,"sdiff","GNU diffutils",dat_12018,sub_68f0("Thomas Lord"),0);
        sub_3c7f();
        return 0;
      case 0x77:
        sub_4ebd("-W");
        sub_4ebd(optarg);
        break;
      case 0x80:
        *dat_120a0 = optarg;
        break;
      case 0x81:
        sub_3cd6();
        sub_3c7f();
        return 0;
      case 0x82:
        sub_4ebd("--strip-trailing-cr");
        break;
      case 0x83:
        sub_4ebd("--tabsize");
        sub_4ebd(optarg);
      
    }
  }
  if (a0 - optind != 2) {
    if (2 <= a0 - optind)
      sub_3c09("extra operand \'%s\'",a1[(long)optind + 2]); // no-return
    sub_3c09("missing operand after \'%s\'",a1[(long)a0 + -1]); // no-return
  }
  if (!dat_124e8) {
    if (dat_124f0)
      sub_4ebd("--suppress-common-lines");
    sub_4ebd("-y");
    sub_4ebd("--");
    sub_4ebd(a1[optind]);
    sub_4ebd(a1[(long)optind + 1]);
    sub_4ebd(0);
    execvp(*dat_120a0,dat_120a0);
    sub_3eab(*dat_120a0); // no-return
  }
  v1 = sub_5e22(a1[optind]);
  v2 = sub_5e22(a1[(long)optind + 1]);
  if (v2 & v1)
    sub_3e69("both files to be compared are directories"); // no-return
  v5 = sub_410d(a1[optind],v1,a1[(long)optind + 1]);
  v14 = v5;
  v15 = sub_3fa3(v5,"r");
  v5 = sub_410d(a1[(long)optind + 1],v2,a1[optind]);
  v16 = v5;
  v17 = sub_3fa3(v5,"r");
  v18 = sub_3fa3(dat_124e8,"w");
  sub_4ebd("--sdiff-merge-assist");
  sub_4ebd("--");
  sub_4ebd(a1[optind]);
  sub_4ebd(a1[(long)optind + 1]);
  sub_4ebd(0);
  sub_4fdd();
  if (pipe(&v6))
    sub_3eab("pipe"); // no-return
  dat_120b8 = fork();
  if (dat_120b8 <= -1)
    sub_3eab("fork"); // no-return
  if (!dat_120b8) {
    if (dat_12450 != 1)
      sub_4fa3(2,1);
    sub_4fa3(0xd,0);
    close(v6);
    if (v20 != 1) {
      dup2(v20,1);
      close(v20);
    }
    execvp(*dat_120a0,dat_120a0);
    v4 = (*__errno_location() != 2) ? 0x7e : 0x7f; // branch-flip
    _exit(v4); // no-return
  }
  close(v20);
  v19 = fdopen(v6,"r");
  if (v19) {
    sub_425e(v7,v19);
    sub_425e(v8,v15);
    sub_425e(v9,v17);
    v3 = sub_5b03(v7,v8,v14,v9,v16,v18);
    sub_3fe7(v15);
    sub_3fe7(v17);
    sub_3fe7(v18);
    v12 = 0;
    sub_3fe7(v19);
    while (v4 = waitpid(dat_120b8,&v10,0), v4 <= -1) {
      if (*__errno_location() != 4)
        sub_3eab("waitpid"); // no-return
      sub_51e3();
    }
    dat_120b8 = 0;
    if (dat_120a8) {
      unlink(dat_120a8);
      dat_120a8 = NULL;
    }
    if (v3 == '\x01') {
      sub_3ef1(v12,v10,1,*dat_120a0);
      sub_510d(0);
      sub_51e3();
      exit(v10 >> 8 & 0xff); // no-return
    }
    sub_3e3e(); // no-return
  }
  sub_3eab("fdopen"); // no-return
}


// Function: sub_4ebd @ 0x4ebd
void sub_4ebd(unsigned long a0)
{
  unsigned long v1;
  
  if (dat_12598 == dat_125a0) {
    if (dat_125a0) { // branch-flip
      if (0x7ffffffffffffff <= dat_125a0)
        sub_9384(); // no-return
      dat_125a0 *= 2;
    }
    else {
      dat_125a0 = 0x10;
    }
    dat_120a0 = sub_8e0c(dat_120a0,dat_125a0 * 8);
  }
  v1 = dat_12598;
  dat_12598 += 1;
  *(unsigned long *)(dat_120a0 + v1 * 8) = a0;
}


// Function: sub_4f78 @ 0x4f78
void sub_4f78(int a0) // return-dupe
{
  if ((a0 == 2) && (dat_124f1 == '\x01'))
    return;
  dat_124f4 = a0;
}


// Function: sub_4fa3 @ 0x4fa3
void sub_4fa3(int a0,unsigned long a1)
{
  dat_12500 = a1;
  sigaction(a0,(void *)0x12500,NULL);
}


// Function: sub_4fdd @ 0x4fdd
void sub_4fdd(void)
{
  int v1; // stack - 0xc
  
  dat_12588 = 0x10000000;
  sigemptyset((void *)0x12508);
  for (v1 = 0; v1 <= 6; v1 = v1 + 1) {
    sigaddset((void *)0x12508,*(int *)((long)v1 * 4 + 0xd020));
  }
  for (v1 = 0; v1 <= 6; v1 = v1 + 1) {
    sigaction(*(int *)((long)v1 * 4 + 0xd020),NULL,(void *)((long)v1 * 0x98 + 0x120c0));
    if (*(long *)((long)v1 * 0x98 + 0x120c0) != 1)
      sub_4fa3(*(unsigned int *)((long)v1 * 4 + 0xd020),sub_4f78);
  }
  signal(0x11,0);
  dat_124f8 = 1;
}


// Function: sub_510d @ 0x510d
void sub_510d(int a0) // return-dupe
{
  int v1; // stack - 0xc
  
  if (!dat_124f8)
    return;
  for (v1 = 0; v1 <= 6; v1 = v1 + 1) {
    if (((!a0) || (a0 == *(int *)((long)v1 * 4 + 0xd020))) && (*(long *)((long)v1 * 0x98 + 0x120c0) != 1))
      sigaction(*(int *)((long)v1 * 4 + 0xd020),(void *)((long)v1 * 0x98 + 0x120c0),NULL);
  }
}


// Function: sub_51e3 @ 0x51e3
void sub_51e3(void)
{
  int v1;
  
  v1 = dat_124f4;
  if (dat_124f4) {
    sub_3df5(0);
    sub_510d(v1);
    raise(v1);
    exit(2); // no-return
  }
}


// Function: sub_5229 @ 0x5229
void sub_5229(void)
{
  fprintf(stderr,"%s",gettext("ed:\tEdit then use both versions, each decorated with a header.\neb:\tEdit then use both versions.\nel or e1:\tEdit then use the left version.\ner or e2:\tEdit then use the right version.\ne:\tDiscard both versions then edit a new one.\nl or 1:\tUse the left version.\nr or 2:\tUse the right version.\ns:\tSilently include common lines.\nv:\tVerbosely include common lines.\nq:\tQuit.\n"));
}


// Function: sub_5264 @ 0x5264
int sub_5264(void)
{
  int v1; // eax
  int v2; // eax
  
  while( true ) {
    v1 = getchar_unlocked();
    if ((!(*(unsigned short *)((long)v1 * 2 + *(long *)__ctype_b_loc()) & 0x2000)) || (v1 == 10)) break;
    sub_51e3();
  }
  v2 = ferror_unlocked(stdin);
  if (v2)
    sub_3eab(gettext("read failed")); // no-return
  return v1;
}


// Function: sub_52d6 @ 0x52d6
void sub_52d6(void)
{
  int v1; // eax
  
  do {
    v1 = getchar_unlocked();
    if (v1 == 10) break;
  } while (v1 != -1);
  v1 = ferror_unlocked(stdin);
  if (v1)
    sub_3eab(gettext("read failed")); // no-return
}


// Function: sub_5326 @ 0x5326
unsigned long sub_5326(unsigned long a0,unsigned long a1,long a2,long a3,unsigned long a4,unsigned long a5,long a6,long a7,unsigned long a8) // ternary
{
  bool v1;
  unsigned int v10; // stack - 0x10068
  int v11; // stack - 0x10064
  long v12; // stack - 0x10060
  long v13; // stack - 0x10058
  long v14; // stack - 0x10050
  long v15; // stack - 0x10048
  long v16; // stack - 0x10040
  char *v17; // stack - 0x10030
  unsigned long v18; // stack - 0x10028
  unsigned long v2;
  int v3; // eax
  unsigned long v4; // stack - 0x10038
  int v5; // stack - 0x10078
  char v6 [65552];
  int v7; // stack - 0x10074
  int v8; // stack - 0x10070
  int v9; // stack - 0x1006c
  
  v2 = a8;
label_5392:
  do {
    v1 = 0;
label_555d:
    if (!v1) {
      if (putchar_unlocked(0x25) != 0x25)
        sub_3eab(gettext("write failed")); // no-return
      sub_40d3(stdout);
      v7 = sub_5264();
      if (v7 == 0x76) {
label_546f:
        if (sub_5264() != 10) { // branch-flip
          sub_5229();
          sub_52d6();
        }
        else {
          v1 = 1;
        }
        goto label_555d;
      }
      if ((0x76 < v7) || (0x73 < v7)) goto label_5552;
      if ((0x71 <= v7) || (v7 == 0x6c)) goto label_546f;
      if (0x6c < v7) goto label_5552;
      if (v7 != 0x65) {
        if ((0x65 < v7) || (0x32 < v7)) goto label_5552;
        if (0x31 <= v7) goto label_546f;
        if (v7 != -1) { // branch-flip
          if (v7 != 10) goto label_5552;
        }
        else {
          v3 = feof_unlocked(stdin);
          if (v3) {
            v1 = 1;
            v7 = 0x71;
            goto label_555d;
          }
label_5552:
          sub_52d6();
        }
        sub_5229();
        goto label_555d;
      }
      v8 = sub_5264();
      if (0x73 <= v8) { // branch-flip
label_551e:
        sub_5229();
        sub_52d6();
      }
      else if (0x62 <= v8) { // branch-flip
        if (!(0x10405UL >> ((char)v8 + 0x9eU & 0x3f) & 1)) goto label_551e;
label_54f6:
        if (sub_5264() == 10) goto label_552a;
        sub_5229();
        sub_52d6();
      }
      else {
        if (v8 != 10) {
          if ((10 <= v8) && ((unsigned int)(v8 - 0x31U) <= 1)) goto label_54f6;
          goto label_551e;
        }
label_552a:
        v1 = 1;
      }
      goto label_555d;
    }
    if (0x76 < v7) {
label_5ae2:
      sub_5229();
      goto label_5392;
    }
    if (v7 <= 100) {
      if (v7 == 0x31) {
        sub_432d(a0,a3,v2); // return-dupe
        sub_4414(a4,a7);
        return 1;
      }
      if (v7 == 0x32) {
        sub_432d(a4,a7,v2); // return-dupe
        sub_4414(a0,a3);
        return 1;
      }
      goto label_5ae2;
    }
    switch(v7) {
      case 0x65:
        goto label_5673;
      default:
        goto label_5ae2;
      case 0x6c:
        sub_432d(a0,a3,v2);
        sub_4414(a4,a7);
        return 1;
      case 0x71:
        return 0;
      case 0x72:
        sub_432d(a4,a7,v2);
        sub_4414(a0,a3);
        return 1;
      case 0x73:
        dat_124f0 = 1;
        goto label_5392;
      case 0x76:
        dat_124f0 = 0;
      
    }
  } while( true );
label_5673:
  if (dat_120a8) // branch-flip
    dat_120b0 = fopen(dat_120a8,"w");
  else {
    v9 = sub_5e9b();
    if (v9 <= -1)
      sub_3eab("mkstemp"); // no-return
    dat_120b0 = fdopen(v9,"w");
  }
  if (!dat_120b0)
    sub_3eab(dat_120a8); // no-return
  if (v8 != 0x6c) { // branch-flip
    if (v8 <= 0x6c) {
      if (v8 != 100) { // branch-flip
        if ((0x65 <= v8) || ((v8 != 0x31 && (v8 != 0x62)))) goto label_580e;
      }
      else if (a3) {
        v13 = a3 + a2 + -1;
        v12 = a2;
        if (a3 != 1) // branch-flip
          fprintf(dat_120b0,"--- %s %ld,%ld\n",a1,a2,v13);
        else {
          fprintf(dat_120b0,"--- %s %ld\n",a1,a2);
        }
      }
      goto label_57ec;
    }
label_580e:
    sub_4414(a0,a3);
  }
  else {
label_57ec:
    sub_432d(a0,a3,dat_120b0);
  }
  if (v8 != 0x72) {
    if (0x73 <= v8) {
label_5927:
      sub_4414(a4,a7);
      goto label_593e;
    }
    if (v8 != 100) { // branch-flip
      if ((0x65 <= v8) || ((v8 != 0x32 && (v8 != 0x62)))) goto label_5927;
    }
    else if (a7) {
      v14 = a6;
      v15 = a7 + a6 + -1;
      if (a7 != 1) // branch-flip
        fprintf(dat_120b0,"+++ %s %ld,%ld\n",a5,a6,v15);
      else {
        fprintf(dat_120b0,"+++ %s %ld\n",a5,a6);
      }
    }
  }
  sub_432d(a4,a7,dat_120b0);
label_593e:
  sub_3fe7(dat_120b0);
  v10 = 0;
  dat_124f1 = 1;
  sub_51e3();
  v4 = dat_12010;
  v17 = dat_120a8;
  v18 = 0;
  v11 = fork();
  if (!v11) {
    execvp(dat_12010,&v4);
    v3 = (*__errno_location() != 2) ? 0x7e : 0x7f; // branch-flip
    _exit(v3); // no-return
  }
  if (0 <= v11) {
    while( true ) {
      v3 = waitpid(v11,&v5,0);
      if (0 <= v3) {
        dat_124f1 = 0;
        sub_3ef1(v10,v5,0,dat_12010);
        dat_120b0 = (unsigned long)sub_3fa3(dat_120a8,"r");
        while( true ) {
          v16 = sub_4019(v6,0x10000,dat_120b0);
          if (!v16) break;
          sub_51e3();
          sub_4082(v6,v16,v2);
        }
        sub_3fe7(dat_120b0);
        return 1;
      }
      if (*__errno_location() != 4) break;
      sub_51e3();
    }
    sub_3eab("waitpid"); // no-return
  }
  sub_3eab("fork"); // no-return
}


// Function: sub_5b03 @ 0x5b03
unsigned long sub_5b03(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5)
{
  char v1; // al
  long v10; // stack - 0x128
  long v11; // stack - 0x120
  unsigned long v2; // rax
  char v3; // stack - 0x118
  char v4 [263];
  char *v5; // stack - 0x150
  long v6; // stack - 0x148
  long v7; // stack - 0x140
  long v8; // stack - 0x138
  long v9; // stack - 0x130
  
  v6 = 1;
  v7 = 1;
  while( true ) {
    while( true ) {
      v2 = sub_4493(a0,&v3,0x100);
      if ((int)v2 <= 0)
        return CONCAT71((undefined7)((unsigned long)v2 >> 8),(int)v2 != 0);
      sub_51e3();
      if (v3 != ' ') break;
      puts(v4);
    }
    *__errno_location() = 0;
    v8 = strtoimax(v4,&v5,10);
    if (((v8 < 0) || (*__errno_location())) || (*v5 != ',')) break;
    v9 = v8;
    v8 = strtoimax(&v5[1],&v5,10);
    if (((v8 < 0) || (*__errno_location())) || (*v5))
      sub_3e69(&v3); // no-return
    v10 = v8;
    v11 = v8;
    if (v8 <= v9)
      v11 = v9;
    if (v3 != 'c') { // branch-flip
      if (v3 != 'i')
        sub_3e69(&v3); // no-return
      if (dat_124f0) // branch-flip
        sub_4414(a0,v11);
      else {
        sub_432d(a0,v11,stdout);
      }
      sub_432d(a1,v9,a5);
      sub_4414(a3,v10);
    }
    else {
      sub_432d(a0,v11,stdout);
      v1 = sub_5326(a1,a2,v6,v9,a3,a4,v7,v10,a5);
      if (v1 != '\x01')
        return 0;
    }
    v6 += v9;
    v7 += v10;
  }
  sub_3e69(&v3); // no-return
}


// Function: sub_5e22 @ 0x5e22
unsigned int sub_5e22(char *a0) // return-dupe
{
  char v1 [24];
  unsigned int v2; // stack - 0x90
  
  if ((!stat(a0,v1)) && ((v2 & 0xf000) == 0x4000))
    return 1;
  return 0;
}


// Function: sub_5e9b @ 0x5e9b
int sub_5e9b(void)
{
  int v1; // eax
  char *v2; // rax
  char *v3; // rax
  
  v2 = getenv("TMPDIR");
  if (!v2)
    v2 = "/tmp";
  v3 = (char *)sub_8dc0(strlen(v2) + 0xd);
  sprintf(v3,"%s/sdiffXXXXXX",v2);
  v1 = mkstemp(v3);
  if (0 <= v1)
    dat_120a8 = v3;
  return v1;
}

