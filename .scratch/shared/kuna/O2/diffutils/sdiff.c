// Function: main @ 0x3b00
int main(int argc,char **argv,char **envp)
{
  char **v1;
  FILE *v10; // rax
  char *v11;
  long v12;
  char *v13; // rax
  long v14; // rax
  int *v15; // rax
  char *v16;
  int v17; // stack - 0x150
  unsigned int v18; // stack - 0x1c0
  char v19; // stack - 0x148
  unsigned char v2; // al
  char v20 [263];
  FILE *v21; // stack - 0x178
  unsigned long v22; // stack - 0x1b8
  unsigned long v23; // stack - 0x198
  unsigned long v24; // rbp
  long v25;
  sigaction *v26; // r14
  unsigned long v27; // r14
  int *v28; // r15
  bool v29;
  unsigned char v3; // al
  long v30; // stack - 0x210
  long v31; // stack - 0x208
  unsigned int v32; // stack - 0x1bc
  char *v33; // stack - 0x1b0
  char *v34; // stack - 0x1a8
  char *v35; // stack - 0x1a0
  char *v36; // stack - 0x190
  char *v37; // stack - 0x188
  char *v38; // stack - 0x180
  char *v39; // stack - 0x170
  int v4;
  char *v40; // stack - 0x168
  char *v41; // stack - 0x160
  int v42; // stack - 0x14c
  char *v5;
  unsigned long v6;
  char *v7; // rax
  unsigned long v8; // rax
  FILE *v9;
  
  dat_10020 = 2;
  sub_5b80(*argv);
  setlocale(6,"");
  bindtextdomain("diffutils","/usr/local/share/locale");
  textdomain("diffutils");
  sub_5ac0(sub_4990);
  sub_82b0();
  v5 = getenv("EDITOR");
  if (v5)
    dat_10010 = v5;
  sub_4900("diff");
  while (v4 = getopt_long(argc,argv,"abBdEHiI:lo:stvw:WZ",(option *)0xf880,NULL), v6 = dat_20620, v4 != -1) {
    switch(v4) {
      case 0x42:
        sub_4900("-B");
        break;
      default:
        sub_4840(NULL,0); // no-return
      case 0x45:
        sub_4900("-E");
        break;
      case 0x48:
        sub_4900("-H");
        break;
      case 0x49:
        sub_4900("-I");
        sub_4900(optarg);
        break;
      case 0x57:
        sub_4900("-w");
        break;
      case 0x5a:
        sub_4900("-Z");
        break;
      case 0x61:
        sub_4900("-a");
        break;
      case 0x62:
        sub_4900("-b");
        break;
      case 100:
        sub_4900("-d");
        break;
      case 0x69:
        sub_4900("-i");
        break;
      case 0x6c:
        sub_4900("--left-column");
        break;
      case 0x6f:
        dat_10168 = optarg;
        break;
      case 0x73:
        dat_10161 = '\x01';
        break;
      case 0x74:
        sub_4900("-t");
        break;
      case 0x76:
        sub_7bf0(stdout,"sdiff","GNU diffutils",dat_10018,sub_6020("Thomas Lord"),0);
        sub_4cc0();
        return 0;
      case 0x77:
        sub_4900("-W");
        sub_4900(optarg);
        break;
      case 0x80:
        *dat_105c0 = optarg;
        break;
      case 0x81:
        v12 = 0xf7a0;
        __printf_chk(1,dcgettext(NULL,"Usage: %s [OPTION]... FILE1 FILE2\n",5),v6);
        __printf_chk(1,"%s\n\n",dcgettext(NULL,"Side-by-side merge of differences between FILE1 and FILE2.",5));
        v9 = stdout;
        fputs_unlocked(dcgettext(NULL,"Mandatory arguments to long options are mandatory for short options too.\n",5),v9);
        v5 = "-o, --output=FILE            operate interactively, sending output to FILE";
        do {
          if (*v5) { // branch-flip
            __printf_chk(1,"  %s\n",dcgettext(NULL,v5,5));
            v5 = *(char **)(v12 + 8);
          }
          else {
            v11 = *(char **)&stdout->field_0x28;
            if (*(char **)&stdout->field_0x30 <= v11) // branch-flip
              __overflow(stdout,10);
            else {
              *(char **)&stdout->field_0x28 = &v11[1];
              *v11 = 10;
            }
            v5 = *(char **)(v12 + 8);
          }
          v12 += 8;
        } while (v5);
        v5 = dcgettext(NULL,"Exit status is 0 if inputs are the same, 1 if different, 2 if trouble.",5);
        __printf_chk(1,"\n%s\n%s\n",dcgettext(NULL,"If a FILE is \'-\', read standard input.",5),v5);
        sub_7cd0();
        sub_4cc0();
        return 0;
      case 0x82:
        sub_4900("--strip-trailing-cr");
        break;
      case 0x83:
        sub_4900("--tabsize");
        sub_4900(optarg);
      
    }
  }
  if (argc - optind != 2) {
    if (2 <= argc - optind)
      sub_4840("extra operand \'%s\'",argv[(long)optind + 2]); // no-return
    sub_4840("missing operand after \'%s\'",argv[(long)argc + -1]); // no-return
  }
  if (!dat_10168) {
    if (dat_10161)
      sub_4900("--suppress-common-lines");
    sub_4900("-y");
    sub_4900("--");
    sub_4900(argv[optind]);
    sub_4900(argv[(long)optind + 1]);
    sub_4900(0);
    v1 = dat_105c0;
    execvp(*dat_105c0,dat_105c0);
    sub_4b30(*v1); // no-return
  }
  v2 = sub_48a0(argv[optind]);
  v3 = sub_48a0(argv[(long)optind + 1]);
  if (v2 & v3)
    sub_4b00("both files to be compared are directories",v2); // no-return
  v5 = (char *)sub_4be0(argv[optind],v2,argv[(long)optind + 1]);
  v6 = sub_4b60(v5,"r");
  v7 = (char *)sub_4be0(argv[(long)optind + 1],v3,argv[optind]);
  v8 = sub_4b60(v7,"r");
  v9 = (FILE *)sub_4b60(dat_10168,"w");
  sub_4900("--sdiff-merge-assist");
  sub_4900("--");
  sub_4900(argv[optind]);
  v15 = (int *)0xbd30;
  v28 = (int *)0xbd30;
  sub_4900(argv[(long)optind + 1]);
  sub_4900(0);
  dat_10148 = 0x10000000;
  sigemptyset((sigset_t *)0x100c8);
  do {
    v4 = *v28;
    v28 = &v28[1];
    sigaddset((sigset_t *)0x100c8,v4);
  } while (v28 != (int *)0xbd4c);
  v26 = (sigaction *)0x10180;
  do {
    v4 = *v15;
    sigaction(v4,NULL,v26);
    if (*(long *)v26 != 1) {
      dat_100c0 = sub_4820;
      sigaction(v4,(sigaction *)0x100c0,NULL);
    }
    v26 = &v26[1];
    v15 = &v15[1];
  } while (v26 != (sigaction *)0x105a8);
  signal(0x11,0);
  dat_10158 = '\x01';
  if (pipe(&v17))
    sub_4b30("pipe"); // no-return
  dat_105a8 = fork();
  if (dat_105a8 < 0)
    sub_4b30("fork"); // no-return
  if (!dat_105a8) {
    if (dat_10510 != 1) {
      dat_100c0 = (void *)0x1;
      sigaction(2,(sigaction *)0x100c0,NULL);
    }
    dat_100c0 = NULL;
    sigaction(0xd,(sigaction *)0x100c0,NULL);
    close(v17);
    if (v42 != 1) {
      dup2(v42,1);
      close(v42);
    }
    execvp(*dat_105c0,dat_105c0);
    _exit((*__errno_location() == 2) + 0x7e); // no-return
  }
  close(v42);
  v10 = fdopen(v17,"r");
  if (!v10)
    sub_4b30("fdopen"); // no-return
  v21 = v10;
  v11 = (char *)sub_7df0(0x10001);
  *v11 = 10;
  v41 = v11;
  v40 = v11;
  v39 = v11;
  v22 = v6;
  v35 = (char *)sub_7df0(0x10001);
  *v35 = 10;
  v34 = v35;
  v33 = v35;
  v23 = v8;
  v38 = (char *)sub_7df0(0x10001);
  v31 = 1;
  v37 = v38;
  v36 = v38;
  *v38 = 10;
  v30 = 1;
  while( true ) {
    while( true ) {
      v24 = 0x100;
      v16 = &v19;
      while( true ) {
        v13 = rawmemchr(v11,10);
        v27 = (long)v13 - (long)v11;
        if (v24 <= v27) {
          v29 = 0;
          goto label_441a;
        }
        memcpy(v16,v11,v27);
        v11 = v40;
        v4 = dat_1015c;
        if (v13 < v41) break;
        v12 = sub_4d80(v40,v21);
        v4 = dat_1015c;
        v39 = v11;
        v41 = &v11[v12];
        *v41 = 10;
        if (dat_1015c) {
          if (1 <= dat_105a8)
            kill(dat_105a8,0xd);
          if (dat_105b8)
            unlink(dat_105b8);
          if (dat_10158)
            sub_4a10(v4);
          raise(v4);
          exit(2); // no-return
        }
        if (!v12) {
          v29 = v27 == 0;
          goto label_441a;
        }
        v16 = &v16[v27];
        v24 -= v27;
      }
      v11 = &v13[1];
      v16[v27] = '\0';
      v39 = v11;
      if (dat_1015c) {
        if (1 <= dat_105a8)
          kill(dat_105a8,0xd);
        if (dat_105b8)
          unlink(dat_105b8);
        if (dat_10158)
          sub_4a10(v4);
        raise(v4);
        exit(2); // no-return
      }
      if (v19 != ' ') break;
      puts(v20);
    }
    v15 = __errno_location();
    *v15 = 0;
    v12 = strtoimax(v20,&v18,10);
    if ((((v12 <= -1) || (*v15)) || (*(char *)CONCAT44(v32,v18) != ',')) || (((v14 = strtoimax(&((char *)CONCAT44(v32,v18))[1],&v18,10), v14 <= -1 || (*v15)) || (*(char *)CONCAT44(v32,v18))))) break;
    v25 = v14;
    if (v14 <= v12)
      v25 = v12;
    if (v19 != 'c') { // branch-flip
      if (v19 != 'i') break;
      if (dat_10161) // branch-flip
        sub_4f30(&v21,v25);
      else {
        sub_5000(&v21,v25,stdout);
      }
      sub_5000(&v22,v12,v9);
      sub_4f30(&v23,v14);
    }
    else {
      sub_5000(&v21,v25,stdout);
      if (!sub_5120(&v22,v5,v30,v12,&v23,v7,v31,v14,v9)) {
        v29 = 0;
label_441a:
        sub_4bc0(v6);
        sub_4bc0(v8);
        sub_4bc0(v9);
        sub_4bc0(v10);
        while (waitpid(dat_105a8,(int *)&v18,0) <= -1) {
          if (*__errno_location() != 4)
            sub_4b30("waitpid"); // no-return
          sub_4a90();
        }
        dat_105a8 = 0;
        if (dat_105b8) {
          unlink(dat_105b8);
          dat_105b8 = NULL;
        }
        if (v29) {
          sub_4dc0(v18,1,*dat_105c0);
          if (dat_10158)
            sub_4a10(0);
          sub_4a90();
          exit(v18 >> 8 & 0xff); // no-return
        }
        sub_4ad0(); // no-return
      }
    }
    v31 += v14;
    v30 += v12;
    v11 = v39;
  }
  sub_4b00(&v19); // no-return
}


