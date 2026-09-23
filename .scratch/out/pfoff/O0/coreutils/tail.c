// Function: _DT_INIT @ 0x2000
void _DT_INIT(void) // return-dupe
{
  if (!dat_17fe0)
    return;
  (*dat_17fe0)();
}

// Function: sub_2020 @ 0x2020
void sub_2020(void)
{
  (*dat_17d78)(); // jump-as-call
}

// Function: __cxa_finalize @ 0x24d0
void __cxa_finalize(void)
{
  (*dat_17ff0)(); // jump-as-call
}

// Function: getenv @ 0x24e0
char * getenv(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_17d80)(); // jump-as-call
  return v1;
}

// Function: raise @ 0x24f0
int raise(int a0)
{
  int v1; // eax
  
  v1 = (*dat_17d88)(); // jump-as-call
  return v1;
}

// Function: free @ 0x2500
void free(void *a0)
{
  (*dat_17d90)(); // jump-as-call
}

// Function: abort @ 0x2510
void abort(void)
{
  (*dat_17d98)(); // jump-as-call
}

// Function: __errno_location @ 0x2520
int * __errno_location(void)
{
  int *v1; // rax
  
  v1 = (int *)(*dat_17da0)(); // jump-as-call
  return v1;
}

// Function: strncmp @ 0x2530
int strncmp(char *a0,char *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_17da8)(); // jump-as-call
  return v1;
}

// Function: _exit @ 0x2540
void _exit(int a0)
{
  (*dat_17db0)(); // jump-as-call
}

// Function: __fpending @ 0x2550
unsigned long __fpending(FILE *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_17db8)(); // jump-as-call
  return v1;
}

// Function: isatty @ 0x2560
int isatty(int a0)
{
  int v1; // eax
  
  v1 = (*dat_17dc0)(); // jump-as-call
  return v1;
}

// Function: reallocarray @ 0x2570
void * reallocarray(void *a0,unsigned long a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_17dc8)(); // jump-as-call
  return v1;
}

// Function: strtod @ 0x2580
void strtod(void)
{
  (*dat_17dd0)(); // jump-as-call
}

// Function: fcntl @ 0x2590
int fcntl(int a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_17dd8)(); // jump-as-call
  return v1;
}

// Function: clearerr_unlocked @ 0x25a0
void clearerr_unlocked(FILE *a0)
{
  (*dat_17de0)(); // jump-as-call
}

// Function: textdomain @ 0x25b0
char * textdomain(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_17de8)(); // jump-as-call
  return v1;
}

// Function: strtod_l @ 0x25c0
void strtod_l(void)
{
  (*dat_17df0)(); // jump-as-call
}

// Function: fclose @ 0x25d0
int fclose(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_17df8)(); // jump-as-call
  return v1;
}

// Function: bindtextdomain @ 0x25e0
char * bindtextdomain(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_17e00)(); // jump-as-call
  return v1;
}

// Function: __ctype_get_mb_cur_max @ 0x25f0
unsigned long __ctype_get_mb_cur_max(void)
{
  unsigned long v1; // rax
  
  v1 = (*dat_17e08)(); // jump-as-call
  return v1;
}

// Function: strlen @ 0x2600
unsigned long strlen(char *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_17e10)(); // jump-as-call
  return v1;
}

// Function: __stack_chk_fail @ 0x2610
void __stack_chk_fail(void)
{
  (*dat_17e18)(); // jump-as-call
}

// Function: getopt_long @ 0x2620
int getopt_long(int a0,char **a1,char *a2,option *a3,int *a4)
{
  int v1; // eax
  
  v1 = (*dat_17e20)(); // jump-as-call
  return v1;
}

// Function: mbrtowc @ 0x2630
unsigned long mbrtowc(void *a0,char *a1,unsigned long a2,mbstate_t *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_17e28)(); // jump-as-call
  return v1;
}

// Function: gettext @ 0x2640
char * gettext(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_17e30)(); // jump-as-call
  return v1;
}

// Function: strchr @ 0x2650
char * strchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_17e38)(); // jump-as-call
  return v1;
}

// Function: printf @ 0x2660
int printf(char *a0,...)
{
  int v1; // eax
  
  v1 = (*dat_17e40)(); // jump-as-call
  return v1;
}

// Function: newlocale @ 0x2670
void newlocale(void)
{
  (*dat_17e48)(); // jump-as-call
}

// Function: nanosleep @ 0x2680
int nanosleep(timespec *a0,timespec *a1)
{
  int v1; // eax
  
  v1 = (*dat_17e50)(); // jump-as-call
  return v1;
}

// Function: strrchr @ 0x2690
char * strrchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_17e58)(); // jump-as-call
  return v1;
}

// Function: lseek @ 0x26a0
void lseek(void)
{
  (*dat_17e60)(); // jump-as-call
}

// Function: __assert_fail @ 0x26b0
void __assert_fail(char *a0,char *a1,unsigned int a2,char *a3)
{
  (*dat_17e68)(); // jump-as-call
}

// Function: fputs @ 0x26c0
int fputs(char *a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_17e70)(); // jump-as-call
  return v1;
}

// Function: memset @ 0x26d0
void * memset(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_17e78)(); // jump-as-call
  return v1;
}

// Function: close @ 0x26e0
int close(int a0)
{
  int v1; // eax
  
  v1 = (*dat_17e80)(); // jump-as-call
  return v1;
}

// Function: memchr @ 0x26f0
void * memchr(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_17e88)(); // jump-as-call
  return v1;
}

// Function: read @ 0x2700
long read(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_17e90)(); // jump-as-call
  return v1;
}

// Function: lstat @ 0x2710
int lstat(char *a0,stat *a1)
{
  int v1; // eax
  
  v1 = (*dat_17e98)(); // jump-as-call
  return v1;
}

// Function: memcmp @ 0x2720
int memcmp(void *a0,void *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_17ea0)(); // jump-as-call
  return v1;
}

// Function: fputs_unlocked @ 0x2730
int fputs_unlocked(char *a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_17ea8)(); // jump-as-call
  return v1;
}

// Function: rawmemchr @ 0x2740
void * rawmemchr(void *a0,int a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_17eb0)(); // jump-as-call
  return v1;
}

// Function: ferror_unlocked @ 0x2750
int ferror_unlocked(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_17eb8)(); // jump-as-call
  return v1;
}

// Function: calloc @ 0x2760
void * calloc(unsigned long a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_17ec0)(); // jump-as-call
  return v1;
}

// Function: strcmp @ 0x2770
int strcmp(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_17ec8)(); // jump-as-call
  return v1;
}

// Function: fprintf @ 0x2780
int fprintf(FILE *a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_17ed0)(); // jump-as-call
  return v1;
}

// Function: stat @ 0x2790
int stat(char *a0,stat *a1)
{
  int v1; // eax
  
  v1 = (*dat_17ed8)(); // jump-as-call
  return v1;
}

// Function: strtol @ 0x27a0
long strtol(char *a0,char **a1,int a2)
{
  long v1; // rax
  
  v1 = (*dat_17ee0)(); // jump-as-call
  return v1;
}

// Function: memcpy @ 0x27b0
void * memcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_17ee8)(); // jump-as-call
  return v1;
}

// Function: kill @ 0x27c0
int kill(int a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_17ef0)(); // jump-as-call
  return v1;
}

// Function: inotify_init @ 0x27d0
void inotify_init(void)
{
  (*dat_17ef8)(); // jump-as-call
}

// Function: fileno @ 0x27e0
int fileno(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_17f00)(); // jump-as-call
  return v1;
}

// Function: putc_unlocked @ 0x27f0
int putc_unlocked(int a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_17f08)(); // jump-as-call
  return v1;
}

// Function: pause @ 0x2800
int pause(void)
{
  int v1; // eax
  
  v1 = (*dat_17f10)(); // jump-as-call
  return v1;
}

// Function: malloc @ 0x2810
void * malloc(unsigned long a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_17f18)(); // jump-as-call
  return v1;
}

// Function: fflush @ 0x2820
int fflush(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_17f20)(); // jump-as-call
  return v1;
}

// Function: inotify_add_watch @ 0x2830
void inotify_add_watch(void)
{
  (*dat_17f28)(); // jump-as-call
}

// Function: nl_langinfo @ 0x2840
char * nl_langinfo(int a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_17f30)(); // jump-as-call
  return v1;
}

// Function: __freading @ 0x2850
int __freading(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_17f38)(); // jump-as-call
  return v1;
}

// Function: fwrite_unlocked @ 0x2860
unsigned long fwrite_unlocked(void *a0,unsigned long a1,unsigned long a2,FILE *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_17f40)(); // jump-as-call
  return v1;
}

// Function: realloc @ 0x2870
void * realloc(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_17f48)(); // jump-as-call
  return v1;
}

// Function: setlocale @ 0x2880
char * setlocale(int a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_17f50)(); // jump-as-call
  return v1;
}

// Function: poll @ 0x2890
void poll(void)
{
  (*dat_17f58)(); // jump-as-call
}

// Function: error @ 0x28a0
void error(int a0,int a1,char *a2,...)
{
  (*dat_17f60)(); // jump-as-call
}

// Function: memrchr @ 0x28b0
void * memrchr(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_17f68)(); // jump-as-call
  return v1;
}

// Function: open @ 0x28c0
int open(char *a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_17f70)(); // jump-as-call
  return v1;
}

// Function: fseeko @ 0x28d0
int fseeko(FILE *a0,long a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_17f78)(); // jump-as-call
  return v1;
}

// Function: strtoumax @ 0x28e0
void strtoumax(void)
{
  (*dat_17f80)(); // jump-as-call
}

// Function: fstatfs @ 0x28f0
int fstatfs(int a0,statfs *a1)
{
  int v1; // eax
  
  v1 = (*dat_17f88)(); // jump-as-call
  return v1;
}

// Function: __cxa_atexit @ 0x2900
void __cxa_atexit(void)
{
  (*dat_17f90)(); // jump-as-call
}

// Function: exit @ 0x2910
void exit(int a0)
{
  (*dat_17f98)(); // jump-as-call
}

// Function: fflush_unlocked @ 0x2920
int fflush_unlocked(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_17fa0)(); // jump-as-call
  return v1;
}

// Function: mbsinit @ 0x2930
int mbsinit(mbstate_t *a0)
{
  int v1; // eax
  
  v1 = (*dat_17fa8)(); // jump-as-call
  return v1;
}

// Function: iswprint @ 0x2940
int iswprint(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_17fb0)(); // jump-as-call
  return v1;
}

// Function: inotify_rm_watch @ 0x2950
void inotify_rm_watch(void)
{
  (*dat_17fb8)(); // jump-as-call
}

// Function: fstat @ 0x2960
int fstat(int a0,stat *a1)
{
  int v1; // eax
  
  v1 = (*dat_17fc0)(); // jump-as-call
  return v1;
}

// Function: __ctype_b_loc @ 0x2970
void * __ctype_b_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_17fc8)(); // jump-as-call
  return v1;
}

