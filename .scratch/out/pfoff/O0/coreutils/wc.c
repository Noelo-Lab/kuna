// Function: _DT_INIT @ 0x2000
void _DT_INIT(void) // return-dupe
{
  if (!dat_efe0)
    return;
  (*dat_efe0)();
}

// Function: sub_2020 @ 0x2020
void sub_2020(void)
{
  (*dat_edb8)(); // jump-as-call
}

// Function: free @ 0x2440
void free(void *a0)
{
  (*dat_efc8)(); // jump-as-call
}

// Function: malloc @ 0x2450
void * malloc(unsigned long a0)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_efe8)(); // jump-as-call
  return v1;
}

// Function: __cxa_finalize @ 0x2460
void __cxa_finalize(void)
{
  (*dat_eff8)(); // jump-as-call
}

// Function: getenv @ 0x2470
char * getenv(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_edc0)(); // jump-as-call
  return v1;
}

// Function: abort @ 0x2480
void abort(void)
{
  (*dat_edc8)(); // jump-as-call
}

// Function: __errno_location @ 0x2490
int * __errno_location(void)
{
  int *v1; // rax
  
  v1 = (int *)(*dat_edd0)(); // jump-as-call
  return v1;
}

// Function: strncmp @ 0x24a0
int strncmp(char *a0,char *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_edd8)(); // jump-as-call
  return v1;
}

// Function: _exit @ 0x24b0
void _exit(int a0)
{
  (*dat_ede0)(); // jump-as-call
}

// Function: __fpending @ 0x24c0
unsigned long __fpending(FILE *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_ede8)(); // jump-as-call
  return v1;
}

// Function: ferror @ 0x24d0
int ferror(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_edf0)(); // jump-as-call
  return v1;
}

// Function: iswcntrl @ 0x24e0
int iswcntrl(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_edf8)(); // jump-as-call
  return v1;
}

// Function: reallocarray @ 0x24f0
void * reallocarray(void *a0,unsigned long a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_ee00)(); // jump-as-call
  return v1;
}

// Function: textdomain @ 0x2500
char * textdomain(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_ee08)(); // jump-as-call
  return v1;
}

// Function: fclose @ 0x2510
int fclose(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_ee10)(); // jump-as-call
  return v1;
}

// Function: bindtextdomain @ 0x2520
char * bindtextdomain(char *a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_ee18)(); // jump-as-call
  return v1;
}

// Function: __ctype_get_mb_cur_max @ 0x2530
unsigned long __ctype_get_mb_cur_max(void)
{
  unsigned long v1; // rax
  
  v1 = (*dat_ee20)(); // jump-as-call
  return v1;
}

// Function: strlen @ 0x2540
unsigned long strlen(char *a0)
{
  unsigned long v1; // rax
  
  v1 = (*dat_ee28)(); // jump-as-call
  return v1;
}

// Function: __stack_chk_fail @ 0x2550
void __stack_chk_fail(void)
{
  (*dat_ee30)(); // jump-as-call
}

// Function: getopt_long @ 0x2560
int getopt_long(int a0,char **a1,char *a2,option *a3,int *a4)
{
  int v1; // eax
  
  v1 = (*dat_ee38)(); // jump-as-call
  return v1;
}

// Function: mbrtowc @ 0x2570
unsigned long mbrtowc(void *a0,char *a1,unsigned long a2,mbstate_t *a3)
{
  unsigned long v1; // rax
  
  v1 = (*dat_ee40)(); // jump-as-call
  return v1;
}

// Function: gettext @ 0x2580
char * gettext(char *a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_ee48)(); // jump-as-call
  return v1;
}

// Function: strchr @ 0x2590
char * strchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_ee50)(); // jump-as-call
  return v1;
}

// Function: printf @ 0x25a0
int printf(char *a0,...)
{
  int v1; // eax
  
  v1 = (*dat_ee58)(); // jump-as-call
  return v1;
}

// Function: strrchr @ 0x25b0
char * strrchr(char *a0,int a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_ee60)(); // jump-as-call
  return v1;
}

// Function: lseek @ 0x25c0
void lseek(void)
{
  (*dat_ee68)(); // jump-as-call
}

// Function: __assert_fail @ 0x25d0
void __assert_fail(char *a0,char *a1,unsigned int a2,char *a3)
{
  (*dat_ee70)(); // jump-as-call
}

// Function: fputs @ 0x25e0
int fputs(char *a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_ee78)(); // jump-as-call
  return v1;
}

// Function: memset @ 0x25f0
void * memset(void *a0,int a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_ee80)(); // jump-as-call
  return v1;
}

// Function: fgetc @ 0x2600
int fgetc(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_ee88)(); // jump-as-call
  return v1;
}

// Function: close @ 0x2610
int close(int a0)
{
  int v1; // eax
  
  v1 = (*dat_ee90)(); // jump-as-call
  return v1;
}

// Function: posix_fadvise @ 0x2620
void posix_fadvise(void)
{
  (*dat_ee98)(); // jump-as-call
}

// Function: read @ 0x2630
long read(int a0,void *a1,unsigned long a2)
{
  long v1; // rax
  
  v1 = (*dat_eea0)(); // jump-as-call
  return v1;
}

// Function: memcmp @ 0x2640
int memcmp(void *a0,void *a1,unsigned long a2)
{
  int v1; // eax
  
  v1 = (*dat_eea8)(); // jump-as-call
  return v1;
}

// Function: fputs_unlocked @ 0x2650
int fputs_unlocked(char *a0,FILE *a1)
{
  int v1; // eax
  
  v1 = (*dat_eeb0)(); // jump-as-call
  return v1;
}

// Function: rawmemchr @ 0x2660
void * rawmemchr(void *a0,int a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_eeb8)(); // jump-as-call
  return v1;
}

// Function: ferror_unlocked @ 0x2670
int ferror_unlocked(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_eec0)(); // jump-as-call
  return v1;
}

// Function: calloc @ 0x2680
void * calloc(unsigned long a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_eec8)(); // jump-as-call
  return v1;
}

// Function: strcmp @ 0x2690
int strcmp(char *a0,char *a1)
{
  int v1; // eax
  
  v1 = (*dat_eed0)(); // jump-as-call
  return v1;
}

// Function: fprintf @ 0x26a0
int fprintf(FILE *a0,char *a1,...)
{
  int v1; // eax
  
  v1 = (*dat_eed8)(); // jump-as-call
  return v1;
}

// Function: feof @ 0x26b0
int feof(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_eee0)(); // jump-as-call
  return v1;
}

// Function: stat @ 0x26c0
int stat(char *a0,stat *a1)
{
  int v1; // eax
  
  v1 = (*dat_eee8)(); // jump-as-call
  return v1;
}

// Function: memcpy @ 0x26d0
void * memcpy(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_eef0)(); // jump-as-call
  return v1;
}

// Function: putchar_unlocked @ 0x26e0
int putchar_unlocked(int a0)
{
  int v1; // eax
  
  v1 = (*dat_eef8)(); // jump-as-call
  return v1;
}

// Function: fileno @ 0x26f0
int fileno(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_ef00)(); // jump-as-call
  return v1;
}

// Function: wcwidth @ 0x2700
int wcwidth(int a0)
{
  int v1; // eax
  
  v1 = (*dat_ef08)(); // jump-as-call
  return v1;
}

// Function: fflush @ 0x2710
int fflush(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_ef10)(); // jump-as-call
  return v1;
}

// Function: nl_langinfo @ 0x2720
char * nl_langinfo(int a0)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_ef18)(); // jump-as-call
  return v1;
}

// Function: __freading @ 0x2730
int __freading(FILE *a0)
{
  int v1; // eax
  
  v1 = (*dat_ef20)(); // jump-as-call
  return v1;
}

// Function: realloc @ 0x2740
void * realloc(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_ef28)(); // jump-as-call
  return v1;
}

// Function: setlocale @ 0x2750
char * setlocale(int a0,char *a1)
{
  char *v1; // rax
  
  v1 = (char *)(*dat_ef30)(); // jump-as-call
  return v1;
}

// Function: setvbuf @ 0x2760
int setvbuf(FILE *a0,char *a1,int a2,unsigned long a3)
{
  int v1; // eax
  
  v1 = (*dat_ef38)(); // jump-as-call
  return v1;
}

// Function: iswspace @ 0x2770
int iswspace(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_ef40)(); // jump-as-call
  return v1;
}

// Function: btowc @ 0x2780
unsigned int btowc(int a0)
{
  unsigned int v1; // eax
  
  v1 = (*dat_ef48)(); // jump-as-call
  return v1;
}

// Function: memmove @ 0x2790
void * memmove(void *a0,void *a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_ef50)(); // jump-as-call
  return v1;
}

// Function: error @ 0x27a0
void error(int a0,int a1,char *a2,...)
{
  (*dat_ef58)(); // jump-as-call
}

// Function: open @ 0x27b0
int open(char *a0,int a1,...)
{
  int v1; // eax
  
  v1 = (*dat_ef60)(); // jump-as-call
  return v1;
}

// Function: fseeko @ 0x27c0
int fseeko(FILE *a0,long a1,int a2)
{
  int v1; // eax
  
  v1 = (*dat_ef68)(); // jump-as-call
  return v1;
}

// Function: fopen @ 0x27d0
FILE * fopen(char *a0,char *a1)
{
  FILE *v1; // rax
  
  v1 = (FILE *)(*dat_ef70)(); // jump-as-call
  return v1;
}

// Function: sysconf @ 0x27e0
long sysconf(int a0)
{
  long v1; // rax
  
  v1 = (*dat_ef78)(); // jump-as-call
  return v1;
}

// Function: __cxa_atexit @ 0x27f0
void __cxa_atexit(void)
{
  (*dat_ef80)(); // jump-as-call
}

// Function: sysinfo @ 0x2800
int sysinfo(void *a0)
{
  int v1; // eax
  
  v1 = (*dat_ef88)(); // jump-as-call
  return v1;
}

// Function: getpagesize @ 0x2810
int getpagesize(void)
{
  int v1; // eax
  
  v1 = (*dat_ef90)(); // jump-as-call
  return v1;
}

// Function: exit @ 0x2820
void exit(int a0)
{
  (*dat_ef98)(); // jump-as-call
}

// Function: mbsinit @ 0x2830
int mbsinit(mbstate_t *a0)
{
  int v1; // eax
  
  v1 = (*dat_efa0)(); // jump-as-call
  return v1;
}

// Function: iswprint @ 0x2840
int iswprint(unsigned int a0)
{
  int v1; // eax
  
  v1 = (*dat_efa8)(); // jump-as-call
  return v1;
}

// Function: fstat @ 0x2850
int fstat(int a0,stat *a1)
{
  int v1; // eax
  
  v1 = (*dat_efb0)(); // jump-as-call
  return v1;
}

// Function: getdelim @ 0x2860
long getdelim(char **a0,void *a1,int a2,FILE *a3)
{
  long v1; // rax
  
  v1 = (*dat_efb8)(); // jump-as-call
  return v1;
}

// Function: __ctype_b_loc @ 0x2870
void * __ctype_b_loc(void)
{
  void *v1; // rax
  
  v1 = (void *)(*dat_efc0)(); // jump-as-call
  return v1;
}

