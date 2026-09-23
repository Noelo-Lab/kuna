// Function: _DT_INIT @ 0x2000
void _DT_INIT(void) // return-dupe
{
  if (!dat_12fd8)
    return;
  (*dat_12fd8)();
}

// Function: sub_2020 @ 0x2020
void sub_2020(void)
{
  (*dat_12d78)(); // jump-as-call
}

// Function: __cxa_finalize @ 0x24c0
void __cxa_finalize(void)
{
  (*dat_12fe8)(); // jump-as-call
}

// Function: getenv @ 0x24d0
char * getenv(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_12d80)(); // jump-as-call
  return v1;
}

// Function: raise @ 0x24e0
int raise(int a0)
{
  int v1; // eax
  
  v1 = (*dat_12d88)(); // jump-as-call
  return v1;
}

// Function: free @ 0x24f0
void free(void *a0)
{
  (*dat_12d90)(); // jump-as-call
}

// Function: abort @ 0x2500
void abort(void)
{
  (*dat_12d98)(); // jump-as-call
}

// Function: __errno_location @ 0x2510
int * __errno_location(void)
{
  int *v1; // rax
  
  v1 = (int *)(*dat_12da0)(); // jump-as-call
  return v1;
}

// Function: strncmp @ 0x2520
int strncmp(char *a0,char *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_12da8)(); // jump-as-call
  return v1;
}

// Function: _exit @ 0x2530
void _exit(int a0)
{
  (*dat_12db0)(); // jump-as-call
}

// Function: __fpending @ 0x2540
unsigned long __fpending(FILE *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_12db8)(); // jump-as-call
  return v1;
}

// Function: isatty @ 0x2550
int isatty(int a0)
{
  int v1; // eax
  
  v1 = (*dat_12dc0)(); // jump-as-call
  return v1;
}

// Function: reallocarray @ 0x2560
void * reallocarray(void *a0,unsigned long a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_12dc8)(); // jump-as-call
  return v1;
}

// Function: strtod @ 0x2570
void strtod(void)
{
  (*dat_12dd0)(); // jump-as-call
}

// Function: fcntl @ 0x2580
int fcntl(int a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_12dd8)(); // jump-as-call
  return v1;
}

// Function: clearerr_unlocked @ 0x2590
void clearerr_unlocked(FILE *a0)
{
  (*dat_12de0)(); // jump-as-call
}

// Function: textdomain @ 0x25a0
char * textdomain(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_12de8)(); // jump-as-call
  return v1;
}

// Function: strtod_l @ 0x25b0
void strtod_l(void)
{
  (*dat_12df0)(); // jump-as-call
}

// Function: fclose @ 0x25c0
int fclose(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_12df8)(); // jump-as-call
  return v1;
}

// Function: bindtextdomain @ 0x25d0
char * bindtextdomain(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_12e00)(); // jump-as-call
  return v1;
}

// Function: dcgettext @ 0x25e0
char * dcgettext(char *a0,char *a1,int a2)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_12e08)(); // jump-as-call
  return v1;
}

// Function: __ctype_get_mb_cur_max @ 0x25f0
unsigned long __ctype_get_mb_cur_max(void)
{
  unsigned long v1; // rax
  
  v1 = (*dat_12e10)(); // jump-as-call
  return v1;
}

// Function: strlen @ 0x2600
unsigned long strlen(char *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_12e18)(); // jump-as-call
  return v1;
}

// Function: __stack_chk_fail @ 0x2610
void __stack_chk_fail(void)
{
  (*dat_12e20)(); // jump-as-call
}

// Function: getopt_long @ 0x2620
int getopt_long(int a0,char **a1,char *a2,option *a3,int *a4)
{
  int v1; // eax
  
  v1 = (*dat_12e28)(); // jump-as-call
  return v1;
}

// Function: mbrtowc @ 0x2630
unsigned long mbrtowc(void *a0,char *a1,unsigned long a2,mbstate_t *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_12e30)(); // jump-as-call
  return v1;
}

// Function: strchr @ 0x2640
char * strchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_12e38)(); // jump-as-call
  return v1;
}

// Function: newlocale @ 0x2650
void newlocale(void)
{
  (*dat_12e40)(); // jump-as-call
}

// Function: nanosleep @ 0x2660
int nanosleep(timespec *a0,timespec *a1)
{
  int v1; // eax
  
  v1 = (*dat_12e48)(); // jump-as-call
  return v1;
}

// Function: __overflow @ 0x2670
int __overflow(FILE *a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_12e50)(); // jump-as-call
  return v1;
}

// Function: strrchr @ 0x2680
char * strrchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_12e58)(); // jump-as-call
  return v1;
}

// Function: lseek @ 0x2690
void lseek(void)
{
  (*dat_12e60)(); // jump-as-call
}

// Function: __assert_fail @ 0x26a0
void __assert_fail(char *a0,char *a1,unsigned int a2,char *a3)
{
  (*dat_12e68)(); // jump-as-call
}

// Function: fputs @ 0x26b0
int fputs(char *a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_12e70)(); // jump-as-call
  return v1;
}

// Function: memset @ 0x26c0
void * memset(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_12e78)(); // jump-as-call
  return v1;
}

// Function: close @ 0x26d0
int close(int a0)
{
  int v1; // eax
  
  v1 = (*dat_12e80)(); // jump-as-call
  return v1;
}

// Function: memchr @ 0x26e0
void * memchr(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_12e88)(); // jump-as-call
  return v1;
}

// Function: read @ 0x26f0
long read(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_12e90)(); // jump-as-call
  return v1;
}

// Function: lstat @ 0x2700
int lstat(char *a0,stat *a1)
{
  int v1; // eax
  
  v1 = (*dat_12e98)(); // jump-as-call
  return v1;
}

// Function: memcmp @ 0x2710
int memcmp(void *a0,void *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_12ea0)(); // jump-as-call
  return v1;
}

// Function: fputs_unlocked @ 0x2720
int fputs_unlocked(char *a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_12ea8)(); // jump-as-call
  return v1;
}

// Function: rawmemchr @ 0x2730
void * rawmemchr(void *a0,int a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_12eb0)(); // jump-as-call
  return v1;
}

// Function: calloc @ 0x2740
void * calloc(unsigned long a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_12eb8)(); // jump-as-call
  return v1;
}

// Function: strcmp @ 0x2750
int strcmp(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_12ec0)(); // jump-as-call
  return v1;
}

// Function: stat @ 0x2760
int stat(char *a0,stat *a1)
{
  int v1; // eax
  
  v1 = (*dat_12ec8)(); // jump-as-call
  return v1;
}

// Function: strtol @ 0x2770
long strtol(char *a0,char **a1,int a2)
{
  long v1; // rax
  
  v1 = (*dat_12ed0)(); // jump-as-call
  return v1;
}

// Function: memcpy @ 0x2780
void * memcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_12ed8)(); // jump-as-call
  return v1;
}

// Function: kill @ 0x2790
int kill(int a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_12ee0)(); // jump-as-call
  return v1;
}

// Function: inotify_init @ 0x27a0
void inotify_init(void)
{
  (*dat_12ee8)(); // jump-as-call
}

// Function: fileno @ 0x27b0
int fileno(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_12ef0)(); // jump-as-call
  return v1;
}

// Function: pause @ 0x27c0
int pause(void)
{
  int v1; // eax
  
  v1 = (*dat_12ef8)(); // jump-as-call
  return v1;
}

// Function: malloc @ 0x27d0
void * malloc(unsigned long a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_12f00)(); // jump-as-call
  return v1;
}

// Function: fflush @ 0x27e0
int fflush(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_12f08)(); // jump-as-call
  return v1;
}

// Function: inotify_add_watch @ 0x27f0
void inotify_add_watch(void)
{
  (*dat_12f10)(); // jump-as-call
}

// Function: nl_langinfo @ 0x2800
char * nl_langinfo(int a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_12f18)(); // jump-as-call
  return v1;
}

// Function: __freading @ 0x2810
int __freading(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_12f20)(); // jump-as-call
  return v1;
}

// Function: fwrite_unlocked @ 0x2820
unsigned long fwrite_unlocked(void *a0,unsigned long a1,unsigned long a2,FILE *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_12f28)(); // jump-as-call
  return v1;
}

// Function: realloc @ 0x2830
void * realloc(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_12f30)(); // jump-as-call
  return v1;
}

// Function: setlocale @ 0x2840
char * setlocale(int a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_12f38)(); // jump-as-call
  return v1;
}

// Function: __printf_chk @ 0x2850
int __printf_chk(int a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_12f40)(); // jump-as-call
  return v1;
}

// Function: poll @ 0x2860
void poll(void)
{
  (*dat_12f48)(); // jump-as-call
}

// Function: error @ 0x2870
void error(int a0,int a1,char *a2,...)
{
  (*dat_12f50)(); // jump-as-call
}

// Function: memrchr @ 0x2880
void * memrchr(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_12f58)(); // jump-as-call
  return v1;
}

// Function: open @ 0x2890
int open(char *a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_12f60)(); // jump-as-call
  return v1;
}

// Function: fseeko @ 0x28a0
int fseeko(FILE *a0,long a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_12f68)(); // jump-as-call
  return v1;
}

// Function: strtoumax @ 0x28b0
void strtoumax(void)
{
  (*dat_12f70)(); // jump-as-call
}

// Function: fstatfs @ 0x28c0
int fstatfs(int a0,statfs *a1)
{
  int v1; // eax
  
  v1 = (*dat_12f78)(); // jump-as-call
  return v1;
}

// Function: __cxa_atexit @ 0x28d0
void __cxa_atexit(void)
{
  (*dat_12f80)(); // jump-as-call
}

// Function: exit @ 0x28e0
void exit(int a0)
{
  (*dat_12f88)(); // jump-as-call
}

// Function: __fprintf_chk @ 0x28f0
int __fprintf_chk(FILE *a0,int a1,char *a2,...)
{
  int v1; // eax
  
  v1 = (*dat_12f90)(); // jump-as-call
  return v1;
}

// Function: fflush_unlocked @ 0x2900
int fflush_unlocked(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_12f98)(); // jump-as-call
  return v1;
}

// Function: mbsinit @ 0x2910
int mbsinit(mbstate_t *a0)
{
  int v1; // eax
  
  v1 = (*dat_12fa0)(); // jump-as-call
  return v1;
}

// Function: iswprint @ 0x2920
int iswprint(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_12fa8)(); // jump-as-call
  return v1;
}

// Function: inotify_rm_watch @ 0x2930
void inotify_rm_watch(void)
{
  (*dat_12fb0)(); // jump-as-call
}

// Function: fstat @ 0x2940
int fstat(int a0,stat *a1)
{
  int v1; // eax
  
  v1 = (*dat_12fb8)(); // jump-as-call
  return v1;
}

// Function: __ctype_b_loc @ 0x2950
void * __ctype_b_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_12fc0)(); // jump-as-call
  return v1;
}

