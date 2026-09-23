// Function: main @ 0x26e0

byte main(int param_1,undefined8 *param_2)

{
  char *__s1;
  byte bVar1;
  byte bVar2;
  int iVar3;
  FILE *pFVar4;
  undefined8 uVar5;
  int *piVar6;
  undefined8 uVar7;
  ulong extraout_RDX;
  ulong uVar8;
  ulong extraout_RDX_00;
  char *pcVar9;
  char *__modes;
  char *local_48;
  
  __modes = (char *)0x0;
  FUN_00104350(*param_2);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  FUN_00107c40(FUN_001040f0);
  DAT_00118130 = 0;
  DAT_00118131 = 0;
  DAT_00118132 = 0;
  DAT_00118133 = 0;
  DAT_00118120 = 0x4b;
  DAT_00118128 = "";
  DAT_0011811c = 0;
  DAT_00118118 = 0;
  DAT_00118114 = 0;
  if (((1 < param_1) && (pcVar9 = (char *)param_2[1], *pcVar9 == '-')) &&
     ((int)pcVar9[1] - 0x30U < 10)) {
    __modes = pcVar9 + 1;
    uVar5 = *param_2;
    param_1 = param_1 + -1;
    param_2 = param_2 + 1;
    *param_2 = uVar5;
  }
  local_48 = (char *)0x0;
  pcVar9 = "0123456789cstuw:p:g:";
  while (iVar3 = getopt_long(param_1,param_2,"0123456789cstuw:p:g:",&PTR_s_crown_margin_0010ca80,0),
        iVar3 != -1) {
    if (0x77 < iVar3) {
switchD_00102803_caseD_64:
      if (iVar3 - 0x30U < 10) {
        uVar5 = dcgettext(0,
                          "invalid option -- %c; -WIDTH is recognized only when it is the first\noption; use -w N instead"
                          ,5);
        error(0,0,uVar5,iVar3);
      }
      FUN_00103dc0(1);
LAB_00102aaf:
      FUN_00106dd0(stdout,&DAT_00108556,"GNU coreutils",PTR_DAT_0010d010,"Ross Paterson",0);
                    /* WARNING: Subroutine does not return */
      exit(0);
    }
    if (iVar3 < 99) {
      if (iVar3 == -0x83) goto LAB_00102aaf;
      if (iVar3 != -0x82) goto switchD_00102803_caseD_64;
      FUN_00103dc0(0);
      break;
    }
    switch(iVar3) {
    case 99:
      DAT_00118133 = 1;
      break;
    default:
      goto switchD_00102803_caseD_64;
    case 0x67:
      local_48 = optarg;
      break;
    case 0x70:
      FUN_00102fd0(optarg);
      break;
    case 0x73:
      DAT_00118131 = 1;
      break;
    case 0x74:
      DAT_00118132 = 1;
      break;
    case 0x75:
      DAT_00118130 = 1;
      break;
    case 0x77:
      __modes = optarg;
    }
  }
  if (__modes == (char *)0x0) {
    if (local_48 != (char *)0x0) {
      uVar5 = dcgettext(0,"invalid width",5);
      DAT_00118110 = FUN_00107450(local_48,0,0x4b,"",uVar5,0);
      DAT_00118120 = DAT_00118110 + 10;
      uVar8 = extraout_RDX_00;
      goto LAB_0010292f;
    }
  }
  else {
    pcVar9 = "invalid width";
    uVar5 = dcgettext(0,"invalid width",5);
    iVar3 = FUN_00107450(__modes,0,0x9c4,"",uVar5,0);
    DAT_00118120 = iVar3;
    if (local_48 != (char *)0x0) {
      uVar5 = dcgettext(0,"invalid width",5);
      DAT_00118110 = FUN_00107450(local_48,0,(long)iVar3,"",uVar5,0);
      uVar8 = extraout_RDX;
      goto LAB_0010292f;
    }
  }
  DAT_00118110 = (DAT_00118120 * 0xbb) / 200;
  uVar8 = (long)(DAT_00118120 * 0xbb) % 200 & 0xffffffff;
LAB_0010292f:
  if (optind == param_1) {
    bVar2 = FUN_00103a30(stdin,&DAT_00108792,uVar8);
    goto LAB_00102a16;
  }
  local_48 = (char *)0x0;
  bVar2 = 1;
  pcVar9 = "-";
  __modes = "r";
  if (param_1 <= optind) {
LAB_00102a2a:
    return bVar2 ^ 1;
  }
  do {
    __s1 = (char *)param_2[optind];
    iVar3 = strcmp(__s1,pcVar9);
    if (iVar3 == 0) {
      bVar1 = FUN_00103a30(stdin,__s1);
      local_48 = (char *)0x1;
      bVar2 = bVar2 & bVar1;
    }
    else {
      pFVar4 = fopen(__s1,__modes);
      if (pFVar4 == (FILE *)0x0) goto LAB_00102b3f;
      bVar1 = FUN_00103a30(pFVar4,__s1);
      bVar2 = bVar2 & bVar1;
    }
    while (optind = optind + 1, param_1 <= optind) {
      if ((char)local_48 == '\0') goto LAB_00102a2a;
LAB_00102a16:
      iVar3 = FUN_001041e0(stdin);
      if (iVar3 == 0) goto LAB_00102a2a;
      param_2 = (undefined8 *)dcgettext(0,"closing standard input",5);
      piVar6 = __errno_location();
      error(1,*piVar6,"%s",param_2);
LAB_00102b3f:
      uVar5 = FUN_001065c0(4);
      uVar7 = dcgettext(0,"cannot open %s for reading",5);
      piVar6 = __errno_location();
      bVar2 = 0;
      error(0,*piVar6,uVar7,uVar5);
    }
  } while( true );
}




