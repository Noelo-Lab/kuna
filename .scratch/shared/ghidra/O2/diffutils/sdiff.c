// Function: main @ 0x3b00

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 main(int param_1,undefined **param_2)

{
  char *pcVar1;
  uint uVar2;
  byte bVar3;
  byte bVar4;
  char cVar5;
  int iVar6;
  __pid_t _Var7;
  char *pcVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  undefined8 uVar13;
  FILE *pFVar14;
  undefined1 *__s;
  long lVar15;
  undefined1 *puVar16;
  intmax_t iVar17;
  intmax_t iVar18;
  int *piVar19;
  char *pcVar20;
  ulong uVar21;
  char **ppcVar22;
  intmax_t iVar23;
  sigaction *__oact;
  ulong __n;
  int *piVar24;
  long in_FS_OFFSET;
  bool bVar25;
  long local_210;
  long local_208;
  undefined8 local_1c0;
  undefined8 local_1b8;
  undefined1 *local_1b0;
  undefined1 *local_1a8;
  undefined1 *local_1a0;
  undefined8 local_198;
  undefined1 *local_190;
  undefined1 *local_188;
  undefined1 *local_180;
  FILE *local_178;
  undefined1 *local_170;
  undefined1 *local_168;
  undefined1 *local_160;
  int local_150;
  int local_14c;
  char local_148;
  char local_147 [263];
  long local_40;
  
  pcVar20 = (char *)(long)param_1;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  DAT_00110020 = 2;
  FUN_00105b80(*param_2);
  setlocale(6,"");
  bindtextdomain("diffutils","/usr/local/share/locale");
  textdomain("diffutils");
  FUN_00105ac0(FUN_00104990);
  FUN_001082b0();
  pcVar8 = getenv("EDITOR");
  if (pcVar8 != (char *)0x0) {
    PTR_s_ed_0010b970_9_00110010 = pcVar8;
  }
  FUN_00104900(&DAT_0010ba47);
  while (iVar6 = getopt_long(param_1,param_2,"abBdEHiI:lo:stvw:WZ",&PTR_s_diff_program_0010f880,0),
        uVar9 = DAT_00120620, iVar6 != -1) {
    switch(iVar6) {
    case 0x42:
      FUN_00104900(&DAT_0010ba09);
      break;
    default:
      goto switchD_00103bef_caseD_43;
    case 0x45:
      FUN_00104900(&DAT_0010ba0f);
      break;
    case 0x48:
      FUN_00104900(&DAT_0010ba12);
      break;
    case 0x49:
      FUN_00104900(&DAT_0010ba18);
      FUN_00104900(optarg);
      break;
    case 0x57:
      FUN_00104900(&DAT_0010ba4f);
      break;
    case 0x5a:
      FUN_00104900(&DAT_0010ba52);
      break;
    case 0x61:
      FUN_00104900(&DAT_0010ba03);
      break;
    case 0x62:
      FUN_00104900(&DAT_0010ba06);
      break;
    case 100:
      FUN_00104900(&DAT_0010ba0c);
      break;
    case 0x69:
      FUN_00104900(&DAT_0010ba15);
      break;
    case 0x6c:
      FUN_00104900("--left-column");
      break;
    case 0x6f:
      DAT_00110168 = optarg;
      break;
    case 0x73:
      DAT_00110161 = '\x01';
      break;
    case 0x74:
      FUN_00104900(&DAT_0010ba29);
      break;
    case 0x76:
      uVar9 = FUN_00106020("Thomas Lord");
      FUN_00107bf0(stdout,&DAT_0010ba46,"GNU diffutils",PTR_DAT_00110018,uVar9,0);
      FUN_00104cc0();
LAB_00103dc7:
      if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
        __stack_chk_fail();
      }
      return 0;
    case 0x77:
      FUN_00104900(&DAT_0010ba4c);
      FUN_00104900(optarg);
      break;
    case 0x80:
      *DAT_001105c0 = optarg;
      break;
    case 0x81:
      uVar10 = dcgettext(0,"Usage: %s [OPTION]... FILE1 FILE2\n",5);
      param_2 = &PTR_s__o____output_FILE_operate_intera_0010f7a0;
      __printf_chk(1,uVar10,uVar9);
      uVar9 = dcgettext(0,"Side-by-side merge of differences between FILE1 and FILE2.",5);
      __printf_chk(1,&DAT_0010ba55,uVar9);
      pFVar14 = stdout;
      pcVar8 = (char *)dcgettext(0,
                                 "Mandatory arguments to long options are mandatory for short options too.\n"
                                 ,5);
      pcVar20 = "  %s\n";
      fputs_unlocked(pcVar8,pFVar14);
      pcVar8 = "-o, --output=FILE            operate interactively, sending output to FILE";
      goto LAB_00103cdd;
    case 0x82:
      FUN_00104900("--strip-trailing-cr");
      break;
    case 0x83:
      FUN_00104900("--tabsize");
      FUN_00104900(optarg);
    }
  }
  if (param_1 - optind == 2) {
    if (DAT_00110168 != (char *)0x0) {
      bVar3 = FUN_001048a0(param_2[optind]);
      bVar4 = FUN_001048a0(param_2[(long)optind + 1]);
      if ((bVar3 & bVar4) == 0) {
        uVar9 = FUN_00104be0(param_2[optind],bVar3,param_2[(long)optind + 1]);
        uVar10 = FUN_00104b60(uVar9,"r");
        uVar11 = FUN_00104be0(param_2[(long)optind + 1],bVar4,param_2[optind]);
        uVar12 = FUN_00104b60(uVar11,"r");
        uVar13 = FUN_00104b60(DAT_00110168,&DAT_0010ba50);
        FUN_00104900("--sdiff-merge-assist");
        FUN_00104900(&DAT_0010bae3);
        FUN_00104900(param_2[optind]);
        piVar19 = &DAT_0010bd30;
        piVar24 = &DAT_0010bd30;
        FUN_00104900(param_2[(long)optind + 1]);
        FUN_00104900(0);
        _DAT_00110148 = 0x10000000;
        sigemptyset((sigset_t *)&DAT_001100c8);
        do {
          iVar6 = *piVar24;
          piVar24 = piVar24 + 1;
          sigaddset((sigset_t *)&DAT_001100c8,iVar6);
        } while (piVar24 != (int *)&DAT_0010bd4c);
        __oact = (sigaction *)&DAT_00110180;
        do {
          iVar6 = *piVar19;
          sigaction(iVar6,(sigaction *)0x0,__oact);
          if ((__oact->__sigaction_handler).sa_handler != (__sighandler_t)0x1) {
            DAT_001100c0 = FUN_00104820;
            sigaction(iVar6,(sigaction *)&DAT_001100c0,(sigaction *)0x0);
          }
          __oact = __oact + 1;
          piVar19 = piVar19 + 1;
        } while (__oact != (sigaction *)&DAT_001105a8);
        signal(0x11,(__sighandler_t)0x0);
        DAT_00110158 = '\x01';
        iVar6 = pipe(&local_150);
        if (iVar6 != 0) {
                    /* WARNING: Subroutine does not return */
          FUN_00104b30(&DAT_0010bafb);
        }
        DAT_001105a8 = fork();
        if (DAT_001105a8 < 0) {
                    /* WARNING: Subroutine does not return */
          FUN_00104b30(&DAT_0010b9cf);
        }
        if (DAT_001105a8 == 0) {
          if (DAT_00110510 != 1) {
            DAT_001100c0 = (code *)0x1;
            sigaction(2,(sigaction *)&DAT_001100c0,(sigaction *)0x0);
          }
          DAT_001100c0 = (code *)0x0;
          sigaction(0xd,(sigaction *)&DAT_001100c0,(sigaction *)0x0);
          close(local_150);
          if (local_14c != 1) {
            dup2(local_14c,1);
            close(local_14c);
          }
          execvp(*DAT_001105c0,DAT_001105c0);
          piVar19 = __errno_location();
                    /* WARNING: Subroutine does not return */
          _exit((*piVar19 == 2) + 0x7e);
        }
        close(local_14c);
        pFVar14 = fdopen(local_150,"r");
        if (pFVar14 == (FILE *)0x0) {
                    /* WARNING: Subroutine does not return */
          FUN_00104b30("fdopen");
        }
        local_178 = pFVar14;
        __s = (undefined1 *)FUN_00107df0(0x10001);
        *__s = 10;
        local_1b8 = uVar10;
        local_170 = __s;
        local_168 = __s;
        local_160 = __s;
        local_1b0 = (undefined1 *)FUN_00107df0(0x10001);
        *local_1b0 = 10;
        local_1a8 = local_1b0;
        local_1a0 = local_1b0;
        local_198 = uVar12;
        local_190 = (undefined1 *)FUN_00107df0(0x10001);
        local_208 = 1;
        *local_190 = 10;
        local_210 = 1;
        local_188 = local_190;
        local_180 = local_190;
        do {
          while( true ) {
            uVar21 = 0x100;
            pcVar20 = &local_148;
            while( true ) {
              puVar16 = rawmemchr(__s,10);
              __n = (long)puVar16 - (long)__s;
              if (uVar21 <= __n) {
                bVar25 = false;
                goto LAB_0010441a;
              }
              memcpy(pcVar20,__s,__n);
              __s = local_168;
              uVar2 = DAT_0011015c;
              if (puVar16 < local_160) break;
              lVar15 = FUN_00104d80(local_168,local_178);
              uVar2 = DAT_0011015c;
              local_170 = __s;
              local_160 = __s + lVar15;
              *local_160 = 10;
              if (uVar2 != 0) {
                if (0 < DAT_001105a8) {
                  kill(DAT_001105a8,0xd);
                }
                if (DAT_001105b8 != (char *)0x0) {
                  unlink(DAT_001105b8);
                }
                if (DAT_00110158 != '\0') {
                  FUN_00104a10(uVar2);
                }
                raise(uVar2);
                    /* WARNING: Subroutine does not return */
                exit(2);
              }
              if (lVar15 == 0) {
                bVar25 = __n == 0;
                goto LAB_0010441a;
              }
              pcVar20 = pcVar20 + __n;
              uVar21 = uVar21 - __n;
            }
            ppcVar22 = (char **)(ulong)DAT_0011015c;
            __s = puVar16 + 1;
            pcVar20[__n] = '\0';
            local_170 = __s;
            if (uVar2 != 0) goto LAB_00104586;
            if (local_148 != ' ') break;
            puts(local_147);
          }
          piVar19 = __errno_location();
          *piVar19 = 0;
          iVar17 = strtoimax(local_147,(char **)&local_1c0,10);
          if ((((iVar17 < 0) || (*piVar19 != 0)) ||
              (*(char *)CONCAT44(local_1c0._4_4_,(uint)local_1c0) != ',')) ||
             (((iVar18 = strtoimax((char *)CONCAT44(local_1c0._4_4_,(uint)local_1c0) + 1,
                                   (char **)&local_1c0,10), iVar18 < 0 || (*piVar19 != 0)) ||
              (*(char *)CONCAT44(local_1c0._4_4_,(uint)local_1c0) != '\0')))) {
LAB_0010457d:
            FUN_00104b00(&local_148);
            ppcVar22 = (char **)&local_1c0;
LAB_00104586:
            if (0 < DAT_001105a8) {
              kill(DAT_001105a8,0xd);
            }
            if (DAT_001105b8 != (char *)0x0) {
              unlink(DAT_001105b8);
            }
            if (DAT_00110158 != '\0') {
              FUN_00104a10((ulong)ppcVar22 & 0xffffffff);
            }
            raise((int)ppcVar22);
                    /* WARNING: Subroutine does not return */
            exit(2);
          }
          iVar23 = iVar18;
          if (iVar18 <= iVar17) {
            iVar23 = iVar17;
          }
          if (local_148 == 'c') {
            FUN_00105000(&local_178,iVar23,stdout);
            cVar5 = FUN_00105120(&local_1b8,uVar9,local_210,iVar17,&local_198,uVar11,local_208,
                                 iVar18,uVar13);
            if (cVar5 == '\0') goto LAB_00104509;
          }
          else {
            if (local_148 != 'i') goto LAB_0010457d;
            if (DAT_00110161 == '\0') {
              FUN_00105000(&local_178,iVar23,stdout);
            }
            else {
              FUN_00104f30();
            }
            FUN_00105000(&local_1b8,iVar17,uVar13);
            FUN_00104f30(&local_198,iVar18);
          }
          local_208 = local_208 + iVar18;
          local_210 = local_210 + iVar17;
          __s = local_170;
        } while( true );
      }
      FUN_00104b00("both files to be compared are directories",bVar3);
    }
    if (DAT_00110161 != '\0') {
      FUN_00104900("--suppress-common-lines");
    }
    FUN_00104900(&DAT_0010bae0);
    FUN_00104900(&DAT_0010bae3);
    FUN_00104900(param_2[optind]);
    FUN_00104900(param_2[(long)optind + 1]);
    FUN_00104900(0);
    ppcVar22 = DAT_001105c0;
    execvp(*DAT_001105c0,DAT_001105c0);
                    /* WARNING: Subroutine does not return */
    FUN_00104b30(*ppcVar22);
  }
  if (1 < param_1 - optind) {
    pcVar8 = "extra operand \'%s\'";
    FUN_00104840("extra operand \'%s\'",param_2[(long)optind + 2]);
    do {
      __overflow((_IO_FILE *)pcVar8,10);
      while( true ) {
        pcVar8 = param_2[1];
        param_2 = param_2 + 1;
        while( true ) {
          if (pcVar8 == (char *)0x0) {
            uVar9 = dcgettext(0,
                              "Exit status is 0 if inputs are the same, 1 if different, 2 if trouble."
                              ,5);
            uVar10 = dcgettext(0,"If a FILE is \'-\', read standard input.",5);
            __printf_chk(1,"\n%s\n%s\n",uVar10,uVar9);
            FUN_00107cd0();
            FUN_00104cc0();
            goto LAB_00103dc7;
          }
LAB_00103cdd:
          if (*pcVar8 == '\0') break;
          param_2 = param_2 + 1;
          uVar9 = dcgettext(0,pcVar8,5);
          __printf_chk(1,pcVar20,uVar9);
          pcVar8 = *param_2;
        }
        pcVar1 = stdout->_IO_write_ptr;
        pcVar8 = (char *)stdout;
        if (stdout->_IO_write_end <= pcVar1) break;
        stdout->_IO_write_ptr = pcVar1 + 1;
        *pcVar1 = '\n';
      }
    } while( true );
  }
  FUN_00104840("missing operand after \'%s\'",param_2[(long)(pcVar20 + -1)]);
