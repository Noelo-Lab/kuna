// Function: usage @ 0x2aa2

void usage(int param_1)

{
  FILE *pFVar1;
  undefined8 uVar2;
  char *pcVar3;
  
  uVar2 = DAT_00118158;
  if (param_1 == 0) {
    pcVar3 = (char *)gettext("Usage: %s [-WIDTH] [OPTION]... [FILE]...\n");
    printf(pcVar3,uVar2);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "Reformat each paragraph in the FILE(s), writing to standard output.\nThe option -WIDTH is an abbreviated form of --width=DIGITS.\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    FUN_001027a9();
    FUN_001027da();
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "  -c, --crown-margin        preserve indentation of first two lines\n  -p, --prefix=STRING       reformat only lines beginning with STRING,\n                              reattaching the prefix to reformatted lines\n  -s, --split-only          split long lines, but do not refill\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "  -t, --tagged-paragraph    indentation of first line different from second\n  -u, --uniform-spacing     one space between words, two after sentences\n  -w, --width=WIDTH         maximum line width (default of 75 columns)\n  -g, --goal=WIDTH          goal width (default of 93% of width)\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext("      --help        display this help and exit\n");
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext("      --version     output version information and exit\n");
    fputs_unlocked(pcVar3,pFVar1);
    FUN_0010280b(&DAT_00109581);
  }
  else {
    pcVar3 = (char *)gettext("Try \'%s --help\' for more information.\n");
    fprintf(stderr,pcVar3,uVar2);
  }
                    /* WARNING: Subroutine does not return */
  exit(param_1);
}




// Function: main @ 0x2bde

byte main(int param_1,undefined8 *param_2)