// Function: set_other_indent @ 0x2c80

void set_other_indent(char param_1)

{
  if (DAT_00118131 != '\0') {
    DAT_0010d0ec = DAT_0010d0f0;
    return;
  }
  if (DAT_00118133 != '\0') {
    DAT_0010d0ec = DAT_0011810c;
    if (param_1 == '\0') {
      DAT_0010d0ec = DAT_0010d0f0;
    }
    return;
  }
  if (DAT_00118132 == '\0') {
    DAT_0010d0ec = DAT_0010d0f0;
  }
  else {
    if ((param_1 != '\0') && (DAT_0011810c != DAT_0010d0f0)) {
      DAT_0010d0ec = DAT_0011810c;
      return;
    }
    if (DAT_0010d0ec == DAT_0010d0f0) {
      DAT_0010d0ec = -(uint)(DAT_0010d0f0 == 0) & 3;
      return;
    }
  }
  return;
}




// Function: same_para @ 0x2d10

bool same_para(int param_1)

{
  if ((DAT_0010d0e4 == DAT_0010d0f4) && (DAT_0010d0e4 + DAT_0011811c <= DAT_0011810c)) {
    return param_1 != 10 && param_1 != -1;
  }
  return false;
}




// Function: base_cost @ 0x2d50

long base_cost(undefined *param_1)

{
  byte bVar1;
  undefined1 auVar2 [16];
  undefined1 auVar3 [16];
  long lVar4;
  
  lVar4 = 0x1324;
  if (&DAT_0010d120 < param_1) {
    bVar1 = param_1[-0x18];
    if ((bVar1 & 2) == 0) {
      lVar4 = 0xce4;
      if ((((bVar1 & 4) == 0) && (lVar4 = 0x1324, &DAT_0010d148 < param_1)) &&
         ((param_1[-0x40] & 8) != 0)) {
        auVar3._8_8_ = 0;
        auVar3._0_8_ = SUB168(SEXT816(40000),8);
        lVar4 = SUB168((auVar3 << 0x40 | ZEXT816(40000)) /
                       SEXT816((long)(*(int *)(param_1 + -0x20) + 2)),0) + 0x1324;
      }
    }
    else {
      lVar4 = (ulong)(-(uint)((bVar1 & 8) == 0) & 0x58804) + 0x960;
    }
  }
  if ((param_1[0x10] & 1) == 0) {
    if ((param_1[0x10] & 8) != 0) {
      auVar2._8_8_ = 0;
      auVar2._0_8_ = SUB168(SEXT816(0x57e4),8);
      return lVar4 + SUB168((auVar2 << 0x40 | ZEXT816(0x57e4)) /
                            SEXT816((long)(*(int *)(param_1 + 8) + 2)),0);
    }
  }
  else {
    lVar4 = lVar4 + -0x640;
  }
  return lVar4;
}




