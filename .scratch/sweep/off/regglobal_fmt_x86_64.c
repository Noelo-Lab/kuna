// Function: _init @ 0x1570
void _init(void) // return-dupe
{
  if (!dat_209fe8)
    return;
  (*dat_209fe8)();
}

// Function: sub_1590 @ 0x1590
void sub_1590(void)
{
  (*dat_209e38)(); // jump-as-call
}

// Function: __uflow @ 0x15a0
void __uflow(void)
{
  (*dat_209e40)(); // jump-as-call
}

// Function: free @ 0x15b0
void free(void *a0)
{
  (*dat_209e48)(); // jump-as-call
}

// Function: abort @ 0x15c0
void abort(void)
{
  (*dat_209e50)(); // jump-as-call
}

// Function: __errno_location @ 0x15d0
int * __errno_location(void)
{
  int *v1; // rax
  
  v1 = (int *)(*dat_209e58)(); // jump-as-call
  return v1;
}

// Function: strncmp @ 0x15e0
int strncmp(char *a0,char *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_209e60)(); // jump-as-call
  return v1;
}

// Function: _exit @ 0x15f0
void _exit(int a0)
{
  (*dat_209e68)(); // jump-as-call
}

// Function: __fpending @ 0x1600
unsigned long __fpending(void *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_209e70)(); // jump-as-call
  return v1;
}

// Function: reallocarray @ 0x1610
void * reallocarray(void *a0,unsigned long a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_209e78)(); // jump-as-call
  return v1;
}

// Function: clearerr_unlocked @ 0x1620
void clearerr_unlocked(void *a0)
{
  (*dat_209e80)(); // jump-as-call
}

// Function: textdomain @ 0x1630
char * textdomain(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_209e88)(); // jump-as-call
  return v1;
}

// Function: fclose @ 0x1640
int fclose(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_209e90)(); // jump-as-call
  return v1;
}

// Function: bindtextdomain @ 0x1650
char * bindtextdomain(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_209e98)(); // jump-as-call
  return v1;
}

// Function: dcgettext @ 0x1660
char * dcgettext(char *a0,char *a1,int a2)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_209ea0)(); // jump-as-call
  return v1;
}

// Function: __ctype_get_mb_cur_max @ 0x1670
unsigned long __ctype_get_mb_cur_max(void)
{
  unsigned long v1; // rax
  
  v1 = (*dat_209ea8)(); // jump-as-call
  return v1;
}

// Function: strlen @ 0x1680
unsigned long strlen(char *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_209eb0)(); // jump-as-call
  return v1;
}

// Function: __stack_chk_fail @ 0x1690
void __stack_chk_fail(void)
{
  (*dat_209eb8)(); // jump-as-call
}

// Function: getopt_long @ 0x16a0
int getopt_long(int a0,char **a1,char *a2,void *a3,int *a4)
{
  int v1; // eax
  
  v1 = (*dat_209ec0)(); // jump-as-call
  return v1;
}

// Function: mbrtowc @ 0x16b0
unsigned long mbrtowc(void *a0,char *a1,unsigned long a2,void *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_209ec8)(); // jump-as-call
  return v1;
}

// Function: strchr @ 0x16c0
char * strchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_209ed0)(); // jump-as-call
  return v1;
}

// Function: __overflow @ 0x16d0
int __overflow(void *a0,int a1)
{
  int v1; // eax
  
  v1 = (*dat_209ed8)(); // jump-as-call
  return v1;
}

// Function: strrchr @ 0x16e0
char * strrchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_209ee0)(); // jump-as-call
  return v1;
}

// Function: lseek @ 0x16f0
void lseek(void)
{
  (*dat_209ee8)(); // jump-as-call
}

// Function: __assert_fail @ 0x1700
void __assert_fail(char *a0,char *a1,unsigned int a2,char *a3)
{
  (*dat_209ef0)(); // jump-as-call
}

// Function: __strtoul_internal @ 0x1710
void __strtoul_internal(void)
{
  (*dat_209ef8)(); // jump-as-call
}

// Function: memset @ 0x1720
void * memset(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_209f00)(); // jump-as-call
  return v1;
}

// Function: posix_fadvise @ 0x1730
void posix_fadvise(void)
{
  (*dat_209f08)(); // jump-as-call
}

// Function: memcmp @ 0x1740
int memcmp(void *a0,void *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_209f10)(); // jump-as-call
  return v1;
}

// Function: fputs_unlocked @ 0x1750
int fputs_unlocked(char *a0,void *a1)
{
  int v1; // eax
  
  v1 = (*dat_209f18)(); // jump-as-call
  return v1;
}

// Function: calloc @ 0x1760
void * calloc(unsigned long a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_209f20)(); // jump-as-call
  return v1;
}

// Function: fputc_unlocked @ 0x1770
void fputc_unlocked(void)
{
  (*dat_209f28)(); // jump-as-call
}

// Function: __memmove_chk @ 0x1780
void * __memmove_chk(void *a0,void *a1,unsigned long a2,unsigned long a3)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_209f30)(); // jump-as-call
  return v1;
}

// Function: memcpy @ 0x1790
void * memcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_209f38)(); // jump-as-call
  return v1;
}

// Function: fileno @ 0x17a0
int fileno(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_209f40)(); // jump-as-call
  return v1;
}

// Function: malloc @ 0x17b0
void * malloc(unsigned long a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_209f48)(); // jump-as-call
  return v1;
}

// Function: fflush @ 0x17c0
int fflush(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_209f50)(); // jump-as-call
  return v1;
}

// Function: nl_langinfo @ 0x17d0
char * nl_langinfo(int a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_209f58)(); // jump-as-call
  return v1;
}

// Function: __freading @ 0x17e0
int __freading(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_209f60)(); // jump-as-call
  return v1;
}

// Function: fwrite_unlocked @ 0x17f0
unsigned long fwrite_unlocked(void *a0,unsigned long a1,unsigned long a2,void *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_209f68)(); // jump-as-call
  return v1;
}

// Function: realloc @ 0x1800
void * realloc(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_209f70)(); // jump-as-call
  return v1;
}

// Function: setlocale @ 0x1810
char * setlocale(int a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_209f78)(); // jump-as-call
  return v1;
}

// Function: __printf_chk @ 0x1820
int __printf_chk(int a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_209f80)(); // jump-as-call
  return v1;
}

// Function: error @ 0x1830
void error(int a0,int a1,char *a2,...)
{
  (*dat_209f88)(); // jump-as-call
}

// Function: fseeko @ 0x1840
void fseeko(void)
{
  (*dat_209f90)(); // jump-as-call
}

// Function: fopen @ 0x1850
void * fopen(char *a0,char *a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_209f98)(); // jump-as-call
  return v1;
}

// Function: __cxa_atexit @ 0x1860
void __cxa_atexit(void)
{
  (*dat_209fa0)(); // jump-as-call
}

// Function: exit @ 0x1870
void exit(int a0)
{
  (*dat_209fa8)(); // jump-as-call
}

// Function: fwrite @ 0x1880
unsigned long fwrite(void *a0,unsigned long a1,unsigned long a2,void *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_209fb0)(); // jump-as-call
  return v1;
}

// Function: __fprintf_chk @ 0x1890
int __fprintf_chk(void *a0,int a1,char *a2,...)
{
  int v1; // eax
  
  v1 = (*dat_209fb8)(); // jump-as-call
  return v1;
}

// Function: mbsinit @ 0x18a0
int mbsinit(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_209fc0)(); // jump-as-call
  return v1;
}

// Function: iswprint @ 0x18b0
int iswprint(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_209fc8)(); // jump-as-call
  return v1;
}

// Function: __ctype_b_loc @ 0x18c0
void * __ctype_b_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_209fd0)(); // jump-as-call
  return v1;
}

// Function: __cxa_finalize @ 0x18d0
void __cxa_finalize(void)
{
  (*dat_209ff8)(); // jump-as-call
}

// Function: quotearg_buffer_restyled.cold.3 @ 0x18e0
void quotearg_buffer_restyled.cold.3(void)
{
  abort(); // no-return
}

// Function: quotearg_n_style.cold.4 @ 0x18e5
void quotearg_n_style.cold.4(void)
{
  abort(); // no-return
}

// Function: quotearg_n_style_mem.cold.5 @ 0x18ea
void quotearg_n_style_mem.cold.5(void)
{
  abort(); // no-return
}

// Function: quotearg_n_style_colon.cold.6 @ 0x18ef
void quotearg_n_style_colon.cold.6(void)
{
  abort(); // no-return
}

// Function: main @ 0x1900
int main(int argc,char **argv)
{
  _IO_FILE *f; // rax
  unsigned char *file;
  unsigned char *v1;
  char *v10;
  long v11; // rcx
  unsigned char *v12;
  unsigned char *v13;
  char *v14;
  bool v15; // cf
  bool v16;
  unsigned char *v2;
  bool v3;
  unsigned int v4; // eax
  unsigned int v5;
  int v6; // eax
  int v7; // eax
  char *v8; // rax
  char *v9;
  
  v9 = NULL;
  set_program_name(*argv);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  atexit(close_stdout);
  uniform = 0;
  split = 0;
  tagged = 0;
  crown = 0;
  max_width = 0x4b;
  prefix = "";
  prefix_full_length = 0;
  prefix_lead_space = 0;
  prefix_length = 0;
  if (((2 <= argc) && (v10 = argv[1], *v10 == '-')) && ((unsigned int)((int)v10[1] - 0x30U) <= 9)) {
    v9 = &v10[1];
    v10 = *argv;
    argc -= 1;
    argv = &argv[1];
    *argv = v10;
  }
  v10 = NULL;
  while (v4 = getopt_long(argc,argv,"0123456789cstuw:p:g:",long_options,NULL), v4 != 0xffffffff) {
    switch(v4) {
      default:
        if (v4 - 0x30 <= 9)
          error(0,0,dcgettext(NULL,"invalid option -- %c; -WIDTH is recognized only when it is the first\noption; use -w N instead",5),(unsigned long)v4);
        usage(1); // no-return
      case 99:
        crown = 1;
        break;
      case 0x67:
        v10 = optarg;
        break;
      case 0x70:
        prefix_lead_space = 0;
        v14 = optarg;
        if (*optarg == ' ') {
          do {
            prefix_lead_space = (1U - (int)optarg) + (int)v14;
            v14 = &v14[1];
          } while (*v14 == ' ');
        }
        prefix = v14;
        prefix_full_length = (int)strlen(v14);
        v8 = &v14[prefix_full_length];
        if (v14 < v8) {
          do {
            if (v8[-1] != ' ') break;
            v8 = &v8[-1];
          } while (v14 != v8);
        }
        *v8 = '\0';
        prefix_length = (int)v8 - (int)v14;
        break;
      case 0x73:
        split = 1;
        break;
      case 0x74:
        tagged = 1;
        break;
      case 0x75:
        uniform = 1;
        break;
      case 0x77:
        v9 = optarg;
        break;
      case -0x83:
        version_etc(stdout,"fmt","GNU coreutils",Version,"Ross Paterson",0);
        exit(0); // no-return
      case -0x82:
        usage(0); // no-return
      
    }
  }
  if (v9) { // branch-flip
    v14 = dcgettext(NULL,"invalid width",5);
    v6 = (int)xdectoumax(v9,0,0x9c4,"",v14,0);
    max_width = v6;
    if (v10) { // branch-flip
      v9 = dcgettext(NULL,"invalid width",5);
      goal_width = (int)xdectoumax(v10,0,(long)v6,"",v9,0);
    }
    else {
label_1d19:
      goal_width = (max_width * 0xbb) / 200;
    }
  }
  else {
    if (!v10) goto label_1d19;
    v9 = dcgettext(NULL,"invalid width",5);
    goal_width = (int)xdectoumax(v10,0,0x4b,"",v9,0);
    max_width = goal_width + 10;
  }
  if (optind != argc) { // branch-flip
    v4 = 1;
    if (argc <= (int)optind) {
      return v4 ^ 1; // return-dupe
    }
    v15 = 0;
    v3 = 0;
    v16 = 1;
    v4 = 1;
    v5 = optind;
    do {
      v11 = 2;
      file = (unsigned char *)argv[(int)v5];
      v12 = file;
      v13 = (unsigned char *)"-";
      do {
        if (!v11) break;
        v11 -= 1;
        v2 = &v13[1];
        v1 = &v12[1];
        v15 = *v12 < *v13;
        v16 = *v12 == *v13;
        v12 = v1;
        v13 = v2;
      } while (v16);
      if ((!v15 && !v16) != v15) { // branch-flip
        f = fopen((char *)file,"r");
        if (f) // branch-flip
          v4 &= fmt(f,(char *)file);
        else {
          v9 = quotearg_style(shell_escape_always_quoting_style,(char *)file);
          v10 = dcgettext(NULL,"cannot open %s for reading",5);
          v4 = 0;
          error(0,*__errno_location(),v10,v9);
        }
      }
      else {
        v3 = 1;
        v16 = fmt(stdin,(char *)file);
        v4 &= v16;
      }
      v5 = optind + 1;
      v15 = v5 < (unsigned int)argc;
      v16 = v5 == argc;
      optind = v5;
    } while ((int)v5 < argc);
    if (!v3) {
      return v4 ^ 1;
    }
  }
  else {
    v16 = fmt(stdin,"-");
    v4 = (unsigned int)v16;
  }
  v6 = rpl_fclose(stdin);
  if (v6) {
    v9 = dcgettext(NULL,"closing standard input",5);
    error(1,*__errno_location(),"%s",v9);
    return v7;
  }
  return v4 ^ 1;
}

// Function: _start @ 0x1e10
void _start(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_209fe0)(main,v2,&Stack0000000000000008,__libc_csu_init,__libc_csu_fini,a2,v1);
  do {
  } while( true );
}

// Function: deregister_tm_clones @ 0x1e40
void deregister_tm_clones(void)
{
  return;
}

// Function: register_tm_clones @ 0x1e70
void register_tm_clones(void)
{
  return;
}

// Function: __do_global_dtors_aux @ 0x1eb0
void __do_global_dtors_aux(void)
{
  if (!completed.7997) {
    if (dat_209ff8)
      __cxa_finalize(dat_20a008);
    deregister_tm_clones();
    completed.7997 = 1;
    return;
  }
}