// Function: sub_2880 @ 0x2880
void sub_2880(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_efd0)(main,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: sub_28b0 @ 0x28b0
void sub_28b0(void)
{
  return;
}

// Function: _FINI_0 @ 0x2920
void _FINI_0(void)
{
  if (!dat_f108) {
    if (dat_eff8)
      __cxa_finalize(dat_f008);
    sub_28b0();
    dat_f108 = 1;
    return;
  }
}

// Function: _INIT_0 @ 0x2960
void _INIT_0(void)
{
  return;
}

// Function: sub_2969 @ 0x2969
char sub_2969(char a0)
{
  return a0;
}

// Function: sub_2978 @ 0x2978
void sub_2978(void)
{
  FILE *v1;
  
  v1 = stdout;
  fputs_unlocked(gettext("\nWith no FILE, or when FILE is -, read standard input.\n"),v1);
}

// Function: sub_29a9 @ 0x29a9
void sub_29a9(char *a0) // ternary
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

// Function: sub_2c40 @ 0x2c40
unsigned int sub_2c40(long a0) // return-dupe
{
  if (((*(unsigned int *)(a0 + 0x18) & 0xf000) != 0x8000) && ((*(unsigned int *)(a0 + 0x18) & 0xf000) != 0xa000))
    return 0;
  return 1;
}

// Function: sub_2c7f @ 0x2c7f
unsigned int sub_2c7f(int a0,unsigned int *a1) // ternary
{
  unsigned int *v1;
  unsigned int v2;
  
  if (a0) { // branch-flip
    if (a0 != 1) {
      if (a0 != 2) {
        if (a0 != 3) {
          if (a0 != 4) {
            if (a0 != 5) {
              if (a0 != 6) {
                if (a0 != 7) {
                  if (a0 != 9) {
                    if (a0 != 10) {
                      if (a0 != 0xb) {
                        if (a0 != 0xd) {
                          if (a0 != 0xf) {
                            if (a0 != -0x7ffffffe) {
                              if (a0 != -0x7ffffffd) {
                                v1 = (a0 != -0x7ffffffc) ? (unsigned int *)cpuid(a0) : (unsigned int *)cpuid_brand_part3_info(0x80000004);
                              }
                              else {
                                v1 = (unsigned int *)cpuid_brand_part2_info(0x80000003);
                              }
                            }
                            else {
                              v1 = (unsigned int *)cpuid_brand_part1_info(0x80000002);
                            }
                          }
                          else {
                            v1 = (unsigned int *)cpuid_Quality_of_Service_info(0xf);
                          }
                        }
                        else {
                          v1 = (unsigned int *)cpuid_Processor_Extended_States_info(0xd);
                        }
                      }
                      else {
                        v1 = (unsigned int *)cpuid_Extended_Topology_info(0xb);
                      }
                    }
                    else {
                      v1 = (unsigned int *)cpuid_Architectural_Performance_Monitoring_info(10);
                    }
                  }
                  else {
                    v1 = (unsigned int *)cpuid_Direct_Cache_Access_info(9);
                  }
                }
                else {
                  v1 = (unsigned int *)cpuid_Extended_Feature_Enumeration_info(7);
                }
              }
              else {
                v1 = (unsigned int *)cpuid_Thermal_Power_Management_info(6);
              }
            }
            else {
              v1 = (unsigned int *)cpuid_MONITOR_MWAIT_Features_info(5);
            }
          }
          else {
            v1 = (unsigned int *)cpuid_Deterministic_Cache_Parameters_info(4);
          }
        }
        else {
          v1 = (unsigned int *)cpuid_serial_info(3);
        }
      }
      else {
        v1 = (unsigned int *)cpuid_cache_tlb_info(2);
      }
    }
    else {
      v1 = (unsigned int *)cpuid_Version_info(1);
    }
  }
  else {
    v1 = (unsigned int *)cpuid_basic_info(0);
  }
  v2 = *v1;
  if (a1)
    *a1 = v1[1];
  return v2;
}

// Function: sub_2cb7 @ 0x2cb7
unsigned long sub_2cb7(unsigned int a0,unsigned int *a1,unsigned int *a2,unsigned int *a3,unsigned int *a4) // return-dupe, ternary
{
  unsigned int *v1;
  unsigned int v2;
  unsigned int v3;
  unsigned int v4;
  unsigned int v5; // eax
  
  v5 = sub_2c7f(a0 & 0x80000000,NULL);
  if ((v5) && (a0 <= v5)) {
    if (a0) { // branch-flip
      if (a0 != 1) {
        if (a0 != 2) {
          if (a0 != 3) {
            if (a0 != 4) {
              if (a0 != 5) {
                if (a0 != 6) {
                  if (a0 != 7) {
                    if (a0 != 9) {
                      if (a0 != 10) {
                        if (a0 != 0xb) {
                          if (a0 != 0xd) {
                            if (a0 != 0xf) {
                              if (a0 != 0x80000002) {
                                if (a0 != 0x80000003) {
                                  v1 = (a0 != 0x80000004) ? (unsigned int *)cpuid(a0) : (unsigned int *)cpuid_brand_part3_info(0x80000004);
                                }
                                else {
                                  v1 = (unsigned int *)cpuid_brand_part2_info(0x80000003);
                                }
                              }
                              else {
                                v1 = (unsigned int *)cpuid_brand_part1_info(0x80000002);
                              }
                            }
                            else {
                              v1 = (unsigned int *)cpuid_Quality_of_Service_info(0xf);
                            }
                          }
                          else {
                            v1 = (unsigned int *)cpuid_Processor_Extended_States_info(0xd);
                          }
                        }
                        else {
                          v1 = (unsigned int *)cpuid_Extended_Topology_info(0xb);
                        }
                      }
                      else {
                        v1 = (unsigned int *)cpuid_Architectural_Performance_Monitoring_info(10);
                      }
                    }
                    else {
                      v1 = (unsigned int *)cpuid_Direct_Cache_Access_info(9);
                    }
                  }
                  else {
                    v1 = (unsigned int *)cpuid_Extended_Feature_Enumeration_info(7);
                  }
                }
                else {
                  v1 = (unsigned int *)cpuid_Thermal_Power_Management_info(6);
                }
              }
              else {
                v1 = (unsigned int *)cpuid_MONITOR_MWAIT_Features_info(5);
              }
            }
            else {
              v1 = (unsigned int *)cpuid_Deterministic_Cache_Parameters_info(4);
            }
          }
          else {
            v1 = (unsigned int *)cpuid_serial_info(3);
          }
        }
        else {
          v1 = (unsigned int *)cpuid_cache_tlb_info(2);
        }
      }
      else {
        v1 = (unsigned int *)cpuid_Version_info(1);
      }
    }
    else {
      v1 = (unsigned int *)cpuid_basic_info(0);
    }
    v2 = v1[1];
    v3 = v1[2];
    v4 = v1[3];
    *a1 = *v1;
    *a2 = v2;
    *a3 = v4;
    *a4 = v3;
    return 1;
  }
  return 0;
}

// Function: sub_2d2f @ 0x2d2f
unsigned long sub_2d2f(unsigned int a0,unsigned long a1,unsigned int *a2,unsigned int *a3,unsigned int *a4,unsigned int *a5) // return-dupe, ternary
{
  unsigned int *v1;
  unsigned int v2;
  unsigned int v3;
  unsigned int v4;
  unsigned int v5; // eax
  
  v5 = sub_2c7f(a0 & 0x80000000,NULL);
  if ((v5) && (a0 <= v5)) {
    if (a0) { // branch-flip
      if (a0 != 1) {
        if (a0 != 2) {
          if (a0 != 3) {
            if (a0 != 4) {
              if (a0 != 5) {
                if (a0 != 6) {
                  if (a0 != 7) {
                    if (a0 != 9) {
                      if (a0 != 10) {
                        if (a0 != 0xb) {
                          if (a0 != 0xd) {
                            if (a0 != 0xf) {
                              if (a0 != 0x80000002) {
                                if (a0 != 0x80000003) {
                                  v1 = (a0 != 0x80000004) ? (unsigned int *)cpuid(a0) : (unsigned int *)cpuid_brand_part3_info(0x80000004);
                                }
                                else {
                                  v1 = (unsigned int *)cpuid_brand_part2_info(0x80000003);
                                }
                              }
                              else {
                                v1 = (unsigned int *)cpuid_brand_part1_info(0x80000002);
                              }
                            }
                            else {
                              v1 = (unsigned int *)cpuid_Quality_of_Service_info(0xf);
                            }
                          }
                          else {
                            v1 = (unsigned int *)cpuid_Processor_Extended_States_info(0xd);
                          }
                        }
                        else {
                          v1 = (unsigned int *)cpuid_Extended_Topology_info(0xb);
                        }
                      }
                      else {
                        v1 = (unsigned int *)cpuid_Architectural_Performance_Monitoring_info(10);
                      }
                    }
                    else {
                      v1 = (unsigned int *)cpuid_Direct_Cache_Access_info(9);
                    }
                  }
                  else {
                    v1 = (unsigned int *)cpuid_Extended_Feature_Enumeration_info(7);
                  }
                }
                else {
                  v1 = (unsigned int *)cpuid_Thermal_Power_Management_info(6);
                }
              }
              else {
                v1 = (unsigned int *)cpuid_MONITOR_MWAIT_Features_info(5);
              }
            }
            else {
              v1 = (unsigned int *)cpuid_Deterministic_Cache_Parameters_info(4);
            }
          }
          else {
            v1 = (unsigned int *)cpuid_serial_info(3);
          }
        }
        else {
          v1 = (unsigned int *)cpuid_cache_tlb_info(2);
        }
      }
      else {
        v1 = (unsigned int *)cpuid_Version_info(1);
      }
    }
    else {
      v1 = (unsigned int *)cpuid_basic_info(0);
    }
    v2 = v1[1];
    v3 = v1[2];
    v4 = v1[3];
    *a2 = *v1;
    *a3 = v2;
    *a4 = v4;
    *a5 = v3;
    return 1;
  }
  return 0;
}

// Function: sub_2daf @ 0x2daf
unsigned long sub_2daf(void) // return-dupe x2, return-dupe x3
{
  bool v1;
  bool v2;
  unsigned int v3 [2]; // stack - 0x18
  unsigned int v4; // stack - 0x1c
  unsigned int v5; // stack - 0x20
  
  v5 = 0;
  v4 = 0;
  v3[0] = 0;
  v3[1] = 0;
  v2 = 0;
  v1 = 0;
  if ((sub_2cb7(1,&v5,&v4,v3,&v3[1])) && (v2 = 1, v3[0] & 0x8000000))
    v1 = 1;
  if (v1) {
    v3[1] = 0;
    v3[0] = 0;
    v4 = 0;
    v5 = 0;
    if (sub_2d2f(7,0,&v5,&v4,v3,&v3[1])) { // branch-flip
      if (!(v4 & 0x20))
        v1 = 0;
    }
    else {
      v2 = 0;
    }
  }
  if (!v2) {
    if (!dat_f110)
      return 0;
    error(0,0,"%s",gettext("failed to get cpuid"));
    return 0;
  }
  if (!v1) {
    if (!dat_f110)
      return 0;
    error(0,0,"%s",gettext("avx2 support not detected"));
    return 0;
  }
  if (!dat_f110)
    return 1;
  error(0,0,"%s",gettext("using avx2 hardware support"));
  return 1;
}

// Function: sub_2f7b @ 0x2f7b
void sub_2f7b(int a0)
{
  FILE *v1;
  char *v2;
  
  v2 = dat_f178;
  if (a0) // branch-flip
    fprintf(stderr,gettext("Try \'%s --help\' for more information.\n"),v2);
  else {
    printf(gettext("Usage: %s [OPTION]... [FILE]...\n  or:  %s [OPTION]... --files0-from=F\n"),v2,v2);
    v1 = stdout;
    fputs_unlocked(gettext("Print newline, word, and byte counts for each FILE, and a total line if\nmore than one FILE is specified.  A word is a non-zero-length sequence of\nprintable characters delimited by white space.\n"),v1);
    sub_2978();
    v1 = stdout;
    fputs_unlocked(gettext("\nThe options below may be used to select which counts are printed, always in\nthe following order: newline, word, character, byte, maximum line length.\n  -c, --bytes            print the byte counts\n  -m, --chars            print the character counts\n  -l, --lines            print the newline counts\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --files0-from=F    read input from the files specified by\n                           NUL-terminated names in file F;\n                           If F is - then read names from standard input\n  -L, --max-line-length  print the maximum display width\n  -w, --words            print the word counts\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --help        display this help and exit\n"),v1);
    v1 = stdout;
    fputs_unlocked(gettext("      --version     output version information and exit\n"),v1);
    sub_29a9("wc");
  }
  exit(a0); // no-return
}

// Function: sub_30be @ 0x30be
unsigned long sub_30be(int a0) // early-return
{
  if ((dat_f158 != '\x01') && ((((a0 == 0xa0 || (a0 == 0x2007)) || (a0 == 0x202f)) || (a0 == 0x2060))))
    return 1;
  return 0;
}

// Function: sub_3109 @ 0x3109
void sub_3109(int a0)
{
  sub_30be(btowc(a0));
}

// Function: sub_312b @ 0x312b
void sub_312b(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,char *a5)
{
  unsigned long v1; // rax
  char v2 [24];
  char *v3; // stack - 0x30
  
  v3 = "%*s";
  if (dat_f140) {
    v1 = sub_5729(a0,v2);
    printf("%*s",(unsigned long)dat_f148,v1);
    v3 = " %*s";
  }
  if (dat_f141) {
    v1 = sub_5729(a1,v2);
    printf(v3,(unsigned long)dat_f148,v1);
    v3 = " %*s";
  }
  if (dat_f142) {
    v1 = sub_5729(a2,v2);
    printf(v3,(unsigned long)dat_f148,v1);
    v3 = " %*s";
  }
  if (dat_f143) {
    v1 = sub_5729(a3,v2);
    printf(v3,(unsigned long)dat_f148,v1);
    v3 = " %*s";
  }
  if (dat_f144) {
    v1 = sub_5729(a4,v2);
    printf(v3,(unsigned long)dat_f148,v1);
  }
  if (a5) {
    if (strchr(a5,10))
      a5 = (char *)sub_79a7(0,3,a5);
    printf(" %s",a5);
  }
  putchar_unlocked(10);
}

// Function: sub_3328 @ 0x3328
unsigned long sub_3328(char *a0,unsigned int a1,long *a2,long *a3) // return-dupe
{
  bool v1;
  unsigned long v2;
  char *v3; // rax
  unsigned long v4; // rax
  char v5 [16392];
  long v6; // stack - 0x4058
  long v7; // stack - 0x4050
  char *v8; // stack - 0x4048
  
  v1 = 0;
  if ((a2) && (a3)) {
    v7 = 0;
    v6 = 0;
    while( true ) {
      v2 = v6;
      v4 = sub_8412(a1,v5,0x4000);
      if (!v4) {
        *a3 = v7;
        *a2 = v6;
        return 1;
      }
      if (v4 == 0xffffffffffffffff) break;
      v7 += v4;
      v8 = v5;
      v3 = &v5[v4];
      if (v1) {
        *v3 = '\n';
        while (v8 = rawmemchr(v8,10), v8 < v3) {
          v8 = &v8[1];
          v6 += 1;
        }
      }
      else {
        for (; v8 != v3; v8 = &v8[1]) {
          v6 += (int)(unsigned int)(*v8 == '\n');
        }
      }
      if ((unsigned long)(v6 - v2) <= v4 / 0xf) // branch-flip
        v1 = 1;
      else {
        v1 = 0;
      }
    }
    v3 = (char *)sub_79a7(0,3,a0);
    error(0,*__errno_location(),"%s",v3);
    return 0;
  }
  return 0;
}

// Function: sub_3591 @ 0x3591
unsigned char sub_3591(int a0,char *a1,struct_1 *a2,unsigned long a3) // ternary, warn: iteboolean: re-rolled 2 0/1 select diamond(s) to a boolean assignment in sub_3591
{
  char *v1;
  long v10; // stack - 0x40a8
  long v11; // stack - 0x40b0
  mbstate_t v12; // stack - 0x4038
  unsigned int v13; // stack - 0x40b8
  bool v14;
  unsigned long v15; // stack - 0x40e8
  char v16; // stack - 0x40c1
  char v17; // stack - 0x40c0
  unsigned char v18; // stack - 0x40bf
  unsigned char v19; // stack - 0x40bd
  char v2;
  unsigned char v20; // stack - 0x40ba
  int v21; // stack - 0x40b4
  unsigned long v22; // stack - 0x40a0
  unsigned long v23; // stack - 0x4098
  long v24; // stack - 0x4090
  unsigned long v25; // stack - 0x4088
  unsigned long v26; // stack - 0x4080
  unsigned long v27; // stack - 0x4078
  char *v28; // stack - 0x4070
  long v29; // stack - 0x4068
  long v3;
  unsigned long v30; // stack - 0x4060
  char *v31; // stack - 0x4058
  char *v32; // stack - 0x4050
  unsigned long v33; // stack - 0x4048
  long v34; // stack - 0x4040
  mbstate_t v35; // stack - 0x4030
  bool v4;
  unsigned long v5;
  unsigned char v6; // al
  int v7; // eax
  unsigned short *v8;
  char v9 [16392];
  
  v16 = 1;
  if (a1) // branch-flip
    v8 = (unsigned short *)a1;
  else {
    v8 = (char *)gettext("standard input");
  }
  v25 = 0;
  v10 = 0;
  v24 = 0;
  v23 = 0;
  v11 = 0;
  v32 = (char *)v8;
  v8 = (unsigned long)__ctype_get_mb_cur_max();
  if (2 <= v8) { // branch-flip
    v6 = dat_f143;
    v17 = v6;
    v6 = dat_f142;
    v18 = v6;
  }
  else {
    v6 = ((dat_f143) || (dat_f142));
    v18 = 0;
    v17 = v6;
  }
  v6 = ((dat_f141) || (dat_f144));
  if ((((v17 != '\x01') || (v18)) || (dat_f140)) || ((bool)v6))
    sub_54b7(a0,0,0,2);
  if (((!v17) || (v18 == 1)) || ((dat_f140 == '\x01' || ((bool)v6)))) {
    if ((v18 == 1) || ((bool)v6)) {
      v8 = (unsigned long)__ctype_get_mb_cur_max();
      if (2 <= v8) { // branch-flip
        v19 = 0;
        v26 = 0;
        v12 = 0;
        v14 = 0;
        v27 = 0;
        while( true ) {
          v8 = (unsigned long)(0x4000 - v27);
          v8 = (unsigned long)sub_8412(a0,&v9[v27],v8);
          v5 = (unsigned long)v8;
          v22 = (unsigned long)v8;
          if (!v8) break;
          if (v8 == (unsigned short *)0xffffffffffffffff) {
            v8 = (char *)sub_79a7(0,3,v32);
            v1 = (char *)v8;
            v8 = (int *)__errno_location();
            error(0,*(int *)v8,"%s",v1);
            v16 = 0;
            break;
          }
          v8 = (long)((long)v8 + v10);
          v28 = v9;
          v22 = v5 + v27;
          v10 = (long)v8;
          do {
            v4 = 1;
            if ((v14) || (v7 = (int)*v28, v6 = sub_5893(v7), !v6)) {
              v14 = 1;
              v35 = v12;
              v8 = (long)sub_58d8(&v13,v28,v22,&v12);
              v29 = (long)v8;
              if (v8 != (unsigned short *)0xfffffffffffffffe) {
                if (v8 != (unsigned short *)0xffffffffffffffff) {
                  v7 = mbsinit(&v12);
                  v14 = v7 == 0;
                  if (!v29) {
                    v13 = 0;
                    v29 = 1;
                  }
                  goto label_3c3e;
                }
                v28 = &v28[1];
                v22 -= 1;
                goto label_3e20;
              }
              v12 = v35;
              break;
            }
            v29 = 1;
            v7 = (unsigned int)*v28;
            v4 = 0;
            v13 = v7;
label_3c3e:
            switch(v13) {
              case 9:
                v8 = (unsigned long)(v26 & 0xfffffffffffffff8);
                v8 = (unsigned long)((long)v8 + 8);
                v26 = (unsigned long)v8;
                break;
              case 10:
                v8 = (long)(v11 + 1);
                v11 = (long)v8;
                goto label_3c85;
              case 0xb:
                break;
              case 0xc:
              case 0xd:
label_3c85:
                if (v25 < v26)
                  v25 = v26;
                v26 = 0;
                break;
              default:
                if ((v4) && (v7 = iswprint(v13), v7)) {
                  if (dat_f144) {
                    v7 = wcwidth(v13);
                    v21 = v7;
                    if (1 <= v7) {
                      v8 = (long)(long)v7;
                      v26 += (long)v8;
                    }
                  }
                  v7 = iswspace(v13);
                  if ((v7) || (v7 = sub_30be(v13), v7)) {
                    goto label_3cd6;
                  }
                  v19 = 1;
                }
                else if (!v4) {
                  v8 = __ctype_b_loc();
                  v3 = *(long *)v8;
                  v7 = (int)*v28;
                  v6 = sub_2969(v7);
                  v8 = (unsigned long)(unsigned long)v6;
                  v8 = (unsigned short *)((long)v8 * 2 + v3);
                  if (*v8 & 0x4000) {
                    v26 += 1;
                    v8 = __ctype_b_loc();
                    v3 = *(long *)v8;
                    v7 = (int)*v28;
                    v6 = sub_2969(v7);
                    v8 = (unsigned long)(unsigned long)v6;
                    v8 = (unsigned short *)((long)v8 * 2 + v3);
                    if (*v8 & 0x2000) goto label_3cd6;
                    v19 = 1;
                  }
                }
                goto label_3dfc;
              case 0x20:
                v26 += 1;
              
            }
label_3cd6:
            v8 = (unsigned long)(unsigned long)v19;
            v23 += (long)v8;
            v19 = 0;
label_3dfc:
            v28 = &v28[v29];
            v22 -= v29;
            v24 += 1;
label_3e20:
          } while (v22);
          if (v22) {
            if (v22 == 0x4000) {
              v28 = &v28[1];
              v22 = 0x3fff;
            }
            memmove(v9,v28,v22);
          }
          v27 = v22;
        }
        if (v25 < v26)
          v25 = v26;
        v8 = (unsigned long)(unsigned long)v19;
        v23 += (long)v8;
      }
      else {
        v20 = 0;
        v30 = 0;
        while( true ) {
          v8 = (unsigned long)sub_8412(a0,v9,0x4000);
          v22 = (unsigned long)v8;
          if (!v8) break;
          v31 = v9;
          if (v8 == (unsigned short *)0xffffffffffffffff) {
            v8 = (char *)sub_79a7(0,3,v32);
            v1 = (char *)v8;
            v8 = (int *)__errno_location();
            error(0,*(int *)v8,"%s",v1);
            v16 = 0;
            break;
          }
          v8 = (long)((long)v8 + v10);
          v10 = (long)v8;
          do {
            v1 = &v31[1];
            v2 = *v31;
            v31 = v1;
            switch(v2) {
              case 9:
                v8 = (unsigned long)(v30 & 0xfffffffffffffff8);
                v8 = (unsigned long)((long)v8 + 8);
                v30 = (unsigned long)v8;
                break;
              case 10:
                v8 = (long)(v11 + 1);
                v11 = (long)v8;
                goto label_3fe3;
              case 0xb:
                break;
              case 0xc:
              case 0xd:
label_3fe3:
                if (v25 < v30)
                  v25 = v30;
                v30 = 0;
                break;
              default:
                v8 = __ctype_b_loc();
                v3 = *(long *)v8;
                v8 = (char *)&v31[-1];
                v7 = (int)*(char *)v8;
                v6 = sub_2969(v7);
                v8 = (unsigned long)(unsigned long)v6;
                v8 = (unsigned short *)((long)v8 * 2 + v3);
                if (*v8 & 0x4000) {
                  v30 += 1;
                  v8 = __ctype_b_loc();
                  v3 = *(long *)v8;
                  v8 = (char *)&v31[-1];
                  v7 = (int)*(char *)v8;
                  v6 = sub_2969(v7);
                  v8 = (unsigned long)(unsigned long)v6;
                  v8 = (unsigned short *)((long)v8 * 2 + v3);
                  if (!(*v8 & 0x2000)) {
                    v8 = (char *)&v31[-1];
                    v7 = (int)*(char *)v8;
                    v6 = sub_2969(v7);
                    v7 = sub_3109(v6);
                    if (!v7) {
                      v20 = 1;
                      goto label_40f9;
                    }
                  }
                  v8 = (unsigned long)(unsigned long)v20; // crossjump-dupe
                  v23 += (long)v8;
                  v20 = 0;
                }
                goto label_40f9;
              case 0x20:
                v30 += 1;
              
            }
label_4031:
            v8 = (unsigned long)(unsigned long)v20;
            v23 += (long)v8;
            v20 = 0;
label_40f9:
            v22 -= 1;
          } while (v22);
        }
        if (v25 < v30)
          v25 = v30;
        v8 = (unsigned long)(unsigned long)v20;
        v23 += (long)v8;
      }
    }
    else {
      v6 = sub_2daf();
      if (v6)
        dat_f010 = sub_a1b1;
      v6 = (*dat_f010)(v32,a0,&v11,&v10);
      v16 = v6;
    }
  }
  else {
    v14 = 0;
    if (1 <= a2->field_0x0) {
      v7 = fstat(a0,(stat *)&a2->field_0x4[4]);
      a2->field_0x0 = v7;
    }
    if (((!a2->field_0x0) && (v8 = (char *)&a2->field_0x4[4], v6 = sub_2c40(v8), v6)) && (0 <= (long)a2->field_0x38)) {
      v33 = a2->field_0x38;
      v15 = a3;
      if ((long)a3 <= -1) {
        v8 = (unsigned long)lseek(a0,0,1);
        v15 = (unsigned long)v8;
      }
      if (v33 % dat_f150) { // branch-flip
        v8 = (v15 <= v33) ? (long)(v33 - v15) : NULL; // branch-flip
        v14 = 1;
        v10 = (long)v8;
      }
      else {
        if ((1 <= (long)a2->field_0x40) && ((unsigned long)a2->field_0x40 <= 0x2000000000000000))
          v8 = (unsigned long)(a2->field_0x40 + 1);
        else {
          v8 = (unsigned short *)0x201;
        }
        v8 = (long)(v33 - v33 % (unsigned long)v8);
        v34 = (long)v8;
        if (((0 <= (long)v15) && ((long)v15 < (long)v8)) && (v8 = (long)lseek(a0,v8,1), 0 <= (long)v8)) {
          v8 = (long)(v34 - v15);
          v10 = (long)v8;
        }
      }
    }
    if (!v14) {
      sub_54b7(a0,0,0,2);
      while( true ) {
        v8 = (unsigned long)sub_8412(a0,v9,0x4000);
        v22 = (unsigned long)v8;
        if (!v8) break;
        if (v8 == (unsigned short *)0xffffffffffffffff) {
          v8 = (char *)sub_79a7(0,3,v32);
          v1 = (char *)v8;
          v8 = (int *)__errno_location();
          error(0,*(int *)v8,"%s",v1);
          v16 = 0;
          break;
        }
        v8 = (long)((long)v8 + v10);
        v10 = (long)v8;
      }
    }
  }
  if (v18 < dat_f142)
    v24 = v10;
  sub_312b(v11,v23,v24,v10,v25,a1);
  v8 = (long)(v11 + dat_f118);
  dat_f118 = (long)v8;
  v8 = (long)(v23 + dat_f120);
  dat_f120 = (long)v8;
  v8 = (long)(v24 + dat_f128);
  dat_f128 = (long)v8;
  v8 = (long)(v10 + dat_f130);
  dat_f130 = (long)v8;
  if (dat_f138 < v25)
    dat_f138 = v25;
  v6 = v16;
  return v6;
}

// Function: sub_4263 @ 0x4263
unsigned long sub_4263(char *a0,struct_1 *a1)
{
  unsigned char v1; // al
  int v2; // eax
  unsigned long v3; // rax
  char *v4; // rax
  
  if ((!a0) || (!strcmp(a0,"-"))) {
    dat_f14c = 1;
    sub_945e(0,0);
    v3 = sub_3591(0,a0,a1,0xffffffffffffffff);
    return v3;
  }
  v2 = open(a0,0);
  if (v2 == -1) {
    v4 = (char *)sub_79a7(0,3,a0);
    error(0,*__errno_location(),"%s",v4);
    return 0;
  }
  v1 = sub_3591(v2,a0,a1,0);
  if (close(v2)) {
    v4 = (char *)sub_79a7(0,3,a0);
    error(0,*__errno_location(),"%s",v4);
    return 0;
  }
  return (unsigned long)v1;
}

// Function: sub_43a5 @ 0x43a5
unsigned int * sub_43a5(unsigned long a0,long a1)
{
  char *v1;
  int v2; // eax
  unsigned long v3;
  unsigned int *v4; // rax
  unsigned long v5; // stack - 0x18
  
  if (a0) // branch-flip
    v3 = a0;
  else {
    v3 = 1;
  }
  v4 = (unsigned int *)sub_8f54(v3,0x98);
  if ((a0) && ((a0 != 1 || ((unsigned int)dat_f144 + (unsigned int)dat_f140 + (unsigned int)dat_f141 + (unsigned int)dat_f142 + (unsigned int)dat_f143 != 1)))) {
    for (v5 = 0; v5 < a0; v5 = v5 + 1) {
      if (*(long *)(a1 + v5 * 8)) { // branch-flip
        v1 = *(char **)(a1 + v5 * 8);
        if (!strcmp(v1,"-")) goto label_448c;
        v2 = stat(*(char **)(a1 + v5 * 8),(stat *)&v4[v5 * 0x26 + 2]);
      }
      else {
label_448c:
        v2 = fstat(0,(stat *)&v4[v5 * 0x26 + 2]);
      }
      v4[v5 * 0x26] = v2;
    }
  }
  else {
    *v4 = 1;
  }
  return v4;
}

// Function: sub_4549 @ 0x4549
int sub_4549(unsigned long a0,int *a1)
{
  int v1; // stack - 0x20
  int v2; // stack - 0x1c
  unsigned long v3; // stack - 0x18
  unsigned long v4; // stack - 0x10
  
  v1 = 1;
  if ((a0) && (*a1 <= 0)) {
    v2 = 1;
    v3 = 0;
    for (v4 = 0; v4 < a0; v4 = v4 + 1) {
      if (!a1[v4 * 0x26]) {
        if ((a1[v4 * 0x26 + 8] & 0xf000U) != 0x8000) // branch-flip
          v2 = 7;
        else {
          v3 += *(long *)&a1[v4 * 0x26 + 0xe];
        }
      }
    }
    for (; 10 <= v3; v3 = v3 / 10) {
      v1 += 1;
    }
    if (v1 < v2)
      v1 = v2;
  }
  return v1;
}

// Function: main @ 0x4670
int main(int argc,char **argv,char **envp)
{
  bool v1;
  unsigned long v10; // rax
  char *v11; // rax
  long v12; // rax
  stat v13; // stack - 0xb8
  long v14; // stack - 0x1d8
  unsigned int v15; // stack - 0x224
  long v16; // fs_offset
  double v17; // xmm0_qa
  double v18; // xmm0_qa
  double v19;
  bool v2;
  bool v20; // stack - 0x227
  int v21; // stack - 0x220
  int v22; // stack - 0x21c
  long v23; // stack - 0x218
  char **v24; // stack - 0x210
  char *v25; // stack - 0x208
  struct_4 *v26; // stack - 0x200
  FILE *v27; // stack - 0x1f8
  int *v28; // stack - 0x1f0
  char *v29; // stack - 0x1e8
  char v3; // al
  unsigned long v30; // stack - 0x1e0
  char **v31; // stack - 0x1d0
  long v32; // stack - 0x20
  unsigned char v4; // al
  int v5; // eax
  int v6; // eax
  int v7; // eax
  int v8; // eax
  char *v9; // rax
  
  v32 = *(long *)(v16 + 0x28);
  v25 = NULL;
  sub_5c1b(*argv);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  sub_ac00(sub_53bf);
  dat_f150 = (long)getpagesize();
  setvbuf(stdout,NULL,1,0);
  dat_f158 = getenv("POSIXLY_CORRECT") != NULL;
  dat_f143 = '\0';
  dat_f142 = '\0';
  dat_f141 = '\0';
  dat_f140 = '\0';
  dat_f144 = '\0';
  dat_f138 = 0;
  dat_f130 = 0;
  dat_f128 = 0;
  dat_f120 = 0;
  dat_f118 = 0;
  while( true ) {
    v22 = getopt_long(argc,argv,"clLmw",(option *)0xea20,NULL);
    if (v22 == -1) break;
    if (0x81 < v22) {
label_48e7:
      sub_2f7b(1); // no-return, return-dupe
    }
    if (v22 <= 0x4b) {
      if (v22 == -0x83) {
        sub_8bfb(stdout,"wc","GNU coreutils",dat_f018,"Paul Rubin","David MacKenzie",0);
        exit(0); // no-return
      }
      if (v22 == -0x82)
        sub_2f7b(0); // no-return
      sub_2f7b(1);
    }
    switch(v22) {
      case 0x4c:
        dat_f144 = '\x01';
        break;
      default:
        goto label_48e7;
      case 99:
        dat_f143 = '\x01';
        break;
      case 0x6c:
        dat_f140 = '\x01';
        break;
      case 0x6d:
        dat_f142 = '\x01';
        break;
      case 0x77:
        dat_f141 = '\x01';
        break;
      case 0x80:
        dat_f110 = 1;
        break;
      case 0x81:
        v25 = optarg;
      
    }
  }
  if ((((dat_f140 != '\x01') && (dat_f141 != '\x01')) && (dat_f142 != '\x01')) && ((dat_f143 != '\x01' && (dat_f144 != '\x01')))) {
    dat_f143 = '\x01';
    dat_f141 = '\x01';
    dat_f140 = '\x01';
  }
  v2 = 0;
  if (v25) { // branch-flip
    if (optind < argc) {
      v9 = (char *)sub_7815(4,argv[optind]);
      error(0,0,gettext("extra operand %s"),v9);
      fprintf(stderr,"%s\n",gettext("file operands cannot be combined with --files0-from"));
      sub_2f7b(1); // no-return
    }
    if (strcmp(v25,"-")) { // branch-flip
      v27 = fopen(v25,"r");
      if (!v27) {
        v10 = sub_7815(4,v25);
        v9 = gettext("cannot open %s for reading");
        error(1,*__errno_location(),v9,v10);
        return v5;
      }
    }
    else {
      v27 = stdin;
    }
    v6 = fileno(v27);
    if ((!fstat(v6,&v13)) && ((v13._24_4_ & 0xf000) == 0x8000)) {
      sub_5aa9();
      if (v17 / dat_b988 <= dat_b990) {
        sub_5aa9();
        v19 = v18 / dat_b988;
      }
      else {
        v19 = dat_b990;
      }
      if ((double)(long)v13._48_8_ <= v19) {
        v2 = 1;
        sub_7c7f(&v14);
        v3 = sub_809e(v27,&v14);
        if ((v3 != '\x01') || (v6 = sub_552b(v27), v6)) {
          v9 = (char *)sub_7815(4,v25);
          error(1,0,gettext("cannot read file names from %s"),v9);
          return v7;
        }
        v24 = v31;
        v23 = v14;
        v26 = (struct_4 *)sub_5165(v31);
        goto label_4d00;
      }
    }
    v24 = NULL;
    v23 = 0;
    v26 = (struct_4 *)sub_51ba(v27);
  }
  else {
    if (argc <= optind) { // branch-flip
      v24 = (char **)0xf160;
      v23 = 1;
    }
    else {
      v24 = &argv[optind];
      v23 = (long)(argc - optind);
    }
    v26 = (struct_4 *)sub_5165(v24);
  }
label_4d00:
  if (!v26)
    sub_9413(); // no-return
  v28 = (int *)sub_43a5(v23,v24);
  dat_f148 = sub_4549(v23,v28);
  v20 = 1;
  v21 = 0;
  while( true ) {
    v1 = 0;
    v29 = (char *)sub_5227(v26,&v15);
    if (!v29) break;
    if (((v25) && (!strcmp(v25,"-"))) && (!strcmp(v29,"-"))) {
      v9 = (char *)sub_7815(4,v29);
      error(0,0,gettext("when reading file names from stdin, no file name of %s allowed"),v9);
      v1 = 1;
    }
    if (!*v29) {
      if (v25) { // branch-flip
        v30 = sub_5312(v26);
        v9 = gettext("invalid zero-length file name");
        v11 = (char *)sub_79a7(0,3,v25);
        error(0,0,"%s:%lu: %s",v11,v30,v9);
      }
      else {
        error(0,0,"%s",gettext("invalid zero-length file name"));
      }
      v1 = 1;
    }
    if (v1)
      v20 = 0;
    else {
      if (v23) // branch-flip
        v12 = (long)v21 * 0x98;
      else {
        v12 = 0;
      }
      v4 = sub_4263(v29,(struct_1 *)(v12 + (long)v28));
      v20 = (v4 & v20) != 0;
    }
    if (!v23)
      *v28 = 1;
    v21 += 1;
  }
  if (v15 == 4) {
    v10 = sub_79a7(0,3,v25);
    v9 = gettext("%s: read error");
    error(0,*__errno_location(),v9,v10);
    v20 = 0;
label_5026:
    if (((v20) && (!v25)) && (!sub_5312(v26))) {
      v4 = sub_4263(NULL,v28);
      v20 = (v4 & v20) != 0;
    }
    if (v2)
      sub_7d41(&v14);
    if (2 <= (unsigned long)sub_5312(v26))
      sub_312b(dat_f118,dat_f120,dat_f128,dat_f130,dat_f138,gettext("total"));
    sub_5350(v26);
    free(v28);
    if ((dat_f14c) && (close(0))) {
      error(1,*__errno_location(),"-");
      return v8;
    }
    if (v32 == *(long *)(v16 + 0x28))
      return (int)(v20 ^ 1);
    __stack_chk_fail(); // no-return
  }
  if (v15 <= 4) {
    if (v15 == 2) goto label_5026;
    if (v15 == 3)
      sub_9413(); // no-return
  }
  __assert_fail("!\"unexpected error code from argv_iter\"","src/wc.c",0x3aa,"main"); // no-return
}

// Function: sub_5165 @ 0x5165
struct_11 * sub_5165(char **a0) // early-return
{
  struct_11 *v1; // rax
  
  v1 = malloc(0x30);
  if (!v1)
    return NULL;
  v1->field_0x0 = 0;
  v1->field_0x20 = a0;
  v1->field_0x28 = a0;
  return v1;
}

// Function: sub_51ba @ 0x51ba
unsigned long * sub_51ba(unsigned long a0) // early-return
{
  unsigned long *v1; // rax
  
  v1 = malloc(0x30);
  if (!v1)
    return NULL;
  *v1 = a0;
  v1[2] = 0;
  v1[3] = 0;
  v1[1] = 0;
  v1[4] = 0;
  return v1;
}

// Function: sub_5227 @ 0x5227
long sub_5227(struct_3 *a0,unsigned int *a1) // early-return x2, ternary
{
  FILE *v1;
  long *v2;
  long v3; // rax
  unsigned int v4; // edx
  
  if (a0->field_0x0) { // branch-flip
    v3 = getdelim((char **)&a0->field_0x10,a0->field_0x18,0,(FILE *)a0->field_0x0);
    if (v3 <= -1) {
      v1 = (FILE *)a0->field_0x0;
      v4 = (feof(v1)) ? 2 : 4; // branch-flip
      *a1 = v4;
      v3 = 0;
      return v3;
    }
    *a1 = 1;
    a0->field_0x8 = a0->field_0x8 + 1;
    v3 = a0->field_0x10;
  }
  else {
    if (!*a0->field_0x28) {
      *a1 = 2;
      v3 = 0;
      return v3;
    }
    *a1 = 1;
    v2 = a0->field_0x28;
    a0->field_0x28 = &v2[1];
    v3 = *v2;
  }
  return v3;
}

// Function: sub_5312 @ 0x5312
long sub_5312(struct_4 *a0)
{
  long v1;
  
  if (a0->field_0x0) // branch-flip
    v1 = a0->field_0x8;
  else {
    v1 = a0->field_0x28 - a0->field_0x20 >> 3;
  }
  return v1;
}

// Function: sub_5350 @ 0x5350
void sub_5350(struct_5 *a0)
{
  if (a0->field_0x0)
    free(a0->field_0x10);
  free(a0);
}

// Function: sub_538b @ 0x538b
void sub_538b(unsigned long a0)
{
  dat_f168 = a0;
}

// Function: sub_53a5 @ 0x53a5
void sub_53a5(char a0)
{
  dat_f170 = a0;
}

// Function: sub_53bf @ 0x53bf
void sub_53bf(void)
{
  int v1; // eax
  char *v2; // rax
  char *v3; // rax
  
  v1 = sub_953d(stdout);
  if (v1) {
    if (dat_f170 == '\x01') {
      if (*__errno_location() == 0x20) goto label_5490;
    }
    v2 = gettext("write error");
    if (dat_f168) { // branch-flip
      v3 = (char *)sub_7956(dat_f168);
      error(0,*__errno_location(),"%s: %s",v3,v2);
    }
    else {
      error(0,*__errno_location(),"%s",v2);
    }
    _exit(dat_f020); // no-return
  }
label_5490:
  v1 = sub_953d(stderr);
  if (v1)
    _exit(dat_f020); // no-return
}

// Function: sub_54b7 @ 0x54b7
void sub_54b7(unsigned int a0,unsigned long a1,unsigned long a2,unsigned int a3)
{
  posix_fadvise(a0,a1,a2,a3);
}

// Function: sub_54ec @ 0x54ec
void sub_54ec(FILE *a0,unsigned int a1) // return-dupe
{
  if (!a0)
    return;
  sub_54b7(fileno(a0),0,0,a1);
}

// Function: sub_552b @ 0x552b
unsigned long sub_552b(FILE *a0)
{
  int v1; // eax
  int v2; // stack - 0x14
  unsigned int v3; // stack - 0x10
  
  v2 = 0;
  if (fileno(a0) <= -1)
    return CONCAT44(dat_4,fclose(a0));
  if (__freading(a0)) {
    v1 = fileno(a0);
    if (lseek(v1,0,1) == -1) goto label_55b9;
  }
  if (sub_561c(a0))
    v2 = *__errno_location();
label_55b9:
  v3 = fclose(a0);
  if (v2) {
    *__errno_location() = v2;
    v3 = 0xffffffff;
  }
  return (unsigned long)v3;
}

// Function: sub_55e4 @ 0x55e4
void sub_55e4(FILE *a0) // return-dupe
{
  if (!(*(unsigned int *)a0 & 0x100))
    return;
  sub_566b(a0,0,1);
}

// Function: sub_561c @ 0x561c
void sub_561c(FILE *a0)
{
  if ((a0) && (__freading(a0))) {
    sub_55e4(a0);
    fflush(a0);
    return;
  }
  fflush(a0);
}

// Function: sub_566b @ 0x566b
unsigned long sub_566b(FILE *a0,long a1,int a2) // early-return x2
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

// Function: sub_5729 @ 0x5729
char * sub_5729(unsigned long a0,long a1)
{
  unsigned long v1; // stack - 0x20
  char *v2; // stack - 0x10
  
  v2 = (char *)(a1 + 0x14);
  *v2 = '\0';
  v1 = a0;
  do {
    v2 = &v2[-1];
    *v2 = (char)v1 + ((char)(v1 / 10 << 2) + (char)(v1 / 10)) * '\xfe' + '0';
    v1 /= 10;
  } while (v1);
  return v2;
}

// Function: sub_57b2 @ 0x57b2
unsigned int sub_57b2(unsigned int a0)
{
  int v1; // eax
  
  v1 = wcwidth(a0);
  if (v1 <= -1) {
    v1 = iswcntrl(a0);
    v1 = (unsigned int)(v1 == 0);
  }
  return v1;
}

// Function: sub_57ed @ 0x57ed
void sub_57ed(struct_9 *a0,struct_10 *a1) // return-dupe
{
  if ((struct_10 *)a1->field_0x0 != &a1[1]) // branch-flip
    a0->field_0x0 = a1->field_0x0;
  else {
    memcpy(&a0[1],&a1[1],a1->field_0x8);
    a0->field_0x0 = (long *)&a0[1];
  }
  a0->field_0x8 = a1->field_0x8;
  a0->field_0x10 = a1->field_0x10;
  if (!a0->field_0x10)
    return;
  a0->field_0x14 = a1->field_0x14;
}

// Function: sub_5893 @ 0x5893
bool sub_5893(unsigned char a0)
{
  return (*(unsigned int *)((long)(int)(unsigned int)(a0 >> 5) * 4 + 0xb9c0) >> (a0 & 0x1f) & 1) != 0;
}

// Function: sub_58d8 @ 0x58d8
unsigned long sub_58d8(unsigned int *a0,char *a1,unsigned long a2,mbstate_t *a3) // early-return
{
  long v1;
  unsigned int *v2; // stack - 0x30
  unsigned long v3; // stack - 0x18
  
  v2 = a0;
  if (!a0)
    v2 = &v1;
  v3 = mbrtowc(v2,a1,a2,a3);
  if (((0xfffffffffffffffe <= v3) && (a2)) && (sub_95cc(0) != '\x01')) {
    *v2 = (unsigned int)(unsigned char)*a1;
    return 1;
  }
  return v3;
}

// Function: sub_5982 @ 0x5982
double sub_5982(void)
{
  char v1 [32];
  double v2;
  double v3; // xmm0_qa
  unsigned long v4; // stack - 0x68
  unsigned int v5; // stack - 0x20
  
  v2 = (double)sysconf(0x55);
  v3 = (double)sysconf(0x1e);
  if ((v2 < 0.0) || (v3 < 0.0)) {
    if (sysinfo(v1)) // branch-flip
      v2 = dat_b9e0;
    else {
      v2 = (double)v5 * (double)v4;
    }
  }
  else {
    v2 = v2 * v3;
  }
  return v2;
}

// Function: sub_5aa9 @ 0x5aa9
double sub_5aa9(void)
{
  char v1 [40];
  double v2; // xmm0_qa
  double v3; // xmm0_qa
  double v4; // xmm0_qa
  unsigned long v5; // stack - 0x60
  unsigned long v6; // stack - 0x50
  unsigned int v7; // stack - 0x20
  
  v2 = (double)sysconf(0x56);
  v3 = (double)sysconf(0x1e);
  if ((v2 < 0.0) || (v3 < 0.0)) {
    if (sysinfo(v1)) { // branch-flip
      sub_5982();
      v2 = v4 / dat_b9e8;
    }
    else {
      v2 = (double)v7 * ((double)v5 + (double)v6);
    }
  }
  else {
    v2 = v2 * v3;
  }
  return v2;
}

// Function: sub_5c1b @ 0x5c1b
void sub_5c1b(char *a0)
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
  dat_f178 = v2;
  program_invocation_name = v2;
}