// Function: line_cost @ 0x2e00

long line_cost(long param_1,int param_2)

{
  long lVar1;
  long lVar2;
  
  if (DAT_0010d100 == param_1) {
    lVar1 = 0;
  }
  else {
    lVar1 = (long)((DAT_00118110 - param_2) * 10);
    lVar1 = lVar1 * lVar1;
    if (DAT_0010d100 != *(long *)(param_1 + 0x20)) {
      lVar2 = (long)((param_2 - *(int *)(param_1 + 0x14)) * 10);
      return lVar1 + (lVar2 * lVar2 >> 1);
    }
  }
  return lVar1;
}




// Function: fmt_paragraph @ 0x2e50

void fmt_paragraph(void)

{
  undefined4 uVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  int iVar5;
  long lVar6;
  undefined *puVar7;
  long lVar8;
  int iVar9;
  long lVar10;
  undefined *puVar11;
  undefined *puVar12;
  undefined *puVar13;
  
  iVar5 = DAT_00118120;
  puVar12 = DAT_0010d100;
  puVar11 = &DAT_0010d120;
  uVar1 = *(undefined4 *)(DAT_0010d100 + 8);
  puVar13 = DAT_0010d100 + -0x28;
  *(undefined8 *)(DAT_0010d100 + 0x18) = 0;
  *(int *)(puVar12 + 8) = iVar5;
  iVar4 = DAT_0010d0f0;
  iVar3 = DAT_0010d0ec;
  iVar2 = DAT_0010d0e0;
  if ((undefined *)0x10d11f < puVar13) {
    do {
      lVar10 = 0x7fffffffffffffff;
      iVar9 = iVar3;
      if (puVar13 == puVar11) {
        iVar9 = iVar4;
      }
      iVar9 = iVar9 + *(int *)(puVar13 + 8);
      puVar7 = puVar13;
      do {
        puVar7 = puVar7 + 0x28;
        lVar6 = FUN_00102e00(puVar7,iVar9);
        lVar6 = lVar6 + *(long *)(puVar7 + 0x18);
        if ((puVar13 == puVar11) && (0 < iVar2)) {
          lVar8 = (long)((iVar9 - iVar2) * 10);
          lVar6 = lVar6 + (lVar8 * lVar8 >> 1);
        }
        if (lVar6 < lVar10) {
          *(undefined **)(puVar13 + 0x20) = puVar7;
          *(int *)(puVar13 + 0x14) = iVar9;
          lVar10 = lVar6;
        }
      } while ((puVar12 != puVar7) &&
              (iVar9 = iVar9 + *(int *)(puVar7 + 8) + *(int *)(puVar7 + -0x1c), iVar9 < iVar5));
      puVar7 = puVar13 + -0x28;
      lVar6 = FUN_00102d50(puVar13);
      *(long *)(puVar13 + 0x18) = lVar6 + lVar10;
      puVar13 = puVar7;
    } while (puVar11 <= puVar7);
  }
  *(undefined4 *)(puVar12 + 8) = uVar1;
  return;
}




// Function: set_prefix @ 0x2fd0

void set_prefix(char *param_1)