switchD_00103bef_caseD_43:
  FUN_00104840(0,0);
LAB_00104504:
                    /* WARNING: Subroutine does not return */
  FUN_00104ad0();
LAB_00104509:
  bVar25 = false;
LAB_0010441a:
  FUN_00104bc0(uVar10);
  FUN_00104bc0(uVar12);
  FUN_00104bc0(uVar13);
  FUN_00104bc0(pFVar14);
  while (_Var7 = waitpid(DAT_001105a8,(int *)&local_1c0,0), _Var7 < 0) {
    piVar19 = __errno_location();
    if (*piVar19 != 4) {
                    /* WARNING: Subroutine does not return */
      FUN_00104b30("waitpid");
    }
    FUN_00104a90();
  }
  DAT_001105a8 = 0;
  if (DAT_001105b8 != (char *)0x0) {
    unlink(DAT_001105b8);
    DAT_001105b8 = (char *)0x0;
  }
  if (bVar25) {
    FUN_00104dc0((uint)local_1c0,1,*DAT_001105c0);
    if (DAT_00110158 != '\0') {
      FUN_00104a10(0);
    }
    FUN_00104a90();
                    /* WARNING: Subroutine does not return */
    exit((uint)local_1c0 >> 8 & 0xff);
  }
  goto LAB_00104504;
}




