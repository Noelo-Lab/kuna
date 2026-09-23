// Function: sub_3b60 @ 0x3b60
unsigned long sub_3b60(int a0,char **a1) // return-dupe
{
  long *v1;
  unsigned long v10; // rax
  void *v11; // rax
  int v12; // stack - 0x48
  char v13 [32];
  char v14 [32];
  char v15 [32];
  unsigned int v16; // stack - 0xac
  int v17; // stack - 0x44
  char v2; // al
  char v3; // al
  int v4; // eax
  char *v5; // rax
  unsigned long v6; // rax
  unsigned long v7; // rax
  unsigned long v8; // rax
  unsigned long v9; // rax
  
  dat_f020 = 2;
  sub_5960(*a1);
  setlocale(6,"");
  bindtextdomain("diffutils","/usr/local/share/locale");
  textdomain("diffutils");
  sub_58a0(sub_4300);
  sub_7bf0();
  v5 = getenv("EDITOR");
  if (v5)
    dat_f010 = v5;
  sub_45b0("diff");
  while (v4 = getopt_long(a0,a1,"abBdEHiI:lo:stvw:WZ",(void *)0xe860,NULL), v4 != -1) {
    switch(v4) {
      case 0x42:
        sub_45b0("-B");
        break;
      default:
        sub_44c0(0,0); // no-return
      case 0x45:
        sub_45b0("-E");
        break;
      case 0x48:
        sub_45b0("-H");
        break;
      case 0x49:
        sub_45b0("-I");
        sub_45b0(optarg);
        break;
      case 0x57:
        sub_45b0("-w");
        break;
      case 0x5a:
        sub_45b0("-Z");
        break;
      case 0x61:
        sub_45b0("-a");
        break;
      case 0x62:
        sub_45b0("-b");
        break;
      case 100:
        sub_45b0("-d");
        break;
      case 0x69:
        sub_45b0("-i");
        break;
      case 0x6c:
        sub_45b0("--left-column");
        break;
      case 0x6f:
        dat_f168 = optarg;
        break;
      case 0x73:
        dat_f161 = '\x01';
        break;
      case 0x74:
        sub_45b0("-t");
        break;
      case 0x76:
        sub_7690(stdout,"sdiff","GNU diffutils",dat_f018,sub_5c70("Thomas Lord"),0);
        sub_4a10();
        return 0;
      case 0x77:
        sub_45b0("-W");
        sub_45b0(optarg);
        break;
      case 0x80:
        *dat_f5c0 = optarg;
        break;
      case 0x81:
        sub_4380();
        sub_4a10();
        return 0;
      case 0x82:
        sub_45b0("--strip-trailing-cr");
        break;
      case 0x83:
        sub_45b0("--tabsize");
        sub_45b0(optarg);
      
    }
  }
  if (a0 - optind != 2) {
    if (a0 - optind <= 1)
      sub_44c0("missing operand after \'%s\'",a1[(long)a0 + -1]); // no-return
    sub_44c0("extra operand \'%s\'",a1[(long)optind + 2]); // no-return
  }
  if (!dat_f168) {
    if (dat_f161)
      sub_45b0("--suppress-common-lines");
    sub_45b0("-y");
    sub_45b0("--");
    sub_45b0(a1[optind]);
    sub_45b0(a1[(long)optind + 1]);
    sub_45b0(0);
    v1 = dat_f5c0;
    execvp(*dat_f5c0,dat_f5c0);
    sub_49e0(*v1); // no-return
  }
  v2 = sub_4520(a1[optind]);
  v3 = sub_4520(a1[(long)optind + 1]);
  if ((v2) && (v3))
    sub_48d0("both files to be compared are directories"); // no-return
  v6 = sub_4900(a1[optind],v2,a1[(long)optind + 1]);
  v7 = sub_4a60(v6,"r");
  v8 = sub_4900(a1[(long)optind + 1],v3,a1[optind]);
  v9 = sub_4a60(v8,"r");
  v10 = sub_4a60(dat_f168,"w");
  sub_45b0("--sdiff-merge-assist");
  sub_45b0("--");
  sub_45b0(a1[optind]);
  sub_45b0(a1[(long)optind + 1]);
  sub_45b0(0);
  sub_46f0();
  if (pipe(&v12))
    sub_49e0("pipe"); // no-return
  dat_f5a8 = fork();
  if (dat_f5a8 < 0)
    sub_49e0("fork"); // no-return
  if (!dat_f5a8) {
    if (dat_f510 != 1)
      sub_4640(2,1);
    sub_4640(0xd,0);
    close(v12);
    if (v17 != 1) {
      dup2(v17,1);
      close(v17);
    }
    execvp(*dat_f5c0,dat_f5c0);
    _exit((*__errno_location() == 2) + 0x7e); // no-return
  }
  close(v17);
  v11 = fdopen(v12,"r");
  if (v11) {
    sub_4580(v13,v11);
    sub_4580(v14,v7);
    sub_4580(v15,v9);
    v2 = sub_5480(v13,v14,v6,v15,v8,v10);
    sub_4b40(v7);
    sub_4b40(v9);
    sub_4b40(v10);
    sub_4b40(v11);
    while (v4 = waitpid(dat_f5a8,(int *)&v16,0), v4 <= -1) {
      if (*__errno_location() != 4)
        sub_49e0("waitpid"); // no-return
      sub_4870();
    }
    dat_f5a8 = 0;
    if (dat_f5b8) {
      unlink(dat_f5b8);
      dat_f5b8 = NULL;
    }
    if (v2) {
      sub_4e40(v16,1,*dat_f5c0);
      sub_4660(0);
      sub_4870();
      exit(v16 >> 8 & 0xff); // no-return
    }
    sub_48a0(); // no-return
  }
  sub_49e0("fdopen"); // no-return
}


