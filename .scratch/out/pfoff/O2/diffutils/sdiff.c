// Function: _DT_INIT @ 0x3000
void _DT_INIT(void) // return-dupe
{
  if (!dat_ffe8)
    return;
  (*dat_ffe8)();
}

// Function: sub_3020 @ 0x3020
void sub_3020(void)
{
  (*dat_fd20)(); // jump-as-call
}

// Function: __cxa_finalize @ 0x3590
void __cxa_finalize(void)
{
  (*dat_fff8)(); // jump-as-call
}

// Function: __uflow @ 0x35a0
int __uflow(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_fd28)(); // jump-as-call
  return v1;
}

// Function: getenv @ 0x35b0
char * getenv(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_fd30)(); // jump-as-call
  return v1;
}

// Function: raise @ 0x35c0
int raise(int a0)
{
  int v1; // eax
  
  v1 = (*dat_fd38)(); // jump-as-call
  return v1;
}

// Function: free @ 0x35d0
void free(void *a0)
{
  (*dat_fd40)(); // jump-as-call
}

// Function: strtoimax @ 0x35e0
void strtoimax(void)
{
  (*dat_fd48)(); // jump-as-call
}

// Function: __mempcpy_chk @ 0x35f0
void * __mempcpy_chk(void *a0,void *a1,unsigned long a2,unsigned long a3)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_fd50)(); // jump-as-call
  return v1;
}

// Function: abort @ 0x3600
void abort(void)
{
  (*dat_fd58)(); // jump-as-call
}

// Function: __errno_location @ 0x3610
int * __errno_location(void)
{
  int *v1; // rax
  
  v1 = (int *)(*dat_fd60)(); // jump-as-call
  return v1;
}

// Function: unlink @ 0x3620
int unlink(char *a0)
{
  int v1; // eax
  
  v1 = (*dat_fd68)(); // jump-as-call
  return v1;
}

// Function: strncmp @ 0x3630
int strncmp(char *a0,char *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_fd70)(); // jump-as-call
  return v1;
}

// Function: _exit @ 0x3640
void _exit(int a0)
{
  (*dat_fd78)(); // jump-as-call
}

// Function: iconv @ 0x3650
void iconv(void)
{
  (*dat_fd80)(); // jump-as-call
}

// Function: puts @ 0x3660
int puts(char *a0)
{
  int v1; // eax
  
  v1 = (*dat_fd88)(); // jump-as-call
  return v1;
}

// Function: sigaction @ 0x3670
int sigaction(int a0,sigaction *a1,sigaction *a2)
{
  int v1; // eax
  
  v1 = (*dat_fd90)(); // jump-as-call
  return v1;
}

// Function: iswcntrl @ 0x3680
int iswcntrl(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_fd98)(); // jump-as-call
  return v1;
}

// Function: reallocarray @ 0x3690
void * reallocarray(void *a0,unsigned long a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_fda0)(); // jump-as-call
  return v1;
}

// Function: fcntl @ 0x36a0
int fcntl(int a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_fda8)(); // jump-as-call
  return v1;
}

// Function: write @ 0x36b0
long write(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_fdb0)(); // jump-as-call
  return v1;
}

// Function: fread_unlocked @ 0x36c0
unsigned long fread_unlocked(void *a0,unsigned long a1,unsigned long a2,FILE *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_fdb8)(); // jump-as-call
  return v1;
}

// Function: textdomain @ 0x36d0
char * textdomain(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_fdc0)(); // jump-as-call
  return v1;
}

// Function: __open_2 @ 0x36e0
int __open_2(char *a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_fdc8)(); // jump-as-call
  return v1;
}

// Function: fclose @ 0x36f0
int fclose(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_fdd0)(); // jump-as-call
  return v1;
}

// Function: bindtextdomain @ 0x3700
char * bindtextdomain(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_fdd8)(); // jump-as-call
  return v1;
}

// Function: dcgettext @ 0x3710
char * dcgettext(char *a0,char *a1,int a2)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_fde0)(); // jump-as-call
  return v1;
}

// Function: __ctype_get_mb_cur_max @ 0x3720
unsigned long __ctype_get_mb_cur_max(void)
{
  unsigned long v1; // rax
  
  v1 = (*dat_fde8)(); // jump-as-call
  return v1;
}

// Function: strlen @ 0x3730
unsigned long strlen(char *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_fdf0)(); // jump-as-call
  return v1;
}

// Function: __stack_chk_fail @ 0x3740
void __stack_chk_fail(void)
{
  (*dat_fdf8)(); // jump-as-call
}

// Function: mmap @ 0x3750
void mmap(void)
{
  (*dat_fe00)(); // jump-as-call
}

// Function: getopt_long @ 0x3760
int getopt_long(int a0,char **a1,char *a2,option *a3,int *a4)
{
  int v1; // eax
  
  v1 = (*dat_fe08)(); // jump-as-call
  return v1;
}

// Function: mbrtowc @ 0x3770
unsigned long mbrtowc(void *a0,char *a1,unsigned long a2,mbstate_t *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_fe10)(); // jump-as-call
  return v1;
}

// Function: dup2 @ 0x3780
int dup2(int a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_fe18)(); // jump-as-call
  return v1;
}

// Function: strchr @ 0x3790
char * strchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_fe20)(); // jump-as-call
  return v1;
}

// Function: __overflow @ 0x37a0
int __overflow(FILE *a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_fe28)(); // jump-as-call
  return v1;
}

// Function: strrchr @ 0x37b0
char * strrchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_fe30)(); // jump-as-call
  return v1;
}

// Function: lseek @ 0x37c0
void lseek(void)
{
  (*dat_fe38)(); // jump-as-call
}

// Function: __assert_fail @ 0x37d0
void __assert_fail(char *a0,char *a1,unsigned int a2,char *a3)
{
  (*dat_fe40)(); // jump-as-call
}

// Function: fputs @ 0x37e0
int fputs(char *a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_fe48)(); // jump-as-call
  return v1;
}

// Function: strnlen @ 0x37f0
unsigned long strnlen(char *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  v1 = (*dat_fe50)(); // jump-as-call
  return v1;
}

// Function: close @ 0x3800
int close(int a0)
{
  int v1; // eax
  
  v1 = (*dat_fe58)(); // jump-as-call
  return v1;
}

// Function: pipe @ 0x3810
int pipe(int *a0)
{
  int v1; // eax
  
  v1 = (*dat_fe60)(); // jump-as-call
  return v1;
}

// Function: memchr @ 0x3820
void * memchr(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_fe68)(); // jump-as-call
  return v1;
}

// Function: read @ 0x3830
long read(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_fe70)(); // jump-as-call
  return v1;
}

// Function: memcmp @ 0x3840
int memcmp(void *a0,void *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_fe78)(); // jump-as-call
  return v1;
}

// Function: fputs_unlocked @ 0x3850
int fputs_unlocked(char *a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_fe80)(); // jump-as-call
  return v1;
}

// Function: rawmemchr @ 0x3860
void * rawmemchr(void *a0,int a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_fe88)(); // jump-as-call
  return v1;
}

// Function: calloc @ 0x3870
void * calloc(unsigned long a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_fe90)(); // jump-as-call
  return v1;
}

// Function: strcmp @ 0x3880
int strcmp(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_fe98)(); // jump-as-call
  return v1;
}

// Function: signal @ 0x3890
void signal(void)
{
  (*dat_fea0)(); // jump-as-call
}

// Function: sigemptyset @ 0x38a0
int sigemptyset(sigset_t *a0)
{
  int v1; // eax
  
  v1 = (*dat_fea8)(); // jump-as-call
  return v1;
}

// Function: stat @ 0x38b0
int stat(char *a0,stat *a1)
{
  int v1; // eax
  
  v1 = (*dat_feb0)(); // jump-as-call
  return v1;
}

// Function: memcpy @ 0x38c0
void * memcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_feb8)(); // jump-as-call
  return v1;
}

// Function: kill @ 0x38d0
int kill(int a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_fec0)(); // jump-as-call
  return v1;
}

// Function: mincore @ 0x38e0
int mincore(void *a0,unsigned long a1,char *a2)
{
  int v1; // eax
  
  v1 = (*dat_fec8)(); // jump-as-call
  return v1;
}

// Function: wcwidth @ 0x38f0
int wcwidth(int a0)
{
  int v1; // eax
  
  v1 = (*dat_fed0)(); // jump-as-call
  return v1;
}

// Function: iswalnum @ 0x3900
int iswalnum(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_fed8)(); // jump-as-call
  return v1;
}

// Function: malloc @ 0x3910
void * malloc(unsigned long a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_fee0)(); // jump-as-call
  return v1;
}

// Function: nl_langinfo @ 0x3920
char * nl_langinfo(int a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_fee8)(); // jump-as-call
  return v1;
}

// Function: mkstemp @ 0x3930
int mkstemp(char *a0)
{
  int v1; // eax
  
  v1 = (*dat_fef0)(); // jump-as-call
  return v1;
}

// Function: fwrite_unlocked @ 0x3940
unsigned long fwrite_unlocked(void *a0,unsigned long a1,unsigned long a2,FILE *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_fef8)(); // jump-as-call
  return v1;
}

// Function: realloc @ 0x3950
void * realloc(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_ff00)(); // jump-as-call
  return v1;
}

// Function: fdopen @ 0x3960
FILE * fdopen(int a0,char *a1)
{
  FILE *v1; // rax
  
  v1 = (FILE *)(*dat_ff08)(); // jump-as-call
  return v1;
}

// Function: munmap @ 0x3970
int munmap(void *a0,unsigned long a1)
{
  int v1; // eax
  
  v1 = (*dat_ff10)(); // jump-as-call
  return v1;
}

// Function: setlocale @ 0x3980
char * setlocale(int a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_ff18)(); // jump-as-call
  return v1;
}

// Function: __printf_chk @ 0x3990
int __printf_chk(int a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_ff20)(); // jump-as-call
  return v1;
}

// Function: iconv_close @ 0x39a0
void iconv_close(void)
{
  (*dat_ff28)(); // jump-as-call
}

// Function: iswspace @ 0x39b0
int iswspace(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_ff30)(); // jump-as-call
  return v1;
}

// Function: mempcpy @ 0x39c0
void * mempcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_ff38)(); // jump-as-call
  return v1;
}

// Function: memmove @ 0x39d0
void * memmove(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_ff40)(); // jump-as-call
  return v1;
}

// Function: error @ 0x39e0
void error(int a0,int a1,char *a2,...)
{
  (*dat_ff48)(); // jump-as-call
}

// Function: waitpid @ 0x39f0
int waitpid(int a0,int *a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_ff50)(); // jump-as-call
  return v1;
}

// Function: open @ 0x3a00
int open(char *a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_ff58)(); // jump-as-call
  return v1;
}

// Function: fopen @ 0x3a10
FILE * fopen(char *a0,char *a1)
{
  FILE *v1; // rax
  
  v1 = (FILE *)(*dat_ff60)(); // jump-as-call
  return v1;
}

// Function: perror @ 0x3a20
void perror(char *a0)
{
  (*dat_ff68)(); // jump-as-call
}

// Function: execvp @ 0x3a30
int execvp(char *a0,char **a1)
{
  int v1; // eax
  
  v1 = (*dat_ff70)(); // jump-as-call
  return v1;
}

// Function: getpagesize @ 0x3a40
int getpagesize(void)
{
  int v1; // eax
  
  v1 = (*dat_ff78)(); // jump-as-call
  return v1;
}

// Function: exit @ 0x3a50
void exit(int a0)
{
  (*dat_ff80)(); // jump-as-call
}

// Function: __fprintf_chk @ 0x3a60
int __fprintf_chk(FILE *a0,int a1,char *a2,...)
{
  int v1; // eax
  
  v1 = (*dat_ff88)(); // jump-as-call
  return v1;
}

// Function: fflush_unlocked @ 0x3a70
int fflush_unlocked(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_ff90)(); // jump-as-call
  return v1;
}

// Function: strdup @ 0x3a80
char * strdup(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_ff98)(); // jump-as-call
  return v1;
}

// Function: mbsinit @ 0x3a90
int mbsinit(mbstate_t *a0)
{
  int v1; // eax
  
  v1 = (*dat_ffa0)(); // jump-as-call
  return v1;
}

// Function: sigaltstack @ 0x3aa0
int sigaltstack(void *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_ffa8)(); // jump-as-call
  return v1;
}

// Function: sigaddset @ 0x3ab0
int sigaddset(sigset_t *a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_ffb0)(); // jump-as-call
  return v1;
}

// Function: fork @ 0x3ac0
int fork(void)
{
  int v1; // eax
  
  v1 = (*dat_ffb8)(); // jump-as-call
  return v1;
}

// Function: __ctype_b_loc @ 0x3ad0
void * __ctype_b_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_ffc0)(); // jump-as-call
  return v1;
}

// Function: iconv_open @ 0x3ae0
void iconv_open(void)
{
  (*dat_ffc8)(); // jump-as-call
}