// Function: frame_dummy @ 0x1ef0
void frame_dummy(void)
{
  register_tm_clones(); // tail-call
}

// Function: fmt_paragraph @ 0x1f00
void fmt_paragraph(void)
{
  unsigned char v1;
  long v10;
  long v11; // r14
  int v12;
  unsigned int v2;
  char v3 [16];
  char v4 [16];
  char *v5;
  char *v6;
  long v7;
  char *v8; // rdx
  char *v9;
  
  v2 = *(unsigned int *)&word_limit[8];
  *(unsigned long *)&word_limit[0x18] = 0;
  *(int *)&word_limit[8] = max_width;
  v6 = word_limit;
  while (v5 = &v6[-0x28], (char *)0x20a120 <= v5) {
    v12 = other_indent;
    if (v5 == unused_word_type)
      v12 = first_indent;
    v10 = 0x7fffffffffffffff;
    v12 += *(int *)&v6[-0x20];
    v9 = v5;
    do {
      v8 = &v9[0x28];
      if (word_limit != v8) { // branch-flip
        v7 = (long)((goal_width - v12) * 10);
        v7 *= v7;
        if (word_limit != *(char **)&v9[0x48]) {
          v11 = (long)((v12 - *(int *)&v9[0x3c]) * 10);
          v7 += v11 * v11 >> 1;
        }
      }
      else {
        v7 = 0;
      }
      v7 += *(long *)&v9[0x40];
      if ((v5 == unused_word_type) && (1 <= last_line_length)) {
        v11 = (long)((v12 - last_line_length) * 10);
        v7 += v11 * v11 >> 1;
      }
      if (v7 < v10) {
        *(char **)&v6[-8] = v8;
        *(int *)&v6[-0x14] = v12;
        v10 = v7;
      }
    } while ((word_limit != v8) && (v12 = v12 + *(int *)&v9[0x30] + *(int *)&v9[0xc], v9 = v8, v12 < max_width));
    v7 = 0x1324;
    if (unused_word_type < v5) {
      v1 = v6[-0x40];
      if (v1 & 2) // branch-flip
        v7 = (unsigned long)(-(unsigned int)((v1 & 8) == 0) & 0x58804) + 0x960;
      else {
        v7 = 0xce4;
        if (((!(v1 & 4)) && (v7 = 0x1324, (char *)0x20a149 <= v5)) && (v6[-0x68] & 8)) {
          v4._8_8_ = 0;
          v4._0_8_ = SUB168(SEXT816(40000),8);
          v7 = SUB168((v4._0_16_ << 0x40 | ZEXT816(40000)) / SEXT816((long)(*(int *)&v6[-0x48] + 2)),0) + 0x1324;
        }
      }
    }
    if (v6[-0x18] & 1) // branch-flip
      v7 -= 0x640;
    else if (v6[-0x18] & 8) {
      v3._8_8_ = 0;
      v3._0_8_ = SUB168(SEXT816(0x57e4),8);
      v7 += SUB168((v3._0_16_ << 0x40 | ZEXT816(0x57e4)) / SEXT816((long)(*(int *)&v6[-0x20] + 2)),0);
    }
    *(long *)&v6[-0x10] = v10 + v7;
    v6 = v5;
  }
  *(unsigned int *)&word_limit[8] = v2;
}

// Function: get_space @ 0x20f0
int get_space(_IO_FILE *f,int c)
{
  int v1;
  unsigned char *v2;
  unsigned long v3; // rax
  
  v3 = (unsigned long)(unsigned int)c;
  do {
    while (v1 = (int)v3, v1 != 0x20) {
      if (v1 != 9)
        return v1;
      tabs = 1;
      v1 = in_column + 7;
      if (0 <= in_column)
        v1 = in_column;
      in_column = (v1 >> 3) * 8 + 8;
      v2 = (unsigned char *)f->_IO_read_ptr;
      if (v2 < f->_IO_read_end) goto label_2134;
label_2155:
      v3 = __uflow(f);
    }
    in_column += 1;
    v2 = (unsigned char *)f->_IO_read_ptr;
    if (f->_IO_read_end <= v2) goto label_2155;
label_2134:
    f->_IO_read_ptr = (char *)&v2[1];
    v3 = (unsigned long)*v2;
  } while( true );
}

// Function: get_prefix @ 0x2170
int get_prefix(_IO_FILE *f)
{
  unsigned int c;
  unsigned char v1;
  unsigned char *v2;
  unsigned int v3;
  unsigned char *v4;
  
  v4 = (unsigned char *)f->_IO_read_ptr;
  in_column = 0;
  if (f->_IO_read_end <= v4) // branch-flip
    v3 = __uflow(f);
  else {
    f->_IO_read_ptr = (char *)&v4[1];
    v3 = (unsigned int)*v4;
  }
  v3 = get_space(f,v3);
  if (prefix_length) { // branch-flip
    next_prefix_indent = in_column;
    c = v3;
    if (!*prefix)
      return get_space(f,c); // return-dupe, tail-call
    v4 = prefix;
    if (v3 == *prefix) {
      do {
        in_column += 1;
        v2 = (unsigned char *)f->_IO_read_ptr;
        if (f->_IO_read_end <= v2) { // branch-flip
          c = __uflow(f);
          v1 = v4[1];
          v3 = c;
        }
        else {
          f->_IO_read_ptr = (char *)&v2[1];
          v1 = v4[1];
          c = (unsigned int)*v2;
          v3 = (unsigned int)*v2;
        }
        if (!v1) {
          return get_space(f,c);
        }
        v4 = &v4[1];
      } while (v1 == v3);
    }
  }
  else {
    next_prefix_indent = prefix_lead_space;
    if (in_column <= prefix_lead_space)
      next_prefix_indent = in_column;
  }
  return v3;
}

// Function: put_space @ 0x2260
void put_space(int space)
{
  char *v1;
  int v2;
  int v3;
  int v4;
  
  space = out_column + space;
  v2 = out_column;
  if (tabs) {
    v4 = space + 7;
    if (0 <= space)
      v4 = space;
    v3 = (v4 >> 3) * 8;
    if ((v3 <= out_column + 1) || (v3 <= out_column)) goto label_22df;
    do {
      v1 = *(char **)((long)stdout + 0x28);
      if (*(char **)((long)stdout + 0x30) <= v1) // branch-flip
        __overflow(stdout,9);
      else {
        *(char **)((long)stdout + 0x28) = &v1[1];
        *v1 = 9;
      }
      v2 = out_column + 7;
      if (0 <= out_column)
        v2 = out_column;
      v3 = (v2 >> 3) + 1;
      out_column = v3 * 8;
      v2 = out_column;
    } while (v3 < v4 >> 3);
  }
label_22df:
  while (v2 < space) {
    v1 = *(char **)((long)stdout + 0x28);
    if (*(char **)((long)stdout + 0x30) <= v1) // branch-flip
      __overflow(stdout,0x20);
    else {
      *(char **)((long)stdout + 0x28) = &v1[1];
      *v1 = 0x20;
    }
    out_column += 1;
    v2 = out_column;
  }
}

// Function: set_other_indent @ 0x2340
void set_other_indent(char a0) // return-dupe x2
{
  if (split) {
    other_indent = first_indent;
    return;
  }
  if (crown) {
    if (a0) {
      other_indent = in_column;
      return;
    }
    other_indent = first_indent;
    return;
  }
  if (!tagged) {
    other_indent = first_indent;
    return;
  }
  if ((a0) && (in_column != first_indent)) {
    other_indent = in_column;
    return;
  }
  if (first_indent != other_indent)
    return;
  other_indent = -(unsigned int)(first_indent == 0) & 3;
}

// Function: put_word.isra.1 @ 0x23d0
void put_word.isra.1(unsigned char *a0,int *a1)
{
  unsigned char v1;
  int v2;
  unsigned char *v3;
  unsigned char *v4; // rbx
  
  v2 = *a1;
  if (v2) {
    v3 = a0;
    do {
      v4 = &v3[1];
      v1 = *v3;
      v3 = *(unsigned char **)((long)stdout + 0x28);
      if (*(unsigned char **)((long)stdout + 0x30) <= v3) // branch-flip
        __overflow(stdout,(unsigned int)v1);
      else {
        *(unsigned char **)((long)stdout + 0x28) = &v3[1];
        *v3 = v1;
      }
      v3 = v4;
    } while (v4 != &a0[(unsigned long)(unsigned int)(v2 - 1) + 1]);
    out_column += *a1;
    return;
  }
}

// Function: put_line @ 0x2450
void put_line(Word *w,int indent)
{
  Word *v1;
  char *v2;
  Word *v3;
  Word *v4; // rbx
  Word *v5;
  
  out_column = 0;
  put_space(prefix_indent);
  fputs_unlocked(prefix,stdout);
  out_column = prefix_length + out_column;
  put_space(indent - out_column);
  v1 = w->next_break;
  v5 = (Word *)((long)v1 + -0x28);
  if (w != v5) {
    v3 = w;
    do {
      v4 = &v3[1];
      put_word.isra.1(v3->text,&v3->length);
      put_space(v3->space);
      v3 = v4;
    } while (v5 != v4);
    v5 = (Word *)((long)&w[1].text + ((long)v1 + (0xffffffffffffffb0U - (long)w) & 0xfffffffffffffff8U));
  }
  put_word.isra.1(v5->text,&v5->length);
  last_line_length = out_column;
  v2 = *(char **)((long)stdout + 0x28);
  if (v2 < *(char **)((long)stdout + 0x30)) {
    *(char **)((long)stdout + 0x28) = &v2[1];
    *v2 = 10;
    return;
  }
  __overflow(stdout,10); // tail-call
}

// Function: put_paragraph @ 0x2540
void put_paragraph(Word *finish) // return-dupe
{
  Word **v1;
  Word *w;
  
  put_line((Word *)unused_word_type,first_indent);
  w = unused_word_type._32_8_;
  if (unused_word_type._32_8_ == finish)
    return;
  do {
    put_line(w,other_indent);
    v1 = &w->next_break;
    w = (Word *)*v1;
  } while (finish != (Word *)*v1);
}

// Function: flush_paragraph @ 0x2590
void flush_paragraph(void)
{
  Word *v1;
  long v2;
  Word *v3;
  Word *v4;
  long v5;
  char *v6;
  Word *v7;
  
  v3 = word_limit;
  if (word_limit == (Word *)unused_word_type) {
    fwrite_unlocked(parabuf,1,(long)wptr - 0x213d80,stdout);
    wptr = parabuf;
    return;
  }
  fmt_paragraph();
  v7 = v3;
  if (v3 != unused_word_type._32_8_) {
    v2 = 0x7fffffffffffffff;
    v4 = unused_word_type._32_8_;
    v5 = (unused_word_type._32_8_)->best_cost;
    do {
      v1 = (Word *)v4->next_break;
      v5 -= v1->best_cost;
      if (v5 < v2) {
        v2 = v5;
        v7 = v4;
      }
      if (v2 <= 0x7ffffffffffffff6)
        v2 += 9;
      v4 = v1;
      v5 = v1->best_cost;
    } while (v3 != v1);
  }
  put_paragraph(v7);
  v2 = (long)wptr;
  __memmove_chk(parabuf,v7->text,(long)wptr - (long)v7->text,5000);
  v6 = v7->text;
  v5 = -(long)((int)v6 + -0x213d80);
  wptr = (long)(v2 + v5);
  if (v7 <= word_limit) {
    v3 = v7;
    while( true ) {
      v4 = &v3[1];
      v3->text = &v6[v5];
      if (word_limit < v4) break;
      v6 = v4->text;
      v3 = v4;
    }
  }
  v2 = (long)word_limit - (long)&v7[-0xd0d4];
  __memmove_chk(unused_word_type,v7,((long)word_limit - (long)v7 >> 3) * 8 + 0x28,40000);
  word_limit = (Word *)v2;
}

// Function: get_line @ 0x2700
int get_line(_IO_FILE *f,int c)
{
  unsigned char v1;
  long v10;
  unsigned char *v11;
  unsigned char *v2;
  int v3; // eax
  long *v4; // rax
  unsigned int *v5;
  bool v6; // dl
  unsigned long *v7;
  unsigned char v8;
  int v9; // edx
  
  v4 = __ctype_b_loc();
  v7 = word_limit;
label_2728:
  do {
    *v7 = wptr;
    v5 = wptr;
    do {
      if (v5 == &out_column) {
        set_other_indent(1);
        flush_paragraph();
        v5 = wptr;
      }
      wptr = (unsigned int *)((long)v5 + 1);
      *(char *)v5 = (char)c;
      v11 = (unsigned char *)f->_IO_read_ptr;
      if (f->_IO_read_end <= v11) { // branch-flip
        c = __uflow(f);
        v5 = wptr;
        if (c == 0xffffffff) {
          v10 = *v4;
          break;
        }
      }
      else {
        f->_IO_read_ptr = (char *)&v11[1];
        c = (unsigned int)*v11;
        v5 = wptr;
      }
      v10 = *v4;
    } while (!(*(unsigned char *)(v10 + 1 + SEXT48(c) * 2) & 0x20));
    v7 = word_limit;
    v2 = (unsigned char *)*word_limit;
    v3 = (int)v5 - (int)v2;
    *(int *)&word_limit[1] = v3;
    v9 = in_column + v3;
    v8 = *v2;
    v11 = &v2[(long)v3 + -1];
    v1 = *v11;
    in_column = v9;
    *(unsigned char *)&v7[2] = *(unsigned char *)&v7[2] & 0xfa | strchr("([\'`\"",(int)(char)v8) != NULL | *(unsigned char *)(v10 + (unsigned long)v1 * 2) & 4;
    if (v11 <= v2) // branch-flip
      v3 = (int)(char)*v11;
    else {
      while( true ) {
        v3 = (int)(char)*v11;
        if (!strchr(")]\'\"",v3)) break;
        if (v2 == &v11[-1]) {
          v3 = (int)(char)v11[-1];
          break;
        }
        v11 = &v11[-1];
      }
    }
    *(unsigned char *)&v7[2] = *(unsigned char *)&v7[2] & 0xfd | (strchr(".?!",v3) != NULL) * '\x02';
    c = get_space(f,c);
    v9 = in_column - v9;
    *(int *)((long)word_limit + 0xc) = v9;
    if (c != 0xffffffff) { // branch-flip
      if (*(unsigned char *)&word_limit[2] & 2) { // branch-flip
        v6 = 1 < v9 || c == 10U;
        if (1 < v9 || c == 10U) goto label_28d8;
        v8 = *(unsigned char *)&word_limit[2] & 0xf7;
        *(unsigned char *)&word_limit[2] = v8;
      }
      else {
        v6 = 0;
label_28d8:
        v8 = v6 * '\b';
        *(unsigned char *)&word_limit[2] = *(unsigned char *)&word_limit[2] & 0xf7 | v8;
        if (c == 10) goto label_2924;
      }
      if (uniform) goto label_2924;
      if (word_limit != (unsigned long *)0x213d10) {
        v7 = &word_limit[5];
        word_limit = v7;
        goto label_2728;
      }
label_2990:
      set_other_indent(1);
      flush_paragraph();
    }
    else {
      *(unsigned char *)&word_limit[2] = *(unsigned char *)&word_limit[2] | 8;
label_2924:
      *(unsigned int *)((long)word_limit + 0xc) = ((*(unsigned char *)&word_limit[2] & 8) != 0) + 1;
      if (word_limit == (unsigned long *)0x213d10) goto label_2990;
    }
    v7 = &word_limit[5];
    word_limit = v7;
    if ((c == 10) || (c == 0xffffffff))
      return get_prefix(f); // tail-call
  } while( true );
}

