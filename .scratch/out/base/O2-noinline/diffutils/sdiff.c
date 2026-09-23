// Function: _DT_INIT @ 0x3000
void _DT_INIT(void) // return-dupe
{
  if (!dat_efd8)
    return;
  (*dat_efd8)();
}

// Function: sub_3020 @ 0x3020
void sub_3020(void)
{
  (*dat_ed00)(); // jump-as-call
}

// Function: __cxa_finalize @ 0x35b0
void __cxa_finalize(void)
{
  (*dat_efe8)(); // jump-as-call
}

// Function: getenv @ 0x35c0
char * getenv(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_ed08)(); // jump-as-call
  return v1;
}

// Function: raise @ 0x35d0
int raise(int a0)
{
  int v1; // eax
  
  v1 = (*dat_ed10)(); // jump-as-call
  return v1;
}

// Function: free @ 0x35e0
void free(void *a0)
{
  (*dat_ed18)(); // jump-as-call
}

// Function: strtoimax @ 0x35f0
void strtoimax(void)
{
  (*dat_ed20)(); // jump-as-call
}

// Function: __mempcpy_chk @ 0x3600
void * __mempcpy_chk(void *a0,void *a1,unsigned long a2,unsigned long a3)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_ed28)(); // jump-as-call
  return v1;
}

// Function: abort @ 0x3610
void abort(void)
{
  (*dat_ed30)(); // jump-as-call
}

// Function: __errno_location @ 0x3620
int * __errno_location(void)
{
  int *v1; // rax
  
  v1 = (int *)(*dat_ed38)(); // jump-as-call
  return v1;
}

// Function: unlink @ 0x3630
int unlink(char *a0)
{
  int v1; // eax
  
  v1 = (*dat_ed40)(); // jump-as-call
  return v1;
}

// Function: strncmp @ 0x3640
int strncmp(char *a0,char *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_ed48)(); // jump-as-call
  return v1;
}

// Function: _exit @ 0x3650
void _exit(int a0)
{
  (*dat_ed50)(); // jump-as-call
}

// Function: iconv @ 0x3660
void iconv(void)
{
  (*dat_ed58)(); // jump-as-call
}

// Function: puts @ 0x3670
int puts(char *a0)
{
  int v1; // eax
  
  v1 = (*dat_ed60)(); // jump-as-call
  return v1;
}

// Function: sigaction @ 0x3680
int sigaction(int a0,void *a1,void *a2)
{
  int v1; // eax
  
  v1 = (*dat_ed68)(); // jump-as-call
  return v1;
}

// Function: iswcntrl @ 0x3690
void iswcntrl(void)
{
  (*dat_ed70)(); // jump-as-call
}

// Function: reallocarray @ 0x36a0
void * reallocarray(void *a0,unsigned long a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_ed78)(); // jump-as-call
  return v1;
}

// Function: getchar_unlocked @ 0x36b0
void getchar_unlocked(void)
{
  (*dat_ed80)(); // jump-as-call
}

// Function: fcntl @ 0x36c0
int fcntl(int a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_ed88)(); // jump-as-call
  return v1;
}

// Function: write @ 0x36d0
long write(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_ed90)(); // jump-as-call
  return v1;
}

// Function: fread_unlocked @ 0x36e0
void fread_unlocked(void)
{
  (*dat_ed98)(); // jump-as-call
}

// Function: textdomain @ 0x36f0
char * textdomain(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_eda0)(); // jump-as-call
  return v1;
}

// Function: __open_2 @ 0x3700
void __open_2(void)
{
  (*dat_eda8)(); // jump-as-call
}

// Function: fclose @ 0x3710
int fclose(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_edb0)(); // jump-as-call
  return v1;
}

// Function: bindtextdomain @ 0x3720
char * bindtextdomain(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_edb8)(); // jump-as-call
  return v1;
}

// Function: dcgettext @ 0x3730
char * dcgettext(char *a0,char *a1,int a2)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_edc0)(); // jump-as-call
  return v1;
}

// Function: __ctype_get_mb_cur_max @ 0x3740
unsigned long __ctype_get_mb_cur_max(void)
{
  unsigned long v1; // rax
  
  v1 = (*dat_edc8)(); // jump-as-call
  return v1;
}

// Function: strlen @ 0x3750
unsigned long strlen(char *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_edd0)(); // jump-as-call
  return v1;
}

// Function: __stack_chk_fail @ 0x3760
void __stack_chk_fail(void)
{
  (*dat_edd8)(); // jump-as-call
}

// Function: mmap @ 0x3770
void mmap(void)
{
  (*dat_ede0)(); // jump-as-call
}

// Function: getopt_long @ 0x3780
int getopt_long(int a0,char **a1,char *a2,void *a3,int *a4)
{
  int v1; // eax
  
  v1 = (*dat_ede8)(); // jump-as-call
  return v1;
}

// Function: mbrtowc @ 0x3790
unsigned long mbrtowc(void *a0,char *a1,unsigned long a2,void *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_edf0)(); // jump-as-call
  return v1;
}

// Function: dup2 @ 0x37a0
int dup2(int a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_edf8)(); // jump-as-call
  return v1;
}

// Function: strchr @ 0x37b0
char * strchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_ee00)(); // jump-as-call
  return v1;
}

// Function: strrchr @ 0x37c0
char * strrchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_ee08)(); // jump-as-call
  return v1;
}

// Function: lseek @ 0x37d0
void lseek(void)
{
  (*dat_ee10)(); // jump-as-call
}

// Function: __assert_fail @ 0x37e0
void __assert_fail(char *a0,char *a1,unsigned int a2,char *a3)
{
  (*dat_ee18)(); // jump-as-call
}

// Function: fputs @ 0x37f0
int fputs(char *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_ee20)(); // jump-as-call
  return v1;
}

// Function: strnlen @ 0x3800
unsigned long strnlen(char *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  v1 = (*dat_ee28)(); // jump-as-call
  return v1;
}

// Function: close @ 0x3810
int close(int a0)
{
  int v1; // eax
  
  v1 = (*dat_ee30)(); // jump-as-call
  return v1;
}

// Function: pipe @ 0x3820
int pipe(int *a0)
{
  int v1; // eax
  
  v1 = (*dat_ee38)(); // jump-as-call
  return v1;
}

// Function: memchr @ 0x3830
void * memchr(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_ee40)(); // jump-as-call
  return v1;
}

// Function: read @ 0x3840
long read(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_ee48)(); // jump-as-call
  return v1;
}

// Function: memcmp @ 0x3850
int memcmp(void *a0,void *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_ee50)(); // jump-as-call
  return v1;
}

// Function: fputs_unlocked @ 0x3860
int fputs_unlocked(char *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_ee58)(); // jump-as-call
  return v1;
}

// Function: rawmemchr @ 0x3870
void * rawmemchr(void *a0,int a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_ee60)(); // jump-as-call
  return v1;
}

// Function: ferror_unlocked @ 0x3880
int ferror_unlocked(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_ee68)(); // jump-as-call
  return v1;
}

// Function: calloc @ 0x3890
void * calloc(unsigned long a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_ee70)(); // jump-as-call
  return v1;
}

// Function: strcmp @ 0x38a0
int strcmp(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_ee78)(); // jump-as-call
  return v1;
}

// Function: signal @ 0x38b0
void signal(void)
{
  (*dat_ee80)(); // jump-as-call
}

// Function: sigemptyset @ 0x38c0
int sigemptyset(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_ee88)(); // jump-as-call
  return v1;
}

// Function: stat @ 0x38d0
int stat(char *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_ee90)(); // jump-as-call
  return v1;
}

// Function: memcpy @ 0x38e0
void * memcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_ee98)(); // jump-as-call
  return v1;
}

// Function: putchar_unlocked @ 0x38f0
int putchar_unlocked(int a0)
{
  int v1; // eax
  
  v1 = (*dat_eea0)(); // jump-as-call
  return v1;
}

// Function: kill @ 0x3900
int kill(int a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_eea8)(); // jump-as-call
  return v1;
}

// Function: mincore @ 0x3910
void mincore(void)
{
  (*dat_eeb0)(); // jump-as-call
}

// Function: wcwidth @ 0x3920
int wcwidth(int a0)
{
  int v1; // eax
  
  v1 = (*dat_eeb8)(); // jump-as-call
  return v1;
}

// Function: iswalnum @ 0x3930
void iswalnum(void)
{
  (*dat_eec0)(); // jump-as-call
}

// Function: malloc @ 0x3940
void * malloc(unsigned long a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_eec8)(); // jump-as-call
  return v1;
}

// Function: nl_langinfo @ 0x3950
char * nl_langinfo(int a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_eed0)(); // jump-as-call
  return v1;
}

// Function: mkstemp @ 0x3960
void mkstemp(void)
{
  (*dat_eed8)(); // jump-as-call
}

// Function: fwrite_unlocked @ 0x3970
unsigned long fwrite_unlocked(void *a0,unsigned long a1,unsigned long a2,void *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_eee0)(); // jump-as-call
  return v1;
}

// Function: realloc @ 0x3980
void * realloc(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_eee8)(); // jump-as-call
  return v1;
}

// Function: fdopen @ 0x3990
void * fdopen(int a0,char *a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_eef0)(); // jump-as-call
  return v1;
}

// Function: munmap @ 0x39a0
int munmap(void *a0,unsigned long a1)
{
  int v1; // eax
  
  v1 = (*dat_eef8)(); // jump-as-call
  return v1;
}

// Function: setlocale @ 0x39b0
char * setlocale(int a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_ef00)(); // jump-as-call
  return v1;
}

// Function: __printf_chk @ 0x39c0
int __printf_chk(int a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_ef08)(); // jump-as-call
  return v1;
}

// Function: iconv_close @ 0x39d0
void iconv_close(void)
{
  (*dat_ef10)(); // jump-as-call
}

// Function: iswspace @ 0x39e0
int iswspace(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_ef18)(); // jump-as-call
  return v1;
}

// Function: mempcpy @ 0x39f0
void * mempcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_ef20)(); // jump-as-call
  return v1;
}

// Function: memmove @ 0x3a00
void * memmove(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_ef28)(); // jump-as-call
  return v1;
}

// Function: error @ 0x3a10
void error(int a0,int a1,char *a2,...)
{
  (*dat_ef30)(); // jump-as-call
}

// Function: waitpid @ 0x3a20
int waitpid(int a0,int *a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_ef38)(); // jump-as-call
  return v1;
}

// Function: open @ 0x3a30
int open(char *a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_ef40)(); // jump-as-call
  return v1;
}

// Function: fopen @ 0x3a40
void * fopen(char *a0,char *a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_ef48)(); // jump-as-call
  return v1;
}

// Function: perror @ 0x3a50
void perror(char *a0)
{
  (*dat_ef50)(); // jump-as-call
}

