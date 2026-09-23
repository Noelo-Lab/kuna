// Function: main @ 0x26a0

byte main(int param_1,undefined8 *param_2)

{
  byte bVar1;
  byte bVar2;
  int iVar3;
  size_t sVar4;
  char *pcVar5;
  FILE *pFVar6;
  undefined8 uVar7;
  undefined8 *puVar8;
  int *piVar9;
  undefined8 uVar10;
  ulong extraout_RDX;
  ulong uVar11;
  ulong extraout_RDX_00;
  char *pcVar12;
  char *unaff_R15;
  char *local_48;
  char *local_40;
  
  FUN_00104350(*param_2);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  FUN_00107b90(FUN_00104100);
  DAT_00117130 = 0;
  DAT_00117131 = 0;
  DAT_00117132 = 0;
  DAT_00117133 = 0;
  DAT_00117120 = 0x4b;
  DAT_00117128 = "";
  DAT_0011711c = 0;
  DAT_00117118 = 0;
  DAT_00117114 = 0;
  local_40 = (char *)0x0;
  puVar8 = param_2;
  if (((1 < param_1) && (pcVar12 = (char *)param_2[1], *pcVar12 == '-')) &&
     ((int)pcVar12[1] - 0x30U < 10)) {
    local_40 = pcVar12 + 1;
    puVar8 = param_2 + 1;
    param_1 = param_1 + -1;
    *puVar8 = *param_2;
  }
  local_48 = (char *)0x0;
  pcVar12 = "0123456789cstuw:p:g:";
  while (iVar3 = getopt_long(param_1,puVar8,"0123456789cstuw:p:g:",&PTR_s_crown_margin_0010baa0,0),
        iVar3 != -1) {
    if (0x77 < iVar3) {
switchD_001027cb_caseD_64:
      if (iVar3 - 0x30U < 10) {
        uVar7 = dcgettext(0,
                          "invalid option -- %c; -WIDTH is recognized only when it is the first\noption; use -w N instead"
                          ,5);
        error(0,0,uVar7,iVar3);
      }
      FUN_00103d70(1);
LAB_00102adf:
      FUN_00106c80(stdout,&DAT_0010803c,"GNU coreutils",PTR_DAT_0010c010,"Ross Paterson",0);
                    /* WARNING: Subroutine does not return */
      exit(0);
    }
    if (iVar3 < 99) {
      if (iVar3 == -0x83) goto LAB_00102adf;
      if (iVar3 != -0x82) goto switchD_001027cb_caseD_64;
      FUN_00103d70(0);
      break;
    }
    switch(iVar3) {
    case 99:
      DAT_00117133 = 1;
      break;
    default:
      goto switchD_001027cb_caseD_64;
    case 0x67:
      local_48 = optarg;
      break;
    case 0x70:
      DAT_00117118 = 0;
      unaff_R15 = optarg;
      if (*optarg == ' ') {
        do {
          DAT_00117118 = (1 - (int)optarg) + (int)unaff_R15;
          unaff_R15 = unaff_R15 + 1;
        } while (*unaff_R15 == ' ');
      }
      DAT_00117128 = unaff_R15;
      sVar4 = strlen(unaff_R15);
      DAT_0011711c = (int)sVar4;
      pcVar5 = unaff_R15 + DAT_0011711c;
      if (unaff_R15 < pcVar5) {
        do {
          if (pcVar5[-1] != ' ') break;
          pcVar5 = pcVar5 + -1;
        } while (unaff_R15 != pcVar5);
      }
      *pcVar5 = '\0';
      DAT_00117114 = (int)pcVar5 - (int)unaff_R15;
      break;
    case 0x73:
      DAT_00117131 = 1;
      break;
    case 0x74:
      DAT_00117132 = 1;
      break;
    case 0x75:
      DAT_00117130 = 1;
      break;
    case 0x77:
      local_40 = optarg;
    }
  }
  if (local_40 == (char *)0x0) {
    if (local_48 != (char *)0x0) {
      uVar7 = dcgettext(0,"invalid width",5);
      DAT_00117110 = FUN_001074a0(local_48,0,0x4b,"",uVar7,0);
      DAT_00117120 = DAT_00117110 + 10;
      uVar11 = extraout_RDX_00;
      goto LAB_00102954;
    }
  }
  else {
    pcVar12 = "invalid width";
    uVar7 = dcgettext(0,"invalid width",5);
    iVar3 = FUN_001074a0(local_40,0,0x9c4,"",uVar7,0);
    DAT_00117120 = iVar3;
    if (local_48 != (char *)0x0) {
      uVar7 = dcgettext(0,"invalid width",5);
      DAT_00117110 = FUN_001074a0(local_48,0,(long)iVar3,"",uVar7,0);
      uVar11 = extraout_RDX;
      goto LAB_00102954;
    }
  }
  DAT_00117110 = (DAT_00117120 * 0xbb) / 200;
  uVar11 = (long)(DAT_00117120 * 0xbb) % 200 & 0xffffffff;
LAB_00102954:
  if (optind == param_1) {
    bVar2 = FUN_00103700(stdin,&DAT_00108792,uVar11);
    goto LAB_00102a46;
  }
  local_48 = (char *)0x0;
  bVar2 = 1;
  pcVar12 = "-";
  unaff_R15 = "r";
  if (param_1 <= optind) {
LAB_00102a5a:
    return bVar2 ^ 1;
  }
  do {
    pcVar5 = (char *)puVar8[optind];
    iVar3 = strcmp(pcVar5,pcVar12);
    if (iVar3 == 0) {
      bVar1 = FUN_00103700(stdin,pcVar5);
      local_48 = (char *)0x1;
      bVar2 = bVar2 & bVar1;
    }
    else {
      pFVar6 = fopen(pcVar5,unaff_R15);
      if (pFVar6 == (FILE *)0x0) goto LAB_00102b6f;
      bVar1 = FUN_00103700(pFVar6,pcVar5);
      bVar2 = bVar2 & bVar1;
    }
    while (optind = optind + 1, param_1 <= optind) {
      if ((char)local_48 == '\0') goto LAB_00102a5a;
LAB_00102a46:
      iVar3 = FUN_001041f0(stdin);
      if (iVar3 == 0) goto LAB_00102a5a;
      puVar8 = (undefined8 *)dcgettext(0,"closing standard input",5);
      piVar9 = __errno_location();
      error(1,*piVar9,"%s",puVar8);
LAB_00102b6f:
      uVar7 = FUN_00106030(4);
      uVar10 = dcgettext(0,"cannot open %s for reading",5);
      piVar9 = __errno_location();
      bVar2 = 0;
      error(0,*piVar9,uVar10,uVar7);
    }
  } while( true );
}