{
  size_t sVar1;
  char *pcVar2;
  int iVar3;
  
  DAT_00118118 = 0;
  if (*param_1 == ' ') {
    iVar3 = (int)param_1;
    do {
      DAT_00118118 = (1 - iVar3) + (int)param_1;
      param_1 = param_1 + 1;
    } while (*param_1 == ' ');
  }
  DAT_00118128 = param_1;
  sVar1 = strlen(param_1);
  DAT_0011811c = (int)sVar1;
  pcVar2 = param_1 + DAT_0011811c;
  if (param_1 < pcVar2) {
    do {
      if (pcVar2[-1] != ' ') break;
      pcVar2 = pcVar2 + -1;
    } while (pcVar2 != param_1);
  }
  *pcVar2 = '\0';
  DAT_00118114 = (int)pcVar2 - (int)param_1;
  return;
}




// Function: get_space @ 0x3050

void get_space(FILE *param_1,int param_2)

{
  int iVar1;
  
  do {
    if (param_2 == 0x20) {
      DAT_0011810c = DAT_0011810c + 1;
    }
    else {
      if (param_2 != 9) {
        return;
      }
      DAT_0010d0f8 = 1;
      iVar1 = DAT_0011810c + 7;
      if (-1 < DAT_0011810c) {
        iVar1 = DAT_0011810c;
      }
      DAT_0011810c = (iVar1 >> 3) * 8 + 8;
    }
    param_2 = getc_unlocked(param_1);
  } while( true );
}




// Function: get_prefix @ 0x30b0

void get_prefix(FILE *param_1)

{
  byte bVar1;
  int iVar2;
  uint uVar3;
  byte *pbVar4;
  
  DAT_0011810c = 0;
  iVar2 = getc_unlocked(param_1);
  uVar3 = FUN_00103050(param_1,iVar2);
  DAT_0010d0e4 = DAT_0011810c;
  if (DAT_00118114 == 0) {
    if (DAT_00118118 < DAT_0011810c) {
      DAT_0010d0e4 = DAT_00118118;
    }
  }
  else {
    bVar1 = *DAT_00118128;
    pbVar4 = DAT_00118128;
    while( true ) {
      if (bVar1 == 0) {
        FUN_00103050(param_1,uVar3);
        return;
      }
      if (bVar1 != uVar3) break;
      pbVar4 = pbVar4 + 1;
      DAT_0011810c = DAT_0011810c + 1;
      uVar3 = getc_unlocked(param_1);
      bVar1 = *pbVar4;
    }
  }
  return;
}




// Function: put_space @ 0x3150

void put_space(int param_1)

{
  uint uVar1;
  uint uVar2;
  int iVar3;
  
  uVar1 = DAT_00118108 + param_1;
  if (DAT_0010d0f8 != '\0') {
    uVar2 = uVar1 + 7;
    if (-1 < (int)uVar1) {
      uVar2 = uVar1;
    }
    if ((DAT_00118108 + 1 < (int)(uVar2 & 0xfffffff8)) && (DAT_00118108 < (int)(uVar2 & 0xfffffff8))
       ) {
      do {
        putchar_unlocked(9);
        iVar3 = DAT_00118108 + 7;
        if (-1 < DAT_00118108) {
          iVar3 = DAT_00118108;
        }
        iVar3 = (iVar3 >> 3) + 1;
        DAT_00118108 = iVar3 * 8;
      } while (iVar3 < (int)uVar2 >> 3);
    }
  }
  for (; DAT_00118108 < (int)uVar1; DAT_00118108 = DAT_00118108 + 1) {
    putchar_unlocked(0x20);
  }
  return;
}




// Function: put_word @ 0x31f0

void put_word(undefined8 *param_1)

