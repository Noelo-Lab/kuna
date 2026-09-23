// Function: main @ 0x2a20

undefined8 main(int param_1,undefined8 *param_2)

{
  long *plVar1;
  long *plVar2;
  byte *pbVar3;
  void *__ptr;
  size_t __n;
  FILE *pFVar4;
  uint uVar5;
  char cVar6;
  byte bVar7;
  undefined1 uVar8;
  byte bVar9;
  int iVar10;
  uint uVar11;
  int iVar12;
  uint uVar13;
  undefined8 uVar14;
  long lVar15;
  long lVar16;
  long lVar18;
  long lVar19;
  _IO_marker *p_Var20;
  char *pcVar21;
  undefined8 uVar22;
  char *pcVar23;
  undefined **ppuVar24;
  long *plVar25;
  char *pcVar26;
  byte bVar27;
  uint uVar28;
  ulong uVar29;
  char *pcVar30;
  char *pcVar31;
  long lVar32;
  _IO_FILE *p_Var33;
  _IO_FILE *p_Var34;
  _IO_FILE *p_Var35;
  _IO_FILE *p_Var36;
  char *pcVar37;
  long lVar38;
  long in_FS_OFFSET;
  bool bVar39;
  undefined1 auVar40 [12];
  char *local_1f0;
  long *local_1e8;
  _IO_FILE **local_1d0;
  long local_1b0;
  byte local_18d;
  undefined1 local_180 [8];
  void *local_178;
  void *local_170;
  _IO_FILE *local_168;
  int local_160 [5];
  int local_14c;
  long local_148 [6];
  char *local_118 [8];
  stat local_d8;
  char local_42;
  undefined1 local_41;
  long local_40;
  ulong uVar17;
  
  p_Var34 = (_IO_FILE *)0x0;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  DAT_00111020 = 2;
  FUN_001050e0(*param_2);
  setlocale(6,"");
  bindtextdomain("diffutils","/usr/local/share/locale");
  textdomain("diffutils");
  FUN_00105020(0);
  FUN_00107940();
  iVar12 = 0;
  while( true ) {
    uVar11 = 0x110940;
    iVar10 = getopt_long(param_1,param_2,"aeimvx3AEL:TX");
    uVar22 = DAT_00121140;
    uVar28 = (uint)p_Var34;
    if (iVar10 == -1) break;
    switch(iVar10) {
    case 0x33:
      DAT_001110ed = '\x01';
      p_Var34 = (_IO_FILE *)(ulong)(uVar28 | 1);
      break;
    default:
      goto switchD_00102aef_caseD_34;
    case 0x41:
      DAT_001110eb = 1;
      p_Var34 = (_IO_FILE *)(ulong)(uVar28 | 2);
      DAT_001110ef = 1;
      break;
    case 0x45:
      DAT_001110ef = 1;
      p_Var34 = (_IO_FILE *)(ulong)(uVar28 | 4);
      break;
    case 0x4c:
      if (2 < iVar12) {
        FUN_001042a0("too many file label options",0);
        goto LAB_00104185;
      }
      local_148[(long)iVar12 + 10] = (long)optarg;
      iVar12 = iVar12 + 1;
      break;
    case 0x54:
      DAT_001110ee = '\x01';
      break;
    case 0x58:
      DAT_001110ec = '\x01';
      p_Var34 = (_IO_FILE *)(ulong)(uVar28 | 8);
      break;
    case 0x61:
      DAT_001110f2 = 1;
      break;
    case 0x65:
      p_Var34 = (_IO_FILE *)(ulong)(uVar28 | 0x10);
      break;
    case 0x69:
      DAT_001110ea = 1;
      break;
    case 0x6d:
      DAT_001110e9 = 1;
      break;
    case 0x76:
      uVar22 = FUN_00105580("Randy Smith");
      FUN_00107150(stdout,"diff3","GNU diffutils",PTR_DAT_00111018,uVar22,0);
      FUN_00104690();
      goto LAB_00102d26;
    case 0x78:
      DAT_001110ec = '\x01';
      p_Var34 = (_IO_FILE *)(ulong)(uVar28 | 0x20);
      break;
    case 0x80:
      PTR_DAT_00111010 = optarg;
      break;
    case 0x81:
      uVar14 = dcgettext(0,"Usage: %s [OPTION]... MYFILE OLDFILE YOURFILE\n",5);
      ppuVar24 = &PTR_s__A____show_all_output_all_change_001108a0;
      __printf_chk(1,uVar14,uVar22);
      uVar22 = dcgettext(0,"Compare three files line by line.",5);
      __printf_chk(1,&DAT_0010ca44,uVar22);
      pFVar4 = stdout;
      pcVar26 = (char *)dcgettext(0,
                                  "Mandatory arguments to long options are mandatory for short options too.\n"
                                  ,5);
      fputs_unlocked(pcVar26,pFVar4);
      pcVar26 = "-A, --show-all              output all changes, bracketing conflicts";
      do {
        if (*pcVar26 == '\0') {
          pcVar26 = stdout->_IO_write_ptr;
          if (pcVar26 < stdout->_IO_write_end) {
            stdout->_IO_write_ptr = pcVar26 + 1;
            *pcVar26 = '\n';
          }
          else {
            __overflow(stdout,10);
          }
          pcVar26 = ppuVar24[1];
          pFVar4 = stdout;
        }
        else {
          uVar22 = dcgettext(0,pcVar26,5);
          __printf_chk(1,"  %s\n",uVar22);
          pcVar26 = ppuVar24[1];
          pFVar4 = stdout;
        }
        ppuVar24 = ppuVar24 + 1;
        stdout = pFVar4;
      } while (pcVar26 != (char *)0x0);
      pcVar26 = (char *)dcgettext(0,
                                  "\nThe default output format is a somewhat human-readable representation of\nthe changes.\n\nThe -e, -E, -x, -X (and corresponding long) options cause an ed script\nto be output instead of the default.\n\nFinally, the -m (--merge) option causes diff3 to do the merge internally\nand output the actual merged file.  For unusual input, this is more\nrobust than using ed.\n"
                                  ,5);
      fputs_unlocked(pcVar26,pFVar4);
      uVar22 = dcgettext(0,"Exit status is 0 if successful, 1 if conflicts, 2 if trouble.",5);
      uVar14 = dcgettext(0,"If a FILE is \'-\', read standard input.",5);
      __printf_chk(1,"\n%s\n%s\n",uVar14,uVar22);
      FUN_00107230();
      FUN_00104690();
LAB_00102d26:
      if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
        return 0;
      }
LAB_00104185:
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    case 0x82:
      DAT_001110f1 = 1;
    }
  }
  uVar11 = (uint)DAT_001110e9;
  DAT_001110f0 = (DAT_001110e9 ^ 1) & uVar28 != 0;
  bVar9 = uVar28 == 0 & DAT_001110e9;
  DAT_001110eb = DAT_001110eb | bVar9;
  DAT_001110ef = bVar9 | DAT_001110ef;
  if ((((uVar28 - 1 & uVar28) == 0) && ((DAT_001110e9 & DAT_001110ea) == 0)) &&
     ((iVar12 == 0 || (DAT_001110ef != 0)))) {
    lVar15 = (long)optind;
    auVar40[8] = uVar28 != 0;
    auVar40._0_8_ = lVar15;
    auVar40._9_3_ = 0;
    local_18d = 0;
    if (param_1 - optind != 3) {
      if (param_1 - optind < 3) {
        FUN_001042a0("missing operand after \'%s\'",param_2[(long)param_1 + -1]);
switchD_00102aef_caseD_34:
        lVar15 = FUN_001042a0(0,0);
      }
      auVar40 = FUN_001042a0("extra operand \'%s\'",param_2[lVar15 + 3]);
    }
    p_Var36 = (_IO_FILE *)(param_2 + auVar40._0_8_);
    for (lVar15 = (long)iVar12; (int)lVar15 < 3; lVar15 = lVar15 + 1) {
      local_148[lVar15 + 10] = *(long *)(&p_Var36->_flags + lVar15 * 2);
    }
    uVar28 = (auVar40._8_4_ | uVar11) & 0xff;
    uVar11 = 2 - uVar28;
    local_1b0 = (long)(int)uVar11;
    iVar12 = strcmp(*(char **)(&p_Var36->_flags + local_1b0 * 2),"-");
    if (iVar12 != 0) {
LAB_00102ed8:
      local_160[0] = 0;
      local_160[3] = 0;
      iVar12 = 3 - uVar11;
      lVar15 = (long)iVar12;
      local_160[lVar15 + 3] = 1;
      local_160[local_1b0 + 3] = 2;
      p_Var33 = (_IO_FILE *)&p_Var36->_IO_read_base;
      p_Var35 = p_Var36;
      local_160[1] = iVar12;
      local_160[2] = uVar11;
      do {
        pcVar26 = *(char **)p_Var35;
        iVar10 = strcmp(pcVar26,"-");
        if (iVar10 != 0) {
          iVar10 = stat(pcVar26,&local_d8);
          if (iVar10 < 0) goto LAB_00103afe;
          if ((local_d8.st_mode & 0xf000) == 0x4000) {
                    /* WARNING: Subroutine does not return */
            error(2,0x15,&DAT_0010c958,*(undefined8 *)p_Var35);
          }
        }
        p_Var35 = (_IO_FILE *)&p_Var35->_IO_read_ptr;
      } while (p_Var33 != p_Var35);
      signal(0x11,(__sighandler_t)0x0);
      p_Var34 = (_IO_FILE *)&DAT_0010cd20;
      uVar22 = *(undefined8 *)(&p_Var36->_flags + (long)local_14c * 2);
      lVar16 = FUN_001046e0(*(undefined8 *)(&p_Var36->_flags + (long)local_160[4] * 2),uVar22,
                            local_180,&local_170);
      local_148[4] = FUN_001046e0(*(undefined8 *)p_Var36,uVar22,local_180,&local_178);
      local_148[5] = lVar16;
      local_1d0 = &local_168;
      local_168 = (_IO_FILE *)0x0;
LAB_00102ff0:
      p_Var35 = local_168;
      bVar27 = DAT_001110f0;
      bVar9 = DAT_001110e9;
      pFVar4 = stdout;
      if (local_148[4] != 0) {
        local_148[3] = 0;
        local_148[2] = 0;
        local_148[1] = 0;
        local_148[0] = 0;
        if (local_148[5] == 0) {
          uVar28 = 0;
          lVar32 = 0;
          uVar17 = 1;
          uVar29 = 0;
          lVar16 = local_148[4];
        }
        else {
          lVar16 = *(long *)(local_148[5] + 0x10);
          lVar32 = *(long *)(local_148[4] + 0x10);
          uVar29 = (ulong)(lVar16 < lVar32);
          uVar28 = (uint)(lVar16 < lVar32);
          uVar17 = (ulong)(lVar32 <= lVar16);
          lVar32 = uVar29 << 3;
          lVar16 = local_148[uVar29 + 4];
        }
LAB_0010306f:
        local_148[3] = 0;
        local_148[2] = 0;
        local_148[1] = 0;
        local_148[0] = 0;
        lVar38 = *(long *)(lVar16 + 0x40);
        lVar18 = *(long *)(lVar16 + 0x18);
        local_148[uVar29 + 2] = lVar16;
        local_148[uVar29] = lVar16;
        local_148[uVar29 + 4] = lVar38;
        lVar38 = local_148[uVar17 + 4];
        *(undefined8 *)(lVar16 + 0x40) = 0;
        if (lVar38 != 0) {
          do {
            if (lVar18 + 1 < *(long *)(lVar38 + 0x10)) break;
            if (local_148[uVar17] == 0) {
              local_148[uVar17] = lVar38;
            }
            else {
              *(long *)(local_148[uVar17 + 2] + 0x40) = lVar38;
            }
            local_148[uVar17 + 2] = lVar38;
            lVar16 = *(long *)(local_148[uVar17 + 4] + 0x40);
            *(undefined8 *)(lVar38 + 0x40) = 0;
            lVar38 = *(long *)(lVar38 + 0x18);
            local_148[uVar17 + 4] = lVar16;
            uVar5 = uVar28;
            uVar13 = uVar28 ^ 1;
            if (lVar18 < lVar38) {
              lVar18 = lVar38;
              uVar5 = uVar28 ^ 1;
              uVar13 = uVar28;
            }
            uVar28 = uVar5;
            uVar17 = (ulong)(int)uVar13;
            lVar38 = local_148[uVar17 + 4];
          } while (lVar38 != 0);
          uVar29 = (ulong)(int)uVar28;
        }
        pcVar26 = *(char **)(*(long *)((long)local_148 + lVar32) + 0x10);
        pcVar30 = *(char **)(local_148[uVar29 + 2] + 0x18);
        lVar16 = 0;
        do {
          plVar25 = *(long **)((long)local_148 + lVar16);
          if (plVar25 == (long *)0x0) {
            *(char **)((long)local_148 + lVar16 + 0x30) =
                 pcVar26 + (*(long *)(p_Var34->_shortbuf + lVar16 * 2 + -0x73) -
                           (long)p_Var34->_IO_write_end);
            lVar32 = *(long *)(p_Var34->_shortbuf + lVar16 * 2 + -0x73) -
                     (long)p_Var34->_IO_write_end;
          }
          else {
            *(char **)((long)local_148 + lVar16 + 0x30) = pcVar26 + (*plVar25 - plVar25[2]);
            lVar32 = *(long *)(lVar16 + (long)(local_148 + 2));
            lVar32 = *(long *)(lVar32 + 8) - *(long *)(lVar32 + 0x18);
          }
          pcVar23 = local_118[1];
          pcVar21 = local_118[0];
          *(char **)((long)local_148 + lVar16 + 0x40) = pcVar30 + lVar32;
          pcVar37 = local_118[3];
          pcVar31 = local_118[2];
          lVar16 = lVar16 + 8;
        } while (lVar16 != 0x10);
        p_Var34 = (_IO_FILE *)FUN_00107350(0x70);
        p_Var34->_flags = 0;
        p_Var34->_chain = (_IO_FILE *)0x0;
        p_Var34->_IO_read_ptr = pcVar21;
        p_Var34->_IO_read_base = pcVar23;
        p_Var34->_IO_read_end = pcVar31;
        p_Var34->_IO_write_base = pcVar37;
        p_Var34->_IO_write_ptr = pcVar26;
        p_Var34->_IO_write_end = pcVar30;
        pcVar30 = pcVar31 + (1 - (long)pcVar21);
        if (pcVar30 == (char *)0x0) {
          p_Var34->_IO_buf_base = (char *)0x0;
          p_Var34->_IO_backup_base = (char *)0x0;
        }
        else {
          pcVar37 = (char *)FUN_001076b0(pcVar30,8);
          p_Var34->_IO_buf_base = pcVar37;
          pcVar30 = (char *)FUN_001076b0(pcVar30);
          pcVar37 = p_Var34->_IO_write_base;
          pcVar23 = p_Var34->_IO_read_base;
          p_Var34->_IO_backup_base = pcVar30;
        }
        pcVar37 = pcVar37 + (1 - (long)pcVar23);
        if (pcVar37 == (char *)0x0) {
          p_Var34->_IO_buf_end = (char *)0x0;
          pcVar30 = (char *)0x0;
        }
        else {
          pcVar30 = (char *)FUN_001076b0(pcVar37,8);
          p_Var34->_IO_buf_end = pcVar30;
          pcVar30 = (char *)FUN_001076b0(pcVar37);
        }
        p_Var34->_IO_save_end = pcVar30;
        pcVar30 = p_Var34->_IO_write_end + (1 - (long)p_Var34->_IO_write_ptr);
        if (pcVar30 == (char *)0x0) {
          p_Var34->_IO_save_base = (char *)0x0;
          p_Var20 = (_IO_marker *)0x0;
        }
        else {
          pcVar37 = (char *)FUN_001076b0(pcVar30,8);
          p_Var34->_IO_save_base = pcVar37;
          p_Var20 = (_IO_marker *)FUN_001076b0(pcVar30);
        }
        p_Var34->_markers = p_Var20;
        local_1e8 = local_148;
        do {
          lVar16 = *local_1e8;
          if (lVar16 != 0) {
            pcVar30 = p_Var34->_IO_save_base;
            do {
              lVar32 = (*(long *)(lVar16 + 0x10) - (long)pcVar26) * 8;
              cVar6 = FUN_00104460(*(undefined8 *)(lVar16 + 0x28),*(undefined8 *)(lVar16 + 0x38),
                                   pcVar30 + lVar32,(long)p_Var20 + lVar32,
                                   (*(long *)(lVar16 + 0x18) - *(long *)(lVar16 + 0x10)) + 1);
              if (cVar6 == '\0') goto LAB_001032db;
              lVar16 = *(long *)(lVar16 + 0x40);
            } while (lVar16 != 0);
          }
          local_1e8 = local_1e8 + 1;
        } while (local_148 + 2 != local_1e8);
        lVar16 = 0;
        do {
          plVar25 = *(long **)((long)local_148 + lVar16);
          lVar32 = 0;
          pcVar30 = pcVar21;
          while( true ) {
            pcVar37 = pcVar31 + 1;
            if (plVar25 != (long *)0x0) {
              pcVar37 = (char *)*plVar25;
            }
            if ((long)pcVar37 <= (long)pcVar30) break;
            pcVar30 = pcVar30 + 1;
            *(undefined8 *)(*(long *)(p_Var34->_shortbuf + lVar16 + -0x4b) + lVar32) =
                 *(undefined8 *)(p_Var34->_IO_save_base + lVar32);
            *(undefined8 *)(*(long *)(p_Var34->_shortbuf + lVar16 + -0x33) + lVar32) =
                 *(undefined8 *)((long)&p_Var20->_next + lVar32);
            lVar32 = lVar32 + 8;
          }
          if (plVar25 != (long *)0x0) {
            lVar32 = *(long *)(p_Var34->_shortbuf + lVar16 + -0x4b);
            lVar38 = *(long *)(p_Var34->_shortbuf + lVar16 + -0x33);
LAB_001033b0:
            lVar18 = (*plVar25 - (long)pcVar21) * 8;
            cVar6 = FUN_00104460(plVar25[4],plVar25[6],lVar32 + lVar18,lVar38 + lVar18);
            if (cVar6 != '\0') {
              plVar1 = plVar25 + 1;
              plVar2 = plVar25 + 3;
              plVar25 = (long *)plVar25[8];
              lVar19 = (*plVar1 + 1) - (long)pcVar21;
              lVar18 = (*plVar2 - (long)pcVar26) * 8;
              do {
                if (plVar25 == (long *)0x0) {
                  if ((long)(pcVar31 + (1 - (long)pcVar21)) <= lVar19) goto LAB_00103447;
                }
                else if (*plVar25 - (long)pcVar21 <= lVar19) goto LAB_001033b0;
                *(undefined8 *)(lVar32 + lVar19 * 8) =
                     *(undefined8 *)(p_Var34->_IO_save_base + lVar18 + 8);
                *(undefined8 *)(lVar38 + lVar19 * 8) =
                     *(undefined8 *)((long)&p_Var20->_sbuf + lVar18);
                lVar19 = lVar19 + 1;
                lVar18 = lVar18 + 8;
              } while( true );
            }
LAB_001032db:
                    /* WARNING: Subroutine does not return */
            FUN_00104300("internal error: screwup in format of diff blocks");
          }
LAB_00103447:
          if (lVar16 + 8 == 0x10) goto code_r0x00103455;
          pcVar21 = *(char **)((long)local_148 + lVar16 + 0x38);
          pcVar31 = *(char **)((long)local_148 + lVar16 + 0x48);
          lVar16 = lVar16 + 8;
        } while( true );
      }
      if (local_148[5] != 0) {
        local_148[3] = 0;
        uVar17 = 0;
        lVar32 = 8;
        uVar28 = 1;
        local_148[2] = 0;
        uVar29 = 1;
        local_148[1] = 0;
        local_148[0] = 0;
        lVar16 = local_148[5];
        goto LAB_0010306f;
      }
      if (DAT_001110f0 != 0) {
        p_Var34 = (_IO_FILE *)0x0;
        p_Var36 = local_168;
        while (p_Var36 != (_IO_FILE *)0x0) {
          p_Var33 = p_Var36->_chain;
          p_Var36->_chain = p_Var34;
          p_Var34 = p_Var36;
          p_Var36 = p_Var33;
        }
        lVar15 = lVar15 * 0x10;
        local_1b0 = local_1b0 * 0x10;
        do {
          if (p_Var34 == (_IO_FILE *)0x0) {
            if (DAT_001110ea != 0) {
              fputs_unlocked("w\nq\n",pFVar4);
            }
            goto LAB_00103918;
          }
          if (p_Var34->_flags == 4) {
LAB_00103da6:
            if (DAT_001110ed == '\0') {
              pcVar26 = p_Var34->_IO_read_ptr;
              pcVar30 = p_Var34->_IO_read_end;
              if (DAT_001110ef == 0) {
LAB_00103f3d:
                if (*(long *)(p_Var34->_shortbuf + local_1b0 + -0x73) -
                    *(long *)(p_Var34->_shortbuf + local_1b0 + -0x7b) == -1) {
                  if (pcVar26 == pcVar30) {
                    __fprintf_chk(pFVar4,1,"%ldd\n",pcVar26);
                  }
                  else {
                    __fprintf_chk(pFVar4,1,&DAT_0010caea,pcVar26,pcVar30);
                  }
                }
                else {
                  if ((long)pcVar30 - (long)pcVar26 == -1) {
                    __fprintf_chk(pFVar4,1,"%lda\n",pcVar30);
                  }
                  else if (pcVar30 == pcVar26) {
                    __fprintf_chk(pFVar4,1,"%ldc\n",pcVar30);
                  }
                  else {
                    __fprintf_chk(pFVar4,1,&DAT_0010cafa,pcVar26,pcVar30);
                  }
                  lVar16 = *(long *)(p_Var34->_shortbuf + local_1b0 + -0x73);
                  lVar32 = *(long *)(p_Var34->_shortbuf + local_1b0 + -0x7b);
                  uVar8 = FUN_00104500(pFVar4,p_Var34,uVar11);
                  FUN_001045f0(pFVar4,uVar8,pcVar26,lVar16 + (1 - lVar32));
                }
              }
              else {
                __fprintf_chk(pFVar4,1,"%lda\n",pcVar30);
                bVar9 = 0;
                if (DAT_001110eb != 0) {
                  __fprintf_chk(pFVar4,1,"||||||| %s\n",local_118[5]);
                  bVar9 = FUN_00104500(pFVar4,p_Var34,iVar12);
                }
                fputs_unlocked("=======\n",pFVar4);
                bVar7 = FUN_00104500(pFVar4,p_Var34,uVar11);
                __fprintf_chk(pFVar4,1,">>>>>>> %s\n",local_118[6]);
                FUN_001045f0(pFVar4,bVar9 | bVar7,pcVar30 + 2,
                             *(long *)(p_Var34->_shortbuf + local_1b0 + -0x73) +
                             *(long *)(p_Var34->_shortbuf + lVar15 + -0x73) +
                             ((3 - *(long *)(p_Var34->_shortbuf + lVar15 + -0x7b)) -
                             *(long *)(p_Var34->_shortbuf + local_1b0 + -0x7b)));
                __fprintf_chk(pFVar4,1,"%lda\n<<<<<<< %s\n",pcVar26 + -1,local_118[4]);
                uVar8 = 0;
LAB_00104072:
                FUN_001045f0(pFVar4,uVar8,pcVar26 + 1,
                             *(long *)(p_Var34->_shortbuf + lVar15 + -0x73) +
                             (1 - *(long *)(p_Var34->_shortbuf + lVar15 + -0x7b)));
                local_18d = bVar27;
              }
            }
          }
          else {
            iVar10 = local_160[(ulong)(p_Var34->_flags - 5) + 3];
            if (iVar10 == 1) {
              if (DAT_001110eb != 0) {
                pcVar30 = p_Var34->_IO_read_end;
                pcVar26 = p_Var34->_IO_read_ptr;
                __fprintf_chk(pFVar4,1,"%lda\n",pcVar30);
                __fprintf_chk(pFVar4,1,">>>>>>> %s\n",local_118[6]);
                FUN_001045f0(pFVar4,0,pcVar30 + 2,
                             *(long *)(p_Var34->_shortbuf + local_1b0 + -0x73) +
                             *(long *)(p_Var34->_shortbuf + lVar15 + -0x73) +
                             ((3 - *(long *)(p_Var34->_shortbuf + lVar15 + -0x7b)) -
                             *(long *)(p_Var34->_shortbuf + local_1b0 + -0x7b)));
                __fprintf_chk(pFVar4,1,"%lda\n<<<<<<< %s\n",pcVar26 + -1,local_118[5]);
                uVar8 = FUN_00104500(pFVar4,p_Var34,iVar12);
                fputs_unlocked("=======\n",pFVar4);
                goto LAB_00104072;
              }
            }
            else if (iVar10 == 2) {
              if (DAT_001110ec == '\0') {
                pcVar26 = p_Var34->_IO_read_ptr;
                pcVar30 = p_Var34->_IO_read_end;
                goto LAB_00103f3d;
              }
            }
            else if (iVar10 == -1) goto LAB_00103da6;
          }
          p_Var34 = p_Var34->_chain;
        } while( true );
      }
      if (DAT_001110e9 == 0) {
        pcVar26 = "  ";
        p_Var34 = local_168;
        if (DAT_001110ee != '\0') {
          pcVar26 = "\t";
        }
        for (; p_Var34 != (_IO_FILE *)0x0; p_Var34 = p_Var34->_chain) {
          if (p_Var34->_flags == 4) {
            local_42 = '\0';
            local_1e8._0_4_ = 3;
            local_1f0._0_4_ = 3;
          }
          else {
            uVar11 = p_Var34->_flags - 5;
            if (2 < uVar11) {
                    /* WARNING: Subroutine does not return */
              FUN_00104300("internal error: invalid diff type passed to output");
            }
            local_1f0._0_4_ = local_160[(ulong)uVar11 + 3];
            local_41 = 0;
            local_42 = (char)(int)local_1f0 + '1';
            local_1e8._0_4_ = (uint)((int)local_1f0 == 0);
          }
          __fprintf_chk(pFVar4,1,"====%s\n",&local_42);
          uVar11 = 0;
          do {
            lVar32 = (long)local_160[(int)uVar11];
            lVar15 = *(long *)(p_Var34->_shortbuf + lVar32 * 0x10 + -0x7b);
            lVar16 = *(long *)(p_Var34->_shortbuf + lVar32 * 0x10 + -0x73);
            uVar28 = uVar11 + 1;
            __fprintf_chk(pFVar4,1,&DAT_0010cb23,uVar28);
            if (lVar15 == lVar16) {
              __fprintf_chk(pFVar4,1,"%ldc\n",lVar15);
            }
            else if (lVar15 - lVar16 == 1) {
              __fprintf_chk(pFVar4,1,"%lda\n",lVar15 + -1);
            }
            else {
              __fprintf_chk(pFVar4,1,&DAT_0010cafa,lVar15,lVar16);
            }
            if ((lVar15 <= lVar16) && ((uint)local_1e8 != uVar11)) {
              lVar38 = 0;
              do {
                fputs_unlocked(pcVar26,pFVar4);
                __ptr = *(void **)(*(long *)(p_Var34->_shortbuf + lVar32 * 8 + -0x4b) + lVar38 * 8);
                __n = *(size_t *)(*(long *)(p_Var34->_shortbuf + lVar32 * 8 + -0x33) + lVar38 * 8);
                lVar38 = lVar38 + 1;
                fwrite_unlocked(__ptr,1,__n,pFVar4);
              } while (lVar38 <= lVar16 - lVar15);
              if (*(char *)((long)__ptr + (__n - 1)) != '\n') {
                uVar22 = dcgettext(0,"No newline at end of file",5);
                __fprintf_chk(pFVar4,1,"\n\\ %s\n",uVar22);
              }
            }
            if ((int)local_1f0 == 1) {
              uVar28 = *(uint *)(&DAT_0010cd00 + (long)(int)uVar11 * 4);
            }
            uVar11 = uVar28;
          } while ((int)uVar28 < 3);
        }
        local_18d = 0;
        goto LAB_00103918;
      }
      FUN_00107810(*(undefined8 *)p_Var36,"r",stdin);
      p_Var33 = stdin;
      p_Var34 = stdout;
      local_18d = 0;
      pcVar26 = (char *)0x0;
      goto LAB_001036fa;
    }
    uVar11 = uVar28 + 1;
    p_Var34 = (_IO_FILE *)(ulong)uVar11;
    iVar12 = strcmp(*(char **)p_Var36,"-");
    if (iVar12 != 0) {
      local_1b0 = (long)(int)uVar11;
      iVar12 = strcmp(*(char **)(&p_Var36->_flags + local_1b0 * 2),"-");
      if (iVar12 != 0) goto LAB_00102ed8;
    }
  }
  else {
    FUN_001042a0("incompatible options",0);
  }
                    /* WARNING: Subroutine does not return */
  FUN_00104300("\'-\' specified for more than one input file");