// Function: __sprintf_chk @ 0x3af0
int __sprintf_chk(char *a0,int a1,unsigned long a2,char *a3,...)
{
  int v1; // eax
  
  v1 = (*dat_ffd0)(); // jump-as-call
  return v1;
}

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

// Function: sub_4730 @ 0x4730
void sub_4730(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_ffd8)(main,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: sub_4760 @ 0x4760
void sub_4760(void)
{
  return;
}

// Function: _FINI_0 @ 0x47d0
void _FINI_0(void)
{
  if (!dat_10088) {
    if (dat_fff8)
      __cxa_finalize(dat_10008);
    sub_4760();
    dat_10088 = 1;
    return;
  }
}

// Function: _INIT_0 @ 0x4810
void _INIT_0(void)
{
  return;
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

// Function: sub_4990 @ 0x4990
void sub_4990(void)
{
  if (0 < dat_105a8) {
    kill(dat_105a8,0xd);
    if (dat_105b8) {
      unlink(dat_105b8); // tail-call
      return;
    }
    return;
  }
  if (dat_105b8) {
    unlink(dat_105b8); // tail-call
    return;
  }
}

// Function: sub_4a10 @ 0x4a10
void sub_4a10(int a0)
{
  int v1;
  sigaction *v2;
  sigaction *v3; // rbx
  int *v4; // rbp
  
  v4 = (int *)0xbd30;
  v2 = (sigaction *)0x10180;
  do {
    while (((!a0 || (a0 == *v4)) && (*(long *)v2 != 1))) {
      v1 = *v4;
      v3 = &v2[1];
      v4 = &v4[1];
      sigaction(v1,v2,NULL);
      v2 = v3;
      if (v3 == (sigaction *)0x105a8)
        return;
    }
    v2 = &v2[1];
    v4 = &v4[1];
  } while (v2 != (sigaction *)0x105a8);
}

// Function: sub_4a90 @ 0x4a90
void sub_4a90(void)
{
  int v1;
  
  v1 = dat_1015c;
  if (!dat_1015c)
    return;
  sub_4990(0);
  if (dat_10158)
    sub_4a10(v1);
  raise(v1);
  exit(2); // no-return
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

// Function: sub_4b60 @ 0x4b60
void sub_4b60(char *a0,char *a1)
{
  if (fopen(a0,a1))
    return;
  sub_4b30(a0); // no-return
}

// Function: sub_4b80 @ 0x4b80
void sub_4b80(void *a0,unsigned long a1,FILE *a2)
{
  if (a1 == fwrite_unlocked(a0,1,a1,a2))
    return;
  sub_4b30(dcgettext(NULL,"write failed",5)); // no-return
}

// Function: sub_4bc0 @ 0x4bc0
void sub_4bc0(FILE *a0)
{
  if (!fclose(a0))
    return;
  sub_4b30("fclose"); // no-return
}

// Function: sub_4be0 @ 0x4be0
char * sub_4be0(char *a0,bool a1,char *a2)
{
  long v1;
  char *v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // rax
  char *v5;
  unsigned long v6; // r15
  
  if (strcmp(a0,"-")) {
    v5 = a0;
    if (a1) {
      v6 = 0;
      v2 = (char *)sub_58b0(a2);
      v3 = strlen(a0);
      v4 = sub_5910(v2);
      if (*(char *)sub_58b0(a0))
        v6 = (unsigned long)(a0[v3 - 1] != '/');
      v1 = v4 + v6 + v3;
      v5 = (char *)sub_7df0(v1 + 1);
      memcpy(v5,a0,v3);
      v5[v3] = '/';
      memcpy(&v5[v6 + v3],v2,v4);
      v5[v1] = '\0';
    }
    return v5;
  }
  sub_4b00("cannot interactively merge standard input"); // no-return
}

// Function: sub_4cc0 @ 0x4cc0
void sub_4cc0(void)
{
  if (*(unsigned char *)stdout & 0x20)
    sub_4b00("write failed"); // no-return
  if (!fclose(stdout))
    return;
  sub_4b30(dcgettext(NULL,"standard output",5)); // no-return
}

// Function: sub_4d10 @ 0x4d10
void sub_4d10(void)
{
  char *v1;
  int v2; // eax
  
  do {
    while (v1 = *(char **)&stdin->field_0x8, *(char **)&stdin->field_0x10 <= v1) {
      v2 = __uflow(stdin);
      if ((v2 == -1) || (v2 == 10)) goto label_4d35;
    }
    *(char **)&stdin->field_0x8 = &v1[1];
  } while (*v1 != '\n');
label_4d35:
  if (*(unsigned char *)stdin & 0x20)
    sub_4b30(dcgettext(NULL,"read failed",5)); // no-return
}

// Function: sub_4d80 @ 0x4d80
void sub_4d80(void *a0,FILE *a1)
{
  if ((!fread_unlocked(a0,1,0x10000,a1)) && (*(unsigned char *)a1 & 0x20))
    sub_4b30(dcgettext(NULL,"read failed",5)); // no-return
}

// Function: sub_4dc0 @ 0x4dc0
void sub_4dc0(unsigned long a0,int a1,char *a2)
{
  unsigned char v1;
  unsigned int v2; // ebx
  unsigned int v3;
  char *v4; // rsi
  
  if (a0 & 0x7f) { // branch-flip
    v4 = "subsidiary program \'%s\' failed";
    v3 = 0x7fffffff;
  }
  else {
    v1 = (unsigned char)(a0 >> 8);
    v2 = (unsigned int)v1;
    if ((int)(unsigned int)v1 <= a1)
      return;
    v4 = "subsidiary program \'%s\' could not be invoked";
    v3 = (unsigned int)v1;
    if ((v1 != 0x7e) && (v4 = "subsidiary program \'%s\' failed (exit status %d)", v3 = v2, v2 == 0x7f))
      v4 = "subsidiary program \'%s\' not found";
  }
  error(0,0,dcgettext(NULL,v4,5),a2,(unsigned long)v3);
  sub_4ad0(); // no-return
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

// Function: sub_58b0 @ 0x58b0
char * sub_58b0(char *a0)
{
  bool v1;
  char *v2;
  char v3;
  
  v3 = *a0;
  while (v3 == '/') {
    v2 = &a0[1];
    a0 = &a0[1];
    v3 = *v2;
  }
  if (v3) {
    v1 = 0;
    v2 = a0;
    do {
      while (v3 != '/') {
        if (v1) {
          v1 = 0;
          a0 = v2;
        }
        v3 = v2[1];
        v2 = &v2[1];
        if (!v3)
          return a0;
      }
      v3 = v2[1];
      v2 = &v2[1];
      v1 = 1;
    } while (v3);
  }
  return a0;
}

// Function: sub_5910 @ 0x5910
void sub_5910(char *a0)
{
  long v1;
  unsigned long v2;
  
  v2 = strlen(a0);
  do {
    if (2 > v2) break;
    v1 = v2 - 1;
    v2 -= 1;
  } while (a0[v1] == '/');
}

// Function: sub_5940 @ 0x5940
void sub_5940(void)
{
  return;
}

// Function: sub_5950 @ 0x5950
void sub_5950(int a0)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  char *v3; // rax
  char *v4; // rax
  char v5 [4104]; // stack - 0x1038
  char *v6;
  long v7; // fs_offset
  unsigned long v8; // stack - 0x30
  
  v8 = *(unsigned long *)(v7 + 0x28);
  (*dat_10600)();
  if (a0) // branch-flip
    v6 = dat_105f8;
  else {
    v6 = dat_105f0;
  }
  v1 = strlen(dat_105e8);
  v2 = strlen(v6);
  if (0x1000 <= v1 + v2) { // branch-flip
    write(2,dat_105e8,v1);
    write(2,": ",2);
    write(2,v6,v2);
    v5[0] = 10;
    v4 = (char *)0x1;
  }
  else {
    v3 = __mempcpy_chk(v5,dat_105e8,v1,0x1002);
    *(unsigned short *)v3 = s_bd6d._0_2_;
    v4 = mempcpy(&v3[2],v6,v2);
    *v4 = 10;
    v4 = &v4[1 - (long)v5];
  }
  write(2,v5,(unsigned long)v4);
  if (!a0)
    _exit(dat_10020); // no-return
  raise(a0);
  abort(); // no-return
}

// Function: sub_5a70 @ 0x5a70
unsigned long sub_5a70(unsigned long a0,int a1)
{
  if (!a1)
    return 0;
  sub_5950(0xb); // no-return
}

// Function: sub_5a90 @ 0x5a90
void sub_5a90(unsigned long a0)
{
  if ((int)a0)
    a0 = (unsigned long)(-(unsigned int)(dat_105e0 == 0) & 0xb);
  sub_5950(a0); // no-return
}

// Function: sub_5ac0 @ 0x5ac0
unsigned long sub_5ac0(void *a0) // return-dupe
{
  if (!a0)
    a0 = sub_5940;
  dat_10600 = a0;
  dat_105f8 = dcgettext(NULL,"program error",5);
  dat_105f0 = dcgettext(NULL,"stack overflow",5);
  dat_105e8 = sub_5b70();
  if (!sub_6740(sub_5a90,0x10620,0x10000)) {
    dat_105e0 = sub_66c0(sub_5a70);
    return 0;
  }
  *__errno_location() = 0x5f;
  return 0xffffffff;
}

// Function: sub_5b70 @ 0x5b70
unsigned long sub_5b70(void)
{
  return __progname;
}

// Function: sub_5b80 @ 0x5b80
void sub_5b80(char *a0)
{
  char *v1;
  char *v2; // rax
  
  if (a0) {
    v2 = strrchr(a0,0x2f);
    if ((v2) && (v1 = &v2[1], 7 <= (long)v1 - (long)a0)) {
      if (!strncmp(&v2[-6],"/.libs/",7)) {
        a0 = v1;
        if (!strncmp(v1,"lt-",3)) {
          a0 = &v2[4];
          __progname = a0;
        }
      }
    }
    dat_20620 = a0;
    program_invocation_name = a0;
    return;
  }
  fputs("A NULL argv[0] was passed through an exec system call.\n",stderr);
  abort(); // no-return
}

// Function: sub_5c20 @ 0x5c20
void sub_5c20(struct_2 *a0)
{
  unsigned char v1;
  unsigned char *v2;
  unsigned long v3; // rax
  long v4; // rax
  
  if (!a0->field_0x0) {
    v1 = *a0->field_0x10;
    if (*(unsigned int *)((unsigned long)(v1 >> 5) * 4 + 0xc260) >> (v1 & 0x1f) & 1) {
      a0->field_0x18 = 1;
      v1 = *a0->field_0x10;
      a0->field_0x20 = '\x01';
      a0->field_0x24 = (int)(char)v1;
      goto label_5c61;
    }
    if (!mbsinit((mbstate_t *)&a0->field_0x1[3]))
      __assert_fail("mbsinit (&iter->state)","mbuiter.h",0x8f,"mbuiter_multi_next"); // no-return
    a0->field_0x0 = '\x01';
  }
  v3 = __ctype_get_mb_cur_max();
  v2 = a0->field_0x10;
  v4 = sub_8380(&a0->field_0x24,v2,sub_70a0(v2,v3),(mbstate_t *)&a0->field_0x1[3]);
  a0->field_0x18 = v4;
  if (v4 == -1) {
    a0->field_0x18 = 1;
    a0->field_0x20 = '\0';
    a0->field_0xc = '\x01';
    return;
  }
  if (v4 == -2) {
    v3 = strlen((char *)a0->field_0x10);
    a0->field_0x20 = '\0';
    a0->field_0x18 = v3;
    a0->field_0xc = '\x01';
    return;
  }
  if (!v4) {
    a0->field_0x18 = 1;
    if (*a0->field_0x10)
      __assert_fail("*iter->cur.ptr == \'\\0\'","mbuiter.h",0xab,"mbuiter_multi_next"); // no-return
    if (a0->field_0x24)
      __assert_fail("iter->cur.wc == 0","mbuiter.h",0xac,"mbuiter_multi_next"); // no-return
  }
  a0->field_0x20 = '\x01';
  if (mbsinit((mbstate_t *)&a0->field_0x1[3])) {
    a0->field_0x0 = '\0';
    a0->field_0xc = '\x01';
    return;
  }
label_5c61:
  a0->field_0xc = '\x01';
  return;
}

// Function: sub_5d90 @ 0x5d90
unsigned long sub_5d90(char *a0,char *a1)
{
  char v1;
  char v10; // stack - 0xac
  char *v11; // stack - 0xa8
  long v12; // stack - 0xa0
  char v13; // stack - 0x98
  unsigned int v14; // stack - 0x94
  unsigned long v15; // stack - 0x74
  char v16; // stack - 0x6c
  char *v17; // stack - 0x68
  long v18; // stack - 0x60
  char v19; // stack - 0x58
  unsigned char v2;
  int v20; // stack - 0x54
  char *v3; // rax
  long *v4; // rax
  char *v5; // rax
  char v6 [4]; // stack - 0xb8
  char v7 [4]; // stack - 0x78
  unsigned char *v8;
  unsigned long v9; // stack - 0xb4
  
  v3 = (char *)sub_7110(a1,2);
  v1 = *a0;
  while ((v1 && (v5 = (char *)sub_8db0(a0,v3), v5))) {
    if (2 <= __ctype_get_mb_cur_max()) { // branch-flip
      v6[0] = 0;
      v9 = 0;
      v11 = a0;
      if (v5 <= a0) { // branch-flip
label_5ec1:
        v8 = (unsigned char *)0x1;
      }
      else {
        do {
          v10 = '\0';
          sub_5c20(v6);
          if ((v13) && (!v14)) {
            abort(); // no-return, return-dupe
          }
          v11 = &v11[v12];
          v10 = 0;
        } while (v11 < v5);
        if (!v13) goto label_5ec1;
        v8 = (unsigned char *)CONCAT71((undefined7)((unsigned long)v8 >> 8),iswalnum(v14) == 0);
      }
      v11 = v5;
      v6[0] = 0;
      v9 = 0;
      v17 = v3;
      v7[0] = 0;
      v15 = 0;
      while( true ) {
        v10 = '\0';
        v16 = 0;
        sub_5c20(v7);
        if ((v19) && (!v20)) break;
        if (!v10)
          sub_5c20(v6);
        if ((v13) && (!v14))
          abort();
        v11 = &v11[v12];
        v17 = &v17[v18];
      }
      if (!v10)
        sub_5c20(v6);
      if (((((v13) && (!v14)) || (!v13)) || (!iswalnum(v14))) && ((char)v8)) {
        free(v3); // return-dupe
        return (unsigned long)v8 & 0xffffffff;
      }
      v11 = v5;
      v6[0] = 0;
      v9 = 0;
      v10 = '\0';
      sub_5c20(v6);
      if ((v13) && (!v14)) break;
      a0 = &v5[v12];
      v1 = *a0;
    }
    else {
      v8 = (unsigned char *)&v5[strlen(v3)];
      if (((v5 <= a0) || (v4 = __ctype_b_loc(), !(*(unsigned char *)(*v4 + (unsigned long)(unsigned char)v5[-1] * 2) & 8))) && ((v2 = *v8, !v2 || (!(*(unsigned char *)(*(long *)__ctype_b_loc() + (unsigned long)v2 * 2) & 8))))) {
        v8 = (unsigned char *)0x1;
        free(v3);
        return (unsigned long)v8 & 0xffffffff;
      }
      if (!*v5) break;
      v1 = v5[1];
      a0 = &v5[1];
    }
  }
  v8 = NULL;
  free(v3);
  return (unsigned long)v8 & 0xffffffff;
}

// Function: sub_6020 @ 0x6020
char * sub_6020(char *a0)
{
  char *v1; // rax
  unsigned long v2; // rax
  char *v3; // rax
  
  v1 = dcgettext(NULL,a0,5);
  if (a0 != v1) {
    if (!sub_5d90(v1,a0)) {
      v2 = strlen(v1);
      v3 = (char *)sub_7df0(v2 + 4 + strlen(a0));
      __sprintf_chk(v3,1,0xffffffffffffffff,"%s (%s)",v1,a0);
      return v3;
    }
  }
  return v1;
}

// Function: sub_60b0 @ 0x60b0
char * sub_60b0(char *a0,char *a1)
{
  char v1; // al
  char *v10;
  char *v11; // stack - 0x48
  int v2; // eax
  char *v3; // rax
  char *v4;
  unsigned long v5; // rax
  char *v6;
  char *v7; // rax
  void *v8; // rax
  unsigned long *v9; // rbx
  
  v3 = dcgettext(NULL,a0,5);
  v4 = (char *)sub_8510();
  v6 = v3;
  if (sub_8410(v4,"UTF-8")) { // branch-flip
    v7 = (char *)sub_8350(a1,"UTF-8",v4);
    v5 = strlen(v4);
    v8 = memcpy((void *)sub_7df0(v5 + 0xb),v4,v5);
    v9 = (unsigned long *)(v5 + (long)v8);
    *v9 = 0x4c534e4152542f2f;
    *(unsigned short *)&v9[1] = 0x5449;
    *(char *)((long)v9 + 10) = 0;
    a1 = (char *)sub_8350(a1,"UTF-8",v8);
    free(v8);
    v11 = v7;
    if (a1) { // branch-flip
      if (strchr(a1,0x3f)) {
        free(a1);
        goto label_62b3;
      }
      v4 = a1;
      if (!v7) {
        v10 = a1;
        goto label_611a;
      }
      if (!strcmp(v3,a0)) {
        if (v7 == a1)
          return v7;
        free(a1);
        return v7;
      }
    }
    else {
label_62b3:
      if (!v7) {
        v11 = NULL;
        a1 = NULL;
        v10 = a0;
        v4 = NULL;
        goto label_611a;
      }
      if (!strcmp(v3,a0))
        return v7;
      a1 = NULL;
      v4 = NULL;
    }
    if (!sub_5d90(v3,a0)) {
      a0 = v7;
label_613f:
      if (((!v11) || (!sub_5d90(v3,v11))) && ((!a1 || (!sub_5d90(v3,a1))))) goto label_61bc;
      if (!v7) goto label_616d;
    }
    free(v7);
  }
  else {
    v2 = strcmp(v3,a0);
    if (a1) {
      v10 = a1;
      v4 = NULL;
      v11 = a1;
label_611a:
      if (!strcmp(v3,a0))
        return v10;
      v1 = sub_5d90(v3,a0);
      v7 = NULL;
      a0 = v10;
      if (v1) goto label_616d;
      goto label_613f;
    }
    if (!v2)
      return a0;
    if (sub_5d90(v3,a0))
      return v3;
    v4 = NULL;
    v7 = NULL;
label_61bc:
    v5 = strlen(v3);
    v6 = (char *)sub_7df0(v5 + 4 + strlen(a0));
    __sprintf_chk(v6,1,0xffffffffffffffff,"%s (%s)",v3,a0);
    if (v7)
      free(v7);
  }
label_616d:
  if (v4)
    free(v4);
  return v6;
}

// Function: sub_63b0 @ 0x63b0
void sub_63b0(unsigned long a0,long a1,long a2)
{
  unsigned long v1;
  unsigned long v2;
  long v3;
  int v4;
  int v5; // eax
  int *v6; // rax
  unsigned long v7; // stack - 0x68
  long v8; // stack - 0x60
  void *v9; // stack - 0x58
  
  v1 = *(unsigned long *)(a1 + 0x10);
  if (dat_20628) { // branch-flip
    v4 = (*dat_20628)(v1,0);
    if (v4)
      return;
    if (dat_20640) goto label_63fe;
  }
  else {
    if (!dat_20640) {
      signal(0xb,0); // return-dupe, tail-call
      return;
    }
label_63fe:
    v3 = dat_20648;
    v2 = *(unsigned long *)(a2 + 0xa0);
    if (dat_20648) {
      v6 = __errno_location();
      v4 = *v6;
      v5 = sub_6c80(v3,&v7);
      *v6 = v4;
      if (0 <= v5) {
        if (v7 <= v1) { // branch-flip
          if (v1 <= (unsigned long)(v8 - 1U)) goto label_64e5;
        }
        else if ((*v9)(v1,&v7)) {
label_64e5:
          (*dat_20640)(dat_20638 <= v2 && v2 <= dat_20638 + dat_20630,a2);
        }
      }
    }
  }
  if ((dat_20628) && (v4 = (*dat_20628)(v1,1), v4))
    return;
  signal(0xb,0);
  return;
}

// Function: sub_6540 @ 0x6540
void sub_6540(void)
{
  void *v1; // stack - 0xa8
  sigset_t v2;
  int v3; // stack - 0x20
  
  v1 = sub_63b0;
  sigemptyset(&v2);
  sigaddset(&v2,1);
  sigaddset(&v2,2);
  sigaddset(&v2,3);
  sigaddset(&v2,0xd);
  sigaddset(&v2,0xe);
  sigaddset(&v2,0xf);
  sigaddset(&v2,10);
  sigaddset(&v2,0xc);
  sigaddset(&v2,0x11);
  sigaddset(&v2,0x11);
  sigaddset(&v2,0x17);
  sigaddset(&v2,0x1d);
  sigaddset(&v2,0x1d);
  sigaddset(&v2,0x18);
  sigaddset(&v2,0x19);
  sigaddset(&v2,0x1a);
  sigaddset(&v2,0x1b);
  sigaddset(&v2,0x1e);
  sigaddset(&v2,0x1c);
  v3 = (-(unsigned int)(dat_20640 == 0) & 0xf8000000) + 0x8000004;
  sigaction(0xb,(sigaction *)&v1,NULL);
}

// Function: sub_66c0 @ 0x66c0
unsigned long sub_66c0(unsigned long a0)
{
  dat_20628 = a0;
  sub_6540();
  return 0;
}

// Function: sub_66e0 @ 0x66e0
void sub_66e0(void)
{
  dat_20628 = 0;
  if (dat_20640)
    return;
  signal(0xb,0); // tail-call
}

// Function: sub_6710 @ 0x6710
unsigned long sub_6710(void *a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  (*a0)(a1,a2,a3);
  return 1;
}

// Function: sub_6740 @ 0x6740
long sub_6740(unsigned long a0,unsigned long a1,unsigned long a2)
{
  int v1; // eax
  long v2; // rax
  unsigned long v3; // stack - 0x58
  char v4 [4];
  unsigned int v5; // stack - 0x50
  unsigned int v6; // stack - 0x4c
  unsigned long v7; // stack - 0x48
  
  if (!dat_20648) {
    v1 = sub_6c80(v4,&v3);
    if (0 <= v1) { // branch-flip
      v2 = CONCAT44(v6,v5) + -1;
      dat_20648 = v2;
    }
    else {
      v2 = dat_20648;
    }
    if (!v2) {
      v2 = 0xffffffff;
      return v2;
    }
  }
  v5 = 0;
  dat_20630 = a2;
  dat_20638 = a1;
  dat_20640 = a0;
  v3 = a1;
  v7 = a2;
  v1 = sigaltstack(&v3,NULL);
  if (v1 <= -1) {
    v2 = 0xffffffff;
    return v2;
  }
  sub_6540();
  v2 = 0;
  return v2;
}

// Function: sub_6820 @ 0x6820
void sub_6820(void) // return-dupe
{
  char v1 [8];
  unsigned int v2; // stack - 0x20
  
  dat_20640 = 0;
  if (dat_20628) // branch-flip
    sub_6540();
  else {
    signal(0xb,0);
  }
  v2 = 2;
  if (0 <= sigaltstack(v1,NULL))
    return;
  perror("gnulib sigsegv (stackoverflow_deinstall_handler)");
}

// Function: sub_68b0 @ 0x68b0
bool sub_68b0(long a0,struct_0 *a1)
{
  return (unsigned long)(a1->field_0x0 - a0) <= (unsigned long)(a1->field_0x0 - a1->field_0x18) >> 1;
}

// Function: sub_68d0 @ 0x68d0
unsigned long sub_68d0(struct_1 *a0,long *a1)
{
  unsigned char v1;
  long v2;
  long v3; // rcx
  int v4; // esi
  
  v4 = 0;
  v3 = 0;
  while (v2 = a0->field_0x0, v2 != a0->field_0x8) {
    v1 = *(unsigned char *)(a0->field_0x18 + v2);
    if (10 <= (unsigned int)v1 - 0x30) { // branch-flip
      if (6 <= (unsigned int)v1 - 0x41) { // branch-flip
        if (5 < (unsigned int)v1 - 0x61) goto label_6950;
        v3 = v3 * 0x10 + (long)(int)((unsigned int)v1 - 0x57);
      }
      else {
        v3 = v3 * 0x10 + (long)(int)((unsigned int)v1 - 0x37);
      }
    }
    else {
      v3 = v3 * 0x10 + (long)(int)((unsigned int)v1 - 0x30);
    }
    v4 += 1;
    a0->field_0x0 = v2 + 1;
  }
  a0->field_0x10 = 1;
label_6950:
  if (v4) {
    *a1 = v3;
    return 0;
  }
  return 0xffffffff;
}

// Function: sub_6960 @ 0x6960
unsigned long sub_6960(unsigned long a0,long *a1) // return-dupe
{
  unsigned long v1;
  unsigned long v2;
  unsigned long v3;
  long v4; // rax
  unsigned long v5; // rax
  long v6;
  void *v7;
  unsigned long v8;
  unsigned long v9; // stack - 0x58
  
  v1 = dat_20650;
  v3 = a0 * 2 - *a1;
  if (a0 < v3)
    return 0;
  v4 = (v3 / dat_20650) * dat_20650;
  v5 = (((unsigned long)(*a1 - 1U) / dat_20650 + 1) * dat_20650 - v4) / dat_20650;
  v3 = 1;
  if (v5 <= 1)
    return 1;
  do {
    v9 = v3;
    v3 = v9 * 2;
  } while (v3 < v5);
  v9 &= 0x7fffffffffffffff;
  while( true ) {
    v2 = v9;
    v7 = (void *)(v4 + v9 * v1);
    if (v9 < v5) {
      v8 = v9;
      do {
        if (0 <= mincore(v7,v1,&v6))
          return 0;
        v8 += v3 & 0xfffffffffffffffe;
        v7 = (void *)((long)v7 + v9 * v1 * 2);
      } while (v8 < v5);
    }
    if (v9 < 2) break;
    v9 >>= 1;
    v3 = v2;
  }
  return 1;
}

// Function: sub_6aa0 @ 0x6aa0
void * sub_6aa0(void *a0)
{
  unsigned long v1;
  unsigned long v2;
  char v3 [1032];
  unsigned long v4; // rsi
  unsigned long v5; // rsi
  void *v6; // r14
  
  v2 = 0x400;
  while( true ) {
    v1 = dat_20650;
    if (!a0)
      return NULL;
    if ((unsigned long)a0 / dat_20650 < v2)
      v2 = (unsigned long)a0 / dat_20650;
    v4 = dat_20650 * v2;
    v6 = (void *)((long)a0 - v4);
    if (mincore(v6,v4,v3) < 0) break;
    a0 = v6;
  }
  do {
    if (v2 == 1)
      return a0;
    while( true ) {
      v4 = v2 + 1 >> 1;
      v5 = v1 * v4;
      v6 = (void *)((long)a0 - v5);
      if (mincore(v6,v5,v3) < 0) break;
      v2 >>= 1;
      a0 = v6;
      if (v2 == 1)
        return v6;
    }
    v2 = v4;
  } while( true );
}

// Function: sub_6b80 @ 0x6b80
void * sub_6b80(long a0) // return-dupe
{
  unsigned long v1;
  unsigned long v2;
  unsigned long v3;
  char v4 [1032];
  void *v5; // rdi
  
  v1 = dat_20650;
  v5 = (void *)(a0 + dat_20650);
  if (!v5)
    return NULL;
  v2 = 0x400;
  while( true ) {
    if ((unsigned long)-(long)v5 / v1 < v2)
      v2 = (unsigned long)-(long)v5 / v1;
    if (mincore(v5,v1 * v2,v4) <= -1) break;
    v5 = (void *)((long)v5 + v1 * v2);
    if (!v5)
      return NULL;
  }
  do {
    v3 = v2;
    if (v2 == 1)
      return v5;
    while( true ) {
      v2 = v3 + 1 >> 1;
      if (mincore(v5,v1 * v2,v4) < 0) break;
      v3 >>= 1;
      v5 = (void *)((long)v5 + v1 * v2);
      if (v3 == 1)
        return v5;
    }
  } while( true );
}

// Function: sub_6c80 @ 0x6c80
unsigned long sub_6c80(char *a0,unsigned long *a1)
{
  char *v1;
  char *v10;
  char *v11; // r12
  char *v12;
  char *v13; // stack - 0x98
  char *v14; // stack - 0x90
  long v15; // stack - 0x88
  char *v16; // stack - 0x70
  char *v17; // stack - 0x68
  char *v18; // stack - 0x60
  char *v19; // stack - 0x58
  char *v2;
  char *v20; // stack - 0x50
  int v3; // eax
  int v4; // eax
  long v5;
  void *v6; // rax
  char *v7;
  unsigned long v8;
  char v9 [8];
  
  v3 = open("/proc/self/maps",0);
  if (0 <= v3) {
    v16 = v9;
    v11 = NULL;
    v15 = 0;
    v12 = (char *)0x1;
    v17 = NULL;
    v19 = NULL;
    v20 = NULL;
    do {
      do {
        if (s_104a <= v12) { // branch-flip
          v4 = (int)read(v3,v16,(unsigned long)v12);
          if (0 <= v4) {
            if (!v4) goto label_6ea8;
            if (v12 < (char *)(long)(v4 + 0x1049)) goto label_6d40;
            v5 = (long)v4;
            while( true ) {
              v4 = (int)read(v3,&v16[v5],(long)v12 - v5);
              if (v4 <= -1) break;
              if ((unsigned long)((long)v12 - v5) < (unsigned long)(long)(v4 + 0x1049)) goto label_6d40;
              if (!v4) {
                close(v3);
                v10 = NULL;
                goto label_6f10;
              }
              v5 = v4 + v5;
            }
          }
          if (*__errno_location() != 4) {
label_6ea8:
            close(v3);
            goto label_6de0;
          }
        }
        else {
label_6d40:
          if (v11) { // branch-flip
            v12 = (char *)((long)v12 * 2);
            if (!v12) goto label_6ea8;
            if (v17)
              munmap(v17,(unsigned long)v18);
          }
          else {
            v11 = (char *)(long)getpagesize();
            v12 = v11;
            if (v11 < s_104a) {
              do {
                v12 = (char *)((long)v12 * 2);
              } while (v12 < s_104a);
            }
          }
          v17 = (char *)mmap(0,v12,3,0x22,0xffffffff,0);
          if (v17 == (char *)0xffffffffffffffff) {
            close(v3);
            goto label_6df4;
          }
          v20 = &v17[(long)v12];
          v19 = v17;
          v16 = v17;
          v18 = v12;
        }
      } while (0 <= (long)lseek(v3,0,0));
      close(v3);
      v3 = open("/proc/self/maps",0);
    } while (0 <= v3);
label_6de0:
    if (v17)
      munmap(v17,(unsigned long)v18);
  }
label_6df4:
  v8 = dat_20650;
  if (!dat_20650) {
    v8 = (unsigned long)getpagesize();
    dat_20650 = v8;
  }
  v5 = ((unsigned long)a0 / v8) * v8;
  *a1 = sub_6aa0(v5);
  a1[1] = sub_6b80(v5);
  v6 = sub_6960;
label_6ff3:
  a1[2] = v6;
  return 0;
label_6f10:
  if (((((int)sub_68d0() < 0) || (v15 == v5)) || (v12 = &v16[v15], v15 = v15 + 1, *v12 != '-')) || ((int)sub_68d0() < 0)) goto label_6de0;
  do {
    if (v5 == v15) break;
    v12 = &v16[v15];
    v15 += 1;
  } while (*v12 != '\n');
  v7 = v14;
  if (v13 <= v19) { // branch-flip
    v1 = &v14[-1];
    if (v1 < &v20[-1]) goto label_6fc0;
    v2 = v10;
    if ((v13 < v19) && ((v2 = v19, v13 <= a0 && (a0 <= &v19[-1])))) {
      v14 = v19;
      goto label_6fce;
    }
    v10 = v2;
    if (v1 <= &v20[-1]) // branch-flip
      v7 = v10;
    else if ((v20 <= a0) && (a0 <= v1)) {
      v13 = v20;
      goto label_6fce;
    }
  }
  else {
label_6fc0:
    if ((v13 <= a0) && (a0 <= &v14[-1])) {
label_6fce:
      *a1 = v13;
      a1[1] = v14;
      a1[3] = v10;
      if (v17)
        munmap(v17,(unsigned long)v18);
      v6 = sub_68b0;
      goto label_6ff3;
    }
  }
  v10 = v7;
  goto label_6f10;
}

// Function: sub_70a0 @ 0x70a0
void * sub_70a0(void *a0,unsigned long a1)
{
  void *v1; // rax
  unsigned long v2; // rdx
  bool v3; // zf
  
  v1 = memchr(a0,0,a1);
  v2 = (long)v1 + (1U - (long)a0);
  v3 = v1 != NULL;
  v1 = (void *)a1;
  if (v3)
    v1 = (void *)v2;
  return v1;
}

// Function: sub_70e0 @ 0x70e0
void sub_70e0(void)
{
  __assert_fail("iter->cur.wc == 0","mbiter.h",0xa3,"mbiter_multi_next"); // no-return
}

// Function: sub_7110 @ 0x7110
unsigned char * sub_7110(char *a0,int a1)
{
  unsigned char v1;
  int v10; // r15d
  unsigned char *v11; // stack - 0x90
  char v12; // stack - 0x74
  unsigned char *v13; // stack - 0x70
  long v14; // stack - 0x68
  char v15; // stack - 0x60
  long v2;
  bool v3;
  unsigned char *v4; // rax
  long *v5; // rax
  mbstate_t v6; // stack - 0x7c
  unsigned int v7 [7]; // stack - 0x5c
  unsigned char *v8;
  unsigned char *v9;
  
  v4 = (unsigned char *)strdup(a0);
  if (!v4)
    sub_8270(); // no-return
  if (2 <= __ctype_get_mb_cur_max()) { // branch-flip
    if (a1) {
      v13 = v4;
      v3 = 0;
      v9 = &v4[strlen((char *)v4)];
      v6 = 0;
      v8 = v4;
      while( true ) {
        v12 = 0;
        if (v8 >= v9) break;
        v12 = 0;
        if (v3) {
label_7564:
          v14 = sub_8380(v7,v8,(long)v9 - (long)v8,&v6);
          if (v14 != -1) { // branch-flip
            if (v14 != -2) {
              if (!v14) {
                v14 = 1;
                if (*v13) {
                  v14 = 1; // return-dupe
                  __assert_fail("*iter->cur.ptr == \'\\0\'","mbiter.h",0xa2,"mbiter_multi_next"); // no-return
                }
                if (v7[0]) {
                  v14 = 1;
                  sub_70e0(); // no-return, return-dupe
                }
              }
              v15 = '\x01';
              if (mbsinit(&v6))
                v3 = 0;
              goto label_7525;
            }
            v14 = (long)v9 - (long)v13;
            v15 = '\0';
            v12 = 1;
          }
          else {
            v14 = 1;
            v15 = '\0';
            v12 = 1;
          }
label_7303:
          v8 = v13;
          break;
        }
        if (!(*(unsigned int *)((unsigned long)(*v8 >> 5) * 4 + 0xc260) >> (*v8 & 0x1f) & 1)) {
          if (mbsinit(&v6)) {
            v3 = 1;
            goto label_7564;
          }
          __assert_fail("mbsinit (&iter->state)","mbiter.h",0x87,"mbiter_multi_next"); // return-dupe, no-return
        }
        v14 = 1;
        v7[0] = (unsigned int)(char)*v8;
        v15 = '\x01';
label_7525:
        v12 = 1;
        if (!iswspace(v7[0])) goto label_7303;
        v8 = &v13[v14];
        v13 = v8;
      }
      memmove(v4,v8,strlen((char *)v8) + 1);
      if (a1 == 1)
        return v4;
    }
    v13 = v4;
    v10 = 0;
    v3 = 0;
    v9 = &v4[strlen((char *)v4)];
    v6 = 0;
    v8 = v4;
    if (v4 < v9) {
      do {
        v12 = 0;
        if (v3) {
label_727a:
          v14 = sub_8380(v7,v8,(long)v9 - (long)v8,&v6);
          if (v14 != -1) { // branch-flip
            if (v14 != -2) { // branch-flip
              if (!v14) {
                v14 = 1;
                if (*v13) {
                  v14 = 1;
                  __assert_fail("*iter->cur.ptr == \'\\0\'","mbiter.h",0xa2,"mbiter_multi_next");
                }
                if (v7[0]) {
                  v14 = 1;
                  sub_70e0();
                }
              }
              v15 = '\x01';
              if (mbsinit(&v6))
                v3 = 0;
            }
            else {
              v14 = (long)v9 - (long)v13;
              v15 = '\0';
            }
          }
          else {
            v14 = 1;
            v15 = '\0';
          }
        }
        else {
          if (!(*(unsigned int *)((unsigned long)(*v8 >> 5) * 4 + 0xc260) >> (*v8 & 0x1f) & 1)) {
            if (!mbsinit(&v6))
              __assert_fail("mbsinit (&iter->state)","mbiter.h",0x87,"mbiter_multi_next");
            v3 = 1;
            goto label_727a;
          }
          v14 = 1;
          v7[0] = (unsigned int)(char)*v8;
          v15 = '\x01';
        }
        v12 = 1;
        if (v10) { // branch-flip
          if (v10 != 1) { // branch-flip
            if ((v10 != 2) || (!v15)) goto label_71ab;
            if (!iswspace(v7[0]))
              v10 = 1;
          }
          else if ((((v15) && (iswspace(v7[0]))) && (v15)) && (iswspace(v7[0]))) {
            v11 = v13;
            v10 = 2;
          }
        }
        else if (v15) { // branch-flip
          if (!iswspace(v7[0])) {
            if (!v15) goto label_71ab;
            v10 = 1;
            iswspace(v7[0]);
          }
        }
        else {
label_71ab:
          v10 = 1;
        }
        v8 = &v13[v14];
        v13 = v8;
      } while (v8 < v9);
      if (v10 == 2)
        *v11 = 0;
    }
  }
  else {
    if (a1) {
      v1 = *v4;
      v9 = v4;
      if (v1) {
        v5 = __ctype_b_loc();
        do {
          if (!(*(unsigned char *)(*v5 + 1 + (unsigned long)v1 * 2) & 0x20)) break;
          v1 = v9[1];
          v9 = &v9[1];
        } while (v1);
      }
      memmove(v4,v9,strlen((char *)v9) + 1);
      if (a1 == 1)
        return v4;
    }
    v9 = &v4[strlen((char *)v4) - 1];
    if (v4 <= v9) {
      v2 = *(long *)__ctype_b_loc();
      do {
        if (!(*(unsigned char *)(v2 + 1 + (unsigned long)*v9 * 2) & 0x20))
          return v4;
        *v9 = 0;
        v9 = &v9[-1];
      } while (v4 <= v9);
    }
  }
  return v4;
}

// Function: sub_76b0 @ 0x76b0
long sub_76b0(FILE *a0,char *a1,char *a2,char *a3,unsigned long *a4,unsigned long a5)
{
  long v1;
  unsigned long v10; // stack - 0x48
  char *v2; // rax
  unsigned long v3;
  unsigned long v4;
  unsigned long v5;
  unsigned long v6;
  unsigned long v7; // stack - 0x60
  unsigned long v8; // stack - 0x58
  unsigned long v9; // stack - 0x50
  
  if (a1) // branch-flip
    __fprintf_chk(a0,1,"%s (%s) %s\n",a1,a2,a3);
  else {
    __fprintf_chk(a0,1,"%s %s\n",a2,a3);
  }
  __fprintf_chk(a0,1,"Copyright %s %d Free Software Foundation, Inc.",dcgettext(NULL,"(C)",5),0x7e5);
  fputs_unlocked("\n",a0);
  __fprintf_chk(a0,1,dcgettext(NULL,"License GPLv3+: GNU GPL version 3 or later <%s>.\nThis is free software: you are free to change and redistribute it.\nThere is NO WARRANTY, to the extent permitted by law.\n",5),"https://gnu.org/licenses/gpl.html");
  fputs_unlocked("\n",a0);
  if (10 <= a5) {
    v10 = a4[7];
    v2 = "Written by %s, %s, %s,\n%s, %s, %s, %s,\n%s, %s, and others.\n";
    v9 = a4[6];
    v8 = a4[5];
    v7 = a4[4];
    v6 = a4[3];
    v5 = a4[2];
    v3 = a4[1];
    v4 = *a4;
    v2 = dcgettext(NULL,v2,5); // return-dupe
label_77dd:
    v1 = CONCAT44(dat_4,__fprintf_chk(a0,1,v2,v4,v3,v5,v6,v7,v8,v9,v10));
label_7816:
    return v1;
  }
  v1 = (long)*(int *)(a5 * 4 + 0xc1b8) + 0xc1b8;
  switch(a5) {
    case 0:
      goto label_7816;
    case 1:
      v6 = *a4;
      v2 = dcgettext(NULL,"Written by %s.\n",5);
      return __fprintf_chk(a0,1,v2,v6); // tail-call
    case 2:
      v6 = a4[1];
      v5 = *a4;
      v2 = dcgettext(NULL,"Written by %s and %s.\n",5);
      return __fprintf_chk(a0,1,v2,v5,v6); // tail-call
    case 3:
      v6 = a4[2];
      v5 = a4[1];
      v3 = *a4;
      v2 = dcgettext(NULL,"Written by %s, %s, and %s.\n",5);
      return __fprintf_chk(a0,1,v2,v3,v5,v6); // tail-call
    case 4:
      v1 = a4[3];
      v6 = a4[2];
      v5 = a4[1];
      v3 = *a4;
      v2 = dcgettext(NULL,"Written by %s, %s, %s,\nand %s.\n",5);
      break;
    case 5:
      v1 = a4[3];
      v6 = a4[2];
      v5 = a4[1];
      v3 = *a4;
      v2 = dcgettext(NULL,"Written by %s, %s, %s,\n%s, and %s.\n",5);
      break;
    case 6:
      v8 = a4[5];
      v7 = a4[4];
      v6 = a4[3];
      v5 = a4[2];
      v3 = a4[1];
      v4 = *a4;
      v2 = dcgettext(NULL,"Written by %s, %s, %s,\n%s, %s, and %s.\n",5);
      return __fprintf_chk(a0,1,v2,v4,v3,v5,v6,v7,v8); // return-dupe
    case 7:
      v8 = a4[5];
      v7 = a4[4];
      v6 = a4[3];
      v5 = a4[2];
      v3 = a4[1];
      v4 = *a4;
      v2 = dcgettext(NULL,"Written by %s, %s, %s,\n%s, %s, %s, and %s.\n",5);
      return __fprintf_chk(a0,1,v2,v4,v3,v5,v6,v7,v8);
    case 8:
      v10 = a4[7];
      v9 = a4[6];
      v8 = a4[5];
      v7 = a4[4];
      v6 = a4[3];
      v5 = a4[2];
      v3 = a4[1];
      v4 = *a4;
      v2 = dcgettext(NULL,"Written by %s, %s, %s,\n%s, %s, %s, %s,\nand %s.\n",5);
      goto label_77dd;
    case 9:
      v10 = a4[7];
      v2 = "Written by %s, %s, %s,\n%s, %s, %s, %s,\n%s, and %s.\n";
      v9 = a4[6];
      v8 = a4[5];
      v7 = a4[4];
      v6 = a4[3];
      v5 = a4[2];
      v3 = a4[1];
      v4 = *a4;
      v2 = dcgettext(NULL,v2,5);
      v1 = CONCAT44(dat_4,__fprintf_chk(a0,1,v2,v4,v3,v5,v6,v7,v8,v9,v10));
      return v1;
    
  }
  __fprintf_chk(a0,1,v2,v3,v5,v6);
  return v1;
}

// Function: sub_7b30 @ 0x7b30
void sub_7b30(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,long *a4)
{
  long v1;
  long v2;
  
  v2 = 0;
  v1 = *a4;
  while (v1) {
    v2 += 1;
    v1 = a4[v2];
  }
  sub_76b0(); // tail-call
}

// Function: sub_7b50 @ 0x7b50
void sub_7b50(FILE *a0,char *a1,char *a2,char *a3,struct_3 *a4)
{
  unsigned int v1;
  long *v2;
  long v3;
  long v4 [11];
  long v5; // r9
  
  v5 = 0;
  do {
    v1 = a4->field_0x0;
    if (0x30 <= v1) { // branch-flip
      v2 = a4->field_0x8;
      a4->field_0x8 = &v2[1];
      v3 = *v2;
      v4[v5] = v3;
    }
    else {
      a4->field_0x0 = v1 + 8;
      v3 = *(long *)((unsigned long)v1 + a4->field_0x10);
      v4[v5] = v3;
    }
  } while ((v3) && (v5 = v5 + 1, v5 != 10));
  sub_76b0(a0,a1,a2,a3);
}

// Function: sub_7bf0 @ 0x7bf0
void sub_7bf0(FILE *a0,char *a1,char *a2,unsigned long a3,unsigned long a4,unsigned long a5)
{
  long v1;
  unsigned long v2;
  long v3 [4];
  long v4 [11];
  long *v5;
  long *v6; // rsi
  long v7; // r9
  long v8 [2]; // stack - 0x18
  
  v2 = 0x20;
  v8[0] = a4;
  v5 = (long *)&Stack0000000000000008;
  v8[1] = a5;
  v7 = 0;
  do {
    if (0x30 <= (unsigned int)v2) { // branch-flip
      v6 = &v5[1];
      v1 = *v5;
      v4[v7] = v1;
      v5 = v6;
    }
    else {
      v1 = *(long *)((long)v3 + v2);
      v4[v7] = v1;
      v2 = (unsigned long)((unsigned int)v2 + 8);
    }
  } while ((v1) && (v7 = v7 + 1, v7 != 10));
  sub_76b0(a0,a1,a2);
}

// Function: sub_7cd0 @ 0x7cd0
void sub_7cd0(void)
{
  fputs_unlocked("\n",stdout);
  __printf_chk(1,dcgettext(NULL,"Report bugs to: %s\n",5),"bug-diffutils@gnu.org");
  __printf_chk(1,dcgettext(NULL,"%s home page: <%s>\n",5),"GNU diffutils","https://www.gnu.org/software/diffutils/");
  __printf_chk(1,dcgettext(NULL,"General help using GNU software: <%s>\n",5),"https://www.gnu.org/gethelp/"); // tail-call
}

// Function: sub_7d80 @ 0x7d80
void sub_7d80(unsigned long a0,unsigned long a1)
{
  if (reallocarray(NULL,a0,a1))
    return;
  sub_8270(); // no-return
}

// Function: sub_7db0 @ 0x7db0
void sub_7db0(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!reallocarray(a0,a1,a2)) && ((!a0 || ((a1 && (a2))))))
    sub_8270(); // no-return
}

// Function: sub_7df0 @ 0x7df0
void sub_7df0(unsigned long a0)
{
  if (malloc(a0))
    return;
  sub_8270(); // no-return
}

// Function: sub_7e10 @ 0x7e10
void sub_7e10(void)
{
  sub_7df0(); // tail-call
}

// Function: sub_7e20 @ 0x7e20
void sub_7e20(unsigned long a0)
{
  if (malloc(a0))
    return;
  sub_8270(); // no-return
}

// Function: sub_7e40 @ 0x7e40
void sub_7e40(void *a0,unsigned long a1)
{
  if ((!realloc(a0,a1)) && ((!a0 || (a1))))
    sub_8270(); // no-return
}

// Function: sub_7e80 @ 0x7e80
void sub_7e80(void *a0,unsigned long a1)
{
  if (realloc(a0,a1 | a1 == 0))
    return;
  sub_8270(); // no-return
}

// Function: sub_7eb0 @ 0x7eb0
void sub_7eb0(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!reallocarray(a0,a1,a2)) && ((!a0 || ((a1 && (a2))))))
    sub_8270(); // no-return
}