// Function: sub_5d06 @ 0x5d06
unsigned long sub_5d06(void *a0)
{
  int v1;
  unsigned long v2; // rax
  
  v1 = *__errno_location();
  if (!a0)
    a0 = (void *)0xf180;
  v2 = sub_931b(a0,0x38);
  *__errno_location() = v1;
  return v2;
}

// Function: sub_5d55 @ 0x5d55
unsigned int sub_5d55(unsigned int *a0)
{
  if (!a0)
    a0 = (unsigned int *)0xf180;
  return *a0;
}

// Function: sub_5d79 @ 0x5d79
void sub_5d79(unsigned int *a0,unsigned int a1)
{
  if (!a0)
    a0 = (unsigned int *)0xf180;
  *a0 = a1;
}

// Function: sub_5da4 @ 0x5da4
unsigned int sub_5da4(long a0,unsigned char a1,unsigned int a2)
{
  unsigned int v1; // eax
  unsigned int *v2; // rax
  
  if (!a0)
    a0 = 0xf180;
  v2 = (unsigned int *)((unsigned long)(a1 >> 5) * 4 + a0 + 8);
  v1 = *v2 >> (a1 & 0x1f) & 1;
  *v2 = *v2 ^ (a2 & 1 ^ v1) << (a1 & 0x1f);
  return v1;
}

