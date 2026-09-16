// Function: _init @ 0xdec
void _init(void) // injected, return-dupe
{
  if (!dat_9fd4)
    return;
  __gmon_start__();
}

// Function: sub_e10 @ 0xe10
void sub_e10(void)
{
  int v1; // ebx
  
  (**(void **)(v1 + 8))(); // jump-as-call
}

// Function: strcmp @ 0xe20
int strcmp(char *a0,char *a1)
{
  void *v1;
  int v2; // ebx
  
  v1 = *(void **)(v2 + 0xc); // jump-as-call
  return (*v1)();
}

// Function: fflush @ 0xe30
int fflush(void *a0)
{
  void *v1;
  int v2; // ebx
  
  v1 = *(void **)(v2 + 0x10); // jump-as-call
  return (*v1)();
}

// Function: _exit @ 0xe40
void _exit(int a0)
{
  int v1; // ebx
  
  (**(void **)(v1 + 0x14))(); // jump-as-call
}

// Function: free @ 0xe50
void free(void *a0)
{
  int v1; // ebx
  
  (**(void **)(v1 + 0x18))(); // jump-as-call
}

// Function: memcpy @ 0xe60
void * memcpy(void *a0,void *a1,unsigned int a2)
{
  void *v1;
  int v2; // ebx
  
  v1 = *(void **)(v2 + 0x1c); // jump-as-call
  return (void *)(*v1)();
}

// Function: mbsinit @ 0xe70
int mbsinit(void *a0)
{
  void *v1;
  int v2; // ebx
  
  v1 = *(void **)(v2 + 0x20); // jump-as-call
  return (*v1)();
}

// Function: fclose @ 0xe80
int fclose(void *a0)
{
  void *v1;
  int v2; // ebx
  
  v1 = *(void **)(v2 + 0x24); // jump-as-call
  return (*v1)();
}

// Function: fseeko64 @ 0xe90
void fseeko64(void)
{
  int v1; // ebx
  
  (**(void **)(v1 + 0x28))(); // jump-as-call
}

// Function: memcmp @ 0xea0
int memcmp(void *a0,void *a1,unsigned int a2)
{
  void *v1;
  int v2; // ebx
  
  v1 = *(void **)(v2 + 0x2c); // jump-as-call
  return (*v1)();
}

// Function: dcgettext @ 0xeb0
char * dcgettext(char *a0,char *a1,int a2)
{
  void *v1;
  int v2; // ebx
  
  v1 = *(void **)(v2 + 0x30); // jump-as-call
  return (char *)(*v1)();
}

// Function: stpcpy @ 0xec0
char * stpcpy(char *a0,char *a1)
{
  void *v1;
  int v2; // ebx
  
  v1 = *(void **)(v2 + 0x34); // jump-as-call
  return (char *)(*v1)();
}

// Function: __stack_chk_fail @ 0xed0
void __stack_chk_fail(void)
{
  int v1; // ebx
  
  (**(void **)(v1 + 0x38))(); // jump-as-call
}

// Function: textdomain @ 0xee0
char * textdomain(char *a0)
{
  void *v1;
  int v2; // ebx
  
  v1 = *(void **)(v2 + 0x3c); // jump-as-call
  return (char *)(*v1)();
}

// Function: iswprint @ 0xef0
int iswprint(unsigned int a0)
{
  void *v1;
  int v2; // ebx
  
  v1 = *(void **)(v2 + 0x40); // jump-as-call
  return (*v1)();
}

// Function: fopen64 @ 0xf00
void fopen64(void)
{
  int v1; // ebx
  
  (**(void **)(v1 + 0x44))(); // jump-as-call
}

// Function: fwrite @ 0xf10
unsigned int fwrite(void *a0,unsigned int a1,unsigned int a2,void *a3)
{
  void *v1;
  int v2; // ebx
  
  v1 = *(void **)(v2 + 0x48); // jump-as-call
  return (*v1)();
}

// Function: __ctype_get_mb_cur_max @ 0xf20
unsigned int __ctype_get_mb_cur_max(void)
{
  void *v1;
  int v2; // ebx
  
  v1 = *(void **)(v2 + 0x4c); // jump-as-call
  return (*v1)();
}

// Function: strcpy @ 0xf30
char * strcpy(char *a0,char *a1)
{
  void *v1;
  int v2; // ebx
  
  v1 = *(void **)(v2 + 0x50); // jump-as-call
  return (char *)(*v1)();
}

// Function: __fpending @ 0xf40
unsigned int __fpending(void *a0)
{
  void *v1;
  int v2; // ebx
  
  v1 = *(void **)(v2 + 0x54); // jump-as-call
  return (*v1)();
}

// Function: mbrtowc @ 0xf50
unsigned int mbrtowc(void *a0,char *a1,unsigned int a2,void *a3)
{
  void *v1;
  int v2; // ebx
  
  v1 = *(void **)(v2 + 0x58); // jump-as-call
  return (*v1)();
}

// Function: __cxa_atexit @ 0xf60
void __cxa_atexit(void)
{
  int v1; // ebx
  
  (**(void **)(v1 + 0x5c))(); // jump-as-call
}

// Function: error @ 0xf70
void error(int a0,int a1,char *a2,...)
{
  int v1; // ebx
  
  (**(void **)(v1 + 0x60))(); // jump-as-call
}

// Function: realloc @ 0xf80
void * realloc(void *a0,unsigned int a1)
{
  void *v1;
  int v2; // ebx
  
  v1 = *(void **)(v2 + 100); // jump-as-call
  return (void *)(*v1)();
}

// Function: malloc @ 0xf90
void * malloc(unsigned int a0)
{
  void *v1;
  int v2; // ebx
  
  v1 = *(void **)(v2 + 0x68); // jump-as-call
  return (void *)(*v1)();
}

// Function: __freading @ 0xfa0
int __freading(void *a0)
{
  void *v1;
  int v2; // ebx
  
  v1 = *(void **)(v2 + 0x6c); // jump-as-call
  return (*v1)();
}

// Function: exit @ 0xfb0
void exit(int a0)
{
  int v1; // ebx
  
  (**(void **)(v1 + 0x70))(); // jump-as-call
}

// Function: fputs_unlocked @ 0xfc0
int fputs_unlocked(char *a0,void *a1)
{
  void *v1;
  int v2; // ebx
  
  v1 = *(void **)(v2 + 0x74); // jump-as-call
  return (*v1)();
}

// Function: getopt_long @ 0xfd0
int getopt_long(int a0,char **a1,char *a2,void *a3,int *a4)
{
  void *v1;
  int v2; // ebx
  
  v1 = *(void **)(v2 + 0x78); // jump-as-call
  return (*v1)();
}

// Function: strchr @ 0xfe0
char * strchr(char *a0,int a1)
{
  void *v1;
  int v2; // ebx
  
  v1 = *(void **)(v2 + 0x7c); // jump-as-call
  return (char *)(*v1)();
}

// Function: strlen @ 0xff0
unsigned int strlen(char *a0)
{
  void *v1;
  int v2; // ebx
  
  v1 = *(void **)(v2 + 0x80); // jump-as-call
  return (*v1)();
}

// Function: __libc_start_main @ 0x1000
void __libc_start_main(void)
{
  int v1; // ebx
  
  (**(void **)(v1 + 0x84))(); // jump-as-call
}

// Function: re_search @ 0x1010
void re_search(void)
{
  int v1; // ebx
  
  (**(void **)(v1 + 0x88))(); // jump-as-call
}

// Function: memset @ 0x1020
void * memset(void *a0,int a1,unsigned int a2)
{
  void *v1;
  int v2; // ebx
  
  v1 = *(void **)(v2 + 0x8c); // jump-as-call
  return (void *)(*v1)();
}

// Function: __errno_location @ 0x1030
int * __errno_location(void)
{
  void *v1;
  int v2; // ebx
  
  v1 = *(void **)(v2 + 0x90); // jump-as-call
  return (int *)(*v1)();
}

// Function: fileno @ 0x1040
int fileno(void *a0)
{
  void *v1;
  int v2; // ebx
  
  v1 = *(void **)(v2 + 0x94); // jump-as-call
  return (*v1)();
}

// Function: fwrite_unlocked @ 0x1050
unsigned int fwrite_unlocked(void *a0,unsigned int a1,unsigned int a2,void *a3)
{
  void *v1;
  int v2; // ebx
  
  v1 = *(void **)(v2 + 0x98); // jump-as-call
  return (*v1)();
}

// Function: __printf_chk @ 0x1060
int __printf_chk(int a0,char *a1,...)
{
  void *v1;
  int v2; // ebx
  
  v1 = *(void **)(v2 + 0x9c); // jump-as-call
  return (*v1)();
}

// Function: __uflow @ 0x1070
void __uflow(void)
{
  int v1; // ebx
  
  (**(void **)(v1 + 0xa0))(); // jump-as-call
}

// Function: setlocale @ 0x1080
char * setlocale(int a0,char *a1)
{
  void *v1;
  int v2; // ebx
  
  v1 = *(void **)(v2 + 0xa4); // jump-as-call
  return (char *)(*v1)();
}

// Function: clearerr_unlocked @ 0x1090
void clearerr_unlocked(void *a0)
{
  int v1; // ebx
  
  (**(void **)(v1 + 0xa8))(); // jump-as-call
}

// Function: strrchr @ 0x10a0
char * strrchr(char *a0,int a1)
{
  void *v1;
  int v2; // ebx
  
  v1 = *(void **)(v2 + 0xac); // jump-as-call
  return (char *)(*v1)();
}

// Function: __overflow @ 0x10b0
int __overflow(void *a0,int a1)
{
  void *v1;
  int v2; // ebx
  
  v1 = *(void **)(v2 + 0xb0); // jump-as-call
  return (*v1)();
}

// Function: lseek64 @ 0x10c0
void lseek64(void)
{
  int v1; // ebx
  
  (**(void **)(v1 + 0xb4))(); // jump-as-call
}

// Function: __strtoll_internal @ 0x10d0
void __strtoll_internal(void)
{
  int v1; // ebx
  
  (**(void **)(v1 + 0xb8))(); // jump-as-call
}

// Function: re_compile_pattern @ 0x10e0
void re_compile_pattern(void)
{
  int v1; // ebx
  
  (**(void **)(v1 + 0xbc))(); // jump-as-call
}

// Function: __fprintf_chk @ 0x10f0
int __fprintf_chk(void *a0,int a1,char *a2,...)
{
  void *v1;
  int v2; // ebx
  
  v1 = *(void **)(v2 + 0xc0); // jump-as-call
  return (*v1)();
}

// Function: bindtextdomain @ 0x1100
char * bindtextdomain(char *a0,char *a1)
{
  void *v1;
  int v2; // ebx
  
  v1 = *(void **)(v2 + 0xc4); // jump-as-call
  return (char *)(*v1)();
}

// Function: posix_fadvise64 @ 0x1110
void posix_fadvise64(void)
{
  int v1; // ebx
  
  (**(void **)(v1 + 200))(); // jump-as-call
}

// Function: strncmp @ 0x1120
int strncmp(char *a0,char *a1,unsigned int a2)
{
  void *v1;
  int v2; // ebx
  
  v1 = *(void **)(v2 + 0xcc); // jump-as-call
  return (*v1)();
}

// Function: abort @ 0x1130
void abort(void)
{
  int v1; // ebx
  
  (**(void **)(v1 + 0xd0))(); // jump-as-call
}

// Function: strtol @ 0x1140
int strtol(char *a0,char **a1,int a2)
{
  void *v1;
  int v2; // ebx
  
  v1 = *(void **)(v2 + 0xd4); // jump-as-call
  return (*v1)();
}

// Function: __assert_fail @ 0x1150
void __assert_fail(char *a0,char *a1,unsigned int a2,char *a3)
{
  int v1; // ebx
  
  (**(void **)(v1 + 0xd8))(); // jump-as-call
}

// Function: __ctype_b_loc @ 0x1160
void * __ctype_b_loc(void)
{
  void *v1;
  int v2; // ebx
  
  v1 = *(void **)(v2 + 0xdc); // jump-as-call
  return (void *)(*v1)();
}

// Function: calloc @ 0x1170
void * calloc(unsigned int a0,unsigned int a1)
{
  void *v1;
  int v2; // ebx
  
  v1 = *(void **)(v2 + 0xe0); // jump-as-call
  return (void *)(*v1)();
}

// Function: __cxa_finalize @ 0x1180
void __cxa_finalize(void)
{
  int v1; // ebx
  
  (**(void **)(v1 + 0xf0))(); // jump-as-call
}

// Function: __gmon_start__ @ 0x1188
void __gmon_start__(void)
{
  int v1; // ebx
  
  (**(void **)(v1 + 0xf8))(); // jump-as-call
}