// Function: sub_7ef0 @ 0x7ef0
void sub_7ef0(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!a1) || (!a2)) {
    a1 = 1;
    a2 = 1;
  }
  if (reallocarray(a0,a1,a2))
    return;
  sub_8270(); // no-return
}

// Function: sub_7f30 @ 0x7f30
void sub_7f30(void *a0,unsigned long *a1)
{
  unsigned long v1; // rax
  unsigned long v2; // rbx
  unsigned long v3;
  
  v3 = *a1;
  if (a0) { // branch-flip
    v1 = (v3 >> 1) + 1;
    v2 = v3 + v1;
    if (!CARRY8(v3,v1)) {
      if ((reallocarray(a0,v2,1)) || (!v2)) {
        *a1 = v2;
        return;
      }
    }
  }
  else {
    if (!v3)
      v3 = 0x80;
    if (reallocarray(NULL,v3,1)) {
      *a1 = v3;
      return;
    }
  }
  sub_8270(); // no-return
}

// Function: sub_7fc0 @ 0x7fc0
void sub_7fc0(void *a0,unsigned long *a1,unsigned long a2)
{
  char v1 [16];
  unsigned long v2; // rax
  unsigned long v3; // rbx
  unsigned long v4;
  
  v4 = *a1;
  if (a0) { // branch-flip
    v2 = (v4 >> 1) + 1;
    v3 = v4 + v2;
    if (!CARRY8(v4,v2)) {
      if (((reallocarray(a0,v3,a2)) || (!v3)) || (!a2)) {
        *a1 = v3;
        return;
      }
    }
  }
  else {
    if (!v4) {
      v1._8_8_ = 0;
      v1._0_8_ = a2;
      v4 = SUB168((ZEXT816(0) << 0x40 | ZEXT816(0x80)) / v1._0_16_,0) + (unsigned long)(0x81 <= a2);
    }
    if (reallocarray(NULL,v4,a2)) {
      *a1 = v4;
      return;
    }
  }
  sub_8270(); // no-return
}

