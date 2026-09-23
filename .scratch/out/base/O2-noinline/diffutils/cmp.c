// Function: _DT_INIT @ 0x2000
void _DT_INIT(void) // return-dupe
{
  if (!dat_cfd0)
    return;
  (*dat_cfd0)();
}

// Function: sub_2020 @ 0x2020
void sub_2020(void)
{
  (*dat_cd80)(); // jump-as-call
}

// Function: __cxa_finalize @ 0x24a0
void __cxa_finalize(void)
{
  (*dat_cfe0)(); // jump-as-call
}

// Function: raise @ 0x24b0
int raise(int a0)
{
  int v1; // eax
  
  v1 = (*dat_cd88)(); // jump-as-call
  return v1;
}

// Function: free @ 0x24c0
void free(void *a0)
{
  (*dat_cd90)(); // jump-as-call
}

// Function: strtoimax @ 0x24d0
void strtoimax(void)
{
  (*dat_cd98)(); // jump-as-call
}

// Function: __mempcpy_chk @ 0x24e0
void * __mempcpy_chk(void *a0,void *a1,unsigned long a2,unsigned long a3)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_cda0)(); // jump-as-call
  return v1;
}

// Function: abort @ 0x24f0
void abort(void)
{
  (*dat_cda8)(); // jump-as-call
}

// Function: __errno_location @ 0x2500
int * __errno_location(void)
{
  int *v1; // rax
  
  v1 = (int *)(*dat_cdb0)(); // jump-as-call
  return v1;
}

// Function: strncmp @ 0x2510
int strncmp(char *a0,char *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_cdb8)(); // jump-as-call
  return v1;
}

// Function: _exit @ 0x2520
void _exit(int a0)
{
  (*dat_cdc0)(); // jump-as-call
}

// Function: iconv @ 0x2530
void iconv(void)
{
  (*dat_cdc8)(); // jump-as-call
}

// Function: sigaction @ 0x2540
int sigaction(int a0,void *a1,void *a2)
{
  int v1; // eax
  
  v1 = (*dat_cdd0)(); // jump-as-call
  return v1;
}

// Function: iswcntrl @ 0x2550
void iswcntrl(void)
{
  (*dat_cdd8)(); // jump-as-call
}

// Function: reallocarray @ 0x2560
void * reallocarray(void *a0,unsigned long a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_cde0)(); // jump-as-call
  return v1;
}

// Function: fcntl @ 0x2570
int fcntl(int a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_cde8)(); // jump-as-call
  return v1;
}

// Function: write @ 0x2580
long write(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_cdf0)(); // jump-as-call
  return v1;
}

// Function: textdomain @ 0x2590
char * textdomain(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_cdf8)(); // jump-as-call
  return v1;
}

// Function: __open_2 @ 0x25a0
void __open_2(void)
{
  (*dat_ce00)(); // jump-as-call
}

// Function: fclose @ 0x25b0
int fclose(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_ce08)(); // jump-as-call
  return v1;
}

// Function: bindtextdomain @ 0x25c0
char * bindtextdomain(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_ce10)(); // jump-as-call
  return v1;
}

// Function: dcgettext @ 0x25d0
char * dcgettext(char *a0,char *a1,int a2)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_ce18)(); // jump-as-call
  return v1;
}

// Function: __ctype_get_mb_cur_max @ 0x25e0
unsigned long __ctype_get_mb_cur_max(void)
{
  unsigned long v1; // rax
  
  v1 = (*dat_ce20)(); // jump-as-call
  return v1;
}

// Function: strlen @ 0x25f0
unsigned long strlen(char *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_ce28)(); // jump-as-call
  return v1;
}

// Function: __stack_chk_fail @ 0x2600
void __stack_chk_fail(void)
{
  (*dat_ce30)(); // jump-as-call
}

// Function: mmap @ 0x2610
void mmap(void)
{
  (*dat_ce38)(); // jump-as-call
}

// Function: getopt_long @ 0x2620
int getopt_long(int a0,char **a1,char *a2,void *a3,int *a4)
{
  int v1; // eax
  
  v1 = (*dat_ce40)(); // jump-as-call
  return v1;
}

// Function: mbrtowc @ 0x2630
unsigned long mbrtowc(void *a0,char *a1,unsigned long a2,void *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_ce48)(); // jump-as-call
  return v1;
}

// Function: strchr @ 0x2640
char * strchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_ce50)(); // jump-as-call
  return v1;
}

// Function: strrchr @ 0x2650
char * strrchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_ce58)(); // jump-as-call
  return v1;
}

// Function: lseek @ 0x2660
void lseek(void)
{
  (*dat_ce60)(); // jump-as-call
}

// Function: __assert_fail @ 0x2670
void __assert_fail(char *a0,char *a1,unsigned int a2,char *a3)
{
  (*dat_ce68)(); // jump-as-call
}

// Function: fputs @ 0x2680
int fputs(char *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_ce70)(); // jump-as-call
  return v1;
}

// Function: strnlen @ 0x2690
unsigned long strnlen(char *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  v1 = (*dat_ce78)(); // jump-as-call
  return v1;
}

// Function: close @ 0x26a0
int close(int a0)
{
  int v1; // eax
  
  v1 = (*dat_ce80)(); // jump-as-call
  return v1;
}

// Function: memchr @ 0x26b0
void * memchr(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_ce88)(); // jump-as-call
  return v1;
}

// Function: read @ 0x26c0
long read(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_ce90)(); // jump-as-call
  return v1;
}

// Function: memcmp @ 0x26d0
int memcmp(void *a0,void *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_ce98)(); // jump-as-call
  return v1;
}

// Function: fputs_unlocked @ 0x26e0
int fputs_unlocked(char *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_cea0)(); // jump-as-call
  return v1;
}

// Function: rawmemchr @ 0x26f0
void * rawmemchr(void *a0,int a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_cea8)(); // jump-as-call
  return v1;
}

// Function: ferror_unlocked @ 0x2700
int ferror_unlocked(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_ceb0)(); // jump-as-call
  return v1;
}

// Function: calloc @ 0x2710
void * calloc(unsigned long a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_ceb8)(); // jump-as-call
  return v1;
}

// Function: strcmp @ 0x2720
int strcmp(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_cec0)(); // jump-as-call
  return v1;
}

// Function: signal @ 0x2730
void signal(void)
{
  (*dat_cec8)(); // jump-as-call
}

// Function: sigemptyset @ 0x2740
int sigemptyset(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_ced0)(); // jump-as-call
  return v1;
}

// Function: stat @ 0x2750
int stat(char *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_ced8)(); // jump-as-call
  return v1;
}

// Function: memcpy @ 0x2760
void * memcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_cee0)(); // jump-as-call
  return v1;
}

// Function: mincore @ 0x2770
void mincore(void)
{
  (*dat_cee8)(); // jump-as-call
}

// Function: wcwidth @ 0x2780
int wcwidth(int a0)
{
  int v1; // eax
  
  v1 = (*dat_cef0)(); // jump-as-call
  return v1;
}

// Function: iswalnum @ 0x2790
void iswalnum(void)
{
  (*dat_cef8)(); // jump-as-call
}

// Function: malloc @ 0x27a0
void * malloc(unsigned long a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_cf00)(); // jump-as-call
  return v1;
}

// Function: nl_langinfo @ 0x27b0
char * nl_langinfo(int a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_cf08)(); // jump-as-call
  return v1;
}

// Function: realloc @ 0x27c0
void * realloc(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_cf10)(); // jump-as-call
  return v1;
}

// Function: munmap @ 0x27d0
int munmap(void *a0,unsigned long a1)
{
  int v1; // eax
  
  v1 = (*dat_cf18)(); // jump-as-call
  return v1;
}

// Function: setlocale @ 0x27e0
char * setlocale(int a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_cf20)(); // jump-as-call
  return v1;
}

// Function: __printf_chk @ 0x27f0
int __printf_chk(int a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_cf28)(); // jump-as-call
  return v1;
}

// Function: iconv_close @ 0x2800
void iconv_close(void)
{
  (*dat_cf30)(); // jump-as-call
}

// Function: iswspace @ 0x2810
int iswspace(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_cf38)(); // jump-as-call
  return v1;
}

// Function: mempcpy @ 0x2820
void * mempcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_cf40)(); // jump-as-call
  return v1;
}

// Function: memmove @ 0x2830
void * memmove(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_cf48)(); // jump-as-call
  return v1;
}

// Function: error @ 0x2840
void error(int a0,int a1,char *a2,...)
{
  (*dat_cf50)(); // jump-as-call
}

// Function: open @ 0x2850
int open(char *a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_cf58)(); // jump-as-call
  return v1;
}

// Function: perror @ 0x2860
void perror(char *a0)
{
  (*dat_cf60)(); // jump-as-call
}

// Function: getpagesize @ 0x2870
int getpagesize(void)
{
  int v1; // eax
  
  v1 = (*dat_cf68)(); // jump-as-call
  return v1;
}

// Function: exit @ 0x2880
void exit(int a0)
{
  (*dat_cf70)(); // jump-as-call
}

// Function: __fprintf_chk @ 0x2890
int __fprintf_chk(void *a0,int a1,char *a2,...)
{
  int v1; // eax
  
  v1 = (*dat_cf78)(); // jump-as-call
  return v1;
}

// Function: strdup @ 0x28a0
char * strdup(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_cf80)(); // jump-as-call
  return v1;
}

// Function: mbsinit @ 0x28b0
int mbsinit(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_cf88)(); // jump-as-call
  return v1;
}

// Function: sigaltstack @ 0x28c0
void sigaltstack(void)
{
  (*dat_cf90)(); // jump-as-call
}

// Function: fstat @ 0x28d0
int fstat(int a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_cf98)(); // jump-as-call
  return v1;
}

// Function: sigaddset @ 0x28e0
int sigaddset(void *a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_cfa0)(); // jump-as-call
  return v1;
}

// Function: __ctype_b_loc @ 0x28f0
void * __ctype_b_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_cfa8)(); // jump-as-call
  return v1;
}

// Function: iconv_open @ 0x2900
void iconv_open(void)
{
  (*dat_cfb0)(); // jump-as-call
}

// Function: __sprintf_chk @ 0x2910
int __sprintf_chk(char *a0,int a1,unsigned long a2,char *a3,...)
{
  int v1; // eax
  
  v1 = (*dat_cfb8)(); // jump-as-call
  return v1;
}

// Function: sub_2920 @ 0x2920
undefined16 sub_2920(void)
{
  char v1 [16];
  unsigned long v2; // rax
  
  *__errno_location() = 0xc;
  v1._8_8_ = 0;
  v1._0_8_ = v2;
  return v1._0_16_ << 0x40;
}