// Function: sub_5e34 @ 0x5e34
unsigned int sub_5e34(long a0,unsigned int a1)
{
  unsigned int v1;
  long v2; // stack - 0x20
  
  v2 = a0;
  if (!a0)
    v2 = 0xf180;
  v1 = *(unsigned int *)(v2 + 4);
  *(unsigned int *)(v2 + 4) = a1;
  return v1;
}

// Function: sub_5e6e @ 0x5e6e
void sub_5e6e(unsigned int *a0,long a1,long a2)
{
  unsigned int *v1; // stack - 0x10
  
  v1 = a0;
  if (!a0)
    v1 = (unsigned int *)0xf180;
  *v1 = 10;
  if ((a1) && (a2)) {
    *(long *)&v1[10] = a1;
    *(long *)&v1[0xc] = a2;
    return;
  }
  abort(); // no-return
}

// Function: sub_5ed0 @ 0x5ed0
unsigned long * sub_5ed0(unsigned long *a0,unsigned int a1)
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

// Function: sub_5f95 @ 0x5f95
char * sub_5f95(char *a0,int a1) // early-return x6
{
  char *v1; // rax
  char *v2; // rax
  
  v1 = gettext(a0);
  if (v1 != a0)
    return v1;
  v2 = (char *)sub_975d();
  if (!sub_94c3(v2,"UTF-8")) {
    if (*a0 == '`')
      return "‘";
    return "’";
  }
  if (sub_94c3(v2,"GB18030")) {
    if (a1 == 9)
      return "\"";
    return "\'";
  }
  if (*a0 == '`')
    return (char *)0xbade;
  return (char *)0xbae2;
}

