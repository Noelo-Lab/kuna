// Function: _DT_INIT @ 0x2000
void _DT_INIT(void) // return-dupe
{
  if (!dat_10fe8)
    return;
  (*dat_10fe8)();
}

// Function: sub_2020 @ 0x2020
void sub_2020(void)
{
  (*dat_10d58)(); // jump-as-call
}

// Function: __cxa_finalize @ 0x2520
void __cxa_finalize(void)
{
  (*dat_10ff8)(); // jump-as-call
}

// Function: __uflow @ 0x2530
int __uflow(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_10d60)(); // jump-as-call
  return v1;
}

// Function: raise @ 0x2540
int raise(int a0)
{
  int v1; // eax
  
  v1 = (*dat_10d68)(); // jump-as-call
  return v1;
}

// Function: free @ 0x2550
void free(void *a0)
{
  (*dat_10d70)(); // jump-as-call
}

// Function: __mempcpy_chk @ 0x2560
void * __mempcpy_chk(void *a0,void *a1,unsigned long a2,unsigned long a3)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_10d78)(); // jump-as-call
  return v1;
}

// Function: abort @ 0x2570
void abort(void)
{
  (*dat_10d80)(); // jump-as-call
}

// Function: __errno_location @ 0x2580
int * __errno_location(void)
{
  int *v1; // rax
  
  v1 = (int *)(*dat_10d88)(); // jump-as-call
  return v1;
}

// Function: strncmp @ 0x2590
int strncmp(char *a0,char *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_10d90)(); // jump-as-call
  return v1;
}

// Function: _exit @ 0x25a0
void _exit(int a0)
{
  (*dat_10d98)(); // jump-as-call
}

// Function: iconv @ 0x25b0
void iconv(void)
{
  (*dat_10da0)(); // jump-as-call
}

// Function: sigaction @ 0x25c0
int sigaction(int a0,sigaction *a1,sigaction *a2)
{
  int v1; // eax
  
  v1 = (*dat_10da8)(); // jump-as-call
  return v1;
}

// Function: iswcntrl @ 0x25d0
int iswcntrl(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_10db0)(); // jump-as-call
  return v1;
}

// Function: reallocarray @ 0x25e0
void * reallocarray(void *a0,unsigned long a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_10db8)(); // jump-as-call
  return v1;
}

// Function: fcntl @ 0x25f0
int fcntl(int a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_10dc0)(); // jump-as-call
  return v1;
}

// Function: write @ 0x2600
long write(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_10dc8)(); // jump-as-call
  return v1;
}

// Function: textdomain @ 0x2610
char * textdomain(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_10dd0)(); // jump-as-call
  return v1;
}

// Function: __open_2 @ 0x2620
int __open_2(char *a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_10dd8)(); // jump-as-call
  return v1;
}

// Function: fclose @ 0x2630
int fclose(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_10de0)(); // jump-as-call
  return v1;
}

// Function: bindtextdomain @ 0x2640
char * bindtextdomain(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_10de8)(); // jump-as-call
  return v1;
}

// Function: dcgettext @ 0x2650
char * dcgettext(char *a0,char *a1,int a2)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_10df0)(); // jump-as-call
  return v1;
}

// Function: __ctype_get_mb_cur_max @ 0x2660
unsigned long __ctype_get_mb_cur_max(void)
{
  unsigned long v1; // rax
  
  v1 = (*dat_10df8)(); // jump-as-call
  return v1;
}

// Function: strlen @ 0x2670
unsigned long strlen(char *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_10e00)(); // jump-as-call
  return v1;
}

// Function: __stack_chk_fail @ 0x2680
void __stack_chk_fail(void)
{
  (*dat_10e08)(); // jump-as-call
}

// Function: mmap @ 0x2690
void mmap(void)
{
  (*dat_10e10)(); // jump-as-call
}

// Function: getopt_long @ 0x26a0
int getopt_long(int a0,char **a1,char *a2,option *a3,int *a4)
{
  int v1; // eax
  
  v1 = (*dat_10e18)(); // jump-as-call
  return v1;
}

// Function: mbrtowc @ 0x26b0
unsigned long mbrtowc(void *a0,char *a1,unsigned long a2,mbstate_t *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_10e20)(); // jump-as-call
  return v1;
}

// Function: dup2 @ 0x26c0
int dup2(int a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_10e28)(); // jump-as-call
  return v1;
}

// Function: strchr @ 0x26d0
char * strchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_10e30)(); // jump-as-call
  return v1;
}

// Function: __overflow @ 0x26e0
int __overflow(FILE *a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_10e38)(); // jump-as-call
  return v1;
}

// Function: strrchr @ 0x26f0
char * strrchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_10e40)(); // jump-as-call
  return v1;
}

// Function: lseek @ 0x2700
void lseek(void)
{
  (*dat_10e48)(); // jump-as-call
}

// Function: __assert_fail @ 0x2710
void __assert_fail(char *a0,char *a1,unsigned int a2,char *a3)
{
  (*dat_10e50)(); // jump-as-call
}

// Function: fputs @ 0x2720
int fputs(char *a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_10e58)(); // jump-as-call
  return v1;
}

// Function: memset @ 0x2730
void * memset(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_10e60)(); // jump-as-call
  return v1;
}

// Function: freopen @ 0x2740
FILE * freopen(char *a0,char *a1,FILE *a2)
{
  FILE *v1; // rax
  
  v1 = (FILE *)(*dat_10e68)(); // jump-as-call
  return v1;
}

// Function: strnlen @ 0x2750
unsigned long strnlen(char *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  v1 = (*dat_10e70)(); // jump-as-call
  return v1;
}

// Function: close @ 0x2760
int close(int a0)
{
  int v1; // eax
  
  v1 = (*dat_10e78)(); // jump-as-call
  return v1;
}

// Function: pipe @ 0x2770
int pipe(int *a0)
{
  int v1; // eax
  
  v1 = (*dat_10e80)(); // jump-as-call
  return v1;
}

// Function: memchr @ 0x2780
void * memchr(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_10e88)(); // jump-as-call
  return v1;
}

// Function: read @ 0x2790
long read(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_10e90)(); // jump-as-call
  return v1;
}

// Function: memcmp @ 0x27a0
int memcmp(void *a0,void *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_10e98)(); // jump-as-call
  return v1;
}

// Function: fputs_unlocked @ 0x27b0
int fputs_unlocked(char *a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_10ea0)(); // jump-as-call
  return v1;
}

// Function: calloc @ 0x27c0
void * calloc(unsigned long a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_10ea8)(); // jump-as-call
  return v1;
}

// Function: strcmp @ 0x27d0
int strcmp(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_10eb0)(); // jump-as-call
  return v1;
}

// Function: signal @ 0x27e0
void signal(void)
{
  (*dat_10eb8)(); // jump-as-call
}

// Function: sigemptyset @ 0x27f0
int sigemptyset(sigset_t *a0)
{
  int v1; // eax
  
  v1 = (*dat_10ec0)(); // jump-as-call
  return v1;
}

// Function: stat @ 0x2800
int stat(char *a0,stat *a1)
{
  int v1; // eax
  
  v1 = (*dat_10ec8)(); // jump-as-call
  return v1;
}

// Function: memcpy @ 0x2810
void * memcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_10ed0)(); // jump-as-call
  return v1;
}

// Function: mincore @ 0x2820
int mincore(void *a0,unsigned long a1,char *a2)
{
  int v1; // eax
  
  v1 = (*dat_10ed8)(); // jump-as-call
  return v1;
}

// Function: wcwidth @ 0x2830
int wcwidth(int a0)
{
  int v1; // eax
  
  v1 = (*dat_10ee0)(); // jump-as-call
  return v1;
}

// Function: iswalnum @ 0x2840
int iswalnum(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_10ee8)(); // jump-as-call
  return v1;
}

// Function: malloc @ 0x2850
void * malloc(unsigned long a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_10ef0)(); // jump-as-call
  return v1;
}

// Function: nl_langinfo @ 0x2860
char * nl_langinfo(int a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_10ef8)(); // jump-as-call
  return v1;
}

// Function: fwrite_unlocked @ 0x2870
unsigned long fwrite_unlocked(void *a0,unsigned long a1,unsigned long a2,FILE *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_10f00)(); // jump-as-call
  return v1;
}

// Function: realloc @ 0x2880
void * realloc(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_10f08)(); // jump-as-call
  return v1;
}

// Function: munmap @ 0x2890
int munmap(void *a0,unsigned long a1)
{
  int v1; // eax
  
  v1 = (*dat_10f10)(); // jump-as-call
  return v1;
}

// Function: setlocale @ 0x28a0
char * setlocale(int a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_10f18)(); // jump-as-call
  return v1;
}

// Function: __printf_chk @ 0x28b0
int __printf_chk(int a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_10f20)(); // jump-as-call
  return v1;
}

// Function: iconv_close @ 0x28c0
void iconv_close(void)
{
  (*dat_10f28)(); // jump-as-call
}

// Function: iswspace @ 0x28d0
int iswspace(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_10f30)(); // jump-as-call
  return v1;
}

// Function: mempcpy @ 0x28e0
void * mempcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_10f38)(); // jump-as-call
  return v1;
}

// Function: memmove @ 0x28f0
void * memmove(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_10f40)(); // jump-as-call
  return v1;
}

// Function: error @ 0x2900
void error(int a0,int a1,char *a2,...)
{
  (*dat_10f48)(); // jump-as-call
}

// Function: waitpid @ 0x2910
int waitpid(int a0,int *a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_10f50)(); // jump-as-call
  return v1;
}

// Function: open @ 0x2920
int open(char *a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_10f58)(); // jump-as-call
  return v1;
}

// Function: perror @ 0x2930
void perror(char *a0)
{
  (*dat_10f60)(); // jump-as-call
}

// Function: execvp @ 0x2940
int execvp(char *a0,char **a1)
{
  int v1; // eax
  
  v1 = (*dat_10f68)(); // jump-as-call
  return v1;
}

// Function: getpagesize @ 0x2950
int getpagesize(void)
{
  int v1; // eax
  
  v1 = (*dat_10f70)(); // jump-as-call
  return v1;
}

// Function: exit @ 0x2960
void exit(int a0)
{
  (*dat_10f78)(); // jump-as-call
}

// Function: __fprintf_chk @ 0x2970
int __fprintf_chk(FILE *a0,int a1,char *a2,...)
{
  int v1; // eax
  
  v1 = (*dat_10f80)(); // jump-as-call
  return v1;
}

// Function: strdup @ 0x2980
char * strdup(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_10f88)(); // jump-as-call
  return v1;
}

// Function: mbsinit @ 0x2990
int mbsinit(mbstate_t *a0)
{
  int v1; // eax
  
  v1 = (*dat_10f90)(); // jump-as-call
  return v1;
}

// Function: sigaltstack @ 0x29a0
int sigaltstack(void *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_10f98)(); // jump-as-call
  return v1;
}

// Function: iswprint @ 0x29b0
int iswprint(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_10fa0)(); // jump-as-call
  return v1;
}

// Function: fstat @ 0x29c0
int fstat(int a0,stat *a1)
{
  int v1; // eax
  
  v1 = (*dat_10fa8)(); // jump-as-call
  return v1;
}

// Function: sigaddset @ 0x29d0
int sigaddset(sigset_t *a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_10fb0)(); // jump-as-call
  return v1;
}

// Function: fork @ 0x29e0
int fork(void)
{
  int v1; // eax
  
  v1 = (*dat_10fb8)(); // jump-as-call
  return v1;
}

// Function: __ctype_b_loc @ 0x29f0
void * __ctype_b_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_10fc0)(); // jump-as-call
  return v1;
}

// Function: iconv_open @ 0x2a00
void iconv_open(void)
{
  (*dat_10fc8)(); // jump-as-call
}

// Function: __sprintf_chk @ 0x2a10
int __sprintf_chk(char *a0,int a1,unsigned long a2,char *a3,...)
{
  int v1; // eax
  
  v1 = (*dat_10fd0)(); // jump-as-call
  return v1;
}

