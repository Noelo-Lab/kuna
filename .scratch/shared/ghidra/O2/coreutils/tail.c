// Function: main @ 0x2960

byte main(ulong param_1,pollfd param_2)

{
  undefined1 *puVar1;
  undefined1 uVar2;
  uint uVar3;
  undefined *puVar4;
  __dev_t _Var5;
  byte bVar6;
  char cVar7;
  byte bVar8;
  byte bVar9;
  int iVar10;
  int iVar11;
  uint uVar12;
  uint32_t __mask;
  int iVar13;
  int iVar14;
  stat *psVar15;
  undefined **ppuVar16;
  void *pvVar17;
  pollfd pVar18;
  __blksize_t _Var19;
  pollfd pVar20;
  long lVar21;
  int *piVar22;
  char *pcVar23;
  undefined8 uVar24;
  __off_t _Var25;
  ulong uVar26;
  size_t sVar27;
  long lVar28;
  undefined8 uVar29;
  undefined **ppuVar30;
  pollfd pVar31;
  undefined8 extraout_RDX;
  pollfd pVar32;
  pollfd extraout_RDX_00;
  char *pcVar33;
  byte bVar34;
  uint *puVar35;
  stat *psVar36;
  uint uVar37;
  stat *__ptr;
  pollfd pVar38;
  undefined8 in_R10;
  char *pcVar39;
  undefined **ppuVar40;
  char *pcVar41;
  long in_FS_OFFSET;
  bool bVar42;
  pollfd local_260;
  size_t local_258;
  stat *local_250;
  stat *local_248;
  pollfd local_240;
  pollfd local_238;
  char *local_230;
  stat *local_228;
  pollfd local_218;
  uint local_210;
  byte local_209;
  pollfd local_208;
  stat *local_200;
  stat *local_1f8;
  pollfd local_1f0;
  long local_1e8;
  pollfd local_1d8;
  pollfd local_1c8 [8];
  int local_184;
  stat local_168;
  stat local_d8;
  long local_40;
  
  pcVar41 = "";
  pVar18 = (pollfd)(param_1 & 0xffffffff);
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_1d8.fd = 10;
  local_1d8.events = 0;
  local_1d8.revents = 0;
  FUN_00109150(*(undefined **)param_2);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  FUN_0010cf70(FUN_001079b0);
  DAT_0011310e = '\0';
  DAT_00113119 = 1;
  DAT_00113115 = '\0';
  DAT_00113116 = 0;
  DAT_00113118 = 0;
  DAT_00113114 = '\n';
  iVar10 = (int)param_1;
  if (iVar10 != 2) {
    if (iVar10 == 3) {
      pcVar39 = *(undefined **)((long)param_2 + 0x10);
      if ((*pcVar39 != '-') || (pcVar39[1] == '\0')) goto LAB_00102a30;
    }
    else {
      if (iVar10 != 4) goto LAB_00102a52;
      pcVar39 = *(undefined **)((long)param_2 + 0x10);
    }
    iVar10 = strcmp(pcVar39,"--");
    if (iVar10 != 0) goto LAB_00102a52;
  }
LAB_00102a30:
  iVar10 = FUN_001090c0();
  pcVar39 = *(undefined **)((long)param_2 + 8);
  pVar38 = (pollfd)((long)pcVar39 + 1);
  if (*pcVar39 == '+') {
    bVar8 = 1;
    if (0x2b8 < iVar10 - 0x30db0U) goto LAB_001038a5;
  }
  else if ((*pcVar39 == '-') &&
          ((iVar10 < 0x30db0 || (pcVar39[(ulong)(pcVar39[1] == 'c') + 1] != '\0')))) {
    bVar8 = 0;
LAB_001038a5:
    cVar7 = pcVar39[1];
    pVar20 = pVar38;
    while ((int)cVar7 - 0x30U < 10) {
      pcVar39 = (char *)((long)pVar20 + 1);
      pVar20 = (pollfd)((long)pVar20 + 1);
      cVar7 = *pcVar39;
    }
    if (cVar7 == 'c') goto LAB_00104240;
    pVar32 = pVar20;
    if (cVar7 == 'l') {
      iVar10 = 1;
      pVar31.fd = 10;
      pVar31.events = 0;
      pVar31.revents = 0;
      goto LAB_001041b9;
    }
    if (cVar7 == 'b') {
      iVar10 = 0;
      pVar31.fd = 0x1400;
      pVar31.events = 0;
      pVar31.revents = 0;
      goto LAB_001041b9;
    }
    iVar10 = 1;
    pVar31.fd = 10;
    pVar31.events = 0;
    pVar31.revents = 0;
    goto LAB_001038fb;
  }
LAB_00102a52:
  lVar21 = 0;
LAB_00102a54:
  local_260.fd = 0;
  ppuVar16 = (undefined **)((long)param_2 + lVar21 * 8);
  uVar37 = pVar18.fd - (int)lVar21;
  ppuVar30 = (undefined **)(ulong)uVar37;
  ppuVar40 = &PTR_s_bytes_0010e133_0x12_00112900;
  pcVar39 = "c:n:fFqs:vz0123456789";
  local_208 = DAT_0010e4a0;
  puVar35 = &switchD_00102ad3::switchdataD_0010e338;
LAB_00102a90:
  iVar10 = getopt_long(uVar37,ppuVar16,"c:n:fFqs:vz0123456789",&PTR_s_bytes_0010e133_0x12_00112900,0
                      );
  if (iVar10 != -1) {
    if (0x85 < iVar10) {
switchD_00102ad3_caseD_3a:
      iVar10 = FUN_00106f70(1);
switchD_00102ad3_caseD_30:
      uVar24 = dcgettext(0,"option used in invalid context -- %c",5);
      error(1,0,uVar24,iVar10);
      local_260.fd = iVar10;
      goto LAB_001042f0;
    }
    if (iVar10 < 0x30) {
      if (iVar10 == -0x83) {
LAB_0010479d:
        FUN_0010bb50(stdout,&DAT_0010e03b,"GNU coreutils",PTR_DAT_00113030,"Paul Rubin",
                     "David MacKenzie","Ian Lance Taylor","Jim Meyering",0,in_R10);
                    /* WARNING: Subroutine does not return */
        exit(0);
      }
      if (iVar10 == -0x82) {
        FUN_00106f70(0);
        uVar24 = extraout_RDX;
LAB_00102d30:
        uVar24 = dcgettext(0,"invalid number of bytes",uVar24);
        goto LAB_00102b8c;
      }
      goto switchD_00102ad3_caseD_3a;
    }
    switch(iVar10) {
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
      goto switchD_00102ad3_caseD_30;
    default:
      goto switchD_00102ad3_caseD_3a;
    case 0x46:
      DAT_00113118 = 1;
      DAT_00113020 = 1;
      DAT_0011311a = 1;
      break;
    case 99:
    case 0x6e:
      DAT_00113119 = iVar10 == 0x6e;
      if (*optarg == '+') {
        DAT_00113116 = 1;
      }
      else if (*optarg == '-') {
        optarg = optarg + 1;
      }
      uVar24 = 5;
      if (iVar10 != 0x6e) goto LAB_00102d30;
      uVar24 = dcgettext(0,"invalid number of lines",5);
LAB_00102b8c:
      local_1d8 = (pollfd)FUN_0010c370(optarg,0,0xffffffffffffffff,"bkKmMGTPEZY0",uVar24,0);
      break;
    case 0x66:
    case 0x84:
      DAT_00113118 = 1;
      if (optarg == (char *)0x0) {
        DAT_00113020 = 2;
      }
      else {
        lVar21 = FUN_00107710("--follow",optarg,&PTR_s_descriptor_00112b00,&DAT_0010e498,4,
                              PTR_FUN_00113038,1);
        DAT_00113020 = *(int *)(&DAT_0010e498 + lVar21 * 4);
      }
      break;
    case 0x73:
      cVar7 = FUN_0010c450(optarg,0,local_1c8,FUN_001078d0);
      if (cVar7 != '\0') {
        local_208 = local_1c8[0];
        if (0.0 <= (double)local_1c8[0]) break;
      }
      ppuVar16 = (undefined **)FUN_0010b520(optarg);
      uVar24 = dcgettext(0,"invalid number of seconds: %s",5);
      error(1,0,uVar24,ppuVar16);
    case 0x71:
      local_260.fd = 2;
      break;
    case 0x76:
      local_260.fd = 1;
      break;
    case 0x7a:
      DAT_00113114 = '\0';
      break;
    case 0x80:
      DAT_0011311a = 1;
      break;
    case 0x81:
      uVar24 = dcgettext(0,"invalid maximum number of unchanged stats between opens",5);
      DAT_00113018 = (undefined *)FUN_0010c370(optarg,0,0xffffffffffffffff,pcVar41,uVar24,0);
      break;
    case 0x82:
      uVar24 = dcgettext(0,"invalid PID",5);
      DAT_00113110 = FUN_0010c370(optarg,0,0x7fffffff,pcVar41,uVar24,0);
      break;
    case 0x83:
      DAT_0011310d = '\x01';
      break;
    case 0x85:
      goto switchD_00102ad3_caseD_85;
    }
    goto LAB_00102a90;
  }
  if (DAT_0011311a == 0) {
LAB_00103167:
    if (DAT_00113110 != 0) {
      if (DAT_00113118 == 0) {
        uVar24 = dcgettext(0,"warning: PID ignored; --pid=PID is useful only when following",5);
        error(0,0,uVar24);
      }
      else {
LAB_00102da1:
        iVar10 = kill(DAT_00113110,0);
        if ((iVar10 != 0) && (piVar22 = __errno_location(), *piVar22 == 0x26)) {
          uVar24 = dcgettext(0,"warning: --pid=PID is not supported on this system",5);
          error(0,0,uVar24);
          DAT_00113110 = 0;
        }
      }
    }
  }
  else {
    if (DAT_00113118 == 0) {
      DAT_0011311a = 0;
      pcVar41 = "warning: --retry ignored; --retry is useful only when following";
LAB_00103152:
      uVar24 = dcgettext(0,pcVar41,5);
      error(0,0,uVar24);
      goto LAB_00103167;
    }
    if (DAT_00113020 == 2) {
      pcVar41 = "warning: --retry only effective for the initial open";
      goto LAB_00103152;
    }
    if (DAT_00113110 != 0) goto LAB_00102da1;
  }
  if ((DAT_00113116 != 0) && (local_1d8 != (pollfd)0x0)) {
    local_1d8 = (pollfd)((long)local_1d8 + -1);
  }
  ppuVar30 = &PTR_DAT_00113028;
  local_230 = (char *)0x1;
  if (optind < (int)uVar37) {
    ppuVar30 = ppuVar16 + optind;
    local_230 = (char *)(long)(int)(uVar37 - optind);
  }
  ppuVar40._0_4_ = 0;
  ppuVar40._4_2_ = 0;
  ppuVar40._6_2_ = 0;
  pcVar41 = "-";
  ppuVar16 = (undefined **)0x1;
  puVar35 = (uint *)(ppuVar30 + (long)local_230);
  pcVar39 = (char *)ppuVar30;
  do {
    iVar10 = strcmp(*(char **)pcVar39,"-");
    if (iVar10 == 0) {
      ppuVar40._0_4_ = 1;
      ppuVar40._4_2_ = 0;
      ppuVar40._6_2_ = 0;
    }
    pcVar39 = (char *)((long)pcVar39 + 8);
  } while ((uint *)pcVar39 != puVar35);
  if ((char)ppuVar40 == '\0') {
LAB_00102e85:
    if ((local_1d8 != (pollfd)0x0) || (DAT_00113118 != 0)) goto LAB_00102e9d;
  }
  else {
    if (DAT_00113020 == 1) {
      uVar24 = FUN_0010ae30(4,&DAT_0010e101);
      uVar29 = dcgettext(0,"cannot follow %s by name",5);
      error(1,0,uVar29,uVar24);
      goto LAB_00104bc0;
    }
    if (DAT_00113118 != 0) {
      if (((DAT_00113110 == 0) && (local_230 == (char *)0x1)) && (DAT_00113020 == 2)) {
        pcVar39 = (char *)&local_d8;
        iVar10 = fstat(0,(stat *)pcVar39);
        if ((iVar10 == 0) && ((local_d8.st_mode & 0xf000) != 0x8000)) goto LAB_00102e85;
      }
      iVar10 = isatty(0);
      if (iVar10 != 0) {
LAB_001042f0:
        uVar24 = dcgettext(0,"warning: following standard input indefinitely is ineffective",5);
        error(0,0,uVar24);
      }
      goto LAB_00102e85;
    }
    if (local_1d8 != (pollfd)0x0) goto LAB_00102e9d;
  }
  bVar8 = 0;
  if (DAT_00113116 == 0) {
LAB_00103381:
    if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
      return bVar8;
    }
LAB_00104bc0:
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
LAB_00102e9d:
  local_1f0 = (pollfd)FUN_0010be60(local_230,0x60);
  pVar18 = local_1f0;
  do {
    puVar4 = *ppuVar30;
    ppuVar30 = ppuVar30 + 1;
    *(undefined **)pVar18 = puVar4;
    pVar18 = (pollfd)((long)pVar18 + 0x60);
  } while (ppuVar30 != (undefined **)puVar35);
  if ((local_260.fd == 1) || ((local_260.fd == 0 && (local_230 != (char *)0x1)))) {
    DAT_00113115 = '\x01';
  }
  local_209 = 1;
  local_1e8 = (long)local_230 * 0x60;
  local_218 = (pollfd)((long)local_1f0 + local_1e8);
  local_260 = local_1f0;
LAB_00102f20:
  local_240 = local_1d8;
  local_250 = *(stat **)local_260;
  iVar10 = strcmp((char *)local_250,pcVar41);
  if (iVar10 == 0) {
    DAT_0011310e = '\x01';
    local_258 = 0;
LAB_00102f60:
    *(char *)((long)local_260 + 0x36) = '\x01';
  }
  else {
    uVar37 = FUN_00107dd0(local_250,0);
    bVar8 = DAT_0011311a;
    ppuVar16 = (undefined **)(ulong)DAT_0011311a;
    local_258 = (size_t)uVar37;
    if (DAT_0011311a != 0) {
      local_250 = *(stat **)local_260;
      if (uVar37 == 0xffffffff) {
        *(char *)((long)local_260 + 0x36) = '\0';
        goto LAB_001032ba;
      }
      goto LAB_00102f60;
    }
    local_250 = *(stat **)local_260;
    *(char *)((long)local_260 + 0x36) = '\x01';
    if (uVar37 == 0xffffffff) {
LAB_001032ba:
      piVar22 = __errno_location();
      if (DAT_00113118 != 0) {
        iVar10 = *piVar22;
        *(undefined4 *)((long)local_260 + 0x38) = 0xffffffff;
        *(int *)((long)local_260 + 0x3c) = iVar10;
        *(byte *)((long)local_260 + 0x34) = bVar8 ^ 1;
        *(undefined **)((long)local_260 + 0x28) = (undefined *)0x0;
        *(undefined **)((long)local_260 + 0x20) = (undefined *)0x0;
      }
      iVar10 = strcmp((char *)local_250,pcVar41);
      pcVar23 = (char *)local_250;
      if (iVar10 == 0) {
        pcVar23 = (char *)dcgettext(0,"standard input",5);
      }
      ppuVar16 = (undefined **)FUN_0010ae30(4,pcVar23);
      uVar24 = dcgettext(0,"cannot open %s for reading",5);
      error(0,*piVar22,uVar24,ppuVar16);
      local_209 = 0;
      goto LAB_00103340;
    }
  }
  bVar8 = (byte)ppuVar16;
  if (DAT_00113115 != '\0') {
    iVar11 = strcmp((char *)local_250,pcVar41);
    if (iVar11 == 0) {
      local_250 = (stat *)dcgettext(0,"standard input",5);
    }
    pcVar23 = "\n";
    if (DAT_00113010 != '\0') {
      pcVar23 = "";
    }
    __printf_chk(1,"%s==> %s <==\n",pcVar23,local_250);
    DAT_00113010 = '\0';
    local_250 = *(stat **)local_260;
  }
  iVar11 = strcmp((char *)local_250,pcVar41);
  if (iVar11 == 0) {
    local_250 = (stat *)dcgettext(0,"standard input",5);
  }
  bVar34 = DAT_00113119;
  local_1c8[0].fd = 0;
  local_1c8[0].events = 0;
  local_1c8[0].revents = 0;
  local_1f8 = (stat *)CONCAT71(local_1f8._1_7_,DAT_00113119);
  local_200 = &local_d8;
  if (DAT_00113119 != 0) {
    iVar11 = fstat((int)local_258,local_200);
    if (iVar11 != 0) goto LAB_00103590;
    local_210 = (uint)DAT_00113116;
    if (DAT_00113116 != 0) {
      uVar37 = FUN_00105ce0(local_250,local_258,local_1d8,local_1c8);
      if (uVar37 == 0) {
        lVar21 = FUN_00106020(0,local_250,local_258,0xffffffffffffffff);
        local_1c8[0] = (pollfd)((long)local_1c8[0] + lVar21);
      }
      else {
LAB_00103df1:
        local_210 = uVar37 >> 0x1f;
      }
      goto joined_r0x001035da;
    }
    if (((DAT_0011310d == '\0') && ((local_d8.st_mode & 0xf000) == 0x8000)) &&
       (ppuVar16 = (undefined **)lseek((int)local_258,0,1),
       ppuVar16 != (undefined **)0xffffffffffffffff)) {
      pVar18 = (pollfd)lseek((int)local_258,0,2);
      if ((long)ppuVar16 < (long)pVar18) {
        local_210 = (uint)bVar34;
        local_1c8[0] = pVar18;
        if (pVar18 != (pollfd)0x0) {
          bVar8 = FUN_00106190(local_250,local_258,local_1d8,ppuVar16);
          local_210 = (uint)bVar8;
        }
        goto joined_r0x001035da;
      }
      _Var25 = lseek((int)local_258,(__off_t)ppuVar16,0);
      if (_Var25 < 0) {
        FUN_00105f40(ppuVar16,0,local_250);
        goto LAB_00104273;
      }
    }
    local_248 = (stat *)FUN_0010bd20(0x2018);
    local_248[0x38].__unused[2] = 0;
    local_248[0x38].__unused[1] = 0;
    local_248[0x39].st_dev = 0;
    psVar15 = (stat *)FUN_0010bd20(0x2018);
    local_238.fd = 0;
    local_238.events = 0;
    local_238.revents = 0;
    psVar36 = psVar15;
    local_228 = local_248;
    while (pcVar39 = (char *)psVar36,
          ppuVar16 = (undefined **)FUN_0010b540(local_258,psVar15,0x2000),
          (char *)((long)ppuVar16 - 1U) < (char *)0xfffffffffffffffe) {
      ((stat *)((long)pcVar39 + 0x1f80))->__unused[1] = (long)ppuVar16;
      iVar11 = (int)DAT_00113114;
      local_1c8[0] = (pollfd)((long)local_1c8[0] + (long)ppuVar16);
      ((stat *)((long)pcVar39 + 0x1f80))->__unused[2] = 0;
      ((stat *)((long)pcVar39 + 0x2010))->st_dev = 0;
      psVar36 = psVar15;
      while (pvVar17 = memchr(psVar36,iVar11,
                              (size_t)((long)ppuVar16 +
                                      (long)psVar15->__unused + (-0x78 - (long)psVar36))),
            pvVar17 != (void *)0x0) {
        ((stat *)((long)pcVar39 + 0x1f80))->__unused[2] =
             ((stat *)((long)pcVar39 + 0x1f80))->__unused[2] + 1;
        psVar36 = (stat *)((long)pvVar17 + 1);
      }
      local_238 = (pollfd)((long)local_238 + ((stat *)((long)pcVar39 + 0x1f80))->__unused[2]);
      if ((char *)((long)ppuVar16 + local_248[0x38].__unused[1]) < (char *)0x2000) {
        memcpy((void *)((long)local_248->__unused + local_248[0x38].__unused[1] + -0x78),psVar15,
               (size_t)ppuVar16);
        local_248[0x38].__unused[1] =
             local_248[0x38].__unused[1] + ((stat *)((long)pcVar39 + 0x1f80))->__unused[1];
        local_248[0x38].__unused[2] =
             local_248[0x38].__unused[2] + ((stat *)((long)pcVar39 + 0x1f80))->__unused[2];
        psVar36 = (stat *)pcVar39;
      }
      else {
        local_248[0x39].st_dev = (__dev_t)pcVar39;
        pVar18 = (pollfd)((long)local_238 - local_228[0x38].__unused[2]);
        local_248 = (stat *)pcVar39;
        if ((ulong)local_1d8 < (ulong)pVar18) {
          psVar15 = local_228;
          psVar36 = local_228;
          local_238 = pVar18;
          local_228 = (stat *)local_228[0x39].st_dev;
        }
        else {
          psVar15 = (stat *)FUN_0010bd20(0x2018);
          psVar36 = psVar15;
        }
      }
    }
    free(pcVar39);
    if (ppuVar16 == (undefined **)0xffffffffffffffff) {
      pcVar39 = (char *)FUN_0010ae30(4,local_250);
      ppuVar16 = (undefined **)dcgettext(0,"error reading %s",5);
      piVar22 = __errno_location();
      error(0,*piVar22,ppuVar16,pcVar39);
LAB_001034b9:
      ppuVar40._0_4_ = 0;
      ppuVar40._4_2_ = 0;
      ppuVar40._6_2_ = 0;
      if (local_228 != (stat *)0x0) goto LAB_001034c8;
    }
    else {
      lVar21 = local_248[0x38].__unused[1];
      local_210 = (uint)(lVar21 == 0 || local_1d8 == (pollfd)0x0);
      if (lVar21 == 0 || local_1d8 == (pollfd)0x0) goto LAB_001034b9;
      iVar11 = (int)DAT_00113114;
      if (*(char *)((long)local_248->__unused + lVar21 + -0x79) != DAT_00113114) {
        local_248[0x38].__unused[2] = local_248[0x38].__unused[2] + 1;
        local_238 = (pollfd)((long)local_238 + 1);
      }
      pVar18 = (pollfd)((long)local_238 - local_228[0x38].__unused[2]);
      pcVar39 = (char *)local_228;
      while (pVar38 = pVar18, (ulong)local_1d8 < (ulong)pVar38) {
        pcVar39 = (char *)((stat *)((long)pcVar39 + 0x2010))->st_dev;
        local_238 = pVar38;
        pVar18 = (pollfd)((long)pVar38 - ((stat *)((long)pcVar39 + 0x1f80))->__unused[2]);
      }
      lVar21 = ((stat *)((long)pcVar39 + 0x1f80))->__unused[1];
      psVar15 = (stat *)pcVar39;
      if ((ulong)local_1d8 < (ulong)local_238) {
        lVar28 = (long)local_238 - (long)local_1d8;
        do {
          pvVar17 = rawmemchr(psVar15,iVar11);
          psVar15 = (stat *)((long)pvVar17 + 1);
          lVar28 = lVar28 + -1;
        } while (lVar28 != 0);
      }
      ppuVar16 = (undefined **)((long)pcVar39 + (lVar21 - (long)psVar15));
      if (ppuVar16 != (undefined **)0x0) {
        FUN_00105c60(psVar15,ppuVar16);
      }
      for (_Var5 = ((stat *)((long)pcVar39 + 0x2010))->st_dev; _Var5 != 0;
          _Var5 = *(__dev_t *)(_Var5 + 0x2010)) {
        if (*(long *)(_Var5 + 0x2000) != 0) {
          FUN_00105c60(_Var5);
        }
      }
      local_210 = (uint)bVar34;
LAB_001034c8:
      do {
        psVar15 = (stat *)local_228[0x39].st_dev;
        free(local_228);
        local_228 = psVar15;
      } while (psVar15 != (stat *)0x0);
      ppuVar40._0_4_ = 0;
      ppuVar40._4_2_ = 0;
      ppuVar40._6_2_ = 0;
    }
joined_r0x001035da:
    if (DAT_00113118 != 0) {
      *(uint *)((long)local_260 + 0x3c) = local_210 - 1;
      iVar11 = fstat((int)local_258,local_200);
      if (iVar11 < 0) {
        piVar22 = __errno_location();
        pcVar23 = *(char **)local_260;
        *(int *)((long)local_260 + 0x3c) = *piVar22;
        iVar10 = strcmp(pcVar23,pcVar41);
        if (iVar10 == 0) {
          pcVar23 = (char *)dcgettext(0,"standard input",5);
        }
        ppuVar16 = (undefined **)FUN_0010ae30(4,pcVar23);
        uVar24 = dcgettext(0,"error reading %s",5);
        error(0,*piVar22,uVar24,ppuVar16);
        goto LAB_00103546;
      }
      if ((((local_d8.st_mode & 0xf000) - 0x1000 & 0xffffe000) != 0) &&
         ((local_d8.st_mode & 0xb000) != 0x8000)) goto LAB_00103f51;
      if ((byte)local_210 == 0) goto LAB_00103546;
      *(int *)((long)local_260 + 0x38) = (int)local_258;
      pcVar23 = *(char **)local_260;
      *(__mode_t *)((long)local_260 + 0x30) = local_d8.st_mode;
      *(pollfd *)((long)local_260 + 8) = local_1c8[0];
      *(uint *)((long)local_260 + 0x40) = -(uint)(iVar10 == 0) | 1;
      *(__time_t *)((long)local_260 + 0x10) = local_d8.st_mtim.tv_sec;
      *(undefined **)((long)local_260 + 0x58) = (undefined *)0x0;
      *(long *)((long)local_260 + 0x18) = local_d8.st_mtim.tv_nsec;
      *(char *)((long)local_260 + 0x34) = '\0';
      *(undefined **)((long)local_260 + 0x20) =
           (undefined *)
           CONCAT26(local_d8.st_dev._6_2_,CONCAT24(local_d8.st_dev._4_2_,(int)local_d8.st_dev));
      *(__ino_t *)((long)local_260 + 0x28) = local_d8.st_ino;
      iVar10 = strcmp(pcVar23,pcVar41);
      if (iVar10 == 0) {
LAB_00104196:
        pcVar23 = (char *)dcgettext(0,"standard input",5);
      }
      cVar7 = FUN_00105460(local_258,pcVar23);
      *(char *)((long)local_260 + 0x35) = cVar7;
      goto LAB_00103340;
    }
    if ((iVar10 == 0) || (iVar10 = close((int)local_258), iVar10 == 0)) {
      local_209 = local_209 & (byte)local_210;
      goto LAB_00103340;
    }
    pcVar39 = *(char **)local_260;
    iVar10 = strcmp(pcVar39,pcVar41);
    if (iVar10 == 0) {
LAB_0010402f:
      pcVar39 = (char *)dcgettext(0,"standard input",5);
    }
    pcVar39 = (char *)FUN_0010ae30(4,pcVar39);
    ppuVar16 = (undefined **)dcgettext(0,"error reading %s",5);
    piVar22 = __errno_location();
    error(0,*piVar22,ppuVar16,pcVar39);
    local_209 = 0;
    goto LAB_00103340;
  }
  iVar11 = fstat((int)local_258,local_200);
  if (iVar11 != 0) {
LAB_00103590:
    pcVar39 = (char *)FUN_0010ae30(4,local_250);
    ppuVar16 = (undefined **)dcgettext(0,"cannot fstat %s",5);
    piVar22 = __errno_location();
    error(0,*piVar22,ppuVar16,pcVar39);
    local_210 = 0;
    goto joined_r0x001035da;
  }
  local_210 = (uint)DAT_00113116;
  if (DAT_00113116 == 0) {
    if ((DAT_0011310d != '\0') || ((long)local_1d8 < 0)) {
LAB_00103678:
      ppuVar40._0_4_ = 0;
      ppuVar40._4_2_ = 0;
      ppuVar40._6_2_ = 0;
      psVar15 = (stat *)FUN_0010bd20(0x2010);
      psVar15[0x38].__unused[1] = 0;
      psVar15[0x38].__unused[2] = 0;
      psVar36 = (stat *)FUN_0010bd20(0x2010);
      pcVar39 = (char *)psVar15;
      while (__ptr = psVar36, psVar36 = psVar15, sVar27 = FUN_0010b540(local_258,__ptr,0x2000),
            sVar27 - 1 < 0xfffffffffffffffe) {
        local_1c8[0] = (pollfd)((long)local_1c8[0] + sVar27);
        ppuVar40 = (undefined **)((long)ppuVar40 + sVar27);
        __ptr[0x38].__unused[1] = sVar27;
        lVar21 = ((stat *)((long)pcVar39 + 0x1f80))->__unused[1];
        __ptr[0x38].__unused[2] = 0;
        psVar15 = psVar36;
        if (sVar27 + lVar21 < 0x2000) {
          memcpy((void *)((long)((stat *)pcVar39)->__unused + lVar21 + -0x78),__ptr,sVar27);
          ((stat *)((long)pcVar39 + 0x1f80))->__unused[1] =
               ((stat *)((long)pcVar39 + 0x1f80))->__unused[1] + __ptr[0x38].__unused[1];
          psVar36 = __ptr;
        }
        else {
          ((stat *)((long)pcVar39 + 0x1f80))->__unused[2] = (long)__ptr;
          pVar18 = (pollfd)((long)ppuVar40 - psVar36[0x38].__unused[1]);
          pcVar39 = (char *)__ptr;
          if ((ulong)local_1d8 < (ulong)pVar18) {
            psVar15 = (stat *)psVar36[0x38].__unused[2];
            ppuVar40 = (undefined **)pVar18;
          }
          else {
            psVar36 = (stat *)FUN_0010bd20(0x2010);
          }
        }
      }
      free(__ptr);
      ppuVar16 = (undefined **)(sVar27 + 1);
      if (ppuVar16 == (undefined **)0x0) {
        pcVar39 = (char *)FUN_0010ae30(4,local_250);
        ppuVar16 = (undefined **)dcgettext(0,"error reading %s",5);
        piVar22 = __errno_location();
        error(0,*piVar22,ppuVar16,pcVar39);
        if (psVar36 == (stat *)0x0) goto joined_r0x001035da;
      }
      else {
        lVar21 = psVar36[0x38].__unused[1];
        pVar18 = (pollfd)((long)ppuVar40 - lVar21);
        psVar15 = psVar36;
        while (pVar38 = pVar18, (ulong)local_1d8 < (ulong)pVar38) {
          psVar15 = (stat *)psVar15[0x38].__unused[2];
          lVar21 = psVar15[0x38].__unused[1];
          ppuVar40 = (undefined **)pVar38;
          pVar18 = (pollfd)((long)pVar38 - lVar21);
        }
        lVar28 = 0;
        if ((ulong)local_1d8 < ppuVar40) {
          lVar28 = (long)ppuVar40 - (long)local_1d8;
        }
        if (lVar21 != lVar28) {
          FUN_00105c60((long)psVar15->__unused + lVar28 + -0x78);
        }
        while (psVar15 = (stat *)psVar15[0x38].__unused[2], psVar15 != (stat *)0x0) {
          if (psVar15[0x38].__unused[1] != 0) {
            FUN_00105c60(psVar15);
          }
        }
        local_210 = 1;
      }
      do {
        psVar15 = (stat *)psVar36[0x38].__unused[2];
        free(psVar36);
        psVar36 = psVar15;
      } while (psVar15 != (stat *)0x0);
      goto joined_r0x001035da;
    }
    if ((local_d8.st_mode & 0xd000) == 0x8000) {
      pVar18.fd = -1;
      pVar18.events = -1;
      pVar18.revents = -1;
      pcVar23 = (char *)local_d8.st_size;
    }
    else {
      pVar18 = (pollfd)lseek((int)local_258,-(long)local_1d8,2);
      if (pVar18 == (pollfd)0xffffffffffffffff) goto LAB_00103678;
      pcVar23 = (char *)((long)local_1d8 + (long)pVar18);
    }
    _Var19 = local_d8.st_blksize;
    if (0x1fffffffffffffff < local_d8.st_blksize - 1U) {
      _Var19 = 0x200;
    }
    if ((long)pcVar23 <= _Var19) goto LAB_00103678;
    if ((pVar18 == (pollfd)0xffffffffffffffff) &&
       (pVar18 = (pollfd)lseek((int)local_258,0,1), (long)pVar18 < 0)) {
      FUN_00105f40(0,1,local_250);
      goto LAB_00104196;
    }
    if (((long)pVar18 < (long)pcVar23) && ((ulong)local_1d8 < (ulong)((long)pcVar23 - (long)pVar18))
       ) {
      param_2 = (pollfd)((long)pcVar23 - (long)local_1d8);
      _Var25 = lseek((int)local_258,(__off_t)param_2,0);
      pVar18 = param_2;
      if (-1 < _Var25) goto LAB_00103a6a;
      pVar38 = param_2;
      FUN_00105f40(param_2,0,local_250);
      pVar20 = extraout_RDX_00;
      pVar18 = param_2;
LAB_00104240:
      iVar10 = 0;
      pVar31.fd = 10;
      pVar31.events = 0;
      pVar31.revents = 0;
      pVar32 = pVar20;
LAB_001041b9:
      pVar20 = (pollfd)((long)pVar32 + 1);
LAB_001038fb:
      bVar42 = *(char *)pVar20 == 'f';
      if (bVar42) {
        pVar20 = (pollfd)((long)pVar20 + 1);
      }
      psVar15 = (stat *)(ulong)bVar42;
      if (*(char *)pVar20 != '\0') goto LAB_00102a52;
      if ((pVar38 != pVar32) &&
         (uVar37 = FUN_0010c520(), pVar31 = local_1d8, (uVar37 & 0xfffffffd) != 0)) {
        uVar24 = FUN_0010b520(*(undefined **)((long)param_2 + 8));
        uVar29 = dcgettext(0,"invalid number",5);
        piVar22 = __errno_location();
        error(1,*piVar22,"%s: %s",uVar29,uVar24);
        psVar36 = local_200;
LAB_00104c07:
        local_200 = psVar36;
        pcVar39 = "inotify resources exhausted";
LAB_00104c13:
        uVar24 = dcgettext(0,pcVar39,5);
        error(0,0,uVar24);
LAB_00104c28:
        FUN_00108980(local_260);
        close(iVar10);
        piVar22 = __errno_location();
        *piVar22 = 0;
        pcVar39 = (char *)psVar15;
LAB_00104c45:
        uVar24 = dcgettext(0,"inotify cannot be used, reverting to polling",5);
        piVar22 = __errno_location();
        error(0,*piVar22,uVar24);
LAB_00103b98:
        DAT_0011310c = '\x01';
        if (DAT_00113110 == 0) {
          pVar38.fd = 0;
          pVar38.events = 0;
          pVar38.revents = 0;
          if (((DAT_00113020 == 2) && (local_230 == (char *)0x1)) &&
             (*(int *)((long)local_1f0 + 0x38) != -1)) {
            local_260.fd = *(uint *)((long)local_1f0 + 0x30);
            pVar38._1_7_ = 0;
            pVar38.fd._0_1_ = (*(uint *)((long)local_1f0 + 0x30) & 0xf000) != 0x8000;
          }
        }
        else {
          pVar38.fd = 0;
          pVar38.events = 0;
          pVar38.revents = 0;
        }
        local_210 = 0;
        local_250 = (stat *)(local_230 + -1);
        local_240 = (pollfd)((long)((long)local_1f0 + 0x34) + local_1e8);
        local_238 = pVar38;
LAB_00103bf5:
        local_258 = local_258 & 0xffffffffffffff00;
        pcVar23 = (char *)0x0;
        ppuVar40 = (undefined **)local_1f0;
        while( true ) {
          if (*(char *)((long)ppuVar40 + 0x34) != '\0') goto LAB_00103ca6;
          uVar37 = *(uint *)((long)ppuVar40 + 0x38);
          uVar26 = (ulong)uVar37;
          iVar10 = pVar38.fd;
          if (-1 < (int)uVar37) break;
          pcVar23 = pcVar23 + 1;
          ppuVar40 = (undefined **)((long)ppuVar40 + 0x60);
          FUN_00106660();
          if (local_230 == pcVar23) {
            do {
              if ((DAT_0011311a == 0) || (DAT_00113020 != 1)) {
                pVar18 = (pollfd)((long)local_1f0 + 0x34);
                do {
                  if ((-1 < *(int *)((long)pVar18 + 4)) ||
                     ((*(char *)pVar18 != '\x01' && (DAT_0011311a != 0)))) goto LAB_001045ae;
                  pVar18 = (pollfd)((long)pVar18 + 0x60);
                } while (pVar18 != local_240);
                uVar24 = dcgettext(0,"no files remaining",5);
                error(0,0,uVar24);
LAB_00103369:
                do {
                  do {
                    if ((DAT_0011310e == '\0') || (iVar10 = close(0), -1 < iVar10)) {
                      bVar8 = local_209 ^ 1;
                      goto LAB_00103381;
                    }
                    piVar22 = __errno_location();
                    error(1,*piVar22,pcVar41);
LAB_00103f51:
                    bVar8 = DAT_0011311a ^ 1;
                    pcVar23 = (char *)((long)local_260 + 0x3c);
                    pcVar23[0] = -1;
                    pcVar23[1] = -1;
                    pcVar23[2] = -1;
                    pcVar23[3] = -1;
                    *(char *)((long)local_260 + 0x36) = '\0';
                    *(byte *)((long)local_260 + 0x34) = bVar8;
                    if (bVar8 != 0) {
                      dcgettext(0,"; giving up on this name",5);
                    }
                    pcVar23 = *(char **)local_260;
                    iVar10 = strcmp(pcVar23,pcVar41);
                    if (iVar10 == 0) {
LAB_00104273:
                      pcVar23 = (char *)dcgettext(0,"standard input",5);
                    }
                    ppuVar16 = (undefined **)FUN_0010b1b0(0,3,pcVar23);
                    uVar24 = dcgettext(0,"%s: cannot follow end of this type of file%s",5);
                    error(0,0,uVar24,ppuVar16);
LAB_00103546:
                    pcVar23 = *(char **)local_260;
                    *(byte *)((long)local_260 + 0x34) = DAT_0011311a ^ 1;
                    iVar10 = strcmp(pcVar23,pcVar41);
                    if (iVar10 == 0) {
                      pcVar23 = (char *)dcgettext(0,"standard input",5);
                    }
                    FUN_00105bd0(local_258 & 0xffffffff,pcVar23);
                    local_209 = 0;
                    *(undefined4 *)((long)local_260 + 0x38) = 0xffffffff;
LAB_00103340:
                    bVar8 = DAT_00113118;
                    local_260 = (pollfd)((long)local_260 + 0x60);
                    if (local_218 != local_260) goto LAB_00102f20;
                    local_248 = (stat *)(ulong)DAT_00113118;
                  } while (DAT_00113118 == 0);
                  pcVar23 = (char *)0x0;
                  pVar38 = local_1f0;
                  do {
                    iVar10 = strcmp(*(char **)pVar38,pcVar41);
                    if ((((iVar10 == 0) && (*(char *)((long)pVar38 + 0x34) == '\0')) &&
                        (-1 < *(int *)((long)pVar38 + 0x38))) &&
                       ((*(uint *)((long)pVar38 + 0x30) & 0xf000) == 0x1000)) {
                      *(undefined4 *)((long)pVar38 + 0x38) = 0xffffffff;
                      *(char *)((long)pVar38 + 0x34) = '\x01';
                    }
                    else {
                      pcVar23 = pcVar23 + 1;
                    }
                    pVar38 = (pollfd)((long)pVar38 + 0x60);
                  } while (local_218 != pVar38);
                } while (pcVar23 == (char *)0x0);
                iVar10 = fstat(1,&local_168);
                if (-1 < iVar10) {
                  DAT_00113117 = (local_168.st_mode & 0xf000) == 0x1000;
                  pVar18 = local_1f0;
                  if (DAT_0011310c != '\0') goto LAB_00103b98;
                  goto LAB_00103b2b;
                }
                uVar26 = dcgettext(0,"standard output",5);
                piVar22 = __errno_location();
                error(1,*piVar22,uVar26);
LAB_00104af9:
                lVar21 = -2;
LAB_00103c8b:
                lVar21 = FUN_00106020(0,pcVar39,uVar26 & 0xffffffff,lVar21);
                *(undefined **)((long)ppuVar40 + 8) = *(undefined **)((long)ppuVar40 + 8) + lVar21;
                local_258 = CONCAT71(local_258._1_7_,(byte)local_258 | lVar21 != 0);
              }
              else {
LAB_001045ae:
                if (((byte)local_258 == '\x01') && ((char)local_238.fd == '\0')) {
                  if (DAT_00113117 == 0) goto LAB_00103bf5;
                  local_1c8[0].fd = 1;
                  local_1c8[0].events = 0;
                  local_1c8[0].revents = 0;
                  iVar10 = poll(local_1c8,1,0);
                  if (iVar10 < 0) goto LAB_00103bf5;
LAB_001045f9:
                  if (((ulong)local_1c8[0] & 0x18000000000000) != 0) goto LAB_00104b75;
                }
                else {
                  iVar10 = fflush_unlocked(stdout);
                  if (iVar10 != 0) goto LAB_00104b49;
                  if (DAT_00113117 != 0) {
                    local_1c8[0].fd = 1;
                    local_1c8[0].events = 0;
                    local_1c8[0].revents = 0;
                    iVar10 = poll(local_1c8,1,0);
                    if (-1 < iVar10) goto LAB_001045f9;
                  }
                }
                if ((byte)local_258 != '\0') goto LAB_00103bf5;
                if ((byte)local_210 != '\0') goto LAB_00103369;
                if (((DAT_00113110 != 0) && (iVar10 = kill(DAT_00113110,0), iVar10 != 0)) &&
                   (piVar22 = __errno_location(), *piVar22 != 1)) {
                  local_210 = (uint)(byte)local_248;
                  goto LAB_00103bf5;
                }
                iVar10 = FUN_0010c3a0(local_208);
                if (iVar10 == 0) goto LAB_00103bf5;
                uVar26 = dcgettext(0,"cannot read realtime clock",5);
                piVar22 = __errno_location();
                error(1,*piVar22,uVar26);
LAB_0010466a:
                *(undefined4 *)((long)ppuVar40 + 0x38) = 0xffffffff;
                local_260 = (pollfd)__errno_location();
                *(undefined4 *)((long)ppuVar40 + 0x3c) = *(undefined4 *)local_260;
                uVar24 = FUN_0010b1b0(0,3,pcVar39);
                error(0,*(undefined4 *)local_260,&DAT_0010e513,uVar24);
                close((int)uVar26);
              }
LAB_00103ca6:
              pcVar23 = pcVar23 + 1;
              ppuVar40 = (undefined **)((long)ppuVar40 + 0x60);
            } while (local_230 == pcVar23);
          }
        }
        pcVar39 = *ppuVar40;
        iVar11 = strcmp(pcVar39,pcVar41);
        if (iVar11 == 0) {
          pcVar39 = (char *)dcgettext(0,"standard input",5);
        }
        uVar3 = *(uint *)((long)ppuVar40 + 0x30);
        local_260.fd = uVar3;
        iVar11 = *(int *)((long)ppuVar40 + 0x40);
        if (iVar10 != *(int *)((long)ppuVar40 + 0x40)) {
          uVar12 = FUN_00107b60(uVar26,3);
          if (iVar10 == 0) {
            if ((-1 < (int)uVar12) &&
               ((uVar12 == (uVar12 | 0x800) || (iVar11 = FUN_00107b60(uVar26,4), iVar11 != -1)))) {
LAB_00103c76:
              *(int *)((long)ppuVar40 + 0x40) = iVar10;
              iVar11 = iVar10;
              goto LAB_00103c7c;
            }
          }
          else if (-1 < (int)uVar12) goto LAB_00103c76;
          piVar22 = __errno_location();
          if (((*(uint *)((long)ppuVar40 + 0x30) & 0xf000) != 0x8000) || (*piVar22 != 1)) {
            uVar24 = FUN_0010b1b0(0,3,pcVar39);
            uVar29 = dcgettext(0,"%s: cannot change nonblocking mode",5);
            error(1,*piVar22,uVar29,uVar24);
LAB_00104b49:
            uVar24 = dcgettext(0,"write error",5);
            piVar22 = __errno_location();
            error(1,*piVar22,uVar24);
            goto LAB_00104b75;
          }
          iVar11 = *(int *)((long)ppuVar40 + 0x40);
        }
LAB_00103c7c:
        lVar21 = -2;
        if (iVar11 != 0) goto LAB_00103c8b;
        iVar10 = fstat(uVar37,&local_d8);
        if (iVar10 != 0) goto LAB_0010466a;
        if ((*(uint *)((long)ppuVar40 + 0x30) == local_d8.st_mode) &&
           ((((local_d8.st_mode & 0xf000) != 0x8000 ||
             (*(undefined **)((long)ppuVar40 + 8) == (undefined *)local_d8.st_size)) &&
            (((uint)(local_d8.st_mtim.tv_nsec < (long)*(undefined **)((long)ppuVar40 + 0x18)) -
             (uint)((long)*(undefined **)((long)ppuVar40 + 0x18) < local_d8.st_mtim.tv_nsec)) +
             ((uint)(local_d8.st_mtim.tv_sec < (long)*(undefined **)((long)ppuVar40 + 0x10)) -
             (uint)((long)*(undefined **)((long)ppuVar40 + 0x10) < local_d8.st_mtim.tv_sec)) * 2 ==
             0)))) {
          bVar42 = *(undefined **)((long)ppuVar40 + 0x58) < DAT_00113018;
          *(undefined **)((long)ppuVar40 + 0x58) = *(undefined **)((long)ppuVar40 + 0x58) + 1;
          if (bVar42) goto LAB_00103ca6;
          if (DAT_00113020 != 1) goto LAB_00103ca6;
          FUN_00106660();
          *(undefined **)((long)ppuVar40 + 0x58) = (undefined *)0x0;
          goto LAB_00103ca6;
        }
        *(__mode_t *)((long)ppuVar40 + 0x30) = local_d8.st_mode;
        *(undefined **)((long)ppuVar40 + 0x58) = (undefined *)0x0;
        *(__time_t *)((long)ppuVar40 + 0x10) = local_d8.st_mtim.tv_sec;
        *(long *)((long)ppuVar40 + 0x18) = local_d8.st_mtim.tv_nsec;
        if (((uVar3 & 0xf000) == 0x8000) &&
           (local_d8.st_size < (long)*(undefined **)((long)ppuVar40 + 8))) {
          local_260 = (pollfd)FUN_0010b1b0(0,3,pcVar39);
          uVar24 = dcgettext(0,"%s: file truncated",5);
          error(0,0,uVar24,local_260);
          _Var25 = lseek(uVar37,0,0);
          if (_Var25 < 0) {
            FUN_00105f40(0,0,pcVar39);
            goto LAB_0010479d;
          }
          *(undefined **)((long)ppuVar40 + 8) = (undefined *)0x0;
        }
        if (((stat *)pcVar23 != local_250) && (DAT_00113115 != '\0')) {
          local_260 = (pollfd)(CONCAT44(local_260._4_4_,uVar3) & 0xffffffff0000f000);
          pcVar33 = "\n";
          if (DAT_00113010 != '\0') {
            pcVar33 = "";
          }
          __printf_chk(1,"%s==> %s <==\n",pcVar33,pcVar39);
          DAT_00113010 = '\0';
        }
        local_250 = (stat *)pcVar23;
        if (*(int *)((long)ppuVar40 + 0x40) != 0) goto LAB_00104af9;
        lVar21 = -1;
        if ((uVar3 & 0xf000) != 0x8000) goto LAB_00103c8b;
        if (*(char *)((long)ppuVar40 + 0x35) == '\0') goto LAB_00103c8b;
        lVar21 = local_d8.st_size - (long)*(undefined **)((long)ppuVar40 + 8);
        goto LAB_00103c8b;
      }
      local_1d8 = pVar31;
      lVar21 = 1;
      DAT_00113119 = (byte)iVar10;
      DAT_00113116 = bVar8;
      DAT_00113118 = bVar42;
      goto LAB_00102a54;
    }
  }
  else {
    if ((DAT_0011310d == '\0') && (-1 < (long)local_1d8)) {
      if ((local_d8.st_mode & 0xf000) == 0x8000) {
        _Var25 = lseek((int)local_258,(__off_t)local_1d8,1);
        if (_Var25 < 0) {
          FUN_00105f40(local_1d8,1,local_250);
          goto LAB_0010402f;
        }
      }
      else {
        _Var25 = lseek((int)local_258,(__off_t)local_1d8,1);
        if (_Var25 == -1) goto LAB_00103a3e;
      }
      local_240.fd = -1;
      local_240.events = -1;
      local_240.revents = -1;
      local_1c8[0] = local_1d8;
      pVar18 = local_1c8[0];
      goto LAB_00103a6a;
    }
LAB_00103a3e:
    uVar37 = FUN_00105e30(local_250,local_258,local_1d8,local_1c8);
    local_240.fd = -1;
    local_240.events = -1;
    local_240.revents = -1;
    pVar18 = local_1c8[0];
    if (uVar37 != 0) goto LAB_00103df1;
  }
LAB_00103a6a:
  local_1c8[0] = pVar18;
  lVar21 = FUN_00106020(0,local_250,local_258,local_240);
  local_210 = 1;
  local_1c8[0] = (pollfd)((long)local_1c8[0] + lVar21);
  goto joined_r0x001035da;
switchD_00102ad3_caseD_85:
  DAT_0011310c = '\x01';
  goto LAB_00102a90;
  while (pVar18 = (pollfd)((long)pVar18 + 0x60), local_218 != pVar18) {
LAB_00103b2b:
    if ((*(char *)((long)pVar18 + 0x34) == '\0') &&
       (iVar10 = strcmp(*(char **)pVar18,pcVar41), iVar10 == 0)) goto LAB_00103b98;
  }
  lVar21 = 0;
  do {
    if ((-1 < *(int *)((long)((long)local_1f0 + 0x38) + lVar21)) &&
       (((char *)((long)local_1f0 + 0x35))[lVar21] != '\0')) goto LAB_00103b98;
    lVar21 = lVar21 + 0x60;
  } while (local_1e8 != lVar21);
  lVar21 = 0;
  do {
    if ((-1 < *(int *)((long)((long)local_1f0 + 0x38) + lVar21)) &&
       (((char *)((long)local_1f0 + 0x35))[lVar21] == '\0')) {
      bVar34 = 0;
      pcVar39 = (char *)&local_d8;
      pVar18 = local_1f0;
      goto LAB_0010484a;
    }
    lVar21 = lVar21 + 0x60;
  } while (local_1e8 != lVar21);
  goto LAB_00103b98;
  while (pVar18 = (pollfd)((long)pVar18 + 0x60), local_218 != pVar18) {
LAB_0010484a:
    iVar10 = lstat(*(char **)pVar18,(stat *)pcVar39);
    if ((iVar10 == 0) && ((local_d8.st_mode & 0xf000) == 0xa000)) goto LAB_00103b98;
  }
  lVar21 = 0;
  do {
    if (((-1 < *(int *)((long)((long)local_1f0 + 0x38) + lVar21)) &&
        (uVar37 = *(uint *)((long)((long)local_1f0 + 0x30) + lVar21) & 0xf000, uVar37 != 0x8000)) &&
       (uVar37 != 0x1000)) goto LAB_00103b98;
    lVar21 = lVar21 + 0x60;
  } while (local_1e8 != lVar21);
  if ((local_209 != 0) || (DAT_00113020 != 2)) goto LAB_001048c9;
  goto LAB_00103b98;
LAB_001048c9:
  iVar10 = inotify_init();
  if (-1 < iVar10) goto code_r0x001048d9;
  goto LAB_00104c45;
code_r0x001048d9:
  iVar11 = fflush_unlocked(stdout);
  if (iVar11 != 0) {
    pVar18 = (pollfd)dcgettext(0,"write error",5);
    piVar22 = __errno_location();
    error(1,*piVar22,pVar18);
LAB_001052ea:
    uVar24 = FUN_00105fe0(*(undefined **)pVar18);
    psVar15 = (stat *)FUN_0010ae30(4,uVar24);
    uVar24 = dcgettext(0,"%s was replaced",5);
    piVar22 = __errno_location();
    error(0,*piVar22,uVar24,psVar15);
    goto LAB_00104c28;
  }
  local_260 = (pollfd)FUN_001087c0(local_230,0,FUN_00105430,FUN_00105450,0);
  if (local_260 == (pollfd)0x0) {
LAB_00105144:
                    /* WARNING: Subroutine does not return */
    FUN_0010c220();
  }
  __mask = 0xc06;
  if (DAT_00113020 != 1) {
    __mask = 2;
  }
  bVar9 = 0;
  bVar6 = 0;
  local_258 = 0;
  pVar18 = local_1f0;
  do {
    iVar11 = DAT_00113020;
    if (*(char *)((long)pVar18 + 0x34) == '\0') {
      psVar15 = *(stat **)pVar18;
      sVar27 = strlen((char *)psVar15);
      pcVar23 = (char *)((long)pVar18 + 0x44);
      pcVar23[0] = -1;
      pcVar23[1] = -1;
      pcVar23[2] = -1;
      pcVar23[3] = -1;
      if (sVar27 <= local_258) {
        sVar27 = local_258;
      }
      psVar36 = local_200;
      local_258 = sVar27;
      if (iVar11 == 1) {
        lVar21 = FUN_00107a60(psVar15);
        puVar1 = (undefined1 *)((long)psVar15->__unused + lVar21 + -0x78);
        uVar2 = *puVar1;
        lVar28 = FUN_00107840(psVar15);
        *(undefined **)((long)pVar18 + 0x50) = (undefined *)(lVar28 - (long)psVar15);
        *puVar1 = 0;
        pcVar23 = ".";
        if (lVar21 != 0) {
          pcVar23 = *(char **)pVar18;
        }
        iVar11 = inotify_add_watch(iVar10,pcVar23,0x784);
        *(int *)((long)pVar18 + 0x48) = iVar11;
        (*(undefined **)pVar18)[lVar21] = uVar2;
        if (*(int *)((long)pVar18 + 0x48) < 0) {
          piVar22 = __errno_location();
          if (*piVar22 == 0x1c) goto LAB_00104c07;
          uVar24 = FUN_0010ae30(4,*(undefined **)pVar18);
          uVar29 = dcgettext(0,"cannot watch parent directory of %s",5);
          error(0,*piVar22,uVar29,uVar24);
          goto LAB_00104c28;
        }
      }
      iVar11 = inotify_add_watch(iVar10,*(char **)pVar18,__mask);
      *(int *)((long)pVar18 + 0x44) = iVar11;
      if (iVar11 < 0) {
        if (*(int *)((long)pVar18 + 0x38) != -1) {
          bVar9 = bVar8;
        }
        puVar35 = (uint *)__errno_location();
        if ((*puVar35 & 0xffffffef) == 0xc) goto LAB_00104c07;
        if (*puVar35 != *(uint *)((long)pVar18 + 0x3c)) {
          FUN_0010ae30(4,*(undefined **)pVar18);
          uVar24 = dcgettext(0,"cannot watch %s",5);
          error(0,*puVar35,uVar24);
        }
      }
      else {
        lVar21 = FUN_00108e20(local_260);
        bVar6 = bVar8;
        if (lVar21 == 0) goto LAB_00105144;
      }
    }
    pVar18 = (pollfd)((long)pVar18 + 0x60);
  } while (local_218 != pVar18);
  pVar18 = local_1f0;
  psVar15 = (stat *)pcVar39;
  if (DAT_00113020 == 2) {
    if (bVar9 != 0) goto LAB_00104c28;
    if (bVar6 == 0) goto LAB_00104b7f;
  }
  do {
    if (*(char *)((long)pVar18 + 0x34) == '\0') {
      if (DAT_00113020 == 1) {
        FUN_00106660(pVar18,0);
      }
      else if (((*(int *)((long)pVar18 + 0x38) != -1) &&
               (iVar11 = stat(*(char **)pVar18,(stat *)pcVar39), iVar11 == 0)) &&
              ((*(undefined **)((long)pVar18 + 0x20) !=
                (undefined *)
                CONCAT26(local_d8.st_dev._6_2_,CONCAT24(local_d8.st_dev._4_2_,(int)local_d8.st_dev))
               || (*(undefined **)((long)pVar18 + 0x28) != (undefined *)local_d8.st_ino))))
      goto LAB_001052ea;
      FUN_00106430(pVar18);
    }
    pVar18 = (pollfd)((long)pVar18 + 0x60);
    if (local_218 == pVar18) {
      local_240 = (pollfd)(local_258 + 0x11);
      local_258 = FUN_0010bd20(local_240);
      iVar11 = 3;
      local_250 = (stat *)0x0;
      psVar36 = (stat *)0x0;
      break;
    }
  } while( true );
LAB_00104d4b:
  if (((DAT_00113020 == 1) && (DAT_0011311a == 0)) &&
     (lVar21 = FUN_001082f0(local_260), lVar21 == 0)) {
    uVar24 = dcgettext(0,"no files remaining",5);
    error(1,0,uVar24);
LAB_00104ee1:
    FUN_00106430();
    goto LAB_00104d4b;
  }
  if (psVar36 < local_250) {
LAB_00104d63:
    piVar22 = (int *)((long)psVar36->__unused + (local_258 - 0x78));
    uVar37 = piVar22[1];
    uVar3 = piVar22[3];
    iVar14 = *piVar22;
    psVar36 = (stat *)((long)psVar36->__unused + ((ulong)uVar3 - 0x68));
    if ((uVar37 & 0x400) == 0) {
      if (uVar3 == 0) goto LAB_00104e77;
    }
    else if (uVar3 == 0) {
      pcVar39 = (char *)0x0;
      do {
        if (iVar14 == *(int *)((undefined **)((long)local_1f0 + 0x48) + (long)pcVar39 * 0xc)) {
          pcVar39 = "directory containing watched file was removed";
          goto LAB_00104c13;
        }
        pcVar39 = pcVar39 + 1;
      } while (local_230 != pcVar39);
LAB_00104e77:
      local_184 = iVar14;
      ppuVar16 = (undefined **)FUN_00108530(local_260);
      if (ppuVar16 != (undefined **)0x0) {
LAB_00104e47:
        if ((piVar22[1] & 0xe04U) == 0) goto LAB_00104ee1;
        if ((piVar22[1] & 0x400U) != 0) {
          inotify_rm_watch(iVar10,*(int *)((long)ppuVar16 + 0x44));
          FUN_00108e80(local_260);
        }
        FUN_00106660();
      }
      goto LAB_00104d4b;
    }
    pcVar39 = (char *)0x0;
    local_218.fd = iVar10;
    pVar18 = local_1f0;
    do {
      local_200 = psVar36;
      local_1f8 = psVar15;
      if ((iVar14 == *(int *)((long)pVar18 + 0x48)) &&
         (iVar13 = strcmp((char *)(piVar22 + 4),
                          *(undefined **)((long)pVar18 + 0x50) + (long)*(undefined **)pVar18),
         iVar13 == 0)) {
        if (local_230 == pcVar39) goto LAB_00104d4b;
        ppuVar16 = (undefined **)((long)local_1f0 + (long)pcVar39 * 0x60);
        if ((uVar37 & 0x200) == 0) {
          iVar14 = inotify_add_watch(iVar10,*ppuVar16,__mask);
          if (iVar14 < 0) {
            local_200 = (stat *)__errno_location();
            if ((local_200->st_dev & 0xffffffef) == 0xc) goto LAB_00104c07;
            local_218 = (pollfd)FUN_0010ae30(4,*ppuVar16);
            uVar24 = dcgettext(0,"cannot watch %s",5);
            error(0,(int)local_200->st_dev,uVar24);
          }
          iVar13 = *(int *)((long)ppuVar16 + 0x44);
          if (iVar14 == iVar13) {
            if (-1 < iVar13) goto LAB_00104e3a;
          }
          else if (-1 < iVar13) {
            inotify_rm_watch(iVar10,iVar13);
            FUN_00108e80();
          }
          *(int *)((long)ppuVar16 + 0x44) = iVar14;
          if (iVar14 == -1) goto LAB_00104d4b;
          ppuVar30 = (undefined **)FUN_00108e80(local_260,ppuVar16);
          if ((ppuVar30 != (undefined **)0x0) && (ppuVar16 != ppuVar30)) {
            if (DAT_00113020 == 1) {
              FUN_00106660(ppuVar30,0);
            }
            pcVar39 = (char *)((long)ppuVar30 + 0x44);
            pcVar39[0] = -1;
            pcVar39[1] = -1;
            pcVar39[2] = -1;
            pcVar39[3] = -1;
            uVar24 = FUN_00105fe0(*ppuVar30);
            FUN_00105bd0(*(undefined4 *)(ppuVar30 + 7),uVar24);
          }
          lVar21 = FUN_00108e20(local_260);
          if (lVar21 == 0) goto LAB_00105144;
        }
LAB_00104e3a:
        if (DAT_00113020 == 1) {
          FUN_00106660(ppuVar16);
        }
        goto LAB_00104e47;
      }
      pcVar39 = pcVar39 + 1;
      pVar18 = (pollfd)((long)pVar18 + 0x60);
    } while (local_230 != pcVar39);
LAB_00105038:
    psVar36 = local_200;
    psVar15 = local_1f8;
    iVar10 = local_218.fd;
    goto LAB_00104d4b;
  }
  do {
    if (DAT_00113110 == 0) {
LAB_00104fb1:
      iVar14 = -1;
    }
    else {
      if (bVar34 != 0) goto LAB_001052b7;
      iVar14 = kill(DAT_00113110,0);
      if ((iVar14 == 0) || (piVar22 = __errno_location(), *piVar22 == 1)) {
        iVar14 = 0;
        if (0.0 < (double)local_208) {
          if (DAT_0010e4a8 <= (double)local_208) goto LAB_00104fb1;
          iVar14 = (int)(DAT_0010e4b0 * (double)local_208);
          iVar14 = iVar14 + (uint)((double)iVar14 < DAT_0010e4b0 * (double)local_208);
        }
      }
      else {
        iVar14 = 0;
        bVar34 = bVar8;
      }
    }
    local_d8.st_dev._4_2_ = 1;
    local_d8.st_ino = 1;
    local_d8.st_dev._0_4_ = iVar10;
    iVar14 = poll((pollfd *)psVar15,(ulong)DAT_00113117 + 1,iVar14);
  } while (iVar14 == 0);
  if (iVar14 < 0) {
    uVar24 = dcgettext(0,"error waiting for inotify and output events",5);
    piVar22 = __errno_location();
    error(1,*piVar22,uVar24);
LAB_001052b7:
                    /* WARNING: Subroutine does not return */
    exit(0);
  }
  if (local_d8.st_ino._6_2_ != 0) {
LAB_00104b75:
    raise(0xd);
LAB_00104b7f:
                    /* WARNING: Subroutine does not return */
    exit(1);
  }
  local_250 = (stat *)FUN_0010b540(iVar10,local_258,local_240);
  if (local_250 != (stat *)0x0) {
    psVar36 = (stat *)0x0;
    if (local_250 == (stat *)0xffffffffffffffff) {
      piVar22 = __errno_location();
      if (*piVar22 == 0x16) goto LAB_00105149;
      goto LAB_0010500c;
    }
    goto LAB_00104d63;
  }
LAB_00105149:
  if (iVar11 == 0) {
LAB_0010500c:
    uVar24 = dcgettext(0,"error reading inotify event",5);
    piVar22 = __errno_location();
    error(1,*piVar22,uVar24);
    goto LAB_00105038;
  }
  psVar36 = (stat *)0x0;
  iVar11 = iVar11 + -1;
  local_240 = (pollfd)((long)local_240 * 2);
  local_258 = FUN_0010bd70(local_258);
  local_250 = (stat *)0x0;
  goto LAB_00104d4b;
}




