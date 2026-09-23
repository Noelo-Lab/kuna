// Function: try_help @ 0x3c09

void try_help(long param_1,undefined8 param_2)

{
  int iVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  
  if (param_1 != 0) {
    uVar2 = gettext(param_1);
    error(0,0,uVar2,param_2);
  }
  uVar2 = DAT_001225e8;
  uVar3 = gettext("Try \'%s --help\' for more information.");
  error(2,0,uVar3,uVar2);
  iVar1 = ferror_unlocked(stdout);
  if (iVar1 != 0) {
    FUN_00103e69("write failed");
  }
  iVar1 = fclose(stdout);
  if (iVar1 != 0) {
    uVar2 = gettext("standard output");
    FUN_00103eab(uVar2);
  }
  return;
}




// Function: check_stdout @ 0x3c7f

void check_stdout(void)

{
  int iVar1;
  undefined8 uVar2;
  
  iVar1 = ferror_unlocked(stdout);
  if (iVar1 != 0) {
    FUN_00103e69("write failed");
  }
  iVar1 = fclose(stdout);
  if (iVar1 != 0) {
    uVar2 = gettext("standard output");
    FUN_00103eab(uVar2);
  }
  return;
}




// Function: usage @ 0x3cd6

void usage(void)

{
  FILE *__stream;
  char *pcVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined **local_20;
  
  uVar2 = DAT_001225e8;
  pcVar1 = (char *)gettext("Usage: %s [OPTION]... FILE1 FILE2\n");
  printf(pcVar1,uVar2);
  uVar2 = gettext("Side-by-side merge of differences between FILE1 and FILE2.");
  printf("%s\n\n",uVar2);
  __stream = stdout;
  pcVar1 = (char *)gettext(
                          "Mandatory arguments to long options are mandatory for short options too.\n"
                          );
  fputs_unlocked(pcVar1,__stream);
  for (local_20 = &PTR_s__o____output_FILE_operate_intera_00111a40; *local_20 != (undefined *)0x0;
      local_20 = local_20 + 1) {
    if (**local_20 == '\0') {
      putchar_unlocked(10);
    }
    else {
      uVar2 = gettext(*local_20);
      printf("  %s\n",uVar2);
    }
  }
  uVar2 = gettext("Exit status is 0 if inputs are the same, 1 if different, 2 if trouble.");
  uVar3 = gettext("If a FILE is \'-\', read standard input.");
  printf("\n%s\n%s\n",uVar3,uVar2);
  FUN_00108c7b();
  return;
}




// Function: cleanup @ 0x3df5

void cleanup(void)

{
  if (0 < DAT_001120b8) {
    kill(DAT_001120b8,0xd);
  }
  if (DAT_001120a8 != (char *)0x0) {
    unlink(DAT_001120a8);
  }
  return;
}




// Function: exiterr @ 0x3e3e

void exiterr(void)

{
  FUN_00103df5(0);
  FUN_0010510d(0);
  FUN_001051e3();
                    /* WARNING: Subroutine does not return */
  exit(2);
}




// Function: fatal @ 0x3e69

void fatal(undefined8 param_1)

{
  undefined8 uVar1;
  
  uVar1 = gettext(param_1);
  error(0,0,&DAT_0010d847,uVar1);
                    /* WARNING: Subroutine does not return */
  FUN_00103e3e();
}




// Function: perror_fatal @ 0x3eab

void perror_fatal(undefined8 param_1)

{
  int iVar1;
  int *piVar2;
  
  piVar2 = __errno_location();
  iVar1 = *piVar2;
  FUN_001051e3();
  error(0,iVar1,&DAT_0010d847,param_1);
                    /* WARNING: Subroutine does not return */
  FUN_00103e3e();
}




// Function: check_child_status @ 0x3ef1

void check_child_status(int param_1,uint param_2,int param_3,undefined8 param_4)

{
  uint uVar1;
  char *pcVar2;
  undefined8 uVar3;
  
  if ((param_1 == 0) && ((param_2 & 0x7f) == 0)) {
    uVar1 = (int)param_2 >> 8 & 0xff;
  }
  else {
    uVar1 = 0x7fffffff;
  }
  if (param_3 < (int)uVar1) {
    if (uVar1 == 0x7e) {
      pcVar2 = "subsidiary program \'%s\' could not be invoked";
    }
    else if (uVar1 == 0x7f) {
      pcVar2 = "subsidiary program \'%s\' not found";
    }
    else if (uVar1 == 0x7fffffff) {
      pcVar2 = "subsidiary program \'%s\' failed";
    }
    else {
      pcVar2 = "subsidiary program \'%s\' failed (exit status %d)";
    }
    uVar3 = gettext(pcVar2);
    error(0,param_1,uVar3,param_4,uVar1);
                    /* WARNING: Subroutine does not return */
    FUN_00103e3e();
  }
  return;
}




// Function: ck_fopen @ 0x3fa3

FILE * ck_fopen(char *param_1,char *param_2)

{
  FILE *pFVar1;
  
  pFVar1 = fopen(param_1,param_2);
  if (pFVar1 == (FILE *)0x0) {
    FUN_00103eab(param_1);
  }
  return pFVar1;
}




// Function: ck_fclose @ 0x3fe7