// Function: main @ 0x2a20
int main(int argc,char **argv,char **envp)
{
  long *v1;
  int *v10;
  FILE *v11;
  FILE *v12;
  unsigned char v13;
  char v14;
  int v15; // eax
  int v16;
  int v17;
  int v18;
  int v19; // eax
  long *v2;
  unsigned int v20;
  int v21;
  long v22;
  long v23;
  long v24;
  long v25; // rax
  long v26;
  unsigned long v27;
  char *v28;
  char *v29;
  char *v3;
  long v30;
  long v31;
  long *v32;
  char *v33 [4]; // stack - 0xf8
  stat v34; // stack - 0xd8
  int v35 [5]; // stack - 0x160
  char v36 [8];
  void *v37; // stack - 0x170
  void *v38; // stack - 0x178
  int *v39; // stack - 0x168
  char v4;
  long v40 [6]; // stack - 0x148
  char v41; // stack - 0x42
  unsigned char v42;
  long v43;
  long v44;
  unsigned long v45; // rdi
  unsigned long v46;
  long v47;
  long v48;
  char **v49;
  long v5;
  int *v50;
  int *v51;
  long v52;
  bool v53; // cf
  char *v54; // stack - 0x1f0
  long *v55; // stack - 0x1e8
  int **v56; // stack - 0x1d0
  long v57; // stack - 0x1b0
  unsigned char v58; // stack - 0x18d
  int v59; // stack - 0x14c
  void *v6;
  char v60; // stack - 0x41
  long v61 [4]; // stack - 0x118
  void *v7;
  char *v8;
  unsigned char *v9;
  
  v20 = 0;
  dat_11020 = 2;
  sub_50e0(*argv);
  setlocale(6,"");
  bindtextdomain("diffutils","/usr/local/share/locale");
  textdomain("diffutils");
  sub_5020(NULL);
  sub_7940();
  v16 = 0;
  while( true ) {
    v15 = getopt_long(argc,argv,"aeimvx3AEL:TX",(option *)0x10940,NULL);
    v27 = dat_21140;
    if (v15 == -1) break;
    switch(v15) {
      case 0x33:
        dat_110ed = '\x01';
        v20 |= 1;
        break;
      default:
        sub_42a0(NULL,0); // no-return
      case 0x41:
        dat_110eb = 1;
        v20 |= 2;
        dat_110ef = 1;
        break;
      case 0x45:
        dat_110ef = 1;
        v20 |= 4;
        break;
      case 0x4c:
        if (2 < v16)
          sub_42a0("too many file label options",0); // no-return
        v33[v16] = optarg;
        v16 += 1;
        break;
      case 0x54:
        dat_110ee = '\x01';
        break;
      case 0x58:
        dat_110ec = '\x01';
        v20 |= 8;
        break;
      case 0x61:
        dat_110f2 = 1;
        break;
      case 0x65:
        v20 |= 0x10;
        break;
      case 0x69:
        dat_110ea = 1;
        break;
      case 0x6d:
        dat_110e9 = 1;
        break;
      case 0x76:
        sub_7150(stdout,"diff3","GNU diffutils",dat_11018,sub_5580("Randy Smith"),0);
        sub_4690();
        return 0;
      case 0x78:
        dat_110ec = '\x01';
        v20 |= 0x20;
        break;
      case 0x80:
        dat_11010 = optarg;
        break;
      case 0x81:
        v22 = 0x108a0;
        __printf_chk(1,dcgettext(NULL,"Usage: %s [OPTION]... MYFILE OLDFILE YOURFILE\n",5),v27);
        __printf_chk(1,"%s\n\n",dcgettext(NULL,"Compare three files line by line.",5));
        v11 = stdout;
        fputs_unlocked(dcgettext(NULL,"Mandatory arguments to long options are mandatory for short options too.\n",5),v11);
        v28 = "-A, --show-all              output all changes, bracketing conflicts";
        do {
          if (*v28) { // branch-flip
            __printf_chk(1,"  %s\n",dcgettext(NULL,v28,5));
            v28 = *(char **)(v22 + 8);
            v11 = stdout;
          }
          else {
            v8 = *(char **)&stdout->field_0x28;
            if (*(char **)&stdout->field_0x30 <= v8) // branch-flip
              __overflow(stdout,10);
            else {
              *(char **)&stdout->field_0x28 = &v8[1];
              *v8 = 10;
            }
            v28 = *(char **)(v22 + 8);
            v11 = stdout;
          }
          v22 += 8;
          stdout = v11;
        } while (v28);
        fputs_unlocked(dcgettext(NULL,"\nThe default output format is a somewhat human-readable representation of\nthe changes.\n\nThe -e, -E, -x, -X (and corresponding long) options cause an ed script\nto be output instead of the default.\n\nFinally, the -m (--merge) option causes diff3 to do the merge internally\nand output the actual merged file.  For unusual input, this is more\nrobust than using ed.\n",5),v11);
        v28 = dcgettext(NULL,"Exit status is 0 if successful, 1 if conflicts, 2 if trouble.",5);
        __printf_chk(1,"\n%s\n%s\n",dcgettext(NULL,"If a FILE is \'-\', read standard input.",5),v28);
        sub_7230();
        sub_4690();
        return 0;
      case 0x82:
        dat_110f1 = 1;
      
    }
  }
  dat_110f0 = (dat_110e9 ^ 1) & v20 != 0;
  v13 = v20 == 0 & dat_110e9;
  dat_110eb |= v13;
  dat_110ef = v13 | dat_110ef;
  if (((v20 - 1 & v20) || (dat_110e9 & dat_110ea)) || ((v16 && (!dat_110ef))))
    sub_42a0("incompatible options",0); // no-return
  if (argc - optind != 3) {
    if (argc - optind > 2)
      sub_42a0("extra operand \'%s\'",argv[(long)optind + 3]); // no-return
    sub_42a0("missing operand after \'%s\'",argv[(long)argc + -1]); // no-return
  }
  argv = &argv[optind];
  for (v22 = (long)v16; (int)v22 <= 2; v22 = v22 + 1) {
    v33[v22] = argv[v22];
  }
  v13 = v20 != 0 | dat_110e9;
  v16 = 2 - (unsigned int)v13;
  v57 = (long)v16;
  v28 = argv[v57];
  if (strcmp(v28,"-")) {
label_2ed8:
    v35[0] = 0;
    v35[3] = 0;
    v15 = 3 - v16;
    v22 = (long)v15;
    v35[v22 + 3] = 1;
    v35[v57 + 3] = 2;
    v49 = argv;
    v35[1] = v15;
    v35[2] = v16;
    do {
      v28 = *v49;
      if (strcmp(v28,"-")) {
        if (stat(v28,&v34) < 0)
          sub_4330(*v49); // no-return
        if ((v34._24_4_ & 0xf000) == 0x4000) {
          error(2,0x15,"%s",*v49);
          return v19;
        }
      }
      v49 = &v49[1];
    } while (&argv[3] != v49);
    signal(0x11,0);
    v50 = (int *)0xcd20;
    v28 = argv[v59];
    v23 = sub_46e0(argv[v35[4]],v28,v36,&v37);
    v40[4] = sub_46e0(*argv,v28,v36,&v38);
    v40[5] = v23;
    v56 = &v39;
    v39 = NULL;
label_2ff0:
    v23 = v61[3];
    v47 = v61[2];
    v44 = v61[1];
    v52 = v61[0];
    v51 = v39;
    v42 = dat_110f0;
    v13 = dat_110e9;
    v11 = stdout;
    if (v40[4]) {
      v40[3] = 0;
      v40[2] = 0;
      v40[1] = 0;
      v40[0] = 0;
      if (v40[5]) { // branch-flip
        v43 = *(long *)(v40[5] + 0x10);
        v48 = *(long *)(v40[4] + 0x10);
        v45 = (unsigned long)(v43 < v48);
        v20 = (unsigned int)(v43 < v48);
        v46 = (unsigned long)(v48 <= v43);
        v48 = v45 << 3;
        v43 = v40[v45 + 4];
      }
      else {
        v20 = 0;
        v48 = 0;
        v46 = 1;
        v45 = 0;
        v43 = v40[4];
      }
label_306f:
      v40[3] = 0;
      v40[2] = 0;
      v40[1] = 0;
      v40[0] = 0;
      v24 = *(long *)(v43 + 0x40);
      v30 = *(long *)(v43 + 0x18);
      v40[v45 + 2] = v43;
      v40[v45] = v43;
      v40[v45 + 4] = v24;
      v24 = v40[v46 + 4];
      *(unsigned long *)(v43 + 0x40) = 0;
      if (v24) {
        do {
          if (v30 + 1 < *(long *)(v24 + 0x10)) break;
          if (v40[v46]) // branch-flip
            *(long *)(v40[v46 + 2] + 0x40) = v24;
          else {
            v40[v46] = v24;
          }
          v40[v46 + 2] = v24;
          v43 = *(long *)(v40[v46 + 4] + 0x40);
          *(unsigned long *)(v24 + 0x40) = 0;
          v24 = *(long *)(v24 + 0x18);
          v40[v46 + 4] = v43;
          v21 = v20;
          v17 = v20 ^ 1;
          if (v30 < v24) {
            v30 = v24;
            v21 = v20 ^ 1;
            v17 = v20;
          }
          v20 = v21;
          v46 = (unsigned long)v17;
          v24 = v40[v46 + 4];
        } while (v24);
        v45 = (unsigned long)(int)v20;
      }
      v43 = *(long *)(*(long *)((long)v40 + v48) + 0x10);
      v48 = *(long *)(v40[v45 + 2] + 0x18);
      v24 = 0;
      do {
        v32 = *(long **)((long)v40 + v24);
        if (v32) { // branch-flip
          *(long *)((long)v40 + v24 + 0x30) = (v43 - v32[2]) + *v32;
          v30 = *(long *)(v24 + (long)&v40[2]);
          v30 = (v48 - *(long *)(v30 + 0x18)) + *(long *)(v30 + 8);
        }
        else {
          *(long *)((long)v40 + v24 + 0x30) = (v43 - *(long *)&v50[0xc]) + *(long *)((long)v50 + v24 * 2 + 0x10);
          v30 = (v48 - *(long *)&v50[0xc]) + *(long *)((long)v50 + v24 * 2 + 0x10);
        }
        *(long *)((long)v40 + v24 + 0x40) = v30;
        v24 += 8;
      } while (v24 != 0x10);
      v50 = (int *)sub_7350(0x70);
      *v50 = 0;
      v50[0x1a] = 0;
      v50[0x1b] = 0;
      *(long *)&v50[2] = v52;
      *(long *)&v50[6] = v44;
      *(long *)&v50[4] = v47;
      *(long *)&v50[8] = v23;
      *(long *)&v50[10] = v43;
      *(long *)&v50[0xc] = v48;
      v46 = (v47 - v52) + 1;
      if (v46) { // branch-flip
        *(unsigned long *)&v50[0xe] = sub_76b0(v46,8);
        v27 = sub_76b0(v46,8);
        v23 = *(long *)&v50[8];
        v44 = *(long *)&v50[6];
        *(unsigned long *)&v50[0x14] = v27;
      }
      else {
        v50[0xe] = 0;
        v50[0xf] = 0;
        v50[0x14] = 0;
        v50[0x15] = 0;
      }
      v46 = (v23 - v44) + 1;
      if (v46) { // branch-flip
        *(unsigned long *)&v50[0x10] = sub_76b0(v46,8);
        v27 = sub_76b0(v46,8);
      }
      else {
        v50[0x10] = 0;
        v50[0x11] = 0;
        v27 = 0;
      }
      *(unsigned long *)&v50[0x16] = v27;
      v46 = (*(long *)&v50[0xc] - *(long *)&v50[10]) + 1;
      if (v46) { // branch-flip
        *(unsigned long *)&v50[0x12] = sub_76b0(v46,8);
        v23 = sub_76b0(v46,8);
      }
      else {
        v50[0x12] = 0;
        v50[0x13] = 0;
        v23 = 0;
      }
      *(long *)&v50[0x18] = v23;
      v55 = v40;
      do {
        v44 = *v55;
        if (v44) {
          v48 = *(long *)&v50[0x12];
          do {
            v24 = *(long *)(v44 + 0x10);
            v30 = *(long *)(v44 + 0x18);
            v26 = *(long *)(v44 + 0x38);
            v5 = *(long *)(v44 + 0x28);
            v31 = (v24 - v43) * 8;
            if (!sub_4460(v5,v26,v31 + v48,v23 + v31,(v30 - v24) + 1)) {
              sub_4300("internal error: screwup in format of diff blocks"); // return-dupe, no-return
            }
            v44 = *(long *)(v44 + 0x40);
          } while (v44);
        }
        v55 = &v55[1];
      } while (&v40[2] != v55);
      v44 = 0;
      do {
        v32 = *(long **)((long)v40 + v44);
        v24 = 0;
        v48 = v52;
        while( true ) {
          v30 = v47 + 1;
          if (v32)
            v30 = *v32;
          if (v30 <= v48) break;
          v48 += 1;
          *(unsigned long *)(*(long *)((long)v50 + v44 + 0x38) + v24) = *(unsigned long *)(*(long *)&v50[0x12] + v24);
          *(unsigned long *)(*(long *)((long)v50 + v44 + 0x50) + v24) = *(unsigned long *)(v23 + v24);
          v24 += 8;
        }
        if (v32) {
          v48 = *(long *)((long)v50 + v44 + 0x38);
          v24 = *(long *)((long)v50 + v44 + 0x50);
label_33b0:
          v30 = *v32;
          v26 = v32[1];
          v5 = v32[6];
          v31 = v32[4];
          v25 = (v30 - v52) * 8;
          if (sub_4460(v31,v5,v48 + v25,v24 + v25,(v26 - v30) + 1)) {
            v1 = &v32[1];
            v2 = &v32[3];
            v32 = (long *)v32[8];
            v30 = (*v2 - v43) * 8;
            v26 = (*v1 + 1) - v52;
            do {
              v30 += 8;
              if (v32) { // branch-flip
                if (*v32 - v52 <= v26) goto label_33b0;
              }
              else if ((v47 + 1) - v52 <= v26) goto label_3447;
              *(unsigned long *)(v48 + v26 * 8) = *(unsigned long *)(*(long *)&v50[0x12] + v30);
              *(unsigned long *)(v24 + v26 * 8) = *(unsigned long *)(v23 + v30);
              v26 += 1;
            } while( true );
          }
          sub_4300("internal error: screwup in format of diff blocks");
        }
label_3447:
        if (v44 + 8 == 0x10) goto label_3455;
        v52 = *(long *)((long)v40 + v44 + 0x38);
        v47 = *(long *)((long)v40 + v44 + 0x48);
        v44 += 8;
      } while( true );
    }
    if (v40[5]) {
      v40[3] = 0;
      v46 = 0;
      v48 = 8;
      v20 = 1;
      v40[2] = 0;
      v45 = 1;
      v40[1] = 0;
      v40[0] = 0;
      v43 = v40[5];
      goto label_306f;
    }
    if (dat_110f0) {
      v50 = NULL;
      while (v51) {
        v10 = *(int **)&v51[0x1a];
        *(int **)&v51[0x1a] = v50;
        v50 = v51;
        v51 = v10;
      }
      v58 = 0;
      do {
        if (!v50) {
          if (dat_110ea)
            fputs_unlocked("w\nq\n",v11);
          goto label_3918;
        }
        if (*v50 != 4) { // branch-flip
          v18 = v35[(unsigned long)(unsigned int)(*v50 - 5) + 3];
          if (v18 != 1) { // branch-flip
            if (v18 != 2) { // branch-flip
              if (v18 == -1) goto label_3da6;
            }
            else if (!dat_110ec) {
              v23 = *(long *)&v50[2];
              v44 = *(long *)&v50[4];
              goto label_3f3d;
            }
          }
          else if (dat_110eb) {
            v44 = *(long *)&v50[4];
            v23 = *(long *)&v50[2];
            __fprintf_chk(v11,1,"%lda\n",v44);
            __fprintf_chk(v11,1,">>>>>>> %s\n",v33[2]);
            sub_45f0(v11,0,v44 + 2,(*(long *)&v50[v22 * 4 + 4] - *(long *)&v50[v22 * 4 + 2]) + 3 + (*(long *)&v50[v57 * 4 + 4] - *(long *)&v50[v57 * 4 + 2]));
            __fprintf_chk(v11,1,"%lda\n<<<<<<< %s\n",v23 + -1,v33[1]);
            v14 = sub_4500(v11,v50,v15);
            fputs_unlocked("=======\n",v11);
            goto label_4072;
          }
        }
        else {
label_3da6:
          if (!dat_110ed) {
            v23 = *(long *)&v50[2];
            v44 = *(long *)&v50[4];
            if (dat_110ef) { // branch-flip
              __fprintf_chk(v11,1,"%lda\n",v44);
              v20 = (unsigned int)dat_110eb;
              if (dat_110eb) {
                __fprintf_chk(v11,1,"||||||| %s\n",v33[1]);
                v20 = sub_4500(v11,v50,v15);
              }
              fputs_unlocked("=======\n",v11);
              v21 = sub_4500(v11,v50,v16);
              __fprintf_chk(v11,1,">>>>>>> %s\n",v33[2]);
              sub_45f0(v11,(v20 | v21) & 0xff,v44 + 2,(*(long *)&v50[v22 * 4 + 4] - *(long *)&v50[v22 * 4 + 2]) + 3 + (*(long *)&v50[v57 * 4 + 4] - *(long *)&v50[v57 * 4 + 2]));
              __fprintf_chk(v11,1,"%lda\n<<<<<<< %s\n",v23 + -1,v33[0]);
              v14 = 0;
label_4072:
              sub_45f0(v11,(unsigned char)v14,v23 + 1,(*(long *)&v50[v22 * 4 + 4] - *(long *)&v50[v22 * 4 + 2]) + 1);
              v58 = v42;
            }
            else {
label_3f3d:
              if (*(long *)&v50[v57 * 4 + 4] - *(long *)&v50[v57 * 4 + 2] != -1) { // branch-flip
                if (v44 - v23 != -1) { // branch-flip
                  if (v44 != v23) // branch-flip
                    __fprintf_chk(v11,1,"%ld,%ldc\n",v23,v44);
                  else {
                    __fprintf_chk(v11,1,"%ldc\n",v44);
                  }
                }
                else {
                  __fprintf_chk(v11,1,"%lda\n",v44);
                }
                v44 = *(long *)&v50[v57 * 4 + 4];
                v52 = *(long *)&v50[v57 * 4 + 2];
                sub_45f0(v11,(unsigned char)sub_4500(v11,v50,v16),v23,(v44 - v52) + 1);
              }
              else if (v23 != v44) // branch-flip
                __fprintf_chk(v11,1,"%ld,%ldd\n",v23,v44);
              else {
                __fprintf_chk(v11,1,"%ldd\n",v23);
              }
            }
          }
        }
        v50 = *(int **)&v50[0x1a];
      } while( true );
    }
    if (!dat_110e9) {
      v28 = "  ";
      if (dat_110ee)
        v28 = "\t";
      for (v50 = v39; v50; v50 = *(int **)&v50[0x1a]) {
        if (*v50 != 4) { // branch-flip
          v20 = *v50 - 5;
          if (3 <= v20)
            sub_4300("internal error: invalid diff type passed to output"); // no-return
          v54._0_4_ = v35[(unsigned long)v20 + 3];
          v60 = 0;
          v41 = (char)(int)v54 + '1';
          v55._0_4_ = (unsigned int)((int)v54 == 0);
        }
        else {
          v41 = '\0';
          v55._0_4_ = 3;
          v54._0_4_ = 3;
        }
        __fprintf_chk(v11,1,"====%s\n",&v41);
        v20 = 0;
        do {
          v44 = (long)v35[(int)v20];
          v22 = *(long *)&v50[v44 * 4 + 2];
          v23 = *(long *)&v50[v44 * 4 + 4];
          v21 = v20 + 1;
          __fprintf_chk(v11,1,"%d:",v21);
          if (v22 != v23) { // branch-flip
            if (v22 - v23 != 1) // branch-flip
              __fprintf_chk(v11,1,"%ld,%ldc\n",v22,v23);
            else {
              __fprintf_chk(v11,1,"%lda\n",v22 + -1);
            }
          }
          else {
            __fprintf_chk(v11,1,"%ldc\n",v22);
          }
          if ((v22 <= v23) && ((unsigned int)v55 != v20)) {
            v52 = 0;
            do {
              fputs_unlocked(v28,v11);
              v6 = *(void **)(*(long *)&v50[(v44 + 6) * 2 + 2] + v52 * 8);
              v46 = *(unsigned long *)(*(long *)&v50[v44 * 2 + 0x14] + v52 * 8);
              v52 += 1;
              fwrite_unlocked(v6,1,v46,v11);
            } while (v52 <= v23 - v22);
            if (*(char *)((long)v6 + (v46 - 1)) != '\n')
              __fprintf_chk(v11,1,"\n\\ %s\n",dcgettext(NULL,"No newline at end of file",5));
          }
          if ((int)v54 == 1)
            v21 = *(unsigned int *)((long)(int)v20 * 4 + 0xcd00);
          v20 = v21;
        } while (v21 <= 2);
      }
      v58 = 0;
      goto label_3918;
    }
    sub_7810(*argv,"r",stdin);
    v12 = stdin;
    v11 = stdout;
    v58 = 0;
    v23 = 0;
    goto label_36fa;
  }
  v28 = *argv;
  v16 = v13 + 1;
  if (strcmp(v28,"-")) {
    v57 = (long)v16;
    v28 = argv[v57];
    if (strcmp(v28,"-")) goto label_2ed8;
  }
  sub_4300("\'-\' specified for more than one input file"); // no-return
label_3455:
  v18 = 6;
  if (v40[0]) {
    if (v40[1]) { // branch-flip
      v23 = *(long *)&v50[4];
      v18 = 4;
      v44 = *(long *)&v50[2];
      if (v23 - v44 == *(long *)&v50[8] - *(long *)&v50[6]) {
        v24 = 0;
        v52 = *(long *)&v50[0x16];
        v47 = *(long *)&v50[0x10];
        v43 = *(long *)&v50[0x14];
        v48 = *(long *)&v50[0xe];
        do {
          if ((v23 - v44) + 1 == v24) {
            v18 = 7;
            goto label_3492;
          }
          v6 = *(void **)(v48 + v24 * 8);
          if (((!v6) || (v7 = *(void **)(v47 + v24 * 8), !v7)) || (v46 = *(unsigned long *)(v43 + v24 * 8), v46 != *(unsigned long *)(v52 + v24 * 8))) break;
          v24 += 1;
        } while (!memcmp(v6,v7,v46));
        v18 = 4;
      }
    }
    else {
      v18 = 5;
    }
  }
label_3492:
  *v50 = v18;
  *v56 = v50;
  v56 = (int **)&v50[0x1a];
  goto label_2ff0;
label_36fa:
  if (!v51) {
    do {
      v9 = *(unsigned char **)&v12->field_0x8;
      if (*(unsigned char **)&v12->field_0x10 <= v9) { // branch-flip
        v20 = __uflow(v12);
        if ((v20 == 0xffffffff) && (*(unsigned char *)v12 & 0x30)) {
label_3908:
          if (*(unsigned char *)stdin & 0x20)
            sub_4300("read failed"); // no-return
label_3918:
          free(v38);
          free(v37);
          sub_4690();
          exit((unsigned int)v58); // no-return
        }
      }
      else {
        *(unsigned char **)&v12->field_0x8 = &v9[1];
        v20 = (unsigned int)*v9;
      }
      v8 = *(char **)&v11->field_0x28;
      if (*(char **)&v11->field_0x30 <= v8) // branch-flip
        __overflow(v11,v20 & 0xff);
      else {
        *(char **)&v11->field_0x28 = &v8[1];
        *v8 = (char)v20;
      }
    } while( true );
  }
  if (*v51 != 4) { // branch-flip
    v16 = v35[(unsigned long)(unsigned int)(*v51 - 5) + 3];
    if (v16 != 1) { // branch-flip
      if (v16 != 2) { // branch-flip
        if (v16 == -1) goto label_3736;
      }
      else if (!dat_110ec) {
        v16 = 7;
        v55 = (long)(*(long *)&v51[2] + -1);
        v54 = "<<<<<<< %s\n";
        v23 = (*(long *)&v51[2] - v23) + -2;
        v42 = 0;
        if (0 <= v23) goto label_37a8;
        goto label_382f;
      }
    }
    else if (dat_110eb) {
      v16 = 6;
      v55 = (long)(*(long *)&v51[2] + -1);
      v23 = (*(long *)&v51[2] - v23) + -2;
      v54 = "<<<<<<< %s\n";
      v42 = dat_110eb;
      if (0 <= v23) goto label_37a8;
label_39e2:
      __fprintf_chk(v11,1,v54,v33[1]);
      for (v23 = 0; v23 <= *(long *)&v51[v22 * 4 + 4] - *(long *)&v51[v22 * 4 + 2]; v23 = v23 + 1) {
        fwrite_unlocked(*(void **)(*(long *)&v51[v22 * 2 + 0xe] + v23 * 8),1,*(unsigned long *)(*(long *)&v51[v22 * 2 + 0x14] + v23 * 8),v11);
      }
      fputs_unlocked("=======\n",v11); // crossjump-dupe
      v58 = v13;
      v42 = v13;
      goto label_382f;
    }
  }
  else {
label_3736:
    if (!dat_110ed) {
      v55 = (long)(*(long *)&v51[2] + -1);
      v23 = (*(long *)&v51[2] - v23) + -2;
      if (0 <= v23) { // branch-flip
        v16 = 4;
        v54 = "||||||| %s\n";
        v42 = dat_110ef;
label_37a8:
        do {
          v9 = *(unsigned char **)&v12->field_0x8;
          if (*(unsigned char **)&v12->field_0x10 <= v9) { // branch-flip
            v20 = __uflow(v12);
            if (v20 == 0xffffffff) {
              if (*(unsigned int *)v12 & 0x20)
                sub_4330(dcgettext(NULL,"read failed",5)); // return-dupe, no-return
              if (*(unsigned int *)v12 & 0x10)
                sub_4300("input file shrank"); // return-dupe, no-return
            }
          }
          else {
            *(unsigned char **)&v12->field_0x8 = &v9[1];
            v20 = (unsigned int)*v9;
          }
          v8 = *(char **)&v11->field_0x28;
          if (*(char **)&v11->field_0x30 <= v8) // branch-flip
            __overflow(v11,v20 & 0xff);
          else {
            *(char **)&v11->field_0x28 = &v8[1];
            *v8 = (char)v20;
          }
        } while ((v20 != 10) || (v53 = v23 != 0, v23 = v23 + -1, v53));
        v53 = v42 != 0;
        v42 = 0;
        if (v53) {
          if (v16 == 4) goto label_3a91;
label_3808:
          if (dat_110eb) goto label_39e2;
          fputs_unlocked("=======\n",v11);
          v58 = v13;
          v42 = v13;
        }
      }
      else {
        v42 = 0;
        if (dat_110ef) {
          v54 = "||||||| %s\n";
label_3a91:
          __fprintf_chk(v11,1,"<<<<<<< %s\n",v33[0]);
          for (v23 = 0; v23 <= *(long *)&v51[4] - *(long *)&v51[2]; v23 = v23 + 1) {
            fwrite_unlocked(*(void **)(*(long *)&v51[0xe] + v23 * 8),1,*(unsigned long *)(*(long *)&v51[0x14] + v23 * 8),v11);
          }
          goto label_3808;
        }
      }
label_382f:
      for (v23 = 0; v23 <= *(long *)&v51[v57 * 4 + 4] - *(long *)&v51[v57 * 4 + 2]; v23 = v23 + 1) {
        fwrite_unlocked(*(void **)(*(long *)&v51[v57 * 2 + 0xe] + v23 * 8),1,*(unsigned long *)(*(long *)&v51[v57 * 2 + 0x14] + v23 * 8),v11);
      }
      if (v42)
        __fprintf_chk(v11,1,">>>>>>> %s\n",v33[2]);
      v23 = (*(long *)&v51[4] - *(long *)&v51[2]) + 1;
      v44 = v23;
label_389a:
      v44 -= 1;
      if (0 <= v44) {
        v28 = *(char **)&v12->field_0x8;
        v29 = *(char **)&v12->field_0x10;
        do {
          while (v28 >= v29) {
            v16 = __uflow(v12);
            if (v16 == 10) goto label_389a;
            if (v16 == -1) {
              if (*(unsigned int *)v12 & 0x20) {
                sub_4330(dcgettext(NULL,"read failed",5));
              }
              if (*(unsigned int *)v12 & 0x10) {
                if ((v44) || (*(long *)&v51[0x1a])) {
                  sub_4300("input file shrank");
                }
                goto label_3908;
              }
            }
            v29 = *(char **)&v12->field_0x10;
            v28 = *(char **)&v12->field_0x8;
          }
          v3 = &v28[1];
          *(char **)&v12->field_0x8 = v3;
          v4 = *v28;
          v28 = v3;
        } while (v4 != '\n');
        goto label_389a;
      }
      v23 = (long)v55 + v23;
    }
  }
  v51 = *(int **)&v51[0x1a];
  goto label_36fa;
}