// Function: fmt @ 0x29c0
bool fmt(_IO_FILE *f,char *file)
{
  Word *finish;
  unsigned char v1;
  unsigned long v10; // rsi
  unsigned char *v11;
  unsigned char *v2;
  char *v3;
  int v4;
  unsigned int v5;
  unsigned long v6; // rax
  char *v7; // rax
  int v8;
  bool v9;
  
  fadvise(f,2);
  tabs = 0;
  other_indent = 0;
  next_char = get_prefix(f);
  v5 = next_char;
  do {
    last_line_length = 0;
    while ((v5 == 10 || (v5 == 0xffffffff))) {
      out_column = 0;
      v9 = v5 != 0xffffffff && v5 != 10;
      if (next_prefix_indent < in_column) goto label_2a6e;
      if (v5 != 0xffffffff && v5 != 10) goto label_2bb8;
label_2c60:
      if (v5 == 0xffffffff) {
label_2b0a:
        next_char = 0xffffffff;
        if (f->_flags & 0x20) { // branch-flip
          if (f != stdin) // branch-flip
            rpl_fclose(f);
          else {
            clearerr_unlocked(f);
          }
          v6 = quotearg_n_style_colon(0,3,file);
        }
        else {
          if (f == stdin) {
            clearerr_unlocked(f);
            return 1;
          }
          if (!rpl_fclose(f))
            return 1;
          v8 = *__errno_location();
          v1 = (unsigned char)((unsigned int)v8 >> 0x18);
          v9 = (bool)(v1 >> 7);
          if (v8 < 0)
            return (bool)(v1 >> 7);
          v6 = quotearg_n_style_colon(0,3,file);
          v7 = "%s";
          if (v8) {
            error(0,v8,v7,v6); // return-dupe
            return v9;
          }
        }
        v9 = 0;
        v8 = 0;
        v7 = dcgettext(NULL,"read error",5);
        error(0,v8,v7,v6);
        return v9;
      }
label_2c69:
      v3 = *(char **)((long)stdout + 0x28);
      if (*(char **)((long)stdout + 0x30) <= v3) // branch-flip
        __overflow(stdout,10);
      else {
        *(char **)((long)stdout + 0x28) = &v3[1];
        *v3 = 10;
      }
      v5 = get_prefix(f);
    }
    if ((next_prefix_indent < prefix_lead_space) || (in_column < prefix_full_length + next_prefix_indent)) {
      if (in_column <= next_prefix_indent) { // branch-flip
label_2bb8:
        out_column = 0;
        put_space(next_prefix_indent);
        if (in_column == out_column) {
          v8 = 0;
          goto label_2bf4;
        }
        v1 = *prefix;
        v9 = 1;
        v11 = prefix;
        v4 = out_column;
        v8 = in_column;
        if (v1) goto label_2ab7;
      }
      else {
        v9 = 1;
label_2a6e:
        out_column = 0;
        put_space(next_prefix_indent);
        v11 = prefix;
        v4 = out_column;
        v8 = in_column;
        if (in_column != out_column) {
          do {
            v1 = *v11;
            if (!v1) break;
label_2ab7:
            v11 = &v11[1];
            v2 = *(unsigned char **)((long)stdout + 0x28);
            if (*(unsigned char **)((long)stdout + 0x30) <= v2) { // branch-flip
              __overflow(stdout,(unsigned int)v1);
              v8 = in_column;
            }
            else {
              *(unsigned char **)((long)stdout + 0x28) = &v2[1];
              *v2 = v1;
            }
            v4 = out_column + 1;
            out_column = v4;
          } while (v4 != v8);
        }
        if (!v9) {
          if (v5 == 0xffffffff) {
            if (prefix_length + next_prefix_indent <= in_column) {
              v3 = *(char **)((long)stdout + 0x28);
              if (*(char **)((long)stdout + 0x30) <= v3) // branch-flip
                __overflow(stdout,10);
              else {
                *(char **)((long)stdout + 0x28) = &v3[1];
                *v3 = 10;
              }
            }
            goto label_2b0a;
          }
          goto label_2c69;
        }
      }
      v8 -= v4;
label_2bf4:
      put_space(v8);
label_2c1f:
      if (v5 != 10) {
        do {
          if (v5 == 0xffffffff) break;
          v3 = *(char **)((long)stdout + 0x28);
          if (*(char **)((long)stdout + 0x30) <= v3) { // branch-flip
            __overflow(stdout,v5 & 0xff);
            v11 = (unsigned char *)f->_IO_read_ptr;
            if (v11 < f->_IO_read_end) goto label_2c14;
          }
          else {
            *(char **)((long)stdout + 0x28) = &v3[1];
            *v3 = (char)v5;
            v11 = (unsigned char *)f->_IO_read_ptr;
            if (v11 < f->_IO_read_end) goto label_2c14;
          }
          v5 = __uflow(f);
          if (v5 == 10) break;
        } while( true );
      }
      goto label_2c60;
    }
    prefix_indent = next_prefix_indent;
    first_indent = in_column;
    wptr = parabuf;
    word_limit = (Word *)unused_word_type;
    v5 = get_line(f,v5);
    v9 = 0;
    v10 = (unsigned long)v5;
    if ((next_prefix_indent == prefix_indent) && (prefix_full_length + next_prefix_indent <= in_column))
      v9 = v5 != 0xffffffff && v5 != 10;
    v8 = next_prefix_indent;
    v4 = prefix_indent;
    set_other_indent(v9);
    v5 = (unsigned int)v10;
    if (!split) {
      if (crown) { // branch-flip
        if ((((v8 == v4) && (v8 + prefix_full_length <= in_column)) && (v5 != 10)) && (v5 != 0xffffffff)) {
          while( true ) {
            v5 = get_line(f,(int)v10);
            v10 = (unsigned long)v5;
            if ((next_prefix_indent != prefix_indent) || (in_column < next_prefix_indent + prefix_full_length)) break;
            if ((v5 == 10) || ((v5 == 0xffffffff || (in_column != other_indent)))) break;
          }
        }
      }
      else if (tagged) { // branch-flip
        if ((((v8 == v4) && (v8 + prefix_full_length <= in_column)) && (v5 != 10)) && ((v5 != 0xffffffff && (in_column != first_indent)))) {
          while( true ) {
            v5 = get_line(f,(int)v10);
            v10 = (unsigned long)v5;
            if ((next_prefix_indent != prefix_indent) || ((in_column < next_prefix_indent + prefix_full_length || (v5 == 10)))) break;
            if ((v5 == 0xffffffff) || (in_column != other_indent)) break;
          }
        }
      }
      else if ((((v8 == v4) && (v8 + prefix_full_length <= in_column)) && (v5 != 10)) && ((v5 != 0xffffffff && (in_column == other_indent)))) {
        do {
          v5 = get_line(f,(int)v10);
          v10 = (unsigned long)v5;
          if (((next_prefix_indent != prefix_indent) || ((in_column < next_prefix_indent + prefix_full_length || (v5 == 10)))) || (v5 == 0xffffffff)) break;
        } while (in_column == other_indent);
      }
    }
    finish = word_limit;
    if (word_limit <= (Word *)0x20a120)
      __assert_fail("word < word_limit","src/fmt.c",0x26f,"get_paragraph"); // no-return
    word_limit[-1].period = 1;
    word_limit[-1].final = 1;
    next_char = v5;
    fmt_paragraph();
    put_paragraph(finish);
    v5 = next_char;
  } while( true );
label_2c14:
  f->_IO_read_ptr = (char *)&v11[1];
  v5 = (unsigned int)*v11;
  goto label_2c1f;
}

// Function: usage @ 0x3000
void usage(int status)
{
  unsigned char *v1;
  char *v10; // stack - 0xa8
  unsigned char *v11;
  unsigned char *v12;
  long v13; // fs_offset
  bool v14; // cf
  bool v15;
  char *v16; // stack - 0xa0
  char *v17; // stack - 0x70
  char *v18; // stack - 0x68
  char *v19; // stack - 0x60
  unsigned char *v2;
  char *v20; // stack - 0x58
  char *v21; // stack - 0x50
  unsigned long v22; // stack - 0x48
  unsigned long v23; // stack - 0x40
  unsigned long v24; // stack - 0x30
  char *v25 [5]; // stack - 0x98
  void *v3;
  unsigned long v4;
  char **v5;
  unsigned long *v6; // rax
  char *v7; // rax
  char *v8;
  long v9; // rcx
  
  v4 = program_name;
  v24 = *(unsigned long *)(v13 + 0x28);
  if (status) {
    __fprintf_chk(stderr,1,dcgettext(NULL,"Try \'%s --help\' for more information.\n",5),v4);
    exit(status); // return-dupe, no-return
  }
  __printf_chk(1,dcgettext(NULL,"Usage: %s [-WIDTH] [OPTION]... [FILE]...\n",5),v4);
  v3 = stdout;
  fputs_unlocked(dcgettext(NULL,"Reformat each paragraph in the FILE(s), writing to standard output.\nThe option -WIDTH is an abbreviated form of --width=DIGITS.\n",5),v3);
  v3 = stdout;
  fputs_unlocked(dcgettext(NULL,"\nWith no FILE, or when FILE is -, read standard input.\n",5),v3);
  v3 = stdout;
  fputs_unlocked(dcgettext(NULL,"\nMandatory arguments to long options are mandatory for short options too.\n",5),v3);
  v3 = stdout;
  fputs_unlocked(dcgettext(NULL,"  -c, --crown-margin        preserve indentation of first two lines\n  -p, --prefix=STRING       reformat only lines beginning with STRING,\n                              reattaching the prefix to reformatted lines\n  -s, --split-only          split long lines, but do not refill\n",5),v3);
  v3 = stdout;
  fputs_unlocked(dcgettext(NULL,"  -t, --tagged-paragraph    indentation of first line different from second\n  -u, --uniform-spacing     one space between words, two after sentences\n  -w, --width=WIDTH         maximum line width (default of 75 columns)\n  -g, --goal=WIDTH          goal width (default of 93% of width)\n",5),v3);
  v3 = stdout;
  fputs_unlocked(dcgettext(NULL,"      --help     display this help and exit\n",5),v3);
  v3 = stdout;
  fputs_unlocked(dcgettext(NULL,"      --version  output version information and exit\n",5),v3);
  v22 = 0;
  v10 = "[";
  v16 = "test invocation";
  v25[4] = "sha256sum";
  v25[0] = "coreutils";
  v18 = "sha384sum";
  v25[1] = "Multi-call invocation";
  v20 = "sha512sum";
  v23 = 0;
  v25[2] = "sha224sum";
  v25[3] = "sha2 utilities";
  v17 = "sha2 utilities";
  v19 = "sha2 utilities";
  v21 = "sha2 utilities";
  v5 = &v10;
  do {
    v6 = v5;
    v5 = (unsigned long *)&v6[2];
    v12 = (unsigned char *)*v5;
    v14 = 0;
    if (!v12) break;
    v9 = 4;
    v15 = 0;
    v11 = (unsigned char *)"fmt";
    do {
      if (!v9) break;
      v9 -= 1;
      v2 = &v12[1];
      v1 = &v11[1];
      v14 = *v11 < *v12;
      v15 = *v11 == *v12;
      v11 = v1;
      v12 = v2;
    } while (v15);
  } while ((!v14 && !v15) != v14);
  v8 = (char *)v6[3];
  if (v8) { // branch-flip
    __printf_chk(1,dcgettext(NULL,"\n%s online help: <%s>\n",5),"GNU coreutils","https://www.gnu.org/software/coreutils/");
    v7 = setlocale(5,NULL);
    if (v7) {
      if (strncmp(v7,"en_",3)) {
label_33b1:
        v3 = stdout;
        fputs_unlocked(dcgettext(NULL,"Report any translation bugs to <https://translationproject.org/team/>\n",5),v3);
      }
    }
    __printf_chk(1,dcgettext(NULL,"Full documentation <%s%s>\n",5),"https://www.gnu.org/software/coreutils/","fmt");
    v7 = " invocation";
    if (v8 != "fmt")
      v7 = "";
  }
  else {
    __printf_chk(1,dcgettext(NULL,"\n%s online help: <%s>\n",5),"GNU coreutils","https://www.gnu.org/software/coreutils/");
    v8 = setlocale(5,NULL);
    if (v8) {
      if (strncmp(v8,"en_",3)) {
        v8 = "fmt";
        goto label_33b1;
      }
    }
    v8 = "fmt";
    __printf_chk(1,dcgettext(NULL,"Full documentation <%s%s>\n",5),"https://www.gnu.org/software/coreutils/","fmt");
    v7 = " invocation";
  }
  __printf_chk(1,dcgettext(NULL,"or available locally via: info \'(coreutils) %s%s\'\n",5),v8,v7);
  exit(status);
}