void ck_fclose(FILE *param_1)

{
  int iVar1;
  
  iVar1 = fclose(param_1);
  if (iVar1 != 0) {
    FUN_00103eab("fclose");
  }
  return;
}




// Function: ck_fread @ 0x4019

size_t ck_fread(void *param_1,size_t param_2,FILE *param_3)

{
  int iVar1;
  size_t sVar2;
  undefined8 uVar3;
  
  sVar2 = fread_unlocked(param_1,1,param_2,param_3);
  if (sVar2 == 0) {
    iVar1 = ferror_unlocked(param_3);
    if (iVar1 != 0) {
      uVar3 = gettext("read failed");
      FUN_00103eab(uVar3);
    }
  }
  return sVar2;
}




// Function: ck_fwrite @ 0x4082

void ck_fwrite(void *param_1,size_t param_2,FILE *param_3)

{
  size_t sVar1;
  undefined8 uVar2;
  
  sVar1 = fwrite_unlocked(param_1,1,param_2,param_3);
  if (param_2 != sVar1) {
    uVar2 = gettext("write failed");
    FUN_00103eab(uVar2);
  }
  return;
}




// Function: ck_fflush @ 0x40d3

void ck_fflush(FILE *param_1)

{
  int iVar1;
  undefined8 uVar2;
  
  iVar1 = fflush_unlocked(param_1);
  if (iVar1 != 0) {
    uVar2 = gettext("write failed");
    FUN_00103eab(uVar2);
  }
  return;
}




// Function: expand_name @ 0x410d

char * expand_name(char *param_1,char param_2,undefined8 param_3)

{
  byte bVar1;
  int iVar2;
  void *__src;
  size_t __n;
  size_t __n_00;
  char *pcVar3;
  
  iVar2 = strcmp(param_1,"-");
  if (iVar2 == 0) {
    FUN_00103e69("cannot interactively merge standard input");
  }
  if (param_2 == '\x01') {
    __src = (void *)FUN_00105f32(param_3);
    __n = strlen(param_1);
    __n_00 = FUN_00105f9f(__src);
    pcVar3 = (char *)FUN_00105f32(param_1);
    if ((*pcVar3 == '\0') || (param_1[__n - 1] == '/')) {
      bVar1 = 0;
    }
    else {
      bVar1 = 1;
    }
    pcVar3 = (char *)FUN_00108dc0(__n_00 + bVar1 + __n + 1);
    memcpy(pcVar3,param_1,__n);
    pcVar3[__n] = '/';
    memcpy(pcVar3 + bVar1 + __n,__src,__n_00);
    pcVar3[bVar1 + __n + __n_00] = '\0';
    param_1 = pcVar3;
  }
  return param_1;
}




// Function: lf_init @ 0x425e

void lf_init(undefined8 *param_1,undefined8 param_2)

{
  undefined8 uVar1;
  
  *param_1 = param_2;
  uVar1 = FUN_00108dc0(0x10001);
  param_1[3] = uVar1;
  param_1[2] = param_1[3];
  param_1[1] = param_1[2];
  *(undefined1 *)param_1[3] = 10;
  return;
}




// Function: lf_refill @ 0x42c0

long lf_refill(undefined8 *param_1)

{
  long lVar1;
  
  lVar1 = FUN_00104019(param_1[2],0x10000,*param_1);
  param_1[1] = param_1[2];
  param_1[3] = param_1[2] + lVar1;
  *(undefined1 *)param_1[3] = 10;
  FUN_001051e3();
  return lVar1;
}




// Function: lf_copy @ 0x432d

void lf_copy(long param_1,long param_2,undefined8 param_3)

{
  void *pvVar1;
  long lVar2;
  long local_28;
  long local_10;
  
  local_10 = *(long *)(param_1 + 8);
  local_28 = param_2;
  while( true ) {
    while( true ) {
      if (local_28 == 0) {
        FUN_00104082(local_10,*(long *)(param_1 + 8) - local_10,param_3);
        return;
      }
      pvVar1 = rawmemchr(*(void **)(param_1 + 8),10);
      *(void **)(param_1 + 8) = pvVar1;
      if (*(long *)(param_1 + 8) == *(long *)(param_1 + 0x18)) break;
      local_28 = local_28 + -1;
      *(long *)(param_1 + 8) = *(long *)(param_1 + 8) + 1;
    }
    FUN_00104082(local_10,*(long *)(param_1 + 0x18) - local_10,param_3);
    lVar2 = FUN_001042c0(param_1);
    if (lVar2 == 0) break;
    local_10 = *(long *)(param_1 + 8);
  }
  return;
}




// Function: lf_skip @ 0x4414

void lf_skip(long param_1,long param_2)

{
  void *pvVar1;
  long lVar2;
  long local_18;
  
  local_18 = param_2;
  do {
    while( true ) {
      if (local_18 == 0) {
        return;
      }
      pvVar1 = rawmemchr(*(void **)(param_1 + 8),10);
      *(void **)(param_1 + 8) = pvVar1;
      if (*(long *)(param_1 + 8) == *(long *)(param_1 + 0x18)) break;
      local_18 = local_18 + -1;
      *(long *)(param_1 + 8) = *(long *)(param_1 + 8) + 1;
    }
    lVar2 = FUN_001042c0(param_1);
  } while (lVar2 != 0);
  return;
}