// Function: sub_41b0 @ 0x41b0
void sub_41b0(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_10fd8)(main,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: sub_41e0 @ 0x41e0
void sub_41e0(void)
{
  return;
}

// Function: _FINI_0 @ 0x4250
void _FINI_0(void)
{
  if (!dat_110e8) {
    if (dat_10ff8)
      __cxa_finalize(dat_11008);
    sub_41e0();
    dat_110e8 = 1;
    return;
  }
}

// Function: _INIT_0 @ 0x4290
void _INIT_0(void)
{
  return;
}

// Function: sub_42a0 @ 0x42a0
void sub_42a0(char *a0,unsigned long a1)
{
  char *v1;
  
  if (a0)
    error(0,0,dcgettext(NULL,a0,5),a1);
  v1 = dat_21140;
  error(2,0,dcgettext(NULL,"Try \'%s --help\' for more information.",5),v1);
}

// Function: sub_4300 @ 0x4300
void sub_4300(char *a0)
{
  error(2,0,"%s",dcgettext(NULL,a0,5));
}

// Function: sub_4330 @ 0x4330
void sub_4330(char *a0)
{
  error(2,*__errno_location(),"%s",a0);
}

// Function: sub_4360 @ 0x4360
char * sub_4360(char *a0,unsigned long *a1,long *a2,char *a3,char a4)
{
  char *v1;
  char v2;
  char *v3;
  
  if ((*a0 != a4) || (a0[1] != ' '))
    sub_4300("invalid diff format; incorrect leading line chars"); // no-return
  a0 = &a0[2];
  *a1 = (unsigned long)a0;
  v3 = a0;
  while (v1 = &v3[1], *v3 != '\n') {
    v3 = v1;
  }
  *a2 = (long)v1 - (long)a0;
  if ((v1 < a3) && (*v1 == '\\')) {
    if (dat_110f0) // branch-flip
      __fprintf_chk(stderr,1,"%s:",dat_21140);
    else {
      *a2 = ((long)v1 - (long)a0) + -1;
    }
    v3 = &v3[2];
    v2 = dat_110f0;
    while( true ) {
      if (v2) {
        v2 = *v3;
        v1 = *(char **)&stderr->field_0x28;
        if (*(char **)&stderr->field_0x30 <= v1) // branch-flip
          __overflow(stderr,(unsigned int)(unsigned char)v2);
        else {
          *(char **)&stderr->field_0x28 = &v1[1];
          *v1 = v2;
        }
      }
      if (*v3 == '\n') break;
      v3 = &v3[1];
      v2 = dat_110f0;
    }
    return &v3[1];
  }
  return v1;
}

// Function: sub_4460 @ 0x4460
unsigned long sub_4460(long a0,long a1,long a2,long a3,long a4)
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

// Function: sub_4500 @ 0x4500
char sub_4500(FILE *a0,long a1,int a2)
{
  long v1;
  char *v2;
  char *v3;
  long v4; // rbx
  long v5; // r12
  long v6; // r15
  char v7; // stack - 0x41
  
  v6 = (long)a2;
  v5 = v6 * 0x10 + a1;
  if (*(long *)(v5 + 8) <= *(long *)(v5 + 0x10)) { // branch-flip
    v7 = 0;
    v4 = 0;
    do {
      v1 = v4 * 8;
      v2 = *(char **)(*(long *)(a1 + 0x38 + v6 * 8) + v4 * 8);
      if (*v2 == '.') {
        v3 = *(char **)&a0->field_0x28;
        if (*(char **)&a0->field_0x30 <= v3) { // branch-flip
          __overflow(a0,0x2e);
          v7 = 1;
        }
        else {
          v7 = 1;
          *(char **)&a0->field_0x28 = &v3[1];
          *v3 = 0x2e;
        }
      }
      v4 += 1;
      fwrite_unlocked(v2,1,*(unsigned long *)(*(long *)(a1 + 0x50 + v6 * 8) + v1),a0);
    } while (v4 <= *(long *)(v5 + 0x10) - *(long *)(v5 + 8));
  }
  else {
    v7 = 0;
  }
  return v7;
}

// Function: sub_45f0 @ 0x45f0
void sub_45f0(FILE *a0,bool a1,long a2,long a3)
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

// Function: sub_4690 @ 0x4690
void sub_4690(void)
{
  if (*(unsigned char *)stdout & 0x20)
    sub_4300("write failed"); // no-return
  if (!fclose(stdout))
    return;
  sub_4330(dcgettext(NULL,"standard output",5)); // no-return
}

// Function: sub_46e0 @ 0x46e0
unsigned long sub_46e0(char *a0,char *a1,unsigned long *a2,unsigned long *a3)
{
  long v1;
  unsigned char *v10;
  unsigned long v11; // rax
  unsigned char v12;
  unsigned int v13;
  unsigned char *v14;
  long v15;
  unsigned char *v16;
  char *v17 [9]; // stack - 0x120
  int v18; // stack - 0x130
  stat v19; // stack - 0xd8
  unsigned int *v2;
  unsigned int v20; // stack - 0x138
  char *v21; // stack - 0x128
  char *v22; // rsi
  long v23;
  long v24;
  unsigned long v25;
  long *v26;
  unsigned int v27; // stack - 0x134
  int v28; // stack - 0x12c
  unsigned int v3;
  char *v4;
  int v5; // eax
  char **v6;
  char **v7;
  unsigned char *v8;
  long v9;
  
  v21 = dat_11010;
  v6 = v17;
  if (dat_110f2) {
    v17[0] = "-a";
    v6 = &v17[1];
  }
  v7 = v6;
  if (dat_110f1) {
    v7 = &v6[1];
    *v6 = "--strip-trailing-cr";
  }
  v7[2] = a0;
  *v7 = "--horizon-lines=100";
  v7[1] = "--";
  v7[3] = a1;
  v7[4] = NULL;
  if (pipe(&v18))
    sub_4330("pipe"); // no-return
  v5 = fork();
  if (!v5) {
    close(v18);
    if (v28 != 1) {
      dup2(v28,1);
      close(v28);
    }
    execvp(dat_11010,&v21);
    _exit((*__errno_location() == 2) + 0x7e); // no-return
  }
  if (v5 == -1)
    sub_4330("fork"); // no-return
  close(v28);
  if (fstat(v18,&v19))
    sub_4330("fstat"); // no-return
  v25 = v19._56_8_;
  if ((long)v19._56_8_ <= 0)
    v25 = 1;
  v15 = 0;
  v8 = (unsigned char *)sub_7350(v25);
  while( true ) {
    v23 = v25 - v15;
    v9 = sub_7a10(v18,&v8[v15],v23);
    v15 += v9;
    if (v23 != v9) break;
    if (0x3ffffffffffffffe < v25) {
      sub_77d0(); // no-return, return-dupe
    }
    v25 *= 2;
    v8 = (unsigned char *)sub_73a0(v8,v25);
  }
  if (v9 == -1)
    sub_4330(dcgettext(NULL,"read failed",5)); // no-return
  if ((v15) && (v8[v15 + -1] != 10))
    sub_4300("invalid diff format; incomplete last line"); // no-return
  if (close(v18))
    sub_4330("close"); // no-return
  v2 = &v20;
  v5 = waitpid(v5,(int *)v2,0);
  v4 = dat_11010;
  if (v5 < 0)
    sub_4330("waitpid"); // no-return
  if (v20 & 0x7f) { // branch-flip
    v13 = 0x7fffffff;
    v22 = "subsidiary program \'%s\' failed";
  }
  else {
    if (!((int)v20 >> 8 & 0xfeU)) {
      v16 = &v8[v15];
      *a3 = v8;
      v13 = v20;
      v3 = v27;
      do {
        if (v16 <= v8) {
          v2[0] = 0;
          v2[1] = 0;
          *a2 = v26;
          return CONCAT44(v3,v13);
        }
        v20 = v13;
        v27 = v3;
        v26 = (long *)sub_7350(0x48);
        v4 = dat_21140;
        v26[5] = 0;
        v26[4] = 0;
        v26[7] = 0;
        v26[6] = 0;
        v12 = *v8;
        v10 = v8;
        if (v12 != 0x20) goto label_48f5;
        do {
          do {
            v12 = v10[1];
            v10 = &v10[1];
          } while (v12 == 0x20);
label_48f5:
          v13 = (unsigned int)v12;
        } while (v12 == 9);
        if (9 < v13 - 0x30) {
label_4999:
          __fprintf_chk(stderr,1,dcgettext(NULL,"%s: diff failed: ",5),v4);
          do {
            v12 = *v8;
            v16 = *(unsigned char **)&stderr->field_0x28;
            if (*(unsigned char **)&stderr->field_0x30 <= v16) // branch-flip
              __overflow(stderr,(unsigned int)v12);
            else {
              *(unsigned char **)&stderr->field_0x28 = &v16[1];
              *v16 = v12;
            }
            v16 = &v8[1];
            v12 = *v8;
            v8 = v16;
          } while (v12 != 10);
          exit(2); // no-return
        }
        v15 = 0;
        do {
          v10 = &v10[1];
          v15 = (long)(int)(v13 - 0x30) + v15 * 10;
          v13 = (unsigned int)*v10;
        } while (v13 - 0x30 <= 9);
        *v26 = v15;
        v12 = *v10;
        if (v12 != 0x20) goto label_494d;
        do {
          do {
            v12 = v10[1];
            v10 = &v10[1];
          } while (v12 == 0x20);
label_494d:
        } while (v12 == 9);
        if (v12 != 0x2c) // branch-flip
          v26[1] = v15;
        else {
          v14 = &v10[1];
          v13 = (unsigned int)v10[1];
          if (9 < (unsigned int)v10[1] - 0x30) goto label_4999;
          v9 = 0;
          v10 = v14;
          do {
            v10 = &v10[1];
            v9 = (long)(int)(v13 - 0x30) + v9 * 10;
            v13 = (unsigned int)*v10;
          } while (v13 - 0x30 <= 9);
          v26[1] = v9;
        }
        v12 = *v10;
        if ((v12 == 9) || (v12 == 0x20)) {
          do {
            do {
              v12 = v10[1];
              v10 = &v10[1];
            } while (v12 == 0x20);
          } while (v12 == 9);
        }
        if (v12 != 99) { // branch-flip
          if (v12 != 100) { // branch-flip
            if (v12 != 0x61) goto label_4999;
            v5 = 1;
          }
          else {
            v5 = 3;
          }
        }
        else {
          v5 = 2;
        }
        v14 = &v10[1];
        v12 = v10[1];
        if (v12 != 0x20) goto label_4a2c;
        do {
          do {
            v12 = v14[1];
            v14 = &v14[1];
          } while (v12 == 0x20);
label_4a2c:
          v13 = (unsigned int)v12;
        } while (v12 == 9);
        if (9 < v13 - 0x30) goto label_4999;
        v9 = 0;
        do {
          v14 = &v14[1];
          v9 = (long)(int)(v13 - 0x30) + v9 * 10;
          v13 = (unsigned int)*v14;
        } while (v13 - 0x30 <= 9);
        v26[2] = v9;
        v12 = *v14;
        if (v12 != 0x20) goto label_4a7c;
        do {
          do {
            v12 = v14[1];
            v14 = &v14[1];
          } while (v12 == 0x20);
label_4a7c:
        } while (v12 == 9);
        if (v12 != 0x2c) { // branch-flip
          v26[3] = v9;
          v12 = *v14;
        }
        else {
          v13 = (unsigned int)v14[1];
          if (10 <= (unsigned int)v14[1] - 0x30) goto label_4999;
          v14 = &v14[1];
          v23 = 0;
          do {
            v14 = &v14[1];
            v23 = (long)(int)(v13 - 0x30) + v23 * 10;
            v13 = (unsigned int)*v14;
          } while (v13 - 0x30 <= 9);
          v26[3] = v23;
          v12 = *v14;
          if ((v12 == 9) || (v12 == 0x20)) {
            do {
              do {
                v12 = v14[1];
                v14 = &v14[1];
              } while (v12 == 0x20);
            } while (v12 == 9);
          }
        }
        v8 = &v14[1];
        if (v12 != 10) {
          v8 = v14;
          goto label_4999;
        }
        if (v5 != 2) { // branch-flip
          if (v5 == 3) {
            v26[2] = v9 + 1;
            goto label_4ab7;
          }
          *v26 = v15 + 1;
          if (v5 != 1) goto label_4c78;
label_4b6d:
          v15 = v26[3];
          v9 = v26[2];
          v25 = (v15 - v9) + 1;
          if (0xffffffffffffffe < v25) {
            sub_77d0();
          }
          v26[5] = sub_7350(v25 * 8);
          v23 = sub_7350(v25 * 8);
          v26[7] = v23;
          if (v25) {
            v24 = 0;
            while( true ) {
              v8 = (unsigned char *)sub_4360(v8,(unsigned long *)(v24 * 8 + v26[5]),(long *)(v23 + v24 * 8),v16,0x3e);
              v1 = v24 + 1;
              if (v15 - v9 == v24) break;
              v23 = v26[7];
              v24 = v1;
            }
          }
        }
        else {
label_4c78:
          v15 = *v26;
label_4ab7:
          v9 = v26[1];
          v25 = (v9 - v15) + 1;
          if (0xffffffffffffffe < v25)
            sub_77d0();
          v26[4] = sub_7350(v25 * 8);
          v23 = sub_7350(v25 * 8);
          v26[6] = v23;
          if (v25) {
            v24 = 0;
            while( true ) {
              v8 = (unsigned char *)sub_4360(v8,(unsigned long *)(v24 * 8 + v26[4]),(long *)(v23 + v24 * 8),v16,0x3c);
              v1 = v24 + 1;
              if (v9 - v15 == v24) break;
              v23 = v26[6];
              v24 = v1;
            }
          }
          if (v5 == 2) {
            if (strncmp((char *)v8,"---\n",4))
              sub_4300("invalid diff format; invalid change separator"); // no-return
            v8 = &v8[4];
            goto label_4b6d;
          }
          if (v5 != 3) goto label_4b6d;
        }
        *(long **)v2 = v26;
        v2 = (unsigned int *)&v26[8];
        v13 = v20;
        v3 = v27;
      } while( true );
    }
    v13 = (int)v20 >> 8 & 0xff;
    if ((char)(v20 >> 8) != '~') { // branch-flip
      v22 = "subsidiary program \'%s\' failed (exit status %d)";
      if (v13 == 0x7f)
        v22 = "subsidiary program \'%s\' not found";
    }
    else {
      v22 = "subsidiary program \'%s\' could not be invoked";
    }
  }
  error(2,0,dcgettext(NULL,v22,5),v4,(unsigned long)v13);
  return v11;
}

// Function: sub_4ea0 @ 0x4ea0
void sub_4ea0(void)
{
  return;
}

// Function: sub_4eb0 @ 0x4eb0
void sub_4eb0(int a0)
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
  (*dat_11120)();
  if (a0) // branch-flip
    v6 = dat_11118;
  else {
    v6 = dat_11110;
  }
  v1 = strlen(dat_11108);
  v2 = strlen(v6);
  if (0x1000 <= v1 + v2) { // branch-flip
    write(2,dat_11108,v1);
    write(2,": ",2);
    write(2,v6,v2);
    v5[0] = 10;
    v4 = (char *)0x1;
  }
  else {
    v3 = __mempcpy_chk(v5,dat_11108,v1,(unsigned long)0x1002);
    *(unsigned short *)v3 = s_cdb1._0_2_;
    v4 = mempcpy(&v3[2],v6,v2);
    *v4 = 10;
    v4 = &v4[1 - (long)v5];
  }
  write(2,v5,(unsigned long)v4);
  if (!a0)
    _exit(dat_11020); // no-return
  raise(a0);
  abort(); // no-return
}