// Function: main @ 0x1190
void main(int a0,char **a1) // injected
{
  char v1;
  char *v10;
  char *v11;
  void *v12; // eax
  unsigned int v13;
  unsigned int v14; // edx
  int v15; // stack - 0x28
  int v16; // gs_offset
  unsigned int v17; // stack - 0x3c
  unsigned int v18; // stack - 0x24
  unsigned int v19; // stack - 0x1c
  unsigned int v2;
  int *v20; // stack - 0x18
  int *v3;
  unsigned int *v4;
  unsigned int *v5;
  int v6;
  char **v7;
  unsigned char v8; // al
  int v9;
  
  v7 = a1;
  v6 = a0;
  v20 = &a0;
  v19 = 0x11a6;
  v18 = *(unsigned int *)(v16 + 0x14);
  set_program_name(*a1);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  atexit(close_stdout);
  v4 = dat_9ff8;
  have_read_stdin = '\0';
  v17._0_1_ = 1;
label_1230:
  v9 = getopt_long(v6,v7,"h:b:f:v:i:pl:s:w:n:d:",longopts,NULL);
  v5 = dat_9ff8;
  if (v9 != -1) {
    switch(v9) {
      default:
        goto label_127f;
      case 0x62:
        if (build_type_arg()) goto label_1230;
        v2 = *dat_9ff8;
        v17 = quote(v2);
        v10 = "invalid body numbering style: %s";
        break;
      case 100:
        section_del = (char *)*dat_9ff8;
        goto label_1230;
      case 0x66:
        if (build_type_arg()) goto label_1230;
        v2 = *dat_9ff8;
        v17 = quote(v2);
        v10 = "invalid footer numbering style: %s";
        break;
      case 0x68:
        if (build_type_arg()) goto label_1230;
        v2 = *dat_9ff8;
        v17 = quote(v2);
        v10 = "invalid header numbering style: %s";
        break;
      case 0x69:
        goto label_1371;
      case 0x6c:
        v2 = *dat_9ff8;
        if ((!xstrtoimax(v2,0,10,&blank_join,0x5cd5)) && ((0 < blank_join._4_4_ || ((0 <= blank_join._4_4_ && ((int)blank_join)))))) goto label_1230;
        v2 = *v5;
        v17 = quote(v2);
        v10 = "invalid number of blank lines: %s";
        break;
      case 0x6e:
        v10 = (char *)*dat_9ff8;
        if (!strcmp(v10,"ln")) {
          lineno_format = "%-*lld%s";
          goto label_1230;
        }
        v9 = strcmp(v10,"rn");
        if (!v9) {
          lineno_format = "%*lld%s";
          goto label_1230;
        }
        if (!strcmp(v10,"rz")) {
          lineno_format = "%0*lld%s";
          goto label_1230;
        }
        v17 = quote(v10,0x5bce,v9,v9);
        v10 = "invalid line numbering format: %s";
        break;
      case 0x70:
        reset_numbers = 0;
        goto label_1230;
      case 0x73:
        separator_str = (char *)*v4;
        goto label_1230;
      case 0x76:
        v2 = *dat_9ff8;
        if (!xstrtoimax(v2,0,10,&starting_line_number,0x5cd5)) goto label_1230;
        v2 = *v5;
        v17 = quote(v2);
        v10 = "invalid starting line number: %s";
        break;
      case 0x77:
        v2 = *dat_9ff8;
        if ((!xstrtol(v2,0,10,&v15,0x5cd5)) && (1 <= v15)) {
          lineno_width = v15;
          goto label_1230;
        }
        v2 = *v5;
        v17 = quote(v2);
        v10 = "invalid line number field width: %s";
        break;
      case 0x80:
        error(0,0,dcgettext(NULL,"WARNING: --page-increment is deprecated; use --line-increment instead",5),v9);
label_1371:
        v2 = *v4;
        if ((!xstrtoimax(v2,0,10,&page_incr,0x5cd5)) && ((0 < page_incr._4_4_ || ((0 <= page_incr._4_4_ && ((int)page_incr)))))) goto label_1230;
        v2 = *v4;
        v17 = quote(v2);
        v10 = "invalid line number increment: %s";
        break;
      case -0x83:
        version_etc(*dat_9fdc,0x5bef,"GNU coreutils",Version,"Scott Bartram","David MacKenzie",0,v9);
        exit(0); // no-return
      case -0x82:
        usage(0); // no-return
      
    }
    error(0,0,dcgettext(NULL,v10,5),v17);
    v17._0_1_ = 0;
    goto label_1230;
  }
  if (!(unsigned char)v17)
    usage(1); // no-return
  v13 = 0xffffffff;
  v10 = section_del;
  do {
    if (!v13) break;
    v13 -= 1;
    v11 = &v10[1];
    v1 = *v10;
    v10 = v11;
  } while (v1);
  v9 = ~v13 - 1;
  header_del_len = v9 * 3;
  header_del = (char *)xmalloc(header_del_len + 1);
  v10 = section_del;
  v11 = stpcpy(header_del,section_del);
  strcpy(stpcpy(v11,v10),v10);
  body_del_len = v9 * 2;
  body_del = (char *)xmalloc(body_del_len + 1);
  v10 = section_del;
  v11 = stpcpy(body_del,section_del);
  strcpy(v11,v10);
  footer_del_len = v9;
  footer_del = (char *)xmalloc(~v13);
  strcpy(footer_del,section_del);
  initbuffer(line_buf);
  v13 = 0xffffffff;
  v10 = separator_str;
  do {
    if (!v13) break;
    v13 -= 1;
    v11 = &v10[1];
    v1 = *v10;
    v10 = v11;
  } while (v1);
  v12 = (void *)xmalloc(~v13 + lineno_width);
  v13 = (~v13 - 1) + lineno_width;
  print_no_line_fmt = v12;
  memset(v12,0x20,v13);
  v3 = dat_9fc8;
  *(char *)((int)v12 + v13) = 0;
  line_no._0_4_ = (unsigned int)starting_line_number;
  line_no._4_4_ = starting_line_number._4_4_;
  current_type = body_type;
  current_regex = body_regex;
  if (*dat_9fc8 != v6) { // branch-flip
    while (*v3 < v6) {
      v8 = nl_file();
      *v3 = *v3 + 1;
      (unsigned char)v17 &= v8;
    }
  }
  else {
    v17._0_1_ = nl_file();
  }
  if (have_read_stdin) {
    v2 = *dat_9fd8;
    if (rpl_fclose(v2) == -1)
      error(1,*__errno_location(),"-",v14);
  }
  exit((unsigned int)((unsigned char)v17 ^ 1)); // no-return
label_127f:
  v17._0_1_ = 0;
  goto label_1230;
}

// Function: _start @ 0x1844
void _start(unsigned int a0,unsigned int a1)
{
  int v1; // ebx
  unsigned int v2; // stack - 0x4
  
  v2 = a0;
  sub_1876(a1,&v2);
  __libc_start_main(*(unsigned int *)(v1 + 0x878c));
  do {
  } while( true );
}

// Function: sub_1876 @ 0x1876
void sub_1876(void)
{
  return;
}

// Function: __x86.get_pc_thunk.bx @ 0x1880
void __x86.get_pc_thunk.bx(void)
{
  return;
}

// Function: deregister_tm_clones @ 0x1890
void deregister_tm_clones(void) // injected
{
  return;
}

// Function: register_tm_clones @ 0x18d0
void register_tm_clones(void) // injected
{ // warn: funcboundflow: fall-through reached the next function entry; truncating flow here
}

// Function: sub_18f2 @ 0x18f2
void sub_18f2(int a0,int a1,unsigned int a2)
{
  int v1; // eax
  int v2; // ebx
  
  v1 = a0 - (v2 >> 0x1f) >> 1;
  if ((v1) && (*(void **)(a1 + 0x118)))
    (**(void **)(a1 + 0x118))(a2,v1);
}

// Function: __do_global_dtors_aux @ 0x1920
void __do_global_dtors_aux(void) // injected, return-dupe
{
  if (completed.7283)
    return;
  if (dat_9fcc)
    __cxa_finalize(dat_a004);
  deregister_tm_clones();
  completed.7283 = 1;
}

// Function: frame_dummy @ 0x1970
void frame_dummy(void)
{
  register_tm_clones(); // tail-call
}

// Function: __x86.get_pc_thunk.dx @ 0x1979
void __x86.get_pc_thunk.dx(void)
{
  return;
}

// Function: print_lineno @ 0x1980
void print_lineno(void) // injected, return-dupe
{
  unsigned int v1; // esi
  int v2; // edi
  
  __printf_chk(1,lineno_format,lineno_width,(unsigned int)line_no,line_no._4_4_,separator_str);
  v1 = (unsigned int)line_no + (unsigned int)page_incr;
  v2 = line_no._4_4_ + page_incr._4_4_ + (unsigned int)CARRY4((unsigned int)line_no,(unsigned int)page_incr);
  if (line_no._4_4_ < v2) {
    line_no._0_4_ = v1;
    line_no._4_4_ = v2;
    return;
  }
  if ((line_no._4_4_ <= v2) && ((unsigned int)line_no <= v1)) {
    line_no._0_4_ = v1;
    line_no._4_4_ = v2;
    return;
  }
  error(1,0,dcgettext(NULL,"line number overflow",5));
  line_no._0_4_ = v1;
  line_no._4_4_ = v2;
}

// Function: build_type_arg @ 0x1a30
unsigned int build_type_arg(unsigned int *a0,unsigned int *a1,unsigned int a2) // injected, return-dupe
{
  char v1;
  char *v2;
  int v3; // eax
  
  v2 = (char *)*dat_9ff8;
  v1 = *v2;
  if (v1 != 'n') {
    if ('o' <= v1) { // branch-flip
      if (v1 == 'p') {
        *dat_9ff8 = (int)&v2[1];
        *a0 = v2;
        v2 = (char *)*dat_9ff8;
        *a1 = 0;
        a1[4] = a2;
        a1[1] = 0;
        a1[5] = 0;
        *dat_9ff0 = 0x2c6;
        v3 = re_compile_pattern(v2,strlen(v2),a1);
        if (v3) {
          error(1,0,"%s",v3);
          return 1;
        }
        return 1;
      }
      if (v1 != 't')
        return 0;
    }
    else if (v1 != 'a')
      return 0;
  }
  *a0 = v2;
  return 1;
}

// Function: nl_file @ 0x1b00
unsigned int nl_file(unsigned char *a0) // injected
{
  unsigned int v1;
  unsigned char *v10;
  int v11; // eax
  unsigned int *v12;
  unsigned char *v13;
  unsigned int v14;
  bool v15; // cf
  bool v16;
  unsigned char *v17; // stack - 0x20
  unsigned char *v2;
  unsigned char *v3;
  char v4;
  void *v5;
  char *v6;
  unsigned int *v7;
  short *v8;
  char *v9; // eax
  
  v11 = 2;
  v15 = &Stackfffffff0 < (char *)0x2c;
  v16 = &Stack00000000 == (char *)0x3c;
  v10 = a0;
  v13 = (unsigned char *)0x668e;
  do {
    if (!v11) break;
    v11 -= 1;
    v3 = &v13[1];
    v2 = &v10[1];
    v15 = *v10 < *v13;
    v16 = *v10 == *v13;
    v10 = v2;
    v13 = v3;
  } while (v16);
  if ((!v15 && !v16) != v15) { // branch-flip
    v10 = (unsigned char *)fopen64(a0,"r");
    if (!v10) {
      error(0,*__errno_location(),"%s",a0); // return-dupe
      return 0;
    }
  }
  else {
    have_read_stdin = 1;
    v10 = (unsigned char *)*dat_9fd8;
  }
  v17 = (unsigned char *)0x668e;
  fadvise(v10,2);
  v7 = dat_9fdc;
  while (v11 = readlinebuffer(line_buf,v10), v8 = line_buf._8_4_, v14 = line_buf._4_4_, v11) {
    v1 = line_buf._4_4_ - 1;
    if ((2 <= v1) && (*line_buf._8_4_ == *section_del)) {
      if ((v1 != header_del_len) || (v11 = memcmp(line_buf._8_4_,header_del,v1), v11)) {
        if ((v1 != body_del_len) || (v11 = memcmp(v8,body_del,v1), v11)) {
          if ((v1 != footer_del_len) || (v11 = memcmp(v8,footer_del,v1), v11)) goto label_1c20;
          current_type = footer_type;
          current_regex = footer_regex;
        }
        else {
          current_type = body_type;
          current_regex = body_regex;
        }
        v5 = (void *)*dat_9fdc;
        v6 = *(char **)((int)v5 + 0x14);
        if (*(char **)((int)v5 + 0x18) <= v6) // branch-flip
          __overflow(v5,10);
        else {
          *(char **)((int)v5 + 0x14) = &v6[1];
          *v6 = 10;
        }
      }
      else {
        current_type = header_type;
        current_regex = header_regex;
        if (reset_numbers) {
          line_no._0_4_ = (unsigned int)starting_line_number;
          line_no._4_4_ = starting_line_number._4_4_;
        }
        v5 = (void *)*dat_9fdc;
        v6 = *(char **)((int)v5 + 0x14);
        if (*(char **)((int)v5 + 0x18) <= v6) // branch-flip
          __overflow(v5,10);
        else {
          *(char **)((int)v5 + 0x14) = &v6[1];
          *v6 = 10;
        }
      }
    }
    else {
label_1c20:
      v4 = *current_type;
      v12 = v7;
      if (v4 != 'n') { // branch-flip
        if ('o' <= v4) { // branch-flip
          if (v4 != 'p') { // branch-flip
            if (v4 != 't') goto label_1e42;
            if (2 <= v14) goto label_1c50;
          }
          else {
            v11 = re_search(current_regex,line_buf._8_4_,v1,0,v1,0);
            if (v11 != -2) { // branch-flip
              if (v11 != -1) {
                print_lineno();
                v14 = line_buf._4_4_;
                goto label_1e42;
              }
            }
            else {
              v9 = dcgettext(NULL,"error in regular expression search",5);
              error(1,*__errno_location(),v9);
            }
          }
label_1dea:
          v12 = dat_9fdc;
          fputs_unlocked(print_no_line_fmt,(void *)*dat_9fdc);
          v14 = line_buf._4_4_;
        }
        else if (v4 != 'a') { // branch-flip
label_1e42:
          v12 = dat_9fdc;
        }
        else if ((0 <= blank_join._4_4_) && ((1 <= blank_join._4_4_ || (2 <= (unsigned int)blank_join)))) {
          if (v14 <= 1) {
            v15 = 0xffffffff <= (unsigned int)blank_lines.6494;
            (unsigned int)blank_lines.6494 += 1;
            blank_lines.6494._4_4_ += (unsigned int)v15;
            if (blank_join._4_4_ != blank_lines.6494._4_4_ || (unsigned int)blank_join != (unsigned int)blank_lines.6494) goto label_1dea;
          }
          print_lineno();
          blank_lines.6494._0_4_ = 0;
          blank_lines.6494._4_4_ = 0;
          v12 = dat_9fdc;
          v14 = line_buf._4_4_;
        }
        else {
label_1c50:
          print_lineno();
          v14 = line_buf._4_4_;
        }
      }
      else {
        fputs_unlocked(print_no_line_fmt,(void *)*v7);
        v14 = line_buf._4_4_;
      }
      fwrite_unlocked(line_buf._8_4_,1,v14,(void *)*v12);
    }
  }
  v15 = 0;
  if (!(*v10 & 0x20)) {
    v11 = 2;
    v16 = 1;
    v13 = a0;
    do {
      if (!v11) break;
      v11 -= 1;
      v3 = &v17[1];
      v2 = &v13[1];
      v15 = *v13 < *v17;
      v16 = *v13 == *v17;
      v13 = v2;
      v17 = v3;
    } while (v16);
    if ((!v15 && !v16) == v15) {
      clearerr_unlocked(v10);
      return 1;
    }
    if (rpl_fclose(v10) != -1)
      return 1;
  }
  error(0,*__errno_location(),"%s",a0);
  return 0;
}