{
  char cVar1;
  char *pcVar2;
  int iVar3;
  char *pcVar4;
  
  pcVar2 = (char *)*param_1;
  iVar3 = 0;
  if (*(uint *)(param_1 + 1) != 0) {
    pcVar4 = pcVar2 + *(uint *)(param_1 + 1);
    do {
      cVar1 = *pcVar2;
      pcVar2 = pcVar2 + 1;
      putchar_unlocked((int)cVar1);
    } while (pcVar4 != pcVar2);
    iVar3 = *(int *)(param_1 + 1);
  }
  DAT_00118108 = DAT_00118108 + iVar3;
  return;
}




// Function: put_line @ 0x3230

void put_line(long param_1,int param_2)

{
  long lVar1;
  
  DAT_00118108 = 0;
  FUN_00103150(DAT_0010d0f4);
  fputs_unlocked(DAT_00118128,stdout);
  DAT_00118108 = DAT_00118114 + DAT_00118108;
  FUN_00103150(param_2 - DAT_00118108);
  lVar1 = *(long *)(param_1 + 0x20);
  for (; param_1 != lVar1 + -0x28; param_1 = param_1 + 0x28) {
    FUN_001031f0(param_1);
    FUN_00103150(*(undefined4 *)(param_1 + 0xc));
  }
  FUN_001031f0(param_1);
  DAT_0010d0e0 = DAT_00118108;
  putchar_unlocked(10);
  return;
}




// Function: put_paragraph @ 0x32d0

void put_paragraph(long param_1)

{
  long lVar1;
  
  FUN_00103230(&DAT_0010d120,DAT_0010d0f0);
  for (lVar1 = DAT_0010d140; lVar1 != param_1; lVar1 = *(long *)(lVar1 + 0x20)) {
    FUN_00103230(lVar1,DAT_0010d0ec);
  }
  return;
}




// Function: copy_rest @ 0x3320

int copy_rest(FILE *param_1,int param_2)

{
  int iVar1;
  char *pcVar2;
  
  DAT_00118108 = 0;
  if (DAT_0010d0e4 < DAT_0011810c) {
    FUN_00103150();
    pcVar2 = DAT_00118128;
    if (DAT_00118108 != DAT_0011810c) goto LAB_001033a2;
  }
  else {
    if (param_2 == 10 || param_2 == -1) {
      DAT_00118108 = 0;
      return param_2;
    }
    FUN_00103150();
    iVar1 = 0;
    pcVar2 = DAT_00118128;
    if (DAT_00118108 == DAT_0011810c) goto LAB_001033e4;
LAB_001033a2:
    do {
      if (*pcVar2 == '\0') break;
      putchar_unlocked((int)*pcVar2);
      DAT_00118108 = DAT_00118108 + 1;
      pcVar2 = pcVar2 + 1;
    } while (DAT_00118108 != DAT_0011810c);
  }
  if (param_2 == 10 || param_2 == -1) {
    if (param_2 != -1) {
      return param_2;
    }
    if (DAT_0011810c < DAT_00118114 + DAT_0010d0e4) {
      return -1;
    }
    putchar_unlocked(10);
    return -1;
  }
  iVar1 = DAT_0011810c - DAT_00118108;
LAB_001033e4:
  FUN_00103150(iVar1);
  do {
    putchar_unlocked(param_2);
    param_2 = getc_unlocked(param_1);
    if (param_2 == 10) {
      return 10;
    }
  } while (param_2 != -1);
  return -1;
}




// Function: check_punctuation @ 0x3460

void check_punctuation(undefined8 *param_1)