{
  char *__s1;
  bool bVar1;
  undefined1 uVar2;
  undefined1 uVar3;
  undefined1 uVar4;
  byte bVar5;
  int iVar6;
  undefined8 uVar7;
  FILE *pFVar8;
  undefined8 uVar9;
  int *piVar10;
  undefined8 *local_58;
  int local_4c;
  bool local_3e;
  long local_38;
  long local_30;
  
  local_3e = true;
  local_38 = 0;
  local_30 = 0;
  FUN_0010498f(*param_2);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  FUN_00108540(FUN_00104625);
  DAT_0010d103 = 0;
  DAT_0010d102 = 0;
  DAT_0010d101 = 0;
  DAT_0010d100 = 0;
  DAT_0010d110 = 0x4b;
  DAT_0010d108 = &DAT_001091c3;
  DAT_0010d114 = 0;
  DAT_0010d118 = 0;
  DAT_0010d11c = 0;
  uVar2 = 0;
  uVar3 = 0;
  uVar4 = 0;
  local_58 = param_2;
  local_4c = param_1;
  if (((1 < param_1) &&
      (uVar2 = DAT_0010d100, uVar3 = DAT_0010d101, uVar4 = DAT_0010d102, *(char *)param_2[1] == '-')
      ) && ((int)*(char *)(param_2[1] + 1) - 0x30U < 10)) {
    local_38 = param_2[1] + 1;
    DAT_0010d100 = 0;
    DAT_0010d101 = 0;
    DAT_0010d102 = 0;
    param_2[1] = *param_2;
    local_58 = param_2 + 1;
    local_4c = param_1 + -1;
    uVar2 = DAT_0010d100;
    uVar3 = DAT_0010d101;
    uVar4 = DAT_0010d102;
  }
  do {
    while( true ) {
      DAT_0010d102 = uVar4;
      DAT_0010d101 = uVar3;
      DAT_0010d100 = uVar2;
      iVar6 = getopt_long(local_4c,local_58,"0123456789cstuw:p:g:",&PTR_s_crown_margin_0010ca80,0);
      if (iVar6 == -1) {
        if (local_38 != 0) {
          uVar7 = gettext("invalid width");
          DAT_0010d110 = FUN_00107afa(local_38,0,0x9c4,&DAT_001091c3,uVar7,0);
        }
        if (local_30 == 0) {
          DAT_0010d120 = (DAT_0010d110 * 0xbb) / 200;
        }
        else {
          uVar7 = gettext("invalid width");
          DAT_0010d120 = FUN_00107afa(local_30,0,(long)DAT_0010d110,&DAT_001091c3,uVar7,0);
          if (local_38 == 0) {
            DAT_0010d110 = DAT_0010d120 + 10;
          }
        }
        bVar1 = false;
        if (local_4c == optind) {
          bVar1 = true;
          local_3e = (bool)FUN_001031d1(stdin,&DAT_00109687);
        }
        else {
          for (; optind < local_4c; optind = optind + 1) {
            __s1 = (char *)local_58[optind];
            iVar6 = strcmp(__s1,"-");
            if (iVar6 == 0) {
              bVar5 = FUN_001031d1(stdin,__s1);
              local_3e = (bVar5 & local_3e) != 0;
              bVar1 = true;
            }
            else {
              pFVar8 = fopen(__s1,"r");
              if (pFVar8 == (FILE *)0x0) {
                uVar7 = FUN_00106589(4,__s1);
                uVar9 = gettext("cannot open %s for reading");
                piVar10 = __errno_location();
                error(0,*piVar10,uVar9,uVar7);
                local_3e = false;
              }
              else {
                bVar5 = FUN_001031d1(pFVar8,__s1);
                local_3e = (bVar5 & local_3e) != 0;
              }
            }
          }
        }
        if ((bVar1) && (iVar6 = FUN_00104791(stdin), iVar6 != 0)) {
          uVar7 = gettext("closing standard input");
          piVar10 = __errno_location();
          error(1,*piVar10,&DAT_001096bd,uVar7);
        }
        return local_3e ^ 1;
      }
      if (iVar6 < 0x78) break;
switchD_00102d91_caseD_64:
      if (iVar6 - 0x30U < 10) {
        uVar7 = gettext(
                       "invalid option -- %c; -WIDTH is recognized only when it is the first\noption; use -w N instead"
                       );
        error(0,0,uVar7,iVar6);
      }
      FUN_00102aa2(1);
switchD_00102d91_caseD_63:
      DAT_0010d100 = 1;
      uVar2 = DAT_0010d100;
      uVar3 = DAT_0010d101;
      uVar4 = DAT_0010d102;
    }
    if (iVar6 < 99) {
      if (iVar6 == -0x83) {
LAB_00102e36:
        FUN_00107168(stdout,&DAT_00109581,"GNU coreutils",PTR_DAT_0010d010,"Ross Paterson",0);
                    /* WARNING: Subroutine does not return */
        exit(0);
      }
      if (iVar6 == -0x82) {
        FUN_00102aa2(0);
        goto LAB_00102e36;
      }
      goto switchD_00102d91_caseD_64;
    }
    uVar2 = DAT_0010d100;
    uVar3 = DAT_0010d101;
    uVar4 = DAT_0010d102;
    switch(iVar6) {
    case 99:
      goto switchD_00102d91_caseD_63;
    default:
      goto switchD_00102d91_caseD_64;
    case 0x67:
      local_30 = optarg;
      break;
    case 0x70:
      FUN_0010312d(optarg);
      uVar2 = DAT_0010d100;
      uVar3 = DAT_0010d101;
      uVar4 = DAT_0010d102;
      break;
    case 0x73:
      DAT_0010d102 = 1;
      uVar4 = DAT_0010d102;
      break;
    case 0x74:
      DAT_0010d101 = 1;
      uVar3 = DAT_0010d101;
      break;
    case 0x75:
      DAT_0010d103 = 1;
      break;
    case 0x77:
      local_38 = optarg;
    }
  } while( true );
}




// Function: set_prefix @ 0x312d

void set_prefix(char *param_1)

{
  size_t sVar1;
  char *local_20;
  char *local_10;
  
  DAT_0010d118 = 0;
  for (local_20 = param_1; *local_20 == ' '; local_20 = local_20 + 1) {
    DAT_0010d118 = DAT_0010d118 + 1;
  }
  DAT_0010d108 = local_20;
  sVar1 = strlen(local_20);
  DAT_0010d114 = (int)sVar1;
  for (local_10 = local_20 + DAT_0010d114; (local_20 < local_10 && (local_10[-1] == ' '));
      local_10 = local_10 + -1) {
  }
  *local_10 = '\0';
  DAT_0010d11c = (int)local_10 - (int)local_20;
  return;
}