// Function: sub_42e0 @ 0x42e0
void sub_42e0(int a0)
{
  if ((a0 == 2) && (dat_f160))
    return;
  dat_f15c = a0;
}


// Function: sub_4300 @ 0x4300
void sub_4300(void)
{
  if (0 < dat_f5a8) {
    kill(dat_f5a8,0xd);
    if (dat_f5b8) {
      unlink(dat_f5b8); // tail-call
      return;
    }
    return;
  }
  if (dat_f5b8) {
    unlink(dat_f5b8); // tail-call
    return;
  }
}


// Function: sub_4380 @ 0x4380
void sub_4380(void)
{
  void *v1;
  unsigned long v2;
  char v3;
  long v4; // rbx
  char *v5;
  
  v2 = dat_1f620;
  v4 = 0xe780;
  __printf_chk(1,dcgettext(NULL,"Usage: %s [OPTION]... FILE1 FILE2\n",5),v2);
  __printf_chk(1,"%s\n\n",dcgettext(NULL,"Side-by-side merge of differences between FILE1 and FILE2.",5));
  v1 = stdout;
  fputs_unlocked(dcgettext(NULL,"Mandatory arguments to long options are mandatory for short options too.\n",5),v1);
  v3 = '-';
  v5 = "-o, --output=FILE            operate interactively, sending output to FILE";
  while( true ) {
    if (v3) { // branch-flip
      __printf_chk(1,"  %s\n",dcgettext(NULL,v5,5));
      v5 = *(char **)(v4 + 8);
    }
    else {
      putchar_unlocked(10);
      v5 = *(char **)(v4 + 8);
    }
    if (!v5) break;
    v4 += 8;
    v3 = *v5;
  }
  v5 = dcgettext(NULL,"Exit status is 0 if inputs are the same, 1 if different, 2 if trouble.",5);
  __printf_chk(1,"\n%s\n%s\n",dcgettext(NULL,"If a FILE is \'-\', read standard input.",5),v5);
  sub_7740(); // tail-call
}