// Function: execvp @ 0x3a60
void execvp(void)
{
  (*dat_ef58)(); // jump-as-call
}

// Function: getpagesize @ 0x3a70
int getpagesize(void)
{
  int v1; // eax
  
  v1 = (*dat_ef60)(); // jump-as-call
  return v1;
}

// Function: exit @ 0x3a80
void exit(int a0)
{
  (*dat_ef68)(); // jump-as-call
}

// Function: __fprintf_chk @ 0x3a90
int __fprintf_chk(void *a0,int a1,char *a2,...)
{
  int v1; // eax
  
  v1 = (*dat_ef70)(); // jump-as-call
  return v1;
}

// Function: fflush_unlocked @ 0x3aa0
int fflush_unlocked(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_ef78)(); // jump-as-call
  return v1;
}

// Function: strdup @ 0x3ab0
char * strdup(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_ef80)(); // jump-as-call
  return v1;
}

// Function: mbsinit @ 0x3ac0
int mbsinit(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_ef88)(); // jump-as-call
  return v1;
}

// Function: sigaltstack @ 0x3ad0
void sigaltstack(void)
{
  (*dat_ef90)(); // jump-as-call
}

// Function: sigaddset @ 0x3ae0
int sigaddset(void *a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_ef98)(); // jump-as-call
  return v1;
}

// Function: fork @ 0x3af0
int fork(void)
{
  int v1; // eax
  
  v1 = (*dat_efa0)(); // jump-as-call
  return v1;
}

// Function: feof_unlocked @ 0x3b00
void feof_unlocked(void)
{
  (*dat_efa8)(); // jump-as-call
}

// Function: __ctype_b_loc @ 0x3b10
void * __ctype_b_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_efb0)(); // jump-as-call
  return v1;
}

// Function: iconv_open @ 0x3b20
void iconv_open(void)
{
  (*dat_efb8)(); // jump-as-call
}

// Function: __sprintf_chk @ 0x3b30
int __sprintf_chk(char *a0,int a1,unsigned long a2,char *a3,...)
{
  int v1; // eax
  
  v1 = (*dat_efc0)(); // jump-as-call
  return v1;
}

// Function: sub_3b40 @ 0x3b40
undefined16 sub_3b40(void)
{
  char v1 [16];
  unsigned long v2; // rax
  
  *__errno_location() = 0xc;
  v1._8_8_ = 0;
  v1._0_8_ = v2;
  return v1._0_16_ << 0x40;
}

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

// Function: sub_41f0 @ 0x41f0
void sub_41f0(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_efc8)(sub_3b60,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: sub_4220 @ 0x4220
void sub_4220(void)
{
  return;
}

// Function: _FINI_0 @ 0x4290
void _FINI_0(void)
{
  if (!dat_f088) {
    if (dat_efe8)
      __cxa_finalize(dat_f008);
    sub_4220();
    dat_f088 = 1;
    return;
  }
}

// Function: _INIT_0 @ 0x42d0
void _INIT_0(void)
{
  return;
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

// Function: sub_4bd0 @ 0x4bd0
long sub_4bd0(unsigned long a0,void *a1)
{
  long v1; // rax
  
  v1 = fread_unlocked(a0,1,0x10000,a1);
  if (!v1) {
    if (ferror_unlocked(a1))
      sub_49e0(dcgettext(NULL,"read failed",5)); // no-return
  }
  return v1;
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

// Function: sub_4d80 @ 0x4d80
int sub_4d80(long a0,void *a1)
{
  void *v1;
  void *v2; // rax
  unsigned long v3; // rbp
  unsigned long v4; // r14
  
  v4 = 0x100;
  while( true ) {
    v1 = *(void **)(a0 + 8);
    v2 = rawmemchr(v1,10);
    v3 = (long)v2 - (long)v1;
    if (v4 <= v3)
      return 0;
    memcpy(a1,v1,v3);
    if (v2 < *(void **)(a0 + 0x18)) break;
    if (!sub_4c30(a0))
      return -(unsigned int)(v3 == 0);
    a1 = (void *)((long)a1 + v3);
    v4 -= v3;
  }
  *(char *)((long)a1 + v3) = 0;
  *(long *)(a0 + 8) = (long)v2 + 1;
  return 1;
}

// Function: sub_4e40 @ 0x4e40
void sub_4e40(unsigned long a0,int a1,unsigned long a2)
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
  sub_48a0(); // no-return
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

// Function: sub_5690 @ 0x5690
char * sub_5690(char *a0)
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

// Function: sub_56f0 @ 0x56f0
void sub_56f0(char *a0)
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

// Function: sub_5720 @ 0x5720
void sub_5720(void)
{
  return;
}

// Function: sub_5730 @ 0x5730
void sub_5730(int a0)
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
  (*dat_f600)();
  if (a0) // branch-flip
    v6 = dat_f5f8;
  else {
    v6 = dat_f5f0;
  }
  v1 = strlen(dat_f5e8);
  v2 = strlen(v6);
  if (0x1000 <= v1 + v2) { // branch-flip
    write(2,dat_f5e8,v1);
    write(2,": ",2);
    write(2,v6,v2);
    v5[0] = 10;
    v4 = (char *)0x1;
  }
  else {
    v3 = __mempcpy_chk(v5,dat_f5e8,v1,(unsigned long)0x1002);
    *(unsigned short *)v3 = s_ad6d._0_2_;
    v4 = mempcpy(&v3[2],v6,v2);
    *v4 = 10;
    v4 = &v4[1 - (long)v5];
  }
  write(2,v5,(unsigned long)v4);
  if (!a0)
    _exit(dat_f020); // no-return
  raise(a0);
  abort(); // no-return
}

// Function: sub_5850 @ 0x5850
unsigned long sub_5850(unsigned long a0,int a1)
{
  if (!a1)
    return 0;
  sub_5730(0xb); // no-return
}

// Function: sub_5870 @ 0x5870
void sub_5870(unsigned long a0)
{
  if ((int)a0)
    a0 = (unsigned long)(-(unsigned int)(dat_f5e0 == 0) & 0xb);
  sub_5730(a0); // no-return
}

// Function: sub_58a0 @ 0x58a0
unsigned long sub_58a0(void *a0) // return-dupe
{
  if (!a0)
    a0 = sub_5720;
  dat_f600 = a0;
  dat_f5f8 = dcgettext(NULL,"program error",5);
  dat_f5f0 = dcgettext(NULL,"stack overflow",5);
  dat_f5e8 = sub_5950();
  if (!sub_63e0(sub_5870,0xf620,0x10000)) {
    dat_f5e0 = sub_6360(sub_5850);
    return 0;
  }
  *__errno_location() = 0x5f;
  return 0xffffffff;
}

// Function: sub_5950 @ 0x5950
unsigned long sub_5950(void)
{
  return __progname;
}

// Function: sub_5960 @ 0x5960
void sub_5960(char *a0)
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
    dat_1f620 = a0;
    program_invocation_name = a0;
    return;
  }
  fputs("A NULL argv[0] was passed through an exec system call.\n",stderr);
  abort(); // no-return
}