// Function: sub_2940 @ 0x2940
unsigned long sub_2940(int a0,char **a1)
{
  unsigned long v1;
  long v10; // rax
  long v11; // rax
  long v12;
  long v13;
  unsigned long v14; // rax
  char *v15; // stack - 0x170
  long v16; // stack - 0x168
  long v17; // stack - 0xd8
  long v18; // stack - 0x160
  unsigned int v19; // stack - 0x150
  char *v2;
  long v20; // stack - 0x140
  long v21; // stack - 0xd0
  unsigned int v22; // stack - 0xc0
  long v23; // stack - 0xb0
  int v3;
  int v4; // eax
  int v5; // eax
  unsigned long v6; // rax
  unsigned long v7; // rax
  long v8;
  unsigned long v9; // rax
  
  dat_d020 = 2;
  sub_4180(*a1);
  setlocale(6,"");
  bindtextdomain("diffutils","/usr/local/share/locale");
  textdomain("diffutils");
  sub_3f90(0);
  sub_64f0();
  while (v3 = getopt_long(a0,a1,"bci:ln:sv",(void *)0xca40,NULL), v3 != -1) {
    switch(v3) {
      case 0x62:
      case 99:
        dat_d0b2 = 1;
        break;
      default:
        sub_3240(0,0); // no-return
      case 0x69:
        sub_32d0(0,&optarg,0x3a);
        v2 = &optarg[1];
        if (*optarg != ':') { // branch-flip
          optarg = v2;
          if (dat_d0c8 < dat_d0c0)
            dat_d0c8 = dat_d0c0;
        }
        else {
          optarg = v2;
          sub_32d0(1,&optarg,0);
        }
        break;
      case 0x6c:
        sub_32a0(1);
        break;
      case 0x6e:
        v3 = sub_6640(optarg,0,0,&v15,"kKMGTPEZY0");
        if ((v3) || ((long)v15 < 0))
          sub_3240("invalid --bytes value \'%s\'",optarg); // no-return
        if (((long)dat_d010 < 0) || ((long)v15 <= (long)dat_d010))
          dat_d010 = v15;
        break;
      case 0x73:
        sub_32a0(3);
        break;
      case 0x76:
        v6 = sub_4490("David MacKenzie");
        v7 = sub_4520("Torbjorn Granlund",0x9682);
        sub_5f90(stdout,"cmp","GNU diffutils",dat_d018,v7,v6,0,v7);
        sub_3d80();
        return 0;
      case 0x80:
        sub_3370();
        sub_3d80();
        return 0;
      
    }
  }
  if (optind == a0)
    sub_3240("missing operand after \'%s\'",a1[(long)a0 + -1]); // no-return
  dat_d230 = a1[optind];
  dat_d238 = "-";
  v3 = optind + 1;
  if (optind + 1 < a0) {
    dat_d238 = a1[(long)optind + 1];
    v3 = optind + 2;
  }
  optind = v3;
  v8 = (long)optind;
  if (optind < a0) {
    optind += 1;
    v15 = a1[v8];
    sub_32d0(0,&v15,0);
    v8 = (long)optind;
    if (optind < a0) {
      optind += 1;
      v15 = a1[v8];
      sub_32d0(1,&v15,0);
      if (optind < a0)
        sub_3240("extra operand \'%s\'",a1[optind]); // no-return
    }
  }
  v8 = 0;
  do {
    if (((v8) && (dat_d0c0 == dat_d0c8)) && (v3 = strcmp(dat_d230,dat_d238), !v3))
      return 0;
    v2 = *(char **)(v8 * 8 + 0xd230);
    if (strcmp(v2,"-")) { // branch-flip
      v3 = open(v2,0,0);
      *(int *)(v8 * 4 + 0xd220) = v3;
      if (v3 <= -1) {
        if (dat_d0b4 == 3)
          exit(2); // no-return
        error(2,*__errno_location(),"%s",*(unsigned long *)((long)(int)v8 * 8 + 0xd230)); // return-dupe
        return v9;
      }
    }
    else {
      *(unsigned int *)(v8 * 4 + 0xd220) = 0;
      v3 = 0;
    }
    if (fstat(v3,(void *)(v8 * 0x90 + 0xd100))) {
      error(2,*__errno_location(),"%s",*(unsigned long *)((long)(int)v8 * 8 + 0xd230));
      return v9;
    }
    if (v8 == 1) goto label_2ca0;
    v8 = 1;
  } while( true );
label_2ca0:
  if ((dat_d108 != dat_d198) || (dat_d100 != dat_d190)) {
    if ((dat_d118 & 0xf000) != 0x6000) {
      if ((dat_d118 & 0xf000) == 0x2000) {
        if ((dat_d1a8 & 0xf000) == 0x2000) goto label_2f2c;
        goto label_2e90;
      }
      goto label_2cd7;
    }
    if ((dat_d1a8 & 0xf000) == 0x6000) {
label_2f2c:
      if (dat_d128 == dat_d1b8) goto label_2dd8;
      goto label_2cd7;
    }
label_2e90:
    if (dat_d0b4 == 3) goto label_2d0c;
label_2e9d:
    if ((fstat(1,&v16)) || (stat("/dev/null",&v17))) goto label_2eb4;
    if ((v18 != v21) || (v16 != v17)) {
      if ((v19 & 0xf000) != 0x6000) { // branch-flip
        if (((v19 & 0xf000) == 0x2000) && ((v22 & 0xf000) == 0x2000)) {
label_2faa:
          if (v20 == v23) goto label_2fbd;
        }
      }
      else if ((v22 & 0xf000) == 0x6000) goto label_2faa;
    }
    else {
label_2fbd:
      dat_d0b4 = 2;
    }
label_2eb4:
    if (dat_d0b4 != 3) goto label_2d0c;
  }
  else {
label_2dd8:
    if (((dat_d118 == dat_d1a8) && (dat_d110 == dat_d1a0)) && ((dat_d11c == dat_d1ac && ((((dat_d120 == dat_d1b0 && (dat_d130 == dat_d1c0)) && (dat_d158 == dat_d1e8)) && (dat_d168 == dat_d1f8)))))) {
      v8 = sub_34c0(0);
      if (v8 == sub_34c0(1))
        return 0;
    }
label_2cd7:
    if (dat_d0b4 != 3) goto label_2e9d;
  }
  v8 = dat_d130;
  if (((dat_d118 & 0xf000) == 0x8000) && ((dat_d1a8 & 0xf000) == 0x8000)) {
    v10 = sub_34c0(0);
    v13 = dat_d1c0;
    v11 = sub_34c0(1);
    v12 = 0;
    if (0 <= v8 - v10)
      v12 = v8 - v10;
    v13 -= v11;
    if (v13 <= -1)
      v13 = 0;
    if (v13 != v12) {
      if (0 <= (long)dat_d010) {
        if (v12 < v13)
          v13 = v12;
        if ((long)dat_d010 <= v13) goto label_2d0c;
      }
      exit(1); // no-return
    }
  }
label_2d0c:
  dat_d0d0 = sub_6ac0(dat_d138,dat_d1c8,0x7ffffffffffffff7);
  v1 = dat_d0d0 + 0xf;
  dat_d0e0 = sub_6100((v1 >> 3) << 4);
  dat_d0e8 = dat_d0e0 + (v1 & 0xfffffffffffffff8);
  v4 = sub_35e0();
  v5 = close(dat_d220);
  v3 = 0;
  if (!v5) {
    v3 = close(dat_d224);
    if (!v3) {
      if ((v4) && (dat_d0b4 < 2))
        sub_3d80();
      exit(v4); // no-return
    }
    v3 = 1;
  }
  error(2,*__errno_location(),"%s",*(unsigned long *)((long)v3 * 8 + 0xd230));
  return v14;
}