// Function: fmt_paragraph @ 0x2cb0

void fmt_paragraph(void)

{
  byte bVar1;
  undefined4 uVar2;
  undefined1 auVar3 [16];
  undefined1 auVar4 [16];
  undefined *puVar5;
  undefined *puVar6;
  int iVar7;
  int iVar8;
  int iVar9;
  undefined *puVar10;
  int iVar11;
  int iVar12;
  long lVar13;
  undefined *puVar14;
  long lVar16;
  long lVar17;
  int iVar18;
  undefined *puVar15;
  
  iVar12 = DAT_00117120;
  puVar10 = DAT_0010c100;
  uVar2 = *(undefined4 *)(DAT_0010c100 + 8);
  *(undefined8 *)(DAT_0010c100 + 0x18) = 0;
  *(int *)(puVar10 + 8) = iVar12;
  iVar11 = DAT_00117110;
  iVar9 = DAT_0010c0f0;
  iVar8 = DAT_0010c0ec;
  iVar7 = DAT_0010c0e0;
  puVar6 = puVar10;
  while (puVar5 = puVar6 + -0x28, (undefined *)0x10c11f < puVar5) {
    iVar18 = iVar8;
    if (puVar5 == &DAT_0010c120) {
      iVar18 = iVar9;
    }
    lVar16 = 0x7fffffffffffffff;
    iVar18 = iVar18 + *(int *)(puVar6 + -0x20);
    puVar15 = puVar5;
    do {
      puVar14 = puVar15 + 0x28;
      if (puVar10 == puVar14) {
        lVar13 = 0;
      }
      else {
        lVar13 = (long)((iVar11 - iVar18) * 10);
        lVar13 = lVar13 * lVar13;
        if (puVar10 != *(undefined **)(puVar15 + 0x48)) {
          lVar17 = (long)((iVar18 - *(int *)(puVar15 + 0x3c)) * 10);
          lVar13 = lVar13 + (lVar17 * lVar17 >> 1);
        }
      }
      lVar13 = lVar13 + *(long *)(puVar15 + 0x40);
      if ((puVar5 == &DAT_0010c120) && (0 < iVar7)) {
        lVar17 = (long)((iVar18 - iVar7) * 10);
        lVar13 = lVar13 + (lVar17 * lVar17 >> 1);
      }
      if (lVar13 < lVar16) {
        *(undefined **)(puVar6 + -8) = puVar14;
        *(int *)(puVar6 + -0x14) = iVar18;
        lVar16 = lVar13;
      }
    } while ((puVar10 != puVar14) &&
            (iVar18 = iVar18 + *(int *)(puVar15 + 0x30) + *(int *)(puVar15 + 0xc), puVar15 = puVar14
            , iVar18 < iVar12));
    lVar13 = 0x1324;
    if (&DAT_0010c120 < puVar5) {
      bVar1 = puVar6[-0x40];
      if ((bVar1 & 2) == 0) {
        lVar13 = 0xce4;
        if ((((bVar1 & 4) == 0) && (lVar13 = 0x1324, &DAT_0010c148 < puVar5)) &&
           ((puVar6[-0x68] & 8) != 0)) {
          auVar4._8_8_ = 0;
          auVar4._0_8_ = SUB168(SEXT816(40000),8);
          lVar13 = SUB168((auVar4 << 0x40 | ZEXT816(40000)) /
                          SEXT816((long)(*(int *)(puVar6 + -0x48) + 2)),0) + 0x1324;
        }
      }
      else {
        lVar13 = (ulong)(-(uint)((bVar1 & 8) == 0) & 0x58804) + 0x960;
      }
    }
    if ((puVar6[-0x18] & 1) == 0) {
      if ((puVar6[-0x18] & 8) != 0) {
        auVar3._8_8_ = 0;
        auVar3._0_8_ = SUB168(SEXT816(0x57e4),8);
        lVar13 = lVar13 + SUB168((auVar3 << 0x40 | ZEXT816(0x57e4)) /
                                 SEXT816((long)(*(int *)(puVar6 + -0x20) + 2)),0);
      }
    }
    else {
      lVar13 = lVar13 + -0x640;
    }
    *(long *)(puVar6 + -0x10) = lVar16 + lVar13;
    puVar6 = puVar5;
  }
  *(undefined4 *)(puVar10 + 8) = uVar2;
  return;
}