// Function: sub_5a00 @ 0x5a00
unsigned long sub_5a00(char *a0,unsigned long a1)
{
  char v1;
  char v10; // stack - 0xac
  char *v11; // stack - 0xa8
  long v12; // stack - 0xa0
  char v13; // stack - 0x98
  int v14; // stack - 0x94
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
  
  v3 = (char *)sub_6e40(a1,2);
  v1 = *a0;
  while ((v1 && (v5 = (char *)sub_8550(a0,v3), v5))) {
    if (2 <= __ctype_get_mb_cur_max()) { // branch-flip
      v6[0] = 0;
      v9 = 0;
      v11 = a0;
      if (v5 <= a0) { // branch-flip
label_5b29:
        v8 = (unsigned char *)0x1;
      }
      else {
        do {
          v10 = 0;
          sub_8bf0(v6);
          if ((v13) && (!v14)) {
            abort(); // no-return, return-dupe
          }
          v11 = &v11[v12];
          v10 = 0;
        } while (v11 < v5);
        if (!v13) goto label_5b29;
        v8 = (unsigned char *)CONCAT71((undefined7)((unsigned long)v8 >> 8),iswalnum() == 0);
      }
      v11 = v5;
      v6[0] = 0;
      v9 = 0;
      v17 = v3;
      v7[0] = 0;
      v15 = 0;
      while( true ) {
        v10 = 0;
        v16 = 0;
        sub_8bf0(v7);
        if ((v19) && (!v20)) break;
        sub_8bf0(v6);
        if ((v13) && (!v14))
          abort();
        v11 = &v11[v12];
        v17 = &v17[v18];
      }
      sub_8bf0(v6);
      if ((((v13) && (!v14)) || ((!v13 || (!iswalnum())))) && ((char)v8)) {
        free(v3); // return-dupe
        return (unsigned long)v8 & 0xffffffff;
      }
      v11 = v5;
      v6[0] = 0;
      v9 = 0;
      v10 = 0;
      sub_8bf0(v6);
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

// Function: sub_5c70 @ 0x5c70
char * sub_5c70(char *a0)
{
  char *v1; // rax
  unsigned long v2; // rax
  char *v3; // rax
  
  v1 = dcgettext(NULL,a0,5);
  if (a0 != v1) {
    if (!sub_5a00(v1,a0)) {
      v2 = strlen(v1);
      v3 = (char *)sub_7800(v2 + 4 + strlen(a0));
      __sprintf_chk(v3,1,0xffffffffffffffff,"%s (%s)",v1,a0);
      return v3;
    }
  }
  return v1;
}

// Function: sub_5d00 @ 0x5d00
char * sub_5d00(char *a0,char *a1)
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
  v4 = (char *)sub_7d90();
  v6 = v3;
  if (sub_7cc0(v4,"UTF-8")) { // branch-flip
    v7 = (char *)sub_7c90(a1,"UTF-8",v4);
    v5 = strlen(v4);
    v8 = memcpy((void *)sub_7800(v5 + 0xb),v4,v5);
    v9 = (unsigned long *)(v5 + (long)v8);
    *v9 = 0x4c534e4152542f2f;
    *(unsigned short *)&v9[1] = 0x5449;
    *(char *)((long)v9 + 10) = 0;
    a1 = (char *)sub_7c90(a1,"UTF-8",v8);
    free(v8);
    v11 = v7;
    if (a1) { // branch-flip
      if (strchr(a1,0x3f)) {
        free(a1);
        goto label_5f03;
      }
      v4 = a1;
      if (!v7) {
        v10 = a1;
        goto label_5d6a;
      }
      if (!strcmp(v3,a0)) {
        if (v7 == a1)
          return v7;
        free(a1);
        return v7;
      }
    }
    else {
label_5f03:
      if (!v7) {
        v11 = NULL;
        a1 = NULL;
        v10 = a0;
        v4 = NULL;
        goto label_5d6a;
      }
      if (!strcmp(v3,a0))
        return v7;
      a1 = NULL;
      v4 = NULL;
    }
    if (!sub_5a00(v3,a0)) {
      a0 = v7;
label_5d8f:
      if (((!v11) || (!sub_5a00(v3,v11))) && ((!a1 || (!sub_5a00(v3,a1))))) goto label_5e0c;
      if (!v7) goto label_5dbd;
    }
    free(v7);
  }
  else {
    v2 = strcmp(v3,a0);
    if (a1) {
      v10 = a1;
      v4 = NULL;
      v11 = a1;
label_5d6a:
      if (!strcmp(v3,a0))
        return v10;
      v1 = sub_5a00(v3,a0);
      v7 = NULL;
      a0 = v10;
      if (v1) goto label_5dbd;
      goto label_5d8f;
    }
    if (!v2)
      return a0;
    if (sub_5a00(v3,a0))
      return v3;
    v4 = NULL;
    v7 = NULL;
label_5e0c:
    v5 = strlen(v3);
    v6 = (char *)sub_7800(v5 + 4 + strlen(a0));
    __sprintf_chk(v6,1,0xffffffffffffffff,"%s (%s)",v3,a0);
    if (v7)
      free(v7);
  }
label_5dbd:
  if (v4)
    free(v4);
  return v6;
}

// Function: sub_6000 @ 0x6000
void sub_6000(unsigned long a0) // return-dupe
{
  char v1 [8];
  long v2; // stack - 0x30
  
  if ((int)sub_6da0(a0,v1) <= -1)
    return;
  dat_1f648 = v2 + -1;
}

// Function: sub_6050 @ 0x6050
void sub_6050(unsigned long a0,long a1,long a2)
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
  if (dat_1f628) { // branch-flip
    v4 = (*dat_1f628)(v1,0);
    if (v4)
      return;
    if (dat_1f640) goto label_609e;
  }
  else {
    if (!dat_1f640) {
      signal(0xb,0); // return-dupe, tail-call
      return;
    }
label_609e:
    v3 = dat_1f648;
    v2 = *(unsigned long *)(a2 + 0xa0);
    if (dat_1f648) {
      v6 = __errno_location();
      v4 = *v6;
      v5 = sub_6da0(v3,&v7);
      *v6 = v4;
      if (0 <= v5) {
        if (v7 <= v1) { // branch-flip
          if (v1 <= (unsigned long)(v8 - 1U)) goto label_6185;
        }
        else if ((*v9)(v1,&v7)) {
label_6185:
          (*dat_1f640)(dat_1f638 <= v2 && v2 <= dat_1f638 + dat_1f630,a2);
        }
      }
    }
  }
  if ((dat_1f628) && (v4 = (*dat_1f628)(v1,1), v4))
    return;
  signal(0xb,0);
  return;
}

// Function: sub_61e0 @ 0x61e0
void sub_61e0(void)
{
  void *v1; // stack - 0xa8
  char v2 [128];
  int v3; // stack - 0x20
  
  v1 = sub_6050;
  sigemptyset(v2);
  sigaddset(v2,1);
  sigaddset(v2,2);
  sigaddset(v2,3);
  sigaddset(v2,0xd);
  sigaddset(v2,0xe);
  sigaddset(v2,0xf);
  sigaddset(v2,10);
  sigaddset(v2,0xc);
  sigaddset(v2,0x11);
  sigaddset(v2,0x11);
  sigaddset(v2,0x17);
  sigaddset(v2,0x1d);
  sigaddset(v2,0x1d);
  sigaddset(v2,0x18);
  sigaddset(v2,0x19);
  sigaddset(v2,0x1a);
  sigaddset(v2,0x1b);
  sigaddset(v2,0x1e);
  sigaddset(v2,0x1c);
  v3 = (-(unsigned int)(dat_1f640 == 0) & 0xf8000000) + 0x8000004;
  sigaction(0xb,&v1,NULL);
}

// Function: sub_6360 @ 0x6360
unsigned long sub_6360(unsigned long a0)
{
  dat_1f628 = a0;
  sub_61e0();
  return 0;
}

// Function: sub_6380 @ 0x6380
void sub_6380(void)
{
  dat_1f628 = 0;
  if (dat_1f640)
    return;
  signal(0xb,0); // tail-call
}

// Function: sub_63b0 @ 0x63b0
unsigned long sub_63b0(void *a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  (*a0)(a1,a2,a3);
  return 1;
}

// Function: sub_63e0 @ 0x63e0
unsigned long sub_63e0(unsigned long a0,unsigned long a1,unsigned long a2) // return-dupe
{
  unsigned long v1; // stack - 0x48
  unsigned int v2; // stack - 0x40
  unsigned long v3; // stack - 0x38
  
  if ((!dat_1f648) && (sub_6000(&v1), !dat_1f648))
    return 0xffffffff;
  v2 = 0;
  dat_1f630 = a2;
  dat_1f638 = a1;
  dat_1f640 = a0;
  v1 = a1;
  v3 = a2;
  if ((int)sigaltstack(&v1,0) <= -1)
    return 0xffffffff;
  sub_61e0();
  return 0;
}

// Function: sub_6490 @ 0x6490
void sub_6490(void) // return-dupe
{
  char v1 [8];
  unsigned int v2; // stack - 0x20
  
  dat_1f640 = 0;
  if (dat_1f628) // branch-flip
    sub_61e0();
  else {
    signal(0xb,0);
  }
  v2 = 2;
  if (0 <= (int)sigaltstack(v1,0))
    return;
  perror("gnulib sigsegv (stackoverflow_deinstall_handler)");
}

// Function: sub_6520 @ 0x6520
bool sub_6520(long a0,long *a1)
{
  return (unsigned long)(*a1 - a0) <= (unsigned long)(*a1 - a1[3]) >> 1;
}

// Function: sub_6540 @ 0x6540
unsigned long sub_6540(long *a0)
{
  if (*a0 != a0[1])
    return (unsigned long)*(unsigned char *)(a0[3] + *a0);
  *(unsigned int *)&a0[2] = 1;
  return 0xffffffff;
}

// Function: sub_6560 @ 0x6560
void sub_6560(long *a0) // return-dupe
{
  if ((int)sub_6540() <= -1)
    return;
  *a0 = *a0 + 1;
}

// Function: sub_6570 @ 0x6570
unsigned long sub_6570(unsigned long a0,long *a1)
{
  int v1; // eax
  long v2; // rcx
  int v3; // esi
  
  v2 = 0;
  v3 = 0;
  do {
    v1 = sub_6540();
    if (10 <= (unsigned int)(v1 - 0x30U)) { // branch-flip
      if (6 <= (unsigned int)(v1 - 0x41U)) { // branch-flip
        if (6 <= (unsigned int)(v1 - 0x61U)) {
          if (v3) {
            *a1 = v2;
            return 0;
          }
          return 0xffffffff;
        }
        v2 = v2 * 0x10 + (long)(v1 + -0x57);
      }
      else {
        v2 = v2 * 0x10 + (long)(v1 + -0x37);
      }
    }
    else {
      v2 = v2 * 0x10 + (long)(v1 - 0x30U);
    }
    sub_6560();
    v3 += 1;
  } while( true );
}

// Function: sub_65e0 @ 0x65e0
unsigned long sub_65e0(unsigned long *a0,unsigned long a1,unsigned long a2)
{
  unsigned long *v1;
  unsigned long v2;
  
  if ((a1 <= *a0) && (*a0 <= a2 - 1)) {
    v1 = (unsigned long *)a0[1];
    v1[1] = a2;
    v2 = a0[2];
    *v1 = a1;
    v1[3] = v2;
    *(unsigned int *)&a0[3] = 0;
    return 1;
  }
  a0[2] = a2;
  return 0;
}

// Function: sub_6620 @ 0x6620
void sub_6620(void)
{
  dat_1f650 = (long)getpagesize();
}

// Function: sub_6640 @ 0x6640
void sub_6640(long a0)
{
  if (*(void **)(a0 + 0x20)) {
    munmap(*(void **)(a0 + 0x20),*(unsigned long *)(a0 + 0x28)); // tail-call
    return;
  }
}

// Function: sub_6660 @ 0x6660
unsigned int sub_6660(unsigned long a0)
{
  unsigned int v1; // eax
  long v2;
  
  v1 = mincore(a0,dat_1f650,&v2);
  return ~v1 >> 0x1f;
}

// Function: sub_66b0 @ 0x66b0
unsigned long sub_66b0(unsigned long a0,unsigned long a1) // return-dupe
{
  unsigned long v1;
  long v2; // rax
  unsigned long v3; // rax
  unsigned long v4;
  long v5;
  unsigned long v6; // r14
  unsigned long v7;
  
  v1 = dat_1f650;
  v2 = (a0 / dat_1f650) * dat_1f650;
  v3 = ((a1 / dat_1f650 + 1) * dat_1f650 - v2) / dat_1f650;
  if (v3 <= 1)
    return 1;
  v4 = 1;
  do {
    v4 *= 2;
  } while (v4 < v3);
  do {
    v6 = v4 >> 1;
    v5 = v2 + v1 * v6;
    if (v6 < v3) {
      v7 = v6;
      do {
        if (sub_6660(v5))
          return 0;
        v7 += v4 & 0xfffffffffffffffe;
        v5 += v1 * v6 * 2;
      } while (v7 < v3);
    }
    v4 = v6;
  } while (2 <= v6);
  return 1;
}

// Function: sub_67a0 @ 0x67a0
unsigned long sub_67a0(unsigned long a0,long *a1)
{
  long v1;
  unsigned long v2; // rdi
  
  v1 = *a1;
  v2 = a0 * 2 - v1;
  if (a0 < v2)
    return 0;
  return sub_66b0(v2,v1 + -1); // tail-call
}

// Function: sub_67d0 @ 0x67d0
unsigned long sub_67d0(unsigned long a0)
{
  unsigned long v1;
  unsigned long v2;
  char v3 [1032];
  long v4; // rsi
  unsigned long v5; // r14
  unsigned long v6; // r15
  
  v2 = 0x400;
  while( true ) {
    v1 = dat_1f650;
    if (!a0)
      return 0;
    if (a0 / dat_1f650 < v2)
      v2 = a0 / dat_1f650;
    v4 = dat_1f650 * v2;
    v5 = a0 - v4;
    if ((int)mincore(v5,v4,v3) < 0) break;
    a0 = v5;
  }
  do {
    if (v2 == 1)
      return a0;
    while( true ) {
      v5 = v2 + 1 >> 1;
      v4 = v1 * v5;
      v6 = a0 - v4;
      if ((int)mincore(v6,v4,v3) < 0) break;
      v2 >>= 1;
      a0 = v6;
      if (v2 == 1)
        return v6;
    }
    v2 = v5;
  } while( true );
}

// Function: sub_68b0 @ 0x68b0
long sub_68b0(long a0) // return-dupe
{
  unsigned long v1;
  unsigned long v2;
  unsigned long v3;
  char v4 [1032];
  
  v1 = dat_1f650;
  a0 += dat_1f650;
  if (!a0)
    return 0;
  v2 = 0x400;
  while( true ) {
    if ((unsigned long)-a0 / v1 < v2)
      v2 = (unsigned long)-a0 / v1;
    if ((int)mincore(a0,v1 * v2,v4) <= -1) break;
    a0 += v1 * v2;
    if (!a0)
      return 0;
  }
  do {
    v3 = v2;
    if (v2 == 1)
      return a0;
    while( true ) {
      v2 = v3 + 1 >> 1;
      if ((int)mincore(a0,v1 * v2,v4) < 0) break;
      v3 >>= 1;
      a0 += v1 * v2;
      if (v3 == 1)
        return a0;
    }
  } while( true );
}

// Function: sub_69b0 @ 0x69b0
unsigned long sub_69b0(unsigned long a0,unsigned long *a1)
{
  long v1; // rbp
  
  if (!dat_1f650)
    sub_6620();
  v1 = (a0 / dat_1f650) * dat_1f650;
  *a1 = sub_67d0(v1);
  a1[1] = sub_68b0(v1);
  a1[2] = sub_67a0;
  return 0;
}

// Function: sub_6a20 @ 0x6a20
unsigned long sub_6a20(unsigned long *a0) // return-dupe x2
{
  void *v1;
  int v2; // eax
  int v3; // eax
  long v4;
  unsigned long v5; // rax
  char *v6; // r12
  char *v7;
  
  v2 = open("/proc/self/maps",0);
  if (v2 <= -1)
    return 0xffffffff;
  *a0 = 0;
  v7 = (char *)0x1;
  *(unsigned int *)&a0[2] = 0;
  v6 = NULL;
  a0[3] = &a0[8];
  a0[4] = 0;
  a0[6] = 0;
  a0[7] = 0;
  do {
    if (0x104a <= v7) { // branch-flip
      v1 = (void *)a0[3];
      v3 = (int)read(v2,v1,(unsigned long)v7);
      if (0 <= v3) {
        if (!v3) goto label_6bec;
        if (v7 < (char *)(long)(v3 + 0x1049)) goto label_6abf;
        v4 = (long)v3;
        while( true ) {
          a0[1] = v4;
          v5 = read(v2,(void *)(v4 + a0[3]),(long)v7 - v4);
          v3 = (int)v5;
          if (v3 <= -1) break;
          if ((unsigned long)((long)v7 - a0[1]) < (unsigned long)(long)(v3 + 0x1049)) goto label_6abf;
          if (!v3) {
            close(v2);
            return v5 & 0xffffffff;
          }
          v4 = (long)v3 + a0[1];
        }
      }
      if (*__errno_location() != 4) {
label_6bec:
        close(v2);
        v1 = (void *)a0[4];
        goto label_6bfa;
      }
    }
    else {
label_6abf:
      if (v6) { // branch-flip
        v7 = (char *)((long)v7 * 2);
        if (!v7) goto label_6bec;
        if ((void *)a0[4])
          munmap((void *)a0[4],a0[5]);
      }
      else {
        v6 = (char *)(long)getpagesize();
        for (v7 = v6; v7 <= "pen"; v7 = (char *)((long)v7 * 2)) {
        }
      }
      v4 = mmap(0,v7,3,0x22,0xffffffff,0);
      a0[4] = v4;
      if (v4 == -1) {
        close(v2);
        return 0xffffffff;
      }
      a0[5] = v7;
      a0[6] = v4;
      a0[7] = &v7[v4];
      a0[3] = v4;
    }
    if ((long)lseek(v2,0,0) <= -1) {
      close(v2);
      v2 = open("/proc/self/maps",0);
      if (v2 <= -1) {
        v1 = (void *)a0[4];
label_6bfa:
        if (!v1)
          return 0xffffffff;
        munmap(v1,a0[5]);
        return 0xffffffff;
      }
    }
  } while( true );
}

// Function: sub_6c60 @ 0x6c60
unsigned long sub_6c60(unsigned long a0) // return-dupe
{
  unsigned long v1;
  unsigned long v10; // stack - 0x48
  long v11; // stack - 0x40
  int v2; // eax
  unsigned long v3; // stack - 0x88
  char v4 [48];
  long v5; // stack - 0x80
  long v6;
  unsigned long v7;
  unsigned long *v8;
  long *v9;
  
  v8 = &v3;
  if ((int)sub_6a20(v4) <= -1)
    return 0xffffffff;
  v9 = &v5;
  v7 = v10;
label_6cb0:
  do {
    v2 = sub_6570(v4,v8);
    if (((v2 < 0) || (sub_6560() != 0x2d)) || (v2 = sub_6570(v9), v2 < 0)) {
      sub_6640(v4); // return-dupe
      return 0;
    }
    do {
      v2 = sub_6560(v4);
      if (v2 == -1) break;
    } while (v2 != 10);
    if (v3 <= v7) { // branch-flip
      v1 = v5 - 1;
      if (v1 < (unsigned long)(v11 - 1U)) goto label_6d50;
      v6 = v5;
      if ((v3 < v7) && (v2 = sub_65e0(a0,v3,v7), v2)) {
        sub_6640(v4);
        return 0;
      }
      if (v1 <= (unsigned long)(v11 - 1U)) goto label_6cb0;
      v2 = sub_65e0(a0,v11,v6);
    }
    else {
label_6d50:
      v2 = sub_65e0(a0,v3,v5);
    }
    if (v2) {
      sub_6640(v4);
      return 0;
    }
  } while( true );
}

// Function: sub_6d90 @ 0x6d90
void sub_6d90(void)
{
  sub_6c60(); // tail-call
}

// Function: sub_6da0 @ 0x6da0
unsigned long sub_6da0(unsigned long a0,long a1)
{
  unsigned long v1; // stack - 0x48
  long v2; // stack - 0x40
  unsigned long v3; // stack - 0x38
  int v4; // stack - 0x30
  
  v3 = 0;
  v4 = -1;
  v1 = a0;
  v2 = a1;
  sub_6d90(&v1);
  if (!v4) {
    *(void **)(a1 + 0x10) = sub_6520;
    return 0;
  }
  return sub_69b0(a0,a1); // tail-call
}

// Function: sub_6e40 @ 0x6e40
unsigned char * sub_6e40(char *a0,int a1)
{
  unsigned char v1;
  unsigned long v10; // stack - 0x6c
  char v11; // stack - 0x64
  unsigned char *v12; // stack - 0x60
  long v13; // stack - 0x58
  char v14; // stack - 0x50
  unsigned int v15; // stack - 0x4c
  long v2;
  int v3; // eax
  unsigned char *v4; // rax
  unsigned char *v5;
  unsigned char *v6; // stack - 0x78
  int v7; // eax
  unsigned char *v8;
  char v9; // stack - 0x70
  
  v4 = (unsigned char *)strdup(a0);
  if (!v4)
    sub_7bb0(); // no-return
  if (2 <= __ctype_get_mb_cur_max()) { // branch-flip
    if (a1) {
      v12 = v4;
      v9 = 0;
      v6 = &v4[strlen((char *)v4)];
      v11 = 0;
      v10 = 0;
      v5 = v4;
      if (v4 < v6) {
        do {
          v11 = 0;
          sub_7dd0(&v6);
          if ((!v14) || (v7 = iswspace(v15), !v7)) {
            v5 = v12;
            break;
          }
          v12 = &v12[v13];
          v11 = 0;
          v5 = v12;
        } while (v12 < v6);
      }
      memmove(v4,v5,strlen((char *)v5) + 1);
      if (a1 == 1)
        return v4;
    }
    v12 = v4;
    v7 = 0;
    v9 = 0;
    v6 = &v4[strlen((char *)v4)];
    v10 = 0;
    if (v4 < v6) {
      do {
        v11 = 0;
        sub_7dd0(&v6);
        if (v7) { // branch-flip
          if (v7 != 1) { // branch-flip
            if ((v7 != 2) || (!v14)) goto label_6ecb;
            v3 = iswspace(v15);
            if (!v3)
              v7 = 1;
          }
          else if ((((v14) && (v3 = iswspace(v15), v3)) && (v14)) && (v3 = iswspace(v15), v3)) {
            v7 = 2;
            v8 = v12;
          }
          else {
label_700f:
          }
        }
        else {
          if (v14) {
            v3 = iswspace(v15);
            if (v3) goto label_700f;
            if (v14) {
              v7 = 1;
              iswspace(v15);
              goto label_6ed5;
            }
          }
label_6ecb:
          v7 = 1;
        }
label_6ed5:
        v12 = &v12[v13];
      } while (v12 < v6);
      if (v7 == 2)
        *v8 = 0;
    }
  }
  else {
    if (a1) {
      v1 = *v4;
      v5 = v4;
      if (v1) {
        do {
          if (!(*(unsigned char *)(*(long *)__ctype_b_loc() + 1 + (unsigned long)v1 * 2) & 0x20)) break;
          v1 = v5[1];
          v5 = &v5[1];
        } while (v1);
      }
      memmove(v4,v5,strlen((char *)v5) + 1);
      if (a1 == 1)
        return v4;
    }
    v5 = &v4[strlen((char *)v4) - 1];
    if (v4 <= v5) {
      v2 = *(long *)__ctype_b_loc();
      do {
        if (!(*(unsigned char *)(v2 + 1 + (unsigned long)*v5 * 2) & 0x20))
          return v4;
        *v5 = 0;
        v5 = &v5[-1];
      } while (v4 <= v5);
    }
  }
  return v4;
}

// Function: sub_7150 @ 0x7150
long sub_7150(void *a0,long a1,unsigned long a2,unsigned long a3,unsigned long *a4,unsigned long a5)
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
label_727d:
    v1 = CONCAT44(dat_4,__fprintf_chk(a0,1,v2,v4,v3,v5,v6,v7,v8,v9,v10));
label_72b6:
    return v1;
  }
  v1 = (long)*(int *)(a5 * 4 + 0xb130) + 0xb130;
  switch(a5) {
    case 0:
      goto label_72b6;
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
      goto label_727d;
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

// Function: sub_75d0 @ 0x75d0
void sub_75d0(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,long *a4)
{
  long v1;
  long v2;
  
  v2 = 0;
  v1 = *a4;
  while (v1) {
    v2 += 1;
    v1 = a4[v2];
  }
  sub_7150(); // tail-call
}

// Function: sub_75f0 @ 0x75f0
void sub_75f0(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned int *a4)
{
  unsigned int v1;
  long *v2;
  long v3;
  long v4 [11];
  long v5; // r9
  
  v5 = 0;
  do {
    v1 = *a4;
    if (0x30 <= v1) { // branch-flip
      v2 = *(long **)&a4[2];
      *(long **)&a4[2] = &v2[1];
      v3 = *v2;
      v4[v5] = v3;
    }
    else {
      *a4 = v1 + 8;
      v3 = *(long *)((unsigned long)v1 + *(long *)&a4[4]);
      v4[v5] = v3;
    }
  } while ((v3) && (v5 = v5 + 1, v5 != 10));
  sub_7150(a0,a1,a2,a3);
}

// Function: sub_7690 @ 0x7690
void sub_7690(void)
{
  sub_75f0();
}

// Function: sub_7740 @ 0x7740
void sub_7740(void)
{
  fputs_unlocked("\n",stdout);
  __printf_chk(1,dcgettext(NULL,"Report bugs to: %s\n",5),"bug-diffutils@gnu.org");
  __printf_chk(1,dcgettext(NULL,"%s home page: <%s>\n",5),"GNU diffutils","https://www.gnu.org/software/diffutils/");
  __printf_chk(1,dcgettext(NULL,"General help using GNU software: <%s>\n",5),"https://www.gnu.org/gethelp/"); // tail-call
}

// Function: sub_77f0 @ 0x77f0
long sub_77f0(long a0)
{
  if (a0)
    return a0;
  sub_7bb0(); // no-return
}

// Function: sub_7800 @ 0x7800
void sub_7800(unsigned long a0)
{
  sub_77f0(malloc(a0)); // tail-call
}

// Function: sub_7820 @ 0x7820
void sub_7820(void)
{
  sub_7800(); // tail-call
}

// Function: sub_7830 @ 0x7830
void sub_7830(void)
{
  sub_77f0(sub_7d20()); // tail-call
}

// Function: sub_7850 @ 0x7850
void sub_7850(void *a0,unsigned long a1)
{
  if ((!realloc(a0,a1)) && ((!a0 || (a1))))
    sub_7bb0(); // no-return
}

// Function: sub_7890 @ 0x7890
void sub_7890(void)
{
  sub_77f0(sub_7d30()); // tail-call
}

// Function: sub_78b0 @ 0x78b0
void sub_78b0(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!reallocarray(a0,a1,a2)) && ((!a0 || ((a1 && (a2))))))
    sub_7bb0(); // no-return
}