// Function: sub_4820 @ 0x4820
void sub_4820(int a0)
{
  if ((a0 == 2) && (dat_10160))
    return;
  dat_1015c = a0;
}


// Function: sub_4840 @ 0x4840
void sub_4840(char *a0,unsigned long a1)
{
  char *v1;
  
  if (a0)
    error(0,0,dcgettext(NULL,a0,5),a1);
  v1 = dat_20620;
  error(2,0,dcgettext(NULL,"Try \'%s --help\' for more information.",5),v1);
}


// Function: sub_48a0 @ 0x48a0
unsigned int sub_48a0(char *a0)
{
  unsigned int v1;
  stat v2; // stack - 0xa8
  
  v1 = 0;
  if (!stat(a0,&v2))
    v1 = CONCAT31((undefined3)((v2._24_4_ & 0xf000) >> 8),(v2._24_4_ & 0xf000) == 0x4000);
  return v1;
}


// Function: sub_4900 @ 0x4900
void sub_4900(unsigned long a0)
{
  long v1;
  unsigned long v2; // rsi
  
  v1 = dat_105c0;
  if (dat_100a8 == dat_100a0) {
    if (dat_100a8) { // branch-flip
      if (0x7fffffffffffffe < dat_100a8)
        sub_8270(); // no-return
      dat_100a0 = dat_100a8 * 2;
      v2 = dat_100a8 << 4;
    }
    else {
      dat_100a0 = 0x10;
      v2 = 0x80;
    }
    v1 = sub_7e40(dat_105c0,v2);
    dat_105c0 = v1;
  }
  *(unsigned long *)(v1 + dat_100a8 * 8) = a0;
  dat_100a8 += 1;
}