// Function: main @ 0x2960
int main(int argc,char **argv,char **envp)
{
  unsigned char *v1;
  char v10;
  int v11;
  int v12;
  int v13;
  int v14; // eax
  int v15; // eax
  int v16; // eax
  unsigned int v17; // eax
  int v18; // eax
  int v19; // eax
  char **v2;
  int v20; // eax
  int v21; // eax
  int v22; // eax
  int v23; // eax
  int v24; // eax
  int v25; // eax
  int v26; // eax
  long *v27; // rax
  long v28; // rax
  void *v29;
  unsigned char v3;
  unsigned long v30;
  void *v31;
  double v32;
  double v33;
  unsigned char *v34;
  char *v35;
  int *v36;
  long v37;
  long *v38;
  unsigned int *v39; // rax
  char v4;
  unsigned long v40; // rax
  double v41;
  unsigned long v42;
  unsigned int v43; // edx
  unsigned int v44;
  char **v45;
  unsigned char *v46;
  char *v47;
  char v48;
  int v49;
  unsigned int v5;
  long v50;
  long *v51;
  double v52 [8]; // stack - 0x1c8
  stat v53; // stack - 0xd8
  double v54; // stack - 0x1d8
  stat v55; // stack - 0x168
  long *v56; // stack - 0x1d0
  void *v57; // rbp
  unsigned long v58; // r10
  unsigned char v59;
  long v6;
  char **v60;
  void *v61;
  bool v62;
  unsigned char v63;
  long v64;
  long *v65; // stack - 0x260
  void *v66; // stack - 0x258
  char *v67; // stack - 0x250
  void *v68; // stack - 0x248
  double v69; // stack - 0x240
  double v7;
  double v70; // stack - 0x238
  unsigned long v71; // stack - 0x230
  void *v72; // stack - 0x228
  long *v73; // stack - 0x220
  unsigned int v74; // stack - 0x210
  unsigned char v75; // stack - 0x209
  double v76; // stack - 0x208
  int v77; // stack - 0x184
  long v78; // stack - 0x40
  char v8;
  char v9;
  
  v78 = *(long *)(v64 + 0x28);
  v54 = 4.94065645841247e-323;
  sub_9150(*argv);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  sub_cf70(sub_79b0);
  dat_1310e = '\0';
  dat_13119 = 1;
  dat_13115 = '\0';
  dat_13116 = 0;
  dat_13118 = '\0';
  dat_13114 = '\n';
  if (argc != 2) { // branch-flip
    if (argc == 3) {
      v35 = argv[2];
      if ((*v35 == '-') && (v35[1])) goto label_2a20;
      goto label_2a30;
    }
    if (argc == 4) {
      v35 = argv[2];
label_2a20:
      if (!strcmp(v35,"--")) goto label_2a30;
    }
label_2a52:
    v28 = 0;
  }
  else {
label_2a30:
    v11 = sub_90c0();
    v35 = argv[1];
    v1 = (unsigned char *)&v35[1];
    if (*v35 == '+') {
      v59 = 1;
      if (0x2b8 < (unsigned int)(v11 - 0x30db0U)) goto label_38a5;
      goto label_2a52;
    }
    if ((*v35 != '-') || ((0x30db0 <= v11 && (!v35[(unsigned long)(v35[1] == 'c') + 1])))) goto label_2a52;
    v59 = 0;
label_38a5:
    v63 = v35[1];
    v46 = v1;
    while ((unsigned int)((int)(char)v63 - 0x30U) <= 9) {
      v34 = &v46[1];
      v46 = &v46[1];
      v63 = *v34;
    }
    if (v63 != 99) { // branch-flip
      if (v63 == 0x6c) {
        v63 = 1;
        v41 = 4.94065645841247e-323;
        goto label_41b9;
      }
      if (v63 == 0x62) {
        v63 = 0;
        v41 = 2.52961610670718e-320;
        goto label_41b9;
      }
      v63 = 1;
      v41 = 4.94065645841247e-323;
      v34 = v46;
    }
    else {
      v63 = 0;
      v41 = 4.94065645841247e-323;
label_41b9:
      v34 = &v46[1];
    }
    v3 = *v34;
    if (v3 == 0x66)
      v34 = &v34[1];
    if (*v34) goto label_2a52;
    if (v1 != v46) { // branch-flip
      if (sub_c520(v1,NULL,10,&v54,"b") & 0xfffffffd) {
        v35 = (char *)sub_b520(argv[1]);
        v47 = dcgettext(NULL,"invalid number",5);
        error(1,*__errno_location(),"%s: %s",v47,v35);
        return v22;
      }
    }
    else {
      v54 = v41;
    }
    v28 = 1;
    dat_13116 = v59;
    dat_13118 = v3 == 0x66;
    dat_13119 = v63;
  }
  v11 = 0;
  argv = &argv[v28];
  argc -= (int)v28;
  v76 = dat_e4a0;
label_2a90:
  v12 = getopt_long(argc,argv,"c:n:fFqs:vz0123456789",(option *)0x12900,NULL);
  if (v12 != -1) {
    if (0x85 < v12) {
label_42ba:
      sub_6f70(1); // no-return, return-dupe
    }
    if (v12 <= 0x2f) {
      if (v12 == -0x83) {
        sub_bb50(stdout,"tail","GNU coreutils",dat_13030,"Paul Rubin","David MacKenzie","Ian Lance Taylor","Jim Meyering",0,v58);
        exit(0); // no-return
      }
      if (v12 == -0x82)
        sub_6f70(0); // no-return
      sub_6f70(1);
    }
    switch(v12) {
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
        error(1,0,dcgettext(NULL,"option used in invalid context -- %c",5),(char)v12);
        return v15;
      default:
        goto label_42ba;
      case 0x46:
        dat_13118 = '\x01';
        dat_13020 = 1;
        dat_1311a = 1;
        break;
      case 99:
      case 0x6e:
        dat_13119 = v12 == 0x6e;
        if (*optarg != '+') { // branch-flip
          if (*optarg == '-')
            optarg = &optarg[1];
        }
        else {
          dat_13116 = 1;
        }
        if (v12 != 0x6e) // branch-flip
          v35 = dcgettext(NULL,"invalid number of bytes",5);
        else {
          v35 = dcgettext(NULL,"invalid number of lines",5);
        }
        v54 = (double)sub_c370(optarg,0,0xffffffffffffffff,"bkKmMGTPEZY0",v35,0);
        break;
      case 0x66:
      case 0x84:
        dat_13118 = '\x01';
        if (optarg) // branch-flip
          dat_13020 = *(int *)(sub_7710("--follow",optarg,0x12b00,0xe498,4,dat_13038,1) * 4 + 0xe498);
        else {
          dat_13020 = 2;
        }
        break;
      case 0x71:
label_2cf0:
        v11 = 2;
        break;
      case 0x73:
        v9 = sub_c450(optarg,0,v52,sub_78d0);
        if (v9) {
          v76 = v52[0];
          if (0.0 <= v52[0]) break;
        }
        argv = (char **)sub_b520(optarg);
        error(1,0,dcgettext(NULL,"invalid number of seconds: %s",5),argv);
        goto label_2cf0;
      case 0x76:
        v11 = 1;
        break;
      case 0x7a:
        dat_13114 = '\0';
        break;
      case 0x80:
        dat_1311a = 1;
        break;
      case 0x81:
        dat_13018 = sub_c370(optarg,0,0xffffffffffffffff,"",dcgettext(NULL,"invalid maximum number of unchanged stats between opens",5),0);
        break;
      case 0x82:
        dat_13110 = sub_c370(optarg,0,0x7fffffff,"",dcgettext(NULL,"invalid PID",5),0);
        break;
      case 0x83:
        dat_1310d = '\x01';
        break;
      case 0x85:
        goto label_2bd5;
      
    }
    goto label_2a90;
  }
  if (dat_1311a) { // branch-flip
    if (!dat_13118) {
      dat_1311a = 0;
      v35 = "warning: --retry ignored; --retry is useful only when following";
label_3152:
      error(0,0,dcgettext(NULL,v35,5));
      goto label_3167;
    }
    if (dat_13020 == 2) {
      v35 = "warning: --retry only effective for the initial open";
      goto label_3152;
    }
    if (dat_13110) goto label_2da1;
  }
  else {
label_3167:
    if (dat_13110) {
      if (dat_13118) { // branch-flip
label_2da1:
        if ((kill(dat_13110,0)) && (*__errno_location() == 0x26)) {
          error(0,0,dcgettext(NULL,"warning: --pid=PID is not supported on this system",5));
          dat_13110 = 0;
        }
      }
      else {
        error(0,0,dcgettext(NULL,"warning: PID ignored; --pid=PID is useful only when following",5));
      }
    }
  }
  if ((dat_13116) && (v54 != 0.0))
    v54 = (double)((long)v54 - 1);
  v45 = (char **)0x13028;
  v71 = 1;
  if (optind < argc) {
    v45 = &argv[optind];
    v71 = (unsigned long)(argc - optind);
  }
  v62 = 0;
  v2 = &v45[v71];
  v60 = v45;
  do {
    v35 = *v60;
    if (!strcmp(v35,"-"))
      v62 = 1;
    v60 = &v60[1];
  } while (v60 != v2);
  if (v62) {
    if (dat_13020 == 1) {
      v35 = (char *)sub_ae30(4,"-");
      error(1,0,dcgettext(NULL,"cannot follow %s by name",5),v35);
      return v21;
    }
    if (dat_13118) {
      if (((((dat_13110) || (v71 != 1)) || (dat_13020 != 2)) || ((fstat(0,&v53) || ((v53._24_4_ & 0xf000) == 0x8000)))) && (isatty(0)))
        error(0,0,dcgettext(NULL,"warning: following standard input indefinitely is ineffective",5));
      goto label_2e85;
    }
    if (v54 == 0.0) goto label_3d8d;
  }
  else {
label_2e85:
    if ((v54 == 0.0) && (!dat_13118)) {
label_3d8d:
      v44 = 0;
      if (!dat_13116) goto label_3381;
    }
  }
  v27 = (long *)sub_be60(v71,0x60);
  v38 = v27;
  do {
    v35 = *v45;
    v45 = &v45[1];
    v51 = &v38[0xc];
    *v38 = (long)v35;
    v38 = v51;
  } while (v45 != v2);
  if ((v11 == 1) || ((!v11 && (v71 != 1))))
    dat_13115 = '\x01';
  v75 = 1;
  v65 = v27;
  v28 = v71 * 0x60;
  v38 = &v27[v71 * 0xc];
  do {
    v41 = v54;
    v69 = v54;
    v67 = (char *)*v65;
    v11 = strcmp(v67,"-");
    if (v11) { // branch-flip
      v12 = sub_7dd0(v67,0);
      v59 = dat_1311a;
      if (dat_1311a) { // branch-flip
        v67 = (char *)*v65;
        if (v12 != -1) goto label_2f60;
        *(char *)((long)v65 + 0x36) = 0;
      }
      else {
        v67 = (char *)*v65;
        *(char *)((long)v65 + 0x36) = 1;
        if (v12 != -1) goto label_2f69;
      }
      v36 = __errno_location();
      if (dat_13118) {
        v11 = *v36;
        *(unsigned int *)&v65[7] = 0xffffffff;
        *(int *)((long)v65 + 0x3c) = v11;
        *(unsigned char *)((long)v65 + 0x34) = v59 ^ 1;
        v65[5] = 0;
        v65[4] = 0;
      }
      if (!strcmp(v67,"-"))
        v67 = dcgettext(NULL,"standard input",5);
      v35 = (char *)sub_ae30(4,v67);
      v47 = dcgettext(NULL,"cannot open %s for reading",5);
      error(0,*v36,v47,v35);
      v75 = 0;
    }
    else {
      dat_1310e = '\x01';
      v12 = 0;
label_2f60:
      *(char *)((long)v65 + 0x36) = 1;
label_2f69:
      if (dat_13115) {
        if (!strcmp(v67,"-"))
          v67 = dcgettext(NULL,"standard input",5);
        v35 = "\n";
        if (dat_13010)
          v35 = "";
        __printf_chk(1,"%s==> %s <==\n",v35,v67);
        dat_13010 = '\0';
        v67 = (char *)*v65;
      }
      if (!strcmp(v67,"-"))
        v67 = dcgettext(NULL,"standard input",5);
      v59 = dat_13119;
      v52[0] = 0.0;
      if (dat_13119) { // branch-flip
        if (fstat(v12,&v53)) { // branch-flip
label_3590:
          v40 = sub_ae30(4,v67);
          v35 = dcgettext(NULL,"cannot fstat %s",5);
          error(0,*__errno_location(),v35,v40);
          v74 = 0;
        }
        else {
          v74 = (unsigned int)dat_13116;
          if (dat_13116) {
            v44 = sub_5ce0(v67,v12,v41,v52);
            if (v44) { // branch-flip
label_3df1:
              v74 = v44 >> 0x1f;
            }
            else {
              v52[0] = (double)((long)v52[0] + sub_6020(0,v67,v12,0xffffffffffffffff));
            }
            goto label_35da;
          }
          if (((!dat_1310d) && ((v53._24_4_ & 0xf000) == 0x8000)) && (v30 = lseek(v12,0,1), v30 != 0xffffffffffffffff)) {
            v32 = (double)lseek(v12,0,2);
            if ((long)v30 < (long)v32) {
              v52[0] = v32;
              v74 = (unsigned int)v59;
              if (v32 != 0.0)
                v74 = (unsigned int)(unsigned char)sub_6190(v67,v12,v41,v30,v32,v52);
              goto label_35da;
            }
            if ((long)lseek(v12,v30,0) <= -1)
              sub_5f40(v30,0,v67); // no-return
          }
          v68 = (void *)sub_bd20(0x2018);
          *(unsigned long *)((long)v68 + 0x2008) = 0;
          *(unsigned long *)((long)v68 + 0x2000) = 0;
          *(unsigned long *)((long)v68 + 0x2010) = 0;
          v29 = (void *)sub_bd20(0x2018);
          v72 = v68;
          v70 = 0.0;
          v31 = v29;
          while( true ) {
            v61 = v31;
            v30 = sub_b540(v12,v29,0x2000);
            if (v30 - 1 > 0xfffffffffffffffd) break;
            *(unsigned long *)((long)v61 + 0x2000) = v30;
            v49 = (int)dat_13114;
            v52[0] = (double)((long)v52[0] + v30);
            *(unsigned long *)((long)v61 + 0x2008) = 0;
            *(unsigned long *)((long)v61 + 0x2010) = 0;
            v31 = v29;
            while (v31 = memchr(v31,v49,(long)v29 + (v30 - (long)v31)), v31) {
              *(long *)((long)v61 + 0x2008) = *(long *)((long)v61 + 0x2008) + 1;
              v31 = (void *)((long)v31 + 1);
            }
            v70 = (double)((long)v70 + *(long *)((long)v61 + 0x2008));
            if (0x2000 <= v30 + *(long *)((long)v68 + 0x2000)) { // branch-flip
              *(void **)((long)v68 + 0x2010) = v61;
              v32 = (double)((long)v70 - *(long *)((long)v72 + 0x2008));
              v68 = v61;
              if ((unsigned long)v32 <= (unsigned long)v41) { // branch-flip
                v29 = (void *)sub_bd20(0x2018);
                v31 = v29;
              }
              else {
                v29 = v72;
                v31 = v72;
                v70 = v32;
                v72 = *(void **)((long)v72 + 0x2010);
              }
            }
            else {
              memcpy((void *)(*(long *)((long)v68 + 0x2000) + (long)v68),v29,v30);
              *(long *)((long)v68 + 0x2000) = *(long *)((long)v68 + 0x2000) + *(long *)((long)v61 + 0x2000);
              *(long *)((long)v68 + 0x2008) = *(long *)((long)v68 + 0x2008) + *(long *)((long)v61 + 0x2008);
              v31 = v61;
            }
          }
          free(v61);
          if (v30 != 0xffffffffffffffff) { // branch-flip
            v50 = *(long *)((long)v68 + 0x2000);
            v74 = (unsigned int)(!v50 || v41 == 0.0);
            if (!v50 || v41 == 0.0) goto label_34b9;
            v49 = (int)dat_13114;
            if (*(char *)((long)v68 + v50 + -1) != dat_13114) {
              *(long *)((long)v68 + 0x2008) = *(long *)((long)v68 + 0x2008) + 1;
              v70 = (double)((long)v70 + 1);
            }
            v32 = (double)((long)v70 - *(long *)((long)v72 + 0x2008));
            v29 = v72;
            if ((unsigned long)v41 < (unsigned long)v32) {
              do {
                v70 = v32;
                v29 = *(void **)((long)v29 + 0x2010);
                v32 = (double)((long)v70 - *(long *)((long)v29 + 0x2008));
              } while ((unsigned long)v41 < (unsigned long)v32);
            }
            v50 = *(long *)((long)v29 + 0x2000);
            v31 = v29;
            if ((unsigned long)v41 < (unsigned long)v70) {
              v37 = (long)v70 - (long)v41;
              do {
                v31 = (void *)((long)rawmemchr(v31,v49) + 1);
                v37 -= 1;
              } while (v37);
            }
            v30 = (long)v29 + (v50 - (long)v31);
            if (v30)
              sub_5c60(v31,v30);
            for (v50 = *(long *)((long)v29 + 0x2010); v50; v50 = *(long *)(v50 + 0x2010)) {
              if (*(unsigned long *)(v50 + 0x2000))
                sub_5c60(v50,*(unsigned long *)(v50 + 0x2000));
            }
            v74 = (unsigned int)v59;
          }
          else {
            v40 = sub_ae30(4,v67);
            v35 = dcgettext(NULL,"error reading %s",5);
            error(0,*__errno_location(),v35,v40);
label_34b9:
            if (!v72) goto label_35da;
          }
          do {
            v29 = *(void **)((long)v72 + 0x2010);
            free(v72);
            v72 = v29;
          } while (v29);
        }
      }
      else {
        if (fstat(v12,&v53)) goto label_3590;
        v74 = (unsigned int)dat_13116;
        if (dat_13116) { // branch-flip
          if ((dat_1310d) || ((long)v41 <= -1)) {
label_3a3e:
            v44 = sub_5e30(v67,v12,v41,v52);
            v69 = -NAN;
            if (v44) goto label_3df1;
          }
          else {
            if ((v53._24_4_ & 0xf000) != 0x8000) { // branch-flip
              if (lseek(v12,v41,1) == -1) goto label_3a3e;
            }
            else if ((long)lseek(v12,v41,1) <= -1)
              sub_5f40(v41,1,v67); // no-return
            v69 = -NAN;
            v52[0] = v41;
          }
label_3a6a:
          v50 = sub_6020(0,v67,v12,v69);
          v74 = 1;
          v52[0] = (double)((long)v52[0] + v50);
        }
        else {
          if ((!dat_1310d) && (0 <= (long)v41)) {
            if ((v53._24_4_ & 0xd000) != 0x8000) { // branch-flip
              v32 = (double)lseek(v12,-(long)v41,2);
              if (v32 == -NAN) goto label_3678;
              v50 = (long)v41 + (long)v32;
            }
            else {
              v32 = -NAN;
              v50 = v53._48_8_;
            }
            v37 = v53._56_8_;
            if (0x2000000000000000 <= (unsigned long)(v53._56_8_ - 1U))
              v37 = 0x200;
            if (v37 < v50) {
              if ((v32 == -NAN) && (v32 = (double)lseek(v12,0,1), (long)v32 <= -1))
                sub_5f40(0,1,v67); // no-return
              if (((long)v32 < v50) && ((unsigned long)v41 < (unsigned long)(v50 - (long)v32))) {
                v32 = (double)(v50 - (long)v41);
                if ((long)lseek(v12,v32,0) <= -1)
                  sub_5f40(v32,0,v67); // no-return
              }
              v52[0] = v32;
              goto label_3a6a;
            }
          }
label_3678:
          v29 = (void *)sub_bd20(0x2010);
          *(unsigned long *)((long)v29 + 0x2000) = 0;
          *(unsigned long *)((long)v29 + 0x2008) = 0;
          v31 = (void *)sub_bd20(0x2010);
          v61 = v29;
          v32 = 0.0;
          while( true ) {
            v57 = v31;
            v31 = v29;
            v30 = sub_b540(v12,v57,0x2000);
            if (v30 - 1 > 0xfffffffffffffffd) break;
            v52[0] = (double)((long)v52[0] + v30);
            v32 = (double)((long)v32 + v30);
            *(unsigned long *)((long)v57 + 0x2000) = v30;
            v50 = *(long *)((long)v61 + 0x2000);
            *(unsigned long *)((long)v57 + 0x2008) = 0;
            v29 = v31; // branch-flip
            if (0x2000 <= v30 + v50) {
              *(void **)((long)v61 + 0x2008) = v57;
              v33 = (double)((long)v32 - *(long *)((long)v31 + 0x2000));
              v61 = v57;
              if ((unsigned long)v33 <= (unsigned long)v41) // branch-flip
                v31 = (void *)sub_bd20(0x2010);
              else {
                v29 = *(void **)((long)v31 + 0x2008);
                v32 = v33;
              }
            }
            else {
              memcpy((void *)((long)v61 + v50),v57,v30);
              *(long *)((long)v61 + 0x2000) = *(long *)((long)v61 + 0x2000) + *(long *)((long)v57 + 0x2000);
              v31 = v57;
            }
          }
          free(v57);
          if (v30 != 0xffffffffffffffff) { // branch-flip
            v50 = *(long *)((long)v31 + 0x2000);
            v33 = (double)((long)v32 - v50);
            v29 = v31;
            while (v7 = v33, (unsigned long)v41 < (unsigned long)v7) {
              v29 = *(void **)((long)v29 + 0x2008);
              v50 = *(long *)((long)v29 + 0x2000);
              v33 = (double)((long)v7 - v50);
              v32 = v7;
            }
            v37 = 0;
            if ((unsigned long)v41 < (unsigned long)v32)
              v37 = (long)v32 - (long)v41;
            if (v50 - v37)
              sub_5c60((void *)((long)v29 + v37),v50 - v37);
            while (v29 = *(void **)((long)v29 + 0x2008), v29) {
              if (*(unsigned long *)((long)v29 + 0x2000))
                sub_5c60(v29,*(unsigned long *)((long)v29 + 0x2000));
            }
            v74 = 1;
          }
          else {
            v40 = sub_ae30(4,v67);
            v35 = dcgettext(NULL,"error reading %s",5);
            error(0,*__errno_location(),v35,v40);
            if (!v31) goto label_35da;
          }
          do {
            v29 = *(void **)((long)v31 + 0x2008);
            free(v31);
            v31 = v29;
          } while (v29);
        }
      }
label_35da:
      if (dat_13118) { // branch-flip
        *(unsigned int *)((long)v65 + 0x3c) = v74 - 1;
        if (0 <= fstat(v12,&v53)) { // branch-flip
          if (((v53._24_4_ & 0xf000) - 0x1000 & 0xffffe000) && ((v53._24_4_ & 0xb000) != 0x8000)) {
            v35 = "";
            *(unsigned int *)((long)v65 + 0x3c) = 0xffffffff;
            *(char *)((long)v65 + 0x36) = 0;
            *(unsigned char *)((long)v65 + 0x34) = dat_1311a ^ 1;
            if (dat_1311a ^ 1)
              v35 = dcgettext(NULL,"; giving up on this name",5);
            v47 = (char *)*v65;
            if (!strcmp(v47,"-"))
              v47 = dcgettext(NULL,"standard input",5);
            v47 = (char *)sub_b1b0(0,3,v47);
            error(0,0,dcgettext(NULL,"%s: cannot follow end of this type of file%s",5),v47,v35);
          }
          else if ((unsigned char)v74) {
            *(int *)&v65[7] = v12;
            v35 = (char *)*v65;
            *(unsigned int *)&v65[6] = v53._24_4_;
            v65[1] = (long)v52[0];
            *(unsigned int *)&v65[8] = -(unsigned int)(v11 == 0) | 1;
            v65[2] = v53._88_8_;
            v65[0xb] = 0;
            v65[3] = v53._96_8_;
            *(char *)((long)v65 + 0x34) = 0;
            v65[4] = CONCAT26(v53._6_2_,CONCAT24(v53._4_2_,v53._0_4_));
            v65[5] = v53._8_8_;
            if (!strcmp(v35,"-"))
              v35 = dcgettext(NULL,"standard input",5);
            *(char *)((long)v65 + 0x35) = sub_5460(v12,v35);
            goto label_3340;
          }
        }
        else {
          v36 = __errno_location();
          v35 = (char *)*v65;
          *(int *)((long)v65 + 0x3c) = *v36;
          if (!strcmp(v35,"-"))
            v35 = dcgettext(NULL,"standard input",5);
          v35 = (char *)sub_ae30(4,v35);
          v47 = dcgettext(NULL,"error reading %s",5);
          error(0,*v36,v47,v35);
        }
        v35 = (char *)*v65;
        *(unsigned char *)((long)v65 + 0x34) = dat_1311a ^ 1;
        if (!strcmp(v35,"-"))
          v35 = dcgettext(NULL,"standard input",5);
        sub_5bd0(v12,v35);
        v75 = 0;
        *(unsigned int *)&v65[7] = 0xffffffff;
      }
      else if ((v11) && (close(v12))) {
        v35 = (char *)*v65;
        if (!strcmp(v35,"-"))
          v35 = dcgettext(NULL,"standard input",5);
        v40 = sub_ae30(4,v35);
        v35 = dcgettext(NULL,"error reading %s",5);
        error(0,*__errno_location(),v35,v40);
        v75 = 0;
      }
      else {
        v75 &= (unsigned char)v74;
      }
    }
label_3340:
    v9 = dat_13118;
    v65 = &v65[0xc];
  } while (v38 != v65);
  if (dat_13118) {
    v50 = 0;
    v51 = v27;
    do {
      v35 = (char *)*v51;
      if ((((strcmp(v35,"-")) || (*(char *)((long)v51 + 0x34))) || ((int)v51[7] <= -1)) || ((*(unsigned int *)&v51[6] & 0xf000) != 0x1000))
        v50 += 1;
      else {
        *(unsigned int *)&v51[7] = 0xffffffff;
        *(char *)((long)v51 + 0x34) = 1;
      }
      v51 = &v51[0xc];
    } while (v38 != v51);
    if (v50) {
      if (fstat(1,&v55) < 0) {
        v35 = dcgettext(NULL,"standard output",5);
        error(1,*__errno_location(),v35);
        return v18;
      }
      dat_13117 = (v55._24_4_ & 0xf000) == 0x1000;
      if (!dat_1310c) {
        v51 = v27;
        do {
          if ((!*(char *)((long)v51 + 0x34)) && (v35 = (char *)*v51, !strcmp(v35,"-"))) goto label_3b98;
          v51 = &v51[0xc];
        } while (v38 != v51);
        v50 = 0;
        do {
          if ((0 <= *(int *)((long)v27 + v50 + 0x38)) && (*(char *)((long)v27 + v50 + 0x35))) goto label_3b98;
          v50 += 0x60;
        } while (v28 != v50);
        v50 = 0;
        do {
          if ((0 <= *(int *)((long)v27 + v50 + 0x38)) && (!*(char *)((long)v27 + v50 + 0x35))) {
            v48 = '\0';
            v51 = v27;
            goto label_484a;
          }
          v50 += 0x60;
        } while (v28 != v50);
      }
label_3b98:
      dat_1310c = '\x01';
      if (dat_13110) // branch-flip
        v44 = 0;
      else {
        v44 = 0;
        if (((dat_13020 == 2) && (v71 == 1)) && ((int)v27[7] != -1))
          v44 = (unsigned int)((*(unsigned int *)&v27[6] & 0xf000) != 0x8000);
      }
      v67 = (unsigned long)(v71 - 1);
      v48 = '\0';
label_3bf5:
      v62 = 0;
      v30 = 0;
      v38 = v27;
      do {
        while( true ) {
          if (*(char *)((long)v38 + 0x34)) goto label_3ca6;
          v11 = (int)v38[7];
          if (v11 <= -1) break;
          v35 = (char *)*v38;
          if (!strcmp(v35,"-"))
            v35 = dcgettext(NULL,"standard input",5);
          v5 = *(unsigned int *)&v38[6];
          v13 = *(unsigned int *)&v38[8];
          if (v44 != *(unsigned int *)&v38[8]) {
            v13 = sub_7b60(v11,3);
            if (v44) { // branch-flip
              if (0 <= v13) {
label_3c76:
                *(unsigned int *)&v38[8] = v44;
                v13 = v44;
                goto label_3c7c;
              }
            }
            else if ((0 <= v13) && ((v43 = v13 | 0x800, v13 == v43 || (sub_7b60(v11,4) != -1)))) goto label_3c76;
            v36 = __errno_location();
            if (((*(unsigned int *)&v38[6] & 0xf000) != 0x8000) || (*v36 != 1)) {
              v35 = (char *)sub_b1b0(0,3,v35);
              v47 = dcgettext(NULL,"%s: cannot change nonblocking mode",5);
              error(1,*v36,v47,v35);
              return v19;
            }
            v13 = *(unsigned int *)&v38[8];
          }
label_3c7c:
          v42 = 0xfffffffffffffffe;
          if (v13) { // branch-flip
label_3c8b:
            v50 = sub_6020(0,v35,v11,v42);
            v38[1] = v38[1] + v50;
            v62 = (bool)(v62 | v50 != 0);
          }
          else if (fstat(v11,&v53)) { // branch-flip
            *(unsigned int *)&v38[7] = 0xffffffff;
            v36 = __errno_location();
            *(int *)((long)v38 + 0x3c) = *v36;
            v35 = (char *)sub_b1b0(0,3,v35);
            error(0,*v36,"%s",v35);
            close(v11);
          }
          else {
            if ((*(unsigned int *)&v38[6] != v53._24_4_) || ((((v53._24_4_ & 0xf000) == 0x8000 && (v38[1] != v53._48_8_)) || (((unsigned int)(v53._96_8_ < v38[3]) - (unsigned int)(v38[3] < v53._96_8_)) + ((unsigned int)(v53._88_8_ < v38[2]) - (unsigned int)(v38[2] < v53._88_8_)) * 2)))) {
              *(unsigned int *)&v38[6] = v53._24_4_;
              v38[0xb] = 0;
              v38[2] = v53._88_8_;
              v38[3] = v53._96_8_;
              if (((v5 & 0xf000) == 0x8000) && (v53._48_8_ < v38[1])) {
                v47 = (char *)sub_b1b0(0,3,v35);
                error(0,0,dcgettext(NULL,"%s: file truncated",5),v47);
                if ((long)lseek(v11,0,0) < 0)
                  sub_5f40(0,0,v35); // no-return
                v38[1] = 0;
              }
              if (((char *)v30 != v67) && (dat_13115)) {
                v47 = "\n";
                if (dat_13010)
                  v47 = "";
                __printf_chk(1,"%s==> %s <==\n",v47,v35);
                dat_13010 = '\0';
              }
              v67 = (char *)v30;
              if ((int)v38[8]) // branch-flip
                v42 = 0xfffffffffffffffe;
              else {
                v42 = 0xffffffffffffffff;
                if (((v5 & 0xf000) == 0x8000) && (*(char *)((long)v38 + 0x35)))
                  v42 = v53._48_8_ - v38[1];
              }
              goto label_3c8b;
            }
            v42 = v38[0xb];
            v38[0xb] = v42 + 1;
            if ((dat_13018 <= v42) && (dat_13020 == 1)) {
              sub_6660(v38,(int)v38[8] != 0);
              v38[0xb] = 0;
            }
          }
label_3ca6:
          v30 += 1;
          v38 = &v38[0xc];
          if (v71 == v30) goto label_3ce2;
        }
        v30 += 1;
        v51 = &v38[0xc];
        sub_6660(v38,v44);
        v38 = v51;
      } while (v71 != v30);
label_3ce2:
      if ((!dat_1311a) || (dat_13020 != 1)) {
        v35 = (char *)((long)v27 + 0x34);
        while ((*(int *)&v35[4] <= -1 && ((*v35 == '\x01' || (!dat_1311a))))) {
          v35 = &v35[0x60];
          if (v35 == (char *)((long)v27 + v28 + 0x34)) {
            error(0,0,dcgettext(NULL,"no files remaining",5));
            goto label_3369;
          }
        }
      }
      if ((!v62) || (v44)) {
        if (fflush_unlocked(stdout)) {
          v35 = dcgettext(NULL,"write error",5);
          error(1,*__errno_location(),v35);
          return v20;
        }
        if (dat_13117) {
          v52[0] = 4.94065645841247e-324;
          if (0 <= (int)poll(v52,1,0)) goto label_45f9;
        }
      }
      else {
        if (!dat_13117) goto label_3bf5;
        v52[0] = 4.94065645841247e-324;
        if ((int)poll(v52,1,0) < 0) goto label_3bf5;
label_45f9:
        if ((unsigned long)v52[0] & 0x18000000000000) {
          raise(0xd); // return-dupe
          exit(1); // return-dupe, no-return
        }
      }
      if (!v62) {
        if (v48) goto label_3369;
        if (((dat_13110) && (kill(dat_13110,0))) && (*__errno_location() != 1))
          v48 = v9;
        else if (sub_c3a0(v76)) {
          v35 = dcgettext(NULL,"cannot read realtime clock",5);
          error(1,*__errno_location(),v35);
          return v16;
        }
      }
      goto label_3bf5;
    }
  }
label_3369:
  if ((dat_1310e) && (close(0) <= -1)) {
    error(1,*__errno_location(),"-");
    return v14;
  }
  v44 = (unsigned int)(v75 ^ 1);
label_3381:
  if (v78 == *(long *)(v64 + 0x28))
    return v44;
  __stack_chk_fail(); // no-return
label_2bd5:
  dat_1310c = '\x01';
  goto label_2a90;
  while (v51 = &v51[0xc], v38 != v51) {
label_484a:
    v35 = (char *)*v51;
    if ((!lstat(v35,&v53)) && ((v53._24_4_ & 0xf000) == 0xa000)) goto label_3b98;
  }
  v50 = 0;
  do {
    if (((0 <= *(int *)((long)v27 + v50 + 0x38)) && (v44 = *(unsigned int *)((long)v27 + v50 + 0x30) & 0xf000, v44 != 0x8000)) && (v44 != 0x1000)) goto label_3b98;
    v50 += 0x60;
  } while (v28 != v50);
  if ((v75) || (dat_13020 != 2)) {
    v11 = inotify_init();
    if (0 <= v11) {
      if (fflush_unlocked(stdout)) {
        v35 = dcgettext(NULL,"write error",5);
        error(1,*__errno_location(),v35);
        return v26;
      }
      v50 = sub_87c0(v71,0,sub_5430,sub_5450,0);
      if (!v50)
        sub_c220(); // no-return, return-dupe
      v17 = 0xc06;
      if (dat_13020 != 1)
        v17 = 2;
      v10 = '\0';
      v8 = '\0';
      v66 = NULL;
      v51 = v27;
      do {
        v12 = dat_13020;
        if (!*(char *)((long)v51 + 0x34)) {
          v35 = (char *)*v51;
          v29 = (unsigned long)strlen(v35);
          *(unsigned int *)((long)v51 + 0x44) = 0xffffffff;
          if (v29 <= v66)
            v29 = v66;
          if (v12 == 1) {
            v37 = sub_7a60(v35);
            v4 = v35[v37];
            v51[10] = sub_7840(v35) - (long)v35;
            v35[v37] = '\0';
            v35 = ".";
            if (v37)
              v35 = (char *)*v51;
            *(unsigned int *)&v51[9] = inotify_add_watch(v11,v35,0x784);
            *(char *)(*v51 + v37) = v4;
            if ((int)v51[9] < 0) {
              v36 = __errno_location();
              if (*v36 != 0x1c) { // branch-flip
                v35 = (char *)sub_ae30(4,(char *)*v51);
                v47 = dcgettext(NULL,"cannot watch parent directory of %s",5);
                error(0,*v36,v47,v35);
              }
              else {
label_4c07:
                v35 = "inotify resources exhausted";
label_4c13:
                error(0,0,dcgettext(NULL,v35,5));
              }
              goto label_4c28;
            }
          }
          v12 = inotify_add_watch(v11,*v51,v17);
          *(int *)((long)v51 + 0x44) = v12;
          v66 = v29;
          if (0 <= v12) { // branch-flip
            if (!sub_8e20(v50,v51)) {
              sub_c220();
            }
            v8 = v9;
          }
          else {
            if ((int)v51[7] != -1)
              v10 = v9;
            v39 = (unsigned int *)__errno_location();
            if ((*v39 & 0xffffffef) == 0xc) goto label_4c07;
            if (*v39 != *(unsigned int *)((long)v51 + 0x3c)) {
              v35 = (char *)sub_ae30(4,(char *)*v51);
              v47 = dcgettext(NULL,"cannot watch %s",5);
              error(0,*v39,v47,v35);
            }
          }
        }
        v51 = &v51[0xc];
      } while (v38 != v51);
      if (dat_13020 != 2) {
label_4c85:
        v56 = &v27[v71 * 0xc + -0xc];
        v51 = v27;
        do {
          if (!*(char *)((long)v51 + 0x34)) {
            if (dat_13020 != 1) { // branch-flip
              if ((((int)v51[7] != -1) && (v35 = (char *)*v51, !stat(v35,&v53))) && ((v51[4] != CONCAT26(v53._6_2_,CONCAT24(v53._4_2_,v53._0_4_)) || (v51[5] != v53._8_8_)))) {
                v35 = (char *)*v51;
                v40 = sub_ae30(4,(char *)sub_5fe0(v35));
                v35 = dcgettext(NULL,"%s was replaced",5);
                error(0,*__errno_location(),v35,v40);
                goto label_4c28;
              }
            }
            else {
              sub_6660(v51,0);
            }
            sub_6430(v51,&v56);
          }
          v51 = &v51[0xc];
        } while (v38 != v51);
        v30 = 0;
        v69 = (unsigned long)((long)v66 + 0x11);
        v66 = (void *)sub_bd20(v69);
        v70._0_4_ = 3;
        v67 = NULL;
label_4d4b:
        do {
          if (((dat_13020 == 1) && (!dat_1311a)) && (!sub_82f0(v50))) {
            error(1,0,dcgettext(NULL,"no files remaining",5));
            return v23;
          }
          if (v67 > v30) {
label_4d63:
            v36 = (int *)((long)v66 + v30);
            v44 = v36[1];
            v5 = v36[3];
            v12 = *v36;
            v30 = v30 + 0x10 + (unsigned long)v5;
            if (v44 & 0x400) { // branch-flip
              if (!v5) {
                v42 = 0;
                do {
                  if (v12 == (int)v27[v42 * 0xc + 9]) {
                    v35 = "directory containing watched file was removed";
                    goto label_4c13;
                  }
                  v42 += 1;
                } while (v71 != v42);
label_4e77:
                v77 = v12;
                v38 = (long *)sub_8530(v50,v52);
                if (v38) {
label_4e47:
                  if (v36[1] & 0xe04U) { // branch-flip
                    if (v36[1] & 0x400U) {
                      inotify_rm_watch(v11,*(unsigned int *)((long)v38 + 0x44));
                      sub_8e80(v50,v38);
                    }
                    sub_6660(v38,0);
                  }
                  else {
                    sub_6430(v38,&v56);
                  }
                }
                goto label_4d4b;
              }
            }
            else if (!v5) goto label_4e77;
            v42 = 0;
            v38 = v27;
            do {
              if ((v12 == (int)v38[9]) && (v37 = v38[10], v6 = *v38, !strcmp((char *)&v36[4],(char *)(v37 + v6)))) {
                if (v71 == v42) break;
                v38 = &v27[v42 * 0xc];
                if (!(v44 & 0x200)) {
                  v12 = inotify_add_watch(v11,*v38,v17);
                  if (v12 <= -1) {
                    v39 = (unsigned int *)__errno_location();
                    if ((*v39 & 0xffffffef) == 0xc) goto label_4c07;
                    v35 = (char *)sub_ae30(4,(char *)*v38);
                    v47 = dcgettext(NULL,"cannot watch %s",5);
                    error(0,*v39,v47,v35);
                  }
                  v49 = *(int *)((long)v38 + 0x44);
                  if (v12 != v49) { // branch-flip
                    if (0 <= v49) {
                      inotify_rm_watch(v11);
                      sub_8e80(v50,v38);
                    }
                  }
                  else if (0 <= v49) goto label_4e3a;
                  *(int *)((long)v38 + 0x44) = v12;
                  if (v12 == -1) break;
                  v73 = (long *)sub_8e80(v50,v38);
                  if ((v73) && (v38 != v73)) {
                    if (dat_13020 == 1)
                      sub_6660(v73,0);
                    *(unsigned int *)((long)v73 + 0x44) = 0xffffffff;
                    v35 = (char *)sub_5fe0((char *)*v73);
                    sub_5bd0(*(unsigned int *)&v73[7],v35);
                  }
                  if (!sub_8e20(v50,v38)) {
                    sub_c220();
                  }
                }
label_4e3a:
                if (dat_13020 == 1)
                  sub_6660(v38,0);
                goto label_4e47;
              }
              v42 += 1;
              v38 = &v38[0xc];
            } while (v71 != v42);
            goto label_4d4b;
          }
          do {
            if (dat_13110) { // branch-flip
              if (v48)
                exit(0); // no-return
              if ((kill(dat_13110,0)) && (*__errno_location() != 1)) {
                v12 = 0;
                v48 = v9;
              }
              else {
                v12 = 0;
                if (0.0 < v76) {
                  if (dat_e4a8 <= v76) goto label_4fb1;
                  v12 = (int)(dat_e4b0 * v76);
                  v12 += (unsigned int)((double)v12 < dat_e4b0 * v76);
                }
              }
            }
            else {
label_4fb1:
              v12 = -1;
            }
            v53._4_2_ = 1;
            v53._8_8_ = 1;
            v53._0_4_ = v11;
            v12 = poll(&v53,(unsigned long)dat_13117 + 1,v12);
          } while (!v12);
          if (v12 < 0) {
            v35 = dcgettext(NULL,"error waiting for inotify and output events",5);
            error(1,*__errno_location(),v35);
            return v25;
          }
          if (v53._14_2_) {
            raise(0xd);
            exit(1);
          }
          v67 = (unsigned long)sub_b540(v11,v66,v69);
          if (v67) {
            v30 = 0;
            if (v67 != (char *)0xffffffffffffffff) goto label_4d63;
            if (*__errno_location() != 0x16) {
label_500c:
              v35 = dcgettext(NULL,"error reading inotify event",5);
              error(1,*__errno_location(),v35);
              return v24;
            }
          }
          if (!v70._0_4_) goto label_500c;
          v30 = 0;
          v70._0_4_ -= 1;
          v69 = (unsigned long)((long)v69 * 2);
          v66 = (void *)sub_bd70(v66,v69);
          v67 = NULL;
        } while( true );
      }
      if (!v10) {
        if (!v8) {
          exit(1);
        }
        goto label_4c85;
      }
label_4c28:
      sub_8980(v50);
      close(v11);
      *__errno_location() = 0;
    }
    v35 = dcgettext(NULL,"inotify cannot be used, reverting to polling",5);
    error(0,*__errno_location(),v35);
  }
  goto label_3b98;
  raise(0xd);
  exit(1);
}