code_r0x00103455:
  iVar10 = 6;
  if (local_148[0] != 0) {
    if (local_148[1] == 0) {
      iVar10 = 5;
    }
    else {
      iVar10 = 4;
      lVar16 = (long)p_Var34->_IO_read_end - (long)p_Var34->_IO_read_ptr;
      if (lVar16 == (long)p_Var34->_IO_write_base - (long)p_Var34->_IO_read_base) {
        lVar32 = 0;
        pcVar26 = p_Var34->_IO_save_end;
        pcVar30 = p_Var34->_IO_buf_end;
        pcVar37 = p_Var34->_IO_backup_base;
        pcVar23 = p_Var34->_IO_buf_base;
        do {
          if (lVar16 + 1 == lVar32) {
            iVar10 = 7;
            goto LAB_00103492;
          }
          if (((*(void **)(pcVar23 + lVar32 * 8) == (void *)0x0) ||
              (*(void **)(pcVar30 + lVar32 * 8) == (void *)0x0)) ||
             (*(size_t *)(pcVar37 + lVar32 * 8) != *(size_t *)(pcVar26 + lVar32 * 8))) break;
          iVar10 = memcmp(*(void **)(pcVar23 + lVar32 * 8),*(void **)(pcVar30 + lVar32 * 8),
                          *(size_t *)(pcVar37 + lVar32 * 8));
          lVar32 = lVar32 + 1;
        } while (iVar10 == 0);
        iVar10 = 4;
      }
    }
  }