// Function: sub_4ad0 @ 0x4ad0
void sub_4ad0(void)
{
  sub_4990(0);
  if (dat_10158)
    sub_4a10(0);
  sub_4a90();
  exit(2); // no-return
}


// Function: sub_4b00 @ 0x4b00
void sub_4b00(char *a0)
{
  error(0,0,"%s",dcgettext(NULL,a0,5));
  sub_4ad0(); // no-return
}


// Function: sub_4b30 @ 0x4b30
void sub_4b30(char *a0)
{
  int v1;
  
  v1 = *__errno_location();
  sub_4a90();
  error(0,v1,"%s",a0);
  sub_4ad0(); // no-return
}


// Function: sub_4b80 @ 0x4b80
void sub_4b80(void *a0,unsigned long a1,FILE *a2)
{
  if (a1 == fwrite_unlocked(a0,1,a1,a2))
    return;
  sub_4b30(dcgettext(NULL,"write failed",5)); // no-return
}


// Function: sub_4e40 @ 0x4e40
void sub_4e40(void)
{
  unsigned char v1;
  long v2;
  unsigned char *v3;
  int v4;
  int v5;
  long *v6; // rax
  
  v6 = __ctype_b_loc();
  while( true ) {
    v3 = *(unsigned char **)&stdin->field_0x8;
    if (*(unsigned char **)&stdin->field_0x10 <= v3) { // branch-flip
      v5 = __uflow(stdin);
      v1 = *(unsigned char *)(*v6 + 1 + (long)v5 * 2);
      v4 = dat_1015c;
    }
    else {
      v2 = *v6;
      *(unsigned char **)&stdin->field_0x8 = &v3[1];
      v5 = (unsigned int)*v3;
      v1 = *(unsigned char *)(v2 + 1 + (long)(int)(unsigned int)*v3 * 2);
      v4 = dat_1015c;
    }
    dat_1015c = v4;
    if ((!(v1 & 0x20)) || (v5 == 10)) break;
    if (v4) {
      if (1 <= dat_105a8)
        kill(dat_105a8,0xd);
      if (dat_105b8)
        unlink(dat_105b8);
      if (dat_10158)
        sub_4a10(v4);
      raise(v4);
      exit(2); // no-return
    }
  }
  if (!(*(unsigned char *)stdin & 0x20))
    return;
  sub_4b30(dcgettext(NULL,"read failed",5)); // no-return
}