// Function: sub_605a @ 0x605a
unsigned long sub_605a(long a0,unsigned long a1,char *a2,unsigned long a3,int a4,unsigned int a5,long a6,char *a7,char *a8)
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
label_6114:
  switch(v19) {
    case 0:
      v14 = 0;
      break;
    case 1:
label_6268:
      v14 = 1;
      goto label_626c;
    case 2:
label_627b:
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
      goto label_6268;
    case 4:
label_626c:
      if (!v14)
        v2 = 1;
      goto label_627b;
    case 5:
label_6156:
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
      goto label_6156;
    case 7:
      v2 = 1;
      v14 = 0;
      break;
    case 8:
    case 9:
    case 10:
      if (v19 != 10) {
        v18 = (char *)sub_5f95("`",v19);
        v17 = (char *)sub_5f95("\'",v19);
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
label_6f1a:
  if (v20 != 0xffffffffffffffff) // branch-flip
    v16 = v24 != v20;
  else {
    v16 = a2[v24] != '\0';
  }
  if (!v16) goto label_6f52;
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
      if (v14) goto label_7096;
      v4 = 1;
    }
  }
  v22 = a2[v24];
  if (v22 == 0x7e) {
label_6839:
    if (!v24) {
label_6844:
      v16 = 1;
label_6848:
      if (v19 != 2) goto label_6d5f;
label_6621:
      if (v14) goto label_7096;
    }
    goto label_6d5f;
  }
  if (0x7e < v22) {
label_6925:
    if (v9 != 1) {
      memset(&v12,0,8);
      v29 = 0;
      v16 = 1;
      if (v20 == 0xffffffffffffffff)
        v20 = strlen(a2);
label_699f:
      v31 = sub_58d8(&v13,&a2[v24 + v29],v20 - (v24 + v29),&v12);
      if (v31) {
        if (v31 != 0xffffffffffffffff) { // branch-flip
          if (v31 != 0xfffffffffffffffe) goto label_6a4d;
          v16 = 0;
          for (; (v29 + v24 < v20 && (a2[v24 + v29])); v29 = v29 + 1) {
          }
        }
        else {
          v16 = 0;
        }
      }
      goto label_6afc;
    }
    v29 = 1;
    v16 = (*(unsigned short *)((unsigned long)v22 * 2 + *(long *)__ctype_b_loc()) & 0x4000) != 0;
label_6afc:
    if ((v29 <= 1) && ((!v2 || (v16)))) goto label_6d5f;
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
        if (v14) goto label_7096;
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
    goto label_6e85;
  }
  if (v22 == 0x7d) {
label_6807:
    if (v20 != 0xffffffffffffffff) // branch-flip
      v15 = v20 != 1;
    else {
      v15 = a2[1] != '\0';
    }
    if (v15) goto label_6d5f;
    goto label_6839;
  }
  if (0x7d < v22) goto label_6925;
  if (v22 == 0x7c) goto label_6848;
  if (0x7c < v22) goto label_6925;
  if (v22 == 0x7b) goto label_6807;
  if (0x7b < v22) goto label_6925;
  if (0x3f < v22) {
    if ((0x7a < v22) || (v22 <= 0x40)) goto label_6925;
    v11 = 1L << (v22 + 0xbf & 0x3f);
    if (v11 & 0x3ffffff53ffffff) goto label_691c;
    if (v11 & 0xa4000000) goto label_6848;
    if (!(v11 & 0x8000000)) goto label_6925;
    if (v19 == 2) {
      if (!v14) goto label_6e85;
      goto label_7096;
    }
    v23 = v22;
    if (((!v2) || (!v14)) || (!v28)) goto label_67dd;
    goto label_6e85;
  }
  switch(v22) {
    case 0:
      if (!v2) {
        if (!(a5 & 1)) goto label_6d5f;
        goto label_6f15;
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
        goto label_6d5f;
      }
      goto label_7096;
    default:
      goto label_6925;
    case 7:
      v23 = 0x61;
      break;
    case 8:
      v23 = 0x62;
      break;
    case 9:
      v23 = 0x74;
      goto label_67dd;
    case 10:
      v23 = 0x6e;
      goto label_67dd;
    case 0xb:
      v23 = 0x76;
      break;
    case 0xc:
      v23 = 0x66;
      break;
    case 0xd:
      v23 = 0x72;
label_67dd:
      if ((v19 == 2) && (v14)) goto label_7096;
      break;
    case 0x20:
      goto label_6844;
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
      goto label_6848;
    case 0x23:
      goto label_6839;
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
label_691c:
      v16 = 1;
      goto label_6d5f;
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
          goto label_6d5f;
        }
        goto label_7096;
      }
      goto label_6d5f;
    case 0x3f:
      if (v19 == 2) goto label_6621;
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
          goto label_6d5f;
        }
        goto label_7096;
      }
      goto label_6d5f;
    
  }
  if (v2) {
    v22 = v23;
label_6dca:
    if (v14) goto label_7096;
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
label_6d5f:
    if (((((v2) && (v19 != 2)) || (v14)) && ((v8 && (*(unsigned int *)(v8 + (unsigned long)(v22 >> 5) * 4) >> (v22 & 0x1f) & 1)))) || (v4)) goto label_6dca;
  }
label_6e85:
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
label_6f15:
  v24 += 1;
  goto label_6f1a;
label_6a4d:
  if ((v14) && (v19 == 2)) {
    for (v30 = 1; v30 < v31; v30 = v30 + 1) {
      if (((unsigned int)((int)a2[v24 + v29 + v30] - 0x5bU) <= 0x21) && (0x20000002bU >> ((unsigned char)((int)a2[v24 + v29 + v30] - 0x5bU) & 0x3f) & 1)) goto label_7096;
    }
  }
  if (!iswprint(v13))
    v16 = 0;
  v29 += v31;
  if (mbsinit(&v12)) goto label_6afc;
  goto label_699f;
