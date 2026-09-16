// Function: _DT_INIT @ 0x2000
void _DT_INIT(void) // return-dupe
{
  if (!dat_5fe0)
    return;
  (*dat_5fe0)();
}

// Function: sub_2020 @ 0x2020
void sub_2020(void)
{
  (*dat_5e50)(); // jump-as-call
}

// Function: __cxa_finalize @ 0x2320
void __cxa_finalize(void)
{
  (*dat_5ff0)(); // jump-as-call
}

// Function: localtime @ 0x2330
void * localtime(void *a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_5e58)(); // jump-as-call
  return v1;
}

// Function: __errno_location @ 0x2340
int * __errno_location(void)
{
  int *v1; // rax
  
  v1 = (int *)(*dat_5e60)(); // jump-as-call
  return v1;
}

// Function: strncmp @ 0x2350
int strncmp(char *a0,char *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_5e68)(); // jump-as-call
  return v1;
}

// Function: chroot @ 0x2360
void chroot(void)
{
  (*dat_5e70)(); // jump-as-call
}

// Function: puts @ 0x2370
int puts(char *a0)
{
  int v1; // eax
  
  v1 = (*dat_5e78)(); // jump-as-call
  return v1;
}

// Function: ferror @ 0x2380
int ferror(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_5e80)(); // jump-as-call
  return v1;
}

// Function: fread @ 0x2390
unsigned long fread(void *a0,unsigned long a1,unsigned long a2,void *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_5e88)(); // jump-as-call
  return v1;
}

// Function: textdomain @ 0x23a0
char * textdomain(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_5e90)(); // jump-as-call
  return v1;
}

// Function: fclose @ 0x23b0
int fclose(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_5e98)(); // jump-as-call
  return v1;
}

// Function: getpwuid @ 0x23c0
void * getpwuid(unsigned int a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_5ea0)(); // jump-as-call
  return v1;
}

// Function: bindtextdomain @ 0x23d0
char * bindtextdomain(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_5ea8)(); // jump-as-call
  return v1;
}

// Function: dcgettext @ 0x23e0
char * dcgettext(char *a0,char *a1,int a2)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_5eb0)(); // jump-as-call
  return v1;
}

// Function: chdir @ 0x23f0
int chdir(char *a0)
{
  int v1; // eax
  
  v1 = (*dat_5eb8)(); // jump-as-call
  return v1;
}

// Function: __stack_chk_fail @ 0x2400
void __stack_chk_fail(void)
{
  (*dat_5ec0)(); // jump-as-call
}

// Function: getuid @ 0x2410
unsigned int getuid(void)
{
  unsigned int v1; // eax
  
  v1 = (*dat_5ec8)(); // jump-as-call
  return v1;
}

// Function: getopt_long @ 0x2420
int getopt_long(int a0,char **a1,char *a2,void *a3,int *a4)
{
  int v1; // eax
  
  v1 = (*dat_5ed0)(); // jump-as-call
  return v1;
}

// Function: strrchr @ 0x2430
char * strrchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_5ed8)(); // jump-as-call
  return v1;
}

// Function: __assert_fail @ 0x2440
void __assert_fail(char *a0,char *a1,unsigned int a2,char *a3)
{
  (*dat_5ee0)(); // jump-as-call
}

// Function: fputs @ 0x2450
int fputs(char *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_5ee8)(); // jump-as-call
  return v1;
}

// Function: strcmp @ 0x2460
int strcmp(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_5ef0)(); // jump-as-call
  return v1;
}

// Function: putc @ 0x2470
int putc(int a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_5ef8)(); // jump-as-call
  return v1;
}

// Function: getpwnam @ 0x2480
void * getpwnam(char *a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_5f00)(); // jump-as-call
  return v1;
}

// Function: setpwent @ 0x2490
void setpwent(void)
{
  (*dat_5f08)(); // jump-as-call
}

// Function: strtol @ 0x24a0
long strtol(char *a0,char **a1,int a2)
{
  long v1; // rax
  
  v1 = (*dat_5f10)(); // jump-as-call
  return v1;
}

// Function: getpwent @ 0x24b0
void getpwent(void)
{
  (*dat_5f18)(); // jump-as-call
}