// Function: sub_4fd0 @ 0x4fd0
unsigned long sub_4fd0(unsigned long a0,int a1)
{
  if (!a1)
    return 0;
  sub_4eb0(0xb); // no-return
}

// Function: sub_4ff0 @ 0x4ff0
void sub_4ff0(unsigned long a0)
{
  if ((int)a0)
    a0 = (unsigned long)(-(unsigned int)(dat_11100 == 0) & 0xb);
  sub_4eb0(a0); // no-return
}

// Function: sub_5020 @ 0x5020
unsigned long sub_5020(void *a0) // return-dupe
{
  if (!a0)
    a0 = sub_4ea0;
  dat_11120 = a0;
  dat_11118 = dcgettext(NULL,"program error",5);
  dat_11110 = dcgettext(NULL,"stack overflow",5);
  dat_11108 = sub_50d0();
  if (!sub_5ca0(sub_4ff0,0x11140,0x10000)) {
    dat_11100 = sub_5c20(sub_4fd0);
    return 0;
  }
  *__errno_location() = 0x5f;
  return 0xffffffff;
}

// Function: sub_50d0 @ 0x50d0
unsigned long sub_50d0(void)
{
  return __progname;
}

// Function: sub_50e0 @ 0x50e0
void sub_50e0(char *a0)
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
    dat_21140 = a0;
    program_invocation_name = a0;
    return;
  }
  fputs("A NULL argv[0] was passed through an exec system call.\n",stderr);
  abort(); // no-return
}

// Function: sub_5180 @ 0x5180
void sub_5180(struct_3 *a0)
{
  unsigned char v1;
  unsigned char *v2;
  unsigned long v3; // rax
  long v4; // rax
  
  if (!a0->field_0x0) {
    v1 = *a0->field_0x10;
    if (*(unsigned int *)((unsigned long)(v1 >> 5) * 4 + 0xd300) >> (v1 & 0x1f) & 1) {
      a0->field_0x18 = 1;
      v1 = *a0->field_0x10;
      a0->field_0x20 = '\x01';
      a0->field_0x24 = (int)(char)v1;
      goto label_51c1;
    }
    if (!mbsinit((mbstate_t *)&a0->field_0x1[3]))
      __assert_fail("mbsinit (&iter->state)","mbuiter.h",0x8f,"mbuiter_multi_next"); // no-return
    a0->field_0x0 = '\x01';
  }
  v3 = __ctype_get_mb_cur_max();
  v2 = a0->field_0x10;
  v4 = sub_7b30(&a0->field_0x24,v2,sub_6600(v2,v3),(mbstate_t *)&a0->field_0x1[3]);
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
label_51c1:
  a0->field_0xc = '\x01';
  return;
}

// Function: sub_52f0 @ 0x52f0
unsigned long sub_52f0(char *a0,char *a1)
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
  
  v3 = (char *)sub_6670(a1,2);
  v1 = *a0;
  while ((v1 && (v5 = (char *)sub_8560(a0,v3), v5))) {
    if (2 <= __ctype_get_mb_cur_max()) { // branch-flip
      v6[0] = 0;
      v9 = 0;
      v11 = a0;
      if (v5 <= a0) { // branch-flip
label_5421:
        v8 = (unsigned char *)0x1;
      }
      else {
        do {
          v10 = '\0';
          sub_5180(v6);
          if ((v13) && (!v14)) {
            abort(); // no-return, return-dupe
          }
          v11 = &v11[v12];
          v10 = 0;
        } while (v11 < v5);
        if (!v13) goto label_5421;
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
        sub_5180(v7);
        if ((v19) && (!v20)) break;
        if (!v10)
          sub_5180(v6);
        if ((v13) && (!v14))
          abort();
        v11 = &v11[v12];
        v17 = &v17[v18];
      }
      if (!v10)
        sub_5180(v6);
      if (((((v13) && (!v14)) || (!v13)) || (!iswalnum(v14))) && ((char)v8)) {
        free(v3); // return-dupe
        return (unsigned long)v8 & 0xffffffff;
      }
      v11 = v5;
      v6[0] = 0;
      v9 = 0;
      v10 = '\0';
      sub_5180(v6);
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

// Function: sub_5580 @ 0x5580
char * sub_5580(char *a0)
{
  char *v1; // rax
  unsigned long v2; // rax
  char *v3; // rax
  
  v1 = dcgettext(NULL,a0,5);
  if (a0 != v1) {
    if (!sub_52f0(v1,a0)) {
      v2 = strlen(v1);
      v3 = (char *)sub_7350(v2 + 4 + strlen(a0));
      __sprintf_chk(v3,1,0xffffffffffffffff,"%s (%s)",v1,a0);
      return v3;
    }
  }
  return v1;
}

// Function: sub_5610 @ 0x5610
char * sub_5610(char *a0,char *a1)
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
  v4 = (char *)sub_7cc0();
  v6 = v3;
  if (sub_7bc0(v4,"UTF-8")) { // branch-flip
    v7 = (char *)sub_79e0(a1,"UTF-8",v4);
    v5 = strlen(v4);
    v8 = memcpy((void *)sub_7350(v5 + 0xb),v4,v5);
    v9 = (unsigned long *)(v5 + (long)v8);
    *v9 = 0x4c534e4152542f2f;
    *(unsigned short *)&v9[1] = 0x5449;
    *(char *)((long)v9 + 10) = 0;
    a1 = (char *)sub_79e0(a1,"UTF-8",v8);
    free(v8);
    v11 = v7;
    if (a1) { // branch-flip
      if (strchr(a1,0x3f)) {
        free(a1);
        goto label_5813;
      }
      v4 = a1;
      if (!v7) {
        v10 = a1;
        goto label_567a;
      }
      if (!strcmp(v3,a0)) {
        if (v7 == a1)
          return v7;
        free(a1);
        return v7;
      }
    }
    else {
label_5813:
      if (!v7) {
        v11 = NULL;
        a1 = NULL;
        v10 = a0;
        v4 = NULL;
        goto label_567a;
      }
      if (!strcmp(v3,a0))
        return v7;
      a1 = NULL;
      v4 = NULL;
    }
    if (!sub_52f0(v3,a0)) {
      a0 = v7;
label_569f:
      if (((!v11) || (!sub_52f0(v3,v11))) && ((!a1 || (!sub_52f0(v3,a1))))) goto label_571c;
      if (!v7) goto label_56cd;
    }
    free(v7);
  }
  else {
    v2 = strcmp(v3,a0);
    if (a1) {
      v10 = a1;
      v4 = NULL;
      v11 = a1;
label_567a:
      if (!strcmp(v3,a0))
        return v10;
      v1 = sub_52f0(v3,a0);
      v7 = NULL;
      a0 = v10;
      if (v1) goto label_56cd;
      goto label_569f;
    }
    if (!v2)
      return a0;
    if (sub_52f0(v3,a0))
      return v3;
    v4 = NULL;
    v7 = NULL;
label_571c:
    v5 = strlen(v3);
    v6 = (char *)sub_7350(v5 + 4 + strlen(a0));
    __sprintf_chk(v6,1,0xffffffffffffffff,"%s (%s)",v3,a0);
    if (v7)
      free(v7);
  }
label_56cd:
  if (v4)
    free(v4);
  return v6;
}

// Function: sub_5910 @ 0x5910
void sub_5910(unsigned long a0,long a1,long a2)
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
  if (dat_21148) { // branch-flip
    v4 = (*dat_21148)(v1,0);
    if (v4)
      return;
    if (dat_21160) goto label_595e;
  }
  else {
    if (!dat_21160) {
      signal(0xb,0); // return-dupe, tail-call
      return;
    }
label_595e:
    v3 = dat_21168;
    v2 = *(unsigned long *)(a2 + 0xa0);
    if (dat_21168) {
      v6 = __errno_location();
      v4 = *v6;
      v5 = sub_61e0(v3,&v7);
      *v6 = v4;
      if (0 <= v5) {
        if (v7 <= v1) { // branch-flip
          if (v1 <= (unsigned long)(v8 - 1U)) goto label_5a45;
        }
        else if ((*v9)(v1,&v7)) {
label_5a45:
          (*dat_21160)(dat_21158 <= v2 && v2 <= dat_21158 + dat_21150,a2);
        }
      }
    }
  }
  if ((dat_21148) && (v4 = (*dat_21148)(v1,1), v4))
    return;
  signal(0xb,0);
  return;
}