// Function: sub_44c0 @ 0x44c0
void sub_44c0(char *a0,unsigned long a1)
{
  unsigned long v1;
  
  if (a0)
    error(0,0,dcgettext(NULL,a0,5),a1);
  v1 = dat_1f620;
  error(2,0,dcgettext(NULL,"Try \'%s --help\' for more information.",5),v1);
}


// Function: sub_4520 @ 0x4520
unsigned int sub_4520(char *a0)
{
  unsigned int v1;
  char v2 [24];
  unsigned int v3; // stack - 0x90
  
  v1 = 0;
  if (!stat(a0,v2))
    v1 = CONCAT31((undefined3)((v3 & 0xf000) >> 8),(v3 & 0xf000) == 0x4000);
  return v1;
}


// Function: sub_4580 @ 0x4580
void sub_4580(unsigned long *a0,unsigned long a1)
{
  char *v1; // rax
  
  *a0 = a1;
  v1 = (char *)sub_7800(0x10001);
  a0[3] = v1;
  a0[2] = v1;
  a0[1] = v1;
  *v1 = 10;
}


// Function: sub_45b0 @ 0x45b0
void sub_45b0(unsigned long a0)
{
  long v1;
  
  v1 = dat_f5c0;
  if (dat_f0a8 == dat_f0a0) {
    if (dat_f0a8) { // branch-flip
      if (0x7fffffffffffffe < dat_f0a8)
        sub_7bb0(); // no-return
      dat_f0a0 = dat_f0a8 * 2;
      v1 = dat_f0a8 << 4;
    }
    else {
      dat_f0a0 = 0x10;
      v1 = 0x80;
    }
    v1 = sub_7850(dat_f5c0,v1);
    dat_f5c0 = v1;
  }
  *(unsigned long *)(v1 + dat_f0a8 * 8) = a0;
  dat_f0a8 += 1;
}


// Function: sub_4640 @ 0x4640
void sub_4640(int a0,unsigned long a1)
{
  dat_f0c0 = a1;
  sigaction(a0,(void *)0xf0c0,NULL); // tail-call
}


// Function: sub_4660 @ 0x4660
void sub_4660(int a0)
{
  int v1;
  long *v2;
  long *v3; // rbx
  int *v4; // rbp
  
  if (!dat_f158)
    return;
  v4 = (int *)0xad30;
  v2 = (long *)0xf180;
  do {
    while (((!a0 || (*v4 == a0)) && (*v2 != 1))) {
      v1 = *v4;
      v3 = &v2[0x13];
      v4 = &v4[1];
      sigaction(v1,v2,NULL);
      v2 = v3;
      if (v3 == (long *)0xf5a8)
        return;
    }
    v2 = &v2[0x13];
    v4 = &v4[1];
  } while (v2 != (long *)0xf5a8);
}


// Function: sub_46f0 @ 0x46f0
void sub_46f0(void)
{
  int *v1; // rbx
  long *v2; // rbx
  int *v3; // rbp
  int v4;
  
  v3 = (int *)0xad34;
  v1 = (int *)0xad34;
  dat_f148 = 0x10000000;
  sigemptyset((void *)0xf0c8);
  v4 = 1;
  while( true ) {
    sigaddset((void *)0xf0c8,v4);
    if (v1 == (int *)0xad4c) break;
    v4 = *v1;
    v1 = &v1[1];
  }
  v2 = (long *)0xf180;
  v4 = 1;
  while( true ) {
    sigaction(v4,NULL,v2);
    if (*v2 != 1)
      sub_4640(v4,sub_42e0);
    v2 = &v2[0x13];
    if (v2 == (long *)0xf5a8) break;
    v4 = *v3;
    v3 = &v3[1];
  }
  signal(0x11,0);
  dat_f158 = 1;
}


// Function: sub_47b0 @ 0x47b0
void sub_47b0(void) // return-dupe
{
  char *v1; // rax
  char *v2; // rax
  
  v1 = getenv("TMPDIR");
  if (!v1)
    v1 = "/tmp";
  v2 = (char *)sub_7800(strlen(v1) + 0xd);
  __sprintf_chk(v2,1,0xffffffffffffffff,"%s/sdiffXXXXXX",v1);
  if ((int)mkstemp(v2) <= -1)
    return;
  dat_f5b8 = v2;
}