// Function: sub_3100 @ 0x3100
void sub_3100(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_cfc0)(sub_2940,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: sub_3130 @ 0x3130
void sub_3130(void)
{
  return;
}

// Function: _FINI_0 @ 0x31a0
void _FINI_0(void)
{
  if (!dat_d088) {
    if (dat_cfe0)
      __cxa_finalize(dat_d008);
    sub_3130();
    dat_d088 = 1;
    return;
  }
}

// Function: _INIT_0 @ 0x31e0
void _INIT_0(void)
{
  return;
}

// Function: sub_31f0 @ 0x31f0
long sub_31f0(long *a0,long *a1)
{
  long *v1;
  
  v1 = a0;
  if (*a0 != *a1) { // branch-flip
    for (; (char)*v1 == (char)*a1; v1 = (long *)((long)v1 + 1)) {
label_3230:
      a1 = (long *)((long)a1 + 1);
    }
  }
  else {
    do {
      v1 = &v1[1];
      a1 = &a1[1];
    } while (*v1 == *a1);
    if ((char)*v1 == (char)*a1) goto label_3230;
  }
  return (long)v1 - (long)a0;
}

// Function: sub_3240 @ 0x3240
void sub_3240(char *a0,unsigned long a1)
{
  unsigned long v1;
  
  if (a0)
    error(0,0,dcgettext(NULL,a0,5),a1);
  v1 = dat_1d280;
  error(2,0,dcgettext(NULL,"Try \'%s --help\' for more information.",5),v1);
}

// Function: sub_32a0 @ 0x32a0
void sub_32a0(int a0)
{
  if ((dat_d0b4) && (dat_d0b4 != a0))
    sub_3240("options -l and -s are incompatible",0); // no-return
  dat_d0b4 = a0;
}

// Function: sub_32d0 @ 0x32d0
void sub_32d0(int a0,unsigned long *a1,char a2) // return-dupe
{
  unsigned long v1;
  int v2; // eax
  long v3; // stack - 0x38
  
  v1 = *a1;
  v2 = sub_6640(v1,a1,0,&v3,"kKMGTPEZY0");
  if (((!v2) || ((v2 == 2 && (*(char *)*a1 == a2)))) && (0 <= v3)) {
    if (v3 <= *(long *)((long)a0 * 8 + 0xd0c0))
      return;
    *(long *)((long)a0 * 8 + 0xd0c0) = v3;
    return;
  }
  sub_3240("invalid --ignore-initial value \'%s\'",v1); // no-return
}

// Function: sub_3370 @ 0x3370
void sub_3370(void)
{
  void *v1;
  unsigned long v2;
  char *v3; // rax
  long *v4; // rbx
  char *v5;
  
  v2 = dat_1d280;
  v4 = (long *)0xc9e0;
  __printf_chk(1,dcgettext(NULL,"Usage: %s [OPTION]... FILE1 [FILE2 [SKIP1 [SKIP2]]]\n",5),v2);
  __printf_chk(1,"%s\n",dcgettext(NULL,"Compare two files byte by byte.",5));
  __printf_chk(1,"\n%s\n\n",dcgettext(NULL,"The optional SKIP1 and SKIP2 specify the number of bytes to skip\nat the beginning of each file (zero by default).",5));
  v1 = stdout;
  fputs_unlocked(dcgettext(NULL,"Mandatory arguments to long options are mandatory for short options too.\n",5),v1);
  v5 = "-b, --print-bytes          print differing bytes";
  do {
    v4 = &v4[1];
    __printf_chk(1,"  %s\n",dcgettext(NULL,v5,5));
    v5 = (char *)*v4;
  } while (v5);
  v5 = dcgettext(NULL,"Exit status is 0 if inputs are the same, 1 if different, 2 if trouble.",5);
  v3 = dcgettext(NULL,"If a FILE is \'-\' or missing, read standard input.",5);
  __printf_chk(1,"\n%s\n\n%s\n%s\n",dcgettext(NULL,"SKIP values may be followed by the following multiplicative suffixes:\nkB 1000, K 1024, MB 1,000,000, M 1,048,576,\nGB 1,000,000,000, G 1,073,741,824, and so on for T, P, E, Z, Y.",5),v3,v5);
  sub_6040(); // tail-call
}

// Function: sub_34c0 @ 0x34c0
unsigned long sub_34c0(int a0)
{
  unsigned long v1; // rax
  long v2; // rbx
  
  v2 = (long)a0;
  if (*(char *)(v2 + 0xd0b0))
    return *(unsigned long *)(v2 * 8 + 0xd0a0);
  *(char *)(v2 + 0xd0b0) = 1;
  v1 = lseek(*(unsigned int *)(v2 * 4 + 0xd220),*(unsigned long *)(v2 * 8 + 0xd0c0),1);
  *(unsigned long *)(v2 * 8 + 0xd0a0) = v1;
  return v1;
}

// Function: sub_3510 @ 0x3510
long sub_3510(char *a0,long a1)
{
  char *v1;
  char v2;
  long v3; // r12
  
  v3 = 0;
  v1 = &a0[a1];
  v2 = *v1;
  *v1 = 10;
  while( true ) {
    a0 = rawmemchr(a0,10);
    if (v1 == a0) break;
    v3 += 1;
    a0 = &a0[1];
  }
  *v1 = v2;
  return v3;
}

// Function: sub_3550 @ 0x3550
void sub_3550(unsigned char *a0,unsigned char a1)
{
  unsigned char *v1; // rbp
  
  if (!(*(unsigned char *)(*(long *)__ctype_b_loc() + 1 + (unsigned long)a1 * 2) & 0x40)) {
    if ((char)a1 < '\0') {
      a1 += 0x80;
      v1 = &a0[2];
      a0[0] = 0x4d;
      a0[1] = 0x2d;
      a0 = v1;
    }
    if (0x20 <= a1) { // branch-flip
      if (a1 == 0x7f) {
        *a0 = 0x5e;
        a1 = 0x3f;
        a0 = &a0[1];
      }
    }
    else {
      *a0 = 0x5e;
      a1 += 0x40;
      a0 = &a0[1];
    }
  }
  *a0 = a1;
  a0[1] = 0;
}

// Function: sub_35e0 @ 0x35e0
unsigned long sub_35e0(void) // return-dupe x2
{
  unsigned char v1;
  unsigned long v10;
  unsigned long v11;
  unsigned long v12;
  int *v13; // rax
  unsigned long v14; // rax
  unsigned long v15; // rax
  unsigned long v16;
  char *v17; // rax
  unsigned long v18; // rax
  unsigned long v19; // rax
  void *v2;
  char v20 [24];
  char v21 [5];
  char v22 [32];
  char v23 [5];
  long v24;
  long v25; // r13
  long v26;
  unsigned long v27; // stack - 0xe0
  int v28; // stack - 0xd8
  unsigned int v29; // stack - 0xcc
  void *v3;
  long v30; // stack - 0xc8
  bool v31; // stack - 0xa1
  unsigned long v4;
  unsigned char v5;
  unsigned int v6; // eax
  unsigned int v7; // eax
  unsigned long v8;
  unsigned long v9;
  
  v26 = dat_d130;
  v3 = dat_d0e8;
  v2 = dat_d0e0;
  v27 = dat_d010;
  if (dat_d0b4 == 1) {
    v8 = 0x7fffffffffffffff;
    if (0 <= (long)dat_d010)
      v8 = dat_d010;
    if ((dat_d118 & 0xf000) == 0x8000) {
      v9 = v26 - sub_34c0(0);
      if ((long)v9 < (long)v8)
        v8 = v9;
    }
    v26 = dat_d1c0;
    if ((dat_d1a8 & 0xf000) == 0x8000) {
      v9 = v26 - sub_34c0(1);
      if ((long)v9 < (long)v8)
        v8 = v9;
    }
    v29 = 1;
    while (v8 = (long)v8 / 10, v8) {
      v29 += 1;
    }
  }
  v26 = 0;
  do {
    v8 = *(unsigned long *)(v26 * 8 + 0xd0c0);
    if ((v8) && (sub_34c0(v26) == -1)) {
      do {
        v9 = dat_d0d0;
        if (v8 <= dat_d0d0)
          v9 = v8;
        v12 = sub_6a10(*(unsigned int *)(v26 * 4 + 0xd220),v2,v9);
        if (v9 != v12) {
          if (v12 == 0xffffffffffffffff) {
            v13 = __errno_location();
            error(2,*v13,"%s",*(unsigned long *)((long)(int)v26 * 8 + 0xd230));
            return v14;
          }
          break;
        }
        v8 -= v9;
      } while (v8);
    }
    if (v26 == 1) goto label_3658;
    v26 = 1;
  } while( true );
label_3658:
  v31 = 1;
  v28 = 0;
  v30 = 1;
  v26 = 1;
  v8 = dat_d0d0;
  do {
    v9 = v8;
    if (0 <= (long)v27) {
      if (v27 <= v8)
        v9 = v27;
      v27 -= v9;
    }
    v8 = sub_6a10(dat_d220,v2,v9);
    if (v8 == 0xffffffffffffffff) {
      error(2,*__errno_location(),"%s",dat_d230);
      return v19;
    }
    v9 = sub_6a10(dat_d224,v3,v9);
    if (v9 == 0xffffffffffffffff) {
      error(2,*__errno_location(),"%s",dat_d238);
      return v18;
    }
    v12 = v9;
    if (v8 <= v9)
      v12 = v8;
    if (memcmp(v2,v3,v12)) { // branch-flip
      if ((v9 <= v8) && (*(unsigned char *)((long)v3 + v8) = 0x55, v8 != v9))
        v5 = 0xaa;
      else {
        *(unsigned char *)((long)v2 + v9) = 0x79;
        v5 = ~*(unsigned char *)((long)v3 + v8);
      }
      *(unsigned char *)((long)v2 + v8) = v5;
      *(unsigned char *)((long)v3 + v9) = ~*(unsigned char *)((long)v2 + v9);
      v6 = ~(unsigned int)v8 & 7;
      if (v6) {
        v7 = 0;
        do {
          v10 = (unsigned long)v7;
          v7 += 1;
          *(char *)((long)v2 + v10 + v8 + 1) = 0;
        } while (v7 < v6);
      }
      v6 = ~(unsigned int)v9 & 7;
      if (v6) {
        v7 = 0;
        do {
          v10 = (unsigned long)v7;
          v7 += 1;
          *(char *)((long)v3 + v10 + v9 + 1) = 0;
        } while (v7 < v6);
      }
      v10 = sub_31f0(v2,v3);
      v24 = v26 + v10;
      if ((v10) && (!dat_d0b4)) {
label_3905:
        v30 += sub_3510(v2,v10);
        if (v10 < v12) {
label_3a01:
          v11 = sub_40e0(v24,v22);
          v15 = sub_40e0(v30,v20);
          if (!dat_d0b2) {
            v17 = dcgettext(NULL,"%s %s differ: byte %s, line %s\n",5);
            if ((v17 == "%s %s differ: byte %s, line %s\n") && (!sub_4050(5)))
              v17 = "%s %s differ: char %s, line %s\n";
            __printf_chk(1,v17,dat_d230,dat_d238,v11,v15);
            return 1;
          }
          v5 = *(unsigned char *)((long)v2 + v10);
          v1 = *(unsigned char *)((long)v3 + v10);
          sub_3550(v23,(unsigned long)v5);
          sub_3550(v21,v1);
          v4 = dat_d238;
          v16 = dat_d230;
          __printf_chk(1,dcgettext(NULL,"%s %s differ: byte %s, line %s is %3o %s %3o %s\n",5),v16,v4,v11,v15,(unsigned long)v5,v23,(unsigned long)v1,v21);
          return 1;
        }
        v31 = *(char *)((long)v2 + (v10 - 1)) == '\n';
      }
      else if (v10 < v12) {
        if (dat_d0b4 == 2) {
          v28 = 1;
          if (v8 != v9)
            return 1;
          break;
        }
        if (3 <= dat_d0b4) { // branch-flip
          if (dat_d0b4 == 3)
            return 1;
        }
        else {
          if (!dat_d0b4) goto label_3a01;
          do {
            while( true ) {
              v25 = v24;
              v5 = *(unsigned char *)((long)v2 + (v25 - v26));
              v1 = *(unsigned char *)((long)v3 + (v25 - v26));
              if (v5 != v1) break;
label_37fa:
              v24 = v25 + 1;
              if (v26 + v12 == v25 + 1) goto label_38b0;
            }
            v11 = sub_40e0(v25,v20);
            if (!dat_d0b2) {
              __printf_chk(1,"%*s %3o %3o\n",(unsigned long)v29,v11,(unsigned long)v5,(unsigned long)v1);
              goto label_37fa;
            }
            sub_3550(v21,(unsigned long)v5);
            sub_3550(v22,v1);
            __printf_chk(1,"%*s %3o %-4s %3o %s\n",(unsigned long)v29,v11,(unsigned long)v5,v21,(unsigned long)v1,v22);
            v24 = v25 + 1;
          } while (v26 + v12 != v25 + 1);
label_38b0:
          v24 = v25 + 1;
          v28 = -1;
        }
      }
    }
    else {
      v24 = v26 + v12;
      if ((!dat_d0b4) && (v12)) {
        v10 = v12;
        goto label_3905;
      }
    }
    if (v8 != v9) {
      if (dat_d0b4 == 3)
        return 1;
      v11 = *(unsigned long *)((unsigned long)(v9 < v8) * 8 + 0xd230);
      if (v24 != 1) {
        v15 = sub_40e0(v24 + -1,v22);
        if (!dat_d0b4) {
          v16 = sub_40e0(v30 - (unsigned long)v31,v20);
          if (v31) // branch-flip
            v17 = dcgettext(NULL,"cmp: EOF on %s after byte %s, line %s\n",5);
          else {
            v17 = dcgettext(NULL,"cmp: EOF on %s after byte %s, in line %s\n",5);
          }
          __fprintf_chk(stderr,1,v17,v11,v15,v16);
          return 1;
        }
        __fprintf_chk(stderr,1,dcgettext(NULL,"cmp: EOF on %s after byte %s\n",5),v11,v15);
        return 1;
      }
      __fprintf_chk(stderr,1,dcgettext(NULL,"cmp: EOF on %s which is empty\n",5),v11);
      return 1;
    }
    v26 = v24;
  } while (dat_d0d0 == v8);
  return (unsigned long)(v28 != 0);
}

// Function: sub_3d80 @ 0x3d80
void sub_3d80(void)
{
  int v1; // eax
  char *v2; // rax
  
  v1 = ferror_unlocked(stdout);
  if (v1) {
    error(2,0,"%s",dcgettext(NULL,"write failed",5));
    return;
  }
  v1 = fclose(stdout);
  if (!v1)
    return;
  v2 = dcgettext(NULL,"standard output",5);
  error(2,*__errno_location(),"%s",v2);
}

// Function: sub_3e10 @ 0x3e10
void sub_3e10(void)
{
  return;
}

// Function: sub_3e20 @ 0x3e20
void sub_3e20(int a0)
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
  (*dat_d260)();
  if (a0) // branch-flip
    v6 = dat_d258;
  else {
    v6 = dat_d250;
  }
  v1 = strlen(dat_d248);
  v2 = strlen(v6);
  if (0x1000 <= v1 + v2) { // branch-flip
    write(2,dat_d248,v1);
    write(2,": C",2);
    write(2,v6,v2);
    v5[0] = 10;
    v4 = (char *)0x1;
  }
  else {
    v3 = __mempcpy_chk(v5,dat_d248,v1,0x1002);
    *(unsigned short *)v3 = s_984c._0_2_;
    v4 = mempcpy(&v3[2],v6,v2);
    *v4 = 10;
    v4 = &v4[1 - (long)v5];
  }
  write(2,v5,(unsigned long)v4);
  if (!a0)
    _exit(dat_d020); // no-return
  raise(a0);
  abort(); // no-return
}