// Function: sub_8050 @ 0x8050
void sub_8050(void *a0,long *a1,long a2,long a3,long a4)
{
  long v1;
  long v2;
  long v3;
  long v4;
  
  v1 = *a1;
  v2 = (v1 >> 1) + v1;
  if (SCARRY8(v1 >> 1,v1))
    v2 = 0x7fffffffffffffff;
  v4 = a3;
  if (v2 <= a3)
    v4 = v2;
  if (0 <= a3)
    v2 = v4;
  v3 = v2 * a4;
  if (SEXT816(v3) != SEXT816(v2) * SEXT816(a4)) // branch-flip
    v4 = 0x7fffffffffffffff;
  else {
    if (0x80 <= v3) goto label_810e;
    v4 = 0x80;
  }
  v2 = v4 / a4;
  v3 = v4 - v4 % a4;
label_810e:
  if (!a0)
    *a1 = 0;
  if (((a2 <= v2 - v1) || ((v2 = v1 + a2, !SCARRY8(v1,a2) && (((v2 <= a3 || (a3 <= -1)) && (v3 = v2 * a4, SEXT816(v3) == SEXT816(v2) * SEXT816(a4))))))) && ((realloc(a0,v3) || ((a0 && (!v3)))))) {
    *a1 = v2;
    return;
  }
  sub_8270(); // no-return
}