// Function: sub_4830 @ 0x4830
void sub_4830(void)
{
  __fprintf_chk(stderr,1,"%s",dcgettext(NULL,"ed:\tEdit then use both versions, each decorated with a header.\neb:\tEdit then use both versions.\nel or e1:\tEdit then use the left version.\ner or e2:\tEdit then use the right version.\ne:\tDiscard both versions then edit a new one.\nl or 1:\tUse the left version.\nr or 2:\tUse the right version.\ns:\tSilently include common lines.\nv:\tVerbosely include common lines.\nq:\tQuit.\n",5)); // tail-call
}


// Function: sub_4870 @ 0x4870
void sub_4870(void)
{
  int v1;
  
  v1 = dat_f15c;
  if (!dat_f15c)
    return;
  sub_4300(0);
  sub_4660(v1);
  raise(v1);
  exit(2); // no-return
}


// Function: sub_48a0 @ 0x48a0
void sub_48a0(void)
{
  sub_4300(0);
  sub_4660(0);
  sub_4870();
  exit(2); // no-return
}


// Function: sub_48d0 @ 0x48d0
void sub_48d0(char *a0)
{
  error(0,0,"%s",dcgettext(NULL,a0,5));
  sub_48a0(); // no-return
}


// Function: sub_4900 @ 0x4900
char * sub_4900(char *a0,char a1,unsigned long a2)
{
  long v1;
  void *v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // rax
  char *v5;
  unsigned long v6; // r15
  
  if (strcmp(a0,"-")) {
    v5 = a0;
    if (a1) {
      v6 = 0;
      v2 = (void *)sub_5690(a2);
      v3 = strlen(a0);
      v4 = sub_56f0(v2);
      if (*(char *)sub_5690(a0))
        v6 = (unsigned long)(a0[v3 - 1] != '/');
      v1 = v6 + v3 + v4;
      v5 = (char *)sub_7800(v1 + 1);
      memcpy(v5,a0,v3);
      v5[v3] = '/';
      memcpy(&v5[v6 + v3],v2,v4);
      v5[v1] = '\0';
    }
    return v5;
  }
  sub_48d0("cannot interactively merge standard input"); // no-return
}


// Function: sub_49e0 @ 0x49e0
void sub_49e0(unsigned long a0)
{
  int v1;
  
  v1 = *__errno_location();
  sub_4870();
  error(0,v1,"%s",a0);
  sub_48a0(); // no-return
}


// Function: sub_4a10 @ 0x4a10
void sub_4a10(void)
{
  int v1; // eax
  
  v1 = ferror_unlocked(stdout);
  if (v1)
    sub_48d0("write failed"); // no-return
  v1 = fclose(stdout);
  if (!v1)
    return;
  sub_49e0(dcgettext(NULL,"standard output",5)); // no-return
}


// Function: sub_4a60 @ 0x4a60
void sub_4a60(char *a0,char *a1)
{
  if (fopen(a0,a1))
    return;
  sub_49e0(a0); // no-return
}


// Function: sub_4a80 @ 0x4a80
void sub_4a80(void *a0,unsigned long a1,void *a2)
{
  if (fwrite_unlocked(a0,1,a1,a2) == a1)
    return;
  sub_49e0(dcgettext(NULL,"write failed",5)); // no-return
}


// Function: sub_4ac0 @ 0x4ac0
void sub_4ac0(void *a0)
{
  if (!fflush_unlocked(a0))
    return;
  sub_49e0(dcgettext(NULL,"write failed",5)); // no-return
}


// Function: sub_4af0 @ 0x4af0
void sub_4af0(void)
{
  int v1; // eax
  
  do {
    v1 = getchar_unlocked();
    if (v1 == -1) break;
  } while (v1 != 10);
  v1 = ferror_unlocked(stdin);
  if (!v1)
    return;
  sub_49e0(dcgettext(NULL,"read failed",5)); // no-return
}