// Function: wd_hasher @ 0x5430

ulong wd_hasher(long param_1,ulong param_2)

{
  return (ulong)(long)*(int *)(param_1 + 0x44) % param_2;
}




// Function: wd_comparator @ 0x5450

undefined4 wd_comparator(long param_1,long param_2)

{
  return CONCAT31((int3)((uint)*(int *)(param_2 + 0x44) >> 8),
                  *(int *)(param_1 + 0x44) == *(int *)(param_2 + 0x44));
}




// Function: fremote @ 0x5460

ulong fremote(int param_1,undefined8 param_2)

{
  int iVar1;
  int *piVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined8 unaff_R12;
  ulong uVar5;
  undefined7 uVar6;
  long in_FS_OFFSET;
  statfs local_a8;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = fstatfs(param_1,&local_a8);
  if (iVar1 != 0) {
    piVar2 = __errno_location();
    uVar5 = 1;
    if (*piVar2 != 0x26) {
      uVar3 = FUN_0010ae30(4,param_2);
      uVar4 = dcgettext(0,"cannot determine location of %s. reverting to polling",5);
      error(0,*piVar2,uVar4,uVar3);
    }
    goto LAB_001054a8;
  }
  if (local_a8.f_type != 0x2bad1dea) {
    uVar6 = (undefined7)((ulong)unaff_R12 >> 8);
    if (local_a8.f_type < 0x2bad1deb) {
      if (local_a8.f_type < 0x9fa3) {
        if (local_a8.f_type < 0x9fa0) {
          if (local_a8.f_type < 0x4007) {
            if (0x3fff < local_a8.f_type) {
              uVar5 = (ulong)((uint)(0x51L >> ((byte)local_a8.f_type & 0x3f)) & 1 ^ 1);
              goto LAB_001054a8;
            }
            if (local_a8.f_type < 0x1390) {
              if (0x1372 < local_a8.f_type) {
                uVar5 = (ulong)((uint)(0x10001401L >> ((byte)local_a8.f_type + 0x8d & 0x3f)) & 1 ^ 1
                               );
                goto LAB_001054a8;
              }
              if (local_a8.f_type != 0x187) {
                if (local_a8.f_type < 0x188) {
                  uVar5 = CONCAT71(uVar6,(local_a8.f_type - 0x2fU & 0xfffffffffffffffb) != 0);
                }
                else {
                  uVar5 = CONCAT71(uVar6,local_a8.f_type != 0x7c0);
                }
                goto LAB_001054a8;
              }
            }
            else if (local_a8.f_type != 0x2478) {
              if (0x2478 < local_a8.f_type) {
                uVar5 = CONCAT71(uVar6,local_a8.f_type != 0x3434);
                goto LAB_001054a8;
              }
              if (local_a8.f_type != 0x1cd1) {
                uVar5 = CONCAT71(uVar6,local_a8.f_type != 0x2468);
                goto LAB_001054a8;
              }
            }
          }
          else if (local_a8.f_type != 0x4d5a) {
            if (local_a8.f_type < 0x4d5b) {
              if (local_a8.f_type != 0x4858) {
                if (0x4858 < local_a8.f_type) {
                  uVar5 = CONCAT71(uVar6,local_a8.f_type != 0x4d44);
                  goto LAB_001054a8;
                }
                if (local_a8.f_type != 0x4244) {
                  uVar5 = CONCAT71(uVar6,local_a8.f_type != 0x482b);
                  goto LAB_001054a8;
                }
              }
            }
            else if (local_a8.f_type != 0x72b6) {
              if (0x72b6 < local_a8.f_type) {
                uVar5 = CONCAT71(uVar6,local_a8.f_type != 0x9660);
                goto LAB_001054a8;
              }
              if (local_a8.f_type != 0x5df5) {
                uVar5 = CONCAT71(uVar6,local_a8.f_type != 0x7275);
                goto LAB_001054a8;
              }
            }
          }
        }
      }
      else if (local_a8.f_type < 0x12ff7b8) {
        if ((local_a8.f_type < 0x12ff7b4) && (local_a8.f_type != 0x27e0eb)) {
          if (local_a8.f_type < 0x27e0ec) {
            if (local_a8.f_type != 0xef53) {
              if (local_a8.f_type < 0xef54) {
                if ((local_a8.f_type != 0xadff) && (local_a8.f_type != 0xef51)) {
                  uVar5 = CONCAT71(uVar6,local_a8.f_type != 0xadf5);
                  goto LAB_001054a8;
                }
              }
              else if (local_a8.f_type != 0xf15f) {
                uVar5 = CONCAT71(uVar6,local_a8.f_type != 0x11954);
                goto LAB_001054a8;
              }
            }
          }
          else if (local_a8.f_type != 0x1021994) {
            if (local_a8.f_type < 0x1021995) {
              if (local_a8.f_type != 0x414a53) {
                uVar5 = CONCAT71(uVar6,local_a8.f_type != 0xc0ffee);
                goto LAB_001054a8;
              }
            }
            else if (local_a8.f_type != 0x1021997) {
              uVar5 = CONCAT71(uVar6,local_a8.f_type != 0x12fd16d);
              goto LAB_001054a8;
            }
          }
        }
      }
      else if (local_a8.f_type != 0x15013346) {
        if (local_a8.f_type < 0x15013347) {
          if (local_a8.f_type != 0xbad1dea) {
            if (local_a8.f_type < 0xbad1deb) {
              if (local_a8.f_type != 0x7655821) {
                uVar5 = CONCAT71(uVar6,local_a8.f_type != 0x9041934);
                goto LAB_001054a8;
              }
            }
            else if (local_a8.f_type != 0x11307854) {
              uVar5 = CONCAT71(uVar6,local_a8.f_type != 0x13661366);
              goto LAB_001054a8;
            }
          }
        }
        else if (local_a8.f_type != 0x2011bab0) {
          if (local_a8.f_type < 0x2011bab1) {
            if (local_a8.f_type != 0x19800202) {
              uVar5 = CONCAT71(uVar6,local_a8.f_type != 0x1badface);
              goto LAB_001054a8;
            }
          }
          else if (local_a8.f_type != 0x24051905) {
            uVar5 = CONCAT71(uVar6,local_a8.f_type != 0x28cd3d45);
            goto LAB_001054a8;
          }
        }
      }
    }
    else if (local_a8.f_type != 0x62656570) {
      if (local_a8.f_type < 0x62656571) {
        if (local_a8.f_type != 0x53464846) {
          if (local_a8.f_type < 0x53464847) {
            if (local_a8.f_type != 0x453dcd28) {
              if (local_a8.f_type < 0x453dcd29) {
                if (local_a8.f_type != 0x42494e4d) {
                  if (local_a8.f_type < 0x42494e4e) {
                    if ((local_a8.f_type != 0x3153464a) && (local_a8.f_type != 0x42465331)) {
                      uVar5 = CONCAT71(uVar6,local_a8.f_type != 0x2fc12fc1);
                      goto LAB_001054a8;
                    }
                  }
                  else if (local_a8.f_type != 0x43415d53) {
                    uVar5 = CONCAT71(uVar6,local_a8.f_type != 0x444d4142);
                    goto LAB_001054a8;
                  }
                }
              }
              else if (local_a8.f_type != 0x52654973) {
                if (local_a8.f_type < 0x52654974) {
                  if (local_a8.f_type != 0x454d444d) {
                    uVar5 = CONCAT71(uVar6,local_a8.f_type != 0x45584653);
                    goto LAB_001054a8;
                  }
                }
                else if (local_a8.f_type != 0x5345434d) {
                  uVar5 = CONCAT71(uVar6,local_a8.f_type != 0x5346314d);
                  goto LAB_001054a8;
                }
              }
            }
          }
          else if (local_a8.f_type != 0x58465342) {
            if (local_a8.f_type < 0x58465343) {
              if (local_a8.f_type != 0x54190100) {
                if (local_a8.f_type < 0x54190101) {
                  if (local_a8.f_type != 0x5346544e) {
                    uVar5 = CONCAT71(uVar6,local_a8.f_type != 0x534f434b);
                    goto LAB_001054a8;
                  }
                }
                else if (local_a8.f_type != 0x565a4653) {
                  uVar5 = CONCAT71(uVar6,local_a8.f_type != 0x58295829);
                  goto LAB_001054a8;
                }
              }
            }
            else if (local_a8.f_type != 0x5dca2df5) {
              if (local_a8.f_type < 0x5dca2df6) {
                if (local_a8.f_type != 0x5a3c69f0) {
                  uVar5 = CONCAT71(uVar6,local_a8.f_type != 0x5a4f4653);
                  goto LAB_001054a8;
                }
              }
              else if (local_a8.f_type != 0x6165676c) {
                uVar5 = CONCAT71(uVar6,local_a8.f_type != 0x62646576);
                goto LAB_001054a8;
              }
            }
          }
        }
      }
      else if (local_a8.f_type != 0x858458f6) {
        if (local_a8.f_type < 0x858458f7) {
          if (local_a8.f_type != 0x6c6f6f70) {
            if (local_a8.f_type < 0x6c6f6f71) {
              if (local_a8.f_type != 0x64646178) {
                if (local_a8.f_type < 0x64646179) {
                  if ((local_a8.f_type != 0x63677270) && (local_a8.f_type != 0x64626720)) {
                    uVar5 = CONCAT71(uVar6,local_a8.f_type != 0x62656572);
                    goto LAB_001054a8;
                  }
                }
                else if (local_a8.f_type != 0x67596969) {
                  uVar5 = CONCAT71(uVar6,local_a8.f_type != 0x68191122);
                  goto LAB_001054a8;
                }
              }
            }
            else if (local_a8.f_type != 0x73717368) {
              if (local_a8.f_type < 0x73717369) {
                if (local_a8.f_type != 0x6e736673) {
                  uVar5 = CONCAT71(uVar6,local_a8.f_type != 0x73636673);
                  goto LAB_001054a8;
                }
              }
              else if (local_a8.f_type != 0x73727279) {
                uVar5 = CONCAT71(uVar6,local_a8.f_type != 0x74726163);
                goto LAB_001054a8;
              }
            }
          }
        }
        else if (local_a8.f_type != 0xcafe4a11) {
          if (local_a8.f_type < 0xcafe4a12) {
            if (local_a8.f_type != 0xabba1974) {
              if (local_a8.f_type < 0xabba1975) {
                if (local_a8.f_type != 0x9123683e) {
                  uVar5 = CONCAT71(uVar6,local_a8.f_type != 0x958458f6);
                  goto LAB_001054a8;
                }
              }
              else if (local_a8.f_type != 0xc7571590) {
                uVar5 = CONCAT71(uVar6,local_a8.f_type != 0xc97e8168);
                goto LAB_001054a8;
              }
            }
          }
          else if (local_a8.f_type != 0xf2f52010) {
            if (local_a8.f_type < 0xf2f52011) {
              if (local_a8.f_type != 0xde5e81e4) {
                uVar5 = CONCAT71(uVar6,local_a8.f_type != 0xe0f5e1e2);
                goto LAB_001054a8;
              }
            }
            else if (local_a8.f_type != 0xf97cff8c) {
              uVar5 = CONCAT71(uVar6,local_a8.f_type != 0xf995e849);
              goto LAB_001054a8;
            }
          }
        }
      }
    }
  }
  uVar5 = 0;
LAB_001054a8:
  if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar5 & 0xffffffff;
}