// Function: sub_3f40 @ 0x3f40
unsigned long sub_3f40(unsigned long a0,int a1)
{
  if (!a1)
    return 0;
  sub_3e20(0xb); // no-return
}

// Function: sub_3f60 @ 0x3f60
void sub_3f60(unsigned long a0)
{
  if ((int)a0)
    a0 = (unsigned long)(-(unsigned int)(dat_d240 == 0) & 0xb);
  sub_3e20(a0); // no-return
}

// Function: sub_3f90 @ 0x3f90
unsigned long sub_3f90(void *a0) // return-dupe
{
  if (!a0)
    a0 = sub_3e10;
  dat_d260 = a0;
  dat_d258 = dcgettext(NULL,"program error",5);
  dat_d250 = dcgettext(NULL,"stack overflow",5);
  dat_d248 = sub_4040();
  if (!sub_4ce0(sub_3f60,0xd280,0x10000)) {
    dat_d240 = sub_4c60(sub_3f40);
    return 0;
  }
  *__errno_location() = 0x5f;
  return 0xffffffff;
}

// Function: sub_4040 @ 0x4040
unsigned long sub_4040(void)
{
  return __progname;
}

// Function: sub_4050 @ 0x4050
bool sub_4050(unsigned long a0)
{
  char v1 [264];
  bool v2; // r12b
  
  v2 = 0;
  if ((!sub_48e0(a0,v1,0x101)) && (strcmp(v1,"C")))
    v2 = strcmp(v1,"POSIX") != 0;
  return v2;
}

// Function: sub_40e0 @ 0x40e0
char * sub_40e0(unsigned long a0,long a1)
{
  unsigned long v1;
  char *v2;
  char *v3; // r8
  
  *(char *)(a1 + 0x14) = 0;
  v2 = (char *)(a1 + 0x14);
  if ((long)a0 <= -1) {
    do {
      v3 = v2;
      v1 = (long)a0 / 10;
      v3[-1] = ((char)v1 * '\n' + '0') - (char)a0;
      a0 = v1;
      v2 = &v3[-1];
    } while (v1);
    v3[-2] = '-';
    return &v3[-2];
  }
  do {
    v2 = &v2[-1];
    v1 = a0 / 10;
    *v2 = (char)a0 + (char)v1 * '\xf6' + '0';
    a0 = v1;
  } while (v1);
  return v2;
}

// Function: sub_4180 @ 0x4180
void sub_4180(char *a0)
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
    dat_1d280 = a0;
    program_invocation_name = a0;
    return;
  }
  fputs("A NULL argv[0] was passed through an exec system call.\n",stderr);
  abort(); // no-return
}

// Function: sub_4220 @ 0x4220
unsigned long sub_4220(char *a0,unsigned long a1)
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
  
  v3 = (char *)sub_5740(a1,2);
  v1 = *a0;
  while ((v1 && (v5 = (char *)sub_73c0(a0,v3), v5))) {
    if (2 <= __ctype_get_mb_cur_max()) { // branch-flip
      v6[0] = 0;
      v9 = 0;
      v11 = a0;
      if (v5 <= a0) { // branch-flip
label_4349:
        v8 = (unsigned char *)0x1;
      }
      else {
        do {
          v10 = 0;
          sub_7a60(v6);
          if ((v13) && (!v14)) {
            abort(); // no-return, return-dupe
          }
          v11 = &v11[v12];
          v10 = 0;
        } while (v11 < v5);
        if (!v13) goto label_4349;
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
        sub_7a60(v7);
        if ((v19) && (!v20)) break;
        sub_7a60(v6);
        if ((v13) && (!v14))
          abort();
        v11 = &v11[v12];
        v17 = &v17[v18];
      }
      sub_7a60(v6);
      if ((((v13) && (!v14)) || ((!v13 || (!iswalnum())))) && ((char)v8)) {
        free(v3); // return-dupe
        return (unsigned long)v8 & 0xffffffff;
      }
      v11 = v5;
      v6[0] = 0;
      v9 = 0;
      v10 = 0;
      sub_7a60(v6);
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

// Function: sub_4490 @ 0x4490
char * sub_4490(char *a0)
{
  char *v1; // rax
  unsigned long v2; // rax
  char *v3; // rax
  
  v1 = dcgettext(NULL,a0,5);
  if (a0 != v1) {
    if (!sub_4220(v1,a0)) {
      v2 = strlen(v1);
      v3 = (char *)sub_6100(v2 + 4 + strlen(a0));
      __sprintf_chk(v3,1,0xffffffffffffffff,"%s (%s)",v1,a0);
      return v3;
    }
  }
  return v1;
}

// Function: sub_4520 @ 0x4520
char * sub_4520(char *a0,char *a1)
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
  v4 = (char *)sub_6c00();
  v6 = v3;
  if (sub_6b30(v4,"UTF-8")) { // branch-flip
    v7 = (char *)sub_6590(a1,"UTF-8",v4);
    v5 = strlen(v4);
    v8 = memcpy((void *)sub_6100(v5 + 0xb),v4,v5);
    v9 = (unsigned long *)(v5 + (long)v8);
    *v9 = 0x4c534e4152542f2f;
    *(unsigned short *)&v9[1] = 0x5449;
    *(char *)((long)v9 + 10) = 0;
    a1 = (char *)sub_6590(a1,"UTF-8",v8);
    free(v8);
    v11 = v7;
    if (a1) { // branch-flip
      if (strchr(a1,0x3f)) {
        free(a1);
        goto label_4723;
      }
      v4 = a1;
      if (!v7) {
        v10 = a1;
        goto label_458a;
      }
      if (!strcmp(v3,a0)) {
        if (v7 == a1)
          return v7;
        free(a1);
        return v7;
      }
    }
    else {
label_4723:
      if (!v7) {
        v11 = NULL;
        a1 = NULL;
        v10 = a0;
        v4 = NULL;
        goto label_458a;
      }
      if (!strcmp(v3,a0))
        return v7;
      a1 = NULL;
      v4 = NULL;
    }
    if (!sub_4220(v3,a0)) {
      a0 = v7;
label_45af:
      if (((!v11) || (!sub_4220(v3,v11))) && ((!a1 || (!sub_4220(v3,a1))))) goto label_462c;
      if (!v7) goto label_45dd;
    }
    free(v7);
  }
  else {
    v2 = strcmp(v3,a0);
    if (a1) {
      v10 = a1;
      v4 = NULL;
      v11 = a1;
label_458a:
      if (!strcmp(v3,a0))
        return v10;
      v1 = sub_4220(v3,a0);
      v7 = NULL;
      a0 = v10;
      if (v1) goto label_45dd;
      goto label_45af;
    }
    if (!v2)
      return a0;
    if (sub_4220(v3,a0))
      return v3;
    v4 = NULL;
    v7 = NULL;
label_462c:
    v5 = strlen(v3);
    v6 = (char *)sub_6100(v5 + 4 + strlen(a0));
    __sprintf_chk(v6,1,0xffffffffffffffff,"%s (%s)",v3,a0);
    if (v7)
      free(v7);
  }
label_45dd:
  if (v4)
    free(v4);
  return v6;
}

// Function: sub_4820 @ 0x4820
void sub_4820(int a0)
{
  setlocale(a0,NULL); // tail-call
}

// Function: sub_4830 @ 0x4830
unsigned long sub_4830(unsigned long a0,char *a1,unsigned long a2) // return-dupe x2
{
  char *v1; // rax
  unsigned long v2; // rax
  
  v1 = (char *)sub_4820();
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

// Function: sub_48e0 @ 0x48e0
void sub_48e0(void)
{
  sub_4830(); // tail-call
}

// Function: sub_48f0 @ 0x48f0
void sub_48f0(void)
{
  sub_4820(); // tail-call
}

// Function: sub_4900 @ 0x4900
void sub_4900(unsigned long a0) // return-dupe
{
  char v1 [8];
  long v2; // stack - 0x30
  
  if ((int)sub_56a0(a0,v1) <= -1)
    return;
  dat_1d2a8 = v2 + -1;
}

// Function: sub_4950 @ 0x4950
void sub_4950(unsigned long a0,long a1,long a2)
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
  if (dat_1d288) { // branch-flip
    v4 = (*dat_1d288)(v1,0);
    if (v4)
      return;
    if (dat_1d2a0) goto label_499e;
  }
  else {
    if (!dat_1d2a0) {
      signal(0xb,0); // return-dupe, tail-call
      return;
    }
label_499e:
    v3 = dat_1d2a8;
    v2 = *(unsigned long *)(a2 + 0xa0);
    if (dat_1d2a8) {
      v6 = __errno_location();
      v4 = *v6;
      v5 = sub_56a0(v3,&v7);
      *v6 = v4;
      if (0 <= v5) {
        if (v7 <= v1) { // branch-flip
          if (v1 <= (unsigned long)(v8 - 1U)) goto label_4a85;
        }
        else if ((*v9)(v1,&v7)) {
label_4a85:
          (*dat_1d2a0)(dat_1d298 <= v2 && v2 <= dat_1d298 + dat_1d290,a2);
        }
      }
    }
  }
  if ((dat_1d288) && (v4 = (*dat_1d288)(v1,1), v4))
    return;
  signal(0xb,0);
  return;
}

// Function: sub_4ae0 @ 0x4ae0
void sub_4ae0(void)
{
  void *v1; // stack - 0xa8
  char v2 [128];
  int v3; // stack - 0x20
  
  v1 = sub_4950;
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
  v3 = (-(unsigned int)(dat_1d2a0 == 0) & 0xf8000000) + 0x8000004;
  sigaction(0xb,&v1,NULL);
}

// Function: sub_4c60 @ 0x4c60
unsigned long sub_4c60(unsigned long a0)
{
  dat_1d288 = a0;
  sub_4ae0();
  return 0;
}

// Function: sub_4c80 @ 0x4c80
void sub_4c80(void)
{
  dat_1d288 = 0;
  if (dat_1d2a0)
    return;
  signal(0xb,0); // tail-call
}

// Function: sub_4cb0 @ 0x4cb0
unsigned long sub_4cb0(void *a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  (*a0)(a1,a2,a3);
  return 1;
}

// Function: sub_4ce0 @ 0x4ce0
unsigned long sub_4ce0(unsigned long a0,unsigned long a1,unsigned long a2) // return-dupe
{
  unsigned long v1; // stack - 0x48
  unsigned int v2; // stack - 0x40
  unsigned long v3; // stack - 0x38
  
  if ((!dat_1d2a8) && (sub_4900(&v1), !dat_1d2a8))
    return 0xffffffff;
  v2 = 0;
  dat_1d290 = a2;
  dat_1d298 = a1;
  dat_1d2a0 = a0;
  v1 = a1;
  v3 = a2;
  if ((int)sigaltstack(&v1,0) <= -1)
    return 0xffffffff;
  sub_4ae0();
  return 0;
}

// Function: sub_4d90 @ 0x4d90
void sub_4d90(void) // return-dupe
{
  char v1 [8];
  unsigned int v2; // stack - 0x20
  
  dat_1d2a0 = 0;
  if (dat_1d288) // branch-flip
    sub_4ae0();
  else {
    signal(0xb,0);
  }
  v2 = 2;
  if (0 <= (int)sigaltstack(v1,0))
    return;
  perror("gnulib sigsegv (stackoverflow_deinstall_handler)");
}

// Function: sub_4e20 @ 0x4e20
bool sub_4e20(long a0,long *a1)
{
  return (unsigned long)(*a1 - a0) <= (unsigned long)(*a1 - a1[3]) >> 1;
}

// Function: sub_4e40 @ 0x4e40
unsigned long sub_4e40(long *a0)
{
  if (*a0 != a0[1])
    return (unsigned long)*(unsigned char *)(a0[3] + *a0);
  *(unsigned int *)&a0[2] = 1;
  return 0xffffffff;
}

// Function: sub_4e60 @ 0x4e60
void sub_4e60(long *a0) // return-dupe
{
  if ((int)sub_4e40() <= -1)
    return;
  *a0 = *a0 + 1;
}

// Function: sub_4e70 @ 0x4e70
unsigned long sub_4e70(unsigned long a0,long *a1)
{
  int v1; // eax
  long v2; // rcx
  int v3; // esi
  
  v2 = 0;
  v3 = 0;
  do {
    v1 = sub_4e40();
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
    sub_4e60();
    v3 += 1;
  } while( true );
}

// Function: sub_4ee0 @ 0x4ee0
unsigned long sub_4ee0(unsigned long *a0,unsigned long a1,unsigned long a2)
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

// Function: sub_4f20 @ 0x4f20
void sub_4f20(void)
{
  dat_1d2b0 = (long)getpagesize();
}

// Function: sub_4f40 @ 0x4f40
void sub_4f40(long a0)
{
  if (*(void **)(a0 + 0x20)) {
    munmap(*(void **)(a0 + 0x20),*(unsigned long *)(a0 + 0x28)); // tail-call
    return;
  }
}

// Function: sub_4f60 @ 0x4f60
unsigned int sub_4f60(unsigned long a0)
{
  unsigned int v1; // eax
  long v2;
  
  v1 = mincore(a0,dat_1d2b0,&v2);
  return ~v1 >> 0x1f;
}

// Function: sub_4fb0 @ 0x4fb0
unsigned long sub_4fb0(unsigned long a0,unsigned long a1) // return-dupe
{
  unsigned long v1;
  long v2; // rax
  unsigned long v3; // rax
  unsigned long v4;
  long v5;
  unsigned long v6; // r14
  unsigned long v7;
  
  v1 = dat_1d2b0;
  v2 = (a0 / dat_1d2b0) * dat_1d2b0;
  v3 = ((a1 / dat_1d2b0 + 1) * dat_1d2b0 - v2) / dat_1d2b0;
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
        if (sub_4f60(v5))
          return 0;
        v7 += v4 & 0xfffffffffffffffe;
        v5 += v1 * v6 * 2;
      } while (v7 < v3);
    }
    v4 = v6;
  } while (2 <= v6);
  return 1;
}