// Function: sub_78f0 @ 0x78f0
void sub_78f0(unsigned long a0,unsigned long a1)
{
  sub_78b0(0,a0,a1); // tail-call
}

// Function: sub_7900 @ 0x7900
void sub_7900(void)
{
  sub_78b0(); // tail-call
}

// Function: sub_7910 @ 0x7910
void sub_7910(void)
{
  sub_77f0(sub_7d60()); // tail-call
}

// Function: sub_7930 @ 0x7930
void sub_7930(long a0,unsigned long *a1,unsigned long a2)
{
  char v1 [16];
  unsigned long v2; // rax
  unsigned long v3;
  bool v4; // cf
  
  v3 = *a1;
  if (a0) { // branch-flip
    v2 = (v3 >> 1) + 1;
    v4 = CARRY8(v3,v2);
    v3 += v2;
    if (v4)
      sub_7bb0(); // no-return
  }
  else if (!v3) {
    v1._8_8_ = 0;
    v1._0_8_ = a2;
    v3 = SUB168((ZEXT816(0) << 0x40 | ZEXT816(0x80)) / v1._0_16_,0) + (unsigned long)(0x81 <= a2);
  }
  sub_78b0(a0,v3,a2);
  *a1 = v3;
}

// Function: sub_79a0 @ 0x79a0
void sub_79a0(unsigned long a0,unsigned long a1)
{
  sub_7930(a0,a1,1); // tail-call
}