// Function: sub_5340 @ 0x5340
void sub_5340(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_12fc8)(main,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: sub_5370 @ 0x5370
void sub_5370(void)
{
  return;
}

// Function: _FINI_0 @ 0x53e0
void _FINI_0(void)
{
  if (!dat_13108) {
    if (dat_12fe8)
      __cxa_finalize(dat_13008);
    sub_5370();
    dat_13108 = 1;
    return;
  }
}

// Function: _INIT_0 @ 0x5420
void _INIT_0(void)
{
  return;
}

// Function: sub_5430 @ 0x5430
unsigned long sub_5430(long a0,unsigned long a1)
{
  return (unsigned long)(long)*(int *)(a0 + 0x44) % a1;
}

// Function: sub_5450 @ 0x5450
unsigned int sub_5450(long a0,long a1)
{
  return CONCAT31((undefined3)((unsigned int)*(int *)(a1 + 0x44) >> 8),*(int *)(a0 + 0x44) == *(int *)(a1 + 0x44));
}

// Function: sub_5460 @ 0x5460
unsigned long sub_5460(int a0,char *a1) // ternary
{
  int *v1; // rax
  char *v2; // rax
  char *v3; // rax
  statfs v4; // stack - 0xa8
  unsigned long v5; // r12
  unsigned long v6; // r12
  undefined7 v7; // r12
  
  if (fstatfs(a0,&v4)) {
    v1 = __errno_location();
    v6 = 1;
    if (*v1 != 0x26) {
      v2 = (char *)sub_ae30(4,a1);
      v3 = dcgettext(NULL,"cannot determine location of %s. reverting to polling",5);
      error(0,*v1,v3,v2);
    }
    return v6 & 0xffffffff; // return-dupe
  }
  if (v4._0_8_ != 0x2bad1dea) {
    v7 = (undefined7)((unsigned long)v5 >> 8); // branch-flip
    if (0x2bad1deb <= (long)v4._0_8_) {
      if (v4._0_8_ != 0x62656570) {
        if (0x62656571 <= (long)v4._0_8_) { // branch-flip
          if (v4._0_8_ != 0x858458f6) {
            if (0x858458f7 <= (long)v4._0_8_) { // branch-flip
              if (v4._0_8_ != 0xcafe4a11) {
                if (0xcafe4a12 <= (long)v4._0_8_) { // branch-flip
                  if (v4._0_8_ != 0xf2f52010) {
                    if (0xf2f52011 <= (long)v4._0_8_) { // branch-flip
                      if (v4._0_8_ != 0xf97cff8c) {
                        v6 = CONCAT71(v7,v4._0_8_ != 0xf995e849);
                        return v6 & 0xffffffff;
                      }
                    }
                    else if (v4._0_8_ != 0xde5e81e4) {
                      v6 = CONCAT71(v7,v4._0_8_ != 0xe0f5e1e2);
                      return v6 & 0xffffffff;
                    }
                  }
                }
                else if (v4._0_8_ != 0xabba1974) {
                  if (0xabba1975 <= (long)v4._0_8_) { // branch-flip
                    if (v4._0_8_ != 0xc7571590) {
                      v6 = CONCAT71(v7,v4._0_8_ != 0xc97e8168);
                      return v6 & 0xffffffff;
                    }
                  }
                  else if (v4._0_8_ != 0x9123683e) {
                    v6 = CONCAT71(v7,v4._0_8_ != 0x958458f6);
                    return v6 & 0xffffffff;
                  }
                }
              }
            }
            else if (v4._0_8_ != 0x6c6f6f70) {
              if (0x6c6f6f71 <= (long)v4._0_8_) { // branch-flip
                if (v4._0_8_ != 0x73717368) {
                  if (0x73717369 <= (long)v4._0_8_) { // branch-flip
                    if (v4._0_8_ != 0x73727279) {
                      v6 = CONCAT71(v7,v4._0_8_ != 0x74726163);
                      return v6 & 0xffffffff;
                    }
                  }
                  else if (v4._0_8_ != 0x6e736673) {
                    v6 = CONCAT71(v7,v4._0_8_ != 0x73636673);
                    return v6 & 0xffffffff;
                  }
                }
              }
              else if (v4._0_8_ != 0x64646178) {
                if (0x64646179 <= (long)v4._0_8_) { // branch-flip
                  if (v4._0_8_ != 0x67596969) {
                    v6 = CONCAT71(v7,v4._0_8_ != 0x68191122);
                    return v6 & 0xffffffff;
                  }
                }
                else if ((v4._0_8_ != 0x63677270) && (v4._0_8_ != 0x64626720)) {
                  v6 = CONCAT71(v7,v4._0_8_ != 0x62656572);
                  return v6 & 0xffffffff;
                }
              }
            }
          }
        }
        else if (v4._0_8_ != 0x53464846) {
          if (0x53464847 <= (long)v4._0_8_) { // branch-flip
            if (v4._0_8_ != 0x58465342) {
              if (0x58465343 <= (long)v4._0_8_) { // branch-flip
                if (v4._0_8_ != 0x5dca2df5) {
                  if (0x5dca2df6 <= (long)v4._0_8_) { // branch-flip
                    if (v4._0_8_ != 0x6165676c) {
                      v6 = CONCAT71(v7,v4._0_8_ != 0x62646576);
                      return v6 & 0xffffffff;
                    }
                  }
                  else if (v4._0_8_ != 0x5a3c69f0) {
                    v6 = CONCAT71(v7,v4._0_8_ != 0x5a4f4653);
                    return v6 & 0xffffffff;
                  }
                }
              }
              else if (v4._0_8_ != 0x54190100) {
                if (0x54190101 <= (long)v4._0_8_) { // branch-flip
                  if (v4._0_8_ != 0x565a4653) {
                    v6 = CONCAT71(v7,v4._0_8_ != 0x58295829);
                    return v6 & 0xffffffff;
                  }
                }
                else if (v4._0_8_ != 0x5346544e) {
                  v6 = CONCAT71(v7,v4._0_8_ != 0x534f434b);
                  return v6 & 0xffffffff;
                }
              }
            }
          }
          else if (v4._0_8_ != 0x453dcd28) {
            if (0x453dcd29 <= (long)v4._0_8_) { // branch-flip
              if (v4._0_8_ != 0x52654973) {
                if (0x52654974 <= (long)v4._0_8_) { // branch-flip
                  if (v4._0_8_ != 0x5345434d) {
                    v6 = CONCAT71(v7,v4._0_8_ != 0x5346314d);
                    return v6 & 0xffffffff;
                  }
                }
                else if (v4._0_8_ != 0x454d444d) {
                  v6 = CONCAT71(v7,v4._0_8_ != 0x45584653);
                  return v6 & 0xffffffff;
                }
              }
            }
            else if (v4._0_8_ != 0x42494e4d) {
              if (0x42494e4e <= (long)v4._0_8_) { // branch-flip
                if (v4._0_8_ != 0x43415d53) {
                  v6 = CONCAT71(v7,v4._0_8_ != 0x444d4142);
                  return v6 & 0xffffffff;
                }
              }
              else if ((v4._0_8_ != 0x3153464a) && (v4._0_8_ != 0x42465331)) {
                v6 = CONCAT71(v7,v4._0_8_ != 0x2fc12fc1);
                return v6 & 0xffffffff;
              }
            }
          }
        }
      }
    }
    else if (0x9fa3 <= (long)v4._0_8_) { // branch-flip
      if (0x12ff7b8 <= (long)v4._0_8_) { // branch-flip
        if (v4._0_8_ != 0x15013346) {
          if (0x15013347 <= (long)v4._0_8_) { // branch-flip
            if (v4._0_8_ != 0x2011bab0) {
              if (0x2011bab1 <= (long)v4._0_8_) { // branch-flip
                if (v4._0_8_ != 0x24051905) {
                  v6 = CONCAT71(v7,v4._0_8_ != 0x28cd3d45);
                  return v6 & 0xffffffff;
                }
              }
              else if (v4._0_8_ != 0x19800202) {
                v6 = CONCAT71(v7,v4._0_8_ != 0x1badface);
                return v6 & 0xffffffff;
              }
            }
          }
          else if (v4._0_8_ != 0xbad1dea) {
            if (0xbad1deb <= (long)v4._0_8_) { // branch-flip
              if (v4._0_8_ != 0x11307854) {
                v6 = CONCAT71(v7,v4._0_8_ != 0x13661366);
                return v6 & 0xffffffff;
              }
            }
            else if (v4._0_8_ != 0x7655821) {
              v6 = CONCAT71(v7,v4._0_8_ != 0x9041934);
              return v6 & 0xffffffff;
            }
          }
        }
      }
      else if (((long)v4._0_8_ <= 0x12ff7b3) && (v4._0_8_ != 0x27e0eb)) {
        if (0x27e0ec <= (long)v4._0_8_) { // branch-flip
          if (v4._0_8_ != 0x1021994) {
            if (0x1021995 <= (long)v4._0_8_) { // branch-flip
              if (v4._0_8_ != 0x1021997) {
                v6 = CONCAT71(v7,v4._0_8_ != 0x12fd16d);
                return v6 & 0xffffffff;
              }
            }
            else if (v4._0_8_ != 0x414a53) {
              v6 = CONCAT71(v7,v4._0_8_ != 0xc0ffee);
              return v6 & 0xffffffff;
            }
          }
        }
        else if (v4._0_8_ != 0xef53) {
          if (0xef54 <= (long)v4._0_8_) { // branch-flip
            if (v4._0_8_ != 0xf15f) {
              v6 = CONCAT71(v7,v4._0_8_ != 0x11954);
              return v6 & 0xffffffff;
            }
          }
          else if ((v4._0_8_ != 0xadff) && (v4._0_8_ != 0xef51)) {
            v6 = CONCAT71(v7,v4._0_8_ != 0xadf5);
            return v6 & 0xffffffff;
          }
        }
      }
    }
    else if ((long)v4._0_8_ <= 0x9f9f) {
      if (0x4007 <= (long)v4._0_8_) { // branch-flip
        if (v4._0_8_ != 0x4d5a) {
          if (0x4d5b <= (long)v4._0_8_) { // branch-flip
            if (v4._0_8_ != 0x72b6) {
              if (0x72b7 <= (long)v4._0_8_) {
                v6 = CONCAT71(v7,v4._0_8_ != 0x9660);
                return v6 & 0xffffffff;
              }
              if (v4._0_8_ != 0x5df5) {
                v6 = CONCAT71(v7,v4._0_8_ != 0x7275);
                return v6 & 0xffffffff;
              }
            }
          }
          else if (v4._0_8_ != 0x4858) {
            if (0x4859 <= (long)v4._0_8_) {
              v6 = CONCAT71(v7,v4._0_8_ != 0x4d44);
              return v6 & 0xffffffff;
            }
            if (v4._0_8_ != 0x4244) {
              v6 = CONCAT71(v7,v4._0_8_ != 0x482b);
              return v6 & 0xffffffff;
            }
          }
        }
      }
      else {
        if (0x3fff < (long)v4._0_8_) {
          v6 = (unsigned long)((unsigned int)(0x51L >> (v4._0_1_ & 0x3f)) & 1 ^ 1);
          return v6 & 0xffffffff;
        }
        if (0x1390 <= (long)v4._0_8_) { // branch-flip
          if (v4._0_8_ != 0x2478) {
            if (0x2479 <= (long)v4._0_8_) {
              v6 = CONCAT71(v7,v4._0_8_ != 0x3434);
              return v6 & 0xffffffff;
            }
            if (v4._0_8_ != 0x1cd1) {
              v6 = CONCAT71(v7,v4._0_8_ != 0x2468);
              return v6 & 0xffffffff;
            }
          }
        }
        else {
          if (0x1372 < (long)v4._0_8_) {
            v6 = (unsigned long)((unsigned int)(0x10001401L >> (v4._0_1_ + 0x8d & 0x3f)) & 1 ^ 1);
            return v6 & 0xffffffff;
          }
          if (v4._0_8_ != 0x187) {
            v6 = (0x188 <= (long)v4._0_8_) ? CONCAT71(v7,v4._0_8_ != 0x7c0) : CONCAT71(v7,(v4._0_8_ - 0x2f & 0xfffffffffffffffb) != 0); // branch-flip
            return v6 & 0xffffffff;
          }
        }
      }
    }
  }
  v6 = 0;
  return v6 & 0xffffffff;
}

// Function: sub_5bd0 @ 0x5bd0
void sub_5bd0(unsigned int a0,char *a1)
{
  unsigned long v1; // rax
  char *v2; // rax
  
  if (a0 + 1 <= 1)
    return;
  if (!close(a0))
    return;
  v1 = sub_ae30(4,a1);
  v2 = dcgettext(NULL,"closing %s (fd=%d)",5);
  error(0,*__errno_location(),v2,v1,(unsigned long)a0); // tail-call
}

// Function: sub_5c60 @ 0x5c60
void sub_5c60(void *a0,unsigned long a1)
{
  unsigned long v1; // rax
  char *v2; // rax
  
  if (a1 <= fwrite_unlocked(a0,1,a1,stdout))
    return;
  clearerr_unlocked(stdout);
  v1 = sub_ae30(4,"standard output");
  v2 = dcgettext(NULL,"error writing %s",5);
  error(1,*__errno_location(),v2,v1);
}

// Function: sub_5ce0 @ 0x5ce0
unsigned long sub_5ce0(char *a0,unsigned int a1,long a2,long *a3) // return-dupe
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
  do {
    v2 = sub_b540(a1,v6,_DT_INIT);
    if (!v2)
      return 0xffffffff;
    if (v2 == -1) {
      v4 = sub_ae30(4,a0);
      v5 = dcgettext(NULL,"error reading %s",5);
      error(0,*__errno_location(),v5,v4);
      return 1;
    }
    *a3 = *a3 + v2;
    v8 = (int)dat_13114;
    v1 = &v6[v2];
    v7 = v6;
    while (v3 = memchr(v7,v8,(long)v1 - (long)v7), v3) {
      v7 = (char *)((long)v3 + 1);
      a2 -= 1;
      if (!a2) {
        if (v1 <= v7)
          return 0;
        if (!((long)v1 - (long)v7))
          return 0;
        sub_5c60(v7,(long)v1 - (long)v7);
        return 0;
      }
    }
  } while( true );
}

