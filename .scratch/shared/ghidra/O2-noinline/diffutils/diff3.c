// Function: main @ 0x3ac0

undefined8 main(int param_1,undefined8 *param_2)

{
  char *__s1;
  byte bVar1;
  int iVar2;
  int iVar3;
  long lVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  uint uVar7;
  uint uVar8;
  uint uVar9;
  undefined8 *puVar10;
  long in_FS_OFFSET;
  bool bVar11;
  undefined1 auVar12 [12];
  int local_12c;
  undefined1 local_128 [8];
  void *local_120;
  void *local_118;
  undefined4 local_110;
  int local_10c;
  int local_108;
  int local_104 [3];
  undefined8 local_f8 [4];
  stat local_d8;
  long local_40;
  
  uVar9 = 0;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  DAT_00113020 = 2;
  FUN_001062e0(*param_2);
  setlocale(6,"");
  bindtextdomain("diffutils","/usr/local/share/locale");
  textdomain("diffutils");
  FUN_00106220(0);
  FUN_001086a0();
  local_12c = 0;
LAB_00103b60:
  uVar8 = 0x112920;
  iVar2 = getopt_long(param_1,param_2,"aeimvx3AEL:TX",&PTR_s_diff_program_00112920,0);
  if (iVar2 != -1) {
    switch(iVar2) {
    case 0x33:
      DAT_001130ed = 1;
      uVar9 = uVar9 | 1;
      goto LAB_00103b60;
    default:
      goto switchD_00103b8f_caseD_34;
    case 0x41:
      DAT_001130eb = 1;
      uVar9 = uVar9 | 2;
      DAT_001130ef = 1;
      goto LAB_00103b60;
    case 0x45:
      DAT_001130ef = 1;
      uVar9 = uVar9 | 4;
      goto LAB_00103b60;
    case 0x4c:
      if (2 < local_12c) goto LAB_00103ff4;
      local_f8[local_12c] = optarg;
      local_12c = local_12c + 1;
      goto LAB_00103b60;
    case 0x54:
      DAT_001130ee = 1;
      goto LAB_00103b60;
    case 0x58:
      DAT_001130ec = 1;
      uVar9 = uVar9 | 8;
      goto LAB_00103b60;
    case 0x61:
      DAT_001130f2 = 1;
      goto LAB_00103b60;
    case 0x65:
      uVar9 = uVar9 | 0x10;
      goto LAB_00103b60;
    case 0x69:
      DAT_001130ea = '\x01';
      goto LAB_00103b60;
    case 0x6d:
      DAT_001130e9 = 1;
      goto LAB_00103b60;
    case 0x76:
      uVar6 = FUN_001065f0("Randy Smith");
      FUN_00108010(stdout,"diff3","GNU diffutils",PTR_DAT_00113018,uVar6,0);
      FUN_00105c30();
      break;
    case 0x78:
      DAT_001130ec = 1;
      uVar9 = uVar9 | 0x20;
      goto LAB_00103b60;
    case 0x80:
      PTR_DAT_00113010 = optarg;
      goto LAB_00103b60;
    case 0x81:
      FUN_00104240();
      FUN_00105c30();
      break;
    case 0x82:
      goto switchD_00103b8f_caseD_82;
    }
    if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
      return 0;
    }
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  bVar11 = uVar9 != 0;
  uVar8 = (uint)bVar11;
  uVar7 = (uint)bVar11;
  DAT_001130f0 = (DAT_001130e9 ^ 1) & bVar11;
  bVar1 = uVar9 == 0 & DAT_001130e9;
  DAT_001130eb = DAT_001130eb | bVar1;
  DAT_001130ef = bVar1 | DAT_001130ef;
  if ((((uVar9 - 1 & uVar9) == 0) && ((DAT_001130ea == '\0' || (DAT_001130e9 == 0)))) &&
     ((local_12c == 0 || (DAT_001130ef != 0)))) {
    lVar4 = (long)optind;
    auVar12[8] = DAT_001130e9;
    auVar12._0_8_ = lVar4;
    auVar12._9_3_ = 0;
    if (param_1 - optind != 3) {
      uVar7 = (uint)bVar11;
      if (param_1 - optind < 3) {
        FUN_001041b0("missing operand after \'%s\'",param_2[(long)param_1 + -1]);
switchD_00103b8f_caseD_34:
        lVar4 = FUN_001041b0(0,0);
        uVar7 = uVar8;
      }
      auVar12 = FUN_001041b0("extra operand \'%s\'",param_2[lVar4 + 3]);
    }
    param_2 = param_2 + auVar12._0_8_;
    for (lVar4 = (long)local_12c; (int)lVar4 < 3; lVar4 = lVar4 + 1) {
      local_f8[lVar4] = param_2[lVar4];
    }
    uVar9 = (uVar7 | auVar12._8_4_) & 0xff;
    iVar2 = 2 - uVar9;
    lVar4 = (long)iVar2;
    iVar3 = strcmp((char *)param_2[lVar4],"-");
    if (iVar3 == 0) {
      iVar2 = uVar9 + 1;
      iVar3 = strcmp((char *)*param_2,"-");
      if (iVar3 == 0) goto LAB_00104010;
      lVar4 = (long)iVar2;
      iVar3 = strcmp((char *)param_2[lVar4],"-");
      if (iVar3 == 0) goto LAB_00104010;
    }
    local_10c = 3 - iVar2;
    local_110 = 0;
    local_104[0] = 0;
    local_104[local_10c] = 1;
    local_104[lVar4] = 2;
    puVar10 = param_2;
    local_108 = iVar2;
    while( true ) {
      __s1 = (char *)*puVar10;
      iVar2 = strcmp(__s1,"-");
      if (iVar2 != 0) break;
LAB_00103e45:
      puVar10 = puVar10 + 1;
      if (param_2 + 3 == puVar10) {
        signal(0x11,(__sighandler_t)0x0);
        uVar6 = param_2[local_104[2]];
        uVar5 = FUN_00105d70(param_2[local_104[1]],uVar6,local_128,&local_118);
        uVar6 = FUN_00105d70(param_2[local_104[0]],uVar6,local_128,&local_120);
        uVar6 = FUN_00104e40(uVar6);
        if (DAT_001130f0 == 0) {
          if (DAT_001130e9 == 0) {
            FUN_00105520(stdout,uVar6,&local_110,local_104);
            uVar9 = 0;
          }
          else {
            FUN_00108570(param_2[local_104[0]],"r",stdin);
            uVar9 = FUN_001057d0(stdin,stdout,uVar6,&local_110,local_104,local_f8[0]);
            iVar2 = ferror_unlocked(stdin);
            if (iVar2 != 0) {
                    /* WARNING: Subroutine does not return */
              FUN_00104210("read failed");
            }
          }
        }
        else {
          uVar9 = FUN_001050f0(stdout,uVar6,&local_110,local_104,local_f8[0],local_f8[1],local_f8[2]
                               ,uVar5);
        }
        free(local_120);
        free(local_118);
        FUN_00105c30();
                    /* WARNING: Subroutine does not return */
        exit(uVar9 & 0xff);
      }
    }
    iVar2 = stat(__s1,&local_d8);
    if (-1 < iVar2) {
      if ((local_d8.st_mode & 0xf000) == 0x4000) {
                    /* WARNING: Subroutine does not return */
        error(2,0x15,&DAT_0010d990,*puVar10);
      }
      goto LAB_00103e45;
    }
    FUN_001043a0(*puVar10);
LAB_00103ff4:
    FUN_001041b0("too many file label options",0);
  }
  FUN_001041b0("incompatible options",0);
LAB_00104010:
                    /* WARNING: Subroutine does not return */
  FUN_00104210("\'-\' specified for more than one input file");
switchD_00103b8f_caseD_82:
  DAT_001130f1 = 1;
  goto LAB_00103b60;
}