// Function: lf_snarf @ 0x4493

undefined8 lf_snarf(long param_1,void *param_2,ulong param_3)

{
  void *__s;
  void *pvVar1;
  ulong __n;
  long lVar2;
  ulong local_40;
  void *local_38;
  
  local_40 = param_3;
  local_38 = param_2;
  while( true ) {
    __s = *(void **)(param_1 + 8);
    pvVar1 = rawmemchr(__s,10);
    __n = (long)pvVar1 - (long)__s;
    if (local_40 <= __n) {
      return 0;
    }
    memcpy(local_38,__s,__n);
    if (pvVar1 < *(void **)(param_1 + 0x18)) break;
    lVar2 = FUN_001042c0(param_1);
    if (lVar2 == 0) {
      if (__n != 0) {
        return 0;
      }
      return 0xffffffff;
    }
    local_38 = (void *)((long)local_38 + __n);
    local_40 = local_40 - __n;
  }
  *(undefined1 *)(__n + (long)local_38) = 0;
  *(long *)(param_1 + 8) = (long)pvVar1 + 1;
  return 1;
}




// Function: main @ 0x4573

undefined8 main(int param_1,undefined8 *param_2)

{
  byte bVar1;
  byte bVar2;
  char cVar3;
  int iVar4;
  __pid_t _Var5;
  undefined8 uVar6;
  int *piVar7;
  long in_FS_OFFSET;
  int local_bc;
  int local_b8;
  undefined4 local_b4;
  char *local_b0;
  undefined8 local_a8;
  undefined8 local_a0;
  undefined8 local_98;
  undefined8 local_90;
  undefined8 local_88;
  FILE *local_80;
  undefined1 local_78 [32];
  undefined1 local_58 [32];
  undefined1 local_38 [32];
  int local_18;
  int local_14;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  DAT_00112020 = 2;
  FUN_0010632f(*param_2);
  setlocale(6,"");
  bindtextdomain("diffutils","/usr/local/share/locale");
  textdomain("diffutils");
  FUN_0010626b(FUN_00103df5);
  FUN_001093c4();
  local_b0 = getenv("EDITOR");
  if (local_b0 != (char *)0x0) {
    PTR_DAT_00112010 = local_b0;
  }
  FUN_00104ebd(&DAT_0010d963);
LAB_00104879:
  local_b8 = getopt_long(param_1,param_2,"abBdEHiI:lo:stvw:WZ",&PTR_s_diff_program_001117a0,0);
  if (local_b8 == -1) {
    if (param_1 - optind != 2) {
      if (param_1 - optind < 2) {
        FUN_00103c09("missing operand after \'%s\'",param_2[(long)param_1 + -1]);
      }
      FUN_00103c09("extra operand \'%s\'",param_2[(long)optind + 2]);
    }
    if (DAT_001124e8 == (char *)0x0) {
      if (DAT_001124f0 != '\0') {
        FUN_00104ebd("--suppress-common-lines");
      }
      FUN_00104ebd(&DAT_0010da32);
      FUN_00104ebd(&DAT_0010da35);
      FUN_00104ebd(param_2[optind]);
      FUN_00104ebd(param_2[(long)optind + 1]);
      FUN_00104ebd(0);
      execvp(*DAT_001120a0,DAT_001120a0);
      FUN_00103eab(*DAT_001120a0);
    }
    bVar1 = FUN_00105e22(param_2[optind]);
    bVar2 = FUN_00105e22(param_2[(long)optind + 1]);
    if ((bVar2 & bVar1) != 0) {
      FUN_00103e69("both files to be compared are directories");
    }
    local_a8 = FUN_0010410d(param_2[optind],bVar1,param_2[(long)optind + 1]);
    local_a0 = FUN_00103fa3(local_a8,&DAT_0010da62);
    local_98 = FUN_0010410d(param_2[(long)optind + 1],bVar2,param_2[optind]);
    local_90 = FUN_00103fa3(local_98,&DAT_0010da62);
    local_88 = FUN_00103fa3(DAT_001124e8,&DAT_0010da64);
    FUN_00104ebd("--sdiff-merge-assist");
    FUN_00104ebd(&DAT_0010da35);
    FUN_00104ebd(param_2[optind]);
    FUN_00104ebd(param_2[(long)optind + 1]);
    FUN_00104ebd(0);
    FUN_00104fdd();
    iVar4 = pipe(&local_18);
    if (iVar4 != 0) {
      FUN_00103eab(&DAT_0010da7b);
    }
    DAT_001120b8 = fork();
    if (DAT_001120b8 < 0) {
      FUN_00103eab(&DAT_0010da80);
    }
    if (DAT_001120b8 != 0) {
      close(local_14);
      local_80 = fdopen(local_18,"r");
      if (local_80 == (FILE *)0x0) {
        FUN_00103eab("fdopen");
      }
      FUN_0010425e(local_38,local_80);
      FUN_0010425e(local_78,local_a0);
      FUN_0010425e(local_58,local_90);
      cVar3 = FUN_00105b03(local_38,local_78,local_a8,local_58,local_98,local_88);
      FUN_00103fe7(local_a0);
      FUN_00103fe7(local_90);
      FUN_00103fe7(local_88);
      local_b4 = 0;
      FUN_00103fe7(local_80);
      while (_Var5 = waitpid(DAT_001120b8,&local_bc,0), _Var5 < 0) {
        piVar7 = __errno_location();
        if (*piVar7 == 4) {
          FUN_001051e3();
        }
        else {
          FUN_00103eab("waitpid");
        }
      }
      DAT_001120b8 = 0;
      if (DAT_001120a8 != (char *)0x0) {
        unlink(DAT_001120a8);
        DAT_001120a8 = (char *)0x0;
      }
      if (cVar3 != '\x01') {
                    /* WARNING: Subroutine does not return */
        FUN_00103e3e();
      }
      FUN_00103ef1(local_b4,local_bc,1,*DAT_001120a0);
      FUN_0010510d(0);
      FUN_001051e3();
                    /* WARNING: Subroutine does not return */
      exit(local_bc >> 8 & 0xff);
    }
    if (DAT_00112450 != 1) {
      FUN_00104fa3(2,1);
    }
    FUN_00104fa3(0xd,0);
    close(local_18);
    if (local_14 != 1) {
      dup2(local_14,1);
      close(local_14);
    }
    execvp(*DAT_001120a0,DAT_001120a0);
    piVar7 = __errno_location();
    if (*piVar7 == 2) {
      iVar4 = 0x7f;
    }
    else {
      iVar4 = 0x7e;
    }
                    /* WARNING: Subroutine does not return */
    _exit(iVar4);
  }
  switch(local_b8) {
  case 0x42:
    FUN_00104ebd(&DAT_0010d96e);
    goto LAB_00104879;
  default:
    FUN_00103c09(0,0);
    goto LAB_00104879;
  case 0x45:
    FUN_00104ebd(&DAT_0010d974);
    goto LAB_00104879;
  case 0x48:
    FUN_00104ebd(&DAT_0010d977);
    goto LAB_00104879;
  case 0x49:
    FUN_00104ebd(&DAT_0010d97d);
    FUN_00104ebd(optarg);
    goto LAB_00104879;
  case 0x57:
    FUN_00104ebd(&DAT_0010d9b4);
    goto LAB_00104879;
  case 0x5a:
    FUN_00104ebd(&DAT_0010d9b7);
    goto LAB_00104879;
  case 0x61:
    FUN_00104ebd(&DAT_0010d968);
    goto LAB_00104879;
  case 0x62:
    FUN_00104ebd(&DAT_0010d96b);
    goto LAB_00104879;
  case 100:
    FUN_00104ebd(&DAT_0010d971);
    goto LAB_00104879;
  case 0x69:
    FUN_00104ebd(&DAT_0010d97a);
    goto LAB_00104879;
  case 0x6c:
    FUN_00104ebd("--left-column");
    goto LAB_00104879;
  case 0x6f:
    DAT_001124e8 = optarg;
    goto LAB_00104879;
  case 0x73:
    DAT_001124f0 = '\x01';
    goto LAB_00104879;
  case 0x74:
    FUN_00104ebd(&DAT_0010d98e);
    goto LAB_00104879;
  case 0x76:
    uVar6 = FUN_001068f0("Thomas Lord");
    FUN_00108b97(stdout,"sdiff","GNU diffutils",PTR_DAT_00112018,uVar6,0);
    FUN_00103c7f();
    break;
  case 0x77:
    FUN_00104ebd(&DAT_0010d9b1);
    FUN_00104ebd(optarg);
    goto LAB_00104879;
  case 0x80:
    *DAT_001120a0 = optarg;
    goto LAB_00104879;
  case 0x81:
    FUN_00103cd6();
    FUN_00103c7f();
    break;
  case 0x82:
    FUN_00104ebd("--strip-trailing-cr");
    goto LAB_00104879;
  case 0x83:
    goto switchD_0010467e_caseD_83;
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return 0;
switchD_0010467e_caseD_83:
  FUN_00104ebd("--tabsize");
  FUN_00104ebd(optarg);
  goto LAB_00104879;
}




// Function: diffarg @ 0x4ebd

void diffarg(undefined8 param_1)

{
  long lVar1;
  
  if (DAT_00112598 == DAT_001125a0) {
    if (DAT_001125a0 == 0) {
      DAT_001125a0 = 0x10;
    }
    else {
      if (0x7fffffffffffffe < DAT_001125a0) {
        FUN_00109384();
      }
      DAT_001125a0 = DAT_001125a0 * 2;
    }
    DAT_001120a0 = FUN_00108e0c(DAT_001120a0,DAT_001125a0 * 8);
  }
  lVar1 = DAT_00112598 * 8;
  DAT_00112598 = DAT_00112598 + 1;
  *(undefined8 *)(DAT_001120a0 + lVar1) = param_1;
  return;
}




// Function: catchsig @ 0x4f78

void catchsig(int param_1)

{
  if ((param_1 != 2) || (DAT_001124f1 != '\x01')) {
    DAT_001124f4 = param_1;
  }
  return;
}




// Function: signal_handler @ 0x4fa3

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void signal_handler(int param_1,undefined8 param_2)

{
  _DAT_00112500 = param_2;
  sigaction(param_1,(sigaction *)&DAT_00112500,(sigaction *)0x0);
  return;
}




// Function: trapsigs @ 0x4fdd

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void trapsigs(void)

{
  int local_c;
  
  _DAT_00112588 = 0x10000000;
  sigemptyset((sigset_t *)&DAT_00112508);
  for (local_c = 0; local_c < 7; local_c = local_c + 1) {
    sigaddset((sigset_t *)&DAT_00112508,*(int *)(&DAT_0010d020 + (long)local_c * 4));
  }
  for (local_c = 0; local_c < 7; local_c = local_c + 1) {
    sigaction(*(int *)(&DAT_0010d020 + (long)local_c * 4),(sigaction *)0x0,
              (sigaction *)(&DAT_001120c0 + (long)local_c * 0x98));
    if (*(long *)(&DAT_001120c0 + (long)local_c * 0x98) != 1) {
      FUN_00104fa3(*(undefined4 *)(&DAT_0010d020 + (long)local_c * 4),FUN_00104f78);
    }
  }
  signal(0x11,(__sighandler_t)0x0);
  DAT_001124f8 = 1;
  return;
}




// Function: untrapsig @ 0x510d

void untrapsig(int param_1)

{
  int local_c;
  
  if (DAT_001124f8 != '\0') {
    for (local_c = 0; local_c < 7; local_c = local_c + 1) {
      if (((param_1 == 0) || (param_1 == *(int *)(&DAT_0010d020 + (long)local_c * 4))) &&
         (*(long *)(&DAT_001120c0 + (long)local_c * 0x98) != 1)) {
        sigaction(*(int *)(&DAT_0010d020 + (long)local_c * 4),
                  (sigaction *)(&DAT_001120c0 + (long)local_c * 0x98),(sigaction *)0x0);
      }
    }
  }
  return;
}




// Function: checksigs @ 0x51e3

void checksigs(void)

{
  int __sig;
  
  __sig = DAT_001124f4;
  if (DAT_001124f4 != 0) {
    FUN_00103df5(0);
    FUN_0010510d(__sig);
    raise(__sig);
                    /* WARNING: Subroutine does not return */
    exit(2);
  }
  return;
}




// Function: give_help @ 0x5229

void give_help(void)

{
  undefined8 uVar1;
  
  uVar1 = gettext(
                 "ed:\tEdit then use both versions, each decorated with a header.\neb:\tEdit then use both versions.\nel or e1:\tEdit then use the left version.\ner or e2:\tEdit then use the right version.\ne:\tDiscard both versions then edit a new one.\nl or 1:\tUse the left version.\nr or 2:\tUse the right version.\ns:\tSilently include common lines.\nv:\tVerbosely include common lines.\nq:\tQuit.\n"
                 );
  fprintf(stderr,"%s",uVar1);
  return;
}




// Function: skip_white @ 0x5264

int skip_white(void)

{
  int iVar1;
  int iVar2;
  ushort **ppuVar3;
  undefined8 uVar4;
  
  while( true ) {
    iVar1 = getchar_unlocked();
    ppuVar3 = __ctype_b_loc();
    if ((((*ppuVar3)[iVar1] & 0x2000) == 0) || (iVar1 == 10)) break;
    FUN_001051e3();
  }
  iVar2 = ferror_unlocked(stdin);
  if (iVar2 != 0) {
    uVar4 = gettext("read failed");
    FUN_00103eab(uVar4);
  }
  return iVar1;
}




// Function: flush_line @ 0x52d6

void flush_line(void)

{
  int iVar1;
  undefined8 uVar2;
  
  do {
    iVar1 = getchar_unlocked();
    if (iVar1 == 10) break;
  } while (iVar1 != -1);
  iVar1 = ferror_unlocked(stdin);
  if (iVar1 != 0) {
    uVar2 = gettext("read failed");
    FUN_00103eab(uVar2);
  }
  return;
}




// Function: edit @ 0x5326

undefined8
edit(undefined8 param_1,undefined8 param_2,long param_3,long param_4,undefined8 param_5,
            undefined8 param_6,long param_7,long param_8,undefined8 param_9)

{
  bool bVar1;
  undefined1 *puVar2;
  int iVar3;
  __pid_t _Var4;
  undefined8 uVar5;
  int *piVar6;
  undefined1 *puVar7;
  long in_FS_OFFSET;
  int iStack_10078;
  int iStack_10074;
  int iStack_10070;
  int iStack_1006c;
  undefined4 uStack_10068;
  __pid_t _Stack_10064;
  long lStack_10060;
  long lStack_10058;
  long lStack_10050;
  long lStack_10048;
  long lStack_10040;
  char *pcStack_10038;
  char *pcStack_10030;
  undefined8 uStack_10028;
  undefined1 auStack_10018 [16];
  undefined1 auStack_10008 [65528];
  long local_10;
  
  puVar2 = &stack0xfffffffffffffff8;
  do {
    puVar7 = puVar2;
    *(undefined8 *)(puVar7 + -0x1000) = *(undefined8 *)(puVar7 + -0x1000);
    puVar2 = puVar7 + -0x1000;
  } while (puVar7 + -0x1000 != auStack_10008);
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
LAB_00105392:
  do {
    bVar1 = false;
LAB_0010555d:
    if (!bVar1) {
      *(undefined8 *)(puVar7 + -0x10c8) = 0x1053a8;
      iVar3 = putchar_unlocked(0x25);
      if (iVar3 != 0x25) {
        *(undefined8 *)(puVar7 + -0x10c8) = 0x1053bc;
        uVar5 = gettext("write failed");
        *(undefined8 *)(puVar7 + -0x10c8) = 0x1053c4;
        FUN_00103eab(uVar5);
      }
      *(undefined8 *)(puVar7 + -0x10c8) = 0x1053d3;
      FUN_001040d3(stdout);
      *(undefined8 *)(puVar7 + -0x10c8) = 0x1053d8;
      iStack_10074 = FUN_00105264();
      if (iStack_10074 == 0x76) {
LAB_0010546f:
        *(undefined8 *)(puVar7 + -0x10c8) = 0x105474;
        iVar3 = FUN_00105264();
        if (iVar3 == 10) {
          bVar1 = true;
        }
        else {
          *(undefined8 *)(puVar7 + -0x10c8) = 0x10547e;
          FUN_00105229();
          *(undefined8 *)(puVar7 + -0x10c8) = 0x105483;
          FUN_001052d6();
        }
        goto LAB_0010555d;
      }
      if ((0x76 < iStack_10074) || (0x73 < iStack_10074)) goto LAB_00105552;
      if ((0x70 < iStack_10074) || (iStack_10074 == 0x6c)) goto LAB_0010546f;
      if (0x6c < iStack_10074) goto LAB_00105552;
      if (iStack_10074 != 0x65) {
        if ((0x65 < iStack_10074) || (0x32 < iStack_10074)) goto LAB_00105552;
        if (0x30 < iStack_10074) goto LAB_0010546f;
        if (iStack_10074 == -1) {
          *(undefined8 *)(puVar7 + -0x10c8) = 0x10553b;
          iVar3 = feof_unlocked(stdin);
          if (iVar3 != 0) {
            bVar1 = true;
            iStack_10074 = 0x71;
            goto LAB_0010555d;
          }
LAB_00105552:
          *(undefined8 *)(puVar7 + -0x10c8) = 0x105557;
          FUN_001052d6();
        }
        else if (iStack_10074 != 10) goto LAB_00105552;
        *(undefined8 *)(puVar7 + -0x10c8) = 0x10555c;
        FUN_00105229();
        goto LAB_0010555d;
      }
      *(undefined8 *)(puVar7 + -0x10c8) = 0x105499;
      iStack_10070 = FUN_00105264();
      if (iStack_10070 < 0x73) {
        if (iStack_10070 < 0x62) {
          if (iStack_10070 != 10) {
            if ((9 < iStack_10070) && (iStack_10070 - 0x31U < 2)) goto LAB_001054f6;
            goto LAB_0010551e;
          }
LAB_0010552a:
          bVar1 = true;
        }
        else {
          if ((0x10405UL >> ((char)iStack_10070 + 0x9eU & 0x3f) & 1) == 0) goto LAB_0010551e;
LAB_001054f6:
          *(undefined8 *)(puVar7 + -0x10c8) = 0x1054fb;
          iVar3 = FUN_00105264();
          if (iVar3 == 10) goto LAB_0010552a;
          *(undefined8 *)(puVar7 + -0x10c8) = 0x105505;
          FUN_00105229();
          *(undefined8 *)(puVar7 + -0x10c8) = 0x10550a;
          FUN_001052d6();
        }
      }
      else {
LAB_0010551e:
        *(undefined8 *)(puVar7 + -0x10c8) = 0x105523;
        FUN_00105229();
        *(undefined8 *)(puVar7 + -0x10c8) = 0x105528;
        FUN_001052d6();
      }
      goto LAB_0010555d;
    }
    if (0x76 < iStack_10074) {
switchD_001055ce_caseD_66:
      *(undefined8 *)(puVar7 + -0x10c8) = 0x105ae7;
      FUN_00105229();
      goto LAB_00105392;
    }
    if (iStack_10074 < 0x65) {
      if (iStack_10074 == 0x31) goto switchD_001055ce_caseD_6c;
      if (iStack_10074 == 0x32) goto switchD_001055ce_caseD_72;
      goto switchD_001055ce_caseD_66;
    }
    switch(iStack_10074) {
    case 0x65:
      goto switchD_001055ce_caseD_65;
    default:
      goto switchD_001055ce_caseD_66;
    case 0x6c:
switchD_001055ce_caseD_6c:
      *(undefined8 *)(puVar7 + -0x10c8) = 0x1055f1;
      FUN_0010432d(param_1,param_4,param_9);
      *(undefined8 *)(puVar7 + -0x10c8) = 0x105607;
      FUN_00104414(param_5,param_8);
      uVar5 = 1;
      goto LAB_00105aed;
    case 0x71:
      uVar5 = 0;
      goto LAB_00105aed;
    case 0x72:
switchD_001055ce_caseD_72:
      *(undefined8 *)(puVar7 + -0x10c8) = 0x10562e;
      FUN_0010432d(param_5,param_8,param_9);
      *(undefined8 *)(puVar7 + -0x10c8) = 0x105647;
      FUN_00104414(param_1,param_4);
      uVar5 = 1;
      goto LAB_00105aed;
    case 0x73:
      DAT_001124f0 = 1;
      break;
    case 0x76:
      DAT_001124f0 = 0;
    }
  } while( true );
switchD_001055ce_caseD_65:
  if (DAT_001120a8 == (char *)0x0) {
    *(undefined8 *)(puVar7 + -0x10c8) = 0x1056a6;
    iStack_1006c = FUN_00105e9b();
    if (iStack_1006c < 0) {
      *(undefined8 *)(puVar7 + -0x10c8) = 0x1056c4;
      FUN_00103eab("mkstemp");
    }
    *(undefined8 *)(puVar7 + -0x10c8) = 0x1056db;
    DAT_001120b0 = fdopen(iStack_1006c,"w");
  }
  else {
    *(undefined8 *)(puVar7 + -0x10c8) = 0x105698;
    DAT_001120b0 = fopen(DAT_001120a8,"w");
  }
  if (DAT_001120b0 == (FILE *)0x0) {
    *(undefined8 *)(puVar7 + -0x10c8) = 0x1056fd;
    FUN_00103eab(DAT_001120a8);
  }
  if (iStack_10070 == 0x6c) {
LAB_001057ec:
    *(undefined8 *)(puVar7 + -0x10c8) = 0x10580c;
    FUN_0010432d(param_1,param_4,DAT_001120b0);
  }
  else {
    if (iStack_10070 < 0x6d) {
      if (iStack_10070 == 100) {
        if (param_4 != 0) {
          lStack_10058 = param_4 + param_3 + -1;
          lStack_10060 = param_3;
          if (param_4 == 1) {
            *(undefined8 *)(puVar7 + -0x10c8) = 0x1057b7;
            fprintf(DAT_001120b0,"--- %s %ld\n",param_2,param_3);
          }
          else {
            *(undefined8 *)(puVar7 + -0x10c8) = 0x1057ec;
            fprintf(DAT_001120b0,"--- %s %ld,%ld\n",param_2,param_3,lStack_10058);
          }
        }
      }
      else if ((100 < iStack_10070) || ((iStack_10070 != 0x31 && (iStack_10070 != 0x62))))
      goto LAB_0010580e;
      goto LAB_001057ec;
    }
LAB_0010580e:
    *(undefined8 *)(puVar7 + -0x10c8) = 0x105827;
    FUN_00104414(param_1,param_4);
  }
  if (iStack_10070 != 0x72) {
    if (0x72 < iStack_10070) {
LAB_00105927:
      *(undefined8 *)(puVar7 + -0x10c8) = 0x10593d;
      FUN_00104414(param_5,param_8);
      goto LAB_0010593e;
    }
    if (iStack_10070 == 100) {
      if (param_8 != 0) {
        lStack_10050 = param_7;
        lStack_10048 = param_8 + param_7 + -1;
        if (param_8 == 1) {
          *(undefined8 *)(puVar7 + -0x10c8) = 0x1058d3;
          fprintf(DAT_001120b0,"+++ %s %ld\n",param_6,param_7);
        }
        else {
          *(undefined8 *)(puVar7 + -0x10c8) = 0x105908;
          fprintf(DAT_001120b0,"+++ %s %ld,%ld\n",param_6,param_7,lStack_10048);
        }
      }
    }
    else if ((100 < iStack_10070) || ((iStack_10070 != 0x32 && (iStack_10070 != 0x62))))
    goto LAB_00105927;
  }
  *(undefined8 *)(puVar7 + -0x10c8) = 0x105925;
  FUN_0010432d(param_5,param_8,DAT_001120b0);
LAB_0010593e:
  *(undefined8 *)(puVar7 + -0x10c8) = 0x10594d;
  FUN_00103fe7(DAT_001120b0);
  uStack_10068 = 0;
  DAT_001124f1 = 1;
  *(undefined8 *)(puVar7 + -0x10c8) = 0x105963;
  FUN_001051e3();
  pcStack_10038 = PTR_DAT_00112010;
  pcStack_10030 = DAT_001120a8;
  uStack_10028 = 0;
  *(undefined8 *)(puVar7 + -0x10c8) = 0x10598f;
  _Stack_10064 = fork();
  if (_Stack_10064 == 0) {
    *(undefined8 *)(puVar7 + -0x10c8) = 0x1059b7;
    execvp(PTR_DAT_00112010,&pcStack_10038);
    *(undefined8 *)(puVar7 + -0x10c8) = 0x1059bc;
    piVar6 = __errno_location();
    if (*piVar6 == 2) {
      iVar3 = 0x7f;
    }
    else {
      iVar3 = 0x7e;
    }
                    /* WARNING: Subroutine does not return */
    *(undefined8 *)(puVar7 + -0x10c8) = 0x1059d6;
    _exit(iVar3);
  }
  if (-1 < _Stack_10064) goto LAB_00105a10;
  *(undefined8 *)(puVar7 + -0x10c8) = 0x1059ee;
  FUN_00103eab(&DAT_0010da80);
  do {
    *(undefined8 *)(puVar7 + -0x10c8) = 0x1059f3;
    piVar6 = __errno_location();
    if (*piVar6 == 4) {
      *(undefined8 *)(puVar7 + -0x10c8) = 0x1059ff;
      FUN_001051e3();
    }
    else {
      *(undefined8 *)(puVar7 + -0x10c8) = 0x105a10;
      FUN_00103eab("waitpid");
    }
LAB_00105a10:
    *(undefined8 *)(puVar7 + -0x10c8) = 0x105a2c;
    _Var4 = waitpid(_Stack_10064,&iStack_10078,0);
  } while (_Var4 < 0);
  DAT_001124f1 = 0;
  *(undefined8 *)(puVar7 + -0x10c8) = 0x105a59;
  FUN_00103ef1(uStack_10068,iStack_10078,0,PTR_DAT_00112010);
  *(undefined8 *)(puVar7 + -0x10c8) = 0x105a72;
  DAT_001120b0 = (FILE *)FUN_00103fa3(DAT_001120a8,&DAT_0010da62);
  while( true ) {
    *(undefined8 *)(puVar7 + -0x10c8) = 0x105abb;
    lStack_10040 = FUN_00104019(auStack_10018,0x10000,DAT_001120b0);
    if (lStack_10040 == 0) break;
    *(undefined8 *)(puVar7 + -0x10c8) = 0x105a80;
    FUN_001051e3();
    *(undefined8 *)(puVar7 + -0x10c8) = 0x105aa0;
    FUN_00104082(auStack_10018,lStack_10040,param_9);
  }
  *(undefined8 *)(puVar7 + -0x10c8) = 0x105adb;
  FUN_00103fe7(DAT_001120b0);
  uVar5 = 1;
LAB_00105aed:
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    *(undefined8 *)(puVar7 + -0x10c8) = 0x105b01;
    __stack_chk_fail();
  }
  return uVar5;
}