// Function: sub_5e30 @ 0x5e30
unsigned long sub_5e30(char *a0,unsigned int a1,unsigned long a2,long *a3) // return-dupe
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  char *v3; // rax
  char v4 [8200];
  
  if (!a2)
    return 0;
  do {
    v1 = sub_b540(a1,v4,_DT_INIT);
    if (!v1)
      return 0xffffffff;
    if (v1 == 0xffffffffffffffff) {
      v2 = sub_ae30(4,a0);
      v3 = dcgettext(NULL,"error reading %s",5);
      error(0,*__errno_location(),v3,v2);
      return 1;
    }
    *a3 = *a3 + v1;
    if (a2 < v1) {
      if (v1 - a2) {
        sub_5c60(&v4[a2],v1 - a2);
        return 0;
      }
      return 0;
    }
    a2 -= v1;
  } while (a2);
  return 0;
}

// Function: sub_5f40 @ 0x5f40
void sub_5f40(unsigned long a0,int a1,char *a2)
{
  unsigned long v1; // rax
  int *v2; // rax
  unsigned long v3; // rax
  char v4 [24];
  char *v5; // rax
  long v6; // fs_offset
  unsigned long v7; // stack - 0x30
  
  v7 = *(unsigned long *)(v6 + 0x28);
  v1 = sub_9020(a0,v4);
  v2 = __errno_location();
  if (a1 != 1) { // branch-flip
    v3 = sub_b1b0(0,3,a2);
    v5 = "%s: cannot seek to offset %s";
  }
  else {
    v3 = sub_b1b0(0,3,a2);
    v5 = "%s: cannot seek to relative offset %s";
  }
  v5 = dcgettext(NULL,v5,5);
  error(0,*v2,v5,v3,v1);
  exit(1); // no-return
}

// Function: sub_5fe0 @ 0x5fe0
char * sub_5fe0(char *a0)
{
  if (strcmp(a0,"-"))
    return a0;
  return dcgettext(NULL,"standard input",5); // tail-call
}

// Function: sub_6020 @ 0x6020
int * sub_6020(char a0,char *a1,unsigned int a2,unsigned long a3)
{
  int *v1;
  int *v2; // rax
  unsigned long v3;
  char *v4;
  unsigned long v5;
  char v6 [8200];
  long v7; // r12
  
  v7 = 0;
  v5 = a3;
  do {
    v3 = 0x2000;
    if (v5 <= 0x2000)
      v3 = v5;
    v2 = (unsigned long)sub_b540(a2,v6,v3);
    if (v2 == (int *)0xffffffffffffffff) {
      v2 = __errno_location();
      v1 = v2;
      if (*v2 == 0xb) {
        v2 = (int *)v7;
        return v2;
      }
      v2 = (char *)sub_ae30(4,a1);
      v4 = (char *)v2;
      v2 = (char *)dcgettext(NULL,"error reading %s",5);
      error(1,*v1,(char *)v2,v4);
      return v2;
    }
    if (!v2) {
      v2 = (int *)v7;
      return v2;
    }
    if (a0) {
      v4 = "\n";
      if (dat_13010)
        v4 = "";
      __printf_chk(1,"%s==> %s <==\n",v4,a1);
      dat_13010 = '\0';
    }
    v7 += (long)v2;
    sub_5c60(v6,v2);
    if (a3 != 0xffffffffffffffff) {
      v5 -= (long)v2;
      if (!v5) {
        v2 = (int *)v7;
        return v2;
      }
      if (a3 == 0xfffffffffffffffe) {
        v2 = (int *)v7;
        return v2;
      }
    }
    a0 = 0;
  } while( true );
}

// Function: sub_6190 @ 0x6190
unsigned long sub_6190(char *a0,unsigned int a1,long a2,unsigned long a3,unsigned long a4,long *a5) // return-dupe
{
  long v1; // rax
  unsigned long v10; // stack - 0x2050
  unsigned long v2;
  void *v3; // rax
  long v4;
  unsigned long v5; // rax
  char *v6; // rax
  unsigned long v7;
  int v8; // ebx
  char v9 [8200];
  
  if (!a2)
    return 1;
  v1 = (long)(a4 - a3) % 0x2000;
  v2 = 0x2000;
  if (v1)
    v2 = v1;
  v1 = a4 - v2;
  v10 = a4;
  if ((long)lseek(a1,v1,0) < 0)
    sub_5f40(v1,0,a0); // no-return
  v2 = sub_b540(a1,v9,v2);
  if (v2 != 0xffffffffffffffff) {
    v8 = (int)dat_13114;
    *a5 = v1 + v2;
    if ((v2) && (v9[v2 - 1] != dat_13114))
      a2 -= 1;
    do {
      v7 = v2;
      while ((v7 && (v3 = memrchr(v9,v8,v7), v3))) {
        v4 = a2 + -1;
        v7 = (long)v3 - (long)v9;
        if (!a2) {
          if (v2 - 1 != v7)
            sub_5c60((void *)((long)v3 + 1));
          v4 = sub_6020(0,a0,a1,(v10 - v2) - v1);
          *a5 = *a5 + v4;
          return 1;
        }
        a2 = v4;
      }
      if (v1 == a3) {
        if (0 <= (long)lseek(a1,v1,0)) {
          *a5 = sub_6020(0,a0,a1,v10) + v1;
          return 1;
        }
        sub_5f40(v1,0,a0); // return-dupe, no-return
      }
      v1 -= 0x2000;
      if ((long)lseek(a1,v1,0) < 0) {
        sub_5f40(v1,0,a0);
      }
      v2 = sub_b540(a1,v9,0x2000);
      if (v2 == 0xffffffffffffffff) break;
      *a5 = v1 + v2;
      if (!v2)
        return 1;
      v8 = (int)dat_13114;
    } while( true );
  }
  v5 = sub_ae30(4,a0);
  v6 = dcgettext(NULL,"error reading %s",5);
  error(0,*__errno_location(),v6,v5);
  return 0;
}

// Function: sub_6430 @ 0x6430
void sub_6430(struct_1 *a0,long *a1) // return-dupe x3
{
  unsigned int v1;
  long v2; // rax
  char *v3;
  int *v4; // rax
  char *v5; // rax
  stat v6; // stack - 0xc8
  int v7;
  bool v8;
  
  v7 = a0->field_0x38;
  if (v7 == -1)
    return;
  v3 = a0->field_0x0;
  if (!strcmp(v3,"-")) {
    v3 = dcgettext(NULL,"standard input",5);
    v7 = a0->field_0x38;
  }
  if (fstat(v7,&v6)) {
    v4 = __errno_location();
    a0->field_0x3c = *v4;
    sub_5bd0(a0->field_0x38,v3);
    a0->field_0x38 = 0xffffffff;
    return;
  }
  if ((a0->field_0x30 & 0xf000) == 0x8000) {
    if ((long)a0->field_0x8 <= (long)v6._48_8_) { // branch-flip
      if ((v6._48_8_ == a0->field_0x8) && (!(((unsigned int)((long)v6._96_8_ < a0->field_0x18) - (unsigned int)(a0->field_0x18 < (long)v6._96_8_)) + ((unsigned int)((long)v6._88_8_ < a0->field_0x10) - (unsigned int)(a0->field_0x10 < (long)v6._88_8_)) * 2)))
        return;
    }
    else {
      v5 = (char *)sub_b1b0(0,3,v3);
      error(0,0,dcgettext(NULL,"%s: file truncated",5),v5);
      v1 = a0->field_0x38;
      if ((long)lseek(v1,0,0) < 0)
        sub_5f40(0,0,v3); // no-return
      a0->field_0x8 = 0;
    }
  }
  v8 = 0;
  if (dat_13115)
    v8 = (struct_1 *)*a1 != a0;
  v2 = sub_6020(v8,v3,a0->field_0x38,0xffffffffffffffff);
  a0->field_0x8 = a0->field_0x8 + v2;
  if (!v2)
    return;
  *a1 = (long)a0;
  if (!fflush_unlocked(stdout))
    return;
  v3 = dcgettext(NULL,"write error",5);
  error(1,*__errno_location(),v3);
}

// Function: sub_6660 @ 0x6660
void sub_6660(struct_2 *a0,unsigned int a1)
{
  char v1;
  unsigned long v10; // rax
  char *v11;
  char *v12;
  stat v13; // stack - 0xd8
  unsigned int v2;
  int v3;
  char v4;
  int v5; // eax
  int v6; // eax
  unsigned int v7;
  int v8; // eax
  unsigned int *v9; // rax
  
  v11 = a0->field_0x0;
  v5 = strcmp(v11,"-");
  v2 = a0->field_0x3c;
  v4 = a0->field_0x36;
  if (v5) { // branch-flip
    v6 = sub_7dd0(v11,((a1 ^ 1) & 0xff) << 0xb);
    v7 = a0->field_0x3c;
  }
  else {
    v6 = 0;
    v7 = v2;
  }
  v1 = dat_1311a;
  if ((a0->field_0x38 == -1) == (v7 == 0))
    __assert_fail("valid_file_spec (f)","src/tail.c",0x3e2,"recheck"); // no-return
  if (dat_1311a) { // branch-flip
    if (v6 != -1) { // branch-flip
      a0->field_0x36 = 1;
      if ((dat_1310c) || ((v11 = a0->field_0x0, lstat(v11,&v13) || ((v13._24_4_ & 0xf000) != 0xa000)))) goto label_67fb;
    }
    else {
      a0->field_0x36 = 0;
      v11 = a0->field_0x0;
      if (dat_1310c) {
        a0->field_0x3c = *__errno_location();
label_6991:
        v5 = strcmp(v11,"-");
        if (v4) {
          if (!v5)
            v11 = dcgettext(NULL,"standard input",5);
          v11 = (char *)sub_ae30(4,v11);
          v12 = dcgettext(NULL,"%s has become inaccessible",5);
          error(0,a0->field_0x3c,v12,v11);
          v11 = a0->field_0x0;
          goto label_675a;
        }
        goto label_69a7;
      }
      if ((lstat(v11,&v13)) || ((v13._24_4_ & 0xf000) != 0xa000)) goto label_6714;
    }
label_6cd0:
    v11 = a0->field_0x0;
    a0->field_0x3c = 0xffffffff;
    *(char *)&a0->field_0x34 = 1;
    if (!strcmp(v11,"-"))
      v11 = dcgettext(NULL,"standard input",5);
    v11 = (char *)sub_ae30(4,v11);
    error(0,0,dcgettext(NULL,"%s has been replaced with an untailable symbolic link",5),v11);
    v11 = a0->field_0x0;
  }
  else {
    a0->field_0x36 = 1;
    if ((!dat_1310c) && ((v11 = a0->field_0x0, !lstat(v11,&v13) && ((v13._24_4_ & 0xf000) == 0xa000)))) goto label_6cd0;
    if (v6 != -1) {
label_67fb:
      if (fstat(v6,&v13) < 0) goto label_6714;
      if (((v13._24_4_ & 0xf000) - 0x1000 & 0xffffe000) && ((v13._24_4_ & 0xb000) != 0x8000)) {
        a0->field_0x3c = 0xffffffff;
        a0->field_0x36 = 0;
        if ((v1) && (dat_13020 == 1)) {
          *(char *)&a0->field_0x34 = 0;
          if (v4) // branch-flip
            v11 = "";
          else {
            v11 = "";
            if (v2 == 0xffffffff) goto label_69a7;
          }
        }
        else {
          *(char *)&a0->field_0x34 = 1;
          if ((!v4) && (v2 == 0xffffffff)) goto label_69a7;
          v11 = dcgettext(NULL,"; giving up on this name",5);
        }
        v12 = a0->field_0x0;
        if (!strcmp(v12,"-"))
          v12 = dcgettext(NULL,"standard input",5);
        v12 = (char *)sub_ae30(4,v12);
        error(0,0,dcgettext(NULL,"%s has been replaced with an untailable file%s",5),v12,v11);
      }
      else {
        v11 = a0->field_0x0;
        if (!strcmp(v11,"-"))
          v11 = dcgettext(NULL,"standard input",5);
        v4 = sub_5460(v6,v11);
        *(char *)((long)&a0->field_0x34 + 1) = v4;
        if ((!v4) || (dat_1310c)) {
          a0->field_0x3c = 0;
          v3 = a0->field_0x38;
          if (v2 & 0xfffffffd) { // branch-flip
            if (v3 != -1)
              __assert_fail("f->fd == -1","src/tail.c",0x42f,"recheck"); // no-return
            v11 = a0->field_0x0;
            if (!strcmp(v11,"-"))
              v11 = dcgettext(NULL,"standard input",5);
            v10 = sub_ae30(4,v11);
            v11 = "%s has become accessible";
          }
          else {
            v11 = a0->field_0x0;
            v8 = strcmp(v11,"-");
            if (v3 != -1) {
              if ((a0->field_0x28 == v13._8_8_) && (a0->field_0x20 == v13._0_8_)) {
                if (!v8)
                  v11 = dcgettext(NULL,"standard input",5);
                sub_5bd0(v6,v11);
                return;
              }
              if (!v8)
                v11 = dcgettext(NULL,"standard input",5);
              v11 = (char *)sub_ae30(4,v11);
              error(0,0,dcgettext(NULL,"%s has been replaced;  following new file",5),v11);
              v11 = a0->field_0x0;
              if (!strcmp(v11,"-"))
                v11 = dcgettext(NULL,"standard input",5);
              sub_5bd0(a0->field_0x38,v11);
              goto label_68e3;
            }
            if (!v8)
              v11 = dcgettext(NULL,"standard input",5);
            v10 = sub_ae30(4,v11);
            v11 = "%s has appeared;  following new file";
          }
          error(0,0,dcgettext(NULL,v11,5),v10);
label_68e3:
          a1 &= 0xff;
          a0->field_0x38 = v6;
          if (!v5)
            a1 = 0xffffffff;
          *(char *)&a0->field_0x34 = 0;
          a0->field_0x8 = 0;
          a0->field_0x10 = v13._88_8_;
          a0->field_0x40 = a1;
          v11 = a0->field_0x0;
          a0->field_0x18 = v13._96_8_;
          a0->field_0x58 = 0;
          a0->field_0x20 = v13._0_8_;
          a0->field_0x28 = v13._8_8_;
          a0->field_0x30 = v13._24_4_;
          if (!strcmp(v11,"-"))
            v11 = dcgettext(NULL,"standard input",5);
          if (0 <= (long)lseek(v6,0,0))
            return;
          sub_5f40(0,0,v11); // no-return
        }
        v11 = a0->field_0x0;
        a0->field_0x3c = 0xffffffff;
        if (!strcmp(v11,"-"))
          v11 = dcgettext(NULL,"standard input",5);
        v11 = (char *)sub_ae30(4,v11);
        error(0,0,dcgettext(NULL,"%s has been replaced with an untailable remote file",5),v11);
        a0->field_0x34 = 0x101;
      }
label_69a7:
      v11 = a0->field_0x0;
      v5 = strcmp(v11,"-");
      goto label_69b7;
    }
label_6714:
    v1 = a0->field_0x36;
    v9 = (unsigned int *)__errno_location();
    v11 = a0->field_0x0;
    v7 = *v9;
    a0->field_0x3c = v7;
    if (!v1) goto label_6991;
    v5 = strcmp(v11,"-");
    if (v7 != v2) {
      if (!v5)
        v11 = dcgettext(NULL,"standard input",5);
      v11 = (char *)sub_b1b0(0,3,v11);
      error(0,*v9,"%s",v11);
      v11 = a0->field_0x0;
    }
  }
label_675a:
  v5 = strcmp(v11,"-");
label_69b7:
  if (!v5)
    v11 = dcgettext(NULL,"standard input",5);
  sub_5bd0(v6,v11);
  v11 = a0->field_0x0;
  if (!strcmp(v11,"-"))
    v11 = dcgettext(NULL,"standard input",5);
  sub_5bd0(a0->field_0x38,v11);
  a0->field_0x38 = 0xffffffff;
}

// Function: sub_6f70 @ 0x6f70
void sub_6f70(int a0)
{
  FILE *v1;
  char *v10; // stack - 0x88
  char *v11; // stack - 0x80
  char *v12; // stack - 0x78
  char *v13; // stack - 0x70
  char *v14; // stack - 0x68
  char *v15; // stack - 0x60
  unsigned long v16; // stack - 0x58
  unsigned long v17; // stack - 0x50
  unsigned long v18; // stack - 0x40
  char *v19 [4]; // stack - 0xa8
  int v2; // eax
  char *v3; // rax
  char **v4; // rbx
  char *v5; // stack - 0xb8
  char *v6;
  char *v7; // r13
  long v8; // fs_offset
  char *v9; // stack - 0xb0
  
  v6 = dat_13138;
  v4 = &v5;
  v18 = *(unsigned long *)(v8 + 0x28);
  if (a0) // branch-flip
    __fprintf_chk(stderr,1,dcgettext(NULL,"Try \'%s --help\' for more information.\n",5),v6);
  else {
    __printf_chk(1,dcgettext(NULL,"Usage: %s [OPTION]... [FILE]...\n",5),v6);
    __printf_chk(1,dcgettext(NULL,"Print the last %d lines of each FILE to standard output.\nWith more than one FILE, precede each with a header giving the file name.\n",5),10);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nWith no FILE, or when FILE is -, read standard input.\n",5),v1);
    v1 = stdout;
    fputs_unlocked(dcgettext(NULL,"\nMandatory arguments to long options are mandatory for short options too.\n",5),v1);
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
    v5 = "[";
    v6 = "[";
    v9 = "test invocation";
    v19[0] = "coreutils";
    v19[1] = "Multi-call invocation";
    v10 = "sha256sum";
    v19[2] = "sha224sum";
    v12 = "sha384sum";
    v19[3] = "sha2 utilities";
    v11 = "sha2 utilities";
    v13 = "sha2 utilities";
    v14 = "sha512sum";
    v15 = "sha2 utilities";
    v16 = 0;
    v17 = 0;
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
    if (v3) {
      v2 = strncmp(v3,"en_",3);
      v1 = stdout;
      if (v2)
        fputs_unlocked(dcgettext(NULL,"Report any translation bugs to <https://translationproject.org/team/>\n",5),v1);
    }
    v3 = "tail";
    if (!strcmp("tail","["))
      v3 = "test";
    v7 = " invocation";
    __printf_chk(1,dcgettext(NULL,"Full documentation <%s%s>\n",5),"https://www.gnu.org/software/coreutils/",v3);
    if (v6 != "tail")
      v7 = "";
    __printf_chk(1,dcgettext(NULL,"or available locally via: info \'(coreutils) %s%s\'\n",5),v6,v7);
  }
  exit(a0); // no-return
}

// Function: sub_73d0 @ 0x73d0
void sub_73d0(void)
{
  sub_6f70(1); // tail-call, no-return
}

// Function: sub_73e0 @ 0x73e0
unsigned long sub_73e0(char *a0,long *a1,void *a2,unsigned long a3)
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

// Function: sub_74f0 @ 0x74f0
long sub_74f0(char *a0,long *a1) // return-dupe
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

// Function: sub_7550 @ 0x7550
void sub_7550(char *a0,char *a1,long a2)
{
  char *v1; // rax
  unsigned long v2; // rax
  
  if (a2 != -1) // branch-flip
    v1 = dcgettext(NULL,"ambiguous argument %s for %s",5);
  else {
    v1 = dcgettext(NULL,"invalid argument %s for %s",5);
  }
  v2 = sub_b500(1,a0);
  error(0,0,v1,sub_ad00(0,8,a1),v2); // tail-call
}

// Function: sub_75e0 @ 0x75e0
void sub_75e0(unsigned long *a0,void *a1,unsigned long a2)
{
  char *v1;
  char *v2;
  FILE *v3;
  long v4;
  void *v5;
  
  v3 = stderr;
  v5 = NULL;
  fputs_unlocked(dcgettext(NULL,"Valid arguments are:",5),v3);
  v4 = 0;
  v1 = (char *)*a0;
  while (v1) {
    if ((v4) && (!memcmp(v5,a1,a2)))
      __fprintf_chk(stderr,1,", %s",(char *)sub_b520(v1));
    else {
      __fprintf_chk(stderr,1,"\n  - %s",sub_b520(v1));
      v5 = a1;
    }
    v4 += 1;
    a1 = (void *)((long)a1 + a2);
    v1 = (char *)a0[v4];
  }
  v2 = *(char **)&stderr->field_0x28;
  if (v2 < *(char **)&stderr->field_0x30) {
    *(char **)&stderr->field_0x28 = &v2[1];
    *v2 = 10;
    return;
  }
  __overflow(stderr,10); // tail-call
}

// Function: sub_7710 @ 0x7710
long sub_7710(char *a0,char *a1,long *a2,void *a3,unsigned long a4,void *a5,char a6) // early-return
{
  char *v1;
  long v2;
  
  if (a6) { // branch-flip
    v2 = sub_73e0(a1,a2,a3,a4);
    if (0 <= v2)
      return v2;
  }
  else {
    v2 = 0;
    v1 = (char *)*a2;
    while (v1) {
      if (!strcmp(v1,a1))
        return v2;
      v2 += 1;
      v1 = (char *)a2[v2];
    }
    v2 = -1;
  }
  sub_7550(a0,a1,v2);
  sub_75e0(a2,a3,a4);
  (*a5)();
  return -1;
}

// Function: sub_77e0 @ 0x77e0
long sub_77e0(void *a0,long *a1,void *a2,unsigned long a3) // return-dupe
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

// Function: sub_7840 @ 0x7840
char * sub_7840(char *a0)
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

// Function: sub_78a0 @ 0x78a0
void sub_78a0(char *a0)
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

// Function: sub_78d0 @ 0x78d0
unsigned long sub_78d0(unsigned long a0,unsigned long *a1)
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
    v5 = sub_c950(a0,&v4);
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

// Function: sub_7990 @ 0x7990
void sub_7990(unsigned long a0)
{
  dat_13128 = a0;
}

// Function: sub_79a0 @ 0x79a0
void sub_79a0(char a0)
{
  dat_13120 = a0;
}