// Function: skipwhite @ 0x4110

void skipwhite(char *param_1)

{
  char cVar1;
  
  cVar1 = *param_1;
  if (cVar1 != ' ') goto LAB_0010412d;
  do {
    do {
      cVar1 = param_1[1];
      param_1 = param_1 + 1;
    } while (cVar1 == ' ');
LAB_0010412d:
  } while (cVar1 == '\t');
  return;
}




// Function: readnum @ 0x4140

byte * readnum(byte *param_1,long *param_2)

{
  byte *pbVar1;
  uint uVar2;
  
  uVar2 = (uint)*param_1;
  pbVar1 = (byte *)0x0;
  if (*param_1 - 0x30 < 10) {
    do {
      param_1 = param_1 + 1;
      pbVar1 = (byte *)((long)(int)(uVar2 - 0x30) + (long)pbVar1 * 10);
      uVar2 = (uint)*param_1;
    } while (uVar2 - 0x30 < 10);
    *param_2 = (long)pbVar1;
    return param_1;
  }
  return pbVar1;
}




// Function: reverse_diff3_blocklist @ 0x4180

void reverse_diff3_blocklist(long param_1)

{
  long lVar1;
  long lVar2;
  
  if (param_1 != 0) {
    lVar2 = 0;
    do {
      lVar1 = param_1;
      param_1 = *(long *)(lVar1 + 0x68);
      *(long *)(lVar1 + 0x68) = lVar2;
      lVar2 = lVar1;
    } while (param_1 != 0);
  }
  return;
}




// Function: try_help @ 0x41b0

void try_help(long param_1,undefined8 param_2)

{
  undefined8 uVar1;
  undefined8 uVar2;
  
  uVar1 = DAT_00123140;
  if (param_1 != 0) {
    uVar1 = dcgettext(0,param_1,5);
                    /* WARNING: Subroutine does not return */
    error(0,0,uVar1,param_2);
  }
  uVar2 = dcgettext(0,"Try \'%s --help\' for more information.",5);
                    /* WARNING: Subroutine does not return */
  error(2,0,uVar2,uVar1);
}




// Function: fatal @ 0x4210

void fatal(undefined8 param_1)

{
  undefined8 uVar1;
  
  uVar1 = dcgettext(0,param_1,5);
                    /* WARNING: Subroutine does not return */
  error(2,0,&DAT_0010d990,uVar1);
}




// Function: usage @ 0x4240

void usage(void)

{
  FILE *pFVar1;
  char cVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  char *pcVar5;
  undefined **ppuVar6;
  
  uVar4 = DAT_00123140;
  ppuVar6 = &PTR_s__A____show_all_output_all_change_00112880;
  uVar3 = dcgettext(0,"Usage: %s [OPTION]... MYFILE OLDFILE YOURFILE\n",5);
  __printf_chk(1,uVar3,uVar4);
  uVar4 = dcgettext(0,"Compare three files line by line.",5);
  __printf_chk(1,&DAT_0010d993,uVar4);
  pFVar1 = stdout;
  pcVar5 = (char *)dcgettext(0,
                             "Mandatory arguments to long options are mandatory for short options too.\n"
                             ,5);
  fputs_unlocked(pcVar5,pFVar1);
  cVar2 = '-';
  pcVar5 = "-A, --show-all              output all changes, bracketing conflicts";
  while( true ) {
    if (cVar2 == '\0') {
      putchar_unlocked(10);
      pcVar5 = ppuVar6[1];
      pFVar1 = stdout;
    }
    else {
      uVar4 = dcgettext(0,pcVar5,5);
      __printf_chk(1,"  %s\n",uVar4);
      pcVar5 = ppuVar6[1];
      pFVar1 = stdout;
    }
    stdout = pFVar1;
    if (pcVar5 == (char *)0x0) break;
    ppuVar6 = ppuVar6 + 1;
    cVar2 = *pcVar5;
  }
  pcVar5 = (char *)dcgettext(0,
                             "\nThe default output format is a somewhat human-readable representation of\nthe changes.\n\nThe -e, -E, -x, -X (and corresponding long) options cause an ed script\nto be output instead of the default.\n\nFinally, the -m (--merge) option causes diff3 to do the merge internally\nand output the actual merged file.  For unusual input, this is more\nrobust than using ed.\n"
                             ,5);
  fputs_unlocked(pcVar5,pFVar1);
  uVar4 = dcgettext(0,"Exit status is 0 if successful, 1 if conflicts, 2 if trouble.",5);
  uVar3 = dcgettext(0,"If a FILE is \'-\', read standard input.",5);
  __printf_chk(1,"\n%s\n%s\n",uVar3,uVar4);
  FUN_001080c0();
  return;
}




// Function: perror_with_exit @ 0x43a0

void perror_with_exit(undefined8 param_1)

{
  int *piVar1;
  
  piVar1 = __errno_location();
                    /* WARNING: Subroutine does not return */
  error(2,*piVar1,&DAT_0010d990,param_1);
}




// Function: read_diff @ 0x43d0

ulong read_diff(undefined *param_1,undefined *param_2,long *param_3)