// Function: usage @ 0x1f60
void usage(unsigned int a0,int a1) // injected
{
  void *v1;
  unsigned int *v2;
  int v3;
  char *v4;
  unsigned int v5;
  char *v6; // eax
  int v7; // eax
  unsigned int v8; // eax
  unsigned int v9; // edx
  
  v3 = a1;
  v5 = program_name;
  if (a1) { // branch-flip
    v4 = dcgettext(NULL,"Try `%s --help\' for more information.\n",5);
    __fprintf_chk((void *)*dat_9fc4,1,v4,v5);
  }
  else {
    __printf_chk(1,dcgettext(NULL,"Usage: %s [OPTION]... [FILE]...\n",5),v5,a0);
    v2 = dat_9fdc;
    v1 = (void *)*dat_9fdc;
    fputs_unlocked(dcgettext(NULL,"Write each FILE to standard output, with line numbers added.\nWith no FILE, or when FILE is -, read standard input.\n\n",5),v1);
    v1 = (void *)*v2;
    fputs_unlocked(dcgettext(NULL,"Mandatory arguments to long options are mandatory for short options too.\n",5),v1);
    v1 = (void *)*v2;
    fputs_unlocked(dcgettext(NULL,"  -b, --body-numbering=STYLE      use STYLE for numbering body lines\n  -d, --section-delimiter=CC      use CC for separating logical pages\n  -f, --footer-numbering=STYLE    use STYLE for numbering footer lines\n",5),v1);
    v1 = (void *)*v2;
    fputs_unlocked(dcgettext(NULL,"  -h, --header-numbering=STYLE    use STYLE for numbering header lines\n  -i, --line-increment=NUMBER     line number increment at each line\n  -l, --join-blank-lines=NUMBER   group of NUMBER empty lines counted as one\n  -n, --number-format=FORMAT      insert line numbers according to FORMAT\n  -p, --no-renumber               do not reset line numbers at logical pages\n  -s, --number-separator=STRING   add STRING after (possible) line number\n",5),v1);
    v1 = (void *)*v2;
    fputs_unlocked(dcgettext(NULL,"  -v, --starting-line-number=NUMBER  first line number on each logical page\n  -w, --number-width=NUMBER       use NUMBER columns for line numbers\n",5),v1);
    v1 = (void *)*v2;
    fputs_unlocked(dcgettext(NULL,"      --help     display this help and exit\n",5),v1);
    v1 = (void *)*v2;
    fputs_unlocked(dcgettext(NULL,"      --version  output version information and exit\n",5),v1);
    v1 = (void *)*v2;
    fputs_unlocked(dcgettext(NULL,"\nBy default, selects -v1 -i1 -l1 -sTAB -w6 -nrn -hn -bt -fn.  CC are\ntwo delimiter characters for separating logical pages, a missing\nsecond character implies :.  Type \\\\ for \\.  STYLE is one of:\n",5),v1);
    v1 = (void *)*v2;
    fputs_unlocked(dcgettext(NULL,"\n  a         number all lines\n  t         number only nonempty lines\n  n         number no lines\n  pBRE      number only lines that contain a match for the basic regular\n            expression, BRE\n\nFORMAT is one of:\n\n  ln   left justified, no leading zeros\n  rn   right justified, no leading zeros\n  rz   right justified, leading zeros\n\n",5),v1);
    v5 = last_component(program_name);
    __printf_chk(1,dcgettext(NULL,"\nReport %s bugs to %s\n",5),v5,"bug-coreutils@gnu.org");
    __printf_chk(1,dcgettext(NULL,"%s home page: <%s>\n",5),"GNU coreutils","http://www.gnu.org/software/coreutils/");
    v1 = (void *)*v2;
    fputs_unlocked(dcgettext(NULL,"General help using GNU software: <http://www.gnu.org/gethelp/>\n",5),v1);
    v4 = NULL;
    v6 = setlocale(5,NULL);
    v5 = 5;
    if (v6) {
      v4 = "en_";
      v5 = 3;
      v7 = strncmp(v6,"en_",3);
      a0 = v9;
      v8 = last_component(program_name,0x5bac,3,v9);
      if (v7) {
        v4 = dcgettext(NULL,"Report %s translation bugs to <http://translationproject.org/team/>\n",5);
        a0 = v8;
        __printf_chk(1,v4,v8,v8);
        v5 = v8;
      }
    }
    v5 = last_component(program_name,v4,v5,a0);
    __printf_chk(1,dcgettext(NULL,"For complete documentation, run: info coreutils \'%s invocation\'\n",5),v5);
  }
  exit(v3); // no-return
}

// Function: close_stdout_set_file_name @ 0x2240
void close_stdout_set_file_name(unsigned int a0) // injected
{
  file_name = a0;
}

// Function: close_stdout_set_ignore_EPIPE @ 0x2260
void close_stdout_set_ignore_EPIPE(char a0) // injected
{
  ignore_EPIPE = a0;
}

// Function: close_stdout @ 0x2280
void close_stdout(void) // injected
{
  int *v1; // eax
  char *v2; // eax
  unsigned int v3;
  
  v3 = *dat_9fdc;
  if (close_stream(v3)) {
    v1 = __errno_location();
    if ((!ignore_EPIPE) || (*v1 != 0x20)) {
      v2 = dcgettext(NULL,"write error",5);
      if (file_name) { // branch-flip
        v3 = quotearg_colon(file_name,"write error",5,v1);
        error(0,*v1,"%s: %s",v3,v2);
      }
      else {
        error(0,*v1,"%s",v2);
      }
      _exit(exit_failure); // return-dupe, no-return
    }
  }
  v3 = *dat_9fc4;
  if (!close_stream(v3))
    return;
  _exit(exit_failure);
}

// Function: __x86.get_pc_thunk.ax @ 0x233c
unsigned int __x86.get_pc_thunk.ax(void)
{
  unsigned int v1; // stack + 0x0
  
  return v1;
}

// Function: last_component @ 0x2340
char * last_component(char *a0)
{
  bool v1;
  char *v2;
  char v3;
  
  v3 = *a0;
  while (v3 == '/') {
    a0 = &a0[1];
    v3 = *a0;
  }
  if (v3) {
    v1 = 0;
    v2 = a0;
    do {
      while (v3 == '/') {
        v2 = &v2[1];
        v3 = *v2;
        v1 = 1;
        if (!v3)
          return a0;
      }
      if (v1) {
        v1 = 0;
        a0 = v2;
      }
      v2 = &v2[1];
      v3 = *v2;
    } while (v3);
  }
  return a0;
}

// Function: base_len @ 0x2390
unsigned int base_len(char *a0) // injected, early-return
{
  int v1;
  unsigned int v2;
  unsigned int v3; // edx
  
  v2 = strlen(a0);
  if ((2 <= v2) && (a0[v2 - 1] == '/')) {
    do {
      v3 = v2 - 1;
      if (v3 == 1)
        return 1;
      v1 = v2 - 2;
      v2 = v3;
    } while (a0[v1] == '/');
    return v3;
  }
  return v2;
}

// Function: fdadvise @ 0x2400
void fdadvise(unsigned int a0,unsigned int a1,unsigned int a2,unsigned int a3,unsigned int a4,unsigned int a5) // injected
{
  posix_fadvise64(a0,a1,a2,a3,a4,a5);
}

// Function: fadvise @ 0x2440
void fadvise(void *a0,unsigned int a1) // injected, return-dupe
{
  if (!a0)
    return;
  posix_fadvise64(fileno(a0),0,0,0,0,a1);
}

// Function: initbuffer @ 0x2480
void initbuffer(unsigned int *a0)
{
  *a0 = 0;
  a0[1] = 0;
  a0[2] = 0;
}

// Function: readlinebuffer_delim @ 0x24a0
int * readlinebuffer_delim(int *a0,unsigned char *a1,unsigned char a2) // injected
{
  unsigned char *v1;
  unsigned char *v10; // stack - 0x28
  int v2;
  unsigned char *v3;
  unsigned char v4;
  unsigned int v5; // eax
  unsigned int v6;
  unsigned char v7;
  unsigned char *v8;
  unsigned char *v9; // stack - 0x2c
  
  v4 = a2;
  v3 = a1;
  v5 = (unsigned int)(char)a2;
  v10 = (unsigned char *)a0[2];
  v9 = &v10[*a0];
  v8 = v10;
  if (*a1 & 0x10)
    return NULL;
  do {
    while( true ) {
      v1 = *(unsigned char **)&v3[4];
      if (*(unsigned char **)&v3[8] <= v1) { // branch-flip
        v6 = __uflow(v3);
        if (v6 != 0xffffffff) // branch-flip
          v7 = (unsigned char)v6;
        else {
          if (v10 == v8)
            return NULL;
          if (*v3 & 0x20)
            return NULL;
          if (v8[-1] == v4) goto label_2551;
          v6 = v5;
          v7 = v4;
        }
      }
      else {
        *(unsigned char **)&v3[4] = &v1[1];
        v6 = (unsigned int)*v1;
        v7 = *v1;
      }
      if (v8 != v9) break;
      v2 = *a0;
      v10 = (unsigned char *)x2realloc(v10,a0);
      v8 = &v10[v2];
      a0[2] = (int)v10;
      v9 = &v10[*a0];
      *v8 = v7;
      v8 = &v8[1];
      if (v6 == v5) goto label_2551;
    }
    v1 = &v8[1];
    *v8 = v7;
    v8 = v1;
  } while (v6 != v5);
label_2551:
  a0[1] = (int)v8 - (int)v10;
  return a0;
}

// Function: readlinebuffer @ 0x25c0
void readlinebuffer(unsigned int a0,unsigned int a1)
{
  readlinebuffer_delim(a0,a1,10);
}

// Function: freebuffer @ 0x25e0
void freebuffer(int a0) // injected
{
  free(*(void **)(a0 + 8));
}

// Function: set_program_name @ 0x2600
void set_program_name(unsigned char *a0) // injected
{
  unsigned char *v1;
  bool v10; // cf
  bool v11; // zf
  unsigned char *v2;
  unsigned char *v3;
  char *v4; // eax
  unsigned int v5; // ecx
  int v6; // ecx
  unsigned char *v7;
  unsigned char *v8;
  bool v9;
  
  if (!a0) {
    fwrite("A NULL argv[0] was passed through an exec system call.\n",1,0x37,(void *)*dat_9fc4);
    abort(); // no-return
  }
  v4 = strrchr((char *)a0,0x2f);
  if (v4) {
    v1 = (unsigned char *)&v4[1];
    v5 = (int)v1 - (int)a0;
    v9 = v5 < 6;
    v11 = v5 == 6;
    if (7 <= (int)v5) {
      v6 = 7;
      v7 = (unsigned char *)&v4[-6];
      v8 = (unsigned char *)"/.libs/";
      do {
        if (!v6) break;
        v6 -= 1;
        v3 = &v8[1];
        v2 = &v7[1];
        v9 = *v7 < *v8;
        v11 = *v7 == *v8;
        v7 = v2;
        v8 = v3;
      } while (v11);
      v10 = 0;
      if ((!v9 && !v11) == v9) {
        v6 = 3;
        v9 = 1;
        v7 = v1;
        v8 = (unsigned char *)0x668c;
        do {
          if (!v6) break;
          v6 -= 1;
          v3 = &v8[1];
          v2 = &v7[1];
          v10 = *v7 < *v8;
          v9 = *v7 == *v8;
          v7 = v2;
          v8 = v3;
        } while (v9);
        a0 = v1;
        if ((!v10 && !v9) == v10) {
          a0 = (unsigned char *)&v4[4];
          *dat_9fec = (int)a0;
        }
      }
    }
  }
  program_name = a0;
  *dat_9fe4 = (int)a0;
}

// Function: quote_n @ 0x26b0
void quote_n(unsigned int a0,unsigned int a1) // injected
{
  quotearg_n_style(a0,6,a1);
}

// Function: quote @ 0x26e0
void quote(unsigned int a0) // injected
{
  quotearg_n_style(0,6,a0);
}