// Function: sub_5aa0 @ 0x5aa0
void sub_5aa0(void)
{
  void *v1; // stack - 0xa8
  sigset_t v2;
  int v3; // stack - 0x20
  
  v1 = sub_5910;
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
  v3 = (-(unsigned int)(dat_21160 == 0) & 0xf8000000) + 0x8000004;
  sigaction(0xb,(sigaction *)&v1,NULL);
}

// Function: sub_5c20 @ 0x5c20
unsigned long sub_5c20(unsigned long a0)
{
  dat_21148 = a0;
  sub_5aa0();
  return 0;
}

// Function: sub_5c40 @ 0x5c40
void sub_5c40(void)
{
  dat_21148 = 0;
  if (dat_21160)
    return;
  signal(0xb,0); // tail-call
}

// Function: sub_5c70 @ 0x5c70
unsigned long sub_5c70(void *a0,unsigned long a1,unsigned long a2,unsigned long a3)
{
  (*a0)(a1,a2,a3);
  return 1;
}

// Function: sub_5ca0 @ 0x5ca0
long sub_5ca0(unsigned long a0,unsigned long a1,unsigned long a2)
{
  int v1; // eax
  long v2; // rax
  unsigned long v3; // stack - 0x58
  char v4 [4];
  unsigned int v5; // stack - 0x50
  unsigned int v6; // stack - 0x4c
  unsigned long v7; // stack - 0x48
  
  if (!dat_21168) {
    v1 = sub_61e0(v4,&v3);
    if (0 <= v1) { // branch-flip
      v2 = CONCAT44(v6,v5) + -1;
      dat_21168 = v2;
    }
    else {
      v2 = dat_21168;
    }
    if (!v2) {
      v2 = 0xffffffff;
      return v2;
    }
  }
  v5 = 0;
  dat_21150 = a2;
  dat_21158 = a1;
  dat_21160 = a0;
  v3 = a1;
  v7 = a2;
  v1 = sigaltstack(&v3,NULL);
  if (v1 <= -1) {
    v2 = 0xffffffff;
    return v2;
  }
  sub_5aa0();
  v2 = 0;
  return v2;
}

// Function: sub_5d80 @ 0x5d80
void sub_5d80(void) // return-dupe
{
  char v1 [8];
  unsigned int v2; // stack - 0x20
  
  dat_21160 = 0;
  if (dat_21148) // branch-flip
    sub_5aa0();
  else {
    signal(0xb,0);
  }
  v2 = 2;
  if (0 <= sigaltstack(v1,NULL))
    return;
  perror("gnulib sigsegv (stackoverflow_deinstall_handler)");
}

// Function: sub_5e10 @ 0x5e10
bool sub_5e10(long a0,struct_0 *a1)
{
  return (unsigned long)(a1->field_0x0 - a0) <= (unsigned long)(a1->field_0x0 - a1->field_0x18) >> 1;
}

// Function: sub_5e30 @ 0x5e30
unsigned long sub_5e30(struct_1 *a0,long *a1)
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
        if (5 < (unsigned int)v1 - 0x61) goto label_5eb0;
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
label_5eb0:
  if (v4) {
    *a1 = v3;
    return 0;
  }
  return 0xffffffff;
}

// Function: sub_5ec0 @ 0x5ec0
unsigned long sub_5ec0(unsigned long a0,long *a1) // return-dupe
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
  
  v1 = dat_21170;
  v3 = a0 * 2 - *a1;
  if (a0 < v3)
    return 0;
  v4 = (v3 / dat_21170) * dat_21170;
  v5 = (((unsigned long)(*a1 - 1U) / dat_21170 + 1) * dat_21170 - v4) / dat_21170;
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

// Function: sub_6000 @ 0x6000
void * sub_6000(void *a0)
{
  unsigned long v1;
  unsigned long v2;
  char v3 [1032];
  unsigned long v4; // rsi
  unsigned long v5; // rsi
  void *v6; // r14
  
  v2 = 0x400;
  while( true ) {
    v1 = dat_21170;
    if (!a0)
      return NULL;
    if ((unsigned long)a0 / dat_21170 < v2)
      v2 = (unsigned long)a0 / dat_21170;
    v4 = dat_21170 * v2;
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

// Function: sub_60e0 @ 0x60e0
void * sub_60e0(long a0) // return-dupe
{
  unsigned long v1;
  unsigned long v2;
  unsigned long v3;
  char v4 [1032];
  void *v5; // rdi
  
  v1 = dat_21170;
  v5 = (void *)(a0 + dat_21170);
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

// Function: sub_61e0 @ 0x61e0
unsigned long sub_61e0(char *a0,unsigned long *a1)
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
        if (0x104a <= v12) { // branch-flip
          v4 = (int)read(v3,v16,(unsigned long)v12);
          if (0 <= v4) {
            if (!v4) goto label_6408;
            if (v12 < (char *)(long)(v4 + 0x1049)) goto label_62a0;
            v5 = (long)v4;
            while( true ) {
              v4 = (int)read(v3,&v16[v5],(long)v12 - v5);
              if (v4 <= -1) break;
              if ((unsigned long)((long)v12 - v5) < (unsigned long)(long)(v4 + 0x1049)) goto label_62a0;
              if (!v4) {
                close(v3);
                v10 = NULL;
                goto label_6470;
              }
              v5 = v4 + v5;
            }
          }
          if (*__errno_location() != 4) {
label_6408:
            close(v3);
            goto label_6340;
          }
        }
        else {
label_62a0:
          if (v11) { // branch-flip
            v12 = (char *)((long)v12 * 2);
            if (!v12) goto label_6408;
            if (v17)
              munmap(v17,(unsigned long)v18);
          }
          else {
            v11 = (char *)(long)getpagesize();
            for (v12 = v11; v12 <= (char *)0x1049; v12 = (char *)((long)v12 * 2)) {
            }
          }
          v17 = (char *)mmap(0,v12,3,0x22,0xffffffff,0);
          if (v17 == (char *)0xffffffffffffffff) {
            close(v3);
            goto label_6354;
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
label_6340:
    if (v17)
      munmap(v17,(unsigned long)v18);
  }
label_6354:
  v8 = dat_21170;
  if (!dat_21170) {
    v8 = (unsigned long)getpagesize();
    dat_21170 = v8;
  }
  v5 = ((unsigned long)a0 / v8) * v8;
  *a1 = sub_6000(v5);
  a1[1] = sub_60e0(v5);
  v6 = sub_5ec0;
label_6553:
  a1[2] = v6;
  return 0;
label_6470:
  if (((((int)sub_5e30() < 0) || (v15 == v5)) || (v12 = &v16[v15], v15 = v15 + 1, *v12 != '-')) || ((int)sub_5e30() < 0)) goto label_6340;
  do {
    if (v5 == v15) break;
    v12 = &v16[v15];
    v15 += 1;
  } while (*v12 != '\n');
  v7 = v14;
  if (v13 <= v19) { // branch-flip
    v1 = &v14[-1];
    if (v1 < &v20[-1]) goto label_6520;
    v2 = v10;
    if ((v13 < v19) && ((v2 = v19, v13 <= a0 && (a0 <= &v19[-1])))) {
      v14 = v19;
      goto label_652e;
    }
    v10 = v2;
    if (v1 <= &v20[-1]) // branch-flip
      v7 = v10;
    else if ((v20 <= a0) && (a0 <= v1)) {
      v13 = v20;
      goto label_652e;
    }
  }
  else {
label_6520:
    if ((v13 <= a0) && (a0 <= &v14[-1])) {
label_652e:
      *a1 = v13;
      a1[1] = v14;
      a1[3] = v10;
      if (v17)
        munmap(v17,(unsigned long)v18);
      v6 = sub_5e10;
      goto label_6553;
    }
  }
  v10 = v7;
  goto label_6470;
}

// Function: sub_6600 @ 0x6600
void * sub_6600(void *a0,unsigned long a1)
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

// Function: sub_6640 @ 0x6640
void sub_6640(void)
{
  __assert_fail("iter->cur.wc == 0","mbiter.h",0xa3,"mbiter_multi_next"); // no-return
}

// Function: sub_6670 @ 0x6670
unsigned char * sub_6670(char *a0,int a1)
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
    sub_77d0(); // no-return
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
label_6ac4:
          v14 = sub_7b30(v7,v8,(long)v9 - (long)v8,&v6);
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
                  sub_6640(); // no-return, return-dupe
                }
              }
              v15 = '\x01';
              if (mbsinit(&v6))
                v3 = 0;
              goto label_6a85;
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
label_6863:
          v8 = v13;
          break;
        }
        if (!(*(unsigned int *)((unsigned long)(*v8 >> 5) * 4 + 0xd300) >> (*v8 & 0x1f) & 1)) {
          if (mbsinit(&v6)) {
            v3 = 1;
            goto label_6ac4;
          }
          __assert_fail("mbsinit (&iter->state)","mbiter.h",0x87,"mbiter_multi_next"); // return-dupe, no-return
        }
        v14 = 1;
        v7[0] = (unsigned int)(char)*v8;
        v15 = '\x01';
label_6a85:
        v12 = 1;
        if (!iswspace(v7[0])) goto label_6863;
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
label_67da:
          v14 = sub_7b30(v7,v8,(long)v9 - (long)v8,&v6);
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
                  sub_6640();
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
          if (!(*(unsigned int *)((unsigned long)(*v8 >> 5) * 4 + 0xd300) >> (*v8 & 0x1f) & 1)) {
            if (!mbsinit(&v6))
              __assert_fail("mbsinit (&iter->state)","mbiter.h",0x87,"mbiter_multi_next");
            v3 = 1;
            goto label_67da;
          }
          v14 = 1;
          v7[0] = (unsigned int)(char)*v8;
          v15 = '\x01';
        }
        v12 = 1;
        if (v10) { // branch-flip
          if (v10 != 1) { // branch-flip
            if ((v10 != 2) || (!v15)) goto label_670b;
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
            if (!v15) goto label_670b;
            v10 = 1;
            iswspace(v7[0]);
          }
        }
        else {
label_670b:
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

// Function: sub_6c10 @ 0x6c10
long sub_6c10(FILE *a0,char *a1,char *a2,char *a3,unsigned long *a4,unsigned long a5)
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
label_6d3d:
    v1 = CONCAT44(dat_4,__fprintf_chk(a0,1,v2,v4,v3,v5,v6,v7,v8,v9,v10));
label_6d76:
    return v1;
  }
  v1 = (long)*(int *)(a5 * 4 + 0xd208) + 0xd208;
  switch(a5) {
    case 0:
      goto label_6d76;
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
      goto label_6d3d;
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

// Function: sub_7090 @ 0x7090
void sub_7090(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,long *a4)
{
  long v1;
  long v2;
  
  v2 = 0;
  v1 = *a4;
  while (v1) {
    v2 += 1;
    v1 = a4[v2];
  }
  sub_6c10(); // tail-call
}

// Function: sub_70b0 @ 0x70b0
void sub_70b0(FILE *a0,char *a1,char *a2,char *a3,struct_4 *a4)
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
  sub_6c10(a0,a1,a2,a3);
}

// Function: sub_7150 @ 0x7150
void sub_7150(FILE *a0,char *a1,char *a2,unsigned long a3,unsigned long a4,unsigned long a5)
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
  sub_6c10(a0,a1,a2);
}

// Function: sub_7230 @ 0x7230
void sub_7230(void)
{
  fputs_unlocked("\n",stdout);
  __printf_chk(1,dcgettext(NULL,"Report bugs to: %s\n",5),"bug-diffutils@gnu.org");
  __printf_chk(1,dcgettext(NULL,"%s home page: <%s>\n",5),"GNU diffutils","https://www.gnu.org/software/diffutils/");
  __printf_chk(1,dcgettext(NULL,"General help using GNU software: <%s>\n",5),"https://www.gnu.org/gethelp/"); // tail-call
}

// Function: sub_72e0 @ 0x72e0
void sub_72e0(unsigned long a0,unsigned long a1)
{
  if (reallocarray(NULL,a0,a1))
    return;
  sub_77d0(); // no-return
}

// Function: sub_7310 @ 0x7310
void sub_7310(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!reallocarray(a0,a1,a2)) && ((!a0 || ((a1 && (a2))))))
    sub_77d0(); // no-return
}

// Function: sub_7350 @ 0x7350
void sub_7350(unsigned long a0)
{
  if (malloc(a0))
    return;
  sub_77d0(); // no-return
}

// Function: sub_7370 @ 0x7370
void sub_7370(void)
{
  sub_7350(); // tail-call
}

// Function: sub_7380 @ 0x7380
void sub_7380(unsigned long a0)
{
  if (malloc(a0))
    return;
  sub_77d0(); // no-return
}

// Function: sub_73a0 @ 0x73a0
void sub_73a0(void *a0,unsigned long a1)
{
  if ((!realloc(a0,a1)) && ((!a0 || (a1))))
    sub_77d0(); // no-return
}

// Function: sub_73e0 @ 0x73e0
void sub_73e0(void *a0,unsigned long a1)
{
  if (realloc(a0,a1 | a1 == 0))
    return;
  sub_77d0(); // no-return
}

// Function: sub_7410 @ 0x7410
void sub_7410(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!reallocarray(a0,a1,a2)) && ((!a0 || ((a1 && (a2))))))
    sub_77d0(); // no-return
}

// Function: sub_7450 @ 0x7450
void sub_7450(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!a1) || (!a2)) {
    a1 = 1;
    a2 = 1;
  }
  if (reallocarray(a0,a1,a2))
    return;
  sub_77d0(); // no-return
}

// Function: sub_7490 @ 0x7490
void sub_7490(void *a0,unsigned long *a1)
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
  sub_77d0(); // no-return
}

// Function: sub_7520 @ 0x7520
void sub_7520(void *a0,unsigned long *a1,unsigned long a2)
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
  sub_77d0(); // no-return
}

// Function: sub_75b0 @ 0x75b0
void sub_75b0(void *a0,long *a1,long a2,long a3,long a4)
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
    if (0x80 <= v3) goto label_766e;
    v4 = 0x80;
  }
  v2 = v4 / a4;
  v3 = v4 - v4 % a4;
label_766e:
  if (!a0)
    *a1 = 0;
  if (((a2 <= v2 - v1) || ((v2 = v1 + a2, !SCARRY8(v1,a2) && (((v2 <= a3 || (a3 <= -1)) && (v3 = v2 * a4, SEXT816(v3) == SEXT816(v2) * SEXT816(a4))))))) && ((realloc(a0,v3) || ((a0 && (!v3)))))) {
    *a1 = v2;
    return;
  }
  sub_77d0(); // no-return
}

// Function: sub_76b0 @ 0x76b0
void sub_76b0(unsigned long a0,unsigned long a1)
{
  if (calloc(a0,a1))
    return;
  sub_77d0(); // no-return
}

// Function: sub_76d0 @ 0x76d0
void sub_76d0(unsigned long a0)
{
  sub_76b0(a0,1); // tail-call
}

// Function: sub_76e0 @ 0x76e0
void sub_76e0(unsigned long a0,unsigned long a1)
{
  if (calloc(a0,a1))
    return;
  sub_77d0(); // no-return
}

// Function: sub_7700 @ 0x7700
void sub_7700(unsigned long a0)
{
  sub_76e0(a0,1); // tail-call
}

// Function: sub_7710 @ 0x7710
void sub_7710(void *a0,unsigned long a1)
{
  memcpy((void *)sub_7350(a1),a0,a1); // tail-call
}

// Function: sub_7740 @ 0x7740
void sub_7740(void *a0,unsigned long a1)
{
  memcpy((void *)sub_7380(a1),a0,a1); // tail-call
}

// Function: sub_7770 @ 0x7770
void sub_7770(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)sub_7380(a1 + 1);
  *(char *)((long)v1 + a1) = 0;
  memcpy(v1,a0,a1); // tail-call
}

// Function: sub_77b0 @ 0x77b0
void sub_77b0(char *a0)
{
  sub_7710(a0,strlen(a0) + 1); // tail-call
}

// Function: sub_77d0 @ 0x77d0
void sub_77d0(void)
{
  error(dat_11020,0,"%s",dcgettext(NULL,"memory exhausted",5));
  abort(); // no-return
}