// Function: fmt @ 0x31d1

uint fmt(FILE *param_1,undefined8 param_2)

{
  char cVar1;
  int iVar2;
  uint *puVar3;
  undefined8 uVar4;
  undefined *puVar5;
  uint local_1c;
  
  FUN_00104752(param_1,2);
  DAT_00118128 = 0;
  DAT_00118134 = 0;
  DAT_00118138 = FUN_00103978(param_1);
  while( true ) {
    cVar1 = FUN_001033b7(param_1);
    if (cVar1 == '\0') break;
    FUN_00103db9();
    FUN_0010411d(DAT_00118120);
  }
  iVar2 = ferror_unlocked(param_1);
  if (iVar2 == 0) {
    local_1c = 0xffffffff;
  }
  else {
    local_1c = 0;
  }
  if (param_1 == stdin) {
    clearerr_unlocked(param_1);
  }
  else {
    iVar2 = FUN_00104791(param_1);
    if ((iVar2 != 0) && ((int)local_1c < 0)) {
      puVar3 = (uint *)__errno_location();
      local_1c = *puVar3;
    }
  }
  if (-1 < (int)local_1c) {
    uVar4 = FUN_0010671b(0,3,param_2);
    if (local_1c == 0) {
      puVar5 = (undefined *)gettext("read error");
    }
    else {
      puVar5 = &DAT_001096bd;
    }
    error(0,local_1c,puVar5,uVar4);
  }
  return local_1c >> 0x1f;
}




// Function: set_other_indent @ 0x32fb

void set_other_indent(char param_1)

{
  if (DAT_0010d102 == '\0') {
    if (DAT_0010d100 == '\0') {
      if (DAT_0010d101 == '\0') {
        DAT_00118134 = DAT_00118130;
      }
      else if ((param_1 == '\0') || (DAT_0010d124 == DAT_00118130)) {
        if (DAT_00118134 == DAT_00118130) {
          if (DAT_00118130 == 0) {
            DAT_00118134 = 3;
          }
          else {
            DAT_00118134 = 0;
          }
        }
      }
      else {
        DAT_00118134 = DAT_0010d124;
      }
    }
    else {
      DAT_00118134 = DAT_00118130;
      if (param_1 != '\0') {
        DAT_00118134 = DAT_0010d124;
      }
    }
  }
  else {
    DAT_00118134 = DAT_00118130;
  }
  return;
}




// Function: get_paragraph @ 0x33b7

undefined8 get_paragraph(undefined8 param_1)

{
  undefined *puVar1;
  undefined1 uVar2;
  char cVar3;
  int iVar4;
  int local_c;
  
  DAT_00118140 = 0;
  local_c = DAT_00118138;
  while ((((local_c == 10 || (local_c == -1)) || (DAT_0011813c < DAT_0010d118)) ||
         (DAT_0010d124 < DAT_0011813c + DAT_0010d114))) {
    iVar4 = FUN_00103630(param_1,local_c);
    if (iVar4 == -1) {
      DAT_00118138 = 0xffffffff;
      return 0;
    }
    putchar_unlocked(10);
    local_c = FUN_00103978(param_1);
  }
  DAT_0011812c = DAT_0011813c;
  DAT_00118130 = DAT_0010d124;
  DAT_0010e4c8 = &DAT_0010d140;
  DAT_00118120 = &DAT_0010e4e0;
  local_c = FUN_00103793(param_1,local_c);
  uVar2 = FUN_00103743(local_c);
  FUN_001032fb(uVar2);
  if (DAT_0010d102 == '\0') {
    if (DAT_0010d100 == '\0') {
      if (DAT_0010d101 == '\0') {
        while ((cVar3 = FUN_00103743(local_c), cVar3 != '\0' && (DAT_0010d124 == DAT_00118134))) {
          local_c = FUN_00103793(param_1,local_c);
        }
      }
      else {
        cVar3 = FUN_00103743(local_c);
        if ((cVar3 != '\0') && (DAT_0010d124 != DAT_00118130)) {
          do {
            local_c = FUN_00103793(param_1,local_c);
            cVar3 = FUN_00103743(local_c);
            if (cVar3 == '\0') break;
          } while (DAT_0010d124 == DAT_00118134);
        }
      }
    }
    else {
      cVar3 = FUN_00103743(local_c);
      if (cVar3 != '\0') {
        do {
          local_c = FUN_00103793(param_1,local_c);
          cVar3 = FUN_00103743(local_c);
          if (cVar3 == '\0') break;
        } while (DAT_0010d124 == DAT_00118134);
      }
    }
  }
  puVar1 = DAT_00118120;
  if ((undefined *)0x10e4e0 < DAT_00118120) {
    DAT_00118120[-0x18] = DAT_00118120[-0x18] | 8;
    DAT_00118120[-0x18] = DAT_00118120[-0x18] & 0xfd | ((byte)puVar1[-0x18] >> 3 & 1) * '\x02';
    DAT_00118138 = local_c;
    return 1;
  }
                    /* WARNING: Subroutine does not return */
  __assert_fail("word < word_limit","src/fmt.c",0x270,"get_paragraph");
}