// Function: start_lines @ 0x5ce0

undefined8 start_lines(undefined8 param_1,undefined4 param_2,long param_3,long *param_4)

{
  undefined1 *puVar1;
  long lVar2;
  void *pvVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  int *piVar6;
  undefined1 *__s;
  int __c;
  long in_FS_OFFSET;
  undefined1 local_2048 [8200];
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  uVar4 = 0;
  if (param_3 != 0) {
    while( true ) {
      lVar2 = FUN_0010b540(param_2,local_2048,0x2000);
      if (lVar2 == 0) break;
      if (lVar2 == -1) {
        uVar4 = FUN_0010ae30(4,param_1);
        uVar5 = dcgettext(0,"error reading %s",5);
        piVar6 = __errno_location();
        error(0,*piVar6,uVar5,uVar4);
        uVar4 = 1;
        goto LAB_00105d94;
      }
      *param_4 = *param_4 + lVar2;
      __c = (int)DAT_00113114;
      puVar1 = local_2048 + lVar2;
      __s = local_2048;
      while( true ) {
        pvVar3 = memchr(__s,__c,(long)puVar1 - (long)__s);
        if (pvVar3 == (void *)0x0) break;
        __s = (undefined1 *)((long)pvVar3 + 1);
        param_3 = param_3 + -1;
        if (param_3 == 0) {
          uVar4 = 0;
          if ((__s < puVar1) && ((long)puVar1 - (long)__s != 0)) {
            FUN_00105c60(__s,(long)puVar1 - (long)__s);
            uVar4 = 0;
          }
          goto LAB_00105d94;
        }
      }
    }
    uVar4 = 0xffffffff;
  }
LAB_00105d94:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar4;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: start_bytes @ 0x5e30

undefined8 start_bytes(undefined8 param_1,undefined4 param_2,ulong param_3,long *param_4)

{
  ulong uVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  int *piVar4;
  long in_FS_OFFSET;
  undefined1 auStack_2038 [8200];
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  do {
    if (param_3 == 0) {
LAB_00105eb8:
      uVar2 = 0;
LAB_00105eba:
      if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
        return uVar2;
      }
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
    uVar1 = FUN_0010b540(param_2,auStack_2038,0x2000);
    if (uVar1 == 0) {
      uVar2 = 0xffffffff;
      goto LAB_00105eba;
    }
    if (uVar1 == 0xffffffffffffffff) {
      uVar2 = FUN_0010ae30(4,param_1);
      uVar3 = dcgettext(0,"error reading %s",5);
      piVar4 = __errno_location();
      error(0,*piVar4,uVar3,uVar2);
      uVar2 = 1;
      goto LAB_00105eba;
    }
    *param_4 = *param_4 + uVar1;
    if (param_3 < uVar1) {
      if (uVar1 - param_3 != 0) {
        FUN_00105c60(auStack_2038 + param_3,uVar1 - param_3);
      }
      goto LAB_00105eb8;
    }
    param_3 = param_3 - uVar1;
  } while( true );
}




// Function: dump_remainder @ 0x6020

long dump_remainder(char param_1,undefined8 param_2,undefined4 param_3,ulong param_4)

{
  long lVar1;
  int *piVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  ulong uVar5;
  char *pcVar6;
  ulong uVar7;
  long lVar8;
  long in_FS_OFFSET;
  undefined1 local_2048 [8200];
  long local_40;
  
  lVar8 = 0;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  uVar7 = param_4;
  while( true ) {
    uVar5 = 0x2000;
    if (uVar7 < 0x2001) {
      uVar5 = uVar7;
    }
    lVar1 = FUN_0010b540(param_3,local_2048,uVar5);
    if (lVar1 == -1) break;
    if (lVar1 == 0) goto LAB_001060dd;
    if (param_1 != '\0') {
      pcVar6 = "\n";
      if (DAT_00113010 != '\0') {
        pcVar6 = "";
      }
      __printf_chk(1,"%s==> %s <==\n",pcVar6,param_2);
      DAT_00113010 = '\0';
    }
    lVar8 = lVar8 + lVar1;
    FUN_00105c60(local_2048,lVar1);
    if ((param_4 != 0xffffffffffffffff) &&
       ((uVar7 = uVar7 - lVar1, uVar7 == 0 || (param_4 == 0xfffffffffffffffe)))) goto LAB_001060dd;
    param_1 = '\0';
  }
  piVar2 = __errno_location();
  if (*piVar2 != 0xb) {
    uVar3 = FUN_0010ae30(4,param_2);
    uVar4 = dcgettext(0,"error reading %s",5);
    error(1,*piVar2,uVar4,uVar3);
    goto LAB_00106186;
  }
LAB_001060dd:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return lVar8;
  }
LAB_00106186:
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: file_lines @ 0x6190

undefined8
file_lines(undefined8 param_1,int param_2,long param_3,long param_4,long param_5,long *param_6)

{
  char cVar1;
  long lVar2;
  __off_t _Var3;
  size_t __n;
  void *pvVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  int *piVar7;
  long lVar8;
  int __c;
  size_t sVar9;
  long in_FS_OFFSET;
  bool bVar10;
  undefined8 local_2050;
  undefined1 local_2048 [8200];
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_2050 = param_5;
  if (param_3 != 0) {
    lVar2 = (param_5 - param_4) % 0x2000;
    lVar8 = 0x2000;
    if (lVar2 != 0) {
      lVar8 = lVar2;
    }
    param_5 = param_5 - lVar8;
    _Var3 = lseek(param_2,param_5,0);
    if (-1 < _Var3) {
      __n = FUN_0010b540(param_2,local_2048,lVar8);
      cVar1 = DAT_00113114;
      if (__n == 0xffffffffffffffff) {
LAB_001063a7:
        uVar5 = FUN_0010ae30(4,param_1);
        uVar6 = dcgettext(0,"error reading %s",5);
        piVar7 = __errno_location();
        error(0,*piVar7,uVar6,uVar5);
        uVar5 = 0;
      }
      else {
        __c = (int)DAT_00113114;
        *param_6 = param_5 + __n;
        sVar9 = __n;
        if ((__n != 0) && (local_2048[__n - 1] != cVar1)) {
          param_3 = param_3 + -1;
        }
LAB_00106295:
        do {
          if ((__n == 0) || (pvVar4 = memrchr(local_2048,__c,__n), pvVar4 == (void *)0x0)) {
            if (param_5 == param_4) {
              _Var3 = lseek(param_2,param_5,0);
              if (-1 < _Var3) {
                lVar8 = FUN_00106020(0,param_1,param_2,local_2050);
                *param_6 = lVar8 + param_5;
                uVar5 = 1;
                goto LAB_00106315;
              }
            }
            else {
              param_5 = param_5 + -0x2000;
              _Var3 = lseek(param_2,param_5,0);
              if (-1 < _Var3) {
                __n = FUN_0010b540(param_2,local_2048,0x2000);
                if (__n == 0xffffffffffffffff) goto LAB_001063a7;
                *param_6 = param_5 + __n;
                if (__n == 0) goto LAB_00106310;
                __c = (int)DAT_00113114;
                sVar9 = __n;
                goto LAB_00106295;
              }
            }
            FUN_00105f40(param_5,0,param_1);
            goto LAB_001063a7;
          }
          __n = (long)pvVar4 - (long)local_2048;
          bVar10 = param_3 != 0;
          param_3 = param_3 + -1;
        } while (bVar10);
        if (sVar9 - 1 != __n) {
          FUN_00105c60((long)pvVar4 + 1);
        }
        lVar8 = FUN_00106020(0,param_1,param_2,(local_2050 - sVar9) - param_5);
        *param_6 = *param_6 + lVar8;
        uVar5 = 1;
      }
      goto LAB_00106315;
    }
    FUN_00105f40(param_5,0,param_1);
  }
LAB_00106310:
  uVar5 = 1;
LAB_00106315:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar5;
}




// Function: check_fspec @ 0x6430

void check_fspec(undefined8 *param_1,undefined8 *param_2)

{
  int iVar1;
  int iVar2;
  long lVar3;
  char *__s1;
  undefined8 uVar4;
  int *piVar5;
  undefined8 uVar6;
  __off_t _Var7;
  long in_FS_OFFSET;
  bool bVar8;
  stat sStack_c8;
  long local_30;
  
  iVar2 = *(int *)(param_1 + 7);
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  if (iVar2 != -1) {
    __s1 = (char *)*param_1;
    iVar1 = strcmp(__s1,"-");
    if (iVar1 == 0) {
      __s1 = (char *)dcgettext(0,"standard input",5);
      iVar2 = *(int *)(param_1 + 7);
    }
    iVar2 = fstat(iVar2,&sStack_c8);
    if (iVar2 == 0) {
      if ((*(uint *)(param_1 + 6) & 0xf000) == 0x8000) {
        if (sStack_c8.st_size < (long)param_1[1]) {
          uVar4 = FUN_0010b1b0(0,3,__s1);
          uVar6 = dcgettext(0,"%s: file truncated",5);
          error(0,0,uVar6,uVar4);
          _Var7 = lseek(*(int *)(param_1 + 7),0,0);
          if (_Var7 < 0) {
            FUN_00105f40(0,0,__s1);
            goto LAB_0010664c;
          }
          param_1[1] = 0;
        }
        else if ((sStack_c8.st_size == param_1[1]) &&
                (((uint)(sStack_c8.st_mtim.tv_nsec < (long)param_1[3]) -
                 (uint)((long)param_1[3] < sStack_c8.st_mtim.tv_nsec)) +
                 ((uint)(sStack_c8.st_mtim.tv_sec < (long)param_1[2]) -
                 (uint)((long)param_1[2] < sStack_c8.st_mtim.tv_sec)) * 2 == 0)) goto LAB_001064d7;
      }
      bVar8 = false;
      if (DAT_00113115 != '\0') {
        bVar8 = (undefined8 *)*param_2 != param_1;
      }
      lVar3 = FUN_00106020(bVar8,__s1,*(undefined4 *)(param_1 + 7),0xffffffffffffffff);
      param_1[1] = param_1[1] + lVar3;
      if (lVar3 != 0) {
        *param_2 = param_1;
        iVar2 = fflush_unlocked(stdout);
        if (iVar2 != 0) {
          uVar4 = dcgettext(0,"write error",5);
          piVar5 = __errno_location();
          error(1,*piVar5,uVar4);
          goto LAB_001065c0;
        }
      }
    }
    else {
LAB_001065c0:
      piVar5 = __errno_location();
      *(int *)((long)param_1 + 0x3c) = *piVar5;
      FUN_00105bd0(*(undefined4 *)(param_1 + 7),__s1);
      *(undefined4 *)(param_1 + 7) = 0xffffffff;
    }
  }
LAB_001064d7:
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
LAB_0010664c:
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: recheck @ 0x6660

void recheck(undefined8 *param_1,uint param_2)

{
  char cVar1;
  char cVar2;
  int iVar3;
  int __fd;
  uint uVar4;
  int iVar5;
  uint *puVar6;
  __off_t _Var7;
  int *piVar8;
  char *pcVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  char *pcVar12;
  char *pcVar13;
  uint uVar14;
  long in_FS_OFFSET;
  char *local_f0;
  stat local_d8;
  long local_40;
  
  pcVar9 = (char *)*param_1;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  iVar3 = strcmp(pcVar9,"-");
  uVar14 = *(uint *)((long)param_1 + 0x3c);
  local_f0._0_1_ = *(char *)((long)param_1 + 0x36);
  if (iVar3 == 0) {
    __fd = 0;
    uVar4 = uVar14;
  }
  else {
    __fd = FUN_00107dd0(pcVar9,((param_2 ^ 1) & 0xff) << 0xb);
    uVar4 = *(uint *)((long)param_1 + 0x3c);
  }
  cVar1 = DAT_0011311a;
  cVar2 = DAT_0011310c;
  if ((*(int *)(param_1 + 7) == -1) == (uVar4 == 0)) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("valid_file_spec (f)","src/tail.c",0x3e2,"recheck");
  }
  if (DAT_0011311a == '\0') {
    *(undefined1 *)((long)param_1 + 0x36) = 1;
    if (((cVar2 == '\0') && (iVar5 = lstat((char *)*param_1,&local_d8), iVar5 == 0)) &&
       ((local_d8.st_mode & 0xf000) == 0xa000)) goto LAB_00106cd0;
    if (__fd != -1) {
LAB_001067fb:
      iVar5 = fstat(__fd,&local_d8);
      if (iVar5 < 0) goto LAB_00106714;
      if ((((local_d8.st_mode & 0xf000) - 0x1000 & 0xffffe000) == 0) ||
         ((local_d8.st_mode & 0xb000) == 0x8000)) {
        local_f0 = (char *)*param_1;
        iVar5 = strcmp(local_f0,"-");
        pcVar9 = local_f0;
        if (iVar5 == 0) {
          pcVar9 = (char *)dcgettext(0,"standard input",5);
        }
        cVar2 = FUN_00105460(__fd,pcVar9);
        *(char *)((long)param_1 + 0x35) = cVar2;
        if ((cVar2 == '\0') || (DAT_0011310c != '\0')) {
          *(undefined4 *)((long)param_1 + 0x3c) = 0;
          uVar4 = *(uint *)(param_1 + 7);
          if ((uVar14 & 0xfffffffd) == 0) {
            pcVar9 = (char *)*param_1;
            local_f0 = (char *)(ulong)uVar4;
            pcVar12 = local_f0;
            local_f0._0_1_ = (char)uVar4;
            iVar5 = strcmp(pcVar9,"-");
            if (uVar4 == 0xffffffff) {
              if (iVar5 == 0) {
                pcVar9 = (char *)dcgettext(0,"standard input",5);
              }
              pcVar9 = (char *)FUN_0010ae30(4,pcVar9);
              pcVar13 = "%s has appeared;  following new file";
              local_f0 = pcVar12;
              goto LAB_001068cb;
            }
            if ((param_1[5] == local_d8.st_ino) && (param_1[4] == local_d8.st_dev)) {
              if (iVar5 == 0) {
                pcVar9 = (char *)dcgettext(0,"standard input",5);
              }
              FUN_00105bd0(__fd,pcVar9);
              goto LAB_001067a0;
            }
            if (iVar5 == 0) {
              pcVar9 = (char *)dcgettext(0,"standard input",5);
            }
            uVar10 = FUN_0010ae30(4,pcVar9);
            uVar11 = dcgettext(0,"%s has been replaced;  following new file",5);
            error(0,0,uVar11,uVar10);
            pcVar9 = (char *)*param_1;
            iVar5 = strcmp(pcVar9,"-");
            if (iVar5 == 0) {
              pcVar9 = (char *)dcgettext(0,"standard input",5);
            }
            FUN_00105bd0(*(undefined4 *)(param_1 + 7),pcVar9);
          }
          else {
            if (uVar4 != 0xffffffff) {
                    /* WARNING: Subroutine does not return */
              __assert_fail("f->fd == -1","src/tail.c",0x42f,"recheck");
            }
            pcVar9 = (char *)*param_1;
            iVar5 = strcmp(pcVar9,"-");
            if (iVar5 == 0) {
              pcVar9 = (char *)dcgettext(0,"standard input",5);
            }
            pcVar9 = (char *)FUN_0010ae30(4,pcVar9);
            pcVar13 = "%s has become accessible";
LAB_001068cb:
            uVar10 = dcgettext(0,pcVar13,5);
            error(0,0,uVar10,pcVar9);
          }
          uVar14 = (uint)pcVar9;
          param_2 = param_2 & 0xff;
          *(int *)(param_1 + 7) = __fd;
          if (iVar3 == 0) {
            param_2 = 0xffffffff;
          }
          *(undefined1 *)((long)param_1 + 0x34) = 0;
          param_1[1] = 0;
          param_1[2] = local_d8.st_mtim.tv_sec;
          *(uint *)(param_1 + 8) = param_2;
          pcVar9 = (char *)*param_1;
          param_1[3] = local_d8.st_mtim.tv_nsec;
          param_1[0xb] = 0;
          param_1[4] = local_d8.st_dev;
          param_1[5] = local_d8.st_ino;
          *(__mode_t *)(param_1 + 6) = local_d8.st_mode;
          iVar3 = strcmp(pcVar9,"-");
          if (iVar3 == 0) {
            pcVar9 = (char *)dcgettext(0,"standard input",5);
          }
          _Var7 = lseek(__fd,0,0);
          if (-1 < _Var7) goto LAB_001067a0;
          cVar2 = FUN_00105f40(0,0,pcVar9);
          goto LAB_00106978;
        }
        pcVar9 = (char *)*param_1;
        *(undefined4 *)((long)param_1 + 0x3c) = 0xffffffff;
        iVar3 = strcmp(pcVar9,"-");
        if (iVar3 == 0) {
          pcVar9 = (char *)dcgettext(0,"standard input",5);
        }
        uVar10 = FUN_0010ae30(4,pcVar9);
        uVar11 = dcgettext(0,"%s has been replaced with an untailable remote file",5);
        error(0,0,uVar11,uVar10);
        *(undefined2 *)((long)param_1 + 0x34) = 0x101;
      }
      else {
        *(undefined4 *)((long)param_1 + 0x3c) = 0xffffffff;
        *(undefined1 *)((long)param_1 + 0x36) = 0;
        if ((cVar1 == '\0') || (DAT_00113020 != 1)) {
          *(undefined1 *)((long)param_1 + 0x34) = 1;
          if (((char)local_f0 == '\0') && (uVar14 == 0xffffffff)) goto LAB_001069a7;
          pcVar9 = (char *)dcgettext(0,"; giving up on this name",5);
        }
        else {
          *(undefined1 *)((long)param_1 + 0x34) = 0;
          if ((char)local_f0 == '\0') {
            pcVar9 = "";
            if (uVar14 == 0xffffffff) goto LAB_001069a7;
          }
          else {
            pcVar9 = "";
          }
        }
        pcVar12 = (char *)*param_1;
        iVar3 = strcmp(pcVar12,"-");
        if (iVar3 == 0) {
          pcVar12 = (char *)dcgettext(0,"standard input",5);
        }
        uVar10 = FUN_0010ae30(4,pcVar12);
        uVar11 = dcgettext(0,"%s has been replaced with an untailable file%s",5);
        error(0,0,uVar11,uVar10,pcVar9);
      }
      goto LAB_001069a7;
    }
LAB_00106714:
    cVar2 = *(char *)((long)param_1 + 0x36);
    puVar6 = (uint *)__errno_location();
    pcVar9 = (char *)*param_1;
    uVar4 = *puVar6;
    *(uint *)((long)param_1 + 0x3c) = uVar4;
    if (cVar2 == '\0') goto LAB_00106991;
    iVar3 = strcmp(pcVar9,"-");
    if (uVar4 != uVar14) {
      if (iVar3 == 0) {
        pcVar9 = (char *)dcgettext(0,"standard input",5);
      }
      uVar10 = FUN_0010b1b0(0,3,pcVar9);
      error(0,*puVar6,&DAT_0010e513,uVar10);
      pcVar9 = (char *)*param_1;
    }
LAB_0010675a:
    iVar3 = strcmp(pcVar9,"-");
  }
  else {
    if (__fd != -1) {
      *(undefined1 *)((long)param_1 + 0x36) = 1;
      if (((cVar2 != '\0') || (iVar5 = lstat((char *)*param_1,&local_d8), iVar5 != 0)) ||
         ((local_d8.st_mode & 0xf000) != 0xa000)) goto LAB_001067fb;
      goto LAB_00106cd0;
    }
LAB_00106978:
    *(undefined1 *)((long)param_1 + 0x36) = 0;
    pcVar9 = (char *)*param_1;
    if (cVar2 == '\0') {
      iVar3 = lstat(pcVar9,&local_d8);
      if ((iVar3 != 0) || ((local_d8.st_mode & 0xf000) != 0xa000)) goto LAB_00106714;
LAB_00106cd0:
      pcVar9 = (char *)*param_1;
      *(undefined4 *)((long)param_1 + 0x3c) = 0xffffffff;
      *(undefined1 *)((long)param_1 + 0x34) = 1;
      iVar3 = strcmp(pcVar9,"-");
      if (iVar3 == 0) {
        pcVar9 = (char *)dcgettext(0,"standard input",5);
      }
      uVar10 = FUN_0010ae30(4,pcVar9);
      uVar11 = dcgettext(0,"%s has been replaced with an untailable symbolic link",5);
      error(0,0,uVar11,uVar10);
      pcVar9 = (char *)*param_1;
      goto LAB_0010675a;
    }
    piVar8 = __errno_location();
    *(int *)((long)param_1 + 0x3c) = *piVar8;
LAB_00106991:
    iVar3 = strcmp(pcVar9,"-");
    if ((char)local_f0 != '\0') {
      if (iVar3 == 0) {
        pcVar9 = (char *)dcgettext(0,"standard input",5);
      }
      uVar10 = FUN_0010ae30(4,pcVar9);
      uVar11 = dcgettext(0,"%s has become inaccessible",5);
      error(0,*(undefined4 *)((long)param_1 + 0x3c),uVar11,uVar10);
      pcVar9 = (char *)*param_1;
      goto LAB_0010675a;
    }
LAB_001069a7:
    pcVar9 = (char *)*param_1;
    iVar3 = strcmp(pcVar9,"-");
  }
  if (iVar3 == 0) {
    pcVar9 = (char *)dcgettext(0,"standard input",5);
  }
  FUN_00105bd0(__fd,pcVar9);
  pcVar9 = (char *)*param_1;
  iVar3 = strcmp(pcVar9,"-");
  if (iVar3 == 0) {
    pcVar9 = (char *)dcgettext(0,"standard input",5);
  }
  FUN_00105bd0(*(undefined4 *)(param_1 + 7),pcVar9);
  *(undefined4 *)(param_1 + 7) = 0xffffffff;
LAB_001067a0:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: usage @ 0x6f70

void usage(int param_1)

{
  FILE *pFVar1;
  int iVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  char *pcVar5;
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
  
  uVar4 = DAT_00113138;
  ppuVar6 = &local_b8;
  local_40 = *(undefined8 *)(in_FS_OFFSET + 0x28);
  if (param_1 == 0) {
    uVar3 = dcgettext(0,"Usage: %s [OPTION]... [FILE]...\n",5);
    __printf_chk(1,uVar3,uVar4);
    uVar4 = dcgettext(0,
                      "Print the last %d lines of each FILE to standard output.\nWith more than one FILE, precede each with a header giving the file name.\n"
                      ,5);
    __printf_chk(1,uVar4,10);
    pFVar1 = stdout;
    pcVar5 = (char *)dcgettext(0,"\nWith no FILE, or when FILE is -, read standard input.\n",5);
    fputs_unlocked(pcVar5,pFVar1);
    pFVar1 = stdout;
    pcVar5 = (char *)dcgettext(0,
                               "\nMandatory arguments to long options are mandatory for short options too.\n"
                               ,5);
    fputs_unlocked(pcVar5,pFVar1);
    pFVar1 = stdout;
    pcVar5 = (char *)dcgettext(0,
                               "  -c, --bytes=[+]NUM       output the last NUM bytes; or use -c +NUM to\n                             output starting with byte NUM of each file\n"
                               ,5);
    fputs_unlocked(pcVar5,pFVar1);
    pFVar1 = stdout;
    pcVar5 = (char *)dcgettext(0,
                               "  -f, --follow[={name|descriptor}]\n                           output appended data as the file grows;\n                             an absent option argument means \'descriptor\'\n  -F                       same as --follow=name --retry\n"
                               ,5);
    fputs_unlocked(pcVar5,pFVar1);
    uVar4 = dcgettext(0,
                      "  -n, --lines=[+]NUM       output the last NUM lines, instead of the last %d;\n                             or use -n +NUM to output starting with line NUM\n      --max-unchanged-stats=N\n                           with --follow=name, reopen a FILE which has not\n                             changed size after N (default %d) iterations\n                             to see if it has been unlinked or renamed\n                             (this is the usual case of rotated log files);\n                             with inotify, this option is rarely useful\n"
                      ,5);
    __printf_chk(1,uVar4,10,5);
    pFVar1 = stdout;
    pcVar5 = (char *)dcgettext(0,
                               "      --pid=PID            with -f, terminate after process ID, PID dies\n  -q, --quiet, --silent    never output headers giving file names\n      --retry              keep trying to open a file if it is inaccessible\n"
                               ,5);
    fputs_unlocked(pcVar5,pFVar1);
    pFVar1 = stdout;
    pcVar5 = (char *)dcgettext(0,
                               "  -s, --sleep-interval=N   with -f, sleep for approximately N seconds\n                             (default 1.0) between iterations;\n                             with inotify and --pid=P, check process P at\n                             least once every N seconds\n  -v, --verbose            always output headers giving file names\n"
                               ,5);
    fputs_unlocked(pcVar5,pFVar1);
    pFVar1 = stdout;
    pcVar5 = (char *)dcgettext(0,"  -z, --zero-terminated    line delimiter is NUL, not newline\n",5
                              );
    fputs_unlocked(pcVar5,pFVar1);
    pFVar1 = stdout;
    pcVar5 = (char *)dcgettext(0,"      --help        display this help and exit\n",5);
    fputs_unlocked(pcVar5,pFVar1);
    pFVar1 = stdout;
    pcVar5 = (char *)dcgettext(0,"      --version     output version information and exit\n",5);
    fputs_unlocked(pcVar5,pFVar1);
    pFVar1 = stdout;
    pcVar5 = (char *)dcgettext(0,
                               "\nNUM may have a multiplier suffix:\nb 512, kB 1000, K 1024, MB 1000*1000, M 1024*1024,\nGB 1000*1000*1000, G 1024*1024*1024, and so on for T, P, E, Z, Y.\nBinary prefixes can be used, too: KiB=K, MiB=M, and so on.\n\n"
                               ,5);
    fputs_unlocked(pcVar5,pFVar1);
    pFVar1 = stdout;
    pcVar5 = (char *)dcgettext(0,
                               "With --follow (-f), tail defaults to following the file descriptor, which\nmeans that even if a tail\'ed file is renamed, tail will continue to track\nits end.  This default behavior is not desirable when you really want to\ntrack the actual name of the file, not the file descriptor (e.g., log\nrotation).  Use --follow=name in that case.  That causes tail to track the\nnamed file in a way that accommodates renaming, removal and creation.\n"
                               ,5);
    fputs_unlocked(pcVar5,pFVar1);
    local_b8 = &DAT_0010e039;
    pcVar5 = "[";
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
      iVar2 = strcmp("tail",pcVar5);
      if (iVar2 == 0) break;
      pcVar5 = *(char **)((long)ppuVar6 + 0x10);
      ppuVar6 = (undefined **)((long)ppuVar6 + 0x10);
    } while (pcVar5 != (char *)0x0);
    puVar8 = *(undefined **)((long)ppuVar6 + 8);
    if (puVar8 == (undefined *)0x0) {
      puVar8 = &DAT_0010e03b;
    }
    uVar4 = dcgettext(0,"\n%s online help: <%s>\n",5);
    __printf_chk(1,uVar4,"GNU coreutils","https://www.gnu.org/software/coreutils/");
    pcVar5 = setlocale(5,(char *)0x0);
    if (pcVar5 != (char *)0x0) {
      iVar2 = strncmp(pcVar5,"en_",3);
      pFVar1 = stdout;
      if (iVar2 != 0) {
        pcVar5 = (char *)dcgettext(0,
                                   "Report any translation bugs to <https://translationproject.org/team/>\n"
                                   ,5);
        fputs_unlocked(pcVar5,pFVar1);
      }
    }
    puVar7 = &DAT_0010e03b;
    iVar2 = strcmp("tail","[");
    if (iVar2 == 0) {
      puVar7 = &DAT_0010e040;
    }
    uVar4 = dcgettext(0,"Full documentation <%s%s>\n",5);
    pcVar5 = " invocation";
    __printf_chk(1,uVar4,"https://www.gnu.org/software/coreutils/",puVar7);
    if (puVar8 != &DAT_0010e03b) {
      pcVar5 = "";
    }
    uVar4 = dcgettext(0,"or available locally via: info \'(coreutils) %s%s\'\n",5);
    __printf_chk(1,uVar4,puVar8,pcVar5);
  }
  else {
    uVar3 = dcgettext(0,"Try \'%s --help\' for more information.\n",5);
    __fprintf_chk(stderr,1,uVar3,uVar4);
  }
                    /* WARNING: Subroutine does not return */
  exit(param_1);
}




// Function: raw_hasher @ 0x7e50

ulong raw_hasher(ulong param_1,ulong param_2)

{
  return (param_1 >> 3 | param_1 << 0x3d) % param_2;
}




// Function: raw_comparator @ 0x7e70

bool raw_comparator(long param_1,long param_2)

{
  return param_2 == param_1;
}




// Function: check_tuning @ 0x7e80

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 check_tuning(long param_1)

{
  float fVar1;
  float *pfVar2;
  float fVar3;
  
  pfVar2 = *(float **)(param_1 + 0x28);
  if (pfVar2 == (float *)&DAT_0010e590) {
    return 1;
  }
  fVar1 = pfVar2[2];
  if ((((_DAT_0010e5a4 < fVar1) && (fVar1 < DAT_0010e5a8)) && (_DAT_0010e5ac < pfVar2[3])) &&
     (_DAT_0010e5c0 <= *pfVar2)) {
    fVar3 = *pfVar2 + _DAT_0010e5a4;
    if (((fVar3 < pfVar2[1]) && (pfVar2[1] <= DAT_0010e5b0)) && (fVar3 < fVar1)) {
      return 1;
    }
  }
  *(undefined **)(param_1 + 0x28) = &DAT_0010e590;
  return 0;
}




// Function: hash_find_entry @ 0x7f10

long hash_find_entry(long *param_1,long param_2,undefined8 *param_3,char param_4)

{
  long *plVar1;
  long lVar2;
  char cVar3;
  ulong uVar4;
  long lVar5;
  long *plVar6;
  
  uVar4 = (*(code *)param_1[6])(param_2,param_1[2]);
  if ((ulong)param_1[2] <= uVar4) {
                    /* WARNING: Subroutine does not return */
    abort();
  }
  plVar6 = (long *)(uVar4 * 0x10 + *param_1);
  *param_3 = plVar6;
  lVar5 = *plVar6;
  if (lVar5 == 0) {
LAB_00107fbf:
    lVar5 = 0;
  }
  else {
    if (lVar5 != param_2) {
      cVar3 = (*(code *)param_1[7])(param_2);
      if (cVar3 == '\0') {
        for (; (long *)plVar6[1] != (long *)0x0; plVar6 = (long *)plVar6[1]) {
          if ((*(long *)plVar6[1] == param_2) ||
             (cVar3 = (*(code *)param_1[7])(param_2), cVar3 != '\0')) {
            plVar1 = (long *)plVar6[1];
            lVar5 = *plVar1;
            if (param_4 == '\0') {
              return lVar5;
            }
            plVar6[1] = plVar1[1];
            *plVar1 = 0;
            plVar1[1] = param_1[9];
            param_1[9] = (long)plVar1;
            return lVar5;
          }
        }
        goto LAB_00107fbf;
      }
      lVar5 = *plVar6;
    }
    if (param_4 != '\0') {
      plVar1 = (long *)plVar6[1];
      if (plVar1 != (long *)0x0) {
        lVar2 = plVar1[1];
        *plVar6 = *plVar1;
        plVar6[1] = lVar2;
        *plVar1 = 0;
        plVar1[1] = param_1[9];
        param_1[9] = (long)plVar1;
        return lVar5;
      }
      *plVar6 = 0;
    }
  }
  return lVar5;
}




// Function: transfer_entries @ 0x8160

undefined8 transfer_entries(long *param_1,undefined8 *param_2,char param_3)

{
  long lVar1;
  ulong uVar2;
  long *plVar3;
  ulong uVar4;
  long *plVar5;
  long *plVar6;
  long *plVar7;
  long lVar8;
  
  plVar6 = (long *)*param_2;
  if (plVar6 < (long *)param_2[1]) {
    do {
      while (lVar8 = *plVar6, lVar8 != 0) {
        if ((long *)plVar6[1] != (long *)0x0) {
          uVar4 = param_1[2];
          plVar7 = (long *)plVar6[1];
          do {
            lVar8 = *plVar7;
            uVar2 = (*(code *)param_1[6])(lVar8,uVar4);
            uVar4 = param_1[2];
            if (uVar4 <= uVar2) goto LAB_001082cb;
            plVar3 = (long *)(uVar2 * 0x10 + *param_1);
            plVar5 = (long *)plVar7[1];
            if (*plVar3 == 0) {
              *plVar3 = lVar8;
              lVar8 = param_1[9];
              param_1[3] = param_1[3] + 1;
              *plVar7 = 0;
              plVar7[1] = lVar8;
              param_1[9] = (long)plVar7;
            }
            else {
              plVar7[1] = plVar3[1];
              plVar3[1] = (long)plVar7;
            }
            plVar7 = plVar5;
          } while (plVar5 != (long *)0x0);
          lVar8 = *plVar6;
        }
        plVar6[1] = 0;
        if (param_3 != '\0') break;
        uVar4 = (*(code *)param_1[6])(lVar8,param_1[2]);
        if ((ulong)param_1[2] <= uVar4) {
LAB_001082cb:
                    /* WARNING: Subroutine does not return */
          abort();
        }
        plVar7 = (long *)(uVar4 * 0x10 + *param_1);
        if (*plVar7 == 0) {
          *plVar7 = lVar8;
          param_1[3] = param_1[3] + 1;
        }
        else {
          plVar5 = (long *)param_1[9];
          if (plVar5 == (long *)0x0) {
            plVar5 = malloc(0x10);
            if (plVar5 == (long *)0x0) {
              return 0;
            }
          }
          else {
            param_1[9] = plVar5[1];
          }
          lVar1 = plVar7[1];
          *plVar5 = lVar8;
          plVar5[1] = lVar1;
          plVar7[1] = (long)plVar5;
        }
        *plVar6 = 0;
        plVar6 = plVar6 + 2;
        param_2[3] = param_2[3] + -1;
        if ((long *)param_2[1] <= plVar6) {
          return 1;
        }
      }
      plVar6 = plVar6 + 2;
    } while (plVar6 < (long *)param_2[1]);
  }
  return 1;
}




// Function: hash_print_statistics @ 0x83d0

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void hash_print_statistics(undefined8 *param_1,undefined8 param_2)

{
  ulong uVar1;
  ulong uVar2;
  long lVar3;
  long *plVar4;
  ulong uVar5;
  ulong uVar6;
  
  uVar6 = 0;
  uVar1 = param_1[2];
  uVar2 = param_1[3];
  for (plVar4 = (long *)*param_1; plVar4 < (long *)param_1[1]; plVar4 = plVar4 + 2) {
    while (*plVar4 == 0) {
      plVar4 = plVar4 + 2;
      if ((long *)param_1[1] <= plVar4) goto LAB_0010843d;
    }
    uVar5 = 1;
    for (lVar3 = plVar4[1]; lVar3 != 0; lVar3 = *(long *)(lVar3 + 8)) {
      uVar5 = uVar5 + 1;
    }
    if (uVar6 < uVar5) {
      uVar6 = uVar5;
    }
  }
LAB_0010843d:
  __fprintf_chk(param_2,1,"# entries:         %lu\n",param_1[4]);
  __fprintf_chk(param_2,1,"# buckets:         %lu\n",uVar1);
  __fprintf_chk(((double)uVar2 * _DAT_0010e5c0) / (double)uVar1,param_2,1,
                "# buckets used:    %lu (%.2f%%)\n",uVar2);
  __fprintf_chk(param_2,1,"max bucket length: %lu\n",uVar6);
  return;
}




// Function: hash_lookup @ 0x8530

long hash_lookup(long *param_1,long param_2)

{
  char cVar1;
  ulong uVar2;
  long *plVar3;
  long lVar4;
  
  uVar2 = (*(code *)param_1[6])(param_2,param_1[2]);
  if ((ulong)param_1[2] <= uVar2) {
                    /* WARNING: Subroutine does not return */
    abort();
  }
  plVar3 = (long *)(uVar2 * 0x10 + *param_1);
  lVar4 = *plVar3;
  if (lVar4 != 0) {
    while( true ) {
      if (lVar4 == param_2) {
        return param_2;
      }
      cVar1 = (*(code *)param_1[7])(param_2);
      if (cVar1 != '\0') break;
      plVar3 = (long *)plVar3[1];
      if (plVar3 == (long *)0x0) {
        return 0;
      }
      lVar4 = *plVar3;
    }
    return *plVar3;
  }
  return 0;
}




// Function: hash_get_next @ 0x85f0

long hash_get_next(long *param_1,long param_2)

{
  long lVar1;
  ulong uVar2;
  long *plVar3;
  long *plVar4;
  
  uVar2 = (*(code *)param_1[6])(param_2,param_1[2]);
  if ((ulong)param_1[2] <= uVar2) {
                    /* WARNING: Subroutine does not return */
    abort();
  }
  plVar3 = (long *)(uVar2 * 0x10 + *param_1);
  plVar4 = plVar3;
  do {
    lVar1 = *plVar4;
    plVar4 = (long *)plVar4[1];
    if (lVar1 == param_2) {
      if (plVar4 != (long *)0x0) {
        return *plVar4;
      }
      break;
    }
  } while (plVar4 != (long *)0x0);
  do {
    plVar3 = plVar3 + 2;
    if ((long *)param_1[1] <= plVar3) {
      return 0;
    }
  } while (*plVar3 == 0);
  return *plVar3;
}




// Function: hash_do_for_each @ 0x86d0

long hash_do_for_each(ulong *param_1,code *param_2,undefined8 param_3)

{
  char cVar1;
  long *plVar2;
  long lVar3;
  long lVar4;
  long *plVar5;
  
  plVar5 = (long *)*param_1;
  if (plVar5 < (long *)param_1[1]) {
    lVar4 = 0;
    do {
      lVar3 = *plVar5;
      plVar2 = plVar5;
      if (lVar3 != 0) {
        while( true ) {
          cVar1 = (*param_2)(lVar3,param_3);
          if (cVar1 == '\0') {
            return lVar4;
          }
          plVar2 = (long *)plVar2[1];
          lVar4 = lVar4 + 1;
          if (plVar2 == (long *)0x0) break;
          lVar3 = *plVar2;
        }
      }
      plVar5 = plVar5 + 2;
    } while (plVar5 < (long *)param_1[1]);
  }
  else {
    lVar4 = 0;
  }
  return lVar4;
}




// Function: hash_initialize @ 0x87c0

undefined8 *
hash_initialize(undefined8 param_1,undefined *param_2,code *param_3,code *param_4,undefined8 param_5)

{
  char cVar1;
  undefined8 *__ptr;
  size_t __nmemb;
  void *pvVar2;
  
  if (param_3 == (code *)0x0) {
    param_3 = FUN_00107e50;
  }
  if (param_4 == (code *)0x0) {
    param_4 = FUN_00107e70;
  }
  __ptr = malloc(0x50);
  if (__ptr != (undefined8 *)0x0) {
    if (param_2 == (undefined *)0x0) {
      param_2 = &DAT_0010e590;
    }
    __ptr[5] = param_2;
    cVar1 = FUN_00107e80(__ptr);
    if (cVar1 != '\0') {
      __nmemb = FUN_00108020(*(undefined4 *)(param_2 + 8),param_1,param_2[0x10]);
      __ptr[2] = __nmemb;
      if (__nmemb != 0) {
        pvVar2 = calloc(__nmemb,0x10);
        *__ptr = pvVar2;
        if (pvVar2 != (void *)0x0) {
          __ptr[6] = param_3;
          __ptr[3] = 0;
          __ptr[7] = param_4;
          __ptr[8] = param_5;
          __ptr[1] = (void *)((long)pvVar2 + __ptr[2] * 0x10);
          __ptr[4] = 0;
          __ptr[9] = 0;
          return __ptr;
        }
      }
    }
    free(__ptr);
  }
  return (undefined8 *)0x0;
}




// Function: hash_clear @ 0x88d0

void hash_clear(undefined8 *param_1)

{
  undefined8 *puVar1;
  undefined8 uVar2;
  undefined8 *puVar3;
  code *pcVar4;
  code *pcVar5;
  long *plVar6;
  long *plVar7;
  
  plVar6 = (long *)*param_1;
  if ((long *)*param_1 < (long *)param_1[1]) {
    do {
      while (*plVar6 != 0) {
        puVar3 = (undefined8 *)plVar6[1];
        pcVar4 = (code *)param_1[8];
        while (puVar3 != (undefined8 *)0x0) {
          pcVar5 = (code *)0x0;
          if (pcVar4 != (code *)0x0) {
            (*pcVar4)(*puVar3);
            pcVar5 = (code *)param_1[8];
          }
          puVar1 = (undefined8 *)puVar3[1];
          uVar2 = param_1[9];
          *puVar3 = 0;
          puVar3[1] = uVar2;
          param_1[9] = puVar3;
          puVar3 = puVar1;
          pcVar4 = pcVar5;
        }
        if (pcVar4 != (code *)0x0) {
          (*pcVar4)(*plVar6);
        }
        *plVar6 = 0;
        plVar7 = plVar6 + 2;
        plVar6[1] = 0;
        plVar6 = plVar7;
        if ((long *)param_1[1] <= plVar7) goto LAB_0010896b;
      }
      plVar6 = plVar6 + 2;
    } while (plVar6 < (long *)param_1[1]);
  }
LAB_0010896b:
  param_1[3] = 0;
  param_1[4] = 0;
  return;
}




// Function: hash_free @ 0x8980

void hash_free(ulong *param_1)

{
  void *pvVar1;
  void *pvVar2;
  long *plVar3;
  long *plVar4;
  long *plVar5;
  long lVar6;
  
  plVar5 = (long *)*param_1;
  plVar3 = (long *)param_1[1];
  if ((param_1[8] != 0) && (param_1[4] != 0)) {
    if (plVar3 <= plVar5) goto LAB_00108a2c;
    do {
      while (lVar6 = *plVar5, plVar4 = plVar5, lVar6 == 0) {
        plVar5 = plVar5 + 2;
        if (plVar3 <= plVar5) goto LAB_001089ef;
      }
      while( true ) {
        (*(code *)param_1[8])(lVar6);
        plVar4 = (long *)plVar4[1];
        if (plVar4 == (long *)0x0) break;
        lVar6 = *plVar4;
      }
      plVar3 = (long *)param_1[1];
      plVar5 = plVar5 + 2;
    } while (plVar5 < plVar3);
LAB_001089ef:
    plVar5 = (long *)*param_1;
  }
  if (plVar5 < plVar3) {
    do {
      pvVar2 = (void *)plVar5[1];
      while (pvVar2 != (void *)0x0) {
        pvVar1 = *(void **)((long)pvVar2 + 8);
        free(pvVar2);
        pvVar2 = pvVar1;
      }
      plVar5 = plVar5 + 2;
    } while (plVar5 < (long *)param_1[1]);
  }
LAB_00108a2c:
  pvVar2 = (void *)param_1[9];
  while (pvVar2 != (void *)0x0) {
    pvVar1 = *(void **)((long)pvVar2 + 8);
    free(pvVar2);
    pvVar2 = pvVar1;
  }
  free((void *)*param_1);
  free(param_1);
  return;
}




// Function: hash_rehash @ 0x8a70

undefined4 hash_rehash(undefined8 *param_1,undefined8 param_2)

{
  char cVar1;
  undefined4 uVar2;
  size_t __nmemb;
  long in_FS_OFFSET;
  void *local_88;
  void *local_80;
  size_t local_78;
  undefined8 local_70;
  undefined8 local_68;
  undefined8 local_60;
  undefined8 local_58;
  undefined8 local_50;
  undefined8 local_48;
  undefined8 local_40;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  __nmemb = FUN_00108020(*(undefined4 *)(param_1[5] + 8),param_2,*(undefined1 *)(param_1[5] + 0x10))
  ;
  if (__nmemb != 0) {
    if (param_1[2] == __nmemb) {
      uVar2 = 1;
      goto LAB_00108b76;
    }
    local_88 = calloc(__nmemb,0x10);
    if (local_88 != (void *)0x0) {
      local_80 = (void *)((long)local_88 + __nmemb * 0x10);
      local_70 = 0;
      local_60 = param_1[5];
      local_68 = 0;
      local_58 = param_1[6];
      local_50 = param_1[7];
      local_48 = param_1[8];
      local_40 = param_1[9];
      local_78 = __nmemb;
      uVar2 = FUN_00108160(&local_88,param_1,0);
      if ((char)uVar2 == '\0') {
        param_1[9] = local_40;
        cVar1 = FUN_00108160(param_1,&local_88,1);
        if (cVar1 != '\0') {
          cVar1 = FUN_00108160(param_1,&local_88,0);
          if (cVar1 != '\0') {
            free(local_88);
            goto LAB_00108b76;
          }
        }
                    /* WARNING: Subroutine does not return */
        abort();
      }
      free((void *)*param_1);
      *param_1 = local_88;
      param_1[1] = local_80;
      param_1[2] = local_78;
      param_1[3] = local_70;
      param_1[9] = local_40;
      goto LAB_00108b76;
    }
  }
  uVar2 = 0;
LAB_00108b76:
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: hash_insert_if_absent @ 0x8bf0

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 hash_insert_if_absent(long param_1,long param_2,long *param_3)

{
  char cVar1;
  long lVar2;
  ulong uVar3;
  long *plVar4;
  ulong uVar5;
  undefined8 uVar6;
  long in_FS_OFFSET;
  float fVar7;
  undefined1 auVar8 [16];
  undefined1 auVar9 [16];
  long *local_38;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_2 == 0) goto LAB_00108df3;
  lVar2 = FUN_00107f10(param_1,param_2,&local_38,0);
  if (lVar2 != 0) {
    uVar6 = 0;
    if (param_3 != (long *)0x0) {
      *param_3 = lVar2;
    }
    goto LAB_00108c3e;
  }
  uVar5 = *(ulong *)(param_1 + 0x18);
  if ((long)uVar5 < 0) {
    uVar3 = *(ulong *)(param_1 + 0x10);
    auVar9._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar9._0_4_ = (float)uVar5;
    if (-1 < (long)uVar3) goto LAB_00108c7f;
LAB_00108d06:
    auVar8._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar8._0_4_ = (float)uVar3;
  }
  else {
    auVar9._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar9._0_4_ = (float)(long)uVar5;
    uVar3 = *(ulong *)(param_1 + 0x10);
    if ((long)uVar3 < 0) goto LAB_00108d06;
LAB_00108c7f:
    auVar8._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar8._0_4_ = (float)(long)uVar3;
  }
  if (*(float *)(*(long *)(param_1 + 0x28) + 8) * auVar8._0_4_ < auVar9._0_4_) {
    FUN_00107e80(param_1);
    lVar2 = *(long *)(param_1 + 0x28);
    if (auVar9._0_4_ <= auVar8._0_4_ * *(float *)(lVar2 + 8)) goto LAB_00108c9e;
    fVar7 = auVar8._0_4_ * *(float *)(lVar2 + 0xc);
    if (*(char *)(lVar2 + 0x10) == '\0') {
      fVar7 = fVar7 * *(float *)(lVar2 + 8);
    }
    if (fVar7 < _DAT_0010e5b4) {
      if (_DAT_0010e5b8 <= fVar7) {
        uVar5 = (long)(fVar7 - _DAT_0010e5b8) ^ 0x8000000000000000;
      }
      else {
        uVar5 = (ulong)fVar7;
      }
      cVar1 = FUN_00108a70(param_1,uVar5);
      if (cVar1 != '\0') {
        lVar2 = FUN_00107f10(param_1,param_2,&local_38,0);
        if (lVar2 != 0) {
LAB_00108df3:
                    /* WARNING: Subroutine does not return */
          abort();
        }
        goto LAB_00108c9e;
      }
    }
LAB_00108d5d:
    uVar6 = 0xffffffff;
  }
  else {
LAB_00108c9e:
    if (*local_38 == 0) {
      *local_38 = param_2;
      uVar6 = 1;
      *(long *)(param_1 + 0x20) = *(long *)(param_1 + 0x20) + 1;
      *(long *)(param_1 + 0x18) = *(long *)(param_1 + 0x18) + 1;
      goto LAB_00108c3e;
    }
    plVar4 = *(long **)(param_1 + 0x48);
    if (plVar4 == (long *)0x0) {
      plVar4 = malloc(0x10);
      if (plVar4 == (long *)0x0) goto LAB_00108d5d;
    }
    else {
      *(long *)(param_1 + 0x48) = plVar4[1];
    }
    lVar2 = local_38[1];
    *plVar4 = param_2;
    uVar6 = 1;
    plVar4[1] = lVar2;
    local_38[1] = (long)plVar4;
    *(long *)(param_1 + 0x20) = *(long *)(param_1 + 0x20) + 1;
  }
LAB_00108c3e:
  if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar6;
}




// Function: hash_insert @ 0x8e20

undefined8 hash_insert(undefined8 param_1,undefined8 param_2)

{
  int iVar1;
  long in_FS_OFFSET;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = FUN_00108bf0(param_1,param_2,&local_18);
  if (iVar1 == -1) {
    param_2 = 0;
  }
  else if (iVar1 == 0) {
    param_2 = local_18;
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return param_2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: hash_remove @ 0x8e80

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

long hash_remove(long param_1,undefined8 param_2)

{
  float *pfVar1;
  void *pvVar2;
  void *__ptr;
  char cVar3;
  long lVar4;
  ulong uVar5;
  ulong uVar6;
  long in_FS_OFFSET;
  float fVar7;
  undefined1 auVar8 [16];
  undefined1 auVar9 [16];
  long *local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  lVar4 = FUN_00107f10(param_1,param_2,&local_28,1);
  if ((lVar4 == 0) || (*(long *)(param_1 + 0x20) = *(long *)(param_1 + 0x20) + -1, *local_28 != 0))
  goto LAB_00108ec3;
  uVar5 = *(long *)(param_1 + 0x18) - 1;
  *(ulong *)(param_1 + 0x18) = uVar5;
  if ((long)uVar5 < 0) {
    uVar6 = *(ulong *)(param_1 + 0x10);
    auVar9._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar9._0_4_ = (float)uVar5;
    if (-1 < (long)uVar6) goto LAB_00108f10;
LAB_00108fc6:
    auVar8._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar8._0_4_ = (float)uVar6;
  }
  else {
    auVar9._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar9._0_4_ = (float)(long)uVar5;
    uVar6 = *(ulong *)(param_1 + 0x10);
    if ((long)uVar6 < 0) goto LAB_00108fc6;
LAB_00108f10:
    auVar8._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar8._0_4_ = (float)(long)uVar6;
  }
  if (auVar9._0_4_ < **(float **)(param_1 + 0x28) * auVar8._0_4_) {
    FUN_00107e80(param_1);
    pfVar1 = *(float **)(param_1 + 0x28);
    if (auVar9._0_4_ < *pfVar1 * auVar8._0_4_) {
      fVar7 = auVar8._0_4_ * pfVar1[1];
      if (*(char *)(pfVar1 + 4) == '\0') {
        fVar7 = fVar7 * pfVar1[2];
      }
      if (_DAT_0010e5b8 <= fVar7) {
        uVar5 = (long)(fVar7 - _DAT_0010e5b8) ^ 0x8000000000000000;
      }
      else {
        uVar5 = (ulong)fVar7;
      }
      cVar3 = FUN_00108a70(param_1,uVar5);
      if (cVar3 == '\0') {
        __ptr = *(void **)(param_1 + 0x48);
        while (__ptr != (void *)0x0) {
          pvVar2 = *(void **)((long)__ptr + 8);
          free(__ptr);
          __ptr = pvVar2;
        }
        *(undefined8 *)(param_1 + 0x48) = 0;
      }
    }
  }
LAB_00108ec3:
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return lVar4;
}