// Function: catchsig @ 0x4820

void catchsig(int param_1)

{
  if ((param_1 == 2) && (DAT_00110160 != '\0')) {
    return;
  }
  DAT_0011015c = param_1;
  return;
}




// Function: try_help @ 0x4840

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
    uStack_10 = 0x104859;
    uVar3 = dcgettext(0,param_1,5);
    uStack_10 = 0x10486a;
    error(0,0,uVar3,param_2);
  }
  uVar3 = DAT_00120620;
  uStack_10 = 0x104884;
  uVar4 = dcgettext(0,"Try \'%s --help\' for more information.",5);
  __file = (char *)0x2;
  uStack_10 = 0x104898;
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




// Function: diraccess @ 0x48a0

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




// Function: diffarg @ 0x4900

void diffarg(undefined8 param_1)

{
  long lVar1;
  
  if (DAT_001100a8 == DAT_001100a0) {
    if (DAT_001100a8 == 0) {
      DAT_001100a0 = 0x10;
      lVar1 = 0x80;
    }
    else {
      if (0x7fffffffffffffe < DAT_001100a8) {
                    /* WARNING: Subroutine does not return */
        FUN_00108270();
      }
      DAT_001100a0 = DAT_001100a8 * 2;
      lVar1 = DAT_001100a8 << 4;
    }
    DAT_001105c0 = FUN_00107e40(DAT_001105c0,lVar1);
  }
  *(undefined8 *)(DAT_001105c0 + DAT_001100a8 * 8) = param_1;
  DAT_001100a8 = DAT_001100a8 + 1;
  return;
}