// Function: sub_50a0 @ 0x50a0
unsigned long sub_50a0(unsigned long a0,long *a1)
{
  long v1;
  unsigned long v2; // rdi
  
  v1 = *a1;
  v2 = a0 * 2 - v1;
  if (a0 < v2)
    return 0;
  return sub_4fb0(v2,v1 + -1); // tail-call
}

// Function: sub_50d0 @ 0x50d0
unsigned long sub_50d0(unsigned long a0)
{
  unsigned long v1;
  unsigned long v2;
  char v3 [1032];
  long v4; // rsi
  unsigned long v5; // r14
  unsigned long v6; // r15
  
  v2 = 0x400;
  while( true ) {
    v1 = dat_1d2b0;
    if (!a0)
      return 0;
    if (a0 / dat_1d2b0 < v2)
      v2 = a0 / dat_1d2b0;
    v4 = dat_1d2b0 * v2;
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

// Function: sub_51b0 @ 0x51b0
long sub_51b0(long a0) // return-dupe
{
  unsigned long v1;
  unsigned long v2;
  unsigned long v3;
  char v4 [1032];
  
  v1 = dat_1d2b0;
  a0 += dat_1d2b0;
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

// Function: sub_52b0 @ 0x52b0
unsigned long sub_52b0(unsigned long a0,unsigned long *a1)
{
  long v1; // rbp
  
  if (!dat_1d2b0)
    sub_4f20();
  v1 = (a0 / dat_1d2b0) * dat_1d2b0;
  *a1 = sub_50d0(v1);
  a1[1] = sub_51b0(v1);
  a1[2] = sub_50a0;
  return 0;
}

// Function: sub_5320 @ 0x5320
unsigned long sub_5320(unsigned long *a0) // return-dupe x2
{
  void *v1;
  int v2; // eax
  int v3; // eax
  long v4;
  unsigned long v5; // rax
  unsigned long v6; // r12
  unsigned long v7;
  
  v2 = open("/proc/self/maps",0);
  if (v2 <= -1)
    return 0xffffffff;
  *a0 = 0;
  v7 = 1;
  *(unsigned int *)&a0[2] = 0;
  v6 = 0;
  a0[3] = &a0[8];
  a0[4] = 0;
  a0[6] = 0;
  a0[7] = 0;
  do {
    if (0x104a <= v7) { // branch-flip
      v1 = (void *)a0[3];
      v3 = (int)read(v2,v1,v7);
      if (0 <= v3) {
        if (!v3) goto label_54ec;
        if (v7 < (unsigned long)(long)(v3 + 0x1049)) goto label_53bf;
        v4 = (long)v3;
        while( true ) {
          a0[1] = v4;
          v5 = read(v2,(void *)(v4 + a0[3]),v7 - v4);
          v3 = (int)v5;
          if (v3 <= -1) break;
          if (v7 - a0[1] < (unsigned long)(long)(v3 + 0x1049)) goto label_53bf;
          if (!v3) {
            close(v2);
            return v5 & 0xffffffff;
          }
          v4 = (long)v3 + a0[1];
        }
      }
      if (*__errno_location() != 4) {
label_54ec:
        close(v2);
        v1 = (void *)a0[4];
        goto label_54fa;
      }
    }
    else {
label_53bf:
      if (v6) { // branch-flip
        v7 *= 2;
        if (!v7) goto label_54ec;
        if ((void *)a0[4])
          munmap((void *)a0[4],a0[5]);
      }
      else {
        v6 = (unsigned long)getpagesize();
        for (v7 = v6; v7 <= 0x1049; v7 = v7 * 2) {
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
      a0[7] = v7 + v4;
      a0[3] = v4;
    }
    if ((long)lseek(v2,0,0) <= -1) {
      close(v2);
      v2 = open("/proc/self/maps",0);
      if (v2 <= -1) {
        v1 = (void *)a0[4];
label_54fa:
        if (!v1)
          return 0xffffffff;
        munmap(v1,a0[5]);
        return 0xffffffff;
      }
    }
  } while( true );
}

// Function: sub_5560 @ 0x5560
unsigned long sub_5560(unsigned long a0) // return-dupe
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
  if ((int)sub_5320(v4) <= -1)
    return 0xffffffff;
  v9 = &v5;
  v7 = v10;
label_55b0:
  do {
    v2 = sub_4e70(v4,v8);
    if (((v2 < 0) || (sub_4e60() != 0x2d)) || (v2 = sub_4e70(v9), v2 < 0)) {
      sub_4f40(v4); // return-dupe
      return 0;
    }
    do {
      v2 = sub_4e60(v4);
      if (v2 == -1) break;
    } while (v2 != 10);
    if (v3 <= v7) { // branch-flip
      v1 = v5 - 1;
      if (v1 < (unsigned long)(v11 - 1U)) goto label_5650;
      v6 = v5;
      if ((v3 < v7) && (v2 = sub_4ee0(a0,v3,v7), v2)) {
        sub_4f40(v4);
        return 0;
      }
      if (v1 <= (unsigned long)(v11 - 1U)) goto label_55b0;
      v2 = sub_4ee0(a0,v11,v6);
    }
    else {
label_5650:
      v2 = sub_4ee0(a0,v3,v5);
    }
    if (v2) {
      sub_4f40(v4);
      return 0;
    }
  } while( true );
}

// Function: sub_5690 @ 0x5690
void sub_5690(void)
{
  sub_5560(); // tail-call
}

// Function: sub_56a0 @ 0x56a0
unsigned long sub_56a0(unsigned long a0,long a1)
{
  unsigned long v1; // stack - 0x48
  long v2; // stack - 0x40
  unsigned long v3; // stack - 0x38
  int v4; // stack - 0x30
  
  v3 = 0;
  v4 = -1;
  v1 = a0;
  v2 = a1;
  sub_5690(&v1);
  if (!v4) {
    *(void **)(a1 + 0x10) = sub_4e20;
    return 0;
  }
  return sub_52b0(a0,a1); // tail-call
}

// Function: sub_5740 @ 0x5740
unsigned char * sub_5740(char *a0,int a1)
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
    sub_64b0(); // no-return
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
          sub_6c40(&v6);
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
        sub_6c40(&v6);
        if (v7) { // branch-flip
          if (v7 != 1) { // branch-flip
            if ((v7 != 2) || (!v14)) goto label_57cb;
            v3 = iswspace(v15);
            if (!v3)
              v7 = 1;
          }
          else if ((((v14) && (v3 = iswspace(v15), v3)) && (v14)) && (v3 = iswspace(v15), v3)) {
            v7 = 2;
            v8 = v12;
          }
          else {
label_590f:
          }
        }
        else {
          if (v14) {
            v3 = iswspace(v15);
            if (v3) goto label_590f;
            if (v14) {
              v7 = 1;
              iswspace(v15);
              goto label_57d5;
            }
          }
label_57cb:
          v7 = 1;
        }
label_57d5:
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

// Function: sub_5a50 @ 0x5a50
long sub_5a50(void *a0,long a1,unsigned long a2,unsigned long a3,unsigned long *a4,unsigned long a5)
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
label_5b7d:
    v1 = CONCAT44(dat_4,__fprintf_chk(a0,1,v2,v4,v3,v5,v6,v7,v8,v9,v10));
label_5bb6:
    return v1;
  }
  v1 = (long)*(int *)(a5 * 4 + 0x9c18) + 0x9c18;
  switch(a5) {
    case 0:
      goto label_5bb6;
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
      goto label_5b7d;
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

// Function: sub_5ed0 @ 0x5ed0
void sub_5ed0(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,long *a4)
{
  long v1;
  long v2;
  
  v2 = 0;
  v1 = *a4;
  while (v1) {
    v2 += 1;
    v1 = a4[v2];
  }
  sub_5a50(); // tail-call
}

// Function: sub_5ef0 @ 0x5ef0
void sub_5ef0(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned int *a4)
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
  sub_5a50(a0,a1,a2,a3);
}

// Function: sub_5f90 @ 0x5f90
void sub_5f90(void)
{
  sub_5ef0();
}

// Function: sub_6040 @ 0x6040
void sub_6040(void)
{
  fputs_unlocked("\n",stdout);
  __printf_chk(1,dcgettext(NULL,"Report bugs to: %s\n",5),"bug-diffutils@gnu.org");
  __printf_chk(1,dcgettext(NULL,"%s home page: <%s>\n",5),"GNU diffutils","https://www.gnu.org/software/diffutils/");
  __printf_chk(1,dcgettext(NULL,"General help using GNU software: <%s>\n",5),"https://www.gnu.org/gethelp/"); // tail-call
}

// Function: sub_60f0 @ 0x60f0
long sub_60f0(long a0)
{
  if (a0)
    return a0;
  sub_64b0(); // no-return
}

// Function: sub_6100 @ 0x6100
void sub_6100(unsigned long a0)
{
  sub_60f0(malloc(a0)); // tail-call
}

// Function: sub_6120 @ 0x6120
void sub_6120(void)
{
  sub_6100(); // tail-call
}

// Function: sub_6130 @ 0x6130
void sub_6130(void)
{
  sub_60f0(sub_6b90()); // tail-call
}

// Function: sub_6150 @ 0x6150
void sub_6150(void *a0,unsigned long a1)
{
  if ((!realloc(a0,a1)) && ((!a0 || (a1))))
    sub_64b0(); // no-return
}

// Function: sub_6190 @ 0x6190
void sub_6190(void)
{
  sub_60f0(sub_6ba0()); // tail-call
}

// Function: sub_61b0 @ 0x61b0
void sub_61b0(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!reallocarray(a0,a1,a2)) && ((!a0 || ((a1 && (a2))))))
    sub_64b0(); // no-return
}

// Function: sub_61f0 @ 0x61f0
void sub_61f0(unsigned long a0,unsigned long a1)
{
  sub_61b0(0,a0,a1); // tail-call
}

// Function: sub_6200 @ 0x6200
void sub_6200(void)
{
  sub_61b0(); // tail-call
}

// Function: sub_6210 @ 0x6210
void sub_6210(void)
{
  sub_60f0(sub_6bd0()); // tail-call
}

// Function: sub_6230 @ 0x6230
void sub_6230(long a0,unsigned long *a1,unsigned long a2)
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
      sub_64b0(); // no-return
  }
  else if (!v3) {
    v1._8_8_ = 0;
    v1._0_8_ = a2;
    v3 = SUB168((ZEXT816(0) << 0x40 | ZEXT816(0x80)) / v1._0_16_,0) + (unsigned long)(0x81 <= a2);
  }
  sub_61b0(a0,v3,a2);
  *a1 = v3;
}