// Function: sub_4b40 @ 0x4b40
void sub_4b40(void *a0)
{
  if (!fclose(a0))
    return;
  sub_49e0("fclose"); // no-return
}


// Function: sub_4b60 @ 0x4b60
int sub_4b60(void)
{
  int v1; // eax
  int v2; // eax
  long *v3; // rax
  
  v3 = __ctype_b_loc();
  while( true ) {
    v1 = getchar_unlocked();
    if ((!(*(unsigned char *)(*v3 + 1 + (long)v1 * 2) & 0x20)) || (v1 == 10)) break;
    sub_4870();
  }
  v2 = ferror_unlocked(stdin);
  if (!v2)
    return v1;
  sub_49e0(dcgettext(NULL,"read failed",5)); // no-return
}


// Function: sub_4c30 @ 0x4c30
long sub_4c30(unsigned long *a0)
{
  long v1; // rax
  char *v2; // rdx
  
  v1 = sub_4bd0(a0[2],*a0);
  a0[1] = a0[2];
  v2 = (char *)(a0[2] + v1);
  a0[3] = v2;
  *v2 = 10;
  sub_4870();
  return v1;
}


// Function: sub_4c70 @ 0x4c70
void sub_4c70(long a0,long a1)
{
  void *v1; // rax
  
  do {
    if (!a1)
      return;
    while( true ) {
      v1 = rawmemchr(*(void **)(a0 + 8),10);
      *(void **)(a0 + 8) = v1;
      if (v1 != *(void **)(a0 + 0x18)) break;
      if (!sub_4c30(a0))
        return;
    }
    *(long *)(a0 + 8) = (long)v1 + 1;
    a1 -= 1;
  } while( true );
}


// Function: sub_4cd0 @ 0x4cd0
void sub_4cd0(long a0,long a1,unsigned long a2)
{
  void *v1;
  long v2; // rsi
  void *v3;
  void *v4;
  
  v1 = *(void **)(a0 + 8);
  if (a1) { // branch-flip
    v3 = *(void **)(a0 + 0x18);
    v4 = v1;
    do {
      while( true ) {
        v1 = rawmemchr(v1,10);
        *(void **)(a0 + 8) = v1;
        if (v1 != v3) break;
        sub_4a80(v4,(long)v3 - (long)v4,a2);
        if (!sub_4c30(a0))
          return;
        v1 = *(void **)(a0 + 8);
        v3 = *(void **)(a0 + 0x18);
        v4 = v1;
      }
      v1 = (void *)((long)v1 + 1);
      *(void **)(a0 + 8) = v1;
      a1 -= 1;
    } while (a1);
    v2 = (long)v1 - (long)v4;
    v1 = v4;
  }
  else {
    v2 = 0;
  }
  sub_4a80(v1,v2,a2); // tail-call
}