LAB_00103492:
  p_Var34->_flags = iVar10;
  *local_1d0 = p_Var34;
  local_1d0 = &p_Var34->_chain;
  goto LAB_00102ff0;
LAB_001036fa:
  if (p_Var35 == (_IO_FILE *)0x0) goto LAB_00103b2e;
  if (p_Var35->_flags == 4) {
LAB_00103736:
    if (DAT_001110ed == '\0') {
      local_1e8 = (long *)(p_Var35->_IO_read_ptr + -1);
      pcVar26 = p_Var35->_IO_read_ptr + (-2 - (long)pcVar26);
      if ((long)pcVar26 < 0) {
        bVar27 = 0;
        if (DAT_001110ef != 0) {
          local_1f0 = "||||||| %s\n";
LAB_00103a91:
          __fprintf_chk(p_Var34,1,"<<<<<<< %s\n",local_118[4]);
          for (lVar16 = 0; lVar16 <= (long)p_Var35->_IO_read_end - (long)p_Var35->_IO_read_ptr;
              lVar16 = lVar16 + 1) {
            fwrite_unlocked(*(void **)(p_Var35->_IO_buf_base + lVar16 * 8),1,
                            *(size_t *)(p_Var35->_IO_backup_base + lVar16 * 8),p_Var34);
          }
          goto LAB_00103808;
        }
      }
      else {
        iVar12 = 4;
        local_1f0 = "||||||| %s\n";
        bVar27 = DAT_001110ef;
LAB_001037a8:
        do {
          pbVar3 = (byte *)p_Var33->_IO_read_ptr;
          if (pbVar3 < p_Var33->_IO_read_end) {
            p_Var33->_IO_read_ptr = (char *)(pbVar3 + 1);
            uVar11 = (uint)*pbVar3;
          }
          else {
            uVar11 = __uflow(p_Var33);
            if (uVar11 == 0xffffffff) {
              if ((p_Var33->_flags & 0x20U) != 0) goto LAB_00103ae3;
              if ((p_Var33->_flags & 0x10U) != 0) {
LAB_001037d4:
                    /* WARNING: Subroutine does not return */
                FUN_00104300("input file shrank");
              }
            }
          }
          pcVar30 = p_Var34->_IO_write_ptr;
          if (pcVar30 < p_Var34->_IO_write_end) {
            p_Var34->_IO_write_ptr = pcVar30 + 1;
            *pcVar30 = (char)uVar11;
          }
          else {
            __overflow(p_Var34,uVar11 & 0xff);
          }
        } while ((uVar11 != 10) || (bVar39 = pcVar26 != (char *)0x0, pcVar26 = pcVar26 + -1, bVar39)
                );
        bVar39 = bVar27 != 0;
        bVar27 = 0;
        if (bVar39) {
          if (iVar12 == 4) goto LAB_00103a91;
LAB_00103808:
          if (DAT_001110eb != 0) goto LAB_001039e2;
          goto LAB_00103815;
        }
      }
LAB_0010382f:
      for (lVar16 = 0;
          lVar16 <= *(long *)(p_Var35->_shortbuf + local_1b0 * 0x10 + -0x73) -
                    *(long *)(p_Var35->_shortbuf + local_1b0 * 0x10 + -0x7b); lVar16 = lVar16 + 1) {
        fwrite_unlocked(*(void **)(*(long *)(p_Var35->_shortbuf + local_1b0 * 8 + -0x4b) +
                                  lVar16 * 8),1,
                        *(size_t *)
                         (*(long *)(p_Var35->_shortbuf + local_1b0 * 8 + -0x33) + lVar16 * 8),
                        p_Var34);
      }
      if (bVar27 != 0) {
        __fprintf_chk(p_Var34,1,">>>>>>> %s\n",local_118[6]);
      }
      pcVar26 = p_Var35->_IO_read_end;
      pcVar30 = p_Var35->_IO_read_ptr;
      pcVar37 = pcVar26 + (1 - (long)pcVar30);
LAB_0010389a:
      pcVar37 = pcVar37 + -1;
      if (-1 < (long)pcVar37) {
        pcVar23 = p_Var33->_IO_read_end;
        pcVar21 = p_Var33->_IO_read_ptr;
        do {
          while (pcVar23 <= pcVar21) {
            iVar12 = __uflow(p_Var33);
            if (iVar12 == 10) goto LAB_0010389a;
            if (iVar12 == -1) {
              if ((p_Var33->_flags & 0x20U) != 0) goto LAB_00103ae3;
              if ((p_Var33->_flags & 0x10U) != 0) {
                if ((pcVar37 != (char *)0x0) || (p_Var35->_chain != (_IO_FILE *)0x0))
                goto LAB_001037d4;
                goto LAB_00103908;
              }
            }
            pcVar21 = p_Var33->_IO_read_ptr;
            pcVar23 = p_Var33->_IO_read_end;
          }
          p_Var33->_IO_read_ptr = pcVar21 + 1;
          cVar6 = *pcVar21;
          pcVar21 = pcVar21 + 1;
        } while (cVar6 != '\n');
        goto LAB_0010389a;
      }
      pcVar26 = (char *)((long)local_1e8 + (long)(pcVar26 + (1 - (long)pcVar30)));
    }
  }
  else {
    iVar12 = local_160[(ulong)(p_Var35->_flags - 5) + 3];
    if (iVar12 == 1) {
      if (DAT_001110eb != 0) {
        iVar12 = 6;
        local_1e8 = (long *)(p_Var35->_IO_read_ptr + -1);
        pcVar26 = p_Var35->_IO_read_ptr + (-2 - (long)pcVar26);
        local_1f0 = "<<<<<<< %s\n";
        bVar27 = DAT_001110eb;
        if (-1 < (long)pcVar26) goto LAB_001037a8;
LAB_001039e2:
        __fprintf_chk(p_Var34,1,local_1f0,local_118[5]);
        for (lVar16 = 0;
            lVar16 <= *(long *)(p_Var35->_shortbuf + lVar15 * 0x10 + -0x73) -
                      *(long *)(p_Var35->_shortbuf + lVar15 * 0x10 + -0x7b); lVar16 = lVar16 + 1) {
          fwrite_unlocked(*(void **)(*(long *)(p_Var35->_shortbuf + lVar15 * 8 + -0x4b) + lVar16 * 8
                                    ),1,
                          *(size_t *)
                           (*(long *)(p_Var35->_shortbuf + lVar15 * 8 + -0x33) + lVar16 * 8),p_Var34
                         );
        }
LAB_00103815:
        fputs_unlocked("=======\n",p_Var34);
        local_18d = bVar9;
        bVar27 = bVar9;
        goto LAB_0010382f;
      }
    }
    else if (iVar12 == 2) {
      if (DAT_001110ec == '\0') {
        iVar12 = 7;
        local_1e8 = (long *)(p_Var35->_IO_read_ptr + -1);
        local_1f0 = "<<<<<<< %s\n";
        pcVar26 = p_Var35->_IO_read_ptr + (-2 - (long)pcVar26);
        bVar27 = 0;
        if (-1 < (long)pcVar26) goto LAB_001037a8;
        goto LAB_0010382f;
      }
    }
    else if (iVar12 == -1) goto LAB_00103736;
  }
  p_Var35 = p_Var35->_chain;
  goto LAB_001036fa;