// Function: sub_62a0 @ 0x62a0
void sub_62a0(unsigned long a0,unsigned long a1)
{
  sub_6230(a0,a1,1); // tail-call
}

// Function: sub_62b0 @ 0x62b0
void sub_62b0(long a0,long *a1,long a2,long a3,long a4)
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
    if (0x80 <= v3) goto label_6356;
    v3 = 0x80;
  }
  v2 = v3 / a4;
  v3 -= v3 % a4;
label_6356:
  if (!a0)
    *a1 = 0;
  if ((v2 - v1 < a2) && ((v2 = v1 + a2, SCARRY8(v1,a2) || (((a3 < v2 && (0 <= a3)) || (v3 = v2 * a4, SEXT816(v3) != SEXT816(v2) * SEXT816(a4)))))))
    sub_64b0(a0,v3); // no-return
  sub_6150(a0,v3);
  *a1 = v2;
}

// Function: sub_6390 @ 0x6390
void sub_6390(unsigned long a0,unsigned long a1)
{
  sub_60f0(calloc(a0,a1)); // tail-call
}

// Function: sub_63b0 @ 0x63b0
void sub_63b0(unsigned long a0)
{
  sub_6390(a0,1); // tail-call
}

// Function: sub_63c0 @ 0x63c0
void sub_63c0(void)
{
  sub_60f0(sub_6bc0()); // tail-call
}

// Function: sub_63e0 @ 0x63e0
void sub_63e0(unsigned long a0)
{
  sub_63c0(a0,1); // tail-call
}

// Function: sub_63f0 @ 0x63f0
void sub_63f0(void *a0,unsigned long a1)
{
  memcpy((void *)sub_6100(a1),a0,a1); // tail-call
}

// Function: sub_6420 @ 0x6420
void sub_6420(void *a0,unsigned long a1)
{
  memcpy((void *)sub_6130(a1),a0,a1); // tail-call
}

// Function: sub_6450 @ 0x6450
void sub_6450(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)sub_6130(a1 + 1);
  *(char *)((long)v1 + a1) = 0;
  memcpy(v1,a0,a1); // tail-call
}

// Function: sub_6490 @ 0x6490
void sub_6490(char *a0)
{
  sub_63f0(a0,strlen(a0) + 1); // tail-call
}

// Function: sub_64b0 @ 0x64b0
void sub_64b0(void)
{
  error(dat_d020,0,"%s",dcgettext(NULL,"memory exhausted",5));
  abort(); // no-return
}

// Function: sub_64f0 @ 0x64f0
void sub_64f0(void)
{
  int v1; // eax
  
  v1 = sub_7c30();
  if (!v1)
    return;
  error(dat_d020,v1,dcgettext(NULL,"standard file descriptors",5)); // tail-call
}

// Function: sub_6530 @ 0x6530
int sub_6530(void)
{
  int v1; // eax
  
  v1 = sub_7ce0();
  if (v1 < 0) {
    if (*__errno_location() == 0xc)
      sub_64b0(); // no-return
  }
  return v1;
}

// Function: sub_6560 @ 0x6560
long sub_6560(void)
{
  long v1; // rax
  
  v1 = sub_7f60();
  if (!v1) {
    if (*__errno_location() == 0xc)
      sub_64b0(); // no-return
  }
  return v1;
}

// Function: sub_6590 @ 0x6590
long sub_6590(void)
{
  long v1; // rax
  
  v1 = sub_81b0();
  if (!v1) {
    if (*__errno_location() == 0xc)
      sub_64b0(); // no-return
  }
  return v1;
}

// Function: sub_65c0 @ 0x65c0
unsigned long sub_65c0(long *a0,int a1)
{
  long v1;
  char v2 [16];
  long v3; // rcx
  long v4; // rsi
  unsigned long v5; // r9
  
  v3 = -0x8000000000000000;
  v4 = (long)a1;
  v1 = *a0;
  v5 = 1;
  v2._8_8_ = 0;
  v2._0_8_ = SUB168(SEXT816(-0x8000000000000000),8);
  if ((SUB168((v2._0_16_ << 0x40 | ZEXT816(0x8000000000000000)) / SEXT816(v4),0) <= v1) && (v3 = 0x7fffffffffffffff, v1 <= 0x7fffffffffffffff / v4)) {
    v5 = 0;
    v3 = v1 * v4;
  }
  *a0 = v3;
  return v5;
}

// Function: sub_6610 @ 0x6610
unsigned int sub_6610(unsigned long a0,unsigned int a1,int a2)
{
  unsigned int v1; // eax
  unsigned int v2; // r11d
  
  v2 = 0;
  do {
    a2 -= 1;
    v1 = sub_65c0(a0,a1);
    v2 |= v1;
  } while (a2);
  return v2;
}

// Function: sub_6640 @ 0x6640
unsigned int sub_6640(unsigned char *a0,unsigned long *a1,unsigned int a2,unsigned long *a3,char *a4)
{
  unsigned char v1;
  unsigned int v10;
  unsigned char v2;
  unsigned char *v3;
  unsigned int v4; // eax
  int *v5; // rax
  long v6;
  unsigned long v7; // stack - 0x48
  int v8; // ecx
  unsigned long v9; // r8
  
  if (0x25 <= a2)
    __assert_fail("0 <= strtol_base && strtol_base <= 36","xstrtol.c",0x54,"xstrtoimax"); // no-return
  if (!a1)
    a1 = &v6;
  v5 = __errno_location();
  *v5 = 0;
  v7 = strtoimax(a0,a1,a2);
  v3 = (unsigned char *)*a1;
  if (v3 != a0) { // branch-flip
    if (*v5) { // branch-flip
      if (*v5 != 0x22)
        return 4;
      v10 = 1;
    }
    else {
      v10 = 0;
    }
    if ((!a4) || (v1 = *v3, !v1)) goto label_66cc;
    v8 = (int)(char)v1;
  }
  else {
    if (!a4)
      return 4;
    v1 = *v3;
    if (!v1)
      return 4;
    v8 = (int)(char)v1;
    if (!strchr(a4,v8))
      return 4;
    v7 = 1;
    v10 = 0;
  }
  if (strchr(a4,v8)) {
    v4 = (unsigned int)v1 - 0x45;
    v8 = 1;
    v9 = 0x400;
    if (((unsigned char)v4 <= 0x2f) && (0x814400308945U >> ((unsigned long)v4 & 0x3f) & 1)) {
      v9 = 0x400;
      v8 = 1;
      if (strchr(a4,0x30)) {
        v2 = v3[1];
        if (v2 != 0x44) { // branch-flip
          if (v2 != 0x69) { // branch-flip
            v8 = (v2 == 0x42) + 1;
            v9 = 0x400;
            if (v2 == 0x42)
              v9 = 1000;
          }
          else {
            v8 = (v3[2] == 0x42) + 1 + (unsigned int)(v3[2] == 0x42);
          }
        }
        else {
          v8 = 2;
          v9 = 1000;
        }
      }
    }
    switch((unsigned int)v1 - 0x42 & 0xff) {
      case 0:
        v10 |= sub_65c0(&v7,0x400);
        break;
      default:
        goto label_66cf;
      case 3:
        v10 |= sub_6610(&v7,v9,6);
        break;
      case 5:
      case 0x25:
        v10 |= sub_6610(&v7,v9,3);
        break;
      case 9:
      case 0x29:
        v10 |= sub_6610(&v7,v9,1);
        break;
      case 0xb:
      case 0x2b:
        v10 |= sub_6610(&v7,v9,2);
        break;
      case 0xe:
        v10 |= sub_6610(&v7,v9,5);
        break;
      case 0x12:
      case 0x32:
        v10 |= sub_6610(&v7,v9,4);
        break;
      case 0x17:
        v10 |= sub_6610(&v7,v9,8);
        break;
      case 0x18:
        v10 |= sub_6610(&v7,v9,7);
        break;
      case 0x20:
        v10 |= sub_65c0(&v7,0x200);
        break;
      case 0x21:
        break;
      case 0x35:
        v10 |= sub_65c0(&v7,2);
      
    }
    *a1 = &v3[v8];
    if (v3[v8])
      v10 |= 2;
label_66cc:
    *a3 = v7;
    return v10;
  }
label_66cf:
  *a3 = v7;
  return v10 | 2;
}