// Function: sub_4f30 @ 0x4f30
void sub_4f30(unsigned long *a0,long a1)
{
  int v1;
  void *v2; // rax
  long v3; // rax
  char *v4; // rdx
  
  do {
    if (!a1)
      return;
    while( true ) {
      v2 = rawmemchr((void *)a0[1],10);
      a0[1] = v2;
      if (v2 != (void *)a0[3]) break;
      v3 = sub_4d80((void *)a0[2],(FILE *)*a0);
      v1 = dat_1015c;
      a0[1] = a0[2];
      v4 = (char *)(a0[2] + v3);
      a0[3] = v4;
      *v4 = 10;
      if (dat_1015c) {
        if (1 <= dat_105a8)
          kill(dat_105a8,0xd);
        if (dat_105b8)
          unlink(dat_105b8);
        if (dat_10158)
          sub_4a10(v1);
        raise(v1);
        exit(2); // no-return
      }
      if (!v3)
        return;
    }
    a0[1] = (long)v2 + 1;
    a1 -= 1;
  } while( true );
}


// Function: sub_5000 @ 0x5000
void sub_5000(unsigned long *a0,long a1,FILE *a2)
{
  int v1;
  void *v2;
  long v3; // rax
  char *v4; // rdx
  void *v5;
  unsigned long v6; // rsi
  void *v7;
  
  v2 = (void *)a0[1];
  if (a1) { // branch-flip
    v5 = (void *)a0[3];
    v7 = v2;
    do {
      while( true ) {
        v2 = rawmemchr(v2,10);
        a0[1] = v2;
        if (v2 != v5) break;
        sub_4b80(v7,(long)v5 - (long)v7,a2);
        v3 = sub_4d80((void *)a0[2],(FILE *)*a0);
        v1 = dat_1015c;
        a0[1] = a0[2];
        v4 = (char *)(a0[2] + v3);
        a0[3] = v4;
        *v4 = 10;
        if (dat_1015c) {
          if (1 <= dat_105a8)
            kill(dat_105a8,0xd);
          if (dat_105b8)
            unlink(dat_105b8);
          if (dat_10158)
            sub_4a10(v1);
          raise(v1);
          exit(2); // no-return
        }
        if (!v3)
          return;
        v2 = (void *)a0[1];
        v5 = (void *)a0[3];
        v7 = v2;
      }
      v2 = (void *)((long)v2 + 1);
      a0[1] = v2;
      a1 -= 1;
    } while (a1);
    v6 = (long)v2 - (long)v7;
    v2 = v7;
  }
  else {
    v6 = 0;
  }
  sub_4b80(v2,v6,a2); // tail-call
}