// Function: exiterr @ 0x4ad0

void exiterr(void)

{
  FUN_00104990(0);
  if (DAT_00110158 != '\0') {
    FUN_00104a10(0);
  }
  FUN_00104a90();
                    /* WARNING: Subroutine does not return */
  exit(2);
}




// Function: fatal @ 0x4b00

void fatal(undefined8 param_1)

{
  undefined8 uVar1;
  
  uVar1 = dcgettext(0,param_1,5);
  error(0,0,&DAT_0010b949,uVar1);
                    /* WARNING: Subroutine does not return */
  FUN_00104ad0();
}




// Function: perror_fatal @ 0x4b30

void perror_fatal(undefined8 param_1)

{
  int iVar1;
  int *piVar2;
  
  piVar2 = __errno_location();
  iVar1 = *piVar2;
  FUN_00104a90();
  error(0,iVar1,&DAT_0010b949,param_1);
                    /* WARNING: Subroutine does not return */
  FUN_00104ad0();
}




// Function: ck_fwrite @ 0x4b80

void ck_fwrite(void *param_1,size_t param_2,FILE *param_3)

{
  size_t sVar1;
  undefined8 uVar2;
  
  sVar1 = fwrite_unlocked(param_1,1,param_2,param_3);
  if (param_2 == sVar1) {
    return;
  }
  uVar2 = dcgettext(0,"write failed",5);
                    /* WARNING: Subroutine does not return */
  FUN_00104b30(uVar2);
}




// Function: skip_white @ 0x4e40

void skip_white(void)

{
  byte bVar1;
  ushort *puVar2;
  byte *pbVar3;
  int __sig;
  uint uVar4;
  ushort **ppuVar5;
  undefined8 uVar6;
  
  ppuVar5 = __ctype_b_loc();
  while( true ) {
    pbVar3 = (byte *)stdin->_IO_read_ptr;
    if (pbVar3 < stdin->_IO_read_end) {
      puVar2 = *ppuVar5;
      stdin->_IO_read_ptr = (char *)(pbVar3 + 1);
      uVar4 = (uint)*pbVar3;
      bVar1 = *(byte *)((long)puVar2 + (long)(int)(uint)*pbVar3 * 2 + 1);
      __sig = DAT_0011015c;
    }
    else {
      uVar4 = __uflow(stdin);
      bVar1 = *(byte *)((long)*ppuVar5 + (long)(int)uVar4 * 2 + 1);
      __sig = DAT_0011015c;
    }
    DAT_0011015c = __sig;
    if (((bVar1 & 0x20) == 0) || (uVar4 == 10)) break;
    if (__sig != 0) {
      if (0 < DAT_001105a8) {
        kill(DAT_001105a8,0xd);
      }
      if (DAT_001105b8 != (char *)0x0) {
        unlink(DAT_001105b8);
      }
      if (DAT_00110158 != '\0') {
        FUN_00104a10(__sig);
      }
      raise(__sig);
                    /* WARNING: Subroutine does not return */
      exit(2);
    }
  }
  if ((stdin->_flags & 0x20) == 0) {
    return;
  }
  uVar6 = dcgettext(0,"read failed",5);
                    /* WARNING: Subroutine does not return */
  FUN_00104b30(uVar6);
}




// Function: lf_skip @ 0x4f30

void lf_skip(undefined8 *param_1,long param_2)

{
  int __sig;
  void *pvVar1;
  long lVar2;
  undefined1 *puVar3;
  
  do {
    if (param_2 == 0) {
      return;
    }
    while( true ) {
      pvVar1 = rawmemchr((void *)param_1[1],10);
      param_1[1] = pvVar1;
      if (pvVar1 != (void *)param_1[3]) break;
      lVar2 = FUN_00104d80(param_1[2],*param_1);
      __sig = DAT_0011015c;
      param_1[1] = param_1[2];
      puVar3 = (undefined1 *)(param_1[2] + lVar2);
      param_1[3] = puVar3;
      *puVar3 = 10;
      if (__sig != 0) {
        if (0 < DAT_001105a8) {
          kill(DAT_001105a8,0xd);
        }
        if (DAT_001105b8 != (char *)0x0) {
          unlink(DAT_001105b8);
        }
        if (DAT_00110158 != '\0') {
          FUN_00104a10(__sig);
        }
        raise(__sig);
                    /* WARNING: Subroutine does not return */
        exit(2);
      }
      if (lVar2 == 0) {
        return;
      }
    }
    param_1[1] = (long)pvVar1 + 1;
    param_2 = param_2 + -1;
  } while( true );
}