// Function: sub_79b0 @ 0x79b0
void sub_79b0(void)
{
  int v1; // eax
  int *v2; // rax
  char *v3; // rax
  char *v4; // rax
  
  v1 = sub_c9d0(stdout);
  if (v1) {
    v2 = __errno_location();
    if ((!dat_13120) || (*v2 != 0x20)) {
      v3 = dcgettext(NULL,"write error",5);
      if (dat_13128) { // branch-flip
        v4 = (char *)sub_b090(dat_13128);
        error(0,*v2,"%s: %s",v4,v3);
      }
      else {
        error(0,*v2,"%s",v3);
      }
      _exit(dat_13040); // return-dupe, no-return
    }
  }
  v1 = sub_c9d0(stderr);
  if (!v1)
    return;
  _exit(dat_13040);
}

// Function: sub_7a60 @ 0x7a60
void sub_7a60(char *a0)
{
  char v1;
  long v2;
  unsigned long v3;
  
  v1 = *a0;
  v3 = sub_7840(a0) - (long)a0;
  do {
    if ((v1 == '/') >= v3) break;
    v2 = v3 - 1;
    v3 -= 1;
  } while (a0[v2] == '/');
}

// Function: sub_7aa0 @ 0x7aa0
char * sub_7aa0(char *a0)
{
  unsigned long v1;
  char v2;
  void *v3; // rax
  char *v4; // rax
  unsigned long v5;
  
  v2 = *a0;
  v5 = sub_7840(a0) - (long)a0;
  do {
    if (v5 <= (v2 == '/')) {
      v3 = malloc(v5 + 1 + (v5 ^ 1));
      if (v3) { // branch-flip
        v4 = memcpy(v3,a0,v5);
        if ((v5 ^ 1) & 1) { // branch-flip
          *v4 = 0x2e;
          v5 = 1;
        }
        else {
          v5 = 1;
        }
label_7b0e:
        v4[v5] = 0;
      }
      else {
        v4 = NULL;
      }
      return v4;
    }
    v1 = v5 - 1;
    if (a0[v5 - 1] != '/') {
      v3 = malloc(v5 + 1);
      if (!v3) {
        v4 = NULL;
        return v4;
      }
      v4 = memcpy(v3,a0,v5);
      goto label_7b0e;
    }
    v5 = v1;
  } while( true );
}

// Function: sub_7b60 @ 0x7b60
int sub_7b60(int a0,int a1,unsigned long a2)
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
    if (0 <= dat_13130) { // branch-flip
      v2 = fcntl(a0,0x406,a2 & 0xffffffff);
      if ((0 <= v2) || (*__errno_location() != 0x16)) {
        dat_13130 = 1;
        return v2;
      }
      v2 = fcntl(a0,0,a2 & 0xffffffff);
      if (v2 < 0)
        return v2;
      dat_13130 = -1;
    }
    else {
      v2 = fcntl(a0,0,a2 & 0xffffffff);
      if (v2 < 0)
        return v2;
      if (dat_13130 != -1)
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

// Function: sub_7dd0 @ 0x7dd0
void sub_7dd0(char *a0,unsigned int a1,unsigned long a2)
{
  unsigned long v1; // rdx
  
  v1 = 0;
  if (a1 & 0x40)
    v1 = a2 & 0xffffffff;
  sub_b5b0(open(a0,a1,v1));
}

// Function: sub_7e50 @ 0x7e50
unsigned long sub_7e50(unsigned long a0,unsigned long a1)
{
  return (a0 >> 3 | a0 << 0x3d) % a1;
}

// Function: sub_7e70 @ 0x7e70
bool sub_7e70(long a0,long a1)
{
  return a1 == a0;
}

// Function: sub_7e80 @ 0x7e80
unsigned long sub_7e80(long a0) // return-dupe
{
  float v1;
  float *v2;
  float v3; // xmm1_da
  
  v2 = *(float **)(a0 + 0x28);
  if (v2 == (float *)0xe590)
    return 1;
  v1 = v2[2];
  if ((((dat_e5a4 < v1) && (v1 < dat_e5a8)) && (dat_e5ac < v2[3])) && (dat_e5c0 <= *v2)) {
    v3 = *v2 + dat_e5a4;
    if (((v3 < v2[1]) && (v2[1] <= dat_e5b0)) && (v3 < v1))
      return 1;
  }
  *(unsigned long *)(a0 + 0x28) = 0xe590;
  return 0;
}

// Function: sub_7f10 @ 0x7f10
unsigned long sub_7f10(struct_5 *a0,long a1,unsigned long *a2,bool a3) // early-return
{
  long *v1;
  long v2;
  char v3; // al
  unsigned long v4; // rax
  long *v5;
  
  v4 = (*a0->field_0x30)(a1,a0->field_0x10);
  if ((unsigned long)a0->field_0x10 <= v4)
    abort(); // no-return
  v5 = (long *)(v4 * 0x10 + a0->field_0x0);
  *a2 = v5;
  v4 = *v5;
  if (!v4) {
    v4 = 0;
    return v4;
  }
  if (v4 != a1) {
    v3 = (*a0->field_0x38)(a1);
    if (!v3) {
      while( true ) {
        if (!(long *)v5[1]) {
          v4 = 0;
          return v4;
        }
        if ((*(long *)v5[1] == a1) || (v3 = (*a0->field_0x38)(a1), v3)) break;
        v5 = (long *)v5[1];
      }
      v1 = (long *)v5[1];
      v4 = *v1;
      if (!a3)
        return v4;
      v5[1] = v1[1];
      *v1 = 0;
      v1[1] = a0->field_0x48;
      a0->field_0x48 = (long)v1;
      return v4;
    }
    v4 = *v5;
  }
  if (a3) {
    v1 = (long *)v5[1];
    if (v1) {
      v2 = v1[1];
      *v5 = *v1;
      v5[1] = v2;
      *v1 = 0;
      v1[1] = a0->field_0x48;
      a0->field_0x48 = (long)v1;
      return v4;
    }
    *v5 = 0;
  }
  return v4;
}

// Function: sub_8020 @ 0x8020
unsigned long sub_8020(float a0,unsigned long a1,bool a2) // ternary
{
  unsigned long v1; // rax
  unsigned long v2; // rcx
  unsigned long v3; // rsi
  long v4; // rdi
  unsigned long v5;
  
  if (!a2) {
    a0 = (float)a1 / a0;
    if (dat_e5b4 <= a0) {
      v1 = 0;
      return v1;
    }
    a1 = (a0 < dat_e5b8) ? (unsigned long)a0 : (long)(a0 - dat_e5b8) ^ 0x8000000000000000;
  }
  v1 = 10;
  if (10 <= a1)
    v1 = a1;
  v5 = v1 | 1;
  do {
    if (v5 == 0xffffffffffffffff) {
      v1 = 0;
      return v1;
    }
    v1 = v5 / 3;
    if (10 <= v5) { // branch-flip
      if (v5 != (SUB168(ZEXT816(v5) * ZEXT816(0xaaaaaaaaaaaaaaab),8) & 0xfffffffffffffffe) + v1) {
        v4 = 0x10;
        v3 = 9;
        v2 = 3;
        do {
          v2 += 2;
          v3 += v4;
          if (v5 <= v3) goto label_80f5;
          v4 += 8;
        } while (v5 % v2);
      }
    }
    else {
      v2 = 3;
label_80f5:
      if (v5 % v2) {
        v1 = v5 >> 0x3d;
        if (v5 >> 0x3c & 1) {
          v1 = 0;
          return v1;
        }
        if (v1) {
          v1 = 0;
          return v1;
        }
        v1 = v5;
        return v1;
      }
    }
    v5 += 2;
  } while( true );
}

// Function: sub_8160 @ 0x8160
unsigned long sub_8160(struct_6 *a0,struct_10 *a1,bool a2) // return-dupe x2
{
  long v1;
  unsigned long v2; // rax
  long *v3; // rax
  unsigned long v4;
  long *v5;
  long *v6;
  long *v7;
  long v8;
  
  v6 = a1->field_0x0;
  if (a1->field_0x8 <= v6)
    return 1;
label_8196:
  do {
    v8 = *v6;
    if (v8) {
      if ((long *)v6[1]) {
        v4 = a0->field_0x10;
        v7 = (long *)v6[1];
        while( true ) {
          v8 = *v7;
          v2 = (*a0->field_0x30)(v8,v4);
          v4 = a0->field_0x10;
          if (v4 <= v2) {
            abort(); // no-return, return-dupe
          }
          v3 = (long *)(v2 * 0x10 + a0->field_0x0);
          v5 = (long *)v7[1];
          if (*v3) { // branch-flip
            v7[1] = v3[1];
            v3[1] = (long)v7;
          }
          else {
            *v3 = v8;
            v8 = a0->field_0x48;
            a0->field_0x18 = a0->field_0x18 + 1;
            *v7 = 0;
            v7[1] = v8;
            a0->field_0x48 = (long)v7;
          }
          if (!v5) break;
          v7 = v5;
        }
        v8 = *v6;
      }
      v6[1] = 0;
      if (!a2) {
        v4 = (*a0->field_0x30)(v8,a0->field_0x10);
        if ((unsigned long)a0->field_0x10 <= v4)
          abort();
        v7 = (long *)(v4 * 0x10 + a0->field_0x0);
        if (*v7) { // branch-flip
          v5 = (long *)a0->field_0x48;
          if (v5) // branch-flip
            a0->field_0x48 = v5[1];
          else {
            v5 = malloc(0x10);
            if (!v5)
              return 0;
          }
          v1 = v7[1];
          *v5 = v8;
          v5[1] = v1;
          v7[1] = (long)v5;
        }
        else {
          *v7 = v8;
          a0->field_0x18 = a0->field_0x18 + 1;
        }
        *v6 = 0;
        v6 = &v6[2];
        a1->field_0x18 = a1->field_0x18 + -1;
        if (a1->field_0x8 <= v6)
          return 1;
        goto label_8196;
      }
    }
    v6 = &v6[2];
    if (a1->field_0x8 <= v6)
      return 1;
  } while( true );
}

// Function: sub_82d0 @ 0x82d0
unsigned long sub_82d0(long a0)
{
  return *(unsigned long *)(a0 + 0x10);
}

// Function: sub_82e0 @ 0x82e0
unsigned long sub_82e0(long a0)
{
  return *(unsigned long *)(a0 + 0x18);
}

// Function: sub_82f0 @ 0x82f0
unsigned long sub_82f0(long a0)
{
  return *(unsigned long *)(a0 + 0x20);
}

// Function: sub_8300 @ 0x8300
unsigned long sub_8300(struct_10 *a0)
{
  long v1;
  long *v2;
  unsigned long v3;
  unsigned long v4;
  
  v2 = a0->field_0x0;
  v4 = 0;
  do {
    if (a0->field_0x8 <= v2)
      return v4;
    while (*v2) {
      v3 = 1;
      for (v1 = v2[1]; v1; v1 = *(long *)(v1 + 8)) {
        v3 += 1;
      }
      if (v4 < v3)
        v4 = v3;
      v2 = &v2[2];
      if (a0->field_0x8 <= v2)
        return v4;
    }
    v2 = &v2[2];
  } while( true );
}

// Function: sub_8360 @ 0x8360
bool sub_8360(struct_10 *a0)
{
  long v1;
  long *v2;
  long v3;
  long v4;
  
  v2 = a0->field_0x0;
  v3 = 0;
  v4 = 0;
  do {
    while( true ) {
      if (a0->field_0x8 <= v2) {
        if (a0->field_0x18 != v4)
          return 0;
        return a0->field_0x20 == v3;
      }
      if (*v2) break;
label_8380:
      v2 = &v2[2];
    }
    v1 = v2[1];
    v4 += 1;
    v3 += 1;
    if (!v1) goto label_8380;
    do {
      v1 = *(long *)(v1 + 8);
      v3 += 1;
    } while (v1);
    v2 = &v2[2];
  } while( true );
}

// Function: sub_83d0 @ 0x83d0
void sub_83d0(unsigned long *a0,FILE *a1)
{
  unsigned long v1;
  unsigned long v2;
  long v3;
  long *v4;
  unsigned long v5;
  unsigned long v6;
  
  v6 = 0;
  v1 = a0[2];
  v2 = a0[3];
  for (v4 = (long *)*a0; v4 < (long *)a0[1]; v4 = &v4[2]) {
    while (!*v4) {
      v4 = &v4[2];
      if ((long *)a0[1] <= v4) goto label_843d;
    }
    v5 = 1;
    for (v3 = v4[1]; v3; v3 = *(long *)(v3 + 8)) {
      v5 += 1;
    }
    if (v6 < v5)
      v6 = v5;
  }
label_843d:
  __fprintf_chk(a1,1,"# entries:         %lu\n",a0[4]);
  __fprintf_chk(a1,1,"# buckets:         %lu\n",v1);
  __fprintf_chk(a1,1,"# buckets used:    %lu (%.2f%%)\n",v2,((double)v2 * dat_e5c0) / (double)v1);
  __fprintf_chk(a1,1,"max bucket length: %lu\n",v6); // tail-call
  return;
}

// Function: sub_8530 @ 0x8530
long sub_8530(struct_3 *a0,long a1) // return-dupe
{
  void *v1;
  unsigned long v2; // rax
  long *v3;
  long v4;
  
  v2 = (*a0->field_0x30)(a1,a0->field_0x10);
  if ((unsigned long)a0->field_0x10 <= v2)
    abort(); // no-return
  v3 = (long *)(v2 * 0x10 + a0->field_0x0);
  v4 = *v3;
  if (v4) {
    while( true ) {
      if (v4 == a1)
        return a1;
      v1 = a0->field_0x38;
      if ((*v1)(a1)) break;
      v3 = (long *)v3[1];
      if (!v3)
        return 0;
      v4 = *v3;
    }
    return *v3;
  }
  return 0;
}

// Function: sub_85a0 @ 0x85a0
long sub_85a0(struct_10 *a0)
{
  long *v1;
  
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

// Function: sub_85f0 @ 0x85f0
long sub_85f0(struct_11 *a0,long a1)
{
  long v1;
  unsigned long v2; // rax
  long *v3; // rax
  long *v4;
  
  v2 = (*a0->field_0x30)(a1,a0->field_0x10);
  if ((unsigned long)a0->field_0x10 <= v2)
    abort(); // no-return
  v3 = (long *)(v2 * 0x10 + a0->field_0x0);
  v4 = v3;
  do {
    v1 = *v4;
    v4 = (long *)v4[1];
    if (v1 == a1) {
      if (v4)
        return *v4;
      break;
    }
  } while (v4);
  do {
    v3 = &v3[2];
    if (a0->field_0x8 <= v3)
      return 0;
  } while (!*v3);
  return *v3;
}

// Function: sub_8680 @ 0x8680
void sub_8680(struct_10 *a0,long a1,unsigned long a2)
{
  unsigned long v1; // rax
  long *v2;
  long *v3;
  
  v3 = a0->field_0x0;
  v1 = 0;
  if (a0->field_0x8 <= v3)
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
  } while (v3 < a0->field_0x8);
}

// Function: sub_86d0 @ 0x86d0
long sub_86d0(struct_10 *a0,void *a1,unsigned long a2)
{
  long *v1;
  long v2;
  long v3; // r12
  long *v4;
  
  v4 = a0->field_0x0;
  if (a0->field_0x8 <= v4) // branch-flip
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
    } while (v4 < a0->field_0x8);
  }
  return v3;
}

// Function: sub_8750 @ 0x8750
unsigned long sub_8750(unsigned char *a0,unsigned long a1)
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

// Function: sub_8790 @ 0x8790
void sub_8790(struct_12 *a0)
{
  a0->field_0x10 = 0;
  a0->field_0x0 = 0x3f80000000000000;
  a0->field_0x8 = 0x3fb4fdf43f4ccccd;
}