{
  byte *pbVar1;
  byte *pbVar2;
  char *pcVar3;
  ushort **ppuVar4;
  byte *pbVar5;
  int __c;
  
  pbVar2 = (byte *)*param_1;
  pbVar1 = pbVar2 + (long)*(int *)(param_1 + 1) + -1;
  pcVar3 = strchr("([\'`\"",(int)(char)*pbVar2);
  ppuVar4 = __ctype_b_loc();
  *(byte *)(param_1 + 2) =
       (byte)(*ppuVar4)[*pbVar1] & 4 | pcVar3 != (char *)0x0 | *(byte *)(param_1 + 2) & 0xfa;
  if (pbVar2 < pbVar1) {
    do {
      pbVar5 = pbVar1;
      __c = (int)(char)*pbVar5;
      pcVar3 = strchr(")]\'\"",__c);
      if (pcVar3 == (char *)0x0) goto LAB_001034e6;
      pbVar1 = pbVar5 + -1;
    } while (pbVar2 != pbVar5 + -1);
    __c = (int)(char)pbVar5[-1];
  }
  else {
    __c = (int)(char)*pbVar1;
  }
LAB_001034e6:
  pcVar3 = strchr(".?!",__c);
  *(byte *)(param_1 + 2) = *(byte *)(param_1 + 2) & 0xfd | (pcVar3 != (char *)0x0) * '\x02';
  return;
}




// Function: flush_paragraph @ 0x3530

void flush_paragraph(void)

{
  long *plVar1;
  int iVar2;
  long lVar3;
  long lVar4;
  long *plVar5;
  long *plVar6;
  long lVar7;
  long *plVar8;
  
  plVar1 = DAT_0010d100;
  if (DAT_0010d100 == (long *)&DAT_0010d120) {
    fwrite_unlocked(&DAT_00116d80,1,(long)DAT_00116d60 - 0x116d80,stdout);
    DAT_00116d60 = &DAT_00116d80;
    return;
  }
  FUN_00102e50();
  plVar8 = plVar1;
  if (plVar1 != DAT_0010d140) {
    lVar3 = 0x7fffffffffffffff;
    plVar5 = DAT_0010d140;
    lVar7 = DAT_0010d140[3];
    do {
      plVar6 = (long *)plVar5[4];
      lVar7 = lVar7 - plVar6[3];
      if (lVar7 < lVar3) {
        lVar3 = lVar7;
        plVar8 = plVar5;
      }
      if (lVar3 < 0x7ffffffffffffff7) {
        lVar3 = lVar3 + 9;
      }
      plVar5 = plVar6;
      lVar7 = plVar6[3];
    } while (plVar1 != plVar6);
  }
  FUN_001032d0(plVar8);
  lVar7 = (long)DAT_00116d60;
  iVar2 = __memmove_chk(&DAT_00116d80,*plVar8,(long)DAT_00116d60 - *plVar8,5000);
  plVar1 = DAT_0010d100;
  lVar3 = *plVar8;
  lVar4 = (long)((int)lVar3 - iVar2);
  DAT_00116d60 = (undefined *)(lVar7 - lVar4);
  plVar5 = plVar8;
  if (plVar8 <= DAT_0010d100) {
    while( true ) {
      plVar6 = plVar5 + 5;
      *plVar5 = lVar3 - lVar4;
      if (plVar1 < plVar6) break;
      lVar3 = *plVar6;
      plVar5 = plVar6;
    }
  }
  __memmove_chk(&DAT_0010d120,plVar8,(undefined *)((long)plVar1 + (0x28 - (long)plVar8)),40000);
  DAT_0010d100 = (long *)((long)plVar1 - (long)(plVar8 + -0x21a24));
  return;
}




// Function: get_line @ 0x3690

void get_line(FILE *param_1,int param_2)