// Function: get_space @ 0x2ea0

void get_space(_IO_FILE *param_1,uint param_2)

{
  int iVar1;
  byte *pbVar2;
  
  do {
    while (param_2 != 0x20) {
      if (param_2 != 9) {
        return;
      }
      DAT_0010c0f8 = 1;
      iVar1 = DAT_0011710c + 7;
      if (-1 < DAT_0011710c) {
        iVar1 = DAT_0011710c;
      }
      DAT_0011710c = (iVar1 >> 3) * 8 + 8;
      pbVar2 = (byte *)param_1->_IO_read_ptr;
      if (pbVar2 < param_1->_IO_read_end) goto LAB_00102ee4;
LAB_00102f0d:
      param_2 = __uflow(param_1);
    }
    DAT_0011710c = DAT_0011710c + 1;
    pbVar2 = (byte *)param_1->_IO_read_ptr;
    if (param_1->_IO_read_end <= pbVar2) goto LAB_00102f0d;
LAB_00102ee4:
    param_1->_IO_read_ptr = (char *)(pbVar2 + 1);
    param_2 = (uint)*pbVar2;
  } while( true );
}




// Function: get_prefix @ 0x2f30

void get_prefix(_IO_FILE *param_1)

{
  byte bVar1;
  byte *pbVar2;
  uint uVar3;
  byte *pbVar4;
  
  pbVar4 = (byte *)param_1->_IO_read_ptr;
  DAT_0011710c = 0;
  if (pbVar4 < param_1->_IO_read_end) {
    param_1->_IO_read_ptr = (char *)(pbVar4 + 1);
    uVar3 = (uint)*pbVar4;
  }
  else {
    uVar3 = __uflow(param_1);
  }
  uVar3 = FUN_00102ea0(param_1,uVar3);
  DAT_0010c0e4 = DAT_0011710c;
  if (DAT_00117114 == 0) {
    if (DAT_00117118 < DAT_0011710c) {
      DAT_0010c0e4 = DAT_00117118;
    }
  }
  else {
    bVar1 = *DAT_00117128;
    pbVar4 = DAT_00117128;
    while( true ) {
      if (bVar1 == 0) {
        FUN_00102ea0(param_1,uVar3);
        return;
      }
      if (bVar1 != uVar3) break;
      DAT_0011710c = DAT_0011710c + 1;
      pbVar2 = (byte *)param_1->_IO_read_ptr;
      if (pbVar2 < param_1->_IO_read_end) {
        param_1->_IO_read_ptr = (char *)(pbVar2 + 1);
        uVar3 = (uint)*pbVar2;
      }
      else {
        uVar3 = __uflow(param_1);
      }
      bVar1 = pbVar4[1];
      pbVar4 = pbVar4 + 1;
    }
  }
  return;
}




// Function: put_word @ 0x3000

void put_word(undefined8 *param_1)