// Function: copy_rest @ 0x3630

int copy_rest(FILE *param_1,int param_2)

{
  int local_24;
  char *local_10;
  
  DAT_0010d128 = 0;
  local_24 = param_2;
  if ((DAT_0011813c < DAT_0010d124) || ((param_2 != 10 && (param_2 != -1)))) {
    FUN_001042aa(DAT_0011813c);
    local_10 = DAT_0010d108;
    while ((DAT_0010d128 != DAT_0010d124 && (*local_10 != '\0'))) {
      putchar_unlocked((int)*local_10);
      DAT_0010d128 = DAT_0010d128 + 1;
      local_10 = local_10 + 1;
    }
    if ((param_2 != -1) && (param_2 != 10)) {
      FUN_001042aa(DAT_0010d124 - DAT_0010d128);
    }
    if ((param_2 == -1) && (DAT_0011813c + DAT_0010d11c <= DAT_0010d124)) {
      putchar_unlocked(10);
    }
  }
  while ((local_24 != 10 && (local_24 != -1))) {
    putchar_unlocked(local_24);
    local_24 = getc_unlocked(param_1);
  }
  return local_24;
}




// Function: same_para @ 0x3743

undefined4 same_para(int param_1)

{
  undefined4 uVar1;
  
  if ((((DAT_0011813c == DAT_0011812c) && (DAT_0011813c + DAT_0010d114 <= DAT_0010d124)) &&
      (param_1 != 10)) && (param_1 != -1)) {
    uVar1 = 1;
  }
  else {
    uVar1 = 0;
  }
  return uVar1;
}




// Function: get_line @ 0x3793

void get_line(FILE *param_1,int param_2)

{
  int iVar1;
  undefined8 *puVar2;
  char cVar3;
  undefined4 uVar4;
  int local_34;
  
  local_34 = param_2;
  do {
    *DAT_00118120 = DAT_0010e4c8;
    do {
      if ((undefined8 **)DAT_0010e4c8 == &DAT_0010e4c8) {
        FUN_001032fb(1);
        FUN_00103bd2();
      }
      puVar2 = (undefined8 *)((long)DAT_0010e4c8 + 1);
      *(char *)DAT_0010e4c8 = (char)local_34;
      DAT_0010e4c8 = puVar2;
      local_34 = getc_unlocked(param_1);
      if (local_34 == -1) break;
      cVar3 = FUN_00104512(local_34);
    } while (cVar3 != '\x01');
    puVar2 = DAT_00118120;
    *(int *)(DAT_00118120 + 1) = (int)DAT_0010e4c8 - (int)*DAT_00118120;
    DAT_0010d124 = DAT_0010d124 + *(int *)(puVar2 + 1);
    FUN_00103aba(DAT_00118120);
    iVar1 = DAT_0010d124;
    local_34 = FUN_00103a4c(param_1,local_34);
    *(int *)((long)DAT_00118120 + 0xc) = DAT_0010d124 - iVar1;
    if ((local_34 == -1) ||
       (((*(byte *)(DAT_00118120 + 2) & 2) != 0 &&
        ((local_34 == 10 || (1 < *(int *)((long)DAT_00118120 + 0xc))))))) {
      cVar3 = '\x01';
    }
    else {
      cVar3 = '\0';
    }
    *(byte *)(DAT_00118120 + 2) = *(byte *)(DAT_00118120 + 2) & 0xf7 | cVar3 * '\b';
    if (((local_34 == 10) || (local_34 == -1)) || (DAT_0010d103 != '\0')) {
      if ((*(byte *)(DAT_00118120 + 2) & 8) == 0) {
        uVar4 = 1;
      }
      else {
        uVar4 = 2;
      }
      *(undefined4 *)((long)DAT_00118120 + 0xc) = uVar4;
    }
    if (DAT_00118120 == (undefined8 *)&DAT_001180d0) {
      FUN_001032fb(1);
      FUN_00103bd2();
    }
    DAT_00118120 = DAT_00118120 + 5;
    if ((local_34 == 10) || (local_34 == -1)) {
      FUN_00103978(param_1);
      return;
    }
  } while( true );
}