// Function: quotearg_buffer_restyled @ 0x2710
unsigned int quotearg_buffer_restyled(char *a0,unsigned int a1,char *a2,unsigned int a3,int a4,unsigned int a5,int a6,char *a7,char *a8) // injected
{
  unsigned char v1;
  unsigned int v10; // stack - 0x28
  unsigned int v11;
  unsigned char *v12; // esi
  unsigned int v13;
  int v14;
  bool v15;
  unsigned int v16; // stack - 0x64
  unsigned int v17; // stack - 0x58
  char *v18; // stack - 0x54
  char *v19; // stack - 0x4c
  char v2;
  char *v20; // stack - 0x44
  unsigned char v21; // stack - 0x34
  unsigned char v22; // stack - 0x33
  bool v23; // stack - 0x32
  unsigned int v24; // stack - 0x24
  int v25; // stack - 0x20
  unsigned int v26; // stack - 0x14
  unsigned int v3;
  char *v4;
  unsigned int v5;
  int v6; // eax
  unsigned char v7;
  unsigned int v8;
  unsigned int v9; // stack - 0x2c
  
  v26 = 0x2719;
  v20 = a7;
  v19 = a8;
  v25 = *(int *)(v14 + 0x14);
  v3 = __ctype_get_mb_cur_max();
  v13 = a5 & 2;
  switch(a4) {
    case 0:
      v23 = 0;
      v22 = 0;
      v11 = 0;
      v16 = 0;
      v18 = NULL;
      break;
    case 1:
      v23 = 1;
      v22 = 0;
      v16 = 1;
      v11 = 0;
      a4 = 2;
      v18 = "\'";
      break;
    case 2:
      if (v13) { // branch-flip
        v23 = 1;
        v22 = 0;
        v16 = 1;
        v11 = 0;
        v18 = "\'";
      }
      else {
        if (a1)
          *a0 = 0x27;
        v23 = 0;
        v22 = 0;
        v16 = 1;
        v11 = 1;
        v18 = "\'";
      }
      break;
    case 3:
      if (v13) { // branch-flip
        v23 = 1;
        v22 = 1;
        v16 = 1;
        v11 = 0;
        v18 = "\"";
      }
      else {
        if (a1)
          *a0 = 0x22;
        v23 = 0;
        v22 = 1;
        v16 = 1;
        v11 = 1;
        v18 = "\"";
      }
      break;
    case 4:
      v18 = "\"";
      v23 = 1;
      v22 = 1;
      v16 = 1;
      v11 = 0;
      a4 = 3;
      break;
    case 5:
      v23 = 0;
      v22 = 1;
      v11 = 0;
      v16 = 0;
      v18 = NULL;
      break;
    case 6:
    case 7:
    case 8:
      if (a4 != 8) {
        v4 = dcgettext(NULL,"`",5);
        v20 = v4;
        if ((v4 == "`") && (v20 = "\"", a4 != 7))
          v20 = v4;
        v4 = dcgettext(NULL,"\'",5);
        v19 = v4;
        if ((v4 == "\'") && (v19 = "\"", a4 != 7))
          v19 = v4;
      }
      v11 = 0;
      if (!v13) {
        v2 = *v20;
        v11 = 0;
        if (v2) {
          v11 = 0;
          do {
            if (v11 < a1)
              a0[v11] = v2;
            v11 += 1;
            v2 = v20[v11];
          } while (v2);
        }
      }
      v23 = v13 != 0;
      v18 = v19;
      v16 = strlen(v19);
      v22 = 1;
      break;
    default:
      abort(); // no-return
    
  }
  v13 = 0;
label_27b8:
  v15 = a3 != v13;
  if (a3 == 0xffffffff)
    v15 = a2[v13] != '\0';
  if (!v15) {
    if ((!(v23 & a4 == 2)) || (v11)) {
      v3 = v11;
      if ((v18) && ((!v23 && (v2 = *v18, v2)))) {
        do {
          if (v3 < a1)
            a0[v3] = v2;
          v3 += 1;
          v2 = v18[v3 - v11];
        } while (v2);
      }
      if (v3 < a1)
        a0[v3] = 0;
      goto label_2b54;
    }
label_2b18:
    v3 = quotearg_buffer_restyled(a3,a4,a5 & 0xfffffffd,0,v20,v19);
label_2b54:
    if (v25 == *(int *)(v14 + 0x14))
      return v3;
    __stack_chk_fail_local(); // no-return
  }
  v12 = (unsigned char *)&a2[v13];
  v21 = v16 != 0 & v22;
  v5 = v11;
  if (v21) {
    if (a3 < v13 + v16) {
      v7 = *v12;
      v8 = (unsigned int)v7;
      switch(v7) {
        case 0:
          v21 = 0;
          goto label_2efd;
        default:
          v21 = 0;
          goto label_2bf8;
        case 7:
          v7 = 0x61;
          break;
        case 8:
          v7 = 0x62;
          break;
        case 9:
          v8 = 9;
          v21 = 0;
          v7 = 0x74;
          goto label_2a72;
        case 10:
          v8 = 10;
          v21 = 0;
          v7 = 0x6e;
          goto label_2a72;
        case 0xb:
          v7 = 0x76;
          break;
        case 0xc:
          v7 = 0x66;
          break;
        case 0xd:
          v8 = 0xd;
          v21 = 0;
          v7 = 0x72;
          goto label_2a72;
        case 0x20:
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
        case 0x5b:
        case 0x5e:
        case 0x60:
        case 0x7c:
          v21 = 0;
          goto label_2990;
        case 0x23:
        case 0x7e:
          v21 = 0;
          goto label_29b5;
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
        case 0x41:
        case 0x42:
        case 0x43:
        case 0x44:
        case 0x45:
        case 0x46:
        case 0x47:
        case 0x48:
        case 0x49:
        case 0x4a:
        case 0x4b:
        case 0x4c:
        case 0x4d:
        case 0x4e:
        case 0x4f:
        case 0x50:
        case 0x51:
        case 0x52:
        case 0x53:
        case 0x54:
        case 0x55:
        case 0x56:
        case 0x57:
        case 0x58:
        case 0x59:
        case 0x5a:
        case 0x5d:
        case 0x5f:
        case 0x61:
        case 0x62:
        case 99:
        case 100:
        case 0x65:
        case 0x66:
        case 0x67:
        case 0x68:
        case 0x69:
        case 0x6a:
        case 0x6b:
        case 0x6c:
        case 0x6d:
        case 0x6e:
        case 0x6f:
        case 0x70:
        case 0x71:
        case 0x72:
        case 0x73:
        case 0x74:
        case 0x75:
        case 0x76:
        case 0x77:
        case 0x78:
        case 0x79:
        case 0x7a:
          v21 = 0;
          if (a6) goto label_293b;
          goto label_2969;
        case 0x27:
          v21 = 0;
          goto label_2a05;
        case 0x3f:
          v21 = 0;
          goto label_2865;
        case 0x5c:
          v21 = v22 & v23;
          if (!v21) goto label_3018;
          goto label_2fbf;
        case 0x7b:
        case 0x7d:
          v21 = 0;
          goto label_29c5;
        
      }
      goto label_2ada;
    }
    if (!memcmp(v12,v18,v16)) {
      if (!v23) goto label_2840;
      goto label_2b18;
    }
    v21 = 0;
  }
label_2840:
  v7 = *v12;
  v8 = (unsigned int)v7;
  switch(v7) {
    case 0:
      if (v22) {
label_2efd:
        if (!v23) {
          if (v11 < a1)
            a0[v11] = 0x5c;
          v5 = v11 + 1;
          if ((a3 <= v13 + 1) || (10 <= (unsigned char)(a2[v13 + 1] - 0x30U)))
            v8 = 0x30;
          else {
            if (v5 < a1)
              a0[v5] = 0x30;
            if (v11 + 2 < a1)
              a0[v11 + 2] = 0x30;
            v5 = v11 + 3;
            v8 = 0x30;
          }
          goto label_2933;
        }
        goto label_2b18;
      }
      v8 = 0;
      if (!(a5 & 1)) goto label_2a8e;
      v13 += 1;
      goto label_27b8;
    default:
label_2bf8:
      v1 = v22;
      if (v3 != 1) {
        v10 = 0;
        v24 = 0;
        if (a3 == 0xffffffff)
          a3 = strlen(a2);
        v17 = 0;
        do {
          v8 = v13 + v17;
          v5 = mbrtowc(&v9,&a2[v8],a3 - v8,&v10);
          if (!v5) break;
          if (v5 == 0xffffffff) goto label_317d;
          if (v5 == 0xfffffffe) {
            if ((a3 <= v8) || (!a2[v8])) goto label_317d;
            goto label_327e;
          }
          if (((a4 == 2) && (v23)) && (v5 != 1)) {
            v4 = &a2[v8 + 1];
            do {
              switch(*v4) {
                case 0x5b:
                case 0x5c:
                case 0x5e:
                case 0x60:
                case 0x7c:
                  goto label_2b18;
                
              }
              v4 = &v4[1];
            } while (&a2[v8 + v5] != v4);
          }
          v6 = iswprint(v9);
          if (!v6)
            v15 = 0;
          v17 += v5;
        } while (!mbsinit(&v10));
        v1 = (v15 ^ 1U) & v22;
        goto label_317d;
      }
      v17 = 1;
      v1 = ((unsigned char)((unsigned short)*(unsigned short *)(*(int *)__ctype_b_loc() + (unsigned int)v7 * 2) >> 0xe) ^ 1) & v22;
      goto label_2c2c;
    case 7:
      v8 = 7;
      v7 = 0x61;
      goto label_2acf;
    case 8:
      v8 = 8;
      v7 = 0x62;
      goto label_2acf;
    case 9:
      v8 = 9;
      v7 = 0x74;
      goto label_2a72;
    case 10:
      v8 = 10;
      v7 = 0x6e;
      goto label_2a72;
    case 0xb:
      v8 = 0xb;
      v7 = 0x76;
      goto label_2acf;
    case 0xc:
      v8 = 0xc;
      v7 = 0x66;
      goto label_2acf;
    case 0xd:
      v8 = 0xd;
      v7 = 0x72;
label_2a72:
      if ((a4 != 2) || (!v23)) {
label_2acf:
        if (!v22) goto label_2a8e;
        goto label_2ada;
      }
      goto label_2b18;
    case 0x20:
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
    case 0x5b:
    case 0x5e:
    case 0x60:
    case 0x7c:
label_2990:
      if ((a4 != 2) || (!v23)) break;
      goto label_2b18;
    case 0x23:
    case 0x7e:
label_29b5:
label_29ef:
      if (!v13) goto label_2990;
      break;
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
    case 0x41:
    case 0x42:
    case 0x43:
    case 0x44:
    case 0x45:
    case 0x46:
    case 0x47:
    case 0x48:
    case 0x49:
    case 0x4a:
    case 0x4b:
    case 0x4c:
    case 0x4d:
    case 0x4e:
    case 0x4f:
    case 0x50:
    case 0x51:
    case 0x52:
    case 0x53:
    case 0x54:
    case 0x55:
    case 0x56:
    case 0x57:
    case 0x58:
    case 0x59:
    case 0x5a:
    case 0x5d:
    case 0x5f:
    case 0x61:
    case 0x62:
    case 99:
    case 100:
    case 0x65:
    case 0x66:
    case 0x67:
    case 0x68:
    case 0x69:
    case 0x6a:
    case 0x6b:
    case 0x6c:
    case 0x6d:
    case 0x6e:
    case 0x6f:
    case 0x70:
    case 0x71:
    case 0x72:
    case 0x73:
    case 0x74:
    case 0x75:
    case 0x76:
    case 0x77:
    case 0x78:
    case 0x79:
    case 0x7a:
      break;
    case 0x27:
label_2a05:
      v8 = 0x27;
      if (a4 == 2) {
        if (v23) goto label_2b18;
        if (v11 < a1)
          a0[v11] = 0x27;
        if (v11 + 1 < a1)
          a0[v11 + 1] = 0x5c;
        if (v11 + 2 < a1)
          a0[v11 + 2] = 0x27;
        v11 += 3;
        v8 = 0x27;
      }
      break;
    case 0x3f:
label_2865:
      if (a4 != 2) { // branch-flip
        if ((a4 == 3) && (a5 & 4)) {
          v5 = v13 + 2;
          v8 = 0x3f;
          if ((v5 < a3) && (a2[v13 + 1] == '?')) {
            v8 = (unsigned int)(unsigned char)a2[v5];
            v7 = a2[v5] - 0x21;
            if ((0x1e <= v7) || (!(1 << (v7 & 0x1f) & 0x380051c1U))) goto label_300b;
            if (!v23) {
              if (v11 < a1)
                a0[v11] = 0x3f;
              if (v11 + 1 < a1)
                a0[v11 + 1] = 0x22;
              if (v11 + 2 < a1)
                a0[v11 + 2] = 0x22;
              if (v11 + 3 < a1)
                a0[v11 + 3] = 0x3f;
              v11 += 4;
              v13 = v5;
              break;
            }
            goto label_2b18;
          }
          break;
        }
      }
      else if (v23) goto label_2b18;
label_300b:
      v8 = 0x3f;
      break;
    case 0x5c:
      if ((!v22) || (!v23)) {
label_3018:
        v8 = 0x5c;
        v7 = 0x5c;
        goto label_2a72;
      }
      if (v16) {
label_2fbf:
        v13 += 1;
        v7 = 0x5c;
        goto label_2970;
      }
      goto label_2b18;
    case 0x7b:
    case 0x7d:
label_29c5:
      v15 = a3 != 1;
      if (a3 == 0xffffffff)
        v15 = a2[1] != '\0';
      if (!v15) goto label_29ef;
    
  }
label_2928:
  v5 = v11;
  if (v22) { // branch-flip
label_2933:
    v7 = (unsigned char)v8;
    v11 = v5;
    if (a6) { // branch-flip
label_293b:
      v7 = (unsigned char)v8;
      if (!(*(unsigned int *)(a6 + (v8 >> 5) * 4) & 1 << (v7 & 0x1f))) goto label_295e;
    }
    else {
label_295e:
      if (!v21) {
label_2969:
        v13 += 1;
        goto label_2970;
      }
    }
label_2ada:
    if (v23) goto label_2b18;
    v13 += 1;
    v5 = v11;
  }
  else {
label_2a8e:
    v7 = (unsigned char)v8;
    if (v23) goto label_2933;
    v13 += 1;
    v11 = v5;
    if (!v21) goto label_2970;
  }
  if (v5 < a1)
    a0[v5] = 0x5c;
  v11 = v5 + 1;
label_2970:
  if (v11 < a1)
    a0[v11] = v7;
  v11 += 1;
  goto label_27b8;
  while (v12[v17]) {
label_327e:
    v17 += 1;
    if (a3 <= v13 + v17) break;
  }
label_317d:
  if (v17 <= 1) {
label_2c2c:
    v8 = (unsigned int)v7;
    if (!v1) goto label_2928;
    v1 = v22;
  }
  v17 += v13;
  while( true ) {
    if (v1) { // branch-flip
      if (v23) goto label_2b18;
      if (v11 < a1)
        a0[v11] = 0x5c;
      if (v11 + 1 < a1)
        a0[v11 + 1] = (v7 >> 6) + 0x30;
      if (v11 + 2 < a1)
        a0[v11 + 2] = (v7 >> 3 & 7) + 0x30;
      v11 += 3;
      v7 = (v7 & 7) + 0x30;
    }
    else if (v21) {
      if (v11 < a1)
        a0[v11] = 0x5c;
      v11 += 1;
      v21 = 0;
    }
    v13 += 1;
    if (v17 <= v13) break;
    if (v11 < a1)
      a0[v11] = v7;
    v11 += 1;
    v7 = a2[v13];
  }
  goto label_2970;
}