// Function: sub_5120 @ 0x5120
FILE * sub_5120(unsigned long *a0,char *a1,long a2,long a3,unsigned long *a4,char *a5,long a6,long a7,FILE *a8) // return-dupe x3
{
  char *v1;
  char v10 [65560];
  char *v11; // stack - 0x10068
  int v12;
  char *v13;
  char *v14; // stack - 0x10060
  unsigned long v15; // stack - 0x10058
  unsigned int v2;
  char *v3;
  FILE *v4;
  unsigned int v5; // eax
  FILE *v6; // rax
  int v7;
  unsigned long v8; // rbx
  int v9; // stack - 0x1006c
  
  v4 = a8;
label_51a0:
  v7 = (int)v8;
  v1 = *(char **)&stdout->field_0x28;
  if (*(char **)&stdout->field_0x30 <= v1) { // branch-flip
    v5 = __overflow(stdout,0x25);
    if (v5 != 0x25) {
      v6 = (char *)dcgettext(NULL,"write failed",5); // return-dupe
      sub_4b30(v6); // no-return
    }
  }
  else {
    *(char **)&stdout->field_0x28 = &v1[1];
    *v1 = 0x25;
  }
  v5 = fflush_unlocked(stdout);
  if (v5) {
    v6 = (char *)dcgettext(NULL,"write failed",5);
    sub_4b30(v6);
  }
  v5 = sub_4e40();
  v12 = v5;
  if (0x33 <= (int)v5) { // branch-flip
    if (v5 == 0x65) {
      v6 = (unsigned long)sub_4e40();
      v7 = v5;
      v8 = (unsigned long)v6 & 0xffffffff;
      if (0x33 <= (int)v5) { // branch-flip
        v5 -= 0x62;
        v2 = v5;
        v6 = (unsigned long)(unsigned long)v5;
        if ((v2 <= 0x10) && (0x10405UL >> ((unsigned long)v6 & 0x3f) & 1)) goto label_550e;
      }
      else {
        if (0x30 < (int)v5) {
label_550e:
          v5 = sub_4e40();
        }
        if (v5 == 10) {
label_52f2:
          if (dat_105b8) // branch-flip
            v6 = fopen(dat_105b8,"w");
          else {
            v6 = (char *)getenv("TMPDIR");
            v13 = (char *)v6;
            if (!v6)
              v13 = "/tmp";
            v6 = (unsigned long)strlen(v13);
            v6 = (char *)sub_7df0((long)v6 + 0xd);
            v3 = (char *)v6;
            __sprintf_chk((char *)v6,1,0xffffffffffffffff,"%s/sdiffXXXXXX",v13);
            v5 = mkstemp((char *)v6);
            if ((int)v5 < 0)
              sub_4b30("mkstemp"); // no-return
            dat_105b8 = v3;
            v6 = fdopen(v5,"w");
          }
          dat_105b0 = v6;
          if (!v6)
            sub_4b30(dat_105b8); // no-return
          if (v7 != 100) { // branch-flip
            if (0x65 <= v7) { // branch-flip
              if (v7 == 0x6c) goto label_5342;
label_56ba:
              sub_4f30(a0,a3);
            }
            else {
              if ((v7 != 0x31) && (v7 != 0x62)) goto label_56ba;
label_5342:
              sub_5000(a0,a3,v6);
            }
            if (0x65 <= v7) { // branch-flip
              if (v7 == 0x72) goto label_536b;
            }
            else if ((v7 == 0x32) || (v7 == 0x62)) goto label_536b;
            sub_4f30(a4,a7);
          }
          else {
            if (a3) {
              if (a3 != 1) { // branch-flip
                __fprintf_chk(v6,1,"--- %s %ld,%ld\n",a1,a2,a2 + -1 + a3);
                v6 = dat_105b0;
              }
              else {
                __fprintf_chk(v6,1,"--- %s %ld\n",a1,a2);
                v6 = dat_105b0;
              }
            }
            sub_5000(a0,a3,v6);
            if (a7) {
              if (a7 != 1) // branch-flip
                __fprintf_chk(dat_105b0,1,"+++ %s %ld,%ld\n",a5,a6,a6 + -1 + a7);
              else {
                __fprintf_chk(dat_105b0,1,"+++ %s %ld\n",a5,a6);
              }
            }
label_536b:
            sub_5000(a4,a7,dat_105b0);
          }
          v5 = fclose(dat_105b0);
          if (v5) {
            sub_4b30("fclose"); // return-dupe, no-return
          }
          dat_10160 = 1;
          if (dat_1015c) goto label_5873;
          v15 = 0;
          v11 = dat_10010;
          v14 = dat_105b8;
          v5 = fork();
          v7 = v5;
          if (!v5) {
            execvp(dat_10010,&v11);
            v6 = (int *)__errno_location();
            _exit((*(int *)v6 == 2) + 0x7e); // no-return
          }
          if ((int)v5 < 0)
            sub_4b30("fork"); // no-return
          goto label_540c;
        }
      }
label_525d:
      v6 = (char *)dcgettext(NULL,"ed:\tEdit then use both versions, each decorated with a header.\neb:\tEdit then use both versions.\nel or e1:\tEdit then use the left version.\ner or e2:\tEdit then use the right version.\ne:\tDiscard both versions then edit a new one.\nl or 1:\tUse the left version.\nr or 2:\tUse the right version.\ns:\tSilently include common lines.\nv:\tVerbosely include common lines.\nq:\tQuit.\n",5);
      __fprintf_chk(stderr,1,"%s",v6);
      sub_4d10();
      goto label_51a0;
    }
    if ((int)v5 <= 100) goto label_5236;
    if (0x74 <= (int)v5) { // branch-flip
      if (v5 == 0x76) goto label_5295;
    }
    else {
      if (0x71 <= (int)v5) {
label_5295:
        v5 = sub_4e40();
        if (v5 != 10) goto label_525d;
        if (v12 <= 0x76) {
          if (0x65 <= v12) { // branch-flip
            switch(v12) {
              case 0x65:
                goto label_52f2;
              case 0x6c:
                goto label_54ca;
              case 0x71:
                v6 = NULL;
                return v6;
              case 0x72:
                sub_5000(a4,a7,v4); // return-dupe
                sub_4f30(a0,a3);
                v6 = (FILE *)0x1;
                return v6;
              case 0x73:
                dat_10161 = 1;
                goto label_51a0;
              case 0x76:
                dat_10161 = 0;
                goto label_51a0;
              
            }
          }
          else {
            if (v12 == 0x31) {
label_54ca:
              sub_5000(a0,a3,v4); // return-dupe
              sub_4f30(a4,a7);
              v6 = (FILE *)0x1;
              return v6;
            }
            if (v12 == 0x32) {
              sub_5000(a4,a7,v4);
              sub_4f30(a0,a3);
              v6 = (FILE *)0x1;
              return v6;
            }
          }
        }
        v6 = (char *)dcgettext(NULL,"ed:\tEdit then use both versions, each decorated with a header.\neb:\tEdit then use both versions.\nel or e1:\tEdit then use the left version.\ner or e2:\tEdit then use the right version.\ne:\tDiscard both versions then edit a new one.\nl or 1:\tUse the left version.\nr or 2:\tUse the right version.\ns:\tSilently include common lines.\nv:\tVerbosely include common lines.\nq:\tQuit.\n",5);
        __fprintf_chk(stderr,1,"%s",v6);
        goto label_51a0;
      }
      if (v5 == 0x6c) {
        v5 = sub_4e40();
        if (v5 != 10) goto label_525d;
        sub_5000(a0,a3,v4);
        sub_4f30(a4,a7);
        v6 = (FILE *)0x1;
        return v6;
      }
    }
  }
  else {
    if (0x30 < (int)v5) goto label_5295;
    if (v5 != -1) {
      if (v5 != 10) goto label_5236;
      goto label_51f7;
    }
    if (*(unsigned char *)stdin & 0x10) {
      v6 = NULL;
      return v6;
    }
  }
label_5236:
  sub_4d10();
label_51f7:
  v6 = (char *)dcgettext(NULL,"ed:\tEdit then use both versions, each decorated with a header.\neb:\tEdit then use both versions.\nel or e1:\tEdit then use the left version.\ner or e2:\tEdit then use the right version.\ne:\tDiscard both versions then edit a new one.\nl or 1:\tUse the left version.\nr or 2:\tUse the right version.\ns:\tSilently include common lines.\nv:\tVerbosely include common lines.\nq:\tQuit.\n",5);
  __fprintf_chk(stderr,1,"%s",v6);
  goto label_51a0;
label_5488:
  v6 = (unsigned long)sub_4d80(v10,v6);
  if (!v6) {
    v5 = fclose(dat_105b0);
    if (!v5) {
      v6 = (FILE *)0x1;
      return v6;
    }
    sub_4b30("fclose");
  }
  if (dat_1015c) goto label_5873;
  sub_4b80(v10,v6,v4);
  v6 = dat_105b0;
  goto label_5488;
label_5873:
  v12 = dat_1015c;
  sub_4990(0);
  goto label_5813;
  while( true ) {
    v6 = (int *)__errno_location();
    v12 = dat_1015c;
    if (*(int *)v6 != 4)
      sub_4b30("waitpid"); // no-return
    if (dat_1015c) break;
label_540c:
    v5 = waitpid(v7,&v9,0);
    if (0 <= (int)v5) {
      dat_10160 = 0;
      sub_4dc0(v9,0,dat_10010);
      v13 = dat_105b8;
      v6 = fopen(dat_105b8,"r");
      if (!v6)
        sub_4b30(v13,0); // no-return
      dat_105b0 = v6;
      goto label_5488;
    }
  }
  if (1 <= dat_105a8)
    kill(dat_105a8,0xd);
  if (dat_105b8)
    unlink(dat_105b8);
label_5813:
  if (dat_10158)
    sub_4a10(v12);
  raise(v12);
  exit(2); // no-return
}