{
  undefined *puVar1;
  int iVar2;
  __pid_t _Var3;
  int iVar4;
  undefined **ppuVar5;
  undefined **ppuVar6;
  long lVar7;
  long lVar8;
  int *piVar9;
  undefined8 uVar10;
  FILE *__stream;
  ulong uVar11;
  ulong uVar12;
  long extraout_RDX;
  uint uVar13;
  long lVar14;
  char cVar15;
  char *pcVar16;
  long lVar17;
  long in_FS_OFFSET;
  uint local_134;
  int local_130;
  int local_12c;
  char *local_128;
  undefined *local_120 [9];
  stat local_d8;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_128 = PTR_DAT_00113010;
  ppuVar5 = local_120;
  if (DAT_001130f2 != '\0') {
    local_120[0] = &DAT_0010d9a6;
    ppuVar5 = local_120 + 1;
  }
  ppuVar6 = ppuVar5;
  if (DAT_001130f1 != '\0') {
    ppuVar6 = ppuVar5 + 1;
    *ppuVar5 = "--strip-trailing-cr";
  }
  ppuVar6[2] = param_1;
  *ppuVar6 = "--horizon-lines=100";
  ppuVar6[1] = &DAT_0010d9d1;
  ppuVar6[3] = param_2;
  ppuVar6[4] = (undefined *)0x0;
  iVar2 = pipe(&local_130);
  if (iVar2 != 0) {
    FUN_001043a0(&DAT_0010d9d4);
LAB_0010461c:
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  _Var3 = fork();
  if (_Var3 != 0) {
    if (_Var3 != -1) {
      close(local_12c);
      iVar2 = local_130;
      iVar4 = fstat(local_130,&local_d8);
      if (iVar4 == 0) {
        uVar11 = 1;
        if (0 < local_d8.st_blksize) {
          uVar11 = local_d8.st_blksize;
        }
        lVar14 = 0;
        lVar7 = FUN_00108180(uVar11);
        uVar12 = local_d8.st_blksize;
        while( true ) {
          lVar17 = uVar11 - lVar14;
          lVar8 = FUN_00108770(iVar2,lVar7 + lVar14,lVar17);
          lVar14 = lVar14 + lVar8;
          if (lVar17 != lVar8) break;
          if (0x3ffffffffffffffe < uVar11) {
                    /* WARNING: Subroutine does not return */
            FUN_00108530();
          }
          uVar11 = uVar11 * 2;
          lVar7 = FUN_001081d0(lVar7,uVar11);
        }
        if (lVar8 == -1) {
          cVar15 = -0x1c;
          __stream = (FILE *)dcgettext(0,"read failed",5);
          FUN_001043a0();
          uVar13 = fputs_unlocked(".\n",__stream);
          if (cVar15 == '\0') {
            return (ulong)uVar13;
          }
          if (uVar12 == 1) {
            uVar11 = __fprintf_chk(__stream,1,"%lds/^\\.//\n",extraout_RDX);
            return uVar11;
          }
          uVar11 = __fprintf_chk(__stream,1,&DAT_0010d9fe,extraout_RDX,extraout_RDX + -1 + uVar12);
          return uVar11;
        }
        if ((lVar14 == 0) || (*(char *)(lVar7 + -1 + lVar14) == '\n')) {
          *param_3 = lVar7;
          iVar2 = close(iVar2);
          if (iVar2 == 0) {
            _Var3 = waitpid(_Var3,(int *)&local_134,0);
            puVar1 = PTR_DAT_00113010;
            if (-1 < _Var3) {
              if ((local_134 & 0x7f) == 0) {
                if (((int)local_134 >> 8 & 0xfeU) == 0) {
                  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
                    return lVar7 + lVar14;
                  }
                  goto LAB_0010461c;
                }
                uVar13 = (int)local_134 >> 8 & 0xff;
                if ((char)(local_134 >> 8) == '~') {
                  pcVar16 = "subsidiary program \'%s\' could not be invoked";
                }
                else {
                  pcVar16 = "subsidiary program \'%s\' failed (exit status %d)";
                  if (uVar13 == 0x7f) {
                    pcVar16 = "subsidiary program \'%s\' not found";
                  }
                }
              }
              else {
                uVar13 = 0x7fffffff;
                pcVar16 = "subsidiary program \'%s\' failed";
              }
              uVar10 = dcgettext(0,pcVar16,5);
                    /* WARNING: Subroutine does not return */
              error(2,0,uVar10,puVar1,uVar13);
            }
            FUN_001043a0("waitpid");
          }
          FUN_001043a0("close");
        }
                    /* WARNING: Subroutine does not return */
        FUN_00104210("invalid diff format; incomplete last line");
      }
      FUN_001043a0("fstat");
    }
    FUN_001043a0(&DAT_0010d9d9);
  }
  close(local_130);
  if (local_12c != 1) {
    dup2(local_12c,1);
    close(local_12c);
  }
  execvp(PTR_DAT_00113010,&local_128);
  piVar9 = __errno_location();
                    /* WARNING: Subroutine does not return */
  _exit((*piVar9 == 2) + 0x7e);
}




// Function: undotlines @ 0x46d0

void undotlines(FILE *param_1,char param_2,long param_3,long param_4)

{
  fputs_unlocked(".\n",param_1);
  if (param_2 == '\0') {
    return;
  }
  if (param_4 != 1) {
    __fprintf_chk(param_1,1,&DAT_0010d9fe,param_3,param_3 + -1 + param_4);
    return;
  }
  __fprintf_chk(param_1,1,"%lds/^\\.//\n",param_3);
  return;
}




// Function: scan_diff_line @ 0x4770

char * scan_diff_line(char *param_1,ulong *param_2,long *param_3,char *param_4,char param_5)

{
  char cVar1;
  char cVar2;
  char *pcVar3;
  char *pcVar4;
  
  if ((*param_1 == param_5) && (param_1[1] == ' ')) {
    param_1 = param_1 + 2;
    *param_2 = (ulong)param_1;
    pcVar3 = param_1;
    do {
      pcVar4 = pcVar3;
      pcVar3 = pcVar4 + 1;
    } while (*pcVar4 != '\n');
    *param_3 = (long)pcVar3 - (long)param_1;
    cVar2 = DAT_001130f0;
    if ((pcVar3 < param_4) && (*pcVar3 == '\\')) {
      if (DAT_001130f0 == '\0') {
        *param_3 = ((long)pcVar3 - (long)param_1) + -1;
      }
      else {
        __fprintf_chk(stderr,1,&DAT_0010da0e,DAT_00123140);
        cVar2 = DAT_001130f0;
      }
      pcVar4 = pcVar4 + 2;
      do {
        if (cVar2 != '\0') {
          putc_unlocked((int)*pcVar4,stderr);
        }
        pcVar3 = pcVar4 + 1;
        cVar1 = *pcVar4;
        pcVar4 = pcVar3;
        cVar2 = DAT_001130f0;
      } while (cVar1 != '\n');
    }
    return pcVar3;
  }
                    /* WARNING: Subroutine does not return */
  FUN_00104210("invalid diff format; incorrect leading line chars");
}




// Function: create_diff3_block @ 0x4850

undefined4 *
create_diff3_block(long param_1,long param_2,long param_3,long param_4,undefined8 param_5,
            undefined8 param_6)

{
  undefined4 *puVar1;
  undefined8 uVar2;
  long lVar3;
  
  puVar1 = (undefined4 *)FUN_00108180(0x70);
  *(long *)(puVar1 + 4) = param_2;
  *puVar1 = 0;
  *(undefined8 *)(puVar1 + 0x1a) = 0;
  *(long *)(puVar1 + 2) = param_1;
  *(long *)(puVar1 + 6) = param_3;
  *(long *)(puVar1 + 8) = param_4;
  *(undefined8 *)(puVar1 + 10) = param_5;
  *(undefined8 *)(puVar1 + 0xc) = param_6;
  lVar3 = (param_2 - param_1) + 1;
  if (lVar3 == 0) {
    param_4 = param_4 - param_3;
    *(undefined8 *)(puVar1 + 0xe) = 0;
    *(undefined8 *)(puVar1 + 0x14) = 0;
  }
  else {
    uVar2 = FUN_00108410(lVar3,8);
    *(undefined8 *)(puVar1 + 0xe) = uVar2;
    uVar2 = FUN_00108410(lVar3,8);
    *(undefined8 *)(puVar1 + 0x14) = uVar2;
    param_4 = *(long *)(puVar1 + 8) - *(long *)(puVar1 + 6);
  }
  param_4 = param_4 + 1;
  if (param_4 == 0) {
    *(undefined8 *)(puVar1 + 0x10) = 0;
    uVar2 = 0;
  }
  else {
    uVar2 = FUN_00108410(param_4,8);
    *(undefined8 *)(puVar1 + 0x10) = uVar2;
    uVar2 = FUN_00108410(param_4,8);
  }
  *(undefined8 *)(puVar1 + 0x16) = uVar2;
  lVar3 = (*(long *)(puVar1 + 0xc) - *(long *)(puVar1 + 10)) + 1;
  if (lVar3 == 0) {
    *(undefined8 *)(puVar1 + 0x12) = 0;
    uVar2 = 0;
  }
  else {
    uVar2 = FUN_00108410(lVar3,8);
    *(undefined8 *)(puVar1 + 0x12) = uVar2;
    uVar2 = FUN_00108410(lVar3,8);
  }
  *(undefined8 *)(puVar1 + 0x18) = uVar2;
  return puVar1;
}