// Function: sub_79b0 @ 0x79b0
void sub_79b0(long a0,long *a1,long a2,long a3,long a4)
{
  long v1;
  long v2;
  long v3;
  
  v1 = *a1;
  v2 = (v1 >> 1) + v1;
  if (SCARRY8(v1 >> 1,v1))
    v2 = 0x7fffffffffffffff;
  v3 = a3;
  if (v2 <= a3)
    v3 = v2;
  if (0 <= a3)
    v2 = v3;
  v3 = v2 * a4;
  if (SEXT816(v3) != SEXT816(v2) * SEXT816(a4)) // branch-flip
    v3 = 0x7fffffffffffffff;
  else {
    if (0x80 <= v3) goto label_7a56;
    v3 = 0x80;
  }
  v2 = v3 / a4;
  v3 -= v3 % a4;
label_7a56:
  if (!a0)
    *a1 = 0;
  if ((v2 - v1 < a2) && ((v2 = v1 + a2, SCARRY8(v1,a2) || (((a3 < v2 && (0 <= a3)) || (v3 = v2 * a4, SEXT816(v3) != SEXT816(v2) * SEXT816(a4)))))))
    sub_7bb0(a0,v3); // no-return
  sub_7850(a0,v3);
  *a1 = v2;
}

// Function: sub_7a90 @ 0x7a90
void sub_7a90(unsigned long a0,unsigned long a1)
{
  sub_77f0(calloc(a0,a1)); // tail-call
}

// Function: sub_7ab0 @ 0x7ab0
void sub_7ab0(unsigned long a0)
{
  sub_7a90(a0,1); // tail-call
}

// Function: sub_7ac0 @ 0x7ac0
void sub_7ac0(void)
{
  sub_77f0(sub_7d50()); // tail-call
}

// Function: sub_7ae0 @ 0x7ae0
void sub_7ae0(unsigned long a0)
{
  sub_7ac0(a0,1); // tail-call
}

// Function: sub_7af0 @ 0x7af0
void sub_7af0(void *a0,unsigned long a1)
{
  memcpy((void *)sub_7800(a1),a0,a1); // tail-call
}

// Function: sub_7b20 @ 0x7b20
void sub_7b20(void *a0,unsigned long a1)
{
  memcpy((void *)sub_7830(a1),a0,a1); // tail-call
}

// Function: sub_7b50 @ 0x7b50
void sub_7b50(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)sub_7830(a1 + 1);
  *(char *)((long)v1 + a1) = 0;
  memcpy(v1,a0,a1); // tail-call
}

// Function: sub_7b90 @ 0x7b90
void sub_7b90(char *a0)
{
  sub_7af0(a0,strlen(a0) + 1); // tail-call
}

// Function: sub_7bb0 @ 0x7bb0
void sub_7bb0(void)
{
  error(dat_f020,0,"%s",dcgettext(NULL,"memory exhausted",5));
  abort(); // no-return
}

// Function: sub_7bf0 @ 0x7bf0
void sub_7bf0(void)
{
  int v1; // eax
  
  v1 = sub_8dc0();
  if (!v1)
    return;
  error(dat_f020,v1,dcgettext(NULL,"standard file descriptors",5)); // tail-call
}

// Function: sub_7c30 @ 0x7c30
int sub_7c30(void)
{
  int v1; // eax
  
  v1 = sub_8e70();
  if (v1 < 0) {
    if (*__errno_location() == 0xc)
      sub_7bb0(); // no-return
  }
  return v1;
}

// Function: sub_7c60 @ 0x7c60
long sub_7c60(void)
{
  long v1; // rax
  
  v1 = sub_90f0();
  if (!v1) {
    if (*__errno_location() == 0xc)
      sub_7bb0(); // no-return
  }
  return v1;
}

// Function: sub_7c90 @ 0x7c90
long sub_7c90(void)
{
  long v1; // rax
  
  v1 = sub_9340();
  if (!v1) {
    if (*__errno_location() == 0xc)
      sub_7bb0(); // no-return
  }
  return v1;
}

// Function: sub_7cc0 @ 0x7cc0
int sub_7cc0(char *a0,char *a1) // early-return
{
  unsigned char v1; // al
  unsigned int v2; // eax
  
  if (a0 == a1)
    return 0;
  do {
    v2 = sub_98a0(*a0);
    v1 = sub_98a0(*a1);
    if (!(unsigned char)v2) break;
    a0 = &a0[1];
    a1 = &a1[1];
  } while ((unsigned char)v2 == v1);
  return (v2 & 0xff) - (unsigned int)v1;
}

// Function: sub_7d20 @ 0x7d20
void sub_7d20(unsigned long a0)
{
  malloc(a0); // tail-call
}

// Function: sub_7d30 @ 0x7d30
void sub_7d30(void *a0,unsigned long a1)
{
  realloc(a0,a1 | a1 == 0); // tail-call
}

// Function: sub_7d50 @ 0x7d50
void sub_7d50(unsigned long a0,unsigned long a1)
{
  calloc(a0,a1); // tail-call
}

// Function: sub_7d60 @ 0x7d60
void sub_7d60(void *a0,unsigned long a1,unsigned long a2)
{
  if ((a1) && (a2)) {
    reallocarray(a0,a1,a2); // tail-call
    return;
  }
  reallocarray(a0,1,1); // tail-call
}

// Function: sub_7d90 @ 0x7d90
char * sub_7d90(void) // early-return
{
  char *v1; // rax
  
  v1 = nl_langinfo(0xe);
  if (!v1)
    return "ASCII";
  if (*v1)
    return v1;
  return "ASCII";
}

// Function: sub_7dd0 @ 0x7dd0
void sub_7dd0(long *a0)
{
  char v1;
  char *v2;
  long v3; // rax
  
  if (*(char *)((long)a0 + 0x14))
    return;
  v2 = (char *)a0[3];
  if (!(char)a0[1]) {
    v1 = *v2;
    if (sub_9a80((int)v1)) {
      a0[4] = 1;
      v1 = *v2;
      *(char *)&a0[5] = 1;
      *(int *)((long)a0 + 0x2c) = (int)v1;
      goto label_7e13;
    }
    if (!mbsinit((void *)((long)a0 + 0xc)))
      __assert_fail("mbsinit (&iter->state)","mbiter.h",0x87,"mbiter_multi_next"); // no-return
    *(char *)&a0[1] = 1;
  }
  v3 = sub_9680((long)a0 + 0x2c,v2,*a0 - (long)v2,(void *)((long)a0 + 0xc));
  a0[4] = v3;
  if (v3 == -1) {
    a0[4] = 1;
    *(char *)&a0[5] = 0;
    *(char *)((long)a0 + 0x14) = 1;
    return;
  }
  if (v3 != -2) { // branch-flip
    if (!v3) {
      a0[4] = 1;
      if (*(char *)a0[3])
        __assert_fail("*iter->cur.ptr == \'\\0\'","mbiter.h",0xa2,"mbiter_multi_next"); // no-return
      if (*(int *)((long)a0 + 0x2c))
        __assert_fail("iter->cur.wc == 0","mbiter.h",0xa3,"mbiter_multi_next"); // no-return
    }
    *(char *)&a0[5] = 1;
    if (mbsinit((void *)((long)a0 + 0xc)))
      *(char *)&a0[1] = 0;
  }
  else {
    *(char *)&a0[5] = 0;
    a0[4] = *a0 - a0[3];
  }
label_7e13:
  *(char *)((long)a0 + 0x14) = 1;
  return;
}