// Function: quotearg_n_options @ 0x32f0
char * quotearg_n_options(unsigned int a0,unsigned int a1,unsigned int a2,unsigned int *a3) // injected
{
  unsigned int v1;
  unsigned int *v2;
  unsigned int *v3;
  int v4;
  unsigned int v5;
  unsigned long long *v6;
  unsigned int v7; // eax
  char *v8;
  int *v9; // stack - 0x2c
  
  v9 = __errno_location();
  v4 = *v9;
  if ((int)a0 < 0)
    abort(); // no-return
  v6 = slotvec;
  if (nslots <= a0) {
    v1 = a0 + 1;
    if (0x20000000 <= v1)
      xalloc_die(); // no-return
    if (slotvec != &slotvec0) // branch-flip
      v6 = (unsigned long long *)xrealloc(slotvec,v1 * 8);
    else {
      v6 = (unsigned long long *)xrealloc(0,v1 * 8);
      *(unsigned int *)((int)v6 + 4) = slotvec0._4_4_;
      *(unsigned int *)v6 = (unsigned int)slotvec0;
    }
    slotvec = v6;
    memset(&v6[nslots],0,(v1 - nslots) * 8);
    nslots = v1;
  }
  v3 = (unsigned int *)&v6[a0];
  v1 = a3[1];
  v2 = &a3[2];
  v5 = *v3;
  v8 = (char *)v3[1];
  v7 = quotearg_buffer_restyled(a2,*a3,v1 | 1,v2,a3[10],a3[0xb]);
  if (v5 <= v7) {
    *v3 = v7 + 1;
    if (v8 != slot0)
      free(v8);
    v8 = (char *)xmalloc(v7 + 1);
    v3[1] = (unsigned int)v8;
    quotearg_buffer_restyled(a2,*a3,v1 | 1,v2,a3[10],a3[0xb]);
  }
  *v9 = v4;
  return v8;
}

// Function: clone_quoting_options @ 0x3490
void clone_quoting_options(char *a0) // injected
{
  int v1;
  int *v2; // eax
  char *v3;
  
  v2 = __errno_location();
  v1 = *v2;
  v3 = a0;
  if (!a0)
    v3 = default_quoting_options;
  xmemdup(v3,0x30);
  *v2 = v1;
}

// Function: get_quoting_style @ 0x34d0
unsigned int * get_quoting_style(unsigned int *a0) // injected
{
  unsigned int *v1; // eax
  
  v1 = a0;
  if (!a0)
    v1 = (unsigned int *)default_quoting_options;
  v1 = (unsigned int *)*v1;
  return v1;
}

// Function: set_quoting_style @ 0x34f0
void set_quoting_style(unsigned int *a0,unsigned int a1) // injected
{
  if (!a0)
    a0 = (unsigned int *)default_quoting_options;
  *a0 = a1;
}

// Function: set_char_quoting @ 0x3520
unsigned int set_char_quoting(char *a0,unsigned int a1,unsigned int a2) // injected
{
  unsigned int v1;
  unsigned int v2; // eax
  unsigned char v3; // cl
  
  if (!a0)
    a0 = default_quoting_options;
  v3 = (unsigned char)a1 & 0x1f;
  v1 = *(unsigned int *)&a0[(a1 >> 5 & 7) * 4 + 8];
  v2 = v1 >> v3;
  *(unsigned int *)&a0[(a1 >> 5 & 7) * 4 + 8] = ((a2 ^ v2) & 1) << v3 ^ v1;
  return v2 & 1;
}

// Function: set_quoting_flags @ 0x3570
unsigned int set_quoting_flags(char *a0,unsigned int a1) // injected
{
  unsigned int v1;
  
  if (!a0)
    a0 = default_quoting_options;
  v1 = *(unsigned int *)&a0[4];
  *(unsigned int *)&a0[4] = a1;
  return v1;
}

// Function: set_custom_quoting @ 0x35a0
void set_custom_quoting(unsigned int *a0,int a1,int a2) // injected
{
  if (!a0)
    a0 = (unsigned int *)default_quoting_options;
  *a0 = 8;
  if ((a1) && (a2)) {
    a0[10] = a1;
    a0[0xb] = a2;
    return;
  }
  abort(); // no-return
}

// Function: quotearg_buffer @ 0x35f0
void quotearg_buffer(void) // injected
{
  int v1;
  int *v2; // eax
  unsigned int *v3;
  unsigned int v4; // stack + 0x10
  unsigned int *v5; // stack + 0x14
  
  v3 = v5;
  if (!v5)
    v3 = (unsigned int *)default_quoting_options;
  v2 = __errno_location();
  v1 = *v2;
  quotearg_buffer_restyled(v4,*v3,v3[1],&v3[2],v3[10],v3[0xb]);
  *v2 = v1;
}

// Function: quotearg_alloc_mem @ 0x3650
unsigned int quotearg_alloc_mem(unsigned int a0,unsigned int a1,int *a2,unsigned int *a3) // injected
{
  int v1;
  int *v2; // eax
  unsigned int v3; // eax
  int v4; // eax
  unsigned int v5; // eax
  unsigned int *v6;
  
  v6 = a3;
  if (!a3)
    v6 = (unsigned int *)default_quoting_options;
  v2 = __errno_location();
  v1 = *v2;
  v3 = (unsigned int)(a2 == NULL) | v6[1];
  v4 = quotearg_buffer_restyled(a1,*v6,v3,&v6[2],v6[10],v6[0xb]);
  v5 = xmalloc(v4 + 1);
  quotearg_buffer_restyled(a1,*v6,v3,&v6[2],v6[10],v6[0xb]);
  *v2 = v1;
  if (a2)
    *a2 = v4;
  return v5;
}

// Function: quotearg_alloc @ 0x3710
void quotearg_alloc(unsigned int a0,unsigned int a1,unsigned int a2)
{
  quotearg_alloc_mem(a0,a1,0,a2);
}

// Function: quotearg_free @ 0x3730
void quotearg_free(void) // injected, return-dupe
{
  unsigned int *v1;
  void *v2;
  unsigned long long *v3;
  unsigned int *v4;
  
  v3 = slotvec;
  if (2 <= nslots) {
    v4 = (unsigned int *)((int)slotvec + 0xc);
    v1 = (unsigned int *)((int)slotvec + nslots * 8 + 4);
    do {
      v2 = (void *)*v4;
      v4 = &v4[2];
      free(v2);
    } while (v4 != v1);
  }
  if (*(char **)((int)v3 + 4) != slot0) {
    free(*(char **)((int)v3 + 4));
    slotvec0._0_4_ = 0x100;
    slotvec0._4_4_ = slot0;
  }
  if (v3 == &slotvec0) {
    nslots = 1;
    return;
  }
  free(v3);
  nslots = 1;
  slotvec = &slotvec0;
}

// Function: quotearg_n @ 0x37d0
void quotearg_n(void) // injected
{
  char *v1; // stack + 0x4
  
  v1 = default_quoting_options;
  quotearg_n_options(); // tail-call
}

// Function: quotearg_n_mem @ 0x3800
void quotearg_n_mem(void) // injected
{
  char *v1; // stack + 0x4
  
  v1 = default_quoting_options;
  quotearg_n_options(); // tail-call
}

// Function: quotearg @ 0x3830
void quotearg(void) // injected
{
  char *v1; // stack + 0x4
  
  v1 = default_quoting_options;
  quotearg_n_options(); // tail-call
}

// Function: quotearg_mem @ 0x3860
void quotearg_mem(void) // injected
{
  char *v1; // stack + 0x4
  
  v1 = default_quoting_options;
  quotearg_n_options(); // tail-call
}

// Function: quotearg_n_style @ 0x3890
void quotearg_n_style(unsigned int a0,int a1) // injected
{
  int v1 [14]; // stack - 0x40
  int v2; // gs_offset
  unsigned int v3; // stack - 0x8
  
  v3 = 0x3896;
  v1[0xc] = *(int *)(v2 + 0x14);
  if (a1 == 8)
    abort(); // no-return
  v1[0] = a1;
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
  quotearg_n_options(v1);
  if (v1[0xc] == *(int *)(v2 + 0x14))
    return;
  __stack_chk_fail_local(); // no-return
}

// Function: quotearg_n_style_mem @ 0x3950
void quotearg_n_style_mem(unsigned int a0,int a1) // injected
{
  int v1 [14]; // stack - 0x40
  int v2; // gs_offset
  unsigned int v3; // stack - 0x8
  
  v3 = 0x3956;
  v1[0xc] = *(int *)(v2 + 0x14);
  if (a1 == 8)
    abort(); // no-return
  v1[0] = a1;
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
  quotearg_n_options(v1);
  if (v1[0xc] == *(int *)(v2 + 0x14))
    return;
  __stack_chk_fail_local(); // no-return
}

// Function: quotearg_style @ 0x3a10
void quotearg_style(unsigned int a0,unsigned int a1)
{
  quotearg_n_style(0,a0,a1);
}

// Function: quotearg_style_mem @ 0x3a30
void quotearg_style_mem(unsigned int a0,unsigned int a1,unsigned int a2)
{
  quotearg_n_style_mem(0,a0,a1,a2);
}

// Function: quotearg_char_mem @ 0x3a50
void quotearg_char_mem(unsigned int a0,unsigned int a1,unsigned int a2) // injected
{
  unsigned int v1;
  unsigned int v10; // stack - 0x1c
  unsigned int v11; // stack - 0x18
  unsigned int v12; // stack - 0x14
  int v13; // stack - 0x10
  unsigned char v2; // cl
  unsigned int v3; // stack - 0x40
  unsigned int v4 [4]; // stack - 0x38
  int v5; // gs_offset
  unsigned int v6; // stack - 0x3c
  unsigned int v7; // stack - 0x28
  unsigned int v8; // stack - 0x24
  unsigned int v9; // stack - 0x20
  
  v3 = default_quoting_options._0_4_;
  v13 = *(int *)(v5 + 0x14);
  v6 = default_quoting_options._4_4_;
  v4[0] = default_quoting_options._8_4_;
  v4[1] = default_quoting_options._12_4_;
  v4[2] = default_quoting_options._16_4_;
  v4[3] = default_quoting_options._20_4_;
  v7 = default_quoting_options._24_4_;
  v8 = default_quoting_options._28_4_;
  v9 = default_quoting_options._32_4_;
  v10 = default_quoting_options._36_4_;
  v11 = default_quoting_options._40_4_;
  v12 = default_quoting_options._44_4_;
  v2 = (unsigned char)a2 & 0x1f;
  v1 = v4[a2 >> 5 & 7];
  v4[a2 >> 5 & 7] = (~(v1 >> v2) & 1) << v2 ^ v1;
  quotearg_n_options(&v3);
  if (v13 == *(int *)(v5 + 0x14))
    return;
  __stack_chk_fail_local(); // no-return
}

// Function: quotearg_char @ 0x3b40
void quotearg_char(unsigned int a0,char a1)
{
  quotearg_char_mem(a0,0xffffffff,(int)a1);
}

// Function: quotearg_colon @ 0x3b60
void quotearg_colon(unsigned int a0)
{
  quotearg_char_mem(a0,0xffffffff,0x3a);
}

// Function: quotearg_colon_mem @ 0x3b80
void quotearg_colon_mem(unsigned int a0,unsigned int a1)
{
  quotearg_char_mem(a0,a1,0x3a);
}

// Function: quotearg_n_custom_mem @ 0x3ba0
void quotearg_n_custom_mem(unsigned int a0,int a1,int a2) // injected
{
  unsigned int v1; // stack - 0x40
  unsigned int v10; // stack - 0x20
  unsigned int v11; // stack - 0x1c
  int v12; // stack - 0x18
  int v13; // stack - 0x14
  int v14; // stack - 0x10
  unsigned int v15; // stack - 0xc
  int v2; // gs_offset
  unsigned int v3; // stack - 0x3c
  unsigned int v4; // stack - 0x38
  unsigned int v5; // stack - 0x34
  unsigned int v6; // stack - 0x30
  unsigned int v7; // stack - 0x2c
  unsigned int v8; // stack - 0x28
  unsigned int v9; // stack - 0x24
  
  v15 = 0x3ba7;
  v14 = *(int *)(v2 + 0x14);
  v3 = default_quoting_options._4_4_;
  v1 = 8;
  v4 = default_quoting_options._8_4_;
  v5 = default_quoting_options._12_4_;
  v6 = default_quoting_options._16_4_;
  v7 = default_quoting_options._20_4_;
  v8 = default_quoting_options._24_4_;
  v9 = default_quoting_options._28_4_;
  v10 = default_quoting_options._32_4_;
  v11 = default_quoting_options._36_4_;
  v12 = default_quoting_options._40_4_;
  v13 = default_quoting_options._44_4_;
  if ((a1) && (a2)) {
    v12 = a1;
    v13 = a2;
    quotearg_n_options(&v1);
    if (v14 == *(int *)(v2 + 0x14))
      return;
    __stack_chk_fail_local(); // no-return
  }
  abort(); // no-return
}