// Function: copy_stringlist @ 0x49b0

undefined8 copy_stringlist(long param_1,long param_2,long param_3,long param_4,long param_5)

{
  void *__s2;
  size_t __n;
  int iVar1;
  long lVar2;
  
  if (param_5 == 0) {
    return 1;
  }
  lVar2 = 0;
  while( true ) {
    while( true ) {
      __s2 = *(void **)(param_3 + lVar2 * 8);
      __n = *(size_t *)(param_2 + lVar2 * 8);
      if (__s2 != (void *)0x0) break;
      *(undefined8 *)(param_3 + lVar2 * 8) = *(undefined8 *)(param_1 + lVar2 * 8);
      *(size_t *)(param_4 + lVar2 * 8) = __n;
      lVar2 = lVar2 + 1;
      if (lVar2 == param_5) {
        return 1;
      }
    }
    if (*(size_t *)(param_4 + lVar2 * 8) != __n) {
      return 0;
    }
    iVar1 = memcmp(*(void **)(param_1 + lVar2 * 8),__s2,__n);
    if (iVar1 != 0) break;
    lVar2 = lVar2 + 1;
    if (lVar2 == param_5) {
      return 1;
    }
  }
  return 0;
}




// Function: compare_line_list @ 0x4a50

undefined8 compare_line_list(long param_1,long param_2,long param_3,long param_4,long param_5)

{
  void *__s1;
  void *__s2;
  size_t __n;
  int iVar1;
  long lVar2;
  
  lVar2 = 0;
  while( true ) {
    if (param_5 == lVar2) {
      return 1;
    }
    __s1 = *(void **)(param_1 + lVar2 * 8);
    if (__s1 == (void *)0x0) {
      return 0;
    }
    __s2 = *(void **)(param_3 + lVar2 * 8);
    if (__s2 == (void *)0x0) {
      return 0;
    }
    __n = *(size_t *)(param_2 + lVar2 * 8);
    if (__n != *(size_t *)(param_4 + lVar2 * 8)) break;
    iVar1 = memcmp(__s1,__s2,__n);
    lVar2 = lVar2 + 1;
    if (iVar1 != 0) {
      return 0;
    }
  }
  return 0;
}




// Function: using_to_diff3_block @ 0x4ae0

undefined4 * using_to_diff3_block(long *param_1,long param_2,int param_3,int param_4,long param_5)

{
  long *plVar1;
  long *plVar2;
  long lVar3;
  char cVar4;
  long lVar5;
  undefined4 *puVar6;
  long lVar7;
  long lVar8;
  long lVar9;
  long lVar10;
  long *plVar11;
  long lVar12;
  long in_FS_OFFSET;
  long local_90;
  long local_68 [7];
  
  local_68[5] = *(long *)(in_FS_OFFSET + 0x28);
  lVar3 = *(long *)(param_1[param_3] + 0x10);
  lVar12 = *(long *)(*(long *)(param_2 + (long)param_4 * 8) + 0x18);
  lVar5 = 0;
  do {
    plVar11 = *(long **)((long)param_1 + lVar5);
    if (plVar11 == (long *)0x0) {
      *(long *)((long)local_68 + lVar5) =
           (lVar3 - *(long *)(param_5 + 0x30)) + *(long *)(param_5 + 0x10 + lVar5 * 2);
      lVar8 = (lVar12 - *(long *)(param_5 + 0x30)) + *(long *)(param_5 + 0x10 + lVar5 * 2);
    }
    else {
      *(long *)((long)local_68 + lVar5) = (lVar3 - plVar11[2]) + *plVar11;
      lVar8 = (lVar12 - *(long *)(*(long *)(param_2 + lVar5) + 0x18)) +
              *(long *)(*(long *)(param_2 + lVar5) + 8);
    }
    *(long *)((long)local_68 + lVar5 + 0x10) = lVar8;
    lVar5 = lVar5 + 8;
  } while (lVar5 != 0x10);
  puVar6 = (undefined4 *)FUN_00104850(local_68[0],local_68[2],local_68[1],local_68[3],lVar3);
  local_90 = 0;
  while( true ) {
    lVar12 = param_1[local_90];
    if (lVar12 != 0) {
      lVar5 = *(long *)(puVar6 + 0x18);
      lVar8 = *(long *)(puVar6 + 0x12);
      do {
        lVar9 = (*(long *)(lVar12 + 0x10) - lVar3) * 8;
        cVar4 = FUN_001049b0(*(undefined8 *)(lVar12 + 0x28),*(undefined8 *)(lVar12 + 0x38),
                             lVar9 + lVar8,lVar5 + lVar9,
                             (*(long *)(lVar12 + 0x18) - *(long *)(lVar12 + 0x10)) + 1);
        if (cVar4 == '\0') goto LAB_00104bff;
        lVar12 = *(long *)(lVar12 + 0x40);
      } while (lVar12 != 0);
    }
    if (local_90 == 1) break;
    local_90 = 1;
  }
  lVar12 = local_68[0];
  lVar5 = 0;
  while( true ) {
    plVar11 = *(long **)((long)param_1 + lVar5);
    lVar9 = 0;
    lVar8 = lVar12;
    while( true ) {
      lVar10 = local_68[2] + 1;
      if (plVar11 != (long *)0x0) {
        lVar10 = *plVar11;
      }
      if (lVar10 <= lVar8) break;
      lVar8 = lVar8 + 1;
      *(undefined8 *)(*(long *)((long)puVar6 + lVar5 + 0x38) + lVar9) =
           *(undefined8 *)(*(long *)(puVar6 + 0x12) + lVar9);
      *(undefined8 *)(*(long *)((long)puVar6 + lVar5 + 0x50) + lVar9) =
           *(undefined8 *)(*(long *)(puVar6 + 0x18) + lVar9);
      lVar9 = lVar9 + 8;
    }
    if (plVar11 != (long *)0x0) break;
LAB_00104d8b:
    if (lVar5 + 8 == 0x10) {
      if (*param_1 == 0) {
        *puVar6 = 6;
      }
      else if (param_1[1] == 0) {
        *puVar6 = 5;
      }
      else if ((*(long *)(puVar6 + 4) - *(long *)(puVar6 + 2) ==
                *(long *)(puVar6 + 8) - *(long *)(puVar6 + 6)) &&
              (cVar4 = FUN_00104a50(*(undefined8 *)(puVar6 + 0xe),*(undefined8 *)(puVar6 + 0x14),
                                    *(undefined8 *)(puVar6 + 0x10),*(undefined8 *)(puVar6 + 0x16),
                                    (*(long *)(puVar6 + 4) - *(long *)(puVar6 + 2)) + 1),
              cVar4 != '\0')) {
        *puVar6 = 7;
      }
      else {
        *puVar6 = 4;
      }
LAB_00104c02:
      if (local_68[5] != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
        __stack_chk_fail();
      }
      return puVar6;
    }
    lVar12 = *(long *)((long)local_68 + lVar5 + 8);
    local_68[2] = *(long *)((long)local_68 + lVar5 + 0x18);
    lVar5 = lVar5 + 8;
  }
  lVar8 = *(long *)((long)puVar6 + lVar5 + 0x38);
  lVar9 = *(long *)((long)puVar6 + lVar5 + 0x50);
LAB_00104cf0:
  lVar10 = (*plVar11 - lVar12) * 8;
  cVar4 = FUN_001049b0(plVar11[4],plVar11[6],lVar10 + lVar8,lVar9 + lVar10,
                       (plVar11[1] - *plVar11) + 1);
  if (cVar4 != '\0') {
    plVar1 = plVar11 + 1;
    plVar2 = plVar11 + 3;
    plVar11 = (long *)plVar11[8];
    lVar10 = (*plVar2 - lVar3) * 8;
    lVar7 = (*plVar1 + 1) - lVar12;
    do {
      lVar10 = lVar10 + 8;
      if (plVar11 == (long *)0x0) {
        if ((local_68[2] + 1) - lVar12 <= lVar7) goto LAB_00104d8b;
      }
      else if (*plVar11 - lVar12 <= lVar7) goto LAB_00104cf0;
      *(undefined8 *)(lVar8 + lVar7 * 8) = *(undefined8 *)(*(long *)(puVar6 + 0x12) + lVar10);
      *(undefined8 *)(lVar9 + lVar7 * 8) = *(undefined8 *)(*(long *)(puVar6 + 0x18) + lVar10);
      lVar7 = lVar7 + 1;
    } while( true );
  }
LAB_00104bff:
  puVar6 = (undefined4 *)0x0;
  goto LAB_00104c02;
}