// Function: close_stdout_set_file_name @ 0x33e0
void close_stdout_set_file_name(char *file)
{
  file_name = file;
}

// Function: close_stdout_set_ignore_EPIPE @ 0x33f0
void close_stdout_set_ignore_EPIPE(bool ignore)
{
  ignore_EPIPE = ignore;
}

// Function: close_stdout @ 0x3400
void close_stdout(void)
{
  int v1; // eax
  int *v2; // rax
  char *v3; // rax
  char *v4; // rax
  
  v1 = close_stream(stdout);
  if (v1) {
    v2 = __errno_location();
    if ((!ignore_EPIPE) || (*v2 != 0x20)) {
      v3 = dcgettext(NULL,"write error",5);
      if (file_name) { // branch-flip
        v4 = quotearg_colon(file_name);
        error(0,*v2,"%s: %s",v4,v3);
      }
      else {
        error(0,*v2,"%s",v3);
      }
      _exit(exit_failure); // return-dupe, no-return
    }
  }
  v1 = close_stream(stderr);
  if (!v1)
    return;
  _exit(exit_failure);
}

// Function: fdadvise @ 0x34b0
void fdadvise(void)
{
  posix_fadvise(); // tail-call
}

// Function: fadvise @ 0x34c0
void fadvise(_IO_FILE *fp,int advice)
{
  if (fp) {
    posix_fadvise(fileno(fp),0,0,advice); // tail-call
    return;
  }
}

// Function: rpl_fclose @ 0x34f0
int rpl_fclose(_IO_FILE *fp)
{
  int v1; // eax
  int v2;
  int *v3; // rax
  
  if (0 <= fileno(fp)) {
    if (__freading(fp)) {
      v2 = fileno(fp);
      if (lseek(v2,0,1) == -1) {
        return fclose(fp); // tail-call, return-dupe
      }
    }
    if (rpl_fflush(fp)) {
      v3 = __errno_location();
      v2 = *v3;
      v1 = fclose(fp);
      if (v2) {
        *v3 = v2;
        v1 = -1;
      }
      return v1;
    }
  }
  return fclose(fp);
}

// Function: rpl_fflush @ 0x3570
int rpl_fflush(_IO_FILE *stream)
{
  if (stream) {
    if ((__freading(stream)) && (stream->_flags & 0x100U)) {
      rpl_fseeko(stream,0,1);
      return fflush(stream); // tail-call
    }
  }
  return fflush(stream); // tail-call
}

// Function: rpl_fseeko @ 0x35b0
int rpl_fseeko(_IO_FILE *fp,long offset,int whence)
{
  long v1; // rax
  
  if (((fp->_IO_read_end == fp->_IO_read_ptr) && (fp->_IO_write_ptr == fp->_IO_write_base)) && (!fp->_IO_save_base)) {
    v1 = lseek(fileno(fp),offset,whence);
    if (v1 != -1) {
      fp->_flags = fp->_flags & 0xffffffef;
      fp->_offset = v1;
      v1 = 0;
    }
    return (int)v1;
  }
  return fseeko(); // tail-call
}

// Function: set_program_name @ 0x3610
void set_program_name(char *argv0) // return-dupe x4
{
  unsigned char *v1;
  bool v10; // zf
  unsigned char *v2;
  char *v3; // rax
  long v4; // rcx
  unsigned long v5; // rdx
  unsigned char *v6;
  unsigned char *v7;
  bool v8;
  bool v9; // cf
  
  if (!argv0) {
    fwrite("A NULL argv[0] was passed through an exec system call.\n",1,0x37,stderr);
    abort(); // no-return
  }
  v3 = strrchr(argv0,0x2f);
  if (!v3) {
    program_name = (unsigned char *)argv0;
    program_invocation_name = (unsigned char *)argv0;
    return;
  }
  program_invocation_name = (unsigned char *)&v3[1];
  v5 = (long)program_invocation_name - (long)argv0;
  v8 = v5 < 6;
  v10 = v5 == 6;
  if ((long)v5 <= 6) {
    program_invocation_name = (unsigned char *)argv0;
    program_name = (unsigned char *)argv0;
    return;
  }
  v4 = 7;
  v6 = (unsigned char *)&v3[-6];
  v7 = (unsigned char *)"/.libs/";
  do {
    if (!v4) break;
    v4 -= 1;
    v2 = &v7[1];
    v1 = &v6[1];
    v8 = *v6 < *v7;
    v10 = *v6 == *v7;
    v6 = v1;
    v7 = v2;
  } while (v10);
  v9 = 0;
  if ((!v8 && !v10) != v8) {
    program_invocation_name = (unsigned char *)argv0;
    program_name = (unsigned char *)argv0;
    return;
  }
  v4 = 3;
  v8 = 1;
  v6 = program_invocation_name;
  v7 = (unsigned char *)"lt-";
  do {
    if (!v4) break;
    v4 -= 1;
    v2 = &v7[1];
    v1 = &v6[1];
    v9 = *v6 < *v7;
    v8 = *v6 == *v7;
    v6 = v1;
    v7 = v2;
  } while (v8);
  if ((!v9 && !v8) != v9) {
    program_name = program_invocation_name;
    return;
  }
  __progname = &v3[4];
  program_invocation_name = (unsigned char *)__progname;
  program_name = (unsigned char *)__progname;
}