// Function: sub_6a10 @ 0x6a10
long sub_6a10(int a0,void *a1,long a2)
{
  void *v1;
  unsigned long v2;
  long v3; // rax
  unsigned long v4; // r14
  void *v5;
  
  v4 = 0x7fffffffffffffff;
  v1 = (void *)((long)a1 + a2);
  v5 = a1;
  do {
    while( true ) {
      v2 = (long)v1 - (long)v5;
      if (v4 < (unsigned long)((long)v1 - (long)v5))
        v2 = v4;
      v3 = read(a0,v5,v2);
      if (0 < v3) break;
      if (!v3) {
        return (long)v5 - (long)a1;
      }
      if ((*__errno_location() != 0x16) || (v2 < 0x80000000))
        return -1;
      v4 = 0x7fffffff;
      if (v1 <= v5) {
        return (long)v5 - (long)a1;
      }
    }
    v5 = (void *)((long)v5 + v3);
  } while (v5 < v1);
  return (long)v5 - (long)a1;
}

// Function: sub_6ac0 @ 0x6ac0
void * sub_6ac0(void *a0,void *a1,void *a2)
{
  char v1 [16];
  char v2 [16];
  char v3 [16];
  char v4 [16];
  void *v5;
  void *v6; // rdx
  void *v7;
  
  if (a0) {
    if (a1) {
      v5 = a0;
      v7 = a1;
      while (v6 = (void *)((unsigned long)v5 % (unsigned long)v7), v6) {
        v5 = v7;
        v7 = v6;
      }
      v1._8_8_ = 0;
      v1._0_8_ = v7;
      v2._8_8_ = 0;
      v2._0_8_ = a0;
      v3._8_8_ = 0;
      v3._0_8_ = a1;
      v4._8_8_ = 0;
      v4._0_8_ = SUB168(v2._0_16_ / v1._0_16_,0);
      v5 = SUB168(v3._0_16_ * v4._0_16_,0);
      if ((v5 <= a2) && (!SUB168(v3._0_16_ * v4._0_16_,8)))
        a0 = v5;
    }
    return a0;
  }
  v5 = _DT_INIT;
  if (a1)
    v5 = a1;
  return v5;
}

// Function: sub_6b30 @ 0x6b30
int sub_6b30(char *a0,char *a1) // early-return
{
  unsigned char v1; // al
  unsigned int v2; // eax
  
  if (a0 == a1)
    return 0;
  do {
    v2 = sub_8710(*a0);
    v1 = sub_8710(*a1);
    if (!(unsigned char)v2) break;
    a0 = &a0[1];
    a1 = &a1[1];
  } while ((unsigned char)v2 == v1);
  return (v2 & 0xff) - (unsigned int)v1;
}

// Function: sub_6b90 @ 0x6b90
void sub_6b90(unsigned long a0)
{
  malloc(a0); // tail-call
}

// Function: sub_6ba0 @ 0x6ba0
void sub_6ba0(void *a0,unsigned long a1)
{
  realloc(a0,a1 | a1 == 0); // tail-call
}

// Function: sub_6bc0 @ 0x6bc0
void sub_6bc0(unsigned long a0,unsigned long a1)
{
  calloc(a0,a1); // tail-call
}

// Function: sub_6bd0 @ 0x6bd0
void sub_6bd0(void *a0,unsigned long a1,unsigned long a2)
{
  if ((a1) && (a2)) {
    reallocarray(a0,a1,a2); // tail-call
    return;
  }
  reallocarray(a0,1,1); // tail-call
}

// Function: sub_6c00 @ 0x6c00
char * sub_6c00(void) // early-return
{
  char *v1; // rax
  
  v1 = nl_langinfo(0xe);
  if (!v1)
    return "ASCII";
  if (*v1)
    return v1;
  return "ASCII";
}

// Function: sub_6c40 @ 0x6c40
void sub_6c40(long *a0)
{
  char v1;
  char *v2;
  long v3; // rax
  
  if (*(char *)((long)a0 + 0x14))
    return;
  v2 = (char *)a0[3];
  if (!(char)a0[1]) {
    v1 = *v2;
    if (sub_8860((int)v1)) {
      a0[4] = 1;
      v1 = *v2;
      *(char *)&a0[5] = 1;
      *(int *)((long)a0 + 0x2c) = (int)v1;
      goto label_6c83;
    }
    if (!mbsinit((void *)((long)a0 + 0xc)))
      __assert_fail("mbsinit (&iter->state)","mbiter.h",0x87,"mbiter_multi_next"); // no-return
    *(char *)&a0[1] = 1;
  }
  v3 = sub_84f0((long)a0 + 0x2c,v2,*a0 - (long)v2,(void *)((long)a0 + 0xc));
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
label_6c83:
  *(char *)((long)a0 + 0x14) = 1;
  return;
}

// Function: sub_6db0 @ 0x6db0
void sub_6db0(long *a0,long a1)
{
  a0[3] = a0[3] + a1;
  *a0 = *a0 + a1;
}

// Function: sub_6dc0 @ 0x6dc0
void sub_6dc0(unsigned long *a0,unsigned long *a1)
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
  sub_8800(&a0[3],&a1[3]); // tail-call
}

// Function: sub_6e10 @ 0x6e10
unsigned long sub_6e10(char *a0,long a1,unsigned long a2,unsigned long *a3)
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
      v9 = sub_8750(v9);
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
              goto label_6f08;
            }
            v3 -= *(long *)(v9 + v3 * 8);
            v2 = *(char *)(a1 + v3);
          }
          v3 += 1;
          *(unsigned long *)(v9 + v1 * 8) = v1 - v3;
label_6f08:
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
      *(unsigned long *)&v8[-8] = 0x6f5e;
      sub_87a0(v9);
      return 1;
    }
  }
  return 0;
}

// Function: sub_6fb0 @ 0x6fb0
char sub_6fb0(void *a0,void *a1,long *a2)
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
  v24 = (void *)sub_8880(a1);
  v2._8_8_ = 0;
  v2._0_8_ = v24;
  if ((0 <= SUB168(ZEXT816(0x38) * v2._0_16_,0)) && (!SUB168(ZEXT816(0x38) * v2._0_16_,8))) {
    v18 = (long)v24 * 0x38;
    if (0xfa1 <= v18) { // branch-flip
      v21 = (unsigned long *)sub_8750(v18);
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
        *(unsigned long *)((long)v17 + -8) = 0x712f;
        sub_7a60(v26);
        v23 = v36;
        if ((v36) && (!v37)) break;
        v9 = &v7[6];
        *(unsigned long *)((long)v17 + -8) = 0x7117;
        sub_8800(v7,&v11);
        v11 = (void *)((long)v11 + v35);
        v7 = v9;
      }
      v19 = 0;
      v21[(long)v4 * 6 + 1] = 1;
      v25 = (void *)0x2;
      v7 = v21;
      if ((void *)0x3 <= v24) {
label_7170:
        v9 = &v7[6];
        v1 = *(char *)&v7[8];
        do {
          v6 = &v21[v19 * 6];
          if ((v1) && (*(char *)&v6[2])) {
            if (*(int *)((long)v7 + 0x44) == *(int *)((long)v6 + 0x14)) {
label_71c5:
              v19 += 1;
              v21[(long)((long)v4 * 6 + (long)v25)] = (long)v25 - v19;
              goto label_71da;
            }
          }
          else {
            v18 = v7[7];
            if (v18 == v6[1]) {
              v20 = (void *)*v6;
              v8 = (void *)*v9;
              *(unsigned long *)((long)v17 + -8) = 0x71c1;
              v3 = *(char *)((long)v17 + -8);
              if (!memcmp(v8,v20,v18,v3)) goto label_71c5;
            }
          }
          if (!v19) goto label_73a0;
          v19 -= v21[(long)v4 * 6 + v19];
        } while( true );
      }
label_71fa:
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
          *(unsigned long *)((long)v17 + -8) = 0x725c;
          sub_7a60(v26);
          v18 = v35;
          if (!v36) break;
          if (!v37) goto label_7097;
          v7 = &v21[(long)v20 * 6];
          if (!*(char *)&v7[2]) goto label_72cb;
          if (*(int *)((long)v7 + 0x14) == v37) {
            v8 = v11;
            goto label_7287;
          }
label_734b:
          if (v20) { // branch-flip
            v19 = v21[(long)((long)v4 * 6 + (long)v20)];
            v20 = (void *)((long)v20 - v19);
            for (; v19; v19 = v19 + -1) {
              *(unsigned long *)((long)v17 + -8) = 0x7387;
              sub_7a60(v13);
              if ((v31) && (!v32)) goto label_739a;
              v28 = 0;
              v29 = (void *)((long)v29 + v30);
            }
          }
          else {
            *(unsigned long *)((long)v17 + -8) = 0x72e2;
            sub_7a60(v13);
            if ((v31) && (!v32)) {
label_739a:
              *(unsigned long *)((long)v17 + -8) = 0x739f; // no-return
              abort(*(char *)((long)v17 + -8));
            }
            v28 = 0;
            v29 = (void *)((long)v29 + v30);
            v34 = 0;
            v11 = (void *)((long)v11 + v35);
          }
        }
        v7 = &v21[(long)v20 * 6];
label_72cb:
        if (v7[1] != v35) goto label_734b;
        v8 = (void *)*v7;
        v25 = v11;
        *(unsigned long *)((long)v17 + -8) = 0x7339;
        v5 = memcmp(v8,v11,v35,*(char *)((long)v17 + -8));
        v8 = v25;
        if (v5) goto label_734b;
label_7287:
        v11 = (void *)((long)v8 + v18);
        v34 = 0;
        v20 = (void *)((long)v20 + 1);
      } while (v24 != v20);
      *v22 = (long)v29;
label_7097:
      *(unsigned long *)((long)v17 + -8) = 0x72b8;
      sub_87a0(v21);
      return v23;
    }
  }
  return '\0';
label_73a0:
  v21[(long)((long)v4 * 6 + (long)v25)] = v25;
label_71da:
  v25 = (void *)((long)v25 + 1);
  v7 = v9;
  if (v24 == v25) goto label_71fa;
  goto label_7170;
}