// Function: quotearg_n_custom @ 0x3c90
void quotearg_n_custom(unsigned int a0,unsigned int a1,unsigned int a2,unsigned int a3)
{
  quotearg_n_custom_mem(a0,a1,a2,a3,0xffffffff);
}

// Function: quotearg_custom @ 0x3cb0
void quotearg_custom(unsigned int a0,unsigned int a1,unsigned int a2)
{
  quotearg_n_custom_mem(0,a0,a1,a2,0xffffffff);
}

// Function: quotearg_custom_mem @ 0x3cd0
void quotearg_custom_mem(unsigned int a0,unsigned int a1,unsigned int a2,unsigned int a3)
{
  quotearg_n_custom_mem(0,a0,a1,a2,a3);
}

// Function: version_etc_arn @ 0x3cf0
void version_etc_arn(void *a0,int a1,int a2,int a3,unsigned int *a4,unsigned int a5) // injected
{
  void *v1;
  int v10;
  int v11;
  char *v12;
  unsigned int v13;
  unsigned int v14; // stack - 0x38
  unsigned int v15; // stack - 0x34
  unsigned int v16; // stack - 0x30
  unsigned int v17; // stack - 0x2c
  unsigned int v18; // stack - 0x28
  unsigned int v19; // stack - 0x24
  unsigned int *v2;
  unsigned int v20; // stack - 0x20
  char *v3; // eax
  unsigned int v4;
  unsigned int v5;
  unsigned int v6;
  unsigned int v7;
  unsigned int v8;
  unsigned int v9;
  
  v13 = a5;
  v2 = a4;
  v1 = a0;
  if (a1) { // branch-flip
    v12 = "%s (%s) %s\n";
    v10 = a1;
    v11 = a2;
  }
  else {
    v12 = "%s %s\n";
    v10 = a2;
    v11 = a3;
  }
  __fprintf_chk(a0,1,v12,v10,v11);
  __fprintf_chk(v1,1,"Copyright %s %d Free Software Foundation, Inc.",dcgettext(NULL,"(C)",5),0x7dc);
  fputs_unlocked(dcgettext(NULL,"\nLicense GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>.\nThis is free software: you are free to change and redistribute it.\nThere is NO WARRANTY, to the extent permitted by law.\n\n",5),v1);
  switch(v13) {
    case 0:
      abort(); // no-return
    case 1:
      v13 = *v2;
      __fprintf_chk(v1,1,dcgettext(NULL,"Written by %s.\n",5),v13);
      return;
    case 2:
      v5 = v2[1];
      v6 = *v2;
      v13 = 5;
      v12 = "Written by %s and %s.\n";
      v3 = dcgettext(NULL,"Written by %s and %s.\n",5);
      break;
    case 3:
      v12 = (char *)v2[2];
      v5 = v2[1];
      v6 = *v2;
      v13 = 5;
      v3 = dcgettext(NULL,"Written by %s, %s, and %s.\n",5);
      break;
    case 4:
      v13 = v2[3];
      v12 = (char *)v2[2];
      v5 = v2[1];
      v6 = *v2;
      v3 = dcgettext(NULL,"Written by %s, %s, %s,\nand %s.\n",5);
      break;
    case 5:
      v13 = v2[2];
      v5 = v2[4];
      v6 = v2[3];
      v9 = v2[1];
      v8 = *v2;
      __fprintf_chk(v1,1,dcgettext(NULL,"Written by %s, %s, %s,\n%s, and %s.\n",5),v8,v9,v13,v6,v5);
      return;
    case 6:
      v8 = v2[2];
      v6 = v2[4];
      v5 = v2[5];
      v9 = v2[3];
      v7 = v2[1];
      v12 = "Written by %s, %s, %s,\n%s, %s, and %s.\n";
      v4 = *v2;
      v13 = 5;
      v3 = dcgettext(NULL,"Written by %s, %s, %s,\n%s, %s, and %s.\n",5);
      __fprintf_chk(v1,1,v3,v4,v7,v8,v9,v6,v5,v12,v13); // return-dupe
      return;
    case 7:
      v6 = v2[4];
      v5 = v2[5];
      v12 = (char *)v2[6];
      v9 = v2[3];
      v8 = v2[2];
      v7 = v2[1];
      v4 = *v2;
      v13 = 5;
      v3 = dcgettext(NULL,"Written by %s, %s, %s,\n%s, %s, %s, and %s.\n",5);
      __fprintf_chk(v1,1,v3,v4,v7,v8,v9,v6,v5,v12,v13);
      return;
    case 8:
      v13 = v2[7];
      v5 = v2[5];
      v12 = (char *)v2[6];
      v6 = v2[4];
      v9 = v2[3];
      v8 = v2[2];
      v7 = v2[1];
      v4 = *v2;
      v3 = dcgettext(NULL,"Written by %s, %s, %s,\n%s, %s, %s, %s,\nand %s.\n",5);
      __fprintf_chk(v1,1,v3,v4,v7,v8,v9,v6,v5,v12,v13);
      return;
    case 9:
      v20 = v2[8];
      v14 = v2[7];
      v19 = v2[6];
      v15 = v2[5];
      v18 = v2[2];
      v16 = v2[4];
      v17 = v2[3];
      v13 = v2[1];
      v5 = *v2;
      v12 = "Written by %s, %s, %s,\n%s, %s, %s, %s,\n%s, and %s.\n";
      __fprintf_chk(v1,1,dcgettext(NULL,v12,5),v5,v13,v18,v17,v16,v15,v19,v14,v20); // return-dupe
      return;
    default:
      v20 = v2[8];
      v14 = v2[7];
      v19 = v2[6];
      v15 = v2[5];
      v18 = v2[2];
      v16 = v2[4];
      v17 = v2[3];
      v13 = v2[1];
      v5 = *v2;
      v12 = "Written by %s, %s, %s,\n%s, %s, %s, %s,\n%s, %s, and others.\n";
      __fprintf_chk(v1,1,dcgettext(NULL,v12,5),v5,v13,v18,v17,v16,v15,v19,v14,v20);
      return;
    
  }
  __fprintf_chk(v1,1,v3,v6,v5,v12,v13);
}

// Function: version_etc_ar @ 0x40d0
void version_etc_ar(unsigned int a0,unsigned int a1,unsigned int a2,unsigned int a3,int *a4)
{
  int v1;
  int v2;
  
  v2 = 0;
  v1 = *a4;
  while (v1) {
    v2 += 1;
    v1 = a4[v2];
  }
  version_etc_arn(a0,a1,a2,a3,a4,v2);
}

// Function: version_etc_va @ 0x4110
void version_etc_va(unsigned int a0,unsigned int a1,unsigned int a2,unsigned int a3,int a4)
{
  int v1;
  int v2; // eax
  int v3 [10];
  int v4; // gs_offset
  int v5; // stack - 0x20
  
  v5 = *(int *)(v4 + 0x14);
  v2 = 0;
  do {
    v1 = *(int *)(a4 + v2 * 4);
    v3[v2] = v1;
    if (!v1) break;
    v2 += 1;
  } while (v2 != 10);
  version_etc_arn(a0,a1,a2,a3,v3,v2);
  if (v5 == *(int *)(v4 + 0x14))
    return;
  __stack_chk_fail_local(); // no-return
}

// Function: version_etc @ 0x4180
void version_etc(unsigned int a0,unsigned int a1,unsigned int a2,unsigned int a3)
{
  int v1;
  int v2; // eax
  int v3 [10];
  int v4; // gs_offset
  int v5; // stack - 0x20
  
  v5 = *(int *)(v4 + 0x14);
  v2 = 0;
  do {
    v1 = *(int *)&(&Stack00000014)[v2 * 4];
    v3[v2] = v1;
    if (!v1) break;
    v2 += 1;
  } while (v2 != 10);
  version_etc_arn(a0,a1,a2,a3,v3,v2);
  if (v5 == *(int *)(v4 + 0x14))
    return;
  __stack_chk_fail_local(); // no-return
}

// Function: emit_bug_reporting_address @ 0x41f0
void emit_bug_reporting_address(void) // injected
{
  void *v1;
  
  __printf_chk(1,dcgettext(NULL,"\nReport bugs to: %s\n",5),"bug-coreutils@gnu.org");
  __printf_chk(1,dcgettext(NULL,"%s home page: <%s>\n",5),"GNU coreutils","http://www.gnu.org/software/coreutils/");
  v1 = (void *)*dat_9fdc;
  fputs_unlocked(dcgettext(NULL,"General help using GNU software: <http://www.gnu.org/gethelp/>\n",5),v1);
}

// Function: xmalloc @ 0x4280
void xmalloc(unsigned int a0) // injected
{
  if ((!malloc(a0)) && (a0))
    xalloc_die(0x4287); // no-return
}

// Function: xrealloc @ 0x42b0
void * xrealloc(void *a0,unsigned int a1) // injected, early-return
{
  void *v1; // eax
  
  if ((!a1) && (a0)) {
    free(a0);
    return NULL;
  }
  v1 = realloc(a0,a1);
  if ((!v1) && (a1))
    xalloc_die(0x42b7); // no-return
  return v1;
}

// Function: x2realloc @ 0x4310
void x2realloc(int a0,unsigned int *a1) // injected
{
  unsigned int v1;
  
  v1 = *a1;
  if (a0) { // branch-flip
    if (0xaaaaaaaa <= v1)
      xalloc_die(); // no-return
    v1 += v1 + 1 >> 1;
  }
  else if (!v1)
    v1 = 0x40;
  *a1 = v1;
  a1 = (unsigned int *)v1;
  xrealloc(); // tail-call
}

// Function: xzalloc @ 0x4370
void xzalloc(unsigned int a0) // injected
{
  unsigned int v1;
  
  v1 = a0;
  memset((void *)xmalloc(a0),0,v1);
}

// Function: xcalloc @ 0x43a0
void xcalloc(unsigned int a0,unsigned int a1) // injected
{
  if (calloc(a0,a1))
    return;
  xalloc_die(); // no-return
}

// Function: xmemdup @ 0x43d0
void xmemdup(void *a0,unsigned int a1) // injected
{
  unsigned int v1;
  
  v1 = a1;
  memcpy((void *)xmalloc(a1),a0,v1);
}

// Function: xstrdup @ 0x4400
void xstrdup(char *a0) // injected
{
  xmemdup(a0,strlen(a0) + 1);
}

// Function: xalloc_die @ 0x4430
void xalloc_die(void) // injected
{
  error(exit_failure,0,"%s",dcgettext(NULL,"memory exhausted",5));
  abort(); // no-return
}