// Function: sub_8150 @ 0x8150
void sub_8150(unsigned long a0,unsigned long a1)
{
  if (calloc(a0,a1))
    return;
  sub_8270(); // no-return
}

// Function: sub_8170 @ 0x8170
void sub_8170(unsigned long a0)
{
  sub_8150(a0,1); // tail-call
}

// Function: sub_8180 @ 0x8180
void sub_8180(unsigned long a0,unsigned long a1)
{
  if (calloc(a0,a1))
    return;
  sub_8270(); // no-return
}

// Function: sub_81a0 @ 0x81a0
void sub_81a0(unsigned long a0)
{
  sub_8180(a0,1); // tail-call
}

// Function: sub_81b0 @ 0x81b0
void sub_81b0(char *a0,unsigned long a1)
{
  memcpy((void *)sub_7df0(a1),a0,a1); // tail-call
}

// Function: sub_81e0 @ 0x81e0
void sub_81e0(void *a0,unsigned long a1)
{
  memcpy((void *)sub_7e20(a1),a0,a1); // tail-call
}

// Function: sub_8210 @ 0x8210
void sub_8210(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)sub_7e20(a1 + 1);
  *(char *)((long)v1 + a1) = 0;
  memcpy(v1,a0,a1); // tail-call
}

// Function: sub_8250 @ 0x8250
void sub_8250(char *a0)
{
  sub_81b0(a0,strlen(a0) + 1); // tail-call
}