// Function: sub_7810 @ 0x7810
void sub_7810(char *a0,char *a1,FILE *a2)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  char *v3; // rax
  
  if (freopen(a0,a1,a2))
    return;
  if (!a0) {
    if (stdin != a2) { // branch-flip
      if (stdout != a2) { // branch-flip
        if (stderr != a2) // branch-flip
          a0 = dcgettext(NULL,"unknown stream",5);
        else {
          a0 = dcgettext(NULL,"stderr",5);
        }
      }
      else {
        a0 = dcgettext(NULL,"stdout",5);
      }
    }
    else {
      a0 = dcgettext(NULL,"stdin",5);
    }
  }
  v1 = sub_af40(1,a1);
  v2 = sub_af40(0,a0);
  v3 = dcgettext(NULL,"failed to reopen %s with mode %s",5);
  error(dat_11020,*__errno_location(),v3,v2,v1); // tail-call
}

// Function: sub_7940 @ 0x7940
void sub_7940(void)
{
  int v1; // eax
  
  v1 = sub_b040();
  if (!v1)
    return;
  error(dat_11020,v1,dcgettext(NULL,"standard file descriptors",5)); // tail-call
}

// Function: sub_7980 @ 0x7980
int sub_7980(unsigned long a0,long a1,unsigned long a2,long *a3,unsigned long *a4)
{
  int v1; // eax
  
  v1 = sub_b0f0(a0,a1,a2,a3,a4);
  if (v1 < 0) {
    if (*__errno_location() == 0xc)
      sub_77d0(); // no-return
  }
  return v1;
}

// Function: sub_79b0 @ 0x79b0
long sub_79b0(void)
{
  long v1; // rax
  
  v1 = sub_b370();
  if (!v1) {
    if (*__errno_location() == 0xc)
      sub_77d0(); // no-return
  }
  return v1;
}

// Function: sub_79e0 @ 0x79e0
long sub_79e0(char *a0,long a1,long a2)
{
  long v1; // rax
  
  v1 = sub_b5c0(a0,a1,a2);
  if (!v1) {
    if (*__errno_location() == 0xc)
      sub_77d0(); // no-return
  }
  return v1;
}

// Function: sub_7a10 @ 0x7a10
long sub_7a10(int a0,void *a1,long a2)
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

// Function: sub_7ac0 @ 0x7ac0
void * sub_7ac0(void *a0,void *a1,void *a2)
{
  char v1 [16];
  char v2 [16];
  void *v3;
  void *v4; // rdx
  void *v5;
  
  if (a0) {
    if (a1) {
      v3 = a0;
      v5 = a1;
      while (v4 = (void *)((unsigned long)v3 % (unsigned long)v5), v4) {
        v3 = v5;
        v5 = v4;
      }
      v1._8_8_ = 0;
      v1._0_8_ = v5;
      v2._8_8_ = 0;
      v2._0_8_ = a0;
      v1._0_16_ = ZEXT816(a1) * ZEXT816(SUB168(v2._0_16_ / v1._0_16_,0));
      v3 = SUB168(v1._0_16_,0);
      if ((v3 <= a2) && (!SUB168(v1._0_16_,8)))
        a0 = v3;
    }
    return a0;
  }
  v3 = _DT_INIT;
  if (a1)
    v3 = a1;
  return v3;
}

// Function: sub_7b30 @ 0x7b30
unsigned long sub_7b30(unsigned int *a0,char *a1,unsigned long a2,mbstate_t *a3)
{
  unsigned long v1; // rax
  long v2;
  
  if (!a0)
    a0 = &v2;
  v1 = mbrtowc(a0,a1,a2,a3);
  if (((0xfffffffffffffffe <= v1) && (a2)) && (!sub_7c30(0))) {
    v1 = 1;
    *a0 = (unsigned int)(unsigned char)*a1;
  }
  return v1;
}

// Function: sub_7bc0 @ 0x7bc0
int sub_7bc0(long a0,long a1)
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
label_7bdd:
        v5 += 0x20;
        v2 += 0x20;
      }
      if (!v1) break;
    }
    else {
      v1 += 0x20;
      v4 += 0x20;
      if (v5 - 0x41 <= 0x19) goto label_7bdd;
    }
    v3 += 1;
  } while (v4 == v2);
  return v1 - v5;
}

// Function: sub_7c30 @ 0x7c30
bool sub_7c30(unsigned long a0)
{
  char v1 [264];
  bool v2; // r12b
  
  v2 = 0;
  if ((!sub_af80(a0,v1,0x101)) && (strcmp(v1,"C")))
    v2 = strcmp(v1,"POSIX") != 0;
  return v2;
}

// Function: sub_7cc0 @ 0x7cc0
char * sub_7cc0(void) // early-return
{
  char *v1; // rax
  
  v1 = nl_langinfo(0xe);
  if (!v1)
    return "ASCII";
  if (*v1)
    return v1;
  return "ASCII";
}

// Function: sub_7d00 @ 0x7d00
int sub_7d00(unsigned int a0)
{
  int v1; // eax
  
  v1 = wcwidth(a0);
  if (0 <= v1)
    return v1;
  return (unsigned long)(iswcntrl(a0) == 0);
}

// Function: sub_7d30 @ 0x7d30
void sub_7d30(struct_5 *a0,struct_6 *a1) // return-dupe
{
  char v1;
  long v2;
  struct_6 *v3;
  
  v3 = (struct_6 *)a1->field_0x0;
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

// Function: sub_7d90 @ 0x7d90
unsigned int sub_7d90(unsigned char a0)
{
  return *(unsigned int *)((unsigned long)(a0 >> 5) * 4 + 0xd300) >> (a0 & 0x1f) & 1;
}

// Function: sub_7db0 @ 0x7db0
unsigned long sub_7db0(char *a0,long a1,unsigned long a2,unsigned long *a3)
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
      v9 = sub_b8f0(v9);
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
              goto label_7ea8;
            }
            v3 -= *(long *)(v9 + v3 * 8);
            v2 = *(char *)(a1 + v3);
          }
          v3 += 1;
          *(unsigned long *)(v9 + v1 * 8) = v1 - v3;
label_7ea8:
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
      *(unsigned long *)&v8[-8] = 0x7efe;
      sub_b940(v9);
      return 1;
    }
  }
  return 0;
}

// Function: sub_7f50 @ 0x7f50
void sub_7f50(struct_3 *a0)
{
  unsigned char v1;
  unsigned char *v2;
  unsigned long v3; // rax
  long v4; // rax
  
  if (!a0->field_0x0) {
    v1 = *a0->field_0x10;
    if (*(unsigned int *)((unsigned long)(v1 >> 5) * 4 + 0xd300) >> (v1 & 0x1f) & 1) {
      a0->field_0x18 = 1;
      v1 = *a0->field_0x10;
      a0->field_0x20 = '\x01';
      a0->field_0x24 = (int)(char)v1;
      goto label_7f91;
    }
    if (!mbsinit((mbstate_t *)&a0->field_0x1[3]))
      __assert_fail("mbsinit (&iter->state)","mbuiter.h",0x8f,"mbuiter_multi_next"); // no-return
    a0->field_0x0 = '\x01';
  }
  v3 = __ctype_get_mb_cur_max();
  v2 = a0->field_0x10;
  v4 = sub_7b30(&a0->field_0x24,v2,sub_6600(v2,v3),(mbstate_t *)&a0->field_0x1[3]);
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
label_7f91:
  a0->field_0xc = '\x01';
  return;
}

// Function: sub_80c0 @ 0x80c0
char sub_80c0(void *a0,unsigned char *a1,unsigned long *a2)
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
  v8 = (void *)sub_b970(a1);
  v24 = v8;
  v3 = ZEXT816(v8);
  v8 = SUB168(ZEXT816(0x38) * v3,0);
  if ((0 <= (long)v8) && (!SUB168(ZEXT816(0x38) * v3,8))) {
    v18 = (long)v24 * 0x38;
    if (0xfa1 <= v18) { // branch-flip
      v8 = (unsigned long *)sub_b8f0(v18);
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
        *(unsigned long *)((long)v16 + -8) = 0x824e;
        sub_7f50(v27);
        v25 = v38;
        if ((v38) && (!v39)) break;
        if (v36 != v10) // branch-flip
          v8[-3] = v36;
        else {
          *(unsigned long *)((long)v16 + -8) = 0x827b;
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
label_82c0:
        v20 = &v4[6];
        v1 = *(char *)&v4[8];
        do {
          v8 = &v19[v21 * 6];
          if ((v1) && (*(char *)&v8[2])) {
            if (*(int *)((long)v4 + 0x44) == *(int *)((long)v8 + 0x14)) {
label_8316:
              v21 += 1;
              v8 = (long)((long)v26 - v21);
              v19[(long)((long)v5 * 6 + (long)v26)] = v8;
              goto label_832b;
            }
          }
          else {
            v18 = v4[7];
            if (v18 == v8[1]) {
              v22 = (void *)*v8;
              v2 = (void *)*v20;
              *(unsigned long *)((long)v16 + -8) = 0x8312;
              v7 = memcmp(v2,v22,v18);
              if (!v7) goto label_8316;
            }
          }
          if (!v21) goto label_8520;
          v21 -= v19[(long)v5 * 6 + v21];
        } while( true );
      }
label_834b:
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
label_83a0:
      v6 = v1;
      if (!v6) goto label_83ef;
      if (!v38) goto label_8401;
label_83aa:
      if (v39) {
        v8 = &v19[(long)v22 * 6];
        if (!*(char *)&v8[2]) goto label_840c;
        if (*(int *)((long)v8 + 0x14) != v39) goto label_8416;
        v17 = v36;
        v18 = v37;
label_83d3:
        v36 = (void *)((long)v17 + v18);
        v35 = '\0';
        v22 = (void *)((long)v22 + 1);
        if (v24 != v22) {
label_83ef:
          do {
            *(unsigned long *)((long)v16 + -8) = 0x83fb;
            sub_7f50(v27);
            if (v38) goto label_83aa;
label_8401:
            v8 = &v19[(long)v22 * 6];
label_840c:
            v18 = v37;
            if (v8[1] == v37) {
              v2 = (void *)*v8;
              v26 = v36;
              *(unsigned long *)((long)v16 + -8) = 0x8486;
              v7 = memcmp(v2,v36,v37);
              v17 = v26;
              if (!v7) goto label_83d3;
            }
label_8416:
            if (v22) {
              v21 = v19[(long)((long)v5 * 6 + (long)v22)];
              v8 = (void *)((long)v22 - v21);
              v1 = v35;
              v26 = v8;
              if (!v21) goto label_83a0;
              v22 = v8;
              if (!v29) goto label_84f5;
              goto label_84ce;
            }
            if (!v29) {
              *(unsigned long *)((long)v16 + -8) = 0x8430;
              sub_7f50(v12);
            }
            if ((v32) && (!v33)) goto label_854e;
            v29 = '\0';
            v30 = (void *)((long)v30 + v31);
            v35 = '\0';
            v36 = (void *)((long)v36 + v37);
          } while( true );
        }
        *v23 = (unsigned long)v30;
      }
      *(unsigned long *)((long)v16 + -8) = 0x8549;
      sub_b940(v19);
      v6 = v25;
      return v6;
    }
  }
  v6 = '\0';
  return v6;
label_8520:
  v19[(long)((long)v5 * 6 + (long)v26)] = v26;
label_832b:
  v26 = (void *)((long)v26 + 1);
  v4 = v20;
  if (v24 == v26) goto label_834b;
  goto label_82c0;
label_84ce:
  v26 = v22;
  if ((v32) && (!v33)) {
label_854e:
    *(unsigned long *)((long)v16 + -8) = 0x8553; // no-return
    abort();
  }
  v29 = '\0';
  v30 = (void *)((long)v30 + v31);
  v21 -= 1;
  if (!v21) goto label_8510;
label_84f5:
  *(unsigned long *)((long)v16 + -8) = 0x84fd;
  sub_7f50(v12);
  v22 = v26;
  goto label_84ce;
label_8510:
  v1 = v35;
  goto label_83a0;
}

