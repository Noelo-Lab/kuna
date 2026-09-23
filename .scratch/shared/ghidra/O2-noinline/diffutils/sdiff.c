// Function: main @ 0x3b60

undefined8 main(int param_1,undefined8 *param_2)

{
  char **ppcVar1;
  char cVar2;
  char cVar3;
  int iVar4;
  __pid_t _Var5;
  char *pcVar6;
  long lVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  FILE *pFVar13;
  int *piVar14;
  long in_FS_OFFSET;
  uint local_ac;
  undefined1 local_a8 [32];
  undefined1 local_88 [32];
  undefined1 local_68 [32];
  int local_48;
  int local_44;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  DAT_0010f020 = 2;
  FUN_00105960(*param_2);
  setlocale(6,"");
  bindtextdomain("diffutils","/usr/local/share/locale");
  textdomain("diffutils");
  FUN_001058a0(FUN_00104300);
  FUN_00107bf0();
  pcVar6 = getenv("EDITOR");
  if (pcVar6 != (char *)0x0) {
    PTR_s_ed_0010a997_9_0010f010 = pcVar6;
  }
  FUN_001045b0(&DAT_0010aa5a);
LAB_00103c20:
  iVar4 = getopt_long(param_1,param_2,"abBdEHiI:lo:stvw:WZ",&PTR_s_diff_program_0010e860,0);
  if (iVar4 != -1) {
    switch(iVar4) {
    case 0x42:
      FUN_001045b0(&DAT_0010aa1c);
      goto LAB_00103c20;
    default:
      goto switchD_00103c4f_caseD_43;
    case 0x45:
      FUN_001045b0(&DAT_0010aa22);
      goto LAB_00103c20;
    case 0x48:
      FUN_001045b0(&DAT_0010aa25);
      goto LAB_00103c20;
    case 0x49:
      FUN_001045b0(&DAT_0010aa2b);
      FUN_001045b0(optarg);
      goto LAB_00103c20;
    case 0x57:
      FUN_001045b0(&DAT_0010aa62);
      goto LAB_00103c20;
    case 0x5a:
      FUN_001045b0(&DAT_0010aa65);
      goto LAB_00103c20;
    case 0x61:
      FUN_001045b0(&DAT_0010aa16);
      goto LAB_00103c20;
    case 0x62:
      FUN_001045b0(&DAT_0010aa19);
      goto LAB_00103c20;
    case 100:
      FUN_001045b0(&DAT_0010aa1f);
      goto LAB_00103c20;
    case 0x69:
      FUN_001045b0(&DAT_0010aa28);
      goto LAB_00103c20;
    case 0x6c:
      FUN_001045b0("--left-column");
      goto LAB_00103c20;
    case 0x6f:
      DAT_0010f168 = optarg;
      goto LAB_00103c20;
    case 0x73:
      DAT_0010f161 = '\x01';
      goto LAB_00103c20;
    case 0x74:
      FUN_001045b0(&DAT_0010aa3c);
      goto LAB_00103c20;
    case 0x76:
      uVar8 = FUN_00105c70("Thomas Lord");
      FUN_00107690(stdout,&DAT_0010aa59,"GNU diffutils",PTR_DAT_0010f018,uVar8,0);
      FUN_00104a10();
      break;
    case 0x77:
      FUN_001045b0(&DAT_0010aa5f);
      FUN_001045b0(optarg);
      goto LAB_00103c20;
    case 0x80:
      *DAT_0010f5c0 = optarg;
      goto LAB_00103c20;
    case 0x81:
      FUN_00104380();
      FUN_00104a10();
      break;
    case 0x82:
      FUN_001045b0("--strip-trailing-cr");
      goto LAB_00103c20;
    case 0x83:
      goto switchD_00103c4f_caseD_83;
    }
    if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
      return 0;
    }
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  lVar7 = (long)optind;
  if (param_1 - optind != 2) {
    if (param_1 - optind < 2) {
      FUN_001044c0("missing operand after \'%s\'",param_2[(long)param_1 + -1]);
switchD_00103c4f_caseD_43:
      FUN_001044c0(0,0);
      goto LAB_0010409f;
    }
    lVar7 = FUN_001044c0("extra operand \'%s\'",param_2[lVar7 + 2]);
  }
  if (DAT_0010f168 == (char *)0x0) {
    if (DAT_0010f161 != '\0') {
      FUN_001045b0("--suppress-common-lines");
    }
    FUN_001045b0(&DAT_0010aae0);
    FUN_001045b0(&DAT_0010aae3);
    FUN_001045b0(param_2[optind]);
    FUN_001045b0(param_2[(long)optind + 1]);
    FUN_001045b0(0);
    ppcVar1 = DAT_0010f5c0;
    execvp(*DAT_0010f5c0,DAT_0010f5c0);
                    /* WARNING: Subroutine does not return */
    FUN_001049e0(*ppcVar1);
  }
  cVar2 = FUN_00104520(param_2[lVar7]);
  cVar3 = FUN_00104520(param_2[(long)optind + 1]);
  if ((cVar2 != '\0') && (cVar3 != '\0')) {
    FUN_001048d0("both files to be compared are directories",cVar2);
  }
  uVar8 = FUN_00104900(param_2[optind]);
  uVar9 = FUN_00104a60(uVar8,"r");
  uVar10 = FUN_00104900(param_2[(long)optind + 1],cVar3,param_2[optind]);
  uVar11 = FUN_00104a60(uVar10,"r");
  uVar12 = FUN_00104a60(DAT_0010f168,&DAT_0010aa63);
  FUN_001045b0("--sdiff-merge-assist");
  FUN_001045b0(&DAT_0010aae3);
  FUN_001045b0(param_2[optind]);
  FUN_001045b0(param_2[(long)optind + 1]);
  FUN_001045b0(0);
  FUN_001046f0();
  iVar4 = pipe(&local_48);
  if (iVar4 != 0) {
                    /* WARNING: Subroutine does not return */
    FUN_001049e0(&DAT_0010aafb);
  }
  DAT_0010f5a8 = fork();
  if (DAT_0010f5a8 < 0) {
                    /* WARNING: Subroutine does not return */
    FUN_001049e0(&DAT_0010a9ea);
  }
  if (DAT_0010f5a8 == 0) {
    if (DAT_0010f510 != 1) {
      FUN_00104640(2,1);
    }
    FUN_00104640(0xd,0);
    close(local_48);
    if (local_44 != 1) {
      dup2(local_44,1);
      close(local_44);
    }
    execvp(*DAT_0010f5c0,DAT_0010f5c0);
    piVar14 = __errno_location();
                    /* WARNING: Subroutine does not return */
    _exit((*piVar14 == 2) + 0x7e);
  }
  close(local_44);
  pFVar13 = fdopen(local_48,"r");
  if (pFVar13 == (FILE *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_001049e0("fdopen");
  }
  FUN_00104580(local_68,pFVar13);
  FUN_00104580(local_a8,uVar9);
  FUN_00104580(local_88,uVar11);
  cVar2 = FUN_00105480(local_68,local_a8,uVar8,local_88,uVar10,uVar12);
  FUN_00104b40(uVar9);
  FUN_00104b40(uVar11);
  FUN_00104b40(uVar12);
  FUN_00104b40(pFVar13);
  while (_Var5 = waitpid(DAT_0010f5a8,(int *)&local_ac,0), _Var5 < 0) {
    piVar14 = __errno_location();
    if (*piVar14 != 4) {
                    /* WARNING: Subroutine does not return */
      FUN_001049e0("waitpid");
    }
    FUN_00104870();
  }
  DAT_0010f5a8 = 0;
  if (DAT_0010f5b8 != (char *)0x0) {
    unlink(DAT_0010f5b8);
    DAT_0010f5b8 = (char *)0x0;
  }
  if (cVar2 == '\0') {
                    /* WARNING: Subroutine does not return */
    FUN_001048a0();
  }
LAB_0010409f:
  FUN_00104e40(local_ac,1,*DAT_0010f5c0);
  FUN_00104660(0);
  FUN_00104870();
                    /* WARNING: Subroutine does not return */
  exit(local_ac >> 8 & 0xff);
switchD_00103c4f_caseD_83:
  FUN_001045b0("--tabsize");
  FUN_001045b0(optarg);
  goto LAB_00103c20;
}




// Function: catchsig @ 0x42e0

void catchsig(int param_1)

{
  if ((param_1 == 2) && (DAT_0010f160 != '\0')) {
    return;
  }
  DAT_0010f15c = param_1;
  return;
}




// Function: cleanup @ 0x4300

void cleanup(void)

{
  if (0 < DAT_0010f5a8) {
    kill(DAT_0010f5a8,0xd);
    if (DAT_0010f5b8 != (char *)0x0) {
      unlink(DAT_0010f5b8);
      return;
    }
    return;
  }
  if (DAT_0010f5b8 != (char *)0x0) {
    unlink(DAT_0010f5b8);
    return;
  }
  return;
}




// Function: usage @ 0x4380

void usage(void)

{
  FILE *__stream;
  char cVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  char *pcVar4;
  undefined **ppuVar5;
  
  uVar3 = DAT_0011f620;
  ppuVar5 = &PTR_s__o____output_FILE_operate_intera_0010e780;
  uVar2 = dcgettext(0,"Usage: %s [OPTION]... FILE1 FILE2\n",5);
  __printf_chk(1,uVar2,uVar3);
  uVar3 = dcgettext(0,"Side-by-side merge of differences between FILE1 and FILE2.",5);
  __printf_chk(1,&DAT_0010a949,uVar3);
  __stream = stdout;
  pcVar4 = (char *)dcgettext(0,
                             "Mandatory arguments to long options are mandatory for short options too.\n"
                             ,5);
  fputs_unlocked(pcVar4,__stream);
  cVar1 = '-';
  pcVar4 = "-o, --output=FILE            operate interactively, sending output to FILE";
  while( true ) {
    if (cVar1 == '\0') {
      putchar_unlocked(10);
      pcVar4 = ppuVar5[1];
    }
    else {
      uVar3 = dcgettext(0,pcVar4,5);
      __printf_chk(1,"  %s\n",uVar3);
      pcVar4 = ppuVar5[1];
    }
    if (pcVar4 == (char *)0x0) break;
    ppuVar5 = ppuVar5 + 1;
    cVar1 = *pcVar4;
  }
  uVar3 = dcgettext(0,"Exit status is 0 if inputs are the same, 1 if different, 2 if trouble.",5);
  uVar2 = dcgettext(0,"If a FILE is \'-\', read standard input.",5);
  __printf_chk(1,"\n%s\n%s\n",uVar2,uVar3);
  FUN_00107740();
  return;
}




// Function: try_help @ 0x44c0

undefined4 try_help(long param_1,undefined8 param_2)

{
  int iVar1;
  undefined4 uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  char *__file;
  long in_FS_OFFSET;
  stat sStack_b0;
  long lStack_18;
  undefined8 uStack_10;
  
  if (param_1 != 0) {
    uStack_10 = 0x1044d9;
    uVar3 = dcgettext(0,param_1,5);
    uStack_10 = 0x1044ea;
    error(0,0,uVar3,param_2);
  }
  uVar3 = DAT_0011f620;
  uStack_10 = 0x104504;
  uVar4 = dcgettext(0,"Try \'%s --help\' for more information.",5);
  __file = (char *)0x2;
  uStack_10 = 0x104518;
  error(2,0,uVar4,uVar3);
  lStack_18 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = stat(__file,&sStack_b0);
  uVar2 = 0;
  if (iVar1 == 0) {
    uVar2 = CONCAT31((int3)((sStack_b0.st_mode & 0xf000) >> 8),
                     (sStack_b0.st_mode & 0xf000) == 0x4000);
  }
  if (lStack_18 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: diraccess @ 0x4520

undefined4 diraccess(char *param_1)

{
  int iVar1;
  undefined4 uVar2;
  long in_FS_OFFSET;
  stat sStack_a8;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = stat(param_1,&sStack_a8);
  uVar2 = 0;
  if (iVar1 == 0) {
    uVar2 = CONCAT31((int3)((sStack_a8.st_mode & 0xf000) >> 8),
                     (sStack_a8.st_mode & 0xf000) == 0x4000);
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: lf_init @ 0x4580

void lf_init(undefined8 *param_1,undefined8 param_2)

{
  undefined1 *puVar1;
  
  *param_1 = param_2;
  puVar1 = (undefined1 *)FUN_00107800(0x10001);
  param_1[3] = puVar1;
  param_1[2] = puVar1;
  param_1[1] = puVar1;
  *puVar1 = 10;
  return;
}




// Function: diffarg @ 0x45b0

void diffarg(undefined8 param_1)

{
  long lVar1;
  
  if (DAT_0010f0a8 == DAT_0010f0a0) {
    if (DAT_0010f0a8 == 0) {
      DAT_0010f0a0 = 0x10;
      lVar1 = 0x80;
    }
    else {
      if (0x7fffffffffffffe < DAT_0010f0a8) {
                    /* WARNING: Subroutine does not return */
        FUN_00107bb0();
      }
      DAT_0010f0a0 = DAT_0010f0a8 * 2;
      lVar1 = DAT_0010f0a8 << 4;
    }
    DAT_0010f5c0 = FUN_00107850(DAT_0010f5c0,lVar1);
  }
  *(undefined8 *)(DAT_0010f5c0 + DAT_0010f0a8 * 8) = param_1;
  DAT_0010f0a8 = DAT_0010f0a8 + 1;
  return;
}




// Function: signal_handler @ 0x4640

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void signal_handler(int param_1,undefined8 param_2)

{
  _DAT_0010f0c0 = param_2;
  sigaction(param_1,(sigaction *)&DAT_0010f0c0,(sigaction *)0x0);
  return;
}




// Function: untrapsig @ 0x4660

void untrapsig(int param_1)

{
  int __sig;
  sigaction *__act;
  sigaction *psVar1;
  int *piVar2;
  
  if (DAT_0010f158 == '\0') {
    return;
  }
  piVar2 = &DAT_0010ad30;
  __act = (sigaction *)&DAT_0010f180;
  do {
    while (((param_1 == 0 || (*piVar2 == param_1)) &&
           ((__act->__sigaction_handler).sa_handler != (__sighandler_t)0x1))) {
      __sig = *piVar2;
      psVar1 = __act + 1;
      piVar2 = piVar2 + 1;
      sigaction(__sig,__act,(sigaction *)0x0);
      __act = psVar1;
      if (psVar1 == (sigaction *)&DAT_0010f5a8) {
        return;
      }
    }
    __act = __act + 1;
    piVar2 = piVar2 + 1;
  } while (__act != (sigaction *)&DAT_0010f5a8);
  return;
}




// Function: trapsigs @ 0x46f0

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void trapsigs(void)

{
  int *piVar1;
  sigaction *__oact;
  int *piVar2;
  int iVar3;
  
  piVar2 = &DAT_0010ad34;
  piVar1 = &DAT_0010ad34;
  _DAT_0010f148 = 0x10000000;
  sigemptyset((sigset_t *)&DAT_0010f0c8);
  iVar3 = 1;
  while( true ) {
    sigaddset((sigset_t *)&DAT_0010f0c8,iVar3);
    if (piVar1 == (int *)&DAT_0010ad4c) break;
    iVar3 = *piVar1;
    piVar1 = piVar1 + 1;
  }
  __oact = (sigaction *)&DAT_0010f180;
  iVar3 = 1;
  while( true ) {
    sigaction(iVar3,(sigaction *)0x0,__oact);
    if ((__oact->__sigaction_handler).sa_handler != (__sighandler_t)0x1) {
      FUN_00104640(iVar3);
    }
    __oact = __oact + 1;
    if (__oact == (sigaction *)&DAT_0010f5a8) break;
    iVar3 = *piVar2;
    piVar2 = piVar2 + 1;
  }
  signal(0x11,(__sighandler_t)0x0);
  DAT_0010f158 = 1;
  return;
}




// Function: temporary_file @ 0x47b0

void temporary_file(void)

{
  int iVar1;
  char *__s;
  size_t sVar2;
  char *__template;
  
  __s = getenv("TMPDIR");
  if (__s == (char *)0x0) {
    __s = "/tmp";
  }
  sVar2 = strlen(__s);
  __template = (char *)FUN_00107800(sVar2 + 0xd);
  __sprintf_chk(__template,1,0xffffffffffffffff,"%s/sdiffXXXXXX",__s);
  iVar1 = mkstemp(__template);
  if (-1 < iVar1) {
    DAT_0010f5b8 = __template;
  }
  return;
}




// Function: give_help @ 0x4830

void give_help(void)

{
  undefined8 uVar1;
  
  uVar1 = dcgettext(0,
                    "ed:\tEdit then use both versions, each decorated with a header.\neb:\tEdit then use both versions.\nel or e1:\tEdit then use the left version.\ner or e2:\tEdit then use the right version.\ne:\tDiscard both versions then edit a new one.\nl or 1:\tUse the left version.\nr or 2:\tUse the right version.\ns:\tSilently include common lines.\nv:\tVerbosely include common lines.\nq:\tQuit.\n"
                    ,5);
  __fprintf_chk(stderr,1,&DAT_0010a977,uVar1);
  return;
}




// Function: checksigs @ 0x4870

void checksigs(void)

{
  int __sig;
  
  __sig = DAT_0010f15c;
  if (DAT_0010f15c == 0) {
    return;
  }
  FUN_00104300(0);
  FUN_00104660(__sig);
  raise(__sig);
                    /* WARNING: Subroutine does not return */
  exit(2);
}




// Function: exiterr @ 0x48a0

void exiterr(void)

{
  FUN_00104300(0);
  FUN_00104660(0);
  FUN_00104870();
                    /* WARNING: Subroutine does not return */
  exit(2);
}




// Function: fatal @ 0x48d0

void fatal(undefined8 param_1)

{
  undefined8 uVar1;
  
  uVar1 = dcgettext(0,param_1,5);
  error(0,0,&DAT_0010a977,uVar1);
                    /* WARNING: Subroutine does not return */
  FUN_001048a0();
}




// Function: expand_name @ 0x4900

char * expand_name(char *param_1,char param_2,undefined8 param_3)

{
  long lVar1;
  int iVar2;
  void *__src;
  size_t __n;
  size_t __n_00;
  char *pcVar3;
  int *piVar4;
  ulong uVar5;
  
  iVar2 = strcmp(param_1,"-");
  if (iVar2 != 0) {
    pcVar3 = param_1;
    if (param_2 != '\0') {
      uVar5 = 0;
      __src = (void *)FUN_00105690(param_3);
      __n = strlen(param_1);
      __n_00 = FUN_001056f0(__src);
      pcVar3 = (char *)FUN_00105690(param_1);
      if (*pcVar3 != '\0') {
        uVar5 = (ulong)(param_1[__n - 1] != '/');
      }
      lVar1 = uVar5 + __n + __n_00;
      pcVar3 = (char *)FUN_00107800(lVar1 + 1);
      memcpy(pcVar3,param_1,__n);
      pcVar3[__n] = '/';
      memcpy(pcVar3 + uVar5 + __n,__src,__n_00);
      pcVar3[lVar1] = '\0';
    }
    return pcVar3;
  }
  pcVar3 = "cannot interactively merge standard input";
  FUN_001048d0("cannot interactively merge standard input");
  piVar4 = __errno_location();
  iVar2 = *piVar4;
  FUN_00104870();
  error(0,iVar2,&DAT_0010a977,pcVar3);
                    /* WARNING: Subroutine does not return */
  FUN_001048a0();
}




// Function: perror_fatal @ 0x49e0

void perror_fatal(undefined8 param_1)

{
  int iVar1;
  int *piVar2;
  
  piVar2 = __errno_location();
  iVar1 = *piVar2;
  FUN_00104870();
  error(0,iVar1,&DAT_0010a977,param_1);
                    /* WARNING: Subroutine does not return */
  FUN_001048a0();
}




// Function: check_stdout @ 0x4a10

void check_stdout(void)

{
  undefined8 uVar1;
  int iVar2;
  
  iVar2 = ferror_unlocked(stdout);
  if (iVar2 == 0) {
    iVar2 = fclose(stdout);
    if (iVar2 == 0) {
      return;
    }
  }
  else {
    FUN_001048d0("write failed");
  }
  uVar1 = dcgettext(0,"standard output",5);
                    /* WARNING: Subroutine does not return */
  FUN_001049e0(uVar1);
}




// Function: ck_fopen @ 0x4a60

void ck_fopen(char *param_1,char *param_2)

{
  FILE *pFVar1;
  
  pFVar1 = fopen(param_1,param_2);
  if (pFVar1 != (FILE *)0x0) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  FUN_001049e0(param_1);
}




// Function: ck_fwrite @ 0x4a80

void ck_fwrite(void *param_1,size_t param_2,FILE *param_3)

{
  size_t sVar1;
  undefined8 uVar2;
  
  sVar1 = fwrite_unlocked(param_1,1,param_2,param_3);
  if (sVar1 == param_2) {
    return;
  }
  uVar2 = dcgettext(0,"write failed",5);
                    /* WARNING: Subroutine does not return */
  FUN_001049e0(uVar2);
}




// Function: ck_fflush @ 0x4ac0

void ck_fflush(FILE *param_1)

{
  undefined8 uVar1;
  int iVar2;
  
  iVar2 = fflush_unlocked(param_1);
  if (iVar2 == 0) {
    return;
  }
  uVar1 = dcgettext(0,"write failed",5);
                    /* WARNING: Subroutine does not return */
  FUN_001049e0(uVar1);
}




// Function: flush_line @ 0x4af0

void flush_line(void)

{
  undefined8 uVar1;
  int iVar2;
  
  do {
    iVar2 = getchar_unlocked();
    if (iVar2 == -1) break;
  } while (iVar2 != 10);
  iVar2 = ferror_unlocked(stdin);
  if (iVar2 == 0) {
    return;
  }
  uVar1 = dcgettext(0,"read failed",5);
                    /* WARNING: Subroutine does not return */
  FUN_001049e0(uVar1);
}




// Function: ck_fclose @ 0x4b40

void ck_fclose(FILE *param_1)

{
  int iVar1;
  
  iVar1 = fclose(param_1);
  if (iVar1 == 0) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  FUN_001049e0("fclose");
}




// Function: skip_white @ 0x4b60

int skip_white(void)

{
  int iVar1;
  int iVar2;
  ushort **ppuVar3;
  undefined8 uVar4;
  
  ppuVar3 = __ctype_b_loc();
  while( true ) {
    iVar1 = getchar_unlocked();
    if (((*(byte *)((long)*ppuVar3 + (long)iVar1 * 2 + 1) & 0x20) == 0) || (iVar1 == 10)) break;
    FUN_00104870();
  }
  iVar2 = ferror_unlocked(stdin);
  if (iVar2 == 0) {
    return iVar1;
  }
  uVar4 = dcgettext(0,"read failed",5);
                    /* WARNING: Subroutine does not return */
  FUN_001049e0(uVar4);
}




// Function: lf_refill @ 0x4c30

long lf_refill(undefined8 *param_1)

{
  long lVar1;
  undefined1 *puVar2;
  
  lVar1 = FUN_00104bd0(param_1[2],*param_1);
  param_1[1] = param_1[2];
  puVar2 = (undefined1 *)(param_1[2] + lVar1);
  param_1[3] = puVar2;
  *puVar2 = 10;
  FUN_00104870();
  return lVar1;
}




// Function: lf_skip @ 0x4c70

void lf_skip(long param_1,long param_2)

{
  void *pvVar1;
  long lVar2;
  
  do {
    if (param_2 == 0) {
      return;
    }
    while( true ) {
      pvVar1 = rawmemchr(*(void **)(param_1 + 8),10);
      *(void **)(param_1 + 8) = pvVar1;
      if (pvVar1 != *(void **)(param_1 + 0x18)) break;
      lVar2 = FUN_00104c30(param_1);
      if (lVar2 == 0) {
        return;
      }
    }
    *(long *)(param_1 + 8) = (long)pvVar1 + 1;
    param_2 = param_2 + -1;
  } while( true );
}




// Function: lf_copy @ 0x4cd0

void lf_copy(long param_1,long param_2,undefined8 param_3)

{
  void *pvVar1;
  long lVar2;
  void *pvVar3;
  void *pvVar4;
  
  pvVar1 = *(void **)(param_1 + 8);
  if (param_2 == 0) {
    lVar2 = 0;
  }
  else {
    pvVar3 = *(void **)(param_1 + 0x18);
    pvVar4 = pvVar1;
    do {
      while( true ) {
        pvVar1 = rawmemchr(pvVar1,10);
        *(void **)(param_1 + 8) = pvVar1;
        if (pvVar1 != pvVar3) break;
        FUN_00104a80(pvVar4,(long)pvVar3 - (long)pvVar4,param_3);
        lVar2 = FUN_00104c30(param_1);
        if (lVar2 == 0) {
          return;
        }
        pvVar1 = *(void **)(param_1 + 8);
        pvVar3 = *(void **)(param_1 + 0x18);
        pvVar4 = pvVar1;
      }
      pvVar1 = (void *)((long)pvVar1 + 1);
      *(void **)(param_1 + 8) = pvVar1;
      param_2 = param_2 + -1;
    } while (param_2 != 0);
    lVar2 = (long)pvVar1 - (long)pvVar4;
    pvVar1 = pvVar4;
  }
  FUN_00104a80(pvVar1,lVar2,param_3);
  return;
}




// Function: edit @ 0x4ec0

undefined8
edit(undefined8 param_1,undefined8 param_2,undefined8 param_3,long param_4,
            undefined8 param_5,undefined8 param_6)

{
  undefined1 *puVar1;
  int iVar2;
  __pid_t __pid;
  __pid_t _Var3;
  undefined8 uVar4;
  int *piVar5;
  long lVar6;
  int unaff_EBX;
  undefined1 *puVar7;
  long in_FS_OFFSET;
  undefined1 auStack_10030 [65536];
  
  puVar1 = &stack0xffffffffffffffd0;
  do {
    puVar7 = puVar1;
    *(undefined8 *)(puVar7 + -0x1000) = *(undefined8 *)(puVar7 + -0x1000);
    puVar1 = puVar7 + -0x1000;
  } while (puVar7 + -0x1000 != auStack_10030);
  uVar4 = *(undefined8 *)(puVar7 + 0xf048);
  *(undefined8 *)(puVar7 + -0x1068) = param_1;
  *(undefined8 *)(puVar7 + -0x1060) = param_2;
  *(undefined8 *)(puVar7 + -0x1058) = param_3;
  *(undefined8 *)(puVar7 + -0x1050) = param_6;
  *(undefined8 *)(puVar7 + 0xeff0) = *(undefined8 *)(in_FS_OFFSET + 0x28);
LAB_00104f30:
  *(undefined8 *)(puVar7 + -0x1080) = 0x104f3a;
  iVar2 = putchar_unlocked(0x25);
joined_r0x00104f3d:
  if (iVar2 != 0x25) {
    *(undefined8 *)(puVar7 + -0x1080) = 0x104f95;
    uVar4 = dcgettext(0,"write failed",5);
                    /* WARNING: Subroutine does not return */
    *(undefined8 *)(puVar7 + -0x1080) = 0x104f9d;
    FUN_001049e0(uVar4);
  }
  *(undefined8 *)(puVar7 + -0x1080) = 0x104f4b;
  FUN_00104ac0(stdout);
  *(undefined8 *)(puVar7 + -0x1080) = 0x104f50;
  iVar2 = FUN_00104b60();
  if (0x32 < iVar2) goto LAB_00104fa0;
  if (0x30 < iVar2) goto LAB_00104fed;
  if (iVar2 == -1) {
    *(undefined8 *)(puVar7 + -0x1080) = 0x10503c;
    iVar2 = feof_unlocked(stdin);
    if (iVar2 == 0) goto LAB_00104fae;
switchD_00105024_caseD_71:
    uVar4 = 0;
    goto LAB_00105046;
  }
  if (iVar2 == 10) goto switchD_00105024_caseD_66;
  goto LAB_00104fae;
LAB_00104fa0:
  if (iVar2 == 0x65) {
    *(undefined8 *)(puVar7 + -0x1080) = 0x105075;
    unaff_EBX = FUN_00104b60();
    if (unaff_EBX < 0x33) {
      if (0x30 < unaff_EBX) goto LAB_00105236;
      if (unaff_EBX != 10) goto LAB_00104fd5;
switchD_00105024_caseD_65:
      if (DAT_0010f5b8 == (char *)0x0) {
        *(undefined8 *)(puVar7 + -0x1080) = 0x1052cf;
        iVar2 = FUN_001047b0();
        if (iVar2 < 0) {
                    /* WARNING: Subroutine does not return */
          *(undefined8 *)(puVar7 + -0x1080) = 0x105430;
          FUN_001049e0("mkstemp");
        }
        *(undefined8 *)(puVar7 + -0x1080) = 0x1052e5;
        DAT_0010f5b0 = fdopen(iVar2,"w");
      }
      else {
        *(undefined8 *)(puVar7 + -0x1080) = 0x1050b5;
        DAT_0010f5b0 = fopen(DAT_0010f5b8,"w");
      }
      if (DAT_0010f5b0 == (FILE *)0x0) {
                    /* WARNING: Subroutine does not return */
        *(undefined8 *)(puVar7 + -0x1080) = 0x10546f;
        FUN_001049e0(DAT_0010f5b8);
      }
      if (unaff_EBX == 100) {
        if (param_4 != 0) {
          if (param_4 == 1) {
            *(undefined8 *)(puVar7 + -0x1080) = 0x10540c;
            __fprintf_chk(DAT_0010f5b0,1,"--- %s %ld\n",*(undefined8 *)(puVar7 + -0x1060),
                          *(undefined8 *)(puVar7 + -0x1058));
          }
          else {
            *(undefined8 *)(puVar7 + -0x1080) = 0x10535a;
            __fprintf_chk(DAT_0010f5b0,1,"--- %s %ld,%ld\n",*(undefined8 *)(puVar7 + -0x1060),
                          *(long *)(puVar7 + -0x1058),*(long *)(puVar7 + -0x1058) + -1 + param_4);
          }
        }
        *(undefined8 *)(puVar7 + -0x1080) = 0x105371;
        FUN_00104cd0(*(undefined8 *)(puVar7 + -0x1068),param_4,DAT_0010f5b0);
        if (*(long *)(puVar7 + 0xf040) != 0) {
          if (*(long *)(puVar7 + 0xf040) == 1) {
            *(undefined8 *)(puVar7 + -0x1080) = 0x1053e7;
            __fprintf_chk(DAT_0010f5b0,1,"+++ %s %ld\n",*(undefined8 *)(puVar7 + -0x1050),
                          *(undefined8 *)(puVar7 + 0xf038));
          }
          else {
            *(undefined8 *)(puVar7 + -0x1080) = 0x1053c2;
            __fprintf_chk(DAT_0010f5b0,1,"+++ %s %ld,%ld\n",*(undefined8 *)(puVar7 + -0x1050),
                          *(long *)(puVar7 + 0xf038),
                          *(long *)(puVar7 + 0xf038) + -1 + *(long *)(puVar7 + 0xf040));
          }
        }
LAB_00105109:
        *(undefined8 *)(puVar7 + -0x1080) = 0x105120;
        FUN_00104cd0(param_5,*(undefined8 *)(puVar7 + 0xf040),DAT_0010f5b0);
      }
      else {
        if (unaff_EBX < 0x65) {
          if ((unaff_EBX != 0x31) && (unaff_EBX != 0x62)) goto LAB_00105311;
LAB_001050e2:
          *(undefined8 *)(puVar7 + -0x1080) = 0x1050f2;
          FUN_00104cd0(*(undefined8 *)(puVar7 + -0x1068),param_4,DAT_0010f5b0);
        }
        else {
          if (unaff_EBX == 0x6c) goto LAB_001050e2;
LAB_00105311:
          *(undefined8 *)(puVar7 + -0x1080) = 0x10531e;
          FUN_00104c70(*(undefined8 *)(puVar7 + -0x1068),param_4);
        }
        if (unaff_EBX < 0x65) {
          if ((unaff_EBX == 0x32) || (unaff_EBX == 0x62)) goto LAB_00105109;
        }
        else if (unaff_EBX == 0x72) goto LAB_00105109;
        *(undefined8 *)(puVar7 + -0x1080) = 0x105303;
        FUN_00104c70(param_5,*(undefined8 *)(puVar7 + 0xf040));
      }
      *(undefined8 *)(puVar7 + -0x1080) = 0x10512c;
      FUN_00104b40(DAT_0010f5b0);
      DAT_0010f160 = 1;
      *(undefined8 *)(puVar7 + -0x1080) = 0x105138;
      FUN_00104870();
      *(undefined8 *)(puVar7 + -0x1028) = 0;
      *(undefined **)(puVar7 + -0x1038) = PTR_s_ed_0010a997_9_0010f010;
      *(char **)(puVar7 + -0x1030) = DAT_0010f5b8;
      *(undefined8 *)(puVar7 + -0x1080) = 0x10515e;
      __pid = fork();
      if (__pid == 0) {
        *(undefined8 *)(puVar7 + -0x1080) = 0x10544d;
        execvp(PTR_s_ed_0010a997_9_0010f010,(char **)(puVar7 + -0x1038));
        *(undefined8 *)(puVar7 + -0x1080) = 0x105452;
        piVar5 = __errno_location();
        iVar2 = *piVar5;
                    /* WARNING: Subroutine does not return */
        *(undefined8 *)(puVar7 + -0x1080) = 0x105463;
        _exit((iVar2 == 2) + 0x7e);
      }
      if (__pid < 0) {
                    /* WARNING: Subroutine does not return */
        *(undefined8 *)(puVar7 + -0x1080) = 0x10543c;
        FUN_001049e0(&DAT_0010a9ea);
      }
      while( true ) {
        *(undefined8 *)(puVar7 + -0x1080) = 0x105197;
        _Var3 = waitpid(__pid,(int *)(puVar7 + -0x103c),0);
        if (-1 < _Var3) break;
        *(undefined8 *)(puVar7 + -0x1080) = 0x10517d;
        piVar5 = __errno_location();
        if (*piVar5 != 4) {
                    /* WARNING: Subroutine does not return */
          *(undefined8 *)(puVar7 + -0x1080) = 0x105424;
          FUN_001049e0("waitpid");
        }
        *(undefined8 *)(puVar7 + -0x1080) = 0x10518b;
        FUN_00104870();
      }
      DAT_0010f160 = 0;
      *(undefined8 *)(puVar7 + -0x1080) = 0x1051b9;
      FUN_00104e40(*(undefined4 *)(puVar7 + -0x103c),0,PTR_s_ed_0010a997_9_0010f010);
      *(undefined8 *)(puVar7 + -0x1080) = 0x1051cc;
      DAT_0010f5b0 = (FILE *)FUN_00104a60(DAT_0010f5b8,"r");
      while( true ) {
        *(undefined8 *)(puVar7 + -0x1080) = 0x105202;
        lVar6 = FUN_00104bd0(puVar7 + -0x1018,DAT_0010f5b0);
        if (lVar6 == 0) break;
        *(undefined8 *)(puVar7 + -0x1080) = 0x1051e5;
        FUN_00104870();
        *(undefined8 *)(puVar7 + -0x1080) = 0x1051f3;
        FUN_00104a80(puVar7 + -0x1018,lVar6,uVar4);
      }
      *(undefined8 *)(puVar7 + -0x1080) = 0x105216;
      FUN_00104b40(DAT_0010f5b0);
      uVar4 = 1;
LAB_00105046:
      if (*(long *)(puVar7 + 0xeff0) != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
        *(undefined **)(puVar7 + -0x1080) = &UNK_00105474;
        __stack_chk_fail();
      }
      return uVar4;
    }
    if ((unaff_EBX - 0x62U < 0x11) && ((0x10405UL >> ((ulong)(unaff_EBX - 0x62U) & 0x3f) & 1) != 0))
    {
LAB_00105236:
      *(undefined8 *)(puVar7 + -0x1080) = 0x10523b;
      iVar2 = FUN_00104b60();
      if (iVar2 == 10) goto switchD_00105024_caseD_65;
    }
LAB_00104fd5:
    *(undefined8 *)(puVar7 + -0x1080) = 0x104fda;
    FUN_00104830();
    *(undefined8 *)(puVar7 + -0x1080) = 0x104fdf;
    FUN_00104af0();
    goto LAB_00104f30;
  }
  if (iVar2 < 0x65) {
LAB_00104fae:
    *(undefined8 *)(puVar7 + -0x1080) = 0x104fb3;
    FUN_00104af0();
  }
  else {
    if (0x73 < iVar2) {
      if (iVar2 == 0x76) goto LAB_00104fed;
      goto LAB_00104fae;
    }
    if (iVar2 < 0x71) {
      if (iVar2 != 0x6c) goto LAB_00104fae;
      *(undefined8 *)(puVar7 + -0x1080) = 0x104fcc;
      iVar2 = FUN_00104b60();
      if (iVar2 != 10) goto LAB_00104fd5;
      goto switchD_00105024_caseD_6c;
    }
LAB_00104fed:
    *(int *)(puVar7 + -0x106c) = iVar2;
    *(undefined8 *)(puVar7 + -0x1080) = 0x104ff6;
    iVar2 = FUN_00104b60();
    if (iVar2 != 10) goto LAB_00104fd5;
    iVar2 = *(int *)(puVar7 + -0x106c);
    if (iVar2 < 0x77) {
      if (iVar2 < 0x65) {
        if (iVar2 == 0x31) goto switchD_00105024_caseD_6c;
        if (iVar2 == 0x32) goto switchD_00105024_caseD_72;
      }
      else {
        switch(iVar2) {
        case 0x65:
          goto switchD_00105024_caseD_65;
        case 0x6c:
switchD_00105024_caseD_6c:
          *(undefined8 *)(puVar7 + -0x1080) = 0x1052b0;
          FUN_00104cd0(*(undefined8 *)(puVar7 + -0x1068),param_4,uVar4);
          *(undefined8 *)(puVar7 + -0x1080) = 0x1052c0;
          FUN_00104c70(param_5,*(undefined8 *)(puVar7 + 0xf040));
          uVar4 = 1;
          goto LAB_00105046;
        case 0x71:
          goto switchD_00105024_caseD_71;
        case 0x72:
switchD_00105024_caseD_72:
          *(undefined8 *)(puVar7 + -0x1080) = 0x105271;
          FUN_00104cd0(param_5,*(undefined8 *)(puVar7 + 0xf040),uVar4);
          *(undefined8 *)(puVar7 + -0x1080) = 0x10527e;
          FUN_00104c70(*(undefined8 *)(puVar7 + -0x1068),param_4);
          uVar4 = 1;
          goto LAB_00105046;
        case 0x73:
          goto switchD_00105024_caseD_73;
        case 0x76:
          DAT_0010f161 = 0;
          goto LAB_00104f30;
        }
      }
    }
  }
switchD_00105024_caseD_66:
  *(undefined8 *)(puVar7 + -0x1080) = 0x104f73;
  FUN_00104830();
  *(undefined8 *)(puVar7 + -0x1080) = 0x104f7d;
  iVar2 = putchar_unlocked(0x25);
  goto joined_r0x00104f3d;
switchD_00105024_caseD_73:
  DAT_0010f161 = 1;
  goto LAB_00104f30;
}




// Function: interact @ 0x5480

bool interact(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                 undefined8 param_5,undefined8 param_6)

{
  char cVar1;
  int iVar2;
  int *piVar3;
  intmax_t iVar4;
  intmax_t iVar5;
  intmax_t iVar6;
  long in_FS_OFFSET;
  bool bVar7;
  long local_190;
  long local_188;
  char *local_150;
  char local_148;
  char local_147 [263];
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_188 = 1;
  local_190 = 1;
LAB_001054e0:
  iVar2 = FUN_00104d80(param_1,&local_148);
  do {
    if (iVar2 < 1) {
      bVar7 = iVar2 != 0;
LAB_001055f9:
      if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
        return bVar7;
      }
      goto LAB_00105689;
    }
    FUN_00104870();
    if (local_148 == ' ') break;
    piVar3 = __errno_location();
    *piVar3 = 0;
    iVar4 = strtoimax(local_147,&local_150,10);
    if ((((iVar4 < 0) || (*piVar3 != 0)) || (*local_150 != ',')) ||
       (((iVar5 = strtoimax(local_150 + 1,&local_150,10), iVar5 < 0 || (*piVar3 != 0)) ||
        (*local_150 != '\0')))) {
LAB_00105681:
      FUN_001048d0(&local_148);
LAB_00105689:
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
    iVar6 = iVar5;
    if (iVar5 <= iVar4) {
      iVar6 = iVar4;
    }
    if (local_148 == 'c') {
      FUN_00104cd0(param_1,iVar6,stdout);
      cVar1 = FUN_00104ec0(param_2,param_3,local_190,iVar4,param_4,param_5,local_188,iVar5,param_6);
      bVar7 = false;
      if (cVar1 == '\0') goto LAB_001055f9;
    }
    else {
      if (local_148 != 'i') goto LAB_00105681;
      if (DAT_0010f161 == '\0') {
        FUN_00104cd0(param_1,iVar6,stdout);
      }
      else {
        FUN_00104c70(param_1);
      }
      FUN_00104cd0(param_2,iVar4,param_6);
      FUN_00104c70(param_4,iVar5);
    }
    local_190 = local_190 + iVar4;
    local_188 = local_188 + iVar5;
    iVar2 = FUN_00104d80(param_1,&local_148);
  } while( true );
  puts(local_147);
  goto LAB_001054e0;
}