LAB_00103ae3:
  uVar22 = dcgettext(0,"read failed",5);
  FUN_00104330(uVar22);
LAB_00103afe:
  FUN_00104330(*(undefined8 *)p_Var35);
LAB_00103b2e:
  do {
    pbVar3 = (byte *)p_Var33->_IO_read_ptr;
    if (pbVar3 < p_Var33->_IO_read_end) {
      p_Var33->_IO_read_ptr = (char *)(pbVar3 + 1);
      uVar11 = (uint)*pbVar3;
    }
    else {
      uVar11 = __uflow(p_Var33);
      if ((uVar11 == 0xffffffff) && ((p_Var33->_flags & 0x30) != 0)) {
LAB_00103908:
        if ((stdin->_flags & 0x20) != 0) {
                    /* WARNING: Subroutine does not return */
          FUN_00104300("read failed");
        }
LAB_00103918:
        free(local_178);
        free(local_170);
        FUN_00104690();
                    /* WARNING: Subroutine does not return */
        exit((uint)local_18d);
      }
    }
    pcVar26 = p_Var34->_IO_write_ptr;
    if (pcVar26 < p_Var34->_IO_write_end) {
      p_Var34->_IO_write_ptr = pcVar26 + 1;
      *pcVar26 = (char)uVar11;
    }
    else {
      __overflow(p_Var34,uVar11 & 0xff);
    }
  } while( true );
}