// Function: sub_73c0 @ 0x73c0
char * sub_73c0(char *a0,char *a1) // early-return, return-dupe x4
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
    sub_7a60(v12);
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
        sub_7a60(&v15);
        if ((char)v39) { // branch-flip
          if (!v40)
            return NULL;
          v4 = 10 <= v19 & v20;
          if (v4) {
            if (v19 * 5 <= v9) { // branch-flip
label_7940:
              v17 = v9 - v21;
              while ((v17 && ((sub_7a60(v16), !v32 || (v33))))) {
                v29 = 0;
                v30 = &v30[v31];
                v17 -= 1;
              }
              sub_7a60(v16);
              v20 = v32;
              if (v32) { // branch-flip
                if (!v33) {
                  v20 = sub_6fb0(a0,a1,&v11);
                  if (v20)
                    return v11;
                }
              }
              else {
                v20 = 1;
              }
              v21 = v9;
              if (!(char)v39) goto label_7716;
            }
            else {
              v20 = v4;
            }
          }
          v10 = v9 + 1;
          if (!v26) goto label_7716;
          if (v40 == v27) goto label_7756;
label_76a8:
          v7 = v37;
          v18 = v38;
        }
        else {
          v4 = 10 <= v19 & v20;
          if (v4) { // branch-flip
            if (v19 * 5 <= v9) goto label_7940;
            v20 = v4;
          }
          else {
          }
label_7716:
          v18 = v38;
          v7 = v37;
          v10 = v9 + 1;
          if (v38 == v25) {
            v6 = memcmp(v37,v24,v38);
            if (!v6) {
label_7756:
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
              sub_7a60(v13);
              if ((v56) && (!v57))
                abort(); // no-return
              v54 = &v54[v55];
              do {
                v10 += 1;
                v53 = 0;
                sub_7a60(v13);
                if ((v56) && (!v57))
                  return v37;
                sub_7a60(&v14);
                v18 = v55;
                v1 = v54;
                v9 = v47;
                v7 = v46;
                if ((char)v48) { // branch-flip
                  if (!v48._4_4_)
                    return NULL;
                  if (!v56) goto label_78c3;
                  if (v48._4_4_ != v57) goto label_76a8;
                }
                else {
label_78c3:
                  if ((v47 != v55) || (v6 = memcmp(v46,v54,v47), v6)) goto label_76a8;
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
              if (*v7) goto label_74ed;
            }
            v5 = sub_6e10(v1,a1,strlen(a1),&v11);
            if ((char)v5)
              return v11;
            v7 = NULL;
          }
          else {
label_74ed:
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

// Function: sub_7a60 @ 0x7a60
void sub_7a60(char *a0)
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
    if (sub_8860((int)v1)) {
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
      goto label_7aa2;
    }
    if (!mbsinit(&a0[4]))
      __assert_fail("mbsinit (&iter->state)","mbuiter.h",0x8f,"mbuiter_multi_next"); // no-return
    *a0 = '\x01';
  }
  v4 = __ctype_get_mb_cur_max();
  v3 = *(unsigned long *)&a0[0x10];
  v5 = sub_84f0(&a0[0x24],v3,sub_8270(v3,v4),&a0[4]);
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
label_7aa2:
  a0[0xc] = '\x01';
  return;
}

// Function: sub_7be0 @ 0x7be0
void sub_7be0(long a0,long a1)
{
  *(long *)(a0 + 0x10) = *(long *)(a0 + 0x10) + a1;
}

// Function: sub_7bf0 @ 0x7bf0
void sub_7bf0(char *a0,char *a1)
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
  sub_8800(&a0[0x10],&a1[0x10]); // tail-call
}

// Function: sub_7c30 @ 0x7c30
int sub_7c30(void) // early-return x2
{
  int v1; // eax
  int v2; // ebx
  int v3; // ebp
  
  v3 = 1;
  v2 = 0;
  do {
    v1 = sub_83a0(v2,1);
    if (0 <= v1) { // branch-flip
label_7c63:
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
        goto label_7c63;
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

// Function: sub_7ce0 @ 0x7ce0
unsigned long sub_7ce0(unsigned long a0,long a1,unsigned long a2,long *a3,unsigned long *a4) // return-dupe
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
label_7e82:
      if (iconv(a2,0,0,&v5,&v4) != -1) {
        if (v4)
          abort(); // no-return
        *a3 = (long)v3;
        *a4 = (unsigned long)v2;
        return 0;
      }
label_7f2e:
      if ((char *)*a3 != v3) {
        free(v3);
        return 0xffffffff;
      }
      return 0xffffffff;
    }
    if (iconv(a2,&v7,&v8,&v5,&v4) == -1) {
      if (*__errno_location() != 0x16) goto label_7f2e;
      goto label_7e82;
    }
    a1 = v8;
  } while( true );
}

// Function: sub_7f60 @ 0x7f60
char * sub_7f60(char *a0,unsigned long a1) // return-dupe
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
    if (*v4 == 0x16) goto label_80fd;
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
label_80fd:
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

// Function: sub_81b0 @ 0x81b0
char * sub_81b0(char *a0,unsigned long a1,unsigned long a2)
{
  int v1;
  char *v2; // rax
  long v3; // rax
  int *v4; // rax
  
  if ((!*a0) || (!sub_6b30(a1,a2))) {
    v2 = strdup(a0);
    if (!v2)
      *__errno_location() = 0xc;
    return v2;
  }
  v3 = iconv_open(a2,a1);
  if (v3 == -1)
    return NULL;
  v2 = (char *)sub_7f60(a0,v3);
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

// Function: sub_8270 @ 0x8270
void * sub_8270(void *a0,unsigned long a1)
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

// Function: sub_82b0 @ 0x82b0
void sub_82b0(int a0,unsigned long a1)
{
  fcntl(a0,0,a1 & 0xffffffff); // tail-call
}

// Function: sub_82c0 @ 0x82c0
int sub_82c0(unsigned long a0,unsigned long a1)
{
  int v1;
  int v2; // eax
  unsigned int v3; // eax
  int *v4; // rax
  
  if (0 <= dat_1d2b8) { // branch-flip
    v2 = fcntl((int)a0,0x406,a1 & 0xffffffff);
    if ((0 <= v2) || (*__errno_location() != 0x16)) {
      dat_1d2b8 = 1;
      return v2;
    }
    v2 = sub_82b0(a0 & 0xffffffff,a1 & 0xffffffff);
    if (v2 < 0)
      return v2;
    dat_1d2b8 = -1;
  }
  else {
    v2 = sub_82b0(a0,a1);
    if (v2 < 0)
      return v2;
    if (dat_1d2b8 != -1)
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

// Function: sub_83a0 @ 0x83a0
void sub_83a0(int a0,int a1,unsigned int a2)
{
  unsigned long v1;
  unsigned long v2; // stack - 0x28
  
  v2 = CONCAT44(v2._4_4_,a2);
  if (!a1) {
    sub_82b0(a0,a2);
    return;
  }
  if (a1 == 0x406) {
    sub_82c0(a0,a2);
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

// Function: sub_84f0 @ 0x84f0
unsigned long sub_84f0(unsigned int *a0,unsigned char *a1,unsigned long a2,void *a3)
{
  unsigned long v1; // rax
  long v2;
  
  if (!a0)
    a0 = &v2;
  v1 = mbrtowc(a0,(char *)a1,a2,a3);
  if (((0xfffffffffffffffe <= v1) && (a2)) && (!sub_4050(0))) {
    v1 = 1;
    *a0 = (unsigned int)*a1;
  }
  return v1;
}

// Function: sub_8580 @ 0x8580
unsigned long sub_8580(int a0) // early-return
{
  unsigned long v1; // rax
  
  if (0x5a < a0)
    return CONCAT71((undefined7)((unsigned long)v1 >> 8),(unsigned int)(a0 - 0x61U) <= 0x19);
  if (0x41 <= a0)
    return 1;
  return (unsigned long)((unsigned int)(a0 - 0x30U) <= 9);
}

// Function: sub_85b0 @ 0x85b0
bool sub_85b0(int a0)
{
  if (a0 <= 0x5a)
    return 0x40 < a0;
  return (unsigned int)(a0 - 0x61U) <= 0x19;
}

// Function: sub_85d0 @ 0x85d0
bool sub_85d0(unsigned int a0)
{
  return a0 <= 0x7f;
}

// Function: sub_85e0 @ 0x85e0
unsigned int sub_85e0(int a0,unsigned long a1,unsigned int a2)
{
  unsigned long v1; // rax
  
  return (unsigned int)CONCAT71((undefined7)((unsigned long)v1 >> 8),a0 == 0x20) | CONCAT31((undefined3)((unsigned int)a2 >> 8),a0 == 9);
}

// Function: sub_8600 @ 0x8600
unsigned long sub_8600(unsigned int a0)
{
  unsigned long v1; // rax
  
  if ((int)a0 <= 0x1f)
    return (unsigned long)(~a0 >> 0x1f);
  return CONCAT71((undefined7)((unsigned long)v1 >> 8),a0 == 0x7f);
}

// Function: sub_8620 @ 0x8620
bool sub_8620(int a0)
{
  return (unsigned int)(a0 - 0x30U) <= 9;
}

// Function: sub_8630 @ 0x8630
bool sub_8630(int a0)
{
  return (unsigned int)(a0 - 0x21U) <= 0x5d;
}

// Function: sub_8640 @ 0x8640
bool sub_8640(int a0)
{
  return (unsigned int)(a0 - 0x61U) <= 0x19;
}

// Function: sub_8650 @ 0x8650
bool sub_8650(int a0)
{
  return (unsigned int)(a0 - 0x20U) <= 0x5e;
}

// Function: sub_8660 @ 0x8660
unsigned long sub_8660(int a0)
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

// Function: sub_86b0 @ 0x86b0
bool sub_86b0(int a0)
{
  if (a0 <= 0xd)
    return 8 < a0;
  return a0 == 0x20;
}

// Function: sub_86d0 @ 0x86d0
bool sub_86d0(int a0)
{
  return (unsigned int)(a0 - 0x41U) <= 0x19;
}

// Function: sub_86e0 @ 0x86e0
unsigned int sub_86e0(int a0)
{
  if ((unsigned int)(a0 - 0x30U) <= 0x36)
    return (unsigned int)(0x7e0000007e03ff >> ((unsigned char)(a0 - 0x30U) & 0x3f)) & 1;
  return 0;
}

// Function: sub_8710 @ 0x8710
int sub_8710(int a0)
{
  if ((unsigned int)(a0 - 0x41U) <= 0x19)
    a0 += 0x20;
  return a0;
}

// Function: sub_8730 @ 0x8730
int sub_8730(int a0)
{
  if ((unsigned int)(a0 - 0x61U) <= 0x19)
    a0 -= 0x20;
  return a0;
}

// Function: sub_8750 @ 0x8750
long sub_8750(unsigned long a0)
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

// Function: sub_87a0 @ 0x87a0
void sub_87a0(unsigned long a0)
{
  if (a0 & 0xf)
    abort(); // no-return
  if (!(a0 & 0x10))
    return;
  free((void *)(a0 - *(unsigned char *)(a0 - 1))); // tail-call
}

// Function: sub_87d0 @ 0x87d0
int sub_87d0(int a0)
{
  int v1; // eax
  
  v1 = wcwidth(a0);
  if (0 <= v1)
    return v1;
  return (unsigned long)(iswcntrl(a0) == 0);
}

// Function: sub_8800 @ 0x8800
void sub_8800(long *a0,long *a1) // return-dupe
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

// Function: sub_8860 @ 0x8860
unsigned int sub_8860(unsigned char a0)
{
  return *(unsigned int *)((unsigned long)(a0 >> 5) * 4 + 0x9e60) >> (a0 & 0x1f) & 1;
}

// Function: sub_8880 @ 0x8880
unsigned long sub_8880(char *a0)
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
    sub_7a60(v1);
    if ((v7) && (!v8)) break;
    v5 = &v5[v6];
    v2 += 1;
  }
  return v2;
}

// Function: _DT_FINI @ 0x893c
void _DT_FINI(void)
{
  return;
}