// Function: sub_7f40 @ 0x7f40
void sub_7f40(long *a0,long a1)
{
  a0[3] = a0[3] + a1;
  *a0 = *a0 + a1;
}

// Function: sub_7f50 @ 0x7f50
void sub_7f50(unsigned long *a0,unsigned long *a1)
{
  char v1;
  
  *a0 = *a1;
  v1 = *(char *)&a1[1];
  *(char *)&a0[1] = v1;
  if (v1) // branch-flip
    *(unsigned long *)((long)a0 + 0xc) = *(unsigned long *)((long)a1 + 0xc);
  else {
    *(unsigned long *)((long)a0 + 0xc) = 0;
  }
  *(char *)((long)a0 + 0x14) = *(char *)((long)a1 + 0x14);
  sub_9a20(&a0[3],&a1[3]); // tail-call
}

// Function: sub_7fa0 @ 0x7fa0
unsigned long sub_7fa0(char *a0,long a1,unsigned long a2,unsigned long *a3)
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
      v9 = sub_9970(v9);
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
              goto label_8098;
            }
            v3 -= *(long *)(v9 + v3 * 8);
            v2 = *(char *)(a1 + v3);
          }
          v3 += 1;
          *(unsigned long *)(v9 + v1 * 8) = v1 - v3;
label_8098:
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
      *(unsigned long *)&v8[-8] = 0x80ee;
      sub_99c0(v9);
      return 1;
    }
  }
  return 0;
}

// Function: sub_8140 @ 0x8140
char sub_8140(void *a0,void *a1,long *a2)
{
  char v1;
  void *v10; // stack - 0xf8
  void *v11; // stack - 0x78
  char v12 [4]; // stack - 0x88
  char v13 [4]; // stack - 0xc8
  void **v14;
  char *v15; // rsp
  void **v16; // rsp
  void **v17;
  unsigned long v18;
  long v19;
  char v2 [16];
  void *v20;
  unsigned long *v21;
  long *v22; // stack - 0xf0
  char v23; // stack - 0xe1
  void *v24; // stack - 0xe0
  void *v25; // stack - 0xd8
  char *v26; // stack - 0xd0
  unsigned long v27; // stack - 0xc4
  char v28; // stack - 0xbc
  void *v29; // stack - 0xb8
  char v3;
  long v30; // stack - 0xb0
  char v31; // stack - 0xa8
  int v32; // stack - 0xa4
  unsigned long v33; // stack - 0x84
  char v34; // stack - 0x7c
  unsigned long v35; // stack - 0x70
  char v36; // stack - 0x68
  int v37; // stack - 0x64
  void *v4;
  int v5; // eax
  unsigned long *v6; // rax
  unsigned long *v7;
  void *v8;
  unsigned long *v9; // rbx
  
  v16 = &v10;
  v14 = &v10;
  v17 = &v10;
  v10 = a0;
  v22 = a2;
  v24 = (void *)sub_9aa0(a1);
  v2._8_8_ = 0;
  v2._0_8_ = v24;
  if ((0 <= SUB168(ZEXT816(0x38) * v2._0_16_,0)) && (!SUB168(ZEXT816(0x38) * v2._0_16_,8))) {
    v18 = (long)v24 * 0x38;
    if (0xfa1 <= v18) { // branch-flip
      v21 = (unsigned long *)sub_9970(v18);
      v4 = v24;
    }
    else {
      v17 = &v10;
      while (v16 != (void **)((long)&v10 - (v18 + 0x36 & 0xfffffffffffff000))) {
        v14 = (char *)((long)v17 + -0x1000);
        v15 = (char *)((long)v17 + -0x1000);
        *(unsigned long *)((long)v17 + -8) = *(unsigned long *)((long)v17 + -8);
        v16 = (char *)((long)v17 + -0x1000);
        v17 = (void **)v15;
      }
      v18 = (unsigned long)((unsigned int)(v18 + 0x36) & 0xff0);
      v19 = -v18;
      v17 = (char *)((long)v14 + v19);
      if (v18)
        *(unsigned long *)((long)v14 + -8) = *(unsigned long *)((long)v14 + -8);
      v21 = (unsigned long *)(((unsigned long)((long)v14 + v19 + 0xf) & 0xfffffffffffffff0) + 0x1f & 0xffffffffffffffe0);
      v4 = v24;
    }
    if (v21) {
      v12[0] = 0;
      v26 = v12;
      v33 = 0;
      v7 = v21;
      v24 = v4;
      v11 = a1;
      while( true ) {
        v34 = 0;
        *(unsigned long *)((long)v17 + -8) = 0x82bf;
        sub_8bf0(v26);
        v23 = v36;
        if ((v36) && (!v37)) break;
        v9 = &v7[6];
        *(unsigned long *)((long)v17 + -8) = 0x82a7;
        sub_9a20(v7,&v11);
        v11 = (void *)((long)v11 + v35);
        v7 = v9;
      }
      v19 = 0;
      v21[(long)v4 * 6 + 1] = 1;
      v25 = (void *)0x2;
      v7 = v21;
      if ((void *)0x3 <= v24) {
label_8300:
        v9 = &v7[6];
        v1 = *(char *)&v7[8];
        do {
          v6 = &v21[v19 * 6];
          if ((v1) && (*(char *)&v6[2])) {
            if (*(int *)((long)v7 + 0x44) == *(int *)((long)v6 + 0x14)) {
label_8355:
              v19 += 1;
              v21[(long)((long)v4 * 6 + (long)v25)] = (long)v25 - v19;
              goto label_836a;
            }
          }
          else {
            v18 = v7[7];
            if (v18 == v6[1]) {
              v20 = (void *)*v6;
              v8 = (void *)*v9;
              *(unsigned long *)((long)v17 + -8) = 0x8351;
              v3 = *(char *)((long)v17 + -8);
              if (!memcmp(v8,v20,v18,v3)) goto label_8355;
            }
          }
          if (!v19) goto label_8530;
          v19 -= v21[(long)v4 * 6 + v19];
        } while( true );
      }
label_838a:
      v12[0] = 0;
      v20 = NULL;
      v13[0] = 0;
      *v22 = 0;
      v27 = 0;
      v29 = v10;
      v28 = 0;
      v11 = v10;
      v33 = 0;
      v34 = 0;
      do {
        while( true ) {
          *(unsigned long *)((long)v17 + -8) = 0x83ec;
          sub_8bf0(v26);
          v18 = v35;
          if (!v36) break;
          if (!v37) goto label_8227;
          v7 = &v21[(long)v20 * 6];
          if (!*(char *)&v7[2]) goto label_845b;
          if (*(int *)((long)v7 + 0x14) == v37) {
            v8 = v11;
            goto label_8417;
          }
label_84db:
          if (v20) { // branch-flip
            v19 = v21[(long)((long)v4 * 6 + (long)v20)];
            v20 = (void *)((long)v20 - v19);
            for (; v19; v19 = v19 + -1) {
              *(unsigned long *)((long)v17 + -8) = 0x8517;
              sub_8bf0(v13);
              if ((v31) && (!v32)) goto label_852a;
              v28 = 0;
              v29 = (void *)((long)v29 + v30);
            }
          }
          else {
            *(unsigned long *)((long)v17 + -8) = 0x8472;
            sub_8bf0(v13);
            if ((v31) && (!v32)) {
label_852a:
              *(unsigned long *)((long)v17 + -8) = 0x852f; // no-return
              abort(*(char *)((long)v17 + -8));
            }
            v28 = 0;
            v29 = (void *)((long)v29 + v30);
            v34 = 0;
            v11 = (void *)((long)v11 + v35);
          }
        }
        v7 = &v21[(long)v20 * 6];
label_845b:
        if (v7[1] != v35) goto label_84db;
        v8 = (void *)*v7;
        v25 = v11;
        *(unsigned long *)((long)v17 + -8) = 0x84c9;
        v5 = memcmp(v8,v11,v35,*(char *)((long)v17 + -8));
        v8 = v25;
        if (v5) goto label_84db;
label_8417:
        v11 = (void *)((long)v8 + v18);
        v34 = 0;
        v20 = (void *)((long)v20 + 1);
      } while (v24 != v20);
      *v22 = (long)v29;
label_8227:
      *(unsigned long *)((long)v17 + -8) = 0x8448;
      sub_99c0(v21);
      return v23;
    }
  }
  return '\0';
label_8530:
  v21[(long)((long)v4 * 6 + (long)v25)] = v25;
label_836a:
  v25 = (void *)((long)v25 + 1);
  v7 = v9;
  if (v24 == v25) goto label_838a;
  goto label_8300;
}