// Function: try_help @ 0x42a0

void try_help(long param_1,undefined8 param_2)

{
  undefined8 uVar1;
  undefined8 uVar2;
  
  uVar1 = DAT_00121140;
  if (param_1 != 0) {
    uVar1 = dcgettext(0,param_1,5);
                    /* WARNING: Subroutine does not return */
    error(0,0,uVar1,param_2);
  }
  uVar2 = dcgettext(0,"Try \'%s --help\' for more information.",5);
                    /* WARNING: Subroutine does not return */
  error(2,0,uVar2,uVar1);
}




// Function: fatal @ 0x4300

void fatal(undefined8 param_1)

{
  undefined8 uVar1;
  
  uVar1 = dcgettext(0,param_1,5);
                    /* WARNING: Subroutine does not return */
  error(2,0,&DAT_0010c958,uVar1);
}




// Function: perror_with_exit @ 0x4330

void perror_with_exit(undefined8 param_1)

{
  int *piVar1;
  
  piVar1 = __errno_location();
                    /* WARNING: Subroutine does not return */
  error(2,*piVar1,&DAT_0010c958,param_1);
}




// Function: scan_diff_line @ 0x4360

byte * scan_diff_line(char *param_1,ulong *param_2,long *param_3,byte *param_4,char param_5)