// Function: make_3way_diff @ 0x4e40

long make_3way_diff(long param_1,long param_2)

{
  long lVar1;
  long lVar2;
  uint uVar3;
  long lVar5;
  long *plVar6;
  ulong uVar7;
  undefined *puVar8;
  uint uVar9;
  uint uVar10;
  long in_FS_OFFSET;
  long local_50;
  long local_48 [5];
  long local_20;
  long local_10;
  ulong uVar4;
  
  puVar8 = &DAT_0010dd60;
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  plVar6 = &local_50;
  local_48[4] = param_1;
  local_50 = 0;
  local_20 = param_2;
  if (param_1 == 0) goto LAB_00104fab;
  do {
    local_48[3] = 0;
    local_48[2] = 0;
    local_48[1] = 0;
    local_48[0] = 0;
    if (local_20 == 0) {
      uVar4 = 1;
      uVar10 = 0;
      uVar7 = 0;
    }
    else {
      lVar1 = *(long *)(local_20 + 0x10);
      lVar5 = *(long *)(param_1 + 0x10);
      uVar10 = (uint)(lVar1 < lVar5);
      uVar7 = (ulong)(lVar1 < lVar5);
      uVar4 = (ulong)(lVar5 <= lVar1);
      param_1 = local_48[uVar7 + 4];
    }
    while( true ) {
      local_48[3] = 0;
      local_48[2] = 0;
      local_48[1] = 0;
      local_48[0] = 0;
      lVar1 = *(long *)(param_1 + 0x40);
      lVar5 = *(long *)(param_1 + 0x18);
      local_48[uVar7 + 2] = param_1;
      local_48[uVar7] = param_1;
      local_48[uVar7 + 4] = lVar1;
      *(undefined8 *)(param_1 + 0x40) = 0;
      lVar1 = local_48[uVar4 + 4];
      uVar9 = uVar10;
      while ((lVar1 != 0 && (*(long *)(lVar1 + 0x10) <= lVar5 + 1))) {
        if (local_48[uVar4] == 0) {
          local_48[uVar4] = lVar1;
        }
        else {
          *(long *)(local_48[uVar4 + 2] + 0x40) = lVar1;
        }
        local_48[uVar4 + 2] = lVar1;
        lVar2 = *(long *)(local_48[uVar4 + 4] + 0x40);
        *(undefined8 *)(lVar1 + 0x40) = 0;
        lVar1 = *(long *)(lVar1 + 0x18);
        local_48[uVar4 + 4] = lVar2;
        uVar3 = uVar9 ^ 1;
        if (lVar5 < lVar1) {
          lVar5 = lVar1;
          uVar3 = uVar9;
          uVar9 = uVar9 ^ 1;
        }
        uVar4 = (ulong)(int)uVar3;
        lVar1 = local_48[uVar4 + 4];
      }
      puVar8 = (undefined *)FUN_00104ae0(local_48,local_48 + 2,uVar10,uVar9,puVar8);
      param_1 = local_48[4];
      if (puVar8 == (undefined *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_00104210("internal error: screwup in format of diff blocks");
      }
      *plVar6 = (long)puVar8;
      plVar6 = (long *)(puVar8 + 0x68);
      if (param_1 != 0) break;
LAB_00104fab:
      if (local_20 == 0) {
        if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
          return local_50;
        }
                    /* WARNING: Subroutine does not return */
        __stack_chk_fail();
      }
      uVar7 = 1;
      uVar4 = 0;
      local_48[3] = 0;
      local_48[2] = 0;
      uVar10 = 1;
      local_48[1] = 0;
      local_48[0] = 0;
      param_1 = local_20;
    }
  } while( true );
}




// Function: dotlines @ 0x5030

undefined1 dotlines(FILE *param_1,long param_2,int param_3)

{
  long lVar1;
  char *__ptr;
  long lVar2;
  long lVar3;
  long lVar4;
  undefined1 local_41;
  
  lVar3 = (long)param_3;
  lVar4 = lVar3 * 0x10 + param_2;
  if (*(long *)(lVar4 + 0x10) < *(long *)(lVar4 + 8)) {
    local_41 = 0;
  }
  else {
    local_41 = 0;
    lVar2 = 0;
    do {
      lVar1 = lVar2 * 8;
      __ptr = *(char **)(*(long *)(param_2 + 0x38 + lVar3 * 8) + lVar2 * 8);
      if (*__ptr == '.') {
        fputc_unlocked(0x2e,param_1);
        local_41 = 1;
      }
      lVar2 = lVar2 + 1;
      fwrite_unlocked(__ptr,1,*(size_t *)(*(long *)(param_2 + 0x50 + lVar3 * 8) + lVar1),param_1);
    } while (lVar2 <= *(long *)(lVar4 + 0x10) - *(long *)(lVar4 + 8));
  }
  return local_41;
}




// Function: output_diff3_edscript @ 0x50f0

undefined8
output_diff3_edscript(FILE *param_1,undefined8 param_2,int *param_3,long param_4,undefined8 param_5,
            undefined8 param_6,undefined8 param_7)