label_6f52:
  if (((!v25) && (v19 == 2)) && (v14)) {
label_7096:
    if ((v19 == 2) && (v2))
      v19 = 4;
    return sub_605a(a0,v21,a2,v20,v19,a5 & 0xfffffffd,0,v18,v17);
  }
  if (((v19 == 2) && (!v14)) && (v6)) {
    if (v7)
      return sub_605a(a0,v26,a2,v20,5,a5,v8,v18,v17);
    if ((!v21) && (v26)) {
      v21 = v26;
      v25 = 0;
      goto label_6114;
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

// Function: sub_711b @ 0x711b
unsigned long sub_711b(long a0,unsigned long a1,char *a2,unsigned long a3,unsigned int *a4)
{
  int v1;
  unsigned long v2; // rax
  
  if (!a4)
    a4 = (unsigned int *)0xf180;
  v1 = *__errno_location();
  v2 = sub_605a(a0,a1,a2,a3,*a4,a4[1],&a4[2],*(char **)&a4[10],*(char **)&a4[0xc]);
  *__errno_location() = v1;
  return v2;
}

// Function: sub_71c3 @ 0x71c3
void sub_71c3(char *a0,unsigned long a1,unsigned int *a2)
{
  sub_71f9(a0,a1,NULL,a2);
}

// Function: sub_71f9 @ 0x71f9
long sub_71f9(char *a0,unsigned long a1,long *a2,unsigned int *a3)
{
  int v1;
  unsigned int v2; // eax
  long v3; // rax
  long v4; // rax
  
  if (!a3)
    a3 = (unsigned int *)0xf180;
  v1 = *__errno_location();
  v2 = (unsigned int)(a2 == NULL) | a3[1];
  v3 = sub_605a(0,0,a0,a1,*a3,v2,&a3[2],*(char **)&a3[10],*(char **)&a3[0xc]);
  v4 = sub_8e27(v3 + 1U);
  sub_605a(v4,v3 + 1U,a0,a1,*a3,v2,&a3[2],*(char **)&a3[10],*(char **)&a3[0xc]);
  *__errno_location() = v1;
  if (a2)
    *a2 = v3;
  return v4;
}

// Function: sub_7325 @ 0x7325
void sub_7325(void) // return-dupe
{
  void *v1;
  int v2; // stack - 0x14
  
  v1 = dat_f0b0;
  for (v2 = 1; v2 < dat_f040; v2 = v2 + 1) {
    free(*(void **)((long)v1 + (long)v2 * 0x10 + 8));
  }
  if (*(long *)((long)v1 + 8) != 0xf1c0) {
    free(*(void **)((long)v1 + 8));
    dat_f0a0 = 0x100;
    dat_f0a8 = 0xf1c0;
  }
  if (v1 == (void *)0xf0a0) {
    dat_f040 = 1;
    return;
  }
  free(v1);
  dat_f040 = 1;
  dat_f0b0 = (void *)0xf0a0;
}

// Function: sub_73e4 @ 0x73e4
void * sub_73e4(int a0,char *a1,unsigned long a2,struct_0 *a3)
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
  v6 = dat_f0b0;
  if ((0 <= a0) && (a0 <= 0x7ffffffe)) {
    if (dat_f040 <= a0) {
      v5 = dat_f0b0 == (unsigned long *)0xf0a0;
      v4 = (long)dat_f040;
      if (v5)
        v3 = NULL;
      else {
        v3 = dat_f0b0;
      }
      dat_f0b0 = (unsigned long *)sub_90b7(v3,&v4,(long)((a0 - dat_f040) + 1),0x7fffffff,0x10);
      v6 = dat_f0b0;
      if (v5) {
        *dat_f0b0 = dat_f0a0;
        dat_f0b0[1] = dat_f0a8;
      }
      memset(&dat_f0b0[(long)dat_f040 * 2],0,(v4 - dat_f040) * 0x10);
      dat_f040 = (int)v4;
    }
    v8 = v6[(long)a0 * 2];
    v7 = (void *)v6[(long)a0 * 2 + 1];
    v2 = a3->field_0x4 | 1;
    v9 = sub_605a(v7,v8,a1,a2,a3->field_0x0,v2,a3->field_0x8,a3->field_0x28,a3->field_0x30);
    if (v8 <= v9) {
      v8 = v9 + 1;
      v6[(long)a0 * 2] = v8;
      if (v7 != (void *)0xf1c0)
        free(v7);
      v7 = (void *)sub_8e27(v8);
      v6[(long)a0 * 2 + 1] = v7;
      sub_605a(v7,v8,a1,a2,a3->field_0x0,v2,a3->field_0x8,a3->field_0x28,a3->field_0x30);
    }
    *__errno_location() = v1;
    return v7;
  }
  abort(); // no-return
}

// Function: sub_7693 @ 0x7693
void sub_7693(unsigned int a0,char *a1)
{
  sub_73e4(a0,a1,0xffffffffffffffff,(struct_0 *)0xf180);
}

// Function: sub_76c7 @ 0x76c7
void sub_76c7(unsigned int a0,char *a1,unsigned long a2)
{
  sub_73e4(a0,a1,a2,(struct_0 *)0xf180);
}

// Function: sub_76f9 @ 0x76f9
void sub_76f9(char *a0)
{
  sub_7693(0,a0);
}

// Function: sub_771c @ 0x771c
void sub_771c(char *a0,unsigned long a1)
{
  sub_76c7(0,a0,a1);
}

// Function: sub_7747 @ 0x7747
void sub_7747(unsigned int a0,unsigned int a1,char *a2)
{
  char v1 [56];
  
  sub_5ed0(v1,a1);
  sub_73e4(a0,a2,0xffffffffffffffff,v1);
}

// Function: sub_77af @ 0x77af
void sub_77af(unsigned int a0,unsigned int a1,char *a2,unsigned long a3)
{
  char v1 [56];
  
  sub_5ed0(v1,a1);
  sub_73e4(a0,a2,a3,v1);
}

// Function: sub_7815 @ 0x7815
void sub_7815(unsigned int a0,char *a1)
{
  sub_7747(0,a0,a1);
}

// Function: sub_783d @ 0x783d
void sub_783d(unsigned int a0,char *a1,unsigned long a2)
{
  sub_77af(0,a0,a1,a2);
}

// Function: sub_786d @ 0x786d
void sub_786d(char *a0,unsigned long a1,char a2)
{
  unsigned long v1; // stack - 0x48
  unsigned long v2; // stack - 0x40
  unsigned long v3; // stack - 0x38
  unsigned long v4; // stack - 0x30
  unsigned long v5; // stack - 0x28
  unsigned long v6; // stack - 0x20
  unsigned long v7; // stack - 0x18
  
  v1 = dat_f180;
  v2 = dat_f188;
  v3 = dat_f190;
  v4 = dat_f198;
  v5 = dat_f1a0;
  v6 = dat_f1a8;
  v7 = dat_f1b0;
  sub_5da4(&v1,(int)a2,1);
  sub_73e4(0,a0,a1,&v1);
}

// Function: sub_7928 @ 0x7928
void sub_7928(char *a0,char a1)
{
  sub_786d(a0,0xffffffffffffffff,(int)a1);
}

// Function: sub_7956 @ 0x7956
void sub_7956(char *a0)
{
  sub_7928(a0,0x3a);
}

// Function: sub_7979 @ 0x7979
void sub_7979(char *a0,unsigned long a1)
{
  sub_786d(a0,a1,0x3a);
}

// Function: sub_79a7 @ 0x79a7
void sub_79a7(unsigned int a0,unsigned int a1,char *a2)
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
  sub_5ed0(&v1,a1);
  v2 = v1;
  v12 = v3;
  v13 = v4;
  v14 = v5;
  v15 = v6;
  v16 = v7;
  v17 = v8;
  sub_5da4(&v2,0x3a,1);
  sub_73e4(v11,v9,0xffffffffffffffff,&v2);
}

// Function: sub_7a69 @ 0x7a69
void sub_7a69(unsigned int a0,long a1,long a2,char *a3)
{
  sub_7aa3(a0,a1,a2,a3,0xffffffffffffffff);
}

// Function: sub_7aa3 @ 0x7aa3
void sub_7aa3(unsigned int a0,long a1,long a2,char *a3,unsigned long a4)
{
  unsigned long v1; // stack - 0x48
  unsigned long v2; // stack - 0x40
  unsigned long v3; // stack - 0x38
  unsigned long v4; // stack - 0x30
  unsigned long v5; // stack - 0x28
  unsigned long v6; // stack - 0x20
  unsigned long v7; // stack - 0x18
  
  v1 = dat_f180;
  v2 = dat_f188;
  v3 = dat_f190;
  v4 = dat_f198;
  v5 = dat_f1a0;
  v6 = dat_f1a8;
  v7 = dat_f1b0;
  sub_5e6e(&v1,a1,a2);
  sub_73e4(a0,a3,a4,&v1);
}

// Function: sub_7b61 @ 0x7b61
void sub_7b61(long a0,long a1,char *a2)
{
  sub_7a69(0,a0,a1,a2);
}

// Function: sub_7b94 @ 0x7b94
void sub_7b94(long a0,long a1,char *a2,unsigned long a3)
{
  sub_7aa3(0,a0,a1,a2,a3);
}

// Function: sub_7bd2 @ 0x7bd2
void sub_7bd2(unsigned int a0,char *a1,unsigned long a2)
{
  sub_73e4(a0,a1,a2,(struct_0 *)0xf060);
}

// Function: sub_7c04 @ 0x7c04
void sub_7c04(char *a0,unsigned long a1)
{
  sub_7bd2(0,a0,a1);
}

// Function: sub_7c2f @ 0x7c2f
void sub_7c2f(unsigned int a0,char *a1)
{
  sub_7bd2(a0,a1,0xffffffffffffffff);
}

// Function: sub_7c5c @ 0x7c5c
void sub_7c5c(char *a0)
{
  sub_7c2f(0,a0);
}

// Function: sub_7c7f @ 0x7c7f
void sub_7c7f(unsigned long *a0)
{
  *a0 = 0;
  a0[1] = 0;
  a0[2] = 0;
  _obstack_begin((obstack *)&a0[3],0,0,dat_efe8,dat_efc8);
  _obstack_begin((obstack *)&a0[0xe],0,0,dat_efe8,dat_efc8);
  _obstack_begin((obstack *)&a0[0x19],0,0,dat_efe8,dat_efc8);
}

// Function: sub_7d41 @ 0x7d41
void sub_7d41(long a0)
{
  _obstack_free((obstack *)(a0 + 0x18),NULL);
  _obstack_free((obstack *)(a0 + 0x70),NULL);
  _obstack_free((obstack *)(a0 + 200),NULL);
}

// Function: sub_7e7a @ 0x7e7a
void sub_7e7a(struct_7 *a0)
{
  long v1; // stack - 0x70
  obstack *v10; // stack - 0x28
  unsigned long v11; // stack - 0x20
  obstack *v12; // stack - 0x18
  char *v2; // stack - 0x68
  char *v3; // stack - 0x60
  long v4; // stack - 0x58
  long v5; // stack - 0x50
  obstack *v6; // stack - 0x48
  obstack *v7; // stack - 0x40
  obstack *v8; // stack - 0x38
  long *v9; // stack - 0x30
  
  v2 = &a0->field_0x8[0x10];
  v1 = (a0->field_0x30 - a0->field_0x28) + -1;
  v3 = &a0->field_0x8[0x10];
  v4 = a0->field_0x28;
  if (v4 == a0->field_0x30)
    a0->field_0x68 = a0->field_0x68 | 2;
  a0->field_0x30 = ~a0->field_0x48 & a0->field_0x30 + a0->field_0x48;
  if ((unsigned long)(a0->field_0x38 - a0->field_0x20) < (unsigned long)(a0->field_0x30 - a0->field_0x20))
    a0->field_0x30 = a0->field_0x38;
  a0->field_0x28 = a0->field_0x30;
  v6 = (obstack *)&a0->field_0x69[7];
  v5 = v4;
  v7 = v6;
  if ((unsigned long)(a0->field_0x90 - a0->field_0x88) <= 7)
    _obstack_newchunk(v6,8);
  v8 = v6;
  v9 = *(long **)&v6->field_0x18;
  *v9 = v5;
  *(long *)&v6->field_0x18 = *(long *)&v6->field_0x18 + 8;
  v10 = (obstack *)&a0->field_0x98[0x30];
  v11 = 8;
  v12 = v10;
  if ((unsigned long)(a0->field_0xe8 - a0->field_0xe0) < 8)
    _obstack_newchunk(v10,8);
  memcpy(*(void **)&v10->field_0x18,&v1,v11);
  *(unsigned long *)&v10->field_0x18 = *(long *)&v10->field_0x18 + v11;
  a0->field_0x0 = a0->field_0x0 + 1;
}