{
  byte bVar1;
  char cVar2;
  byte *pbVar3;
  byte *pbVar4;
  byte *pbVar5;
  
  if ((*param_1 != param_5) || (param_1[1] != ' ')) {
                    /* WARNING: Subroutine does not return */
    FUN_00104300("invalid diff format; incorrect leading line chars");
  }
  pbVar4 = (byte *)(param_1 + 2);
  *param_2 = (ulong)pbVar4;
  pbVar5 = pbVar4;
  do {
    pbVar3 = pbVar5;
    pbVar5 = pbVar3 + 1;
  } while (*pbVar3 != 10);
  *param_3 = (long)pbVar5 - (long)pbVar4;
  cVar2 = DAT_001110f0;
  if ((pbVar5 < param_4) && (*pbVar5 == 0x5c)) {
    if (DAT_001110f0 == '\0') {
      *param_3 = ((long)pbVar5 - (long)pbVar4) + -1;
    }
    else {
      __fprintf_chk(stderr,1,&DAT_0010c95b,DAT_00121140);
      cVar2 = DAT_001110f0;
    }
    pbVar5 = pbVar3 + 2;
    do {
      if (cVar2 != '\0') {
        bVar1 = *pbVar5;
        pbVar4 = (byte *)stderr->_IO_write_ptr;
        if (pbVar4 < stderr->_IO_write_end) {
          stderr->_IO_write_ptr = (char *)(pbVar4 + 1);
          *pbVar4 = bVar1;
        }
        else {
          __overflow(stderr,(uint)bVar1);
        }
      }
      pbVar4 = pbVar5 + 1;
      bVar1 = *pbVar5;
      pbVar5 = pbVar4;
      cVar2 = DAT_001110f0;
    } while (bVar1 != 10);
    return pbVar4;
  }
  return pbVar5;
}




// Function: copy_stringlist @ 0x4460

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




// Function: dotlines @ 0x4500

undefined1 dotlines(_IO_FILE *param_1,long param_2,int param_3)