// Function: sub_87c0 @ 0x87c0
unsigned long * sub_87c0(unsigned long a0,long a1,void *a2,void *a3,unsigned long a4) // return-dupe
{
  unsigned long *v1; // rax
  unsigned long v2; // rax
  void *v3; // rax
  
  if (!a2)
    a2 = sub_7e50;
  if (!a3)
    a3 = sub_7e70;
  v1 = malloc(0x50);
  if (!v1)
    return NULL;
  if (!a1)
    a1 = 0xe590;
  v1[5] = a1;
  if (sub_7e80(v1)) {
    v2 = sub_8020(*(float *)(a1 + 8),a0,(unsigned char)*(char *)(a1 + 0x10));
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

// Function: sub_88d0 @ 0x88d0
void sub_88d0(struct_13 *a0)
{
  unsigned long *v1;
  unsigned long v2;
  void *v3;
  unsigned long *v4;
  long *v5;
  long *v6; // r12
  
  v5 = a0->field_0x0;
  if (a0->field_0x0 < a0->field_0x8) {
    do {
      while (!*v5) {
        v5 = &v5[2];
        if (a0->field_0x8 <= v5) goto label_896b;
      }
      v3 = a0->field_0x40;
      v4 = (unsigned long *)v5[1];
      if ((unsigned long *)v5[1]) {
        while( true ) {
          if (v3) {
            (*v3)(*v4);
            v3 = a0->field_0x40;
          }
          v1 = (unsigned long *)v4[1];
          v2 = a0->field_0x48;
          *v4 = 0;
          v4[1] = v2;
          a0->field_0x48 = v4;
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
    } while (v6 < a0->field_0x8);
  }
label_896b:
  a0->field_0x18 = 0;
  a0->field_0x20 = 0;
  return;
}

// Function: sub_8980 @ 0x8980
void sub_8980(struct_4 *a0)
{
  void *v1;
  void *v2;
  long *v3;
  long *v4;
  long v5;
  
  v4 = a0->field_0x0;
  v3 = a0->field_0x8;
  if ((a0->field_0x40) && (a0->field_0x20)) {
    if (v3 <= v4) goto label_8a2c;
    do {
      while (v5 = *v4, !v5) {
        v4 = &v4[2];
        if (v3 <= v4) goto label_89ef;
      }
      v3 = v4;
      while( true ) {
        (*(void *)a0->field_0x40)(v5);
        v3 = (long *)v3[1];
        if (!v3) break;
        v5 = *v3;
      }
      v3 = a0->field_0x8;
      v4 = &v4[2];
    } while (v4 < v3);
label_89ef:
    v4 = a0->field_0x0;
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
    } while (v4 < a0->field_0x8);
  }
label_8a2c:
  v2 = a0->field_0x48;
  while (v2) {
    v1 = *(void **)((long)v2 + 8);
    free(v2);
    v2 = v1;
  }
  free(a0->field_0x0);
  free(a0); // tail-call
}

// Function: sub_8a70 @ 0x8a70
unsigned int sub_8a70(struct_8 *a0,unsigned long a1) // return-dupe
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
  
  v4 = sub_8020(*(float *)(a0->field_0x28 + 8),a1,(unsigned char)*(char *)(a0->field_0x28 + 0x10));
  if (!v4)
    return 0;
  if (a0->field_0x10 == v4)
    return 1;
  v2 = calloc(v4,0x10);
  if (!v2)
    return 0;
  v3 = (void *)((long)v2 + v4 * 0x10);
  v5 = 0;
  v7 = a0->field_0x28;
  v6 = 0;
  v8 = a0->field_0x30;
  v9 = a0->field_0x38;
  v10 = a0->field_0x40;
  v11 = a0->field_0x48;
  v1 = sub_8160(&v2,a0,0);
  if (!(char)v1) {
    a0->field_0x48 = v11;
    if ((sub_8160(a0,&v2,1)) && (sub_8160(a0,&v2,0))) {
      free(v2);
      return v1;
    }
    abort(); // no-return
  }
  free(a0->field_0x0);
  a0->field_0x0 = v2;
  a0->field_0x8 = v3;
  a0->field_0x10 = v4;
  a0->field_0x18 = v5;
  a0->field_0x48 = v11;
  return v1;
}

// Function: sub_8bf0 @ 0x8bf0
long * sub_8bf0(long a0,long a1,long *a2) // return-dupe, ternary
{
  long v1;
  long *v2; // rax
  long *v3; // stack - 0x38
  unsigned long v4;
  float v5; // xmm4_da
  float v6; // xmm5_da
  
  if (!a1) {
    abort(); // no-return, return-dupe
  }
  v2 = (long)sub_7f10(a0,a1,&v3,0);
  if (v2) {
    if (!a2) {
      v2 = NULL;
      return v2;
    }
    *a2 = (long)v2;
    v2 = NULL;
    return v2;
  }
  v4 = *(unsigned long *)(a0 + 0x18);
  if (0 <= (long)v4) { // branch-flip
    v6 = (float)(long)v4;
    v2 = *(long **)(a0 + 0x10);
    if ((long)v2 > -1) goto label_8c7f;
label_8d06:
    v5 = (float)v2;
  }
  else {
    v2 = *(long **)(a0 + 0x10);
    v6 = (float)v4;
    if (0 > (long)v2) goto label_8d06;
label_8c7f:
    v5 = (float)(long)v2;
  }
  if (*(float *)(*(long *)(a0 + 0x28) + 8) * v5 < v6) {
    sub_7e80(a0);
    v1 = *(long *)(a0 + 0x28);
    if (v5 * *(float *)(v1 + 8) < v6) {
      v5 = v5 * *(float *)(v1 + 0xc);
      if (!*(char *)(v1 + 0x10))
        v5 = v5 * *(float *)(v1 + 8);
      if (dat_e5b4 <= v5) {
        v2 = (long *)0xffffffff;
        return v2;
      }
      v4 = (dat_e5b8 <= v5) ? (long)(v5 - dat_e5b8) ^ 0x8000000000000000 : (unsigned long)v5;
      if (!sub_8a70(a0,v4)) {
        v2 = (long *)0xffffffff;
        return v2;
      }
      v2 = (long)sub_7f10(a0,a1,&v3,0);
      if (v2)
        abort();
    }
  }
  if (!*v3) {
    *v3 = a1;
    *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + 1;
    *(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + 1;
    v2 = (long *)0x1;
    return v2;
  }
  v2 = *(long **)(a0 + 0x48);
  if (v2) // branch-flip
    *(long *)(a0 + 0x48) = v2[1];
  else {
    v2 = malloc(0x10);
    if (!v2) {
      v2 = (long *)0xffffffff;
      return v2;
    }
  }
  v1 = v3[1];
  *v2 = a1;
  v2[1] = v1;
  v3[1] = (long)v2;
  *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + 1;
  v2 = (long *)0x1;
  return v2;
}

// Function: sub_8e20 @ 0x8e20
unsigned long sub_8e20(long a0,unsigned long a1) // early-return
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x18
  bool v4; // zf
  
  v1 = sub_8bf0(a0,a1,&v3);
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

// Function: sub_8e80 @ 0x8e80
long sub_8e80(long a0,long a1) // ternary x2
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
  
  v5 = sub_7f10(a0,a1,&v7,1);
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
      sub_7e80(a0);
      v2 = *(float **)(a0 + 0x28);
      if (v9 < *v2 * v8) {
        v8 = v8 * v2[1];
        if (!*(char *)&v2[4])
          v8 = v8 * v2[2];
        v6 = (dat_e5b8 <= v8) ? (long)(v8 - dat_e5b8) ^ 0x8000000000000000 : (unsigned long)v8;
        if (!sub_8a70(a0,v6)) {
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

// Function: sub_9010 @ 0x9010
long sub_9010(long a0)
{
  return sub_8e80(a0); // tail-call
}

// Function: sub_9020 @ 0x9020
char * sub_9020(unsigned long a0,long a1)
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

// Function: sub_90c0 @ 0x90c0
unsigned long sub_90c0(void)
{
  char *v1; // rax
  long v2; // rax
  char *v3; // stack - 0x18
  unsigned long v4;
  
  v4 = 0x31069;
  v1 = getenv("_POSIX2_VERSION");
  if (((v1) && (*v1)) && (v2 = strtol(v1,&v3,10), !*v3)) {
    if (-0x80000000 <= v2) { // branch-flip
      v4 = 0x7fffffff;
      if (v2 <= 0x7fffffff)
        v4 = v2;
    }
    else {
      v4 = 0x80000000;
    }
  }
  return v4 & 0xffffffff;
}

// Function: sub_9150 @ 0x9150
void sub_9150(char *a0)
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
    dat_13138 = a0;
    program_invocation_name = a0;
    return;
  }
  fputs("A NULL argv[0] was passed through an exec system call.\n",stderr);
  abort(); // no-return
}

// Function: sub_91f0 @ 0x91f0
char * sub_91f0(char *a0,int a1) // return-dupe x3
{
  unsigned char *v1; // rax
  
  v1 = (unsigned char *)sub_cc60();
  if ((*v1 & 0xdf) != 0x55) { // branch-flip
    if (((((*v1 & 0xdf) == 0x47) && ((v1[1] & 0xdf) == 0x42)) && (v1[2] == 0x31)) && (((v1[3] == 0x38 && (v1[4] == 0x30)) && ((v1[5] == 0x33 && ((v1[6] == 0x30 && (!v1[7])))))))) {
      if (*a0 == '`')
        return (char *)0xe627;
      return (char *)0xe620;
    }
  }
  else if (((((v1[1] & 0xdf) == 0x54) && ((v1[2] & 0xdf) == 0x46)) && (v1[3] == 0x2d)) && ((v1[4] == 0x38 && (!v1[5])))) {
    if (*a0 == '`')
      return "‘";
    return "’";
  }
  if (a1 == 9)
    return "\"";
  return "\'";
}

// Function: sub_92d0 @ 0x92d0
long * sub_92d0(char *a0,unsigned long a1,char *a2,unsigned long a3,int a4,unsigned int a5,long a6,char *a7,char *a8)
{
  unsigned long v1;
  long *v10; // rax
  char v11;
  unsigned char *v12;
  unsigned long v13;
  unsigned long v14;
  mbstate_t v15; // stack - 0x48
  unsigned int v16; // stack - 0x4c
  unsigned char v17;
  unsigned int v18;
  unsigned long v19;
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
  bool v5;
  long v6;
  long v7;
  unsigned char v8;
  unsigned int v9; // eax
  
  v29 = a6;
  v36 = a7;
  v35 = a8;
  v34 = a5;
label_9340:
  v10 = (unsigned long)__ctype_get_mb_cur_max();
  v33 = (unsigned long)v10;
  v18 = v34 & 2;
  switch(a4) {
    case 0:
      v25 = 0;
      v2 = 0;
      v13 = 0;
      v30 = 0;
      v31 = NULL;
      break;
    case 1:
label_97e6:
      v2 = 0;
      goto label_97a0;
    case 2:
      if (!v18) {
        v2 = 0;
        goto label_9901;
      }
      v25 = 1;
      v2 = 0;
      v13 = 0;
      v30 = 1;
      v31 = "\'";
      break;
    case 3:
      v2 = 1;
label_97a0:
      v25 = 1;
      v13 = 0;
      a4 = 2;
      v30 = 1;
      v31 = "\'";
      break;
    case 4:
      if (!v18) {
        v2 = 1;
        goto label_9901;
      }
      goto label_97e6;
    case 5:
      if (!v18) goto label_9741;
      v25 = 1;
      v2 = 1;
      v13 = 0;
      v30 = 1;
      v31 = "\"";
      break;
    case 6:
      v25 = 1;
      v13 = 0;
      v2 = 1;
      v30 = 1;
      a4 = 5;
      v31 = "\"";
      break;
    case 7:
      v25 = 0;
      v2 = 1;
      v13 = 0;
      v30 = 0;
      v31 = NULL;
      break;
    case 8:
    case 9:
    case 10:
      if (a4 != 10) {
        v10 = (char *)dcgettext(NULL,"`",5);
        v36 = (char *)v10;
        if (v10 == (long *)"`") {
          v10 = (char *)sub_91f0("`",a4);
          v36 = (char *)v10;
        }
        v10 = (char *)dcgettext(NULL,"\'",5);
        v35 = (char *)v10;
        if (v10 == (long *)"\'") {
          v10 = (char *)sub_91f0("\'",a4);
          v35 = (char *)v10;
        }
      }
      v13 = 0;
      if (!v18) {
        v11 = *v36;
        while (v11) {
          if (v13 < a1)
            a0[v13] = v11;
          v13 += 1;
          v11 = v36[v13];
        }
      }
      v25 = v18 != 0;
      v2 = 1;
      v10 = (unsigned long)strlen(v35);
      v31 = v35;
      v30 = (unsigned long)v10;
      break;
    default:
      abort(); // no-return
    
  }
  v4 = 1;
  v20 = 0;
  v32 = 0;
  v14 = a1;
  v3 = 0;
label_942b:
  do {
label_943e:
    v23 = 0;
    a1 = v14;
label_9448:
    v14 = v32;
    v26 = a3 != v23;
    if (a3 == 0xffffffffffffffff)
      v26 = a2[v23] != '\0';
    if (v26) {
      v12 = (unsigned char *)&a2[v23];
      v22 = (bool)(a4 != 2 & v2);
      v26 = 1;
      v14 = v13;
      v27 = v2;
      v28 = v2;
      if (v22) {
        v24 = 1;
        if (v30) {
          if ((a3 == 0xffffffffffffffff) && (2 <= v30)) {
            v10 = (unsigned long)strlen(a2);
            a3 = (unsigned long)v10;
          }
          if ((a3 < v23 + v30) || (v9 = memcmp(v12,v31,v30), v9)) goto label_9db0;
          if (!v25) {
            v8 = *v12;
            v19 = (unsigned long)v8;
            v21 = v19;
            if ((char)v8 <= '?') {
              switch(v8) {
                case 0:
                  goto label_99ac;
                default:
                  goto label_9683;
                case 7:
                  goto label_9997;
                case 8:
                  goto label_9966;
                case 9:
                  goto label_9a33;
                case 10:
                  goto label_998d;
                case 0xb:
                  goto label_9a60;
                case 0xc:
                  goto label_994b;
                case 0xd:
                  goto label_989d;
                case 0x20:
                  goto label_9a6d;
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
                  goto label_9636;
                case 0x23:
                  goto label_9a10;
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
                  goto label_9568;
                case 0x27:
                  goto label_9882;
                case 0x3f:
                  goto label_9841;
                
              }
            }
            v5 = v22;
            if ((char)v8 <= 'z') {
              if (v8 == 0x40) goto label_9683;
              v10 = (unsigned long)(1L << (v8 + 0xbf & 0x3f));
              if ((unsigned long)v10 & 0x3ffffff53ffffff) goto label_9568;
              if (!((unsigned long)v10 & 0xa4000000)) goto label_9e54;
              goto label_9d98;
            }
            if (v8 == 0x7d) goto label_9f20;
            if ((char)v8 <= '}') {
              v21 = 0x7b;
              if (v8 == 0x7b) goto label_9d7a;
              v21 = 0x7c;
              if (v8 == 0x7c) goto label_9d98;
              goto label_9683;
            }
            if (v8 != 0x7e) goto label_9683;
label_9ed4:
            if (!v23) {
              v8 = a4 == 2;
              v21 = 0x7e;
              v26 = v3;
              goto label_96f9;
            }
            v21 = 0x7e;
            goto label_9ee2;
          }
          goto label_966c;
        }
label_9db0:
        v8 = *v12;
        v19 = (unsigned long)v8;
        v21 = v19;
        if ('?' < (char)v8) {
          if ('z' < (char)v8) {
            if (v8 != 0x7d) { // branch-flip
              if ('~' <= (char)v8) {
                v22 = 0;
                if (v8 == 0x7e) goto label_9ed4;
                goto label_9680;
              }
              v21 = 0x7b;
              v22 = 0;
              if (v8 != 0x7b) {
                v21 = 0x7c;
                v22 = 0;
                v5 = 0;
                if (v8 != 0x7c) goto label_9683;
                goto label_9d98;
              }
            }
            else {
              v21 = 0x7d;
              v22 = 0;
            }
label_9d7a:
            if (a3 == 0xffffffffffffffff) goto label_9f2f;
label_9d88:
            v5 = v22;
            if (a3 == 1) goto label_9a15;
label_9d98:
            v8 = a4 == 2;
            v26 = v3;
            v24 = 0;
            v22 = v5;
            goto label_96f9;
          }
          if (v8 == 0x40) goto label_9680;
          v10 = (unsigned long)(1L << (v8 + 0xbf & 0x3f));
          v22 = 0;
          if ((unsigned long)v10 & 0x3ffffff53ffffff) goto label_9568;
          v22 = 0;
          v5 = 0;
          if ((unsigned long)v10 & 0xa4000000) goto label_9d98;
label_9e54:
          if ((char)v19 == '\\') {
            if (((bool)(v2 & v25)) && (v30)) goto label_9e7a;
            v17 = 0x5c;
            goto label_98a2;
          }
label_9683:
          v17 = (unsigned char)v19;
          if (v33 != 1) goto label_9c03;
label_9690:
          v10 = __ctype_b_loc();
          v21 = 1;
          v26 = (*(unsigned char *)(*v10 + 1 + v19 * 2) & 0x40) == 0;
          v27 = (bool)(v26 & v2);
          v26 = !v26;
          goto label_96de;
        }
        switch(v8) {
          case 0:
            goto label_999e;
          default:
label_9680:
            v22 = 0;
            goto label_9683;
          case 7:
label_9997:
            v17 = 0x61;
            goto label_9977;
          case 8:
label_9966:
            v17 = 0x62;
            goto label_9977;
          case 9:
            v22 = 0;
label_9a33:
            v21 = 9;
            v17 = 0x74;
            goto label_9a40;
          case 10:
label_998d:
            v17 = 0x6e;
            break;
          case 0xb:
label_9a60:
            v17 = 0x76;
            break;
          case 0xc:
label_994b:
            v17 = 0x66;
label_9977:
            if (v25) {
              v2 = 1;
              goto label_98cf;
            }
            v24 = 0;
            goto label_95ee;
          case 0xd:
label_989d:
            v17 = 0x72;
            break;
          case 0x20:
            v22 = 0;
label_9a6d:
            v21 = 0x20;
            goto label_9568;
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
            v22 = 0;
            v26 = 0;
            goto label_9639;
          case 0x23:
            v21 = 0x23;
            v22 = 0;
            goto label_9a15;
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
label_9568:
            v8 = 0;
            goto label_9578;
          case 0x27:
            v22 = 0;
            goto label_9882;
          case 0x3f:
            v22 = 0;
            goto label_9841;
          
        }
        goto label_98a2;
      }
      v17 = *v12;
      v19 = (unsigned long)v17;
      if ('?' < (char)v17) {
        if ('{' <= (char)v17) {
          if (v17 == 0x7d) {
label_9f20:
            v21 = 0x7d;
            if (a3 != 0xffffffffffffffff) goto label_9d88;
label_9f2f:
            v5 = v22;
            if (a2[1]) goto label_9d98;
            goto label_9a15;
          }
          if ((char)v17 <= '}') {
            v21 = 0x7b;
            if (v17 != 0x7b) {
              if (v17 == 0x7c) goto label_9636;
              goto label_9683;
            }
            goto label_9d7a;
          }
          v21 = 0x7e;
          if (v17 == 0x7e) goto label_9a15;
          if (v33 == 1) goto label_9690;
label_9c03:
          v15 = 0;
          if (a3 == 0xffffffffffffffff) {
            v10 = (unsigned long)strlen(a2);
            a3 = (unsigned long)v10;
          }
          v21 = 0;
          do {
            v19 = v23 + v21;
            v10 = (long)sub_cca0(&v16,&a2[v19],a3 - v19,&v15);
            v7 = (long)v10;
            v24 = v26;
            if (!v10) break;
            if (v10 == (long *)0xffffffffffffffff) {
              v24 = 0;
              goto label_a16b;
            }
            if (v10 == (long *)0xfffffffffffffffe) {
              v10 = (long *)v19;
              if (v19 >= a3) goto label_a4ef;
              goto label_a4e5;
            }
            if (((a4 == 2) && (v25)) && (v10 != (long *)0x1)) {
              v12 = (unsigned char *)&a2[v19 + 1];
              v6 = v19 + (long)v10;
              do {
                v9 = (unsigned int)*v12;
                v9 -= 0x5b;
                v18 = v9;
                v10 = (unsigned long)(unsigned long)v9;
                v19 = (unsigned long)v10;
                v8 = (unsigned char)v18;
                if ((v8 <= 0x21) && (0x20000002bU >> (v19 & 0x3f) & 1)) goto label_9d3d;
                v12 = &v12[1];
              } while ((unsigned char *)&a2[v6] != v12);
            }
            v9 = iswprint(v16);
            if (!v9)
              v26 = 0;
            v21 += v7;
            v9 = mbsinit(&v15);
            v24 = v26;
          } while (!v9);
          v27 = (bool)((v24 ^ 1U) & v2);
          goto label_a16b;
        }
        if (v17 == 0x40) goto label_9683;
        v10 = (unsigned long)(1L << (v17 + 0xbf & 0x3f));
        if ((unsigned long)v10 & 0x3ffffff53ffffff) goto label_96f0;
        if ((unsigned long)v10 & 0xa4000000) goto label_9636;
        if (v17 != 0x5c) goto label_9683;
        if (a4 == 2) {
          if (!v25) goto label_9e7a;
          goto label_9663;
        }
        if (!(bool)(v2 & v25 & v30 != 0)) {
          v21 = 0x5c;
          v17 = 0x5c;
          v22 = 0;
          goto label_9a40;
        }
label_9e7a:
        v23 += 1;
        v24 = 0;
        v17 = 0x5c;
        v8 = v20;
        goto label_9e90;
      }
      switch(v17) {
        case 0:
          if (!v2) {
            v22 = 0;
            v21 = 0;
            if (v34 & 1) goto label_9b12;
            goto label_9a4b;
          }
label_999e:
          if (!v25) {
            v22 = 0;
label_99ac:
            v27 = a4 == 2;
            v8 = v20 ^ 1;
            v8 &= v27;
            if ((bool)v8) {
              if (v13 < a1)
                a0[v13] = 0x27;
              if (v13 + 1 < a1)
                a0[v13 + 1] = 0x24;
              if (v13 + 2 < a1)
                a0[v13 + 2] = 0x27;
              v14 = v13 + 3;
              if (a1 <= v13 + 3) {
                v13 += 4;
                v24 = 0;
                v17 = 0x30;
                v20 = (bool)v8;
                goto label_9723;
              }
label_a2b5:
              a0[v14] = 0x5c;
              v13 = v14;
              v20 = (bool)v8;
            }
            else if (v13 < a1) {
              v8 = v20;
              goto label_a2b5;
            }
            v14 = v13 + 1;
            if (a4 != 2) {
              v21 = 0x30;
              if ((v23 + 1 < a3) && (v8 = a2[v23 + 1] - 0x30, v8 <= 9)) {
                if (v14 < a1)
                  a0[v14] = 0x30;
                if (v13 + 2 < a1)
                  a0[v13 + 2] = 0x30;
                v14 = v13 + 3;
                v21 = 0x30;
              }
label_a2ec:
              v17 = (unsigned char)v21;
              v24 = 0;
              v13 = v14;
              v8 = v26;
              if (!v2 || v27) goto label_9723;
              goto label_9578;
            }
            v17 = 0x30;
            v24 = 0;
            v13 = v14;
            v8 = v26;
            goto label_9723;
          }
          if (a4 != 2) goto label_966c;
          goto label_98db;
        default:
          goto label_9683;
        case 7:
          v21 = 7;
          v17 = 0x61;
          break;
        case 8:
          v21 = 8;
          v17 = 0x62;
          break;
        case 9:
          v21 = 9;
          v17 = 0x74;
          goto label_9acf;
        case 10:
          v21 = 10;
          v17 = 0x6e;
          goto label_9acf;
        case 0xb:
          v21 = 0xb;
          v17 = 0x76;
          break;
        case 0xc:
          v21 = 0xc;
          v17 = 0x66;
          break;
        case 0xd:
          v21 = 0xd;
          v17 = 0x72;
label_9acf:
          v8 = a4 == 2 & v25;
          v22 = 0;
          if (!(bool)v8) goto label_9a40;
label_9651:
          a4 = 2;
label_9663:
          if (!v28) goto label_966c;
          goto label_98db;
        case 0x20:
          v19 = 0x20;
          goto label_9639;
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
label_9636:
          v26 = 0;
label_9639:
          v21 = v19;
          v24 = v26;
          if ((a4 == 2) && (v25)) goto label_9651;
          goto label_96f7;
        case 0x23:
label_9a10:
          v21 = 0x23;
label_9a15:
          if (!v23) {
            v19 = v21;
            goto label_9639;
          }
label_9ee2:
          v8 = a4 == 2;
          v26 = v3;
          v24 = 0;
          goto label_96f9;
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
          goto label_96f0;
        case 0x27:
label_9882:
          v24 = v26;
          if (a4 != 2) {
            v8 = 0;
            v21 = 0x27;
            goto label_96f9;
          }
          if (!v25) {
            if (a1) { // branch-flip
              v14 = 0;
              v21 = a1;
              if (v32) goto label_a30b;
            }
            else {
label_a30b:
              if (v13 < a1)
                a0[v13] = 0x27;
              if (v13 + 1 < a1)
                a0[v13 + 1] = 0x5c;
              v14 = a1;
              v21 = v32;
              if (v13 + 2 < a1)
                a0[v13 + 2] = 0x27;
            }
            v13 += 3;
            v8 = 0;
            v17 = 0x27;
            a1 = v14;
            v32 = v21;
            v20 = 0;
            v3 = v26;
            goto label_9723;
          }
          goto label_9663;
        case 0x3f:
label_9841:
          if (a4 == 2) {
            if (v25) goto label_9663;
            v8 = 0;
            v17 = 0x3f;
            v24 = 0;
            goto label_9723;
          }
          if ((((a4 == 5) && (v34 & 4)) && (v19 = v23 + 2, v19 < a3)) && (a2[v23 + 1] == '?')) {
            v8 = a2[v19];
            v21 = (unsigned long)v8;
            if (v8 <= 0x3e) {
              v10 = (unsigned long)(0x7000a38200000000 >> (v8 & 0x3f));
              v1 = (unsigned long)v10 & 1;
              v8 = (unsigned char)v10 & 1;
              if (!v1) {
                v21 = 0x3f;
                v26 = v3;
                v24 = 0;
                goto label_96f9;
              }
              if (v25) goto label_966c;
              if (v13 < a1)
                a0[v13] = 0x3f;
              if (v13 + 1 < a1)
                a0[v13 + 1] = 0x22;
              if (v13 + 2 < a1)
                a0[v13 + 2] = 0x22;
              if (v13 + 3 < a1)
                a0[v13 + 3] = 0x3f;
              v14 = v13 + 4;
              v27 = 0;
              v26 = 0;
              v23 = v19;
              goto label_a2ec;
            }
          }
          v8 = 0;
          v21 = 0x3f;
          v26 = v3;
          v24 = 0;
          goto label_96f9;
        
      }
label_9a40:
      if (!v2) {
label_9a4b:
        v17 = (unsigned char)v21;
        v24 = 0;
        v8 = 0;
        v26 = v3;
        if (!v25) goto label_9720;
        goto label_9578;
      }
label_98a2:
      v24 = 0;
      goto label_98b3;
    }
    if (!(bool)(v13 == 0 & v25 & a4 == 2)) {
      v8 = a4 == 2 & (v25 ^ 1U);
      v25 = (bool)(v25 ^ 1U);
      if ((!(bool)v8) || (v25 = (bool)v8, !v3)) {
label_a5ab:
        v10 = (long *)v13;
        if (((v31) && (v25)) && (v11 = *v31, v11)) {
          do {
            if (v10 < a1)
              a0[(long)v10] = v11;
            v10 = (unsigned long)((long)v10 + 1);
            v11 = v31[(long)v10 - v13];
          } while (v11);
        }
        if (v10 < a1)
          a0[(long)v10] = 0;
        return v10;
      }
      if (!v4) {
        v8 = !a1 && v32;
        v25 = v3;
        if (a1 || !v32) goto label_a5ab;
        v23 = v32;
        goto label_a03b;
      }
      v13 = 0;
      a4 = 5;
      v10 = (unsigned long)__ctype_get_mb_cur_max();
      v30 = 1;
      v33 = (unsigned long)v10;
      v31 = "\"";
      if (!(v34 & 2)) goto label_a65b;
      v3 = 0;
      v20 = 0;
      v32 = 0;
      v2 = v4;
      v25 = v4;
      goto label_943e;
    }
label_9d3d:
    if (v2) {
label_98db:
      v2 = 1;
      v34 &= 0xfffffffd;
      v10 = (unsigned long)__ctype_get_mb_cur_max();
      v29 = 0;
      v33 = (unsigned long)v10;
    }
    else {
      v2 = 0;
      v34 &= 0xfffffffd;
      v10 = (unsigned long)__ctype_get_mb_cur_max();
      v29 = 0;
      v33 = (unsigned long)v10;
    }
label_9901:
    v25 = 0;
    a4 = 2;
    v13 = 1;
    v31 = "\'";
    v30 = 1;
    v20 = 0;
    v4 = 1;
    if (a1) { // branch-flip
      v8 = 0;
      v23 = 0;
      v32 = a1;
label_a03b:
      v13 = 1;
      *a0 = 0x27;
      a4 = 2;
      v25 = 0;
      v30 = 1;
      v31 = "\'";
      v14 = v32;
      v32 = v23;
      v3 = (bool)v8;
    }
    else {
      v32 = 0;
      v14 = a1;
      v3 = 0;
    }
  } while( true );
  while( true ) {
    v21 += 1;
    v10 = (unsigned long)(v23 + v21);
    if (a3 <= v10) break;
label_a4e5:
    if (!a2[(long)v10]) break;
  }
label_a4ef:
  v24 = 0;
label_a16b:
  v19 = (unsigned long)v17;
  v26 = v24;
  if (2 <= v21) {
label_a175:
    v26 = 0;
    v21 += v23;
    v14 = v23;
    do {
      v17 = (unsigned char)v19;
      if (v27) {
        v28 = a4 == 2;
        if (v25) goto label_9663;
        v8 = v28 & (v20 ^ 1U);
        if ((bool)v8) {
          if (v13 < a1)
            a0[v13] = 0x27;
          if (v13 + 1 < a1)
            a0[v13 + 1] = 0x24;
          if (v13 + 2 < a1)
            a0[v13 + 2] = 0x27;
          v13 += 3;
          v20 = 1;
        }
        if (v13 < a1)
          a0[v13] = 0x5c;
        if (v13 + 1 < a1) {
          v8 = (char)(v19 >> 6) + '0';
          a0[v13 + 1] = v8;
        }
        if (v13 + 2 < a1) {
          v8 = ((unsigned char)(v19 >> 3) & 7) + 0x30;
          a0[v13 + 2] = v8;
        }
        v23 = v14 + 1;
        v13 += 3;
        v17 = (v17 & 7) + 0x30;
        if (v21 <= v23) goto label_9601;
        v26 = 1;
      }
      else {
        v8 = v26 ^ 1;
        v8 &= v20;
        if (v22) {
          if (v13 < a1)
            a0[v13] = 0x5c;
          v13 += 1;
        }
        v23 = v14 + 1;
        if (v21 <= v23) {
          goto label_9e90;
        }
        if ((bool)v8) {
          if (v13 < a1)
            a0[v13] = 0x27;
          if (v13 + 1 < a1)
            a0[v13 + 1] = 0x27;
          v13 += 2;
          v22 = 0;
          v20 = 0;
        }
        else {
          v22 = 0;
        }
      }
      v14 += 1;
      if (v13 < a1)
        a0[v13] = v17;
      v19 = (unsigned long)(unsigned char)a2[v14];
      v13 += 1;
    } while( true );
  }
label_96de:
  if (v27) {
    v24 = 0;
    v27 = v2;
    goto label_a175;
  }
label_96f0:
  v21 = v19;
  v24 = v26;
label_96f7:
  v8 = a4 == 2;
  v26 = v3;
label_96f9:
  v17 = (unsigned char)v21;
  v27 = (bool)v8 == 0;
  v3 = v26;
  v8 = 0;
  if ((v27 && v2) || (v8 = 0, v25)) {
label_9578:
    v17 = (unsigned char)v21;
    v13 = v14;
    if (!v29) goto label_9723;
    if (!(*(unsigned int *)(v29 + (v21 >> 5) * 4) >> (v17 & 0x1f) & 1)) goto label_9723;
  }
  else {
label_9720:
    v8 = 0;
    v3 = v26;
label_9723:
    if (!v22) {
      v8 ^= 1;
      v23 += 1;
      v8 &= v20;
label_9e90:
      if ((bool)v8) {
        if (v13 < a1)
          a0[v13] = 0x27;
        if (v13 + 1 < a1)
          a0[v13 + 1] = 0x27;
        v20 = 0;
        v13 += 2;
      }
      goto label_9601;
    }
  }
label_98b3:
  if (v25) {
label_98cf:
    if ((bool)(v2 & a4 == 2)) goto label_98db;
label_966c:
    v34 &= 0xfffffffd;
    v29 = 0;
    goto label_9340;
  }
  v8 = v20 ^ 1;
  v8 &= a4 == 2;
  if ((bool)v8) {
    if (v13 < a1)
      a0[v13] = 0x27;
    if (v13 + 1 < a1)
      a0[v13 + 1] = 0x24;
    if (v13 + 2 < a1)
      a0[v13 + 2] = 0x27;
    v13 += 3;
    v20 = 1;
  }
label_95ee:
  if (v13 < a1)
    a0[v13] = 0x5c;
  v13 += 1;
  v23 += 1;
label_9601:
  if (v13 < a1)
    a0[v13] = v17;
  v13 += 1;
  if (!v24)
    v4 = 0;
  goto label_9448;
label_9b12:
  v23 += 1;
  goto label_9448;
label_a65b:
  a1 = v32;
label_9741:
  v25 = 0;
  a4 = 5;
  v13 = 1;
  v31 = "\"";
  v2 = 1;
  v30 = 1;
  v20 = 0;
  v3 = 0;
  v4 = 1;
  v32 = 0;
  v14 = a1;
  if (a1)
    *a0 = 0x22;
  goto label_942b;
}

// Function: sub_a760 @ 0xa760
char * sub_a760(unsigned int a0,char *a1,unsigned long a2,struct_0 *a3)
{
  unsigned long *v1;
  long v10; // stack - 0x48
  int *v11; // stack - 0x68
  int v2;
  unsigned int v3;
  unsigned long v4;
  unsigned long *v5;
  unsigned long v6;
  char *v7;
  long v8; // rdx
  unsigned int v9; // ebx
  
  v11 = __errno_location();
  v2 = *v11;
  if (a0 <= 0x7ffffffe) {
    v5 = dat_130a0;
    if (dat_13098 <= (int)a0) {
      v10 = (long)dat_13098;
      v8 = (long)(int)((a0 - dat_13098) + 1);
      if (dat_130a0 != (unsigned long *)0x130b0) // branch-flip
        v5 = (unsigned long *)sub_c000(dat_130a0,&v10,v8,0x7fffffff,0x10);
      else {
        v5 = (unsigned long *)sub_c000(NULL,&v10,v8,0x7fffffff,0x10);
        *v5 = dat_130b0;
        v5[1] = dat_130b8;
      }
      dat_130a0 = v5;
      memset(&v5[(long)dat_13098 * 2],0,(v10 - dat_13098) * 0x10);
      dat_13098 = (int)v10;
    }
    v1 = &v5[(long)(int)a0 * 2];
    v4 = *v1;
    v7 = (char *)v1[1];
    v9 = a3->field_0x4 | 1;
    v6 = sub_92d0(v7,v4,a1,a2,a3->field_0x0,v9,a3->field_0x8,a3->field_0x28,a3->field_0x30);
    if (v4 <= v6) {
      v6 += 1;
      *v1 = v6;
      if (v7 != (char *)0x13140)
        free(v7);
      v7 = (char *)sub_bd60(v6);
      v3 = a3->field_0x0;
      v1[1] = (unsigned long)v7;
      sub_92d0(v7,v6,a1,a2,v3,v9,a3->field_0x8,a3->field_0x28,a3->field_0x30);
    }
    *v11 = v2;
    return v7;
  }
  abort(); // no-return
}

// Function: sub_a950 @ 0xa950
void sub_a950(void *a0)
{
  int v1;
  int *v2; // rax
  
  v2 = __errno_location();
  v1 = *v2;
  if (!a0)
    a0 = (void *)0x13240;
  sub_c160(a0,0x38);
  *v2 = v1;
}

// Function: sub_a990 @ 0xa990
unsigned int sub_a990(unsigned int *a0)
{
  if (!a0)
    a0 = (unsigned int *)0x13240;
  return *a0;
}

// Function: sub_a9b0 @ 0xa9b0
void sub_a9b0(unsigned int *a0,unsigned int a1)
{
  if (!a0)
    a0 = (unsigned int *)0x13240;
  *a0 = a1;
}

// Function: sub_a9d0 @ 0xa9d0
unsigned int sub_a9d0(long a0,unsigned char a1,unsigned int a2)
{
  unsigned int *v1;
  unsigned int v2;
  unsigned int v3; // eax
  
  if (!a0)
    a0 = 0x13240;
  v1 = (unsigned int *)(a0 + 8 + (unsigned long)(a1 >> 5) * 4);
  v2 = *v1;
  v3 = v2 >> (a1 & 0x1f);
  *v1 = ((a2 ^ v3) & 1) << (a1 & 0x1f) ^ v2;
  return v3 & 1;
}

// Function: sub_aa10 @ 0xaa10
unsigned int sub_aa10(long a0,unsigned int a1)
{
  unsigned int v1;
  
  if (!a0)
    a0 = 0x13240;
  v1 = *(unsigned int *)(a0 + 4);
  *(unsigned int *)(a0 + 4) = a1;
  return v1;
}

// Function: sub_aa30 @ 0xaa30
void sub_aa30(unsigned int *a0,long a1,long a2)
{
  if (!a0)
    a0 = (unsigned int *)0x13240;
  *a0 = 10;
  if ((a1) && (a2)) {
    *(long *)&a0[10] = a1;
    *(long *)&a0[0xc] = a2;
    return;
  }
  abort(); // no-return
}

// Function: sub_aa70 @ 0xaa70
void sub_aa70(char *a0,unsigned long a1,char *a2,unsigned long a3,unsigned int *a4)
{
  int v1;
  int *v2; // rax
  
  if (!a4)
    a4 = (unsigned int *)0x13240;
  v2 = __errno_location();
  v1 = *v2;
  sub_92d0(a0,a1,a2,a3,*a4,a4[1],&a4[2],*(char **)&a4[10],*(char **)&a4[0xc]);
  *v2 = v1;
}

// Function: sub_aaf0 @ 0xaaf0
char * sub_aaf0(char *a0,unsigned long a1,long *a2,unsigned int *a3)
{
  int v1;
  int *v2; // rax
  long v3; // rax
  char *v4; // rax
  unsigned int v5; // r9d
  
  if (!a3)
    a3 = (unsigned int *)0x13240;
  v2 = __errno_location();
  v1 = *v2;
  v5 = (unsigned int)(a2 == NULL) | a3[1];
  v3 = sub_92d0(NULL,0,a0,a1,*a3,v5,&a3[2],*(char **)&a3[10],*(char **)&a3[0xc]);
  v4 = (char *)sub_bd60(v3 + 1U);
  sub_92d0(v4,v3 + 1U,a0,a1,*a3,v5,&a3[2],*(char **)&a3[10],*(char **)&a3[0xc]);
  *v2 = v1;
  if (a2)
    *a2 = v3;
  return v4;
}

// Function: sub_abe0 @ 0xabe0
char * sub_abe0(char *a0,unsigned long a1,unsigned int *a2)
{
  return sub_aaf0(a0,a1,NULL,a2); // tail-call
}

// Function: sub_abf0 @ 0xabf0
void sub_abf0(void) // return-dupe
{
  unsigned long *v1;
  void *v2;
  void *v3;
  unsigned long *v4;
  
  v3 = dat_130a0;
  if (2 <= dat_13098) {
    v4 = (unsigned long *)((long)dat_130a0 + 0x18);
    v1 = (unsigned long *)((long)dat_130a0 + (unsigned long)(unsigned int)(dat_13098 - 2) * 0x10 + 0x28);
    do {
      v2 = (void *)*v4;
      v4 = &v4[2];
      free(v2);
    } while (v4 != v1);
  }
  if (*(void **)((long)v3 + 8) != (void *)0x13140) {
    free(*(void **)((long)v3 + 8));
    dat_130b8 = 0x13140;
    dat_130b0 = 0x100;
  }
  if (v3 == (void *)0x130b0) {
    dat_13098 = 1;
    return;
  }
  free(v3);
  dat_13098 = 1;
  dat_130a0 = (void *)0x130b0;
}

// Function: sub_ac90 @ 0xac90
char * sub_ac90(unsigned int a0,char *a1)
{
  return sub_a760(a0,a1,0xffffffffffffffff,(struct_0 *)0x13240); // tail-call
}

// Function: sub_acb0 @ 0xacb0
char * sub_acb0(unsigned int a0,char *a1,unsigned long a2)
{
  return sub_a760(a0,a1,a2); // tail-call
}

// Function: sub_acc0 @ 0xacc0
char * sub_acc0(char *a0)
{
  return sub_a760(0,a0,0xffffffffffffffff,(struct_0 *)0x13240); // tail-call
}

// Function: sub_ace0 @ 0xace0
char * sub_ace0(char *a0,unsigned long a1)
{
  return sub_a760(0,a0,a1,(struct_0 *)0x13240); // tail-call
}

// Function: sub_ad00 @ 0xad00
void sub_ad00(unsigned int a0,int a1,char *a2)
{
  int v1 [14]; // stack - 0x48
  
  if (a1 != 10) {
    v1[1] = 0;
    v1[2] = 0;
    v1[3] = 0;
    v1[4] = 0;
    v1[5] = 0;
    v1[6] = 0;
    v1[7] = 0;
    v1[8] = 0;
    v1[9] = 0;
    v1[10] = 0;
    v1[0xb] = 0;
    v1[0xc] = 0;
    v1[0xd] = 0;
    v1[0] = a1;
    sub_a760(a0,a2,0xffffffffffffffff,v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_ada0 @ 0xada0
void sub_ada0(unsigned int a0,int a1,char *a2,unsigned long a3)
{
  int v1 [14]; // stack - 0x48
  
  if (a1 != 10) {
    v1[1] = 0;
    v1[2] = 0;
    v1[3] = 0;
    v1[4] = 0;
    v1[5] = 0;
    v1[6] = 0;
    v1[7] = 0;
    v1[8] = 0;
    v1[9] = 0;
    v1[10] = 0;
    v1[0xb] = 0;
    v1[0xc] = 0;
    v1[0xd] = 0;
    v1[0] = a1;
    sub_a760(a0,a2,a3,v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_ae30 @ 0xae30
void sub_ae30(int a0,char *a1)
{
  int v1 [14]; // stack - 0x48
  
  if (a0 != 10) {
    v1[1] = 0;
    v1[2] = 0;
    v1[3] = 0;
    v1[4] = 0;
    v1[5] = 0;
    v1[6] = 0;
    v1[7] = 0;
    v1[8] = 0;
    v1[9] = 0;
    v1[10] = 0;
    v1[0xb] = 0;
    v1[0xc] = 0;
    v1[0xd] = 0;
    v1[0] = a0;
    sub_a760(0,a1,0xffffffffffffffff,v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_aec0 @ 0xaec0
void sub_aec0(int a0,char *a1,unsigned long a2)
{
  int v1 [14]; // stack - 0x48
  
  if (a0 != 10) {
    v1[1] = 0;
    v1[2] = 0;
    v1[3] = 0;
    v1[4] = 0;
    v1[5] = 0;
    v1[6] = 0;
    v1[7] = 0;
    v1[8] = 0;
    v1[9] = 0;
    v1[10] = 0;
    v1[0xb] = 0;
    v1[0xc] = 0;
    v1[0xd] = 0;
    v1[0] = a0;
    sub_a760(0,a1,a2,v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_af50 @ 0xaf50
void sub_af50(char *a0,unsigned long a1,unsigned char a2)
{
  unsigned int *v1;
  unsigned int v2;
  unsigned long v3; // stack - 0x48
  unsigned long v4; // stack - 0x40
  unsigned long v5; // stack - 0x38
  unsigned long v6; // stack - 0x30
  unsigned long v7; // stack - 0x28
  unsigned long v8; // stack - 0x20
  unsigned long v9; // stack - 0x18
  
  v3 = dat_13240;
  v4 = dat_13248;
  v9 = dat_13270;
  v5 = dat_13250;
  v6 = dat_13258;
  v7 = dat_13260;
  v8 = dat_13268;
  v1 = (unsigned int *)((long)&v4 + (unsigned long)(a2 >> 5) * 4);
  v2 = *v1;
  *v1 = (~(v2 >> (a2 & 0x1f)) & 1) << (a2 & 0x1f) ^ v2;
  sub_a760(0,a0,a1,&v3);
}

// Function: sub_aff0 @ 0xaff0
void sub_aff0(char *a0,unsigned char a1)
{
  unsigned int *v1;
  unsigned int v2;
  unsigned long v3; // stack - 0x48
  unsigned long v4; // stack - 0x40
  unsigned long v5; // stack - 0x38
  unsigned long v6; // stack - 0x30
  unsigned long v7; // stack - 0x28
  unsigned long v8; // stack - 0x20
  unsigned long v9; // stack - 0x18
  
  v3 = dat_13240;
  v4 = dat_13248;
  v9 = dat_13270;
  v5 = dat_13250;
  v6 = dat_13258;
  v7 = dat_13260;
  v8 = dat_13268;
  v1 = (unsigned int *)((long)&v4 + (unsigned long)(a1 >> 5) * 4);
  v2 = *v1;
  *v1 = (~(v2 >> (a1 & 0x1f)) & 1) << (a1 & 0x1f) ^ v2;
  sub_a760(0,a0,0xffffffffffffffff,&v3);
}

// Function: sub_b090 @ 0xb090
void sub_b090(char *a0)
{
  unsigned long v1; // stack - 0x48
  unsigned long v2; // stack - 0x40
  unsigned int v3; // stack - 0x3c
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  unsigned long v7; // stack - 0x20
  unsigned long v8; // stack - 0x18
  
  v1 = dat_13240;
  v3 = (unsigned int)((unsigned long)dat_13248 >> 0x20);
  v8 = dat_13270;
  v4 = dat_13250;
  v5 = dat_13258;
  v6 = dat_13260;
  v7 = dat_13268;
  v2 = (unsigned int)dat_13248;
  v2 = CONCAT44(~v3 & 0x4000000 ^ v3,v2);
  sub_a760(0,a0,0xffffffffffffffff,&v1);
}

// Function: sub_b120 @ 0xb120
void sub_b120(char *a0,unsigned long a1)
{
  unsigned long v1; // stack - 0x48
  unsigned long v2; // stack - 0x40
  unsigned int v3; // stack - 0x3c
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  unsigned long v7; // stack - 0x20
  unsigned long v8; // stack - 0x18
  
  v1 = dat_13240;
  v3 = (unsigned int)((unsigned long)dat_13248 >> 0x20);
  v8 = dat_13270;
  v4 = dat_13250;
  v5 = dat_13258;
  v6 = dat_13260;
  v7 = dat_13268;
  v2 = (unsigned int)dat_13248;
  v2 = CONCAT44(~v3 & 0x4000000 ^ v3,v2);
  sub_a760(0,a0,a1,&v1);
}

// Function: sub_b1b0 @ 0xb1b0
void sub_b1b0(unsigned int a0,int a1,char *a2)
{
  int v1 [14]; // stack - 0x48
  
  if (a1 != 10) {
    v1[1] = 0;
    v1[2] = 0;
    v1[3] = 0x4000000;
    v1[4] = 0;
    v1[5] = 0;
    v1[6] = 0;
    v1[7] = 0;
    v1[8] = 0;
    v1[9] = 0;
    v1[10] = 0;
    v1[0xb] = 0;
    v1[0xc] = 0;
    v1[0xd] = 0;
    v1[0] = a1;
    sub_a760(a0,a2,0xffffffffffffffff,v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_b250 @ 0xb250
void sub_b250(unsigned int a0,long a1,long a2,char *a3)
{
  unsigned long v1; // stack - 0x48
  unsigned int v2; // stack - 0x44
  unsigned long v3; // stack - 0x40
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  long v7; // stack - 0x20
  long v8; // stack - 0x18
  
  v3 = dat_13248;
  v4 = dat_13250;
  v5 = dat_13258;
  v8 = dat_13270;
  v2 = (unsigned int)((unsigned long)dat_13240 >> 0x20);
  v1 = CONCAT44(v2,10);
  v6 = dat_13260;
  v7 = dat_13268;
  if ((a1) && (a2)) {
    v7 = a1;
    v8 = a2;
    sub_a760(a0,a3,0xffffffffffffffff,&v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_b2f0 @ 0xb2f0
void sub_b2f0(unsigned int a0,long a1,long a2,char *a3,unsigned long a4)
{
  unsigned long v1; // stack - 0x48
  unsigned int v2; // stack - 0x44
  unsigned long v3; // stack - 0x40
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  long v7; // stack - 0x20
  long v8; // stack - 0x18
  
  v3 = dat_13248;
  v4 = dat_13250;
  v5 = dat_13258;
  v8 = dat_13270;
  v2 = (unsigned int)((unsigned long)dat_13240 >> 0x20);
  v1 = CONCAT44(v2,10);
  v6 = dat_13260;
  v7 = dat_13268;
  if ((a1) && (a2)) {
    v7 = a1;
    v8 = a2;
    sub_a760(a0,a3,a4,&v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_b390 @ 0xb390
void sub_b390(long a0,long a1,char *a2)
{
  unsigned long v1; // stack - 0x48
  unsigned int v2; // stack - 0x44
  unsigned long v3; // stack - 0x40
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  long v7; // stack - 0x20
  long v8; // stack - 0x18
  
  v3 = dat_13248;
  v4 = dat_13250;
  v5 = dat_13258;
  v8 = dat_13270;
  v2 = (unsigned int)((unsigned long)dat_13240 >> 0x20);
  v1 = CONCAT44(v2,10);
  v6 = dat_13260;
  v7 = dat_13268;
  if ((a0) && (a1)) {
    v7 = a0;
    v8 = a1;
    sub_a760(0,a2,0xffffffffffffffff,&v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_b430 @ 0xb430
void sub_b430(long a0,long a1,char *a2,unsigned long a3)
{
  unsigned long v1; // stack - 0x48
  unsigned int v2; // stack - 0x44
  unsigned long v3; // stack - 0x40
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  long v7; // stack - 0x20
  long v8; // stack - 0x18
  
  v3 = dat_13248;
  v4 = dat_13250;
  v5 = dat_13258;
  v8 = dat_13270;
  v2 = (unsigned int)((unsigned long)dat_13240 >> 0x20);
  v1 = CONCAT44(v2,10);
  v6 = dat_13260;
  v7 = dat_13268;
  if ((a0) && (a1)) {
    v7 = a0;
    v8 = a1;
    sub_a760(0,a2,a3,&v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_b4d0 @ 0xb4d0
char * sub_b4d0(unsigned int a0,char *a1,unsigned long a2)
{
  return sub_a760(a0,a1,a2); // tail-call
}

// Function: sub_b4e0 @ 0xb4e0
char * sub_b4e0(char *a0,unsigned long a1)
{
  return sub_a760(0,a0,a1,(struct_0 *)0x13060); // tail-call
}

// Function: sub_b500 @ 0xb500
char * sub_b500(unsigned int a0,char *a1)
{
  return sub_a760(a0,a1,0xffffffffffffffff,(struct_0 *)0x13060); // tail-call
}

// Function: sub_b520 @ 0xb520
char * sub_b520(char *a0)
{
  return sub_a760(0,a0,0xffffffffffffffff,(struct_0 *)0x13060); // tail-call
}

// Function: sub_b540 @ 0xb540
long sub_b540(int a0,void *a1,unsigned long a2)
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

// Function: sub_b5b0 @ 0xb5b0
unsigned int sub_b5b0(unsigned int a0)
{
  int v1;
  unsigned int v2; // eax
  int *v3; // rax
  
  if (3 <= a0)
    return a0;
  v2 = sub_ce00(a0);
  v3 = __errno_location();
  v1 = *v3;
  close(a0);
  *v3 = v1;
  return v2;
}

// Function: sub_b610 @ 0xb610
long sub_b610(FILE *a0,char *a1,char *a2,char *a3,unsigned long *a4,unsigned long a5)
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
label_b73d:
    v1 = CONCAT44(dat_4,__fprintf_chk(a0,1,v2,v4,v3,v5,v6,v7,v8,v9,v10));
label_b776:
    return v1;
  }
  v1 = (long)*(int *)(a5 * 4 + 0xece8) + 0xece8;
  switch(a5) {
    case 0:
      goto label_b776;
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
      goto label_b73d;
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

// Function: sub_ba90 @ 0xba90
void sub_ba90(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,long *a4)
{
  long v1;
  long v2;
  
  v2 = 0;
  v1 = *a4;
  while (v1) {
    v2 += 1;
    v1 = a4[v2];
  }
  sub_b610(); // tail-call
}

// Function: sub_bab0 @ 0xbab0
void sub_bab0(FILE *a0,char *a1,char *a2,char *a3,struct_14 *a4)
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
  sub_b610(a0,a1,a2,a3);
}

// Function: sub_bb50 @ 0xbb50
void sub_bb50(FILE *a0,char *a1,char *a2,unsigned long a3,unsigned long a4,unsigned long a5)
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
  sub_b610(a0,a1,a2);
}

// Function: sub_bc30 @ 0xbc30
void sub_bc30(void)
{
  fputs_unlocked("\n",stdout);
  __printf_chk(1,dcgettext(NULL,"Report bugs to: %s\n",5),"bug-coreutils@gnu.org");
  __printf_chk(1,dcgettext(NULL,"%s home page: <%s>\n",5),"GNU coreutils","https://www.gnu.org/software/coreutils/");
  __printf_chk(1,dcgettext(NULL,"General help using GNU software: <%s>\n",5),"https://www.gnu.org/gethelp/"); // tail-call
}

// Function: sub_bce0 @ 0xbce0
void sub_bce0(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!reallocarray(a0,a1,a2)) && ((!a0 || ((a1 && (a2))))))
    sub_c220(); // no-return
}

// Function: sub_bd20 @ 0xbd20
void sub_bd20(unsigned long a0)
{
  if (malloc(a0))
    return;
  sub_c220(); // no-return
}

// Function: sub_bd40 @ 0xbd40
void sub_bd40(unsigned long a0)
{
  if (malloc(a0))
    return;
  sub_c220(); // no-return
}

// Function: sub_bd60 @ 0xbd60
void sub_bd60(void)
{
  sub_bd20(); // tail-call
}

// Function: sub_bd70 @ 0xbd70
void sub_bd70(void *a0,unsigned long a1)
{
  if ((!realloc(a0,a1)) && ((!a0 || (a1))))
    sub_c220(); // no-return
}

// Function: sub_bdb0 @ 0xbdb0
void sub_bdb0(void *a0,unsigned long a1)
{
  if (realloc(a0,a1 | a1 == 0))
    return;
  sub_c220(); // no-return
}

// Function: sub_bde0 @ 0xbde0
void sub_bde0(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!reallocarray(a0,a1,a2)) && ((!a0 || ((a1 && (a2))))))
    sub_c220(); // no-return
}

// Function: sub_be20 @ 0xbe20
void sub_be20(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!a1) || (!a2)) {
    a1 = 1;
    a2 = 1;
  }
  if (reallocarray(a0,a1,a2))
    return;
  sub_c220(); // no-return
}

// Function: sub_be60 @ 0xbe60
void sub_be60(unsigned long a0,unsigned long a1)
{
  if (reallocarray(NULL,a0,a1))
    return;
  sub_c220(); // no-return
}

// Function: sub_be90 @ 0xbe90
void sub_be90(unsigned long a0,unsigned long a1)
{
  if ((!a0) || (!a1)) {
    a0 = 1;
    a1 = 1;
  }
  if (!reallocarray(NULL,a0,a1))
    sub_c220(); // no-return
}

// Function: sub_bee0 @ 0xbee0
void sub_bee0(void *a0,unsigned long *a1)
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
  sub_c220(); // no-return
}

// Function: sub_bf70 @ 0xbf70
void sub_bf70(void *a0,unsigned long *a1,unsigned long a2)
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
  sub_c220(); // no-return
}

// Function: sub_c000 @ 0xc000
void sub_c000(void *a0,long *a1,long a2,long a3,long a4)
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
    if (0x80 <= v3) goto label_c0be;
    v4 = 0x80;
  }
  v2 = v4 / a4;
  v3 = v4 - v4 % a4;
label_c0be:
  if (!a0)
    *a1 = 0;
  if (((a2 <= v2 - v1) || ((v2 = v1 + a2, !SCARRY8(v1,a2) && (((v2 <= a3 || (a3 <= -1)) && (v3 = v2 * a4, SEXT816(v3) == SEXT816(v2) * SEXT816(a4))))))) && ((realloc(a0,v3) || ((a0 && (!v3)))))) {
    *a1 = v2;
    return;
  }
  sub_c220(); // no-return
}

// Function: sub_c100 @ 0xc100
void sub_c100(unsigned long a0,unsigned long a1)
{
  if (calloc(a0,a1))
    return;
  sub_c220(); // no-return
}

// Function: sub_c120 @ 0xc120
void sub_c120(unsigned long a0)
{
  sub_c100(a0,1); // tail-call
}

// Function: sub_c130 @ 0xc130
void sub_c130(unsigned long a0,unsigned long a1)
{
  if (calloc(a0,a1))
    return;
  sub_c220(); // no-return
}

// Function: sub_c150 @ 0xc150
void sub_c150(unsigned long a0)
{
  sub_c130(a0,1); // tail-call
}

// Function: sub_c160 @ 0xc160
void sub_c160(void *a0,unsigned long a1)
{
  memcpy((void *)sub_bd20(a1),a0,a1); // tail-call
}

// Function: sub_c190 @ 0xc190
void sub_c190(void *a0,unsigned long a1)
{
  memcpy((void *)sub_bd40(a1),a0,a1); // tail-call
}

// Function: sub_c1c0 @ 0xc1c0
void sub_c1c0(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)sub_bd40(a1 + 1);
  *(char *)((long)v1 + a1) = 0;
  memcpy(v1,a0,a1); // tail-call
}

// Function: sub_c200 @ 0xc200
void sub_c200(char *a0)
{
  sub_c160(a0,strlen(a0) + 1); // tail-call
}

// Function: sub_c220 @ 0xc220
void sub_c220(void)
{
  error(dat_13040,0,"%s",dcgettext(NULL,"memory exhausted",5));
  abort(); // no-return
}

// Function: sub_c260 @ 0xc260
unsigned long sub_c260(unsigned char *a0,unsigned int a1,unsigned long a2,unsigned long a3,unsigned long a4,char *a5,int a6)
{
  int v1;
  char *v2; // rax
  int *v3; // rax
  unsigned long v4; // stack - 0x48
  
  v1 = sub_c520(a0,NULL,a1,&v4);
  if (v1) { // branch-flip
    v3 = __errno_location();
    if (v1 != 1) {
      if (v1 == 3)
        *v3 = 0;
      goto label_c308;
    }
  }
  else {
    if ((a2 <= v4) && (v4 <= a3))
      return v4;
    v3 = __errno_location();
    if (v4 <= 0x3fffffff) {
      *v3 = 0x22;
      goto label_c308;
    }
  }
  *v3 = 0x4b;
label_c308:
  v2 = (char *)sub_b520(a0);
  v1 = *v3;
  if (v1 == 0x16) goto label_c340;
  do {
    if (!a6)
      a6 = 1;
    error(a6,v1,"%s: %s",a5,v2);
label_c340:
    v1 = 0;
  } while( true );
}

// Function: sub_c370 @ 0xc370
void sub_c370(unsigned char *a0,unsigned long a1,unsigned long a2,unsigned long a3,char *a4,unsigned long a5)
{
  sub_c260(a0,10,a1,a2,a3,a4,a5);
}

// Function: sub_c3a0 @ 0xc3a0
unsigned long sub_c3a0(double a0)
{
  unsigned int *v1; // rax
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x38
  char v4 [16];
  unsigned long v5; // stack - 0x30
  
  v1 = (unsigned int *)__errno_location();
  if (dat_ed60 <= a0) {
    do {
      pause();
      if (*v1 != 4) break;
      pause();
    } while (*v1 == 4);
  }
  v4 = sub_ca40();
  v5 = SUB168(v4,8);
  v3 = SUB168(v4,0);
  do {
    *v1 = 0;
    v2 = sub_cd30(&v3,&v3);
    if (!(int)v2)
      return v2;
  } while (!(*v1 & 0xfffffffb));
  return 0xffffffff;
}

// Function: sub_c450 @ 0xc450
bool sub_c450(char *a0,unsigned long *a1,double *a2,void *a3)
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
      if ((*v2) || (v3 = 1, v4 == dat_ed68)) goto label_c4c5;
label_c4b3:
      v3 = *v1 != 0x22;
      goto label_c4bc;
    }
    v3 = 1;
    if (v4 != dat_ed68) goto label_c4b3;
  }
  else {
    v3 = 0;
label_c4bc:
    if (!a1) goto label_c4c5;
  }
  *a1 = v2;
label_c4c5:
  *a2 = v4;
  return v3;
}

// Function: sub_c520 @ 0xc520
unsigned int sub_c520(unsigned char *a0,long *a1,unsigned int a2,unsigned long *a3,char *a4)
{
  unsigned char v1;
  unsigned long v10;
  long v11;
  int v12; // esi
  int v13; // r9d
  unsigned int v14;
  char v2 [16];
  unsigned int v3; // eax
  int *v4; // rax
  long *v5; // rax
  unsigned char *v6;
  unsigned long v7;
  unsigned long v8; // rcx
  unsigned char v9;
  
  if (0x25 <= a2)
    __assert_fail("0 <= strtol_base && strtol_base <= 36","lib/xstrtol.c",0x55,"xstrtoumax"); // no-return
  if (!a1)
    a1 = &v11;
  v4 = __errno_location();
  *v4 = 0;
  v9 = *a0;
  v5 = __ctype_b_loc();
  v6 = a0;
  while (*(unsigned char *)(*v5 + 1 + (unsigned long)v9 * 2) & 0x20) {
    v9 = v6[1];
    v6 = &v6[1];
  }
  if (v9 == 0x2d)
    return 4;
  v7 = strtoumax(a0,a1,a2);
  v6 = (unsigned char *)*a1;
  if (v6 != a0) { // branch-flip
    if (*v4) { // branch-flip
      v14 = 1;
      if (*v4 != 0x22)
        return 4;
    }
    else {
      v14 = 0;
    }
    if (!a4) goto label_c5dd;
    v9 = *v6;
    if (!v9) goto label_c5dd;
    v10 = v7;
    if (!strchr(a4,(int)(char)v9)) goto label_c5e5;
  }
  else {
    if ((!a4) || (v9 = *a0, !v9))
      return 4;
    v14 = 0;
    v10 = 1;
    if (!strchr(a4,(int)(char)v9))
      return 4;
  }
  v3 = (unsigned int)v9 - 0x45;
  v13 = 1;
  v8 = 0x400;
  if (((unsigned char)v3 <= 0x2f) && (0x814400308945U >> ((unsigned long)v3 & 0x3f) & 1)) {
    v8 = 0x400;
    v13 = 1;
    if (strchr(a4,0x30)) {
      v1 = v6[1];
      if (v1 != 0x44) { // branch-flip
        if (v1 != 0x69) { // branch-flip
          v13 = (v1 == 0x42) + 1;
          if (v1 == 0x42)
            v8 = 1000;
        }
        else {
          v13 = (v6[2] == 0x42) + 1 + (unsigned int)(v6[2] == 0x42);
        }
      }
      else {
        v13 = 2;
        v8 = 1000;
      }
    }
  }
  v7 = v10;
  switch((unsigned int)v9 - 0x42 & 0xff) {
    case 0:
      v7 = v10 << 10;
      if (v10 >> 0x36) {
        v14 = 1;
        v7 = 0xffffffffffffffff;
      }
      break;
    default:
label_c5e5:
      *a3 = v10;
      return v14 | 2;
    case 3:
      v12 = 6;
      v3 = 0;
      do {
        v2 = ZEXT816(v10);
        v10 = SUB168(v2 * ZEXT816(v8),0);
        if (SUB168(v2 * ZEXT816(v8),8)) {
          v3 = 1;
          v10 = 0xffffffffffffffff;
        }
        v12 -= 1;
      } while (v12);
      goto label_c7e4;
    case 5:
    case 0x25:
      if ((SUB168(ZEXT816(v10) * ZEXT816(v8),8)) || (v2 = ZEXT816(SUB168(ZEXT816(v10) * ZEXT816(v8),0)) * ZEXT816(v8), v7 = SUB168(v2,0), SUB168(v2,8))) goto label_c7a9;
label_c777:
      if (SUB168(ZEXT816(v7) * ZEXT816(v8),8)) { // branch-flip
        v14 = 1;
        v7 = 0xffffffffffffffff;
      }
      else {
        v7 = SUB168(ZEXT816(v7) * ZEXT816(v8),0);
      }
      break;
    case 9:
    case 0x29:
      v7 = SUB168(ZEXT816(v10) * ZEXT816(v8),0);
      if (SUB168(ZEXT816(v10) * ZEXT816(v8),8)) goto label_c7a9;
      break;
    case 0xb:
    case 0x2b:
      v7 = SUB168(ZEXT816(v10) * ZEXT816(v8),0);
      if (!SUB168(ZEXT816(v10) * ZEXT816(v8),8)) goto label_c777;
label_c7a9:
      v14 = 1;
      v7 = 0xffffffffffffffff;
      break;
    case 0xe:
      v12 = 5;
      v3 = 0;
      do {
        v2 = ZEXT816(v10);
        v10 = SUB168(v2 * ZEXT816(v8),0);
        if (SUB168(v2 * ZEXT816(v8),8)) {
          v3 = 1;
          v10 = 0xffffffffffffffff;
        }
        v12 -= 1;
      } while (v12);
      goto label_c7e4;
    case 0x12:
    case 0x32:
      v12 = 4;
      v3 = 0;
      do {
        v2 = ZEXT816(v10);
        v10 = SUB168(v2 * ZEXT816(v8),0);
        if (SUB168(v2 * ZEXT816(v8),8)) {
          v3 = 1;
          v10 = 0xffffffffffffffff;
        }
        v12 -= 1;
      } while (v12);
      goto label_c7e4;
    case 0x17:
      v12 = 8;
      v3 = 0;
      do {
        v2 = ZEXT816(v10);
        v10 = SUB168(v2 * ZEXT816(v8),0);
        if (SUB168(v2 * ZEXT816(v8),8)) {
          v3 = 1;
          v10 = 0xffffffffffffffff;
        }
        v12 -= 1;
      } while (v12);
      goto label_c7e4;
    case 0x18:
      v12 = 7;
      v3 = 0;
      do {
        v2 = ZEXT816(v10);
        v10 = SUB168(v2 * ZEXT816(v8),0);
        if (SUB168(v2 * ZEXT816(v8),8)) {
          v3 = 1;
          v10 = 0xffffffffffffffff;
        }
        v12 -= 1;
      } while (v12);
label_c7e4:
      v14 |= v3;
      v7 = v10;
      break;
    case 0x20:
      if (v10 >> 0x37) { // branch-flip
        v14 = 1;
        v7 = 0xffffffffffffffff;
      }
      else {
        v7 = v10 << 9;
      }
      break;
    case 0x21:
      break;
    case 0x35:
      if (0 <= (long)v10) // branch-flip
        v7 = v10 * 2;
      else {
        v14 = 1;
        v7 = 0xffffffffffffffff;
      }
    
  }
  *a1 = (long)&v6[v13];
  if (v6[v13])
    v14 |= 2;
label_c5dd:
  *a3 = v7;
  return v14;
}

// Function: sub_c950 @ 0xc950
unsigned long sub_c950(char *a0,char **a1) // return-dupe
{
  unsigned long v1; // xmm0_qa
  
  if (!dat_13278)
    dat_13278 = newlocale(0x1fbf,"C",0);
  if (!dat_13278) {
    if (!a1)
      return 0;
    *a1 = a0;
    return 0;
  }
  strtod_l(a0,a1); // tail-call
  return v1;
}

// Function: sub_c9d0 @ 0xc9d0
unsigned long sub_c9d0(FILE *a0) // early-return x2, return-dupe
{
  unsigned int v1;
  unsigned long v2; // rax
  unsigned long v3; // rax
  
  v2 = __fpending(a0);
  v1 = *(unsigned int *)a0;
  v3 = sub_cb00(a0);
  if (v1 & 0x20) {
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

// Function: sub_ca40 @ 0xca40
undefined16 sub_ca40(double a0)
{
  char v1 [16];
  char v2 [16];
  long v3;
  long v4; // r8
  double v5; // xmm0_qa
  
  v4 = -0x8000000000000000;
  v3 = 0;
  if (dat_ee90 < a0) {
    v3 = 999999999;
    v4 = 0x7fffffffffffffff;
    if (a0 < dat_ed60) {
      v5 = (a0 - (double)(long)a0) * dat_ee98;
      v3 = (long)v5;
      v3 = (unsigned long)((double)v3 < v5) + v3;
      v4 = (long)a0 + v3 / 1000000000;
      v3 %= 1000000000;
      if (v3 < 0) {
        v2._8_8_ = v3 + 1000000000;
        v2._0_8_ = v4 + -1;
        return v2._0_16_;
      }
    }
  }
  v1._8_8_ = v3;
  v1._0_8_ = v4;
  return v1._0_16_;
}

// Function: sub_cb00 @ 0xcb00
unsigned long sub_cb00(FILE *a0) // early-return
{
  int v1; // eax
  int v2;
  int *v3; // rax
  
  if (0 <= fileno(a0)) {
    if (__freading(a0)) {
      v2 = fileno(a0);
      if (lseek(v2,0,1) == -1) {
        return fclose(a0); // tail-call, return-dupe
      }
    }
    if (sub_cb90(a0)) {
      v3 = __errno_location();
      v2 = *v3;
      v1 = fclose(a0);
      if (!v2)
        return CONCAT44(dat_4,v1);
      *v3 = v2;
      return 0xffffffff;
    }
  }
  return fclose(a0);
}

// Function: sub_cb90 @ 0xcb90
void sub_cb90(FILE *a0)
{
  if (a0) {
    if ((__freading(a0)) && (*(unsigned int *)a0 & 0x100)) {
      sub_cbe0(a0,0,1);
      fflush(a0); // tail-call
      return;
    }
  }
  fflush(a0); // tail-call
}

// Function: sub_cbe0 @ 0xcbe0
int sub_cbe0(FILE *a0,long a1,int a2) // return-dupe
{
  long v1; // rax
  
  if (((*(long *)&a0->field_0x10 == *(long *)&a0->field_0x8) && (*(long *)&a0->field_0x28 == *(long *)&a0->field_0x20)) && (!*(long *)&a0->field_0x48)) {
    v1 = lseek(fileno(a0),a1,a2);
    if (v1 != -1) {
      *(unsigned int *)a0 = *(unsigned int *)a0 & 0xffffffef;
      *(long *)&a0->field_0x90 = v1;
      return 0;
    }
    return 0xffffffff;
  }
  return fseeko(a0,a1,a2); // tail-call
}

// Function: sub_cc60 @ 0xcc60
char * sub_cc60(void) // early-return
{
  char *v1; // rax
  
  v1 = nl_langinfo(0xe);
  if (!v1)
    return "ASCII";
  if (*v1)
    return v1;
  return "ASCII";
}

// Function: sub_cca0 @ 0xcca0
unsigned long sub_cca0(unsigned int *a0,char *a1,unsigned long a2,mbstate_t *a3)
{
  unsigned long v1; // rax
  long v2;
  
  if (!a0)
    a0 = &v2;
  v1 = mbrtowc(a0,a1,a2,a3);
  if (((0xfffffffffffffffe <= v1) && (a2)) && (!sub_ce20(0))) {
    v1 = 1;
    *a0 = (unsigned int)(unsigned char)*a1;
  }
  return v1;
}

// Function: sub_cd30 @ 0xcd30
unsigned long sub_cd30(struct_9 *a0,timespec *a1) // early-return
{
  int v1; // eax
  long v2;
  timespec v3; // stack - 0x38
  
  v3._8_8_ = a0->field_0x8;
  if (1000000000 <= v3._8_8_) {
    *__errno_location() = 0x16;
    return 0xffffffff;
  }
  v2 = a0->field_0x0;
  while( true ) {
    if (v2 <= 0x1fa400) {
      v3._0_8_ = v2;
      return CONCAT44(dat_4,nanosleep(&v3,a1));
    }
    v3._0_8_ = 0x1fa400;
    v2 -= 0x1fa400;
    v1 = nanosleep(&v3,a1);
    if (v1) break;
    v3._8_8_ = 0;
  }
  if (!a1)
    return CONCAT44(dat_4,v1);
  *(long *)a1 = *(long *)a1 + v2;
  return CONCAT44(dat_4,v1);
}

// Function: sub_ce00 @ 0xce00
void sub_ce00(unsigned long a0)
{
  sub_7b60(a0,0,3); // tail-call
}

// Function: sub_ce20 @ 0xce20
bool sub_ce20(unsigned long a0)
{
  char v1 [264];
  bool v2; // r12b
  
  v2 = 0;
  if ((!sub_ceb0(a0,v1,0x101)) && (strcmp(v1,"C")))
    v2 = strcmp(v1,"POSIX") != 0;
  return v2;
}

// Function: sub_ceb0 @ 0xceb0
unsigned long sub_ceb0(int a0,char *a1,unsigned long a2) // return-dupe x2
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

// Function: sub_cf60 @ 0xcf60
void sub_cf60(int a0)
{
  setlocale(a0,NULL); // tail-call
}

// Function: sub_cf70 @ 0xcf70
void sub_cf70(unsigned long a0)
{
  __cxa_atexit(a0,0,dat_13008); // tail-call
}

// Function: _DT_FINI @ 0xcf84
void _DT_FINI(void)
{
  return;
}