{
  char cVar1;
  undefined4 *puVar2;
  undefined8 *puVar3;
  bool bVar4;
  int iVar5;
  
LAB_001036b8:
  do {
    puVar2 = DAT_00116d60;
    *DAT_0010d100 = DAT_00116d60;
    do {
      if (puVar2 == &DAT_00118108) {
        FUN_00102c80(1);
        FUN_00103530();
        puVar2 = DAT_00116d60;
      }
      *(char *)puVar2 = (char)param_2;
      DAT_00116d60 = (undefined4 *)((long)puVar2 + 1);
      param_2 = getc_unlocked(param_1);
    } while ((param_2 != -1) &&
            (cVar1 = FUN_00104030(param_2), puVar2 = DAT_00116d60, cVar1 == '\0'));
    iVar5 = (int)DAT_00116d60 - (int)*DAT_0010d100;
    *(int *)(DAT_0010d100 + 1) = iVar5;
    iVar5 = DAT_0011810c + iVar5;
    DAT_0011810c = iVar5;
    FUN_00103460();
    param_2 = FUN_00103050(param_1,param_2);
    puVar3 = DAT_0010d100;
    iVar5 = DAT_0011810c - iVar5;
    *(int *)((long)DAT_0010d100 + 0xc) = iVar5;
    if (param_2 == -1) {
      *(byte *)(puVar3 + 2) = *(byte *)(puVar3 + 2) | 8;
LAB_001037d4:
      *(uint *)((long)puVar3 + 0xc) = ((*(byte *)(puVar3 + 2) & 8) != 0) + 1;
      if (puVar3 == (undefined8 *)&DAT_00116d10) goto LAB_00103820;
    }
    else {
      if ((*(byte *)(puVar3 + 2) & 2) == 0) {
        bVar4 = false;
LAB_00103791:
        *(byte *)(puVar3 + 2) = *(byte *)(puVar3 + 2) & 0xf7 | bVar4 * '\b';
        if (param_2 == 10) goto LAB_001037d4;
      }
      else {
        bVar4 = 1 < iVar5 || param_2 == 10;
        if (1 < iVar5 || param_2 == 10) goto LAB_00103791;
        *(byte *)(puVar3 + 2) = *(byte *)(puVar3 + 2) & 0xf7;
      }
      if (DAT_00118130 != '\0') goto LAB_001037d4;
      if (puVar3 != (undefined8 *)&DAT_00116d10) {
        DAT_0010d100 = puVar3 + 5;
        goto LAB_001036b8;
      }
LAB_00103820:
      FUN_00102c80(1);
      FUN_00103530();
      puVar3 = DAT_0010d100;
    }
    DAT_0010d100 = puVar3 + 5;
    if ((param_2 == 10) || (param_2 == -1)) {
      FUN_001030b0(param_1);
      return;
    }
  } while( true );
}




// Function: get_paragraph @ 0x3850

undefined8 get_paragraph(undefined8 param_1)

{
  undefined1 uVar1;
  char cVar2;
  int iVar3;
  uint uVar4;
  undefined4 uVar5;
  ulong uVar6;
  
  DAT_0010d0e0 = 0;
  iVar3 = DAT_0010d0e8;
  while ((((iVar3 == 10 || (iVar3 == -1)) || (DAT_0010d0e4 < DAT_00118118)) ||
         (DAT_0011810c < DAT_0011811c + DAT_0010d0e4))) {
    iVar3 = FUN_00103320(param_1);
    if (iVar3 == -1) {
      DAT_0010d0e8 = iVar3;
      return 0;
    }
    putchar_unlocked(10);
    iVar3 = FUN_001030b0(param_1);
  }
  DAT_0010d0f4 = DAT_0010d0e4;
  DAT_0010d0f0 = DAT_0011810c;
  DAT_00116d60 = &DAT_00116d80;
  DAT_0010d100 = &DAT_0010d120;
  uVar4 = FUN_00103690(param_1);
  uVar6 = (ulong)uVar4;
  uVar1 = FUN_00102d10(uVar4);
  FUN_00102c80(uVar1);
  uVar5 = (undefined4)uVar6;
  if (DAT_00118131 == '\0') {
    cVar2 = FUN_00102d10(uVar6 & 0xffffffff);
    uVar5 = (undefined4)uVar6;
    if (DAT_00118133 == '\0') {
      if (DAT_00118132 == '\0') {
        while( true ) {
          uVar5 = (undefined4)uVar6;
          cVar2 = FUN_00102d10(uVar6 & 0xffffffff);
          if ((cVar2 == '\0') || (DAT_0011810c != DAT_0010d0ec)) break;
          uVar4 = FUN_00103690(param_1);
          uVar6 = (ulong)uVar4;
        }
      }
      else if ((cVar2 != '\0') && (DAT_0011810c != DAT_0010d0f0)) {
        do {
          uVar5 = FUN_00103690(param_1);
          cVar2 = FUN_00102d10(uVar5);
          if (cVar2 == '\0') break;
        } while (DAT_0011810c == DAT_0010d0ec);
      }
    }
    else if (cVar2 != '\0') {
      do {
        uVar5 = FUN_00103690(param_1);
        cVar2 = FUN_00102d10(uVar5);
        if (cVar2 == '\0') break;
      } while (DAT_0011810c == DAT_0010d0ec);
    }
  }
  if (DAT_0010d100 <= &DAT_0010d120) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("word < word_limit","src/fmt.c",0x270,"get_paragraph");
  }
  DAT_0010d100[-0x18] = DAT_0010d100[-0x18] | 10;
  DAT_0010d0e8 = uVar5;
  return 1;
}