{
  long lVar1;
  char *__ptr;
  char *pcVar2;
  long lVar3;
  long lVar4;
  long lVar5;
  undefined1 local_41;
  
  lVar5 = (long)param_3;
  lVar4 = lVar5 * 0x10 + param_2;
  if (*(long *)(lVar4 + 0x10) < *(long *)(lVar4 + 8)) {
    local_41 = 0;
  }
  else {
    local_41 = 0;
    lVar3 = 0;
    do {
      lVar1 = lVar3 * 8;
      __ptr = *(char **)(*(long *)(param_2 + 0x38 + lVar5 * 8) + lVar3 * 8);
      if (*__ptr == '.') {
        pcVar2 = param_1->_IO_write_ptr;
        if (pcVar2 < param_1->_IO_write_end) {
          local_41 = 1;
          param_1->_IO_write_ptr = pcVar2 + 1;
          *pcVar2 = '.';
        }
        else {
          __overflow(param_1,0x2e);
          local_41 = 1;
        }
      }
      lVar3 = lVar3 + 1;
      fwrite_unlocked(__ptr,1,*(size_t *)(*(long *)(param_2 + 0x50 + lVar5 * 8) + lVar1),param_1);
    } while (lVar3 <= *(long *)(lVar4 + 0x10) - *(long *)(lVar4 + 8));
  }
  return local_41;
}




// Function: process_diff @ 0x46e0

undefined8
process_diff(undefined *param_1,undefined *param_2,undefined8 *param_3,undefined8 *param_4)