// Function: get_prefix @ 0x3978

uint get_prefix(FILE *param_1)

{
  int iVar1;
  uint local_14;
  byte *local_10;
  
  DAT_0010d124 = 0;
  iVar1 = getc_unlocked(param_1);
  local_14 = FUN_00103a4c(param_1,iVar1);
  if (DAT_0010d11c == 0) {
    DAT_0011813c = DAT_0010d118;
    if (DAT_0010d124 <= DAT_0010d118) {
      DAT_0011813c = DAT_0010d124;
    }
  }
  else {
    DAT_0011813c = DAT_0010d124;
    for (local_10 = DAT_0010d108; *local_10 != 0; local_10 = local_10 + 1) {
      if (local_14 != *local_10) {
        return local_14;
      }
      DAT_0010d124 = DAT_0010d124 + 1;
      local_14 = getc_unlocked(param_1);
    }
    local_14 = FUN_00103a4c(param_1,local_14);
  }
  return local_14;
}




// Function: get_space @ 0x3a4c

int get_space(FILE *param_1,int param_2)

{
  undefined4 local_14;
  
  local_14 = param_2;
  do {
    if (local_14 == 0x20) {
      DAT_0010d124 = DAT_0010d124 + 1;
    }
    else {
      if (local_14 != 9) {
        return local_14;
      }
      DAT_00118128 = 1;
      if (DAT_0010d124 < 0) {
        DAT_0010d124 = DAT_0010d124 + 7;
      }
      DAT_0010d124 = ((DAT_0010d124 >> 3) + 1) * 8;
    }
    local_14 = getc_unlocked(param_1);
  } while( true );
}




// Function: check_punctuation @ 0x3aba

void check_punctuation(undefined8 *param_1)

{
  byte bVar1;
  byte *pbVar2;
  char *pcVar3;
  ushort **ppuVar4;
  byte *local_18;
  
  pbVar2 = (byte *)*param_1;
  local_18 = pbVar2 + (long)*(int *)(param_1 + 1) + -1;
  bVar1 = *local_18;
  pcVar3 = strchr("([\'`\"",(int)(char)*pbVar2);
  *(byte *)(param_1 + 2) = *(byte *)(param_1 + 2) & 0xfe | pcVar3 != (char *)0x0;
  ppuVar4 = __ctype_b_loc();
  *(byte *)(param_1 + 2) = *(byte *)(param_1 + 2) & 0xfb | (((*ppuVar4)[bVar1] & 4) != 0) * '\x04';
  for (; pbVar2 < local_18; local_18 = local_18 + -1) {
    pcVar3 = strchr(")]\'\"",(int)(char)*local_18);
    if (pcVar3 == (char *)0x0) break;
  }
  pcVar3 = strchr(".?!",(int)(char)*local_18);
  *(byte *)(param_1 + 2) = *(byte *)(param_1 + 2) & 0xfd | (pcVar3 != (char *)0x0) * '\x02';
  return;
}




// Function: flush_paragraph @ 0x3bd2

void flush_paragraph(void)