// Function: lf_copy @ 0x5000

void lf_copy(undefined8 *param_1,long param_2,undefined8 param_3)

{
  int __sig;
  void *pvVar1;
  undefined1 *puVar2;
  void *pvVar3;
  long lVar4;
  void *pvVar5;
  
  pvVar1 = (void *)param_1[1];
  if (param_2 == 0) {
    lVar4 = 0;
  }
  else {
    pvVar3 = (void *)param_1[3];
    pvVar5 = pvVar1;
    do {
      while( true ) {
        pvVar1 = rawmemchr(pvVar1,10);
        param_1[1] = pvVar1;
        if (pvVar1 != pvVar3) break;
        FUN_00104b80(pvVar5,(long)pvVar3 - (long)pvVar5,param_3);
        lVar4 = FUN_00104d80(param_1[2],*param_1);
        __sig = DAT_0011015c;
        param_1[1] = param_1[2];
        puVar2 = (undefined1 *)(param_1[2] + lVar4);
        param_1[3] = puVar2;
        *puVar2 = 10;
        if (__sig != 0) {
          if (0 < DAT_001105a8) {
            kill(DAT_001105a8,0xd);
          }
          if (DAT_001105b8 != (char *)0x0) {
            unlink(DAT_001105b8);
          }
          if (DAT_00110158 != '\0') {
            FUN_00104a10(__sig);
          }
          raise(__sig);
                    /* WARNING: Subroutine does not return */
          exit(2);
        }
        if (lVar4 == 0) {
          return;
        }
        pvVar1 = (void *)param_1[1];
        pvVar3 = (void *)param_1[3];
        pvVar5 = pvVar1;
      }
      pvVar1 = (void *)((long)pvVar1 + 1);
      param_1[1] = pvVar1;
      param_2 = param_2 + -1;
    } while (param_2 != 0);
    lVar4 = (long)pvVar1 - (long)pvVar5;
    pvVar1 = pvVar5;
  }
  FUN_00104b80(pvVar1,lVar4,param_3);
  return;
}




// Function: edit @ 0x5120

undefined8
edit(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
            undefined8 param_5,undefined8 param_6)