// Function: sub_809e @ 0x809e
unsigned long sub_809e(FILE *a0,long a1)
{
  char *v1;
  long v2;
  int v3; // eax
  
  while( true ) {
    v3 = fgetc(a0);
    if (v3 == -1) break;
    if (*(long *)(a1 + 0x38) == *(long *)(a1 + 0x30))
      _obstack_newchunk((obstack *)(a1 + 0x18),1);
    v1 = *(char **)(a1 + 0x30);
    *(char **)(a1 + 0x30) = &v1[1];
    *v1 = (char)v3;
    if (!v3)
      sub_7e7a(a1);
  }
  if (*(long *)(a1 + 0x30) != *(long *)(a1 + 0x28)) {
    if (*(long *)(a1 + 0x38) == *(long *)(a1 + 0x30))
      _obstack_newchunk((obstack *)(a1 + 0x18),1);
    v1 = *(char **)(a1 + 0x30);
    *(char **)(a1 + 0x30) = &v1[1];
    *v1 = 0;
    sub_7e7a(a1);
  }
  if ((unsigned long)(*(long *)(a1 + 0x90) - *(long *)(a1 + 0x88)) <= 7)
    _obstack_newchunk((obstack *)(a1 + 0x70),8);
  **(unsigned long **)(a1 + 0x88) = 0;
  *(long *)(a1 + 0x88) = *(long *)(a1 + 0x88) + 8;
  v2 = *(long *)(a1 + 0x80);
  if (v2 == *(long *)(a1 + 0x88))
    *(unsigned char *)(a1 + 0xc0) = *(unsigned char *)(a1 + 0xc0) | 2;
  *(unsigned long *)(a1 + 0x88) = ~*(unsigned long *)(a1 + 0xa0) & *(long *)(a1 + 0x88) + *(long *)(a1 + 0xa0);
  if ((unsigned long)(*(long *)(a1 + 0x90) - *(long *)(a1 + 0x78)) < (unsigned long)(*(long *)(a1 + 0x88) - *(long *)(a1 + 0x78)))
    *(unsigned long *)(a1 + 0x88) = *(unsigned long *)(a1 + 0x90);
  *(unsigned long *)(a1 + 0x80) = *(unsigned long *)(a1 + 0x88);
  *(long *)(a1 + 8) = v2;
  v2 = *(long *)(a1 + 0xd8);
  if (v2 == *(long *)(a1 + 0xe0))
    *(unsigned char *)(a1 + 0x118) = *(unsigned char *)(a1 + 0x118) | 2;
  *(unsigned long *)(a1 + 0xe0) = ~*(unsigned long *)(a1 + 0xf8) & *(long *)(a1 + 0xe0) + *(long *)(a1 + 0xf8);
  if ((unsigned long)(*(long *)(a1 + 0xe8) - *(long *)(a1 + 0xd0)) < (unsigned long)(*(long *)(a1 + 0xe0) - *(long *)(a1 + 0xd0)))
    *(unsigned long *)(a1 + 0xe0) = *(unsigned long *)(a1 + 0xe8);
  *(unsigned long *)(a1 + 0xd8) = *(unsigned long *)(a1 + 0xe0);
  *(long *)(a1 + 0x10) = v2;
  v3 = ferror(a0);
  return CONCAT71((undefined7)(CONCAT44(dat_4,v3) >> 8),v3 == 0);
}

// Function: sub_8412 @ 0x8412
long sub_8412(int a0,void *a1,unsigned long a2)
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

// Function: sub_8486 @ 0x8486
void sub_8486(FILE *a0,char *a1,char *a2,char *a3,unsigned long *a4,unsigned long a5) // return-dupe x10
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

// Function: sub_8a82 @ 0x8a82
void sub_8a82(FILE *a0,char *a1,char *a2,char *a3,unsigned long *a4)
{
  long v1; // stack - 0x10
  
  for (v1 = 0; a4[v1]; v1 = v1 + 1) {
  }
  sub_8486(a0,a1,a2,a3,a4,v1);
}

// Function: sub_8af5 @ 0x8af5
void sub_8af5(FILE *a0,char *a1,char *a2,char *a3,struct_8 *a4)
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
  sub_8486(a0,a1,a2,a3,v2,v3);
}

// Function: sub_8bfb @ 0x8bfb
void sub_8bfb(unsigned long a0,unsigned long a1,unsigned long a2,unsigned long a3,unsigned long a4,unsigned long a5,unsigned long a6,unsigned long a7,FILE *a8,char *a9,char *a10,char *a11,unsigned long a12,unsigned long a13)
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
  sub_8af5(a8,a9,a10,a11,&v3);
}

// Function: sub_8cdf @ 0x8cdf
void sub_8cdf(void)
{
  fputs_unlocked("\n",stdout);
  printf(gettext("Report bugs to: %s\n"),"bug-coreutils@gnu.org");
  printf(gettext("%s home page: <%s>\n"),"GNU coreutils","https://www.gnu.org/software/coreutils/");
  printf(gettext("General help using GNU software: <%s>\n"),"https://www.gnu.org/gethelp/");
}

// Function: sub_8d88 @ 0x8d88
void sub_8d88(void *a0,unsigned long a1,unsigned long a2)
{
  sub_8ec1(a0,a1,a2);
}

// Function: sub_8db9 @ 0x8db9
long sub_8db9(long a0)
{
  if (!a0)
    sub_9413(); // no-return
  return a0;
}

// Function: sub_8ddb @ 0x8ddb
void sub_8ddb(unsigned long a0)
{
  sub_8db9(malloc(a0));
}

// Function: sub_8e01 @ 0x8e01
void sub_8e01(unsigned long a0)
{
  sub_8db9(sub_9690(a0));
}

// Function: sub_8e27 @ 0x8e27
void sub_8e27(unsigned long a0)
{
  sub_8ddb(a0);
}

// Function: sub_8e45 @ 0x8e45
void * sub_8e45(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = realloc(a0,a1);
  if ((!v1) && ((!a0 || (a1))))
    sub_9413(); // no-return
  return v1;
}

// Function: sub_8e90 @ 0x8e90
void sub_8e90(void *a0,unsigned long a1)
{
  sub_8db9(sub_96ae(a0,a1));
}

// Function: sub_8ec1 @ 0x8ec1
void * sub_8ec1(void *a0,unsigned long a1,unsigned long a2)
{
  void *v1; // rax
  
  v1 = reallocarray(a0,a1,a2);
  if ((!v1) && ((!a0 || ((a1 && (a2))))))
    sub_9413(); // no-return
  return v1;
}

// Function: sub_8f1b @ 0x8f1b
void sub_8f1b(void *a0,unsigned long a1,unsigned long a2)
{
  sub_8db9(sub_970e(a0,a1,a2));
}

// Function: sub_8f54 @ 0x8f54
void sub_8f54(unsigned long a0,unsigned long a1)
{
  sub_8ec1(NULL,a0,a1);
}

// Function: sub_8f7f @ 0x8f7f
void sub_8f7f(unsigned long a0,unsigned long a1)
{
  sub_8f1b(NULL,a0,a1);
}

// Function: sub_8faa @ 0x8faa
void sub_8faa(void *a0,unsigned long *a1)
{
  sub_8fd8(a0,a1,1);
}

// Function: sub_8fd8 @ 0x8fd8
unsigned long sub_8fd8(void *a0,unsigned long *a1,unsigned long a2)
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
      sub_9413(); // no-return
  }
  else if (!v6) {
    v1._8_8_ = 0;
    v1._0_8_ = a2;
    v2 = SUB168((ZEXT816(0) << 0x40 | ZEXT816(0x80)) / v1._0_16_,0);
    v6 = v2 + (unsigned long)(v2 == 0);
  }
  v4 = sub_8ec1(a0,v6,a2);
  *a1 = v6;
  return v4;
}

// Function: sub_90b7 @ 0x90b7
unsigned long sub_90b7(void *a0,long *a1,long a2,long a3,long a4) // ternary
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
    sub_9413(); // no-return
  v3 = sub_8e45(a0,v5);
  *a1 = v4;
  return v3;
}

// Function: sub_9273 @ 0x9273
void sub_9273(unsigned long a0)
{
  sub_92b9(a0,1);
}

// Function: sub_9296 @ 0x9296
void sub_9296(unsigned long a0)
{
  sub_92ea(a0,1);
}

// Function: sub_92b9 @ 0x92b9
void sub_92b9(unsigned long a0,unsigned long a1)
{
  sub_8db9(calloc(a0,a1));
}

// Function: sub_92ea @ 0x92ea
void sub_92ea(unsigned long a0,unsigned long a1)
{
  sub_8db9(sub_96e5(a0,a1));
}

// Function: sub_931b @ 0x931b
void sub_931b(void *a0,unsigned long a1)
{
  memcpy((void *)sub_8ddb(a1),a0,a1);
}

// Function: sub_9353 @ 0x9353
void sub_9353(void *a0,unsigned long a1)
{
  memcpy((void *)sub_8e01(a1),a0,a1);
}

// Function: sub_9393 @ 0x9393
void sub_9393(void *a0,unsigned long a1)
{
  void *v1; // rax
  
  v1 = (void *)sub_8e01(a1 + 1);
  *(char *)((long)v1 + a1) = 0;
  memcpy(v1,a0,a1);
}

// Function: sub_93e2 @ 0x93e2
void sub_93e2(char *a0)
{
  sub_931b(a0,strlen(a0) + 1);
}

// Function: sub_9413 @ 0x9413
void sub_9413(void)
{
  error(dat_f020,0,"%s",gettext("memory exhausted"));
  abort(); // no-return
}

// Function: sub_9453 @ 0x9453
void sub_9453(void)
{
  return;
}

// Function: sub_945e @ 0x945e
void sub_945e(unsigned int a0,unsigned int a1) // return-dupe
{
  if (0 <= (int)sub_94a0(a0,a1))
    return;
  sub_9453();
}

// Function: sub_948b @ 0x948b
unsigned long sub_948b(void)
{
  return 0;
}

// Function: sub_94a0 @ 0x94a0
void sub_94a0(unsigned int a0,unsigned int a1)
{
  sub_948b(a0,a1);
}

// Function: sub_94c3 @ 0x94c3
int sub_94c3(char *a0,char *a1) // early-return
{
  unsigned char v1; // al
  unsigned char v2; // al
  
  if (a0 == a1)
    return 0;
  do {
    v1 = sub_a16b((unsigned char)*a0);
    v2 = sub_a16b((unsigned char)*a1);
    if (!v1) break;
    a0 = &a0[1];
    a1 = &a1[1];
  } while (v1 == v2);
  return (unsigned int)v1 - (unsigned int)v2;
}

// Function: sub_953d @ 0x953d
unsigned long sub_953d(FILE *a0)
{
  int v1; // eax
  unsigned long v2; // rax
  bool v3; // zf
  
  v2 = __fpending(a0);
  v1 = ferror_unlocked(a0);
  v3 = sub_552b(a0) == 0;
  if ((!v1) && ((v3 || ((!v2 && (*__errno_location() == 9))))))
    return 0;
  if (v3)
    *__errno_location() = 0;
  return 0xffffffff;
}

// Function: sub_95cc @ 0x95cc
unsigned int sub_95cc(unsigned int a0) // return-dupe
{
  char v1 [264];
  
  if (sub_9ec9(a0,v1,0x101))
    return 0;
  if ((strcmp(v1,"C")) && (strcmp(v1,"POSIX")))
    return 1;
  return 0;
}

// Function: sub_9676 @ 0x9676
unsigned long sub_9676(void)
{
  *__errno_location() = 0xc;
  return 0;
}

// Function: sub_9690 @ 0x9690
void sub_9690(unsigned long a0)
{
  malloc(a0);
}

// Function: sub_96ae @ 0x96ae
void sub_96ae(void *a0,unsigned long a1)
{
  realloc(a0,a1 == 0 | a1);
}

// Function: sub_96e5 @ 0x96e5
void sub_96e5(unsigned long a0,unsigned long a1)
{
  calloc(a0,a1);
}

// Function: sub_970e @ 0x970e
void sub_970e(void *a0,unsigned long a1,unsigned long a2)
{
  unsigned long v1; // stack - 0x20
  unsigned long v2; // stack - 0x18
  
  if ((!a1) || (v1 = a2, v2 = a1, !a2)) {
    v1 = 1;
    v2 = 1;
  }
  reallocarray(a0,v2,v1);
}

// Function: sub_975d @ 0x975d
char * sub_975d(void)
{
  char *v1; // stack - 0x10
  
  v1 = nl_langinfo(0xe);
  if (!v1)
    v1 = "";
  if (!*v1)
    v1 = "ASCII";
  return v1;
}

// Function: sub_97a5 @ 0x97a5
void sub_97a5(long a0,unsigned long a1) // return-dupe
{
  if (*(unsigned char *)(a0 + 0x50) & 1) {
    (**(void **)(a0 + 0x38))(*(unsigned long *)(a0 + 0x48),a1);
    return;
  }
  (**(void **)(a0 + 0x38))(a1);
}

// Function: sub_97f9 @ 0x97f9
void sub_97f9(long a0,unsigned long a1) // return-dupe
{
  if (*(unsigned char *)(a0 + 0x50) & 1) {
    (**(void **)(a0 + 0x40))(*(unsigned long *)(a0 + 0x48),a1);
    return;
  }
  (**(void **)(a0 + 0x40))(a1);
}

// Function: sub_984e @ 0x984e
unsigned long sub_984e(struct_6 *a0,long a1,long a2)
{
  long v1;
  long *v2;
  long v3; // stack - 0x30
  long v4; // stack - 0x28
  
  v3 = a2;
  if (!a2)
    v3 = 0x10;
  v4 = a1;
  if (!a1)
    v4 = 0xfe0;
  a0->field_0x0 = v4;
  a0->field_0x30 = v3 + -1;
  v1 = a0->field_0x0;
  a0->field_0x8 = (long *)sub_97a5(a0,v1);
  v2 = a0->field_0x8;
  if (!v2)
    (*obstack_alloc_failed_handler)();
  a0->field_0x10 = -v3 & (long)v2 + v3 + 0xf;
  a0->field_0x18 = a0->field_0x10;
  *v2 = a0->field_0x0 + (long)v2;
  a0->field_0x20 = *v2;
  v2[1] = 0;
  a0->field_0x50 = a0->field_0x50 & 0xfd;
  a0->field_0x50 = a0->field_0x50 & 0xfb;
  return 1;
}