// Function: time @ 0x24c0
void time(void)
{
  (*dat_5f20)(); // jump-as-call
}

// Function: fileno @ 0x24d0
int fileno(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_5f28)(); // jump-as-call
  return v1;
}

// Function: getgid @ 0x24e0
unsigned int getgid(void)
{
  unsigned int v1; // eax
  
  v1 = (*dat_5f30)(); // jump-as-call
  return v1;
}

// Function: fflush @ 0x24f0
int fflush(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_5f38)(); // jump-as-call
  return v1;
}

// Function: endpwent @ 0x2500
void endpwent(void)
{
  (*dat_5f40)(); // jump-as-call
}

// Function: setreuid @ 0x2510
void setreuid(void)
{
  (*dat_5f48)(); // jump-as-call
}

// Function: setregid @ 0x2520
void setregid(void)
{
  (*dat_5f50)(); // jump-as-call
}

// Function: setlocale @ 0x2530
char * setlocale(int a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_5f58)(); // jump-as-call
  return v1;
}

// Function: __printf_chk @ 0x2540
int __printf_chk(int a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_5f60)(); // jump-as-call
  return v1;
}

// Function: strftime @ 0x2550
unsigned long strftime(char *a0,unsigned long a1,char *a2,void *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_5f68)(); // jump-as-call
  return v1;
}

// Function: fsync @ 0x2560
int fsync(int a0)
{
  int v1; // eax
  
  v1 = (*dat_5f70)(); // jump-as-call
  return v1;
}

// Function: __explicit_bzero_chk @ 0x2570
void __explicit_bzero_chk(void *a0,unsigned long a1,unsigned long a2)
{
  (*dat_5f78)(); // jump-as-call
}

// Function: access @ 0x2580
int access(char *a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_5f80)(); // jump-as-call
  return v1;
}

// Function: fseeko @ 0x2590
void fseeko(void)
{
  (*dat_5f88)(); // jump-as-call
}

// Function: fopen @ 0x25a0
void * fopen(char *a0,char *a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_5f90)(); // jump-as-call
  return v1;
}

// Function: strtoul @ 0x25b0
unsigned long strtoul(char *a0,char **a1,int a2)
{
  unsigned long v1; // rax
  
  v1 = (*dat_5f98)(); // jump-as-call
  return v1;
}

// Function: exit @ 0x25c0
void exit(int a0)
{
  (*dat_5fa0)(); // jump-as-call
}

// Function: fwrite @ 0x25d0
unsigned long fwrite(void *a0,unsigned long a1,unsigned long a2,void *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_5fa8)(); // jump-as-call
  return v1;
}

// Function: __fprintf_chk @ 0x25e0
int __fprintf_chk(void *a0,int a1,char *a2,...)
{
  int v1; // eax
  
  v1 = (*dat_5fb0)(); // jump-as-call
  return v1;
}

// Function: strerror @ 0x25f0
char * strerror(int a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_5fb8)(); // jump-as-call
  return v1;
}

// Function: fstat @ 0x2600
int fstat(int a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_5fc0)(); // jump-as-call
  return v1;
}

// Function: __ctype_b_loc @ 0x2610
void * __ctype_b_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_5fc8)(); // jump-as-call
  return v1;
}