{
  long lVar1;
  undefined1 uVar2;
  byte bVar3;
  byte bVar4;
  int *piVar5;
  long lVar6;
  long lVar7;
  long lVar8;
  int iVar9;
  undefined8 uVar10;
  
  piVar5 = (int *)FUN_00104180(param_2);
  if (piVar5 == (int *)0x0) {
    uVar10 = 0;
  }
  else {
    uVar10 = 0;
LAB_0010515f:
    do {
      if (*piVar5 == 4) {
LAB_00105167:
        if (DAT_001130ed == '\0') {
          lVar8 = *(long *)(piVar5 + (long)*param_3 * 4 + 2);
          lVar7 = *(long *)(piVar5 + (long)*param_3 * 4 + 4);
          if (DAT_001130ef == '\0') {
LAB_0010518f:
            if (*(long *)(piVar5 + (long)param_3[2] * 4 + 4) -
                *(long *)(piVar5 + (long)param_3[2] * 4 + 2) != -1) {
              if (lVar7 - lVar8 == -1) {
                __fprintf_chk(param_1,1,"%lda\n",lVar7);
              }
              else if (lVar7 == lVar8) {
                __fprintf_chk(param_1,1,"%ldc\n",lVar7);
              }
              else {
                __fprintf_chk(param_1,1,&DAT_0010da54,lVar8);
              }
              lVar6 = (long)param_3[2];
              lVar7 = *(long *)(piVar5 + lVar6 * 4 + 4);
              lVar1 = *(long *)(piVar5 + lVar6 * 4 + 2);
              uVar2 = FUN_00105030(param_1,piVar5,lVar6);
              FUN_001046d0(param_1,uVar2,lVar8,(lVar7 - lVar1) + 1);
              piVar5 = *(int **)(piVar5 + 0x1a);
              if (piVar5 == (int *)0x0) break;
              goto LAB_0010515f;
            }
            if (lVar8 == lVar7) {
              __fprintf_chk(param_1,1,"%ldd\n",lVar8);
            }
            else {
              __fprintf_chk(param_1,1,&DAT_0010da4a,lVar8);
            }
          }
          else {
            __fprintf_chk(param_1,1,"%lda\n",lVar7);
            bVar4 = 0;
            if (DAT_001130eb != '\0') {
              __fprintf_chk(param_1,1,"||||||| %s\n",param_6);
              bVar4 = FUN_00105030(param_1,piVar5,param_3[1]);
            }
            fputs_unlocked("=======\n",param_1);
            bVar3 = FUN_00105030(param_1,piVar5,param_3[2]);
            bVar4 = bVar4 | bVar3;
            iVar9 = 4;
LAB_001052b9:
            __fprintf_chk(param_1,1,">>>>>>> %s\n",param_7);
            FUN_001046d0(param_1,bVar4,lVar7 + 2,
                         (*(long *)(piVar5 + (long)param_3[1] * 4 + 4) -
                         *(long *)(piVar5 + (long)param_3[1] * 4 + 2)) + 3 +
                         (*(long *)(piVar5 + (long)param_3[2] * 4 + 4) -
                         *(long *)(piVar5 + (long)param_3[2] * 4 + 2)));
            if (iVar9 == 4) {
              __fprintf_chk(param_1,1,"%lda\n<<<<<<< %s\n",lVar8 + -1,param_5);
              uVar2 = 0;
            }
            else {
              __fprintf_chk(param_1,1,"%lda\n<<<<<<< %s\n",lVar8 + -1,param_6);
              uVar2 = 0;
              if (iVar9 == 6) {
                uVar2 = FUN_00105030(param_1,piVar5,param_3[1]);
                fputs_unlocked("=======\n",param_1);
              }
            }
            uVar10 = 1;
            FUN_001046d0(param_1,uVar2,lVar8 + 1,
                         (*(long *)(piVar5 + (long)param_3[1] * 4 + 4) -
                         *(long *)(piVar5 + (long)param_3[1] * 4 + 2)) + 1);
          }
        }
      }
      else {
        iVar9 = *(int *)(param_4 + (ulong)(*piVar5 - 5) * 4);
        if (iVar9 == 1) {
          if (DAT_001130eb != '\0') {
            iVar9 = 6;
            lVar7 = *(long *)(piVar5 + (long)*param_3 * 4 + 4);
            lVar8 = *(long *)(piVar5 + (long)*param_3 * 4 + 2);
            __fprintf_chk(param_1,1,"%lda\n",lVar7);
            bVar4 = 0;
            goto LAB_001052b9;
          }
        }
        else if (iVar9 == 2) {
          if (DAT_001130ec == '\0') {
            lVar8 = *(long *)(piVar5 + (long)*param_3 * 4 + 2);
            lVar7 = *(long *)(piVar5 + (long)*param_3 * 4 + 4);
            goto LAB_0010518f;
          }
        }
        else if (iVar9 == -1) goto LAB_00105167;
      }
      piVar5 = *(int **)(piVar5 + 0x1a);
    } while (piVar5 != (int *)0x0);
  }
  if (DAT_001130ea != '\0') {
    fputs_unlocked("w\nq\n",param_1);
  }
  return uVar10;
}




// Function: output_diff3 @ 0x5520

void output_diff3(FILE *param_1,int *param_2,long param_3,long param_4)