// Function: fmt @ 0x3a30

undefined4 fmt(FILE *param_1,undefined8 param_2)

{
  undefined4 uVar1;
  int iVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  int *piVar5;
  
  FUN_001041b0(param_1,2);
  DAT_0010d0f8 = 0;
  DAT_0010d0ec = 0;
  DAT_0010d0e8 = FUN_001030b0(param_1);
  while (uVar1 = FUN_00103850(param_1), (char)uVar1 != '\0') {
    FUN_00102e50();
    FUN_001032d0(DAT_0010d100);
  }
  iVar2 = ferror_unlocked(param_1);
  if (iVar2 == 0) {
    if (param_1 == stdin) {
      clearerr_unlocked(param_1);
      return 1;
    }
    iVar2 = FUN_001041e0();
    if (iVar2 == 0) {
      return 1;
    }
    piVar5 = __errno_location();
    iVar2 = *piVar5;
    if (iVar2 < 0) {
      return 1;
    }
    uVar3 = FUN_001066c0(0,3,param_2);
    if (iVar2 != 0) {
      error(0,iVar2,"%s",uVar3);
      return uVar1;
    }
  }
  else {
    if (param_1 == stdin) {
      clearerr_unlocked(param_1);
    }
    else {
      FUN_001041e0();
    }
    uVar3 = FUN_001066c0(0,3,param_2);
  }
  uVar4 = dcgettext(0,"read error",5);
  error(0,0,uVar4,uVar3);
  return uVar1;
}




// Function: usage @ 0x3dc0

void usage(int param_1)

{
  FILE *pFVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  char *pcVar4;
  
  uVar2 = DAT_00118148;
  if (param_1 == 0) {
    uVar3 = dcgettext(0,"Usage: %s [-WIDTH] [OPTION]... [FILE]...\n",5);
    __printf_chk(1,uVar3,uVar2);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "Reformat each paragraph in the FILE(s), writing to standard output.\nThe option -WIDTH is an abbreviated form of --width=DIGITS.\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    FUN_00102f70();
    FUN_00102fa0();
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "  -c, --crown-margin        preserve indentation of first two lines\n  -p, --prefix=STRING       reformat only lines beginning with STRING,\n                              reattaching the prefix to reformatted lines\n  -s, --split-only          split long lines, but do not refill\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "  -t, --tagged-paragraph    indentation of first line different from second\n  -u, --uniform-spacing     one space between words, two after sentences\n  -w, --width=WIDTH         maximum line width (default of 75 columns)\n  -g, --goal=WIDTH          goal width (default of 93% of width)\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,"      --help        display this help and exit\n",5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,"      --version     output version information and exit\n",5);
    fputs_unlocked(pcVar4,pFVar1);
    FUN_00103b90();
  }
  else {
    uVar3 = dcgettext(0,"Try \'%s --help\' for more information.\n",5);
    __fprintf_chk(stderr,1,uVar3,uVar2);
  }
                    /* WARNING: Subroutine does not return */
  exit(param_1);
}