// Function: interact @ 0x5b03

bool interact(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                 undefined8 param_5,undefined8 param_6)

{
  char cVar1;
  int iVar2;
  int *piVar3;
  long in_FS_OFFSET;
  bool bVar4;
  char *local_150;
  long local_148;
  long local_140;
  intmax_t local_138;
  intmax_t local_130;
  long local_128;
  long local_120;
  char local_118;
  char acStack_117 [263];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_148 = 1;
  local_140 = 1;
LAB_00105b61:
  do {
    iVar2 = FUN_00104493(param_1,&local_118,0x100);
    if (iVar2 < 1) {
      bVar4 = iVar2 != 0;
LAB_00105e0c:
      if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
        __stack_chk_fail();
      }
      return bVar4;
    }
    FUN_001051e3();
    if (local_118 == ' ') {
      puts(acStack_117);
      goto LAB_00105b61;
    }
    piVar3 = __errno_location();
    *piVar3 = 0;
    local_138 = strtoimax(acStack_117,&local_150,10);
    if (local_138 < 0) {
LAB_00105c19:
      FUN_00103e69(&local_118);
    }
    else {
      piVar3 = __errno_location();
      if ((*piVar3 != 0) || (*local_150 != ',')) goto LAB_00105c19;
    }
    local_130 = local_138;
    local_138 = strtoimax(local_150 + 1,&local_150,10);
    if (local_138 < 0) {
LAB_00105c82:
      FUN_00103e69(&local_118);
    }
    else {
      piVar3 = __errno_location();
      if ((*piVar3 != 0) || (*local_150 != '\0')) goto LAB_00105c82;
    }
    local_128 = local_138;
    local_120 = local_138;
    if (local_138 <= local_130) {
      local_120 = local_130;
    }
    if (local_118 == 'c') {
      FUN_0010432d(param_1,local_120,stdout);
      cVar1 = FUN_00105326(param_2,param_3,local_148,local_130,param_4,param_5,local_140,local_128,
                           param_6);
      if (cVar1 != '\x01') {
        bVar4 = false;
        goto LAB_00105e0c;
      }
    }
    else if (local_118 == 'i') {
      if (DAT_001124f0 == '\0') {
        FUN_0010432d(param_1,local_120,stdout);
      }
      else {
        FUN_00104414(param_1,local_120);
      }
      FUN_0010432d(param_2,local_130,param_6);
      FUN_00104414(param_4,local_128);
    }
    else {
      FUN_00103e69(&local_118);
    }
    local_148 = local_148 + local_130;
    local_140 = local_140 + local_128;
  } while( true );
}




// Function: diraccess @ 0x5e22

undefined4 diraccess(char *param_1)

{
  int iVar1;
  undefined4 uVar2;
  long in_FS_OFFSET;
  stat local_a8;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = stat(param_1,&local_a8);
  if ((iVar1 == 0) && ((local_a8.st_mode & 0xf000) == 0x4000)) {
    uVar2 = 1;
  }
  else {
    uVar2 = 0;
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar2;
}




// Function: temporary_file @ 0x5e9b

int temporary_file(void)

{
  int iVar1;
  char *__s;
  size_t sVar2;
  char *__s_00;
  
  __s = getenv("TMPDIR");
  if (__s == (char *)0x0) {
    __s = "/tmp";
  }
  sVar2 = strlen(__s);
  __s_00 = (char *)FUN_00108dc0(sVar2 + 0xd);
  sprintf(__s_00,"%s/sdiffXXXXXX",__s);
  iVar1 = mkstemp(__s_00);
  if (-1 < iVar1) {
    DAT_001120a8 = __s_00;
  }
  return iVar1;
}



