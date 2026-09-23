// Function: _DT_INIT @ 0x2000
void _DT_INIT(void) // return-dupe
{
  if (!dat_14fe0)
    return;
  (*dat_14fe0)();
}

// Function: sub_2020 @ 0x2020
void sub_2020(void)
{
  (*dat_14d78)(); // jump-as-call
}

// Function: __cxa_finalize @ 0x24d0
void __cxa_finalize(void)
{
  (*dat_14ff0)(); // jump-as-call
}

// Function: getenv @ 0x24e0
char * getenv(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_14d80)(); // jump-as-call
  return v1;
}

// Function: raise @ 0x24f0
int raise(int a0)
{
  int v1; // eax
  
  v1 = (*dat_14d88)(); // jump-as-call
  return v1;
}

// Function: free @ 0x2500
void free(void *a0)
{
  (*dat_14d90)(); // jump-as-call
}

// Function: abort @ 0x2510
void abort(void)
{
  (*dat_14d98)(); // jump-as-call
}

// Function: __errno_location @ 0x2520
int * __errno_location(void)
{
  int *v1; // rax
  
  v1 = (int *)(*dat_14da0)(); // jump-as-call
  return v1;
}

// Function: strncmp @ 0x2530
int strncmp(char *a0,char *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_14da8)(); // jump-as-call
  return v1;
}

// Function: _exit @ 0x2540
void _exit(int a0)
{
  (*dat_14db0)(); // jump-as-call
}

// Function: __fpending @ 0x2550
unsigned long __fpending(void *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_14db8)(); // jump-as-call
  return v1;
}

// Function: isatty @ 0x2560
int isatty(int a0)
{
  int v1; // eax
  
  v1 = (*dat_14dc0)(); // jump-as-call
  return v1;
}

// Function: reallocarray @ 0x2570
void * reallocarray(void *a0,unsigned long a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_14dc8)(); // jump-as-call
  return v1;
}

// Function: strtod @ 0x2580
void strtod(void)
{
  (*dat_14dd0)(); // jump-as-call
}

// Function: fcntl @ 0x2590
int fcntl(int a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_14dd8)(); // jump-as-call
  return v1;
}

// Function: clearerr_unlocked @ 0x25a0
void clearerr_unlocked(void *a0)
{
  (*dat_14de0)(); // jump-as-call
}

// Function: textdomain @ 0x25b0
char * textdomain(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_14de8)(); // jump-as-call
  return v1;
}

// Function: strtod_l @ 0x25c0
void strtod_l(void)
{
  (*dat_14df0)(); // jump-as-call
}

// Function: fclose @ 0x25d0
int fclose(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_14df8)(); // jump-as-call
  return v1;
}

// Function: bindtextdomain @ 0x25e0
char * bindtextdomain(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_14e00)(); // jump-as-call
  return v1;
}

// Function: dcgettext @ 0x25f0
char * dcgettext(char *a0,char *a1,int a2)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_14e08)(); // jump-as-call
  return v1;
}

// Function: __ctype_get_mb_cur_max @ 0x2600
unsigned long __ctype_get_mb_cur_max(void)
{
  unsigned long v1; // rax
  
  v1 = (*dat_14e10)(); // jump-as-call
  return v1;
}

// Function: strlen @ 0x2610
unsigned long strlen(char *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_14e18)(); // jump-as-call
  return v1;
}

// Function: __stack_chk_fail @ 0x2620
void __stack_chk_fail(void)
{
  (*dat_14e20)(); // jump-as-call
}

// Function: getopt_long @ 0x2630
int getopt_long(int a0,char **a1,char *a2,void *a3,int *a4)
{
  int v1; // eax
  
  v1 = (*dat_14e28)(); // jump-as-call
  return v1;
}

// Function: mbrtowc @ 0x2640
unsigned long mbrtowc(void *a0,char *a1,unsigned long a2,void *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_14e30)(); // jump-as-call
  return v1;
}

// Function: strchr @ 0x2650
char * strchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_14e38)(); // jump-as-call
  return v1;
}

// Function: newlocale @ 0x2660
void newlocale(void)
{
  (*dat_14e40)(); // jump-as-call
}

// Function: nanosleep @ 0x2670
void nanosleep(void)
{
  (*dat_14e48)(); // jump-as-call
}

// Function: strrchr @ 0x2680
char * strrchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_14e50)(); // jump-as-call
  return v1;
}

// Function: lseek @ 0x2690
void lseek(void)
{
  (*dat_14e58)(); // jump-as-call
}

// Function: __assert_fail @ 0x26a0
void __assert_fail(char *a0,char *a1,unsigned int a2,char *a3)
{
  (*dat_14e60)(); // jump-as-call
}

// Function: fputs @ 0x26b0
int fputs(char *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_14e68)(); // jump-as-call
  return v1;
}

// Function: memset @ 0x26c0
void * memset(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_14e70)(); // jump-as-call
  return v1;
}

// Function: close @ 0x26d0
int close(int a0)
{
  int v1; // eax
  
  v1 = (*dat_14e78)(); // jump-as-call
  return v1;
}

// Function: memchr @ 0x26e0
void * memchr(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_14e80)(); // jump-as-call
  return v1;
}

// Function: read @ 0x26f0
long read(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_14e88)(); // jump-as-call
  return v1;
}

// Function: lstat @ 0x2700
int lstat(char *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_14e90)(); // jump-as-call
  return v1;
}

// Function: memcmp @ 0x2710
int memcmp(void *a0,void *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_14e98)(); // jump-as-call
  return v1;
}

// Function: fputs_unlocked @ 0x2720
int fputs_unlocked(char *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_14ea0)(); // jump-as-call
  return v1;
}

// Function: rawmemchr @ 0x2730
void * rawmemchr(void *a0,int a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_14ea8)(); // jump-as-call
  return v1;
}

// Function: ferror_unlocked @ 0x2740
int ferror_unlocked(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_14eb0)(); // jump-as-call
  return v1;
}

// Function: calloc @ 0x2750
void * calloc(unsigned long a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_14eb8)(); // jump-as-call
  return v1;
}

// Function: strcmp @ 0x2760
int strcmp(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_14ec0)(); // jump-as-call
  return v1;
}

// Function: stat @ 0x2770
int stat(char *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_14ec8)(); // jump-as-call
  return v1;
}

// Function: strtol @ 0x2780
long strtol(char *a0,char **a1,int a2)
{
  long v1; // rax
  
  v1 = (*dat_14ed0)(); // jump-as-call
  return v1;
}

// Function: memcpy @ 0x2790
void * memcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_14ed8)(); // jump-as-call
  return v1;
}

// Function: kill @ 0x27a0
int kill(int a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_14ee0)(); // jump-as-call
  return v1;
}

// Function: inotify_init @ 0x27b0
void inotify_init(void)
{
  (*dat_14ee8)(); // jump-as-call
}

// Function: fileno @ 0x27c0
int fileno(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_14ef0)(); // jump-as-call
  return v1;
}

// Function: putc_unlocked @ 0x27d0
int putc_unlocked(int a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_14ef8)(); // jump-as-call
  return v1;
}

// Function: pause @ 0x27e0
void pause(void)
{
  (*dat_14f00)(); // jump-as-call
}

// Function: malloc @ 0x27f0
void * malloc(unsigned long a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_14f08)(); // jump-as-call
  return v1;
}

// Function: fflush @ 0x2800
int fflush(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_14f10)(); // jump-as-call
  return v1;
}

// Function: inotify_add_watch @ 0x2810
void inotify_add_watch(void)
{
  (*dat_14f18)(); // jump-as-call
}

// Function: nl_langinfo @ 0x2820
char * nl_langinfo(int a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_14f20)(); // jump-as-call
  return v1;
}

// Function: __freading @ 0x2830
int __freading(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_14f28)(); // jump-as-call
  return v1;
}

// Function: fwrite_unlocked @ 0x2840
unsigned long fwrite_unlocked(void *a0,unsigned long a1,unsigned long a2,void *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_14f30)(); // jump-as-call
  return v1;
}

// Function: realloc @ 0x2850
void * realloc(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_14f38)(); // jump-as-call
  return v1;
}

// Function: setlocale @ 0x2860
char * setlocale(int a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_14f40)(); // jump-as-call
  return v1;
}

// Function: __printf_chk @ 0x2870
int __printf_chk(int a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_14f48)(); // jump-as-call
  return v1;
}

// Function: poll @ 0x2880
void poll(void)
{
  (*dat_14f50)(); // jump-as-call
}

// Function: error @ 0x2890
void error(int a0,int a1,char *a2,...)
{
  (*dat_14f58)(); // jump-as-call
}

// Function: memrchr @ 0x28a0
void memrchr(void)
{
  (*dat_14f60)(); // jump-as-call
}

// Function: open @ 0x28b0
int open(char *a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_14f68)(); // jump-as-call
  return v1;
}

// Function: fseeko @ 0x28c0
void fseeko(void)
{
  (*dat_14f70)(); // jump-as-call
}

// Function: strtoumax @ 0x28d0
void strtoumax(void)
{
  (*dat_14f78)(); // jump-as-call
}

// Function: fstatfs @ 0x28e0
void fstatfs(void)
{
  (*dat_14f80)(); // jump-as-call
}

// Function: __cxa_atexit @ 0x28f0
void __cxa_atexit(void)
{
  (*dat_14f88)(); // jump-as-call
}

// Function: exit @ 0x2900
void exit(int a0)
{
  (*dat_14f90)(); // jump-as-call
}

// Function: __fprintf_chk @ 0x2910
int __fprintf_chk(void *a0,int a1,char *a2,...)
{
  int v1; // eax
  
  v1 = (*dat_14f98)(); // jump-as-call
  return v1;
}

// Function: fflush_unlocked @ 0x2920
int fflush_unlocked(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_14fa0)(); // jump-as-call
  return v1;
}

// Function: mbsinit @ 0x2930
int mbsinit(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_14fa8)(); // jump-as-call
  return v1;
}

// Function: iswprint @ 0x2940
int iswprint(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_14fb0)(); // jump-as-call
  return v1;
}

// Function: inotify_rm_watch @ 0x2950
void inotify_rm_watch(void)
{
  (*dat_14fb8)(); // jump-as-call
}

// Function: fstat @ 0x2960
int fstat(int a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_14fc0)(); // jump-as-call
  return v1;
}

// Function: __ctype_b_loc @ 0x2970
void * __ctype_b_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_14fc8)(); // jump-as-call
  return v1;
}

// Function: sub_2980 @ 0x2980
undefined16 sub_2980(void)
{
  char v1 [16];
  unsigned long v2; // rax
  
  *__errno_location() = 0xc;
  v1._8_8_ = 0;
  v1._0_8_ = v2;
  return v1._0_16_ << 0x40;
}

// Function: sub_29a0 @ 0x29a0
unsigned long sub_29a0(int a0,unsigned long *a1)
{
  bool v1;
  char *v10;
  unsigned long v11; // rax
  unsigned long v12; // rax
  unsigned long v13; // rax
  unsigned long v14;
  unsigned long v15; // rax
  unsigned long *v16;
  unsigned int v17; // ebx
  long v18; // stack - 0xf0
  int v19; // stack - 0xf4
  unsigned char v2; // al
  unsigned long v20; // stack - 0xe8
  char v21 [24];
  unsigned long v22; // stack - 0xe0
  unsigned long *v23;
  long v24; // stack - 0x100
  unsigned int v25; // stack - 0xc0
  unsigned int v3; // eax
  int v4; // eax
  int v5; // eax
  unsigned long *v6; // rax
  unsigned long *v7;
  unsigned long *v8; // rax
  int *v9; // rax
  
  v19 = 0;
  v18 = 10;
  v20 = dat_f4f0;
  sub_91a0(*a1);
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
  v2 = sub_3bc0(a0,a1,&v18);
  a0 -= (unsigned int)v2;
  sub_7080(a0,&a1[v2],&v18,&v19,&v20);
  if ((dat_15116) && (v18))
    v18 -= 1;
  v16 = (unsigned long *)0x15028;
  v24 = 1;
  if (optind < a0) {
    v16 = &(&a1[v2])[optind];
    v24 = (long)(a0 - optind);
  }
  v1 = 0;
  v23 = &v16[v24];
  v7 = v16;
  do {
    v10 = (char *)*v7;
    if (!strcmp(v10,"-"))
      v1 = 1;
    v7 = &v7[1];
  } while (v23 != v7);
  if (v1) {
    if (dat_15020 == 1) {
      v14 = sub_b410(4,"-");
      error(1,0,dcgettext(NULL,"cannot follow %s by name",5),v14);
      return v15;
    }
    if (dat_15118) {
      if (((((dat_15110) || (dat_15020 != 2)) || (v24 != 1)) || ((fstat(0,v21) || ((v25 & 0xf000) == 0x8000)))) && (isatty(0)))
        error(0,0,dcgettext(NULL,"warning: following standard input indefinitely is ineffective",5));
      goto label_2b55;
    }
    if (v18) goto label_2b6a;
  }
  else {
label_2b55:
    if ((v18) || (dat_15118)) goto label_2b6a;
  }
  if (!dat_15116)
    return 0;
label_2b6a:
  v6 = (unsigned long *)sub_c0c0(v24,0x60);
  v7 = v6;
  do {
    v14 = *v16;
    v16 = &v16[1];
    v8 = &v7[0xc];
    *v7 = v14;
    v7 = v8;
  } while (v16 != v23);
  if ((v19 == 1) || ((v24 != 1 && (!v19))))
    dat_15115 = 1;
  v17 = 1;
  v16 = v6;
  do {
    v23 = &v16[0xc];
    v3 = sub_6ab0(v16,v18);
    v17 &= v3;
    v16 = v23;
  } while (&v6[v24 * 0xc] != v23);
  if ((dat_15118) && (sub_3a50(v6,v24))) {
    if (fstat(1,v21) < 0) {
      v10 = dcgettext(NULL,"standard output",5);
      error(1,*__errno_location(),v10);
      return v12;
    }
    dat_15117 = (v25 & 0xf000) == 0x1000;
    if ((((!dat_1510c) && (!sub_3ae0(v6,v24))) && (!sub_38d0(v6,v24))) && (((sub_3900(v6) && (!sub_4b40(v6))) && ((!sub_3930(v6,v24) && (((char)v17 || (dat_15020 != 2)))))))) {
      v4 = inotify_init();
      v9 = __errno_location();
      if (0 <= v4) {
        v5 = fflush_unlocked(stdout);
        if (v5) {
          v10 = dcgettext(NULL,"write error",5);
          error(1,*v9,v10);
          return v13;
        }
        sub_5880(v20,v4,v6,v24,&v22);
        sub_8a10(v22);
        close(v4);
        *v9 = 0;
      }
      v10 = dcgettext(NULL,"inotify cannot be used, reverting to polling",5);
      error(0,*v9,v10);
    }
    dat_1510c = '\x01';
    sub_6220(v20,v6,v24);
  }
  if ((dat_1510e) && (close(0) <= -1)) {
    error(1,*__errno_location(),"-");
    return v11;
  }
  return (unsigned long)(v17 ^ 1);
}