{
  undefined1 *puVar1;
  int iVar2;
  __pid_t __pid;
  __pid_t _Var3;
  undefined8 uVar4;
  int *piVar5;
  FILE *pFVar6;
  long lVar7;
  undefined8 uVar8;
  char *pcVar9;
  size_t sVar10;
  char *__template;
  int unaff_EBX;
  undefined1 *puVar11;
  long in_FS_OFFSET;
  undefined1 auStack_10030 [65536];
  
  puVar1 = &stack0xffffffffffffffd0;
  do {
    puVar11 = puVar1;
    *(undefined8 *)(puVar11 + -0x1000) = *(undefined8 *)(puVar11 + -0x1000);
    puVar1 = puVar11 + -0x1000;
  } while (puVar11 + -0x1000 != auStack_10030);
  uVar8 = *(undefined8 *)(puVar11 + 0xf048);
  *(undefined8 *)(puVar11 + -0x1068) = param_1;
  *(undefined8 *)(puVar11 + -0x1060) = param_2;
  *(undefined8 *)(puVar11 + -0x1058) = param_3;
  *(undefined8 *)(puVar11 + -0x1078) = param_4;
  *(undefined8 *)(puVar11 + -0x1070) = param_5;
  *(undefined8 *)(puVar11 + -0x1050) = param_6;
  *(undefined8 *)(puVar11 + 0xeff0) = *(undefined8 *)(in_FS_OFFSET + 0x28);
LAB_001051a0:
  pcVar9 = stdout->_IO_write_ptr;
  if (pcVar9 < stdout->_IO_write_end) {
    stdout->_IO_write_ptr = pcVar9 + 1;
    *pcVar9 = '%';
  }
  else {
    *(undefined8 *)(puVar11 + -0x1090) = 0x105572;
    iVar2 = __overflow(stdout,0x25);
    if (iVar2 != 0x25) goto LAB_0010557b;
  }
  *(undefined8 *)(puVar11 + -0x1090) = 0x1051cc;
  iVar2 = fflush_unlocked(stdout);
  if (iVar2 != 0) {
LAB_0010557b:
    *(undefined8 *)(puVar11 + -0x1090) = 0x10558e;
    uVar8 = dcgettext(0,"write failed",5);
                    /* WARNING: Subroutine does not return */
    *(undefined8 *)(puVar11 + -0x1090) = 0x105596;
    FUN_00104b30(uVar8);
  }
  *(undefined8 *)(puVar11 + -0x1090) = 0x1051d9;
  iVar2 = FUN_00104e40();
  if (iVar2 < 0x33) {
    if (0x30 < iVar2) goto LAB_00105295;
    if (iVar2 != -1) {
      if (iVar2 != 10) goto LAB_00105236;
      goto LAB_001051f7;
    }
    if ((*stdin & 0x10) != 0) {
switchD_001052cc_caseD_71:
      uVar8 = 0;
      goto LAB_001054ca;
    }
  }
  else {
    if (iVar2 == 0x65) {
      *(undefined8 *)(puVar11 + -0x1090) = 0x1052d5;
      unaff_EBX = FUN_00104e40();
      if (unaff_EBX < 0x33) {
        iVar2 = unaff_EBX;
        if (0x30 < unaff_EBX) goto LAB_0010550e;
joined_r0x00105516:
        if (iVar2 == 10) goto switchD_001052cc_caseD_65;
      }
      else if ((unaff_EBX - 0x62U < 0x11) &&
              ((0x10405UL >> ((ulong)(unaff_EBX - 0x62U) & 0x3f) & 1) != 0)) {
LAB_0010550e:
        *(undefined8 *)(puVar11 + -0x1090) = 0x105513;
        iVar2 = FUN_00104e40();
        goto joined_r0x00105516;
      }
LAB_0010525d:
      *(undefined8 *)(puVar11 + -0x1090) = 0x10526c;
      uVar4 = dcgettext(0,
                        "ed:\tEdit then use both versions, each decorated with a header.\neb:\tEdit then use both versions.\nel or e1:\tEdit then use the left version.\ner or e2:\tEdit then use the right version.\ne:\tDiscard both versions then edit a new one.\nl or 1:\tUse the left version.\nr or 2:\tUse the right version.\ns:\tSilently include common lines.\nv:\tVerbosely include common lines.\nq:\tQuit.\n"
                        ,5);
      *(undefined8 *)(puVar11 + -0x1090) = 0x105285;
      __fprintf_chk(stderr,1,&DAT_0010b949,uVar4);
      *(undefined8 *)(puVar11 + -0x1090) = 0x10528a;
      FUN_00104d10();
      goto LAB_001051a0;
    }
    if (100 < iVar2) {
      if (0x73 < iVar2) {
        if (iVar2 == 0x76) goto LAB_00105295;
        goto LAB_00105236;
      }
      if (iVar2 < 0x71) {
        if (iVar2 != 0x6c) goto LAB_00105236;
        *(undefined8 *)(puVar11 + -0x1090) = 0x105254;
        iVar2 = FUN_00104e40();
        if (iVar2 != 10) goto LAB_0010525d;
        goto switchD_001052cc_caseD_6c;
      }
LAB_00105295:
      *(int *)(puVar11 + -0x107c) = iVar2;
      *(undefined8 *)(puVar11 + -0x1090) = 0x10529e;
      iVar2 = FUN_00104e40();
      if (iVar2 != 10) goto LAB_0010525d;
      iVar2 = *(int *)(puVar11 + -0x107c);
      if (0x76 < iVar2) {
switchD_001052cc_caseD_66:
        *(undefined8 *)(puVar11 + -0x1090) = 0x1055af;
        uVar4 = dcgettext(0,
                          "ed:\tEdit then use both versions, each decorated with a header.\neb:\tEdit then use both versions.\nel or e1:\tEdit then use the left version.\ner or e2:\tEdit then use the right version.\ne:\tDiscard both versions then edit a new one.\nl or 1:\tUse the left version.\nr or 2:\tUse the right version.\ns:\tSilently include common lines.\nv:\tVerbosely include common lines.\nq:\tQuit.\n"
                          ,5);
        *(undefined8 *)(puVar11 + -0x1090) = 0x1055c8;
        __fprintf_chk(stderr,1,&DAT_0010b949,uVar4);
        goto LAB_001051a0;
      }
      if (100 < iVar2) {
        switch(iVar2) {
        case 0x65:
switchD_001052cc_caseD_65:
          if (DAT_001105b8 == (char *)0x0) {
            *(undefined8 *)(puVar11 + -0x1090) = 0x105622;
            pcVar9 = getenv("TMPDIR");
            if (pcVar9 == (char *)0x0) {
              pcVar9 = "/tmp";
            }
            *(undefined8 *)(puVar11 + -0x1090) = 0x10563b;
            sVar10 = strlen(pcVar9);
            *(undefined8 *)(puVar11 + -0x1090) = 0x105644;
            __template = (char *)FUN_00107df0(sVar10 + 0xd);
            *(undefined8 *)(puVar11 + -0x1090) = 0x105667;
            __sprintf_chk(__template,1,0xffffffffffffffff,"%s/sdiffXXXXXX",pcVar9);
            *(undefined8 *)(puVar11 + -0x1090) = 0x10566f;
            iVar2 = mkstemp(__template);
            if (iVar2 < 0) {
                    /* WARNING: Subroutine does not return */
              *(undefined8 *)(puVar11 + -0x1090) = 0x10578a;
              FUN_00104b30("mkstemp");
            }
            *(undefined8 *)(puVar11 + -0x1090) = 0x10568c;
            DAT_001105b8 = __template;
            DAT_001105b0 = fdopen(iVar2,"w");
          }
          else {
            *(undefined8 *)(puVar11 + -0x1090) = 0x105315;
            DAT_001105b0 = fopen(DAT_001105b8,"w");
          }
          if (DAT_001105b0 == (FILE *)0x0) {
                    /* WARNING: Subroutine does not return */
            *(undefined **)(puVar11 + -0x1090) = &UNK_001058a1;
            FUN_00104b30(DAT_001105b8);
          }
          if (unaff_EBX == 100) {
            if (*(long *)(puVar11 + -0x1078) != 0) {
              if (*(long *)(puVar11 + -0x1078) == 1) {
                *(undefined8 *)(puVar11 + -0x1090) = 0x1057d5;
                __fprintf_chk(DAT_001105b0,1,"--- %s %ld\n",*(undefined8 *)(puVar11 + -0x1060),
                              *(undefined8 *)(puVar11 + -0x1058));
              }
              else {
                *(undefined8 *)(puVar11 + -0x1090) = 0x10570f;
                __fprintf_chk(DAT_001105b0,1,"--- %s %ld,%ld\n",*(undefined8 *)(puVar11 + -0x1060),
                              *(long *)(puVar11 + -0x1058),
                              *(long *)(puVar11 + -0x1058) + -1 + *(long *)(puVar11 + -0x1078));
              }
            }
            *(undefined8 *)(puVar11 + -0x1090) = 0x105728;
            FUN_00105000(*(undefined8 *)(puVar11 + -0x1068),*(undefined8 *)(puVar11 + -0x1078),
                         DAT_001105b0);
            if (*(long *)(puVar11 + 0xf040) != 0) {
              if (*(long *)(puVar11 + 0xf040) == 1) {
                *(undefined8 *)(puVar11 + -0x1090) = 0x1057b0;
                __fprintf_chk(DAT_001105b0,1,"+++ %s %ld\n",*(undefined8 *)(puVar11 + -0x1050),
                              *(undefined8 *)(puVar11 + 0xf038));
              }
              else {
                *(undefined8 *)(puVar11 + -0x1090) = 0x105779;
                __fprintf_chk(DAT_001105b0,1,"+++ %s %ld,%ld\n",*(undefined8 *)(puVar11 + -0x1050),
                              *(long *)(puVar11 + 0xf038),
                              *(long *)(puVar11 + 0xf038) + -1 + *(long *)(puVar11 + 0xf040));
              }
            }
LAB_0010536b:
            *(undefined8 *)(puVar11 + -0x1090) = 0x105384;
            FUN_00105000(*(undefined8 *)(puVar11 + -0x1070),*(undefined8 *)(puVar11 + 0xf040),
                         DAT_001105b0);
          }
          else {
            if (unaff_EBX < 0x65) {
              if ((unaff_EBX == 0x31) || (unaff_EBX == 0x62)) goto LAB_00105342;
LAB_001056ba:
              *(undefined8 *)(puVar11 + -0x1090) = 0x1056c9;
              FUN_00104f30(*(undefined8 *)(puVar11 + -0x1068),*(undefined8 *)(puVar11 + -0x1078));
            }
            else {
              if (unaff_EBX != 0x6c) goto LAB_001056ba;
LAB_00105342:
              *(undefined8 *)(puVar11 + -0x1090) = 0x105354;
              FUN_00105000(*(undefined8 *)(puVar11 + -0x1068),*(undefined8 *)(puVar11 + -0x1078),
                           DAT_001105b0);
            }
            if (unaff_EBX < 0x65) {
              if ((unaff_EBX == 0x32) || (unaff_EBX == 0x62)) goto LAB_0010536b;
            }
            else if (unaff_EBX == 0x72) goto LAB_0010536b;
            *(undefined8 *)(puVar11 + -0x1090) = 0x1056ac;
            FUN_00104f30(*(undefined8 *)(puVar11 + -0x1070),*(undefined8 *)(puVar11 + 0xf040));
          }
          *(undefined8 *)(puVar11 + -0x1090) = 0x105390;
          iVar2 = fclose(DAT_001105b0);
          if (iVar2 != 0) goto LAB_00105889;
          DAT_00110160 = 1;
          if (DAT_0011015c != 0) goto LAB_00105873;
          *(undefined8 *)(puVar11 + -0x1028) = 0;
          *(undefined **)(puVar11 + -0x1038) = PTR_s_ed_0010b970_9_00110010;
          *(char **)(puVar11 + -0x1030) = DAT_001105b8;
          *(undefined8 *)(puVar11 + -0x1090) = 0x1053d3;
          __pid = fork();
          if (__pid == 0) {
            *(undefined8 *)(puVar11 + -0x1090) = 0x10585d;
            execvp(PTR_s_ed_0010b970_9_00110010,(char **)(puVar11 + -0x1038));
            *(undefined8 *)(puVar11 + -0x1090) = 0x105862;
            piVar5 = __errno_location();
            iVar2 = *piVar5;
                    /* WARNING: Subroutine does not return */
            *(undefined8 *)(puVar11 + -0x1090) = 0x105873;
            _exit((iVar2 == 2) + 0x7e);
          }
          if (__pid < 0) {
                    /* WARNING: Subroutine does not return */
            *(undefined8 *)(puVar11 + -0x1090) = 0x10584c;
            FUN_00104b30(&DAT_0010b9cf);
          }
          goto LAB_0010540c;
        default:
          goto switchD_001052cc_caseD_66;
        case 0x6c:
          goto switchD_001052cc_caseD_6c;
        case 0x71:
          goto switchD_001052cc_caseD_71;
        case 0x72:
          goto switchD_001052cc_caseD_72;
        case 0x73:
          DAT_00110161 = 1;
          break;
        case 0x76:
          DAT_00110161 = 0;
        }
        goto LAB_001051a0;
      }
      if (iVar2 != 0x31) {
        if (iVar2 != 0x32) goto switchD_001052cc_caseD_66;
switchD_001052cc_caseD_72:
        *(undefined8 *)(puVar11 + -0x1090) = 0x10554b;
        FUN_00105000(*(undefined8 *)(puVar11 + -0x1070),*(undefined8 *)(puVar11 + 0xf040),uVar8);
        *(undefined8 *)(puVar11 + -0x1090) = 0x10555a;
        FUN_00104f30(*(undefined8 *)(puVar11 + -0x1068),*(undefined8 *)(puVar11 + -0x1078));
        uVar8 = 1;
        goto LAB_001054ca;
      }
switchD_001052cc_caseD_6c:
      *(undefined8 *)(puVar11 + -0x1090) = 0x1055fa;
      FUN_00105000(*(undefined8 *)(puVar11 + -0x1068),*(undefined8 *)(puVar11 + -0x1078),uVar8);
      *(undefined8 *)(puVar11 + -0x1090) = 0x10560c;
      FUN_00104f30(*(undefined8 *)(puVar11 + -0x1070),*(undefined8 *)(puVar11 + 0xf040));
      uVar8 = 1;
      goto LAB_001054ca;
    }
  }
LAB_00105236:
  *(undefined8 *)(puVar11 + -0x1090) = 0x10523b;
  FUN_00104d10();
LAB_001051f7:
  *(undefined8 *)(puVar11 + -0x1090) = 0x105206;
  uVar4 = dcgettext(0,
                    "ed:\tEdit then use both versions, each decorated with a header.\neb:\tEdit then use both versions.\nel or e1:\tEdit then use the left version.\ner or e2:\tEdit then use the right version.\ne:\tDiscard both versions then edit a new one.\nl or 1:\tUse the left version.\nr or 2:\tUse the right version.\ns:\tSilently include common lines.\nv:\tVerbosely include common lines.\nq:\tQuit.\n"
                    ,5);
  *(undefined8 *)(puVar11 + -0x1090) = 0x10521f;
  __fprintf_chk(stderr,1,&DAT_0010b949,uVar4);
  goto LAB_001051a0;
LAB_00105488:
  *(undefined8 *)(puVar11 + -0x1090) = 0x105490;
  lVar7 = FUN_00104d80(puVar11 + -0x1018,DAT_001105b0);
  if (lVar7 == 0) goto code_r0x00105498;
  if (DAT_0011015c != 0) goto LAB_00105873;
  *(undefined8 *)(puVar11 + -0x1090) = 0x105481;
  FUN_00104b80(puVar11 + -0x1018,lVar7,uVar8);
  goto LAB_00105488;
LAB_00105873:
  iVar2 = DAT_0011015c;
  *(undefined8 *)(puVar11 + -0x1090) = 0x10587a;
  FUN_00104990(0);
  goto LAB_00105813;
code_r0x00105498:
  *(undefined8 *)(puVar11 + -0x1090) = 0x1054a4;
  iVar2 = fclose(DAT_001105b0);
  if (iVar2 != 0) {
LAB_00105889:
                    /* WARNING: Subroutine does not return */
    *(undefined8 *)(puVar11 + -0x1090) = 0x105895;
    FUN_00104b30("fclose");
  }
  uVar8 = 1;
LAB_001054ca:
  if (*(long *)(puVar11 + 0xeff0) == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar8;
  }
                    /* WARNING: Subroutine does not return */
  *(undefined8 *)(puVar11 + -0x1090) = 0x105881;
  __stack_chk_fail();
  while( true ) {
    *(undefined8 *)(puVar11 + -0x1090) = 0x1053f5;
    piVar5 = __errno_location();
    iVar2 = DAT_0011015c;
    if (*piVar5 != 4) {
                    /* WARNING: Subroutine does not return */
      *(undefined8 *)(puVar11 + -0x1090) = 0x105840;
      FUN_00104b30("waitpid");
    }
    if (DAT_0011015c != 0) break;
LAB_0010540c:
    *(undefined8 *)(puVar11 + -0x1090) = 0x105418;
    _Var3 = waitpid(__pid,(int *)(puVar11 + -0x103c),0);
    if (-1 < _Var3) {
      DAT_00110160 = 0;
      *(undefined8 *)(puVar11 + -0x1090) = 0x105435;
      FUN_00104dc0(*(undefined4 *)(puVar11 + -0x103c),0,PTR_s_ed_0010b970_9_00110010);
      pcVar9 = DAT_001105b8;
      *(undefined8 *)(puVar11 + -0x1090) = 0x10544b;
      pFVar6 = fopen(DAT_001105b8,"r");
      if (pFVar6 == (FILE *)0x0) {
                    /* WARNING: Subroutine does not return */
        *(undefined8 *)(puVar11 + -0x1090) = 0x105889;
        FUN_00104b30(pcVar9,0);
      }
      DAT_001105b0 = pFVar6;
      goto LAB_00105488;
    }
  }
  if (0 < DAT_001105a8) {
    *(undefined8 *)(puVar11 + -0x1090) = 0x1057fb;
    kill(DAT_001105a8,0xd);
  }
  if (DAT_001105b8 != (char *)0x0) {
    *(undefined8 *)(puVar11 + -0x1090) = 0x105813;
    unlink(DAT_001105b8);
  }
LAB_00105813:
  if (DAT_00110158 != '\0') {
    *(undefined8 *)(puVar11 + -0x1090) = 0x105823;
    FUN_00104a10(iVar2);
  }
  *(undefined8 *)(puVar11 + -0x1090) = 0x10582a;
  raise(iVar2);
                    /* WARNING: Subroutine does not return */
  *(undefined8 *)(puVar11 + -0x1090) = 0x105834;
  exit(2);
}