{
  byte bVar1;
  uint uVar2;
  byte *pbVar3;
  byte *pbVar4;
  byte *pbVar5;
  int iVar6;
  
  uVar2 = *(uint *)(param_1 + 1);
  pbVar3 = (byte *)*param_1;
  iVar6 = 0;
  if (uVar2 != 0) {
    pbVar4 = pbVar3;
    do {
      pbVar5 = pbVar4 + 1;
      bVar1 = *pbVar4;
      pbVar4 = (byte *)stdout->_IO_write_ptr;
      if (pbVar4 < stdout->_IO_write_end) {
        stdout->_IO_write_ptr = (char *)(pbVar4 + 1);
        *pbVar4 = bVar1;
      }
      else {
        __overflow(stdout,(uint)bVar1);
      }
      pbVar4 = pbVar5;
    } while (pbVar5 != pbVar3 + uVar2);
    iVar6 = *(int *)(param_1 + 1);
  }
  DAT_00117108 = DAT_00117108 + iVar6;
  return;
}




// Function: put_space @ 0x3060

void put_space(int param_1)

{
  uint uVar1;
  char *pcVar2;
  uint uVar3;
  int iVar4;
  
  uVar1 = DAT_00117108 + param_1;
  if (DAT_0010c0f8 != '\0') {
    uVar3 = uVar1 + 7;
    if (-1 < (int)uVar1) {
      uVar3 = uVar1;
    }
    if ((DAT_00117108 + 1 < (int)(uVar3 & 0xfffffff8)) && (DAT_00117108 < (int)(uVar3 & 0xfffffff8))
       ) {
      do {
        pcVar2 = stdout->_IO_write_ptr;
        if (pcVar2 < stdout->_IO_write_end) {
          stdout->_IO_write_ptr = pcVar2 + 1;
          *pcVar2 = '\t';
        }
        else {
          __overflow(stdout,9);
        }
        iVar4 = DAT_00117108 + 7;
        if (-1 < DAT_00117108) {
          iVar4 = DAT_00117108;
        }
        iVar4 = (iVar4 >> 3) + 1;
        DAT_00117108 = iVar4 * 8;
      } while (iVar4 < (int)uVar3 >> 3);
    }
  }
  for (; DAT_00117108 < (int)uVar1; DAT_00117108 = DAT_00117108 + 1) {
    pcVar2 = stdout->_IO_write_ptr;
    if (pcVar2 < stdout->_IO_write_end) {
      stdout->_IO_write_ptr = pcVar2 + 1;
      *pcVar2 = ' ';
    }
    else {
      __overflow(stdout,0x20);
    }
  }
  return;
}




// Function: put_line @ 0x3140

void put_line(long param_1,int param_2)

{
  long lVar1;
  char *pcVar2;
  
  DAT_00117108 = 0;
  FUN_00103060(DAT_0010c0f4);
  fputs_unlocked(DAT_00117128,stdout);
  DAT_00117108 = DAT_00117114 + DAT_00117108;
  FUN_00103060(param_2 - DAT_00117108);
  lVar1 = *(long *)(param_1 + 0x20);
  for (; param_1 != lVar1 + -0x28; param_1 = param_1 + 0x28) {
    FUN_00103000(param_1);
    FUN_00103060(*(undefined4 *)(param_1 + 0xc));
  }
  FUN_00103000(param_1);
  DAT_0010c0e0 = DAT_00117108;
  pcVar2 = stdout->_IO_write_ptr;
  if (pcVar2 < stdout->_IO_write_end) {
    stdout->_IO_write_ptr = pcVar2 + 1;
    *pcVar2 = '\n';
    return;
  }
  __overflow(stdout,10);
  return;
}




// Function: flush_paragraph @ 0x3200

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
  
  plVar1 = DAT_0010c100;
  if (DAT_0010c100 != (long *)&DAT_0010c120) {
    FUN_00102cb0();
    plVar8 = plVar1;
    if (plVar1 != DAT_0010c140) {
      lVar3 = 0x7fffffffffffffff;
      plVar5 = DAT_0010c140;
      lVar7 = DAT_0010c140[3];
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
    FUN_00103140(&DAT_0010c120,DAT_0010c0f0);
    lVar3 = (long)DAT_00115d60;
    for (plVar1 = DAT_0010c140; DAT_00115d60 = (undefined *)lVar3, plVar8 != plVar1;
        plVar1 = (long *)plVar1[4]) {
      FUN_00103140(plVar1,DAT_0010c0ec);
      lVar3 = (long)DAT_00115d60;
    }
    iVar2 = __memmove_chk(&DAT_00115d80,*plVar8,lVar3 - *plVar8,5000);
    plVar1 = DAT_0010c100;
    lVar7 = *plVar8;
    lVar4 = (long)((int)lVar7 - iVar2);
    DAT_00115d60 = (undefined *)(lVar3 - lVar4);
    plVar5 = plVar8;
    if (plVar8 <= DAT_0010c100) {
      while( true ) {
        plVar6 = plVar5 + 5;
        *plVar5 = lVar7 - lVar4;
        if (plVar1 < plVar6) break;
        lVar7 = *plVar6;
        plVar5 = plVar6;
      }
    }
    __memmove_chk(&DAT_0010c120,plVar8,(undefined *)((long)plVar1 + (0x28 - (long)plVar8)),40000);
    DAT_0010c100 = (long *)((long)plVar1 - (long)(plVar8 + -0x21824));
    return;
  }
  fwrite_unlocked(&DAT_00115d80,1,(long)DAT_00115d60 - 0x115d80,stdout);
  DAT_00115d60 = &DAT_00115d80;
  return;
}