// Function: sub_8270 @ 0x8270
void sub_8270(void)
{
  error(dat_10020,0,"%s",dcgettext(NULL,"memory exhausted",5));
  abort(); // no-return
}

// Function: sub_82b0 @ 0x82b0
void sub_82b0(void)
{
  int v1; // eax
  
  v1 = sub_9540();
  if (!v1)
    return;
  error(dat_10020,v1,dcgettext(NULL,"standard file descriptors",5)); // tail-call
}

// Function: sub_82f0 @ 0x82f0
int sub_82f0(unsigned long a0,long a1,unsigned long a2,long *a3,unsigned long *a4)
{
  int v1; // eax
  
  v1 = sub_95f0(a0,a1,a2,a3,a4);
  if (v1 < 0) {
    if (*__errno_location() == 0xc)
      sub_8270(); // no-return
  }
  return v1;
}

// Function: sub_8320 @ 0x8320
long sub_8320(void)
{
  long v1; // rax
  
  v1 = sub_9870();
  if (!v1) {
    if (*__errno_location() == 0xc)
      sub_8270(); // no-return
  }
  return v1;
}

// Function: sub_8350 @ 0x8350
long sub_8350(char *a0,long a1,long a2)
{
  long v1; // rax
  
  v1 = sub_9ac0(a0,a1,a2);
  if (!v1) {
    if (*__errno_location() == 0xc)
      sub_8270(); // no-return
  }
  return v1;
}

// Function: sub_8380 @ 0x8380
unsigned long sub_8380(unsigned int *a0,char *a1,unsigned long a2,mbstate_t *a3)
{
  unsigned long v1; // rax
  long v2;
  
  if (!a0)
    a0 = &v2;
  v1 = mbrtowc(a0,a1,a2,a3);
  if (((0xfffffffffffffffe <= v1) && (a2)) && (!sub_8480(0))) {
    v1 = 1;
    *a0 = (unsigned int)(unsigned char)*a1;
  }
  return v1;
}

// Function: sub_8410 @ 0x8410
int sub_8410(long a0,long a1)
{
  unsigned int v1;
  unsigned char v2;
  long v3; // rdx
  unsigned char v4;
  unsigned int v5;
  
  if (a0 == a1)
    return 0;
  v3 = 0;
  do {
    v4 = *(unsigned char *)(a0 + v3);
    v1 = (unsigned int)v4;
    v2 = *(unsigned char *)(a1 + v3);
    v5 = (unsigned int)v2;
    if (0x1a <= v1 - 0x41) { // branch-flip
      if (v5 - 0x41 <= 0x19) {
label_842d:
        v5 += 0x20;
        v2 += 0x20;
      }
      if (!v1) break;
    }
    else {
      v1 += 0x20;
      v4 += 0x20;
      if (v5 - 0x41 <= 0x19) goto label_842d;
    }
    v3 += 1;
  } while (v4 == v2);
  return v1 - v5;
}

// Function: sub_8480 @ 0x8480
bool sub_8480(unsigned long a0)
{
  char v1 [264];
  bool v2; // r12b
  
  v2 = 0;
  if ((!sub_9480(a0,v1,0x101)) && (strcmp(v1,"C")))
    v2 = strcmp(v1,"POSIX") != 0;
  return v2;
}

// Function: sub_8510 @ 0x8510
char * sub_8510(void) // early-return
{
  char *v1; // rax
  
  v1 = nl_langinfo(0xe);
  if (!v1)
    return "ASCII";
  if (*v1)
    return v1;
  return "ASCII";
}

// Function: sub_8550 @ 0x8550
int sub_8550(unsigned int a0)
{
  int v1; // eax
  
  v1 = wcwidth(a0);
  if (0 <= v1)
    return v1;
  return (unsigned long)(iswcntrl(a0) == 0);
}

// Function: sub_8580 @ 0x8580
void sub_8580(struct_4 *a0,struct_5 *a1) // return-dupe
{
  char v1;
  long v2;
  struct_5 *v3;
  
  v3 = (struct_5 *)a1->field_0x0;
  if (v3 == &a1[1])
    v3 = memcpy(&a0[1],&a1[1],a1->field_0x8);
  v2 = a1->field_0x8;
  a0->field_0x0 = v3;
  a0->field_0x8 = v2;
  v1 = a1->field_0x10;
  a0->field_0x10 = v1;
  if (!v1)
    return;
  a0->field_0x14 = a1->field_0x14;
}

// Function: sub_85e0 @ 0x85e0
unsigned int sub_85e0(unsigned char a0)
{
  return *(unsigned int *)((unsigned long)(a0 >> 5) * 4 + 0xc260) >> (a0 & 0x1f) & 1;
}

// Function: sub_8600 @ 0x8600
unsigned long sub_8600(char *a0,long a1,unsigned long a2,unsigned long *a3)
{
  unsigned long v1; // rax
  char *v10;
  char v2;
  long v3;
  char v4 [8];
  char *v5;
  char *v6; // rsp
  char *v7; // rsp
  char *v8;
  unsigned long v9;
  
  v5 = v4;
  v7 = v4;
  v8 = v4;
  v9 = a2 * 8;
  if ((0 <= (long)v9) && (!(a2 >> 0x3d))) {
    if (0xfa1 <= v9) // branch-flip
      v9 = sub_9df0(v9);
    else {
      v8 = v4;
      while (v7 != &v4[-(v9 + 0x36 & 0xfffffffffffff000)]) {
        v5 = &v8[-0x1000];
        v6 = &v8[-0x1000];
        *(unsigned long *)&v8[-8] = *(unsigned long *)&v8[-8];
        v7 = &v8[-0x1000];
        v8 = v6;
      }
      v9 = (unsigned long)((unsigned int)(v9 + 0x36) & 0xff0);
      v3 = -v9;
      v8 = &v5[v3];
      if (v9)
        *(unsigned long *)&v5[-8] = *(unsigned long *)&v5[-8];
      v9 = ((unsigned long)&v5[v3 + 0xf] & 0xfffffffffffffff0) + 0x1f & 0xffffffffffffffe0;
    }
    if (v9) {
      *(unsigned long *)(v9 + 8) = 1;
      if (3 <= a2) {
        v3 = 0;
        v1 = 2;
        do {
          v2 = *(char *)(a1 + v3);
          while (*(char *)(a1 + -1 + v1) != v2) {
            if (!v3) {
              *(unsigned long *)(v9 + v1 * 8) = v1;
              v3 = 0;
              goto label_86f8;
            }
            v3 -= *(long *)(v9 + v3 * 8);
            v2 = *(char *)(a1 + v3);
          }
          v3 += 1;
          *(unsigned long *)(v9 + v1 * 8) = v1 - v3;
label_86f8:
          v1 += 1;
        } while (a2 != v1);
      }
      *a3 = 0;
      v2 = *a0;
      if (v2) {
        v1 = 0;
        v10 = a0;
        do {
          if (*(char *)(a1 + v1) != v2) { // branch-flip
            if (v1) { // branch-flip
              v10 = &v10[*(long *)(v9 + v1 * 8)];
              v1 -= *(long *)(v9 + v1 * 8);
            }
            else {
              v10 = &v10[1];
              a0 = &a0[1];
            }
          }
          else {
            v1 += 1;
            a0 = &a0[1];
            if (a2 == v1) {
              *a3 = v10;
              break;
            }
          }
          v2 = *a0;
        } while (v2);
      }
      *(unsigned long *)&v8[-8] = 0x874e;
      sub_9e40(v9);
      return 1;
    }
  }
  return 0;
}

// Function: sub_87a0 @ 0x87a0
void sub_87a0(struct_2 *a0)
{
  unsigned char v1;
  unsigned char *v2;
  unsigned long v3; // rax
  long v4; // rax
  
  if (!a0->field_0x0) {
    v1 = *a0->field_0x10;
    if (*(unsigned int *)((unsigned long)(v1 >> 5) * 4 + 0xc260) >> (v1 & 0x1f) & 1) {
      a0->field_0x18 = 1;
      v1 = *a0->field_0x10;
      a0->field_0x20 = '\x01';
      a0->field_0x24 = (int)(char)v1;
      goto label_87e1;
    }
    if (!mbsinit((mbstate_t *)&a0->field_0x1[3]))
      __assert_fail("mbsinit (&iter->state)","mbuiter.h",0x8f,"mbuiter_multi_next"); // no-return
    a0->field_0x0 = '\x01';
  }
  v3 = __ctype_get_mb_cur_max();
  v2 = a0->field_0x10;
  v4 = sub_8380(&a0->field_0x24,v2,sub_70a0(v2,v3),(mbstate_t *)&a0->field_0x1[3]);
  a0->field_0x18 = v4;
  if (v4 == -1) {
    a0->field_0x18 = 1;
    a0->field_0x20 = '\0';
    a0->field_0xc = '\x01';
    return;
  }
  if (v4 == -2) {
    v3 = strlen((char *)a0->field_0x10);
    a0->field_0x20 = '\0';
    a0->field_0x18 = v3;
    a0->field_0xc = '\x01';
    return;
  }
  if (!v4) {
    a0->field_0x18 = 1;
    if (*a0->field_0x10)
      __assert_fail("*iter->cur.ptr == \'\\0\'","mbuiter.h",0xab,"mbuiter_multi_next"); // no-return
    if (a0->field_0x24)
      __assert_fail("iter->cur.wc == 0","mbuiter.h",0xac,"mbuiter_multi_next"); // no-return
  }
  a0->field_0x20 = '\x01';
  if (mbsinit((mbstate_t *)&a0->field_0x1[3])) {
    a0->field_0x0 = '\0';
    a0->field_0xc = '\x01';
    return;
  }
label_87e1:
  a0->field_0xc = '\x01';
  return;
}