{
  long lVar1;
  long lVar2;
  void *__ptr;
  size_t __n;
  uint uVar3;
  uint uVar4;
  char *__s;
  undefined8 uVar5;
  long lVar6;
  long lVar7;
  long in_FS_OFFSET;
  int local_58;
  uint local_54;
  char local_42;
  undefined1 local_41;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  __s = "  ";
  if (DAT_001130ee != '\0') {
    __s = "\t";
  }
  for (; param_2 != (int *)0x0; param_2 = *(int **)(param_2 + 0x1a)) {
    if (*param_2 == 4) {
      local_42 = '\0';
      local_54 = 3;
      local_58 = 3;
    }
    else {
      uVar4 = *param_2 - 5;
      if (2 < uVar4) {
                    /* WARNING: Subroutine does not return */
        FUN_00104210("internal error: invalid diff type passed to output");
      }
      local_41 = 0;
      local_58 = *(int *)(param_4 + (ulong)uVar4 * 4);
      local_42 = (char)local_58 + '1';
      local_54 = (uint)(local_58 == 0);
    }
    __fprintf_chk(param_1,1,"====%s\n",&local_42);
    uVar4 = 0;
    do {
      lVar6 = (long)*(int *)(param_3 + (long)(int)uVar4 * 4);
      lVar1 = *(long *)(param_2 + lVar6 * 4 + 2);
      lVar2 = *(long *)(param_2 + lVar6 * 4 + 4);
      uVar3 = uVar4 + 1;
      __fprintf_chk(param_1,1,&DAT_0010da70,uVar3);
      if (lVar1 == lVar2) {
        __fprintf_chk(param_1,1,"%ldc\n",lVar1);
      }
      else if (lVar1 - lVar2 == 1) {
        __fprintf_chk(param_1,1,"%lda\n",lVar1 + -1);
      }
      else {
        __fprintf_chk(param_1,1,&DAT_0010da54,lVar1,lVar2);
      }
      if ((lVar1 <= lVar2) && (local_54 != uVar4)) {
        lVar7 = 0;
        do {
          fputs_unlocked(__s,param_1);
          __ptr = *(void **)(*(long *)(param_2 + (lVar6 + 6) * 2 + 2) + lVar7 * 8);
          __n = *(size_t *)(*(long *)(param_2 + lVar6 * 2 + 0x14) + lVar7 * 8);
          lVar7 = lVar7 + 1;
          fwrite_unlocked(__ptr,1,__n,param_1);
        } while (lVar7 <= lVar2 - lVar1);
        if (*(char *)((long)__ptr + (__n - 1)) != '\n') {
          uVar5 = dcgettext(0,"No newline at end of file",5);
          __fprintf_chk(param_1,1,"\n\\ %s\n",uVar5);
        }
      }
      if (local_58 == 1) {
        uVar3 = *(uint *)(&DAT_0010dd40 + (long)(int)uVar4 * 4);
      }
      uVar4 = uVar3;
    } while ((int)uVar3 < 3);
  }
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: output_diff3_merge @ 0x57d0

undefined1
output_diff3_merge(FILE *param_1,FILE *param_2,int *param_3,int *param_4,long param_5,undefined8 param_6,
            undefined8 param_7,undefined8 param_8)

{
  long lVar1;
  long lVar2;
  int iVar3;
  int iVar4;
  int iVar5;
  long lVar6;
  undefined8 uVar7;
  long lVar8;
  bool bVar9;
  char *local_60;
  long local_58;
  int local_50;
  char local_4a;
  undefined1 local_49;
  
  if (param_3 != (int *)0x0) {
    lVar6 = 0;
    local_49 = 0;
    iVar3 = *param_3;
    if (iVar3 == 4) goto LAB_0010584a;
    do {
      iVar3 = *(int *)(param_5 + (ulong)(iVar3 - 5) * 4);
      if (iVar3 == 1) {
        local_4a = DAT_001130eb;
        if (DAT_001130eb != '\0') {
          local_50 = 6;
          local_58 = *(long *)(param_3 + 2) + -1;
          lVar6 = (*(long *)(param_3 + 2) - lVar6) + -2;
          local_60 = "<<<<<<< %s\n";
          if (-1 < lVar6) goto LAB_00105898;
          goto LAB_00105aef;
        }
      }
      else if (iVar3 == 2) {
        local_4a = DAT_001130ec;
        if (DAT_001130ec == '\0') {
          local_50 = 7;
          local_58 = *(long *)(param_3 + 2) + -1;
          lVar6 = (*(long *)(param_3 + 2) - lVar6) + -2;
          local_60 = "<<<<<<< %s\n";
          if (-1 < lVar6) goto LAB_00105898;
          goto LAB_001058fa;
        }
      }
      else if (iVar3 == -1) goto LAB_0010584a;
      while( true ) {
        param_3 = *(int **)(param_3 + 0x1a);
        if (param_3 == (int *)0x0) goto LAB_001059ac;
        iVar3 = *param_3;
        if (iVar3 != 4) break;
LAB_0010584a:
        if (DAT_001130ed == '\0') {
          local_4a = DAT_001130ef;
          local_58 = *(long *)(param_3 + 2) + -1;
          lVar6 = (*(long *)(param_3 + 2) - lVar6) + -2;
          if (lVar6 < 0) {
            if (DAT_001130ef != '\0') {
              local_60 = "||||||| %s\n";
LAB_00105b9f:
              __fprintf_chk(param_2,1,"<<<<<<< %s\n",param_6);
              for (lVar6 = 0; lVar8 = (long)*param_4,
                  lVar6 <= *(long *)(param_3 + lVar8 * 4 + 4) - *(long *)(param_3 + lVar8 * 4 + 2);
                  lVar6 = lVar6 + 1) {
                fwrite_unlocked(*(void **)(*(long *)(param_3 + lVar8 * 2 + 0xe) + lVar6 * 8),1,
                                *(size_t *)(*(long *)(param_3 + lVar8 * 2 + 0x14) + lVar6 * 8),
                                param_2);
              }
              goto LAB_001058d4;
            }
          }
          else {
            local_50 = 4;
            local_60 = "||||||| %s\n";
LAB_00105898:
            do {
              while (iVar3 = getc_unlocked(param_1), iVar3 == -1) {
                iVar3 = ferror_unlocked(param_1);
                if (iVar3 != 0) goto LAB_00105c04;
                iVar3 = feof_unlocked(param_1);
                if (iVar3 != 0) goto LAB_00105a4c;
                putc_unlocked(-1,param_2);
              }
              putc_unlocked(iVar3,param_2);
            } while ((iVar3 != 10) || (bVar9 = lVar6 != 0, lVar6 = lVar6 + -1, bVar9));
            if (local_4a != '\0') {
              if (local_50 == 4) goto LAB_00105b9f;
LAB_001058d4:
              if (DAT_001130eb != '\0') {
LAB_00105aef:
                __fprintf_chk(param_2,1,local_60,param_7);
                lVar6 = (long)param_4[1];
                if (*(long *)(param_3 + lVar6 * 4 + 2) <= *(long *)(param_3 + lVar6 * 4 + 4)) {
                  lVar8 = 0;
                  do {
                    lVar1 = lVar8 * 8;
                    lVar2 = lVar8 * 8;
                    lVar8 = lVar8 + 1;
                    fwrite_unlocked(*(void **)(*(long *)(param_3 + lVar6 * 2 + 0xe) + lVar2),1,
                                    *(size_t *)(*(long *)(param_3 + lVar6 * 2 + 0x14) + lVar1),
                                    param_2);
                    lVar6 = (long)param_4[1];
                  } while (lVar8 <= *(long *)(param_3 + lVar6 * 4 + 4) -
                                    *(long *)(param_3 + lVar6 * 4 + 2));
                }
              }
              fputs_unlocked("=======\n",param_2);
              local_4a = '\x01';
              local_49 = 1;
            }
          }
LAB_001058fa:
          lVar6 = (long)param_4[2];
          if (*(long *)(param_3 + lVar6 * 4 + 2) <= *(long *)(param_3 + lVar6 * 4 + 4)) {
            lVar8 = 0;
            do {
              lVar1 = lVar8 * 8;
              lVar2 = lVar8 * 8;
              lVar8 = lVar8 + 1;
              fwrite_unlocked(*(void **)(*(long *)(param_3 + lVar6 * 2 + 0xe) + lVar2),1,
                              *(size_t *)(*(long *)(param_3 + lVar6 * 2 + 0x14) + lVar1),param_2);
              lVar6 = (long)param_4[2];
            } while (lVar8 <= *(long *)(param_3 + lVar6 * 4 + 4) -
                              *(long *)(param_3 + lVar6 * 4 + 2));
          }
          if (local_4a != '\0') {
            __fprintf_chk(param_2,1,">>>>>>> %s\n",param_8);
          }
          lVar8 = *(long *)(param_3 + 4) - *(long *)(param_3 + 2);
          lVar6 = local_58 + 1 + lVar8;
          if (-1 < lVar8) {
            do {
              while (iVar3 = getc_unlocked(param_1), iVar3 != 10) {
                if (iVar3 == -1) {
                  iVar3 = ferror_unlocked(param_1);
                  if (iVar3 != 0) goto LAB_00105c04;
                  iVar3 = feof_unlocked(param_1);
                  if (iVar3 != 0) {
                    if ((lVar8 == 0) && (*(long *)(param_3 + 0x1a) == 0)) {
                      return local_49;
                    }
LAB_00105a4c:
                    /* WARNING: Subroutine does not return */
                    FUN_00104210("input file shrank");
                  }
                }
              }
              bVar9 = lVar8 != 0;
              lVar8 = lVar8 + -1;
            } while (bVar9);
          }
        }
      }
    } while( true );
  }
LAB_00105c1f:
  local_49 = 0;
LAB_001059ac:
  do {
    iVar3 = getc_unlocked(param_1);
    if (iVar3 == -1) {
      iVar4 = ferror_unlocked(param_1);
      iVar5 = feof_unlocked(param_1);
      if (iVar4 != 0 || iVar5 != 0) {
        return local_49;
      }
    }
    putc_unlocked(iVar3,param_2);
  } while( true );
LAB_00105c04:
  uVar7 = dcgettext(0,"read failed",5);
  FUN_001043a0(uVar7);
  goto LAB_00105c1f;
}




// Function: check_stdout @ 0x5c30

ulong check_stdout(void)

{
  char cVar1;
  int iVar2;
  undefined8 *puVar3;
  undefined8 uVar4;
  long lVar5;
  char *pcVar6;
  char *pcVar7;
  char *pcVar8;
  ulong uVar9;
  
  iVar2 = ferror_unlocked(stdout);
  if (iVar2 != 0) {
                    /* WARNING: Subroutine does not return */
    FUN_00104210("write failed");
  }
  iVar2 = fclose(stdout);
  if (iVar2 == 0) {
    return 0;
  }
  pcVar7 = "standard output";
  puVar3 = (undefined8 *)dcgettext(0,"standard output",5);
  FUN_001043a0();
  pcVar8 = pcVar7;
  uVar4 = FUN_00104110(*puVar3);
  lVar5 = FUN_00104140(uVar4);
  if (lVar5 != 0) {
    pcVar6 = (char *)FUN_00104110(lVar5);
    if (*pcVar6 == ',') {
      pcVar6 = (char *)FUN_00104140(pcVar6 + 1,pcVar7 + 8);
      if (pcVar6 == (char *)0x0) {
        return 0;
      }
    }
    else {
      *(undefined8 *)(pcVar7 + 8) = *(undefined8 *)pcVar7;
    }
    pcVar7 = (char *)FUN_00104110(pcVar6);
    cVar1 = *pcVar7;
    if (cVar1 == 'c') {
      uVar9 = 2;
    }
    else if (cVar1 == 'd') {
      uVar9 = 3;
    }
    else {
      if (cVar1 != 'a') {
        return 0;
      }
      uVar9 = 1;
    }
    uVar4 = FUN_00104110(pcVar7 + 1,pcVar8 + 0x10);
    lVar5 = FUN_00104140(uVar4);
    if (lVar5 != 0) {
      pcVar7 = (char *)FUN_00104110(lVar5);
      if (*pcVar7 == ',') {
        lVar5 = FUN_00104140(pcVar7 + 1,pcVar8 + 0x18);
        if (lVar5 == 0) {
          return 0;
        }
        pcVar7 = (char *)FUN_00104110(lVar5);
      }
      else {
        *(undefined8 *)(pcVar8 + 0x18) = *(undefined8 *)(pcVar8 + 0x10);
      }
      *puVar3 = pcVar7;
      return uVar9 & 0xffffffff;
    }
  }
  return 0;
}




// Function: process_diff @ 0x5d70

long process_diff(undefined8 param_1,undefined8 param_2,undefined8 *param_3,undefined8 *param_4)

{
  ulong uVar1;
  long lVar2;
  long lVar3;
  undefined8 uVar4;
  int iVar5;
  char *pcVar6;
  long lVar7;
  char *pcVar8;
  undefined8 uVar9;
  long lVar10;
  long *plVar11;
  long *unaff_R15;
  long in_FS_OFFSET;
  char *local_58;
  char *local_50;
  long local_48;
  long local_40;
  
  plVar11 = &local_48;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  pcVar6 = (char *)FUN_001043d0(param_1,param_2,&local_58);
  *param_4 = local_58;
  local_50 = local_58;
  if (local_58 < pcVar6) {
LAB_00105dd0:
    do {
      unaff_R15 = (long *)FUN_00108180(0x48);
      unaff_R15[5] = 0;
      unaff_R15[4] = 0;
      unaff_R15[7] = 0;
      unaff_R15[6] = 0;
      iVar5 = FUN_00105c80(&local_50,unaff_R15);
      uVar4 = DAT_00123140;
      if ((iVar5 == 0) || (pcVar8 = local_50 + 1, *local_50 != '\n')) {
        uVar9 = dcgettext(0,"%s: diff failed: ",5);
        __fprintf_chk(stderr,1,uVar9,uVar4);
        do {
          pcVar6 = local_50;
          putc_unlocked((int)*local_50,stderr);
          local_50 = pcVar6 + 1;
        } while (*pcVar6 != '\n');
                    /* WARNING: Subroutine does not return */
        exit(2);
      }
      local_50 = pcVar8;
      if (iVar5 == 2) {
LAB_00105e3d:
        lVar2 = unaff_R15[1];
        lVar3 = *unaff_R15;
        uVar1 = (lVar2 - lVar3) + 1;
        if (0xffffffffffffffe < uVar1) goto LAB_00106088;
        lVar7 = FUN_00108180();
        unaff_R15[4] = lVar7;
        lVar7 = FUN_00108180(uVar1 * 8);
        unaff_R15[6] = lVar7;
        if (uVar1 != 0) {
          lVar10 = 0;
          while( true ) {
            pcVar8 = (char *)FUN_00104770(pcVar8,lVar10 * 8 + unaff_R15[4],lVar7 + lVar10 * 8,pcVar6
                                          ,0x3c);
            local_50 = pcVar8;
            if (lVar2 - lVar3 == lVar10) break;
            lVar7 = unaff_R15[6];
            lVar10 = lVar10 + 1;
          }
        }
        pcVar8 = local_50;
        if (iVar5 == 2) {
          iVar5 = strncmp(local_50,"---\n",4);
          if (iVar5 != 0) {
                    /* WARNING: Subroutine does not return */
            FUN_00104210("invalid diff format; invalid change separator");
          }
          local_50 = pcVar8 + 4;
        }
        else if (iVar5 == 3) {
          *plVar11 = (long)unaff_R15;
          plVar11 = unaff_R15 + 8;
          if (pcVar6 <= local_50) break;
          goto LAB_00105dd0;
        }
      }
      else {
        if (iVar5 == 3) {
          unaff_R15[2] = unaff_R15[2] + 1;
          goto LAB_00105e3d;
        }
        if (iVar5 != 1) {
                    /* WARNING: Subroutine does not return */
          FUN_00104210("internal error: invalid diff type in process_diff");
        }
        *unaff_R15 = *unaff_R15 + 1;
      }
      lVar2 = unaff_R15[3];
      lVar3 = unaff_R15[2];
      uVar1 = (lVar2 - lVar3) + 1;
      if (0xffffffffffffffe < uVar1) {
LAB_00106088:
                    /* WARNING: Subroutine does not return */
        FUN_00108530();
      }
      lVar7 = FUN_00108180();
      unaff_R15[5] = lVar7;
      lVar7 = FUN_00108180(uVar1 * 8);
      unaff_R15[7] = lVar7;
      if (uVar1 != 0) {
        lVar10 = 0;
        while( true ) {
          local_50 = (char *)FUN_00104770(local_50,lVar10 * 8 + unaff_R15[5],lVar7 + lVar10 * 8,
                                          pcVar6,0x3e);
          if (lVar2 - lVar3 == lVar10) break;
          lVar7 = unaff_R15[7];
          lVar10 = lVar10 + 1;
        }
      }
      *plVar11 = (long)unaff_R15;
      plVar11 = unaff_R15 + 8;
    } while (local_50 < pcVar6);
  }
  *plVar11 = 0;
  *param_3 = unaff_R15;
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return local_48;
}