// Function: sub_2620 @ 0x2620
void sub_2620(int a0,char **a1)
{
  unsigned int v1;
  unsigned long v10; // stack - 0x58
  long v11; // stack - 0x50
  long v12; // stack - 0x48
  unsigned long v13; // rbx
  char *v14;
  long v15; // fs_offset
  unsigned long v16; // stack - 0x40
  unsigned long v2;
  char v3; // al
  int v4; // eax
  int v5; // eax
  void *v6;
  char *v7;
  long v8; // rax
  unsigned int v9;
  
  v5 = 0;
  v16 = *(unsigned long *)(v15 + 0x28);
  v10 = 0;
  v11 = 0;
  dat_61c0 = sub_38d0(*a1);
  sub_3e60(dat_61c0);
  sub_3e80(stderr);
  setlocale(6,"");
  bindtextdomain("shadow","/usr/share/locale");
  textdomain("shadow");
  sub_3ad0("-R",a0,a1);
label_26d3:
  v4 = getopt_long(a0,a1,"ahl:m:rR:t:u:",(void *)0x6020,NULL);
  v2 = dat_61c0;
  if (v4 == -1) {
    if (optind < a0) {
      v7 = a1[optind];
      __fprintf_chk(stderr,1,dcgettext(NULL,"%s: unexpected argument: %s\n",5),v2,v7);
      sub_3050(1); // no-return
    }
    if (dat_6273) { // branch-flip
      if (((dat_6272) || (dat_6271)) || (dat_6270)) {
        sub_3050(2); // no-return, return-dupe
      }
label_29f1:
      if (dat_6270) goto label_282e;
      dat_6298 = fopen("/var/log/faillog","r");
    }
    else {
      if ((!dat_6272) && (!dat_6271)) goto label_29f1;
label_282e:
      dat_6298 = fopen("/var/log/faillog","r+");
    }
    if (!dat_6298) {
      v7 = strerror(*__errno_location());
      v14 = "%s: Cannot open %s: %s\n";
label_2aaa:
      v2 = dat_61c0;
      __fprintf_chk(stderr,1,dcgettext(NULL,v14,5),v2,"/var/log/faillog",v7);
      exit(1); // no-return
    }
    v4 = fileno(dat_6298);
    v4 = fstat(v4,(void *)0x61e0);
    v2 = v10;
    if (v4) {
      v7 = strerror(*__errno_location());
      v14 = "%s: Cannot get the size of %s: %s\n";
      goto label_2aaa;
    }
    if (dat_6272) {
      if (((dat_6274) && (dat_6280)) && ((dat_6277 && (dat_6288 == dat_6278)))) {
        v3 = sub_31a0(dat_6288 & 0xffffffff,v10);
        if (v3)
          dat_6276 = 1;
      }
      else if (dat_6275) { // branch-flip
        v9 = (unsigned int)(dat_6210 >> 5);
        v13 = 0;
        if (dat_6280)
          v13 = dat_6288 & 0xffffffff;
        v9 = (v9 - 1) + (unsigned int)(v9 < 2);
        if (dat_6277)
          v9 = dat_6278;
        for (; (unsigned int)v13 <= v9; v13 = (unsigned long)((unsigned int)v13 + 1)) {
          if (sub_31a0(v13,v2))
            dat_6276 = 1;
        }
      }
      else {
        setpwent();
        while (v8 = getpwent(), v8) {
          v9 = *(unsigned int *)(v8 + 0x10);
          if (((!dat_6274) || (((!dat_6280 || (dat_6288 <= v9)) && ((!dat_6277 || (v9 <= dat_6278)))))) && (sub_31a0(v9,v2)))
            dat_6276 = 1;
        }
        endpwent();
      }
    }
    if (dat_6271) {
      if ((((dat_6274) && (dat_6280)) && (dat_6277)) && (dat_6288 == dat_6278)) {
        v13 = dat_6288 & 0xffffffff;
        if (sub_3320(v13,v5))
          dat_6276 = 1;
      }
      else if (dat_6275) { // branch-flip
        v9 = (unsigned int)(dat_6210 >> 5);
        v13 = 0;
        if (dat_6280)
          v13 = dat_6288 & 0xffffffff;
        v9 = (v9 - 1) + (unsigned int)(v9 < 2);
        if (dat_6277)
          v9 = dat_6278;
        for (; (unsigned int)v13 <= v9; v13 = (unsigned long)((unsigned int)v13 + 1)) {
          if (sub_3320(v13,v5))
            dat_6276 = 1;
        }
      }
      else {
        setpwent();
        while (v8 = getpwent(), v8) {
          v9 = *(unsigned int *)(v8 + 0x10);
          if (((!dat_6274) || (((!dat_6280 || (dat_6288 <= v9)) && ((!dat_6277 || (v9 <= dat_6278)))))) && (sub_3320(v9,v5)))
            dat_6276 = 1;
        }
        endpwent();
      }
    }
    if (dat_6270) {
      if ((dat_6274) && (dat_6280)) {
        if (!dat_6277) {
          v9 = (unsigned int)(dat_6210 >> 5);
          v13 = dat_6210 >> 5 & 0xffffffff;
          if (2 <= v9) goto label_2b51;
          if (dat_6275) goto label_2de8;
          goto label_2df3;
        }
        if (dat_6288 != dat_6278) {
          v9 = (unsigned int)(dat_6210 >> 5);
          if (2 <= v9) goto label_2b51;
          goto label_2b58;
        }
        v13 = dat_6288 & 0xffffffff;
        if (sub_34a0(v13))
          dat_6276 = 1;
      }
      else {
        v9 = (unsigned int)(dat_6210 >> 5);
        v13 = dat_6210 >> 5 & 0xffffffff;
        if (2 <= v9) {
label_2b51:
          v13 = (unsigned long)(v9 - 1);
        }
        v9 = (unsigned int)v13;
        if (dat_6277) {
label_2b58:
          v13 = (unsigned long)v9;
          if (dat_6278 < v9)
            v13 = dat_6278 & 0xffffffff;
        }
        v9 = (unsigned int)v13;
        if (dat_6275) { // branch-flip
          v9 = 0;
          if (dat_6280) {
label_2de8:
            v9 = dat_6288;
          }
          for (; v9 <= (unsigned int)v13; v9 = v9 + 1) {
            if (sub_34a0(v9))
              dat_6276 = 1;
          }
        }
        else {
label_2df3:
          setpwent();
          while (v8 = getpwent(), v8) {
            v1 = *(unsigned int *)(v8 + 0x10);
            if (((!dat_6274) || (((!dat_6280 || (dat_6288 <= v1)) && (v1 <= v9)))) && (sub_34a0(v1)))
              dat_6276 = 1;
          }
          endpwent();
        }
      }
    }
    if ((dat_6272) || (dat_6271)) {
label_2a7c:
      v6 = dat_6298;
    }
    else if (dat_6270) { // branch-flip
label_28c4:
      v6 = dat_6298;
      if (!dat_6270) {
        fclose(dat_6298);
        exit((unsigned int)dat_6276); // return-dupe, no-return
      }
    }
    else {
      if (((dat_6274) && (dat_6280)) && ((dat_6277 && (dat_6288 == dat_6278)))) {
        v9 = (unsigned int)dat_6288;
        sub_3630(getpwuid(v9),1);
      }
      else {
        setpwent();
        while (v8 = getpwent(), v8) {
          if ((!dat_6274) || (((!dat_6280 || (dat_6288 <= *(unsigned int *)(v8 + 0x10))) && ((!dat_6277 || (*(unsigned int *)(v8 + 0x10) <= dat_6278))))))
            sub_3630(v8,dat_6275);
        }
        endpwent();
      }
      if (dat_6272) goto label_2a7c;
      v6 = dat_6298;
      if (!dat_6271) goto label_28c4;
    }
    if ((!ferror(v6)) && (!fflush(v6))) {
      v5 = fileno(dat_6298);
      if ((!fsync(v5)) && (v5 = fclose(dat_6298), !v5)) {
        exit((unsigned int)dat_6276);
      }
    }
    v7 = strerror(*__errno_location());
    v2 = dat_61c0;
    __fprintf_chk(stderr,1,dcgettext(NULL,"%s: Failed to write %s: %s\n",5),v2,"/var/log/faillog",v7);
    fclose(dat_6298);
    dat_6276 = 1;
    exit((unsigned int)dat_6276);
  }
  switch(v4) {
    case 0x52:
      goto label_26d3;
    default:
      goto label_2701;
    case 0x61:
      dat_6275 = '\x01';
      goto label_26d3;
    case 0x68:
      sub_3050(0); // no-return
    case 0x6c:
      v4 = sub_3de0(optarg,&v10);
      if (v4) {
        dat_6272 = '\x01';
        goto label_26d3;
      }
      break;
    case 0x6d:
      v5 = sub_3de0(optarg,&v12);
      if ((v5) && (v5 = (int)(short)v12, v12 == (short)v12)) {
        dat_6271 = '\x01';
        goto label_26d3;
      }
      break;
    case 0x72:
      dat_6270 = '\x01';
      goto label_26d3;
    case 0x74:
      v4 = sub_3de0(optarg,&v11);
      if (v4) {
        dat_6290 = v11 * 0x15180;
        dat_6273 = '\x01';
        goto label_26d3;
      }
      break;
    case 0x75:
      dat_6274 = '\x01';
      v6 = getpwnam(optarg);
      if (v6) { // branch-flip
        dat_6278 = (unsigned long)*(unsigned int *)((long)v6 + 0x10);
        dat_6280 = '\x01';
        dat_6277 = '\x01';
        dat_6288 = dat_6278;
      }
      else {
        v4 = sub_3900(optarg,0x6288,0x6280,0x6278,0x6277);
        if (!v4) {
          v7 = "%s: Unknown user or range: %s\n";
          goto label_29a2;
        }
      }
      goto label_26d3;
    
  }
  v7 = "%s: invalid numeric argument \'%s\'\n";
label_29a2:
  v2 = dat_61c0;
  v14 = optarg;
  __fprintf_chk(stderr,1,dcgettext(NULL,v7,5),v2,v14);
  exit(3); // no-return
  sub_3050(2);
}