// Function: sub_8560 @ 0x8560
char * sub_8560(char *a0,char *a1) // early-return, return-dupe
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
    sub_7f50(v11);
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
        sub_7f50(&v13);
        if ((char)v38) { // branch-flip
          if (!v39)
            return NULL;
          v4 = 10 <= v18 & v19;
          if (v4) {
            if (v18 * 5 <= v8) { // branch-flip
label_8ae0:
              v17 = v8 - v20;
              if (v17) { // branch-flip
                if (!v28) goto label_8b2d;
                while ((!v31 || (v32))) {
                  v28 = '\0';
                  v29 = &v29[v30];
                  v17 -= 1;
                  if (!v17) goto label_8b60;
label_8b2d:
                  sub_7f50(v15);
                }
                if (v28) goto label_8bf0;
label_8b60:
                sub_7f50(v15);
              }
              else if (!v28) goto label_8b60;
              v19 = v31;
              if (v31) { // branch-flip
                if (!v32) {
label_8bf0:
                  v19 = sub_80c0(a0,a1,&v10);
                  if (v19)
                    return v10;
                }
              }
              else {
                v19 = 1;
              }
              v20 = v8;
              if (!(char)v38) goto label_88b6;
            }
            else {
              v19 = v4;
            }
          }
          v9 = v8 + 1;
          if (!v25) goto label_88b6;
          if (v39 == v26) goto label_88f6;
label_8848:
          v6 = v36;
          v16 = v37;
        }
        else {
          v4 = 10 <= v18 & v19;
          if (v4) { // branch-flip
            if (v18 * 5 <= v8) goto label_8ae0;
            v19 = v4;
          }
          else {
          }
label_88b6:
          v16 = v37;
          v6 = v36;
          v9 = v8 + 1;
          if (v37 == v24) {
            if (!memcmp(v36,v23,v37)) {
label_88f6:
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
              sub_7f50(v12);
              if ((v55) && (!v56))
                abort(); // no-return
              v53 = &v53[v54];
              do {
                v9 += 1;
                v52 = 0;
                sub_7f50(v12);
                if ((v55) && (!v56))
                  return v36;
                if (!v43[4])
                  sub_7f50(&v14);
                v16 = v54;
                v1 = v53;
                v8 = v46;
                v6 = v45;
                if ((char)v47) { // branch-flip
                  if (!v47._4_4_)
                    return NULL;
                  if (!v55) goto label_8a6e;
                  if (v47._4_4_ != v56) goto label_8848;
                }
                else {
label_8a6e:
                  if ((v46 != v54) || (memcmp(v45,v53,v46))) goto label_8848;
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
              if (*v6) goto label_868d;
            }
            v5 = sub_7db0(v1,a1,strlen(a1),&v10);
            if ((char)v5)
              return v10;
            v6 = NULL;
          }
          else {
label_868d:
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

// Function: sub_8c30 @ 0x8c30
char * sub_8c30(char *a0,int a1) // return-dupe x3
{
  unsigned char *v1; // rax
  
  v1 = (unsigned char *)sub_7cc0();
  if ((*v1 & 0xdf) != 0x55) { // branch-flip
    if (((((*v1 & 0xdf) == 0x47) && ((v1[1] & 0xdf) == 0x42)) && (v1[2] == 0x31)) && (((v1[3] == 0x38 && (v1[4] == 0x30)) && ((v1[5] == 0x33 && ((v1[6] == 0x30 && (!v1[7])))))))) {
      if (*a0 == '`')
        return (char *)0xd33c;
      return (char *)0xd337;
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

// Function: sub_8d10 @ 0x8d10
long * sub_8d10(char *a0,unsigned long a1,char *a2,unsigned long a3,int a4,unsigned int a5,long a6,char *a7,char *a8)
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
label_8d80:
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
label_9226:
      v2 = 0;
      goto label_91e0;
    case 2:
      if (!v18) {
        v2 = 0;
        goto label_9341;
      }
      v25 = 1;
      v2 = 0;
      v13 = 0;
      v30 = 1;
      v31 = "\'";
      break;
    case 3:
      v2 = 1;
label_91e0:
      v25 = 1;
      v13 = 0;
      a4 = 2;
      v30 = 1;
      v31 = "\'";
      break;
    case 4:
      if (!v18) {
        v2 = 1;
        goto label_9341;
      }
      goto label_9226;
    case 5:
      if (!v18) goto label_9181;
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
          v10 = (char *)sub_8c30("`",a4);
          v36 = (char *)v10;
        }
        v10 = (char *)dcgettext(NULL,"\'",5);
        v35 = (char *)v10;
        if (v10 == (long *)0xce32) {
          v10 = (char *)sub_8c30("\'",a4);
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
label_8e6b:
  do {
label_8e7e:
    v23 = 0;
    a1 = v14;
label_8e88:
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
          if ((a3 < v23 + v30) || (v9 = memcmp(v12,v31,v30), v9)) goto label_97f0;
          if (!v25) {
            v8 = *v12;
            v19 = (unsigned long)v8;
            v21 = v19;
            if ((char)v8 <= '?') {
              switch(v8) {
                case 0:
                  goto label_93ec;
                default:
                  goto label_90c3;
                case 7:
                  goto label_93d7;
                case 8:
                  goto label_93a6;
                case 9:
                  goto label_9473;
                case 10:
                  goto label_93cd;
                case 0xb:
                  goto label_94a0;
                case 0xc:
                  goto label_938b;
                case 0xd:
                  goto label_92dd;
                case 0x20:
                  goto label_94ad;
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
                  goto label_9076;
                case 0x23:
                  goto label_9450;
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
                  goto label_8fa8;
                case 0x27:
                  goto label_92c2;
                case 0x3f:
                  goto label_9281;
                
              }
            }
            v5 = v22;
            if ((char)v8 <= 'z') {
              if (v8 == 0x40) goto label_90c3;
              v10 = (unsigned long)(1L << (v8 + 0xbf & 0x3f));
              if ((unsigned long)v10 & 0x3ffffff53ffffff) goto label_8fa8;
              if (!((unsigned long)v10 & 0xa4000000)) goto label_9894;
              goto label_97d8;
            }
            if (v8 == 0x7d) goto label_9960;
            if ((char)v8 <= '}') {
              v21 = 0x7b;
              if (v8 == 0x7b) goto label_97ba;
              v21 = 0x7c;
              if (v8 == 0x7c) goto label_97d8;
              goto label_90c3;
            }
            if (v8 != 0x7e) goto label_90c3;
label_9914:
            if (!v23) {
              v8 = a4 == 2;
              v21 = 0x7e;
              v26 = v3;
              goto label_9139;
            }
            v21 = 0x7e;
            goto label_9922;
          }
          goto label_90ac;
        }
label_97f0:
        v8 = *v12;
        v19 = (unsigned long)v8;
        v21 = v19;
        if ('?' < (char)v8) {
          if ('z' < (char)v8) {
            if (v8 != 0x7d) { // branch-flip
              if ('~' <= (char)v8) {
                v22 = 0;
                if (v8 == 0x7e) goto label_9914;
                goto label_90c0;
              }
              v21 = 0x7b;
              v22 = 0;
              if (v8 != 0x7b) {
                v21 = 0x7c;
                v22 = 0;
                v5 = 0;
                if (v8 != 0x7c) goto label_90c3;
                goto label_97d8;
              }
            }
            else {
              v21 = 0x7d;
              v22 = 0;
            }
label_97ba:
            if (a3 == 0xffffffffffffffff) goto label_996f;
label_97c8:
            v5 = v22;
            if (a3 == 1) goto label_9455;
label_97d8:
            v8 = a4 == 2;
            v26 = v3;
            v24 = 0;
            v22 = v5;
            goto label_9139;
          }
          if (v8 == 0x40) goto label_90c0;
          v10 = (unsigned long)(1L << (v8 + 0xbf & 0x3f));
          v22 = 0;
          if ((unsigned long)v10 & 0x3ffffff53ffffff) goto label_8fa8;
          v22 = 0;
          v5 = 0;
          if ((unsigned long)v10 & 0xa4000000) goto label_97d8;
label_9894:
          if ((char)v19 == '\\') {
            if (((bool)(v2 & v25)) && (v30)) goto label_98ba;
            v17 = 0x5c;
            goto label_92e2;
          }
label_90c3:
          v17 = (unsigned char)v19;
          if (v33 != 1) goto label_9643;
label_90d0:
          v10 = __ctype_b_loc();
          v21 = 1;
          v26 = (*(unsigned char *)(*v10 + 1 + v19 * 2) & 0x40) == 0;
          v27 = (bool)(v26 & v2);
          v26 = !v26;
          goto label_911e;
        }
        switch(v8) {
          case 0:
            goto label_93de;
          default:
label_90c0:
            v22 = 0;
            goto label_90c3;
          case 7:
label_93d7:
            v17 = 0x61;
            goto label_93b7;
          case 8:
label_93a6:
            v17 = 0x62;
            goto label_93b7;
          case 9:
            v22 = 0;
label_9473:
            v21 = 9;
            v17 = 0x74;
            goto label_9480;
          case 10:
label_93cd:
            v17 = 0x6e;
            break;
          case 0xb:
label_94a0:
            v17 = 0x76;
            break;
          case 0xc:
label_938b:
            v17 = 0x66;
label_93b7:
            if (v25) {
              v2 = 1;
              goto label_930f;
            }
            v24 = 0;
            goto label_902e;
          case 0xd:
label_92dd:
            v17 = 0x72;
            break;
          case 0x20:
            v22 = 0;
label_94ad:
            v21 = 0x20;
            goto label_8fa8;
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
            goto label_9079;
          case 0x23:
            v21 = 0x23;
            v22 = 0;
            goto label_9455;
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
label_8fa8:
            v8 = 0;
            goto label_8fb8;
          case 0x27:
            v22 = 0;
            goto label_92c2;
          case 0x3f:
            v22 = 0;
            goto label_9281;
          
        }
        goto label_92e2;
      }
      v17 = *v12;
      v19 = (unsigned long)v17;
      if ('?' < (char)v17) {
        if ('{' <= (char)v17) {
          if (v17 == 0x7d) {
label_9960:
            v21 = 0x7d;
            if (a3 != 0xffffffffffffffff) goto label_97c8;
label_996f:
            v5 = v22;
            if (a2[1]) goto label_97d8;
            goto label_9455;
          }
          if ((char)v17 <= '}') {
            v21 = 0x7b;
            if (v17 != 0x7b) {
              if (v17 == 0x7c) goto label_9076;
              goto label_90c3;
            }
            goto label_97ba;
          }
          v21 = 0x7e;
          if (v17 == 0x7e) goto label_9455;
          if (v33 == 1) goto label_90d0;
label_9643:
          v15 = 0;
          if (a3 == 0xffffffffffffffff) {
            v10 = (unsigned long)strlen(a2);
            a3 = (unsigned long)v10;
          }
          v21 = 0;
          do {
            v19 = v23 + v21;
            v10 = (long)sub_7b30(&v16,&a2[v19],a3 - v19,&v15);
            v7 = (long)v10;
            v24 = v26;
            if (!v10) break;
            if (v10 == (long *)0xffffffffffffffff) {
              v24 = 0;
              goto label_9bab;
            }
            if (v10 == (long *)0xfffffffffffffffe) {
              v10 = (long *)v19;
              if (v19 >= a3) goto label_9f2f;
              goto label_9f25;
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
                if ((v8 <= 0x21) && (0x20000002bU >> (v19 & 0x3f) & 1)) goto label_977d;
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
          goto label_9bab;
        }
        if (v17 == 0x40) goto label_90c3;
        v10 = (unsigned long)(1L << (v17 + 0xbf & 0x3f));
        if ((unsigned long)v10 & 0x3ffffff53ffffff) goto label_9130;
        if ((unsigned long)v10 & 0xa4000000) goto label_9076;
        if (v17 != 0x5c) goto label_90c3;
        if (a4 == 2) {
          if (!v25) goto label_98ba;
          goto label_90a3;
        }
        if (!(bool)(v2 & v25 & v30 != 0)) {
          v21 = 0x5c;
          v17 = 0x5c;
          v22 = 0;
          goto label_9480;
        }
label_98ba:
        v23 += 1;
        v24 = 0;
        v17 = 0x5c;
        v8 = v20;
        goto label_98d0;
      }
      switch(v17) {
        case 0:
          if (!v2) {
            v22 = 0;
            v21 = 0;
            if (v34 & 1) goto label_9552;
            goto label_948b;
          }
label_93de:
          if (!v25) {
            v22 = 0;
label_93ec:
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
                goto label_9163;
              }
label_9cf5:
              a0[v14] = 0x5c;
              v13 = v14;
              v20 = (bool)v8;
            }
            else if (v13 < a1) {
              v8 = v20;
              goto label_9cf5;
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
label_9d2c:
              v17 = (unsigned char)v21;
              v24 = 0;
              v13 = v14;
              v8 = v26;
              if (!v2 || v27) goto label_9163;
              goto label_8fb8;
            }
            v17 = 0x30;
            v24 = 0;
            v13 = v14;
            v8 = v26;
            goto label_9163;
          }
          if (a4 != 2) goto label_90ac;
          goto label_931b;
        default:
          goto label_90c3;
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
          goto label_950f;
        case 10:
          v21 = 10;
          v17 = 0x6e;
          goto label_950f;
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
label_950f:
          v8 = a4 == 2 & v25;
          v22 = 0;
          if (!(bool)v8) goto label_9480;
label_9091:
          a4 = 2;
label_90a3:
          if (!v28) goto label_90ac;
          goto label_931b;
        case 0x20:
          v19 = 0x20;
          goto label_9079;
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
label_9076:
          v26 = 0;
label_9079:
          v21 = v19;
          v24 = v26;
          if ((a4 == 2) && (v25)) goto label_9091;
          goto label_9137;
        case 0x23:
label_9450:
          v21 = 0x23;
label_9455:
          if (!v23) {
            v19 = v21;
            goto label_9079;
          }
label_9922:
          v8 = a4 == 2;
          v26 = v3;
          v24 = 0;
          goto label_9139;
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
          goto label_9130;
        case 0x27:
label_92c2:
          v24 = v26;
          if (a4 != 2) {
            v8 = 0;
            v21 = 0x27;
            goto label_9139;
          }
          if (!v25) {
            if (a1) { // branch-flip
              v14 = 0;
              v21 = a1;
              if (v32) goto label_9d4b;
            }
            else {
label_9d4b:
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
            goto label_9163;
          }
          goto label_90a3;
        case 0x3f:
label_9281:
          if (a4 == 2) {
            if (v25) goto label_90a3;
            v8 = 0;
            v17 = 0x3f;
            v24 = 0;
            goto label_9163;
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
                goto label_9139;
              }
              if (v25) goto label_90ac;
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
              goto label_9d2c;
            }
          }
          v8 = 0;
          v21 = 0x3f;
          v26 = v3;
          v24 = 0;
          goto label_9139;
        
      }
label_9480:
      if (!v2) {
label_948b:
        v17 = (unsigned char)v21;
        v24 = 0;
        v8 = 0;
        v26 = v3;
        if (!v25) goto label_9160;
        goto label_8fb8;
      }
label_92e2:
      v24 = 0;
      goto label_92f3;
    }
    if (!(bool)(v13 == 0 & v25 & a4 == 2)) {
      v8 = a4 == 2 & (v25 ^ 1U);
      v25 = (bool)(v25 ^ 1U);
      if ((!(bool)v8) || (v25 = (bool)v8, !v3)) {
label_9feb:
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
        if (a1 || !v32) goto label_9feb;
        v23 = v32;
        goto label_9a7b;
      }
      v13 = 0;
      a4 = 5;
      v10 = (unsigned long)__ctype_get_mb_cur_max();
      v30 = 1;
      v33 = (unsigned long)v10;
      v31 = "\"";
      if (!(v34 & 2)) goto label_a09b;
      v3 = 0;
      v20 = 0;
      v32 = 0;
      v2 = v4;
      v25 = v4;
      goto label_8e7e;
    }
label_977d:
    if (v2) {
label_931b:
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
label_9341:
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
label_9a7b:
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
label_9f25:
    if (!a2[(long)v10]) break;
  }
label_9f2f:
  v24 = 0;
label_9bab:
  v19 = (unsigned long)v17;
  v26 = v24;
  if (2 <= v21) {
label_9bb5:
    v26 = 0;
    v21 += v23;
    v14 = v23;
    do {
      v17 = (unsigned char)v19;
      if (v27) {
        v28 = a4 == 2;
        if (v25) goto label_90a3;
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
        if (v21 <= v23) goto label_9041;
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
          goto label_98d0;
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
label_911e:
  if (v27) {
    v24 = 0;
    v27 = v2;
    goto label_9bb5;
  }
label_9130:
  v21 = v19;
  v24 = v26;
label_9137:
  v8 = a4 == 2;
  v26 = v3;
label_9139:
  v17 = (unsigned char)v21;
  v27 = (bool)v8 == 0;
  v3 = v26;
  v8 = 0;
  if ((v27 && v2) || (v8 = 0, v25)) {
label_8fb8:
    v17 = (unsigned char)v21;
    v13 = v14;
    if (!v29) goto label_9163;
    if (!(*(unsigned int *)(v29 + (v21 >> 5) * 4) >> (v17 & 0x1f) & 1)) goto label_9163;
  }
  else {
label_9160:
    v8 = 0;
    v3 = v26;
label_9163:
    if (!v22) {
      v8 ^= 1;
      v23 += 1;
      v8 &= v20;
label_98d0:
      if ((bool)v8) {
        if (v13 < a1)
          a0[v13] = 0x27;
        if (v13 + 1 < a1)
          a0[v13 + 1] = 0x27;
        v20 = 0;
        v13 += 2;
      }
      goto label_9041;
    }
  }
label_92f3:
  if (v25) {
label_930f:
    if ((bool)(v2 & a4 == 2)) goto label_931b;
label_90ac:
    v34 &= 0xfffffffd;
    v29 = 0;
    goto label_8d80;
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
label_902e:
  if (v13 < a1)
    a0[v13] = 0x5c;
  v13 += 1;
  v23 += 1;
label_9041:
  if (v13 < a1)
    a0[v13] = v17;
  v13 += 1;
  if (!v24)
    v4 = 0;
  goto label_8e88;
label_9552:
  v23 += 1;
  goto label_8e88;
label_a09b:
  a1 = v32;
label_9181:
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
  goto label_8e6b;
}

// Function: sub_a1a0 @ 0xa1a0
char * sub_a1a0(unsigned int a0,char *a1,unsigned long a2,struct_2 *a3)
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
    v5 = dat_11080;
    if (dat_11078 <= (int)a0) {
      v10 = (long)dat_11078;
      v8 = (long)(int)((a0 - dat_11078) + 1);
      if (dat_11080 != (unsigned long *)0x11090) // branch-flip
        v5 = (unsigned long *)sub_75b0(dat_11080,&v10,v8,0x7fffffff,0x10);
      else {
        v5 = (unsigned long *)sub_75b0(NULL,&v10,v8,0x7fffffff,0x10);
        *v5 = dat_11090;
        v5[1] = dat_11098;
      }
      dat_11080 = v5;
      memset(&v5[(long)dat_11078 * 2],0,(v10 - dat_11078) * 0x10);
      dat_11078 = (int)v10;
    }
    v1 = &v5[(long)(int)a0 * 2];
    v4 = *v1;
    v7 = (char *)v1[1];
    v9 = a3->field_0x4 | 1;
    v6 = sub_8d10(v7,v4,a1,a2,a3->field_0x0,v9,a3->field_0x8,a3->field_0x28,a3->field_0x30);
    if (v4 <= v6) {
      v6 += 1;
      *v1 = v6;
      if (v7 != (char *)0x21180)
        free(v7);
      v7 = (char *)sub_7350(v6);
      v3 = a3->field_0x0;
      v1[1] = (unsigned long)v7;
      sub_8d10(v7,v6,a1,a2,v3,v9,a3->field_0x8,a3->field_0x28,a3->field_0x30);
    }
    *v11 = v2;
    return v7;
  }
  abort(); // no-return
}

// Function: sub_a390 @ 0xa390
void sub_a390(void *a0)
{
  int v1;
  int *v2; // rax
  
  v2 = __errno_location();
  v1 = *v2;
  if (!a0)
    a0 = (void *)0x21280;
  sub_7710(a0,0x38);
  *v2 = v1;
}

// Function: sub_a3d0 @ 0xa3d0
unsigned int sub_a3d0(unsigned int *a0)
{
  if (!a0)
    a0 = (unsigned int *)0x21280;
  return *a0;
}

// Function: sub_a3f0 @ 0xa3f0
void sub_a3f0(unsigned int *a0,unsigned int a1)
{
  if (!a0)
    a0 = (unsigned int *)0x21280;
  *a0 = a1;
}

// Function: sub_a410 @ 0xa410
unsigned int sub_a410(long a0,unsigned char a1,unsigned int a2)
{
  unsigned int *v1;
  unsigned int v2;
  unsigned int v3; // eax
  
  if (!a0)
    a0 = 0x21280;
  v1 = (unsigned int *)(a0 + 8 + (unsigned long)(a1 >> 5) * 4);
  v2 = *v1;
  v3 = v2 >> (a1 & 0x1f);
  *v1 = ((a2 ^ v3) & 1) << (a1 & 0x1f) ^ v2;
  return v3 & 1;
}

// Function: sub_a450 @ 0xa450
unsigned int sub_a450(long a0,unsigned int a1)
{
  unsigned int v1;
  
  if (!a0)
    a0 = 0x21280;
  v1 = *(unsigned int *)(a0 + 4);
  *(unsigned int *)(a0 + 4) = a1;
  return v1;
}

// Function: sub_a470 @ 0xa470
void sub_a470(unsigned int *a0,long a1,long a2)
{
  if (!a0)
    a0 = (unsigned int *)0x21280;
  *a0 = 10;
  if ((a1) && (a2)) {
    *(long *)&a0[10] = a1;
    *(long *)&a0[0xc] = a2;
    return;
  }
  abort(); // no-return
}

// Function: sub_a4b0 @ 0xa4b0
void sub_a4b0(char *a0,unsigned long a1,char *a2,unsigned long a3,unsigned int *a4)
{
  int v1;
  int *v2; // rax
  
  if (!a4)
    a4 = (unsigned int *)0x21280;
  v2 = __errno_location();
  v1 = *v2;
  sub_8d10(a0,a1,a2,a3,*a4,a4[1],&a4[2],*(char **)&a4[10],*(char **)&a4[0xc]);
  *v2 = v1;
}

// Function: sub_a530 @ 0xa530
char * sub_a530(char *a0,unsigned long a1,long *a2,unsigned int *a3)
{
  int v1;
  int *v2; // rax
  long v3; // rax
  char *v4; // rax
  unsigned int v5; // r9d
  
  if (!a3)
    a3 = (unsigned int *)0x21280;
  v2 = __errno_location();
  v1 = *v2;
  v5 = (unsigned int)(a2 == NULL) | a3[1];
  v3 = sub_8d10(NULL,0,a0,a1,*a3,v5,&a3[2],*(char **)&a3[10],*(char **)&a3[0xc]);
  v4 = (char *)sub_7350(v3 + 1U);
  sub_8d10(v4,v3 + 1U,a0,a1,*a3,v5,&a3[2],*(char **)&a3[10],*(char **)&a3[0xc]);
  *v2 = v1;
  if (a2)
    *a2 = v3;
  return v4;
}

// Function: sub_a620 @ 0xa620
char * sub_a620(char *a0,unsigned long a1,unsigned int *a2)
{
  return sub_a530(a0,a1,NULL,a2); // tail-call
}

// Function: sub_a630 @ 0xa630
void sub_a630(void) // return-dupe
{
  unsigned long *v1;
  void *v2;
  void *v3;
  unsigned long *v4;
  
  v3 = dat_11080;
  if (2 <= dat_11078) {
    v4 = (unsigned long *)((long)dat_11080 + 0x18);
    v1 = (unsigned long *)((long)dat_11080 + (unsigned long)(unsigned int)(dat_11078 - 2) * 0x10 + 0x28);
    do {
      v2 = (void *)*v4;
      v4 = &v4[2];
      free(v2);
    } while (v4 != v1);
  }
  if (*(void **)((long)v3 + 8) != (void *)0x21180) {
    free(*(void **)((long)v3 + 8));
    dat_11098 = 0x21180;
    dat_11090 = 0x100;
  }
  if (v3 == (void *)0x11090) {
    dat_11078 = 1;
    return;
  }
  free(v3);
  dat_11078 = 1;
  dat_11080 = (void *)0x11090;
}

// Function: sub_a6d0 @ 0xa6d0
char * sub_a6d0(unsigned int a0,char *a1)
{
  return sub_a1a0(a0,a1,0xffffffffffffffff,(struct_2 *)0x21280); // tail-call
}

// Function: sub_a6f0 @ 0xa6f0
char * sub_a6f0(unsigned int a0,char *a1,unsigned long a2)
{
  return sub_a1a0(a0,a1,a2); // tail-call
}

// Function: sub_a700 @ 0xa700
char * sub_a700(char *a0)
{
  return sub_a1a0(0,a0,0xffffffffffffffff,(struct_2 *)0x21280); // tail-call
}

// Function: sub_a720 @ 0xa720
char * sub_a720(char *a0,unsigned long a1)
{
  return sub_a1a0(0,a0,a1,(struct_2 *)0x21280); // tail-call
}

// Function: sub_a740 @ 0xa740
void sub_a740(unsigned int a0,int a1,char *a2)
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
    sub_a1a0(a0,a2,0xffffffffffffffff,v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_a7e0 @ 0xa7e0
void sub_a7e0(unsigned int a0,int a1,char *a2,unsigned long a3)
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
    sub_a1a0(a0,a2,a3,v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_a870 @ 0xa870
void sub_a870(int a0,char *a1)
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
    sub_a1a0(0,a1,0xffffffffffffffff,v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_a900 @ 0xa900
void sub_a900(int a0,char *a1,unsigned long a2)
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
    sub_a1a0(0,a1,a2,v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_a990 @ 0xa990
void sub_a990(char *a0,unsigned long a1,unsigned char a2)
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
  
  v3 = dat_21280;
  v4 = dat_21288;
  v9 = dat_212b0;
  v5 = dat_21290;
  v6 = dat_21298;
  v7 = dat_212a0;
  v8 = dat_212a8;
  v1 = (unsigned int *)((long)&v4 + (unsigned long)(a2 >> 5) * 4);
  v2 = *v1;
  *v1 = (~(v2 >> (a2 & 0x1f)) & 1) << (a2 & 0x1f) ^ v2;
  sub_a1a0(0,a0,a1,&v3);
}

// Function: sub_aa30 @ 0xaa30
void sub_aa30(char *a0,unsigned char a1)
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
  
  v3 = dat_21280;
  v4 = dat_21288;
  v9 = dat_212b0;
  v5 = dat_21290;
  v6 = dat_21298;
  v7 = dat_212a0;
  v8 = dat_212a8;
  v1 = (unsigned int *)((long)&v4 + (unsigned long)(a1 >> 5) * 4);
  v2 = *v1;
  *v1 = (~(v2 >> (a1 & 0x1f)) & 1) << (a1 & 0x1f) ^ v2;
  sub_a1a0(0,a0,0xffffffffffffffff,&v3);
}

// Function: sub_aad0 @ 0xaad0
void sub_aad0(char *a0)
{
  unsigned long v1; // stack - 0x48
  unsigned long v2; // stack - 0x40
  unsigned int v3; // stack - 0x3c
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  unsigned long v7; // stack - 0x20
  unsigned long v8; // stack - 0x18
  
  v1 = dat_21280;
  v3 = (unsigned int)((unsigned long)dat_21288 >> 0x20);
  v8 = dat_212b0;
  v4 = dat_21290;
  v5 = dat_21298;
  v6 = dat_212a0;
  v7 = dat_212a8;
  v2 = (unsigned int)dat_21288;
  v2 = CONCAT44(~v3 & 0x4000000 ^ v3,v2);
  sub_a1a0(0,a0,0xffffffffffffffff,&v1);
}

// Function: sub_ab60 @ 0xab60
void sub_ab60(char *a0,unsigned long a1)
{
  unsigned long v1; // stack - 0x48
  unsigned long v2; // stack - 0x40
  unsigned int v3; // stack - 0x3c
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  unsigned long v7; // stack - 0x20
  unsigned long v8; // stack - 0x18
  
  v1 = dat_21280;
  v3 = (unsigned int)((unsigned long)dat_21288 >> 0x20);
  v8 = dat_212b0;
  v4 = dat_21290;
  v5 = dat_21298;
  v6 = dat_212a0;
  v7 = dat_212a8;
  v2 = (unsigned int)dat_21288;
  v2 = CONCAT44(~v3 & 0x4000000 ^ v3,v2);
  sub_a1a0(0,a0,a1,&v1);
}

// Function: sub_abf0 @ 0xabf0
void sub_abf0(unsigned int a0,int a1,char *a2)
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
    sub_a1a0(a0,a2,0xffffffffffffffff,v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_ac90 @ 0xac90
void sub_ac90(unsigned int a0,long a1,long a2,char *a3)
{
  unsigned long v1; // stack - 0x48
  unsigned int v2; // stack - 0x44
  unsigned long v3; // stack - 0x40
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  long v7; // stack - 0x20
  long v8; // stack - 0x18
  
  v3 = dat_21288;
  v4 = dat_21290;
  v5 = dat_21298;
  v8 = dat_212b0;
  v2 = (unsigned int)((unsigned long)dat_21280 >> 0x20);
  v1 = CONCAT44(v2,10);
  v6 = dat_212a0;
  v7 = dat_212a8;
  if ((a1) && (a2)) {
    v7 = a1;
    v8 = a2;
    sub_a1a0(a0,a3,0xffffffffffffffff,&v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_ad30 @ 0xad30
void sub_ad30(unsigned int a0,long a1,long a2,char *a3,unsigned long a4)
{
  unsigned long v1; // stack - 0x48
  unsigned int v2; // stack - 0x44
  unsigned long v3; // stack - 0x40
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  long v7; // stack - 0x20
  long v8; // stack - 0x18
  
  v3 = dat_21288;
  v4 = dat_21290;
  v5 = dat_21298;
  v8 = dat_212b0;
  v2 = (unsigned int)((unsigned long)dat_21280 >> 0x20);
  v1 = CONCAT44(v2,10);
  v6 = dat_212a0;
  v7 = dat_212a8;
  if ((a1) && (a2)) {
    v7 = a1;
    v8 = a2;
    sub_a1a0(a0,a3,a4,&v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_add0 @ 0xadd0
void sub_add0(long a0,long a1,char *a2)
{
  unsigned long v1; // stack - 0x48
  unsigned int v2; // stack - 0x44
  unsigned long v3; // stack - 0x40
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  long v7; // stack - 0x20
  long v8; // stack - 0x18
  
  v3 = dat_21288;
  v4 = dat_21290;
  v5 = dat_21298;
  v8 = dat_212b0;
  v2 = (unsigned int)((unsigned long)dat_21280 >> 0x20);
  v1 = CONCAT44(v2,10);
  v6 = dat_212a0;
  v7 = dat_212a8;
  if ((a0) && (a1)) {
    v7 = a0;
    v8 = a1;
    sub_a1a0(0,a2,0xffffffffffffffff,&v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_ae70 @ 0xae70
void sub_ae70(long a0,long a1,char *a2,unsigned long a3)
{
  unsigned long v1; // stack - 0x48
  unsigned int v2; // stack - 0x44
  unsigned long v3; // stack - 0x40
  unsigned long v4; // stack - 0x38
  unsigned long v5; // stack - 0x30
  unsigned long v6; // stack - 0x28
  long v7; // stack - 0x20
  long v8; // stack - 0x18
  
  v3 = dat_21288;
  v4 = dat_21290;
  v5 = dat_21298;
  v8 = dat_212b0;
  v2 = (unsigned int)((unsigned long)dat_21280 >> 0x20);
  v1 = CONCAT44(v2,10);
  v6 = dat_212a0;
  v7 = dat_212a8;
  if ((a0) && (a1)) {
    v7 = a0;
    v8 = a1;
    sub_a1a0(0,a2,a3,&v1);
    return;
  }
  abort(); // no-return
}

// Function: sub_af10 @ 0xaf10
char * sub_af10(unsigned int a0,char *a1,unsigned long a2)
{
  return sub_a1a0(a0,a1,a2); // tail-call
}

// Function: sub_af20 @ 0xaf20
char * sub_af20(char *a0,unsigned long a1)
{
  return sub_a1a0(0,a0,a1,(struct_2 *)0x11040); // tail-call
}

// Function: sub_af40 @ 0xaf40
char * sub_af40(unsigned int a0,char *a1)
{
  return sub_a1a0(a0,a1,0xffffffffffffffff,(struct_2 *)0x11040); // tail-call
}

// Function: sub_af60 @ 0xaf60
char * sub_af60(char *a0)
{
  return sub_a1a0(0,a0,0xffffffffffffffff,(struct_2 *)0x11040); // tail-call
}

// Function: sub_af80 @ 0xaf80
unsigned long sub_af80(int a0,char *a1,unsigned long a2) // return-dupe x2
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

// Function: sub_b030 @ 0xb030
void sub_b030(int a0)
{
  setlocale(a0,NULL); // tail-call
}

// Function: sub_b040 @ 0xb040
int sub_b040(void) // early-return x2
{
  int v1; // eax
  int v2; // ebx
  int v3; // ebp
  
  v3 = 1;
  v2 = 0;
  do {
    v1 = sub_b680(v2,1);
    if (0 <= v1) { // branch-flip
label_b073:
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
        goto label_b073;
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

// Function: sub_b0f0 @ 0xb0f0
unsigned long sub_b0f0(unsigned long a0,long a1,unsigned long a2,long *a3,unsigned long *a4) // return-dupe
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
label_b292:
      if (iconv(a2,0,0,&v5,&v4) != -1) {
        if (v4)
          abort(); // no-return
        *a3 = (long)v3;
        *a4 = (unsigned long)v2;
        return 0;
      }
label_b33e:
      if ((char *)*a3 != v3) {
        free(v3);
        return 0xffffffff;
      }
      return 0xffffffff;
    }
    if (iconv(a2,&v7,&v8,&v5,&v4) == -1) {
      if (*__errno_location() != 0x16) goto label_b33e;
      goto label_b292;
    }
    a1 = v8;
  } while( true );
}

// Function: sub_b370 @ 0xb370
char * sub_b370(char *a0,unsigned long a1) // return-dupe
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
    if (*v4 == 0x16) goto label_b50d;
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
label_b50d:
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

// Function: sub_b5c0 @ 0xb5c0
char * sub_b5c0(char *a0,long a1,long a2)
{
  int v1;
  char *v2; // rax
  long v3; // rax
  int *v4; // rax
  
  if ((!*a0) || (!sub_7bc0(a1,a2))) {
    v2 = strdup(a0);
    if (!v2)
      *__errno_location() = 0xc;
    return v2;
  }
  v3 = iconv_open(a2,a1);
  if (v3 == -1)
    return NULL;
  v2 = (char *)sub_b370(a0,v3);
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

// Function: sub_b680 @ 0xb680
int sub_b680(int a0,int a1,unsigned long a2)
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
    if (0 <= dat_212b8) { // branch-flip
      v2 = fcntl(a0,0x406,a2 & 0xffffffff);
      if ((0 <= v2) || (*__errno_location() != 0x16)) {
        dat_212b8 = 1;
        return v2;
      }
      v2 = fcntl(a0,0,a2 & 0xffffffff);
      if (v2 < 0)
        return v2;
      dat_212b8 = -1;
    }
    else {
      v2 = fcntl(a0,0,a2 & 0xffffffff);
      if (v2 < 0)
        return v2;
      if (dat_212b8 != -1)
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

// Function: sub_b8f0 @ 0xb8f0
long sub_b8f0(unsigned long a0)
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

// Function: sub_b940 @ 0xb940
void sub_b940(unsigned long a0)
{
  if (a0 & 0xf)
    abort(); // no-return
  if (!(a0 & 0x10))
    return;
  free((void *)(a0 - *(unsigned char *)(a0 - 1))); // tail-call
}

// Function: sub_b970 @ 0xb970
unsigned long sub_b970(unsigned char *a0)
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
label_ba4c:
      v3 = __ctype_get_mb_cur_max();
      v2 = v8;
      v9 = sub_7b30(v5,v2,sub_6600(v8,v3),&v4);
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
          goto label_ba28;
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
      if (!(*(unsigned int *)((unsigned long)(*v8 >> 5) * 4 + 0xd300) >> (*v8 & 0x1f) & 1)) {
        if (!mbsinit(&v4))
          __assert_fail("mbsinit (&iter->state)","mbuiter.h",0x8f,"mbuiter_multi_next"); // no-return
        v1 = 1;
        goto label_ba4c;
      }
      v9 = 1;
      v5[0] = (int)(char)*v8;
      v10 = 1;
label_ba28:
      if (!v5[0])
        return v6;
    }
    v8 = &v8[v9];
    v6 += 1;
  } while( true );
}

// Function: _DT_FINI @ 0xbbbc
void _DT_FINI(void)
{
  return;
}