{
  int iVar1;
  long *local_20;
  long *local_18;
  long local_10;
  
  if (DAT_00118120 == (long *)&DAT_0010e4e0) {
    fwrite_unlocked(&DAT_0010d140,1,(long)DAT_0010e4c8 - 0x10d140,stdout);
    DAT_0010e4c8 = &DAT_0010d140;
  }
  else {
    FUN_00103db9();
    local_20 = DAT_00118120;
    local_10 = 0x7fffffffffffffff;
    for (local_18 = DAT_0010e500; local_18 != DAT_00118120; local_18 = (long *)local_18[4]) {
      if (local_18[3] - *(long *)(local_18[4] + 0x18) < local_10) {
        local_20 = local_18;
        local_10 = local_18[3] - *(long *)(local_18[4] + 0x18);
      }
      if (local_10 < 0x7ffffffffffffff7) {
        local_10 = local_10 + 9;
      }
    }
    FUN_0010411d(local_20);
    memmove(&DAT_0010d140,(void *)*local_20,(long)DAT_0010e4c8 - *local_20);
    iVar1 = (int)*local_20 + -0x10d140;
    DAT_0010e4c8 = (undefined *)((long)DAT_0010e4c8 - (long)iVar1);
    for (local_18 = local_20; local_18 <= DAT_00118120; local_18 = local_18 + 5) {
      *local_18 = *local_18 - (long)iVar1;
    }
    memmove(&DAT_0010e4e0,local_20,(size_t)((long)DAT_00118120 + (0x28 - (long)local_20)));
    DAT_00118120 = (long *)((long)DAT_00118120 - (long)(local_20 + -0x21c9c));
  }
  return;
}




// Function: fmt_paragraph @ 0x3db9

void fmt_paragraph(void)

{
  undefined *puVar1;
  undefined4 uVar2;
  undefined *puVar3;
  long lVar4;
  int local_30;
  undefined *local_28;
  undefined *local_20;
  long local_18;
  long local_10;
  
  *(undefined8 *)(DAT_00118120 + 0x18) = 0;
  uVar2 = *(undefined4 *)(DAT_00118120 + 8);
  *(int *)(DAT_00118120 + 8) = DAT_0010d110;
  puVar3 = DAT_00118120;
  while (local_28 = puVar3 + -0x28, &DAT_0010e4e0 <= local_28) {
    local_10 = 0x7fffffffffffffff;
    local_30 = DAT_00118134;
    if (local_28 == &DAT_0010e4e0) {
      local_30 = DAT_00118130;
    }
    local_20 = local_28;
    local_30 = local_30 + *(int *)(puVar3 + -0x20);
    do {
      puVar1 = local_20 + 0x28;
      local_18 = FUN_0010406b(puVar1,local_30);
      local_18 = *(long *)(local_20 + 0x40) + local_18;
      if ((local_28 == &DAT_0010e4e0) && (0 < DAT_00118140)) {
        local_18 = local_18 +
                   ((long)((local_30 - DAT_00118140) * 10) * (long)((local_30 - DAT_00118140) * 10))
                   / 2;
      }
      if (local_18 < local_10) {
        local_10 = local_18;
        *(undefined **)(puVar3 + -8) = puVar1;
        *(int *)(puVar3 + -0x14) = local_30;
      }
    } while ((puVar1 != DAT_00118120) &&
            (local_30 = local_30 + *(int *)(local_20 + 0x30) + *(int *)(local_20 + 0xc),
            local_20 = puVar1, local_30 < DAT_0010d110));
    lVar4 = FUN_00103f67(local_28);
    *(long *)(puVar3 + -0x10) = local_10 + lVar4;
    puVar3 = local_28;
  }
  *(undefined4 *)(DAT_00118120 + 8) = uVar2;
  return;
}




// Function: base_cost @ 0x3f67

long base_cost(undefined *param_1)