// Function: get_line @ 0x3420

void get_line(_IO_FILE *param_1,uint param_2)

{
  byte *pbVar1;
  byte *pbVar2;
  undefined4 *puVar3;
  bool bVar4;
  int iVar5;
  ushort **ppuVar6;
  undefined4 *puVar7;
  char *pcVar8;
  int iVar9;
  undefined8 *puVar10;
  byte *pbVar11;
  
  ppuVar6 = __ctype_b_loc();
LAB_00103450:
  do {
    puVar7 = DAT_00115d60;
    *DAT_0010c100 = DAT_00115d60;
    puVar3 = DAT_00115d60;
    do {
      while( true ) {
        DAT_00115d60 = puVar7;
        if (DAT_00115d60 == &DAT_00117108) {
          DAT_00115d60 = puVar3;
          FUN_00103390(1);
          FUN_00103200();
        }
        puVar7 = (undefined4 *)((long)DAT_00115d60 + 1);
        *(char *)DAT_00115d60 = (char)param_2;
        pbVar1 = (byte *)param_1->_IO_read_ptr;
        DAT_00115d60 = puVar7;
        if (pbVar1 < param_1->_IO_read_end) {
          param_1->_IO_read_ptr = (char *)(pbVar1 + 1);
          param_2 = (uint)*pbVar1;
        }
        else {
          param_2 = __uflow(param_1);
          puVar7 = DAT_00115d60;
          if (param_2 == 0xffffffff) goto LAB_001034a7;
        }
        puVar3 = DAT_00115d60;
        if ((int)param_2 < 0xe) break;
        if (param_2 == 0x20) goto LAB_001034a7;
      }
    } while ((int)param_2 < 9);
LAB_001034a7:
    puVar10 = DAT_0010c100;
    pbVar2 = (byte *)*DAT_0010c100;
    iVar5 = (int)puVar7 - (int)pbVar2;
    *(int *)(DAT_0010c100 + 1) = iVar5;
    iVar9 = DAT_0011710c + iVar5;
    pbVar1 = pbVar2 + (long)iVar5 + -1;
    DAT_0011710c = iVar9;
    pcVar8 = strchr("([\'`\"",(int)(char)*pbVar2);
    *(byte *)(puVar10 + 2) =
         (byte)(*ppuVar6)[*pbVar1] & 4 | pcVar8 != (char *)0x0 | *(byte *)(puVar10 + 2) & 0xfa;
    if (pbVar2 < pbVar1) {
      do {
        pbVar11 = pbVar1;
        iVar5 = (int)(char)*pbVar11;
        pcVar8 = strchr(")]\'\"",iVar5);
        if (pcVar8 == (char *)0x0) goto LAB_00103544;
        pbVar1 = pbVar11 + -1;
      } while (pbVar2 != pbVar11 + -1);
      iVar5 = (int)(char)pbVar11[-1];
    }
    else {
      iVar5 = (int)(char)*pbVar1;
    }
LAB_00103544:
    pcVar8 = strchr(".?!",iVar5);
    *(byte *)(puVar10 + 2) = *(byte *)(puVar10 + 2) & 0xfd | (pcVar8 != (char *)0x0) * '\x02';
    param_2 = FUN_00102ea0(param_1,param_2);
    puVar10 = DAT_0010c100;
    iVar9 = DAT_0011710c - iVar9;
    *(int *)((long)DAT_0010c100 + 0xc) = iVar9;
    if (param_2 == 0xffffffff) {
      *(byte *)(puVar10 + 2) = *(byte *)(puVar10 + 2) | 8;
LAB_0010363c:
      *(uint *)((long)puVar10 + 0xc) = ((*(byte *)(puVar10 + 2) & 8) != 0) + 1;
      if (puVar10 == (undefined8 *)&DAT_00115d10) goto LAB_001036d0;
    }
    else {
      if ((*(byte *)(puVar10 + 2) & 2) == 0) {
        bVar4 = false;
LAB_001035b7:
        *(byte *)(puVar10 + 2) = *(byte *)(puVar10 + 2) & 0xf7 | bVar4 * '\b';
        if (param_2 == 10) goto LAB_0010363c;
      }
      else {
        bVar4 = 1 < iVar9 || param_2 == 10;
        if (1 < iVar9 || param_2 == 10) goto LAB_001035b7;
        *(byte *)(puVar10 + 2) = *(byte *)(puVar10 + 2) & 0xf7;
      }
      if (DAT_00117130 != '\0') goto LAB_0010363c;
      if (puVar10 != (undefined8 *)&DAT_00115d10) {
        DAT_0010c100 = puVar10 + 5;
        goto LAB_00103450;
      }
LAB_001036d0:
      FUN_00103390(1);
      FUN_00103200();
      puVar10 = DAT_0010c100;
    }
    DAT_0010c100 = puVar10 + 5;
    if ((param_2 == 10) || (param_2 == 0xffffffff)) {
      FUN_00102f30(param_1);
      return;
    }
  } while( true );
}