// Function: sub_8910 @ 0x8910
char sub_8910(void *a0,unsigned char *a1,unsigned long *a2)
{
  char v1;
  unsigned char v10 [32];
  char v11 [4]; // stack - 0x88
  char v12 [4]; // stack - 0xc8
  void **v13;
  char *v14; // rsp
  void **v15; // rsp
  void **v16;
  unsigned char *v17;
  unsigned long v18;
  unsigned long *v19;
  void *v2;
  unsigned long *v20; // r13
  long v21;
  void *v22;
  unsigned long *v23; // stack - 0xf0
  void *v24; // stack - 0xe8
  char v25; // stack - 0xd9
  void *v26; // stack - 0xd8
  char *v27; // stack - 0xd0
  unsigned long v28; // stack - 0xc4
  char v29; // stack - 0xbc
  char v3 [16];
  void *v30; // stack - 0xb8
  long v31; // stack - 0xb0
  char v32; // stack - 0xa8
  int v33; // stack - 0xa4
  unsigned long v34; // stack - 0x84
  char v35; // stack - 0x7c
  unsigned char *v36; // stack - 0x78
  unsigned long v37; // stack - 0x70
  char v38; // stack - 0x68
  int v39; // stack - 0x64
  unsigned long *v4;
  void *v5;
  char v6; // al
  unsigned int v7; // eax
  unsigned long *v8;
  void *v9; // stack - 0xf8
  
  v15 = &v9;
  v13 = &v9;
  v16 = &v9;
  v9 = a0;
  v23 = a2;
  v8 = (void *)sub_9e70(a1);
  v24 = v8;
  v3 = ZEXT816(v8);
  v8 = SUB168(ZEXT816(0x38) * v3,0);
  if ((0 <= (long)v8) && (!SUB168(ZEXT816(0x38) * v3,8))) {
    v18 = (long)v24 * 0x38;
    if (0xfa1 <= v18) { // branch-flip
      v8 = (unsigned long *)sub_9df0(v18);
      v19 = v8;
      v5 = v24;
    }
    else {
      v16 = &v9;
      while (v15 != (void **)((long)&v9 - (v18 + 0x36 & 0xfffffffffffff000))) {
        v13 = (char *)((long)v16 + -0x1000);
        v14 = (char *)((long)v16 + -0x1000);
        *(unsigned long *)((long)v16 + -8) = *(unsigned long *)((long)v16 + -8);
        v15 = (char *)((long)v16 + -0x1000);
        v16 = (void **)v14;
      }
      v7 = (unsigned int)(v18 + 0x36) & 0xff0;
      v8 = (unsigned long)(unsigned long)v7;
      v16 = (char *)((long)v13 + -(long)v8);
      if (v8)
        *(unsigned long *)((long)v13 + -8) = *(unsigned long *)((long)v13 + -8);
      v19 = (unsigned long *)(((unsigned long)((long)v13 + -(long)v8 + 0xf) & 0xfffffffffffffff0) + 0x1f & 0xffffffffffffffe0);
      v5 = v24;
    }
    if (v19) {
      v8 = &v19[3];
      v11[0] = 0;
      v27 = v11;
      v34 = 0;
      v24 = v5;
      v36 = a1;
      while( true ) {
        v35 = 0;
        *(unsigned long *)((long)v16 + -8) = 0x8a9e;
        sub_87a0(v27);
        v25 = v38;
        if ((v38) && (!v39)) break;
        if (v36 != v10) // branch-flip
          v8[-3] = v36;
        else {
          *(unsigned long *)((long)v16 + -8) = 0x8acb;
          memcpy(v8,v10,v37);
          v8[-3] = v8;
        }
        v8[-2] = v37;
        *(char *)&v8[-1] = v38;
        if (v38)
          *(int *)((long)v8 + -4) = v39;
        v36 = &v36[v37];
        v8 = &v8[6];
      }
      v21 = 0;
      v19[(long)v5 * 6 + 1] = 1;
      v26 = (void *)0x2;
      v4 = v19;
      if ((void *)0x3 <= v24) {
label_8b10:
        v20 = &v4[6];
        v1 = *(char *)&v4[8];
        do {
          v8 = &v19[v21 * 6];
          if ((v1) && (*(char *)&v8[2])) {
            if (*(int *)((long)v4 + 0x44) == *(int *)((long)v8 + 0x14)) {
label_8b66:
              v21 += 1;
              v8 = (long)((long)v26 - v21);
              v19[(long)((long)v5 * 6 + (long)v26)] = v8;
              goto label_8b7b;
            }
          }
          else {
            v18 = v4[7];
            if (v18 == v8[1]) {
              v22 = (void *)*v8;
              v2 = (void *)*v20;
              *(unsigned long *)((long)v16 + -8) = 0x8b62;
              v7 = memcmp(v2,v22,v18);
              if (!v7) goto label_8b66;
            }
          }
          if (!v21) goto label_8d70;
          v21 -= v19[(long)v5 * 6 + v21];
        } while( true );
      }
label_8b9b:
      v11[0] = 0;
      v22 = NULL;
      v12[0] = 0;
      *v23 = 0;
      v28 = 0;
      v29 = '\0';
      v34 = 0;
      v35 = '\0';
      v30 = v9;
      v36 = v9;
      v6 = 0;
      v1 = v6;
label_8bf0:
      v6 = v1;
      if (!v6) goto label_8c3f;
      if (!v38) goto label_8c51;
label_8bfa:
      if (v39) {
        v8 = &v19[(long)v22 * 6];
        if (!*(char *)&v8[2]) goto label_8c5c;
        if (*(int *)((long)v8 + 0x14) != v39) goto label_8c66;
        v17 = v36;
        v18 = v37;
label_8c23:
        v36 = (void *)((long)v17 + v18);
        v35 = '\0';
        v22 = (void *)((long)v22 + 1);
        if (v24 != v22) {
label_8c3f:
          do {
            *(unsigned long *)((long)v16 + -8) = 0x8c4b;
            sub_87a0(v27);
            if (v38) goto label_8bfa;
label_8c51:
            v8 = &v19[(long)v22 * 6];
label_8c5c:
            v18 = v37;
            if (v8[1] == v37) {
              v2 = (void *)*v8;
              v26 = v36;
              *(unsigned long *)((long)v16 + -8) = 0x8cd6;
              v7 = memcmp(v2,v36,v37);
              v17 = v26;
              if (!v7) goto label_8c23;
            }
label_8c66:
            if (v22) {
              v21 = v19[(long)((long)v5 * 6 + (long)v22)];
              v8 = (void *)((long)v22 - v21);
              v1 = v35;
              v26 = v8;
              if (!v21) goto label_8bf0;
              v22 = v8;
              if (!v29) goto label_8d45;
              goto label_8d1e;
            }
            if (!v29) {
              *(unsigned long *)((long)v16 + -8) = 0x8c80;
              sub_87a0(v12);
            }
            if ((v32) && (!v33)) goto label_8d9e;
            v29 = '\0';
            v30 = (void *)((long)v30 + v31);
            v35 = '\0';
            v36 = (void *)((long)v36 + v37);
          } while( true );
        }
        *v23 = (unsigned long)v30;
      }
      *(unsigned long *)((long)v16 + -8) = 0x8d99;
      sub_9e40(v19);
      v6 = v25;
      return v6;
    }
  }
  v6 = '\0';
  return v6;
label_8d70:
  v19[(long)((long)v5 * 6 + (long)v26)] = v26;
label_8b7b:
  v26 = (void *)((long)v26 + 1);
  v4 = v20;
  if (v24 == v26) goto label_8b9b;
  goto label_8b10;
label_8d1e:
  v26 = v22;
  if ((v32) && (!v33)) {
label_8d9e:
    *(unsigned long *)((long)v16 + -8) = 0x8da3; // no-return
    abort();
  }
  v29 = '\0';
  v30 = (void *)((long)v30 + v31);
  v21 -= 1;
  if (!v21) goto label_8d60;
label_8d45:
  *(unsigned long *)((long)v16 + -8) = 0x8d4d;
  sub_87a0(v12);
  v22 = v26;
  goto label_8d1e;
label_8d60:
  v1 = v35;
  goto label_8bf0;
}

// Function: sub_8db0 @ 0x8db0
char * sub_8db0(char *a0,char *a1) // early-return, return-dupe
{
  char *v1;
  char *v10; // stack - 0x190
  char v11 [4]; // stack - 0x188
  char v12 [4]; // stack - 0x88
  unsigned int v13; // stack - 0x108
  unsigned long v14; // stack - 0xc8
  char v15 [4]; // stack - 0x148
  unsigned long v16;
  long v17;
  unsigned long v18; // stack - 0x1e0
  unsigned char v19; // stack - 0x1d1
  char v2;
  unsigned long v20; // stack - 0x1a8
  unsigned long v21; // stack - 0x184
  char v22; // stack - 0x17c
  char *v23; // stack - 0x178
  unsigned long v24; // stack - 0x170
  char v25; // stack - 0x168
  int v26; // stack - 0x164
  unsigned long v27; // stack - 0x144
  char v28; // stack - 0x13c
  char *v29; // stack - 0x138
  char v3;
  long v30; // stack - 0x130
  unsigned char v31; // stack - 0x128
  int v32; // stack - 0x124
  unsigned int v33; // stack - 0x104
  unsigned int v34; // stack - 0x100
  unsigned int v35; // stack - 0xfc
  char *v36; // stack - 0xf8
  unsigned long v37; // stack - 0xf0
  unsigned int v38; // stack - 0xe8
  int v39; // stack - 0xe4
  unsigned char v4; // al
  unsigned long v40; // stack - 0xe0
  unsigned long v41; // stack - 0xd8
  unsigned long v42; // stack - 0xd0
  char v43 [5]; // stack - 0xc0
  undefined3 v44; // stack - 0xbb
  char *v45; // stack - 0xb8
  unsigned long v46; // stack - 0xb0
  unsigned long v47; // stack - 0xa8
  unsigned long v48; // stack - 0xa0
  unsigned long v49; // stack - 0x98
  unsigned int v5; // eax
  unsigned long v50; // stack - 0x90
  unsigned long v51; // stack - 0x84
  char v52; // stack - 0x7c
  char *v53; // stack - 0x78
  unsigned long v54; // stack - 0x70
  char v55; // stack - 0x68
  int v56; // stack - 0x64
  char *v6;
  unsigned long v7;
  unsigned long v8;
  unsigned long v9; // rdx
  
  if (2 <= __ctype_get_mb_cur_max()) { // branch-flip
    v11[0] = 0;
    v21 = 0;
    v22 = 0;
    v23 = a1;
    sub_87a0(v11);
    if ((!v25) || (v26)) {
      v19 = 1;
      v15[0] = 0;
      v27 = 0;
      v28 = '\0';
      v13 &= 0xffffff00;
      v34 = 0;
      v33 = 0;
      v18 = 0;
      v20 = 0;
      v8 = 0;
      v29 = a1;
      v36 = a0;
      do {
        v35 &= 0xffffff00;
        sub_87a0(&v13);
        if ((char)v38) { // branch-flip
          if (!v39)
            return NULL;
          v4 = 10 <= v18 & v19;
          if (v4) {
            if (v18 * 5 <= v8) { // branch-flip
label_9330:
              v17 = v8 - v20;
              if (v17) { // branch-flip
                if (!v28) goto label_937d;
                while ((!v31 || (v32))) {
                  v28 = '\0';
                  v29 = &v29[v30];
                  v17 -= 1;
                  if (!v17) goto label_93b0;
label_937d:
                  sub_87a0(v15);
                }
                if (v28) goto label_9440;
label_93b0:
                sub_87a0(v15);
              }
              else if (!v28) goto label_93b0;
              v19 = v31;
              if (v31) { // branch-flip
                if (!v32) {
label_9440:
                  v19 = sub_8910(a0,a1,&v10);
                  if (v19)
                    return v10;
                }
              }
              else {
                v19 = 1;
              }
              v20 = v8;
              if (!(char)v38) goto label_9106;
            }
            else {
              v19 = v4;
            }
          }
          v9 = v8 + 1;
          if (!v25) goto label_9106;
          if (v39 == v26) goto label_9146;
label_9098:
          v6 = v36;
          v16 = v37;
        }
        else {
          v4 = 10 <= v18 & v19;
          if (v4) { // branch-flip
            if (v18 * 5 <= v8) goto label_9330;
            v19 = v4;
          }
          else {
          }
label_9106:
          v16 = v37;
          v6 = v36;
          v9 = v8 + 1;
          if (v37 == v24) {
            if (!memcmp(v36,v23,v37)) {
label_9146:
              v9 = v8 + 1;
              v14 = CONCAT44(v33,v13);
              v12[0] = 0;
              v47 = CONCAT44(v39,v38);
              v52 = 0;
              v46 = v37;
              v45 = &v36[v37];
              v44 = (undefined3)(v35 >> 8);
              v43[4] = 0;
              v43._0_4_ = v34;
              v51 = 0;
              v48 = v40;
              v49 = v41;
              v50 = v42;
              v53 = a1;
              sub_87a0(v12);
              if ((v55) && (!v56))
                abort(); // no-return
              v53 = &v53[v54];
              do {
                v9 += 1;
                v52 = 0;
                sub_87a0(v12);
                if ((v55) && (!v56))
                  return v36;
                if (!v43[4])
                  sub_87a0(&v14);
                v16 = v54;
                v1 = v53;
                v8 = v46;
                v6 = v45;
                if ((char)v47) { // branch-flip
                  if (!v47._4_4_)
                    return NULL;
                  if (!v55) goto label_92be;
                  if (v47._4_4_ != v56) goto label_9098;
                }
                else {
label_92be:
                  if ((v46 != v54) || (memcmp(v45,v53,v46))) goto label_9098;
                }
                v45 = &v6[v8];
                v43[4] = 0;
                v53 = &v1[v16];
              } while( true );
            }
          }
        }
        v18 += 1;
        v36 = &v6[v16];
        v8 = v9;
      } while( true );
    }
  }
  else {
    v2 = *a1;
    if (v2) {
      v3 = *a0;
      if (!v3)
        return NULL;
      v16 = 0;
      v9 = 0;
      v8 = 0;
      v5 = 1;
      v6 = a1;
      do {
        v16 += 1;
        v7 = v9 + 1;
        v1 = &a0[1];
        if (v2 == v3) {
          v3 = a1[1];
          if (!v3)
            return a0;
          while( true ) {
            v17 = v7 - v9;
            if (!a0[v17])
              return NULL;
            v7 += 1;
            if (a0[v17] != v3) break;
            v3 = a1[v7 - v9];
            if (!v3)
              return a0;
          }
        }
        v9 = v7;
        if (!*v1)
          return NULL;
        v4 = 10 <= v16 & (unsigned char)v5;
        if (v4) {
          if (v16 * 5 <= v9) { // branch-flip
            if (v6) {
              v6 = &v6[strnlen(v6,v9 - v8)];
              v8 = v9;
              if (*v6) goto label_8edd;
            }
            v5 = sub_8600(v1,a1,strlen(a1),&v10);
            if ((char)v5)
              return v10;
            v6 = NULL;
          }
          else {
label_8edd:
            v5 = (unsigned int)v4;
          }
        }
        v3 = *v1;
        a0 = v1;
      } while( true );
    }
  }
  return a0;
}