// Function: sub_2ec0 @ 0x2ec0
void sub_2ec0(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_14fd0)(sub_29a0,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: sub_2ef0 @ 0x2ef0
void sub_2ef0(void)
{
  return;
}

// Function: _FINI_0 @ 0x2f60
void _FINI_0(void)
{
  if (!dat_15108) {
    if (dat_14ff0)
      __cxa_finalize(dat_15008);
    sub_2ef0();
    dat_15108 = 1;
    return;
  }
}

// Function: _INIT_0 @ 0x2fa0
void _INIT_0(void)
{
  return;
}

// Function: sub_2fb0 @ 0x2fb0
unsigned int sub_2fb0(char *a0) // return-dupe x5, switch-return x24
{
  bool v1; // zf
  
  if (a0 == (char *)0x65735543)
    return 0;
  if ((char *)0x65735544 <= a0) {
    if (a0 == (char *)0x7c7c6673)
      return 0;
    if (a0 <= (char *)0x7c7c6673) {
      if (a0 == (char *)0x73757245)
        return 0;
      if ((char *)0x73757246 <= a0) {
        if (a0 == (char *)0x786f4256)
          return 0;
        if (a0 <= (char *)0x786f4256) {
          if (a0 == (char *)0x7461636f)
            return 0;
          return ((unsigned int)(a0 == (char *)0x74726163) - 1) + (unsigned int)(a0 == (char *)0x74726163);
        }
        return -(unsigned int)(a0 != (char *)0x794c7630);
      }
      if (a0 == (char *)0x6b414653)
        return 0;
      if (a0 <= (char *)0x6b414653) {
        if (a0 == (char *)0x65735546)
          return 0;
        if (a0 <= (char *)0x65735545)
          return 0xffffffff;
        if (a0 == (char *)0x67596969)
          return 1;
        return ((unsigned int)(a0 == (char *)0x68191122) - 1) + (unsigned int)(a0 == (char *)0x68191122);
      }
      if (a0 == (char *)0x6e667364)
        return 0;
      if ((char *)0x6e667365 <= a0) {
        if (a0 == (char *)0x73717368)
          return 1;
        if (a0 <= (char *)0x73717368) {
          if (a0 == (char *)0x6e736673)
            return 1;
          return ((unsigned int)(a0 == (char *)0x73636673) - 1) + (unsigned int)(a0 == (char *)0x73636673);
        }
        return ((unsigned int)(a0 == (char *)0x73727279) - 1) + (unsigned int)(a0 == (char *)0x73727279);
      }
      return ((unsigned int)(a0 == (char *)0x6c6f6f70) - 1) + (unsigned int)(a0 == (char *)0x6c6f6f70);
    }
    if (a0 == (char *)0xbeefdead)
      return 0;
    if ((char *)0xbeefdeae <= a0) {
      if (a0 == (char *)0xfe534d42)
        return 0;
      if ((char *)0xfe534d43 <= a0)
        return -(unsigned int)(a0 != (char *)0xff534d42);
      if (a0 == (char *)0xe0f5e1e2)
        return 1;
      if ((char *)0xe0f5e1e3 <= a0) {
        if (a0 == (char *)0xf97cff8c)
          return 1;
        if (a0 == (char *)0xf995e849)
          return 1;
        return ((unsigned int)(a0 == (char *)0xf2f52010) - 1) + (unsigned int)(a0 == (char *)0xf2f52010);
      }
      if (a0 == (char *)0xcafe4a11)
        return 1;
      if (a0 <= (char *)0xcafe4a11) {
        if (a0 == (char *)0xc7571590)
          return 1;
        return ((unsigned int)(a0 == (char *)0xc97e8168) - 1) + (unsigned int)(a0 == (char *)0xc97e8168);
      }
      return ((unsigned int)(a0 == (char *)0xde5e81e4) - 1) + (unsigned int)(a0 == (char *)0xde5e81e4);
    }
    if (a0 == (char *)0xaad7aaea)
      return 0;
    if ((char *)0xaad7aaeb <= a0) {
      if (a0 == (char *)0xabba1974)
        return 1;
      return -(unsigned int)(a0 != (char *)0xbacbacbc);
    }
    if (a0 == (char *)0xa501fcf5)
      return 0;
    if (a0 <= (char *)0xa501fcf5) {
      if (a0 == (char *)0x9123683e)
        return 1;
      v1 = ((unsigned long)a0 & 0xffffffffefffffff) == 0x858458f6;
      return ((unsigned int)v1 - 1) + (unsigned int)v1;
    }
    return 0xffffffff;
  }
  if (a0 == (char *)0xbd00bd0)
    return 0;
  if ((char *)0xbd00bd1 <= a0) {
    if (a0 == (char *)0x5345434d)
      return 1;
    if (a0 <= (char *)0x5345434d) {
      if (a0 == (char *)0x47504653)
        return 0;
      if ((char *)0x47504654 <= a0) {
        if (a0 == (char *)0x50495045)
          return 0;
        return ((unsigned int)(a0 == (char *)0x52654973) - 1) + (unsigned int)(a0 == (char *)0x52654973);
      }
      if (a0 == (char *)0x19830326)
        return 0;
      if (a0 <= (char *)0x19830326) {
        if (a0 == (char *)0x15013346)
          return 1;
        if (a0 <= (char *)0x15013346) {
          if (a0 == (char *)0x11307854)
            return 1;
          return ((unsigned int)(a0 == (char *)0x13661366) - 1) + (unsigned int)(a0 == (char *)0x13661366);
        }
        return ((unsigned int)(a0 == (char *)0x19800202) - 1) + (unsigned int)(a0 == (char *)0x19800202);
      }
      if (a0 == (char *)0x42465331)
        return 1;
      if ((char *)0x42465332 <= a0) {
        if (a0 == (char *)0x453dcd28)
          return 1;
        if ((char *)0x453dcd29 <= a0) {
          if (a0 == (char *)0x454d444d)
            return 1;
          return ((unsigned int)(a0 == (char *)0x45584653) - 1) + (unsigned int)(a0 == (char *)0x45584653);
        }
        if (a0 == (char *)0x43415d53)
          return 1;
        if (a0 == (char *)0x444d4142)
          return 1;
        return ((unsigned int)(a0 == (char *)0x42494e4d) - 1) + (unsigned int)(a0 == (char *)0x42494e4d);
      }
      if (a0 == (char *)0x28cd3d45)
        return 1;
      if (a0 <= (char *)0x28cd3d45) {
        if (a0 == (char *)0x2011bab0)
          return 1;
        if (a0 == (char *)0x24051905)
          return 1;
        return ((unsigned int)(a0 == (char *)0x1badface) - 1) + (unsigned int)(a0 == (char *)0x1badface);
      }
      if (a0 == (char *)0x2fc12fc1)
        return 1;
      if (a0 == (char *)0x3153464a)
        return 1;
      return ((unsigned int)(a0 == (char *)0x2bad1dea) - 1) + (unsigned int)(a0 == (char *)0x2bad1dea);
    }
    if (a0 == (char *)0x61636673)
      return 0;
    if (a0 <= (char *)0x61636673) {
      if (a0 == (char *)0x5346414f)
        return 0;
      if (a0 <= (char *)0x5346414f)
        return ((unsigned int)(a0 == (char *)0x5346314d) - 1) + (unsigned int)(a0 == (char *)0x5346314d);
      if (a0 == (char *)0x58295829)
        return 1;
      if ((char *)0x5829582a <= a0) {
        if (a0 == (char *)0x5a4f4653)
          return 1;
        if (a0 <= (char *)0x5a4f4653) {
          if (a0 == (char *)0x58465342)
            return 1;
          return ((unsigned int)(a0 == (char *)0x5a3c69f0) - 1) + (unsigned int)(a0 == (char *)0x5a3c69f0);
        }
        return ((unsigned int)(a0 == (char *)0x5dca2df5) - 1) + (unsigned int)(a0 == (char *)0x5dca2df5);
      }
      if (a0 == (char *)0x534f434b)
        return 1;
      if ((char *)0x534f434c <= a0) {
        if (a0 == (char *)0x54190100)
          return 1;
        return ((unsigned int)(a0 == (char *)0x565a4653) - 1) + (unsigned int)(a0 == (char *)0x565a4653);
      }
      if (a0 == (char *)0x53464846)
        return 1;
      return ((unsigned int)(a0 == (char *)0x5346544e) - 1) + (unsigned int)(a0 == (char *)0x5346544e);
    }
    if (a0 == (char *)0x61756673)
      return 0;
    if (a0 <= (char *)0x61756673)
      return ((unsigned int)(a0 == (char *)0x6165676c) - 1) + (unsigned int)(a0 == (char *)0x6165676c);
    if (a0 == (char *)0x63677270)
      return 1;
    if ((char *)0x63677271 <= a0) {
      if (a0 == (char *)0x64626720)
        return 1;
      return ((unsigned int)(a0 == (char *)0x64646178) - 1) + (unsigned int)(a0 == (char *)0x64646178);
    }
    if (((unsigned long)a0 & 0xfffffffffffffffd) == 0x62656570)
      return 1;
    return ((unsigned int)(a0 == (char *)0x62646576) - 1) + (unsigned int)(a0 == (char *)0x62646576);
  }
  if (a0 == (char *)0x6969)
    return 0;
  if (a0 <= (char *)0x6969) {
    if (a0 == (char *)0x517b)
      return 0;
    if ((char *)0x517c <= a0) {
      if (a0 == (char *)0x564c)
        return 0;
      return ((unsigned int)(a0 == (char *)0x5df5) - 1) + (unsigned int)(a0 == (char *)0x5df5);
    }
    if (a0 == (char *)0x2478)
      return 1;
    if (a0 <= (char *)0x2478) {
      if ("HK\x01" <= a0) {
        if (a0 == (char *)0x1cd1)
          return 1;
        return ((unsigned int)(a0 == (char *)0x2468) - 1) + (unsigned int)(a0 == (char *)0x2468);
      }
      if ((char *)0x1373 <= a0)
        return -(unsigned int)((0x10001401UL >> ((char)a0 + 0x8dU & 0x3f) & 1) == 0) | 1;
      if (a0 == (char *)0x187)
        return 1;
      if (a0 <= (char *)0x187)
        return (-(unsigned int)(((unsigned long)&a0[-0x2f] & 0xfffffffffffffffb) == 0) & 2) - 1;
      return ((unsigned int)(a0 == "") - 1) + (unsigned int)(a0 == "");
    }
    if (a0 == (char *)0x4244)
      return 1;
    if (a0 <= (char *)0x4244) {
      if (a0 == (char *)0x3434)
        return 1;
      if ((char *)0x7 <= &a0[-0x4000])
        return 0xffffffff;
      return -(unsigned int)((0x51UL >> ((unsigned char)&a0[-0x4000] & 0x3f) & 1) == 0) | 1;
    }
    if (a0 == (char *)0x4d44)
      return 1;
    if (a0 <= (char *)0x4d44) {
      if (a0 == (char *)0x482b)
        return 1;
      return ((unsigned int)(a0 == (char *)0x4858) - 1) + (unsigned int)(a0 == (char *)0x4858);
    }
    return ((unsigned int)(a0 == (char *)0x4d5a) - 1) + (unsigned int)(a0 == (char *)0x4d5a);
  }
  if (a0 == (char *)0x1161970)
    return 0;
  if ((char *)0x1161971 <= a0) {
    if (a0 == (char *)0x13111a8)
      return 0;
    if (a0 <= (char *)0x13111a8) {
      if (a0 == (char *)0x12fd16d)
        return 1;
      return (-(unsigned int)(&a0[-0x12ff7b4] < (char *)0x4) & 2) - 1;
    }
    if (a0 == (char *)0x9041934)
      return 1;
    if (a0 == (char *)0xbad1dea)
      return 1;
    return ((unsigned int)(a0 == (char *)0x7655821) - 1) + (unsigned int)(a0 == (char *)0x7655821);
  }
  if (a0 == (char *)0xc36400)
    return 0;
  if ((char *)0xc36401 <= a0) {
    if (a0 == (char *)0x1021994)
      return 1;
    return ((unsigned int)(a0 == (char *)0x1021997) - 1) + (unsigned int)(a0 == (char *)0x1021997);
  }
  if (a0 == " to polling")
    return 1;
  if (a0 <= " to polling") {
    if ((char *)0x9fa3 <= a0) {
      if (a0 == (char *)0xadf5)
        return 1;
      return ((unsigned int)(a0 == (char *)0xadff) - 1) + (unsigned int)(a0 == (char *)0xadff);
    }
    if ((char *)0x9fa0 <= a0)
      return 1;
    if (a0 == (char *)0x72b6)
      return 1;
    if (a0 == (char *)0x9660)
      return 1;
    return ((unsigned int)(a0 == (char *)0x7275) - 1) + (unsigned int)(a0 == (char *)0x7275);
  }
  if (a0 == (char *)0x27e0eb)
    return 1;
  if ((char *)0x27e0ec <= a0) {
    if (a0 == (char *)0x414a53)
      return 1;
    return ((unsigned int)(a0 == (char *)0xc0ffee) - 1) + (unsigned int)(a0 == (char *)0xc0ffee);
  }
  if (a0 == "exhausted")
    return 1;
  if (a0 == (char *)0x11954)
    return 1;
  return ((unsigned int)(a0 == "o polling") - 1) + (unsigned int)(a0 == "o polling");
}

// Function: sub_3830 @ 0x3830
void sub_3830(long a0,unsigned int a1,unsigned long a2,unsigned long *a3,unsigned int a4)
{
  unsigned int v1;
  unsigned long v2;
  char v3 [16];
  
  *(unsigned int *)(a0 + 0x38) = a1;
  *(unsigned long *)(a0 + 8) = a2;
  v3 = sub_b7b0(a3);
  *(long *)(a0 + 0x10) = SUB168(v3,0);
  v2 = *a3;
  *(long *)(a0 + 0x18) = SUB168(v3,8);
  *(unsigned long *)(a0 + 0x20) = v2;
  *(unsigned long *)(a0 + 0x28) = a3[1];
  v1 = *(unsigned int *)&a3[3];
  *(unsigned int *)(a0 + 0x40) = a4;
  *(unsigned int *)(a0 + 0x30) = v1;
  *(unsigned long *)(a0 + 0x58) = 0;
  *(char *)(a0 + 0x34) = 0;
}

// Function: sub_3880 @ 0x3880
unsigned char sub_3880(long a0,long a1) // early-return
{
  unsigned char v1; // al
  unsigned char *v2; // rcx
  unsigned char *v3; // rdi
  
  if ((dat_1511a) && (dat_15020 == 1))
    return dat_1511a;
  v3 = (unsigned char *)(a0 + 0x34);
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
char sub_38d0(long a0,long a1)
{
  char *v1; // rax
  char *v2; // rdi
  
  v2 = (char *)(a0 + 0x35);
  v1 = &v2[a1 * 0x60];
  while ((*(int *)&v2[3] <= -1 || (!*v2))) {
    v2 = &v2[0x60];
    if (v2 == v1)
      return '\0';
  }
  return *v2;
}

// Function: sub_3900 @ 0x3900
unsigned long sub_3900(long a0,long a1)
{
  char *v1; // rax
  char *v2; // rdi
  
  v2 = (char *)(a0 + 0x35);
  v1 = &v2[a1 * 0x60];
  while ((*(int *)&v2[3] < 0 || (*v2))) {
    v2 = &v2[0x60];
    if (v2 == v1)
      return 0;
  }
  return 1;
}

// Function: sub_3930 @ 0x3930
unsigned int sub_3930(long a0,long a1)
{
  unsigned int *v1; // rcx
  unsigned int v2; // edx
  unsigned int *v3; // rdi
  
  v3 = (unsigned int *)(a0 + 0x30);
  v1 = &v3[a1 * 0x18];
  while (((int)v3[2] <= -1 || (v2 = *v3 & 0xf000, v2 == 0x1000 || v2 == 0x8000))) {
    v3 = &v3[0x18];
    if (v1 == v3)
      return 0;
  }
  return CONCAT31((undefined3)(v3[2] >> 8),v2 != 0x1000 && v2 != 0x8000);
}

// Function: sub_3980 @ 0x3980
unsigned long sub_3980(long a0,unsigned long a1)
{
  return (unsigned long)(long)*(int *)(a0 + 0x44) % a1;
}

// Function: sub_39a0 @ 0x39a0
unsigned int sub_39a0(long a0,long a1)
{
  return CONCAT31((undefined3)((unsigned int)*(int *)(a1 + 0x44) >> 8),*(int *)(a0 + 0x44) == *(int *)(a1 + 0x44));
}

// Function: sub_39b0 @ 0x39b0
void sub_39b0(void)
{
  void *v1;
  
  v1 = stdout;
  fputs_unlocked(dcgettext(NULL,"\nWith no FILE, or when FILE is -, read standard input.\n",5),v1); // tail-call
}

// Function: sub_39e0 @ 0x39e0
void sub_39e0(void)
{
  void *v1;
  
  v1 = stdout;
  fputs_unlocked(dcgettext(NULL,"\nMandatory arguments to long options are mandatory for short options too.\n",5),v1); // tail-call
}

// Function: sub_3a10 @ 0x3a10
void sub_3a10(unsigned long a0)
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
void sub_3b30(unsigned int a0,unsigned long a1)
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
unsigned long sub_3bc0(int a0,long a1,unsigned long *a2) // return-dupe
{
  char v1;
  bool v10; // zf
  int v2; // eax
  char *v3;
  char *v4;
  unsigned long v5; // rax
  unsigned long v6; // rax
  char *v7;
  char v8; // bpl
  char v9; // r13b
  
  if (a0 != 2) {
    if (a0 != 3) { // branch-flip
      if (a0 != 4)
        return 0;
      v4 = *(char **)(a1 + 0x10);
    }
    else {
      v4 = *(char **)(a1 + 0x10);
      if ((*v4 != '-') || (!v4[1])) goto label_3bf5;
    }
    if (strcmp(v4,"--"))
      return 0;
  }
label_3bf5:
  v2 = sub_9110();
  v7 = *(char **)(a1 + 8);
  v4 = &v7[1];
  if (*v7 != '+') { // branch-flip
    if (*v7 != '-')
      return 0;
    if ((0x30db0 <= v2) && (!v7[(unsigned long)(v7[1] == 'c') + 1]))
      return 0;
    v8 = 0;
  }
  else {
    v8 = 1;
    if ((unsigned int)(v2 - 0x30db0U) <= 0x2b8)
      return 0;
  }
  v1 = v7[1];
  v7 = v4;
  while ((unsigned int)((int)v1 - 0x30U) <= 9) {
    v3 = &v7[1];
    v7 = &v7[1];
    v1 = *v3;
  }
  if (v1 != 'c') { // branch-flip
    if (v1 != 'l') { // branch-flip
      if (v1 != 'b') {
        v9 = 1;
        v6 = 10;
        v3 = v7;
        goto label_3c8b;
      }
      v9 = 0;
      v6 = 0x1400;
    }
    else {
      v9 = 1;
      v6 = 10;
    }
  }
  else {
    v9 = 0;
    v6 = 10;
  }
  v3 = &v7[1];
label_3c8b:
  v10 = *v3 == 'f';
  if (v10)
    v3 = &v3[1];
  if (*v3)
    return 0;
  if (v4 == v7) {
    *a2 = v6;
    dat_15116 = v8;
    dat_15118 = v10;
    dat_15119 = v9;
    return 1;
  }
  if (!(sub_c6c0(v4,0,10,a2,"b") & 0xfffffffd)) {
    dat_15116 = v8;
    dat_15118 = v10;
    dat_15119 = v9;
    return 1;
  }
  v6 = sub_b6d0(*(unsigned long *)(a1 + 8));
  v4 = dcgettext(NULL,"invalid number",5);
  error(1,*__errno_location(),"%s: %s",v4,v6);
  return v5;
}

// Function: sub_3da0 @ 0x3da0
void sub_3da0(void *a0,unsigned long a1)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  char *v3; // rax
  
  if (!a1)
    return;
  v1 = fwrite_unlocked(a0,1,a1,stdout);
  if (a1 <= v1)
    return;
  clearerr_unlocked(stdout);
  v2 = sub_b410(4,"standard output");
  v3 = dcgettext(NULL,"error writing %s",5);
  error(1,*__errno_location(),v3,v2);
}

// Function: sub_3e30 @ 0x3e30
unsigned long sub_3e30(unsigned long a0,unsigned int a1,long a2,long *a3) // return-dupe
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
int * sub_3f80(char a0,unsigned long a1,unsigned int a2,void *a3)
{
  int *v1;
  unsigned long v2;
  int *v3; // rax
  void *v4;
  void *v5;
  char v6 [8200];
  long v7; // r12
  
  v7 = 0;
  v5 = a3;
  do {
    v4 = _DT_INIT;
    if (v5 <= (void *)0x2000)
      v4 = v5;
    v3 = (long)sub_b6e0(a2,v6,v4);
    if (v3 == (int *)0xffffffffffffffff) {
      v3 = __errno_location();
      v1 = v3;
      if (*v3 == 0xb) {
        v3 = (int *)v7;
        return v3;
      }
      v3 = (unsigned long)sub_b410(4,a1);
      v2 = v3;
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
    if (a3 != (void *)0xffffffffffffffff) {
      v5 = &v5[-(long)v3];
      if (!v5) {
        v3 = (int *)v7;
        return v3;
      }
      if (a3 == (void *)0xfffffffffffffffe) {
        v3 = (int *)v7;
        return v3;
      }
    }
    a0 = '\0';
  } while( true );
}

// Function: sub_40c0 @ 0x40c0
unsigned long sub_40c0(unsigned long a0,unsigned int a1,unsigned long a2,long *a3) // return-dupe
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
void sub_41c0(unsigned long a0,unsigned long a1,int a2,unsigned long a3)
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
unsigned long sub_4290(unsigned long a0,unsigned int a1,long a2,long a3,long a4,long *a5) // return-dupe
{
  long v1;
  long v10; // rsi
  long v11; // stack - 0x2050
  long v2; // rax
  long v3; // rax
  unsigned long v4; // rax
  char *v5; // rax
  long v6;
  int v7; // ebx
  void *v8; // rbx
  char v9 [8200];
  
  if (!a2)
    return 1;
  v8 = (void *)((a4 - a3) % 0x2000);
  if (!v8)
    v8 = _DT_INIT;
  v10 = a4 - (long)v8;
  v11 = a4;
  sub_41c0(a1,v10,0,a0);
  v2 = sub_b6e0(a1,v9,v8);
  if (v2 != -1) {
    v7 = (int)dat_15114;
    *a5 = v10 + v2;
    if ((v2) && (v9[v2 + -1] != dat_15114))
      a2 -= 1;
    while( true ) {
      v6 = v2;
      while ((v6 && (v3 = memrchr(v9,v7), v3))) {
        v1 = a2 + -1;
        v6 = v3 - (long)v9;
        if (!a2) {
          sub_3da0(v3 + 1,(v2 + -1) - v6);
          v2 = sub_3f80(0,a0,a1,(v11 - v2) - v10);
          *a5 = *a5 + v2;
          return 1;
        }
        a2 = v1;
      }
      if (v10 == a3) {
        sub_41c0(a1,v10,0,a0);
        *a5 = v10 + sub_3f80(0,a0,a1,v11);
        return 1;
      }
      v10 -= 0x2000;
      sub_41c0(a1,v10,0,a0);
      v2 = sub_b6e0(a1,v9,_DT_INIT);
      if (v2 == -1) break;
      *a5 = v10 + v2;
      if (!v2)
        return 1;
      v7 = (int)dat_15114;
    }
  }
  v4 = sub_b410(4,a0);
  v5 = dcgettext(NULL,"error reading %s",5);
  error(0,*__errno_location(),v5,v4);
  return 0;
}

// Function: sub_4510 @ 0x4510
unsigned long sub_4510(unsigned long a0,unsigned int a1,unsigned long a2,long *a3)
{
  unsigned long v1;
  void *v10; // r13
  void *v11;
  unsigned long v12;
  void *v2;
  void *v3;
  unsigned long v4;
  unsigned long v5; // rax
  char *v6; // rax
  void *v7;
  long v8;
  unsigned long v9; // r12
  
  v2 = (void *)sub_bfa0(0x2010);
  *(unsigned long *)((long)v2 + 0x2000) = 0;
  *(unsigned long *)((long)v2 + 0x2008) = 0;
  v3 = (void *)sub_bfa0(0x2010);
  v7 = v3;
  v11 = v2;
  v12 = 0;
  while (v10 = v7, v7 = v2, v4 = sub_b6e0(a1,v3,_DT_INIT), v4 - 1 <= 0xfffffffffffffffd) {
    v12 += v4;
    *a3 = *a3 + v4;
    *(unsigned long *)((long)v10 + 0x2000) = v4;
    v8 = *(long *)((long)v11 + 0x2000);
    *(unsigned long *)((long)v10 + 0x2008) = 0;
    v2 = v7; // branch-flip
    if (_DT_INIT <= (void *)(v8 + v4)) {
      *(void **)((long)v11 + 0x2008) = v10;
      v4 = v12 - *(long *)((long)v7 + 0x2000);
      v11 = v10; // branch-flip
      if (v4 <= a2) {
        v3 = (void *)sub_bfa0(0x2010);
        v7 = v3;
      }
      else {
        v2 = *(void **)((long)v7 + 0x2008);
        v3 = v7;
        v12 = v4;
      }
    }
    else {
      memcpy((void *)(v8 + (long)v11),v3,v4);
      *(long *)((long)v11 + 0x2000) = *(long *)((long)v11 + 0x2000) + *(long *)((long)v10 + 0x2000);
      v7 = v10;
    }
  }
  free(v10);
  if (v4 != 0xffffffffffffffff) { // branch-flip
    v8 = *(long *)((long)v7 + 0x2000);
    v4 = v12 - v8;
    v2 = v7;
    while (v1 = v4, a2 < v1) {
      v2 = *(void **)((long)v2 + 0x2008);
      v8 = *(long *)((long)v2 + 0x2000);
      v4 = v1 - v8;
      v12 = v1;
    }
    v3 = v2;
    if (a2 < v12) {
      v8 += a2 - v12;
      v3 = (void *)((long)v2 + (v12 - a2));
    }
    sub_3da0(v3,v8);
    for (v8 = *(long *)((long)v2 + 0x2008); v8; v8 = *(long *)(v8 + 0x2008)) {
      sub_3da0(v8,*(unsigned long *)&_DT_INIT[v8]);
    }
    v9 = 1;
  }
  else {
    v5 = sub_b410(4,a0);
    v6 = dcgettext(NULL,"error reading %s",5);
    v9 = 0;
    error(0,*__errno_location(),v6,v5);
    if (!v7)
      return 0;
  }
  do {
    v2 = *(void **)((long)v7 + 0x2008);
    free(v7);
    v7 = v2;
  } while (v2);
  return v9;
}

// Function: sub_4760 @ 0x4760
unsigned long sub_4760(unsigned long a0,unsigned int a1,unsigned long a2,long *a3)
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
  while (v10 = v2, v3 = sub_b6e0(a1,v13,_DT_INIT), (unsigned long)(v3 - 1U) <= 0xfffffffffffffffd) {
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
    if (_DT_INIT <= (void *)(*(unsigned long *)((long)v13 + 0x2000) + *(long *)((long)v9 + 0x2000))) {
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
        sub_3da0(v3,*(unsigned long *)&_DT_INIT[v3]);
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
unsigned long sub_4a40(unsigned long a0,unsigned long a1) // return-dupe x3
{
  int v1; // eax
  int *v2; // rax
  unsigned long v3; // rax
  char *v4; // rax
  unsigned long v5 [15]; // stack - 0xa8
  
  if (!fstatfs(a0,v5)) {
    v1 = sub_2fb0(v5[0]);
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
  v3 = sub_b410(4,a1);
  v4 = dcgettext(NULL,"cannot determine location of %s. reverting to polling",5);
  error(0,*v2,v4,v3);
  return 1;
}

// Function: sub_4b40 @ 0x4b40
unsigned long sub_4b40(unsigned long *a0,long a1)
{
  char *v1;
  char v2 [24];
  unsigned long *v3; // rbp
  unsigned int v4; // stack - 0xa0
  
  v3 = &a0[a1 * 0xc];
  while ((v1 = (char *)*a0, lstat(v1,v2) || ((v4 & 0xf000) != 0xa000))) {
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

// Function: sub_4c60 @ 0x4c60
unsigned int sub_4c60(unsigned long a0,int a1,unsigned long a2,long *a3)
{
  unsigned int v1; // eax
  unsigned long v2; // rax
  char *v3; // rax
  long v4; // rax
  long v5; // rax
  char v6 [24];
  unsigned int v7; // eax
  unsigned int v8; // stack - 0xc0
  
  if (fstat(a1,v6)) { // branch-flip
    v2 = sub_b410(4,a0);
    v3 = dcgettext(NULL,"cannot fstat %s",5);
    v7 = 0;
    error(0,*__errno_location(),v3,v2);
  }
  else {
    v7 = (unsigned int)dat_15116;
    if (dat_15116) { // branch-flip
      v1 = sub_3e30(a0,a1,a2,a3);
      if (v1) // branch-flip
        v7 = v1 >> 0x1f;
      else {
        v4 = sub_3f80(0,a0,a1,0xffffffffffffffff);
        *a3 = *a3 + v4;
      }
    }
    else {
      if (((!dat_1510d) && ((v8 & 0xf000) == 0x8000)) && (v4 = lseek(a1,0,1), v4 != -1)) {
        v5 = lseek(a1,0,2);
        if (v4 < v5) {
          *a3 = v5;
          if (!v5)
            return 1;
          return sub_4290(a0,a1,a2,v4,v5,a3);
        }
        sub_41c0(a1,v4,0,a0);
      }
      v7 = sub_4760(a0,a1,a2,a3);
    }
  }
  return v7;
}

// Function: sub_4e20 @ 0x4e20
void sub_4e20(void)
{
  void *v1;
  char *v10; // stack - 0x78
  char *v11; // stack - 0x70
  char *v12; // stack - 0x68
  char *v13; // stack - 0x60
  unsigned long v14; // stack - 0x58
  unsigned long v15; // stack - 0x50
  char *v16 [4]; // stack - 0xa8
  int v2; // eax
  char *v3; // rax
  char **v4; // rbx
  char *v5; // stack - 0xb8
  char *v6;
  char *v7; // stack - 0xb0
  char *v8; // stack - 0x88
  char *v9; // stack - 0x80
  
  v6 = "[";
  v4 = &v5;
  v5 = "[";
  v7 = "test invocation";
  v16[0] = "coreutils";
  v16[1] = "Multi-call invocation";
  v16[2] = "sha224sum";
  v8 = "sha256sum";
  v16[3] = "sha2 utilities";
  v9 = "sha2 utilities";
  v10 = "sha384sum";
  v11 = "sha2 utilities";
  v12 = "sha512sum";
  v13 = "sha2 utilities";
  v14 = 0;
  v15 = 0;
  do {
    if (!strcmp("tail",v6)) break;
    v6 = *(char **)((long)v4 + 0x10);
    v4 = (char *)((long)v4 + 0x10);
  } while (v6);
  v6 = *(char **)((long)v4 + 8);
  if (!v6)
    v6 = "tail";
  __printf_chk(1,dcgettext(NULL,"\n%s online help: <%s>\n",5),"GNU coreutils","https://www.gnu.org/software/coreutils/");
  v3 = setlocale(5,NULL);
  if ((v3) && (v2 = strncmp(v3,"en_",3), v1 = stdout, v2))
    fputs_unlocked(dcgettext(NULL,"Report any translation bugs to <https://translationproject.org/team/>\n",5),v1);
  v3 = "tail";
  if (!strcmp("tail","["))
    v3 = "test";
  __printf_chk(1,dcgettext(NULL,"Full documentation <%s%s>\n",5),"https://www.gnu.org/software/coreutils/",v3);
  v3 = "";
  if (v6 == "tail")
    v3 = " invocation";
  __printf_chk(1,dcgettext(NULL,"or available locally via: info \'(coreutils) %s%s\'\n",5),v6,v3); // tail-call
}

// Function: sub_5050 @ 0x5050
char * sub_5050(char *a0)
{
  if (strcmp(a0,"-"))
    return a0;
  return dcgettext(NULL,"standard input",5); // tail-call
}

// Function: sub_5090 @ 0x5090
void sub_5090(unsigned long *a0,unsigned long *a1) // return-dupe x3
{
  unsigned long v1;
  char v10 [16];
  long v11; // stack - 0x98
  int v2;
  unsigned long v3; // rax
  long v4; // rax
  char *v5; // rax
  int *v6; // rax
  unsigned long v7;
  char v8 [48];
  bool v9;
  
  if (*(int *)&a0[7] == -1)
    return;
  v3 = sub_5050(*a0);
  v2 = *(int *)&a0[7];
  if (fstat(v2,v8)) {
    v6 = __errno_location();
    *(int *)((long)a0 + 0x3c) = *v6;
    sub_3b30(*(unsigned int *)&a0[7],v3);
    *(unsigned int *)&a0[7] = 0xffffffff;
    return;
  }
  if ((*(unsigned int *)&a0[6] & 0xf000) == 0x8000) {
    if ((long)a0[1] <= v11) { // branch-flip
      if (v11 == a0[1]) {
        v10 = sub_b7b0(v8);
        v7 = a0[2];
        v1 = a0[3];
        if (!sub_b800(v7,v1,SUB168(v10,0),SUB168(v10,8)))
          return;
      }
    }
    else {
      v7 = sub_b510(0,3,v3);
      error(0,0,dcgettext(NULL,"%s: file truncated",5),v7);
      sub_41c0(*(unsigned int *)&a0[7],0,0,v3);
      a0[1] = 0;
    }
  }
  v9 = 0;
  if (dat_15115)
    v9 = (unsigned long *)*a1 != a0;
  v4 = sub_3f80(v9,v3,*(unsigned int *)&a0[7],0xffffffffffffffff);
  a0[1] = a0[1] + v4;
  if (!v4)
    return;
  *a1 = a0;
  v2 = fflush_unlocked(stdout);
  if (!v2)
    return;
  v5 = dcgettext(NULL,"write error",5);
  error(1,*__errno_location(),v5);
}

// Function: sub_5260 @ 0x5260
unsigned int sub_5260(int a0,int a1,unsigned int a2)
{
  unsigned long v1; // rax
  
  return (unsigned int)CONCAT71((undefined7)((unsigned long)v1 >> 8),a0 == -1) ^ CONCAT31((undefined3)((unsigned int)a2 >> 8),a1 == 0);
}

// Function: sub_5270 @ 0x5270
void sub_5270(unsigned long *a0,unsigned char a1)
{
  unsigned int v1;
  char *v10;
  long v11; // stack - 0xd8
  unsigned long v12; // r8
  long v13; // stack - 0xd0
  unsigned int v14; // stack - 0xc0
  unsigned int v2;
  char v3;
  char v4; // al
  char v5;
  int v6; // eax
  int v7; // eax
  unsigned int *v8; // rax
  unsigned long v9;
  
  v10 = (char *)*a0;
  v6 = strcmp(v10,"-");
  v1 = *(unsigned int *)((long)a0 + 0x3c);
  v5 = *(char *)((long)a0 + 0x36);
  if (v6) { // branch-flip
    v7 = sub_7ea0(v10,(unsigned int)(a1 ^ 1) << 0xb);
    v2 = *(unsigned int *)((long)a0 + 0x3c);
  }
  else {
    v7 = 0;
    v2 = v1;
  }
  v4 = sub_5260(*(unsigned int *)&a0[7],v2);
  v3 = dat_1511a;
  if (!v4)
    __assert_fail("valid_file_spec (f)","src/tail.c",0x3e2,"recheck"); // no-return
  if (dat_1511a) { // branch-flip
    if (v7 != -1) { // branch-flip
      *(char *)((long)a0 + 0x36) = 1;
      if ((dat_1510c) || ((v10 = (char *)*a0, lstat(v10,&v11) || ((v14 & 0xf000) != 0xa000)))) goto label_53ca;
    }
    else {
      *(char *)((long)a0 + 0x36) = 0;
      v10 = (char *)*a0;
      if (dat_1510c) {
        *(int *)((long)a0 + 0x3c) = *__errno_location();
        goto label_54d9;
      }
      if ((lstat(v10,&v11)) || ((v14 & 0xf000) != 0xa000)) goto label_5321;
    }
  }
  else {
    *(char *)((long)a0 + 0x36) = 1;
    if ((dat_1510c) || ((v10 = (char *)*a0, lstat(v10,&v11) || ((v14 & 0xf000) != 0xa000)))) {
      if (v7 != -1) {
label_53ca:
        if (0 <= fstat(v7,&v11)) {
          if ((!((v14 & 0xf000) - 0x1000 & 0xffffe000)) || ((v14 & 0xb000) == 0x8000)) {
            v9 = *a0;
            v5 = sub_4a40(v7,sub_5050(v9));
            *(char *)((long)a0 + 0x35) = v5;
            if ((v5) && (!dat_1510c)) {
              *(unsigned int *)((long)a0 + 0x3c) = 0xffffffff;
              v9 = *a0;
              v9 = sub_b410(4,sub_5050(v9));
              error(0,0,dcgettext(NULL,"%s has been replaced with an untailable remote file",5),v9);
              *(unsigned short *)((long)a0 + 0x34) = 0x101;
              goto label_5348;
            }
            *(unsigned int *)((long)a0 + 0x3c) = 0;
            if (v1 & 0xfffffffd) { // branch-flip
              if (*(int *)&a0[7] != -1)
                __assert_fail("f->fd == -1","src/tail.c",0x42f,"recheck"); // no-return
              v9 = *a0;
              v9 = sub_b410(4,sub_5050(v9));
              v10 = "%s has become accessible";
            }
            else {
              v9 = *a0;
              if (*(int *)&a0[7] != -1) {
                if ((a0[5] == v13) && (a0[4] == v11)) {
                  sub_3b30(v7,sub_5050(v9));
                  return;
                }
                v9 = sub_b410(4,sub_5050(v9));
                error(0,0,dcgettext(NULL,"%s has been replaced;  following new file",5),v9);
                v9 = sub_5050(*a0);
                sub_3b30(*(unsigned int *)&a0[7],v9);
                goto label_5484;
              }
              v9 = sub_b410(4,sub_5050(v9));
              v10 = "%s has appeared;  following new file";
            }
            error(0,0,dcgettext(NULL,v10,5),v9);
label_5484:
            v12 = (unsigned long)a1;
            if (!v6)
              v12 = 0xffffffff;
            sub_3830(a0,v7,0,&v11,v12);
            v9 = *a0;
            sub_41c0(v7,0,0,sub_5050(v9));
            return;
          }
          *(unsigned int *)((long)a0 + 0x3c) = 0xffffffff;
          *(char *)((long)a0 + 0x36) = 0;
          if ((v3) && (dat_15020 == 1)) {
            *(char *)((long)a0 + 0x34) = 0;
            if (v5) // branch-flip
              v10 = "";
            else {
              v10 = "";
              if (v1 == 0xffffffff) goto label_5348;
            }
          }
          else {
            *(char *)((long)a0 + 0x34) = 1;
            if ((!v5) && (v1 == 0xffffffff)) goto label_5348;
            v10 = dcgettext(NULL,"; giving up on this name",5);
          }
          v9 = *a0;
          v9 = sub_b410(4,sub_5050(v9));
          error(0,0,dcgettext(NULL,"%s has been replaced with an untailable file%s",5),v9,v10);
          goto label_5348;
        }
      }
label_5321:
      v8 = (unsigned int *)__errno_location();
      v10 = (char *)*a0;
      v2 = *v8;
      *(unsigned int *)((long)a0 + 0x3c) = v2;
      if (*(char *)((long)a0 + 0x36)) {
        if (v2 != v1) {
          v9 = sub_b510(0,3,sub_5050(v10));
          error(0,*v8,"%s",v9);
        }
        goto label_5348;
      }
label_54d9:
      if (v5) {
        v9 = sub_b410(4,sub_5050(v10));
        v10 = dcgettext(NULL,"%s has become inaccessible",5);
        error(0,*(int *)((long)a0 + 0x3c),v10,v9);
      }
      goto label_5348;
    }
  }
  *(unsigned int *)((long)a0 + 0x3c) = 0xffffffff;
  v9 = *a0;
  *(char *)((long)a0 + 0x34) = 1;
  v9 = sub_b410(4,sub_5050(v9));
  error(0,0,dcgettext(NULL,"%s has been replaced with an untailable symbolic link",5),v9);
label_5348:
  v9 = *a0;
  sub_3b30(v7,sub_5050(v9));
  v9 = sub_5050(*a0);
  sub_3b30(*(unsigned int *)&a0[7],v9);
  *(unsigned int *)&a0[7] = 0xffffffff;
  return;
}

// Function: sub_5880 @ 0x5880
void sub_5880(double a0,unsigned int a1,long *a2,long a3,long *a4) // return-dupe
{
  char v1;
  int v10;
  long v11;
  unsigned long v12; // rax
  long v13; // rax
  unsigned int *v14; // rax
  long *v15;
  unsigned long v16; // rax
  char *v17;
  long *v18; // rax
  int *v19;
  unsigned int v2;
  long *v20; // stack - 0x140
  unsigned int v21; // stack - 0xd8
  char v22 [68];
  char *v23;
  unsigned long v24;
  unsigned long v25; // stack - 0x1a8
  unsigned long v26; // stack - 0x198
  int *v27; // stack - 0x180
  long v28; // stack - 0x168
  int v29; // stack - 0x15c
  unsigned int v3;
  int v30; // stack - 0xf4
  unsigned short v31; // stack - 0xd4
  unsigned short v32; // stack - 0xd2
  long v33; // stack - 0xd0
  int v4;
  long v5;
  long v6;
  bool v7;
  bool v8;
  unsigned int v9; // eax
  
  v11 = sub_8850(a3,0,sub_3980,sub_39a0,0);
  if (!v11)
    sub_c360(); // no-return, return-dupe
  v8 = 0;
  v7 = 0;
  *a4 = v11;
  v25._0_4_ = dat_15020;
  v9 = 2;
  if (dat_15020 == 1)
    v9 = 0xc06;
  v24 = 0;
  v15 = a2;
  do {
    if (!*(char *)((long)v15 + 0x34)) {
      v17 = (char *)*v15;
      v12 = strlen(v17);
      *(unsigned int *)((long)v15 + 0x44) = 0xffffffff;
      if (v24 < v12)
        v24 = v12;
      if ((int)v25 == 1) {
        v13 = sub_7bb0(v17);
        v1 = v17[v13];
        v23 = ".";
        v15[10] = sub_7990(v17) - (long)v17;
        v17[v13] = '\0';
        if (v13)
          v23 = (char *)*v15;
        *(unsigned int *)&v15[9] = inotify_add_watch(a1,v23,0x784);
        *(char *)(*v15 + v13) = v1;
        if ((int)v15[9] < 0) {
          v19 = __errno_location();
          if (*v19 != 0x1c) {
            v16 = sub_b410(4,*v15);
            v17 = dcgettext(NULL,"cannot watch parent directory of %s",5);
            error(0,*v19,v17,v16); // tail-call
            return;
          }
label_5b10:
          v17 = "inotify resources exhausted";
          error(0,0,dcgettext(NULL,v17,5)); // return-dupe, tail-call
          return;
        }
        v17 = (char *)*v15;
      }
      v10 = inotify_add_watch(a1,v17,v9);
      *(int *)((long)v15 + 0x44) = v10;
      if (0 <= v10) { // branch-flip
        if (!sub_8e70(v11,v15)) {
          sub_c360();
        }
        v7 = 1;
        v25._0_4_ = dat_15020;
      }
      else {
        if ((int)v15[7] != -1)
          v8 = 1;
        v14 = (unsigned int *)__errno_location();
        if ((*v14 & 0xffffffef) == 0xc) goto label_5b10;
        if (*(unsigned int *)((long)v15 + 0x3c) != *v14) {
          v16 = sub_b410(4,*v15);
          v17 = dcgettext(NULL,"cannot watch %s",5);
          error(0,*v14,v17,v16);
          v25._0_4_ = dat_15020;
        }
      }
    }
    v15 = &v15[0xc];
  } while (v15 != &a2[a3 * 0xc]);
  if ((int)v25 == 2) {
    if (v8)
      return;
    if (!v7)
      exit(1); // no-return
  }
  v20 = &a2[a3 * 0xc + -0xc];
  v15 = a2;
  do {
    if (!*(char *)((long)v15 + 0x34)) {
      if (dat_15020 != 1) { // branch-flip
        if ((((int)v15[7] != -1) && (v17 = (char *)*v15, !stat(v17,&v21))) && ((v15[4] != CONCAT26(v32,CONCAT24(v31,v21)) || (v15[5] != v33)))) {
          v11 = *v15;
          v16 = sub_b410(4,sub_5050(v11));
          v17 = dcgettext(NULL,"%s was replaced",5);
          error(0,*__errno_location(),v17,v16);
          return;
        }
      }
      else {
        sub_5270(v15,0);
      }
      sub_5090(v15,&v20);
    }
    v15 = &v15[0xc];
  } while (v15 != &a2[a3 * 0xc]);
  v28 = v24 + 0x11;
  v7 = 0;
  v26 = 0;
  v27 = (int *)sub_bfa0(v28);
  v25 = 0;
  v29 = 3;
label_5bf7:
  do {
    if (((dat_15020 == 1) && (!dat_1511a)) && (!sub_83f0(v11))) {
      error(1,0,dcgettext(NULL,"no files remaining",5));
      return;
    }
    if (v26 <= v25) {
      do {
        if (dat_15110) { // branch-flip
          if (v7)
            exit(0); // no-return
          v10 = kill(dat_15110,0);
          if ((v10) && (*__errno_location() != 1)) {
            v10 = 0;
            v7 = 1;
          }
          else {
            v10 = 0;
            if (0.0 < a0) {
              if (dat_f4e0 <= a0) goto label_5e10;
              v10 = (int)(dat_f4e8 * a0);
              v10 = (unsigned int)((double)v10 < dat_f4e8 * a0) + v10;
            }
          }
        }
        else {
label_5e10:
          v10 = -1;
        }
        v31 = 1;
        v33 = 1;
        v21 = a1;
        v10 = poll(&v21,(unsigned long)dat_15117 + 1,v10);
      } while (!v10);
      if (v10 < 0) {
        v17 = dcgettext(NULL,"error waiting for inotify and output events",5);
        error(1,*__errno_location(),v17);
        return;
      }
      if (v33._6_2_)
        sub_4bd0(); // no-return
      v26 = sub_b6e0(a1,v27,v28);
      if (v26) {
        v13 = 0x10;
        v19 = v27;
        if (v26 != 0xffffffffffffffff) goto label_5c1f;
        if (*__errno_location() != 0x16) {
label_5ed4:
          v17 = dcgettext(NULL,"error reading inotify event",5);
          error(1,*__errno_location(),v17);
          return;
        }
      }
      if (!v29) goto label_5ed4;
      v29 -= 1;
      v28 *= 2;
      v26 = 0;
      v27 = (int *)sub_bff0(v27,v28);
      v25 = 0;
      goto label_5bf7;
    }
    v13 = v25 + 0x10;
    v19 = (int *)((long)v27 + v25);
label_5c1f:
    v2 = v19[3];
    v10 = *v19;
    v25 = v13 + (unsigned long)v2;
    v3 = v19[1];
    if (v3 & 0x400) { // branch-flip
      if (v2) goto label_5c55;
      v15 = &a2[9];
      do {
        if ((int)*v15 == v10) {
          v17 = "directory containing watched file was removed";
          error(0,0,dcgettext(NULL,v17,5));
          return;
        }
        v15 = &v15[0xc];
      } while (&(&a2[9])[a3 * 0xc] != v15);
label_5e28:
      v30 = v10;
      v15 = (long *)sub_85f0(v11,v22);
      if (v15) {
        v2 = v19[1];
label_5cde:
        if (v2 & 0xe04) { // branch-flip
          if (v2 & 0x400) {
            inotify_rm_watch(a1,*(unsigned int *)((long)v15 + 0x44));
            sub_8ed0(v11,v15);
          }
          sub_5270(v15,0);
        }
        else {
          sub_5090(v15,&v20);
        }
      }
    }
    else {
      if (!v2) goto label_5e28;
label_5c55:
      v13 = 0;
      v15 = a2;
      do {
        if (((int)v15[9] == v10) && (v5 = v15[10], v6 = *v15, !strcmp((char *)&v19[4],(char *)(v5 + v6)))) {
          if (a3 == v13) goto label_5bf7;
          v15 = &a2[v13 * 0xc];
          if (!(v3 & 0x200)) {
            v10 = inotify_add_watch(a1,*v15,v9);
            if (v10 <= -1) {
              v14 = (unsigned int *)__errno_location();
              if ((*v14 & 0xffffffef) == 0xc) goto label_5b10;
              v16 = sub_b410(4,*v15);
              v17 = dcgettext(NULL,"cannot watch %s",5);
              error(0,*v14,v17,v16);
            }
            v4 = *(int *)((long)v15 + 0x44);
            if (v4 != v10) { // branch-flip
              if (0 <= v4) {
                inotify_rm_watch(a1);
                sub_8ed0(v11,v15);
              }
            }
            else if (0 <= v4) goto label_5cc7;
            *(int *)((long)v15 + 0x44) = v10;
            if (v10 == -1) goto label_5bf7;
            v18 = (long *)sub_8ed0(v11,v15);
            if ((v18) && (v15 != v18)) {
              if (dat_15020 == 1)
                sub_5270(v18,0);
              *(unsigned int *)((long)v18 + 0x44) = 0xffffffff;
              v16 = sub_5050(*v18);
              sub_3b30((int)v18[7],v16);
            }
            if (!sub_8e70(v11,v15)) {
              sub_c360();
            }
          }
label_5cc7:
          if (dat_15020 == 1)
            sub_5270(v15,0);
          v2 = v19[1];
          goto label_5cde;
        }
        v13 += 1;
        v15 = &v15[0xc];
      } while (a3 != v13);
    }
  } while( true );
}

// Function: sub_6220 @ 0x6220
void sub_6220(unsigned long a0,unsigned long *a1,long a2)
{
  unsigned int v1;
  int v10; // eax
  unsigned long v11; // rax
  char *v12; // rax
  unsigned long v13;
  int *v14; // rax
  long v15; // rax
  unsigned long *v16;
  unsigned long *v17; // rbx
  char v18 [24];
  long v19; // rbp
  unsigned long v2;
  unsigned int v20;
  bool v21;
  char v22 [16];
  long v23; // stack - 0x100
  unsigned int v24; // stack - 0xc0
  long v25; // stack - 0xa8
  unsigned long v3;
  bool v4;
  bool v5;
  bool v6;
  unsigned int v7; // eax
  unsigned int v8;
  int v9;
  
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
        v9 = *(int *)&v16[7];
        if (v9 <= -1) break;
        v11 = sub_5050(*v16);
        v1 = *(unsigned int *)&v16[6];
        v8 = v20;
        if (*(unsigned int *)&v16[8] != v20) {
          v7 = sub_7d50(v9,3);
          if (v20) { // branch-flip
            if (0 <= (int)v7) goto label_6309;
          }
          else if ((0 <= (int)v7) && ((v7 == (v7 | 0x800) || (sub_7d50(v9,4) != -1)))) {
label_6309:
            *(unsigned int *)&v16[8] = v20;
            goto label_6310;
          }
          v14 = __errno_location();
          if (((*(unsigned int *)&v16[6] & 0xf000) != 0x8000) || (*v14 != 1)) {
            v11 = sub_b510(0,3,v11);
            v12 = dcgettext(NULL,"%s: cannot change nonblocking mode",5);
            error(1,*v14,v12,v11);
            return;
          }
          v8 = *(unsigned int *)&v16[8];
        }
label_6310:
        v15 = -2;
        if (v8) { // branch-flip
label_631f:
          v15 = sub_3f80(0,v11,v9,v15);
          v16[1] = v16[1] + v15;
          v6 = (bool)(v6 | v15 != 0);
        }
        else {
          v10 = fstat(v9,v18);
          v8 = v24;
          if (!v10) {
            if ((*(unsigned int *)&v16[6] == v24) && (((v24 & 0xf000) != 0x8000 || (v16[1] == v25)))) {
              v22 = sub_b7b0(v18);
              v13 = v16[2];
              v2 = v16[3];
              if (!sub_b800(v13,v2,SUB168(v22,0),SUB168(v22,8))) {
                v3 = v16[0xb];
                v16[0xb] = v3 + 1;
                if ((dat_15018 <= v3) && (dat_15020 == 1)) {
                  sub_5270(v16,*(int *)&v16[8] != 0);
                  v16[0xb] = 0;
                }
                goto label_633a;
              }
            }
            v22 = sub_b7b0(v18);
            v16[0xb] = 0;
            v16[2] = SUB168(v22,0);
            v16[3] = SUB168(v22,8);
            *(unsigned int *)&v16[6] = v8;
            if (((v1 & 0xf000) == 0x8000) && (v25 < (long)v16[1])) {
              v13 = sub_b510(0,3,v11);
              error(0,0,dcgettext(NULL,"%s: file truncated",5),v13);
              sub_41c0(v9,0,0,v11);
              v16[1] = 0;
            }
            if ((v23 != v19) && (dat_15115))
              sub_3a10(v11);
            v23 = v19;
            if (*(int *)&v16[8]) // branch-flip
              v15 = -2;
            else {
              v15 = -1;
              if (((v1 & 0xf000) == 0x8000) && (*(char *)((long)v16 + 0x35)))
                v15 = v25 - v16[1];
            }
            goto label_631f;
          }
          *(unsigned int *)&v16[7] = 0xffffffff;
          v14 = __errno_location();
          *(int *)((long)v16 + 0x3c) = *v14;
          v11 = sub_b510(0,3,v11);
          error(0,*v14,"%s",v11);
          close(v9);
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
      v9 = fflush_unlocked(stdout);
      if (v9) {
        v12 = dcgettext(NULL,"write error",5);
        error(1,*__errno_location(),v12);
        return;
      }
      sub_4bf0();
      if (!v6) {
        if (v5)
          return;
        if (((dat_15110) && (v9 = kill(dat_15110,0), v9)) && (*__errno_location() != 1))
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

// Function: sub_6810 @ 0x6810
bool sub_6810(unsigned int a0)
{
  return (a0 & 0xd000) == 0x8000;
}

// Function: sub_6820 @ 0x6820
unsigned long sub_6820(unsigned long a0,int a1,unsigned long a2,long *a3) // early-return
{
  unsigned int v1; // eax
  long v10; // stack - 0x90
  long v2; // rax
  unsigned long v3; // rax
  char *v4; // rax
  unsigned long v5; // rax
  long v6;
  char v7 [24];
  unsigned int v8; // stack - 0xb0
  long v9; // stack - 0x98
  
  if (fstat(a1,v7)) {
    v3 = sub_b410(4,a0);
    v4 = dcgettext(NULL,"cannot fstat %s",5);
    error(0,*__errno_location(),v4,v3);
    return 0;
  }
  if (dat_15116) {
    if (((dat_1510d) || ((long)a2 <= -1)) || ((((v8 & 0xf000) != 0x8000 || ((long)sub_41c0(a1,a2,1,a0) <= -1)) && (lseek(a1,a2,1) == -1)))) {
      v1 = sub_40c0(a0,a1,a2,a3);
      a2 = 0xffffffffffffffff;
      if (v1)
        return (unsigned long)(v1 >> 0x1f);
    }
    else {
      *a3 = *a3 + a2;
      a2 = 0xffffffffffffffff;
    }
label_68c0:
    v2 = sub_3f80(0,a0,a1,a2);
    *a3 = *a3 + v2;
    return 1;
  }
  if ((!dat_1510d) && (0 <= (long)a2)) {
    if (sub_6810(v8)) { // branch-flip
      v2 = -1;
      v6 = v9;
    }
    else {
      v2 = lseek(a1,-a2,2);
      if (v2 == -1) {
        v5 = sub_4510(a0,a1,a2,a3); // return-dupe
        return v5;
      }
      v6 = v2 + a2;
    }
    if (0x2000000000000000 <= (unsigned long)(v10 - 1U))
      v10 = 0x200;
    if (v10 < v6) {
      if (v2 == -1)
        v2 = sub_41c0(a1,0,1,a0);
      if ((v2 < v6) && (a2 < (unsigned long)(v6 - v2))) {
        v2 = v6 - a2;
        sub_41c0(a1,v2,0,a0);
      }
      *a3 = v2;
      goto label_68c0;
    }
  }
  v5 = sub_4510(a0,a1,a2,a3);
  return v5;
}

// Function: sub_6a90 @ 0x6a90
void sub_6a90(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long *a3)
{
  *a3 = 0;
  if (dat_15119) {
    sub_4c60(); // tail-call
    return;
  }
  sub_6820(); // tail-call
}

// Function: sub_6ab0 @ 0x6ab0
unsigned int sub_6ab0(unsigned long *a0,unsigned long a1)
{
  unsigned char v1;
  unsigned int v10; // stack - 0xc0
  int v2;
  unsigned int v3; // eax
  int v4; // eax
  int *v5; // rax
  unsigned long v6;
  char *v7;
  unsigned long v8; // stack - 0xe0
  char v9 [24];
  
  v7 = (char *)*a0;
  v2 = strcmp(v7,"-");
  if (v2) { // branch-flip
    v4 = sub_7ea0(v7,0);
    v1 = dat_1511a;
    if (!dat_1511a) {
      *(char *)((long)a0 + 0x36) = 1;
      v7 = (char *)*a0;
      if (v4 == -1) {
label_6c50:
        v5 = __errno_location();
        if (dat_15118) {
          *(unsigned int *)&a0[7] = 0xffffffff;
          v2 = *v5;
          *(unsigned char *)((long)a0 + 0x34) = v1 ^ 1;
          *(int *)((long)a0 + 0x3c) = v2;
          a0[5] = 0;
          a0[4] = 0;
        }
        v6 = *a0;
        v6 = sub_b410(4,sub_5050(v6));
        v7 = dcgettext(NULL,"cannot open %s for reading",5);
        error(0,*v5,v7,v6);
        return 0;
      }
      goto label_6b04;
    }
    v7 = (char *)*a0;
    if (v4 == -1) {
      *(char *)((long)a0 + 0x36) = 0;
      goto label_6c50;
    }
  }
  else {
    dat_1510e = 1;
    v4 = 0;
  }
  *(char *)((long)a0 + 0x36) = 1;
label_6b04:
  if (dat_15115) {
    sub_3a10(sub_5050(v7));
    v7 = (char *)*a0;
  }
  v3 = sub_6a90(sub_5050(v7),v4,a1,&v8);
  if (dat_15118) { // branch-flip
    *(unsigned int *)((long)a0 + 0x3c) = (v3 & 0xff) - 1;
    if (0 <= fstat(v4,v9)) { // branch-flip
      if (((v10 & 0xf000) - 0x1000 & 0xffffe000) && ((v10 & 0xb000) != 0x8000)) {
        *(unsigned int *)((long)a0 + 0x3c) = 0xffffffff;
        v7 = "";
        *(char *)((long)a0 + 0x36) = 0;
        *(unsigned char *)((long)a0 + 0x34) = dat_1511a ^ 1;
        if (dat_1511a ^ 1)
          v7 = dcgettext(NULL,"; giving up on this name",5);
        v6 = *a0;
        v6 = sub_b510(0,3,sub_5050(v6));
        error(0,0,dcgettext(NULL,"%s: cannot follow end of this type of file%s",5),v6,v7);
      }
      else if ((char)v3) {
        sub_3830(a0,v4,v8,v9,-(unsigned int)(v2 == 0) | 1);
        v6 = sub_5050(*a0);
        *(char *)((long)a0 + 0x35) = sub_4a40(v4,v6);
        return v3;
      }
    }
    else {
      v5 = __errno_location();
      v6 = *a0;
      *(int *)((long)a0 + 0x3c) = *v5;
      v6 = sub_b410(4,sub_5050(v6));
      v7 = dcgettext(NULL,"error reading %s",5);
      error(0,*v5,v7,v6);
    }
    v6 = *a0;
    v3 = 0;
    *(unsigned char *)((long)a0 + 0x34) = dat_1511a ^ 1;
    sub_3b30(v4,sub_5050(v6));
    *(unsigned int *)&a0[7] = 0xffffffff;
  }
  else if ((v2) && (close(v4))) {
    v6 = *a0;
    v6 = sub_b410(4,sub_5050(v6));
    v7 = dcgettext(NULL,"error reading %s",5);
    v3 = 0;
    error(0,*__errno_location(),v7,v6);
  }
  return v3;
}

// Function: sub_6e60 @ 0x6e60
void sub_6e60(int a0)
{
  void *v1;
  unsigned long v2;
  
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

// Function: sub_7080 @ 0x7080
void sub_7080(int a0,char **a1,unsigned long *a2,unsigned int *a3,double *a4)
{
  char v1; // al
  int v2; // eax
  long v3; // rax
  unsigned long v4; // rax
  double v5; // stack - 0x48
  char *v6; // rax
  
  while( true ) {
    v2 = getopt_long(a0,a1,"c:n:fFqs:vz0123456789",(void *)0x14900,NULL);
    if (v2 == -1) break;
    if (0x85 < v2)
      sub_6e60(1); // no-return, return-dupe
    if (v2 <= 0x2f) {
      if (v2 == -0x83) {
        sub_be30(stdout,"tail","GNU coreutils",dat_15030,"Paul Rubin","David MacKenzie","Ian Lance Taylor","Jim Meyering",0,CONCAT44(dat_4,v2));
        exit(0); // no-return
      }
      if (v2 == -0x82)
        sub_6e60(0); // no-return
      sub_6e60(1);
    }
    switch(v2) {
      case 0x30:
      case 0x31:
      case 0x32:
      case 0x33:
      case 0x34:
      case 0x35:
      case 0x36:
      case 0x37:
      case 0x38:
      case 0x39:
        error(1,0,dcgettext(NULL,"option used in invalid context -- %c",5),v2);
        return;
      default:
        goto label_74d0;
      case 0x46:
        dat_15118 = '\x01';
        dat_15020 = 1;
        dat_1511a = '\x01';
        break;
      case 99:
      case 0x6e:
        dat_15119 = v2 == 0x6e;
        if (*optarg != '+') { // branch-flip
          if (*optarg == '-')
            optarg = &optarg[1];
        }
        else {
          dat_15116 = 1;
        }
        if (v2 != 0x6e) // branch-flip
          v6 = dcgettext(NULL,"invalid number of bytes",5);
        else {
          v6 = dcgettext(NULL,"invalid number of lines",5);
        }
        v4 = sub_c4b0(optarg,0,0xffffffffffffffff,"bkKmMGTPEZY0",v6,0);
        *a2 = v4;
        break;
      case 0x66:
      case 0x84:
        dat_15118 = '\x01';
        if (optarg) { // branch-flip
          v3 = sub_78a0("--follow",optarg,0x14b00,0xf4d8,4,dat_15038,1);
          dat_15020 = *(int *)(v3 * 4 + 0xf4d8);
        }
        else {
          dat_15020 = 2;
        }
        break;
      case 0x71:
        *a3 = 2;
        break;
      case 0x73:
        v1 = sub_c590(optarg,0,&v5,sub_7a20);
        if ((!v1) || (v5 < 0.0)) {
          v4 = sub_b6d0(optarg);
          error(1,0,dcgettext(NULL,"invalid number of seconds: %s",5),v4);
          return;
        }
        *a4 = v5;
        break;
      case 0x76:
        *a3 = 1;
        break;
      case 0x7a:
        dat_15114 = 0;
        break;
      case 0x80:
        dat_1511a = '\x01';
        break;
      case 0x81:
        dat_15018 = sub_c4b0(optarg,0,0xffffffffffffffff,"",dcgettext(NULL,"invalid maximum number of unchanged stats between opens",5),0);
        break;
      case 0x82:
        dat_15110 = sub_c4b0(optarg,0,0x7fffffff,"",dcgettext(NULL,"invalid PID",5),0);
        break;
      case 0x83:
        dat_1510d = 1;
        break;
      case 0x85:
        dat_1510c = 1;
      
    }
  }
  if (dat_1511a) {
    if (dat_15118) { // branch-flip
      if (dat_15020 != 2) {
        if (!dat_15110)
          return;
        goto label_73cd;
      }
      v6 = "warning: --retry only effective for the initial open";
    }
    else {
      dat_1511a = '\0';
      v6 = "warning: --retry ignored; --retry is useful only when following";
    }
    error(0,0,dcgettext(NULL,v6,5));
  }
  if (!dat_15110)
    return;
  if (!dat_15118) {
    error(0,0,dcgettext(NULL,"warning: PID ignored; --pid=PID is useful only when following",5)); // tail-call
    return;
  }
label_73cd:
  v2 = kill(dat_15110,0);
  if ((v2) && (*__errno_location() == 0x26)) {
    error(0,0,dcgettext(NULL,"warning: --pid=PID is not supported on this system",5));
    dat_15110 = 0;
  }
}

// Function: sub_7590 @ 0x7590
void sub_7590(void)
{
  sub_6e60(1); // tail-call, no-return
}

// Function: sub_75a0 @ 0x75a0
unsigned long sub_75a0(char *a0,long *a1,void *a2,unsigned long a3)
{
  bool v1;
  unsigned long v2;
  int v3; // eax
  unsigned long v4; // rax
  long v5; // rbx
  void *v6;
  char *v7;
  long v8; // stack - 0x58
  
  v4 = strlen(a0);
  v2 = v4;
  v7 = (char *)*a1;
  if (v7) { // branch-flip
    v1 = 0;
    v5 = 0;
    v8 = -1;
    v6 = a2;
    do {
      v3 = strncmp(v7,a0,v2);
      if (!v3) {
        v4 = strlen(v7);
        if (v4 == v2) {
          v4 = v5;
          return v4;
        }
        if (v8 != -1) { // branch-flip
          if (a2) { // branch-flip
            v3 = memcmp((void *)(v8 * a3 + (long)a2),v6,a3);
            if (v3)
              v1 = 1;
          }
          else {
            v1 = 1;
          }
        }
        else {
          v8 = v5;
        }
      }
      v5 += 1;
      v6 = (void *)((long)v6 + a3);
      v7 = (char *)a1[v5];
    } while (v7);
    v4 = -2;
    if (!v1)
      v4 = v8;
    v8 = v4;
  }
  else {
    v8 = -1;
  }
  v4 = v8;
  return v4;
}

// Function: sub_76b0 @ 0x76b0
long sub_76b0(char *a0,long *a1) // return-dupe
{
  long v1; // rbx
  char *v2;
  
  v2 = (char *)*a1;
  if (!v2)
    return -1;
  v1 = 0;
  do {
    if (!strcmp(v2,a0))
      return v1;
    v1 += 1;
    v2 = (char *)a1[v1];
  } while (v2);
  return -1;
}

// Function: sub_7710 @ 0x7710
void sub_7710(unsigned long a0,unsigned long a1,long a2)
{
  char *v1; // rax
  unsigned long v2; // rax
  
  if (a2 != -1) // branch-flip
    v1 = dcgettext(NULL,"ambiguous argument %s for %s",5);
  else {
    v1 = dcgettext(NULL,"invalid argument %s for %s",5);
  }
  v2 = sub_b6c0(1,a0);
  error(0,0,v1,sub_b340(0,8,a1),v2); // tail-call
}

// Function: sub_77a0 @ 0x77a0
void sub_77a0(long *a0,void *a1,unsigned long a2)
{
  long v1;
  void *v2;
  long v3;
  void *v4;
  
  v2 = stderr;
  v4 = NULL;
  fputs_unlocked(dcgettext(NULL,"Valid arguments are:",5),v2);
  v3 = 0;
  v1 = *a0;
  while (v1) {
    if ((v3) && (!memcmp(v4,a1,a2)))
      __fprintf_chk(stderr,1,", %s",sub_b6d0(v1));
    else {
      __fprintf_chk(stderr,1,"\n  - %s",sub_b6d0(v1));
      v4 = a1;
    }
    v3 += 1;
    a1 = (void *)((long)a1 + a2);
    v1 = a0[v3];
  }
  putc_unlocked(10,stderr); // tail-call
}

// Function: sub_78a0 @ 0x78a0
long sub_78a0(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,void *a5,char a6) // early-return
{
  long v1; // rax
  
  if (a6) // branch-flip
    v1 = sub_75a0(a1,a2,a3,a4);
  else {
    v1 = sub_76b0(a1,a2);
  }
  if (0 <= v1)
    return v1;
  sub_7710(a0,a1,v1);
  sub_77a0(a2,a3,a4);
  (*a5)();
  return -1;
}

// Function: sub_7930 @ 0x7930
long sub_7930(void *a0,long *a1,void *a2,unsigned long a3) // return-dupe
{
  long v1;
  
  v1 = *a1;
  if (!v1)
    return 0;
  do {
    a1 = &a1[1];
    if (!memcmp(a0,a2,a3))
      return v1;
    v1 = *a1;
    a2 = (void *)((long)a2 + a3);
  } while (v1);
  return 0;
}

// Function: sub_7990 @ 0x7990
char * sub_7990(char *a0)
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

// Function: sub_79f0 @ 0x79f0
void sub_79f0(char *a0)
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

// Function: sub_7a20 @ 0x7a20
unsigned long sub_7a20(unsigned long a0,unsigned long *a1)
{
  int v1;
  int *v2; // rax
  char *v3; // stack - 0x40
  char *v4; // stack - 0x38
  unsigned long v5; // xmm0_qa
  unsigned long v6; // stack - 0x50
  
  v6 = strtod(a0,&v3);
  if (*v3) {
    v2 = __errno_location();
    v1 = *v2;
    v5 = sub_cdc0(a0,&v4);
    if (v4 <= v3) // branch-flip
      *v2 = v1;
    else {
      v6 = v5;
      v3 = v4;
    }
  }
  if (a1)
    *a1 = v3;
  return v6;
}

// Function: sub_7ae0 @ 0x7ae0
void sub_7ae0(unsigned long a0)
{
  dat_15128 = a0;
}

// Function: sub_7af0 @ 0x7af0
void sub_7af0(char a0)
{
  dat_15120 = a0;
}

// Function: sub_7b00 @ 0x7b00
void sub_7b00(void)
{
  int v1; // eax
  int *v2; // rax
  char *v3; // rax
  unsigned long v4; // rax
  
  v1 = sub_ce10(stdout);
  if (v1) {
    v2 = __errno_location();
    if ((!dat_15120) || (*v2 != 0x20)) {
      v3 = dcgettext(NULL,"write error",5);
      if (dat_15128) { // branch-flip
        v4 = sub_b4f0(dat_15128);
        error(0,*v2,"%s: %s",v4,v3);
      }
      else {
        error(0,*v2,"%s",v3);
      }
      _exit(dat_15040); // return-dupe, no-return
    }
  }
  v1 = sub_ce10(stderr);
  if (!v1)
    return;
  _exit(dat_15040);
}

// Function: sub_7bb0 @ 0x7bb0
void sub_7bb0(char *a0)
{
  char v1;
  long v2;
  unsigned long v3;
  
  v1 = *a0;
  v3 = sub_7990(a0) - (long)a0;
  do {
    if ((v1 == '/') >= v3) break;
    v2 = v3 - 1;
    v3 -= 1;
  } while (a0[v2] == '/');
}

// Function: sub_7bf0 @ 0x7bf0
char * sub_7bf0(void *a0)
{
  unsigned long v1; // rax
  void *v2; // rax
  char *v3;
  
  v1 = sub_7bb0(a0);
  v2 = malloc(v1 + 1 + (unsigned long)(v1 == 0));
  v3 = NULL;
  if (v2) {
    v3 = memcpy(v2,a0,v1);
    if (!v1) {
      *v3 = 0x2e;
      v1 = 1;
    }
    v3[v1] = 0;
  }
  return v3;
}

// Function: sub_7c60 @ 0x7c60
void sub_7c60(int a0,unsigned long a1)
{
  fcntl(a0,0,a1 & 0xffffffff); // tail-call
}

// Function: sub_7c70 @ 0x7c70
int sub_7c70(unsigned long a0,unsigned long a1)
{
  int v1;
  int v2; // eax
  unsigned int v3; // eax
  int *v4; // rax
  
  if (0 <= dat_15130) { // branch-flip
    v2 = fcntl((int)a0,0x406,a1 & 0xffffffff);
    if ((0 <= v2) || (*__errno_location() != 0x16)) {
      dat_15130 = 1;
      return v2;
    }
    v2 = sub_7c60(a0 & 0xffffffff,a1 & 0xffffffff);
    if (v2 < 0)
      return v2;
    dat_15130 = -1;
  }
  else {
    v2 = sub_7c60(a0,a1);
    if (v2 < 0)
      return v2;
    if (dat_15130 != -1)
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

// Function: sub_7d50 @ 0x7d50
void sub_7d50(int a0,int a1,unsigned int a2)
{
  unsigned long v1;
  unsigned long v2; // stack - 0x28
  
  v2 = CONCAT44(v2._4_4_,a2);
  if (!a1) {
    sub_7c60(a0,a2);
    return;
  }
  if (a1 == 0x406) {
    sub_7c70(a0,a2);
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

// Function: sub_7ea0 @ 0x7ea0
void sub_7ea0(char *a0,unsigned int a1,unsigned long a2)
{
  unsigned long v1; // rdx
  
  v1 = 0;
  if (a1 & 0x40)
    v1 = a2 & 0xffffffff;
  sub_b890(open(a0,a1,v1));
}

// Function: sub_7f20 @ 0x7f20
unsigned long sub_7f20(unsigned long a0)
{
  char v1 [16];
  unsigned long v2; // rax
  unsigned long v3;
  unsigned long v4; // rdx
  unsigned long v5; // rsi
  long v6; // r8
  
  v1._8_8_ = 0;
  v1._0_8_ = a0;
  v2 = SUB168(v1._0_16_ * ZEXT816(0xaaaaaaaaaaaaaaab),0);
  v4 = a0 % 3;
  if ((10 <= a0) && (v4)) {
    v6 = 0x10;
    v5 = 9;
    v3 = 3;
    do {
      v3 += 2;
      v5 += v6;
      v2 = a0 / v3;
      v4 = a0 % v3;
      if (a0 <= v5) break;
      v6 += 8;
    } while (v4);
  }
  v3 = v2 >> 8;
  v2 = CONCAT71((undefined7)v3,v4 != 0);
  return v2;
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
unsigned long sub_8000(long a0) // return-dupe
{
  float v1;
  float *v2;
  float v3; // xmm1_da
  
  v2 = *(float **)(a0 + 0x28);
  if (v2 == (float *)0xf5d0)
    return 1;
  v1 = v2[2];
  if ((((dat_f5e4 < v1) && (v1 < dat_f5e8)) && (dat_f5ec < v2[3])) && (dat_f600 <= *v2)) {
    v3 = *v2 + dat_f5e4;
    if (((v3 < v2[1]) && (v2[1] <= dat_f5f0)) && (v3 < v1))
      return 1;
  }
  *(unsigned long *)(a0 + 0x28) = 0xf5d0;
  return 0;
}

// Function: sub_8090 @ 0x8090
void sub_8090(long a0,unsigned long *a1)
{
  unsigned long v1;
  
  v1 = *(unsigned long *)(a0 + 0x48);
  *a1 = 0;
  a1[1] = v1;
  *(unsigned long **)(a0 + 0x48) = a1;
}

// Function: sub_80b0 @ 0x80b0
long sub_80b0(long *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  v1 = (*(void *)a0[6])(a1,a0[2]);
  if (v1 < (unsigned long)a0[2])
    return v1 * 0x10 + *a0;
  abort(); // no-return
}

// Function: sub_80e0 @ 0x80e0
long sub_80e0(long a0,long a1,unsigned long *a2,char a3) // early-return
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
    v1 = *(void **)(a0 + 0x38);
    if (!(*v1)(a1)) {
      while( true ) {
        if (!(long *)v4[1])
          return 0;
        if ((*(long *)v4[1] == a1) || (v1 = *(void **)(a0 + 0x38), (*v1)(a1))) break;
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
void sub_81e0(long a0)
{
  if (*(long *)(a0 + 0x48)) {
    *(unsigned long *)(a0 + 0x48) = *(unsigned long *)(*(long *)(a0 + 0x48) + 8);
    return;
  }
  malloc(0x10); // tail-call
}

// Function: sub_8210 @ 0x8210
unsigned long sub_8210(long a0,unsigned long *a1,char a2) // return-dupe x2
{
  long v1;
  long *v2; // rax
  long *v3;
  long *v4;
  long *v5;
  long v6;
  
  v5 = (long *)*a1;
  if ((long *)a1[1] <= v5)
    return 1;
  do {
    while (v6 = *v5, !v6) {
label_8238:
      v5 = &v5[2];
      if ((long *)a1[1] <= v5)
        return 1;
    }
    v3 = (long *)v5[1];
    if ((long *)v5[1]) {
      do {
        while( true ) {
          v6 = *v3;
          v2 = (long *)sub_80b0(a0,v6);
          v4 = (long *)v3[1];
          if (*v2) break;
          *v2 = v6;
          *(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + 1;
          sub_8090(a0);
          v3 = v4;
          if (!v4) goto label_82a8;
        }
        v3[1] = v2[1];
        v2[1] = (long)v3;
        v3 = v4;
      } while (v4);
label_82a8:
      v6 = *v5;
    }
    v5[1] = 0;
    if (a2) goto label_8238;
    v3 = (long *)sub_80b0(a0,v6);
    if (*v3) { // branch-flip
      v4 = (long *)sub_81e0(a0);
      if (!v4)
        return 0;
      v1 = v3[1];
      *v4 = v6;
      v4[1] = v1;
      v3[1] = (long)v4;
    }
    else {
      *v3 = v6;
      *(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + 1;
    }
    *v5 = 0;
    v5 = &v5[2];
    a1[3] = a1[3] + -1;
    if ((long *)a1[1] <= v5)
      return 1;
  } while( true );
}

// Function: sub_8340 @ 0x8340
unsigned long sub_8340(float a0,unsigned long a1,char a2) // early-return, ternary
{
  unsigned long v1; // rax
  
  if (!a2) {
    a0 = (float)a1 / a0;
    if (dat_f5f4 <= a0)
      return 0;
    a1 = (a0 < dat_f5f8) ? (unsigned long)a0 : (long)(a0 - dat_f5f8) ^ 0x8000000000000000;
  }
  v1 = sub_7f90(a1);
  if ((!(v1 >> 0x3c & 1)) && (!(v1 >> 0x3d)))
    return v1;
  return 0;
}

// Function: sub_83d0 @ 0x83d0
unsigned long sub_83d0(long a0)
{
  return *(unsigned long *)(a0 + 0x10);
}

// Function: sub_83e0 @ 0x83e0
unsigned long sub_83e0(long a0)
{
  return *(unsigned long *)(a0 + 0x18);
}

// Function: sub_83f0 @ 0x83f0
unsigned long sub_83f0(long a0)
{
  return *(unsigned long *)(a0 + 0x20);
}

// Function: sub_8400 @ 0x8400
unsigned long sub_8400(unsigned long *a0)
{
  long v1;
  long *v2;
  unsigned long v3;
  unsigned long v4;
  
  v2 = (long *)*a0;
  v4 = 0;
  do {
    if ((long *)a0[1] <= v2)
      return v4;
    while (*v2) {
      v3 = 1;
      for (v1 = v2[1]; v1; v1 = *(long *)(v1 + 8)) {
        v3 += 1;
      }
      if (v4 < v3)
        v4 = v3;
      v2 = &v2[2];
      if ((long *)a0[1] <= v2)
        return v4;
    }
    v2 = &v2[2];
  } while( true );
}

// Function: sub_8460 @ 0x8460
bool sub_8460(unsigned long *a0)
{
  long v1;
  long *v2;
  long v3;
  long v4;
  
  v2 = (long *)*a0;
  v3 = 0;
  v4 = 0;
  do {
    while( true ) {
      if ((long *)a0[1] <= v2) {
        if (a0[3] != v4)
          return 0;
        return a0[4] == v3;
      }
      if (*v2) break;
label_8480:
      v2 = &v2[2];
    }
    v1 = v2[1];
    v4 += 1;
    v3 += 1;
    if (!v1) goto label_8480;
    do {
      v1 = *(long *)(v1 + 8);
      v3 += 1;
    } while (v1);
    v2 = &v2[2];
  } while( true );
}

// Function: sub_84d0 @ 0x84d0
void sub_84d0(unsigned long a0,void *a1)
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
  __fprintf_chk(a1,1,"# buckets used:    %lu (%.2f%%)\n",v3);
  __fprintf_chk(a1,1,"max bucket length: %lu\n",v4); // tail-call
}

// Function: sub_85f0 @ 0x85f0
long sub_85f0(long a0,long a1)
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
    v1 = *(void **)(a0 + 0x38);
    if ((*v1)(a1)) break;
    v2 = (long *)v2[1];
    if (!v2)
      return 0;
    v3 = *v2;
  }
  return *v2;
}

// Function: sub_8650 @ 0x8650
long sub_8650(unsigned long *a0)
{
  long *v1;
  
  if (!a0[4])
    return 0;
  v1 = (long *)*a0;
  while( true ) {
    if ((long *)a0[1] <= v1)
      abort(); // no-return
    if (*v1) break;
    v1 = &v1[2];
  }
  return *v1;
}

// Function: sub_86a0 @ 0x86a0
long sub_86a0(long a0,long a1)
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
    if (*(long **)(a0 + 8) <= v2)
      return 0;
  } while (!*v2);
  return *v2;
}

// Function: sub_8710 @ 0x8710
void sub_8710(unsigned long *a0,long a1,unsigned long a2)
{
  unsigned long v1; // rax
  long *v2;
  long *v3;
  
  v3 = (long *)*a0;
  v1 = 0;
  if ((long *)a0[1] <= v3)
    return;
  do {
    if (*v3) {
      v2 = v3;
      do {
        if (a2 <= v1)
          return;
        v1 += 1;
        *(long *)(a1 + -8 + v1 * 8) = *v2;
        v2 = (long *)v2[1];
      } while (v2);
    }
    v3 = &v3[2];
  } while (v3 < (long *)a0[1]);
}

// Function: sub_8760 @ 0x8760
long sub_8760(unsigned long *a0,void *a1,unsigned long a2)
{
  long *v1;
  long v2;
  long v3; // r12
  long *v4;
  
  v4 = (long *)*a0;
  if ((long *)a0[1] <= v4) // branch-flip
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
    } while (v4 < (long *)a0[1]);
  }
  return v3;
}

// Function: sub_87e0 @ 0x87e0
unsigned long sub_87e0(unsigned char *a0,unsigned long a1)
{
  unsigned char v1;
  unsigned long v2;
  
  v2 = 0;
  v1 = *a0;
  while (v1) {
    a0 = &a0[1];
    v2 = (v2 * 0x1f + (unsigned long)v1) % a1;
    v1 = *a0;
  }
  return v2;
}

// Function: sub_8820 @ 0x8820
void sub_8820(unsigned long *a0)
{
  *(char *)&a0[2] = 0;
  *a0 = 0x3f80000000000000;
  a0[1] = 0x3fb4fdf43f4ccccd;
}

// Function: sub_8850 @ 0x8850
unsigned long * sub_8850(unsigned long a0,long a1,void *a2,void *a3,unsigned long a4) // return-dupe
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
    a1 = 0xf5d0;
  v1[5] = a1;
  if (sub_8000(v1)) {
    v2 = sub_8340(*(unsigned int *)(a1 + 8),a0,*(char *)(a1 + 0x10));
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
void sub_8960(unsigned long *a0)
{
  unsigned long *v1;
  unsigned long v2;
  void *v3;
  unsigned long *v4;
  long *v5;
  long *v6; // r12
  
  v5 = (long *)*a0;
  if ((long *)*a0 < (long *)a0[1]) {
    do {
      while (!*v5) {
        v5 = &v5[2];
        if ((long *)a0[1] <= v5) goto label_89fb;
      }
      v3 = (void *)a0[8];
      v4 = (unsigned long *)v5[1];
      if ((unsigned long *)v5[1]) {
        while( true ) {
          if (v3) {
            (*v3)(*v4);
            v3 = (void *)a0[8];
          }
          v1 = (unsigned long *)v4[1];
          v2 = a0[9];
          *v4 = 0;
          v4[1] = v2;
          a0[9] = v4;
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
    } while (v6 < (long *)a0[1]);
  }
label_89fb:
  a0[3] = 0;
  a0[4] = 0;
  return;
}

// Function: sub_8a10 @ 0x8a10
void sub_8a10(unsigned long *a0)
{
  void *v1;
  void *v2;
  long *v3;
  long *v4;
  long v5;
  
  v4 = (long *)*a0;
  v3 = (long *)a0[1];
  if ((a0[8]) && (a0[4])) {
    if (v3 <= v4) goto label_8abc;
    do {
      while (v5 = *v4, !v5) {
        v4 = &v4[2];
        if (v3 <= v4) goto label_8a7f;
      }
      v3 = v4;
      while( true ) {
        (*(void *)a0[8])(v5);
        v3 = (long *)v3[1];
        if (!v3) break;
        v5 = *v3;
      }
      v3 = (long *)a0[1];
      v4 = &v4[2];
    } while (v4 < v3);
label_8a7f:
    v4 = (long *)*a0;
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
    } while (v4 < (long *)a0[1]);
  }
label_8abc:
  v2 = (void *)a0[9];
  while (v2) {
    v1 = *(void **)((long)v2 + 8);
    free(v2);
    v2 = v1;
  }
  free((void *)*a0);
  free(a0); // tail-call
}

// Function: sub_8b00 @ 0x8b00
unsigned int sub_8b00(unsigned long *a0,unsigned long a1) // return-dupe
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
  
  v4 = sub_8340(*(unsigned int *)(a0[5] + 8),a1,*(char *)(a0[5] + 0x10));
  if (!v4)
    return 0;
  if (a0[2] == v4)
    return 1;
  v2 = calloc(v4,0x10);
  if (!v2)
    return 0;
  v3 = (void *)((long)v2 + v4 * 0x10);
  v5 = 0;
  v7 = a0[5];
  v6 = 0;
  v8 = a0[6];
  v9 = a0[7];
  v10 = a0[8];
  v11 = a0[9];
  v1 = sub_8210(&v2,a0,0);
  if (!(char)v1) {
    a0[9] = v11;
    if ((sub_8210(a0,&v2,1)) && (sub_8210(a0,&v2,0))) {
      free(v2);
      return v1;
    }
    abort(); // no-return
  }
  free((void *)*a0);
  *a0 = v2;
  a0[1] = v3;
  a0[2] = v4;
  a0[3] = v5;
  a0[9] = v11;
  return v1;
}

// Function: sub_8c80 @ 0x8c80
unsigned long sub_8c80(long a0,long a1,long *a2) // ternary
{
  long v1;
  long *v2; // rax
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
    v6 = (float)*(unsigned long *)(a0 + 0x18);
    v5 = (float)*(unsigned long *)(a0 + 0x10);
    if (*(float *)(*(long *)(a0 + 0x28) + 8) * v5 < v6) {
      sub_8000(a0);
      v1 = *(long *)(a0 + 0x28);
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
      *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + 1;
      *(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + 1;
      return 1;
    }
    v2 = (long *)sub_81e0(a0);
    if (!v2)
      return 0xffffffff;
    *v2 = a1;
    v2[1] = v3[1];
    v3[1] = (long)v2;
    *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + 1;
    return 1;
  }
  abort();
}

// Function: sub_8e70 @ 0x8e70
unsigned long sub_8e70(unsigned long a0,unsigned long a1) // early-return
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
long sub_8ed0(long a0,unsigned long a1) // ternary x2
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
  *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + -1;
  if (!*v7) {
    v6 = *(long *)(a0 + 0x18) - 1;
    *(unsigned long *)(a0 + 0x18) = v6;
    v1 = (0 <= (long)v6) ? *(unsigned long *)(a0 + 0x10) : *(unsigned long *)(a0 + 0x10); // branch-flip
    v9 = (float)v6;
    v8 = (float)v1;
    if (v9 < **(float **)(a0 + 0x28) * v8) {
      sub_8000(a0);
      v2 = *(float **)(a0 + 0x28);
      if (v9 < *v2 * v8) {
        v8 = v8 * v2[1];
        if (!*(char *)&v2[4])
          v8 = v8 * v2[2];
        v6 = (dat_f5f8 <= v8) ? (long)(v8 - dat_f5f8) ^ 0x8000000000000000 : (unsigned long)v8;
        if (!sub_8b00(a0,v6)) {
          v4 = *(void **)(a0 + 0x48);
          while (v4) {
            v3 = *(void **)((long)v4 + 8);
            free(v4);
            v4 = v3;
          }
          *(unsigned long *)(a0 + 0x48) = 0;
        }
      }
    }
    return v5;
  }
  return v5;
}

// Function: sub_9060 @ 0x9060
void sub_9060(void)
{
  sub_8ed0(); // tail-call
}

// Function: sub_9070 @ 0x9070
char * sub_9070(unsigned long a0,long a1)
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

// Function: sub_9110 @ 0x9110
unsigned long sub_9110(void)
{
  char *v1; // rax
  unsigned long v2; // rax
  char *v3; // stack - 0x18
  unsigned long v4;
  
  v4 = 0x31069;
  v1 = getenv("_POSIX2_VERSION");
  if (((v1) && (*v1)) && (v2 = strtol(v1,&v3,10), !*v3)) {
    if (-0x80000000 <= (long)v2) { // branch-flip
      v4 = 0x7fffffff;
      if ((long)v2 <= 0x7fffffff)
        v4 = v2;
    }
    else {
      v4 = 0x80000000;
    }
  }
  return v4 & 0xffffffff;
}

// Function: sub_91a0 @ 0x91a0
void sub_91a0(char *a0)
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
    dat_15138 = a0;
    program_invocation_name = a0;
    return;
  }
  fputs("A NULL argv[0] was passed through an exec system call.\n",stderr);
  abort(); // no-return
}

// Function: sub_9240 @ 0x9240
int * sub_9240(int *a0,int a1)
{
  if (a1 != 10) {
    *a0 = a1;
    a0[1] = 0;
    a0[2] = 0;
    a0[3] = 0;
    a0[4] = 0;
    a0[5] = 0;
    a0[6] = 0;
    a0[7] = 0;
    a0[8] = 0;
    a0[9] = 0;
    a0[10] = 0;
    a0[0xb] = 0;
    a0[0xc] = 0;
    a0[0xd] = 0;
    return a0;
  }
  abort(); // no-return
}

// Function: sub_9290 @ 0x9290
bool sub_9290(long a0,long a1)
{
  return sub_cd10(a0 + 9,a1 + 9) == 0;
}

// Function: sub_92b0 @ 0x92b0
unsigned long sub_92b0(long a0,unsigned long a1,unsigned char a2)
{
  unsigned char v1;
  
  if (sub_cc90((int)(char)a2)) // branch-flip
    v1 = *(unsigned char *)(a0 + 8) & 0xdf;
  else {
    v1 = *(unsigned char *)(a0 + 8);
  }
  if (v1 == a2) {
    if (a2)
      return sub_9290(a0,a1); // tail-call
    return 1;
  }
  return 0;
}

// Function: sub_9310 @ 0x9310
unsigned long sub_9310(long a0,unsigned long a1,unsigned char a2,char a3)
{
  unsigned char v1;
  
  if (sub_cc90((int)(char)a2)) // branch-flip
    v1 = *(unsigned char *)(a0 + 7) & 0xdf;
  else {
    v1 = *(unsigned char *)(a0 + 7);
  }
  if (v1 == a2) {
    if (a2)
      return sub_92b0(a0,a1,(int)a3); // tail-call
    return 1;
  }
  return 0;
}

// Function: sub_9390 @ 0x9390
unsigned long sub_9390(long a0,unsigned long a1,unsigned char a2,char a3,char a4)
{
  unsigned char v1;
  
  if (sub_cc90((int)(char)a2)) // branch-flip
    v1 = *(unsigned char *)(a0 + 6) & 0xdf;
  else {
    v1 = *(unsigned char *)(a0 + 6);
  }
  if (v1 == a2) {
    if (a2)
      return sub_9310(a0,a1,(int)a3,(int)a4); // tail-call
    return 1;
  }
  return 0;
}

// Function: sub_9400 @ 0x9400
unsigned long sub_9400(long a0,unsigned long a1,unsigned char a2,char a3,char a4,char a5)
{
  unsigned char v1;
  
  if (sub_cc90((int)(char)a2)) // branch-flip
    v1 = *(unsigned char *)(a0 + 5) & 0xdf;
  else {
    v1 = *(unsigned char *)(a0 + 5);
  }
  if (v1 == a2) {
    if (a2)
      return sub_9390(a0,a1,(int)a3,(int)a4,(int)a5); // tail-call
    return 1;
  }
  return 0;
}

// Function: sub_94a0 @ 0x94a0
unsigned long sub_94a0(long a0,unsigned long a1,unsigned char a2,char a3,char a4,char a5,unsigned int a6)
{
  unsigned int v1;
  unsigned char v2;
  
  v1 = a6;
  if (sub_cc90((int)(char)a2)) // branch-flip
    v2 = *(unsigned char *)(a0 + 4) & 0xdf;
  else {
    v2 = *(unsigned char *)(a0 + 4);
  }
  if (v2 == a2) {
    if (a2)
      return sub_9400(a0,a1,(int)a3,(int)a4,(int)a5,(int)(char)v1); // tail-call
    return 1;
  }
  return 0;
}

// Function: sub_9540 @ 0x9540
unsigned long sub_9540(long a0,unsigned long a1,unsigned char a2,char a3,char a4,char a5,int a6,unsigned int a7)
{
  unsigned int v1;
  unsigned int v2;
  unsigned char v3;
  
  v2 = a7;
  v1 = a6;
  if (sub_cc90((int)(char)a2)) // branch-flip
    v3 = *(unsigned char *)(a0 + 3) & 0xdf;
  else {
    v3 = *(unsigned char *)(a0 + 3);
  }
  if (v3 == a2) {
    if (a2) {
      a6 = (int)(char)v2;
      return sub_94a0(a0,a1,(int)a3,(int)a4,(int)a5,(int)(char)v1); // tail-call
    }
    return 1;
  }
  return 0;
}

// Function: sub_9600 @ 0x9600
unsigned long sub_9600(long a0,unsigned long a1,unsigned char a2,char a3,char a4,char a5,int a6,int a7,unsigned int a8)
{
  unsigned int v1;
  unsigned int v2;
  unsigned int v3;
  unsigned char v4;
  
  v3 = a8;
  v2 = a7;
  v1 = a6;
  if (sub_cc90((int)(char)a2)) // branch-flip
    v4 = *(unsigned char *)(a0 + 2) & 0xdf;
  else {
    v4 = *(unsigned char *)(a0 + 2);
  }
  if (v4 == a2) {
    if (a2) {
      a7 = (int)(char)v3;
      a6 = (int)(char)v2;
      return sub_9540(a0,a1,(int)a3,(int)a4,(int)a5,(int)(char)v1); // tail-call
    }
    return 1;
  }
  return 0;
}

// Function: sub_96c0 @ 0x96c0
unsigned long sub_96c0(long a0,unsigned long a1,unsigned char a2,char a3,char a4,char a5,int a6,int a7,int a8,unsigned int a9)
{
  unsigned int v1;
  unsigned int v2;
  unsigned int v3;
  unsigned int v4;
  unsigned char v5;
  char v6; // stack - 0x3c
  
  v4 = a9;
  v3 = a8;
  v2 = a7;
  v1 = a6;
  if (sub_cc90((int)(char)a2)) // branch-flip
    v5 = *(unsigned char *)(a0 + 1) & 0xdf;
  else {
    v5 = *(unsigned char *)(a0 + 1);
  }
  if (v5 == a2) {
    if (a2) {
      a8 = (int)(char)v4;
      a7 = (int)(char)v3;
      a6 = (int)(char)v2;
      v6 = (char)v1;
      return sub_9600(a0,a1,(int)a3,(int)a4,(int)a5,(int)v6); // tail-call
    }
    return 1;
  }
  return 0;
}

// Function: sub_97a0 @ 0x97a0
unsigned long sub_97a0(unsigned char *a0,unsigned long a1,unsigned char a2,char a3,char a4,char a5,unsigned int a6,unsigned int a7,unsigned int a8) // early-return
{
  unsigned int v1;
  unsigned int v2;
  unsigned int v3;
  
  v3 = a8;
  v2 = a7;
  v1 = a6;
  if (sub_cc90((int)(char)a2)) { // branch-flip
    if (a2 != (*a0 & 0xdf))
      return 0;
  }
  else if (a2 != *a0)
    return 0;
  return sub_96c0(a0,a1,(int)a3,(int)a4,(int)a5,(int)(char)v1,(int)(char)v2,(int)(char)v3,0,0);
}

// Function: sub_9860 @ 0x9860
char * sub_9860(char *a0,int a1) // return-dupe, ternary x2
{
  char *v1; // rax
  unsigned long v2; // rax
  
  v1 = dcgettext(NULL,a0,5);
  if (a0 == v1) {
    v2 = sub_d130();
    if (sub_97a0(v2,"UTF-8",0x55,0x54,0x46,0x2d,0x38,0,0)) { // branch-flip
      v1 = (*v1 != '`') ? "’" : "‘"; // branch-flip
    }
    else {
      if (!sub_97a0(v2,"GB18030",0x47,0x42,0x31,0x38,0x30,0x33,0x30)) {
        if (a1 == 9)
          return "\"";
        return "\'";
      }
      v1 = (*v1 != '`') ? (char *)0xf660 : (char *)0xf667; // branch-flip
    }
  }
  return v1;
}

// Function: sub_9990 @ 0x9990
long * sub_9990(char *a0,unsigned long a1,char *a2,unsigned long a3,int a4,unsigned int a5,long a6,char *a7,char *a8)
{
  unsigned long v1;
  char v10;
  unsigned char *v11;
  unsigned long v12;
  unsigned long v13;
  unsigned long v14; // stack - 0x48
  unsigned int v15; // stack - 0x4c
  unsigned char v16;
  unsigned int v17;
  unsigned long v18;
  unsigned char v19;
  bool v2;
  bool v20;
  unsigned long v21;
  bool v22;
  unsigned long v23;
  bool v24;
  bool v25;
  bool v26;
  bool v27;
  bool v28;
  long v29; // stack - 0xe0
  bool v3;
  unsigned long v30; // stack - 0xd8
  char *v31; // stack - 0xb0
  unsigned long v32; // stack - 0xa8
  unsigned long v33; // stack - 0xa0
  unsigned int v34; // stack - 0x80
  char *v35; // stack - 0x70
  char *v36; // stack - 0x68
  bool v4;
  long v5;
  long v6;
  unsigned char v7;
  unsigned int v8; // eax
  long *v9; // rax
  
  v29 = a6;
  v36 = a7;
  v35 = a8;
  v34 = a5;
label_9a00:
  v9 = (unsigned long)__ctype_get_mb_cur_max();
  v33 = (unsigned long)v9;
  v17 = v34 & 2;
  switch(a4) {
    case 0:
      v24 = 0;
      v2 = 0;
      v12 = 0;
      v30 = 0;
      v31 = NULL;
      break;
    case 1:
label_9e86:
      v2 = 0;
      goto label_9e40;
    case 2:
      if (!v17) {
        v2 = 0;
        goto label_9fa1;
      }
      v24 = 1;
      v2 = 0;
      v12 = 0;
      v30 = 1;
      v31 = "\'";
      break;
    case 3:
      v2 = 1;
label_9e40:
      v24 = 1;
      v12 = 0;
      a4 = 2;
      v30 = 1;
      v31 = "\'";
      break;
    case 4:
      if (!v17) {
        v2 = 1;
        goto label_9fa1;
      }
      goto label_9e86;
    case 5:
      if (!v17) goto label_9de1;
      v24 = 1;
      v2 = 1;
      v12 = 0;
      v30 = 1;
      v31 = "\"";
      break;
    case 6:
      v24 = 1;
      v12 = 0;
      v2 = 1;
      v30 = 1;
      a4 = 5;
      v31 = "\"";
      break;
    case 7:
      v24 = 0;
      v2 = 1;
      v12 = 0;
      v30 = 0;
      v31 = NULL;
      break;
    case 8:
    case 9:
    case 10:
      if (a4 != 10) {
        v9 = (char *)sub_9860("`",a4);
        v36 = (char *)v9;
        v9 = (char *)sub_9860("\'",a4);
        v35 = (char *)v9;
      }
      v12 = 0;
      if (!v17) {
        v10 = *v36;
        while (v10) {
          if (v12 < a1)
            a0[v12] = v10;
          v12 += 1;
          v10 = v36[v12];
        }
      }
      v24 = v17 != 0;
      v2 = 1;
      v9 = (unsigned long)strlen(v35);
      v31 = v35;
      v30 = (unsigned long)v9;
      break;
    default:
      abort(); // no-return
    
  }
  v4 = 1;
  v20 = 0;
  v32 = 0;
  v13 = a1;
  v3 = 0;
label_9ac3:
  do {
label_9ad6:
    v23 = 0;
    a1 = v13;
label_9ae0:
    v13 = v32;
    v25 = a3 != v23;
    if (a3 == 0xffffffffffffffff)
      v25 = a2[v23] != '\0';
    if (v25) {
      v11 = (unsigned char *)&a2[v23];
      v25 = (bool)(a4 != 2 & v2);
      v26 = 1;
      v13 = v12;
      v27 = v2;
      v28 = v2;
      v22 = v25;
      if (v25) {
        if (v30) {
          if ((a3 == 0xffffffffffffffff) && (2 <= v30)) {
            v9 = (unsigned long)strlen(a2);
            a3 = (unsigned long)v9;
          }
          if ((a3 < v23 + v30) || (v8 = memcmp(v11,v31,v30), v8)) goto label_a450;
          if (!v24) {
            v7 = *v11;
            v18 = (unsigned long)v7;
            v21 = v18;
            if ((char)v7 <= '?') {
              switch(v7) {
                case 0:
                  goto label_a04c;
                default:
                  goto label_9d23;
                case 7:
                  goto label_a010;
                case 8:
                  goto label_9ff5;
                case 9:
                  goto label_a0d3;
                case 10:
                  goto label_9feb;
                case 0xb:
                  goto label_a100;
                case 0xc:
                  goto label_a037;
                case 0xd:
                  goto label_9f3d;
                case 0x20:
                  goto label_a10d;
                case 0x21:
                case 0x22:
                case 0x24:
                case 0x26:
                case 0x28:
                case 0x29:
                case 0x2a:
                case 0x3b:
                case 0x3c:
                case 0x3d:
                case 0x3e:
                  goto label_9cd6;
                case 0x23:
                  goto label_a0b0;
                case 0x25:
                case 0x2b:
                case 0x2c:
                case 0x2d:
                case 0x2e:
                case 0x2f:
                case 0x30:
                case 0x31:
                case 0x32:
                case 0x33:
                case 0x34:
                case 0x35:
                case 0x36:
                case 0x37:
                case 0x38:
                case 0x39:
                case 0x3a:
                  goto label_9c08;
                case 0x27:
                  goto label_9f22;
                case 0x3f:
                  goto label_9ee1;
                
              }
            }
            if ('{' > (char)v7) {
              if (v7 == 0x40) goto label_9d23;
              v9 = (unsigned long)(1L << (v7 + 0xbf & 0x3f));
              if ((unsigned long)v9 & 0x3ffffff53ffffff) goto label_9c08;
              if (!((unsigned long)v9 & 0xa4000000)) goto label_a4f4;
              goto label_a438;
            }
            if (v7 == 0x7d) goto label_a5c0;
            if ((char)v7 <= '}') {
              v21 = 0x7b;
              if (v7 == 0x7b) goto label_a41a;
              v21 = 0x7c;
              if (v7 == 0x7c) goto label_a438;
              goto label_9d23;
            }
            if (v7 != 0x7e) goto label_9d23;
label_a574:
            if (!v23) {
              v7 = a4 == 2;
              v21 = 0x7e;
              v26 = v3;
              goto label_9d99;
            }
            v21 = 0x7e;
            goto label_a582;
          }
          goto label_9d0c;
        }
label_a450:
        v7 = *v11;
        v18 = (unsigned long)v7;
        v21 = v18;
        if ('?' < (char)v7) {
          if ('{' <= (char)v7) { // branch-flip
            if (v7 != 0x7d) { // branch-flip
              if ('~' <= (char)v7) {
                v22 = 0;
                if (v7 == 0x7e) goto label_a574;
                goto label_9d20;
              }
              v21 = 0x7b;
              v25 = 0;
              if (v7 != 0x7b) {
                v21 = 0x7c;
                v25 = 0;
                v22 = 0;
                if (v7 == 0x7c) goto label_a438;
                goto label_9d23;
              }
            }
            else {
              v21 = 0x7d;
              v25 = 0;
            }
label_a41a:
            if (a3 == 0xffffffffffffffff) goto label_a5cf;
label_a428:
            v22 = v25;
            if (a3 == 1) goto label_a0b5;
          }
          else {
            if (v7 == 0x40) goto label_9d20;
            v9 = (unsigned long)(1L << (v7 + 0xbf & 0x3f));
            v22 = 0;
            if ((unsigned long)v9 & 0x3ffffff53ffffff) goto label_9c08;
            v25 = 0;
            v22 = 0;
            if (!((unsigned long)v9 & 0xa4000000)) {
label_a4f4:
              if ((char)v18 != '\\') goto label_9d23;
              if (((bool)(v2 & v24)) && (v30)) goto label_a51a;
              v16 = 0x5c;
              goto label_9f42;
            }
          }
label_a438:
          v7 = a4 == 2;
          v26 = v3;
          v25 = 0;
          goto label_9d99;
        }
        switch(v7) {
          case 0:
            goto label_a03e;
          default:
label_9d20:
            v25 = 0;
label_9d23:
            v16 = (unsigned char)v18;
            v22 = v25;
            if (v33 != 1) goto label_a2a3;
label_9d30:
            v9 = __ctype_b_loc();
            v21 = 1;
            v26 = (*(unsigned char *)(*v9 + 1 + v18 * 2) & 0x40) == 0;
            v27 = (bool)(v26 & v2);
            v26 = !v26;
            v22 = v25;
            goto label_9d7e;
          case 7:
label_a010:
            v16 = 0x61;
            if (!v24) goto label_a008;
            goto label_9f6f;
          case 8:
label_9ff5:
            v16 = 0x62;
            goto label_9ffa;
          case 9:
            v25 = 0;
label_a0d3:
            v21 = 9;
            v16 = 0x74;
            goto label_a0e0;
          case 10:
label_9feb:
            v16 = 0x6e;
            break;
          case 0xb:
label_a100:
            v16 = 0x76;
            break;
          case 0xc:
label_a037:
            v16 = 0x66;
label_9ffa:
            if (v24) goto label_9f6f;
label_a008:
            v25 = 0;
            goto label_9c8e;
          case 0xd:
label_9f3d:
            v16 = 0x72;
            break;
          case 0x20:
            v22 = 0;
label_a10d:
            v21 = 0x20;
            goto label_9c08;
          case 0x21:
          case 0x22:
          case 0x24:
          case 0x26:
          case 0x28:
          case 0x29:
          case 0x2a:
          case 0x3b:
          case 0x3c:
          case 0x3d:
          case 0x3e:
            v25 = 0;
            v26 = 0;
            goto label_9cd9;
          case 0x23:
            v21 = 0x23;
            v25 = 0;
            goto label_a0b5;
          case 0x25:
          case 0x2b:
          case 0x2c:
          case 0x2d:
          case 0x2e:
          case 0x2f:
          case 0x30:
          case 0x31:
          case 0x32:
          case 0x33:
          case 0x34:
          case 0x35:
          case 0x36:
          case 0x37:
          case 0x38:
          case 0x39:
          case 0x3a:
            v22 = 0;
label_9c08:
            v7 = 0;
            goto label_9c18;
          case 0x27:
            v22 = 0;
            goto label_9f22;
          case 0x3f:
            v22 = 0;
            goto label_9ee1;
          
        }
        goto label_9f42;
      }
      v16 = *v11;
      v18 = (unsigned long)v16;
      if ('?' < (char)v16) {
        if ('{' <= (char)v16) {
          if (v16 == 0x7d) {
label_a5c0:
            v21 = 0x7d;
            if (a3 != 0xffffffffffffffff) goto label_a428;
label_a5cf:
            v22 = v25;
            if (a2[1]) goto label_a438;
            goto label_a0b5;
          }
          if ((char)v16 <= '}') {
            v21 = 0x7b;
            if (v16 != 0x7b) {
              if (v16 == 0x7c) goto label_9cd6;
              goto label_9d23;
            }
            goto label_a41a;
          }
          v21 = 0x7e;
          if (v16 == 0x7e) goto label_a0b5;
          if (v33 == 1) goto label_9d30;
label_a2a3:
          v14 = 0;
          if (a3 == 0xffffffffffffffff) {
            v9 = (unsigned long)strlen(a2);
            a3 = (unsigned long)v9;
          }
          v21 = 0;
          do {
            v18 = v23 + v21;
            v9 = (long)sub_d170(&v15,&a2[v18],a3 - v18,&v14);
            v6 = (long)v9;
            v25 = v26;
            if (!v9) break;
            if (v9 == (long *)0xffffffffffffffff) {
              v25 = 0;
              goto label_a80b;
            }
            if (v9 == (long *)0xfffffffffffffffe) {
              v9 = (long *)v18;
              if (a3 > v18) goto label_ab85;
              goto label_ab8f;
            }
            if (((a4 == 2) && (v24)) && (v9 != (long *)0x1)) {
              v11 = (unsigned char *)&a2[v18 + 1];
              v5 = v18 + (long)v9;
              do {
                v8 = (unsigned int)*v11;
                v8 -= 0x5b;
                v17 = v8;
                v9 = (unsigned long)(unsigned long)v8;
                v18 = (unsigned long)v9;
                v7 = (unsigned char)v17;
                if ((v7 <= 0x21) && (0x20000002bU >> (v18 & 0x3f) & 1)) goto label_a3dd;
                v11 = &v11[1];
              } while ((unsigned char *)&a2[v5] != v11);
            }
            v8 = iswprint(v15);
            if (!v8)
              v26 = 0;
            v21 += v6;
            v8 = mbsinit(&v14);
            v25 = v26;
          } while (!v8);
          v27 = (bool)((v25 ^ 1U) & v2);
          goto label_a80b;
        }
        if (v16 == 0x40) goto label_9d23;
        v9 = (unsigned long)(1L << (v16 + 0xbf & 0x3f));
        if ((unsigned long)v9 & 0x3ffffff53ffffff) goto label_9d90;
        if ((unsigned long)v9 & 0xa4000000) goto label_9cd6;
        if (v16 != 0x5c) goto label_9d23;
        if (a4 == 2) {
          if (!v24) goto label_a51a;
          goto label_9d03;
        }
        if (!(bool)(v2 & v24 & v30 != 0)) {
          v21 = 0x5c;
          v16 = 0x5c;
          v25 = 0;
          goto label_a0e0;
        }
label_a51a:
        v23 += 1;
        v25 = 0;
        v16 = 0x5c;
        v7 = v20;
        goto label_a530;
      }
      switch(v16) {
        case 0:
          if (!v2) {
            v22 = 0;
            v21 = 0;
            if (v34 & 1) goto label_a1b2;
            goto label_a0eb;
          }
label_a03e:
          if (!v24) {
            v22 = 0;
label_a04c:
            v27 = a4 == 2;
            v7 = v20 ^ 1;
            v7 &= v27;
            if ((bool)v7) {
              if (v12 < a1)
                a0[v12] = 0x27;
              if (v12 + 1 < a1)
                a0[v12 + 1] = 0x24;
              if (v12 + 2 < a1)
                a0[v12 + 2] = 0x27;
              v13 = v12 + 3;
              if (a1 <= v12 + 3) {
                v12 += 4;
                v25 = 0;
                v16 = 0x30;
                v20 = (bool)v7;
                goto label_9dc3;
              }
label_a955:
              a0[v13] = 0x5c;
              v12 = v13;
              v20 = (bool)v7;
            }
            else if (v12 < a1) {
              v7 = v20;
              goto label_a955;
            }
            v13 = v12 + 1;
            if (a4 != 2) {
              v21 = 0x30;
              if ((v23 + 1 < a3) && (v7 = a2[v23 + 1] - 0x30, v7 <= 9)) {
                if (v13 < a1)
                  a0[v13] = 0x30;
                if (v12 + 2 < a1)
                  a0[v12 + 2] = 0x30;
                v13 = v12 + 3;
                v21 = 0x30;
              }
label_a98c:
              v16 = (unsigned char)v21;
              v25 = 0;
              v12 = v13;
              v7 = v26;
              if (!v2 || v27) goto label_9dc3;
              goto label_9c18;
            }
            v16 = 0x30;
            v25 = 0;
            v12 = v13;
            v7 = v26;
            goto label_9dc3;
          }
          if (a4 != 2) goto label_9d0c;
          goto label_9f7b;
        default:
          goto label_9d23;
        case 7:
          v21 = 7;
          v16 = 0x61;
          break;
        case 8:
          v21 = 8;
          v16 = 0x62;
          break;
        case 9:
          v21 = 9;
          v16 = 0x74;
          goto label_a16f;
        case 10:
          v21 = 10;
          v16 = 0x6e;
          goto label_a16f;
        case 0xb:
          v21 = 0xb;
          v16 = 0x76;
          break;
        case 0xc:
          v21 = 0xc;
          v16 = 0x66;
          break;
        case 0xd:
          v21 = 0xd;
          v16 = 0x72;
label_a16f:
          v7 = a4 == 2 & v24;
          v25 = 0;
          if (!(bool)v7) goto label_a0e0;
label_9cf1:
          a4 = 2;
label_9d03:
          if (!v28) goto label_9d0c;
          goto label_9f7b;
        case 0x20:
          v18 = 0x20;
          goto label_9cd9;
        case 0x21:
        case 0x22:
        case 0x24:
        case 0x26:
        case 0x28:
        case 0x29:
        case 0x2a:
        case 0x3b:
        case 0x3c:
        case 0x3d:
        case 0x3e:
label_9cd6:
          v26 = 0;
          goto label_9cd9;
        case 0x23:
label_a0b0:
          v21 = 0x23;
label_a0b5:
          v22 = v25;
          if (v23) {
label_a582:
            v7 = a4 == 2;
            v26 = v3;
            v25 = 0;
            goto label_9d99;
          }
          v18 = v21;
label_9cd9:
          v21 = v18;
          v22 = v25;
          v25 = v26;
          if ((a4 == 2) && (v24)) goto label_9cf1;
          goto label_9d97;
        case 0x25:
        case 0x2b:
        case 0x2c:
        case 0x2d:
        case 0x2e:
        case 0x2f:
        case 0x30:
        case 0x31:
        case 0x32:
        case 0x33:
        case 0x34:
        case 0x35:
        case 0x36:
        case 0x37:
        case 0x38:
        case 0x39:
        case 0x3a:
          goto label_9d90;
        case 0x27:
label_9f22:
          v25 = v26;
          if (a4 != 2) {
            v7 = 0;
            v21 = 0x27;
            goto label_9d99;
          }
          if (!v24) {
            if (a1) { // branch-flip
              v13 = 0;
              v18 = a1;
              if (v32) goto label_a9ab;
            }
            else {
label_a9ab:
              if (v12 < a1)
                a0[v12] = 0x27;
              if (v12 + 1 < a1)
                a0[v12 + 1] = 0x5c;
              v13 = a1;
              v18 = v32;
              if (v12 + 2 < a1)
                a0[v12 + 2] = 0x27;
            }
            v12 += 3;
            v7 = 0;
            v16 = 0x27;
            a1 = v13;
            v32 = v18;
            v20 = 0;
            v3 = v26;
            goto label_9dc3;
          }
          goto label_9d03;
        case 0x3f:
label_9ee1:
          if (a4 != 2) {
            if ((((a4 == 5) && (v34 & 4)) && (v18 = v23 + 2, v18 < a3)) && (a2[v23 + 1] == '?')) {
              v7 = a2[v18];
              v21 = (unsigned long)v7;
              if (v7 <= 0x3e) {
                v9 = (unsigned long)(0x7000a38200000000 >> (v7 & 0x3f));
                v1 = (unsigned long)v9 & 1;
                v7 = (unsigned char)v9 & 1;
                if (!v1) {
                  v21 = 0x3f;
                  v26 = v3;
                  v25 = 0;
                  goto label_9d99;
                }
                if (v24) goto label_9d0c;
                if (v12 < a1)
                  a0[v12] = 0x3f;
                if (v12 + 1 < a1)
                  a0[v12 + 1] = 0x22;
                if (v12 + 2 < a1)
                  a0[v12 + 2] = 0x22;
                if (v12 + 3 < a1)
                  a0[v12 + 3] = 0x3f;
                v13 = v12 + 4;
                v27 = 0;
                v26 = 0;
                v23 = v18;
                goto label_a98c;
              }
            }
            v7 = 0;
            v21 = 0x3f;
            v26 = v3;
            v25 = 0;
            goto label_9d99;
          }
          if (v24) goto label_9d03;
          v7 = 0;
          v16 = 0x3f;
          v25 = 0;
          goto label_9dc3;
        
      }
label_a0e0:
      v22 = v25;
      if (!v2) {
label_a0eb:
        v16 = (unsigned char)v21;
        v25 = 0;
        v7 = 0;
        v26 = v3;
        if (!v24) goto label_9dc0;
        goto label_9c18;
      }
label_9f42:
      v25 = 0;
      goto label_9f53;
    }
    if (!(bool)(v12 == 0 & v24 & a4 == 2)) {
      v7 = a4 == 2 & (v24 ^ 1U);
      v24 = (bool)(v24 ^ 1U);
      if ((!(bool)v7) || (v24 = (bool)v7, !v3)) {
label_ac0f:
        v9 = (long *)v12;
        if (((v31) && (v24)) && (v10 = *v31, v10)) {
          do {
            if (v9 < a1)
              a0[(long)v9] = v10;
            v9 = (unsigned long)((long)v9 + 1);
            v10 = v31[(long)v9 - v12];
          } while (v10);
        }
        if (v9 < a1)
          a0[(long)v9] = 0;
        return v9;
      }
      if (!v4) {
        v7 = !a1 && v32;
        v24 = v3;
        if (a1 || !v32) goto label_ac0f;
        v23 = v32;
        goto label_a6db;
      }
      v12 = 0;
      a4 = 5;
      v9 = (unsigned long)__ctype_get_mb_cur_max();
      v30 = 1;
      v33 = (unsigned long)v9;
      v31 = "\"";
      if (!(v34 & 2)) goto label_acbf;
      v3 = 0;
      v20 = 0;
      v32 = 0;
      v2 = v4;
      v24 = v4;
      goto label_9ad6;
    }
label_a3dd:
    if (v2) {
label_9f7b:
      v2 = 1;
      v34 &= 0xfffffffd;
      v9 = (unsigned long)__ctype_get_mb_cur_max();
      v29 = 0;
      v33 = (unsigned long)v9;
    }
    else {
      v2 = 0;
      v34 &= 0xfffffffd;
      v9 = (unsigned long)__ctype_get_mb_cur_max();
      v29 = 0;
      v33 = (unsigned long)v9;
    }
label_9fa1:
    v24 = 0;
    a4 = 2;
    v12 = 1;
    v31 = "\'";
    v30 = 1;
    v20 = 0;
    v4 = 1;
    if (a1) { // branch-flip
      v7 = 0;
      v23 = 0;
      v32 = a1;
label_a6db:
      v12 = 1;
      *a0 = 0x27;
      a4 = 2;
      v24 = 0;
      v30 = 1;
      v31 = "\'";
      v13 = v32;
      v32 = v23;
      v3 = (bool)v7;
    }
    else {
      v32 = 0;
      v13 = a1;
      v3 = 0;
    }
  } while( true );
  while( true ) {
    v21 += 1;
    v9 = (unsigned long)(v23 + v21);
    if (a3 <= v9) break;
label_ab85:
    if (!a2[(long)v9]) break;
  }
label_ab8f:
  v25 = 0;
label_a80b:
  v18 = (unsigned long)v16;
  v26 = v25;
  if (2 <= v21) {
label_a815:
    v19 = 0;
    v21 += v23;
    v13 = v23;
    do {
      v16 = (unsigned char)v18;
      if (v27) {
        v28 = a4 == 2;
        if (v24) goto label_9d03;
        v7 = v28 & (v20 ^ 1U);
        if ((bool)v7) {
          if (v12 < a1)
            a0[v12] = 0x27;
          if (v12 + 1 < a1)
            a0[v12 + 1] = 0x24;
          if (v12 + 2 < a1)
            a0[v12 + 2] = 0x27;
          v12 += 3;
          v20 = 1;
        }
        if (v12 < a1)
          a0[v12] = 0x5c;
        if (v12 + 1 < a1) {
          v7 = (char)(v18 >> 6) + '0';
          a0[v12 + 1] = v7;
        }
        if (v12 + 2 < a1) {
          v7 = ((unsigned char)(v18 >> 3) & 7) + 0x30;
          a0[v12 + 2] = v7;
        }
        v23 = v13 + 1;
        v12 += 3;
        v16 = (v16 & 7) + 0x30;
        if (v21 <= v23) goto label_9ca1;
        v19 = 1;
      }
      else {
        v7 = v19 ^ 1;
        v7 &= v20;
        if (v22) {
          if (v12 < a1)
            a0[v12] = 0x5c;
          v12 += 1;
        }
        v23 = v13 + 1;
        if (v21 <= v23) {
          goto label_a530;
        }
        if ((bool)v7) {
          if (v12 < a1)
            a0[v12] = 0x27;
          if (v12 + 1 < a1)
            a0[v12 + 1] = 0x27;
          v12 += 2;
          v22 = 0;
          v20 = 0;
        }
        else {
          v22 = 0;
        }
      }
      v13 += 1;
      if (v12 < a1)
        a0[v12] = v16;
      v18 = (unsigned long)(unsigned char)a2[v13];
      v12 += 1;
    } while( true );
  }
label_9d7e:
  if (v27) {
    v25 = 0;
    v27 = v2;
    goto label_a815;
  }
label_9d90:
  v21 = v18;
  v25 = v26;
label_9d97:
  v7 = a4 == 2;
  v26 = v3;
label_9d99:
  v16 = (unsigned char)v21;
  v27 = (bool)v7 == 0;
  v3 = v26;
  v7 = 0;
  if ((v27 && v2) || (v7 = 0, v24)) {
label_9c18:
    v16 = (unsigned char)v21;
    v12 = v13;
    if (!v29) goto label_9dc3;
    if (!(*(unsigned int *)(v29 + (v21 >> 5) * 4) >> (v16 & 0x1f) & 1)) goto label_9dc3;
  }
  else {
label_9dc0:
    v7 = 0;
    v3 = v26;
label_9dc3:
    if (!v22) {
      v7 ^= 1;
      v23 += 1;
      v7 &= v20;
label_a530:
      if ((bool)v7) {
        if (v12 < a1)
          a0[v12] = 0x27;
        if (v12 + 1 < a1)
          a0[v12 + 1] = 0x27;
        v20 = 0;
        v12 += 2;
      }
      goto label_9ca1;
    }
  }
label_9f53:
  if (v24) {
    v24 = v2;
label_9f6f:
    if ((bool)(v24 & a4 == 2)) goto label_9f7b;
label_9d0c:
    v34 &= 0xfffffffd;
    v29 = 0;
    goto label_9a00;
  }
  v7 = v20 ^ 1;
  v7 &= a4 == 2;
  if ((bool)v7) {
    if (v12 < a1)
      a0[v12] = 0x27;
    if (v12 + 1 < a1)
      a0[v12 + 1] = 0x24;
    if (v12 + 2 < a1)
      a0[v12 + 2] = 0x27;
    v12 += 3;
    v20 = 1;
  }
label_9c8e:
  if (v12 < a1)
    a0[v12] = 0x5c;
  v12 += 1;
  v23 += 1;
label_9ca1:
  if (v12 < a1)
    a0[v12] = v16;
  v12 += 1;
  if (!v25)
    v4 = 0;
  goto label_9ae0;
label_a1b2:
  v23 += 1;
  goto label_9ae0;
label_acbf:
  a1 = v32;
label_9de1:
  v24 = 0;
  a4 = 5;
  v12 = 1;
  v31 = "\"";
  v2 = 1;
  v30 = 1;
  v20 = 0;
  v3 = 0;
  v4 = 1;
  v32 = 0;
  v13 = a1;
  if (a1)
    *a0 = 0x22;
  goto label_9ac3;
}

// Function: sub_adc0 @ 0xadc0
void * sub_adc0(unsigned int a0,unsigned long a1,unsigned long a2,unsigned int *a3)
{
  unsigned long *v1;
  long v10; // stack - 0x48
  int *v11; // stack - 0x68
  int v2;
  unsigned int v3;
  unsigned int v4;
  unsigned long v5;
  unsigned long *v6;
  unsigned long v7;
  void *v8;
  long v9; // rdx
  
  v11 = __errno_location();
  v2 = *v11;
  if (a0 <= 0x7ffffffe) {
    v6 = dat_150a0;
    if (dat_15098 <= (int)a0) {
      v10 = (long)dat_15098;
      v9 = (long)(int)((a0 - dat_15098) + 1);
      if (dat_150a0 != (unsigned long *)0x150b0) // branch-flip
        v6 = (unsigned long *)sub_c160(dat_150a0,&v10,v9,0x7fffffff,0x10);
      else {
        v6 = (unsigned long *)sub_c160(0,&v10,v9,0x7fffffff,0x10);
        *v6 = dat_150b0;
        v6[1] = dat_150b8;
      }
      dat_150a0 = v6;
      memset(&v6[(long)dat_15098 * 2],0,(v10 - dat_15098) * 0x10);
      dat_15098 = (int)v10;
    }
    v1 = &v6[(long)(int)a0 * 2];
    v3 = a3[1];
    v5 = *v1;
    v8 = (void *)v1[1];
    v7 = sub_9990(v8,v5,a1,a2,*a3,v3 | 1,&a3[2],*(unsigned long *)&a3[10],*(unsigned long *)&a3[0xc]);
    if (v5 <= v7) {
      v7 += 1;
      *v1 = v7;
      if (v8 != (void *)0x15140)
        free(v8);
      v8 = (void *)sub_bfe0(v7);
      v4 = *a3;
      v1[1] = (unsigned long)v8;
      sub_9990(v8,v7,a1,a2,v4,v3 | 1,&a3[2],*(unsigned long *)&a3[10],*(unsigned long *)&a3[0xc]);
    }
    *v11 = v2;
    return v8;
  }
  abort(); // no-return
}

// Function: sub_afb0 @ 0xafb0
void sub_afb0(long a0)
{
  int v1;
  int *v2; // rax
  
  v2 = __errno_location();
  v1 = *v2;
  if (!a0)
    a0 = 0x15240;
  sub_c2a0(a0,0x38);
  *v2 = v1;
}

// Function: sub_aff0 @ 0xaff0
unsigned int sub_aff0(unsigned int *a0)
{
  if (!a0)
    a0 = (unsigned int *)0x15240;
  return *a0;
}

// Function: sub_b010 @ 0xb010
void sub_b010(unsigned int *a0,unsigned int a1)
{
  if (!a0)
    a0 = (unsigned int *)0x15240;
  *a0 = a1;
}

// Function: sub_b030 @ 0xb030
unsigned int sub_b030(long a0,unsigned char a1,unsigned int a2)
{
  unsigned int *v1;
  unsigned int v2;
  unsigned int v3; // eax
  
  if (!a0)
    a0 = 0x15240;
  v1 = (unsigned int *)(a0 + 8 + (unsigned long)(a1 >> 5) * 4);
  v2 = *v1;
  v3 = v2 >> (a1 & 0x1f);
  *v1 = ((a2 ^ v3) & 1) << (a1 & 0x1f) ^ v2;
  return v3 & 1;
}

// Function: sub_b070 @ 0xb070
unsigned int sub_b070(long a0,unsigned int a1)
{
  unsigned int v1;
  
  if (!a0)
    a0 = 0x15240;
  v1 = *(unsigned int *)(a0 + 4);
  *(unsigned int *)(a0 + 4) = a1;
  return v1;
}

// Function: sub_b090 @ 0xb090
void sub_b090(unsigned int *a0,long a1,long a2)
{
  if (!a0)
    a0 = (unsigned int *)0x15240;
  *a0 = 10;
  if ((a1) && (a2)) {
    *(long *)&a0[10] = a1;
    *(long *)&a0[0xc] = a2;
    return;
  }
  abort(); // no-return
}

// Function: sub_b0d0 @ 0xb0d0
void sub_b0d0(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned int *a4)
{
  int v1;
  int *v2; // rax
  
  if (!a4)
    a4 = (unsigned int *)0x15240;
  v2 = __errno_location();
  v1 = *v2;
  sub_9990(a0,a1,a2,a3,*a4,a4[1],&a4[2],*(unsigned long *)&a4[10],*(unsigned long *)&a4[0xc]);
  *v2 = v1;
}

// Function: sub_b150 @ 0xb150
unsigned long sub_b150(unsigned long a0,unsigned long a1,long *a2,unsigned int *a3)
{
  int v1;
  int *v2; // rax
  long v3; // rax
  unsigned long v4; // rax
  unsigned int v5; // r9d
  
  if (!a3)
    a3 = (unsigned int *)0x15240;
  v2 = __errno_location();
  v1 = *v2;
  v5 = (unsigned int)(a2 == NULL) | a3[1];
  v3 = sub_9990(0,0,a0,a1,*a3,v5,&a3[2],*(unsigned long *)&a3[10],*(unsigned long *)&a3[0xc]);
  v4 = sub_bfe0(v3 + 1);
  sub_9990(v4,v3 + 1,a0,a1,*a3,v5,&a3[2],*(unsigned long *)&a3[10],*(unsigned long *)&a3[0xc]);
  *v2 = v1;
  if (a2)
    *a2 = v3;
  return v4;
}

// Function: sub_b240 @ 0xb240
void sub_b240(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_b150(a0,a1,0,a2); // tail-call
}

// Function: sub_b250 @ 0xb250
void sub_b250(void) // return-dupe
{
  unsigned long *v1;
  void *v2;
  void *v3;
  unsigned long *v4;
  
  v3 = dat_150a0;
  if (2 <= dat_15098) {
    v4 = (unsigned long *)((long)dat_150a0 + 0x18);
    v1 = (unsigned long *)((long)dat_150a0 + (unsigned long)(unsigned int)(dat_15098 - 2) * 0x10 + 0x28);
    do {
      v2 = (void *)*v4;
      v4 = &v4[2];
      free(v2);
    } while (v4 != v1);
  }
  if (*(void **)((long)v3 + 8) != (void *)0x15140) {
    free(*(void **)((long)v3 + 8));
    dat_150b8 = 0x15140;
    dat_150b0 = 0x100;
  }
  if (v3 == (void *)0x150b0) {
    dat_15098 = 1;
    return;
  }
  free(v3);
  dat_15098 = 1;
  dat_150a0 = (void *)0x150b0;
}

// Function: sub_b2f0 @ 0xb2f0
void sub_b2f0(unsigned long a0,unsigned long a1)
{
  sub_adc0(a0,a1,0xffffffffffffffff,0x15240); // tail-call
}

// Function: sub_b310 @ 0xb310
void sub_b310(void)
{
  sub_adc0(); // tail-call
}

// Function: sub_b320 @ 0xb320
void sub_b320(unsigned long a0)
{
  sub_b2f0(0,a0); // tail-call
}

// Function: sub_b330 @ 0xb330
void sub_b330(unsigned long a0,unsigned long a1)
{
  sub_b310(0,a0,a1); // tail-call
}

// Function: sub_b340 @ 0xb340
void sub_b340(unsigned int a0,unsigned long a1,unsigned long a2)
{
  char v1 [56];
  
  sub_9240(v1);
  sub_adc0(a0,a2,0xffffffffffffffff,v1);
}

// Function: sub_b3a0 @ 0xb3a0
void sub_b3a0(unsigned int a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  char v1 [56];
  
  sub_9240(v1);
  sub_adc0(a0,a2,a3,v1);
}

// Function: sub_b410 @ 0xb410
void sub_b410(unsigned int a0,unsigned long a1)
{
  sub_b340(0,a0,a1); // tail-call
}

// Function: sub_b420 @ 0xb420
void sub_b420(unsigned int a0,unsigned long a1,unsigned long a2)
{
  sub_b3a0(0,a0,a1,a2); // tail-call
}

// Function: sub_b440 @ 0xb440
void sub_b440(unsigned long a0,unsigned long a1,char a2,unsigned long a3)
{
  unsigned long v1; // stack - 0x48
  unsigned long *v2; // rsp
  unsigned long v3; // stack - 0x40
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  unsigned long v7; // stack - 0x20
  unsigned long v8; // stack - 0x18
  
  v2 = &v1;
  v1 = dat_15240;
  v3 = dat_15248;
  v4 = dat_15250;
  v5 = dat_15258;
  v8 = dat_15270;
  v6 = dat_15260;
  v7 = dat_15268;
  sub_b030(&v1,(int)a2,1,a3,a0,a1);
  sub_adc0(0,a0,a1,v2);
}

// Function: sub_b4d0 @ 0xb4d0
void sub_b4d0(unsigned long a0,char a1)
{
  sub_b440(a0,0xffffffffffffffff,(int)a1); // tail-call
}

// Function: sub_b4f0 @ 0xb4f0
void sub_b4f0(unsigned long a0)
{
  sub_b4d0(a0,0x3a); // tail-call
}

// Function: sub_b500 @ 0xb500
void sub_b500(unsigned long a0,unsigned long a1)
{
  sub_b440(a0,a1,0x3a); // tail-call
}

// Function: sub_b510 @ 0xb510
void sub_b510(unsigned int a0,unsigned long a1,unsigned long a2)
{
  unsigned long v1; // stack - 0x98
  unsigned long v10; // stack - 0x50
  unsigned long v11; // stack - 0x48
  unsigned long v12; // stack - 0x40
  unsigned long v13; // stack - 0x38
  unsigned long v14; // stack - 0x30
  unsigned long v15; // stack - 0x28
  unsigned long v2; // stack - 0x58
  unsigned long *v3;
  unsigned long v4; // stack - 0x90
  unsigned long v5; // stack - 0x88
  unsigned long v6; // stack - 0x80
  unsigned long v7; // stack - 0x78
  unsigned long v8; // stack - 0x70
  unsigned long v9; // stack - 0x68
  
  sub_9240(&v1);
  v3 = &v2;
  v2 = v1;
  v10 = v4;
  v15 = v9;
  v11 = v5;
  v12 = v6;
  v13 = v7;
  v14 = v8;
  sub_b030(v3,0x3a,1);
  sub_adc0(a0,a2,0xffffffffffffffff,v3);
}

// Function: sub_b5b0 @ 0xb5b0
void sub_b5b0(unsigned int a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4)
{
  unsigned long v1; // stack - 0x68
  unsigned long v2; // stack - 0x60
  unsigned long v3; // stack - 0x58
  unsigned long v4; // stack - 0x50
  unsigned long v5; // stack - 0x48
  unsigned long v6; // stack - 0x40
  unsigned long v7; // stack - 0x38
  
  v1 = dat_15240;
  v2 = dat_15248;
  v7 = dat_15270;
  v3 = dat_15250;
  v4 = dat_15258;
  v5 = dat_15260;
  v6 = dat_15268;
  sub_b090(&v1);
  sub_adc0(a0,a3,a4,&v1);
}

// Function: sub_b650 @ 0xb650
void sub_b650(void)
{
  sub_b5b0(); // tail-call
}

// Function: sub_b660 @ 0xb660
void sub_b660(unsigned long a0,unsigned long a1,unsigned long a2)
{
  sub_b650(0,a0,a1,a2); // tail-call
}

// Function: sub_b680 @ 0xb680
void sub_b680(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  sub_b5b0(0,a0,a1,a2,a3); // tail-call
}

// Function: sub_b6a0 @ 0xb6a0
void sub_b6a0(void)
{
  sub_adc0(); // tail-call
}

// Function: sub_b6b0 @ 0xb6b0
void sub_b6b0(unsigned long a0,unsigned long a1)
{
  sub_b6a0(0,a0,a1); // tail-call
}

// Function: sub_b6c0 @ 0xb6c0
void sub_b6c0(unsigned long a0,unsigned long a1)
{
  sub_b6a0(a0,a1,0xffffffffffffffff); // tail-call
}

// Function: sub_b6d0 @ 0xb6d0
void sub_b6d0(unsigned long a0)
{
  sub_b6c0(0,a0); // tail-call
}

// Function: sub_b6e0 @ 0xb6e0
long sub_b6e0(int a0,void *a1,unsigned long a2)
{
  int v1;
  long v2; // rax
  
  while( true ) {
    do {
      v2 = read(a0,a1,a2);
      if (0 <= v2)
        return v2;
      v1 = *__errno_location();
    } while (v1 == 4);
    if (a2 <= 0x7ff00000) break;
    if (v1 != 0x16)
      return v2;
    a2 = 0x7ff00000;
  }
  return v2;
}

// Function: sub_b750 @ 0xb750
unsigned long sub_b750(long a0)
{
  return *(unsigned long *)(a0 + 0x50);
}

// Function: sub_b760 @ 0xb760
unsigned long sub_b760(long a0)
{
  return *(unsigned long *)(a0 + 0x70);
}

// Function: sub_b770 @ 0xb770
unsigned long sub_b770(long a0)
{
  return *(unsigned long *)(a0 + 0x60);
}

// Function: sub_b780 @ 0xb780
unsigned long sub_b780(void)
{
  return 0;
}

// Function: sub_b790 @ 0xb790
undefined16 sub_b790(long a0)
{
  return *(char (*)[16])(a0 + 0x48);
}

// Function: sub_b7a0 @ 0xb7a0
undefined16 sub_b7a0(long a0)
{
  return *(char (*)[16])(a0 + 0x68);
}

// Function: sub_b7b0 @ 0xb7b0
undefined16 sub_b7b0(long a0)
{
  return *(char (*)[16])(a0 + 0x58);
}

// Function: sub_b7c0 @ 0xb7c0
undefined16 sub_b7c0(void)
{
  char v1 [16];
  
  v1._8_8_ = 0xffffffffffffffff;
  v1._0_8_ = 0xffffffffffffffff;
  return v1._0_16_;
}

// Function: sub_b7e0 @ 0xb7e0
unsigned int sub_b7e0(unsigned int a0)
{
  return a0;
}

// Function: sub_b7f0 @ 0xb7f0
undefined16 sub_b7f0(unsigned long a0,unsigned long a1)
{
  char v1 [16];
  
  v1._8_8_ = a1;
  v1._0_8_ = a0;
  return v1._0_16_;
}

// Function: sub_b800 @ 0xb800
int sub_b800(long a0,long a1,long a2,long a3)
{
  return ((unsigned int)(a3 < a1) - (unsigned int)(a1 < a3)) + ((unsigned int)(a2 < a0) - (unsigned int)(a0 < a2)) * 2;
}

// Function: sub_b830 @ 0xb830
int sub_b830(long a0,long a1,unsigned long a2)
{
  return ((unsigned int)(a0 == 0) & (unsigned int)CONCAT71((undefined7)((unsigned long)a2 >> 8),a1 != 0)) + (unsigned int)(0 < a0) + (int)(a0 >> 0x3f);
}

// Function: sub_b860 @ 0xb860
double sub_b860(long a0,long a1)
{
  return (double)a1 / dat_fa48 + (double)a0;
}

// Function: sub_b890 @ 0xb890
unsigned int sub_b890(unsigned int a0)
{
  int v1;
  unsigned int v2; // eax
  int *v3; // rax
  
  if (3 <= a0)
    return a0;
  v2 = sub_d2d0();
  v3 = __errno_location();
  v1 = *v3;
  close(a0);
  *v3 = v1;
  return v2;
}

// Function: sub_b8f0 @ 0xb8f0
long sub_b8f0(void *a0,long a1,unsigned long a2,unsigned long a3,unsigned long *a4,unsigned long a5)
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
  __fprintf_chk(a0,1,"Copyright %s %d Free Software Foundation, Inc.",dcgettext(NULL,"(C)",5),0x7e6);
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
label_ba1d:
    v1 = CONCAT44(dat_4,__fprintf_chk(a0,1,v2,v4,v3,v5,v6,v7,v8,v9,v10));
label_ba56:
    return v1;
  }
  v1 = (long)*(int *)(a5 * 4 + 0xfd50) + 0xfd50;
  switch(a5) {
    case 0:
      goto label_ba56;
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
      goto label_ba1d;
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

// Function: sub_bd70 @ 0xbd70
void sub_bd70(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,long *a4)
{
  long v1;
  long v2;
  
  v2 = 0;
  v1 = *a4;
  while (v1) {
    v2 += 1;
    v1 = a4[v2];
  }
  sub_b8f0(); // tail-call
}

// Function: sub_bd90 @ 0xbd90
void sub_bd90(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned int *a4)
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
  sub_b8f0(a0,a1,a2,a3);
}

// Function: sub_be30 @ 0xbe30
void sub_be30(void)
{
  sub_bd90();
}

// Function: sub_bee0 @ 0xbee0
void sub_bee0(void)
{
  fputs_unlocked("\n",stdout);
  __printf_chk(1,dcgettext(NULL,"Report bugs to: %s\n",5),"bug-coreutils@gnu.org");
  __printf_chk(1,dcgettext(NULL,"%s home page: <%s>\n",5),"GNU coreutils","https://www.gnu.org/software/coreutils/");
  __printf_chk(1,dcgettext(NULL,"General help using GNU software: <%s>\n",5),"https://www.gnu.org/gethelp/"); // tail-call
}

// Function: sub_bf90 @ 0xbf90
long sub_bf90(long a0)
{
  if (a0)
    return a0;
  sub_c360(); // no-return
}

// Function: sub_bfa0 @ 0xbfa0
void sub_bfa0(unsigned long a0)
{
  sub_bf90(malloc(a0)); // tail-call
}

// Function: sub_bfc0 @ 0xbfc0
void sub_bfc0(void)
{
  sub_bf90(sub_d0c0()); // tail-call
}

// Function: sub_bfe0 @ 0xbfe0
void sub_bfe0(void)
{
  sub_bfa0(); // tail-call
}

// Function: sub_bff0 @ 0xbff0
void sub_bff0(void *a0,unsigned long a1)
{
  if ((!realloc(a0,a1)) && ((!a0 || (a1))))
    sub_c360(); // no-return
}

// Function: sub_c030 @ 0xc030
void sub_c030(void)
{
  sub_bf90(sub_d0d0()); // tail-call
}

// Function: sub_c050 @ 0xc050
void sub_c050(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!reallocarray(a0,a1,a2)) && ((!a0 || ((a1 && (a2))))))
    sub_c360(); // no-return
}

// Function: sub_c090 @ 0xc090
void sub_c090(void)
{
  sub_c050(); // tail-call
}

// Function: sub_c0a0 @ 0xc0a0
void sub_c0a0(void)
{
  sub_bf90(sub_d100()); // tail-call
}

// Function: sub_c0c0 @ 0xc0c0
void sub_c0c0(unsigned long a0,unsigned long a1)
{
  sub_c050(0,a0,a1); // tail-call
}

// Function: sub_c0d0 @ 0xc0d0
void sub_c0d0(unsigned long a0,unsigned long a1)
{
  sub_c0a0(0,a0,a1); // tail-call
}

// Function: sub_c0e0 @ 0xc0e0
void sub_c0e0(long a0,unsigned long *a1,unsigned long a2)
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
      sub_c360(); // no-return
  }
  else if (!v3) {
    v1._8_8_ = 0;
    v1._0_8_ = a2;
    v3 = SUB168((ZEXT816(0) << 0x40 | ZEXT816(0x80)) / v1._0_16_,0) + (unsigned long)(0x81 <= a2);
  }
  sub_c050(a0,v3,a2);
  *a1 = v3;
}

// Function: sub_c150 @ 0xc150
void sub_c150(unsigned long a0,unsigned long a1)
{
  sub_c0e0(a0,a1,1); // tail-call
}

// Function: sub_c160 @ 0xc160
void sub_c160(long a0,long *a1,long a2,long a3,long a4)
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
    if (0x80 <= v3) goto label_c206;
    v3 = 0x80;
  }
  v2 = v3 / a4;
  v3 -= v3 % a4;
label_c206:
  if (!a0)
    *a1 = 0;
  if ((v2 - v1 < a2) && ((v2 = v1 + a2, SCARRY8(v1,a2) || (((a3 < v2 && (0 <= a3)) || (v3 = v2 * a4, SEXT816(v3) != SEXT816(v2) * SEXT816(a4)))))))
    sub_c360(a0,v3); // no-return
  sub_bff0(a0,v3);
  *a1 = v2;
}

// Function: sub_c240 @ 0xc240
void sub_c240(unsigned long a0,unsigned long a1)
{
  sub_bf90(calloc(a0,a1)); // tail-call
}

// Function: sub_c260 @ 0xc260
void sub_c260(unsigned long a0)
{
  sub_c240(a0,1); // tail-call
}

// Function: sub_c270 @ 0xc270
void sub_c270(void)
{
  sub_bf90(sub_d0f0()); // tail-call
}

// Function: sub_c290 @ 0xc290
void sub_c290(unsigned long a0)
{
  sub_c270(a0,1); // tail-call
}

// Function: sub_c2a0 @ 0xc2a0
void sub_c2a0(void *a0,unsigned long a1)
{
  memcpy((void *)sub_bfa0(a1),a0,a1); // tail-call
}

// Function: sub_c2d0 @ 0xc2d0
void sub_c2d0(void *a0,unsigned long a1)
{
  memcpy((void *)sub_bfc0(a1),a0,a1); // tail-call
}

// Function: sub_c300 @ 0xc300
void sub_c300(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)sub_bfc0(a1 + 1);
  *(char *)((long)v1 + a1) = 0;
  memcpy(v1,a0,a1); // tail-call
}

// Function: sub_c340 @ 0xc340
void sub_c340(char *a0)
{
  sub_c2a0(a0,strlen(a0) + 1); // tail-call
}

// Function: sub_c360 @ 0xc360
void sub_c360(void)
{
  error(dat_15040,0,"%s",dcgettext(NULL,"memory exhausted",5));
  abort(); // no-return
}

// Function: sub_c3a0 @ 0xc3a0
unsigned long sub_c3a0(unsigned long a0,unsigned int a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,int a6)
{
  int v1;
  int *v2; // rax
  unsigned long v3; // stack - 0x48
  
  v1 = sub_c6c0(a0,0,a1,&v3);
  if (v1) { // branch-flip
    v2 = __errno_location();
    if (v1 != 1) {
      if (v1 == 3)
        *v2 = 0;
      goto label_c448;
    }
  }
  else {
    if ((a2 <= v3) && (v3 <= a3))
      return v3;
    v2 = __errno_location();
    if (v3 <= 0x3fffffff) {
      *v2 = 0x22;
      goto label_c448;
    }
  }
  *v2 = 0x4b;
label_c448:
  sub_b6d0(a0);
  v1 = *v2;
  if (v1 == 0x16) goto label_c480;
  do {
    if (!a6)
      a6 = 1;
    error(a6,v1,"%s: %s",a5);
label_c480:
    v1 = 0;
  } while( true );
}

// Function: sub_c4b0 @ 0xc4b0
void sub_c4b0(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5)
{
  sub_c3a0(a0,10,a1,a2,a3,a4,a5);
}

// Function: sub_c4e0 @ 0xc4e0
unsigned long sub_c4e0(double a0)
{
  unsigned int *v1; // rax
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x38
  char v4 [16];
  unsigned long v5; // stack - 0x30
  
  v1 = (unsigned int *)__errno_location();
  if (dat_fdc0 <= a0) {
    do {
      pause();
      if (*v1 != 4) break;
      pause();
    } while (*v1 == 4);
  }
  v4 = sub_ce80();
  v5 = SUB168(v4,8);
  v3 = SUB168(v4,0);
  do {
    *v1 = 0;
    v2 = sub_d200(&v3,&v3);
    if (!(int)v2)
      return v2;
  } while (!(*v1 & 0xfffffffb));
  return 0xffffffff;
}

// Function: sub_c590 @ 0xc590
bool sub_c590(char *a0,unsigned long *a1,double *a2,void *a3)
{
  int *v1; // rax
  char *v2; // stack - 0x38
  bool v3; // r8b
  double v4; // xmm0_qa
  
  v1 = __errno_location();
  *v1 = 0;
  v4 = (double)(*a3)(a0,&v2);
  if (v2 != a0) { // branch-flip
    if (!a1) {
      v3 = 0;
      if ((*v2) || (v3 = 1, v4 == dat_fdc8)) goto label_c605;
label_c5f3:
      v3 = *v1 != 0x22;
      goto label_c5fc;
    }
    v3 = 1;
    if (v4 != dat_fdc8) goto label_c5f3;
  }
  else {
    v3 = 0;
label_c5fc:
    if (!a1) goto label_c605;
  }
  *a1 = v2;
label_c605:
  *a2 = v4;
  return v3;
}

// Function: sub_c660 @ 0xc660
bool sub_c660(unsigned long *a0,int a1)
{
  char v1 [16];
  char v2 [16];
  unsigned long v3; // rax
  bool v4; // cf
  
  v1._8_8_ = 0;
  v1._0_8_ = (long)a1;
  v2._8_8_ = 0;
  v2._0_8_ = *a0;
  v3 = SUB168(v1._0_16_ * v2._0_16_,0);
  v4 = SUB168(v1._0_16_ * v2._0_16_,8) != 0;
  if (v4)
    v3 = 0xffffffffffffffff;
  *a0 = v3;
  return v4;
}

// Function: sub_c690 @ 0xc690
unsigned int sub_c690(unsigned long a0,unsigned long a1,int a2)
{
  unsigned int v1; // r9d
  
  v1 = 0;
  do {
    a2 -= 1;
    v1 |= sub_c660();
  } while (a2);
  return v1;
}

// Function: sub_c6c0 @ 0xc6c0
unsigned int sub_c6c0(unsigned char *a0,unsigned long *a1,unsigned int a2,unsigned long *a3,char *a4)
{
  unsigned char v1;
  unsigned int v10;
  unsigned int v2; // eax
  int *v3; // rax
  unsigned char *v4;
  unsigned char v5;
  long v6;
  unsigned long v7; // stack - 0x48
  int v8; // ecx
  unsigned long v9; // r8
  
  if (0x25 <= a2)
    __assert_fail("0 <= strtol_base && strtol_base <= 36","lib/xstrtol.c",0x55,"xstrtoumax"); // no-return
  if (!a1)
    a1 = &v6;
  v3 = __errno_location();
  *v3 = 0;
  v5 = *a0;
  v4 = a0;
  while (*(unsigned char *)(*(long *)__ctype_b_loc() + 1 + (unsigned long)v5 * 2) & 0x20) {
    v5 = v4[1];
    v4 = &v4[1];
  }
  if (v5 == 0x2d)
    return 4;
  v7 = strtoumax(a0,a1,a2);
  v4 = (unsigned char *)*a1;
  if (v4 != a0) { // branch-flip
    if (*v3) { // branch-flip
      v10 = 1;
      if (*v3 != 0x22)
        return 4;
    }
    else {
      v10 = 0;
    }
    if (!a4) {
      *a3 = v7;
      return v10;
    }
    v5 = *v4;
    if (!v5) goto label_c782;
    v8 = (int)(char)v5;
  }
  else {
    if ((!a4) || (v5 = *a0, !v5))
      return 4;
    v8 = (int)(char)v5;
    if (!strchr(a4,v8))
      return 4;
    v7 = 1;
    v10 = 0;
  }
  if (!strchr(a4,v8)) {
label_c789:
    *a3 = v7;
    return v10 | 2;
  }
  v2 = (unsigned int)v5 - 0x45;
  v8 = 1;
  v9 = 0x400;
  if (((unsigned char)v2 <= 0x2f) && (0x814400308945U >> ((unsigned long)v2 & 0x3f) & 1)) {
    v9 = 0x400;
    v8 = 1;
    if (strchr(a4,0x30)) {
      v1 = v4[1];
      if (v1 != 0x44) { // branch-flip
        if (v1 != 0x69) { // branch-flip
          v8 = (v1 == 0x42) + 1;
          v9 = 0x400;
          if (v1 == 0x42)
            v9 = 1000;
        }
        else {
          v8 = (v4[2] == 0x42) + 1 + (unsigned int)(v4[2] == 0x42);
        }
      }
      else {
        v8 = 2;
        v9 = 1000;
      }
    }
  }
  switch((unsigned int)v5 - 0x42 & 0xff) {
    case 0:
      v10 |= sub_c660(&v7,0x400);
      break;
    default:
      goto label_c789;
    case 3:
      v10 |= sub_c690(&v7,v9,6);
      break;
    case 5:
    case 0x25:
      v10 |= sub_c690(&v7,v9,3);
      break;
    case 9:
    case 0x29:
      v10 |= sub_c690(&v7,v9,1);
      break;
    case 0xb:
    case 0x2b:
      v10 |= sub_c690(&v7,v9,2);
      break;
    case 0xe:
      v10 |= sub_c690(&v7,v9,5);
      break;
    case 0x12:
    case 0x32:
      v10 |= sub_c690(&v7,v9,4);
      break;
    case 0x17:
      v10 |= sub_c690(&v7,v9,8);
      break;
    case 0x18:
      v10 |= sub_c690(&v7,v9,7);
      break;
    case 0x20:
      v10 |= sub_c660(&v7,0x200);
      break;
    case 0x21:
      break;
    case 0x35:
      v10 |= sub_c660(&v7,2);
    
  }
  *a1 = &v4[v8];
  if (v4[v8])
    v10 |= 2;
label_c782:
  *a3 = v7;
  return v10;
}

// Function: sub_caa0 @ 0xcaa0
unsigned long sub_caa0(unsigned long a0,unsigned char a1)
{
  return a0 << (a1 & 0x3f) | a0 >> 0x40 - (a1 & 0x3f);
}

// Function: sub_cab0 @ 0xcab0
unsigned long sub_cab0(unsigned long a0,unsigned char a1)
{
  return a0 >> (a1 & 0x3f) | a0 << 0x40 - (a1 & 0x3f);
}

// Function: sub_cac0 @ 0xcac0
unsigned int sub_cac0(unsigned int a0,unsigned char a1)
{
  return a0 << (a1 & 0x1f) | a0 >> 0x20 - (a1 & 0x1f);
}

// Function: sub_cad0 @ 0xcad0
unsigned int sub_cad0(unsigned int a0,unsigned char a1)
{
  return a0 >> (a1 & 0x1f) | a0 << 0x20 - (a1 & 0x1f);
}

// Function: sub_cae0 @ 0xcae0
void sub_cae0(void)
{
  sub_caa0(); // tail-call
}

// Function: sub_caf0 @ 0xcaf0
void sub_caf0(void)
{
  sub_cab0(); // tail-call
}

// Function: sub_cb00 @ 0xcb00
unsigned int sub_cb00(unsigned int a0,unsigned char a1)
{
  return CONCAT22((short)((unsigned int)a0 >> 0x10),(unsigned short)a0 << (a1 & 0xf) | (unsigned short)a0 >> 0x10 - (a1 & 0xf));
}

// Function: sub_cb10 @ 0xcb10
unsigned int sub_cb10(unsigned int a0,unsigned char a1)
{
  return CONCAT22((short)((unsigned int)a0 >> 0x10),(unsigned short)a0 >> (a1 & 0xf) | (unsigned short)a0 << 0x10 - (a1 & 0xf));
}

// Function: sub_cb20 @ 0xcb20
unsigned int sub_cb20(unsigned int a0,unsigned char a1)
{
  return CONCAT31((undefined3)((unsigned int)a0 >> 8),(unsigned char)a0 << (a1 & 7) | (unsigned char)a0 >> 8 - (a1 & 7));
}

// Function: sub_cb30 @ 0xcb30
unsigned int sub_cb30(unsigned int a0,unsigned char a1)
{
  return CONCAT31((undefined3)((unsigned int)a0 >> 8),(unsigned char)a0 >> (a1 & 7) | (unsigned char)a0 << 8 - (a1 & 7));
}

// Function: sub_cb40 @ 0xcb40
unsigned long sub_cb40(int a0) // early-return
{
  unsigned long v1; // rax
  
  if (0x5a < a0)
    return CONCAT71((undefined7)((unsigned long)v1 >> 8),(unsigned int)(a0 - 0x61U) <= 0x19);
  if (0x41 <= a0)
    return 1;
  return (unsigned long)((unsigned int)(a0 - 0x30U) <= 9);
}

// Function: sub_cb70 @ 0xcb70
bool sub_cb70(int a0)
{
  if (a0 <= 0x5a)
    return 0x40 < a0;
  return (unsigned int)(a0 - 0x61U) <= 0x19;
}

// Function: sub_cb90 @ 0xcb90
bool sub_cb90(unsigned int a0)
{
  return a0 <= 0x7f;
}

// Function: sub_cba0 @ 0xcba0
unsigned int sub_cba0(int a0,unsigned long a1,unsigned int a2)
{
  unsigned long v1; // rax
  
  return (unsigned int)CONCAT71((undefined7)((unsigned long)v1 >> 8),a0 == 0x20) | CONCAT31((undefined3)((unsigned int)a2 >> 8),a0 == 9);
}

// Function: sub_cbc0 @ 0xcbc0
unsigned long sub_cbc0(unsigned int a0)
{
  unsigned long v1; // rax
  
  if ((int)a0 <= 0x1f)
    return (unsigned long)(~a0 >> 0x1f);
  return CONCAT71((undefined7)((unsigned long)v1 >> 8),a0 == 0x7f);
}

// Function: sub_cbe0 @ 0xcbe0
bool sub_cbe0(int a0)
{
  return (unsigned int)(a0 - 0x30U) <= 9;
}

// Function: sub_cbf0 @ 0xcbf0
bool sub_cbf0(int a0)
{
  return (unsigned int)(a0 - 0x21U) <= 0x5d;
}

// Function: sub_cc00 @ 0xcc00
bool sub_cc00(int a0)
{
  return (unsigned int)(a0 - 0x61U) <= 0x19;
}

// Function: sub_cc10 @ 0xcc10
bool sub_cc10(int a0)
{
  return (unsigned int)(a0 - 0x20U) <= 0x5e;
}

// Function: sub_cc20 @ 0xcc20
unsigned long sub_cc20(int a0)
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

// Function: sub_cc70 @ 0xcc70
bool sub_cc70(int a0)
{
  if (a0 <= 0xd)
    return 8 < a0;
  return a0 == 0x20;
}

// Function: sub_cc90 @ 0xcc90
bool sub_cc90(int a0)
{
  return (unsigned int)(a0 - 0x41U) <= 0x19;
}

// Function: sub_cca0 @ 0xcca0
unsigned int sub_cca0(int a0)
{
  if ((unsigned int)(a0 - 0x30U) <= 0x36)
    return (unsigned int)(0x7e0000007e03ff >> ((unsigned char)(a0 - 0x30U) & 0x3f)) & 1;
  return 0;
}

// Function: sub_ccd0 @ 0xccd0
int sub_ccd0(int a0)
{
  if ((unsigned int)(a0 - 0x41U) <= 0x19)
    a0 += 0x20;
  return a0;
}

// Function: sub_ccf0 @ 0xccf0
int sub_ccf0(int a0)
{
  if ((unsigned int)(a0 - 0x61U) <= 0x19)
    a0 -= 0x20;
  return a0;
}

// Function: sub_cd10 @ 0xcd10
int sub_cd10(char *a0,char *a1) // early-return
{
  unsigned char v1; // al
  unsigned int v2; // eax
  
  if (a0 == a1)
    return 0;
  do {
    v2 = sub_ccd0(*a0);
    v1 = sub_ccd0(*a1);
    if (!(unsigned char)v2) break;
    a0 = &a0[1];
    a1 = &a1[1];
  } while ((unsigned char)v2 == v1);
  return (v2 & 0xff) - (unsigned int)v1;
}

// Function: sub_cd70 @ 0xcd70
long sub_cd70(void)
{
  if (dat_15278)
    return dat_15278;
  dat_15278 = newlocale(0x1fbf,"C",0);
  return dat_15278;
}

// Function: sub_cdc0 @ 0xcdc0
unsigned long sub_cdc0(unsigned long a0,unsigned long *a1) // return-dupe
{
  long v1; // rax
  unsigned long v2; // xmm0_qa
  
  v1 = sub_cd70();
  if (v1) {
    strtod_l(a0,a1,v1); // tail-call
    return v2;
  }
  if (!a1)
    return 0;
  *a1 = a0;
  return 0;
}

// Function: sub_ce10 @ 0xce10
unsigned long sub_ce10(void *a0) // early-return x2, return-dupe
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // rax
  
  v2 = __fpending(a0);
  v1 = ferror_unlocked(a0);
  v3 = sub_cf50(a0);
  if (v1) {
    if ((int)v3)
      return 0xffffffff;
    *__errno_location() = 0;
    return 0xffffffff;
  }
  if ((int)v3) {
    if (v2)
      return 0xffffffff;
    return (unsigned long)-(unsigned int)(*__errno_location() != 9);
  }
  return v3;
}

// Function: sub_ce80 @ 0xce80
void sub_ce80(double a0)
{
  long v1; // rax
  long v2; // rdi
  double v3; // xmm0_qa
  
  if (a0 <= dat_fef0) {
    sub_b7f0(0x8000000000000000,0); // tail-call
    return;
  }
  if (dat_fdc0 <= a0) {
    sub_b7f0(0x7fffffffffffffff,999999999); // tail-call
    return;
  }
  v3 = (a0 - (double)(long)a0) * dat_fa48;
  v1 = (long)v3;
  v1 = (unsigned long)((double)v1 < v3) + v1;
  v2 = (long)a0 + v1 / 1000000000;
  v1 %= 1000000000;
  if (0 <= v1) {
    sub_b7f0(v2); // tail-call
    return;
  }
  sub_b7f0(v2 + -1,v1 + 1000000000); // tail-call
}

// Function: sub_cf50 @ 0xcf50
unsigned long sub_cf50(void *a0) // early-return
{
  int v1;
  int *v2; // rax
  
  if (0 <= fileno(a0)) {
    if (__freading(a0)) {
      v1 = fileno(a0);
      if (lseek(v1,0,1) == -1) {
        return fclose(a0); // tail-call, return-dupe
      }
    }
    if (sub_d000(a0)) {
      v2 = __errno_location();
      v1 = *v2;
      if (!v1)
        return CONCAT44(dat_4,fclose(a0));
      *v2 = v1;
      return 0xffffffff;
    }
  }
  return fclose(a0);
}

// Function: sub_cfe0 @ 0xcfe0
void sub_cfe0(unsigned int *a0)
{
  if (!(*a0 & 0x100))
    return;
  sub_d040(a0,0,1); // tail-call
}

// Function: sub_d000 @ 0xd000
void sub_d000(void *a0)
{
  if (a0) {
    if (__freading(a0)) {
      sub_cfe0(a0);
      fflush(a0); // tail-call
      return;
    }
  }
  fflush(a0); // tail-call
}

// Function: sub_d040 @ 0xd040
unsigned long sub_d040(unsigned int *a0,unsigned long a1,unsigned int a2) // return-dupe
{
  long v1; // rax
  
  if (((*(long *)&a0[4] == *(long *)&a0[2]) && (*(long *)&a0[10] == *(long *)&a0[8])) && (!*(long *)&a0[0x12])) {
    v1 = lseek(fileno(a0),a1,a2);
    if (v1 != -1) {
      *a0 = *a0 & 0xffffffef;
      *(long *)&a0[0x24] = v1;
      return 0;
    }
    return 0xffffffff;
  }
  return fseeko(a0); // tail-call
}

// Function: sub_d0c0 @ 0xd0c0
void sub_d0c0(unsigned long a0)
{
  malloc(a0); // tail-call
}

// Function: sub_d0d0 @ 0xd0d0
void sub_d0d0(void *a0,unsigned long a1)
{
  realloc(a0,a1 | a1 == 0); // tail-call
}

// Function: sub_d0f0 @ 0xd0f0
void sub_d0f0(unsigned long a0,unsigned long a1)
{
  calloc(a0,a1); // tail-call
}

// Function: sub_d100 @ 0xd100
void sub_d100(void *a0,unsigned long a1,unsigned long a2)
{
  if ((a1) && (a2)) {
    reallocarray(a0,a1,a2); // tail-call
    return;
  }
  reallocarray(a0,1,1); // tail-call
}

// Function: sub_d130 @ 0xd130
char * sub_d130(void) // early-return
{
  char *v1; // rax
  
  v1 = nl_langinfo(0xe);
  if (!v1)
    return "ASCII";
  if (*v1)
    return v1;
  return "ASCII";
}

// Function: sub_d170 @ 0xd170
unsigned long sub_d170(unsigned int *a0,unsigned char *a1,unsigned long a2,void *a3)
{
  unsigned long v1; // rax
  long v2;
  
  if (!a0)
    a0 = &v2;
  v1 = mbrtowc(a0,(char *)a1,a2,a3);
  if (((0xfffffffffffffffe <= v1) && (a2)) && (!sub_d2f0(0))) {
    v1 = 1;
    *a0 = (unsigned int)*a1;
  }
  return v1;
}

// Function: sub_d200 @ 0xd200
unsigned long sub_d200(long *a0,long *a1) // early-return
{
  unsigned long v1; // rax
  long v2;
  long v3; // stack - 0x38
  unsigned long v4; // stack - 0x30
  
  v4 = a0[1];
  if (1000000000 <= v4) {
    *__errno_location() = 0x16;
    return 0xffffffff;
  }
  v2 = *a0;
  while( true ) {
    if (v2 <= 0x1fa400) {
      v3 = v2;
      return nanosleep(&v3,a1);
    }
    v3 = 0x1fa400;
    v2 -= 0x1fa400;
    v1 = nanosleep(&v3,a1);
    if ((int)v1) break;
    v4 = 0;
  }
  if (!a1)
    return v1;
  *a1 = *a1 + v2;
  return v1;
}

// Function: sub_d2d0 @ 0xd2d0
void sub_d2d0(unsigned long a0)
{
  sub_7d50(a0,0,3); // tail-call
}

// Function: sub_d2f0 @ 0xd2f0
bool sub_d2f0(unsigned long a0)
{
  char v1 [264];
  bool v2; // r12b
  
  v2 = 0;
  if ((!sub_d440(a0,v1,0x101)) && (strcmp(v1,"C")))
    v2 = strcmp(v1,"POSIX") != 0;
  return v2;
}

// Function: sub_d380 @ 0xd380
void sub_d380(int a0)
{
  setlocale(a0,NULL); // tail-call
}

// Function: sub_d390 @ 0xd390
unsigned long sub_d390(unsigned long a0,char *a1,unsigned long a2) // return-dupe x2
{
  char *v1; // rax
  unsigned long v2; // rax
  
  v1 = (char *)sub_d380();
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

// Function: sub_d440 @ 0xd440
void sub_d440(void)
{
  sub_d390(); // tail-call
}

// Function: sub_d450 @ 0xd450
void sub_d450(void)
{
  sub_d380(); // tail-call
}

// Function: sub_d460 @ 0xd460
void sub_d460(unsigned long a0)
{
  __cxa_atexit(a0,0,dat_15008); // tail-call
}

// Function: _DT_FINI @ 0xd474
void _DT_FINI(void)
{
  return;
}