// Function: fmt @ 0x3700

undefined8 fmt(_IO_FILE *param_1,undefined8 param_2)

{
  byte bVar1;
  byte *pbVar2;
  undefined *puVar3;
  undefined *puVar4;
  uint uVar5;
  int iVar6;
  uint uVar7;
  undefined8 uVar8;
  char *pcVar9;
  int *piVar10;
  uint uVar11;
  ulong uVar12;
  ulong uVar13;
  byte *pbVar14;
  bool bVar15;
  
  FUN_001041c0(param_1,2);
  DAT_0010c0f8 = 0;
  DAT_0010c0ec = 0;
  DAT_0010c0e8 = FUN_00102f30(param_1);
LAB_00103746:
  DAT_0010c0e0 = 0;
  uVar5 = DAT_0010c0e8;
  do {
    if ((uVar5 == 10) || (uVar5 == 0xffffffff)) {
      DAT_00117108 = 0;
      bVar15 = uVar5 != 10 && uVar5 != 0xffffffff;
      if ((int)DAT_0010c0e4 < DAT_0011710c) goto LAB_0010379e;
      if (bVar15) {
        FUN_00103060();
        if (DAT_00117108 != DAT_0011710c) goto LAB_0010393a;
        FUN_00103060(0);
        goto joined_r0x00103955;
      }
LAB_001038db:
      if (uVar5 == 0xffffffff) {
LAB_0010383a:
        DAT_0010c0e8 = 0xffffffff;
        if ((param_1->_flags & 0x20) == 0) {
          if (param_1 == stdin) {
            clearerr_unlocked(param_1);
            return 1;
          }
          iVar6 = FUN_001041f0();
          if (iVar6 == 0) {
            return 1;
          }
          piVar10 = __errno_location();
          iVar6 = *piVar10;
          if (iVar6 < 0) {
            return 1;
          }
          uVar8 = FUN_001063b0(0,3,param_2);
          if (iVar6 != 0) {
            pcVar9 = "%s";
            goto LAB_00103890;
          }
        }
        else {
          if (param_1 == stdin) {
            clearerr_unlocked(param_1);
          }
          else {
            FUN_001041f0();
          }
          uVar8 = FUN_001063b0(0,3,param_2);
        }
        iVar6 = 0;
        pcVar9 = (char *)dcgettext(0,"read error",5);
LAB_00103890:
        error(0,iVar6,pcVar9,uVar8);
        return 0;
      }
    }
    else {
      if ((DAT_00117118 <= (int)DAT_0010c0e4) &&
         ((int)(DAT_0011711c + DAT_0010c0e4) <= DAT_0011710c)) break;
      DAT_00117108 = 0;
      if (DAT_0011710c <= (int)DAT_0010c0e4) {
        FUN_00103060();
        if (DAT_00117108 != DAT_0011710c) {
LAB_0010393a:
          bVar15 = true;
          pbVar14 = DAT_00117128;
          iVar6 = DAT_0011710c;
          goto LAB_001037df;
        }
        FUN_00103060(0);
LAB_00103981:
        do {
          pcVar9 = stdout->_IO_write_ptr;
          if (pcVar9 < stdout->_IO_write_end) {
            stdout->_IO_write_ptr = pcVar9 + 1;
            *pcVar9 = (char)uVar5;
          }
          else {
            __overflow(stdout,uVar5 & 0xff);
          }
          pbVar14 = (byte *)param_1->_IO_read_ptr;
          if (pbVar14 < param_1->_IO_read_end) {
            param_1->_IO_read_ptr = (char *)(pbVar14 + 1);
            uVar5 = (uint)*pbVar14;
            bVar15 = true;
          }
          else {
            uVar5 = __uflow(param_1);
            bVar15 = uVar5 != 0xffffffff;
          }
        } while ((uVar5 != 10) && (bVar15));
        goto LAB_001038db;
      }
      bVar15 = true;
LAB_0010379e:
      DAT_00117108 = 0;
      FUN_00103060();
      pbVar14 = DAT_00117128;
      iVar6 = DAT_0011710c;
      if (DAT_0011710c != DAT_00117108) {
LAB_001037df:
        do {
          bVar1 = *pbVar14;
          if (bVar1 == 0) break;
          pbVar2 = (byte *)stdout->_IO_write_ptr;
          if (pbVar2 < stdout->_IO_write_end) {
            stdout->_IO_write_ptr = (char *)(pbVar2 + 1);
            *pbVar2 = bVar1;
          }
          else {
            __overflow(stdout,(uint)bVar1);
            iVar6 = DAT_0011710c;
          }
          DAT_00117108 = DAT_00117108 + 1;
          pbVar14 = pbVar14 + 1;
        } while (DAT_00117108 != iVar6);
      }
      if (bVar15) {
        FUN_00103060(iVar6 - DAT_00117108);
joined_r0x00103955:
        if (uVar5 != 0xffffffff) goto LAB_00103981;
LAB_00103822:
        if ((int)(DAT_00117114 + DAT_0010c0e4) <= DAT_0011710c) {
          pcVar9 = stdout->_IO_write_ptr;
          if (pcVar9 < stdout->_IO_write_end) {
            stdout->_IO_write_ptr = pcVar9 + 1;
            *pcVar9 = '\n';
          }
          else {
            __overflow(stdout,10);
          }
        }
        goto LAB_0010383a;
      }
      if (uVar5 == 0xffffffff) goto LAB_00103822;
    }
    pcVar9 = stdout->_IO_write_ptr;
    if (pcVar9 < stdout->_IO_write_end) {
      stdout->_IO_write_ptr = pcVar9 + 1;
      *pcVar9 = '\n';
    }
    else {
      __overflow(stdout,10);
    }
    uVar5 = FUN_00102f30(param_1);
  } while( true );
  DAT_0010c0f4 = DAT_0010c0e4;
  DAT_0010c0f0 = DAT_0011710c;
  DAT_00115d60 = &DAT_00115d80;
  DAT_0010c100 = &DAT_0010c120;
  uVar5 = FUN_00103420(param_1,uVar5);
  uVar12 = (ulong)DAT_0010c0e4;
  bVar15 = false;
  uVar13 = (ulong)uVar5;
  if ((DAT_0010c0e4 == DAT_0010c0f4) && ((int)(DAT_0011711c + DAT_0010c0e4) <= DAT_0011710c)) {
    bVar15 = uVar5 != 0xffffffff && uVar5 != 10;
  }
  uVar5 = DAT_0010c0f4;
  FUN_00103390(bVar15);
  uVar7 = (uint)uVar13;
  if (DAT_00117131 == '\0') {
    uVar11 = (uint)uVar12;
    if (DAT_00117133 == '\0') {
      if (DAT_00117132 == '\0') {
        if (uVar11 == uVar5) {
          do {
            uVar7 = (uint)uVar13;
            if ((((DAT_0011710c < (int)uVar12 + DAT_0011711c) || (uVar7 == 10)) ||
                (uVar7 == 0xffffffff)) || (DAT_0010c0ec != DAT_0011710c)) break;
            uVar7 = FUN_00103420(param_1);
            uVar12 = (ulong)DAT_0010c0e4;
            uVar13 = (ulong)uVar7;
          } while (DAT_0010c0e4 == DAT_0010c0f4);
        }
      }
      else if (((uVar11 == uVar5) && ((int)(uVar11 + DAT_0011711c) <= DAT_0011710c)) &&
              ((uVar7 != 10 && ((uVar7 != 0xffffffff && (DAT_0010c0f0 != DAT_0011710c)))))) {
        while (uVar7 = FUN_00103420(param_1), DAT_0010c0e4 == DAT_0010c0f4) {
          if ((((DAT_0011710c < (int)(DAT_0010c0e4 + DAT_0011711c)) || (uVar7 == 10)) ||
              (uVar7 == 0xffffffff)) || (DAT_0010c0ec != DAT_0011710c)) break;
        }
      }
    }
    else if (((uVar11 == uVar5) && ((int)(uVar11 + DAT_0011711c) <= DAT_0011710c)) &&
            ((uVar7 != 10 && (uVar7 != 0xffffffff)))) {
      while (((uVar7 = FUN_00103420(param_1), DAT_0010c0e4 == DAT_0010c0f4 &&
              ((int)(DAT_0010c0e4 + DAT_0011711c) <= DAT_0011710c)) && (uVar7 != 10))) {
        if ((uVar7 == 0xffffffff) || (DAT_0010c0ec != DAT_0011710c)) break;
      }
    }
  }
  puVar4 = DAT_0010c100;
  if (DAT_0010c100 < (undefined *)0x10c121) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("word < word_limit","src/fmt.c",0x270,"get_paragraph");
  }
  DAT_0010c100[-0x18] = DAT_0010c100[-0x18] | 10;
  DAT_0010c0e8 = uVar7;
  FUN_00102cb0();
  FUN_00103140(&DAT_0010c120,DAT_0010c0f0);
  for (puVar3 = DAT_0010c140; puVar3 != puVar4; puVar3 = *(undefined **)(puVar3 + 0x20)) {
    FUN_00103140(puVar3,DAT_0010c0ec);
  }
  goto LAB_00103746;
}