// Function: xstrtoimax @ 0x4470
unsigned int xstrtoimax(char *a0,unsigned int *a1,unsigned int a2,unsigned int *a3,char *a4) // injected
{
  char v1;
  char v10;
  int v11; // edx
  int v12;
  int v13;
  unsigned int *v14;
  int v15; // edx
  unsigned int v16;
  int v17; // gs_offset
  unsigned long long v18;
  unsigned long long v19;
  char *v2;
  unsigned long long v20; // stack - 0x5c
  int v21; // stack - 0x4c
  unsigned int v22; // stack - 0x40
  int v23; // stack - 0x20
  unsigned int v24; // stack - 0x14
  unsigned long long v3;
  long long v4;
  char *v5;
  unsigned int *v6;
  char *v7;
  int *v8; // eax
  unsigned int v9; // eax
  
  v7 = a4;
  v6 = a3;
  v5 = a0;
  v24 = 0x4479;
  v23 = *(int *)(v17 + 0x14);
  if (0x25 <= a2)
    __assert_fail("0 <= strtol_base && strtol_base <= 36","xstrtol.c",0x60,"xstrtoimax"); // no-return
  v14 = a1;
  if (!a1)
    v14 = &v13;
  v8 = __errno_location();
  *v8 = 0;
  v20 = __strtoll_internal(a0,v14,a2,0);
  v2 = (char *)*v14;
  if (v2 != v5) {
    if (*v8) { // branch-flip
      v16 = 4;
      if (*v8 != 0x22) goto label_4516;
      v16 = 1;
    }
    else {
      v16 = 0;
    }
    if ((v7) && (v10 = *v2, v10)) goto label_4595;
    goto label_4506;
  }
  if (!v7) {
    v16 = 4;
    goto label_4516;
  }
  v10 = *v2;
  v16 = 4;
  if ((!v10) || (!strchr(v7,(int)v10))) goto label_4516;
  v20 = 1;
  v16 = 0;
label_4595:
  if (!strchr(v7,(int)v10)) {
    v16 |= 2;
    *v6 = (unsigned int)v20;
    v6[1] = v20._4_4_;
    goto label_4516;
  }
  if (strchr(v7,0x30)) { // branch-flip
    v1 = v2[1];
    if (v1 != 'D') {
      if (v1 == 'i') {
        v21 = (unsigned int)(v2[2] == 'B') * 2 + 1;
        v9 = 0x400;
        goto label_45fc;
      }
      if (v1 != 'B') goto label_45ef;
    }
    v21 = 2;
    v9 = 1000;
  }
  else {
label_45ef:
    v21 = 1;
    v9 = 0x400;
  }
label_45fc:
  switch(v10) {
    case 0x42:
      if (-0x200000 <= (int)v20._4_4_) { // branch-flip
        if (0x200000 <= (int)v20._4_4_) { // branch-flip
label_4e90:
          v16 = 1;
          v20 = 0x7fffffffffffffff;
        }
        else {
          v20 = CONCAT44(v20._4_4_ << 10 | (unsigned int)v20 >> 0x16,(unsigned int)v20 << 10);
        }
      }
      else {
label_4da8:
        v16 = 1;
        v20 = 0x8000000000000000;
      }
      break;
    default:
      v16 |= 2;
      *v6 = (unsigned int)v20;
      v6[1] = v20._4_4_;
      goto label_4516;
    case 0x45:
      v18 = __divdi3(0,0x80000000,v9,0);
      v11 = (int)((unsigned long long)v18 >> 0x20);
      v15 = 6;
      v22 = 0;
      do {
        if ((v11 <= (int)v20._4_4_) && ((v11 < (int)v20._4_4_ || ((unsigned int)v18 <= (unsigned int)v20)))) {
          v19 = __divdi3(0xffffffff,0x7fffffff,v9,0);
          v12 = (int)((unsigned long long)v19 >> 0x20);
          if (((int)v20._4_4_ <= v12) && (((int)v20._4_4_ < v12 || ((unsigned int)v20 <= (unsigned int)v19)))) {
            v3 = (unsigned long long)(unsigned int)v20;
            v20._0_4_ = (unsigned int)(v3 * v9);
            v20._4_4_ = (int)(v3 * v9 >> 0x20) + v9 * v20._4_4_;
          }
          else {
            v22 = 1;
            v20._0_4_ = 0xffffffff;
            v20._4_4_ = 0x7fffffff;
          }
        }
        else {
          v22 = 1;
          v20._0_4_ = 0;
          v20._4_4_ = 0x80000000;
        }
        v15 -= 1;
      } while (v15);
      goto label_4c60;
    case 0x47:
    case 0x67:
      v18 = __divdi3(0,0x80000000,v9,0);
      v11 = (int)((unsigned long long)v18 >> 0x20);
      v15 = 3;
      v22 = 0;
      do {
        if ((v11 <= (int)v20._4_4_) && ((v11 < (int)v20._4_4_ || ((unsigned int)v18 <= (unsigned int)v20)))) {
          v19 = __divdi3(0xffffffff,0x7fffffff,v9,0);
          v12 = (int)((unsigned long long)v19 >> 0x20);
          if (((int)v20._4_4_ <= v12) && (((int)v20._4_4_ < v12 || ((unsigned int)v20 <= (unsigned int)v19)))) {
            v3 = (unsigned long long)(unsigned int)v20;
            v20._0_4_ = (unsigned int)(v3 * v9);
            v20._4_4_ = (int)(v3 * v9 >> 0x20) + v9 * v20._4_4_;
          }
          else {
            v22 = 1;
            v20._0_4_ = 0xffffffff;
            v20._4_4_ = 0x7fffffff;
          }
        }
        else {
          v22 = 1;
          v20._0_4_ = 0;
          v20._4_4_ = 0x80000000;
        }
        v15 -= 1;
      } while (v15);
      v16 |= v22;
      break;
    case 0x4b:
    case 0x6b:
      v18 = __divdi3(0,0x80000000,v9,0);
      v11 = (int)((unsigned long long)v18 >> 0x20);
      if (((int)v20._4_4_ < v11) || (((int)v20._4_4_ <= v11 && ((unsigned int)v20 < (unsigned int)v18)))) goto label_4da8;
      v18 = __divdi3(0xffffffff,0x7fffffff,v9,0);
      v11 = (int)((unsigned long long)v18 >> 0x20);
      if ((v11 < (int)v20._4_4_) || ((v11 <= (int)v20._4_4_ && ((unsigned int)v18 < (unsigned int)v20)))) goto label_4e90;
      v4 = (v20 & 0xffffffff) * (unsigned long long)v9;
      v20 = CONCAT44((int)((unsigned long long)v4 >> 0x20) + v9 * v20._4_4_,(int)v4);
      break;
    case 0x4d:
    case 0x6d:
      v18 = __divdi3(0,0x80000000,v9,0);
      v11 = (int)((unsigned long long)v18 >> 0x20);
      if ((v11 <= (int)v20._4_4_) && ((v11 < (int)v20._4_4_ || ((unsigned int)v18 <= (unsigned int)v20)))) {
        v19 = __divdi3(0xffffffff,0x7fffffff,v9,0);
        v15 = (int)((unsigned long long)v19 >> 0x20);
        if (((int)v20._4_4_ <= v15) && (((int)v20._4_4_ < v15 || ((unsigned int)v20 <= (unsigned int)v19)))) {
          v3 = (v20 & 0xffffffff) * (unsigned long long)v9;
          v12 = (int)(v3 >> 0x20) + v20._4_4_ * v9;
          if ((v12 < v11) || ((v12 <= v11 && ((unsigned int)v3 < (unsigned int)v18)))) goto label_4ed0;
          if ((v12 <= v15) && ((v12 < v15 || ((unsigned int)v3 <= (unsigned int)v19)))) {
            v4 = (v3 & 0xffffffff) * (unsigned long long)v9;
            v20._4_4_ = (int)((unsigned long long)v4 >> 0x20);
            v20._0_4_ = (unsigned int)v4;
            v20 = CONCAT44(v20._4_4_ + v12 * v9,(unsigned int)v20);
            goto label_46d0;
          }
        }
        v20 = 0x7fffffffffffffff;
        v16 = 1;
      }
      else {
label_4ed0:
        v20 = 0x8000000000000000;
        v16 = 1;
      }
      break;
    case 0x50:
      v18 = __divdi3(0,0x80000000,v9,0);
      v11 = (int)((unsigned long long)v18 >> 0x20);
      v15 = 5;
      v22 = 0;
      do {
        if ((v11 <= (int)v20._4_4_) && ((v11 < (int)v20._4_4_ || ((unsigned int)v18 <= (unsigned int)v20)))) {
          v19 = __divdi3(0xffffffff,0x7fffffff,v9,0);
          v12 = (int)((unsigned long long)v19 >> 0x20);
          if (((int)v20._4_4_ <= v12) && (((int)v20._4_4_ < v12 || ((unsigned int)v20 <= (unsigned int)v19)))) {
            v3 = (unsigned long long)(unsigned int)v20;
            v20._0_4_ = (unsigned int)(v3 * v9);
            v20._4_4_ = (int)(v3 * v9 >> 0x20) + v9 * v20._4_4_;
          }
          else {
            v22 = 1;
            v20._0_4_ = 0xffffffff;
            v20._4_4_ = 0x7fffffff;
          }
        }
        else {
          v22 = 1;
          v20._0_4_ = 0;
          v20._4_4_ = 0x80000000;
        }
        v15 -= 1;
      } while (v15);
      goto label_4c60;
    case 0x54:
    case 0x74:
      v18 = __divdi3(0,0x80000000,v9,0);
      v11 = (int)((unsigned long long)v18 >> 0x20);
      v22 = 0;
      v15 = 4;
      do {
        if ((v11 <= (int)v20._4_4_) && ((v11 < (int)v20._4_4_ || ((unsigned int)v18 <= (unsigned int)v20)))) {
          v19 = __divdi3(0xffffffff,0x7fffffff,v9,0);
          v12 = (int)((unsigned long long)v19 >> 0x20);
          if (((int)v20._4_4_ <= v12) && (((int)v20._4_4_ < v12 || ((unsigned int)v20 <= (unsigned int)v19)))) {
            v3 = (unsigned long long)(unsigned int)v20;
            v20._0_4_ = (unsigned int)(v3 * v9);
            v20._4_4_ = (int)(v3 * v9 >> 0x20) + v9 * v20._4_4_;
          }
          else {
            v22 = 1;
            v20._0_4_ = 0xffffffff;
            v20._4_4_ = 0x7fffffff;
          }
        }
        else {
          v22 = 1;
          v20._0_4_ = 0;
          v20._4_4_ = 0x80000000;
        }
        v15 -= 1;
      } while (v15);
      goto label_4c60;
    case 0x59:
      v18 = __divdi3(0,0x80000000,v9,0);
      v11 = (int)((unsigned long long)v18 >> 0x20);
      v15 = 8;
      v22 = 0;
      do {
        if ((v11 <= (int)v20._4_4_) && ((v11 < (int)v20._4_4_ || ((unsigned int)v18 <= (unsigned int)v20)))) {
          v19 = __divdi3(0xffffffff,0x7fffffff,v9,0);
          v12 = (int)((unsigned long long)v19 >> 0x20);
          if (((int)v20._4_4_ <= v12) && (((int)v20._4_4_ < v12 || ((unsigned int)v20 <= (unsigned int)v19)))) {
            v3 = (unsigned long long)(unsigned int)v20;
            v20._0_4_ = (unsigned int)(v3 * v9);
            v20._4_4_ = (int)(v3 * v9 >> 0x20) + v9 * v20._4_4_;
          }
          else {
            v22 = 1;
            v20._0_4_ = 0xffffffff;
            v20._4_4_ = 0x7fffffff;
          }
        }
        else {
          v22 = 1;
          v20._0_4_ = 0;
          v20._4_4_ = 0x80000000;
        }
        v15 -= 1;
      } while (v15);
      goto label_4c60;
    case 0x5a:
      v18 = __divdi3(0,0x80000000,v9,0);
      v11 = (int)((unsigned long long)v18 >> 0x20);
      v15 = 7;
      v22 = 0;
      do {
        if ((v11 <= (int)v20._4_4_) && ((v11 < (int)v20._4_4_ || ((unsigned int)v18 <= (unsigned int)v20)))) {
          v19 = __divdi3(0xffffffff,0x7fffffff,v9,0);
          v12 = (int)((unsigned long long)v19 >> 0x20);
          if (((int)v20._4_4_ <= v12) && (((int)v20._4_4_ < v12 || ((unsigned int)v20 <= (unsigned int)v19)))) {
            v3 = (unsigned long long)(unsigned int)v20;
            v20._0_4_ = (unsigned int)(v3 * v9);
            v20._4_4_ = (int)(v3 * v9 >> 0x20) + v9 * v20._4_4_;
          }
          else {
            v22 = 1;
            v20._0_4_ = 0xffffffff;
            v20._4_4_ = 0x7fffffff;
          }
        }
        else {
          v22 = 1;
          v20._0_4_ = 0;
          v20._4_4_ = 0x80000000;
        }
        v15 -= 1;
      } while (v15);
label_4c60:
      v16 |= v22;
      break;
    case 0x62:
      if ((int)v20._4_4_ < -0x400000) goto label_4da8;
      if (0x3fffff < (int)v20._4_4_) goto label_4e90;
      v20 = CONCAT44(v20._4_4_ << 9 | (unsigned int)v20 >> 0x17,(unsigned int)v20 << 9);
      break;
    case 99:
      break;
    case 0x77:
      if ((int)v20._4_4_ < -0x40000000) goto label_4da8;
      if (0x3fffffff < (int)v20._4_4_) goto label_4e90;
      v20 = CONCAT44(v20._4_4_ << 1 | (unsigned int)v20 >> 0x1f,(unsigned int)v20 * 2);
    
  }
label_46d0:
  *v14 = &v2[v21];
  if (v2[v21])
    v16 |= 2;
label_4506:
  *v6 = (unsigned int)v20;
  v6[1] = v20._4_4_;
label_4516:
  if (v23 == *(int *)(v17 + 0x14))
    return v16;
  __stack_chk_fail_local(); // no-return
}