// Function: sub_4ec0 @ 0x4ec0
int * sub_4ec0(unsigned long a0,unsigned long a1,long a2,long a3,unsigned long a4,unsigned long a5,long a6,long a7,unsigned long a8)
{
  int v1;
  unsigned long v10; // stack - 0x10068
  char *v11; // stack - 0x10060
  unsigned long v12; // stack - 0x10058
  unsigned int v2;
  unsigned long v3;
  unsigned int v4; // eax
  int *v5; // rax
  int v6;
  unsigned long v7; // rbx
  int v8; // stack - 0x1006c
  char v9 [65560];
  
  v3 = a8;
label_4f30:
  v6 = (int)v7;
  v4 = putchar_unlocked(0x25);
label_4f3d:
  if (v4 != 0x25) {
    v5 = (char *)dcgettext(NULL,"write failed",5);
    sub_49e0(v5); // no-return
  }
  sub_4ac0(stdout);
  v4 = sub_4b60();
  v1 = v4;
  if (0x32 < (int)v4) goto label_4fa0;
  if (0x30 < (int)v4) goto label_4fed;
  if (v4 != -1) {
    if (v4 != 10) goto label_4fae;
    goto label_4f6e;
  }
  v4 = feof_unlocked(stdin);
  if (v4) {
    v5 = NULL;
    return v5;
  }
  goto label_4fae;
label_4fa0:
  if (v4 == 0x65) {
    v5 = (unsigned long)sub_4b60();
    v6 = v4;
    v7 = (unsigned long)v5 & 0xffffffff;
    if ((int)v4 <= 0x32) {
      if (0x30 < (int)v4) goto label_5236;
      if (v4 != 10) goto label_4fd5;
label_5092:
      if (dat_f5b8) // branch-flip
        v5 = fopen(dat_f5b8,"w");
      else {
        v4 = sub_47b0();
        if ((int)v4 < 0)
          sub_49e0("mkstemp"); // no-return
        v5 = fdopen(v4,"w");
      }
      dat_f5b0 = v5;
      if (!v5)
        sub_49e0(dat_f5b8); // no-return
      if (v6 != 100) { // branch-flip
        if (0x65 <= v6) { // branch-flip
          if (v6 != 0x6c) goto label_5311;
label_50e2:
          sub_4cd0(a0,a3,v5);
        }
        else {
          if ((v6 == 0x31) || (v6 == 0x62)) goto label_50e2;
label_5311:
          sub_4c70(a0,a3);
        }
        if (0x65 <= v6) { // branch-flip
          if (v6 == 0x72) goto label_5109;
        }
        else if ((v6 == 0x32) || (v6 == 0x62)) goto label_5109;
        sub_4c70(a4,a7);
      }
      else {
        if (a3) {
          if (a3 != 1) { // branch-flip
            __fprintf_chk(v5,1,"--- %s %ld,%ld\n",a1,a2,a2 + -1 + a3);
            v5 = dat_f5b0;
          }
          else {
            __fprintf_chk(v5,1,"--- %s %ld\n",a1,a2);
            v5 = dat_f5b0;
          }
        }
        sub_4cd0(a0,a3,v5);
        if (a7) {
          if (a7 != 1) // branch-flip
            __fprintf_chk(dat_f5b0,1,"+++ %s %ld,%ld\n",a5,a6,a6 + -1 + a7);
          else {
            __fprintf_chk(dat_f5b0,1,"+++ %s %ld\n",a5,a6);
          }
        }
label_5109:
        sub_4cd0(a4,a7,dat_f5b0);
      }
      sub_4b40(dat_f5b0);
      dat_f160 = 1;
      sub_4870();
      v12 = 0;
      v10 = dat_f010;
      v11 = dat_f5b8;
      v4 = fork();
      v6 = v4;
      if (!v4) {
        execvp(dat_f010,&v10);
        v5 = __errno_location();
        _exit((*v5 == 2) + 0x7e); // no-return
      }
      if ((int)v4 <= -1)
        sub_49e0("fork"); // no-return
      while( true ) {
        v4 = waitpid(v6,&v8,0);
        if (0 <= (int)v4) {
          dat_f160 = 0;
          sub_4e40(v8,0,dat_f010);
          v5 = (unsigned long)sub_4a60(dat_f5b8,"r");
          dat_f5b0 = v5;
          while (v5 = (long)sub_4bd0(v9,v5), v5) {
            sub_4870();
            sub_4a80(v9,v5,v3);
            v5 = dat_f5b0;
          }
          sub_4b40(dat_f5b0);
          v5 = (int *)0x1;
          return v5;
        }
        v5 = __errno_location();
        if (*v5 != 4) break;
        sub_4870();
      }
      sub_49e0("waitpid"); // no-return
    }
    v4 -= 0x62;
    v2 = v4;
    v5 = (unsigned long)(unsigned long)v4;
    if ((v2 <= 0x10) && (0x10405UL >> ((unsigned long)v5 & 0x3f) & 1)) {
label_5236:
      v4 = sub_4b60();
      if (v4 == 10) goto label_5092;
    }
label_4fd5:
    sub_4830();
    sub_4af0();
    goto label_4f30;
  }
  if (0x65 <= (int)v4) {
    if ((int)v4 <= 0x73) {
      if ((int)v4 <= 0x70) {
        if (v4 == 0x6c) {
          v4 = sub_4b60();
          if (v4 == 10) {
            sub_4cd0(a0,a3,v3); // return-dupe
            sub_4c70(a4,a7);
            v5 = (int *)0x1;
            return v5;
          }
          goto label_4fd5;
        }
        goto label_4fae;
      }
label_4fed:
      v4 = sub_4b60();
      if (v4 != 10) goto label_4fd5;
      if (v1 <= 0x76) {
        if (0x65 <= v1) {
          switch(v1) {
            case 0x65:
              goto label_5092;
            default:
              goto label_4f6e;
            case 0x6c:
              sub_4cd0(a0,a3,v3);
              sub_4c70(a4,a7);
              v5 = (int *)0x1;
              return v5;
            case 0x71:
              v5 = NULL;
              return v5;
            case 0x72:
              sub_4cd0(a4,a7,v3); // return-dupe
              sub_4c70(a0,a3);
              v5 = (int *)0x1;
              return v5;
            case 0x73:
              dat_f161 = 1;
              break;
            case 0x76:
              dat_f161 = 0;
            
          }
          goto label_4f30;
        }
        if (v1 == 0x31) {
          sub_4cd0(a0,a3,v3);
          sub_4c70(a4,a7);
          v5 = (int *)0x1;
          return v5;
        }
        if (v1 == 0x32) {
          sub_4cd0(a4,a7,v3);
          sub_4c70(a0,a3);
          v5 = (int *)0x1;
          return v5;
        }
      }
      goto label_4f6e;
    }
    if (v4 == 0x76) goto label_4fed;
  }
label_4fae:
  sub_4af0();
label_4f6e:
  sub_4830();
  v4 = putchar_unlocked(0x25);
  goto label_4f3d;
}