// Function: usage @ 0x3d70

void usage(int param_1)

{
  FILE *pFVar1;
  int iVar2;
  undefined8 uVar3;
  char *pcVar4;
  undefined8 uVar5;
  undefined **ppuVar6;
  undefined *puVar7;
  undefined *puVar8;
  long in_FS_OFFSET;
  undefined *local_b8;
  char *local_b0;
  char *local_a8 [4];
  char *local_88;
  char *local_80;
  char *local_78;
  char *local_70;
  char *local_68;
  char *local_60;
  undefined8 local_58;
  undefined8 local_50;
  undefined8 local_40;
  
  uVar5 = DAT_00117148;
  ppuVar6 = &local_b8;
  local_40 = *(undefined8 *)(in_FS_OFFSET + 0x28);
  if (param_1 == 0) {
    uVar3 = dcgettext(0,"Usage: %s [-WIDTH] [OPTION]... [FILE]...\n",5);
    __printf_chk(1,uVar3,uVar5);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "Reformat each paragraph in the FILE(s), writing to standard output.\nThe option -WIDTH is an abbreviated form of --width=DIGITS.\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,"\nWith no FILE, or when FILE is -, read standard input.\n",5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "\nMandatory arguments to long options are mandatory for short options too.\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
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
    local_b8 = &DAT_0010803a;
    pcVar4 = "[";
    local_b0 = "test invocation";
    local_a8[0] = "coreutils";
    local_a8[1] = "Multi-call invocation";
    local_88 = "sha256sum";
    local_a8[2] = "sha224sum";
    local_78 = "sha384sum";
    local_a8[3] = "sha2 utilities";
    local_80 = "sha2 utilities";
    local_70 = "sha2 utilities";
    local_68 = "sha512sum";
    local_60 = "sha2 utilities";
    local_58 = 0;
    local_50 = 0;
    do {
      iVar2 = strcmp("fmt",pcVar4);
      if (iVar2 == 0) break;
      pcVar4 = *(char **)((long)ppuVar6 + 0x10);
      ppuVar6 = (undefined **)((long)ppuVar6 + 0x10);
    } while (pcVar4 != (char *)0x0);
    puVar8 = *(undefined **)((long)ppuVar6 + 8);
    if (puVar8 == (undefined *)0x0) {
      puVar8 = &DAT_0010803c;
    }
    uVar5 = dcgettext(0,"\n%s online help: <%s>\n",5);
    __printf_chk(1,uVar5,"GNU coreutils","https://www.gnu.org/software/coreutils/");
    pcVar4 = setlocale(5,(char *)0x0);
    if (pcVar4 != (char *)0x0) {
      iVar2 = strncmp(pcVar4,"en_",3);
      pFVar1 = stdout;
      if (iVar2 != 0) {
        pcVar4 = (char *)dcgettext(0,
                                   "Report any translation bugs to <https://translationproject.org/team/>\n"
                                   ,5);
        fputs_unlocked(pcVar4,pFVar1);
      }
    }
    puVar7 = &DAT_0010803c;
    iVar2 = strcmp("fmt","[");
    if (iVar2 == 0) {
      puVar7 = &DAT_00108040;
    }
    uVar5 = dcgettext(0,"Full documentation <%s%s>\n",5);
    pcVar4 = " invocation";
    __printf_chk(1,uVar5,"https://www.gnu.org/software/coreutils/",puVar7);
    if (puVar8 != &DAT_0010803c) {
      pcVar4 = "";
    }
    uVar5 = dcgettext(0,"or available locally via: info \'(coreutils) %s%s\'\n",5);
    __printf_chk(1,uVar5,puVar8,pcVar4);
  }
  else {
    uVar3 = dcgettext(0,"Try \'%s --help\' for more information.\n",5);
    __fprintf_chk(stderr,1,uVar3,uVar5);
  }
                    /* WARNING: Subroutine does not return */
  exit(param_1);
}