// Function: xstrtol @ 0x4f30
unsigned int xstrtol(char *a0,char **a1,unsigned int a2,int *a3,char *a4) // injected
{
  char v1;
  unsigned int v10;
  unsigned int v11; // ebp
  int v12; // gs_offset
  int v13; // stack - 0x40
  int v14; // stack - 0x3c
  unsigned int v15; // stack - 0x34
  unsigned int v16; // stack - 0x30
  int v17; // stack - 0x20
  unsigned int v18; // stack - 0x14
  char *v2;
  int *v3; // eax
  int v4;
  int v5; // eax
  char v6;
  int v7; // eax
  int v8;
  char **v9;
  
  v18 = 0x4f39;
  v17 = *(int *)(v12 + 0x14);
  if (0x25 <= a2)
    __assert_fail("0 <= strtol_base && strtol_base <= 36","xstrtol.c",0x60,"xstrtol"); // no-return
  v9 = a1;
  if (!a1)
    v9 = &v8;
  v3 = __errno_location();
  *v3 = 0;
  v4 = strtol(a0,v9,a2);
  v2 = *v9;
  if (v2 != a0) {
    if (*v3) { // branch-flip
      v10 = 4;
      if (*v3 != 0x22) goto label_4fc3;
      v10 = 1;
    }
    else {
      v10 = 0;
    }
    if ((a4) && (v6 = *v2, v6)) goto label_503c;
    goto label_4fbd;
  }
  if (!a4) {
    v10 = 4;
    goto label_4fc3;
  }
  v6 = *v2;
  v10 = 4;
  if ((!v6) || (!strchr(a4,(int)v6))) goto label_4fc3;
  v4 = 1;
  v10 = 0;
label_503c:
  if (!strchr(a4,(int)v6)) {
label_50d0:
    v10 |= 2;
    *a3 = v4;
    goto label_4fc3;
  }
  if (strchr(a4,0x30)) { // branch-flip
    v1 = v2[1];
    if (v1 != 'D') {
      if (v1 == 'i') {
        v13 = 0x400;
        v14 = (unsigned int)(v2[2] == 'B') * 2 + 1;
        goto label_50b6;
      }
      if (v1 != 'B') goto label_50a6;
    }
    v14 = 2;
    v13 = 1000;
  }
  else {
label_50a6:
    v14 = 1;
    v13 = 0x400;
  }
label_50b6:
  switch(v6) {
    case 0x42:
      if (-0x200000 <= v4) { // branch-flip
        if (0x200000 <= v4) { // branch-flip
label_54f0:
          v10 = 1;
          v4 = 0x7fffffff;
        }
        else {
          v4 <<= 10;
        }
      }
      else {
label_5440:
        v10 = 1;
        v4 = -0x80000000;
      }
      break;
    default:
      goto label_50d0;
    case 0x45:
      v7 = 6;
      v15 = 0;
      do {
        if ((int)(-0x80000000 / (long long)v13) <= v4) { // branch-flip
          if (v4 <= (int)(0x7fffffff / (unsigned long long)(long long)v13)) // branch-flip
            v4 *= v13;
          else {
            v15 = 1;
            v4 = 0x7fffffff;
          }
        }
        else {
          v15 = 1;
          v4 = -0x80000000;
        }
        v7 -= 1;
      } while (v7);
      v10 |= v15;
      break;
    case 0x47:
    case 0x67:
      v7 = 3;
      v16 = 0;
      do {
        if ((int)(-0x80000000 / (long long)v13) <= v4) { // branch-flip
          if (v4 <= (int)(0x7fffffff / (unsigned long long)(long long)v13)) // branch-flip
            v4 *= v13;
          else {
            v16 = 1;
            v4 = 0x7fffffff;
          }
        }
        else {
          v16 = 1;
          v4 = -0x80000000;
        }
        v7 -= 1;
      } while (v7);
      goto label_5271;
    case 0x4b:
    case 0x6b:
      if (v4 < (int)(-0x80000000 / (long long)v13)) goto label_5440;
      if ((int)(0x7fffffff / (unsigned long long)(long long)v13) < v4) goto label_54f0;
      v4 *= v13;
      break;
    case 0x4d:
    case 0x6d:
      v7 = (int)(-0x80000000 / (long long)v13);
      if (v7 <= v4) { // branch-flip
        v5 = (int)(0x7fffffff / (unsigned long long)(long long)v13);
        if (v4 <= v5) {
          v4 *= v13;
          if (v4 < v7) goto label_5552;
          if (v4 <= v5) {
            v4 *= v13;
            goto label_5110;
          }
        }
        v4 = 0x7fffffff;
        v10 = 1;
      }
      else {
label_5552:
        v4 = -0x80000000;
        v10 = 1;
      }
      break;
    case 0x50:
      v7 = 5;
      v16 = 0;
      do {
        if ((int)(-0x80000000 / (long long)v13) <= v4) { // branch-flip
          if (v4 <= (int)(0x7fffffff / (unsigned long long)(long long)v13)) // branch-flip
            v4 *= v13;
          else {
            v16 = 1;
            v4 = 0x7fffffff;
          }
        }
        else {
          v16 = 1;
          v4 = -0x80000000;
        }
        v7 -= 1;
      } while (v7);
      goto label_5271;
    case 0x54:
    case 0x74:
      v7 = 4;
      v16 = 0;
      do {
        if ((int)(-0x80000000 / (long long)v13) <= v4) { // branch-flip
          if (v4 <= (int)(0x7fffffff / (unsigned long long)(long long)v13)) // branch-flip
            v4 *= v13;
          else {
            v16 = 1;
            v4 = 0x7fffffff;
          }
        }
        else {
          v16 = 1;
          v4 = -0x80000000;
        }
        v7 -= 1;
      } while (v7);
label_5271:
      v10 |= v16;
      break;
    case 0x59:
      v7 = 8;
      v11 = 0;
      do {
        if ((int)(-0x80000000 / (long long)v13) <= v4) { // branch-flip
          if (v4 <= (int)(0x7fffffff / (unsigned long long)(long long)v13)) // branch-flip
            v4 *= v13;
          else {
            v11 = 1;
            v4 = 0x7fffffff;
          }
        }
        else {
          v11 = 1;
          v4 = -0x80000000;
        }
        v7 -= 1;
      } while (v7);
      v10 |= v11;
      break;
    case 0x5a:
      v7 = 7;
      v11 = 0;
      do {
        if ((int)(-0x80000000 / (long long)v13) <= v4) { // branch-flip
          if (v4 <= (int)(0x7fffffff / (unsigned long long)(long long)v13)) // branch-flip
            v4 *= v13;
          else {
            v11 = 1;
            v4 = 0x7fffffff;
          }
        }
        else {
          v11 = 1;
          v4 = -0x80000000;
        }
        v7 -= 1;
      } while (v7);
      v10 |= v11;
      break;
    case 0x62:
      if (v4 < -0x400000) goto label_5440;
      if (0x3fffff < v4) goto label_54f0;
      v4 <<= 9;
      break;
    case 99:
      break;
    case 0x77:
      if (v4 < -0x40000000) goto label_5440;
      if (0x3fffffff < v4) goto label_54f0;
      v4 *= 2;
    
  }
label_5110:
  *v9 = &v2[v14];
  if (v2[v14])
    v10 |= 2;
label_4fbd:
  *a3 = v4;
label_4fc3:
  if (v17 == *(int *)(v12 + 0x14))
    return v10;
  __stack_chk_fail_local(); // no-return
}

// Function: rpl_fclose @ 0x55a0
int rpl_fclose(void *a0) // injected
{
  void *v1;
  int v2;
  int *v3; // eax
  int v4; // eax
  unsigned long long v5;
  
  v1 = a0;
  if (0 <= fileno(a0)) {
    if (__freading(a0)) { // branch-flip
      v5 = lseek64(fileno(a0),0,0,1);
      if (((unsigned int)v5 & (unsigned int)((unsigned long long)v5 >> 0x20)) == 0xffffffff) {
        return fclose(v1); // return-dupe
      }
      v2 = rpl_fflush(v1);
    }
    else {
      v2 = rpl_fflush(a0);
    }
    if (v2) {
      v3 = __errno_location();
      v2 = *v3;
      v4 = fclose(v1);
      if (!v2)
        return v4;
      *v3 = v2;
      return -1;
    }
  }
  return fclose(v1);
}

// Function: rpl_fflush @ 0x5660
void rpl_fflush(unsigned int *a0) // injected
{
  unsigned int *v1;
  
  v1 = a0;
  if (a0) {
    if ((__freading(a0)) && (*a0 & 0x100))
      rpl_fseeko(a0,0,0,1,0x5667);
  }
  fflush(v1);
}

// Function: rpl_fseeko @ 0x56c0
unsigned int rpl_fseeko(unsigned int *a0,unsigned int a1,unsigned int a2,unsigned int a3) // injected, early-return x2
{
  unsigned int *v1;
  unsigned int v2; // edx
  unsigned long long v3;
  
  v1 = a0;
  if (((a0[2] == a0[1]) && (a0[5] == a0[4])) && (!a0[9])) {
    v3 = lseek64(fileno(a0),a1,a2,a3);
    v2 = (unsigned int)((unsigned long long)v3 >> 0x20);
    if ((v2 & (unsigned int)v3) != 0xffffffff) {
      v1[0x13] = (unsigned int)v3;
      *v1 = *v1 & 0xffffffef;
      v1[0x14] = v2;
      return 0;
    }
    return 0xffffffff;
  }
  return fseeko64(a0,a1,a2,a3);
}

// Function: close_stream @ 0x5750
int close_stream(unsigned int *a0) // injected, early-return
{
  unsigned int v1;
  unsigned int v2; // eax
  int v3; // eax
  
  v2 = __fpending(a0);
  v1 = *a0;
  v3 = rpl_fclose(a0);
  if (v1 & 0x20) {
    if (v3)
      return -1;
    *__errno_location() = 0;
    return -1;
  }
  if (!v3)
    return 0;
  if (v2)
    return -1;
  return -(unsigned int)(*__errno_location() != 9);
}

// Function: __divdi3 @ 0x57f0
unsigned long long __divdi3(unsigned int a0,unsigned int a1,unsigned int a2,unsigned int a3)
{
  unsigned long long v1;
  bool v10; // cf
  unsigned int v11; // stack - 0x2c
  unsigned int v12; // stack - 0x24
  unsigned long long v2;
  unsigned long long v3;
  long long v4;
  unsigned char v5;
  int v6;
  unsigned char v7; // cl
  unsigned int v8; // edx
  unsigned int v9;
  
  v12 = 0;
  v11 = a0;
  if ((int)a1 <= -1) {
    v11 = -a0;
    v12 = 0xffffffff;
    a1 = -(a1 + (a0 != 0));
  }
  if ((int)a3 <= -1) {
    v10 = a2 != 0;
    a2 = -a2;
    v12 = ~v12;
    a3 = -(a3 + v10);
  }
  if (a3) { // branch-flip
    if (a3 <= a1) { // branch-flip
      v9 = 0x1f;
      if (a3) {
        for (; !(a3 >> v9); v9 = v9 - 1) {
        }
      }
      if (v9 ^ 0x1f) { // branch-flip
        v5 = (unsigned char)(v9 ^ 0x1f);
        v7 = 0x20 - v5;
        v1 = (unsigned long long)(a2 >> (v7 & 0x1f) | a3 << (v5 & 0x1f));
        v2 = CONCAT44(a1 >> (v7 & 0x1f),a1 << (v5 & 0x1f) | v11 >> (v7 & 0x1f));
        v3 = v2 / v1;
        v6 = (int)v3;
        v9 = (unsigned int)(v2 % v1);
        v4 = (v3 & 0xffffffff) * (unsigned long long)(a2 << (v5 & 0x1f));
        v8 = (unsigned int)((unsigned long long)v4 >> 0x20);
        if ((v8 <= v9) && (((unsigned int)v4 <= v11 << (v5 & 0x1f) || (v9 != v8))))
          v9 = 0;
        else {
          v6 -= 1;
          v9 = 0;
        }
      }
      else {
        v9 = 0;
        if ((a3 < a1) || (v6 = 0, a2 <= v11))
          v6 = 1;
      }
    }
    else {
      v9 = 0;
      v6 = 0;
    }
  }
  else if (a2 <= a1) { // branch-flip
    if (!a2)
      a2 = (unsigned int)(1 / 0);
    v9 = a1 / a2;
    v6 = (int)(((unsigned long long)a1 % (unsigned long long)a2 << 0x20 | (unsigned long long)v11) / (unsigned long long)a2);
  }
  else {
    v9 = 0;
    v6 = (int)(CONCAT44(a1,v11) / (unsigned long long)a2);
  }
  if (v12) {
    v10 = v6 != 0;
    v6 = -v6;
    v9 = -(v9 + v10);
  }
  return CONCAT44(v9,v6);
}

// Function: __udivdi3 @ 0x5940
unsigned long long __udivdi3(unsigned int a0,unsigned int a1,unsigned int a2,unsigned int a3)
{
  unsigned long long v1;
  unsigned long long v2;
  unsigned long long v3;
  long long v4;
  unsigned char v5;
  unsigned int v6;
  unsigned char v7; // cl
  unsigned int v8; // edx
  unsigned int v9; // edx
  
  if (!a3) {
    if (a2 > a1)
      return CONCAT44(a1,a0) / (unsigned long long)a2 & 0xffffffff;
    if (!a2)
      a2 = (unsigned int)(1 / 0);
    return CONCAT44(a1 / a2,(int)(((unsigned long long)a1 % (unsigned long long)a2 << 0x20 | (unsigned long long)a0) / (unsigned long long)a2));
  }
  if (a3 <= a1) { // branch-flip
    v6 = 0x1f;
    if (a3) {
      for (; !(a3 >> v6); v6 = v6 - 1) {
      }
    }
    if (v6 ^ 0x1f) { // branch-flip
      v5 = (unsigned char)(v6 ^ 0x1f);
      v7 = 0x20 - v5;
      v1 = (unsigned long long)(a3 << (v5 & 0x1f) | a2 >> (v7 & 0x1f));
      v2 = CONCAT44(a1 >> (v7 & 0x1f),a1 << (v5 & 0x1f) | a0 >> (v7 & 0x1f));
      v3 = v2 / v1;
      v6 = (unsigned int)v3;
      v8 = (unsigned int)(v2 % v1);
      v4 = (v3 & 0xffffffff) * (unsigned long long)(a2 << (v5 & 0x1f));
      v9 = (unsigned int)((unsigned long long)v4 >> 0x20);
      if ((v8 < v9) || ((a0 << (v5 & 0x1f) < (unsigned int)v4 && (v8 == v9))))
        return (unsigned long long)(v6 - 1);
    }
    else if ((a3 < a1) || (v6 = 0, a2 <= a0))
      v6 = 1;
  }
  else {
    v6 = 0;
  }
  return (unsigned long long)v6;
}

// Function: __libc_csu_init @ 0x5a60
void __libc_csu_init(unsigned int a0,unsigned int a1,unsigned int a2) // injected
{
  unsigned int v1;
  int v2; // edi
  
  v1 = a2;
  _init();
  v2 = 0;
  do {
    (**(void **)(v2 * 4 + 0x9cb8))(a0,a1,v1);
    v2 += 1;
  } while (v2 != 1);
}

// Function: __libc_csu_fini @ 0x5ac0
void __libc_csu_fini(void)
{
  return;
}

// Function: atexit @ 0x5ad0
void atexit(unsigned int a0) // injected
{
  __cxa_atexit(a0,0,dat_a004);
}

// Function: __stack_chk_fail_local @ 0x5b00
void __stack_chk_fail_local(void) // injected
{
  __stack_chk_fail(); // no-return
}

// Function: _fini @ 0x5b14
void _fini(void) // injected
{
  return;
}