{
  undefined1 auVar1 [16];
  undefined1 auVar2 [16];
  long local_10;
  
  local_10 = 0x1324;
  if (&DAT_0010e4e0 < param_1) {
    if ((param_1[-0x18] & 2) == 0) {
      if ((param_1[-0x18] & 4) == 0) {
        if ((&DAT_0010e508 < param_1) && ((param_1[-0x40] & 8) != 0)) {
          auVar1._8_8_ = 0;
          auVar1._0_8_ = SUB168(SEXT816(40000),8);
          local_10 = SUB168((auVar1 << 0x40 | ZEXT816(40000)) /
                            SEXT816((long)(*(int *)(param_1 + -0x20) + 2)),0) + 0x1324;
        }
      }
      else {
        local_10 = 0xce4;
      }
    }
    else if ((param_1[-0x18] & 8) == 0) {
      local_10 = 0x59164;
    }
    else {
      local_10 = 0x960;
    }
  }
  if ((param_1[0x10] & 1) == 0) {
    if ((param_1[0x10] & 8) != 0) {
      auVar2._8_8_ = 0;
      auVar2._0_8_ = SUB168(SEXT816(0x57e4),8);
      local_10 = local_10 +
                 SUB168((auVar2 << 0x40 | ZEXT816(0x57e4)) /
                        SEXT816((long)(*(int *)(param_1 + 8) + 2)),0);
    }
  }
  else {
    local_10 = local_10 + -0x640;
  }
  return local_10;
}




// Function: line_cost @ 0x406b

long line_cost(long param_1,int param_2)

{
  long lVar1;
  
  if (param_1 == DAT_00118120) {
    lVar1 = 0;
  }
  else {
    lVar1 = (long)((DAT_0010d120 - param_2) * 10) * (long)((DAT_0010d120 - param_2) * 10);
    if (*(long *)(param_1 + 0x20) != DAT_00118120) {
      param_2 = param_2 - *(int *)(param_1 + 0x14);
      lVar1 = lVar1 + ((long)(param_2 * 10) * (long)(param_2 * 10)) / 2;
    }
  }
  return lVar1;
}




// Function: put_paragraph @ 0x411d

void put_paragraph(long param_1)

{
  long local_10;
  
  FUN_0010417f(&DAT_0010e4e0,DAT_00118130);
  for (local_10 = DAT_0010e500; local_10 != param_1; local_10 = *(long *)(local_10 + 0x20)) {
    FUN_0010417f(local_10,DAT_00118134);
  }
  return;
}




// Function: put_line @ 0x417f

void put_line(long param_1,int param_2)

{
  long lVar1;
  undefined8 local_20;
  
  DAT_0010d128 = 0;
  FUN_001042aa(DAT_0011812c);
  fputs_unlocked(DAT_0010d108,stdout);
  DAT_0010d128 = DAT_0010d11c + DAT_0010d128;
  FUN_001042aa(param_2 - DAT_0010d128);
  lVar1 = *(long *)(param_1 + 0x20);
  for (local_20 = param_1; local_20 != lVar1 + -0x28; local_20 = local_20 + 0x28) {
    FUN_00104248(local_20);
    FUN_001042aa(*(undefined4 *)(local_20 + 0xc));
  }
  FUN_00104248(local_20);
  DAT_00118140 = DAT_0010d128;
  putchar_unlocked(10);
  return;
}




// Function: put_word @ 0x4248

void put_word(undefined8 *param_1)

{
  int local_14;
  char *local_10;
  
  local_10 = (char *)*param_1;
  for (local_14 = *(int *)(param_1 + 1); local_14 != 0; local_14 = local_14 + -1) {
    putchar_unlocked((int)*local_10);
    local_10 = local_10 + 1;
  }
  DAT_0010d128 = DAT_0010d128 + *(int *)(param_1 + 1);
  return;
}




// Function: put_space @ 0x42aa

void put_space(int param_1)

{
  int iVar1;
  
  param_1 = param_1 + DAT_0010d128;
  if (DAT_00118128 != '\0') {
    iVar1 = param_1;
    if (param_1 < 0) {
      iVar1 = param_1 + 7;
    }
    iVar1 = (iVar1 >> 3) << 3;
    if (DAT_0010d128 + 1 < iVar1) {
      for (; DAT_0010d128 < iVar1; DAT_0010d128 = ((DAT_0010d128 >> 3) + 1) * 8) {
        putchar_unlocked(9);
        if (DAT_0010d128 < 0) {
          DAT_0010d128 = DAT_0010d128 + 7;
        }
      }
    }
  }
  for (; DAT_0010d128 < param_1; DAT_0010d128 = DAT_0010d128 + 1) {
    putchar_unlocked(0x20);
  }
  return;
}