// Function: gettext_quote.part.1 @ 0x36b0
char * gettext_quote.part.1(char *a0,int a1) // return-dupe x3
{
  unsigned char *v1; // rax
  
  v1 = (unsigned char *)locale_charset();
  if ((*v1 & 0xdf) != 0x55) { // branch-flip
    if (((((*v1 & 0xdf) == 0x47) && ((v1[1] & 0xdf) == 0x42)) && (v1[2] == 0x31)) && (((v1[3] == 0x38 && (v1[4] == 0x30)) && ((v1[5] == 0x33 && ((v1[6] == 0x30 && (!v1[7])))))))) {
      if (*a0 == '`')
        return (char *)0x6f58;
      return (char *)0x6f5c;
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

// Function: quotearg_buffer_restyled @ 0x3790
unsigned long quotearg_buffer_restyled(char *a0,unsigned long a1,char *a2,unsigned long a3,int a4,unsigned int a5,long a6,char *a7,char *a8)
{
  unsigned char *v1;
  int v10; // eax
  unsigned long v11; // rax
  char *v12;
  char v13;
  unsigned char v14;
  unsigned int v15; // ebx
  unsigned long v16;
  __mbstate_t v17; // stack - 0x48
  unsigned int v18; // stack - 0x4c
  unsigned long v19;
  unsigned long v2;
  bool v20;
  unsigned long v21;
  unsigned long v22;
  unsigned long v23;
  bool v24;
  bool v25;
  bool v26;
  bool v27;
  bool v28;
  unsigned long v29; // stack - 0xe0
  bool v3;
  char *v30; // stack - 0xb0
  unsigned long v31; // stack - 0xa8
  unsigned long v32; // stack - 0xa0
  char *v33; // stack - 0x98
  char *v34; // stack - 0x90
  bool v4;
  bool v5;
  bool v6;
  bool v7;
  unsigned char v8;
  long v9;
  
  v9 = a6;
  v34 = a7;
  v33 = a8;
  v32 = __ctype_get_mb_cur_max();
  v15 = a5 & 2;
  v24 = v15 != 0;
  switch(a4) {
    case 0:
      v3 = 1;
      v6 = 0;
      v22 = 0;
      v4 = 0;
      v31 = 0;
      v24 = 0;
      v28 = 0;
      v29 = 0;
      v30 = NULL;
      break;
    case 1:
label_382d:
      v28 = 0;
label_3850:
      v4 = 0;
      v31 = 0;
      v24 = 1;
      v3 = 1;
      v6 = 0;
      v22 = 0;
      a4 = 2;
      v29 = 1;
      v30 = "\'";
      break;
    case 2:
      if (v15) {
        v3 = 1;
        v6 = 0;
        v22 = 0;
        v4 = 0;
        v31 = 0;
        v24 = 1;
        v28 = 0;
        v29 = 1;
        v30 = "\'";
        break;
      }
      v28 = 0;
label_442b:
      if (a1) {
        v6 = 0;
        v4 = 0;
        v19 = 0;
        v3 = 1;
        v31 = a1;
        goto label_3fc9;
      }
      v3 = 1;
      v6 = 0;
      v22 = 1;
      v31 = 0;
      a4 = 2;
      v4 = 0;
      v24 = 0;
      v29 = 1;
      v30 = "\'";
      break;
    case 3:
      v28 = 1;
      goto label_3850;
    case 4:
      if (!v15) {
        v28 = 1;
        goto label_442b;
      }
      goto label_382d;
    case 5:
      if (v15) { // branch-flip
        v3 = 1;
        v6 = 0;
        v22 = 0;
        v4 = 0;
        v31 = 0;
        v24 = 1;
        v28 = 1;
        v29 = 1;
        v30 = "\"";
      }
      else {
        if (a1)
          *a0 = 0x22;
        v4 = 0;
        v31 = 0;
        v3 = 1;
        v6 = 0;
        v24 = 0;
        v22 = 1;
        v28 = 1;
        v29 = 1;
        v30 = "\"";
      }
      break;
    case 6:
      v3 = 1;
      v6 = 0;
      v22 = 0;
      v4 = 0;
      a4 = 5;
      v31 = 0;
      v24 = 1;
      v28 = 1;
      v29 = 1;
      v30 = "\"";
      break;
    case 7:
      v3 = 1;
      v6 = 0;
      v22 = 0;
      v4 = 0;
      v31 = 0;
      v24 = 0;
      v28 = 1;
      v29 = 0;
      v30 = NULL;
      break;
    case 8:
    case 9:
    case 10:
      if (a4 != 10) {
        v34 = dcgettext(NULL,"`",5);
        if (v34 == "`")
          v34 = (char *)gettext_quote.part.1("`",a4);
        v33 = dcgettext(NULL,"\'",5);
        if (v33 == "\'")
          v33 = (char *)gettext_quote.part.1("\'",a4);
      }
      v22 = 0;
      if (!v15) {
        v13 = *v34;
        while (v13) {
          if (v22 < a1)
            a0[v22] = v13;
          v22 += 1;
          v13 = v34[v22];
        }
      }
      v30 = v33;
      v6 = 0;
      v29 = strlen(v33);
      v3 = 1;
      v4 = 0;
      v31 = 0;
      v28 = 1;
      break;
    default:
      abort(); // no-return
    
  }
  do {
    v19 = 0;
label_3880:
    v25 = a3 != v19;
    if (a3 == 0xffffffffffffffff)
      v25 = a2[v19] != '\0';
    if (v25) {
      v20 = (bool)(a4 != 2 & v28);
      v1 = (unsigned char *)&a2[v19];
      v25 = 1;
      v23 = v22;
      v7 = v6;
      if (!v20) {
        v16 = (unsigned long)*v1;
        switch(*v1) {
          case 0:
            if (v28) goto label_3c60;
            v20 = 0;
            v16 = 0;
            if (a5 & 1) goto label_3e50;
            goto label_3b1b;
          default:
label_3d1d:
            v26 = v28;
            if (v32 != 1) {
              v17.__count = 0;
              v17.__value.__wch = 0;
              if (a3 == 0xffffffffffffffff)
                a3 = strlen(a2);
              v21 = 0;
              do {
                v2 = v19 + v21;
                v11 = rpl_mbrtowc((int *)&v18,&a2[v2],a3 - v2,&v17);
                if (!v11) break;
                if (v11 == 0xffffffffffffffff) {
                  v25 = 0;
                  goto label_4724;
                }
                if (v11 == 0xfffffffffffffffe) {
                  if ((a3 <= v2) || (!a2[v2])) goto label_4888;
                  goto label_4877;
                }
                if (((a4 == 2) && (v24)) && (v11 != 1)) {
                  v12 = &a2[v2 + 1];
                  do {
                    if (((unsigned char)(*v12 + 0xa5U) <= 0x21) && (1L << (*v12 + 0xa5U & 0x3f) & 0x20000002bU)) {
                      a4 = 2;
                      goto label_3b8a;
                    }
                    v12 = &v12[1];
                  } while (&a2[v2 + v11] != v12);
                }
                v10 = iswprint(v18);
                if (!v10)
                  v25 = 0;
                v21 += v11;
              } while (!mbsinit(&v17));
              v26 = (bool)((v25 ^ 1U) & v28);
              goto label_4724;
            }
            v21 = 1;
            v26 = (*(unsigned short *)(*(long *)__ctype_b_loc() + v16 * 2) & 0x4000) == 0;
            v25 = !v26;
            v26 = (bool)(v26 & v28);
            goto label_3d6b;
          case 7:
            v16 = 7;
            v14 = 0x61;
            break;
          case 8:
            v16 = 8;
            v14 = 0x62;
            break;
          case 9:
            v16 = 9;
            v14 = 0x74;
            goto label_3daa;
          case 10:
            v16 = 10;
            v14 = 0x6e;
            goto label_3daa;
          case 0xb:
            v16 = 0xb;
            v14 = 0x76;
            break;
          case 0xc:
            v16 = 0xc;
            v14 = 0x66;
            break;
          case 0xd:
            v16 = 0xd;
            v14 = 0x72;
label_3daa:
            v20 = 0;
            if ((bool)(a4 == 2 & v24)) {
label_3dbf:
              a4 = 2;
              goto label_3b8a;
            }
            break;
          case 0x20:
            v16 = 0x20;
            goto label_39b6;
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
            v25 = 0;
label_39b6:
            v27 = a4 == 2;
            if ((v24) && (v27)) goto label_3dbf;
            goto label_39c8;
          case 0x23:
          case 0x7e:
label_39ad:
            v27 = a4 == 2;
            if (!v19) goto label_39b6;
            goto label_3b4f;
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
            goto label_3d73;
          case 0x27:
label_3ac5:
            v4 = v25;
            if (a4 != 2) {
              v27 = 0;
              v16 = 0x27;
              goto label_39c8;
            }
            if (!v24) {
              if (a1) { // branch-flip
                v23 = 0;
                v16 = a1;
                if (v31) goto label_4274;
              }
              else {
label_4274:
                if (v22 < a1)
                  a0[v22] = 0x27;
                if (v22 + 1 < a1)
                  a0[v22 + 1] = 0x5c;
                v23 = a1;
                v16 = v31;
                if (v22 + 2 < a1)
                  a0[v22 + 2] = 0x27;
              }
              v22 += 3;
              v14 = 0x27;
              a1 = v23;
              v31 = v16;
              v26 = 0;
              v7 = 0;
              goto label_3b30;
            }
            goto label_3b8a;
          case 0x3f:
label_3a95:
            if (a4 == 2) {
              if (!v24) {
                v14 = 0x3f;
                v26 = 0;
                v25 = 0;
                goto label_3b30;
              }
              goto label_3b8a;
            }
            if (a4 != 5) {
              v27 = a4 == 2;
              v16 = 0x3f;
              v25 = 0;
              goto label_39c8;
            }
            if ((((!(a5 & 4)) || (v21 = v19 + 2, a3 <= v21)) || (a2[v19 + 1] != '?')) || ((v16 = (unsigned long)(unsigned char)a2[v21], 0x3f <= (unsigned char)a2[v21] || (!(0x7000a38200000000U >> (v16 & 0x3f) & 1))))) {
              v27 = 0;
              v25 = 0;
              v16 = 0x3f;
              goto label_39c8;
            }
            if (!v24) {
              if (v22 < a1)
                a0[v22] = 0x3f;
              if (v22 + 1 < a1)
                a0[v22 + 1] = 0x22;
              if (v22 + 2 < a1)
                a0[v22 + 2] = 0x22;
              if (v22 + 3 < a1)
                a0[v22 + 3] = 0x3f;
              v23 = v22 + 4;
              v27 = 0;
              v19 = v21;
              v5 = 0;
              goto label_3cf7;
            }
            return quotearg_buffer_restyled(a0,a1,a2,a3,a4,a5 & 0xfffffffd,0,v34,v33); // return-dupe
          case 0x5c:
            if (a4 != 2) goto label_3aed;
            if (!v24) {
label_3e75:
              v19 += 1;
              v25 = 0;
              v14 = 0x5c;
              goto label_3e88;
            }
            goto label_3b8a;
          case 0x7b:
          case 0x7d:
label_3985:
            v26 = a3 != 1;
            if (a3 == 0xffffffffffffffff)
              v26 = a2[1] != '\0';
            v27 = a4 == 2;
            if (!v26) goto label_39ad;
label_3b4f:
            v25 = 0;
            goto label_39c8;
          
        }
label_3b10:
        if (!v28) {
label_3b1b:
          v25 = 0;
          goto label_3b1e;
        }
label_3b65:
        v25 = 0;
        goto label_3b76;
      }
      if (v29) {
        if ((a3 == 0xffffffffffffffff) && (2 <= v29))
          a3 = strlen(a2);
        if ((a3 < v19 + v29) || (memcmp(v1,v30,v29))) goto label_3ef8;
        if (!v24) {
          v16 = (unsigned long)*v1;
          v26 = v20;
          switch(*v1) {
            case 0:
              goto label_3c70;
            default:
              goto label_3d1d;
            case 7:
              goto label_3c50;
            case 8:
              goto label_3c48;
            case 9:
              goto label_3c35;
            case 10:
              goto label_3c20;
            case 0xb:
              goto label_4400;
            case 0xc:
              goto label_3c00;
            case 0xd:
              goto label_3b60;
            case 0x20:
              goto label_3f5a;
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
              goto label_3b4d;
            case 0x23:
            case 0x7e:
              goto label_3f3a;
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
              goto label_3f22;
            case 0x27:
              goto label_3ac5;
            case 0x3f:
              goto label_3a95;
            case 0x5c:
              goto label_3aed;
            case 0x7b:
            case 0x7d:
              goto label_3985;
            
          }
        }
        return quotearg_buffer_restyled(a0,a1,a2,a3,a4,a5 & 0xfffffffd,0,v34,v33);
      }
label_3ef8:
      v16 = (unsigned long)*v1;
      switch(*v1) {
        case 0:
label_3c60:
          if (!v24) {
            v20 = 0;
label_3c70:
            v27 = a4 == 2;
            v26 = (bool)((v6 ^ 1U) & v27);
            if (v26) {
              if (v22 < a1)
                a0[v22] = 0x27;
              if (v22 + 1 < a1)
                a0[v22 + 1] = 0x24;
              if (v22 + 2 < a1)
                a0[v22 + 2] = 0x27;
              v23 = v22 + 3;
              v6 = v26;
              if (a1 <= v22 + 3) {
                v22 += 4;
                v25 = 0;
                v14 = 0x30;
                v7 = v26;
                goto label_3b30;
              }
label_3cba:
              a0[v23] = 0x5c;
              v22 = v23;
            }
            else if (v22 < a1) {
              goto label_3cba;
            }
            v23 = v22 + 1;
            if (a4 != 2) { // branch-flip
              v16 = 0x30;
              v5 = v25;
              if ((v19 + 1 < a3) && ((unsigned char)(a2[v19 + 1] - 0x30U) <= 9)) {
                if (v23 < a1)
                  a0[v23] = 0x30;
                if (v22 + 2 < a1)
                  a0[v22 + 2] = 0x30;
                v23 = v22 + 3;
                v16 = 0x30;
              }
label_3cf7:
              v14 = (unsigned char)v16;
              v25 = 0;
              v22 = v23;
              v26 = v5;
              v7 = v6;
              if (v28 && !v27) goto label_39d8;
            }
            else {
              v14 = 0x30;
              v22 = v23;
              v26 = v25;
              v7 = v6;
              v25 = 0;
            }
            goto label_3b30;
          }
          v28 = a4 == 2;
          goto label_3b8a;
        default:
          v20 = 0;
          goto label_3d1d;
        case 7:
label_3c50:
          v14 = 0x61;
          break;
        case 8:
label_3c48:
          v14 = 0x62;
          break;
        case 9:
          v20 = 0;
label_3c35:
          v16 = 9;
          v14 = 0x74;
          goto label_3b10;
        case 10:
label_3c20:
          v14 = 0x6e;
          goto label_3b65;
        case 0xb:
label_4400:
          v14 = 0x76;
          goto label_3b65;
        case 0xc:
label_3c00:
          v14 = 0x66;
          break;
        case 0xd:
label_3b60:
          v14 = 0x72;
          goto label_3b65;
        case 0x20:
          v26 = 0;
label_3f5a:
          v16 = 0x20;
          goto label_3f22;
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
          v20 = 0;
label_3b4d:
          v27 = 0;
          goto label_3b4f;
        case 0x23:
        case 0x7e:
          v26 = 0;
label_3f3a:
          if (!v19) {
            v27 = 0;
            v25 = v20;
            v20 = v26;
            goto label_39c8;
          }
          v5 = 0;
          v25 = 0;
          v20 = v26;
          goto label_39d8;
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
          v26 = 0;
label_3f22:
          v5 = 0;
          v25 = v20;
          v20 = v26;
          goto label_39d8;
        case 0x27:
          v20 = 0;
          goto label_3ac5;
        case 0x3f:
          v20 = 0;
          goto label_3a95;
        case 0x5c:
          v20 = 0;
label_3aed:
          if (((bool)(v28 & v24)) && (v29)) goto label_3e75;
          v16 = 0x5c;
          v14 = 0x5c;
          goto label_3b10;
        case 0x7b:
        case 0x7d:
          v20 = 0;
          goto label_3985;
        
      }
      if (v24) goto label_3b80;
      v25 = 0;
      goto label_3a4e;
    }
    if ((!v22) && ((bool)(v24 & a4 == 2))) {
      a4 = 2;
label_3b8a:
      if (v28)
        a4 = 4;
      return quotearg_buffer_restyled(a0,a1,a2,a3,a4,a5 & 0xfffffffd,0,v34,v33);
    }
    v25 = (bool)(a4 == 2 & (v24 ^ 1U));
    v24 = (bool)(v24 ^ 1U);
    if ((!v25) || (v24 = v25, !v4)) {
label_491b:
      v19 = v22;
      if (((v30) && (v24)) && (v13 = *v30, v13)) {
        do {
          if (v19 < a1)
            a0[v19] = v13;
          v19 += 1;
          v13 = v30[v19 - v22];
        } while (v13);
      }
      if (v19 < a1)
        a0[v19] = 0;
      return v19;
    }
    if (v3)
      return quotearg_buffer_restyled(a0,v31,a2,a3,5,a5,v9,v34,v33);
    v24 = v4;
    if (a1 || !v31) goto label_491b;
    v19 = v31;
    v4 = !a1 && v31;
label_3fc9:
    *a0 = 0x27;
    a4 = 2;
    v22 = 1;
    v30 = "\'";
    v29 = 1;
    v24 = 0;
    a1 = v31;
    v31 = v19;
  } while( true );
label_3e50:
  v19 += 1;
  goto label_3880;
  while (v1[v21]) {
label_4877:
    v21 += 1;
    if (a3 <= v19 + v21) break;
  }
label_4888:
  v25 = 0;
label_4724:
  if (2 <= v21) {
label_42b5:
    v8 = 0;
    v21 += v19;
    v23 = v19;
    do {
      v14 = (unsigned char)v16;
      if (v26) {
        if (v24) {
          v28 = a4 == 2;
          goto label_3b8a;
        }
        if ((bool)(a4 == 2 & (v7 ^ 1U))) {
          if (v22 < a1)
            a0[v22] = 0x27;
          if (v22 + 1 < a1)
            a0[v22 + 1] = 0x24;
          if (v22 + 2 < a1)
            a0[v22 + 2] = 0x27;
          v22 += 3;
          v7 = 1;
        }
        if (v22 < a1)
          a0[v22] = 0x5c;
        if (v22 + 1 < a1)
          a0[v22 + 1] = (char)(v16 >> 6) + '0';
        if (v22 + 2 < a1)
          a0[v22 + 2] = ((unsigned char)(v16 >> 3) & 7) + 0x30;
        v19 = v23 + 1;
        v22 += 3;
        v14 = (v14 & 7) + 0x30;
        if (v21 <= v19) goto label_46d0;
        v8 = 1;
      }
      else {
        v6 = (bool)((v8 ^ 1) & v7);
        if (v20) {
          if (v22 < a1)
            a0[v22] = 0x5c;
          v22 += 1;
        }
        v19 = v23 + 1;
        if (v21 <= v19) {
          goto label_3e88;
        }
        if (v6) {
          if (v22 < a1)
            a0[v22] = 0x27;
          if (v22 + 1 < a1)
            a0[v22 + 1] = 0x27;
          v22 += 2;
          v20 = 0;
          v7 = 0;
        }
        else {
          v20 = 0;
        }
      }
      v23 += 1;
      if (v22 < a1)
        a0[v22] = v14;
      v16 = (unsigned long)(unsigned char)a2[v23];
      v22 += 1;
    } while( true );
  }
label_3d6b:
  if (v26) {
    v25 = 0;
    v26 = v28;
    goto label_42b5;
  }
label_3d73:
  v27 = a4 == 2;
label_39c8:
  v5 = 0;
  if ((bool)(v27 | v28 ^ 1U)) {
label_3b1e:
    v14 = (unsigned char)v16;
    v5 = 0;
    v26 = 0;
    if (v24) goto label_39d8;
label_3b30:
    v6 = v7;
    if (!v20) {
      v19 += 1;
      v6 = (bool)((v26 ^ 1U) & v7);
label_3e88:
      if (v6) {
        if (v22 < a1)
          a0[v22] = 0x27;
        if (v22 + 1 < a1)
          a0[v22 + 1] = 0x27;
        v22 += 2;
        v7 = 0;
      }
      goto label_3a60;
    }
  }
  else {
label_39d8:
    v14 = (unsigned char)v16;
    v22 = v23;
    v26 = v5;
    v7 = v6;
    if (!v9) goto label_3b30;
    if (!(*(unsigned int *)(v9 + (v16 >> 5) * 4) >> (v14 & 0x1f) & 1)) goto label_3b30;
  }
label_3b76:
  if (v24) {
label_3b80:
    v28 = (bool)(v28 & a4 == 2);
    goto label_3b8a;
  }
  if ((bool)((v6 ^ 1U) & a4 == 2)) {
    if (v22 < a1)
      a0[v22] = 0x27;
    if (v22 + 1 < a1)
      a0[v22 + 1] = 0x24;
    if (v22 + 2 < a1)
      a0[v22 + 2] = 0x27;
    v22 += 3;
    v6 = 1;
  }
label_3a4e:
  if (v22 < a1)
    a0[v22] = 0x5c;
  v22 += 1;
  v19 += 1;
  v7 = v6;
label_3a60:
  v6 = v7;
  if (v22 < a1)
    a0[v22] = v14;
  v22 += 1;
  if (!v25)
    v3 = 0;
  goto label_3880;
label_46d0:
  goto label_3a60;
}

// Function: quotearg_n_options @ 0x4a80
char * quotearg_n_options(int n,char *arg,unsigned long argsize,quoting_options *options)
{
  long n_incr_min; // rdx
  char *p;
  int v1;
  quoting_style v2;
  unsigned long v3;
  unsigned int v4; // eax
  int *v5; // rax
  unsigned long *v6;
  unsigned long v7;
  unsigned long *v8; // rbx
  long v9; // stack - 0x48
  
  v5 = __errno_location();
  v1 = *v5;
  if ((0 <= n) && (n != 0x7fffffff)) {
    v6 = slotvec;
    if (nslots <= n) {
      v9 = (long)nslots;
      n_incr_min = (long)((n - nslots) + 1);
      if (slotvec != (unsigned long *)slotvec0) // branch-flip
        v6 = xpalloc(slotvec,&v9,n_incr_min,0x7fffffff,0x10);
      else {
        v6 = xpalloc(NULL,&v9,n_incr_min,0x7fffffff,0x10);
        *v6 = slotvec0._0_8_;
        v6[1] = slotvec0._8_8_;
      }
      slotvec = v6;
      memset(&v6[(long)nslots * 2],0,(v9 - nslots) * 0x10);
      nslots = (int)v9;
    }
    v8 = &v6[(long)n * 2];
    v4 = options->flags | 1;
    v3 = *v8;
    p = (char *)v8[1];
    v7 = quotearg_buffer_restyled(p,v3,arg,argsize,options->style,v4,options->quote_these_too,options->left_quote,options->right_quote);
    if (v3 <= v7) {
      v7 += 1;
      *v8 = v7;
      if (p != slot0)
        rpl_free(p);
      p = xcharalloc(v7);
      v2 = options->style;
      v8[1] = (unsigned long)p;
      quotearg_buffer_restyled(p,v7,arg,argsize,v2,v4,options->quote_these_too,options->left_quote,options->right_quote);
    }
    *v5 = v1;
    return p;
  }
  abort(); // no-return
}

// Function: clone_quoting_options @ 0x4c80
quoting_options * clone_quoting_options(quoting_options *o)
{
  int v1;
  int *v2; // rax
  quoting_options *v3; // rax
  
  v2 = __errno_location();
  v1 = *v2;
  if (!o)
    o = (quoting_options *)default_quoting_options;
  v3 = xmemdup(o,0x38);
  *v2 = v1;
  return v3;
}

// Function: get_quoting_style @ 0x4cc0
quoting_style get_quoting_style(quoting_options *o)
{
  if (!o)
    o = (quoting_options *)default_quoting_options;
  return o->style;
}

// Function: set_quoting_style @ 0x4ce0
void set_quoting_style(quoting_options *o,quoting_style s)
{
  if (!o)
    o = (quoting_options *)default_quoting_options;
  o->style = s;
}

// Function: set_char_quoting @ 0x4d00
unsigned int set_char_quoting(char *a0,unsigned char a1,unsigned int a2)
{
  unsigned int v1;
  unsigned int v2; // eax
  
  if (!a0)
    a0 = default_quoting_options;
  v1 = *(unsigned int *)&a0[(unsigned long)(a1 >> 5) * 4 + 8];
  v2 = v1 >> (a1 & 0x1f);
  *(unsigned int *)&a0[(unsigned long)(a1 >> 5) * 4 + 8] = ((a2 ^ v2) & 1) << (a1 & 0x1f) ^ v1;
  return v2 & 1;
}

// Function: set_quoting_flags @ 0x4d40
int set_quoting_flags(quoting_options *o,int i)
{
  int v1;
  
  if (!o)
    o = (quoting_options *)default_quoting_options;
  v1 = o->flags;
  o->flags = i;
  return v1;
}

// Function: set_custom_quoting @ 0x4d60
void set_custom_quoting(unsigned int *a0,long a1,long a2)
{
  if (!a0)
    a0 = (unsigned int *)default_quoting_options;
  *a0 = 10;
  if ((a1) && (a2)) {
    *(long *)&a0[10] = a1;
    *(long *)&a0[0xc] = a2;
    return;
  }
  abort(); // no-return
}

// Function: quotearg_buffer @ 0x4d90
unsigned long quotearg_buffer(char *buffer,unsigned long buffersize,char *arg,unsigned long argsize,quoting_options *o)
{
  int v1;
  int *v2; // rax
  unsigned long v3; // rax
  
  if (!o)
    o = (quoting_options *)default_quoting_options;
  v2 = __errno_location();
  v1 = *v2;
  v3 = quotearg_buffer_restyled(buffer,buffersize,arg,argsize,o->style,o->flags,o->quote_these_too,o->left_quote,o->right_quote);
  *v2 = v1;
  return v3;
}

// Function: quotearg_alloc_mem @ 0x4e10
char * quotearg_alloc_mem(char *arg,unsigned long argsize,unsigned long *size,quoting_options *o)
{
  int v1;
  int *v2; // rax
  unsigned long v3; // rax
  char *v4; // rax
  unsigned int v5; // ebp
  
  if (!o)
    o = (quoting_options *)default_quoting_options;
  v2 = __errno_location();
  v1 = *v2;
  v5 = (unsigned int)(size == NULL) | o->flags;
  v3 = quotearg_buffer_restyled(0,0,arg,argsize,o->style,v5,o->quote_these_too,o->left_quote,o->right_quote);
  v4 = xcharalloc(v3 + 1);
  quotearg_buffer_restyled(v4,v3 + 1,arg,argsize,o->style,v5,o->quote_these_too,o->left_quote,o->right_quote);
  *v2 = v1;
  if (size)
    *size = v3;
  return v4;
}

// Function: quotearg_alloc @ 0x4f00
char * quotearg_alloc(char *arg,unsigned long argsize,quoting_options *o)
{
  return quotearg_alloc_mem(arg,argsize,NULL,o); // tail-call
}

// Function: quotearg_free @ 0x4f10
void quotearg_free(void) // return-dupe
{
  void *p;
  char *p_1;
  int v1; // ebx
  unsigned long *v2;
  
  p_1 = slotvec;
  if (2 <= nslots) {
    v2 = (unsigned long *)&slotvec[0x18];
    v1 = 1;
    do {
      p = (void *)*v2;
      v1 += 1;
      v2 = &v2[2];
      rpl_free(p);
    } while (v1 < nslots);
  }
  if (*(char **)&p_1[8] != slot0) {
    rpl_free(*(char **)&p_1[8]);
    slotvec0._8_8_ = slot0;
    slotvec0._0_8_ = 0x100;
  }
  if (p_1 == slotvec0) {
    nslots = 1;
    return;
  }
  rpl_free(p_1);
  nslots = 1;
  slotvec = slotvec0;
}

// Function: quotearg_n @ 0x4fa0
void quotearg_n(int a0,char *a1)
{
  quotearg_n_options(a0,a1,0xffffffffffffffff,(quoting_options *)default_quoting_options); // tail-call
}

// Function: quotearg_n_mem @ 0x4fc0
void quotearg_n_mem(int a0,char *a1,unsigned long a2)
{
  quotearg_n_options(a0,a1,a2,(quoting_options *)default_quoting_options); // tail-call
}

// Function: quotearg @ 0x4fd0
char * quotearg(char *arg)
{
  return quotearg_n_options(0,arg,0xffffffffffffffff,(quoting_options *)default_quoting_options); // tail-call
}

// Function: quotearg_mem @ 0x4ff0
char * quotearg_mem(char *arg,unsigned long argsize)
{
  return quotearg_n_options(0,arg,argsize,(quoting_options *)default_quoting_options); // tail-call
}

// Function: quotearg_n_style @ 0x5010
void quotearg_n_style(int a0,quoting_style a1,char *a2)
{
  quoting_options v1; // stack - 0x48
  
  if (a1 != custom_quoting_style) {
    v1.flags = 0;
    v1.quote_these_too[0] = 0;
    v1.quote_these_too[1] = 0;
    v1.quote_these_too[2] = 0;
    v1.quote_these_too[3] = 0;
    v1.quote_these_too[4] = 0;
    v1.quote_these_too[5] = 0;
    v1.quote_these_too[6] = 0;
    v1.quote_these_too[7] = 0;
    v1.left_quote = NULL;
    v1.right_quote = NULL;
    v1.style = a1;
    quotearg_n_options(a0,a2,0xffffffffffffffff,&v1);
    return;
  }
  abort(); // no-return
}

// Function: quotearg_n_style_mem @ 0x50a0
void quotearg_n_style_mem(int a0,quoting_style a1,char *a2,unsigned long a3)
{
  quoting_options v1; // stack - 0x48
  
  if (a1 != custom_quoting_style) {
    v1.flags = 0;
    v1.quote_these_too[0] = 0;
    v1.quote_these_too[1] = 0;
    v1.quote_these_too[2] = 0;
    v1.quote_these_too[3] = 0;
    v1.quote_these_too[4] = 0;
    v1.quote_these_too[5] = 0;
    v1.quote_these_too[6] = 0;
    v1.quote_these_too[7] = 0;
    v1.left_quote = NULL;
    v1.right_quote = NULL;
    v1.style = a1;
    quotearg_n_options(a0,a2,a3,&v1);
    return;
  }
  abort(); // no-return
}

// Function: quotearg_style @ 0x5130
char * quotearg_style(quoting_style s,char *arg)
{
  return (char *)quotearg_n_style(0,s,arg); // tail-call
}

// Function: quotearg_style_mem @ 0x5140
char * quotearg_style_mem(quoting_style s,char *arg,unsigned long argsize)
{
  return (char *)quotearg_n_style_mem(0,s,arg,argsize); // tail-call
}

// Function: quotearg_char_mem @ 0x5150
char * quotearg_char_mem(char *arg,unsigned long argsize,char ch)
{
  quoting_options options; // stack - 0x48
  unsigned int v1;
  char *v2; // rax
  
  options._0_8_ = default_quoting_options._0_8_;
  options.quote_these_too._0_8_ = default_quoting_options._8_8_;
  options.right_quote = default_quoting_options._48_8_;
  options.quote_these_too._8_8_ = default_quoting_options._16_8_;
  options.quote_these_too._16_8_ = default_quoting_options._24_8_;
  options.quote_these_too._24_8_ = default_quoting_options._32_8_;
  options.left_quote = default_quoting_options._40_8_;
  v1 = options.quote_these_too[(unsigned char)ch >> 5];
  options.quote_these_too[(unsigned char)ch >> 5] = (~(v1 >> (ch & 0x1fU)) & 1) << (ch & 0x1fU) ^ v1;
  v2 = quotearg_n_options(0,arg,argsize,&options);
  return v2;
}

// Function: quotearg_char @ 0x51f0
void quotearg_char(char *a0,char a1)
{
  quotearg_char_mem(a0,0xffffffffffffffff,a1); // tail-call
}

// Function: quotearg_colon @ 0x5200
char * quotearg_colon(char *arg)
{
  return quotearg_char_mem(arg,0xffffffffffffffff,':'); // tail-call
}

// Function: quotearg_colon_mem @ 0x5220
char * quotearg_colon_mem(char *arg,unsigned long argsize)
{
  return quotearg_char_mem(arg,argsize,':'); // tail-call
}

// Function: quotearg_n_style_colon @ 0x5230
void quotearg_n_style_colon(int a0,quoting_style a1,char *a2)
{
  quoting_options v1; // stack - 0x48
  
  if (a1 != custom_quoting_style) {
    v1.quote_these_too[0] = 0;
    v1.quote_these_too[1] = 0x4000000;
    v1.flags = 0;
    v1.quote_these_too[2] = 0;
    v1.quote_these_too[3] = 0;
    v1.quote_these_too[4] = 0;
    v1.quote_these_too[5] = 0;
    v1.quote_these_too[6] = 0;
    v1.quote_these_too[7] = 0;
    v1.left_quote = NULL;
    v1.right_quote = NULL;
    v1.style = a1;
    quotearg_n_options(a0,a2,0xffffffffffffffff,&v1);
    return;
  }
  abort(); // no-return
}

// Function: quotearg_n_custom_mem @ 0x52d0
char * quotearg_n_custom_mem(int n,char *left_quote,char *right_quote,char *arg,unsigned long argsize)
{
  quoting_options o; // stack - 0x48
  char *v1; // rax
  
  o.quote_these_too._0_8_ = default_quoting_options._8_8_;
  o.quote_these_too._8_8_ = default_quoting_options._16_8_;
  o.quote_these_too._16_8_ = default_quoting_options._24_8_;
  o.quote_these_too._24_8_ = default_quoting_options._32_8_;
  o.left_quote = default_quoting_options._40_8_;
  o.right_quote = default_quoting_options._48_8_;
  o.flags = (int)((unsigned long)default_quoting_options._0_8_ >> 0x20);
  o.style = 10;
  if ((left_quote) && (right_quote)) {
    o.left_quote = left_quote;
    o.right_quote = right_quote;
    v1 = quotearg_n_options(n,arg,argsize,&o);
    return v1;
  }
  abort(); // no-return
}

// Function: quotearg_n_custom @ 0x5370
void quotearg_n_custom(int a0,char *a1,char *a2,char *a3)
{
  quotearg_n_custom_mem(a0,a1,a2,a3,0xffffffffffffffff); // tail-call
}

// Function: quotearg_custom @ 0x5380
char * quotearg_custom(char *left_quote,char *right_quote,char *arg)
{
  return quotearg_n_custom_mem(0,left_quote,right_quote,arg,0xffffffffffffffff); // tail-call
}

// Function: quotearg_custom_mem @ 0x53a0
char * quotearg_custom_mem(char *left_quote,char *right_quote,char *arg,unsigned long argsize)
{
  return quotearg_n_custom_mem(0,left_quote,right_quote,arg,argsize); // tail-call
}

// Function: quote_n_mem @ 0x53c0
void quote_n_mem(int a0,char *a1,unsigned long a2)
{
  quotearg_n_options(a0,a1,a2,(quoting_options *)quote_quoting_options); // tail-call
}

// Function: quote_mem @ 0x53d0
char * quote_mem(char *arg,unsigned long argsize)
{
  return quotearg_n_options(0,arg,argsize,(quoting_options *)quote_quoting_options); // tail-call
}

// Function: quote_n @ 0x53f0
void quote_n(int a0,char *a1)
{
  quotearg_n_options(a0,a1,0xffffffffffffffff,(quoting_options *)quote_quoting_options); // tail-call
}

// Function: quote @ 0x5410
char * quote(char *arg)
{
  return quotearg_n_options(0,arg,0xffffffffffffffff,(quoting_options *)quote_quoting_options); // tail-call
}

// Function: version_etc_arn @ 0x5430
void version_etc_arn(_IO_FILE *stream,char *command_name,char *package,char *version,char **authors,unsigned long n_authors) // return-dupe
{
  char *v1;
  char *v2;
  char *v3; // rax
  char *v4;
  char *v5;
  char *v6;
  char *v7; // stack - 0x58
  char *v8; // stack - 0x50
  char *v9; // stack - 0x48
  
  if (command_name) // branch-flip
    __fprintf_chk(stream,1,"%s (%s) %s\n",command_name,package,version);
  else {
    __fprintf_chk(stream,1,"%s %s\n",package,version);
  }
  __fprintf_chk(stream,1,"Copyright %s %d Free Software Foundation, Inc.",dcgettext(NULL,"(C)",5),0x7e6);
  fputc_unlocked(10,stream);
  __fprintf_chk(stream,1,dcgettext(NULL,"License GPLv3+: GNU GPL version 3 or later <%s>.\nThis is free software: you are free to change and redistribute it.\nThere is NO WARRANTY, to the extent permitted by law.\n",5),"https://gnu.org/licenses/gpl.html");
  fputc_unlocked(10,stream);
  switch(n_authors) {
    case 0:
      return;
    case 1:
      v4 = *authors;
      __fprintf_chk(stream,1,dcgettext(NULL,"Written by %s.\n",5),v4); // tail-call
      return;
    case 2:
      v4 = authors[1];
      v5 = *authors;
      __fprintf_chk(stream,1,dcgettext(NULL,"Written by %s and %s.\n",5),v5,v4); // tail-call
      return;
    case 3:
      v4 = authors[2];
      v5 = authors[1];
      v6 = *authors;
      __fprintf_chk(stream,1,dcgettext(NULL,"Written by %s, %s, and %s.\n",5),v6,v5,v4); // tail-call
      return;
    case 4:
      v4 = authors[3];
      v5 = authors[2];
      v6 = authors[1];
      v2 = *authors;
      v1 = dcgettext(NULL,"Written by %s, %s, %s,\nand %s.\n",5);
      break;
    case 5:
      v4 = authors[3];
      v5 = authors[2];
      v6 = authors[1];
      v2 = *authors;
      v1 = dcgettext(NULL,"Written by %s, %s, %s,\n%s, and %s.\n",5);
      break;
    case 6:
      v7 = authors[1];
      v5 = authors[5];
      v6 = authors[4];
      v2 = authors[3];
      v4 = authors[2];
      v1 = *authors;
      v3 = dcgettext(NULL,"Written by %s, %s, %s,\n%s, %s, and %s.\n",5);
      __fprintf_chk(stream,1,v3,v1,v7,v4,v2,v6,v5); // return-dupe
      return;
    case 7:
      v4 = authors[2];
      v5 = authors[5];
      v6 = authors[4];
      v2 = authors[3];
      v7 = authors[1];
      v1 = *authors;
      v3 = dcgettext(NULL,"Written by %s, %s, %s,\n%s, %s, %s, and %s.\n",5);
      __fprintf_chk(stream,1,v3,v1,v7,v4,v2,v6,v5);
      return;
    case 8:
      v9 = authors[7];
      v7 = authors[2];
      v8 = authors[1];
      v4 = authors[6];
      v5 = authors[5];
      v6 = authors[4];
      v2 = authors[3];
      v1 = *authors;
      v3 = dcgettext(NULL,"Written by %s, %s, %s,\n%s, %s, %s, %s,\nand %s.\n",5);
      __fprintf_chk(stream,1,v3,v1,v8,v7,v2,v6,v5,v4,v9); // return-dupe
      return;
    case 9:
      v9 = authors[7];
      v3 = "Written by %s, %s, %s,\n%s, %s, %s, %s,\n%s, and %s.\n";
      v7 = authors[2];
      v8 = authors[1];
      v4 = authors[6];
      v5 = authors[5];
      v6 = authors[4];
      v2 = authors[3];
      v1 = *authors;
label_55f3:
      v3 = dcgettext(NULL,v3,5);
      __fprintf_chk(stream,1,v3,v1,v8,v7,v2,v6,v5,v4,v9);
      return;
    default:
      v9 = authors[7];
      v3 = "Written by %s, %s, %s,\n%s, %s, %s, %s,\n%s, %s, and others.\n";
      v7 = authors[2];
      v8 = authors[1];
      v4 = authors[6];
      v5 = authors[5];
      v6 = authors[4];
      v2 = authors[3];
      v1 = *authors;
      goto label_55f3;
    
  }
  __fprintf_chk(stream,1,v1,v2,v6,v5,v4);
}

// Function: version_etc_ar @ 0x5870
void version_etc_ar(_IO_FILE *stream,char *command_name,char *package,char *version,char **authors)
{
  unsigned long n_authors;
  char *v1;
  
  n_authors = 0;
  v1 = *authors;
  while (v1) {
    n_authors += 1;
    v1 = authors[n_authors];
  }
  version_etc_arn(stream,command_name,package,version,authors,n_authors); // tail-call
}

// Function: version_etc_va @ 0x5890
void version_etc_va(_IO_FILE *a0,char *a1,char *a2,char *a3,unsigned int *a4)
{
  unsigned long n_authors; // r9
  unsigned int v1;
  long *v2;
  char *v3;
  char *v4 [11];
  
  n_authors = 0;
  do {
    v1 = *a4;
    if (0x30 <= v1) { // branch-flip
      v2 = *(long **)&a4[2];
      *(long **)&a4[2] = &v2[1];
      v3 = (char *)*v2;
      v4[n_authors] = v3;
    }
    else {
      *a4 = v1 + 8;
      v3 = *(char **)((unsigned long)v1 + *(long *)&a4[4]);
      v4[n_authors] = v3;
    }
  } while ((v3) && (n_authors = n_authors + 1, n_authors != 10));
  version_etc_arn(a0,a1,a2,a3,v4,n_authors);
}

// Function: version_etc @ 0x5920
void version_etc(_IO_FILE *stream,char *command_name,char *package,char *version,...)
{
  unsigned long n_authors; // r9
  char *v1;
  char *v2 [11];
  unsigned long v3 [4];
  unsigned long v4; // r8
  unsigned long v5; // r9
  unsigned long v6;
  unsigned long *v7;
  unsigned long *v8; // r11
  unsigned long v9 [3]; // stack - 0x28
  
  v6 = 0x20;
  v9[0] = v4;
  v7 = (unsigned long *)&Stack0000000000000008;
  v9[1] = v5;
  n_authors = 0;
  do {
    if (0x30 <= (unsigned int)v6) { // branch-flip
      v8 = &v7[1];
      v1 = (char *)*v7;
      v2[n_authors] = v1;
      v7 = v8;
    }
    else {
      v1 = *(char **)((long)v3 + v6);
      v2[n_authors] = v1;
      v6 = (unsigned long)((unsigned int)v6 + 8);
    }
  } while ((v1) && (n_authors = n_authors + 1, n_authors != 10));
  version_etc_arn(stream,command_name,package,version,v2,n_authors);
}

// Function: emit_bug_reporting_address @ 0x5a00
void emit_bug_reporting_address(void)
{
  fputc_unlocked(10,stdout);
  __printf_chk(1,dcgettext(NULL,"Report bugs to: %s\n",5),"bug-coreutils@gnu.org");
  __printf_chk(1,dcgettext(NULL,"%s home page: <%s>\n",5),"GNU coreutils","https://www.gnu.org/software/coreutils/");
  __printf_chk(1,dcgettext(NULL,"General help using GNU software: <%s>\n",5),"https://www.gnu.org/gethelp/"); // tail-call
}

// Function: xmalloc @ 0x5aa0
void xmalloc(unsigned long a0)
{
  if (malloc(a0))
    return;
  xalloc_die(); // no-return
}

// Function: ximalloc @ 0x5ac0
void ximalloc(unsigned long a0)
{
  if (malloc(a0))
    return;
  xalloc_die(); // no-return
}

// Function: xcharalloc @ 0x5ae0
char * xcharalloc(unsigned long n)
{
  char *v1; // rax
  
  v1 = malloc(n);
  if (v1)
    return v1;
  xalloc_die(); // no-return
}

// Function: xrealloc @ 0x5b00
void xrealloc(void *a0,unsigned long a1)
{
  if ((!realloc(a0,a1)) && ((!a0 || (a1))))
    xalloc_die(); // no-return
}

// Function: xirealloc @ 0x5b30
void * xirealloc(void *p,long s)
{
  void *v1; // rax
  
  v1 = realloc(p,s | (unsigned long)(s == 0));
  if (v1)
    return v1;
  xalloc_die(); // no-return
}

// Function: xreallocarray @ 0x5b60
void xreallocarray(void *a0,unsigned long a1,unsigned long a2)
{
  if ((!reallocarray(a0,a1,a2)) && ((!a0 || ((a1 && (a2))))))
    xalloc_die(); // no-return
}

// Function: xnrealloc @ 0x5ba0
void * xnrealloc(void *p,unsigned long n,unsigned long s)
{
  return (void *)xreallocarray(); // tail-call
}

// Function: xireallocarray @ 0x5bb0
void * xireallocarray(void *p,long n,long s)
{
  void *v1; // rax
  
  if ((!n) || (!s)) {
    n = 1;
    s = 1;
  }
  v1 = reallocarray(p,n,s);
  if (v1)
    return v1;
  xalloc_die(); // no-return
}

// Function: xnmalloc @ 0x5bf0
void * xnmalloc(unsigned long n,unsigned long s)
{
  return (void *)xreallocarray(0,n,s); // tail-call
}

// Function: xinmalloc @ 0x5c00
void * xinmalloc(long n,long s)
{
  return xireallocarray(NULL,n,s); // tail-call
}

// Function: x2nrealloc @ 0x5c10
void * x2nrealloc(void *p,unsigned long *pn,unsigned long s)
{
  char v1 [16];
  unsigned long v2; // rax
  void *v3; // rax
  unsigned long v4;
  bool v5; // cf
  
  v4 = *pn;
  if (p) { // branch-flip
    v2 = (v4 >> 1) + 1;
    v5 = CARRY8(v4,v2);
    v4 += v2;
    if (v5)
      xalloc_die(); // no-return
  }
  else if (!v4) {
    v1._8_8_ = 0;
    v1._0_8_ = s;
    v4 = (unsigned long)(0x81 <= s) + SUB168((ZEXT816(0) << 0x40 | ZEXT816(0x80)) / v1._0_16_,0);
  }
  v3 = (void *)xreallocarray(p,v4,s);
  *pn = v4;
  return v3;
}

// Function: x2realloc @ 0x5c80
void * x2realloc(void *p,unsigned long *ps)
{
  return x2nrealloc(p,ps,1); // tail-call
}

// Function: xpalloc @ 0x5c90
void * xpalloc(void *pa,long *pn,long n_incr_min,long n_max,long s)
{
  long v1;
  void *v2; // rax
  long v3;
  long v4;
  
  v1 = *pn;
  v3 = (v1 >> 1) + v1;
  if (SCARRY8(v1 >> 1,v1))
    v3 = 0x7fffffffffffffff;
  if ((n_max < v3) && (0 <= n_max))
    v3 = n_max;
  v4 = v3 * s;
  if (SEXT816(v4) != SEXT816(v3) * SEXT816(s)) // branch-flip
    v4 = 0x7fffffffffffffff;
  else {
    if (0x80 <= v4) goto label_5d36;
    v4 = 0x80;
  }
  v3 = v4 / s;
  v4 -= v4 % s;
label_5d36:
  if (!pa)
    *pn = 0;
  if ((v3 - v1 < n_incr_min) && ((v3 = v1 + n_incr_min, SCARRY8(v1,n_incr_min) || (((n_max < v3 && (-1 < n_max)) || (v4 = v3 * s, SEXT816(v4) != SEXT816(v3) * SEXT816(s)))))))
    xalloc_die(); // no-return
  v2 = (void *)xrealloc(pa,v4);
  *pn = v3;
  return v2;
}

// Function: xzalloc @ 0x5d70
void * xzalloc(unsigned long s)
{
  void *v1; // rax
  
  v1 = calloc(s,1);
  if (v1)
    return v1;
  xalloc_die(); // no-return
}

// Function: xizalloc @ 0x5d90
void * xizalloc(long s)
{
  void *v1; // rax
  
  v1 = calloc(s,1);
  if (v1)
    return v1;
  xalloc_die(); // no-return
}

// Function: xcalloc @ 0x5db0
void xcalloc(unsigned long a0,unsigned long a1)
{
  if (calloc(a0,a1))
    return;
  xalloc_die(); // no-return
}

// Function: xicalloc @ 0x5dd0
void xicalloc(unsigned long a0,unsigned long a1)
{
  if (calloc(a0,a1))
    return;
  xalloc_die(); // no-return
}

// Function: xmemdup @ 0x5df0
void * xmemdup(void *p,unsigned long s)
{
  void *v1; // rax
  
  v1 = malloc(s);
  if (v1)
    return memcpy(v1,p,s); // tail-call
  xalloc_die(); // no-return
}

// Function: ximemdup @ 0x5e30
void * ximemdup(void *p,long s)
{
  void *v1; // rax
  
  v1 = malloc(s);
  if (v1)
    return memcpy(v1,p,s); // tail-call
  xalloc_die(); // no-return
}

// Function: ximemdup0 @ 0x5e70
char * ximemdup0(void *p,long s)
{
  void *v1; // rax
  
  v1 = malloc(s + 1);
  if (v1) {
    *(char *)((long)v1 + s) = 0;
    return memcpy(v1,p,s); // tail-call
  }
  xalloc_die(); // no-return
}

// Function: xstrdup @ 0x5eb0
char * xstrdup(char *string)
{
  unsigned long v1; // rax
  
  v1 = strlen(string);
  return xmemdup(string,v1 + 1); // tail-call
}

// Function: xalloc_die @ 0x5ed0
void xalloc_die(void)
{
  error(exit_failure,0,"%s",dcgettext(NULL,"memory exhausted",5));
  abort(); // no-return
}

// Function: xnumtoumax @ 0x5f10
unsigned long xnumtoumax(char *n_str,int base,unsigned long min,unsigned long max,char *suffixes,char *err,int err_exit)
{
  unsigned long tnum; // stack - 0x38
  strtol_error v1; // eax
  int *v2; // rax
  int v3;
  
  v1 = xstrtoumax(n_str,NULL,base,&tnum,suffixes);
  if (v1 != LONGINT_OK) { // branch-flip
    v2 = __errno_location();
    if (v1 != LONGINT_OVERFLOW) {
      if (v1 == LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW)
        *v2 = 0;
      goto label_5fb0;
    }
  }
  else {
    if ((min <= tnum) && (tnum <= max))
      return tnum;
    v2 = __errno_location();
    if (tnum <= 0x3fffffff) {
      *v2 = 0x22;
      goto label_5fb0;
    }
  }
  *v2 = 0x4b;
label_5fb0:
  quote(n_str);
  v3 = *v2;
  if (v3 == 0x16) goto label_5fe8;
  do {
    if (!err_exit)
      err_exit = 1;
    error(err_exit,v3,"%s: %s",err);
label_5fe8:
    v3 = 0;
  } while( true );
}

// Function: xdectoumax @ 0x6010
unsigned long xdectoumax(char *n_str,unsigned long min,unsigned long max,char *suffixes,char *err,int err_exit)
{
  return xnumtoumax(n_str,10,min,max,suffixes,err,err_exit);
}

// Function: xstrtoumax @ 0x6040
strtol_error xstrtoumax(char *s,char **ptr,int strtol_base,unsigned long *val,char *valid_suffixes)
{
  long t_ptr;
  unsigned char v1;
  char v10 [16];
  char v11 [16];
  char v12 [16];
  char v13 [16];
  char v14 [16];
  char v15 [16];
  char v16 [16];
  char v17 [16];
  char v18 [16];
  char v19 [16];
  bool v2;
  char v20 [16];
  char v21 [16];
  char v22 [16];
  unsigned int v23; // eax
  int *v24; // rax
  unsigned char *v25;
  unsigned long v26;
  unsigned long v27;
  strtol_error v28;
  unsigned char v29;
  char v3 [16];
  unsigned long v30; // rbx
  int v31; // esi
  int v32; // edi
  strtol_error v33;
  strtol_error v34;
  char v4 [16];
  char v5 [16];
  char v6 [16];
  char v7 [16];
  char v8 [16];
  char v9 [16];
  
  if (0x25 <= (unsigned int)strtol_base)
    __assert_fail("0 <= strtol_base && strtol_base <= 36","lib/xstrtol.c",0x55,"xstrtoumax"); // no-return
  if (!ptr)
    ptr = &t_ptr;
  v24 = __errno_location();
  *v24 = 0;
  v29 = *s;
  v25 = (unsigned char *)s;
  while (*(unsigned char *)(*(long *)__ctype_b_loc() + 1 + (unsigned long)v29 * 2) & 0x20) {
    v25 = &v25[1];
    v29 = *v25;
  }
  if (v29 == 0x2d)
    return LONGINT_INVALID;
  v26 = __strtoul_internal(s,ptr,strtol_base,0);
  v25 = (unsigned char *)*ptr;
  if (v25 != (unsigned char *)s) { // branch-flip
    if (*v24) { // branch-flip
      v34 = LONGINT_OVERFLOW;
      v33 = LONGINT_OVERFLOW;
      if (*v24 != 0x22)
        return LONGINT_INVALID;
    }
    else {
      v34 = LONGINT_OK;
      v33 = LONGINT_OK;
    }
    if ((!valid_suffixes) || (v29 = *v25, !v29)) goto label_60fb;
    if (!strchr(valid_suffixes,(int)(char)v29)) goto label_60fe;
  }
  else {
    if ((!valid_suffixes) || (v29 = *s, !v29))
      return LONGINT_INVALID;
    v34 = LONGINT_OK;
    v33 = LONGINT_OK;
    v26 = 1;
    if (!strchr(valid_suffixes,(int)(char)v29))
      return LONGINT_INVALID;
  }
  v23 = (unsigned int)v29 - 0x45;
  if (((0x30 <= (unsigned char)v23) || (!(0x814400308945U >> ((unsigned long)v23 & 0x3f) & 1))) || (!strchr(valid_suffixes,0x30))) {
    v31 = 1;
    v27 = 0x400;
  }
  else {
    v1 = v25[1];
    if (v1 != 0x44) { // branch-flip
      if (v1 != 0x69) { // branch-flip
        v27 = 1000;
        v31 = (v1 == 0x42) + 1;
        if (v1 != 0x42)
          v27 = 0x400;
      }
      else {
        v27 = 0x400;
        v31 = (v25[2] == 0x42) + 1 + (unsigned int)(v25[2] == 0x42);
      }
    }
    else {
      v31 = 2;
      v27 = 1000;
    }
  }
  v28 = v33;
  switch((unsigned int)v29 - 0x42 & 0xff) {
    case 0:
      v27 = v26 << 10;
      v30 = v26 >> 0x36;
      v26 = 0xffffffffffffffff;
      v28 = LONGINT_OVERFLOW;
      v34 = v28;
      if (!v30) {
        v26 = v27;
        v28 = v33;
        v34 = v33;
      }
      break;
    default:
label_60fe:
      *val = v26;
      return v33 | LONGINT_INVALID_SUFFIX_CHAR;
    case 3:
      v32 = 6;
      v34 = LONGINT_OK;
      do {
        v8._8_8_ = 0;
        v8._0_8_ = v26;
        v18._8_8_ = 0;
        v18._0_8_ = v27;
        v26 = SUB168(v8._0_16_ * v18._0_16_,0);
        if (SUB168(v8._0_16_ * v18._0_16_,8)) {
          v34 = LONGINT_OVERFLOW;
          v26 = 0xffffffffffffffff;
        }
        v32 -= 1;
      } while (v32);
      goto label_62ac;
    case 5:
    case 0x25:
      v3._8_8_ = 0;
      v3._0_8_ = v26;
      v13._8_8_ = 0;
      v13._0_8_ = v27;
      if ((!SUB168(v3._0_16_ * v13._0_16_,8)) && (v4._8_8_ = 0, v4._0_8_ = SUB168(v3._0_16_ * v13._0_16_,0), v14._8_8_ = 0, v14._0_8_ = v27, v26 = SUB168(v4._0_16_ * v14._0_16_,0), !SUB168(v4._0_16_ * v14._0_16_,8))) goto label_638a;
label_621c:
      v26 = 0xffffffffffffffff;
      v28 = LONGINT_OVERFLOW;
      v34 = LONGINT_OVERFLOW;
      break;
    case 9:
    case 0x29:
      v5._8_8_ = 0;
      v5._0_8_ = v26;
      v15._8_8_ = 0;
      v15._0_8_ = (long)(int)v27;
      v26 = SUB168(v5._0_16_ * v15._0_16_,0);
      if (SUB168(v5._0_16_ * v15._0_16_,8)) goto label_621c;
      break;
    case 0xb:
    case 0x2b:
      v11._8_8_ = 0;
      v11._0_8_ = v26;
      v21._8_8_ = 0;
      v21._0_8_ = v27;
      v26 = SUB168(v11._0_16_ * v21._0_16_,0);
      if (SUB168(v11._0_16_ * v21._0_16_,8)) goto label_621c;
label_638a:
      v12._8_8_ = 0;
      v12._0_8_ = v26;
      v22._8_8_ = 0;
      v22._0_8_ = v27;
      v26 = 0xffffffffffffffff;
      v28 = LONGINT_OVERFLOW;
      v34 = LONGINT_OVERFLOW;
      if (!SUB168(v12._0_16_ * v22._0_16_,8)) {
        v26 = SUB168(v12._0_16_ * v22._0_16_,0);
        v28 = v33;
        v34 = v33;
      }
      break;
    case 0xe:
      v32 = 5;
      v34 = LONGINT_OK;
      do {
        v10._8_8_ = 0;
        v10._0_8_ = v26;
        v20._8_8_ = 0;
        v20._0_8_ = v27;
        v26 = SUB168(v10._0_16_ * v20._0_16_,0);
        if (SUB168(v10._0_16_ * v20._0_16_,8)) {
          v34 = LONGINT_OVERFLOW;
          v26 = 0xffffffffffffffff;
        }
        v32 -= 1;
      } while (v32);
      goto label_62ac;
    case 0x12:
    case 0x32:
      v32 = 4;
      v34 = LONGINT_OK;
      do {
        v9._8_8_ = 0;
        v9._0_8_ = v26;
        v19._8_8_ = 0;
        v19._0_8_ = v27;
        v26 = SUB168(v9._0_16_ * v19._0_16_,0);
        if (SUB168(v9._0_16_ * v19._0_16_,8)) {
          v34 = LONGINT_OVERFLOW;
          v26 = 0xffffffffffffffff;
        }
        v32 -= 1;
      } while (v32);
      goto label_62ac;
    case 0x17:
      v32 = 8;
      v34 = LONGINT_OK;
      do {
        v7._8_8_ = 0;
        v7._0_8_ = v26;
        v17._8_8_ = 0;
        v17._0_8_ = v27;
        v26 = SUB168(v7._0_16_ * v17._0_16_,0);
        if (SUB168(v7._0_16_ * v17._0_16_,8)) {
          v34 = LONGINT_OVERFLOW;
          v26 = 0xffffffffffffffff;
        }
        v32 -= 1;
      } while (v32);
      goto label_62ac;
    case 0x18:
      v32 = 7;
      v34 = LONGINT_OK;
      do {
        v6._8_8_ = 0;
        v6._0_8_ = v26;
        v16._8_8_ = 0;
        v16._0_8_ = v27;
        v26 = SUB168(v6._0_16_ * v16._0_16_,0);
        if (SUB168(v6._0_16_ * v16._0_16_,8)) {
          v34 = LONGINT_OVERFLOW;
          v26 = 0xffffffffffffffff;
        }
        v32 -= 1;
      } while (v32);
label_62ac:
      v28 = v33 | v34;
      v34 = v33 | v34;
      break;
    case 0x20:
      v27 = v26 << 9;
      v30 = v26 >> 0x37;
      v26 = 0xffffffffffffffff;
      v28 = LONGINT_OVERFLOW;
      v34 = LONGINT_OVERFLOW;
      if (!v30) {
        v26 = v27;
        v28 = v33;
        v34 = v33;
      }
      break;
    case 0x21:
      break;
    case 0x35:
      v27 = v26 * 2;
      v2 = 0 <= (long)v26;
      v26 = 0xffffffffffffffff;
      v28 = LONGINT_OVERFLOW;
      v34 = LONGINT_OVERFLOW;
      if (v2) {
        v26 = v27;
        v28 = v33;
        v34 = v33;
      }
    
  }
  *ptr = (char *)&v25[v31];
  v33 = v34;
  if (v25[v31])
    v33 = v28 | LONGINT_INVALID_SUFFIX_CHAR;
label_60fb:
  *val = v26;
  return v33;
}

// Function: close_stream @ 0x6470
int close_stream(_IO_FILE *stream) // return-dupe
{
  unsigned int v1;
  int v2; // eax
  unsigned long v3; // rax
  
  v3 = __fpending(stream);
  v1 = stream->_flags;
  v2 = rpl_fclose(stream);
  if (v1 & 0x20) {
    if (v2)
      return -1;
    *__errno_location() = 0;
    return -1;
  }
  if (v2) {
    if (v3)
      return -1;
    return -(unsigned int)(*__errno_location() != 9);
  }
  return 0;
}

// Function: rpl_free @ 0x64d0
void rpl_free(void *p)
{
  int err [2]; // stack - 0x28
  int *v1; // rax
  
  v1 = __errno_location();
  err[0] = *v1;
  *v1 = 0;
  err[1] = err[0];
  free(p);
  *v1 = err[*v1 == 0];
}

// Function: locale_charset @ 0x6540
char * locale_charset(void) // early-return
{
  char *v1; // rax
  
  v1 = nl_langinfo(0xe);
  if (!v1)
    return "ASCII";
  if (*v1)
    return v1;
  return "ASCII";
}

// Function: rpl_mbrtowc @ 0x6580
unsigned long rpl_mbrtowc(int *pwc,char *s,unsigned long n,__mbstate_t *ps)
{
  unsigned long v1; // rax
  long wc;
  
  if (!pwc)
    pwc = &wc;
  v1 = mbrtowc(pwc,s,n,ps);
  if (((0xfffffffffffffffe <= v1) && (n)) && (!hard_locale(0))) {
    v1 = 1;
    *pwc = (unsigned int)(unsigned char)*s;
  }
  return v1;
}

// Function: hard_locale @ 0x6610
bool hard_locale(int category)
{
  char locale [257];
  unsigned char *v1;
  unsigned char *v2;
  long v3; // rcx
  unsigned char *v4;
  unsigned char *v5;
  bool v6; // cf
  bool v7; // cf
  bool v8;
  
  v6 = 0;
  v8 = 0;
  if (!setlocale_null_r(category,locale,0x101)) {
    v3 = 2;
    v8 = 1;
    v4 = (unsigned char *)locale;
    v5 = (unsigned char *)"C";
    do {
      if (!v3) break;
      v3 -= 1;
      v2 = &v5[1];
      v1 = &v4[1];
      v6 = *v4 < *v5;
      v8 = *v4 == *v5;
      v4 = v1;
      v5 = v2;
    } while (v8);
    v7 = 0;
    v6 = (!v6 && !v8) == v6;
    v8 = 0;
    if (!v6) {
      v3 = 6;
      v4 = (unsigned char *)locale;
      v5 = (unsigned char *)"POSIX";
      do {
        if (!v3) break;
        v3 -= 1;
        v2 = &v5[1];
        v1 = &v4[1];
        v7 = *v4 < *v5;
        v6 = *v4 == *v5;
        v4 = v1;
        v5 = v2;
      } while (v6);
      v8 = (!v7 && !v6) != v7;
    }
  }
  return v8;
}

// Function: setlocale_null_r @ 0x66b0
int setlocale_null_r(int category,char *buf,unsigned long bufsize) // return-dupe x2
{
  char *v1; // rax
  unsigned long v2; // rax
  
  v1 = setlocale(category,NULL);
  if (!v1) {
    if (!bufsize)
      return 0x16;
    *buf = '\0';
    return 0x16;
  }
  v2 = strlen(v1);
  if (bufsize <= v2) {
    if (!bufsize)
      return 0x22;
    memcpy(buf,v1,bufsize - 1);
    buf[bufsize - 1] = '\0';
    return 0x22;
  }
  memcpy(buf,v1,v2 + 1);
  return 0;
}

// Function: setlocale_null @ 0x6760
char * setlocale_null(int category)
{
  return setlocale(category,NULL); // tail-call
}

// Function: __libc_csu_init @ 0x6770
void __libc_csu_init(unsigned int a0,unsigned long a1,unsigned long a2)
{
  long v1; // rbx
  
  _init();
  v1 = 0;
  do {
    (**(void **)(v1 * 8 + 0x209a90))(a0,a1,a2);
    v1 += 1;
  } while (v1 != 1);
}

// Function: __libc_csu_fini @ 0x67e0
void __libc_csu_fini(void)
{
  return;
}

// Function: atexit @ 0x67f0
void atexit(unsigned long a0)
{
  __cxa_atexit(a0,0,dat_20a008); // tail-call
}

// Function: _fini @ 0x6800
void _fini(void)
{
  return;
}