// Function: sub_2980 @ 0x2980
void sub_2980(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_17fd0)(main,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: sub_29b0 @ 0x29b0
void sub_29b0(void)
{
  return;
}

// Function: _FINI_0 @ 0x2a20
void _FINI_0(void)
{
  if (!dat_18128) {
    if (dat_17ff0)
      __cxa_finalize(dat_18008);
    sub_29b0();
    dat_18128 = 1;
    return;
  }
}

// Function: _INIT_0 @ 0x2a60
void _INIT_0(void)
{
  return;
}

// Function: sub_2a69 @ 0x2a69
void sub_2a69(void)
{
  FILE *v1;
  
  v1 = stdout;
  fputs_unlocked(gettext("\nWith no FILE, or when FILE is -, read standard input.\n"),v1);
}

// Function: sub_2a9a @ 0x2a9a
void sub_2a9a(void)
{
  FILE *v1;
  
  v1 = stdout;
  fputs_unlocked(gettext("\nMandatory arguments to long options are mandatory for short options too.\n"),v1);
}

// Function: sub_2acb @ 0x2acb
void sub_2acb(char *a0) // ternary
{
  FILE *v1;
  char *v10; // stack - 0x78
  char *v11; // stack - 0x70
  char *v12; // stack - 0x68
  char *v13; // stack - 0x60
  char *v14; // stack - 0x58
  char *v15; // stack - 0x50
  char *v16; // stack - 0x48
  char *v17; // stack - 0x40
  unsigned long v18; // stack - 0x38
  unsigned long v19; // stack - 0x30
  int v2; // eax
  char *v3;
  char *v4; // stack - 0x98
  char *v5; // stack - 0xb8
  char **v6; // stack - 0xb0
  char *v7; // stack - 0x90
  char *v8; // stack - 0x88
  char *v9; // stack - 0x80
  
  v4 = "[";
  v7 = "test invocation";
  v8 = "coreutils";
  v9 = "Multi-call invocation";
  v10 = "sha224sum";
  v11 = "sha2 utilities";
  v12 = "sha256sum";
  v13 = "sha2 utilities";
  v14 = "sha384sum";
  v15 = "sha2 utilities";
  v16 = "sha512sum";
  v17 = "sha2 utilities";
  v18 = 0;
  v19 = 0;
  v6 = &v4;
  while ((*v6 && (v3 = *v6, strcmp(a0,v3)))) {
    v6 = &v6[2];
  }
  v5 = a0;
  if (v6[1])
    v5 = v6[1];
  printf(gettext("\n%s online help: <%s>\n"),"GNU coreutils","https://www.gnu.org/software/coreutils/");
  v3 = setlocale(5,NULL);
  if ((v3) && (v2 = strncmp(v3,"en_",3), v1 = stdout, v2))
    fputs_unlocked(gettext("Report any translation bugs to <https://translationproject.org/team/>\n"),v1);
  if (strcmp(a0,"[")) // branch-flip
    v3 = a0;
  else {
    v3 = "test";
  }
  printf(gettext("Full documentation <%s%s>\n"),"https://www.gnu.org/software/coreutils/",v3);
  v3 = (v5 != a0) ? "" : " invocation"; // branch-flip
  printf(gettext("or available locally via: info \'(coreutils) %s%s\'\n"),v5,v3);
}

// Function: sub_2d62 @ 0x2d62
unsigned int sub_2d62(long a0) // return-dupe
{
  if (((*(unsigned int *)(a0 + 0x18) & 0xf000) != 0x8000) && ((*(unsigned int *)(a0 + 0x18) & 0xf000) != 0xa000))
    return 0;
  return 1;
}

// Function: sub_2da1 @ 0x2da1
unsigned long sub_2da1(unsigned long a0) // switch-return x132
{
  if (a0 == 0xff534d42)
    return 0;
  if (a0 <= 0xff534d42) {
    if (a0 == 0xfe534d42)
      return 0;
    if (a0 <= 0xfe534d42) {
      if (a0 == 0xf995e849)
        return 1;
      if (a0 <= 0xf995e849) {
        if (a0 == 0xf97cff8c)
          return 1;
        if (a0 <= 0xf97cff8c) {
          if (a0 == 0xf2f52010)
            return 1;
          if (a0 <= 0xf2f52010) {
            if (a0 == 0xe0f5e1e2)
              return 1;
            if (a0 <= 0xe0f5e1e2) {
              if (a0 == 0xde5e81e4)
                return 1;
              if (a0 <= 0xde5e81e4) {
                if (a0 == 0xcafe4a11)
                  return 1;
                if (a0 <= 0xcafe4a11) {
                  if (a0 == 0xc97e8168)
                    return 1;
                  if (a0 <= 0xc97e8168) {
                    if (a0 == 0xc7571590)
                      return 1;
                    if (a0 <= 0xc7571590) {
                      if (a0 == 0xbeefdead)
                        return 0;
                      if (a0 <= 0xbeefdead) {
                        if (a0 == 0xbacbacbc)
                          return 0;
                        if (a0 <= 0xbacbacbc) {
                          if (a0 == 0xabba1974)
                            return 1;
                          if (a0 <= 0xabba1974) {
                            if (a0 == 0xaad7aaea)
                              return 0;
                            if (a0 <= 0xaad7aaea) {
                              if (a0 == 0xa501fcf5)
                                return 0;
                              if (a0 <= 0xa501fcf5) {
                                if (a0 == 0x958458f6)
                                  return 1;
                                if (a0 <= 0x958458f6) {
                                  if (a0 == 0x9123683e)
                                    return 1;
                                  if (a0 <= 0x9123683e) {
                                    if (a0 == 0x858458f6)
                                      return 1;
                                    if (a0 <= 0x858458f6) {
                                      if (a0 == 0x7c7c6673)
                                        return 0;
                                      if (a0 <= 0x7c7c6673) {
                                        if (a0 == 0x794c7630)
                                          return 0;
                                        if (a0 <= 0x794c7630) {
                                          if (a0 == 0x786f4256)
                                            return 0;
                                          if (a0 <= 0x786f4256) {
                                            if (a0 == 0x74726163)
                                              return 1;
                                            if (a0 <= 0x74726163) {
                                              if (a0 == 0x7461636f)
                                                return 0;
                                              if (a0 <= 0x7461636f) {
                                                if (a0 == 0x73757245)
                                                  return 0;
                                                if (a0 <= 0x73757245) {
                                                  if (a0 == 0x73727279)
                                                    return 1;
                                                  if (a0 <= 0x73727279) {
                                                    if (a0 == 0x73717368)
                                                      return 1;
                                                    if (a0 <= 0x73717368) {
                                                      if (a0 == 0x73636673)
                                                        return 1;
                                                      if (a0 <= 0x73636673) {
                                                        if (a0 == 0x6e736673)
                                                          return 1;
                                                        if (a0 <= 0x6e736673) {
                                                          if (a0 == 0x6e667364)
                                                            return 0;
                                                          if (a0 <= 0x6e667364) {
                                                            if (a0 == 0x6c6f6f70)
                                                              return 1;
                                                            if (a0 <= 0x6c6f6f70) {
                                                              if (a0 == 0x6b414653)
                                                                return 0;
                                                              if (a0 <= 0x6b414653) {
                                                                if (a0 == 0x68191122)
                                                                  return 1;
                                                                if (a0 <= 0x68191122) {
                                                                  if (a0 == 0x67596969)
                                                                    return 1;
                                                                  if (a0 <= 0x67596969) {
                                                                    if (a0 == 0x65735546)
                                                                      return 0;
                                                                    if (a0 <= 0x65735546) {
                                                                      if (a0 == 0x65735543)
                                                                        return 0;
                                                                      if (a0 <= 0x65735543) {
                                                                        if (a0 == 0x64646178)
                                                                          return 1;
                                                                        if (a0 <= 0x64646178) {
                                                                          if (a0 == 0x64626720)
                                                                            return 1;
                                                                          if (a0 <= 0x64626720) {
                                                                            if (a0 == 0x63677270)
                                                                              return 1;
                                                                            if (a0 <= 0x63677270) {
                                                                              if (a0 == 0x62656572)
                                                                                return 1;
                                                                              if (a0 <= 0x62656572) {
                                                                                if (a0 == 0x62656570)
                                                                                  return 1;
                                                                                if (a0 <= 0x62656570) {
                                                                                  if (a0 == 0x62646576)
                                                                                    return 1;
                                                                                  if (a0 <= 0x62646576) {
                                                                                    if (a0 == 0x61756673)
                                                                                      return 0;
                                                                                    if (a0 <= 0x61756673) {
                                                                                      if (a0 == 0x6165676c)
                                                                                        return 1;
                                                                                      if (a0 <= 0x6165676c) {
                                                                                        if (a0 == 0x61636673)
                                                                                          return 0;
                                                                                        if (a0 <= 0x61636673) {
                                                                                          if (a0 == 0x5dca2df5)
                                                                                            return 1;
                                                                                          if (a0 <= 0x5dca2df5) {
                                                                                            if (a0 == 0x5a4f4653)
                                                                                              return 1;
                                                                                            if (a0 <= 0x5a4f4653) {
                                                                                              if (a0 == 0x5a3c69f0)
                                                                                                return 1;
                                                                                              if (a0 <= 0x5a3c69f0) {
                                                                                                if (a0 == 0x58465342)
                                                                                                  return 1;
                                                                                                if (a0 <= 0x58465342) {
                                                                                                  if (a0 == 0x58295829)
                                                                                                    return 1;
                                                                                                  if (a0 <= 0x58295829) {
                                                                                                    if (a0 == 0x565a4653)
                                                                                                      return 1;
                                                                                                    if (a0 <= 0x565a4653) {
                                                                                                      if (a0 == 0x54190100)
                                                                                                        return 1;
                                                                                                      if (a0 <= 0x54190100) {
                                                                                                        if (a0 == 0x534f434b)
                                                                                                          return 1;
                                                                                                        if (a0 <= 0x534f434b) {
                                                                                                          if (a0 == 0x5346544e)
                                                                                                            return 1;
                                                                                                          if (a0 <= 0x5346544e) {
                                                                                                            if (a0 == 0x53464846)
                                                                                                              return 1;
                                                                                                            if (a0 <= 0x53464846) {
                                                                                                              if (a0 == 0x5346414f)
                                                                                                                return 0;
                                                                                                              if (a0 <= 0x5346414f) {
                                                                                                                if (a0 == 0x5346314d)
                                                                                                                  return 1;
                                                                                                                if (a0 <= 0x5346314d) {
                                                                                                                  if (a0 == 0x5345434d)
                                                                                                                    return 1;
                                                                                                                  if (a0 <= 0x5345434d) {
                                                                                                                    if (a0 == 0x52654973)
                                                                                                                      return 1;
                                                                                                                    if (a0 <= 0x52654973) {
                                                                                                                      if (a0 == 0x50495045)
                                                                                                                        return 0;
                                                                                                                      if (a0 <= 0x50495045) {
                                                                                                                        if (a0 == 0x47504653)
                                                                                                                          return 0;
                                                                                                                        if (a0 <= 0x47504653) {
                                                                                                                          if (a0 == 0x45584653)
                                                                                                                            return 1;
                                                                                                                          if (a0 <= 0x45584653) {
                                                                                                                            if (a0 == 0x454d444d)
                                                                                                                              return 1;
                                                                                                                            if (a0 <= 0x454d444d) {
                                                                                                                              if (a0 == 0x453dcd28)
                                                                                                                                return 1;
                                                                                                                              if (a0 <= 0x453dcd28) {
                                                                                                                                if (a0 == 0x444d4142)
                                                                                                                                  return 1;
                                                                                                                                if (a0 <= 0x444d4142) {
                                                                                                                                  if (a0 == 0x43415d53)
                                                                                                                                    return 1;
                                                                                                                                  if (a0 <= 0x43415d53) {
                                                                                                                                    if (a0 == 0x42494e4d)
                                                                                                                                      return 1;
                                                                                                                                    if (a0 <= 0x42494e4d) {
                                                                                                                                      if (a0 == 0x42465331)
                                                                                                                                        return 1;
                                                                                                                                      if (a0 <= 0x42465331) {
                                                                                                                                        if (a0 == 0x3153464a)
                                                                                                                                          return 1;
                                                                                                                                        if (a0 <= 0x3153464a) {
                                                                                                                                          if (a0 == 0x2fc12fc1)
                                                                                                                                            return 1;
                                                                                                                                          if (a0 <= 0x2fc12fc1) {
                                                                                                                                            if (a0 == 0x2bad1dea)
                                                                                                                                              return 1;
                                                                                                                                            if (a0 <= 0x2bad1dea) {
                                                                                                                                              if (a0 == 0x28cd3d45)
                                                                                                                                                return 1;
                                                                                                                                              if (a0 <= 0x28cd3d45) {
                                                                                                                                                if (a0 == 0x24051905)
                                                                                                                                                  return 1;
                                                                                                                                                if (a0 <= 0x24051905) {
                                                                                                                                                  if (a0 == 0x2011bab0)
                                                                                                                                                    return 1;
                                                                                                                                                  if (a0 <= 0x2011bab0) {
                                                                                                                                                    if (a0 == 0x1badface)
                                                                                                                                                      return 1;
                                                                                                                                                    if (a0 <= 0x1badface) {
                                                                                                                                                      if (a0 == 0x19830326)
                                                                                                                                                        return 0;
                                                                                                                                                      if (a0 <= 0x19830326) {
                                                                                                                                                        if (a0 == 0x19800202)
                                                                                                                                                          return 1;
                                                                                                                                                        if (a0 <= 0x19800202) {
                                                                                                                                                          if (a0 == 0x15013346)
                                                                                                                                                            return 1;
                                                                                                                                                          if (a0 <= 0x15013346) {
                                                                                                                                                            if (a0 == 0x13661366)
                                                                                                                                                              return 1;
                                                                                                                                                            if (a0 <= 0x13661366) {
                                                                                                                                                              if (a0 == 0x11307854)
                                                                                                                                                                return 1;
                                                                                                                                                              if (a0 <= 0x11307854) {
                                                                                                                                                                if (a0 == 0xbd00bd0)
                                                                                                                                                                  return 0;
                                                                                                                                                                if (a0 <= 0xbd00bd0) {
                                                                                                                                                                  if (a0 == 0xbad1dea)
                                                                                                                                                                    return 1;
                                                                                                                                                                  if (a0 <= 0xbad1dea) {
                                                                                                                                                                    if (a0 == 0x9041934)
                                                                                                                                                                      return 1;
                                                                                                                                                                    if (a0 <= 0x9041934) {
                                                                                                                                                                      if (a0 == 0x7655821)
                                                                                                                                                                        return 1;
                                                                                                                                                                      if (a0 <= 0x7655821) {
                                                                                                                                                                        if (a0 == 0x13111a8)
                                                                                                                                                                          return 0;
                                                                                                                                                                        if (a0 <= 0x13111a8) {
                                                                                                                                                                          if (a0 == 0x12ff7b7)
                                                                                                                                                                            return 1;
                                                                                                                                                                          if (a0 <= 0x12ff7b7) {
                                                                                                                                                                            if (a0 == 0x12ff7b6)
                                                                                                                                                                              return 1;
                                                                                                                                                                            if (a0 <= 0x12ff7b6) {
                                                                                                                                                                              if (a0 == 0x12ff7b5)
                                                                                                                                                                                return 1;
                                                                                                                                                                              if (a0 <= 0x12ff7b5) {
                                                                                                                                                                                if (a0 == 0x12ff7b4)
                                                                                                                                                                                  return 1;
                                                                                                                                                                                if (a0 <= 0x12ff7b4) {
                                                                                                                                                                                  if (a0 == 0x12fd16d)
                                                                                                                                                                                    return 1;
                                                                                                                                                                                  if (a0 <= 0x12fd16d) {
                                                                                                                                                                                    if (a0 == 0x1161970)
                                                                                                                                                                                      return 0;
                                                                                                                                                                                    if (a0 <= 0x1161970) {
                                                                                                                                                                                      if (a0 == 0x1021997)
                                                                                                                                                                                        return 1;
                                                                                                                                                                                      if (a0 <= 0x1021997) {
                                                                                                                                                                                        if (a0 == 0x1021994)
                                                                                                                                                                                          return 1;
                                                                                                                                                                                        if (a0 <= 0x1021994) {
                                                                                                                                                                                          if (a0 == 0xc36400)
                                                                                                                                                                                            return 0;
                                                                                                                                                                                          if (a0 <= 0xc36400) {
                                                                                                                                                                                            if (a0 == 0xc0ffee)
                                                                                                                                                                                              return 1;
                                                                                                                                                                                            if (a0 <= 0xc0ffee) {
                                                                                                                                                                                              if (a0 == 0x414a53)
                                                                                                                                                                                                return 1;
                                                                                                                                                                                              if (a0 <= 0x414a53) {
                                                                                                                                                                                                if (a0 == 0x27e0eb)
                                                                                                                                                                                                  return 1;
                                                                                                                                                                                                if (a0 <= 0x27e0eb) {
                                                                                                                                                                                                  if (a0 == 0x11954)
                                                                                                                                                                                                    return 1;
                                                                                                                                                                                                  if (a0 <= 0x11954) {
                                                                                                                                                                                                    if (a0 == 0xf15f)
                                                                                                                                                                                                      return 1;
                                                                                                                                                                                                    if (a0 <= 0xf15f) {
                                                                                                                                                                                                      if (a0 == 0xef53)
                                                                                                                                                                                                        return 1;
                                                                                                                                                                                                      if (a0 <= 0xef53) {
                                                                                                                                                                                                        if (a0 == 0xef51)
                                                                                                                                                                                                          return 1;
                                                                                                                                                                                                        if (a0 <= 0xef51) {
                                                                                                                                                                                                          if (a0 == 0xadff)
                                                                                                                                                                                                            return 1;
                                                                                                                                                                                                          if (a0 <= 0xadff) {
                                                                                                                                                                                                            if (a0 == 0xadf5)
                                                                                                                                                                                                              return 1;
                                                                                                                                                                                                            if (a0 <= 0xadf5) {
                                                                                                                                                                                                              if (a0 == 0x9fa2)
                                                                                                                                                                                                                return 1;
                                                                                                                                                                                                              if (a0 <= 0x9fa2) {
                                                                                                                                                                                                                if (a0 == 0x9fa1)
                                                                                                                                                                                                                  return 1;
                                                                                                                                                                                                                if (a0 <= 0x9fa1) {
                                                                                                                                                                                                                  if (a0 == 0x9fa0)
                                                                                                                                                                                                                    return 1;
                                                                                                                                                                                                                  if (a0 <= 0x9fa0) {
                                                                                                                                                                                                                    if (a0 == 0x9660)
                                                                                                                                                                                                                      return 1;
                                                                                                                                                                                                                    if (a0 <= 0x9660) {
                                                                                                                                                                                                                      if (a0 == 0x72b6)
                                                                                                                                                                                                                        return 1;
                                                                                                                                                                                                                      if (a0 <= 0x72b6) {
                                                                                                                                                                                                                        if (a0 == 0x7275)
                                                                                                                                                                                                                          return 1;
                                                                                                                                                                                                                        if (a0 <= 0x7275) {
                                                                                                                                                                                                                          if (a0 == 0x6969)
                                                                                                                                                                                                                            return 0;
                                                                                                                                                                                                                          if (a0 <= 0x6969) {
                                                                                                                                                                                                                            if (a0 == 0x5df5)
                                                                                                                                                                                                                              return 1;
                                                                                                                                                                                                                            if (a0 <= 0x5df5) {
                                                                                                                                                                                                                              if (a0 == 0x564c)
                                                                                                                                                                                                                                return 0;
                                                                                                                                                                                                                              if (a0 <= 0x564c) {
                                                                                                                                                                                                                                if (a0 == 0x517b)
                                                                                                                                                                                                                                  return 0;
                                                                                                                                                                                                                                if (a0 <= 0x517b) {
                                                                                                                                                                                                                                  if (a0 == 0x4d5a)
                                                                                                                                                                                                                                    return 1;
                                                                                                                                                                                                                                  if (a0 <= 0x4d5a) {
                                                                                                                                                                                                                                    if (a0 == 0x4d44)
                                                                                                                                                                                                                                      return 1;
                                                                                                                                                                                                                                    if (a0 <= 0x4d44) {
                                                                                                                                                                                                                                      if (a0 == 0x4858)
                                                                                                                                                                                                                                        return 1;
                                                                                                                                                                                                                                      if (a0 <= 0x4858) {
                                                                                                                                                                                                                                        if (a0 == 0x482b)
                                                                                                                                                                                                                                          return 1;
                                                                                                                                                                                                                                        if (a0 <= 0x482b) {
                                                                                                                                                                                                                                          if (a0 == 0x4244)
                                                                                                                                                                                                                                            return 1;
                                                                                                                                                                                                                                          if (a0 <= 0x4244) {
                                                                                                                                                                                                                                            if (a0 == 0x4006)
                                                                                                                                                                                                                                              return 1;
                                                                                                                                                                                                                                            if (a0 <= 0x4006) {
                                                                                                                                                                                                                                              if (a0 == 0x4004)
                                                                                                                                                                                                                                                return 1;
                                                                                                                                                                                                                                              if (a0 <= 0x4004) {
                                                                                                                                                                                                                                                if (a0 == 0x4000)
                                                                                                                                                                                                                                                  return 1;
                                                                                                                                                                                                                                                if (a0 <= 0x4000) {
                                                                                                                                                                                                                                                  if (a0 == 0x3434)
                                                                                                                                                                                                                                                    return 1;
                                                                                                                                                                                                                                                  if (a0 <= 0x3434) {
                                                                                                                                                                                                                                                    if (a0 == 0x2478)
                                                                                                                                                                                                                                                      return 1;
                                                                                                                                                                                                                                                    if (a0 <= 0x2478) {
                                                                                                                                                                                                                                                      if (a0 == 0x2468)
                                                                                                                                                                                                                                                        return 1;
                                                                                                                                                                                                                                                      if (a0 <= 0x2468) {
                                                                                                                                                                                                                                                        if (a0 == 0x1cd1)
                                                                                                                                                                                                                                                          return 1;
                                                                                                                                                                                                                                                        if (a0 <= 0x1cd1) {
                                                                                                                                                                                                                                                          if (a0 == 0x138f)
                                                                                                                                                                                                                                                            return 1;
                                                                                                                                                                                                                                                          if (a0 <= 0x138f) {
                                                                                                                                                                                                                                                            if (a0 == 0x137f)
                                                                                                                                                                                                                                                              return 1;
                                                                                                                                                                                                                                                            if (a0 <= 0x137f) {
                                                                                                                                                                                                                                                              if (a0 == 0x137d)
                                                                                                                                                                                                                                                                return 1;
                                                                                                                                                                                                                                                              if (a0 <= 0x137d) {
                                                                                                                                                                                                                                                                if (a0 == 0x1373)
                                                                                                                                                                                                                                                                  return 1;
                                                                                                                                                                                                                                                                if (a0 <= 0x1373) {
                                                                                                                                                                                                                                                                  if (a0 == 0x7c0)
                                                                                                                                                                                                                                                                    return 1;
                                                                                                                                                                                                                                                                  if (a0 <= 0x7c0) {
                                                                                                                                                                                                                                                                    if (a0 == 0x187)
                                                                                                                                                                                                                                                                      return 1;
                                                                                                                                                                                                                                                                    if (a0 <= 0x187) {
                                                                                                                                                                                                                                                                      if (a0 == 0x2f)
                                                                                                                                                                                                                                                                        return 1;
                                                                                                                                                                                                                                                                      if (a0 == 0x33)
                                                                                                                                                                                                                                                                        return 1;
                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                  }
                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                              }
                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                          }
                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                      }
                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                  }
                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                              }
                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                          }
                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                      }
                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                  }
                                                                                                                                                                                                                                }
                                                                                                                                                                                                                              }
                                                                                                                                                                                                                            }
                                                                                                                                                                                                                          }
                                                                                                                                                                                                                        }
                                                                                                                                                                                                                      }
                                                                                                                                                                                                                    }
                                                                                                                                                                                                                  }
                                                                                                                                                                                                                }
                                                                                                                                                                                                              }
                                                                                                                                                                                                            }
                                                                                                                                                                                                          }
                                                                                                                                                                                                        }
                                                                                                                                                                                                      }
                                                                                                                                                                                                    }
                                                                                                                                                                                                  }
                                                                                                                                                                                                }
                                                                                                                                                                                              }
                                                                                                                                                                                            }
                                                                                                                                                                                          }
                                                                                                                                                                                        }
                                                                                                                                                                                      }
                                                                                                                                                                                    }
                                                                                                                                                                                  }
                                                                                                                                                                                }
                                                                                                                                                                              }
                                                                                                                                                                            }
                                                                                                                                                                          }
                                                                                                                                                                        }
                                                                                                                                                                      }
                                                                                                                                                                    }
                                                                                                                                                                  }
                                                                                                                                                                }
                                                                                                                                                              }
                                                                                                                                                            }
                                                                                                                                                          }
                                                                                                                                                        }
                                                                                                                                                      }
                                                                                                                                                    }
                                                                                                                                                  }
                                                                                                                                                }
                                                                                                                                              }
                                                                                                                                            }
                                                                                                                                          }
                                                                                                                                        }
                                                                                                                                      }
                                                                                                                                    }
                                                                                                                                  }
                                                                                                                                }
                                                                                                                              }
                                                                                                                            }
                                                                                                                          }
                                                                                                                        }
                                                                                                                      }
                                                                                                                    }
                                                                                                                  }
                                                                                                                }
                                                                                                              }
                                                                                                            }
                                                                                                          }
                                                                                                        }
                                                                                                      }
                                                                                                    }
                                                                                                  }
                                                                                                }
                                                                                              }
                                                                                            }
                                                                                          }
                                                                                        }
                                                                                      }
                                                                                    }
                                                                                  }
                                                                                }
                                                                              }
                                                                            }
                                                                          }
                                                                        }
                                                                      }
                                                                    }
                                                                  }
                                                                }
                                                              }
                                                            }
                                                          }
                                                        }
                                                      }
                                                    }
                                                  }
                                                }
                                              }
                                            }
                                          }
                                        }
                                      }
                                    }
                                  }
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
  return 0xffffffff;
}

// Function: sub_4115 @ 0x4115
void sub_4115(int a0)
{
  FILE *v1;
  char *v2;
  
  v2 = dat_18150;
  if (a0) // branch-flip
    fprintf(stderr,gettext("Try \'%s --help\' for more information.\n"),v2);
  else {
    printf(gettext("Usage: %s [OPTION]... [FILE]...\n"),v2);
    printf(gettext("Print the last %d lines of each FILE to standard output.\nWith more than one FILE, precede each with a header giving the file name.\n"),10);
    sub_2a69();
    sub_2a9a();
    v1 = stdout;
    fputs_unlocked(gettext("  -c, --bytes=[+]NUM       output the last NUM bytes; or use -c +NUM to\n                             output starting with byte NUM of each file\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -f, --follow[={name|descriptor}]\n                           output appended data as the file grows;\n                             an absent option argument means \'descriptor\'\n  -F                       same as --follow=name --retry\n"),v1);
    printf(gettext("  -n, --lines=[+]NUM       output the last NUM lines, instead of the last %d;\n                             or use -n +NUM to output starting with line NUM\n      --max-unchanged-stats=N\n                           with --follow=name, reopen a FILE which has not\n                             changed size after N (default %d) iterations\n                             to see if it has been unlinked or renamed\n                             (this is the usual case of rotated log files);\n                             with inotify, this option is rarely useful\n"),10,5);
    v1 = stdout;
    fputs_unlocked(gettext("      --pid=PID            with -f, terminate after process ID, PID dies\n  -q, --quiet, --silent    never output headers giving file names\n      --retry              keep trying to open a file if it is inaccessible\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -s, --sleep-interval=N   with -f, sleep for approximately N seconds\n                             (default 1.0) between iterations;\n                             with inotify and --pid=P, check process P at\n                             least once every N seconds\n  -v, --verbose            always output headers giving file names\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("  -z, --zero-terminated    line delimiter is NUL, not newline\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --help        display this help and exit\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --version     output version information and exit\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("\nNUM may have a multiplier suffix:\nb 512, kB 1000, K 1024, MB 1000*1000, M 1024*1024,\nGB 1000*1000*1000, G 1024*1024*1024, and so on for T, P, E, Z, Y.\nBinary prefixes can be used, too: KiB=K, MiB=M, and so on.\n\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("With --follow (-f), tail defaults to following the file descriptor, which\nmeans that even if a tail\'ed file is renamed, tail will continue to track\nits end.  This default behavior is not desirable when you really want to\ntrack the actual name of the file, not the file descriptor (e.g., log\nrotation).  Use --follow=name in that case.  That causes tail to track the\nnamed file in a way that accommodates renaming, removal and creation.\n"),v1);
    sub_2acb("tail");
  }
  exit(a0); // no-return
}

// Function: sub_431c @ 0x431c
void sub_431c(void)
{
  raise(0xd);
  exit(1); // no-return
}

// Function: sub_4338 @ 0x4338
void sub_4338(void)
{
  unsigned int v1; // stack - 0x18
  unsigned short v2; // stack - 0x14
  unsigned short v3; // stack - 0x12
  
  if (dat_1812f == '\x01') {
    v1 = 1;
    v3 = 0;
    v2 = 0;
    if ((0 <= (int)poll(&v1,1,0)) && (v3 & 0x18))
      sub_431c(); // no-return
  }
}

// Function: sub_43ba @ 0x43ba
bool sub_43ba(struct_2 *a0)
{
  return (a0->field_0x3c == 0) != (a0->field_0x38 == -1);
}

// Function: sub_43ee @ 0x43ee
char * sub_43ee(unsigned long *a0)
{
  char *v1;
  char *v2; // rax
  
  v1 = (char *)*a0;
  if (strcmp(v1,"-")) // branch-flip
    v2 = (char *)*a0;
  else {
    v2 = gettext("standard input");
  }
  return v2;
}

// Function: sub_4435 @ 0x4435
void sub_4435(long a0,unsigned int a1,unsigned long a2,struct_1 *a3,unsigned int a4)
{
  char v1 [16];
  
  *(unsigned int *)(a0 + 0x38) = a1;
  *(unsigned long *)(a0 + 8) = a2;
  v1 = sub_ee31(a3);
  *(long *)(a0 + 0x10) = SUB168(v1,0);
  *(long *)(a0 + 0x18) = SUB168(v1,8);
  *(unsigned long *)(a0 + 0x20) = a3->field_0x0;
  *(unsigned long *)(a0 + 0x28) = a3->field_0x8;
  *(unsigned int *)(a0 + 0x30) = a3->field_0x18;
  *(unsigned int *)(a0 + 0x40) = a4;
  *(unsigned long *)(a0 + 0x58) = 0;
  *(char *)(a0 + 0x34) = 0;
}

// Function: sub_44d5 @ 0x44d5
void sub_44d5(unsigned int a0,char *a1)
{
  unsigned long v1; // rax
  char *v2; // rax
  
  if ((a0 != 0xffffffff) && (a0)) {
    if (close(a0)) {
      v1 = sub_e8ca(4,a1);
      v2 = gettext("closing %s (fd=%d)");
      error(0,*__errno_location(),v2,v1,(unsigned long)a0);
    }
  }
}

// Function: sub_4559 @ 0x4559
void sub_4559(char *a0) // ternary
{
  char *v1; // rax
  
  v1 = (dat_18020) ? "" : "\n"; // branch-flip
  printf("%s==> %s <==\n",v1,a0);
  dat_18020 = 0;
}

// Function: sub_45a9 @ 0x45a9
void sub_45a9(void *a0,unsigned long a1)
{
  unsigned long v1; // rax
  char *v2; // rax
  
  if (a1) {
    if (fwrite_unlocked(a0,1,a1,stdout) < a1) {
      clearerr_unlocked(stdout);
      v1 = sub_e8ca(4,"standard output");
      v2 = gettext("error writing %s");
      error(1,*__errno_location(),v2,v1);
      return;
    }
  }
}

// Function: sub_4649 @ 0x4649
long sub_4649(bool a0,char *a1,unsigned int a2,unsigned long a3)
{
  unsigned long v1;
  unsigned long v2; // rax
  char *v3; // rax
  long v4; // rax
  char v5 [8200];
  bool v6; // stack - 0x204c
  long v7; // stack - 0x2048
  unsigned long v8; // stack - 0x2040
  
  v7 = 0;
  v6 = a0;
  v8 = a3;
  do {
    do {
      v1 = v8;
      if (0x2001 <= v8)
        v1 = 0x2000;
      v1 = sub_ed34(a2,v5,v1);
      if (v1 == 0xffffffffffffffff) {
        if (*__errno_location() == 0xb)
          return v7;
        v2 = sub_e8ca(4,a1);
        v3 = gettext("error reading %s");
        error(1,*__errno_location(),v3,v2);
        return v4;
      }
      if (!v1)
        return v7;
      if (v6) {
        sub_4559(a1);
        v6 = 0;
      }
      sub_45a9(v5,v1);
      v7 += v1;
    } while (a3 == 0xffffffffffffffff);
    v8 -= v1;
    if (!v8)
      return v7;
  } while (a3 != 0xfffffffffffffffe);
  return v7;
}

// Function: sub_4804 @ 0x4804
long sub_4804(unsigned int a0,long a1,int a2,char *a3)
{
  long v1; // rax
  unsigned long v2; // rax
  unsigned long v3; // rax
  char *v4; // rax
  char v5 [24];
  
  v1 = lseek(a0,a1,a2);
  if (0 <= v1)
    return v1;
  v2 = sub_cade(a1,v5);
  if (a2 != 2) { // branch-flip
    if (2 < a2)
      abort(); // no-return, return-dupe
    if (a2) { // branch-flip
      if (a2 != 1) {
        abort();
      }
      v3 = sub_ea5c(0,3,a3);
      v4 = gettext("%s: cannot seek to relative offset %s");
      error(0,*__errno_location(),v4,v3,v2);
    }
    else {
      v3 = sub_ea5c(0,3,a3);
      v4 = gettext("%s: cannot seek to offset %s");
      error(0,*__errno_location(),v4,v3,v2);
    }
  }
  else {
    v3 = sub_ea5c(0,3,a3);
    v4 = gettext("%s: cannot seek to end-relative offset %s");
    error(0,*__errno_location(),v4,v3,v2);
  }
  exit(1); // no-return
}

// Function: sub_49bd @ 0x49bd
unsigned long sub_49bd(char *a0,unsigned int a1,long a2,long a3,unsigned long a4,long *a5) // return-dupe x2
{
  unsigned long v1; // rax
  void *v10; // stack - 0x2030
  char *v2; // rax
  long v3;
  char v4 [8200];
  bool v5; // zf
  long v6; // stack - 0x2060
  unsigned long v7; // stack - 0x2048
  long v8; // stack - 0x2040
  unsigned long v9; // stack - 0x2038
  
  if (!a2)
    return 1;
  v7 = (long)(a4 - a3) % 0x2000;
  if (!v7)
    v7 = 0x2000;
  v8 = a4 - v7;
  sub_4804(a1,v8,0,a0);
  v7 = sub_ed34(a1,v4,v7);
  if (v7 == 0xffffffffffffffff) {
    v1 = sub_e8ca(4,a0);
    v2 = gettext("error reading %s");
    error(0,*__errno_location(),v2,v1);
    return 0;
  }
  *a5 = v8 + v7;
  v6 = a2;
  if ((v7) && (v4[v7 - 1] != dat_18132))
    v6 = a2 + -1;
  do {
    v9 = v7;
    while (v9) {
      v10 = memrchr(v4,(int)dat_18132,v9);
      if (!v10) break;
      v9 = (long)v10 - (long)v4;
      v3 = v6 + -1;
      v5 = v6 == 0;
      v6 = v3;
      if (v5) {
        sub_45a9((void *)((long)v10 + 1),(v7 - v9) - 1);
        v3 = sub_4649(0,a0,a1,a4 - (v7 + v8));
        *a5 = v3 + *a5;
        return 1;
      }
    }
    if (v8 == a3) {
      sub_4804(a1,a3,0,a0);
      *a5 = a3 + sub_4649(0,a0,a1,a4);
      return 1;
    }
    v8 -= 0x2000;
    sub_4804(a1,v8,0,a0);
    v7 = sub_ed34(a1,v4,0x2000);
    if (v7 == 0xffffffffffffffff) {
      v1 = sub_e8ca(4,a0);
      v2 = gettext("error reading %s");
      error(0,*__errno_location(),v2,v1);
      return 0;
    }
    *a5 = v8 + v7;
    if (!v7)
      return 1;
  } while( true );
}

// Function: sub_4df7 @ 0x4df7
char sub_4df7(char *a0,unsigned int a1,unsigned long a2,long *a3)
{
  long *v1;
  void *v10; // stack - 0x58
  unsigned long v11; // stack - 0x50
  void *v12; // stack - 0x48
  void *v13; // stack - 0x40
  long v14; // stack - 0x38
  void *v2;
  long v3;
  unsigned long v4; // rax
  char *v5; // rax
  int v6; // edx
  char v7; // stack - 0x69
  void *v8; // stack - 0x68
  void *v9; // stack - 0x60
  
  v11 = 0;
  v7 = 1;
  v9 = (void *)sub_f96c(0x2018);
  v8 = v9;
  *(unsigned long *)((long)v9 + 0x2008) = 0;
  *(unsigned long *)((long)v9 + 0x2000) = *(unsigned long *)((long)v9 + 0x2008);
  *(unsigned long *)((long)v9 + 0x2010) = 0;
  v10 = (void *)sub_f96c(0x2018);
  while ((v3 = sub_ed34(a1,v10,_DT_INIT), v3 && (v3 != -1))) {
    *(long *)((long)v10 + 0x2000) = v3;
    *a3 = *a3 + v3;
    *(unsigned long *)((long)v10 + 0x2008) = 0;
    *(unsigned long *)((long)v10 + 0x2010) = 0;
    v12 = v10;
    while (v12 = memchr(v12,(int)dat_18132,(long)v10 + (v3 - (long)v12)), v12) {
      v12 = (void *)((long)v12 + 1);
      *(long *)((long)v10 + 0x2008) = *(long *)((long)v10 + 0x2008) + 1;
    }
    v11 += *(long *)((long)v10 + 0x2008);
    if (0x2000 <= (unsigned long)(*(long *)((long)v9 + 0x2000) + *(long *)((long)v10 + 0x2000))) { // branch-flip
      *(void **)((long)v9 + 0x2010) = v10;
      v9 = *(void **)((long)v9 + 0x2010);
      if (v11 - *(long *)((long)v8 + 0x2008) <= a2) // branch-flip
        v10 = (void *)sub_f96c(0x2018);
      else {
        v10 = v8;
        v11 -= *(long *)((long)v8 + 0x2008);
        v8 = *(void **)((long)v8 + 0x2010);
      }
    }
    else {
      memcpy((void *)((long)v9 + *(long *)((long)v9 + 0x2000)),v10,*(unsigned long *)((long)v10 + 0x2000));
      *(long *)((long)v9 + 0x2000) = *(long *)((long)v9 + 0x2000) + *(long *)((long)v10 + 0x2000);
      *(long *)((long)v9 + 0x2008) = *(long *)((long)v9 + 0x2008) + *(long *)((long)v10 + 0x2008);
    }
  }
  free(v10);
  if (v3 != -1) { // branch-flip
    if ((!*(long *)((long)v9 + 0x2000)) || (!a2)) goto label_5272;
    if (*(char *)((long)v9 + *(long *)((long)v9 + 0x2000) + -1) != dat_18132) {
      *(long *)((long)v9 + 0x2008) = *(long *)((long)v9 + 0x2008) + 1;
      v11 += 1;
    }
    v10 = v8;
    for (; a2 < v11 - *(long *)((long)v10 + 0x2008); v11 = v11 - *v1) {
      v1 = (long *)((long)v10 + 0x2008);
      v10 = *(void **)((long)v10 + 0x2010);
    }
    v13 = v10;
    v3 = *(long *)((long)v10 + 0x2000);
    if (a2 < v11) {
      for (v14 = v11 - a2; v14; v14 = v14 + -1) {
        v6 = (int)dat_18132;
        v13 = (void *)((long)rawmemchr(v13,v6) + 1);
      }
    }
    sub_45a9(v13,(long)v10 + (v3 - (long)v13));
    for (v10 = *(void **)((long)v10 + 0x2010); v10; v10 = *(void **)((long)v10 + 0x2010)) {
      sub_45a9(v10,*(unsigned long *)&_DT_INIT[(long)v10]);
    }
  }
  else {
    v4 = sub_e8ca(4,a0);
    v5 = gettext("error reading %s");
    error(0,*__errno_location(),v5,v4);
    v7 = 0;
  }
label_5272:
  while (v8) {
    v2 = *(void **)((long)v8 + 0x2010);
    free(v8);
    v8 = v2;
  }
  return v7;
}

// Function: sub_5286 @ 0x5286
char sub_5286(char *a0,unsigned int a1,unsigned long a2,long *a3)
{
  long *v1;
  long v10; // stack - 0x30
  unsigned long v11; // stack - 0x28
  void *v2;
  long v3; // rax
  unsigned long v4; // rax
  char *v5; // rax
  char v6; // stack - 0x49
  void *v7; // stack - 0x48
  void *v8; // stack - 0x40
  void *v9; // stack - 0x38
  
  v11 = 0;
  v6 = 1;
  v8 = (void *)sub_f96c(0x2010);
  v7 = v8;
  *(unsigned long *)((long)v8 + 0x2000) = 0;
  *(unsigned long *)((long)v8 + 0x2008) = 0;
  v9 = (void *)sub_f96c(0x2010);
  while ((v3 = sub_ed34(a1,v9,_DT_INIT), v3 && (v3 != -1))) {
    *a3 = *a3 + v3;
    *(long *)((long)v9 + 0x2000) = v3;
    *(unsigned long *)((long)v9 + 0x2008) = 0;
    v11 += *(long *)((long)v9 + 0x2000);
    if (0x2000 <= (unsigned long)(*(long *)((long)v8 + 0x2000) + *(long *)((long)v9 + 0x2000))) { // branch-flip
      *(void **)((long)v8 + 0x2008) = v9;
      v8 = *(void **)((long)v8 + 0x2008);
      if (v11 - *(long *)((long)v7 + 0x2000) <= a2) // branch-flip
        v9 = (void *)sub_f96c(0x2010);
      else {
        v9 = v7;
        v11 -= *(long *)((long)v7 + 0x2000);
        v7 = *(void **)((long)v7 + 0x2008);
      }
    }
    else {
      memcpy((void *)((long)v8 + *(long *)((long)v8 + 0x2000)),v9,*(unsigned long *)((long)v9 + 0x2000));
      *(long *)((long)v8 + 0x2000) = *(long *)((long)v8 + 0x2000) + *(long *)((long)v9 + 0x2000);
    }
  }
  free(v9);
  if (v3 != -1) { // branch-flip
    v9 = v7;
    for (; a2 < v11 - *(long *)((long)v9 + 0x2000); v11 = v11 - *v1) {
      v1 = (long *)((long)v9 + 0x2000);
      v9 = *(void **)((long)v9 + 0x2008);
    }
    if (v11 <= a2) // branch-flip
      v10 = 0;
    else {
      v10 = v11 - a2;
    }
    sub_45a9((void *)(v10 + (long)v9),*(long *)((long)v9 + 0x2000) - v10);
    for (v9 = *(void **)((long)v9 + 0x2008); v9; v9 = *(void **)((long)v9 + 0x2008)) {
      sub_45a9(v9,*(unsigned long *)&_DT_INIT[(long)v9]);
    }
  }
  else {
    v4 = sub_e8ca(4,a0);
    v5 = gettext("error reading %s");
    error(0,*__errno_location(),v5,v4);
    v6 = 0;
  }
  while (v7) {
    v2 = *(void **)((long)v7 + 0x2008);
    free(v7);
    v7 = v2;
  }
  return v6;
}

// Function: sub_55ae @ 0x55ae
unsigned long sub_55ae(char *a0,unsigned int a1,unsigned long a2,long *a3)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  char *v3; // rax
  char v4 [8200];
  unsigned long v5; // stack - 0x2050
  
  v5 = a2;
  while( true ) {
    if (!v5)
      return 0;
    v1 = sub_ed34(a1,v4,_DT_INIT);
    if (!v1)
      return 0xffffffff;
    if (v1 == 0xffffffffffffffff) {
      v2 = sub_e8ca(4,a0);
      v3 = gettext("error reading %s");
      error(0,*__errno_location(),v3,v2);
      return 1;
    }
    *a3 = *a3 + v1;
    if (v5 < v1) break;
    v5 -= v1;
  }
  sub_45a9(&v4[v5],v1 - v5);
  return 0;
}

// Function: sub_5741 @ 0x5741
unsigned long sub_5741(char *a0,unsigned int a1,long a2,long *a3) // return-dupe
{
  long v1; // rax
  unsigned long v2; // rax
  char *v3; // rax
  char *v4; // rax
  void *v5; // rax
  char v6 [8200];
  long v7; // stack - 0x2060
  char *v8; // stack - 0x2040
  
  v7 = a2;
  if (!a2)
    return 0;
  while( true ) {
    v1 = sub_ed34(a1,v6,_DT_INIT);
    if (!v1)
      return 0xffffffff;
    if (v1 == -1) break;
    v4 = &v6[v1];
    *a3 = *a3 + v1;
    v8 = v6;
    while (v5 = memchr(v8,(int)dat_18132,(long)v4 - (long)v8), v5) {
      v8 = (char *)((long)v5 + 1);
      v7 -= 1;
      if (!v7) {
        if (v4 <= v8)
          return 0;
        sub_45a9(v8,(long)v4 - (long)v8);
        return 0;
      }
    }
  }
  v2 = sub_e8ca(4,a0);
  v3 = gettext("error reading %s");
  error(0,*__errno_location(),v3,v2);
  return 1;
}

// Function: sub_5938 @ 0x5938
char sub_5938(int a0,char *a1)
{
  int v1; // eax
  unsigned long v2; // rax
  char *v3; // rax
  statfs v4; // stack - 0x98
  char v5; // stack - 0x9d
  
  v5 = 1;
  if (fstatfs(a0,&v4)) { // branch-flip
    if (*__errno_location() != 0x26) {
      v2 = sub_e8ca(4,a1);
      v3 = gettext("cannot determine location of %s. reverting to polling");
      error(0,*__errno_location(),v3,v2);
    }
  }
  else {
    v1 = sub_2da1(v4._0_8_);
    if (v1 != 1) { // branch-flip
      if ((1 < v1) || ((v1 != -1 && (v1))))
        __assert_fail("!\"unexpected return value from is_local_fs_type\"","src/tail.c",0x3cc,"fremote"); // no-return
    }
    else {
      v5 = 0;
    }
  }
  return v5;
}

// Function: sub_5a69 @ 0x5a69
void sub_5a69(struct_2 *a0,unsigned char a1) // return-dupe, ternary x2, warn: iteboolean: re-rolled 2 0/1 select diamond(s) to a boolean assignment in sub_5a69
{
  char v1;
  unsigned long v10; // rdx
  stat v11; // stack - 0xb8
  bool v12; // zf
  int v2;
  bool v3;
  bool v4;
  char v5; // al
  int v6; // eax
  char *v7;
  char *v8; // rax
  unsigned int v9; // ecx
  
  v3 = 1;
  v7 = a0->field_0x0;
  v12 = strcmp(v7,"-") == 0;
  v1 = a0->field_0x36;
  v2 = a0->field_0x3c;
  if (v12)
    v6 = 0;
  else {
    v10 = (a1) ? 0 : 0x800; // branch-flip
    v6 = sub_af09(a0->field_0x0,v10);
  }
  if (!sub_43ba(a0))
    __assert_fail("valid_file_spec (f)","src/tail.c",0x3e2,"recheck"); // no-return
  v5 = ((dat_1812c != '\x01') || (v6 != -1));
  a0->field_0x36 = v5;
  if (((dat_1813a != '\x01') && (v7 = a0->field_0x0, !lstat(v7,&v11))) && ((v11._24_4_ & 0xf000) == 0xa000)) {
    v3 = 0;
    a0->field_0x3c = 0xffffffff;
    a0->field_0x34 = 1;
    v7 = (char *)sub_e8ca(4,(char *)sub_43ee(a0));
    error(0,0,gettext("%s has been replaced with an untailable symbolic link"),v7);
  }
  else if ((v6 != -1) && (0 <= fstat(v6,&v11))) {
    if ((((v11._24_4_ & 0xf000) != 0x8000) && ((v11._24_4_ & 0xf000) != 0x1000)) && (((v11._24_4_ & 0xf000) != 0xc000 && ((v11._24_4_ & 0xf000) != 0x2000)))) {
      v3 = 0;
      a0->field_0x3c = 0xffffffff;
      a0->field_0x36 = 0;
      v5 = ((dat_1812c != '\x01') || (dat_18010 != 1));
      a0->field_0x34 = v5;
      if ((v1) || (v2 != a0->field_0x3c)) {
        if (a0->field_0x34) // branch-flip
          v7 = gettext("; giving up on this name");
        else {
          v7 = "";
        }
        v8 = (char *)sub_e8ca(4,(char *)sub_43ee(a0));
        error(0,0,gettext("%s has been replaced with an untailable file%s"),v8,v7);
      }
    }
    else {
      v7 = (char *)sub_43ee(a0);
      a0->field_0x35 = sub_5938(v6,v7);
      if ((a0->field_0x35) && (dat_1813a != '\x01')) {
        v3 = 0;
        a0->field_0x3c = 0xffffffff;
        v7 = (char *)sub_e8ca(4,(char *)sub_43ee(a0));
        error(0,0,gettext("%s has been replaced with an untailable remote file"),v7);
        a0->field_0x34 = 1;
        a0->field_0x35 = 1;
      }
      else {
        a0->field_0x3c = 0;
      }
    }
  }
  else {
    v3 = 0;
    a0->field_0x3c = *__errno_location();
    if (a0->field_0x36 != '\x01') { // branch-flip
      if (v1) {
        v7 = (char *)sub_e8ca(4,(char *)sub_43ee(a0));
        v8 = gettext("%s has become inaccessible");
        error(0,a0->field_0x3c,v8,v7);
      }
    }
    else if (v2 != *__errno_location()) {
      v7 = (char *)sub_ea5c(0,3,(char *)sub_43ee(a0));
      error(0,*__errno_location(),"%s",v7);
    }
  }
  v4 = 0;
  if (v3) {
    if ((v2) && (v2 != 2)) {
      v4 = 1;
      if (a0->field_0x38 != -1)
        __assert_fail("f->fd == -1","src/tail.c",0x42f,"recheck"); // no-return
      v7 = (char *)sub_e8ca(4,(char *)sub_43ee(a0));
      error(0,0,gettext("%s has become accessible"),v7);
    }
    else if (a0->field_0x38 != -1) { // branch-flip
      if ((a0->field_0x28 != v11._8_8_) || (a0->field_0x20 != v11._0_8_)) {
        v4 = 1;
        v7 = (char *)sub_e8ca(4,(char *)sub_43ee(a0));
        error(0,0,gettext("%s has been replaced;  following new file"),v7);
        v7 = (char *)sub_43ee(a0);
        sub_44d5(a0->field_0x38,v7);
      }
      else {
        sub_44d5(v6,(char *)sub_43ee(a0));
      }
    }
    else {
      v4 = 1;
      v7 = (char *)sub_e8ca(4,(char *)sub_43ee(a0));
      error(0,0,gettext("%s has appeared;  following new file"),v7);
    }
  }
  else {
    sub_44d5(v6,(char *)sub_43ee(a0));
    v7 = (char *)sub_43ee(a0);
    sub_44d5(a0->field_0x38,v7);
    a0->field_0x38 = 0xffffffff;
  }
  if (!v4)
    return;
  v9 = (v12) ? 0xffffffff : (unsigned int)a1;
  sub_4435(a0,v6,0,&v11,v9);
  sub_4804(v6,0,0,(char *)sub_43ee(a0));
}

// Function: sub_6245 @ 0x6245
unsigned long sub_6245(long a0,unsigned long a1) // return-dupe
{
  unsigned long v1; // stack - 0x10
  
  if ((dat_1812c) && (dat_18010 == 1))
    return 1;
  v1 = 0;
  while( true ) {
    if (a1 <= v1)
      return 0;
    if (0 <= *(int *)(a0 + v1 * 0x60 + 0x38)) break;
    if ((*(char *)(a0 + v1 * 0x60 + 0x34) != '\x01') && (dat_1812c))
      return 1;
    v1 += 1;
  }
  return 1;
}

// Function: sub_62f3 @ 0x62f3
void sub_62f3(long a0,unsigned long a1) // ternary, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_62f3
{
  int v1;
  long v10;
  char *v11; // rax
  char *v12; // rax
  stat v13; // stack - 0xb8
  bool v14;
  char v15 [16];
  unsigned long v16; // stack - 0xe0
  unsigned long v17; // stack - 0xd8
  unsigned long v18; // stack - 0xd0
  unsigned int v2;
  unsigned long v3;
  bool v4;
  unsigned char v5; // al
  int v6; // eax
  unsigned int v7; // eax
  unsigned long v8; // rax
  long v9;
  
  v5 = !((((dat_18134) || (dat_18010 != 2)) || (a1 != 1)) || ((*(int *)(a0 + 0x38) == -1 || ((*(unsigned int *)(a0 + 0x30) & 0xf000) == 0x8000))));
  v4 = 0;
  v16 = a1 - 1;
  do {
    v14 = 0;
    for (v17 = 0; v17 < a1; v17 = v17 + 1) {
      if (!*(char *)(a0 + v17 * 0x60 + 0x34)) {
        if (0 <= *(int *)(a0 + v17 * 0x60 + 0x38)) { // branch-flip
          v1 = *(int *)(a0 + v17 * 0x60 + 0x38);
          v12 = (char *)sub_43ee((unsigned long *)(a0 + v17 * 0x60));
          v2 = *(unsigned int *)(a0 + v17 * 0x60 + 0x30);
          if (*(unsigned int *)(a0 + v17 * 0x60 + 0x40) != (unsigned int)v5) {
            v6 = sub_aa63(v1,3);
            v7 = (v5) ? 0 : 0x800; // branch-flip
            if ((0 <= v6) && (((v7 | v6) == v6 || (sub_aa63(v1,4,v7 | v6) != -1))))
              *(unsigned int *)(v17 * 0x60 + a0 + 0x40) = (unsigned int)v5;
            else if (((*(unsigned int *)(a0 + v17 * 0x60 + 0x30) & 0xf000) != 0x8000) || (*__errno_location() != 1)) {
              v8 = sub_ea5c(0,3,v12);
              v12 = gettext("%s: cannot change nonblocking mode");
              error(1,*__errno_location(),v12,v8);
              return;
            }
          }
          if (!*(int *)(a0 + v17 * 0x60 + 0x40)) {
            if (fstat(v1,&v13)) {
              *(unsigned int *)(a0 + v17 * 0x60 + 0x38) = 0xffffffff;
              *(int *)(v17 * 0x60 + a0 + 0x3c) = *__errno_location();
              v12 = (char *)sub_ea5c(0,3,v12);
              error(0,*__errno_location(),"%s",v12);
              close(v1);
              goto label_6bb6;
            }
            if ((*(unsigned int *)(a0 + v17 * 0x60 + 0x30) == v13._24_4_) && (((v13._24_4_ & 0xf000) != 0x8000 || (*(long *)(a0 + v17 * 0x60 + 8) == v13._48_8_)))) {
              v15 = sub_ee31(&v13);
              v9 = a0 + v17 * 0x60;
              v10 = *(long *)(v9 + 0x10);
              v9 = *(long *)(v9 + 0x18);
              if (!sub_eeaf(v10,v9,SUB168(v15,0),SUB168(v15,8))) {
                v10 = a0 + v17 * 0x60;
                v3 = *(unsigned long *)(v10 + 0x58);
                *(unsigned long *)(v10 + 0x58) = v3 + 1;
                if ((dat_18018 <= v3) && (dat_18010 == 1)) {
                  sub_5a69((struct_2 *)(a0 + v17 * 0x60),*(int *)(a0 + v17 * 0x60 + 0x40) != 0);
                  *(unsigned long *)(a0 + v17 * 0x60 + 0x58) = 0;
                }
                goto label_6bb6;
              }
            }
            v10 = v17 * 0x60 + a0;
            v15 = sub_ee31(&v13);
            *(long *)(v10 + 0x10) = SUB168(v15,0);
            *(long *)(v10 + 0x18) = SUB168(v15,8);
            *(unsigned int *)(v17 * 0x60 + a0 + 0x30) = v13._24_4_;
            *(unsigned long *)(a0 + v17 * 0x60 + 0x58) = 0;
            if (((v2 & 0xf000) == 0x8000) && ((long)v13._48_8_ < *(long *)(a0 + v17 * 0x60 + 8))) {
              v11 = (char *)sub_ea5c(0,3,v12);
              error(0,0,gettext("%s: file truncated"),v11);
              sub_4804(v1,0,0,v12);
              *(unsigned long *)(a0 + v17 * 0x60 + 8) = 0;
            }
            if (v17 != v16) {
              if (dat_18131)
                sub_4559(v12);
              v16 = v17;
            }
          }
          if (*(int *)(a0 + v17 * 0x60 + 0x40)) // branch-flip
            v18 = 0xfffffffffffffffe;
          else if (((v2 & 0xf000) != 0x8000) || (!*(char *)(a0 + v17 * 0x60 + 0x35)))
            v18 = 0xffffffffffffffff;
          else {
            v18 = v13._48_8_ - *(long *)(a0 + v17 * 0x60 + 8);
          }
          v10 = sub_4649(0,v12,v1,v18);
          v14 = v10 || v14;
          *(long *)(a0 + v17 * 0x60 + 8) = *(long *)(a0 + v17 * 0x60 + 8) + v10;
        }
        else {
          sub_5a69((struct_2 *)(a0 + v17 * 0x60),v5);
        }
      }
label_6bb6:
    }
    if (sub_6245(a0,a1) != '\x01') {
      error(0,0,gettext("no files remaining"));
      return;
    }
    if (((!v14) || (v5)) && (fflush_unlocked(stdout))) {
      v12 = gettext("write error");
      error(1,*__errno_location(),v12);
      return;
    }
    sub_4338();
    if (!v14) {
      if (v4)
        return;
      if (((dat_18134) && (kill(dat_18134,0))) && (*__errno_location() != 1))
        v4 = 1;
      else {
        v4 = 0;
      }
      if ((!v4) && (sub_101af())) {
        v12 = gettext("cannot read realtime clock");
        error(1,*__errno_location(),v12);
        return;
      }
    }
  } while( true );
}

// Function: sub_6d57 @ 0x6d57
unsigned long sub_6d57(long a0,unsigned long a1)
{
  unsigned long v1; // stack - 0x10
  
  v1 = 0;
  while( true ) {
    if (a1 <= v1)
      return 0;
    if ((0 <= *(int *)(a0 + v1 * 0x60 + 0x38)) && (*(char *)(a0 + v1 * 0x60 + 0x35))) break;
    v1 += 1;
  }
  return 1;
}

// Function: sub_6dd3 @ 0x6dd3
unsigned long sub_6dd3(long a0,unsigned long a1)
{
  unsigned long v1; // stack - 0x10
  
  v1 = 0;
  while( true ) {
    if (a1 <= v1)
      return 0;
    if ((0 <= *(int *)(a0 + v1 * 0x60 + 0x38)) && (*(char *)(a0 + v1 * 0x60 + 0x35) != '\x01')) break;
    v1 += 1;
  }
  return 1;
}

// Function: sub_6e52 @ 0x6e52
unsigned long sub_6e52(long a0,unsigned long a1)
{
  char *v1;
  stat v2; // stack - 0xa8
  unsigned long v3; // stack - 0xb0
  
  v3 = 0;
  while( true ) {
    if (a1 <= v3)
      return 0;
    v1 = *(char **)(a0 + v3 * 0x60);
    if ((!lstat(v1,&v2)) && ((v2._24_4_ & 0xf000) == 0xa000)) break;
    v3 += 1;
  }
  return 1;
}

// Function: sub_6f11 @ 0x6f11
unsigned long sub_6f11(long a0,unsigned long a1)
{
  unsigned long v1; // stack - 0x10
  
  v1 = 0;
  while( true ) {
    if (a1 <= v1)
      return 0;
    if (((0 <= *(int *)(a0 + v1 * 0x60 + 0x38)) && ((*(unsigned int *)(a0 + v1 * 0x60 + 0x30) & 0xf000) != 0x8000)) && ((*(unsigned int *)(a0 + v1 * 0x60 + 0x30) & 0xf000) != 0x1000)) break;
    v1 += 1;
  }
  return 1;
}

// Function: sub_6fc5 @ 0x6fc5
unsigned long sub_6fc5(long a0,unsigned long a1)
{
  char *v1;
  unsigned long v2; // stack - 0x10
  
  v2 = 0;
  while( true ) {
    if (a1 <= v2)
      return 0;
    if ((*(char *)(a0 + v2 * 0x60 + 0x34) != '\x01') && (v1 = *(char **)(a0 + v2 * 0x60), !strcmp(v1,"-"))) break;
    v2 += 1;
  }
  return 1;
}

// Function: sub_705a @ 0x705a
unsigned long sub_705a(long a0,unsigned long a1)
{
  return (unsigned long)(long)*(int *)(a0 + 0x44) % a1;
}

// Function: sub_7089 @ 0x7089
unsigned int sub_7089(long a0,long a1)
{
  return CONCAT31((undefined3)((unsigned int)*(int *)(a1 + 0x44) >> 8),*(int *)(a0 + 0x44) == *(int *)(a1 + 0x44));
}

// Function: sub_70be @ 0x70be
void sub_70be(unsigned long *a0,long *a1) // return-dupe x3, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_70be
{
  int v1;
  long v2;
  char v3; // al
  char *v4; // rax
  char *v5; // rax
  long v6;
  stat v7; // stack - 0xb8
  char v8 [16];
  
  if (*(int *)&a0[7] == -1)
    return;
  v4 = (char *)sub_43ee(a0);
  v1 = *(int *)&a0[7];
  if (fstat(v1,&v7)) {
    *(int *)((long)a0 + 0x3c) = *__errno_location();
    sub_44d5(*(unsigned int *)&a0[7],v4);
    *(unsigned int *)&a0[7] = 0xffffffff;
    return;
  }
  if (((*(unsigned int *)&a0[6] & 0xf000) != 0x8000) || ((long)a0[1] <= (long)v7._48_8_)) {
    if (((*(unsigned int *)&a0[6] & 0xf000) == 0x8000) && (v7._48_8_ == a0[1])) {
      v8 = sub_ee31(&v7);
      v6 = a0[2];
      v2 = a0[3];
      if (!sub_eeaf(v6,v2,SUB168(v8,0),SUB168(v8,8)))
        return;
    }
  }
  else {
    v5 = (char *)sub_ea5c(0,3,v4);
    error(0,0,gettext("%s: file truncated"),v5);
    sub_4804(*(unsigned int *)&a0[7],0,0,v4);
    a0[1] = 0;
  }
  v3 = ((dat_18131) && (a0 != (unsigned long *)*a1));
  v6 = sub_4649((unsigned char)v3,v4,*(unsigned int *)&a0[7],0xffffffffffffffff);
  a0[1] = v6 + a0[1];
  if (!v6)
    return;
  *a1 = (long)a0;
  if (!fflush_unlocked(stdout))
    return;
  v4 = gettext("write error");
  error(1,*__errno_location(),v4);
}

// Function: sub_7379 @ 0x7379
void sub_7379(double a0,unsigned int a1,long a2,unsigned long a3,long *a4) // warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_7379
{
  char v1;
  int *v10; // rax
  unsigned long v11;
  stat v12; // stack - 0xb8
  long v13; // stack - 0x190
  char v14 [68];
  bool v15;
  int v16; // stack - 0x1a4
  unsigned int v17; // stack - 0x1a0
  int v18; // stack - 0x19c
  int v19; // stack - 0x198
  long v2;
  unsigned long v20; // stack - 0x188
  void *v21; // stack - 0x180
  unsigned long v22; // stack - 0x178
  unsigned long v23; // stack - 0x170
  unsigned long v24; // stack - 0x168
  unsigned long *v25; // stack - 0x160
  unsigned long v26; // stack - 0x158
  long v27; // stack - 0x150
  unsigned long v28; // stack - 0x148
  long v29; // stack - 0x140
  bool v3;
  double v30; // stack - 0x138
  int *v31; // stack - 0x130
  int *v32; // stack - 0x128
  unsigned long *v33; // stack - 0x120
  int v34; // stack - 0xd4
  bool v4;
  bool v5;
  bool v6;
  int v7;
  long v8;
  char *v9;
  
  v16 = 3;
  v5 = 0;
  v4 = 0;
  v3 = 0;
  v15 = 0;
  v6 = 0;
  v20 = 0;
  v22 = 0;
  v23 = 0;
  v27 = sub_b9fe(a3,0,sub_705a,sub_7089,0);
  if (!v27)
    sub_ffa4(); // no-return
  *a4 = v27;
  v17 = 2;
  if (dat_18010 == 1)
    v17 = 0xc06;
  for (v24 = 0; v24 < a3; v24 = v24 + 1) {
    if (*(char *)(a2 + v24 * 0x60 + 0x34) != '\x01') {
      v28 = strlen(*(char **)(a2 + v24 * 0x60));
      if (v20 < v28)
        v20 = v28;
      *(unsigned int *)(a2 + v24 * 0x60 + 0x44) = 0xffffffff;
      if (dat_18010 == 1) {
        v29 = sub_a943(*(char **)(a2 + v24 * 0x60));
        v1 = *(char *)(v29 + *(long *)(a2 + v24 * 0x60));
        v8 = sub_a685(*(char **)(a2 + v24 * 0x60));
        *(long *)(a2 + v24 * 0x60 + 0x50) = v8 - *(long *)(a2 + v24 * 0x60);
        *(char *)(v29 + *(long *)(a2 + v24 * 0x60)) = 0;
        if (v29) // branch-flip
          v9 = *(char **)(a2 + v24 * 0x60);
        else {
          v9 = ".";
        }
        v8 = v24 * 0x60;
        *(unsigned int *)(v8 + a2 + 0x48) = inotify_add_watch(a1,v9,0x784);
        *(char *)(*(long *)(a2 + v24 * 0x60) + v29) = v1;
        if (*(int *)(a2 + v24 * 0x60 + 0x48) <= -1) {
          if (*__errno_location() != 0x1c) { // branch-flip
            v11 = sub_e8ca(4,*(char **)(a2 + v24 * 0x60));
            v9 = gettext("cannot watch parent directory of %s");
            error(0,*__errno_location(),v9,v11);
          }
          else {
            error(0,0,gettext("inotify resources exhausted"));
          }
          v3 = 1;
          break;
        }
      }
      v11 = *(unsigned long *)(a2 + v24 * 0x60);
      v8 = v24 * 0x60;
      *(unsigned int *)(v8 + a2 + 0x44) = inotify_add_watch(a1,v11,v17);
      if (0 <= *(int *)(a2 + v24 * 0x60 + 0x44)) { // branch-flip
        v8 = v24 * 0x60;
        if (!sub_c73f(v27,v8 + a2))
          sub_ffa4(); // no-return
        v5 = 1;
      }
      else {
        if (*(int *)(a2 + v24 * 0x60 + 0x38) != -1)
          v4 = 1;
        if ((*__errno_location() == 0x1c) || (*__errno_location() == 0xc)) {
          v15 = 1;
          error(0,0,gettext("inotify resources exhausted"));
          break;
        }
        if (*__errno_location() != *(int *)(a2 + v24 * 0x60 + 0x3c)) {
          v11 = sub_e8ca(4,*(char **)(a2 + v24 * 0x60));
          v9 = gettext("cannot watch %s");
          error(0,*__errno_location(),v9,v11);
        }
      }
    }
  }
  if (((v15) || (v3)) || ((dat_18010 == 2 && (v4))))
    return;
  if ((dat_18010 == 2) && (!v5))
    exit(1); // no-return
  v13 = a2 + a3 * 0x60 + -0x60;
  for (v24 = 0; v24 < a3; v24 = v24 + 1) {
    if (*(char *)(a2 + v24 * 0x60 + 0x34) != '\x01') {
      if (dat_18010 != 1) { // branch-flip
        if (((*(int *)(a2 + v24 * 0x60 + 0x38) != -1) && (v9 = *(char **)(a2 + v24 * 0x60), !stat(v9,&v12))) && ((*(long *)(a2 + v24 * 0x60 + 0x20) != CONCAT26(v12._6_2_,CONCAT24(v12._4_2_,v12._0_4_)) || (*(long *)(a2 + v24 * 0x60 + 0x28) != CONCAT26(v12._14_2_,CONCAT24(v12._12_2_,v12._8_4_)))))) {
          v8 = v24 * 0x60;
          v11 = sub_e8ca(4,(char *)sub_43ee((unsigned long *)(a2 + v8)));
          v9 = gettext("%s was replaced");
          error(0,*__errno_location(),v9,v11);
          return;
        }
      }
      else {
        sub_5a69((struct_2 *)(a2 + v24 * 0x60),0);
      }
      sub_70be((unsigned long *)(v24 * 0x60 + a2),&v13);
    }
  }
  v20 += 0x11;
  v21 = (void *)sub_f96c(v20);
label_7c7d:
  while( true ) {
    if (((dat_18010 == 1) && (dat_1812c != '\x01')) && (!sub_b071(v27))) {
      error(1,0,gettext("no files remaining"));
      return;
    }
    if (v22 < v23) break;
    do {
      v18 = -1;
      if (dat_18134) {
        if (v6)
          exit(0); // no-return
        if ((kill(dat_18134,0)) && (*__errno_location() != 1))
          v6 = 1;
        else {
          v6 = 0;
        }
        if ((v6) || (a0 <= 0.0))
          v18 = 0;
        else if (a0 < dat_13558) {
          v30 = dat_13560 * a0;
          v18 = (int)v30 + (unsigned int)((double)(int)v30 < v30);
        }
      }
      v12._4_2_ = 1;
      v12._8_4_ = 1;
      v12._14_2_ = 0;
      v12._12_2_ = 0;
      v12._0_4_ = a1;
      v7 = poll(&v12,dat_1812f + 1,v18);
    } while (!v7);
    if (v7 <= -1) {
      v9 = gettext("error waiting for inotify and output events");
      error(1,*__errno_location(),v9);
      return;
    }
    if (v12._14_2_)
      sub_431c(); // no-return
    v23 = sub_ed34(a1,v21,v20);
    v22 = 0;
    if (((v23) && ((v23 != 0xffffffffffffffff || (*__errno_location() != 0x16)))) || (v7 = v16 + -1, v15 = v16 == 0, v16 = v7, v15)) {
      if ((!v23) || (v23 == 0xffffffffffffffff)) {
        v9 = gettext("error reading inotify event");
        error(1,*__errno_location(),v9);
        return;
      }
      break;
    }
    v23 = 0;
    v20 <<= 1;
    v21 = (void *)sub_f9d6(v21,v20);
  }
  v31 = (int *)(v22 + (long)v21);
  v22 = v22 + (unsigned int)v31[3] + 0x10;
  v32 = v31;
  if ((v31[1] & 0x400U) && (!v31[3])) {
    for (v24 = 0; v24 < a3; v24 = v24 + 1) {
      if (*v31 == *(int *)(a2 + v24 * 0x60 + 0x48)) {
        error(0,0,gettext("directory containing watched file was removed"));
        return;
      }
    }
  }
  if (v31[3]) { // branch-flip
    v26 = 0;
    while ((v26 < a3 && ((*(int *)(a2 + v26 * 0x60 + 0x48) != *v32 || (v8 = *(long *)(a2 + v26 * 0x60), v2 = *(long *)(a2 + v26 * 0x60 + 0x50), v10 = &v32[4], strcmp((char *)v10,(char *)(v8 + v2))))))) {
      v26 += 1;
    }
    if (v26 == a3) goto label_7c7d;
    v25 = (unsigned long *)(a2 + v26 * 0x60);
    v19 = -1;
    v15 = (v32[1] & 0x200U) != 0;
    if (!v15) {
      v19 = inotify_add_watch(a1,*(unsigned long *)(a2 + v26 * 0x60),v17);
      if (v19 <= -1) {
        if ((*__errno_location() == 0x1c) || (*__errno_location() == 0xc)) {
          error(0,0,gettext("inotify resources exhausted"));
          return;
        }
        v11 = sub_e8ca(4,*(char **)(a2 + v26 * 0x60));
        v9 = gettext("cannot watch %s");
        error(0,*__errno_location(),v9,v11);
      }
    }
    v15 = !((v15) || ((0 <= *(int *)((long)v25 + 0x44) && (v19 == *(int *)((long)v25 + 0x44)))));
    if (v15) {
      if (0 <= *(int *)((long)v25 + 0x44)) {
        inotify_rm_watch(a1,*(unsigned int *)((long)v25 + 0x44));
        sub_c7af(v27,v25);
      }
      *(int *)((long)v25 + 0x44) = v19;
      if (v19 == -1) goto label_7c7d;
      v33 = (unsigned long *)sub_c7af(v27,v25);
      if ((v33) && (v33 != v25)) {
        if (dat_18010 == 1)
          sub_5a69(v33,0);
        *(unsigned int *)((long)v33 + 0x44) = 0xffffffff;
        v9 = (char *)sub_43ee(v33);
        sub_44d5(*(unsigned int *)&v33[7],v9);
      }
      if (!sub_c73f(v27,v25))
        sub_ffa4(); // no-return
    }
    if (dat_18010 == 1)
      sub_5a69(v25,0);
  }
  else {
    v34 = *v31;
    v25 = (unsigned long *)sub_b349(v27,v14);
  }
  if (v25) {
    if (v32[1] & 0xe04U) { // branch-flip
      if (v32[1] & 0x400U) {
        inotify_rm_watch(a1,*(unsigned int *)((long)v25 + 0x44));
        sub_c7af(v27,v25);
      }
      sub_5a69(v25,0);
    }
    else {
      sub_70be(v25,&v13);
    }
  }
  goto label_7c7d;
}

// Function: sub_8520 @ 0x8520
unsigned long sub_8520(char *a0,int a1,unsigned long a2,long *a3) // return-dupe
{
  unsigned int v1; // eax
  unsigned long v2; // rax
  char *v3; // rax
  long v4; // rax
  stat v5; // stack - 0xb8
  unsigned long v6; // stack - 0xf0
  long v7; // stack - 0xd0
  long v8; // stack - 0xc8
  
  if (fstat(a1,&v5)) {
    v2 = sub_e8ca(4,a0);
    v3 = gettext("cannot fstat %s");
    error(0,*__errno_location(),v3,v2);
    return 0;
  }
  if (dat_18130) { // branch-flip
    if (((dat_18139 != '\x01') && (0 <= (long)a2)) && ((((v5._24_4_ & 0xf000) == 0x8000 && (0 <= (long)sub_4804(a1,a2,1,a0))) || (lseek(a1,a2,1) != -1))))
      *a3 = *a3 + a2;
    else {
      v1 = sub_55ae(a0,a1,a2,a3);
      if (v1)
        return (unsigned long)(v1 >> 0x1f);
    }
    v6 = 0xffffffffffffffff;
  }
  else {
    v7 = -1;
    v8 = -1;
    if ((dat_18139 != '\x01') && (0 <= (long)a2)) {
      if (sub_2d62(&v5)) // branch-flip
        v7 = v5._48_8_;
      else {
        v8 = lseek(a1,-a2,2);
        if (v8 != -1)
          v7 = a2 + v8;
      }
    }
    if (((long)v5._56_8_ <= 0) || (0x2000000000000000 < v5._56_8_))
      v5._56_8_ = 0x200;
    if (v7 <= (long)v5._56_8_)
      return sub_5286(a0,a1,a2,a3);
    if (v8 == -1)
      v8 = sub_4804(a1,0,1,a0);
    if ((v8 < v7) && (a2 < (unsigned long)(v7 - v8))) {
      v8 = v7 - a2;
      sub_4804(a1,v8,0,a0);
    }
    *a3 = v8;
    v6 = a2;
  }
  v4 = sub_4649(0,a0,a1,v6);
  *a3 = v4 + *a3;
  return 1;
}

// Function: sub_88c7 @ 0x88c7
int * sub_88c7(char *a0,int a1,unsigned long a2,unsigned long *a3) // early-return x3
{
  unsigned long v1;
  char *v2;
  long v3;
  char v4; // al
  unsigned int v5; // eax
  int *v6; // rax
  stat v7; // stack - 0xb8
  long v8; // stack - 0xc8
  
  v5 = fstat(a1,&v7);
  if (v5) {
    v6 = (unsigned long)sub_e8ca(4,a0);
    v1 = v6;
    v6 = (char *)gettext("cannot fstat %s");
    v2 = (char *)v6;
    v6 = __errno_location();
    error(0,*v6,v2,v1);
    v6 = NULL;
    return v6;
  }
  if (dat_18130) { // branch-flip
    v5 = sub_5741(a0,a1,a2,a3);
    if (!v5) {
      v6 = (long)sub_4649(0,a0,a1,0xffffffffffffffff);
      *a3 = (long)v6 + *a3;
      v6 = (int *)0x1;
      return v6;
    }
    v5 >>= 0x1f;
    v6 = (unsigned long)(unsigned long)v5;
  }
  else {
    v8 = -1;
    if ((dat_18139 != '\x01') && (v5 = v7._24_4_ & 0xf000, v5 == 0x8000)) {
      v6 = (long)lseek(a1,0,1);
      v3 = (long)v6;
      v8 = (long)v6;
      if ((v6 != (int *)0xffffffffffffffff) && (v6 = (unsigned long)lseek(a1,0,2), v3 < (long)v6)) {
        *a3 = (unsigned long)v6;
        if (!v6) {
          v6 = (int *)0x1;
          return v6;
        }
        v4 = sub_49bd(a0,a1,a2,v3,v6,a3);
        if (v4 == '\x01') {
          v6 = (int *)0x1;
          return v6;
        }
        v6 = NULL;
        return v6;
      }
    }
    if (v8 != -1)
      sub_4804(a1,v8,0,a0);
    v6 = (unsigned long)sub_4df7(a0,a1,a2,a3);
  }
  return v6;
}

// Function: sub_8b67 @ 0x8b67
void sub_8b67(char *a0,unsigned int a1,unsigned long a2,unsigned long *a3) // return-dupe
{
  *a3 = 0;
  if (dat_1812d) {
    sub_88c7(a0,a1,a2,a3);
    return;
  }
  sub_8520(a0,a1,a2,a3);
}

// Function: sub_8bca @ 0x8bca
unsigned char sub_8bca(struct_8 *a0,unsigned long a1) // ternary, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_8bca
{
  char v1; // al
  int v10; // stack - 0xc4
  unsigned long v2; // rax
  char *v3;
  char *v4; // rax
  unsigned int v5; // ecx
  unsigned long v6; // stack - 0xc0
  stat v7; // stack - 0xb8
  bool v8; // zf
  unsigned char v9; // stack - 0xc6
  
  v3 = a0->field_0x0;
  v8 = strcmp(v3,"-") != 0;
  if (v8) {
    v3 = a0->field_0x0;
    v10 = sub_af09(v3,0);
  }
  else {
    dat_18138 = 1;
    v10 = 0;
    sub_ffef(0,0);
  }
  v1 = ((dat_1812c != '\x01') || (v10 != -1));
  a0->field_0x36 = v1;
  if (v10 != -1) { // branch-flip
    if (dat_18131)
      sub_4559((char *)sub_43ee(a0));
    v9 = sub_8b67((char *)sub_43ee(a0),v10,a1,&v6);
    if (dat_1812e) { // branch-flip
      a0->field_0x3c = (unsigned int)v9 - 1;
      if (0 <= fstat(v10,&v7)) { // branch-flip
        if (((((v7._24_4_ & 0xf000) != 0x8000) && ((v7._24_4_ & 0xf000) != 0x1000)) && ((v7._24_4_ & 0xf000) != 0xc000)) && ((v7._24_4_ & 0xf000) != 0x2000)) {
          v9 = 0;
          a0->field_0x3c = 0xffffffff;
          a0->field_0x36 = 0;
          a0->field_0x34 = dat_1812c == '\0';
          if (a0->field_0x34) // branch-flip
            v3 = gettext("; giving up on this name");
          else {
            v3 = "";
          }
          v4 = (char *)sub_ea5c(0,3,(char *)sub_43ee(a0));
          error(0,0,gettext("%s: cannot follow end of this type of file%s"),v4,v3);
        }
      }
      else {
        v9 = 0;
        a0->field_0x3c = *__errno_location();
        v2 = sub_e8ca(4,(char *)sub_43ee(a0));
        v3 = gettext("error reading %s");
        error(0,*__errno_location(),v3,v2);
      }
      if (v9 != 1) { // branch-flip
        a0->field_0x34 = dat_1812c == '\0';
        sub_44d5(v10,(char *)sub_43ee(a0));
        a0->field_0x38 = 0xffffffff;
      }
      else {
        v5 = (v8) ? 1 : 0xffffffff;
        sub_4435(a0,v10,v6,&v7,v5);
        v3 = (char *)sub_43ee(a0);
        a0->field_0x35 = sub_5938(v10,v3);
      }
    }
    else {
      if (!v8)
        return v9;
      if (!close(v10))
        return v9;
      v2 = sub_e8ca(4,(char *)sub_43ee(a0));
      v3 = gettext("error reading %s");
      error(0,*__errno_location(),v3,v2);
      v9 = 0;
    }
  }
  else {
    if (dat_1812e) {
      a0->field_0x38 = 0xffffffff;
      a0->field_0x3c = *__errno_location();
      a0->field_0x34 = dat_1812c == '\0';
      a0->field_0x28 = 0;
      a0->field_0x20 = 0;
    }
    v2 = sub_e8ca(4,(char *)sub_43ee(a0));
    v3 = gettext("cannot open %s for reading");
    error(0,*__errno_location(),v3,v2);
    v9 = 0;
  }
  return v9;
}

// Function: sub_90f5 @ 0x90f5
unsigned char * sub_90f5(int a0,long a1,long *a2) // return-dupe x5, warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_90f5
{
  unsigned char *v1;
  unsigned char *v10; // rax
  char v11; // stack - 0x3d
  char v12; // stack - 0x3c
  int v13; // stack - 0x38
  unsigned char *v14; // stack - 0x30
  char v2;
  unsigned char v3;
  bool v4;
  char *v5;
  char *v6;
  unsigned char *v7;
  bool v8; // al
  unsigned int v9; // eax
  
  v13 = 10;
  v12 = 1;
  if (((a0 != 2) && ((a0 != 3 || ((v10 = (unsigned long *)(a1 + 0x10), **(char **)v10 == '-' && (v10 = (long *)(a1 + 0x10), v10 = (char *)(*(long *)v10 + 1), *v10)))))) && ((a0 <= 2 || ((5 <= a0 || (v10 = (unsigned long *)(a1 + 0x10), v9 = strcmp(*(char **)v10,"--"), v9)))))) {
    v10 = NULL;
    return v10;
  }
  v9 = sub_cc1c();
  v4 = (int)v9 <= 0x30daf;
  v8 = ((v4) || (0x31069 <= (int)v9));
  v1 = (unsigned char *)&(*(char **)(a1 + 8))[1];
  v2 = **(char **)(a1 + 8);
  v14 = v1;
  if (v2 != '+') { // branch-flip
    if (v2 != '-') {
      v10 = NULL;
      return v10;
    }
    if ((!v4) && (v10 = &v1[*v1 == 99], !*v10)) {
      v10 = NULL;
      return v10;
    }
    v11 = 0;
  }
  else {
    if (!v8) {
      v10 = NULL;
      return v10;
    }
    v11 = 1;
  }
  for (; v7 = v14, v9 = (int)(char)*v14, v9 = v9 - 0x30, v9 <= 9; v14 = &v14[1]) {
  }
  v3 = *v14;
  if (v3 != 0x6c) {
    if ('m' <= (char)v3) goto label_929b;
    if (v3 != 0x62) { // branch-flip
      if (v3 != 99) goto label_929b;
    }
    else {
      v13 = 0x1400;
    }
    v12 = 0;
  }
  v14 = &v14[1];
label_929b:
  v3 = *v14;
  if (v3 == 0x66)
    v14 = &v14[1];
  if (*v14) {
    v10 = NULL;
    return v10;
  }
  if (v1 != v7) { // branch-flip
    v9 = sub_10410(v1,NULL,10,a2,"b");
    v9 &= 0xfffffffd;
    if (v9) {
      v10 = (unsigned long *)(a1 + 8);
      v10 = (char *)sub_ed11(*(char **)v10);
      v5 = (char *)v10;
      v10 = (char *)gettext("invalid number");
      v6 = (char *)v10;
      v10 = (int *)__errno_location();
      error(1,*(int *)v10,"%s: %s",v6,v5);
      return v10;
    }
  }
  else {
    *a2 = (long)v13;
  }
  v10 = (unsigned char *)0x1;
  dat_1812d = v12;
  dat_1812e = v3 == 0x66;
  dat_18130 = v11;
  return v10;
}

// Function: sub_9381 @ 0x9381
void sub_9381(int a0,char **a1,unsigned long *a2,unsigned int *a3,double *a4)
{
  char v1; // al
  int v2; // eax
  char *v3; // rax
  unsigned long v4; // rax
  double v5; // stack - 0x28
  
  while( true ) {
    v2 = getopt_long(a0,a1,"c:n:fFqs:vz0123456789",(option *)0x17920,NULL);
    if (v2 == -1) {
      if (dat_1812c) {
        if (dat_1812e != '\x01') { // branch-flip
          dat_1812c = '\0';
          error(0,0,gettext("warning: --retry ignored; --retry is useful only when following"));
        }
        else if (dat_18010 == 2)
          error(0,0,gettext("warning: --retry only effective for the initial open"));
      }
      if ((dat_18134) && (dat_1812e != '\x01'))
        error(0,0,gettext("warning: PID ignored; --pid=PID is useful only when following"));
      else if ((dat_18134) && ((kill(dat_18134,0) && (*__errno_location() == 0x26)))) {
        error(0,0,gettext("warning: --pid=PID is not supported on this system"));
        dat_18134 = 0;
      }
      return;
    }
    if (0x85 < v2) break;
    if (v2 <= 0x2f) {
      if (v2 == -0x83) {
        sub_f78c(stdout,"tail","GNU coreutils",dat_18030,"Paul Rubin","David MacKenzie","Ian Lance Taylor","Jim Meyering",0);
        exit(0); // no-return
      }
      if (v2 == -0x82)
        sub_4115(0); // no-return
      break;
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
        error(1,0,gettext("option used in invalid context -- %c"),(char)v2);
        return;
      default:
        goto label_975f;
      case 0x46:
        dat_1812e = '\x01';
        dat_18010 = 1;
        dat_1812c = '\x01';
        break;
      case 99:
      case 0x6e:
        dat_1812d = v2 == 0x6e;
        if (*optarg != '+') { // branch-flip
          if (*optarg == '-')
            optarg = &optarg[1];
        }
        else {
          dat_18130 = 1;
        }
        if ((bool)dat_1812d)
          v3 = gettext("invalid number of lines");
        else {
          v3 = gettext("invalid number of bytes");
        }
        *a2 = sub_10156(optarg,0,0xffffffffffffffff,"bkKmMGTPEZY0",v3,0);
        break;
      case 0x66:
      case 0x84:
        dat_1812e = '\x01';
        if (optarg) // branch-flip
          dat_18010 = *(int *)(sub_a546("--follow",optarg,0x17900,0x12210,4,dat_18038,1) * 4 + 0x12210);
        else {
          dat_18010 = 2;
        }
        break;
      case 0x71:
        *a3 = 2;
        break;
      case 0x73:
        v1 = sub_10261(optarg,0,&v5,sub_a745);
        if ((v1 != '\x01') || (v5 < 0.0)) {
          v4 = sub_ed11(optarg);
          error(1,0,gettext("invalid number of seconds: %s"),v4);
        }
        *a4 = v5;
        break;
      case 0x76:
        *a3 = 1;
        break;
      case 0x7a:
        dat_18132 = 0;
        break;
      case 0x80:
        dat_1812c = '\x01';
        break;
      case 0x81:
        dat_18018 = sub_10156(optarg,0,0xffffffffffffffff,0x121c3,gettext("invalid maximum number of unchanged stats between opens"),0);
        break;
      case 0x82:
        dat_18134 = sub_10156(optarg,0,0x7fffffff,0x121c3,gettext("invalid PID"),0);
        break;
      case 0x83:
        dat_18139 = 1;
        break;
      case 0x85:
        dat_1813a = 1;
      
    }
  }
label_975f:
  sub_4115(1); // no-return
}

// Function: sub_98cb @ 0x98cb
long sub_98cb(long a0,unsigned long a1) // warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in sub_98cb
{
  char *v1;
  bool v2; // al
  long v3; // stack - 0x18
  unsigned long v4; // stack - 0x10
  
  v3 = 0;
  for (v4 = 0; v4 < a1; v4 = v4 + 1) {
    v1 = *(char **)(a0 + v4 * 0x60);
    v2 = !(((strcmp(v1,"-")) || (*(char *)(a0 + v4 * 0x60 + 0x34) == '\x01')) || ((*(int *)(a0 + v4 * 0x60 + 0x38) <= -1 || ((*(unsigned int *)(a0 + v4 * 0x60 + 0x30) & 0xf000) != 0x1000))));
    if (v2) {
      *(unsigned int *)(a0 + v4 * 0x60 + 0x38) = 0xffffffff;
      *(char *)(a0 + v4 * 0x60 + 0x34) = 1;
    }
    else {
      v3 += 1;
    }
  }
  return v3;
}

// Function: main @ 0x9a14
int main(int argc,char **argv,char **envp) // warn: iteboolean: re-rolled 1 0/1 select diamond(s) to a boolean assignment in main
{
  unsigned char v1; // al
  long v10; // stack - 0xf0
  unsigned long v11; // stack - 0xe8
  int v12; // stack - 0xf8
  stat v13; // stack - 0xb8
  unsigned long v14; // stack - 0xe0
  bool v15; // stack - 0xfc
  int v16; // stack - 0xf4
  unsigned long v17; // stack - 0xd8
  char **v18; // stack - 0xd0
  unsigned long v19; // stack - 0xc8
  bool v2;
  long v20; // stack - 0xc0
  char v3; // al
  int v4; // eax
  unsigned int v5; // eax
  int v6; // eax
  int v7; // eax
  int v8; // eax
  char *v9;
  
  v12 = 0;
  v15 = 1;
  v10 = 10;
  v11 = dat_13568;
  sub_ccd0(*argv);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  sub_11700(sub_a84b);
  dat_18138 = '\0';
  dat_1812d = 1;
  dat_18131 = 0;
  dat_18130 = '\0';
  dat_1812e = '\0';
  dat_18132 = 10;
  v1 = sub_90f5(argc,argv,&v10);
  argc -= (unsigned int)v1;
  sub_9381(argc,&argv[v1],&v10,&v12,&v11);
  if ((dat_18130) && (v10))
    v10 -= 1;
  if (argc <= optind) { // branch-flip
    v17 = 1;
    v18 = (char **)0x18028;
  }
  else {
    v17 = (unsigned long)(argc - optind);
    v18 = &(&argv[v1])[optind];
  }
  v2 = 0;
  for (v19 = 0; v19 < v17; v19 = v19 + 1) {
    v9 = v18[v19];
    if (!strcmp(v9,"-"))
      v2 = 1;
  }
  if ((v2) && (dat_18010 == 1)) {
    v9 = (char *)sub_e8ca(4,"-");
    error(1,0,gettext("cannot follow %s by name"),v9);
    return v4;
  }
  if ((dat_1812e) && (v2)) {
    v2 = !((dat_18134) || ((((dat_18010 != 2 || (v17 != 1)) || (fstat(0,&v13))) || ((v13._24_4_ & 0xf000) == 0x8000))));
    if ((!v2) && (isatty(0)))
      error(0,0,gettext("warning: following standard input indefinitely is ineffective"));
  }
  if (((v10) || (dat_1812e == '\x01')) || (dat_18130 == '\x01')) {
    v20 = sub_fae5(v17,0x60);
    for (v19 = 0; v19 < v17; v19 = v19 + 1) {
      *(char **)(v19 * 0x60 + v20) = v18[v19];
    }
    if ((v12 == 1) || ((!v12 && (2 <= v17))))
      dat_18131 = 1;
    sub_ffef(1,0);
    for (v19 = 0; v19 < v17; v19 = v19 + 1) {
      v1 = sub_8bca((struct_8 *)(v20 + v19 * 0x60),v10);
      v15 = (v1 & v15) != 0;
    }
    if ((dat_1812e) && (sub_98cb(v20,v17))) {
      if (fstat(1,&v13) <= -1) {
        v9 = gettext("standard output");
        error(1,*__errno_location(),v9);
        return v6;
      }
      dat_1812f = (v13._24_4_ & 0xf000) == 0x1000;
      if ((dat_1813a != '\x01') && (((((v3 = sub_6fc5(v20,v17), v3 || (v3 = sub_6d57(v20,v17), v3)) || (v3 = sub_6dd3(v20,v17), v3 != '\x01')) || ((v3 = sub_6e52(v20,v17), v3 || (v3 = sub_6f11(v20,v17), v3)))) || ((v15 != 1 && (dat_18010 == 2))))))
        dat_1813a = '\x01';
      if (dat_1813a != '\x01') {
        v16 = inotify_init();
        if (0 <= v16) {
          if (fflush_unlocked(stdout)) {
            v9 = gettext("write error");
            error(1,*__errno_location(),v9);
            return v7;
          }
          sub_7379(v16,v20,v17,&v14);
          sub_bc7e(v14);
          close(v16);
          *__errno_location() = 0;
        }
        v9 = gettext("inotify cannot be used, reverting to polling");
        error(0,*__errno_location(),v9);
      }
      dat_1813a = '\x01';
      sub_62f3(v20,v17,v17,v11);
    }
    if ((dat_18138) && (close(0) <= -1)) {
      error(1,*__errno_location(),"-");
      return v8;
    }
    v5 = (unsigned int)(v15 ^ 1);
  }
  else {
    v5 = 0;
  }
  return v5;
}

// Function: sub_a198 @ 0xa198
void sub_a198(void)
{
  sub_4115(1); // no-return
}

// Function: sub_a1ad @ 0xa1ad
long sub_a1ad(char *a0,long a1,long a2,unsigned long a3) // early-return
{
  char *v1;
  bool v2;
  unsigned long v3; // rax
  long v4; // stack - 0x20
  long v5; // stack - 0x18
  
  v5 = -1;
  v2 = 0;
  v3 = strlen(a0);
  v4 = 0;
  do {
    if (!*(long *)(a1 + v4 * 8)) {
      if (v2)
        return -2;
      return v5;
    }
    v1 = *(char **)(a1 + v4 * 8);
    if (!strncmp(v1,a0,v3)) {
      v1 = *(char **)(a1 + v4 * 8);
      if (v3 == strlen(v1))
        return v4;
      if (v5 != -1) { // branch-flip
        if ((!a2) || (memcmp((void *)(v5 * a3 + a2),(void *)(a3 * v4 + a2),a3)))
          v2 = 1;
      }
      else {
        v5 = v4;
      }
    }
    v4 += 1;
  } while( true );
}

// Function: sub_a2e0 @ 0xa2e0
long sub_a2e0(char *a0,long a1)
{
  char *v1;
  long v2; // stack - 0x10
  
  v2 = 0;
  while( true ) {
    if (!*(long *)(a1 + v2 * 8))
      return -1;
    v1 = *(char **)(a1 + v2 * 8);
    if (!strcmp(v1,a0)) break;
    v2 += 1;
  }
  return v2;
}

// Function: sub_a356 @ 0xa356
void sub_a356(char *a0,char *a1,long a2)
{
  char *v1; // rax
  unsigned long v2; // rax
  
  if (a2 != -1) // branch-flip
    v1 = gettext("ambiguous argument %s for %s");
  else {
    v1 = gettext("invalid argument %s for %s");
  }
  v2 = sub_ece4(1,a0);
  error(0,0,v1,sub_e7fc(0,8,a1),v2);
}

// Function: sub_a3ef @ 0xa3ef
void sub_a3ef(long a0,long a1,unsigned long a2)
{
  char *v1;
  FILE *v2;
  long v3; // stack - 0x28
  void *v4; // stack - 0x20
  
  v2 = stderr;
  v4 = NULL;
  fputs_unlocked(gettext("Valid arguments are:"),v2);
  v3 = 0;
  do {
    if (!*(long *)(a0 + v3 * 8)) {
      putc_unlocked(10,stderr);
      return;
    }
    if (v3) { // branch-flip
      if (memcmp(v4,(void *)(a2 * v3 + a1),a2)) goto label_a470;
      v1 = *(char **)(a0 + v3 * 8);
      fprintf(stderr,", %s",(char *)sub_ed11(v1));
    }
    else {
label_a470:
      v1 = *(char **)(a0 + v3 * 8);
      fprintf(stderr,"\n  - %s",(char *)sub_ed11(v1));
      v4 = (void *)(a1 + a2 * v3);
    }
    v3 += 1;
  } while( true );
}

// Function: sub_a546 @ 0xa546
long sub_a546(char *a0,char *a1,long a2,long a3,unsigned long a4,void *a5,unsigned int a6) // early-return, ternary
{
  long v1; // stack - 0x10
  
  v1 = ((char)a6) ? sub_a1ad(a1,a2,a3,a4) : sub_a2e0(a1,a2); // branch-flip
  if (0 <= v1)
    return v1;
  sub_a356(a0,a1,v1);
  sub_a3ef(a2,a3,a4);
  (*a5)();
  return -1;
}

// Function: sub_a5f5 @ 0xa5f5
unsigned long sub_a5f5(void *a0,long a1,long a2,unsigned long a3)
{
  long v1; // stack - 0x10
  
  v1 = 0;
  while( true ) {
    if (!*(long *)(a1 + v1 * 8))
      return 0;
    if (!memcmp(a0,(void *)(a3 * v1 + a2),a3)) break;
    v1 += 1;
  }
  return *(unsigned long *)(a1 + v1 * 8);
}

// Function: sub_a685 @ 0xa685
char * sub_a685(char *a0)
{
  bool v1;
  char *v2; // stack - 0x18
  char *v3; // stack - 0x10
  
  v1 = 0;
  for (v2 = a0; *v2 == '/'; v2 = &v2[1]) {
  }
  for (v3 = v2; *v3; v3 = &v3[1]) {
    if (*v3 != '/') { // branch-flip
      if (!v1) goto label_a6dc;
      v2 = v3;
      v1 = 0;
    }
    else {
      v1 = 1;
    }
label_a6dc:
  }
  return v2;
}

// Function: sub_a6f2 @ 0xa6f2
unsigned long sub_a6f2(char *a0)
{
  unsigned long v1; // stack - 0x18
  
  for (v1 = strlen(a0); (2 <= v1 && (a0[v1 - 1] == '/')); v1 = v1 - 1) {
  }
  return v1;
}

// Function: sub_a745 @ 0xa745
unsigned long sub_a745(unsigned long a0,unsigned long *a1)
{
  int v1;
  char *v2; // stack - 0x30
  char *v3; // stack - 0x28
  unsigned long v4; // xmm0_qa
  unsigned long v5; // stack - 0x20
  
  strtod(a0,&v2);
  v5 = v4;
  if (*v2) {
    v1 = *__errno_location();
    sub_10b1d(a0,&v3);
    if (v3 <= v2) // branch-flip
      *__errno_location() = v1;
    else {
      v5 = v4;
      v2 = v3;
    }
  }
  if (a1)
    *a1 = v2;
  v4 = v5;
  return v4;
}

// Function: sub_a817 @ 0xa817
void sub_a817(unsigned long a0)
{
  dat_18140 = a0;
}

// Function: sub_a831 @ 0xa831
void sub_a831(char a0)
{
  dat_18148 = a0;
}

// Function: sub_a84b @ 0xa84b
void sub_a84b(void)
{
  int v1; // eax
  char *v2; // rax
  char *v3; // rax
  
  v1 = sub_10b8a(stdout);
  if (v1) {
    if (dat_18148 == '\x01') {
      if (*__errno_location() == 0x20) goto label_a91c;
    }
    v2 = gettext("write error");
    if (dat_18140) { // branch-flip
      v3 = (char *)sub_ea0b(dat_18140);
      error(0,*__errno_location(),"%s: %s",v3,v2);
    }
    else {
      error(0,*__errno_location(),"%s",v2);
    }
    _exit(dat_18040); // no-return
  }
label_a91c:
  v1 = sub_10b8a(stderr);
  if (v1)
    _exit(dat_18040); // no-return
}

// Function: sub_a943 @ 0xa943
unsigned long sub_a943(char *a0)
{
  char v1;
  unsigned long v2; // stack - 0x18
  
  v1 = *a0;
  for (v2 = sub_a685(a0) - (long)a0; ((v1 == '/') < v2 && (a0[v2 - 1] == '/')); v2 = v2 - 1) {
  }
  return v2;
}

// Function: sub_a9c8 @ 0xa9c8
char * sub_a9c8(char *a0) // early-return
{
  char *v1; // rax
  unsigned long v2; // stack - 0x18
  
  v2 = sub_a943(a0);
  v1 = malloc(v2 + (v2 == 0) + 1);
  if (!v1)
    return NULL;
  memcpy(v1,a0,v2);
  if (!v2) {
    v2 = 1;
    *v1 = 0x2e;
  }
  v1[v2] = 0;
  return v1;
}

// Function: sub_aa63 @ 0xaa63
int sub_aa63(int a0,int a1)
{
  unsigned long v1;
  unsigned long v2; // stack - 0xa8
  
  if (!a1)
    return sub_add5(a0,(unsigned int)v2);
  if (a1 == 0x406)
    return sub_ae06(a0,(unsigned int)v2);
  if (0xc <= a1) { // branch-flip
    if ((0x40b <= a1) || (a1 <= 0x3ff)) {
      return fcntl(a0,a1,v2); // return-dupe
    }
    v1 = 1L << ((unsigned char)a1 & 0x3f);
    if (v1 & 0x2c5) {
      return fcntl(a0,a1,v2 & 0xffffffff); // return-dupe
    }
    v1 &= 0x502;
  }
  else {
    if (a1 <= -1) {
      return fcntl(a0,a1,v2);
    }
    v1 = 1L << ((unsigned char)a1 & 0x3f);
    if (v1 & 0x515)
      return fcntl(a0,a1,v2 & 0xffffffff);
    v1 &= 0xa0a;
  }
  if (v1)
    return fcntl(a0,a1);
  return fcntl(a0,a1,v2);
}

// Function: sub_add5 @ 0xadd5
int sub_add5(int a0,unsigned int a1)
{
  return fcntl(a0,0,(unsigned long)a1);
}

// Function: sub_ae06 @ 0xae06
int sub_ae06(int a0,unsigned int a1)
{
  int v1;
  unsigned int v2; // eax
  int v3; // stack - 0x14
  
  if (0 <= dat_1814c) { // branch-flip
    v3 = fcntl(a0,0x406,(unsigned long)a1);
    if ((0 <= v3) || (*__errno_location() != 0x16))
      dat_1814c = 1;
    else {
      v3 = sub_add5(a0,a1);
      if (0 <= v3)
        dat_1814c = -1;
    }
  }
  else {
    v3 = sub_add5(a0,a1);
  }
  if ((0 <= v3) && (dat_1814c == -1)) {
    v2 = fcntl(v3,1);
    if ((0 <= (int)v2) && (fcntl(v3,2,(unsigned long)(v2 | 1)) != -1))
      return v3;
    v1 = *__errno_location();
    close(v3);
    *__errno_location() = v1;
    v3 = -1;
  }
  return v3;
}

// Function: sub_af09 @ 0xaf09
void sub_af09(char *a0,unsigned int a1)
{
  unsigned int v1; // stack - 0xdc
  unsigned int v2; // stack - 0xa8
  
  v1 = 0;
  if (a1 & 0x40)
    v1 = v2;
  sub_efc6(open(a0,a1,(unsigned long)v1));
}

// Function: sub_b045 @ 0xb045
unsigned long sub_b045(long a0)
{
  return *(unsigned long *)(a0 + 0x10);
}

// Function: sub_b05b @ 0xb05b
unsigned long sub_b05b(long a0)
{
  return *(unsigned long *)(a0 + 0x18);
}

// Function: sub_b071 @ 0xb071
unsigned long sub_b071(long a0)
{
  return *(unsigned long *)(a0 + 0x20);
}

// Function: sub_b087 @ 0xb087
unsigned long sub_b087(struct_11 *a0)
{
  long *v1; // stack - 0x28
  unsigned long v2; // stack - 0x20
  long *v3; // stack - 0x18
  unsigned long v4; // stack - 0x10
  
  v2 = 0;
  for (v1 = a0->field_0x0; v1 < a0->field_0x8; v1 = &v1[2]) {
    if (*v1) {
      v3 = v1;
      v4 = 1;
      while (v3 = (long *)v3[1], v3) {
        v4 += 1;
      }
      if (v2 < v4)
        v2 = v4;
    }
  }
  return v2;
}

// Function: sub_b109 @ 0xb109
unsigned long sub_b109(struct_11 *a0) // return-dupe
{
  long *v1; // stack - 0x28
  long v2; // stack - 0x20
  long v3; // stack - 0x18
  long *v4; // stack - 0x10
  
  v2 = 0;
  v3 = 0;
  for (v1 = a0->field_0x0; v1 < a0->field_0x8; v1 = &v1[2]) {
    if (*v1) {
      v4 = v1;
      v2 += 1;
      do {
        v3 += 1;
        v4 = (long *)v4[1];
      } while (v4);
    }
  }
  if ((v2 == a0->field_0x18) && (v3 == a0->field_0x20))
    return 1;
  return 0;
}

// Function: sub_b1a7 @ 0xb1a7
void sub_b1a7(unsigned long a0,FILE *a1)
{
  unsigned long v1; // rax
  unsigned long v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // rax
  
  v1 = sub_b071(a0);
  v2 = sub_b045(a0);
  v3 = sub_b05b(a0);
  v4 = sub_b087(a0);
  fprintf(a1,"# entries:         %lu\n",v1);
  fprintf(a1,"# buckets:         %lu\n",v2);
  fprintf(a1,"# buckets used:    %lu (%.2f%%)\n",v3,(dat_13678 * (double)v3) / (double)v2);
  fprintf(a1,"max bucket length: %lu\n",v4);
}

// Function: sub_b2ee @ 0xb2ee
long sub_b2ee(struct_4 *a0,unsigned long a1)
{
  unsigned long v1; // rax
  
  v1 = (*a0->field_0x30)(a1,a0->field_0x10);
  if ((unsigned long)a0->field_0x10 <= v1)
    abort(); // no-return
  return v1 * 0x10 + a0->field_0x0;
}

// Function: sub_b349 @ 0xb349
long sub_b349(long a0,long a1) // return-dupe
{
  void *v1;
  long v2;
  long *v3; // stack - 0x18
  
  v3 = (long *)sub_b2ee(a0,a1);
  if (!*v3)
    return 0;
  while( true ) {
    if (!v3)
      return 0;
    if ((a1 == *v3) || (v1 = *(void **)(a0 + 0x38), v2 = *v3, (*v1)(a1,v2))) break;
    v3 = (long *)v3[1];
  }
  return *v3;
}

// Function: sub_b3e0 @ 0xb3e0
long sub_b3e0(struct_11 *a0) // early-return
{
  long *v1; // stack - 0x10
  
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

// Function: sub_b440 @ 0xb440
long sub_b440(long a0,long a1) // early-return
{
  long *v1; // stack - 0x18
  long *v2; // stack - 0x10
  
  v1 = (long *)sub_b2ee(a0,a1);
  v2 = v1;
  while ((a1 != *v2 || (!v2[1]))) {
    v2 = (long *)v2[1];
    if (!v2) {
      do {
        v1 = &v1[2];
        if (*(long **)(a0 + 8) <= v1)
          return 0;
      } while (!*v1);
      return *v1;
    }
  }
  return *(long *)v2[1];
}

// Function: sub_b4de @ 0xb4de
unsigned long sub_b4de(struct_11 *a0,long a1,unsigned long a2)
{
  unsigned long v1;
  unsigned long v2; // stack - 0x20
  long *v3; // stack - 0x18
  long *v4; // stack - 0x10
  
  v2 = 0;
  v3 = a0->field_0x0;
  do {
    if (a0->field_0x8 <= v3)
      return v2;
    if (*v3) {
      v4 = v3;
      while (v4) {
        if (a2 <= v2)
          return v2;
        v1 = v2 + 1;
        *(long *)(v2 * 8 + a1) = *v4;
        v4 = (long *)v4[1];
        v2 = v1;
      }
    }
    v3 = &v3[2];
  } while( true );
}

// Function: sub_b57e @ 0xb57e
long sub_b57e(struct_11 *a0,void *a1,unsigned long a2)
{
  long v1;
  long v2; // stack - 0x20
  long *v3; // stack - 0x18
  long *v4; // stack - 0x10
  
  v2 = 0;
  v3 = a0->field_0x0;
  do {
    if (a0->field_0x8 <= v3)
      return v2;
    if (*v3) {
      for (v4 = v3; v4; v4 = (long *)v4[1]) {
        v1 = *v4;
        if ((*a1)(v1,a2) != '\x01')
          return v2;
        v2 += 1;
      }
    }
    v3 = &v3[2];
  } while( true );
}

// Function: sub_b616 @ 0xb616
unsigned long sub_b616(unsigned char *a0,unsigned long a1)
{
  unsigned char *v1; // stack - 0x20
  unsigned long v2; // stack - 0x10
  
  v2 = 0;
  for (v1 = a0; *v1; v1 = &v1[1]) {
    v2 = ((unsigned long)*v1 + v2 * 0x1f) % a1;
  }
  return v2;
}

// Function: sub_b670 @ 0xb670
unsigned long sub_b670(unsigned long a0)
{
  unsigned long v1; // stack - 0x18
  unsigned long v2; // stack - 0x10
  
  v2 = 9;
  for (v1 = 3; (v2 < a0 && (a0 % v1)); v1 = v1 + 2) {
    v2 += (v1 + 1) * 4;
  }
  return CONCAT71((undefined7)(a0 % v1 >> 8),a0 % v1 != 0);
}

// Function: sub_b6df @ 0xb6df
unsigned long sub_b6df(unsigned long a0)
{
  unsigned long v1; // stack - 0x10
  
  v1 = a0;
  if (a0 <= 9)
    v1 = 10;
  v1 |= 1;
  while( true ) {
    if (v1 == 0xffffffffffffffff)
      return 0xffffffffffffffff;
    if (sub_b670(v1) == '\x01') break;
    v1 += 2;
  }
  return v1;
}

// Function: sub_b72a @ 0xb72a
void sub_b72a(struct_12 *a0)
{
  a0->field_0x0 = 0;
  a0->field_0x4 = dat_13680;
  a0->field_0x8 = dat_13684;
  a0->field_0xc = dat_13688;
  a0->field_0x10 = 0;
}

// Function: sub_b780 @ 0xb780
unsigned long sub_b780(unsigned long a0,unsigned long a1)
{
  return (unsigned long)sub_10957(a0,3) % a1;
}

// Function: sub_b7bb @ 0xb7bb
unsigned long sub_b7bb(long a0,long a1)
{
  return CONCAT71((undefined7)((unsigned long)a0 >> 8),a0 == a1);
}

// Function: sub_b7d8 @ 0xb7d8
unsigned long sub_b7d8(long a0) // return-dupe x2
{
  float *v1;
  
  v1 = *(float **)(a0 + 0x28);
  if (v1 == (float *)0x135f0)
    return 1;
  if ((((dat_1368c < v1[2]) && (v1[2] < dat_13680 - dat_1368c)) && (dat_1368c + dat_13680 < v1[3])) && (((0.0 <= *v1 && (*v1 + dat_1368c < v1[1])) && ((v1[1] <= dat_13680 && (*v1 + dat_1368c < v1[2]))))))
    return 1;
  *(unsigned long *)(a0 + 0x28) = 0x135f0;
  return 0;
}

// Function: sub_b8e5 @ 0xb8e5
unsigned long sub_b8e5(unsigned long a0,long a1) // early-return x2
{
  unsigned long v1; // rax
  float v2; // xmm0_da
  unsigned long v3; // stack - 0x20
  
  v3 = a0;
  if (*(char *)(a1 + 0x10) != '\x01') {
    v2 = (float)a0 / *(float *)(a1 + 8);
    if (dat_13690 <= v2)
      return 0;
    if (dat_13694 <= v2)
      v3 = (long)(v2 - dat_13694) ^ 0x8000000000000000;
    else {
      v3 = (unsigned long)v2;
    }
  }
  v1 = sub_b6df(v3);
  if (!((long)(v1 << 3) <= -1 || v1 >> 0x3d))
    return v1;
  return 0;
}

// Function: sub_b9fe @ 0xb9fe
long * sub_b9fe(unsigned long a0,long a1,void *a2,void *a3,long a4) // early-return x2
{
  unsigned long v1;
  long *v2; // rax
  void *v3; // stack - 0x38
  void *v4; // stack - 0x30
  long v5; // stack - 0x28
  
  v4 = a2;
  if (!a2)
    v4 = sub_b780;
  v3 = a3;
  if (!a3)
    v3 = sub_b7bb;
  v2 = malloc(0x50);
  if (!v2)
    return NULL;
  v5 = a1;
  if (!a1)
    v5 = 0x135f0;
  v2[5] = v5;
  if (sub_b7d8(v2) == '\x01') {
    v2[2] = sub_b8e5(a0,v5);
    if (v2[2]) {
      v1 = v2[2];
      *v2 = (long)calloc(v1,0x10);
      if (*v2) {
        v2[1] = *v2 + v2[2] * 0x10;
        v2[3] = 0;
        v2[4] = 0;
        v2[6] = (long)v4;
        v2[7] = (long)v3;
        v2[8] = a4;
        v2[9] = 0;
        return v2;
      }
    }
  }
  free(v2);
  return NULL;
}

// Function: sub_bb73 @ 0xbb73
void sub_bb73(struct_13 *a0)
{
  unsigned long *v1;
  long *v2; // stack - 0x20
  unsigned long *v3; // stack - 0x18
  
  for (v2 = a0->field_0x0; v2 < a0->field_0x8; v2 = &v2[2]) {
    if (*v2) {
      v3 = (unsigned long *)v2[1];
      while (v3) {
        if (a0->field_0x40)
          (*(void *)a0->field_0x40)(*v3);
        *v3 = 0;
        v1 = (unsigned long *)v3[1];
        v3[1] = a0->field_0x48;
        a0->field_0x48 = v3;
        v3 = v1;
      }
      if (a0->field_0x40)
        (*(void *)a0->field_0x40)(*v2);
      *v2 = 0;
      v2[1] = 0;
    }
  }
  a0->field_0x18 = 0;
  a0->field_0x20 = 0;
}

// Function: sub_bc7e @ 0xbc7e
void sub_bc7e(struct_10 *a0)
{
  void *v1;
  long *v2; // stack - 0x20
  long *v3; // stack - 0x18
  
  if ((a0->field_0x40) && (a0->field_0x20)) {
    for (v2 = a0->field_0x0; v2 < a0->field_0x8; v2 = &v2[2]) {
      if (*v2) {
        for (v3 = v2; v3; v3 = (long *)v3[1]) {
          (*(void *)a0->field_0x40)(*v3);
        }
      }
    }
  }
  for (v2 = a0->field_0x0; v2 < a0->field_0x8; v2 = &v2[2]) {
    v3 = (void *)v2[1];
    while (v3) {
      v1 = *(void **)((long)v3 + 8);
      free(v3);
      v3 = v1;
    }
  }
  v3 = a0->field_0x48;
  while (v3) {
    v1 = *(void **)((long)v3 + 8);
    free(v3);
    v3 = v1;
  }
  free(a0->field_0x0);
  free(a0);
}

// Function: sub_bdad @ 0xbdad
void * sub_bdad(long a0)
{
  void *v1; // stack - 0x10
  
  if (*(long *)(a0 + 0x48)) { // branch-flip
    v1 = *(void **)(a0 + 0x48);
    *(unsigned long *)(a0 + 0x48) = *(unsigned long *)((long)v1 + 8);
  }
  else {
    v1 = malloc(0x10);
  }
  return v1;
}

// Function: sub_bdfc @ 0xbdfc
void sub_bdfc(long a0,struct_5 *a1)
{
  a1->field_0x0 = 0;
  a1->field_0x8 = *(unsigned long *)(a0 + 0x48);
  *(struct_5 **)(a0 + 0x48) = a1;
}

// Function: sub_be36 @ 0xbe36
long sub_be36(long a0,long a1,unsigned long *a2,bool a3) // early-return
{
  void *v1;
  long v2;
  struct_5 *v3;
  long v4;
  unsigned long v5;
  long *v6; // stack - 0x38
  
  v6 = (long *)sub_b2ee(a0,a1);
  *a2 = v6;
  if (!*v6)
    return 0;
  if ((a1 != *v6) && (v1 = *(void **)(a0 + 0x38), v2 = *v6, !(*v1)(a1,v2))) {
    while( true ) {
      if (!v6[1])
        return 0;
      if ((a1 == *(long *)v6[1]) || (v1 = *(void **)(a0 + 0x38), v5 = *(unsigned long *)v6[1], (*v1)(a1,v5))) break;
      v6 = (long *)v6[1];
    }
    v2 = *(long *)v6[1];
    if (a3) {
      v3 = (struct_5 *)v6[1];
      v6[1] = v3->field_0x8;
      sub_bdfc(a0,v3);
    }
    return v2;
  }
  v2 = *v6;
  if (a3) {
    if (v6[1]) { // branch-flip
      v3 = (struct_5 *)v6[1];
      v4 = v3->field_0x8;
      *v6 = v3->field_0x0;
      v6[1] = v4;
      sub_bdfc(a0,v3);
    }
    else {
      *v6 = 0;
    }
  }
  return v2;
}

// Function: sub_bfcf @ 0xbfcf
unsigned long sub_bfcf(long a0,struct_11 *a1,bool a2)
{
  long *v1;
  long v2;
  struct_3 *v3; // rax
  struct_3 *v4; // rax
  long *v5; // stack - 0x38
  long *v6; // stack - 0x30
  
  v5 = a1->field_0x0;
  do {
    if (a1->field_0x8 <= v5)
      return 1;
    if (*v5) {
      v6 = (long *)v5[1];
      while (v6) {
        v2 = *v6;
        v3 = (struct_3 *)sub_b2ee(a0,v2);
        v1 = (long *)v6[1];
        if (v3->field_0x0) { // branch-flip
          v6[1] = v3->field_0x8;
          v3->field_0x8 = (long)v6;
        }
        else {
          v3->field_0x0 = v2;
          *(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + 1;
          sub_bdfc(a0,v6);
        }
        v6 = v1;
      }
      v2 = *v5;
      v5[1] = 0;
      if (!a2) {
        v3 = (struct_3 *)sub_b2ee(a0,v2);
        if (v3->field_0x0) { // branch-flip
          v4 = (struct_3 *)sub_bdad(a0);
          if (!v4)
            return 0;
          v4->field_0x0 = v2;
          v4->field_0x8 = v3->field_0x8;
          v3->field_0x8 = (long)v4;
        }
        else {
          v3->field_0x0 = v2;
          *(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + 1;
        }
        *v5 = 0;
        a1->field_0x18 = a1->field_0x18 + -1;
      }
    }
    v5 = &v5[2];
  } while( true );
}

// Function: sub_c1a3 @ 0xc1a3
unsigned long sub_c1a3(struct_7 *a0,unsigned long a1) // return-dupe x4
{
  void *v1; // stack - 0x68
  unsigned long v10; // stack - 0x20
  void *v2; // stack - 0x60
  unsigned long v3; // stack - 0x58
  unsigned long v4; // stack - 0x50
  unsigned long v5; // stack - 0x48
  unsigned long v6; // stack - 0x40
  unsigned long v7; // stack - 0x38
  unsigned long v8; // stack - 0x30
  unsigned long v9; // stack - 0x28
  
  v3 = sub_b8e5(a1,a0->field_0x28);
  if (!v3)
    return 0;
  if (v3 == a0->field_0x10)
    return 1;
  v1 = calloc(v3,0x10);
  if (!v1)
    return 0;
  v2 = (void *)((long)v1 + v3 * 0x10);
  v4 = 0;
  v5 = 0;
  v6 = a0->field_0x28;
  v7 = a0->field_0x30;
  v8 = a0->field_0x38;
  v9 = a0->field_0x40;
  v10 = a0->field_0x48;
  if (sub_bfcf(&v1,a0,0)) {
    free(a0->field_0x0);
    a0->field_0x0 = v1;
    a0->field_0x8 = v2;
    a0->field_0x10 = v3;
    a0->field_0x18 = v4;
    a0->field_0x48 = v10;
    return 1;
  }
  a0->field_0x48 = v10;
  if ((sub_bfcf(a0,&v1,1) == '\x01') && (sub_bfcf(a0,&v1,0) == '\x01')) {
    free(v1);
    return 0;
  }
  abort(); // no-return
}

// Function: sub_c3ce @ 0xc3ce
unsigned long sub_c3ce(long a0,long a1,long *a2) // return-dupe x2, return-dupe, ternary x2
{
  unsigned long v1; // rax
  long *v2; // rax
  long *v3; // stack - 0x30
  float v4; // xmm0_da
  long v5; // stack - 0x28
  long v6; // stack - 0x20
  
  if (!a1)
    abort(); // no-return
  v5 = sub_be36(a0,a1,&v3,0);
  if (v5) {
    if (!a2)
      return 0;
    *a2 = v5;
    return 0;
  }
  if (((float)*(unsigned long *)(a0 + 0x10) * *(float *)(*(long *)(a0 + 0x28) + 8) < (float)*(unsigned long *)(a0 + 0x18)) && (sub_b7d8(a0), (float)*(unsigned long *)(a0 + 0x10) * *(float *)(*(long *)(a0 + 0x28) + 8) < (float)*(unsigned long *)(a0 + 0x18))) {
    v6 = *(long *)(a0 + 0x28);
    v4 = (*(char *)(v6 + 0x10)) ? (float)*(unsigned long *)(a0 + 0x10) * *(float *)(v6 + 0xc) : *(float *)(v6 + 8) * (*(float *)(v6 + 0xc) * (float)*(unsigned long *)(a0 + 0x10)); // branch-flip
    if (dat_13690 <= v4)
      return 0xffffffff;
    v1 = (dat_13694 <= v4) ? (long)(v4 - dat_13694) ^ 0x8000000000000000 : (unsigned long)v4;
    if (sub_c1a3(a0,v1) != '\x01')
      return 0xffffffff;
    if (sub_be36(a0,a1,&v3,0))
      abort(); // no-return
  }
  if (!*v3) {
    *v3 = a1;
    *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + 1;
    *(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + 1;
    return 1;
  }
  v2 = (long *)sub_bdad(a0);
  if (v2) {
    *v2 = a1;
    v2[1] = v3[1];
    v3[1] = (long)v2;
    *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + 1;
    return 1;
  }
  return 0xffffffff;
}

// Function: sub_c73f @ 0xc73f
unsigned long sub_c73f(long a0,unsigned long a1) // early-return
{
  int v1; // eax
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x18
  
  v1 = sub_c3ce(a0,a1,&v3);
  if (v1 != -1) {
    if (v1) // branch-flip
      v2 = a1;
    else {
      v2 = v3;
    }
    return v2;
  }
  v2 = 0;
  return v2;
}

// Function: sub_c7af @ 0xc7af
long sub_c7af(long a0,long a1) // early-return, ternary x2
{
  char v1; // al
  long *v2; // stack - 0x40
  float v3; // xmm0_da
  void *v4; // stack - 0x38
  long v5; // stack - 0x30
  long v6; // stack - 0x28
  unsigned long v7; // stack - 0x20
  void *v8; // stack - 0x18
  
  v5 = sub_be36(a0,a1,&v2,1);
  if (!v5)
    return 0;
  *(long *)(a0 + 0x20) = *(long *)(a0 + 0x20) + -1;
  if (((!*v2) && (*(long *)(a0 + 0x18) = *(long *)(a0 + 0x18) + -1, (float)*(unsigned long *)(a0 + 0x18) < (float)*(unsigned long *)(a0 + 0x10) * **(float **)(a0 + 0x28))) && (sub_b7d8(a0), (float)*(unsigned long *)(a0 + 0x18) < (float)*(unsigned long *)(a0 + 0x10) * **(float **)(a0 + 0x28))) {
    v6 = *(long *)(a0 + 0x28);
    if (*(char *)(v6 + 0x10)) { // branch-flip
      v3 = (float)*(unsigned long *)(a0 + 0x10) * *(float *)(v6 + 4);
      v7 = (dat_13694 <= v3) ? (long)(v3 - dat_13694) ^ 0x8000000000000000 : (unsigned long)v3;
    }
    else {
      v3 = *(float *)(v6 + 8) * (*(float *)(v6 + 4) * (float)*(unsigned long *)(a0 + 0x10));
      v7 = (dat_13694 <= v3) ? (long)(v3 - dat_13694) ^ 0x8000000000000000 : (unsigned long)v3;
    }
    v1 = sub_c1a3(a0,v7);
    if (v1 != '\x01') {
      v4 = *(void **)(a0 + 0x48);
      while (v4) {
        v8 = *(void **)((long)v4 + 8);
        free(v4);
        v4 = v8;
      }
      *(unsigned long *)(a0 + 0x48) = 0;
    }
  }
  return v5;
}

// Function: sub_cab5 @ 0xcab5
void sub_cab5(long a0,long a1)
{
  sub_c7af(a0,a1);
}

// Function: sub_cade @ 0xcade
char * sub_cade(long a0,long a1)
{
  char *v1;
  long v2; // stack - 0x20
  char *v3; // stack - 0x10
  
  v3 = (char *)(a1 + 0x14);
  *v3 = '\0';
  v2 = a0;
  v1 = v3;
  if (0 <= a0) { // branch-flip
    do {
      v3 = &v3[-1];
      *v3 = (char)v2 + (char)(v2 / 10) * '\xf6' + '0';
      v2 /= 10;
    } while (v2);
  }
  else {
    do {
      v3 = v1;
      v3[-1] = 0x30 - ((char)v2 + (char)(v2 / 10) * '\xf6');
      v2 /= 10;
      v1 = &v3[-1];
    } while (v2);
    v3 = &v3[-2];
    *v3 = '-';
  }
  return v3;
}

// Function: sub_cc1c @ 0xcc1c
long sub_cc1c(void) // early-return x2
{
  long v1; // rax
  char *v2; // stack - 0x30
  long v3; // stack - 0x28
  char *v4; // stack - 0x20
  
  v3 = 0x31069;
  v4 = getenv("_POSIX2_VERSION");
  if (((v4) && (*v4)) && (v1 = strtol(v4,&v2,10), !*v2))
    v3 = v1;
  if (-0x80000000 <= v3) {
    if (v3 <= 0x7fffffff)
      return v3;
    return 0x7fffffff;
  }
  return 0x80000000;
}

// Function: sub_ccd0 @ 0xccd0
void sub_ccd0(char *a0)
{
  char *v1;
  char *v2; // stack - 0x20
  
  if (!a0) {
    fputs("A NULL argv[0] was passed through an exec system call.\n",stderr);
    abort(); // no-return
  }
  v1 = strrchr(a0,0x2f);
  if (v1) // branch-flip
    v1 = &v1[1];
  else {
    v1 = a0;
  }
  v2 = a0;
  if (7 <= (long)v1 - (long)a0) {
    v2 = a0;
    if (!strncmp(&v1[-7],"/.libs/",7)) {
      v2 = v1;
      if (!strncmp(v1,"lt-",3)) {
        __progname = &v1[3];
        v2 = __progname;
      }
    }
  }
  dat_18150 = v2;
  program_invocation_name = v2;
}

// Function: sub_cdbb @ 0xcdbb
unsigned long sub_cdbb(void *a0)
{
  int v1;
  unsigned long v2; // rax
  
  v1 = *__errno_location();
  if (!a0)
    a0 = (void *)0x18160;
  v2 = sub_feac(a0,0x38);
  *__errno_location() = v1;
  return v2;
}

// Function: sub_ce0a @ 0xce0a
unsigned int sub_ce0a(unsigned int *a0)
{
  if (!a0)
    a0 = (unsigned int *)0x18160;
  return *a0;
}

// Function: sub_ce2e @ 0xce2e
void sub_ce2e(unsigned int *a0,unsigned int a1)
{
  if (!a0)
    a0 = (unsigned int *)0x18160;
  *a0 = a1;
}

// Function: sub_ce59 @ 0xce59
unsigned int sub_ce59(long a0,unsigned char a1,unsigned int a2)
{
  unsigned int v1; // eax
  unsigned int *v2; // rax
  
  if (!a0)
    a0 = 0x18160;
  v2 = (unsigned int *)((unsigned long)(a1 >> 5) * 4 + a0 + 8);
  v1 = *v2 >> (a1 & 0x1f) & 1;
  *v2 = *v2 ^ (a2 & 1 ^ v1) << (a1 & 0x1f);
  return v1;
}

// Function: sub_cee9 @ 0xcee9
unsigned int sub_cee9(long a0,unsigned int a1)
{
  unsigned int v1;
  long v2; // stack - 0x20
  
  v2 = a0;
  if (!a0)
    v2 = 0x18160;
  v1 = *(unsigned int *)(v2 + 4);
  *(unsigned int *)(v2 + 4) = a1;
  return v1;
}

// Function: sub_cf23 @ 0xcf23
void sub_cf23(unsigned int *a0,long a1,long a2)
{
  unsigned int *v1; // stack - 0x10
  
  v1 = a0;
  if (!a0)
    v1 = (unsigned int *)0x18160;
  *v1 = 10;
  if ((a1) && (a2)) {
    *(long *)&v1[10] = a1;
    *(long *)&v1[0xc] = a2;
    return;
  }
  abort(); // no-return
}

// Function: sub_cf85 @ 0xcf85
unsigned long * sub_cf85(unsigned long *a0,unsigned int a1)
{
  unsigned long v1; // stack - 0x58
  
  if (a1 != 10) {
    v1 = (unsigned long)a1;
    *a0 = v1;
    a0[1] = 0;
    a0[2] = 0;
    a0[3] = 0;
    a0[4] = 0;
    a0[5] = 0;
    a0[6] = 0;
    return a0;
  }
  abort(); // no-return
}

// Function: sub_d04a @ 0xd04a
char * sub_d04a(char *a0,int a1) // early-return x6
{
  char *v1; // rax
  char *v2; // rax
  
  v1 = gettext(a0);
  if (v1 != a0)
    return v1;
  v2 = (char *)sub_1105e();
  if (!sub_10a66(v2,"UTF-8")) {
    if (*a0 == '`')
      return "‘";
    return "’";
  }
  if (sub_10a66(v2,"GB18030")) {
    if (a1 == 9)
      return "\"";
    return "\'";
  }
  if (*a0 == '`')
    return (char *)0x1379e;
  return (char *)0x137a2;
}

// Function: sub_d10f @ 0xd10f
unsigned long sub_d10f(long a0,unsigned long a1,char *a2,unsigned long a3,int a4,unsigned int a5,long a6,char *a7,char *a8)
{
  char *v1;
  unsigned long v10;
  unsigned long v11;
  mbstate_t v12;
  unsigned int v13; // stack - 0x74
  bool v14;
  bool v15;
  bool v16;
  char *v17; // stack - 0xc8
  char *v18; // stack - 0xc0
  int v19; // stack - 0xac
  bool v2;
  unsigned long v20; // stack - 0xa8
  unsigned long v21; // stack - 0x98
  unsigned char v22; // stack - 0x7b
  unsigned char v23; // stack - 0x7a
  unsigned long v24; // stack - 0x70
  unsigned long v25; // stack - 0x68
  unsigned long v26; // stack - 0x60
  char *v27; // stack - 0x58
  unsigned long v28; // stack - 0x50
  unsigned long v29; // stack - 0x48
  bool v3;
  unsigned long v30; // stack - 0x40
  unsigned long v31; // stack - 0x38
  unsigned long v32; // stack - 0x30
  bool v4;
  bool v5;
  bool v6;
  bool v7;
  long v8;
  unsigned long v9; // rax
  
  v8 = a6;
  v18 = a7;
  v17 = a8;
  v25 = 0;
  v26 = 0;
  v27 = NULL;
  v28 = 0;
  v2 = 0;
  v9 = __ctype_get_mb_cur_max();
  v14 = (a5 & 2) != 0;
  v3 = 0;
  v6 = 0;
  v7 = 1;
  v19 = a4;
  v20 = a3;
  v21 = a1;
label_d1c9:
  switch(v19) {
    case 0:
      v14 = 0;
      break;
    case 1:
label_d31d:
      v14 = 1;
      goto label_d321;
    case 2:
label_d330:
      v19 = 2;
      if (!v14) {
        if (v25 < v21)
          *(char *)(v25 + a0) = 0x27;
        v25 += 1;
      }
      v27 = "\'";
      v28 = 1;
      break;
    case 3:
      v2 = 1;
      goto label_d31d;
    case 4:
label_d321:
      if (!v14)
        v2 = 1;
      goto label_d330;
    case 5:
label_d20b:
      if (!v14) {
        if (v25 < v21)
          *(char *)(v25 + a0) = 0x22;
        v25 += 1;
      }
      v2 = 1;
      v27 = "\"";
      v28 = 1;
      break;
    case 6:
      v19 = 5;
      v14 = 1;
      goto label_d20b;
    case 7:
      v2 = 1;
      v14 = 0;
      break;
    case 8:
    case 9:
    case 10:
      if (v19 != 10) {
        v18 = (char *)sub_d04a("`",v19);
        v17 = (char *)sub_d04a("\'",v19);
      }
      if (!v14) {
        for (v27 = v18; *v27; v27 = &v27[1]) {
          if (v25 < v21)
            *(char *)(a0 + v25) = *v27;
          v25 += 1;
        }
      }
      v2 = 1;
      v27 = v17;
      v28 = strlen(v17);
      break;
    default:
      abort(); // no-return
    
  }
  v24 = 0;
label_dfcf:
  if (v20 != 0xffffffffffffffff) // branch-flip
    v16 = v24 != v20;
  else {
    v16 = a2[v24] != '\0';
  }
  if (!v16) goto label_e007;
  v4 = 0;
  v5 = 0;
  v16 = 0;
  if (((v2) && (v19 != 2)) && (v28)) {
    v11 = v24 + v28;
    if ((v20 != 0xffffffffffffffff) || (v28 <= 1))
      v10 = v20;
    else {
      v20 = strlen(a2);
      v10 = v20;
    }
    if ((v11 <= v10) && (v1 = &a2[v24], !memcmp(v1,v27,v28))) {
      if (v14) goto label_e14b;
      v4 = 1;
    }
  }
  v22 = a2[v24];
  if (v22 == 0x7e) {
label_d8ee:
    if (!v24) {
label_d8f9:
      v16 = 1;
label_d8fd:
      if (v19 != 2) goto label_de14;
label_d6d6:
      if (v14) goto label_e14b;
    }
    goto label_de14;
  }
  if (0x7e < v22) {
label_d9da:
    if (v9 != 1) {
      memset(&v12,0,8);
      v29 = 0;
      v16 = 1;
      if (v20 == 0xffffffffffffffff)
        v20 = strlen(a2);
label_da54:
      v31 = sub_110a6(&v13,&a2[v24 + v29],v20 - (v24 + v29),&v12);
      if (v31) {
        if (v31 != 0xffffffffffffffff) { // branch-flip
          if (v31 != 0xfffffffffffffffe) goto label_db02;
          v16 = 0;
          for (; (v29 + v24 < v20 && (a2[v24 + v29])); v29 = v29 + 1) {
          }
        }
        else {
          v16 = 0;
        }
      }
      goto label_dbb1;
    }
    v29 = 1;
    v16 = (*(unsigned short *)((unsigned long)v22 * 2 + *(long *)__ctype_b_loc()) & 0x4000) != 0;
label_dbb1:
    if ((v29 <= 1) && ((!v2 || (v16)))) goto label_de14;
    v32 = v29 + v24;
    while( true ) {
      if ((!v2) || (v16)) {
        if (v4) {
          if (v25 < v21)
            *(char *)(v25 + a0) = 0x5c;
          v25 += 1;
          v4 = 0;
        }
      }
      else {
        if (v14) goto label_e14b;
        v5 = 1;
        if ((v19 == 2) && (!v3)) {
          if (v25 < v21)
            *(char *)(v25 + a0) = 0x27;
          if (v25 + 1 < v21)
            *(char *)(v25 + 1 + a0) = 0x24;
          if (v25 + 2 < v21)
            *(char *)(v25 + 2 + a0) = 0x27;
          v25 += 3;
          v3 = 1;
        }
        if (v25 < v21)
          *(char *)(v25 + a0) = 0x5c;
        if (v25 + 1 < v21)
          *(unsigned char *)(v25 + 1 + a0) = (v22 >> 6) + 0x30;
        if (v25 + 2 < v21)
          *(unsigned char *)(v25 + 2 + a0) = (v22 >> 3 & 7) + 0x30;
        v25 += 3;
        v22 = (v22 & 7) + 0x30;
      }
      if (v32 <= v24 + 1) break;
      if ((v3) && (!v5)) {
        if (v25 < v21)
          *(char *)(v25 + a0) = 0x27;
        if (v25 + 1 < v21)
          *(char *)(v25 + 1 + a0) = 0x27;
        v25 += 2;
        v3 = 0;
      }
      if (v25 < v21)
        *(unsigned char *)(a0 + v25) = v22;
      v25 += 1;
      v24 += 1;
      v22 = a2[v24];
    }
    goto label_df3a;
  }
  if (v22 == 0x7d) {
label_d8bc:
    if (v20 != 0xffffffffffffffff) // branch-flip
      v15 = v20 != 1;
    else {
      v15 = a2[1] != '\0';
    }
    if (v15) goto label_de14;
    goto label_d8ee;
  }
  if (0x7d < v22) goto label_d9da;
  if (v22 == 0x7c) goto label_d8fd;
  if (0x7c < v22) goto label_d9da;
  if (v22 == 0x7b) goto label_d8bc;
  if (0x7b < v22) goto label_d9da;
  if (0x3f < v22) {
    if ((0x7a < v22) || (v22 <= 0x40)) goto label_d9da;
    v11 = 1L << (v22 + 0xbf & 0x3f);
    if (v11 & 0x3ffffff53ffffff) goto label_d9d1;
    if (v11 & 0xa4000000) goto label_d8fd;
    if (!(v11 & 0x8000000)) goto label_d9da;
    if (v19 == 2) {
      if (!v14) goto label_df3a;
      goto label_e14b;
    }
    v23 = v22;
    if (((!v2) || (!v14)) || (!v28)) goto label_d892;
    goto label_df3a;
  }
  switch(v22) {
    case 0:
      if (!v2) {
        if (!(a5 & 1)) goto label_de14;
        goto label_dfca;
      }
      if (!v14) {
        v5 = 1;
        if ((v19 == 2) && (!v3)) {
          if (v25 < v21)
            *(char *)(v25 + a0) = 0x27;
          if (v25 + 1 < v21)
            *(char *)(v25 + 1 + a0) = 0x24;
          if (v25 + 2 < v21)
            *(char *)(v25 + 2 + a0) = 0x27;
          v25 += 3;
          v3 = 1;
        }
        if (v25 < v21)
          *(char *)(v25 + a0) = 0x5c;
        v11 = v25 + 1;
        if ((((v19 != 2) && (v24 + 1 < v20)) && ('0' <= a2[v24 + 1])) && (a2[v24 + 1] <= '9')) {
          if (v11 < v21)
            *(char *)(v11 + a0) = 0x30;
          if (v25 + 2 < v21)
            *(char *)(v25 + 2 + a0) = 0x30;
          v11 = v25 + 3;
        }
        v25 = v11;
        v22 = 0x30;
        goto label_de14;
      }
      goto label_e14b;
    default:
      goto label_d9da;
    case 7:
      v23 = 0x61;
      break;
    case 8:
      v23 = 0x62;
      break;
    case 9:
      v23 = 0x74;
      goto label_d892;
    case 10:
      v23 = 0x6e;
      goto label_d892;
    case 0xb:
      v23 = 0x76;
      break;
    case 0xc:
      v23 = 0x66;
      break;
    case 0xd:
      v23 = 0x72;
label_d892:
      if ((v19 == 2) && (v14)) goto label_e14b;
      break;
    case 0x20:
      goto label_d8f9;
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
      goto label_d8fd;
    case 0x23:
      goto label_d8ee;
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
label_d9d1:
      v16 = 1;
      goto label_de14;
    case 0x27:
      v6 = 1;
      v16 = 1;
      if (v19 == 2) {
        if (!v14) {
          if ((v21) && (!v26)) {
            v26 = v21;
            v21 = 0;
          }
          if (v25 < v21)
            *(char *)(v25 + a0) = 0x27;
          if (v25 + 1 < v21)
            *(char *)(v25 + 1 + a0) = 0x5c;
          if (v25 + 2 < v21)
            *(char *)(v25 + 2 + a0) = 0x27;
          v25 += 3;
          v3 = 0;
          goto label_de14;
        }
        goto label_e14b;
      }
      goto label_de14;
    case 0x3f:
      if (v19 == 2) goto label_d6d6;
      if ((((v19 == 5) && (a5 & 4)) && ((v24 + 2 < v20 && ((a2[v24 + 1] == '?' && ((unsigned char)a2[v24 + 2] <= 0x3e)))))) && (0x7000a38200000000U >> (a2[v24 + 2] & 0x3fU) & 1)) {
        if (!v14) {
          v22 = a2[v24 + 2];
          v24 += 2;
          if (v25 < v21)
            *(char *)(v25 + a0) = 0x3f;
          if (v25 + 1 < v21)
            *(char *)(v25 + 1 + a0) = 0x22;
          if (v25 + 2 < v21)
            *(char *)(v25 + 2 + a0) = 0x22;
          if (v25 + 3 < v21)
            *(char *)(v25 + 3 + a0) = 0x3f;
          v25 += 4;
          goto label_de14;
        }
        goto label_e14b;
      }
      goto label_de14;
    
  }
  if (v2) {
    v22 = v23;
label_de7f:
    if (v14) goto label_e14b;
    v5 = 1;
    if ((v19 == 2) && (!v3)) {
      if (v25 < v21)
        *(char *)(v25 + a0) = 0x27;
      if (v25 + 1 < v21)
        *(char *)(v25 + 1 + a0) = 0x24;
      if (v25 + 2 < v21)
        *(char *)(v25 + 2 + a0) = 0x27;
      v25 += 3;
      v3 = 1;
    }
    if (v25 < v21)
      *(char *)(v25 + a0) = 0x5c;
    v25 += 1;
  }
  else {
label_de14:
    if (((((v2) && (v19 != 2)) || (v14)) && ((v8 && (*(unsigned int *)(v8 + (unsigned long)(v22 >> 5) * 4) >> (v22 & 0x1f) & 1)))) || (v4)) goto label_de7f;
  }
label_df3a:
  if ((v3) && (!v5)) {
    if (v25 < v21)
      *(char *)(v25 + a0) = 0x27;
    if (v25 + 1 < v21)
      *(char *)(v25 + 1 + a0) = 0x27;
    v25 += 2;
    v3 = 0;
  }
  if (v25 < v21)
    *(unsigned char *)(a0 + v25) = v22;
  v25 += 1;
  if (!v16)
    v7 = 0;
label_dfca:
  v24 += 1;
  goto label_dfcf;
label_db02:
  if ((v14) && (v19 == 2)) {
    for (v30 = 1; v30 < v31; v30 = v30 + 1) {
      if (((unsigned int)((int)a2[v24 + v29 + v30] - 0x5bU) <= 0x21) && (0x20000002bU >> ((unsigned char)((int)a2[v24 + v29 + v30] - 0x5bU) & 0x3f) & 1)) goto label_e14b;
    }
  }
  if (!iswprint(v13))
    v16 = 0;
  v29 += v31;
  if (mbsinit(&v12)) goto label_dbb1;
  goto label_da54;
label_e007:
  if (((!v25) && (v19 == 2)) && (v14)) {
label_e14b:
    if ((v19 == 2) && (v2))
      v19 = 4;
    return sub_d10f(a0,v21,a2,v20,v19,a5 & 0xfffffffd,0,v18,v17);
  }
  if (((v19 == 2) && (!v14)) && (v6)) {
    if (v7)
      return sub_d10f(a0,v26,a2,v20,5,a5,v8,v18,v17);
    if ((!v21) && (v26)) {
      v21 = v26;
      v25 = 0;
      goto label_d1c9;
    }
  }
  if ((v27) && (!v14)) {
    for (; *v27; v27 = &v27[1]) {
      if (v25 < v21)
        *(char *)(a0 + v25) = *v27;
      v25 += 1;
    }
  }
  if (v25 < v21)
    *(char *)(v25 + a0) = 0;
  return v25;
}

// Function: sub_e1d0 @ 0xe1d0
unsigned long sub_e1d0(long a0,unsigned long a1,char *a2,unsigned long a3,unsigned int *a4)
{
  int v1;
  unsigned long v2; // rax
  
  if (!a4)
    a4 = (unsigned int *)0x18160;
  v1 = *__errno_location();
  v2 = sub_d10f(a0,a1,a2,a3,*a4,a4[1],&a4[2],*(char **)&a4[10],*(char **)&a4[0xc]);
  *__errno_location() = v1;
  return v2;
}

// Function: sub_e278 @ 0xe278
void sub_e278(char *a0,unsigned long a1,unsigned int *a2)
{
  sub_e2ae(a0,a1,NULL,a2);
}

// Function: sub_e2ae @ 0xe2ae
long sub_e2ae(char *a0,unsigned long a1,long *a2,unsigned int *a3)
{
  int v1;
  unsigned int v2; // eax
  long v3; // rax
  long v4; // rax
  
  if (!a3)
    a3 = (unsigned int *)0x18160;
  v1 = *__errno_location();
  v2 = (unsigned int)(a2 == NULL) | a3[1];
  v3 = sub_d10f(0,0,a0,a1,*a3,v2,&a3[2],*(char **)&a3[10],*(char **)&a3[0xc]);
  v4 = sub_f9b8(v3 + 1U);
  sub_d10f(v4,v3 + 1U,a0,a1,*a3,v2,&a3[2],*(char **)&a3[10],*(char **)&a3[0xc]);
  *__errno_location() = v1;
  if (a2)
    *a2 = v3;
  return v4;
}

// Function: sub_e3da @ 0xe3da
void sub_e3da(void) // return-dupe
{
  void *v1;
  int v2; // stack - 0x14
  
  v1 = dat_180d0;
  for (v2 = 1; v2 < dat_18060; v2 = v2 + 1) {
    free(*(void **)((long)v1 + (long)v2 * 0x10 + 8));
  }
  if (*(long *)((long)v1 + 8) != 0x181a0) {
    free(*(void **)((long)v1 + 8));
    dat_180c0 = 0x100;
    dat_180c8 = 0x181a0;
  }
  if (v1 == (void *)0x180c0) {
    dat_18060 = 1;
    return;
  }
  free(v1);
  dat_18060 = 1;
  dat_180d0 = (void *)0x180c0;
}

// Function: sub_e499 @ 0xe499
void * sub_e499(int a0,char *a1,unsigned long a2,struct_0 *a3)
{
  int v1;
  unsigned int v2; // eax
  unsigned long *v3;
  long v4; // stack - 0x38
  bool v5; // zf
  unsigned long *v6; // stack - 0x30
  void *v7; // stack - 0x28
  unsigned long v8; // stack - 0x20
  unsigned long v9; // stack - 0x18
  
  v1 = *__errno_location();
  v6 = dat_180d0;
  if ((0 <= a0) && (a0 <= 0x7ffffffe)) {
    if (dat_18060 <= a0) {
      v5 = dat_180d0 == (unsigned long *)0x180c0;
      v4 = (long)dat_18060;
      if (v5)
        v3 = NULL;
      else {
        v3 = dat_180d0;
      }
      dat_180d0 = (unsigned long *)sub_fc48(v3,&v4,(long)((a0 - dat_18060) + 1),0x7fffffff,0x10);
      v6 = dat_180d0;
      if (v5) {
        *dat_180d0 = dat_180c0;
        dat_180d0[1] = dat_180c8;
      }
      memset(&dat_180d0[(long)dat_18060 * 2],0,(v4 - dat_18060) * 0x10);
      dat_18060 = (int)v4;
    }
    v8 = v6[(long)a0 * 2];
    v7 = (void *)v6[(long)a0 * 2 + 1];
    v2 = a3->field_0x4 | 1;
    v9 = sub_d10f(v7,v8,a1,a2,a3->field_0x0,v2,a3->field_0x8,a3->field_0x28,a3->field_0x30);
    if (v8 <= v9) {
      v8 = v9 + 1;
      v6[(long)a0 * 2] = v8;
      if (v7 != (void *)0x181a0)
        free(v7);
      v7 = (void *)sub_f9b8(v8);
      v6[(long)a0 * 2 + 1] = v7;
      sub_d10f(v7,v8,a1,a2,a3->field_0x0,v2,a3->field_0x8,a3->field_0x28,a3->field_0x30);
    }
    *__errno_location() = v1;
    return v7;
  }
  abort(); // no-return
}

// Function: sub_e748 @ 0xe748
void sub_e748(unsigned int a0,char *a1)
{
  sub_e499(a0,a1,0xffffffffffffffff,(struct_0 *)0x18160);
}

// Function: sub_e77c @ 0xe77c
void sub_e77c(unsigned int a0,char *a1,unsigned long a2)
{
  sub_e499(a0,a1,a2,(struct_0 *)0x18160);
}

// Function: sub_e7ae @ 0xe7ae
void sub_e7ae(char *a0)
{
  sub_e748(0,a0);
}

// Function: sub_e7d1 @ 0xe7d1
void sub_e7d1(char *a0,unsigned long a1)
{
  sub_e77c(0,a0,a1);
}

// Function: sub_e7fc @ 0xe7fc
void sub_e7fc(unsigned int a0,unsigned int a1,char *a2)
{
  char v1 [56];
  
  sub_cf85(v1,a1);
  sub_e499(a0,a2,0xffffffffffffffff,v1);
}

// Function: sub_e864 @ 0xe864
void sub_e864(unsigned int a0,unsigned int a1,char *a2,unsigned long a3)
{
  char v1 [56];
  
  sub_cf85(v1,a1);
  sub_e499(a0,a2,a3,v1);
}

// Function: sub_e8ca @ 0xe8ca
void sub_e8ca(unsigned int a0,char *a1)
{
  sub_e7fc(0,a0,a1);
}

// Function: sub_e8f2 @ 0xe8f2
void sub_e8f2(unsigned int a0,char *a1,unsigned long a2)
{
  sub_e864(0,a0,a1,a2);
}

// Function: sub_e922 @ 0xe922
void sub_e922(char *a0,unsigned long a1,char a2)
{
  unsigned long v1; // stack - 0x48
  unsigned long v2; // stack - 0x40
  unsigned long v3; // stack - 0x38
  unsigned long v4; // stack - 0x30
  unsigned long v5; // stack - 0x28
  unsigned long v6; // stack - 0x20
  unsigned long v7; // stack - 0x18
  
  v1 = dat_18160;
  v2 = dat_18168;
  v3 = dat_18170;
  v4 = dat_18178;
  v5 = dat_18180;
  v6 = dat_18188;
  v7 = dat_18190;
  sub_ce59(&v1,(int)a2,1);
  sub_e499(0,a0,a1,&v1);
}

// Function: sub_e9dd @ 0xe9dd
void sub_e9dd(char *a0,char a1)
{
  sub_e922(a0,0xffffffffffffffff,(int)a1);
}

// Function: sub_ea0b @ 0xea0b
void sub_ea0b(char *a0)
{
  sub_e9dd(a0,0x3a);
}

// Function: sub_ea2e @ 0xea2e
void sub_ea2e(char *a0,unsigned long a1)
{
  sub_e922(a0,a1,0x3a);
}

// Function: sub_ea5c @ 0xea5c
void sub_ea5c(unsigned int a0,unsigned int a1,char *a2)
{
  unsigned long v1; // stack - 0x98
  unsigned int v10; // stack - 0x50
  unsigned int v11; // stack - 0x4c
  unsigned long v12; // stack - 0x40
  unsigned long v13; // stack - 0x38
  unsigned long v14; // stack - 0x30
  unsigned long v15; // stack - 0x28
  unsigned long v16; // stack - 0x20
  unsigned long v17; // stack - 0x18
  unsigned long v2; // stack - 0x48
  unsigned long v3; // stack - 0x90
  unsigned long v4; // stack - 0x88
  unsigned long v5; // stack - 0x80
  unsigned long v6; // stack - 0x78
  unsigned long v7; // stack - 0x70
  unsigned long v8; // stack - 0x68
  char *v9; // stack - 0x58
  
  v9 = a2;
  v10 = a1;
  v11 = a0;
  sub_cf85(&v1,a1);
  v2 = v1;
  v12 = v3;
  v13 = v4;
  v14 = v5;
  v15 = v6;
  v16 = v7;
  v17 = v8;
  sub_ce59(&v2,0x3a,1);
  sub_e499(v11,v9,0xffffffffffffffff,&v2);
}

// Function: sub_eb1e @ 0xeb1e
void sub_eb1e(unsigned int a0,long a1,long a2,char *a3)
{
  sub_eb58(a0,a1,a2,a3,0xffffffffffffffff);
}

// Function: sub_eb58 @ 0xeb58
void sub_eb58(unsigned int a0,long a1,long a2,char *a3,unsigned long a4)
{
  unsigned long v1; // stack - 0x48
  unsigned long v2; // stack - 0x40
  unsigned long v3; // stack - 0x38
  unsigned long v4; // stack - 0x30
  unsigned long v5; // stack - 0x28
  unsigned long v6; // stack - 0x20
  unsigned long v7; // stack - 0x18
  
  v1 = dat_18160;
  v2 = dat_18168;
  v3 = dat_18170;
  v4 = dat_18178;
  v5 = dat_18180;
  v6 = dat_18188;
  v7 = dat_18190;
  sub_cf23(&v1,a1,a2);
  sub_e499(a0,a3,a4,&v1);
}

// Function: sub_ec16 @ 0xec16
void sub_ec16(long a0,long a1,char *a2)
{
  sub_eb1e(0,a0,a1,a2);
}

// Function: sub_ec49 @ 0xec49
void sub_ec49(long a0,long a1,char *a2,unsigned long a3)
{
  sub_eb58(0,a0,a1,a2,a3);
}

// Function: sub_ec87 @ 0xec87
void sub_ec87(unsigned int a0,char *a1,unsigned long a2)
{
  sub_e499(a0,a1,a2,(struct_0 *)0x18080);
}

// Function: sub_ecb9 @ 0xecb9
void sub_ecb9(char *a0,unsigned long a1)
{
  sub_ec87(0,a0,a1);
}

// Function: sub_ece4 @ 0xece4
void sub_ece4(unsigned int a0,char *a1)
{
  sub_ec87(a0,a1,0xffffffffffffffff);
}

// Function: sub_ed11 @ 0xed11
void sub_ed11(char *a0)
{
  sub_ece4(0,a0);
}

// Function: sub_ed34 @ 0xed34
long sub_ed34(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  unsigned long v2; // stack - 0x30
  
  v2 = a2;
  while( true ) {
    do {
      v1 = read(a0,a1,v2);
      if (0 <= v1)
        return v1;
    } while (*__errno_location() == 4);
    if (*__errno_location() != 0x16) break;
    if (v2 <= 0x7ff00000)
      return v1;
    v2 = 0x7ff00000;
  }
  return v1;
}

// Function: sub_eda8 @ 0xeda8
unsigned long sub_eda8(long a0)
{
  return *(unsigned long *)(a0 + 0x50);
}

// Function: sub_edbe @ 0xedbe
unsigned long sub_edbe(long a0)
{
  return *(unsigned long *)(a0 + 0x70);
}

// Function: sub_edd4 @ 0xedd4
unsigned long sub_edd4(long a0)
{
  return *(unsigned long *)(a0 + 0x60);
}

// Function: sub_edea @ 0xedea
unsigned long sub_edea(void)
{
  return 0;
}

// Function: sub_edfd @ 0xedfd
undefined16 sub_edfd(long a0)
{
  return *(char (*)[16])(a0 + 0x48);
}

// Function: sub_ee17 @ 0xee17
undefined16 sub_ee17(long a0)
{
  return *(char (*)[16])(a0 + 0x68);
}

// Function: sub_ee31 @ 0xee31
undefined16 sub_ee31(long a0)
{
  return *(char (*)[16])(a0 + 0x58);
}

// Function: sub_ee4b @ 0xee4b
undefined16 sub_ee4b(void)
{
  char v1 [16];
  
  v1._8_8_ = 0xffffffffffffffff;
  v1._0_8_ = 0xffffffffffffffff;
  return v1._0_16_;
}

// Function: sub_ee71 @ 0xee71
unsigned int sub_ee71(unsigned int a0)
{
  return a0;
}

// Function: sub_ee85 @ 0xee85
undefined16 sub_ee85(unsigned long a0,unsigned long a1)
{
  char v1 [16];
  
  v1._8_8_ = a1;
  v1._0_8_ = a0;
  return v1._0_16_;
}

// Function: sub_eeaf @ 0xeeaf
int sub_eeaf(long a0,long a1,long a2,long a3)
{
  return ((unsigned int)(a3 < a1) - (unsigned int)(a1 < a3)) + ((unsigned int)(a2 < a0) - (unsigned int)(a0 < a2)) * 2;
}

// Function: sub_ef25 @ 0xef25
int sub_ef25(long a0,long a1)
{
  return (unsigned int)(a1 && !a0) + (unsigned int)(0 < a0) + (int)(a0 >> 0x3f);
}

// Function: sub_ef77 @ 0xef77
double sub_ef77(long a0,long a1)
{
  return (double)a1 / dat_138d8 + (double)a0;
}

// Function: sub_efc6 @ 0xefc6
int sub_efc6(int a0)
{
  int v1;
  int v2; // stack - 0x1c
  
  v2 = a0;
  if ((0 <= a0) && (a0 <= 2)) {
    v2 = sub_11256(a0);
    v1 = *__errno_location();
    close(a0);
    *__errno_location() = v1;
  }
  return v2;
}

// Function: sub_f017 @ 0xf017
void sub_f017(FILE *a0,char *a1,char *a2,char *a3,unsigned long *a4,unsigned long a5) // return-dupe x10
{
  unsigned long v1;
  char *v10;
  char *v11;
  char *v12;
  char *v13;
  char *v14;
  char *v15;
  char *v16;
  char *v17;
  char *v18;
  unsigned long v2;
  unsigned long v3;
  unsigned long v4;
  unsigned long v5;
  unsigned long v6;
  unsigned long v7;
  unsigned long v8;
  unsigned long v9;
  
  if (a1) // branch-flip
    fprintf(a0,"%s (%s) %s\n",a1,a2,a3);
  else {
    fprintf(a0,"%s %s\n",a2,a3);
  }
  fprintf(a0,"Copyright %s %d Free Software Foundation, Inc.",gettext("(C)"),0x7e6);
  fputs_unlocked("\n",a0);
  fprintf(a0,gettext("License GPLv3+: GNU GPL version 3 or later <%s>.\nThis is free software: you are free to change and redistribute it.\nThere is NO WARRANTY, to the extent permitted by law.\n"),"https://gnu.org/licenses/gpl.html");
  fputs_unlocked("\n",a0);
  switch(a5) {
    case 0:
      return;
    case 1:
      v1 = *a4;
      fprintf(a0,gettext("Written by %s.\n"),v1);
      return;
    case 2:
      v1 = a4[1];
      v2 = *a4;
      fprintf(a0,gettext("Written by %s and %s.\n"),v2,v1);
      return;
    case 3:
      v1 = a4[2];
      v2 = a4[1];
      v3 = *a4;
      fprintf(a0,gettext("Written by %s, %s, and %s.\n"),v3,v2,v1);
      return;
    case 4:
      v1 = a4[3];
      v2 = a4[2];
      v3 = a4[1];
      v4 = *a4;
      fprintf(a0,gettext("Written by %s, %s, %s,\nand %s.\n"),v4,v3,v2,v1);
      return;
    case 5:
      v1 = a4[4];
      v2 = a4[3];
      v3 = a4[2];
      v4 = a4[1];
      v5 = *a4;
      fprintf(a0,gettext("Written by %s, %s, %s,\n%s, and %s.\n"),v5,v4,v3,v2,v1);
      return;
    case 6:
      v1 = a4[5];
      v2 = a4[4];
      v3 = a4[3];
      v4 = a4[2];
      v5 = a4[1];
      v6 = *a4;
      fprintf(a0,gettext("Written by %s, %s, %s,\n%s, %s, and %s.\n"),v6,v5,v4,v3,v2,v1);
      return;
    case 7:
      v1 = a4[6];
      v2 = a4[5];
      v3 = a4[4];
      v4 = a4[3];
      v5 = a4[2];
      v6 = a4[1];
      v7 = *a4;
      fprintf(a0,gettext("Written by %s, %s, %s,\n%s, %s, %s, and %s.\n"),v7,v6,v5,v4,v3,v2,v1);
      return;
    case 8:
      v1 = a4[7];
      v2 = a4[6];
      v3 = a4[5];
      v4 = a4[4];
      v5 = a4[3];
      v6 = a4[2];
      v7 = a4[1];
      v8 = *a4;
      fprintf(a0,gettext("Written by %s, %s, %s,\n%s, %s, %s, %s,\nand %s.\n"),v8,v7,v6,v5,v4,v3,v2,v1);
      return;
    case 9:
      v1 = a4[8];
      v2 = a4[7];
      v3 = a4[6];
      v4 = a4[5];
      v5 = a4[4];
      v6 = a4[3];
      v7 = a4[2];
      v8 = a4[1];
      v9 = *a4;
      fprintf(a0,gettext("Written by %s, %s, %s,\n%s, %s, %s, %s,\n%s, and %s.\n"),v9,v8,v7,v6,v5,v4,v3,v2,v1);
      return;
    default:
      v10 = (char *)a4[8];
      v11 = (char *)a4[7];
      v12 = (char *)a4[6];
      v13 = (char *)a4[5];
      v14 = (char *)a4[4];
      v15 = (char *)a4[3];
      v16 = (char *)a4[2];
      v17 = (char *)a4[1];
      v18 = (char *)*a4;
      fprintf(a0,gettext("Written by %s, %s, %s,\n%s, %s, %s, %s,\n%s, %s, and others.\n"),v18,v17,v16,v15,v14,v13,v12,v11,v10);
      return;
    
  }
}

// Function: sub_f613 @ 0xf613
void sub_f613(FILE *a0,char *a1,char *a2,char *a3,unsigned long *a4)
{
  long v1; // stack - 0x10
  
  for (v1 = 0; a4[v1]; v1 = v1 + 1) {
  }
  sub_f017(a0,a1,a2,a3,a4,v1);
}

// Function: sub_f686 @ 0xf686
void sub_f686(FILE *a0,char *a1,char *a2,char *a3,struct_9 *a4)
{
  long *v1;
  long v2 [11];
  unsigned long v3; // stack - 0x70
  
  for (v3 = 0; v3 <= 9; v3 = v3 + 1) {
    if (0x30 <= a4->field_0x0) { // branch-flip
      v1 = a4->field_0x8;
      a4->field_0x8 = &v1[1];
    }
    else {
      v1 = (long *)((unsigned long)a4->field_0x0 + a4->field_0x10);
      a4->field_0x0 = a4->field_0x0 + 8;
    }
    v2[v3] = *v1;
    if (!v2[v3]) break;
  }
  sub_f017(a0,a1,a2,a3,v2,v3);
}

// Function: sub_f78c @ 0xf78c
void sub_f78c(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,unsigned long a6,unsigned long a7,FILE *a8,char *a9,char *a10,char *a11,unsigned long a12,unsigned long a13)
{
  char v1; // al
  unsigned long v10; // stack - 0x78
  unsigned long v11; // stack - 0x68
  unsigned long v12; // stack - 0x58
  unsigned long v13; // stack - 0x48
  unsigned long v14; // stack - 0x38
  unsigned long v15; // stack - 0x28
  unsigned long v16; // stack - 0x18
  char v2 [32];
  unsigned int v3; // stack - 0xd8
  unsigned int v4; // stack - 0xd4
  char *v5; // stack - 0xd0
  char *v6; // stack - 0xc8
  unsigned long v7; // stack - 0x98
  unsigned long v8; // stack - 0x90
  unsigned long v9; // stack - 0x88
  
  if (v1) {
    v9 = a0;
    v10 = a1;
    v11 = a2;
    v12 = a3;
    v13 = a4;
    v14 = a5;
    v15 = a6;
    v16 = a7;
  }
  v3 = 0x20;
  v4 = 0x30;
  v5 = &Stack0000000000000008;
  v6 = v2;
  v7 = a12;
  v8 = a13;
  sub_f686(a8,a9,a10,a11,&v3);
}

// Function: sub_f870 @ 0xf870
void sub_f870(void)
{
  fputs_unlocked("\n",stdout);
  printf(gettext("Report bugs to: %s\n"),"bug-coreutils@gnu.org");
  printf(gettext("%s home page: <%s>\n"),"GNU coreutils","https://www.gnu.org/software/coreutils/");
  printf(gettext("General help using GNU software: <%s>\n"),"https://www.gnu.org/gethelp/");
}

// Function: sub_f919 @ 0xf919
void sub_f919(void *a0,unsigned long a1,unsigned long a2)
{
  sub_fa52(a0,a1,a2);
}

// Function: sub_f94a @ 0xf94a
long sub_f94a(long a0)
{
  if (!a0)
    sub_ffa4(); // no-return
  return a0;
}

// Function: sub_f96c @ 0xf96c
void sub_f96c(unsigned long a0)
{
  sub_f94a(malloc(a0));
}

// Function: sub_f992 @ 0xf992
void sub_f992(unsigned long a0)
{
  sub_f94a(sub_10f91(a0));
}

// Function: sub_f9b8 @ 0xf9b8
void sub_f9b8(unsigned long a0)
{
  sub_f96c(a0);
}

// Function: sub_f9d6 @ 0xf9d6
void * sub_f9d6(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = realloc(a0,a1);
  if ((!v1) && ((!a0 || (a1))))
    sub_ffa4(); // no-return
  return v1;
}

// Function: sub_fa21 @ 0xfa21
void sub_fa21(void *a0,unsigned long a1)
{
  sub_f94a(sub_10faf(a0,a1));
}

// Function: sub_fa52 @ 0xfa52
void * sub_fa52(void *a0,unsigned long a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = reallocarray(a0,a1,a2);
  if ((!v1) && ((!a0 || ((a1 && (a2))))))
    sub_ffa4(); // no-return
  return v1;
}

// Function: sub_faac @ 0xfaac
void sub_faac(void *a0,unsigned long a1,unsigned long a2)
{
  sub_f94a(sub_1100f(a0,a1,a2));
}

// Function: sub_fae5 @ 0xfae5
void sub_fae5(unsigned long a0,unsigned long a1)
{
  sub_fa52(NULL,a0,a1);
}

// Function: sub_fb10 @ 0xfb10
void sub_fb10(unsigned long a0,unsigned long a1)
{
  sub_faac(NULL,a0,a1);
}

// Function: sub_fb3b @ 0xfb3b
void sub_fb3b(void *a0,unsigned long *a1)
{
  sub_fb69(a0,a1,1);
}

// Function: sub_fb69 @ 0xfb69
unsigned long sub_fb69(void *a0,unsigned long *a1,unsigned long a2)
{
  char v1 [16];
  long v2; // rax
  unsigned long v3; // rax
  unsigned long v4; // rax
  bool v5; // cf
  unsigned long v6; // stack - 0x18
  
  v6 = *a1;
  if (a0) { // branch-flip
    v3 = (v6 >> 1) + 1;
    v5 = CARRY8(v3,v6);
    v6 = v3 + v6;
    if (v5)
      sub_ffa4(); // no-return
  }
  else if (!v6) {
    v1._8_8_ = 0;
    v1._0_8_ = a2;
    v2 = SUB168((ZEXT816(0) << 0x40 | ZEXT816(0x80)) / v1._0_16_,0);
    v6 = v2 + (unsigned long)(v2 == 0);
  }
  v4 = sub_fa52(a0,v6,a2);
  *a1 = v6;
  return v4;
}

// Function: sub_fc48 @ 0xfc48
unsigned long sub_fc48(void *a0,long *a1,long a2,long a3,long a4) // ternary
{
  long v1;
  long v2; // rax
  unsigned long v3; // rax
  long v4; // stack - 0x30
  unsigned long v5; // stack - 0x28
  
  v1 = *a1;
  v4 = v1 + (v1 >> 1);
  if (SCARRY8(v1,v1 >> 1))
    v4 = 0x7fffffffffffffff;
  if ((0 <= a3) && (a3 < v4))
    v4 = a3;
  v5 = v4 * a4;
  if (SEXT816((long)v5) != SEXT816(v4) * SEXT816(a4)) // branch-flip
    v2 = 0x7fffffffffffffff;
  else {
    v2 = (0x80 <= (long)v5) ? 0 : 0x80; // branch-flip
  }
  if (v2) {
    v4 = v2 / a4;
    v5 = v2 - v2 % a4;
  }
  if (!a0)
    *a1 = 0;
  if ((v4 - v1 < a2) && ((v4 = a2 + v1, SCARRY8(a2,v1) || (((0 <= a3 && (a3 < v4)) || (v5 = v4 * a4, SEXT816((long)v5) != SEXT816(v4) * SEXT816(a4)))))))
    sub_ffa4(); // no-return
  v3 = sub_f9d6(a0,v5);
  *a1 = v4;
  return v3;
}

// Function: sub_fe04 @ 0xfe04
void sub_fe04(unsigned long a0)
{
  sub_fe4a(a0,1);
}

// Function: sub_fe27 @ 0xfe27
void sub_fe27(unsigned long a0)
{
  sub_fe7b(a0,1);
}

// Function: sub_fe4a @ 0xfe4a
void sub_fe4a(unsigned long a0,unsigned long a1)
{
  sub_f94a(calloc(a0,a1));
}

// Function: sub_fe7b @ 0xfe7b
void sub_fe7b(unsigned long a0,unsigned long a1)
{
  sub_f94a(sub_10fe6(a0,a1));
}

// Function: sub_feac @ 0xfeac
void sub_feac(void *a0,unsigned long a1)
{
  memcpy((void *)sub_f96c(a1),a0,a1);
}

// Function: sub_fee4 @ 0xfee4
void sub_fee4(void *a0,unsigned long a1)
{
  memcpy((void *)sub_f992(a1),a0,a1);
}

// Function: sub_ff24 @ 0xff24
void sub_ff24(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)sub_f992(a1 + 1);
  *(char *)((long)v1 + a1) = 0;
  memcpy(v1,a0,a1);
}

// Function: sub_ff73 @ 0xff73
void sub_ff73(char *a0)
{
  sub_feac(a0,strlen(a0) + 1);
}

// Function: sub_ffa4 @ 0xffa4
void sub_ffa4(void)
{
  error(dat_18040,0,"%s",gettext("memory exhausted"));
  abort(); // no-return
}

// Function: sub_ffe4 @ 0xffe4
void sub_ffe4(void)
{
  return;
}

// Function: sub_ffef @ 0xffef
void sub_ffef(unsigned int a0,unsigned int a1) // return-dupe
{
  if (0 <= (int)sub_10883(a0,a1))
    return;
  sub_ffe4();
}

// Function: sub_1001c @ 0x1001c
unsigned long sub_1001c(unsigned char *a0,unsigned int a1,unsigned long a2,unsigned long a3,char *a4,unsigned long a5,int a6)
{
  int v1;
  unsigned long v2; // rax
  unsigned long v3; // stack - 0x28
  int v4;
  int v5; // stack - 0x2c
  
  v5 = sub_10410(a0,NULL,a1,&v3,a4);
  if (v5) { // branch-flip
    if (v5 != 1) { // branch-flip
      if (v5 == 3)
        *__errno_location() = 0;
    }
    else {
      *__errno_location() = 0x4b;
    }
  }
  else if ((v3 < a2) || (a3 < v3)) {
    v5 = 1;
    if (0x40000000 <= v3) // branch-flip
      *__errno_location() = 0x4b;
    else {
      *__errno_location() = 0x22;
    }
  }
  if (v5) {
    v2 = sub_ed11(a0);
    if (*__errno_location() != 0x16) // branch-flip
      v4 = *__errno_location();
    else {
      v4 = 0;
    }
    if (a6) // branch-flip
      v1 = a6;
    else {
      v1 = 1;
    }
    error(v1,v4,"%s: %s",a5,v2);
  }
  return v3;
}

// Function: sub_10156 @ 0x10156
void sub_10156(unsigned char *a0,unsigned long a1,unsigned long a2,char *a3,unsigned long a4,unsigned int a5)
{
  sub_1001c(a0,10,a1,a2,a3,a4,a5);
}

// Function: sub_101af @ 0x101af
unsigned long sub_101af(double a0)
{
  unsigned long v1; // stack - 0x28
  char v2 [16];
  unsigned long v3; // stack - 0x20
  
  if (dat_13cb0 <= a0) {
    do {
      pause();
    } while (*__errno_location() == 4);
  }
  v2 = sub_10c19();
  v3 = SUB168(v2,8);
  v1 = SUB168(v2,0);
  do {
    *__errno_location() = 0;
    if (!sub_11150(&v1,&v1))
      return 0;
  } while ((*__errno_location() == 4) || (!*__errno_location()));
  return 0xffffffff;
}

// Function: sub_10261 @ 0x10261
char sub_10261(char *a0,unsigned long *a1,double *a2,void *a3)
{
  char *v1; // stack - 0x20
  double v2; // xmm0_qa
  char v3; // stack - 0x21
  double v4; // stack - 0x18
  
  v3 = 1;
  *__errno_location() = 0;
  (*a3)(a0,&v1);
  v4 = v2;
  if ((a0 != v1) && ((a1 || (!*v1)))) {
    if ((v2 == 0.0) || (*__errno_location() != 0x22)) goto label_10300;
    v3 = 0;
  }
  else {
    v3 = 0;
  }
label_10300:
  if (a1)
    *a1 = v1;
  *a2 = v4;
  return v3;
}

// Function: sub_10339 @ 0x10339
bool sub_10339(unsigned long *a0,int a1)
{
  char v1 [16];
  bool v2;
  
  v2 = 0;
  if (((long)a1 <= -1) && (*a0))
    v2 = 1;
  v1 = ZEXT816((unsigned long)(long)a1) * ZEXT816(*a0);
  if (SUB168(v1,8))
    v2 = 1;
  if (!v2)
    *a0 = SUB168(v1,0);
  else {
    *a0 = 0xffffffffffffffff;
  }
  return v2;
}

// Function: sub_103cb @ 0x103cb
unsigned int sub_103cb(unsigned long *a0,unsigned int a1,int a2)
{
  int v1;
  int v2; // stack - 0x28
  unsigned int v3; // stack - 0xc
  
  v3 = 0;
  v2 = a2;
  while (v1 = v2 + -1, v2) {
    v3 |= sub_10339(a0,a1);
    v2 = v1;
  }
  return v3;
}

// Function: sub_10410 @ 0x10410
unsigned int sub_10410(unsigned char *a0,long *a1,int a2,unsigned long *a3,char *a4) // early-return x2
{
  char v1;
  long *v10; // stack - 0x18
  long v2;
  unsigned long v3; // stack - 0x28
  unsigned char v4; // stack - 0x41
  unsigned int v5; // stack - 0x40
  unsigned int v6; // stack - 0x3c
  int v7; // stack - 0x38
  unsigned int v8; // stack - 0x34
  unsigned char *v9; // stack - 0x20
  
  v5 = 0;
  if ((a2 < 0) || (0x25 <= a2))
    __assert_fail("0 <= strtol_base && strtol_base <= 36","lib/xstrtol.c",0x55,"xstrtoumax"); // no-return
  if (!a1)
    a1 = &v2;
  v10 = a1;
  *__errno_location() = 0;
  v4 = *a0;
  v9 = a0;
  while (*(unsigned short *)((unsigned long)v4 * 2 + *(long *)__ctype_b_loc()) & 0x2000) {
    v9 = &v9[1];
    v4 = *v9;
  }
  if (v4 == 0x2d)
    return 4;
  v3 = strtoumax(a0,v10,a2);
  if (a0 != (unsigned char *)*v10) { // branch-flip
    if (*__errno_location()) {
      if (*__errno_location() != 0x22)
        return 4;
      v5 = 1;
    }
  }
  else {
    if (((!a4) || (!*(char *)*v10)) || (v1 = *(char *)*v10, !strchr(a4,(int)v1)))
      return 4;
    v3 = 1;
  }
  if (!a4) {
    *a3 = v3;
    return v5;
  }
  if (*(char *)*v10) {
    v6 = 0x400;
    v7 = 1;
    v1 = *(char *)*v10;
    if (!strchr(a4,(int)v1)) {
      *a3 = v3;
      return v5 | 2;
    }
    if ((((unsigned int)((int)*(char *)*v10 - 0x45U) <= 0x2f) && (0x814400308945U >> ((unsigned char)((int)*(char *)*v10 - 0x45U) & 0x3f) & 1)) && (strchr(a4,0x30))) {
      v1 = *(char *)(*v10 + 1);
      if (v1 != 'i') { // branch-flip
        if ((v1 <= 'i') && ((v1 == 'B' || (v1 == 'D')))) {
          v6 = 1000;
          v7 = 2;
        }
      }
      else if (*(char *)(*v10 + 2) == 'B')
        v7 = 3;
    }
    switch(*(char *)*v10) {
      case 0x42:
        v8 = sub_10339(&v3,0x400);
        break;
      default:
        *a3 = v3;
        return v5 | 2;
      case 0x45:
        v8 = sub_103cb(&v3,v6,6);
        break;
      case 0x47:
      case 0x67:
        v8 = sub_103cb(&v3,v6,3);
        break;
      case 0x4b:
      case 0x6b:
        v8 = sub_103cb(&v3,v6,1);
        break;
      case 0x4d:
      case 0x6d:
        v8 = sub_103cb(&v3,v6,2);
        break;
      case 0x50:
        v8 = sub_103cb(&v3,v6,5);
        break;
      case 0x54:
      case 0x74:
        v8 = sub_103cb(&v3,v6,4);
        break;
      case 0x59:
        v8 = sub_103cb(&v3,v6,8);
        break;
      case 0x5a:
        v8 = sub_103cb(&v3,v6,7);
        break;
      case 0x62:
        v8 = sub_10339(&v3,0x200);
        break;
      case 99:
        v8 = 0;
        break;
      case 0x77:
        v8 = sub_10339(&v3,2);
      
    }
    v5 |= v8;
    *v10 = *v10 + (long)v7;
    if (*(char *)*v10)
      v5 |= 2;
  }
  *a3 = v3;
  return v5;
}

// Function: sub_1086e @ 0x1086e
unsigned long sub_1086e(void)
{
  return 0;
}

// Function: sub_10883 @ 0x10883
void sub_10883(unsigned int a0,unsigned int a1)
{
  sub_1086e(a0,a1);
}

// Function: sub_108a6 @ 0x108a6
unsigned long sub_108a6(unsigned long a0,unsigned char a1)
{
  return a0 << (a1 & 0x3f) | a0 >> 0x40 - (a1 & 0x3f);
}

// Function: sub_108c6 @ 0x108c6
unsigned long sub_108c6(unsigned long a0,unsigned char a1)
{
  return a0 >> (a1 & 0x3f) | a0 << 0x40 - (a1 & 0x3f);
}

// Function: sub_108e6 @ 0x108e6
unsigned int sub_108e6(unsigned int a0,unsigned char a1)
{
  return a0 << (a1 & 0x1f) | a0 >> 0x20 - (a1 & 0x1f);
}

// Function: sub_10902 @ 0x10902
unsigned int sub_10902(unsigned int a0,unsigned char a1)
{
  return a0 >> (a1 & 0x1f) | a0 << 0x20 - (a1 & 0x1f);
}

// Function: sub_1091e @ 0x1091e
unsigned long sub_1091e(unsigned long a0,unsigned char a1)
{
  return a0 >> (0x40 - a1 & 0x3f) | a0 << (a1 & 0x3f);
}

// Function: sub_10957 @ 0x10957
unsigned long sub_10957(unsigned long a0,unsigned char a1)
{
  return a0 << (0x40 - a1 & 0x3f) | a0 >> (a1 & 0x3f);
}

// Function: sub_10990 @ 0x10990
unsigned int sub_10990(unsigned short a0,unsigned char a1)
{
  return (unsigned int)(a0 >> (0x10 - a1 & 0x1f)) | (unsigned int)a0 << (a1 & 0x1f);
}

// Function: sub_109c6 @ 0x109c6
unsigned int sub_109c6(unsigned short a0,unsigned char a1)
{
  return (unsigned int)a0 << (0x10 - a1 & 0x1f) | (unsigned int)(a0 >> (a1 & 0x1f));
}

// Function: sub_109fc @ 0x109fc
unsigned int sub_109fc(unsigned char a0,unsigned char a1)
{
  return (unsigned int)(a0 >> (8 - a1 & 0x1f)) | (unsigned int)a0 << (a1 & 0x1f);
}

// Function: sub_10a31 @ 0x10a31
unsigned int sub_10a31(unsigned char a0,unsigned char a1)
{
  return (unsigned int)a0 << (8 - a1 & 0x1f) | (unsigned int)(a0 >> (a1 & 0x1f));
}

// Function: sub_10a66 @ 0x10a66
int sub_10a66(char *a0,char *a1) // early-return
{
  unsigned char v1; // al
  unsigned char v2; // al
  
  if (a0 == a1)
    return 0;
  do {
    v1 = sub_114d9((unsigned char)*a0);
    v2 = sub_114d9((unsigned char)*a1);
    if (!v1) break;
    a0 = &a0[1];
    a1 = &a1[1];
  } while (v1 == v2);
  return (unsigned int)v1 - (unsigned int)v2;
}

// Function: sub_10ae0 @ 0x10ae0
long sub_10ae0(void)
{
  if (!dat_182a0)
    dat_182a0 = newlocale(0x1fbf,"C",0);
  return dat_182a0;
}

// Function: sub_10b1d @ 0x10b1d
unsigned long sub_10b1d(char *a0,char **a1)
{
  long v1; // rax
  unsigned long v2;
  unsigned long v3; // xmm0_qa
  
  v1 = sub_10ae0();
  if (v1) { // branch-flip
    strtod_l(a0,a1,v1);
    v2 = v3;
  }
  else {
    if (a1)
      *a1 = a0;
    v2 = 0;
  }
  return v2;
}

// Function: sub_10b8a @ 0x10b8a
unsigned long sub_10b8a(FILE *a0)
{
  int v1; // eax
  unsigned long v2; // rax
  bool v3; // zf
  
  v2 = __fpending(a0);
  v1 = ferror_unlocked(a0);
  v3 = sub_10d79(a0) == 0;
  if ((!v1) && ((v3 || ((!v2 && (*__errno_location() == 9))))))
    return 0;
  if (v3)
    *__errno_location() = 0;
  return 0xffffffff;
}

// Function: sub_10c19 @ 0x10c19
void sub_10c19(double a0) // return-dupe x2
{
  double v1; // xmm0_qa
  long v2; // stack - 0x20
  long v3; // stack - 0x18
  
  if (a0 <= dat_13dd8) {
    sub_ee85(0x8000000000000000,0);
    return;
  }
  if (dat_13de0 <= a0) {
    sub_ee85(0x7fffffffffffffff,999999999);
    return;
  }
  v1 = dat_13de8 * (a0 - (double)(long)a0);
  v3 = (long)v1;
  v3 += (unsigned long)((double)v3 < v1);
  v2 = (long)a0 + v3 / 1000000000;
  v3 %= 1000000000;
  if (v3 <= -1) {
    v2 -= 1;
    v3 += 1000000000;
  }
  sub_ee85(v2,v3);
}

// Function: sub_10d79 @ 0x10d79
unsigned long sub_10d79(FILE *a0)
{
  int v1; // eax
  int v2; // stack - 0x14
  unsigned int v3; // stack - 0x10
  
  v2 = 0;
  if (fileno(a0) <= -1)
    return CONCAT44(dat_4,fclose(a0));
  if (__freading(a0)) {
    v1 = fileno(a0);
    if (lseek(v1,0,1) == -1) goto label_10e07;
  }
  if (sub_10e6a(a0))
    v2 = *__errno_location();
label_10e07:
  v3 = fclose(a0);
  if (v2) {
    *__errno_location() = v2;
    v3 = 0xffffffff;
  }
  return (unsigned long)v3;
}

// Function: sub_10e32 @ 0x10e32
void sub_10e32(FILE *a0) // return-dupe
{
  if (!(*(unsigned int *)a0 & 0x100))
    return;
  sub_10eb9(a0,0,1);
}

// Function: sub_10e6a @ 0x10e6a
void sub_10e6a(FILE *a0)
{
  if ((a0) && (__freading(a0))) {
    sub_10e32(a0);
    fflush(a0);
    return;
  }
  fflush(a0);
}

// Function: sub_10eb9 @ 0x10eb9
unsigned long sub_10eb9(FILE *a0,long a1,int a2) // early-return x2
{
  long v1; // rax
  
  if (((*(long *)&a0->field_0x10 == *(long *)&a0->field_0x8) && (*(long *)&a0->field_0x28 == *(long *)&a0->field_0x20)) && (!*(long *)&a0->field_0x48)) {
    v1 = lseek(fileno(a0),a1,a2);
    if (v1 == -1)
      return 0xffffffff;
    *(unsigned int *)a0 = *(unsigned int *)a0 & 0xffffffef;
    *(long *)&a0->field_0x90 = v1;
    return 0;
  }
  return CONCAT44(dat_4,fseeko(a0,a1,a2));
}

// Function: sub_10f77 @ 0x10f77
unsigned long sub_10f77(void)
{
  *__errno_location() = 0xc;
  return 0;
}

// Function: sub_10f91 @ 0x10f91
void sub_10f91(unsigned long a0)
{
  malloc(a0);
}

// Function: sub_10faf @ 0x10faf
void sub_10faf(void *a0,unsigned long a1)
{
  realloc(a0,a1 == 0 | a1);
}

// Function: sub_10fe6 @ 0x10fe6
void sub_10fe6(unsigned long a0,unsigned long a1)
{
  calloc(a0,a1);
}

// Function: sub_1100f @ 0x1100f
void sub_1100f(void *a0,unsigned long a1,unsigned long a2)
{
  unsigned long v1; // stack - 0x20
  unsigned long v2; // stack - 0x18
  
  if ((!a1) || (v1 = a2, v2 = a1, !a2)) {
    v1 = 1;
    v2 = 1;
  }
  reallocarray(a0,v2,v1);
}

// Function: sub_1105e @ 0x1105e
char * sub_1105e(void)
{
  char *v1; // stack - 0x10
  
  v1 = nl_langinfo(0xe);
  if (!v1)
    v1 = "";
  if (!*v1)
    v1 = "ASCII";
  return v1;
}

// Function: sub_110a6 @ 0x110a6
unsigned long sub_110a6(unsigned int *a0,char *a1,unsigned long a2,mbstate_t *a3) // early-return
{
  long v1;
  unsigned int *v2; // stack - 0x30
  unsigned long v3; // stack - 0x18
  
  v2 = a0;
  if (!a0)
    v2 = &v1;
  v3 = mbrtowc(v2,a1,a2,a3);
  if (((0xfffffffffffffffe <= v3) && (a2)) && (sub_1151f(0) != '\x01')) {
    *v2 = (unsigned int)(unsigned char)*a1;
    return 1;
  }
  return v3;
}

// Function: sub_11150 @ 0x11150
unsigned long sub_11150(struct_3 *a0,timespec *a1) // early-return
{
  unsigned int v1; // eax
  timespec v2; // stack - 0x28
  long v3; // stack - 0x38
  
  if ((0 <= a0->field_0x8) && (a0->field_0x8 <= 999999999)) {
    v3 = a0->field_0x0;
    v2._8_8_ = a0->field_0x8;
    while( true ) {
      if (v3 <= 0x1fa400) {
        v2._0_8_ = v3;
        return CONCAT44(dat_4,nanosleep(&v2,a1));
      }
      v2._0_8_ = 0x1fa400;
      v1 = nanosleep(&v2,a1);
      v3 -= 0x1fa400;
      if (v1) break;
      v2._8_8_ = 0;
    }
    if (a1)
      *(long *)a1 = *(long *)a1 + v3;
    return (unsigned long)v1;
  }
  *__errno_location() = 0x16;
  return 0xffffffff;
}

// Function: sub_11256 @ 0x11256
void sub_11256(unsigned int a0)
{
  sub_aa63(a0,0,3);
}

// Function: sub_11280 @ 0x11280
unsigned long sub_11280(int a0) // return-dupe
{
  if (0x7a < a0)
    return 0;
  if (0x61 <= a0)
    return 1;
  if (0x39 < a0) {
    if (0x1a <= (unsigned int)(a0 - 0x41U))
      return 0;
    return 1;
  }
  if (a0 <= 0x2f)
    return 0;
  return 1;
}

// Function: sub_112be @ 0x112be
unsigned long sub_112be(int a0) // return-dupe
{
  if (0x5a < a0) {
    if (0x1a <= (unsigned int)(a0 - 0x61U))
      return 0;
    return 1;
  }
  if (a0 <= 0x40)
    return 0;
  return 1;
}

// Function: sub_112f0 @ 0x112f0
bool sub_112f0(unsigned int a0)
{
  return a0 < 0x80;
}

// Function: sub_11311 @ 0x11311
unsigned int sub_11311(int a0) // return-dupe
{
  if ((a0 != 0x20) && (a0 != 9))
    return 0;
  return 1;
}

// Function: sub_11339 @ 0x11339
unsigned long sub_11339(int a0) // return-dupe
{
  if (0x1f < a0) {
    if (a0 != 0x7f)
      return 0;
    return 1;
  }
  if (a0 <= -1)
    return 0;
  return 1;
}

// Function: sub_11366 @ 0x11366
bool sub_11366(int a0)
{
  return (unsigned int)(a0 - 0x30U) < 10;
}

// Function: sub_1138a @ 0x1138a
bool sub_1138a(int a0)
{
  return (unsigned int)(a0 - 0x21U) < 0x5e;
}

// Function: sub_113ae @ 0x113ae
bool sub_113ae(int a0)
{
  return (unsigned int)(a0 - 0x61U) < 0x1a;
}

// Function: sub_113d2 @ 0x113d2
bool sub_113d2(int a0)
{
  return (unsigned int)(a0 - 0x20U) < 0x5f;
}

// Function: sub_113f6 @ 0x113f6
unsigned long sub_113f6(int a0) // return-dupe, return-dupe x3
{
  if (0x7f <= a0)
    return 0;
  if (0x7b <= a0)
    return 1;
  if (0x61 <= a0)
    return 0;
  if (a0 <= 0x5a) {
    if (0x2f < a0) {
      if (7 <= (unsigned int)(a0 - 0x3aU))
        return 0;
      return 1;
    }
    if (a0 <= 0x20)
      return 0;
    return 1;
  }
  return 1;
}

// Function: sub_11440 @ 0x11440
unsigned long sub_11440(int a0) // return-dupe
{
  if (0xd < a0) {
    if (a0 != 0x20)
      return 0;
    return 1;
  }
  if (a0 <= 8)
    return 0;
  return 1;
}

// Function: sub_1146d @ 0x1146d
bool sub_1146d(int a0)
{
  return (unsigned int)(a0 - 0x41U) < 0x1a;
}

// Function: sub_11491 @ 0x11491
unsigned long sub_11491(int a0) // return-dupe
{
  if (((unsigned int)(a0 - 0x30U) <= 0x36) && (0x7e0000007e03ffU >> ((unsigned char)(a0 - 0x30U) & 0x3f) & 1))
    return 1;
  return 0;
}

// Function: sub_114d9 @ 0x114d9
int sub_114d9(int a0)
{
  if ((unsigned int)(a0 - 0x41U) <= 0x19)
    a0 += 0x20;
  return a0;
}

// Function: sub_114fc @ 0x114fc
int sub_114fc(int a0)
{
  if ((unsigned int)(a0 - 0x61U) <= 0x19)
    a0 -= 0x20;
  return a0;
}

// Function: sub_1151f @ 0x1151f
unsigned int sub_1151f(unsigned int a0) // return-dupe
{
  char v1 [264];
  
  if (sub_116a9(a0,v1,0x101))
    return 0;
  if ((strcmp(v1,"C")) && (strcmp(v1,"POSIX")))
    return 1;
  return 0;
}

// Function: sub_115c9 @ 0x115c9
char * sub_115c9(int a0)
{
  return setlocale(a0,NULL);
}

// Function: sub_115f1 @ 0x115f1
unsigned long sub_115f1(unsigned int a0,char *a1,unsigned long a2) // return-dupe x2
{
  char *v1; // rax
  unsigned long v2; // rax
  
  v1 = (char *)sub_115c9(a0);
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

// Function: sub_116a9 @ 0x116a9
void sub_116a9(unsigned int a0,char *a1,unsigned long a2)
{
  sub_115f1(a0,a1,a2);
}

// Function: sub_116d7 @ 0x116d7
void sub_116d7(unsigned int a0)
{
  sub_115c9(a0);
}

// Function: sub_11700 @ 0x11700
void sub_11700(unsigned long a0)
{
  __cxa_atexit(a0,0,dat_18008); // tail-call
}

// Function: _DT_FINI @ 0x11714
void _DT_FINI(void)
{
  return;
}