// Function: sub_9480 @ 0x9480
unsigned long sub_9480(int a0,char *a1,unsigned long a2) // return-dupe x2
{
  char *v1; // rax
  unsigned long v2; // rax
  
  v1 = setlocale(a0,NULL);
  if (!v1) {
    if (!a2)
      return 0x16;
    *a1 = '\0';
    return 0x16;
  }
  v2 = strlen(v1);
  if (a2 <= v2) {
    if (!a2)
      return 0x22;
    memcpy(a1,v1,a2 - 1);
    a1[a2 - 1] = '\0';
    return 0x22;
  }
  memcpy(a1,v1,v2 + 1);
  return 0;
}

// Function: sub_9530 @ 0x9530
void sub_9530(int a0)
{
  setlocale(a0,NULL); // tail-call
}

// Function: sub_9540 @ 0x9540
int sub_9540(void) // early-return x2
{
  int v1; // eax
  int v2; // ebx
  int v3; // ebp
  
  v3 = 1;
  v2 = 0;
  do {
    v1 = sub_9b80(v2,1);
    if (0 <= v1) { // branch-flip
label_9573:
      if (v3 == 3) {
        v1 = 0;
        return v1;
      }
    }
    else {
      if ((v2) || (v1 = __open_2("/dev/full",1), v1 < 0)) {
        v1 = __open_2("/dev/null",(unsigned int)(v2 == 0));
        if (v1 < 0) {
          v1 = *__errno_location();
          return v1;
        }
        if (3 <= v1) {
          close(v1); // return-dupe
          v1 = 0;
          return v1;
        }
        goto label_9573;
      }
      if (3 <= v1) {
        close(v1);
        v1 = 0;
        return v1;
      }
    }
    v2 += 1;
    v3 += 1;
  } while( true );
}

// Function: sub_95f0 @ 0x95f0
unsigned long sub_95f0(unsigned long a0,long a1,unsigned long a2,long *a3,unsigned long *a4) // return-dupe
{
  int v1;
  char *v2; // rax
  char *v3;
  char *v4; // stack - 0x1050
  char *v5; // stack - 0x1058
  char v6 [4104];
  unsigned long v7; // stack - 0x1068
  long v8; // stack - 0x1060
  
  v2 = NULL;
  iconv(a2,0,0,0,0);
  v7 = a0;
  v8 = a1;
  if (a1) {
    do {
      v4 = (char *)0x1000;
      v5 = v6;
      if (iconv(a2,&v7,&v8,&v5,&v4) == -1) {
        v1 = *__errno_location();
        if (v1 != 7) {
          if (v1 != 0x16)
            return 0xffffffff;
          break;
        }
      }
      v2 = &v5[(long)v2 - (long)v6];
    } while (v8);
  }
  v4 = (char *)0x1000;
  v5 = v6;
  if (iconv(a2,0,0,&v5,&v4) == -1)
    return 0xffffffff;
  v2 = &v5[(long)v2 - (long)v6];
  if (!v2) {
    *a4 = 0;
    return 0;
  }
  v3 = (char *)*a3;
  if (((!v3) || ((char *)*a4 < v2)) && (v3 = malloc((unsigned long)v2), !v3)) {
    *__errno_location() = 0xc;
    return 0xffffffff;
  }
  iconv(a2,0,0,0,0);
  v7 = a0;
  v8 = a1;
  v5 = v3;
  v4 = v2;
  do {
    if (!a1) {
label_9792:
      if (iconv(a2,0,0,&v5,&v4) != -1) {
        if (v4)
          abort(); // no-return
        *a3 = (long)v3;
        *a4 = (unsigned long)v2;
        return 0;
      }
label_983e:
      if ((char *)*a3 != v3) {
        free(v3);
        return 0xffffffff;
      }
      return 0xffffffff;
    }
    if (iconv(a2,&v7,&v8,&v5,&v4) == -1) {
      if (*__errno_location() != 0x16) goto label_983e;
      goto label_9792;
    }
    a1 = v8;
  } while( true );
}

// Function: sub_9870 @ 0x9870
char * sub_9870(char *a0,unsigned long a1) // return-dupe
{
  unsigned long v1;
  unsigned long v10;
  char *v11;
  long v12; // rdx
  char *v2;
  char *v3; // rax
  int *v4; // rax
  unsigned long v5;
  unsigned long v6; // stack - 0x48
  char *v7; // stack - 0x50
  unsigned long v8; // stack - 0x58
  char *v9; // stack - 0x60
  
  v9 = a0;
  v8 = strlen(a0);
  v10 = v8;
  if (v8 <= 0xfffffff)
    v10 = v8 << 4;
  v5 = v10 + 1;
  v2 = malloc(v5);
  if (!v2) {
    *__errno_location() = 0xc;
    return NULL;
  }
  iconv(a1,0,0,0,0,v10,v10);
  v11 = v2;
  while (v7 = v2, v6 = v10, iconv(a1,&v9,&v8,&v7,&v6) == -1) {
    v4 = __errno_location();
    if (*v4 == 0x16) goto label_9a0d;
    if (*v4 != 7) {
      free(v11); // return-dupe
      return NULL;
    }
    v1 = v5 * 2;
    v12 = (long)v7 - (long)v11;
    if ((v1 <= v5) || (v3 = realloc(v11,v1), !v3)) {
      *v4 = 0xc;
      free(v11);
      return NULL;
    }
    v2 = &v3[v12];
    v10 = (v1 - 1) - v12;
    v5 = v1;
    v11 = v3;
  }
label_9a0d:
  do {
    if (iconv(a1,0,0,&v7,&v6) != -1) {
      v2 = &v7[1];
      *v7 = 0;
      v10 = (long)v2 - (long)v11;
      if (v5 <= v10)
        return v11;
      v7 = v2;
      v2 = realloc(v11,v10);
      if (!v2)
        return v11;
      return v2;
    }
    v4 = __errno_location();
    if (*v4 != 7) {
      free(v11);
      return NULL;
    }
    v10 = v5 * 2;
    v12 = (long)v7 - (long)v11;
    if ((v10 <= v5) || (v2 = realloc(v11,v10), !v2)) {
      *v4 = 0xc;
      free(v11);
      return NULL;
    }
    v7 = &v2[v12];
    v6 = (v10 - 1) - v12;
    v5 = v10;
    v11 = v2;
  } while( true );
}

// Function: sub_9ac0 @ 0x9ac0
char * sub_9ac0(char *a0,long a1,long a2)
{
  int v1;
  char *v2; // rax
  long v3; // rax
  int *v4; // rax
  
  if ((!*a0) || (!sub_8410(a1,a2))) {
    v2 = strdup(a0);
    if (!v2)
      *__errno_location() = 0xc;
    return v2;
  }
  v3 = iconv_open(a2,a1);
  if (v3 == -1)
    return NULL;
  v2 = (char *)sub_9870(a0,v3);
  if (v2) {
    if (0 <= (int)iconv_close(v3))
      return v2;
    free(v2);
    return NULL;
  }
  v4 = __errno_location();
  v1 = *v4;
  iconv_close(v3);
  *v4 = v1;
  return NULL;
}

// Function: sub_9b80 @ 0x9b80
int sub_9b80(int a0,int a1,unsigned long a2)
{
  int v1;
  int v2; // eax
  unsigned int v3; // eax
  unsigned long v4;
  int *v5; // rax
  unsigned long v6; // stack - 0x38
  
  if (!a1)
    return fcntl(a0,0,a2 & 0xffffffff);
  if (a1 == 0x406) {
    if (0 <= dat_20658) { // branch-flip
      v2 = fcntl(a0,0x406,a2 & 0xffffffff);
      if ((0 <= v2) || (*__errno_location() != 0x16)) {
        dat_20658 = 1;
        return v2;
      }
      v2 = fcntl(a0,0,a2 & 0xffffffff);
      if (v2 < 0)
        return v2;
      dat_20658 = -1;
    }
    else {
      v2 = fcntl(a0,0,a2 & 0xffffffff);
      if (v2 < 0)
        return v2;
      if (dat_20658 != -1)
        return v2;
    }
    v3 = fcntl(v2,1);
    if ((0 <= (int)v3) && (fcntl(v2,2,(unsigned long)(v3 | 1)) != -1))
      return v2;
    v5 = __errno_location();
    v1 = *v5;
    close(v2);
    *v5 = v1;
    return -1;
  }
  if (0xc <= a1) { // branch-flip
    if (0xb <= (unsigned int)(a1 - 0x400U)) {
      v2 = fcntl(a0,a1,v6); // return-dupe
      return v2;
    }
    v4 = 1L << ((unsigned char)(a1 - 0x400U) & 0x3f);
    if (v4 & 0x2c5) {
      return fcntl(a0,a1,(unsigned long)(unsigned int)v6); // return-dupe
    }
    v4 &= 0x502;
  }
  else {
    if (a1 <= 0) {
      v2 = fcntl(a0,a1,v6);
      return v2;
    }
    v4 = 1L << ((unsigned char)a1 & 0x3f);
    if (v4 & 0x514)
      return fcntl(a0,a1,(unsigned long)(unsigned int)v6);
    v4 &= 0xa0a;
  }
  if (v4)
    return fcntl(a0,a1);
  v2 = fcntl(a0,a1,v6);
  return v2;
}

// Function: sub_9df0 @ 0x9df0
long sub_9df0(unsigned long a0)
{
  long v1;
  void *v2; // rax
  unsigned long v3; // rax
  
  v3 = a0 + 0x20;
  if ((0 <= (long)v3) && (a0 <= v3)) {
    v2 = malloc(v3);
    if (v2) {
      v3 = (long)v2 + 0x10U & 0xffffffffffffffe0;
      v1 = v3 + 0x10;
      *(char *)(v3 + 0xf) = (char)v1 - (char)v2;
      return v1;
    }
    return 0;
  }
  return 0;
}

// Function: sub_9e40 @ 0x9e40
void sub_9e40(unsigned long a0)
{
  if (a0 & 0xf)
    abort(); // no-return
  if (!(a0 & 0x10))
    return;
  free((void *)(a0 - *(unsigned char *)(a0 - 1))); // tail-call
}

// Function: sub_9e70 @ 0x9e70
unsigned long sub_9e70(unsigned char *a0)
{
  bool v1;
  char v10; // stack - 0x58
  unsigned char *v2;
  unsigned long v3; // rax
  mbstate_t v4; // stack - 0x74
  int v5 [9]; // stack - 0x54
  unsigned long v6; // r12
  char v7; // stack - 0x6c
  unsigned char *v8; // stack - 0x68
  unsigned long v9; // stack - 0x60
  
  if (__ctype_get_mb_cur_max() <= 1)
    return strlen((char *)a0); // tail-call
  v6 = 0;
  v1 = 0;
  v4 = 0;
  v8 = a0;
  do {
    v7 = 0;
    if (v1) {
label_9f4c:
      v3 = __ctype_get_mb_cur_max();
      v2 = v8;
      v9 = sub_8380(v5,v2,sub_70a0(v8,v3),&v4);
      if (v9 != 0xffffffffffffffff) { // branch-flip
        if (v9 != 0xfffffffffffffffe) {
          if (!v9) {
            v9 = 1;
            if (*v8)
              __assert_fail("*iter->cur.ptr == \'\\0\'","mbuiter.h",0xab,"mbuiter_multi_next"); // no-return
            if (v5[0])
              __assert_fail("iter->cur.wc == 0","mbuiter.h",0xac,"mbuiter_multi_next"); // no-return
          }
          v10 = 1;
          if (mbsinit(&v4))
            v1 = 0;
          goto label_9f28;
        }
        v9 = strlen((char *)v8);
        v10 = 0;
      }
      else {
        v9 = 1;
        v10 = 0;
      }
    }
    else {
      if (!(*(unsigned int *)((unsigned long)(*v8 >> 5) * 4 + 0xc260) >> (*v8 & 0x1f) & 1)) {
        if (!mbsinit(&v4))
          __assert_fail("mbsinit (&iter->state)","mbuiter.h",0x8f,"mbuiter_multi_next"); // no-return
        v1 = 1;
        goto label_9f4c;
      }
      v9 = 1;
      v5[0] = (int)(char)*v8;
      v10 = 1;
label_9f28:
      if (!v5[0])
        return v6;
    }
    v8 = &v8[v9];
    v6 += 1;
  } while( true );
}

// Function: _DT_FINI @ 0xa0bc
void _DT_FINI(void)
{
  return;
}