// Function: _obstack_begin @ 0x9974
int _obstack_begin(obstack *a0,unsigned long a1,unsigned long a2,void *a3,void *a4)
{
  unsigned char v1; // dl
  
  *(void **)&a0->field_0x38 = a3;
  *(void **)&a0->field_0x40 = a4;
  v1 = a0->field_0x50 & 0xfe;
  a0->field_0x50 = v1;
  return sub_984e(a0,a1,a2);
}

// Function: _obstack_begin_1 @ 0x99d3
int _obstack_begin_1(obstack *a0,unsigned long a1,unsigned long a2,void *a3,void *a4,void *a5)
{
  unsigned char v1; // dl
  
  *(void **)&a0->field_0x38 = a3;
  *(void **)&a0->field_0x40 = a4;
  *(void **)&a0->field_0x48 = a5;
  v1 = a0->field_0x50 | 1;
  a0->field_0x50 = v1;
  return sub_984e(a0,a1,a2);
}

// Function: _obstack_newchunk @ 0x9a42
void _obstack_newchunk(obstack *a0,unsigned long a1)
{
  long v1;
  unsigned long v2; // rax
  unsigned long v3; // rax
  void *v4; // rax
  unsigned long *v5; // stack - 0x40
  unsigned long v6; // stack - 0x38
  
  v1 = *(long *)&a0->field_0x8;
  v5 = NULL;
  v2 = *(long *)&a0->field_0x18 - *(long *)&a0->field_0x10;
  a1 += v2;
  v3 = a1 + *(long *)&a0->field_0x30;
  v6 = v3 + (v2 >> 3) + 100;
  if (v6 < v3)
    v6 = v3;
  if (v6 < *(unsigned long *)a0)
    v6 = *(unsigned long *)a0;
  if ((v2 <= a1) && (a1 <= v3))
    v5 = (unsigned long *)sub_97a5(a0,v6);
  if (!v5)
    (*obstack_alloc_failed_handler)();
  *(unsigned long **)&a0->field_0x8 = v5;
  v5[1] = v1;
  *(unsigned long *)&a0->field_0x20 = (long)v5 + v6;
  *v5 = *(unsigned long *)&a0->field_0x20;
  v4 = (void *)(~*(unsigned long *)&a0->field_0x30 & (long)v5 + *(long *)&a0->field_0x30 + 0x10);
  memcpy(v4,*(void **)&a0->field_0x10,v2);
  if ((!(a0->field_0x50 & 2)) && (*(unsigned long *)&a0->field_0x10 == (~*(unsigned long *)&a0->field_0x30 & v1 + 0x10 + *(long *)&a0->field_0x30))) {
    v5[1] = *(unsigned long *)(v1 + 8);
    sub_97f9(a0,v1);
  }
  *(void **)&a0->field_0x10 = v4;
  *(unsigned long *)&a0->field_0x18 = *(long *)&a0->field_0x10 + v2;
  a0->field_0x50 = a0->field_0x50 & 0xfd;
}

// Function: _obstack_allocated_p @ 0x9c39
bool _obstack_allocated_p(long a0,unsigned long *a1)
{
  unsigned long *v1; // stack - 0x18
  
  for (v1 = *(unsigned long **)(a0 + 8); (v1 && ((a1 <= v1 || ((unsigned long *)*v1 < a1)))); v1 = (unsigned long *)v1[1]) {
  }
  return v1 != NULL;
}

// Function: _obstack_free @ 0x9c96
void _obstack_free(obstack *a0,void *a1) // return-dupe
{
  unsigned long *v1;
  unsigned long *v2; // stack - 0x18
  
  v2 = *(unsigned long **)&a0->field_0x8;
  while ((v2 && ((a1 <= v2 || ((void *)*v2 < a1))))) {
    v1 = (unsigned long *)v2[1];
    sub_97f9(a0,v2);
    a0->field_0x50 = a0->field_0x50 | 2;
    v2 = v1;
  }
  if (v2) {
    *(void **)&a0->field_0x18 = a1;
    *(unsigned long *)&a0->field_0x10 = *(unsigned long *)&a0->field_0x18;
    *(unsigned long *)&a0->field_0x20 = *v2;
    *(unsigned long **)&a0->field_0x8 = v2;
    return;
  }
  if (!a1)
    return;
  abort(); // no-return
}

// Function: _obstack_memory_used @ 0x9d5a
unsigned long _obstack_memory_used(obstack *a0)
{
  long *v1; // stack - 0x18
  unsigned long v2; // stack - 0x10
  
  v2 = 0;
  for (v1 = *(long **)&a0->field_0x8; v1; v1 = (long *)v1[1]) {
    v2 += *v1 - (long)v1;
  }
  return v2;
}

// Function: sub_9da4 @ 0x9da4
void sub_9da4(void)
{
  fprintf(stderr,"%s\n",gettext("memory exhausted"));
  exit(dat_f020); // no-return
}

// Function: sub_9de9 @ 0x9de9
char * sub_9de9(int a0)
{
  return setlocale(a0,NULL);
}

// Function: sub_9e11 @ 0x9e11
unsigned long sub_9e11(unsigned int a0,char *a1,unsigned long a2) // return-dupe x2
{
  char *v1; // rax
  unsigned long v2; // rax
  
  v1 = (char *)sub_9de9(a0);
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

// Function: sub_9ec9 @ 0x9ec9
void sub_9ec9(unsigned int a0,char *a1,unsigned long a2)
{
  sub_9e11(a0,a1,a2);
}

// Function: sub_9ef7 @ 0x9ef7
void sub_9ef7(unsigned int a0)
{
  sub_9de9(a0);
}

// Function: sub_9f12 @ 0x9f12
unsigned long sub_9f12(int a0) // return-dupe
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

// Function: sub_9f50 @ 0x9f50
unsigned long sub_9f50(int a0) // return-dupe
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

// Function: sub_9f82 @ 0x9f82
bool sub_9f82(unsigned int a0)
{
  return a0 < 0x80;
}

// Function: sub_9fa3 @ 0x9fa3
unsigned int sub_9fa3(int a0) // return-dupe
{
  if ((a0 != 0x20) && (a0 != 9))
    return 0;
  return 1;
}

// Function: sub_9fcb @ 0x9fcb
unsigned long sub_9fcb(int a0) // return-dupe
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

// Function: sub_9ff8 @ 0x9ff8
bool sub_9ff8(int a0)
{
  return (unsigned int)(a0 - 0x30U) < 10;
}

// Function: sub_a01c @ 0xa01c
bool sub_a01c(int a0)
{
  return (unsigned int)(a0 - 0x21U) < 0x5e;
}

// Function: sub_a040 @ 0xa040
bool sub_a040(int a0)
{
  return (unsigned int)(a0 - 0x61U) < 0x1a;
}

// Function: sub_a064 @ 0xa064
bool sub_a064(int a0)
{
  return (unsigned int)(a0 - 0x20U) < 0x5f;
}

// Function: sub_a088 @ 0xa088
unsigned long sub_a088(int a0) // return-dupe, return-dupe x3
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

// Function: sub_a0d2 @ 0xa0d2
unsigned long sub_a0d2(int a0) // return-dupe
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

// Function: sub_a0ff @ 0xa0ff
bool sub_a0ff(int a0)
{
  return (unsigned int)(a0 - 0x41U) < 0x1a;
}

// Function: sub_a123 @ 0xa123
unsigned long sub_a123(int a0) // return-dupe
{
  if (((unsigned int)(a0 - 0x30U) <= 0x36) && (0x7e0000007e03ffU >> ((unsigned char)(a0 - 0x30U) & 0x3f) & 1))
    return 1;
  return 0;
}

// Function: sub_a16b @ 0xa16b
int sub_a16b(int a0)
{
  if ((unsigned int)(a0 - 0x41U) <= 0x19)
    a0 += 0x20;
  return a0;
}

// Function: sub_a18e @ 0xa18e
int sub_a18e(int a0)
{
  if ((unsigned int)(a0 - 0x61U) <= 0x19)
    a0 -= 0x20;
  return a0;
}

// Function: sub_a1b1 @ 0xa1b1
unsigned long sub_a1b1(char *a0,unsigned int a1,long *a2,long *a3) // return-dupe
{
  char v1 [16];
  char v10 [32];
  char v11 [32];
  int v12; // eax
  int v13; // eax
  int v14; // eax
  int v15; // eax
  int v16; // eax
  int v17; // eax
  int v18; // eax
  char *v19; // rax
  char v2 [16];
  char (*v20)[32]; // rax
  char v21 [16344];
  char v22 [32]; // ymm0
  char (*v23)[32]; // stack - 0x4360
  long v24; // stack - 0x4358
  long v25; // stack - 0x4350
  unsigned long v26; // stack - 0x4348
  char (*v27)[32]; // stack - 0x4340
  char v28 [32]; // stack - 0x42a0
  char v29 [608]; // stack - 0x4280
  char v3 [16];
  char v4 [16];
  char v5 [16];
  char v6 [16];
  char v7 [16];
  char v8 [16];
  char v9 [16];
  
  v24 = 0;
  v25 = 0;
  if ((a2) && (a3)) {
    v1 = vpinsrb_avx(ZEXT116(10),10,1);
    v2 = vpinsrb_avx(ZEXT116(10),10,1);
    v3 = vpinsrb_avx(ZEXT116(10),10,1);
    v4 = vpinsrb_avx(ZEXT116(10),10,1);
    v5 = vpinsrb_avx(ZEXT116(10),10,1);
    v6 = vpinsrb_avx(ZEXT116(10),10,1);
    v7 = vpinsrb_avx(ZEXT116(10),10,1);
    v8 = vpinsrb_avx(ZEXT116(10),10,1);
    v1 = vpunpcklwd_avx(v1,v2);
    v2 = vpunpcklwd_avx(v3,v4);
    v3 = vpunpcklwd_avx(v5,v6);
    v4 = vpunpcklwd_avx(v7,v8);
    v1 = vpunpckldq_avx(v1,v2);
    v9 = vpunpcklqdq_avx(v1,vpunpckldq_avx(v3,v4));
    v1 = vpinsrb_avx(ZEXT116(10),10,1);
    v2 = vpinsrb_avx(ZEXT116(10),10,1);
    v3 = vpinsrb_avx(ZEXT116(10),10,1);
    v4 = vpinsrb_avx(ZEXT116(10),10,1);
    v5 = vpinsrb_avx(ZEXT116(10),10,1);
    v6 = vpinsrb_avx(ZEXT116(10),10,1);
    v7 = vpinsrb_avx(ZEXT116(10),10,1);
    v8 = vpinsrb_avx(ZEXT116(10),10,1);
    v1 = vpunpcklwd_avx(v1,v2);
    v2 = vpunpcklwd_avx(v3,v4);
    v3 = vpunpcklwd_avx(v5,v6);
    v4 = vpunpcklwd_avx(v7,v8);
    v1 = vpunpckldq_avx(v1,v2);
    v1 = vpunpcklqdq_avx(v1,vpunpckldq_avx(v3,v4));
    v22._0_16_ = ZEXT116(0) * v1 + ZEXT116(1) * v9;
    v22._16_16_ = ZEXT116(1) * v1;
    while( true ) {
      v29._0_32_ = ZEXT1632(ZEXT816(0));
      v28._0_32_ = ZEXT1632(ZEXT816(0));
      v26 = sub_8412(a1,v21,0x3fc0);
      if (!v26) {
        *a2 = v24;
        *a3 = v25;
        return 1;
      }
      if (v26 == 0xffffffffffffffff) break;
      v25 += v26;
      v23 = (char (*)[32])v21;
      v20 = (char (*)[32])&v21[v26];
      for (; 0x40 <= v26; v26 = v26 - 0x40) {
        v11 = vpcmpeqb_avx2(*v23,v22._0_32_);
        v10 = vpcmpeqb_avx2(v23[1],v22._0_32_);
        v28._0_32_ = vpsubb_avx2(v28._0_32_,v11);
        v29._0_32_ = vpsubb_avx2(v29._0_32_,v10);
        v23 = &v23[2];
      }
      v11 = vpsadbw_avx2(v28._0_32_,ZEXT1632(ZEXT816(0)));
      v12 = vpextrw_avx(SUB3216(v11,0),0);
      v13 = vpextrw_avx(SUB3216(v11,0),4);
      v28._16_16_ = SUB3216(v11,0x10);
      v14 = vpextrw_avx(v28._16_16_,0);
      v15 = vpextrw_avx(v28._16_16_,4);
      v11 = vpsadbw_avx2(v29._0_32_,ZEXT1632(ZEXT816(0)));
      v16 = vpextrw_avx(SUB3216(v11,0),0);
      v17 = vpextrw_avx(SUB3216(v11,0),4);
      v29._16_16_ = SUB3216(v11,0x10);
      v18 = vpextrw_avx(v29._16_16_,0);
      v24 = v24 + (int)((v15 & 0xffff) + (v12 & 0xffff) + (v13 & 0xffff) + (v14 & 0xffff)) + (long)(int)((vpextrw_avx(v29._16_16_,4) & 0xffff) + (v16 & 0xffff) + (v17 & 0xffff) + (v18 & 0xffff));
      for (v27 = v23; v27 != v20; v27 = (char (*)[32])&(*v27)[1]) {
        v24 += (int)(unsigned int)((*v27)[0] == '\n');
      }
    }
    v19 = (char *)sub_79a7(0,3,a0);
    error(0,*__errno_location(),"%s",v19);
    return 0;
  }
  return 0;
}

// Function: sub_ac00 @ 0xac00
void sub_ac00(unsigned long a0)
{
  __cxa_atexit(a0,0,dat_f008); // tail-call
}

// Function: _DT_FINI @ 0xac14
void _DT_FINI(void)
{
  return;
}