// Function: sub_8550 @ 0x8550
char * sub_8550(char *a0,char *a1) // early-return, return-dupe x4
{
  char *v1;
  unsigned long v10; // rdx
  char *v11; // stack - 0x190
  char v12 [4]; // stack - 0x188
  char v13 [4]; // stack - 0x88
  unsigned long v14; // stack - 0xc8
  unsigned int v15; // stack - 0x108
  char v16 [4]; // stack - 0x148
  long v17;
  unsigned long v18;
  unsigned long v19; // stack - 0x1d8
  char v2;
  unsigned char v20; // stack - 0x1c9
  unsigned long v21; // stack - 0x1a8
  unsigned long v22; // stack - 0x184
  char v23; // stack - 0x17c
  char *v24; // stack - 0x178
  unsigned long v25; // stack - 0x170
  char v26; // stack - 0x168
  int v27; // stack - 0x164
  unsigned long v28; // stack - 0x144
  char v29; // stack - 0x13c
  char v3;
  char *v30; // stack - 0x138
  long v31; // stack - 0x130
  unsigned char v32; // stack - 0x128
  int v33; // stack - 0x124
  unsigned int v34; // stack - 0x104
  unsigned int v35; // stack - 0x100
  unsigned int v36; // stack - 0xfc
  char *v37; // stack - 0xf8
  unsigned long v38; // stack - 0xf0
  unsigned int v39; // stack - 0xe8
  unsigned char v4; // al
  int v40; // stack - 0xe4
  unsigned long v41; // stack - 0xe0
  unsigned long v42; // stack - 0xd8
  unsigned long v43; // stack - 0xd0
  unsigned long v44; // stack - 0xc0
  undefined3 v45; // stack - 0xbb
  char *v46; // stack - 0xb8
  unsigned long v47; // stack - 0xb0
  unsigned long v48; // stack - 0xa8
  unsigned long v49; // stack - 0xa0
  unsigned int v5; // eax
  unsigned long v50; // stack - 0x98
  unsigned long v51; // stack - 0x90
  unsigned long v52; // stack - 0x84
  char v53; // stack - 0x7c
  char *v54; // stack - 0x78
  unsigned long v55; // stack - 0x70
  char v56; // stack - 0x68
  int v57; // stack - 0x64
  int v6; // eax
  char *v7;
  unsigned long v8;
  unsigned long v9;
  
  if (2 <= __ctype_get_mb_cur_max()) { // branch-flip
    v12[0] = 0;
    v22 = 0;
    v23 = 0;
    v24 = a1;
    sub_8bf0(v12);
    if ((!v26) || (v27)) {
      v20 = 1;
      v16[0] = 0;
      v28 = 0;
      v29 = 0;
      v15 &= 0xffffff00;
      v35 = 0;
      v34 = 0;
      v19 = 0;
      v21 = 0;
      v9 = 0;
      v30 = a1;
      v37 = a0;
      do {
        v36 &= 0xffffff00;
        sub_8bf0(&v15);
        if ((char)v39) { // branch-flip
          if (!v40)
            return NULL;
          v4 = 10 <= v19 & v20;
          if (v4) {
            if (v19 * 5 <= v9) { // branch-flip
label_8ad0:
              v17 = v9 - v21;
              while ((v17 && ((sub_8bf0(v16), !v32 || (v33))))) {
                v29 = 0;
                v30 = &v30[v31];
                v17 -= 1;
              }
              sub_8bf0(v16);
              v20 = v32;
              if (v32) { // branch-flip
                if (!v33) {
                  v20 = sub_8140(a0,a1,&v11);
                  if (v20)
                    return v11;
                }
              }
              else {
                v20 = 1;
              }
              v21 = v9;
              if (!(char)v39) goto label_88a6;
            }
            else {
              v20 = v4;
            }
          }
          v10 = v9 + 1;
          if (!v26) goto label_88a6;
          if (v40 == v27) goto label_88e6;
label_8838:
          v7 = v37;
          v18 = v38;
        }
        else {
          v4 = 10 <= v19 & v20;
          if (v4) { // branch-flip
            if (v19 * 5 <= v9) goto label_8ad0;
            v20 = v4;
          }
          else {
          }
label_88a6:
          v18 = v38;
          v7 = v37;
          v10 = v9 + 1;
          if (v38 == v25) {
            v6 = memcmp(v37,v24,v38);
            if (!v6) {
label_88e6:
              v10 = v9 + 1;
              v14 = CONCAT44(v34,v15);
              v13[0] = 0;
              v48 = CONCAT44(v40,v39);
              v53 = 0;
              v47 = v38;
              v46 = &v37[v38];
              v45 = (undefined3)(v36 >> 8);
              v44 = (undefined5)v35;
              v52 = 0;
              v49 = v41;
              v50 = v42;
              v51 = v43;
              v54 = a1;
              sub_8bf0(v13);
              if ((v56) && (!v57))
                abort(); // no-return
              v54 = &v54[v55];
              do {
                v10 += 1;
                v53 = 0;
                sub_8bf0(v13);
                if ((v56) && (!v57))
                  return v37;
                sub_8bf0(&v14);
                v18 = v55;
                v1 = v54;
                v9 = v47;
                v7 = v46;
                if ((char)v48) { // branch-flip
                  if (!v48._4_4_)
                    return NULL;
                  if (!v56) goto label_8a53;
                  if (v48._4_4_ != v57) goto label_8838;
                }
                else {
label_8a53:
                  if ((v47 != v55) || (v6 = memcmp(v46,v54,v47), v6)) goto label_8838;
                }
                v46 = &v7[v9];
                v44 = (undefined5)(unsigned int)v44;
                v54 = &v1[v18];
              } while( true );
            }
          }
        }
        v19 += 1;
        v37 = &v7[v18];
        v9 = v10;
      } while( true );
    }
  }
  else {
    v2 = *a1;
    if (v2) {
      v3 = *a0;
      if (!v3)
        return NULL;
      v18 = 0;
      v10 = 0;
      v9 = 0;
      v5 = 1;
      v7 = a1;
      do {
        v18 += 1;
        v8 = v10 + 1;
        v1 = &a0[1];
        if (v2 == v3) {
          v3 = a1[1];
          if (!v3)
            return a0;
          while( true ) {
            v17 = v8 - v10;
            if (!a0[v17])
              return NULL;
            v8 += 1;
            if (a0[v17] != v3) break;
            v3 = a1[v8 - v10];
            if (!v3)
              return a0;
          }
        }
        v10 = v8;
        if (!*v1)
          return NULL;
        v4 = 10 <= v18 & (unsigned char)v5;
        if (v4) {
          if (v18 * 5 <= v10) { // branch-flip
            if (v7) {
              v7 = &v7[strnlen(v7,v10 - v9)];
              v9 = v10;
              if (*v7) goto label_867d;
            }
            v5 = sub_7fa0(v1,a1,strlen(a1),&v11);
            if ((char)v5)
              return v11;
            v7 = NULL;
          }
          else {
label_867d:
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

// Function: sub_8bf0 @ 0x8bf0
void sub_8bf0(char *a0)
{
  char v1;
  char *v2;
  unsigned long v3;
  unsigned long v4; // rax
  long v5; // rax
  
  if (a0[0xc])
    return;
  if (!*a0) {
    v2 = *(char **)&a0[0x10];
    v1 = *v2;
    if (sub_9a80((int)v1)) {
      a0[0x18] = '\x01';
      a0[0x19] = '\0';
      a0[0x1a] = '\0';
      a0[0x1b] = '\0';
      a0[0x1c] = '\0';
      a0[0x1d] = '\0';
      a0[0x1e] = '\0';
      a0[0x1f] = '\0';
      v1 = *v2;
      a0[0x20] = '\x01';
      *(int *)&a0[0x24] = (int)v1;
      goto label_8c32;
    }
    if (!mbsinit(&a0[4]))
      __assert_fail("mbsinit (&iter->state)","mbuiter.h",0x8f,"mbuiter_multi_next"); // no-return
    *a0 = '\x01';
  }
  v4 = __ctype_get_mb_cur_max();
  v3 = *(unsigned long *)&a0[0x10];
  v5 = sub_9680(&a0[0x24],v3,sub_9400(v3,v4),&a0[4]);
  *(long *)&a0[0x18] = v5;
  if (v5 == -1) {
    a0[0x18] = '\x01';
    a0[0x19] = '\0';
    a0[0x1a] = '\0';
    a0[0x1b] = '\0';
    a0[0x1c] = '\0';
    a0[0x1d] = '\0';
    a0[0x1e] = '\0';
    a0[0x1f] = '\0';
    a0[0x20] = '\0';
    a0[0xc] = '\x01';
    return;
  }
  if (v5 != -2) { // branch-flip
    if (!v5) {
      a0[0x18] = '\x01';
      a0[0x19] = '\0';
      a0[0x1a] = '\0';
      a0[0x1b] = '\0';
      a0[0x1c] = '\0';
      a0[0x1d] = '\0';
      a0[0x1e] = '\0';
      a0[0x1f] = '\0';
      if (**(char **)&a0[0x10])
        __assert_fail("*iter->cur.ptr == \'\\0\'","mbuiter.h",0xab,"mbuiter_multi_next"); // no-return
      if (*(int *)&a0[0x24])
        __assert_fail("iter->cur.wc == 0","mbuiter.h",0xac,"mbuiter_multi_next"); // no-return
    }
    a0[0x20] = '\x01';
    if (mbsinit(&a0[4]))
      *a0 = '\0';
  }
  else {
    v4 = strlen(*(char **)&a0[0x10]);
    a0[0x20] = '\0';
    *(unsigned long *)&a0[0x18] = v4;
  }
label_8c32:
  a0[0xc] = '\x01';
  return;
}

// Function: sub_8d70 @ 0x8d70
void sub_8d70(long a0,long a1)
{
  *(long *)(a0 + 0x10) = *(long *)(a0 + 0x10) + a1;
}

// Function: sub_8d80 @ 0x8d80
void sub_8d80(char *a0,char *a1)
{
  char v1;
  
  v1 = *a1;
  *a0 = v1;
  if (v1) // branch-flip
    *(unsigned long *)&a0[4] = *(unsigned long *)&a1[4];
  else {
    a0[4] = '\0';
    a0[5] = '\0';
    a0[6] = '\0';
    a0[7] = '\0';
    a0[8] = '\0';
    a0[9] = '\0';
    a0[10] = '\0';
    a0[0xb] = '\0';
  }
  a0[0xc] = a1[0xc];
  sub_9a20(&a0[0x10],&a1[0x10]); // tail-call
}

// Function: sub_8dc0 @ 0x8dc0
int sub_8dc0(void) // early-return x2
{
  int v1; // eax
  int v2; // ebx
  int v3; // ebp
  
  v3 = 1;
  v2 = 0;
  do {
    v1 = sub_9530(v2,1);
    if (0 <= v1) { // branch-flip
label_8df3:
      if (v3 == 3) {
        v1 = 0;
        return v1;
      }
    }
    else {
      if ((v2) || (v1 = __open_2("/dev/full",1), v1 < 0)) {
        v1 = __open_2("/dev/null",v2 == 0);
        if (v1 < 0) {
          v1 = *__errno_location();
          return v1;
        }
        if (3 <= v1) {
          close(v1); // return-dupe
          v1 = 0;
          return v1;
        }
        goto label_8df3;
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

// Function: sub_8e70 @ 0x8e70
unsigned long sub_8e70(unsigned long a0,long a1,unsigned long a2,long *a3,unsigned long *a4) // return-dupe
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
label_9012:
      if (iconv(a2,0,0,&v5,&v4) != -1) {
        if (v4)
          abort(); // no-return
        *a3 = (long)v3;
        *a4 = (unsigned long)v2;
        return 0;
      }
label_90be:
      if ((char *)*a3 != v3) {
        free(v3);
        return 0xffffffff;
      }
      return 0xffffffff;
    }
    if (iconv(a2,&v7,&v8,&v5,&v4) == -1) {
      if (*__errno_location() != 0x16) goto label_90be;
      goto label_9012;
    }
    a1 = v8;
  } while( true );
}

// Function: sub_90f0 @ 0x90f0
char * sub_90f0(char *a0,unsigned long a1) // return-dupe
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
    if (*v4 == 0x16) goto label_928d;
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
label_928d:
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

// Function: sub_9340 @ 0x9340
char * sub_9340(char *a0,unsigned long a1,unsigned long a2)
{
  int v1;
  char *v2; // rax
  long v3; // rax
  int *v4; // rax
  
  if ((!*a0) || (!sub_7cc0(a1,a2))) {
    v2 = strdup(a0);
    if (!v2)
      *__errno_location() = 0xc;
    return v2;
  }
  v3 = iconv_open(a2,a1);
  if (v3 == -1)
    return NULL;
  v2 = (char *)sub_90f0(a0,v3);
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

// Function: sub_9400 @ 0x9400
void * sub_9400(void *a0,unsigned long a1)
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

// Function: sub_9440 @ 0x9440
void sub_9440(int a0,unsigned long a1)
{
  fcntl(a0,0,a1 & 0xffffffff); // tail-call
}

// Function: sub_9450 @ 0x9450
int sub_9450(unsigned long a0,unsigned long a1)
{
  int v1;
  int v2; // eax
  unsigned int v3; // eax
  int *v4; // rax
  
  if (0 <= dat_1f658) { // branch-flip
    v2 = fcntl((int)a0,0x406,a1 & 0xffffffff);
    if ((0 <= v2) || (*__errno_location() != 0x16)) {
      dat_1f658 = 1;
      return v2;
    }
    v2 = sub_9440(a0 & 0xffffffff,a1 & 0xffffffff);
    if (v2 < 0)
      return v2;
    dat_1f658 = -1;
  }
  else {
    v2 = sub_9440(a0,a1);
    if (v2 < 0)
      return v2;
    if (dat_1f658 != -1)
      return v2;
  }
  v3 = fcntl(v2,1);
  if ((0 <= (int)v3) && (fcntl(v2,2,(unsigned long)(v3 | 1)) != -1))
    return v2;
  v4 = __errno_location();
  v1 = *v4;
  close(v2);
  *v4 = v1;
  return -1;
}

// Function: sub_9530 @ 0x9530
void sub_9530(int a0,int a1,unsigned int a2)
{
  unsigned long v1;
  unsigned long v2; // stack - 0x28
  
  v2 = CONCAT44(v2._4_4_,a2);
  if (!a1) {
    sub_9440(a0,a2);
    return;
  }
  if (a1 == 0x406) {
    sub_9450(a0,a2);
    return;
  }
  if (0xc <= a1) { // branch-flip
    if (0xb <= (unsigned int)(a1 - 0x400U)) {
      fcntl(a0,a1,v2); // return-dupe
      return;
    }
    v1 = 1L << ((unsigned char)(a1 - 0x400U) & 0x3f);
    if (v1 & 0x2c5) {
      fcntl(a0,a1,(unsigned long)a2); // return-dupe, return-dupe
      return;
    }
    v1 &= 0x502;
  }
  else {
    if (a1 <= 0) {
      fcntl(a0,a1,v2);
      return;
    }
    v1 = 1L << ((unsigned char)a1 & 0x3f);
    if (v1 & 0x514) {
      fcntl(a0,a1,(unsigned long)a2);
      return;
    }
    v1 &= 0xa0a;
  }
  if (v1) {
    fcntl(a0,a1);
    return;
  }
  fcntl(a0,a1,v2);
  return;
}

// Function: sub_9680 @ 0x9680
unsigned long sub_9680(unsigned int *a0,unsigned char *a1,unsigned long a2,void *a3)
{
  unsigned long v1; // rax
  long v2;
  
  if (!a0)
    a0 = &v2;
  v1 = mbrtowc(a0,(char *)a1,a2,a3);
  if (((0xfffffffffffffffe <= v1) && (a2)) && (!sub_98e0(0))) {
    v1 = 1;
    *a0 = (unsigned int)*a1;
  }
  return v1;
}

// Function: sub_9710 @ 0x9710
unsigned long sub_9710(int a0) // early-return
{
  unsigned long v1; // rax
  
  if (0x5a < a0)
    return CONCAT71((undefined7)((unsigned long)v1 >> 8),(unsigned int)(a0 - 0x61U) <= 0x19);
  if (0x41 <= a0)
    return 1;
  return (unsigned long)((unsigned int)(a0 - 0x30U) <= 9);
}

// Function: sub_9740 @ 0x9740
bool sub_9740(int a0)
{
  if (a0 <= 0x5a)
    return 0x40 < a0;
  return (unsigned int)(a0 - 0x61U) <= 0x19;
}

// Function: sub_9760 @ 0x9760
bool sub_9760(unsigned int a0)
{
  return a0 <= 0x7f;
}

// Function: sub_9770 @ 0x9770
unsigned int sub_9770(int a0,unsigned long a1,unsigned int a2)
{
  unsigned long v1; // rax
  
  return (unsigned int)CONCAT71((undefined7)((unsigned long)v1 >> 8),a0 == 0x20) | CONCAT31((undefined3)((unsigned int)a2 >> 8),a0 == 9);
}

// Function: sub_9790 @ 0x9790
unsigned long sub_9790(unsigned int a0)
{
  unsigned long v1; // rax
  
  if ((int)a0 <= 0x1f)
    return (unsigned long)(~a0 >> 0x1f);
  return CONCAT71((undefined7)((unsigned long)v1 >> 8),a0 == 0x7f);
}

// Function: sub_97b0 @ 0x97b0
bool sub_97b0(int a0)
{
  return (unsigned int)(a0 - 0x30U) <= 9;
}

// Function: sub_97c0 @ 0x97c0
bool sub_97c0(int a0)
{
  return (unsigned int)(a0 - 0x21U) <= 0x5d;
}

// Function: sub_97d0 @ 0x97d0
bool sub_97d0(int a0)
{
  return (unsigned int)(a0 - 0x61U) <= 0x19;
}

// Function: sub_97e0 @ 0x97e0
bool sub_97e0(int a0)
{
  return (unsigned int)(a0 - 0x20U) <= 0x5e;
}

// Function: sub_97f0 @ 0x97f0
unsigned long sub_97f0(int a0)
{
  unsigned long v1; // rax
  undefined7 v2; // rax
  
  v2 = (undefined7)((unsigned long)v1 >> 8);
  if (0x60 < a0)
    return CONCAT71(v2,(unsigned int)(a0 - 0x7bU) <= 3);
  if (a0 <= 0x5a) {
    if (a0 <= 0x2f)
      return CONCAT71(v2,0x20 < a0);
    return CONCAT71(v2,(unsigned int)(a0 - 0x3aU) <= 6);
  }
  return 1;
}

// Function: sub_9840 @ 0x9840
bool sub_9840(int a0)
{
  if (a0 <= 0xd)
    return 8 < a0;
  return a0 == 0x20;
}

// Function: sub_9860 @ 0x9860
bool sub_9860(int a0)
{
  return (unsigned int)(a0 - 0x41U) <= 0x19;
}

// Function: sub_9870 @ 0x9870
unsigned int sub_9870(int a0)
{
  if ((unsigned int)(a0 - 0x30U) <= 0x36)
    return (unsigned int)(0x7e0000007e03ff >> ((unsigned char)(a0 - 0x30U) & 0x3f)) & 1;
  return 0;
}

// Function: sub_98a0 @ 0x98a0
int sub_98a0(int a0)
{
  if ((unsigned int)(a0 - 0x41U) <= 0x19)
    a0 += 0x20;
  return a0;
}

// Function: sub_98c0 @ 0x98c0
int sub_98c0(int a0)
{
  if ((unsigned int)(a0 - 0x61U) <= 0x19)
    a0 -= 0x20;
  return a0;
}

// Function: sub_98e0 @ 0x98e0
bool sub_98e0(unsigned long a0)
{
  char v1 [264];
  bool v2; // r12b
  
  v2 = 0;
  if ((!sub_9c20(a0,v1,0x101)) && (strcmp(v1,"C")))
    v2 = strcmp(v1,"POSIX") != 0;
  return v2;
}

// Function: sub_9970 @ 0x9970
long sub_9970(unsigned long a0)
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

// Function: sub_99c0 @ 0x99c0
void sub_99c0(unsigned long a0)
{
  if (a0 & 0xf)
    abort(); // no-return
  if (!(a0 & 0x10))
    return;
  free((void *)(a0 - *(unsigned char *)(a0 - 1))); // tail-call
}

// Function: sub_99f0 @ 0x99f0
int sub_99f0(int a0)
{
  int v1; // eax
  
  v1 = wcwidth(a0);
  if (0 <= v1)
    return v1;
  return (unsigned long)(iswcntrl(a0) == 0);
}

// Function: sub_9a20 @ 0x9a20
void sub_9a20(long *a0,long *a1) // return-dupe
{
  long v1;
  long *v2;
  
  v2 = (long *)*a1;
  if (v2 == &a1[3])
    v2 = memcpy(&a0[3],&a1[3],a1[1]);
  v1 = a1[1];
  *a0 = (long)v2;
  a0[1] = v1;
  v1 = a1[2];
  *(char *)&a0[2] = (char)v1;
  if (!(char)v1)
    return;
  *(unsigned int *)((long)a0 + 0x14) = *(unsigned int *)((long)a1 + 0x14);
}

// Function: sub_9a80 @ 0x9a80
unsigned int sub_9a80(unsigned char a0)
{
  return *(unsigned int *)((unsigned long)(a0 >> 5) * 4 + 0xb260) >> (a0 & 0x1f) & 1;
}

// Function: sub_9aa0 @ 0x9aa0
unsigned long sub_9aa0(char *a0)
{
  char v1 [4]; // stack - 0x68
  unsigned long v2; // r12
  unsigned long v3; // stack - 0x64
  char v4; // stack - 0x5c
  char *v5; // stack - 0x58
  long v6; // stack - 0x50
  char v7; // stack - 0x48
  int v8; // stack - 0x44
  
  if (__ctype_get_mb_cur_max() <= 1)
    return strlen(a0); // tail-call
  v2 = 0;
  v1[0] = 0;
  v3 = 0;
  v5 = a0;
  while( true ) {
    v4 = 0;
    sub_8bf0(v1);
    if ((v7) && (!v8)) break;
    v5 = &v5[v6];
    v2 += 1;
  }
  return v2;
}

// Function: sub_9b60 @ 0x9b60
void sub_9b60(int a0)
{
  setlocale(a0,NULL); // tail-call
}

// Function: sub_9b70 @ 0x9b70
unsigned long sub_9b70(unsigned long a0,char *a1,unsigned long a2) // return-dupe x2
{
  char *v1; // rax
  unsigned long v2; // rax
  
  v1 = (char *)sub_9b60();
  if (!v1) {
    if (!a2)
      return 0x16;
    *a1 = 0;
    return 0x16;
  }
  v2 = strlen(v1);
  if (a2 <= v2) {
    if (!a2)
      return 0x22;
    memcpy(a1,v1,a2 - 1);
    a1[a2 - 1] = 0;
    return 0x22;
  }
  memcpy(a1,v1,v2 + 1);
  return 0;
}

// Function: sub_9c20 @ 0x9c20
void sub_9c20(void)
{
  sub_9b70(); // tail-call
}

// Function: sub_9c30 @ 0x9c30
void sub_9c30(void)
{
  sub_9b60(); // tail-call
}

// Function: _DT_FINI @ 0x9c3c
void _DT_FINI(void)
{
  return;
}