// Function: sub_2f60 @ 0x2f60
void sub_2f60(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_5fd0)(sub_2620,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: sub_2f90 @ 0x2f90
void sub_2f90(void)
{
  return;
}

// Function: _FINI_0 @ 0x3000
void _FINI_0(void)
{
  if (!dat_61a8) {
    if (dat_5ff0)
      __cxa_finalize(dat_6008);
    sub_2f90();
    dat_61a8 = 1;
    return;
  }
}

// Function: _INIT_0 @ 0x3040
void _INIT_0(void)
{
  return;
}

// Function: sub_3050 @ 0x3050
void sub_3050(int a0)
{
  unsigned long v1;
  void *v2;
  
  v1 = dat_61c0;
  v2 = stderr;
  if (!a0)
    v2 = stdout;
  __fprintf_chk(v2,1,dcgettext(NULL,"Usage: %s [options]\n\nOptions:\n",5),v1);
  fputs(dcgettext(NULL,"  -a, --all                     display faillog records for all users\n",5),v2);
  fputs(dcgettext(NULL,"  -h, --help                    display this help message and exit\n",5),v2);
  fputs(dcgettext(NULL,"  -l, --lock-secs SEC           after failed login lock account for SEC seconds\n",5),v2);
  fputs(dcgettext(NULL,"  -m, --maximum MAX             set maximum failed login counters to MAX\n",5),v2);
  fputs(dcgettext(NULL,"  -r, --reset                   reset the counters of login failures\n",5),v2);
  fputs(dcgettext(NULL,"  -R, --root CHROOT_DIR         directory to chroot into\n",5),v2);
  fputs(dcgettext(NULL,"  -t, --time DAYS               display faillog records more recent than DAYS\n",5),v2);
  fputs(dcgettext(NULL,"  -u, --user LOGIN/RANGE        display faillog record or maintains failure\n                                counters and limits (if used with -r, -m,\n                                or -l) only for the specified LOGIN(s)\n",5),v2);
  fputs("\n",v2);
  exit(a0); // no-return
}

// Function: sub_31a0 @ 0x31a0
unsigned long sub_31a0(unsigned long a0,long a1)
{
  unsigned long v1;
  int v2; // eax
  unsigned long v3; // rax
  char v4 [24];
  char *v5; // rsi
  long v6; // r12
  long v7; // stack - 0x40
  
  v6 = (a0 & 0xffffffff) * 0x20;
  if (v6 + 0x20U <= dat_6210) { // branch-flip
    v2 = fseeko(dat_6298,v6,0);
    if (v2)
      __assert_fail("0 == err","faillog.c",0x1b3,"set_locktime_one"); // no-return
    v3 = fread(v4,0x20,1,dat_6298);
    if (v3 != 1) {
      v5 = "%s: Failed to get the entry for UID %lu\n";
      v1 = dat_61c0; // return-dupe
      __fprintf_chk(stderr,1,dcgettext(NULL,v5,5),v1,a0 & 0xffffffff);
      return 1;
    }
  }
  else {
    __explicit_bzero_chk(v4,0x20,0x20);
  }
  if (v7 == a1)
    return 0;
  v7 = a1;
  v2 = fseeko(dat_6298,v6,0);
  if ((!v2) && (v3 = fwrite(v4,0x20,1,dat_6298), v3 == 1)) {
    fflush(dat_6298);
    return 0;
  }
  v5 = "%s: Failed to set locktime for UID %lu\n";
  v1 = dat_61c0;
  __fprintf_chk(stderr,1,dcgettext(NULL,v5,5),v1,a0 & 0xffffffff);
  return 1;
}

// Function: sub_3320 @ 0x3320
unsigned long sub_3320(unsigned long a0,short a1)
{
  unsigned long v1;
  int v2; // eax
  unsigned long v3; // rax
  char v4 [2];
  char *v5; // rsi
  long v6; // r12
  short v7; // stack - 0x56
  
  v6 = (a0 & 0xffffffff) * 0x20;
  if (v6 + 0x20U <= dat_6210) { // branch-flip
    v2 = fseeko(dat_6298,v6,0);
    if (v2)
      __assert_fail("0 == err","faillog.c",0x13e,"setmax_one"); // no-return
    v3 = fread(v4,0x20,1,dat_6298);
    if (v3 != 1) {
      v5 = "%s: Failed to get the entry for UID %lu\n";
      v1 = dat_61c0; // return-dupe
      __fprintf_chk(stderr,1,dcgettext(NULL,v5,5),v1,a0 & 0xffffffff);
      return 1;
    }
  }
  else {
    __explicit_bzero_chk(v4,0x20,0x20);
  }
  if (v7 == a1)
    return 0;
  v7 = a1;
  v2 = fseeko(dat_6298,v6,0);
  if ((!v2) && (v3 = fwrite(v4,0x20,1,dat_6298), v3 == 1)) {
    fflush(dat_6298);
    return 0;
  }
  v5 = "%s: Failed to set max for UID %lu\n";
  v1 = dat_61c0;
  __fprintf_chk(stderr,1,dcgettext(NULL,v5,5),v1,a0 & 0xffffffff);
  return 1;
}

// Function: sub_34a0 @ 0x34a0
unsigned long sub_34a0(unsigned long a0)
{
  unsigned long v1;
  int v2; // eax
  unsigned long v3; // rax
  short v4 [20]; // stack - 0x58
  long v5; // rbp
  char *v6; // rsi
  
  v5 = (a0 & 0xffffffff) * 0x20;
  if (v5 + 0x20U <= dat_6210) { // branch-flip
    v2 = fseeko(dat_6298,v5,0);
    if (v2)
      __assert_fail("0 == err","faillog.c",0xcc,"reset_one"); // no-return
    v3 = fread(v4,0x20,1,dat_6298);
    if (v3 != 1) {
      v6 = "%s: Failed to get the entry for UID %lu\n";
      v1 = dat_61c0; // return-dupe
      __fprintf_chk(stderr,1,dcgettext(NULL,v6,5),v1,a0 & 0xffffffff);
      return 1;
    }
  }
  else {
    __explicit_bzero_chk(v4,0x20,0x20);
  }
  if (!v4[0])
    return 0;
  v4[0] = 0;
  v2 = fseeko(dat_6298,v5,0);
  if ((!v2) && (v3 = fwrite(v4,0x20,1,dat_6298), v3 == 1)) {
    fflush(dat_6298);
    return 0;
  }
  v6 = "%s: Failed to reset fail count for UID %lu\n";
  v1 = dat_61c0;
  __fprintf_chk(stderr,1,dcgettext(NULL,v6,5),v1,a0 & 0xffffffff);
  return 1;
}

// Function: sub_3630 @ 0x3630
void sub_3630(unsigned long *a0,char a1)
{
  unsigned int v1;
  char v10 [88];
  char v11 [12];
  char *v12; // rsi
  short v13; // stack - 0x96
  long v14; // stack - 0x80
  unsigned long v2;
  int v3; // eax
  void *v4; // rax
  unsigned long v5; // rax
  long v6;
  short v7; // stack - 0x98
  long v8; // stack - 0xa0
  long v9; // stack - 0x88
  
  if (a0) {
    if ((unsigned long)*(unsigned int *)&a0[2] * 0x20 + 0x20 <= dat_6210) { // branch-flip
      v3 = fseeko(dat_6298,(unsigned long)*(unsigned int *)&a0[2] * 0x20,0);
      if (v3)
        __assert_fail("0 == err","faillog.c",0x66,"print_one"); // no-return
      v5 = fread(&v7,0x20,1,dat_6298);
      v2 = dat_61c0;
      if (v5 != 1) {
        v1 = *(unsigned int *)&a0[2];
        __fprintf_chk(stderr,1,dcgettext(NULL,"%s: Failed to get the entry for UID %lu\n",5),v2,(unsigned long)v1);
        return;
      }
    }
    else {
      __explicit_bzero_chk(&v7,0x20,0x20);
    }
    if (((a1) || (v9)) && ((time(&v8), !dat_6273 || (v8 - v9 <= dat_6290)))) {
      if (!dat_61c8) {
        puts(dcgettext(NULL,"Login       Failures Maximum Latest                   On\n",5));
        dat_61c8 = '\x01';
      }
      v4 = localtime(&v9);
      if (v4) { // branch-flip
        strftime(v10,0x50,"%D %H:%M:%S %z",v4);
        __printf_chk(1,"%-9s   %5d    %5d   ",*a0,(unsigned long)(unsigned int)(int)v7,(unsigned long)(unsigned int)(int)v13);
        __printf_chk(1,"%s  %s",v10,v11);
        if (v14) {
          if ((v9 + v14 <= v8) || (!v7)) {
            v12 = " [%lds lock]";
            v6 = v14;
          }
          else {
            v6 = (v9 + v14) - v8;
            v12 = " [%lus left]";
          }
          __printf_chk(1,dcgettext(NULL,v12,5),v6);
        }
        putc(10,stdout);
      }
      else {
        __fprintf_chk(stderr,1,"Cannot read time from faillog.\n");
      }
    }
  }
}

// Function: sub_38d0 @ 0x38d0
char * sub_38d0(char *a0)
{
  char *v1;
  char *v2; // rax
  bool v3; // zf
  
  v2 = strrchr(a0,0x2f);
  v1 = &v2[1];
  v3 = v2 != NULL;
  v2 = a0;
  if (v3)
    v2 = v1;
  return v2;
}

// Function: sub_3900 @ 0x3900
unsigned long sub_3900(char *a0,unsigned long *a1,char *a2,unsigned long *a3,char *a4)
{
  char *v1;
  char v2;
  long *v3; // rax
  int *v4; // rax
  unsigned long v5; // rax
  char *v6; // stack - 0x48
  
  if (a0) {
    if (*a0 != '-') { // branch-flip
      v4 = __errno_location();
      *v4 = 0;
      v5 = strtoul(a0,&v6,10);
      if (*v4 != 0x22) {
        if (!*v6) {
          *a2 = 1;
          *a4 = 1;
          *a1 = v5;
          *a3 = v5;
          return 1;
        }
        if (*v6 == '-') {
          v1 = &v6[1];
          v2 = v6[1];
          if (!v2) {
            *a2 = 1;
            *a4 = 0;
            *a1 = v5;
            return 1;
          }
          v6 = v1;
          if (*(unsigned char *)(*(long *)__ctype_b_loc() + 1 + (long)v2 * 2) & 8) {
            *a2 = 1;
            *a1 = v5;
            *v4 = 0;
            v5 = strtoul(v1,&v6,10);
            if ((!*v6) && (*v4 != 0x22)) {
              *a4 = 1;
              *a3 = v5;
              return 1;
            }
          }
        }
      }
    }
    else {
      v3 = __ctype_b_loc();
      if (*(unsigned char *)(*v3 + 1 + (long)a0[1] * 2) & 8) {
        v4 = __errno_location();
        *v4 = 0;
        v5 = strtoul(&a0[1],&v6,10);
        if ((!*v6) && (*v4 != 0x22)) {
          *a2 = 0;
          *a4 = 1;
          *a3 = v5;
          return 1;
        }
      }
    }
  }
  return 0;
}

// Function: sub_3ad0 @ 0x3ad0
void sub_3ad0(char *a0,int a1,long a2) // return-dupe x2
{
  unsigned int v1; // eax
  unsigned int v2; // eax
  char *v3;
  char *v4;
  unsigned long v5; // rax
  void *v6; // rax
  int v7; // ebp
  char *v8; // rax
  
  if (a1 <= 0)
    return;
  v3 = NULL;
  v7 = 0;
  do {
    while( true ) {
      v4 = *(char **)(a2 + (long)v7 * 8);
      if (strcmp(v4,"--root")) break;
label_3b5b:
      if (v3) {
label_3c2a:
        v5 = sub_3e70();
        v3 = dcgettext(NULL,"%s: multiple --root options\n",5);
        __fprintf_chk((void *)sub_3eb0(),1,v3,v5);
        exit(3); // no-return
      }
      if (v7 + 1 == a1) {
        v5 = sub_3e70();
        v8 = dcgettext(NULL,"%s: option \'%s\' requires an argument\n",5);
        v6 = (void *)sub_3eb0();
        __fprintf_chk(v6,1,v8,v5,v4); // return-dupe
        exit(3); // no-return
      }
      v3 = *(char **)(a2 + 8 + (long)v7 * 8);
      v7 += 2;
      if (a1 <= v7) goto label_3b7f;
    }
    if (strncmp(v4,"--root=",7)) { // branch-flip
      if (!strcmp(v4,a0)) goto label_3b5b;
    }
    else {
      if (v3) goto label_3c2a;
      v3 = &v4[7];
    }
    v7 += 1;
  } while (v7 < a1);
label_3b7f:
  if (!v3)
    return;
  v1 = getgid();
  v2 = getgid();
  if (!setregid(v2,v1)) {
    v1 = getuid();
    v2 = getuid();
    if (!setreuid(v2,v1)) {
      if (*v3 == '/') {
        if (access(v3,0)) { // branch-flip
          v4 = strerror(*__errno_location());
          v5 = sub_3e70();
          v8 = "%s: cannot access chroot directory %s: %s\n";
        }
        else if (chdir(v3)) { // branch-flip
          v4 = strerror(*__errno_location());
          v5 = sub_3e70();
          v8 = "%s: cannot chdir to chroot directory %s: %s\n";
        }
        else {
          if (!chroot(v3))
            return;
          v4 = strerror(*__errno_location());
          v5 = sub_3e70();
          v8 = "%s: unable to chroot to directory %s: %s\n";
        }
        v8 = dcgettext(NULL,v8,5);
        __fprintf_chk((void *)sub_3eb0(),1,v8,v5,v3,v4);
        exit(3); // no-return
      }
      v5 = sub_3e70();
      v8 = dcgettext(NULL,"%s: invalid chroot path \'%s\', only absolute paths are supported.\n",5);
      v6 = (void *)sub_3eb0();
      v4 = v3;
      __fprintf_chk(v6,1,v8,v5,v4);
      exit(3);
    }
  }
  v3 = strerror(*__errno_location());
  v5 = sub_3e70();
  v4 = dcgettext(NULL,"%s: failed to drop privileges (%s)\n",5);
  __fprintf_chk((void *)sub_3eb0(),1,v4,v5,v3);
  exit(1); // no-return
}

// Function: sub_3de0 @ 0x3de0
unsigned long sub_3de0(char *a0,long *a1)
{
  int *v1; // rax
  long v2; // rax
  char *v3; // stack - 0x28
  unsigned long v4; // r8
  
  v1 = __errno_location();
  *v1 = 0;
  v2 = strtol(a0,&v3,0);
  v4 = 0;
  if (((*a0) && (!*v3)) && (*v1 != 0x22)) {
    *a1 = v2;
    v4 = 1;
  }
  return v4;
}

// Function: sub_3e60 @ 0x3e60
void sub_3e60(unsigned long a0)
{
  dat_6140 = a0;
}

// Function: sub_3e70 @ 0x3e70
unsigned long sub_3e70(void)
{
  return dat_6140;
}

// Function: sub_3e80 @ 0x3e80
void sub_3e80(long a0)
{
  if (a0) {
    dat_62a0 = a0;
    return;
  }
  dat_62a0 = stderr;
}

// Function: sub_3eb0 @ 0x3eb0
long sub_3eb0(void)
{
  if (dat_62a0)
    return dat_62a0;
  return stderr;
}

// Function: _DT_FINI @ 0x3ed4
void _DT_FINI(void)
{
  return;
}