// Function: sub_5480 @ 0x5480
unsigned long sub_5480(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5)
{
  int v1;
  long v10; // stack - 0x190
  long v11; // stack - 0x188
  unsigned long v2;
  int *v3; // rax
  long v4; // rax
  long v5; // rax
  char v6; // stack - 0x148
  char *v7; // stack - 0x150
  char v8 [263];
  long v9;
  
  v11 = 1;
  v10 = 1;
  do {
    v2 = sub_4d80(a0,&v6);
    v1 = (int)v2;
    while( true ) {
      if (v1 <= 0)
        return CONCAT71((undefined7)((unsigned long)v2 >> 8),v1 != 0);
      sub_4870();
      if (v6 == ' ') break;
      v3 = __errno_location();
      *v3 = 0;
      v4 = strtoimax(v8,&v7,10);
      if ((((v4 < 0) || (*v3)) || (*v7 != ',')) || (((v5 = strtoimax(&v7[1],&v7,10), v5 < 0 || (*v3)) || (*v7))))
        sub_48d0(&v6); // return-dupe, no-return
      v9 = v5;
      if (v5 <= v4)
        v9 = v4;
      if (v6 != 'c') { // branch-flip
        if (v6 != 'i') {
          sub_48d0(&v6);
        }
        if (dat_f161) // branch-flip
          sub_4c70(a0,v9);
        else {
          sub_4cd0(a0,v9,stdout);
        }
        sub_4cd0(a1,v4,a5);
        sub_4c70(a3,v5);
      }
      else {
        sub_4cd0(a0,v9,stdout);
        v2 = sub_4ec0(a1,a2,v10,v4,a3,a4,v11,v5,a5);
        if (!(char)v2)
          return v2;
      }
      v10 += v4;
      v11 += v5;
      v2 = sub_4d80(a0,&v6);
      v1 = (int)v2;
    }
    puts(v8);
  } while( true );
}