{
  uint *__stat_loc;
  byte bVar1;
  int iVar2;
  __pid_t _Var3;
  int iVar4;
  undefined **ppuVar5;
  undefined **ppuVar6;
  byte *pbVar7;
  long lVar8;
  byte *pbVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  int *piVar12;
  uint uVar13;
  byte *pbVar14;
  long lVar15;
  byte *pbVar16;
  char *pcVar17;
  undefined *unaff_R12;
  long lVar18;
  long lVar19;
  ulong uVar20;
  byte *pbVar21;
  long *unaff_R15;
  long in_FS_OFFSET;
  uint local_138;
  undefined4 uStack_134;
  int local_130;
  int local_12c;
  char *local_128;
  undefined *local_120 [9];
  stat local_d8;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_128 = PTR_DAT_00111010;
  ppuVar5 = local_120;
  if (DAT_001110f2 != '\0') {
    local_120[0] = &DAT_0010c98c;
    ppuVar5 = local_120 + 1;
  }
  ppuVar6 = ppuVar5;
  if (DAT_001110f1 != '\0') {
    ppuVar6 = ppuVar5 + 1;
    *ppuVar5 = "--strip-trailing-cr";
  }
  ppuVar6[2] = param_1;
  *ppuVar6 = "--horizon-lines=100";
  uVar13 = 0x10c9b7;
  ppuVar6[1] = &DAT_0010c9b7;
  ppuVar6[3] = param_2;
  ppuVar6[4] = (undefined *)0x0;
  iVar2 = pipe(&local_130);
  if (iVar2 == 0) {
    _Var3 = fork();
    if (_Var3 == 0) goto LAB_00104e2b;
    if (_Var3 == -1) {
      FUN_00104330(&DAT_0010c9bf);
    }
    else {
      close(local_12c);
      iVar2 = local_130;
      iVar4 = fstat(local_130,&local_d8);
      if (iVar4 != 0) {
        FUN_00104330("fstat");
        goto LAB_00104e1f;
      }
      uVar20 = local_d8.st_blksize;
      if (local_d8.st_blksize < 1) {
        uVar20 = 1;
      }
      lVar15 = 0;
      pbVar7 = (byte *)FUN_00107350(uVar20);
      while( true ) {
        lVar18 = uVar20 - lVar15;
        lVar8 = FUN_00107a10(iVar2,pbVar7 + lVar15,lVar18);
        lVar15 = lVar15 + lVar8;
        if (lVar18 != lVar8) break;
        if (0x3ffffffffffffffe < uVar20) goto LAB_00104d6a;
        uVar20 = uVar20 * 2;
        pbVar7 = (byte *)FUN_001073a0(pbVar7,uVar20);
      }
      if (lVar8 != -1) {
        if ((lVar15 != 0) && (pbVar7[lVar15 + -1] != 10)) goto LAB_00104e8f;
        iVar2 = close(iVar2);
        if (iVar2 == 0) {
          __stat_loc = &local_138;
          _Var3 = waitpid(_Var3,(int *)__stat_loc,0);
          if (-1 < _Var3) {
            if ((local_138 & 0x7f) != 0) {
              uVar13 = 0x7fffffff;
              pcVar17 = "subsidiary program \'%s\' failed";
              unaff_R12 = PTR_DAT_00111010;
              goto LAB_00104d87;
            }
            bVar1 = (byte)(local_138 >> 8);
            if (((int)local_138 >> 8 & 0xfeU) == 0) {
              pbVar16 = pbVar7 + lVar15;
              *param_4 = pbVar7;
              do {
                if (pbVar16 <= pbVar7) {
                  __stat_loc[0] = 0;
                  __stat_loc[1] = 0;
                  *param_3 = unaff_R15;
                  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
                    return CONCAT44(uStack_134,local_138);
                  }
                    /* WARNING: Subroutine does not return */
                  __stack_chk_fail();
                }
                unaff_R15 = (long *)FUN_00107350(0x48);
                unaff_R15[5] = 0;
                unaff_R15[4] = 0;
                unaff_R15[7] = 0;
                unaff_R15[6] = 0;
                bVar1 = *pbVar7;
                pbVar9 = pbVar7;
                if (bVar1 != 0x20) goto LAB_001048f5;
                do {
                  do {
                    bVar1 = pbVar9[1];
                    pbVar9 = pbVar9 + 1;
                  } while (bVar1 == 0x20);
LAB_001048f5:
                  uVar13 = (uint)bVar1;
                } while (bVar1 == 9);
                pbVar21 = pbVar7;
                if (9 < uVar13 - 0x30) {
LAB_00104999:
                  uVar11 = DAT_00121140;
                  uVar10 = dcgettext(0,"%s: diff failed: ",5);
                  __fprintf_chk(stderr,1,uVar10,uVar11);
                  do {
                    bVar1 = *pbVar21;
                    pbVar7 = (byte *)stderr->_IO_write_ptr;
                    if (pbVar7 < stderr->_IO_write_end) {
                      stderr->_IO_write_ptr = (char *)(pbVar7 + 1);
                      *pbVar7 = bVar1;
                    }
                    else {
                      __overflow(stderr,(uint)bVar1);
                    }
                    bVar1 = *pbVar21;
                    pbVar21 = pbVar21 + 1;
                  } while (bVar1 != 10);
                    /* WARNING: Subroutine does not return */
                  exit(2);
                }
                lVar15 = 0;
                do {
                  pbVar9 = pbVar9 + 1;
                  lVar15 = (long)(int)(uVar13 - 0x30) + lVar15 * 10;
                  uVar13 = (uint)*pbVar9;
                } while (uVar13 - 0x30 < 10);
                *unaff_R15 = lVar15;
                bVar1 = *pbVar9;
                if (bVar1 != 0x20) goto LAB_0010494d;
                do {
                  do {
                    bVar1 = pbVar9[1];
                    pbVar9 = pbVar9 + 1;
                  } while (bVar1 == 0x20);
LAB_0010494d:
                } while (bVar1 == 9);
                if (bVar1 == 0x2c) {
                  uVar13 = (uint)pbVar9[1];
                  if (9 < pbVar9[1] - 0x30) goto LAB_00104999;
                  lVar8 = 0;
                  pbVar9 = pbVar9 + 1;
                  do {
                    pbVar9 = pbVar9 + 1;
                    lVar8 = (long)(int)(uVar13 - 0x30) + lVar8 * 10;
                    uVar13 = (uint)*pbVar9;
                  } while (uVar13 - 0x30 < 10);
                  unaff_R15[1] = lVar8;
                }
                else {
                  unaff_R15[1] = lVar15;
                }
                bVar1 = *pbVar9;
                if ((bVar1 == 9) || (bVar1 == 0x20)) {
                  do {
                    do {
                      bVar1 = pbVar9[1];
                      pbVar9 = pbVar9 + 1;
                    } while (bVar1 == 0x20);
                  } while (bVar1 == 9);
                }
                if (bVar1 == 99) {
                  iVar2 = 2;
                }
                else if (bVar1 == 100) {
                  iVar2 = 3;
                }
                else {
                  if (bVar1 != 0x61) goto LAB_00104999;
                  iVar2 = 1;
                }
                pbVar14 = pbVar9 + 1;
                bVar1 = pbVar9[1];
                if (bVar1 != 0x20) goto LAB_00104a2c;
                do {
                  do {
                    bVar1 = pbVar14[1];
                    pbVar14 = pbVar14 + 1;
                  } while (bVar1 == 0x20);
LAB_00104a2c:
                  uVar13 = (uint)bVar1;
                } while (bVar1 == 9);
                if (9 < uVar13 - 0x30) goto LAB_00104999;
                lVar8 = 0;
                do {
                  pbVar14 = pbVar14 + 1;
                  lVar8 = (long)(int)(uVar13 - 0x30) + lVar8 * 10;
                  uVar13 = (uint)*pbVar14;
                } while (uVar13 - 0x30 < 10);
                unaff_R15[2] = lVar8;
                bVar1 = *pbVar14;
                if (bVar1 != 0x20) goto LAB_00104a7c;
                do {
                  do {
                    bVar1 = pbVar14[1];
                    pbVar14 = pbVar14 + 1;
                  } while (bVar1 == 0x20);
LAB_00104a7c:
                } while (bVar1 == 9);
                if (bVar1 == 0x2c) {
                  uVar13 = (uint)pbVar14[1];
                  if (9 < pbVar14[1] - 0x30) goto LAB_00104999;
                  pbVar14 = pbVar14 + 1;
                  lVar18 = 0;
                  do {
                    pbVar14 = pbVar14 + 1;
                    lVar18 = (long)(int)(uVar13 - 0x30) + lVar18 * 10;
                    uVar13 = (uint)*pbVar14;
                  } while (uVar13 - 0x30 < 10);
                  unaff_R15[3] = lVar18;
                  bVar1 = *pbVar14;
                  if ((bVar1 == 9) || (bVar1 == 0x20)) {
                    do {
                      do {
                        bVar1 = pbVar14[1];
                        pbVar14 = pbVar14 + 1;
                      } while (bVar1 == 0x20);
                    } while (bVar1 == 9);
                  }
                }
                else {
                  unaff_R15[3] = lVar8;
                  bVar1 = *pbVar14;
                }
                pbVar7 = pbVar14 + 1;
                pbVar21 = pbVar14;
                if (bVar1 != 10) goto LAB_00104999;
                if (iVar2 == 2) {
LAB_00104c78:
                  lVar15 = *unaff_R15;
LAB_00104ab7:
                  lVar8 = unaff_R15[1];
                  uVar20 = (lVar8 - lVar15) + 1;
                  if (0xffffffffffffffe < uVar20) {
LAB_00104d6a:
                    /* WARNING: Subroutine does not return */
                    FUN_001077d0();
                  }
                  lVar18 = FUN_00107350();
                  unaff_R15[4] = lVar18;
                  lVar18 = FUN_00107350(uVar20 * 8);
                  unaff_R15[6] = lVar18;
                  if (uVar20 != 0) {
                    lVar19 = 0;
                    while( true ) {
                      pbVar7 = (byte *)FUN_00104360(pbVar7,lVar19 * 8 + unaff_R15[4],
                                                    lVar18 + lVar19 * 8,pbVar16,0x3c);
                      if (lVar8 - lVar15 == lVar19) break;
                      lVar18 = unaff_R15[6];
                      lVar19 = lVar19 + 1;
                    }
                  }
                  if (iVar2 == 2) {
                    iVar2 = strncmp((char *)pbVar7,"---\n",4);
                    if (iVar2 != 0) {
                    /* WARNING: Subroutine does not return */
                      FUN_00104300("invalid diff format; invalid change separator");
                    }
                    pbVar7 = pbVar7 + 4;
                    goto LAB_00104b6d;
                  }
                  if (iVar2 != 3) goto LAB_00104b6d;
                }
                else {
                  if (iVar2 == 3) {
                    unaff_R15[2] = lVar8 + 1;
                    goto LAB_00104ab7;
                  }
                  *unaff_R15 = lVar15 + 1;
                  if (iVar2 != 1) goto LAB_00104c78;
LAB_00104b6d:
                  lVar15 = unaff_R15[3];
                  lVar8 = unaff_R15[2];
                  uVar20 = (lVar15 - lVar8) + 1;
                  if (0xffffffffffffffe < uVar20) goto LAB_00104d6a;
                  lVar18 = FUN_00107350();
                  unaff_R15[5] = lVar18;
                  lVar18 = FUN_00107350(uVar20 * 8);
                  unaff_R15[7] = lVar18;
                  if (uVar20 != 0) {
                    lVar19 = 0;
                    while( true ) {
                      pbVar7 = (byte *)FUN_00104360(pbVar7,lVar19 * 8 + unaff_R15[5],
                                                    lVar18 + lVar19 * 8,pbVar16,0x3e);
                      if (lVar15 - lVar8 == lVar19) break;
                      lVar18 = unaff_R15[7];
                      lVar19 = lVar19 + 1;
                    }
                  }
                }
                *(long **)__stat_loc = unaff_R15;
                __stat_loc = (uint *)(unaff_R15 + 8);
              } while( true );
            }
            goto LAB_00104dc5;
          }
          FUN_00104330("waitpid");
        }
        FUN_00104330("close");
LAB_00104e8f:
                    /* WARNING: Subroutine does not return */
        FUN_00104300("invalid diff format; incomplete last line");
      }
      uVar11 = dcgettext(0,"read failed",5);
      bVar1 = FUN_00104330(uVar11);
LAB_00104dc5:
      uVar13 = (uint)bVar1;
      unaff_R12 = PTR_DAT_00111010;
      if (bVar1 == 0x7e) {
        pcVar17 = "subsidiary program \'%s\' could not be invoked";
        uVar13 = 0x7e;
        goto LAB_00104d87;
      }
    }
    pcVar17 = "subsidiary program \'%s\' failed (exit status %d)";
    if (uVar13 == 0x7f) {
      pcVar17 = "subsidiary program \'%s\' not found";
    }
LAB_00104d87:
    uVar11 = dcgettext(0,pcVar17,5);
                    /* WARNING: Subroutine does not return */
    error(2,0,uVar11,unaff_R12,uVar13);
  }
LAB_00104e1f:
  FUN_00104330(&DAT_0010c9ba);
LAB_00104e2b:
  close(local_130);
  if (local_12c != 1) {
    dup2(local_12c,1);
    close(local_12c);
  }
  execvp(PTR_DAT_00111010,&local_128);
  piVar12 = __errno_location();
                    /* WARNING: Subroutine does not return */
  _exit((*piVar12 == 2) + 0x7e);
}



